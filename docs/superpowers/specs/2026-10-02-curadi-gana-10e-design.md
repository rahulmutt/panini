# Curādi gaṇa (gaṇa 10), slice 10e — the adanta roots and the ā-garvīya

## Summary

10d's "Later slices" list put the adanta roots next, with the ā-garvīya
list. A curādi root whose upadeśa ends in a bare `a` (`10.0389 kaTa`
*kathayati*) loses that `a` before ārdhadhātuka ṇic by **6.4.48** *ato
lopaḥ ārdhadhātuke*. By 1.1.57 *acaḥ parasmin pūrvavidhau* the deleted `a`
still counts for rules that look at what precedes it, so 7.2.116 *ata
upadhāyāḥ* does not lengthen (*kathayati*, not *kāthayati*) and 7.3.86 does
not guṇate (*kuhayate*, not *kohayate*). The gaṇasūtra **10.0497** *ā
garvād ātmanepadinaḥ* makes the run `10.0440 pada` … `10.0449 garva`
ātmanepadī, as 10.0496 does for the ākusmīya.

This slice curates 92 of the 96 adanta rows and adds those two rule ids.
The four optional-ṇic adanta rows are out (see Scope).

## Scope

**In:** 92 rows, across laṭ / laṅ / loṭ / vidhiliṅ.

- **83 ubhayapadī rows**, by 1.3.74 (`PadaAssignment::Nic`, like √cur), in
  both padas: 83 × 72 = 5976 cells. These are `10.0108 mArga` and every
  bare-`a`-final row from `10.0389 kaTa` to `10.0492 Deka`, except the
  thirteen below.
- **9 ā-garvīya rows**, `10.0440 pada` … `10.0448 satra`, ātmanepada only
  (`PadaAssignment::AaGarviya`): 9 × 36 = 324 cells.

That makes 6300 cells in total. `code` is the it-stripped upadeśa in every
row (`kaTa`, `kuha`, `sanketa`, `danqa`). The final `a` is not an it, so
`dhatupatha_numbers_resolve_upstream` covers the new rows without change.

The 83 ubhayapadī rows fall into these classes by what they exercise
beyond 6.4.48. Every one of those rules already exists:

| class | rows | also exercised |
|---|---|---|
| plain | 54 | nothing else |
| ṇatva | 20 (`mArga`, `vara`, `raha` …) | 8.4.2 |
| vowel-initial | 5 (`Una`, `ansa`, `anDa`, `anka`, `anga`) | 6.4.72 āṭ in laṅ |
| ch-initial | 3 (`Cidra`, `Ceda`, `Cada`) | 6.1.73, 8.4.40 (`Cidra` also 8.4.2) |
| √spṛha | 1 (`10.0410 spfha`) | 8.4.2; vidyut also credits 8.3.110 (see Decisions) |

The ā-garvīya class includes the vowel-initial `10.0447 arTa` (6.4.72).
The six anusvāra rows (`sanketa`, `sangrAma`, `danqa`, `ansa`, `anka`,
`anga`) need **8.3.24** *naś cāpadāntasya jhali* on the root's own `n`,
then 8.4.58 (except `ansa`, whose `s` is no yay: *aṃsayati*). `anDa` takes
the same pair, with 8.4.58 restoring the `n`. 8.3.24 exists but is
guarded to rudhādi and juhotyādi, so it is widened (amendment, see
Decisions).

**Out:**

- `10.0400 pata`, `10.0451 mUtra`, `10.0456 katra` and `10.0449 garva` go
  to the optional-ṇic slice. vidyut makes their ṇic optional (Kaumudī
  2573.1–.3), and `pata` adds a ṇic-less lopa-less variant (*pAtayati /
  patayati / patati*). `garva` is also ā-garvīya: 10.0497 applies only on
  its ṇic branch, so its parasmaipada (*garvati*) exists only without ṇic.
  `AA_GARVIYA` still includes it (Decisions);
- 1.1.57 as a credited step (Decisions);
- everything on 10a's, 10b's, 10c's and 10d's out-of-scope lists.

### Decisions

