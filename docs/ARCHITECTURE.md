# Architecture

Data flow for `check`:
`input → panini-lipi (→SLP1) → panini-analyze (candidates) → panini-prakriya
(derive each) → panini (keep exact matches) → render`.

## Crates
- `panini-lipi` — SLP1 ⇄ IAST/HK/Devanāgarī + scheme detection. No grammar.
- `panini-data` — curated roots (each tagged with its gaṇa and pada), tiṅ
  table, enums. No I/O beyond embedded data.
- `panini-prakriya` — the engine: `Term`/`Prakriya`/`Context` model, it-samjna,
  and `TINANTA_RULES`, the ordered rule list that `tinanta::derive` runs via
  `run_pipeline`. Pure SLP1, no I/O.
- `panini-analyze` — proposes candidate `(root, lakāra, pada, puruṣa, vacana)`
  inputs, with the pada taken from the root's tag.
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

Three gaṇas are covered: bhvādi (1), divādi (4), tudādi (6). gaṇa is carried
as a tag on the aṅga term (`Tag::Divadi` / `Tag::Tudadi`, mirroring how
`Tag::Atmanepadin` carries pada), read by 3.1.69 and 3.1.77. The vikaraṇa
itself is selected by 3.1.68 (śap, bhvādi), 3.1.69 (śyan, divādi), and 3.1.77
(śa, tudādi).

adādi (gaṇa 2) is being introduced, carried the same way via `Tag::Adadi`. It
is the first gaṇa where the vikaraṇa is *luk'd*: 3.1.68 still inserts śap
(bhvādi and adādi share the same vikaraṇa rule), and **2.4.72
*adiprabhṛtibhyaḥ śapaḥ*** then empties it for adādi roots. The śap term is
kept in place with empty text rather than removed, so the `ANGA`/`SHAP`/
`ENDING` term indices stay stable for downstream rules.

adādi is covered in laṭ, laṅ and loṭ only. adādi × vidhiliṅ needs the
athematic optative (the yās → yuḥ reduction), which is not implemented; a
scope gate at the top of `panini_prakriya::derive` — deliberately *not* a
`Rule`, so it is never mistaken for grammar — blocks that combination so it
reports INVALID instead of emitting a non-word. A blocked prakriya's partial
text never counts as a match in `Panini::check`. Coverage of the enumerable
(root × lakāra) space is pinned by
`crates/panini/tests/paradigm.rs::paradigm_covers_every_enumerable_cell`,
whose `GATED` list holds exactly this one hole.

## Rule trace
Every applied sūtra is logged as a `RuleStep { sutra, name, before, after }`.
The `check` result carries the full trace per analysis.
