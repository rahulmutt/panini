# `check()` from a Form Index — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `panini_analyze::candidates(surface)` return exactly the candidates that derive `surface`, from a lazily built corpus index. This makes `Panini::check()` cost a lookup instead of a full corpus derivation. Then retire the test tier, fixtures and mutation-gate bookkeeping that existed only because `check()` was slow.

**Architecture:**
- A process-wide `LazyLock<HashMap<String, Vec<Candidate>>>` lives in `panini-analyze`. A pure `index_from()` helper builds it from `(candidate, [(blocked, text)])` pairs.
- `Panini::check()` is untouched: it still re-derives each proposed candidate and confirms by exact match.
- The golden tests go back to calling the real `check()`.
- The roundtrip becomes exhaustive, gains a brute-force completeness oracle, and runs in the blocking tier.

**Tech Stack:** Rust 1.98.1 pinned via `mise`.
- Tasks: `mise run build | test | lint | fmt | fmt-check | mutants | audit`.
- `cargo-mutants` 27.1.0 comes from `mise.dev.toml`.

**Spec:** `docs/superpowers/specs/2026-09-30-check-form-index-design.md`

## Global Constraints

- `crates/panini/src/lib.rs` is **not modified**. `check()`'s analyses, their order, traces and duplicate-branch entries must stay identical.
- `panini-analyze` gains exactly one dependency: `panini-prakriya = { path = "../panini-prakriya" }`.
- `#![forbid(unsafe_code)]` stays at the top of every crate root.
- `mise run lint` is `cargo clippy --workspace --all-targets -- -D warnings`, so any warning fails.
- Mutation cap rule, verbatim from `AGENTS.md`: the cap must clear a full UNCAUGHT run of the workspace suite at the parallelism used by at least 5×. When it doesn't, the new cap is 6 × the longest uncaught test phase. Take the floor by measurement, never by scaling.
- `-j 4`, with `CARGO_MUTANTS_JOBS` unset (`env -u CARGO_MUTANTS_JOBS`).
- **Tool hazards (from this repo's record):**
  - The mise shim for `cargo-mutants` fails in background shells. Use `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`.
  - Every `cargo-mutants` invocation rotates `mutants.out`, so always pass `-o`.
  - Background shells die at ~60 min. Launch campaigns with `setsid nohup … &` and watch them with `pgrep -x cargo-mutants`, never `pgrep -f`.
- Run every `cargo`/`mise run` command in the **foreground** with an explicit Bash timeout of 600000 ms. None of them takes more than a minute now.

## Review Focus

1. **Homographs.** 315 forms have more than one candidate, and two roots share `code == "aS"`. `check()` must report *every* reading, and in `all_candidates()` order. This is pinned by Task 2's oracle, which compares sorted fingerprints (completeness), and by Task 1's `index_from` input-order test.
2. **Vikalpa branches that produce one form twice.** None exist in today's corpus. If one appears, `check()` must report both branches once each, not four times. Pinned by Task 1's `same_form_branches_list_candidate_once`.
3. **Blocked branches.** None exist in today's corpus. A future one must never become a lookup key. Pinned by Task 1's `blocked_branch_is_not_indexed`.
4. **Non-forms and non-SLP1 input.** Junk, empty strings, IAST or Devanagari. `check()` normalizes before calling `candidates()`, so an unknown key must simply give `Invalid`. Pinned by `known_nonforms_are_invalid` (unchanged) and Task 1's `proposes_nothing_for_a_nonform` (which includes `""`). Normalization in `check()` is untouched; `valid_word_returns_trace` in `crates/panini/src/lib.rs` already checks IAST input (`"bhavati"`).
5. **A stale cap hiding survivors.** A cap derived from the old ~115 s floor would be vacuous-safe but slow. A cap derived from a *contended* measurement could reclassify real survivors as TIMEOUT. Task 4 measures on a quiet host and checks `timeout.txt`.

---

## File Structure

| file | responsibility after this plan |
|---|---|
| `crates/panini-analyze/src/lib.rs` | `Candidate`; `all_candidates()` (the cross-product); `candidates(surface)` backed by `INDEX`; pure `index_from()`; unit tests |
| `crates/panini-analyze/Cargo.toml` | + `panini-prakriya` |
| `crates/panini/tests/paradigm/main.rs` | the two golden loops call `engine.check()` |
| `crates/panini/tests/common/index.rs` | **deleted** |
| `crates/panini/tests/common/mod.rs` | no `pub mod index` |
| `crates/panini/tests/roundtrip.rs` | one exhaustive `roundtrip` test plus a local brute-force `oracle()` |
| `mise.toml` | no `test-full`; `mutants` covers both packages at the measured cap |
| `.github/workflows/ci.yml` | no `test-full` job |
| `AGENTS.md` | tasks line, no test-full bullet, cargo-mutants paragraph collapsed to its durable parts plus one current entry |
| `docs/ARCHITECTURE.md` | lines 4 and 15 describe the index |

---

### Task 1: Index-backed `candidates()` in `panini-analyze`

**Files:**
- Modify: `crates/panini-analyze/Cargo.toml` (the `[dependencies]` table)
- Modify: `crates/panini-analyze/src/lib.rs` (whole file)
- Modify: `crates/panini/tests/common/index.rs:22,57`. This keeps the test fixture green until Task 2 deletes it.

**Interfaces:**
- Produces:
  - `pub fn all_candidates() -> Vec<Candidate>`
  - `pub fn candidates(surface_slp1: &str) -> Vec<Candidate>` (same signature, new meaning)
  - `#[derive(Clone, Copy)] pub struct Candidate { pub dhatu: &'static Dhatu, pub lakara: Lakara, pub pada: Pada, pub purusha: Purusha, pub vacana: Vacana }` (fields unchanged)
  - `pub const LAKARAS` (unchanged)
- Private: `fn index_from(derived: impl IntoIterator<Item = (Candidate, Vec<(bool, String)>)>) -> HashMap<String, Vec<Candidate>>`

- [ ] **Step 1: Write the failing tests**

Replace the `#[cfg(test)] mod tests { … }` block at the bottom of `crates/panini-analyze/src/lib.rs` with:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    /// `Dhatu::dhatupatha` is the unique key; `code` is not (both √aś rows
    /// spell `aS`).
    fn key(c: &Candidate) -> (&'static str, Lakara, Pada, Purusha, Vacana) {
        (c.dhatu.dhatupatha, c.lakara, c.pada, c.purusha, c.vacana)
    }

    fn keys(cs: &[Candidate]) -> Vec<(&'static str, Lakara, Pada, Purusha, Vacana)> {
        cs.iter().map(key).collect()
    }

    /// Two distinct candidates to feed `index_from` synthetic branches.
    fn two() -> (Candidate, Candidate) {
        let all = all_candidates();
        (all[0], all[1])
    }

    fn b(blocked: bool, text: &str) -> (bool, String) {
        (blocked, text.to_string())
    }

    #[test]
    fn proposes_bhu_prathama_eka_for_bhavati() {
        let cands = candidates("Bavati");
        assert!(cands.iter().any(|c| c.dhatu.code == "BU"
            && matches!(c.purusha, panini_data::Purusha::Prathama)
            && matches!(c.vacana, panini_data::Vacana::Eka)));
    }

    #[test]
    fn proposes_candidates_for_a_derived_form() {
        assert!(!candidates("BavAmaH").is_empty());
    }

    #[test]
    fn proposes_nothing_for_a_nonform() {
        assert!(candidates("gacCati").is_empty());
        assert!(candidates("").is_empty());
    }

    #[test]
    fn all_candidates_is_the_full_cross_product() {
        let expected: usize = dhatus()
            .iter()
            .map(|d| LAKARAS.len() * CELLS.len() * d.pada.padas().len())
            .sum();
        assert_eq!(all_candidates().len(), expected);
    }

    #[test]
    fn blocked_branch_is_not_indexed() {
        let (a, _) = two();
        let index = index_from([(a, vec![b(true, "x"), b(false, "y")])]);
        assert!(!index.contains_key("x"));
        assert_eq!(keys(&index["y"]), keys(&[a]));
    }

    #[test]
    fn same_form_branches_list_candidate_once() {
        let (a, _) = two();
        let index = index_from([(a, vec![b(false, "x"), b(false, "x")])]);
        assert_eq!(keys(&index["x"]), keys(&[a]));
    }

    #[test]
    fn candidate_with_two_forms_is_indexed_under_both() {
        let (a, _) = two();
        let index = index_from([(a, vec![b(false, "x"), b(false, "y")])]);
        assert_eq!(keys(&index["x"]), keys(&[a]));
        assert_eq!(keys(&index["y"]), keys(&[a]));
        assert_eq!(index.len(), 2);
    }

    #[test]
    fn candidates_keep_input_order_under_a_shared_form() {
        let (a, c) = two();
        let ab = index_from([(a, vec![b(false, "x")]), (c, vec![b(false, "x")])]);
        assert_eq!(keys(&ab["x"]), keys(&[a, c]));
        let ba = index_from([(c, vec![b(false, "x")]), (a, vec![b(false, "x")])]);
        assert_eq!(keys(&ba["x"]), keys(&[c, a]));
    }
}
```

- [ ] **Step 2: Run the tests and confirm they fail**

Run: `mise exec -- cargo test -p panini-analyze 2>&1 | tail -15`
Expected: compile errors: `cannot find function 'all_candidates'`, `cannot find function 'index_from'`.

- [ ] **Step 3: Implement**

Add to `crates/panini-analyze/Cargo.toml` under `[dependencies]`:

```toml
panini-prakriya = { path = "../panini-prakriya" }
```

In `crates/panini-analyze/src/lib.rs`, replace everything from the first line down to (not including) `#[cfg(test)]` with:

```rust
#![forbid(unsafe_code)]
use std::collections::HashMap;
use std::sync::LazyLock;

use panini_data::{Dhatu, Lakara, Pada, Purusha, Vacana, dhatus};
use panini_prakriya::derive;

#[derive(Clone, Copy)]
pub struct Candidate {
    pub dhatu: &'static Dhatu,
    pub lakara: Lakara,
    pub pada: Pada,
    pub purusha: Purusha,
    pub vacana: Vacana,
}

/// The lakāras this build can derive. The analyzer proposes the
/// (root × lakāra × cell) inputs that derive a surface form; the engine
/// confirms by exact surface match.
pub const LAKARAS: &[Lakara] = &[Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin];

const CELLS: &[(Purusha, Vacana)] = &[
    (Purusha::Prathama, Vacana::Eka),
    (Purusha::Prathama, Vacana::Dvi),
    (Purusha::Prathama, Vacana::Bahu),
    (Purusha::Madhyama, Vacana::Eka),
    (Purusha::Madhyama, Vacana::Dvi),
    (Purusha::Madhyama, Vacana::Bahu),
    (Purusha::Uttama, Vacana::Eka),
    (Purusha::Uttama, Vacana::Dvi),
    (Purusha::Uttama, Vacana::Bahu),
];

/// Every (root × lakāra × cell × pada) this build can derive, in a fixed
/// order: the single source of what the analyzer can propose.
pub fn all_candidates() -> Vec<Candidate> {
    let mut out = Vec::new();
    for d in dhatus() {
        for &lakara in LAKARAS {
            for &(purusha, vacana) in CELLS {
                for &pada in d.pada.padas() {
                    out.push(Candidate {
                        dhatu: d,
                        lakara,
                        pada,
                        purusha,
                        vacana,
                    });
                }
            }
        }
    }
    out
}

/// Exactly the candidates whose derivation produces `surface_slp1` as an
/// unblocked branch, in `all_candidates()` order. The engine re-derives
/// them to confirm and to attach traces.
pub fn candidates(surface_slp1: &str) -> Vec<Candidate> {
    INDEX.get(surface_slp1).cloned().unwrap_or_default()
}

/// The whole corpus, derived once per process on the first `candidates()`
/// call.
static INDEX: LazyLock<HashMap<String, Vec<Candidate>>> = LazyLock::new(|| {
    index_from(all_candidates().into_iter().map(|c| {
        let branches = derive(c.dhatu, c.lakara, c.pada, c.purusha, c.vacana)
            .into_iter()
            .map(|p| (p.blocked, p.text()))
            .collect();
        (c, branches)
    }))
});

/// Groups candidates by the surface forms their `(blocked, text)` branches
/// produce, keeping input order under every form.
///
/// A blocked branch's text is a partial string (often the bare root code),
/// never a surface form, so it is not indexed (cf. the pada blocks in
/// 1.3.12 / 1.3.78). A candidate whose vikalpa branches produce one form
/// twice is listed under it once: `check()` re-derives the candidate and
/// reports every matching branch itself, so a second listing would report
/// each branch twice.
fn index_from(
    derived: impl IntoIterator<Item = (Candidate, Vec<(bool, String)>)>,
) -> HashMap<String, Vec<Candidate>> {
    let mut index: HashMap<String, Vec<Candidate>> = HashMap::new();
    for (c, branches) in derived {
        let mut forms: Vec<String> = Vec::new();
        for (blocked, text) in branches {
            if !blocked && !forms.contains(&text) {
                forms.push(text);
            }
        }
        for form in forms {
            index.entry(form).or_default().push(c);
        }
    }
    index
}

```

