# Juhotyādi gaṇa slice 3c2 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate √hā parasmaipada (`03.0009 o~hA\k`) and √gā (`03.0026 gA\`) into the juhotyādi gaṇa, taking the golden suite from 3852 to 3924 cells. This adds `Rule.bars` (the apavāda relation as data, enforced by `run_pipeline`) and four rules: 7.4.78, 6.4.118, and the vikalpas 6.4.117 and 6.4.116.

**Architecture:** Nine tasks.
- **Task 1** is controller plumbing (`Rule.bars`, `Prakriya.barred`). It declares no bar yet and moves no golden.
- **Tasks 2–4** add rules that fire on no curated root yet. Each is gated on **per-rule guard tests plus the 3852 priors staying byte-identical**.
- **Task 5** lands the two rows and their goldens, turning 72 cells green in one step.
- **Task 6** adds the trace pins.
- **Tasks 7–9** are the audit and doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.98.1 pinned via `mise`. `mise run build | test | test-full | lint | fmt | fmt-check | mutants | audit`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`.

**Spec:** `docs/superpowers/specs/2026-09-29-juhotyadi-gana-3c2-design.md`

## Global Constraints

- **The 3852 pre-existing cells must stay byte-identical through Tasks 1–4.** Regenerate no golden and change no pinned trace.
- **No unfalsifiable guard clauses.** A clause that no cell and no guard test can make false is a mutation survivor. Either witness it, delete it, or kill it with a direct guard test on a hand-built `Prakriya`.
- **Row identity:** every new rule keys on `p.ctx.dhatupatha`, never on `ANGA.text`.
  - `03.0008` and `03.0009` are both `hA`.
  - 7.4.78 is keyed on `03.0026`; 6.4.116, 6.4.117 and 6.4.118 on `03.0009`.
  - Each guard carries a comment naming the upadeśa.
- **Barring is declared, never guarded.** No rule may read `p.log` for `"6.4.117"`. 6.4.117 declares `bars: &["6.4.116", "6.4.113", "6.4.112"]`, and 6.4.116, 6.4.113 and 6.4.112 are left untouched.
- **Goldens are transcribed from this plan** (vidyut's output at `8da2f90b`, re-probed 2026-09-29), never invented. **Do not edit a golden to match the engine.**
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit.
- **`mise run test` takes ~70 s** (AGENTS.md, 3852 cells). Run it in the **foreground** with a timeout of at least 600000 ms; never background it and end a turn. `mise run test-full` takes ~25 minutes; same rule, timeout 3600000 ms.
- `mise run test -- -p X` does not scope. Scope unit tests with `mise exec -- cargo test -p <crate> <filter>`.
- Rule ids and SLP1 names, copied from vidyut's `sutrapatha.tsv`, are quoted identically everywhere:
  - `6.4.116 jahAteSca`
  - `6.4.117 A ca hO`
  - `6.4.118 lopo yi`
  - `7.4.78 bahulaM Candasi`

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-prakriya/src/rule.rs` | Task 1: `Rule.bars` |
| `crates/panini-prakriya/src/prakriya.rs` | Task 1: `Prakriya.barred` |
| `crates/panini-prakriya/src/controller.rs` | Task 1: enforcement + controller tests |
| every file with a `Rule` literal | Task 1: `bars: &[],` on all 121 (mechanical) |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Task 1: `exactly_the_pinned_bars`; Tasks 2–4: rule-order and vikalpa pins |
| `crates/panini-prakriya/src/tinanta/abhyasa.rs` | Task 2: 7.4.78 |
| `crates/panini-prakriya/src/tinanta/guna.rs` | Task 3: 6.4.118; Task 4: 6.4.117, 6.4.116 |
| `crates/panini-data/src/lib.rs` | Task 5: two `Dhatu` rows, the gaṇa-row test, doc-comment counts |
| `crates/panini/tests/paradigm/data/juhotyadi.rs` | Task 5: 8 `PARADIGM` blocks, 31 `ALTERNATES` rows |
| `crates/panini/tests/paradigm/main.rs` | Task 5: `VIKALPA_RULES`, totals, buckets, key census, prose |
| `crates/panini/tests/trace/juhotyadi.rs` | Task 6: trace pins |
| `tools/audit/`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, 3a's spec | Task 7 |
| `AGENTS.md`, `mise.toml` | Task 8: mutation record; the cap only if it must move |

---

## Task 1: `Rule.bars` — the apavāda relation, enforced by the controller

Plumbing only. Every rule gets `bars: &[]`, so no branch is ever barred and no golden moves.

**Files:**
- Modify: `crates/panini-prakriya/src/rule.rs` (the `Rule` struct)
- Modify: `crates/panini-prakriya/src/prakriya.rs` (the `Prakriya` struct)
- Modify: `crates/panini-prakriya/src/controller.rs` (`run_pipeline`; its `mod tests`)
- Modify: every `.rs` file under `crates/panini-prakriya/src` that has a `Rule` literal (121 of them)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (a new pin test)

**Interfaces:**
- Consumes: nothing.
- Produces: `Rule.bars: &'static [&'static str]`; `Prakriya.barred: Vec<&'static str>`. After a rule's `apply` returns true on a branch, `run_pipeline` extends that branch's `barred` with `rule.bars`, and it skips any rule whose `id` is in a branch's `barred`.

- [ ] **Step 1: Add the field to every `Rule` literal (mechanical)**

Every `Rule` literal has exactly one line matching `vikalpa: true,` or `vikalpa: false,`. The struct definition's `pub vikalpa: bool,` does not match. Insert `bars: &[],` after each one, at the same indentation:

```bash
cd crates/panini-prakriya/src
grep -rlE '^\s*vikalpa: (true|false),$' --include=*.rs . | xargs sed -i -E 's/^(\s*)vikalpa: (true|false),$/&\n\1bars: \&[],/'
grep -rcE '^\s*bars: &\[\],$' --include=*.rs . | awk -F: '{s+=$2} END {print s}'   # expect 121
cd -
```

Expected: `121`, one per `Rule` literal. (`grep "Rule {"` counts 124, but three of those hits are `pub struct Rule {` and two `-> &'static Rule {` return types.) The compiler is the real check: any literal the sed missed fails to build with `missing field \`bars\``, and you add the line by hand.

- [ ] **Step 2: Add the fields**

In `rule.rs`, after the `pub vikalpa: bool,` field and before `pub apply`, add:

```rust
    /// The apavāda relation, declared: rule ids this rule BARS on every branch
    /// where it fires. `run_pipeline` skips a barred rule on that branch, so the
    /// barred rules' own guards never have to know about their apavāda. For a
    /// vikalpa rule that is the applied clone only; the declined branch still
    /// runs every barred rule. 6.4.117 *ā ca hau* is the first user: it keeps
    /// √hā's `A` before *hi* by changing no text, and bars 6.4.116, 6.4.113 and
    /// 6.4.112, the three rules that would otherwise change that `A`.
    ///
    /// Scope is the BRANCH, not a site. A tinanta prakriyā has one aṅga, so the
    /// two coincide today; a pipeline with several sites must revisit this.
    ///
    /// Ids are strings, checked by `exactly_the_pinned_bars` in
    /// `tinanta/derivation_tests.rs`: every barred id must name a rule that
    /// runs AFTER its barrer, because barring an earlier rule does nothing.
    pub bars: &'static [&'static str],
```

In `prakriya.rs`, after the `pub blocked: bool,` field, add:

```rust
    /// Rule ids barred on this branch by a rule that already fired on it
    /// (`Rule.bars`). Cloned with the branch, so a vikalpa's applied and
    /// declined readings diverge here, and every later fork of a barred branch
    /// inherits the bar.
    pub barred: Vec<&'static str>,
```

Run `mise exec -- cargo build -p panini-prakriya 2>&1 | tail -20`. If any `Prakriya { … }` literal lists every field without `..Default::default()`, the compiler names it. Add `barred: Vec::new(),` there.

- [ ] **Step 3: Write the failing controller tests**

In `controller.rs`'s `mod tests`, after the `PUSH_M` const, add:

```rust
    /// A mandatory rule that pushes `b` and bars `x`.
    const BARS_X: Rule = Rule {
        id: "bx",
        name: "test",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &["x"],
        apply: |p| {
            let b = p.snapshot();
            p.terms[0].text.push('b');
            p.record("bx", "test", b);
            true
        },
    };

    /// The same, optional.
    const BARS_X_OPTIONALLY: Rule = Rule {
        id: "bxv",
        name: "test",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["x"],
        apply: |p| {
            let b = p.snapshot();
            p.terms[0].text.push('b');
            p.record("bxv", "test", b);
            true
        },
    };

    /// A barring rule that always declines its own guard.
    const BARS_X_BUT_DECLINES: Rule = Rule {
        id: "bxd",
        name: "test",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &["x"],
        apply: |_p| false,
    };
```

At the end of `mod tests`, add:

```rust
    #[test]
    fn a_barred_rule_is_skipped_on_the_barring_branch() {
        // Without the bar, PUSH_X would fork: ["ab", "abx"].
        let out = run_pipeline(p1("a"), &[&[BARS_X, PUSH_X][..]]);
        assert_eq!(texts(&out), vec!["ab"]);
        assert_eq!(out[0].barred, vec!["x"]);
    }

    #[test]
    fn an_optional_barrer_bars_its_applied_branch_only() {
        // Declined branch "a" still forks on PUSH_X; applied branch "ab" is barred.
        let out = run_pipeline(p1("a"), &[&[BARS_X_OPTIONALLY, PUSH_X][..]]);
        assert_eq!(texts(&out), vec!["a", "ax", "ab"]);
        assert!(out[0].barred.is_empty());
        assert_eq!(out[2].barred, vec!["x"]);
    }

    #[test]
    fn a_barring_rule_that_declines_bars_nothing() {
        let out = run_pipeline(p1("a"), &[&[BARS_X_BUT_DECLINES, PUSH_X][..]]);
        assert_eq!(texts(&out), vec!["a", "ax"]);
        assert!(out.iter().all(|b| b.barred.is_empty()));
    }

    #[test]
    fn a_bar_skips_only_the_rule_it_names() {
        let out = run_pipeline(p1("a"), &[&[BARS_X, PUSH_Y, PUSH_M][..]]);
        assert_eq!(texts(&out), vec!["abm", "abym"]);
    }

    #[test]
    fn forks_of_a_barred_branch_inherit_the_bar() {
        // bxv: [a, ab]. PUSH_Y forks both: [a, ay, ab, aby]. PUSH_X forks
        // only the unbarred two.
        let out = run_pipeline(p1("a"), &[&[BARS_X_OPTIONALLY, PUSH_Y, PUSH_X][..]]);
        assert_eq!(texts(&out), vec!["a", "ax", "ay", "ayx", "ab", "aby"]);
    }

    #[test]
    fn a_text_neutral_barring_fork_survives_once_a_barred_rule_diverges_it() {
        // 6.4.117's shape: an optional rule that changes no text and bars the
        // rule that would. Unlike `convergent_forks_collapse_to_the_declined_branch`,
        // the barred rule makes the two branches differ, so both survive,
        // declined first.
        const KEEP: Rule = Rule {
            id: "keep",
            name: "test",
            kind: RuleKind::Vidhi,
            vikalpa: true,
            bars: &["m"],
            apply: |p| {
                let b = p.snapshot();
                p.record("keep", "test", b);
                true
            },
        };
        let out = run_pipeline(p1("a"), &[&[KEEP, PUSH_M][..]]);
        assert_eq!(texts(&out), vec!["am", "a"]);
        let ids: Vec<&str> = out[1].log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, vec!["keep"]);
    }
```

