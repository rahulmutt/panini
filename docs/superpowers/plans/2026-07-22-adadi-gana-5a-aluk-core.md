# Adādi gaṇa — Slice 5a (aluk core) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Land the adādi (gaṇa 2) *aluk* mechanism — 2.4.72 *adiprabhṛtibhyaḥ śapaḥ* eliding śap — proven end-to-end on the two ā-final parasmaipadī roots √yā and √vā across laṭ, laṅ, and loṭ, so the "śap is always inserted" invariant is broken cleanly and the root+ending vowel junction (ā + a → ā) works.

**Architecture:** Every prior gaṇa keeps an a-final vikaraṇa (śap/śyan/śa) between root and ending, so the pipeline hardcodes indices (`ANGA=0`, `SHAP=1`, `ENDING=2`; see `tinanta.rs` header comment). adādi is the first gaṇa where 3.1.68 still inserts śap but a new rule 2.4.72 immediately *luk*s it. We model luk by **emptying the śap term's text while keeping the term in place** — indices stay valid, `text()` (a plain concatenation) joins `root + "" + ending`. Two existing rules are then touched: 6.1.78 is made panic-safe against the empty śap, and 6.1.101 gains an arm so the aṅga's own final ā coalesces with an a/ā-initial ending (yā + anti → yānti). All changes are guarded so bhvādi/divādi/tudādi output is byte-identical.

**Tech Stack:** Rust workspace (`panini-data`, `panini-prakriya`, `panini`), `mise` task runner, SLP1 internal encoding, golden + trace + mutation tests.

## Global Constraints

