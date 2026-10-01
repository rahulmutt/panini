# Contributor & agent guide

## Environment
- Toolchain is pinned via `mise` (`mise install`) to rust 1.99.0. Do not install
  Rust globally.
- Tasks: `mise run build | test | lint | fmt | fmt-check | mutants | audit`.
- `mise run test` is the whole suite, including the exhaustive roundtrip:
  every derived form goes through the real `Panini::check()` and is compared
  against a brute-force oracle (`crates/panini/tests/roundtrip.rs`). It
  takes a few seconds, because `panini_analyze::candidates()` answers from a
  corpus index built once per process.
- Optional dev/audit tooling is pinned in `mise.dev.toml`. Install it on demand:
  `MISE_ENV=dev mise install`. This provides:
  - `cargo-mutants` (mutation testing) — `mise run mutants` runs
    `cargo mutants --package panini-prakriya --package panini-analyze
    --test-workspace=true --timeout 110 -j 4`. Run the gate through the task
    rather than reconstructing the flags. The `--test-workspace` flag is
    required so each **mutant** run exercises the `panini` crate's golden
    paradigm/trace/roundtrip tests, not just the mutated packages' own unit
    tests — but it does NOT apply to
    cargo-mutants' own **baseline** run, which always exercises only the
    mutated packages' tests regardless of the flag. The explicit `--timeout`
    is still required: cargo-mutants calibrates its per-mutant timeout from
    the baseline's runtime (`panini-prakriya`'s unit tests, ~2s, with a 20s
    floor), and that auto-calibrated value does not guarantee the 5x margin
    over a full uncaught `panini` suite run (8.6-18.2s as measured in 3f2: 8.64/9.06s in the isolated probe, 13.4/18.2s under campaign load) under `-j 4` load.
    **The cap must clear a full UNCAUGHT run of the workspace suite at the
    parallelism you actually use.** Under a cap that doesn't, a mutant that
    survives is recorded as a **timeout rather than a survivor**, so a
    reported zero-survivor run that checks only `missed.txt` is vacuous
    instead of clean. Always check `timeout.txt` alongside `missed.txt`.
    This repo has hit it twice: slice 7a through suite growth (`--timeout
    300`, 9 timeouts) and slice 7b through parallelism (`-j 16`, 43
    timeouts). `cargo mutants` also reads `-j` from `CARGO_MUTANTS_JOBS`, so
    an unqualified cap can be defeated by the environment alone; keep `-j`
    at or below 4, or re-measure and raise the cap in step.
    **The floor behind the 110s cap, measured at 4608 cells on Rust 1.99.0,
    2026-10-01.** Two `mise run test` runs took 7.848s and 7.864s wall clock
    (host load averages `16.19 16.94 17.52` after the second; the host is
    shared and was loaded by other tenants with no busy process of ours, so
    this is not comparable with the 8.019s / 8.348s taken at 4572 cells on
    1.98.1 under load about 34, nor with the 5.418s / 5.419s at 4428 cells
    under load 7.46). An isolated `-j 4`
    probe of the two documented equivalent mutants ran the full suite
    uncaught in 8.644s (`adesha.rs:589:30`) and 9.056s
    (`tripadi.rs:1270:38`). Under campaign load the uncaught phases were
    18.223s and 13.372s (below). The cap is 6 × the longest of the
    campaign-load phases (109.3s), rounded up to the next 10s: 110. At the
    old 80s the margin on the longest equivalent was 4.39x, under the 5x
    rule, so the cap moved from 80 to 110 in this slice (it was 60 in 3f's
    campaign, 900 against the Θ(N²) suite, before `candidates()` answered
    from a corpus index). The campaign-load phases grew from 3f's 12.358s /
    11.614s although the isolated probe got faster; the cause is unmeasured
    (candidates: campaign-window host contention, the rustc 1.99.0 bump),
    so take the campaign-load phase, never the
    isolated one. Take the floor by measurement, never by scaling it by cell
    count or by a projected contention multiplier. Re-measure the floor and
    an uncaught `-j 4` run whenever the golden suite grows, and change
    `mise.toml` and this paragraph together.
    **One timeout is correct and permanent.** `tripadi.rs`'s 8.4.2 ṇatva
    backward scan decrements a loop index with `j -= 1`; the `j /= 1` mutant
    makes `j` constant, and the loop never terminates. No assertion can ever
    catch it — the mutated run never reaches one — so the cap itself *is*
    the detection mechanism. This is a different phenomenon from the
    reclassification problem above (a real survivor misreported because the
    cap is too short): this mutant hangs at any cap. Identify it by that
    shape rather than by its line number, which drifts, and do not chase it
    with a bigger `--timeout` or a code change; the loop is correct, working
    code. Any other timeout is a suspect survivor: re-run it alone before
    concluding anything.
    **Two tool hazards.** Every `cargo-mutants` invocation rotates
    `mutants.out` → `mutants.out.old` and discards the previous `.old`, so
    pass `-o` to an isolated directory for any follow-up run against a
    finished campaign. The mise shim fails in background shells ("no version
    is set for shim: cargo-mutants"); run the installed `cargo-mutants`
    binary directly, with the task's arguments.
    **Current record (juhotyādi 3f2, 2026-10-01).** Campaign at
    `-j 4 --timeout 80` (the cap the campaign ran under; 3f2 then raised it
    to 110, below), `--package panini-prakriya --package
    panini-analyze --test-workspace=true`, `-o
    /home/dev/mutants-records/juhotyadi-3f2`, launched detached, window
    19:24:40 - 19:44:23 UTC (host load average about 16-17 at the floor), on
    the tree at `85d12af`. **773 mutants tested: 722 caught, 48 unviable, 2
    missed, 1 timeout.** **panini-prakriya: 761 mutants, 714 caught, 44
    unviable, 2 missed, 1 timeout.** Its non-caught set (44 / 2 / 1) is
    identical to 3f's up to the two `tripadi.rs` entries moving down
    (1223 to 1270, 1549 to 1583) under this slice's `tripadi.rs` edits; the
    adesha equivalent still sits at `adesha.rs:589:30`. `missed.txt` held
    exactly:
    ```
    crates/panini-prakriya/src/tinanta/adesha.rs:589:30: replace + with *
    crates/panini-prakriya/src/tinanta/tripadi.rs:1270:38: replace - with /
    ```
    `timeout.txt` held exactly the permanent ṇatva mutant:
    ```
    crates/panini-prakriya/src/tinanta/tripadi.rs:1583:23: replace -= with /=
    ```
    **panini-analyze: 12 mutants, 8 caught, 4 unviable, 0
    missed, 0 timeout.** None non-caught beyond the 4 unviable (the
    `vec![Default::default()]` and
    `HashMap::from_iter([..., vec![Default::default()]])` replacements do not
    compile, as `Candidate` has no `Default`). The two packages sum to the
    773 / 722 / 48 / 2 / 1 total. No 6.4.100 mutant (`guna.rs:1362-1387`: 2
    mutants, both caught), no 8.2.26 mutant (`tripadi.rs:269-289`: 9
    mutants, all caught) and no 8.4.55 mutant (`tripadi.rs:1487-1518`: 8
    mutants, all caught) is missed, timed out or unviable, so nothing needs
    an unviable-without-reason exemption.
    Under campaign load the two uncaught equivalents' test phases were 18.223s
    (`adesha.rs:589:30`) and 13.372s (`tripadi.rs:1270:38`). Caught test
    phases (722) ran min 0.102s, median 2.580s, p90 10.411s, max 26.026s.
    Against the 80s cap the margin on the longest equivalent was 80 / 18.223 =
    4.39x, below the 5x rule, so the cap moves to 6 × 18.223 = 109.3s, rounded
    up to 110, and `mise.toml` and this file changed together (margin 6.04x
    at 110; the slowest caught phase is 4.23x, but a caught mutant ends when
    its first assertion fails, so only the uncaught equivalents set the cap).
    A higher cap can only turn timeouts into outcomes, and the only timeout is
    the permanent `j /= 1` hang, so the campaign's outcomes stand without a
    re-run. `outcomes.json` is kept at
    `/home/dev/mutants-records/juhotyadi-3f2/mutants.out/outcomes.json`, with
    a durable copy at
    `/home/dev/mutants-records/juhotyadi-3f2/outcomes.durable.json`.
    The juhotyādi 3f record it replaces: `git show 85d12af:AGENTS.md`.
    **The per-slice history** of the floor, the cap and every campaign from
    the pada audit through slice 3d, all measured against the Θ(N²) suite,
    was removed in the commit that introduced this paragraph. Read it with
    `git show 440c8c3^:AGENTS.md`.
  - `cargo-deny` + `cargo-audit` (supply-chain checks) — `mise run audit` runs
    `cargo audit && cargo deny check` and is expected to pass, including
    `cargo deny check advisories`.
  - `cargo-fuzz` (fuzzing of `panini-lipi`, target at `crates/panini-lipi/fuzz`)
    — pinned here, but real fuzzing still needs a **nightly** Rust toolchain,
    which is not provisioned in this environment; install nightly yourself.

## Rules of the codebase
- SLP1 is the only internal representation; transliterate only in `panini-lipi`.
- `#![forbid(unsafe_code)]` in every non-fuzz crate (the `panini-lipi` fuzz
  target under `crates/panini-lipi/fuzz` legitimately omits it, since it uses
  `#![no_main]` plus the libfuzzer harness macro).
- Grammar changes are gated by the golden paradigm test
  (`crates/panini/tests/paradigm/`, 4608 cells, nine gaṇas — eight complete,
  tanādi closing at 10/10 in slice 8b (nine of its ten dhātupāṭha rows
  curated in slice 8a; √kṛ, the tenth and last, in 8b), and juhotyādi (3)
  opened in slice 3a at 2 of its 26 rows, at 4 after slice 3b curated √bhī
  and √hrī, at 8 after slice 3c curated √dā, √dhā, √mā
  and √hā (ātmanepada), at 10 after slice 3c2 curated √hā (parasmaipada) and
  √gā, at 16 after slice 3d curated √pṝ, √pṛ, √bhṛ, √ghṛ, √hṛ
  and √sṛ, at 17 after slice 3d2 curated √ṛ, at 20 after slice 3e curated √ṇij, √vij
  and √viṣ, at 24 after slice 3f curated √kit, √tur, √dhiṣ and √dhan, and now
  at 25 of its 26 after slice 3f2 curated √bhas —
  `PARADIGM`
    stays one-form-per-cell: a cell forked by an optional rule keeps its
    other forms — a second (596 cells), a third (155 cells), a fourth
    (eighteen
    cells, rudhādi's √piṣ and — new in slice 7d — √śiṣ loṭ madhyama eka, and
    — new in slice 8a — fifteen more spread across tanādi's four ik-upadhā
    roots kziR/fR/tfR/GfR, and — new in slice 3b — √bhī's vidhiliṅ prathama
    eka) and
    — the loṭ parasmaipada cells of
    rudhādi's √kṛt, √rudh, √bhid, √kṣud, √tṛd, √und and — new in slice 7f —
    √chid and √chṛd,
    eight ways tied as the record until slice 8a, when the loṭ parasmaipada
    prathama AND madhyama eka of tanādi's four ik-upadhā roots kziR, fR, tfR
    and GfR doubled it to sixteen, and slice 3b's √bhī loṭ parasmaipada
    madhyama eka took it to seventeen — a fourth
    and fifth (prathama eka) or a fourth through sixth (madhyama eka), or
    seventh for slice 3c2's √hā (`03.0009`) loṭ madhyama eka, the one
    seven-form cell — in
    `ALTERNATES` (1091 rows in all, so 4608 + 1091 = 5699 forms total); √bhuj
    joins neither fork record — its forks stack only 7.1.35 and 8.4.56, the
    same two-deep profile as √yuj — but the √bhuj/1.3.66 slice adds two
    trace pins of its own, `bhunkte_trace_credits_1_3_66_not_1_3_72` and
    `bhunakti_trace_credits_the_shesa_1_3_78`, and
    `derivation_set_is_exactly_pinned` asserts each cell's derivation set is
    exactly the union of the two. The suite is no longer filtered by any
    one-form-per-cell convention — the
    "retiring the conventions" slice retired the last two (7.1.35 tātaṅ,
    8.4.56 pausal cartva), and `PARADIGM`'s index 0 is now genuinely the
    declined derivation rather than a hand-picked citation form: prathama
    eka of laṅ and vidhiliṅ is the jaś form for parasmaipada roots
    (`aBavad`, `Baved`), since 8.2.39 *jhalāṁ jaśo'nte* is obligatory;
    bhvādi/divādi/
    tudādi are complete across laṭ/laṅ/loṭ/vidhiliṅ × parasmaipada/
    ātmanepada, and adādi (gaṇa 2) is now **complete** — √yā/√vā/√ad
    (parasmaipada) and √ās/√vas/√śī (ātmanepada) are complete across all four
    lakāras (laṭ/laṅ/loṭ/vidhiliṅ). √ad (parasmaipada) lands the internal
    junction sandhi cartva (8.4.55); √ās (ātmanepada) lands 7.1.5
    ātmanepadeṣv anataḥ and extends 6.1.90 āṭaś ca / 6.1.66 lopo vyor vali to
    the athematic (śap-luk'd) ātmanepada path (loṭ 1sg + optative); √vas
    (ātmanepada) is the second witness for 8.2.25 dhi ca, which elides an
    aṅga-final `s` before a Dh-initial affix (ADve, vaDve) — it replaced the
    8.4.53 jaśtva analysis slice 5d shipped, and 8.4.53 was removed as
    unreachable; √śī (ātmanepada) closes the gaṇa and lands 7.4.21 śīṅaḥ
    sārvadhātuke guṇaḥ (guṇa despite the ṅit ending — the gaṇa's only visible
    guṇa), 7.1.6 śīṅo ruṭ (Serate), and 8.3.59 ādeśapratyayayoḥ, the engine's
    first ṣatva (Seze, Sezva) — see
    `docs/superpowers/specs/2026-07-25-adadi-si-5f-design.md` for the full
    rule analysis; kryādi (gaṇa 9) is now **complete** — six roots across all
    four lakāras, the first gaṇa whose vikaraṇa (śnā) is itself reshaped by
    the ending. √kliś, √gudh, √aś (parasmaipada) landed in slice 9a; √muṣ,
    √vrī (parasmaipada) and √vṛṅ (ātmanepada) landed in slice 9b along with
    8.4.1 / 8.4.2, the engine's first ṇatva. √vṛṅ is the gaṇa's **only**
    ātmanepadī root — every other ātmanepada form in kryādi belongs to an
    ubhayapadī root, and no kryādi ubhayapadī root is curated. The
    ubhayapada slice landed 1.3.72 *svaritañitaḥ* (with rudhādi's √rudh),
    so the pada model no longer stands in their way; whether any given one
    needs phonology of its own is a per-root question nobody has asked yet —
    see `docs/superpowers/specs/2026-07-28-kryadi-gana-design.md`; svādi
    (gaṇa 5) is now **complete** — six roots across all four lakāras: √āp,
    √śak, √hi and √ri (parasmaipada), √aś (`05.0020`, distinct from kryādi's
    `09.0059`) and √ṣṭigh (`stiG`) (ātmanepada). Its vikaraṇa is śnu (3.1.73),
    and it is the first gaṇa where 7.3.84's guṇa lands on the vikaraṇa rather
    than the root: 7.3.84 now applies twice, once with respect to śnu and once
    with respect to the ending (1.4.13 makes the aṅga affix-relative), giving
    `Apnoti` against the ṅit-blocked `ApnutaH`. The other split running
    through the gaṇa is *asaṁyogapūrva* — whether śnu's `u` is preceded by a
    conjunct decides both the yaṇ alternation (6.4.87 / 6.4.77: `hinvanti`
    against `Apnuvanti`) and the hi-luk (6.4.106: `hinu` against `Apnuhi`) —
    see `docs/superpowers/specs/2026-07-29-svadi-gana-design.md`.) rudhādi
    (gaṇa 7, vikaraṇa śnam) is now **complete** — the first gaṇa curated at
    its full dhātupāṭha strength. Nine of its 25 dhātupāṭha roots are ubhayapadī
    (`~^`-marked); slice 7a lands three roots that need nothing beyond the
    gaṇa's own spine (√kṛt, √hiṃs — stored `hins` — and √khid), 7b adds
    three more, one per consonant family: √bhañj (cu-class final), √piṣ
    (ṣ-final) and √indh (jhaṣ-final, the gaṇa's second ātmanepada root),
    and the ubhayapada slice adds the gaṇa's own **eponym**, √rudh
    (`07.0001 ru\Di~^r`), with 1.3.72 *svaritañitaḥ* — the engine's first
    ubhayapadī root, deriving a full paradigm in each pada. Slice 7c then
    curated four more, taking the gaṇa from seven roots to **eleven**:
    √bhid (`07.0002 Bi\di~^r`), √kṣud (`07.0006 kzu\di~^r`), √yuj
    (`07.0007 yu\ji~^r`) and √tṛd (`07.0009 u~tfdi~^r`), all four
    ubhayapadī by 1.3.72 and all four pinned in both padas. The
    8.2.30/8.2.39 generalization slice then curated two more, taking the
    gaṇa to **thirteen**: √ric (`07.0004 ri\ci~^r`) and √vic
    (`07.0005 vi\ci~^r`), also ubhayapadī by 1.3.72 and pinned in both
    padas. Rudhādi 7d then curated eight more, on the audited numbers
    alone with no new sūtra, taking the gaṇa to **twenty-one**: √vid
    (`07.0013 vi\da~\`, ātmanepadī), √śiṣ (`07.0014 Si\zx~`), √und
    (`07.0020 undI~`), √añj (`07.0021 anjU~`), √tañc (`07.0022 tancU~`),
    √vij (`07.0023 o~vijI~`), √vṛj (`07.0024 vfjI~`) and √pṛc
    (`07.0025 pfcI~`), the other seven parasmaipadī and none of the eight
    ubhayapadī.
    Rudhādi 7e then curated the ninth and last reachable non-ubhayapadī
    root, taking the gaṇa to **twenty-two**: √tṛh (`07.0018 tfha~`), behind
    three new sūtras — 7.3.92 *tṛṇaha im* (the *im* augment), 8.2.31 *ho
    ḍhaḥ* and 8.3.13 *ḍho ḍhe lopaḥ* — and three widenings of rules the
    engine already had: 8.4.41 *ṣṭunā ṣṭuḥ* (its trigger widened from a
    bare `z` literal to the full ṣṭu class `z`/`w`/`W`/`q`/`Q`/`R`), 8.2.41
    *ṣaḍhoḥ kaḥ si* (widened from `z` alone to `z`/`Q`, the dvandva's other
    named sound) and 6.1.87 *ād guṇaḥ* (a second arm, for the `a i` the
    *im* augment leaves wholly inside `SHAP`, distinct from the junction
    arm's ending-initial `i`/`I`). **7d's own deferral undercounted the
    gap**: it named only the three sūtras √tṛh lacked outright, but three
    rules the engine already had were too narrow to carry the root as
    well — and 8.4.41's own guard comment, titled NARROW GUARD before this
    slice renamed it, had predicted exactly this widening. √tṛh's deepest
    cells — the ones every other stop-final rudhādi root turns into
    six-form forks (8.4.53 voices, 8.4.65 optionally elides, 7.1.35 and
    8.4.56 each multiply that by three) — hold only **three** forms,
    because 8.3.13 obligatorily elides the very ḍh that 8.4.65 would
    otherwise fork on; `tripadi.rs`'s comment on 8.3.13 and
    `trnaddhi_trace_has_8_3_13_and_no_8_4_65` in `panini`'s trace suite are
    the record. A standing divergence from vidyut-prakriya's own traces,
    not introduced by this slice: vidyut credits 6.1.68 *hal ṅyāb bhyo
    dīrghāt su-ti-sy-apṛktaṁ hal* for the apṛkta-`t` deletion every curated
    rudhādi root's laṅ takes (`akfRat`, `aBinat`, `apinaw`, `aBanak`, and
    now `atfReq`); this engine has no 6.1.68 and reaches the same surface
    through 8.2.23 *saṁyogāntasya lopaḥ* instead — audited clean either
    way, and predating √tṛh. And 6.3.111 *ḍhralope pūrvasya dīrgho'ṇaḥ* is
    deliberately unimplemented: it lengthens a preceding *aṇ* before the
    very ḍh-elision 8.3.13 performs, but no √tṛh cell presents one there
    (the elided ḍh always follows `e` or `M`), and vidyut-prakriya's own
    traces do not emit it either — the reason is recorded in place at
    8.3.13, not merely deferred.
    `curated_pada_agrees_with_upadesha_markers` in `panini-data` now
    re-derives all 66 verdicts from the vendored upadeśa, so the column
    cannot drift from the data that determines it. That discharges
    the **ubhayapada** deferral as such: 1.3.72 is no longer what keeps any
    root out, and — since rudhādi 7f curated √chid and √chṛd — every
    ubhayapadī-marked root in the curated set now derives, verified cell by
    cell against vidyut-prakriya.
    The pada audit added two more: `01.1049 RI\Y` (√nī, bhvādi) and
    `06.0001 tu\da~^` (√tud, tudādi), both ubhayapadī by 1.3.72 and both
    curated parasmaipada until then. √tud was a known deferral; √nī was
    named by no deferral list and was read past by every slice from v1 on.
    **√bhid, √kṣud, √yuj and √tṛd were called "curation-only" for months
    with no run behind the claim; 7c ran it.** None of the four needed a
    new sūtra, and the whole-corpus cross-implementation audit of
    2026-08-17, against vidyut-prakriya at commit
    `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, found **zero differences
    across all 2160 cells / 2496 forms / 53 roots** — with the `entry`
    negative control **verified failing first** (exit 1, 36 √bhū cells
    flagged), so the zero is not vacuous. Byte-identity to vidyut is now a
    sourced result rather than an assertion.
    **The 8.2.30/8.2.39 generalization slice ran the harness twice.** The
    first run, against vidyut-prakriya at the same `8da2f90` commit, found
    **four differing cells** — √ric's and √vic's declined laṅ **parasmaipada**
    prathama and madhyama eka — which is what exposed 8.2.39's narrow
    guard as a real defect rather than a documented deferral: real code,
    not only the synthetic `entry` control, tripped the harness. After
    8.2.39 was generalised, the second run came back clean: **zero
    differences across all 2304 cells / 2654 forms / 55 roots**, with the
    `entry` negative control verified failing first (36 √bhū cells) both
    times.
    **Rudhādi 7d's own cross-implementation audit** ran the same probe
    against vidyut-prakriya at commit `8da2f90`, over the grown corpus:
    **zero differences across all 2592 cells / 3014 forms / 63 roots**,
    with the `entry` negative control verified failing first. No `Rule`
    changed — the eight roots derive on the rules already in the pipeline.
    **Rudhādi 7e's own cross-implementation audit** ran the same probe
    against vidyut-prakriya at commit `8da2f90`, over the corpus grown by
    √tṛh: **zero differences across all 2628 cells / 3057 forms / 64
    roots**, with both negative controls (`entry` and `form`) verified
    failing first. This is the first of these audits with new `Rule`s
    behind it: 7.3.92, 8.2.31 and 8.3.13, plus the two tripādī widenings
    above (8.4.41 and 8.2.41) — an earlier task in this slice had already
    proved those two widenings inert on the pre-7e 2592-cell corpus by a
    byte-for-byte dump diff of its own, before √tṛh was curated at all.
    6.1.87's second arm was not part of that dump diff: it landed later,
    and is inert by construction rather than by measurement — it is
    gated on `7.3.92` appearing in `p.log`, so no pre-7e derivation can
    reach it, and the residual risk is what the 2628-cell audit covers.
    **Rudhādi 7f's own cross-implementation audit** ran the same probe
    against vidyut-prakriya at commit `8da2f90`, over the corpus grown by
    √chid and √chṛd: **zero differences across all 2772 cells / 3259
    forms / 66 roots**, with both negative controls (`entry` and `form`)
    verified failing first. The new `Rule`s behind it are 6.1.73 and
    8.4.40 — an earlier task in this slice had already proved both inert
    on the pre-7f 2628-cell corpus by a byte-for-byte dump diff of its
    own, before √chid and √chṛd were curated at all.
    **The √bhuj/1.3.66 slice's own cross-implementation audit** ran the
    same probe against vidyut-prakriya at commit
    `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, over the corpus grown by
    √bhuj: **zero differences across all 2844 cells / 3338 forms / 67
    roots**, with both negative controls (`entry` and `form`) verified
    failing first. 1.3.66 is √bhuj's only new `Rule`, and it is a
    root-keyed pada assignment structurally identical to 1.3.72's, which
    this engine already implements.
    **√ric and √vic** needed no new sūtra, but 8.2.30 *coḥ kuḥ* needed more
    than the one-line guard widening it looked like: they are c-final, and
    the rule was hardcoded to a single `j` → `g` pair — its match read `j`
    alone AND its substitute was a literal `'g'`, while its comment claimed
    a 1.1.50 *sthāne'ntaratamaḥ* nearest-velar substitution (voicing and
    aspiration preserved) that the code did not implement. Widening the
    match alone would have reached the right surface (`riRakti`, since
    8.4.55 *khari ca* devoices the resulting `g` to `k` before `ti`) but
    through a wrong intermediate (`riRagti`), so both match and substitute
    now read one `kutva_of` map (cu → ku) instead — the substitute *is* the
    map, not a case split. That fix exposed 8.2.39 *jhalāṁ jaśo'nte* as
    narrower than it looked: its three-literal guard (`t`/`z`/`D`) had
    never had to classify a voiceless word-final velar, because no curated
    root had produced one before √ric and √vic did. 8.2.39 now reads a
    `jashtva_of` map on both sides too, plus a no-op guard for the table's
    fixed points, so √ric's and √vic's declined laṅ prathama and madhyama
    eka reach `ariRag`/`avinag` (jaśtva-voiced), with 8.4.56 *vā'vasāne*
    supplying the optional `ariRak`/`avinak` — the same √bhañj-pattern fork
    √yuj's `ayunag`/`ayunak` already witnesses. Rudhādi 7d then curated
    eight of the nine
    remaining reachable non-ubhayapadī roots — √śiṣ, √und, √añj, √tañc,
    √vij, √vṛj, √pṛc and √vid — on the audited numbers alone, with no new
    sūtra: each exercises machinery already in the pipeline — 6.4.23 *śnān
    nalopaḥ* then 6.4.111 *śnasor allopaḥ* for √und, √añj and √tañc (√und
    additionally taking 6.4.72 *āṭ* with 6.1.90 *āṭaś ca*), 8.2.30 *coḥ
    kuḥ* (kutva) for √añj, √tañc and √pṛc, 8.4.1 *raṣābhyāṁ no ṇaḥ* (ṇatva)
    for √vṛj and √pṛc, 8.4.41 and 8.2.41 for √śiṣ, and 8.4.65 *jharo jhari
    savarṇe* for √vid — plus two SLP1 surface collisions, which number
    keying makes moot, rather than needing anything new. (vidyut-prakriya
    credits 6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti* for √und's `unad → und`
    step; this engine rejects that credit, does not implement 6.4.24 at
    all, and pins the rejection in `tests/trace/`.) That left √tṛh as the
    ninth and only reachable non-ubhayapadī root still out, deferred to
    slice 7e behind three sūtras the engine did not implement: 7.3.92
    *tṛṇaha im* (the *im* augment), 8.2.31 *ho ḍhaḥ* and 8.3.13 *ḍho ḍhe
    lopaḥ* — see the 7e paragraph above for what curating it actually
    took.
    Rudhādi 7f then curated the last two reachable ubhayapadī roots,
    taking the gaṇa to **twenty-four**: √chid (`07.0003 Ci\di~^r`) and
    √chṛd (`07.0008 u~Cfdi~^r`), behind two new sūtras — 6.1.73 *che ca*
    (the tuk augment before a `C` after a short vowel) and 8.4.40 *stoḥ
    ścunā ścuḥ* (the ścutva that follows it) — without which their laṅ
    cells would surface `aCinat` for `acCinat`. Both are pinned in both
    padas. √bhuj (`07.0017 Bu\ja~`)
    is the twenty-fifth entry, and the √bhuj/1.3.66 slice curated it: 1.3.66
    *bhujo'navane* is implemented as an unconditional ubhayapada assignment
    (`PadaAssignment::UbhayapadaAnavane` → `Tag::Anavane`, so the trace
    credits 1.3.66 and can never reach 1.3.72), with *anavane* — the
    **sense** restriction the sūtra actually imposes — recorded as an
    unimplemented sense restriction on 1.3.72's own precedent, since neither
    engine models sense. **25 curated — no rudhādi entry remains out.**
    Rudhādi is the first gaṇa curated at its full dhātupāṭha strength.
    √indh's pada was **verified, not inferred from its
    ñi**: `YiinDI~\`'s ñi it-marker is one of the two things 1.3.72 reads,
    which would have made the root ubhayapadī alongside √rudh, so it was
    checked against vidyut-prakriya — which derives √indh in ātmanepada
    only, against a `~^r` control (√rudh) that does derive both padas — and
    the root's own anudātta settles its pada by 1.3.12 *anudāttaṅita
    ātmanepadam*. śnam is the engine's first **infix**: unlike every other
    vikaraṇa it is not a suffix, and the pipeline's fixed
    `[AGAMA, ABHYASA, ANGA, SHAP, ENDING]` slots have
    nowhere to put one, so 3.1.78 splits the root across `ANGA` and `SHAP`
    instead — `terms[SHAP].text` for rudhādi is śnam followed by the root's
    own tail, not the vikaraṇa alone (`kft` → `[kf, nat, ti]`); see the
    "REPRESENTATION" note on 3.1.78 in `tinanta/vikarana.rs` and the caveat
    in `tinanta/terms.rs`. 8.4.53 *jhalāṁ jaś jhaśi* was restored in 7a,
    with `kfndDi` as its witness, after `9fa8e5f` removed it as
    unreachable — 8.2.25 *dhi ca* bled every path that used to reach it, but
    √kṛt's stem-final `t` (not an `s`) is genuinely jaśtva's; 7b generalised
    its guard from the one shape 7a's witnesses reached it through (a
    word-final `Di`) to the sūtra's own condition — any jhal before any
    jhaś, anywhere in the word — which is what carries √piṣ's loṭ madhyama
    eka to `piMqQi` (`piRqQi` after 8.4.58) and lets √indh's mid-word `D`s
    reach the rule at all. 7b's own four sūtras are one per family: 8.2.30
    *coḥ kuḥ* (√bhañj's `j` → `g`), 8.4.41 *ṣṭunā ṣṭuḥ* and 8.2.41 *ṣaḍhoḥ
    kaḥ si* (√piṣ), and 8.2.40 *jhaṣas tathor dho'dhaḥ* (√indh). √rudh
    needed no new phonology of its own: 1.3.72 aside, the ubhayapada slice
    only widened 8.2.39 *jhalāṁ jaśo'nte*'s guard by exactly one arm (`D`),
    which is what makes `aruRad` derivable — and, through 8.2.75 *daś ca*'s
    own `ends_with('d')` guard, the `aruRaH` branch too. (That `t`/`z`/`D`
    three-literal guard was not the end of the story: the 8.2.30/8.2.39
    generalization slice later replaced it with a `jashtva_of` map, once
    √ric and √vic exercised a shape it could not classify — see above.)
    The vikalpa
    set is unchanged at **seven** rules, in pipeline order: 7.1.35, 3.4.111,
    6.4.107, 8.2.74, 8.2.75, 8.4.65, 8.4.56 — 7b is the first gaṇa slice
    since the `vikalpa` flag landed (`53e03e7`) to add none, and the
    ubhayapada slice adds none either: 1.3.72 is deliberately **not**
    optional, because a root's two padas are two cells, not two branches of
    one cell; kryādi and svādi predate the flag rather than having declined
    to use it. Four
    orderings in the tripādī are deliberate, and they differ in what they
    pin. **8.2.74 and 8.2.75 above 8.2.73**, against sūtra order, are
    *derivation* constraints — both replace the dhātu's own final, and
    8.2.73 manufactures a `d` from that final, so 8.2.74 run below it would
    find `d` where it needs `s` and never derive `ahinaH`; reversing 8.2.74
    and 8.2.73 was tried and empirically fails four tests, and the order is
    pinned by `shnams_ru_fires_on_the_dhatus_own_final` plus
    `tinanta_rule_order_is_pinned` (7b moved 8.2.75 above 8.2.73 for the
    same structural reason, which made its `p.log` read unreachable and let
    it be deleted). Since juhotyādi 3f, 8.2.75 carries no gaṇa test (√kit's
    *acikeH*), and since 3f2 neither do 8.2.73 and 8.2.74 (√bhas's *abaBat*,
    *abaBaH*). **8.2.41 below
    8.2.23** is a *derivation* constraint too, and the sharpest one this
    slice adds: at laṅ madhyama eka the ending is a bare `s`, and 8.2.23
    *saṁyogāntasya lopaḥ* elides it before 8.2.41 can see it, so the cell
    reduces exactly as laṅ prathama eka does. Reversed, √piṣ surfaces
    `apinak` instead of `apinaq`/`apinaw` — a real-looking form that splits
    madhyama eka from prathama eka and that no guard test would flag; only
    `shadhoh_kah_si_declines_when_8_2_23_ate_the_s_first` and
    `apinaq_trace_pins_8_2_23_above_8_2_41` catch it.
    **8.4.41 above 8.4.53** is sūtra order, and it is
    load-bearing *as this engine implements them*: 8.4.41's trigger set is
    narrowed to `z` alone, so reversed, 8.4.53's `z → q` jaśtva consumes
    the trigger first and √piṣ stalls at `piMqDi`. Widening that trigger to
    the ṭ-varga stops the sūtra also names (`w W q Q R`) would restore
    convergence and make the placement sūtra order only. **8.4.65 above
    8.4.56**, by contrast, is only a *trace-order* constraint — both
    orderings derive the same six forms for the cell that exercises them,
    and the wrong order only changes which intermediate form each optional
    branch's trace passes through
    (`krntat_trace_shows_savarna_elision_above_pausal` in
    `crates/panini/tests/trace/rudhadi.rs` is the sole pin; no surface-form golden
    catches a reversal).
    tanādi (gaṇa 8, vikaraṇa the bare `u` of 3.1.79) is now **complete** —
    slice 8a curated nine of its ten dhātupāṭha rows (√tan, √san, √kṣaṇ,
    √kṣiṇ, √ṛṇ, √tṛ and √ghṛ, all seven ubhayapadī by 1.3.72, plus √van and
    √man, both ātmanepadī by 1.3.12), taking the curated set from 67 to
    76 roots; slice 8b then curated √kṛ (`08.0010`), the tenth and last
    row, the one root 3.1.79 itself names, behind three new root-keyed
    rules — 6.4.110 *ata ut sārvadhātuke*, 6.4.108 *nityaṁ karoteḥ* and
    6.4.109 *ye ca* — taking the curated set to **77** roots and closing
    the gaṇa at 10/10. See
    `docs/ARCHITECTURE.md`'s tanādi paragraph and
    `docs/superpowers/specs/2026-08-30-tanadi-gana-design.md` for the rule
    analysis.
  and by the ordered-trace test (`crates/panini/tests/trace/`), which pins
  rule order. Surface forms and trace order there are the source of truth;
  sūtra ids/names in traces must match the cited reference. In practice that
  reference is vidyut-prakriya's machine-readable `data/sutrapatha.tsv`
  (ashtadhyayi.com is a JS single-page app that cannot be fetched
  programmatically), and that is what specs, plans, and verification in this
  repo actually check ids/names against.
  Each gaṇa slice also runs a **whole-corpus cross-implementation audit**
  against that same vendored checkout, comparing derivation sets cell by cell.
  The harness is `tools/audit/panini_full_audit.rs`; it is a `vidyut-prakriya`
  example rather than a workspace member (it depends on both engines), so
  `tools/audit/README.md` covers copying it into a vidyut checkout and running
  it, including the negative controls that must pass before a zero-difference
  result means anything. It lived out-of-repo until 2026-08-16, which cost
  three slices a from-scratch rebuild apiece (7b: 1728 cells, 1941
  forms, zero differences; the ubhayapada slice audited its own addition —
  √rudh's 72 cells, split per pada via `Tinanta::builder().pada(...)`, at
  vidyut commit `8da2f90`, plus the negative that vidyut derives √indh in
  ātmanepada only against √rudh as the `~^r` control — and the corpus it sat
  in then stood at 1872 cells and 2114 forms). The pada audit ran the harness
  after it was already committed in-repo, needing no rebuild, and its own
  full-corpus run found the same **zero differences across 1872 cells / 2114
  forms / 49 roots** at vidyut commit `8da2f90`, with both negative controls
  verified failing first. **Slice 7c re-ran that same committed harness over
  the grown corpus: zero differences across 2160 cells / 2496 forms / 53
  roots**, at vidyut commit `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, with
  the `entry` negative control **verified failing first** — exit 1, 36 √bhū
  cells flagged. **The 8.2.30/8.2.39 generalization slice ran it twice.**
  The first run, at the
  vidyut commit, found four differing cells — √ric's and √vic's declined
  laṅ **parasmaipada** prathama and madhyama eka — the first time this
  harness caught a real defect in real code rather than only the synthetic
  `entry` control; that is what exposed 8.2.39's guard as too narrow. After
  8.2.39 was generalised alongside 8.2.30, the second run came back clean:
  **zero differences across 2304 cells / 2654 forms / 55 roots**, with
  `entry` verified failing first both times. **Rudhādi 7d re-ran the same
  committed harness once more, at the same vidyut commit `8da2f90`, over
  the corpus grown by its eight roots: zero differences across 2592 cells
  / 3014 forms / 63 roots**, with `entry` verified failing first — the
  first of these runs with no `Rule` diff behind
  it at all: the eight roots derive on the rules already in the pipeline.
  **Rudhādi 7e re-ran the same committed harness once more, at the same
  vidyut commit `8da2f90`, over the corpus grown by √tṛh: zero differences
  across 2628 cells / 3057 forms / 64 roots**, with both `entry` and
  `form` negative controls verified failing first, and the first of these
  runs with new `Rule`s behind it: 7.3.92, 8.2.31 and 8.3.13.
  **Rudhādi 7f re-ran the same committed harness once more, at the same
  vidyut commit `8da2f90`, over the corpus grown by √chid and √chṛd: zero
  differences across 2772 cells / 3259 forms / 66 roots**, with both
  `entry` and `form` negative controls verified failing first, and, like
  7e's run, one with new `Rule`s behind it: 6.1.73 and 8.4.40 this time,
  where 7e's were 7.3.92, 8.2.31 and 8.3.13. **The √bhuj/1.3.66 slice
  re-ran the same committed harness once more, at the same vidyut commit
  `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, over the corpus grown by
  √bhuj: zero differences across 2844 cells / 3338 forms / 67 roots**,
  with both `entry` and `form` negative controls verified failing first —
  the record until tanādi 8a (below). Its one new `Rule`, 1.3.66, is a root-keyed pada
  assignment structurally identical to 1.3.72's already-implemented one —
  the same pattern the pipeline already carries, applied to a second
  root-list. **Tanādi 8a's own cross-implementation audit** ran the same
  committed harness once more, at the same vidyut commit
  `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, over the corpus grown by its
  nine curated roots: **zero differences across 3420 cells / 4321 forms /
  76 roots**, with the `entry` negative control verified failing first
  (exit 1, 36 √bhū cells, `Bavati` vs `paWati`) — the record until tanādi
  8b (below). It
  did not come back clean on the first pass: an initial run found four
  differing cells, all `08.0005` (√ṛṇ, `fR`) laṅ uttama-puruṣa dvi/bahu in
  both padas, diagnosed and fixed in commit `88cae65` before the clean
  re-run above. Two new `Rule`s are behind it — 3.1.79 (the bare `u`
  vikaraṇa) and 7.3.86's new vikalpa arm (the gaṇa's own guṇa/aguṇa
  fork) — plus two structural engine changes the divergence exposed: the
  asaṁyogapūrva helper (`terms.rs`, renamed `shnu_asamyogapurva` →
  `vikarana_u_asamyogapurva` to read tanādi's bare `u` as well as svādi's
  `nu`) reads only surface characters, so evaluated after 6.1.90 it could
  no longer tell a genuinely non-conjunct `u` (fR's) from a guṇa'd
  conjunct one (arR's) — 6.1.90's āṭ-vṛddhi ekādeśa merges the augment
  into both the same way, leaving the same three characters (`rR`) either
  way — and the fix moved 6.4.106/6.4.107 themselves ahead of that
  ekādeśa instead, so the helper reads the aṅga before the merge erases
  the distinction; and `run_pipeline` gained a convergent-fork collapse (dedup on
  identical live-branch text, first/declined branch kept), needed once
  7.3.86's new vikalpa arm could put a guṇa'd and an āṭ-vṛddhi'd branch on
  the same surface (`A+fR` and `A+arR` both → `ArRot`). **Tanādi 8b's own
  cross-implementation audit** ran the same committed harness once more,
  at the same vidyut commit `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`,
  over the corpus grown by √kṛ, the gaṇa's tenth and last root: **zero
  differences across 3492 cells / 4399 forms / 77 roots**, with the
  `entry` negative control verified failing first — the record until
  juhotyādi 3a's own audit (`tools/audit/README.md`'s 2026-09-07 entry,
  3564 cells / 4483 forms / 79 roots), itself superseded by juhotyādi 3b's
  own audit (`tools/audit/README.md`'s 2026-09-09 entry, 3636 cells / 4595
  forms / 81 roots), and that by juhotyādi 3c's (`tools/audit/README.md`'s
  2026-09-12 entry, 3852 cells / 4823 forms / 85 roots), and that by juhotyādi
  3c2's (`tools/audit/README.md`'s 2026-09-29 entry, 3924 cells / 4926 forms /
  87 roots), and that by juhotyādi 3d's (`tools/audit/README.md`'s 2026-09-29
  entry, 4176 cells / 5208 forms / 93 roots), and that by juhotyādi 3d2's
  (`tools/audit/README.md`'s 2026-09-30 entry, 4212 cells / 5249 forms / 94
  roots), and that by juhotyādi 3e's (`tools/audit/README.md`'s 2026-10-01
  entry, 4428 cells / 5486 forms / 97 roots), and that by juhotyādi 3f's
  (`tools/audit/README.md`'s 2026-10-01 entry, 4572 cells / 5655 forms / 101
  roots), and that by juhotyādi 3f2's (`tools/audit/README.md`'s 2026-10-01
  entry, 4608 cells / 5699 forms / 102 roots).
  Three new `Rule`s are behind it, all root-keyed to √kṛ and all in
  `guna.rs` — 6.4.110 *ata ut sārvadhātuke*, 6.4.108 *nityaṁ karoteḥ* and
  6.4.109 *ye ca* — plus one engine change with no `Rule` of its own:
  8.2.79 *na bhakurchurām* is modelled as a named exclusion guard inside
  8.2.77 *hali ca*'s own `apply` (`tripadi.rs`), rather than as a
  separate rule, so 8.2.77's lengthening declines on √kṛ's `kur` aṅga and
  vidyut-prakriya's own log still records 8.2.79 on every `kur` cell, which
  is why the forms agree. See
  `tools/audit/README.md`'s own recorded result for the full breakdown.
  Those
  totals are asserted by the harness itself rather than reported from
  whatever it happened to enumerate, so a corpus that grows without the
  harness being updated fails loudly instead of quietly auditing a subset.
  The harness resolves each root
  to a `data/dhatupatha.tsv` entry by its **dhātupāṭha number** (`07.0016` for
  √bhañj), which is `Dhatu::dhatupatha` and the root's identity in this repo.
  That closed the one circularity this audit used to carry: selection
  previously required vidyut to reproduce **this engine's own pinned laṭ
  prathama eka form**, so for a root whose new sūtra shaped exactly that cell
  — √bhañj's `Banakti`, √piṣ's `pinazwi`, √indh's `indDe` — the anchoring cell
  was the one cell the audit could not independently validate. The numbers
  themselves are held honest in-repo by `dhatupatha_numbers_resolve_upstream`,
  which it-strips each vendored upadeśa (1.3.2, 1.3.5, 1.3.3, then 6.1.64 and
  6.1.65) and compares it against the stored `code` — an assertion that cannot
  be satisfied by copying back our own choice, unlike matching on number or
  artha alone (upstream has 8- and 15-way artha collisions).
  Two comments inside `crates/panini-prakriya/src` still carry pre-7c
  figures — `controller.rs:152` and `tinanta/guna.rs:1233` (drifted from
  the `:130`/`:943` this ledger previously cited, as later slices added
  code above them; confirmed by re-reading each construct at its current
  line rather than trusting the old numbers to have held) cite the corpus
  size as 1872/1864-of-1872, now seven slices further stale: the corpus
  stood at 2304/2296-of-2304 as of the 8.2.30/8.2.39 slice, stood at
  2592/2584-of-2592 as of rudhādi 7d, stood at 2628/2620-of-2628 as of
  rudhādi 7e, stood at 2772/2764-of-2772 as of rudhādi 7f, stood at
  2844/2836-of-2844 as of the √bhuj/1.3.66 slice — the same 8
  cells 6.4.107 always fired on through all of those slices
  (`key_count("6.4.107") == 8` back then, pinned in
  `derivation_set_shape_matches_the_audited_numbers`,
  `crates/panini/tests/paradigm/main.rs`), unmoved by 7d, 7e, 7f or
  √bhuj/1.3.66 since 6.4.107 concerned only svādi's √hi and √ri — stood at
  3420 cells as of tanādi 8a, and now
  stands at 3492 cells as of tanādi 8b, where `controller.rs:152`'s own
  claim ("only 8 cells fire it at all") is no longer merely
  corpus-size-stale but flatly wrong: tanādi's bare `u` is asaṁyogapūrva
  for every one of its nine 8a-curated roots (√kṛ, 8b's own root, does not
  add to this count — 6.4.108 empties its `u` first), so 6.4.107 now fires
  on **72**
  cells across eleven roots (`key_count("6.4.107") == 72`, the same
  test), not 8 — the "8 cells" figure was never re-derived when the gaṇa
  landed. `guna.rs:1233`'s own claim ("1872 goldens move") stays stale
  only in the ordinary corpus-size sense, not wrong in kind: 4608 goldens
  would move today. Neither comment was touched by tanādi 8a or 8b, consistent
  with every slice since 7c. Rudhādi 7d touched neither comment — its one permitted
  engine-comment edit is the comment above
  `vrddhi_of_ac_vowels_all_arms` in `tinanta/sound.rs`. Rudhādi 7e, rudhādi
  7f and the √bhuj/1.3.66 slice touched neither comment either. The corpus
  stands at 3564 cells as of juhotyādi 3a (`guna.rs:1233`'s claim now
  anchored at `guna.rs:1472`, `controller.rs:152`'s at `controller.rs:153`),
  and this slice touched neither comment either. Juhotyādi 3b touched
  neither comment either: `controller.rs:153`'s anchor is unchanged, and
  `guna.rs:1472`'s has drifted further, to `guna.rs:1666` (3b's `guna.rs`
  additions — 6.4.82's widening, 6.4.77's iyaṅ arm and 6.4.115 — all land
  earlier in the file, above this test; 7.4.59 and 7.4.60 live in
  `abhyasa.rs` and cannot move a `guna.rs` line). The corpus
  stands at 3636 cells as of juhotyādi 3b. Juhotyādi 3c touched neither
  comment either; the corpus stands at 3852 cells as of 3c.
  (`controller.rs:153`'s anchor unchanged — re-confirmed empty
  `git diff main -- crates/panini-prakriya/src/controller.rs` — and
  `guna.rs:1666`'s drifted further, to `guna.rs:1939`: Task 9's own fix
  round added four lines to `guna.rs`'s 6.1.78 justification, which sits
  above it, after the line was first measured at `guna.rs:1935`). Juhotyādi 3c2 touched neither
  comment either; the corpus stands at 3924 cells as of 3c2 (`guna.rs:1939`'s claim now
  anchored at `guna.rs:2162`, `controller.rs:153`'s at `controller.rs:206`:
  3c2's `Rule.bars` field and its plumbing moved the latter, and 3c2's four
  rules in `guna.rs` moved the former; both lines measured by grep at this
  commit). Juhotyādi 3d touched neither comment either; the corpus stands at
  4176 cells as of 3d (`guna.rs:2162`'s claim now anchored at `guna.rs:2226`,
  `controller.rs:206`'s at `controller.rs:206`: 3d added 7.1.102 and 6.1.77's
  aṅga arm above it in `guna.rs`, and 3d's comment sweep reflowed the 6.4.82
  and 6.1.77 comments above it too, while nothing in 3d touched
  `controller.rs`; both lines measured by grep at this commit). Juhotyādi 3d2 touched neither
  comment either; the corpus stands at 4212 cells as of 3d2 (`guna.rs:2226`'s
  claim now anchored at `guna.rs:2228`, `controller.rs:206`'s unchanged at
  `controller.rs:206`: 3d2's 6.4.78 work grew the 6.1.78 comment in `guna.rs`
  by two lines above the test, while `controller.rs` is unchanged from 3d,
  so the 3d entry's `controller.rs:206` was correct; both lines measured by
  grep at this commit). Juhotyādi 3e touched neither comment either; the corpus
  stands at 4428 cells as of 3e (`guna.rs:2228`'s claim now anchored at
  `guna.rs:2384` (moved from 2376 by the final-review comment on 7.3.87's order), `controller.rs:206`'s at `controller.rs:206`: 3e's 7.3.87 rule
  and its tests landed in `guna.rs` above the test, while `controller.rs` is
  unchanged; both lines measured by grep at this commit). Juhotyādi 3f touched neither comment either; the corpus stands at 4572 cells as of 3f (`guna.rs:2384`'s claim anchored at `guna.rs:2386`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit). Juhotyādi 3f2 touched neither comment either; the corpus stands at 4608 cells as of 3f2 (`guna.rs:2386`'s claim anchored at `guna.rs:2433`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit). A third,
  `tinanta/tripadi.rs`'s comment on 8.2.30 (formerly the one calling √bhañj
  rudhādi's one cu-final curated root), was **not** left stale the same
  way: the 8.2.30/8.2.39 generalization slice rewrote it in place, since
  generalising that rule past a single hardcoded root was the whole point
  of that slice, and its diff to `panini-prakriya` was never meant to stay
  empty the way 7c's was. The remaining two were left as-is deliberately by
  7c: 7c's central claim was a byte-identical, engine-untouched slice, and
  its mutation gate's validity depended on that package's diff staying
  empty, so no comment inside it was touched even to fix drift, at the
  time. This note is the record of that deferral, now down to two
  comments.
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, not as a
  branch inside `derive`. `TINANTA_RULES` is a list of eight stage arrays,
  each living in its own file under `crates/panini-prakriya/src/tinanta/`; add
  the rule to the stage its pipeline position falls in, and add its id to
  `tinanta_rule_order_is_pinned` in the same position. Which stage a rule
  belongs to is decided by its position relative to **3.1.68**, not by its
  sūtra family: rules before
  3.1.68 address the ending as `ENDING_PRE_SHAP` (index 3), rules after it as
  `ENDING` (index 4), and `terms[SHAP].text` may be empty for adādi. Two
  permanent, usually-empty slots — `AGAMA` (0) and `ABHYASA` (1) — precede
  the aṅga; see `tinanta/terms.rs`. Per-rule guard tests go beside the rule
  in its stage
  file; tests asserting a surface form or trace go in
  `tinanta/derivation_tests.rs`. **Write a per-rule guard test where the
  rule's precondition can be built directly on a hand-built `Prakriya`.
  Where it cannot — because only an upstream rule chain produces that
  state — cite the covering derivation or trace test in the rule's own
  comment instead.** That is narrower than "every rule gets a guard test",
  and it is not the blanket exemption 7a's deferred #5 asked for ("per-rule
  guard tests for tripādī rules are not achievable"): `tripadi.rs` carries
  nineteen of them today, including
  `jhalam_jasho_ante_fires_on_any_pada_final_jhal_jashtva_of_resolves` and
  `va_avasane_fires_only_on_a_pada_final_jhal`. Whole-word scope is not what
  blocks a guard test; an unconstructible precondition is. `derive` carries
  no grammar branches: the only gana-conditioned logic there is aṅga tagging
  (`Tag::Adadi` &c.), which feeds the guarded rules rather than substituting
  for them. Fixtures shared across `crates/panini/tests/*.rs`
  integration-test binaries (e.g. `CELLS`, `LAKARA_BY_NAME`) go in
  `crates/panini/tests/common/mod.rs`, `mod`-included by each file that
  needs them — do not redefine them per test file.
- **Optional (*vikalpa*) rules set `Rule.vikalpa = true`.** `run_pipeline`
  forks there: it clones each live branch, applies to the clone, and keeps
  the clone only if `apply` returned true, so a rule that declines its own
  guard forks nothing. The declined branch keeps its index and the applied
  clone is inserted immediately after it, which is why index 0 of a
  derivation is always what the engine would have produced with no optional
  rules at all. `derive` therefore returns `Vec<Prakriya>`, and a cell may
  have more than one valid form. Add an optional rule exactly as any other —
  in its stage file, with its id in `tinanta_rule_order_is_pinned` in
  position — and also add it to
  `exactly_the_pinned_vikalpa_rules_are_optional`, which pins the whole
  optional set by id. **Eleven rules are optional today, in pipeline order:
  7.1.35, 3.4.111, 7.3.86, 6.4.117, 6.4.116, 6.4.115, 6.4.107, 8.2.74, 8.2.75, 8.4.65, 8.4.56 —
  6.4.115 landed in slice 3b, the engine's ninth vikalpa rule, root-keyed to
  √bhī and forking across all four lakāras (a kṅit-sārvadhātuka fork; loṭ
  is where it stacks with 7.1.35). Slice 3c2's 6.4.117 and 6.4.116 are the
  tenth and eleventh.** (7.3.86
  is the vikalpa entry only — its *nitya* entry, just above it in the
  pipeline, is not optional.) 7.1.35 and
  8.4.56 can both fire on one derivation, stacking into a three-branch
  cell — loṭ prathama eka forks twice, giving `Bavatu` / `BavatAd` /
  `BavatAt`. Eight rudhādi roots — √kṛt, √rudh, √bhid, √kṣud, √tṛd, √und,
  √chid and √chṛd — each
  stack three of the eleven (7.1.35,
  8.4.65, 8.4.56) on their own loṭ parasmaipada cells, and tanādi's
  kziR/fR/tfR/GfR stack a different three (7.1.35, 7.3.86, 8.4.56) on
  theirs — five branches at
  prathama eka, six at madhyama eka
  (`kfndDi` / `kfnDi` / `kfnttAd` / `kfntAd` / `kfnttAt` /
  `kfntAt`, and `rundDi` / `runDi` / `rundDAd` / `runDAd` / `rundDAt` /
  `runDAt`, and likewise for √bhid, √kṣud, √tṛd, √und, √chid and √chṛd) —
  because 8.4.56 only
  reaches the two tātaṅ (7.1.35) branches,
  not the two vowel-final ones. √yuj, ubhayapadī like √bhid, √kṣud and
  √tṛd (its own 7c cohort) but
  not dental-final, stops at three forms in the same two cells
  (`yunaktu`/`yuNktAd`/`yuNktAt`, `yuNgDi`/`yuNktAd`/`yuNktAt`): 8.2.30 *coḥ
  kuḥ* replaces its stem-final palatal `j` with the velar `g` (8.4.55 *khari
  ca* devoices that to `k` before the `t` of tātaṅ, which is where `yuNktAd`'s
  `k` comes from), so the junction 8.4.65 would need is velar against dental
  — `g` + `D`, then `k` + `t` — and never savarṇa the way the dental-final
  roots' `d` + `D` and geminate `t` + `t` are. 8.4.65's site never arises. See
  `docs/ARCHITECTURE.md`'s branch-count
  paragraph for the full accounting.
- **An optional rule's position relative to its consumers depends on what its
  mutation does to the predicates they read — the operative question is not
  "does a consumer read what I wrote?" but "does my mutation make the
  predicate lie?".** Nothing enforces either direction.
  - If the mutation **destroys the evidence** for a predicate without
    changing the underlying grammatical fact, the rule must sit **after**
    every such consumer, or a consumer placed below it would be right on one
    branch and wrong on the other — surfacing as half a paradigm being wrong
    with both halves individually plausible. 6.4.107 leaves
    `terms[SHAP].text == "n"` for svādi and `""` for tanādi, which
    invalidates two predicates: `vikarana_u_asamyogapurva` (renamed from
    `shnu_asamyogapurva`; its first guard now matches SHAP text of `"nu"`
    (svādi) or `"u"` (tanādi)) and
    `sound_before_ending` (which reads the last char before the ending — `u`
    before the mutation, something else after) — the vikaraṇa *is* still
    śnu or tanādi's bare `u`, but
    nothing downstream can tell any more. Every rule that reads the
    vikaraṇa's u-bearing
    text must precede it — 6.4.87 and 6.4.106 via `vikarana_u_asamyogapurva`,
    6.4.77, which open-codes the same `text == "nu"` test, and 6.1.77, which
    open-codes the tanādi-specific `text == "u"` test — and all four do.
    `sound_before_ending`'s one consumer below 6.4.107, 6.4.101 (`her DiH`,
    `crates/panini-prakriya/src/tinanta/adesha.rs`), is the exception a
    provably disjoint guard covers: it requires `ENDING.text == "hi"`, which
    6.4.107 already excludes by requiring an m- or v-initial ending, so the
    two rules never contend and 6.4.101 is safe where it sits.
  - If instead the mutation **changes the fact itself**, the rule must sit
    **before** every such consumer, so they read the new value rather than a
    stale one. 7.1.35 replaces the ending `tu`/`hi` with tātaṅ, and the
    ending genuinely is no longer `hi` — a consumer below gets the *right*
    answer, one above gets a stale one. 3.1.83 *halaḥ śnaḥ śānac ca*, 6.4.105
    *ato heḥ*, and 6.4.106 *utaś ca* all read `ENDING`/`ENDING_PRE_SHAP`'s
    text directly, and 7.3.84's second (ending-relative) application reads
    its ṅitva, so all four must sit below 7.1.35 to see the tātaṅ shape
    rather than the pre-mutation `hi` — kryādi's tātaṅ branch would surface
    `kliSAnatAt` instead of `kliSnItAt` if 3.1.83 ran first. 7.1.35 is
    ordered above all of them, at the end of the tiṅ stage, and nothing
    enforces that but the `kliSnItAt` trace pin.
