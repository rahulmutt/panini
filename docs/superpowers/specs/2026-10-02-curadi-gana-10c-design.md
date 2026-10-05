# Curādi gaṇa (gaṇa 10), slice 10c — the remaining ākusmīya roots in bulk

## Summary

Slice 10b opened curādi's ākusmīya antargaṇa (`10.0192 cita~` …
`10.0236 kusma~`, ātmanepadī by the gaṇasūtra 10.0496 *ā kusmād
ātmanepadinaḥ*) at four rows, and built everything the class needs:
`PadaAssignment::Akusmiya`, the `AKUSMIYA` range, `Tag::Akusmiya`, the
10.0496 rule and 1.3.78's decline. Its "Later slices" list put the remaining
ākusmīya rows next, in bulk.

This slice curates **33** of the remaining 41 rows. It is a data-only slice:
the current engine (main at `54a5fcb`) already derives all 33 rows,
1188 cells, with **zero differences** from vidyut. There are no engine
changes and no new rule ids.

The range then stands at 37 of 45 curated. The eight left out belong to
other classes (see Scope).

## Scope

**In:** these 33 rows, ātmanepada only, across laṭ / laṅ / loṭ / vidhiliṅ
(33 × 36 = 1188 cells), grouped by the pre-ṇic change:

| pre-ṇic change | rows (number `code`, laṭ prathama eka) |
|---|---|
| 7.3.86 guṇa (6) | `10.0197 qip` qepayate · `10.0205 kil` kelayate · `10.0206 pil` pelayate · `10.0221 truw` trowayate · `10.0222 kuw` kowayate · `10.0232 vid` vedayate |
| 7.2.116 (10) | `10.0195 das` dAsayate · `10.0196 qap` qApayate · `10.0200 spaS` spASayate · `10.0210 lal` lAlayate · `10.0214 SaW` SAWayate · `10.0216 syam` syAmayate · `10.0218 Sam` SAmayate · `10.0223 gal` gAlayate · `10.0224 Bal` BAlayate · `10.0234 man` mAnayate |
| none (17) | `10.0201 tarj` tarjayate · `10.0202 Barts` Bartsayate · `10.0203 bast` bastayate · `10.0204 ganD` ganDayate · `10.0207 vizk` vizkayate · `10.0208 hizk` hizkayate · `10.0209 nizk` nizkayate · `10.0211 kUR` kURayate · `10.0212 tUR` tURayate · `10.0213 BrUR` BrURayate · `10.0215 yakz` yakzayate · `10.0217 gUr` gUrayate · `10.0219 lakz` lakzayate · `10.0220 kuts` kutsayate · `10.0225 kUw` kUwayate · `10.0226 kuww` kuwwayate · `10.0233 mAn` mAnayate |

The "none" rows have no laghu ik upadhā for 7.3.86 and no upadhā `a` for
7.2.116: a long vowel, a conjunct, or a non-vowel upadhā. Every arm of
7.3.86 (i, u) and 7.2.116 was already reached by 10a/10b goldens, so this
slice adds no new rule coverage. It is breadth.

The `code` column is the it-stripped upadeśa in every row: none of the 33
carries a nasal-inserting `i` (`idit`) marker or an initial `z`/`R`. But
`dhatupatha_numbers_resolve_upstream` does **not** apply unchanged (an
amendment made after prototyping; see "√das and the sibling check" below).

**Out:**

- the six optional-ṇic ākusmīya rows (`10.0193 daSi~`, `10.0194 dasi~`,
  `10.0198 tatri~`, `10.0199 matri~`, `10.0227 vancu~`, `10.0230 divu~`).
  vidyut derives a parasmaipada no-ṇic paradigm for each, and the
  ātmanepada ones need 7.1.58 or 7.3.63. They go to the optional-ṇic slice;
- `10.0231 gf` and `10.0235 yu`, which need 7.2.115 before ṇic;
- gaṇasūtra **10.0494** *nānye mito 'hetau* (see √syam and √śam below), mit
  roots generally, and everything else on 10a's and 10b's out-of-scope lists.
  (Slice 10d took the mit roots — see `2026-10-02-curadi-gana-10d-design.md` —
  and moved 01.0934 and 10.0494 to a causative slice: without a causative,
  01.0934 can never fire.)

