# Juhotyādi gaṇa (gaṇa 3), slice 3d — the six consonant-initial ṛ-roots

The prep spec (`2026-09-05-juhotyadi-gana-design.md`) tabled 3d as seven rows,
288 cells:

> | 3d | √bhṛ, √pṛ, √pṝ, √ghṛ, √hṛ, √sṛ, √ṛ | 288 | 7.4.66 *ur at* (+1.1.51),
> 7.4.77 *arti-pipartyoś ca*, 7.1.102 *ud oṣṭhyapūrvasya*, 6.4.78
> *abhyāsasyāsavarṇe*, 6.4.72 / 6.1.90 on the abhyāsa, 8.3.110 |

We re-probed it at the audited commit `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`.
We also ran this engine at `4e2274a` over the same cells on hand-built rows and
diffed the results cell by cell against vidyut. The record holds exactly: 288
cells, 323 forms. HEAD matches **0 of 288** cells. They all share one cause:
without 7.4.66 the abhyāsa keeps its ṛ-vowel (*bṛbharti* for *bibharti*).

**3d is split, in the same way 3c was split.** √ṛ (`03.0017`, *iyarti*,
*aiyaḥ*) is the only vowel-initial row. It carries everything structural: 6.4.78,
laṅ's 6.4.72 āṭ and a double 6.1.90 whose vṛddhi lands on the abhyāsa, and the
prep spec's review checkpoint on whether 6.4.71 / 6.4.72 should read `ANGA` or
the abhyāsa. It becomes **slice 3d2**. This slice takes the six
consonant-initial rows, and every rule it adds or widens is text-local to one
term.

## Scope

