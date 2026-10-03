# Curādi gaṇa (gaṇa 10), slice 10f — optional ṇic

## Summary

10e's "Later slices" list put optional ṇic next. Some curādi roots take
ṇic only optionally: Kaumudī 2564 (idit roots, *daṃśayati* / *daṃśati*),
2570 (ñit and udit roots), 2573.1 (`pata`) and 2573.3 (the roots the
Kaumudī names there: `mUtra`, `katra`, `garva`; *mūtrayati* / *mūtrati*). On the ṇic-less branch the root
inflects like a bhvādi root with śap, and its pada comes from its own
markers. The ṇic-only pada gaṇasūtras 10.0496 (ākusmīya) and 10.0497
(ā-garvīya), and 1.3.74 *ṇicaś ca*, hold only on the ṇic branch. `pata`
adds 2573.2, an optional deletion of its final `a` on the ṇic branch, so
that 7.2.116 lengthens (*pātayati* beside *patayati*).

This slice builds that mechanism, adds the five ids, gives 6.1.97 and
6.1.101 a second entry for the junction a ṇic-less adanta root brings
(amendment, see Decisions), and curates the ten rows earlier specs set
aside for it: the six optional-ṇic ākusmīya rows
(10b, 10c) and the four optional-ṇic adanta rows (10e).

## Scope

**In:** 10 rows, both padas, across laṭ / laṅ / loṭ / vidhiliṅ: 720 cells.

| row | upadeśa | trigger | ṇic-branch pada | `PadaAssignment` |
|---|---|---|---|---|
| `10.0193` | `daSi~` | 2564 | ātmanepada, 10.0496 | `Akusmiya` |
| `10.0194` | `dasi~` | 2564 | ātmanepada, 10.0496 | `Akusmiya` |
| `10.0198` | `tatri~` | 2564 | ātmanepada, 10.0496 | `Akusmiya` |
| `10.0199` | `matri~` | 2564 | ātmanepada, 10.0496 | `Akusmiya` |
| `10.0227` | `vancu~` | 2570 | ātmanepada, 10.0496 | `Akusmiya` |
| `10.0230` | `divu~` | 2570 | ātmanepada, 10.0496 | `Akusmiya` |
| `10.0400` | `pata` | 2573.1 (+ 2573.2) | both, 1.3.74 | `Nic` |
| `10.0449` | `garva` | 2573.3 | ātmanepada, 10.0497 | `AaGarviya` |
| `10.0451` | `mUtra` | 2573.3 | both, 1.3.74 | `Nic` |
| `10.0456` | `katra` | 2573.3 | both, 1.3.74 | `Nic` |

On the ṇic-less branch all ten are parasmaipada by 1.3.78: none carries
an ātmanepada marker.

Codes are the it-stripped upadeśa with 7.1.58's num stored, as 10c's
generalized `stored_form` already computes: `danS`, `dans`, `tantr`,
`mantr`, `vanc`, `div`, `pata`, `garva`, `mUtra`, `katra`.

**Out:**

- The other 173 optional-ṇic rows. A throwaway scan of all 492 curādi
  rows vidyut resolves found 183 uncurated rows with optional ṇic:
  10.0499 āsvadīya 59, 2564 idit 57, 10.0498 ādhṛṣīya 51, 2570 ñit/udit 10,
  2573.3 3, and 2565, 2571, 2573.1 one each. The triggers 10.0498, 10.0499,
  2565 (ṝ-final), 2571 (`Guzi~r`) and 2572 (īdit) wait for bulk slices.
- 7.1.58 as a credited step. It stays the stored-`code` simplification
  that `07.0019 hins` documents.
- 7.3.63 *vañceḥ gatau* (Decisions).
- Everything on 10a's to 10e's out-of-scope lists.

### Decisions

**The trigger is a row-keyed table.** `panini-data` gains

```rust
pub const OPTIONAL_NIC: &[(&str, &str)] = &[
    ("10.0193", "2564"), ("10.0194", "2564"),
    ("10.0198", "2564"), ("10.0199", "2564"),
    ("10.0227", "2570"), ("10.0230", "2570"),
    ("10.0400", "2573.1"),
    ("10.0449", "2573.3"), ("10.0451", "2573.3"), ("10.0456", "2573.3"),
];
```

