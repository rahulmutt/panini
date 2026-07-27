# kṅiti-ca guard fix and tinanta.rs split — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the 1.1.5 *kṅiti ca* guard operative on the śap-luk'd (adādi) path, and split the 4058-line `tinanta.rs` into a `tinanta/` module of nine files along the pipeline's own stage boundaries — with zero change to any derived form or trace.

**Architecture:** Two commit groups on one branch. Group 1 replaces a duplicated fixed-index guard (`p.terms[SHAP].has(Tag::Ngit)`, at 7.3.84 and 7.3.86) with a single `following_sarvadhatuka` helper that asks which term actually follows the aṅga — śap when it has text, the ending when śap has been luk'd by 2.4.72. Group 2 converts `TINANTA_RULES` from one flat `&[Rule]` into `&[&[Rule]]`, a list of six stages, and moves each stage into its own file together with its guard tests. The flattened rule order must be identical to `main` throughout, pinned by a verbatim id-sequence test written *before* any code moves.

**Tech Stack:** Rust 1.97.1 (pinned via mise), `cargo test`, `cargo clippy`, `cargo mutants` 27.1.0, `cargo audit` / `cargo deny`.

**Spec:** `docs/superpowers/specs/2026-07-27-kniti-guard-and-tinanta-split-design.md`

**Branch:** `kniti-guard-tinanta-split` (already created; the spec is committed at `d0e7cfa`).

## Global Constraints

- Toolchain is pinned by mise to **rust 1.97.1**. Never install Rust globally. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants | audit`.
- To scope tests to one crate, use `mise exec -- cargo test -p panini-prakriya`. **`mise run test -- -p X` does not scope** — it appends the args after `--workspace` and runs everything.
- To run mutation testing, invoke the binary directly — the mise shim fails in background shells:
  `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`
- `#![forbid(unsafe_code)]` in every non-fuzz crate. Do not add `unsafe`.
- SLP1 is the only internal representation. No transliteration outside `panini-lipi`.
- **No golden form and no trace line may change.** The paradigm test (`crates/panini/tests/paradigm.rs`, 1080 forms) and the ordered-trace test (`crates/panini/tests/trace.rs`) are the source of truth. If engine output diverges from a golden, the engine is wrong — **escalate; never edit a golden to match engine output.**
- **Rule order is the grammar.** The flattened `TINANTA_RULES` sequence must be byte-identical to `main` at every commit in this plan.
- Sūtra ids and names in traces must match vidyut-prakriya's `data/sutrapatha.tsv`. This plan adds no new sūtra, so no new citation checking is required.
- `mise run mutants` must end at **zero survivors**.
- Commit after every task. Do not squash groups together.

---

## File Structure

Group 1 touches only `crates/panini-prakriya/src/tinanta.rs`.

Group 2 turns that file into a directory. Final layout of `crates/panini-prakriya/src/tinanta/`:

| file | rules | responsibility |
|---|---|---|
| `mod.rs` | — | `derive`, the `TINANTA_RULES` stage list, `rules()`; module declarations. Reads as the pipeline's table of contents. |
| `terms.rs` | — | `ANGA` / `ENDING_PRE_SHAP` / `SHAP` / `ENDING`, the 3.1.68-bisection NOTE, the empty-śap caveat, `following_sarvadhatuka` |
| `sound.rs` | — | `guna_of`, `vrddhi_of`, `is_vowel`, `is_jhal`, `is_khar`, `cartva_of`, `is_vibhakti_protected_final` + their unit tests |
| `samjna.rs` | 5 | 1.3.12, 1.3.78, 3.4.78, 1.3.9, 1.2.4 — pada sanction, it-elision, ending insertion. **Before** 3.1.68. |
| `tin.rs` | 18 | 3.4.85 … 3.4.102 — lakāra → tiṅ substitution and ending reshaping. **Before** 3.1.68. |
| `vikarana.rs` | 5 | 3.1.69, 3.1.77, 3.1.68, 2.4.72, 1.2.4 — **contains** the boundary. |
| `anga.rs` | 14 | 6.4.71 … 7.3.101, including 6.1.78 — **after** the boundary. |
| `adesha.rs` | 8 | 6.1.101 … 6.4.101 — after the boundary. |
| `tripadi.rs` | 6 | 8.2.77 … 8.4.55 — after the boundary. |
| `derivation_tests.rs` | — | `#[cfg(test)]` only: whole-derivation tests, cross-cutting invariants, the ordered-id pin, and the `pub(crate)` test helpers. |

Also modified: `crates/panini-prakriya/src/controller.rs` (signature), `docs/ARCHITECTURE.md`, `crates/panini/tests/trace.rs` (header comment only), `AGENTS.md`.

**The authoritative rule order** (56 ids, current `main`), which every task must preserve:

```
 1  1.3.12    15  3.4.100   29  6.4.71    43  6.1.101
 2  1.3.78    16  3.4.80    30  6.4.72    44  6.1.96
 3  3.4.78    17  3.4.79    31  7.3.100   45  6.1.90
 4  1.3.9     18  3.4.91    32  7.1.5     46  6.1.97
 5  1.2.4     19  3.4.93    33  7.1.6     47  6.1.87
 6  3.4.85    20  3.4.90    34  7.1.3     48  6.1.66
 7  3.4.108   21  3.4.92    35  7.2.79    49  6.4.105
 8  3.4.105   22  3.4.103   36  7.2.80    50  6.4.101
 9  3.4.106   23  3.4.102   37  7.2.81    51  8.2.77
10  3.4.101   24  3.1.69    38  7.4.21    52  8.2.23
11  3.4.99    25  3.1.77    39  7.3.84    53  8.2.25
12  3.4.87    26  3.1.68    40  7.3.86    54  8.3.15
13  3.4.89    27  2.4.72    41  6.1.78    55  8.3.59
14  3.4.86    28  1.2.4     42  7.3.101   56  8.4.55
```

Stage cuts: `samjna` 1–5, `tin` 6–23, `vikarana` 24–28, `anga` 29–42, `adesha` 43–50, `tripadi` 51–56.

---

# Group 1 — the kṅiti ca guard