**Rows (6):** `03.0006 quBf\Y` √bhṛ (ubhayapada), and `03.0005 pf\` √pṛ,
`03.0004 pF` √pṝ, `03.0015 Gf\` √ghṛ, `03.0016 hf\` √hṛ, `03.0018 sf\` √sṛ (all
parasmaipada). **252 cells, 282 forms.**

**New rules (3):** 7.4.66 *ur at* and 7.4.77 *arti-pipartyoś ca*
(`abhyasa.rs`); 7.1.102 *ud oṣṭhyapūrvasya* (`guna.rs`).

**Widened rules (4):** 7.4.60 *halādiḥ śeṣaḥ* (the whole abhyāsa, not only its
initial cluster); 7.4.76 *bhṛñām it* (adds `03.0006`); 6.1.77 *iko yaṇ aci* (an
aṅga arm); 8.2.77 *hali ca* (the root+ending junction when SHAP is empty).

**Not transcribed (2):** 8.3.110 and 8.4.37. See "What vidyut credits that
this slice does not write".

**No new vikalpa.** The engine stays at eleven optional rules. Every multi-form
cell comes from the standing 7.1.35 / 8.4.56 forks.

**Unchanged, confirmed by the probe traces:** 6.1.10, 7.4.59, 7.4.62, 7.3.84,
7.3.83, 7.1.4, 6.1.66, 8.4.54 (already widened to `B` in 3b), 8.4.2 and 8.4.56.
`strip_anubandhas` gives `Bf pf pF Gf hf sf`, and `pada_from_upadesha` gives
ubhayapada for `quBf\Y` (1.3.72, ñit) and parasmaipada for the rest (1.3.78).

**Out of scope:** √ṛ and all of 3d2's machinery (6.4.78, 6.4.72 / 6.1.90 on the
abhyāsa, the augment-guard re-read); 3e, 3f; curādi.

## Root selection

| row | entry | root | pada | sanction |
|---|---|---|---|---|
| 03.0006 | quBf\Y | Bf | ubhaya | 1.3.72 (ñit) |
| 03.0005 | pf\ | pf | parasmai | 1.3.78 |
| 03.0004 | pF | pF | parasmai | 1.3.78 |
| 03.0015 | Gf\ | Gf | parasmai | 1.3.78 |
| 03.0016 | hf\ | hf | parasmai | 1.3.78 |
| 03.0018 | sf\ | sf | parasmai | 1.3.78 |

`curated_pada_agrees_with_upadesha_markers` and
`dhatupatha_numbers_resolve_upstream` cover all six without edits.

## The rules

vidyut's order on the abhyāsa, read off the step traces (*bibhrati*, *piprati*,
*pipurati*, *sasrati*), is 6.1.10 → 7.4.66 → 1.1.51 → 7.4.60 → 7.4.62 / 7.4.76 /
7.4.77. The stage becomes:

**6.1.10 → 7.4.66 → 7.4.60 → 7.4.59 → 7.4.62 → 7.4.76 → 7.4.77 → 7.4.78**

### 7.4.66 *ur at* — new, `abhyasa.rs`, directly after 6.1.10

The abhyāsa's ṛ-vowel (`f` or `F`) becomes `a`. 1.1.51 *uraṇ raparaḥ* makes it
`ar`, so the rule writes `ar`. 1.1.51 is not credited, exactly as guṇa's `ar`
does not credit it (it is in the prep appendix's "another convention" list).
7.4.60 then removes the `r`.

- **Guard on text, not row number.** No abhyāsa before 3d contains a ṛ-vowel,
  and the sūtra names a sound, not roots. The no-op guard is the text test
  itself.
- **It fires on every 3d cell. vidyut's fires only on the kṅit cells.** vidyut
  copies the guṇated stem (`Bar` on *bibharti*), so 7.4.66 has nothing to do on
  a pit cell. This engine copies the bare root (the stage header's
  dvitva-before-guṇa note), so the abhyāsa is always `Bf`. The forms agree. The
  divergence shows only in this engine's own trace pins, which pin THIS order.
  That is the same situation the header already records for 7.4.59.
- On √pṝ it runs before 7.4.59, so 7.4.59 finds a short `a` and declines. Its
  no-op guard already covers that.

### 7.4.60 *halādiḥ śeṣaḥ* — widened

The sūtra keeps the abhyāsa's first consonant and elides every other one. Today
the rule trims only the initial cluster (`hrI` → `hI`), and on `Bar` it would
compute `Bar` unchanged. Widened: **keep the first consonant and every vowel,
and drop every other consonant.** For a single-ekāc abhyāsa (6.1.10's NARROW
note) that is the sūtra.

- `Bar` → `Ba`, `par` → `pa`, `Gar` → `Ga`, `har` → `ha`, `sar` → `sa`.
- `hrI` → `hI` still holds. It is √hrī's prior, now pinned as a byte-identical
  case of the widened rule.
- The vowel-initial fall-through (√ṛ, 3d2) and the no-op guard stay.
- The comment "√hrī is the ONLY cluster-initial row … this rule will never gain
  a second witness" becomes false. The six 3d rows are the second witness, of a
  different arm. Rewrite it.

### 7.4.76 *bhṛñām it* — widened

The row-number match gains `03.0006`, the third root the sūtra names. Its
comment already says "slice 3d adds it with its witness". After 7.4.60:
`Ba` → `Bi`, and 8.4.54 makes it *bi-*.

### 7.4.77 *arti-pipartyoś ca* — new, `abhyasa.rs`, after 7.4.76

The abhyāsa vowel of √ṛ (*arti*) and √pṛ (*piparti*) becomes `i`. vidyut applies
it to both `pf\` and `pF` (its credits: `03.0004`, `03.0005`, `03.0017`).

- **Keyed by row number** (`03.0005`, `03.0004`), the 7.4.76 / 7.4.78
  precedent: the sūtra names roots, and by this stage `ANGA.text` no longer
  identifies them (`par`, `pur`).
- 3d2 adds `03.0017` with its witness. Writing an unwitnessed number now would
  be a mutation survivor.
- 7.4.76 and 7.4.77 name disjoint roots, so their relative order is decided by
  the trace pins only, and follows vidyut's (7.4.76 < 7.4.77).

### 7.1.102 *ud oṣṭhyapūrvasya* — new, `guna.rs`, after 7.3.84's applications

An aṅga-final `F` that follows a labial becomes `ur` (1.1.51, uncredited):
`pF` → `pur`.

- **It must see only the cells guṇa left alone.** On pit cells 7.3.84 has
  already turned `F` into `ar` (*piparti*), and the rule's `F` test declines
  there. That is the same self-guarding 6.1.77 relies on after 7.3.84.
- vidyut runs it before 6.1.10 on the kṅit cells, so its abhyāsa is copied from
  `pur`. Here the abhyāsa is copied from `pF` and reaches the same `pi-` through
  7.4.66 → 7.4.60 → 7.4.77. The forms agree; the pins hold this order.
- The labial test covers `p b B m` plus `v`, the oṣṭhya set the sūtra reads.
  Only `p` is witnessed, so the guard test must pin a non-labial declining
  rather than enumerate the class.

### 6.1.77 *iko yaṇ aci* — an aṅga arm

HEAD derives `bfBfAte` for *bibhrāte*. The rule has only its vikaraṇa arm (the
tanādi `u`), and its comment says to widen by arm, with a witness. The new arm:
**an aṅga-final `f` becomes `r` when SHAP is empty and the ending is
vowel-initial.** This gives *bibhrati*, *bibhrāte*, *bibhrate*, *bibhrīta*,
*piprati*, *jaghrati*, *jahrati*, *sasrati*.

- **The SHAP-empty clause keeps it off every thematic root.** Where śap or
  another vikaraṇa intervenes, the aṅga is not adjacent to the ending.
- The other athematic paths are adādi's luk (SHAP empty) and rudhādi's śnam.
  Adādi's six curated roots are `yA vA ad As vas SI`, none `f`-final. All 25
  curated rudhādi roots are consonant-final, and none ends in `r` or `v`. So no
  prior can reach this arm or the 8.2.77 widening below, whatever rudhādi's SHAP
  holds.
- Pit vowel-initial endings (loṭ uttama *āni / āva / āma*, ātmanepada *ai*) and
  laṅ's *jus* are guṇated first by 7.3.84 / 7.3.83. The aṅga ends in `r`, and
  the arm's `f` test declines (*bibharāṇi*, *abibharuḥ*).
- √pṝ never reaches it: on kṅit cells its aṅga is already `pur` (7.1.102), and
  *pipurati* keeps the `ur`.

### 8.2.77 *hali ca* — widened to the root+ending junction

*pipūrtaḥ* needs `pur` → `pUr` before a consonant. HEAD's 8.2.77 reads the first
character of SHAP. On the juhotyādi path SHAP is empty, so it declines silently.
Its own comment names this case: "when a consonant-final r/v-upadhā adādi root
lands, this must generalize to the root+ending junction — 6.1.78's athematic arm
… is the worked example to follow." √pṝ is that root (athematic through ślu, not
luk).

- **Widening:** read the first character of the first non-empty term after
  `ANGA`, following 6.1.78's fallback to `ENDING` when SHAP is empty.
- It fires on every consonant-initial kṅit ending: *pipūrtaḥ*, *pipūrhi*,
  *pipūryāt*, *apipūrva*, …
- It declines before a vowel (*pipurati*).
- The 8.2.79 `kur` guard is untouched. No curated SHAP-empty root has an
  r/v-final aṅga with an `i`/`u` upadhā (adādi's six above), so no prior moves.
- The "currently unreachable" comment is rewritten, not deleted, and names √pṝ.

### What vidyut credits that this slice does not write

- **8.3.110 *na rapara-sṛpi-sṛji-…*** — credited on *sasrati* and *sasratu*. It
  bars ṣatva. Here the engine's 8.3.59 cannot reach that `s` anyway: it needs a
  preceding iṇ or ku, and the abhyāsa vowel is `a`. So no form depends on the
  bar. It is not transcribed, and a trace pin holds both absences (no 8.3.59, no
  8.3.110 on *sasrati*). This is the same stance the 8.2.79 guard takes: the
  forms agree whether or not the log names the rule. If a later slice gives an
  iṇ-preceded `sr`, 8.3.110 lands then with its witness.
- **8.4.37 *padāntasya*** — credited once, on *bibhrīran*. It already exists as
  the silent guard inside 8.4.2 (`tripadi.rs`, "8.4.37 padAntasya"). This is not
  new work. The prep table omitted it; its correction is noted here.

## Counts

| | before | after |
|---|---|---|
| roots | 87 | **93** |
| root × pada × lakāra blocks | 436 | **464** |
| cells | 3924 | **4176** |
| forms | 4926 | **5208** |
| `ALTERNATES` rows | 1002 | **1032** |
| optional rules | 11 | 11 |

The 28 new blocks are √bhṛ's 2 padas × 4 lakāras plus 5 roots × 1 pada × 4
lakāras.

Shape buckets (`derivation_set_shape_matches_the_audited_numbers`):

| forms per cell | before | after | new cells |
|---|---|---|---|
| 1 | 3184 | **3418** | 234 |
| 2 | 571 | **577** | 6 — vidhiliṅ prathama eka, one per root |
| 3 | 123 | **135** | 12 — loṭ prathama and madhyama eka, two per root |
| 4 | 18 | 18 | — |
| 5 | 10 | 10 | — |
| 6 | 17 | 17 | — |
| 7 | 1 | 1 | — |

New `ALTERNATES` rows by key, five per root:

| key | per root | total | registry |
|---|---|---|---|
| `7.1.35` | 2 | 12 | 124 → 136 |
| `7.1.35+8.4.56` | 2 | 12 | 124 → 136 |
| `8.4.56` | 1 | 6 | 146 → 152 |
| | **5** | **30** | 1002 → 1032 |

√bhṛ's ātmanepada adds none: its 36 cells are single-form. vidyut gives √bhṛ
77 forms for 72 cells, the same five extras as each parasmaipada root.

## Testing

**Guard tests** (per rule, in the slice-7 style):

- **7.4.66** fires on `Bf` and `pF` (→ `Bar`, `par`). It declines on every prior
  abhyāsa shape (`hu`, `ki`, `BI`, `dA`, …).
- **7.4.60** turns `Bar` → `Ba` and still `hrI` → `hI`. It records nothing on a
  single-consonant open abhyāsa (`hu`).
- **7.4.76** fires on `03.0006` and still on `03.0007` / `03.0008`. It still
  declines on `03.0009`.
- **7.4.77** fires on `03.0005` and `03.0004`. It declines on `03.0006` and on
  the `""` default.
- **7.1.102** fires on `pF` before a kṅit ending. It declines on `pf` (short
  vowel), on a non-labial before `F` (hand-built), and on the guṇated `par`.
- **6.1.77's aṅga arm** fires on `Bf` + `ati`. It declines before a
  consonant-initial ending, when SHAP is non-empty (a thematic `f`-final aṅga,
  hand-built), and on a guṇated `Bar` + `Ani`. The vikaraṇa arm's existing tests
  are unchanged.
- **8.2.77** fires on `pur` + `tas` with SHAP empty. It declines on `pur` +
  `ati`. The `kur` exemption and the SHAP-present path keep their existing
  tests.

**Trace pins** (`crates/panini/tests/trace/juhotyadi.rs`):

- *bibharti*: 6.1.10 < 7.4.66 < 7.4.60 < 7.4.76 < 7.3.84 < 8.4.54. This is the
  √bhṛ witness 7.4.76's comment promised.
- *bibhrati*: 6.1.77, with 7.4.76 before it.
- *piparti*: 7.4.66 < 7.4.60 < 7.4.77.
- *pipūrtaḥ*: 7.1.102, then 8.2.77. *pipurati* has 7.1.102 and no 8.2.77.
- *jagharti*: 7.4.66 < 7.4.60 < 7.4.62 < 8.4.54.
- *sasrati*: 6.1.77, and neither 8.3.59 nor 8.3.110 in its log.

**Goldens:** 28 new `PARADIGM` blocks and 30 `ALTERNATES` rows, generated and then
checked against the tables above. The 3924 priors stay byte-identical.

## Audit and gate

- **Audit.** Repoint `/tmp/vidyut-full`'s dev-deps at this worktree first. For
  this spec's probe they were repointed from the deleted 3c2 worktree to
  `/workspace`, which is the pre-slice engine. Raise the committed `tools/audit`
  harness to 93 / 4176 / 5208 and expect zero divergence against `8da2f90b`.
  Run the README's `entry` negative control before believing the result.
- **`mise run test-full`** once, before the gate.
- **Mutation gate.**
  - The suite grows 6.4%, the largest step since 8a, and 3c2's record already
    projects the cap off 700s. So **re-measure** the full uncaught-suite floor at
    4176 cells, at the campaign's parallelism, and set `--timeout` above it. Do
    not project it from 3924.
  - Run in the foreground with an explicit long timeout. Pass `-o` to an
    isolated directory, and copy `outcomes.json` somewhere durable before any
    follow-up invocation.
  - Chunk with `--iterate` and the known missed/timeout exclusions if the
    campaign nears the 60-minute background limit.
  - The non-caught set must equal the documented equivalents plus the known
    permanent timeout. AGENTS.md names every missed or timed-out mutant
    verbatim.
  - Every new or widened guard clause must be caught, not merely reached.

## Docs sweep

README, AGENTS, ARCHITECTURE and every count: juhotyādi reaches **sixteen of
twenty-six** rows, 4176 cells, 1032 `ALTERNATES` rows, 5208 forms, 93 roots.

The grep covers wrapped counts, rule-scoped counts, spelled-out number words
("ten of its 26", "3924 priors"), root-shape literals (`Bf`, `pf`, `pF`, `Gf`,
`hf`, `sf`, `Bar`, `pur`, `sasr`), and every file a behaviour task touched. It
must include the comments this slice makes false:

- 7.4.60's "ONLY cluster-initial row … never gain a second witness";
- 6.1.77's "Only the vikaraṇa arm is written";
- 8.2.77's "Currently unreachable";
- 7.4.76's "slice 3d adds it";
- the `abhyasa.rs` module header's rule list;
- the stage header's 7.4.59 divergence note, which gains 7.4.66 and 7.1.102.

The prep spec's "Later slices" table gets a pointer blockquote under its 3d row,
as 3c's did.

## Risks

1. **7.4.66 and 7.1.102 fire in a different order, and on more cells, than
   vidyut's.** This is inherent to the bare-root copy. The audit compares form
   sets, and the pins hold this engine's order. If a pin is ever "fixed" to
   vidyut's order, the stage's documented dvitva-before-guṇa choice has been
   reversed by accident.
2. **The 7.4.60 widening reaches a prior.** Only √hrī has a consonant after its
   abhyāsa's first. It keeps `hI`, and the byte-identical priors guard it.
3. **The 6.1.77 and 8.2.77 widenings reach a SHAP-empty prior.** The adādi grep
   above shows none can. The guard tests pin the SHAP-present decline, so a
   widening that drops the SHAP-empty clause is caught.
4. **Suite cost.** Six more roots grow `roundtrip_sampled`'s residual Θ(N²) term.
   That is why the cap is re-measured rather than projected.
5. **Hand-derived counts.** They are stated exactly above, so the generated
   goldens contradict them audibly.

## Success criteria

- 4176 cells VALID with the trace pins; the 3924 priors byte-identical.
- Audit zero-divergence at 93 / 4176 / 5208 against `8da2f90b`, with the
  negative control verified failing.
- `mise run test-full` passes.
- Gate clean at a re-measured cap. The non-caught set is named verbatim in
  AGENTS.md.
- Juhotyādi at sixteen of its twenty-six rows; README, AGENTS and ARCHITECTURE
  swept.

## Later slices

- **3d2 — √ṛ (`03.0017`), 36 cells, 41 forms.** 6.4.78 *abhyāsasyāsavarṇe*
  (*iyarti*); 7.4.77's `03.0017` arm; laṅ's 6.4.72 āṭ with 6.1.90 twice, the
  vṛddhi landing on the abhyāsa (*aiyaḥ*, *aiyaruḥ*); 6.1.77 on *iyrati*. It
  also takes the prep spec's review checkpoint: re-read whether 6.4.71 / 6.4.72
  should guard on the abhyāsa rather than `ANGA`, now against a live
  vowel-initial witness.
- **3e** (√nij, √vij, √viṣ) and **3f** (the six ordinary-vowel rows) remain as
  3a's table has them.

Re-probe each one before writing its spec.