- [ ] **Step 4: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya controller:: 2>&1 | tail -30`
Expected: four of the six new tests FAIL: `a_barred_rule_is_skipped_on_the_barring_branch`, `an_optional_barrer_bars_its_applied_branch_only`, `forks_of_a_barred_branch_inherit_the_bar` and `a_text_neutral_barring_fork_survives_once_a_barred_rule_diverges_it`. `barred` stays empty and barred rules still run. The other two, `a_barring_rule_that_declines_bars_nothing` and `a_bar_skips_only_the_rule_it_names`, already PASS. They guard against OVER-barring, and they are what kill the mutants that would bar on a declined rule or bar every rule. The existing controller tests PASS.

- [ ] **Step 5: Enforce the bar in `run_pipeline`**

Replace the per-branch loop body in `run_pipeline`:

```rust
            for (i, branch) in branches.iter_mut().enumerate() {
                if branch.blocked {
                    continue;
                }
                if rule.vikalpa {
                    …
                    if (rule.apply)(&mut applied) {
                        forks.push((i, applied));
                    }
                } else {
                    (rule.apply)(branch);
                }
            }
```

with this (the two comments inside the `vikalpa` arm are kept verbatim):

```rust
            for (i, branch) in branches.iter_mut().enumerate() {
                // A barred rule is skipped exactly as a blocked branch is:
                // an apavāda that fired earlier on this branch has already
                // decided what this rule would change (`Rule.bars`).
                if branch.blocked || branch.barred.contains(&rule.id) {
                    continue;
                }
                if rule.vikalpa {
                    // Clone first, apply to the clone: the branch in place
                    // is the DECLINED reading and must stay untouched, so
                    // that index 0 remains byte-identical to what the
                    // pre-fork engine produced for this cell.
                    let mut applied = branch.clone();
                    // Keep the clone only if the rule actually fired. A
                    // vikalpa rule that declines its own guard offers no
                    // choice at all, so there is nothing to fork.
                    if (rule.apply)(&mut applied) {
                        applied.barred.extend_from_slice(rule.bars);
                        forks.push((i, applied));
                    }
                } else if (rule.apply)(branch) {
                    branch.barred.extend_from_slice(rule.bars);
                }
            }
```

In `run_pipeline`'s doc comment, after the paragraph ending `callers already test \`blocked\` and must keep doing so, since a blocked branch's partial text is not a surface form.`, add:

```rust
///
/// A rule whose id is in a branch's `barred` list is skipped on that branch.
/// A rule that fires adds its own `bars` to that list (for a vikalpa rule, on
/// the applied clone only). This is how an apavāda that changes no text, such
/// as 6.4.117, keeps its branch from the rules it overrides.
```

- [ ] **Step 6: Run the controller tests**

Run: `mise exec -- cargo test -p panini-prakriya controller:: 2>&1 | tail -30`
Expected: all PASS.

- [ ] **Step 7: Pin the declared bars**

In `tinanta/derivation_tests.rs`, immediately after `exactly_the_pinned_vikalpa_rules_are_optional`, add:

```rust
/// `Rule.bars` names rules by string id, and a string can silently stop
/// matching: a typo, a renumbered rule, or a reorder that moves the barred
/// rule ABOVE its barrer, where barring it does nothing. Pin the whole
/// relation, and require every barred id to run after its barrer.
///
/// Ids are not unique in the pipeline (7.3.84, 7.3.86 and 1.2.4 each appear
/// twice), so "runs after" means some later occurrence.
#[test]
fn exactly_the_pinned_bars() {
    let ids: Vec<&str> = rules().map(|r| r.id).collect();
    for (i, r) in rules().enumerate() {
        for barred in r.bars {
            assert!(
                ids[i + 1..].contains(barred),
                "{} bars {barred}, which does not run after it",
                r.id
            );
        }
    }
    let actual: Vec<(&str, &[&str])> = rules()
        .filter(|r| !r.bars.is_empty())
        .map(|r| (r.id, r.bars))
        .collect();
    let expected: Vec<(&str, &[&str])> = vec![];
    assert_eq!(actual, expected);
}
```

Task 4 replaces `expected` with 6.4.117's entry.

- [ ] **Step 8: Run the full suite**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS at 3852 cells, with every golden and trace unchanged. No rule declares a bar yet.

- [ ] **Step 9: Commit**

```bash
mise run fmt && mise run lint
git add -A crates/panini-prakriya
git commit -m "feat(controller): Rule.bars — the apavāda relation as data

run_pipeline skips a rule barred on a branch by an earlier rule that fired
there; a vikalpa's bars apply to its applied clone only. Every rule declares
bars: &[] for now, so no golden moves. exactly_the_pinned_bars checks each
barred id runs after its barrer.

Claude-Session: https://claude.ai/code/session_01TFmGpY78kwhk6ypKTXzJwn"
```

---

## Task 2: 7.4.78 *bahulaṁ chandasi*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/abhyasa.rs` (a rule after 7.4.76; two tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: `Context.dhatupatha` (slice 3c), `slu_prakriya(root, ending)` in `abhyasa.rs`'s tests.
- Produces: rule `"7.4.78"`, placed immediately after `"7.4.76"`.

- [ ] **Step 1: Write the failing tests**

At the end of `abhyasa.rs`'s `mod tests`, add:

```rust
    #[test]
    fn bahulam_chandasi_makes_the_abhyasa_vowel_i_for_ga() {
        // 7.4.78, applied to √gā (03.0026) on the Kaumudī's authority. After
        // 7.4.62: ja → ji (jigAti).
        let rule = rules().find(|r| r.id == "7.4.78").unwrap();
        let mut p = slu_prakriya("gA", "ti");
        p.ctx.dhatupatha = "03.0026";
        p.terms[ABHYASA].text = "ja".into();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "ji");
        assert_eq!(p.log.last().unwrap().sutra, "7.4.78");
    }

    #[test]
    fn bahulam_chandasi_declines_off_its_row() {
        // Keyed by number: `03.0009` (jahAti) and the hand-built default must
        // keep their abhyāsa `a`.
        let rule = rules().find(|r| r.id == "7.4.78").unwrap();
        for (root, number, abhyasa) in [("gA", "", "ja"), ("hA", "03.0009", "Ja")] {
            let mut p = slu_prakriya(root, "ti");
            p.ctx.dhatupatha = number;
            p.terms[ABHYASA].text = abhyasa.into();
            assert!(!(rule.apply)(&mut p), "{number:?}");
            assert_eq!(p.terms[ABHYASA].text, abhyasa, "{number:?}");
            assert!(p.log.is_empty(), "{number:?}");
        }
    }
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya bahulam_chandasi 2>&1 | tail -20`
Expected: FAIL, panicking on `unwrap()` because there is no rule `7.4.78`.

- [ ] **Step 3: Add the rule**

In `ABHYASA_RULES`, immediately after the 7.4.76 `Rule { … },` and before the closing `];`, add:

```rust
    // 7.4.78 bahulaṁ chandasi: in the Veda, the abhyāsa takes `i` variously.
    // A CHĀNDASA sūtra, applied here to √gā (`03.0026 gA\`) on the authority
    // of the Siddhānta-kaumudī, which derives jigAti by it. vidyut-prakriya
    // does the same and says so ("This is a chAndasa rule, but the SK applies
    // it to derive jigAti from gA, which is a Vedic root."). The Pāṇinian id
    // is kept and the source is recorded here: the 7.3.86 vikalpa-arm
    // precedent for a non-Aṣṭādhyāyī authority.
    //
    // *bahulam* is not modelled as a vikalpa: vidyut gives jig- only, and so
    // does the Kaumudī's form.
    //
    // KEYED BY ROW NUMBER, like 7.4.76 above it: the sūtra itself names no
    // root, and the application is to this one row. After 7.4.62: ja → ji.
    Rule {
        id: "7.4.78",
        name: "bahulaM Candasi",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            // 03.0026 gA\ (√gā)
            if p.ctx.dhatupatha != "03.0026" {
                return false;
            }
            let t: String = p.terms[ABHYASA]
                .text
                .chars()
                .map(|c| if is_vowel(c) { 'i' } else { c })
                .collect();
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("7.4.78", "bahulaM Candasi", before);
            true
        },
    },
```

- [ ] **Step 4: Update the pinned order**

In `derivation_tests.rs`'s `tinanta_rule_order_is_pinned`, change `"7.4.76", "6.4.71",` to `"7.4.76", "7.4.78", "6.4.71",`. `mise run fmt` re-wraps the array.

- [ ] **Step 5: Run the tests, then the full suite**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`, then `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS. The 3852 priors are unchanged, because no curated row is `03.0026`.

- [ ] **Step 6: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/abhyasa.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(abhyasa): 7.4.78 bahulaM Candasi for √gā, on the Kaumudī's authority

Claude-Session: https://claude.ai/code/session_01TFmGpY78kwhk6ypKTXzJwn"
```

---

## Task 3: 6.4.118 *lopo yi*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (a rule between 6.4.119 and 6.4.113; a helper and two tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (the pinned order)

**Interfaces:**
- Consumes: `abhyasta_prakriya(abhyasa, anga, ghu, ending, ngit)` in `guna.rs`'s tests; `following_sarvadhatuka` (already imported in `guna.rs`).
- Produces: rule `"6.4.118"`, placed immediately after `"6.4.119"`. Test helper `jaha_prakriya(ending: &str, ngit: bool) -> Prakriya`, which Task 4 reuses.

- [ ] **Step 1: Write the failing tests**

In `guna.rs`'s `mod tests`, immediately after the `abhyasta_prakriya` helper, add:

```rust
    /// √hā (`03.0009 o~hA\k`) at the guṇa stage: abhyāsa `Ja` (7.4.62 has run;
    /// 8.4.54's `j` comes later), aṅga `hA`, empty śap, `ending`.
    fn jaha_prakriya(ending: &str, ngit: bool) -> Prakriya {
        let mut p = abhyasta_prakriya("Ja", "hA", false, ending, ngit);
        p.ctx.dhatupatha = "03.0009";
        p
    }
