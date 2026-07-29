# Svādi gaṇa (gaṇa 5) — prep + slice 5a Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add svādi (gaṇa 5) to the tiṅanta engine — six roots, 1296 → 1512 golden forms — by teaching four existing rules that the aṅga is affix-relative and adding four new sūtras.

**Architecture:** Two behaviour-preserving prep commits (split `anga.rs` into `anga.rs` + `guna.rs`; give `Dhatu` a gaṇa-qualified `id`), then slice 5a: the new vikaraṇa śnu (3.1.73), a second application of 7.3.84 that guṇates the vikaraṇa's final `u`, the śnu alternations 6.4.87 / 6.4.77, the hi-luk 6.4.106, and vikaraṇa-aware arms for 6.1.78, 6.1.90 and 6.4.101.

**Tech Stack:** Rust (edition 2021), Cargo workspace, `mise` (toolchain + task runner), `cargo-mutants`, `cargo-deny` / `cargo-audit`.

**Spec:** `docs/superpowers/specs/2026-07-29-svadi-gana-design.md`. Read it before Task 1. Where this plan and the spec disagree, the spec is wrong and should be corrected — but stop and say so rather than silently diverging.

**Scope note:** This plan covers **prep 1, prep 2 and slice 5a only**. Slice 5b (optional-rule support + 6.4.107) is deliberately excluded: the spec scopes it but does not design the fork machinery — what a "derivation" becomes when one prakriyā splits into a set, and what that does to `derive`'s signature, the `panini` facade, the CLI's `--json` shape and the golden table's schema. 5b gets its own brainstorm → spec → plan cycle after 5a lands. Do not improvise it here.

## Global Constraints

- Toolchain is pinned via `mise` (rust 1.97.1). Never install Rust globally. Build/test with `mise run build | test | lint | fmt | fmt-check | mutants | audit`.
- To scope tests to one crate during iteration use `mise exec -- cargo test -p panini-prakriya`. `mise run test -- -p X` does **not** scope.
- To run mutation testing, invoke the `cargo-mutants` binary **directly**, not through the `mise` shim — the shim fails in background shells.
- SLP1 is the only internal representation. Transliteration lives only in `panini-lipi`.
- `#![forbid(unsafe_code)]` in every non-fuzz crate.
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, never as a branch inside `derive`. `derive`'s only gaṇa-conditioned logic is aṅga tagging.
- Which stage file a rule belongs to is decided by its **pipeline position relative to 3.1.68**, not by its sūtra family. Rules ordered before 3.1.68 address the ending as `ENDING_PRE_SHAP` (index 1); rules after it use `ENDING` (index 2) and may use `SHAP` (index 1). See `crates/panini-prakriya/src/tinanta/terms.rs`.
- `terms[SHAP].text` may be **empty** (adādi, 2.4.72). Any rule reading "the segment after the aṅga" must handle that — `ends_with` / `is_empty` / `chars().next()` as an `Option` are safe; `chars().next().unwrap()` panics.
- Per-rule guard tests go beside the rule in its stage file. Tests asserting a surface form or trace go in `tinanta/derivation_tests.rs` (unit) or `crates/panini/tests/` (integration).
- Sūtra ids and names in traces must match `vidyut-prakriya`'s `data/sutrapatha.tsv`. ashtadhyayi.com is a JS SPA and cannot be fetched programmatically.
- **Never edit a golden to match engine output.** A golden that does not match is an engine bug; escalate rather than mask. This has caught real bugs in this repo twice.

## File Structure

| file | change | responsibility after the change |
|---|---|---|
| `crates/panini-prakriya/src/tinanta/anga.rs` | shrink | augments and ending reshaping: 6.4.71, 6.4.72, 7.3.100, 7.1.5, 7.1.6, 7.1.3, 7.2.79, 7.2.80, 7.2.81 |
| `crates/panini-prakriya/src/tinanta/guna.rs` | **create** | vowel gradation and vikaraṇa reshaping: 7.4.21, 7.3.84 ×2, 6.4.87, 6.4.77, 7.3.86, 6.1.78, 7.3.101, 6.4.112, 6.4.113 |
| `crates/panini-prakriya/src/tinanta/mod.rs` | modify | `TINANTA_RULES` gains a seventh stage |
| `crates/panini-prakriya/src/tinanta/vikarana.rs` | modify | + 3.1.73 |
| `crates/panini-prakriya/src/tinanta/adesha.rs` | modify | + 6.4.106; 6.1.90 and 6.4.101 made vikaraṇa-aware |
| `crates/panini-prakriya/src/tinanta/terms.rs` | modify | + `sound_before_ending` helper |
| `crates/panini-prakriya/src/tinanta/sound.rs` | modify | + `shnu_asamyogapurva` helper |
| `crates/panini-prakriya/src/term.rs` | modify | + `Tag::Svadi` |
| `crates/panini-data/src/lib.rs` | modify | + `Gana::Svadi`, + `Dhatu::id`, + six root rows |
| `data/dhatupatha.tsv` | modify | + six reference rows (not compiled; kept in sync by hand) |
| `crates/panini/tests/paradigm.rs` | modify | + 24 golden blocks (216 forms) |
| `crates/panini/tests/trace.rs` | modify | + 12 ordered-trace pins |

---

## Task 1: Prep 1 — split `anga.rs` into `anga.rs` + `guna.rs`

Pure move. No rule is added, removed, renamed, or reordered.

**Files:**
- Create: `crates/panini-prakriya/src/tinanta/guna.rs`
- Modify: `crates/panini-prakriya/src/tinanta/anga.rs`
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs`

**Interfaces:**
- Consumes: nothing.
- Produces: `pub(crate) static GUNA: &[Rule]`, the seventh entry of `TINANTA_RULES`, positioned immediately after `anga::ANGA_RULES`.

- [ ] **Step 1: Record the baseline you must not change**

```bash
mise exec -- cargo test -p panini 2>&1 | tail -20
```
Expected: all tests pass. Note the counts — they must be identical at Step 7.

- [ ] **Step 2: Create `guna.rs` and move the seven rules verbatim**

Cut the `Rule { … }` literals for **7.4.21, 7.3.84, 7.3.86, 6.1.78, 7.3.101, 6.4.112, 6.4.113** out of `ANGA_RULES` — they are contiguous, running from the `7.4.21` entry to the end of the array — and paste them, byte for byte, into a new file:

```rust
//! Vowel gradation and vikaraṇa reshaping: 7.4.21 … 6.4.113.
//!
//! Split out of `anga.rs` (which had reached 1110 lines) ahead of svādi.
//! The cut falls after 7.2.81: `anga.rs` keeps the augments and the rules
//! that reshape the *ending*, this file takes the rules that reshape a
//! *vowel* — the aṅga's, or the vikaraṇa's.
//!
//! Ordered AFTER 3.1.68 — ending at `ENDING`, śap at `SHAP`, and
//! `terms[SHAP].text` may be empty (2.4.72). See `super::terms`.
//!
//! 6.1.78 *eco'yavāyāvaḥ* sits in this stage rather than with the other
//! 6.1.x rules in `super::adesha` because that is where the pipeline order
//! puts it, between 7.3.86 and 7.3.101. Order outranks sūtra family: the
//! flattened sequence is the grammar.

use crate::rule::{Rule, RuleKind};
use crate::term::Tag;
use crate::tinanta::sound::{guna_of, is_vowel};
use crate::tinanta::terms::{ANGA, ENDING, SHAP, following_sarvadhatuka};

pub(crate) static GUNA: &[Rule] = &[
    // … the seven moved Rule literals, in their existing order …
];

#[cfg(test)]
mod tests {
    // … the moved test functions …
}
```

Move the `#[cfg(test)] mod tests` functions that belong to the moved rules too — the `7.3.86` guard-edge pins and the `7.3.84` 1.1.5 guard pins are the two clearly-labelled blocks; take any test that names 7.4.21, 7.3.84, 7.3.86, 6.1.78, 7.3.101, 6.4.112 or 6.4.113.

Fix the `use` lists in both files afterwards so each imports exactly what it still needs. `cargo` will tell you: unused imports are warnings, and `mise run lint` treats them as errors.

- [ ] **Step 3: Register the new stage in `mod.rs`**

Add `mod guna;` to the alphabetical `mod` block, and insert the stage into `TINANTA_RULES` **after** `anga::ANGA_RULES`:

```rust
pub static TINANTA_RULES: &[&[Rule]] = &[
    samjna::SAMJNA,
    tin::TIN,
    vikarana::VIKARANA,
    anga::ANGA_RULES,
    guna::GUNA,
    adesha::ADESHA,
    tripadi::TRIPADI,
];
```

Update the module doc comment: it says "six ordered rule-stage modules" and lists them. Make it seven and add `guna` between `anga` and `adesha`.

- [ ] **Step 4: Run the rule-order test**

```bash
mise exec -- cargo test -p panini-prakriya tinanta_rule_order_is_pinned
```
Expected: **PASS, with no edit to the test.** The flattened sequence is unchanged, which is the whole proof that the move was safe. If it fails, a rule moved position — fix the move, do not touch the expected array.

- [ ] **Step 5: Run the full suite**

```bash
mise run test
```
Expected: PASS, with the same test counts as Step 1.

- [ ] **Step 6: Point the docs at the new file**

In `AGENTS.md`, the "Rules of the codebase" bullet says `TINANTA_RULES` "is a list of six stage arrays". Change six to seven. In `docs/ARCHITECTURE.md`, find the stage-file list and add `guna.rs` with a one-line description matching the module doc comment.

- [ ] **Step 7: Verify nothing moved and commit**

```bash
mise run fmt-check && mise run lint && mise run test
git add -A
git commit -m "refactor(prakriya): extract tinanta/guna.rs, the vowel-gradation stage

anga.rs had reached 1110 lines. The cut falls after 7.2.81: anga.rs keeps
the augments and ending-reshaping rules, guna.rs takes 7.4.21, 7.3.84,
7.3.86, 6.1.78, 7.3.101, 6.4.112 and 6.4.113 — the rules that reshape a
vowel. Pure move: the flattened rule order is unchanged, so
tinanta_rule_order_is_pinned passes untouched."
```

