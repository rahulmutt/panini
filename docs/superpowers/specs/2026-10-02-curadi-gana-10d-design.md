# Curādi gaṇa (gaṇa 10), slice 10d — the jñapādi mit roots

## Summary

10c's "Later slices" list put the mit roots next. In curādi, mit-tva comes
from the gaṇasūtra **10.0493**. It marks the seven rows `10.0118 jYapa~` …
`10.0124 ciY` (the jñapādi) as mit, and **6.4.92** *mitāṃ hrasvaḥ* then
shortens their upadhā before ṇic. 7.2.116 *ata upadhāyāḥ* lengthens
`jYap` to `jYAp` before ṇit ṇic, and 6.4.92 shortens it back:
*jñapayati*, not *jñāpayati*.

This slice curates six of the seven rows, in both padas, and adds those two
rule ids. √ci is out (see Scope). So are 01.0934 and 10.0494, which 10c's
list said this slice would take (see Decisions).

## Scope

**In:** six rows, ubhayapadī by 1.3.74 (`PadaAssignment::Nic`, like √cur),
across laṭ / laṅ / loṭ / vidhiliṅ in both padas (6 × 72 = 432 cells):

| row | upadeśa | artha | laṭ prathama eka (P / Ā) |
|---|---|---|---|
| `10.0118` | `jYapa~` | jYAne jYApane ca | jYapayati / jYapayate |
| `10.0119` | `yama~` | parivezaRe | yamayati / yamayate |
| `10.0120` | `caha~` | parikalkane | cahayati / cahayate |
| `10.0121` | `capa~` | parikalpane | capayati / capayate |
| `10.0122` | `raha~` | tyAge | rahayati / rahayate |
| `10.0123` | `bala~` | prARane | balayati / balayate |

Every row has an upadhā `a`, so every row takes 7.2.116 and then 6.4.92.
`code` is the it-stripped upadeśa in every row. No code is shared with an
already-curated row, and none of the 432 forms appears in any existing
golden.

**Out:**

- `10.0124 ciY` (√ci). vidyut derives three forms per cell for it
  (*cayayati* / *capayati* / *cayati*). These need 7.2.115 *aco ñṇiti*,
  6.1.54 *ciṣphuroḥ ṇau* (optional ā), 7.3.36's puk, and an optional-ṇic
  rule (Kaumudī 2570). √ci goes to whichever slice first needs 7.2.115,
  together with √smiṅ, `10.0231 gf` and `10.0235 yu`;
- gaṇasūtras **01.0934** and **10.0494** (see Decisions);
- everything else on 10a's, 10b's and 10c's out-of-scope lists.

### Decisions

**Mit-tva is a dhātupāṭha range, modelled as 10.0496 was.** 10.0493 names
a contiguous run of rows, so the data layer gets
`pub const JNAPADI: RangeInclusive<&str> = "10.0118"..="10.0124"`, beside
`AKUSMIYA`. The range includes `10.0124 ciY`: √ci is mit, so the
range needs no change when √ci is curated. `derive` adds `Tag::Mit` to a
curādi root in this range, in the same way it adds `Tag::Ghu` from
`samjna::GHU`. This is a saṁjñā verdict, not a pada assignment, and
`PadaAssignment` is untouched. A per-row `mit: bool` field was rejected:
on every other row the field would be false.

**6.4.92 runs in the sanādi stage, earlier than vidyut runs it.** vidyut
credits 6.4.92 among its asiddhavat rules, after 7.3.84 has guṇated ṇic's
`i` (`jYAp + e + a + ti` → `jYap + e + a + ti`). This engine places it in
`tinanta::sanadi`, right after 7.2.116 and before 7.3.86. At that point
`ANGA` and ṇic are still separate terms, and the upadhā is one character
read. After 3.1.32 folds ṇic into `ANGA`, the root's vowel would have to be
found inside `jYApe`. That needs a ṇic-aware upadhā read in a stage that
today knows nothing of ṇic. The forms are the same either way, because no
rule between the two positions reads the root's vowel. The traces differ
only in where 6.4.92 appears. The goldens and the audit compare forms, and
the trace tests pin this engine's order.