```

At the end of `mod tests`, add:

```rust
    #[test]
    fn lopo_yi_elides_the_a_of_ha_before_y() {
        // vidhiliṅ: yāsuṭ sits on the ending's text, Ngit by 3.4.103.
        // Ja + hA + yAt → Ja + h + yAt (jahyAt).
        let rule = rules().find(|r| r.id == "6.4.118").unwrap();
        for ending in ["yAt", "yAtAm", "yus"] {
            let mut p = jaha_prakriya(ending, true);
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ANGA].text, "h", "{ending}");
            assert_eq!(p.log.last().unwrap().sutra, "6.4.118");
        }
    }

    #[test]
    fn lopo_yi_declines_off_its_row_off_y_on_pit_off_a_and_when_short() {
        let rule = rules().find(|r| r.id == "6.4.118").unwrap();
        // 03.0008 is `hA` too and takes 6.4.113 (jihIyAt is not in its
        // ātmanepada paradigm, but the guard must not rest on that).
        for number in ["03.0008", ""] {
            let mut p = jaha_prakriya("yAt", true);
            p.ctx.dhatupatha = number;
            assert!(!(rule.apply)(&mut p), "{number:?}");
            assert_eq!(p.terms[ANGA].text, "hA");
        }
        // A consonant other than y: 6.4.116 / 6.4.113's cell (jahItaH).
        let mut p = jaha_prakriya("tas", true);
        assert!(!(rule.apply)(&mut p));
        // A y-initial follower that is not kṅit: unreachable in the corpus,
        // where yāsuṭ is always ṅit, so this test is what holds the clause.
        let mut p = jaha_prakriya("yAt", false);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "hA");
        // Not ā-final.
        let mut p = jaha_prakriya("yAt", true);
        p.terms[ANGA].text = "hi".into();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "hi");
        // No ending term: must not panic indexing ENDING.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("hA")]),
            ..Default::default()
        };
        p.ctx.dhatupatha = "03.0009";
        assert!(!(rule.apply)(&mut p));
        assert!(p.log.is_empty());
    }
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya lopo_yi 2>&1 | tail -20`
Expected: FAIL, panicking on `unwrap()` because there is no rule `6.4.118`.

- [ ] **Step 3: Add the rule**

In `guna.rs`, immediately after the 6.4.119 `Rule { … },` and before the `// --- 6.4.113 / 6.4.112:` block comment, add:

```rust
    // --- 6.4.118 / 6.4.117 / 6.4.116: √hā's (jahāti's) ā ------------------
    //
    // *jahāteḥ* is the juhotyādi √ohāk, `03.0009 o~hA\k`. `03.0008 o~hA\N`
    // enters the derivation as the same `hA` and takes none of the three, so
    // all three key on the NUMBER (3c's argument for `Context.dhatupatha`).
    //
    // All three sit ABOVE 6.4.113, which each of them overrides, and read the
    // follower through `following_sarvadhatuka` exactly as 6.4.113's abhyasta
    // arm does: under ślu the śap is empty, so the follower is the ending,
    // which in vidhiliṅ carries yāsuṭ on its text and Ngit from 3.4.103.

    // 6.4.118 lopo yi: √hā's ā is elided before a y-initial kṅit
    // sārvadhātuka. Ja + hA + yAt → jahyAt, not *jahIyAt (6.4.113).
    //
    // FIRST of the three, as vidyut orders it (`if y-initial { 6.4.118 } else
    // { 6.4.117 / 6.4.116 }`). The elided ā is what 6.4.116, 6.4.113 and
    // 6.4.112 all require, so every one of them declines after it with no
    // guard of its own. *kṅiti* comes by anuvṛtti from 6.4.113. No corpus cell
    // offers a y-initial follower that is not ṅit (yāsuṭ always is), so the
    // clause is held by a guard test, not a cell.
    Rule {
        id: "6.4.118",
        name: "lopo yi",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            // 03.0009 o~hA\k (√hā, jahāti)
            if p.ctx.dhatupatha != "03.0009" || p.terms.len() <= ENDING {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix('A') else {
                return false;
            };
            let stem = stem.to_string();
            let follower = following_sarvadhatuka(p)
                .expect("the len() <= ENDING guard above implies a follower");
            if !follower.has(Tag::Ngit) || !follower.text.starts_with('y') {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = stem;
            p.record("6.4.118", "lopo yi", before);
            true
        },
    },
```

- [ ] **Step 4: Update the pinned order**

In `tinanta_rule_order_is_pinned`, change `"6.4.119", "6.4.113",` to `"6.4.119", "6.4.118", "6.4.113",`.

Also update the module doc's first line in `guna.rs`: `//! Vowel gradation and vikaraṇa reshaping: 7.4.21 … 6.4.119, 6.4.113, 6.4.112, 6.4.115.` becomes `//! Vowel gradation and vikaraṇa reshaping: 7.4.21 … 6.4.119, 6.4.118 … 6.4.116, 6.4.113, 6.4.112, 6.4.115.`

- [ ] **Step 5: Run the tests, then the full suite**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`, then `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS, with the 3852 priors unchanged.

- [ ] **Step 6: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/guna.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(guna): 6.4.118 lopo yi for √hā (03.0009)

Claude-Session: https://claude.ai/code/session_01TFmGpY78kwhk6ypKTXzJwn"
```

---

## Task 4: 6.4.117 *ā ca hau* and 6.4.116 *jahāteś ca*, the first `bars`

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (two rules after 6.4.118; tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (rule order; vikalpa pin; bars pin)
- Modify: `crates/panini-prakriya/src/rule.rs` (the ordering-caveat comment gains 6.4.116's argument)

**Interfaces:**
- Consumes: `jaha_prakriya` (Task 3); `Rule.bars` and its enforcement (Task 1); `crate::controller::run_pipeline`.
- Produces: rules `"6.4.117"` (vikalpa, `bars: &["6.4.116", "6.4.113", "6.4.112"]`) and `"6.4.116"` (vikalpa), in the order `6.4.118, 6.4.117, 6.4.116, 6.4.113`. The engine goes to eleven optional rules.

- [ ] **Step 1: Write the failing tests**

At the end of `guna.rs`'s `mod tests`, add:

```rust
    #[test]
    fn a_ca_hau_records_a_step_that_changes_no_text() {
        let rule = rules().find(|r| r.id == "6.4.117").unwrap();
        let mut p = jaha_prakriya("hi", true);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "hA");
        let step = p.log.last().unwrap();
        assert_eq!(step.sutra, "6.4.117");
        assert_eq!(step.before, step.after);
        assert_eq!(rule.bars, &["6.4.116", "6.4.113", "6.4.112"]);
        assert!(rule.vikalpa);
    }

    #[test]
    fn a_ca_hau_declines_off_hi_off_its_row_off_a_and_when_short() {
        let rule = rules().find(|r| r.id == "6.4.117").unwrap();
        // The tātaṅ branch: jahItAt / jahitAt are 6.4.113's and 6.4.116's.
        let mut p = jaha_prakriya("tAt", true);
        assert!(!(rule.apply)(&mut p));
        for number in ["03.0008", ""] {
            let mut p = jaha_prakriya("hi", true);
            p.ctx.dhatupatha = number;
            assert!(!(rule.apply)(&mut p), "{number:?}");
        }
        // Not ā-final: no cell reaches this (6.4.118 needs a y), so the test
        // is what holds the clause.
        let mut p = jaha_prakriya("hi", true);
        p.terms[ANGA].text = "hi".into();
        assert!(!(rule.apply)(&mut p));
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("hA")]),
            ..Default::default()
        };
        p.ctx.dhatupatha = "03.0009";
        assert!(!(rule.apply)(&mut p));
        assert!(p.log.is_empty());
    }

    #[test]
    fn jahates_ca_gives_i_before_a_hal_initial_kngit() {
        let rule = rules().find(|r| r.id == "6.4.116").unwrap();
        for ending in ["tas", "hi", "tAt", "va"] {
            let mut p = jaha_prakriya(ending, true);
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ANGA].text, "hi", "{ending}");
            assert_eq!(p.log.last().unwrap().sutra, "6.4.116");
        }
        assert!(rule.vikalpa);
    }

    #[test]
    fn jahates_ca_declines_on_pit_ajadi_off_its_row_off_a_and_when_short() {
        let rule = rules().find(|r| r.id == "6.4.116").unwrap();
        // Pit: jahAti.
        let mut p = jaha_prakriya("ti", false);
        assert!(!(rule.apply)(&mut p));
        // Vowel-initial: jahati is 6.4.112's.
        let mut p = jaha_prakriya("ati", true);
        assert!(!(rule.apply)(&mut p));
        // √ohāṅ (03.0008) takes 6.4.113 only: jihIte, never *jihite.
        for number in ["03.0008", ""] {
            let mut p = jaha_prakriya("te", true);
            p.ctx.dhatupatha = number;
            assert!(!(rule.apply)(&mut p), "{number:?}");
        }
        // Empty follower.
        let mut p = jaha_prakriya("", true);
        assert!(!(rule.apply)(&mut p));
        // Not ā-final: after 6.4.118 (jahyAt).
        let mut p = jaha_prakriya("yAt", true);
        p.terms[ANGA].text = "h".into();
        assert!(!(rule.apply)(&mut p));
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("hA")]),
            ..Default::default()
        };
        p.ctx.dhatupatha = "03.0009";
        assert!(!(rule.apply)(&mut p));
        assert!(p.log.is_empty());
    }

    /// The five rules in pipeline order, run as one stage through the real
    /// controller, so the bars are enforced exactly as in `derive`.
    fn run_ha_rules(p: Prakriya) -> Vec<Prakriya> {
        let stage: Vec<Rule> = ["6.4.118", "6.4.117", "6.4.116", "6.4.113", "6.4.112"]
            .iter()
            .map(|id| *rules().find(|r| r.id == *id).unwrap())
            .collect();
        crate::controller::run_pipeline(p, &[&stage[..]])
    }

    #[test]
    fn before_hi_the_three_readings_are_i_long_i_short_and_a_kept() {
        // Declined (6.4.113: jahIhi), 6.4.116 (jahihi), 6.4.117 (jahAhi).
        // 6.4.117's branch is barred from all three rules that would change
        // its `A` — 6.4.112 included, or it would give *jahhi.
        let out = run_ha_rules(jaha_prakriya("hi", true));
        let texts: Vec<String> = out.iter().map(|p| p.text()).collect();
        assert_eq!(texts, vec!["JahIhi", "Jahihi", "JahAhi"]);
        let ids: Vec<&str> = out[2].log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, vec!["6.4.117"]);
    }

    #[test]
    fn before_a_consonant_two_readings_and_before_y_or_a_vowel_one() {
        let texts = |p: Prakriya| run_ha_rules(p).iter().map(|b| b.text()).collect::<Vec<_>>();
        assert_eq!(texts(jaha_prakriya("tas", true)), vec!["JahItas", "Jahitas"]);
        assert_eq!(texts(jaha_prakriya("yAt", true)), vec!["JahyAt"]);
        assert_eq!(texts(jaha_prakriya("ati", true)), vec!["Jahati"]);
        assert_eq!(texts(jaha_prakriya("ti", false)), vec!["JahAti"]);
    }
```

The `guna.rs` test module already has `use super::*;`, which brings `Rule` into scope. If the compiler says it doesn't, add `use crate::rule::Rule;`.

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya -- a_ca_hau jahates_ca before_hi before_a_consonant 2>&1 | tail -20`
Expected: FAIL, because the rules are missing.

