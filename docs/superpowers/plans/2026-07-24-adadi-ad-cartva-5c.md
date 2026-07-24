# Adādi √ad (cartva) — Slice 5c Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the consonant-final root √ad (adādi, parasmaipada) across all four lakāras, landing the engine's first internal junction sandhi (8.4.55 cartva) plus two small √ad-specific rules (6.4.101 her dhiḥ, laṅ a-augment).

**Architecture:** √ad is registered as one `Dhatu` in `panini-data`. Three self-guarding `Rule`s are added to `TINANTA_RULES` in `panini-prakriya/src/tinanta.rs`: 6.4.101 and the laṅ a-augment in the aṅga layer, 8.4.55 cartva as the new final (tripādī) rule. Correctness is pinned by a 36-form golden block (`paradigm.rs`) and three ordered-trace pins (`trace.rs`), each rule guarded so it fires only on √ad's consonant junction and leaves every existing form untouched.

**Tech Stack:** Rust (edition per workspace), `mise` task runner, `cargo` test/mutants. SLP1 internal representation. Reference of record: ashtadhyayi.com.

## Global Constraints

- SLP1 is the only internal representation; no transliteration outside `panini-lipi`. (SLP1 note: `t`=त, `T`=थ, `d`=द, `D`=ध, `A`=आ, `H`=visarga.)
- `#![forbid(unsafe_code)]` in every non-fuzz crate.
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, never as a branch inside `derive`. `derive` carries no grammar branches.
- Grammar changes are gated by the golden paradigm test (`crates/panini/tests/paradigm.rs`) and the ordered-trace test (`crates/panini/tests/trace.rs`). Surface forms and trace order there are the source of truth; sūtra ids/names in traces must match ashtadhyayi.com.
- Every root/form/sūtra-id is verified against ashtadhyayi.com before it is committed (reference-first discipline).
- Build/test via `mise run build | test | lint | fmt-check | mutants`. Never install Rust globally.
- Golden paradigm coverage moves **936 → 972** (√ad × 36 cells). Root count **26 → 27**.

---

## Task 0: Pin √ad's paradigm and sūtra ids against the reference

This is a research task, not a code task — it de-risks the two open items (the a-augment sūtra id, and the exact 36 forms) before any rule is written. No commit; its output is written into the later tasks' test data.

