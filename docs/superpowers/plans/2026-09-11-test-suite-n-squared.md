# Removing the Test Suite's N² — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Cut the golden suite's wall clock from ~2481 s to ~35 s by deriving the corpus once per test binary instead of once per `check()` call, without losing mutation coverage.

**Architecture:** `Panini::check()` is Θ(corpus) because `candidates()` discards its argument and returns the full cross-product. The three hot loops call it once per form, making the suite Θ(N²). A new test-only `FormIndex` performs that cross-product derivation **once** per test binary; the three loops become lookups. A sampled real `check()` over every root stays in the blocking tier and doubles as the gate reconciling the index against `check()`; the exhaustive version survives behind `#[ignore]`.

**Tech Stack:** Rust 1.98.1 (mise-pinned), `cargo test`, `cargo-mutants` (via `MISE_ENV=dev`), GitHub Actions.

**Spec:** `docs/superpowers/specs/2026-09-11-test-suite-n-squared-design.md`

## Global Constraints

- Branch is `test-suite-n-squared`, already created off `main`. Do not branch again.
- **No new dependencies.** No rayon, no `[profile.test]`, no `[profile.dev]`. The spec declines all three in §6; adding one silently is a plan violation.
- **`check()` in `crates/panini/src/lib.rs` and `candidates()` in `crates/panini-analyze/src/lib.rs` must not be modified.** This slice is test-harness only (spec §2). A task that finds itself wanting to edit production code has hit a problem the plan did not anticipate — stop and report rather than widening scope.
- **Run every test command in the FOREGROUND with an explicit long timeout.** Backgrounding the suite and ending the turn orphans it. Tasks 2 and 6 run commands that take up to 45 minutes; pass `timeout: 3600000` (60 min) to the Bash tool.
- **Scope cargo commands with `mise exec`, not `mise run`.** `mise run test -- -p panini` does **not** scope to the package. Use `mise exec -- cargo test -p panini --test paradigm`.
- Every commit message ends with:
  `Claude-Session: https://claude.ai/code/session_01FzNcnmPnRppZFU1jfqsMna`
- `crates/panini/tests/common/mod.rs` carries `#![allow(dead_code)]`; it applies to child modules, so a new submodule needs no attribute of its own.
- The enums `Lakara`, `Pada`, `Purusha`, `Vacana` derive only `Debug, Clone, Copy, PartialEq, Eq` — **no `Hash`, no `Ord`.** Do not use them as `HashMap` keys or call `.sort()` on them. Where ordering is needed, sort on `format!("{:?}", ..)`.

---

### Task 1: The corpus index

**Files:**
- Create: `crates/panini/tests/common/index.rs`
- Modify: `crates/panini/tests/common/mod.rs` (add the `pub mod index;` declaration)
- Test: `crates/panini/tests/roundtrip.rs` (a temporary agreement test, absorbed by Task 4)

**Interfaces:**
- Consumes: `panini_analyze::candidates`, `panini_prakriya::derive`, `panini_lipi::normalize` — all already dependencies of the `panini` package and therefore available to its test targets.
- Produces: `common::index::corpus_index() -> &'static FormIndex`, `FormIndex::analyses(&self, form: &str) -> &[IndexedAnalysis]`, and the `IndexedAnalysis` struct with fields `dhatu: &'static str`, `lakara: Lakara`, `pada: Pada`, `purusha: Purusha`, `vacana: Vacana`. Tasks 2, 3 and 4 all rely on exactly these names.

- [ ] **Step 1: Write the failing test**

Append to `crates/panini/tests/roundtrip.rs`:

```rust
#[test]
fn index_agrees_with_check_on_known_forms() {
    let engine = Panini::new();
    let index = common::index::corpus_index();
    for form in ["Bavati", "paWati", "aBavat", "juhoti", "alaBata"] {
        let mut from_check: Vec<String> = engine
            .check(form)
            .analyses
            .iter()
            .map(|a| format!("{:?}", (a.dhatu.as_str(), a.lakara, a.pada, a.purusha, a.vacana)))
            .collect();
        let mut from_index: Vec<String> = index
            .analyses(form)
            .iter()
            .map(|a| format!("{:?}", (a.dhatu, a.lakara, a.pada, a.purusha, a.vacana)))
            .collect();
        from_check.sort();
        from_index.sort();
        assert_eq!(from_index, from_check, "index disagrees with check() for {form}");
    }
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `mise exec -- cargo test -p panini --test roundtrip index_agrees_with_check_on_known_forms`
Expected: FAIL to compile — `could not find 'index' in 'common'`.

- [ ] **Step 3: Write the implementation**

Create `crates/panini/tests/common/index.rs`:

```rust
//! The corpus derived once, for tests that would otherwise call
//! `Panini::check()` in a loop.
//!
//! `panini_analyze::candidates()` discards its argument and returns the full
//! cross-product, so every `check()` re-derives the entire corpus and a loop
//! over N forms costs N². This module does that derivation once per test
//! binary and answers lookups from the result.
//!
//! The build below mirrors `Panini::check()`'s predicate exactly — same
//! candidate set, same `!blocked` filter, same exact-match on `text()`. That
//! duplication is deliberate but load-bearing: `roundtrip_sampled` in
//! `roundtrip.rs` reconciles this index against the real `check()` on every
//! root, and is what makes trusting it safe. If you change `check()`, that
//! test is what will tell you this file needs the same change.