- [ ] **Step 3: Add the two rules**

In `guna.rs`, immediately after the 6.4.118 `Rule { … },`, add:

```rust
    // 6.4.117 ā ca hau (vikalpa): before *hi*, √hā's ā optionally stays ā.
    // jahAhi, beside 6.4.116's jahihi and 6.4.113's jahIhi.
    //
    // A substitution of `A` for `A`: it CHANGES NO TEXT. vidyut writes it as
    // `optional_run_at("6.4.117", i, op::antya("A"))`, whose only effect is
    // that neither 6.4.116 nor 6.4.113/6.4.112 runs on its branch. Here that
    // is `bars`, enforced by `run_pipeline` (`Rule.bars`), so none of those
    // three guards mentions this rule. 6.4.112 is the easy one to miss: *hi*
    // is kṅit (1.2.4) and the `A` is still there, so without the bar
    // 6.4.112's abhyasta arm would elide it and give *jahhi.
    //
    // The fork survives the convergent-fork collapse because the barred
    // rules make the declined branch differ (jahIhi); see
    // `a_text_neutral_barring_fork_survives_once_a_barred_rule_diverges_it`.
    //
    // Reads ENDING directly, as 6.4.119 does: *hau* names the ending itself.
    // No cell offers a non-ā-final √hā aṅga before *hi* (6.4.118 needs a y),
    // so the `A` clause is held by a guard test.
    Rule {
        id: "6.4.117",
        name: "A ca hO",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["6.4.116", "6.4.113", "6.4.112"],
        apply: |p| {
            // 03.0009 o~hA\k (√hā, jahāti)
            if p.ctx.dhatupatha != "03.0009" || p.terms.len() <= ENDING {
                return false;
            }
            if p.terms[ENDING].text != "hi" || !p.terms[ANGA].text.ends_with('A') {
                return false;
            }
            let before = p.snapshot();
            p.record("6.4.117", "A ca hO", before);
            true
        },
    },
    // 6.4.116 jahāteś ca (vikalpa): before a consonant-initial kṅit
    // sārvadhātuka, √hā's ā optionally becomes `i`, continuing 6.4.114's
    // *it* against 6.4.113's `ī`. jahitaH beside jahItaH; jahihi beside
    // jahIhi.
    //
    // *hali* and *kṅiti* come by anuvṛtti from 6.4.113, and cells witness
    // both: `ti` (pit) must give jahAti, and `ati` (ajādi) must give jahati.
    //
    // ORDERING CAVEAT (`Rule.vikalpa`): this rule invalidates "the aṅga ends
    // in `A`" on its branch. The only consumers below it are 6.4.113 and
    // 6.4.112, and on this branch both are MEANT to decline, which they do on
    // the `i`. The invalidation is the intended bleeding, not a hazard.
    Rule {
        id: "6.4.116",
        name: "jahAteSca",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &[],
        apply: |p| {
            // 03.0009 o~hA\k (√hā, jahāti)
            if p.ctx.dhatupatha != "03.0009" || p.terms.len() <= ENDING {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix('A') else {
                return false;
            };
            let stem = format!("{stem}i");
            let follower = following_sarvadhatuka(p)
                .expect("the len() <= ENDING guard above implies a follower");
            if !follower.has(Tag::Ngit) {
                return false;
            }
            let Some(next) = follower.text.chars().next() else {
                return false;
            };
            if is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = stem;
            p.record("6.4.116", "jahAteSca", before);
            true
        },
    },
```

- [ ] **Step 4: Update the three pins**

In `derivation_tests.rs`:

1. In `tinanta_rule_order_is_pinned`, change `"6.4.119", "6.4.118", "6.4.113",` to `"6.4.119", "6.4.118", "6.4.117", "6.4.116", "6.4.113",`.
2. In `exactly_the_pinned_vikalpa_rules_are_optional`, change the expected array to:

   ```rust
       let expected = [
           "7.1.35", "3.4.111", "7.3.86", "6.4.117", "6.4.116", "6.4.115", "6.4.107", "8.2.74",
           "8.2.75", "8.4.65", "8.4.56",
       ];
   ```

3. In `exactly_the_pinned_bars`, change `let expected: Vec<(&str, &[&str])> = vec![];` to:

   ```rust
       let expected: Vec<(&str, &[&str])> = vec![("6.4.117", &["6.4.116", "6.4.113", "6.4.112"][..])];
   ```

In `guna.rs`, 6.4.115's comment says `The engine's ninth vikalpa.`. Change it to `The engine's ninth vikalpa (6.4.117 and 6.4.116, slice 3c2, are the tenth and eleventh).`

- [ ] **Step 5: Run the tests, then the full suite**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`, then `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS, with the 3852 priors unchanged. No curated row is `03.0009`.

- [ ] **Step 6: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/guna.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(guna): 6.4.117 A ca hO and 6.4.116 jahAteSca — the first bars

6.4.117 keeps √hā's A before hi by changing no text and barring 6.4.116,
6.4.113 and 6.4.112 on its branch. 6.4.116 gives i before a hal-initial
kṅit. The engine now has eleven optional rules.

Claude-Session: https://claude.ai/code/session_01TFmGpY78kwhk6ypKTXzJwn"
```

---

## Task 5: The two rows and their paradigm goldens

This task turns the 72 new cells green.

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (two `Dhatu` rows; the gaṇa-row test; three doc-comment counts)
- Modify: `crates/panini/tests/paradigm/data/juhotyadi.rs` (8 `PARADIGM` blocks, 31 `ALTERNATES` rows)
- Modify: `crates/panini/tests/paradigm/main.rs` (`VIKALPA_RULES`, totals, a `sevens` bucket, key census, prose)

**Interfaces:**
- Consumes: Tasks 1–4.
- Produces: `dhatus()` rows `03.0009` (`hA`, `Parasmaipada`) and `03.0026` (`gA`, `Parasmaipada`), both `Gana::Juhotyadi`.

- [ ] **Step 1: Add the two `Dhatu` rows**

`DHATUS` is ordered by number. Insert the `03.0009` row after the `03.0008` row, and the `03.0026` row after the `03.0020` row:

```rust
    Dhatu {
        // 03.0009 `o~hA\k` tyAge (√ohāk, jahāti). The `o~` is an it by 1.3.2;
        // parasmaipadī by 1.3.78. Enters the derivation as `hA`, exactly like
        // 03.0008 — so 7.4.76 declines on it by number (jahAti, not *jihAti),
        // and 6.4.116 / 6.4.117 / 6.4.118, which *jahāteḥ* names, fire on it by
        // number: jahitaH / jahItaH, jahAhi / jahihi / jahIhi, jahyAt. Slice 3c2.
        dhatupatha: "03.0009",
        code: "hA",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "tyAge",
    },
```

```rust
    Dhatu {
        // 03.0026 `gA\` stutO (√gā). Parasmaipadī by 1.3.78 (the `\` is the
        // root vowel's accent). A Vedic root: its abhyāsa takes `i` by the
        // chāndasa 7.4.78, applied on the Kaumudī's authority (jigAti), keyed
        // by this number. Not ghu; 6.4.113 gives jigItaH. Slice 3c2.
        dhatupatha: "03.0026",
        code: "gA",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "stutO",
    },
```

The arthas are upstream's verbatim (`03.0009 o~hA\k tyAge`, `03.0026 gA\ stutO`, checked against `data/dhatupatha.tsv` while writing this plan); `dhatupatha_numbers_resolve_upstream` holds them.

In the `Dhatu` struct's doc comments:
- `re-derives 84 of these 85` → `re-derives 86 of these 87`
- `The test covers the 85 roots curated here` → `The test covers the 87 roots curated here`

In `pada_from_upadesha`'s doc comment:
- `55 of the 85 curated roots` → `57 of the 87 curated roots`
- the `34` that begins the next line → `36`

Both new upadeśas carry exactly one backslash, on the root vowel, and none on an it. In `curated_roots_have_expected_ganas_and_padas`, change `assert_eq!(dhatus().len(), 85);` to `87`.

- [ ] **Step 2: Extend the gaṇa-row test**

Rename `juhotyadi_rows_are_the_eight_curated_roots` to `juhotyadi_rows_are_the_ten_curated_roots`. Update every reference to the old name; `grep -rn "juhotyadi_rows_are_the_eight" .` finds them. In its comment, replace `The gaṇa is PARTIAL at 8 of` / `its 26 dhātupāṭha rows; slices 3c2 and 3d–3f close it.` with:

```rust
        // Slice 3c2 adds √hā parasmaipada (03.0009, jahāti) and √gā
        // (03.0026), both parasmaipadī by 1.3.78. The gaṇa is PARTIAL at 10
        // of its 26 dhātupāṭha rows; slices 3d–3f close it.
```

Its expected vector becomes:

```rust
            vec![
                ("03.0001", "hu", PadaAssignment::Parasmaipada),
                ("03.0002", "BI", PadaAssignment::Parasmaipada),
                ("03.0003", "hrI", PadaAssignment::Parasmaipada),
                ("03.0007", "mA", PadaAssignment::Atmanepada),
                ("03.0008", "hA", PadaAssignment::Atmanepada),
                ("03.0009", "hA", PadaAssignment::Parasmaipada),
                ("03.0010", "dA", PadaAssignment::Ubhayapada),
                ("03.0011", "DA", PadaAssignment::Ubhayapada),
                ("03.0020", "ki", PadaAssignment::Parasmaipada),
                ("03.0026", "gA", PadaAssignment::Parasmaipada),
            ]
```

Replace its final comment (`Slice 3c's root-specific rules … enters the derivation as \`hA\` too.`) with:

```rust
        // Every root-specific rule of slices 3c and 3c2 (7.4.76, 7.4.78,
        // 8.2.38, 8.2.40's adhaḥ, 6.4.116–6.4.118) and 1.1.20's Tag::Ghu key
        // on the dhātupāṭha NUMBER, so `hA`, `dA`, `DA` and `gA` need no
        // uniqueness tripwire. `hA` is now held by two rows, 03.0008 and
        // 03.0009, which is exactly why.
```

- [ ] **Step 3: Run the data tests**

Run: `mise exec -- cargo test -p panini-data 2>&1 | tail -20`
Expected: PASS, including `curated_pada_agrees_with_upadesha_markers` and `dhatupatha_numbers_resolve_upstream`.

- [ ] **Step 4: Add the `PARADIGM` blocks**

`PARADIGM` in `crates/panini/tests/paradigm/data/juhotyadi.rs` is ordered by the slice that added each block, not by number (3a's `03.0020` precedes 3c's `03.0010`). **Append** all eight blocks to the end of `PARADIGM`, `03.0009` first. Index 0 of each cell is the declined derivation: `ajahAd`, `jahAtu`, `jahIhi`, `jahyAd`, and the `jahItaH`-type ī forms. `mise run fmt` re-wraps the arrays.

