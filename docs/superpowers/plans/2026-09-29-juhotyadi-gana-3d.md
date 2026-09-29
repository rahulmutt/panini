# Juhotyādi gaṇa slice 3d Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate the six consonant-initial ṛ-roots of juhotyādi into the engine: √bhṛ (`03.0006`, ubhayapada), √pṛ (`03.0005`), √pṝ (`03.0004`), √ghṛ (`03.0015`), √hṛ (`03.0016`) and √sṛ (`03.0018`). This takes the golden suite from 3924 to 4176 cells. It adds three sūtras (7.4.66, 7.4.77, 7.1.102) and widens four (7.4.60, 7.4.76, 6.1.77, 8.2.77).

**Architecture:** Nine tasks.
- **Tasks 1–4** add or widen rules that fire on no curated root yet. Each is gated on **per-rule guard tests plus the 3924 priors staying byte-identical**.
  - Tasks 1–2 are the abhyāsa stage.
  - Task 3 is the guṇa stage.
  - Task 4 is the tripādī.
- **Task 5** lands the six rows and their goldens, turning 252 cells green in one step.
- **Task 6** adds the trace pins.
- **Tasks 7–9** are the audit and doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.98.1 pinned via `mise`. Tasks: `mise run build | test | test-full | lint | fmt | fmt-check | mutants | audit`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`.

**Spec:** `docs/superpowers/specs/2026-09-29-juhotyadi-gana-3d-design.md`

## Global Constraints

- **The 3924 pre-existing cells must stay byte-identical through Tasks 1–4.** Regenerate no golden and change no pinned trace.
- **No unfalsifiable guard clauses.** A clause that no cell and no guard test can make false is a mutation survivor. Either witness it, delete it, or kill it with a direct guard test on a hand-built `Prakriya`.
- **Rules that name roots key on `p.ctx.dhatupatha`, never on `ANGA.text`.** Each such guard carries a comment naming the upadeśa.
  - 7.4.76 gains `03.0006`.
  - 7.4.77 is keyed on `03.0005` and `03.0004`. `03.0017` (√ṛ) is **not** added; that is slice 3d2's.
  - 7.4.66 and 7.1.102 name sounds, not roots, and guard on text.
- **√ṛ (`03.0017`) is out of scope.** Add no row for it, write no 6.4.78, and do not touch 6.4.71 / 6.4.72 / 6.1.90.
- **Engine order differs from vidyut's on purpose.** This engine copies the bare root before guṇa, so 7.4.66 fires on every 3d cell and 7.1.102 runs after dvitva. Pins hold THIS order. Never "fix" a pin toward vidyut's trace.
- **Goldens are transcribed from this plan** (vidyut's output at `8da2f90b`, generated 2026-09-29 by `/tmp/vidyut-full/vidyut-prakriya/examples/juhotyadi_3d_golden.rs`), never invented. **Do not edit a golden to match the engine.**
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit.
- **`mise run test` takes ~80 s** (AGENTS.md, 3924 cells). Run it in the **foreground** with a timeout of at least 600000 ms; never background it and end a turn. `mise run test-full` takes ~25 minutes or more; same rule, timeout 3600000 ms.
- `mise run test -- -p X` does not scope. Scope unit tests with `mise exec -- cargo test -p <crate> <filter>`.
- Rule ids and SLP1 names of the new rules are copied verbatim from vidyut's `sutrapatha.tsv`:
  - `7.4.66 urat`
  - `7.4.77 artipipartyoSca`
  - `7.1.102 udozWyapUrvasya`

  Existing rules keep their existing `name` strings (`halAdiH SezaH`, `BfYAm it`, `iko yaR aci`, `hali ca`).

## Review Focus

These are inputs the spec implies but no rule's own guard test exercises. Each has its test in the owning task.

1. **√pṛ and √pṝ are homographs wherever guṇa has run** (*piparti*, *apipaH*, *piparARi*). `check()` must report both roots, and only the right one where they diverge (*pipftaH* / *pipUrtaH*). → Task 5, `both_pr_roots_analyse_their_shared_forms`.
2. **A consonant-final abhyāsa that is not ṛ-derived.** √bhas (3f) copies as `Bas`. The widened 7.4.60 must give `Ba` without any 7.4.66. → Task 1, a `Bas` row in `haladih_shesha_elides_every_consonant_but_the_first`.
3. **7.1.102's `v` arm.** *oṣṭhya* includes `v` (vidyut's `OSHTHYA` is pu-varga + `v`), but no curated root witnesses it. A `vF` aṅga must give `vur`. → Task 3, in `ud_oshthyapurvasya_makes_a_labial_final_rr_ur`.
4. **√bhṛ's two new pada-ambiguous surfaces** (`abiBfta`, `biBftAm`). `check()` reports both padas, and README's quoted list must match the test. → Task 5 (the list test) and Task 7 (README).
5. **A live vowel-initial vikaraṇa in front of a consonant-initial ending, under the widened 8.2.77.** The aṅga meets the vikaraṇa's vowel, so the rule must not read past it to the ending. → Task 4, `hali_ca_reads_a_live_vikarana_not_the_ending`.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-prakriya/src/tinanta/abhyasa.rs` | Task 1: 7.4.66, 7.4.60 widened; Task 2: 7.4.76 widened, 7.4.77 |
| `crates/panini-prakriya/src/tinanta/guna.rs` | Task 3: 7.1.102, 6.1.77's aṅga arm |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | Task 4: 8.2.77 widened |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Tasks 1–3: `tinanta_rule_order_is_pinned` |
| `crates/panini-data/src/lib.rs` | Task 5: six `Dhatu` rows, the gaṇa-row test, doc-comment counts |
| `crates/panini/tests/paradigm/data/juhotyadi.rs` | Task 5: 28 `PARADIGM` blocks, 30 `ALTERNATES` rows |
| `crates/panini/tests/paradigm/main.rs` | Task 5: totals, buckets, key census, pada-ambiguous list, homograph test, prose |
| `crates/panini/tests/trace/juhotyadi.rs` | Task 6: trace pins |
| `tools/audit/`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, 3a's spec | Task 7 |
| `AGENTS.md`, `mise.toml` | Task 8: mutation record; the cap if it must move |

---

## Task 1: 7.4.66 *ur at* and the widened 7.4.60 *halādiḥ śeṣaḥ*

The two land together because 7.4.66's `ar` is exactly the shape the old 7.4.60 cannot trim. Each still has its own tests.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/abhyasa.rs` (a rule after 6.1.10; 7.4.60's body and comment; tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: `slu_prakriya(root, ending)` in `abhyasa.rs`'s tests; `is_vowel` (already imported).
- Produces: rule `"7.4.66"`, placed immediately after `"6.1.10"`. 7.4.60 keeps its id and name.

- [ ] **Step 1: Write the failing tests**

At the end of `abhyasa.rs`'s `mod tests`, add:

```rust
    #[test]
    fn ur_at_makes_the_abhyasa_r_vowel_ar() {
        // 7.4.66, with 1.1.51's r-rapara uncredited (as guṇa's `ar` is): the
        // copied root's ṛ-vowel, short or long, becomes `ar`. The aṅga is
        // untouched: the sūtra names the abhyāsa.
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        let rule = rules().find(|r| r.id == "7.4.66").unwrap();
        for (root, want) in [("Bf", "Bar"), ("pF", "par"), ("Gf", "Gar"), ("sf", "sar")] {
            let mut p = slu_prakriya(root, "ti");
            assert!((r_10.apply)(&mut p));
            assert!((rule.apply)(&mut p), "{root}");
            assert_eq!(p.terms[ABHYASA].text, want, "{root}");
            assert_eq!(p.terms[ANGA].text, root, "{root}: the aṅga is untouched");
            assert_eq!(p.log.last().unwrap().sutra, "7.4.66");
        }
    }

    #[test]
    fn ur_at_records_nothing_without_an_r_vowel() {
        // The no-op guard is the whole guard: every pre-3d abhyāsa, and the
        // empty slot of every other gaṇa, must come back unchanged and
        // unlogged, or every prior trace grows a step.
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        let rule = rules().find(|r| r.id == "7.4.66").unwrap();
        for root in ["hu", "ki", "BI", "hrI", "dA", "gA"] {
            let mut p = slu_prakriya(root, "ti");
            assert!((r_10.apply)(&mut p));
            p.log.clear();
            assert!(!(rule.apply)(&mut p), "{root}");
            assert_eq!(p.terms[ABHYASA].text, root, "{root}");
            assert!(p.log.is_empty(), "{root}");
        }
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Bf"), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "");
        assert!(p.log.is_empty());
    }

    #[test]
    fn haladih_shesha_elides_every_consonant_but_the_first() {
        // 7.4.60, widened in slice 3d: the abhyāsa keeps its first hal and
        // loses every other one, final ones included. 7.4.66's `ar` is the
        // corpus witness (Bar → Ba, bibharti). `Bas` is √bhas's (slice 3f)
        // shape, a final consonant 7.4.66 did not put there; the rule must
        // not depend on where the consonant came from.
        let r_60 = rules().find(|r| r.id == "7.4.60").unwrap();
        for (abhyasa, want) in [
            ("Bar", "Ba"),
            ("par", "pa"),
            ("Gar", "Ga"),
            ("har", "ha"),
            ("sar", "sa"),
            ("Bas", "Ba"),
        ] {
            let mut p = slu_prakriya("Bf", "ti");
            p.terms[ABHYASA].text = abhyasa.into();
            assert!((r_60.apply)(&mut p), "{abhyasa}");
            assert_eq!(p.terms[ABHYASA].text, want, "{abhyasa}");
            assert_eq!(p.log.last().unwrap().sutra, "7.4.60");
        }
    }

    #[test]
    fn ur_at_runs_before_haladih_shesha() {
        // √bhṛ through the first three rules of this stage: Bf → Bf Bf →
        // Bar Bf → Ba Bf. The order is vidyut's (7.4.66 < 7.4.60), and it is
        // load-bearing: 7.4.60 first would find `Bf`, trim nothing, and leave
        // 7.4.66's `r` for good.
        let mut p = slu_prakriya("Bf", "ti");
        for id in ["6.1.10", "7.4.66", "7.4.60"] {
            let r = rules().find(|r| r.id == id).unwrap();
            assert!((r.apply)(&mut p), "{id}");
        }
        assert_eq!(p.terms[ABHYASA].text, "Ba");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, vec!["6.1.10", "7.4.66", "7.4.60"]);
    }
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya -- ur_at haladih_shesha_elides 2>&1 | tail -20`
Expected: FAIL.
- The three `ur_at` tests panic on `unwrap()`, because there is no rule `7.4.66`.
- `haladih_shesha_elides_every_consonant_but_the_first` fails its first `assert!`: HEAD's 7.4.60 computes `Bar` unchanged and declines.

- [ ] **Step 3: Add 7.4.66**

In `ABHYASA_RULES`, immediately after the 6.1.10 `Rule { … },` and before the `// 7.4.60 halādiḥ śeṣaḥ` comment, add:

```rust
    // 7.4.66 ur at: the abhyāsa's ṛ-vowel becomes `a` — `ar` by 1.1.51
    // uraṇ raparaḥ, which (like guṇa's `ar`) is not credited. ṛ names ṝ too
    // (1.1.9's savarṇa): Bf → Bar, pF → par. 7.4.60 then elides the `r`.
    //
    // FIRST after 6.1.10, as vidyut orders it (7.4.66 < 7.4.60). It fires on
    // EVERY cell of a ṛ-root here, where vidyut's fires only on the kṅit
    // cells: vidyut copies the guṇated stem (`Bar` on bibharti) and finds no
    // ṛ to change, while this stage copies the bare root (the header's
    // dvitva-before-guṇa note). The forms agree; the trace pins hold this
    // order.
    //
    // Guarded on the sound, not the row: the sūtra names a vowel, and no
    // abhyāsa before slice 3d holds one. The no-op test is the whole guard —
    // an abhyāsa without `f`/`F`, and the empty slot of every other gaṇa,
    // comes back unchanged.
    Rule {
        id: "7.4.66",
        name: "urat",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let t = p.terms[ABHYASA].text.replace(['f', 'F'], "ar");
            if t == p.terms[ABHYASA].text {
                return false;
            }
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("7.4.66", "urat", before);
            true
        },
    },
```

- [ ] **Step 4: Widen 7.4.60**