## Task 1: `following_sarvadhatuka` and the guard rewire

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` — add helper near the index constants (after `const ENDING: usize = 2;` and its NOTE block, currently ending ~line 129); rewire 7.3.84 (`:1124`) and 7.3.86 (`:1151`); rewrite 7.4.21's comment (`:1061`–`:1095`)
- Test: `crates/panini-prakriya/src/tinanta.rs` (the in-file `mod tests`)

**Interfaces:**
- Consumes: `Prakriya`, `Term`, `Tag::Ngit`, the `ANGA` / `SHAP` / `ENDING` constants — all already in this file.
- Produces: `fn following_sarvadhatuka(p: &Prakriya) -> Option<&Term>` (module-private for now; becomes `pub(crate)` in `terms.rs` at Task 6). Later tasks move it but do not change its signature.

**Background you need:** adādi (gaṇa 2) roots *luk* the śap vikaraṇa. 2.4.72 does this by emptying śap's `text` while leaving the term in place, so the `ANGA`(0) / `SHAP`(1) / `ENDING`(2) indices stay valid. An empty term interposes nothing, so on that path the *ending* is what immediately follows the aṅga — and the ending is what carries the `Tag::Ngit` that 1.2.4's first application assigns. A guard that reads `terms[SHAP]` therefore never sees it.

**Do not touch** the third `p.terms[SHAP].has(Tag::Ngit)` in this file, at **`:797`**, inside 1.2.4's *second* application. It is an idempotence guard ("have I already tagged this vikaraṇa?"), not an application of 1.1.5, and its subject is correctly the vikaraṇa term itself. Changing it is a bug.

- [ ] **Step 1: Write the four failing tests**

Add these to the existing `mod tests` in `crates/panini-prakriya/src/tinanta.rs`, next to `sarvadhatukardhadhatukayoh_blocks_guna_when_vikarana_is_ngit` (currently `:3394`). The aṅga texts are *constructed shapes*, not real derivations — the existing tests in this file do the same (`"nI"` is not an adādi root either); what is being pinned is the guard's boundary, not a paradigm cell.

```rust
    #[test]
    fn sarvadhatukardhadhatukayoh_blocks_guna_when_luk_shap_ending_is_ngit() {
        // The athematic (śap-luk'd) shape: an empty śap interposes nothing,
        // so the NGIT ending is what immediately follows the aGga and 1.1.5
        // must block guNa. Before this arm existed the guard read only
        // terms[SHAP] -- which on this path carries Sap's own Tag::Pit and
        // can never be Ngit -- so the block was silently inoperative.
        let mut p = Prakriya {
            terms: vec![Term::new("nI"), Term::new(""), Term::new("te")],
            log: vec![],
            ..Default::default()
        };
        p.terms[ENDING].add(Tag::Ngit);
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.3.84").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "nI");
    }

    #[test]
    fn sarvadhatukardhadhatukayoh_applies_guna_when_luk_shap_ending_is_not_ngit() {
        // Same athematic shape, non-Ngit ending: nothing blocks, guNa fires.
        // This is the "just outside the guard" half of the pair -- without it
        // a mutant that always blocks on the athematic path would survive.
        let mut p = Prakriya {
            terms: vec![Term::new("nI"), Term::new(""), Term::new("ti")],
            log: vec![],
            ..Default::default()
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.3.84").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "ne");
    }

    #[test]
    fn pugantalaghupadhasya_blocks_guna_when_luk_shap_ending_is_ngit() {
        // 7.3.86 carries the identical 1.1.5 guard, so it needs the identical
        // pair. "vft" is a light ik penult before a single consonant, which is
        // this rule's shape; the Ngit ending must still block it.
        let mut p = Prakriya {
            terms: vec![Term::new("vft"), Term::new(""), Term::new("te")],
            log: vec![],
            ..Default::default()
        };
        p.terms[ENDING].add(Tag::Ngit);
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.3.86").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "vft");
    }

    #[test]
    fn pugantalaghupadhasya_applies_guna_when_luk_shap_ending_is_not_ngit() {
        let mut p = Prakriya {
            terms: vec![Term::new("vft"), Term::new(""), Term::new("ti")],
            log: vec![],
            ..Default::default()
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.3.86").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "vart");
    }
```

- [ ] **Step 2: Run the tests to verify two of them fail**

Run: `mise exec -- cargo test -p panini-prakriya luk_shap_ending`

Expected: the two `..._is_ngit` tests **FAIL** (the current guard reads the empty śap, misses the ṅit, applies guṇa — `assert!(!(rule.apply)(...))` trips). The two `..._is_not_ngit` tests **PASS** already; that is correct and expected, they are the outside-the-guard half.

- [ ] **Step 3: Add the helper**

Insert immediately after the `// A further caveat since adādi (gaṇa 2) landed: …` NOTE block that follows `const ENDING: usize = 2;` (currently ends ~`:129`), before `/// The ordered rule list.`:

```rust
/// The sārvadhātuka that immediately follows the aṅga — the term **1.1.5
/// *kṅiti ca*** interrogates when it asks whether guṇa is blocked.
///
/// Normally that is the vikaraṇa at `SHAP`. But 2.4.72
/// *adiprabhṛtibhyaḥ śapaḥ* luks śap for adādi by emptying its text while
/// leaving the term in place (so these indices stay valid), and an empty
/// term interposes nothing: the ending is then what immediately follows the
/// aṅga, and the ending is what carries the ṅit tag 1.2.4's first
/// application assigns. Reading a fixed `SHAP` index therefore renders
/// 1.1.5 inoperative for the whole śap-luk'd path.
///
/// Returning the *immediate* follower — rather than testing every later
/// term — is what keeps the thematic ātmanepada path correct: for √labh the
/// ending is ṅit but śap is pit, and guṇa rightly proceeds.
///
/// Only meaningful after 3.1.68 has inserted śap. Every caller is ordered
/// after it. Returns `None` when there is no follower at all (a hand-built
/// one-term prakriya in a unit test), in which case nothing can block.
fn following_sarvadhatuka(p: &Prakriya) -> Option<&Term> {
    match p.terms.get(SHAP) {
        Some(shap) if !shap.text.is_empty() => Some(shap),
        Some(_) => p.terms.get(ENDING),
        None => None,
    }
}
```

- [ ] **Step 4: Rewire 7.3.84**

In the 7.3.84 rule body (currently `:1122`–`:1126`), replace:

```rust
            // 1.1.5 kṅiti ca: a following ṅit sārvadhātuka blocks guṇa. The
            // vikaraṇa at SHAP is ṅit (1.2.4) exactly when apit (śyan, śa);
            // śap is pit and is not, so bhvādi guṇa is unaffected.
            if p.terms.len() > SHAP && p.terms[SHAP].has(Tag::Ngit) {
                return false;
            }
```

with:

```rust
            // 1.1.5 kṅiti ca: a following ṅit sārvadhātuka blocks guṇa. On
            // the thematic path that follower is the vikaraṇa, ṅit (1.2.4)
            // exactly when apit (śyan, śa); śap is pit and is not, so bhvādi
            // guṇa is unaffected. On the śap-luk'd path it is the ending —
            // see `following_sarvadhatuka`. Narrowness: the sūtra is *kṅiti*,
            // ṅit OR kit; this engine has no kit tag because no implemented
            // rule assigns or consumes one. Widen this test the moment a kit
            // sārvadhātuka enters scope.
            if following_sarvadhatuka(p).is_some_and(|t| t.has(Tag::Ngit)) {
                return false;
            }
```

- [ ] **Step 5: Rewire 7.3.86**

In the 7.3.86 rule body (currently `:1149`–`:1153`), replace the identical five lines with:

```rust
            // 1.1.5 kṅiti ca, exactly as at 7.3.84 above — same follower
            // lookup, same ṅit-only narrowness.
            if following_sarvadhatuka(p).is_some_and(|t| t.has(Tag::Ngit)) {
                return false;
            }
```

- [ ] **Step 6: Run the new tests to verify they pass**

Run: `mise exec -- cargo test -p panini-prakriya luk_shap_ending`

Expected: all four PASS.

- [ ] **Step 7: Rewrite 7.4.21's comment — delete the latency note**

In the comment block above the 7.4.21 rule (currently `:1061`–`:1095`), replace this paragraph:

```rust
    // Ordered immediately before 7.3.84, which then declines on its own — its
    // target must be ik-final and `Se` is not — so 7.3.84's 1.1.5 guard is
    // untouched, and the trace credits the guṇa to the sūtra that licenses it.
```

with:

```rust
    // Ordered immediately before 7.3.84, and now genuinely its apavāda: on
    // this śap-luk'd path the ṅit ātmanepada ending IS the immediate follower
    // (see `following_sarvadhatuka`), so 1.1.5 really does block 7.3.84 here
    // and 7.4.21 is the targeted override that licenses *śete*. The ordering
    // additionally covers the loṭ-uttama cells, whose endings 1.2.4's first
    // application deliberately leaves untagged: there nothing blocks 7.3.84,
    // but 7.4.21 has already reshaped the aṅga to `Se`, on which 7.3.84
    // declines by its own shape guard (`guna_of('e')` is `None`). Either way
    // the trace credits the guṇa to the sūtra that licenses it.
```

Then **delete the entire final paragraph** of that comment block, from `// Latency note: 7.3.84's 1.1.5 guard currently tests \`p.terms[SHAP]\`, but` through `// ending's own Ngit tag, the way 7.2.81 does.` (11 lines). It describes a defect that no longer exists.

- [ ] **Step 8: Run the full suite — the zero-delta check**

Run: `mise run test`

Expected: **everything passes, with no change to any of the 1080 golden forms and no change to any trace line.** This is the load-bearing verification of the whole slice.

**If any golden or trace test fails:** the spec's analysis was wrong. **Stop. Do not edit the golden.** Report which cell diverged, its expected and actual forms, and the two traces. An implementer editing a golden to match engine output is this project's canonical red flag for a masked engine bug.

- [ ] **Step 9: Lint and format**

Run: `mise run fmt && mise run lint`

Expected: clean. `clippy` runs with `-D warnings`.

- [ ] **Step 10: Commit**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "fix(prakriya): 1.1.5 kṅiti ca guard reads the actual following sārvadhātuka

7.3.84 and 7.3.86 both tested p.terms[SHAP].has(Tag::Ngit), a fixed index
holding the vikaraṇa. Since adādi, 2.4.72 luks śap by emptying it in place,
so the ṅit tag rides on ENDING and the 1.1.5 block was inoperative for the
whole śap-luk'd path.

Replace the duplicated inline condition with following_sarvadhatuka(), which
returns śap when it has text and the ending when śap is luk'd. Behaviour is
unchanged today — the only śap-luk'd ik-final aṅga is √śī, and 7.4.21
reshapes it to Se before 7.3.84 looks — so all 1080 goldens and every trace
are byte-identical. 7.4.21's latency note is deleted: it is now genuinely
1.1.5's apavāda rather than accidentally ordered ahead of a sleeping guard.

1.2.4's own idempotence guard at the former :797 is deliberately untouched."
```

---

## Task 2: Prove the new arm under mutation

The new athematic branch is traversed by no golden form, so it is exactly the kind of code this project has twice refused to keep unpinned (8.4.53 removed as unreachable; 6.1.78's E/O arms dropped). Task 1's tests are supposed to kill every mutant in it. This task proves that before the refactor buries it.

**Files:** none modified unless a survivor is found.

- [ ] **Step 1: Install the dev tooling if it is not present**

Run: `MISE_ENV=dev mise install`

Expected: `cargo-mutants` 27.1.0 available at
`/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`.

- [ ] **Step 2: List the mutants in the new helper and the two guards**

Run:

```bash
/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants \
  --package panini-prakriya --test-workspace=true --timeout 60 \
  --file crates/panini-prakriya/src/tinanta.rs \
  --re 'following_sarvadhatuka' --list
```

Expected: several mutants, including replacing the function body with `None`, and negating / replacing the `!shap.text.is_empty()` condition with `true` and with `false`.

- [ ] **Step 3: Run them**

Run the same command without `--list`.

Expected: **0 survivors.** Reasoning, so you can tell a real survivor from a flake:
- body → `None`: nothing ever blocks; `sarvadhatukardhadhatukayoh_blocks_guna_when_vikarana_is_ngit` (the pre-existing thematic test) fails.
- `!shap.text.is_empty()` → `true`: always reads śap; `sarvadhatukardhadhatukayoh_blocks_guna_when_luk_shap_ending_is_ngit` fails.
- `!shap.text.is_empty()` → `false`: always reads the ending; on the 2-term thematic prakriya `get(ENDING)` is `None`, so nothing blocks and the thematic test fails.

- [ ] **Step 4: If there is a survivor, resolve it before continuing**

Add a rule-level test that executes the surviving branch, following the inside/outside pattern of Step 1's tests, and re-run Step 3.

**If no test can kill it,** the branch is genuinely unexecutable and this project does not keep unexecutable arms under the mutation gate. In that case: revert the athematic arm, restore 7.4.21's latency note with an added sentence recording that the arm was attempted and why it was withdrawn, and report the outcome. Do **not** park the mutant with a comment — 7.3.100's "unkillable" mutant was parked on a case analysis that later proved wrong, and slice 5f had to undo it.

- [ ] **Step 5: Commit only if Step 4 changed something**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "test(prakriya): pin the remaining kṅiti-guard mutant"
```

If Step 3 was already clean, there is nothing to commit — say so and move on.

---

# Group 2 — the split

## Task 3: Pin the rule order before anything moves

The ordered array *is* the grammar. A split that silently reorders a rule can still produce correct surface forms while producing wrong derivations. This pin must exist **before** the first line moves.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (the in-file `mod tests`)

**Interfaces:**
- Consumes: `TINANTA_RULES`.
- Produces: the test `tinanta_rule_order_is_pinned`, which Task 12 moves verbatim to `derivation_tests.rs`.

- [ ] **Step 1: Write the test**

Add to `mod tests` in `crates/panini-prakriya/src/tinanta.rs`:

```rust
    /// The ordered rule list IS the grammar this crate implements, so its
    /// sequence is pinned verbatim, not merely by the per-derivation traces
    /// in `crates/panini/tests/trace.rs` (which only pin order along the
    /// paths representative forms happen to take).
    ///
    /// If you add a rule, add its id here in position. If this test fails
    /// after a refactor that was supposed to move code without changing it,
    /// the refactor reordered the grammar — fix the refactor, not this list.
    ///
    /// 1.2.4 appears twice, deliberately: once tagging apit ātmanepada
    /// endings, once tagging the apit vikaraṇa after 3.1.68 inserts it.
    #[test]
    fn tinanta_rule_order_is_pinned() {
        let expected = [
            "1.3.12", "1.3.78", "3.4.78", "1.3.9", "1.2.4", "3.4.85", "3.4.108", "3.4.105",
            "3.4.106", "3.4.101", "3.4.99", "3.4.87", "3.4.89", "3.4.86", "3.4.100", "3.4.80",
            "3.4.79", "3.4.91", "3.4.93", "3.4.90", "3.4.92", "3.4.103", "3.4.102", "3.1.69",
            "3.1.77", "3.1.68", "2.4.72", "1.2.4", "6.4.71", "6.4.72", "7.3.100", "7.1.5",
            "7.1.6", "7.1.3", "7.2.79", "7.2.80", "7.2.81", "7.4.21", "7.3.84", "7.3.86",
            "6.1.78", "7.3.101", "6.1.101", "6.1.96", "6.1.90", "6.1.97", "6.1.87", "6.1.66",
            "6.4.105", "6.4.101", "8.2.77", "8.2.23", "8.2.25", "8.3.15", "8.3.59", "8.4.55",
        ];
        let actual: Vec<&str> = TINANTA_RULES.iter().map(|r| r.id).collect();
        assert_eq!(actual, expected);
    }
```

- [ ] **Step 2: Run it**

Run: `mise exec -- cargo test -p panini-prakriya tinanta_rule_order_is_pinned`

Expected: **PASS** on the first run. This test describes current reality; it is a regression pin, not a TDD red step. If it fails, the id list above was transcribed wrong — fix the list against `grep -n '^        id: "' crates/panini-prakriya/src/tinanta.rs`, not the rule array.

- [ ] **Step 3: Commit**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "test(prakriya): pin the 56-rule TINANTA_RULES order verbatim

Guards the file split that follows: rule order is the grammar, and a reorder
can preserve surface forms while corrupting derivations."
```

---

## Task 4: `tinanta.rs` → `tinanta/mod.rs` (pure rename)

**Files:**
- Rename: `crates/panini-prakriya/src/tinanta.rs` → `crates/panini-prakriya/src/tinanta/mod.rs`

- [ ] **Step 1: Move the file with git so rename detection works**

```bash
mkdir -p crates/panini-prakriya/src/tinanta
git mv crates/panini-prakriya/src/tinanta.rs crates/panini-prakriya/src/tinanta/mod.rs
```

- [ ] **Step 2: Verify nothing else needs changing**

`crates/panini-prakriya/src/lib.rs` declares `pub mod tinanta;` — that resolves to `tinanta/mod.rs` unchanged. No edit needed.

- [ ] **Step 3: Run the full suite**

Run: `mise run test`

Expected: all pass, byte-identical output. This commit changes no code at all.

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "refactor(prakriya): tinanta.rs -> tinanta/mod.rs (pure rename)"
```

---

## Task 5: Extract `terms.rs`

**Files:**
- Create: `crates/panini-prakriya/src/tinanta/terms.rs`
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs`

**Interfaces:**
- Produces: `pub(crate) const ANGA: usize`, `ENDING_PRE_SHAP: usize`, `SHAP: usize`, `ENDING: usize`; `pub(crate) fn following_sarvadhatuka(p: &Prakriya) -> Option<&Term>`. Every later stage file imports these.

- [ ] **Step 1: Create the file**

Move into `crates/panini-prakriya/src/tinanta/terms.rs`, **verbatim**, the four index constants with their doc comments, the long `// NOTE:` block about 3.1.68 bisecting the array, the `// A further caveat since adādi (gaṇa 2) landed:` block, and `following_sarvadhatuka` (currently `mod.rs` lines ~97–160). Add at the top:

```rust
//! Term layout for the tiṅanta pipeline: which index holds what, when that
//! changes, and which term counts as "the follower" for rules that ask.
//!
//! Every rule in this pipeline addresses terms by these constants. The two
//! caveats below are load-bearing — a rule that ignores either produces a
//! non-word or panics, with no test able to name the cause.

use crate::prakriya::Prakriya;
use crate::term::Term;
```

Change each item's visibility to `pub(crate)`: the four constants and `following_sarvadhatuka`.

- [ ] **Step 2: Wire it into `mod.rs`**

Delete the moved items from `mod.rs`. Add near the other module items at the top of `mod.rs`:

```rust
mod terms;

pub(crate) use terms::{ANGA, ENDING, ENDING_PRE_SHAP, SHAP, following_sarvadhatuka};
```

The `pub(crate) use` re-export keeps every existing reference in `mod.rs` compiling unchanged.

- [ ] **Step 3: Build and test**

Run: `mise exec -- cargo test -p panini-prakriya`

Expected: all pass. If `clippy` later complains about an unused re-export, that means a stage extraction has since removed the last user — deal with it in that task, not here.

- [ ] **Step 4: Run the full suite and lint**

Run: `mise run test && mise run lint`

Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "refactor(prakriya): extract tinanta/terms.rs

The index constants, the 3.1.68-bisection NOTE and the empty-śap caveat now
live next to the one function that encapsulates them, where a new rule's
author will find them."
```

---

## Task 6: Extract `sound.rs`

**Files:**
- Create: `crates/panini-prakriya/src/tinanta/sound.rs`
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs`

**Interfaces:**
- Produces: `pub(crate) fn guna_of(v: char) -> Option<&'static str>`, `vrddhi_of(v: char) -> Option<char>`, `is_vowel(c: char) -> bool`, `is_jhal(c: char) -> bool`, `is_khar(c: char) -> bool`, `cartva_of(c: char) -> Option<char>`, `is_vibhakti_protected_final(c: char) -> bool`.

- [ ] **Step 1: Create the file**

Move the seven functions with their doc comments verbatim from the top of `mod.rs` (currently lines ~9–95) into `crates/panini-prakriya/src/tinanta/sound.rs`, each made `pub(crate)`. Add at the top:

```rust
//! Varṇa and pratyāhāra classification: the sound layer the rules stand on.
//!
//! Pure functions over SLP1 characters, with no knowledge of terms, tags or
//! derivation state. Several are deliberately narrower than the pratyāhāra
//! they name — each says so, with the trigger for widening it.
```

- [ ] **Step 2: Move their unit tests**

Move these three test functions verbatim from `mod.rs`'s `mod tests` into a `#[cfg(test)] mod tests` at the bottom of `sound.rs`, with `use super::*;`:

- `guna_of_ik_vowels_all_arms`
- `vrddhi_of_ac_vowels_all_arms`
- `is_vowel_distinguishes_vowels_from_consonants`

- [ ] **Step 3: Wire it into `mod.rs`**

```rust
mod sound;

pub(crate) use sound::{
    cartva_of, guna_of, is_jhal, is_khar, is_vibhakti_protected_final, is_vowel, vrddhi_of,
};
```

- [ ] **Step 4: Test and lint**

Run: `mise run test && mise run lint`

Expected: clean, all pass.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "refactor(prakriya): extract tinanta/sound.rs

The varṇa/pratyāhāra classifiers are a layer below the rules and were already
tested as one; give them a file so no stage has to arbitrarily own them."
```

---

## Task 7: The stage-list mechanism, proved on `tripadi.rs`

This is the only task in Group 2 that changes a type. It extracts the smallest stage (6 rules) so the mechanism is proved end-to-end before the large moves.

**Files:**
- Create: `crates/panini-prakriya/src/tinanta/tripadi.rs`
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs`, `crates/panini-prakriya/src/controller.rs`

**Interfaces:**
- Produces:
  - `pub(crate) static TRIPADI: &[Rule]` in `tripadi.rs`
  - `pub static TINANTA_RULES: &[&[Rule]]` in `mod.rs` (type changed from `&[Rule]`)
  - `pub fn rules() -> impl Iterator<Item = &'static Rule>` in `mod.rs`
  - `pub fn run_pipeline(p: &mut Prakriya, stages: &[&[Rule]])` in `controller.rs` (signature changed)

- [ ] **Step 1: Check for external users of `TINANTA_RULES`**

Run: `grep -rn "TINANTA_RULES" crates/ --include=*.rs`

Expected: hits only inside `crates/panini-prakriya/src/tinanta/mod.rs`, plus a prose mention in `crates/panini/tests/trace.rs`'s header comment (Task 13 updates that). If any other crate uses it as a `&[Rule]`, stop and report — the spec assumed there were none.

- [ ] **Step 2: Change `run_pipeline`**

In `crates/panini-prakriya/src/controller.rs`, replace the function with:

```rust
/// Apply each stage in order, and each rule within a stage in order, at most
/// once. Rules self-guard via `apply` returning false when inapplicable.
/// Ordering is the controller's concern.
///
/// Stages are a file-organisation boundary, not a grammatical one: the
/// flattened sequence is what the grammar is, and it must read the same as it
/// did when the rules lived in a single array.
pub fn run_pipeline(p: &mut Prakriya, stages: &[&[Rule]]) {
    for stage in stages {
        for rule in *stage {
            if p.blocked {
                return;
            }
            (rule.apply)(p);
        }
    }
}
```

Update its own unit test's call site — `run_pipeline(&mut p, &rules);` becomes:

```rust
        run_pipeline(&mut p, &[&rules[..]]);
```

- [ ] **Step 3: Create `tripadi.rs`**

Move the last six rules — **8.2.77, 8.2.23, 8.2.25, 8.3.15, 8.3.59, 8.4.55** (currently `mod.rs` from the comment block introducing 8.2.77 at ~`:1610` through the closing `},` of 8.4.55 at ~`:1853`) — verbatim into `crates/panini-prakriya/src/tinanta/tripadi.rs`:

```rust
//! Tripādī: 8.2.77 … 8.4.55.
//!
//! Ordered AFTER 3.1.68, so the ending is at `ENDING` (index 2) and śap at
//! `SHAP` (index 1); `terms[SHAP].text` may be empty (2.4.72). See
//! `super::terms`.

use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::term::Tag;
use crate::tinanta::sound::{cartva_of, is_jhal, is_khar};
use crate::tinanta::terms::{ANGA, ENDING, SHAP};

pub(crate) static TRIPADI: &[Rule] = &[
    // … the six rules, moved verbatim …
];
```

**Import list caveat:** the `use` lines above are a starting point, not gospel. After moving the rules, let `cargo build` tell you exactly which items are missing or unused and adjust. The same applies to every stage file in Tasks 8–12 — each lists a plausible import set that you must reconcile against the compiler. Do not guess and leave it; `mise run lint` fails on unused imports.

- [ ] **Step 4: Convert `TINANTA_RULES` to a stage list**

In `mod.rs`, the remaining rule array (everything from 1.3.12 through 6.4.101) becomes a private stage, and the public item becomes the list:

```rust
/// Every rule not yet extracted into its own stage file. Shrinks to nothing
/// as the split proceeds; delete this constant when the last stage moves out.
static UNSPLIT: &[Rule] = &[
    // … the remaining 50 rules, unchanged …
];

/// The ordered rule list, as a sequence of pipeline stages. Read the stages
/// in order, and the rules within each stage in order: that flattened
/// sequence IS the grammar this crate implements. Every rule self-guards and
/// returns whether it fired.
pub static TINANTA_RULES: &[&[Rule]] = &[UNSPLIT, tripadi::TRIPADI];

/// The rules in pipeline order, flattened across stages.
pub fn rules() -> impl Iterator<Item = &'static Rule> {
    TINANTA_RULES.iter().flat_map(|stage| stage.iter())
}
```

Add `mod tripadi;` alongside the other module declarations.

- [ ] **Step 5: Update every `TINANTA_RULES.iter()` call site in tests**

In `mod.rs`'s `mod tests`, replace every `TINANTA_RULES.iter()` with `rules()`. There are roughly forty, of three shapes:

```rust
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.3.84").unwrap();
```
becomes
```rust
        let rule = rules().find(|r| r.id == "7.3.84").unwrap();
```

in `tinanta_rule_order_is_pinned` (Task 3) and `recorded_step_names_match_tinanta_rules_for_every_id`:

```rust
        let actual: Vec<&str> = rules().map(|r| r.id).collect();
```

and — **the subtle one** — in `sarvadhatukam_apit_second_application_single_term_does_not_panic`, which targets the *second* 1.2.4 entry positionally:

```rust
        assert_eq!(rules().filter(|r| r.id == "1.2.4").count(), 2, "…");
        let rule = rules().filter(|r| r.id == "1.2.4").nth(1).unwrap();
```

This test is the reason `rules()` must preserve order across stages, and it keeps working only because it does. Note also that a plain `.find(|r| r.id == "1.2.4")` anywhere returns the **first** application (the ātmanepada-ending one in `samjna`), never the vikaraṇa one.

Find them all with: `grep -n "TINANTA_RULES" crates/panini-prakriya/src/tinanta/mod.rs`

- [ ] **Step 6: Move the tripādī guard tests**

The rule for the whole split, applied in every extraction task from here on:

- A test that builds a `Prakriya` **by hand** and calls `(rule.apply)(&mut p)` → goes to the stage file owning that rule.
- A test that calls `derive`, `form`, `form_g`, `lin_form` or a `*_a_form` helper and asserts a **surface form or a trace** → stays in `mod.rs` for now; Task 12 sends it to `derivation_tests.rs`. This holds even when the test's name mentions a sūtra: `cartva_turns_d_to_t_before_khar` names 8.4.55 but asserts derived forms, so it stays.

Regenerate the authoritative mapping at any point with:

```bash
awk '/^    fn /{n=$2} /rules\(\)\.find|TINANTA_RULES\.iter\(\)\.find/{split($0,a,"\""); if(a[2]!="") print a[2]"\t"n}' \
  crates/panini-prakriya/src/tinanta/mod.rs | sort
```

For **tripādī**, move these three verbatim into a `#[cfg(test)] mod tests` at the bottom of `tripadi.rs`:

- `hali_ca_two_char_anga_still_fires` (8.2.77)
- `hali_ca_uses_n_minus_2_not_n_over_2` (8.2.77)
- `shatva_declines_for_every_pre_existing_junction` (8.3.59)

with the preamble:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::term::Term;
    use crate::tinanta::rules;
```

The other 8.x-named tests (`cartva_turns_d_to_t_before_khar`, `cartva_guard_is_khar_only_not_m_or_vowel`, `dhi_ca_*`, `her_dhih_*`, `voiced_junction_*`, `shatva_retroflexes_the_endings_s_after_shings_e`) go through `derive` or a `form` helper — leave them where they are.

- [ ] **Step 7: Test**

Run: `mise run test`

Expected: all pass. **`tinanta_rule_order_is_pinned` passing is the proof that the mechanism preserves order** — if it fails, the moved block was cut at the wrong boundary.

- [ ] **Step 8: Lint and format**

Run: `mise run fmt && mise run lint`

- [ ] **Step 9: Commit**

```bash
git add -A
git commit -m "refactor(prakriya): TINANTA_RULES becomes a stage list; extract tripadi

run_pipeline now walks stages then rules. The flattened sequence is unchanged
and pinned by tinanta_rule_order_is_pinned. Everything stays static — no
LazyLock, no allocation — and the stage boundary is now visible in the type."
```

---

## Task 8: Extract `adesha.rs`

**Files:**
- Create: `crates/panini-prakriya/src/tinanta/adesha.rs`
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs`

**Interfaces:**
- Produces: `pub(crate) static ADESHA: &[Rule]` — 8 rules.

- [ ] **Step 1: Move the rules**

Move **6.1.101, 6.1.96, 6.1.90, 6.1.97, 6.1.87, 6.1.66, 6.4.105, 6.4.101** — from the comment block introducing 6.1.101 through the closing `},` of 6.4.101 — verbatim into the new file:

```rust
//! Ādeśa and sandhi: 6.1.101 … 6.4.101.
//!
//! Ordered AFTER 3.1.68 — ending at `ENDING`, śap at `SHAP`, and
//! `terms[SHAP].text` may be empty (2.4.72). See `super::terms`.
//!
//! Three rules here (6.1.90 āṭaś ca, 6.1.66 lopo vyor vali, and 6.1.78 over
//! in `super::anga`) carry explicit *athematic arms* for the śap-luk'd path.
//! Those arms duplicate a follower lookup on purpose: each is pinned by its
//! own `*_athematic_*` guard tests asserting disjointness from its thematic
//! arm, and funnelling them through one shared helper would collapse three
//! independent mutation pins into one.

use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use crate::tinanta::sound::{is_vowel, vrddhi_of};
use crate::tinanta::terms::{ANGA, ENDING, SHAP};

pub(crate) static ADESHA: &[Rule] = &[
    // … the eight rules, moved verbatim …
];
```

Reconcile the imports against the compiler.

- [ ] **Step 2: Wire into `mod.rs`**

Add `mod adesha;` and put it in the stage list **before** `tripadi`:

```rust
pub static TINANTA_RULES: &[&[Rule]] = &[UNSPLIT, adesha::ADESHA, tripadi::TRIPADI];
```

- [ ] **Step 3: Move the guard tests**

Move verbatim into a `#[cfg(test)] mod tests` in `adesha.rs`, using the same preamble as Task 7 Step 6:

- `vali_lopa_spares_a_following_vowel` (6.1.66)
- `lopo_vyor_vali_athematic_arm_requires_an_empty_shap` (6.1.66)
- `awas_ca_ending_arm_requires_a_third_term` (6.1.90)
- `awas_ca_athematic_arm_requires_a_third_term` (6.1.90)
- `awas_ca_athematic_arm_requires_an_empty_shap` (6.1.90)
- `usyapadantat_drops_a_before_us_and_spares_iyus` (6.1.96)
- `usyapadantat_uses_n_minus_3_not_n_over_3` (6.1.96)
- `akah_savarne_dirghah_adadi_arm_two_term_anga_does_not_panic` (6.1.101)
- `savarna_dirgha_adadi_lin_1sg_arm` (6.1.101)
- `savarna_dirgha_adadi_lin_1sg_arm_two_term_prakriya_does_not_panic` (6.1.101)

Re-run the mapping command from Task 7 Step 6 to confirm no 6.1.x / 6.4.10x test was missed.

- [ ] **Step 4: Test, lint, commit**

Run: `mise run test && mise run fmt && mise run lint`

Expected: all pass, `tinanta_rule_order_is_pinned` included.

```bash
git add -A
git commit -m "refactor(prakriya): extract tinanta/adesha.rs (6.1.101 … 6.4.101)"
```

---

## Task 9: Extract `anga.rs`

The largest stage: 14 rules plus the `pugantalaghupadhasya_*` and `eco_yavayavah_athematic_*` guard blocks.

**Files:**
- Create: `crates/panini-prakriya/src/tinanta/anga.rs`
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs`

**Interfaces:**
- Produces: `pub(crate) static ANGA_RULES: &[Rule]` — 14 rules. **Named `ANGA_RULES`, not `ANGA`,** because `ANGA` is already the index constant in `terms.rs` and this file imports it.

- [ ] **Step 1: Move the rules**

Move **6.4.71, 6.4.72, 7.3.100, 7.1.5, 7.1.6, 7.1.3, 7.2.79, 7.2.80, 7.2.81, 7.4.21, 7.3.84, 7.3.86, 6.1.78, 7.3.101** — from the comment block introducing 6.4.71 through the closing `},` of 7.3.101 — verbatim into:

```rust
//! Aṅga operations: 6.4.71 … 7.3.101.
//!
//! Ordered AFTER 3.1.68 — ending at `ENDING`, śap at `SHAP`, and
//! `terms[SHAP].text` may be empty (2.4.72). See `super::terms`.
//!
//! 6.1.78 *eco'yavāyāvaḥ* sits in this stage rather than with the other
//! 6.1.x rules in `super::adesha` because that is where the pipeline order
//! puts it, between 7.3.86 and 7.3.101. Order outranks sūtra family: the
//! flattened sequence is the grammar.

use crate::context::Context;
use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use crate::tinanta::sound::{guna_of, is_vowel};
use crate::tinanta::terms::{ANGA, ENDING, SHAP, following_sarvadhatuka};
use panini_data::Lakara;

pub(crate) static ANGA_RULES: &[Rule] = &[
    // … the fourteen rules, moved verbatim …
];
```

Reconcile the imports against the compiler — this stage's rules read `p.ctx.lakara`, `p.ctx.purusha` and several tags, so the set above is a starting point only.

- [ ] **Step 2: Wire into `mod.rs`**

```rust
pub static TINANTA_RULES: &[&[Rule]] =
    &[UNSPLIT, anga::ANGA_RULES, adesha::ADESHA, tripadi::TRIPADI];
```

- [ ] **Step 3: Move the guard tests**

Move verbatim into `anga.rs`'s test module:

- `rut_requires_both_shing_and_a_fired_seven_one_five` (7.1.6)
- `salopa_elides_only_the_non_final_s` (7.2.79)
- `ato_yeyah_rewrites_the_ya_prefix_after_shap_a` (7.2.80)
- `ato_yeyah_requires_vidhilin_lakara` (7.2.80)
- `ato_yeyah_requires_shap_a` (7.2.80)
- `ato_nitah_requires_the_ngit_tag` (7.2.81)
- `shings_guna_leaves_every_other_adadi_root_alone` (7.4.21)
- `sarvadhatukardhadhatukayoh_blocks_guna_when_vikarana_is_ngit` (7.3.84)
- `sarvadhatukardhadhatukayoh_single_term_anga_still_applies_guna` (7.3.84)
- `sarvadhatukardhadhatukayoh_blocks_guna_when_luk_shap_ending_is_ngit` (7.3.84, from Task 1)
- `sarvadhatukardhadhatukayoh_applies_guna_when_luk_shap_ending_is_not_ngit` (7.3.84, from Task 1)
- `pugantalaghupadhasya_one_char_anga_returns_false_without_panic` (7.3.86)
- `pugantalaghupadhasya_two_char_ik_penult_fires` (7.3.86)
- `pugantalaghupadhasya_skips_vowel_final_anga` (7.3.86)
- `pugantalaghupadhasya_uses_n_minus_2_not_n_over_2` (7.3.86)
- `pugantalaghupadhasya_single_term_still_applies_guna` (7.3.86)
- `pugantalaghupadhasya_blocks_guna_when_luk_shap_ending_is_ngit` (7.3.86, from Task 1)
- `pugantalaghupadhasya_applies_guna_when_luk_shap_ending_is_not_ngit` (7.3.86, from Task 1)
- `eco_yavayavah_athematic_arm_requires_a_third_term` (6.1.78)
- `eco_yavayavah_athematic_arm_requires_an_empty_shap` (6.1.78)

Leave in `mod.rs` for Task 12: `eco_yavayavah_athematic_arm_produces_the_ay_adesha`, `shi_takes_guna_despite_the_ngit_ending`, `shings_jha_takes_the_rut_augment`, `shings_vidhilin_3pl_takes_no_rut`, `vrt_lat_uses_laghupadha_guna`, `siyut_survives_salopa_as_long_i`, `seventwone_five_atmanepada_3pl_uses_at_not_ant`, `anatah_declines_for_a_final_atmanepada_angas`. Every one of these goes through `derive` or a `form` helper and asserts a surface form or a trace, so by the Task 7 Step 6 rule they are whole-derivation tests despite naming a 7.x sūtra.

- [ ] **Step 4: Test, lint, commit**

Run: `mise run test && mise run fmt && mise run lint`

```bash
git add -A
git commit -m "refactor(prakriya): extract tinanta/anga.rs (6.4.71 … 7.3.101)"
```

- [ ] **Step 5: Measure it**

Run: `wc -l crates/panini-prakriya/src/tinanta/*.rs`

If `anga.rs` is well past ~700 lines, note the count in your report. The natural follow-up seam is 6.4.71–7.2.81 vs 7.4.21–7.3.101 (the guṇa cluster plus 6.1.78). **Do not split it in this task** — report the number and let the reviewer decide.

---

## Task 10: Extract `vikarana.rs`

**Files:**
- Create: `crates/panini-prakriya/src/tinanta/vikarana.rs`
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs`

**Interfaces:**
- Produces: `pub(crate) static VIKARANA: &[Rule]` — 5 rules.

- [ ] **Step 1: Move the rules**

Move **3.1.69, 3.1.77, 3.1.68, 2.4.72, 1.2.4** (the *second* 1.2.4, the one that tags the vikaraṇa) — from the comment block introducing 3.1.69 through the closing `},` of that 1.2.4 — verbatim into:

```rust
//! Vikaraṇa selection and luk: 3.1.69, 3.1.77, 3.1.68, 2.4.72, 1.2.4.
//!
//! **This stage contains the 3.1.68 boundary.** Rules before 3.1.68 in this
//! file address the ending as `ENDING_PRE_SHAP` (index 1); rules after it use
//! `ENDING` (index 2) and may use `SHAP`. Get this wrong and a rule mutates
//! śap while believing it is mutating the ending, or panics indexing a slot
//! that does not exist yet. See `super::terms`.
//!
//! 2.4.72 luks śap by emptying its text in place rather than removing the
//! term, which is what keeps every later index valid — and what makes
//! `terms[SHAP].text` possibly empty for the rest of the pipeline.

use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use crate::tinanta::terms::{ANGA, ENDING_PRE_SHAP, SHAP};

pub(crate) static VIKARANA: &[Rule] = &[
    // … the five rules, moved verbatim …
];
```

Reconcile imports against the compiler.

- [ ] **Step 2: Wire into `mod.rs`**

```rust
pub static TINANTA_RULES: &[&[Rule]] = &[
    UNSPLIT,
    vikarana::VIKARANA,
    anga::ANGA_RULES,
    adesha::ADESHA,
    tripadi::TRIPADI,
];
```

- [ ] **Step 3: Move the guard tests**

- `kartari_sap_single_term_anga_does_not_panic` (3.1.68)
- `adiprabhrtibhyah_sapah_single_term_anga_does_not_panic` (2.4.72)
- `sarvadhatukam_apit_second_application_single_term_does_not_panic` (1.2.4, second — the `.filter(…).nth(1)` locator from Task 7 Step 5; move its `assert_eq!(…count(), 2, …)` guard with it, since that assertion is what makes `nth(1)` safe)

Leave `div_lengthens_before_syan` and `adadi_luk_present_no_junction_cells` in `mod.rs` — they assert forms.

- [ ] **Step 4: Test, lint, commit**

Run: `mise run test && mise run fmt && mise run lint`

```bash
git add -A
git commit -m "refactor(prakriya): extract tinanta/vikarana.rs (contains the 3.1.68 boundary)"
```

---

## Task 11: Extract `tin.rs`

**Files:**
- Create: `crates/panini-prakriya/src/tinanta/tin.rs`
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs`

**Interfaces:**
- Produces: `pub(crate) static TIN: &[Rule]` — 18 rules.

- [ ] **Step 1: Move the rules**

Move **3.4.85, 3.4.108, 3.4.105, 3.4.106, 3.4.101, 3.4.99, 3.4.87, 3.4.89, 3.4.86, 3.4.100, 3.4.80, 3.4.79, 3.4.91, 3.4.93, 3.4.90, 3.4.92, 3.4.103, 3.4.102** — from the comment block introducing 3.4.85 through the closing `},` of 3.4.102 — verbatim into:

```rust
//! Lakāra → tiṅ substitution and ending reshaping: 3.4.85 … 3.4.102.
//!
//! Ordered **BEFORE** 3.1.68, so every rule here addresses the ending as
//! `ENDING_PRE_SHAP` (index 1) — śap does not exist yet, and `ENDING`
//! (index 2) would panic. See `super::terms`.
//!
//! The split from `super::samjna` falls at 3.4.78, which is what *inserts*
//! the ending; everything from 3.4.85 on substitutes and reshapes it.

use crate::context::Context;
use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use crate::tinanta::terms::{ANGA, ENDING_PRE_SHAP};
use panini_data::{Lakara, Pada, Purusha, Vacana};

pub(crate) static TIN: &[Rule] = &[
    // … the eighteen rules, moved verbatim …
];
```

Reconcile imports against the compiler.

- [ ] **Step 2: Wire into `mod.rs`**

```rust
pub static TINANTA_RULES: &[&[Rule]] = &[
    UNSPLIT,
    tin::TIN,
    vikarana::VIKARANA,
    anga::ANGA_RULES,
    adesha::ADESHA,
    tripadi::TRIPADI,
];
```

- [ ] **Step 3: Move the guard tests**

- `thasah_se_precedes_and_preempts_ter_e` (3.4.79)
- `aduttamasya_requires_uttama_purusha` (3.4.92)
- `eta_ai_requires_ending_to_actually_end_in_e` (3.4.93)
- `itash_ca_fires_for_vidhilin` (3.4.100)
- `itash_ca_never_touches_lot_even_when_ngit_like` (3.4.100)
- `mip_becomes_am_in_vidhilin` (3.4.101)
- `yasut_prefixes_the_substituted_ending` (3.4.103)
- `yasut_is_vidhilin_only` (3.4.103)
- `jher_jus_replaces_ji_and_elides_the_j_marker` (3.4.108)
- `jher_jus_leaves_lat_and_lot_ji_alone` (3.4.108)
- `itash_ca_and_yasut_are_parasmaipada_only` — looks up 3.4.100 and 3.4.103 together, both in this stage

Leave `savabhyam_vamau_preempts_am_etah` (3.4.91) and `am_etah_is_lot_only` (3.4.90) in `mod.rs` — both go through a `form` helper and assert forms, so Task 12 sends them to `derivation_tests.rs`.

- [ ] **Step 4: Test, lint, commit**

Run: `mise run test && mise run fmt && mise run lint`

```bash
git add -A
git commit -m "refactor(prakriya): extract tinanta/tin.rs (3.4.85 … 3.4.102)"
```

---

## Task 12: Extract `samjna.rs` and `derivation_tests.rs`; empty `mod.rs`

**Files:**
- Create: `crates/panini-prakriya/src/tinanta/samjna.rs`, `crates/panini-prakriya/src/tinanta/derivation_tests.rs`
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs`

**Interfaces:**
- Produces: `pub(crate) static SAMJNA: &[Rule]` — 5 rules; `pub(crate)` test helpers `form`, `form_g`, `lin_form`, `lat_a_form`, `lot_a_form`, `lin_a_form` in `derivation_tests.rs`.

- [ ] **Step 1: Move the last five rules**

Move **1.3.12, 1.3.78, 3.4.78, 1.3.9, 1.2.4** (the *first* 1.2.4) verbatim into:

```rust
//! Saṃjñā, pada sanction and ending insertion: 1.3.12, 1.3.78, 3.4.78,
//! 1.3.9, 1.2.4.
//!
//! Ordered **BEFORE** 3.1.68 — the ending lives at `ENDING_PRE_SHAP`
//! (index 1) and śap does not exist yet. See `super::terms`.
//!
//! 3.4.78 is what inserts the ending; `super::tin` picks up from 3.4.85 and
//! reshapes it. 1.2.4 appears here tagging apit ātmanepada endings ṅit, and
//! again in `super::vikarana` tagging the apit vikaraṇa once it exists.

use crate::context::Context;
use crate::it_samjna::run_it_samjna;
use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use crate::tinanta::sound::is_vibhakti_protected_final;
use crate::tinanta::terms::{ANGA, ENDING_PRE_SHAP};
use panini_data::{Lakara, Pada, Purusha, tin_ending};

pub(crate) static SAMJNA: &[Rule] = &[
    // … the five rules, moved verbatim …
];
```

Reconcile imports against the compiler.

- [ ] **Step 2: Move the saṃjñā guard tests into `samjna.rs`**

- `it_samjna_rule_reports_when_ending_is_reduced` (1.3.9)
- `sarvadhatukam_apit_tags_atmanepada_endings_ngit` (1.2.4, first)
- `sarvadhatukam_apit_skips_parasmaipada_and_lot_uttama` (1.2.4, first)

Leave `pada_sanction_blocks_wrong_pada_derivations` and `pada_sanction_records_the_sanctioning_sutra` in the derivation set — both call `derive` and assert on the resulting trace, so they belong in `derivation_tests.rs` (Step 3 below).

- [ ] **Step 3: Create `derivation_tests.rs` with everything left**

Move the *entire remaining* `mod tests` body from `mod.rs` into `crates/panini-prakriya/src/tinanta/derivation_tests.rs`. Its header:

```rust
//! Whole-derivation tests: cases that assert a surface form, a trace, or an
//! invariant over the pipeline as a whole, rather than one rule's guard.
//!
//! Per-rule guard tests live beside their rule in the stage files. If a test
//! you are adding calls `(rule.apply)(&mut p)` on a hand-built prakriya, it
//! belongs there, not here.

use super::*;
use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};
```

Mark the six shared helpers `pub(crate)` so stage test modules can import them:

```rust
pub(crate) fn form(code: &str, pu: Purusha, va: Vacana) -> String { … }
pub(crate) fn form_g(code: &str, la: Lakara, pu: Purusha, va: Vacana) -> String { … }
pub(crate) fn lin_form(code: &str, pu: Purusha, va: Vacana) -> String { … }
pub(crate) fn lat_a_form(code: &str, pu: Purusha, va: Vacana) -> String { … }
pub(crate) fn lot_a_form(code: &str, pu: Purusha, va: Vacana) -> String { … }
pub(crate) fn lin_a_form(code: &str, pu: Purusha, va: Vacana) -> String { … }
```

Note `lin_form` and `lin_a_form` are currently nested inside the test module at different points — hoist both to the module's top level next to the others.

Declare it from `mod.rs`:

```rust
#[cfg(test)]
pub(crate) mod derivation_tests;
```

Any stage test that needs a helper imports it, e.g.
`use crate::tinanta::derivation_tests::form_g;`

- [ ] **Step 4: Reduce `mod.rs` and delete `UNSPLIT`**

`mod.rs` should now contain only: module declarations, the `pub(crate) use` re-exports, `TINANTA_RULES`, `rules()`, `derive`, and the `#[cfg(test)] pub(crate) mod derivation_tests;` line. Delete the now-empty `UNSPLIT` constant and its comment.

```rust
pub static TINANTA_RULES: &[&[Rule]] = &[
    samjna::SAMJNA,
    tin::TIN,
    vikarana::VIKARANA,
    anga::ANGA_RULES,
    adesha::ADESHA,
    tripadi::TRIPADI,
];
```

Then prune the `pub(crate) use` re-exports from Tasks 5 and 6 down to what `mod.rs` itself still uses (probably nothing, now that all rules have left) — `mise run lint` will tell you, since `-D warnings` fails on unused imports.

- [ ] **Step 5: Test**

Run: `mise run test`

Expected: all pass. `tinanta_rule_order_is_pinned` is now checking the full six-stage flattening against the original 56-id sequence — this is the moment it earns its keep.

- [ ] **Step 6: Measure**

Run: `wc -l crates/panini-prakriya/src/tinanta/*.rs`

Expected: no file much over ~700 lines. Report the actual numbers.

- [ ] **Step 7: Lint, format, commit**

Run: `mise run fmt && mise run lint`

```bash
git add -A
git commit -m "refactor(prakriya): extract samjna + derivation_tests; tinanta/ split complete

mod.rs is now the pipeline's table of contents: six stages, derive, rules().
Per-rule guard tests live beside their rules; whole-derivation tests and the
shared helpers live in derivation_tests.rs."
```

---

## Task 13: Update the documentation that names the old file

**Files:**
- Modify: `docs/ARCHITECTURE.md:22`, `crates/panini/tests/trace.rs:16`, `AGENTS.md`

- [ ] **Step 1: Confirm nothing else points at the old path**

Run: `grep -rn "tinanta\.rs" . --include=*.md --include=*.rs --include=*.toml | grep -v '^./target' | grep -v docs/superpowers/plans | grep -v docs/superpowers/specs`

Historical plans and specs keep their original wording — they are dated records. Everything else must be updated.

- [ ] **Step 2: Update `docs/ARCHITECTURE.md`**

Replace the paragraph at `:22`:

```markdown
`TINANTA_RULES` (in `crates/panini-prakriya/src/tinanta.rs`) is a single
ordered `&[Rule]` covering all four lakāras. Each rule self-guards on
`Prakriya.ctx` (lakāra, pada, puruṣa, vacana) and returns whether it fired.
Reading the list top to bottom IS reading the grammar this crate implements.
```

with:

```markdown
`TINANTA_RULES` (in `crates/panini-prakriya/src/tinanta/mod.rs`) is an
ordered `&[&[Rule]]` — six pipeline stages, each in its own file — covering
all four lakāras. Each rule self-guards on `Prakriya.ctx` (lakāra, pada,
puruṣa, vacana) and returns whether it fired. Reading the stages in order,
and the rules within each stage in order, IS reading the grammar this crate
implements; `tinanta::rules()` yields that flattened sequence.

| stage file | rules | position |
|---|---|---|
| `samjna.rs` | 1.3.12, 1.3.78, 3.4.78, 1.3.9, 1.2.4 | before 3.1.68 |
| `tin.rs` | 3.4.85 … 3.4.102 | before 3.1.68 |
| `vikarana.rs` | 3.1.69, 3.1.77, 3.1.68, 2.4.72, 1.2.4 | contains 3.1.68 |
| `anga.rs` | 6.4.71 … 7.3.101 (incl. 6.1.78) | after 3.1.68 |
| `adesha.rs` | 6.1.101 … 6.4.101 | after 3.1.68 |
| `tripadi.rs` | 8.2.77 … 8.4.55 | after 3.1.68 |

The stage boundary is file organisation, not grammar: the flattened order is
what matters, and `tinanta_rule_order_is_pinned` in `derivation_tests.rs`
pins all 56 ids verbatim. `tinanta/terms.rs` holds the term-index constants
and the reason 3.1.68 bisects the pipeline; `tinanta/sound.rs` holds the
varṇa classifiers.
```

- [ ] **Step 3: Update `crates/panini/tests/trace.rs`**

In the header comment at `:16`, replace:

```
//! read `TINANTA_RULES` in `crates/panini-prakriya/src/tinanta.rs` top to
//! bottom. That static array — not this comment — is the source of truth for
//! sequencing;
```

with:

```
//! read `TINANTA_RULES` in `crates/panini-prakriya/src/tinanta/mod.rs` and
//! then its six stage files in that order. That flattened static sequence —
//! not this comment — is the source of truth for sequencing;
```

This is a comment-only change; no test logic moves.

- [ ] **Step 4: Update `AGENTS.md`**

Replace the bullet:

```markdown
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, not as a
  branch inside `derive`.
```

with:

```markdown
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, not as a
  branch inside `derive`. `TINANTA_RULES` is a list of six stage files under
  `crates/panini-prakriya/src/tinanta/`; add the rule to the stage its
  pipeline position falls in, and add its id to `tinanta_rule_order_is_pinned`
  in the same position. Which stage a rule belongs to is decided by its
  position relative to **3.1.68**, not by its sūtra family: rules before
  3.1.68 address the ending as `ENDING_PRE_SHAP` (index 1), rules after it as
  `ENDING` (index 2), and `terms[SHAP].text` may be empty for adādi. See
  `tinanta/terms.rs`. Per-rule guard tests go beside the rule in its stage
  file; tests asserting a surface form or trace go in
  `tinanta/derivation_tests.rs`.
```

Keep the rest of that bullet (the sentence about `derive` carrying no grammar branches, only aṅga tagging) unchanged.

- [ ] **Step 5: Verify and commit**

Run: `mise run test`

Expected: pass (the `trace.rs` change is a comment, but run it anyway).

```bash
git add -A
git commit -m "docs: point ARCHITECTURE.md, trace.rs and AGENTS.md at the tinanta/ stages"
```

---

## Task 14: Full gate

**Files:** none, unless something fails.

- [ ] **Step 1: Format check**

Run: `mise run fmt-check`

Expected: clean.

- [ ] **Step 2: Lint**

Run: `mise run lint`

Expected: clean, `-D warnings`.

- [ ] **Step 3: Full test suite**

Run: `mise run test`

Expected: all pass — 1080 golden forms, the ordered traces, the rule-order pin, every stage's guard tests.

- [ ] **Step 4: Supply chain**

Run: `mise run audit`

Expected: `cargo audit` and `cargo deny check` both clean, advisories included.

- [ ] **Step 5: Mutation gate**

Run:

```bash
/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants \
  --package panini-prakriya --test-workspace=true --timeout 60
```

Expected: **0 survivors.** This is a long run; let it finish.

If a survivor appears, it will most likely be in code this slice *moved* rather than wrote — a test that was killing it may have been left behind in the wrong file, or a `use` reshuffle may have changed which tests compile. Find the test that used to kill it and check it still runs. **Do not park a mutant with a comment**; if a mutant is genuinely unkillable, the code it lives in is unreachable and should be deleted, per this project's standing rule.

- [ ] **Step 6: Confirm the diff is a move, not a rewrite**

Run: `git diff --stat main...HEAD` and `git diff -M --summary main...HEAD`

Expected: `tinanta.rs` shows as renamed to `tinanta/mod.rs`, with the stage files as new. Spot-check two or three rules against `main` to confirm their bodies are unchanged:

```bash
git show main:crates/panini-prakriya/src/tinanta.rs | sed -n '/id: "8.3.59"/,/^    },/p'
sed -n '/id: "8.3.59"/,/^    },/p' crates/panini-prakriya/src/tinanta/tripadi.rs
```

Expected: identical rule bodies.

- [ ] **Step 7: Report**

Report to the reviewer:
- the `wc -l` table for `crates/panini-prakriya/src/tinanta/*.rs`
- confirmation that all 1080 goldens and every trace are unchanged from `main`
- the mutation result
- whether `anga.rs` warrants the follow-up split flagged in Task 9 Step 5

Do not open a PR without being asked.

---

## Notes for the executor

**The single most important invariant:** no derived form and no trace line changes, at any commit in this plan. `mise run test` after every task is not ceremony — it is the only thing standing between a "pure move" and a silent grammar change.

**If a golden test fails at any point:** stop and report. Do not edit the golden. This project treats an implementer editing a golden to match engine output as a red flag for a masked engine bug, and it has been right about that before.

**Moving code means moving it verbatim.** The comments in these rules carry rule-ordering rationale, mutation-gate reasoning, and restore triggers for deliberately narrow guards. They are not decoration. If a comment refers to something by its old location ("the rule below", "see 8.4.53's removal above"), fix the *reference*, and say so in the commit message — but never delete the comment to avoid the problem.
