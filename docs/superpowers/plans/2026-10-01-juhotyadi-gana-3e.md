# Juhotyādi gaṇa slice 3e Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate √nij, √vij and √viṣ (`03.0012`–`03.0014`, *nenekti* / *vevekti* / *vevezwi*), all three ubhayapadī. This takes the golden suite from 4212 to 4428 cells and juhotyādi to 20 of its 26 rows. It adds two sūtras: 7.4.75 *nijāṁ trayāṇāṁ guṇaḥ ślau* and 7.3.87 *nābhyastasyāci piti sārvadhātuke*.

**Architecture:** Seven tasks.
- **Tasks 1 and 2** are the whole engine change: 7.4.75 in the abhyāsa stage (`abhyasa.rs`), then 7.3.87 in `guna.rs`. 7.3.87 is a mandatory rule that changes no text and bars 7.3.86 through `Rule.bars`. Neither rule fires on a curated root yet, so both are gated on **guard tests plus the 4212 priors staying byte-identical**.
- **Task 3** lands the three rows and their goldens, turning 216 cells green.
- **Task 4** adds the trace pins.
- **Tasks 5–7** are the audit and doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.98.1 pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants | audit`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`.

**Spec:** `docs/superpowers/specs/2026-10-01-juhotyadi-gana-3e-design.md`

**Workspace:** the branch `juhotyadi-3e` already exists. It holds the spec and this plan and is checked out at `/workspace/.worktrees/juhotyadi-3e`. Every path below is relative to that directory.

## Global Constraints