Replace 7.4.60's comment block (from `// 7.4.60 halādiḥ śeṣaḥ: of the abhyāsa's initial consonant cluster only` through `// every √hu, √ki and √bhī trace grows a step and the 3924 priors break.`) with:

```rust
    // 7.4.60 halādiḥ śeṣaḥ: of the abhyāsa's consonants only the first
    // remains. Two witnesses: √hrī's initial cluster (hrI → hI, which 7.4.59
    // then shortens to hi and 7.4.62 palatalizes to Ji: jihreti), and — new
    // in slice 3d — the `r` 7.4.66 leaves on every ṛ-root's abhyāsa (Bar →
    // Ba: bibharti, pa: piparti, Ga: jagharti).
    //
    // WIDENED in slice 3d from an initial-cluster trim to the sūtra: keep the
    // first consonant and every vowel, drop every other consonant. For a
    // single-ekāc abhyāsa (6.1.10's NARROW note) that is the whole rule.
    //
    // The no-op guard is 8.4.53's: for a single-consonant open abhyāsa the
    // result equals the input, and the rule must record nothing there or
    // every √hu, √ki and √bhī trace grows a step and the priors break.
```

Replace the body of its `apply` closure with:

```rust
        apply: |p| {
            let s: Vec<char> = p.terms[ABHYASA].text.chars().collect();
            let Some(&first) = s.first() else {
                return false;
            };
            // *halādiḥ* names a consonant-initial abhyāsa. vidyut elides √ṛ's
            // `r` too (ar → a on 03.0017, its 7.4.60), so whether this
            // fall-through survives is slice 3d2's to decide with that
            // witness; no curated row reaches it today.
            if is_vowel(first) {
                return false;
            }
            let t: String = std::iter::once(first)
                .chain(s[1..].iter().copied().filter(|c| is_vowel(*c)))
                .collect();
            if t == p.terms[ABHYASA].text {
                return false;
            }
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("7.4.60", "halAdiH SezaH", before);
            true
        },
```

- [ ] **Step 5: Correct the comments this makes false**

In `abhyasa.rs`'s tests:
- In `haladih_shesha_keeps_only_the_first_consonant_of_the_abhyasa`, replace the comment's last two sentences (`This is the` / `ONLY cluster-initial row in the whole gaṇa, so this test is the` / `rule's only witness and no later slice adds a second.`) with `This is the only cluster-initial row in the whole gaṇa; slice 3d's ṛ-roots witness the rule's other arm, a non-initial consonant after the vowel.`
- In `haladih_shesha_declines_for_a_vowel_initial_abhyasa`, change `// 3d's √ṛ is the vowel-initial row.` to `// √ṛ (slice 3d2) is the vowel-initial row, and vidyut does trim its abhyāsa (ar → a); 3d2 revisits this guard.`

Update the module doc's first line: `//! Reduplication: 6.1.10, 7.4.60, 7.4.59, 7.4.62, 7.4.76, 7.4.78 — dvitva and the` → `//! Reduplication: 6.1.10, 7.4.66, 7.4.60, 7.4.59, 7.4.62, 7.4.76, 7.4.78 — dvitva and the`. Task 2 inserts 7.4.77.

In the module header's `DVITVA RUNS BEFORE GUṆA.` paragraph, after `fires only on the long-vowel roots it names.`, add: `The same choice makes 7.4.66 *ur at* (slice 3d) fire on every cell of a ṛ-root, where vidyut's fires only on the kṅit ones.`

- [ ] **Step 6: Update the pinned order**

In `derivation_tests.rs`'s `tinanta_rule_order_is_pinned`, change `"6.1.10", "7.4.60",` to `"6.1.10", "7.4.66", "7.4.60",`. `mise run fmt` re-wraps the array.

- [ ] **Step 7: Run the tests, then the full suite**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`, then `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS. The 3924 priors are unchanged, because no curated abhyāsa holds a ṛ-vowel or a non-initial consonant after its vowel.

- [ ] **Step 8: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/abhyasa.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(abhyasa): 7.4.66 urat, and 7.4.60 widened to every non-initial consonant"
```

---

## Task 2: 7.4.76 gains √bhṛ, and 7.4.77 *arti-pipartyoś ca*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/abhyasa.rs` (7.4.76's guard and comment; a rule after 7.4.76; tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (the pinned order)

**Interfaces:**
- Consumes: Task 1's 7.4.66 and widened 7.4.60; `slu_prakriya`; `ABHYASA_RULES`.
- Produces: rule `"7.4.77"`, placed immediately after `"7.4.76"`. Test helper `run_abhyasa_stage(root: &str, number: &'static str) -> Prakriya`, used only in this task.

- [ ] **Step 1: Write the failing tests**

In `bhrnam_it_makes_the_abhyasa_vowel_i_for_the_two_rows_it_names`, rename it to `bhrnam_it_makes_the_abhyasa_vowel_i_for_the_three_rows_it_names`. Change its comment's first line to `// 7.4.76. √bhṛñ (03.0006), √māṅ (03.0007) and √ohāṅ (03.0008), after` / `// 7.4.60, 7.4.59 and 7.4.62: Ba → Bi (bibharti), ma → mi (mimIte), Ja → Ji (jihIte).`. Make its loop array:

```rust
        for (root, number, abhyasa, want) in [
            ("Bf", "03.0006", "Ba", "Bi"),
            ("mA", "03.0007", "ma", "mi"),
            ("hA", "03.0008", "Ja", "Ji"),
        ]
```

At the end of `mod tests`, add:

```rust
    #[test]
    fn arti_pipartyos_ca_makes_the_abhyasa_vowel_i_for_pr_and_prr() {
        // 7.4.77, *piparti*: vidyut applies it to both pf\ (03.0005) and pF
        // (03.0004). After 7.4.66 and 7.4.60: pa → pi.
        let rule = rules().find(|r| r.id == "7.4.77").unwrap();
        for (root, number) in [("pf", "03.0005"), ("pF", "03.0004")] {
            let mut p = slu_prakriya(root, "ti");
            p.ctx.dhatupatha = number;
            p.terms[ABHYASA].text = "pa".into();
            assert!((rule.apply)(&mut p), "{number}");
            assert_eq!(p.terms[ABHYASA].text, "pi", "{number}");
            assert_eq!(p.log.last().unwrap().sutra, "7.4.77");
        }
    }

    #[test]
    fn arti_pipartyos_ca_declines_off_its_rows() {
        // Keyed by number. √bhṛ's `i` is 7.4.76's, √sṛ keeps its `a`
        // (sasarti), and the hand-built default names no row. √ṛ (03.0017)
        // is the sūtra's other root and arrives with its witness in 3d2.
        let rule = rules().find(|r| r.id == "7.4.77").unwrap();
        for (root, number, abhyasa) in [
            ("Bf", "03.0006", "Ba"),
            ("sf", "03.0018", "sa"),
            ("pf", "", "pa"),
        ] {
            let mut p = slu_prakriya(root, "ti");
            p.ctx.dhatupatha = number;
            p.terms[ABHYASA].text = abhyasa.into();
            assert!(!(rule.apply)(&mut p), "{number:?}");
            assert_eq!(p.terms[ABHYASA].text, abhyasa, "{number:?}");
            assert!(p.log.is_empty(), "{number:?}");
        }
    }

    /// The whole abhyāsa stage, in pipeline order, on one ślu'd root.
    fn run_abhyasa_stage(root: &str, number: &'static str) -> Prakriya {
        let mut p = slu_prakriya(root, "ti");
        p.ctx.dhatupatha = number;
        for r in ABHYASA_RULES {
            (r.apply)(&mut p);
        }
        p
    }

    #[test]
    fn the_r_roots_reach_their_abhyasa_through_ur_at_then_haladih_shesha() {
        // Each of 3d's six rows through the whole stage: exactly the rules
        // named, in this order, and the abhyāsa 8.4.54 later finishes
        // (Bi → bi, Ja → ja).
        for (root, number, want, ids) in [
            ("Bf", "03.0006", "Bi", vec!["6.1.10", "7.4.66", "7.4.60", "7.4.76"]),
            ("pf", "03.0005", "pi", vec!["6.1.10", "7.4.66", "7.4.60", "7.4.77"]),
            ("pF", "03.0004", "pi", vec!["6.1.10", "7.4.66", "7.4.60", "7.4.77"]),
            ("Gf", "03.0015", "Ja", vec!["6.1.10", "7.4.66", "7.4.60", "7.4.62"]),
            ("hf", "03.0016", "Ja", vec!["6.1.10", "7.4.66", "7.4.60", "7.4.62"]),
            ("sf", "03.0018", "sa", vec!["6.1.10", "7.4.66", "7.4.60"]),
        ] {
            let p = run_abhyasa_stage(root, number);
            assert_eq!(p.terms[ABHYASA].text, want, "{number}");
            assert_eq!(p.terms[ANGA].text, root, "{number}");
            let got: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(got, ids, "{number}");
        }
    }
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya -- bhrnam_it arti_pipartyos the_r_roots 2>&1 | tail -30`
Expected: FAIL.
- `bhrnam_it_…three_rows…` fails on `03.0006`.
- The two `arti_pipartyos` tests panic on `unwrap()`.
- `the_r_roots_…` fails on `03.0006` (no 7.4.76) and on `03.0005` (no 7.4.77).

- [ ] **Step 3: Widen 7.4.76**

Change its guard to:

```rust
            if !matches!(p.ctx.dhatupatha, "03.0006" | "03.0007" | "03.0008") {
                return false;
            }
```

In its comment, replace everything from `// KEYED BY ROW NUMBER` through `// witness; an unwitnessed number here would be a mutation survivor.` with:

```rust
    // KEYED BY ROW NUMBER (`ctx.dhatupatha`), not by `ANGA.text`: the sūtra
    // names three roots, and `03.0009 o~hA\k` enters the derivation as `hA`
    // exactly like `03.0008 o~hA\N` while taking no 7.4.76 (jahAti).
    //   03.0006 quBf\Y √bhṛñ (slice 3d: Ba → Bi, bibharti)
    //   03.0007 mA\N   √māṅ
    //   03.0008 o~hA\N √ohāṅ
```

Also change `// ma → mi (mimIte), and — after 7.4.59 and 7.4.62 — Ja → Ji (jihIte).` to `// Ba → Bi (bibharti, after 7.4.66 and 7.4.60), ma → mi (mimIte), and — after 7.4.59 and 7.4.62 — Ja → Ji (jihIte).`

- [ ] **Step 4: Add 7.4.77**

Immediately after the 7.4.76 `Rule { … },` and before the `// 7.4.78 bahulaṁ chandasi` comment, add:

```rust
    // 7.4.77 arti-pipartyoś ca: the abhyāsa of √ṛ (*arti*) and √pṛ
    // (*piparti*) takes `i` too. vidyut applies *piparti* to both pf\ and pF,
    // the two rows that spell it: after 7.4.66 and 7.4.60, pa → pi.
    //
    // KEYED BY ROW NUMBER, like 7.4.76 above it: by this stage `ANGA.text`
    // still reads `pf`/`pF`, but the sūtra names roots, and the precedent is
    // the number.
    //   03.0004 pF  √pṝ
    //   03.0005 pf\ √pṛ
    // `03.0017 f\` (√ṛ, *arti*) is the sūtra's first root and joins with its
    // witness in slice 3d2; an unwitnessed number here would be a mutation
    // survivor.
    //
    // After 7.4.76: the two name disjoint roots, so only the trace pins decide
    // the order, and they follow vidyut's.
    Rule {
        id: "7.4.77",
        name: "artipipartyoSca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !matches!(p.ctx.dhatupatha, "03.0004" | "03.0005") {
                return false;
            }
            let t: String = p.terms[ABHYASA]
                .text
                .chars()
                .map(|c| if is_vowel(c) { 'i' } else { c })
                .collect();
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("7.4.77", "artipipartyoSca", before);
            true
        },
    },
```

The module doc's first line becomes `//! Reduplication: 6.1.10, 7.4.66, 7.4.60, 7.4.59, 7.4.62, 7.4.76, 7.4.77, 7.4.78 — dvitva and the`.

- [ ] **Step 5: Update the pinned order**

In `tinanta_rule_order_is_pinned`, change `"7.4.76", "7.4.78",` to `"7.4.76", "7.4.77", "7.4.78",`.

