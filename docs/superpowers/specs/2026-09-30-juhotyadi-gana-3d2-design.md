# Juhotyādi gaṇa (gaṇa 3), slice 3d2 — √ṛ, the vowel-initial ṛ-root

Slice 3d (`2026-09-29-juhotyadi-gana-3d-design.md`) split √ṛ out of the prep
spec's 3d row and handed it this slice:

> **3d2 — √ṛ (`03.0017`), 36 cells, 41 forms.** 6.4.78 *abhyāsasyāsavarṇe*
> (*iyarti*); 7.4.77's `03.0017` arm; laṅ's 6.4.72 āṭ with 6.1.90 twice, the
> vṛddhi landing on the abhyāsa (*aiyaḥ*, *aiyaruḥ*); 6.1.77 on *iyrati*. It
> also takes the prep spec's review checkpoint […] It must also revisit
> **7.4.60's vowel-initial fall-through**.

We re-probed it at the audited commit `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`
and ran this engine at `f0d35f7` over the same cells on a hand-built row,
diffing cell by cell against vidyut. The record holds exactly: **36 cells, 41
forms**. HEAD matches **0 of 36**, all for one cause: the abhyāsa stays `ar`
(*ararti*, *AraH*) where vidyut has `iy` (*iyarti*, *EyaH*). Every ending-side
step already agrees.

## Scope

