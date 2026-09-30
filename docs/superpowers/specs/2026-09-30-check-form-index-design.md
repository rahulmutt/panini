# `check()` from a form index — design

*2026-09-30*

## 1. The problem, measured

`panini_analyze::candidates()` discards its argument and returns the full
(root × lakāra × cell × pada) cross-product, so every `Panini::check()`
re-derives the entire corpus: ~0.3 s a call in the dev profile at 4176 cells.
PR #36 (`2026-09-11-test-suite-n-squared-design.md`) took the golden loops off
`check()` with a test-side index, but three costs remained, all from this one
cause:

| | at 4176 cells |
|---|---|
| `mise run test` | 93 s wall (host load ~14): paradigm 36.75 s, roundtrip 48.08 s, trace 6.60 s |
| `mise run test-full` | 40 m 59 s (slice 3d), close to the ~60 min background-shell limit |
| mutation `--timeout` | 900 s, due to rise to 1000 s next slice |

`roundtrip_sampled` and `known_nonforms_are_invalid` still call the real
`check()` in a loop, so the blocking tier is still Θ(N²).

**Spike (2026-09-30, throwaway).** A process-wide lazy form → candidates index
consulted by `check()` gave:

| | before | spike |
|---|---|---|
| `mise run test` (wall, incl. rebuild) | 93 s | 4.0 s |
| `mise run test-full` | ~41 min | 4.5 s (`roundtrip_exhaustive` 2.0 s) |
| rebuild after a code change in `panini-prakriya` | — | ~1.1 s |

All tests were green, including the exhaustive index-vs-`check()`
reconciliation.

## 2. Goals and non-goals

**Goals**
- `check()` costs one lookup plus the derivations that actually match, instead
  of a full corpus derivation.
- The exhaustive roundtrip, including a completeness check against a
  brute-force oracle, runs in the blocking tier. `test-full` is retired.
- The mutation gate is re-based on the new floor, and its coverage record is
  shown to match slice 3d's.

**Invariants**
- `check()`'s public behaviour is unchanged: the same analyses, in the same
  order, with the same traces and the same duplicate-branch entries.
- Every derived form still round-trips through the real `check()`.
- Mutation coverage of `panini-prakriya` is no weaker than 3d's.

**Non-goals**
- Constant-factor levers (`opt-level`, threads in `check()`, nextest). These
  are moot at ~4 s.
- Narrowing `candidates()` by inspecting the surface string (endings, root
  shape). Augments and reduplication make a sound filter hard, and the index
  makes it unnecessary.
- Rewriting earlier specs and plans. They are a historical record.

## 3. Product change: `panini-analyze`

`crates/panini-analyze/Cargo.toml` gains
`panini-prakriya = { path = "../panini-prakriya" }`. There is no cycle:
prakriya depends only on data.

```rust
#[derive(Clone, Copy)]
pub struct Candidate { /* unchanged fields */ }

/// Every (root × lakāra × cell × pada) this build can derive, in a fixed
/// order. The single source of what the analyzer can propose.
pub fn all_candidates() -> Vec<Candidate>;   // today's candidates() loop, unchanged

/// Exactly the candidates whose derivation produces `surface_slp1` as an
/// unblocked branch, in `all_candidates()` order. The engine re-derives them
/// to confirm and to attach traces.
pub fn candidates(surface_slp1: &str) -> Vec<Candidate>;
```

`candidates()` reads a private
`static INDEX: LazyLock<HashMap<String, Vec<Candidate>>>`, built on first use:

- For each candidate `c` in `all_candidates()`, and each branch `p` of
  `panini_prakriya::derive(c…)`:
  - skip `p` if it is `blocked`;
  - otherwise push `c` under `p.text()`, unless `c` is already the last entry
    under that key. Two vikalpa branches of one candidate that produce the
    same form list the candidate once.
- `candidates(s)` returns `INDEX.get(s)` copied into a `Vec`, or an empty
  `Vec`.

