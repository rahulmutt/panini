# The pada audit — all 49 curated roots against their upadeśa

Two slices in a row deferred the same thing in nearly the same words. The
dhātupāṭha-number slice:

> **Auditing the 49 roots for mis-assigned pada.** √tud (`06.0001`) remains the
> known case. This slice makes it visible for the first time in the data: the
> vendored upadeśa `tu\da~^` carries the svarita `^` that 1.3.72 reads, sitting
> one column from a `PadaAssignment::Parasmaipada` that contradicts it.

And the ubhayapada slice:

> **√tud is a known-open correctness gap.** […] After this slice it is a
> *curation* choice, not an engine limit — the engine can now derive it
> correctly and chooses not to. Auditing all 48 roots for mis-assigned pada is
> its own slice, with its own vidyut audit.

This is that slice. It runs the audit, and the audit finds a second root.

## Scope

New: two `PadaAssignment` flips, eight golden `PARADIGM` blocks, and one test
that re-derives every root's pada from the vendored upadeśa.

Deleted: the `tudate` entry from the INVALID list, and the `Dhatu::pada` doc
comment's deferral paragraph.

No grammar changes, no new sūtras, no `Rule` added or reordered, no change to
`TINANTA_RULES` or its pinned order. The vikalpa set stays at seven. Root count
stays at **49**. `PARADIGM` goes from 200 root×lakāra blocks to **208**
(1800 → **1872** cells); `ALTERNATES` is expected to stay at 242 rows, for
2042 → **2114** forms.

Out of scope, deferred — all three carried forward unchanged:

- **It-stripping as real sūtras** (1.3.2, 1.3.3, 1.3.5, 1.3.9), so that `code`
  is *derived* from the upadeśa rather than curated. This slice adds a second
  test-private consumer of the same normalizer and deliberately does not
  promote it, for the reason the number slice gave: upadeśa preprocessing is
  not the tiṅanta pipeline `TINANTA_RULES` models, so it needs its own
  pipeline concept.
- **The eight remaining `~^r` rudhādi roots**, and 8.2.30 *coḥ kuḥ*'s
  generalisation past `j`.
- **√bhuj**, whose 1.3.66 *bhujo'navane* forks its pada on sense.

## What the audit found

### The method

For each of the 49 curated rows, read the **vendored** upadeśa from
`data/dhatupatha.tsv` and re-derive the pada from its it-markers alone:

1. an anudātta it or a ṅ it → **1.3.12** *anudāttaṅita ātmanepadam*;
2. otherwise a svarita it or a ñ it → **1.3.72** *svaritañitaḥ kartrabhiprāye
   kriyāphale*, i.e. ubhayapada, since 1.3.78 supplies the other arm;
3. otherwise → **1.3.78** *śeṣāt kartari parasmaipadam*.

