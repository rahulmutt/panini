# Curādi gaṇa (gaṇa 10), slice 10h — the ādhṛṣīya: 10.0498

## Summary

10g's "Later slices" list put the ādhṛṣīya next: the gaṇasūtra 10.0498
*ā dhṛṣād vā* makes ṇic optional for the 51 rows from `10.0338 yu\ja~` to
`10.0388 Dfza~`. This slice curates 50 of them; `10.0368 za\da~` waits for
upasargas (Out).

10f's fork carries most rows unchanged. Four things are new:

- the gaṇasūtra itself, as a fifth optional-ṇic vikalpa, checked first;
- a pada variant, `PadaAssignment::NicUbhayapada`, for the six svarita or
  ñit rows, whose ṇic-less branch is ubhayapadī by 1.3.72;
- 7.2.115 *aco ñṇiti* before ṇic (eight vowel-final rows), with a sanādi
  run of 6.1.78 it feeds, and the vārttika 7.3.37.2 (nuk for √dhū, √prī);
- 7.2.114 *mṛjer vṛddhiḥ* (√mṛj), on both branches.

A throwaway prototype built all of this and audited it against vidyut
(Evidence): zero differences across the whole corpus, and every
pre-existing root's trace byte-identical.

## Scope

**In:** 50 rows, both padas, across laṭ / laṅ / loṭ / vidhiliṅ: 3600 cells.
Codes are `stored_form`'s output.

