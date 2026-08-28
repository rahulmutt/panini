# Splitting the golden test files — paradigm.rs and trace.rs by gaṇa

Every spec since rudhādi 7e has deferred the same item with the same reason:

> **Splitting `crates/panini/tests/paradigm.rs`**, headed past 6,000 lines.
> Still its own slice, for 7e's reason: a large mechanical diff is the worst
> possible neighbour for the data the audit exists to validate. It is the
> natural next item once rudhādi closes.

Rudhādi closed at 25/25 in the √bhuj/1.3.66 slice. This is that slice. It is
the first slice in the repo that touches **no engine code and no golden
content** — it moves tests, and its entire risk budget is spent proving the
move lost nothing. `trace.rs` (1,838 lines, growing every slice) gets the
same treatment in the same slice, so the test tree is reorganized once, not
twice.

`paradigm.rs` stands at 6,428 lines: `PARADIGM` (316 blocks, 2844 cells),
`ALTERNATES` (494 rows), `VIKALPA_RULES`, two helpers, and fifteen tests.
`trace.rs` holds three helpers and ~70 named-witness tests ordered by the
gaṇa progression.

## The constraint that shapes the design

Every top-level file in `tests/` is a separate test binary. The
mutation-timing lore in AGENTS.md — uncaught-run floors, the 1200s cap
arithmetic, the `-j` interactions — is calibrated against the current
two-binary suite, and every extra binary adds link time to each of the
~500 runs of a mutation campaign. So the split uses Cargo's **directory
form**: `tests/paradigm/main.rs` and `tests/trace/main.rs` each remain one
binary. Two binaries before, two after; the timing lore carries over
unchanged. (`tests/common/` has no `main.rs` and is not compiled as a
binary — unchanged.)

Rejected alternatives: one binary per gaṇa (multiplies per-mutant link time
and invalidates every measured timing figure); `include!`-fragment tricks to
keep `PARADIGM` a single `const` (macro or build-script machinery to
preserve a `const`-ness nothing depends on).

## Layout

```
crates/panini/tests/
  common/mod.rs                 unchanged, stays shared
  roundtrip.rs                  unchanged
  paradigm/
    main.rs                     all fifteen #[test] fns, both helpers,
                                VIKALPA_RULES
    data/
      mod.rs                    row type aliases; declares the gaṇa modules;
                                concatenates the statics
      bhvadi.rs  divadi.rs  tudadi.rs  adadi.rs
      kryadi.rs  svadi.rs   rudhadi.rs
  trace/
    main.rs                     module declarations only
    helpers.rs                  trace_for, cell_trace, at
    bhvadi.rs  divadi.rs  tudadi.rs  adadi.rs
    kryadi.rs  svadi.rs   rudhadi.rs
                                (one file per gaṇa that has witnesses; a
                                gaṇa with none gets no file)
```

Each `data/<gaṇa>.rs` exports that gaṇa's rows:

```rust
pub const PARADIGM: &[ParadigmRow] = &[ /* this gaṇa's blocks */ ];
pub const ALTERNATES: &[AlternateRow] = &[ /* this gaṇa's rows */ ];
```

and `data/mod.rs` concatenates them:

```rust
pub type ParadigmRow = (&'static str, &'static str, Pada, [&'static str; 9]);
pub type AlternateRow = (&'static str, &'static str, Pada, usize, &'static str, &'static str);

pub static PARADIGM: LazyLock<Vec<ParadigmRow>> = LazyLock::new(|| {
    [bhvadi::PARADIGM, divadi::PARADIGM, /* … */].concat()
});
// ALTERNATES likewise.
```

`LazyLock` (stable since 1.80; toolchain is 1.98) keeps the names and nearly
all call sites intact: `PARADIGM.len()` and `.iter()` work through deref;
only bare `for x in PARADIGM` loops gain an `.iter()`.

The partition is mechanical and unambiguous: every row's first column is a
`Dhatu::dhatupatha` number, and its prefix names the gaṇa — `01.*` bhvādi,
`02.*` adādi, `04.*` divādi, `05.*` svādi, `06.*` tudādi, `07.*` rudhādi,
`09.*` kryādi. The two √aś rows land in different files (`09.0059` kryādi,
`05.0020` svādi) exactly as the number-identity slice intended.

A future gaṇa slice adds one data file plus its registration lines in
`data/mod.rs` (and, when it has trace witnesses, one trace module). A future
rule slice touches only the gaṇa files it changes. The audit-to-golden
transcription workflow is unchanged in substance: audited output is
transcribed into the owning gaṇa's `data/<gaṇa>.rs` instead of into the
monolith.

## Row order: the partition reorders, and that is safe

Neither const is gaṇa-contiguous today. `PARADIGM` interleaves divādi and
tudādi blocks (the two gaṇas were curated in one slice); `ALTERNATES` is
grouped by slice/lakāra chronology, not by gaṇa. A per-gaṇa partition
therefore necessarily changes row order relative to a naive concatenation of
the old file.

This is safe because row order is semantically irrelevant: every consumer
test builds sets, filters, finds, or counts — `derivation_set_is_exactly_pinned`,
`paradigm_covers_every_enumerable_cell`, `every_alternate_names_a_real_cell`,
`pada_ambiguous_surfaces_are_exactly_these` and the rest are all
order-insensitive, and `ALTERNATES` names cells by index within a block,
never by position in the const. The verification below proves **multiset
identity**, which is the property the tests actually consume.