The one piece of notation that has to be read correctly is **where an accent
attaches**. Upstream writes the accent *after* the `~` that marks an anunāsika
it, so `~\` and `~^` are an anudātta it and a svarita it — while a `\` sitting
directly on a vowel elsewhere in the upadeśa is the root's own accent and says
nothing about pada. Conflating the two is not a subtle error, it is a
catastrophic one: 35 of the 49 curated roots carry a `\` somewhere in their
upadeśa, so the naive reading calls 35 of them ātmanepada — including √bhū.

Marker identification otherwise reuses `strip_anubandhas`'s existing
semantics — 1.3.3 *halantyam* decided on the original upadeśa (so `RI\Y`'s
final `Y` is an it), and 1.3.5 *ādir ñiṭuḍavaḥ* for an initial `Yi`.

### √tud was known; √nī was not

Two rows disagree with their own upadeśa:

| number | root | gaṇa | curated | upadeśa | markers |
|---|---|---|---|---|---|
| `06.0001` | √tud | tudādi | Parasmaipada | `tu\da~^` | **Ubhayapada** — svarita it |
| `01.1049` | √nī | bhvādi | Parasmaipada | `RI\Y` | **Ubhayapada** — ñ it, by 1.3.3 |

Both are genuinely ubhayapadī: *tudati / tudate*, *nayati / nayate*.

√tud was flagged four times across two slices. **√nī was flagged nowhere.**
Every deferral naming this work named √tud alone, and one of them — "√tud
remains the known case" — asserted the singular. It sat in bhvādi, the first
and largest curated gaṇa, from the v1 slice onward, and eleven subsequent
slice since read past it.

That is the entire argument of the next section. An audit performed once finds
√nī; an audit that is a test finds the next one.

### The other 47 agree

Including the three roots whose markers are least obvious, each of which the
check has to get right for the run to mean anything:

- `02.0026 SIN` (√śī) and `09.0045 vfN` (√vṛṅ) carry no `~` at all. Their pada
  comes from a ṅ it identified by 1.3.3, and both are curated ātmanepada.
- `07.0011 YiinDI~\` (√indh) satisfies **both** 1.3.12 and 1.3.72, and is
  curated ātmanepada — see below.

## Why the check becomes a test, not an act

### Non-circularity

The test reads the vendored upadeśa and applies three sūtras to it. It consults
nothing this repo wrote about the root. That is the same property that makes
`dhatupatha_numbers_resolve_upstream` meaningful, and it is why the new test
belongs beside it, sharing its `upstream_rows` helper and its notion of an it.

An audit run by hand against vidyut-prakriya is *also* non-circular, but it is
not repeatable by anyone who has not set up a vidyut checkout. The
cross-implementation audit stays the authority on derived **forms**; this test
is the authority on the **pada column**, and it runs in `cargo test`.

### The 1.3.12-beats-1.3.72 arm is the load-bearing one

`07.0011 YiinDI~\` (√indh) carries a `Yi`, which 1.3.72 reads, *and* an
anudātta it, which 1.3.12 reads. The verdict must be ātmanepada, not
ubhayapada. Ordering the two clauses the other way makes √indh come out
ubhayapadī and grows it a parasmaipada column it must not have.

This is not a new finding — it is exactly what `Tag::Ubhayapadin`'s doc comment
in `term.rs` already records ("the tag is deliberately NOT named for 1.3.72's
condition […] It must never reach 1.3.72 at all"), verified against
vidyut-prakriya in the ubhayapada slice against a `~^r` control. What is new is
that the precedence now has a *second*, independent encoding: once in the
engine's tag, once in the audit's clause order. It gets its own named assertion
so that a reversal fails loudly rather than silently re-deriving the tag's own
opinion.

### Gaṇa gets the same treatment

Nothing today asserts that `Dhatu::gana` agrees with the dhātupāṭha number,
though the number's first two digits *are* the gaṇa (`06.0001` ⇒ tudādi). It is
the same class of hole as the pada column — a hand-copied verdict beside the
data that determines it — and closing it is two lines in the same test. Both
√aś rows make it non-trivial rather than decorative: `05.0020` and `09.0059`
share a `code` and are distinguished only by gaṇa.

## What changes downstream

### The data rows

Two `pada` fields flip to `PadaAssignment::Ubhayapada`. Nothing else in
`panini-data` changes.

### No engine change whatsoever

`PadaAssignment::Ubhayapada` already sets `Tag::Ubhayapadin`; 1.3.72 already
reads it and sanctions the ātmanepada cell; 1.3.78 already declines rather than
blocks when it is present; `padas()` already fans an ubhayapadī root out to two
cells. All of it landed with √rudh and none of it is root-specific. The two
flips are the whole mechanism.

That this is true is a claim the slice must *verify*, not assume — but if it
turns out false, the slice has grown a grammar change and needs re-scoping
rather than a patch.

### The golden tables

Eight new `PARADIGM` blocks — `01.1049` and `06.0001`, each × four lakāras,
each `Pada::Atmanepada`. Both roots' parasmaipada blocks and their existing
`ALTERNATES` rows are untouched, and **no existing form may change**.

`ALTERNATES` is expected to gain nothing. The 21 ātmanepada alternate rows in
the suite today are all rudhādi (`07.0001`, `07.0011`, `07.0012`) and all on
8.4.65 *jhayo ho'nyatarasyām*, which needs a jhay-plus-`D` shape neither of
these roots presents. No bhvādi or tudādi ātmanepada block forks today. Of the
other six optional rules, 7.1.35 replaces loṭ's `tu`/`hi` and so never reaches
an ātmanepada ending, 6.4.107 is śnu's, and 8.4.56 wants a pada-final jhal
where these endings are vowel-final.

This is a prediction, not a premise. The audit checks it, and a fork that shows
up is a finding to record, not a surprise to absorb quietly.

### `tudate` stops being INVALID

`crates/panini/tests/paradigm.rs` pins `tudate` as INVALID under a nine-line
comment that already anticipates this slice:

> `tudate` is a REAL Sanskrit form -- √tud is ubhayapadī […] It is INVALID here
> only because `Dhatu.pada` records a curated verdict […] Auditing the whole
> table for mis-assigned pada is its own slice; until then this entry pins the
> documented meaning of INVALID ("not derivable within the covered grammar"),
> not a claim about Sanskrit.

The entry and its comment are deleted, and `tudate` becomes a golden — it is
`06.0001` laṭ ātmanepada prathama eka. This is the one place where the slice
changes an existing test's verdict rather than adding to it, and it is a
correctness fix: the engine now derives a real Sanskrit form it previously
rejected.

Neighbouring INVALID entries stay. `manyati` and `vidyati` are wrong-pada
crosses on genuinely ātmanepada-only roots and remain wrong.

## Testing

### Goldens are transcribed from the audit, never hand-authored

**No golden form in this slice may be written by hand.** All 72 are transcribed
from the cross-implementation audit's own output for the two roots.

This is a constraint on the implementation plan, not a style note. Spec and
plan code blocks in this repo are transcribed verbatim into source, so a form
this document invents becomes a form the suite pins — and a hand-derived
paradigm that is 70/72 correct is worse than none, because the two wrong cells
arrive wearing the same authority as the rest. The spec therefore carries **no
appendix of expected forms**, which is a deliberate departure from the
ubhayapada slice's shape.

### The new guard test

`curated_pada_agrees_with_upadesha_markers`, in `panini-data`, covering all 49
rows, plus named assertions for:

- the 1.3.12-over-1.3.72 precedence on `07.0011` (√indh must be ātmanepada,
  not ubhayapada);
- the two ṅ-it-by-1.3.3 roots, `02.0026` and `09.0045`;
- accent placement — that a root-vowel `\` does not make `01.0001 BU`
  ātmanepada.

The last of these is the one that keeps the test from passing vacuously: a
mis-written accent check that flags everything would still agree with the
column on every ātmanepada root.

### Cross-implementation audit

The committed harness at `tools/audit/`, copied per its README and **not
rewritten**. Negative controls first, then a zero-difference run over all 1872
cells, with `01.1049` and `06.0001` split per pada via
`Tinanta::builder().pada(...)` as √rudh was. The harness resolves by dhātupāṭha
number, so both roots anchor without depending on any cell this slice pins.

The audit is also what supplies the 72 golden forms, so it runs *before* the
goldens are written, not after them as a confirmation.

### Mutation gate

The suite grows 1800 → 1872 cells, so the uncaught-run floor moves again.
AGENTS.md's guidance applies unchanged: **measure** the floor rather than
scaling it by cell count, run at `-j 4` with an explicit `--timeout` (2400,
per the ubhayapada slice's own campaign), and read `timeout.txt` alongside
`missed.txt`. The known-permanent `tripadi.rs` ṇatva-scan timeout is expected
and is not a survivor.

## The doc claims this slice falsifies

Four places assert a version of "√tud is the known case, and pada is a
deferral". All are now wrong, and one is wrong in the way that has bitten this
repo twice before (`c4b3907`; the ubhayapada slice's stated top risk): the
sentence stays *nearly* true, so it survives review.

- **`Dhatu::pada`'s doc comment** — "a curated verdict […] Reading real markers
  here would make 1.3.72 fire on every root whose markers satisfy it, and
  √tud's do […] A documented deferral in one field, lifted root-by-root as
  sūtras are implemented." The field stays curated and the first half stays
  true, but it is no longer a *deferral*: every verdict now agrees with the
  markers, and a test says so. Rewrite to cite the test, as the
  `dhatupatha` field's comment cites `dhatupatha_numbers_resolve_upstream`.
- **AGENTS.md** — the cell, form and block counts in the paradigm paragraph and
  the audit paragraph (1800 → 1872, 2042 → 2114); and √rudh as "the engine's
  first ubhayapadī root", which stays true as *first* but must stop reading as
  *only*.
- **`docs/ARCHITECTURE.md`** — less than it first appears, and the bullet is
  kept to say so. It carries no cell or form totals, and its pada-coordinate
  paragraph ("an ubhayapadī root contributes *two* `PARADIGM` blocks per
  lakāra […] 1.3.72 is deliberately absent from the vikalpa set") is exactly
  right and describes what these two roots now do. What needs re-reading is
  the rudhādi paragraph's "√rudh […] as the engine's **first** ubhayapadī
  root": true as written, but it is the only ubhayapada sentence in the file,
  so it currently carries the implication of *only*.
- **`2026-07-21-divadi-tudadi-ganas-design.md`** — "**√tud is ubhayapadī** in
  the Dhātupāṭha (svarita it). This slice takes only its **parasmaipada** pada
  […] the ātmanepada tudati-form is future work." Footnote it as discharged,
  the way slice 5f footnoted the roadmap's √śī row.

Old **plans** are historical records and are not rewritten. Specs get
footnotes; AGENTS.md and the source get corrections.

One claim this slice must be careful *not* to overstate: the audit covers the
**49 curated roots**, not the dhātupāṭha's 2259. A root curated in a future
slice can still arrive with a mis-assigned pada — the test catches it at that
point, which is the whole value, but nothing here makes the table
self-maintaining.

## Ordering

1. The guard test first, **failing** on the two known rows. It is the artifact
   that outlives the slice, and writing it first means the two flips are
   verified by something written before anyone knew which rows would flip.
2. Flip the two `pada` fields; the test goes green.
3. Run the cross-implementation audit for the two roots, negative controls
   first. Transcribe the 72 forms.
4. Add the eight `PARADIGM` blocks; delete the `tudate` INVALID entry.
5. Full-corpus audit at 1872 cells, zero differences.
6. Mutation gate, floor re-measured.
7. Doc corrections.