---

## Task 2: Prep 2 — gaṇa-qualified `Dhatu::id`

`Dhatu.code` is both the root's SLP1 text and its lookup key. Svādi's √aś and kryādi's √aś share a text, so a second `aS` row would make `find(|d| d.code == "aS")` return the kryādi row and leave the svādi root permanently unreachable — silently. This task separates identity from text **before** that row exists.

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: nothing.
- Produces: `Dhatu { pub id: &'static str, pub code: &'static str, pub gana: Gana, pub pada: Pada, pub artha: &'static str }`. `id` is the lookup key everywhere; `code` remains the SLP1 text handed to `Term::new` in `derive`.

- [ ] **Step 1: Write the failing test**

In `crates/panini-data/src/lib.rs`'s `mod tests`:

```rust
#[test]
fn id_is_the_lookup_key_and_is_unique() {
    let ids: Vec<&str> = dhatus().iter().map(|d| d.id).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "dhatu ids must be unique");
    // Until svādi lands every id equals its code; the field exists so that
    // stops being required.
    for d in dhatus() {
        assert!(!d.id.is_empty());
    }
}
```

- [ ] **Step 2: Run it to verify it fails**

```bash
mise exec -- cargo test -p panini-data id_is_the_lookup_key
```
Expected: FAIL to compile — `no field 'id' on type 'Dhatu'`.

- [ ] **Step 3: Add the field**

```rust
#[derive(Debug, Clone, Copy)]
pub struct Dhatu {
    /// Unique lookup key. Equal to `code` except where two roots in
    /// different gaṇas share an SLP1 form, in which case it is
    /// gaṇa-qualified (`aS.5` vs `aS.9`). Never hand this to `Term::new`.
    pub id: &'static str,
    /// The root's SLP1 text, as it enters the derivation.
    pub code: &'static str,
    pub gana: Gana,
    /// Which pada this root takes. Ubhayapadi roots are out of scope; each
    /// curated root has exactly one pada.
    pub pada: Pada,
    pub artha: &'static str,
}
```

Then add `id: "<same as code>",` as the first field of all 36 existing `Dhatu` literals.

- [ ] **Step 4: Migrate every by-code lookup to by-id**

These are the call sites. Change `d.code ==` to `d.id ==` in each:

- `crates/panini-data/src/lib.rs` — lines ~314–361 and ~406, ~417 (the `dhatus_are_present`-style tests)
- `crates/panini-prakriya/src/tinanta/derivation_tests.rs` — lines ~15, 24, 29, 34, 39, 44, 244, 361, 386

Leave `derivation_tests.rs:434` alone — that one reports `d.code` in an assertion message, which is still the right thing to show.

Do **not** change `crates/panini/src/lib.rs`'s `dhatu: c.dhatu.code.to_string()`. `Analysis.dhatu` is a user-facing root spelling, not a key.

- [ ] **Step 5: Run the tests**

```bash
mise exec -- cargo test -p panini-data && mise run test
```
Expected: PASS, same counts as Task 1 Step 5.

- [ ] **Step 6: Commit**

```bash
mise run fmt-check && mise run lint
git add -A
git commit -m "refactor(data): give Dhatu an id distinct from its SLP1 text

code doubled as the lookup key, so two roots sharing an SLP1 form would
collapse: find(|d| d.code == \"aS\") returns whichever row comes first and
the other becomes unreachable with no error. svādi's √aś (05.0020
vyāptau) collides with kryādi's (09.0059 bhojane), so the field is split
before that row exists. Every existing id equals its code; no behaviour
changes."
```

---

## Task 3: `Gana::Svadi`, `Tag::Svadi`, and rule 3.1.73

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini-prakriya/src/term.rs`
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs`
- Modify: `crates/panini-prakriya/src/tinanta/vikarana.rs`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: nothing.
- Produces: `Gana::Svadi`, `Tag::Svadi`, and a rule with `id: "3.1.73", name: "svAdiByaH SnuH"` ordered in `VIKARANA` between 3.1.69 and 3.1.77. After it fires, `terms[SHAP].text == "nu"` and `terms[SHAP]` carries `Tag::Vikarana | Tag::Sarvadhatuka`; the second 1.2.4 (already present, last in `VIKARANA`) then adds `Tag::Ngit`.

- [ ] **Step 1: Write the failing guard tests**

In `vikarana.rs`'s `mod tests`:

```rust
#[test]
fn svadibhyah_shnu_inserts_nu_for_svadi_only() {
    let mut p = Prakriya {
        terms: vec![Term::new("Ap"), Term::new("ti")],
        ..Default::default()
    };
    p.terms[ANGA].add(Tag::Svadi);
    let rule = rules().find(|r| r.id == "3.1.73").unwrap();
    assert!((rule.apply)(&mut p));
    assert_eq!(p.terms[SHAP].text, "nu");
    assert!(p.terms[SHAP].has(Tag::Vikarana));
    assert!(p.terms[SHAP].has(Tag::Sarvadhatuka));
    assert_eq!(p.terms[ENDING].text, "ti");
}

#[test]
fn svadibhyah_shnu_declines_without_the_gana_tag() {
    // bhvādi: no Tag::Svadi, so the apavāda must not fire and 3.1.68 keeps
    // its utsarga job.
    let mut p = Prakriya {
        terms: vec![Term::new("BU"), Term::new("ti")],
        ..Default::default()
    };
    let rule = rules().find(|r| r.id == "3.1.73").unwrap();
    assert!(!(rule.apply)(&mut p));
}

#[test]
fn shnu_is_tagged_ngit_by_the_second_1_2_4_without_change() {
    // śnu carries no p-anubandha, so the existing second 1.2.4 must tag it
    // ṅit with no edit. This is what blocks the FIRST 7.3.84 on ik-final
    // roots (hinoti, not *henoti).
    let mut p = Prakriya {
        terms: vec![Term::new("hi"), Term::new("ti")],
        ..Default::default()
    };
    p.terms[ANGA].add(Tag::Svadi);
    let shnu = rules().find(|r| r.id == "3.1.73").unwrap();
    assert!((shnu.apply)(&mut p));
    assert_eq!(rules().filter(|r| r.id == "1.2.4").count(), 2);
    let second = rules().filter(|r| r.id == "1.2.4").nth(1).unwrap();
    assert!((second.apply)(&mut p));
    assert!(p.terms[SHAP].has(Tag::Ngit));
}
```

- [ ] **Step 2: Run to verify they fail**

```bash
mise exec -- cargo test -p panini-prakriya svadibhyah_shnu
```
Expected: FAIL to compile — no `Tag::Svadi`, and `unwrap()` on a missing rule.

- [ ] **Step 3: Add the enum variants**

In `crates/panini-data/src/lib.rs`:

```rust
pub enum Gana {
    Bhvadi,
    Divadi,
    Tudadi,
    Adadi,
    Kryadi,
    Svadi,
}
```

In `crates/panini-prakriya/src/term.rs`, after the `Kryadi` variant:

```rust
    /// The dhātu belongs to svādi (gaṇa 5), whose vikaraṇa is śnu. Read by
    /// 3.1.73 alone. Mirrors Divadi/Tudadi/Adadi/Kryadi.
    Svadi,
```

In `crates/panini-prakriya/src/tinanta/mod.rs`, extend `derive`'s match:

```rust
        match dhatu.gana {
            Gana::Divadi => t.add(Tag::Divadi),
            Gana::Tudadi => t.add(Tag::Tudadi),
            Gana::Adadi => t.add(Tag::Adadi),
            Gana::Kryadi => t.add(Tag::Kryadi),
            Gana::Svadi => t.add(Tag::Svadi),
            Gana::Bhvadi => {}
        }
```

- [ ] **Step 4: Add rule 3.1.73**

In `vikarana.rs`, insert between the 3.1.69 and 3.1.77 entries (sūtra order among the mutually-exclusive apavādas; all four precede the utsarga 3.1.68):

```rust
    // 3.1.73 svādibhyaḥ śnuḥ: svādi (gaṇa 5) takes śnu, not śap. Apavāda to
    // 3.1.68, ordered before it, exactly as 3.1.69, 3.1.77 and 3.1.81 are.
    //
    // śnu is apit, so the second 1.2.4 below tags it ṅit with no change of
    // its own — which is what blocks the FIRST 7.3.84 on the ik-final roots
    // (hi, ri): hinoti, not *henoti. The guṇa svādi IS famous for lands on
    // śnu's own `u` and belongs to 7.3.84's SECOND application (`guna.rs`),
    // because by 1.4.13 the aṅga for the tiṅ ending is root + vikaraṇa.
    //
    // Unlike śnā, śnu's text never changes shape here — 6.4.87 and 6.4.77
    // rewrite its `u` later, in `guna.rs`, and only before a vowel.
    Rule {
        id: "3.1.73",
        name: "svAdiByaH SnuH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Svadi) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("Snu");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.73", "svAdiByaH SnuH", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S → nu
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
```

- [ ] **Step 5: Update the pinned rule order**

In `derivation_tests.rs`'s `tinanta_rule_order_is_pinned`, insert `"3.1.73"` immediately after `"3.1.69"`. The array goes from 62 to 63 entries.

- [ ] **Step 6: Run the tests**

```bash
mise exec -- cargo test -p panini-prakriya && mise run test
```
Expected: PASS. All 1296 goldens unchanged — 3.1.73 is gaṇa-gated and no root carries `Tag::Svadi` yet.

- [ ] **Step 7: Commit**

```bash
mise run fmt-check && mise run lint
git add -A
git commit -m "feat(prakriya): 3.1.73 svAdiByaH SnuH + the svādi gaṇa tag

The vikaraṇa itself, with no root yet carrying the tag. śnu is apit, so
the existing second 1.2.4 tags it ṅit unchanged."
```

---

## Task 4: 7.3.84's second application

