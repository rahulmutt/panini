# Architecture

Data flow for `check`:
`input → panini-lipi (→SLP1) → panini-analyze (candidates) → panini-prakriya
(derive each) → panini (keep exact matches) → render`.

## Crates
- `panini-lipi` — SLP1 ⇄ IAST/HK/Devanāgarī + scheme detection. No grammar.
- `panini-data` — curated roots, tiṅ table, enums. No I/O beyond embedded data.
- `panini-prakriya` — the engine: `Term`/`Prakriya`/`Context` model, it-samjna,
  and `TINANTA_RULES`, the ordered rule list that `tinanta::derive` runs via
  `run_pipeline`. Pure SLP1, no I/O.
- `panini-analyze` — proposes candidate `(root, puruṣa, vacana)` inputs.
- `panini` — facade: `Panini::check` / `Panini::derive`, `Verdict`, `Analysis`.
- `panini-cli` — the `panini` binary (`check` subcommand; `--trace`, `--json`,
  `--out`, validity exit codes).

## The rule pipeline

`TINANTA_RULES` (in `crates/panini-prakriya/src/tinanta.rs`) is a single
ordered `&[Rule]` covering all four lakāras. Each rule self-guards on
`Prakriya.ctx` (lakāra, pada, puruṣa, vacana) and returns whether it fired.
Reading the list top to bottom IS reading the grammar this crate implements.

Rule order is load-bearing and several orderings are non-obvious; the
constraints and their justifications are documented in the design specs
under `docs/superpowers/specs/`. The exact ordered traces in
`crates/panini/tests/trace.rs` are what pin them.

## Rule trace
Every applied sūtra is logged as a `RuleStep { sutra, name, before, after }`.
The `check` result carries the full trace per analysis.
