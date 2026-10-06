# Curādi 10m: `picca~` and the 8.2.30 narrowing

10l's "Later slices" set this slice aside for one row, `10.0175 picca~`
*kuṭṭane*, because curating it needs 8.2.30 *coḥ kuḥ* narrowed. The rudhādi
and juhotyādi rows that 8.2.30 fires on depend on that rule. Changing it on
its own slice gives the change its own trace diff and review.

The defect: 8.2.30 scans the whole word for any cu sound followed by a jhal.
After ṇic, `picc` reaches the tripādī as `piccay…`. The first `c` stands
before the second `c`, a jhal, so the rule fires inside the root and gives
*pikcayati*. vidyut fires 8.2.30 per term. The cu must be the term's **final**
sound, and the next non-empty term must be a pratyaya or āgama beginning with
a jhal, or the term must end the pada. vidyut then gives *piccayati*.

After this slice, curādi is **open at 491 of its 509** rows.

## Scope

**In:** `10.0175 picca~` (`picc`), `PadaAssignment::Nic`, ubhayapadī by
1.3.74 *ṇicaś ca*, across laṭ / laṅ / loṭ / vidhiliṅ, plus the 8.2.30
narrowing. The ṇic is obligatory, so no branch is blocked.

- **72 cells** (1 × 2 padas × 4 lakāras × 9), **78 forms**. The six extra
  forms are the parasmaipada block's usual multi-form cells: laṅ and
  vidhiliṅ prathama eka (`d`/`t`), and loṭ prathama and madhyama eka (`tAd`
  / `tAt`). So **6 new `ALTERNATES` rows** (11666 → 11672).
- **Totals:** 593 → **594** roots, 38160 → **38232** cells, 49826 →
  **49904** forms. Blocked branches stay at **6552**.
- Guru upadhā (the conjunct `cc`), so 7.3.86 declines before ṇic, and laṭ P
  3sg is *piccayati*. Its prakriyā credits 3.1.25, 3.1.32, 3.1.68 and 7.3.84,
  and **no 8.2.30**.

**Out:**
- vidyut's 8.2.30 also turns an upadhā `Y` into `N`. This engine reaches
  √bhañj's nasal by its own rules, and every prior trace is byte-identical
  without that step, so it is not ported.
- The other eighteen curādi rows. They get re-scoped by prototype in later
  slices (see Later slices).

### Decisions

**8.2.30 reads terms, not the flattened word.** The new `apply`:

1. Walk the terms in order. A term qualifies if its last character is a cu
   sound (`kutva_of(last).is_some()`).
2. Find the next **non-empty** term after it. The rule applies if:
   - there is none: the cu ends the pada; or
   - that term is not `ANGA`, and its first character is a jhal.
3. The first qualifying term gets its last character replaced with
   `kutva_of`'s velar, and the rule records once.

`kutva_of` still drives both the match and the substitute. The comment block
explaining why survives, and so does its pointer to
`rinakti_trace_reaches_k_in_one_step`.

**Why `j != ANGA`.** vidyut requires the following term to be a pratyaya or
āgama. This engine doesn't tag terms that way, but its slots are fixed:
`AGAMA`, `ABHYASA`, `ANGA`, `SHAP`, `ENDING`. Every term after `ANGA` is an
affix or āgama. The only non-affix that can follow a term is `ANGA` itself,
after `AGAMA` or `ABHYASA`. The guard rules that boundary out. It is
**unreachable** in the corpus: an abhyāsa is always vowel-final after 7.4.60
*halādiḥ śeṣaḥ*, and so is the āṭ/aṭ. With the guard deleted, the
prototype's full trace dump was byte-identical. So it is pinned by a
hand-built unit test, not by any golden (see Tests).

**Rudhādi is unaffected by construction.** śnam leaves the root's tail in
`SHAP` (`Ba | naj | ti`, `ri | nac | ti`), so the cu is already term-final.
The jhal that conditions it is the first character of `ENDING`, or nothing
follows (word-final, after 8.2.23). The existing unit test
`coh_kuh_fires_only_word_finally_or_before_a_jhal` passes unchanged.

**The comment block's "No cell in this suite has two cu sounds" paragraph is
no longer true.** `picc` has two. It is rewritten to name `picc` as the
witness that the scan reads the term-final sound only. "Read via
`word_chars`" becomes an account of why the scan walks terms. The
cross-term adjacency it described is now the "next non-empty term" step.

## Evidence

A throwaway worktree (detached at `2bb7a4f`, never pushed) carried the
narrowing and the `picca~` row.