Two equality details matter:
- Candidate equality in the dedupe compares the dhātu by pointer identity
  (`std::ptr::eq`) plus the four enums. `Dhatu::code` is not unique (√aś), so
  it cannot be the comparison.
- Entries under a key stay in `all_candidates()` order because the build walks
  that order.

**`Panini::check()` is not modified.** It still re-derives each proposed
candidate and keeps branches where `!p.blocked && p.text() == slp1`. Because
the index proposes exactly the candidates that loop would have matched, in
the same relative order, `check()`'s output is identical. The only thing that
changes is how many non-matching candidates it derives: from ~4000 to zero.

**The CLI** runs one `check()` per invocation. Before, that call derived the
whole corpus; now the index build does, so the cost is unchanged.

**Unit tests in `panini-analyze`**
- Keep `proposes_bhu_prathama_eka_for_bhavati`.
- Keep `always_narrows_to_nonempty_for_covered_ending` (`BavAmaH` is a real
  √BU laṭ form). Rename it to say what it now checks: that a derived form's
  candidates are non-empty.
- New: a non-form (`gacCati`) proposes nothing.
- New: `all_candidates().len()` equals
  `Σ_d LAKARAS.len() × 9 × d.pada.padas().len()`.

## 4. Test harness

### 4.1 Paradigm loops go back to the real `check()`

`every_form_validates_and_matches` and `every_alternate_validates_and_matches`
(`crates/panini/tests/paradigm/main.rs`) replace
`common::index::corpus_index()` / `index.analyses(form)` with
`Panini::new()` / `engine.check(form).analyses`. The assertions and messages
stay word for word. `a.dhatu` becomes a `String`, which compares with
`d.code: &str` as written.

### 4.2 `tests/common/index.rs` is deleted

Delete the file, and remove `pub mod index;` from `tests/common/mod.rs`.

### 4.3 One exhaustive `roundtrip` test, with an oracle

`crates/panini/tests/roundtrip.rs` becomes a single `#[test] fn roundtrip()`
over the full cross-product. `sample()`, `roundtrip_sampled`,
`roundtrip_exhaustive`, the `#[ignore]` and the two-entry-point module doc go
away. The cross-product iterator is `panini_analyze::all_candidates()`, which
replaces the file's local `full_cross_product()`.

The oracle is local to the file:

```rust
/// What `check()` used to compute for every call: the whole corpus, derived
/// by brute force, grouped by surface form. It is the specification the
/// analyzer's index must meet, not a shortcut for other tests, so it lives
/// here and nowhere else.
fn oracle() -> HashMap<String, Vec<String /* fingerprint */>>;
```

It walks `all_candidates()`, derives each candidate, and for each unblocked
branch records the fingerprint `format!("{:?}", (code, lakara, pada, purusha,
vacana))` under `p.text()`. Each vec is then sorted.

For every candidate and every derived branch, the test asserts:
1. The branch is not blocked (kept from today).
2. `check(form)` has an analysis with this dhātu, lakāra and form (kept from
   today).
3. **Completeness:** the sorted fingerprints of `check(form).analyses` equal
   `oracle[form]`. The analyzer's index can neither drop nor invent a reading.
   This replaces the old index-vs-`check()` reconciliation, and it now runs on
   every form on every run instead of weekly.

The per-branch fingerprints in `oracle` are **not deduped**, and neither are
`check()`'s analyses, so a vikalpa candidate with two same-form branches
contributes two equal fingerprints to both sides.

### 4.4 Unchanged

`known_nonforms_are_invalid` stays the negative-path test of `check()`, and is
now fast. The `trace/` binary and `tests/cli.rs` are unchanged.

## 5. Tooling and docs

- **`mise.toml`**
  - Delete `[tasks.test-full]`.
  - `mutants` adds `--package panini-analyze` (§6).
  - Set `--timeout` to the measured cap (§6).
- **`.github/workflows/ci.yml`**
  - Delete the `test-full` job.
  - The weekly cron stays for `audit`; update its comment.
