# The dhātupāṭha number becomes the root's identity

AGENTS.md records one residual circularity in the cross-implementation audit
and names its fix:

> the harness resolves each root to a `data/dhatupatha.tsv` entry by requiring
> vidyut to reproduce **this engine's own pinned laṭ prathama eka form**, so
> for a root whose new sūtra shapes exactly that cell — √bhañj's `Banakti`,
> √piṣ's `pinazwi`, √indh's `indDe` — the anchoring cell is the one cell the
> audit cannot independently validate. The next slice should key entry
> selection on the dhātupāṭha number instead (`07.0016` for √bhañj); that
> number currently sits only in a comment above nine of the 49 `Dhatu` rows,
> so doing so means promoting it to a real field on `Dhatu` first.

This slice does that, and takes the further step the number makes available:
it becomes the root's *identity*, retiring `Dhatu::id` and the two mechanisms
that existed only to keep `id` unique.

## Scope

New: one field on `Dhatu` (`dhatupatha`), one vendored upstream data file, one
test that checks the 49 assignments against it, and one test-private
it-stripping normalizer.

Deleted: `Dhatu::id`, the `aS.5` gaṇa-qualification mechanism, the `his`/`hins`
lookup-key-vs-stored-form split, our hand-synced `data/dhatupatha.tsv` mirror,
and AGENTS.md's "known and unfixed" paragraph.

No grammar changes. `PARADIGM` stays at 1800 cells, `ALTERNATES` at 242 rows,
2042 forms, 49 roots, seven optional rules. **No surface form changes.** Three
`artha` strings change, none of which any rule reads.

Out of scope, deferred:

- **Implementing it-stripping as real sūtras** (1.3.2, 1.3.3, 1.3.5, 1.3.9) so
  that `code` is *derived* from the upadeśa rather than curated. That is the
  honest end state — it would retire the hand-written `code` column the way
  this slice retires `id` — but upadeśa preprocessing is not the tiṅanta
  pipeline `TINANTA_RULES` models, so it needs its own pipeline concept. The
  normalizer here is test-private precisely so it does not pre-empt that
  design.
- **Auditing the 49 roots for mis-assigned pada.** Still open, still its own
  slice. √tud (`06.0001 tu\da~^`) remains the known case, and the upadeśa this
  slice puts on the record — `tu\da~^`, carrying the svarita `^` that 1.3.72
  reads — makes the mis-assignment visible in the data for the first time.
- **The eight remaining `~^r` rudhādi roots**, and 8.2.30's generalisation.
  Unchanged.

## Why identity, and not merely provenance

The number could have been added as a fifth field beside `id`, purely to key
the audit. Making it the identity instead is what lets three things be deleted
rather than accumulated.

### `id` exists only because SLP1 codes collide

`Dhatu::id` is documented as "unique lookup key, usually equal to `code`", with
two exceptions:

1. **Collision handling.** When a later gaṇa's root collides with an SLP1 form
   already in use, the incumbent keeps its bare `code` and the newcomer is
   gaṇa-qualified: kryādi's `aS` keeps id `aS`, svādi's gets `aS.5`.
2. **Rule-driven storage.** `his` is the lookup key but `hins` is stored,
   because 7.1.58 *idito num dhātoḥ* is not derivable and the *num* is kept as
   a stated simplification.

Both exceptions are artifacts of using a non-unique string as a key. The
dhātupāṭha number is unique across all 2260 upstream entries by construction,
so both dissolve: kryādi's and svādi's √aś become `09.0059` and `05.0020`,
distinct without anyone deciding which one was the incumbent, and `hins` stands
alone as a `code` with its existing 7.1.58 note, with no second string to
reconcile it against.

The `aS.5` mechanism is not merely renamed here. It is deleted, along with
`exactly_the_qualified_ids_are_qualified`'s pin that `qualified == vec!["aS.5"]`
— a test that existed to hold a workaround in place.

### Nothing user-facing depends on `id`

`Analysis::dhatu` reports `code`, not `id`, and says so in its doc comment. The
CLI prints that. So retiring `id` changes no output, no JSON shape, and no
`check` behaviour; the only consumers are the golden tables and the data
crate's own unit tests.

## The data model

