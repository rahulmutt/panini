# Tanādi gaṇa (8a / 8b) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Cover gaṇa 8 (tanādi) completely: nine roots on the u-vikaraṇa in slice 8a, then √kṛ with the 6.4.108–110 specials in slice 8b, taking the corpus from 67 roots / 2844 cells to 77 roots / 3492 cells.

**Architecture:** One new vikaraṇa rule (3.1.79) introduces the engine's first non-śit — therefore ārdhadhātuka — vikaraṇa, and the śnu-keyed guards (the asaṁyogapūrva helper behind 6.4.106/6.4.107, the second 1.2.4) are generalized rather than duplicated. New grammar: 6.1.77 *iko yaṇ aci* (the u→v the probe showed vidyut credits), a vikalpa arm of 7.3.86 for the four ik-upadhā roots (the Kaumudī 2547.1 optionality, keyed by the Pāṇinian id), the `f`→`Ar` arm of vṛddhi, and — 8b only — 6.4.110/6.4.108/6.4.109 for √kṛ. Every golden is generated from the engine after a zero-difference vidyut audit; prior goldens stay byte-identical throughout.

**Tech Stack:** Rust 1.98.0 pinned via `mise`; `cargo test` golden suite; `cargo-mutants` mutation gate; `vidyut-prakriya` at the vendored commit via the committed harness in `tools/audit/`.

**Spec:** `docs/superpowers/specs/2026-08-30-tanadi-gana-design.md`

**Branch:** `tanadi-gana` already exists with the spec committed (`137eaa4`). Slice 8a's Tasks 1–10 continue on it; do not branch again. Slice 8b starts a NEW branch `tanadi-gana-8b` from `main` after 8a merges (Task 11).

**Probe provenance:** every surface form quoted in this plan comes from a planning-time probe of vidyut-prakriya at the vendored commit `8da2f90` (all ten 08.* entries, all four lakāras, both padas, with step logs). They exist so a wrong result is recognisable; they are **expectations, not goldens** — nothing is typed into a test except generator output after the audit passes.

## Global Constraints

- **Toolchain:** rust **1.98.0** via `mise`. `mise run test -- -p X` does NOT scope — use `mise exec -- cargo test -p X`.
- **Run the golden suite in the FOREGROUND with an explicit long timeout.** The floor was 1132s at 2844 cells and this slice adds +20%; budget ≥ 30 minutes (`timeout` parameter 2400000 ms). Do not background it; do not end a turn while it runs — a backgrounded suite gets orphaned.
- **Prior goldens are byte-identical through every engine task.** Tasks 1–4 change engine code with zero tanādi roots curated, so all 2844 cells and every trace must be unchanged — that IS the test of the generalization. Any svādi drift is a defect in the task that caused it, not noise.
- **Goldens are generated, never hand-authored.** `PARADIGM`/`ALTERNATES` rows come from the throwaway generator run against the engine the audit certified.
- **The audit's negative control runs first.** A zero-difference verdict without a verified-failing `entry` control proves nothing.
- **`mise run mutants` is `-j 4 --timeout 4800`.** Run the task, don't reconstruct flags. `CARGO_MUTANTS_JOBS` in the environment can defeat `-j`; check it is unset.
- **SLP1 throughout.** `M` anusvāra, `N` velar ṅ, `Y` palatal ñ, `R` retroflex ṇ, `z` retroflex ṣ, `f` vocalic ṛ, `E`/`O` ai/au.
- **Executing from a worktree:** the audit tasks repoint `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml`'s `panini`/`panini-data` dev-dep paths at THE CHECKOUT BEING AUDITED. A stale path audits the wrong engine (this bit a prior slice); set it explicitly in Tasks 6 and 13.

## Numbers this plan changes

| quantity | pre-8a | post-8a | post-8b |
|---|---|---|---|
| `dhatus().len()` | 67 | **76** | **77** |
| `PARADIGM.len()` (blocks) | 316 | **380** | **388** |
| cells (blocks × 9) | 2844 | **3420** | **3492** |
| `ALTERNATES.len()` | 494 | measured (probe ≈ 421 new) | measured (probe ≈ 6 more) |
| forms (cells + alternates) | 3338 | measured | measured |
| roots that admit both padas | 12 | **19** | **20** |
| `VIKALPA_RULES` entries | 7 | **8** (+`7.3.86`) | 8 |

Cell-multiplicity buckets, old values (`derivation_set_shape_matches_the_audited_numbers`): ones **2493**, twos **250**, threes **83**, fours **2**, fives **8**, sixes **8**. The probe expects 8a to add six-form cells (the loṭ prathama-eka AND madhyama-eka of kziR/fR/tfR/GfR) and four-form cells (their laṅ prathama eka); the generator measures the real buckets in Task 7.

`ALTERNATES` key counts, old values: `8.4.56` 114, `7.1.35` 92, `7.1.35+8.4.56` 92, `3.4.111` 2, `6.4.107` 8, `8.4.65` 145, `8.2.75` 8, `8.2.74` 1, `7.1.35+8.4.65` 16, `7.1.35+8.4.65+8.4.56` 16. 8a adds `6.4.107` rows and entirely new `7.3.86`-bearing keys; all measured in Task 7.

## What the planning-time probe settled (spec's open points)

- **u→v before vowel-initial endings is 6.1.77 *iko yaṇ aci*** (`tanvanti`, `tanvate`, `tanvIta`). 6.4.87 stays śnu-only.
- **Upadhā-guṇa is an optional FORK, not an either/or**: kziRoti/kzeRoti, fRoti/arRoti, tfRoti/tarRoti, GfRoti/GarRoti. vidyut credits 7.3.86 on the guṇa branch and Kaumudī gaṇasūtra **2547.1** (an optional guṇa-apavāda tag on exactly those four upadeśas) on the other. This engine models it as a **vikalpa arm of 7.3.86** — the alternate keys stay Pāṇinian; the Kaumudī source lives in the comment. Note the guṇa fires before ṅit endings too (`tarRvanti`): its trigger is the ārdhadhātuka `u`, on which 1.1.5 has no purchase.
- **fR's laṅ converges**: A+fR and A+arR both surface `ArRot` (6.1.90's vṛddhi swallows the fork). The controller must collapse same-text live branches or the audit's `n_branches == n_forms` invariant breaks (Task 4).
- **`vrddhi_of` has no `f` arm** and returns `Option<char>`; `Ar` is two chars. Task 2 makes it `Option<&'static str>` with the 1.1.51 *uraṇ raparaḥ* arm — the 7d √und story (`u` arm, `Onad`) repeated for `f` (`ArRot`).
- **vanu~\ is ātmanepadī by its marker**; vidyut additionally derives `vanoti` via Kaumudī **2547.2** (optionally removing the anudātta-it). Curated ātmanepada-only: `curated_pada_agrees_with_upadesha_markers` derives pada from markers, and the audit enumerates OUR padas, so no diff can surface. Recorded in the row comment and README, the 1.3.72-sense precedent.
- **√kṛ**: `karoti / kurutaH / kurvanti / kurvaH / kurmaH / kuru / kuryAt / kurute`; `kurvaH`/`kurmaH` have NO 6.4.107 alternates (6.4.108 is nitya). Rule order observed: 7.3.84 (kar) → 6.4.110 (kur) → 6.4.108/6.4.109. Since 6.4.110 must precede 6.1.77 (`kurvanti`, else \*karvanti), all three √kṛ rules land in `guna.rs` after 7.3.84's second application — not beside their 6.4.10x siblings in `adesha.rs`. vidyut also records 8.2.79 (*na bhakurchurām*, blocking a lengthening this engine never implemented — 8.2.77 is absent too); record, don't implement.

## File Structure

| file | responsibility | task |
|---|---|---|
| `crates/panini-data/src/lib.rs` | `Gana::Tanadi`; nine rows (T5), √kṛ row (T11); `stored_form` z-arm; row tests | 1, 5, 11 |
| `crates/panini-prakriya/src/term.rs` | `Tag::Tanadi` | 1 |
| `crates/panini-prakriya/src/tinanta/mod.rs` | `Gana::Tanadi → Tag::Tanadi` | 1 |
| `crates/panini-prakriya/src/tinanta/vikarana.rs` | 3.1.79; second 1.2.4 guard | 1 |
| `crates/panini-prakriya/src/tinanta/sound.rs` | `vrddhi_of` → `&str`, `f` arm | 2 |
| `crates/panini-prakriya/src/tinanta/adesha.rs` | 6.1.90's two arms follow the new signature; 6.4.106/107 call the widened helper | 2, 3 |
| `crates/panini-prakriya/src/tinanta/terms.rs` | helper widened + renamed; `following_sarvadhatuka` doc | 3 |
| `crates/panini-prakriya/src/tinanta/guna.rs` | 6.4.87 śnu self-guard; 6.1.77; 7.3.86 vikalpa arm + nitya gaṇa guard; (8b) 6.4.110/108/109 | 3, 4, 12 |
| `crates/panini-prakriya/src/controller.rs` | convergent-fork collapse | 4 |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | pinned rule order | 1, 3, 4, 12 |
| `tools/audit/panini_full_audit.rs` + `README.md` | `gana_name` Tanadi arm; totals; recorded results | 6, 13 |
| `crates/panini/tests/paradigm/data/tanadi.rs` (+`mod.rs`) | the new gaṇa's goldens | 7, 14 |
| `crates/panini/tests/paradigm/main.rs` | `VIKALPA_RULES`, shape test, ambiguity set | 7, 14 |
| `crates/panini/tests/trace/tanadi.rs` (+`main.rs`) | trace pins | 8, 14 |
| `AGENTS.md`, `README.md`, `docs/ARCHITECTURE.md`, `data/ATTRIBUTION.md` | census, floors, recorded results | 9, 10, 15 |

**Expected-red window (8a).** Task 5 adds nine rows with no goldens behind them, so `paradigm_covers_every_enumerable_cell` fails (64 unpinned root×pada×lakāra triples) from Task 5 until Task 7. Same for Task 11 → Task 14 (8 triples). Intended; do not hand-author blocks to silence it.

---

