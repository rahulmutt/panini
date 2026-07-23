# Adādi gaṇa — Slice 5b (vidhiliṅ ungate)

**Status:** Design, approved in brainstorming 2026-07-23.

Builds on `2026-07-22-adadi-gana-design.md` (the slice-5 design) and its
realized aluk core, `2026-07-22-adadi-gana-5a-aluk-core.md` (slice 5a plan).
Also depends on `2026-07-20-vidhilin-lakara-design.md` (slice 2), whose yāsuṭ
chain this slice extends.

## Summary

Slice 5a landed adādi's *aluk* mechanism (2.4.72 *adiprabhṛtibhyaḥ śapaḥ*) for
the two ā-final parasmaipadī roots √yā and √vā across laṭ, laṅ, and loṭ, but
**deliberately gated adādi × vidhiliṅ** behind an honest-decline branch in
`derive` — the athematic optative junction was left for 5b. This slice removes
that gate.

The full adādi spec bundles four independent pieces under "5b": the athematic
optative (this slice), the consonant-final junction sandhi (√ad + cartva), the
voiced junction (√vas + jaśtva), and the ātmanepada side (√ās, √śī). This
sub-slice takes **only the first** — the optative — because it is the one piece
that closes the standing scope gate, needs **no new phonology**, and makes
adādi's lakāra coverage uniform (all four lakāras for its two ā-final roots)
before the root set widens. Every later root then gets all four lakāras in one
golden block instead of needing a second visit.

## Scope

Unchanged from 5a: adādi (gaṇa 2), parasmaipada, the two ā-final roots √yā
(*prāpaṇe*) and √vā (*gati-gandhanayoḥ*), all nine *puruṣa* × *vacana* cells.

New in this slice: the fourth lakāra, **vidhiliṅ**, for those two roots.

Out of scope, deferred to the remaining 5b pieces: the consonant-final /
voiced junction roots (√ad + 8.4.55 cartva, √vas + 8.2.39 jaśtva), the
ātmanepada side (√ās, √śī + 7.4.21 / 7.1.6), and everything the parent spec
already defers (other gaṇas, ubhayapadī roots, āśīrliṅ, …). This slice adds no
new root, no new gaṇa, and no internal junction sandhi.

Coverage: golden paradigm **918 → 936** forms (2 roots × 9 vidhiliṅ cells = 18
new). Root count unchanged at 26.

## The two defects (and why only two)

Slice 2's yāsuṭ chain already derives **7 of the 9** adādi vidhiliṅ cells
correctly once the gate is lifted — verified by running the ungated pipeline:

| cell | ungated output | status |
| --- | --- | --- |
| 3sg | `yāyāt` | correct |
| 3du | `yāyātām` | correct |
| **3pl** | `yāyāuḥ` | **wrong** → yāyuḥ |
| 2sg | `yāyāḥ` | correct |
| 2du | `yāyātam` | correct |
| 2pl | `yāyāta` | correct |
| **1sg** | `yāyāam` | **wrong** → yāyām |
| 1du | `yāyāva` | correct |
| 1pl | `yāyāma` | correct |

(√vā is identical, one letter over.) The correct paradigm parallels √as
exactly — *syāt, syātām, syuḥ, syāḥ, syātam, syāta, syām, syāva, syāma*.