use std::collections::HashMap;
use std::sync::LazyLock;

use panini_analyze::candidates;
use panini_data::{Lakara, Pada, Purusha, Vacana};
use panini_lipi::normalize;
use panini_prakriya::derive as derive_prakriya;

/// One analysis of one form. Deliberately carries no `trace`: holding every
/// branch's rule log would balloon the index for a comparison the
/// `tests/trace/` binary already owns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexedAnalysis {
    /// `Dhatu::code`, matching what `Analysis::dhatu` reports — the
    /// user-facing spelling, deliberately not unique.
    pub dhatu: &'static str,
    pub lakara: Lakara,
    pub pada: Pada,
    pub purusha: Purusha,
    pub vacana: Vacana,
}

pub struct FormIndex {
    map: HashMap<String, Vec<IndexedAnalysis>>,
}

impl FormIndex {
    /// Every analysis of `form`, empty if the corpus derives no such form.
    /// Normalizes the way `check()` does, so a caller cannot accidentally
    /// bypass scheme detection by passing IAST or Devanagari.
    pub fn analyses(&self, form: &str) -> &[IndexedAnalysis] {
        let (slp1, _) = normalize(form);
        self.map.get(&slp1).map_or(&[][..], |v| v.as_slice())
    }
}

static INDEX: LazyLock<FormIndex> = LazyLock::new(|| {
    let mut map: HashMap<String, Vec<IndexedAnalysis>> = HashMap::new();
    for c in candidates("") {
        for p in derive_prakriya(c.dhatu, c.lakara, c.pada, c.purusha, c.vacana) {
            // Same guard as `check()`: a blocked prakriyā's text is a partial
            // string that can still collide with a genuine input.
            if p.blocked {
                continue;
            }
            map.entry(p.text()).or_default().push(IndexedAnalysis {
                dhatu: c.dhatu.code,
                lakara: c.lakara,
                pada: c.pada,
                purusha: c.purusha,
                vacana: c.vacana,
            });
        }
    }
    FormIndex { map }
});

/// The corpus index, derived on first use and shared for the rest of the
/// binary's life.
pub fn corpus_index() -> &'static FormIndex {
    &INDEX
}
```

Add to `crates/panini/tests/common/mod.rs`, immediately after the `#![allow(dead_code)]` line:

```rust
pub mod index;
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `mise exec -- cargo test -p panini --test roundtrip index_agrees_with_check_on_known_forms`
Expected: PASS, in roughly 2–4 s (one index build at ~0.3 s plus five real `check()` calls at ~0.25 s each).

- [ ] **Step 5: Commit**

```bash
git add crates/panini/tests/common/index.rs crates/panini/tests/common/mod.rs crates/panini/tests/roundtrip.rs
git commit -m "test: derive the corpus once into a FormIndex

candidates() discards its argument, so every check() re-derives the whole
corpus and a loop over N forms costs N squared. Derive it once per test
binary instead. Mirrors check()'s predicate deliberately; roundtrip_sampled
will reconcile the two.