The slice's central rule. By 1.4.13 *yasmāt pratyayavidhis tadādi pratyaye'ṅgam* the aṅga is defined relative to the affix, so 7.3.84 has two occasions: once with respect to the vikaraṇa (the existing entry, which guṇates the root), once with respect to the tiṅ ending (this entry, which guṇates the vikaraṇa).

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: `Tag::Ngit`, `following_sarvadhatuka` (unused here — this rule reads `ENDING` directly), `guna_of` from `super::sound`.
- Produces: a second `Rule` with `id: "7.3.84"`, ordered immediately after 7.3.86. Tests must locate it with `rules().filter(|r| r.id == "7.3.84").nth(1)`, mirroring the existing double-1.2.4 idiom.

- [ ] **Step 1: Write the failing tests**

In `guna.rs`'s `mod tests`:

```rust
// --- 7.3.84 second application: the vikaraṇa-aṅga guṇa ------------------

fn second_7_3_84() -> &'static Rule {
    assert_eq!(
        rules().filter(|r| r.id == "7.3.84").count(),
        2,
        "expected exactly two 7.3.84 entries; nth(1) locator assumes this"
    );
    rules().filter(|r| r.id == "7.3.84").nth(1).unwrap()
}

#[test]
fn second_7_3_84_gunates_shnu_before_a_pit_ending() {
    // Ap + nu + ti → Ap + no + ti. `ti` is pit, so 1.1.5 does not block.
    let mut p = Prakriya {
        terms: vec![Term::new("Ap"), Term::new("nu"), Term::new("ti")],
        ..Default::default()
    };
    assert!((second_7_3_84().apply)(&mut p));
    assert_eq!(p.terms[SHAP].text, "no");
    assert_eq!(p.terms[ANGA].text, "Ap", "the root must not be touched");
}

#[test]
fn second_7_3_84_blocked_by_a_ngit_ending() {
    // Ap + nu + taH → ApnutaH. `tas` is apit, so the first 1.2.4 tagged it
    // ṅit and 1.1.5 blocks guṇa. This is the gaṇa's signature contrast.
    let mut p = Prakriya {
        terms: vec![Term::new("Ap"), Term::new("nu"), Term::new("taH")],
        ..Default::default()
    };
    p.terms[ENDING].add(Tag::Ngit);
    assert!(!(second_7_3_84().apply)(&mut p));
    assert_eq!(p.terms[SHAP].text, "nu");
}

#[test]
fn second_7_3_84_declines_on_a_thematic_vikarana() {
    // bhvādi: SHAP is śap's `a`, not an ik. The no-delta guard, half one.
    let mut p = Prakriya {
        terms: vec![Term::new("Bo"), Term::new("a"), Term::new("ti")],
        ..Default::default()
    };
    assert!(!(second_7_3_84().apply)(&mut p));
    assert_eq!(p.terms[SHAP].text, "a");
}

#[test]
fn second_7_3_84_declines_on_kryadi_shni() {
    // kryādi: `nI` IS ik-final, so only the 1.1.5 guard keeps this rule off
    // it — and 6.4.113 only ever produces `nI` before a ṅit ending, so the
    // guard is always satisfied. The no-delta guard, half two. If this ever
    // fires, kryādi surfaces *kliSne and 1296 goldens move.
    let mut p = Prakriya {
        terms: vec![Term::new("kliS"), Term::new("nI"), Term::new("taH")],
        ..Default::default()
    };
    p.terms[ENDING].add(Tag::Ngit);
    assert!(!(second_7_3_84().apply)(&mut p));
    assert_eq!(p.terms[SHAP].text, "nI");
}

#[test]
fn second_7_3_84_declines_on_an_empty_shap() {
    // adādi: śap is luk'd to an empty string. Must not panic.
    let mut p = Prakriya {
        terms: vec![Term::new("ad"), Term::new(""), Term::new("ti")],
        ..Default::default()
    };
    assert!(!(second_7_3_84().apply)(&mut p));
}
```

- [ ] **Step 2: Run to verify they fail**

```bash
mise exec -- cargo test -p panini-prakriya second_7_3_84
```
Expected: FAIL on the count assertion — "expected exactly two 7.3.84 entries", found 1.

- [ ] **Step 3: Add the rule**

In `guna.rs`, immediately **after** the 7.3.86 entry and **before** 6.1.78:

```rust
    // 7.3.84 sārvadhātukārdhadhātukayoḥ — SECOND APPLICATION, on the
    // vikaraṇa. This is not a duplicate: by 1.4.13 yasmāt pratyayavidhis
    // tadādi pratyaye'ṅgam the aṅga is defined relative to the affix, so
    // the sūtra has two occasions in a single derivation. With respect to
    // the vikaraṇa the aṅga is the root — that is the entry above. With
    // respect to the tiṅ ending the aṅga is root + vikaraṇa, and its final
    // ik belongs to the vikaraṇa. Ap + nu + ti → Ap + no + ti.
    //
    // The pipeline already carries two applications of 1.2.4 for exactly
    // this reason (ending, then vikaraṇa); this is the same shape. Both
    // entries appear in `tinanta_rule_order_is_pinned`, and tests locate
    // this one with `.filter(id == "7.3.84").nth(1)`. Do not "deduplicate".
    //
    // Reads terms[ENDING] directly rather than `following_sarvadhatuka`:
    // that helper answers "what follows the aṅga", which for this
    // application is the vikaraṇa being operated on, not the trigger.
    //
    // NO DELTA on any pre-existing form, by guard rather than by argument.
    // The complete inventory of SHAP texts reaching this point is `a`
    // (śap/śa), `ya` (śyan), `` (adādi luk), `Ana` (śānac), `nA`/`n`
    // (śnā, 6.4.112) and `nI` (śnā, 6.4.113). Only `nI` is ik-final, and
    // 6.4.113 produces it ONLY before a ṅit ending — so the 1.1.5 test
    // below declines there. Two tests pin both halves.
    //
    // Ordered BEFORE 6.1.78: the loṭ uttama endings are vowel-initial and
    // pit, so guṇa leaves `no`, which 6.1.78 must then make `nav`. Ordered
    // after it, ApnavAni surfaces as *ApnoAni. Ordered BEFORE 6.4.87/6.4.77
    // for the same cells: those fire on a vowel-initial ending too, and
    // would take `nu` to `nuv` first, giving *ApnuvAni.
    Rule {
        id: "7.3.84",
        name: "sArvaDAtukArDaDAtukayoH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms.len() <= ENDING {
                return false;
            }
            // 1.1.5 kṅiti ca, as in the first application. Same ṅit-only
            // narrowness: no kit tag exists in this engine yet.
            if p.terms[ENDING].has(Tag::Ngit) {
                return false;
            }
            let Some(last) = p.terms[SHAP].text.chars().last() else {
                return false;
            };
            let Some(g) = guna_of(last) else {
                return false;
            };
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[SHAP].text.chars().collect();
            s.pop();
            p.terms[SHAP].text = s.into_iter().collect::<String>() + g;
            p.record("7.3.84", "sArvaDAtukArDaDAtukayoH", before);
            true
        },
    },
```

- [ ] **Step 4: Update the pinned rule order**

Insert a second `"7.3.84"` immediately after `"7.3.86"`. Add a comment above the array noting that 7.3.84 appears twice by design, the way the existing double 1.2.4 is explained. The array goes from 63 to 64 entries.

- [ ] **Step 5: Run the tests**

```bash
mise exec -- cargo test -p panini-prakriya && mise run test
```
Expected: PASS, all 1296 goldens and traces unchanged.

- [ ] **Step 6: Commit**

```bash
mise run fmt-check && mise run lint
git add -A
git commit -m "feat(prakriya): 7.3.84's second application, on the vikaraṇa

By 1.4.13 the aṅga is affix-relative, so 7.3.84 has two occasions: the
existing entry guṇates the root with respect to the vikaraṇa, this one
guṇates the vikaraṇa with respect to the ending (Ap + nu + ti → Apnoti).
Mirrors the pipeline's existing double 1.2.4.

Inert on every existing path by guard: only śnā's nI is ik-final, and
6.4.113 produces it solely before a ṅit ending. Both halves pinned."
```

---

## Task 5: 6.1.78's vikaraṇa arm

6.1.78 *eco'yavāyāvaḥ* opens by reading `terms[ANGA]`'s final character and returning unless it is `e`/`o`. For √āp that character is `p`, so the rule returns before either existing arm runs — and the `o` that Task 4 just put on the vikaraṇa never becomes `av`. Without this, every loṭ uttama cell and the laṅ uttama-eka cells are wrong: \*`ApnoAni` for `ApnavAni`.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs`

**Interfaces:**
- Consumes: Task 4's guṇated SHAP.
- Produces: no new rule id. 6.1.78 gains a third arm; the order array is unchanged.

- [ ] **Step 1: Write the failing tests**

```rust
#[test]
fn eco_yavayavah_converts_the_vikaranas_o_before_a_vowel_ending() {
    // Ap + no + Ani → Ap + nav + Ani → ApnavAni.
    let mut p = Prakriya {
        terms: vec![Term::new("Ap"), Term::new("no"), Term::new("Ani")],
        ..Default::default()
    };
    let rule = rules().find(|r| r.id == "6.1.78").unwrap();
    assert!((rule.apply)(&mut p));
    assert_eq!(p.terms[SHAP].text, "nav");
    assert_eq!(p.terms[ANGA].text, "Ap");
}

#[test]
fn eco_yavayavah_vikarana_arm_declines_before_a_consonant_ending() {
    // Apnoti: `ti` is consonant-initial, so nothing converts.
    let mut p = Prakriya {
        terms: vec![Term::new("Ap"), Term::new("no"), Term::new("ti")],
        ..Default::default()
    };
    let rule = rules().find(|r| r.id == "6.1.78").unwrap();
    assert!(!(rule.apply)(&mut p));
    assert_eq!(p.terms[SHAP].text, "no");
}

#[test]
fn eco_yavayavah_thematic_arm_still_wins_for_bhvadi() {
    // Bo + a + ti → Bav + a + ti. The root's `o`, not the vikaraṇa's.
    let mut p = Prakriya {
        terms: vec![Term::new("Bo"), Term::new("a"), Term::new("ti")],
        ..Default::default()
    };
    let rule = rules().find(|r| r.id == "6.1.78").unwrap();
    assert!((rule.apply)(&mut p));
    assert_eq!(p.terms[ANGA].text, "Bav");
    assert_eq!(p.terms[SHAP].text, "a");
}