```rust
pub struct Dhatu {
    /// Dhātupāṭha entry number — the unique key. Names a row of
    /// `data/dhatupatha.tsv`, and `dhatupatha_numbers_resolve_upstream`
    /// checks that the row it names is the right one.
    pub dhatupatha: &'static str,   // "07.0001"
    /// The root's SLP1 text, as it enters the derivation.
    pub code: &'static str,         // "ruD"
    pub gana: Gana,
    pub pada: PadaAssignment,
    pub artha: &'static str,
}
```

### `gana` stays, and becomes a check

The number's prefix encodes the gaṇa (`01`→`Bhvadi` … `09`→`Kryadi`), so the
`gana` field is now redundant. It stays anyway: the rule pipeline reads `Gana`
pervasively, and deriving it would mean parsing a string on every lookup to
recover an enum the caller already wanted.

The redundancy converts into a check rather than a liability. A test asserts
every row's `gana` matches its number's prefix, so a number typed into the
wrong gaṇa's block fails immediately — a class of error that is otherwise
invisible, because a wrong-gaṇa number still names a real upstream row.

Note the prefix mapping is not dense: this engine covers seven of the ten
gaṇas, so `03`, `08` and `10` have no `Gana` variant. The test maps in the
direction that exists (variant → prefix) and does not attempt the inverse.

## Verification

### The vendored file replaces our mirror

`data/dhatupatha.tsv` today is a 49-row mirror of the Rust static, hand-synced,
and its own header states it is "not parsed by any code in this repo." It is
the weakest artifact in the repo: a hand-maintained duplicate with nothing
checking it.

It is replaced in place by upstream's file — vidyut-prakriya's
`data/dhatupatha.tsv` at commit `8da2f90`, 2260 rows, 54K, verbatim. Keeping
both would mean one real data file and one hand-copied shadow of a hand-copied
shadow.

**All ten gaṇas are vendored, not filtered to the seven we cover.** Filtering
is curation, and curation is the thing this slice is trying to stop trusting.
It also keeps a future gaṇa slice from having to re-vendor.

The file's provenance sits in a header comment: upstream repo, commit
`8da2f90`, and the MIT grant (the data was sourced from ashtadhyayi.com, whose
author shared it with vidyut under MIT; vidyut's `data/README.md` records
this). MIT data in an Apache-2.0 repo is fine with the notice retained.
Pinning the commit in the header makes re-vendoring a deliberate act with a
visible diff, rather than a silent refresh.

### Three arthas are corrected against upstream

Reconciling all 49 rows turns up three divergences in our mirror. Each is
adopted from upstream:

| number | root | ours | upstream |
| --- | --- | --- | --- |
| `02.0045` | √vā | `gatigandhanayoH` | `gatiganDanayoH` |
| `07.0015` | √piṣ | `saYcUrRane hiMsAyAM ca` | `saYcUrRane hiMsAyAm ca` |
| `04.0001` | √div | `krIqAyAm` | `krIqAvijigIzAvyavahAra…gatizu` |

√vā's is a genuine SLP1 error: `ndh` reads as *d* + *h*, where the aspirate `D`
is meant. √piṣ's is an anusvāra slip. √div's looks like a deliberate
abbreviation of a ten-sense compound; upstream's full string is adopted,
because the field's purpose after this slice is provenance rather than gloss.

No rule reads `artha`, so no derivation changes. `artha` is a disambiguator for
human readers and — after this slice — one of the four things the upstream
check compares.

### The test

One test in `panini-data`'s `#[cfg(test)]` module, `include_str!`-ing the
vendored file. For each of the 49 rows it asserts:

1. the number names a real upstream row, and upstream numbers are unique;
2. that row's upadeśa it-strips to our `code`;
3. the artha matches byte-for-byte;
4. the number's prefix matches the `gana` enum.

**Step 2 is the one that breaks the circularity.** Steps 1 and 3 alone would
still pass if a number pointed at a *sibling* entry sharing the same artha, and
those siblings are abundant upstream: `vyaktAyAM vAci` covers eight bhvādi
entries, `vfdDO` fifteen. Matching on gaṇa and artha alone resolves only 29 of
our 49 roots uniquely, leaves 17 ambiguous, and fails outright on 3. Relating
the upadeśa to the code is the only one of the four assertions that cannot be
satisfied by copying back the choice we made.