Claude-Session: https://claude.ai/code/session_01FzNcnmPnRppZFU1jfqsMna"
```

---

### Task 2: Convert `every_form_validates_and_matches`

**Files:**
- Modify: `crates/panini/tests/paradigm/main.rs:55-88` (the `every_form_validates_and_matches` body)

**Interfaces:**
- Consumes: `common::index::corpus_index()` and `IndexedAnalysis` from Task 1.
- Produces: nothing new.

This is a behaviour-preserving conversion, so there is no new failing test to write. The gate is that the **same assertions still pass** and the wall clock collapses. Measure before and after; a conversion that passes instantly because it stopped asserting anything is the failure mode to rule out.

- [ ] **Step 1: Measure the binary before the change**

Run: `mise exec -- cargo test -p panini --test paradigm -- --nocapture`
Use a Bash tool `timeout` of `3600000` and run it in the FOREGROUND.
Expected: PASS in roughly 1100–1150 s. Record the reported time for `every_form_validates_and_matches`.

- [ ] **Step 2: Replace the body**

In `crates/panini/tests/paradigm/main.rs`, replace the body of `every_form_validates_and_matches` — keeping its existing doc comment and the long in-body comment about `dhatupatha` vs `code` resolution exactly as they are — with:

```rust
#[test]
fn every_form_validates_and_matches() {
    let index = common::index::corpus_index();
    for (root, lakara, row_pada, forms) in PARADIGM.iter() {
        let d = dhatus().iter().find(|d| d.dhatupatha == *root).unwrap();
        for expected in forms {
            let analyses = index.analyses(expected);
            assert!(
                !analyses.is_empty(),
                "expected VALID for {expected} ({root} {lakara})"
            );
            assert!(
                analyses.iter().any(|a| a.dhatu == d.code
                    && a.pada == *row_pada
                    && panini::lakara_name(a.lakara) == *lakara),
                "no {lakara} analysis of {root} produced {expected}"
            );
        }
    }
}
```

Note the dropped `a.form_slp1 == *expected` clause: `index.analyses(expected)` is keyed on the form, so every returned analysis is already an analysis *of that form*. The clause is not weakened, it is structural.

- [ ] **Step 3: Verify the assertions still bite**

Temporarily change `a.pada == *row_pada` to `a.pada != *row_pada`, run
`mise exec -- cargo test -p panini --test paradigm every_form_validates_and_matches`,
and confirm it FAILS with `no laT analysis of ... produced ...`. Then change it back.
This proves the converted test still asserts rather than trivially passing.

- [ ] **Step 4: Run the binary and confirm the drop**

Run: `mise exec -- cargo test -p panini --test paradigm`
Use a Bash tool `timeout` of `1800000` and run it in the FOREGROUND.
Expected: PASS. `every_form_validates_and_matches` drops from ~1100 s to under 1 s; the binary as a whole should now land near 250–300 s, still carrying the unconverted 959-call alternates loop from Task 3.

- [ ] **Step 5: Commit**

```bash
git add crates/panini/tests/paradigm/main.rs
git commit -m "test(paradigm): every_form_validates_and_matches reads the index

3636 check() calls at ~0.3s each become 3636 hashmap lookups. Assertions
unchanged except a.form_slp1 == expected, which the keyed lookup makes
structural.

Claude-Session: https://claude.ai/code/session_01FzNcnmPnRppZFU1jfqsMna"
```

---

### Task 3: Convert `every_alternate_validates_and_matches`

**Files:**
- Modify: `crates/panini/tests/paradigm/main.rs:94-112` (the `every_alternate_validates_and_matches` body)

**Interfaces:**
- Consumes: `common::index::corpus_index()` from Task 1.
- Produces: nothing new.

- [ ] **Step 1: Replace the body**

Keep the existing doc comment above the test verbatim. Replace the function with:

```rust
#[test]
fn every_alternate_validates_and_matches() {
    let index = common::index::corpus_index();
    for (root, lakara, row_pada, _cell, form, _key) in ALTERNATES.iter() {
        let d = dhatus().iter().find(|d| d.dhatupatha == *root).unwrap();
        let analyses = index.analyses(form);
        assert!(
            !analyses.is_empty(),
            "expected VALID for alternate {form} ({root} {lakara})"
        );
        assert!(
            analyses.iter().any(|a| a.dhatu == d.code
                && a.pada == *row_pada
                && panini::lakara_name(a.lakara) == *lakara),
            "no {lakara} analysis of {root} produced alternate {form}"
        );
    }
}
```

- [ ] **Step 2: Verify the assertions still bite**

Temporarily change `a.dhatu == d.code` to `a.dhatu != d.code`, run
`mise exec -- cargo test -p panini --test paradigm every_alternate_validates_and_matches`,
and confirm it FAILS with `no ... analysis of ... produced alternate ...`. Then change it back.

- [ ] **Step 3: Run the whole paradigm binary**

Run: `mise exec -- cargo test -p panini --test paradigm`
Use a Bash tool `timeout` of `1800000` and run it in the FOREGROUND.
Expected: PASS in roughly 6–10 s. The remaining cost is one index build (~0.3 s) plus `known_nonforms_are_invalid`'s ~20 real `check()` calls (~5–6 s), which stay on the real engine deliberately.

- [ ] **Step 4: Confirm the point tests were left alone**

Run: `grep -n "engine.check(" crates/panini/tests/paradigm/main.rs`
Expected: exactly two hits — one in `known_nonforms_are_invalid` and one in `both_ash_roots_derive`. (`pada_ambiguous_surfaces_are_exactly_these` touches no engine at all — it computes its answer from the `PARADIGM` tables alone.) If a hit inside `every_form_validates_and_matches` or `every_alternate_validates_and_matches` remains, the conversion is incomplete; if the `known_nonforms_are_invalid` hit is gone, scope was exceeded — spec §4 requires it stay on the real engine, because "absent from the index" is not an honest rendering of "the engine rejects this".

- [ ] **Step 5: Commit**

```bash
git add crates/panini/tests/paradigm/main.rs
git commit -m "test(paradigm): every_alternate_validates_and_matches reads the index

