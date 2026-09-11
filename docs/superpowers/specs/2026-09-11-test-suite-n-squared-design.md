# Removing the test suite's N² — design

*2026-09-11*

## 1. The problem, measured

`crates/panini-analyze/src/lib.rs:28` builds the full candidate cross-product
and **discards its argument**:

```rust
pub fn candidates(surface_slp1: &str) -> Vec<Candidate> {
    let mut out = Vec::new();
    for d in dhatus() { for &lakara in LAKARAS { for &(purusha, vacana) in CELLS {
        for &pada in d.pada.padas() { out.push(Candidate { .. }); } } } }
    let _ = surface_slp1;   // never read
    out
}
```

So every `Panini::check()` re-derives the entire corpus — 82 dhātus × 4 lakāras
× 9 cells × padas ≈ 3636 candidates, each fully derived through
`derive_prakriya`. A single `check()` costs **~0.30 s** in the dev profile
(measured 2026-09-11 via `target/debug/panini check`: 333, 303, 302, 297,
292 ms for `Bavati`, `paWati`, `aBavat`, `karoti`, `juhoti`).

The golden suite calls `check()` once per form, so it is **Θ(N²) in corpus
size**. Slice 3b recorded the floor as three components — which are the three
*test binaries*, not individual test functions:

| binary | `check()` calls | recorded | implied per call |
| --- | --- | --- | --- |
| `paradigm` | 3636 goldens + 959 alternates + ~25 point tests ≈ 4620 | 1122.81 s | 0.243 s |
| `roundtrip` | 4595 forms (one per derived branch) | 1354.58 s | 0.295 s |
| `trace` | ~15 | 4.07 s | — |
| **total** | **~9230** | **2481.46 s** | |

The two implied per-call figures bracket the 0.30 s measured through the CLI
(which carries process startup the in-process calls do not). They differ from
each other by 21%, which this spec does not attempt to explain — the mechanism
is certain and the magnitude is settled, but **the exact per-binary split is
not reconciled**, and §8 projects a range rather than a point because of it.

The load is spread across **three** hot loops, not the two the earlier note
named: `roundtrip.rs:16`, `every_form_validates_and_matches`
(`paradigm/main.rs:73`), and `every_alternate_validates_and_matches`
(`paradigm/main.rs:99`, 959 rows — roughly a quarter of the paradigm binary).

This also explains why the floor outran cell-count scaling every slice: 2628 →
3636 cells is 1.38×, but 943.70 s → 2483 s is 2.63× — superlinear, because the
per-call cost grows with the corpus at the same time as the call count does.

The consequence reaches past the suite. `cargo-mutants` re-runs the whole
workspace suite per mutant, so the N² multiplies through every campaign: slice
3b's was 692 mutants at `-j 4`, projected ~37 h.

## 2. Scope

**In scope — test harness only.** Remove the N² from the three hot loops by
deriving the corpus once per test binary and asserting against that index.

**Explicit non-goals**, each considered and declined:

- **Fixing `candidates()` to narrow by surface.** The honest analyzer fix, but
  any over-narrowing is a silent false negative and sandhi (ṣatva, luk, dhi)
  makes ending-matching genuinely unsound in places. Out of scope here; the
  exhaustive roundtrip preserved in §5 is precisely the gate a future
  narrowing slice would need.
- **Caching the index inside `Panini`.** Would fix the CLI too, but makes
  `Panini::new()`/first `check()` pay a full corpus derive — a regression for
  the one-word CLI invocation that is the CLI's whole use case.
- **`[profile.test] opt-level = 2`.** See §6: against an O(N) suite this is a
  net loss, not a win.

`check()` therefore remains ~0.30 s per call for CLI users. That is a known,
deliberate limitation of this slice, to be recorded as such in `AGENTS.md`
rather than left to read as an oversight.

## 3. The corpus index

New `crates/panini/tests/common/index.rs`, reached by both integration binaries
through the existing `common` module — directly from `roundtrip.rs`, and via
`#[path = "../common/mod.rs"]` from `paradigm/main.rs`.