- **7.3.84 and 1.2.4 each appear twice in `TINANTA_RULES`, by design — do not
  "deduplicate" them.** 1.4.13 *yasmāt pratyayavidhis tadādi pratyaye'ṅgam*
  makes the aṅga affix-relative, and a derivation with a live vikaraṇa has
  two affixes for these rules to apply with respect to: once for the
  vikaraṇa, once for the tiṅ ending. Svādi is where this became visible for
  7.3.84 (`Apnoti` needs the vikaraṇa-relative application; `ApnutaH` blocks
  it because `tas` is ṅit) but 1.2.4's second entry predates it. See
  `docs/superpowers/specs/2026-07-29-svadi-gana-design.md`'s "7.3.84
  *sārvadhātukārdhadhātukayoḥ*, second application" section.
- The `panini-cli` binary has a single subcommand, `check` (flags `--trace`,
  `--json`, `--out`, `--in`). There is no `derive` subcommand in v1. `--in auto`
  (the default) auto-detects the input transliteration scheme; passing an
  explicit `--in` scheme (`slp1`/`iast`/`hk`/`deva`) makes that scheme
  authoritative, overriding auto-detection.
- **When a rule's substitute set widens, re-check every downstream guard
  that enumerates literals over the sounds it can now emit, in the same
  slice.** The 8.2.30/8.2.39 generalization slice paid for skipping this:
  once 8.2.30 read `kutva_of` instead of a hardcoded `g`, its output
  alphabet gained a voiceless `k`, and 8.2.39's `t`/`z`/`D`-only literal
  guard had never seen a word-final voiceless velar because nothing had
  ever produced one. Six `NARROW GUARD` sites remain in `tripadi.rs` and
  `anga.rs` today — each is a deliberate, commented narrowing, and each is
  a standing instance of this same hazard for the next slice that widens
  what feeds it.