#[test]
fn eco_yavayavah_athematic_arm_still_wins_for_adadi() {
    // Se + "" + Iyran → Say + "" + Iyran (√śī vidhiliṅ 3pl).
    let mut p = Prakriya {
        terms: vec![Term::new("Se"), Term::new(""), Term::new("Iyran")],
        ..Default::default()
    };
    let rule = rules().find(|r| r.id == "6.1.78").unwrap();
    assert!((rule.apply)(&mut p));
    assert_eq!(p.terms[ANGA].text, "Say");
}
```

- [ ] **Step 2: Run to verify the new one fails**

```bash
mise exec -- cargo test -p panini-prakriya eco_yavayavah
```
Expected: `eco_yavayavah_converts_the_vikaranas_o` FAILS (the rule returns `false`); the other three PASS.

- [ ] **Step 3: Restructure 6.1.78 so the early return does not short-circuit**

Replace the opening lines

```rust
            let anga_last = p.terms[ANGA].text.chars().last().unwrap();
            let sub = match anga_last {
                'e' => "ay",
                'o' => "av",
                _ => return false,
            };
```

with a helper and a non-returning lookup, then add the third arm **after** the two existing ones:

```rust
            fn sub_for(c: char) -> Option<&'static str> {
                match c {
                    'e' => Some("ay"),
                    'o' => Some("av"),
                    _ => None,
                }
            }

            // The two arms below operate on the aṅga's own final ec. They
            // are reached only when the root has one; svādi's roots never
            // do, which is why the vikaraṇa arm at the bottom exists.
            if let Some(anga_last) = p.terms[ANGA].text.chars().last()
                && let Some(sub) = sub_for(anga_last)
            {
                // … the existing thematic arm, unchanged …
                // … the existing athematic arm, unchanged …
            }

            // Vikaraṇa arm (svādi): 7.3.84's second application has just
            // guṇated śnu's `u` to `o`, so the ec this sūtra converts sits
            // on the VIKARAṆA, not on the aṅga — Ap + no + Ani → Apnav +
            // Ani. Mutually exclusive with both arms above by construction:
            // those require the aṅga to end in e/o, which no svādi root
            // does, and this one requires SHAP to end in e/o, which none of
            // śap `a`, śyan `ya`, śa `a`, śnā `nA`/`n`/`nI`, śānac `Ana` or
            // adādi's empty śap ever does.
            if p.terms.len() > ENDING
                && let Some(shap_last) = p.terms[SHAP].text.chars().last()
                && let Some(sub) = sub_for(shap_last)
                && let Some(next_first) = p.terms[ENDING].text.chars().next()
                && is_vowel(next_first)
            {
                let before = p.snapshot();
                let mut s: Vec<char> = p.terms[SHAP].text.chars().collect();
                s.pop();
                p.terms[SHAP].text = s.into_iter().collect::<String>() + sub;
                p.record("6.1.78", "eco'yavAyAvaH", before);
                return true;
            }
            false
```

Keep both existing arms byte-identical inside the new `if let` block — only their surrounding scope changes.

- [ ] **Step 4: Run the tests**

```bash
mise exec -- cargo test -p panini-prakriya && mise run test
```
Expected: PASS. The restructuring is the risky part of this task; the 1296-form regression is what proves it.

- [ ] **Step 5: Commit**

```bash
mise run fmt-check && mise run lint
git add -A
git commit -m "feat(prakriya): 6.1.78 gains a vikaraṇa arm

The rule returned before either existing arm unless terms[ANGA] ended in
e/o. For svādi the guṇated o sits on the vikaraṇa (Apno), so ApnavAni
would have surfaced as *ApnoAni — 18 of the gaṇa's 216 cells. Third arm
converts SHAP's final ec instead; mutually exclusive with the other two,
since no svādi root has an ec-final aṅga and no vikaraṇa but śnu is ever
ec-final."
```

---

## Task 6: 6.4.87 and 6.4.77 — śnu's `u` before a vowel

Before a vowel-initial sārvadhātuka, śnu's `u` becomes `v` when *asaṁyogapūrva* (6.4.87, the apavāda) and `uv` otherwise (6.4.77, the utsarga).

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/sound.rs`
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: `is_vowel` from `super::sound`.
- Produces: `pub(crate) fn shnu_asamyogapurva(p: &Prakriya) -> bool` in `sound.rs`, used here and by 6.4.106 in Task 8. Two rules, `6.4.87` then `6.4.77`, ordered immediately after 7.3.84's second application.

- [ ] **Step 1: Write the failing helper test**

In `sound.rs`'s `mod tests` — an **enumerated membership test**, because this predicate fails silently:

```rust
#[test]
fn shnu_asamyogapurva_is_true_exactly_for_the_vowel_final_roots() {
    // The `u` of śnu is asaṁyogapūrva iff the `n` follows a vowel — i.e.
    // iff the aṅga's final character is a vowel. A wrong predicate here
    // turns hinu into *hinuhi and Apnuhi into *Apnu, both of which look
    // like plausible Sanskrit, so enumerate rather than rely on goldens.
    for (root, expected) in [
        ("hi", true),   // svādi, vowel-final
        ("ri", true),   // svādi, vowel-final
        ("Ap", false),  // svādi, `pn` conjunct
        ("Sak", false), // svādi, `kn` conjunct
        ("aS", false),  // svādi, `Sn` conjunct — the counter-intuitive one
        ("stiG", false),// svādi, `Gn` conjunct
        ("kliS", false),// kryādi control
        ("BU", false),  // bhvādi control (SHAP is not `nu` anyway)
    ] {
        let mut p = Prakriya {
            terms: vec![Term::new(root), Term::new("nu"), Term::new("anti")],
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        assert_eq!(
            shnu_asamyogapurva(&p),
            expected,
            "{root}: asaṁyogapūrva should be {expected}"
        );
    }
}
```

- [ ] **Step 2: Write the failing rule tests**

In `guna.rs`'s `mod tests`:

```rust
#[test]
fn hushnuvoh_yields_yan_for_a_vowel_final_root() {
    // hi + nu + anti → hi + nv + anti → hinvanti.
    let mut p = Prakriya {
        terms: vec![Term::new("hi"), Term::new("nu"), Term::new("anti")],
        ..Default::default()
    };
    p.terms[SHAP].add(Tag::Vikarana);
    let rule = rules().find(|r| r.id == "6.4.87").unwrap();
    assert!((rule.apply)(&mut p));
    assert_eq!(p.terms[SHAP].text, "nv");
}

#[test]
fn hushnuvoh_declines_on_a_conjunct_and_leaves_it_to_6_4_77() {
    let mut p = Prakriya {
        terms: vec![Term::new("Ap"), Term::new("nu"), Term::new("anti")],
        ..Default::default()
    };
    p.terms[SHAP].add(Tag::Vikarana);
    let apavada = rules().find(|r| r.id == "6.4.87").unwrap();
    assert!(!(apavada.apply)(&mut p));
    let utsarga = rules().find(|r| r.id == "6.4.77").unwrap();
    assert!((utsarga.apply)(&mut p));
    assert_eq!(p.terms[SHAP].text, "nuv");
}

#[test]
fn shnu_vowel_rules_decline_before_a_consonant_ending() {
    // ApnutaH: `taH` is consonant-initial, so neither fires.
    let mut p = Prakriya {
        terms: vec![Term::new("Ap"), Term::new("nu"), Term::new("taH")],
        ..Default::default()
    };
    p.terms[SHAP].add(Tag::Vikarana);
    for id in ["6.4.87", "6.4.77"] {
        let rule = rules().find(|r| r.id == id).unwrap();
        assert!(!(rule.apply)(&mut p), "{id} should decline");
    }
    assert_eq!(p.terms[SHAP].text, "nu");
}

#[test]
fn shnu_vowel_rules_decline_once_guna_has_run() {
    // ApnavAni: 7.3.84's second application already made SHAP `no`, so
    // neither rule matches `nu` any more. This is what keeps the ordering
    // constraint honest — *ApnuvAni is the failure it prevents.
    let mut p = Prakriya {
        terms: vec![Term::new("Ap"), Term::new("no"), Term::new("Ani")],
        ..Default::default()
    };
    p.terms[SHAP].add(Tag::Vikarana);
    for id in ["6.4.87", "6.4.77"] {
        let rule = rules().find(|r| r.id == id).unwrap();
        assert!(!(rule.apply)(&mut p), "{id} should decline");
    }
}

#[test]
fn shnu_vowel_rules_never_touch_another_ganas_vikarana() {
    // kryādi's `nA` and bhvādi's `a` must be invisible to both rules.
    for shap in ["nA", "nI", "n", "a", "ya", "Ana", ""] {
        let mut p = Prakriya {
            terms: vec![Term::new("kliS"), Term::new(shap), Term::new("anti")],
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        for id in ["6.4.87", "6.4.77"] {
            let rule = rules().find(|r| r.id == id).unwrap();
            assert!(!(rule.apply)(&mut p), "{id} fired on SHAP {shap:?}");
        }
    }
}
```

- [ ] **Step 3: Run to verify they fail**

```bash
mise exec -- cargo test -p panini-prakriya shnu_asamyogapurva hushnuvoh shnu_vowel
```
Expected: FAIL to compile — no `shnu_asamyogapurva`, no 6.4.87 / 6.4.77.

- [ ] **Step 4: Add the helper**

In `sound.rs`:

```rust
/// Is śnu's `u` *asaṁyogapūrva* — preceded by a single consonant rather
/// than a conjunct?
///
/// The condition 6.4.87 inherits by anuvṛtti from 6.4.82 *er anekāco'saṁ-
/// yogapūrvasya*, and the same condition 6.4.106 states in its own text.
/// The `u` is always preceded by śnu's own `n`, so the question reduces to
/// whether that `n` follows a vowel — i.e. whether the aṅga's final
/// character is a vowel.
///
/// √hi and √ri qualify (`hinu`, `riRu`); √āp, √śak, √aś and √ṣṭigh do not
/// (`Apnu`, `Saknu`, `aSnu`, `stiGnu`). √aś is the counter-intuitive one:
/// it looks like √su, but the root's own final `S` joins śnu's `n` into a
/// conjunct — which is why `aSnumahe` has no lopa alternate while
/// `sunmahe` does.
///
/// Returns false whenever the vikaraṇa is not śnu, so callers do not need
/// their own gaṇa test.
pub(crate) fn shnu_asamyogapurva(p: &Prakriya) -> bool {
    if p.terms.get(SHAP).map(|t| t.text.as_str()) != Some("nu") {
        return false;
    }
    p.terms[ANGA]
        .text
        .chars()
        .last()
        .is_some_and(is_vowel)
}
```

Add the imports `sound.rs` needs for this (`crate::prakriya::Prakriya`, `crate::tinanta::terms::{ANGA, SHAP}`).

- [ ] **Step 5: Add the two rules**

In `guna.rs`, immediately after 7.3.84's second application:

```rust
    // 6.4.87 huśnuvoḥ sārvadhātuke: for √hu and śnu, before a sārvadhātuka,
    // yaṇ — `u` → `v` — rather than 6.4.77's uvaṅ. hi + nu + anti →
    // hinvanti; ri + nu + antu → riRvantu (ṇatva lands later, in tripadi).
    //
    // The *asaṁyogapūrva* restriction is anuvṛtti from 6.4.82 er anekāco'-
    // saṁyogapūrvasya; it is not visible in this sūtra's own words, which
    // is why the guard would otherwise look invented. It is what separates
    // hinvanti from Apnuvanti.
    //
    // The √hu arm is not implemented: √hu is juhotyādi, out of scope. Widen
    // when gaṇa 3 lands.
    //
    // APAVĀDA to 6.4.77 below, and ordered before it as the pipeline's other
    // apavāda pairs are (3.1.69 before 3.1.68; 6.4.72 before 6.4.71). It
    // self-guards: once this rule has written `nv`, 6.4.77's `nu` test no
    // longer matches, so no "did the apavāda fire?" check is needed.
    Rule {
        id: "6.4.87",
        name: "huSnuvoH sArvaDAtuke",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms.len() <= ENDING || !shnu_asamyogapurva(p) {
                return false;
            }
            let Some(next) = p.terms[ENDING].text.chars().next() else {
                return false;
            };
            if !is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "nv".into();
            p.record("6.4.87", "huSnuvoH sArvaDAtuke", before);
            true
        },
    },
    // 6.4.77 aci śnudhātubhruvāṁ yvor iyaṅuvaṅau: before a vowel, śnu's `u`
    // becomes uvaṅ. Ap + nu + anti → Apnuvanti; aS + nu + ate → aSnuvate;
    // aS + nu + Iyta → aSnuvIta (6.1.66 drops the y later, in `adesha`).
    //
    // Only the śnu arm is implemented. The *dhātu* arm (ī/ū-final roots) and
    // the *bhrū* arm have no root in scope — recorded rather than written,
    // as 6.4.112's *abhyasta* half and 6.4.113's *aghoḥ* are. Widen when a
    // root reaches either.
    //
    // Reads terms[ENDING] directly, NOT `following_sarvadhatuka`: that
    // helper answers "what follows the aṅga", which here is śnu itself —
    // this rule needs what follows śnu. Same reasoning as 6.4.112/6.4.113.
    Rule {
        id: "6.4.77",
        name: "aci SnuDAtuBruvAM yvoriyaNuvaNO",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms.len() <= ENDING || p.terms[SHAP].text != "nu" {
                return false;
            }
            let Some(next) = p.terms[ENDING].text.chars().next() else {
                return false;
            };
            if !is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "nuv".into();
            p.record("6.4.77", "aci SnuDAtuBruvAM yvoriyaNuvaNO", before);
            true
        },
    },
```

- [ ] **Step 6: Update the pinned rule order**

Insert `"6.4.87", "6.4.77"` immediately after the second `"7.3.84"`. The array goes from 64 to 66 entries.

- [ ] **Step 7: Run the tests**

```bash
mise exec -- cargo test -p panini-prakriya && mise run test
```
Expected: PASS, 1296 goldens unchanged — both rules require `SHAP == "nu"`, which no existing gaṇa produces.

- [ ] **Step 8: Commit**

```bash
mise run fmt-check && mise run lint
git add -A
git commit -m "feat(prakriya): 6.4.87 + 6.4.77, śnu's u before a vowel

yaṇ when asaṁyogapūrva (hinvanti), uvaṅ otherwise (Apnuvanti). 6.4.87 is
the apavāda and is ordered first, self-guarding by rewriting nu to nv.
The asaṁyogapūrva predicate is a shared helper with an enumerated
membership test — it fails silently, so goldens are not enough."
```

---

## Task 7: `sound_before_ending` and the 6.4.101 fix

6.4.101 *hujhalbhyo her dhiḥ* turns loṭ 2sg `hi` into `Di` after a jhal-final aṅga (√ad's `adDi`). It reads the **root's** final character. `is_jhal('p')` and `is_jhal('k')` are both true, so for √āp and √śak it fires and produces \*`ApnuDi` / \*`SaknuDi` — while the sound actually preceding `hi` is śnu's `u`.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/terms.rs`
- Modify: `crates/panini-prakriya/src/tinanta/adesha.rs`

**Interfaces:**
- Consumes: nothing.
- Produces: `pub(crate) fn sound_before_ending(p: &Prakriya) -> Option<char>` in `terms.rs`.

- [ ] **Step 1: Write the failing tests**

In `adesha.rs`'s `mod tests`:

```rust
#[test]
fn her_dhih_reads_the_sound_before_the_ending_not_the_root() {
    // Ap + nu + hi must stay Apnuhi. `p` is a jhal, but it is not what
    // precedes `hi` — śnu's `u` is, and `u` is not a jhal.
    let mut p = Prakriya {
        terms: vec![Term::new("Ap"), Term::new("nu"), Term::new("hi")],
        ..Default::default()
    };
    let rule = rules().find(|r| r.id == "6.4.101").unwrap();
    assert!(!(rule.apply)(&mut p));
    assert_eq!(p.terms[ENDING].text, "hi");
}

#[test]
fn her_dhih_still_fires_for_adadi_across_an_empty_shap() {
    // √ad: śap is luk'd, so the nearest non-empty term before the ending is
    // the root itself and `d` is still the right character. adDi.
    let mut p = Prakriya {
        terms: vec![Term::new("ad"), Term::new(""), Term::new("hi")],
        ..Default::default()
    };
    let rule = rules().find(|r| r.id == "6.4.101").unwrap();
    assert!((rule.apply)(&mut p));
    assert_eq!(p.terms[ENDING].text, "Di");
}

#[test]
fn her_dhih_declines_for_kryadi_shni() {
    // vrI + nI + hi → vrIRIhi. `I` is not a jhal. Unchanged by this task,
    // pinned so the change is provably a no-op here too.
    let mut p = Prakriya {
        terms: vec![Term::new("vrI"), Term::new("nI"), Term::new("hi")],
        ..Default::default()
    };
    let rule = rules().find(|r| r.id == "6.4.101").unwrap();
    assert!(!(rule.apply)(&mut p));
}
```

- [ ] **Step 2: Run to verify the first fails**

```bash
mise exec -- cargo test -p panini-prakriya her_dhih
```
Expected: `her_dhih_reads_the_sound_before_the_ending_not_the_root` FAILS — the rule fires and writes `Di`. The other two PASS.

- [ ] **Step 3: Add the helper**

In `terms.rs`, beside `following_sarvadhatuka`:

```rust
/// The sound immediately preceding the ending — the last character of the
/// nearest **non-empty** term before `ENDING`.
///
/// Rules that ask "what does the ending attach to?" cannot read `ANGA`: a
/// non-empty vikaraṇa sits between the two, and it is the vikaraṇa's final
/// sound the ending actually meets. They cannot read `SHAP` either, because
/// 2.4.72 luks śap to an empty string for adādi, where the ending really
/// does attach to the root.
///
/// The fallback is what keeps `adDi` working: with śap empty the search
/// walks past it to the root's `d`. 8.3.59 in `tripadi.rs` open-codes the
/// same walk for the same reason.
///
/// Returns `None` for a prakriyā with no term before the ending.
pub(crate) fn sound_before_ending(p: &Prakriya) -> Option<char> {
    p.terms
        .get(..ENDING)?
        .iter()
        .rev()
        .find_map(|t| t.text.chars().last())
}
```

- [ ] **Step 4: Use it in 6.4.101**

Replace

```rust
            let Some(last) = p.terms[ANGA].text.chars().last() else {
                return false;
            };
```

with

```rust
            // NOT terms[ANGA]. The jhal this sūtra tests is the sound the
            // ending attaches to, which for a gaṇa with a live vikaraṇa is
            // the vikaraṇa's final, not the root's. Reading ANGA fired on
            // √āp's `p` and √śak's `k` and gave *ApnuDi / *SaknuDi, even
            // though śnu's `u` sits between. adādi still reaches the root
            // because its śap is empty and the helper walks past it.
            let Some(last) = sound_before_ending(p) else {
                return false;
            };
```

and add `sound_before_ending` to `adesha.rs`'s `use crate::tinanta::terms::{…}` list.

- [ ] **Step 5: Run the tests**

```bash
mise exec -- cargo test -p panini-prakriya && mise run test
```
Expected: PASS, 1296 goldens unchanged. `adDi` is the tripwire — if it moves, the fallback is wrong.

- [ ] **Step 6: Commit**

```bash
mise run fmt-check && mise run lint
git add -A
git commit -m "fix(prakriya): 6.4.101 reads the sound before the ending

is_jhal('p') and is_jhal('k') are true, so the rule fired on √āp and √śak
and would have given *ApnuDi / *SaknuDi once svādi's roots landed — śnu's
u sits between the root and the ending. New sound_before_ending helper
walks to the nearest non-empty term, which keeps adādi's adDi working
across its luk'd śap. No existing form changes."
```

---

## Task 8: 6.4.106 — the hi-luk after a non-conjunct `u`

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/adesha.rs`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: `shnu_asamyogapurva` from Task 6.
- Produces: a rule `6.4.106`, ordered immediately after 6.4.105 and before 6.4.101.

- [ ] **Step 1: Write the failing tests**

```rust
#[test]
fn utash_ca_luks_hi_after_a_non_conjunct_u() {
    // hi + nu + hi → hinu.
    let mut p = Prakriya {
        terms: vec![Term::new("hi"), Term::new("nu"), Term::new("hi")],
        ..Default::default()
    };
    p.terms[SHAP].add(Tag::Vikarana);
    let rule = rules().find(|r| r.id == "6.4.106").unwrap();
    assert!((rule.apply)(&mut p));
    assert_eq!(p.terms[ENDING].text, "");
}

#[test]
fn utash_ca_declines_after_a_conjunct_u() {
    // Ap + nu + hi → Apnuhi. The asaṁyogapūrva clause is the whole rule.
    let mut p = Prakriya {
        terms: vec![Term::new("Ap"), Term::new("nu"), Term::new("hi")],
        ..Default::default()
    };
    p.terms[SHAP].add(Tag::Vikarana);
    let rule = rules().find(|r| r.id == "6.4.106").unwrap();
    assert!(!(rule.apply)(&mut p));
    assert_eq!(p.terms[ENDING].text, "hi");
}

#[test]
fn utash_ca_declines_when_the_ending_is_not_hi() {
    let mut p = Prakriya {
        terms: vec![Term::new("hi"), Term::new("nu"), Term::new("ti")],
        ..Default::default()
    };
    p.terms[SHAP].add(Tag::Vikarana);
    let rule = rules().find(|r| r.id == "6.4.106").unwrap();
    assert!(!(rule.apply)(&mut p));
}
```

- [ ] **Step 2: Run to verify they fail**

```bash
mise exec -- cargo test -p panini-prakriya utash_ca
```
Expected: FAIL — no rule with id `6.4.106`.

- [ ] **Step 3: Add the rule**

In `adesha.rs`, between the 6.4.105 and 6.4.101 entries:

```rust
    // 6.4.106 utaś ca pratyayād asaṁyogapūrvāt: `hi` is luk'd after an
    // affix-final `u` that is not conjunct-preceded. hi + nu + hi → hinu;
    // ri + nu + hi → riRu (ṇatva lands later). Ap + nu + hi keeps its `hi`
    // → Apnuhi, and that pair is the rule's pin.
    //
    // Continues the luk of 6.4.105 ato heḥ immediately above, which is why
    // it sits here rather than in sūtra-number order elsewhere. 6.4.105
    // declines for svādi on its own guard (the stem ends in `u`, not a
    // short `a`), so the two never contend.
    //
    // Must precede 6.4.101 her DhiH below: for the conjunct roots this rule
    // deliberately leaves `hi` standing, and 6.4.101 is what must then also
    // decline — see its own comment on reading the sound before the ending.
    Rule {
        id: "6.4.106",
        name: "utaSca pratyayAdasaMyogapUrvAt",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms.len() <= ENDING || p.terms[ENDING].text != "hi" {
                return false;
            }
            if !shnu_asamyogapurva(p) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = String::new();
            p.record("6.4.106", "utaSca pratyayAdasaMyogapUrvAt", before);
            true
        },
    },