**The block is a tag, `Tag::AtLopa`, set by 6.4.48 where vidyut runs it.**
vidyut credits 6.4.48 right after 3.4.114 and before 3.1.32
(`kaTa i` → `kaT i`). It sets `FlagAtLopa`, and 7.2.116 never fires
because of that flag. This engine does the same. 6.4.48 sits in
`tinanta::sanadi` after 3.4.114, deletes `ANGA`'s final `a` and adds
`Tag::AtLopa`. 7.2.116 and the sanādi 7.3.86 then decline on that tag.

Two alternatives were rejected:

- **Placing 6.4.48 after 7.3.86, so the two upadhā-readers decline on
  `kaTa`'s and `kuha`'s shape.** This needs no tag and gives the same
  traces, because nothing fires between the two positions. But the block
  becomes implicit in rule order. The next upadhā-reader placed before
  6.4.48 would silently break it, and later slices need the marker anyway:
  vidyut reads `FlagAtLopa` again in luṅ's abhyāsa (`abhyasasya.rs`), in
  guṇa (`guna_vrddhi.rs`) and in subanta.
- **Storing `kaT` with an `Adanta` tag in the data layer.** This breaks the
  upstream-resolution check and hides a real sūtra.

The tag survives 3.1.32's fold, because tags on `ANGA` are kept, so a later
stage can read it.

**1.1.57 is not credited.** vidyut records no step for it; the block
shows only as the absence of 7.2.116 and 7.3.86. Crediting it would add a
trace-only difference and a record-only rule with nothing to fail. The
comments on 6.4.48, 7.2.116 and 7.3.86 name it.

**The guṇa-stage 7.3.86 pair is untouched.** It reads the aṅga after
3.1.32, when a curādi aṅga is the vowel-final `kaTi`, and its "final-vowel
aṅgas are 7.3.84's business" early return already declines. Only the
sanādi entry, the one occasion that sees the root beside ṇic, needs the
guard.

**10.0497 mirrors 10.0496 exactly.** The data layer gets `pub const
AA_GARVIYA: RangeInclusive<&str> = "10.0440"..="10.0449"`, beside
`AKUSMIYA` and `JNAPADI`. It also gets `PadaAssignment::AaGarviya`, which
`derive` maps to `Tag::AaGarviya`. The sanādi entry credits the gaṇasūtra
on ātmanepada and blocks parasmaipada. vidyut keys the list by upadeśa
(`AA_GARVIYA` in `dhatu_gana.rs`). The ten upadeśas are exactly rows
`10.0440`–`10.0449`, in order, so a range reads it as `AKUSMIYA` reads
10.0496's. The range includes `10.0449 garva`, as `JNAPADI` includes √ci,
so it needs no change when garva is curated. A curated row is
`PadaAssignment::AaGarviya` exactly when it is curādi and in this range.

**√spṛha's 8.3.110 is not credited.** vidyut records 8.3.110 *na
raparasṛpisṛjispṛśispṛhisavanādīnām* on `10.0410 spfha`, blocking ṣatva
of the root's `s`. This engine's 8.3.59 retroflexes only an affix-initial
`s` after the aṅga, so the root's `s` never reaches it: there is nothing to
block, and the forms agree. The difference is in the trace only. The goldens
and the audit compare forms.

**8.3.24 widens to a curādi root's own `n` (amendment).** The
prototype found the engine deriving *sanketayati* where vidyut has
*saṅketayati*: 432 cells over the six anusvāra rows, the only
differences in the slice. 8.3.24 is gaṇa-guarded to rudhādi and juhotyādi
so that it never reaches the 7.1.3 `n` of *bhavanti* or *corayanti*, which
the engine has no pada-boundary notion to exclude. The widening admits a
curādi root, but searches only `ANGA`'s own characters. That `n` is inside
the dhātu, so *apadāntasya* holds by construction, and the 7.1.3 `n` (in
the tiṅ term) stays out of reach. No new rule id. One prior row moves:
10c's `10.0204 ganD` has its own `n` before `D`, and its 36 branches gain
the 8.3.24 → 8.4.58 pair. vidyut credits that pair on *gandhayate* too,
and the form is unchanged. `is_natva_target`'s fold of 8.3.24 stays: the
7.1.3 `n` of every non-rudhādi, non-juhotyādi root still needs it.

Two alternatives were rejected. Storing the post-sandhi codes (`saNketa`,
`aMsa`, …) like the `hins` / `stiG` precedents would keep a rule's output
in data. Deferring the six rows would split the class for no reason the
grammar gives.