```rust
pub struct FormIndex { /* HashMap<String, Vec<IndexedAnalysis>> */ }

pub struct IndexedAnalysis {
    pub dhatu: String,     // Dhatu::code, as `Analysis::dhatu` reports it
    pub lakara: Lakara,
    pub pada: Pada,
    pub purusha: Purusha,
    pub vacana: Vacana,
}

/// One full cross-product derivation, built once per test binary.
pub fn corpus_index() -> &'static FormIndex;

impl FormIndex {
    /// Normalizes `form` the way `check()` does before looking it up, so a
    /// caller cannot accidentally bypass scheme detection.
    pub fn analyses(&self, form: &str) -> &[IndexedAnalysis];
}
```

Built behind a `LazyLock` by mirroring `check()`'s predicate exactly:
iterate `candidates("")`, call `derive_prakriya`, keep branches where
`!p.blocked`, key on `p.text()`. Cost: one `check()`'s worth of work
(~0.30 s) for the entire binary.

**`IndexedAnalysis` deliberately omits `trace: Vec<RuleStep>`.** Carrying every
branch's rule log would balloon the index for a comparison the `tests/trace/`
binary already owns. The drift gate in §5 therefore reconciles every field
*except* traces.

The map is private; `analyses()` is the only way in, which is what keeps the
normalization guarantee above enforceable rather than conventional.

## 4. Converting the loops

`every_form_validates_and_matches` and `every_alternate_validates_and_matches`
keep their assertions **verbatim**, reading `corpus_index().analyses(form)` where
they read `engine.check(form).analyses`. The `Verdict::Valid` assertion becomes
non-empty membership.

**The rule is: convert the loops, leave the point tests alone.** Every other
`check()` call site stays on the real engine — `known_nonforms_are_invalid`
(~20 forms, ~6 s), `both_ash_roots_derive`,
`pada_ambiguous_surfaces_are_exactly_these`, and `tests/trace/helpers.rs`.
Together they are under 25 calls. `known_nonforms_are_invalid` in particular
**must not** be converted: it is the only test of `check()`'s negative path,
and "absent from the index" is not an honest rendering of "the engine rejects
this".

## 5. The roundtrip: sampled, exhaustive, and the drift gate

`roundtrip.rs` becomes one parameterized body with two entry points, so the
sampled and exhaustive versions cannot drift apart — only the iterator differs:

```rust
fn roundtrip_over(cells: impl Iterator<Item = (&'static Dhatu, Lakara, Pada, Purusha, Vacana)>);

#[test] fn roundtrip_sampled() { roundtrip_over(sample()) }

#[test]
#[ignore = "exhaustive; run via `mise run test-full`"]
fn roundtrip_exhaustive() { roundtrip_over(full_cross_product()) }
```

`sample()` rotates deterministically over `dhatus()`: for root *i*, take
`LAKARAS[i % 4]`, `CELLS[i % 9]`, and `d.pada.padas()[i % padas.len()]`. Since
`lcm(4, 9) = 36` and there are 82 roots, `(i % 4, i % 9)` is `i % 36`, so
**every lakāra × cell pair appears at least twice** while every root appears
exactly once. No RNG and no seed to record.

The sample grows linearly with the corpus — a new root adds itself. This is the
property that stops the floor from going quadratic again as gaṇas land.

Each sampled cell does two jobs, which is why the sample earns its wall clock:

1. **The original assertion**, unchanged in semantics: derive, then real
   `engine.check(form)` recovers `d.code`, `form`, and `lakara`.
2. **The drift gate**: assert `corpus_index().analyses(form)` equals
   `check(form).analyses` as a set, on the five non-trace fields.

Job 2 is what makes §3 safe. `index.rs` duplicates ~8 lines of `check()`, and
without reconciliation a change to `check()` would silently stop the paradigm
suite from testing the engine. The index is never trusted on its own
authority — it is continuously checked against the thing it stands in for,
across every root.

## 6. Tiering and wiring

- `mise run test` — unchanged command, now the blocking tier.
- **New** `mise run test-full` — `cargo test --workspace -- --include-ignored`.
- `.github/workflows/ci.yml` already carries a `schedule:` cron (Mondays 06:00
  UTC) for the audit job. A `test-full` job joins it.
