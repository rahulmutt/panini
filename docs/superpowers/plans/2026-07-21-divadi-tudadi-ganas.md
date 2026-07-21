# Divādi + Tudādi gaṇas Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add divādi (gaṇa 4) and tudādi (gaṇa 6) alongside bhvādi, with three parasmaipadī and three ātmanepadī roots each, growing golden coverage 432 → 864 forms across the four existing lakāras and both padas.

**Architecture:** Each gaṇa differs only in its vikaraṇa (the affix between root and ending): bhvādi = śap (pit), divādi = śyan (apit), tudādi = śa (apit). The apit vikaraṇas are made ṅit by 1.2.4 and block guṇa by 1.1.5 — the one piece of new grammar. Two new insertion rules (3.1.69, 3.1.77), a second application of 1.2.4, two guṇa-rule guards, and one root-specific rule (8.2.77, for √div) carry the whole slice. The analyzer, facade, and CLI need no code changes.

**Tech Stack:** Rust workspace (`panini-data`, `panini-prakriya`, `panini`), `mise` task runner, SLP1 internal encoding, golden + trace + mutation tests.

## Global Constraints

- Toolchain via `mise` (rust 1.97.1); build/test with `mise run build | test`. Never install Rust globally.
- `#![forbid(unsafe_code)]` in every crate touched.
- SLP1 is the only internal representation; no transliteration outside `panini-lipi` (untouched here).
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, not as a branch inside `derive`.
- Sūtra ids/names in traces must match the reference (ashtadhyayi.com / the `ashtadhyayi-com/data` GitHub mirror). Task 2 verifies the three new names; do not skip it.
- **The 432 existing golden forms and every pinned bhvādi trace must never change.** The mechanism is designed to be a no-op for bhvādi (śap is pit): the second 1.2.4 skips śap, and the 1.1.5 guards pass because śap never carries `Tag::Ngit`. Any bhvādi golden/trace change is a bug.
- **Working state between tasks:** tasks are ordered so `mise run test` passes after each. New golden blocks (Task 7) land only after the rules that derive them (Tasks 3–6); until then no divādi/tudādi form is asserted.
- Root data verified against the Vidyut Dhātupāṭha mirror (`vidyut-prakriya/data/dhatupatha.tsv`). √tud is taken parasmaipada-only (its ubhayapada svarita and 1.3.72 are deferred, per the spec).

---

### Task 1: Data layer — two gaṇas and twelve roots

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (the `Gana` enum, the `DHATUS` static, the count test)
- Modify: `data/dhatupatha.tsv` (reference mirror of the Rust static — keep in sync)

**Interfaces:**
- Produces: `Gana::Divadi`, `Gana::Tudadi`; twelve new `Dhatu` entries reachable via `dhatus()`. `dhatus().len() == 24`.

- [ ] **Step 1: Update the failing count test.** In `crates/panini-data/src/lib.rs`, change `has_twelve_curated_roots_with_padas` to assert 24 and to spot-check one new root per gaṇa/pada:

```rust
    #[test]
    fn has_twentyfour_curated_roots_with_padas() {
        assert_eq!(dhatus().len(), 24);
        let bu = dhatus().iter().find(|d| d.code == "BU").unwrap();
        assert!(matches!(bu.pada, Pada::Parasmaipada));
        let labh = dhatus().iter().find(|d| d.code == "laB").unwrap();
        assert!(matches!(labh.pada, Pada::Atmanepada));
        // Both vowel-initial atmanepadi roots must be present (they exercise
        // the AT-augment path 6.4.72/6.1.90).
        assert!(dhatus().iter().any(|d| d.code == "eD"));
        assert!(dhatus().iter().any(|d| d.code == "Ikz"));
        // New gaṇas, both padas.
        let div = dhatus().iter().find(|d| d.code == "div").unwrap();
        assert!(matches!(div.gana, Gana::Divadi) && matches!(div.pada, Pada::Parasmaipada));
        let man = dhatus().iter().find(|d| d.code == "man").unwrap();
        assert!(matches!(man.gana, Gana::Divadi) && matches!(man.pada, Pada::Atmanepada));
        let tud = dhatus().iter().find(|d| d.code == "tud").unwrap();
        assert!(matches!(tud.gana, Gana::Tudadi) && matches!(tud.pada, Pada::Parasmaipada));
        let juz = dhatus().iter().find(|d| d.code == "juz").unwrap();
        assert!(matches!(juz.gana, Gana::Tudadi) && matches!(juz.pada, Pada::Atmanepada));
    }
```

- [ ] **Step 2: Run it to verify it fails.**

Run: `mise run test -- -p panini-data`
Expected: FAIL (`Gana::Divadi`/`Gana::Tudadi` unknown; `len()` is 12).

- [ ] **Step 3: Add the enum variants.** In `crates/panini-data/src/lib.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gana {
    Bhvadi,
    Divadi,
    Tudadi,
}
```

- [ ] **Step 4: Append the twelve roots** to the `DHATUS` static (after the `Ikz` entry, before the closing `];`):

