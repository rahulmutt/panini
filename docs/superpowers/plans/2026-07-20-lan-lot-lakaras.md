# Laṅ and Loṭ Lakāras Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend the tiṅanta engine from one lakāra (laṭ) to three, adding laṅ (imperfect) and loṭ (imperative), by first making the dormant `Rule`/`run_pipeline` abstraction the real derivation path.

**Architecture:** `Prakriya` gains a `Context` carrying the derivation's lakāra/pada/puruṣa/vacana. Every step of today's straight-line `tinanta::derive` becomes a `Rule` with an explicit guard, and `derive` becomes "build the initial `Prakriya`, then `run_pipeline(&mut p, TINANTA_RULES)`". One ordered rule list serves all three lakāras; lakāra-specific rules guard on `ctx`.

**Tech Stack:** Rust (edition 2024, MSRV 1.97.1), cargo workspace, `mise` task runner, `cargo-mutants` for mutation testing.

## Global Constraints

- Toolchain is pinned via `mise`. Never install Rust globally. Use `mise run build | test | lint | fmt | fmt-check | mutants`.
- `#![forbid(unsafe_code)]` in every non-fuzz crate. Do not remove it.
- SLP1 is the only internal representation. No transliteration outside `panini-lipi`.
- Sūtra ids and names in traces **must** match the cited reference (ashtadhyayi.com). This is a hard gate, not a nicety.
- The golden paradigm test (`crates/panini/tests/paradigm.rs`) is the source of truth for surface forms. Grammar changes are gated by it.
- `crates/panini/tests/trace.rs` pins **exact ordered** sūtra sequences for three laṭ forms. These must keep passing byte-for-byte through Task 1's refactor.
- Run `mise run fmt` and `mise run lint` before every commit. Clippy warnings are denied.
- SLP1 is case-sensitive: `a` and `A`, `t` and `T` are different phonemes. Never lowercase or uppercase SLP1 text.

---

## File Structure

| File | Responsibility | Tasks |
|---|---|---|
| `crates/panini-prakriya/src/context.rs` | **New.** `Context` struct — the derivation's grammatical coordinates. | 1 |
| `crates/panini-prakriya/src/prakriya.rs` | Modify. `Prakriya` gains `ctx: Context`. | 1 |
| `crates/panini-prakriya/src/tinanta.rs` | Modify heavily. Becomes `TINANTA_RULES` (ordered `&[Rule]`) + a thin `derive`. | 1, 3, 4 |
| `crates/panini-prakriya/src/lib.rs` | Modify. Re-export `Context`. | 1 |
| `crates/panini-data/src/lib.rs` | Modify. `Lakara` gains `Lan`, `Lot`. | 3, 4 |
| `crates/panini-analyze/src/lib.rs` | Modify. Enumerate candidates over the lakāra axis. | 3, 4 |
| `crates/panini/src/lib.rs` | Modify. `Analysis` gains `lakara`. | 2 |
| `crates/panini-cli/src/main.rs` | Modify. Render the lakāra in text and JSON output. | 2 |
| `crates/panini/tests/paradigm.rs` | Modify. 54 → 162 golden forms; new negatives. | 3, 4 |
| `crates/panini/tests/trace.rs` | Modify. Add laṅ and loṭ ordered-trace cases. | 3, 4 |
| `crates/panini/tests/roundtrip.rs` | Modify. Extend over the lakāra axis. | 5 |
| `docs/ARCHITECTURE.md`, `README.md`, `AGENTS.md` | Modify. Reflect 3 lakāras and the pipeline. | 5 |

**Task dependency:** 1 → 2 → 3 → 4 → 5. Task 1 is a pure refactor; Tasks 3 and 4 each add one lakāra end-to-end.

---

### Task 1: Wire up the rule pipeline (behavior-identical refactor)

Convert `tinanta::derive` from a straight-line procedure into an ordered rule list executed by `run_pipeline`. **No behavior changes.** Success is defined entirely by existing tests continuing to pass.

**Files:**
- Create: `crates/panini-prakriya/src/context.rs`
- Modify: `crates/panini-prakriya/src/prakriya.rs`
- Modify: `crates/panini-prakriya/src/tinanta.rs` (full rewrite of the non-test half)
- Modify: `crates/panini-prakriya/src/lib.rs`
- Modify: `crates/panini-prakriya/src/controller.rs` (test fixture only)
- Modify: `crates/panini-prakriya/src/it_samjna.rs` (test fixtures only)

**Interfaces:**
- Consumes: `panini_data::{Dhatu, Lakara, Pada, Purusha, Vacana, tin_ending}`, `crate::rule::{Rule, RuleKind}`, `crate::controller::run_pipeline`, `crate::it_samjna::run_it_samjna`.
- Produces:
  - `panini_prakriya::Context { lakara: Lakara, pada: Pada, purusha: Purusha, vacana: Vacana, is_ngit_like: bool }` with `Context::new(lakara, pada, purusha, vacana) -> Context` and `impl Default`.
  - `Prakriya.ctx: Context` (public field).
  - `pub static TINANTA_RULES: &[Rule]` in `tinanta`.
  - `tinanta::derive(&Dhatu, Lakara, Pada, Purusha, Vacana) -> Prakriya` — signature unchanged, but the `lakara` parameter is now actually used (stored in `ctx`).

- [ ] **Step 1: Record the baseline**

Before changing anything, capture that the suite is green so you can tell a refactor regression from a pre-existing failure.

```bash
cd /workspace
mise run test 2>&1 | tail -20
```

Expected: all tests pass. Note the total test count.

- [ ] **Step 2: Create the `Context` type**

Create `crates/panini-prakriya/src/context.rs`:

```rust
use panini_data::{Lakara, Pada, Purusha, Vacana};

/// The grammatical coordinates of a derivation.
///
/// Rules in `TINANTA_RULES` self-guard on this rather than being selected into
/// per-lakāra lists, mirroring the Aṣṭādhyāyī: one ordered rule set whose rules
/// state their own conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Context {
    pub lakara: Lakara,
    pub pada: Pada,
    pub purusha: Purusha,
    pub vacana: Vacana,
    /// Whether ṅit-conditioned rules (3.4.99, 3.4.101) apply.
    ///
    /// True inherently for laṅ, which is ṅit by nature. For loṭ it is set at
    /// derivation time by 3.4.85 loṭo laṅvat, an *atideśa* — keeping that piece
    /// of grammar in the rule list where it appears in the trace, rather than
    /// hiding it in a match arm here.
    pub is_ngit_like: bool,
}

impl Context {
    pub fn new(lakara: Lakara, pada: Pada, purusha: Purusha, vacana: Vacana) -> Context {
        Context {
            lakara,
            pada,
            purusha,
            vacana,
            // Task 3 changes this to `matches!(lakara, Lakara::Lan)` once that
            // variant exists — laṅ is ṅit inherently. loṭ acquires it at
            // derivation time via rule 3.4.85 (Task 4).
            is_ngit_like: false,
        }
    }
}

impl Default for Context {
    /// A placeholder context, so `Prakriya::default()` keeps working for unit
    /// tests that build a `Prakriya` by hand and exercise no context-guarded
    /// rule. It is deliberately NOT `#[derive(Default)]` on the `Lakara` /
    /// `Pada` enums: there is no such thing as a "default lakāra" in the
    /// grammar, and claiming one in the public data API would be a lie.
    fn default() -> Context {
        Context::new(
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        )
    }
}
```

`is_ngit_like` starts as a hardcoded `false` because `Lakara::Lan` does not exist until Task 3, which replaces that line. The `lakara` parameter is therefore unused inside `Context::new` for now — it is still stored in the struct field, so this produces no warning.

- [ ] **Step 3: Add `ctx` to `Prakriya`**

In `crates/panini-prakriya/src/prakriya.rs`, change the struct and add the import:

```rust
use crate::context::Context;
use crate::term::Term;
```

```rust
#[derive(Debug, Clone, Default)]
pub struct Prakriya {
    pub terms: Vec<Term>,
    pub log: Vec<RuleStep>,
    pub ctx: Context,
}
```

Leave `text`, `snapshot`, and `record` unchanged.

- [ ] **Step 4: Export `Context` from the crate root**

In `crates/panini-prakriya/src/lib.rs`, add the module and re-export:

```rust
pub mod context;
```

(alongside the existing `pub mod controller;` etc., keeping alphabetical order — `context` goes before `controller`)

```rust
pub use context::Context;
```

(alongside the existing `pub use controller::run_pipeline;`)

- [ ] **Step 5: Fix the struct-literal test fixtures**

Three existing tests build `Prakriya` with a struct literal and will now fail to compile with "missing field `ctx`". Add `..Default::default()` to each.

In `crates/panini-prakriya/src/lib.rs`, in `mod tests`:

```rust
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("a"), Term::new("ti")],
            log: vec![],
            ..Default::default()
        };