**Four homograph pairs.** Four new rows share surfaces with curated rows:

| new row | curated row | shared cells |
|---|---|---|
| `10.0396 raha` | `10.0122 raha~` (10d, mit) | all 72 |
| `10.0405 caha` | `10.0120 caha~` (10d, mit) | all 72 |
| `10.0432 kUwa` | `10.0225 kUwa~` (ākusmīya) | the 36 ātmanepada cells |
| `10.0486 vizka` | `10.0207 vizka~` (ākusmīya) | the 36 ātmanepada cells |

These are the only collisions. 216 of the new surfaces appear in the
pre-slice goldens, all from these four rows. A check() witness from any of
these rows must expect both analyses, and its traces differ (6.4.48 against
7.2.116 + 6.4.92, or against 10.0496).

## Evidence

The probes are throwaway, in `/tmp/vidyut-full`, against vidyut-prakriya at
`8da2f90`, the vendored dhātupāṭha's commit. None of them ships.

- `examples/adanta_probe_10e.rs` derives all 96 bare-`a`-final curādi rows
  in both padas and prints each row's rule ids. The table above comes from
  grouping those ids against `10.0389 kaTa`'s:
  - every ubhayapadī row: 78 forms over 72 cells, with four multi-form
    cells (the same as √cur);
  - every ā-garvīya row: 36 forms over 36 ātmanepada cells, and none in
    parasmaipada;
  - `mUtra` and `katra`: 120 forms, with 2573.3;
  - `garva`: *garvati* in parasmaipada only, with 10.0497 and 2573.3;
  - `pata`: 198 forms, with 2573.1, 2573.2 and 7.2.116.
- `examples/adanta_trace_10e.rs`: the √kath laṭ prathama eka trace is
  3.1.25 `kaTa Ric` → 3.4.114 → **6.4.48** `kaT i` → 3.1.32 → 3.2.123 →
  1.3.78 → 3.4.78 → 3.1.68 → 3.4.113 → 7.3.84 `kaT e a ti` → 6.1.78. √kuha
  (ātmanepada) opens with **10.0497**, and its parasmaipada derives nothing.
- `examples/adanta_forms_10e.rs` dumps every vidyut form of the 92 rows.
  Intersecting those forms with the pre-slice goldens gives the homograph
  table above.
- Engine read-only checks at main `c6811b1`:
  - no curated `code` ends in `a`, so 6.4.48 can fire on no prior row;
  - 8.2.x, 8.3.24, 8.4.2, 8.4.40, 8.4.58, 6.1.73 and 6.4.72 already exist;
  - 6.4.48, 1.1.57, 8.3.110 and 10.0497 do not.

The goldens are generated from the engine and asserted equal to vidyut's
derivation sets, cell by cell. They are not typed from this spec.

**Prototype (amendment).** The whole slice was built on a throwaway
worktree before the plan was written. Results:
- the full suite, clippy and fmt-check are green;
- the golden generator derived all 6300 new cells (6798 forms) with the
  prototype engine and found every one equal to vidyut's set, once 8.3.24
  was widened. Before that it found 432 differences, all on the six
  anusvāra rows;
- the audit shows zero differences at 242 / 12996 / 14660, and the `entry`
  control fails on 36 cells;
- of the 7862 prior live-branch logs, all are byte-identical main against
  the prototype except √gandh's 36, each of which gains exactly 8.3.24 →
  8.4.58;
- every projected count held: 1444 blocks, 11816 / 790 / 343 one-, two-
  and three-form cells, ALTERNATES 1664 (348 / 340 / 340 by key), 6798
  6.4.48 credits, 324 10.0497 credits, pada-ambiguous surfaces 96 → 420.

The prototype corrected this spec in five places, all amended inline:
- 8.3.24 widens (Decisions), so it adds a guard change in `tripadi.rs`;
- two juhotyādi trace tests pin 8.3.24's and 8.4.40's credit sites and
  move: 8.3.24 gains √gandh and seven adanta rows (582 curādi branches),
  and 8.4.40 gains the three ch-initial rows' laṅ (57 branches);
- 10d's `curadi_analyses_its_jnapadi_forms` expects two analyses for
  *rahayati* and *cahayati*;