**Files:** none (records findings in this plan's task notes / commit messages downstream).

- [ ] **Step 1: Fetch √ad's parasmaipada paradigm.** Open ashtadhyayi.com's dhātu page for अद् (gaṇa 2, *adādi*) and record all 36 forms (laṭ, laṅ, loṭ, vidhiliṅ; 3s 3d 3p / 2s 2d 2p / 1s 1d 1p) in SLP1. Cross-check against the design's provisional table:

| Lakāra    | forms (SLP1) |
|-----------|--------------|
| laṭ (`laT`)      | `atti attaH adanti` / `atsi atTaH atTa` / `admi advaH admaH` |
| laṅ (`laN`)      | `Adat AttAm Adan` / `AdaH Attam Atta` / `Adam Adva Adma` |
| loṭ (`loT`)      | `attu attAm adantu` / `adDi attam atta` / `adAni adAva adAma` |
| vidhiliṅ (`viDiliN`) | `adyAt adyAtAm adyuH` / `adyAH adyAtam adyAta` / `adyAm adyAva adyAma` |

Expected: the table matches. If any cell differs, the reference wins — update the table in Tasks 4–7 accordingly and note the discrepancy.

- [ ] **Step 2: Resolve the laṅ-singular a-augment sūtra id.** From ashtadhyayi.com's prakriya for **अदत्** (laṅ 3sg) and **अदः** (laṅ 2sg), record the exact sūtra id + name that inserts the `a` between `Ad` and the ending. This becomes the `id`/`name` of the rule in Task 6 and the trace pin in Task 9.
  - Expected: a single āgama sūtra/vārttika (a √ad-specific aṭ-āgama). If the reference instead derives `Adat` by a mechanism *other* than an inserted `a` (e.g. a non-elision), re-derive Task 6's rule to match the reference's steps and adjust the Task 9 trace. Do not proceed to Task 6 until this id is known.

- [ ] **Step 3: Confirm the 6.4.101 and 8.4.55 chains.** For **अद्धि** (loṭ 2sg) confirm the reference traces 3.4.87 → (6.4.105 declines) → **6.4.101 *her dhiḥ***. For **अत्ति** (laṭ 3sg) confirm it ends in **8.4.55 *khari ca***. Record the exact `name` strings (SLP1/transliterated) as ashtadhyayi.com renders them, for the `record(...)` calls.
  - Expected: names like `her DiH` / `KariM ca` — match whatever the reference and the existing codebase convention (e.g. `"KaravasAnayor visarjanIyaH"`) use for transliteration.

---

## Task 1: Register √ad in the dhātu data

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (the `DHATUS` array, after the √vā entry ~line 189; and the `dhatus().len()` assertion ~line 251)

**Interfaces:**
- Produces: a `Dhatu { code: "ad", gana: Gana::Adadi, pada: Pada::Parasmaipada, artha: "BakzaRe" }` retrievable via `dhatus().iter().find(|d| d.code == "ad")`.

- [ ] **Step 1: Write the failing test.** Add to the `#[cfg(test)]` module in `crates/panini-data/src/lib.rs`, and update the existing length assertion:

```rust
#[test]
fn ad_is_registered_as_adadi_parasmaipada() {
    let ad = dhatus().iter().find(|d| d.code == "ad").expect("√ad present");
    assert!(matches!(ad.gana, Gana::Adadi));
    assert!(matches!(ad.pada, Pada::Parasmaipada));
    assert_eq!(ad.artha, "BakzaRe");
}
```

Also change the existing count assertion from `assert_eq!(dhatus().len(), 26);` to `assert_eq!(dhatus().len(), 27);`.

- [ ] **Step 2: Run test to verify it fails.**

Run: `mise run test -- -p panini-data ad_is_registered`
Expected: FAIL — `√ad present` panic (root not found) and/or the length assertion fails at 26.

- [ ] **Step 3: Add the data entry.** Insert into the `DHATUS` array immediately after the √vā entry (keep it inside the adādi group; update the group comment to name √ad):

```rust
    Dhatu {
        code: "ad",
        gana: Gana::Adadi,
        pada: Pada::Parasmaipada,
        artha: "BakzaRe",
    },
```

- [ ] **Step 4: Run tests to verify they pass.**

Run: `mise run test -- -p panini-data`
Expected: PASS (both the new test and the updated length assertion).

- [ ] **Step 5: Commit.**

```bash
git add crates/panini-data/src/lib.rs
git commit -m "feat(data): register √ad (adādi, parasmaipada)"
```

---

## Task 2: 8.4.55 khari ca (cartva) — the junction sandhi

Add cartva as the new final rule in `TINANTA_RULES`. This is the headline; do it first among the rules so the laṭ cells (`atti`, `atsi`, …) light up, then layer the laṅ/loṭ specials on top.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (insert a new `Rule` immediately before the closing `];` of `TINANTA_RULES`, after the 8.3.15 rule ~line 1338; add a unit test in the `#[cfg(test)]` module)

**Interfaces:**
- Consumes: `ANGA` (index 0) and the last term (the ending); `is_vowel`, `is_jhal`, `is_khar` char classifiers (add whichever do not yet exist alongside the existing `is_vowel`).
- Produces: rule `id = "8.4.55"`, `name = "KariM ca"` (confirm exact spelling in Task 0 Step 3). Changes a jhal at the aṅga's final position to its corresponding car (voiceless unaspirated) when the following segment is a khar.

- [ ] **Step 1: Write the failing test.** Add to the `#[cfg(test)]` module in `tinanta.rs` (reuse the existing `form(code, purusha, vacana)` / derivation helpers — mirror the nearby `divadi_tudadi_*` / `adadi_*` tests):

```rust
#[test]
fn cartva_turns_d_to_t_before_khar() {
    // √ad laṭ: 3sg atti (d+t), 2sg atsi (d+s), 2pl atTa (d+T).
    assert_eq!(form_lakara("ad", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "atti");
    assert_eq!(form_lakara("ad", Lakara::Lat, Purusha::Madhyama, Vacana::Eka), "atsi");
    assert_eq!(form_lakara("ad", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu), "atTa");
    // Not before a non-khar (m/v) or a vowel: admi, advaH, adanti stay.
    assert_eq!(form_lakara("ad", Lakara::Lat, Purusha::Uttama, Vacana::Eka), "admi");
    assert_eq!(form_lakara("ad", Lakara::Lat, Purusha::Prathama, Vacana::Bahu), "adanti");
}
```

If a `form_lakara(code, lakara, purusha, vacana)` helper does not already exist in the test module, add a thin one next to the existing `form(...)` helper that threads the `Lakara` through `Context::new(...)` and returns the derived surface string.

- [ ] **Step 2: Run test to verify it fails.**

Run: `mise run test -- -p panini-prakriya cartva_turns_d_to_t`
Expected: FAIL — `atti`/`atsi`/`atTa` derive as `adti`/`adsi`/`adTa` (cartva not yet applied).

- [ ] **Step 3: Add the classifiers (if missing).** Near the existing `is_vowel` helper in `tinanta.rs`, add:

```rust
/// A jhal (obstruent) — the set 8.4.55's target ranges over. For this slice
/// only `d` is exercised, but the classifier is written generally.
fn is_jhal(c: char) -> bool {
    matches!(
        c,
        'k'|'K'|'g'|'G'|'c'|'C'|'j'|'J'|'w'|'W'|'q'|'Q'|'t'|'T'|'d'|'D'|'p'|'P'|'b'|'B'|'S'|'z'|'s'|'h'
    )
}

/// A khar (voiceless obstruent) — the trigger of 8.4.55 (khari ca).
fn is_khar(c: char) -> bool {
    matches!(c, 'k'|'K'|'c'|'C'|'w'|'W'|'t'|'T'|'p'|'P'|'S'|'z'|'s')
}

/// The car (voiceless unaspirated) substitute of a jhal, per 8.4.55.
/// Only `d → t` is exercised this slice; extend as later roots demand.
fn cartva_of(c: char) -> Option<char> {
    match c {
        'd' | 'D' | 't' | 'T' => Some('t'),
        'g' | 'G' | 'k' | 'K' => Some('k'),
        'b' | 'B' | 'p' | 'P' => Some('p'),
        'j' | 'J' | 'c' | 'C' => Some('c'),
        'q' | 'Q' | 'w' | 'W' => Some('w'),
        _ => None,
    }
}
```

- [ ] **Step 4: Add the rule.** Insert immediately before the closing `];` of `TINANTA_RULES` (after the 8.3.15 rule), so it is the last — and latest-ordered — tripādī rule:

```rust
    // 8.4.55 khari ca (cartva): a jhal at the aṅga's final position, meeting a
    // khar across the root+ending junction, becomes its car (voiceless
    // unaspirated). √ad's d before ti/tas/si/tha → t: atti, attaH, atsi, atTa.
    // The engine's first internal junction sandhi; general, reused by every
    // later gaṇa/subanta slice. Placed last: latest tripādī rule (8.4 > 8.3).
    Rule {
        id: "8.4.55",
        name: "KariM ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // The following segment is the first char of the first non-empty
            // term after the aṅga (the ending; śap, if present, is luk'd/empty).
            let next = p
                .terms
                .iter()
                .skip(ANGA + 1)
                .find_map(|t| t.text.chars().next());
            let Some(next) = next else { return false };
            if !is_khar(next) {
                return false;
            }
            let Some(last) = p.terms[ANGA].text.chars().last() else {
                return false;
            };
            if !is_jhal(last) {
                return false;
            }
            let Some(sub) = cartva_of(last) else { return false };
            if sub == last {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ANGA].text.chars().collect();
            s.pop();
            s.push(sub);
            p.terms[ANGA].text = s.into_iter().collect();
            p.record("8.4.55", "KariM ca", before);
            true
        },
    },
```

- [ ] **Step 5: Run tests to verify they pass.**

Run: `mise run test -- -p panini-prakriya cartva_turns_d_to_t`
Expected: PASS. Then `mise run test -- -p panini-prakriya` — no existing test regresses (cartva fires only on a jhal-final aṅga before a khar; every current root is vowel/ā-final before its ending).

- [ ] **Step 6: Commit.**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "feat(prakriya): 8.4.55 khari ca (cartva) — first internal junction sandhi"
```

---

## Task 3: 6.4.101 her dhiḥ — loṭ 2sg addhí

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (insert a `Rule` immediately after the 6.4.105 *ato heḥ* rule ~line 1252; add a unit test)

**Interfaces:**
- Consumes: the loṭ 2sg ending is `hi` (already set by 3.4.87 *ser hyapic ca*); `is_jhal` (from Task 2); `ENDING_PRE_SHAP` / the ending term.
- Produces: rule `id = "6.4.101"`, `name = "her DiH"` (confirm in Task 0). Converts ending `hi` → `Di` after a jhal-final aṅga.

- [ ] **Step 1: Write the failing test.**

```rust
#[test]
fn her_dhih_gives_addhi_for_consonant_root() {
    // √ad loṭ 2sg: 3.4.87 si→hi, 6.4.105 declines (d, not short a),
    // 6.4.101 hi→Di → adDi.
    assert_eq!(form_lakara("ad", Lakara::Lot, Purusha::Madhyama, Vacana::Eka), "adDi");
    // Thematic root unaffected: √bhū loṭ 2sg is Bava (hi luk'd by 6.4.105).
    assert_eq!(form_lakara("BU", Lakara::Lot, Purusha::Madhyama, Vacana::Eka), "Bava");
}
```

- [ ] **Step 2: Run test to verify it fails.**

Run: `mise run test -- -p panini-prakriya her_dhih_gives_addhi`
Expected: FAIL — √ad loṭ 2sg derives as `adhi` (hi survives unchanged; 6.4.101 missing). The √bhū assertion already passes.

- [ ] **Step 3: Add the rule.** Insert immediately after the 6.4.105 *ato heḥ* rule (so 6.4.101 acts only when 6.4.105 has declined — both are heḥ-operations on the loṭ 2sg):

```rust
    // 6.4.101 hujhalbhyo her dhiḥ: the loṭ 2sg `hi` becomes `Di` after a
    // jhal-final aṅga (and after √hu, out of scope). √ad: 6.4.105 ato heḥ
    // declined (its aṅga ends in `d`, not a short `a`), so `hi` survives to
    // here → adDi. Thematic roots never reach this: their `hi` is luk'd by
    // 6.4.105 behind śap's `a`.
    Rule {
        id: "6.4.101",
        name: "her DiH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms[ENDING_PRE_SHAP].text != "hi" {
                return false;
            }
            let Some(last) = p.terms[ANGA].text.chars().last() else {
                return false;
            };
            if !is_jhal(last) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "Di".into();
            p.record("6.4.101", "her DiH", before);
            true
        },
    },