| row | upadeśa | code | pada | row | upadeśa | code | pada |
|---|---|---|---|---|---|---|---|
| `10.0338` | `yu\ja~` | `yuj` | Nic | `10.0364` | `cIka~` | `cIk` | Nic |
| `10.0339` | `pfca~` | `pfc` | Nic | `10.0365` | `arda~^` | `ard` | NicUbhayapada |
| `10.0340` | `arca~` | `arc` | Nic | `10.0366` | `hisi~` | `hins` | Nic |
| `10.0341` | `zaha~` | `sah` | Nic | `10.0367` | `arha~` | `arh` | Nic |
| `10.0342` | `Ira~` | `Ir` | Nic | `10.0369` | `SunDa~` | `SunD` | Nic |
| `10.0343` | `lI` | `lI` | Nic | `10.0370` | `Cada~` | `Cad` | Nic |
| `10.0344` | `vfjI~` | `vfj` | Nic | `10.0371` | `juza~` | `juz` | Nic |
| `10.0345` | `vfY` | `vf` | NicUbhayapada | `10.0372` | `DUY` | `DU` | NicUbhayapada |
| `10.0346` | `jF` | `jF` | Nic | `10.0373` | `prIY` | `prI` | NicUbhayapada |
| `10.0347` | `jri\` | `jri` | Nic | `10.0374` | `SranTa~` | `SranT` | Nic |
| `10.0348` | `ri\ca~` | `ric` | Nic | `10.0375` | `granTa~` | `granT` | Nic |
| `10.0349` | `Si\za~` | `Siz` | Nic | `10.0376` | `Apx~` | `Ap` | Nic |
| `10.0350` | `ta\pa~` | `tap` | Nic | `10.0377` | `tanu~` | `tan` | Nic |
| `10.0351` | `tfpa~` | `tfp` | Nic | `10.0378` | `cana~` | `can` | Nic |
| `10.0352` | `CfdI~` | `Cfd` | Nic | `10.0379` | `vada~^` | `vad` | NicUbhayapada |
| `10.0353` | `cfpa~` | `cfp` | Nic | `10.0380` | `va\ca~` | `vac` | Nic |
| `10.0354` | `Cfpa~` | `Cfp` | Nic | `10.0381` | `mAna~` | `mAn` | Nic |
| `10.0355` | `tfpa~` | `tfp` | Nic | `10.0382` | `BU` | `BU` | Nic |
| `10.0356` | `dfpa~` | `dfp` | Nic | `10.0383` | `garha~` | `garh` | Nic |
| `10.0357` | `dfBI~` | `dfB` | Nic | `10.0384` | `mArga~` | `mArg` | Nic |
| `10.0358` | `dfBa~` | `dfB` | Nic | `10.0385` | `kaWi~` | `kanW` | Nic |
| `10.0359` | `lawa~` | `law` | Nic | `10.0386` | `mfjU~` | `mfj` | Nic |
| `10.0360` | `SraTa~` | `SraT` | Nic | `10.0387` | `mfza~^` | `mfz` | NicUbhayapada |
| `10.0361` | `mI\` | `mI` | Nic | `10.0388` | `Dfza~` | `Dfz` | Nic |
| `10.0362` | `granTa~` | `granT` | Nic | | | | |
| `10.0363` | `SIka~` | `SIk` | Nic | | | | |

Every row's `OPTIONAL_NIC` entry is `"10.0498"`, including the idit
`hisi~` and `kaWi~` and the udit `tanu~` and `mfjU~`.

**Out:**

- `10.0368 za\da~`. vidyut builds it with the upasarga ā
  (`dhatupatha.rs`: `prefixes(&["AN"])`) and derives *āsīdati* /
  *āsādayati* through 7.3.78 *sīda*. This engine has no upasargas; the row
  waits for the slice that adds them. It stays inside the exclusion
  assertion (Decisions).
- 10.0499 āsvadīya (59 rows, 279–337): the next slice.
- √ci (`10.0124`), √gṛ (`10.0231`), √yu (`10.0235`) and √smiṅ, though
  7.2.115 lands here. √ci also needs 6.1.54 and 7.3.36; each row is a
  later slice's curation (Later slices).
- 7.3.39 (√lī's nuk): vidyut keeps curādi lI out of its scope, and so does
  this slice.
- 7.1.58 as a credited step; the stored-num simplification stands.
- The causative, and everything on 10a's to 10g's out-of-scope lists.

### Decisions

**10.0498 is a sanādi vikalpa, first among the optional-ṇic forks.** It
is `skip_nic` keyed on the row's `OPTIONAL_NIC` entry, with 2564's bars
(`3.1.25`, `2573.2`), placed before 2564. vidyut checks the ādhṛṣīya
antargaṇa before idit or udit (`dhatu_karya.rs`), so `hisi~` and `kaWi~`
credit 10.0498, never 2564, and `tanu~` and `mfjU~` never 2570. Not a
sūtra of the Aṣṭādhyāyī: a dhātupāṭha gaṇasūtra, numbered as vidyut
numbers it (`DP("10.0498")`), like 10.0496 and 10.0497.

**`optional_nic_from_upadesha` learns the range.** The test helper
returns `"10.0498"` for any row in `10.0338..=10.0388` before reading
markers. The exclusion assertion in `optional_nic_matches_upadesha_markers`
narrows to the āsvadīya range `10.0279..=10.0337` plus `10.0368`.

**The ṇic-less branch of a svarita or ñit row is ubhayapadī:
`PadaAssignment::NicUbhayapada`.** On its ṇic branch the row is 1.3.74's,
both padas, exactly as `Nic`. On its ṇic-less branch vidyut credits 1.3.72
(`atmanepada.rs` reads the svarita/ñit marker of the term it is given, and
on the ṇic branch that term is ṇic). The variant tags the aṅga
`Tag::Nic` and `Tag::NicUbhayapada`; `skip_nic`, as it strips `Tag::Nic`,
adds `Tag::Ubhayapadin` when `Tag::NicUbhayapada` is present. 1.3.72 then
fires unchanged on the ṇic-less ātmanepada cells, and never on a ṇic
branch. Its doc follows `UbhayapadaAnavane`'s: name both sanctioning
sūtras and the branch each governs.

Two alternatives were rejected. Curating these rows `Ubhayapada` and
having 1.3.72 decline when ṇic is present makes 1.3.72's guard depend on
the affix, which `term.rs`'s pada-tag notes rule out. A third
`OPTIONAL_NIC` column holding the ṇic-less pada splits one row's pada
verdict across two tables.

**`Dhatu::padas` is unchanged.** Every `NicUbhayapada` row already admits
both padas through `PadaAssignment::padas`; the variant joins the
two-pada arm, parasmaipada first.

**7.2.115 *aco ñṇiti*, before ṇic, in sanādi.** Vṛddhi of an ac-final
aṅga before ñit or ṇit ṇic, placed after 6.4.92 and 7.3.37.2 and before
7.3.86. It fires on the ṇic branch of `lI`, `vf`, `jF`, `jri`, `mI`, `DU`,
`prI` and `BU`; `jF` goes straight to `jAr`. 10b to 10g deferred it to
"whichever slice first needs it": this slice is that slice. It reads the
ṇic term's own ṇit tag, not the row; the corpus-wide test (Testing) pins
that it fires on no other root.

**6.1.78 runs a second time, in sanādi, after 7.2.115.** The vṛddhi of
`lI`, `mI`, `jri`, `BU`, `DU` and `prI` leaves `lE`, `mE`, `jrE`, `BO`, …
before ṇic's `i` (`vf` and `jF` reach `-Ar` directly), and `eco 'yavāyāvaḥ` must make `lAy`, `BAv` before 3.1.32
folds ṇic in. Same sūtra and id as the `GUNA` entry; a second `Rule` in
`SANADI`. Tests that look 6.1.78 up by id use a stage-scoped lookup, as
`sanadi::tests::rule` already does, so neither entry shadows the other.

