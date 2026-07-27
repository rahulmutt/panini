# The kṅiti-ca guard fix and the tinanta.rs split (Phase 2, debt slice)

**Status:** Design, approved in brainstorming 2026-07-27.

Builds on `2026-07-25-adadi-si-5f-design.md` (slice 5f, which closed adādi and
left both of these debts explicitly flagged). Adds no grammar coverage: no new
root, no new lakāra, no new gaṇa, no new golden form.

## Summary

Adādi closed with slice 5f — 30 roots, 1080 golden forms, four gaṇas × four
lakāras × both padas. Before the codebase grows along a new axis, this slice
pays down the two debts that slice left behind:

1. **The 1.1.5 *kṅiti ca* guard is inoperative on the śap-luk'd path.**
   7.3.84 and 7.3.86 both test `p.terms[SHAP].has(Tag::Ngit)`, a fixed index
   that holds the vikaraṇa. For adādi the vikaraṇa is luk'd — present but
   empty — and the ṅit tag rides on the ending instead, so the block never
   fires there. Replace the duplicated inline condition with one helper that
   asks which term actually follows the aṅga.

2. **`tinanta.rs` is 4058 lines.** Three consecutive specs flagged it as
   growing and none fixed it. Split it into a `tinanta/` module of nine files
   along the pipeline's own stage boundaries, with each stage's guard tests
   co-located with its rules.

The two are taken together, in that order, on one branch: the guard fix is a
semantic change small enough to review against current history, and landing it
first means the mutation gate runs once over the final shape rather than twice.

## Scope

**In scope:** the 1.1.5 guard at 7.3.84 and 7.3.86; the `tinanta.rs` → `tinanta/`
split; the `TINANTA_RULES` type change and `run_pipeline`'s signature; the
doc pointers that name `tinanta.rs` by path.

**Out of scope:** any grammar coverage; the deferred adādi irregulars (√as,
√han, √brū, √i); the next gaṇa or lakāra axis; rewriting the three existing
athematic arms (6.1.78, 6.1.90, 6.1.66) onto the new helper — see
"Approaches considered" below for why.

**Expected behavioural delta: none.** All 1080 golden forms and every trace
line must be byte-identical to `main` at the end of this slice.

## Debt 1 — the kṅiti ca guard

### The defect

1.1.5 *kṅiti ca* blocks guṇa when the item **immediately following** the aṅga
is ṅit (or kit; the engine has no kit tag today — see "Deliberate narrowness"
below). The engine encodes "immediately following" as the fixed index `SHAP`:

```rust
if p.terms.len() > SHAP && p.terms[SHAP].has(Tag::Ngit) {
    return false;
}
```

That is correct only while a vikaraṇa actually occupies that slot. After
**2.4.72 *adiprabhṛtibhyaḥ śapaḥ*** luks śap for adādi, `terms[SHAP]` is an
empty term still carrying śap's own `Tag::Pit` (3.1.68), and the ṅit tag —
assigned by 1.2.4's first application, which is ātmanepada-gated and excludes
loṭ uttama (`tinanta.rs:236`) — rides on `ENDING`. So on the śap-luk'd path
the guard interrogates a term that can never be ṅit, and 1.1.5 is silently off.

The same condition, with the same comment, appears at both **7.3.84**
(`tinanta.rs:1124`) and **7.3.86** (`tinanta.rs:1151`). Duplicating a subtle
condition in two places is how it came to be wrong in both.

There is a **third** `p.terms[SHAP].has(Tag::Ngit)` in the file, at
`tinanta.rs:797`, inside 1.2.4's *second* application. That one is **not** part
of this fix and must be left alone: it is an idempotence guard asking "have I
already tagged this vikaraṇa?", not an application of 1.1.5, and its subject is
correctly the vikaraṇa term itself. A search-and-replace across all three sites
would be a bug.

### The fix

One helper in `terms.rs`, answering *which sārvadhātuka immediately follows
the aṅga*:

