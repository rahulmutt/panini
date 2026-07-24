# Adādi √ās (ātmanepada, first voiced junction) — Slice 5d Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the adādi ātmanepadī root √ās (*upaveśane*) across all four lakāras (36 golden forms), landing two new rules — 7.1.5 *ātmanepadeṣv anataḥ* and the engine's first *voiced* internal junction (jaśtva, `s → d` before `dh`).

**Architecture:** √ās is registered as one new `Dhatu`; the two new grammar rules are self-guarding entries in `TINANTA_RULES` (per AGENTS.md — no branch in `derive`), fed by the existing `Tag::Adadi` tagging and `Pada::Atmanepada` threading. Rule ① (7.1.5) is an apavāda placed before 7.1.3 in the 7.1 band; rule ② (voiced junction) is a general `is_jhas`/`jastva_of`-driven tripādī rule placed immediately before the existing 8.4.55 cartva. Everything else reuses slice-3 ātmanepada endings + the adādi luk (2.4.72) unchanged.

**Tech Stack:** Rust (workspace, edition per `Cargo.toml`); `mise` task runner (`mise run test | fmt-check | lint | audit | mutants`); tests in `crates/panini/tests/{paradigm,trace}.rs` (golden) and `crates/panini-prakriya/src/tinanta.rs` `#[cfg(test)]` (unit); reference = ashtadhyayi.com.

## Global Constraints

- SLP1 is the only internal representation; never transliterate outside `panini-lipi`. (SLP1 crib for this slice: `A`=ā, `I`=ī, `E`=ai, `D`=dh, `T`=th, `H`=visarga.)
- `#![forbid(unsafe_code)]` holds in every non-fuzz crate — add no `unsafe`.
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, never as a branch inside `derive`.
- Every sūtra id and name recorded in a trace must match the cited reference (ashtadhyayi.com). Where an id is not yet pinned, verify against the reference before committing the trace.
- No existing bhvādi/divādi/tudādi/adādi(√yā,√vā,√ad) golden or trace output may change: both new rules must be guarded to fire only on √ās's non-a-final ātmanepada junction. Run the full suite after each rule.
- Toolchain is pinned via `mise` (rust 1.97.1). Do not install Rust globally. Run `MISE_ENV=dev mise install` once before the mutation gate (Task 5).
- Verify every one of √ās's 36 surface forms against ashtadhyayi.com at write time (Task 4).

---

## File structure

- `crates/panini-data/src/lib.rs` — add the √ās `Dhatu` entry to `DHATUS`; update the root-count test 27→28; add a √ās registration test. (Task 1)
- `data/dhatupatha.tsv` — mirror the √ās row. (Task 1)
- `crates/panini-prakriya/src/tinanta.rs` — add rule ① (7.1.5) and rule ② (voiced junction) + the `is_jhas`/`jastva_of` helpers, plus unit/guard tests. (Tasks 2, 3, 5)
- `crates/panini/tests/trace.rs` — two new ordered-trace pins (`Asate`, `AdDve`). (Tasks 2, 3)
- `crates/panini/tests/paradigm.rs` — four new √ās golden blocks (laṭ/laṅ/loṭ/vidhiliṅ); add a wrong-pada non-form. (Tasks 4, 5)

---

## Task 1: Register √ās in the data layer

**Files:**
- Modify: `crates/panini-data/src/lib.rs:209-215` (append to `DHATUS` after the `ad` entry) and `:255-256` (count test) and `:313-322` (add registration test after `ad_is_registered_as_adadi_parasmaipada`)
- Modify: `data/dhatupatha.tsv` (append one row after the `ad` row)

**Interfaces:**
- Produces: a `Dhatu { code: "As", gana: Gana::Adadi, pada: Pada::Atmanepada, artha: "upaveSane" }` reachable via `dhatus()`. Later tasks derive it with `derive(d, la, d.pada, pu, va)`.

- [ ] **Step 1: Update the root-count test to expect 28 and assert √ās**

In `crates/panini-data/src/lib.rs`, edit `has_twentyseven_curated_roots_with_padas` (line ~255). Rename it and bump the count, and add a √ās assertion beside the √yā/√vā ones:

```rust
    #[test]
    fn has_twentyeight_curated_roots_with_padas() {
        assert_eq!(dhatus().len(), 28);
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
        // New this slice: adadi ātmanepada √ās.
        let as_ = dhatus().iter().find(|d| d.code == "As").unwrap();
        assert!(matches!(as_.gana, Gana::Adadi) && matches!(as_.pada, Pada::Atmanepada));
    }
```

- [ ] **Step 2: Add a focused √ās registration test**

Immediately after `ad_is_registered_as_adadi_parasmaipada` (ends line ~322), add:

```rust
    #[test]
    fn as_is_registered_as_adadi_atmanepada() {
        let as_ = dhatus()
            .iter()
            .find(|d| d.code == "As")
            .expect("√ās present");
        assert!(matches!(as_.gana, Gana::Adadi));
        assert!(matches!(as_.pada, Pada::Atmanepada));
        assert_eq!(as_.artha, "upaveSane");
    }
```

- [ ] **Step 3: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-data`
Expected: FAIL — `has_twentyeight...` (unresolved name / `dhatus().len()` is 27) and `as_is_registered...` (`.expect("√ās present")` panics).

- [ ] **Step 4: Add the √ās `Dhatu` entry**

In `crates/panini-data/src/lib.rs`, inside `DHATUS`, after the `ad` entry (line ~210-214) and before the closing `];`:

```rust
    Dhatu {
        code: "As",
        gana: Gana::Adadi,
        pada: Pada::Atmanepada,
        artha: "upaveSane",
    },
```

Update the adādi block comment above √yā (line ~195-196) to mention √ās:

```rust
    // adādi (gaṇa 2) — śap luk (2.4.72). √ad/√yā/√vā parasmaipada; √ās
    // ātmanepada — covered across all four lakāras (laṭ/laṅ/loṭ/vidhiliṅ).
```

- [ ] **Step 5: Mirror the row in the dhātupāṭha TSV**

In `data/dhatupatha.tsv`, after the `ad` row (line 27), add one tab-separated row:

```
As	adadi	atmanepada	upaveSane
```

- [ ] **Step 6: Run the tests to verify they pass**

Run: `mise exec -- cargo test -p panini-data`
Expected: PASS (all data tests, including the two new/renamed ones).

- [ ] **Step 7: Commit**

```bash
git add crates/panini-data/src/lib.rs data/dhatupatha.tsv
git commit -m "feat(data): register √ās (adādi, ātmanepada)"
```

---

## Task 2: Rule ① — 7.1.5 *ātmanepadeṣv anataḥ* (3pl `Asate` / `Asata` / `AsatAm`)

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` — insert a `Rule` immediately before the 7.1.3 block (the `// 7.1.3 jho'ntaḥ:` comment at line ~891); add unit tests in the `#[cfg(test)] mod tests` block
- Modify: `crates/panini/tests/trace.rs` — add the `Asate` ordered-trace pin

**Interfaces:**
- Consumes: `ANGA` (0), `ENDING` (2), `Pada::Atmanepada`, `p.terms[..].text`, `p.snapshot()`, `p.record(id, name, before)` — all already defined in `tinanta.rs`.
- Produces: the 3pl ātmanepada ending rewritten from `Je`/`Ja`/`JAm` to `ate`/`ata`/`atAm` for a non-a-final aṅga; `derive("As", Lat, Atmanepada, Prathama, Bahu).text() == "Asate"`.

**Background (why the guard checks the preceding segment, not `terms[ANGA]`):** the real path (captured via `check laBante --trace`) is `Ja → Je` (3.4.79, before 7.1.3) → `Je → ante` (7.1.3) → `laBante` (6.1.97). For an a-final thematic root the char before the ending is the śap `a` (at `SHAP`, index 1); for adādi √ās the śap is luk'd (empty), so that char is the root-final `s`. 7.1.5 must fire on `s` and decline on `a` — so it inspects the **last non-empty char before `ENDING`**, not `terms[ANGA]` (which is `laB`, consonant-final, and would wrongly fire). This mirrors 8.4.55's forward "first non-empty char after the aṅga" idiom, in reverse.

- [ ] **Step 1: Write the failing unit test**

