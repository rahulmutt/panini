# ubhayapada — 1.3.72 *svaritañitaḥ*, and √rudh

Every gaṇa spec since gaṇa 1 has deferred the same thing. The wording barely
changes: "**Ubhayapadī roots and 1.3.72 *svaritañitaḥ***, unchanged from every
gaṇa spec so far." It is why rudhādi lacks its own eponym — √rudh is
`07.0001`, the root the gaṇa is named after — and why nine of that gaṇa's 25
roots are out of reach.

This slice pays it off. It adds one sūtra, one data-layer distinction, and one
root, and it is deliberately the smallest slice that can do so: √rudh needs
**no new phonology whatsoever**. Every form it produces is derivable by
machinery 7a and 7b already built, the moment 1.3.72 lets its ātmanepada arm
exist.

## Scope

New: one sūtra (1.3.72), one widened sūtra (1.3.78), a three-valued pada
assignment in the data layer, a pada column on the golden tables, and √rudh.
72 new cells, 101 new forms, 29 new alternates.

| id | dhātupāṭha | pada | laṭ prathama eka |
| --- | --- | --- | --- |
| `ruD` | 07.0001 `ru\Di~^r` *āvaraṇe* | **ubhayapada** | `ruRadDi` / `rundDe` |

Counts: `PARADIGM` 1728 → **1800** cells, `ALTERNATES` 213 → **242** rows,
1941 → **2042** forms, 48 → **49** roots. The vikalpa set stays at **seven**
rules; this slice adds no optional rule.

`ruD` collides with no existing SLP1 code, so the `aS.5` qualification
mechanism stays at exactly one user.

Out of scope, deferred:

- **The other eight `~^r` rudhādi roots** — `Bi\di~^r`, `Ci\di~^r`, `ri\ci~^r`,
  `vi\ci~^r`, `kzu\di~^r`, `yu\ji~^r`, `u~Cfdi~^r`, `u~tfdi~^r` (√bhid, √chid,
  √ric, √vic, √kṣud, √yuj, √chṛd, √tṛd). After this slice they are deferred by
  **curation**, not by missing machinery. That distinction is the single most important
  documentation change here; see "The doc claims this slice falsifies".
- **The nine reachable non-ubhayapadī rudhādi roots** (√śiṣ, √tṛh, √und, √añj,
  √tañc, √vij, √vṛj, √pṛc, √vid) and **√bhuj**, whose 1.3.66 *bhujo'navane*
  forks on sense rather than on a pada axis this engine models. Unchanged from
  7b.
- **Auditing the other 47 roots for mis-assigned pada.** At least one is known
  to be affected: the divādi/tudādi plan records that "√tud is taken
  parasmaipada-only (its ubhayapada svarita and 1.3.72 are deferred, per the
  spec)." Once 1.3.72 exists that is a curation decision rather than an engine
  limit, and it should be revisited — but as its own slice, with its own audit,
  not smuggled in here. See "Why the data layer stores a verdict".

## The pada model

### Pada is already a coordinate, not a property

The engine is better shaped for this than it looks. `Context.pada` is a
*requested* coordinate, and 1.3.12 / 1.3.78 in `samjna.rs` either sanction the
request or set `p.blocked`. Nothing in the pipeline assumes a root has one
pada; the assumption lives entirely in the data layer (`Dhatu.pada: Pada`,
"Ubhayapadi roots are out of scope; each curated root has exactly one pada")
and in the analyzer, which proposes only `d.pada`.

So "ubhayapadī" means exactly: a root that gets **sanctioned in both padas**
rather than blocked in one. No new pipeline concept is required.

### Why the data layer stores a verdict

`Dhatu.pada` keeps its name and changes type:

```rust
pub enum PadaAssignment {
    Parasmaipada,
    Atmanepada,
    Ubhayapada,
}
```

`Context.pada` stays the two-valued `Pada`. This split is load-bearing: the
data layer says what a root *admits*, the context says what is *being
derived*, and no derivation may request an "ubhayapada" cell, because no such
cell exists.