- [ ] **Step 4: Keep the test fixture green**

`crates/panini/tests/common/index.rs` builds its index from `candidates("")`, which now returns nothing. Point it at the cross-product. Task 2 deletes the file.

- Line 22: `use panini_analyze::candidates;` → `use panini_analyze::all_candidates;`
- Line 57: `for c in candidates("") {` → `for c in all_candidates() {`

- [ ] **Step 5: Run the whole suite**

Run: `mise exec -- cargo test -p panini-analyze 2>&1 | tail -5`
Expected: `test result: ok. 8 passed`.

Run: `time mise run test 2>&1 | grep -E "Running|test result|real|FAILED|panicked"`
Expected: every binary `ok`. `roundtrip_sampled` and `known_nonforms_are_invalid` are now fast, so the wall time is well under 20 s.

- [ ] **Step 6: Lint and format**

Run: `mise run fmt && mise run lint 2>&1 | tail -3`
Expected: `Finished`, no warnings.

- [ ] **Step 7: Commit**

```bash
git add crates/panini-analyze crates/panini/tests/common/index.rs
git commit -m "feat(analyze): candidates() proposes exactly the candidates that derive the form

A lazy per-process corpus index replaces the full cross-product, so check()
derives only matching candidates instead of the whole corpus."
```

---

