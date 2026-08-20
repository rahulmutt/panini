# rudhādi (gaṇa 7) — 8.2.30 *coḥ kuḥ* generalised, and √ric and √vic

Three documents defer √ric and √vic to the same sentence, and the sentence is
right about the cause and careful about the cost:

> **√ric and √vic** need no new sūtra, but the work in 8.2.30 *coḥ kuḥ* is more
> than the one-line guard widening it looks like: they are c-final, and the rule
> is hardcoded to a single `j` → `g` pair — its match reads `j` alone AND its
> substitute is a literal `'g'`, while its comment claims a 1.1.50
> *sthāne'ntaratamaḥ* nearest-velar substitution (voicing and aspiration
> preserved) that the code does not implement.

The rule's own comment says the same thing about itself, and says what to do:

> NARROW GUARD, by design […] the pair is hardcoded at both ends — the match
> tests `j` and the substitute is the literal `'g'`. The 1.1.50 nearest-velar
> account above is therefore a description of the sūtra, NOT of this code. When
> a `c`-tailed root lands (√ric, √vic), widen the match AND replace the
> hardcoded substitute with a real cu -> ku map; widening only the match would
> substitute `g` for a `c` and reach the right surface by accident, 8.4.55
> khari ca devoicing it to `k` afterwards.

This slice does exactly that, and then curates the two roots the fix unlocks.

The defect is worth naming precisely, because its shape is unusual: **the code
is correct today and would remain correct-by-surface under the lazy fix.** With
the match widened and the substitute left literal, √ric would derive `riRagti`
and 8.4.55 *khari ca* would devoice the `g` to `k`, yielding `riRakti` — the
right form, through a `g` that 8.2.30 has no business producing from a `c`.
Every paradigm golden would pass. Only a trace can tell the two implementations
apart, which is why this spec treats one trace test as the slice's load-bearing
assertion rather than as coverage.

## Scope

New: `kutva_of` in `panini-prakriya`'s `sound.rs` with its all-arms unit test;
two `Dhatu` rows; 16 `PARADIGM` blocks; whatever `ALTERNATES` rows the new cells
fork into; trace pins for √ric and √vic.

Changed: 8.2.30's `apply` and the comment block above it, both in
`tinanta/tripadi.rs`; the corpus-total assertions in `panini-data`,
`crates/panini/tests/paradigm.rs` and `tools/audit/panini_full_audit.rs`; and
the six documentation sites enumerated under "The doc claims this slice
falsifies".

No new sūtra. No `Rule` added, removed or reordered; no change to
`TINANTA_RULES` or its pinned order; no change to any other guard. The vikalpa
set stays at seven. 8.2.30 itself remains a `RuleKind::Vidhi`, non-vikalpa, in
the same position in the tripādī array.

Root count goes 53 → **55**. rudhādi goes 11 → **13** of its 25 dhātupāṭha
entries. `PARADIGM` goes 240 → **256** root×pada×lakāra blocks (46 single-pada
roots × 4, plus 9 ubhayapadī × 8), i.e. 2160 → **2304** cells. The form total is
not predicted here; it is read off the audit.

Out of scope, deferred — carried forward unchanged:

- **6.1.73 *che ca* and 8.4.40 *stoḥ ścunā ścuḥ***, and with them √chid and
  √chṛd. These are two sūtras the engine does not implement, not a
  generalisation of one it does; the distinction is what keeps them a separate
  slice.
- **The nine reachable non-ubhayapadī rudhādi roots** — √śiṣ, √tṛh, √und, √añj,
  √tañc, √vij, √vṛj, √pṛc, √vid.
- **√bhuj** (`07.0017`), whose 1.3.66 *bhujo'navane* forks its pada on sense
  rather than on an axis this engine models.
- **It-stripping as real sūtras** (1.3.2, 1.3.3, 1.3.5, 1.3.9). This slice adds
  two more curated rows and so makes the case slightly stronger without
  pre-empting the design.
- **8.2.39 *jhalāṁ jaśo'nte*'s own narrow guard**, and every other narrow guard
  in `tripadi.rs`. 8.2.30 is generalised here because this slice's roots demand
  it, not because narrow guards are being retired as a class. Nothing else is
  touched.

After this slice, **12 of rudhādi's 25 entries remain out**.

## The two roots

