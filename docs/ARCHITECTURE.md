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

`TINANTA_RULES` (in `crates/panini-prakriya/src/tinanta/mod.rs`) is an
ordered `&[&[Rule]]` — seven pipeline stages, each in its own file — covering
all four lakāras. Each rule self-guards on `Prakriya.ctx` (lakāra, pada,
puruṣa, vacana) and returns whether it fired. Reading the stages in order,
and the rules within each stage in order, IS reading the grammar this crate
implements; `tinanta::rules()` yields that flattened sequence.

| stage file | rules | position |
|---|---|---|
| `samjna.rs` | 1.3.12, 1.3.78, 3.4.78, 1.3.9, 1.2.4 | before 3.1.68 |
| `tin.rs` | 3.4.85 … 3.4.102 | before 3.1.68 |
| `vikarana.rs` | 3.1.69, 3.1.73, 3.1.77, 3.1.81, 3.1.68, 2.4.72, 3.1.83, 1.2.4 | contains 3.1.68 |
| `anga.rs` | 6.4.71 … 7.2.81 | after 3.1.68 |
| `guna.rs` | 7.4.21, 7.3.84, 7.3.86, 7.3.84 (again — see below), 6.4.87, 6.4.77, 6.1.78, 7.3.101, 6.4.112, 6.4.113 — vowel gradation and vikaraṇa reshaping | after 3.1.68 |
| `adesha.rs` | 6.1.101 … 6.4.101 | after 3.1.68 |
| `tripadi.rs` | 8.2.77 … 8.4.55, 8.4.1, 8.4.2 | after 3.1.68 |

The stage boundary is file organisation, not grammar: the flattened order is
what matters, and `tinanta_rule_order_is_pinned` in `derivation_tests.rs`
pins all 67 ids verbatim. `tinanta/terms.rs` holds the term-index constants
and the reason 3.1.68 bisects the pipeline; `tinanta/sound.rs` holds the
varṇa classifiers.

Rule order is load-bearing and several orderings are non-obvious; the
constraints and their justifications are documented in the design specs
under `docs/superpowers/specs/`. The exact ordered traces in
`crates/panini/tests/trace.rs` are what pin them.

Six gaṇas are covered: bhvādi (1), divādi (4), tudādi (6), adādi (2), kryādi
(9), svādi (5). gaṇa is carried as a tag on the aṅga term (`Tag::Divadi` /
`Tag::Tudadi` / `Tag::Adadi` / `Tag::Kryadi` / `Tag::Svadi`, mirroring how
`Tag::Atmanepadin` carries pada), read by 3.1.69, 3.1.73, 3.1.77, 3.1.81, and
2.4.72. The vikaraṇa itself is selected by 3.1.68 (śap, bhvādi and adādi),
3.1.69 (śyan, divādi), 3.1.73 (śnu, svādi), 3.1.77 (śa, tudādi), and 3.1.81
(śnā, kryādi).

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
analysis.

kryādi (gaṇa 9) is thematic — śnā occupies the same `SHAP` slot as śap, śyan
and śa — but it is the first gaṇa whose vikaraṇa is itself reshaped by the
ending: 6.4.112 elides its `ā` before a vowel-initial kṅit sārvadhātuka
(kliSnanti), 6.4.113 turns it into `ī` before a consonant-initial one
(kliSnItaH), and 3.1.83 replaces it wholesale with śānac before `hi` after a
consonant-final root (kliSAna). That split is driven by 1.2.4, which as of
this slice tags parasmaipada apit endings ṅit as well as ātmanepada ones —
the distinction between pit `tip` (kliSnAti) and apit `tas` (kliSnItaH) is
the whole paradigm.

svādi (gaṇa 5) is thematic like divādi, tudādi and kryādi — its vikaraṇa is
śnu (3.1.73) — but it is the first gaṇa where 7.3.84's guṇa is not blocked
outright by 1.1.5: śnu is apit, so guṇa reaches it, and the ik that guṇates
belongs to the vikaraṇa rather than the root. That forces four rules to stop
reading "the aṅga's final sound" as shorthand for "the root's final sound",
because 1.4.13 *yasmāt pratyayavidhis tadādi pratyaye'ṅgam* makes the aṅga
affix-relative and svādi is the first gaṇa where the two readings diverge
visibly: **7.3.84** now has a second `Rule` entry (same id, ordered right
after the first) that guṇates `terms[SHAP]` with respect to `terms[ENDING]`
(`Ap` + `nu` + `ti` → `Apnoti`, blocked before ṅit `tas` → `ApnutaH`);
**6.1.78** *eco'yavāyāvaḥ* gained a vikaraṇa arm so the resulting `o`
converts to `av` before a vowel (`ApnavAni`); **6.1.90** *āṭaś ca*'s
athematic arm widened from `SHAP.is_empty()` to "any non-`a`/`A`-final
SHAP", since a non-empty `SHAP` no longer implies the root abuts the
ending; and **6.4.101** *hujhalbhyo her dhiḥ* was rewritten to read the
sound immediately before the ending rather than the aṅga's own final sound.
None of the four is optional and each is revealed by a different cell — see
`docs/superpowers/specs/2026-07-29-svadi-gana-design.md`'s "Three existing
rules assume the aṅga is the root" for the full analysis, and
`AGENTS.md`'s note on 7.3.84 / 1.2.4 appearing twice by design.

The gaṇa's other rules turn on a single predicate, *asaṁyogapūrva* — whether
śnu's `u` is preceded by a conjunct: 6.4.87 *huśnuvoḥ sārvadhātuke* (yaṇ,
apavāda) and 6.4.77 *aci śnudhātubhruvāṁ yvor iyaṅuvaṅau* (uvaṅ, utsarga)
split `hinvanti` from `Apnuvanti` before a vowel-initial ending, and 6.4.106
*utaś ca pratyayād asaṁyogapūrvāt* splits `hinu` from `Apnuhi` at `hi`.

8.4.1 / 8.4.2 are the engine's first ṇatva. They are guarded to skip an `n`
that is word-final or immediately followed by a jhal — the effect of 8.4.37
*padāntasya* and of 8.3.24 *naś cāpadāntasya jhali* bleeding the rule, neither
of which is modelled here because the engine has no anusvāra machinery. The
guard is exactly equivalent within tripādī order; it costs trace fidelity, and
it is the first thing liṭ and luṅ will want retired. `asmaran` and `BAzante`
are the goldens that pin it.

`derive` itself carries no scope gate — it only tags the dhātu and
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
