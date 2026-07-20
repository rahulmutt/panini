# Vidhiliṅ Lakāra Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the vidhiliṅ (optative) lakāra to the tiṅanta engine — bhavet, bhavetām, bhaveyuḥ… — growing golden coverage from 162 to 216 forms.

**Architecture:** Six new self-guarding `Rule`s in the ordered `TINANTA_RULES` array plus two guard widenings (3.4.100, 3.4.101) from laṅ-only to ṅit-based. The yāsuṭ-āgama is a text prefix on the ending term (āṭ/aṭ precedent), so the fixed ANGA/SHAP/ENDING term indices never move. Spec: `docs/superpowers/specs/2026-07-20-vidhilin-lakara-design.md`.

**Tech Stack:** Rust (pinned via `mise`, rust 1.97.1). Run everything through `mise run <task>`, never bare cargo. Test with `mise run test`, format with `mise run fmt`, lint with `mise run lint`.

## Global Constraints

- SLP1 is the only internal representation; transliterate only in `panini-lipi`.
- `#![forbid(unsafe_code)]` in every non-fuzz crate.
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, not as a branch inside `derive`.
- Sūtra ids/names in traces must match ashtadhyayi.com (Task 1 verifies them; the trace-name gate test and the SLP1-digraph test in `tinanta.rs` enforce them mechanically).
- Grammar changes are gated by `crates/panini/tests/paradigm.rs` (golden forms) and `crates/panini/tests/trace.rs` (pinned rule order). The 162 existing golden forms must pass after every task.
- Rules ordered BEFORE 3.1.68 in `TINANTA_RULES` address the ending as `ENDING_PRE_SHAP` (index 1); rules AFTER it use `ENDING` (index 2) and `SHAP` (index 1). See the boundary comment in `tinanta.rs`.
- Commit after every task (steps say when).

## Domain primer (read this first)

You are adding the Sanskrit optative. SLP1 is an ASCII encoding of Sanskrit where capital letters are distinct phonemes: `A`=ā, `B`=bh, `T`=th, `W`=ṭh, `w`=ṭ, `z`=ṣ, `N`=ṅ, `R`=ṇ, `J`=jh, `E`=ai, `H`=visarga. So "Bavet" is *bhavet*. A derivation starts from a root (`BU`) plus an abstract tense marker (lakāra), replaces the marker with a personal ending (tiṅ), inserts the class vowel śap (`a`), and then a cascade of sound rules produces the surface form. Every rule application is logged with its sūtra number (e.g. "3.4.78") and sūtra name in SLP1 — this trace is the product's headline output, which is why rule attribution and order matter as much as the surface string.

The target paradigm for root BU (all nine person/number cells, SLP1):
`Bavet, BavetAm, BaveyuH, BaveH, Bavetam, Baveta, Baveyam, Baveva, Bavema`.

The ending chain this plan builds, using 3sg as the example:
`tip` →(1.3.9 drop marker p)→ `ti` →(3.4.100 drop final i)→ `t` →(3.4.103 prefix yAs)→ `yAst` →(7.2.79 drop non-final s)→ `yAt` →(7.2.80 yA→iy after a)→ `iyt`; then `Bava`+`iyt` →(6.1.87 a+i→e)→ `Bave`+`yt` →(6.1.66 drop y before consonant)→ `Bavet`.

---

### Task 1: Verify sūtra ids and names against ashtadhyayi.com

**Files:**
- Modify (only if a discrepancy is found): `docs/superpowers/specs/2026-07-20-vidhilin-lakara-design.md`

**Interfaces:**
- Produces: the six verified `(id, SLP1 name)` pairs used verbatim as string literals in Tasks 3–7. The draft values are below; later tasks use them as written unless this task falsifies one.

- [ ] **Step 1: Fetch each sūtra page and compare**

Fetch these URLs (WebFetch or `curl -s <url>` if WebFetch is unavailable) and check that the sūtra text matches the draft name and that the sūtra at that number does what the table claims:

| id | URL | draft SLP1 name | claimed effect |
|----|-----|-----------------|----------------|
| 3.4.108 | https://ashtadhyayi.com/sutraani/3.4.108 | `Jer jus` | liṅ's jhi → jus |
| 3.4.103 | https://ashtadhyayi.com/sutraani/3.4.103 | `yAsuwparasmEpadezUdAtto Nic ca` | yāsuṭ āgama to liṅ's parasmaipada endings, ṅit |
| 7.2.79 | https://ashtadhyayi.com/sutraani/7.2.79 | `liNaH salopo'nantyasya` | non-final s of sārvadhātuka liṅ elided |
| 7.2.80 | https://ashtadhyayi.com/sutraani/7.2.80 | `ato yeyaH` | yā → iy after a-final aṅga |
| 6.1.87 | https://ashtadhyayi.com/sutraani/6.1.87 | `Ad guRaH` | a + ik → guṇa (a+i → e) |
| 6.1.66 | https://ashtadhyayi.com/sutraani/6.1.66 | `lopo vyor vali` | v/y elided before val consonant |