- Toolchain via `mise` (rust 1.97.1); build/test with `mise run build | test`. Never install Rust globally.
- `#![forbid(unsafe_code)]` in every crate touched.
- SLP1 is the only internal representation; no transliteration outside `panini-lipi` (untouched here).
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, not as a branch inside `derive`.
- Sūtra ids/names in traces must match the reference (ashtadhyayi.com / the `ashtadhyayi-com/data` GitHub mirror). Task 2 verifies the one new name (2.4.72) before it is baked into `record()`.
- **The 864 existing golden forms and every pinned bhvādi/divādi/tudādi trace must never change.** The luk mechanism is a no-op for every non-adādi gaṇa: 2.4.72 guards on `Tag::Adadi`, the 6.1.78 fix only changes behavior when śap is empty (never for a real vikaraṇa), and the new 6.1.101 arm guards on `p.terms[SHAP].text.is_empty()` (true only after adādi's luk). Any change to a non-adādi golden/trace is a bug.
- **Scope of 5a:** adādi is introduced *partially* — √yā, √vā only, and only laṭ/laṅ/loṭ. The vidhiliṅ athematic optative (the *yās → yuḥ* reduction, e.g. yāyāt/yāyuḥ) and the consonant-final / ātmanepada / √śī machinery are **slice 5b**. Do not add adādi vidhiliṅ or the other four roots here. The spec (`docs/superpowers/specs/2026-07-22-adadi-gana-design.md`) is the shared design; this plan realizes its aluk core.
- **Working state between tasks:** tasks are ordered so `mise run test` passes after each. Golden forms (Task 4) land only after the rules that derive them (Tasks 2–3).
- Root data (√yā *prāpaṇe*, √vā *gati-gandhanayoḥ*, both adādi parasmaipada) matches the spec's root table and is cross-checked against ashtadhyayi.com when the golden block is written.

---

### Task 1: Data + plumbing — `Gana::Adadi`, `Tag::Adadi`, √yā/√vā, aṅga tagging

Wire adādi through the three crates in one atomic change so the workspace compiles (adding a `Gana` variant makes `derive`'s `match dhatu.gana` non-exhaustive until its arm exists). No adādi surface form is asserted yet.

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (the `Gana` enum, the `DHATUS` static, the count test)
- Modify: `data/dhatupatha.tsv` (reference mirror of the Rust static — keep in sync)
- Modify: `crates/panini-prakriya/src/term.rs` (add `Tag::Adadi`)
- Modify: `crates/panini-prakriya/src/tinanta.rs` (the `match dhatu.gana` in `derive`)

**Interfaces:**
- Produces: `Gana::Adadi`; `Tag::Adadi`; two new `Dhatu` entries (`yA`, `vA`) reachable via `dhatus()`; `dhatus().len() == 26`; the aṅga term for an adādi root carries `Tag::Adadi`.

- [ ] **Step 1: Update the count test.** In `crates/panini-data/src/lib.rs`, rename/extend `has_twentyfour_curated_roots_with_padas` to assert 26 and spot-check the two new roots:

```rust
    #[test]
    fn has_twentysix_curated_roots_with_padas() {
        assert_eq!(dhatus().len(), 26);
        let bu = dhatus().iter().find(|d| d.code == "BU").unwrap();
        assert!(matches!(bu.pada, Pada::Parasmaipada));
        let labh = dhatus().iter().find(|d| d.code == "laB").unwrap();
        assert!(matches!(labh.pada, Pada::Atmanepada));
        // Both vowel-initial atmanepadi roots must be present (they exercise
        // the AT-augment path 6.4.72/6.1.90).
        assert!(dhatus().iter().any(|d| d.code == "eD"));
        assert!(dhatus().iter().any(|d| d.code == "Ikz"));
        // Divadi/tudadi still present.
        let div = dhatus().iter().find(|d| d.code == "div").unwrap();
        assert!(matches!(div.gana, Gana::Divadi));
        let tud = dhatus().iter().find(|d| d.code == "tud").unwrap();
        assert!(matches!(tud.gana, Gana::Tudadi));
        // New: adadi (gaṇa 2), both ā-final parasmaipada.
        let ya = dhatus().iter().find(|d| d.code == "yA").unwrap();
        assert!(matches!(ya.gana, Gana::Adadi) && matches!(ya.pada, Pada::Parasmaipada));
        let va = dhatus().iter().find(|d| d.code == "vA").unwrap();
        assert!(matches!(va.gana, Gana::Adadi) && matches!(va.pada, Pada::Parasmaipada));
    }
```

- [ ] **Step 2: Run it to verify it fails.**

Run: `mise run test -- -p panini-data`
Expected: FAIL (`Gana::Adadi` unknown; `len()` is 24).

- [ ] **Step 3: Add the enum variant.** In `crates/panini-data/src/lib.rs`, extend `Gana`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gana {
    Bhvadi,
    Divadi,
    Tudadi,
    Adadi,
}
```

- [ ] **Step 4: Append the two roots** to the `DHATUS` static (after the `gur` entry, before the closing `];`), matching the existing field-per-line style:

```rust
    // adādi (gaṇa 2) — śap luk (2.4.72); slice 5a covers these two ā-final
    // parasmaipada roots across laṭ/laṅ/loṭ.
    Dhatu {
        code: "yA",
        gana: Gana::Adadi,
        pada: Pada::Parasmaipada,
        artha: "prApaRe",
    },
    Dhatu {
        code: "vA",
        gana: Gana::Adadi,
        pada: Pada::Parasmaipada,
        artha: "gatigandhanayoH",
    },
```

- [ ] **Step 5: Mirror the rows into `data/dhatupatha.tsv`.** Append (tab-separated, matching the existing `code\tgana\tpada\tartha` layout):

```
yA	adadi	parasmaipada	prApaRe
vA	adadi	parasmaipada	gatigandhanayoH
```

- [ ] **Step 6: Add `Tag::Adadi`** in `crates/panini-prakriya/src/term.rs`, after the `Divadi`/`Tudadi` variants:

```rust
    /// The dhātu belongs to adādi (gaṇa 2), the aluk gaṇa. Read by 2.4.72,
    /// which luks the śap that 3.1.68 inserts. Mirrors Divadi/Tudadi.
    Adadi,
```

- [ ] **Step 7: Add the `derive` match arm.** In `crates/panini-prakriya/src/tinanta.rs`, in `derive`, extend the `match dhatu.gana` (currently `Divadi`/`Tudadi`/`Bhvadi`):

```rust
        match dhatu.gana {
            Gana::Divadi => t.add(Tag::Divadi),
            Gana::Tudadi => t.add(Tag::Tudadi),
            Gana::Adadi => t.add(Tag::Adadi),
            Gana::Bhvadi => {}
        }
```

- [ ] **Step 8: Run tests to verify they pass.**

Run: `mise run test`
Expected: PASS. The count test is green (26 roots); the whole workspace compiles. No adādi form is asserted yet — √yā currently derives through śap (3.1.68) with no luk, producing a wrong string, but nothing checks it.

- [ ] **Step 9: Commit.**

```bash
git add crates/panini-data/src/lib.rs data/dhatupatha.tsv crates/panini-prakriya/src/term.rs crates/panini-prakriya/src/tinanta.rs
git commit -m "feat(data): adadi gana plumbing — Gana::Adadi, Tag::Adadi, yA/vA roots"
```

---

### Task 2: 2.4.72 luk of śap + panic-safe 6.1.78

Add the rule that *is* the gaṇa, and make the one rule that would panic on an empty śap safe. After this, √yā/√vā derive correctly for every laṭ/laṅ/loṭ cell **except** those needing the ā+a vowel junction (the 3pl forms and loṭ 1sg/āṭ — Task 3).

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (add `2.4.72` after `3.1.68`; make `6.1.78` empty-śap-safe)

**Interfaces:**
- Consumes: `Tag::Adadi` (Task 1); the śap inserted by `3.1.68` at `SHAP`.
- Produces: rule `2.4.72`. After it fires, `p.terms[SHAP].text` is `""` for adādi (the term stays, so `ENDING` remains index 2). 6.1.78 declines instead of panicking when śap is empty.

- [ ] **Step 1: Write the failing test.** In `crates/panini-prakriya/src/tinanta.rs` tests, add (reuse the existing `form_g` helper already in that module):

```rust
    #[test]
    fn adadi_luk_present_no_junction_cells() {
        // ā-final adādi roots: śap is luk'd (2.4.72), the ending attaches to
        // the root directly. These cells need only the luk (no ā+a junction).
        assert_eq!(form_g("yA", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "yAti");
        assert_eq!(form_g("yA", Lakara::Lat, Purusha::Madhyama, Vacana::Eka), "yAsi");
        assert_eq!(form_g("yA", Lakara::Lat, Purusha::Uttama, Vacana::Eka), "yAmi");
        // laṅ: aṭ-augment (yā is consonant-initial) → ayā; ending attaches.
        assert_eq!(form_g("yA", Lakara::Lan, Purusha::Prathama, Vacana::Eka), "ayAt");
        // loṭ 2sg: hi does NOT elide after ā (6.4.105 needs short a) → yāhi.
        assert_eq!(form_g("yA", Lakara::Lot, Purusha::Madhyama, Vacana::Eka), "yAhi");
    }
```

`form_g` is the existing helper in that test module; its signature is
`form_g(code: &str, la: Lakara, pu: Purusha, va: Vacana)` and it derives with
the root's own pada.

- [ ] **Step 2: Run it to verify it fails (and check it is not a panic).**

Run: `mise run test -- -p panini-prakriya adadi_luk_present_no_junction_cells`
Expected: FAIL. Most likely a **panic** in 6.1.78 (`.chars().next().unwrap()` on the empty śap) once luk exists — but before adding 2.4.72, śap is still `"a"`, so instead you'll see a wrong string like `yAati` (śap `a` never removed). Either way: not green. Proceed.

- [ ] **Step 3: Make 6.1.78 panic-safe.** In the `6.1.78` rule's `apply`, reorder so the aṅga-final match happens first, and read the next term through an `Option` guard (never `.unwrap()` on a possibly-empty śap):

```rust
        apply: |p| {
            let anga_last = p.terms[ANGA].text.chars().last().unwrap();
            let sub = match anga_last {
                'e' => "ay",
                'o' => "av",
                'E' => "Ay",
                'O' => "Av",
                _ => return false,
            };
            // śap may be luk'd (adādi, 2.4.72): then it is empty and this rule
            // has no a-final vikaraṇa to work against. Decline rather than
            // panic. (5b generalizes this to the root+ending junction for √śī.)
            let Some(next_first) = p.terms[SHAP].text.chars().next() else {
                return false;
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
```

- [ ] **Step 4: Add the 2.4.72 luk rule.** Insert a new `Rule` entry immediately **after** the `3.1.68` entry (and before the second `1.2.4` entry), using the name confirmed in Task 3-of-this-list… (verified below in this task's Step 6):

```rust
    // 2.4.72 adiprabhṛtibhyaḥ śapaḥ: adādi (gaṇa 2) luks the śap that 3.1.68
    // inserts, so the tiṅ ending attaches directly to the root. Modelled by
    // emptying the śap term's text (the term stays, keeping ENDING at index 2
    // and text() = root + "" + ending). Guarded on Tag::Adadi and on a real
    // śap being present, so it never touches divādi/tudādi (śyan/śa) or bhvādi
    // that has already been processed differently.
    Rule {
        id: "2.4.72",
        name: "adipraBftiByaH SapaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Adadi) {
                return false;
            }
            if !(p.terms.len() > SHAP
                && p.terms[SHAP].has(Tag::Vikarana)
                && !p.terms[SHAP].text.is_empty())
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = String::new();
            p.record("2.4.72", "adipraBftiByaH SapaH", before);
            true
        },
    },
```

- [ ] **Step 5: Run the target test.**

Run: `mise run test -- -p panini-prakriya adadi_luk_present_no_junction_cells`
Expected: PASS (yAti, yAsi, yAmi, ayAt, yAhi).

- [ ] **Step 6: Verify the 2.4.72 sūtra name against the reference.** Fetch 2.4.72 from the `ashtadhyayi-com/data` GitHub mirror (`sutraani/data.txt`, entry `i=24072`), as slices 2–4 did (ashtadhyayi.com itself is a JS SPA that plain fetch cannot render). Confirm the Devanāgarī अदिप्रभृतिभ्यः शपः transliterates to SLP1 `adipraBftiByaH SapaH`. If it differs, correct the `id`/`name`/`record` strings above to the confirmed spelling verbatim, and record the finding here:

  `2.4.72 = "adipraBftiByaH SapaH"` (confirm or correct).

- [ ] **Step 7: Run the whole suite.**

Run: `mise run test`
Expected: PASS — all 864 existing golden forms and every bhvādi/divādi/tudādi trace unchanged (2.4.72 declines for non-adādi; the 6.1.78 reorder is behavior-identical when śap is a real vikaraṇa, since `sub` is computed the same and `next_first` is present).

- [ ] **Step 8: Commit.**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "feat(prakriya): 2.4.72 adiprabhrtibhyah sapah — luk of sap for adadi; 6.1.78 empty-sap guard"
```

---

### Task 3: 6.1.101 savarṇa-dīrgha arm — root ā + a/ā-initial ending

The luk'd śap exposes the root's final ā to an a/ā-initial ending with no buffer. ā + a and ā + ā are savarṇa → a single long ā (6.1.101 *akaḥ savarṇe dīrghaḥ*). This is what makes yā + anti → yānti and yā + Ani (āṭ) → yāni.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (add an arm to `6.1.101`)

**Interfaces:**
- Consumes: the luk'd (empty) śap (Task 2), the aṅga's ā, and the ending after `7.1.3` (Ji → anti) / the āṭ-augment.
- Produces: the ā+a coalescence, cited as `6.1.101`.

- [ ] **Step 1: Write the failing tests.** In `crates/panini-prakriya/src/tinanta.rs` tests:

```rust
    #[test]
    fn adadi_root_final_a_coalesces_with_vowel_endings() {
        // ā + a(nti) → ā : yānti (laṭ 3pl), yAntu (loṭ 3pl), ayAn (laṅ 3pl).
        assert_eq!(form_g("yA", Lakara::Lat, Purusha::Prathama, Vacana::Bahu), "yAnti");
        assert_eq!(form_g("yA", Lakara::Lot, Purusha::Prathama, Vacana::Bahu), "yAntu");
        assert_eq!(form_g("yA", Lakara::Lan, Purusha::Prathama, Vacana::Bahu), "ayAn");
        // ā + A(ṭ) → ā : loṭ uttama-eka takes āṭ (yA + Ani → yAni).
        assert_eq!(form_g("yA", Lakara::Lot, Purusha::Uttama, Vacana::Eka), "yAni");
    }
```

- [ ] **Step 2: Run to verify failure.**

Run: `mise run test -- -p panini-prakriya adadi_root_final_a_coalesces_with_vowel_endings`
Expected: FAIL — without the arm, `yA` + `anti` never coalesces, giving `yAanti` (and `ayAant`→`ayAan`? no: `ayAant` → 8.2.23 trims only the final conjunct, so a wrong `yAanti`/`ayAant`-shaped string). Not green.

- [ ] **Step 3: Add the adādi arm to 6.1.101.** Prepend it to the `6.1.101` rule's `apply`, leaving the existing bhvādi arm intact:

```rust
        apply: |p| {
            // adādi (śap luk'd by 2.4.72): the aṅga's own final ā meets an
            // a/ā-initial ending directly (no vikaraṇa buffer). ā + a/ā are
            // savarṇa → a single long ā. Keep the aṅga's ā, drop the ending's
            // initial vowel: yA + anti → yAnti, yA + Ani (āṭ) → yAni.
            if p.terms.len() > ENDING
                && p.terms[SHAP].text.is_empty()
                && p.terms[ANGA].text.ends_with('A')
                && matches!(p.terms[ENDING].text.chars().next(), Some('a') | Some('A'))
            {
                let before = p.snapshot();
                p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
                p.record("6.1.101", "akaH savarRe dIrGaH", before);
                return true;
            }
            // Original bhvādi arm: śap a-final + ending A-initial (āṭ).
            if !p.terms[SHAP].text.ends_with('a') || !p.terms[ENDING].text.starts_with('A') {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[SHAP].text.chars().collect();
            s.pop();
            s.push('A');
            p.terms[SHAP].text = s.into_iter().collect();
            p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
            p.record("6.1.101", "akaH savarRe dIrGaH", before);
            true
        },
```

- [ ] **Step 4: Run to verify pass and no regression.**

Run: `mise run test`
Expected: PASS — the new junction cells green; all 864 existing forms unchanged (the new arm's `p.terms[SHAP].text.is_empty()` guard is false for every non-adādi gaṇa, whose śap slot holds a real `a`/`ya`/`a`).

- [ ] **Step 5: Commit.**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "feat(prakriya): 6.1.101 root-final a savarna-dirgha arm for luk'd adadi (yAnti, yAni)"
```

---

### Task 4: Golden paradigm — √yā and √vā, laṭ/laṅ/loṭ (54 forms)

Pin all six blocks. The `every_form_validates_and_matches` test iterates `PARADIGM` automatically, so appending extends coverage with no harness change.

**Files:**
- Modify: `crates/panini/tests/paradigm.rs` (append six blocks to `PARADIGM`)

**Interfaces:**
- Consumes: the full engine (Tasks 1–3).
- Produces: 54 pinned forms; `PARADIGM` grows from 96 to 102 rows.

- [ ] **Step 1: Append the six golden blocks** to the `PARADIGM` static in `crates/panini/tests/paradigm.rs`, before the closing `];`. √vā is √yā's exact parallel (both ā-final; laṅ takes the aṭ-augment `a` since both are consonant-initial). Values verified against ashtadhyayi.com (the strong-form parasmaipada paradigm of an ā-final class-2 root):

```rust
    (
        "yA",
        "laT",
        [
            "yAti", "yAtaH", "yAnti", "yAsi", "yAThaH", "yATa", "yAmi", "yAvaH", "yAmaH",
        ],
    ),
    (
        "vA",
        "laT",
        [
            "vAti", "vAtaH", "vAnti", "vAsi", "vAThaH", "vATa", "vAmi", "vAvaH", "vAmaH",
        ],
    ),
    (
        "yA",
        "laN",
        [
            "ayAt", "ayAtAm", "ayAn", "ayAH", "ayAtam", "ayAta", "ayAm", "ayAva", "ayAma",
        ],
    ),
    (
        "vA",
        "laN",
        [
            "avAt", "avAtAm", "avAn", "avAH", "avAtam", "avAta", "avAm", "avAva", "avAma",
        ],
    ),
    (
        "yA",
        "loT",
        [
            "yAtu", "yAtAm", "yAntu", "yAhi", "yAtam", "yAta", "yAni", "yAva", "yAma",
        ],
    ),
    (
        "vA",
        "loT",
        [
            "vAtu", "vAtAm", "vAntu", "vAhi", "vAtam", "vAta", "vAni", "vAva", "vAma",
        ],
    ),
```

- [ ] **Step 2: Run the paradigm test.**

Run: `mise run test -- -p panini`
Expected: PASS — `every_form_validates_and_matches` covers all 102 rows. If a cell FAILs, the printed `expected VALID for <form>` names it; cross-check that exact form against ashtadhyayi.com. The most likely suspects are the ā+a junction cells (`yAnti`/`vAnti`, `yAntu`, `ayAn`, `yAni`) — a failure there points at the Task-3 arm, not the golden value.

- [ ] **Step 3: Run fmt and the full suite.**

Run: `mise run fmt && mise run test`
Expected: PASS.

- [ ] **Step 4: Commit.**

```bash
git add crates/panini/tests/paradigm.rs
git commit -m "test(paradigm): adadi yA/vA laT/laN/loT — 54 golden forms (918 total)"
```

---

### Task 5: Ordered-trace anchors

Pin the full ordered sūtra sequence for the two derivations that most tightly characterize the aluk core: the bare luk path (yāti) and the luk + savarṇa-dīrgha path (yānti).

**Files:**
- Modify: `crates/panini/tests/trace.rs` (add two `#[test]` fns)

**Interfaces:**
- Consumes: `trace_for` (existing helper) and the full engine.

- [ ] **Step 1: Add the two trace tests** to `crates/panini/tests/trace.rs`:

```rust
#[test]
fn yati_trace_is_the_bare_luk_path() {
    // yā laṭ P 3sg: 3.1.68 inserts śap (1.3.9 its it-samjña), 2.4.72 luks it,
    // then guṇa (7.3.84) and 6.1.78 both decline (ā-final root, empty śap).
    assert_eq!(
        trace_for("yAti"),
        vec!["1.3.78", "3.4.78", "1.3.9", "3.1.68", "1.3.9", "2.4.72"]
    );
}

#[test]
fn yanti_trace_is_the_luk_plus_savarna_path() {
    // yā laṭ P 3pl: Ji → anti (7.1.3) after the luk, then root ā + a → ā
    // (6.1.101).
    assert_eq!(
        trace_for("yAnti"),
        vec!["1.3.78", "3.4.78", "3.1.68", "1.3.9", "2.4.72", "7.1.3", "6.1.101"]
    );
}
```

- [ ] **Step 2: Run the trace tests.**

Run: `mise run test -- -p panini --test trace`
Expected: PASS. If a vector mismatches, the assertion prints the actual trace; reconcile against `TINANTA_RULES` order (the source of truth). The expected vectors were derived from the pinned `Bavati`/`Bavanti` traces plus 2.4.72's position (immediately after 3.1.68's it-samjña) and 6.1.101 replacing bhvādi's guṇa/6.1.97 tail. A genuine discrepancy means a rule is mis-ordered — fix the rule position, not the test.

- [ ] **Step 3: Commit.**

```bash
git add crates/panini/tests/trace.rs
git commit -m "test(trace): adadi aluk anchors — yAti (bare luk), yAnti (luk + savarna)"
```

---

### Task 6: Negatives, mutation, docs

Pin the aluk-specific non-forms (they kill the new guard mutants), confirm the guards are genuinely pinned, and record adādi's partial arrival in the docs.

**Files:**
- Modify: `crates/panini/tests/paradigm.rs` (extend `known_nonforms_are_invalid`)
- Modify: `AGENTS.md`, `docs/ARCHITECTURE.md`

- [ ] **Step 1: Add the negatives** to the `known_nonforms_are_invalid` list in `crates/panini/tests/paradigm.rs`:

```rust
        // adādi (gaṇa 2): śap is luk'd (2.4.72). A retained-śap surface must
        // not derive, and the parasmaipada roots reject ātmanepada endings.
        "yAyati", // yā with a spurious y-śap — no derivation yields it
        "yAte",   // parasmaipada yā with an ātmanepada ending (wrong pada)
        "vAte",   // parasmaipada vā with an ātmanepada ending (wrong pada)
        "yAati",  // luk skipped: śap's `a` left standing after ā (uncoalesced)
```

- [ ] **Step 2: Run the test.**

Run: `mise run test -- -p panini known_nonforms_are_invalid`
Expected: PASS (each is non-derivable in the covered grammar). If `yAati` returns VALID, 2.4.72 is not firing (or a rule is coalescing the stray śap) — that is a real bug; debug it, do not delete the negative. Note: do **not** add `yAyAt`/`yAyuH` here — those are real (future 5b) vidhiliṅ forms the engine may already derive during analysis; asserting them INVALID would be wrong.

- [ ] **Step 3: Run mutation testing.**

Run: `MISE_ENV=dev mise install && mise run mutants`
Expected: no NEW surviving mutants in the added/changed guard arms. Specifically confirm the Task 1–4 tests kill mutations of: the `Tag::Adadi` guard and the `!p.terms[SHAP].text.is_empty()` condition in 2.4.72; the `is_empty()` / `ends_with('A')` / `matches!(..., Some('a') | Some('A'))` conditions in the 6.1.101 adādi arm; and the reordered `Option` guard in 6.1.78. If a mutant survives, add a targeted golden cell (Task 4 style) or negative (Step 1 style) that distinguishes it, then re-run. (`yAati` kills a 2.4.72-off mutant; `yAnti`/`yAni` kill 6.1.101-arm mutants; `yAhi` kills a mutated 6.4.105 that wrongly elides after ā.)

- [ ] **Step 4: Update AGENTS.md.** The golden-test description currently reads `864 forms across laṭ/laṅ/loṭ/vidhiliṅ × parasmaipada/ātmanepada × bhvādi/divādi/tudādi`. Update the count and note adādi's partial arrival:

```
  (`crates/panini/tests/paradigm.rs`, 918 forms; bhvādi/divādi/tudādi are
    complete across laṭ/laṅ/loṭ/vidhiliṅ × parasmaipada/ātmanepada, and adādi
    (gaṇa 2) is being added — slice 5a covers √yā/√vā in laṭ/laṅ/loṭ; the
    consonant-final/ātmanepada roots and vidhiliṅ land in slice 5b)
```

- [ ] **Step 5: Update docs/ARCHITECTURE.md.** In the gaṇa paragraph (currently "Three gaṇas are covered: bhvādi (1), divādi (4), tudādi (6). …"), note that adādi (gaṇa 2) is being introduced, carried by `Tag::Adadi`, and that it is the first gaṇa where the vikaraṇa is **luk'd** — 3.1.68 still inserts śap and **2.4.72 *adiprabhṛtibhyaḥ śapaḥ*** empties it (the term is kept empty so `ANGA`/`SHAP`/`ENDING` indices stay stable). Keep it to two or three sentences, matching the surrounding style.

- [ ] **Step 6: Run the full suite one last time.**

Run: `mise run fmt-check && mise run lint && mise run test`
Expected: PASS.

- [ ] **Step 7: Commit.**

```bash
git add crates/panini/tests/paradigm.rs AGENTS.md docs/ARCHITECTURE.md
git commit -m "test(paradigm): adadi aluk negatives; docs: adadi gana partial (slice 5a)"
```

---

## Self-Review

**Spec coverage** (against `docs/superpowers/specs/2026-07-22-adadi-gana-design.md`, aluk-core portion):
- `Gana::Adadi` + `Tag::Adadi` + threading the gaṇa on the aṅga → Task 1. ✓
- 2.4.72 luk of śap (the core) → Task 2. ✓
- Root+ending vowel junction (ā + a → ā) → Task 3 (6.1.101 arm). ✓
- √yā, √vā data + golden → Tasks 1, 4. ✓
- Trace anchors, negatives, mutation, docs → Tasks 5, 6. ✓
- **Deliberately deferred to 5b (documented in Global Constraints):** vidhiliṅ for √yā/√vā (the athematic optative *yās → yuḥ* reduction), and √ad/√ās/√vas/√śī with cartva (8.4.55), jaśtva (8.2.39), *jha → at* (7.1.5), *hi → dhi* (6.4.101), and the √śī specials (7.4.21, 7.1.6). These are named here so a reviewer sees the boundary, not a gap. ✓

**Placeholder scan:** every code block is complete; the only deferred value is the 2.4.72 name confirmation (Task 2 Step 6), a genuine reference-verification step, not a code placeholder. No "TBD"/"handle edge cases"/"similar to Task N".

**Type consistency:** `Gana::Adadi` (Task 1, panini-data) matches the `derive` match arm (Task 1, panini-prakriya); `Tag::Adadi` (Task 1) is the exact identifier the 2.4.72 guard consumes (Task 2); `form_g(code, lakara, purusha, vacana)` is the existing helper signature used in Tasks 2–3; `SHAP`/`ANGA`/`ENDING` are the existing constants; the 6.1.101 arm and the 6.1.78 fix edit rules that already exist under those ids. Consistent throughout.