```

Add `shnu_asamyogapurva` to `adesha.rs`'s `use crate::tinanta::sound::{…}` list.

- [ ] **Step 4: Update the pinned rule order**

Insert `"6.4.106"` between `"6.4.105"` and `"6.4.101"`. The array goes from 66 to 67 entries.

- [ ] **Step 5: Run the tests**

```bash
mise exec -- cargo test -p panini-prakriya && mise run test
```
Expected: PASS, 1296 goldens unchanged.

- [ ] **Step 6: Commit**

```bash
mise run fmt-check && mise run lint
git add -A
git commit -m "feat(prakriya): 6.4.106 utaS ca, the śnu hi-luk

hinu and riRu against Apnuhi and Saknuhi. Reuses the asaṁyogapūrva helper
6.4.87 introduced; placed after 6.4.105, whose luk it continues, and
before 6.4.101."
```

---

## Task 9: 6.1.90's athematic arm widens

For svādi, `SHAP` at the junction stage is `nav` — not empty, not `a`-final, not `A`-final. No arm of 6.1.101 fires, and no arm of 6.1.90 fires, so the āṭ `A` is stranded at the head of the ending and `aSnavE` surfaces as \*`aSnavAE`.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/adesha.rs`

**Interfaces:**
- Consumes: Task 5's `nav`.
- Produces: no new rule id.

- [ ] **Step 1: Write the failing tests**

```rust
#[test]
fn atash_ca_athematic_arm_fires_for_a_svadi_stem() {
    // aS + nav + AE → aS + nav + E → aSnavE (loṭ ātmanepada uttama eka).
    let mut p = Prakriya {
        terms: vec![Term::new("aS"), Term::new("nav"), Term::new("AE")],
        ..Default::default()
    };
    let rule = rules().find(|r| r.id == "6.1.90").unwrap();
    assert!((rule.apply)(&mut p));
    assert_eq!(p.terms[ENDING].text, "E");
    assert_eq!(p.terms[SHAP].text, "nav");
}

#[test]
fn atash_ca_athematic_arm_still_fires_for_adadi() {
    // As + "" + AE → AsE. The arm's original job; must not regress.
    let mut p = Prakriya {
        terms: vec![Term::new("As"), Term::new(""), Term::new("AE")],
        ..Default::default()
    };
    let rule = rules().find(|r| r.id == "6.1.90").unwrap();
    assert!((rule.apply)(&mut p));
    assert_eq!(p.terms[ENDING].text, "E");
}

#[test]
fn atash_ca_athematic_arm_stays_off_a_and_capital_a_final_shap() {
    // bhvādi (`A` after 6.1.101) and kryādi (`nA`) are handled by the
    // thematic arm; this arm must keep its hands off both.
    for shap in ["a", "A", "ya", "yA", "nA", "Ana"] {
        let mut p = Prakriya {
            terms: vec![Term::new("laB"), Term::new(shap), Term::new("AE")],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.90").unwrap();
        let fired = (rule.apply)(&mut p);
        // Either the thematic arm fired (A-final SHAP) or nothing did; in
        // neither case may the ENDING keep a stranded leading A.
        if !fired {
            assert_eq!(p.terms[ENDING].text, "AE", "SHAP {shap:?}");
        }
    }
}

#[test]
fn atash_ca_declines_when_the_ending_is_not_a_plus_ec() {
    // ApnavAni: `Ani` is A + n, not A + ec, so nothing coalesces.
    let mut p = Prakriya {
        terms: vec![Term::new("Ap"), Term::new("nav"), Term::new("Ani")],
        ..Default::default()
    };
    let rule = rules().find(|r| r.id == "6.1.90").unwrap();
    assert!(!(rule.apply)(&mut p));
    assert_eq!(p.terms[ENDING].text, "Ani");
}
```

- [ ] **Step 2: Run to verify the first fails**

```bash
mise exec -- cargo test -p panini-prakriya atash_ca
```
Expected: `atash_ca_athematic_arm_fires_for_a_svadi_stem` FAILS; the rest PASS.

- [ ] **Step 3: Widen the guard**

In 6.1.90's athematic ending arm, replace

```rust
            if p.terms.len() > ENDING && p.terms[SHAP].text.is_empty() {
```

with

```rust
            // Widened from `is_empty()` for svādi. The arm's job is "the
            // coalescence rules never consumed the āṭ A into SHAP, so it
            // still leads the ending" — and emptiness was only ever a proxy
            // for that. adādi's empty śap qualifies, as before; so does
            // svādi's `nav`, which fails every arm of 6.1.101 (its `v` is
            // neither savarṇa with A nor an `a`/`A` for the bhvādi and
            // kryādi arms) and so really does leave the A stranded.
            //
            // `a`- and `A`-final SHAPs are excluded because for them 6.1.101
            // HAS already acted: bhvādi's śap became `A` and kryādi's śnā
            // already swallowed the ending's leading A, so both are the
            // thematic arm's business and reaching here would double-count.
            // This is the correction vikarana.rs's 3.1.81 comment predicts
            // in general terms — is_empty() as a stand-in for "the thematic
            // path didn't apply" silently declines for a non-empty,
            // non-`a`-final vikaraṇa.
            let shap = &p.terms[SHAP].text;
            if p.terms.len() > ENDING && !shap.ends_with('a') && !shap.ends_with('A') {
```

- [ ] **Step 4: Run the tests**

```bash
mise exec -- cargo test -p panini-prakriya && mise run test
```
Expected: PASS, 1296 goldens unchanged. `AsE` is the tripwire.

- [ ] **Step 5: Commit**

```bash
mise run fmt-check && mise run lint
git add -A
git commit -m "fix(prakriya): 6.1.90's athematic arm widens past is_empty()

svādi's junction-stage SHAP is `nav`: it fails every arm of 6.1.101, so
the āṭ A really is stranded at the head of the ending, but the arm that
handles that case was gated on the śap being EMPTY. aSnavE would have
surfaced as *aSnavAE. Guard is now 'SHAP ends in neither a nor A', which
keeps adādi's AsE and excludes the thematic path exactly as before."
```

---

## Task 10: Land the four parasmaipadī roots and their 144 goldens

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `data/dhatupatha.tsv`
- Modify: `crates/panini/tests/paradigm.rs`
- Modify: `crates/panini/tests/trace.rs`

**Interfaces:**
- Consumes: everything from Tasks 3–9.
- Produces: four `Dhatu` rows with `id`/`code` `"Ap"`, `"Sak"`, `"hi"`, `"ri"` and `gana: Gana::Svadi`. `dhatus().len()` becomes 40.

- [ ] **Step 1: Add the reference rows to `data/dhatupatha.tsv`**

Append (tab-separated; the file is documentation kept in sync by hand, not compiled):

```
Ap	svadi	parasmaipada	vyAptO
Sak	svadi	parasmaipada	SaktO
hi	svadi	parasmaipada	gatO vfdDO ca
ri	svadi	parasmaipada	hiMsAyAm
```