- [ ] **Step 6: Run the tests, then the full suite**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`, then `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS, with the 3924 priors unchanged; no curated row is `03.0004`, `03.0005` or `03.0006`.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/abhyasa.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(abhyasa): 7.4.77 artipipartyoSca for √pṛ/√pṝ, and 7.4.76 gains √bhṛñ"
```

---

## Task 3: 7.1.102 *ud oṣṭhyapūrvasya* and 6.1.77's aṅga arm

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (a rule after the second 7.3.84; 6.1.77's body and comment; tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (the pinned order)

**Interfaces:**
- Consumes: `abhyasta_prakriya(abhyasa, anga, ghu, ending, ngit)` in `guna.rs`'s tests; `is_vowel`, `Tag`, `SHAP`, `ENDING` (already imported).
- Produces: rule `"7.1.102"`, placed immediately after the second `"7.3.84"` and before `"6.4.110"`. That puts it ahead of 6.1.77, which is load-bearing: *pipurati* needs `pF` → `pur` before 6.1.77 could see a ṛ-final aṅga.

- [ ] **Step 1: Write the failing tests**

At the end of `guna.rs`'s `mod tests`, add:

```rust
    // --- 7.1.102 ud oṣṭhyapūrvasya ------------------------------------------

    #[test]
    fn ud_oshthyapurvasya_makes_a_labial_final_rr_ur() {
        // √pṝ before a kṅit ending, where guṇa declined: pF → pur
        // (pipUrtaH once 8.2.77 lengthens it). `vF` holds the sūtra's `v`,
        // which *oṣṭhya* includes (vidyut: pu-varga + v) and no curated root
        // witnesses.
        let rule = rules().find(|r| r.id == "7.1.102").unwrap();
        for (anga, want) in [("pF", "pur"), ("vF", "vur")] {
            let mut p = abhyasta_prakriya("pi", anga, false, "tas", true);
            assert!((rule.apply)(&mut p), "{anga}");
            assert_eq!(p.terms[ANGA].text, want, "{anga}");
            assert_eq!(p.log.last().unwrap().sutra, "7.1.102");
        }
    }

    #[test]
    fn ud_oshthyapurvasya_declines_on_short_r_a_non_labial_and_after_guna() {
        // `pf` is √pṛ's short ṛ (pipftaH keeps it); `tF` has a non-labial
        // before the ṝ (7.1.100's shape, not this rule's); `par` is what 7.3.84
        // leaves on a pit cell (piparti); a bare `F` has no preceding sound.
        let rule = rules().find(|r| r.id == "7.1.102").unwrap();
        for anga in ["pf", "tF", "par", "F"] {
            let mut p = abhyasta_prakriya("pi", anga, false, "tas", true);
            assert!(!(rule.apply)(&mut p), "{anga}");
            assert_eq!(p.terms[ANGA].text, anga, "{anga}");
            assert!(p.log.is_empty(), "{anga}");
        }
    }

    // --- 6.1.77 iko yaṇ aci: the aṅga arm ------------------------------------

    #[test]
    fn iko_yan_aci_anga_arm_turns_a_final_r_to_r_before_a_vowel() {
        // √bhṛ under ślu: Bf + ati → Br + ati (bibhrati), and every
        // vowel-initial ending the corpus gives it (bibhrAte, bibhrate,
        // bibhrIta, abibhri, bibhre).
        let rule = rules().find(|r| r.id == "6.1.77").unwrap();
        for ending in ["ati", "Ate", "ate", "Ita", "i", "e"] {
            let mut p = abhyasta_prakriya("Bi", "Bf", false, ending, true);
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ANGA].text, "Br", "{ending}");
            assert_eq!(p.terms[SHAP].text, "", "{ending}");
            assert_eq!(p.log.last().unwrap().sutra, "6.1.77");
        }
    }

    #[test]
    fn iko_yan_aci_anga_arm_declines_where_the_ending_or_the_anga_is_wrong() {
        let rule = rules().find(|r| r.id == "6.1.77").unwrap();
        // A consonant-initial ending: bibhftaH.
        let mut p = abhyasta_prakriya("Bi", "Bf", false, "tas", true);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Bf");
        // A guṇated aṅga before a pit vowel: bibharARi.
        let mut p = abhyasta_prakriya("Bi", "Bar", false, "Ani", false);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Bar");
        // A long ṝ: 7.1.102 has already taken √pṝ to `pur` (pipurati); the
        // arm is the short ṛ's only.
        let mut p = abhyasta_prakriya("pi", "pF", false, "ati", true);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "pF");
        assert!(p.log.is_empty());
    }

    #[test]
    fn iko_yan_aci_anga_arm_declines_across_a_live_vikarana() {
        // SHAP non-empty: the ending does not meet the aṅga. Hand-built (no
        // curated ṛ-final aṅga keeps its ṛ in front of a vikaraṇa); this is
        // the test that holds the arm's SHAP-empty clause.
        let rule = rules().find(|r| r.id == "6.1.77").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Bf"), Term::new("nA"), Term::new("anti")]),
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Bf");
        assert_eq!(p.terms[SHAP].text, "nA");
        assert!(p.log.is_empty());
    }

    #[test]
    fn iko_yan_aci_vikarana_arm_is_unchanged_by_the_restructure() {
        // tanādi: tan + u + anti → tan + v + anti (tanvanti). An untagged `u`
        // and a consonant-initial ending both decline.
        let rule = rules().find(|r| r.id == "6.1.77").unwrap();
        let tan = |ending: &str, tagged: bool| {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new("tan"), Term::new("u"), Term::new(ending)]),
                ..Default::default()
            };
            if tagged {
                p.terms[SHAP].add(Tag::Vikarana);
            }
            p
        };
        let mut p = tan("anti", true);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "v");
        assert_eq!(p.terms[ANGA].text, "tan");
        for (ending, tagged) in [("anti", false), ("tas", true)] {
            let mut p = tan(ending, tagged);
            assert!(!(rule.apply)(&mut p), "{ending} {tagged}");
            assert_eq!(p.terms[SHAP].text, "u");
        }
    }
```

If `Prakriya`, `Term` or `with_slots` are not already in scope in `guna.rs`'s `mod tests`, add them to its `use` lines, the way `abhyasa.rs`'s tests import them. Several existing `guna.rs` tests already build `Prakriya { terms: with_slots(…) }`, so they should be.

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya -- ud_oshthyapurvasya iko_yan_aci 2>&1 | tail -30`
Expected: FAIL.
- The two `ud_oshthyapurvasya` tests panic on `unwrap()`.
- `iko_yan_aci_anga_arm_turns_…` fails, because HEAD's 6.1.77 has no aṅga arm.
- The three decline/regression tests PASS already; that is expected, and they hold the new clauses from Step 4 on.

- [ ] **Step 3: Add 7.1.102**

In `GUNA_RULES`, immediately after the second 7.3.84 `Rule { … },` (the one reading `p.terms[SHAP]`) and before the `// ------` line that opens the `The √kṛ specials, 6.4.108–110.` block comment, add:

```rust
    // 7.1.102 ud oṣṭhyapūrvasya: a dhātu-final ṝ after a labial becomes `u`
    // — `ur` by 1.1.51, uncredited as guṇa's `ar` is. √pṝ: pF → pur before a
    // kṅit ending, which 8.2.77 then lengthens before a consonant
    // (pipUrtaH, pipUryAt) and leaves short before a vowel (pipurati).
    //
    // Runs where guṇa declined, and only there, by ORDER, not by guard:
    // it sits after both 7.3.84 applications, so on a pit cell the aṅga is
    // already `par` (piparti) and the `F` test declines. vidyut runs it at the
    // same point ("only when a pratyaya has tried and failed to apply guna").
    // BEFORE 6.1.77 below, whose aṅga arm must never see a ṛ-final aṅga that
    // this rule was about to change.
    //
    // vidyut runs it before 6.1.10 on the kṅit cells, so its abhyāsa is copied
    // from `pur`. This engine copies `pF` and reaches the same `pi-` by 7.4.66,
    // 7.4.60 and 7.4.77. The forms agree; the trace pins hold this order.
    //
    // *oṣṭhya* is the pu-varga plus `v` (vidyut's `OSHTHYA`). Only `p` is
    // witnessed in the corpus; the guard test holds `v` and a non-labial.
    Rule {
        id: "7.1.102",
        name: "udozWyapUrvasya",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let Some(stem) = p.terms[ANGA].text.strip_suffix('F') else {
                return false;
            };
            if !stem
                .chars()
                .last()
                .is_some_and(|c| matches!(c, 'p' | 'P' | 'b' | 'B' | 'm' | 'v'))
            {
                return false;
            }
            let t = format!("{stem}ur");
            let before = p.snapshot();
            p.terms[ANGA].text = t;
            p.record("7.1.102", "udozWyapUrvasya", before);
            true
        },
    },
```

- [ ] **Step 4: Give 6.1.77 its aṅga arm**

Replace 6.1.77's comment paragraph `// Only the vikaraṇa arm is written: no other ik-vowel hiatus survives` … `// witness, when a root needs one.` with:

```rust
    // Two arms. The VIKARAṆA arm (tanādi 8a) is the one described above. The
    // AṄGA arm (juhotyādi 3d) takes an aṅga-final `f` to `r` when ślu has
    // left SHAP empty and the ending is vowel-initial: Bf + ati → Br + ati
    // (bibhrati, bibhrAte, bibhrIta; piprati, jaghrati, jahrati, sasrati).
    // The SHAP-empty clause is what keeps it off every thematic ṛ-root. The
    // only other empty-SHAP paths are adādi's luk (`yA vA ad As vas SI`, none
    // `f`-final) and nothing else. Pit vowel-initial endings (loṭ uttama,
    // ātmanepada *ai*) and laṅ's *jus* are guṇated first by 7.3.84 / 7.3.83,
    // so the aṅga ends in `r` there and the `f` test declines (bibharARi,
    // abibharuH). The long `F` is excluded: 7.1.102 above has already taken
    // √pṝ to `pur` wherever guṇa declined (pipurati). Other ik-vowel hiatuses
    // still have no arm here; widen by arm, with a witness.
```

Replace its `apply` closure with:

```rust
        apply: |p| {
            let Some(next) = p.terms.get(ENDING).and_then(|t| t.text.chars().next()) else {
                return false;
            };
            if !is_vowel(next) {
                return false;
            }
            // Vikaraṇa arm: tanādi's bare `u` (3.1.79).
            if p.terms[SHAP].text == "u" && p.terms[SHAP].has(Tag::Vikarana) {
                let before = p.snapshot();
                p.terms[SHAP].text = "v".into();
                p.record("6.1.77", "iko yaR aci", before);
                return true;
            }
            // Aṅga arm: a ṛ-final aṅga with the ending directly after it (ślu).
            if p.terms[SHAP].text.is_empty()
                && let Some(stem) = p.terms[ANGA].text.strip_suffix('f')
            {
                let t = format!("{stem}r");
                let before = p.snapshot();
                p.terms[ANGA].text = t;
                p.record("6.1.77", "iko yaR aci", before);
                return true;
            }
            false
        },
```

Change the first line of 6.1.77's comment from `// 6.1.77 iko yaṇ aci: the tanādi vikaraṇa's `u` becomes `v` before a` to `// 6.1.77 iko yaṇ aci: an ik before a vowel becomes its yaṇ. The tanādi vikaraṇa's `u` becomes `v` before a`. `mise run fmt` does not reflow comments, so rewrap that paragraph by hand to the file's width.

- [ ] **Step 5: Update the pinned order**

In `tinanta_rule_order_is_pinned`, the second `"7.3.84"` is the one followed by `"6.4.110"`. Change `"7.3.84", "6.4.110",` to `"7.3.84", "7.1.102", "6.4.110",`. Check that exactly one occurrence changed.