959 ALTERNATES rows, ~a quarter of the paradigm binary's old wall clock.
known_nonforms_are_invalid stays on the real check(): it is the only test of
the negative path, which an index lookup cannot represent honestly.

Claude-Session: https://claude.ai/code/session_01FzNcnmPnRppZFU1jfqsMna"
```

---

### Task 4: The sampled roundtrip and the drift gate

**Files:**
- Modify: `crates/panini/tests/roundtrip.rs` (full rewrite; absorbs Task 1's temporary test)

**Interfaces:**
- Consumes: `common::index::corpus_index()` from Task 1; `panini_analyze::LAKARAS`; `common::CELLS`.
- Produces: `roundtrip_sampled` (blocking tier) and `roundtrip_exhaustive` (`#[ignore]`d, run by Task 5's `test-full`).

- [ ] **Step 1: Replace the file**

Replace the whole of `crates/panini/tests/roundtrip.rs` with:

```rust
//! Every form the engine derives must be recoverable by `check()`.
//!
//! Two entry points share one body, so the sampled and exhaustive versions
//! cannot drift apart — only the iterator differs. The sampled one runs in the
//! blocking tier; the exhaustive one runs via `mise run test-full`.
//!
//! The body also reconciles `common::index` against the real `check()`. That
//! is what lets the paradigm binary trust the index: it is never taken on its
//! own authority, but checked against the thing it stands in for, on every
//! root.

mod common;

use common::CELLS;
use common::index::corpus_index;
use panini::Panini;
use panini_analyze::LAKARAS;
use panini_data::{Dhatu, Lakara, Pada, Purusha, Vacana};
use panini_data::dhatus;

type Cell = (&'static Dhatu, Lakara, Pada, Purusha, Vacana);

/// One cell per root, rotating the lakāra and the (puruṣa, vacana) pair
/// independently. `lcm(4, 9) == 36` and there are more than 72 roots, so every
/// lakāra × cell pair appears at least twice while every root appears exactly
/// once. No RNG, so no seed to record — and a new root adds itself, which is
/// what keeps this sample growing linearly with the corpus rather than
/// quadratically.
fn sample() -> impl Iterator<Item = Cell> {
    dhatus().iter().enumerate().map(|(i, d)| {
        let padas = d.pada.padas();
        let (purusha, vacana) = CELLS[i % CELLS.len()];
        (d, LAKARAS[i % LAKARAS.len()], padas[i % padas.len()], purusha, vacana)
    })
}

/// The same cross-product `panini_analyze::candidates()` builds.
fn full_cross_product() -> impl Iterator<Item = Cell> {
    dhatus().iter().flat_map(|d| {
        // `CELLS` is a `const` array, so `CELLS.iter()` would borrow a
        // temporary that is dropped at the end of the statement while the
        // returned iterator still holds it. `into_iter()` takes an owned copy.
        // `LAKARAS` is already a `&'static [Lakara]`, so `.iter()` is fine.
        LAKARAS.iter().flat_map(move |&lakara| {
            CELLS.into_iter().flat_map(move |(purusha, vacana)| {
                d.pada
                    .padas()
                    .iter()
                    .map(move |&pada| (d, lakara, pada, purusha, vacana))
            })
        })
    })
}

fn fingerprints(analyses: impl Iterator<Item = (String, Lakara, Pada, Purusha, Vacana)>) -> Vec<String> {
    let mut v: Vec<String> = analyses.map(|a| format!("{a:?}")).collect();
    v.sort();
    v
}

