# Juhotyādi gaṇa (gaṇa 3), slice 3c2 — √hā (parasmaipada), √gā, and `Rule.bars`

Slice 3c left the gaṇa at eight of its twenty-six rows and recorded this slice a
slice ahead:

> 72 cells, 103 forms (61 + 42); suite 3852 → 3924 cells, 4823 → 4926 forms,
> 85 → 87 roots. […] 6.4.116 *jahāteś ca* […] 6.4.117 *ā ca hau* […] 6.4.118
> *lopo yi* […] A seven-form cell […] 7.4.78 *bahulaṁ chandasi* […] 7.4.76 must
> decline on `03.0009`.

Re-probed at the audited commit `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, and
with this engine at `1dcbc1b` run over the same cells on hand-built rows and
diffed cell by cell against vidyut, the record holds exactly: 72 cells, 103
forms. HEAD already matches **12 of 72** cells. The rest split cleanly:

- **√gā (`03.0026`) differs on all 36 cells, and only in the abhyāsa vowel**:
  *jag-* for *jig-*. One rule, 7.4.78, closes every cell. No new vikalpa: its
  multi-form cells are the existing 7.1.35 / 8.4.56 forks (*jigIhi* only; 6.4.117
  is √hā's).
- **√hā (`03.0009`) differs on 24 cells**, all from three rules: 6.4.118 before
  `y` (vidhiliṅ, 9 cells), and the 6.4.116 / 6.4.117 vikalpas (15 cells).

3c2 decided the two questions 3c left open:

- **7.4.78 is transcribed, citing the Siddhānta-kaumudī.** It is a chāndasa
  sūtra. vidyut applies it to √gā on the Kaumudī's authority (its own comment:
  "This is a chAndasa rule, but the SK applies it to derive jigAti from gA, which
  is a Vedic root."). This follows the 7.3.86 vikalpa-arm precedent: the
  Pāṇinian id is kept and the non-Aṣṭādhyāyī source is recorded in the comment.
  No repo-wide convention is added.
- **A text-neutral vikalpa marks its branch through a declared apavāda relation,
  `Rule.bars`**, not through a guard that reads the log. See below.

## Scope

**Rows (2):** `03.0009 o~hA\k` √hā and `03.0026 gA\` √gā, both parasmaipada.
**72 cells, 103 forms.**

**New plumbing (1):** `Rule.bars` and `Prakriya.barred`, enforced in
`run_pipeline`.

**New rules (4):** 6.4.118 *lopo yi*, 6.4.117 *ā ca hau* (vikalpa), 6.4.116
*jahāteś ca* (vikalpa), all in `guna.rs`; 7.4.78 *bahulaṁ chandasi*
(`abhyasa.rs`). The engine goes from nine optional rules to **eleven**.

**Unchanged, confirmed by the HEAD diff:** 6.4.113 and 6.4.112's abhyasta arms
(they give *jahati*, *jahItaH*, *jigItaH*, *ajiguH* once the new rules are in
place), 7.4.76's number guard (*jahAti* already correct at HEAD), 7.4.62, 7.4.59,
8.4.54, 7.1.35, 8.4.56, 7.2.79, 6.1.96, 6.1.101, 3.4.109, and the data layer.
`strip_anubandhas` gives `hA` and `gA`, and `pada_from_upadesha` gives
parasmaipada for both (1.3.78).

**Out of scope:** 3d–3f; curādi; moving 7.1.6's read of the log for 7.1.5, or any
other existing guard, onto `bars`; any other chāndasa sūtra.

## Root selection

| row | entry | root | pada | sanction |
|---|---|---|---|---|
| 03.0009 | o~hA\k | hA | parasmai | 1.3.78 (the `o~` is 1.3.2's) |
| 03.0026 | gA\ | gA | parasmai | 1.3.78 |

`curated_pada_agrees_with_upadesha_markers` and
`dhatupatha_numbers_resolve_upstream` cover both without edits.

## `Rule.bars`: the apavāda relation as data

6.4.117 *ā ca hau* optionally keeps √hā's `A` before *hi*. vidyut writes it as
`optional_run_at("6.4.117", i, op::antya("A"))` on an aṅga that already ends in
`A`: a step that changes no text, whose only effect is that 6.4.116 and 6.4.113
do not run on its branch. Grammatically it is an apavāda of both.

The fork machinery already allows a text-neutral vikalpa. `run_pipeline` keeps
the clone whenever `apply` returns true, and `Prakriya::record` does not require
the text to change. What is missing is a way for the branch to block the two
rules. Adding "6.4.117 is not in `p.log`" to their guards would work (7.1.6
reads the log for 7.1.5 today), but it hides the relation inside a guard, keys it
on a string nothing checks, and every future optional apavāda would repeat it.
So the relation becomes data on the rule, and the controller enforces it:

```rust
pub struct Rule {
    // … id, name, kind, vikalpa, apply …
    /// Rules this one bars on every branch where it fires: the apavāda
    /// relation, declared rather than re-derived in the barred rules' guards.
    /// `run_pipeline` skips a barred rule on that branch.
    ///
    /// Scope is the BRANCH, not a site. A tinanta prakriyā has one aṅga, so the
    /// two coincide today; a pipeline with several sites must revisit this.
    pub bars: &'static [&'static str],
}