- [ ] **Step 6: Run the tests, then the full suite**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`, then `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS, with the 3924 priors unchanged.
- No curated aṅga ends in `F`.
- The only curated `f`-final aṅgas are bhvādi `smf`, kryādi `vf` and tanādi `kf`, and all three carry a non-empty SHAP.
- The tanādi goldens re-witness the vikaraṇa arm.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/guna.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(guna): 7.1.102 udozWyapUrvasya, and 6.1.77 gains its aṅga arm"
```

---

## Task 4: 8.2.77 *hali ca* reads the ending when ślu leaves SHAP empty

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (8.2.77's follower read and comment; tests)

**Interfaces:**
- Consumes: `ANGA`, `SHAP`, `ENDING`, `with_slots`, `is_vowel` (already imported in `tripadi.rs` and its tests).
- Produces: no new symbol.

- [ ] **Step 1: Write the failing tests**

In `tripadi.rs`'s `mod tests`, immediately after `hali_ca_still_lengthens_the_divyati_shaped_root`, add:

```rust
    #[test]
    fn hali_ca_reads_the_ending_when_slu_leaves_shap_empty() {
        // √pṝ (03.0004) after 7.1.102: pur + "" + tas. With śap ślu'd the
        // ending is what meets the aṅga, so its `t` is the hal: pUr
        // (pipUrtaH). Before a vowel-initial ending the rule declines
        // (pipurati).
        let rule = rules().find(|r| r.id == "8.2.77").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("pur"), Term::new(""), Term::new("tas")]),
            log: vec![],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "pUr");
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("pur"), Term::new(""), Term::new("ati")]),
            log: vec![],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "pur");
    }

    #[test]
    fn hali_ca_reads_a_live_vikarana_not_the_ending() {
        // A non-empty, vowel-initial SHAP in front of a consonant-initial
        // ending: the aṅga meets the vikaraṇa's vowel and must not lengthen.
        // Hand-built — no curated r/v-final aṅga carries śap — and it is what
        // kills a mutant that reads the ending whenever SHAP is present.
        let rule = rules().find(|r| r.id == "8.2.77").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("div"), Term::new("a"), Term::new("ti")]),
            log: vec![],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "div");
    }
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya -- hali_ca 2>&1 | tail -20`
Expected: `hali_ca_reads_the_ending_when_slu_leaves_shap_empty` FAILS at its first `assert!`, because HEAD reads SHAP's empty text and declines. `hali_ca_reads_a_live_vikarana_not_the_ending` passes already; it holds Step 3's `!is_empty()` clause.

- [ ] **Step 3: Widen the follower read**

In 8.2.77's `apply`, replace this block:

```rust
            // Reads śap as "the segment following the aṅga"; when śap is luk'd
            // (adādi, 2.4.72) that is empty and the rule silently declines.
            // Currently unreachable (no r/v-final adādi root in scope); when a
            // consonant-final r/v-upadhā adādi root lands, this must generalize
            // to the root+ending junction — 6.1.78's athematic arm (added in
            // slice 5f for √śī, which falls back to `p.terms[ENDING]` when
            // SHAP is empty) is the worked example to follow.
            let Some(next) = p.terms.get(SHAP).and_then(|t| t.text.chars().next()) else {
                return false;
            };
```

with:

```rust
            // The segment the aṅga meets: śap when it has text, else the
            // ending. Juhotyādi's ślu (2.4.75) empties SHAP, so √pṝ's `pur`
            // meets the ending directly (pipUrtaH) — generalized in slice 3d on
            // 6.1.78's athematic arm, which falls back to `p.terms[ENDING]` the
            // same way. Adādi's luk (2.4.72) takes the same path, but no curated
            // adādi aṅga ends in r/v after i/u. Open-coded rather than calling
            // `following_sarvadhatuka`, as the follower lookups in this crate
            // are, so each keeps its own mutation pin.
            let follower = match p.terms.get(SHAP) {
                Some(t) if !t.text.is_empty() => Some(t),
                Some(_) => p.terms.get(ENDING),
                None => None,
            };
            let Some(next) = follower.and_then(|t| t.text.chars().next()) else {
                return false;
            };
```

- [ ] **Step 4: Run the tests, then the full suite**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`, then `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS, with the 3924 priors unchanged. The only newly reachable shape is an r/v-final aṅga with an i/u upadhā and an empty SHAP, and no curated root has one; adādi's six are `yA vA ad As vas SI`.

- [ ] **Step 5: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/tripadi.rs
git commit -m "feat(tripadi): 8.2.77 hali ca reads the ending when ślu empties SHAP"
```

---

## Task 5: The six rows and their paradigm goldens

This task turns the 252 new cells green.

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (six `Dhatu` rows; the gaṇa-row test; doc-comment counts)
- Modify: `crates/panini/tests/paradigm/data/juhotyadi.rs` (28 `PARADIGM` blocks, 30 `ALTERNATES` rows)
- Modify: `crates/panini/tests/paradigm/main.rs` (totals, buckets, key census, the pada-ambiguous list, a homograph test, prose)

**Interfaces:**
- Consumes: Tasks 1–4.
- Produces: `dhatus()` rows `03.0004` `pF`, `03.0005` `pf`, `03.0006` `Bf` (`Ubhayapada`), `03.0015` `Gf`, `03.0016` `hf` and `03.0018` `sf`, all `Gana::Juhotyadi`.

- [ ] **Step 1: Add the six `Dhatu` rows**

`DHATUS` is ordered by number. Insert `03.0004`, `03.0005` and `03.0006` after the `03.0003` row, and `03.0015`, `03.0016` and `03.0018` after the `03.0011` row. The arthas are upstream's verbatim (`data/dhatupatha.tsv` lines 1268–1270, 1279, 1280, 1282); `dhatupatha_numbers_resolve_upstream` holds them.

```rust
    Dhatu {
        // 03.0004 `pF` pAlanapUraRayoH (√pṝ). Parasmaipadī by 1.3.78 (no accent
        // mark at all). Its abhyāsa takes `i` by 7.4.77, keyed by this number
        // (piparti). 7.1.102 makes its labial-preceded ṝ `ur` wherever guṇa
        // declines (pipurati), and 8.2.77 lengthens that before a consonant
        // (pipUrtaH). Slice 3d.
        dhatupatha: "03.0004",
        code: "pF",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "pAlanapUraRayoH",
    },
    Dhatu {
        // 03.0005 `pf\` pAlanapUraRayoH (√pṛ). Parasmaipadī by 1.3.78 (the `\`
        // is the root vowel's accent). 7.4.66 then 7.4.60 then 7.4.77, keyed by
        // this number: piparti, pipftaH. Shares every guṇated form with
        // 03.0004. Slice 3d.
        dhatupatha: "03.0005",
        code: "pf",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "pAlanapUraRayoH",
    },
    Dhatu {
        // 03.0006 `quBf\Y` DAraRapozaRayoH (√bhṛñ). The `qu` is an it by
        // 1.3.5; ubhayapadī by 1.3.72 (ñit). Its abhyāsa takes `i` by 7.4.76,
        // keyed by this number (bibharti), and 6.1.77's aṅga arm gives
        // bibhrati / bibhrAte. Slice 3d.
        dhatupatha: "03.0006",
        code: "Bf",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "DAraRapozaRayoH",
    },
```

```rust
    Dhatu {
        // 03.0015 `Gf\` kzaraRadIptyoH (√ghṛ). Parasmaipadī by 1.3.78. 7.4.66,
        // 7.4.60, then 7.4.62's gh → jh and 8.4.54's j: jagharti. Slice 3d.
        dhatupatha: "03.0015",
        code: "Gf",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "kzaraRadIptyoH",
    },
    Dhatu {
        // 03.0016 `hf\` prasahyakaraRe (√hṛ). Parasmaipadī by 1.3.78. The same
        // path as √ghṛ, with 7.4.62's h → jh: jaharti. Slice 3d.
        dhatupatha: "03.0016",
        code: "hf",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "prasahyakaraRe",
    },
    Dhatu {
        // 03.0018 `sf\` gatO (√sṛ). Parasmaipadī by 1.3.78. 7.4.66 and 7.4.60
        // alone: sasarti. vidyut also credits 8.3.110 on sasrati, a bar on a
        // ṣatva that 8.3.59 cannot reach here, so it is not transcribed (see
        // the 3d spec). Slice 3d.
        dhatupatha: "03.0018",
        code: "sf",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "gatO",
    },
```

In the `Dhatu` struct's doc comments:
- `re-derives 86 of these 87` → `re-derives 92 of these 93`
- `The test covers the 87 roots curated here` → `The test covers the 93 roots curated here`

In `pada_from_upadesha`'s doc comment: `57 of the 87 curated roots carry a \`\\\` at all, and 36` → `62 of the 93 curated roots carry a \`\\\` at all, and 41`. Five of the six new upadeśas carry exactly one `\`, on the root vowel; `pF` carries none.

In `curated_roots_have_expected_ganas_and_padas`, change `assert_eq!(dhatus().len(), 87);` to `93`.

- [ ] **Step 2: Extend the gaṇa-row test**

Rename `juhotyadi_rows_are_the_ten_curated_roots` to `juhotyadi_rows_are_the_sixteen_curated_roots`. Update every reference to the old name; `grep -rn "juhotyadi_rows_are_the_ten" .` finds them (one is at `lib.rs` around line 1814). In its comment, replace these three lines:

```rust
        // Slice 3c2 adds √hā parasmaipada (03.0009, jahāti) and √gā
        // (03.0026), both parasmaipadī by 1.3.78. The gaṇa is PARTIAL at 10
        // of its 26 dhātupāṭha rows; slices 3d–3f close it.
```

with:

```rust
        // Slice 3c2 adds √hā parasmaipada (03.0009, jahāti) and √gā
        // (03.0026), both parasmaipadī by 1.3.78. Slice 3d adds the six
        // consonant-initial ṛ-roots, √pṝ, √pṛ, √bhṛ (ubhayapadī by 1.3.72,
        // ñit), √ghṛ, √hṛ and √sṛ. The gaṇa is PARTIAL at 16 of its 26
        // dhātupāṭha rows; slices 3d2, 3e and 3f close it.
```

Its expected vector becomes:

```rust
            vec![
                ("03.0001", "hu", PadaAssignment::Parasmaipada),
                ("03.0002", "BI", PadaAssignment::Parasmaipada),
                ("03.0003", "hrI", PadaAssignment::Parasmaipada),
                ("03.0004", "pF", PadaAssignment::Parasmaipada),
                ("03.0005", "pf", PadaAssignment::Parasmaipada),
                ("03.0006", "Bf", PadaAssignment::Ubhayapada),
                ("03.0007", "mA", PadaAssignment::Atmanepada),
                ("03.0008", "hA", PadaAssignment::Atmanepada),
                ("03.0009", "hA", PadaAssignment::Parasmaipada),
                ("03.0010", "dA", PadaAssignment::Ubhayapada),
                ("03.0011", "DA", PadaAssignment::Ubhayapada),
                ("03.0015", "Gf", PadaAssignment::Parasmaipada),
                ("03.0016", "hf", PadaAssignment::Parasmaipada),
                ("03.0018", "sf", PadaAssignment::Parasmaipada),
                ("03.0020", "ki", PadaAssignment::Parasmaipada),
                ("03.0026", "gA", PadaAssignment::Parasmaipada),
            ]
```

Replace its final comment (`// Every root-specific rule of slices 3c and 3c2 (7.4.76, 7.4.78,` … `// 03.0009, which is exactly why.`) with:

```rust
        // Every root-specific rule of slices 3c, 3c2 and 3d (7.4.76, 7.4.77,
        // 7.4.78, 8.2.38, 8.2.40's adhaḥ, 6.4.116–6.4.118) and 1.1.20's
        // Tag::Ghu key on the dhātupāṭha NUMBER, so `hA`, `dA`, `DA`, `gA`,
        // `pf`, `pF` and `Bf` need no uniqueness tripwire. `hA` is held by two
        // rows, 03.0008 and 03.0009, which is exactly why. Slice 3d's 7.4.66
        // and 7.1.102 name sounds (a ṛ-vowel; a labial before ṝ), not roots.
```

- [ ] **Step 3: Run the data tests**

Run: `mise exec -- cargo test -p panini-data 2>&1 | tail -20`
Expected: PASS, including `curated_pada_agrees_with_upadesha_markers` and `dhatupatha_numbers_resolve_upstream`.

- [ ] **Step 4: Add the `PARADIGM` blocks**

`PARADIGM` in `crates/panini/tests/paradigm/data/juhotyadi.rs` is ordered by the slice that added each block, not by number. **Append** all 28 blocks to the end of `PARADIGM`, in this order. Index 0 of each multi-form cell is the declined derivation (`-tu`, `-hi`, `-yAd`). `mise run fmt` re-wraps the arrays.