Within each gaṇa file, rows keep their relative order from the monolith (a
stable partition), so group comments that head runs of related rows keep
their rows beneath them. A comment whose group spans more than one gaṇa is
placed, at the split, with the gaṇa its text most concerns, or duplicated
with an ownership note — decided case by case in the plan, never dropped.

## What moves, and what changes in what moves

**paradigm side.** The fifteen `#[test]` fns, the two helpers, and
`VIKALPA_RULES` move to `paradigm/main.rs` verbatim, doc comments included.
The doc comments on the consts themselves — the row-format contract, the
two-√aś note, the alternates-key discipline — move to `data/mod.rs`, which
becomes the front door a reader hits first. Per-gaṇa files carry a one-line
header naming the gaṇa and its number prefix; inline comments travel with
their rows. Test code is untouched except the mechanical `.iter()`
adjustments and the `use` of the `data` module.

**trace side.** The three helpers move to `trace/helpers.rs`. Each witness
test moves whole — doc comment included, relative order within its group
preserved — into the file of **the gaṇa of the root it derives** (√bhū,
√labh, √vṛt, √edh → `bhvadi.rs`; √yā, √ad, √ās, √vas… → `adadi.rs`; and so
on). The assignment is derived from each test's root, enumerated in the
plan, not guessed from names. Test fn names do not change.

**Unchanged:** `common/mod.rs` (reached from the directory-form binaries via
`#[path = "../common/mod.rs"] mod common;`), `roundtrip.rs`, every test
name, every assertion, every golden string. No engine crate is touched. The
`mise` tasks and CI run `cargo test` on the workspace and need no config
change.

## Verification: structural proof, no mutation campaign

This slice moves the very tests that catch mutants, so a silently dropped
row or test would surface only as lost coverage. The merge gate is a
structural proof that nothing was lost, in place of a campaign:

1. **Golden-content identity.** Before the split, a temporary dump test
   (never committed) prints every `PARADIGM` and `ALTERNATES` row in a
   canonical one-row-per-line form, **sorted** by (number, lakāra, pada
   — and for alternates, cell and form and key). Capture the output. After
   the split, the same dump runs against the concatenated statics. The two
   captures must be byte-identical. Duplicates are printed, not collapsed,
   so an accidental duplication or loss is visible. This proves no row was
   lost, duplicated, or edited in transit.
2. **Test-inventory identity.** `cargo test -p panini -- --list` before and
   after: same count and same set of test fn names, module prefixes
   normalized away. This proves no `#[test]` was dropped in the trace
   reshuffle.
3. **Existing pinned invariants as backstop.**
   `derivation_set_shape_matches_the_audited_numbers` (2844 cells, 494
   alternates), `paradigm_covers_every_enumerable_cell`, and
   `every_form_validates_and_matches` still pass and independently
   re-detect most transcription accidents.
4. **Full suite green** via `mise run test`.

No mutation campaign runs: `panini-prakriya` is untouched and the proof
above establishes test-content identity directly. The partition itself
should be scripted (partition rows by number prefix) rather than
hand-copied, to shrink the transcription surface the proof exists to check.

## Doc sweep

Current-state docs that name the old paths are updated; historical specs and
plans are not rewritten (repo convention).

- `README.md` — the `pada_ambiguous_surfaces_are_exactly_these` pointer.
- `docs/ARCHITECTURE.md` — the `paradigm_covers_every_enumerable_cell`
  pointer.
- `tools/audit/README.md` — both references, including the one that tells
  future slices where audited output is transcribed (now
  `tests/paradigm/data/<gaṇa>.rs`).
- `AGENTS.md` — the golden-gate path in "Rules of the codebase"; the
  `paradigm.rs:5934` line-anchored reference to `key_count("6.4.107") == 8`,
  re-anchored as file + test name (line anchors are fragile; the sweep
  replaces this one with a name anchor rather than a fresh line number); and
  one new line in the mutation-testing section noting the suite was
  restructured into directory form with the timing-relevant shape (two
  binaries) preserved.

## Deliberately out of scope

- Any change to test content, assertions, or golden strings.
- Any engine change.
- Splitting `rudhadi.rs` further. It will land at roughly 2,800 data lines —
  much the largest gaṇa file, and acceptable: the point of the split is
  per-gaṇa ownership, not a line quota, and every line of it is one gaṇa's
  data under one header.
- `roundtrip.rs` and `common/mod.rs`.
- A mutation campaign (see Verification).

## Success criteria

- Both dumps byte-identical; test inventory identical; full suite green.
- Two test binaries before and after (`cargo test -p panini` lists exactly
  `paradigm`, `roundtrip`, `trace`).
- Every file under `tests/` holds one gaṇa's data or one binary's tests;
  the largest, `paradigm/data/rudhadi.rs` at roughly 2,800 lines, is less
  than half the monolith and wholly one gaṇa's. The next gaṇa slice can
  land its goldens by adding one data file plus registration lines.
- Doc sweep applied; no current-state doc names `tests/paradigm.rs` or
  `tests/trace.rs`.
