# Contributor & agent guide

## Environment
- Toolchain is pinned via `mise` (`mise install`) to rust 1.97.1. Do not install
  Rust globally.
- Tasks: `mise run build | test | lint | fmt | fmt-check | mutants | audit`.
- Optional dev/audit tooling is pinned in `mise.dev.toml`. Install it on demand:
  `MISE_ENV=dev mise install`. This provides:
  - `cargo-mutants` (mutation testing) — `mise run mutants` runs
    `cargo mutants --package panini-prakriya --test-workspace=true --timeout
    2400 -j 4` (the `--test-workspace` flag is required so each **mutant** run
    exercises the `panini` crate's golden paradigm/trace/roundtrip tests, not
    just `panini-prakriya`'s own unit tests — but it does NOT apply to
    cargo-mutants' own **baseline** run, which always exercises only the
    mutated package's tests regardless of the flag). The explicit, generous
    `--timeout` is required for the same asymmetry: cargo-mutants calibrates
    its per-mutant timeout from the baseline's runtime, but the baseline here
    (`panini-prakriya`'s unit tests, ~2s) is far faster than an actual mutant
    run (the full `panini` golden suite, ~183s at 1728 cells when the mutant
    is caught in the paradigm binary and the run aborts there — but ~380s
    when it is NOT caught and the suite runs to completion; both re-measured
    in slice 7b, and ~140s / ~300s at 1620 cells before it. At the **1800**
    cells of the ubhayapada 1.3.72 slice, a standalone `mise run test` — one
    suite, no mutation campaign alongside it — measured paradigm ~207s and
    roundtrip ~240s (trace ~2s), i.e. an **uncontended** uncaught floor of
    ~450s, *more* than the ~395s a 4%-growth scaling predicts. Scale the
    floor by measurement, not by cell count). Under a cap
    that doesn't clear that uncaught-run floor — or auto-derived timing,
    which falls back to a 20s floor — a mutant that survives is recorded
    as a **timeout rather than a survivor**, so a reported zero-survivor
    run (checking only `missed.txt`) can be vacuous instead of clean:
    slice 7a's own mutation run hit this exactly, with `--timeout 300`
    producing 9 timeouts once the suite grew to 1620 cells — a cap that no
    longer had headroom over a full uncaught run, not merely over a
    caught-and-aborted one. Always pass an explicit `--timeout` with
    headroom over a full uncaught run of the workspace suite, and always
    check `timeout.txt` alongside `missed.txt`. **The cap must clear a full
    uncaught run at the parallelism you actually use, not just at `-j 1`.**
    1200s clears the ~380s uncontended floor measured at 1728 cells only
    while contention stays low: slice 7b ran the gate at `-j 16` on 24 cores
    and got **43 timeouts** where one was expected — the same vacuity,
    reached through parallelism instead of suite growth. Re-running exactly
    those at `-j 4 --timeout 2400` caught 43 of 44 in 389–449s — 2.1–2.5×
    the ~183s caught-and-aborted figure measured standalone, which is the
    only direct evidence here of what `-j 4` costs. **Nothing in this repo
    has yet measured a `-j 4` run against the 1800-cell suite.** Extrapolate
    the same 2.1–2.5× onto its ~450s uncontended floor and an uncaught
    mutant at `-j 4` lands somewhere near 950–1100s: still under the 1200s
    cap, but with a margin of tens of percent, not the 3× the 1728-cell
    figures suggest. Treat 1200s at `-j 4` as adequate-but-unverified, not
    comfortable; if a timeout appears that is not the known permanent one,
    re-run it alone before concluding anything. `cargo mutants` also
    reads `-j` from `CARGO_MUTANTS_JOBS`, so an unqualified cap can be
    defeated by the environment alone. Keep `-j` at or below 4 with the
    1200s cap, or raise the cap in step, and re-measure both the floor and
    the margin the next time the golden suite grows.
    Re-running those 9 at `--timeout 1200` resolved them into two different
    outcomes, both worth knowing about: **3 were genuine survivors** — all
    three mutants of `Context::is_tip`, whose only caller was 8.2.73's
    `is_tip() || is_sip()` guard; `dhatu_is_pada_final` plus the `s`-final
    check already selected exactly the cells that guard was trying to name,
    so it was dead weight, and both the clause and `Context::is_tip` itself
    (having no other caller) were deleted — the discipline working exactly
    as intended, on a guard the previous fix round had not thought to
    question because it looked like ordinary domain modelling, not
    redundant plumbing. **1 is a genuine, permanent timeout, not a
    survivor**: `tripadi.rs`'s ṇatva backward scan (`is_natva_target`'s
    caller) decrements a loop index with `j -= 1`; mutating that to `j /=
    1` makes `j` constant and the loop never terminates. No assertion can
    ever catch this — the mutated run never reaches one — so the 1200s cap
    itself *is* the detection mechanism, not a symptom of too short a cap.
    This is a different phenomenon from the reclassification problem above
    (a real survivor misreported as a timeout because the cap is too
    short): here the mutant genuinely does hang, at any cap, and a
    `timeout.txt` entry of this shape is the correct, permanent verdict —
    do not chase it with a bigger `--timeout` or a code change; the loop
    itself is correct, working code.
    **The ubhayapada 1.3.72 slice's own campaign did not use the 1200s
    default.** It ran by hand at `-j 4 --timeout 2400` — 522 mutants, 482
    caught, 0 missed, 39 unviable, and the one known-permanent timeout
    above — doubling the cap precisely because the ~450s uncontended floor
    times an unverified `-j 4` contention factor left no margin worth
    reading a `timeout.txt` entry against. `mise.toml`'s default was
    deliberately left at 1200 at the time, with `--timeout 2400` passed by
    hand; **that is no longer so — `mise run mutants` now runs
    `-j 4 --timeout 2400` itself**, so run the gate through the task rather
    than reconstructing the flags, and change the default only against a
    fresh measurement of the floor and the margin.
    **The pada audit measured both at 1872 cells.** Uncontended floor:
    paradigm ~205s, roundtrip ~236s, trace ~2s (uncaught total ~443s) — flat
    against the 1800-cell ~450s figure, in fact slightly below it. Campaign
    at `-j 4 --timeout 2400`: 522 mutants, 482 caught, 0 missed, 39 unviable,
    and the one known-permanent `tripadi.rs` timeout. This is the `-j 4`
    timing measurement the paragraph above calls for, taken at the suite's
    size at that point rather than the 1800 cells named there, which will
    not recur: `outcomes.json`'s per-mutant test-phase durations for the 482
    caught mutants put the median at 30.1s, p90 at 346.6s, p99 at 547.2s, and
    the max at 754.6s, with only 4 mutants over 600s and none over 1200s. The
    slowest caught mutant,
    `crates/panini-prakriya/src/tinanta/vikarana.rs:316:17`, is a near-direct
    measurement of that worst case rather than an inference from contention
    alone: it ran the golden suite to completion without being caught —
    `paradigm` passed (11 passed, 356.90s against an uncontended 205.47s
    standalone, 1.74×) and `roundtrip` passed (1 passed, 391.84s against an
    uncontended 236.36s standalone, 1.66×), with `trace` adding ~0s — and
    was caught only afterwards, by `panini-prakriya --lib`, which failed in
    0.20s. 356.90 + 391.84 ≈ 748.7s of its 754.6s total was therefore the
    golden suite running to completion without catching it, which is exactly
    the work an uncaught mutant does. The measured worst-case contention
    factor, 754.6 / 443 ≈ 1.7×, is accordingly apples-to-apples against
    the uncontended floor — both divide by an uncaught-run time, not a
    caught-and-aborted one — well under the 2.1–2.5× inferred from the
    `-j 16` re-runs above, so that extrapolation was pessimistic. 2400 kept
    real margin — roughly 3.2× the slowest observed caught mutant. 1200
    would also have sufficed in this run, since nothing exceeded it, but only
    by ~1.6× over that same 754.6s max, not the ~3× the 1728-cell figures
    showed. This run caught every mutant, so a genuinely uncaught mutant —
    one that also spends `trace`'s ~2s rather than the ~0s this one did,
    and is never caught by `--lib` — remains unobserved; but because the
    slowest caught mutant already paid the full golden-suite cost before
    `--lib` caught it, the uncaught worst case is no longer purely inferred
    from contention factors, only from the small remainder that one mutant
    leaves unmeasured. Keep `--timeout 2400` rather than dropping to 1200
    until a genuinely uncaught mutant is actually observed.
    **Slice 7c re-measured both at 2160 cells.** Uncontended floor:
    paradigm 276.99s, roundtrip 331.81s, trace 1.93s — an uncaught total of
    **610.73s**, where scaling the 1872-cell ~443s figure by cell count
    predicted ~510s. Cell count has now failed as a multiplier in both
    directions (flat from 1800 to 1872, then 38% for a 15% growth): measure
    the floor, never scale it. Campaign at `-j 4 --timeout 2400`: **522
    mutants, 482 caught, 0 missed, 39 unviable, 1 timeout** — that one being
    the known-permanent `tripadi.rs:1140:23` non-terminating-loop mutant
    described above, which is the correct verdict at any cap.
    `outcomes.json`'s per-mutant test-phase durations for the 482 caught
    mutants put the median at 46.1s, p90 at 419.0s, p99 at 679.4s and the
    max at 848.2s, with **44** mutants over 600s and none over 1200s
    (the pada audit's were 30.1 / 346.6 / 547.2 / 754.6 with only **4** over
    600s — the over-600s count jumping 4 → 44 is the number to watch here,
    not the max). **Two margins, and they answer different questions.**
    Against the worst **caught** mutant, 848.2s, *directly measured*: 2.83×
    at 2400, 1.41× at 1200. Against the worst **uncaught** run — the
    610.73s floor times the 1.70× `-j 4` contention factor the pada audit
    measured, so ~1040s, a *projection* and not a measurement: **2.31× at
    2400**, 1.16× at 1200. The uncaught figure is the one that governs the
    vacuity question, because a genuine survivor runs the suite to
    completion whereas a caught run can abort the moment it is detected —
    do not quote the caught max as if it were the margin basis, and do not
    quote the projection as if it were measured. 1200 would have passed
    this campaign too, nothing having exceeded it, but 1.16× of projected
    headroom is exactly the shape that turns a "0 missed" into a vacuous
    one on a busier machine. Keep 2400.
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
  (`crates/panini/tests/paradigm.rs`, 2304 cells, six complete gaṇas plus
  rudhādi partial — `PARADIGM`
    stays one-form-per-cell: a cell forked by an optional rule keeps its
    other forms — a second (172 cells), a third (65 cells), a fourth (1 cell,
    rudhādi's √piṣ loṭ madhyama eka) and — the loṭ parasmaipada cells of
    rudhādi's √kṛt, √rudh, √bhid, √kṣud and √tṛd, five ways tied as the
    sharpest forks in the suite — a fourth
    and fifth (prathama eka) or a fourth through sixth (madhyama eka) — in
    `ALTERNATES` (350 rows in all, so 2304 + 350 = 2654 forms total), and
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
    (gaṇa 7, vikaraṇa śnam) is **partial**, not complete — the first gaṇa
    described that way. Nine of its 25 dhātupāṭha roots are ubhayapadī
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
    padas.
    `curated_pada_agrees_with_upadesha_markers` in `panini-data` now
    re-derives all 55 verdicts from the vendored upadeśa, so the column
    cannot drift from the data that determines it. That discharges
    the **ubhayapada** deferral as such: 1.3.72 is no longer what keeps any
    root out, and the **two** ubhayapadī roots still outside the curated
    set are out for narrower, root-specific reasons, verified cell by cell
    against vidyut-prakriya.
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
    **four differing cells** — √ric's and √vic's declined laṅ prathama and
    madhyama eka, in both padas — which is what exposed 8.2.39's narrow
    guard as a real defect rather than a documented deferral: real code,
    not only the synthetic `entry` control, tripped the harness. After
    8.2.39 was generalised, the second run came back clean: **zero
    differences across all 2304 cells / 2654 forms / 55 roots**, with the
    `entry` negative control verified failing first (36 √bhū cells) both
    times.
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
    √yuj's `ayunag`/`ayunak` already witnesses. **√chid and √chṛd** need two
    sūtras the engine does not have — 6.1.73 *che ca*, the tuk augment
    before a `C` after a short vowel, and 8.4.40 *stoḥ ścunā ścuḥ*, the
    ścutva that follows it — without which their laṅ cells surface
    `aCinat` for `acCinat`. Nine reachable non-ubhayapadī
    rudhādi roots remain out — √śiṣ, √tṛh, √und, √añj, √tañc, √vij, √vṛj,
    √pṛc and √vid — each bringing machinery of its own (7.1.58 *idito num
    dhātoḥ* for √und, 6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti* for √añj and
    √tañc, and two SLP1 surface collisions, which number keying makes
    moot). √bhuj (`07.0017 Bu\ja~`) is the twenty-fifth entry and out on
    different grounds again: 1.3.66
    *bhujo'navane* forks its pada on **sense**, not on an axis this engine
    models. 13 curated + 2 uncurated ubhayapadī + 9 reachable + √bhuj = 25,
    so **12 of the 25 remain out**.
    The root count is not what keeps the gaṇa partial — thirteen is well
    past the six every completed gaṇa *after bhvādi* has here (bhvādi,
    the first, has twelve) — and neither, any longer, is 1.3.72: what
    remains is the two-sūtra gap that keeps √chid and √chṛd out, the nine
    uncurated reachable roots, and
    √bhuj's sense axis. √indh's pada was **verified, not inferred from its
    ñi**: `YiinDI~\`'s ñi it-marker is one of the two things 1.3.72 reads,
    which would have made the root ubhayapadī alongside √rudh, so it was
    checked against vidyut-prakriya — which derives √indh in ātmanepada
    only, against a `~^r` control (√rudh) that does derive both padas — and
    the root's own anudātta settles its pada by 1.3.12 *anudāttaṅita
    ātmanepadam*. śnam is the engine's first **infix**: unlike every other
    vikaraṇa it is not a suffix, and the pipeline's fixed
    `[ANGA, SHAP, ENDING]` slots have
    nowhere to put one, so 3.1.78 splits the root across the first two
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
    own `ends_with('d')` guard, the `aruRaH` branch too. The vikalpa
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
    it be deleted). **8.2.41 below
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
    `crates/panini/tests/trace.rs` is the sole pin; no surface-form golden
    catches a reversal).
  and by the ordered-trace test (`crates/panini/tests/trace.rs`), which pins
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
  cells flagged. **The 8.2.30/8.2.39 generalization slice ran it twice, and
  its second run is the current record.** The first run, at the same
  vidyut commit, found four differing cells — √ric's and √vic's declined
  laṅ prathama and madhyama eka, in both padas — the first time this
  harness caught a real defect in real code rather than only the synthetic
  `entry` control; that is what exposed 8.2.39's guard as too narrow. After
  8.2.39 was generalised alongside 8.2.30, the second run came back clean:
  **zero differences across 2304 cells / 2654 forms / 55 roots**, with
  `entry` verified failing first both times. Those
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
  Three comments inside `crates/panini-prakriya/src` still carry pre-7c
  figures — `controller.rs:130` and `tinanta/guna.rs:943` cite the corpus
  size as 1872/1864-of-1872 rather than 2160/2152-of-2160, and
  `tinanta/tripadi.rs:269` calls √bhañj rudhādi's only cu-final curated
  root, which √yuj's `j`-final stem (also reaching 8.2.30 *coḥ kuḥ*) now
  contradicts. They were left as-is deliberately: 7c's central claim is a
  byte-identical, engine-untouched slice, and the mutation gate's validity
  depends on that package's diff staying empty, so no comment inside it was
  touched even to fix drift. This note is the record of that deferral.
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, not as a
  branch inside `derive`. `TINANTA_RULES` is a list of seven stage arrays,
  each living in its own file under `crates/panini-prakriya/src/tinanta/`; add
  the rule to the stage its pipeline position falls in, and add its id to
  `tinanta_rule_order_is_pinned` in the same position. Which stage a rule
  belongs to is decided by its position relative to **3.1.68**, not by its
  sūtra family: rules before
  3.1.68 address the ending as `ENDING_PRE_SHAP` (index 1), rules after it as
  `ENDING` (index 2), and `terms[SHAP].text` may be empty for adādi. See
  `tinanta/terms.rs`. Per-rule guard tests go beside the rule in its stage
  file; tests asserting a surface form or trace go in
  `tinanta/derivation_tests.rs`. **Write a per-rule guard test where the
  rule's precondition can be built directly on a hand-built `Prakriya`.
  Where it cannot — because only an upstream rule chain produces that
  state — cite the covering derivation or trace test in the rule's own
  comment instead.** That is narrower than "every rule gets a guard test",
  and it is not the blanket exemption 7a's deferred #5 asked for ("per-rule
  guard tests for tripādī rules are not achievable"): `tripadi.rs` carries
  eighteen of them today, including
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
  optional set by id. **Seven rules are optional today, in pipeline order:
  7.1.35, 3.4.111, 6.4.107, 8.2.74, 8.2.75, 8.4.65, 8.4.56.** 7.1.35 and
  8.4.56 can both fire on one derivation, stacking into a three-branch
  cell — loṭ prathama eka forks twice, giving `Bavatu` / `BavatAd` /
  `BavatAt`. Five rudhādi roots — √kṛt, √rudh, √bhid, √kṣud and √tṛd — each
  stack three of the seven (7.1.35,
  8.4.65, 8.4.56) on their own loṭ parasmaipada cells — five branches at
  prathama eka, six at madhyama eka
  (`kfndDi` / `kfnDi` / `kfnttAd` / `kfntAd` / `kfnttAt` /
  `kfntAt`, and `rundDi` / `runDi` / `rundDAd` / `runDAd` / `rundDAt` /
  `runDAt`, and likewise for √bhid, √kṣud and √tṛd) — because 8.4.56 only
  reaches the two tātaṅ (7.1.35) branches,
  not the two vowel-final ones. √yuj, ubhayapadī like those last three but
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
    `terms[SHAP].text == "n"`, which invalidates two predicates:
    `shnu_asamyogapurva` (whose first guard is `== "nu"`) and
    `sound_before_ending` (which reads the last char before the ending — `u`
    before the mutation, `n` after) — the vikaraṇa *is* still śnu, but
    nothing downstream can tell any more. Every rule that reads śnu's `nu`
    text must precede it — 6.4.87 and 6.4.106 via `shnu_asamyogapurva`, and
    6.4.77, which open-codes the same `text == "nu"` test — and all three do.
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

## Where things live
See `docs/ARCHITECTURE.md`.