```rust
    ("03.0006", "laT", Pada::Parasmaipada, ["biBarti", "biBftaH", "biBrati", "biBarzi", "biBfTaH", "biBfTa", "biBarmi", "biBfvaH", "biBfmaH"]),
    ("03.0006", "laN", Pada::Parasmaipada, ["abiBaH", "abiBftAm", "abiBaruH", "abiBaH", "abiBftam", "abiBfta", "abiBaram", "abiBfva", "abiBfma"]),
    ("03.0006", "loT", Pada::Parasmaipada, ["biBartu", "biBftAm", "biBratu", "biBfhi", "biBftam", "biBfta", "biBarARi", "biBarAva", "biBarAma"]),
    ("03.0006", "viDiliN", Pada::Parasmaipada, ["biBfyAd", "biBfyAtAm", "biBfyuH", "biBfyAH", "biBfyAtam", "biBfyAta", "biBfyAm", "biBfyAva", "biBfyAma"]),
    ("03.0006", "laT", Pada::Atmanepada, ["biBfte", "biBrAte", "biBrate", "biBfze", "biBrATe", "biBfDve", "biBre", "biBfvahe", "biBfmahe"]),
    ("03.0006", "laN", Pada::Atmanepada, ["abiBfta", "abiBrAtAm", "abiBrata", "abiBfTAH", "abiBrATAm", "abiBfDvam", "abiBri", "abiBfvahi", "abiBfmahi"]),
    ("03.0006", "loT", Pada::Atmanepada, ["biBftAm", "biBrAtAm", "biBratAm", "biBfzva", "biBrATAm", "biBfDvam", "biBarE", "biBarAvahE", "biBarAmahE"]),
    ("03.0006", "viDiliN", Pada::Atmanepada, ["biBrIta", "biBrIyAtAm", "biBrIran", "biBrITAH", "biBrIyATAm", "biBrIDvam", "biBrIya", "biBrIvahi", "biBrImahi"]),
    ("03.0005", "laT", Pada::Parasmaipada, ["piparti", "pipftaH", "piprati", "piparzi", "pipfTaH", "pipfTa", "piparmi", "pipfvaH", "pipfmaH"]),
    ("03.0005", "laN", Pada::Parasmaipada, ["apipaH", "apipftAm", "apiparuH", "apipaH", "apipftam", "apipfta", "apiparam", "apipfva", "apipfma"]),
    ("03.0005", "loT", Pada::Parasmaipada, ["pipartu", "pipftAm", "pipratu", "pipfhi", "pipftam", "pipfta", "piparARi", "piparAva", "piparAma"]),
    ("03.0005", "viDiliN", Pada::Parasmaipada, ["pipfyAd", "pipfyAtAm", "pipfyuH", "pipfyAH", "pipfyAtam", "pipfyAta", "pipfyAm", "pipfyAva", "pipfyAma"]),
    ("03.0004", "laT", Pada::Parasmaipada, ["piparti", "pipUrtaH", "pipurati", "piparzi", "pipUrTaH", "pipUrTa", "piparmi", "pipUrvaH", "pipUrmaH"]),
    ("03.0004", "laN", Pada::Parasmaipada, ["apipaH", "apipUrtAm", "apiparuH", "apipaH", "apipUrtam", "apipUrta", "apiparam", "apipUrva", "apipUrma"]),
    ("03.0004", "loT", Pada::Parasmaipada, ["pipartu", "pipUrtAm", "pipuratu", "pipUrhi", "pipUrtam", "pipUrta", "piparARi", "piparAva", "piparAma"]),
    ("03.0004", "viDiliN", Pada::Parasmaipada, ["pipUryAd", "pipUryAtAm", "pipUryuH", "pipUryAH", "pipUryAtam", "pipUryAta", "pipUryAm", "pipUryAva", "pipUryAma"]),
    ("03.0015", "laT", Pada::Parasmaipada, ["jaGarti", "jaGftaH", "jaGrati", "jaGarzi", "jaGfTaH", "jaGfTa", "jaGarmi", "jaGfvaH", "jaGfmaH"]),
    ("03.0015", "laN", Pada::Parasmaipada, ["ajaGaH", "ajaGftAm", "ajaGaruH", "ajaGaH", "ajaGftam", "ajaGfta", "ajaGaram", "ajaGfva", "ajaGfma"]),
    ("03.0015", "loT", Pada::Parasmaipada, ["jaGartu", "jaGftAm", "jaGratu", "jaGfhi", "jaGftam", "jaGfta", "jaGarARi", "jaGarAva", "jaGarAma"]),
    ("03.0015", "viDiliN", Pada::Parasmaipada, ["jaGfyAd", "jaGfyAtAm", "jaGfyuH", "jaGfyAH", "jaGfyAtam", "jaGfyAta", "jaGfyAm", "jaGfyAva", "jaGfyAma"]),
    ("03.0016", "laT", Pada::Parasmaipada, ["jaharti", "jahftaH", "jahrati", "jaharzi", "jahfTaH", "jahfTa", "jaharmi", "jahfvaH", "jahfmaH"]),
    ("03.0016", "laN", Pada::Parasmaipada, ["ajahaH", "ajahftAm", "ajaharuH", "ajahaH", "ajahftam", "ajahfta", "ajaharam", "ajahfva", "ajahfma"]),
    ("03.0016", "loT", Pada::Parasmaipada, ["jahartu", "jahftAm", "jahratu", "jahfhi", "jahftam", "jahfta", "jaharARi", "jaharAva", "jaharAma"]),
    ("03.0016", "viDiliN", Pada::Parasmaipada, ["jahfyAd", "jahfyAtAm", "jahfyuH", "jahfyAH", "jahfyAtam", "jahfyAta", "jahfyAm", "jahfyAva", "jahfyAma"]),
    ("03.0018", "laT", Pada::Parasmaipada, ["sasarti", "sasftaH", "sasrati", "sasarzi", "sasfTaH", "sasfTa", "sasarmi", "sasfvaH", "sasfmaH"]),
    ("03.0018", "laN", Pada::Parasmaipada, ["asasaH", "asasftAm", "asasaruH", "asasaH", "asasftam", "asasfta", "asasaram", "asasfva", "asasfma"]),
    ("03.0018", "loT", Pada::Parasmaipada, ["sasartu", "sasftAm", "sasratu", "sasfhi", "sasftam", "sasfta", "sasarARi", "sasarAva", "sasarAma"]),
    ("03.0018", "viDiliN", Pada::Parasmaipada, ["sasfyAd", "sasfyAtAm", "sasfyuH", "sasfyAH", "sasfyAtam", "sasfyAta", "sasfyAm", "sasfyAva", "sasfyAma"]),
```

- [ ] **Step 5: Add the `ALTERNATES` rows**

**Append** these to the end of `ALTERNATES` in the same file. The index is the 0-based cell (P.E P.D P.B M.E M.D M.B U.E U.D U.B). The key names the optional rules behind the form, in pipeline order.

```rust
    ("03.0006", "loT", Pada::Parasmaipada, 0, "biBftAd", "7.1.35"),
    ("03.0006", "loT", Pada::Parasmaipada, 0, "biBftAt", "7.1.35+8.4.56"),
    ("03.0006", "loT", Pada::Parasmaipada, 3, "biBftAd", "7.1.35"),
    ("03.0006", "loT", Pada::Parasmaipada, 3, "biBftAt", "7.1.35+8.4.56"),
    ("03.0006", "viDiliN", Pada::Parasmaipada, 0, "biBfyAt", "8.4.56"),
    ("03.0005", "loT", Pada::Parasmaipada, 0, "pipftAd", "7.1.35"),
    ("03.0005", "loT", Pada::Parasmaipada, 0, "pipftAt", "7.1.35+8.4.56"),
    ("03.0005", "loT", Pada::Parasmaipada, 3, "pipftAd", "7.1.35"),
    ("03.0005", "loT", Pada::Parasmaipada, 3, "pipftAt", "7.1.35+8.4.56"),
    ("03.0005", "viDiliN", Pada::Parasmaipada, 0, "pipfyAt", "8.4.56"),
    ("03.0004", "loT", Pada::Parasmaipada, 0, "pipUrtAd", "7.1.35"),
    ("03.0004", "loT", Pada::Parasmaipada, 0, "pipUrtAt", "7.1.35+8.4.56"),
    ("03.0004", "loT", Pada::Parasmaipada, 3, "pipUrtAd", "7.1.35"),
    ("03.0004", "loT", Pada::Parasmaipada, 3, "pipUrtAt", "7.1.35+8.4.56"),
    ("03.0004", "viDiliN", Pada::Parasmaipada, 0, "pipUryAt", "8.4.56"),
    ("03.0015", "loT", Pada::Parasmaipada, 0, "jaGftAd", "7.1.35"),
    ("03.0015", "loT", Pada::Parasmaipada, 0, "jaGftAt", "7.1.35+8.4.56"),
    ("03.0015", "loT", Pada::Parasmaipada, 3, "jaGftAd", "7.1.35"),
    ("03.0015", "loT", Pada::Parasmaipada, 3, "jaGftAt", "7.1.35+8.4.56"),
    ("03.0015", "viDiliN", Pada::Parasmaipada, 0, "jaGfyAt", "8.4.56"),
    ("03.0016", "loT", Pada::Parasmaipada, 0, "jahftAd", "7.1.35"),
    ("03.0016", "loT", Pada::Parasmaipada, 0, "jahftAt", "7.1.35+8.4.56"),
    ("03.0016", "loT", Pada::Parasmaipada, 3, "jahftAd", "7.1.35"),
    ("03.0016", "loT", Pada::Parasmaipada, 3, "jahftAt", "7.1.35+8.4.56"),
    ("03.0016", "viDiliN", Pada::Parasmaipada, 0, "jahfyAt", "8.4.56"),
    ("03.0018", "loT", Pada::Parasmaipada, 0, "sasftAd", "7.1.35"),
    ("03.0018", "loT", Pada::Parasmaipada, 0, "sasftAt", "7.1.35+8.4.56"),
    ("03.0018", "loT", Pada::Parasmaipada, 3, "sasftAd", "7.1.35"),
    ("03.0018", "loT", Pada::Parasmaipada, 3, "sasftAt", "7.1.35+8.4.56"),
    ("03.0018", "viDiliN", Pada::Parasmaipada, 0, "sasfyAt", "8.4.56"),
```

That is **30 rows**, five per root. Check: √bhṛ 72 cells + 5 = 77 forms; each other root 36 + 5 = 41.

- [ ] **Step 6: Update `paradigm/main.rs`**

In `every_alternate_names_the_vikalpa_rules_that_produced_it`'s doc comment, change `otherwise 1002 bare strings` to `otherwise 1032 bare strings`. `VIKALPA_RULES` is unchanged.

In `derivation_set_shape_matches_the_audited_numbers`:
- `assert_eq!(total_cells, 3924, "436 root×lakāra blocks × 9 cells each")` → `4176`, `"464 root×lakāra blocks × 9 cells each"`.
- `ones` 3184 → `3418`; `twos` 571 → `577`.
- `threes` 123 → `135`, appending to its message: `; and — new in slice 3d — the six ṛ-roots', the same way`.
- `fours`, `fives`, `sixes` and `sevens` are unchanged.
- `ALTERNATES.len()` 1002 → `1032`.
- `key_count("8.4.56")` 146 → `152`; `key_count("7.1.35")` 124 → `136`; `key_count("7.1.35+8.4.56")` 124 → `136`.

Check: 3418 + 577 + 135 + 18 + 10 + 17 + 1 = 4176 cells; 3418 + 1154 + 405 + 72 + 50 + 102 + 7 = 5208 forms = 4176 + 1032.

In the doc comment above that test:
- `3924 cells total (436 root×lakāra blocks × 9), of which 3184 hold exactly` / `one form, 571 hold two, 123 hold three (` → `4176 cells total (464 root×lakāra blocks × 9), of which 3418 hold exactly one form, 577 hold two, 135 hold three (`.
- Inside that three-form parenthesis, after the 3c2 clause about √gā, add `, and the six ṛ-roots', new in slice 3d`. Keep the closing `each by 7.1.35/8.4.56)`.
- The key census line `itself has 1002 rows, keyed 146 \`8.4.56\`, 124 \`7.1.35\`, 124 \`7.1.35+8.4.56\`,` → `itself has 1032 rows, keyed 152 \`8.4.56\`, 136 \`7.1.35\`, 136 \`7.1.35+8.4.56\`,`.

Immediately before `/// This test is what keeps the numbers true day to day.`, add:

```rust
///
/// Slice 3d curates the six consonant-initial ṛ-roots — √pṝ (`03.0004`),
/// √pṛ (`03.0005`), √bhṛ (`03.0006`, ubhayapadī by 1.3.72), √ghṛ (`03.0015`),
/// √hṛ (`03.0016`) and √sṛ (`03.0018`) — bringing the gaṇa to sixteen of its
/// twenty-six rows. Its machinery (7.4.66, the widened 7.4.60, 7.4.76's √bhṛñ
/// row, 7.4.77, 7.1.102, 6.1.77's aṅga arm, 8.2.77 on the ślu path) adds no
/// vikalpa and no fork kind. Each parasmaipada column forks exactly where every
/// -oti parasmaipada root does: vidhiliṅ prathama eka on 8.4.56
/// (`biBfyAd`/`biBfyAt`) and the two loṭ tātaṅ cells three ways
/// (`biBartu`/`biBftAd`/`biBftAt`, `biBfhi`/`biBftAd`/`biBftAt`). Laṅ prathama
/// eka does not fork: its `r` goes to visarga (`abiBaH`). √bhṛ's ātmanepada
/// column forks nowhere. Thirty new rows, five per root, all in the three
/// pre-existing keys. The gaṇa is PARTIAL at 16 of its 26 rows.
```

In `pada_ambiguous_surfaces_are_exactly_these`:
- Append to the comment, after `√mā and √hā, ātmanepada-only, contribute nothing, like √van and` / `√man.`: `Slice 3d's √bhṛ, ubhayapadī, contributes the same two-surface shape as √dā's: \`biBftAm\` (loṭ ātmanepada prathama eka = parasmaipada prathama dvi) and \`abiBfta\` (laṅ ātmanepada prathama eka = parasmaipada madhyama bahu), taking the set from forty-eight to fifty with no new collision against any pre-slice surface. The other five 3d roots are parasmaipada-only and contribute nothing.`
- The expected vector becomes (byte-sorted; measured against the goldens while writing this plan):

```rust
        vec![
            "ArRuta", "BinttAm", "BuNktAm", "CfnttAm", "CinttAm", "DattAm", "GfRutAm", "aBintta",
            "aBuNkta", "aDatta", "aGfRuta", "abiBfta", "acCfntta", "acCintta", "adatta",
            "akuruta", "akzaRuta", "akziRuta", "akzuntta", "anayata", "ariNkta", "arundDa",
            "asanuta", "atanuta", "atfRuta", "atfntta", "atudata", "aviNkta", "ayuNkta", "biBftAm",
            "dattAm", "fRutAm", "kurutAm", "kzaRutAm", "kziRutAm", "kzunttAm", "nayatAm",
            "nayetAm", "nayeta", "riNktAm", "rundDAm", "sanutAm", "tanutAm", "tfRutAm", "tfnttAm",
            "tudatAm", "tudetAm", "tudeta", "viNktAm", "yuNktAm",
        ]
```

After `both_ash_roots_derive`, add the Review Focus test for the √pṛ / √pṝ homographs:

```rust
/// √pṛ (`03.0005 pf\`) and √pṝ (`03.0004 pF`) spell the same form wherever
/// guṇa has run — both aṅgas become `par` — and diverge wherever it has not
/// (`pf` stays, `pF` becomes `pur` by 7.1.102). `check` must report both roots
/// for the first kind and only the right one for the second.
#[test]
fn both_pr_roots_analyse_their_shared_forms() {
    let engine = Panini::new();
    let roots = |form: &str| {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        let mut v: Vec<String> = r.analyses.iter().map(|a| a.dhatu.clone()).collect();
        v.sort_unstable();
        v.dedup();
        v
    };
    for form in ["piparti", "apipaH", "piparARi"] {
        assert_eq!(roots(form), vec!["pF", "pf"], "{form}");
    }
    assert_eq!(roots("pipftaH"), vec!["pf"]);
    assert_eq!(roots("pipUrtaH"), vec!["pF"]);
}
```

- [ ] **Step 7: Run the full suite**

Run: `mise run test 2>&1 | tail -40` (foreground, timeout ≥ 600000 ms)
Expected: PASS at 4176 cells, with every 3924 prior unchanged.

If a new cell fails, the engine and the goldens disagree. Read the failing form against the spec's rule sections before touching either, and use superpowers:systematic-debugging. **Do not edit a golden to match the engine**; the goldens are vidyut's output and they are the specification. The likeliest causes, by symptom:
- `bf-`/`pf-` abhyāsa: 7.4.66 did not fire (Task 1).
- `Bar-`: 7.4.60 did not trim (Task 1).
- `pipfrtaH`-shaped: 7.1.102 ran after 6.1.77, or not at all (Task 3).
- `pipurtaH`: 8.2.77 still reads SHAP (Task 4).
- `biBfAte`: 6.1.77's aṅga arm (Task 3).
- `sasfzati`-shaped: an unexpected ṣatva. Do NOT add 8.3.110; find which rule reached the `s`.

- [ ] **Step 8: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-data/src/lib.rs crates/panini/tests/paradigm/data/juhotyadi.rs crates/panini/tests/paradigm/main.rs
git commit -m "feat(data): the six consonant-initial ṛ-roots — juhotyādi at sixteen of twenty-six rows

3924 → 4176 cells, 4926 → 5208 forms, 1002 → 1032 ALTERNATES, 87 → 93 roots."
```

---

## Task 6: The trace pins

**Files:**
- Modify: `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes: `crate::helpers::{at, cell_trace}`. `cell_trace(number, lakara, pada, purusha, vacana) -> (String, Vec<String>)` returns index 0's text and trace, and `at(&[String], &str) -> usize` panics if the sūtra is absent. Every cell pinned below is single-form.

- [ ] **Step 1: Add the pins**

At the end of the file, add:

```rust
#[test]
fn bibharti_trace_is_ur_at_then_haladih_shesha_then_bhrnam_it() {
    // Bf P laT P.E. 7.4.66 fires even here, on a pit cell, because this
    // engine copies the bare root before guṇa. vidyut copies `Bar` and skips
    // it. This is the √bhṛñ witness 7.4.76's comment promised.
    let (text, t) = cell_trace(
        "03.0006",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "biBarti", "got {t:?}");
    assert!(at(&t, "6.1.10") < at(&t, "7.4.66"), "got {t:?}");
    assert!(at(&t, "7.4.66") < at(&t, "7.4.60"), "got {t:?}");
    assert!(at(&t, "7.4.60") < at(&t, "7.4.76"), "got {t:?}");
    assert!(at(&t, "7.4.76") < at(&t, "7.3.84"), "got {t:?}");
    assert!(at(&t, "7.3.84") < at(&t, "8.4.54"), "got {t:?}");
    assert!(!t.contains(&"6.1.77".to_string()), "got {t:?}");
}

#[test]
fn bibhrati_trace_takes_the_anga_arm_of_iko_yan_aci() {
    // Bf P laT P.B. 7.1.4 gives `ati`; the ṛ meets it directly under ślu.
    let (text, t) = cell_trace(
        "03.0006",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "biBrati", "got {t:?}");
    assert!(at(&t, "7.4.76") < at(&t, "7.1.4"), "got {t:?}");
    assert!(at(&t, "7.1.4") < at(&t, "6.1.77"), "got {t:?}");
    assert!(!t.contains(&"7.3.84".to_string()), "got {t:?}");
}

#[test]
fn piparti_trace_is_arti_pipartyos_ca_not_bhrnam_it() {
    // pf P laT P.E.
    let (text, t) = cell_trace(
        "03.0005",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "piparti", "got {t:?}");
    assert!(at(&t, "7.4.66") < at(&t, "7.4.60"), "got {t:?}");
    assert!(at(&t, "7.4.60") < at(&t, "7.4.77"), "got {t:?}");
    assert!(!t.contains(&"7.4.76".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.1.102".to_string()), "got {t:?}");
}

#[test]
fn pipurtah_trace_is_ud_oshthyapurvasya_then_hali_ca() {
    // pF P laT P.D: kṅit `tas`, so guṇa declines, 7.1.102 makes `pur`, and
    // 8.2.77 lengthens it before the ending's `t` with SHAP empty.
    let (text, t) = cell_trace(
        "03.0004",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "pipUrtaH", "got {t:?}");
    assert!(at(&t, "7.4.77") < at(&t, "7.1.102"), "got {t:?}");
    assert!(at(&t, "7.1.102") < at(&t, "8.2.77"), "got {t:?}");
    assert!(!t.contains(&"7.3.84".to_string()), "got {t:?}");
}

#[test]
fn pipurati_trace_has_ud_oshthyapurvasya_but_no_lengthening() {
    // pF P laT P.B: `ati` is vowel-initial, so 8.2.77 declines, and 6.1.77
    // never sees a ṛ-final aṅga.
    let (text, t) = cell_trace(
        "03.0004",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "pipurati", "got {t:?}");
    assert!(t.contains(&"7.1.102".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.2.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.1.77".to_string()), "got {t:?}");
}

#[test]
fn jagharti_trace_is_ur_at_haladih_shesha_kuhos_cuh_then_car_ca() {
    // Gf P laT P.E. gh → jh (7.4.62) → j (8.4.54).
    let (text, t) = cell_trace(
        "03.0015",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "jaGarti", "got {t:?}");
    assert!(at(&t, "7.4.66") < at(&t, "7.4.60"), "got {t:?}");
    assert!(at(&t, "7.4.60") < at(&t, "7.4.62"), "got {t:?}");
    assert!(at(&t, "7.4.62") < at(&t, "8.4.54"), "got {t:?}");
}

#[test]
fn sasrati_trace_has_no_shatva_and_no_8_3_110() {
    // sf P laT P.B. vidyut credits 8.3.110 here, a bar on a ṣatva that this
    // engine's 8.3.59 cannot reach (the abhyāsa vowel is `a`, not iṇ). Both
    // absences are the pin that 8.3.110 was deliberately not transcribed.
    let (text, t) = cell_trace(
        "03.0018",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "sasrati", "got {t:?}");
    assert!(t.contains(&"6.1.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.3.59".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.3.110".to_string()), "got {t:?}");
}
```

If `Lakara::Lat`, `Purusha::Prathama` or `Vacana::Bahu` are spelled differently in `panini_data`, use the spelling the existing pins in this file use.

- [ ] **Step 2: Run the trace suite**

Run: `mise exec -- cargo test -p panini --test trace 2>&1 | tail -20`
Expected: PASS.

If a pin fails, fix the engine, never the pin. The pins encode the spec's rule order.

- [ ] **Step 3: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini/tests/trace/juhotyadi.rs
git commit -m "test(trace): 3d pins — bibharti, bibhrati, piparti, pipUrtaH, pipurati, jagharti, sasrati"
```

---

## Task 7: Audit, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (audit prose), stale engine comments, `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md` (one note)

**Interfaces:**
- Consumes: the finished engine and goldens (Tasks 1–6). Produces no symbols.

- [ ] **Step 1: Update the audit harness's asserted totals**

In `tools/audit/panini_full_audit.rs`:
- header `for each of the 87 curated roots` → `93`
- `twenty-one ubhayapadī by 1.3.72` → `twenty-two ubhayapadī by 1.3.72`
- `87 roots, 3924 cells, 4926 forms` → `93 roots, 4176 cells, 5208 forms`
- `436 root×pada×lakāra` / `blocks × 9 cells, plus 1002` → `464 … plus 1032`
- `the full 3924-cell table` → `4176-cell`
- `assert_eq!(roots_seen.len(), 87, …)` → `93`
- `assert_eq!(n_cells, 3924, "cells: 436 root×pada×lakāra blocks × 9")` → `4176`, `"cells: 464 root×pada×lakāra blocks × 9"`
- `assert_eq!(n_forms, 4926, "forms: 3924 cells + 1002 ALTERNATES rows")` → `5208`, `"forms: 4176 cells + 1032 ALTERNATES rows"`

In `tools/audit/README.md`, `(87 roots, 3924 cells, 4926 forms)` → `(93 roots, 4176 cells, 5208 forms)`.

- [ ] **Step 2: Repoint vidyut's dev-deps at THIS worktree and run the audit**

`/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes absolute dev-dep paths. The brainstorm left them pointing at `/workspace/crates`, the `main` checkout, which does not have this slice. Auditing without repointing checks the pre-slice engine.

