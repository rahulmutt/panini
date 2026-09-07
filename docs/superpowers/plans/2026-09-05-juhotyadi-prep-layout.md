# Juhotyādi prep — the five-slot term layout Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Give the tiṅanta pipeline two permanent leading term slots — `AGAMA` for laṅ's aṭ/āṭ and `ABHYASA` for the reduplicant juhotyādi will need — and move the laṅ augment out of the aṅga's text into its slot, with all 3492 cells and every pinned trace byte-identical.

**Architecture:** `terms.rs` renumbers its constants (`AGAMA` 0, `ABHYASA` 1, `ANGA` 2, `ENDING_PRE_SHAP`/`SHAP` 3, `ENDING` 4) and gains one helper, `with_slots`, that `derive` and every hand-built test prakriya use to build a term vector in the layout. Because `Prakriya::text` concatenates every term, an empty leading slot contributes nothing and every logged before/after string is unchanged. Exactly three rules change behaviour: 6.4.71 and 6.4.72 write their augment into `AGAMA`, and 6.1.90's aṅga arm merges `AGAMA`'s `A` into the first non-empty term after it and empties the slot. No new gaṇa, no new rule, no golden regenerated.

**Tech Stack:** Rust 1.98 pinned via `mise`; `cargo test` golden suite; `cargo-mutants` mutation gate; `vidyut-prakriya` at the vendored commit via the committed harness in `tools/audit/`.

**Spec:** `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md` — the "Prep: the term layout" section is the contract; the "Slice 3a" section is what this layout is for and is NOT built here. Slice 3a gets its own plan after this one merges.

**Branch:** `juhotyadi-prep`, from `main` after the spec commit (`ae01a19`). Execute in a worktree per `superpowers:using-git-worktrees`.

## Global Constraints

