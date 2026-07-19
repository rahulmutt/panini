# Architecture

Data flow for `check`:
`input → panini-lipi (→SLP1) → panini-analyze (candidates) → panini-prakriya
(derive each) → panini (keep exact matches) → render`.

## Crates
- `panini-lipi` — SLP1 ⇄ IAST/HK/Devanāgarī + scheme detection. No grammar.
- `panini-data` — curated roots, tiṅ table, enums. No I/O beyond embedded data.
- `panini-prakriya` — the engine: `Term`/`Prakriya` model, it-samjna, `Rule`
  metadata + ordered controller, `tinanta::derive`. Pure SLP1, no I/O.
- `panini-analyze` — proposes candidate `(root, puruṣa, vacana)` inputs.
- `panini` — facade: `Panini::check` / `Panini::derive`, `Verdict`, `Analysis`.
- `panini-cli` — the `panini` binary (`check` subcommand; `--trace`, `--json`,
  `--out`, validity exit codes).

## Rule trace
Every applied sūtra is logged as a `RuleStep { sutra, name, before, after }`.
The `check` result carries the full trace per analysis.