```

- [ ] **Step 4: Run tests to verify they pass.**

Run: `mise run test -- -p panini-prakriya her_dhih_gives_addhi`
Expected: PASS. Then `mise run test -- -p panini-prakriya` — √yā loṭ 2sg (`yAhi`, ā-final, not jhal) and all thematic loṭ 2sg forms are untouched.

- [ ] **Step 5: Commit.**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "feat(prakriya): 6.4.101 her dhiH — loṭ 2sg addhí for consonant-final √ad"
```

---

## Task 4: laṅ singular a-augment — ā́daḥ / ā́dat

Uses the sūtra id resolved in Task 0 Step 2. The placeholder `id`/`name` below (`"<AD_AUG_ID>"` / `"<AD_AUG_NAME>"`) MUST be replaced with the reference's actual values before committing — do not commit the placeholders.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (insert a `Rule` immediately after the 6.4.72 *āḍ ajādīnām* rule ~line 792; add a unit test)

**Interfaces:**
- Consumes: `Tag::Adadi` on the aṅga; the laṅ singular endings are single-char `s` (2sg) / `t` (3sg) at this point; the augmented aṅga is `Ad`; `is_vowel` (existing) to test consonant-final aṅga.
- Produces: rule `id = "<AD_AUG_ID>"`. Prefixes `a` to the ending (`t`→`at`, `s`→`as`) so the word is vowel-final before the tripādī.