The rejected alternative was to store the upadeśa it-markers the sūtras
actually read (`anudatta_ngit`, `svarita_nit`) and let 1.3.12 and 1.3.72 each
guard on its own condition. That is more faithful, and every combination is
genuinely witnessed (√indh anudātta+ñi, √khid anudātta, √rudh svarita, √bhū
neither), so it is not an over-general model. It was rejected for one concrete
reason: **the moment the engine reads real markers, 1.3.72 fires on every root
whose markers say so**, and √tud's markers say so. Holding this slice's scope
would then require writing `svarita_nit: false` on √tud — a false statement in
the public data API, which is precisely what `context.rs` refuses when it
declines to derive `Default` for `Lakara` ("claiming one in the public data API
would be a lie"). Faithfulness that must be falsified in one row to hold scope
is not faithfulness.

A curated verdict is honest about what it is. The table is already a curation —
48 roots of thousands, stored post-it-stripping, with √hiṃs kept as `hins`
because 7.1.58 is not derivable, labelled in its own comment as "a stated
simplification". `PadaAssignment::Parasmaipada` on √tud with a comment naming
1.3.72 as the reason is the same species: a documented deferral that the next
slice lifts by editing one row.

The cost is real and is stated here rather than discovered later: **1.3.72 will
guard on a stored conclusion, so the data table — not the rule — is where
√rudh's ubhayapada-ness is asserted.** The cross-implementation audit is what
holds that assertion honest. This is the same trust boundary √hiṃs's `hins`
already sits on.

### The tag carries the residue, not the marker

`tinanta/mod.rs` gains one arm: `Ubhayapada` → `Tag::Ubhayapadin`, beside the
existing `Atmanepada` → `Tag::Atmanepadin`.

The tag is deliberately **not** named for 1.3.72's condition (`Svaritanit` or
similar). √indh is the counterexample: `YiinDI~\` carries a ñi, and 1.3.72
reads ñit, so a marker-named tag would have to be true on √indh — yet √indh
must never reach 1.3.72, because its anudātta settles pada by 1.3.12 and
vidyut-prakriya derives it ātmanepada-only. `Tag::Ubhayapadin` must be
documented as meaning *"1.3.72's condition holds **and** 1.3.12's does not"* —
the residue after 1.3.12, which is exactly why √indh does not carry it.

### The three sūtras

In `SAMJNA`, in numeric order, each reading only the aṅga's tags and
`ctx.pada`:

| sūtra | guard | ātmanepada requested | parasmaipada requested |
| --- | --- | --- | --- |
| 1.3.12 *anudāttaṅita ātmanepadam* | `has(Atmanepadin)` | record | **block** |
| 1.3.72 *svaritañitaḥ* | `has(Ubhayapadin)` | record | decline → 1.3.78 |
| 1.3.78 *śeṣāt kartari parasmaipadam* | `!has(Atmanepadin)` | **block**, unless `has(Ubhayapadin)` | record |

1.3.78 barely moves. Its guard is already `!p.terms[ANGA].has(Tag::Atmanepadin)`,
which *already* admits an ubhayapadī root; the entire edit is making its
ātmanepada arm decline instead of block when 1.3.72 has spoken. No new pipeline
state, no cross-rule communication, no ordering dependency.

### Order is not load-bearing, and that claim gets a test

Every pair of the three is disjoint. `Atmanepadin` and `Ubhayapadin` are
mutually exclusive on the root, so 1.3.12 and 1.3.72 can never both fire. Where
1.3.72 and 1.3.78 overlap — an ubhayapadī root is `!Atmanepadin` — they split on
`ctx.pada`: 1.3.72 records only on Ātmanepada, 1.3.78 only on Parasmaipada, and
1.3.78's ātmanepada arm declines rather than blocks in exactly the case 1.3.72
handles.

Commit `ee35a30` had to go back and qualify an order-independence claim that
prose had overstated. So this one is asserted by a test that runs the three
rules in permuted order and requires identical output — not by a comment.

### The semantics are not modelled, and that is stated

1.3.72's real condition is *kartrabhiprāye kriyāphale* — ātmanepada when the
fruit of the action accrues to the agent — and 1.3.78's *śeṣāt* is the
complementary residue. The engine models no semantics, so **both arms derive**,
each trace crediting the sūtra that sanctioned it, and the reader selects by
sense.

This is **not** a vikalpa fork, and conflating the two is the obvious wrong
turn. Pada is a context coordinate, so the two arms are two **cells**, not two
branches of one cell. They must not enter the `Vec<Prakriya>` fork machinery
that `docs/ARCHITECTURE.md` reserves for *anyatarasyām* / *vā* / *vibhāṣā*.
The vikalpa count stays at seven precisely because 1.3.72 is not one.

## √rudh costs the engine no new phonology

This is the reason √rudh alone is a sufficient witness.

### The ṇatva split is strong-vs-weak, not pada-vs-pada

7b's spec quoted √rudh in shorthand as "`ruRadDi` alongside `runDe`/`rundDe`",
which invites the reading that ṇatva tracks pada. It does not. The split is
**strong stem vs. weak stem**, and it falls straight out of the guard already
in `tripadi.rs`:

| forms | shape | ṇatva? | why |
| --- | --- | --- | --- |
| `ruRadDi`, `ruRatsi`, `ruRaDmi`, `aruRat`, `ruRaDAni`, `ruRaDE` | strong — śnam's `a` intact | **yes** | the `n` is followed by `a`, so it is a legal target; 8.4.2 fires with `u` intervening between the trigger `r` and the `n` |
| `runDanti`, `runDaH`, `rundDe`, `runDIta` | weak — `a` elided by 6.4.111 | **no** | the `n` is followed by a jhal, so `is_natva_target` declines (8.3.24 has already bled it) |

`runDanti` is the proof: it is *parasmaipada* and has no ṇ. `ruRaDE` is the
proof from the other side: it is *ātmanepada* and has one, because loṭ uttama
is strong.

Both halves are produced by machinery already present, unmodified. √rudh
becomes a **third independent witness for 8.4.2**, after kryādi's √vrī
(`vrIRAti`) and svādi's √ri (`ariRma`) — and, more usefully, the first root in
the suite where a live ṇatva trigger and the folded 8.3.24 guard coexist, so
that **one root both fires and declines ṇatva depending on stem strength**.
Every existing ṇatva witness fires in every cell that has a trigger; √rudh is
therefore the first direct regression test for `is_natva_target`'s jhal
condition rather than for the trigger scan.

### Everything else maps onto an existing root

- The **ātmanepada arm is structurally √indh's**: `rundDe` / `runDe` is
  `indDe` / `inDe` — 8.2.40 *jhaṣas tathor dho'dhaḥ* then 8.4.65's optional
  savarṇa elision. `runtse` / `runtsva` are `intse` / `intsva`; `ruRaDE`,
  `ruRaDAvahE`, `ruRaDAmahE` are `inaDE`, `inaDAvahE`, `inaDAmahE` with ṇatva
  added by the strong stem.
- The **strong parasmaipada arm is √bhañj's and √piṣ's**: the strong stem plus
  tripādī consonant sandhi. `ruRatsi` and `runtse` are cartva, 8.4.55 *khari
  ca*, turning the root's `D` into `t` before the `s` of si/se; `runDAd` /
  `runDAt` is 7.1.35 *tātaṅ* plus 8.4.56 *vā'vasāne*, both already optional.
- √rudh is also a **second witness for 8.2.74 / 8.2.75**, the *ru* alternation
  before sip that 7a built for √hiṃs: its laṅ parasmaipada madhyama eka is
  `aruRad` / `aruRat` / `aruRaH`, the same three-way shape as √hiṃs's
  `ahinad` / `ahinat` / `ahinaH`.

### A second six-form cell

√rudh's loṭ parasmaipada madhyama eka holds **six** forms — `rundDi`, `runDi`,
`rundDAd`, `runDAd`, `rundDAt`, `runDAt` — tying √kṛt's record, which
`docs/ARCHITECTURE.md` currently calls "the suite's deepest". Its loṭ
parasmaipada prathama eka holds five. Both arise the same way √kṛt's does, with
k = 3 (7.1.35, 8.4.65, 8.4.56) against a 2³ bound of eight.

`docs/ARCHITECTURE.md`'s "Optional rules and the derivation set" section names
√kṛt's cell as uniquely deepest. That sentence needs updating, not rewriting.

## What changes downstream

### The golden tables lose their implicit pada

`PARADIGM` is keyed `(root_id, lakara, [9 forms])` and `ALTERNATES`
`(root_id, lakara, cell, form, key)`. In both, pada is *inferred from the
root* — exactly the assumption 1.3.72 breaks. Both gain an explicit pada
column.

This is forced, not stylistic: the harness asserts `a.pada == d.pada`, and once
`d.pada` is a `PadaAssignment` those comparisons no longer typecheck. They also
get **stronger**. That assertion exists to stop a mis-transcribed row binding to
the wrong √aś — the two share `code == "aS"` and differ only in pada — so
comparing against the row's own declared pada pins the row's claim rather than
the root's. The √aś hole stays closed, and the same protection extends to
√rudh's two blocks.

The edit touches 192 existing `PARADIGM` block headers (48 roots × 4 lakāras)
and all 213 `ALTERNATES` rows. It must land as its **own behaviour-preserving
commit, before √rudh exists**; mixed with eight new blocks and 29 new
alternates, a transcription error in an existing row hides inside the noise. The
repo has the pattern (`2b7cb98`, "retire two latent proxies, byte-identically").

### One accessor carries the fan-out

`PadaAssignment::padas() -> &'static [Pada]`, yielding one entry or two,
**parasmaipada first**.