- **The 4212 pre-existing cells must stay byte-identical, traces included.** Regenerate no golden and change no pinned trace.
- **No unfalsifiable guard clauses.** A clause no cell and no guard test can make false is a mutation survivor. Witness it, delete it, or kill it with a direct guard test on a hand-built `Prakriya`.
- **Rules that name roots key on `p.ctx.dhatupatha`**, with a comment naming the upadeśa (7.4.75). The text `vij` is also `06.0009` (tudādi) and `07.0023` (rudhādi).
- **Engine order differs from vidyut's on purpose.** This engine copies the bare root before guṇa, so 7.4.59 never fires on these rows. 7.3.87 sits in `guna`, after the whole abhyāsa stage, while vidyut credits it between 6.1.10 and 7.4.60. Pins hold THIS order. Never "fix" a pin toward vidyut's trace.
- **Goldens are transcribed from this plan** (vidyut's output at `8da2f90b`, generated 2026-10-01 by `/tmp/vidyut-full/vidyut-prakriya/examples/juhotyadi_3e_probe.rs`). Never invent one. **Do not edit a golden to match the engine.**
- **No new vikalpa.** The engine stays at eleven optional rules, and `exactly_the_pinned_vikalpa_rules_are_optional` must not change.
- **6.1.65 *ṇo naḥ* is not written.** `Ri\ji~^r` is stored as `nij` under the existing stored-form convention.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit.
- `mise run test` takes a few seconds. Run it in the **foreground** with a timeout of 600000 ms. Never background it and end a turn.
- `mise run test -- -p X` does not scope. Scope unit tests with `mise exec -- cargo test -p <crate> <filter>`.
- Copy the new rules' ids and SLP1 names verbatim from vidyut's `sutrapatha.tsv`: `7.4.75 nijAM trayARAM guRaH SlO`, `7.3.87 nAByastasyAci piti sArvaDAtuke`.

## Review Focus

These are inputs the spec implies that no golden cell exercises. Each has its test in the owning task.

1. **A vowel-final abhyasta aṅga before a vowel-initial pit ending** (√hu's *juhavAni*, *ajuhavam*). 7.3.87 must not fire, because *laghūpadhasya* continues from 7.3.86. → Task 2, the `hu` case of `nabhyastasyaci_piti_declines_off_each_condition`; Task 4, `juhavani_trace_has_no_nabhyastasyaci`.
2. **The other two `vij` rows.** Tudādi `06.0009` and rudhādi `07.0023` share the text `vij`, and 7.4.75 must decline on them. → Task 1, `nijam_trayanam_declines_for_the_other_vij_rows_despite_identical_text`.
3. **Each half of `loṭ && uttama` on its own.** Laṭ ātmanepada uttama `e` (*nenije*) and loṭ prathama `atu` (*nenijatu*) are vowel-initial, and 7.3.87 must not fire on them. → Task 2, same decline test.
4. **3f's a-upadhā shape** (`Bas` + `Ani`). 7.3.87 fires as a credited no-op, as vidyut credits it. → Task 2, `nabhyastasyaci_piti_blocks_upadha_guna_before_am_and_the_lot_uttama_endings`.
5. **`check()` on the new forms.** *nenekti*, *nenijAni* and *vevezwi* must resolve to the 3e rows only, never to the other `vij` rows, and the six new pada-ambiguous surfaces must report both padas. → Task 3, `nij_roots_analyse_their_reduplicated_forms`.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-prakriya/src/tinanta/abhyasa.rs` | Task 1: 7.4.75; its unit tests; the module doc |
| `crates/panini-prakriya/src/tinanta/guna.rs` | Task 2: 7.3.87; its unit tests |
| `crates/panini-prakriya/src/rule.rs` | Task 2: `Rule.bars` doc names its second user |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Task 1 and Task 2: `tinanta_rule_order_is_pinned`. Task 2: `exactly_the_pinned_bars` |
| `crates/panini-data/src/lib.rs` | Task 3: the three rows, counts, the gaṇa-row test |
| `crates/panini/tests/paradigm/data/juhotyadi.rs` | Task 3: 24 `PARADIGM` blocks, 21 `ALTERNATES` rows |
| `crates/panini/tests/paradigm/main.rs` | Task 3: totals, buckets, key census, the ambiguity set, the `check()` test. Task 5: audit prose |
| `crates/panini/tests/trace/juhotyadi.rs` | Task 4: eight pins |
| `crates/panini-prakriya/src/tinanta/anga.rs` | Task 5: one stale "3e" comment |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, the 3a spec | Task 5 |
| `AGENTS.md`, maybe `mise.toml` | Task 6 |

---

## Task 1: 7.4.75 *nijāṁ trayāṇāṁ guṇaḥ ślau*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/abhyasa.rs`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: the existing test helpers in `abhyasa.rs`'s `mod tests`: `slu_prakriya(root: &str, ending: &str) -> Prakriya` and `run_abhyasa_stage(root: &str, number: &'static str) -> Prakriya`. Also `crate::tinanta::sound::guna_of(char) -> Option<&'static str>`.
- Produces: a `Rule` with `id: "7.4.75"`, `name: "nijAM trayARAM guRaH SlO"` in `ABHYASA_RULES`, between 7.4.62 and 7.4.76. Task 4 asserts its position in traces.

- [ ] **Step 1: Write the failing tests**

In `abhyasa.rs`'s `mod tests`, immediately before `bhrnam_it_makes_the_abhyasa_vowel_i_for_the_three_rows_it_names`, add:

```rust
    #[test]
    fn nijam_trayanam_gunates_the_abhyasa_of_its_three_rows() {
        // 7.4.75. √ṇij (03.0012), √vij (03.0013) and √viṣ (03.0014), after
        // 7.4.60: ni → ne (nenekti), vi → ve (vevekti, vevezwi).
        let rule = rules().find(|r| r.id == "7.4.75").unwrap();
        for (root, number, abhyasa, want) in [
            ("nij", "03.0012", "ni", "ne"),
            ("vij", "03.0013", "vi", "ve"),
            ("viz", "03.0014", "vi", "ve"),
        ] {
            let mut p = slu_prakriya(root, "ti");
            p.ctx.dhatupatha = number;
            p.terms[ABHYASA].text = abhyasa.into();
            assert!((rule.apply)(&mut p), "{number}");
            assert_eq!(p.terms[ABHYASA].text, want, "{number}");
            assert_eq!(p.terms[ANGA].text, root, "{number}: the aṅga is untouched");
            assert_eq!(p.log.last().unwrap().sutra, "7.4.75");
        }
    }

    #[test]
    fn nijam_trayanam_declines_for_the_other_vij_rows_despite_identical_text() {
        // Tudādi's 06.0009 and rudhādi's 07.0023 are also `vij`. Neither
        // reduplicates, so only a hand-built prakriyā can put an abhyāsa in
        // front of them. The number decides, not the text. "" is a hand-built
        // prakriyā with no row.
        let rule = rules().find(|r| r.id == "7.4.75").unwrap();
        for number in ["06.0009", "07.0023", ""] {
            let mut p = slu_prakriya("vij", "ti");
            p.ctx.dhatupatha = number;
            p.terms[ABHYASA].text = "vi".into();
            assert!(!(rule.apply)(&mut p), "{number:?}");
            assert_eq!(p.terms[ABHYASA].text, "vi", "{number:?}");
            assert!(p.log.is_empty(), "{number:?}");
        }
    }
```

At the end of `mod tests`, after `the_r_roots_reach_their_abhyasa_through_ur_at_then_haladih_shesha`, add:

```rust
    #[test]
    fn the_nij_roots_reach_their_abhyasa_through_haladih_shesha_then_nijam_trayanam() {
        // Slice 3e's three rows through the whole stage. 7.4.60 drops the
        // copy's final consonant (nij → ni). 7.4.59 has nothing to shorten,
        // because this stage copies the bare root, not vidyut's guṇated `nej`.
        // 7.4.75 then guṇates the vowel.
        for (root, number, want) in [
            ("nij", "03.0012", "ne"),
            ("vij", "03.0013", "ve"),
            ("viz", "03.0014", "ve"),
        ] {
            let p = run_abhyasa_stage(root, number);
            assert_eq!(p.terms[ABHYASA].text, want, "{number}");
            assert_eq!(p.terms[ANGA].text, root, "{number}");
            let got: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(got, vec!["6.1.10", "7.4.60", "7.4.75"], "{number}");
        }
    }
```

- [ ] **Step 2: Pin the new rule order**

In `derivation_tests.rs`'s `tinanta_rule_order_is_pinned`, change `"7.4.62", "7.4.76",` to `"7.4.62", "7.4.75", "7.4.76",`. `mise run fmt` re-wraps the array.

- [ ] **Step 3: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -30`
Expected FAIL:
- both `nijam_trayanam_*` tests (panic at `rules().find(...).unwrap()`: no 7.4.75)
- `the_nij_roots_reach_their_abhyasa_through_haladih_shesha_then_nijam_trayanam` (abhyāsa `ni`, not `ne`)
- `tinanta_rule_order_is_pinned`

- [ ] **Step 4: Add 7.4.75**

Change the `sound` import to:

```rust
use crate::tinanta::sound::{cutva_of, guna_of, hrasva_of, is_vowel};
```

Insert this rule into `ABHYASA_RULES` immediately before the `// 7.4.76 bhṛñām it:` comment, after 7.4.62's closing `},`:

```rust
    // 7.4.75 nijāṁ trayāṇāṁ guṇaḥ ślau: under ślu, the abhyāsa of the three
    // roots beginning with √ṇij takes guṇa. After 7.4.60: ni → ne (nenekti),
    // vi → ve (vevekti, vevezwi). vidyut guṇates first, copies `nej`, trims
    // it to `ne` by 7.4.60 and shortens it back to `ni` by 7.4.59 before this
    // rule. This stage copies the bare root, so it skips that round trip.
    //
    // KEYED BY ROW NUMBER, like 7.4.76 below it. The sūtra names three roots,
    // and the text `vij` is also tudādi's 06.0009 and rudhādi's 07.0023,
    // neither of which reduplicates.
    //   03.0012 Ri\ji~^r √ṇij (stored `nij`: 6.1.65 by the stored-form
    //                          convention)
    //   03.0013 vi\ji~^r √vij
    //   03.0014 vi\zx~^  √viṣ
    //
    // Every abhyāsa this reaches is a single ekāc (6.1.10's NARROW note), so
    // guṇating each vowel is guṇating THE vowel.
    Rule {
        id: "7.4.75",
        name: "nijAM trayARAM guRaH SlO",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !matches!(p.ctx.dhatupatha, "03.0012" | "03.0013" | "03.0014") {
                return false;
            }
            let t: String = p.terms[ABHYASA]
                .text
                .chars()
                .map(|c| guna_of(c).map_or_else(|| c.to_string(), str::to_string))
                .collect();
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("7.4.75", "nijAM trayARAM guRaH SlO", before);
            true
        },
    },
```

Update the module doc's first two lines to:

```rust
//! Reduplication: 6.1.10, 7.4.66, 7.4.60, 7.4.59, 7.4.62, 7.4.75, 7.4.76,
//! 7.4.77, 7.4.78, 6.4.78 — dvitva and the rules that reshape the abhyāsa.
```

- [ ] **Step 5: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -30`
Expected: PASS.

- [ ] **Step 6: Run the full suite (priors byte-identical)**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms)
Expected: PASS at 4212 cells, with no golden or trace changed. No 3e row is curated yet, so nothing in the corpus reaches 7.4.75.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/abhyasa.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(abhyasa): 7.4.75 nijāṁ trayāṇāṁ guṇaḥ ślau, keyed to 03.0012–03.0014"
```

---

## Task 2: 7.3.87 *nābhyastasyāci piti sārvadhātuke*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`, `exactly_the_pinned_bars`)
- Modify: `crates/panini-prakriya/src/rule.rs` (the `bars` field's doc comment)

**Interfaces:**
- Consumes: `guna.rs`'s test helper `abhyasta_prakriya(abhyasa: &str, anga: &str, ghu: bool, ending: &str, ngit: bool) -> Prakriya`. It builds a ślu'd prakriyā with `ANGA` tagged Abhyasta, an empty śap, and `ending` at `ENDING`. It sets **no** Sārvadhātuka or Pit tag.
- Produces: a `Rule` with `id: "7.3.87"`, `name: "nAByastasyAci piti sArvaDAtuke"`, `vikalpa: false`, `bars: &["7.3.86"]`, in `GUNA` between the first 7.3.84 and the first 7.3.86. Task 4 asserts its presence or absence in traces.

- [ ] **Step 1: Write the failing tests**

In `guna.rs`'s `mod tests`, change `use panini_data::{Purusha, Vacana};` to `use panini_data::{Lakara, Purusha, Vacana};`. If the compiler reports `Lakara` as already imported through `use super::*;`, revert this line.

Immediately after the `jaha_prakriya` helper, add:

```rust
    /// √ṇij at the guṇa stage: abhyāsa `ne` (7.4.75 has run), aṅga `nij`
    /// tagged Abhyasta, an empty śap, and `ending` tagged Sārvadhātuka (and
    /// Pit when `pit`) in the given lakāra and puruṣa.
    fn nij_prakriya(ending: &str, pit: bool, lakara: Lakara, purusha: Purusha) -> Prakriya {
        let mut p = abhyasta_prakriya("ne", "nij", false, ending, false);
        p.terms[ENDING].add(Tag::Sarvadhatuka);
        if pit {
            p.terms[ENDING].add(Tag::Pit);
        }
        p.ctx.lakara = lakara;
        p.ctx.purusha = purusha;
        p
    }

    #[test]
    fn nabhyastasyaci_piti_blocks_upadha_guna_before_am_and_the_lot_uttama_endings() {
        // 7.3.87 changes no text. It fires, and the controller bars 7.3.86 on
        // its branch: anenijam, nenijAni, nenijE. Laṅ's `am` keeps mip's Pit
        // tag. Loṭ's `Ani`/`AE` carry none: 3.4.92 makes them pit, and 1.2.4
        // reads that off the lakāra and puruṣa, which is how this guard reads
        // it too.
        let rule = rules().find(|r| r.id == "7.3.87").unwrap();
        for (ending, pit, lakara) in [
            ("am", true, Lakara::Lan),
            ("Ani", false, Lakara::Lot),
            ("AE", false, Lakara::Lot),
        ] {
            let mut p = nij_prakriya(ending, pit, lakara, Purusha::Uttama);
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ANGA].text, "nij", "{ending}: no text changes");
            assert_eq!(p.terms[ENDING].text, ending, "{ending}");
            assert_eq!(p.log.last().unwrap().sutra, "7.3.87");
        }
        // 3f's a-upadhā shape (√bhas, 03.0019). An `a` upadhā is laghu too, so
        // the block is credited here, where 7.3.86 has nothing to guṇate.
        let mut p = abhyasta_prakriya("ba", "Bas", false, "Ani", false);
        p.terms[ENDING].add(Tag::Sarvadhatuka);
        p.ctx.lakara = Lakara::Lot;
        p.ctx.purusha = Purusha::Uttama;
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Bas");
    }

    #[test]
    fn nabhyastasyaci_piti_declines_off_each_condition() {
        let rule = rules().find(|r| r.id == "7.3.87").unwrap();
        let declines = |mut p: Prakriya, why: &str| {
            assert!(!(rule.apply)(&mut p), "{why}");
            assert!(p.log.is_empty(), "{why}");
        };
        // *aci*: `mi` is pit but consonant-initial, so nenejmi keeps its guṇa.
        declines(nij_prakriya("mi", true, Lakara::Lat, Purusha::Uttama), "mi");
        // *piti*: a ṅit vowel-initial ending (nenijati), then each half of
        // `loṭ && uttama` on its own: laṭ ātmanepada uttama `e` (nenije) and
        // loṭ prathama `atu` (nenijatu).
        declines(nij_prakriya("ati", false, Lakara::Lat, Purusha::Prathama), "ati");
        declines(nij_prakriya("e", false, Lakara::Lat, Purusha::Uttama), "e");
        declines(nij_prakriya("atu", false, Lakara::Lot, Purusha::Prathama), "atu");
        // *sārvadhātuke*: the same `Ani` without the tag.
        let mut p = nij_prakriya("Ani", false, Lakara::Lot, Purusha::Uttama);
        p.terms[ENDING].remove(Tag::Sarvadhatuka);
        declines(p, "ārdhadhātuka Ani");
        // *abhyastasya*: a laghūpadha aṅga that is not abhyasta.
        let mut p = nij_prakriya("Ani", false, Lakara::Lot, Purusha::Uttama);
        p.terms[ANGA].remove(Tag::Abhyasta);
        declines(p, "not abhyasta");
        // *laghūpadhasya*, continued from 7.3.86. √hu's juhavAni keeps its 7.3.84
        // guṇa: a vowel-final aṅga has no upadhā guṇa to block. A guru upadhā
        // (`nIj`, synthetic) is not 7.3.86's either. A one-letter aṅga has no
        // upadhā at all.
        let mut p = abhyasta_prakriya("Ju", "hu", false, "Ani", false);
        p.terms[ENDING].add(Tag::Sarvadhatuka);
        p.ctx.lakara = Lakara::Lot;
        p.ctx.purusha = Purusha::Uttama;
        declines(p, "hu");
        for anga in ["nIj", "j"] {
            let mut p = nij_prakriya("Ani", false, Lakara::Lot, Purusha::Uttama);
            p.terms[ANGA].text = anga.into();
            declines(p, anga);
        }
    }
```

- [ ] **Step 2: Pin the order and the bar**

In `derivation_tests.rs`:
- In `tinanta_rule_order_is_pinned`, change `"7.3.83", "7.3.84", "7.3.86", "7.3.86",` to `"7.3.83", "7.3.84", "7.3.87", "7.3.86", "7.3.86",`.
- In `exactly_the_pinned_bars`, change the `expected` line to:

```rust
    let expected: Vec<(&str, &[&str])> = vec![
        ("7.3.87", &["7.3.86"][..]),
        ("6.4.117", &["6.4.116", "6.4.113", "6.4.112"][..]),
    ];
```

- [ ] **Step 3: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -30`
Expected FAIL:
- both `nabhyastasyaci_piti_*` tests (panic at `rules().find(...).unwrap()`: no 7.3.87)
- `tinanta_rule_order_is_pinned`
- `exactly_the_pinned_bars`

- [ ] **Step 4: Add 7.3.87**

Change `guna.rs`'s top-level `use panini_data::Lakara;` to `use panini_data::{Lakara, Purusha};`.

Insert this rule into `GUNA` after the first 7.3.84's closing `},` and before the `// 7.3.86 pugantalaghūpadhasya ca: guṇa of a light` comment:

```rust
    // 7.3.87 nābhyastasyāci piti sārvadhātuke: an abhyasta aṅga takes no
    // laghūpadha guṇa before a vowel-initial pit sārvadhātuka ending. √ṇij
    // gives nenijAni, anenijam and nenijE, not *nenejAni (slice 3e).
    // *pugantalaghūpadhasya* continues from 7.3.86, so this is 7.3.86's
    // apavāda and nothing else's: √hu's juhavAni keeps its 7.3.84 guṇa.
    //
    // It changes no text. It bars 7.3.86 on its branch through `Rule.bars`,
    // so 7.3.86's guard never has to know about it. This is the second user
    // of `Rule.bars` after 6.4.117 and the first mandatory one. vidyut credits
    // the same no-op block. Barring is by id, so the branch also skips 7.3.86's
    // tanādi vikalpa entry below. That is inert, since no tanādi aṅga is
    // abhyasta.
    //
    // *piti* is read the way 1.2.4 reads it (`samjna.rs`): the ending's
    // Tag::Pit, OR loṭ uttama. Only tip/sip/mip are tagged pit, and laṅ's
    // `am` keeps mip's tag. The loṭ uttama endings are pit by 3.4.92 *āḍ
    // uttamasya pic ca*, but they are never tagged, because 1.2.4 excludes them
    // by lakāra and puruṣa instead. The tag alone would catch one of the
    // seven cells per root.
    //
    // *laghūpadhasya*: a consonant-final aṅga whose penultimate is a short
    // vowel. `a` is included because vidyut credits this rule on 3f's
    // a-upadhā √bhas, √dhan and √jan, where 7.3.86 has nothing to guṇate
    // anyway. 3f inherits the rule unchanged. It changes forms there on
    // 03.0021 (cikitAni), 03.0022 (tuturARi) and 03.0023 (diDizARi).
    //
    // Reads ENDING directly, as 7.3.92 does. An abhyasta aṅga is always
    // ślu'd, so SHAP is empty and the ending is the following sārvadhātuka.
    Rule {
        id: "7.3.87",
        name: "nAByastasyAci piti sArvaDAtuke",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &["7.3.86"],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Abhyasta) {
                return false;
            }
            let mut rev = p.terms[ANGA].text.chars().rev();
            let (Some(last), Some(upadha)) = (rev.next(), rev.next()) else {
                return false;
            };
            if is_vowel(last) || !matches!(upadha, 'a' | 'i' | 'u' | 'f' | 'x') {
                return false;
            }
            let ending = &p.terms[ENDING];
            if !ending.has(Tag::Sarvadhatuka) || !ending.text.chars().next().is_some_and(is_vowel) {
                return false;
            }
            let lot_uttama = matches!(p.ctx.lakara, Lakara::Lot)
                && matches!(p.ctx.purusha, Purusha::Uttama);
            if !ending.has(Tag::Pit) && !lot_uttama {
                return false;
            }
            let before = p.snapshot();
            p.record("7.3.87", "nAByastasyAci piti sArvaDAtuke", before);
            true
        },
    },
```

In `rule.rs`, in the `bars` field's doc comment, after the sentence ending `the three rules that would otherwise change that \`A\`.`, add:

```rust
    /// 7.3.87 *nābhyastasyāci piti sārvadhātuke* is the second and the first
    /// mandatory one: it changes no text and bars 7.3.86, keeping an abhyasta
    /// aṅga's laghu upadhā before a vowel-initial pit ending (nenijAni).
```

- [ ] **Step 5: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -30`
Expected: PASS.

- [ ] **Step 6: Run the full suite (priors byte-identical)**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms)
Expected: PASS at 4212 cells, with no golden or trace changed. Every curated abhyasta aṅga is vowel-final, so the *laghūpadhasya* clause keeps 7.3.87 off all of them. If any prior √hu, √ki, √bhī, √dā… loṭ uttama or laṅ uttama eka cell gains a 7.3.87 step or loses its guṇa, the laghūpadha clause is wrong. Stop and report.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/guna.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs crates/panini-prakriya/src/rule.rs
git commit -m "feat(guna): 7.3.87 nābhyastasyāci piti sārvadhātuke bars 7.3.86 on an abhyasta laghūpadha aṅga"
```

---

## Task 3: The three rows and their paradigm goldens

This task turns the 216 new cells green.

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini/tests/paradigm/data/juhotyadi.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`

**Interfaces:**
- Consumes: Tasks 1–2.
- Produces: `dhatus()` rows `03.0012` (`nij`), `03.0013` (`vij`) and `03.0014` (`viz`), each `Gana::Juhotyadi` and `PadaAssignment::Ubhayapada`. Task 4 looks them up by number.

- [ ] **Step 1: Add the `Dhatu` rows**

`DHATUS` is ordered by number. Insert after the `03.0011` row and before `03.0015`. Each artha is verbatim from `data/dhatupatha.tsv` lines 1276–1278:

```rust
    Dhatu {
        // 03.0012 `Ri\ji~^r` SOcapozaRayoH (√ṇijir). Ubhayapadī by 1.3.72 (the
        // svarita `~^`). Stored `nij`: the `R` → `n` of 6.1.65 *ṇo naḥ* is the
        // stored-form convention, as for √nī. 7.4.60 trims the copy to `ni`,
        // and 7.4.75, keyed by this number, guṇates it: nenekti. 7.3.87 keeps
        // the root's `i` before the vowel-initial pit endings: nenijAni.
        // Slice 3e.
        dhatupatha: "03.0012",
        code: "nij",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "SOcapozaRayoH",
    },
    Dhatu {
        // 03.0013 `vi\ji~^r` pfTagBAve (√vijir). Ubhayapadī by 1.3.72. The
        // same path as √ṇij: vevekti, vevijAni. Its text `vij` is shared with
        // tudādi's 06.0009 and rudhādi's 07.0023, which is why 7.4.75 keys on
        // the number. Slice 3e.
        dhatupatha: "03.0013",
        code: "vij",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "pfTagBAve",
    },
    Dhatu {
        // 03.0014 `vi\zx~^` vyAptO (√viṣḷ). Ubhayapadī by 1.3.72. The same
        // abhyāsa path, with the root's `z` reaching the existing tripādī
        // rules: 8.4.41 (vevezwi), 8.2.41 then 8.3.59 (vevekzi), 8.4.41 then
        // 8.4.53 (veviqQi), 8.2.39 (aveveq). Slice 3e.
        dhatupatha: "03.0014",
        code: "viz",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "vyAptO",
    },
```

Counts in the same file:
- `Dhatu`'s `pada` doc: `re-derives 93 of these 94` → `re-derives 96 of these 97`; `The test covers the 94 roots curated here` → `The test covers the 97 roots curated here`.
- `pada_from_upadesha`'s doc: `63 of the 94 curated roots carry a \`\\\` at all, and 42` → `66 of the 97 curated roots carry a \`\\\` at all, and 45`. Each new upadeśa has exactly one `\`, on the root vowel.
- `curated_roots_have_expected_ganas_and_padas`: `assert_eq!(dhatus().len(), 94);` → `97`.

- [ ] **Step 2: Extend the gaṇa-row test**

Rename `juhotyadi_rows_are_the_seventeen_curated_roots` to `juhotyadi_rows_are_the_twenty_curated_roots`. Update the other reference too: `grep -rn "juhotyadi_rows_are_the_seventeen" crates` finds it in a comment near line 1904.

In its comment, replace:

```rust
        // by 1.3.78, the gaṇa's one vowel-initial ṛ-root. The gaṇa is PARTIAL
        // at 17 of its 26 dhātupāṭha rows; slices 3e and 3f close it.
```

with:

```rust
        // by 1.3.78, the gaṇa's one vowel-initial ṛ-root. Slice 3e adds √ṇij,
        // √vij and √viṣ (03.0012–03.0014), ubhayapadī by 1.3.72, the three
        // roots 7.4.75 names. The gaṇa is PARTIAL at 20 of its 26 dhātupāṭha
        // rows; slice 3f closes it.
```

In its expected vector, insert after the `03.0011` line:

```rust
                ("03.0012", "nij", PadaAssignment::Ubhayapada),
                ("03.0013", "vij", PadaAssignment::Ubhayapada),
                ("03.0014", "viz", PadaAssignment::Ubhayapada),
```

Find the same test's root-keyed comment (it begins `// Every root-specific rule of slices 3c, 3c2, 3d and 3d2`). Change `slices 3c, 3c2, 3d and 3d2` → `slices 3c, 3c2, 3d, 3d2 and 3e`. Add 7.4.75 to its list of number-keyed rules in the same form the list uses. If the wording differs, keep its form and make the same additions.

- [ ] **Step 3: Run the data tests**

Run: `mise exec -- cargo test -p panini-data 2>&1 | tail -20`
Expected: PASS, including `curated_pada_agrees_with_upadesha_markers` and `dhatupatha_numbers_resolve_upstream`.

- [ ] **Step 4: Add the `PARADIGM` blocks**

**Append** to the end of `PARADIGM` in `crates/panini/tests/paradigm/data/juhotyadi.rs`, just before its closing `];` at about line 889. Blocks are ordered by slice, not by number. Index 0 of each multi-form cell is the declined derivation (`-g`/`-q`, `-tu`, `-Di`, `-yAd`). `mise run fmt` re-wraps.

```rust
    ("03.0012", "laT", Pada::Parasmaipada, ["nenekti", "neniktaH", "nenijati", "nenekzi", "nenikTaH", "nenikTa", "nenejmi", "nenijvaH", "nenijmaH"]),
    ("03.0012", "laN", Pada::Parasmaipada, ["aneneg", "aneniktAm", "anenijuH", "aneneg", "aneniktam", "anenikta", "anenijam", "anenijva", "anenijma"]),
    ("03.0012", "loT", Pada::Parasmaipada, ["nenektu", "neniktAm", "nenijatu", "nenigDi", "neniktam", "nenikta", "nenijAni", "nenijAva", "nenijAma"]),
    ("03.0012", "viDiliN", Pada::Parasmaipada, ["nenijyAd", "nenijyAtAm", "nenijyuH", "nenijyAH", "nenijyAtam", "nenijyAta", "nenijyAm", "nenijyAva", "nenijyAma"]),
    ("03.0012", "laT", Pada::Atmanepada, ["nenikte", "nenijAte", "nenijate", "nenikze", "nenijATe", "nenigDve", "nenije", "nenijvahe", "nenijmahe"]),
    ("03.0012", "laN", Pada::Atmanepada, ["anenikta", "anenijAtAm", "anenijata", "anenikTAH", "anenijATAm", "anenigDvam", "aneniji", "anenijvahi", "anenijmahi"]),
    ("03.0012", "loT", Pada::Atmanepada, ["neniktAm", "nenijAtAm", "nenijatAm", "nenikzva", "nenijATAm", "nenigDvam", "nenijE", "nenijAvahE", "nenijAmahE"]),
    ("03.0012", "viDiliN", Pada::Atmanepada, ["nenijIta", "nenijIyAtAm", "nenijIran", "nenijITAH", "nenijIyATAm", "nenijIDvam", "nenijIya", "nenijIvahi", "nenijImahi"]),
    ("03.0013", "laT", Pada::Parasmaipada, ["vevekti", "veviktaH", "vevijati", "vevekzi", "vevikTaH", "vevikTa", "vevejmi", "vevijvaH", "vevijmaH"]),
    ("03.0013", "laN", Pada::Parasmaipada, ["aveveg", "aveviktAm", "avevijuH", "aveveg", "aveviktam", "avevikta", "avevijam", "avevijva", "avevijma"]),
    ("03.0013", "loT", Pada::Parasmaipada, ["vevektu", "veviktAm", "vevijatu", "vevigDi", "veviktam", "vevikta", "vevijAni", "vevijAva", "vevijAma"]),
    ("03.0013", "viDiliN", Pada::Parasmaipada, ["vevijyAd", "vevijyAtAm", "vevijyuH", "vevijyAH", "vevijyAtam", "vevijyAta", "vevijyAm", "vevijyAva", "vevijyAma"]),
    ("03.0013", "laT", Pada::Atmanepada, ["vevikte", "vevijAte", "vevijate", "vevikze", "vevijATe", "vevigDve", "vevije", "vevijvahe", "vevijmahe"]),
    ("03.0013", "laN", Pada::Atmanepada, ["avevikta", "avevijAtAm", "avevijata", "avevikTAH", "avevijATAm", "avevigDvam", "aveviji", "avevijvahi", "avevijmahi"]),
    ("03.0013", "loT", Pada::Atmanepada, ["veviktAm", "vevijAtAm", "vevijatAm", "vevikzva", "vevijATAm", "vevigDvam", "vevijE", "vevijAvahE", "vevijAmahE"]),
    ("03.0013", "viDiliN", Pada::Atmanepada, ["vevijIta", "vevijIyAtAm", "vevijIran", "vevijITAH", "vevijIyATAm", "vevijIDvam", "vevijIya", "vevijIvahi", "vevijImahi"]),
    ("03.0014", "laT", Pada::Parasmaipada, ["vevezwi", "vevizwaH", "vevizati", "vevekzi", "vevizWaH", "vevizWa", "vevezmi", "vevizvaH", "vevizmaH"]),
    ("03.0014", "laN", Pada::Parasmaipada, ["aveveq", "avevizwAm", "avevizuH", "aveveq", "avevizwam", "avevizwa", "avevizam", "avevizva", "avevizma"]),
    ("03.0014", "loT", Pada::Parasmaipada, ["vevezwu", "vevizwAm", "vevizatu", "veviqQi", "vevizwam", "vevizwa", "vevizARi", "vevizAva", "vevizAma"]),
    ("03.0014", "viDiliN", Pada::Parasmaipada, ["vevizyAd", "vevizyAtAm", "vevizyuH", "vevizyAH", "vevizyAtam", "vevizyAta", "vevizyAm", "vevizyAva", "vevizyAma"]),
    ("03.0014", "laT", Pada::Atmanepada, ["vevizwe", "vevizAte", "vevizate", "vevikze", "vevizATe", "veviqQve", "vevize", "vevizvahe", "vevizmahe"]),
    ("03.0014", "laN", Pada::Atmanepada, ["avevizwa", "avevizAtAm", "avevizata", "avevizWAH", "avevizATAm", "aveviqQvam", "avevizi", "avevizvahi", "avevizmahi"]),
    ("03.0014", "loT", Pada::Atmanepada, ["vevizwAm", "vevizAtAm", "vevizatAm", "vevikzva", "vevizATAm", "veviqQvam", "vevizE", "vevizAvahE", "vevizAmahE"]),
    ("03.0014", "viDiliN", Pada::Atmanepada, ["vevizIta", "vevizIyAtAm", "vevizIran", "vevizITAH", "vevizIyATAm", "vevizIDvam", "vevizIya", "vevizIvahi", "vevizImahi"]),
```

- [ ] **Step 5: Add the `ALTERNATES` rows**

**Append** to the end of `ALTERNATES`, just before its closing `];`. Each row is (root, lakāra, pada, 0-based cell index in the order P.E P.D P.B M.E M.D M.B U.E U.D U.B, form, key); the key is the optional rules behind the form, in pipeline order.

```rust
    ("03.0012", "laN", Pada::Parasmaipada, 0, "anenek", "8.4.56"),
    ("03.0012", "laN", Pada::Parasmaipada, 3, "anenek", "8.4.56"),
    ("03.0012", "loT", Pada::Parasmaipada, 0, "neniktAd", "7.1.35"),
    ("03.0012", "loT", Pada::Parasmaipada, 0, "neniktAt", "7.1.35+8.4.56"),
    ("03.0012", "loT", Pada::Parasmaipada, 3, "neniktAd", "7.1.35"),
    ("03.0012", "loT", Pada::Parasmaipada, 3, "neniktAt", "7.1.35+8.4.56"),
    ("03.0012", "viDiliN", Pada::Parasmaipada, 0, "nenijyAt", "8.4.56"),
    ("03.0013", "laN", Pada::Parasmaipada, 0, "avevek", "8.4.56"),
    ("03.0013", "laN", Pada::Parasmaipada, 3, "avevek", "8.4.56"),
    ("03.0013", "loT", Pada::Parasmaipada, 0, "veviktAd", "7.1.35"),
    ("03.0013", "loT", Pada::Parasmaipada, 0, "veviktAt", "7.1.35+8.4.56"),
    ("03.0013", "loT", Pada::Parasmaipada, 3, "veviktAd", "7.1.35"),
    ("03.0013", "loT", Pada::Parasmaipada, 3, "veviktAt", "7.1.35+8.4.56"),
    ("03.0013", "viDiliN", Pada::Parasmaipada, 0, "vevijyAt", "8.4.56"),
    ("03.0014", "laN", Pada::Parasmaipada, 0, "avevew", "8.4.56"),
    ("03.0014", "laN", Pada::Parasmaipada, 3, "avevew", "8.4.56"),
    ("03.0014", "loT", Pada::Parasmaipada, 0, "vevizwAd", "7.1.35"),
    ("03.0014", "loT", Pada::Parasmaipada, 0, "vevizwAt", "7.1.35+8.4.56"),
    ("03.0014", "loT", Pada::Parasmaipada, 3, "vevizwAd", "7.1.35"),
    ("03.0014", "loT", Pada::Parasmaipada, 3, "vevizwAt", "7.1.35+8.4.56"),
    ("03.0014", "viDiliN", Pada::Parasmaipada, 0, "vevizyAt", "8.4.56"),
```

That is 21 rows, so 216 cells + 21 = 237 forms, vidyut's count. These rows are the first juhotyādi laṅ **madhyama** eka forks. A consonant-final root's 2sg loses its `s` to 8.2.23, the same way rudhādi's does. The ātmanepada columns fork nowhere.

- [ ] **Step 6: Update `paradigm/main.rs`**

In `every_alternate_names_the_vikalpa_rules_that_produced_it`'s doc comment: `otherwise 1037 bare strings` → `otherwise 1058 bare strings`.

In `derivation_set_shape_matches_the_audited_numbers`:
- `assert_eq!(total_cells, 4212, "468 root×lakāra blocks × 9 cells each")` → `4428`, `"492 root×lakāra blocks × 9 cells each"`.
- `ones` 3451 → `3652`; `twos` 578 → `587`.
- `threes` 137 → `143`. In its message, change the final `and — new in slice 3d2 — √ṛ's, the same way"` to `and — new in slice 3d2 — √ṛ's, the same way; and — new in slice 3e — √ṇij's, √vij's and \` / `√viṣ's, the same way"`. Keep the string's `\` line-continuation style.
- `fours`, `fives`, `sixes`, `sevens` unchanged.
- `ALTERNATES.len()` 1037 → `1058`.
- `key_count("8.4.56")` 153 → `162`; `key_count("7.1.35")` 138 → `144`; `key_count("7.1.35+8.4.56")` 138 → `144`.

Check: 3652 + 587 + 143 + 18 + 10 + 17 + 1 = 4428 cells, and 3652 + 1174 + 429 + 72 + 50 + 102 + 7 = 5486 forms = 4428 + 1058.

In the doc comment above that test:
- `4212 cells total (468 root×lakāra blocks × 9), of which 3451 hold exactly` / `one form, 578 hold two, 137 hold three (` → `4428 cells total (492 root×lakāra blocks × 9), of which 3652 hold exactly one form, 587 hold two, 143 hold three (`.
- In that three-form parenthesis, after `and √ṛ's, new in slice 3d2`, add `, and √ṇij's, √vij's and √viṣ's, new in slice 3e`.
- `itself has 1037 rows, keyed 153 \`8.4.56\`, 138 \`7.1.35\`, 138 \`7.1.35+8.4.56\`,` → `itself has 1058 rows, keyed 162 \`8.4.56\`, 144 \`7.1.35\`, 144 \`7.1.35+8.4.56\`,`.
- After the slice-3d2 paragraph (ending `does not fork. Five new rows, all in the three pre-existing keys. The gaṇa` / `is PARTIAL at 17 of its 26 rows.`), add:

```rust
///
/// Slice 3e curates √ṇij (`03.0012`), √vij (`03.0013`) and √viṣ (`03.0014`),
/// ubhayapadī by 1.3.72, bringing the gaṇa to twenty of its twenty-six rows.
/// Its machinery (7.4.75, 7.3.87 barring 7.3.86) adds no vikalpa and no fork
/// kind. Each parasmaipada column forks where every consonant-final
/// parasmaipada root does: laṅ prathama AND madhyama eka on 8.4.56
/// (`aneneg`/`anenek`, 8.2.23 having taken the 2sg `s`), vidhiliṅ prathama eka
/// on 8.4.56 (`nenijyAd`/`nenijyAt`), and the two loṭ tātaṅ cells three ways
/// (`nenektu`/`neniktAd`/`neniktAt`, `nenigDi`/`neniktAd`/`neniktAt`). The
/// ātmanepada columns fork nowhere. Twenty-one new rows, seven per root, all in
/// the three pre-existing keys. The gaṇa is PARTIAL at 20 of its 26 rows.
```

In `pada_ambiguous_surfaces_are_exactly_these`, append to the end of the long comment (after `parasmaipada-only and contribute nothing.`):

```rust
    // Slice 3e's √ṇij, √vij and √viṣ, ubhayapadī, each contribute the same
    // two-surface shape as √bhṛ's: `neniktAm`/`anenikta`,
    // `veviktAm`/`avevikta` and `vevizwAm`/`avevizwa`. That takes the set from
    // fifty to fifty-six, with no new collision against any pre-slice surface.
    // Measured: no 3e form equals any pre-slice form in either pada.
```

and replace the expected vector with:

```rust
        vec![
            "ArRuta", "BinttAm", "BuNktAm", "CfnttAm", "CinttAm", "DattAm", "GfRutAm", "aBintta",
            "aBuNkta", "aDatta", "aGfRuta", "abiBfta", "acCfntta", "acCintta", "adatta", "akuruta",
            "akzaRuta", "akziRuta", "akzuntta", "anayata", "anenikta", "ariNkta", "arundDa",
            "asanuta", "atanuta", "atfRuta", "atfntta", "atudata", "avevikta", "avevizwa",
            "aviNkta", "ayuNkta", "biBftAm", "dattAm", "fRutAm", "kurutAm", "kzaRutAm",
            "kziRutAm", "kzunttAm", "nayatAm", "nayetAm", "nayeta", "neniktAm", "riNktAm",
            "rundDAm", "sanutAm", "tanutAm", "tfRutAm", "tfnttAm", "tudatAm", "tudetAm", "tudeta",
            "veviktAm", "vevizwAm", "viNktAm", "yuNktAm",
        ]
```

After `r_root_analyses_its_reduplicated_forms`, add the Review Focus test:

```rust
/// √vij (`03.0013`) shares its code with tudādi's `06.0009` and rudhādi's
/// `07.0023`, so `check`'s `dhatu` field cannot tell the three apart. Only the
/// juhotyādi row reduplicates, and only it is credited 7.4.75. Every analysis
/// of a 3e form must carry 7.4.75 in its trace. A pada-ambiguous surface must
/// report both padas.
#[test]
fn nij_roots_analyse_their_reduplicated_forms() {
    let engine = Panini::new();
    for (form, root) in [("nenekti", "nij"), ("nenijAni", "nij"), ("vevekti", "vij"), ("vevezwi", "viz")] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert!(!r.analyses.is_empty(), "{form}");
        for a in &r.analyses {
            assert_eq!(a.dhatu, root, "{form}");
            assert!(a.trace.iter().any(|s| s.sutra == "7.4.75"), "{form}: {:?}", a.trace);
        }
    }
    for form in ["neniktAm", "anenikta", "veviktAm", "avevizwa"] {
        let r = engine.check(form);
        let mut padas: Vec<Pada> = r.analyses.iter().map(|a| a.pada).collect();
        padas.dedup();
        assert!(
            padas.contains(&Pada::Parasmaipada) && padas.contains(&Pada::Atmanepada),
            "{form}: {padas:?}"
        );
    }
}
```

If `Pada` is not imported in `main.rs`, add it to the existing `panini_data` import. If `a.trace`'s element type exposes the sūtra under a different field name, use that field. `RuleStep` is re-exported by `panini`; check `crates/panini/src/lib.rs`.

- [ ] **Step 7: Run the full suite**

Run: `mise run test 2>&1 | tail -40` (foreground, timeout 600000 ms)
Expected: PASS at 4428 cells, every 4212 prior unchanged.

If a 3e cell fails, read it against the spec before touching anything, using superpowers:systematic-debugging. **Do not edit a golden to match the engine.** Likely causes by symptom:
- `ni-`/`vi-` abhyāsa: 7.4.75 did not fire. Check its row keys (Task 1).
- `-nejAni`/`-nejam`/`-nejE`/`-vezARi`: 7.3.87 did not fire, or its bar did not reach 7.3.86 (Task 2).
- `nenijmi` instead of `nenejmi`: 7.3.87 fired on a consonant-initial ending.
- A sandhi difference on √viṣ (`z` + `t`/`s`/`D`): the probe found none. Stop and report the trace.

- [ ] **Step 8: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-data/src/lib.rs crates/panini/tests/paradigm/data/juhotyadi.rs crates/panini/tests/paradigm/main.rs
git commit -m "feat(data): √ṇij, √vij, √viṣ (03.0012–03.0014) — juhotyādi at twenty of twenty-six rows

4212 → 4428 cells, 5249 → 5486 forms, 1037 → 1058 ALTERNATES, 94 → 97 roots."
```

---

## Task 4: The trace pins

**Files:**
- Modify: `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes: `crate::helpers::{at, cell_trace}`. `cell_trace(number, lakara, pada, purusha, vacana) -> (String, Vec<String>)` returns index 0's text and trace. `at(&[String], &str) -> usize` panics if the sūtra is absent. Index 0 is the declined derivation, which is the form each pin asserts.

- [ ] **Step 1: Update the module doc**

In the file's `//!` doc, after the sentence ending `…and reaches 6.4.78 later.`, add: `//! Slice 3e's 7.4.75 runs inside the same stage, after 7.4.60, so 7.4.59 never fires on its rows, and 7.3.87 sits in \`guna\` after the whole stage, where vidyut credits it between 6.1.10 and 7.4.60.` Reflow to the file's wrap.

- [ ] **Step 2: Add the pins**

At the end of the file, add:

```rust
#[test]
fn nenekti_trace_is_dvitva_haladih_shesha_nijam_then_upadha_guna() {
    // nij P laT P.E. A pit, consonant-initial ending: 7.3.87 declines (*aci*),
    // and 7.3.86 guṇates the root after the abhyāsa stage has run.
    let (text, t) = cell_trace(
        "03.0012",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "nenekti", "got {t:?}");
    assert!(at(&t, "6.1.10") < at(&t, "7.4.60"), "got {t:?}");
    assert!(at(&t, "7.4.60") < at(&t, "7.4.75"), "got {t:?}");
    assert!(at(&t, "7.4.75") < at(&t, "7.3.86"), "got {t:?}");
    assert!(at(&t, "7.3.86") < at(&t, "8.2.30"), "got {t:?}");
    assert!(at(&t, "8.2.30") < at(&t, "8.4.55"), "got {t:?}");
    assert!(!t.contains(&"7.4.59".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.87".to_string()), "got {t:?}");
}

#[test]
fn nenijati_trace_has_nijam_and_no_upadha_guna_or_its_block() {
    // nij P laT P.B. `ati` is vowel-initial but ṅit (1.2.4). 7.3.86 declines
    // on its own, and 7.3.87 must not fire (*piti*).
    let (text, t) = cell_trace(
        "03.0012",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "nenijati", "got {t:?}");
    assert!(at(&t, "7.4.75") < at(&t, "7.1.4"), "got {t:?}");
    assert!(!t.contains(&"7.3.86".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.87".to_string()), "got {t:?}");
}

#[test]
fn nenijani_trace_is_nabhyastasyaci_barring_upadha_guna() {
    // nij P loT U.E. 3.4.92's `Ani` is pit but untagged. 7.3.87 fires and
    // bars 7.3.86, so the root keeps its `i`.
    let (text, t) = cell_trace(
        "03.0012",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "nenijAni", "got {t:?}");
    assert!(at(&t, "3.4.92") < at(&t, "7.4.75"), "got {t:?}");
    assert!(at(&t, "7.4.75") < at(&t, "7.3.87"), "got {t:?}");
    assert!(!t.contains(&"7.3.86".to_string()), "got {t:?}");
}

#[test]
fn anenijam_trace_is_at_then_nabhyastasyaci_on_the_tagged_am() {
    // nij P laN U.E. Laṅ's `am` keeps mip's Pit tag. 6.4.71's aṭ goes on, and
    // 7.3.87 blocks the guṇa.
    let (text, t) = cell_trace(
        "03.0012",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "anenijam", "got {t:?}");
    assert!(at(&t, "7.4.75") < at(&t, "6.4.71"), "got {t:?}");
    assert!(at(&t, "6.4.71") < at(&t, "7.3.87"), "got {t:?}");
    assert!(!t.contains(&"7.3.86".to_string()), "got {t:?}");
}

#[test]
fn nenikte_trace_has_nijam_and_no_guna() {
    // nij A laT P.E. Ātmanepada `te` is ṅit: no guṇa, no block. 8.2.30 then
    // 8.4.55 take j → g → k.
    let (text, t) = cell_trace(
        "03.0012",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "nenikte", "got {t:?}");
    assert!(at(&t, "7.4.75") < at(&t, "8.2.30"), "got {t:?}");
    assert!(at(&t, "8.2.30") < at(&t, "8.4.55"), "got {t:?}");
    assert!(!t.contains(&"7.3.86".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.87".to_string()), "got {t:?}");
}

#[test]
fn vevezwi_trace_is_upadha_guna_then_shtutva() {
    // viz P laT P.E. z + t: 8.4.41 retroflexes the `t`; 8.2.41 (before s only)
    // stays out.
    let (text, t) = cell_trace(
        "03.0014",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "vevezwi", "got {t:?}");
    assert!(at(&t, "7.4.75") < at(&t, "7.3.86"), "got {t:?}");
    assert!(at(&t, "7.3.86") < at(&t, "8.4.41"), "got {t:?}");
    assert!(!t.contains(&"8.2.41".to_string()), "got {t:?}");
}

#[test]
fn vevekzi_trace_is_shadhoh_kah_si_then_adesha_pratyayayoh() {
    // viz P laT M.E. z + s: 8.2.41 makes the root's `z` a `k`, then 8.3.59
    // makes the ending's `s` a `z`.
    let (text, t) = cell_trace(
        "03.0014",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "vevekzi", "got {t:?}");
    assert!(at(&t, "7.3.86") < at(&t, "8.2.41"), "got {t:?}");
    assert!(at(&t, "8.2.41") < at(&t, "8.3.59"), "got {t:?}");
}

#[test]
fn juhavani_trace_has_no_nabhyastasyaci() {
    // hu P loT U.E. This is a prior cell, pinned for 7.3.87's *laghūpadhasya*
    // clause. √hu's aṅga is vowel-final, so 7.3.84's guṇa stands
    // (hu → ho → hav) and 7.3.87 never fires. vidyut credits it on no
    // curated row.
    let (text, t) = cell_trace(
        "03.0001",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "juhavAni", "got {t:?}");
    assert!(at(&t, "3.4.92") < at(&t, "7.3.84"), "got {t:?}");
    assert!(!t.contains(&"7.3.87".to_string()), "got {t:?}");
}
```

- [ ] **Step 3: Run the suite**

Run: `mise run test 2>&1 | tail -20` (foreground, timeout 600000 ms)
Expected: PASS. If a pin fails on ORDER, report the engine's order; do not "fix" it toward vidyut's. Stop and report the trace. If a pin fails on an id this plan assumed (for example 8.2.30 vs another *coḥ kuḥ* credit), read the actual trace and correct the pin to the engine's real credit only when the form is right and the spec does not name that id. Say so in the commit message.

- [ ] **Step 4: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini/tests/trace/juhotyadi.rs
git commit -m "test(trace): 3e pins — nenekti, nenijati, nenijAni, anenijam, nenikte, vevezwi, vevekzi, juhavAni"
```

---

## Task 5: Audit, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (audit prose), `crates/panini-prakriya/src/tinanta/anga.rs` (one comment), `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md` (one note)

**Interfaces:**
- Consumes: the finished engine and goldens (Tasks 1–4). Produces no symbols.

- [ ] **Step 1: Update the audit harness's asserted totals**

In `tools/audit/panini_full_audit.rs`:
- header `for each of the 94 curated roots` (or its wrapped form) → `97`
- `(two apiece for the twenty-three roots that admit both padas —` / `twenty-two ubhayapadī by 1.3.72, plus √bhuj by 1.3.66)` → `twenty-six` and `twenty-five`
- `94 roots, 4212 cells, 5249 forms` → `97 roots, 4428 cells, 5486 forms`
- `468 root×pada×lakāra` / `blocks × 9 cells, plus 1037` → `492 … plus 1058`
- `the full 4212-cell table` → `4428-cell`
- `assert_eq!(roots_seen.len(), 94, …)` → `97`
- `assert_eq!(n_cells, 4212, "cells: 468 root×pada×lakāra blocks × 9")` → `4428`, `"cells: 492 root×pada×lakāra blocks × 9"`
- `assert_eq!(n_forms, 5249, "forms: 4212 cells + 1037 ALTERNATES rows")` → `5486`, `"forms: 4428 cells + 1058 ALTERNATES rows"`

In `tools/audit/README.md`, `(94 roots, 4212 cells, 5249 forms)` → `(97 roots, 4428 cells, 5486 forms)`.

- [ ] **Step 2: Repoint vidyut's dev-deps at THIS worktree and run the audit**

`/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes absolute dev-dep paths to `/workspace/crates`. That is the `main` checkout, which does not have this slice. Auditing without repointing checks the pre-slice engine and passes vacuously.

```bash
WT="$(git rev-parse --show-toplevel)"
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
cp tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" mise exec rust@1.98.1 -- cargo run --release --example panini_full_audit 2>&1 | tail -15)
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.98.1 -- cargo run --release --example panini_full_audit 2>&1 | tail -8)
```

Copy the committed harness; never rewrite it.
- Expected from the honest run: `AUDIT PASSED: 4428 cells, 5486 forms, zero differences.`
- Expected from the `entry` control: exit 1 with 36 √bhū cells.

If the honest run shows differences, stop and report, and edit nothing. After both runs, restore the dev-deps so later probes from `main` work:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"/workspace/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"/workspace/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
```

- [ ] **Step 3: Record the audit**

In `tools/audit/README.md`, immediately under `## Last recorded result`, add a new dated entry above the 3d2 one, using today's date:

```markdown
YYYY-MM-DD, juhotyādi 3e slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 4428
cells / 5486 forms / 97 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole juhotyādi 3e slice: 7.4.75 *nijāṁ trayāṇāṁ
guṇaḥ ślau* and 7.3.87 *nābhyastasyāci piti sārvadhātuke* (barring 7.3.86),
added for √ṇij (`03.0012`), √vij (`03.0013`) and √viṣ (`03.0014`), with √viṣ's
`z` sandhi reaching the existing tripādī rules unchanged.

Totals: 97 = 94 + 3; 4428 = 4212 + 216 (24 root×pada×lakāra blocks × 9); 5486
= 5249 + 216 + 21 new `ALTERNATES` rows (1037 → 1058), measured via the
harness's corpus block, not assumed.
```

In `crates/panini/tests/paradigm/main.rs`'s audit-chain doc comment, find the sentence ending `…and juhotyādi 3d2's re-ran it at` / `the same commit over all 4212 cells / 5249 forms / 94 roots with zero` / `differences, its \`entry\` negative control verified failing (36 √bhū` / `cells)`. Before its final `.`, add `, and juhotyādi 3e's re-ran it at the same commit over all 4428 cells / 5486 forms / 97 roots with zero differences, its \`entry\` negative control verified failing (36 √bhū cells)`.

- [ ] **Step 4: README.md**

- `**partial** at 17 of its 26 dhātupāṭha rows` → `**partial** at 20 of its 26 dhātupāṭha rows`.
- After the sentence ending `in laṅ the āṭ merges into the abhyāsa (*aiyaḥ*, \`EyaH\`).`, insert: `Slice 3e added √ṇij (\`03.0012\`, *nenekti*), √vij (\`03.0013\`, *vevekti*) and √viṣ (\`03.0014\`, *veveṣṭi*), all ubhayapadī, behind 7.4.75 *nijāṁ trayāṇāṁ guṇaḥ ślau* (the abhyāsa's guṇa) and 7.3.87 *nābhyastasyāci piti sārvadhātuke*, which bars 7.3.86's guṇa before a vowel-initial pit ending (*nenijāni*); √ṇij's \`ṇ\` is stored as \`n\`, the stored-form convention covering 6.1.65.`
- `curated 94-root set` → `curated 97-root set`.
- Multi-form paragraph: `761 of the 4212 cells hold more than one form: 578` / `hold two, 137 hold three (` → `776 of the 4428 cells hold more than one form: 587 hold two, 143 hold three (`. After `and — new in slice 3d2 — √ṛ's` add `, and — new in slice 3e — √ṇij's, √vij's and √viṣ's`. Check: 587 + 143 + 18 + 10 + 17 + 1 = 776.
- Both-pada paragraph: `twenty-three roots that admit both padas` → `twenty-six`; `(twenty-two ubhayapadī by 1.3.72:` → `(twenty-five ubhayapadī by 1.3.72:`; `√kṛ, √dā, √dhā and √bhṛ;` → `√kṛ, √dā, √dhā, √bhṛ, √nij, √vij and √viṣ;`.
- Ambiguity paragraph: `Fifty surfaces are pada-ambiguous` → `Fifty-six surfaces are pada-ambiguous`. Insert `anenikta`, `avevikta`, `avevizwa`, `neniktAm`, `veviktAm`, `vevizwAm` into the backticked list in the same order as the test's vector (Task 3 Step 6). After `(slice 3d) the same again: \`abiBfta\` and \`biBftAm\`.` (before `That enumeration is no`), insert: ` Slice 3e's three roots do the same again: \`anenikta\`/\`neniktAm\`, \`avevikta\`/\`veviktAm\` and \`avevizwa\`/\`vevizwAm\`.`

Reflow edited paragraphs to the file's ~80-column wrap.

- [ ] **Step 5: docs/ARCHITECTURE.md**

- Stage table, `abhyasa.rs` row: `7.4.62, 7.4.76, 7.4.77, 7.4.78, 6.4.78` → `7.4.62, 7.4.75, 7.4.76, 7.4.77, 7.4.78, 6.4.78`.
- Stage table, `guna.rs` row: `7.3.83, 7.3.84, 7.3.86, 7.3.86 (again` → `7.3.83, 7.3.84, 7.3.87, 7.3.86, 7.3.86 (again`.
- `pins all 122 ids verbatim` → `pins all 124 ids verbatim`. After `*abhyāsasyāsavarṇe* (7.4.60 and 7.4.77 widened) — 122 total` add ` — then juhotyādi 3e's two: 7.4.75 *nijāṁ trayāṇāṁ guṇaḥ ślau* and 7.3.87 *nābhyastasyāci piti sārvadhātuke* — 124 total`. Confirm 124 by counting `expected` in `tinanta_rule_order_is_pinned`.
- `**partial** at 17 of its 26 rows (… √ṛ; slice 3d2)` → `**partial** at 20 of its 26 rows (… √ṛ; slice 3d2; √ṇij, √vij, √viṣ; slice 3e)`.
- The row-keyed list `(7.4.76, 7.4.77, 8.2.38, 8.2.40's *adhaḥ*, 6.4.116–6.4.118)` → `(7.4.75, 7.4.76, 7.4.77, 8.2.38, 8.2.40's *adhaḥ*, 6.4.116–6.4.118)`.
- The `Rule.bars` paragraph: after the sentence ending `fall short of the 2^k product: 6.4.117's branch never reaches 6.4.116's fork.`, add: `The second user, and the first mandatory one, is 7.3.87 *nābhyastasyāci piti sārvadhātuke*: it changes no text and bars 7.3.86 on an abhyasta laghūpadha aṅga before a vowel-initial pit ending (*nenijAni*, not \*nenejAni), reading *piti* the way 1.2.4 does, since the loṭ uttama endings are pit by 3.4.92 but never tagged.`
- The 7.1.35 / 8.4.56 paragraph (`grep -n "forking 138 cells" docs/ARCHITECTURE.md`):
  - `forking 138 cells (loṭ` → `forking 144 cells (loṭ`
  - `across the 69 roots with a parasmaipada column` → `72`
  - `the twenty-three roots that admit both` → `twenty-six`; `(twenty-two ubhayapadī by 1.3.72 —` → `(twenty-five ubhayapadī by 1.3.72 —`; `√kṛ, √dā, √dhā and √bhṛ —` → `√kṛ, √dā, √dhā, √bhṛ, √nij, √vij and √viṣ —`
  - `and` / `√bhṛ's parasmaipada column; 69 + 25 = the 94 curated roots)` → `and the parasmaipada columns of √bhṛ, √nij, √vij and √viṣ; 72 + 25 = the 97 curated roots)`
  - `forking 153 cells outright` → `162`
  - `those same 69 parasmaipada columns (131 of them` → `those same 72 parasmaipada columns (137 of them`
  - `plus twenty-two rudhādi laṅ *madhyama* eka cells (` … `bucket),` → keep the rudhādi list and add after its closing parenthesis: `, plus juhotyādi 3e's three laṅ madhyama eka cells (√nij, √vij, √viṣ: \`aneneg ~ anenek\`)`
  - `forking a further 138 (the same` → `forking a further 144 (the same`

  Check: 137 + 22 + 3 = 162. If wording differs, keep its form and make the same substitution. What must be right: 3 new parasmaipada columns, 6 new `7.1.35` rows, 9 new `8.4.56` rows (6 prathama eka, 3 madhyama eka).

- [ ] **Step 6: AGENTS.md**

- Rules of the codebase: `(\`crates/panini/tests/paradigm/\`, 4212 cells,` → `4428 cells`. In the juhotyādi clause, `and now at 17 of its 26 after slice 3d2 curated √ṛ —` → `at 17 after slice 3d2 curated √ṛ, and now at 20 of its 26 after slice 3e curated √ṇij, √vij and √viṣ —`. `(1037 rows in all, so 4212 + 1037 = 5249 forms total)` → `(1058 rows in all, so 4428 + 1058 = 5486 forms total)`. Bucket counts: `a second (578 cells), a third (137 cells)` → `(587 cells)`, `(143 cells)`.
- The number-keyed bullet: `7.4.76, 6.4.116, 6.4.117, 6.4.118, 8.2.38` → `7.4.75, 7.4.76, 6.4.116, 6.4.117, 6.4.118, 8.2.38`. After `and 7.4.77 (\`03.0004\`, \`03.0005\`, \`03.0017\`) follows that precedent.`, add ` 7.4.75 (\`03.0012\`–\`03.0014\`) does too: \`vij\` is also \`06.0009\` and \`07.0023\`.` Update the test name it cites from `juhotyadi_rows_are_the_seventeen_curated_roots` to `juhotyadi_rows_are_the_twenty_curated_roots`.
- The `Rule.bars` bullet: after `so without its bars 6.4.116, 6.4.113 and 6.4.112 would each still rewrite` / `the \`A\` it keeps.`, add ` 7.3.87 is the second and the first mandatory one: it changes no text and bars 7.3.86, so 7.3.86's guard carries no abhyasta exception.`
- The audit record: after `and that by juhotyādi 3d2's` / `(\`tools/audit/README.md\`'s 2026-09-30 entry, 4212 cells / 5249 forms / 94` / `roots)` add `, and that by juhotyādi 3e's (\`tools/audit/README.md\`'s YYYY-MM-DD entry, 4428 cells / 5486 forms / 97 roots)`.
- The stale-comment ledger: `4212 goldens` / `would move today` → `4428 goldens`. After the 3d2 sentence's end (`…both lines measured by grep at this commit).`, before ` A third,`), add: ` Juhotyādi 3e touched neither comment either; the corpus stands at 4428 cells as of 3e (\`guna.rs:2228\`'s claim now anchored at \`guna.rs:<G>\`, \`controller.rs:206\`'s at \`controller.rs:<C>\`: 3e's 7.3.87 rule and its tests landed in \`guna.rs\` above the test, while \`controller.rs\` is unchanged; both lines measured by grep at this commit).` Measure `<G>` with `grep -n "1872 goldens move" crates/panini-prakriya/src/tinanta/guna.rs` and `<C>` with `grep -n "only 8 cells fire" crates/panini-prakriya/src/controller.rs`. Expect `<C>` = 206. `<G>` moves by however many lines Task 2 added above it, roughly 2228 + 60. Measure it; never compute it.

- [ ] **Step 7: The one stale engine comment, and the 3a spec note**

In `crates/panini-prakriya/src/tinanta/anga.rs`'s 6.4.71 comment, change `can falsify (3e's and 3f's roots are consonant-initial), so the ANGA` to `can falsify (3e's curated roots and 3f's are all consonant-initial), so the ANGA`. Reflow the comment if the line grows past the file's wrap.

In `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md`, after the slice-3d2 note beneath the "Later slices" table, add:

```markdown
> Slice 3e (`2026-10-01-juhotyadi-gana-3e-design.md`) took √ṇij, √vij and √viṣ,
> 216 cells: 7.4.75 and 7.3.87, the latter barring 7.3.86 through `Rule.bars`.
> 6.1.65 is not written: `Ri\ji~^r` is stored as `nij` under the stored-form
> convention, as √nī's `RI\Y` is.
```

- [ ] **Step 8: Sweep for anything left stale**

```bash
grep -rn "4212\|5249\|\b1037\b\|94 roots\|94-root\|of these 94\|of the 94\|\b468\b\|761 of\|17 of its 26\|seventeen of its twenty-six\|122 ids\|122 total\|forking 138\|forking 153\|131 of them\|69 + 25\|\b578\b\|137 hold\|(137 cells)\|twenty-three roots\|twenty-two ubhayapad\|Fifty surfaces\|fifty surfaces\|juhotyadi_rows_are_the_seventeen" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.md --include=*.rs
grep -rn "\b3e\b\|slice 3e\|3e's" crates tools README.md AGENTS.md docs/ARCHITECTURE.md
grep -rn "\bnij\b\|\bvij\b\|\bviz\b\|Ri\\\\ji\|7\.4\.75\|7\.3\.87\|first user\|only curated root with an ik upadhā" crates/panini-prakriya/src --include=*.rs | grep "//"
```

Expected residue:
- AGENTS.md's dated mutation record and anything naming 3d2's numbers explicitly as 3d2's. That is history; never rewrite it.
- "3e" where it now records what 3e *did* (the new comments from Tasks 1–5).
- `guna.rs`'s 7.3.86 comment (`The only curated root with an ik upadhā`) was already stale before 3e and is not this slice's to fix. Leave it unless the reviewer asks.

Every "3e" that promises something *will* happen is now false: rewrite it as done or delete it. For every root-shape hit, check the comment is still true with 3e curated.

- [ ] **Step 9: Run the full suite and commit**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms). Expected: PASS at 4428 cells.

```bash
mise run fmt && mise run lint
git add -A
git commit -m "docs: 3e's counts, the audit record, and the sweep

4428 cells / 5486 forms / 97 roots / 1058 ALTERNATES across README,
ARCHITECTURE, AGENTS, paradigm/main.rs and tools/audit. Audit at zero
divergence against 8da2f90b."
```

---

## Task 6: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and the current-record paragraph); `mise.toml` only if the cap moves

Follow AGENTS.md's cargo-mutants protocol. Hazards from this repo's record:
- **Measure, never scale.**
- **Every invocation rotates `mutants.out`**, so always pass `-o`.
- **The mise shim fails in background shells.** Use the real binary: `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`.
- **`pgrep -f` matches its own shell.** Wait on `pgrep -x cargo-mutants`.

- [ ] **Step 1: Measure the floor**

With nothing else running, run this twice: `time mise run test 2>&1 | tail -3` (foreground). Record both wall clocks and `cat /proc/loadavg`. The 4212-cell floor was 5.598 s / 5.588 s.

- [ ] **Step 2: Probe the two uncaught equivalents at `-j 4`**

This slice does not touch `adesha.rs` or `tripadi.rs`, so both should sit where AGENTS.md records them. Confirm that, then run them (foreground, timeout 600000 ms):

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:589:30: replace \+ with \*|tripadi.rs:1217:38: replace - with /"
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 60 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:589:30: replace \+ with \*" --re "tripadi.rs:1217:38: replace - with /" 2>&1 | tail -10
```

Both must be MISSED, not TIMEOUT. Read each test-phase duration from `$SCRATCH/mutants.out/outcomes.json`. If `--list` shows either at another position, use the listed position here and below.

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/juhotyadi-3e"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout 60 -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"
```

The last campaign took 13 minutes. Run nothing CPU-heavy meanwhile. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

- [ ] **Step 4: Read the outcomes**

When `pgrep -x cargo-mutants` returns nothing:

```bash
date -u +"%F %T UTC" > "$OUT/finished"
cp "$OUT/mutants.out/outcomes.json" "$OUT/outcomes.durable.json"
tail -5 "$OUT/campaign.log"
cat "$OUT/mutants.out/missed.txt" "$OUT/mutants.out/timeout.txt"
```

Expected (exit code 3 is normal when a timeout is present):
- `missed.txt` holds exactly `adesha.rs:589:30: replace + with *` and `tripadi.rs:1217:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:1543:23: replace -= with /=`.
- panini-analyze: 0 missed, 0 timeout.

If not:
- Any **other timeout** is a suspect survivor. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant in `guna.rs` or `abhyasa.rs` means a Task 1 or Task 2 guard test does not separate it. The likeliest are 7.3.87's `||`/`&&` swaps, a deleted `!`, or the `matches!` upadhā set, and 7.4.75's row match. Strengthen the named test, commit, and re-run only those mutants with `--re` and a fresh `-o`.

- [ ] **Step 5: Margins**

Inspect one record to see how `duration` is stored:

```bash
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["outcomes"][1])' "$OUT/mutants.out/outcomes.json"
```

Compute the two equivalents' test phases under campaign load, the slowest caught phase, and the caught min/median/p90/max. The margins are 60 ÷ the longest equivalent phase and 60 ÷ the slowest caught phase.
- If the first margin is ≥ 5, the cap stays 60.
- Otherwise the new cap is 6 × that phase, rounded up to the next 10 s. Change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together. A higher cap can only turn timeouts into outcomes, and the only timeout is the permanent hang, so the campaign's outcomes stand without a re-run.

- [ ] **Step 6: Record it in AGENTS.md**

- Rewrite `**The floor behind the 60s cap, measured at 4212 cells on` / `2026-09-30.**` with Step 1's and Step 2's numbers at 4428 cells, and the cap Step 5 chose.
- Replace the `**Current record (juhotyādi 3d2, 2026-09-30).**` paragraph with `**Current record (juhotyādi 3e, YYYY-MM-DD).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**;
  - the non-caught set diffed against 3d2's (the clean result is identical);
  - the campaign-load phases and margins;
  - that no 7.4.75 or 7.3.87 mutant is missed, timed out or unviable-without-reason;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`.

  End it with a pointer to the record it replaces: run `git rev-parse --short HEAD` before committing and write `The juhotyādi 3d2 record it replaces: \`git show <that hash>:AGENTS.md\`.`

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md mise.toml
git commit -m "chore: 3e mutation gate — floor and uncaught run re-measured at 4428 cells

missed.txt holds only the two documented equivalents and timeout.txt only
the permanent ṇatva-scan entry; every 7.4.75 and 7.3.87 clause is killed."
```

---

## Task 7: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -20
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin juhotyadi-3e
gh pr create --title "juhotyādi 3e — √ṇij, √vij, √viṣ" --body "$(cat <<'BODY'
Slice 3e curates √ṇij, √vij and √viṣ (`03.0012`–`03.0014`, *nenekti* /
*vevekti* / *veveṣṭi*), all ubhayapadī. This takes the golden suite from 4212 to
4428 cells and juhotyādi to twenty of its twenty-six rows.

**New rules:**
- 7.4.75 *nijāṁ trayāṇāṁ guṇaḥ ślau* applies guṇa to the abhyāsa (`ni` → `ne`).
  It is keyed by row number, since `vij` is also 06.0009 and 07.0023.
- 7.3.87 *nābhyastasyāci piti sārvadhātuke* is a mandatory rule that changes no
  text and bars 7.3.86 through `Rule.bars`. It applies to an abhyasta
  laghūpadha aṅga before a vowel-initial pit ending (*nenijāni*, *anenijam*).
  It reads *piti* the way 1.2.4 does, because the loṭ uttama endings are pit
  by 3.4.92 but never tagged.

6.1.65 is not written: `Ri\ji~^r` is stored as `nij` under the stored-form
convention. There is no new vikalpa, and every √viṣ sandhi step was already
in place.

Before writing the spec, the slice was re-probed against vidyut and HEAD cell
by cell (0/216 on HEAD, two causes). The audit shows zero divergence against
`8da2f90b`, and the mutation gate is clean.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`: `git worktree remove .worktrees/juhotyadi-3e`, then delete the local and remote branch, and run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| 7.4.75 row-keyed, after 7.4.62 / before 7.4.76, `guna_of`, module doc, row table | 1 |
| 7.4.75 unit tests: fires on three rows; declines on `06.0009`/`07.0023`/`""`; stage-run order | 1 |
| 7.3.87: mandatory, `bars: &["7.3.86"]`, between 7.3.84 and 7.3.86, five-clause guard | 2 |
| 7.3.87 comment: 3.4.92 / untagged pit, tanādi-arm inertness, 3f inheritance, *laghūpadhasya* | 2 |
| `exactly_the_pinned_bars`, `tinanta_rule_order_is_pinned`, `Rule.bars` doc | 1, 2 |
| 7.3.87 unit tests, one per clause, plus `Bas` (3f shape) | 2 |
| Rows, counts, gaṇa-row test, 24 blocks, 21 `ALTERNATES`, buckets, keys | 3 |
| Pada-ambiguity set 50 → 56; `check()` spot test (*nenekti*, *nenijAni*, *vevezwi*) | 3 |
| Trace pins *nenekti*, *nenijati*, *nenijAni*, *anenijam*, *nenikte*, *vevezwi*, *vevekzi* (+ *juhavAni*) | 4 |
| Audit with repoint and negative control; README/ARCHITECTURE/AGENTS; 3a-spec note; "3e" sweep | 5 |
| Floor, uncaught probe, campaign, verbatim non-caught record | 6 |

**Type consistency.** The rule ids and names are `"7.4.75"` / `"nijAM trayARAM guRaH SlO"` and `"7.3.87"` / `"nAByastasyAci piti sArvaDAtuke"`. They match across each rule, its `p.record`, the order pin, the bars pin, the unit tests and the trace pins. `nij_prakriya(&str, bool, Lakara, Purusha)` is defined in Task 2 Step 1 and used only there. `abhyasta_prakriya` and `slu_prakriya` / `run_abhyasa_stage` are the existing helpers.

**Known soft spots.**
- **Pit tags.** The probe confirmed that laṅ's `am` keeps Tag::Pit and that loṭ `Ani` has none. Task 2's synthetic tests set the tags by hand, so if either fact is wrong, only Task 3's goldens and Task 4's pins will show it. The symptom table in Task 3 Step 7 names it.
- **`check()` field names.** Task 3's test uses `a.trace` and `s.sutra`; the step says to adapt them if `RuleStep`'s field is named differently.
- **The `threes` message.** That string in `paradigm/main.rs` uses `\` continuations; edit it without breaking them.
- **Laṅ madhyama eka forks.** These are the first in juhotyādi. ARCHITECTURE's 8.4.56 paragraph gains a juhotyādi term beside rudhādi's (Task 5 Step 5).