keyed by dhātupāṭha number as `GHU` and `JNAPADI` are. The engine never
sees upadeśa markers: `code` is it-stripped, so idit and udit cannot be
read from `danS` or `div`. 2573.3 is a closed list in vidyut too:
`hacky_unclear_has_samyoga_before_a` returns `has_u_in(["mUtra", "katra",
"karta", "garva"])`, the roots the Kaumudī names, and its shape-based body
(a conjunct before the final `a`) is commented out. That body would wrongly
admit 10e's curated `10.0469 Cidra`. (`karta` has no curādi row in the
vendored dhātupāṭha.) The table
lists only curated rows. A non-circular test (Testing) re-derives every
entry from the vendored upadeśa.

Two alternatives were rejected:

- **New `PadaAssignment` variants** (`NicOptional`, `AkusmiyaOptionalNic`,
  …). These conflate whether ṇic is optional with which pada the ṇic
  branch takes, and the variants multiply once 10.0498 and 10.0499 arrive.
- **Shape-based guards in the engine.** Idit cannot be read honestly from a
  num-stored code.

**`PadaAssignment` keeps meaning the ṇic branch's pada.** A new
`Dhatu::padas()` returns the assignment's padas plus parasmaipada when the
row is in `OPTIONAL_NIC`, in the order `[Parasmaipada, Atmanepada]`. That
is the order `PadaAssignment::padas` pins for every two-pada assignment.
Every consumer that enumerates a root's derivable padas switches to it.
The plan locates them by grepping `.padas()` across `crates/` and `tools/`.
The `d.pada.padas()[0]` call sites in unit tests pick one derivable pada
and can stay. For the six ākusmīya rows and `garva` that pada is still
ātmanepada, which still derives.

**Five vikalpa rules at the head of `SANADI`.** 2564, 2570, 2573.1 and
2573.3 come first in `tinanta::sanadi`, before 10.0496, because vidyut
decides ṇic before the pada gaṇasūtras (`dhatu_karya.rs`, "First decide
Ric-pratyaya, since this affects the scope of AkusmIya"). Each fires only
when its own id is the row's `OPTIONAL_NIC` entry, looked up through
`p.ctx.dhatupatha`. Each is `vikalpa: true` with
`bars: &["3.1.25", "2573.2"]`. The applied clone is the ṇic-less branch.
It removes the root's ṇic-branch pada tag (`Tag::Akusmiya`,
`Tag::AaGarviya` or `Tag::Nic`) from `ANGA`. The declined clone is the ṇic
branch and stays at index 0, as for every fork.

Removing the tag is what settles the pada, and no pada rule changes:

- 10.0496, 10.0497 and 1.3.74 decline on their existing tag guards;
- 1.3.78 finds a genuine śeṣa, sanctioning parasmaipada and blocking
  ātmanepada.

Nothing ṇic-dependent needs a new guard either: 3.4.114, 6.4.48, 7.2.116,
the sanādi 7.3.86 and 3.1.32 all self-guard on ṇic being present. No new
tag is needed.

**Rule names are the Siddhānta-Kaumudī's own text (amendment).** Every
`Rule.name` in this engine is SLP1 sūtra text. vidyut's `data/kaumudi.tsv`
has the text for 2570 (`YitkaraRasAmarTyAdasya RijvikalpaH`, at √ci),
2573.1 (`vA RijantaH`) and 2573.2 (`vA'danta ityeke`, both at √pat), but
not for 2564 or 2573.3. Those come from the SK text ashtadhyayi.com
publishes (github `ashtadhyayi-com/data`, `sutraani/kaumudi.txt`): 2564 is
*iditkaraṇaṃ ṇicaḥ pākṣikatve liṅgam* (at √cit, under 1.3.74),
`iditkaraRaM RicaH pAkzikatve liNgam`; 2573.3 is *adantatvasāmarthyāṇ
ṇijvikalpaḥ* (at √garv), `adantatvasAmarTyARRijvikalpaH`. The second says
why vidyut keeps 2573.3 to a list: the SK ties the option to those roots'
being adanta, and names only `mUtra`, `katra` and `garva` (with `karta`).