In `crates/panini-prakriya/src/tinanta.rs`, inside `mod tests`, add:

```rust
    #[test]
    fn seventwone_five_atmanepada_3pl_uses_at_not_ant() {
        // 7.1.5 ātmanepadeṣv anataḥ: √ās (adādi, s-final) 3pl → Asate/Asata/
        // AsatAm (Ja → at, not the `ant` of 7.1.3). A-final thematic roots keep
        // `ante` (7.1.5 declines), so laB is unchanged.
        assert_eq!(
            form_g("As", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "Asate"
        );
        assert_eq!(
            form_g("As", Lakara::Lan, Purusha::Prathama, Vacana::Bahu),
            "Asata"
        );
        assert_eq!(
            form_g("As", Lakara::Lot, Purusha::Prathama, Vacana::Bahu),
            "AsatAm"
        );
        // Guard boundary: a-final ātmanepada aṅga still takes 7.1.3's `ante`.
        assert_eq!(
            form_g("laB", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "laBante"
        );
    }
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `mise exec -- cargo test -p panini-prakriya seventwonev 2>&1 | tail -20`
Expected: FAIL — √ās 3pl currently derives `Asante` (7.1.3 fires unconditionally), not `Asate`. (The `laB` assertion already passes.)

- [ ] **Step 3: Add the 7.1.5 rule**

In `crates/panini-prakriya/src/tinanta.rs`, immediately **before** the `// 7.1.3 jho'ntaḥ:` comment (line ~891), insert:

```rust
    // 7.1.5 ātmanepadeṣv anataḥ: in ātmanepada, the leading `J` (jh) of the
    // ending becomes `at` — not the `ant` of 7.1.3 — when the segment the
    // ending attaches to does not end in short `a`. Apavāda to 7.1.3, ordered
    // before it; 7.1.3 then declines on its own (ending no longer starts `J`).
    // The "anataḥ" test reads the last non-empty char BEFORE the ending: for a
    // thematic root that is the śap vikaraṇa `a` (rule declines → laBante); for
    // adādi √ās the śap is luk'd/empty, so it is the root-final `s` (rule fires
    // → Asate). By this point 3.4.79 has already turned `Ja` → `Je` (laṭ/loṭ),
    // so 7.1.5 strips the leading `J` and prepends `at`: Je → ate, Ja → ata,
    // JAm → atAm. First non-a-final ātmanepadī aṅga in the engine.
    Rule {
        id: "7.1.5",
        name: "AtmanepadezvanataH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.pada, Pada::Atmanepada) {
                return false;
            }
            if !p.terms[ENDING].text.starts_with('J') {
                return false;
            }
            // "anataḥ": the segment before the ending must NOT end in short `a`.
            // Scan the terms before ENDING (skipping the luk'd/empty śap) for
            // the last non-empty char.
            let prev = p.terms[..ENDING]
                .iter()
                .rev()
                .find_map(|t| t.text.chars().last());
            let Some(prev) = prev else {
                return false;
            };
            if prev == 'a' {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("at{rest}");
            p.record("7.1.5", "AtmanepadezvanataH", before);
            true
        },
    },
```

- [ ] **Step 4: Run the unit test to verify it passes**

Run: `mise exec -- cargo test -p panini-prakriya seventwonev 2>&1 | tail -20`
Expected: PASS.

- [ ] **Step 5: Run the whole workspace suite to confirm no regressions**

Run: `mise run test`
Expected: PASS — no existing golden/trace changes (7.1.5 declines for every a-final ātmanepada root and every parasmaipada root).

- [ ] **Step 6: Capture and pin the `Asate` ordered trace**

Confirm the exact sequence with the CLI:

Run: `cargo run -q -p panini-cli -- check 'Asate' --trace`
Expected surface `Asate`, with the ordered sūtras:
`1.3.12, 3.4.78, 1.2.4, 3.4.79, 3.1.68, 1.3.9, 2.4.72, 7.1.5`

(This is `laBante`'s path with the `7.1.3 → 6.1.97` tail replaced by `2.4.72` luk + `7.1.5`. If the captured sequence differs, treat the difference as a signal — reconcile against ashtadhyayi.com's āsate derivation before altering the rule.)