- `terms[SHAP]` has text → that is the follower; return it. (Unchanged for
  every bhvādi / divādi / tudādi derivation.)
- `terms[SHAP]` is empty (śap luk'd, 2.4.72) → the ending is the follower;
  return `terms[ENDING]`.
- fewer than two terms → `None`; nothing follows, so nothing blocks.

Both call sites then read as the sūtra reads:

```rust
if following_sarvadhatuka(p).is_some_and(|t| t.has(Tag::Ngit)) {
    return false;
}
```

Note what the helper deliberately is **not**. On the *thematic* ātmanepada
path (√labh, bhvādi) the ending is ṅit but śap is pit, and guṇa correctly
proceeds — because the immediate follower is śap, not the ending. Encoding
"the immediate follower" rather than "is anything downstream ṅit" is exactly
what keeps that case right, and it is why the helper returns a term rather
than a boolean over all terms.

### Why the delta is zero

The correction makes the block operative on a path where nothing currently
reaches it:

- **7.3.84** needs an *ik*-final aṅga. Of the adādi roots, `yA` / `vA` / `As`
  are ā-final, `ad` / `vas` are consonant-final, and only `SI` ends in an ik.
  For `SI`, **7.4.21 *śīṅaḥ sārvadhātuke guṇaḥ*** is ordered immediately
  before 7.3.84 and has already reshaped the aṅga to `Se`; 7.3.84 then
  declines on its own shape guard (`guna_of('e')` is `None`) whether or not
  the 1.1.5 block is operative.
- **7.3.86** needs a light *ik* penult before a single consonant. `As`, `vas`
  and `SI` each fail that shape test independently of any guard.

So the expected outcome is byte-identical goldens and traces. **This is a
claim to verify by running, not to assert.** The plan's first verification
step after the edit is a full `mise run test`; any diff means this analysis is
wrong, and the response is to escalate, never to edit a golden to match engine
output.

### What it buys

**7.4.21 becomes load-bearing for the reason the sūtra gives.** Today √śī
takes guṇa because 1.1.5 is not looking; afterwards it takes guṇa because a
genuinely operative block is genuinely overridden by its targeted apavāda.
The latency note added in `04ca954` (the comment on 7.4.21 explaining that
7.3.84's guard "is not actually operative at this junction today") is deleted
rather than extended.

It also removes the standing trap: the next root that lands on the athematic
path with an ik-final aṅga and no 7.4.21-style override would have taken guṇa
it should not have, with no test able to see the cause.

### Deliberate narrowness

1.1.5 is *kṅiti* — ṅit **or** kit. This engine has no kit tag, because no
implemented rule assigns or consumes one. The helper and its callers test
`Tag::Ngit` only. That is a narrowed implementation of a wider sūtra, in the
same style as 8.2.25's and 8.3.59's narrow guards, and it is documented in
place with its restore trigger: the first rule that introduces a kit
sārvadhātuka must widen the test.

### The mutation-gate tension

This repo has twice refused to keep correct-but-unexercised code under the
mutation gate: 8.4.53 was removed as unreachable, and 6.1.78's E/O arms were
dropped with a documented restore trigger. The new athematic branch of the
guard is in exactly that position — no golden form traverses it, so
`cargo mutants` will surface it as a survivor unless a test executes it
directly.

The resolution is a **rule-level test that hand-constructs the shape** —
empty śap + ṅit ending + ik-final aṅga — in the style of the existing
`sarvadhatukardhadhatukayoh_blocks_guna_when_vikarana_is_ngit`
(`tinanta.rs:3394`), which already constructs a `Prakriya` by hand for the
thematic case. Such a test is straightforward to write and is expected to
kill the mutant.

**If it cannot**, the honest outcome is to drop the athematic branch and
upgrade 7.4.21's latency note instead of shipping unpinned code. That decision
is made during the plan, on evidence from an actual mutation run — not argued
after the fact.

### Approaches considered

- **Point-fix both sites inline.** Smallest diff, but duplicates the subtle
  condition in two places again. Rejected: the duplication is the defect's
  proximate cause.
- **Extract one helper (chosen).**
- **Extract the helper *and* rewrite the three existing athematic arms
  (6.1.78, 6.1.90, 6.1.66) onto it.** Rejected. Those arms are pinned by
  `*_athematic_*` guard tests that assert *provable disjointness* between the
  thematic and athematic arms of each rule. Funnelling all three through one
  shared helper means a mutant inside the helper can be killed by any single
  caller, collapsing three independent pins into one. The duplication there is
  load-bearing for the mutation gate in a way the 7.3.84 / 7.3.86 duplication
  is not.

## Debt 2 — splitting `tinanta.rs`

### Target layout

`crates/panini-prakriya/src/tinanta.rs` becomes `crates/panini-prakriya/src/tinanta/`:

| file | rules | holds |
|---|---|---|
| `mod.rs` | — | `derive`, the `TINANTA_RULES` stage list, `rules()` |
| `terms.rs` | — | `ANGA` / `ENDING_PRE_SHAP` / `SHAP` / `ENDING`, the 3.1.68-bisection NOTE, `following_sarvadhatuka` |
| `sound.rs` | — | `guna_of`, `vrddhi_of`, `is_vowel`, `is_jhal`, `is_khar`, `cartva_of`, `is_vibhakti_protected_final` |
| `samjna.rs` | 5 | 1.3.12, 1.3.78, 3.4.78, 1.3.9, 1.2.4 — pada sanction, it-elision, ending insertion. **Before** 3.1.68 |
| `tin.rs` | 18 | 3.4.85 … 3.4.102 — lakāra → tiṅ substitution and ending reshaping. **Before** 3.1.68 |
| `vikarana.rs` | 5 | 3.1.69, 3.1.77, 3.1.68, 2.4.72, 1.2.4 — **contains** the boundary |
| `anga.rs` | 14 | 6.4.71 … 7.3.101, including 6.1.78 — **after** the boundary |
| `adesha.rs` | 8 | 6.1.101 … 6.4.101 — after the boundary |
| `tripadi.rs` | 6 | 8.2.77 … 8.4.55 — after the boundary |
| `derivation_tests.rs` | — | `#[cfg(test)]` only: whole-derivation tests and the shared test helpers |

5 + 18 + 5 + 14 + 8 + 6 = **56**, the current rule count.

The rule ids already cluster into these six stages in the existing array
order, so the split follows boundaries the pipeline already has rather than
imposing new ones. Two placements are worth stating so they do not read as
filing mistakes:

- **`samjna` / `tin` divide at 3.4.78.** 3.4.78 *tiptasjhi…* inserts the
  ending; everything from 3.4.85 on substitutes and reshapes it. Splitting a
  23-rule block there keeps both halves near the size of the other stages.
- **6.1.78 sits in `anga`,** not with the other 6.1.x rules, because that is
  where the current order puts it. Order outranks family.

Target: **no file over roughly 700 lines**, down from 4058. `anga.rs` is
expected to be the largest, since it carries both the 7.x rules and the
`pugantalaghupadhasya_*` / `eco_yavayavah_athematic_*` guard blocks. If its
real line count lands well past that once the move is done, splitting it at
the 7.x / 6.1.78 seam is the natural follow-up — decided in the plan against
measured counts, not guessed here.

### Tests move with their rules

Each stage file carries its own guard tests: `pugantalaghupadhasya_*` beside
7.3.86, `awas_ca_athematic_*` beside 6.1.90, `eco_yavayavah_athematic_*`
beside 6.1.78. In this codebase the guard test *is* the rule's documentation
of its own boundary, and separating the two would be a regression in
navigability even though it would shrink the file just as much.

Whole-derivation tests go to `derivation_tests.rs`, a `#[cfg(test)] mod`
declared from `mod.rs`, because they exercise the pipeline rather than any one
stage: the paradigm-shaped cases (`bhu_3sg_is_bhavati`, the `*_all_nine_cells`
blocks) and the cross-cutting invariants
(`recorded_step_names_match_tinanta_rules_for_every_id`,
`sutra_names_contain_no_forbidden_slp1_digraphs`). They are roughly 700 lines
on their own, which is why they get a file rather than living in `mod.rs` —
`mod.rs` should stay small enough to read as a table of contents for the
pipeline. The `form` / `form_g` / `lin_form` / `lin_a_form` helpers move there
too, as `pub(crate)` items the stage files' test modules can import.

The `sound.rs` helpers keep their existing unit tests
(`guna_of_ik_vowels_all_arms`, `vrddhi_of_ac_vowels_all_arms`,
`is_vowel_distinguishes_vowels_from_consonants`), which already treat them as
a standalone layer.

### The `TINANTA_RULES` type change

A single array literal cannot span files, so the flat array becomes a list of
stages:

```rust
pub static TINANTA_RULES: &[&[Rule]] =
    &[SAMJNA, TIN, VIKARANA, ANGA, ADESHA, TRIPADI];
```

with `run_pipeline` taking `&[&[Rule]]` and walking stages, then rules within
each stage. Everything stays `static`: no `LazyLock`, no allocation, no
runtime initialisation. The stage boundary becomes explicit in the type rather
than implicit in one file's line order.

The alternative — leaving `run_pipeline` alone and having `derive` call it
once per stage — was rejected because the `p.blocked` early return would then
work across stage boundaries only by accident (each fresh call happens to
re-check `blocked` on its first iteration). One explicit signature is more
honest than that.

`mod.rs` also gains:

```rust
pub fn rules() -> impl Iterator<Item = &'static Rule>
```

for the ~20 tests that currently do `TINANTA_RULES.iter().find(|r| r.id == …)`,
plus `recorded_step_names_match_tinanta_rules_for_every_id`, which iterates the
whole list.

### The ordering invariant

**The flattened stage list must reproduce the current 56-rule sequence
exactly.** The ordered array *is* the grammar; a split that reorders it is a
grammar change wearing a refactor's clothes, and a subtle reorder can still
produce correct surface forms while producing a wrong derivation.

This gets a new test pinning the flattened id sequence verbatim. It is a
cheaper and sharper guard on this specific refactor than the per-derivation
ordering in `crates/panini/tests/trace.rs`, which pins order only along the
paths representative forms happen to take.

### Comments that must survive the move

Two comments are load-bearing and are the likeliest casualties of a file
split. Both move to `terms.rs` as module documentation, next to the constants
they constrain:

1. **The 3.1.68 bisection.** `ENDING_PRE_SHAP` and `SHAP` are both index 1,
   deliberately. Rules ordered before 3.1.68 must address the ending as
   `ENDING_PRE_SHAP`; rules after it must use `ENDING` (index 2) and may use
   `SHAP`. A rule placed on the wrong side either mutates śap while believing
   it is mutating the ending, or panics indexing `terms[2]` before that slot
   exists.
2. **The empty-śap caveat.** Since adādi, `terms[SHAP].text` may be empty:
   `ends_with` / `is_empty` / `chars().next()` matched as an `Option` are
   safe, while `chars().next().unwrap()` panics.

Each stage file states in one line which side of 3.1.68 it sits on. The split
therefore *promotes* both caveats to a place they can be found, rather than
scattering them.

## Sequencing

One branch, two clearly separated commit groups:

1. **Guard fix** — `following_sarvadhatuka` in the existing `tinanta.rs`, both
   call sites, the new rule-level guard tests, 7.4.21's latency note deleted.
   Reviewable against current history rather than against a 4000-line move.
2. **Split** — `tinanta.rs` → `tinanta/`, the stage list, `run_pipeline`'s
   signature, `rules()`, the ordered-id pin, the doc updates below.

## Documentation to update

Both of these name the file by path and describe it as a single flat array, so
both go stale on the split:

- `docs/ARCHITECTURE.md:22` — "`TINANTA_RULES` (in
  `crates/panini-prakriya/src/tinanta.rs`) is a single ordered `&[Rule]`".
  Becomes the stage list, with the stage table and a restatement that reading
  the stages in order still IS reading the grammar.
- `crates/panini/tests/trace.rs:16` — the "read `TINANTA_RULES` in
  `crates/panini-prakriya/src/tinanta.rs` top to bottom" pointer.
- `AGENTS.md` — "New grammar goes in `TINANTA_RULES` as a self-guarding
  `Rule`" gains *which stage file*, and the rule about which side of 3.1.68 a
  new rule belongs on.

## Testing

Per commit group:

**After group 1 (guard fix):**

- Full `mise run test`. Expectation: zero diff in all 1080 goldens and every
  trace line. Any diff is an escalation, not a golden edit.
- New rule-level guard tests, following the existing inside/outside pattern —
  for each of 7.3.84 and 7.3.86, adjusted to each rule's own shape
  requirement:
  - śap luk'd (empty) + ṅit ending + guṇa-eligible aṅga → blocked. **This is
    the test that must kill the mutant on the new branch.**
  - śap luk'd + non-ṅit ending + guṇa-eligible aṅga → fires.
  - śap non-empty + ṅit vikaraṇa → blocked, as before (regression pin on the
    thematic arm).
  - fewer than two terms → fires, without panicking.

**After group 2 (split):**

- Full suite again, plus the new flattened-id-sequence pin.
- `mise run fmt-check`, `mise run lint`, `mise run audit`.

**Slice end:**

- `mise run mutants`, expected at **zero** survivors. Invoke the
  `cargo-mutants` binary directly rather than through the `mise` shim, which
  fails in background shells.

Scoping a single crate during iteration uses
`mise exec -- cargo test -p panini-prakriya`; `mise run test -- -p X` does not
scope.

`panini-lipi` roundtrip / property / fuzz targets are untouched.

## Risks

1. **The new arm survives mutation.** The central risk, since the fix is
   behaviour-preserving by design and therefore invisible to the golden suite.
   Mitigation and fallback are stated above: a hand-constructed rule-level
   test, and if that cannot kill the mutant, drop the arm and upgrade 7.4.21's
   latency note. Decided on evidence during the plan.
2. **The split silently reorders a rule.** The most dangerous failure mode
   here, because order is grammar and a reorder can preserve surface forms
   while corrupting derivations. Mitigated by the verbatim id-sequence pin,
   which fails on any reorder whether or not surfaces change.
3. **A load-bearing comment is dropped in the move.** Named explicitly above
   with destinations; "comments moved intact" is a review item for group 2,
   not an afterthought.
4. **Diff size.** Group 2 is a ~4000-line move. Rename detection should carry
   most of it, but review effort is real — which is precisely why the semantic
   change is quarantined in group 1.

## Success criteria

- All **1080** golden forms validate `VALID`; every trace is byte-identical to
  `main`.
- 1.1.5's block is operative on the śap-luk'd path, pinned by rule-level tests
  that fail if the branch is removed.
- 7.4.21's latency note is deleted, not extended.
- The duplicated inline guard exists in exactly one place.
- `tinanta/` has no file over roughly 700 lines; the flattened rule order is
  identical to `main` and pinned by a verbatim id-sequence test.
- 1.2.4's idempotence guard at the old `tinanta.rs:797` is unchanged.
- `docs/ARCHITECTURE.md`, `crates/panini/tests/trace.rs` and `AGENTS.md` name
  the new layout; no doc points at `crates/panini-prakriya/src/tinanta.rs`.
- `mise run mutants` reports **zero** survivors; `lint`, `fmt-check` and
  `audit` are clean.
