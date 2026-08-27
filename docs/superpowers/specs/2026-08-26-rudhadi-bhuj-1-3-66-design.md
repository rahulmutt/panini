# rudhādi (gaṇa 7), √bhuj and 1.3.66 — closing the gaṇa at twenty-five

Slice 7f left rudhādi at twenty-four of its twenty-five roots, with the last
one costed rather than merely deferred:

> **√bhuj (`07.0017`)**, and with it 1.3.66 *bhujo'navane*. vidyut derives all
> 72 cells (79 forms), and 1.3.66 is the **only** rule it invokes that this
> engine lacks — a root-keyed pada assignment structurally identical to the
> 1.3.72 rule already in `samjna.rs`, with no new phonology at all (√bhuj is
> √yuj with a `B`). What keeps it out is not cost but a ruling.

This slice makes that ruling and ships the root. rudhādi closes at 25/25.

## The ruling: anavane is recorded, not modelled

1.3.66 *bhujo'navane* sanctions ātmanepada for √bhuj in senses other than
protecting (*avane*); in the protecting sense the root falls to 1.3.78's
śeṣa. Neither this engine nor vidyut models sense, so the slice ships an
**unconditional ubhayapada assignment** — both padas derive, 72 cells — with
*anavane* recorded as an unimplemented sense restriction.

The precedent is 1.3.72 itself, whose *kartrabhiprāye kriyāphale* is exactly
this kind of condition: both arms derive, each trace credits the sūtra that
sanctioned it, and the reader selects by sense. Like 1.3.72, 1.3.66 is **not**
vikalpa — pada is a context coordinate, so the two readings are two CELLS,
never two branches of one cell, and they must not enter the fork machinery
`docs/ARCHITECTURE.md` reserves for anyatarasyām / vā / vibhāṣā.

The restriction is recorded where 1.3.72 records its own: the rule's comment
block in `samjna.rs`, and the doc comments of the new `PadaAssignment` variant
and `Tag` below.

## Scope

**New rules (1):** 1.3.66 *Bujo'navane* in `samjna.rs`, between 1.3.12 and
1.3.72.

**Widened rules (1):** 1.3.78's ātmanepada arm declines on the new tag as
well as on `Ubhayapadin`.

**New phonology (0).** This is 7f's probe finding, restated as this slice's
central claim: every √bhuj cell outside the pada sanction derives on rules
already in the pipeline. √bhuj is √yuj with a `B` — śnam, the laṅ aṭ-augment,
8.2.30's kutva on the final `j`, all already witnessed by √yuj. The audit
gate below is what proves the claim.

**New data:** one `Dhatu` row; one `PadaAssignment` variant and one `Tag`; 8
`PARADIGM` blocks (1 root × 2 padas × 4 lakāras); ~7 `ALTERNATES` rows (the
tilde resolved by the audit); two trace pins.

**Changed:** `rudhadi_rows_are_the_twenty_four_curated_roots` renamed and
extended; `curated_pada_agrees_with_upadesha_markers` gains its first
documented exception; the corpus totals in `panini-data`,
`crates/panini/tests/paradigm.rs` and `tools/audit/panini_full_audit.rs`; the
documentation sites enumerated under "The doc claims this slice falsifies".

## The keying mechanism

No rule in this engine reads a root's identity; data-layer facts flow through
`PadaAssignment` into a `Tag` (the match in `tinanta/mod.rs`), and rules read
tags. 1.3.66 keeps that shape. The alternatives — matching the root by
code/number inside `samjna.rs`, or special-casing `07.0017` at the tag-mapping
boundary — were considered and rejected: the first hardcodes an identity in
two rules (1.3.66 to include, 1.3.72 to exclude), the second leaves the data
row silent about the one fact it exists to curate.

### Data layer

- **`PadaAssignment::UbhayapadaAnavane`** — both padas derive, but the
  ātmanepada arm is sanctioned by 1.3.66's root-keyed exception rather than by
  a svarita/ñit marker. Its `padas()` arm returns the same parasmaipada-first
  pair as `Ubhayapada` (the ordering `ubhayapada_padas_are_parasmaipada_first`
  pins carries over unchanged), and
  `padas_maps_each_assignment_to_its_derivable_padas` grows the fourth
  assertion. The doc comment states the anavane ruling.
