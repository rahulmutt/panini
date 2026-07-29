# Contributor & agent guide

## Environment
- Toolchain is pinned via `mise` (`mise install`) to rust 1.97.1. Do not install
  Rust globally.
- Tasks: `mise run build | test | lint | fmt | fmt-check | mutants | audit`.
- Optional dev/audit tooling is pinned in `mise.dev.toml`. Install it on demand:
  `MISE_ENV=dev mise install`. This provides:
  - `cargo-mutants` (mutation testing) — `mise run mutants` runs
    `cargo mutants --package panini-prakriya --test-workspace=true --timeout
    300` (the `--test-workspace` flag is required so each **mutant** run
    exercises the `panini` crate's golden paradigm/trace/roundtrip tests, not
    just `panini-prakriya`'s own unit tests — but it does NOT apply to
    cargo-mutants' own **baseline** run, which always exercises only the
    mutated package's tests regardless of the flag). The explicit, generous
    `--timeout` is required for the same asymmetry: cargo-mutants calibrates
    its per-mutant timeout from the baseline's runtime, but the baseline here
    (`panini-prakriya`'s unit tests, ~2s) is far faster than an actual mutant
    run (the full `panini` golden suite, ~95s at 1512 forms). Under a short
    cap — or auto-derived timing, which falls back to a 20s floor — a mutant
    that changes nothing detectable exceeds the cap and is recorded as a
    **timeout rather than a survivor**, so a reported zero-survivor run can
    be vacuous instead of clean. Always pass an explicit, generous timeout.
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
  (`crates/panini/tests/paradigm.rs`, 1512 forms, six gaṇas; bhvādi/divādi/
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
    see `docs/superpowers/specs/2026-07-29-svadi-gana-design.md`)
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
  feeds the guarded rules rather than substituting for them.
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
