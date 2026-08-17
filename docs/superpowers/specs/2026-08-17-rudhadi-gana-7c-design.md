# rudhādi (gaṇa 7), slice 7c — the four curation-only roots

Three documents say the same thing about four roots, in nearly the same words.

`docs/ARCHITECTURE.md`:

> **√bhid, √kṣud, √yuj and √tṛd** are curation-only: the engine derives all 72
> cells of each today, byte-identical to vidyut-prakriya.

`AGENTS.md`:

> **√bhid, √kṣud, √yuj and √tṛd are curation-only** — the engine already
> derives all 72 cells of each, byte-identical to vidyut.

And `rudhadi_rows_are_the_seven_curated_roots` in `panini-data`:

> √bhid, √kṣud, √yuj and √tṛd are a table row and an audit apiece (the engine
> already derives all 72 cells of each, byte-identical to vidyut-prakriya)

No spec, no plan, and no commit message records the run that established this.
The claim is repeated in three places and sourced in none. This slice curates
the four roots — and in doing so, finally runs the audit the claim asserts the
result of.

That ordering is the slice's spine: **the audit runs before a single golden is
pinned.** If the claim is true, the slice is four table rows, 32 golden blocks,
and a documentation sweep. If it is false for some root, the slice ships the
rest and replaces an unsourced claim with a sourced deferral. Both outcomes
leave the repo better than an unchecked sentence in three files.

## Scope

New: four `Dhatu` rows, 32 `PARADIGM` blocks, whatever `ALTERNATES` rows the
new cells fork into, two ṇatva trace tests, and one test pinning cross-pada
ambiguous surfaces.

Changed: `mise.toml`'s `mutants` task, whose `run` line has contradicted its own
comment for three slices; and the six documentation sites enumerated under "The
doc claims this slice falsifies", none of which survive the new counts intact.

No grammar changes. No new sūtra, no `Rule` added or reordered, no change to
`TINANTA_RULES` or its pinned order, no change to any guard. The vikalpa set
stays at seven. **`crates/panini-prakriya` is not touched at all** — a fact the
mutation section leans on.

Root count goes 49 → **53**. rudhādi goes 7 → **11** of its 25 dhātupāṭha
entries. `PARADIGM` goes 208 → **240** root×pada×lakāra blocks (46 single-pada
roots × 4, plus 7 ubhayapadī × 8), i.e. 1872 → **2160** cells.

Out of scope, deferred — carried forward unchanged:

- **8.2.30 *coḥ kuḥ*'s generalisation past the hardcoded `j` → `g`**, and with
  it √ric and √vic. This slice deliberately does not touch that rule, for the
  reason given under "√yuj is the load-bearing one" below.
- **6.1.73 *che ca* and 8.4.40 *stoḥ ścunā ścuḥ***, and with them √chid and
  √chṛd.
- **The nine reachable non-ubhayapadī rudhādi roots** — √śiṣ, √tṛh, √und, √añj,
  √tañc, √vij, √vṛj, √pṛc, √vid — each bringing machinery of its own.
- **√bhuj** (`07.0017`), whose 1.3.66 *bhujo'navane* forks its pada on sense
  rather than on an axis this engine models.
- **It-stripping as real sūtras** (1.3.2, 1.3.3, 1.3.5, 1.3.9), so that `code`
  and `pada` are derived from a stored upadeśa rather than curated. This slice
  adds four more curated rows and so makes the case slightly stronger without
  pre-empting the design; upadeśa preprocessing is still not the tiṅanta
  pipeline `TINANTA_RULES` models.

After this slice, **14 of rudhādi's 25 entries remain out**. The gaṇa's prose
should say that number plainly rather than let "partial" drift toward sounding
nearly finished.

## The four roots

| dhātupāṭha | upadeśa | `code` | artha |
|---|---|---|---|
| `07.0002` | `Bi\di~^r` | `Bid` | `vidAraRe` |
| `07.0006` | `kzu\di~^r` | `kzud` | `sampezaRe` |
| `07.0007` | `yu\ji~^r` | `yuj` | `yoge` |
| `07.0009` | `u~tfdi~^r` | `tfd` | `hiMsAnAdarayoH` |