```rust
    // divādi (gaṇa 4) — vikaraṇa śyan (3.1.69)
    Dhatu { code: "div", gana: Gana::Divadi, pada: Pada::Parasmaipada, artha: "krIqAyAm" },
    Dhatu { code: "naS", gana: Gana::Divadi, pada: Pada::Parasmaipada, artha: "adarSane" },
    Dhatu { code: "kup", gana: Gana::Divadi, pada: Pada::Parasmaipada, artha: "kroDe" },
    Dhatu { code: "man", gana: Gana::Divadi, pada: Pada::Atmanepada, artha: "jYAne" },
    Dhatu { code: "yuD", gana: Gana::Divadi, pada: Pada::Atmanepada, artha: "samprahAre" },
    Dhatu { code: "vid", gana: Gana::Divadi, pada: Pada::Atmanepada, artha: "sattAyAm" },
    // tudādi (gaṇa 6) — vikaraṇa śa (3.1.77)
    Dhatu { code: "tud", gana: Gana::Tudadi, pada: Pada::Parasmaipada, artha: "vyaTane" },
    Dhatu { code: "liK", gana: Gana::Tudadi, pada: Pada::Parasmaipada, artha: "akzaravinyAse" },
    Dhatu { code: "viS", gana: Gana::Tudadi, pada: Pada::Parasmaipada, artha: "praveSane" },
    Dhatu { code: "juz", gana: Gana::Tudadi, pada: Pada::Atmanepada, artha: "prItisevanayoH" },
    Dhatu { code: "vij", gana: Gana::Tudadi, pada: Pada::Atmanepada, artha: "BayacalanayoH" },
    Dhatu { code: "gur", gana: Gana::Tudadi, pada: Pada::Atmanepada, artha: "udyamane" },
```