**The ṇic-less adanta branch needs a new junction (amendment).** The
prototype derived *katraati* where vidyut has *katrati*: 144 cells, the
four adanta rows' whole ṇic-less parasmaipada. Without ṇic, 6.4.48 never
deletes the root's final `a`, so the aṅga reaches śap still `a`-final,
which no root did before. vidyut resolves that junction first, before
the vikaraṇa meets the ending: 6.1.97 *ato guṇe* for `a` + `a`
(*katrati*; *katranti* credits 6.1.97 twice, then the śap–*anti* one), and
6.1.101 *akaḥ savarṇe dīrghaḥ* once 7.3.101 has lengthened śap
(*patāmi*). The engine's 6.1.97 and 6.1.101 read only the vikaraṇa–ending
junction, and `ADESHA` runs 6.1.101 before 6.1.97, so an arm inside either
would credit *patāni*'s two steps in the wrong order. Each id therefore
gets a second `Rule` entry at the head of `ADESHA`, each with its own
body (this stage's arms repeat their lookups rather than share a helper,
so each keeps its own mutation pin): the aṅga loses its final `a` when a
thematic śap begins with `a` (6.1.97) or `A` (6.1.101). Ids already repeat across entries
(7.3.86 has three), so no new id. The `adesha.rs` unit tests that look up
6.1.101 by id switch to a stage-local `junction(id)` helper that returns
the last entry, the one they were written for.

**2573.2 is a fifth vikalpa, on `pata`'s ṇic branch only.** It runs after
the four triggers and before 3.1.25. Its guard is `OPTIONAL_NIC`'s 2573.1
entry, and the ṇic-less branch has it barred. It deletes `ANGA`'s final
`a` (`pata` → `pat`). On that branch 6.4.48 then has no `a` to delete and
sets no `Tag::AtLopa`, so 7.2.116 lengthens: *pātayati*. Declined, 6.4.48
deletes the `a` as in 10e: *patayati*. This is the order vidyut uses
(`va_nijanta` decided first, then `2573.2` only if ṇic was taken).

**7.3.63 is not modelled.** vidyut runs 7.3.63 *vañceḥ gatau* on
`vancu~` as a vikalpa with an empty operation (`p.optional_run("7.3.63",
|_| {})`). It doubles every derivation of `10.0227` without changing any
text, and no kutva context arises in these lakāras. `run_pipeline`
collapses convergent live branches, so modelling it would change no form.
The audit compares form sets.

**Index 0.** Where the ṇic branch survives it is index 0. In the six
ākusmīya rows' and `garva`'s parasmaipada cells, the ṇic branch blocks
(10.0496 or 10.0497), so the ṇic-less form is the only live one and goes
in `PARADIGM`. In `pata`'s cells index 0 is *patayati* (both 2573.1 and
2573.2 declined). *pātayati* and *patati* go to `ALTERNATES`.

**The pinned form is the first live branch (amendment).**
`derivation_set_is_exactly_pinned` asserted `branches[0].text()`, which
in those blocked cells is a partial string. It now compares the first
live branch, which is `branches[0]` for every pre-slice cell (no prior
corpus cell holds a blocked branch). For the same reason `roundtrip`,
which asserted that no admitted pada derives a blocked branch, now
requires a live branch in every cell and admits blocked ones only on an
`OPTIONAL_NIC` row. The audit counts 612 such blocked branches.

## Forms

| rows | parasmaipada | ātmanepada |
|---|---|---|
| six ākusmīya | ṇic-less only (*daṃśati*, *vañcati*, *devati*): 42 forms each | ṇic only, 10.0496 (*daṃśayate*): 36 each |
| `garva` | ṇic-less only (*garvati*): 42 | ṇic only, 10.0497 (*garvayate*): 36 |
| `mUtra`, `katra` | ṇic + ṇic-less (*mūtrayati* / *mūtrati*): 84 each | ṇic only, 1.3.74 (*mūtrayate*): 36 each |
| `pata` | *patayati* / *pātayati* / *patati*: 126 | *patayate* / *pātayate*: 72 |