- [ ] **Step 1: Write the failing test.**

```rust
#[test]
fn adadi_lan_singular_a_augment() {
    // √ad laṅ 3sg Adat, 2sg AdaH — the inserted `a` blocks the saṃyogānta
    // collapse (Adt/Ads → Ad) and cartva (d now before `a`, not a khar).
    assert_eq!(form_lakara("ad", Lakara::Lan, Purusha::Prathama, Vacana::Eka), "Adat");
    assert_eq!(form_lakara("ad", Lakara::Lan, Purusha::Madhyama, Vacana::Eka), "AdaH");
    // Dual/plural keep the direct junction (multi-char endings, no a-augment):
    // cartva gives Attam/AttAm/Atta; 1sg Adam untouched.
    assert_eq!(form_lakara("ad", Lakara::Lan, Purusha::Madhyama, Vacana::Dvi), "Attam");
    assert_eq!(form_lakara("ad", Lakara::Lan, Purusha::Prathama, Vacana::Dvi), "AttAm");
    assert_eq!(form_lakara("ad", Lakara::Lan, Purusha::Uttama, Vacana::Eka), "Adam");
}
```

- [ ] **Step 2: Run test to verify it fails.**

Run: `mise run test -- -p panini-prakriya adadi_lan_singular_a_augment`
Expected: FAIL — 3sg/2sg derive as `Ad` (8.2.23 collapses `Adt`/`Ads`), not `Adat`/`AdaH`. The dual `Attam`/`AttAm` already pass (cartva from Task 2); `Adam` already passes.