- `candidates()` gains an inner loop over it: 1728 → 1800 candidates.
- `roundtrip.rs` and `derivation_set_is_exactly_pinned` iterate it.
- The in-crate unit-test call sites that want "the root's own pada"
  (`anga.rs`, `vikarana.rs` ×2, `derivation_tests.rs`) take `padas()[0]`.
  Parasmaipada-first means no existing single-pada root's behaviour moves.

### `check` and the CLI need no signature change

`Analysis.pada` already exists and the CLI already prints it. √rudh returns
more analyses per surface, which the existing multi-analysis path handles. It
is a good stress case: `runDe` is ambiguous *within* ātmanepada as well — laṭ
prathama eka and laṭ uttama eka both produce it — on top of any pada ambiguity.

Adding a root can only ever *add* analyses to an existing surface, never remove
one, so the candidate fan-out carries no regression risk to existing forms.

## Testing

### Goldens

Eight new `PARADIGM` blocks (two padas × four lakāras) and 29 new `ALTERNATES`
rows. Index 0 of each cell is the derivation with **no optional rule applied**,
per the existing convention — so where 8.4.65 forks a cell, the *un*-elided
form is the golden and the elided one is the alternate (`rundDe` is pinned,
`runDe` is filed under `8.4.65`), matching how √indh's cells are already
recorded.