The extra 6 forms per 36 parasmaipada cells are the usual laṅ / vidhiliṅ
cartva (`-t` / `-d`) and loṭ tātaṅ (`-tāt` / `-tād` / `-tu`, `-tāt` /
`-tād` / `-a`) forks.

**Totals:** 720 cells and 984 forms, so 264 new `ALTERNATES` rows. The
corpus goes from 242 roots / 12996 cells / 14660 forms (1664 alternates)
to **252 roots / 13716 cells / 15644 forms (1928 alternates)**. Curādi goes
from 139 to **149 of 509** rows.

**Fork census (amended).** Forms per new cell: 1 form in 548 cells, 2 in
114, 3 in 46, 4 in 4, 6 in 6, 9 in 2. AGENTS.md's census counts exact
buckets (its "a second (790 cells)" is the two-form bucket), so the slice
takes the buckets to 12364 one-form, 904 two-form, 389 three-form, 23
four-form, 10 five-form, 23 six-form, 1 seven-form and 2 nine-form cells.
The first draft read the census as cumulative; it is not.
**`pata`'s parasmaipada loṭ prathama eka and madhyama eka are the new
record at nine forms each** (three readings × the tātaṅ triple), past
slice 3c2's seven-form √hā (`03.0009`) loṭ madhyama eka. The
multi-form cells:

| cell | forms |
|---|---|
| `pata` P loṭ prathama / madhyama eka | 9 |
| `pata` P laṅ / vidhiliṅ prathama eka | 6 |
| `mUtra`, `katra` P loṭ prathama / madhyama eka | 6 |
| `mUtra`, `katra` P laṅ / vidhiliṅ prathama eka | 4 |

**Homographs: none.** None of the 940 distinct new surfaces appears in the
pre-slice goldens, and no surface is shared between two of the ten rows.
The 44 within-row repeats are ordinary: the same surface in two cells of
one root, as with *-tāt* in loṭ prathama and madhyama eka.

## Evidence

The probes are throwaway, in `/tmp/vidyut-full`, against vidyut-prakriya at
`8da2f90`, the vendored dhātupāṭha's commit. None of them ships.

- `examples/nonic_scan_10f.rs` derives laṭ prathama eka in both padas for
  every curādi row vidyut resolves (492; `10.0493` onward are gaṇasūtra
  lines) and prints which optional-ṇic id each takes. That gives the class
  counts under Out, and all 139 curated rows take none.
- `examples/nonic_rows_10f.rs` derives all 72 cells of the ten rows with
  their rule ids. That gives the Forms table, the fork census and the
  homograph result (intersected with every quoted form under
  `crates/panini/tests/paradigm/data/`).
- Rule ids in those derivations that this engine never records: the
  optional-ṇic ids themselves (2564, 2570, 2573.1, 2573.3, 2573.2), 7.1.58
  and 7.3.63 (Decisions), plus bookkeeping ids absent from every existing
  trace too (1.3.1–1.3.8, 1.4.13–14, 3.2.111, 3.2.123, 3.3.161–162,
  3.4.107, 3.4.113, 8.2.66, 8.4.37, 8.4.68). The goldens and the audit
  compare forms.
- Engine read-only checks at main `6864f4f`:
  - 1.3.74 guards on `Tag::Nic`, and 1.3.78's ātmanepada arm declines only
    for `Ubhayapadin` / `Anavane` / `Nic` / `Akusmiya` / `AaGarviya`
    (`tinanta/samjna.rs`). Removing the pada tag therefore makes the
    ṇic-less branch a śeṣa.
  - 8.3.24 reads `Tag::Curadi`, a gaṇa tag that stays on both branches, so
    the ṇic-less `danS`, `tantr`, `vanc` get the same anusvāra pair as the
    ṇic branch.
  - `run_pipeline` (`controller.rs`) forks only on a firing vikalpa rule,
    keeps the declined branch at index 0, applies `bars` to the applied
    clone only, and collapses convergent live branches.