Both wrong cells are the **same junction**: after 7.2.79 *liṅaḥ
salopo'nantyasya* strips the `s` of yāsuṭ, its `ā` sits directly against a
vowel-initial ending — `yA+am` (1sg), `yA+us` (3pl, the ending being `us` from
3.4.108 *jher jus*). In the thematic gaṇas this junction never surfaces:
**7.2.80 *ato yeyaḥ*** (guarded on the śap's final `a`) rewrites `yA` → `iy`
first, so the ending becomes `iyam` / `iyus` and the pre-junction vowel is `i`,
not `ā`. With śap luk'd by 2.4.72, 7.2.80 correctly declines (its `SHAP ends
with 'a'` guard fails on the empty śap), and the raw `ā`+vowel junction is
exposed for the first time.

## The two rule changes

Model **(A)** from brainstorming: text-shape rules on the fused ending term.
Yāsuṭ is a text prefix on the ending (slice 2's decision — not a separate
`Term`, to keep the ANGA/SHAP/ENDING indices fixed), so the junction is
internal to the ending term and both rules key on its text shape, not on a
morpheme boundary.

### New: 6.1.96 *usyapadāntāt* — the 3pl `us` junction

An `a`/`ā` immediately before the ending `us` is elided: `yAus` → `yus`, so
√yā + `yus` → **yāyuḥ**.

- **Guard:** the ending ends in `us` preceded by `a` or `A`.
- **Reference:** confirmed against the reference implementation
  (vidyut-prakriya `ac_sandhi.rs`, rule `6.1.96`, guard
  `x.has_antya('a') || x.has_antya('A')` + `y.has_text("us")`, cited example
  *bhindyuḥ*). The exact SLP1 **name and its ashtadhyayi.com id/text are
  verified before the literal is baked into `record()`** (Task 1), per
  AGENTS.md — the same reference-first discipline slice 2 used for its six new
  sūtras.

### Changed: third arm on 6.1.101 *akaḥ savarṇe dīrghaḥ* — the 1sg `am` junction

6.1.101 already carries two arms (the adādi aṅga-final ā + a/ā ending arm from
5a, and the bhvādi śap-`a` + ending-`ā` arm). This slice adds a third: the
yāsuṭ-internal `Aa` coalesces to a single `A` (`yAam` → `yAm`), so √yā + `yAm`
→ **yāyām**.

- **Guard:** `VidhiLin && SHAP is empty && ending starts_with "yA" && the char
  after that `yA` is `a` or `A``. Only the uttama-eka ending `yAam` matches:
  every other ā-retaining cell (`yAt`, `yAs`, `yAtAm`, …) has a **consonant**
  after `yA`, and the thematic gaṇas never reach this arm because 7.2.80 has
  already turned `yA` into `iy` (so `starts_with "yA"` is false) — and the
  `SHAP is empty` conjunct makes it adādi-only regardless.

## Ordering

Both rules run **after 7.2.80**. That single placement is what makes them
provably inert outside adādi vidhiliṅ:

- By the time 7.2.80 has run, every thematic liṅ ending is `iy…`-shaped, so
  neither `starts_with "yA"` (for 6.1.101's new arm) nor `a/A` before `us` (for
  6.1.96 — the thematic 3pl is `iyus`, pre-`us` char `y`) can match.
- 6.1.101 already sits after 7.2.80 in `TINANTA_RULES`; its new arm needs no
  repositioning.
- 6.1.96 is a new entry, placed in the 6.1 sandhi band (its position relative
  to 6.1.101 is free — the two target disjoint shapes, `…us` vs `…Aa…`).

**Non-adādi invariant, verified not merely asserted.** All 918 pre-existing
golden forms and their traces must be byte-identical. Confirmed by inspection
of the golden set: every existing `-yuḥ` form is thematic (`BaveyuH`,
`tudeyuH`, …), whose pre-`us` char is `y`, so 6.1.96 declines; and no existing
laṅ (or other) 3pl golden ends in an `a/A`+`us` shape. Implementation re-runs
the full 918 before adding the 18 new rows, so any trace drift fails loudly.

## Deleting the gate

The slice-5a scope gate is not grammar — it is an "unimplemented, decline
honestly" branch, called out as the one adjudicated `TINANTA_RULES` exception
in AGENTS.md. This slice retires it:

- **`crates/panini-prakriya/src/tinanta.rs`:** delete the
  `matches!(dhatu.gana, Gana::Adadi) && matches!(lakara, Lakara::VidhiLin)`
  block (the `p.blocked = true; return p;` branch) and its SLICE 5a SCOPE
  BOUNDARY comment. The `derive` function reverts to a plain
  `run_pipeline(&mut p, TINANTA_RULES)` tail with no gana×lakāra branch.
- **`crates/panini/tests/paradigm.rs`:** empty `GATED` (to `&[]`) and update its
  machine-guard block. The `GATED.len() <= 2` and per-entry `viDiliN` / adādi
  assertions become vacuous over an empty slice; keep the
  `unpinned == gated` and `PARADIGM.len() + GATED.len() == roots × lakāras`
  assertions, which now demand that **every** enumerable adādi cell (including
  the four vidhiliṅ) is pinned in `PARADIGM`. Simplify or remove the
  now-obsolete GATED-shape guards rather than leave dead scaffolding.
- **`crates/panini/tests/roundtrip.rs`:** the `if p.blocked { … }` arm becomes
  dead for adādi (nothing is blocked any more). Remove the adādi-specific
  wording; if no lakāra is ever blocked, the branch can go entirely.

## Negatives become the regression test

Slice 5a pinned four strings as permanent negatives with a comment saying 5b
would keep them and add the real forms as goldens:
`yAyAuH`, `yAyAam`, `vAyAuH`, `vAyAam`. They are exactly the non-words the
ungated pipeline emitted **before** these two rules. They **stay INVALID** —
now as the direct regression test that the reduction actually ran (the real
forms `yāyuḥ` / `yāyām` displaced them), not that a gate swallowed them. Update
their comment to say so; do not remove them and do not touch the other 5a
adādi negatives (`yAyati`, `yAte`, `vAte`, `yAati`, `yA`, `vA`).

## Golden and trace additions

- **Golden paradigm** (`crates/panini/tests/paradigm.rs`): add the two
  `("yA", "viDiliN", […])` and `("vA", "viDiliN", […])` blocks with the 9-cell
  paradigms above (`yāyāt, yāyātām, yāyuḥ, yāyāḥ, yāyātam, yāyāta, yāyām,
  yāyāva, yāyāma` and the √vā parallels), verified against ashtadhyayi.com when
  written. Total 918 → 936.
- **Ordered trace** (`crates/panini/tests/trace.rs`): pin the two newly-correct
  paths — **yāyām** (…7.2.79 → 6.1.101 third arm) and **yāyuḥ** (3.4.108 jus →
  … 7.2.79 → 6.1.96). These are the two cells the rules fix; the other seven
  already ride the slice-2 chain and need no new trace. Sūtra ids/names checked
  against ashtadhyayi.com.

## Everything else: no change by construction

- **`panini-data`, `panini-analyze`, `panini` facade, `panini-cli`,
  `panini-lipi`:** untouched. The lakāra already flows through all plumbing
  (adādi vidhiliṅ cells were always *enumerated* — only *blocked* at derive).
  No new phoneme (`yāyuḥ` / `yāyām` are already-supported segments), so the
  lipi roundtrip/property tests are unaffected.
- No new `Term` state, no `Context` coordinate, no index-layout change.
  `TINANTA_RULES` gains exactly one entry (6.1.96) and one existing rule
  (6.1.101) gains one arm.

## Testing

Follows slices 2 and 5a.

- **Golden:** the 18 new forms VALID with correct ordered traces; the 918
  pre-existing forms byte-identical (run before adding the new block).
- **Negatives:** the four converted regressions stay INVALID; the full 5a
  adādi negative set is unchanged.
- **Trace:** the two pinned paths (yāyām, yāyuḥ) match exactly, ids/names per
  ashtadhyayi.com.
- **Coverage guard:** `paradigm_covers_every_enumerable_cell` passes with an
  empty `GATED` — i.e. no enumerable cell is unpinned any more.
- **Mutation testing:** `mise run mutants` at slice end. The new-arm guards
  (6.1.96's `a/A`-before-`us`, 6.1.101's `yA`+vowel third arm) must not
  survive. The 1sg golden (`yāyām`) kills a mutated 6.1.101 arm, the 3pl golden
  (`yāyuḥ`) kills a mutated 6.1.96 guard, and the four retained negatives kill
  a reduction that fails to run.

## Success criteria

- All 936 golden forms (26 roots, adādi now with all four lakāras for √yā/√vā)
  validate VALID with correct ordered traces; the 918 pre-existing forms are
  byte-identical.
- adādi × vidhiliṅ is no longer gated; `derive` has no gana×lakāra branch and
  `GATED` is empty.
- The four converted non-words stay INVALID.
- Every sūtra id and name in a trace matches ashtadhyayi.com (6.1.96 verified).
- Mutation testing shows the two new guards are pinned by tests.
- AGENTS.md's "one adjudicated exception" note and the slice-5b comments in
  tinanta.rs / paradigm.rs / roundtrip.rs are updated to reflect that adādi now
  has uniform four-lakāra coverage for its ā-final roots and the gate is gone.