- the `check()` witnesses are adjusted to forms that are their row's alone:
  laṅ uses bahuvacana (*Onayan*, *acCidrayan*), because the prathama eka
  forms fork on 8.4.56;
- the success criterion on prior traces names √gandh's exception.

## Changes

### Data — `panini-data/src/lib.rs`

- **`AA_GARVIYA`**, documented as 10.0497's scope. Its doc names the
  uncurated `garva` and the reason it is included, and names `Tag::AaGarviya`
  and `derive` as its reader.
- **`PadaAssignment::AaGarviya`**, with a doc like `Akusmiya`'s: ātmanepada
  only, sanctioned by the gaṇasūtra 10.0497, not by a marker of the root's.
  A root carrying it is credited 10.0497 and no pada sūtra (never 1.3.12,
  1.3.74 or 1.3.78). `padas()` returns `&[Pada::Atmanepada]`. Every
  exhaustive `match` on `PadaAssignment` gains the arm. The compiler finds
  them; the grep for `Akusmiya` in Doc sweep finds the prose.
- **`aa_garviya_is_exactly_the_rows_10_0497_names`** (new): pins the range
  to upstream, as `jnapadi_is_exactly_the_rows_10_0493_names` does. The
  vendored rows `10.0440`–`10.0449` are exactly `pada gfha mfga kuha SUra
  vIra sTUla arTa satra garva`, in that order. `10.0439` and `10.0450` lie
  outside the range.
- **92 `Dhatu` rows**, `gana: Gana::Curadi`, in dhātupāṭha order among the
  curādi rows. 83 are `PadaAssignment::Nic` and 9 are `AaGarviya`. Each
  comment follows 10d's shape: number, upadeśa, artha and root, then the
  class.
  - An ubhayapadī row: "adanta: 6.4.48 ato lopaḥ deletes the final `a`
    before ārdhadhātuka ṇic, and by 1.1.57 7.2.116 / 7.3.86 do not see the
    upadhā. Ubhayapadī by 1.3.74. Slice 10e."
  - An ā-garvīya row: the same, then "ātmanepadī by the gaṇasūtra 10.0497".
  - Each homograph row names its partner.
- `curated_pada_agrees_with_upadesha_markers`: inside `AA_GARVIYA` the
  expected assignment is `AaGarviya`, "ā-garvīya, so 10.0497's".
- `curated_roots_have_expected_ganas_and_padas`: 150 → **242** rows.
  `curadi_rows_are_the_forty_seven_curated_roots` →
  `…_one_hundred_thirty_nine_…`.
- The `pada` field doc's census: 102 + 1 (√bhuj) + **93** (1.3.74) + 37
  (10.0496) + **9** (10.0497) = **242**. Re-derive every term; assume none.
  The curādi block comment moves from 47 to **139 of 509**.

### Engine — `panini-prakriya`

- **`Tag::AtLopa`** in `term.rs`. Its doc: set by 6.4.48 in
  `tinanta::sanadi` when it deletes an adanta root's final `a`; read by
  7.2.116 and the sanādi 7.3.86 as 1.1.57's sthānivadbhāva; kept through
  3.1.32 for later stages.
- **`Tag::AaGarviya`** in `term.rs`, documented like `Tag::Akusmiya`: set by
  `derive` from `PadaAssignment::AaGarviya`, and read by 10.0497 and the
  pada sūtras that decline on it.
- **`tinanta/mod.rs`**: `derive` maps `PadaAssignment::AaGarviya` →
  `Tag::AaGarviya`.