### Decisions

**The six intra-curādi homographs are in.** `10.0197 qipa~`,
`10.0211 kURa~`, `10.0214 SaWa~`, `10.0219 lakza~`, `10.0226 kuwwa~` and
`10.0233 mAna~` share an upadeśa with an ubhayapadī curādi row outside the
range (`10.0189`, `10.0438`, `10.0041`, `10.0006`, `10.0034`, `10.0381`).
10b held back `lakza~` to curate it with `10.0006`. This slice reverses
that. Rows are keyed by dhātupāṭha number, and `code` is deliberately not
unique (both √aś rows spell `aS`), so a homograph needs no structural
support. When a partner is curated later, its ātmanepada cells will repeat
these forms. That is expected, the same as any two rows sharing a surface.
Each homograph row's comment names its partner number. (Slice 10k curated
the five ubhayapadī partners `10.0006`, `10.0034`, `10.0041`, `10.0189` and
`10.0438`: see `2026-10-05-curadi-gana-10k-design.md`.)

**√syam and √śam are in, without 10.0494.** vidyut credits 10.0494 on
`10.0216 syama~` and `10.0218 Sama~`. In vidyut it is a bare `p.step` in
the general mittva rules: for a curādi root ending in `am` it is credited
*instead of* 01.0934 (the am-final mittva rule), and changes no form. This
engine implements neither 01.0934 nor any mittva, so a 10.0494 rule here
would block nothing. Deleting it could fail no test, and it would leave a
mutation survivor. The forms already match. The two rows' comments name
them as the witnesses the mit slice inherits. Once that slice adds 01.0934,
these goldens fail unless it also adds 10.0494. (Slice 10d, the mit slice,
did not add 01.0934; the witnesses pass to a causative slice.)

### √das and the sibling check (amendment)

Prototyping found one row the spec's first draft missed.
`dhatupatha_numbers_resolve_upstream` requires each curated number to be the
only row in its gaṇa with the same it-stripped form and the same artha.
`10.0195 dasa~` has the artha *darSanadaMSanayoH*, and so does its neighbour
`10.0194 dasi~`, an excluded optional-ṇic row. The test's `stored_form`
helper models 7.1.58 *idito num dhātoḥ* only as a special case
(`his` → `hins`), so `dasi~` also strips to `das`, and 10.0195 reads as
ambiguous. None of the other 32 rows trips the check.