- [ ] **Step 3: Add the rule.** Insert immediately after the 6.4.72 rule (aṅga layer, before the tripādī, so the inserted `a` reaches 8.2.23 already in place). Replace the placeholder id/name with Task 0's values:

```rust
    // <AD_AUG_ID> <AD_AUG_NAME>: √ad prefixes aṭ (`a`) to a laṅ singular
    // consonant ending (2sg s, 3sg t). Without it, Ad+s / Ad+t are word-final
    // conjuncts that 8.2.23 saṃyogāntasya lopaḥ would strip to bare Ad,
    // collapsing 2sg=3sg=1sg-stem. The inserted `a` makes the word vowel-final:
    // 8.2.23 declines, and cartva (8.4.55) skips the `d` (now before `a`, not a
    // khar) → Adat, Adas→AdaH. Guarded structurally (Tag::Adadi ∧ laṅ ∧
    // consonant-final aṅga ∧ single-char s/t ending): in the current root set
    // that is exactly √ad (√yā/√vā are ā-final). Retighten when √vas lands.
    Rule {
        id: "<AD_AUG_ID>",
        name: "<AD_AUG_NAME>",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lan) || !p.terms[ANGA].has(Tag::Adadi) {
                return false;
            }
            // Consonant-final aṅga only (ā-final √yā/√vā never insert).
            let Some(anga_last) = p.terms[ANGA].text.chars().last() else {
                return false;
            };
            if is_vowel(anga_last) {
                return false;
            }
            // Single-consonant ending: 2sg `s` / 3sg `t` (not the multi-char
            // tam/tAm/ta of dual/plural).
            let e = &p.terms[ENDING_PRE_SHAP].text;
            if e.chars().count() != 1 || !matches!(e.as_str(), "s" | "t") {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = format!("a{e}");
            p.record("<AD_AUG_ID>", "<AD_AUG_NAME>", before);
            true
        },
    },
```

- [ ] **Step 4: Run tests to verify they pass.**

Run: `mise run test -- -p panini-prakriya adadi_lan_singular_a_augment`
Expected: PASS. Then `mise run test -- -p panini-prakriya` — no existing laṅ form regresses (guard requires `Tag::Adadi` + consonant-final aṅga; √yā/√vā are ā-final, thematic roots lack the tag / are a-final).