### Task 2: Golden tests back on the real `check()`, and an exhaustive roundtrip with an oracle

**Files:**
- Modify: `crates/panini/tests/paradigm/main.rs:54-117` (the two loops)
- Delete: `crates/panini/tests/common/index.rs`
- Modify: `crates/panini/tests/common/mod.rs` (drop `pub mod index;` and the blank line after it)
- Rewrite: `crates/panini/tests/roundtrip.rs`

**Interfaces:**
- Consumes (from Task 1): `panini_analyze::all_candidates() -> Vec<Candidate>`, and `Candidate` fields `dhatu, lakara, pada, purusha, vacana`.
- Produces: the blocking-tier `#[test] fn roundtrip()`. Task 3 removes every reference to `roundtrip_sampled`, `roundtrip_exhaustive` and `test-full`.

- [ ] **Step 1: Point the paradigm loops at `check()`**

In `every_form_validates_and_matches`:
- Replace `let index = common::index::corpus_index();` with `let engine = Panini::new();`
- Replace `let analyses = index.analyses(expected);` with these two lines:

```rust
            let r = engine.check(expected);
            let analyses = &r.analyses;
```

In `every_alternate_validates_and_matches`:
- Replace `let index = common::index::corpus_index();` with `let engine = Panini::new();`
- Replace `let analyses = index.analyses(form);` with:

```rust
        let r = engine.check(form);
        let analyses = &r.analyses;
```

Every `assert!` in both functions stays as it is. `a.dhatu == d.code` compiles as written (`String == &str`).

- [ ] **Step 2: Delete the fixture**

```bash
git rm crates/panini/tests/common/index.rs
```

In `crates/panini/tests/common/mod.rs`, delete the line `pub mod index;` and the blank line that follows it.

- [ ] **Step 3: Rewrite `roundtrip.rs`**

Replace the whole of `crates/panini/tests/roundtrip.rs` with:

```rust
//! Every form the engine derives must be recoverable by `check()`, with
//! exactly the analyses a brute-force pass over the whole corpus gives it.
//!
//! This runs over the full cross-product in the blocking tier.
//! `panini_analyze::candidates()` answers from a corpus index, so each
//! `check()` derives only the candidates that match.

use std::collections::HashMap;

use panini::Panini;
use panini_analyze::all_candidates;
use panini_data::{Lakara, Pada, Purusha, Vacana};
use panini_prakriya::derive;

fn fingerprint(
    dhatu: &str,
    lakara: Lakara,
    pada: Pada,
    purusha: Purusha,
    vacana: Vacana,
) -> String {
    format!("{:?}", (dhatu, lakara, pada, purusha, vacana))
}

/// What `check()` computed on every call before the analyzer had an index:
/// the whole corpus, derived by brute force and grouped by surface form. It
/// is the specification `panini_analyze::candidates()` must meet, not a
/// shortcut for other tests, so it lives here and nowhere else.
///
/// Neither side is deduplicated. A candidate whose vikalpa branches produce
/// one form twice contributes two fingerprints here, and two analyses to
/// `check()`.
fn oracle() -> HashMap<String, Vec<String>> {
    let mut by_form: HashMap<String, Vec<String>> = HashMap::new();
    for c in all_candidates() {
        for p in derive(c.dhatu, c.lakara, c.pada, c.purusha, c.vacana) {
            if p.blocked {
                continue;
            }
            by_form.entry(p.text()).or_default().push(fingerprint(
                c.dhatu.code,
                c.lakara,
                c.pada,
                c.purusha,
                c.vacana,
            ));
        }
    }
    for v in by_form.values_mut() {
        v.sort();
    }
    by_form
}

#[test]
fn roundtrip() {
    let engine = Panini::new();
    let oracle = oracle();
    for c in all_candidates() {
        let (d, lakara, pada, purusha, vacana) = (c.dhatu, c.lakara, c.pada, c.purusha, c.vacana);
        for p in engine.derive(d, lakara, pada, purusha, vacana) {
            // The cross-product only ever asks for padas the root admits, so
            // nothing here should be blocked. Assert it rather than
            // filtering: a blocked branch appearing would mean `padas()` and
            // the pada-sanction rules (1.3.12 / 1.3.78) had come apart.
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
                r.analyses
                    .iter()
                    .any(|a| a.dhatu == d.code && a.form_slp1 == form && a.lakara == lakara),
                "roundtrip failed: {} {} -> {}",
                d.code,
                panini::lakara_name(lakara),
                form
            );
            let mut from_check: Vec<String> = r
                .analyses
                .iter()
                .map(|a| fingerprint(&a.dhatu, a.lakara, a.pada, a.purusha, a.vacana))
                .collect();
            from_check.sort();
            assert_eq!(
                from_check, oracle[&form],
                "check() and the brute-force oracle disagree on {form}"
            );
        }
    }
}
```

- [ ] **Step 4: Run it and see it pass**

Run: `mise exec -- cargo test -p panini --test roundtrip 2>&1 | tail -4`
Expected: `test result: ok. 1 passed; 0 failed; 0 ignored`, in a few seconds.

- [ ] **Step 5: Prove the completeness assertion catches what the others miss**

Make a temporary edit to `crates/panini-analyze/src/lib.rs`, in `candidates()`. The edit lists every candidate twice, so `check()` reports every analysis twice:
- the "roundtrip failed" assertion still passes;
- only the oracle assertion should fail.

```rust
    INDEX.get(surface_slp1).map(|v| v.iter().chain(v).copied().collect()).unwrap_or_default()
```

Run: `mise exec -- cargo test -p panini --test roundtrip 2>&1 | grep -E "disagree|roundtrip failed|test result"`
Expected: `check() and the brute-force oracle disagree on …` and `test result: FAILED`. There must be **no** `roundtrip failed` line.

Revert the edit and confirm: `git diff --stat crates/panini-analyze` must show nothing.

- [ ] **Step 6: Full suite, lint, format**

Run: `time mise run test 2>&1 | grep -E "Running|test result|real|FAILED|panicked"`
Expected: all `ok`, with **no** `ignored` count in the roundtrip line. Wall time is around 5 s.

Run: `mise run fmt && mise run lint 2>&1 | tail -3`
Expected: clean.

- [ ] **Step 7: Commit**

```bash
git add -A crates/panini/tests
git commit -m "test: golden loops call check() again; one exhaustive roundtrip with a brute-force oracle

tests/common/index.rs and the sampled/exhaustive split existed only
because check() re-derived the corpus; both go."
```

---

### Task 3: Retire `test-full`, and update the docs that describe the old design

**Files:**
- Modify: `mise.toml` (delete `[tasks.test-full]`)
- Modify: `.github/workflows/ci.yml` (cron comment; delete the `test-full` job)
- Modify: `AGENTS.md:6-13` (tasks line and the test-full bullet)
- Modify: `docs/ARCHITECTURE.md:4,15`