```

In `crates/panini-prakriya/src/controller.rs`, in `mod tests`:

```rust
        let mut p = Prakriya {
            terms: vec![Term::new("Bo"), Term::new("a")],
            log: vec![],
            ..Default::default()
        };
```

In `crates/panini-prakriya/src/it_samjna.rs`, in `mod tests` — **both** tests:

```rust
        let mut p = Prakriya {
            terms: vec![Term::new("Sap")],
            log: vec![],
            ..Default::default()
        };
```

```rust
        let mut p = Prakriya {
            terms: vec![Term::new("tip")],
            log: vec![],
            ..Default::default()
        };
```

- [ ] **Step 6: Verify it compiles and the suite is still green**

```bash
cd /workspace
mise run test 2>&1 | tail -20
```

Expected: same pass count as Step 1. `Prakriya` now carries a context that nothing reads yet.

- [ ] **Step 7: Commit the scaffolding**

```bash
cd /workspace
mise run fmt && mise run lint
git add crates/panini-prakriya/src/
git commit -m "refactor(prakriya): add Context to Prakriya for rule self-guarding"
```

- [ ] **Step 8: Rewrite `tinanta.rs` as an ordered rule list**

Replace everything in `crates/panini-prakriya/src/tinanta.rs` **above** `#[cfg(test)]` with the following. The helper functions `guna_of`, `is_vowel`, and `is_vibhakti_protected_final` are carried over unchanged so the existing unit tests that call them still compile.

```rust
use crate::context::Context;
use crate::controller::run_pipeline;
use crate::it_samjna::run_it_samjna;
use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use panini_data::{Dhatu, Lakara, Pada, Purusha, Vacana, tin_ending};

/// Guṇa substitute of an ik vowel (1.1.2 aden guṇaḥ, applied by 7.3.84).
fn guna_of(v: char) -> Option<&'static str> {
    match v {
        'i' | 'I' => Some("e"),
        'u' | 'U' => Some("o"),
        'f' | 'F' => Some("ar"),
        'x' | 'X' => Some("al"),
        _ => None,
    }
}

fn is_vowel(c: char) -> bool {
    matches!(
        c,
        'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O'
    )
}

/// 1.3.4 na vibhaktau tusmāḥ: a final tu-varga (t/T/d/D/n), `s`, or `m` of a
/// vibhakti is NOT an it, so the shared halantyam elision must be suppressed
/// for such tiṅ endings (e.g. tas, Tas, vas, mas keep their final `s`).
fn is_vibhakti_protected_final(c: char) -> bool {
    matches!(c, 't' | 'T' | 'd' | 'D' | 'n' | 's' | 'm')
}

/// Index of the aṅga (the dhātu) in `terms`. Stable across the pipeline.
const ANGA: usize = 0;

/// Index of the tiṅ ending *before* śap is inserted (3.1.68).
const ENDING_PRE_SHAP: usize = 1;

/// Index of śap once inserted, and of the ending thereafter.
const SHAP: usize = 1;
const ENDING: usize = 2;

/// The ordered rule list. Read it top to bottom against the Aṣṭādhyāyī: this
/// sequence IS the grammar this crate implements. Every rule self-guards and
/// returns whether it fired.
pub static TINANTA_RULES: &[Rule] = &[
    // 3.4.78 tiptasjhi...: replace the lakāra by the tiṅ ending.
    // 3.4.113 tiṅ-śit sārvadhātukam makes it sārvadhātuka.
    Rule {
        id: "3.4.78",
        name: "tiptasjhisipthasthamipvasmas",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let before = p.snapshot();
            let ending = tin_ending(p.ctx.pada, p.ctx.purusha, p.ctx.vacana);
            let mut e = Term::new(ending);
            e.add(Tag::Tin);
            e.add(Tag::Sarvadhatuka);
            p.terms.push(e);
            p.record("3.4.78", "tiptasjhisipthasthamipvasmas", before);
            true
        },
    },
    // it-samjña on the tiṅ ending (1.3.3 halantyam / 1.3.9 tasya lopaḥ),
    // respecting 1.3.4: the final s/t/m of a vibhakti is protected, so only
    // endings whose final is a genuine anubandha (tip/sip/mip → the pit marker
    // `p`) are reduced.
    //
    // This MUST precede the lakāra-specific substitutions below: 3.4.100
    // itaś ca elides the `i` of `tip`, and that `i` is only exposed once
    // halantyam has stripped the `p`.
    Rule {
        id: "1.3.9",
        name: "tasya lopaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let last = p.terms[ENDING_PRE_SHAP].text.chars().last();
            if last.map(is_vibhakti_protected_final).unwrap_or(false) {
                return false;
            }
            let mut e = p.terms[ENDING_PRE_SHAP].clone();
            let original = e.text.clone();
            run_it_samjna(&mut e, p, ENDING_PRE_SHAP);
            p.terms[ENDING_PRE_SHAP] = e;
            p.terms[ENDING_PRE_SHAP].text != original
        },
    },
    // 3.1.68 kartari śap: insert śap between dhātu and ending, run it-samjña
    // on it (Sap → a), and mark the dhātu an aṅga.
    Rule {
        id: "3.1.68",
        name: "kartari Sap",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let before = p.snapshot();
            let mut s = Term::new("Sap");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.68", "kartari Sap", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP);
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 7.1.3 jho'ntaḥ: a leading `J` of the ending → `ant`.
    Rule {
        id: "7.1.3",
        name: "jho'ntaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ENDING].text.starts_with('J') {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("ant{rest}");
            p.record("7.1.3", "jho'ntaH", before);
            true
        },
    },
    // 7.3.84 sārvadhātukārdhadhātukayoḥ: guṇa of the aṅga's final ik.
    Rule {
        id: "7.3.84",
        name: "sArvadhAtukArdhadhAtukayoH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let last = p.terms[ANGA].text.chars().last().unwrap();
            let Some(g) = guna_of(last) else {
                return false;
            };
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ANGA].text.chars().collect();
            s.pop();
            p.terms[ANGA].text = s.into_iter().collect::<String>() + g;
            p.record("7.3.84", "sArvadhAtukArdhadhAtukayoH", before);
            true
        },
    },
    // 6.1.78 eco'yavāyāvaḥ: e/o/E/O before a vowel → ay/av/Ay/Av.
    Rule {
        id: "6.1.78",
        name: "eco'yavAyAvaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let anga_last = p.terms[ANGA].text.chars().last().unwrap();
            let next_first = p.terms[SHAP].text.chars().next().unwrap();
            let sub = match anga_last {
                'e' => "ay",
                'o' => "av",
                'E' => "Ay",
                'O' => "Av",
                _ => return false,
            };
            if !is_vowel(next_first) {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ANGA].text.chars().collect();
            s.pop();
            p.terms[ANGA].text = s.into_iter().collect::<String>() + sub;
            p.record("6.1.78", "eco'yavAyAvaH", before);
            true
        },
    },
    // 7.3.101 ato dīrgho yañi: aṅga-final `a` (śap) → `A` before a yañ-initial
    // sārvadhātuka ending (here: mi/vas/mas).
    Rule {
        id: "7.3.101",
        name: "ato dIrgho yaYi",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let ending_first = p.terms[ENDING].text.chars().next().unwrap();
            if !matches!(ending_first, 'm' | 'v') || p.terms[SHAP].text != "a" {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "A".into();
            p.record("7.3.101", "ato dIrgho yaYi", before);
            true
        },
    },
    // 6.1.97 ato guṇe: a short `a` (the śap) followed by a guṇa vowel yields
    // para-rūpa — a single vowel identical to the following one. For the `anti`
    // ending (Ji → anti), śap `a` + initial `a` of `anti` → a single short `a`
    // (NOT savarṇa-dīrgha `A`), so `Bav`+`a`+`nti` = `Bavanti`. Drop the
    // ending's leading `a`; the surviving śap `a` stands in for the coalesced
    // vowel and the term vector stays consistent for `.text()`.
    Rule {
        id: "6.1.97",
        name: "ato guRe",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms[SHAP].text != "a" || !p.terms[ENDING].text.starts_with('a') {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
            p.record("6.1.97", "ato guRe", before);
            true
        },
    },
    // 8.2.66 sasajuṣo ruḥ + 8.3.15 kharavasānayoḥ: word-final `s` → visarga.
    Rule {
        id: "8.3.15",
        name: "kharavasAnayoH visarjanIyaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.text().ends_with('s') {
                return false;
            }
            let before = p.snapshot();
            let idx = p.terms.len() - 1;
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            s.push('H');
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.3.15", "kharavasAnayoH visarjanIyaH", before);
            true
        },
    },
];

pub fn derive(
    dhatu: &Dhatu,
    lakara: Lakara,
    pada: Pada,
    purusha: Purusha,
    vacana: Vacana,
) -> Prakriya {
    let mut p = Prakriya {
        ctx: Context::new(lakara, pada, purusha, vacana),
        ..Default::default()
    };
    p.terms.push({
        let mut t = Term::new(dhatu.code);
        t.add(Tag::Dhatu);
        t
    });
    run_pipeline(&mut p, TINANTA_RULES);
    p
}
```