In `crates/panini/tests/trace.rs`, after the last adādi trace test, add:

```rust
#[test]
fn asate_trace_uses_anatah_not_jhontah() {
    // √ās adādi ātmanepada laṭ 3pl: Ja → Je (3.4.79) → luk of śap (2.4.72) →
    // 7.1.5 ātmanepadeṣv anataḥ replaces the leading J with `at` (Je → ate),
    // and 7.1.3 declines (ending no longer starts with J): As + ate -> Asate.
    assert_eq!(
        trace_for("Asate"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.68", "1.3.9", "2.4.72", "7.1.5"
        ]
    );
    // 7.1.3 must NOT appear — 7.1.5 is its apavāda here.
    assert!(!trace_for("Asate").contains(&"7.1.3".to_string()));
}
```

- [ ] **Step 7: Run the trace test to verify it passes**

Run: `mise exec -- cargo test -p panini asate_trace 2>&1 | tail -20`
Expected: PASS. (If it fails on the vec, paste the captured trace from Step 6 into the `vec![...]` after confirming it against the reference.)

- [ ] **Step 8: Commit**

```bash
git add crates/panini-prakriya/src/tinanta.rs crates/panini/tests/trace.rs
git commit -m "feat(prakriya): 7.1.5 ātmanepadeṣv anataḥ — adādi ātmanepada 3pl Asate"
```

---

## Task 3: Rule ② — the first voiced junction (jaśtva, `s → d` before `dh` → `AdDve` / `AdDvam`)

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` — add `is_jhas` + `jastva_of` helpers after `cartva_of` (line ~88); insert a `Rule` immediately before the 8.4.55 block (the `// 8.4.55 khari ca (cartva):` comment at line ~1461); add unit + map tests
- Modify: `crates/panini/tests/trace.rs` — add the `AdDve` ordered-trace pin

**Interfaces:**
- Consumes: `ANGA`, `is_jhal` (line 40), `p.terms`, `p.snapshot`, `p.record`.
- Produces: helpers `is_jhas(char) -> bool` and `jastva_of(char) -> Option<char>`; the voiced-junction rule rewriting an aṅga-final jhal to its jaś before a following voiced stop; `derive("As", Lat, Atmanepada, Madhyama, Bahu).text() == "AdDve"`.

