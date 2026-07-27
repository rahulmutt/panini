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

Four gaṇas are covered: bhvādi (1), divādi (4), tudādi (6), adādi (2). gaṇa
is carried as a tag on the aṅga term (`Tag::Divadi` / `Tag::Tudadi` /
`Tag::Adadi`, mirroring how `Tag::Atmanepadin` carries pada), read by 3.1.69,
3.1.77, and 2.4.72. The vikaraṇa itself is selected by 3.1.68 (śap, bhvādi
and adādi), 3.1.69 (śyan, divādi), and 3.1.77 (śa, tudādi).

adādi (gaṇa 2) is the only gaṇa where the vikaraṇa is *luk'd*: 3.1.68 still
inserts śap (bhvādi and adādi share the same vikaraṇa rule), and **2.4.72
*adiprabhṛtibhyaḥ śapaḥ*** then empties it for adādi roots. The śap term is
kept in place with empty text rather than removed, so the `ANGA`/`SHAP`/
`ENDING` term indices stay stable for downstream rules.

adādi is now **complete** across all four lakāras: √yā and √vā, √ad
(parasmaipada), and √ās, √vas and √śī (ātmanepada) each derive in laṭ, laṅ,
loṭ and vidhiliṅ, including the athematic (śap-luk'd) ātmanepada path. The
consonant-final adādi roots meet the ending directly, so they are the
engine's junction witnesses: 8.4.55 cartva (√ad) and 8.2.25 dhi ca, which
elides an aṅga-final `s` before a Dh-initial affix (ADve, vaDve). √śī is the
gaṇa's only guṇa witness and lands the engine's first ṣatva; see
`docs/superpowers/specs/2026-07-25-adadi-si-5f-design.md` for the rule
analysis. `derive` itself carries no scope gate — it only tags the dhātu and
runs `TINANTA_RULES` (see `panini_prakriya::tinanta::derive`). A wrong-pada
derivation is instead blocked by the pada-sanction rule (1.3.12 *anudāttaṅita
ātmanepadam* / 1.3.78 *śeṣāt kartari parasmaipadam*), which sets
`Prakriya.blocked` when the requested pada doesn't match the root's tag; a
blocked prakriya's partial text never counts as a match in `Panini::check`.
`INVALID` means "not derivable within the covered grammar," not
"ungrammatical in Sanskrit." Coverage of the enumerable (root × lakāra)
space is pinned by
`crates/panini/tests/paradigm.rs::paradigm_covers_every_enumerable_cell`.

## Rule trace
Every applied sūtra is logged as a `RuleStep { sutra, name, before, after }`.
The `check` result carries the full trace per analysis.