Two deliberate changes from v1, both behavior-preserving:
1. **`7.3.101` now precedes `6.1.97`.** No laṭ form fires both rules, so laṭ output is unchanged — verified by the trace tests in Step 9. Task 3 explains why the order is required for laṅ.
2. **`7.1.3` now matches a leading `J`** rather than the exact string `"Ji"`. For laṭ the only J-initial ending is `Ji`, so this is identical; laṅ (`J`) and loṭ (`Ju`) need the general form.

- [ ] **Step 9: Verify the refactor changed nothing**

```bash
cd /workspace
mise run test 2>&1 | tail -20
```

Expected: identical pass count to Step 1. **The three tests in `crates/panini/tests/trace.rs` are the critical ones** — they pin the exact ordered sūtra sequence, so if the rule list order or any `record` call drifted, they fail. If they do fail, the diff between expected and actual tells you exactly which rule moved.

- [ ] **Step 10: Confirm `run_pipeline` is on the real path**

The whole point of this task is that the abstraction is no longer dead code. Verify it directly:

```bash
cd /workspace
grep -n "run_pipeline" crates/panini-prakriya/src/tinanta.rs
```

Expected: a hit inside `derive` (not in a test module).

- [ ] **Step 11: Commit**

```bash
cd /workspace
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "refactor(prakriya): derive via ordered TINANTA_RULES pipeline"
```

---

### Task 2: Report the lakāra in analyses

`Analysis` reports puruṣa and vacana but not which lakāra matched. Add it now, while there is still only one lakāra and the change is trivially verifiable.

**Files:**
- Modify: `crates/panini/src/lib.rs`
- Modify: `crates/panini-cli/src/main.rs`
- Test: `crates/panini/src/lib.rs` (`mod tests`)

**Interfaces:**
- Consumes: `panini_data::Lakara`, `panini_analyze::Candidate` (has a `lakara` field already).
- Produces: `panini::Analysis.lakara: Lakara`, and `panini::lakara_name(Lakara) -> &'static str` for display.

- [ ] **Step 1: Write the failing test**

Add to `mod tests` in `crates/panini/src/lib.rs`:

```rust
    #[test]
    fn analysis_reports_its_lakara() {
        let engine = Panini::new();
        let r = engine.check("Bavati");
        let a = r.analyses.iter().find(|a| a.form_slp1 == "Bavati").unwrap();
        assert!(matches!(a.lakara, Lakara::Lat));
        assert_eq!(lakara_name(a.lakara), "laT");
    }
```

- [ ] **Step 2: Run it and confirm it fails**

```bash
cd /workspace
cargo test -p panini analysis_reports_its_lakara 2>&1 | tail -20
```

Expected: FAIL — compile error, `no field 'lakara' on type 'Analysis'` and `cannot find function 'lakara_name'`.

- [ ] **Step 3: Add the field and the display helper**

In `crates/panini/src/lib.rs`, add `lakara` to the struct:

```rust
pub struct Analysis {
    pub dhatu: String,
    pub lakara: Lakara,
    pub purusha: Purusha,
    pub vacana: Vacana,
    pub form_slp1: String,
    pub trace: Vec<RuleStep>,
}
```

Populate it in `check`, in the `analyses.push(...)` call:

```rust
                analyses.push(Analysis {
                    dhatu: c.dhatu.code.to_string(),
                    lakara: c.lakara,
                    purusha: c.purusha,
                    vacana: c.vacana,
                    form_slp1: p.text(),
                    trace: p.log,
                });
```

Add the display helper near `render` at the bottom of the file:

```rust
/// SLP1 name of a lakāra, for display in traces and CLI output.
pub fn lakara_name(lakara: Lakara) -> &'static str {
    match lakara {
        Lakara::Lat => "laT",
    }
}
```

- [ ] **Step 4: Run the test and confirm it passes**