- **`AGENTS.md`**
  - Remove `test-full` from the task list and delete the "Run
    `mise run test-full` once per slice" bullet.
  - Rewrite the cargo-mutants paragraph as described in §6.
  - Sweep for other mentions of `test-full`, `roundtrip_sampled`,
    `common/index.rs`, "Θ(N²)" and the ~0.30 s `check()` cost, including
    wrapped lines and number words. Rewrite them or drop them.
- **`docs/ARCHITECTURE.md`**
  - Line 4: the pipeline notes that `panini-analyze` derives through
    `panini-prakriya` to build its index.
  - Line 15: `panini-analyze` "proposes exactly the candidates whose
    derivation yields the surface form, from a lazily built corpus index".

## 6. Mutation gate

**Scope.** `mise run mutants` becomes
`cargo mutants --package panini-prakriya --package panini-analyze
--test-workspace=true --timeout <cap> -j 4`. The index build (the blocked
filter and the dedupe) is now load-bearing, and the §4.3 oracle is what should
catch mutants in it.

**Re-basing the cap.** Use the existing rule: the cap must clear a full
UNCAUGHT workspace run at the parallelism used, by at least 5×.
1. With nothing else running, measure `mise run test` twice and record both
   as the floor.
2. In an isolated `-j 4` probe, run the two documented equivalent mutants
   (`adesha.rs`, `tripadi.rs`) and record their test phases. Pass `-o`, and
   copy the output before the next invocation, because each run rotates
   `mutants.out`.
3. Cap = 6 × the longest uncaught test phase, rounded up to the next 10 s,
   and not below cargo-mutants' 20 s floor.

**Campaign.** Run the full campaign detached
(`setsid nohup … > log 2>&1 < /dev/null &`, waited on with `kill -0`). Copy
`outcomes.json` somewhere durable as soon as it finishes. Pass criteria:
- **`panini-prakriya`** matches 3d: 735 mutants, 688 caught, 44 unviable,
  2 missed, 1 timeout. The missed set is the same two equivalents, verbatim,
  and the timeout is the permanent `tripadi.rs` `j /= 1`. More caught is
  acceptable only if every newly caught mutant is explained. Fewer caught is a
  failure.
- **`panini-analyze`** has no unexplained survivors. Each missed or timeout
  mutant is either killed by a new test or argued equivalent in writing.

**`AGENTS.md`'s cargo-mutants paragraph** shrinks from ~1,450 lines of
per-slice history to the durable parts:
- the flags and why each is there;
- the uncaught-cap rule and the `timeout.txt` check;
- the permanent timeout;
- one current entry: the floor, the probe, the cap, and the verbatim missed
  and timeout lists for both packages.

The entry names the commit that removed the history (`git show <hash>` to read
it).

## 7. Done means

- `mise run test`, `mise run lint` and `mise run fmt-check` pass.
- `mise run test` runs in ≤ 10 s wall with nothing else running, and the
  measured floor is recorded in `AGENTS.md`.
- The campaign meets §6's pass criteria, and its record is in `AGENTS.md`.
- CI is green on the PR, which is then auto-merged. After that, verify the
  commits are on `main`, then delete the branch and worktree.

## 8. Files touched

| file | change |
|---|---|
| `crates/panini-analyze/Cargo.toml` | + `panini-prakriya` dependency |
| `crates/panini-analyze/src/lib.rs` | `all_candidates()`, index-backed `candidates()`, unit tests |
| `crates/panini/tests/paradigm/main.rs` | two loops back on `check()` |
| `crates/panini/tests/common/index.rs` | deleted |
| `crates/panini/tests/common/mod.rs` | drop `pub mod index` |
| `crates/panini/tests/roundtrip.rs` | single exhaustive `roundtrip` with oracle |
| `mise.toml` | drop `test-full`; `mutants` scope and cap |
| `.github/workflows/ci.yml` | drop `test-full` job |
| `AGENTS.md` | tasks, checklist, cargo-mutants paragraph, sweep |
| `docs/ARCHITECTURE.md` | lines 4 and 15 |

`crates/panini/src/lib.rs` is deliberately **not** in this table.