- **Prior traces:** a scratch example dumped every curated cell's branches,
  blocked ones included (dhātupāṭha number, pada, lakāra, puruṣa, vacana,
  branch index, blocked flag, text, each step's id/before/after). It ran on
  unmodified main and on the prototype. All **56378** prior branches
  (49826 live) are byte-identical.
- **Audit:** the committed harness ran with its totals moved to 594 / 38232
  / 49904, against vidyut `8da2f90`, with the dev-deps repointed at the
  worktree. Result: **zero differences** on the first run, with 6552 blocked
  branches.
- **Negative control:** main's 8.2.30 restored, with the row kept. The audit
  then failed on **72 cells, all `10.0175`'s** (*pikcayati*, …).
- **Guard reachability:** the `j != ANGA` conjunct deleted, then the dump
  re-run. Byte-identical to the prototype's, so the guard is unreachable in
  the corpus.
- **8.2.30's roster** (live branches): 306, on twelve rows. `03.0012` and
  `03.0013` have 30 each. The rudhādi rows `07.0004`, `07.0005`, `07.0007`
  and `07.0017` have 30 each. `07.0016` and `07.0021`–`07.0025` have 21
  each. No curādi row credits it, `10.0175` included.
- **The full suite, `--no-fail-fast`, failed in exactly four tests.** Each
  is updated under Changes, and none is a defect:
  - `panini-data`: `curated_roots_have_expected_ganas_and_padas` (593 →
    594), `curadi_rows_are_the_four_hundred_ninety_curated_roots` (roster).
  - `paradigm`: `paradigm_covers_every_enumerable_cell` (no goldens yet).
  - `trace`: `a_kusmad_is_credited_on_exactly_the_akusmiya_cells`, whose
    count of curated 1.3.74 rows goes 427 → 428.

  Every 8.2.30 unit test and trace pin passed unchanged.

## Changes

### Engine — `panini-prakriya`

- **`tinanta/tripadi.rs`:** 8.2.30's `apply` as Decisions describes it, and
  its comment block rewritten to match (both paragraphs Decisions names).
  If `word_chars` loses this caller, its doc keeps naming its remaining
  callers.
- **Unit tests** in `coh_kuh_fires_only_word_finally_or_before_a_jhal`, or a
  sibling test beside it, each on a hand-built prakriyā via `with_slots`:
  - **term-internal cu declines:** `piccay` + `a` + `ti` (the real
    intermediate: ṇic folded into `ANGA` by 3.1.32, guṇated and made `ay`
    by 6.1.78, then śap) leaves the text unchanged. This is the case main's
    8.2.30 got wrong.
  - **an earlier non-qualifying cu does not hide a later qualifying one:** a
    term with an internal `c` and a final `j` before a jhal-initial
    `ENDING` turns the final `j` to `g`, and only that one.
  - **the `ANGA` guard:** `ABHYASA` ending in a cu before an `ANGA` that
    begins with a jhal declines. This is the only witness for `j != ANGA`.
  - **an empty term between is skipped:** the cu's term, then an empty
    term, then a jhal-initial term, fires. This pins "next **non-empty**
    term".

### Data — `panini-data/src/lib.rs`

- **One `Dhatu` row**, a 10m block after 10l's. Its comment gives the
  number, upadeśa, artha and root. It also gives the guru upadhā before
  ṇic, and that 8.2.30 declines on the root-internal `cc` (the slice's
  reason for being). It ends "Ubhayapadī by 1.3.74 (*piccayati*). Slice
  10m."
- `curadi_rows_are_the_four_hundred_ninety_curated_roots` becomes
  `…_four_hundred_ninety_one_…`, its list and slice paragraph extended.
- `curated_roots_have_expected_ganas_and_padas`: 593 → **594**.
- The `pada` field doc: 1.3.74's 427 curādi rows → **428**, 593 → **594**.
  The agreement test's other buckets are re-derived, not assumed.

### Goldens — `crates/panini/tests/paradigm/data/curadi.rs`

**8 `ParadigmRow`s** (1 row × 2 padas × 4 lakāras) and **6
`AlternateRow`s**, generated from the engine's output and spot-checked
against the prototype's vidyut forms. Review with
`--diff-algorithm=histogram`, and keep the goldens out of review packages.

### Tests

- `derivation_set_shape_matches_the_audited_numbers`: 38160 → **38232**
  cells, 49826 → **49904** forms. Buckets are re-derived from the census.
  The doc gains a 10m paragraph and "OPEN at 491 of its 509 rows".
- `paradigm_covers_every_enumerable_cell` passes once the goldens land.
- `a_kusmad_is_credited_on_exactly_the_akusmiya_cells`: 427 → **428**.
- **A corpus-wide 8.2.30 roster test** in `trace/curadi.rs`, or beside the
  existing rudhādi 8.2.30 pins, whichever fits better: over every curated
  cell, `credited("8.2.30")` is exactly the twelve rows Evidence names, with
  their per-row branch counts. It must also assert that `10.0175` never
  credits 8.2.30. Goldens compare forms, not traces, so this is the guard
  against the narrowing silently dropping or adding a credit.
- **`check()`:** a new `curadi_analyses_its_10m_forms` with *piccayati* (P)
  and *piccayate* (Ā), each with exactly one analysis, `10.0175`. Grep the
  goldens for both forms before asserting it.
- **Prior traces:** dump every prior cell's branches, blocked included, on
  main and on HEAD. They must be byte-identical, all 56378.

TDD: the term-internal and `ANGA`-guard unit tests are written first and
seen failing against main's 8.2.30. The census, coverage, roster and
`check()` assertions are written first and seen failing with the row
absent. The narrowing and the row then make them pass.

## Audit

Copy the harness from `tools/audit/`; do not rewrite it. Repoint
`/tmp/vidyut-full`'s dev-deps at the worktree first, or it audits the
pre-slice engine.

1. The `entry` negative control must fail first, on √bhū's 36 cells.
2. Then the full run must give **zero differences over 594 roots / 38232
   cells / 49904 forms**, with 6552 blocked branches.

The harness's asserted totals and their comments move with it.

## Mutation gate

The rewritten 8.2.30 replaces its mutants with new ones. Every new mutant
must be caught; the unit tests above exist for that, the `ANGA` guard's
especially.

- Re-measure the full uncaught-suite floor and the `skip_nic -> true`
  reading under the campaign's load. The new row adds no optional-ṇic id,
  but it adds cells.
- Set the cap by AGENTS.md's rule. Pass `-o`, and copy `outcomes.json`
  durably before any other invocation.
- Expected: the non-caught set is identical to 10l's, which AGENTS.md names
  verbatim. A new survivor in the 8.2.30 code is a missing test, to be fixed
  with one, not excused.

## Doc sweep

README, AGENTS.md, `docs/ARCHITECTURE.md`, `tools/audit/` (README and the
harness), and the test docs:

- **Counts:** 593 → 594 roots; 38160 → 38232 cells; 49826 → 49904 forms;
  11666 → 11672 `ALTERNATES`. 6552 blocked branches is unchanged.
- **Curādi:** "OPEN at 490 of its 509" → **491 of 509**. AGENTS.md's gaṇa
  history gains a 10m clause.
- **8.2.30's description** wherever it is summarized (README, AGENTS.md,
  ARCHITECTURE, `panini-data`'s rudhādi row comments): "word-final or
  before a jhal" becomes term-final, before a jhal-initial affix or āgama,
  or pada-final. Re-read each one; don't just pattern-match.
- **ARCHITECTURE's census paragraphs:** re-derived from the test census,
  including the "pins all N ids" line at the census paragraph's top.
- **The audit README:** a 10m "Last recorded result" entry.
- **10l's spec:** its "Later slices" 10m bullet gains a pointer to this
  spec.
- **Sweep greps:**
  - spelled-out counts with `grep -i` ("four hundred ninety", "five hundred
    ninety-three", "four hundred twenty-seven")
  - numerals: 490, 593, 427, 38160, 49826, 11666
  - wrapped counts and rule-scoped counts
  - "not yet curated" / "uncurated" / "later slice" / "10m" / `picca`
    phrasings
  - every comment that says no cell has two cu sounds
  - all of `crates/`, including tests
  - recorded file:line anchors, re-grepped at final HEAD

## Success criteria

- The 72 new cells match vidyut, and every prior branch's trace is
  byte-identical between main and HEAD.
- The audit gives zero differences over 594 / 38232 / 49904, after its
  negative control fails.
- 8.2.30's credits are exactly the twelve prior rows, pinned by the roster
  test, and `10.0175` credits none.
- The mutation campaign catches every new mutant. Its non-caught set is
  unchanged from 10l's and named in AGENTS.md.
- `mise run test` and clippy are clean.

## Later slices

- The remaining eighteen curādi rows: re-scope them by prototype first.
- Upasargas, and with them `10.0368 za\da~` (7.3.78) and 6.1.76 *padāntād
  vā*.
- `gupU~`, `paRa~\` and `pana~\` join `AYA` when curated. `kfpU~\` and
  `kfpa~\` join `KRP` when curated.
- A causative (hetumaṇic) slice takes 01.0934 and 10.0494, and with them
  7.3.36's named roots and 6.1.54 on `sPura~`.