```bash
cd /workspace
cargo test -p panini analysis_reports_its_lakara 2>&1 | tail -20
```

Expected: PASS.

- [ ] **Step 5: Surface the lakāra in CLI output**

In `crates/panini-cli/src/main.rs`, change the import:

```rust
use panini::{Panini, Verdict, lakara_name, render};
```

In the JSON branch, add the lakāra to each analysis object:

```rust
                    "analyses": result.analyses.iter().map(|a| serde_json::json!({
                        "dhatu": a.dhatu,
                        "lakara": lakara_name(a.lakara),
                        "form": render(&a.form_slp1, scheme),
                        "trace": a.trace.iter().map(|s| serde_json::json!({"sutra": s.sutra, "name": s.name, "after": s.after})).collect::<Vec<_>>(),
                    })).collect::<Vec<_>>(),
```

In the human-readable branch, include it in the headline:

```rust
                println!(
                    "VALID \u{2713}  {} ({}, {})",
                    render(&a.form_slp1, scheme),
                    a.dhatu,
                    lakara_name(a.lakara)
                );
```

- [ ] **Step 6: Verify the CLI output by eye**

```bash
cd /workspace
cargo run -q -p panini-cli -- check 'bhavati'
cargo run -q -p panini-cli -- check 'bhavati' --json | head -20
```

Expected: `VALID ✓  bhavati (BU, laT)` and a `"lakara": "laT"` key in the JSON.

- [ ] **Step 7: Run the full suite**

```bash
cd /workspace
mise run test 2>&1 | tail -20
```

Expected: all pass. `crates/panini-cli/tests/cli.rs` asserts on CLI output — if it matches the headline exactly it will need updating to the new format; if it only checks for `VALID` it passes unchanged. Read the failure and fix the assertion to match the new format if needed.

- [ ] **Step 8: Commit**

```bash
cd /workspace
mise run fmt && mise run lint
git add crates/panini/src/lib.rs crates/panini-cli/src/main.rs crates/panini-cli/tests/
git commit -m "feat(panini): report the matched lakara in Analysis and CLI output"
```

---

### Task 3: Laṅ (imperfect)

Add the first new lakāra end-to-end: enum variant, rules, analyzer coverage, 54 golden forms, ordered-trace test.

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini-prakriya/src/context.rs`
- Modify: `crates/panini-prakriya/src/tinanta.rs`
- Modify: `crates/panini-analyze/src/lib.rs`
- Modify: `crates/panini/src/lib.rs` (`lakara_name`)
- Test: `crates/panini/tests/paradigm.rs`, `crates/panini/tests/trace.rs`

**Interfaces:**
- Consumes: `Context.is_ngit_like`, `TINANTA_RULES`, `panini::lakara_name`.
- Produces: `Lakara::Lan`; `lakara_name(Lakara::Lan) == "laN"`; `panini_analyze::LAKARAS: &[Lakara]`.

- [ ] **Step 1: Verify the sūtras against the reference**

This is a hard gate from AGENTS.md, and it comes first because a wrong id poisons every trace below. Check each of these on ashtadhyayi.com and confirm both the number and the name:

| id | expected name | expected effect |
|---|---|---|
| 3.4.99 | nityaṃ ṅitaḥ | final `s` of a ṅit-lakāra tiṅ is elided (vas→va, mas→ma) |
| 3.4.100 | itaś ca | final `i` of a ṅit-lakāra tiṅ is elided (ti→t, si→s, Ji→J) |
| 3.4.101 | tasthasthamipāṃ tāṃtaṃtāmaḥ | tas→tAm, Tas→tam, Ta→ta, mip→am |
| 6.4.71 | luṅlaṅlṛṅkṣvaḍudāttaḥ | aṭ-āgama prefixed to the aṅga in laṅ |
| 8.2.23 | saṃyogāntasya lopaḥ | final consonant of a word-final conjunct is elided |

If any differs from the table, use the reference's version and note the correction in the commit message. Record the exact SLP1 transliteration of each name — that string goes in the `name` field and is what the trace prints.

- [ ] **Step 2: Write the failing golden test**

In `crates/panini/tests/paradigm.rs`, restructure the fixture to key on `(root, lakara)`. Replace the `PARADIGM` constant and the `every_form_validates_and_matches` test with:

```rust
use panini::{Panini, Verdict};

/// (root_code, lakara_label, [P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B]) in SLP1.
const PARADIGM: &[(&str, &str, [&str; 9])] = &[
    (
        "BU",
        "laT",
        [
            "Bavati", "BavataH", "Bavanti", "Bavasi", "BavaTaH", "BavaTa", "BavAmi", "BavAvaH",
            "BavAmaH",
        ],
    ),
    (
        "nI",
        "laT",
        [
            "nayati", "nayataH", "nayanti", "nayasi", "nayaTaH", "nayaTa", "nayAmi", "nayAvaH",
            "nayAmaH",
        ],
    ),
    (
        "ji",
        "laT",
        [
            "jayati", "jayataH", "jayanti", "jayasi", "jayaTaH", "jayaTa", "jayAmi", "jayAvaH",
            "jayAmaH",
        ],
    ),
    (
        "smf",
        "laT",
        [
            "smarati", "smarataH", "smaranti", "smarasi", "smaraTaH", "smaraTa", "smarAmi",
            "smarAvaH", "smarAmaH",
        ],
    ),
    (
        "paW",
        "laT",
        [
            "paWati", "paWataH", "paWanti", "paWasi", "paWaTaH", "paWaTa", "paWAmi", "paWAvaH",
            "paWAmaH",
        ],
    ),
    (
        "vad",
        "laT",
        [
            "vadati", "vadataH", "vadanti", "vadasi", "vadaTaH", "vadaTa", "vadAmi", "vadAvaH",
            "vadAmaH",
        ],
    ),
    (
        "BU",
        "laN",
        [
            "aBavat", "aBavatAm", "aBavan", "aBavaH", "aBavatam", "aBavata", "aBavam", "aBavAva",
            "aBavAma",
        ],
    ),
    (
        "nI",
        "laN",
        [
            "anayat", "anayatAm", "anayan", "anayaH", "anayatam", "anayata", "anayam", "anayAva",
            "anayAma",
        ],
    ),
    (
        "ji",
        "laN",
        [
            "ajayat", "ajayatAm", "ajayan", "ajayaH", "ajayatam", "ajayata", "ajayam", "ajayAva",
            "ajayAma",
        ],
    ),
    (
        "smf",
        "laN",
        [
            "asmarat", "asmaratAm", "asmaran", "asmaraH", "asmaratam", "asmarata", "asmaram",
            "asmarAva", "asmarAma",
        ],
    ),
    (
        "paW",
        "laN",
        [
            "apaWat", "apaWatAm", "apaWan", "apaWaH", "apaWatam", "apaWata", "apaWam", "apaWAva",
            "apaWAma",
        ],
    ),
    (
        "vad",
        "laN",
        [
            "avadat", "avadatAm", "avadan", "avadaH", "avadatam", "avadata", "avadam", "avadAva",
            "avadAma",
        ],
    ),
];