### All four are ubhayapadī, and the column is derived rather than asserted

Each carries the `~^` svarita it that 1.3.72 *svaritañitaḥ kartrabhiprāye
kriyāphale* reads, and none carries a trailing `~\` anudātta it for 1.3.12
*anudāttaṅita ātmanepadam* to pre-empt it with — the distinction the pada audit
established when it separated √rudh (`ru\Di~^r`, ubhayapadī) from √indh
(`YiinDI~\`, ātmanepada-only despite its ñi). The `\` inside each entry is the
root vowel's own accent, not an it: the accent attaches *after* the `~` that
marks an it, which is the one piece of notation the pada audit had to get right.

So all four are `PadaAssignment::Ubhayapada`, and
`curated_pada_agrees_with_upadesha_markers` in `panini-data` re-derives all four
verdicts from the vendored upadeśa the moment the rows land. The column cannot
be wrong here without a test failing — which is precisely why the pada column,
the thing that was mis-assigned on two roots as recently as the last slice, is
*not* among this slice's risks.

`07.0009`'s leading `u~` is an it by 1.3.2 *upadeśe'j anunāsika it*. It is
neither anudātta nor ṅit, so it does not reach 1.3.12, and udit's own
consequence (7.2.56 *udito vā*, optional iṭ before ktvā) is not a tiṅanta rule
and so cannot touch these four lakāras. Leading its are already handled: √indh's
`YiinDI~\` strips to `inD`.

Note also that no `code` collides. `Bid`, `kzud`, `yuj` and `tfd` are each new
to the curated set, and under number keying a collision would not matter anyway
— the two √aś rows already share a `code` and are distinct by number.

### What they witness: nothing new, which is the point

All four are d-final or j-final and land squarely inside phonology that √rudh,
√kṛt and √bhañj already exercise. Expected laṭ prathama eka, both padas:

| root | parasmaipada | ātmanepada |
|---|---|---|
| √bhid | `Binatti` | `Bintte` |
| √kṣud | `kzuRatti` | `kzuntte` |
| √yuj | `yunakti` | `yuNkte` |
| √tṛd | `tfRatti` | `tfntte` |

These are the spec's *expectations*, not its pins. The plan transcribes the
engine's actual output after the audit certifies it, never the other way round.

Two of the four are worth naming beyond mere coverage. Both are ṇatva
witnesses, and they fall on *different* arms of it — which is worth stating
precisely, because it is easy to get backwards.

**√kṣud takes 8.4.2 *aṭkupvāṅnumvyavāye'pi***, the non-adjacent ṇatva: in
`kzuRatti` the trigger is the `z` of `kz`, the target is śnam's `n`, and the
root's own aṭ vowel `u` sits between them. That is √rudh's shape (`ruRadDi`,
r-u-n) reached through a **ṣ trigger rather than an r trigger**. Inside rudhādi
— the one gaṇa where 8.3.24 *naścāpadāntasya jhali* is live and bleeds ṇatva off
the weak stem — √rudh is currently the only witness to that arm; 8.4.2's other
curated witnesses (`vrIRAti`, `muzARa`) are kryādi, where 8.3.24 never competes.
√kṣud is therefore the second root to show the strong/weak ṇatva split under
8.3.24 competition, and the first to show it with a sibilant trigger.

**√tṛd takes 8.4.1 *raṣābhyāṁ no ṇaḥ*** instead, the adjacent case: `tfRatti`'s
trigger `f` sits directly against the `n`, with nothing intervening. It is
structurally √kṛt (`kfRatti`), and it leans on `is_natva_trigger`'s `f | F`
arm — the r-vowels counting as triggers by 1.1.51 *uraṇ raparaḥ*, which
`natva_trigger_is_ra_sha_and_the_r_vowels` notes exists for kryādi's √vṛ.

So the pair covers both ṇatva arms rather than doubling up on one, which is the
opposite of what a quick reading of "both have ṇ in the strong stem" suggests.

### √yuj is the load-bearing one

√yuj is j-final, so its strong stem reaches **8.2.30 *coḥ kuḥ*** — the rule
whose match reads a literal `j` and whose substitute is a literal `'g'`, while
its own comment claims the 1.1.50 *sthāne'ntaratamaḥ* nearest-velar
substitution it does not implement. `yunagti` → 8.4.55 *khari ca* → `yunakti`.

That rule is exactly what the **next** slice must generalise, in both its match
and its substitute, to reach √ric and √vic. Landing √yuj's 72 cells now means
the generalisation slice inherits a second independent root pinning the existing
`j` behaviour, rather than having to construct that anchor as part of the change
it is trying to validate. √bhañj has been the rule's only witness since 7b; a
generalisation validated against one witness is a generalisation validated
against the shape of one witness.

This is also why 8.2.30 is explicitly out of scope here. Mixing the fix into the
slice that establishes the anchor would defeat the anchor.

## What changes

### The data rows

Four `Dhatu` entries in `crates/panini-data/src/lib.rs`, following the file's
existing convention: a comment naming the dhātupāṭha entry, its upadeśa, and
what the root brings.

The convention needs one honest adaptation. Every curated rudhādi comment to
date justifies its root by the sūtra it witnesses — √bhañj "witnesses 8.2.30",
√piṣ "witnesses 8.4.41", √indh "witnesses 8.2.40". These four witness nothing
new, and their comments should say so rather than manufacture a justification.
√kṣud notes 8.4.2 under a sibilant trigger, √tṛd notes that it mirrors √kṛt at
8.4.1, √yuj notes its role as 8.2.30's regression anchor, and √bhid notes that
it is the plainest of the four, reaching no rule the gaṇa had not already
reached. Coverage is a sufficient reason for a root to exist, and the comments
should be readable as such.

`rudhadi_rows_are_the_seven_curated_roots` is renamed to
`rudhadi_rows_are_the_eleven_curated_roots`, its expected vector grows by four,
and its long explanatory comment is rewritten around the remaining 14.

### The golden tables

32 new `PARADIGM` blocks (4 roots × 4 lakāras × 2 padas), 288 new cells.
`ALTERNATES` grows by however many forks the new cells produce.

**This spec deliberately does not pin the `ALTERNATES` row count, or therefore
the form total.** √rudh's loṭ madhyama eka forks six ways, but it does so
through 8.2.40 *jhaṣas tathor dho'dhaḥ* plus 8.4.65's optional elision, and
8.2.40 requires a jhaṣ-class (voiced aspirate) final. All four of these roots
are jaś-final or j-final, not jhaṣ-final, so √rudh's fork structure does not
transfer and guessing the count would be guessing. The plan measures it from the
engine and then asserts it; `derivation_set_shape_matches_the_audited_numbers`
in `crates/panini/tests/paradigm.rs` is where the measured numbers become pins.

The existing structural tests need no change to do their work: every
`ALTERNATES` row must still name a real `PARADIGM` block with a matching pada,
and every block's derivation set must still be exactly what `PARADIGM` plus
`ALTERNATES` say.

### Trace pins for the ṇatva arms

Goldens pin surfaces; they do not pin *how* a surface was reached. For these
four roots the two facts most easily wrong are also invisible to a golden:
whether √kṣud's ṇatva goes through 8.4.2 and √tṛd's through 8.4.1, and whether
the weak stems decline ṇatva at all because 8.3.24 got there first.

This is not hypothetical. Drafting this spec, the two roots were first grouped
together as 8.4.2 witnesses, on the reasoning that both show ṇ in the strong
stem — and √tṛd is an 8.4.1 root, because its `f` sits adjacent to the `n` with
no intervener. A golden would not have caught that; `tfRatti` is the right
string either way.

So **two tests** in `crates/panini/tests/trace.rs`, one per root, each modelled
on `rudh_natva_follows_stem_strength_not_pada` — which is itself one test
asserting over three traces. Each asserts:

- the parasmaipada strong stem takes its own arm and not the other one:
  `kzuRatti` contains 8.4.2 and not 8.4.1 (intervening `u`, sibilant trigger);
  `tfRatti` contains 8.4.1 and not 8.4.2 (adjacent `f` trigger);
- the weak stem contains neither, 8.3.24 having turned the nasal into an
  anusvāra before ṇatva looks;
- an ātmanepada strong-stem cell, reached through 1.3.72, still retroflexes —
  so the test distinguishes "ṇatva follows stem strength" from "ṇatva follows
  pada", which is the reason √rudh's test includes `ruRaDE` alongside
  `ruRadDi`.

√bhid and √yuj need no trace pin: neither reaches a rule whose *choice* is in
question. √yuj's 8.2.30 path is pinned by its goldens, which is what the next
slice needs from it.

### The cross-pada ambiguity test

`README.md` currently enumerates the surfaces that are genuinely pada-ambiguous
— `runDAm`, plus √nī's and √tud's six (`anayata`, `nayatAm`, `nayeta`,
`atudata`, `tudatAm`, `tudeta`). That enumeration is a derived fact, stated by
hand, with nothing verifying it. `roundtrip.rs` cannot serve: it asserts only
that *some* analysis recovers the input, so it is blind to how many analyses
there are.

Four new ubhayapadī roots — going from three such roots to seven — will grow
that set, and today it would grow silently while the README kept its old list.
This is the same drift the pada audit closed when it added
`curated_pada_agrees_with_upadesha_markers` so the pada column could not
diverge from the upadeśa that determines it.

So: one test in `paradigm.rs` that walks `PARADIGM`, collects every surface
appearing in both a `Pada::Parasmaipada` and a `Pada::Atmanepada` block, and
asserts the resulting set exactly. The README then quotes a list something
checks. The set is measured by the plan, not guessed here.

### `mise.toml`'s mutants task

The task ships `--timeout 1200` under a comment, four lines above, reading
"Keep -j <= 4, and pass `--timeout 2400` explicitly rather than trusting the
1200 default below". Three slices have now worked around a task that
contradicts itself by invoking `cargo mutants` by hand.

At 2160 cells the 1200s cap is no longer merely awkward — see the numbers under
"Mutation gate". The `run` line becomes `--timeout 2400` and the comment drops
the workaround it no longer needs to describe. This is not new reasoning; it is
the reasoning already written down, applied to the executable.

## Testing

### The cross-implementation audit runs first, and blocks

Add the four `Dhatu` rows and nothing else — no goldens yet. Then run
`tools/audit/panini_full_audit.rs` per `tools/audit/README.md`:

- Clone `vidyut` at the commit recorded in `data/dhatupatha.tsv`'s own header
  (`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` at the time of writing — **read
  the header, do not trust this spec or the harness README**, either of which
  can go stale against a re-vendoring).
- Add `panini` and `panini-data` as path dev-dependencies of `vidyut-prakriya`,
  copy the harness into its `examples/`, and run it under an explicitly named
  toolchain (`mise exec rust@1.97.1 --`), since the vidyut checkout has no
  `mise.toml`.
- Bump the harness's pinned totals at `panini_full_audit.rs:577-579` and its
  three doc-comment sites (lines 12, 24, 54).

The harness's existing commitments carry unchanged and must not be weakened:
entry selection by dhātupāṭha number alone with no fallback; comparison of
derivation **sets**, never index 0; blocked prakriyās filtered on this engine's
side.

**Run the `entry` negative control before recording any clean result.** A
zero-difference run proves nothing until the harness has been shown able to
fail, and this slice's whole purpose is to stop trusting an unverified clean
claim — recording a new one without a control would reproduce the exact fault
it is fixing.

### Goldens are transcribed from the engine, never hand-authored

Only once the audit is clean for a root do its eight blocks get written, from
the engine's own output that the audit has just certified against vidyut. 288
cells is well past the volume at which hand-transcription is reliable, and this
repo's plans are executed verbatim, so the plan must generate the blocks
mechanically rather than spell them out as prose to be retyped.

### Mutation gate

Run at `-j 4 --timeout 2400`, and check `timeout.txt` alongside `missed.txt`.

**The mutant set is unchanged.** Mutants are generated from
`crates/panini-prakriya`, which this slice does not touch, so the campaign
enumerates the pada audit's same 522. What is under test is not the mutants but
the *suite at 15% more cells*: whether any mutant now crosses the cap and is
reclassified from MISSED to TIMEOUT, which is the vacuity failure 7a hit at 1620
cells and 7b hit through `-j 16` contention. A zero-survivor run that was never
checked against `timeout.txt` is not a clean run.

Before the campaign, one uncontended `mise run test` to measure the 2160-cell
floor. Extrapolating the pada audit's measurements at 1872 cells — uncontended
uncaught floor ~443s, worst caught mutant 754.6s at `-j 4`, i.e. ~1.7×
contention — gives a floor near 510s and a worst case near 870s at 2160 cells.
That leaves 2400 with roughly 2.8× margin and would leave 1200 with roughly
1.4×.

**These are priors to be checked, not predictions to be trusted.** The 1872-cell
floor came in slightly *below* the 1800-cell figure, so cell count is
demonstrably not a reliable multiplier for this suite. Measure the floor; record
both it and the campaign's outcome distribution in AGENTS.md's cargo-mutants
paragraph, which is the single source for this reasoning.

Two disciplines are non-negotiable, both because they have already failed here
once: keep `-j` at or below 4 (`cargo mutants` also reads it from
`CARGO_MUTANTS_JOBS`, so an unqualified cap can be defeated by the environment
alone), and treat the known-permanent `tripadi.rs` timeout — the ṇatva backward
scan whose `j -= 1` mutates to `j /= 1` and never terminates — as the correct
verdict it is, not as a survivor to be chased with a bigger cap.

## The doc claims this slice falsifies

Every site below carries a number or a root list this slice invalidates:

| File | What goes stale |
|---|---|
| `README.md` | "seven of its roots … of 25"; the four roots' "purely for want of curation" clause; 1872 cells / 49 roots / 170 multi-form cells / 2114 forms; the list of roots deriving both padas; the ambiguous-surface enumeration |
| `AGENTS.md` | the 1872-cell golden-suite line (~135); the rudhādi deferral paragraph (~209); the recorded audit result (~324); the cargo-mutants floor and campaign numbers |
| `docs/ARCHITECTURE.md` | the rudhādi paragraph (~83–110): "seven roots", the curation-only sentence, and the "what is left" tally |
| `crates/panini-data/src/lib.rs` | `rudhadi_rows_are_the_seven_curated_roots` — name, expected vector, and explanatory comment |
| `tools/audit/panini_full_audit.rs` | pinned totals (577–579) and doc comments (12, 24, 54) |
| `tools/audit/README.md` | "Last recorded result" |

The three curation-only claims quoted at the top of this spec do not merely get
their numbers bumped. They are replaced by a statement of what was actually
run — this slice's audit, at a named vidyut commit, with a named negative
control — so the next reader inherits a sourced fact instead of a repeated
assertion.

## If the premise turns out false

"All four derive byte-identically" is a claim this slice **tests**, not one it
assumes. If the audit disagrees for a root:

1. That root drops from the slice. A root needing grammar is by definition
   outside a curation-only slice's contract.
2. The remaining roots ship, with their goldens, their audit, and their gate.
3. The falsified prose at all three sites is replaced by the specific sūtra or
   guard the root actually needs — an unsourced claim becomes a sourced
   deferral, which is a strictly better state than the one this slice started
   from.

So the slice's counts are written as "four roots, or fewer if the audit says
so": 53 roots and 2160 cells are what a clean audit yields, and the plan
resolves the actual figures at the audit step before any golden is pinned.
Shipping three roots here is a success, not a partial failure.

## Ordering

1. Four `Dhatu` rows; `curated_pada_agrees_with_upadesha_markers` confirms the
   pada column derives from the upadeśa.
2. Cross-implementation audit, with a negative control. **Blocking gate**: any
   divergence resolves per the section above before step 3 begins.
3. Goldens generated from the engine — 32 `PARADIGM` blocks and the
   `ALTERNATES` rows they fork into; measured totals become pins.
4. The two ṇatva trace tests, and the cross-pada ambiguity test with its
   measured set.
5. Floor measurement, then the mutation campaign at `-j 4 --timeout 2400`;
   `mise.toml`'s `run` line updated.
6. Documentation sweep across the six sites above, including the recorded audit
   result and the measured gate numbers.