- **The √bhuj row**: `("07.0017", "Buj", Gana::Rudhadi,
  PadaAssignment::UbhayapadaAnavane)`. No stored-form deviation:
  `strip_anubandhas("Bu\ja~")` yields `Buj` directly, and
  `dhatupatha_numbers_resolve_upstream` verifies the resolution as it does for
  every row.

### The consistency-test exception

`pada_from_upadesha("Bu\ja~")` correctly derives **Parasmaipada**: the `\`
sits on the root vowel, not on an it — the exact conflation that function's
doc comment warns against — and the final `a~` carries no accent, so the
upadeśa has no pada anubandha at all. The curated row says
`UbhayapadaAnavane`. That divergence **is** 1.3.66: a root-keyed assignment no
marker can carry, which is why Pāṇini needed a sūtra naming the root.

`curated_pada_agrees_with_upadesha_markers` therefore gains its first
documented exception, keyed by `07.0017` and asserting both sides — derived
`Parasmaipada`, curated `UbhayapadaAnavane` — so the test fails loudly if
either the upadeśa reading or the curated column ever drifts. The exception's
comment cites 1.3.66 as the reason the two disagree.

### Engine

- **`term.rs`**: `Tag::Anavane`, doc-commented with the same care
  `Ubhayapadin` gets. It means: **1.3.66's root-keyed condition holds** — set
  today on exactly √bhuj. It is deliberately distinct from `Ubhayapadin` so
  that √bhuj can never reach 1.3.72 and the trace can never credit the wrong
  sūtra — the mirror image of the √indh counterexample that forced
  `Ubhayapadin`'s own naming.
- **`tinanta/mod.rs`**: one new match arm, `UbhayapadaAnavane =>
  t.add(Tag::Anavane)`.
- **`samjna.rs`**: rule 1.3.66 *Bujo'navane*, `RuleKind::Vidhi`, `vikalpa:
  false`, slotted between 1.3.12 and 1.3.72 in numeric order. It is an exact
  structural twin of 1.3.72: guard on `Tag::Anavane`; on the ātmanepada
  reading, snapshot, record, sanction; on the parasmaipada reading, **decline
  — never block** — because 1.3.78 immediately below is what sanctions that
  arm, exactly as it does for 1.3.72's roots. The module header's sūtra list
  gains 1.3.66.
- **1.3.78's ātmanepada arm** declines when the ANGA has `Ubhayapadin` **or**
  `Anavane`; only the genuine śeṣa (no pada tag at all) still blocks.

## Corpus growth

| invariant | before | after |
|---|---|---|
| roots | 66 | 67 |
| root×pada×lakāra blocks | 308 | 316 |
| cells | 2772 | 2844 |
| ALTERNATES rows | 487 | ~494 |
| forms | 3259 | ~3338 |

The tildes are 7f's probe figures (79 forms over 72 cells); the audit is the
authority, and the totals are transcribed from it, not from this document.
All three assertion sites move together: `panini-data/src/lib.rs`,
`crates/panini/tests/paradigm.rs`, `tools/audit/panini_full_audit.rs`.

## The trace pins

Two, both on √bhuj, chosen to witness the pada sanction from both sides:

1. An ātmanepada cell whose trace credits **1.3.66** — the new sūtra doing
   the one job it exists for.
2. The corresponding parasmaipada cell crediting **1.3.78** — the śeṣa arm
   that an unconditional assignment must leave open, proving 1.3.66 declined
   rather than blocked.

## Per-rule guard tests

Mirroring the existing 1.3.72 pins in `samjna.rs`:

- 1.3.66 fires on the ātmanepada reading and declines without blocking on the
  parasmaipada one, for an `Anavane`-tagged term.
- 1.3.72 never fires for an `Anavane`-tagged term — the wrong-sūtra-credit
  case, pinned.
- 1.3.78 declines (does not block) an `Anavane`-tagged term's ātmanepada
  reading, and still blocks the genuine śeṣa.

## Verification

1. **Audit first, transcribe second.** Copy the committed harness at
   `tools/audit/` — never rewrite it — extend it to 67 roots, and run √bhuj's
   72 cells against vidyut-prakriya at the audited commit
   `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, with full sūtra traces diffed.
   Expected: byte-identical forms, and 1.3.66 the only rule id newly credited.