### Task 1: `Gana::Tanadi`, 3.1.79, and the ārdhadhātuka u's escape from 1.2.4

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (the `Gana` enum)
- Modify: `crates/panini-prakriya/src/term.rs` (the `Tag` enum)
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs` (the gaṇa→tag match in `derive`)
- Modify: `crates/panini-prakriya/src/tinanta/vikarana.rs` (new rule after the 3.1.78 block; the second 1.2.4's guard)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: nothing.
- Produces: `Gana::Tanadi` (panini-data), `Tag::Tanadi` and rule `3.1.79` inserting a `Term::new("u")` tagged `Vikarana + Ardhadhatuka` at `SHAP`. Every later task relies on: the u carries NO `Sarvadhatuka` and never acquires `Ngit`.

- [ ] **Step 1: Write the failing unit tests**

In `crates/panini-prakriya/src/tinanta/vikarana.rs`'s `tests` module, alongside the existing rule tests:

```rust
    #[test]
    fn tanadi_takes_the_bare_u_and_it_stays_anit() {
        // 3.1.79 tanAdikfYBya uH. The u is the pipeline's first non-śit
        // vikaraṇa: ārdhadhātuka by 3.4.114, so the second 1.2.4 must NOT
        // tag it ṅit. That non-ṅit-ness is what 7.3.84's second application
        // reads when it guṇates u -> o (tanoti) where sunoti's śnu stays
        // blocked — the whole gaṇa hangs on this test.
        let mut p = Prakriya {
            terms: vec![Term::new("tan"), Term::new("ti")],
            ..Default::default()
        };
        p.terms[0].add(Tag::Dhatu);
        p.terms[0].add(Tag::Tanadi);
        let r_79 = rules().find(|r| r.id == "3.1.79").unwrap();
        assert!((r_79.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "u");
        assert!(p.terms[SHAP].has(Tag::Vikarana));
        assert!(p.terms[SHAP].has(Tag::Ardhadhatuka));
        assert!(!p.terms[SHAP].has(Tag::Sarvadhatuka));
        // The second 1.2.4 (the vikaraṇa application) must decline.
        let r_124 = rules().filter(|r| r.id == "1.2.4").last().unwrap();
        assert!(!(r_124.apply)(&mut p));
        assert!(!p.terms[SHAP].has(Tag::Ngit));
    }

    #[test]
    fn second_1_2_4_still_tags_the_shit_vikaranas() {
        // The 1.2.4 guard gains a positive Sarvadhatuka test; śnu (śit,
        // apit) must still come out ṅit or hinoti becomes *henoti.
        let mut p = Prakriya {
            terms: vec![Term::new("hi"), Term::new("nu"), Term::new("ti")],
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        p.terms[SHAP].add(Tag::Sarvadhatuka);
        let r_124 = rules().filter(|r| r.id == "1.2.4").last().unwrap();
        assert!((r_124.apply)(&mut p));
        assert!(p.terms[SHAP].has(Tag::Ngit));
    }
```

Match the surrounding tests' way of enumerating `VIKARANA` rules (if they use a helper other than `rules()`, use that; the existing `6.4.106` lookup in `adesha.rs`'s tests shows the idiom).

- [ ] **Step 2: Run them to verify they fail**

```bash
mise exec -- cargo test -p panini-prakriya tanadi_takes_the_bare_u
```

Expected: FAIL — no rule `3.1.79`, no `Tag::Tanadi` (compile error first; that counts).

- [ ] **Step 3: The enum plumbing**

`crates/panini-data/src/lib.rs`, `Gana` enum — append:

```rust
    Tanadi,
```

`crates/panini-prakriya/src/term.rs`, `Tag` enum — append after the other gaṇa tags (`Rudhadi`):

```rust
    Tanadi,
```

`crates/panini-prakriya/src/tinanta/mod.rs`, the gaṇa match (after the `Gana::Rudhadi` arm):

```rust
            Gana::Tanadi => t.add(Tag::Tanadi),
```

- [ ] **Step 4: The rule and the guard**

In `crates/panini-prakriya/src/tinanta/vikarana.rs`, insert after the 3.1.78 block, before 3.1.81:

```rust
    // 3.1.79 tanādikṛñbhya uḥ: tanādi (gaṇa 8) takes the bare `u`, not śap.
    // Apavāda to 3.1.68, ordered before it exactly as 3.1.69, 3.1.73,
    // 3.1.77, 3.1.78 and 3.1.81 are. The sūtra's own text names √kṛ
    // (*kṛñbhya*), so slice 8b's √kṛ rides this same rule.
    //
    // `u` is the pipeline's FIRST NON-ŚIT VIKARAṆA. With no ś it-marker it
    // is not sārvadhātuka by 3.4.113 tiṅśit sārvadhātukam, hence
    // ārdhadhātuka by 3.4.114 ārdhadhātukaṁ śeṣaḥ — recorded as
    // Tag::Ardhadhatuka rather than as rules, the same way other saṁjñā
    // verdicts live as tags. Two load-bearing consequences: the second
    // 1.2.4 below must not tag it ṅit (its guard now demands
    // Tag::Sarvadhatuka), which is what lets 7.3.84's second application
    // guṇate `u` -> `o` (tanoti) while sunoti's śnu stays blocked; and the
    // four ik-upadhā roots take 7.3.86 against a follower 1.1.5 cannot
    // block (tarRvanti beside tarRoti — see guna.rs's vikalpa arm).
    //
    // No run_it_samjna call: the affix is the bare vowel `u` (the ḥ of
    // `uḥ` is the citation's visarga, not an anubandha), so there is
    // nothing to strip and no 1.3.x step to record — which is why tanoti's
    // trace, unlike Apnoti's, shows no second 1.3.9.
    Rule {
        id: "3.1.79",
        name: "tanAdikfYBya uH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Tanadi) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("u");
            s.add(Tag::Vikarana);
            s.add(Tag::Ardhadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.79", "tanAdikfYBya uH", before);
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
```

Then the second 1.2.4's guard (the vikaraṇa application at the bottom of the file) becomes:

```rust
            if !(p.terms.len() > SHAP
                && p.terms[SHAP].has(Tag::Vikarana)
                && p.terms[SHAP].has(Tag::Sarvadhatuka)
                && !p.terms[SHAP].has(Tag::Pit)
                && !p.terms[SHAP].has(Tag::Ngit))
            {
                return false;
            }
```

and its comment gains one sentence after "so bhvādi is untouched":

```
    // Tanādi's bare `u` (3.1.79) carries Tag::Ardhadhatuka and NO
    // Sarvadhatuka tag, so the positive Sarvadhatuka test added with it
    // excludes exactly that vikaraṇa: sārvadhātukam apit reads the
    // sārvadhātukas, and `u` is not one.
```

Every śit vikaraṇa already carries `Tag::Sarvadhatuka` at creation (3.1.68/69/73/77/78/81 all add it), so this positive test changes nothing for them — the two unit tests prove both directions.

- [ ] **Step 5: Pin the order**

In `derivation_tests.rs`'s `tinanta_rule_order_is_pinned`, insert `"3.1.79"` between `"3.1.78"` and `"3.1.81"`, with a comment naming it the tanādi apavāda.

- [ ] **Step 6: Run the engine crate's tests, then the full suite**

```bash
mise exec -- cargo test -p panini-prakriya
mise run test
```

Expected: both PASS. The full suite (FOREGROUND, ≥30 min) proves byte-identity: 3.1.79 is unreachable with zero tanādi roots, and the 1.2.4 guard change is provably inert for every śit vikaraṇa.

- [ ] **Step 7: Commit**

```bash
git add -A && git commit -m "feat(engine): 3.1.79 tanAdikfYBya uH, the first ardhadhatuka vikarana

Gana::Tanadi plumbed through; the second 1.2.4 now demands Sarvadhatuka,
which the bare u (3.4.114's verdict, recorded as a tag) does not carry.
All 2844 goldens byte-identical."
```

---

### Task 2: vṛddhi grows its `f` arm

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/sound.rs` (`vrddhi_of`, its unit test)
- Modify: `crates/panini-prakriya/src/tinanta/adesha.rs` (6.1.90's two arms, which consume it)

**Interfaces:**
- Consumes: nothing.
- Produces: `vrddhi_of(v: char) -> Option<&'static str>` with arms `a/A→"A"`, `i/I/e/E→"E"`, `u/U/o/O→"O"`, **`f→"Ar"`** (1.1.1 with 1.1.51 *uraṇ raparaḥ*). Task 6's audit needs it for `ArRot` (āṭ + fR).

- [ ] **Step 1: Extend the unit test (failing)**

In `sound.rs`'s `vrddhi_of_ac_vowels_all_arms`, add:

```rust
        assert_eq!(vrddhi_of('f'), Some("Ar"));
```

and change every existing assertion's expectation from `Some('X')` to `Some("X")`. Extend the test's comment: the `f` arm is `Ar` by 1.1.51 uraṇ raparaḥ (the r-appendage on an f-substitute), reached by a golden derivation once fR's laṅ (`ArRot`) lands — the same path 7d's `Onad` opened for `u`.

- [ ] **Step 2: Run to verify it fails**

```bash
mise exec -- cargo test -p panini-prakriya vrddhi_of_ac_vowels
```

Expected: compile FAIL (`char` vs `&str`).

- [ ] **Step 3: Change the function**

```rust
pub(crate) fn vrddhi_of(v: char) -> Option<&'static str> {
    match v {
        'a' | 'A' => Some("A"),
        'i' | 'I' | 'e' | 'E' => Some("E"),
        'u' | 'U' | 'o' | 'O' => Some("O"),
        // 1.1.51 uraR raparaH: a vṛddhi substitute for f carries the r.
        'f' => Some("Ar"),
        _ => None,
    }
}
```

- [ ] **Step 4: Follow the compiler into 6.1.90**

Both arms of 6.1.90 in `adesha.rs` consume `vrddhi_of` as a `char` (pushing it into a string). Change each site to `push_str`/concatenation of the `&str` — mechanical, the compiler enumerates them. Do not change any guard.

- [ ] **Step 5: Engine tests + full suite, foreground**

```bash
mise exec -- cargo test -p panini-prakriya && mise run test
```

Expected: PASS; 2844 goldens byte-identical (no curated root reaches the `f` arm yet).

- [ ] **Step 6: Commit**

```bash
git add -A && git commit -m "feat(engine): vrddhi_of returns str and grows the 1.1.51 f->Ar arm"
```

---

### Task 3: the widened asaṁyogapūrva helper and 6.1.77

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/terms.rs` (helper rename + widen; enumerated test; `following_sarvadhatuka` doc)
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (6.4.87 gains a śnu self-guard; new 6.1.77 after 6.4.77)
- Modify: `crates/panini-prakriya/src/tinanta/adesha.rs` (6.4.106/6.4.107 call the renamed helper)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (order pin gains 6.1.77)

**Interfaces:**
- Consumes: `Tag::Tanadi`, the u vikaraṇa (Task 1).
- Produces: `vikarana_u_asamyogapurva(p: &Prakriya) -> bool` in `terms.rs` — true iff the vikaraṇa's text is `"nu"` or `"u"` AND its `u` is not conjunct-preceded. Rule `6.1.77` (`SHAP.text == "u"` + vowel-initial ending → `SHAP.text = "v"`). Tasks 6–8 rely on `tanu`/`fRu` luk-ing `hi`, `arRu` keeping it, and `tanvaH/tanuvaH` forking.

- [ ] **Step 1: Extend the enumerated helper test (failing)**

In `terms.rs`, rename `shnu_asamyogapurva_is_true_exactly_for_the_vowel_final_roots` to `vikarana_u_asamyogapurva_is_true_exactly_for_the_non_conjunct_stems` and extend its rows (keep every existing row, updating the call):

```rust
            ("tan", "u", true),    // tanādi: single n after a vowel
            ("fR", "u", true),     // tanādi: R after the vowel f
            ("kur", "u", true),    // 8b's √kṛ after 6.4.110: r after u
            ("arR", "u", false),   // guṇa'd fR: rR conjunct — arRuhi keeps hi
            ("tan", "nu", false),  // control: an n-final stem under śnu is a conjunct
```

and update the comment: the false-negative risks are now `*tanuhi` (a missed luk) and `*arRu` (a wrong one) beside the svādi pair.

- [ ] **Step 2: Run to verify it fails**

```bash
mise exec -- cargo test -p panini-prakriya vikarana_u_asamyogapurva
```

Expected: compile FAIL (no such function).

- [ ] **Step 3: Widen the helper**

Replace `shnu_asamyogapurva` in `terms.rs` with:

```rust
/// Is the vikaraṇa's final `u` *asaṁyogapūrva* — preceded by a single
/// consonant (or directly by a vowel) rather than by a conjunct?
///
/// The condition 6.4.87 inherits by anuvṛtti from 6.4.82 *er anekāco'saṁ-
/// yogapūrvasya*, and the one 6.4.106 states in its own text. Two vikaraṇa
/// shapes carry such a `u`: śnu's `nu` (the `u` preceded by śnu's own `n`,
/// so the question is whether the AṄGA ends in a vowel — hinu yes, Apnu
/// no) and tanādi's bare `u` (3.1.79 — the question is whether the aṅga's
/// final consonant follows a vowel: tanu and fRu yes, arRu no, since the
/// guṇa branch's `rR` is a conjunct; that split is exactly vidyut's
/// arRuhi-beside-fRu).
///
/// Returns false for every other SHAP text (śap/śa `a`, śyan `ya`, śnā's
/// shapes, śnam-plus-tail, adādi's empty string, and the post-6.4.107
/// remnants `n`/``), so callers still need no gaṇa test of their own.
pub(crate) fn vikarana_u_asamyogapurva(p: &Prakriya) -> bool {
    let Some(shap) = p.terms.get(SHAP) else {
        return false;
    };
    let anga: Vec<char> = p.terms[ANGA].text.chars().collect();
    // The two sounds before the vikaraṇa's `u`, nearest first.
    let (c1, c2) = match shap.text.as_str() {
        "nu" => (Some('n'), anga.last().copied()),
        "u" => (
            anga.last().copied(),
            anga.len().checked_sub(2).and_then(|i| anga.get(i).copied()),
        ),
        _ => return false,
    };
    match (c1, c2) {
        // `u` directly after a vowel: trivially not conjunct-preceded. No
        // curated root reaches this arm; it is written because it is what
        // the sūtra says, not because a form needs it.
        (Some(v), _) if is_vowel(v) => true,
        // One consonant after a vowel: hinu, tanu, fRu, kuru.
        (Some(c), Some(v)) if !is_vowel(c) && is_vowel(v) => true,
        // A conjunct (Apnu, aSnu, arRu) or nothing readable.
        _ => false,
    }
}
```

Update the two callers in `adesha.rs` (6.4.106, 6.4.107) to the new name — their guards are otherwise UNCHANGED; the widening is the whole point (`tanu`, `tanvaH/tanuvaH` fall out). Update 6.4.107's ordering comment: its mutation now also leaves `SHAP == ""` on the tanādi path, and the enumerated consumers are 6.4.87, 6.4.106, and 6.4.77's open-coded test — confirm the sentence still names every reader.

- [ ] **Step 4: 6.4.87 gains its own śnu test**

The helper now answers true for tanādi, but 6.4.87's sūtra names hu and śnu, and its action writes `"nv"`. At the top of its `apply`:

```rust
            // The sūtra names hu and śnu. Tanādi's bare `u` (which the
            // shared asaṁyogapūrva helper now also admits) is 6.1.77's
            // business below — without this test 6.4.87 would write śnu's
            // `nv` over a vikaraṇa that has no `n`.
            if p.terms[SHAP].text != "nu" {
                return false;
            }
```

- [ ] **Step 5: The 6.1.77 rule**

In `guna.rs`, insert after the 6.4.77 block, before 6.1.78:

```rust
    // 6.1.77 iko yaṇ aci: the tanādi vikaraṇa's `u` becomes `v` before a
    // vowel-initial ending. tan + u + anti → tanvanti; tan + u + ate →
    // tanvate; tan + u + Ita → tanvIta. This is the utsarga whose apavādas
    // the pipeline already carries for śnu — 6.4.87 (yaṇ, now self-guarded
    // to `nu`) and 6.4.77 (uvaṅ) — ordered above it as apavādas are
    // elsewhere; neither can contend here, since both test śnu's text and
    // this rule tests the bare `u`. vidyut-prakriya credits exactly this
    // sūtra for these cells.
    //
    // Only the vikaraṇa arm is written: no other ik-vowel hiatus survives
    // to this point in the pipeline, the same narrowness 6.1.78's three
    // arms and 6.4.77's śnu-only arm document. Widen by arm, with a
    // witness, when a root needs one.
    //
    // Ordered AFTER 7.3.84's second application: the loṭ uttama endings
    // are vowel-initial and pit, so guṇa takes `u` → `o` first and 6.1.78
    // then yields tanavAni — this rule's `u` test declines on the `o`, the
    // same self-guarding 6.4.87/6.4.77 rely on for ApnavAni.
    Rule {
        id: "6.1.77",
        name: "iko yaR aci",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if p.terms[SHAP].text != "u" || !p.terms[SHAP].has(Tag::Vikarana) {
                return false;
            }
            let Some(next) = p.terms.get(ENDING).and_then(|t| t.text.chars().next()) else {
                return false;
            };
            if !is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "v".into();
            p.record("6.1.77", "iko yaR aci", before);
            true
        },
    },
```

Pin `"6.1.77"` in `tinanta_rule_order_is_pinned` between `"6.4.77"` and `"6.1.78"`.

- [ ] **Step 6: Discharge `following_sarvadhatuka`'s restore trigger**

Its doc says it "must become a real guard the moment an ārdhadhātuka affix enters scope." That moment is now. The helper's callers (7.3.84's first application, 7.3.86) use it for 1.1.5's kṅiti test, and 1.1.5 reads the follower's ṅit-ness whatever its saṁjñā — the ārdhadhātuka `u` carries no ṅit, so the answer is right for the right reason. Replace the last paragraph of its doc comment with:

```
/// The name is now one affix too narrow: since 3.1.79, the follower can be
/// the tanādi `u`, which is ārdhadhātuka, not sārvadhātuka. The callers'
/// question — 1.1.5's "is the follower kṅit?" — is saṁjñā-independent, and
/// the u is never ṅit (the second 1.2.4 excludes it by tag), so returning
/// it is correct. The guard this comment used to demand exists as
/// Tag::Ardhadhatuka on the u itself; a future ārdhadhātuka affix that CAN
/// be ṅit is the new restore trigger.
```

- [ ] **Step 7: Engine tests + full suite, foreground**

```bash
mise exec -- cargo test -p panini-prakriya && mise run test
```

Expected: PASS, 2844 goldens byte-identical (6.1.77 and the `"u"` helper arm are unreachable without tanādi roots; 6.4.87's new test is provably behavior-preserving for `"nu"`).

- [ ] **Step 8: Commit**

```bash
git add -A && git commit -m "feat(engine): 6.1.77 iko yaR aci, and the asamyogapurva helper reads any u-final vikarana

6.4.87 self-guards to snu's nu; 6.4.106/107 serve both ganas from the one
widened guard. following_sarvadhatuka's restore trigger discharged in doc.
All 2844 goldens byte-identical."
```

---

### Task 4: the 7.3.86 vikalpa arm and convergent-fork collapse

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (nitya 7.3.86 gains a gaṇa guard; new vikalpa entry after it)
- Modify: `crates/panini-prakriya/src/controller.rs` (`run_pipeline` collapses same-text live branches)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (order pin gains the second `7.3.86`)

**Interfaces:**
- Consumes: `Tag::Tanadi`, the u vikaraṇa.
- Produces: a second `Rule` with id `"7.3.86"`, `vikalpa: true`, firing only for tanādi ik-upadhā aṅgas before the `u`; `run_pipeline` returning at most one live branch per surface text (first — declined — occurrence kept). Task 7's `ALTERNATES` keys `7.3.86`; the audit's `n_branches == n_forms` invariant stays true.

- [ ] **Step 1: Write the failing tests**

In `guna.rs`'s tests:

```rust
    #[test]
    fn pugantalaghupadhasya_tanadi_arm_is_vikalpa_and_ngit_blind() {
        // tfR + u + anti: the vikalpa arm fires (its trigger is the
        // ārdhadhātuka u, on which 1.1.5 has no purchase — vidyut derives
        // tarRvanti), while the nitya entry declines the gaṇa entirely.
        let mut p = Prakriya {
            terms: vec![Term::new("tfR"), Term::new("u"), Term::new("anti")],
            ..Default::default()
        };
        p.terms[0].add(Tag::Dhatu);
        p.terms[0].add(Tag::Tanadi);
        p.terms[1].add(Tag::Vikarana);
        p.terms[1].add(Tag::Ardhadhatuka);
        p.terms[2].add(Tag::Ngit);
        let mut entries = rules().filter(|r| r.id == "7.3.86");
        let nitya = entries.next().unwrap();
        let vikalpa = entries.next().expect("the tanādi arm");
        assert!(!nitya.vikalpa);
        assert!(vikalpa.vikalpa);
        assert!(!(nitya.apply)(&mut p), "gaṇa 8 belongs to the vikalpa arm");
        assert!((vikalpa.apply)(&mut p));
        assert_eq!(p.terms[0].text, "tarR");
    }

    #[test]
    fn pugantalaghupadhasya_tanadi_arm_declines_a_upadha_and_final_ik() {
        // tan (a upadhā — nothing to guṇate) and kf (ik FINAL — 7.3.84's
        // business): both outside the gaṇasūtra's four.
        for root in ["tan", "kf"] {
            let mut p = Prakriya {
                terms: vec![Term::new(root), Term::new("u"), Term::new("ti")],
                ..Default::default()
            };
            p.terms[0].add(Tag::Dhatu);
            p.terms[0].add(Tag::Tanadi);
            p.terms[1].add(Tag::Vikarana);
            let vikalpa = rules().filter(|r| r.id == "7.3.86").nth(1).unwrap();
            assert!(!(vikalpa.apply)(&mut p), "{root}");
        }
    }
```

In `controller.rs`'s tests (using its existing `p1` fixture and vikalpa test-rule idiom — read them first and match):

```rust
    #[test]
    fn convergent_forks_collapse_to_the_declined_branch() {
        // Two branches that assemble the same text are one form. The first
        // real case is fR's laṅ: A+fR and A+arR both surface ArRot once
        // 6.1.90's vṛddhi runs. Keep the FIRST (declined) branch so index
        // 0 stays the ruleless derivation.
        // Build: one vikalpa rule that fires but leaves the text equal.
        let noop_fork = Rule {
            id: "test.noop",
            name: "noop",
            kind: RuleKind::Vidhi,
            vikalpa: true,
            apply: |p| {
                let before = p.snapshot();
                p.record("test.noop", "noop", before);
                true
            },
        };
        let out = run_pipeline(p1("x"), &[&[noop_fork]]);
        assert_eq!(out.len(), 1, "the converged fork must be pruned");
        assert!(out[0].log.iter().all(|s| s.sutra != "test.noop"));
    }
```

- [ ] **Step 2: Run to verify they fail**

```bash
mise exec -- cargo test -p panini-prakriya pugantalaghupadhasya_tanadi
mise exec -- cargo test -p panini-prakriya convergent_forks
```

Expected: FAIL (one 7.3.86 entry; two branches).

- [ ] **Step 3: The nitya guard and the vikalpa arm**

In the existing (nitya) 7.3.86's `apply`, after the 1.1.5 test:

```rust
            if p.terms[ANGA].has(Tag::Tanadi) {
                // Gaṇa 8 is the vikalpa arm's below (Kaumudī 2547.1).
                return false;
            }
```

Immediately after the nitya entry, insert:

```rust
    // 7.3.86 pugantalaghūpadhasya ca — VIKALPA ARM, gaṇa 8 only. The four
    // tanādi roots whose laghu upadhā is an ik guṇate OPTIONALLY before
    // the vikaraṇa `u`: kziRoti/kzeRoti, fRoti/arRoti, tfRoti/tarRoti,
    // GfRoti/GarRoti. The optionality is not the sūtra's own: it is the
    // tanādi gaṇasūtra the Siddhānta-kaumudī carries (vidyut-prakriya
    // applies it at Kaumudī 2547.1, an optional guṇa-apavāda tag on
    // exactly those four upadeśas). This engine keeps the Pāṇinian id on
    // the branch that applies guṇa and records the Kaumudī source here,
    // so ALTERNATES keys stay inside the Aṣṭādhyāyī.
    //
    // Guarded structurally — gaṇa 8, an ik upadhā, the `u` still standing
    // — not by a root list: within gaṇa 8 that selects exactly the
    // gaṇasūtra's four (a-upadhā roots have nothing to guṇate; √kṛ's ik is
    // FINAL, 7.3.84's business). The nitya entry declines the gaṇa on the
    // same tag, so the two entries partition and can never double-apply.
    //
    // NO 1.1.5 test, deliberately: the trigger is the ārdhadhātuka `u`
    // (never ṅit — see 3.1.79), not the tiṅ ending, which is why the guṇa
    // branch exists even before ṅit endings (tarRvanti). This is the
    // hardcoded-follower lesson of the adādi slices applied in advance.
    Rule {
        id: "7.3.86",
        name: "pugantalaGUpaDasya ca",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Tanadi) {
                return false;
            }
            if p.terms.get(SHAP).map(|t| t.text.as_str()) != Some("u") {
                return false;
            }
            let chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let n = chars.len();
            if n < 2 || is_vowel(chars[n - 1]) {
                return false;
            }
            if !matches!(chars[n - 2], 'i' | 'u' | 'f' | 'x') {
                return false;
            }
            let Some(g) = guna_of(chars[n - 2]) else {
                return false;
            };
            let before = p.snapshot();
            let mut s: String = chars[..n - 2].iter().collect();
            s.push_str(g);
            s.push(chars[n - 1]);
            p.terms[ANGA].text = s;
            p.record("7.3.86", "pugantalaGUpaDasya ca", before);
            true
        },
    },
```

Pin the second `"7.3.86"` in the order test directly after the first, with a comment citing the double-7.3.84/double-1.2.4 precedent (both entries are real; do not deduplicate).

- [ ] **Step 4: The collapse in `run_pipeline`**

At the end of `run_pipeline`, before `branches`:

```rust
    // Convergent forks collapse: two LIVE branches that assemble the same
    // text are one form, and every consumer — the golden suite's
    // derivation-set comparison, the audit's n_branches == n_forms
    // invariant ("no cell may yield two live branches with the same
    // text") — is written against that invariant. The first real case is
    // the 7.3.86 vikalpa arm under 6.1.90's āṭ-vṛddhi: A+fR and A+arR
    // both surface ArRot. Keep the FIRST occurrence — the declined
    // branch — so index 0 remains the ruleless derivation. Blocked
    // branches are exempt: their partial text is not a surface form and
    // callers filter them on `blocked`.
    let mut seen: Vec<String> = Vec::new();
    branches.retain(|b| {
        if b.blocked {
            return true;
        }
        let text = b.text();
        if seen.contains(&text) {
            false
        } else {
            seen.push(text);
            true
        }
    });
```

(If `Prakriya::text` is not visible from `controller.rs`, follow the compiler — it is defined on `Prakriya` and used by the `panini` crate; adjust the import, not the design.)

- [ ] **Step 5: Engine tests + full suite, foreground**

```bash
mise exec -- cargo test -p panini-prakriya && mise run test
```

Expected: PASS; 2844 goldens byte-identical (no existing fork converges — the suite run is the proof).

- [ ] **Step 6: Commit**

```bash
git add -A && git commit -m "feat(engine): 7.3.86 gains its tanadi vikalpa arm; convergent forks collapse

The Kaumudi 2547.1 optionality keyed by the Paninian id; run_pipeline now
keeps one live branch per surface, declined first. Goldens byte-identical."
```

---

### Task 5: the nine data rows

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (nine `Dhatu` rows appended after `07.0017`; `dhatus().len()`; new row test; `stored_form`'s z-arm; `gana_matches_dhatupatha_prefix` gains `08` ↔ `Tanadi`)

**Interfaces:**
- Consumes: `Gana::Tanadi` (Task 1).
- Produces: rows addressable as `08.0001`–`08.0009`, which Tasks 6–8 resolve through `dhatus()`.

- [ ] **Step 1: The failing row test**

Add, following the `rudhadi_rows_are_the_twenty_five_curated_roots` pattern:

```rust
    #[test]
    fn tanadi_rows_are_the_nine_curated_roots() {
        // Slice 8a: the gaṇa's nine plain rows, on 3.1.79's bare u. Seven
        // svarita-it (1.3.72, ubhayapadī), two anudātta (1.3.12,
        // ātmanepadī). 08.0010 qukf\Y — √kṛ, ñit, ubhayapadī — is slice
        // 8b's, with the 6.4.108–110 specials; until it lands the gaṇa is
        // PARTIAL at 9 of its 10 dhātupāṭha rows.
        let expected = vec![
            ("08.0001", "tan", PadaAssignment::Ubhayapada),
            ("08.0002", "san", PadaAssignment::Ubhayapada),
            ("08.0003", "kzaR", PadaAssignment::Ubhayapada),
            ("08.0004", "kziR", PadaAssignment::Ubhayapada),
            ("08.0005", "fR", PadaAssignment::Ubhayapada),
            ("08.0006", "tfR", PadaAssignment::Ubhayapada),
            ("08.0007", "GfR", PadaAssignment::Ubhayapada),
            ("08.0008", "van", PadaAssignment::Atmanepada),
            ("08.0009", "man", PadaAssignment::Atmanepada),
        ];
        let actual: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.gana == Gana::Tanadi)
            .map(|d| (d.dhatupatha, d.code, d.pada))
            .collect();
        assert_eq!(actual, expected);
    }
```

(Transcribe the exact comparison idiom from the rudhādi row test — if it compares differently, match it.)

- [ ] **Step 2: Run to verify it fails**

```bash
mise exec -- cargo test -p panini-data tanadi_rows
```

Expected: FAIL — empty actual.

- [ ] **Step 3: The nine rows**

Append to `DHATUS` after the `07.0017` (√bhuj) row:

```rust
    Dhatu {
        // 08.0001 tanu~^ vistAre. The gaṇa's eponym and its plainest row:
        // a-upadhā (nothing for 7.3.86 to touch), n-final after a vowel
        // (asaṁyogapūrva: tanu, tanvaH/tanuvaH), svarita-it → 1.3.72 →
        // ubhayapadī. tanoti / tanute.
        dhatupatha: "08.0001",
        code: "tan",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "vistAre",
    },
    Dhatu {
        // 08.0002 zaRu~^ dAne. Stored per 6.1.64 dhAtvAdeH SaH saH like
        // √ṣṭigh (stiG): the upadeśa's z becomes s, and with the z gone
        // its conditioned retroflex R reverts to n (nimitta-nāśa) —
        // sanoti / sanute, as vidyut derives. stored_form's z-arm carries
        // the same reversal so the resolve test derives san, not saR.
        dhatupatha: "08.0002",
        code: "san",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "dAne",
    },
    Dhatu {
        // 08.0003 kzaRu~^ hiMsAyAm. a-upadhā like √tan; the root's own R
        // (retroflex by its z) stays. kzaRoti / kzaRute.
        dhatupatha: "08.0003",
        code: "kzaR",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 08.0004 kziRu~^ hiMsAyAm. First of the four ik-upadhā roots the
        // tanādi gaṇasūtra (Kaumudī 2547.1; see guna.rs's 7.3.86 vikalpa
        // arm) forks: kziRoti / kzeRoti, and likewise through the whole
        // paradigm. Same artha as its neighbour kzaR — the number, not
        // the meaning, is the identity.
        dhatupatha: "08.0004",
        code: "kziR",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 08.0005 fRu~^ gatO. Vowel-initial AND ik-upadhā (the f is both):
        // the 7.3.86 fork (fRoti/arRoti), the laṅ āṭ (6.4.72 + 6.1.90,
        // whose f → Ar vṛddhi arm this root is the first to reach:
        // ArRot), and the fork CONVERGING under that vṛddhi — A+fR and
        // A+arR are both ArR — which is what run_pipeline's convergent-
        // fork collapse exists for. In loṭ the two stems split the
        // asaṁyogapūrva test: fRu luks hi, arRu (rR conjunct) keeps it —
        // fRu beside arRuhi, the widened helper's sharpest witness.
        dhatupatha: "08.0005",
        code: "fR",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "gatO",
    },
    Dhatu {
        // 08.0006 tfRu~^ adane. ik-upadhā fork: tfRoti / tarRoti.
        dhatupatha: "08.0006",
        code: "tfR",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "adane",
    },
    Dhatu {
        // 08.0007 GfRu~^ dIptO. ik-upadhā fork: GfRoti / GarRoti.
        dhatupatha: "08.0007",
        code: "GfR",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "dIptO",
    },
    Dhatu {
        // 08.0008 vanu~\ yAcane. Anudātta → 1.3.12 → ātmanepadī: vanute.
        // vidyut-prakriya ALSO derives vanoti, via Kaumudī 2547.2 — a
        // gaṇasūtra that optionally removes this one root's anudātta-it.
        // Curated on the dhātupāṭha marker alone, the same record-don't-
        // model posture as 1.3.72's sense condition: the audit enumerates
        // this table's padas, so the parasmaipada column it does not have
        // is a documented deviation, not a latent diff.
        dhatupatha: "08.0008",
        code: "van",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Atmanepada,
        artha: "yAcane",
    },
    Dhatu {
        // 08.0009 manu~\ avaboDane. Anudātta → 1.3.12 → ātmanepadī:
        // manute. Unlike van, no gaṇasūtra clouds it — vidyut derives no
        // parasmaipada either.
        dhatupatha: "08.0009",
        code: "man",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Atmanepada,
        artha: "avaboDane",
    },
```

Transcribe the pada column against the markers, not from muscle memory: the seven `~^` rows are all `Ubhayapada`, the two `~\` rows `Atmanepada`. A slip is exactly what `curated_pada_agrees_with_upadesha_markers` exists to catch — if it flags a row, the row is wrong, not the test.

- [ ] **Step 4: `stored_form`'s z-arm and the count**

In `dhatvadeh_sha_sa`, the z-prefix arm becomes:

```rust
        if let Some(rest) = code.strip_prefix('z') {
            let rest = rest
                .strip_prefix('w')
                .map_or_else(|| rest.to_string(), |r| format!("t{r}"));
            // With the z gone, the retroflexion it conditioned goes too
            // (nimitte naṣṭe naimittikasya apy anivṛttiḥ is the paribhāṣā
            // AGAINST this — but 6.1.64 is nipātana territory and the
            // attested stems are san (zaRu~), stiG (zwiGa~): the R
            // reverts, the w hardens to t). vidyut derives sanoti.
            return format!("s{}", rest.replace('R', "n"));
        }
```

Update `dhatus().len()` to **76**. Add the `"08"` ↔ `Gana::Tanadi` arm to `gana_matches_dhatupatha_prefix` (and any exhaustive `Gana` match the compiler flags).

- [ ] **Step 5: Run the whole `panini-data` suite**

```bash
mise exec -- cargo test -p panini-data
```

Expected: PASS. Three tests are load-bearing and must pass **without being edited** (beyond Step 4's stated changes): `dhatupatha_numbers_resolve_upstream` (san's z-arm reversal, everything else a plain it-strip), `curated_pada_agrees_with_upadesha_markers` (seven `~^` → Ubhayapada, two `~\` → Atmanepada), and the uniqueness checks. If any fails, **stop**: a code or pada that must be argued for is a finding — record it and report.

- [ ] **Step 6: Confirm the expected-red window is exactly one test**

```bash
mise exec -- cargo test -p panini --test paradigm 2>&1 | tail -20
```

Expected: `paradigm_covers_every_enumerable_cell` FAILS listing **64** unpinned triples (7 ubhaya × 2 padas + 2 ātma × 1 pada = 16, × 4 lakāras). Anything else failing must be understood before continuing.

- [ ] **Step 7: Commit**

```bash
git add crates/panini-data/src/lib.rs
git commit -m "feat(data): the nine tanadi rows

tan, san (6.1.64 stored), kzaR, kziR, fR, tfR, GfR ubhayapadi by 1.3.72;
van (Kaumudi 2547.2 recorded, not modelled) and man atmanepadi by 1.3.12.
paradigm_covers_every_enumerable_cell red until the goldens land."
```

---

### Task 6: the cross-implementation audit — the blocking gate

**Files:**
- Modify: `tools/audit/panini_full_audit.rs` (`gana_name` Tanadi arm; corpus totals; module-header totals)
- Modify: `tools/audit/README.md` (totals; "Last recorded result")

**Interfaces:**
- Consumes: Tasks 1–5.
- Produces: a recorded zero-difference verdict and the measured forms total `<N>`, which Task 7 cross-checks.

**This task blocks Task 7.** No golden is pinned before the audit certifies it.

- [ ] **Step 1: Vidyut checkout at the vendored commit**

```bash
head -20 /workspace/data/dhatupatha.tsv | grep -i commit
git -C /tmp/vidyut-full log --oneline -1
```

Both must show the same commit (currently `8da2f90...`). If `/tmp/vidyut-full` is missing, clone and checkout as `tools/audit/README.md` describes.

- [ ] **Step 2: Point the dev-deps at THIS checkout and copy the harness**

In `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml`, set the two dev-dependency paths to the checkout being audited (the worktree's `crates/panini` and `crates/panini-data` if executing in a worktree — verify with `git -C <path> branch --show-current`). Then:

```bash
cp <checkout>/tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
```

**Copy the committed harness; do not rewrite it.** Then make the ONE structural edit it needs this slice, in `<checkout>/tools/audit/panini_full_audit.rs` FIRST and re-copy: `gana_name` gains

```rust
        PGana::Tanadi => "Tanadi",
```

(the compiler forces it — the match is exhaustive).

- [ ] **Step 3: Negative control FIRST**

```bash
cd /tmp/vidyut-full/vidyut-prakriya
PANINI_AUDIT_PERTURB=entry mise exec rust@1.98.0 -- cargo run --release --example panini_full_audit
```

Expected: **exit 1**, 36 differing √bhū cells. If it passes, stop — every later result is worthless.

- [ ] **Step 4: Real run; set the totals**

```bash
mise exec rust@1.98.0 -- cargo run --release --example panini_full_audit 2>&1 | tee /tmp/8a-audit.txt
```

The corpus block prints before the assertions panic on stale totals. Read `roots: 76`, `cells: 3420`, `forms: <N>`; set the three assertions (in BOTH copies — the repo's and the example's):

```rust
    assert_eq!(roots_seen.len(), 76, "curated roots");
    assert_eq!(n_cells, 3420, "cells: 380 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, <N>, "forms: 3420 cells + <N - 3420> ALTERNATES rows");
```

Update the module header's totals (67/2844/3338 → 76/3420/`<N>`, the "494 ALTERNATES rows" and "2844-cell table" mentions) in the same edit. Re-run: expected `AUDIT PASSED: 3420 cells, <N> forms, zero differences.` The probe projects `<N> - 3420` ≈ **421** — a recognition tripwire, not a value to type.

- [ ] **Step 5: If there ARE differences**

The spec fixes the posture: **stop and diagnose; ship what passes, drop what doesn't, record why.** Read the `DIFF` lines' two derivation sets, identify the sūtra at fault from the two traces (the vidyut probe log in this plan's provenance note names the expected rule per cell), and:
- an engine bug in a rule Tasks 1–4 added → fix in that task's file with a unit test, re-run the FULL suite (byte-identity!) and this audit;
- a root vidyut derives differently for a reason outside scope → drop the row (reverting that part of Task 5), record the deferral in the row-test comment and the spec, recompute every count in this plan (a partial slice states its own partiality);
- never patch golden data toward either engine.

- [ ] **Step 6: Record and commit**

Update `tools/audit/README.md`: totals, and a new "Last recorded result" entry in the existing shape — date, slice (`tanadi 8a`), vidyut commit, 76 roots / 3420 cells / `<N>` forms, zero differences, entry control verified failing first. Name the two structural engine changes the verdict now covers (the u-vikaraṇa generalization; the convergent-fork collapse).

```bash
cd <checkout>
git add tools/audit/
git commit -m "test(audit): tanadi 8a is byte-identical to vidyut

76 roots / 3420 cells / <N> forms, zero differences at vidyut <commit>,
entry control verified failing first."
```

---

### Task 7: the goldens, generated

**Files:**
- Create then delete: `crates/panini/tests/print_8a_goldens.rs`
- Create: `crates/panini/tests/paradigm/data/tanadi.rs`
- Modify: `crates/panini/tests/paradigm/data/mod.rs` (mod line + both concat arrays)
- Modify: `crates/panini/tests/paradigm/main.rs` (`VIKALPA_RULES`, shape test + doc comment, ambiguity set)

**Interfaces:**
- Consumes: the audit verdict and `<N>` (Task 6).
- Produces: 64 `PARADIGM` blocks + `ALTERNATES` rows in `data/tanadi.rs`; measured buckets and key counts; the re-measured ambiguity set. Task 8's pins and Tasks 9–10's prose quote these measurements.

- [ ] **Step 1: The throwaway generator**

Create `crates/panini/tests/print_8a_goldens.rs` as a copy of 7d's generator (its full text is in `docs/superpowers/plans/2026-08-20-rudhadi-gana-7d.md`, Task 3 Step 1) with exactly these deltas:
- `NEW_ROOTS`: `["08.0001", "08.0002", "08.0003", "08.0004", "08.0005", "08.0006", "08.0007", "08.0008", "08.0009"]`
- `VIKALPA_RULES` (the generator's local copy): add `"7.3.86"` → `&["7.1.35", "3.4.111", "6.4.107", "7.3.86", "8.2.74", "8.2.75", "8.4.65", "8.4.56"]`
- header comment: slice 8a, and note the generator's per-root loop already iterates `d.pada.padas()`, which yields both padas for the seven ubhayapadī rows.

- [ ] **Step 2: Run it**

```bash
mise exec -- cargo test -p panini --test print_8a_goldens -- --nocapture 2>&1 | tee /tmp/8a-goldens.txt
```

Expected: PASS, printing 64 block lines, the `ALTERNATES` rows, buckets over 576 new cells, per-key counts. Sanity-check **eight single-form strings** (laṭ prathama eka; from the probe):

| root | pada | expected |
|---|---|---|
| `08.0001` | parasmai | `tanoti` |
| `08.0001` | ātmane | `tanute` |
| `08.0002` | parasmai | `sanoti` |
| `08.0003` | parasmai | `kzaRoti` |
| `08.0008` | ātmane | `vanute` |
| `08.0009` | ātmane | `manute` |
| `08.0004` | parasmai | `kziRoti` **plus alternate** `kzeRoti` keyed `7.3.86` |
| `08.0005` | parasmai | `fRoti` plus alternate `arRoti` keyed `7.3.86` |

Also confirm: `08.0005` laṅ parasmaipada prathama eka prints **two** forms only (`ArRod` golden, `ArRot` alternate keyed `8.4.56`, no `7.3.86`-keyed row) — the convergence collapse working; and `08.0005` loṭ madhyama eka is a six-form cell (`fRu`, `fRutAd`, `fRutAt`, `arRuhi`, `arRutAd`, `arRutAt` in some assignment of golden/alternates). If any differs, stop: the audit passed, so a mismatch is generator addressing or a plan expectation being wrong — diagnose which before pasting anything.

- [ ] **Step 3: Create `data/tanadi.rs` and wire it**

Create the file with the same header shape as `data/svadi.rs` (module doc pointing at `super`), declaring `pub const PARADIGM: &[ParadigmRow]` and `pub const ALTERNATES: &[AlternateRow]`, and paste the generator's output verbatim from `/tmp/8a-goldens.txt` — no retyping; `rustfmt` owns the formatting. In `data/mod.rs`: add `pub mod tanadi;` to the mod list and `tanadi::PARADIGM` / `tanadi::ALTERNATES` as the LAST entry of each concat array (the mod doc's own instruction), and extend the mod doc's gaṇa enumeration with `08 tanādi`.

- [ ] **Step 4: `VIKALPA_RULES` and the shape test**

In `paradigm/main.rs`: add `"7.3.86"` to `VIKALPA_RULES` (mirroring the generator). In `derivation_set_shape_matches_the_audited_numbers`: `total_cells` → 3420 (380 blocks), each bucket updated by the generator's printed distribution (old values in "Numbers this plan changes"), `ALTERNATES.len()` → 494 + measured, each `key_count` updated per the generator's per-key counts — including the NEW `7.3.86`-bearing keys, which need new `key_count` assertions. Extend the fives/sixes assertion messages to name the new record-holding cells the generator reports. **Cross-check: `3420 + ALTERNATES.len() == <N>`** from Task 6 — a mismatch means a paste error; resolve here.

Update the long doc comment above the test: totals, key enumeration, the audit verdict line, and its prose claims (re-check each against the measured output — in particular any "every root forks in loṭ and laṅ" style sentence: van/man fork in laṭ/laṅ uttama and loṭ tātaṅ cells per the probe, but verify from output, and reword rather than renumber if a claim no longer holds).

- [ ] **Step 5: Delete the generator; re-measure the ambiguity set**

```bash
rm crates/panini/tests/print_8a_goldens.rs
mise exec -- cargo test -p panini --test paradigm pada_ambiguous
```

Seven new ubhayapadī roots make new pada-ambiguous surfaces near-certain (`tanutAm`, `tanvAtAm`-shape collisions). If it fails, re-derive the set from scratch — temporarily assert against an empty vec, transcribe the printed real set, restore — and extend the comment naming which surfaces the tanādi rows contributed. Do not hand-extend from the diff.

- [ ] **Step 6: Full suite, foreground**

```bash
mise run test
```

Expected: PASS, `paradigm_covers_every_enumerable_cell` closes the red window. Budget 30+ minutes; foreground.

- [ ] **Step 7: Commit**

```bash
git add crates/panini/tests/paradigm/
git commit -m "test(paradigm): tanadi 8a goldens, generated from the audited engine

64 blocks / 576 cells in data/tanadi.rs; VIKALPA_RULES gains 7.3.86; the
corpus reaches 76 roots, 3420 cells, <N> forms."
```

---

### Task 8: the trace pins

**Files:**
- Create: `crates/panini/tests/trace/tanadi.rs`
- Modify: `crates/panini/tests/trace/main.rs` (mod line)

**Interfaces:**
- Consumes: helpers `cell_trace`, `at` (`crate::helpers`); the pinned goldens (Task 7); `panini_prakriya::derive`.
- Produces: the gaṇa's ordered-trace witnesses.

- [ ] **Step 1: Write the module**

Create `crates/panini/tests/trace/tanadi.rs` (header per the other gaṇa modules) with these tests. The surfaces are audited goldens; the order/absence assertions are the grammar. Use the imports the other trace modules use (`crate::helpers::{cell_trace, at}`, plus `panini_data::{Lakara, Pada, Purusha, Vacana, dhatus}` and `panini_prakriya::derive` where branch-level access is needed).

```rust
#[test]
fn tanoti_trace_is_the_ardhadhatuka_u_core() {
    // tan laT P.E. 3.1.79 inserts the bare u; the SECOND 7.3.84 guṇates
    // it before pit ti. Load-bearing absences: no 1.2.4 anywhere (ti is
    // pit; the u is ārdhadhātuka and the second 1.2.4's Sarvadhatuka
    // guard excludes it — tanoti exists BECAUSE that guard does), and no
    // second 1.3.9 (the bare u has no anubandha to strip, unlike śnu).
    let (text, t) = cell_trace("08.0001", Lakara::Lat, Pada::Parasmaipada,
        Purusha::Prathama, Vacana::Eka);
    assert_eq!(text, "tanoti", "got {t:?}");
    assert!(at(&t, "3.1.79") < at(&t, "7.3.84"), "got {t:?}");
    assert!(!t.contains(&"1.2.4".to_string()), "got {t:?}");
    assert!(!t.contains(&"3.1.68".to_string()), "got {t:?}");
}

#[test]
fn tanvanti_trace_credits_6_1_77_not_the_shnu_rules() {
    // tan laT P.B. u -> v before the vowel-initial ṅit ending is 6.1.77
    // iko yaR aci — the sūtra vidyut credits — and must NOT be 6.4.87
    // (names hu/śnu) or 6.4.77 (uvaṅ): a widened-guard regression on
    // either would derive the same surface by the wrong rule, which is
    // exactly what a trace pin exists to catch.
    let (text, t) = cell_trace("08.0001", Lakara::Lat, Pada::Parasmaipada,
        Purusha::Prathama, Vacana::Bahu);
    assert_eq!(text, "tanvanti", "got {t:?}");
    assert!(t.contains(&"6.1.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.87".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.77".to_string()), "got {t:?}");
}

#[test]
fn tanute_trace_opens_with_1_3_72() {
    // tan laT ātmanepada P.E: the svarita-it row reaches 1.3.72 (not
    // 1.3.12, not 1.3.66), and the u then survives unguṇated behind the
    // ṅit te.
    let (text, t) = cell_trace("08.0001", Lakara::Lat, Pada::Atmanepada,
        Purusha::Prathama, Vacana::Eka);
    assert_eq!(text, "tanute", "got {t:?}");
    assert!(at(&t, "1.3.72") < at(&t, "3.1.79"), "got {t:?}");
    assert!(!t.contains(&"1.3.66".to_string()), "got {t:?}");
}

#[test]
fn trnoti_forks_on_7_3_86_and_only_there() {
    // tfR laT P.E: the Kaumudī-2547.1 optionality, keyed by the Pāṇinian
    // id. Branch 0 (declined) is the golden tfRoti with NO 7.3.86 in its
    // log; the other live branch is tarRoti WITH it. Exactly two live
    // branches — the vikalpa arm is the only fork in this cell.
    let d = dhatus().iter().find(|d| d.dhatupatha == "08.0006").unwrap();
    let branches: Vec<_> = derive(d, Lakara::Lat, Pada::Parasmaipada,
        Purusha::Prathama, Vacana::Eka)
        .into_iter().filter(|p| !p.blocked).collect();
    let texts: Vec<String> = branches.iter().map(|p| p.text()).collect();
    assert_eq!(texts, vec!["tfRoti", "tarRoti"], "declined first");
    assert!(!branches[0].log.iter().any(|s| s.sutra == "7.3.86"));
    assert!(branches[1].log.iter().any(|s| s.sutra == "7.3.86"));
}

#[test]
fn rnu_and_arnuhi_split_the_asamyogapurva_test() {
    // fR loT M.E: one cell, two stems, opposite 6.4.106 verdicts. The
    // declined stem fRu (R after the vowel f) luks hi; the guṇa stem
    // arRu (rR conjunct) keeps it — and 6.4.101 must not touch that hi
    // (the sound before it is u, not jhal). The widened helper's whole
    // truth table in one cell.
    let d = dhatus().iter().find(|d| d.dhatupatha == "08.0005").unwrap();
    let branches: Vec<_> = derive(d, Lakara::Lot, Pada::Parasmaipada,
        Purusha::Madhyama, Vacana::Eka)
        .into_iter().filter(|p| !p.blocked).collect();
    let texts: Vec<String> = branches.iter().map(|p| p.text()).collect();
    assert!(texts.contains(&"fRu".to_string()), "got {texts:?}");
    assert!(texts.contains(&"arRuhi".to_string()), "got {texts:?}");
    let frnu = branches.iter().find(|p| p.text() == "fRu").unwrap();
    assert!(frnu.log.iter().any(|s| s.sutra == "6.4.106"));
    let arnuhi = branches.iter().find(|p| p.text() == "arRuhi").unwrap();
    assert!(!arnuhi.log.iter().any(|s| s.sutra == "6.4.106"));
    assert!(!arnuhi.log.iter().any(|s| s.sutra == "6.4.101"));
}

#[test]
fn arnot_trace_reaches_the_f_arm_of_vrddhi_and_the_fork_converges() {
    // fR laN P.E. 6.4.72 puts the āṭ on, 6.1.90 contracts A+f to Ar —
    // the FIRST golden derivation through vrddhi_of's f arm (the 7d
    // √und/Onad story, repeated) — and the 7.3.86 fork CONVERGES under
    // it: A+fR and A+arR are both ArR, so run_pipeline's collapse must
    // leave exactly two live branches (the 8.2.39/8.4.56 pair), neither
    // carrying 7.3.86.
    let d = dhatus().iter().find(|d| d.dhatupatha == "08.0005").unwrap();
    let branches: Vec<_> = derive(d, Lakara::Lan, Pada::Parasmaipada,
        Purusha::Prathama, Vacana::Eka)
        .into_iter().filter(|p| !p.blocked).collect();
    let texts: Vec<String> = branches.iter().map(|p| p.text()).collect();
    assert_eq!(texts, vec!["ArRod", "ArRot"], "collapsed to the 8.4.56 pair");
    for b in &branches {
        assert!(!b.log.iter().any(|s| s.sutra == "7.3.86"),
            "a surviving branch may not carry the converged fork's rule");
        assert!(at(&b.log.iter().map(|s| s.sutra.clone()).collect::<Vec<_>>(), "6.4.72")
            < at(&b.log.iter().map(|s| s.sutra.clone()).collect::<Vec<_>>(), "6.1.90"));
    }
}

#[test]
fn tanu_trace_is_the_hi_luk() {
    // tan loT M.E, declined branch: 6.4.106 luks hi behind the widened
    // helper — the tanādi twin of svādi's hinu.
    let (text, t) = cell_trace("08.0001", Lakara::Lot, Pada::Parasmaipada,
        Purusha::Madhyama, Vacana::Eka);
    assert_eq!(text, "tanu", "got {t:?}");
    assert!(t.contains(&"6.4.106".to_string()), "got {t:?}");
}
```

If a `cell_trace`-based assertion fails on the SURFACE, the golden wins (the audit certified it) — re-read the golden and fix the test's expectation. If it fails on an order/absence assertion, that is a real engine finding: stop and diagnose, do not weaken the pin.

- [ ] **Step 2: Wire and run**

Add `mod tanadi;` to `trace/main.rs`'s mod list (alphabetical position). Then:

```bash
mise exec -- cargo test -p panini --test trace
```

Expected: PASS — the new tests plus every existing one.

- [ ] **Step 3: Commit**

```bash
git add crates/panini/tests/trace/
git commit -m "test(trace): tanadi's ordered witnesses

6.1.77 credited over the snu rules; the 7.3.86 fork and its convergence
under 6.1.90's new f-arm; the fRu/arRuhi asamyogapurva split; the
ardhadhatuka u escaping 1.2.4."
```

---

### Task 9: the mutation gate

**Files:**
- Modify: `AGENTS.md` (the cargo-mutants paragraph)

**Interfaces:**
- Consumes: everything above.
- Produces: the measured floor and campaign record Task 10's prose cites.

- [ ] **Step 1: Measure the uncontended floor**

```bash
cd <checkout> && time mise run test 2>&1 | tee /tmp/8a-floor.txt
```

Alone, foreground, nothing else running. Record per-binary times and the total. **Measure; do not scale** — cell count has failed as a multiplier seven consecutive slices. Prior floor: 1132.12s at 2844 cells (under 1.98.0); this slice is +20.3% cells, the largest growth since the gaṇa began, so treat any projection as a tripwire only.

- [ ] **Step 2: Sanity-check the cap**

Multiply the measured total by the recorded `-j 4` contention range (1.02×–1.43×). If the projection exceeds **4800s**, stop and report — cap changes are recorded in `AGENTS.md` and `mise.toml` together, not made silently. (1132 × 1.2 growth × 1.43 ≈ 1943s: ample margin expected.)

- [ ] **Step 3: Run the campaign**

```bash
cd <checkout> && mise run mutants
```

Hours; foreground. Confirm `CARGO_MUTANTS_JOBS` is unset first. This slice ADDS engine code (unlike 7d), so the mutant population **grows** — new mutants in 3.1.79, 6.1.77, the 7.3.86 arm, `vikarana_u_asamyogapurva`, `vrddhi_of`'s f arm, and the controller's collapse. If a background-shell limit threatens (~60 min sessions), run in chunks with `--iterate` as the standing memory note describes — but prefer one foreground run.

- [ ] **Step 4: Check BOTH `missed.txt` and `timeout.txt`**

Expected: `missed.txt` empty; `timeout.txt` exactly the one known-permanent ṇatva-scan entry (`j /= 1` in `tripadi.rs` — identify by diff shape, not line number). Any other timeout: re-run alone at the same cap before concluding anything. Any survivor: resolve, don't accept — the new-code mutants are exactly where a weak guard test would show up.

- [ ] **Step 5: Record in `AGENTS.md`**

Add a paragraph in the established series shape: 3420 cells, per-binary floor + total, prediction-vs-measurement, campaign tallies (mutants/caught/missed/unviable/timeouts), the duration distribution (the 7d plan's Task 6 Step 5 has the extraction script), both margins labelled measured/projected. Keep every earlier paragraph.

- [ ] **Step 6: Commit**

```bash
git add AGENTS.md
git commit -m "test: mutation gate at 3420 cells, floor re-measured"
```

---

### Task 10: documentation sweep, PR, finish 8a

**Files:**
- Modify: `README.md`, `docs/ARCHITECTURE.md`, `AGENTS.md`, `data/ATTRIBUTION.md`, `crates/panini-prakriya/src/tinanta/guna.rs` (two stale comments)

- [ ] **Step 1: `README.md`**

- "seven gaṇas fully covered" → **eight**, adding *tanādi* (8, vikaraṇa the bare *u* of 3.1.79) — worded as PARTIAL at nine of ten rows, √kṛ deferred to 8b by name.
- "curated 67-root set" → 76; cell/form/fork census → Task 7's measured numbers (the "351 of the 2844 cells" sentence and every bucket).
- The six-form-cell enumeration gains the measured new members (probe expects the loṭ prathama-eka AND madhyama-eka of kziR/fR/tfR/GfR); re-check "nothing forks deeper than six" against the measured buckets before repeating it.
- Both-padas roots: twelve → **nineteen** (the seven tanādi svarita rows join the eleven 1.3.72 roots; the enumeration sentence must now say eighteen by 1.3.72 + √bhuj by 1.3.66).
- The pada-ambiguous surface enumeration → whatever Task 7 Step 5 measured (the test walks `PARADIGM`, and the README mirrors the test).
- One sentence on van: ātmanepadī by its marker; vidyut's Kaumudī 2547.2 parasmaipada recorded, not modelled — the 1.3.72-sense precedent.

- [ ] **Step 2: `docs/ARCHITECTURE.md`**

The gaṇa inventory and counts (mirror Step 1's changes at this file's altitude); the vikaraṇa list gains the bare u and its ārdhadhātuka status; the vikalpa-rule inventory gains 7.3.86 (now EIGHT optional rules — this doc and any file that says "seven").

- [ ] **Step 3: `AGENTS.md`**

Suite-size figures (2844+494=3338 → the new triple), the recorded audit results section (add 8a's run), the gaṇa/root counts in the repo-tour prose. The mutation paragraph is already done (Task 9).

- [ ] **Step 4: `data/ATTRIBUTION.md`**

A tanādi bullet in the 7a/7b shape: the nine numbers with upadeśas; `08.0002` recorded as the second 6.1.64-stored row (after √ṣṭigh) with the R→n reversal stated; van's 2547.2 note; the gaṇa marked partial at 9/10 with √kṛ's named cost (6.4.108–110).

- [ ] **Step 5: the two stale engine comments**

- `guna.rs`, second 7.3.84's "complete inventory of SHAP texts reaching this point": add `u` (tanādi, 3.1.79) to the enumeration — it IS ik-final, which is the point; the sentence claiming only `nI` is ik-final must be rewritten.
- `guna.rs`, 6.1.78's vikaraṇa-arm comment ("which none of śap `a`, śyan `ya` … ever does"): the tanādi u guṇated to `o` now reaches it (tanavAni); the mutual-exclusivity argument still holds — say so with the new member listed.

- [ ] **Step 6: Verify prose against tests**

```bash
cd <checkout> && grep -rn "2844\|3338\|494\|67 root\|67-root\|seven gaṇas\|seven ganas\|twelve roots\|351 of" \
  README.md docs/ARCHITECTURE.md AGENTS.md data/ATTRIBUTION.md tools/audit/README.md | grep -v "recorded\|slice\|was\|prior"
```

Every remaining hit must be a deliberate historical reference. Also grep the seven — now eight — vikalpa rule ids across the docs (the standing sweep lesson: wrapped or rule-scoped counts dodge naive greps; check the files no task owns).

- [ ] **Step 7: Full verification**

```bash
cd <checkout> && mise run fmt-check && mise run lint && mise run test && mise run audit
```

All PASS, foreground.

- [ ] **Step 8: Push, PR, finish**

```bash
git push -u origin tanadi-gana
gh pr create --fill
```

PR body: the audit verdict (vidyut commit, totals, control verified), the mutation result, and the slice's two structural findings (the first ārdhadhātuka vikaraṇa; convergent-fork collapse). Then the `superpowers:finishing-a-development-branch` skill: wait for CI, merge, verify commits on `main`, delete branch and worktree.

---

### Task 11: slice 8b — the √kṛ row

**Files:**
- Modify: `crates/panini-data/src/lib.rs`

- [ ] **Step 1: Branch from fresh `main`**

```bash
git checkout main && git pull && git checkout -b tanadi-gana-8b
```

- [ ] **Step 2: Extend the row test (failing)**

In `tanadi_rows_are_the_nine_curated_roots` — rename to `tanadi_rows_are_the_ten_curated_roots`, append `("08.0010", "kf", PadaAssignment::Ubhayapada)`, and rewrite the comment's PARTIAL paragraph: the gaṇa closes at 10/10 in this slice; √kṛ is ñit (`qukf\Y` — the `qu` is a ḍu-it, 1.3.5; the `\` accent grave sits on the root vowel; the final `Y` is what 1.3.72 reads).

```bash
mise exec -- cargo test -p panini-data tanadi_rows
```

Expected: FAIL.

- [ ] **Step 3: The row**

Append after `08.0009`:

```rust
    Dhatu {
        // 08.0010 qukf\Y karaRe. √kṛ — ñit (the final Y; the qu is a
        // ḍu-it, 1.3.5) → 1.3.72 → ubhayapadī: karoti / kurute. Rides the
        // same 3.1.79 (the sūtra's own *kṛñbhya*), with the three
        // root-keyed specials in guna.rs: 6.4.110 ata ut (kurutaH),
        // 6.4.108 nityaṁ karoteḥ (kurmaH — the lopa 6.4.107 makes
        // optional is nitya here, so NO alternates), 6.4.109 ye ca
        // (kuryAt). 8.2.77's ur-lengthening is absent from this engine,
        // so 8.2.79 na BakurCurAm — which vidyut records blocking it on
        // every kur cell — has nothing to block and is likewise absent.
        dhatupatha: "08.0010",
        code: "kf",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "karaRe",
    },
```

- [ ] **Step 4: Count, suite, red window**

`dhatus().len()` → **77**. `mise exec -- cargo test -p panini-data` all PASS (the three load-bearing tests unedited — kf must it-strip clean and derive Ubhayapada from its Y). Then confirm `paradigm_covers_every_enumerable_cell` fails with exactly **8** new triples.

- [ ] **Step 5: Commit**

```bash
git add crates/panini-data/src/lib.rs
git commit -m "feat(data): 08.0010 qukf\\Y — the tanadi gana's tenth row"
```

---

### Task 12: 6.4.110, 6.4.108, 6.4.109

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (three rules after 7.3.84's second application, before 6.4.87)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (order pin)

**Interfaces:**
- Consumes: the u vikaraṇa, `Tag::Tanadi`, first 7.3.84 (which produces `kar` — the root's final f guṇates against the non-ṅit ārdhadhātuka u, no new rule needed).
- Produces: rules `6.4.110`, `6.4.108`, `6.4.109`, in that order. Task 14 relies on: `kurvaH`/`kurmaH` never fork on 6.4.107 (6.4.108 empties SHAP first, and 6.4.107's helper declines on the empty text — self-guarding, no new guard on 6.4.107).

- [ ] **Step 1: Failing unit tests**

In `guna.rs`'s tests:

```rust
    fn kr_prakriya(ending: &str, ngit: bool) -> Prakriya {
        // Post-first-7.3.84 shape: kar + u + ending.
        let mut p = Prakriya {
            terms: vec![Term::new("kar"), Term::new("u"), Term::new(ending)],
            ..Default::default()
        };
        p.terms[0].add(Tag::Dhatu);
        p.terms[0].add(Tag::Tanadi);
        p.terms[1].add(Tag::Vikarana);
        p.terms[1].add(Tag::Ardhadhatuka);
        if ngit {
            p.terms[2].add(Tag::Ngit);
        }
        p
    }

    #[test]
    fn ata_ut_fires_only_before_ngit_sarvadhatuka() {
        let r = rules().find(|r| r.id == "6.4.110").unwrap();
        let mut p = kr_prakriya("tas", true);
        assert!((r.apply)(&mut p));
        assert_eq!(p.terms[0].text, "kur");
        // karoti's pit ti: no ut.
        let mut p = kr_prakriya("ti", false);
        assert!(!(r.apply)(&mut p));
        // Another tanādi root's a is not karoti's: tan stays tan.
        let mut p = kr_prakriya("tas", true);
        p.terms[0].text = "tan".into();
        assert!(!(r.apply)(&mut p));
    }

    #[test]
    fn nityam_karoteh_empties_the_u_before_m_and_v() {
        let r = rules().find(|r| r.id == "6.4.108").unwrap();
        for ending in ["mas", "vas"] {
            let mut p = kr_prakriya(ending, true);
            p.terms[0].text = "kur".into();
            assert!((r.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[1].text, "", "{ending}");
        }
        // Not before tas — that u survives (kurutaH).
        let mut p = kr_prakriya("tas", true);
        p.terms[0].text = "kur".into();
        assert!(!(r.apply)(&mut p));
    }

    #[test]
    fn ye_ca_empties_the_u_before_y() {
        let r = rules().find(|r| r.id == "6.4.109").unwrap();
        let mut p = kr_prakriya("yAt", true);
        p.terms[0].text = "kur".into();
        assert!((r.apply)(&mut p));
        assert_eq!(p.terms[1].text, "");
    }
```

- [ ] **Step 2: Run to verify they fail**

```bash
mise exec -- cargo test -p panini-prakriya -- ata_ut nityam_karoteh ye_ca
```

Expected: FAIL (no such rules).

- [ ] **Step 3: The three rules**

Insert in `guna.rs` after the second 7.3.84 block, before 6.4.87 (a block comment first):

```rust
    // ------------------------------------------------------------------
    // The √kṛ specials, 6.4.108–110. They live HERE, not with their
    // 6.4.10x siblings in adesha.rs, because 6.4.110 must precede 6.1.77
    // below: kar + u + anti must become kur + u + anti before the u goes
    // to v (kurvanti), and stage files are an organisational boundary,
    // not a grammatical one — the flattened order is the grammar (the
    // same argument that put 7.3.92 in this file). All three are keyed to
    // √kṛ by 6.4.108's *karoteḥ*, carried by anuvṛtti into 109 and 110;
    // the guards read the post-guṇa root text (kar / kur) plus the gaṇa
    // tag rather than a Dhatu identity the pipeline does not carry.
    // vidyut-prakriya additionally records 8.2.79 na BakurCurAm blocking
    // ur-lengthening on every kur cell; this engine implements neither
    // 8.2.77 (the lengthening) nor 8.2.79 (its block), and the forms are
    // identical — recorded here so the absence reads as a decision.
    // ------------------------------------------------------------------
    // 6.4.110 ata ut sārvadhātuke (kṅiti, anuvṛtti from 6.4.98/6.4.108's
    // context): kar's `a` becomes `u` before a ṅit sārvadhātuka —
    // kurutaH, kurvanti, kurute, and (via 6.4.106 next) kuru. Before pit
    // endings it declines and 7.3.84's guṇa run stands: karoti, karavAni.
    Rule {
        id: "6.4.110",
        name: "ata ut sArvaDAtuke",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Tanadi) || p.terms[ANGA].text != "kar" {
                return false;
            }
            if p.terms.get(SHAP).map(|t| t.text.as_str()) != Some("u") {
                return false;
            }
            if !p.terms[ENDING].has(Tag::Ngit) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = "kur".into();
            p.record("6.4.110", "ata ut sArvaDAtuke", before);
            true
        },
    },
    // 6.4.108 nityaṁ karoteḥ: the lopa 6.4.107 makes optional is NITYA
    // for √kṛ before m/v — kurvaH, kurmaH, with no alternate. Ordered
    // before 6.4.107 (adesha.rs) by stage order; once this empties the
    // u, 6.4.107's helper declines on the empty text, so the vikalpa
    // machinery never sees √kṛ — the self-guarding 6.4.87/6.4.77 use.
    Rule {
        id: "6.4.108",
        name: "nityaM karoteH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Tanadi) || p.terms[ANGA].text != "kur" {
                return false;
            }
            if p.terms.get(SHAP).map(|t| t.text.as_str()) != Some("u") {
                return false;
            }
            if !p.terms[ENDING].text.starts_with(['m', 'v']) {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = String::new();
            p.record("6.4.108", "nityaM karoteH", before);
            true
        },
    },
    // 6.4.109 ye ca: the same lopa before y — kuryAt and the rest of
    // vidhiliṅ parasmaipada (the ending term reads `yAt`/`yAtAm`/… here:
    // anga.rs has already fused yAsuṭ into it).
    Rule {
        id: "6.4.109",
        name: "ye ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Tanadi) || p.terms[ANGA].text != "kur" {
                return false;
            }
            if p.terms.get(SHAP).map(|t| t.text.as_str()) != Some("u") {
                return false;
            }
            if !p.terms[ENDING].text.starts_with('y') {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = String::new();
            p.record("6.4.109", "ye ca", before);
            true
        },
    },
```

Pin `"6.4.110"`, `"6.4.108"`, `"6.4.109"` in the order test between the second `"7.3.84"` and `"6.4.87"`, with a comment on why they precede 6.1.77.

- [ ] **Step 4: Engine tests + full suite, foreground**

```bash
mise exec -- cargo test -p panini-prakriya && mise run test
```

Expected: engine tests PASS; the full suite PASSES **except** the Task 11 red window (`paradigm_covers_every_enumerable_cell`, 8 triples) — all 3420 existing cells byte-identical (the guards are kar/kur + Tanadi, inert for every other root).

- [ ] **Step 5: Commit**

```bash
git add -A && git commit -m "feat(engine): the kr specials — 6.4.110 ata ut, 6.4.108 nityam karoteh, 6.4.109 ye ca

In guna.rs before 6.1.77 (kurvanti needs kur before u->v). 6.4.108
empties the u so 6.4.107 self-declines: kurmaH has no alternate."
```

---

### Task 13: the 8b audit

Repeat Task 6 exactly, with: totals **77 roots / 3492 cells / `<N'>` forms** (probe projects ≈ 6 new alternates: the loṭ tātaṅ pairs, laṅ's 8.4.56 row, vidhiliṅ's `kuryAd/kuryAt`); dev-deps repointed at THIS branch's checkout; `tee /tmp/8b-audit.txt`; README entry named `tanadi 8b`. The negative control runs first, as always. **Expected sharp edges if a diff appears:** √kṛ loṭ uttama (`karavARi` — the ṇatva scan must retroflex the ending's n after kar's r; if the engine misses it, that is a tripadi guard finding), and any `kuruvaH`-shaped form (6.4.108 ordering defect). Commit as in Task 6.

---

### Task 14: 8b goldens and trace pins

**Files:** as Task 7 (generator named `print_8b_goldens.rs`, `NEW_ROOTS = ["08.0010"]`) plus `trace/tanadi.rs`.

- [ ] **Step 1: Generate, sanity-check, paste**

Task 7's procedure. Sanity strings from the probe: laṭ P.E `karoti` / ātmanepada `kurute`; laṭ P.D `kurutaH`; U.B `kurmaH` — and **U.D `kurvaH` / U.B `kurmaH` must print NO alternates** (6.4.108 nitya; a `kuruvaH` row is an ordering defect, stop). loṭ M.E is the three-form `kuru/kurutAd/kurutAt`; vidhiliṅ P.E `kuryAd/kuryAt`. Append the rows to `data/tanadi.rs` (same file, the gaṇa's other slice), update the shape test (388 blocks / 3492 cells, buckets, keys — cross-check `3492 + ALTERNATES.len() == <N'>`), delete the generator, re-run the ambiguity test (kurutAm-shape collisions likely; re-derive if moved).

- [ ] **Step 2: The √kṛ trace pins**

Append to `trace/tanadi.rs`:

```rust
#[test]
fn kurutah_trace_orders_guna_before_ata_ut() {
    // kf laT P.D: 7.3.84 makes kar against the ārdhadhātuka u, then
    // 6.4.110 makes kur against the ṅit tas. Reversed, 6.4.110's `kar`
    // guard never matches and kurutaH is underivable.
    let (text, t) = cell_trace("08.0010", Lakara::Lat, Pada::Parasmaipada,
        Purusha::Prathama, Vacana::Dvi);
    assert_eq!(text, "kurutaH", "got {t:?}");
    assert!(at(&t, "7.3.84") < at(&t, "6.4.110"), "got {t:?}");
}

#[test]
fn kurmah_is_nitya_no_6_4_107_fork() {
    // kf laT U.B: 6.4.108 does what 6.4.107 would only offer, so the
    // cell must not fork at all — one live branch, 6.4.108 in its log,
    // 6.4.107 in no branch's.
    let d = dhatus().iter().find(|d| d.dhatupatha == "08.0010").unwrap();
    let branches: Vec<_> = derive(d, Lakara::Lat, Pada::Parasmaipada,
        Purusha::Uttama, Vacana::Bahu)
        .into_iter().filter(|p| !p.blocked).collect();
    assert_eq!(branches.len(), 1, "6.4.108 is nitya");
    assert_eq!(branches[0].text(), "kurmaH");
    assert!(branches[0].log.iter().any(|s| s.sutra == "6.4.108"));
    assert!(!branches[0].log.iter().any(|s| s.sutra == "6.4.107"));
}

#[test]
fn kuryat_trace_takes_ye_ca() {
    // kf viDiliN P.E, declined branch: 6.4.110 then 6.4.109.
    let (text, t) = cell_trace("08.0010", Lakara::VidhiLin, Pada::Parasmaipada,
        Purusha::Prathama, Vacana::Eka);
    assert_eq!(text, "kuryAd", "got {t:?}");
    assert!(at(&t, "6.4.110") < at(&t, "6.4.109"), "got {t:?}");
}

#[test]
fn karoti_runs_7_3_84_twice_and_the_specials_not_at_all() {
    // kf laT P.E: root guṇa against the u, vikaraṇa guṇa against pit ti
    // — the double application the pipeline's two 7.3.84 entries exist
    // for — and every 6.4.10x special declines on the pit ending.
    let (text, t) = cell_trace("08.0010", Lakara::Lat, Pada::Parasmaipada,
        Purusha::Prathama, Vacana::Eka);
    assert_eq!(text, "karoti", "got {t:?}");
    assert_eq!(t.iter().filter(|s| *s == "7.3.84").count(), 2, "got {t:?}");
    for absent in ["6.4.110", "6.4.108", "6.4.109"] {
        assert!(!t.contains(&absent.to_string()), "{absent} in {t:?}");
    }
}
```

(`kuryAd` not `kuryAt`: the declined branch keeps 8.2.39's `d`, with `kuryAt` in `ALTERNATES` under `8.4.56` — the `aBavad` convention. Verify against the pasted goldens; the golden wins.)

- [ ] **Step 3: Full suite, foreground; commit**

```bash
mise run test
git add crates/panini/tests/ && git commit -m "test: kr's audited paradigms and trace pins — the gana closes at 10/10"
```

---

### Task 15: 8b mutation gate, doc sweep, PR

- [ ] **Step 1:** Mutation gate per Task 9 (floor at 3492 cells; new mutants in the three √kṛ rules; record in `AGENTS.md`).
- [ ] **Step 2:** Doc sweep per Task 10, 8b's deltas: the gaṇa **closes at ten of ten** (README's "eight gaṇas" wording loses its partial qualifier; every 9-root/76-root/3420-cell figure moves; both-padas roots → **20**); `ATTRIBUTION.md`'s tanādi bullet gains 08.0010 and drops the partial framing; `adesha.rs`'s 6.4.107 comment paragraph saying "√kṛ is out of scope … 6.4.108 is not implemented" is now FALSE — rewrite it to point at the guna.rs specials; the audit README records 8b's run.
- [ ] **Step 3:** `mise run fmt-check && mise run lint && mise run test && mise run audit` — all PASS, foreground.
- [ ] **Step 4:** Push `tanadi-gana-8b`, PR (audit verdict, mutation result, "the gaṇa closes at 10/10; kurmaH's missing alternate is 6.4.108 working"), then finishing-a-development-branch.

---

## Deferred, and why

- **Kaumudī 2547.2's parasmaipada for van** (`vanoti`): recorded on the row and in README; modelling gaṇasūtra-optional it-markers is a design conversation (it breaks `curated_pada_agrees_with_upadesha_markers`'s premise), not a row edit.
- **8.2.77 *hali ca* / 8.2.79 *na bhakurchurām***: neither implemented; the pair is inseparable (77 without 79 derives \*kUryAt) and no covered form needs either. Recorded in the √kṛ block comment.
- **6.1.77's non-vikaraṇa arms** (root-final ik before vowel, iyaṅ, etc.): no covered hiatus reaches them; widen by arm with a witness.
- **juhotyādi (3) and curādi (10)** — the last two gaṇas, each its own spec. Juhotyādi's reduplication also inherits the deferred √dā/√dhā/√hu notes from kryādi/svādi.
- **The `vrddhi_of` `x` arm** (`Al` by 1.1.51): no ḷ-vowel root in scope anywhere; the f arm's comment names it as the next one.