fn roundtrip_over(cells: impl Iterator<Item = Cell>) {
    let engine = Panini::new();
    let index = corpus_index();
    for (d, lakara, pada, purusha, vacana) in cells {
        for p in engine.derive(d, lakara, pada, purusha, vacana) {
            // The loop only ever asks for padas the root admits, so nothing
            // here should be blocked. Assert it rather than filtering: a
            // blocked branch appearing would mean `padas()` and the pada-
            // sanction rules (1.3.12 / 1.3.78) had come apart.
            assert!(
                !p.blocked,
                "{} {} {:?} {:?} {:?} derived a blocked branch",
                d.code,
                panini::lakara_name(lakara),
                pada,
                purusha,
                vacana
            );
            let form = p.text();
            let r = engine.check(&form);
            assert!(
                r.analyses.iter().any(|a| a.dhatu == d.code
                    && a.form_slp1 == form
                    && a.lakara == lakara),
                "roundtrip failed: {} {} -> {}",
                d.code,
                panini::lakara_name(lakara),
                form
            );
            let from_check = fingerprints(
                r.analyses
                    .iter()
                    .map(|a| (a.dhatu.clone(), a.lakara, a.pada, a.purusha, a.vacana)),
            );
            let from_index = fingerprints(
                index
                    .analyses(&form)
                    .iter()
                    .map(|a| (a.dhatu.to_string(), a.lakara, a.pada, a.purusha, a.vacana)),
            );
            assert_eq!(
                from_index, from_check,
                "common::index disagrees with check() for {form}"
            );
        }
    }
}

#[test]
fn roundtrip_sampled() {
    roundtrip_over(sample());
}

#[test]
#[ignore = "exhaustive; run via `mise run test-full`"]
fn roundtrip_exhaustive() {
    roundtrip_over(full_cross_product());
}
```

- [ ] **Step 2: Run the sampled test**

Run: `mise exec -- cargo test -p panini --test roundtrip`
Expected: PASS in roughly 25–32 s, with `roundtrip_exhaustive` reported as ignored.

- [ ] **Step 3: Verify the drift gate bites**

Temporarily edit `crates/panini/tests/common/index.rs` and delete the
`if p.blocked { continue; }` guard. Run
`mise exec -- cargo test -p panini --test roundtrip roundtrip_sampled`.
Expected: FAIL with `common::index disagrees with check() for ...`. Restore the guard.
This is the single most important verification in the plan: it proves the index cannot silently diverge from `check()`.

- [ ] **Step 4: Run the exhaustive test once**

Run: `mise exec -- cargo test -p panini --test roundtrip -- --ignored`
Use a Bash tool `timeout` of `3600000` and run it in the FOREGROUND.
Expected: PASS in roughly 1150–1400 s. This confirms the rewrite preserved the original exhaustive semantics before it goes behind `#[ignore]`.

- [ ] **Step 5: Commit**

```bash
git add crates/panini/tests/roundtrip.rs
git commit -m "test(roundtrip): sample in the blocking tier, exhaustive behind --ignored

One body, two iterators, so the two cannot drift. The sample takes one cell
per root with the lakara and cell rotated independently, and doubles as the
gate reconciling common::index against the real check().

Claude-Session: https://claude.ai/code/session_01FzNcnmPnRppZFU1jfqsMna"
```

---

### Task 5: The `test-full` tier

**Files:**
- Modify: `mise.toml` (new `[tasks.test-full]`)
- Modify: `.github/workflows/ci.yml` (new job on the existing weekly cron)

**Interfaces:**
- Consumes: `roundtrip_exhaustive` from Task 4.
- Produces: the `mise run test-full` command, which Task 8 references in `AGENTS.md`'s slice checklist.

- [ ] **Step 1: Add the task**

Add to `mise.toml`, immediately after the `[tasks.test]` block:

```toml
[tasks.test-full]
# The blocking tier runs `roundtrip_sampled` -- one cell per root. This runs
# the exhaustive twin as well, which re-derives the whole cross-product and
# calls the real check() on every form: ~20 minutes against the blocking
# tier's ~35 seconds.
#
# Run it once per slice before the mutation gate, and always after touching
# `Panini::check()`, `panini_analyze::candidates()`, or
# `crates/panini/tests/common/index.rs`.
run = "cargo test --workspace -- --include-ignored"
```

- [ ] **Step 2: Verify the task runs**

Run: `mise run test-full`
Use a Bash tool `timeout` of `3600000` and run it in the FOREGROUND.
Expected: PASS. `roundtrip_exhaustive` runs rather than being reported as ignored.

- [ ] **Step 3: Add the CI job**

Append to `.github/workflows/ci.yml`, as a sibling of the existing `audit` job (same indentation as `audit:`):