- **`tinanta/samjna.rs`**: wherever a pada sūtra declines on
  `Tag::Akusmiya` (1.3.78's guard at the 10.0496 comment), it declines on
  `Tag::AaGarviya` too, with comments naming both gaṇasūtras. The
  test helper that maps `PadaAssignment` to tags gains the arm.
- **`tinanta/sanadi.rs`**, two new entries and two guards. The module doc's
  rule list and its "Every rule self-guards" sentence gain both entries.
  - **10.0497** (gaṇasūtra), immediately after 10.0496, in the same shape:
    guard `Tag::AaGarviya`; on ātmanepada it records "A garvAd
    AtmanepadinaH"; on parasmaipada it sets `p.blocked`. Its comment
    explains the range and why `garva` is in it but not curated.
  - **6.4.48 *ato lopaḥ ārdhadhātuke***, immediately after 3.4.114, where
    vidyut runs it. Guards: ṇic at `NIC` carries `Tag::Ardhadhatuka`, and
    `ANGA` ends in short `a`. It removes that `a` and adds `Tag::AtLopa` to
    `ANGA`. Comment: the placement, the tag's later readers, and 1.1.57.
  - **7.2.116** and the sanādi **7.3.86** each gain one early return on
    `Tag::AtLopa`, with a comment citing 1.1.57: the `a` deleted by 6.4.48
    counts as still present for a rule about what precedes it, so the
    upadhā is the consonant before it, not `a` and not a laghu ik.
- **`tinanta_rule_order_is_pinned`**: the sanādi block becomes
  `"10.0496", "10.0497", "10.0493", "3.1.25", "1.3.9", "3.4.114", "6.4.48",
  "7.2.116", "6.4.92", "7.3.86", "3.1.32"`. Its doc names three gaṇasūtras.
- **Unit tests in `sanadi.rs`**:
  - 6.4.48 turns `kaTa` + ārdhadhātuka ṇic into `kaT` and sets
    `Tag::AtLopa`. It declines on `cur`, on `kaTa` with no ṇic, and on
    `kaTa` with a ṇic not yet ārdhadhātuka;
  - 7.2.116 declines on a tagged `kaT` and still lengthens an untagged
    `laq`;
  - 7.3.86 declines on a tagged `kuh` and still guṇates an untagged `cur`;
  - 10.0497 records on ātmanepada and blocks parasmaipada for a
    `Tag::AaGarviya` root, and declines without the tag.

### Goldens — `crates/panini/tests/paradigm/data/curadi.rs`

There are 83 × 8 + 9 × 4 = **700** `ParadigmRow`s and **498**
`AlternateRow`s (six per ubhayapadī root, as for √cur and the jñapādi; the
ā-garvīya roots add none). They are generated from the engine and asserted
equal, cell by cell, to vidyut's derivation sets in the same program. They
go in `curadi.rs` in both statics, wherever dhātupāṭha order puts them
among the existing curādi blocks (the rows are all `10.0108` or
`10.0389`+).

### Tests

- `derivation_set_shape_matches_the_audited_numbers`:

  | | now | after 10e |
  |---|---|---|
  | root×lakāra blocks | 744 | **1444** |
  | cells | 6696 | **12996** |
  | one-form cells | 5848 | **11816** |
  | two-form cells | 624 | **790** |
  | three-form cells | 177 | **343** |
  | forms | 7862 | **14660** |

  Each ubhayapadī root has 72 cells holding 78 forms: 68 one-form cells,
  2 two-form and 2 three-form. Each ā-garvīya root has 36 one-form cells.
  `ALTERNATES` grows by 498. The key counts `8.4.56`, `7.1.35` and
  `7.1.35+8.4.56` each grow by 166. These figures are projections: the
  plan re-derives every one from the test census.
- `pada_ambiguous_surfaces_are_exactly_these`: 96 grows. Each ubhayapadī
  root contributes √cur's four surfaces, except where a homograph already
  contributed them: `raha` and `caha` add none new. Re-derive the count from
  the census; do not take it from this spec.
- `a_kusmad_is_credited_on_exactly_the_akusmiya_cells`: its 1.3.74 half
  lists the `Nic` rows literally (ten today). At **ninety-three** rows the
  literal list goes: that half reads the curādi rows whose `pada` column is
  `Nic` and asserts there are exactly 93 of them. The 10.0496 half keeps
  reading the positional range.
- **`crates/panini/tests/trace/curadi.rs`**:
  - the module doc names 6.4.48 and 10.0497;
  - **`kaTayati_trace_deletes_the_a_and_skips_the_vrddhi`**: √kath P laṭ
    prathama eka. Asserts the full trace, that 3.4.114 < 6.4.48 < 3.1.32,
    and that neither 7.2.116 nor 7.3.86 appears;
  - **`kuhayate_trace_opens_with_a_garvad`**: √kuha Ā laṭ prathama eka.
    Opens with 10.0497, credits 6.4.48, and credits no 7.3.86 and no pada
    sūtra;
  - **`ato_lopa_is_credited_on_exactly_the_adanta_cells`**: a corpus-wide
    walk (`credited`, counting live branches). 6.4.48 is credited on exactly
    **6798** branches (83 × 78 + 9 × 36), every one on a curādi row whose
    upadeśa ends in a bare `a`. No branch of those 92 rows credits 7.2.116
    or 7.3.86;
  - **`a_garvad_is_credited_on_exactly_the_a_garviya_cells`**: 10.0497 is
    credited on exactly **324** branches, every one inside `AA_GARVIYA`, all
    ātmanepada. Parasmaipada on those rows yields only blocked prakriyās.
    The test reads the range, not a list of codes.
- **`check()`**, `curadi_analyses_its_adanta_forms`. Witnesses are derived
  from this spec's enumeration (shape class × homograph rows), not picked by
  hand. Every surface's expected analyses come from grepping the goldens:
  - plain: *kaTayati*, *gaRayati*;
  - would-be 7.3.86: *guRayati*, *kuRayate*;
  - ṇatva: *mArgayARi*;
  - vowel-initial laṅ: *Onayan*, and *ArTayata* (ā-garvīya);
  - ch-initial laṅ: *acCidrayan*;
  - anusvāra: *saNketayati*, *daRqayate*, *aMsayati* (each credits 8.3.24);
  - ā-garvīya: *padayate*, *gfhayate*, *kuhayate*. Each has one ātmanepada
    analysis; *padayati* is Invalid;
  - homographs: *rahayati* and *cahayati* each get two analyses (the mit row
    and the adanta row); *kUwayate* and *vizkayate* each get two (the
    ākusmīya row and the adanta row);
  - the forms the block prevents are Invalid: *kATayati*, *gARayati*
    (7.2.116), *kohayate* and *goRayati* (7.3.86), and so are
    *sanketayati* (no 8.3.24) and the ā-garvīya parasmaipada *padayati*
    and *kuhayati*.
- 10d's `curadi_analyses_its_jnapadi_forms`: *rahayati* and *cahayati*
  now have two analyses each; it checks the mit one.
- **`crates/panini/tests/trace/juhotyadi.rs`**:
  - `nas_capadantasya_is_credited_only_on_rudhadi_dhan_and_jan` →
    `…_rudhadi_dhan_jan_and_curadi_roots`. It admits exactly eight curādi
    rows (`10.0204` and seven adanta rows) and pins their **582** branches;
  - `shcutva_off_jan_is_credited_exactly_as_before_3f3` admits the three
    ch-initial adanta rows and pins their **57** branches beside the old 54.

  The goldens are grepped for each witness before asserting how many
  analyses it has.
- **Prior traces**: dump every prior cell's credited-rule log on main and
  on HEAD, as `trace_dump_10d.rs` does. They must be byte-identical except
  √gandh's 36 lines, each of which gains exactly ` 8.3.24 8.4.58`. 6.4.48
  needs an `a`-final `ANGA`, which no prior row has, and the two new guards
  read a tag no prior row carries. The dump proves it.

TDD order:

1. The order pin, the unit tests and the upstream pin fail first.
2. The rules, the tags and the variant make them pass.
3. The goldens, the count assertions, the trace tests and the `check()`
   test then fail on the new rows, until the data rows are in.

## Audit

Copy the committed harness from `tools/audit/`; do not rewrite it. Repoint
`/tmp/vidyut-full`'s dev-deps at the worktree before running; otherwise the
audit measures the pre-slice engine. The negative control must fail first.
Then the full run: **242 roots / 12996 cells / 14660 forms, zero
differences.** The harness's root-count comments move with it. Its
both-pada count goes from 36 to **119**, and its ātmanepada-only count
grows by 9.

## Mutation gate

The production change is small: two tags, one variant, two rules, two
guards and 8.3.24's widened guard. But the suite nearly doubles in cells. Re-measure the uncaught-suite
floor at the parallelism used, and set the cap from that measurement: 6×
the floor, rounded up to the next 10 s. Do not scale the old cap by the
cell count. Background shells die at about 60 minutes, so chunk the
campaign with `--iterate` and an `--exclude-re` for the known non-caught
pair. Pass `-o`, and copy `outcomes.json` durably before any further
invocation.

Expected outcome:

- every mutant of 6.4.48, 10.0497, the two `Tag::AtLopa` guards, 8.3.24's
  widened guard, the `AaGarviya` arms and `AA_GARVIYA` is caught;
- the rest of the non-caught set is 10d's. The two `tripadi.rs` entries
  keep their columns but move down by the lines the 8.3.24 comment adds,
  so locate them with `--list`; never compute their positions.

A change in the old set means the extra suite time pushed a survivor into
TIMEOUT, and the cap is wrong. AGENTS.md names the non-caught set verbatim.

## Doc sweep

README, AGENTS.md, `docs/ARCHITECTURE.md`, the audit harness, and the test
docs:

- totals:
  - roots: 150 → **242**;
  - cells: 6696 → **12996**;
  - forms: 7862 → **14660**;
  - blocks: 744 → **1444**;
  - curādi "open at 47 of 509" → **139 of 509**;
- the gaṇa history gains "at 139 after slice 10e curated ninety-two adanta
  roots";
- ARCHITECTURE:
  - the rule census, 139 → **141** total, and the `sanadi.rs` row of the
    stage table;
  - the tātaṅ paragraph: the parasmaipada census 88 → **171** (7.1.35's
    176 → **342** cells); the ātmanepada-only census 62 → **71**; the
    both-pada count thirty-six → **one hundred nineteen**; "88 + 62 = 150"
    → **171 + 71 = 242**;
  - the 8.4.56 paragraph: every cell count moves by 166 (two per new
    ubhayapadī root), re-derived;
- README: the curādi paragraph, the 242-root set, the multi-form census,
  the both-pada list, and the pada-ambiguous paragraph;
- AGENTS.md: `Tag::AtLopa` and the convention that a deleted sound's
  sthānivadbhāva is a tag read by the blocked rules, not rule order;
- re-derive the following from the test census rather than assuming them:
  - ALTERNATES (1166 → 1664);
  - pada-ambiguous surfaces;
  - the stage count;
  - every listing of the sanādi rules ("10.0496, 10.0493, 3.1.25, 1.3.9,
    3.4.114, 7.2.116, 6.4.92, 7.3.86, 3.1.32", wrapped or not), which gains
    10.0497 and 6.4.48;
  - every enumeration of the gaṇasūtras ("10.0493 and 10.0496", "both
    gaṇasūtras", "the only id here that is not an Aṣṭādhyāyī sūtra"), which
    gains 10.0497;
- 10d's spec: its "Later slices" paragraph gains a pointer to this spec.

The grep must cover all of `crates/` including tests, plus:

- wrapped counts and spelled-out numbers ("forty-seven", "thirty-six",
  "one hundred fifty"), with `grep -i`;
- rule-scoped counts;
- every enumeration of the sanādi rules and of the gaṇasūtras;
- every mention of "the adanta slice" and "ā-garvīya".

Re-grep recorded file:line anchors at final HEAD.

## Success criteria

- The 6300 new cells match vidyut. All 6696 prior cells and their traces
  are byte-identical between main and HEAD, except that √gandh's 36
  branches gain 8.3.24 → 8.4.58, the pair vidyut credits.
- 6.4.48 is credited on exactly the 6798 live branches of the 92 rows, and
  10.0497 on exactly the 324 ātmanepada branches of the 9 ā-garvīya rows;
  7.2.116 and 7.3.86 are credited on no adanta branch.
- The audit reports zero differences over 242 roots / 12996 cells / 14660
  forms, after its negative control fails.
- The mutation campaign catches every new mutant. The rest of the
  non-caught set is 10d's, named in AGENTS.md.
- `mise run test` and clippy are clean.

## Later slices

- Optional ṇic, next (taken by slice 10f, see
  `2026-10-02-curadi-gana-10f-design.md`). It takes the six optional-ṇic ākusmīya rows and the
  four optional-ṇic adanta rows (`pata`, `mUtra`, `katra`, `garva`), with
  Kaumudī 2573.1–.3 and 10.0497's ṇic-branch restriction on `garva`.
- √smiṅ and 7.2.115 before ṇic ride whichever slice first needs 7.2.115.
  That slice takes √gṛ (`10.0231`), √yu (`10.0235`) and √ci (`10.0124`).
- A causative (hetumaṇic) slice takes the gaṇasūtras 01.0934 and 10.0494.
- The ārdhadhātuka lakāras and luṅ will read `Tag::AtLopa` (luṅ's
  abhyāsa, as vidyut's `abhyasasya.rs` does).