**Row (1):** `03.0017 f\` gatO, √ṛ, parasmaipada by 1.3.78. **36 cells, 41
forms.**

**New rule (1):** 6.4.78 *abhyāsasyāsavarṇe* (`abhyasa.rs`).

**Widened rules (2):** 7.4.60 *halādiḥ śeṣaḥ* (the vowel-initial fall-through
is retired); 7.4.77 *arti-pipartyoś ca* (adds `03.0017`).

**Closed checkpoint:** 6.4.71 / 6.4.72 keep reading `ANGA`'s initial; √ṛ is the
live witness of the class-preservation argument. Comments and one test only.

**No new vikalpa.** The engine stays at eleven optional rules. The five extra
forms come from the standing 7.1.35 / 8.4.56 forks.

**Unchanged, confirmed on HEAD's shape by the probe:** 6.1.10, 7.4.66 (fires on
every cell here, as on 3d's), 6.4.72, 7.3.83, 7.3.84, 6.1.90's aṅga arm, 6.1.77's
aṅga arm (3d), 8.2.23, 8.2.66, 8.3.15, 8.3.59's `r` arm (3d), 8.4.2 and 8.4.56.
Each already fires on the wrong `ar` abhyāsa, so each fires on the right one.

**Out of scope:** 3e, 3f; curādi; vidyut's second 6.1.90 credit and its 6.1.68
credit (see "What vidyut credits that this slice does not write").

## The rules

vidyut's order on the abhyāsa, identical on all 41 prakriyās, is 6.1.10 →
7.4.66 → 1.1.51 → 7.4.60 → 7.4.77 → 6.4.78. (On pit cells vidyut guṇates first
and copies `ar`, so 7.4.66 has nothing to do; this engine copies the bare root
and credits 7.4.66 everywhere — the divergence 3d already records.) The stage
becomes:

**6.1.10 → 7.4.66 → 7.4.60 → 7.4.59 → 7.4.62 → 7.4.76 → 7.4.77 → 7.4.78 → 6.4.78**

### 7.4.60 *halādiḥ śeṣaḥ* — widened

The sūtra keeps the abhyāsa's *ādi hal* and elides every other consonant. A
vowel-initial abhyāsa has no ādi hal, so every consonant goes: vidyut's
`ar` → `a` on `03.0017`. The rule becomes exactly that: **keep the first
character if it is a consonant, keep every vowel, drop every other consonant.**

- The `is_vowel(first)` fall-through is deleted, and with it its comment.
- The no-op guard stays: `f` (a single vowel) still records nothing.
- `haladih_shesha_declines_for_a_vowel_initial_abhyasa` is rewritten as
  `haladih_shesha_trims_a_vowel_initial_abhyasa`: `ar` → `a` and the synthetic
  `ap` → `a` fire and record; `f` declines. The old test's claim that `ap` →
  `a` is a "wrong truncation" is what vidyut's √ṛ trace refutes.

### 7.4.77 *arti-pipartyoś ca* — widened

The row-number match gains `03.0017`, the sūtra's first root (*arti*). The
comment's row table gains `03.0017 f\ √ṛ`, and the "joins with its witness in
3d2" sentence is deleted. After 7.4.60: `a` → `i`.

The decline test's comment (`arti_pipartyos_ca_declines_off_its_rows`) loses its
"√ṛ arrives with its witness in 3d2" line; `03.0017` must not appear in its
off-row list.

### 6.4.78 *abhyāsasyāsavarṇe* — new, `abhyasa.rs`, after 7.4.78

With *aci* and *yvor iyaṅuvaṅau* carried from 6.4.77: an abhyāsa-final `i`/`u`
(short or long) before a vowel that is not savarṇa with it becomes `iy`/`uv`.
`i` + `f` → `iy`: *iyarti*.

- **Placed in the abhyāsa stage**, after 7.4.78. That keeps every edit to the
  abhyāsa's text in one stage, matches vidyut's step adjacency (7.4.77 →
  6.4.78 on every prakriyā), and puts it before 6.4.72 / 6.1.90 (laṅ: `A`+`iy`
  → `Ey`) and 6.1.77 (*iy·f·ati* → *iyrati*, not *irati*) by construction.
- **Guarded on the sound, not the row**, as 7.4.66 is: the sūtra names a
  sound. The follower is `ANGA`'s first character — at this stage the raw
  root's (`f`), which is asavarṇa with `i` whether or not guṇa later makes it
  `a`.
- **The whole sūtra is written**, both *yvoḥ* arms and the asavarṇa clause,
  though √ṛ exercises only the `i` arm against a vowel. Synthetic unit tests
  kill each clause:
  - a `u`-final abhyāsa before a vowel → `uv`;
  - `i` before a savarṇa `i` declines;
  - `ci` before `ki` (consonant follower) declines — also every 3a–3d row;
  - an abhyāsa not ending in `i`/`u` (`a` before a vowel) declines.
- A pipeline-order test pins `6.1.10 → 7.4.66 → 7.4.60 → 7.4.77 → 6.4.78` for
  `03.0017`, as a new row of `the_r_roots_reach_their_abhyasa_through_ur_at_then_haladih_shesha`.

### 6.4.71 / 6.4.72 — checkpoint closed, `ANGA` read kept

The 3a argument: the abhyāsa copies the root's first ekāc, and no rule in
7.4.59–7.4.78 changes its initial's class, so `ANGA`'s initial gives the same
consonant/vowel verdict as the abhyāsa's. 6.4.78 now joins that stage and also
preserves the class (`i` → `iy`, still vowel-initial). √ṛ is the vowel-initial
witness: `ANGA` reads `f`, the abhyāsa reads `i`, both vowels, and HEAD already
picks āṭ.

- The 6.4.71 comment's closing sentence ("slice 3d2's √ṛ … is the vowel-initial
  row that re-checks this argument") becomes the witnessed statement, and its
  rule list names 6.4.78.
- A new test derives √ṛ laṅ prathama eka and asserts 6.4.72 fires, 6.4.71 does
  not, and 6.1.90 writes `Ey` into the abhyāsa.
- `adesha.rs`'s synthetic `A+iy+ar → Eyar` test (6.1.90) drops its "stays
  synthetic until 3d2" caveat and names `03.0017` as the live row.

Restructuring the guards to read the first non-empty term after `AGAMA` was
considered and rejected: no row (3e and 3f are consonant-initial) can tell the
two reads apart, so the clause would be a survivor or need a synthetic test to
justify a change with no observable effect.

## What vidyut credits that this slice does not write

- **6.1.90 twice** on laṅ cells. The second credit changes no text; this
  engine's aṅga arm applies once.
- **6.1.68** on laṅ prathama/madhyama eka, where this engine uses 8.2.23 — the
  standing convention.
- **1.1.51** after 7.4.66 — uncredited here, as for every `ar`.
- **8.4.68** *a a* — the saṃvṛta `a` restoration, never modelled here.

## Corpus

`panini-data` gains the row between `03.0016` and `03.0018`:

| row | entry | root | pada | sanction |
|---|---|---|---|---|
| 03.0017 | f\ | f | parasmai | 1.3.78 |

Its comment names the path: 7.4.66, 7.4.60 (`ar` → `a`), 7.4.77, 6.4.78:
*iyarti*, laṅ *EyaH*. The juhotyādi row-list assertion gains it, and its
comment moves to seventeen of twenty-six with "slices 3e and 3f close it".
`curated_pada_agrees_with_upadesha_markers` and
`dhatupatha_numbers_resolve_upstream` cover it without edits.

## Tests

- **Goldens** (`crates/panini/tests/paradigm/data/juhotyadi.rs`): the 36-cell
  √ṛ table, transcribed from vidyut. **ALTERNATES gains 5 rows** (41 − 36):
  loṭ prathama eka and madhyama eka, two each (7.1.35 *tātaṅ*, then 8.4.56
  `tAd`/`tAt`); vidhiliṅ prathama eka, one (8.4.56 `yAd`/`yAt`).
- **Trace pins** (`crates/panini/tests/trace/juhotyadi.rs`), in THIS engine's
  order:
  - *iyarti* (laṭ prathama eka: pit, guṇa after copy);
  - *iyftaH* (laṭ prathama dvi: kṅit);
  - *iyrati* (laṭ prathama bahu: 6.4.78 before 6.1.77);
  - *EyaruH* (laṅ prathama bahu: 6.4.78 in the abhyāsa stage, then 6.4.72,
    7.3.83, and 6.1.90 writing `Ey` into the abhyāsa).
- **Spot check** in `paradigm/main.rs` beside 3d's (`:987`): *iyarti*, *EyaH*,
  *iyrati*.
- **Unit tests** in `abhyasa.rs` and `anga.rs` as listed under each rule.
- The 4176 prior cells stay byte-identical, traces included.

## Audit

Copy the committed `tools/audit/panini_full_audit.rs`, never rewrite it. Bump
its invariants to **94 roots, 4212 cells (468 blocks × 9), 5249 forms (4212 +
1037 ALTERNATES rows)**. Repoint `/tmp/vidyut-full`'s dev-deps at the slice
worktree before running. Zero divergence against `8da2f90b` is required, and
the mis-resolved-root negative control must be shown failing first.

## Mutation gate

Re-measure the uncaught-suite floor at the campaign's parallelism and set the
cap at 6× it, rounded up to the next 10 s. Copy `outcomes.json` durably before
any further invocation. Expected new non-caught mutants: none — every 6.4.78
clause has a synthetic killer, the 7.4.60 widening is killed by √ṛ and `ap`,
and the 7.4.77 arm by √ṛ's goldens. AGENTS.md names the non-caught set
verbatim.

## Doc sweep

README, AGENTS.md and ARCHITECTURE.md: 93 → 94 roots, sixteen → seventeen of
twenty-six juhotyādi rows, 4176 → 4212 cells, 5208 → 5249 forms. ARCHITECTURE's
"3d2's √ṛ is where that argument meets a vowel-initial witness" becomes the
resolved statement. The prep spec's table notes gain a 3d2 line.

The grep covers wrapped counts, spelled-out numbers, rule-scoped counts, the
root-shape literals (`ar`, `iy`, `Ey`, `f\`), and every "3d2" promise in code:
`abhyasa.rs` (7.4.60, 7.4.77 and their tests), `anga.rs` (6.4.71), `adesha.rs`
(6.1.90's test), `panini-data` (the row-list comment). Each promise is rewritten
as done or deleted.

## Success criteria

- 4212 cells VALID with the trace pins; the 4176 priors byte-identical.
- Audit zero-divergence at 94 / 4212 / 5249 against `8da2f90b`, with the
  negative control verified failing.
- `mise run test` passes.
- Gate clean at a re-measured cap; the non-caught set named verbatim in
  AGENTS.md.
- Juhotyādi at seventeen of its twenty-six rows; no "3d2" promise left in code
  or docs.

## Later slices

- **3e** (√nij, √vij, √viṣ) and **3f** (the six ordinary-vowel rows) remain as
  3a's table has them. Re-probe each before writing its spec.