**Citation note (the slice's one open risk):** the grammatically precise id is **8.4.53 *jhalāṃ jaś jhaśi*** (word-internal jaśtva before a soft/voiced consonant — √ās's `s` is not pada-final, so 8.2.39 *jhalāṃ jaśo'nte* does not apply). This plan implements it as **8.4.53**, placed immediately before 8.4.55 (both in 8.4, cartva last). In Step 5 you **verify this against ashtadhyayi.com's `āddhve` prakriya**; if the reference records 8.2.39 instead, change the `id`/`name` strings and move the whole `Rule` block to just before the 8.3.15 block (line ~1440s) — the guard body and helpers are id-independent.

- [ ] **Step 1: Write the failing unit test**

In `crates/panini-prakriya/src/tinanta.rs` `mod tests`, add:

```rust
    #[test]
    fn voiced_junction_s_becomes_d_before_dhve() {
        // √ās 2pl: the root-final `s` meets the voiced `Dh` of Dve/Dvam and
        // takes its jaś (voiced) counterpart `d`: As + Dve -> AdDve.
        assert_eq!(
            form_g("As", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
            "AdDve"
        );
        assert_eq!(
            form_g("As", Lakara::Lan, Purusha::Madhyama, Vacana::Bahu),
            "AdDvam"
        );
        assert_eq!(
            form_g("As", Lakara::Lot, Purusha::Madhyama, Vacana::Bahu),
            "AdDvam"
        );
        // Guard boundary: a clean `s`-meets-`s` cell is untouched (se is not a
        // jhaś), so 2sg stays Asse — the junction must not over-apply.
        assert_eq!(
            form_g("As", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
            "Asse"
        );
    }
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `mise exec -- cargo test -p panini-prakriya voiced_junction 2>&1 | tail -20`
Expected: FAIL — √ās 2pl currently derives `AsDve`/`AsDvam` (no junction rule), not `AdDve`/`AdDvam`. (The `Asse` assertion already passes.)

- [ ] **Step 3: Add the `is_jhas` and `jastva_of` helpers**

In `crates/panini-prakriya/src/tinanta.rs`, immediately after `cartva_of` (line ~88), add:

```rust
/// A jhaś (voiced stop) — the trigger of the voiced junction (jaśtva before a
/// voiced stop). `Dh` (`D`) of Dve/Dvam is the case this slice exercises.
fn is_jhas(c: char) -> bool {
    matches!(
        c,
        'g' | 'G' | 'j' | 'J' | 'q' | 'Q' | 'd' | 'D' | 'b' | 'B' | 'h'
    )
}

/// The jaś (voiced unaspirated) substitute of a jhal, per the voiced junction
/// (8.4.53 jhalāṃ jaś jhaśi). Only `s → d` is exercised this slice; the stop
/// vargas are written generally for later jhal-final roots. Extend the
/// sibilant/`h` rows as later roots demand.
fn jastva_of(c: char) -> Option<char> {
    match c {
        'k' | 'K' | 'g' | 'G' => Some('g'),
        'c' | 'C' | 'j' | 'J' => Some('j'),
        'w' | 'W' | 'q' | 'Q' => Some('q'),
        't' | 'T' | 'd' | 'D' => Some('d'),
        'p' | 'P' | 'b' | 'B' => Some('b'),
        's' => Some('d'),
        _ => None,
    }
}
```

- [ ] **Step 4: Add the voiced-junction rule**

In `crates/panini-prakriya/src/tinanta.rs`, immediately **before** the `// 8.4.55 khari ca (cartva):` comment (line ~1461), insert:

```rust
    // 8.4.53 jhalāṃ jaś jhaśi (voiced junction / jaśtva): a jhal at the aṅga's
    // final position, meeting a jhaś (voiced stop) across the root+ending
    // junction, becomes its jaś (voiced unaspirated). √ās's `s` before the `Dh`
    // of Dve/Dvam → `d`: As + Dve -> AdDve, As + Dvam -> AdDvam. The engine's
    // first VOICED internal junction — the voiced mirror of 8.4.55's cartva;
    // general, reused unchanged by √vas (5e) and every later jhal-final root.
    // Ordered before 8.4.55: numerically earlier in the tripādī, and their
    // triggers are disjoint (voiced jhaś vs voiceless khar), so neither
    // double-fires. Like 8.4.55 it reads the first non-empty term after the
    // aṅga (śap, if present, is luk'd/empty for adādi).
    Rule {
        id: "8.4.53",
        name: "JalAM jaS JaSi",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let next = p
                .terms
                .iter()
                .skip(ANGA + 1)
                .find_map(|t| t.text.chars().next());
            let Some(next) = next else { return false };
            if !is_jhas(next) {
                return false;
            }
            let Some(last) = p.terms[ANGA].text.chars().last() else {
                return false;
            };
            if !is_jhal(last) {
                return false;
            }
            let Some(sub) = jastva_of(last) else {
                return false;
            };
            if sub == last {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ANGA].text.chars().collect();
            s.pop();
            s.push(sub);
            p.terms[ANGA].text = s.into_iter().collect();
            p.record("8.4.53", "JalAM jaS JaSi", before);
            true
        },
    },
```

- [ ] **Step 5: Verify the sūtra id against the reference, then run the unit test**

Verify on ashtadhyayi.com that `āddhve` (आद्ध्वे) is derived by **8.4.53 jhalāṃ jaś jhaśi** (expected) and not 8.2.39. If it is 8.2.39, update the `id`/`name` strings in the rule and this task's trace, and move the `Rule` block to just before the 8.3.15 block. Then:

Run: `mise exec -- cargo test -p panini-prakriya voiced_junction 2>&1 | tail -20`
Expected: PASS.

- [ ] **Step 6: Add the `jastva_of` map pin (mirrors the `cartva_of` map test)**

In `mod tests`, near `cartva_of_maps_each_jhal_to_its_first_varga_car` (line ~3128), add:

```rust
    #[test]
    fn jastva_of_maps_each_jhal_to_its_jas() {
        // Pin the whole map so a mutated arm can't survive: each varga's members
        // collapse to that varga's jaś (voiced unaspirated); `s → d` is the arm
        // this slice exercises. Non-jhal / unmapped chars return None.
        for c in ['k', 'K', 'g', 'G'] {
            assert_eq!(jastva_of(c), Some('g'), "{c}");
        }
        for c in ['c', 'C', 'j', 'J'] {
            assert_eq!(jastva_of(c), Some('j'), "{c}");
        }
        for c in ['w', 'W', 'q', 'Q'] {
            assert_eq!(jastva_of(c), Some('q'), "{c}");
        }
        for c in ['t', 'T', 'd', 'D'] {
            assert_eq!(jastva_of(c), Some('d'), "{c}");
        }
        for c in ['p', 'P', 'b', 'B'] {
            assert_eq!(jastva_of(c), Some('b'), "{c}");
        }
        assert_eq!(jastva_of('s'), Some('d'));
        assert_eq!(jastva_of('a'), None);
        assert_eq!(jastva_of('m'), None);
    }

    #[test]
    fn is_jhas_is_voiced_stops_only() {
        for c in ['g', 'G', 'j', 'J', 'q', 'Q', 'd', 'D', 'b', 'B', 'h'] {
            assert!(is_jhas(c), "{c} should be jhaś");
        }
        // Voiceless obstruents, sibilants, vowels, semivowels, nasals are not.
        for c in ['t', 'T', 's', 'S', 'z', 'a', 'A', 'v', 'y', 'm', 'n'] {
            assert!(!is_jhas(c), "{c} should not be jhaś");
        }
    }
```

- [ ] **Step 7: Capture and pin the `AdDve` ordered trace**

Run: `cargo run -q -p panini-cli -- check 'AdDve' --trace`
Expected surface `AdDve`, ordered sūtras:
`1.3.12, 3.4.78, 1.2.4, 3.4.79, 3.1.68, 1.3.9, 2.4.72, 8.4.53`

(This is `laBaDve`'s captured path — `1.3.12, 3.4.78, 1.2.4, 3.4.79, 3.1.68, 1.3.9` — with `2.4.72` luk + `8.4.53` appended. Reconcile any difference against the reference before touching the rule.)

In `crates/panini/tests/trace.rs`, add:

```rust
#[test]
fn addhve_trace_ends_in_voiced_junction() {
    // √ās adādi ātmanepada laṭ 2pl: Dvam → Dve (3.4.79), śap luk'd (2.4.72),
    // then the voiced junction turns the aṅga-final `s` into `d` before the
    // `Dh` of Dve: As + Dve -> AdDve.
    assert_eq!(
        trace_for("AdDve"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.68", "1.3.9", "2.4.72", "8.4.53"
        ]
    );
    // cartva (8.4.55, the voiceless junction) must NOT fire here — the trigger
    // is a voiced stop, not a khar.
    assert!(!trace_for("AdDve").contains(&"8.4.55".to_string()));
}
```

(If Step 5 pinned the id as 8.2.39, use that id in both assertions.)

- [ ] **Step 8: Run the trace test and the whole suite**

Run: `mise exec -- cargo test -p panini addhve_trace 2>&1 | tail -20 && mise run test`
Expected: PASS. The full suite confirms √ad's `atti`/`attaH`/… are unchanged (8.4.53 declines for √ad — `t` is not a jhaś — and every thematic/vikaraṇa-buffered root has a vowel after the jhal-final root, so 8.4.53 fires only on √ās 2pl).

- [ ] **Step 9: Commit**

```bash
git add crates/panini-prakriya/src/tinanta.rs crates/panini/tests/trace.rs
git commit -m "feat(prakriya): 8.4.53 jhalāṃ jaś jhaśi — first voiced junction (AdDve)"
```

---

## Task 4: Golden paradigm — √ās's full 36 cells

**Files:**
- Modify: `crates/panini/tests/paradigm.rs` — add four `(&str, &str, [&str; 9])` blocks to `PARADIGM` (after the `ad` `viDiliN` block at line ~1083)

**Interfaces:**
- Consumes: the registered √ās (Task 1) and both new rules (Tasks 2, 3). Coverage is validated by the existing `every_form_validates_and_matches` and `paradigm_covers_every_enumerable_cell` tests — the latter's `PARADIGM.len() + GATED.len() == dhatus().len() * LAKARAS.len()` assertion needs exactly four new blocks (28 roots × 4 lakāras = 112).

**Reference note:** the 36 SLP1 forms below are the derivation's predicted output, structurally parallel to the committed √labh ātmanepada goldens (√ās differs only by being s-final with no vikaraṇa buffer, so no vowel coalescence). Per the Global Constraints, **verify each cell against ashtadhyayi.com** as you pin it; the junction/3pl cells (`Asate`, `Asata`, `AsatAm`, `AdDve`, `AdDvam`) most of all.

- [ ] **Step 1: Add the four √ās golden blocks**

In `crates/panini/tests/paradigm.rs`, inside `PARADIGM`, after the `ad`/`viDiliN` block (line ~1083), add:

```rust
    (
        "As",
        "laT",
        [
            "Aste", "AsAte", "Asate", "Asse", "AsATe", "AdDve", "Ase", "Asvahe", "Asmahe",
        ],
    ),
    (
        "As",
        "laN",
        [
            "Asta", "AsAtAm", "Asata", "AsTAH", "AsATAm", "AdDvam", "Ase", "Asvahi", "Asmahi",
        ],
    ),
    (
        "As",
        "loT",
        [
            "AstAm", "AsAtAm", "AsatAm", "Assva", "AsATAm", "AdDvam", "AsE", "AsAvahE", "AsAmahE",
        ],
    ),
    (
        "As",
        "viDiliN",
        [
            "AsIta", "AsIyAtAm", "AsIran", "AsITAH", "AsIyATAm", "AsIDvam", "AsIya", "AsIvahi",
            "AsImahi",
        ],
    ),
```

- [ ] **Step 2: Run the paradigm tests**

Run: `mise exec -- cargo test -p panini --test paradigm 2>&1 | tail -30`
Expected: PASS — `every_form_validates_and_matches` validates all 36 √ās forms VALID with a matching analysis, and `paradigm_covers_every_enumerable_cell` confirms 112 blocks == 28×4.

If a cell fails (`expected VALID for <form>`): capture what the engine derives with `cargo run -q -p panini-cli -- check '<form>' --trace`, compare against ashtadhyayi.com. A clean-cell mismatch means the predicted golden was wrong (fix the golden); a junction/3pl mismatch means a rule guard is off (fix the rule, not the golden).

- [ ] **Step 3: Run the whole suite**

Run: `mise run test`
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add crates/panini/tests/paradigm.rs
git commit -m "test(paradigm): √ās full 36-cell ātmanepada block (972→1008)"
```

---

## Task 5: Guard/negative pins and the full quality gate

**Files:**
- Modify: `crates/panini/tests/paradigm.rs` — add wrong-pada non-forms to `known_nonforms_are_invalid` (line ~1181)
- Modify: `crates/panini-prakriya/src/tinanta.rs` — add guard-boundary negative pins in `mod tests`

**Interfaces:**
- Consumes: everything from Tasks 1-4. Produces no new surface behavior — this task hardens the guards so mutation testing finds no survivors.

- [ ] **Step 1: Add wrong-pada / junction negatives to the golden non-form list**

In `crates/panini/tests/paradigm.rs`, inside the `for bad in [ ... ]` array of `known_nonforms_are_invalid` (line ~1181), add:

```rust
        "Asati",  // √ās is ātmanepada; a parasmaipada ending must not derive
        "Asante", // 3pl must be Asate (7.1.5), never the `ante` of 7.1.3
        "AsDve",  // 2pl voiced junction must apply: bare s+Dve is not a form
```

- [ ] **Step 2: Run the non-form test**

Run: `mise exec -- cargo test -p panini known_nonforms 2>&1 | tail -20`
Expected: PASS (all three return INVALID — no analysis produces them).

- [ ] **Step 3: Add guard-boundary negative pins for the two new rules**

In `crates/panini-prakriya/src/tinanta.rs` `mod tests`, add:

```rust
    #[test]
    fn anatah_declines_for_a_final_atmanepada_angas() {
        // 7.1.5's "anataḥ" arm: every a-final (thematic / vikaraṇa-buffered)
        // ātmanepada 3pl keeps 7.1.3's `ante`. Pins that the guard reads the
        // preceding segment's `a`, not the consonant-final root.
        assert_eq!(
            form_g("laB", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "laBante"
        );
        assert_eq!(
            form_g("man", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "manyante"
        );
        assert_eq!(
            form_g("juz", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "juzante"
        );
    }

    #[test]
    fn voiced_junction_does_not_touch_non_jhas_or_non_jhal_junctions() {
        // Under-application guard: `s` before the non-jhaś `s`/`th`/`v` of
        // se/sva/thās stays `s` (Asse, Assva, AsTAH) — only a jhaś triggers it.
        assert_eq!(
            form_g("As", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
            "Assva"
        );
        assert_eq!(
            form_g("As", Lakara::Lan, Purusha::Madhyama, Vacana::Eka),
            "AsTAH"
        );
        // Over-application guard: √ad's `d` before the voiced... there is none;
        // √ad is parasmaipada. Its voiceless junctions stay cartva's business:
        assert_eq!(
            form_g("ad", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "atti"
        );
    }
```

- [ ] **Step 4: Run the new unit tests and the whole suite**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20 && mise run test`
Expected: PASS.

- [ ] **Step 5: Static gates — format, lint, audit**

Run: `mise run fmt && mise run fmt-check && mise run lint && mise run audit`
Expected: all PASS (no diff after `fmt`, no clippy warnings, `cargo audit` + `cargo deny check` clean).

- [ ] **Step 6: Mutation gate on the new guards**

Ensure dev tooling is installed once: `MISE_ENV=dev mise install`. Then run the mutation gate (run the binary directly in a foreground shell, per the repo memory note about the mise shim in background shells):

Run: `mise run mutants`
Expected: **0 missed** and **0 survivors** — in particular on the new regions: 7.1.5's `Pada::Atmanepada` / leading-`J` / `prev == 'a'` arms, and 8.4.53's `is_jhas` / `is_jhal` / `jastva_of` / `sub == last` arms, plus the `jastva_of` and `is_jhas` maps.

If a mutant survives, add a negative that distinguishes the mutated guard from the real one (the pins in Steps 1-3 and Task 3 Step 6 are the model), then re-run. Do not weaken the gate.

- [ ] **Step 7: Update the AGENTS.md scope note and README**

In `AGENTS.md`, update the paradigm-count / adādi progress note (search for `972` / "√ad parasmaipada landing … cartva") to reflect √ās landing (1008 forms; adādi ātmanepada opened with √ās; 8.4.53 voiced junction added; √vas/√śī still deferred). In `README.md`, update the root count and the adādi scope sentence to include √ās (ātmanepada). Keep both factual and terse.

- [ ] **Step 8: Final full verification and commit**

Run: `mise run test && mise run fmt-check && mise run lint`
Expected: all PASS.

```bash
git add -A
git commit -m "test(prakriya): pin √ās guard boundaries + docs (slice 5d complete)"
```

---

## Self-review notes

- **Spec coverage:** rule ① (7.1.5) → Task 2; rule ② (voiced junction) → Task 3; the 36-cell golden block + 972→1008 / 27→28 counts → Tasks 1 & 4; the two ordered-trace pins (`Asate`, `AdDve`) → Tasks 2 & 3; negative/guard pins → Tasks 3 & 5; mutation + static gates → Task 5; the citation risk (8.4.53 vs 8.2.39) → Task 3 Steps 5 & 7; "no existing form changes" → the `mise run test` gate after each rule (Tasks 2, 3) and the √ad/√labh regression pins (Task 3 Step 8, Task 5 Step 3).
- **Vidhiliṅ "free":** confirmed by construction — the `ī` of the optative buffers every junction (`AsIDvam` is `s`+`ī`+`Dvam`, no jhaś adjacency) and the 3pl rides slice-2's `ran`; the vidhiliṅ block in Task 4 is validated but needs no new rule or trace.
- **Type/name consistency:** helper names `is_jhas` / `jastva_of`, rule ids `7.1.5` / `8.4.53`, name strings `AtmanepadezvanataH` / `JalAM jaS JaSi`, and root code `As` are used identically across every task.