```rust
    ("03.0009", "laT", Pada::Parasmaipada, ["jahAti", "jahItaH", "jahati", "jahAsi", "jahITaH", "jahITa", "jahAmi", "jahIvaH", "jahImaH"]),
    ("03.0009", "laN", Pada::Parasmaipada, ["ajahAd", "ajahItAm", "ajahuH", "ajahAH", "ajahItam", "ajahIta", "ajahAm", "ajahIva", "ajahIma"]),
    ("03.0009", "loT", Pada::Parasmaipada, ["jahAtu", "jahItAm", "jahatu", "jahIhi", "jahItam", "jahIta", "jahAni", "jahAva", "jahAma"]),
    ("03.0009", "viDiliN", Pada::Parasmaipada, ["jahyAd", "jahyAtAm", "jahyuH", "jahyAH", "jahyAtam", "jahyAta", "jahyAm", "jahyAva", "jahyAma"]),
```

```rust
    ("03.0026", "laT", Pada::Parasmaipada, ["jigAti", "jigItaH", "jigati", "jigAsi", "jigITaH", "jigITa", "jigAmi", "jigIvaH", "jigImaH"]),
    ("03.0026", "laN", Pada::Parasmaipada, ["ajigAd", "ajigItAm", "ajiguH", "ajigAH", "ajigItam", "ajigIta", "ajigAm", "ajigIva", "ajigIma"]),
    ("03.0026", "loT", Pada::Parasmaipada, ["jigAtu", "jigItAm", "jigatu", "jigIhi", "jigItam", "jigIta", "jigAni", "jigAva", "jigAma"]),
    ("03.0026", "viDiliN", Pada::Parasmaipada, ["jigIyAd", "jigIyAtAm", "jigIyuH", "jigIyAH", "jigIyAtam", "jigIyAta", "jigIyAm", "jigIyAva", "jigIyAma"]),
```

- [ ] **Step 5: Add the `ALTERNATES` rows**

**Append** these to the end of `ALTERNATES` in the same file. The index is the 0-based cell (P.E P.D P.B M.E M.D M.B U.E U.D U.B). The key names the optional rules behind the form, in pipeline order (7.1.35 → 6.4.117 / 6.4.116 → 8.4.56).

```rust
    ("03.0009", "laT", Pada::Parasmaipada, 1, "jahitaH", "6.4.116"),
    ("03.0009", "laT", Pada::Parasmaipada, 4, "jahiTaH", "6.4.116"),
    ("03.0009", "laT", Pada::Parasmaipada, 5, "jahiTa", "6.4.116"),
    ("03.0009", "laT", Pada::Parasmaipada, 7, "jahivaH", "6.4.116"),
    ("03.0009", "laT", Pada::Parasmaipada, 8, "jahimaH", "6.4.116"),
    ("03.0009", "laN", Pada::Parasmaipada, 0, "ajahAt", "8.4.56"),
    ("03.0009", "laN", Pada::Parasmaipada, 1, "ajahitAm", "6.4.116"),
    ("03.0009", "laN", Pada::Parasmaipada, 4, "ajahitam", "6.4.116"),
    ("03.0009", "laN", Pada::Parasmaipada, 5, "ajahita", "6.4.116"),
    ("03.0009", "laN", Pada::Parasmaipada, 7, "ajahiva", "6.4.116"),
    ("03.0009", "laN", Pada::Parasmaipada, 8, "ajahima", "6.4.116"),
    ("03.0009", "loT", Pada::Parasmaipada, 0, "jahItAd", "7.1.35"),
    ("03.0009", "loT", Pada::Parasmaipada, 0, "jahItAt", "7.1.35+8.4.56"),
    ("03.0009", "loT", Pada::Parasmaipada, 0, "jahitAd", "7.1.35+6.4.116"),
    ("03.0009", "loT", Pada::Parasmaipada, 0, "jahitAt", "7.1.35+6.4.116+8.4.56"),
    ("03.0009", "loT", Pada::Parasmaipada, 1, "jahitAm", "6.4.116"),
    ("03.0009", "loT", Pada::Parasmaipada, 3, "jahihi", "6.4.116"),
    ("03.0009", "loT", Pada::Parasmaipada, 3, "jahAhi", "6.4.117"),
    ("03.0009", "loT", Pada::Parasmaipada, 3, "jahItAd", "7.1.35"),
    ("03.0009", "loT", Pada::Parasmaipada, 3, "jahItAt", "7.1.35+8.4.56"),
    ("03.0009", "loT", Pada::Parasmaipada, 3, "jahitAd", "7.1.35+6.4.116"),
    ("03.0009", "loT", Pada::Parasmaipada, 3, "jahitAt", "7.1.35+6.4.116+8.4.56"),
    ("03.0009", "loT", Pada::Parasmaipada, 4, "jahitam", "6.4.116"),
    ("03.0009", "loT", Pada::Parasmaipada, 5, "jahita", "6.4.116"),
    ("03.0009", "viDiliN", Pada::Parasmaipada, 0, "jahyAt", "8.4.56"),
```

```rust
    ("03.0026", "laN", Pada::Parasmaipada, 0, "ajigAt", "8.4.56"),
    ("03.0026", "loT", Pada::Parasmaipada, 0, "jigItAd", "7.1.35"),
    ("03.0026", "loT", Pada::Parasmaipada, 0, "jigItAt", "7.1.35+8.4.56"),
    ("03.0026", "loT", Pada::Parasmaipada, 3, "jigItAd", "7.1.35"),
    ("03.0026", "loT", Pada::Parasmaipada, 3, "jigItAt", "7.1.35+8.4.56"),
    ("03.0026", "viDiliN", Pada::Parasmaipada, 0, "jigIyAt", "8.4.56"),
```

That is 25 + 6 = **31 rows**. Check: √hā 36 cells + 25 = 61 forms; √gā 36 + 6 = 42.

- [ ] **Step 6: Update `paradigm/main.rs`**

`VIKALPA_RULES` becomes:

```rust
const VIKALPA_RULES: &[&str] = &[
    "7.1.35", "3.4.111", "7.3.86", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56", "6.4.115",
    "6.4.117", "6.4.116",
];
```

In its doc comment, change `the pin has 6.4.115 fourth, right after 7.3.86, not last as it sits here.` to `the pin has 6.4.117, 6.4.116 and 6.4.115 fourth to sixth, right after 7.3.86, not last as they sit here.` In `every_alternate_names_the_vikalpa_rules_that_produced_it`'s doc comment, change `otherwise 971 bare strings` to `otherwise 1002 bare strings`.

In `derivation_set_shape_matches_the_audited_numbers`:
- `assert_eq!(total_cells, 3852, "428 root×lakāra blocks × 9 cells each")` → `3924`, `"436 root×lakāra blocks × 9 cells each"`.
- Add `let mut sevens = 0usize;` after `sixes`, and the match arm `7 => sevens += 1,` after `6 => sixes += 1,`.
- `ones` 3133 → `3184`; `twos` 554 → `571`.
- `threes` 121 → `123`, appending to its message: `; and — new in slice 3c2 — √gā's, the same way`.
- `fours` 18 unchanged.
- `fives` 9 → `10`, appending: `; and — new in slice 3c2 — √hā's loṭ prathama eka, forking on 7.1.35/6.4.116/8.4.56`.
- `sixes` 17 unchanged.
- After the `sixes` assert, add:

  ```rust
      assert_eq!(
          sevens, 1,
          "seven-form cells — new in slice 3c2, the engine's record: √hā's loṭ parasmaipada \
           madhyama eka, three readings before hi (6.4.113's jahIhi, 6.4.116's jahihi, 6.4.117's \
           jahAhi, the last barring the other two) plus four tātaṅ forms (7.1.35 with 6.4.116 and \
           8.4.56 stacked)"
      );
  ```

- `ALTERNATES.len()` 971 → `1002`.
- `key_count("8.4.56")` 142 → `146`; `key_count("7.1.35")` 120 → `124`; `key_count("7.1.35+8.4.56")` 120 → `124`.
- After the last existing `key_count` assert, add:

  ```rust
      assert_eq!(key_count("6.4.116"), 14, "6.4.116-only alternates");
      assert_eq!(key_count("6.4.117"), 1, "6.4.117-only alternates");
      assert_eq!(key_count("7.1.35+6.4.116"), 2, "7.1.35+6.4.116 alternates");
      assert_eq!(
          key_count("7.1.35+6.4.116+8.4.56"),
          2,
          "7.1.35+6.4.116+8.4.56 alternates"
      );
  ```

  If the test ends with a check that the listed keys sum to `ALTERNATES.len()`, the four new keys must be included in it.

Check: 3184 + 571 + 123 + 18 + 10 + 17 + 1 = 3924 cells; 3184 + 1142 + 369 + 72 + 50 + 102 + 7 = 4926 forms = 3924 + 1002.

In the doc comment above the test:
- Change `3852 cells total (428 root×lakāra blocks × 9), of which 3133 hold exactly` / `one form, 554 hold two, 121 hold three (√hrī's … and √dā's and √dhā's, new in slice 3c, each by` / `7.1.35/8.4.56)` to read `3924 cells total (436 root×lakāra blocks × 9), of which 3184 hold exactly one form, 571 hold two, 123 hold three (√hrī's loṭ prathama and madhyama eka, new in slice 3b, √dā's and √dhā's, new in slice 3c, and √gā's, new in slice 3c2, each by 7.1.35/8.4.56)`.
- In the five-form list, change `nine hold five` to `ten hold five`, and after `forking on 7.1.35/6.4.115/8.4.56)` add `, and — new in slice 3c2 — √hā's loṭ prathama eka, forking on 7.1.35/6.4.116/8.4.56`.
- After the six-form clause ending `the six-form record now stands at seventeen cells, not sixteen, and at three mechanisms, not two)` add `, and one — new in slice 3c2 — holds SEVEN, the engine's new record: √hā's loṭ parasmaipada madhyama eka, where 6.4.117 is the first optional rule to bar others (\`Rule.bars\`), so its three readings before *hi* are not a 2^k product`.
- The key census line `itself has 971 rows, keyed 142 \`8.4.56\`, 120 \`7.1.35\`, 120 \`7.1.35+8.4.56\`,` becomes `itself has 1002 rows, keyed 146 \`8.4.56\`, 124 \`7.1.35\`, 124 \`7.1.35+8.4.56\`,`. At the end of that key list, after `and 1 \`6.4.115+8.4.56\``, add `, 14 \`6.4.116\`, 1 \`6.4.117\`, 2 \`7.1.35+6.4.116\` and 2 \`7.1.35+6.4.116+8.4.56\``.

Immediately before `/// This test is what keeps the numbers true day to day.`, add:

```rust
///
/// Slice 3c2 curates √hā parasmaipada (`03.0009 o~hA\k`, jahāti) and √gā
/// (`03.0026`), both parasmaipadī by 1.3.78, bringing the gaṇa to ten of its
/// twenty-six rows. √gā forks exactly where every -oti parasmaipada root does
/// (`ajigAd`/`ajigAt`, `jigIyAd`/`jigIyAt`, and the two loṭ tātaṅ cells three
/// ways): six rows in the three pre-existing keys. √hā brings the engine's
/// tenth and eleventh vikalpas. 6.4.116 *jahāteś ca* forks every
/// consonant-initial kṅit cell (`jahItaH`/`jahitaH`). 6.4.117 *ā ca hau*
/// changes no text but bars 6.4.116/6.4.113/6.4.112 on its branch (`jahAhi`).
/// Together they make its loṭ madhyama eka the first seven-form cell. Its
/// vidhiliṅ takes 6.4.118 *lopo yi* (`jahyAt`) and forks only on 8.4.56.
/// Thirty-one new rows: 25 for √hā, 6 for √gā. The gaṇa is PARTIAL at 10 of
/// its 26 rows.
```

- [ ] **Step 7: Run the full suite**

Run: `mise run test 2>&1 | tail -40` (foreground, timeout ≥ 600000 ms)
Expected: PASS at 3924 cells, with every 3852 prior unchanged.

If a new cell fails, the engine and the goldens disagree. Read the failing form against the spec's rule table and the seven-form branch table before touching either, and use superpowers:systematic-debugging. **Do not edit a golden to match the engine.** The goldens are vidyut's output and they are the specification. The likeliest cause is a missing bar: *jahAhi* appearing as \**jahhi* or \**jahIhi* means 6.4.117's branch ran a rule it should have barred.

- [ ] **Step 8: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-data/src/lib.rs crates/panini/tests/paradigm/data/juhotyadi.rs crates/panini/tests/paradigm/main.rs
git commit -m "feat(data): √hā (parasmaipada) and √gā — juhotyādi at ten of twenty-six rows

3852 → 3924 cells, 4823 → 4926 forms, 971 → 1002 ALTERNATES, 85 → 87 roots.
√hā's loṭ madhyama eka is the engine's first seven-form cell.

Claude-Session: https://claude.ai/code/session_01TFmGpY78kwhk6ypKTXzJwn"
```

---

## Task 6: The trace pins

**Files:**
- Modify: `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes: `crate::helpers::{at, cell_trace}`. `cell_trace(number, lakara, pada, purusha, vacana) -> (String, Vec<String>)` returns index 0's text and trace. It does not reach the alternate branches, so the alternate pins below call `derive` directly (`derive` and `dhatus` are already imported).

- [ ] **Step 1: Add the pins**

At the end of the file, add:

```rust
/// Every live branch of one √hā cell, as (text, trace).
fn ha_branches(lakara: Lakara, purusha: Purusha, vacana: Vacana) -> Vec<(String, Vec<String>)> {
    let d = dhatus().iter().find(|d| d.dhatupatha == "03.0009").unwrap();
    derive(d, lakara, Pada::Parasmaipada, purusha, vacana)
        .into_iter()
        .filter(|p| !p.blocked)
        .map(|p| (p.text(), p.log.iter().map(|s| s.sutra.clone()).collect()))
        .collect()
}

fn branch<'a>(bs: &'a [(String, Vec<String>)], form: &str) -> &'a Vec<String> {
    &bs.iter().find(|(t, _)| t == form).unwrap_or_else(|| panic!("no branch {form}")).1
}

#[test]
fn jahati_trace_takes_no_bhrnam_it() {
    // hA P laT P.E. 03.0009 is `hA` exactly like 03.0008 (jihIte), and
    // 7.4.76 names only the latter: the derivation witness 3c's guard test
    // promised.
    let (text, t) = cell_trace(
        "03.0009",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "jahAti", "got {t:?}");
    assert!(!t.contains(&"7.4.76".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.116".to_string()), "got {t:?}");
}

#[test]
fn jahahi_trace_is_a_ca_hau_and_nothing_it_bars() {
    // hA P loT M.E, the seven-form cell. 6.4.117 changes no text; its branch
    // must carry none of the three rules it bars — 6.4.112 included, whose
    // absence is what keeps *jahhi out.
    let bs = ha_branches(Lakara::Lot, Purusha::Madhyama, Vacana::Eka);
    assert_eq!(bs.len(), 7, "got {bs:?}");
    assert_eq!(bs[0].0, "jahIhi", "declined first");
    let t = branch(&bs, "jahAhi");
    assert!(t.contains(&"6.4.117".to_string()), "got {t:?}");
    for barred in ["6.4.116", "6.4.113", "6.4.112"] {
        assert!(!t.contains(&barred.to_string()), "{barred} in {t:?}");
    }
    let t = branch(&bs, "jahihi");
    assert!(t.contains(&"6.4.116".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.113".to_string()), "got {t:?}");
    let t = &bs[0].1;
    assert!(t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.117".to_string()), "got {t:?}");
}

#[test]
fn jahitah_trace_credits_jahates_ca_not_i_halyaghoh() {
    // hA P laT P.D. The alternate reading: 6.4.116's i bleeds 6.4.113.
    let bs = ha_branches(Lakara::Lat, Purusha::Prathama, Vacana::Dvi);
    let t = branch(&bs, "jahitaH");
    assert!(at(t, "6.1.10") < at(t, "6.4.116"), "got {t:?}");
    assert!(!t.contains(&"6.4.113".to_string()), "got {t:?}");
    let t = branch(&bs, "jahItaH");
    assert!(t.contains(&"6.4.113".to_string()), "got {t:?}");
}

#[test]
fn jahyat_trace_is_lopo_yi_before_any_i_substitute() {
    // hA P viDiliN P.E. 6.4.118 elides the ā before yāsuṭ's y, so neither
    // 6.4.116 nor 6.4.113 can fire (not *jahIyAt, the pre-slice form).
    let (text, t) = cell_trace(
        "03.0009",
        Lakara::VidhiLin,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "jahyAd", "got {t:?}");
    assert!(at(&t, "7.2.79") < at(&t, "6.4.118"), "got {t:?}");
    for absent in ["6.4.116", "6.4.113", "6.4.112"] {
        assert!(!t.contains(&absent.to_string()), "{absent} in {t:?}");
    }
}

#[test]
fn jigati_trace_is_kuhos_cuh_then_bahulam_chandasi() {
    // gA P laT P.E. 7.4.62 palatalises ga → ja, then 7.4.78 gives ji.
    let (text, t) = cell_trace(
        "03.0026",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "jigAti", "got {t:?}");
    assert!(at(&t, "7.4.62") < at(&t, "7.4.78"), "got {t:?}");
    assert!(!t.contains(&"7.4.76".to_string()), "got {t:?}");
}
```

If `at` takes `&[String]` rather than `&Vec<String>`, the `at(t, …)` calls on a `&Vec<String>` still coerce. If the signature differs, check it in `crates/panini/tests/common/` (or wherever `helpers` lives) and adapt the call, not the helper. If `Lakara::VidhiLin` is spelled differently in `panini_data`, use the spelling the existing pins use.

- [ ] **Step 2: Run the trace suite**

Run: `mise exec -- cargo test -p panini --test trace 2>&1 | tail -20`
Expected: PASS.

If a pin fails, fix the engine, never the pin. The pins encode the spec's rule table.

- [ ] **Step 3: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini/tests/trace/juhotyadi.rs
git commit -m "test(trace): 3c2 pins — jahAti, jahAhi's bars, jahitaH, jahyAt, jigAti

Claude-Session: https://claude.ai/code/session_01TFmGpY78kwhk6ypKTXzJwn"
```

---

## Task 7: Audit, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (audit prose), stale engine comments, `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md` (one note)

**Interfaces:**
- Consumes: the finished engine and goldens (Tasks 1–6). Produces no symbols.

- [ ] **Step 1: Update the audit harness's asserted totals**

In `tools/audit/panini_full_audit.rs`, update the header prose and asserts:
- `85 roots, 3852 cells, 4823 forms` → `87 roots, 3924 cells, 4926 forms`
- `428 root×pada×lakāra blocks × 9 cells, plus 971` → `436 … plus 1002`
- `the full 3852-cell table` → `3924-cell`
- `assert_eq!(roots_seen.len(), 85, …)` → `87`
- `assert_eq!(n_cells, 3852, "cells: 428 root×pada×lakāra blocks × 9")` → `3924`, `"cells: 436 root×pada×lakāra blocks × 9"`
- `assert_eq!(n_forms, 4823, "forms: 3852 cells + 971 ALTERNATES rows")` → `4926`, `"forms: 3924 cells + 1002 ALTERNATES rows"`

The header's ubhayapadī count does not change, because both new roots are parasmaipada-only.

In `tools/audit/README.md`, `(85 roots, 3852 cells, 4823 forms)` → `(87 roots, 3924 cells, 4926 forms)`.

- [ ] **Step 2: Repoint vidyut's dev-deps at THIS worktree and run the audit**

`/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes absolute dev-dep paths. The brainstorm left them pointing at `/workspace/crates`, the `main` checkout, which does not have this slice. Auditing without repointing checks the pre-slice engine.

```bash
WT="$(git rev-parse --show-toplevel)"
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
cp tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" mise exec rust@1.98.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -15)
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.98.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -8)
```

Copy the committed harness; never rewrite it.
- Expected from the honest run: `AUDIT PASSED: 3924 cells, 4926 forms, zero differences.`
- Expected from the `entry` control: it fails with exit 1 and 36 √bhū cells.

If the honest run shows differences, stop. The goldens passed, so the engine and vidyut disagree on a form the goldens do not pin. Report it rather than editing anything.

- [ ] **Step 3: Record the audit**