Because `include_str!` sits inside `#[cfg(test)]`, the 54K is embedded in the
test binary only and does not reach the library.

### The normalizer

Test-private, roughly fifteen lines, applied to upstream's upadeśa:

1. Drop the accent marks `\` and `^` (anudātta and svarita notation).
2. **1.3.2 *upadeśe'j-anunāsika it*** with **1.3.9 *tasya lopaḥ***. Upstream
   writes an anunāsika it as `X~`, so each `X~` pair is deleted.
3. **1.3.5 *ādir ñiṭuḍavaḥ***: an initial `Yi`, `wu` or `qu` is it.
4. **1.3.3 *halantyam***: a final consonant is it.
5. **6.1.64 *dhātvādeḥ ṣaḥ saḥ*** / *ṇaḥ naḥ*: an initial `z` or `R` in the
   upadeśa surfaces as `s`/`n`, which is the form this repo stores. For
   `zwiGa~\` the following retroflex goes with it, giving `stiG` — exactly
   what the old mirror's header already recorded ("the upadeśa's initial `z`
   has already become `s`, and the retroflex `w` along with it"), now stated
   beside the `zwiGa~\` row it describes rather than in place of it.

Step 4 has the one subtlety, and getting it wrong silently corrupts the
result rather than failing loudly. **Anubandha status is decided on the
original upadeśa, before any deletion.** `paWa~` ends in `a~` — a vowel — so
halantyam does not apply and the root's own `W` survives as `paW`. `ru\Di~^r`
ends in `r`, so halantyam does apply and that `r` goes, giving `ruD`. Deciding
after step 2 instead would strip `paWa~` to `pa`, and — worse — would strip
`tfha~` to `tf`, destroying a real root-final `h` while still producing a
plausible-looking string.

One exception is named explicitly: `hins` normalizes from `hisi~`'s `his`,
because 7.1.58's *num* is stored rather than derived. This is the same
deviation the current `id` doc comment records; the slice moves it from a
field-shape workaround to a single named line in a test.

### The method cross-checks itself

Nine of the 49 numbers already sit in hand-written comments above their `Dhatu`
rows, recorded by earlier slices: `07.0001`, `07.0010`, `07.0011`, `07.0012`,
`07.0015`, `07.0016`, `07.0019`, `05.0020`, `05.0021`. All nine match what the
normalizer resolves mechanically and independently. That agreement is evidence
about the method, not just about those nine rows — a normalizer that resolves
all 49 uniquely *and* reproduces every previously hand-recorded number is
unlikely to be resolving them wrongly.

The full 49-row mapping is in the appendix.

## What changes downstream

| file | change |
| --- | --- |
| `crates/panini-data/src/lib.rs` | 49 rows gain `dhatupatha` and lose `id`; the `id` doc comment deleted; ~25 test lookups re-keyed; `id_is_the_lookup_key_and_is_unique` and the `aS.5` qualification test replaced by number-uniqueness plus the new upstream test |
| `crates/panini/tests/paradigm.rs` | 200 `PARADIGM` keys and 242 `ALTERNATES` keys re-keyed; 6 `.id` uses; the file's doc comment |
| `crates/panini/src/lib.rs` | `Analysis::dhatu`'s doc comment points at `dhatupatha`, not `id` |
| `data/dhatupatha.tsv` | our mirror replaced by the vendored upstream file |
| `AGENTS.md` | two `Dhatu::id` references (lines 143, 186), and the audit paragraph — the "residual circularity is known and unfixed" text and its "the next slice should…" sentence both go |

`crates/panini/tests/roundtrip.rs`, `trace.rs` and `tests/common/mod.rs` use no
`.id` and need no change.

### The goldens stay bare

Re-keying `PARADIGM` from `"BU"` to `"01.0001"` costs greppability: a reader
looking for √rudh's blocks can no longer `grep ruD paradigm.rs`. No per-row
comment is added to compensate. This repo has spent many commits deleting doc
claims that had gone false, and 442 uncheckable per-row comments is that
liability at scale, with nothing able to pin them. A reader resolves a number
against the 49-row `DHATUS` table, and `PARADIGM`'s doc comment says so.

### The 442 re-keyed strings are the real risk

The field addition is safe; the mechanical re-key is not. A `PARADIGM` block
whose key changed *and* whose forms changed would be self-consistent, and the
golden suite could not see it — the suite checks that the engine reproduces the
table, not that the table still says what it said yesterday.

The re-key is therefore done by script, and verified by reading `git diff` to
confirm only the key column moved. This is the same discipline the ubhayapada
plan used when it required `paradigm.rs` to be "touched only in the harness
functions, not in the data tables."

## Testing

### The gate

`mise run fmt-check && mise run lint && mise run test && mise run audit`, all
clean. Because no grammar changes, the 1800-cell paradigm suite and the trace
pins must pass **unchanged** — any golden diff other than the key column is a
defect in the re-key, not a result.

### Mutation

`mise run mutants` targets `panini-prakriya`. Everything new here is
`#[cfg(test)]` code in `panini-data`, which cargo-mutants does not mutate, so
this slice adds no mutants, no survivors, and no reason to re-measure the
timeout floor. The known permanent timeout (`tripadi.rs`'s ṇatva backward scan,
`j -= 1` → `j /= 1`) is unaffected.