**7.3.37.2 is the vārttika *dhūñprīñor nug vaktavyaḥ*, optional.** nuk
after `DU` and `prI` before ṇic, placed before 7.2.115 so the nuk branch
escapes vṛddhi (`DUnayati`, `prIRayati`; 8.4.2 gives the ṇ). vidyut marks
it optional per Haradatta and numbers it `Varttika("7.3.37.2")`; this
engine's first vārttika id, kept in vidyut's dotted form. With 7.2.115
each root forks three ways on the parasmaipada cells
(`DAvayati` / `DUnayati` / `Davati`).

**7.2.114 *mṛjer vṛddhiḥ*, on both branches.** On the ṇic branch it sits
in `SANADI` after 6.1.78, before 7.3.86; on the ṇic-less branch in `GUNA`,
before 7.3.84, declining before a ṅit sārvadhātuka (1.1.5). Either way
it runs first and leaves the guṇa rules no laghu ik to read. `mArjati`,
`mArjayati`. Guarded on `Tag::Mrj`, set by
row number from a one-entry list `MRJ = ["10.0386"]`, the `samjna::GHU`
and `JNAPADI` precedent: root text cannot decide it once the code is
`mfj`. Adādi's √mṛj joins the list when it is curated.

**Stored codes need nothing new.** `stored_form` already derives every
code above (`hins`, `kanW` by the stored-num rule; the `\` and `^`
markers strip).

**`10.0367 arha~` resolves through 10g's verdict-aware sibling filter.**
It shares gaṇa, stored form and artha with the uncurated
`10.0257 arha~`. Once the helper is range-aware, their optional-ṇic
verdicts differ (`"10.0498"` vs `None`), so the filter already separates
them. The plan verifies this before adding any pin.

**Homographs follow the `cah` / `rah` precedent**: a shared surface
expects one analysis per row.

- Inside the slice, `tfp` (`10.0351` / `10.0355`), `dfB` (`10.0357` /
  `10.0358`) and `granT` (`10.0362` / `10.0375`) share every form.
- `mArgayARi` is also `10.0108 mArga`'s, and `mAnayate` `10.0233 mAna~`'s.
- `Bavati` (and √bhū's whole ṇic-less parasmaipada) is also
  `01.0001 BU`'s; `vadati` is also bhvādi √vad's. `trace_for` returns the
  first analysis, which stays the bhvādi row's; the plan pins that order
  rather than relying on it silently.
- The plan greps the goldens for every witness form before asserting an
  analysis count.

## Forms

Representative cells, prototype-verified against vidyut:

| row | cell | forms |
|---|---|---|
| `10.0338 yuj` | laṭ P 3sg | *yojati*, *yojayati* |
| `10.0338 yuj` | laṭ Ā 3sg | *yojayate* (ṇic only) |
| `10.0343 lI` | laṭ P 3sg | *layati*, *lāyayati* |
| `10.0345 vf` | laṭ Ā 3sg | *varate*, *vārayate* |
| `10.0372 DU` | laṭ P 3sg | *dhavati*, *dhāvayati*, *dhūnayati* |
| `10.0373 prI` | laṭ Ā 3sg | *prayate*, *prāyayate*, *prīṇayate* |
| `10.0366 hins` | laṭ P 3sg | *hiṃsati*, *hiṃsayati* |
| `10.0386 mfj` | laṭ P 3sg | *mārjati*, *mārjayati* |

A `Nic` row's parasmaipada cells hold both branches' forms and its
ātmanepada cells the ṇic branch only. A `NicUbhayapada` row's cells hold
both branches in both padas.

## Evidence

The prototype (throwaway branch `proto-10h`, not kept) curated the 50 rows
as specified here.

- **New rows:** 3600 cells, 6372 forms, zero differences against vidyut.
- **Negative control:** dropping √prī's nuk produced 72 differing cells.
- **Whole corpus**, through the committed `tools/audit/` harness with
  only its totals raised: **361 roots, 21564 cells, 29096 forms**, zero
  differences (from 311 / 17964 / 22724). ALTERNATES rows: 4760 → 7532.
- **Pre-existing roots:** every trace of all 311 dumped on main and on the
  prototype, byte-identical.
- **Rules the prototype needed beyond the four planned:** the sanādi
  6.1.78 run only. 7.2.116 (`tAnayati`, `sAhayati`), the anit `\` rows,
  `Ap`, `juz` (*joṣati*) and 8.3.24 on `hins` needed nothing.
- **Trace credits that move on new rows only:** 8.4.40 on `Cfd`
  (*acchardat*), 8.3.24 on `granT`. Rosters that enumerate credits gain
  them.

## Testing

**Goldens:** paradigm and alternates rows for all 50 roots, generated
from the engine by a harness that first asserts every cell's form set
equals vidyut's. Review packages exclude the appended goldens; diffs use
`--diff-algorithm=histogram`.

**Census:** `derivation_set_shape_matches_the_audited_numbers` and the
harness's asserts move to 361 / 21564 / 29096, with the fork and
blocked-branch census. 1.3.74's curated-row count 155 → 199.

**Rule pins:** the rule-order pin, the bars and the vikalpa list gain
10.0498, 7.3.37.2, 7.2.115, the sanādi 6.1.78 and both 7.2.114 entries.
`the_optional_nic_ids_are_credited_only_on_their_rows` gains 10.0498.

**Data tests (`panini-data`):**

- `optional_nic_matches_upadesha_markers`: 119 entries; the narrowed
  exclusion assertion.
- `curated_pada_agrees_with_upadesha_markers`: a `NicUbhayapada` arm that
  re-derives 1.3.72 for the ṇic-less branch from the vendored upadeśa's
  `~^` or `Y`, and 1.3.78 for every other optional-ṇic row. Non-circular.
- `dhatupatha_numbers_resolve_upstream`: `10.0367` resolves uniquely
  (Decisions).

**Unit tests, each guard in both directions:**

- 10.0498 fires on an in-range row and declines out of range and on a
  row with another `OPTIONAL_NIC` id.
- `skip_nic` adds `Tag::Ubhayapadin` only under `Tag::NicUbhayapada`;
  1.3.72 never fires on a `NicUbhayapada` ṇic branch.
- 7.2.115: fires on an ac-final aṅga before ṇic, declines on a
  hal-final one and with no ṇic.
- 7.3.37.2: only `DU` and `prI`, only before ṇic.
- 7.2.114: only `Tag::Mrj`, in each stage; the `GUNA` entry declines on
  a ṇijanta aṅga and before a ṅit sārvadhātuka.
- sanādi 6.1.78: `lE` + `i` → `lAy`; declines with no ṇic.

**Corpus-wide tests:** 10.0498, 7.2.115, 7.3.37.2, 7.2.114 and the sanādi
6.1.78 each fire only on this slice's rows: 7.2.115 on exactly the eight
vowel-final rows, the sanādi 6.1.78 on exactly six of them. Goldens ignore traces, so the plan also
diffs every pre-slice root's trace main ↔ HEAD and requires no change.

**Witnesses (`check()`):** derived from this spec's tables, not hand
picked: one ṇic-less and one ṇic witness per pada assignment and per new
rule (`yojati`, `layati` / `lAyayati`, `varate`, `DUnayati`,
`prIRayate`, `mArjati`, `hiMsati`, `kaRWati`), each asserting 10.0498
first in a ṇic-less trace and 3.1.25 with no 10.0498 in a ṇic trace. Plus
every homograph above, with the counts the goldens grep confirms.

**Mutation gate:**

- Scoped with `--in-diff` to the slice's production diff.
- Before setting `--timeout`, re-measure the floor at 21564 cells: a full
  uncaught suite run at the parallelism the campaign will use.
- Run with `-o` to a dedicated directory and copy `outcomes.json` durably.
- Chunk with `--iterate` so no background shell outlives about 60 minutes.
- AGENTS.md names every missed or timeout mutant verbatim.

**Audit:** the committed `tools/audit/` harness, copied, totals raised;
negative control first. vidyut's dev-dependencies are repointed at the
worktree for the run and restored after.

## Documentation

- **AGENTS.md:** curādi "at 258 after slice 10h curated the fifty
  ādhṛṣīya rows"; the corpus counts (311 / 17964 / 22724 → 361 / 21564 /
  29096); the pada census; the fork and blocked census (√dhū and √prī are
  new three-form cells); the audit record and the mutation record.
- **`docs/ARCHITECTURE.md`:** the rule-order pin count, which **does**
  change (new ids and the second 6.1.78 and 7.2.114 entries); the sanādi
  stage description (7.2.115, 6.1.78, 7.2.114, 7.3.37.2); the fork counts
  and census paragraphs.
- **`tools/audit/README.md`:** the corpus totals.
- **`panini-data`:** the `OPTIONAL_NIC` doc (five ids now, 10.0498
  first), the `Dhatu::pada` and `Dhatu::padas` docs (the ṇic-less branch
  is no longer always 1.3.78's), the new variant's doc, the `JNAPADI` doc's
  "waits on 7.2.115" note on √ci (now: waits on its own slice).
- **`tinanta/sanadi.rs`:** the module doc's rule list.
- **Earlier specs:** 10g's "Later slices" gains a pointer to this spec,
  and its √ci bullet is rewritten (7.2.115 is in).
- **Sweep greps:**
  - root-shape literals for every new code
  - all of `crates/`, including tests
  - every enumeration of sibling rules: the optional-ṇic ids
    ("2564, 2570, 2573.1, 2573.3"), the sanādi rule list, and "6.1.78"
  - "7.2.115" across docs and comments: every "deferred / whichever slice
    first needs" phrasing
  - `grep -i` for spelled-out counts ("sixty-nine", "155 curādi")
  - wrapped counts
  - Re-derive every count from the test census at final HEAD.

## Later slices

- 10.0499 āsvadīya (59 rows, 279–337). Teaches the helper its range and
  drops the rest of the exclusion assertion. (Slice 10i took it: see
  `2026-10-04-curadi-gana-10i-design.md`.)
- The one-row triggers 2565 (`pF`) and 2571 (`Guzi~r`), both taken by slice
  10i. 2572 (īdit) is unreachable: every īdit curādi row is āsvadīya or
  ādhṛṣīya, and vidyut checks both antargaṇas first.
- √gṛ (`10.0231`), √yu (`10.0235`) and √smiṅ: 7.2.115 is in, so these are
  curation slices. √ci (`10.0124`) also needs 6.1.54 and 7.3.36.
  (Slice 10j took √smiṅ, √gṛ, √yu and √ci: see `2026-10-04-curadi-gana-10j-design.md`.)
- Upasargas, and with them `10.0368 za\da~` (7.3.78).
- A causative (hetumaṇic) slice takes 01.0934 and 10.0494.