**Prototype (amendment).** The slice was built end to end on a throwaway
worktree before the plan was written:

- the golden generator found every one of the 720 new cells' derivation
  sets equal to vidyut's: 720 cells, 984 forms, 0 differences (144 before
  the aṅga–śap entries, see Decisions);
- a main-vs-prototype dump of all 14660 prior live branches' traces was
  byte-identical;
- the audit at 252 roots / 13716 cells / 15644 forms showed zero
  differences, and the `entry` control failed on 36 cells;
- `cargo mutants --in-diff` over the production diff: 17 mutants, 15
  caught, 2 unviable, 0 missed;
- pada-ambiguous surfaces 420 → 432 (`mUtra`, `katra` and `pata`'s ṇic
  branch, four each).

## Testing

**Unit tests (`tinanta/sanadi.rs`), each with a negative control:**

- For each of 2564, 2570, 2573.1 and 2573.3: it forks on a row whose
  `OPTIONAL_NIC` entry is its own id. It declines on a curādi row outside
  the table, and on a row carrying a sibling id.
- On the applied clone the ṇic-branch pada tag is gone, and 3.1.25 and
  2573.2 are barred.
- 2573.2 forks on `pata`'s ṇic branch, deletes the final `a`, and leaves no
  `Tag::AtLopa` after 6.4.48.

**Data tests (`panini-data`):**

- `optional_nic_matches_upadesha_markers` re-derives every `OPTIONAL_NIC`
  entry from the vendored upadeśa, non-circularly, as
  `dhatupatha_numbers_resolve_upstream` holds `code` to upstream:
  - last marker `i~` → 2564;
  - `u~` or `Y` marker → 2570;
  - `pata` → 2573.1;
  - upadeśa in vidyut's closed list `mUtra` / `katra` / `garva` → 2573.3
    (a list, not a shape: see Decisions).

  It also asserts that the table's rows are exactly the curated rows the
  rule applies to (ten today), so the bulk slices widen it with the rows.
  For 2564 and 2570 the marker test also runs over every curated curādi
  row: none of the 139 pre-slice rows is idit, udit or ñit, so the
  test proves the table misses no curated row.
- `curated_pada_agrees_with_upadesha_markers`: 1.3.74 rows 93 → 96,
  ākusmīya rows 37 → 43, ā-garvīya rows 9 → 10. New: every `OPTIONAL_NIC`
  row's ṇic-less pada is re-derived through 1.3.78 and must be
  parasmaipada.
- `Dhatu::padas()`: the union and its order, pinned for one row of each
  class.

**Corpus-wide trace test.** Each new id appears in a trace only on its own
rows. 10.0496, 10.0497 and 1.3.74 never appear on a branch credited with
2564, 2570, 2573.1 or 2573.3. Goldens ignore traces (3e), so the plan also
diffs every trace of the 242 pre-slice roots main ↔ HEAD, and they must be
byte-identical. That diff, not a `credited()` count, is what holds the
aṅga–śap entries inert on prior roots: `credited` cannot tell an id's two
entries apart, and the old 6.1.101 fires on √cur. The new entries get
guard-level unit tests in `adesha.rs` (fires on an `a`-final aṅga before
the matching thematic śap; declines on a consonant- or `A`-final aṅga, the
other vowel, and a non-thematic vikaraṇa).

**Goldens (`crates/panini/tests/paradigm/`):**

- `PARADIGM` and `ALTERNATES` rows for the ten roots, from the probe.
- `derivation_set_shape_matches_the_audited_numbers` raised to 252 / 13716
  / 15644.
- `check()` witnesses taken from the Forms table, not hand-picked: one per
  class × pada, i.e. √daṃś P and A, √div P (2570, guṇa), `garva` P and A,
  `mUtra` P (both forms) and A, `pata` P (three forms) and A (two). Every
  ākusmīya / `garva` parasmaipada witness asserts that the ṇic branch
  blocked and the only live form is ṇic-less.