**Interfaces:**
- Consumes: Task 2's single `roundtrip` test. After this task, nothing names `test-full`, `roundtrip_sampled`, `roundtrip_exhaustive`, `corpus_index` or `common/index.rs` outside `docs/superpowers/` and the cargo-mutants paragraph that Task 4 rewrites.

- [ ] **Step 1: `mise.toml`**

Delete the whole `[tasks.test-full]` table: the header, its eight comment lines, the `run = "cargo test --workspace -- --include-ignored"` line, and the blank line after it.

- [ ] **Step 2: `ci.yml`**

- Change the cron comment to `# weekly, Monday 06:00 UTC — audit job`.
- Delete the `test-full:` job: every line from `  test-full:` through `        run: mise run test-full`, plus the blank line before it.

Then check:

```bash
python3 -c "import yaml;print(sorted(yaml.safe_load(open('.github/workflows/ci.yml'))['jobs']))"
```

Expected: `['audit', 'build-test-lint']`.

- [ ] **Step 3: `AGENTS.md` environment bullets**

Replace lines 6–13 (from `- Tasks: …` through `  CI runs it on the weekly cron.`) with:

```markdown
- Tasks: `mise run build | test | lint | fmt | fmt-check | mutants | audit`.
- `mise run test` is the whole suite, including the exhaustive roundtrip:
  every derived form goes through the real `Panini::check()` and is compared
  against a brute-force oracle (`crates/panini/tests/roundtrip.rs`). It
  takes a few seconds, because `panini_analyze::candidates()` answers from a
  corpus index built once per process.
```

- [ ] **Step 4: `docs/ARCHITECTURE.md`**

Read lines 1–20 first.
- On line 4, the pipeline text `panini-analyze (candidates)` becomes `panini-analyze (candidates, from a corpus index derived through panini-prakriya)`. Keep the surrounding arrows and the code span intact, and reflow if the line gets too long.
- In the line-15 bullet, replace the description after `` `panini-analyze` — `` with: `proposes exactly the candidate (root, lakāra, pada, puruṣa, vacana) inputs whose derivation yields the surface form, from a corpus index derived once per process on first use; the engine re-derives them to confirm and attach traces.`

- [ ] **Step 5: Sweep**

```bash
grep -rn -E "test-full|roundtrip_sampled|roundtrip_exhaustive|corpus_index|common/index|include-ignored|discards its argument" \
  --include=*.rs --include=*.toml --include=*.md --include=*.yml . | grep -v -E "^\./(docs/superpowers|target)/"
```

Expected: hits **only** in `AGENTS.md` between the line starting `  - \`cargo-mutants\` (mutation testing)` and the line starting `  - \`cargo-deny\``. Task 4 rewrites that block. Also look for wrapped phrasings the grep can miss: search `AGENTS.md` outside that block, and `docs/ARCHITECTURE.md`, for "0.30", "N²", "Θ(N", "~25 min", "weekly" and "exhaustive". Fix any that describe the old design.

- [ ] **Step 6: Verify and commit**

Run: `mise run test 2>&1 | grep -c "test result: ok"` (expected: the same count as before this task), then `mise run fmt-check`.

```bash
git add mise.toml .github/workflows/ci.yml AGENTS.md docs/ARCHITECTURE.md
git commit -m "chore: retire test-full — the exhaustive roundtrip is the blocking tier now"
```

---

### Task 4: Re-base the mutation gate and collapse its record

**Files:**
- Modify: `mise.toml` (`[tasks.mutants]`: its `run` line and the comment line naming the package)
- Modify: `AGENTS.md`, in the cargo-mutants bullet (from `  - \`cargo-mutants\` (mutation testing)` up to, not including, `  - \`cargo-deny\``)

**Interfaces:**
- Consumes: Tasks 1–3 committed and green.
- Produces: the measured values `FLOOR_A`, `FLOOR_B` (s), `PROBE_ADESHA`, `PROBE_TRIPADI` (s), `CAP` (s), and the campaign counts, all written into `AGENTS.md`. Use real measurements; nothing here is projected.

Do all of this on a quiet host. Check `uptime` first. If the 1-minute load is above ~4, record the load next to every number.

- [ ] **Step 1: Measure the floor twice**

```bash
uptime; time mise run test 2>&1 | grep -E "Running tests|finished in|real"
uptime; time mise run test 2>&1 | grep -E "Running tests|finished in|real"
```

Record both `real` values as `FLOOR_A` and `FLOOR_B`, plus the paradigm, roundtrip and trace `finished in` times from each run.