- [ ] **Step 5: Commit.**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "feat(prakriya): laṅ singular a-augment — √ad AdaH/Adat"
```

---

## Task 5: √ad golden paradigm block (all 36 cells)

Now the three rules are in place, pin the full paradigm. This is the primary gate.

**Files:**
- Modify: `crates/panini/tests/paradigm.rs` (add four tuples to the `PARADIGM` array, in the adādi group after the √vā `viDiliN` row ~line 1055)

**Interfaces:**
- Consumes: the `PARADIGM` tuple shape `(&str root, &str lakara_label, [&str; 9])`, order `3s 3d 3p / 2s 2d 2p / 1s 1d 1p`; labels `"laT"`, `"laN"`, `"loT"`, `"viDiliN"`.

- [ ] **Step 1: Add the golden rows (the failing test).** Insert after the √vā `viDiliN` tuple, using the Task 0-verified forms:

```rust
    (
        "ad",
        "laT",
        [
            "atti", "attaH", "adanti", "atsi", "atTaH", "atTa", "admi", "advaH", "admaH",
        ],
    ),
    (
        "ad",
        "laN",
        [
            "Adat", "AttAm", "Adan", "AdaH", "Attam", "Atta", "Adam", "Adva", "Adma",
        ],
    ),
    (
        "ad",
        "loT",
        [
            "attu", "attAm", "adantu", "adDi", "attam", "atta", "adAni", "adAva", "adAma",
        ],
    ),
    (
        "ad",
        "viDiliN",
        [
            "adyAt", "adyAtAm", "adyuH", "adyAH", "adyAtam", "adyAta", "adyAm", "adyAva", "adyAma",
        ],
    ),
```

- [ ] **Step 2: Run the golden test.**

Run: `mise run test -- -p panini paradigm`
Expected: PASS for all four √ad rows (36 cells). If any cell mismatches, compare the derived form to the reference (Task 0); a mismatch is either a wrong golden (fix the string) or a rule gap (return to Tasks 2–4). Do not "fix" by editing a golden to match a wrong derivation.

- [ ] **Step 3: Verify total coverage.** Confirm the paradigm now asserts 972 forms (was 936). If the test harness prints/asserts a count, update it; otherwise confirm 4 new rows × 9 = 36 added.

Run: `mise run test -- -p panini`
Expected: PASS, full `panini` suite green.

- [ ] **Step 4: Commit.**

```bash
git add crates/panini/tests/paradigm.rs
git commit -m "test(paradigm): √ad full 36-cell block (936→972)"
```

---

## Task 6: Ordered-trace pins (atti, adDi, Adat)

Three pins, one per new mechanism. Sūtra ids/names must match ashtadhyayi.com (Task 0).

**Files:**
- Modify: `crates/panini/tests/trace.rs` (add three `#[test]` fns near the existing adādi trace tests ~line 357; use the `trace_for(surface)` helper that returns `Vec<&str>` of rule ids in order)

**Interfaces:**
- Consumes: `trace_for(&str) -> Vec<String>` (or `Vec<&str>`), matching the existing `yayuh_trace_*` tests.

- [ ] **Step 1: Write the trace tests.** Fill each expected vector by reading the actual trace the engine now emits (run `trace_for` once, confirm each id against the reference, then pin). The comments below state the intended chain; the exact prefix of shared rules should mirror the neighbouring adādi traces (`1.3.78`, `3.4.78`, `3.1.68`, `1.3.9`, `2.4.72`, …).

```rust
#[test]
fn atti_trace_ends_in_cartva() {
    // √ad laṭ 3sg: śap inserted (3.1.68) then luk'd (2.4.72), ti direct on d,
    // cartva (8.4.55) d→t → atti.
    let t = trace_for("atti");
    assert_eq!(t.last().copied(), Some("8.4.55"));
    assert!(t.contains(&"2.4.72"));
}

#[test]
fn addhi_trace_uses_her_dhih_after_ato_heh_declines() {
    // √ad loṭ 2sg: 3.4.87 si→hi, then 6.4.101 her dhiH (6.4.105 declines on d).
    let t = trace_for("adDi");
    let i87 = t.iter().position(|r| *r == "3.4.87").expect("3.4.87 present");
    let i101 = t.iter().position(|r| *r == "6.4.101").expect("6.4.101 present");
    assert!(i87 < i101, "3.4.87 must precede 6.4.101");
    assert!(!t.contains(&"6.4.105"), "6.4.105 declines, not recorded");
}

#[test]
fn adat_trace_a_augment_precedes_and_blocks_cartva() {
    // √ad laṅ 3sg: augment (6.4.71/6.4.72), then the a-augment; cartva does
    // NOT fire (d now before `a`), and 8.2.23 does NOT fire (vowel-final).
    let t = trace_for("Adat");
    assert!(t.contains(&"<AD_AUG_ID>"), "a-augment recorded");
    assert!(!t.contains(&"8.4.55"), "cartva blocked by inserted a");
    assert!(!t.contains(&"8.2.23"), "saṃyogānta blocked by inserted a");
}
```