| dhātupāṭha | upadeśa | `code` | artha |
|---|---|---|---|
| `07.0004` | `ri\ci~^r` | `ric` | `virecane` |
| `07.0005` | `vi\ci~^r` | `vic` | `pfTagBAve` |

### Both are ubhayapadī, and the column is derived rather than asserted

Each carries the `~^` svarita it that 1.3.72 *svaritañitaḥ kartrabhiprāye
kriyāphale* reads, and neither carries a trailing `~\` anudātta it for 1.3.12
*anudāttaṅita ātmanepadam* to pre-empt it with — the distinction the pada audit
established when it separated √rudh (`ru\Di~^r`, ubhayapadī) from √indh
(`YiinDI~\`, ātmanepada-only despite its ñi). The `\` inside each entry is the
root vowel's own accent, not an it: the accent attaches *after* the `~` that
marks an it.

So both rows are `PadaAssignment::Ubhayapada`, and
`curated_pada_agrees_with_upadesha_markers` in `panini-data` re-derives both
verdicts from the vendored upadeśa the moment the rows land. The pada column
cannot be wrong here without a test failing.

Neither `code` collides. `ric` and `vic` are each new to the curated set.
rudhādi's own `o~vijI~` (`07.0023`) is a different root from tudādi's √vij and
is not curated; under number keying the question does not arise in any case.

### What they witness

**√ric is the first `c` ever to reach 8.2.30**, and the reason the substitute
has to become a map. Its laṭ parasmaipada prathama eka is expected to derive
`riRakti`: śnam infixes to give `ri | nac | ti`, 8.4.2
*aṭkupvāṅnumvyavāye'pi* applies ṇatva across the intervening `i` (trigger `r`,
target śnam's `n`), and 8.2.30 then replaces the `c` before the jhal `t` with
its nearest velar `k`. 8.4.55 *khari ca* has nothing left to do — `k` is
already voiceless — which is precisely the contrast with √bhañj, where 8.2.30's
`g` and 8.4.55's devoicing are two visible steps.

**√vic is the minimal contrast.** Same gaṇa, same c-final shape, same vikaraṇa,
same 8.2.30 application — and no ṇatva trigger at all, so its laṭ parasmaipada
prathama eka is expected to derive `vinakti` with a dental `n` intact. The pair
isolates 8.4.2 against a controlled background, the way slice 7c used √kṣud and
√tṛd to separate 8.4.2 from 8.4.1.

Expected laṭ prathama eka, both padas:

| root | parasmaipada | ātmanepada |
|---|---|---|
| √ric | `riRakti` | `riNkte` |
| √vic | `vinakti` | `viNkte` |

These are the spec's *expectations*, not its pins. The plan transcribes the
engine's actual output after the audit certifies it, never the other way round.

## What changes

### `kutva_of` in `sound.rs`

A new substitution table beside `guna_of`, `vrddhi_of`, `cartva_of`,
`jashtva_of` and `parasavarna_of`, following their established shape — a
`fn(char) -> Option<char>` returning `None` off-domain:

```rust
/// The *ku* (velar) counterpart of a cu sound, by 1.1.50 sthāne'ntaratamaḥ:
/// the nearest substitute preserves voicing and aspiration, so `c → k` but
/// `j → g`. 8.2.30 coH kuH's substitute.
///
/// `C` and `J` have no curated witness — no rudhādi root in scope is
/// aspirate-cu-final — and are present anyway because the table is a total
/// function of place, the same reason `jashtva_of` carries its 1.1.50-derived
/// `z → q` arm. `kutva_of_cu_all_arms` is what keeps them from rotting.
pub(crate) fn kutva_of(c: char) -> Option<char> {
    Some(match c {
        'c' => 'k',
        'C' => 'K',
        'j' => 'g',
        'J' => 'G',
        _ => return None,
    })
}
```

**Why `sound.rs` and not inline in the rule.** Three reasons, and the third is
the one that decides it. It puts the table where every sibling substitution
table already lives. It gives the match and the substitute a single source of
truth. And it puts the four `delete match arm` mutants inside
`panini-prakriya`'s own unit tests — the package cargo-mutants uses for its
baseline — where `kutva_of_cu_all_arms()` kills them against a ~2s suite
instead of the ~600s golden suite. Inline arms would have had to be killed
through derivation, at full suite cost, and the `C` and `J` arms could not have
been killed at all.

The test joins `jashtva_of_stops_all_arms` and `parasavarna_of_stops_all_arms`
in `sound.rs`'s own test module, asserting all four arms and at least one
off-domain `None`.

### 8.2.30 reads `kutva_of` on both sides

The search predicate widens from a literal `'j'` to "any sound `kutva_of`
knows", and the substitute becomes the value that same lookup returns for the
char actually found — so `w[pos]`'s third element stops being discarded:

```rust
apply: |p| {
    let w = word_chars(p);
    let Some(pos) = w.iter().enumerate().position(|(i, (_, _, c))| {
        kutva_of(*c).is_some() && w.get(i + 1).is_none_or(|(_, _, next)| is_jhal(*next))
    }) else {
        return false;
    };
    let (term, idx, found) = w[pos];
    let Some(to) = kutva_of(found) else {
        return false;
    };
    let before = p.snapshot();
    set_char(p, term, idx, to);
    p.record("8.2.30", "coH kuH", before);
    true
},
```

Three properties of the existing rule are preserved deliberately, and the plan
must not let a rewrite drop them:

1. **The word-final-or-jhal test stays inside the search.** The rule finds the
   first cu that genuinely qualifies, not the first cu full stop, so a
   non-applicable cu earlier in the word can never hide a later applicable one.
2. **The scan still reads `word_chars`, not term boundaries.** śnam's infix
   leaves the root's own tail in `SHAP`, one term short of the actual word end
   (`ri | nac | ti`), so the conditioning jhal is often the first character of
   the *next* term. Word-final falls out of the same scan for free.
3. **The search is not narrowed to a known position.** This is hardening
   against an ordering no witness here exercises, and it stays that way.

`kutva_of` joins the `use crate::tinanta::sound::{...}` list at the top of
`tripadi.rs`.

The `kutva_of(found)` re-lookup is redundant with the predicate by construction.
It is written as a fallible binding rather than an `unwrap` because an `unwrap`
in a tripādī rule would be the only one in the file, and because a mutant that
breaks the correspondence should decline rather than panic.

### The comment block above 8.2.30 is rewritten, not patched

Its "NARROW GUARD, by design" paragraph exists to describe the defect this
slice removes, and leaving it in place would leave the file lying about itself.
Two claims in it need positive restatement rather than deletion:

- **The 1.1.50 account is now a description of the code**, not only of the
  sūtra. That sentence inverts.
- **"No cell in this suite has two `j`s to distinguish"** must be restated over
  cu sounds generally: √ric and √vic each carry exactly one `c`, no curated root
  mixes a `c` with a `j`, and the j-bearing roots that decline (√ji, √juṣ, √vij,
  and √bhañj's own 3pl `Banjanti`) decline for the unchanged reason that their
  `j` precedes a vowel.

The rationale for why the substitute is a lookup — *widening only the match
would reach the right surface through a wrong intermediate* — moves into this
comment from the deferral notes that currently carry it. It must survive
somewhere in the code, or a later contributor is free to "simplify" `kutva_of`
back into a hardcoded char with every surface still passing.

### The data rows

Two `Dhatu` rows in `panini-data`, transcribed from `data/dhatupatha.tsv`
verbatim, both `Gana::Rudhadi` and both `PadaAssignment::Ubhayapada`, each with
a comment naming what it witnesses in the house style — √ric as 8.2.30's first
`c` and an 8.4.2 ṇatva witness, √vic as the no-trigger contrast.

`rudhadi_rows_are_the_eleven_curated_roots` becomes
`rudhadi_rows_are_the_thirteen_curated_roots` and takes the two new rows in
dhātupāṭha order. `dhatus().len()` goes to 55.

### The golden tables

16 new `PARADIGM` blocks — 2 roots × 2 padas × 4 lakāras — plus whatever
`ALTERNATES` rows the new cells fork into. By √bhañj's precedent the word-final
laṅ cells are expected to fork under 8.2.39 *jhalāṁ jaśo'nte* and 8.4.56
*vā'vasāne* (as `aBanag` / `aBanak` does), and loṭ madhyama eka under *hi* /
*tāt*. The actual fork set is whatever the engine produces and the audit
certifies.

`derivation_set_shape_matches_the_audited_numbers` in
`crates/panini/tests/paradigm.rs` takes the new totals, as does the
`assert_eq!` triple at `tools/audit/panini_full_audit.rs:577-579`.

### Trace pins, and the one that carries the slice

Two trace tests pin the ṇatva contrast: √ric's `riRakti` showing 8.4.2 applying
across the intervening vowel, and √vic's `vinakti` showing no ṇatva rule in the
trace at all. The second is a negative pin and is worth having for the same
reason 7c pinned its two arms per root rather than per surface.

The third is the slice's load-bearing assertion. Alongside the existing
`bhanakti_trace_shows_8_2_30_then_8_4_55`, which pins √bhañj's two visible steps
(8.2.30 gives `g`, 8.4.55 devoices it to `k`), this slice pins that √ric reaches
`k` in **one** step — 8.2.30 substituting the nearest velar of a voiceless `c`
directly, with 8.4.55 finding nothing to devoice.

That test is the only thing in the suite that distinguishes a correct 8.2.30
from an accidentally-correct one. Under the lazy fix — match widened, substitute
left literal — every `PARADIGM` cell in this slice still passes, because 8.4.55
launders the wrong `g` into the right `k`. The trace does not launder. Whether
8.4.55 declines outright or applies vacuously through its existing no-op guard
is a fact to read off the run and pin, not to assert here.

### The cross-pada ambiguity test

`pada_ambiguous_surfaces_are_exactly_these` walks `PARADIGM` and asserts the
exact set of surfaces that are pinned cells in both padas at once. Two new
ubhayapadī roots will add whatever collisions they add — the test self-maintains
and needs no editing. `README.md`'s hand-written enumeration of those surfaces
does not self-maintain and must be updated from the test's own failure output.

As established in 7c, an *alternate* form that is pada-ambiguous in its own
right stays out of scope for that test by design: `ALTERNATES` is not walked.

## Testing

### The audit runs after the code, and blocks the goldens

This inverts 7c's ordering, and the inversion is forced. 7c audited before
pinning anything because no code changed and its four roots already derived.
Here the roots do not derive at all until both the rule change and the rows
land. The sequence is therefore:

1. `kutva_of` + its unit test + 8.2.30's rewrite. The existing suite is the
   regression gate: √bhañj and √yuj exercise the `j` path and must be
   byte-identical, and no curated root carries a `c` today, so a green suite
   here means the widening changed nothing it should not have.

   That last claim is checkable and was checked when this spec was written, so
   the plan need not re-derive it: the only curated roots carrying a cu sound
   are √ji, √juṣ, √vij, √bhañj and √yuj, and **all five carry `j`, none
   carries `c`**. Nor can a `c` arise mid-derivation before this rule: the only
   rule in the engine that produces one is 8.4.55 *khari ca* via `cartva_of`,
   and the three tripādī rules ordered ahead of 8.2.30 — 8.2.77, 8.2.23 and
   8.2.25 — produce none. So widening the match adds **no new match site
   anywhere in the existing corpus**, and the `j` path is bit-for-bit the
   behaviour it had before.
2. The two `Dhatu` rows and the count updates.
3. **The cross-implementation audit**, whole corpus.
4. Goldens transcribed from audited output.

Nothing is pinned from an expectation. The expectations in this document exist
to make a wrong audit result recognisable, not to be copied into a test.

### Running the audit

Per `tools/audit/README.md`, unchanged: copy the committed
`tools/audit/panini_full_audit.rs` into a `vidyut` checkout at the commit
recorded in `data/dhatupatha.tsv`'s own header — **copy it, do not rewrite it**
— add this repo's crates as dev-dependencies there, and run it with an
explicitly named toolchain, since the vidyut checkout has no `mise.toml`.

The `entry` negative control must be **verified failing first** (exit 1, 36 √bhū
cells flagged). A zero-difference result recorded without a control proves
nothing, and this repo has already shipped one unsourced byte-identity claim
that took three slices and a dedicated slice to source.

Note that the harness asserts the corpus totals rather than reporting them, so
step 2's count updates must include `panini_full_audit.rs` or the audit will
fail before it compares anything. That is the intended behaviour: the harness is
wrong if it disagrees with the repo.

### Mutation gate

Run through `mise run mutants`, which is now `-j 4 --timeout 2400`; do not
reconstruct the flags by hand. `cargo-mutants` also reads `-j` from
`CARGO_MUTANTS_JOBS`, so the environment can defeat an unqualified cap.

**Re-measure the uncontended floor; do not scale it.** Cell count has failed as
a multiplier in both directions — flat from 1800 to 1872 cells, then +38% for
+15% growth into slice 7c's 610.73s (paradigm 276.99s, roundtrip 331.81s, trace
1.93s). This slice adds 144 cells (+6.7%). Take a standalone `mise run test`
measurement after the goldens land and record it in `AGENTS.md`, as every prior
slice has.

**Check `timeout.txt` alongside `missed.txt`.** Exactly one timeout is expected
and is the correct permanent verdict at any cap: the ṇatva backward-scan mutant
in `tripadi.rs` that turns `j -= 1` into `j /= 1`, making the loop
non-terminating. **Identify it by that shape, not by its line number** — this
slice adds lines to `tripadi.rs` above it, so the `1140:23` recorded in
`AGENTS.md` will move, and a shifted line number must not be mistaken for a new
timeout.

Expect the campaign to grow from 522 mutants to roughly 530 — `kutva_of`
contributes two function-replacement mutants and four arm-deletion mutants, all
killed by `kutva_of_cu_all_arms` in the fast baseline suite — at 0 missed.

## The doc claims this slice falsifies

Six sites state the √ric/√vic deferral or the corpus totals. Each must be
updated in the same slice; past slices have shipped with counts stale in one
file, and the plan should treat these as a checklist rather than a sweep.

| site | what changes |
|---|---|
| `README.md` | rudhādi 11 → 13 roots; "14 of the 25 remain out" → 12; √ric/√vic drop out of the deferral prose; 53 → 55 roots; 2160 → 2304 cells and the new form total; ubhayapadī list 7 → 9; the pada-ambiguous surface enumeration |
| `docs/ARCHITECTURE.md` | the rudhādi paragraph; "nine of rudhādi's 25 roots are ubhayapadī, five of the nine now curated" → seven of nine; the whole 8.2.30 passage, which currently reads as a pending caveat |
| `AGENTS.md` | the rudhādi section; suite-size figures; the recorded audit result; the new mutation floor and campaign numbers |
| `crates/panini-data/src/lib.rs` | the rudhādi deferral comment; `rudhadi_rows_are_the_eleven_curated_roots` → thirteen; `dhatus().len()` |
| `tools/audit/README.md` | "Last recorded result" |
| `crates/panini/tests/paradigm.rs` | the doc comments carrying the audited numbers, and `derivation_set_shape_matches_the_audited_numbers` |

`data/ATTRIBUTION.md` records per-entry discrepancies against upstream for
`07.0010`, `07.0019` and `07.0012`. The plan should check whether `07.0004` and
`07.0005` need an entry there; if the vendored upadeśa matches upstream
unmodified, they do not.

## If the audit shows a difference

The 8.2.30 fix is necessary for these roots. It is not provably sufficient until
the audit runs, and unlike 7c the audit cannot run first. The posture is
therefore fixed in advance, so the slice does not become an open-ended chase:

**Ship what passes; defer the rest with a sourced note.** The rule change lands
regardless — it is correct on its own terms, with √bhañj and √yuj as its
unchanged regression witnesses and √ric or √vic as its `c` witness. Whichever
root is byte-identical gets curated. Any residual difference becomes a deferral
that names the actual sūtra at fault, measured rather than guessed, in the same
form the 6.1.73 / 8.4.40 deferral takes today.

What the slice explicitly does **not** do is expand to implement whatever the
audit turns up. If √ric or √vic needs 8.4.40 *stoḥ ścunā ścuḥ* or anything else
the engine lacks, that root joins the existing deferral and its sūtra is named
there. Counts, prose and totals then reflect whatever actually shipped — a
partial slice states its own partiality rather than carrying forward numbers it
did not reach.

## Ordering

1. `kutva_of` in `sound.rs`, with `kutva_of_cu_all_arms`.
2. 8.2.30 reads it on both sides; comment block rewritten. Existing suite green.
3. The two `Dhatu` rows; `rudhadi_rows_…` renamed and extended; `dhatus().len()`.
4. Corpus totals updated in `paradigm.rs` and `panini_full_audit.rs`.
5. Cross-implementation audit, negative control first. Record the result.
6. `PARADIGM` and `ALTERNATES` transcribed from audited output.
7. Trace pins, including the one-step-vs-two-step 8.2.30 contrast.
8. `mise run test` floor measurement; `mise run mutants`; check both
   `missed.txt` and `timeout.txt`.
9. Documentation sweep across the six sites.