**Decision: generalize the helper, keep √das.** `stored_form` inserts the
num after the last vowel (1.1.47 *mid aco 'ntyāt paraḥ*) for every
**idit** upadeśa, meaning one whose last marker is `i~`. A non-final `i~`
belongs to another marker: irit `i~r` (`ru\Di~^r`) or `cakzi~N`. Those are
the only non-final shapes in the vendored dhātupāṭha. So `dasi~` stores as
`dans`, and `hisi~` → `hins` still comes out unchanged. This is a test-only
change. A new unit test pins the helper: idit `hisi~`/`dasi~`/`tatri~`/`aci~^`
→ `hins`/`dans`/`tantr`/`anc`; non-final `ru\Di~^r` / `ca\kzi~\N` →
`ruD` / `cakz`; plain `dasa~` / `kusma~` → `das` / `kusm`. The optional-ṇic
slice would need the same generalization when it curates `dasi~`.

## Evidence

All evidence comes from throwaway probes in `/tmp/vidyut-full`, against
vidyut-prakriya at `8da2f90`, the vendored dhātupāṭha's commit. None of it
ships.

- `examples/curadi_probe_10c.rs` derives all 45 ākusmīya rows in both padas
  and prints any sūtra id outside the union used by 10b's four rows. The
  33 rows in scope have no parasmaipada forms, and one form per ātmanepada
  cell. Their only extra id is 10.0494, on `10.0216` and `10.0218`. The
  optional-ṇic rows show their parasmaipada no-ṇic paradigms plus 7.1.58 or
  7.3.63; `gf` and `yu` show 7.2.115.
- `examples/curadi_engine_10c.rs` builds a `Dhatu` for each of the 33 rows
  (`PadaAssignment::Akusmiya`), derives every ātmanepada cell with this
  engine (dev-deps at `/workspace`, main `54a5fcb`), and compares derivation
  sets with vidyut: **33 rows, 1188 cells, 0 differences.**
- **Form collisions:** among the 1188 new forms, the only collisions are the
  36 cells shared by `10.0233 mAna~` and `10.0234 mana~`. Both give
  *mAnayate* and so on: one by no change, one by 7.2.116. None of the new
  forms appears in any existing golden.

The goldens are generated from the probe's output, not typed from this
table.

## Changes

### Data — `panini-data/src/lib.rs`

- **33 `Dhatu` rows**, `gana: Gana::Curadi`, `pada:
  PadaAssignment::Akusmiya`, interleaved with 10b's four so that the
  ākusmīya block runs in dhātupāṭha order (the goldens likewise). Each
  carries 10b's comment shape: number, upadeśa, artha and root, then the
  pre-ṇic change (or "no pre-ṇic change", with the reason) and "Ātmanepadī
  by 10.0496. Slice 10c." Homograph rows add their partner number. `10.0216`
  and `10.0218` add the 10.0494 note above. `10.0233` and `10.0234` each
  name the other as sharing every form.
- `curated_roots_have_expected_ganas_and_padas` goes from 111 to 144 rows,
  and `curadi_rows_are_the_eight_curated_roots` becomes
  `…_forty_one_…` with all 41 curādi rows listed.
  `dhatupatha_numbers_resolve_upstream` covers them through the table it
  already walks, once `stored_form` is generalized (above). The `AKUSMIYA` iff test and the split pada-agreement arm
  need no change; they pick the rows up from the table.
- The `pada` field doc: "re-derives 102 of these 111 … four ākusmīya rows'
  are the gaṇasūtra 10.0496's" becomes **102 of these 144 … 37 ākusmīya
  rows'**. 102 + 1 (√bhuj) + 4 (1.3.74) + 37 = 144. The block comment at
  the curādi rows ("four ākusmīya roots … OPEN at 8 of its 509") moves to
  37 ākusmīya roots, **41 of 509**.

### Goldens — `crates/panini/tests/paradigm/data/curadi.rs`

132 `ParadigmRow`s (33 roots × 4 lakāras, ātmanepada), generated from the
probe and spot-checked by hand against the table above. No `AlternateRow`s:
every cell holds one form.

### Tests

- `derivation_set_shape_matches_the_audited_numbers`: 5076 → **6264**
  cells (564 → **696** blocks); one-form cells 4252 → **5440**; every other
  bucket unchanged. Its doc gains a 10c paragraph and "OPEN at 41 of its
  509 rows".
- `crates/panini/tests/trace/curadi.rs`: two tests list 10b's four numbers
  literally. Both switch to the `AKUSMIYA` range rather than to a 37-entry
  list. The range is positional and independent of the curated `pada`
  column, so the check stays non-circular.
  - `a_kusmad_is_credited_on_exactly_the_akusmiya_cells`: the count goes
    from 144 to **1332** (37 × 36). Each hit's number must satisfy
    `AKUSMIYA.contains`. Its comment changes from "4 roots" to 37. The
    1.3.74 half is unchanged.
  - `an_akusmiya_roots_parasmaipada_is_blocked_by_a_kusmad_alone` walks
    every curated row in `AKUSMIYA` (37) and asserts that the walk is not
    empty.
- **Homographs:** before any `check()` assertion names the root a form
  belongs to, grep the goldens for that form. *mAnayate* and its 35
  siblings belong to two rows.
- **`check()`:** a new paradigm test checks one witness per pre-ṇic shape and
  per homograph row. Each gets exactly one analysis, ātmanepada, opening with
  10.0496 and crediting no pada sūtra. *mAnayate* / *amAnayata* get exactly
  two analyses (√mān, √man), and only √man's credits 7.2.116. The
  parasmaipada shapes are Invalid.
- **Prior traces:** dump every prior cell's credited-rule log on main and
  on HEAD, as `trace_dump_10b.rs` does; the two must be byte-identical. A
  data-only slice should move nothing, and this proves it did not.

No engine code changes. The one new unit test is the `stored_form` pin
above, seen failing against the old helper. Otherwise TDD here means the
count and list assertions and the `check()` test are added first and seen
failing (rows absent), then the data rows make them pass.

## Audit

Copy the committed harness from `tools/audit/`; do not rewrite it. Repoint
`/tmp/vidyut-full`'s dev-deps at the worktree before running, or the audit
measures the pre-slice engine. The negative control must fail first. Then
the full run: **144 roots / 6264 cells / 7394 forms, zero differences.**
The harness's root-count comments move with it. Its both-pada count (30)
does not change.

## Mutation gate

No production code changes, but the suite grows by 1188 cells. So
re-measure the uncaught-suite floor at `-j 4` and set the cap from it: 6×
the floor, rounded up to the next 10 s. Pass `-o`, and copy `outcomes.json`
durably before any further invocation. Expected: the non-caught set is
identical to 10b's. A change there means the extra suite time pushed a
survivor into TIMEOUT, and the cap is wrong. AGENTS.md names the
non-caught set verbatim.

## Doc sweep

README, AGENTS.md, `docs/ARCHITECTURE.md`, the audit harness, test docs:

- 111 → 144 roots; 5076 → 6264 cells; 6206 → 7394 forms; 564 → 696
  blocks;
- curādi "open at 8 of 509" → **41 of 509**. The gaṇa tallies stay at ten
  gaṇas, nine complete. AGENTS.md's gaṇa history gains "at 41 after slice
  10c curated 33 more ākusmīya roots";
- ARCHITECTURE's tātaṅ paragraph: "the curated set's 29 ātmanepada-only
  roots" → **62**, and "82 + 29 = the 111 curated roots" → 82 + 62 = 144.
  The 82-root parasmaipada census is unchanged;
- unchanged, but re-derived from the test census rather than assumed:
  ALTERNATES (1130), the multi-form buckets, pada-ambiguous surfaces (72;
  every new form is ātmanepada and none collides with an existing golden),
  and the stage count;
- the 10b spec's out-of-scope bullets ("the other ākusmīya rows … in bulk",
  "`10.0219 lakza~`, deliberately") and its "Later slices" line gain a
  pointer to this spec. The `lakza~` bullet also records the reversal.