Replace `<AD_AUG_ID>` with Task 0's id. Once these structural assertions pass, tighten `atti_trace_*` and `adat_trace_*` to full `assert_eq!(trace_for(...), vec![...])` pins (matching the `yayuh_trace_*` style) using the exact emitted sequence, so rule *order* is locked, not just membership.

- [ ] **Step 2: Run the trace tests.**

Run: `mise run test -- -p panini trace`
Expected: PASS. If an id in a full-vector pin is off, fix the expected vector to the emitted-and-reference-confirmed order (never loosen a pin to hide a real ordering bug).

- [ ] **Step 3: Commit.**

```bash
git add crates/panini/tests/trace.rs
git commit -m "test(trace): pin √ad cartva / her-dhiH / a-augment rule order"
```

---

## Task 7: Mutation guards (negative pins) for the three new guard arms

Pin each new rule's guard so a mutation that widens/drops it is caught. Mirrors the 5b negative-pin discipline.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (`#[cfg(test)]` module) and/or `crates/panini/tests/paradigm.rs` (INVALID pins), following whichever pattern the existing adādi guard-edge tests use (e.g. the `8.2.77` guard-edge pin ~line 2669, and the 5b `INVALID`/negative string pins).

**Interfaces:**
- Consumes: the derivation helpers already used in Tasks 2–4; the "expect INVALID / expect this exact string" pattern from existing tests.

- [ ] **Step 1: Write the guard-boundary tests.**

```rust
#[test]
fn cartva_guard_is_khar_only_not_m_or_vowel() {
    // Over-application killer: d before `m` (admi) or vowel (adanti) must NOT
    // cartva-ize. Under-application killer: d before `t` MUST (atti, not adti).
    assert_eq!(form_lakara("ad", Lakara::Lat, Purusha::Uttama, Vacana::Eka), "admi");
    assert_eq!(form_lakara("ad", Lakara::Lat, Purusha::Prathama, Vacana::Bahu), "adanti");
    assert_ne!(form_lakara("ad", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "adti");
}

#[test]
fn her_dhih_guard_is_jhal_final_only() {
    // ā-final √yā loṭ 2sg keeps hi (yAhi), NOT *yADi: 6.4.101 needs a jhal.
    assert_eq!(form_lakara("yA", Lakara::Lot, Purusha::Madhyama, Vacana::Eka), "yAhi");
}

#[test]
fn a_augment_does_not_leak_into_dual_or_plural() {
    // The single-char length guard: 2du ending `tam` must NOT get an `a`
    // (no *Adatam); it stays Attam via cartva.
    assert_eq!(form_lakara("ad", Lakara::Lan, Purusha::Madhyama, Vacana::Dvi), "Attam");
    assert_ne!(form_lakara("ad", Lakara::Lan, Purusha::Madhyama, Vacana::Dvi), "Adatam");
}
```

- [ ] **Step 2: Run the tests.**

Run: `mise run test -- -p panini-prakriya guard`
Expected: PASS (the rules from Tasks 2–4 already satisfy these; the tests exist to kill future mutants).