#[test]
fn every_form_validates_and_matches() {
    let engine = Panini::new();
    for (root, lakara, forms) in PARADIGM {
        for expected in forms {
            let r = engine.check(expected);
            assert!(
                matches!(r.verdict, Verdict::Valid),
                "expected VALID for {expected} ({root} {lakara})"
            );
            assert!(
                r.analyses.iter().any(|a| a.form_slp1 == *expected
                    && a.dhatu == *root
                    && panini::lakara_name(a.lakara) == *lakara),
                "no {lakara} analysis of {root} produced {expected}"
            );
        }
    }
}
```

Note this assertion is strictly stronger than v1's: it now requires the analysis to come from the *right root and right lakāra*, not merely to produce the right string.

Also update the negatives test:

```rust
#[test]
fn known_nonforms_are_invalid() {
    let engine = Panini::new();
    for bad in [
        // Real cross-lakāra confusions, not junk: laṅ endings require the
        // aṭ-āgama (6.4.71), and laṭ endings forbid it.
        "Bavat",     // laṅ 3sg ending without the augment
        "aBavanti",  // augment on a laṭ form
        // Still out of scope entirely.
        "gacCati", "Bavati123", "tiRRati",
    ] {
        assert!(
            matches!(engine.check(bad).verdict, Verdict::Invalid),
            "expected INVALID for {bad}"
        );
    }
}
```

`Bavatu` is deliberately removed from this list — it becomes VALID in Task 4.

- [ ] **Step 3: Run it and confirm it fails**

```bash
cd /workspace
cargo test -p panini --test paradigm 2>&1 | tail -30
```

Expected: FAIL. `every_form_validates_and_matches` fails on `aBavat` with "expected VALID".

- [ ] **Step 4: Add the `Lan` variant**

In `crates/panini-data/src/lib.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lakara {
    Lat,
    Lan,
}
```

In `crates/panini/src/lib.rs`, extend the display helper:

```rust
pub fn lakara_name(lakara: Lakara) -> &'static str {
    match lakara {
        Lakara::Lat => "laT",
        Lakara::Lan => "laN",
    }
}
```

In `crates/panini-prakriya/src/context.rs`, replace the placeholder from Task 1 Step 2:

```rust
            // laṅ is ṅit inherently; loṭ acquires it via rule 3.4.85.
            is_ngit_like: matches!(lakara, Lakara::Lan),
```

- [ ] **Step 5: Make the analyzer enumerate lakāras**

In `crates/panini-analyze/src/lib.rs`, add the axis and use it:

```rust
/// The lakāras this build can derive. The analyzer proposes every
/// (root × lakāra × cell); the engine confirms by exact surface match.
pub const LAKARAS: &[Lakara] = &[Lakara::Lat, Lakara::Lan];
```

```rust
pub fn candidates(surface_slp1: &str) -> Vec<Candidate> {
    let mut out = Vec::new();
    for d in dhatus() {
        for &lakara in LAKARAS {
            for &(purusha, vacana) in CELLS {
                out.push(Candidate {
                    dhatu: d,
                    lakara,
                    pada: Pada::Parasmaipada,
                    purusha,
                    vacana,
                });
            }
        }
    }
    // Return the full (still tiny) candidate set; the engine confirms by exact match.
    let _ = surface_slp1;
    out
}
```

- [ ] **Step 6: Add the laṅ rules**

In `crates/panini-prakriya/src/tinanta.rs`, insert these into `TINANTA_RULES` **immediately after the `1.3.9` it-samjña rule and before `3.1.68 kartari śap`**:

```rust
    // 3.4.99 nityaṃ ṅitaḥ: the final `s` of a ṅit-lakāra's tiṅ is elided.
    // vas → va, mas → ma.
    Rule {
        id: "3.4.99",
        name: "nityaM Gitax",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.ctx.is_ngit_like || !p.terms[ENDING_PRE_SHAP].text.ends_with('s') {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ENDING_PRE_SHAP].text.chars().collect();
            s.pop();
            p.terms[ENDING_PRE_SHAP].text = s.into_iter().collect();
            p.record("3.4.99", "nityaM Gitax", before);
            true
        },
    },
    // 3.4.101 tasthasthamipāṃ tāṃtaṃtāmaḥ: tas→tAm, Tas→tam, Ta→ta, mip→am.
    //
    // The mip→am arm is laṅ-only: loṭ's uttama-eka is `ni` by the more specific
    // 3.4.89 mer niḥ, so it must not be captured here.
    Rule {
        id: "3.4.101",
        name: "tasTasTamipAM tAMtantAmaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.ctx.is_ngit_like {
                return false;
            }
            let sub = match p.terms[ENDING_PRE_SHAP].text.as_str() {
                "tas" => "tAm",
                "Tas" => "tam",
                "Ta" => "ta",
                "mi" if matches!(p.ctx.lakara, Lakara::Lan) => "am",
                _ => return false,
            };
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = sub.into();
            p.record("3.4.101", "tasTasTamipAM tAMtantAmaH", before);
            true
        },
    },
    // 3.4.100 itaś ca: the final `i` of a ṅit-lakāra's tiṅ is elided.
    // ti → t, si → s, Ji → J.
    Rule {
        id: "3.4.100",
        name: "itaS ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lan) || !p.terms[ENDING_PRE_SHAP].text.ends_with('i')
            {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ENDING_PRE_SHAP].text.chars().collect();
            s.pop();
            p.terms[ENDING_PRE_SHAP].text = s.into_iter().collect();
            p.record("3.4.100", "itaS ca", before);
            true
        },
    },
```

Insert this **immediately after the `3.1.68 kartari śap` rule** (the aṭ-āgama attaches to the aṅga, which 3.1.68 is what marks):

```rust
    // 6.4.71 luṅlaṅlṛṅkṣvaḍudāttaḥ: the aṭ-āgama is prefixed to the aṅga in laṅ.
    //
    // Modelled as a prefix on the aṅga's text rather than as a separate term,
    // so the ANGA/SHAP/ENDING indices stay stable for every later rule. The
    // trace still cites 6.4.71, which is what the reader checks.
    Rule {
        id: "6.4.71",
        name: "luGlaGlfGkzvaqudAttaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lan) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = format!("a{}", p.terms[ANGA].text);
            p.record("6.4.71", "luGlaGlfGkzvaqudAttaH", before);
            true
        },
    },
```

Insert this **immediately before the `8.3.15` visarga rule**:

```rust
    // 8.2.23 saṃyogāntasya lopaḥ: the final consonant of a word-final conjunct
    // is elided. aBavant → aBavan.
    Rule {
        id: "8.2.23",
        name: "saMyogAntasya lopaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let word = p.text();
            let mut tail = word.chars().rev();
            let (Some(last), Some(prev)) = (tail.next(), tail.next()) else {
                return false;
            };
            if is_vowel(last) || is_vowel(prev) {
                return false;
            }
            let before = p.snapshot();
            let idx = p.terms.len() - 1;
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.2.23", "saMyogAntasya lopaH", before);
            true
        },
    },
```

- [ ] **Step 7: Run the golden test**

```bash
cd /workspace
cargo test -p panini --test paradigm 2>&1 | tail -30
```

Expected: PASS, all 108 forms (54 laṭ + 54 laṅ).

If a form is wrong, the assertion message names it. Work that single form through the rule list by hand before changing any rule — the ordering constraints documented in the spec are the usual culprit.

- [ ] **Step 8: Pin the laṅ trace**

Add to `crates/panini/tests/trace.rs`:

```rust
#[test]
fn abhavat_trace_is_exactly_the_lan_augment_path() {
    // BU laṅ prathama eka: tip -> ti (1.3.9) -> t (3.4.100), aṭ-āgama (6.4.71).
    assert_eq!(
        trace_for("aBavat"),
        vec![
            "3.4.78", "1.3.9", "3.4.100", "3.1.68", "1.3.9", "6.4.71", "7.3.84", "6.1.78"
        ]
    );
}