The grep covers wrapped counts, spelled-out numbers ("four ākusmīya",
"thirty-seven"), rule-scoped counts ("10.0496 … 144"), every
"ātmanepada-only" census, and every enumeration of the ākusmīya roots
(√cit, √vṛṣ, √mad, √kusm). Recorded file:line anchors are re-grepped at
final HEAD.

## Success criteria

- 1188 new cells match vidyut. All 5076 prior cells and their traces are
  byte-identical between main and HEAD.
- The audit reports zero differences over 144 roots / 6264 cells / 7394
  forms, after its negative control fails.
- The mutation campaign's non-caught set is unchanged from 10b's, and named
  in AGENTS.md.
- `mise run test` and clippy are clean.

## Later slices

10a's order, now without the ākusmīya bulk: mit roots (taking 01.0934 and
10.0494, with √syam and √śam as the witnesses; slice 10d took the jñapādi and
deferred both gaṇasūtras to a causative slice), adanta roots (with the
ā-garvīya list, 10.0497), then optional ṇic (taking the six optional-ṇic
ākusmīya rows; taken by slice 10f, see `2026-10-02-curadi-gana-10f-design.md`). √smiṅ, and with it 7.2.115 before ṇic, rides whichever
slice first needs 7.2.115. √gṛ (`10.0231`) and √yu (`10.0235`) are the
ākusmīya rows that wait on it. (Slice 10j took √smiṅ, √gṛ, √yu and √ci: see `2026-10-04-curadi-gana-10j-design.md`.)