- [ ] **Step 2: Probe the two uncaught equivalents at `-j 4`**

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null \
  | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /"
```

Expected: exactly `adesha.rs:588:30: replace + with *` and `tripadi.rs:1217:38: replace - with /`. Nothing in this branch touches those files, so the positions have not moved.

```bash
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 900 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:588:30: replace \+ with \*" --re "tripadi.rs:1217:38: replace - with /" 2>&1 | tail -6
python3 - "$SCRATCH/mutants.out/outcomes.json" <<'EOF'
import json, sys
for o in json.load(open(sys.argv[1]))["outcomes"]:
    print(o.get("scenario"), o.get("summary"), [(p.get("process_status"), p.get("phase"), p.get("duration")) for p in o.get("phase_results", [])])
EOF
```

Both must be **MISSED**, not TIMEOUT. Record each one's `test`-phase duration as `PROBE_ADESHA` and `PROBE_TRIPADI`. If the JSON keys differ, print one outcome raw and read `duration` from its test phase.

- [ ] **Step 3: Set the cap**

`L = max(PROBE_ADESHA, PROBE_TRIPADI, FLOOR_A, FLOOR_B)`.
`CAP = max(20, ceil(6 × L / 10) × 10)`.

Change `mise.toml`'s `[tasks.mutants]` `run` line to:

```toml
run = "cargo mutants --package panini-prakriya --package panini-analyze --test-workspace=true --timeout <CAP> -j 4"
```

Substitute the number for `<CAP>`. In the same table's comments, change `the mutated package's own tests` to `the mutated packages' own tests`.

Confirm the new package's mutants list:

```bash
mise exec -- "$CM" mutants --package panini-analyze --list 2>/dev/null
```

Record the list verbatim. You will compare the campaign's `panini-analyze` outcomes against it.

- [ ] **Step 4: Launch the campaign detached**

```bash
OUT="$HOME/mutants-records/check-form-index"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout <CAP> -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"
```

- Check progress with `tail -3 "$OUT/campaign.log"`.
- Check liveness with `pgrep -x cargo-mutants`.
- Run nothing CPU-heavy while it runs.
- Wait with a Monitor or a ScheduleWakeup on `pgrep -x cargo-mutants`, not a foreground sleep loop.

Expected duration: minutes, not hours. Each mutant costs roughly a ~1 s incremental rebuild plus a ≤ ~5 s suite.

- [ ] **Step 5: Read the outcomes against the pass criteria**

When `pgrep -x cargo-mutants` returns nothing:

```bash
date -u +"%F %T UTC" > "$OUT/finished"
tail -5 "$OUT/campaign.log"
for f in caught missed timeout unviable; do echo "$f: $(wc -l < "$OUT/mutants.out/$f.txt")"; done
cat "$OUT/mutants.out/missed.txt" "$OUT/mutants.out/timeout.txt"
grep -c panini-prakriya "$OUT/mutants.out/caught.txt"; grep -c panini-analyze "$OUT/mutants.out/caught.txt"
```

**Pass criteria**, compared with slice 3d's record (735 `panini-prakriya` mutants: 688 caught, 44 unviable, 2 missed, 1 timeout):
- **`panini-prakriya`**
  - 735 mutants, 44 unviable.
  - `missed.txt` holds exactly `adesha.rs:588:30: replace + with *` and `tripadi.rs:1217:38: replace - with /`.
  - `timeout.txt` holds exactly `tripadi.rs:1543:23: replace -= with /=`, the permanent ṇatva scan.
  - Caught = 688.
  - More caught is acceptable only if every newly caught mutant is named and explained (the exhaustive roundtrip now runs per mutant, so this is possible). Fewer caught is a failure: stop and report.
- **`panini-analyze`**: every mutant is caught or unviable. For any missed or timeout mutant, either:
  - add a test to `crates/panini-analyze/src/lib.rs` that kills it, commit it, and re-run just that mutant with `--re` and a fresh `-o "$(mktemp -d)"`; or
  - if it is genuinely equivalent, write down why for Step 6.
- Any other timeout is a suspect survivor. Re-run it alone with a fresh `-o` before concluding anything.
- `cargo-mutants` exits 3 when timeouts are present. That is expected.

- [ ] **Step 6: Rewrite the cargo-mutants paragraph in `AGENTS.md`**

Keep the bullet's opening text (the `--test-workspace` and `--timeout` rationale). Also keep, verbatim, the **"The cap must clear a full UNCAUGHT run…"**, **"One timeout is correct and permanent."** and **"Two tool hazards."** paragraphs. Then:

- In the opening text, update the quoted command to the new `run` line (both packages, `--timeout <CAP>`). Update the sentence about which package's tests the baseline exercises to say "the mutated packages' own unit tests".
- Replace the paragraph that begins **"The floor behind the 900s cap, measured at 4176 cells."**, up to the sentence ending "change `mise.toml` and this paragraph together.", with:

```markdown
    **The floor behind the <CAP>s cap, measured at 4176 cells on
    <YYYY-MM-DD>.** Two quiet `mise run test` runs took <FLOOR_A>s and
    <FLOOR_B>s wall clock (host load <LOAD>). An isolated `-j 4` probe of the
    two documented equivalent mutants ran the full suite uncaught in
    <PROBE_ADESHA>s (`adesha.rs:588:30`) and <PROBE_TRIPADI>s
    (`tripadi.rs:1217:38`). The cap is 6 × the longest of those, rounded up
    to the next 10s. It was 900s against the Θ(N²) suite, before
    `candidates()` answered from a corpus index. Take the floor by
    measurement, never by scaling it by cell count or by a projected
    contention multiplier. Re-measure the floor and an uncaught `-j 4` run
    whenever the golden suite grows, and change `mise.toml` and this
    paragraph together.