- `AGENTS.md`'s slice checklist gains: **run `mise run test-full` once per
  slice, before the mutation gate.**

**`[profile.test] opt-level = 2` is explicitly declined.** Against the N²
suite it was the best value-to-risk change available (~5× on a 2483 s floor).
Against an O(N) suite the cost model inverts: it would save ~33 s of test phase
while making *every per-mutant rebuild* slower, and `cargo-mutants` rebuilds per
mutant. What was recommendation #1 is now a net loss.

## 7. The mutation gate

`--timeout 4800` must come down, and not only for speed. The known-permanent
`tripadi.rs` `j /= 1` hang (a mutant that never terminates, whose correct and
permanent verdict is a timeout) is detected *by* the cap — at 4800 s that one
mutant costs 80 minutes of every campaign.

`AGENTS.md`'s standing rule is that the cap must clear a full **uncaught** run
at the parallelism actually used. Uncontended that becomes ~33–39 s; the repo's own
measured `-j 4` contention factor of 2.1–2.5× puts a contended uncaught run near
70–98 s, pointing at **~600 s**. But this repo's discipline is "scale the floor
by measurement, not by arithmetic," so **600 s is the expected landing spot, not
the decision**: the branch takes a fresh `-j 4` measurement at 3636 cells and
sets `mise.toml`'s default against it.

## 8. Acceptance criteria

A green suite proves nothing here — the change deletes ~26M derivations of
assertion pressure, and the question is whether that pressure was load-bearing.

1. **Mutation-coverage diff, the real gate.** Run the full campaign against the
   new suite and diff caught/missed against slice 3b's recorded outcomes. **Any
   mutant caught before and missed now is a blocker**, resolved by widening the
   sample or adding a pin — never by accepting the loss.
   Operationally: copy 3b's `outcomes.json` aside and pass `-o` first, since
   every `cargo mutants` invocation rotates `mutants.out` → `mutants.out.old`.
2. `mise run test` green, with its wall clock measured and recorded.
3. `mise run test-full` green — the exhaustive roundtrip must still pass.
4. `mise run lint`, `mise run fmt-check` green.

Projected blocking floor, at the 0.243–0.295 s per-call band from §1: the
82-root sample yields ~103 forms (the ~1.26 branches-per-cell fan-out of 4595
forms / 3636 cells), so roundtrip lands at 25–30 s; `known_nonforms_are_invalid`
adds 5–6 s; two index builds and the trace binary add ~3 s. **Total 33–39 s**,
against 2481 s — a 60–75× reduction. The range is the honest form of this
estimate given §1's unreconciled split; the measured figure replaces it in
`AGENTS.md`.

## 9. The floor series discontinuity

`AGENTS.md` tracks a per-slice floor measurement as a continuing series, and
every prior entry was taken at the current config. This branch re-bases it, and
the discontinuity must be **recorded explicitly, not slipped in**: both the old
and the new floor measured at the same 3636 cells, so the series has a
documented hinge rather than an unexplained cliff.

`AGENTS.md`'s `cargo-mutants` paragraph is the single source for this whole
body of reasoning, and most of its careful timeout arithmetic describes a floor
that will no longer exist. It needs a **rewrite**, not an append — preserving
the two findings that survive (the reclassification hazard, and the permanent
`tripadi.rs` timeout) while retiring the figures that the N² produced.

## 10. Files touched

| file | change |
| --- | --- |
| `crates/panini/tests/common/index.rs` | new — the corpus index |
| `crates/panini/tests/common/mod.rs` | re-export `index` |
| `crates/panini/tests/paradigm/main.rs` | two loops read the index; point tests unchanged |
| `crates/panini/tests/roundtrip.rs` | parameterized body, sampled + `#[ignore]`d exhaustive, drift gate |
| `mise.toml` | new `test-full` task; `mutants` `--timeout` re-derived |
| `.github/workflows/ci.yml` | `test-full` job on the existing weekly cron |
| `AGENTS.md` | `cargo-mutants` paragraph rewritten; floor discontinuity; slice checklist; `check()`'s cost noted as deliberate |