This does mean the normalizer gets no mutation coverage. Its warrant is
different in kind: it must resolve all 49 roots to unique upstream entries or
the test fails, and it must reproduce nine independently hand-recorded numbers.
A mutated normalizer that still satisfies both is hard to construct.

### Cross-implementation audit

The audit harness lives out-of-repo under vidyut's `examples/`, as always. This
slice's in-repo deliverable is the field that makes the fix possible plus the
AGENTS.md rewrite; the harness is then re-keyed to select entries by number
instead of by matching this engine's own pinned laṭ prathama eka form, and the
whole-corpus audit re-run at 1800 cells / 2042 forms.

**That re-run is what demonstrates the thesis.** If keying on the number
changes any cell's outcome, the old anchoring was resolving some root to the
wrong entry, and the slice has found a real bug rather than merely tidying a
methodology. Zero differences is the expected result and the one worth
recording.

## The doc claims this slice falsifies

- `AGENTS.md`: "One residual circularity is known and unfixed" — fixed. The
  whole sentence, and the "next slice should…" instruction that follows it, are
  replaced by a description of the number-keyed selection.
- `AGENTS.md` line 143: "√aś (`Dhatu::id` `aS.5`, distinct from …)" — there is
  no `Dhatu::id`. Becomes `05.0020`, distinct from kryādi's `09.0059`.
- `AGENTS.md` line 186: "two `Dhatu::id` collisions" — the collision concept
  retires with the field.
- `crates/panini/src/lib.rs`: "`Dhatu::id` is the unique key; resolve against
  it if you need one" — becomes `Dhatu::dhatupatha`.
- `data/dhatupatha.tsv`'s header: "Not parsed by any code in this repo" — it is
  now parsed by a test, and it is no longer our file.