- The census's new nine-form arm, which with `derivation_set_is_exactly_pinned`
  pins `pata` P loṭ prathama and madhyama eka's nine forms.

**Trace pins** in `crates/panini/tests/trace/curadi.rs` (amended: tests,
not an example file):

- √daṃś parasmaipada laṭ prathama eka: **2564** → 1.3.78 → 3.4.78 → 3.1.68
  … (no 3.1.25, no 10.0496);
- √daṃś ātmanepada: 10.0496 → 3.1.25 … as 10c's ākusmīya rows (held by
  the *daṃśayate* `check()` witness rather than a pin of its own);
- `pata` laṭ prathama eka, all three readings: **2573.1** (ṇic-less);
  **2573.2** → 3.1.25 → 7.2.116 (*pātayati*); 3.1.25 → 6.4.48
  (*patayati*).

**Audit.** Copy the committed `tools/audit/` harness, never rewrite it.
Repoint `/tmp/vidyut-full`'s dev-dependencies at the slice worktree. Prove
the negative control first. The result must be 0 differences over 252
roots / 13716 cells / 15644 forms.

**Mutation gate.**

- Re-measure the floor and the full uncaught-suite run at 13716 cells, at
  the parallelism actually used, and set `--timeout` above it.
- Chunk the campaign under the 60-minute background-shell limit.
- Every new mutant is caught.
- Copy `outcomes.json` somewhere durable, and name the non-caught set
  verbatim in AGENTS.md.

**Done when** the audit, the goldens, the trace diff, the mutation gate,
`mise run test` and clippy are all clean.

## Documentation

- **AGENTS.md:** the curādi coverage sentence (149 of 509, "at 149 after
  slice 10f curated the ten optional-ṇic rows"); the totals (252 / 13716 /
  15644 / 1928); the pada census; the fork census, including the new
  nine-form record that replaces √hā's "the one seven-form cell"; the
  mutation record.
- **ARCHITECTURE:** the rule-order pin count, 141 → 148 entries (five new
  ids and the second 6.1.97 and 6.1.101 entries), including the "pins all N
  ids" sentence at the top of the census paragraph; the stage table's
  `sanadi.rs` and `adesha.rs` rows; the 7.1.35 / 8.4.56 fork counts (342 →
  362, 348 → 354).
- **`tinanta/sanadi.rs`:** the module doc's rule list and "Every rule
  self-guards" paragraph; 10.0497's comment ("not curated yet").
- **`panini-data`:** the `PadaAssignment` docs that say "this engine has
  no optional-ṇic roots yet" and that `garva` "is not curated", and the
  census in `Dhatu::pada`'s doc comment.
- **Earlier specs:** 10e's "Later slices" and the deferral bullets in 10b,
  10c and 10e gain a pointer to this spec.
- **Sweep greps:** root-shape literals (`daMS`, `danS`, `pata`, `garva`,
  `mUtra`, `katra`), all of `crates/` including tests, `grep -i` for
  spelled-out counts ("Ninety-three", "thirty-seven", "seven-form"), and
  wrapped counts. Re-derive every count from the test census.

## Later slices

- Bulk optional-ṇic rows: the 57 idit (2564) and 10 ñit/udit (2570) rows
  (taken by slice 10g, all but √ci, see `2026-10-03-curadi-gana-10g-design.md`),
  then 10.0498 ādhṛṣīya and 10.0499 āsvadīya, then 2565, 2571 and 2572.
  Each widens `OPTIONAL_NIC` and its marker test.
- √smiṅ and 7.2.115 before ṇic ride whichever slice first needs 7.2.115.
  That slice takes √gṛ (`10.0231`), √yu (`10.0235`) and √ci (`10.0124`).
  √ci's optional ṇic is 2570, and it will reuse this slice's mechanism.
- A causative (hetumaṇic) slice takes the gaṇasūtras 01.0934 and 10.0494.
- The ārdhadhātuka lakāras and luṅ will read `Tag::AtLopa`.
