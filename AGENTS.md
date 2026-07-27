# Contributor & agent guide

## Environment
- Toolchain is pinned via `mise` (`mise install`) to rust 1.97.1. Do not install
  Rust globally.
- Tasks: `mise run build | test | lint | fmt | fmt-check | mutants | audit`.
- Optional dev/audit tooling is pinned in `mise.dev.toml`. Install it on demand:
  `MISE_ENV=dev mise install`. This provides:
  - `cargo-mutants` (mutation testing) — `mise run mutants` runs
    `cargo mutants --package panini-prakriya --test-workspace=true` (the
    `--test-workspace` flag is required so the mutation run exercises the
    `panini` crate's golden paradigm/trace/roundtrip tests, not just
    `panini-prakriya`'s own unit tests).
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
  (`crates/panini/tests/paradigm.rs`, 1080 forms; bhvādi/divādi/tudādi are
    complete across laṭ/laṅ/loṭ/vidhiliṅ × parasmaipada/ātmanepada, and adādi
    (gaṇa 2) is now **complete** — √yā/√vā/√ad (parasmaipada) and
    √ās/√vas/√śī (ātmanepada) are complete across all four lakāras
    (laṭ/laṅ/loṭ/vidhiliṅ). √ad (parasmaipada) lands the internal junction
    sandhi cartva (8.4.55); √ās (ātmanepada) lands 7.1.5 ātmanepadeṣv anataḥ
    and extends 6.1.90 āṭaś ca / 6.1.66 lopo vyor vali to the athematic
    (śap-luk'd) ātmanepada path (loṭ 1sg + optative); √vas (ātmanepada) is
    the second witness for 8.2.25 dhi ca, which elides an aṅga-final `s`
    before a Dh-initial affix (ADve, vaDve) — it replaced the 8.4.53 jaśtva
    analysis slice 5d shipped, and 8.4.53 was removed as unreachable; √śī
    (ātmanepada) closes the gaṇa and lands 7.4.21 śīṅaḥ sārvadhātuke guṇaḥ
    (guṇa despite the ṅit ending — the gaṇa's only visible guṇa), 7.1.6
    śīṅo ruṭ (Serate), and 8.3.59 ādeśapratyayayoḥ, the engine's first ṣatva
    (Seze, Sezva) — see
    `docs/superpowers/specs/2026-07-25-adadi-si-5f-design.md` for the full
    rule analysis)
  and by the ordered-trace test (`crates/panini/tests/trace.rs`), which pins
  rule order. Surface forms and trace order there are the source of truth;
  sūtra ids/names in traces must match the cited reference. In practice that
  reference is vidyut-prakriya's machine-readable `data/sutrapatha.tsv`
  (ashtadhyayi.com is a JS single-page app that cannot be fetched
  programmatically), and that is what specs, plans, and verification in this
  repo actually check ids/names against.
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, not as a
  branch inside `derive`. `TINANTA_RULES` is a list of six stage arrays, each
  living in its own file under `crates/panini-prakriya/src/tinanta/`; add
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
- The `panini-cli` binary has a single subcommand, `check` (flags `--trace`,
  `--json`, `--out`, `--in`). There is no `derive` subcommand in v1. `--in auto`
  (the default) auto-detects the input transliteration scheme; passing an
  explicit `--in` scheme (`slp1`/`iast`/`hk`/`deva`) makes that scheme
  authoritative, overriding auto-detection.

## Where things live
See `docs/ARCHITECTURE.md`.
