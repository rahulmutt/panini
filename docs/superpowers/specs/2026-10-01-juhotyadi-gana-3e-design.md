# Juhotyādi gaṇa (gaṇa 3), slice 3e — √nij, √vij, √viṣ

The prep spec (`2026-09-05-juhotyadi-gana-design.md`) tabled this slice as:

> | 3e | √nij, √vij, √viṣ | 216 | 7.4.75 *nijāṁ trayāṇāṁ guṇaḥ ślau*, 7.3.87
> *nābhyastasyāci piti sārvadhātuke*, 6.1.65 *ṇo naḥ* (or the stored-form
> convention) |

and left one decision to it: "Whether 3e stores `nij` for `Ri\ji~^r` or
implements 6.1.65 is 3e's to decide."

We re-probed the three rows at the audited commit
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` and ran this engine at `2f9e5fc`
over the same cells on hand-built rows (stored text `nij` / `vij` / `viz`,
ubhayapada), diffing derivation sets cell by cell. The record holds exactly:
**72 cells and 79 forms per row, 216 cells and 237 forms in all**. HEAD matches
**0 of 216**, for two causes only:

1. **7.4.75 absent — all 216 cells.** The abhyāsa stays `ni` / `vi` where vidyut
   has `ne` / `ve`: `ninekti` / *nenekti*, `ninikte` / *nenikte*, `vivezwi` /
   *vevezwi*, `viviqQve` / *veviqQve*.
2. **7.3.87 absent — 21 of those cells, seven per row** (laṅ uttama eka
   parasmaipada `am`; loṭ uttama `Ani/Ava/Ama` and `AE/AvahE/AmahE`). This
   engine's 7.3.86 guṇates the root: `ninejAni` / *nenijAni*, `aninejam` /
   *anenijam*, `ninejE` / *nenijE*, `vivezARi` / *vevizARi*.

Every ending-side step already agrees once the abhyāsa vowel is right, including
all of √viṣ's `z` junctions: 8.4.41 (*vevezwi*), 8.2.41 → 8.3.59 (*vevekzi*),
8.4.41 → 8.4.53 (*veviqQi*), 8.2.39 word-finally (*aveveq* / *avevew*). vidyut
does not use 8.2.36 on these rows.

## Scope

**Rows (3):** `03.0012 Ri\ji~^r` (√ṇijir), `03.0013 vi\ji~^r` (√vijir),
`03.0014 vi\zx~^` (√viṣḷ), all ubhayapadī by 1.3.72 (svarita). **216 cells, 237
forms.** Suite 4212 → 4428 cells, 5249 → 5486 forms; juhotyādi 17 → 20 of its 26
rows.

**New rules (2):** 7.4.75 *nijāṁ trayāṇāṁ guṇaḥ ślau* (`abhyasa.rs`); 7.3.87
*nābhyastasyāci piti sārvadhātuke* (`guna.rs`), the second user of `Rule.bars`
and its first mandatory one.

**No widened rules. No new vikalpa** — the engine stays at eleven optional
rules. The 21 extra forms are the standing 7.1.35 / 8.4.56 forks.

**Decided: 6.1.65 *ṇo naḥ* is not written.** The stored-form convention covers
it: `dhatvadeh_sha_sa`'s `R` → `n` arm (`panini-data/src/lib.rs:1735`, already
used by √nī `01.1049`) maps `Ri\ji~^r` to `nij`, and
`curated_pada_agrees_with_upadesha_markers` passes on it.

**Unchanged, confirmed on HEAD's shape by the probe:** 1.3.72 (`samjna.rs:123`),
6.1.10, 7.4.60 (`abhyasa.rs:117`), 6.4.71 (`anga.rs:35`), 7.1.4 (`anga.rs:260`),
7.3.86 (`guna.rs:150`, its 1.1.5 ṅit check inline), 6.4.101 (`adesha.rs:721`),
and in `tripadi.rs` 8.2.23, 8.2.30, 8.2.39, 8.2.41, 8.3.15, 8.3.59, 8.4.2,
8.4.41, 8.4.53, 8.4.55 and 8.4.56.

**Out of scope:** 3f; curādi; tagging loṭ uttama endings `Tag::Pit` (see
7.3.87's guard).

## The rules

vidyut's order, identical across the 237 prakriyās:

- pit, consonant-initial ending (*nenekti*, *nenejmi*, laṅ *aneneg*):
  7.3.86 (`nej`) → 6.1.10 → 7.4.60 (`ne`) → 7.4.59 (`ni`) → 7.4.75 (`ne`) →
  tripādī;
- ṅit ending (*nenijati*, *nenikte*, vidhiliṅ, *hi*, *tāt*):
  6.1.10 → 7.4.60 → 7.4.75 → tripādī;
- the 7.3.87 cells: 6.1.10 → 7.3.87 (no text change) → 7.4.60 → 7.4.75.

This engine copies the bare root and guṇates afterwards (`abhyasa.rs:9-20`), so
7.4.59 finds a short vowel and never fires on these rows — the same recorded
divergence as 3d's 7.4.66. 7.4.75 needs only to follow 7.4.60. For the same
reason this engine's 7.3.87 runs after the whole abhyāsa stage, in `guna`,
where vidyut credits it between 6.1.10 and 7.4.60; the audit compares form
sets, so only this engine's trace pins see the order.

### 7.4.75 *nijāṁ trayāṇāṁ guṇaḥ ślau* — new, `abhyasa.rs`

The abhyāsa of the three roots √nij, √vij, √viṣ takes guṇa before ślu. `ni` →
`ne`, `vi` → `ve`.

- **Keyed by row number** (`ctx.dhatupatha` matches `"03.0012" | "03.0013" |
  "03.0014"`), like 7.4.76 / 7.4.77 / 7.4.78. The sūtra names three roots, and
  the text `vij` is also `06.0009` and `07.0023`. The comment carries the row
  table, as 7.4.76's does.
- **Placed after 7.4.62, before 7.4.76.** The stage becomes:

  **6.1.10 → 7.4.66 → 7.4.60 → 7.4.59 → 7.4.62 → 7.4.75 → 7.4.76 → 7.4.77 →
  7.4.78 → 6.4.78**

  7.4.75's rows are disjoint from 7.4.76–7.4.78's, so only the trace pins
  observe the relative order.
- **Guṇa of the abhyāsa's one vowel.** Every abhyāsa this reaches is a single
  ekāc (6.1.10's note), so guṇating the vowel is guṇating the abhyāsa. Only
  `i` → `e` is exercised; the mapping is `sound::guna_of` (`sound.rs:8`), not a
  new table. The module doc's rule list (`abhyasa.rs:1-2`) gains 7.4.75.
- **Unit tests:** fires on each of the three rows (`ni` → `ne`, `vi` → `ve`);
  declines on a synthetic prakriyā with `ctx.dhatupatha = "06.0009"` and
  abhyāsa `vi` (the text-key hazard — tudādi never reduplicates, so only a
  synthetic test can hold this).

### 7.3.87 *nābhyastasyāci piti sārvadhātuke* — new, `guna.rs`

An abhyasta aṅga takes no laghūpadha guṇa before a vowel-initial pit
sārvadhātuka ending. Written as an apavāda through `Rule.bars`, as 6.4.117 is:

- **A mandatory `Rule`** (`vikalpa: false`) with **`bars: &["7.3.86"]`**,
  changing no text, recording `"7.3.87", "nAByastasyAci piti sArvaDAtuke"`.
  vidyut credits the same no-op block.
- **Placed between 7.3.84 and the first 7.3.86** (`guna.rs:150`), so the barred
  rule runs after its barrer, as `exactly_the_pinned_bars` requires. That test
  gains the pair `7.3.87 → 7.3.86`.
- **Guard — all four:**
  1. `ANGA` carries `Tag::Abhyasta` (`abhyasa.rs:60-61`);
  2. the ending is sārvadhātuka;
  3. the ending's first character is a vowel;
  4. the ending is pit, read exactly as 1.2.4 reads it
     (`samjna.rs:249`): `ending.has(Tag::Pit) || (lakara == Lot && purusha ==
     Uttama)`.

  Clause 4 is not `has(Tag::Pit)` alone: only tip / sip / mip are tagged
  (`samjna.rs:195`); loṭ uttama endings are pit by 3.4.92 *āḍ uttamasya pic ca*
  but are never tagged — 1.2.4 excludes them by lakāra and puruṣa. The tag alone
  would catch 1 of the 7 cells per row. The comment says so and cites 3.4.92.
  Tagging those endings `Pit` was rejected for this slice: 7.3.92
  (`guna.rs:329`) also reads the tag, so the change would widen this slice.
- **Barring is by id**, so the branch also skips 7.3.86's tanādi vikalpa arm
  (`guna.rs:208`). Inert: no tanādi aṅga is abhyasta. The comment states it.
- **3f's inheritance**, in the comment: vidyut credits 7.3.87 on all six 3f
  rows; it changes forms on `03.0021` (*cikitAni*), `03.0022` (*tuturARi*) and
  `03.0023` (*diDizARi*) and is a credited no-op on `03.0019`, `03.0024`,
  `03.0025` (a-upadhā, where 7.3.86 declines anyway).
- **No curated row moves.** Every curated abhyasta aṅga (3a–3d2) is vowel-final;
  7.3.86 declines on all of them, and vidyut's 7.3.87 covers laghūpadha guṇa
  only, not 7.3.84. *juhavAni* is unaffected.
- **Unit tests, one per clause:**
  - fires on `am` (tagged Pit) and on loṭ uttama `Ani` (untagged), and the
    pipeline then skips 7.3.86 on that branch;
  - declines on a non-abhyasta aṅga with a vowel-initial pit ending;
  - declines on a consonant-initial pit ending (`mi` — *nenejmi* keeps guṇa);
  - declines on a ṅit vowel-initial ending (`ati`);
  - declines on an ārdhadhātuka ending.

## What vidyut credits that this slice does not write

- **6.1.65** and **1.3.3.1** (√nij, √vij) — the stored-form convention.
- **7.4.59** on these rows — this engine copies before guṇa.
- **6.1.68** on laṅ prathama / madhyama eka — this engine credits 8.2.23.
- **8.2.66**, **8.4.37** (on `-Iran`), **8.4.68** — the standing conventions.

## Corpus

`panini-data` gains the three rows between `03.0011` and `03.0015`:

| row | entry | root | pada | sanction |
|---|---|---|---|---|
| 03.0012 | `Ri\ji~^r` | nij | ubhaya | 1.3.72 (svarita); `R` → `n` by the stored-form convention |
| 03.0013 | `vi\ji~^r` | vij | ubhaya | 1.3.72 (svarita) |
| 03.0014 | `vi\zx~^` | viz | ubhaya | 1.3.72 (svarita) |

Each comment names its path: 7.4.60, 7.4.75 (*nenekti*, *vevekti*, *vevezwi*),
7.3.87 on the uttama cells. The juhotyādi row-list assertion gains all three,
and its comment moves to twenty of twenty-six with "slice 3f closes it".
`curated_pada_agrees_with_upadesha_markers` and
`dhatupatha_numbers_resolve_upstream` cover them without edits (the probe ran
both green).

## Tests

- **Goldens** (`crates/panini/tests/paradigm/data/juhotyadi.rs`): the 216 cells,
  both padas per row, transcribed from vidyut. **ALTERNATES gains 21 rows**,
  seven per row: laṅ prathama and madhyama eka parasmaipada (8.4.56), vidhiliṅ
  prathama eka parasmaipada (8.4.56), loṭ prathama and madhyama eka
  parasmaipada (7.1.35 *tātaṅ*, then 8.4.56) — the shape of 3d2's five plus the
  two laṅ rows a consonant-final root adds.
- **Trace pins** (`crates/panini/tests/trace/juhotyadi.rs`), in THIS engine's
  order:
  - *nenekti* (laṭ prathama eka P): 6.1.10 → 7.4.60 → 7.4.75 in the abhyāsa
    stage, then 7.3.86 on the root, 8.2.30, 8.4.55;
  - *nenijati* (laṭ prathama bahu P): ṅit — no 7.3.86, no 7.3.87 step;
  - *nenijAni* (loṭ uttama eka P): 7.3.87 present, 7.3.86 absent;
  - *anenijam* (laṅ uttama eka P): aṭ, 7.3.87 on the tagged `am`;
  - *nenikte* (laṭ prathama eka Ā);
  - *vevezwi* (laṭ prathama eka P): 8.4.41;
  - *vevekzi* (laṭ madhyama eka P): 8.2.41 → 8.3.59.
- **Spot check** in `paradigm/main.rs` beside 3d2's: *nenekti*, *nenijAni*,
  *vevezwi*.
- **Unit tests** in `abhyasa.rs` and `guna.rs` as listed under each rule.
- The 4212 prior cells stay byte-identical, traces included.

## Audit

Copy the committed `tools/audit/panini_full_audit.rs`, never rewrite it. Bump
its invariants to **97 roots, 4428 cells (492 blocks × 9), 5486 forms (4428 +
1058 ALTERNATES rows)**, with `derivation_set_shape_matches_the_audited_numbers`
raised to match. Repoint `/tmp/vidyut-full`'s dev-deps at the slice worktree
before running. Zero divergence against `8da2f90b` is required, and the
mis-resolved-root negative control must be shown failing first.

## Mutation gate

Re-measure the uncaught-suite floor at the campaign's parallelism and set the
cap at 6× it, rounded up to the next 10 s. Copy `outcomes.json` durably before
any further invocation. Expected new non-caught mutants: none — 7.4.75's row
match is killed by the goldens and its decline by the `06.0009` test; each 7.3.87
clause has a synthetic killer, and the ṅit clause also the *nenijati* trace pin.
AGENTS.md names the non-caught set verbatim.

## Doc sweep

README, AGENTS.md and ARCHITECTURE.md: 94 → 97 roots, seventeen → twenty of
twenty-six juhotyādi rows, 4212 → 4428 cells, 5249 → 5486 forms, ALTERNATES
1037 → 1058. The prep spec's "Later slices" notes gain a 3e line recording the
6.1.65 decision.

The grep covers wrapped counts, spelled-out numbers, rule-scoped counts, the
root-shape literals (`nij`, `vij`, `viz`, `ne`, `ve`, `Ri\ji`), and every "3e"
promise in code and docs. Each promise is rewritten as done, or retargeted to 3f
where it is 3f's. Recorded file:line anchors are re-grepped at final HEAD.

## Success criteria

- 4428 cells VALID with the trace pins; the 4212 priors byte-identical.
- Audit zero-divergence at 97 / 4428 / 5486 against `8da2f90b`, with the
  negative control verified failing.
- `mise run test` passes.
- Gate clean at a re-measured cap; the non-caught set named verbatim in
  AGENTS.md.
- Juhotyādi at twenty of its twenty-six rows; no "3e" promise left in code or
  docs.

## Later slices

- **3f** (√bhas, √kit, √tur, √dhiṣ, √dhan, √jan) remains as the prep spec's
  table has it, and inherits 7.3.87 unchanged. Re-probe it before writing its
  spec.
