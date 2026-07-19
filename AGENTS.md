# Contributor & agent guide

## Environment
- Toolchain is pinned via `mise` (`mise install`). Do not install Rust globally.
- Tasks: `mise run build | test | lint | fmt | mutants | audit`.
- Optional tools (install on demand):
  - `cargo install cargo-mutants` — mutation testing; use
    `cargo mutants --package panini-prakriya` (this is what `mise run mutants` runs).
  - `cargo install cargo-deny` — supply-chain checks. `cargo deny check bans` and
    `cargo deny check licenses` pass. **Known caveat:** `cargo deny check advisories`
    currently fails on this pinned toolchain (rustc 1.83.0 / cargo-deny 0.17.0):
    cargo-deny's rustsec parser cannot read newer advisory entries that use a
    CVSS 4.0 vector (e.g. `cvss = "CVSS:4.0/..."`). Fixing this needs cargo-deny
    ≥ 0.20.2 (and a newer rustc); `mise run audit` will surface this failure — it
    is a known tooling limitation, not a regression in this codebase.
  - `cargo install cargo-fuzz` plus a **nightly** Rust toolchain — needed for real
    fuzzing of `panini-lipi` (the fuzz target lives at `crates/panini-lipi/fuzz`).
    Nightly is not set up in this environment; the fuzz target's harness code
    also compiles/runs in a limited way on stable, but proper fuzzing requires
    installing nightly yourself.

## Rules of the codebase
- SLP1 is the only internal representation; transliterate only in `panini-lipi`.
- `#![forbid(unsafe_code)]` in every crate.
- Grammar changes are gated by the golden paradigm test
  (`crates/panini/tests/paradigm.rs`). Surface forms there are the source of
  truth; sūtra ids/names in traces must match the cited reference
  (ashtadhyayi.com).
- The `panini-cli` binary has a single subcommand, `check` (flags `--trace`,
  `--json`, `--out`). There is no `derive` subcommand in v1. The `--in` flag
  is currently inert — the CLI always auto-detects the input transliteration
  scheme; `--in` does not select it.

## Where things live
See `docs/ARCHITECTURE.md`.