- **Toolchain:** rust **1.98** via `mise`. `mise run test -- -p X` does NOT scope — use `mise exec -- cargo test -p X`.
- **Byte-identical goldens at EVERY commit.** This plan adds no root and changes no form. After every task, `mise run test` passes AND `git status --short crates/panini/tests` is empty. A passing suite IS the byte-identity check — the goldens assert every cell and every pinned trace — so never regenerate a golden to make a task pass; a failing golden is a defect in that task.
- **Run the golden suite in the FOREGROUND with the largest timeout the harness allows.** The floor is 1958.411s at 3492 cells (~33 minutes; paradigm 866s, roundtrip 1087s). If the harness caps a single command below that, launch it detached to a log (`nohup mise run test > /tmp/prep-suite.txt 2>&1 &`) and wait on the log with a monitor loop until the final `test result:` lines appear — never end a turn while the suite runs; a backgrounded suite that is not waited on is orphaned.
- **The augment-slot rules are the ONLY behaviour change.** Tasks 1 and 4 must not touch a rule body; Tasks 2 and 3 touch exactly 6.4.71, 6.4.72, 6.1.90 (and one 6.1.73 unit test's fixture). Any other rule edit is out of scope for this PR.
- **The audit's negative control runs first.** A zero-difference verdict without a verified-failing `entry` control proves nothing.
- **`mise run mutants` is `-j 4 --timeout 4800`.** Run the task, don't reconstruct flags. `CARGO_MUTANTS_JOBS` in the environment can defeat `-j`; check it is unset.
- **SLP1 throughout.** `M` anusvāra, `N` velar ṅ, `Y` palatal ñ, `R` retroflex ṇ, `z` retroflex ṣ, `f` vocalic ṛ, `E`/`O` ai/au.
- **Executing from a worktree:** the audit task repoints `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml`'s `panini`/`panini-data` dev-dep paths at THE CHECKOUT BEING AUDITED. A stale path audits the wrong engine (this bit a prior slice); set it explicitly in Task 5.

## Numbers this plan changes

None. `dhatus().len()` stays 77, `PARADIGM` stays 388 blocks / 3492 cells, forms stay 4399, `VIKALPA_RULES` stays 8, the pinned rule order stays at its current 99 ids. The only number that moves is the term-vector length: every prakriya now has **two more terms** than before, both empty except in laṅ (`AGAMA`).

## What the planning-time reading settled

- **Only three sites read the aṅga's first character or the aṭ log:** 6.4.71 (`anga.rs:24`), 6.4.72 (`anga.rs:43` and the log scan at `:54`), and 6.1.90's aṅga arm (`adesha.rs:344-356`). Everything else that could care — `following_sarvadhatuka`, `sound_before_ending`, `word_chars`, the tripādī scans, 6.1.73 — reads the nearest non-empty term or the assembled word, and is layout-agnostic.
- **48 lines index `terms` by a literal number**, all in test code: `guna.rs` (34, in the √kṛ tests from line 1132 on), `vikarana.rs` (2, line 585-586), `controller.rs` (5), `lib.rs` (1), `it_samjna.rs` (6). The last three files test the generic `Prakriya`/pipeline, not the tiṅanta layout, and stay as they are. The first two migrate to the constants.
- **Every `terms: vec![...]` literal is single-line** (roughly 185 across the `tinanta/` test modules), so one `sed` wraps them all in `with_slots(...)`. Seven test sites build a prakriya by `p.terms.push(...)` instead (`samjna.rs:272, 288, 549, 575, 599`; `tin.rs:777, 800`) and are edited by hand.
- **No production code compares `terms.len()` against a literal.** The `p.terms.len() > SHAP` guards are constant-relative and survive.
- **6.4.72's log scan becomes dead, not redundant.** Once 6.4.71 no longer prefixes `a` onto `ANGA`, `is_vowel(first)` alone tells a genuinely vowel-initial aṅga from a consonant-initial one, and the spec's suggested replacement — "is `AGAMA` empty" — would be a clause no input can falsify (6.4.71 declines for every vowel-initial aṅga, so the slot is always empty when 6.4.72 looks). An unkillable clause is a guaranteed mutation survivor; the scan is deleted with no replacement and the comment records why.
- **6.1.73's tuk lands in `AGAMA`.** `word_chars` finds the short vowel at `(AGAMA, 0)` and inserts `t` after it, so the augment term reads `at` for √chid/√chṛd in laṅ and the word is `atCid…` → `acCid…` exactly as before. The rule is untouched; its unit test's fixture changes shape.
- **The `ends_with("kar")` / `ends_with("kur")` / `ends_with("SI")` guards stay.** They were written for an augment that no longer sits in the text, but the guard tolerance is harmless and their unit tests (`guna.rs:1550`, `:1567`, `tripadi.rs:1678`) still kill the `ends_with → ==` mutant. Comments are reworded in Task 4; guard bodies are not touched (a guard change is grammar review, outside this PR).

## File Structure

| file | responsibility | task |
|---|---|---|
| `crates/panini-prakriya/src/tinanta/terms.rs` | `AGAMA`, `ABHYASA`, renumbered constants; `with_slots`; layout doc | 1, 4 |
| `crates/panini-prakriya/src/tinanta/mod.rs` | `derive` builds through `with_slots` | 1 |
| `crates/panini-prakriya/src/tinanta/{samjna,tin,vikarana,anga,guna,adesha,tripadi,derivation_tests}.rs` (test modules) | hand-built prakriyas through `with_slots`; literal indices → constants | 1 |
| `crates/panini-prakriya/src/tinanta/anga.rs` | 6.4.71 and 6.4.72 write `AGAMA`; 6.1.73 test fixture; comments | 2, 3, 4 |
| `crates/panini-prakriya/src/tinanta/adesha.rs` | 6.1.90 aṅga arm reads `AGAMA` | 3 |
| `crates/panini-prakriya/src/tinanta/{guna,tripadi,tin,derivation_tests}.rs` | stale "aṭ is prefixed onto the text" comments | 4 |
| `docs/ARCHITECTURE.md`, `AGENTS.md` | five-slot layout prose; timing paragraph | 4, 6 |
| `tools/audit/README.md` | "Last recorded result" entry | 5 |

---

### Task 1: the five-slot layout, `with_slots`, and the test migration

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/terms.rs:12-22` (constants) and the test module at `:229+`
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs:70-103` (`derive`)
- Modify: every `tinanta/*.rs` test module (mechanical migration, see Step 4)
- Test: `crates/panini-prakriya/src/tinanta/terms.rs` (tests module), `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Produces: `pub(crate) const AGAMA: usize = 0;`, `pub(crate) const ABHYASA: usize = 1;`, `ANGA = 2`, `ENDING_PRE_SHAP = 3`, `SHAP = 3`, `ENDING = 4`; and `pub(crate) fn with_slots(terms: Vec<Term>) -> Vec<Term>` in `crate::tinanta::terms`. Tasks 2–3 read `AGAMA`; slice 3a reads `ABHYASA`.

- [ ] **Step 1: Write the failing layout tests**

In `terms.rs`'s existing `mod tests`, add:

```rust
    #[test]
    fn with_slots_seats_the_callers_terms_behind_two_empty_leading_slots() {
        // The layout contract every rule addresses terms through: AGAMA
        // and ABHYASA exist on every prakriya and are empty until 6.4.71/
        // 6.4.72 (laṅ) or 6.1.10 (juhotyādi) fill them. text() ignores
        // them, which is what keeps every existing golden byte-identical.
        let terms = with_slots(vec![Term::new("BU"), Term::new("a"), Term::new("ti")]);
        assert_eq!(terms.len(), ENDING + 1);
        assert_eq!(terms[AGAMA].text, "");
        assert_eq!(terms[ABHYASA].text, "");
        assert_eq!(terms[ANGA].text, "BU");
        assert_eq!(terms[SHAP].text, "a");
        assert_eq!(terms[ENDING].text, "ti");
        let p = Prakriya {
            terms,
            ..Default::default()
        };
        assert_eq!(p.text(), "BUati");
        assert_eq!(p.snapshot(), "BUati");
        // The pre-3.1.68 shape: dhātu + ending only.
        let terms = with_slots(vec![Term::new("BU"), Term::new("ti")]);
        assert_eq!(terms.len(), ENDING_PRE_SHAP + 1);
        assert_eq!(terms[ENDING_PRE_SHAP].text, "ti");
    }
```

In `derivation_tests.rs`, add (near `a_augment_does_not_leak_into_dual_or_plural`, line ~1230):

```rust
#[test]
fn derive_seats_the_dhatu_at_anga_behind_two_empty_slots() {
    // `derive` and `with_slots` must agree on the layout, or a hand-built
    // test prakriya and a real derivation address different terms by the
    // same constant.
    let d = dhatus().iter().find(|d| d.dhatupatha == "01.0001").unwrap();
    let p = sole(derive(
        d,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    ));
    assert_eq!(p.text(), "Bavati");
    assert_eq!(p.terms[AGAMA].text, "");
    assert_eq!(p.terms[ABHYASA].text, "");
    assert!(p.terms[ANGA].has(Tag::Dhatu));
    assert!(p.terms[SHAP].has(Tag::Vikarana));
    assert!(p.terms[ENDING].has(Tag::Tin));
}
```

Add to `derivation_tests.rs`'s imports: `use crate::tinanta::terms::{ABHYASA, AGAMA, ANGA, ENDING, SHAP};` (keep only the names it ends up using; clippy runs with `-D warnings`).

- [ ] **Step 2: Run to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya with_slots_seats 2>&1 | tail -20`
Expected: compile error — `cannot find value AGAMA` / `cannot find function with_slots`.

- [ ] **Step 3: Constants, `with_slots`, and `derive`**

In `terms.rs`, replace the constant block (`ANGA` through `ENDING`, lines 12-22) with:

```rust
/// Index of the laṅ augment — aṭ (6.4.71) or āṭ (6.4.72). Permanent slot,
/// empty in every other lakāra; 6.1.90 empties it again when the āṭ merges
/// into the following vowel. Stable across the pipeline.
pub(crate) const AGAMA: usize = 0;

/// Index of the abhyāsa — the reduplicant 6.1.10 ślau copies in front of
/// the aṅga for juhotyādi. Permanent slot, empty for every other gaṇa (and,
/// until slice 3a lands, for every derivation). Stable across the pipeline.
pub(crate) const ABHYASA: usize = 1;

/// Index of the aṅga (the dhātu) in `terms`. Stable across the pipeline.
pub(crate) const ANGA: usize = 2;

/// Index of the tiṅ ending *before* śap is inserted (3.1.68).
pub(crate) const ENDING_PRE_SHAP: usize = 3;

/// Index of śap once inserted, and of the ending thereafter.
pub(crate) const SHAP: usize = 3;
pub(crate) const ENDING: usize = 4;
```

Below the constants' NOTE block (before `following_sarvadhatuka`), add:

```rust
/// The pipeline's term vector for the terms a caller actually has: the two
/// permanent leading slots — `AGAMA` and `ABHYASA` — are prepended empty,
/// so `terms[ANGA]` is the first term the caller supplied. `derive` builds
/// every prakriya through this, and so does every hand-built test prakriya
/// in the stage files' test modules; routing both through one function is
/// what keeps a unit test and a real derivation addressing the same term by
/// the same constant.
pub(crate) fn with_slots(terms: Vec<Term>) -> Vec<Term> {
    let mut out = vec![Term::new(""), Term::new("")];
    out.extend(terms);
    out
}
```

In the NOTE block, fix the two index mentions: "`ENDING_PRE_SHAP` and `SHAP` are deliberately the same value (1)" → "(3)"; "shifts the ending from index 1 to index 2" → "from index 3 to index 4"; "(index 1, where the ending still lives)" → "(index 3, …)"; "(index 2, where it lives once śap has been inserted) and may address śap itself via `SHAP` (also index 1)" → "(index 4, …) … (also index 3)"; "panics indexing `terms[2]`" → "`terms[4]`". Task 4 writes the new layout paragraph; this step only keeps the existing numbers true.

In `mod.rs`, change `derive` so the dhātu term is built first and the vector is assembled through the helper:

```rust
    let mut p = Prakriya {
        ctx: Context::new(lakara, pada, purusha, vacana),
        ..Default::default()
    };
    let mut t = Term::new(dhatu.code);
    t.add(Tag::Dhatu);
    match dhatu.pada {
        PadaAssignment::Parasmaipada => {}
        PadaAssignment::Atmanepada => t.add(Tag::Atmanepadin),
        PadaAssignment::Ubhayapada => t.add(Tag::Ubhayapadin),
        PadaAssignment::UbhayapadaAnavane => t.add(Tag::Anavane),
    }
    match dhatu.gana {
        Gana::Divadi => t.add(Tag::Divadi),
        Gana::Tudadi => t.add(Tag::Tudadi),
        Gana::Adadi => t.add(Tag::Adadi),
        Gana::Kryadi => t.add(Tag::Kryadi),
        Gana::Svadi => t.add(Tag::Svadi),
        Gana::Rudhadi => t.add(Tag::Rudhadi),
        Gana::Tanadi => t.add(Tag::Tanadi),
        Gana::Bhvadi => {}
    }
    p.terms = terms::with_slots(vec![t]);
    run_pipeline(p, TINANTA_RULES)
```

- [ ] **Step 4: Migrate the hand-built test prakriyas**

Every `terms: vec![...]` literal in `tinanta/` is single-line. Wrap them:

```bash
cd crates/panini-prakriya/src/tinanta
grep -l 'terms: vec!\[' *.rs | xargs sed -i -E 's/terms: vec!\[(.*)\],(\s*)$/terms: with_slots(vec![\1]),\2/'
grep -rn 'terms: vec!\[' *.rs   # must print nothing
```

Then, by hand, the seven `push` sites — each `p.terms.push(X);` on a freshly built `Prakriya` becomes `p.terms = with_slots(vec![X]);`:
- `samjna.rs:272` (`pada_prakriya`), `:288` (`anavane_prakriya`), `:549`, `:575`, `:599`
- `tin.rs:777`, `:800`

Then the literal indices, only in the two files whose literals are all in test code:

```bash
sed -i -E 's/terms\[0\]/terms[ANGA]/g; s/terms\[1\]/terms[SHAP]/g; s/terms\[2\]/terms[ENDING]/g' guna.rs vikarana.rs
grep -n 'terms\[[0-9]\]' *.rs   # must print nothing
```

(`controller.rs`, `lib.rs`, `it_samjna.rs` keep their literal indices: they test the generic `Prakriya`/pipeline machinery with their own two- and one-term vectors and never use the tiṅanta constants.)

Then the imports. Each `tinanta/*.rs` test module (`mod tests` in `samjna.rs`, `tin.rs`, `vikarana.rs`, `anga.rs`, `guna.rs`, `adesha.rs`, `tripadi.rs`, `terms.rs`, and the top of `derivation_tests.rs`) needs `with_slots` in scope: add `use crate::tinanta::terms::with_slots;` inside the test module (in `terms.rs` the `use super::*;` already covers it). Compile; remove any import clippy reports as unused or redundant.

- [ ] **Step 5: Unit suite and lint**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -5` — Expected: all pass, including the two new tests.
Run: `mise run lint && mise run fmt-check` — Expected: clean.

- [ ] **Step 6: Full suite, byte-identical**

Run: `mise run test` (foreground, per the Global Constraints).
Expected: PASS across all binaries; `git status --short crates/panini/tests` prints nothing.

- [ ] **Step 7: Commit**

```bash
git add crates/panini-prakriya/src
git commit -m "refactor(engine): five-slot term layout — AGAMA and ABHYASA lead every prakriya

Constants renumbered, with_slots builds every term vector (derive and the
hand-built test prakriyas alike). Both new slots stay empty; text() ignores
them, so all 3492 cells and every trace are byte-identical."
```

---

### Task 2: 6.4.71 writes aṭ into the `AGAMA` slot

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/anga.rs:12-32` (6.4.71), `:84-87` (6.1.73's comment), the `che_ca_inserts_tuk_only_after_a_short_vowel` test (~`:571-583`)
- Test: `crates/panini-prakriya/src/tinanta/anga.rs` test module

**Interfaces:**
- Consumes: `AGAMA`, `with_slots` from Task 1.
- Produces: after 6.4.71, `terms[AGAMA].text == "a"` and `terms[ANGA].text` is the bare root. Task 3 relies on `ANGA`'s first character being the root's own.

- [ ] **Step 1: Write the failing guard tests**

In `anga.rs`'s `mod tests` (imports: `use crate::context::Context;`, `use crate::tinanta::terms::{AGAMA, with_slots};`, `use panini_data::{Purusha, Vacana};` — add whichever are absent; `Lakara`, `Pada`, `ANGA` arrive via `use super::*;`):

```rust
    #[test]
    fn at_augment_lands_in_the_agama_slot_not_the_anga_text() {
        // 6.4.71 luNlaNlfNkzvaqudAttaH. The augment is its own term now:
        // ANGA keeps the bare root, so no later guard has to tolerate a
        // leading `a` that is not the root's.
        let rule = rules().find(|r| r.id == "6.4.71").unwrap();
        let mut p = Prakriya {
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            terms: with_slots(vec![Term::new("BU"), Term::new("a"), Term::new("t")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[AGAMA].text, "a");
        assert_eq!(p.terms[ANGA].text, "BU");
        assert_eq!(p.text(), "aBUat");
        assert_eq!(p.log.last().unwrap().sutra, "6.4.71");
    }

    #[test]
    fn at_augment_declines_outside_lan_and_for_vowel_initial_angas() {
        let rule = rules().find(|r| r.id == "6.4.71").unwrap();
        // laṭ: no augment at all.
        let mut p = Prakriya {
            ctx: Context::new(
                Lakara::Lat,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            terms: with_slots(vec![Term::new("BU"), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[AGAMA].text, "");
        // laṅ, vowel-initial aṅga: 6.4.72's business, not this rule's.
        let mut p = Prakriya {
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            terms: with_slots(vec![Term::new("ad"), Term::new(""), Term::new("t")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[AGAMA].text, "");
    }
```

- [ ] **Step 2: Run to verify the first fails**

Run: `mise exec -- cargo test -p panini-prakriya at_augment 2>&1 | tail -20`
Expected: `at_augment_lands_in_the_agama_slot_not_the_anga_text` FAILS (`AGAMA` is `""`, `ANGA` is `"aBU"`); the decline test passes already.

- [ ] **Step 3: Rewrite 6.4.71**

Replace the rule's comment and body (`anga.rs:12-32`) with:

```rust
    // 6.4.71 luṅlaṅlṛṅkṣvaḍudāttaḥ: the aṭ-āgama precedes the aṅga in laṅ.
    //
    // Written into the permanent `AGAMA` slot, not prefixed onto the aṅga's
    // text: the aṅga's own first character and text stay the root's, so a
    // rule reading either needs no allowance for the augment. (Until the
    // juhotyādi prep this was a text prefix, which is why several guards
    // downstream match with `ends_with` — see their comments.)
    Rule {
        id: "6.4.71",
        name: "luNlaNlfNkzvaqudAttaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let first = p.terms[ANGA].text.chars().next().unwrap();
            if !matches!(p.ctx.lakara, Lakara::Lan) || is_vowel(first) {
                return false;
            }
            let before = p.snapshot();
            p.terms[AGAMA].text = "a".into();
            p.record("6.4.71", "luNlaNlfNkzvaqudAttaH", before);
            true
        },
    },
```

Add `AGAMA` to `anga.rs`'s top-level `use crate::tinanta::terms::{...}` line.

- [ ] **Step 4: Move the 6.1.73 test fixture to the new shape**

In `che_ca_inserts_tuk_only_after_a_short_vowel`, the first case currently builds `Term::new("aCi")` with the augment inside `ANGA`. Replace that case with:

```rust
        // The one site this corpus reaches: 6.4.71's aṭ before a C-initial
        // aṅga. The augment is its own term, so `word_chars` finds the short
        // vowel at (AGAMA, 0) and the tuk lands after it — in AGAMA, which
        // then reads `at`. The word is atCinadt exactly as before.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Ci"), Term::new("nad"), Term::new("t")]),
            ..Default::default()
        };
        p.terms[AGAMA].text = "a".into();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[AGAMA].text, "at");
        assert_eq!(p.terms[ANGA].text, "Ci");
        assert_eq!(p.text(), "atCinadt");
```

And rewrite the 6.1.73 rule comment's paragraph at `anga.rs:84-87` ("The tuk lands INSIDE `ANGA`, because 6.4.71 models the aṭ as a text prefix …") to:

```rust
    // The tuk lands in whichever term holds the short vowel — `AGAMA`, for
    // the laṅ aṭ that is this corpus's only site — because `word_chars`
    // addresses the whole word. ANGA's first character stays `C` and its
    // penult is untouched, so 6.4.72's `is_vowel(first)` guard and every
    // upadhā read below this point are unmoved.
```

- [ ] **Step 5: Unit suite and lint**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -5` — Expected: all pass (the whole-derivation tests `adadi_lan_singular_a_augment`, `a_augment_does_not_leak_into_dual_or_plural` and the √chid laṅ cells keep passing: the assembled word is unchanged).
Run: `mise run lint && mise run fmt-check` — Expected: clean.

- [ ] **Step 6: Full suite, byte-identical**

Run: `mise run test` (foreground). Expected: PASS; `git status --short crates/panini/tests` empty.

- [ ] **Step 7: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/anga.rs
git commit -m "refactor(engine): 6.4.71 writes laN's aT into the AGAMA slot

The aṅga's text is the root's again; 6.1.73's tuk follows the vowel into
the augment term. Assembled words unchanged, goldens byte-identical."
```

---

### Task 3: 6.4.72 writes āṭ into `AGAMA`, and 6.1.90's aṅga arm merges it across terms

These two land together: the moment 6.4.72 stops prefixing `A` onto `ANGA`, 6.1.90's `anga[0] == 'A'` test has nothing to see, and *Adat*/*Ekzata*/*EData* break until the arm reads `AGAMA`. One task, one reviewer gate.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/anga.rs:33-63` (6.4.72)
- Modify: `crates/panini-prakriya/src/tinanta/adesha.rs:329-357` (6.1.90 comment + aṅga arm)
- Test: both files' test modules

**Interfaces:**
- Consumes: Task 2 (ANGA's first character is the root's own).
- Produces: after 6.4.72, `terms[AGAMA].text == "A"`; after 6.1.90's aṅga arm, `AGAMA` is empty and the first non-empty term after it starts with the vṛddhi. Slice 3d's √ṛ (*aiyaḥ*) relies on "first non-empty term after `AGAMA`", which is the abhyāsa there.

- [ ] **Step 1: Write the failing tests**

In `anga.rs`'s `mod tests`:

```rust
    #[test]
    fn aat_augment_lands_in_the_agama_slot_for_vowel_initial_angas() {
        // 6.4.72 Aq ajAdInAm: āṭ, like aṭ, is its own term.
        let rule = rules().find(|r| r.id == "6.4.72").unwrap();
        let mut p = Prakriya {
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            terms: with_slots(vec![Term::new("ad"), Term::new(""), Term::new("t")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[AGAMA].text, "A");
        assert_eq!(p.terms[ANGA].text, "ad");
        assert_eq!(p.text(), "Aadt");
        assert_eq!(p.log.last().unwrap().sutra, "6.4.72");
    }

    #[test]
    fn aat_declines_by_the_angas_own_shape_with_no_log_lookup() {
        // A consonant-initial aṅga that 6.4.71 has just augmented must not
        // take āṭ too. With the augment in its own slot, ANGA's first
        // character is still the consonant, so the shape guard alone
        // decides — there is no log scan left to mutate.
        let r71 = rules().find(|r| r.id == "6.4.71").unwrap();
        let r72 = rules().find(|r| r.id == "6.4.72").unwrap();
        let mut p = Prakriya {
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            terms: with_slots(vec![Term::new("BU"), Term::new("a"), Term::new("t")]),
            ..Default::default()
        };
        assert!((r71.apply)(&mut p));
        assert!(!(r72.apply)(&mut p));
        assert_eq!(p.terms[AGAMA].text, "a");
        // laṭ, vowel-initial: no augment in any slot.
        let mut p = Prakriya {
            ctx: Context::new(
                Lakara::Lat,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            terms: with_slots(vec![Term::new("ad"), Term::new(""), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(r72.apply)(&mut p));
        assert_eq!(p.terms[AGAMA].text, "");
    }
```

In `adesha.rs`'s `mod tests` (imports: `use crate::tinanta::terms::{ABHYASA, AGAMA, with_slots};` — add the missing ones):

```rust
    #[test]
    fn awas_ca_anga_arm_merges_the_agama_into_the_first_non_empty_term() {
        // 6.1.90 AwaS ca, aṅga arm: āṭ + the following initial vowel yield
        // one vṛddhi, written into the term that held the vowel; the
        // augment slot is emptied. A+ad → Ad, A+eD → ED.
        let rule = rules().find(|r| r.id == "6.1.90").unwrap();
        for (root, expected) in [("ad", "Ad"), ("eD", "ED"), ("Ikz", "Ekz")] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new(root), Term::new(""), Term::new("t")]),
                ..Default::default()
            };
            p.terms[AGAMA].text = "A".into();
            assert!((rule.apply)(&mut p), "{root}");
            assert_eq!(p.terms[AGAMA].text, "", "{root}");
            assert_eq!(p.terms[ANGA].text, expected, "{root}");
            assert_eq!(p.text(), format!("{expected}t"), "{root}");
        }
        // "First non-empty term after AGAMA", not "ANGA": an abhyāsa in
        // front of the aṅga is what meets the āṭ (slice 3d's √ṛ, A+iy+ar →
        // Eyar). Nothing fills ABHYASA before juhotyādi lands; this pins
        // the arm's addressing so 3d inherits it rather than re-deriving it.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ar"), Term::new(""), Term::new("t")]),
            ..Default::default()
        };
        p.terms[AGAMA].text = "A".into();
        p.terms[ABHYASA].text = "iy".into();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[AGAMA].text, "");
        assert_eq!(p.terms[ABHYASA].text, "Ey");
        assert_eq!(p.terms[ANGA].text, "ar");
        assert_eq!(p.text(), "Eyart");
    }

    #[test]
    fn awas_ca_anga_arm_declines_without_the_agama() {
        let rule = rules().find(|r| r.id == "6.1.90").unwrap();
        // No augment: a vowel-initial aṅga in laṭ is left alone.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ad"), Term::new(""), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "ad");
        // An `A`-initial aṅga text with an empty AGAMA is not an āṭ: the
        // arm reads the slot, never the aṅga's own first character.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Aad"), Term::new(""), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Aad");
    }
```

- [ ] **Step 2: Run to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya 'aat_\|awas_ca_anga' 2>&1 | tail -30`
Expected: `aat_augment_lands…` FAILS (`AGAMA` is `""`, `ANGA` is `"Aad"`); `awas_ca_anga_arm_merges…` FAILS (arm declines: nothing in `ANGA` starts with `A`); `awas_ca_anga_arm_declines…` FAILS on the second case (the old arm fires on `Aad`). `aat_declines_by_the_angas_own_shape…` may already pass.

- [ ] **Step 3: Rewrite 6.4.72**

Replace the rule's comment and body (`anga.rs:33-63`) with:

```rust
    // 6.4.72 āḍ ajādīnām: vowel-initial aṅgas take the āṭ-āgama in laṅ
    // (apavāda to 6.4.71's aṭ). The A then merges by 6.1.90 āṭaś ca into
    // vṛddhi with the initial vowel of the term that follows it: A+eD → ED,
    // A+Ikz → Ekz, A+ad → Ad.
    //
    // Written into `AGAMA`, like aṭ. The guard is the aṅga's own shape and
    // nothing else: 6.4.71 leaves ANGA's text untouched, so `is_vowel(first)`
    // tells a genuinely vowel-initial root from one 6.4.71 just augmented.
    // This rule used to scan the log for a prior 6.4.71 because the aṭ was
    // once a text prefix (`aBU` reads as vowel-initial); an "is AGAMA empty"
    // clause in its place would be unkillable — 6.4.71 declines for every
    // vowel-initial aṅga, so the slot is always empty when this rule looks
    // — and is deliberately not written.
    Rule {
        id: "6.4.72",
        name: "Aq ajAdInAm",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let first = p.terms[ANGA].text.chars().next().unwrap();
            if !matches!(p.ctx.lakara, Lakara::Lan) || !is_vowel(first) {
                return false;
            }
            let before = p.snapshot();
            p.terms[AGAMA].text = "A".into();
            p.record("6.4.72", "Aq ajAdInAm", before);
            true
        },
    },
```

- [ ] **Step 4: Rewrite 6.1.90's aṅga arm**

In `adesha.rs`, add `AGAMA` to the top-level `use crate::tinanta::terms::{...}` import. In the 6.1.90 comment (`:329-337`) change the aṅga-arm bullet to:

```rust
    // - Aṅga arm (laṅ, the ātmanepada slice's Task 8): 6.4.72's āṭ in
    //   `AGAMA` + the initial vowel of the first non-empty term after it —
    //   the aṅga today, the abhyāsa once juhotyādi reduplicates a
    //   vowel-initial root. The vṛddhi is written into that term and the
    //   augment slot is emptied. A+eD → ED, A+Ikz → Ekz.
```

Replace the arm's body (`:343-357`) with:

```rust
            // Aṅga arm: āṭ in AGAMA + the first non-empty term after it.
            if p.terms[AGAMA].text == "A"
                && let Some(i) =
                    (AGAMA + 1..p.terms.len()).find(|&i| !p.terms[i].text.is_empty())
                && let Some(v0) = p.terms[i].text.chars().next()
                && is_vowel(v0)
                && let Some(v) = vrddhi_of(v0)
            {
                let before = p.snapshot();
                let rest: String = p.terms[i].text.chars().skip(1).collect();
                p.terms[i].text = format!("{v}{rest}");
                p.terms[AGAMA].text.clear();
                p.record("6.1.90", "AwaS ca", before);
                return true;
            }
```

Leave the ending arm below it untouched.

- [ ] **Step 5: Unit suite and lint**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -5` — Expected: all pass, including the whole-derivation laṅ cells of √ad, √īkṣ, √edh, √as and the tanādi `ArRot` collapse test (`controller.rs`'s convergent-fork case: A+fR and A+arR now merge in `ANGA` with `AGAMA` emptied on both branches, same texts).
Run: `mise run lint && mise run fmt-check` — Expected: clean.

- [ ] **Step 6: Full suite, byte-identical**

Run: `mise run test` (foreground). Expected: PASS; `git status --short crates/panini/tests` empty.

- [ ] **Step 7: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/anga.rs crates/panini-prakriya/src/tinanta/adesha.rs
git commit -m "refactor(engine): 6.4.72 writes AT into AGAMA; 6.1.90's anga arm merges it across terms

The augment no longer lives in the aṅga's text anywhere. 6.4.72's log scan
is deleted (an AGAMA-empty clause would be unkillable); 6.1.90 reads the
first non-empty term after the slot, which slice 3d's abhyāsa will be."
```

---

### Task 4: the comment and documentation sweep

No rule body changes. Every comment that describes the augment as a text prefix on the aṅga is now false; each gets the true sentence.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/terms.rs` (NOTE block)
- Modify: `crates/panini-prakriya/src/tinanta/anga.rs:203-209` (7.1.6 comment)
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs:44-45`, `:211-213`, `:381-385`, `:1547-1548`
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs:109-112`, `:1677-1679`
- Modify: `crates/panini-prakriya/src/tinanta/tin.rs:420`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs:1046-1048`
- Modify: `docs/ARCHITECTURE.md:80-96`, `AGENTS.md:1041`, `:1279-1280`

- [ ] **Step 1: `terms.rs` — the layout paragraph**

Insert, at the top of the NOTE block under the constants (before "`ENDING_PRE_SHAP` and `SHAP` are deliberately the same value"):

```rust
// NOTE: the two slots BEFORE the aṅga are permanent and usually empty.
// `AGAMA` holds laṅ's aṭ/āṭ (6.4.71 / 6.4.72) and nothing else; 6.1.90
// empties it again when the āṭ merges into the following vowel. `ABHYASA`
// holds the reduplicant 6.1.10 copies for juhotyādi and is empty for every
// other gaṇa. Both exist on every prakriya so the constants below are
// stable, on the same in-place-empty idiom 2.4.72 uses for śap. Two
// consequences:
//   - Any rule that reads "the term before the aṅga" or "the first term"
//     must skip empty terms (`AGAMA` is empty outside laṅ; `ABHYASA` is
//     empty outside juhotyādi). `word_chars` already does, by construction.
//   - The aṅga's text is the ROOT'S text. Nothing is ever prefixed onto it.
//     A guard that matches the aṅga with `ends_with` rather than `==` is
//     tolerating a hypothetical upasarga, not the augment — the augment
//     lived in the text until the juhotyādi prep, and several comments
//     downstream still say so historically.
```

- [ ] **Step 2: the stale engine comments**

Each site currently says 6.4.71 has prefixed the aṭ onto the aṅga's text. Replace the clause with the true one; keep every surrounding argument.

- `anga.rs:203-209` (7.1.6): "Reading the log for a prior rule is the idiom 6.4.72 already uses to test whether 6.4.71 augmented the aṅga." → "Reading the log for a prior rule is the idiom 6.4.72 used, until the augment moved into its own slot, to test whether 6.4.71 had fired; here it remains the condition itself."
- `guna.rs:44-45` (7.4.21): "`ends_with` rather than `==` because 6.4.71 has already prefixed the laṅ aṭ-augment onto the aṅga (aSI) by this point." → "`ends_with` rather than `==` is a tolerance for a prefixed aṅga; laṅ's aṭ lives in `AGAMA` since the juhotyādi prep, so ANGA reads `SI` in every lakāra."
- `guna.rs:211-213` (7.3.92): the same sentence about `atf` → "`ends_with` rather than `==` — the same allowance 7.4.21's guard makes; the laṅ aṭ that once made ANGA read `atf` now sits in `AGAMA`."
- `guna.rs:381-385` (6.4.110): the `akar`/`akur` sentence → "`ends_with` rather than `==`, as 7.4.21 and 7.3.92 do above — a tolerance for a prefixed aṅga; laṅ's aṭ no longer lives in the text, so ANGA reads `kar`/`kur` in laṅ too."
- `guna.rs:1547-1548` (test comment): "laN's aT-augmented aGga (6.4.71 prefixes onto ANGA's own text): akarutAm must become akurutAm, not decline on `!= "kar"`." → "A prefixed aṅga (`akar` — the shape laN's aT gave ANGA before the augment slot existed) must still pass the tail guard; this pins the `ends_with → ==` mutant."
- `tripadi.rs:109-112` (8.2.79 guard) and `:1677-1679` (its test): the same rewording — `akur` is a prefixed-aṅga tolerance, not laṅ's shape.
- `tin.rs:420`: "(the āṭ 3.4.92 / aṭ 6.4.71 precedent)" → "(the āṭ 3.4.92 precedent; laṅ's aṭ has its own `AGAMA` slot since the juhotyādi prep)".
- `derivation_tests.rs:1046-1048`: "laṅ: 6.4.71 has already prefixed the aṭ-augment, so the aṅga is `aSI` when 7.4.21 runs — the guard must match on the tail, not the whole string." → "laṅ: the aṭ sits in `AGAMA`, so 7.4.21 sees the bare `SI`; the cell still pins the augment's presence in the assembled word."

- [ ] **Step 3: `docs/ARCHITECTURE.md`**

Replace the adādi paragraph's last sentence ("The śap term is kept in place with empty text rather than removed, so the `ANGA`/`SHAP`/`ENDING` term indices stay stable for downstream rules.") with the same sentence plus a new paragraph after it:

> The layout itself is five fixed slots: `AGAMA` (0), `ABHYASA` (1), `ANGA`
> (2), `SHAP` (3, also the ending's index before 3.1.68 inserts śap) and
> `ENDING` (4). The two leading slots are permanent and empty on most
> derivations: `AGAMA` holds laṅ's aṭ or āṭ (6.4.71 / 6.4.72) — written
> there rather than prefixed onto the aṅga's text since the juhotyādi prep,
> so the aṅga's text is always the root's own, and 6.1.90 *āṭaś ca* merges
> the āṭ into the first non-empty term after the slot and empties it —
> and `ABHYASA` is reserved for the reduplicant of 6.1.10 *ślau*, which
> the juhotyādi slices fill. Because `Prakriya::text` concatenates every
> term, an empty slot is invisible in the assembled word and in every
> logged step.

In the rudhādi paragraph, "stretches the same three fixed slots the other way" → "stretches the fixed slots the other way", and "there is no fourth slot to hold one" → "there is no slot between `ANGA` and `SHAP` to hold one".

- [ ] **Step 4: `AGENTS.md`**

- `:1041`: "`[ANGA, SHAP, ENDING]` slots" → "`[AGAMA, ABHYASA, ANGA, SHAP, ENDING]` slots".
- `:1279-1280`: "`ENDING_PRE_SHAP` (index 1), rules after it as `ENDING` (index 2)" → "(index 3) … (index 4)"; add one sentence: "Two permanent, usually-empty slots — `AGAMA` (0) and `ABHYASA` (1) — precede the aṅga; see `tinanta/terms.rs`."

- [ ] **Step 5: Verify prose against code**

```bash
grep -rn "prefixed onto\|prefix on the aṅga\|text prefix\|index 1\b\|index 2\b" crates/panini-prakriya/src docs/ARCHITECTURE.md AGENTS.md README.md | grep -v "used to\|until the\|once a\|before the augment\|historically"
```

Every remaining hit must be a deliberate historical reference or about a different index (e.g. `vrddhi_of`'s arms). Fix the rest.

- [ ] **Step 6: Full verification and commit**

Run: `mise run fmt-check && mise run lint && mise run test` (foreground). Expected: all PASS, `git status --short crates/panini/tests` empty.

```bash
git add crates/panini-prakriya/src docs/ARCHITECTURE.md AGENTS.md
git commit -m "docs: the five-slot term layout — comments and ARCHITECTURE follow the augment into AGAMA"
```

---

### Task 5: the cross-implementation audit — negative control

**Files:**
- Modify: `tools/audit/README.md` ("Last recorded result")

**Interfaces:**
- Consumes: Tasks 1–4.
- Produces: a recorded zero-difference verdict at unchanged totals (77 / 3492 / 4399).

No harness edit is needed: the gaṇa set and the totals are unchanged. The audit can only confirm what the goldens already prove, and it runs anyway because the standing discipline is that every engine change is audited and its control is verified.

- [ ] **Step 1: Vidyut checkout at the vendored commit**

```bash
head -20 /workspace/data/dhatupatha.tsv | grep -i commit
git -C /tmp/vidyut-full log --oneline -1
```

Both must show `8da2f90…`. If `/tmp/vidyut-full` is missing, clone and checkout as `tools/audit/README.md` describes.

- [ ] **Step 2: Point the dev-deps at THIS checkout and copy the harness**

In `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml`, set the two dev-dependency paths (`panini`, `panini-data`) to the checkout being audited — the worktree's `crates/panini` and `crates/panini-data`; verify with `git -C <path> branch --show-current` printing `juhotyadi-prep`. Then:

```bash
cp <checkout>/tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
```

Copy; do not rewrite.

- [ ] **Step 3: Negative control FIRST**

```bash
cd /tmp/vidyut-full/vidyut-prakriya
PANINI_AUDIT_PERTURB=entry mise exec rust@1.98 -- cargo run --release --example panini_full_audit
```

Expected: **exit 1**, 36 differing √bhū cells. If it passes, stop — every later result is worthless.

- [ ] **Step 4: Real run**

```bash
mise exec rust@1.98 -- cargo run --release --example panini_full_audit 2>&1 | tee /tmp/prep-audit.txt
```

Expected: `AUDIT PASSED: 3492 cells, 4399 forms, zero differences.` with `roots: 77`. Any difference is a defect in Tasks 1–3 (the goldens would already have caught it; if they did not, the golden suite has a hole — report it, do not patch data).

- [ ] **Step 5: Record and commit**

Add a "Last recorded result" entry to `tools/audit/README.md` in the existing shape: date, slice (`juhotyādi prep — five-slot layout`), vidyut commit, 77 roots / 3492 cells / 4399 forms, zero differences, entry control verified failing first, and one sentence naming what the verdict now covers (the augment slot and 6.1.90's cross-term merge).

```bash
cd <checkout>
git add tools/audit/README.md
git commit -m "test(audit): the five-slot layout is byte-identical to vidyut

77 roots / 3492 cells / 4399 forms, zero differences at vidyut 8da2f90,
entry control verified failing first."
```

---

### Task 6: the mutation gate

**Files:**
- Modify: `AGENTS.md` (the cargo-mutants paragraph)

**Interfaces:**
- Consumes: everything above.
- Produces: the measured floor and campaign record.

- [ ] **Step 1: Measure the uncontended floor**

```bash
cd <checkout> && time mise run test 2>&1 | tee /tmp/prep-floor.txt
```

Alone, foreground, nothing else running. Record per-binary times and the total. Prior floor: **1958.411s at 3492 cells** (paradigm 866.37s, roundtrip 1087.37s, trace 3.61s). This slice adds no cell, so the floor should be flat; a move of more than a few percent is itself a finding (two extra terms per prakriya — measure, do not assume).

- [ ] **Step 2: Sanity-check the cap**

Multiply the measured total by the recorded `-j 4` contention range (1.02×–1.43×). If the projection exceeds **4800s**, stop and report — cap changes are recorded in `AGENTS.md` and `mise.toml` together, not made silently. (1958 × 1.43 ≈ 2800s: margin expected.)

- [ ] **Step 3: Run the campaign**

```bash
env | grep CARGO_MUTANTS   # must print nothing
cd <checkout> && nohup mise run mutants > /tmp/prep-mutants.txt 2>&1 &
```

Hours. Launched detached (the 8a campaign ran ~16h this way, surviving the ~60-minute background-shell limit); wait on `/tmp/prep-mutants.txt` and `mutants.out/` with a monitor loop, never ending the turn as "done" while it runs. This slice adds three new code sites — `with_slots`, the `AGAMA` writes in 6.4.71/6.4.72, and 6.1.90's `find` over the terms — so expect a handful of new mutants there: `AGAMA + 1` → `AGAMA - 1`/`* 1`, `is_empty` → `!is_empty`, `"a"` → `""`, the `extend` deleted. Each must be caught by Tasks 1–3's unit tests or the laṅ goldens.

- [ ] **Step 4: Check BOTH `missed.txt` and `timeout.txt`**

Expected: `missed.txt` empty; `timeout.txt` exactly the one known-permanent ṇatva-scan entry (`j /= 1` in `tripadi.rs` — identify by diff shape, not line number). Any other timeout: re-run alone at the same cap before concluding anything. Any survivor: resolve, don't accept — a survivor in `with_slots` or the augment writes means a unit test above is weaker than it looks.

- [ ] **Step 5: Extract the duration distribution**

```bash
cd <checkout> && python3 - <<'PY'
import json
d = json.load(open('mutants.out/outcomes.json'))
xs = sorted(
    sum(p['duration'] for p in o.get('phase_results', []) if p['phase'] == 'Test')
    for o in d['outcomes'] if o.get('summary') == 'CaughtMutant'
)
n = len(xs)
pick = lambda q: xs[min(n - 1, int(q * n))]
print(f"caught={n} median={pick(.5):.1f} p90={pick(.9):.1f} p99={pick(.99):.1f} max={xs[-1]:.1f}")
print("over 600s:", sum(x > 600 for x in xs), " over 1200s:", sum(x > 1200 for x in xs))
PY
```

- [ ] **Step 6: Record in `AGENTS.md`**

Append a paragraph in the established series shape: "**The juhotyādi prep (five-slot layout, no new cell) re-measured both at 3492 cells.**" — per-binary floor + total, flat-or-not against 8b's 1958.411s, campaign tallies (mutants / caught / missed / unviable / timeouts), the distribution from Step 5, both margins labelled measured/projected, and the ruling on the cap. Keep every earlier paragraph.

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md
git commit -m "test: mutation gate at 3492 cells on the five-slot layout, floor re-measured"
```

---

### Task 7: PR and finish

- [ ] **Step 1: Full verification**

```bash
cd <checkout> && mise run fmt-check && mise run lint && mise run test && mise run audit
```

All PASS, foreground; `git status --short` empty.

- [ ] **Step 2: Push and PR**

```bash
git push -u origin juhotyadi-prep
gh pr create --title "refactor(engine): five-slot term layout — AGAMA and ABHYASA lead every prakriya" --body "$(cat <<'MD'
Juhotyādi prep, per docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md.

- `terms.rs`: `AGAMA` (0) and `ABHYASA` (1) precede `ANGA` (2); `SHAP`/`ENDING_PRE_SHAP` 3, `ENDING` 4; `with_slots` builds every term vector.
- 6.4.71 / 6.4.72 write laṅ's augment into `AGAMA`; 6.1.90's aṅga arm merges it into the first non-empty term after the slot. The aṅga's text is always the root's own.
- No new root, rule, or golden: 3492 cells and every trace byte-identical.
- Audit: zero differences at vidyut 8da2f90, entry control verified failing first.
- Mutation gate: <tallies from Task 6>; floor <measured>s.
MD
)"
```

- [ ] **Step 3: Finish the branch**

Per the standing instruction (`branch-finish-auto-merge`): wait for CI green, merge with a merge commit (the repo's convention), `git fetch`, verify the commits are ancestors of `origin/main`, and only then delete the branch and the worktree. Slice 3a's plan is written against the merged `main`.

---

## Deferred, and why

- **Slice 3a** (`Gana::Juhotyadi`, 2.4.75, 6.1.10, the `abhyasa.rs` stage, the eight new rules): its own plan, written once this PR merges so its line numbers and the constants it reads are stable. The spec's "Slice 3a" section is its contract.
- **A `NIC` slot for curādi:** the same idiom will serve ṇic between `ANGA` and `SHAP`; not added now because nothing reads it (YAGNI, and an unread slot is an unkillable-mutant magnet).
- **Retiring the `ends_with` tolerances:** the guards are correct and their tests kill the `==` mutant; narrowing them is grammar review for a slice that has a reason to.