#[test]
fn abhavan_trace_is_exactly_the_samyoganta_path() {
    // BU laṅ prathama bahu: Ji -> J (3.4.100) -> ant (7.1.3), then 6.1.97
    // coalescence and 8.2.23 conjunct-final elision: aBavant -> aBavan.
    assert_eq!(
        trace_for("aBavan"),
        vec![
            "3.4.78", "3.4.100", "3.1.68", "1.3.9", "6.4.71", "7.1.3", "7.3.84", "6.1.78",
            "6.1.97", "8.2.23"
        ]
    );
}

#[test]
fn abhavam_trace_shows_dirgha_does_not_fire() {
    // BU laṅ uttama eka: mip -> mi (1.3.9) -> am (3.4.101). The ending begins
    // with a vowel, so 7.3.101 ato dIrgho yaYi must NOT fire — this is why
    // 7.3.101 is ordered before 6.1.97, which strips that leading `a`.
    let trace = trace_for("aBavam");
    assert!(!trace.contains(&"7.3.101".to_string()), "got {trace:?}");
    assert!(trace.contains(&"6.1.97".to_string()), "got {trace:?}");
}
```

The exact sequences above are predictions from the rule list. If a run disagrees, verify by hand which is right before editing either side — a passing test asserting a wrong order is worse than a failing one.

- [ ] **Step 9: Run the full suite**

```bash
cd /workspace
mise run test 2>&1 | tail -20
```

Expected: all pass.

- [ ] **Step 10: Spot-check the CLI**

```bash
cd /workspace
cargo run -q -p panini-cli -- check 'abhavat' --trace
```

Expected: `VALID ✓  abhavat (BU, laN)` followed by a trace including `6.4.71`.

- [ ] **Step 11: Commit**

```bash
cd /workspace
mise run fmt && mise run lint
git add crates/ 
git commit -m "feat(prakriya): laG (imperfect) via aT-Agama and Git-conditioned tiG substitutions"
```

---

### Task 4: Loṭ (imperative)

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini-prakriya/src/tinanta.rs`
- Modify: `crates/panini-analyze/src/lib.rs`
- Modify: `crates/panini/src/lib.rs` (`lakara_name`)
- Test: `crates/panini/tests/paradigm.rs`, `crates/panini/tests/trace.rs`

**Interfaces:**
- Consumes: everything from Task 3.
- Produces: `Lakara::Lot`; `lakara_name(Lakara::Lot) == "loT"`.

- [ ] **Step 1: Verify the sūtras against the reference**

Same hard gate as Task 3 Step 1. Check on ashtadhyayi.com:

| id | expected name | expected effect |
|---|---|---|
| 3.4.85 | loṭo laṅvat | loṭ behaves as laṅ (so 3.4.99 and 3.4.101 apply) |
| 3.4.86 | er uḥ | final `i` of the tiṅ → `u` (ti→tu, Ji→Ju) |
| 3.4.87 | ser hyapic ca | sip → hi, and it is apit |
| 3.4.89 | mer niḥ | mip → ni |
| 3.4.92 | āḍ uttamasya pic ca | āṭ-āgama prefixed to uttama endings |
| 6.1.101 | akaḥ savarṇe dīrghaḥ | a + A → A |
| 6.4.105 | ato heḥ | `hi` is elided after `a` |

- [ ] **Step 2: Write the failing golden test**

Append to `PARADIGM` in `crates/panini/tests/paradigm.rs`:

```rust
    (
        "BU",
        "loT",
        [
            "Bavatu", "BavatAm", "Bavantu", "Bava", "Bavatam", "Bavata", "BavAni", "BavAva",
            "BavAma",
        ],
    ),
    (
        "nI",
        "loT",
        [
            "nayatu", "nayatAm", "nayantu", "naya", "nayatam", "nayata", "nayAni", "nayAva",
            "nayAma",
        ],
    ),
    (
        "ji",
        "loT",
        [
            "jayatu", "jayatAm", "jayantu", "jaya", "jayatam", "jayata", "jayAni", "jayAva",
            "jayAma",
        ],
    ),
    (
        "smf",
        "loT",
        [
            "smaratu", "smaratAm", "smarantu", "smara", "smaratam", "smarata", "smarAni",
            "smarAva", "smarAma",
        ],
    ),
    (
        "paW",
        "loT",
        [
            "paWatu", "paWatAm", "paWantu", "paWa", "paWatam", "paWata", "paWAni", "paWAva",
            "paWAma",
        ],
    ),
    (
        "vad",
        "loT",
        [
            "vadatu", "vadatAm", "vadantu", "vada", "vadatam", "vadata", "vadAni", "vadAva",
            "vadAma",
        ],
    ),
```

Add one more negative to `known_nonforms_are_invalid` — the aṭ-āgama is laṅ-only, so it must not combine with a loṭ ending:

```rust
        "aBavatu",   // augment on a loṭ form
```

- [ ] **Step 3: Run it and confirm it fails**

```bash
cd /workspace
cargo test -p panini --test paradigm 2>&1 | tail -30
```

Expected: FAIL on `Bavatu` with "expected VALID".

- [ ] **Step 4: Add the `Lot` variant and analyzer coverage**

In `crates/panini-data/src/lib.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lakara {
    Lat,
    Lan,
    Lot,
}
```

In `crates/panini/src/lib.rs`:

```rust
pub fn lakara_name(lakara: Lakara) -> &'static str {
    match lakara {
        Lakara::Lat => "laT",
        Lakara::Lan => "laN",
        Lakara::Lot => "loT",
    }
}
```

In `crates/panini-analyze/src/lib.rs`:

```rust
pub const LAKARAS: &[Lakara] = &[Lakara::Lat, Lakara::Lan, Lakara::Lot];
```

- [ ] **Step 5: Add the loṭ ending-substitution rules**

In `crates/panini-prakriya/src/tinanta.rs`, insert this **immediately after the `1.3.9` it-samjña rule and before `3.4.99`** — it must set the flag that 3.4.99 and 3.4.101 read:

```rust
    // 3.4.85 loṭo laṅvat: loṭ behaves as laṅ, so the ṅit-conditioned rules
    // (3.4.99, 3.4.101) apply to it. An atideśa, so it is a rule and appears
    // in the trace rather than being folded into Context::new.
    Rule {
        id: "3.4.85",
        name: "loTo laGvat",
        kind: RuleKind::Atidesha,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) || p.ctx.is_ngit_like {
                return false;
            }
            let before = p.snapshot();
            p.ctx.is_ngit_like = true;
            p.record("3.4.85", "loTo laGvat", before);
            true
        },
    },
```

`RuleKind` has no `Atidesha` variant yet. Add it in `crates/panini-prakriya/src/rule.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleKind {
    Vidhi,
    Samjna,
    Adhikara,
    Paribhasha,
    Atidesha,
}
```