```bash
WT="$(git rev-parse --show-toplevel)"
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
cp tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" mise exec rust@1.98.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -15)
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.98.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -8)
```

Copy the committed harness; never rewrite it.
- Expected from the honest run: `AUDIT PASSED: 4176 cells, 5208 forms, zero differences.`
- Expected from the `entry` control: it fails with exit 1 and 36 √bhū cells.

If the honest run shows differences, stop. The goldens passed, so the engine and vidyut disagree on a form the goldens do not pin. Report it rather than editing anything.

- [ ] **Step 3: Record the audit**

In `tools/audit/README.md`, immediately under `## Last recorded result`, add a new dated entry above the 3c2 one (use today's date):

```markdown
YYYY-MM-DD, juhotyādi 3d slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 4176
cells / 5208 forms / 93 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole juhotyādi 3d slice: 7.4.66 *ur at*, 7.4.60
*halādiḥ śeṣaḥ* widened to every non-initial consonant, 7.4.76's √bhṛñ row,
7.4.77 *arti-pipartyoś ca*, 7.1.102 *ud oṣṭhyapūrvasya*, 6.1.77's aṅga arm,
and 8.2.77 reading the ending on the ślu path — added for √pṝ (`03.0004`),
√pṛ (`03.0005`), √bhṛ (`03.0006`), √ghṛ (`03.0015`), √hṛ (`03.0016`) and √sṛ
(`03.0018`).

Totals: 93 = 87 + 6; 4176 = 3924 + 252 (28 root×pada×lakāra blocks × 9); 5208
= 4926 + 252 + 30 new `ALTERNATES` rows (1002 → 1032), measured via the
harness's corpus block, not assumed.
```

In `crates/panini/tests/paradigm/main.rs`'s audit-chain doc comment, extend the sentence that ends `… and juhotyādi 3c2's re-ran the same probe` / `at the same commit over all 3924 cells / 4926 forms / 87 roots with zero` / `differences, its \`entry\` negative control verified failing (36 √bhū` / `cells).`: before its final `.` add `, and juhotyādi 3d's re-ran it at the same commit over all 4176 cells / 5208 forms / 93 roots with zero differences, its \`entry\` negative control verified failing (36 √bhū cells)`.

- [ ] **Step 4: README.md**

- `**partial** at 10 of its 26 dhātupāṭha rows` → `**partial** at 16 of its 26 dhātupāṭha rows`.
- After the sentence ending `…the first rule to declare an apavāda relation` / `(\`Rule.bars\`).`, insert: `Slice 3d added the six consonant-initial ṛ-roots — √pṝ (\`03.0004\`) and √pṛ (\`03.0005\`), both *piparti*, √bhṛ (\`03.0006\`, *bibharti*, ubhayapadī), √ghṛ (\`03.0015\`, *jagharti*), √hṛ (\`03.0016\`, *jaharti*) and √sṛ (\`03.0018\`, *sasarti*) — behind 7.4.66 *ur at*, 7.4.77 *arti-pipartyoś ca* and 7.1.102 *ud oṣṭhyapūrvasya*, with 7.4.60 widened to every non-initial consonant, 7.4.76 given its √bhṛñ row, 6.1.77 given an aṅga arm, and 8.2.77 reading the ending when ślu empties the śap.`
- `curated 87-root set` → `curated 93-root set`.
- In the multi-form paragraph:
  - `740 of the 3924 cells hold more than one form: 571` / `hold two, 123 hold three (` → `758 of the 4176 cells hold more than one form: 577 hold two, 135 hold three (`.
  - Inside that three-form parenthesis, after `and — new in slice 3c2 — √gā's`, add `, and — new in slice 3d — the six ṛ-roots'`.
  - Check: 577 + 135 + 18 + 10 + 17 + 1 = 758.
- The pada-ambiguous paragraph: `Forty-eight surfaces are pada-ambiguous` → `Fifty surfaces are pada-ambiguous`. Insert `` `abiBfta`, `` after `` `aGfRuta`, `` and `` `biBftAm`, `` after `` `ayuNkta`, ``, so the quoted list matches Task 5's vector.

Reflow the edited paragraphs to the file's existing ~80-column wrap.

- [ ] **Step 5: docs/ARCHITECTURE.md**

- Stage table: the `abhyasa.rs` row becomes `6.1.10, 7.4.66, 7.4.60, 7.4.59, 7.4.62, 7.4.76, 7.4.77, 7.4.78 — dvitva and the abhyāsa's shape`. In the `guna.rs` row, `7.3.84 (again — see below), 6.4.110` → `7.3.84 (again — see below), 7.1.102, 6.4.110`.
- `pins all 118 ids verbatim` → `pins all 121 ids verbatim`. Confirm by counting the entries of `expected` in `tinanta_rule_order_is_pinned`. After `6.4.117 *ā ca hau* and 6.4.116 *jahāteś ca* — 118 total` add ` — then juhotyādi 3d's three: 7.4.66 *ur at*, 7.4.77 *arti-pipartyoś ca* and 7.1.102 *ud oṣṭhyapūrvasya* (7.4.60, 7.4.76, 6.1.77 and 8.2.77 were widened, not added) — 121 total`.
- `**partial** at 10 of its 26 rows (… √hā` / `parasmaipada, √gā; slice 3c2)` → `**partial** at 16 of its 26 rows (… √hā parasmaipada, √gā; slice 3c2; √pṝ, √pṛ, √bhṛ, √ghṛ, √hṛ, √sṛ; slice 3d)`.
- The 7.1.35 / 8.4.56 accounting paragraph (find it with `grep -n "forking 124 cells" docs/ARCHITECTURE.md`):
  - `forking 124 cells` → `136`
  - `across the 62 roots with a parasmaipada column` → `68`
  - the juhotyādi list `√hu, √ki, √bhī, √hrī, √hā (\`03.0009\`) and √gā (all parasmaipada-only)` → `√hu, √ki, √bhī, √hrī, √hā (\`03.0009\`), √gā, √pṝ, √pṛ, √ghṛ, √hṛ and √sṛ (all parasmaipada-only), and √bhṛ's parasmaipada column`
  - `62 + 25 = the 87 curated roots` → `68 + 25 = the 93 curated roots`
  - `forking 146 cells outright` → `152`
  - `across those same 62 parasmaipada columns (124 of` → `68 … (136 of`

  If the list's wording differs from the above, keep its form and make the same substitution. The counts are what must be right: 6 new parasmaipada columns, 12 new `7.1.35` rows, 6 new `8.4.56` rows.
- Add one paragraph after the juhotyādi-3c ordering paragraph (`Two orderings from juhotyādi 3c are worth naming here. …`):

```markdown
One ordering from juhotyādi 3d is worth naming too. **7.1.102 *ud
oṣṭhyapūrvasya* runs after both 7.3.84 applications and before 6.1.77**. After
guṇa, it reaches only the kṅit cells, where guṇa declined (*pipūrtaḥ*, not
\**pipartaḥ*). Before 6.1.77, it turns √pṝ's `F` into `ur` before the yaṇ rule
could see a ṛ-final aṅga (*pipurati*). Separately, the abhyāsa stage now runs
7.4.66 *ur at* on every cell of a ṛ-root. This engine copies the bare root
before guṇa, so √bhṛ's abhyāsa is always `Bf` → `Bar` → (7.4.60) `Ba`, where
vidyut copies the guṇated `Bar` on pit cells and skips 7.4.66. The forms agree
and the *bibharti* pin holds this order.
```

- [ ] **Step 6: AGENTS.md**

- Rules of the codebase: `(\`crates/panini/tests/paradigm/\`, 3924 cells,` → `4176 cells`. In the juhotyādi clause, change `and now at 10 of its 26 after slice 3c2 curated √hā` / `(parasmaipada) and √gā —` to `at 10 after slice 3c2 curated √hā (parasmaipada) and √gā, and now at 16 of its 26 after slice 3d curated √pṝ, √pṛ, √bhṛ, √ghṛ, √hṛ and √sṛ —`. Change `(1002 rows in all, so 3924 + 1002 = 4926 forms total)` → `(1032 rows in all, so 4176 + 1032 = 5208 forms total)`.
- The audit record: after `3c2's (\`tools/audit/README.md\`'s 2026-09-29 entry, 3924 cells / 4926 forms /` / `87 roots)` add `, and that by juhotyādi 3d's (\`tools/audit/README.md\`'s YYYY-MM-DD entry, 4176 cells / 5208 forms / 93 roots)`.
- The stale-comment paragraph: `3924 goldens` / `would move today` → `4176 goldens`. After `…both lines measured by grep at this` / `commit).` add ` Juhotyādi 3d touched neither comment either; the corpus stands at 4176 cells as of 3d (\`guna.rs:2162\`'s claim now anchored at \`guna.rs:<G>\`, \`controller.rs:206\`'s at \`controller.rs:206\`: 3d added 7.1.102 and 6.1.77's aṅga arm above it in \`guna.rs\` and did not touch \`controller.rs\`).` Measure `<G>` with `grep -n "1872 goldens" crates/panini-prakriya/src/tinanta/guna.rs`. Confirm the controller anchor with `grep -n "1800 goldens" crates/panini-prakriya/src/controller.rs`; if it moved, write the measured line.

- [ ] **Step 7: Stale engine comments**

- `abhyasa.rs`: Task 1 already reworded 7.4.60's rule comment. Reword the one left, in `haladih_shesha_records_nothing_for_a_single_initial_consonant`, from `the 3924 priors break` to `the priors break`, so the count cannot go stale again.
- `guna.rs`: 6.4.77's `The 3924 byte-identical` → `The 4176 byte-identical`; `87-root × 4-lakāra grammar` (6.1.78's comment) → `93-root × 4-lakāra grammar`. That comment argues no aṅga can end in a vṛddhi vowel. Check the argument still holds with the 3d roots: none produces `E`/`O` in `ANGA` (6.1.90 is √ṛ's, 3d2). Do not reword it otherwise.
- `terms.rs`: `following_sarvadhatuka`'s and `sound_before_ending`'s docs enumerate open-coded copies of follower walks. Task 4's 8.2.77 open-codes a SHAP-then-ENDING read. If `sound_before_ending`'s doc claims to enumerate every copy of *that* walk, the two are different walks (the next sound after the aṅga vs the sound before the ending), so add nothing. Confirm by reading.
- Sweep for comments that 3d promised or falsified:

```bash
grep -rn "3d\b\|slice 3d\|3d–3f\|3d-3f\|in 3d" crates --include=*.rs
grep -rn "√ṛ\|\bf\b.*vowel-initial\|ONLY cluster-initial\|Only the vikaraṇa arm\|Currently unreachable\|slice 3d adds" crates/panini-prakriya/src --include=*.rs
```

Every comment that says 3d *will* do something must now say it did, or say 3d2 when the promise was √ṛ's. Delete it if the sentence only existed to promise it.

- [ ] **Step 8: Note the slice in 3a's spec**

In `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md`, after the existing slice-3c2 note beneath the "Later slices" table, add:

```markdown
> Slice 3d (`2026-09-29-juhotyadi-gana-3d-design.md`) split this table's 3d
> row: 3d took the six consonant-initial ṛ-roots, 252 cells, and widened
> 7.4.60, 7.4.76, 6.1.77 and 8.2.77 beside the table's new sūtras. √ṛ, with
> 6.4.78 and 6.4.72 / 6.1.90 on the abhyāsa, became slice 3d2. The table's
> 8.3.110 is not transcribed (no form depends on it), and vidyut's 8.4.37 is
> this engine's existing 8.4.2 guard.
```

- [ ] **Step 9: Sweep for anything left stale**

```bash
grep -rn "3924\|4926\|\b1002\b\|87 roots\|87-root\|of these 87\|of the 87\|\b436\b\|740 of\|10 of its 26\|ten of its twenty-six\|118 ids\|118 total\|Forty-eight surfaces\|forty-eight\|62 + 25\|twenty-one ubhayapad" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.md --include=*.rs
grep -rn "\bBf\b\|\bpf\b\|\bpF\b\|\bGf\b\|\bhf\b\|\bsf\b\|\bBar\b\|\bpur\b\|sasr" crates/panini-prakriya/src --include=*.rs | grep "//"
```

Expected residue:
- AGENTS.md's dated mutation-record entries: history; never rewrite them.
- Prose that names 3c2's numbers explicitly as 3c2's.
- The two mid-sweep records in `pada_ambiguous_surfaces_are_exactly_these`'s comment that say "forty-eight" as a historical step.

Fix any other hit. For each root-shape hit, check the comment is still true.

- [ ] **Step 10: Run the full suite**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS at 4176 cells.

- [ ] **Step 11: Commit**

```bash
mise run fmt && mise run lint
git add -A
git commit -m "docs: 3d's counts, the audit record, and the comments it falsifies

4176 cells / 5208 forms / 93 roots / 1032 ALTERNATES across README,
ARCHITECTURE, AGENTS, paradigm/main.rs and tools/audit. Audit at zero
divergence against 8da2f90b."
```

---

## Task 8: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and a dated campaign entry); `mise.toml` if the cap moves

Follow AGENTS.md's cargo-mutants protocol exactly. Four hazards from this repo's record apply:
- **Measure, never scale.**
- **Every invocation rotates `mutants.out`**, so always pass `-o`.
- **The mise shim fails in background shells.** Use the real binary: `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`.
- **Background shells die at ~60 minutes, and 3c2's campaign took 2h01m.** The campaign must run detached with `setsid nohup`, not as a harness background task.

- [ ] **Step 1: Run the exhaustive tier once**

Run: `mise run test-full 2>&1 | tail -20` (foreground, timeout 3600000 ms)
Expected: PASS, including `roundtrip_exhaustive` over all 5208 forms. If it exceeds the timeout, stop and report the measured duration rather than backgrounding it.

- [ ] **Step 2: Measure the uncontended floor**

With nothing else running: `time mise run test 2>&1 | grep -E "Running|finished in|real"` (foreground). Record the wall clock and the times of the paradigm, roundtrip and trace binaries.

Compare with the 3924-cell floor: 80.128 s total, with paradigm 33.14 s, roundtrip 39.98 s and trace 5.53 s. `roundtrip_sampled` is Θ(N²) in roots and six roots is the biggest step since 8a (+6.9% roots, +6.4% cells), so expect roundtrip to grow by well over 6%.

- [ ] **Step 3: Measure a full UNCAUGHT run at `-j 4`**

The two documented equivalent mutants run the suite to completion uncaught. At 3c2's tree they were `adesha.rs:588:30` (`replace + with *`) and `tripadi.rs:1203:38` (`replace - with /`). This slice does not touch `adesha.rs`. Task 4 adds lines to `tripadi.rs` above 1203, so the second has moved. Locate both by source text:

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
git show main:crates/panini-prakriya/src/tinanta/tripadi.rs | sed -n 1203p
grep -nF "$(git show main:crates/panini-prakriya/src/tinanta/tripadi.rs | sed -n 1203p)" crates/panini-prakriya/src/tinanta/tripadi.rs
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:588:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /"
```

Substitute the new tripadi line for `<LINE>`, confirm `--list` shows exactly those two mutants, then run them (foreground, timeout 1800000 ms):

```bash
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 600 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:588:30: replace \+ with \*" --re "tripadi.rs:<LINE>:38: replace - with /" 2>&1 | tail -10
```

Both must be MISSED, not TIMEOUT. Read each one's test-phase duration from `$SCRATCH/mutants.out/outcomes.json`.

**Cap rule, applied to measurements only.** AGENTS.md forbids scaling by cell count or by a projected contention multiplier. 3c2's campaign measured its longer equivalent at 116.26 s under full contention, and the suite has only grown since, so that number is a measured lower bound.
- Let `L` = max(this probe's longer phase, 116.26 s).
- Keep `--timeout 600` if 600 ÷ `L` ≥ 5. Otherwise set the cap to 6 × `L`, rounded up to the next 100 s.
- Change `mise.toml`'s `--timeout` and AGENTS.md's floor paragraph together.
- Use the chosen cap in Steps 4–5 in place of `600`.

- [ ] **Step 4: Launch the campaign detached**

```bash
OUT="$HOME/mutants-records/juhotyadi-3d"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout <CAP> -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"
```

The 3924-cell campaign took 2h01m.
- Check progress with `tail -3 "$OUT/campaign.log"`.
- Check liveness with `pgrep -x cargo-mutants`, **not** `pgrep -f`, which matches its own shell.
- Run nothing CPU-heavy in the meantime, because contention inflates test phases toward the cap.
- Wait with a long ScheduleWakeup or Monitor on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

- [ ] **Step 5: Read both outcome files**

When `pgrep -x cargo-mutants` returns nothing:

```bash
date -u +"%F %T UTC" > "$OUT/finished"
tail -5 "$OUT/campaign.log"
cat "$OUT/mutants.out/missed.txt" "$OUT/mutants.out/timeout.txt"
```

Expected:
- `missed.txt` holds exactly the two documented equivalents at their Step 3 positions.
- `timeout.txt` holds exactly the permanent `tripadi.rs` ṇatva-scan `replace -= with /=`. It was at `:1529:23` on `main`; it has drifted by Task 4's line delta.
- `cargo-mutants` exits 3 when timeouts are present, which is expected.

If the result differs:
- Any **other timeout** is a suspect survivor. Re-run it alone with its own `-o` directory and `--re` before concluding anything.
- Any **other missed** mutant in 3d's code means a guard test does not separate the mutant. The likeliest sources:
  - 7.4.66's `t == …` no-op test
  - 7.4.60's `filter(is_vowel)` or its vowel-initial guard
  - 7.4.77's / 7.4.76's number match
  - 7.1.102's `is_some_and(matches!(…))`
  - 6.1.77's `is_empty()` or `strip_suffix('f')`
  - 8.2.77's `!t.text.is_empty()` arm

  Strengthen the named guard test, commit it, and re-run only those mutants with `--re` and a fresh `-o`.

- [ ] **Step 6: Keep the outcomes and compute the margins**

`$OUT` is already durable; do not run another `cargo-mutants` against it. Inspect one record first to see how `duration` is stored:

```bash
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["outcomes"][1])' "$OUT/mutants.out/outcomes.json"
```

Then compute the caught mutants' test-phase min / median / p90 / p99 / max, and the two margins AGENTS.md records:
- the cap ÷ the longest uncaught test phase in THIS campaign (the two missed equivalents)
- the cap ÷ the slowest caught mutant

If the first margin is below 5×, the cap is too tight for the next slice. Record the ruling and the next cap (6 × that phase, rounded up to 100 s) as 3c2 did. Do not re-run the campaign; the equivalents finished as MISSED, so this campaign's verdict is sound.

- [ ] **Step 7: Record it in AGENTS.md**

- Rewrite the `**The floor behind the 600s cap, measured at 3924 cells.**` paragraph with Step 2's and Step 3's measured numbers at 4176 cells, and the cap Step 3 chose. If the cap moved, change `mise.toml` in the same commit, and change the paragraph's `600s` wording everywhere it names the current cap.
- Append a dated entry after the `**2026-09-29 — slice 3c2 re-measured both at 3924 cells.**` entry, in its style. It records:
  - cell growth 3924 → 4176 (+6.42%) and the floor's measured move
  - the probe phases and the cap ruling from Step 3
  - the campaign window (`started`/`finished`) and wall clock
  - **mutants / caught / unviable / missed / timeout** counts summing to the total
  - `missed.txt` and `timeout.txt` **named verbatim with their new positions**
  - the caught-mutant duration distribution
  - both margins
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`
  - the non-caught set diffed against 3c2's (the clean result is an identical set at drifted positions)
  - that every 3d rule block (7.4.66, 7.4.60, 7.4.76, 7.4.77, 7.1.102, 6.1.77, 8.2.77) has no missed, timeout or unviable mutant