- [ ] **Step 3: Commit.**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "test(prakriya): pin cartva / her-dhiH / a-augment guard boundaries"
```

---

## Task 8: mutants timeout fix + full gate

**Files:**
- Modify: `mise.toml` (`[tasks.mutants]`)

**Interfaces:** none.

- [ ] **Step 1: Raise the per-mutant timeout.** In `mise.toml`, add `--timeout 60` to the `mutants` task's `cargo mutants` invocation (the auto 20 s timeout is under the ~30 s workspace suite, producing false `Timeout`s and a nonzero exit despite 0 missed). Example — extend the existing command:

```toml
[tasks.mutants]
run = "cargo mutants --package panini-prakriya --test-workspace=true --timeout 60"
```

(Match the existing flags in the file; only add `--timeout 60`.)

- [ ] **Step 2: Run the full static + test gate.**

Run: `mise run fmt-check && mise run lint && mise run build && mise run test`
Expected: all PASS. Fix any `fmt`/`clippy` findings (`mise run fmt` for formatting).

- [ ] **Step 3: Run mutation testing on the new rules.**

Run: `mise run mutants`
Expected: 0 MISSED, clean nonzero-free exit. Triage any survivor on the three new rule bodies (cartva's `is_khar`/`cartva_of`, 6.4.101's `is_jhal`, the a-augment's length/tag guards) by adding a killing assertion to Task 7's tests, then re-run.

- [ ] **Step 4: Commit.**

```bash
git add mise.toml
git commit -m "chore(mutants): raise per-mutant timeout to 60s for clean gate"
```

---

## Task 9: Docs — retire the parent spec's √ad deferral note

**Files:**
- Modify: `crates/panini/AGENTS.md` or the top-level `AGENTS.md` scope note (the "adādi … √yā/√vā are complete … the consonant-final … roots (√ad, …) land in later slices" sentence) — narrow it to reflect √ad now complete, √vas/√ās/√śī still deferred.

**Interfaces:** none.

- [ ] **Step 1: Update the scope note.** In `AGENTS.md`, change the adādi status sentence so √ad is listed as complete across all four lakāras (parasmaipada, cartva), and only √vas (jaśtva) and the ātmanepada roots (√ās, √śī) remain deferred. Keep wording consistent with the existing note's style.

- [ ] **Step 2: Verify no stale references.**

Run: `git grep -n "√ad" -- '*.md' 'crates/**/*.rs'`
Expected: remaining mentions describe √ad as implemented (this slice's spec/plan, code comments), none claim it is deferred/gated.

- [ ] **Step 3: Full suite sanity + commit.**

Run: `mise run test`
Expected: PASS.

```bash
git add AGENTS.md crates/panini/AGENTS.md 2>/dev/null; git commit -m "docs: adādi √ad complete (cartva); √vas/ātmanepada still deferred"
```

---

## Self-Review

**Spec coverage:**
- §Scope / §Paradigm map (36 cells, 936→972, root 26→27) → Tasks 1, 5.
- Rule ① 6.4.101 her dhiḥ → Task 3. Rule ② laṅ a-augment → Task 4 (id resolved Task 0). Rule ③ 8.4.55 cartva → Task 2.
- vidhiliṅ free (reuses slice 2) → verified by Task 5's `viDiliN` row (no new rule).
- "On 8.2.23 (already present)" → Task 4 relies on it declining; Task 6 `adat` trace asserts it does not fire.
- §Testing: golden (Task 5), 3 trace pins (Task 6), unit guard boundaries (Task 7), mutation + `--timeout 60` (Task 8), static gates (Task 8).
- §Risks: #1 a-augment sūtra id → Task 0 Step 2 (blocks Task 4). #2 form verification → Task 0 Step 1 (feeds Task 5). #3 cartva tripādī placement → Task 2 Step 4 (inserted as last rule; ordering pinned by Task 6's `atti` full-vector pin).

**Placeholder scan:** the only intentional placeholders are `<AD_AUG_ID>` / `<AD_AUG_NAME>` in Tasks 4 and 6, explicitly gated on Task 0 Step 2 and flagged "do not commit the placeholders." All other steps carry concrete code/commands. No "TBD"/"add error handling"/"similar to Task N".

**Type consistency:** `form_lakara(&str, Lakara, Purusha, Vacana) -> String` is introduced in Task 2 Step 1 and reused verbatim in Tasks 3, 4, 7. `is_jhal`/`is_khar`/`cartva_of` defined in Task 2 Step 3, consumed in Tasks 3 (`is_jhal`) and 4 (`is_vowel`, existing). Rule ids (`"8.4.55"`, `"6.4.101"`, `<AD_AUG_ID>`) are consistent between the rule bodies (Tasks 2–4) and the trace pins (Task 6). `PARADIGM` tuple shape and lakāra labels match the file's existing rows.