- `crates/panini-data/src/lib.rs`: the entire two-exception `id` doc comment,
  and the `stiG` note in the old mirror's header, which described our storage
  choice against an upadeśa the repo did not previously contain. It now sits
  beside the real `zwiGa~\` row.

## Ordering

The re-key must land after the field exists and before `id` is deleted, or the
tables reference a field that is gone. A workable order:

1. Vendor the upstream file; add `dhatupatha` to `Dhatu` and populate 49 rows;
   add the upstream-resolution test and the gaṇa-prefix test. `id` still
   present, still unique, both tests passing.
2. Correct the three arthas. (Separate, so the diff is legible: this is the one
   step that changes shipped data.)
3. Re-key `PARADIGM` and `ALTERNATES` by script; verify the diff touches only
   the key column.
4. Re-key the data crate's ~25 unit-test lookups.
5. Delete `id`, the `aS.5` qualification mechanism and its test, and the
   lookup-key doc comment.
6. Docs: AGENTS.md, `Analysis::dhatu`, `PARADIGM`'s doc comment.
7. Full gate, then the out-of-repo audit re-run.

Steps 1–5 each leave the suite green, so the slice is bisectable throughout.

## Appendix — the 49 assignments

Resolved from vidyut-prakriya's `data/dhatupatha.tsv` at commit `8da2f90` by
the normalizer described above. "6.1.64" marks a root whose stored `code`
differs from the it-stripped upadeśa by *dhātvādeḥ ṣaḥ saḥ* / *ṇaḥ naḥ*.

| number | upadeśa | `code` | gaṇa | note |
| --- | --- | --- | --- | --- |
| `01.0001` | `BU` | `BU` | bhvadi |  |
| `01.1049` | `RI\Y` | `nI` | bhvadi | 6.1.64 |
| `01.0642` | `ji\` | `ji` | bhvadi |  |
| `01.1082` | `smf\` | `smf` | bhvadi |  |
| `01.0381` | `paWa~` | `paW` | bhvadi |  |
| `01.1164` | `vada~` | `vad` | bhvadi |  |
| `01.0002` | `eDa~\` | `eD` | bhvadi |  |
| `01.1130` | `qula\Ba~\z` | `laB` | bhvadi |  |
| `01.0574` | `zevf~\` | `sev` | bhvadi | 6.1.64 |
| `01.0862` | `vftu~\` | `vft` | bhvadi |  |
| `01.0696` | `BAza~\` | `BAz` | bhvadi |  |
| `01.0694` | `Ikza~\` | `Ikz` | bhvadi |  |
| `04.0001` | `divu~` | `div` | divadi | **artha corrected** |
| `04.0091` | `Ra\Sa~` | `naS` | divadi | 6.1.64 |
| `04.0146` | `kupa~` | `kup` | divadi |  |
| `04.0073` | `ma\na~\` | `man` | divadi |  |
| `04.0069` | `yu\Da~\` | `yuD` | divadi |  |
| `04.0067` | `vi\da~\` | `vid` | divadi |  |
| `06.0001` | `tu\da~^` | `tud` | tudadi |  |
| `06.0092` | `liKa~` | `liK` | tudadi |  |
| `06.0160` | `vi\Sa~` | `viS` | tudadi |  |
| `06.0008` | `juzI~\` | `juz` | tudadi |  |
| `06.0009` | `o~vijI~\` | `vij` | tudadi |  |
| `06.0131` | `gurI~\` | `gur` | tudadi |  |
| `02.0044` | `yA\` | `yA` | adadi |  |
| `02.0045` | `vA\` | `vA` | adadi | **artha corrected** |
| `02.0001` | `a\da~` | `ad` | adadi |  |
| `02.0011` | `Asa~\` | `As` | adadi |  |
| `02.0013` | `vasa~\` | `vas` | adadi |  |
| `02.0026` | `SIN` | `SI` | adadi |  |
| `09.0058` | `kliSU~` | `kliS` | kryadi |  |
| `09.0053` | `guDa~` | `guD` | kryadi |  |
| `09.0059` | `aSa~` | `aS` | kryadi |  |
| `09.0066` | `muza~` | `muz` | kryadi |  |
| `09.0040` | `vrI\` | `vrI` | kryadi |  |
| `09.0045` | `vfN` | `vf` | kryadi |  |
| `05.0016` | `A\px~` | `Ap` | svadi |  |
| `05.0017` | `Sa\kx~` | `Sak` | svadi |  |
| `05.0012` | `hi\` | `hi` | svadi |  |
| `05.0032` | `ri\` | `ri` | svadi |  |
| `05.0020` | `aSU~\` | `aS` | svadi |  |
| `05.0021` | `zwiGa~\` | `stiG` | svadi | 6.1.64 |
| `07.0010` | `kftI~` | `kft` | rudhadi |  |
| `07.0019` | `hisi~` | `hins` | rudhadi | 7.1.58 num stored |
| `07.0012` | `Ki\da~\` | `Kid` | rudhadi |  |
| `07.0016` | `Ba\njo~` | `Banj` | rudhadi |  |
| `07.0015` | `pi\zx~` | `piz` | rudhadi | **artha corrected** |
| `07.0011` | `YiinDI~\` | `inD` | rudhadi |  |
| `07.0001` | `ru\Di~^r` | `ruD` | rudhadi |  |