(Existing entries use the expanded field-per-line form; match that or the compact form above — both compile. Keep the file's existing style if the reviewer prefers.)

- [ ] **Step 5: Mirror the rows into `data/dhatupatha.tsv`.** Append (tab-separated: code, gaṇa, pada, artha — match the existing column layout in that file):

```
div	divadi	parasmaipada	krIqAyAm
naS	divadi	parasmaipada	adarSane
kup	divadi	parasmaipada	kroDe
man	divadi	atmanepada	jYAne
yuD	divadi	atmanepada	samprahAre
vid	divadi	atmanepada	sattAyAm
tud	tudadi	parasmaipada	vyaTane
liK	tudadi	parasmaipada	akzaravinyAse
viS	tudadi	parasmaipada	praveSane
juz	tudadi	atmanepada	prItisevanayoH
vij	tudadi	atmanepada	BayacalanayoH
gur	tudadi	atmanepada	udyamane
```

Check the existing header/column order first (`data/dhatupatha.tsv` currently has `code	gana	pada	artha` rows like `BU	bhvadi	parasmaipada	sattAyAm`) and match it exactly.

- [ ] **Step 6: Run tests to verify they pass.**

Run: `mise run test -- -p panini-data`
Expected: PASS (24 roots).

- [ ] **Step 7: Commit.**

```bash
git add crates/panini-data/src/lib.rs data/dhatupatha.tsv
git commit -m "feat(data): divadi + tudadi ganas, twelve new roots"
```

---

### Task 2: Verify the three new sūtra names against the reference

Sūtra ids/names in traces must match the reference (AGENTS.md hard gate). Only three new names are introduced; verify them before they are baked into `record()` calls.

**Files:** none (verification only; findings applied in Tasks 4–6).

- [ ] **Step 1: Fetch the reference text** for 3.1.69, 3.1.77, 8.2.77 from the open data source. ashtadhyayi.com is a JS SPA that plain fetch cannot render — use the `ashtadhyayi-com/data` GitHub repository (or the `ashtadhyayi.github.io` mirror), as slice 2/3 did. See the "Reference verification" section of `docs/superpowers/specs/2026-07-20-vidhilin-lakara-design.md` for the exact method used last time.

- [ ] **Step 2: Confirm the SLP1 transliteration** of each name against the Devanāgarī (not another transliteration — this is how slice 2 caught a spacing bug). Draft names being checked:
  - `3.1.69` → `divAdiByaH Syan` (divādibhyaḥ śyan)
  - `3.1.77` → `tudAdiByaH SaH` (tudādibhyaḥ śaḥ)
  - `8.2.77` → `hali ca` (hali ca)

- [ ] **Step 3: Record the confirmed spellings** here (edit this line in the plan): `3.1.69 = "____"`, `3.1.77 = "____"`, `8.2.77 = "____"`. Tasks 4–6 MUST use these confirmed strings verbatim in both the `id`/`name` fields and the `p.record(...)` calls. If a spelling differs from the draft, use the confirmed one everywhere.

- [ ] **Step 4: Commit** (documentation of the check, if the repo convention records it — otherwise skip and note in the Task 4 commit).

```bash
git commit --allow-empty -m "docs: confirm divadi/tudadi sutra names against the reference"
```

---

### Task 3: The guṇa-block mechanism (Pit tag, second 1.2.4, 1.1.5 guards)

Build the apit-vikaraṇa → ṅit → guṇa-block machinery **first**, guarded so it is a complete no-op for bhvādi (śap is pit). No divādi/tudādi form is asserted yet — this task's proof is that every existing bhvādi test still passes.

**Files:**
- Modify: `crates/panini-prakriya/src/term.rs` (add `Tag::Pit`, `Tag::Divadi`, `Tag::Tudadi`)
- Modify: `crates/panini-prakriya/src/tinanta.rs` (3.1.68 tags śap `Pit`; add second 1.2.4; guard 7.3.84 and 7.3.86)

**Interfaces:**
- Consumes: `Tag::Vikarana`, `Tag::Ngit`, `Tag::Sarvadhatuka` (existing); the `SHAP` index (existing).
- Produces: `Tag::Pit` (on śap), `Tag::Divadi`/`Tag::Tudadi` (declared now, first consumed in Task 4/5), and a second `1.2.4` `Rule` entry positioned directly after `3.1.68`. From here, an inserted apit vikaraṇa at `SHAP` carries `Tag::Ngit`; 7.3.84/7.3.86 decline when `SHAP` is ṅit.

- [ ] **Step 1: Write the failing test.** In `crates/panini-prakriya/src/tinanta.rs` tests, add a guard that śap keeps `Pit` and bhvādi guṇa is unaffected:

```rust
    #[test]
    fn shap_is_pit_and_bhvadi_guna_survives() {
        // Regression guard for Task 3: adding the guṇa-block mechanism must
        // not disturb bhvādi. śap is pit, so 7.3.84 still fires for BU.
        assert_eq!(form("BU", Purusha::Prathama, Vacana::Eka), "Bavati");
        let d = dhatus().iter().find(|d| d.code == "vft").unwrap();
        // vṛt uses 7.3.86 (laghūpadhā guṇa) before śap (pit) → vartate.
        assert_eq!(
            derive(d, Lakara::Lat, Pada::Atmanepada, Purusha::Prathama, Vacana::Eka).text(),
            "vartate"
        );
    }
```

- [ ] **Step 2: Run it to verify it fails/passes.**

Run: `mise run test -- -p panini-prakriya shap_is_pit_and_bhvadi_guna_survives`
Expected: PASS already (bhvādi behavior is correct today). This test is a *regression pin*, not red-first — it must stay green through Steps 3–6. Proceed.

- [ ] **Step 3: Add the tags** in `crates/panini-prakriya/src/term.rs` to the `Tag` enum (after `Ngit`):

```rust
    /// The pratyaya carries the p-anubandha (pit). Set on śap by 3.1.68; the
    /// second 1.2.4 application reads it to leave śap alone (only apit
    /// vikaraṇas — śyan, śa — become ṅit).
    Pit,
    /// The dhātu belongs to divādi (gaṇa 4) / tudādi (gaṇa 6). Data-layer
    /// stand-ins mirroring Atmanepadin, read by 3.1.69 / 3.1.77. bhvādi
    /// carries neither tag.
    Divadi,
    Tudadi,
```

- [ ] **Step 4: Tag śap `Pit` in 3.1.68.** In `crates/panini-prakriya/src/tinanta.rs`, in the `3.1.68` rule, add the `Pit` tag where śap is built (after the `Sarvadhatuka` tag):

```rust
            let mut s = Term::new("Sap");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            s.add(Tag::Pit); // p-anubandha: śap is pit, so 1.2.4 leaves it alone
            p.terms.insert(SHAP, s);
```

- [ ] **Step 5: Add the second 1.2.4 application** as a new `Rule` entry immediately **after** the `3.1.68` entry (and before `6.4.71`):

```rust
    // 1.2.4 sārvadhātukam apit — second application, on the vikaraṇa. The
    // first application (above the boundary) tags apit ātmanepada endings;
    // this one tags the apit sārvadhātuka VIKARAṆA ṅit once it exists. śyan
    // and śa are apit (no p-anubandha); śap carries Tag::Pit (3.1.68) and is
    // skipped — so bhvādi is untouched. NOT pada-gated: śyan/śa are apit in
    // parasmaipada derivations too, which is what blocks guṇa in dīvyati /
    // kupyati / tudati.
    Rule {
        id: "1.2.4",
        name: "sArvaDAtukam apit",
        kind: RuleKind::Atidesha,
        apply: |p| {
            if !(p.terms.len() > SHAP
                && p.terms[SHAP].has(Tag::Vikarana)
                && !p.terms[SHAP].has(Tag::Pit)
                && !p.terms[SHAP].has(Tag::Ngit))
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].add(Tag::Ngit);
            p.record("1.2.4", "sArvaDAtukam apit", before);
            true
        },
    },
```

- [ ] **Step 6: Add the 1.1.5 kṅiti-ca guard** to `7.3.84` and `7.3.86`. At the top of each rule's `apply`, before its existing body:

```rust
            // 1.1.5 kṅiti ca: a following ṅit sārvadhātuka blocks guṇa. The
            // vikaraṇa at SHAP is ṅit (1.2.4) exactly when apit (śyan, śa);
            // śap is pit and is not, so bhvādi guṇa is unaffected.
            if p.terms.len() > SHAP && p.terms[SHAP].has(Tag::Ngit) {
                return false;
            }
```

For `7.3.84` this precedes `let last = p.terms[ANGA].text.chars().last().unwrap();`. For `7.3.86` it precedes `let chars: Vec<char> = p.terms[ANGA].text.chars().collect();`.

- [ ] **Step 7: Run the whole suite.**

Run: `mise run test`
Expected: PASS — all 432 golden forms and every pinned bhvādi trace unchanged (the second 1.2.4 skips śap; the guards pass because śap lacks `Ngit`). If any bhvādi trace changed, the guard is checking the wrong index — confirm it reads `SHAP` (the vikaraṇa), not `ENDING`.

- [ ] **Step 8: Commit.**

```bash
git add crates/panini-prakriya/src/term.rs crates/panini-prakriya/src/tinanta.rs
git commit -m "feat(prakriya): apit-vikarana guna-block scaffolding — Tag::Pit, 2nd 1.2.4, 1.1.5 guards on 7.3.84/86"
```

---

### Task 4: Vikaraṇa selection — 3.1.69 śyan, 3.1.77 śa

Add the two apit vikaraṇas as apavādas to śap, and make the aṅga carry its gaṇa. Now divādi/tudādi derive correctly for every root **except √div** (which still needs 8.2.77, Task 5).

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (tag the aṅga in `derive`; add `3.1.69` and `3.1.77` before `3.1.68`; guard `3.1.68`)

**Interfaces:**
- Consumes: `Tag::Divadi`/`Tag::Tudadi` (Task 3), the `Tag::Ngit`-on-vikaraṇa mechanism (Task 3), confirmed names (Task 2).
- Produces: rules `3.1.69`, `3.1.77` as entries directly before `3.1.68`; a decline-if-vikaraṇa-present guard on `3.1.68`. divādi/tudādi forms now derive with guṇa blocked.

- [ ] **Step 1: Write the failing tests.** In `crates/panini-prakriya/src/tinanta.rs` tests:

```rust
    fn form_g(code: &str, la: Lakara, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, la, d.pada, pu, va).text()
    }

    #[test]
    fn divadi_tudadi_present_third_singular() {
        // Guṇa blocked by 1.1.5 (śyan/śa are ṅit): kup→kupyati NOT kopyati,
        // tud→tudati NOT todati, juṣ→juṣate NOT joṣate.
        assert_eq!(form_g("naS", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "naSyati");
        assert_eq!(form_g("kup", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "kupyati");
        assert_eq!(form_g("man", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "manyate");
        assert_eq!(form_g("yuD", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "yuDyate");
        assert_eq!(form_g("vid", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "vidyate");
        assert_eq!(form_g("tud", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "tudati");
        assert_eq!(form_g("liK", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "liKati");
        assert_eq!(form_g("viS", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "viSati");
        assert_eq!(form_g("juz", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "juzate");
        assert_eq!(form_g("vij", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "vijate");
        assert_eq!(form_g("gur", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "gurate");
    }
```

(√div is intentionally absent — it derives `divyati` until Task 5 adds the lengthening.)

- [ ] **Step 2: Run to verify failure.**

Run: `mise run test -- -p panini-prakriya divadi_tudadi_present_third_singular`
Expected: FAIL (no 3.1.69/3.1.77; divādi/tudādi roots route through śap and derive `Bavati`-shaped junk, e.g. `naS`→`naSati` or guṇa'd forms).

- [ ] **Step 3: Tag the aṅga with its gaṇa** in `derive` (in `crates/panini-prakriya/src/tinanta.rs`). Extend the aṅga-building block:

```rust
    p.terms.push({
        let mut t = Term::new(dhatu.code);
        t.add(Tag::Dhatu);
        if matches!(dhatu.pada, Pada::Atmanepada) {
            t.add(Tag::Atmanepadin);
        }
        match dhatu.gana {
            Gana::Divadi => t.add(Tag::Divadi),
            Gana::Tudadi => t.add(Tag::Tudadi),
            Gana::Bhvadi => {}
        }
        t
    });
```

Add `Gana` to the `use panini_data::{...}` import at the top of the file.

- [ ] **Step 4: Add 3.1.69 and 3.1.77** as two new `Rule` entries directly **before** the `3.1.68` entry (order: 3.1.69, 3.1.77, then 3.1.68), using the Task-2 confirmed names:

```rust
    // 3.1.69 divādibhyaḥ śyan: divādi (gaṇa 4) takes śyan, not śap. Apavāda
    // to the utsarga 3.1.68, ordered before it (as 6.4.72 precedes 6.4.71).
    // śyan is apit; the second 1.2.4 makes it ṅit and 1.1.5 then blocks guṇa.
    Rule {
        id: "3.1.69",
        name: "divAdiByaH Syan",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Divadi) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("Syan");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.69", "divAdiByaH Syan", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S, 1.3.3 strips n → ya
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 3.1.77 tudādibhyaḥ śaḥ: tudādi (gaṇa 6) takes śa, not śap. Apavāda to
    // 3.1.68, same shape as 3.1.69. śa is apit → ṅit (1.2.4) → guṇa blocked.
    Rule {
        id: "3.1.77",
        name: "tudAdiByaH SaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Tudadi) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("Sa");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.77", "tudAdiByaH SaH", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S → a
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
```

- [ ] **Step 5: Guard 3.1.68** so it declines when an apavāda vikaraṇa is already present. At the top of the `3.1.68` rule's `apply`, before `let before = p.snapshot();`:

```rust
            // Utsarga: fires only when no apavāda vikaraṇa (śyan 3.1.69 / śa
            // 3.1.77) is already present. Guarding on the vikaraṇa's presence
            // keeps śap the default without hard-coding a gaṇa, so curādi can
            // reuse śap later.
            if p.terms.len() > SHAP && p.terms[SHAP].has(Tag::Vikarana) {
                return false;
            }
```

- [ ] **Step 6: Run to verify the new tests pass and nothing regressed.**

Run: `mise run test`
Expected: PASS — `divadi_tudadi_present_third_singular` green; all 432 bhvādi golden forms and traces unchanged (bhvādi roots have neither `Divadi` nor `Tudadi`, so 3.1.69/3.1.77 decline and 3.1.68 fires as before).

- [ ] **Step 7: Commit.**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "feat(prakriya): divadi/tudadi vikaranas — 3.1.69 syan, 3.1.77 sa; 3.1.68 utsarga guard"
```

---

### Task 5: 8.2.77 hali ca — √div lengthening

The last rule: √div's short `i` lengthens before the `y` of śyan (div → dīv → dīvyati). Self-guarding on shape, fires for no other curated root.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (add `8.2.77` in the tripādī block, before `8.2.23`)

**Interfaces:**
- Consumes: the guṇa-blocked `div` aṅga (Task 3/4) and the vikaraṇa at `SHAP` (Task 4).
- Produces: rule `8.2.77`.

- [ ] **Step 1: Write the failing tests.**

```rust
    #[test]
    fn div_lengthens_before_syan() {
        assert_eq!(form_g("div", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "dIvyati");
        // laṅ: augment does not disturb the upadhā i.
        assert_eq!(form_g("div", Lakara::Lan, Purusha::Prathama, Vacana::Eka), "adIvyat");
    }
```

- [ ] **Step 2: Run to verify failure.**

Run: `mise run test -- -p panini-prakriya div_lengthens_before_syan`
Expected: FAIL (`divyati` / `adivyat` — short `i`, no 8.2.77 yet).

- [ ] **Step 3: Add 8.2.77** as a new `Rule` entry in the tripādī block, immediately **before** the `8.2.23` entry, using the Task-2 confirmed name:

```rust
    // 8.2.77 hali ca: a root ending in `r`/`v` with a short ik upadhā
    // lengthens that upadhā before a hal (8.2.76 rvorupadhāyā dīrghaḥ is the
    // anuvṛtti source). The only curated root reaching this is div, after
    // guṇa is blocked: div + śyan (y-initial) → dīv → dīvyati. Self-guards on
    // shape; no other curated root fires it (sev has an e-upadhā, vart ends
    // in t).
    Rule {
        id: "8.2.77",
        name: "hali ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let n = chars.len();
            if n < 2 {
                return false;
            }
            let final_c = chars[n - 1];
            let upadha = chars[n - 2];
            if !matches!(final_c, 'r' | 'v') || !matches!(upadha, 'i' | 'u') {
                return false;
            }
            let Some(next) = p.terms.get(SHAP).and_then(|t| t.text.chars().next()) else {
                return false;
            };
            if is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            let long = if upadha == 'i' { 'I' } else { 'U' };
            let mut s: String = chars[..n - 2].iter().collect();
            s.push(long);
            s.push(final_c);
            p.terms[ANGA].text = s;
            p.record("8.2.77", "hali ca", before);
            true
        },
    },
```

- [ ] **Step 4: Run to verify pass.**

Run: `mise run test`
Expected: PASS — `div_lengthens_before_syan` green; no bhvādi/other regressions (8.2.77 declines for every root but div).

- [ ] **Step 5: Commit.**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "feat(prakriya): 8.2.77 hali ca — div upadha lengthening (divyati)"
```

---

### Task 6: Golden paradigm — all 432 new forms

Pin every new surface form. The `every_form_validates_and_matches` and `known_nonforms_are_invalid` tests iterate `PARADIGM` automatically, so appending blocks extends coverage with no harness change.

**Files:**
- Modify: `crates/panini/tests/paradigm.rs` (append 48 blocks to `PARADIGM`)

**Interfaces:**
- Consumes: the full engine (Tasks 1–5).
- Produces: 432 pinned forms; `PARADIGM` grows from 48 to 96 rows.

- [ ] **Step 1: Append the 48 golden blocks** to the `PARADIGM` static in `crates/panini/tests/paradigm.rs`, before the closing `];`. These forms are template-projected from the bhvādi paradigms (the vikaraṇa is a-final like śap, so endings attach identically; √div carries `dIv` throughout via 8.2.77). Blocks, grouped by lakāra:

**laṭ:**

```rust
    ("div", "laT", ["dIvyati", "dIvyataH", "dIvyanti", "dIvyasi", "dIvyaTaH", "dIvyaTa", "dIvyAmi", "dIvyAvaH", "dIvyAmaH"]),
    ("naS", "laT", ["naSyati", "naSyataH", "naSyanti", "naSyasi", "naSyaTaH", "naSyaTa", "naSyAmi", "naSyAvaH", "naSyAmaH"]),
    ("kup", "laT", ["kupyati", "kupyataH", "kupyanti", "kupyasi", "kupyaTaH", "kupyaTa", "kupyAmi", "kupyAvaH", "kupyAmaH"]),
    ("man", "laT", ["manyate", "manyete", "manyante", "manyase", "manyeTe", "manyaDve", "manye", "manyAvahe", "manyAmahe"]),
    ("yuD", "laT", ["yuDyate", "yuDyete", "yuDyante", "yuDyase", "yuDyeTe", "yuDyaDve", "yuDye", "yuDyAvahe", "yuDyAmahe"]),
    ("vid", "laT", ["vidyate", "vidyete", "vidyante", "vidyase", "vidyeTe", "vidyaDve", "vidye", "vidyAvahe", "vidyAmahe"]),
    ("tud", "laT", ["tudati", "tudataH", "tudanti", "tudasi", "tudaTaH", "tudaTa", "tudAmi", "tudAvaH", "tudAmaH"]),
    ("liK", "laT", ["liKati", "liKataH", "liKanti", "liKasi", "liKaTaH", "liKaTa", "liKAmi", "liKAvaH", "liKAmaH"]),
    ("viS", "laT", ["viSati", "viSataH", "viSanti", "viSasi", "viSaTaH", "viSaTa", "viSAmi", "viSAvaH", "viSAmaH"]),
    ("juz", "laT", ["juzate", "juzete", "juzante", "juzase", "juzeTe", "juzaDve", "juze", "juzAvahe", "juzAmahe"]),
    ("vij", "laT", ["vijate", "vijete", "vijante", "vijase", "vijeTe", "vijaDve", "vije", "vijAvahe", "vijAmahe"]),
    ("gur", "laT", ["gurate", "gurete", "gurante", "gurase", "gureTe", "guraDve", "gure", "gurAvahe", "gurAmahe"]),
```

**laṅ:**

```rust
    ("div", "laN", ["adIvyat", "adIvyatAm", "adIvyan", "adIvyaH", "adIvyatam", "adIvyata", "adIvyam", "adIvyAva", "adIvyAma"]),
    ("naS", "laN", ["anaSyat", "anaSyatAm", "anaSyan", "anaSyaH", "anaSyatam", "anaSyata", "anaSyam", "anaSyAva", "anaSyAma"]),
    ("kup", "laN", ["akupyat", "akupyatAm", "akupyan", "akupyaH", "akupyatam", "akupyata", "akupyam", "akupyAva", "akupyAma"]),
    ("man", "laN", ["amanyata", "amanyetAm", "amanyanta", "amanyaTAH", "amanyeTAm", "amanyaDvam", "amanye", "amanyAvahi", "amanyAmahi"]),
    ("yuD", "laN", ["ayuDyata", "ayuDyetAm", "ayuDyanta", "ayuDyaTAH", "ayuDyeTAm", "ayuDyaDvam", "ayuDye", "ayuDyAvahi", "ayuDyAmahi"]),
    ("vid", "laN", ["avidyata", "avidyetAm", "avidyanta", "avidyaTAH", "avidyeTAm", "avidyaDvam", "avidye", "avidyAvahi", "avidyAmahi"]),
    ("tud", "laN", ["atudat", "atudatAm", "atudan", "atudaH", "atudatam", "atudata", "atudam", "atudAva", "atudAma"]),
    ("liK", "laN", ["aliKat", "aliKatAm", "aliKan", "aliKaH", "aliKatam", "aliKata", "aliKam", "aliKAva", "aliKAma"]),
    ("viS", "laN", ["aviSat", "aviSatAm", "aviSan", "aviSaH", "aviSatam", "aviSata", "aviSam", "aviSAva", "aviSAma"]),
    ("juz", "laN", ["ajuzata", "ajuzetAm", "ajuzanta", "ajuzaTAH", "ajuzeTAm", "ajuzaDvam", "ajuze", "ajuzAvahi", "ajuzAmahi"]),
    ("vij", "laN", ["avijata", "avijetAm", "avijanta", "avijaTAH", "avijeTAm", "avijaDvam", "avije", "avijAvahi", "avijAmahi"]),
    ("gur", "laN", ["agurata", "aguretAm", "aguranta", "aguraTAH", "agureTAm", "aguraDvam", "agure", "agurAvahi", "agurAmahi"]),
```

**loṭ:**

```rust
    ("div", "loT", ["dIvyatu", "dIvyatAm", "dIvyantu", "dIvya", "dIvyatam", "dIvyata", "dIvyAni", "dIvyAva", "dIvyAma"]),
    ("naS", "loT", ["naSyatu", "naSyatAm", "naSyantu", "naSya", "naSyatam", "naSyata", "naSyAni", "naSyAva", "naSyAma"]),
    ("kup", "loT", ["kupyatu", "kupyatAm", "kupyantu", "kupya", "kupyatam", "kupyata", "kupyAni", "kupyAva", "kupyAma"]),
    ("man", "loT", ["manyatAm", "manyetAm", "manyantAm", "manyasva", "manyeTAm", "manyaDvam", "manyE", "manyAvahE", "manyAmahE"]),
    ("yuD", "loT", ["yuDyatAm", "yuDyetAm", "yuDyantAm", "yuDyasva", "yuDyeTAm", "yuDyaDvam", "yuDyE", "yuDyAvahE", "yuDyAmahE"]),
    ("vid", "loT", ["vidyatAm", "vidyetAm", "vidyantAm", "vidyasva", "vidyeTAm", "vidyaDvam", "vidyE", "vidyAvahE", "vidyAmahE"]),
    ("tud", "loT", ["tudatu", "tudatAm", "tudantu", "tuda", "tudatam", "tudata", "tudAni", "tudAva", "tudAma"]),
    ("liK", "loT", ["liKatu", "liKatAm", "liKantu", "liKa", "liKatam", "liKata", "liKAni", "liKAva", "liKAma"]),
    ("viS", "loT", ["viSatu", "viSatAm", "viSantu", "viSa", "viSatam", "viSata", "viSAni", "viSAva", "viSAma"]),
    ("juz", "loT", ["juzatAm", "juzetAm", "juzantAm", "juzasva", "juzeTAm", "juzaDvam", "juzE", "juzAvahE", "juzAmahE"]),
    ("vij", "loT", ["vijatAm", "vijetAm", "vijantAm", "vijasva", "vijeTAm", "vijaDvam", "vijE", "vijAvahE", "vijAmahE"]),
    ("gur", "loT", ["guratAm", "guretAm", "gurantAm", "gurasva", "gureTAm", "guraDvam", "gurE", "gurAvahE", "gurAmahE"]),
```

**vidhiliṅ:**

```rust
    ("div", "viDiliN", ["dIvyet", "dIvyetAm", "dIvyeyuH", "dIvyeH", "dIvyetam", "dIvyeta", "dIvyeyam", "dIvyeva", "dIvyema"]),
    ("naS", "viDiliN", ["naSyet", "naSyetAm", "naSyeyuH", "naSyeH", "naSyetam", "naSyeta", "naSyeyam", "naSyeva", "naSyema"]),
    ("kup", "viDiliN", ["kupyet", "kupyetAm", "kupyeyuH", "kupyeH", "kupyetam", "kupyeta", "kupyeyam", "kupyeva", "kupyema"]),
    ("man", "viDiliN", ["manyeta", "manyeyAtAm", "manyeran", "manyeTAH", "manyeyATAm", "manyeDvam", "manyeya", "manyevahi", "manyemahi"]),
    ("yuD", "viDiliN", ["yuDyeta", "yuDyeyAtAm", "yuDyeran", "yuDyeTAH", "yuDyeyATAm", "yuDyeDvam", "yuDyeya", "yuDyevahi", "yuDyemahi"]),
    ("vid", "viDiliN", ["vidyeta", "vidyeyAtAm", "vidyeran", "vidyeTAH", "vidyeyATAm", "vidyeDvam", "vidyeya", "vidyevahi", "vidyemahi"]),
    ("tud", "viDiliN", ["tudet", "tudetAm", "tudeyuH", "tudeH", "tudetam", "tudeta", "tudeyam", "tudeva", "tudema"]),
    ("liK", "viDiliN", ["liKet", "liKetAm", "liKeyuH", "liKeH", "liKetam", "liKeta", "liKeyam", "liKeva", "liKema"]),
    ("viS", "viDiliN", ["viSet", "viSetAm", "viSeyuH", "viSeH", "viSetam", "viSeta", "viSeyam", "viSeva", "viSema"]),
    ("juz", "viDiliN", ["juzeta", "juzeyAtAm", "juzeran", "juzeTAH", "juzeyATAm", "juzeDvam", "juzeya", "juzevahi", "juzemahi"]),
    ("vij", "viDiliN", ["vijeta", "vijeyAtAm", "vijeran", "vijeTAH", "vijeyATAm", "vijeDvam", "vijeya", "vijevahi", "vijemahi"]),
    ("gur", "viDiliN", ["gureta", "gureyAtAm", "gureran", "gureTAH", "gureyATAm", "gureDvam", "gureya", "gurevahi", "guremahi"]),
```

(Match the existing block formatting; `rustfmt` will re-wrap long arrays — run `mise run fmt`.)

- [ ] **Step 2: Run the paradigm test.**

Run: `mise run test -- -p panini`
Expected: PASS — `every_form_validates_and_matches` covers all 96 rows (864 forms). If any row FAILs, the printed `expected VALID for <form>` names the exact cell; cross-check that form against the reference (ashtadhyayi.com) — a mismatch is either a template-projection slip in the plan (fix the golden value) or a real engine bug (stop and debug via systematic-debugging). The most likely suspects are the three least-common roots (`gur`, `vij`, and √div's laṅ) — verify those against the reference first.

- [ ] **Step 3: Run fmt and the full suite.**

Run: `mise run fmt && mise run test`
Expected: PASS.

- [ ] **Step 4: Commit.**

```bash
git add crates/panini/tests/paradigm.rs
git commit -m "test(paradigm): 432 golden forms for divadi + tudadi (864 total)"
```

---

### Task 7: Ordered-trace anchors

Pin the full ordered sūtra sequence for five representative derivations, covering śyan, śa, the guṇa-block, √div's 8.2.77, both padas, and a vidhiliṅ new-gaṇa chain.

**Files:**
- Modify: `crates/panini/tests/trace.rs` (add five `#[test]` fns)

**Interfaces:**
- Consumes: `trace_for` (existing helper) and the full engine.

- [ ] **Step 1: Add the five trace tests** to `crates/panini/tests/trace.rs`:

```rust
#[test]
fn divyati_trace_is_the_syan_block_lengthen_path() {
    // div laṭ P 3sg: śyan (3.1.69) → apit → ṅit (1.2.4, 2nd application);
    // 7.3.84/7.3.86 blocked (no record); 8.2.77 lengthens div → dīv.
    assert_eq!(
        trace_for("dIvyati"),
        vec!["1.3.78", "3.4.78", "1.3.9", "3.1.69", "1.3.9", "1.2.4", "8.2.77"]
    );
}

#[test]
fn tudati_trace_is_the_sa_block_path() {
    // tud laṭ P 3sg: śa (3.1.77) → ṅit (1.2.4); 7.3.86 blocked (no todati).
    assert_eq!(
        trace_for("tudati"),
        vec!["1.3.78", "3.4.78", "1.3.9", "3.1.77", "1.3.9", "1.2.4"]
    );
}

#[test]
fn manyate_trace_is_the_syan_atmanepada_path() {
    // man laṭ Ā 3sg: laBate's path with 3.1.68→3.1.69 and the second 1.2.4
    // (śyan ṅit) appended.
    assert_eq!(
        trace_for("manyate"),
        vec!["1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.69", "1.3.9", "1.2.4"]
    );
}

#[test]
fn jusate_trace_is_the_sa_atmanepada_block_path() {
    // juṣ laṭ Ā 3sg: śa path; 7.3.86 blocked (juṣate NOT joṣate).
    assert_eq!(
        trace_for("juzate"),
        vec!["1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.77", "1.3.9", "1.2.4"]
    );
}

#[test]
fn kupyet_trace_is_the_syan_vidhilin_path() {
    // kup vidhiliṅ P 3sg: bhavet's yāsuṭ chain with śyan instead of śap, the
    // second 1.2.4 (śyan ṅit), and NO 7.3.84/6.1.78 (guṇa blocked; kup has no
    // guṇable final and its upadhā guṇa is blocked).
    assert_eq!(
        trace_for("kupyet"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.100", "3.4.103", "3.1.69", "1.3.9", "1.2.4",
            "7.2.79", "7.2.80", "6.1.87", "6.1.66"
        ]
    );
}
```

- [ ] **Step 2: Run the trace tests.**

Run: `mise run test -- -p panini --test trace`
Expected: PASS. If a vector mismatches, the assertion prints the actual trace. Reconcile against `TINANTA_RULES` order (the source of truth) and the reference: the expected vectors above were derived by hand from the existing `bhavati`/`laBate`/`bhavet` pins plus the new rules' positions. A genuine discrepancy means a rule is mis-ordered — fix the rule position, not the test, unless the reference confirms the test is wrong.

- [ ] **Step 3: Commit.**

```bash
git add crates/panini/tests/trace.rs
git commit -m "test(trace): divadi/tudadi ordered-trace anchors (syan, sa, block, 8.2.77, vidhilin)"
```

---

### Task 8: Cross-gaṇa / guṇa-block / wrong-pada negatives

Pin that the wrong-vikaraṇa, should-have-been-blocked-guṇa, and wrong-pada non-forms return INVALID — these are the tests that kill the new guard mutants.

**Files:**
- Modify: `crates/panini/tests/paradigm.rs` (extend `known_nonforms_are_invalid`)

- [ ] **Step 1: Add the negatives** to the `known_nonforms_are_invalid` list in `crates/panini/tests/paradigm.rs`:

```rust
        // Wrong vikaraṇa: divādi/tudādi roots take śyan/śa, not śap, and
        // bhvādi does not take śyan.
        "divati",  // div with śap instead of śyan
        "tudyati", // tud with śyan instead of śa
        "Bavyati", // BU (bhvādi) with a śyan it has no claim to
        "naSati",  // naś with śap
        "kupati",  // kup with śap
        // Guṇa should have been blocked (1.1.5): these are the guṇa'd forms.
        "kopyati", // kup guṇa'd — 7.3.86 must be blocked by śyan's ṅit
        "todati",  // tud guṇa'd — 7.3.86 must be blocked by śa's ṅit
        "jozate",  // juṣ guṇa'd — block under ātmanepada too
        "devyati", // div guṇa'd (before 8.2.77): guṇa must be blocked
        // Wrong pada: the root's pada tag gates the whole derivation.
        "manyati", // atmanepadin divādi root with a parasmaipada ending
        "vidyati", // atmanepadin divādi root, parasmaipada ending
        "tudate",  // parasmaipada tudādi root with an atmanepada ending
```

- [ ] **Step 2: Run the test.**

Run: `mise run test -- -p panini known_nonforms_are_invalid`
Expected: PASS (each listed form is non-derivable in the covered grammar). If any returns VALID, that is a real over-generation bug — debug it (a missing guard), do not delete the negative.

- [ ] **Step 3: Commit.**

```bash
git add crates/panini/tests/paradigm.rs
git commit -m "test(paradigm): cross-gana, guna-block, and wrong-pada negatives"
```

---

### Task 9: Mutation testing and docs

Confirm the new guards are genuinely pinned, and update the three docs that state coverage/scope.

**Files:**
- Modify: `AGENTS.md`, `README.md`, `docs/ARCHITECTURE.md`

**Interfaces:** none (verification + docs).

- [ ] **Step 1: Run mutation testing.**

Run: `MISE_ENV=dev mise install && mise run mutants`
Expected: no NEW surviving mutants in the added guard arms. Specifically check that mutating any of these is caught by the Task 6–8 tests: the `Tag::Divadi`/`Tag::Tudadi` guards (3.1.69/3.1.77), the `!has(Pit)` / `has(Ngit)` conditions (second 1.2.4 and the 7.3.84/86 guards), the `3.1.68` vikaraṇa-present guard, and 8.2.77's `matches!(final_c, 'r' | 'v')` / `matches!(upadha, 'i' | 'u')` / `is_vowel(next)` guards. If a mutant survives, add a targeted negative (Task 8 style) or golden cell (Task 6 style) that distinguishes it, then re-run.

- [ ] **Step 2: Update AGENTS.md.** Change the golden-test description (currently `432 forms across laṭ/laṅ/loṭ/vidhiliṅ × parasmaipada/ātmanepada`):

```
  (`crates/panini/tests/paradigm.rs`, 864 forms across
    laṭ/laṅ/loṭ/vidhiliṅ × parasmaipada/ātmanepada × bhvādi/divādi/tudādi)
```

Also review the "Non-Goals"/scope notes if any name only bhvādi, and adjust to name the three gaṇas.

- [ ] **Step 3: Update README.md.** Change the Scope paragraph (currently `*bhvādi* (gaṇa 1)`):

```
Finite verbs (*tiṅanta*), three gaṇas — *bhvādi* (1), *divādi* (4), *tudādi*
(6) — *parasmaipada* and *ātmanepada* (pada taken from each root's tag), over
a curated 24-root set, in four lakāras: *laṭ* (present), *laṅ* (imperfect),
*loṭ* (imperative), and *vidhiliṅ* (optative).
```

- [ ] **Step 4: Update docs/ARCHITECTURE.md** if it names a single gaṇa anywhere; note that gaṇa is carried as an aṅga tag (`Tag::Divadi`/`Tag::Tudadi`, mirroring `Atmanepadin`) read by 3.1.69/3.1.77, and that the vikaraṇa is selected by 3.1.68/69/77.

- [ ] **Step 5: Run the full suite one last time.**

Run: `mise run fmt-check && mise run lint && mise run test`
Expected: PASS.

- [ ] **Step 6: Commit.**

```bash
git add AGENTS.md README.md docs/ARCHITECTURE.md
git commit -m "docs: three-gana scope (bhvadi/divadi/tudadi), 864 golden forms"
```

---

## Self-Review

**Spec coverage** (against `docs/superpowers/specs/2026-07-21-divadi-tudadi-ganas-design.md`):
- Data layer (Gana variants, 12 roots, tsv) → Task 1. ✓
- Threading the gaṇa via aṅga tag → Task 3 (tags) + Task 4 (derive). ✓
- 3.1.69 śyan, 3.1.77 śa → Task 4. ✓
- 3.1.68 Pit + utsarga guard → Task 3 (Pit) + Task 4 (guard). ✓
- Second 1.2.4 application → Task 3. ✓
- 1.1.5 guards on 7.3.84/7.3.86 → Task 3. ✓
- 8.2.77 hali ca → Task 5. ✓
- Analyzer/facade/CLI unchanged → confirmed (no task; brute-force analyzer + pada-carrying facade already cover new roots). ✓
- Golden 864 → Task 6. Trace anchors → Task 7. Negatives → Task 8. Mutation + docs → Task 9. Name verification → Task 2. ✓

**Placeholder scan:** the only deferred value is the confirmed sūtra-name spellings (Task 2 Step 3), which is a genuine reference-verification step, not a code placeholder — every code block is complete. No "TBD"/"handle edge cases"/"similar to Task N".

**Type consistency:** `Tag::Pit`/`Tag::Divadi`/`Tag::Tudadi` (Task 3) are the exact identifiers consumed in Tasks 4–5; `Gana::Divadi`/`Gana::Tudadi` (Task 1) match the `derive` match arms (Task 4); `form_g` (Task 4) is reused in Task 5; `SHAP`/`ANGA`/`ENDING` are the existing constants. The second 1.2.4 guards on `SHAP` (the vikaraṇa), matching the 1.1.5 guards — consistent throughout.