- [ ] **Step 2: Add the four `Dhatu` rows**

In `crates/panini-data/src/lib.rs`'s `DHATUS`, after the kryādi block:

```rust
    Dhatu {
        id: "Ap",
        code: "Ap",
        gana: Gana::Svadi,
        pada: Pada::Parasmaipada,
        artha: "vyAptO",
    },
    Dhatu {
        id: "Sak",
        code: "Sak",
        gana: Gana::Svadi,
        pada: Pada::Parasmaipada,
        artha: "SaktO",
    },
    Dhatu {
        id: "hi",
        code: "hi",
        gana: Gana::Svadi,
        pada: Pada::Parasmaipada,
        artha: "gatO vfdDO ca",
    },
    Dhatu {
        id: "ri",
        code: "ri",
        gana: Gana::Svadi,
        pada: Pada::Parasmaipada,
        artha: "hiMsAyAm",
    },
```

Update the `assert_eq!(dhatus().len(), 36)` in that file's tests to `40`, and add presence assertions in the same style as the kryādi ones:

```rust
        // New: svādi (gaṇa 5), all four parasmaipadī.
        for id in ["Ap", "Sak", "hi", "ri"] {
            let d = dhatus().iter().find(|d| d.id == id).unwrap();
            assert!(matches!(d.gana, Gana::Svadi));
            assert!(matches!(d.pada, Pada::Parasmaipada));
        }
```

- [ ] **Step 3: Add the 16 golden blocks**

In `crates/panini/tests/paradigm.rs`'s `PARADIGM`, append blocks in the file's existing shape — `(root_id, lakara_label, [P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B])`. These forms are from the spec's "Golden forms" section, generated by a vidyut-prakriya probe:

```rust
    ("Ap", "laT", ["Apnoti", "ApnutaH", "Apnuvanti", "Apnozi", "ApnuTaH", "ApnuTa", "Apnomi", "ApnuvaH", "ApnumaH"]),
    ("Ap", "laN", ["Apnot", "ApnutAm", "Apnuvan", "ApnoH", "Apnutam", "Apnuta", "Apnavam", "Apnuva", "Apnuma"]),
    ("Ap", "loT", ["Apnotu", "ApnutAm", "Apnuvantu", "Apnuhi", "Apnutam", "Apnuta", "ApnavAni", "ApnavAva", "ApnavAma"]),
    ("Ap", "viDiliN", ["ApnuyAt", "ApnuyAtAm", "ApnuyuH", "ApnuyAH", "ApnuyAtam", "ApnuyAta", "ApnuyAm", "ApnuyAva", "ApnuyAma"]),
    ("Sak", "laT", ["Saknoti", "SaknutaH", "Saknuvanti", "Saknozi", "SaknuTaH", "SaknuTa", "Saknomi", "SaknuvaH", "SaknumaH"]),
    ("Sak", "laN", ["aSaknot", "aSaknutAm", "aSaknuvan", "aSaknoH", "aSaknutam", "aSaknuta", "aSaknavam", "aSaknuva", "aSaknuma"]),
    ("Sak", "loT", ["Saknotu", "SaknutAm", "Saknuvantu", "Saknuhi", "Saknutam", "Saknuta", "SaknavAni", "SaknavAva", "SaknavAma"]),
    ("Sak", "viDiliN", ["SaknuyAt", "SaknuyAtAm", "SaknuyuH", "SaknuyAH", "SaknuyAtam", "SaknuyAta", "SaknuyAm", "SaknuyAva", "SaknuyAma"]),
    ("hi", "laT", ["hinoti", "hinutaH", "hinvanti", "hinozi", "hinuTaH", "hinuTa", "hinomi", "hinuvaH", "hinumaH"]),
    ("hi", "laN", ["ahinot", "ahinutAm", "ahinvan", "ahinoH", "ahinutam", "ahinuta", "ahinavam", "ahinuva", "ahinuma"]),
    ("hi", "loT", ["hinotu", "hinutAm", "hinvantu", "hinu", "hinutam", "hinuta", "hinavAni", "hinavAva", "hinavAma"]),
    ("hi", "viDiliN", ["hinuyAt", "hinuyAtAm", "hinuyuH", "hinuyAH", "hinuyAtam", "hinuyAta", "hinuyAm", "hinuyAva", "hinuyAma"]),
    ("ri", "laT", ["riRoti", "riRutaH", "riRvanti", "riRozi", "riRuTaH", "riRuTa", "riRomi", "riRuvaH", "riRumaH"]),
    ("ri", "laN", ["ariRot", "ariRutAm", "ariRvan", "ariRoH", "ariRutam", "ariRuta", "ariRavam", "ariRuva", "ariRuma"]),
    ("ri", "loT", ["riRotu", "riRutAm", "riRvantu", "riRu", "riRutam", "riRuta", "riRavAni", "riRavAva", "riRavAma"]),
    ("ri", "viDiliN", ["riRuyAt", "riRuyAtAm", "riRuyuH", "riRuyAH", "riRuyAtam", "riRuyAta", "riRuyAm", "riRuyAva", "riRuyAma"]),
```

Match the file's existing formatting — `cargo fmt` will rewrap these.

- [ ] **Step 4: Run the goldens and diagnose every failure as an engine bug**

```bash
mise exec -- cargo test -p panini --test paradigm 2>&1 | head -60
```
Expected: PASS. If a cell fails, **do not edit the golden.** Work out which rule declined, using the spec's per-rule sections. The likely suspects, in order: the loṭ uttama cells (Tasks 4/5 ordering), the loṭ 2sg cells (Tasks 7/8), and `riR*` (8.4.1/8.4.2 seeing the new stem).

- [ ] **Step 5: Add the parasmaipada ordered-trace pins**

In `crates/panini/tests/trace.rs`. The expected sequences must be **read off the engine and then checked against the spec**, not guessed — print them first:

```bash
mise exec -- cargo run -p panini-cli -- check Apnoti --trace
mise exec -- cargo run -p panini-cli -- check ApnavAni --trace
mise exec -- cargo run -p panini-cli -- check Apnuvanti --trace
mise exec -- cargo run -p panini-cli -- check Apnuhi --trace
mise exec -- cargo run -p panini-cli -- check hinvanti --trace
mise exec -- cargo run -p panini-cli -- check hinu --trace
mise exec -- cargo run -p panini-cli -- check riRoti --trace
```

Before pinning each, verify against the spec that the sūtras present are the ones that should be, in the order the spec's placement arguments require. Specifically check that:

- `Apnoti` contains `3.1.73`, one `1.2.4`, and `7.3.84`, and does **not** contain `3.1.68`
- `ApnutaH` contains `3.1.73` and `1.2.4` but **no** `7.3.84`
- `ApnavAni` has `7.3.84` **before** `6.1.78`
- `Apnuvanti` has `6.4.77` and no `6.4.87`
- `hinvanti` has `6.4.87` and no `6.4.77`
- `hinu` has `6.4.106`; `Apnuhi` has neither `6.4.106` nor `6.4.101`
- `riRoti` ends with `8.4.1` or `8.4.2`

Then write one `#[test]` per form, in the file's existing style:

```rust
#[test]
fn apnoti_trace_pins_the_vikarana_guna() {
    // Ap prathama eka. The second 7.3.84 guṇates śnu's `u`; the first
    // declines on the root's `p`. 3.1.68 never fires — 3.1.73 is its
    // apavāda.
    assert_eq!(trace_for("Apnoti"), vec![/* pasted from the run above */]);
}
```

- [ ] **Step 6: Run everything**

```bash
mise run test
```
Expected: PASS — 1440 goldens, and the 1296 pre-existing forms and traces byte-identical.

- [ ] **Step 7: Commit**

```bash
mise run fmt-check && mise run lint
git add -A
git commit -m "feat: svādi parasmaipada — Ap, Sak, hi, ri (1296 -> 1440)

The four parasmaipadī roots across four lakāras. Ap/Sak witness the
conjunct path (Apnuvanti, Apnuhi), hi/ri the asaṁyogapūrva one
(hinvanti, hinu), and riRoti/riRvanti put ṇatva on the new stem."
```

---

## Task 11: Land the two ātmanepadī roots and their 72 goldens

These roots need **no new rule** — they are a pure reuse witness for 7.1.5, 7.2.79/7.2.80, 6.1.66, 6.1.101, 8.3.59 and Task 9's 6.1.90 widening.

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `data/dhatupatha.tsv`
- Modify: `crates/panini/tests/paradigm.rs`
- Modify: `crates/panini/tests/trace.rs`

**Interfaces:**
- Consumes: Tasks 3–10.
- Produces: `Dhatu` rows with `id: "aS.5"` / `code: "aS"` and `id: "stiG"` / `code: "stiG"`. `dhatus().len()` becomes 42.

- [ ] **Step 1: Add the reference rows to `data/dhatupatha.tsv`**

```
aS	svadi	atmanepada	vyAptO saNGAte ca
stiG	svadi	atmanepada	Askandane
```

Add a comment line at the top of the file (or extend the existing header, if one exists) recording that `stiG` is stored post-6.1.64 — the upadeśa is `zwiGa~\`, and no rule in the engine performs the ṣ → s substitution, so `stiGnute`'s trace will not mention 6.1.64.

- [ ] **Step 2: Add the two `Dhatu` rows**

```rust
    Dhatu {
        // 05.0020 aSU~\ vyAptau. Distinct root from kryādi's 09.0059 aSa~
        // Bojane, which shares this SLP1 form — hence the qualified id.
        // aSnute against aSnAti is the pair.
        id: "aS.5",
        code: "aS",
        gana: Gana::Svadi,
        pada: Pada::Atmanepada,
        artha: "vyAptO saNGAte ca",
    },
    Dhatu {
        // 05.0021 zwiGa~\. Stored post-6.1.64 dhātvādeḥ ṣaḥ saḥ: no rule in
        // the engine performs that substitution, so it is a stated
        // simplification, not a derivation step. See the spec's Data section.
        id: "stiG",
        code: "stiG",
        gana: Gana::Svadi,
        pada: Pada::Atmanepada,
        artha: "Askandane",
    },