Insert the next three **after `3.4.101` and before `3.4.100`**. Order within this group is load-bearing: 3.4.87 and 3.4.89 are *apavāda* to 3.4.86 and must precede it.

```rust
    // 3.4.87 ser hyapic ca: loṭ madhyama-eka `si` → `hi`.
    // Apavāda to 3.4.86 er uḥ, hence ordered before it.
    Rule {
        id: "3.4.87",
        name: "ser hyapic ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) || p.terms[ENDING_PRE_SHAP].text != "si" {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "hi".into();
            p.record("3.4.87", "ser hyapic ca", before);
            true
        },
    },
    // 3.4.89 mer niḥ: loṭ uttama-eka `mi` → `ni`.
    // Apavāda to 3.4.86 er uḥ, hence ordered before it.
    Rule {
        id: "3.4.89",
        name: "mer niH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) || p.terms[ENDING_PRE_SHAP].text != "mi" {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "ni".into();
            p.record("3.4.89", "mer niH", before);
            true
        },
    },
    // 3.4.86 er uḥ: the final `i` of the tiṅ → `u`. ti → tu, Ji → Ju.
    //
    // Guarded to exactly `ti`/`Ji` rather than "any i-final ending": `si` and
    // `mi` are preempted by the apavādas above, and by this point they have
    // already become `hi`/`ni`, which are also i-final. The explicit set makes
    // the preemption independent of ordering accidents.
    Rule {
        id: "3.4.86",
        name: "er uH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) {
                return false;
            }
            if !matches!(p.terms[ENDING_PRE_SHAP].text.as_str(), "ti" | "Ji") {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ENDING_PRE_SHAP].text.chars().collect();
            s.pop();
            s.push('u');
            p.terms[ENDING_PRE_SHAP].text = s.into_iter().collect();
            p.record("3.4.86", "er uH", before);
            true
        },
    },
```

Insert this **after `3.4.100` and before `3.1.68`**:

```rust
    // 3.4.92 āḍ uttamasya pic ca: the āṭ-āgama is prefixed to loṭ's uttama
    // endings. ni → Ani, va → Ava, ma → Ama.
    Rule {
        id: "3.4.92",
        name: "Aq uttamasya pic ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot)
                || !matches!(p.ctx.purusha, Purusha::Uttama)
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = format!("A{}", p.terms[ENDING_PRE_SHAP].text);
            p.record("3.4.92", "Aq uttamasya pic ca", before);
            true
        },
    },
```

- [ ] **Step 6: Restrict 7.3.101 and add the two loṭ sandhi rules**

`7.3.101 ato dīrgho yañi` must not fire for loṭ — its uttama forms get their long vowel from 3.4.92 plus savarṇa-dīrgha instead, and letting both fire would double-lengthen. Add the guard to the existing rule:

```rust
        apply: |p| {
            // loṭ uttama gets its dīrgha from 3.4.92 āḍ + 6.1.101 instead.
            if matches!(p.ctx.lakara, Lakara::Lot) {
                return false;
            }
            let ending_first = p.terms[ENDING].text.chars().next().unwrap();
            if !matches!(ending_first, 'm' | 'v') || p.terms[SHAP].text != "a" {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "A".into();
            p.record("7.3.101", "ato dIrgho yaYi", before);
            true
        },
```

Insert `6.1.101` **immediately after `7.3.101` and before `6.1.97`**:

```rust
    // 6.1.101 akaḥ savarṇe dīrghaḥ: śap `a` + the ending's initial `A`
    // (from 3.4.92 āḍ) coalesce to a single `A`. Bav + a + Ani → BavAni.
    Rule {
        id: "6.1.101",
        name: "akaH savarRe dIrghaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms[SHAP].text != "a" || !p.terms[ENDING].text.starts_with('A') {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "A".into();
            p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
            p.record("6.1.101", "akaH savarRe dIrghaH", before);
            true
        },
    },
```

Insert `6.4.105` **immediately after `6.1.97` and before `8.2.23`**:

```rust
    // 6.4.105 ato heḥ: `hi` is elided after a short `a` (the śap).
    // Bav + a + hi → Bava.
    Rule {
        id: "6.4.105",
        name: "ato heH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms[SHAP].text != "a" || p.terms[ENDING].text != "hi" {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = String::new();
            p.record("6.4.105", "ato heH", before);
            true
        },
    },
```

- [ ] **Step 7: Run the golden test**

```bash
cd /workspace
cargo test -p panini --test paradigm 2>&1 | tail -30
```

Expected: PASS, all 162 forms.

- [ ] **Step 8: Pin the loṭ traces**

Add to `crates/panini/tests/trace.rs`:

```rust
#[test]
fn bhavatu_trace_is_exactly_the_lot_er_uh_path() {
    // BU loṭ prathama eka: tip -> ti (1.3.9) -> tu (3.4.86), via 3.4.85.
    assert_eq!(
        trace_for("Bavatu"),
        vec![
            "3.4.78", "1.3.9", "3.4.85", "3.4.86", "3.1.68", "1.3.9", "7.3.84", "6.1.78"
        ]
    );
}

#[test]
fn bhava_trace_shows_hi_elision() {
    // BU loṭ madhyama eka: sip -> si -> hi (3.4.87), elided by 6.4.105.
    let trace = trace_for("Bava");
    assert!(trace.contains(&"3.4.87".to_string()), "got {trace:?}");
    assert!(trace.contains(&"6.4.105".to_string()), "got {trace:?}");
}

#[test]
fn bhavani_trace_shows_aat_not_dirgha() {
    // BU loṭ uttama eka: mip -> mi -> ni (3.4.89) -> Ani (3.4.92), then
    // 6.1.101 savarṇa-dīrgha. 7.3.101 must NOT fire, or the vowel would be
    // lengthened twice.
    let trace = trace_for("BavAni");
    assert!(trace.contains(&"3.4.89".to_string()), "got {trace:?}");
    assert!(trace.contains(&"3.4.92".to_string()), "got {trace:?}");
    assert!(trace.contains(&"6.1.101".to_string()), "got {trace:?}");
    assert!(!trace.contains(&"7.3.101".to_string()), "got {trace:?}");
}
```

- [ ] **Step 9: Run the full suite**

```bash
cd /workspace
mise run test 2>&1 | tail -20
```

Expected: all pass.

- [ ] **Step 10: Spot-check the CLI**

```bash
cd /workspace
cargo run -q -p panini-cli -- check 'bhavatu' --trace
cargo run -q -p panini-cli -- check 'bhavāni' --trace
```

Expected: both `VALID ✓ ... (BU, loT)` with traces citing `3.4.86` and `3.4.92`/`6.1.101` respectively.

- [ ] **Step 11: Commit**

```bash
cd /workspace
mise run fmt && mise run lint
git add crates/
git commit -m "feat(prakriya): loT (imperative) via laGvat atidesa, er uH, Aq-Agama"
```

---

### Task 5: Roundtrip, mutation gate, and docs

**Files:**
- Modify: `crates/panini/tests/roundtrip.rs`
- Modify: `docs/ARCHITECTURE.md`, `README.md`, `AGENTS.md`

**Interfaces:**
- Consumes: `panini_analyze::LAKARAS`, `panini::lakara_name`.
- Produces: nothing new.

- [ ] **Step 1: Extend the roundtrip test over the lakāra axis**