In `tools/audit/README.md`, immediately under `## Last recorded result`, add a new dated entry above the 3c one (use today's date):

```markdown
YYYY-MM-DD, juhotyādi 3c2 slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 3924
cells / 4926 forms / 87 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole juhotyādi 3c2 slice: `Rule.bars` (the apavāda
relation declared on a rule and enforced by `run_pipeline`), 7.4.78
*bahulaṁ chandasi* for √gā on the Kaumudī's authority, 6.4.118 *lopo yi*,
and the vikalpas 6.4.117 *ā ca hau* (text-neutral, barring 6.4.116 / 6.4.113
/ 6.4.112) and 6.4.116 *jahāteś ca* — added for √hā parasmaipada (`03.0009`)
and √gā (`03.0026`).

Totals: 87 = 85 + 2; 3924 = 3852 + 72 (8 root×pada×lakāra blocks × 9); 4926 =
4823 + 72 + 31 new `ALTERNATES` rows (971 → 1002), measured via the harness's
corpus block, not assumed.
```

In `crates/panini/tests/paradigm/main.rs`'s audit-chain doc comment, find the sentence that records juhotyādi 3c's audit (`… over all 3852 cells / 4823 forms / 85 roots with zero differences, its \`entry\` negative control verified failing (36 √bhū cells).`). Extend it with `, and juhotyādi 3c2's re-ran the same probe at the same commit over all 3924 cells / 4926 forms / 87 roots with zero differences, its \`entry\` negative control verified failing (36 √bhū cells)`.

- [ ] **Step 4: README.md**

- `**partial** at 8 of its 26 dhātupāṭha rows` → `**partial** at 10 of its 26 dhātupāṭha rows`.
- After the juhotyādi sentence's slice-3c clause (it ends `… and 8.2.40 given its *adhaḥ*.`), insert before the sentence that follows: `Slice 3c2 added √hā parasmaipada (\`03.0009\`, *jahāti*) and √gā (\`03.0026\`, *jigāti*, by the chāndasa 7.4.78 on the Kaumudī's authority), with 6.4.118 *lopo yi* and two more vikalpas, 6.4.116 *jahāteś ca* and 6.4.117 *ā ca hau*. The latter changes no text and instead bars the rules that would, the first rule to declare an apavāda relation (\`Rule.bars\`).`
- `curated 85-root set` → `curated 87-root set`.
- In the multi-form paragraph:
  - `719 of the 3852 cells hold more than one form: 554` / `hold two, 121 hold three (` → `740 of the 3924 cells hold more than one form: 571 hold two, 123 hold three (`.
  - Inside that three-form parenthesis, after `√dā's and √dhā's`, add `, and — new in slice 3c2 — √gā's`.
  - `nine hold` / `five (and, new in slice 3b, √bhī's loṭ prathama eka, forking on` / `7.1.35/6.4.115/8.4.56)` → `ten hold five (and, new in slice 3b, √bhī's loṭ prathama eka, forking on 7.1.35/6.4.115/8.4.56, and — new in slice 3c2 — √hā's, forking on 7.1.35/6.4.116/8.4.56)`.
  - Replace the sentence `Nothing in the suite` / `forks deeper than six.` with: `One cell — new in slice 3c2 — holds seven: √hā's (\`03.0009\`) loṭ parasmaipada madhyama eka, \`jahIhi\` / \`jahihi\` / \`jahAhi\` / \`jahItAd\` / \`jahItAt\` / \`jahitAd\` / \`jahitAt\`, where 6.4.117 *ā ca hau* keeps the \`A\` by barring the rules that would change it. Nothing forks deeper than seven.`
  - Check: 571 + 123 + 18 + 10 + 17 + 1 = 740.

Reflow the paragraph to the file's existing ~80-column wrap.

- [ ] **Step 5: docs/ARCHITECTURE.md**

- Stage table: the `abhyasa.rs` row gains `7.4.78` after `7.4.76`. In the `guna.rs` row, `6.4.119, 6.4.113, 6.4.112, 6.4.115` → `6.4.119, 6.4.118, 6.4.117, 6.4.116, 6.4.113, 6.4.112, 6.4.115`.
- `pins all 114 ids verbatim` → `pins all 118 ids verbatim`. Confirm by counting the `expected` array entries in `tinanta_rule_order_is_pinned`. After the clause that ends the 3c four (`… 8.2.38 *dadhas tathoś ca* — 114 total`) add ` — then juhotyādi 3c2's four: 7.4.78 *bahulaṁ chandasi*, 6.4.118 *lopo yi*, 6.4.117 *ā ca hau* and 6.4.116 *jahāteś ca* — 118 total`.
- `**partial** at 8 of its 26 rows (… √dā, √dhā, √mā, √hā; slice 3c)` → `**partial** at 10 of its 26 rows (… √dā, √dhā, √mā, √hā; slice 3c; √hā parasmaipada, √gā; slice 3c2)`.
- In `## Optional rules and the derivation set`, after the paragraph beginning `` `run_pipeline` carries the branches as a worklist. ``, add:

```markdown
A rule can also **bar** others. `Rule.bars` lists rule ids that the controller
skips on every branch where the barring rule fired: the apavāda relation,
declared once on the apavāda instead of re-derived in each overridden rule's
guard. A vikalpa's bars land on its applied clone only, and later forks of a
barred branch inherit them. The first user is 6.4.117 *ā ca hau*, which keeps
√hā's `A` before *hi* by changing no text: its branch is barred from 6.4.116,
6.4.113 and 6.4.112, the three rules that would change that `A`. So √hā's loṭ
madhyama eka holds **seven** forms, three readings before *hi* (*jahIhi*,
*jahihi*, *jahAhi*) plus four tātaṅ forms. The bars make the branch count
fall short of the 2^k product: 6.4.117's branch never reaches 6.4.116's fork.
`exactly_the_pinned_bars` checks every barred id runs after its barrer.
Scope is the branch, which is the same as the site while a tinanta prakriyā
has one aṅga.
```

- The paragraph beginning `Juhotyādi 3b added a third six-form mechanism` says `nothing in the suite exceeds six`. Change that clause to `nothing exceeded six until slice 3c2's seven-form cell (below)`, and `joining the eight-cell five-form record above and taking it to nine` → `… to nine (slice 3c2's √hā loṭ prathama eka takes it to ten)`.
- `Nine rules are optional, in pipeline order:` → `Eleven rules are optional, in pipeline order:`, with `**6.4.117** *ā ca hau*, **6.4.116** *jahāteś ca*,` inserted immediately before `**6.4.115** *bhiyo'nyatarasyām*`. Update any following "of the nine" to "of the eleven", and append a sentence: `Slice 3c2 added 6.4.117 and 6.4.116, both root-keyed to √hā (\`03.0009\`), the first a text-neutral vikalpa made effective by \`Rule.bars\`.`
- The 7.1.35 / 8.4.56 accounting:
  - `forking 120 cells` → `124`
  - `across the 60 roots with a parasmaipada column` → `62`
  - `joined by juhotyādi's √hu, √ki, √bhī and √hrī (all parasmaipada-only)` → `joined by juhotyādi's √hu, √ki, √bhī, √hrī, √hā (\`03.0009\`) and √gā (all parasmaipada-only)`
  - `60 + 25 = the 85 curated roots` → `62 + 25 = the 87 curated roots`
  - `forking 142 cells outright` → `146`
  - `across those same 60 parasmaipada columns (120 of` → `62 … (124 of`

- [ ] **Step 6: AGENTS.md**

- Rules of the codebase: `(\`crates/panini/tests/paradigm/\`, 3852 cells,` → `3924 cells`. In the juhotyādi clause, change `and now at 8 of its 26 after slice 3c curated √dā, √dhā, √mā and √hā (ātmanepada) —` to `at 8 after slice 3c curated √dā, √dhā, √mā and √hā (ātmanepada), and now at 10 of its 26 after slice 3c2 curated √hā (parasmaipada) and √gā —`. Change `(971 rows in all, so 3852 + 971 = 4823 forms total)` → `(1002 rows in all, so 3924 + 1002 = 4926 forms total)`.
- The audit record: after `… 2026-09-12 entry, 3852 cells / 4823 forms / 85 roots)` add `, and that by juhotyādi 3c2's (\`tools/audit/README.md\`'s YYYY-MM-DD entry, 3924 cells / 4926 forms / 87 roots)`.
- The stale-comment paragraph: `3852 goldens` → `3924`. After `the corpus stands at 3852 cells as of 3c` add `. Juhotyādi 3c2 touched neither comment either; the corpus stands at 3924 cells as of 3c2`. Then recompute both anchors rather than guessing:
  - `grep -n "1872 goldens" crates/panini-prakriya/src/tinanta/guna.rs` gives the new `guna.rs` line.
  - `grep -n "1800 goldens" crates/panini-prakriya/src/controller.rs` gives the new `controller.rs` line. Task 1 changed this file, so the old anchor has moved.
  - Write the measured lines in.
- The `6.4.115 landed in slice 3b, the engine's ninth vikalpa rule` paragraph: append `Slice 3c2's 6.4.117 and 6.4.116 are the tenth and eleventh.`
- Add a Rules-of-the-codebase bullet immediately after the `A guard that names particular roots keys on the dhātupāṭha number…` bullet:

```markdown
- **An apavāda that must stop other rules on its branch declares it in
  `Rule.bars`; never in the overridden rules' guards, and never by reading
  `p.log`.** `run_pipeline` enforces the bar per branch (a vikalpa's on its
  applied clone only), and `exactly_the_pinned_bars` requires every barred id
  to run after its barrer. 6.4.117 *ā ca hau* is the first: it changes no text,
  so without its bars 6.4.116, 6.4.113 and 6.4.112 would each still rewrite
  the `A` it keeps. 7.1.6's read of `p.log` for 7.1.5 is an ENABLING condition,
  not a bar, and stays as it is.
```

- [ ] **Step 7: Stale engine comments**

- `abhyasa.rs`: both `the 3852 priors break` → `the 3924 priors break`.
- `guna.rs`: 6.4.77's `The 3852 byte-identical` → `The 3924 byte-identical`; `85-root × 4-lakāra grammar` → `87-root × 4-lakāra grammar`.
- `abhyasa.rs`, 7.4.76's comment: after `…taking no 7.4.76 (jahAti).` the comment is still correct; change nothing. Confirm with `grep -n "3c2" crates/panini-prakriya/src/tinanta/*.rs`. Every comment that says 3c2 *will* do something must now say it did, or be deleted if the sentence only existed to promise it.
- `sound.rs`: if `hrasva_of_long_vowels_all_arms` or any table test lists witnesses by slice, check whether √hā or √gā adds a witness to an arm the comment calls unwitnessed. Both are `A`-final, so the `A` arm already has 3c's witnesses and nothing changes. Confirm by reading the comment.

- [ ] **Step 8: Note the slice in 3a's spec**

In `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md`, after the existing slice-3c note beneath the "Later slices" table, add:

```markdown
> Slice 3c2 (`2026-09-29-juhotyadi-gana-3c2-design.md`) took √hā
> (parasmaipada) and √gā, 72 cells, and added `Rule.bars` for 6.4.117.
```

- [ ] **Step 9: Sweep for anything left stale**

```bash
grep -rn "3852\|4823\|\b971\b\|85 roots\|85-root\|of these 85\|\b428\b\|719 of\|8 of its 26\|eight of its twenty-six\|114 ids\|nine optional\|Nine rules are optional\|ninth vikalpa\|60 + 25\|exceeds six\|six-form record" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.md --include=*.rs
grep -rn "3c2\|slice 3c2\|3d–3f\|arrives in 3c2" crates --include=*.rs
grep -rn "\bhA\b\|\bgA\b\|jahAti\|jah\b\|jig\|√hā\|√gā" crates/panini-prakriya/src --include=*.rs | grep "//"
grep -rn "6\.4\.11[2-8]\|7\.4\.7[68]" crates/panini-prakriya/src --include=*.rs | grep -i "defer\|later slice\|not implemented\|not yet\|3c2 must\|no curated"
```

Expected residue: AGENTS.md's dated mutation-record entries (history; never rewrite them), prose that names 3c's numbers explicitly as 3c's, and "six-form record" sentences that are historically true as written. Fix any other hit. "Ninth vikalpa" is fine only where the sentence dates it to slice 3b.

- [ ] **Step 10: Run the full suite**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS at 3924 cells.

- [ ] **Step 11: Commit**

```bash
mise run fmt && mise run lint
git add -A
git commit -m "docs: 3c2's counts, the audit record, Rule.bars, and the comments it falsifies

3924 cells / 4926 forms / 87 roots / 1002 ALTERNATES across README,
ARCHITECTURE, AGENTS, paradigm/main.rs and tools/audit. Audit at zero
divergence against 8da2f90b. ARCHITECTURE documents Rule.bars and the
seven-form cell; AGENTS gains the rule that an apavāda bars by declaration.

Claude-Session: https://claude.ai/code/session_01TFmGpY78kwhk6ypKTXzJwn"
```

---

## Task 8: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and a dated campaign entry); `mise.toml` only if the cap must move

Follow AGENTS.md's cargo-mutants protocol exactly. Three hazards from this repo's record apply:
- **Measure, never scale.**
- **Every invocation rotates `mutants.out`**, so always pass `-o`.
- **The mise shim fails in background shells, and background shells die at ~60 minutes**, so the campaign runs detached on the real binary.

- [ ] **Step 1: Run the exhaustive tier once**

Run: `mise run test-full 2>&1 | tail -20` (foreground, timeout 3600000 ms; ~25 minutes)
Expected: PASS, including `roundtrip_exhaustive` over all 4926 forms.

- [ ] **Step 2: Measure the uncontended floor**

With nothing else running: `time mise run test 2>&1 | grep -E "Running|finished in|real"` (foreground). Record the wall clock and the times of the paradigm, roundtrip and trace binaries. Compare with the 3852-cell floor: 69.998 s total, with paradigm 29.26 s, roundtrip 34.38 s and trace 5.00 s. `roundtrip_sampled` is Θ(N²) in roots, so expect it to grow faster than the cell count (+1.87%).

- [ ] **Step 3: Measure a full UNCAUGHT run at `-j 4`**

The two documented equivalent mutants run the suite to completion uncaught. They sat at `adesha.rs:574:30` (`replace + with *`) and `tripadi.rs:1186:38` (`replace - with /`) on `main`. This slice touches neither file above them, but locate them by source text anyway:

```bash
git show main:crates/panini-prakriya/src/tinanta/adesha.rs | sed -n 574p   # the `s.remove(pos + 1);` line
git show main:crates/panini-prakriya/src/tinanta/tripadi.rs | sed -n 1186p
grep -n "s.remove(pos + 1);" crates/panini-prakriya/src/tinanta/adesha.rs
grep -nF "$(git show main:crates/panini-prakriya/src/tinanta/tripadi.rs | sed -n 1186p)" crates/panini-prakriya/src/tinanta/tripadi.rs
CM="$(mise which cargo-mutants)"
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:<LINE>:30: replace \+ with \*|tripadi.rs:<LINE>:38: replace - with /"
```

Task 1's mechanical `bars: &[],` insertion adds one line per rule above each site, so **both positions will have moved**. Substitute the new line numbers for `<LINE>`, confirm `--list` shows exactly those two mutants, then run them (foreground, timeout 1800000 ms):

```bash
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 600 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:<LINE>:30: replace \+ with \*" --re "tripadi.rs:<LINE>:38: replace - with /" 2>&1 | tail -10
```

Both must be MISSED, not TIMEOUT. Read each one's test-phase duration from `$SCRATCH/mutants.out/outcomes.json`.

**Cap rule:** keep `--timeout 600` if 600 ÷ the longer test phase is at least 5×. Otherwise set the cap to 6 × the longer phase, rounded up to the next 100 s, and change `mise.toml` and AGENTS.md's floor paragraph together.

- [ ] **Step 4: Launch the campaign detached**

```bash
OUT="$HOME/mutants-records/juhotyadi-3c2"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
CM="$(mise which cargo-mutants)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 600 -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"
```

Use the cap from Step 3 if it moved. The 3852-cell campaign took 1 h 32 min.
- Check progress with `tail -3 "$OUT/campaign.log"`.
- Check liveness with `pgrep -x cargo-mutants`, **not** `pgrep -f`, which matches its own shell.
- Run nothing CPU-heavy in the meantime, because contention inflates test phases toward the cap.

- [ ] **Step 5: Read both outcome files**

When `pgrep -x cargo-mutants` returns nothing:

```bash
date -u +"%F %T UTC" > "$OUT/finished"
tail -5 "$OUT/campaign.log"
cat "$OUT/mutants.out/missed.txt" "$OUT/mutants.out/timeout.txt"
```

Expected:
- `missed.txt` holds exactly the two documented equivalents at their Step 3 positions.
- `timeout.txt` holds exactly the permanent `tripadi.rs` ṇatva-scan `replace -= with /=`. It was at `:1506:23` on `main`; find its new line the same way.
- `cargo-mutants` exits 3 when timeouts are present, which is expected.

If the result differs:
- Any **other timeout** is a suspect survivor. Re-run it alone with its own `-o` directory and `--re` before concluding anything.
- Any **other missed** mutant in 3c2's code means a guard test does not separate the mutant. The likeliest sources:
  - `controller.rs`'s `||` in the skip test, or either `extend_from_slice` (Task 1's six tests should kill all three)
  - 6.4.118's `has(Ngit) || starts_with('y')`
  - 6.4.117's `ends_with('A')`
  - 6.4.116's `len() <= ENDING`
  - 7.4.78's number test

  Strengthen the named guard test, commit it, and re-run only those mutants with `--re` and a fresh `-o`.

