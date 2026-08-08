# Optional rules (slice 5b) — 6.4.107 *lopaś cāsyānyatarasyāṁ mvoḥ*

Slice 5a shipped the svādi gaṇa and froze the suite at 1512 byte-identical
forms. It left one gap on purpose: `hinmaH` is pinned INVALID in
`known_nonforms_are_invalid` with a comment naming this slice. Closing that gap
means teaching the engine its first genuinely **optional** rule, which changes
what a derivation *is* — one prakriyā becomes a set.

That is the whole slice. The svādi spec deferred 6.4.107 not because the rule
is hard (its guard is two tests against machinery 5a already built) but because
optional-rule support is not a svādi feature: it is a change to the pipeline
model, and landing it alone — against a suite 5a has already frozen — means
any fallout is attributable to the fork machinery and nothing else.

## Scope

Unchanged: the 42-root set, the four lakāras (laṭ, laṅ, loṭ, vidhiliṅ), all
nine puruṣa × vacana cells, six gaṇas. No new roots, no new gaṇa.

New: the fork mechanism, and exactly one rule that uses it — 6.4.107. Eight
cells gain a second valid form. `PARADIGM`'s 1512 strings do not change.

Out of scope, deferred:

- **7.1.35 *tuhyos tātaṅ āśiṣy anyatarasyām*** (loṭ `ApnutAt`) and **8.4.56
  *vāvasāne*** (pausal `Apnod`). Both are genuine *anyatarasyām*/*vā* rules
  that this machinery would unlock, and both are currently repo-wide
  one-form-per-cell conventions rather than grammar. Retiring them is the
  obvious follow-up, and it is deliberately not this slice: 7.1.35 alone would
  add a loṭ alternate for every root in the suite, which destroys the
  attribution property the 5a/5b split exists to preserve. They become
  tractable once the machinery has one witness in the suite.
- **6.4.108 *nityaṁ karoteḥ***, which makes the same lopa obligatory for √kṛ.
  It is the reason 6.4.107 is optional at all — the *nitya* of 6.4.108 is
  stated against the *anyatarasyām* of 6.4.107 — but √kṛ is not in the root
  set, and it wants 7.1.100 and the 6.4.10x kṛ-specials besides.
- **Ubhayapadī roots and 1.3.72 *svaritañitaḥ***, still deferred, as in every
  gaṇa spec so far.

## The rule

`6.4.107 lopaScAsyAnyatarasyAM mvoH`, verified against vidyut-prakriya's
`data/sutrapatha.tsv` (ashtadhyayi.com is a JS single-page app that cannot be
fetched programmatically; the TSV is what this repo checks ids and names
against).

*Asya* is 6.4.106's `u` — an affix-final `u`, *asaṁyogapūrva* by anuvṛtti from
the same source 6.4.87 inherits it from — and it is optionally elided before
`m` and `v`. The existing `shnu_asamyogapurva` helper in `tinanta/terms.rs` is
exactly that predicate, unchanged and ending-agnostic, so the guard is two
tests:

```rust
Rule {
    id: "6.4.107",
    name: "lopaScAsyAnyatarasyAM mvoH",
    kind: RuleKind::Vidhi,
    vikalpa: true,
    apply: |p| {
        if !p.terms[ENDING].text.starts_with(['m', 'v']) {
            return false;
        }
        if !shnu_asamyogapurva(p) {
            return false;
        }
        let before = p.snapshot();
        p.terms[SHAP].text.pop();
        p.record("6.4.107", "lopaScAsyAnyatarasyAM mvoH", before);
        true
    },
},
```

The mutation is a `pop()` — *lopa* of the `u` — not a rewrite of śnu's text to
`"n"`. The two are indistinguishable in the output string and not
indistinguishable in intent; a test pins the shape.

### Placement

`adesha.rs`, immediately after 6.4.106 and before 6.4.101: the third member of
the 6.4.105 / 6.4.106 luk-and-lopa run, which is also where sūtra order puts
it. `tinanta_rule_order_is_pinned` grows from 67 ids to 68 at that position.

It does not contend with 6.4.101, whose guard requires `ENDING == "hi"` — an
ending that is neither `m`- nor `v`-initial.

### The ordering constraint, which is invisible and permanent

**6.4.107 must be ordered after every consumer of `shnu_asamyogapurva`.**

The helper's first guard is `SHAP.text == "nu"`. On the branch where 6.4.107
has fired, `SHAP.text` is `"n"`, so the helper returns false for the remainder
of the pipeline — correctly, in the sense that there is no longer an `u` to ask
about, but silently. Its three current consumers (6.4.87 and 6.4.77 in
`guna.rs`, 6.4.106 immediately above) all precede it, so the constraint holds
today for free.

A future consumer placed *after* 6.4.107 would read the wrong answer on one of
the two branches only. That failure shows up as half a paradigm being wrong,
with both halves individually plausible. This is recorded in `AGENTS.md`
alongside the `vikalpa` convention, because nothing in the code can enforce it.

### Why the witness is exactly eight cells

The endings 6.4.107 can see are `vas`/`mas` (laṭ uttama dvi/bahu) and
`va`/`ma` (laṅ uttama dvi/bahu), all still pre-visarga at this point in the
pipeline. Vidhiliṅ's corresponding endings surface as `yAva`/`yAma`, which
begin with `y`; loṭ's āṭ-augmented `Ava`/`Ama` begin with `A`. Neither is
`m`- or `v`-initial.

Of the six svādi roots, only √hi and √ri are *asaṁyogapūrva*; √āp, √śak, √aś
and √ṣṭigh all put a conjunct before śnu's `u`. Both ātmanepadī svādi roots
(√aś, √ṣṭigh) are among the conjunct four, so the ātmanepada column is
untouched. Two roots × two lakāras × two cells = eight.

| | laṭ U.D | laṭ U.B | laṅ U.D | laṅ U.B |
|---|---|---|---|---|
| √hi | hinvaH | hinmaH | ahinva | ahinma |
| √ri | riRvaH | riRmaH | ariRva | ariRma |

Slice 5a derives the un-elided member of each pair (`hinuvaH`, `hinumaH`,
`ahinuva`, `ahinuma`, and the √ri counterparts); those stay in `PARADIGM`
unchanged, and this slice adds the elided member as a second valid form.

ṇatva reaches the elided branch normally — 8.4.1 / 8.4.2 live in `tripadi`,
downstream of the fork — so `rinvas` becomes `riRvaH` by the same path
`rinuvas` becomes `riRuvaH`.

## The fork machinery

### `Rule` gains a field

```rust
pub struct Rule {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: RuleKind,
    /// vikalpa — the sūtra applies optionally (anyatarasyām / vā / vibhāṣā).
    /// `run_pipeline` forks the prakriyā here: both the applied and the
    /// declined reading continue as independent derivations.
    pub vikalpa: bool,
    pub apply: fn(&mut Prakriya) -> bool,
}
```

This costs a mechanical `vikalpa: false` on all 67 existing rule literals. The
cheaper alternative — a side table `VIKALPA_RULES: &[&str] = &["6.4.107"]` that
`run_pipeline` consults — is one line, but it puts a fact *about the grammar*
somewhere other than the rule, and this repo's central claim is that reading
`TINANTA_RULES` in order is reading the grammar. `RuleKind` already sits inline
for that reason.

Optionality is a separate axis from sūtra type, so it is a separate field, not
a `RuleKind::Vikalpa` variant: 6.4.107 is a vidhi *and* optional, and
collapsing the axes would lose that.

### `run_pipeline` carries a worklist

```rust
pub fn run_pipeline(p: Prakriya, stages: &[&[Rule]]) -> Vec<Prakriya>
```

Rule-outer, branch-inner. For each rule, walk the live branches:

- A **blocked** branch is skipped, but stays in the vec. Callers already test
  `blocked` and must keep doing so; a blocked branch's partial text is not a
  surface form.
- A **non-`vikalpa`** rule applies in place, exactly as today. One branch in,
  one branch out.
- A **`vikalpa`** rule clones the branch, runs `apply` on the clone, and keeps
  the clone **only if `apply` returned true**. A rule that declines its own
  guard therefore forks nothing, and the ~1504 cells 6.4.107 never touches stay
  single-branch.

Two invariants, both load-bearing:

1. **Branch order is deterministic: declined first.** The declined branch stays
   in place; the applied clone is inserted immediately after it. With one
   optional rule that gives `[hinuvaH, hinvaH]`. Declined-first is chosen over
   applied-first for a specific reason: index 0 is then byte-identical to what
   5a produced for *every* cell in the suite, which turns "the 1512 goldens are
   unchanged" from a hope into a checkable claim (see "Slice split").
2. **Rules see branches in a stable order.** Fork insertions are collected
   during a rule's sweep and applied after it finishes, never mutating the vec
   mid-iteration.

### `derive` returns a set

Both `panini_prakriya::tinanta::derive` and `Panini::derive` return
`Vec<Prakriya>`. All call sites are in tests (~20, listed by
`grep -rn "derive(" --include=*.rs crates`); the existing helpers in
`derivation_tests.rs` absorb most of them.

A `sole()` helper asserts exactly one branch and returns it. Every existing
single-form helper (`form`, `form_g`, `lin_form`, `lat_a_form`, `lot_a_form`,
`lin_a_form`) routes through it, so a cell that forks unexpectedly fails at the
unit-test level, not only in the goldens.

## How the set surfaces

Smaller than the svādi spec anticipated. That spec listed "the `panini` facade,
the CLI's `--json` shape" among what 5b would disturb. In fact
`CheckResult.analyses` is *already* a `Vec<Analysis>`, because a single surface
form can already have several analyses (two roots sharing an SLP1 code,
cross-lakāra coincidences). A fork adds members to a list that already exists.

- **`panini-analyze`: unchanged.** Candidates remain
  `(root, lakāra, pada, puruṣa, vacana)`. The fork is strictly inside
  derivation, downstream of candidate generation.
- **`Panini::check`: one nested loop.**
  `for c in candidates(&slp1) { for p in derive(...) { if !p.blocked && p.text() == slp1 { push } } }`.
  The `!blocked` test already present stays exactly as it is, now per branch.
- **`Analysis`: unchanged.** No "this form is optional" flag. The trace already
  carries `6.4.107 lopaScAsyAnyatarasyAM mvoH`, and that *is* the statement
  that this form arose from an optional rule; a boolean beside it would be a
  second, drift-prone encoding of the same fact.
- **`panini-cli`: no change at all.** No new flag, no `--json` schema change,
  no exit-code change. `panini check hinvaH --trace` works the moment the rule
  lands and shows 6.4.107 in its trace; `panini check hinuvaH --trace` shows
  the same trace without it. The `--json` `analyses` array simply has whatever
  members it has.

**Converging branches are not deduplicated.** 6.4.107 can never produce a
collision — eliding a vowel always changes the string — but if a future
optional rule did, two `Analysis` entries with the same form and different
traces is the honest answer: one form, two derivations. That is information,
not noise. The exhaustiveness test below compares sets of texts, so it is
indifferent either way.

## Verification

### `controller.rs` — fork mechanics

Against synthetic rules, not real sūtras:

- a `vikalpa` rule whose guard declines forks nothing;
- one that fires yields two branches, with the declined branch at index 0;
- later rules apply to each branch independently;
- two firing `vikalpa` rules yield four branches in a deterministic order;
- a blocked branch is skipped by later rules but still returned;
- a non-`vikalpa` rule takes one branch to one branch.

### `derivation_tests.rs`

`tinanta_rule_order_is_pinned` goes 67 → 68 ids, with `6.4.107` seated between
`6.4.106` and `6.4.101`.

A new test asserts **exactly one** rule in `TINANTA_RULES` has
`vikalpa == true`, and that its id is `6.4.107`. This is the tripwire for a
mis-set flag in the 67-line mechanical edit — the failure mode there is a
silently doubled branch, which no golden would necessarily catch.

### `adesha.rs` — per-rule guards, beside the rule

The m/v-initial test; the *asaṁyogapūrva* test; and a pin that the mutation is
`pop()`-shaped (elide the `u`) rather than a rewrite of śnu's text.

### `paradigm.rs` — the golden table

`PARADIGM` keeps its `(&str, &str, [&str; 9])` shape and its 1512 byte-identical
strings. Alternates go in a second table:

```rust
/// (root_id, lakara_label, cell index into the [&str; 9], alternate form).
const ALTERNATES: &[(&str, &str, usize, &str)] = &[
    ("hi", "laT", 7, "hinvaH"),
    ("hi", "laT", 8, "hinmaH"),
    ("hi", "laN", 7, "ahinva"),
    ("hi", "laN", 8, "ahinma"),
    ("ri", "laT", 7, "riRvaH"),
    ("ri", "laT", 8, "riRmaH"),
    ("ri", "laN", 7, "ariRva"),
    ("ri", "laN", 8, "ariRma"),
];
```

Cell indices 7 and 8 are uttama dvi and uttama bahu — `PARADIGM`'s cell order
is `[P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B]`. The forms these pair with
in `PARADIGM` are `hinuvaH` / `hinumaH` / `ahinuva` / `ahinuma` and `riRuvaH` /
`riRumaH` / `ariRuva` / `ariRuma`.

Widening every cell to `[&[&str]; 9]` was considered and rejected: it is
conceptually cleaner (one table, saying what the grammar produces, full stop)
but rewrites all 168 blocks and 1512 strings into `&["Bavati"]` shape — an
enormous mechanical diff over the exact suite 5a just froze, which is the
opposite of what the 5a/5b split is for.

The existing `every_form_validates_and_matches` asserts each pinned form is
*derivable*; it never asserts it is the *only* form. That asymmetry is what
lets alternates land without touching existing strings, and it is also a hole:
nothing currently catches an accidental fork. A new
`derivation_set_is_exactly_pinned` closes it — for all 1512 cells, collect the
non-blocked branch texts and assert the set equals
`{PARADIGM form} ∪ {that cell's ALTERNATES}`. An over-firing 6.4.107 fails; a
stale `ALTERNATES` row fails too.

Alternates additionally go through `check` with the same `(dhatu, pada,
lakāra)` resolution `every_form_validates_and_matches` uses — `PARADIGM`'s
first column is a `Dhatu::id`, resolved against `Analysis::dhatu`'s `code`.

`"hinmaH"` leaves `known_nonforms_are_invalid`, and six over-firing pins join
it:

| non-form | what it would mean |
|---|---|
| `ApnvaH` | fired on a conjunct root (real form `ApnuvaH`) |
| `ApnmaH` | same, bahu (real form `ApnumaH`) |
| `aSnvahe` | fired in the ātmanepada conjunct column (real form `aSnuvahe`) |
| `hinTaH` | fired on an ending that is not m/v-initial (real form `hinuTaH`) |
| `hinyAma` | `starts_with` mistaken for `contains` (real form `hinuyAma`) |
| `BavmaH` | fired where the vikaraṇa is not śnu (real form `BavAmaH`) |

### `trace.rs` — pinning the fork as a fork

The strongest available assertion is differential: **`hinuvaH`'s trace equals
`hinvaH`'s trace with the 6.4.107 step removed.** That pins a shared prefix and
one divergent step, rather than two independently plausible traces that happen
both to be listed.

### `roundtrip.rs`

Iterate all branches per cell.

### Mutation testing

`mise run mutants` after both commits. The new controller logic is prime
territory: the keep-the-clone-only-if-it-fired condition and the insertion
index are both single-token mutations that most goldens would not notice.

## Documentation

- **`AGENTS.md`** — the `vikalpa` convention (a new optional rule sets the flag
  and adds its id to `tinanta_rule_order_is_pinned` in position, as any rule
  does), and the ordering caveat: an optional rule must be ordered after every
  consumer of a predicate its own mutation invalidates.
- **`docs/ARCHITECTURE.md`** — 6.4.107 in the `adesha` stage row; a short
  section on the fork and on `derive` returning a set.
- **`README.md`** — the Scope paragraph gains a sentence: a cell may have more
  than one valid form, and `check` reports every derivation.

## Slice split

| | content | forms | ids |
|---|---|---|---|
| **prep** | `Rule.vikalpa`, worklist `run_pipeline`, `derive` → `Vec<Prakriya>`, `sole()`, all call sites, controller tests | 1512 → 1512 | 67 → 67 |
| **5b** | 6.4.107, `ALTERNATES`, exhaustiveness test, negative pins, trace tests, docs | 1512 → 1512 (+8) | 67 → 68 |

Prep is behaviour-preserving and is verified the way the svādi preps were: all
1512 forms and traces byte-identical, rule order unchanged. It introduces no
grammar and should not be reviewed as though it did.

Declined-first branch ordering buys prep one verification lever the svādi preps
did not have: **index 0 of every returned vec equals the old single
`Prakriya`**, in every cell. "No behaviour changed" is checkable, not asserted.

During prep, the "exactly one `vikalpa` rule" test asserts *zero*.

## Risks

1. **Branch count is 2^k and the controller does not cap it.** With k = 1 the
   maximum is two branches. An explosion would surface as a failing set
   comparison in `derivation_set_is_exactly_pinned`, not as a hang. A cap is
   YAGNI until a slice lands enough optional rules to need one.
2. **The 67-line `vikalpa: false` edit is mechanical, and its blast radius is a
   silently doubled branch.** Mitigated by the "exactly one `vikalpa` rule"
   test, which pins both the count and the id.
3. **`shnu_asamyogapurva` returns false on the elided branch.** Correct today
   because all three consumers precede 6.4.107, and unenforceable by the
   compiler. Recorded in `AGENTS.md`; see "The ordering constraint" above.
4. **Performance: 2× derivation on 8 of 1512 cells.** Negligible, and noted
   only because the FST/index layer of Phase 2 will have to decide what a
   precomputed table does with a cell that has two forms.