**01.0934 and 10.0494 are left out.** 01.0934 makes am-final roots mit
(plus the four divādi roots it names). This engine has no causative, so
the only ṇic it ever adds is curādi's. 10.0494 *nānye mito 'hetau* denies
mit-tva to every curādi root outside 10.0493's list. So in this engine,
01.0934 could reach only curādi am-final roots, and there 10.0494 always
blocks it. 01.0934 would never fire, and 10.0494 would record that it blocks
a rule that does not exist. Neither could fail a test, and both would leave
mutation survivors. Both wait for a causative slice. That slice gives
01.0934 a witness (bhvādi √śam's causative, for one) and 10.0494 something
to block.

`10.0216 syama~` and `10.0218 Sama~` stay witnesses for that slice. vidyut
credits 10.0494 on them as a bare step that changes no form. Their data
comments currently say this mit slice will add 01.0934. The comments are
corrected to name the causative slice.

**√yam is the overlap witness.** `10.0119 yama~` is am-final *and* in
10.0493's list. vidyut credits 10.0494 on it (am-final, curādi) and then
mit-tva by 10.0493. This engine credits only 10.0493, and the form is the
same: *yamayati*.

## Evidence

The probes are throwaway, in `/tmp/vidyut-full`, against vidyut-prakriya at
`8da2f90`, the vendored dhātupāṭha's commit. None of them ships.

- `examples/curadi_probe_10d.rs` derives `10.0118`–`10.0124` in both padas
  and prints every rule id outside the union credited on the curated curādi
  rows. The six in-scope rows add only **10.0493** and **6.4.92**. Each
  row's parasmaipada has 42 forms over 36 cells and its ātmanepada 36 forms.
  `ciY` adds 2570, 6.1.54, 7.2.115 and 7.3.36, with up to 3 forms per
  parasmaipada cell.
- `examples/curadi_forks_10d.rs` prints every forked cell for √cur, √jñap and
  √yam (parasmaipada) and √jñap's full laṭ prathama eka trace. Each root
  forks in the same four cells:
  - laṅ prathama eka: 2 forms;
  - loṭ prathama eka: 3 forms;
  - loṭ madhyama eka: 3 forms;
  - vidhiliṅ prathama eka: 2 forms.

  The trace is 1.3.9 → **10.0493** → 3.1.25 → … → 7.2.116 (`jYAp`) →
  3.1.32 → … → 7.3.84 → **6.4.92** (`jYap`) → 6.1.78.
- `examples/curadi_engine_10d.rs` builds the six rows with this engine
  (dev-deps at `/workspace`, main `e594fe1`) and compares derivation sets
  with vidyut. Result: **432 cells, 432 differences.** Every difference is
  the unshortened upadhā alone (*jñāpayati* against *jñapayati*): shortening
  the root's `A` in the engine's forms reproduces vidyut's sets in all 432.
  This is the failing state the slice's two rules fix.

The goldens are generated from the probe's output, not typed from this
table.

**Prototype (amendment).** The whole slice was built on a throwaway
worktree before the plan was written. Results:
- the full suite, clippy and fmt-check are green;
- the audit shows zero differences at 150 / 6696 / 7862, and the `entry`
  control fails on 36 cells;
- all 7394 prior live-branch logs are byte-identical, main against the
  prototype;
- a diff-scoped cargo-mutants run catches all 8 mutants.

The prototype corrected this spec in four places, all amended inline:
- the pada-ambiguous census grows (it is not unchanged);
- the mit credit count is per branch (468), not per cell;
- the 1.3.74 list in a trace test is hard-coded;
- 6.4.92's index arithmetic needed computing once.

## Changes

### Data — `panini-data/src/lib.rs`

- **`JNAPADI`**, documented as 10.0493's scope. Its doc says the range
  includes the uncurated `10.0124 ciY`, and names `Tag::Mit` and `derive` as
  its reader.
- **Six `Dhatu` rows**, `gana: Gana::Curadi`, `pada: PadaAssignment::Nic`,
  in dhātupāṭha order among the curādi rows. Each comment follows 10a's
  shape: number, upadeśa, artha and root, then "7.2.116 ata upadhāyāḥ
  lengthens the `a` upadhā before ṇit ṇic and 6.4.92 mitāṃ hrasvaḥ shortens
  it back (mit by the gaṇasūtra 10.0493). Ubhayapadī by 1.3.74. Slice 10d."
  √yam's comment adds the am-final overlap (Decisions).
- **`jnapadi_is_exactly_the_rows_10_0493_names`** (new): pins the range to
  upstream, as `ghu_is_exactly_the_six_rows_1_1_20_names` pins `GHU`. The
  vendored `data/dhatupatha.tsv` rows from `10.0118` to `10.0124` are
  exactly the seven upadeśas `jYapa~ yama~ caha~ capa~ raha~ bala~ ciY`, in
  that order. The rows on either side, `10.0117` and `10.0125`, are outside
  the range.
- `curated_roots_have_expected_ganas_and_padas`: 144 → 150 rows.
  `curadi_rows_are_the_forty_one_curated_roots` → `…_forty_seven_…`.
  `dhatupatha_numbers_resolve_upstream` covers the new rows through the
  table it already walks.
- The `pada` field doc's census gains six 1.3.74 rows: 102 + 1 (√bhuj) +
  **10** (1.3.74) + 37 (10.0496) = **150**. The 102 and the 37 are
  re-derived, not assumed. The curādi block comment moves from 41 to
  **47 of 509**.
- The `10.0216` / `10.0218` comments: "the mit slice inherits this row …
  once it adds 01.0934" becomes the causative slice (Decisions).

### Engine — `panini-prakriya`

- **`Tag::Mit`** in `term.rs`. Its doc: set by `derive` from `JNAPADI` on a
  curādi root, and read only by 10.0493 and 6.4.92 in `tinanta::sanadi`.
- **`tinanta/mod.rs`**: `derive` adds `Tag::Mit` when the root is curādi and
  `JNAPADI.contains(&dhatu.dhatupatha)`, beside the `GHU` read.
- **`tinanta/sanadi.rs`**, two new entries. The module doc's rule list and
  its "Every rule self-guards" sentence both gain them.
  - **10.0493** (gaṇasūtra, record-only), after 10.0496 and before 3.1.25,
    where vidyut credits it, as soon as the dhātu is identified. Guard:
    `Tag::Mit` on `ANGA`. It changes no text. The tag is the verdict, as
    `Tag::Akusmiya` is 10.0496's. Comment: why the tag is set in `derive`
    and only credited here, and why 01.0934 and 10.0494 are absent.
  - **6.4.92 *mitāṃ hrasvaḥ***, after 7.2.116 and before 7.3.86. Guards:
    `Tag::Mit` on `ANGA`, and ṇit ṇic at `NIC` (`Tag::Rit`). It shortens a
    long upadhā vowel of `ANGA` through the existing `sound::hrasva_of`
    table (`A`→`a`, `I`→`i`, `U`→`u`, `F`→`f`, `X`→`x`), whose arms are
    already tested, and declines on a short one. In this slice's scope it
    only ever undoes 7.2.116. How √ci's shape reaches it is the √ci slice's
    question: vidyut shortens the last vowel there, not the upadhā. The
    upadhā index is computed once (`checked_sub(2)`); a prototype that
    wrote `chars[n - 2]` twice left an equivalent `n / 2` mutant, since
    n − 2 = n / 2 for every 3- and 4-letter witness. Comment: the placement
    argument from Decisions, including vidyut's later position.
- **`tinanta_rule_order_is_pinned`**: the expected list becomes `"10.0496",
  "10.0493", "3.1.25", "1.3.9", "3.4.114", "7.2.116", "6.4.92", "7.3.86",
  "3.1.32", …`. The doc paragraph for the sanādi stage gains a 10d
  sentence.
- **Unit tests in `sanadi.rs`**, alongside 7.2.116's:
  - 10.0493 fires on a `Tag::Mit` curādi aṅga and declines without the tag;
  - 6.4.92 shortens `jYAp` → `jYap` and hand-built `I`/`U`/`F` upadhās;
  - 6.4.92 declines on a short or non-vowel upadhā, without `Tag::Mit`, and
    without ṇit ṇic present.
- **`derive_tags_mit_on_curadi_rows_in_jnapadi_only`** in
  `derivation_tests.rs`, beside the `Tag::Ghu` test: √jñap and √yam are
  tagged; √śam, √cur and a hand-built bhvādi row numbered `10.0118` are not.
- `derivation_tests.rs`'s doc for the order pin said 10.0496 is "the only
  id here that is not an Aṣṭādhyāyī sūtra"; it now names both gaṇasūtras.

### Goldens — `crates/panini/tests/paradigm/data/curadi.rs`

48 `ParadigmRow`s (6 roots × 2 padas × 4 lakāras) and 36 `AlternateRow`s
(six per root: laṅ and vidhiliṅ prathama eka keyed `8.4.56`, and the two
loṭ tātaṅ cells keyed `7.1.35` and `7.1.35+8.4.56` each). Neither 7.2.116
nor 6.4.92 is a vikalpa key. They are generated from the engine and
asserted equal, cell by cell, to vidyut's derivation sets in the same
program. They are placed after `10.0255` in both statics, ahead of the
ākusmīya block.

### Tests

- `derivation_set_shape_matches_the_audited_numbers`:

  | | now | after 10d |
  |---|---|---|
  | root×lakāra blocks | 696 | **744** |
  | cells | 6264 | **6696** |
  | one-form cells | 5440 | **5848** |
  | two-form cells | 612 | **624** |
  | three-form cells | 165 | **177** |
  | forms | 7394 | **7862** |

  The other buckets are unchanged. Per root, 72 cells hold 78 forms:
  68 one-form cells, 2 two-form, 2 three-form. `ALTERNATES` goes from
  1130 to **1166** rows; the key counts `8.4.56` 170 → **182**, `7.1.35`
  162 → **174** and `7.1.35+8.4.56` 162 → **174**. Its doc gains a 10d
  paragraph and "OPEN at 47 of its 509 rows".
- `pada_ambiguous_surfaces_are_exactly_these`: 72 → **96**. Each new root,
  ubhayapadī and thematic like √cur, contributes √cur's four surfaces
  (`ajYapayata`, `jYapayatAm`, `jYapayetAm`, `jYapayeta`). None collides
  with a pre-slice surface.
- `a_kusmad_is_credited_on_exactly_the_akusmiya_cells`: its 1.3.74 half
  lists the `Nic` rows literally; it goes from four to ten.
- **`crates/panini/tests/trace/curadi.rs`**:
  - the module doc names 10.0493 and 6.4.92;
  - **`jnapayati_trace_lengthens_then_shortens_the_upadha`**: √jñap P laṭ
    prathama eka. Asserts the full trace and the order 10.0493 < 3.1.25 and
    7.2.116 < 6.4.92 < 3.1.32;
  - **`the_mit_rules_are_credited_on_exactly_the_jnapadi_cells`**: a
    corpus-wide walk (`credited`, which counts live branches). 10.0493 and
    6.4.92 are each credited on exactly **468** branches (42 parasmaipada +
    36 ātmanepada per root), every one inside `JNAPADI`. The check reads the
    range, not a list of codes;
  - **`syAmayate_and_SAmayate_keep_their_vrddhi`**: √syam and √śam
    (am-final, not jñapādi) keep 7.2.116 and credit neither new rule.
- **`check()`**, `curadi_analyses_its_jnapadi_forms`: all six rows, both
  padas, from this spec's Scope table. Each laṭ prathama eka form gets exactly
  one analysis, naming its root and pada, opening with 10.0493 and crediting
  7.2.116 before 6.4.92. `ajYapayata`, pada-ambiguous within √jñap, gets two
  analyses, one per pada, both mit. The lengthened forms (*jYApayati*,
  *yAmayati*, *cAhayati*, *cApayati*, *rAhayati*, *bAlayati*,
  *jYApayate*) are Invalid. The goldens were grepped: no witness collides.
- **Prior traces**: dump every prior cell's credited-rule log on main and on
  HEAD, as `trace_dump_10c.rs` does; the two must be byte-identical. The
  new rules guard on `Tag::Mit`, which no prior row carries, so nothing
  prior should move. The dump proves it.

TDD order:

1. The order pin, the unit tests and the upstream pin fail first.
2. The rules and the tag make them pass.
3. The goldens, the count assertions, the trace tests and the `check()`
   test then fail on the new rows, until the data rows and both rules are
   in.

## Audit

Copy the committed harness from `tools/audit/`; do not rewrite it. Repoint
`/tmp/vidyut-full`'s dev-deps at the worktree before running; otherwise the
audit measures the pre-slice engine. The negative control must fail first.
Then the full run: **150 roots / 6696 cells / 7862 forms, zero
differences.** The harness's root-count comments move with it, and its
both-pada count goes from 30 to **36**.

## Mutation gate

The production change is small (one tag, two rules), but the suite grows
by 432 cells. Re-measure the uncaught-suite floor at `-j 4` and set the cap
from it: 6× the floor, rounded up to the next 10 s. Pass `-o`, and copy
`outcomes.json` durably before any further invocation.

Expected outcome:

- every mutant of 10.0493, 6.4.92 and the `Tag::Mit` read is caught;
- the rest of the non-caught set is identical to 10c's.

A change in the old set means the extra suite time pushed a survivor into
TIMEOUT, and the cap is wrong. AGENTS.md names the non-caught set verbatim.

## Doc sweep

README, AGENTS.md, `docs/ARCHITECTURE.md`, the audit harness, and the test
docs:

- totals:
  - roots: 144 → **150**;
  - cells: 6264 → **6696**;
  - forms: 7394 → **7862**;
  - blocks: 696 → **744**;
  - curādi "open at 41 of 509" → **47 of 509**;
- the gaṇa history gains "at 47 after slice 10d curated six jñapādi mit
  roots";
- ARCHITECTURE:
  - the rule census, 137 → **139** total, and the `sanadi.rs` row of the
    stage table;
  - the tātaṅ paragraph: the ātmanepada-only census (62) is unchanged. The
    new rows are ubhayapadī, so they join the parasmaipada census (82 →
    **88**, and 7.1.35's 164 → **176** cells) and the both-pada count
    (thirty → **thirty-six**). "82 + 62 = 144" becomes **88 + 62 = 150**;
  - the 8.4.56 paragraph: 170 → **182** cells outright (147 → **159** laṅ
    and vidhiliṅ prathama eka cells), and the tātaṅ devoicing 164 →
    **176**;
- README: the curādi paragraph, the 150-root set, the multi-form census
  (824 → **848** cells of 6696; 612 → **624** two-form; 165 → **177**
  three-form), the both-pada list (thirty → thirty-six roots), and the
  pada-ambiguous paragraph;
- re-derive the following from the test census rather than assuming them:
  - ALTERNATES (1130 → 1166);
  - pada-ambiguous surfaces (72 → 96);
  - the stage count;
  - every listing of the sanādi rules ("10.0496, 3.1.25, 1.3.9, 3.4.114,
    7.2.116, 7.3.86, 3.1.32", wrapped or not);
- 10c's spec: the out-of-scope bullet on 10.0494 and its "Later slices"
  paragraph gain a pointer to this spec, recording that 01.0934 and
  10.0494 moved to a causative slice.

The grep must cover:

- wrapped counts and spelled-out numbers ("forty-one", "six jñapādi");
- rule-scoped counts;
- every enumeration of the sanādi rules;
- every mention of "the mit slice".

Re-grep recorded file:line anchors at final HEAD.

## Success criteria

- The 432 new cells match vidyut. All 6264 prior cells and their traces are
  byte-identical between main and HEAD.
- 10.0493 and 6.4.92 are credited on exactly the 468 live branches of the
  six `JNAPADI` rows, and nowhere else.
- The audit reports zero differences over 150 roots / 6696 cells / 7862
  forms, after its negative control fails.
- The mutation campaign catches every new mutant. The rest of the
  non-caught set is 10c's, named in AGENTS.md.
- `mise run test` and clippy are clean.

## Later slices

10a's order, now without the mit roots:

- adanta roots, with the ā-garvīya list (10.0497);
- then optional ṇic, taking the six optional-ṇic ākusmīya rows.

√smiṅ and 7.2.115 before ṇic ride whichever slice first needs 7.2.115. That
slice takes √gṛ (`10.0231`), √yu (`10.0235`) and √ci (`10.0124`). √ci also
brings 6.1.54, 7.3.36 and its optional ṇic. A causative (hetumaṇic) slice,
whenever it comes, takes the gaṇasūtras 01.0934 and 10.0494, with √syam
and √śam as their curādi witnesses.