```

- Replace everything from the paragraph beginning **"The record below is the per-slice history"** through the end of the bullet (the line ending `to the next 100s).`, just before `  - \`cargo-deny\``) with:

```markdown
    **Current record (check-form-index, <YYYY-MM-DD>).** Campaign at
    `-j 4 --timeout <CAP>`, `--package panini-prakriya --package
    panini-analyze --test-workspace=true`, `-o
    /home/dev/mutants-records/check-form-index`, launched detached, window
    <START> – <FINISH> UTC. **panini-prakriya: <N> mutants, <C> caught,
    <U> unviable, 2 missed, 1 timeout**, identical in its non-caught set to
    slice 3d's (735 / 688 / 44 / 2 / 1). `missed.txt` held exactly:
    ```
    crates/panini-prakriya/src/tinanta/adesha.rs:588:30: replace + with *
    crates/panini-prakriya/src/tinanta/tripadi.rs:1217:38: replace - with /
    ```
    `timeout.txt` held exactly the permanent ṇatva mutant:
    ```
    crates/panini-prakriya/src/tinanta/tripadi.rs:1543:23: replace -= with /=
    ```
    **panini-analyze: <NA> mutants, <CA> caught, <UA> unviable, <MA>
    missed, <TA> timeout.** <One sentence per non-caught panini-analyze
    mutant: its line verbatim and why it is equivalent — or "none".>
    Caught-mutant test phases: max <MAX>s. `outcomes.json` is kept at
    `/home/dev/mutants-records/check-form-index/mutants.out/outcomes.json`.
    **The per-slice history** of the floor, the cap and every campaign from
    the pada audit through slice 3d, all measured against the Θ(N²) suite,
    was removed in the commit that introduced this paragraph. Read it with
    `git show $(git log -1 --format=%h -S"Current record (check-form-index" -- AGENTS.md)^:AGENTS.md`.
```

Fill every `<…>` from Steps 1–5.

Check that no `<` placeholder survives:

```bash
awk '/cargo-mutants. \(mutation testing\)/,/cargo-deny/' AGENTS.md | grep -n "<[A-Z_]*>\|<One sentence\|<YYYY"
```

Expected: no output.

- [ ] **Step 7: Verify and commit**

Run: `mise run fmt-check && mise run lint 2>&1 | tail -2 && mise run test 2>&1 | grep -E "FAILED|panicked" ; echo done`
Expected: `done`, with no `FAILED` or `panicked` line.

Run the Task 3 sweep grep again. It must now return nothing.

```bash
git add mise.toml AGENTS.md
git commit -m "chore: mutation gate re-based on the index-backed suite; per-slice floor history collapsed"
```

---

## After the tasks

- Push `check-form-index` and open a PR against `main`, linking the spec.
- Standing instruction: when CI is green, auto-merge, verify the commits are on `main`, then delete the branch and the `.worktrees/check-form-index` worktree.