- **A guard that names particular roots keys on the dhātupāṭha number
  wherever the root text is ambiguous.** `ctx.dhatupatha` carries the row
  `derive` was called with, or `""` on a hand-built prakriyā, which every
  such guard declines. 6.4.87, 6.4.101 and 6.4.115 still key on `ANGA.text`
  (`hu`, `BI`), safe only because `juhotyadi_rows_are_the_twenty_five_curated_roots`
  asserts those codes stay unique; 7.4.75, 7.4.76, 6.4.116, 6.4.117, 6.4.118, 8.2.38
  and 8.2.40's *adhaḥ* key on numbers, because `03.0008` and `03.0009` share
  `hA`, and 7.4.77 (`03.0004`, `03.0005`, `03.0017`) follows that precedent. 7.4.75 (`03.0012`–`03.0014`) does too: `vij` is also `06.0009` and `07.0023`. 6.4.100 (`03.0019`, slice 3f2) does too, so a curated √ghas extends its key rather than sharing a text test. 7.4.78 keys
  on a number for a different reason: the sūtra names no root, the Kaumudī
  applies it to one row (03.0026), and `gA` is also `01.1101 gA\N`. A sūtra
  naming a class of roots becomes a saṁjñā tag decided from the number in `derive` —
  `Tag::Ghu` from `tinanta/samjna.rs`'s `GHU` — pinned to the vendored TSV
  rather than to a derivation, so its uncurated members are held too.
- **An apavāda that must stop other rules on its branch declares it in
  `Rule.bars`; never in the overridden rules' guards, and never by reading
  `p.log`.** `run_pipeline` enforces the bar per branch (a vikalpa's on its
  applied clone only), and `exactly_the_pinned_bars` requires every barred id
  to run after its barrer. 6.4.117 *ā ca hau* is the first: it changes no text,
  so without its bars 6.4.116, 6.4.113 and 6.4.112 would each still rewrite
  the `A` it keeps. 7.3.87 is the second and the first mandatory one: it changes
  no text and bars 7.3.86, so 7.3.86's guard carries no abhyasta exception.
  7.1.6's read of `p.log` for 7.1.5 is an ENABLING condition,
  not a bar, and stays as it is.

## Where things live
See `docs/ARCHITECTURE.md`.