2. **Gates**: `mise run fmt-check | lint | build | test`, the golden suite in
   the foreground with an explicit long timeout (~8 minutes at 2844 cells and
   growing).
3. **Mutation campaign** per the standing discipline in AGENTS.md: the
   cargo-mutants binary invoked directly, `-j 4 --timeout 2400`,
   `--test-workspace=true`, `timeout.txt` checked alongside `missed.txt`.
   Expected timeouts: exactly the one known-permanent ṇatva backward-scan
   hang, identified by its shape (`j /= 1` making the loop index constant),
   never by line number. Re-measure the uncontended uncaught floor at 2844
   cells with a standalone `mise run test` and append the figures to
   AGENTS.md's running narrative — this slice grows cells +2.6%, and the
   narrative's standing instruction is to re-measure the floor, not scale it.

## The doc claims this slice falsifies

- **AGENTS.md's √bhuj paragraph** (the rudhādi enumeration around lines
  661–674). 7f's design already ruled on this: the current framing — that
  √bhuj is out because its pada forks "on an axis this engine models" nothing
  of — overstates the obstacle, and must be replaced by what is now true: all
  25 rudhādi roots curated, 1.3.66 implemented as an unconditional ubhayapada
  assignment with *anavane* recorded as unimplemented. The "24 curated + √bhuj
  = 25, so **1** of the 25 remains out" arithmetic goes to zero-out.
- The census-test comment naming twenty-four roots and its "only √bhuj is
  out" commentary.
- Every stale corpus count (`66`, `308`, `2772`, `3259`, `487`,
  "twenty-four"), swept the way the doc-sweep lesson demands: grep the seven
  vikalpa rule ids and the files no task owns, and hand-check wrapped or
  rule-scoped counts the grep cannot match. `ARCHITECTURE.md`, `README.md`
  and `paradigm.rs`'s header are in the sweep. Past plans and specs are
  immutable records — not swept.

## If the audit shows a difference

Stop and diagnose before touching golden data. The 8.2.30/8.2.39 slice is the
precedent: its first run found four differing cells and that was the
*finding*, not a setback. Transcribe `PARADIGM` and `ALTERNATES` from audited
output only, never from 7f's probe or from this document.

## Deliberately out of scope

- **Splitting `crates/panini/tests/paradigm.rs`**, headed past 6,000 lines.
  Still its own slice, for 7e's reason: a large mechanical diff is the worst
  possible neighbour for the data the audit exists to validate. It is the
  natural next item once rudhādi closes.
- **The other root-keyed pada sūtras** (1.3.38–43, 1.3.65, and the rest of
  the family 1.3.66 belongs to), until a curated root needs one. When one
  does, `UbhayapadaAnavane` is the shape to generalize, not a precedent to
  copy blindly — the next sūtra's condition may not be a sense restriction.
- **8.4.41's correspondence side** for `d`/`n`/`s`, still unwitnessed;
  **8.4.42/8.4.43/8.4.44**, still zero invocations.
- **6.3.111** and **6.1.68**, both deliberately absent, both documented.
- **It-stripping as real sūtras** (1.3.2, 1.3.3, 1.3.5, 1.3.9).

## Ordering

1. The data layer: variant, row, `padas()` arm, census rename, the
   consistency-test exception, `padas_maps_each_assignment_to_its_derivable_padas`.
2. The engine: `Tag::Anavane`, the `mod.rs` arm, rule 1.3.66, 1.3.78's
   widened decline, the per-rule guard tests.
3. The audit run, and only then the golden data: 8 `PARADIGM` blocks, the
   `ALTERNATES` rows, the two trace pins, the corpus totals.
4. The doc sweep.
5. The gates and the mutation campaign, with the floor re-measurement
   recorded in AGENTS.md.