- [ ] **Step 6: Keep the outcomes and compute the margins**

`$OUT` is already durable; do not run another `cargo-mutants` against it. Inspect one record first to see how `duration` is stored:

```bash
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["outcomes"][1])' "$OUT/mutants.out/outcomes.json"
```

Then compute the caught mutants' test-phase min / median / p90 / p99 / max, and the two margins AGENTS.md records:
- 600 ÷ the longest uncaught test phase (from Step 3, and from the two missed mutants' phases in this campaign)
- 600 ÷ the slowest caught mutant

- [ ] **Step 7: Record it in AGENTS.md**

- Rewrite the `**The floor behind the 600s cap, measured at 3852 cells.**` paragraph with Step 2's and Step 3's measured numbers at 3924 cells. If the cap moved, update it too, with `mise.toml` changed in the same commit.
- Append a dated entry after the `2026-09-12 — slice 3c re-measured both at 3852 cells.` entry, in its style. It records:
  - cell growth 3852 → 3924 (+1.87%) and the floor's measured move
  - the campaign window (`started`/`finished`) and wall clock
  - **mutants / caught / unviable / missed / timeout** counts summing to the total
  - `missed.txt` and `timeout.txt` **named verbatim with their new positions**
  - the caught-mutant duration distribution
  - both margins
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`
  - the non-caught set diffed against 3c's (the clean result is an identical set at drifted positions)

- [ ] **Step 8: Commit**

```bash
git add AGENTS.md mise.toml
git commit -m "chore: 3c2 mutation gate — floor and uncaught run re-measured at 3924 cells

missed.txt holds only the two documented equivalents and timeout.txt only
the permanent ṇatva-scan entry, at drifted positions; the cap was checked
against a measured -j 4 uncaught run, not scaled.

Claude-Session: https://claude.ai/code/session_01TFmGpY78kwhk6ypKTXzJwn"
```

---

## Task 9: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -20
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin juhotyadi-3c2
gh pr create --title "juhotyādi 3c2 — √hā (parasmaipada), √gā, and Rule.bars" --body "$(cat <<'BODY'
Slice 3c2 curates √hā parasmaipada (`03.0009 o~hA\k`, jahāti) and √gā
(`03.0026`), taking the golden suite from 3852 to 3924 cells and juhotyādi
from eight of its twenty-six rows to ten.

**`Rule.bars`: the apavāda relation as data.** 6.4.117 *ā ca hau* keeps √hā's
`A` before *hi* by changing no text; what makes it effective is that 6.4.116,
6.4.113 and 6.4.112 do not run on its branch. Rather than teach those three
guards to read the log, a rule now declares the rules it bars, `run_pipeline`
skips them on that branch (a vikalpa's applied clone only), and a pin checks
every barred id runs after its barrer.

**New rules:** 7.4.78 *bahulaṁ chandasi* (chāndasa, applied to √gā on the
Kaumudī's authority, as vidyut does), 6.4.118 *lopo yi*, and the vikalpas
6.4.117 and 6.4.116. Eleven optional rules; the first seven-form cell (√hā
loṭ madhyama eka).

Re-probed against vidyut and against this engine's HEAD cell by cell before
the spec; audit at zero divergence against `8da2f90b`; mutation gate clean.

https://claude.ai/code/session_01TFmGpY78kwhk6ypKTXzJwn
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Per the standing instruction:
1. Once checks are green, merge with `gh pr merge --merge --auto`.
2. Confirm the branch's commits are on `main`: after `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. Delete the remote and local branch, and remove the worktree with `git worktree remove .worktrees/juhotyadi-3c2`, run from the main checkout.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| `Rule.bars`, `Prakriya.barred`, controller enforcement, branch-not-site scope note | 1 |
| The "every barred id runs after its barrer" pin | 1, filled in 4 |
| 7.4.78, with the SK attribution | 2 |
| 6.4.118 | 3 |
| 6.4.117 with its three bars; 6.4.116 with the ordering-caveat argument | 4 |
| Eleven optional rules | 4, and `VIKALPA_RULES` in 5 |
| Rows, counts, buckets including `sevens`, the 31 `ALTERNATES` rows by key | 5 |
| Trace pins (*jahAti* without 7.4.76, *jahAhi* barring three, *jahihi*/*jahitaH*, *jahyAt*, *jigAti*) | 6 |
| Audit with dev-dep repoint and negative control; README/ARCHITECTURE/AGENTS sweep, including the four stale-count classes and 3c2's own promise-comments | 7 |
| `test-full`, the re-measured floor, the uncaught run and the verbatim non-caught record | 8 |
| "7.1.6's log read is not migrated" | recorded in Task 1's `bars` doc and Task 7's AGENTS bullet; no code |

**Type consistency.**
- `Rule.bars: &'static [&'static str]` and `Prakriya.barred: Vec<&'static str>` (Task 1) are read in Task 4's `rule.bars` assertion and the controller tests.
- `jaha_prakriya(&str, bool) -> Prakriya` is defined in Task 3 and reused in Task 4.
- `run_ha_rules(Prakriya) -> Vec<Prakriya>` is defined and used only in Task 4.
- `ha_branches` and `branch` are defined and used only in Task 6.
- Rule ids `"7.4.78"`, `"6.4.118"`, `"6.4.117"`, `"6.4.116"` match across rule bodies, both pins, `VIKALPA_RULES`, the `ALTERNATES` keys and the trace assertions.

**Known soft spots.**
- **Pinned-order edits are cumulative.** Task 3 sees `"6.4.119", "6.4.113"`; Task 4 sees `"6.4.119", "6.4.118", "6.4.113"`.
- **Mutant positions drift.** Task 1's mechanical insertion moves every mutant line number in the crate, so Task 8 locates the known mutants by source text.
- **Hand-built √hā texts.** Task 4's `JahIhi`-style texts come from the hand-built abhyāsa `Ja`. The real derivation's `jah-` comes from 8.4.54 later, which is why the goldens and trace pins spell `jah-`.