- [ ] **Step 8: Commit**

```bash
git add AGENTS.md mise.toml
git commit -m "chore: 3d mutation gate — floor and uncaught run re-measured at 4176 cells

missed.txt holds only the two documented equivalents and timeout.txt only
the permanent ṇatva-scan entry, at drifted positions; the cap was checked
against a measured -j 4 uncaught run, not scaled."
```

---

## Task 9: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -20
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin juhotyadi-3d
gh pr create --title "juhotyādi 3d — the six consonant-initial ṛ-roots" --body "$(cat <<'BODY'
Slice 3d curates √pṝ (`03.0004`), √pṛ (`03.0005`), √bhṛ (`03.0006`,
ubhayapadī), √ghṛ (`03.0015`), √hṛ (`03.0016`) and √sṛ (`03.0018`), taking the
golden suite from 3924 to 4176 cells and juhotyādi from ten of its twenty-six
rows to sixteen. √ṛ, the gaṇa's one vowel-initial ṛ-root, is split out to 3d2.

**New rules:** 7.4.66 *ur at* (the abhyāsa's ṛ → `ar`), 7.4.77
*arti-pipartyoś ca* (√pṛ/√pṝ's abhyāsa `i`, by row number) and 7.1.102 *ud
oṣṭhyapūrvasya* (√pṝ's `F` → `ur` where guṇa declined).

**Widened:** 7.4.60 now elides every non-initial consonant of the abhyāsa
(`Bar` → `Ba`), 7.4.76 gains √bhṛñ, 6.1.77 gains an aṅga arm for the ślu path
(*bibhrati*), and 8.2.77 reads the ending when ślu empties the śap
(*pipūrtaḥ*). All four surfaced in the re-probe, not in the prep table.

No new vikalpa. vidyut's 8.3.110 on *sasrati* is not transcribed (no form
depends on it) and is pinned absent.

Re-probed against vidyut and against this engine's HEAD cell by cell before
the spec; audit at zero divergence against `8da2f90b`; mutation gate clean.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Per the standing instruction:
1. Once checks are green, merge with `gh pr merge --merge --auto`.
2. Confirm the branch's commits are on `main`: after `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. Delete the remote and local branch, and remove the worktree with `git worktree remove .worktrees/juhotyadi-3d`, run from the main checkout.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| 7.4.66, directly after 6.1.10, text-guarded, fires on every 3d cell | 1 |
| 7.4.60 widened; √hrī unchanged; comment corrected | 1 |
| 7.4.76 gains `03.0006` | 2 |
| 7.4.77 keyed on `03.0005` / `03.0004`, `03.0017` deferred | 2 |
| 7.1.102 after 7.3.84, before 6.1.77, pu-varga + `v` | 3 |
| 6.1.77's aṅga arm, SHAP-empty, short `f` only | 3 |
| 8.2.77 on the root+ending junction | 4 |
| 8.3.110 not transcribed, pinned absent; 8.4.37 recorded | 5 (row comment), 6 (pin), 7 (3a note) |
| Rows, counts, buckets, 30 `ALTERNATES` rows by key | 5 |
| Trace pins (*bibharti*, *bibhrati*, *piparti*, *pipūrtaḥ*, *pipurati*, *jagharti*, *sasrati*) | 6 |
| Audit with dev-dep repoint and negative control; README/ARCHITECTURE/AGENTS sweep; 3a-spec pointer | 7 |
| `test-full`, re-measured floor, uncaught run, verbatim non-caught record | 8 |

**Type consistency.**
- `slu_prakriya(&str, &str) -> Prakriya` and `abhyasta_prakriya(&str, &str, bool, &str, bool) -> Prakriya` are the existing helpers, used unchanged.
- `run_abhyasa_stage(&str, &'static str) -> Prakriya` is defined and used only in Task 2. It takes `&'static str` because `Context.dhatupatha` is `&'static str`, as the existing tests' `p.ctx.dhatupatha = number` with literal numbers shows.
- Rule ids `"7.4.66"`, `"7.4.77"`, `"7.1.102"` match across rule bodies, `p.record` calls, `tinanta_rule_order_is_pinned` and the trace assertions. Names `urat`, `artipipartyoSca`, `udozWyapUrvasya` are used identically in `name:` and `p.record`.

**Known soft spots.**
- **Pinned-order edits are cumulative.** Task 2 sees `"7.4.76", "7.4.78"`. Task 3 must change the *second* `"7.3.84"` only.
- **Task 3 Step 2 expects three tests to pass before the implementation.** They are decline and regression tests that hold new clauses under mutation, not tests of new behaviour.
- **7.4.60's vowel-initial guard is known to be wrong for √ṛ** (vidyut trims `ar` → `a` on `03.0017`). It is unreachable in this slice's corpus, its comment says so, and 3d2 owns the fix with the witness.
- **Mutant positions drift** in `tripadi.rs` (Task 4) and `guna.rs` (Task 3). Task 8 locates the known mutants by source text.