```yaml
  test-full:
    name: exhaustive suite (weekly)
    if: github.event_name == 'schedule'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: jdx/mise-action@v2
      - name: Test (including #[ignore]d exhaustive tests)
        run: mise run test-full
```

The `if:` is what keeps this off the PR path — the workflow's `on:` block already fires on `pull_request` and `push`, and only the `schedule` trigger should run the ~20-minute job.

- [ ] **Step 4: Verify the workflow still parses**

Run: `python3 -c "import yaml,sys; d=yaml.safe_load(open('.github/workflows/ci.yml')); print(sorted(d['jobs']))"`
Expected: `['audit', 'build-test-lint', 'test-full']`

- [ ] **Step 5: Commit**

```bash
git add mise.toml .github/workflows/ci.yml
git commit -m "chore: mise run test-full, on the existing weekly cron

The exhaustive roundtrip is ~20 minutes against the blocking tier's ~35
seconds, so it runs weekly and per-slice rather than per-PR.

Claude-Session: https://claude.ai/code/session_01FzNcnmPnRppZFU1jfqsMna"
```

---

### Task 6: Re-measure the floor and re-derive the mutation cap

**Files:**
- Modify: `mise.toml` (`[tasks.mutants]` `--timeout`)

**Interfaces:**
- Consumes: the converted suite from Tasks 2–4.
- Produces: the measured floor figures and the new `--timeout`, both of which Task 8 writes into `AGENTS.md`.

This repo's standing rule is **scale the floor by measurement, not by arithmetic.** The spec names ~600 s as the expected landing spot; it is not the decision. Measure first.

- [ ] **Step 1: Measure the uncontended floor**

Run: `mise run test`
Use a Bash tool `timeout` of `1800000` and run it in the FOREGROUND.
Expected: PASS in roughly 33–39 s. Record the per-binary times for `paradigm`, `roundtrip`, and `trace` separately — `AGENTS.md`'s floor series is recorded per binary, and Task 8 needs all three.

- [ ] **Step 2: Save slice 3b's campaign outcomes before anything rotates them**

Every `cargo mutants` invocation rotates `mutants.out` → `mutants.out.old`, so two probe runs destroy a finished campaign's record. Copy it out first:

```bash
mkdir -p /tmp/claude-1000/-workspace/3a98d00f-7194-4658-a4e5-6599a52c1bfb/scratchpad/mutants-baseline
cp mutants.out/outcomes.json /tmp/claude-1000/-workspace/3a98d00f-7194-4658-a4e5-6599a52c1bfb/scratchpad/mutants-baseline/3b-outcomes.json
wc -c /tmp/claude-1000/-workspace/3a98d00f-7194-4658-a4e5-6599a52c1bfb/scratchpad/mutants-baseline/3b-outcomes.json
```

Expected: a non-zero byte count. If `mutants.out/outcomes.json` is missing, try `mutants.out.old/outcomes.json` and say which one you used.

- [ ] **Step 3: Measure a contended uncaught run at `-j 4`**

An *uncaught* mutant is the one that runs the suite to completion, so it sets the cap. Time one directly, bypassing the mise shim (the `cargo-mutants` shim fails under some shells; run the binary):

```bash
MISE_ENV=dev mise exec -- cargo mutants --package panini-prakriya --test-workspace=true \
  -j 4 --timeout 4800 -o /tmp/claude-1000/-workspace/3a98d00f-7194-4658-a4e5-6599a52c1bfb/scratchpad/mutants-probe \
  --file crates/panini-prakriya/src/tripadi.rs 2>&1 | tail -30
```

Use a Bash tool `timeout` of `3600000` and run it in the FOREGROUND. Note `-o`, which keeps the probe away from `mutants.out`.
Expected: a run that reports caught/missed/timeout counts. Read the **longest** per-mutant test-phase duration out of the probe's `outcomes.json`; that, not the uncontended floor, is what the cap must clear.

- [ ] **Step 4: Set the cap**

Set `--timeout` in `mise.toml`'s `[tasks.mutants]` to at least **6× the longest test-phase duration** from Step 3, rounded up to a round number. Replace the stale rationale comment above `run =` — the paragraph about the 943.70 s floor at 2628 cells and the 2400 → 4800 raise describes a suite that no longer exists — with:

```toml
# --timeout must clear a FULL, UNCAUGHT run of the workspace suite at the
# parallelism actually used. cargo-mutants reads -j from CARGO_MUTANTS_JOBS,
# so an unqualified cap can be defeated by the environment alone; keep -j <= 4.
#
# Under too short a cap a genuine survivor is recorded as a TIMEOUT rather
# than a MISSED, which makes a reported zero-survivor run vacuous. Always
# check `timeout.txt` alongside `missed.txt`.
#
# One timeout is correct and permanent: tripadi.rs's natva backward scan
# decrements with `j -= 1`, and the `j /= 1` mutant never terminates. No
# assertion can catch it -- the cap itself is the detection mechanism. Do not
# chase it with a bigger --timeout.
#
# The floor and margin behind this number are recorded in AGENTS.md's
# cargo-mutants paragraph, the single source for this reasoning. Re-check it
# whenever the golden suite grows.
```

- [ ] **Step 5: Commit**

```bash
git add mise.toml
git commit -m "chore: re-derive the mutation cap against the O(N) suite

The 4800s cap cleared an N-squared floor that no longer exists, and it cost
80 minutes per campaign on the known-permanent tripadi.rs hang alone, which
the cap itself detects. Re-measured at -j 4 and set against that.

Claude-Session: https://claude.ai/code/session_01FzNcnmPnRppZFU1jfqsMna"
```

---

### Task 7: The mutation-coverage diff

**Files:**
- None modified unless the diff finds a regression.

**Interfaces:**
- Consumes: slice 3b's `outcomes.json` saved in Task 6 Step 2; the cap set in Task 6 Step 4.
- Produces: the caught/missed/timeout counts Task 8 records in `AGENTS.md`.

**This is the acceptance gate for the whole branch.** A green suite proves nothing here: the change deletes ~26M derivations of assertion pressure, and the question is whether that pressure was load-bearing. Do not skip this task because the tests pass.

- [ ] **Step 1: Run the full campaign**

Run: `mise run mutants`
Use a Bash tool `timeout` of `3600000` and run it in the FOREGROUND. If it is still running at the timeout, re-run with `--iterate` to resume rather than starting over — background shells are killed at ~60 minutes.
Expected: a campaign that completes in hours rather than the ~37 h slice 3b projected.

- [ ] **Step 2: Diff against 3b**

```bash
python3 - <<'PY'
import json
base = "/tmp/claude-1000/-workspace/3a98d00f-7194-4658-a4e5-6599a52c1bfb/scratchpad/mutants-baseline/3b-outcomes.json"
def outcomes(path):
    # Key on the whole serialized Mutant scenario rather than picking fields
    # out of it: cargo-mutants' outcomes.json schema is not pinned here, and a
    # guessed field path would KeyError or, worse, collide silently.
    d = json.load(open(path))
    return {json.dumps(o['scenario']['Mutant'], sort_keys=True): o['summary']
            for o in d['outcomes'] if isinstance(o.get('scenario'), dict) and 'Mutant' in o['scenario']}
old, new = outcomes(base), outcomes("mutants.out/outcomes.json")
assert old and new, "one of the outcome files parsed to nothing -- inspect the schema by hand"

regressed = [k for k, v in old.items() if v == 'CaughtMutant' and new.get(k) not in ('CaughtMutant', None)]
print("caught before, not caught now:", len(regressed))
for k in regressed: print("  ", k, old[k], "->", new.get(k))
print("new totals:", {v: sum(1 for x in new.values() if x == v) for v in set(new.values())})
PY
```

Expected: `caught before, not caught now: 0`.

- [ ] **Step 3: Check `timeout.txt` alongside `missed.txt`**

Run: `cat mutants.out/missed.txt; echo "--- timeouts ---"; cat mutants.out/timeout.txt`
Expected: `missed.txt` empty; `timeout.txt` containing **only** the known-permanent `tripadi.rs` `j /= 1` mutant. Any other timeout must be re-run alone before concluding anything — under a cap with too little headroom, a genuine survivor is recorded as a timeout and a zero-survivor report is vacuous.

- [ ] **Step 4: Resolve any regression**

If Step 2 reports a non-zero count, **the branch is blocked.** Fix it by widening `sample()` in `crates/panini/tests/roundtrip.rs` (e.g. two cells per root instead of one) or by adding an explicit pin for the behaviour the lost mutant covered — never by accepting the loss, and never by reverting to the N² suite. Re-run Steps 1–3 after the fix.

- [ ] **Step 5: Commit the record**

```bash
git add -A
git commit -m "chore: mutation-coverage diff against 3b — no regression

Claude-Session: https://claude.ai/code/session_01FzNcnmPnRppZFU1jfqsMna"
```

If nothing changed in the working tree, skip the commit and say so; the outcome is recorded in Task 8 instead.

---

### Task 8: Rewrite AGENTS.md's timing record