In `crates/panini/tests/roundtrip.rs`, replace `generate_then_check_recovers_inputs`:

```rust
#[test]
fn generate_then_check_recovers_inputs() {
    let engine = Panini::new();
    for d in dhatus() {
        for &lakara in panini_analyze::LAKARAS {
            for &(pu, va) in CELLS {
                let form = engine.derive(d, lakara, Pada::Parasmaipada, pu, va).text();
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
            }
        }
    }
}
```

Remove the now-unused `Lakara` import if the compiler warns; add `panini-analyze` to `crates/panini/Cargo.toml` under `[dev-dependencies]` if it is not already a dependency:

```toml
[dev-dependencies]
panini-analyze = { path = "../panini-analyze" }
```

- [ ] **Step 2: Run it**

```bash
cd /workspace
cargo test -p panini --test roundtrip 2>&1 | tail -20
```

Expected: PASS, 162 roundtrips.

- [ ] **Step 3: Run the mutation gate**

This is the real check on the new guards: with ~20 guarded rules, the likely defect is a dropped or inverted guard, and that is exactly what a surviving mutant exposes.

```bash
cd /workspace
MISE_ENV=dev mise install
mise run mutants 2>&1 | tail -40
```

Expected: no surviving mutants in the rule guards. For each survivor, add a test that kills it — typically a form that exercises the guard's false branch. Do not delete the mutant or weaken the rule to make it pass.

- [ ] **Step 4: Update the docs**

In `README.md`, replace the `## v1 scope` section:

```markdown
## Scope

Finite verbs (*tiṅanta*), *bhvādi* (gaṇa 1), *parasmaipada*, over a curated
6-root set, in three lakāras: *laṭ* (present), *laṅ* (imperfect), and *loṭ*
(imperative). `INVALID` means "not derivable within this covered grammar," not
"ungrammatical in Sanskrit." See `docs/ARCHITECTURE.md`.
```

In `docs/ARCHITECTURE.md`, replace the `panini-prakriya` bullet and add a section:

```markdown
- `panini-prakriya` — the engine: `Term`/`Prakriya`/`Context` model, it-samjna,
  and `TINANTA_RULES`, the ordered rule list that `tinanta::derive` runs via
  `run_pipeline`. Pure SLP1, no I/O.
```

```markdown
## The rule pipeline

`TINANTA_RULES` (in `crates/panini-prakriya/src/tinanta.rs`) is a single
ordered `&[Rule]` covering all three lakāras. Each rule self-guards on
`Prakriya.ctx` (lakāra, pada, puruṣa, vacana) and returns whether it fired.
Reading the list top to bottom IS reading the grammar this crate implements.

Rule order is load-bearing and several orderings are non-obvious; the
constraints and their justifications are documented in
`docs/superpowers/specs/2026-07-20-lan-lot-lakaras-design.md`. The exact
ordered traces in `crates/panini/tests/trace.rs` are what pin them.
```

In `AGENTS.md`, update the grammar-changes rule:

```markdown
- Grammar changes are gated by the golden paradigm test
  (`crates/panini/tests/paradigm.rs`, 162 forms across laṭ/laṅ/loṭ) and by the
  ordered-trace test (`crates/panini/tests/trace.rs`), which pins rule order.
  Surface forms and trace order there are the source of truth; sūtra ids/names
  in traces must match the cited reference (ashtadhyayi.com).
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, not as a
  branch inside `derive`.
```

- [ ] **Step 5: Verify onboarding is still truthful**

Run the sequence a new contributor would, exactly as the docs describe it:

```bash
cd /workspace
mise install
mise run build
mise run test
cargo run -q -p panini-cli -- check 'bhavati' --trace
cargo run -q -p panini-cli -- check 'abhavat' --trace
cargo run -q -p panini-cli -- check 'bhavatu' --trace
```

Expected: all succeed; the three checks print `VALID ✓` with `laT`, `laN`, `loT` respectively. If any command contradicts the docs, fix the docs.

- [ ] **Step 6: Commit**

```bash
cd /workspace
mise run fmt && mise run lint
git add crates/ docs/ README.md AGENTS.md
git commit -m "test: roundtrip over the lakara axis; docs: three-lakara scope and rule pipeline"
```

---

## Self-Review Notes

**Spec coverage.** Context on Prakriya → Task 1 Steps 2–3. `TINANTA_RULES` single ordered list → Task 1 Step 8. All 21 spec rules → Task 1 Step 8 (the 9 carried over from v1), Task 3 Step 6 (3.4.99, 3.4.100, 3.4.101, 6.4.71, 8.2.23), Task 4 Steps 5–6 (3.4.85, 3.4.86, 3.4.87, 3.4.89, 3.4.92, 6.1.101, 6.4.105). `Lakara` variants → Tasks 3/4 Step 4. `tin_ending` and `data/tin.tsv` unchanged → no task touches them, as the spec requires. Analyzer over lakāra axis → Task 3 Step 5, Task 4 Step 4. `Analysis.lakara` → Task 2. CLI rendering → Task 2 Step 5. 162 golden forms → Tasks 3/4 Step 2. Changed negatives incl. `Bavatu` → Task 3 Step 2 (removed), Task 4 Step 2 (`aBavatu` added). Roundtrip over lakāras → Task 5 Step 1. Mutation gate → Task 5 Step 3. Trace tests for lakāra-specific sūtras → Tasks 3/4 Step 8. Reference verification → Tasks 3/4 Step 1. Docs → Task 5 Step 4. **No spec requirement is unaddressed.**

**Ordering constraints from the spec, mapped to where they are enforced:** it-samjña before the substitutions → Task 1 Step 8 (the `1.3.9` rule sits second) with the requirement restated in its comment; 3.4.87/3.4.89 before 3.4.86 → Task 4 Step 5 insertion point plus the explicit `"ti" | "Ji"` guard; 7.3.101 before 6.1.97 → Task 1 Step 8, tested by `abhavam_trace_shows_dirgha_does_not_fire`; 3.4.101's mip arm laṅ-only → Task 3 Step 6 guard; loṭ excluded from 7.3.101 → Task 4 Step 6, tested by `bhavani_trace_shows_aat_not_dirgha`.

**Type consistency.** `Context::new(lakara, pada, purusha, vacana)` — defined Task 1 Step 2, called Task 1 Step 8. `is_ngit_like` — set in `Context::new` (Task 3 Step 4), mutated by 3.4.85 (Task 4 Step 5), read by 3.4.99/3.4.101 (Task 3 Step 6). `lakara_name` — defined Task 2 Step 3, extended Tasks 3/4 Step 4, called in Tasks 2/3/5. `LAKARAS` — defined Task 3 Step 5, extended Task 4 Step 4, consumed Task 5 Step 1. Index constants `ANGA`/`SHAP`/`ENDING`/`ENDING_PRE_SHAP` — defined Task 1 Step 8, used consistently in every rule added in Tasks 3 and 4. `RuleKind::Atidesha` — added Task 4 Step 5 before its first use in the same step.

**Known risk carried from the spec.** The sūtra ids, names, and the exact ordered traces in Tasks 3 and 4 are reconstructions. Step 1 of each task is the reference check, and it is placed first precisely so a correction propagates before the traces are pinned. If a trace assertion disagrees with a run, verify by hand which side is wrong before editing either — a passing test asserting a wrong rule order is worse than a failing one.