```

Update `dhatus().len()` to `42`.

- [ ] **Step 3: Write the collision test**

In `crates/panini-data/src/lib.rs`'s tests — this is what prep 2 existed for:

```rust
#[test]
fn the_two_ash_roots_are_distinct_rows() {
    let svadi = dhatus().iter().find(|d| d.id == "aS.5").unwrap();
    let kryadi = dhatus().iter().find(|d| d.id == "aS").unwrap();
    assert!(matches!(svadi.gana, Gana::Svadi));
    assert!(matches!(kryadi.gana, Gana::Kryadi));
    assert!(matches!(svadi.pada, Pada::Atmanepada));
    assert!(matches!(kryadi.pada, Pada::Parasmaipada));
    // Same surface text, different rows. If ids ever collapse, one of these
    // roots silently stops being derivable.
    assert_eq!(svadi.code, kryadi.code);
}
```

And in `crates/panini/tests/paradigm.rs`, an integration counterpart:

```rust
#[test]
fn both_ash_roots_derive() {
    let engine = Panini::new();
    for form in ["aSnute", "aSnAti"] {
        assert!(matches!(engine.check(form).verdict, Verdict::Valid), "{form}");
    }
}
```

- [ ] **Step 4: Add the 8 golden blocks**

```rust
    ("aS.5", "laT", ["aSnute", "aSnuvAte", "aSnuvate", "aSnuze", "aSnuvATe", "aSnuDve", "aSnuve", "aSnuvahe", "aSnumahe"]),
    ("aS.5", "laN", ["ASnuta", "ASnuvAtAm", "ASnuvata", "ASnuTAH", "ASnuvATAm", "ASnuDvam", "ASnuvi", "ASnuvahi", "ASnumahi"]),
    ("aS.5", "loT", ["aSnutAm", "aSnuvAtAm", "aSnuvatAm", "aSnuzva", "aSnuvATAm", "aSnuDvam", "aSnavE", "aSnavAvahE", "aSnavAmahE"]),
    ("aS.5", "viDiliN", ["aSnuvIta", "aSnuvIyAtAm", "aSnuvIran", "aSnuvITAH", "aSnuvIyATAm", "aSnuvIDvam", "aSnuvIya", "aSnuvIvahi", "aSnuvImahi"]),
    ("stiG", "laT", ["stiGnute", "stiGnuvAte", "stiGnuvate", "stiGnuze", "stiGnuvATe", "stiGnuDve", "stiGnuve", "stiGnuvahe", "stiGnumahe"]),
    ("stiG", "laN", ["astiGnuta", "astiGnuvAtAm", "astiGnuvata", "astiGnuTAH", "astiGnuvATAm", "astiGnuDvam", "astiGnuvi", "astiGnuvahi", "astiGnumahi"]),
    ("stiG", "loT", ["stiGnutAm", "stiGnuvAtAm", "stiGnuvatAm", "stiGnuzva", "stiGnuvATAm", "stiGnuDvam", "stiGnavE", "stiGnavAvahE", "stiGnavAmahE"]),
    ("stiG", "viDiliN", ["stiGnuvIta", "stiGnuvIyAtAm", "stiGnuvIran", "stiGnuvITAH", "stiGnuvIyATAm", "stiGnuvIDvam", "stiGnuvIya", "stiGnuvIvahi", "stiGnuvImahi"]),
```

Note: `PARADIGM`'s first column is now an **id**, so the lookup helper in `paradigm.rs` must resolve `"aS.5"` via `d.id`. Prep 2 already made that change; verify it rather than assume.

- [ ] **Step 5: Run the goldens**

```bash
mise exec -- cargo test -p panini --test paradigm 2>&1 | head -60
```
Expected: PASS. The two cells to watch are `aSnavE` and `stiGnavE` — they are the only witnesses for Task 9's 6.1.90 widening, so if that task was wrong this is where it shows.

- [ ] **Step 6: Add the ātmanepada trace pins**

Print, verify against the spec, then pin — same procedure as Task 10 Step 5:

```bash
mise exec -- cargo run -p panini-cli -- check aSnuvate --trace
mise exec -- cargo run -p panini-cli -- check aSnuzva --trace
mise exec -- cargo run -p panini-cli -- check aSnuvIta --trace
mise exec -- cargo run -p panini-cli -- check aSnavE --trace
mise exec -- cargo run -p panini-cli -- check stiGnute --trace
```

Verify that `aSnuvate` has `7.1.5` before `6.4.77`; `aSnuzva` reaches `8.3.59`; `aSnuvIta` has `7.2.79` then `6.4.77` then `6.1.66`; `aSnavE` has `7.3.84`, `6.1.78`, `6.1.90` in that order.

- [ ] **Step 7: Run everything and commit**

```bash
mise run test && mise run fmt-check && mise run lint
git add -A
git commit -m "feat: svādi ātmanepada — aS.5 and stiG (1440 -> 1512)

The gaṇa's only two ātmanepadī roots. No new rule: pure reuse of 7.1.5,
7.2.79/80, 6.1.66, 6.1.101 and 8.3.59, plus the 6.1.90 widening whose
only witnesses are aSnavE and stiGnavE. aS.5 is the first root whose id
differs from its code — kryādi's aS shares the form."
```

---

## Task 12: Slice 5a docs and the verification gate

**Files:**
- Modify: `AGENTS.md`
- Modify: `docs/ARCHITECTURE.md`
- Modify: `README.md`

- [ ] **Step 1: Refresh `README.md`'s Scope section**

It currently says "four gaṇas" and lists bhvādi, divādi, tudādi, adādi — it predates kryādi as well as svādi. Rewrite to say **six gaṇas**, name all six with their vikaraṇas, and update the root count from 30 to 42. Keep the existing sentence about `INVALID` meaning "not derivable within this covered grammar".

- [ ] **Step 2: Update `AGENTS.md`**

In the golden-paradigm bullet: 1296 → **1512**, five gaṇas → **six**, and add a svādi sentence in the style of the kryādi one — naming śnu, the two-juncture guṇa (`Apnoti` against `ApnutaH`), the *asaṁyogapūrva* split (`hinvanti`/`hinu` against `Apnuvanti`/`Apnuhi`), and pointing at this spec.

Add a line to the "Rules of the codebase" section recording that **7.3.84 and 1.2.4 each appear twice in `TINANTA_RULES` by design**, because the aṅga is affix-relative (1.4.13) — with a pointer to the spec section. This is the single most likely thing for a future contributor to "clean up".

- [ ] **Step 3: Update `docs/ARCHITECTURE.md`**

Add `guna.rs` to the stage-file map (if Task 1 Step 6 did not already), and record the affix-relative aṅga theme: 7.3.84, 6.1.78, 6.1.90 and 6.4.101 all now distinguish "the root's final sound" from "the sound the affix actually meets."

- [ ] **Step 4: Run the full gate**

```bash
mise run fmt-check
mise run lint
mise run test
mise run audit
```
Expected: all clean.

- [ ] **Step 5: Run mutation testing**

Invoke the binary directly — the `mise` shim fails in background shells:

```bash
MISE_ENV=dev mise install
"$(MISE_ENV=dev mise which cargo-mutants)" --package panini-prakriya --test-workspace=true
```
Expected: **zero survivors.** A survivor in a new rule means a guard clause no test distinguishes; add the test rather than weakening the guard. A survivor may also be masked by a downstream repair — 7.3.100's "unkillable" mutant turned out to be one — in which case only an ordered-trace pin can see it.

- [ ] **Step 6: Verify the headline claims before declaring done**

```bash
# 1512 goldens
mise exec -- cargo test -p panini --test paradigm 2>&1 | tail -5
# 67 pinned rule ids
mise exec -- cargo test -p panini-prakriya tinanta_rule_order_is_pinned
# the pre-existing suite is byte-identical
git diff main --stat -- crates/panini/tests/paradigm.rs
```

The last command must show **only additions** for the 1296 pre-existing rows. A modified pre-existing line is a regression, not a re-pin — the spec's zero-delta claim is exact.

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "docs: svādi complete — 1512 goldens, six gaṇas, affix-relative aṅga

README's Scope section refreshed (it still said four gaṇas and predated
kryādi). AGENTS.md records that 7.3.84 and 1.2.4 each appear twice in
TINANTA_RULES by design."
```

---

## Self-Review Notes

Checked against the spec section by section:

- **Slice split** → Tasks 1, 2 (prep 1, prep 2), 3–12 (5a). 5b excluded with a stated reason.
- **Root selection** → Tasks 10, 11. All six roots, with the artha strings from the spec's table.
- **3.1.73** → Task 3. **7.3.84 second application** → Task 4. **6.4.87 / 6.4.77** → Task 6. **6.4.106** → Task 8.
- **Three existing rules assume the aṅga is the root** → 6.1.78 Task 5, 6.1.90 Task 9, 6.4.101 Task 7.
- **The asaṁyogapūrva predicate** → Task 6 Steps 1 and 4, with the enumerated membership test the spec asks for.
- **Data** (`Dhatu::id`, the √aś collision, `stiG`'s 6.1.64 note) → Tasks 2, 11.
- **Golden forms** → Tasks 10 Step 3, 11 Step 4; all 216 forms transcribed from the spec.
- **Testing** → trace pins in Tasks 10 Step 5 and 11 Step 6; the two no-delta tests for the second 7.3.84 in Task 4 Step 1; the gate in Task 12.
- **Success criteria** → Task 12 Step 6 checks the three that are mechanically checkable; the rest are covered by the tasks that produce them.

Naming is consistent across tasks: `shnu_asamyogapurva` (Task 6, used in Task 8), `sound_before_ending` (Task 7), `second_7_3_84` (Task 4), `guna::GUNA` (Task 1), `Dhatu::id` (Task 2, used in Tasks 10–11).

Two places where a step deliberately does not paste a literal value: the ordered traces in Tasks 10 and 11. Those values must come from running the engine — pasting a guessed sequence and calling it a golden is exactly the failure mode this repo has been bitten by. Each of those steps names the sūtras that must be present and their required relative order, so the run is *verified*, not merely recorded.
