# Contributor & agent guide

## Environment
- Toolchain is pinned via `mise` (`mise install`) to rust 1.97.1. Do not install
  Rust globally.
- Tasks: `mise run build | test | lint | fmt | fmt-check | mutants | audit`.
- Optional dev/audit tooling is pinned in `mise.dev.toml`. Install it on demand:
  `MISE_ENV=dev mise install`. This provides:
  - `cargo-mutants` (mutation testing) — `mise run mutants` runs
    `cargo mutants --package panini-prakriya --test-workspace=true --timeout
    1200` (the `--test-workspace` flag is required so each **mutant** run
    exercises the `panini` crate's golden paradigm/trace/roundtrip tests, not
    just `panini-prakriya`'s own unit tests — but it does NOT apply to
    cargo-mutants' own **baseline** run, which always exercises only the
    mutated package's tests regardless of the flag). The explicit, generous
    `--timeout` is required for the same asymmetry: cargo-mutants calibrates
    its per-mutant timeout from the baseline's runtime, but the baseline here
    (`panini-prakriya`'s unit tests, ~2s) is far faster than an actual mutant
    run (the full `panini` golden suite, ~140s at 1620 cells, when the
    mutant is caught and the run aborts early — but close to 300s when it is
    NOT caught and the suite runs to completion). Under a cap that doesn't
    clear that uncaught-run floor — or auto-derived timing, which falls back
    to a 20s floor — a mutant that survives is recorded as a **timeout
    rather than a survivor**, so a reported zero-survivor run (checking only
    `missed.txt`) can be vacuous instead of clean: slice 7a's own mutation
    run hit this exactly, with `--timeout 300` producing 9 timeouts once the
    suite grew to 1620 cells — a cap that no longer had headroom over a full
    uncaught run, not merely over a caught-and-aborted one. Always pass an
    explicit `--timeout` with headroom over a full uncaught run of the
    workspace suite, and always check `timeout.txt` alongside `missed.txt`.
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
  (`crates/panini/tests/paradigm.rs`, 1620 cells, six complete gaṇas plus
  rudhādi partial — `PARADIGM`
    stays one-form-per-cell: a cell forked by an optional rule keeps its
    other forms — a second (78 cells), a third (52 cells), and — rudhādi's
    √kṛt loṭ cells, the sharpest forks in the suite — a fourth and fifth
    (prathama eka) or a fourth through sixth (madhyama eka) — in
    `ALTERNATES` (191 rows in all, so 1620 + 191 = 1811 forms total), and
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
    ubhayapadī root, and ubhayapada (1.3.72 svaritañitaḥ) is still deferred —
    see `docs/superpowers/specs/2026-07-28-kryadi-gana-design.md`; svādi
    (gaṇa 5) is now **complete** — six roots across all four lakāras: √āp,
    √śak, √hi and √ri (parasmaipada), √aś (`Dhatu::id` `aS.5`, distinct from
    kryādi's `aS` — the first root whose `id` differs from its SLP1 `code`)
    and √ṣṭigh (`stiG`) (ātmanepada). Its vikaraṇa is śnu (3.1.73), and it is
    the first gaṇa where 7.3.84's guṇa lands on the vikaraṇa rather than the
    root: 7.3.84 now applies twice, once with respect to śnu and once with
    respect to the ending (1.4.13 makes the aṅga affix-relative), giving
    `Apnoti` against the ṅit-blocked `ApnutaH`. The other split running
    through the gaṇa is *asaṁyogapūrva* — whether śnu's `u` is preceded by a
    conjunct decides both the yaṇ alternation (6.4.87 / 6.4.77: `hinvanti`
    against `Apnuvanti`) and the hi-luk (6.4.106: `hinu` against `Apnuhi`) —
    see `docs/superpowers/specs/2026-07-29-svadi-gana-design.md`.) rudhādi
    (gaṇa 7, vikaraṇa śnam) is **partial**, not complete — the first gaṇa
    described that way. Nine of its 25 dhātupāṭha roots are ubhayapadī
    (`~^`-marked) and 1.3.72 *svaritañitaḥ* is still deferred, so √rudh,
    √bhid, √chid and √yuj are absent and the gaṇa lacks its own eponymous
    root; slice 7a lands three roots that need nothing beyond the gaṇa's own
    spine (√kṛt, √hiṃs — stored `hins` — and √khid), and 7b (√bhañj, √piṣ,
    √indh) closes what is reachable without 1.3.72. śnam is the engine's
    first **infix**: unlike every other vikaraṇa it is not a suffix, and the
    pipeline's fixed `[ANGA, SHAP, ENDING]` slots have nowhere to put one,
    so 3.1.78 splits the root across the first two instead —
    `terms[SHAP].text` for rudhādi is śnam followed by the root's own tail,
    not the vikaraṇa alone (`kft` → `[kf, nat, ti]`); see the
    "REPRESENTATION" note on 3.1.78 in `tinanta/vikarana.rs` and the caveat
    in `tinanta/terms.rs`. 8.4.53 *jhalāṁ jaś jhaśi* is restored this slice,
    with `kfndDi` as its witness, after `9fa8e5f` removed it as
    unreachable — 8.2.25 *dhi ca* bled every path that used to reach it, but
    √kṛt's stem-final `t` (not an `s`) is genuinely jaśtva's. The vikalpa
    set is now **seven** rules, in pipeline order: 7.1.35, 3.4.111, 6.4.107,
    8.2.74, 8.2.75, 8.4.65, 8.4.56. Two orderings within it are deliberate
    and against sūtra order, but differ in what they pin: **8.2.74 above
    8.2.73** is a *derivation* constraint — 8.2.74 replaces the dhātu's own
    final `s`, so run below 8.2.73 it would find `d` instead and never
    derive `ahinaH`; reversing the two was tried and empirically fails four
    tests, and the order is pinned by
    `shnams_ru_fires_on_the_dhatus_own_final` plus
    `tinanta_rule_order_is_pinned`. **8.4.65 above 8.4.56**, by contrast, is
    only a *trace-order* constraint — both orderings derive the same six
    forms for the cell that exercises them, and the wrong order only
    changes which intermediate form each optional branch's trace passes
    through (`krntat_trace_shows_savarna_elision_above_pausal` in
    `crates/panini/tests/trace.rs` is the sole pin; no surface-form golden
    catches a reversal).
  and by the ordered-trace test (`crates/panini/tests/trace.rs`), which pins
  rule order. Surface forms and trace order there are the source of truth;
  sūtra ids/names in traces must match the cited reference. In practice that
  reference is vidyut-prakriya's machine-readable `data/sutrapatha.tsv`
  (ashtadhyayi.com is a JS single-page app that cannot be fetched
  programmatically), and that is what specs, plans, and verification in this
  repo actually check ids/names against.
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
  `tinanta/derivation_tests.rs`. `derive` carries no grammar branches: the
  only gana-conditioned logic there is aṅga tagging (`Tag::Adadi` &c.), which
  feeds the guarded rules rather than substituting for them. Fixtures shared
  across `crates/panini/tests/*.rs` integration-test binaries (e.g. `CELLS`,
  `LAKARA_BY_NAME`) go in `crates/panini/tests/common/mod.rs`, `mod`-included
  by each file that needs them — do not redefine them per test file.
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
  `BavatAt`. rudhādi's √kṛt stacks three of the seven (7.1.35, 8.4.65,
  8.4.56) on its own loṭ cells — five branches at prathama eka, six at
  madhyama eka (`kfndDi` / `kfnDi` / `kfnttAd` / `kfntAd` / `kfnttAt` /
  `kfntAt`) — because 8.4.56 only reaches the two tātaṅ (7.1.35) branches,
  not the two vowel-final ones; see `docs/ARCHITECTURE.md`'s branch-count
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