The site shows IAST; convert to SLP1 with: ā=`A`, ī=`I`, ū=`U`, ṭ=`w`, ṭh=`W`, ḍ=`q`, ṣ=`z`, ś=`S`, ṅ=`N`, ñ=`Y`, ṇ=`R`, ai=`E`, au=`O`, jh=`J`, bh=`B`, th=`T`, dh=`D`, kh=`K`, gh=`G`, ph=`P`, ch=`C`, ḥ=`H`, avagraha (')=`'`. Word spacing follows the existing style in `TINANTA_RULES` (words split at pada boundaries: compare `"Aq uttamasya pic ca"`, `"ato dIrGo yaYi"`).

- [ ] **Step 2: Reconcile**

If every id/name/effect matches: done, no commit (nothing changed). If a name differs: correct the literal in the spec's rule table AND use the corrected literal in the corresponding later task, then commit:

```bash
git add docs/superpowers/specs/2026-07-20-vidhilin-lakara-design.md
git commit -m "docs: correct sutra names in vidhilin spec per ashtadhyayi.com"
```

If an *effect* does not match (wrong sūtra number for the behavior), STOP and report — the spec needs a human decision, not a silent renumber.

---

### Task 2: `Lakara::VidhiLin` plumbing (data enum, Context, lakara_name)

**Files:**
- Modify: `crates/panini-data/src/lib.rs:12-16` (Lakara enum)
- Modify: `crates/panini-prakriya/src/context.rs:14-32` (is_ngit_like)
- Modify: `crates/panini/src/lib.rs:89-95` (lakara_name)

**Interfaces:**
- Produces: `Lakara::VidhiLin` (unit variant), `Context::new(Lakara::VidhiLin, ..)` with `is_ngit_like == true`, `lakara_name(Lakara::VidhiLin) == "viDiliN"`. All later tasks consume these.

- [ ] **Step 1: Write the failing tests**

In `crates/panini-prakriya/src/context.rs`, add at the bottom:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vidhilin_is_ngit_like_inherently() {
        // liṅ, like laṅ, is a ṅit lakāra by its own name (the anubandha ṅ),
        // so no atideśa rule is involved — unlike loṭ (3.4.85).
        let c = Context::new(
            Lakara::VidhiLin,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert!(c.is_ngit_like);
    }

    #[test]
    fn lot_is_not_ngit_like_at_construction() {
        // loṭ acquires ṅit-likeness only via rule 3.4.85 at derivation time.
        let c = Context::new(
            Lakara::Lot,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert!(!c.is_ngit_like);
    }
}
```

In `crates/panini/src/lib.rs` tests module, add:

```rust
    #[test]
    fn vidhilin_has_an_slp1_name() {
        assert_eq!(lakara_name(Lakara::VidhiLin), "viDiliN");
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `mise run test`
Expected: compile error — `VidhiLin` not found in `Lakara`.

- [ ] **Step 3: Implement**

In `crates/panini-data/src/lib.rs`, extend the enum:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lakara {
    Lat,
    Lan,
    Lot,
    /// The optative use of liṅ (sārvadhātuka: bhavet). The benedictive use
    /// (āśīrliṅ, ārdhadhātuka: bhūyāt) derives differently and will be a
    /// separate variant when implemented.
    VidhiLin,
}
```

In `crates/panini-prakriya/src/context.rs`, change the `is_ngit_like` initialisation in `Context::new`:

```rust
            // laṅ and liṅ are ṅit inherently (the ṅ anubandha in their own
            // names); loṭ acquires it via rule 3.4.85.
            is_ngit_like: matches!(lakara, Lakara::Lan | Lakara::VidhiLin),
```

Also update the doc comment on the `is_ngit_like` field (context.rs:14-19) to mention liṅ:

```rust
    /// Whether ṅit-conditioned rules (3.4.99, 3.4.100, 3.4.101) apply.
    ///
    /// True inherently for laṅ and vidhiliṅ, which are ṅit by nature (the ṅ
    /// anubandha in their names). For loṭ it is set at derivation time by
    /// 3.4.85 loṭo laṅvat, an *atideśa* — keeping that piece of grammar in
    /// the rule list where it appears in the trace, rather than hiding it in
    /// a match arm here.
    pub is_ngit_like: bool,
```

In `crates/panini/src/lib.rs`, extend `lakara_name`:

```rust
pub fn lakara_name(lakara: Lakara) -> &'static str {
    match lakara {
        Lakara::Lat => "laT",
        Lakara::Lan => "laN",
        Lakara::Lot => "loT",
        Lakara::VidhiLin => "viDiliN",
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `mise run test`
Expected: PASS (all crates; the 162 golden forms are untouched because nothing derives VidhiLin yet — `LAKARAS` in panini-analyze still lists three).

- [ ] **Step 5: Commit**

```bash
git add crates/panini-data/src/lib.rs crates/panini-prakriya/src/context.rs crates/panini/src/lib.rs
git commit -m "feat(data): Lakara::VidhiLin variant; ngit-like at construction; SLP1 name"
```

---

### Task 3: Rule 3.4.108 jher jus

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (insert rule after the 3.4.85 entry, before the 3.4.101 entry; tests at the bottom)

**Interfaces:**
- Consumes: `Lakara::VidhiLin` (Task 2).
- Produces: for a vidhiliṅ derivation, ending `Ji` becomes `us` before any other ending rule sees it, with trace steps `3.4.108` then `1.3.9`.

- [ ] **Step 1: Write the failing tests**

In the `tests` module of `crates/panini-prakriya/src/tinanta.rs` add (note: `Context` is already imported at the top of the file; add `Pada` to the test module's existing `use panini_data::{...}` line if missing — it currently imports `Lakara, Pada, Purusha, Vacana, dhatus`):

```rust
    #[test]
    fn jher_jus_replaces_ji_and_elides_the_j_marker() {
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("Ji")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Bahu,
            ),
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.108").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "us");
        // Both the substitution and the marker elision must be traced.
        assert!(p.log.iter().any(|s| s.sutra == "3.4.108"));
        assert!(p.log.iter().any(|s| s.sutra == "1.3.9"));
    }

    #[test]
    fn jher_jus_leaves_lat_and_lot_ji_alone() {
        // laṭ's Ji must survive to 7.1.3 jho'ntaḥ (Bavanti), loṭ's to
        // 3.4.86 er uḥ (Bavantu).
        for lakara in [Lakara::Lat, Lakara::Lot] {
            let mut p = Prakriya {
                terms: vec![Term::new("BU"), Term::new("Ji")],
                log: vec![],
                ctx: Context::new(lakara, Pada::Parasmaipada, Purusha::Prathama, Vacana::Bahu),
            };
            let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.108").unwrap();
            assert!(!(rule.apply)(&mut p), "{lakara:?}");
            assert_eq!(p.terms[ENDING_PRE_SHAP].text, "Ji");
        }
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `mise run test`
Expected: FAIL — `find(|r| r.id == "3.4.108")` panics on `unwrap()` (no such rule).

- [ ] **Step 3: Implement the rule**

Insert into `TINANTA_RULES` in `crates/panini-prakriya/src/tinanta.rs`, directly AFTER the `3.4.85` entry and BEFORE the `3.4.101` entry:

```rust
    // 3.4.108 jher jus: in liṅ, the ending Ji is replaced by jus. Apavāda to
    // 3.4.100 itaś ca (Ji is i-final), hence ordered before it — the same
    // preemption pattern as 3.4.87/3.4.89 before 3.4.86.
    //
    // The initial j of jus is an anubandha (1.3.7 cuṭū), elided here and
    // recorded as 1.3.9 per the existing convention that saṃjñā rules
    // (1.3.3/1.3.7/1.3.8) are silent and only the elision is traced. It is
    // NOT folded into run_it_samjna: a general cuṭū arm there would also eat
    // the J of laṭ/loṭ's Ji, which is not an anubandha but a coded segment
    // that must survive for 7.1.3 jho'ntaḥ.
    Rule {
        id: "3.4.108",
        name: "Jer jus",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin)
                || p.terms[ENDING_PRE_SHAP].text != "Ji"
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "jus".into();
            p.record("3.4.108", "Jer jus", before);
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "us".into();
            p.record("1.3.9", "tasya lopaH", before);
            true
        },
    },
```

(If Task 1 corrected the name `"Jer jus"`, use the corrected literal in both places.)

- [ ] **Step 4: Run tests to verify they pass**

Run: `mise run test`
Expected: PASS, including all 162 golden forms (the new rule is inert outside vidhiliṅ).

- [ ] **Step 5: Commit**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "feat(prakriya): 3.4.108 jher jus for vidhilin"
```

---

### Task 4: Widen the 3.4.100 and 3.4.101 guards from laṅ-only to ṅit-based

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (the `3.4.100` and `3.4.101` entries; tests at the bottom)

**Interfaces:**
- Consumes: `is_ngit_like` true for VidhiLin (Task 2).
- Produces: for vidhiliṅ, `ti`→`t`, `si`→`s` (3.4.100) and `mi`→`am` (3.4.101), while laṅ and loṭ behavior is byte-identical to before.

- [ ] **Step 1: Write the failing tests**

In the `tests` module of `tinanta.rs` add:

```rust
    #[test]
    fn itash_ca_fires_for_vidhilin() {
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("ti")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.100").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "t");
    }

    #[test]
    fn itash_ca_never_touches_lot_even_when_ngit_like() {
        // After 3.4.85 loṭ is ṅit-like, and after 3.4.87 its madhyama-eka
        // ending is `hi` — which is i-final. A bare ṅit guard would corrupt
        // it to `h`; the guard must exclude loṭ explicitly.
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("hi")],
            log: vec![],
            ctx: Context::new(
                Lakara::Lot,
                Pada::Parasmaipada,
                Purusha::Madhyama,
                Vacana::Eka,
            ),
        };
        p.ctx.is_ngit_like = true; // as 3.4.85 would have set it
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.100").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "hi");
    }

    #[test]
    fn mip_becomes_am_in_vidhilin() {
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("mi")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Uttama,
                Vacana::Eka,
            ),
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.101").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "am");
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `mise run test`
Expected: FAIL — `itash_ca_fires_for_vidhilin` and `mip_becomes_am_in_vidhilin` fail (guards are laṅ-only); `itash_ca_never_touches_lot_even_when_ngit_like` passes already (it pins current behavior so the widening can't break it).

- [ ] **Step 3: Widen the guards**

In the `3.4.100` entry, replace the guard line

```rust
            if !matches!(p.ctx.lakara, Lakara::Lan) || !p.terms[ENDING_PRE_SHAP].text.ends_with('i')
            {
                return false;
            }
```

with

```rust
            // ṅit lakāras generally — but NOT loṭ, whose i-finals belong to
            // the apavāda set 3.4.86/87/89 (and 3.4.87's output `hi` is
            // itself i-final, so a bare ṅit guard would corrupt it to `h`).
            if !p.ctx.is_ngit_like
                || matches!(p.ctx.lakara, Lakara::Lot)
                || !p.terms[ENDING_PRE_SHAP].text.ends_with('i')
            {
                return false;
            }
```

Also update that rule's header comment from "laṅ-only" to:

```rust
    // 3.4.100 itaś ca: the final `i` of a ṅit-lakāra's tiṅ is elided.
    // laṅ/vidhiliṅ: ti → t, si → s, Ji → J (laṅ; liṅ's Ji is gone by
    // 3.4.108). loṭ is excluded: its final `i` is handled by the apavāda
    // 3.4.86 er uḥ.
```

In the `3.4.101` entry, replace the mip arm

```rust
                "mi" if matches!(p.ctx.lakara, Lakara::Lan) => "am",
```

with

```rust
                // loṭ keeps its apavāda 3.4.89 mer niḥ (mi → ni); every
                // other ṅit-like lakāra takes am.
                "mi" if !matches!(p.ctx.lakara, Lakara::Lot) => "am",
```

and update the rule's header comment sentence "The mip→am arm is laṅ-only" to "The mip→am arm excludes loṭ".

- [ ] **Step 4: Run tests to verify they pass**

Run: `mise run test`
Expected: PASS — including all 162 golden forms and the full pinned-trace suite, which is the regression net for this widening. If any laṅ/loṭ golden or trace test fails, the widening is wrong; stop and re-read the guard.

- [ ] **Step 5: Commit**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "refactor(prakriya): widen 3.4.100/3.4.101 guards from lan-only to ngit-based"
```

---

### Task 5: Rule 3.4.103 yāsuṭ

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (insert rule directly AFTER the `3.4.92` entry, i.e. as the last rule before the 3.1.68 boundary; tests at the bottom)

**Interfaces:**
- Consumes: the substituted endings from Tasks 3–4 (`t`, `tAm`, `us`, `s`, `tam`, `ta`, `am`, `va`, `ma`).
- Produces: vidhiliṅ endings carrying the `yAs` prefix (`yAst`, `yAstAm`, `yAsus`, …) for Task 6 to consume.

- [ ] **Step 1: Write the failing tests**

```rust
    #[test]
    fn yasut_prefixes_the_substituted_ending() {
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("t")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.103").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "yAst");
    }

    #[test]
    fn yasut_is_vidhilin_only() {
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("t")],
            log: vec![],
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.103").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "t");
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `mise run test`
Expected: FAIL — no rule with id `3.4.103`.

- [ ] **Step 3: Implement the rule**

Insert AFTER the `3.4.92` entry, BEFORE the boundary comment block for 3.1.68:

```rust
    // 3.4.103 yāsuṭ parasmaipadeṣūdātto ṅic ca: the yāsuṭ-āgama is prefixed
    // to liṅ's parasmaipada endings. Modelled as a text prefix on the ending
    // term (the āṭ 3.4.92 / aṭ 6.4.71 precedent) so the term indices stay
    // stable. "parasmaipadeṣu" is trivially satisfied — Pada has one variant;
    // revisit the guard when ātmanepada arrives (its liṅ takes sīyuṭ, 3.4.102).
    //
    // MUST follow the 3.4.9x/10x ending substitutions above: their guards
    // match the ending text exactly ("mi", "vas", …), so prefixing yAs first
    // would make every one of them miss.
    Rule {
        id: "3.4.103",
        name: "yAsuwparasmEpadezUdAtto Nic ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = format!("yAs{}", p.terms[ENDING_PRE_SHAP].text);
            p.record("3.4.103", "yAsuwparasmEpadezUdAtto Nic ca", before);
            true
        },
    },
```

(Use the Task-1-verified name literal in both places if it differs.)

- [ ] **Step 4: Run tests to verify they pass**

Run: `mise run test`
Expected: PASS (rule is vidhiliṅ-only; the 162 goldens are untouched).

- [ ] **Step 5: Commit**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "feat(prakriya): 3.4.103 yasut-agama on vidhilin endings"
```

---

### Task 6: Rules 7.2.79 (salopa) and 7.2.80 (ato yeyaḥ)

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (insert both rules directly AFTER the `7.1.3` entry, in this order; tests at the bottom)

**Interfaces:**
- Consumes: post-śap term layout `[ANGA, SHAP, ENDING]` and the `yAs…` endings from Task 5.
- Produces: endings in `iy…` shape (`iyt`, `iytAm`, `iyus`, `iys`, `iytam`, `iyta`, `iyam`, `iyva`, `iyma`) for Task 7 to consume.

These are post-śap rules: they address the ending as `ENDING` (index 2) and śap as `SHAP` (index 1). See the boundary comment in the file.

- [ ] **Step 1: Write the failing tests**

```rust
    #[test]
    fn salopa_elides_only_the_non_final_s() {
        // Madhyama-eka is the trap: yAs + s = yAss, and only the FIRST s is
        // non-final. Eliding both would derive *Bave for BaveH.
        for (ending, want) in [("yAst", "yAt"), ("yAss", "yAs"), ("yAsus", "yAus")] {
            let mut p = Prakriya {
                terms: vec![Term::new("Bav"), Term::new("a"), Term::new(ending)],
                log: vec![],
                ctx: Context::new(
                    Lakara::VidhiLin,
                    Pada::Parasmaipada,
                    Purusha::Prathama,
                    Vacana::Eka,
                ),
            };
            let rule = TINANTA_RULES.iter().find(|r| r.id == "7.2.79").unwrap();
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ENDING].text, want, "{ending}");
        }
    }

    #[test]
    fn ato_yeyah_rewrites_the_ya_prefix_after_shap_a() {
        let mut p = Prakriya {
            terms: vec![Term::new("Bav"), Term::new("a"), Term::new("yAt")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.2.80").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "iyt");
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `mise run test`
Expected: FAIL — no rules with ids `7.2.79` / `7.2.80`.

- [ ] **Step 3: Implement both rules**

Insert AFTER the `7.1.3` entry, 7.2.79 first:

```rust
    // 7.2.79 liṅaḥ salopo 'nantyasya: the non-final s of sārvadhātuka liṅ's
    // ending is elided. yAst → yAt, yAss → yAs (madhyama-eka: only the first
    // s is non-final!), yAsus → yAus. MUST precede 7.2.80: only after the s
    // goes does the ending start with the `yA` shape 7.2.80 rewrites.
    Rule {
        id: "7.2.79",
        name: "liNaH salopo'nantyasya",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin) {
                return false;
            }
            let text = &p.terms[ENDING].text;
            let n = text.chars().count();
            let reduced: String = text
                .chars()
                .enumerate()
                .filter(|&(i, c)| c != 's' || i + 1 == n)
                .map(|(_, c)| c)
                .collect();
            if reduced == *text {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = reduced;
            p.record("7.2.79", "liNaH salopo'nantyasya", before);
            true
        },
    },
    // 7.2.80 ato yeyaḥ: after an a-final aṅga (here: the śap), the yA of the
    // yāsuṭ is replaced by iy. yAt → iyt, yAus → iyus.
    Rule {
        id: "7.2.80",
        name: "ato yeyaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin)
                || p.terms[SHAP].text != "a"
                || !p.terms[ENDING].text.starts_with("yA")
            {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(2).collect();
            p.terms[ENDING].text = format!("iy{rest}");
            p.record("7.2.80", "ato yeyaH", before);
            true
        },
    },
```

(Use Task-1-verified name literals if they differ.)

- [ ] **Step 4: Run tests to verify they pass**

Run: `mise run test`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "feat(prakriya): 7.2.79 salopa and 7.2.80 ato yeyah for vidhilin"
```

---

### Task 7: Rules 6.1.87 (ād guṇaḥ) and 6.1.66 (lopo vyor vali); end-to-end vidhiliṅ forms

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (insert both rules directly AFTER the `6.1.97` entry and BEFORE the `6.4.105` entry, in this order; tests at the bottom; also extend the name-gate test's lakāra array)

**Interfaces:**
- Consumes: the `iy…` endings from Task 6.
- Produces: complete vidhiliṅ surface forms from `derive(d, Lakara::VidhiLin, ..)`; the full 9-cell BU paradigm passes.

- [ ] **Step 1: Write the failing tests**

```rust
    fn lin_form(code: &str, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, Lakara::VidhiLin, Pada::Parasmaipada, pu, va).text()
    }

    #[test]
    fn bhu_vidhilin_all_nine_cells() {
        for (pu, va, want) in [
            (Purusha::Prathama, Vacana::Eka, "Bavet"),
            (Purusha::Prathama, Vacana::Dvi, "BavetAm"),
            (Purusha::Prathama, Vacana::Bahu, "BaveyuH"),
            (Purusha::Madhyama, Vacana::Eka, "BaveH"),
            (Purusha::Madhyama, Vacana::Dvi, "Bavetam"),
            (Purusha::Madhyama, Vacana::Bahu, "Baveta"),
            (Purusha::Uttama, Vacana::Eka, "Baveyam"),
            (Purusha::Uttama, Vacana::Dvi, "Baveva"),
            (Purusha::Uttama, Vacana::Bahu, "Bavema"),
        ] {
            assert_eq!(lin_form("BU", pu, va), want, "{pu:?} {va:?}");
        }
    }

    #[test]
    fn vali_lopa_spares_a_following_vowel() {
        // BaveyuH keeps its y because `u` is not a val consonant; Baveva
        // loses it because `v` is. Pin the guard at the rule level.
        for (ending, fires, want) in [("yva", true, "va"), ("yus", false, "yus")] {
            let mut p = Prakriya {
                terms: vec![Term::new("Bav"), Term::new("e"), Term::new(ending)],
                log: vec![],
                ctx: Context::new(
                    Lakara::VidhiLin,
                    Pada::Parasmaipada,
                    Purusha::Uttama,
                    Vacana::Dvi,
                ),
            };
            let rule = TINANTA_RULES.iter().find(|r| r.id == "6.1.66").unwrap();
            assert_eq!((rule.apply)(&mut p), fires, "{ending}");
            assert_eq!(p.terms[ENDING].text, want, "{ending}");
        }
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `mise run test`
Expected: FAIL — no rules `6.1.87` / `6.1.66`; `bhu_vidhilin_all_nine_cells` produces stuck forms like `Bavaiyt`.

- [ ] **Step 3: Implement both rules**

Insert AFTER the `6.1.97` entry, BEFORE the `6.4.105` entry, 6.1.87 first:

```rust
    // 6.1.87 ād guṇaḥ: śap `a` + ending-initial `i` coalesce to guṇa `e`.
    // Bava + iyt → Bave + yt. Same mechanical shape as 6.1.101 above: the
    // śap stands in for the coalesced vowel, the ending loses its initial.
    // MUST precede 6.1.66: only after the `i` is absorbed does the ending
    // start with the `y` that 6.1.66 tests.
    Rule {
        id: "6.1.87",
        name: "Ad guRaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms[SHAP].text != "a" || !p.terms[ENDING].text.starts_with('i') {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "e".into();
            p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
            p.record("6.1.87", "Ad guRaH", before);
            true
        },
    },
    // 6.1.66 lopo vyor vali: v or y is elided before a val consonant. Here
    // only the ending-initial y from the yāsuṭ chain ever matches: yt → t,
    // yva → va; yus survives (u is a vowel, not in val). The val pratyāhāra
    // is every consonant except y, and no `yy` sequence arises in this
    // engine, so "any consonant" is an exact guard here.
    Rule {
        id: "6.1.66",
        name: "lopo vyor vali",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let mut chars = p.terms[ENDING].text.chars();
            if chars.next() != Some('y') {
                return false;
            }
            let Some(second) = chars.next() else {
                return false;
            };
            if is_vowel(second) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
            p.record("6.1.66", "lopo vyor vali", before);
            true
        },
    },
```

Both are shape-guarded rather than lakāra-guarded (per the spec): outside vidhiliṅ no ending is ever `i`- or `y`-initial at this point in the pipeline, and the shape guard keeps them honest if that ever changes.

- [ ] **Step 4: Extend the name-gate test over the new lakāra**

In the existing test `recorded_step_names_match_tinanta_rules_for_every_id`, change

```rust
        let lakaras = [Lakara::Lat, Lakara::Lan, Lakara::Lot];
```

to

```rust
        let lakaras = [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin];
```

and in its doc comment change "all three lakaras/nine cells" to "all four lakaras/nine cells".

- [ ] **Step 5: Run tests to verify they pass**

Run: `mise run test`
Expected: PASS — the nine BU cells derive, the name gate covers vidhiliṅ traces, the SLP1-digraph gate covers the new names, and the 162 goldens still pass.

- [ ] **Step 6: Commit**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "feat(prakriya): 6.1.87 ad gunah, 6.1.66 lopo vyor vali; vidhilin end-to-end"
```

---

### Task 8: Analyzer coverage (LAKARAS) and roundtrip

**Files:**
- Modify: `crates/panini-analyze/src/lib.rs:14` (LAKARAS)

**Interfaces:**
- Consumes: working `derive(.., Lakara::VidhiLin, ..)` (Task 7).
- Produces: `Panini::check("Bavet")` returns VALID with a vidhiliṅ analysis; `panini_analyze::LAKARAS` has 4 entries (the roundtrip test iterates it, so it covers vidhiliṅ automatically).

- [ ] **Step 1: Write the failing test**

In `crates/panini/src/lib.rs` tests module, add:

```rust
    #[test]
    fn vidhilin_form_checks_valid() {
        let engine = Panini::new();
        let r = engine.check("Bavet");
        assert!(matches!(r.verdict, Verdict::Valid));
        let a = r.analyses.iter().find(|a| a.form_slp1 == "Bavet").unwrap();
        assert_eq!(a.dhatu, "BU");
        assert!(matches!(a.lakara, Lakara::VidhiLin));
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `mise run test`
Expected: FAIL — `check("Bavet")` is Invalid because the analyzer never proposes VidhiLin candidates.

- [ ] **Step 3: Extend LAKARAS**

In `crates/panini-analyze/src/lib.rs`:

```rust
pub const LAKARAS: &[Lakara] = &[Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin];
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `mise run test`
Expected: PASS — including `roundtrip.rs`, which now round-trips 6 roots × 4 lakāras × 9 cells = 216 derivations through `check`.

- [ ] **Step 5: Commit**

```bash
git add crates/panini-analyze/src/lib.rs crates/panini/src/lib.rs
git commit -m "feat(analyze): propose vidhilin candidates; facade test for Bavet"
```

---

### Task 9: Golden paradigm (216 forms), negatives, pinned traces

**Files:**
- Modify: `crates/panini/tests/paradigm.rs` (six new PARADIGM rows; two new negatives)
- Modify: `crates/panini/tests/trace.rs` (three new pinned-trace tests; header comment)

**Interfaces:**
- Consumes: everything above.
- Produces: the grammar gate for this slice. These tables are the source of truth per AGENTS.md.

- [ ] **Step 1: Verify the 54 new forms against the reference**

Before writing them into the golden table, confirm the paradigm below against ashtadhyayi.com's dhātu pages (https://ashtadhyayi.com/dhatu/, root bhū 01.0001, gaṇa bhvādi, vidhiliṅ parasmaipada — the site renders IAST/Devanagari; convert to SLP1). All six roots are regular: the pattern is `<laṭ-stem minus "ati">` + `et/etAm/eyuH/eH/etam/eta/eyam/eva/ema`. If the site disagrees with any form below, STOP and report the discrepancy rather than committing either version.

- [ ] **Step 2: Add the six PARADIGM rows**

Append to the `PARADIGM` array in `crates/panini/tests/paradigm.rs`:

```rust
    (
        "BU",
        "viDiliN",
        [
            "Bavet", "BavetAm", "BaveyuH", "BaveH", "Bavetam", "Baveta", "Baveyam", "Baveva",
            "Bavema",
        ],
    ),
    (
        "nI",
        "viDiliN",
        [
            "nayet", "nayetAm", "nayeyuH", "nayeH", "nayetam", "nayeta", "nayeyam", "nayeva",
            "nayema",
        ],
    ),
    (
        "ji",
        "viDiliN",
        [
            "jayet", "jayetAm", "jayeyuH", "jayeH", "jayetam", "jayeta", "jayeyam", "jayeva",
            "jayema",
        ],
    ),
    (
        "smf",
        "viDiliN",
        [
            "smaret", "smaretAm", "smareyuH", "smareH", "smaretam", "smareta", "smareyam",
            "smareva", "smarema",
        ],
    ),
    (
        "paW",
        "viDiliN",
        [
            "paWet", "paWetAm", "paWeyuH", "paWeH", "paWetam", "paWeta", "paWeyam", "paWeva",
            "paWema",
        ],
    ),
    (
        "vad",
        "viDiliN",
        [
            "vadet", "vadetAm", "vadeyuH", "vadeH", "vadetam", "vadeta", "vadeyam", "vadeva",
            "vadema",
        ],
    ),
```

Update the doc comment on `PARADIGM` if it mentions a count.

- [ ] **Step 3: Add the negatives**

In `known_nonforms_are_invalid`, extend the list with liṅ-flavored cross-lakāra confusions (insert after `"aBavatu"`):

```rust
        "aBavet", // laṅ's aṭ-āgama on a vidhiliṅ form
        "Bavetu", // loṭ's er uḥ ending on a vidhiliṅ stem
```

- [ ] **Step 4: Run the paradigm test**

Run: `mise run test`
Expected: PASS (216 positives, 8 negatives). A failure here with the engine tests green means a golden-table typo — recheck Step 1 before touching engine code.

- [ ] **Step 5: Add the pinned-trace tests**

In `crates/panini/tests/trace.rs`:

First update the header comment: change "across all three lakāras this crate covers (laṭ, laṅ, loṭ) — **nine** tests in total below" to "across all four lakāras this crate covers (laṭ, laṅ, loṭ, vidhiliṅ) — **twelve** tests in total below".

Then append:

```rust
#[test]
fn bhavet_trace_is_exactly_the_vidhilin_vali_lopa_path() {
    // BU vidhiliṅ prathama eka: tip -> ti (1.3.9) -> t (3.4.100, now
    // ṅit-wide), yāsuṭ (3.4.103), salopa (7.2.79), yA -> iy (7.2.80),
    // a+i -> e (6.1.87), y dropped before t (6.1.66).
    assert_eq!(
        trace_for("Bavet"),
        vec![
            "3.4.78", "1.3.9", "3.4.100", "3.4.103", "3.1.68", "1.3.9", "7.2.79", "7.2.80",
            "7.3.84", "6.1.78", "6.1.87", "6.1.66"
        ]
    );
}

#[test]
fn bhaveyuh_trace_is_exactly_the_jus_path() {
    // BU vidhiliṅ prathama bahu: Ji -> jus (3.4.108) -> us (1.3.9), then the
    // yāsuṭ chain; the y of `yus` SURVIVES 6.1.66 (u is not a val consonant),
    // and word-final s becomes visarga (8.3.15).
    assert_eq!(
        trace_for("BaveyuH"),
        vec![
            "3.4.78", "3.4.108", "1.3.9", "3.4.103", "3.1.68", "1.3.9", "7.2.79", "7.2.80",
            "7.3.84", "6.1.78", "6.1.87", "8.3.15"
        ]
    );
}

#[test]
fn bhaveyam_trace_is_exactly_the_widened_mip_path() {
    // BU vidhiliṅ uttama eka: mip -> mi (1.3.9) -> am (3.4.101, mip arm now
    // fires outside laṅ), then the yāsuṭ chain; no 6.1.66 (a is a vowel).
    assert_eq!(
        trace_for("Baveyam"),
        vec![
            "3.4.78", "1.3.9", "3.4.101", "3.4.103", "3.1.68", "1.3.9", "7.2.79", "7.2.80",
            "7.3.84", "6.1.78", "6.1.87"
        ]
    );
}
```

- [ ] **Step 6: Run all tests**

Run: `mise run test`
Expected: PASS. If a pinned trace fails, compare the actual sequence in the assertion message against `TINANTA_RULES` order — a mismatch means either a rule fired that the hand-trace missed (fix the test only if the extra firing is grammatically correct; otherwise fix the guard) or rule insertion order in Tasks 3–7 deviated from the spec table.

- [ ] **Step 7: Commit**

```bash
git add crates/panini/tests/paradigm.rs crates/panini/tests/trace.rs
git commit -m "test: golden paradigm to 216 forms; pinned vidhilin traces; lin negatives"
```

---

### Task 10: Documentation sweep and full verification

**Files:**
- Modify: `AGENTS.md:27` (golden-test description)
- Modify: `README.md:17` (lakāra list)
- Modify: `docs/ARCHITECTURE.md:21` ("all three lakāras")

**Interfaces:**
- Consumes: everything above. Produces: a branch ready for review.

- [ ] **Step 1: Update the three stale scope mentions**

- `AGENTS.md:27`: change "162 forms across laṭ/laṅ/loṭ" to "216 forms across laṭ/laṅ/loṭ/vidhiliṅ".
- `README.md:17`: change "in three lakāras: *laṭ* (present), *laṅ* (imperfect), and *loṭ*" so the sentence reads "…in four lakāras: *laṭ* (present), *laṅ* (imperfect), *loṭ* (imperative), and *vidhiliṅ* (optative)…" — read the surrounding sentence and keep its structure.
- `docs/ARCHITECTURE.md:21`: change "all three lakāras" to "all four lakāras".

Then grep for stragglers and fix any that name the three-lakāra scope (spec/plan documents under `docs/superpowers/` are historical records — do NOT edit them):

Run: `grep -rn "three lak\|162" README.md AGENTS.md docs/ARCHITECTURE.md crates`
Expected: no hits outside historical spec/plan docs.

- [ ] **Step 2: Full verification**

Run, in order:

```bash
mise run fmt
mise run lint
mise run test
```

Expected: fmt makes no changes (or only trivial ones — re-stage them), lint is clean, all tests pass.

- [ ] **Step 3: Mutation testing**

If `cargo-mutants` is not installed: `MISE_ENV=dev mise install` first.

Run: `mise run mutants`
Expected: no surviving mutants in the new rules' guards or bodies. For each survivor in code this slice touched, add a pinning test (rule-level, in `tinanta.rs`'s test module, in the style of Task 4's tests) and re-run. Survivors in code this slice did not touch: report them, don't chase them.

- [ ] **Step 4: Commit**

```bash
git add AGENTS.md README.md docs/ARCHITECTURE.md
git commit -m "docs: four-lakara scope (vidhilin) in AGENTS, README, ARCHITECTURE"
```

(Include any pinning tests from Step 3 in this commit or their own `test:` commit.)

- [ ] **Step 5: Finish the branch**

Use the superpowers:finishing-a-development-branch skill to decide merge/PR handling.