**Files:**
- Modify: `AGENTS.md` (the `cargo-mutants` paragraph under `## Environment`, and the slice checklist)

**Interfaces:**
- Consumes: the floor figures from Task 6 Step 1, the cap from Task 6 Step 4, and the campaign counts from Task 7.
- Produces: nothing later tasks depend on. This is the last task.

- [ ] **Step 1: Read the paragraph in full before touching it**

Run: `sed -n '/cargo-mutants. (mutation testing)/,/^- /p' AGENTS.md | head -120`
Most of its arithmetic describes a floor that no longer exists. It needs a **rewrite, not an append** — but two findings survive verbatim and must be carried across, not paraphrased away:
1. The reclassification hazard: under too short a cap a genuine survivor is recorded as a TIMEOUT, so a zero-survivor report that checks only `missed.txt` is vacuous.
2. The permanent `tripadi.rs` timeout: `j -= 1` mutated to `j /= 1` never terminates, no assertion can catch it, the cap *is* the detection mechanism, and it must not be chased with a bigger cap or a code change.

- [ ] **Step 2: Record the discontinuity explicitly**

The floor series has an entry per slice and every prior one was taken at the old config. Do not silently continue the series. Add, with the real measured numbers from Task 6 Step 1 substituted for the bracketed placeholders:

```markdown
    **2026-09-11 — the floor series has a hinge here; do not read across it.**
    Slice `test-suite-n-squared` removed the suite's Θ(N²): `candidates()`
    discards its argument and returns the full cross-product, so every
    `check()` re-derived the entire corpus and the three hot loops called it
    once per form. The corpus is now derived once per test binary into
    `crates/panini/tests/common/index.rs` and the loops read that index.
    Measured at the same 3636 cells, before and after:
    paradigm 1122.81s → [MEASURED]s, roundtrip 1354.58s → [MEASURED]s,
    trace 4.07s → [MEASURED]s; wall clock 2483s → [MEASURED]s.
    Every floor figure recorded above this entry was taken under the N²
    suite and is not comparable to anything below it.

    The exhaustive roundtrip survives behind `#[ignore]` as
    `roundtrip_exhaustive` and runs via `mise run test-full` — once per slice
    before the mutation gate, and always after touching `Panini::check()`,
    `panini_analyze::candidates()`, or `tests/common/index.rs`.

    Mutation campaign on the converted suite: [N] mutants, [N] caught,
    [N] missed, [N] unviable, and the one known-permanent `tripadi.rs`
    timeout. Diffed against slice 3b's `outcomes.json`: **zero mutants that
    3b caught are uncaught now.**

    Known and deliberate: `Panini::check()` still costs ~0.30s per call for
    CLI users, because this slice was scoped to the test harness. Narrowing
    `candidates()` by surface is the product fix, and `roundtrip_exhaustive`
    is the gate a narrowing slice would need — an over-narrowed candidate set
    shows up there as a roundtrip failure.
```

- [ ] **Step 3: Add the slice checklist line**

Find the slice checklist in `AGENTS.md` and add, next to the existing mutation-gate step:

```markdown
- Run `mise run test-full` once, before the mutation gate. The blocking tier
  samples one cell per root; this runs the exhaustive roundtrip over the whole
  cross-product (~20 minutes).
```

- [ ] **Step 4: Verify no stale figure survives**

Run: `grep -n "2483\|4800\|1354.58\|1122.81\|943.70\|37 h\|37h" AGENTS.md`
Every surviving hit must be inside the historical record or the new hinge entry, describing the *old* regime explicitly. A hit that reads as current guidance is a stale figure and must be rewritten. Also run `grep -n "opt-level" AGENTS.md docs/superpowers/specs/2026-09-11-test-suite-n-squared-design.md` and confirm nothing recommends raising it — §6 declines it, because against an O(N) suite it trades ~30 s of test phase for a slower rebuild on every mutant.

- [ ] **Step 5: Final verification and commit**

```bash
mise run fmt-check && mise run lint && mise run test
```
Expected: all three PASS, with `mise run test` at the figure recorded in Step 2.

```bash
git add AGENTS.md
git commit -m "docs: the floor series has a hinge at the N-squared removal

Rewrites the cargo-mutants paragraph rather than appending: most of its
arithmetic described a floor that no longer exists. Carries across the two
findings that survive -- the survivor-as-timeout reclassification hazard, and
the permanent tripadi.rs timeout the cap itself detects.

Claude-Session: https://claude.ai/code/session_01FzNcnmPnRppZFU1jfqsMna"
```