pub struct Prakriya {
    // …
    /// Rule ids barred on this branch by a rule that fired earlier (`Rule.bars`).
    /// Cloned with the branch, so a vikalpa's two readings diverge here too.
    pub barred: Vec<&'static str>,
}
```

`run_pipeline`:

- skips a rule whose `id` is in `branch.barred`, the same way it skips a
  `blocked` branch;
- after `apply` returns true, extends that branch's `barred` with `rule.bars`.
  For a vikalpa, that is the applied clone only; the declined branch in place is
  untouched.

All 121 `Rule` literals gain `bars: &[]`. The change is mechanical, and it keeps
the relation next to `vikalpa`, where a reader of the rule sees it. A side table
(`APAVADA: &[(&str, &[&str])]`) was considered and rejected because it separates
the relation from its rule.

**Statically checked ids.** A pipeline-level test walks the flattened stage
list. For every rule with a non-empty `bars`, each id must name a rule in the
pipeline, and that rule must come **after** the one barring it. Barring an
earlier rule does nothing, so it is a bug. A typo or a later reorder fails
loudly instead of silently never matching.

**Not migrated:** 7.1.6's read of the log for 7.1.5 is an *enabling* condition,
not barring, so `bars` does not express it. It stays.

## The four rules

All three √hā rules go in `guna.rs`, directly above 6.4.113, in this order. Each
is keyed on `ctx.dhatupatha == "03.0009"`, with a comment naming `o~hA\k`,
because *jahāteḥ* is the juhotyādi √ohāk. `03.0008 o~hA\N` is also `hA`, and root
text cannot separate the two (3c's argument for `Context.dhatupatha`).

| order | rule | kind | guard beyond `03.0009` and "aṅga ends in `A`" | effect | bars |
|---|---|---|---|---|---|
| 1 | 6.4.118 *lopo yi* | nitya | follower is kṅit and `y`-initial | elide `A`: *jahyAt* | — |
| 2 | 6.4.117 *ā ca hau* | vikalpa | `ENDING.text == "hi"` | records a step, no text change: *jahAhi* | `6.4.116`, `6.4.113`, `6.4.112` |
| 3 | 6.4.116 *jahāteś ca* | vikalpa | follower is kṅit and consonant-initial | `A` → `i`: *jahitaH*, *jahihi* | — |
| 4 | 6.4.113 (existing) | nitya | unchanged | `A` → `I` | — |

The follower is read through `following_sarvadhatuka`, as 6.4.113's abhyasta arm
reads it: under ślu the śap is empty, so the follower is the ending, carrying
yāsuṭ in vidhiliṅ and Ngit from 3.4.103.

- **6.4.118 first.** Before `y`, it removes the `A` that 6.4.116, 6.4.113 and
  6.4.112 all require, so all three decline without further guards. vidyut
  gives the same precedence (`if y-initial { 6.4.118 } else { 6.4.117 / 6.4.116 }`).
- **6.4.117 before 6.4.116, barring all three.** *ā ca hau* keeps the `A`, so
  on its branch every rule that would change that `A` must not run: 6.4.116,
  6.4.113, and **6.4.112**. The last one is easy to miss. *hi* is kṅit (1.2.4)
  and the `A` is still there, so without the bar 6.4.112's abhyasta arm elides
  it and gives \**jahhi*. vidyut gets the same exclusion structurally
  (`if !changed { 6.4.113 or 6.4.112 }`). This entry was found while writing
  the spec, not in the probe; the *jahAhi* trace pin ("no 6.4.112") holds it.
- **Ordering caveat (`rule.rs`).** 6.4.116 invalidates "the aṅga ends in `A`".
  Its consumers below it are 6.4.113 and 6.4.112. On 6.4.116's branch both are
  meant to decline and do, so the invalidation is the intended bleeding. The rule
  comment states this argument.
- **7.4.78** goes in `abhyasa.rs` directly after 7.4.76, and is keyed on
  `03.0026`. It turns the abhyāsa's `a` into `i`: *ga* → (7.4.62) *ja* → *ji*.
  The comment carries the SK attribution and vidyut's own note, and says the
  sūtra's *bahulam* is not modelled as a vikalpa, since vidyut gives *jig-*
  only.

## Counts

| | before | after |
|---|---|---|
| roots | 85 | **87** |
| root × pada × lakāra blocks | 428 | **436** |
| cells | 3852 | **3924** |
| forms | 4823 | **4926** |
| `ALTERNATES` rows | 971 | **1002** |
| optional rules | 9 | **11** |

Shape buckets (`derivation_set_shape_matches_the_audited_numbers`):

| forms per cell | before | after | new cells |
|---|---|---|---|
| 1 | 3133 | 3184 | √hā 19, √gā 32 |
| 2 | 554 | 571 | √hā 15, √gā 2 (laṅ, vidhiliṅ prathama eka) |
| 3 | 121 | 123 | √gā loṭ prathama and madhyama eka |
| 4 | 18 | 18 | — |
| 5 | 9 | 10 | √hā loṭ prathama eka |
| 6 | 17 | 17 | — |
| **7** | — | **1** | √hā loṭ madhyama eka, a **new record**; the test gains a `sevens` bucket |

New `ALTERNATES` rows by key (keys join the vikalpa ids that fired, in pipeline
order: 7.1.35 in the tiṅ stage, 6.4.117 / 6.4.116 in `guna.rs`, 8.4.56 in the
tripādī):

| key | √hā | √gā | total | registry |
|---|---|---|---|---|
| `6.4.116` | 14 | — | 14 | new |
| `6.4.117` | 1 | — | 1 | new |
| `7.1.35+6.4.116` | 2 | — | 2 | new |
| `7.1.35+6.4.116+8.4.56` | 2 | — | 2 | new |
| `8.4.56` | 2 | 2 | 4 | 142 → 146 |
| `7.1.35` | 2 | 2 | 4 | 120 → 124 |
| `7.1.35+8.4.56` | 2 | 2 | 4 | 120 → 124 |
| | **25** | **6** | **31** | 971 → 1002 |

The `6.4.116` rows are laṭ ×5 (prathama, madhyama and uttama dvi; madhyama and
uttama bahu), laṅ ×5 (the same five), loṭ ×3 (prathama and madhyama dvi,
madhyama bahu), and loṭ madhyama eka's *jahihi*.

### The seven-form cell

√hā loṭ parasmaipada madhyama eka, in branch order:

1. 7.1.35 forks *hi* / *tāt*.
2. 6.4.117 forks the *hi* branch.
3. 6.4.116 forks the unbarred *hi* branch and the *tāt* branch.
4. 6.4.113 takes whatever keeps its `A`.
5. 8.4.56 forks each *tāt* form's final.

| index | form | key |
|---|---|---|
| 0 | *jahIhi* | (declined: 6.4.113) |
| | *jahihi* | `6.4.116` |
| | *jahAhi* | `6.4.117` |
| | *jahItAd* | `7.1.35` |
| | *jahItAt* | `7.1.35+8.4.56` |
| | *jahitAd* | `7.1.35+6.4.116` |
| | *jahitAt* | `7.1.35+6.4.116+8.4.56` |

## Testing

- **Controller** (`controller.rs`, toy rules): a barred rule is skipped; a
  vikalpa's `bars` apply to the applied clone only, and the declined branch still
  runs the barred rule; a barring rule that declines bars nothing.
- **Pipeline:** every `bars` id names a rule in the flattened pipeline, ordered
  after its barrer.
- **Guards:** each new rule declines on `03.0008` (same `hA`) and on the `""`
  default. 6.4.118 declines before a non-`y` follower. 6.4.117 declines before
  `tAt`. 6.4.116 declines before vowel-initial and pit followers. 7.4.78 declines
  on every `gA`-shaped aṅga but `03.0026`.
- **Trace pins:**
  - *jahAhi*: 6.4.117, and none of 6.4.116, 6.4.113 or 6.4.112 in its log.
  - *jahihi* and *jahitaH*: 6.4.116.
  - *jahyAt*: 6.4.118.
  - *jahAti*: no 7.4.76. This is the witness 3c promised.
  - *jigAti*: 7.4.62, then 7.4.78.
- `exactly_the_pinned_vikalpa_rules_are_optional` gains 6.4.116 and 6.4.117.
- Goldens: two new `PARADIGM` blocks per root (parasmaipada only: 8 blocks), 31
  `ALTERNATES` rows, and the 3852 priors byte-identical.

## Audit and gate

- **Audit.** Repoint `/tmp/vidyut-full`'s dev-deps at this worktree first; they
  pointed at the deleted 3c worktree when this spec was probed. Run the committed
  `tools/audit` harness, raised to 87 / 3924 / 4926, and expect zero divergence
  against `8da2f90b`. Run the README's negative control before believing the
  result.
- **`mise run test-full`** once, before the gate.
- **Mutation gate.** Re-measure the uncaught-suite floor at 3924 cells, at the
  parallelism the campaign uses, and set `--timeout` above it. Do not project it
  from 3852. Pass `-o` to an isolated directory and copy `outcomes.json`
  somewhere durable before any follow-up invocation. The non-caught set must
  equal the documented equivalents plus the known permanent timeout. AGENTS.md
  names every missed or timed-out mutant verbatim. The new controller lines must
  be caught, not merely reached.

## Docs sweep

README, AGENTS and every count: juhotyādi reaches **ten of twenty-six** rows. The
grep covers wrapped counts, rule-scoped counts, spelled-out number words ("nine
optional rules", "six-form record", "eight of twenty-six"), root-shape literals
(`hA`, `gA`, `jah`, `jig`), and every file a behaviour task touched, including
comments this slice's own new rule ids make false (6.4.113 / 6.4.112's
"what reaches here" comments; 7.4.76's `03.0009` note, which now has its
derivation witness).

## Risks

1. **The `bars` field touches every rule.** It is mechanical, and a missing field
   is a compile error, so nothing can be skipped silently. The diff is large but
   uniform; review the controller change on its own.
2. **Barring is rule-wide on the branch.** 6.4.113 also has a śnā arm. A
   branch barred by 6.4.117 is √hā's, which never carries śnā, so nothing is
   lost. The doc comment records the branch-not-site scope.
3. **6.4.112 in `bars`** is the one entry the probe did not show directly
   (vidyut excludes it structurally). If it is dropped, *jahAhi* becomes
   \**jahhi* in the goldens, and the trace pin "no 6.4.112" names the cause.
4. **Suite cost.** Two more roots grow `roundtrip_sampled`'s residual Θ(N²)
   term, hence a re-measured cap rather than a projected one.
5. **Hand-derived counts.** They are stated exactly above so the generated
   goldens contradict them audibly.

## Success criteria

- 3924 cells VALID with the trace pins; the 3852 priors byte-identical.
- Audit zero-divergence at 87 / 3924 / 4926 against `8da2f90b`.
- `mise run test-full` passes.
- Gate clean at a re-measured cap.
- `bars` ids are validated by test; no rule guard reads `p.log` for 6.4.117.
- Juhotyādi at ten of its twenty-six rows; README and AGENTS swept.

## Later slices

3d (the seven ṛ-final rows, including 7.4.76's √bhṛñ arm `03.0006`), 3e (√nij,
√vij, √viṣ) and 3f (the six ordinary-vowel rows) remain. 3a's table stands for
them. Re-probe each before writing its spec.