The 29 alternates decompose as 22 parasmaipada + 7 ātmanepada. This is
independently checkable: 101 forms − 72 cells = 29.

### Trace pins

The sharpest available test of the whole mechanism is one root, one cell, two
padas. √rudh laṭ prathama eka must credit **1.3.78** deriving `ruRadDi` and
**1.3.72** deriving `rundDe`, and **neither trace may contain the other
sūtra**. That single pair pins the entire three-rule table.

A second pin should cover the ṇatva claim directly: `runDanti`'s trace must
*not* contain 8.4.1 or 8.4.2, while `ruRadDi`'s must contain 8.4.2 and not
8.4.1.

### Guard tests

One per way the table can be got wrong:

- **√indh must not reach 1.3.72.** It is ñit, so if `Tag::Ubhayapadin` ever
  drifts from "residue after 1.3.12" toward "has a ñi marker", √indh silently
  grows a parasmaipada column. This is the regression that protects the data
  model choice, and it is the one this slice is most likely to break later.
- √bhū requested ātmanepada still blocks (1.3.78's ātmanepada arm).
- √khid requested parasmaipada still blocks (1.3.12).
- √rudh blocks in **neither** pada.
- The three pada sūtras, run in permuted order, produce identical output.

### The `padas()` ordering survivor

One mutation-survivor risk is designed against up front rather than discovered
in the gate. `padas()` returns `[Parasmaipada, Atmanepada]` for the ubhayapada
case, and **nothing else in this plan would catch a mutant that reverses that
order**: the paradigm and roundtrip harnesses iterate both entries, so order is
invisible to them, and the `padas()[0]` call sites only ever see single-pada
roots.

That is the exact shape of the three `Context::is_tip` survivors 7b found. The
fix is cheap — pin the order in a data-layer unit test, the way
`rudhadi_holds_exactly_the_slice_7b_roots` pins the root table — but it must be
a stated acceptance criterion or it will not get written.

### Cross-implementation audit

The 72 cells below were derived from vidyut-prakriya at commit
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` (2026-06-24), using its
`Tinanta::builder().pada(...)` API so that the two padas are split rather than
merged into one cell. Since the data layer stores a *verdict* rather than the
markers, this audit is what holds the verdict honest, and the commit is
recorded here for that reason.

The audit must also confirm the negative: vidyut derives √indh in ātmanepada
only, against √rudh as the `~^r` control that derives both.

### Mutation gate

Per `AGENTS.md`, `--timeout` must clear a full **uncaught** run at the
parallelism actually used. The suite grows ~4% (1728 → 1800 cells), so the
~380s uncaught floor measured at 1728 moves to roughly 395s. `-j 4
--timeout 1200` keeps the margin; `CARGO_MUTANTS_JOBS` must not be set higher
in the environment, since it can defeat an unqualified `-j`. `timeout.txt` gets
checked alongside `missed.txt` — a zero-survivor report that ignores it is
vacuous.

## The doc claims this slice falsifies

This is the top risk in the slice, and it is documentation, not code. Four
places currently assert that rudhādi is partial **and that only 1.3.72 can
change that**. After this slice all four are wrong in the same subtle way:
rudhādi is *still* partial (7 of 25), but the remaining eight `~^r` roots are
now deferred by curation, not by missing machinery.

- `README.md` — "Nine more rudhādi roots are ubhayapadī and deferred behind
  1.3.72 — which is why the gaṇa lacks its own eponym, √rudh". Both halves
  change: eight, and the gaṇa now *has* its eponym.
- `AGENTS.md` — the rudhādi paragraph making the same claim.
- `crates/panini-data/src/lib.rs` — the comment on
  `rudhadi_holds_exactly_the_slice_7b_roots`: "More roots would not change
  that; only 1.3.72 will." 1.3.72 now has, partly.
- `docs/superpowers/specs/2026-08-13-rudhadi-gana-7b-design.md` — a historical
  document; it should **not** be edited, but the new spec supersedes its
  deferral list and should say so.

Additionally, `docs/ARCHITECTURE.md`'s claim that √kṛt's six-form cell is
uniquely the deepest needs qualifying, and `crates/panini-data/src/lib.rs`'s
`Dhatu.pada` doc comment ("Ubhayapadi roots are out of scope; each curated root
has exactly one pada") is directly falsified.

`c4b3907` — "final-review wave — three false doc claims" — is what this section
exists to prevent a repeat of.

## Ordering

1. Data layer: `PadaAssignment`, `padas()`, the √rudh row, the ordering pin.
2. Term tagging: `Tag::Ubhayapadin`.
3. The three sūtras, with the permutation test.
4. Analyzer fan-out.
5. Harness refactor: pada column across 192 headers, behaviour-preserving, own
   commit.
6. √rudh goldens: 8 blocks, 29 alternates.
7. Trace pins and guard tests.
8. Documentation, per the section above.
9. Mutation gate.

Steps 1–4 are behaviour-preserving for all 48 existing roots; step 6 is the
first that changes any output.

## A drafting constraint on the implementation plan

Following 7b: the plan states conditions, witnesses and acceptance criteria,
and leaves the Rust to be written against them. Plan-authored code blocks get
transcribed verbatim into the tree, scaffolding and all, so the plan should not
carry pre-emptive plumbing.

## Appendix — the full √rudh paradigm

Derived from vidyut-prakriya at the commit recorded above. Forms within a cell
are that tool's sorted set; index 0 is pinned in "Goldens" above, **not** by
this ordering.

### Parasmaipada

| cell | laT | laN | loT | viDiliN |
| --- | --- | --- | --- | --- |
| prathama eka | `ruRadDi` | `aruRad` / `aruRat` | `ruRadDu` / `runDAd` / `runDAt` / `rundDAd` / `rundDAt` | `runDyAd` / `runDyAt` |
| prathama dvi | `runDaH` / `rundDaH` | `arunDAm` / `arundDAm` | `runDAm` / `rundDAm` | `runDyAtAm` |
| prathama bahu | `runDanti` | `arunDan` | `runDantu` | `runDyuH` |
| madhyama eka | `ruRatsi` | `aruRaH` / `aruRad` / `aruRat` | `runDAd` / `runDAt` / `runDi` / `rundDAd` / `rundDAt` / `rundDi` | `runDyAH` |
| madhyama dvi | `runDaH` / `rundDaH` | `arunDam` / `arundDam` | `runDam` / `rundDam` | `runDyAtam` |
| madhyama bahu | `runDa` / `rundDa` | `arunDa` / `arundDa` | `runDa` / `rundDa` | `runDyAta` |
| uttama eka | `ruRaDmi` | `aruRaDam` | `ruRaDAni` | `runDyAm` |
| uttama dvi | `runDvaH` | `arunDva` | `ruRaDAva` | `runDyAva` |
| uttama bahu | `runDmaH` | `arunDma` | `ruRaDAma` | `runDyAma` |

### Ātmanepada

| cell | laT | laN | loT | viDiliN |
| --- | --- | --- | --- | --- |
| prathama eka | `runDe` / `rundDe` | `arunDa` / `arundDa` | `runDAm` / `rundDAm` | `runDIta` |
| prathama dvi | `runDAte` | `arunDAtAm` | `runDAtAm` | `runDIyAtAm` |
| prathama bahu | `runDate` | `arunData` | `runDatAm` | `runDIran` |
| madhyama eka | `runtse` | `arunDAH` / `arundDAH` | `runtsva` | `runDITAH` |
| madhyama dvi | `runDATe` | `arunDATAm` | `runDATAm` | `runDIyATAm` |
| madhyama bahu | `runDve` / `rundDve` | `arunDvam` / `arundDvam` | `runDvam` / `rundDvam` | `runDIDvam` |
| uttama eka | `runDe` | `arunDi` | `ruRaDE` | `runDIya` |
| uttama dvi | `runDvahe` | `arunDvahi` | `ruRaDAvahE` | `runDIvahi` |
| uttama bahu | `runDmahe` | `arunDmahi` | `ruRaDAmahE` | `runDImahi` |

Note the two cross-cell ambiguities the analyzer will now report: `arunDa` is
both parasmaipada laṅ madhyama bahu and ātmanepada laṅ prathama eka, and
`runDe` is both ātmanepada laṭ prathama eka and ātmanepada laṭ uttama eka.
