# Juhotyādi gaṇa slice 3b Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate √bhī (`03.0002`) and √hrī (`03.0003`) into the juhotyādi gaṇa, taking the golden suite from 3564 to 3636 cells, by landing the abhyāsa's own shaping rules (7.4.60, 7.4.59), √bhī's optional hrasva (6.4.115), 6.4.77's iyaṅ arm, and 6.4.82's long-`I` widening — and by deleting a `pada_from_upadesha` clause that has been wrong since it was written.

**Architecture:** Eight tasks. Task 1 is a self-contained data-layer correction that moves no golden. Tasks 2–4 add engine rules that fire on no curated root yet, so each is gated on **per-rule guard tests plus the 3564 priors staying byte-identical** — that bar is the proof behind every widening in this slice. Task 5 lands the data and turns the new cells green in one step; Task 6 pins the traces. Tasks 7–8 are the audit/doc sweep and the mutation gate.

**Tech Stack:** Rust 1.98.0 pinned via `mise`. `mise run build | test | lint | fmt | mutants | audit`. Cross-implementation goldens from vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`.

**Spec:** `docs/superpowers/specs/2026-09-09-juhotyadi-gana-3b-design.md`

## Global Constraints

- **The 3564 pre-existing cells must stay byte-identical through Tasks 1–4.** No golden regenerated, no pinned trace changed. This is not ceremony: it is the evidence that 6.4.82's widening, 6.4.77's general arm and both no-op guards reach nothing they should not.
- Every new rule carries a **no-op guard**: if the computed result equals the input, return `false` and record nothing. A rule that records a no-op step breaks the priors.
- **No unfalsifiable guard clauses.** A clause that cannot be made false by any cell in the suite is a mutation survivor; either witness it, delete it, or kill it with a direct table test. This is the rule 3a applied when it deleted 6.4.72's log scan and refused gaṇa clauses on 6.4.87/6.4.101.
- Root-keyed guards read `ANGA.text` with **no gaṇa clause** beside them (the 6.4.87 / 6.4.101 precedent).
- Golden forms are **transcribed from the spec's appendix**, never invented. The appendix is vidyut's output at the pinned commit.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit.
- **`mise run test` takes ~38 minutes at 3564 cells** (paradigm 1003s, roundtrip 1249s, trace 4s — AGENTS.md's slice-3a measurement). Run it in the foreground with an explicit long timeout; never background it and end a turn.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-data/src/lib.rs` | Task 1's `pada_from_upadesha` correction + disjointness invariant; Task 5's two `Dhatu` rows and the extended gaṇa-row test |
| `crates/panini-prakriya/src/tinanta/sound.rs` | Task 2's `hrasva_of` map and its arm-by-arm table test |
| `crates/panini-prakriya/src/tinanta/abhyasa.rs` | Task 2's 7.4.60 and 7.4.59, between 6.1.10 and 7.4.62 |
| `crates/panini-prakriya/src/tinanta/guna.rs` | Task 3's 6.4.82 widening and 6.4.77 iyaṅ arm; Task 4's 6.4.115 |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | the pinned rule order and the pinned vikalpa set |
| `crates/panini/tests/paradigm/data/juhotyadi.rs` | Task 5's 8 `PARADIGM` blocks and 40 `ALTERNATES` rows |
| `crates/panini/tests/paradigm/main.rs` | Task 5's corpus totals and Task 7's fork-distribution prose |
| `crates/panini/tests/trace/juhotyadi.rs` | Task 6's five trace pins |
| `tools/audit/`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md` | Task 7's counts and doc sweep |

---

## Task 1: Correct `pada_from_upadesha`'s `Yi` clause

Self-contained. Moves no golden, adds no rule. Do it first so Task 5's curated `Parasmaipada` for `03.0002` has somewhere to land.

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (`pada_from_upadesha`, ~line 1455–1482; tests ~line 1639)

**Interfaces:**
- Consumes: nothing from earlier tasks.
- Produces: `pada_from_upadesha(&str) -> PadaAssignment` returning `Parasmaipada` for `"YiBI\\"`. Task 5 relies on this.

- [ ] **Step 1: Write the three failing tests**

Add to the `tests` module in `crates/panini-data/src/lib.rs`. Delete the existing `indh_is_atmanepada_despite_satisfying_1_3_72` and replace it with the renamed version below.

```rust
    #[test]
    fn bhi_is_parasmaipada_despite_its_initial_nyi() {
        // `YiBI\` carries a ñi that 1.3.5 ādir ñiṭuḍavaḥ makes an it — and
        // that is ALL it makes it. The it so supplied is not an anubandha
        // 1.3.72 svaritañitaḥ reads: vidyut-prakriya fires 1.3.72 on NONE
        // of the dhātupāṭha's fourteen `Yi`-initial rows (01.0594, 01.0844,
        // 01.0845, 01.0846, 01.0884, 01.1133, 02.0063, 03.0002, 04.0127,
        // 04.0141, 04.0158, 04.0159, 05.0025, 07.0011). Every ātmanepada
        // verdict among them comes from a `~\`, every parasmaipada one from
        // 1.3.78.
        //
        // √bhī is the first curated root to reach the clause this pins the
        // removal of; before slice 3b it decided nothing, which is how it
        // stayed wrong. `\` here sits on the root vowel, not on an it.
        assert_eq!(pada_from_upadesha("YiBI\\"), PadaAssignment::Parasmaipada);
    }

    #[test]
    fn indh_is_atmanepada_by_its_anudatta_it() {
        // `YiinDI~\`'s ātmanepada comes from the `~\` and nothing else. Its
        // initial ñi is an it by 1.3.5 but decides no pada — see
        // `bhi_is_parasmaipada_despite_its_initial_nyi`.
        //
        // This test used to be named `..._despite_satisfying_1_3_72` and
        // pinned the PRECEDENCE of 1.3.12 over 1.3.72, on the premise that
        // √indh satisfied both branches. It satisfied the second only via
        // the deleted `Yi` clause. The precedence is now carried by
        // `no_upadesha_satisfies_both_pada_branches` instead.
        assert_eq!(pada_from_upadesha("YiinDI~\\"), PadaAssignment::Atmanepada);
    }

    #[test]
    fn no_upadesha_satisfies_both_pada_branches() {
        // With the `Yi` clause gone, NO row in the dhātupāṭha satisfies both
        // of `pada_from_upadesha`'s branch conditions, so their order decides
        // nothing and swapping the two `if` blocks is an unkillable mutant.
        //
        // This test is what replaces that lost falsifiability: it asserts the
        // disjointness as an invariant of the DATA. If upstream ever grows a
        // row carrying both an anudātta/ṅ it and a svarita/ñ it, the
        // precedence question returns loudly instead of silently, and
        // 1.3.12-before-1.3.72 has to be re-argued rather than assumed.
        //
        // Same tripwire idiom as the `code`-uniqueness assertion in
        // `juhotyadi_rows_are_the_four_curated_roots`.
        let mut both: Vec<&str> = Vec::new();
        for (_, upadesha, _) in upstream_rows() {
            let anudatta_it = upadesha.contains("~\\");
            let svarita_it = upadesha.contains("~^");
            let bare: String = upadesha
                .chars()
                .filter(|c| *c != '\\' && *c != '^')
                .collect();
            let final_it = bare.chars().last().filter(|c| is_hal(*c));
            let atmanepada_branch = anudatta_it || final_it == Some('N');
            let ubhayapada_branch = svarita_it || final_it == Some('Y');
            if atmanepada_branch && ubhayapada_branch {
                both.push(upadesha);
            }
        }
        assert!(
            both.is_empty(),
            "these upadeśas satisfy both branches, so their order is load-bearing again: {both:?}"
        );
    }
```

- [ ] **Step 2: Run the tests to verify the first one fails**

Run: `mise exec -- cargo test -p panini-data pada 2>&1 | tail -30`

Expected: `bhi_is_parasmaipada_despite_its_initial_nyi` FAILS with `left: Ubhayapada, right: Parasmaipada`. `indh_is_atmanepada_by_its_anudatta_it` and `no_upadesha_satisfies_both_pada_branches` FAIL to compile or pass trivially — that is fine; the first is the red test.

- [ ] **Step 3: Delete the clause and rewrite the comment**

In `pada_from_upadesha`, replace this:

```rust
        // 1.3.5 ādir ñiṭuḍavaḥ supplies a ñ it as an initial `Yi` too. Do NOT
        // extend this to `wu`/`qu`: the sūtra makes ñi, ṭu AND ḍu its, but
        // only ñi is a ñ-it, and 1.3.72 reads *svarita or ñit* specifically
        // — adding a wu/qu arm here would wrongly make every ṭu/ḍu-initial
        // root ubhayapadī. `01.1130 qula\Ba~\z` (√labh) is a curated
        // ḍu-initial root; it already comes out Ātmanepada correctly via its
        // own `~\`, not via this function.
        let nyit = final_it == Some('Y') || bare.starts_with("Yi");
```

with this:

```rust
        // ONLY a final `Y`. 1.3.5 ādir ñiṭuḍavaḥ does make an initial `Yi`
        // an it — `strip_anubandhas` above strips it, which is why √bhī's
        // curated code is `BI` — but the it it supplies is not an anubandha
        // 1.3.72 svaritañitaḥ reads. vidyut-prakriya fires 1.3.72 on none of
        // the dhātupāṭha's fourteen `Yi`-initial rows; every ātmanepada
        // verdict among them comes from a `~\` and every parasmaipada one
        // from 1.3.78. Pinned by
        // `bhi_is_parasmaipada_despite_its_initial_nyi`.
        //
        // This read `final_it == Some('Y') || bare.starts_with("Yi")` until
        // slice 3b. The disjunct decided nothing for the first 79 curated
        // roots — √indh, the only `Yi`-initial one, is caught by the
        // ātmanepada branch below — so it sat unfalsified until √bhī became
        // the first root whose verdict it decided, and decided wrongly.
        // The same argument forbids a `wu`/`qu` arm here, for the stronger
        // reason that ṭu and ḍu are not ñ-its at all.
        let nyit = final_it == Some('Y');
```

Then replace the precedence comment above the two branches:

```rust
        // ORDER IS LOAD-BEARING. 1.3.12 is tested first because `YiinDI~\`
        // (√indh) satisfies both it and 1.3.72, and must come out ātmanepada.
        // Pinned by `indh_is_atmanepada_despite_satisfying_1_3_72`.
```

with:

```rust
        // The two conditions are DISJOINT over every upstream row — no
        // upadeśa carries both an anudātta/ṅ it and a svarita/ñ it — so this
        // order decides nothing today, and `no_upadesha_satisfies_both_pada_branches`
        // is what asserts that rather than leaving it assumed. The order is
        // kept because 1.3.12 is the apavāda by tradition: if upstream ever
        // grows a row satisfying both, that test fails first and this
        // sequence is the answer already in place.
```

- [ ] **Step 4: Run the tests to verify they pass**

Run: `mise exec -- cargo test -p panini-data 2>&1 | tail -20`
Expected: PASS, including `curated_pada_agrees_with_upadesha_markers` (unchanged — no curated root's verdict moves).

- [ ] **Step 5: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-data/src/lib.rs
git commit -m "fix(data): an initial ñi is an it, not a ñit 1.3.72 reads

pada_from_upadesha treated `Yi`-initial upadeśas as ñit and sent them to
1.3.72. vidyut-prakriya fires 1.3.72 on none of the dhātupāṭha's fourteen
such rows. The clause decided nothing for the first 79 curated roots —
√indh is caught by the earlier anudātta branch — so it stayed wrong until
√bhī became the first root to reach it.

Deleting it makes the branch order vacuous (no upstream row satisfies both
conditions), so the precedence moves from a comment to a data invariant:
no_upadesha_satisfies_both_pada_branches."
```

---

## Task 2: 7.4.60 and 7.4.59 — the abhyāsa's own shape

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/sound.rs` (add `hrasva_of` + its table test)
- Modify: `crates/panini-prakriya/src/tinanta/abhyasa.rs` (two rules between 6.1.10 and 7.4.62, plus guard tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: `ABHYASA`, `ANGA`, `SHAP` from `tinanta::terms`; `Tag::Slu`; `is_vowel` from `tinanta::sound`.
- Produces: `hrasva_of(char) -> Option<char>` in `sound.rs`, used by 7.4.59 only. Rules `"7.4.60"` and `"7.4.59"` in `ABHYASA_RULES`, positioned between `"6.1.10"` and `"7.4.62"`.

- [ ] **Step 1: Write the failing sound-table test**

Add to `sound.rs`'s `tests` module, beside `deaspirate_of_aspirate_stops_all_arms`:

```rust
    #[test]
    fn hrasva_of_long_vowels_all_arms() {
        // 7.4.59 hrasvaḥ's substitute, arm by arm. Only the `I` arm has a
        // cell in slice 3b (√bhī's `BI` and √hrī's `hI`); `A` arrives with
        // √dā in 3c and `F` with √pṝ in 3d. The arms are killed here rather
        // than left waiting for those slices — the same reason every other
        // map in this file has an `_all_arms` test.
        for (from, to) in [('A', 'a'), ('I', 'i'), ('U', 'u'), ('F', 'f'), ('X', 'x')] {
            assert_eq!(hrasva_of(from), Some(to), "{from}");
        }
        // Already hrasva: None, so 7.4.59 can use this lookup as its own
        // match test and record nothing for √hu and √ki.
        for c in ['a', 'i', 'u', 'f', 'x'] {
            assert_eq!(hrasva_of(c), None, "{c}");
        }
        // 1.1.48 ec igghrasvādeśe is DELIBERATELY ABSENT. It would make the
        // hrasva of `e`/`o` come out `i`/`u`, and vidyut-prakriya needs it —
        // it copies the guṇated stem, so its abhyāsa really is `Be` when
        // 7.4.59 fires. This engine copies the bare root BEFORE guṇa, so its
        // abhyāsa never carries an ec here. liṭ, which reduplicates after
        // other operations have run, is the slice that must add the arm.
        for c in ['e', 'E', 'o', 'O'] {
            assert_eq!(hrasva_of(c), None, "{c}");
        }
        // Consonants are not this map's business.
        for c in ['k', 'h', 'B', 'r'] {
            assert_eq!(hrasva_of(c), None, "{c}");
        }
    }
```

- [ ] **Step 2: Run it to verify it fails**

Run: `mise exec -- cargo test -p panini-prakriya hrasva_of 2>&1 | tail -20`
Expected: FAIL to compile — `cannot find function hrasva_of in this scope`.

- [ ] **Step 3: Add `hrasva_of` to `sound.rs`**

Place it immediately after `deaspirate_of`:

```rust
/// The hrasva counterpart of a long simple vowel — 7.4.59 hrasvaḥ's
/// substitute for the abhyāsa. `None` for a vowel that is already hrasva,
/// so 7.4.59 can use this one lookup as its match test as well (the
/// `deaspirate_of` / `kutva_of` idiom) and record nothing for √hu and √ki.
///
/// The ec vowels are absent, and that is a claim about THIS engine's rule
/// order rather than about the sūtra: 1.1.48 ec igghrasvādeśe makes the
/// hrasva of `e`/`o` come out `i`/`u`, which vidyut-prakriya needs because
/// it copies the guṇated stem (its abhyāsa is `Be` when 7.4.59 fires). This
/// engine copies the bare root before guṇa — see `abhyasa.rs`'s header — so
/// no abhyāsa here carries an ec at 7.4.59. liṭ reduplicates after other
/// operations have run and is the slice that must add the arm.
pub(crate) fn hrasva_of(c: char) -> Option<char> {
    Some(match c {
        'A' => 'a',
        'I' => 'i',
        'U' => 'u',
        'F' => 'f',
        'X' => 'x',
        _ => return None,
    })
}
```

- [ ] **Step 4: Run the table test to verify it passes**

Run: `mise exec -- cargo test -p panini-prakriya hrasva_of 2>&1 | tail -10`
Expected: PASS.

- [ ] **Step 5: Write the failing rule guard tests**

Add to `abhyasa.rs`'s `tests` module. `slu_prakriya` already exists there.

```rust
    #[test]
    fn haladih_shesha_keeps_only_the_first_consonant_of_the_abhyasa() {
        // 7.4.60. √hrī's abhyāsa `hrI` loses its r: jihreti. This is the
        // ONLY cluster-initial row in the whole gaṇa, so this test is the
        // rule's only witness and no later slice adds a second.
        let mut p = slu_prakriya("hrI", "ti");
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        assert!((r_10.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "hrI");
        let r_60 = rules().find(|r| r.id == "7.4.60").unwrap();
        assert!((r_60.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "hI");
        assert_eq!(p.terms[ANGA].text, "hrI", "the aṅga is untouched");
        assert_eq!(p.log.last().unwrap().sutra, "7.4.60");
    }

    #[test]
    fn haladih_shesha_records_nothing_for_a_single_initial_consonant() {
        // The no-op guard. √hu, √ki and √bhī all have one initial consonant,
        // so 7.4.60 must return false and leave the log empty — otherwise
        // every 3a trace grows a step and the 3564 priors break.
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        let r_60 = rules().find(|r| r.id == "7.4.60").unwrap();
        for root in ["hu", "ki", "BI"] {
            let mut p = slu_prakriya(root, "ti");
            assert!((r_10.apply)(&mut p));
            p.log.clear();
            assert!(!(r_60.apply)(&mut p), "{root}");
            assert_eq!(p.terms[ABHYASA].text, root, "{root}");
            assert!(p.log.is_empty(), "{root}");
        }
    }

    #[test]
    fn haladih_shesha_declines_for_a_vowel_initial_abhyasa() {
        // 3d's √ṛ is the vowel-initial row. *halādiḥ* names a consonant, so
        // the rule has nothing to keep and must not touch the term.
        let mut p = slu_prakriya("f", "ti");
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        assert!((r_10.apply)(&mut p));
        p.log.clear();
        let r_60 = rules().find(|r| r.id == "7.4.60").unwrap();
        assert!(!(r_60.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "f");
        assert!(p.log.is_empty());
    }

    #[test]
    fn hrasvah_shortens_the_abhyasa_vowel_and_leaves_the_anga_long() {
        // 7.4.59. √bhī: BI → Bi, with the aṅga's own I untouched — that is
        // what lets 6.4.82 fire on a long I later (bibhyati).
        let mut p = slu_prakriya("BI", "ti");
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        assert!((r_10.apply)(&mut p));
        let r_59 = rules().find(|r| r.id == "7.4.59").unwrap();
        assert!((r_59.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "Bi");
        assert_eq!(p.terms[ANGA].text, "BI", "the aṅga keeps its long vowel");
        assert_eq!(p.log.last().unwrap().sutra, "7.4.59");
    }

    #[test]
    fn hrasvah_records_nothing_for_an_already_short_abhyasa() {
        // The no-op guard, on 3a's two roots.
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        let r_59 = rules().find(|r| r.id == "7.4.59").unwrap();
        for root in ["hu", "ki"] {
            let mut p = slu_prakriya(root, "ti");
            assert!((r_10.apply)(&mut p));
            p.log.clear();
            assert!(!(r_59.apply)(&mut p), "{root}");
            assert_eq!(p.terms[ABHYASA].text, root, "{root}");
            assert!(p.log.is_empty(), "{root}");
        }
    }

    #[test]
    fn haladih_shesha_runs_before_hrasvah_and_both_before_kuhoshcuh() {
        // √hrī end to end through this stage: hrI → hI → hi → Ji, which
        // 8.4.54 finishes as `ji` in the tripādī. The order is vidyut's; the
        // forms agree under 7.4.60/7.4.59 either way, so this is the pin
        // that makes the order a checked fact rather than an accident.
        let mut p = slu_prakriya("hrI", "ti");
        for id in ["6.1.10", "7.4.60", "7.4.59", "7.4.62"] {
            let r = rules().find(|r| r.id == id).unwrap();
            assert!((r.apply)(&mut p), "{id}");
        }
        assert_eq!(p.terms[ABHYASA].text, "Ji");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra).collect();
        assert_eq!(ids, vec!["6.1.10", "7.4.60", "7.4.59", "7.4.62"]);
    }
```

- [ ] **Step 6: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya abhyasa 2>&1 | tail -30`
Expected: FAIL — `rules().find(|r| r.id == "7.4.60").unwrap()` panics on `None`.

- [ ] **Step 7: Add the two rules to `ABHYASA_RULES`**

Insert between the `6.1.10` rule and the `7.4.62` rule. Extend the imports at the top of `abhyasa.rs` to `use crate::tinanta::sound::{cutva_of, hrasva_of, is_vowel};`.

```rust
    // 7.4.60 halādiḥ śeṣaḥ: of the abhyāsa's initial consonant cluster only
    // the first consonant remains. hrI → hI, which 7.4.59 then shortens to
    // hi and 7.4.62 palatalizes to Ji (jihreti).
    //
    // √hrī is the ONLY cluster-initial row in the whole gaṇa — the other
    // twenty-five all begin with a single consonant or a vowel — so this
    // rule will never gain a second witness, and
    // `haladih_shesha_keeps_only_the_first_consonant_of_the_abhyasa` carries
    // the load a second root would otherwise share.
    //
    // The no-op guard is 8.4.53's: for a single-consonant abhyāsa the result
    // equals the input, and the rule must record nothing there or every √hu
    // and √ki trace grows a step and the 3564 priors break.
    Rule {
        id: "7.4.60",
        name: "halAdiH SezaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let s: Vec<char> = p.terms[ABHYASA].text.chars().collect();
            let Some(&first) = s.first() else {
                return false;
            };
            // *halādiḥ* names a consonant-initial abhyāsa. 3d's √ṛ is the
            // vowel-initial row and must fall through untouched.
            if is_vowel(first) {
                return false;
            }
            let tail: String = s.iter().skip(1).skip_while(|c| !is_vowel(**c)).collect();
            let t = format!("{first}{tail}");
            if t == p.terms[ABHYASA].text {
                return false;
            }
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("7.4.60", "halAdiH SezaH", before);
            true
        },
    },
    // 7.4.59 hrasvaḥ: the abhyāsa's vowel becomes hrasva. BI → Bi
    // (bibheti), and hI → hi on 7.4.60's output (jihreti). The AṄGA's own
    // vowel is untouched — that is what leaves √bhī a long I for 6.4.82 to
    // find (bibhyati) and what 6.4.115 optionally shortens later.
    //
    // Ordered AFTER 7.4.60 (vidyut's order; the forms agree either way, so
    // the trace pins are what hold it) and BEFORE 7.4.62.
    //
    // Same no-op guard: √hu and √ki are already hrasva, and `hrasva_of`
    // returning None for a short vowel is what makes the guard a one-lookup
    // test. See that function for why the ec arm of 1.1.48 is absent.
    Rule {
        id: "7.4.59",
        name: "hrasvaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let t: String = p.terms[ABHYASA]
                .text
                .chars()
                .map(|c| hrasva_of(c).unwrap_or(c))
                .collect();
            if t == p.terms[ABHYASA].text {
                return false;
            }
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("7.4.59", "hrasvaH", before);
            true
        },
    },
```

- [ ] **Step 8: Update the pinned rule order**

In `derivation_tests.rs`'s `tinanta_rule_order_is_pinned`, change `"6.1.10", "7.4.62",` to `"6.1.10", "7.4.60", "7.4.59", "7.4.62",`.

- [ ] **Step 9: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS, all of them.

- [ ] **Step 10: Run the full suite and confirm the priors are byte-identical**

Run: `mise run test 2>&1 | tail -30` (foreground; ~38 minutes — set an explicit long timeout)
Expected: PASS, 3564 cells, no golden changed. The two new rules fire on no curated root yet; if any golden moves, a no-op guard is missing.

- [ ] **Step 11: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/sound.rs \
        crates/panini-prakriya/src/tinanta/abhyasa.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(engine): 7.4.60 halādiḥ śeṣaḥ and 7.4.59 hrasvaḥ

The abhyāsa's own shaping rules, between 6.1.10 and 7.4.62. Both carry
8.4.53's no-op guard so √hu's and √ki's traces are unchanged — the 3564
priors are byte-identical.

7.4.60 has exactly one witness in the whole gaṇa: √hrī is its only
cluster-initial row. 7.4.59's substitute is a new sound-table map,
hrasva_of, whose unwitnessed arms (A, U, F, X) are killed by a direct
table test rather than left waiting for slices 3c and 3d. The ec arm of
1.1.48 is deliberately absent: this engine copies the bare root before
guṇa, so no abhyāsa carries an ec when 7.4.59 fires."
```

---

## Task 3: 6.4.82's long `I` and 6.4.77's iyaṅ arm

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (6.4.82's guard, 6.4.77's body and comment, guard tests)

**Interfaces:**
- Consumes: `ABHYASA`, `ANGA`, `SHAP`, `ENDING`, `following_sarvadhatuka` from `tinanta::terms`; `is_vowel`.
- Produces: no new symbols. 6.4.82 accepts a final `I`; 6.4.77 rewrites `ANGA` for an `I`-final aṅga before a vowel.

- [ ] **Step 1: Write the failing guard tests**

Add to `guna.rs`'s `tests` module.

```rust
    #[test]
    fn er_anekacah_fires_on_a_long_i_as_well_as_a_short_one() {
        // 6.4.82. The sūtra's *eḥ* denotes both lengths by 1.1.69/1.1.70
        // aṇudit savarṇasya. √ki's `ciki` (short, slice 3a) and √bhī's
        // `BiBI` (long, this slice) are the two witnesses, so neither
        // branch of the length test is a free mutant.
        let rule = rules().find(|r| r.id == "6.4.82").unwrap();
        for (abhyasa, anga, want) in [("ci", "ki", "ky"), ("Bi", "BI", "By")] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new(anga), Term::new(""), Term::new("ati")]),
                ..Default::default()
            };
            p.terms[ABHYASA].text = abhyasa.into();
            assert!((rule.apply)(&mut p), "{anga}");
            assert_eq!(p.terms[ANGA].text, want, "{anga}");
        }
    }

    #[test]
    fn er_anekacah_declines_when_a_conjunct_precedes_the_long_i() {
        // √hrī: the abhyasta span is `Ji` + `hrI`, so the two sounds before
        // the final I are `r` then `h` — saṁyogapūrva. 6.4.82 must decline
        // and leave the cell to 6.4.77, which is the whole reason the two
        // roots of this slice take different rules for the same shape.
        let rule = rules().find(|r| r.id == "6.4.82").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("hrI"), Term::new(""), Term::new("ati")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "Ji".into();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "hrI");
        assert!(p.log.is_empty());
    }

    #[test]
    fn aci_dhatoh_gives_iyan_for_an_i_final_anga_before_a_vowel() {
        // 6.4.77's dhātu arm: hrI + ati → hriy + ati (jihriyati). General
        // over roots, not keyed to √hrī — the sūtra names no root.
        let rule = rules().find(|r| r.id == "6.4.77").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("hrI"), Term::new(""), Term::new("ati")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "Ji".into();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "hriy");
        assert_eq!(p.log.last().unwrap().sutra, "6.4.77");
    }

    #[test]
    fn aci_dhatoh_declines_before_a_consonant_and_for_a_short_i() {
        // Both clauses of the new arm, separately falsified. `tas` is
        // hal-initial (jihrItaH keeps its I); `ciki` is i-final, not
        // I-final, and belongs to 6.4.82.
        let rule = rules().find(|r| r.id == "6.4.77").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("hrI"), Term::new(""), Term::new("tas")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "Ji".into();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "hrI");

        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ki"), Term::new(""), Term::new("ati")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "ci".into();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "ki");
    }

    #[test]
    fn aci_shnu_arm_still_fires_after_the_dhatu_arm_is_added() {
        // Ap + nu + anti → Apnuvanti. The śnu arm is the one 6.4.77 has
        // always carried; adding the dhātu arm must not disturb it.
        let rule = rules().find(|r| r.id == "6.4.77").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Ap"), Term::new("nu"), Term::new("anti")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nuv");
        assert_eq!(p.terms[ANGA].text, "Ap", "the dhātu arm must not fire here");
    }
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya -- er_anekacah aci_ 2>&1 | tail -30`
Expected: FAIL — `er_anekacah_fires_on_a_long_i...` fails on the `BiBI` case; `aci_dhatoh_gives_iyan...` fails because 6.4.77 returns `false` for a non-`nu` SHAP.

- [ ] **Step 3: Widen 6.4.82's length test**

In the `6.4.82` rule body, replace:

```rust
            let mut anga: Vec<char> = p.terms[ANGA].text.chars().collect();
            if anga.last() != Some(&'i') {
                return false;
            }
```

with:

```rust
            let mut anga: Vec<char> = p.terms[ANGA].text.chars().collect();
            // *eḥ* denotes both lengths, by 1.1.69/1.1.70 aṇudit
            // savarṇasya — this is the sūtra's stated scope, not an
            // extension of it. √ki's `ciki` witnesses the short arm (slice
            // 3a, cikyati) and √bhī's `BiBI` the long one (slice 3b,
            // bibhyati); 7.4.59 shortened only the ABHYĀSA, so the aṅga's
            // own vowel is still long when this rule looks.
            if !matches!(anga.last(), Some('i') | Some('I')) {
                return false;
            }
```

- [ ] **Step 4: Add 6.4.77's dhātu arm**

Replace the `6.4.77` rule's comment paragraph that begins "Only the śnu arm is implemented." with:

```rust
    // Two arms now. The *śnu* arm rewrites SHAP; the *dhātu* arm rewrites
    // ANGA — hrI + ati → hriy + ati (jihriyati, slice 3b). The *bhrū* arm
    // has no root in scope.
    //
    // The dhātu arm is general over roots rather than keyed to √hrī: the
    // sūtra names no root. It is safe where it sits — after 7.3.84 and
    // after its apavāda 6.4.82 — because every ī/ū-final curated root is
    // already past it by then: √bhū and √nī have guṇated (`Bo`, `ne`, śap
    // being pit so 1.1.5 does not block), √śī has guṇated by 7.4.21, and
    // √vrī's follower is the hal-initial śnā. The 3564 byte-identical
    // priors are what turn that from an argument into a proof.
    //
    // THE UVAṄ HALF IS NOT WRITTEN. `U` → `uv` has no cell in the suite —
    // no juhotyādi row is ū-final — and an arm with no witness is a
    // guaranteed mutation survivor. Widen by arm, with a witness, exactly
    // as 6.1.78's three arms are. The restore trigger is the first ū-final
    // root that reaches this rule with guṇa blocked.
    //
    // The śnu arm reads terms[ENDING] directly, NOT `following_sarvadhatuka`:
    // that helper answers "what follows the aṅga", which there is śnu itself
    // — it needs what follows śnu. The dhātu arm's follower IS what follows
    // the aṅga, so it uses the helper. Same reasoning as 6.4.112/6.4.113.
```

and replace the rule's `apply` body with:

```rust
        apply: |p| {
            // The śnu arm: the vikaraṇa's `u` becomes uvaṅ.
            if p.terms[SHAP].text == "nu" {
                let Some(next) = p.terms[ENDING].text.chars().next() else {
                    return false;
                };
                if !is_vowel(next) {
                    return false;
                }
                let before = p.snapshot();
                p.terms[SHAP].text = "nuv".into();
                p.record("6.4.77", "aci SnuDAtuBruvAM yvoriyaNuvaNO", before);
                return true;
            }
            // The dhātu arm: an ī-final aṅga becomes iyaṅ before a vowel.
            if !p.terms[ANGA].text.ends_with('I') {
                return false;
            }
            let Some(next) = following_sarvadhatuka(p).and_then(|t| t.text.chars().next()) else {
                return false;
            };
            if !is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            let stem: String = p.terms[ANGA].text.chars().rev().skip(1).collect::<Vec<_>>()
                .into_iter().rev().collect();
            p.terms[ANGA].text = format!("{stem}iy");
            p.record("6.4.77", "aci SnuDAtuBruvAM yvoriyaNuvaNO", before);
            true
        },
```

- [ ] **Step 5: Run the guard tests to verify they pass**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS.

- [ ] **Step 6: Run the full suite and confirm the priors are byte-identical**

Run: `mise run test 2>&1 | tail -30` (foreground; ~38 minutes)
Expected: PASS, 3564 cells, no golden changed. **This is the step that proves the widening reaches nothing it should not.** If a svādi/bhvādi/adādi/kryādi golden moves, 6.4.77's dhātu arm is too wide — stop and re-read the four-root table above rather than narrowing the guard blind.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/guna.rs
git commit -m "feat(engine): 6.4.82 on a long I, and 6.4.77's iyaṅ arm

6.4.82's *eḥ* denotes both lengths by 1.1.69/1.1.70; √ki witnesses the
short arm and √bhī the long one, so neither is a free mutant. What 6.4.82
declines for a conjunct, 6.4.77 takes: hrI + ati → hriy + ati.

The dhātu arm is general over roots — the sūtra names none — and safe
where 6.4.77 already sits, after 7.3.84 and after its apavāda: all four
ī/ū-final curated roots are past it (guṇa, or a hal follower). The 3564
byte-identical priors are the proof. The uvaṅ half stays unwritten: no
ū-final root is in scope and an unwitnessed arm is a mutation survivor."
```

---

## Task 4: 6.4.115 *bhiyo'nyatarasyām*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (new rule immediately after `6.4.113`)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`, `exactly_the_pinned_vikalpa_rules_are_optional`)

**Interfaces:**
- Consumes: `ANGA`, `following_sarvadhatuka`; `Tag::Ngit`; `is_vowel`.
- Produces: rule `"6.4.115"` in `GUNA_RULES`, `vikalpa: true`, positioned immediately after `"6.4.113"`.

- [ ] **Step 1: Write the failing guard tests**

Add to `guna.rs`'s `tests` module. A small helper keeps the four cases honest.

```rust
    /// √bhī under ślu with a given follower: aṅga `BI`, empty śap, the
    /// ending. `ngit` sets 1.2.4's tag on the ending.
    fn bhi_slu_prakriya(ending: &str, ngit: bool) -> Prakriya {
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("BI"), Term::new(""), Term::new(ending)]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "Bi".into();
        if ngit {
            p.terms[ENDING].add(Tag::Ngit);
        }
        p
    }

    #[test]
    fn bhiyah_anyatarasyam_shortens_before_a_hal_initial_kngit() {
        // 6.4.115. bibhītaḥ / bibhitaḥ. The abhyāsa is already `Bi` (7.4.59);
        // this rule shortens the AṄGA.
        let rule = rules().find(|r| r.id == "6.4.115").unwrap();
        let mut p = bhi_slu_prakriya("tas", true);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Bi");
        assert_eq!(p.terms[ABHYASA].text, "Bi", "the abhyāsa is untouched");
        assert_eq!(p.log.last().unwrap().sutra, "6.4.115");
    }

    #[test]
    fn bhiyah_anyatarasyam_declines_before_a_pit_ending() {
        // The *kṅiti* clause, falsified. `si` and `mi` are hal-initial but
        // PIT, so 1.2.4 never made them ṅit and guṇa applies instead:
        // bibheṣi, bibhemi — never *bibhiṣi. Dropping the kṅit test would
        // produce those.
        let rule = rules().find(|r| r.id == "6.4.115").unwrap();
        for ending in ["si", "mi"] {
            let mut p = bhi_slu_prakriya(ending, false);
            assert!(!(rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ANGA].text, "BI", "{ending}");
            assert!(p.log.is_empty(), "{ending}");
        }
    }

    #[test]
    fn bhiyah_anyatarasyam_declines_before_a_vowel_initial_kngit() {
        // The *hali* clause, falsified. `ati` IS kṅit but ajādi, and
        // bibhyati has no second form. Dropping the hal test would produce
        // *bibhiyati.
        let rule = rules().find(|r| r.id == "6.4.115").unwrap();
        let mut p = bhi_slu_prakriya("ati", true);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "BI");
        assert!(p.log.is_empty());
    }

    #[test]
    fn bhiyah_anyatarasyam_is_keyed_to_bhi_alone() {
        // The sūtra names the root, so the guard does too — with no gaṇa
        // clause beside it, which could never be falsified (the 6.4.87 /
        // 6.4.101 precedent). √hrī has the same shape and the same follower
        // and must NOT fork: jihrItaH has one form.
        let rule = rules().find(|r| r.id == "6.4.115").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("hrI"), Term::new(""), Term::new("tas")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "Ji".into();
        p.terms[ENDING].add(Tag::Ngit);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "hrI");
    }

    #[test]
    fn bhiyah_anyatarasyam_reads_the_yasut_prefixed_ending_in_vidhilin() {
        // vidhiliṅ's follower is the ending with yāsuṭ prefixed onto its own
        // text and Ngit set there (3.4.103), so `following_sarvadhatuka`
        // returns `yAt` — hal-initial and ṅit. All nine vidhiliṅ cells fork:
        // bibhīyāt / bibhiyāt.
        let rule = rules().find(|r| r.id == "6.4.115").unwrap();
        let mut p = bhi_slu_prakriya("yAt", true);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Bi");
    }
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya bhiyah 2>&1 | tail -20`
Expected: FAIL — `rules().find(|r| r.id == "6.4.115").unwrap()` panics on `None`.

- [ ] **Step 3: Add the rule immediately after `6.4.113`**

```rust
    // 6.4.115 bhiyo'nyatarasyām: √bhī's ī optionally becomes hrasva.
    // bibhītaḥ / bibhitaḥ, bibhīhi / bibhihi, bibhīyāt / bibhiyāt. The
    // engine's ninth vikalpa.
    //
    // Operates on the AṄGA. 7.4.59 has already shortened the abhyāsa, and
    // that is a different vowel: `Bi` + `BI` is the pair this rule turns
    // into `Bi` + `Bi`.
    //
    // ROOT-KEYED, with no gaṇa clause — the sūtra itself names √bhī, and a
    // gaṇa clause beside `ANGA.text == "BI"` could never be falsified (the
    // 6.4.87 / 6.4.101 precedent). √hrī has the same shape and the same
    // followers and must not fork.
    //
    // *hali* and *kṅiti* both come by anuvṛtti from 6.4.113, and BOTH are
    // load-bearing on this slice's own cells, which is why both are written:
    //   - drop the kṅit test and `si`/`mi` (hal-initial but PIT) give
    //     *bibhiṣi / *bibhimi instead of bibheṣi / bibhemi;
    //   - drop the hal test and `ati` (kṅit but ajādi) gives *bibhiyati
    //     instead of bibhyati, which is 6.4.82's.
    //
    // Reads `following_sarvadhatuka`, unlike 6.4.112/6.4.113 beside it:
    // those need what follows śnā, this needs what follows the AṄGA. Under
    // ślu the śap is empty, so the helper returns the ending — and in
    // vidhiliṅ that ending carries yāsuṭ prefixed onto its own text and
    // Ngit set there by 3.4.103, so all nine of those cells fork.
    //
    // POSITION IS NOT FORCED BY ANY CELL: wherever guṇa fires the kṅit test
    // declines, and where 6.4.82 fires first this rule sees the `By` it left
    // and declines on the text test. It sits beside 6.4.113, the rule it
    // takes its two anuvṛtti conditions from, and the bibhitaḥ trace pin is
    // what holds it there.
    Rule {
        id: "6.4.115",
        name: "Biyo'nyatarasyAm",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            if p.terms[ANGA].text != "BI" {
                return false;
            }
            let Some(follower) = following_sarvadhatuka(p) else {
                return false;
            };
            if !follower.has(Tag::Ngit) {
                return false;
            }
            let Some(next) = follower.text.chars().next() else {
                return false;
            };
            if is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = "Bi".into();
            p.record("6.4.115", "Biyo'nyatarasyAm", before);
            true
        },
    },
```

- [ ] **Step 4: Update both pinned lists**

In `derivation_tests.rs`:
- `tinanta_rule_order_is_pinned`: change `"6.4.112", "6.4.113", "6.1.101",` to `"6.4.112", "6.4.113", "6.4.115", "6.1.101",`.
- `exactly_the_pinned_vikalpa_rules_are_optional`: add `"6.4.115"` to the expected set (it becomes the ninth, beside `8.2.74`, `8.2.75`, `8.4.65`, `8.4.56`, `6.4.107`, `7.3.86`, `3.4.111`, `7.1.35`).

- [ ] **Step 5: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS.

- [ ] **Step 6: Run the full suite and confirm the priors are byte-identical**

Run: `mise run test 2>&1 | tail -30` (foreground; ~38 minutes)
Expected: PASS, 3564 cells. No curated root reads `BI` yet, so the rule fires nowhere.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/guna.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(engine): 6.4.115 bhiyo'nyatarasyām, the ninth vikalpa

√bhī's ī optionally shortens before a hal-initial kṅit. Root-keyed with no
gaṇa clause, the 6.4.87/6.4.101 precedent. Both anuvṛtti conditions from
6.4.113 are written because both are falsifiable on this slice's cells:
si/mi are hal-initial but pit, and ati is kṅit but ajādi.

Position is not forced by any cell; it sits beside 6.4.113, whose two
conditions it inherits."
```

---

## Task 5: The data and the paradigm goldens

This is the task that turns 72 new cells green.

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (two `Dhatu` rows; extend `juhotyadi_rows_are_the_two_curated_roots`)
- Modify: `crates/panini/tests/paradigm/data/juhotyadi.rs` (8 `PARADIGM` blocks, 40 `ALTERNATES` rows)
- Modify: `crates/panini/tests/paradigm/main.rs` (corpus totals)

**Interfaces:**
- Consumes: everything from Tasks 1–4.
- Produces: `dhatus()` rows `03.0002` (`BI`) and `03.0003` (`hrI`), both `PadaAssignment::Parasmaipada`, `Gana::Juhotyadi`.

- [ ] **Step 1: Add the two `Dhatu` rows**

In `panini-data/src/lib.rs`'s `DHATUS`, after the `03.0001` row:

```rust
    Dhatu {
        // 03.0002 `YiBI\` Baye. The initial ñi is an it by 1.3.5 and
        // decides no pada — see `pada_from_upadesha`. Parasmaipada by
        // 1.3.78. Its ī is what 6.4.115 optionally shortens (bibhītaḥ /
        // bibhitaḥ) and what 6.4.82 turns to y before a vowel (bibhyati):
        // `BiBI` is asaṁyogapūrva where √hrī's `JihrI` is not.
        dhatupatha: "03.0002",
        code: "BI",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "Baye",
    },
    Dhatu {
        // 03.0003 `hrI\` lajjAyAm. The gaṇa's only cluster-initial root, so
        // its abhyāsa is 7.4.60's only witness anywhere: hrI → hI → hi → Ji
        // → ji. The conjunct is also why 6.4.82 declines and 6.4.77's iyaṅ
        // arm takes the cell instead (jihriyati).
        dhatupatha: "03.0003",
        code: "hrI",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "lajjAyAm",
    },
```

- [ ] **Step 2: Extend the gaṇa-row test**

Rename `juhotyadi_rows_are_the_two_curated_roots` to `juhotyadi_rows_are_the_four_curated_roots`, update its comment to say the gaṇa stands at 4 of 26 rows and that 3c–3f close it, and change the expected vector to:

```rust
            vec![
                ("03.0001", "hu", PadaAssignment::Parasmaipada),
                ("03.0002", "BI", PadaAssignment::Parasmaipada),
                ("03.0003", "hrI", PadaAssignment::Parasmaipada),
                ("03.0020", "ki", PadaAssignment::Parasmaipada),
            ]
```

Keep the existing `code`-uniqueness tripwire paragraph and extend it: 6.4.115 now identifies √bhī by `ANGA.text == "BI"` on the same premise, so `BI` must stay unique too.

**Note on row order:** `DHATUS` is ordered by `dhatupatha` number, so `03.0002` and `03.0003` sort before the existing `03.0020`. Insert them in numeric position, not at the end.

- [ ] **Step 3: Run the data tests to verify they pass**

Run: `mise exec -- cargo test -p panini-data 2>&1 | tail -20`
Expected: PASS, including `curated_pada_agrees_with_upadesha_markers` — which now exercises Task 1's correction for real.

- [ ] **Step 4: Add the `PARADIGM` blocks**

Append to `PARADIGM` in `crates/panini/tests/paradigm/data/juhotyadi.rs`. Transcribed from the spec's appendix (vidyut at `8da2f90b`); `mise run fmt` will re-wrap the arrays.

```rust
    (
        "03.0002",
        "laT",
        Pada::Parasmaipada,
        [
            "biBeti", "biBItaH", "biByati", "biBezi", "biBITaH", "biBITa", "biBemi", "biBIvaH", "biBImaH",
        ],
    ),
    (
        "03.0002",
        "laN",
        Pada::Parasmaipada,
        [
            "abiBed", "abiBItAm", "abiBayuH", "abiBeH", "abiBItam", "abiBIta", "abiBayam", "abiBIva", "abiBIma",
        ],
    ),
    (
        "03.0002",
        "loT",
        Pada::Parasmaipada,
        [
            "biBetu", "biBItAm", "biByatu", "biBIhi", "biBItam", "biBIta", "biBayAni", "biBayAva", "biBayAma",
        ],
    ),
    (
        "03.0002",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "biBIyAd", "biBIyAtAm", "biBIyuH", "biBIyAH", "biBIyAtam", "biBIyAta", "biBIyAm", "biBIyAva", "biBIyAma",
        ],
    ),
    (
        "03.0003",
        "laT",
        Pada::Parasmaipada,
        [
            "jihreti", "jihrItaH", "jihriyati", "jihrezi", "jihrITaH", "jihrITa", "jihremi", "jihrIvaH", "jihrImaH",
        ],
    ),
    (
        "03.0003",
        "laN",
        Pada::Parasmaipada,
        [
            "ajihred", "ajihrItAm", "ajihrayuH", "ajihreH", "ajihrItam", "ajihrIta", "ajihrayam", "ajihrIva", "ajihrIma",
        ],
    ),
    (
        "03.0003",
        "loT",
        Pada::Parasmaipada,
        [
            "jihretu", "jihrItAm", "jihriyatu", "jihrIhi", "jihrItam", "jihrIta", "jihrayARi", "jihrayAva", "jihrayAma",
        ],
    ),
    (
        "03.0003",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "jihrIyAd", "jihrIyAtAm", "jihrIyuH", "jihrIyAH", "jihrIyAtam", "jihrIyAta", "jihrIyAm", "jihrIyAva", "jihrIyAma",
        ],
    ),
```

- [ ] **Step 5: Add the `ALTERNATES` rows**

Append to `ALTERNATES` in the same file. The index column is the 0-based cell within the nine (P.E P.D P.B M.E M.D M.B U.E U.D U.B); the key names the optional rules that produced the form, in pipeline order.

```rust
    ("03.0002", "laT", Pada::Parasmaipada, 1, "biBitaH", "6.4.115"),
    ("03.0002", "laT", Pada::Parasmaipada, 4, "biBiTaH", "6.4.115"),
    ("03.0002", "laT", Pada::Parasmaipada, 5, "biBiTa", "6.4.115"),
    ("03.0002", "laT", Pada::Parasmaipada, 7, "biBivaH", "6.4.115"),
    ("03.0002", "laT", Pada::Parasmaipada, 8, "biBimaH", "6.4.115"),
    ("03.0002", "laN", Pada::Parasmaipada, 0, "abiBet", "8.4.56"),
    ("03.0002", "laN", Pada::Parasmaipada, 1, "abiBitAm", "6.4.115"),
    ("03.0002", "laN", Pada::Parasmaipada, 4, "abiBitam", "6.4.115"),
    ("03.0002", "laN", Pada::Parasmaipada, 5, "abiBita", "6.4.115"),
    ("03.0002", "laN", Pada::Parasmaipada, 7, "abiBiva", "6.4.115"),
    ("03.0002", "laN", Pada::Parasmaipada, 8, "abiBima", "6.4.115"),
    ("03.0002", "loT", Pada::Parasmaipada, 0, "biBItAd", "7.1.35"),
    ("03.0002", "loT", Pada::Parasmaipada, 0, "biBitAd", "7.1.35+6.4.115"),
    ("03.0002", "loT", Pada::Parasmaipada, 0, "biBitAt", "7.1.35+6.4.115+8.4.56"),
    ("03.0002", "loT", Pada::Parasmaipada, 0, "biBItAt", "7.1.35+8.4.56"),
    ("03.0002", "loT", Pada::Parasmaipada, 1, "biBitAm", "6.4.115"),
    ("03.0002", "loT", Pada::Parasmaipada, 3, "biBihi", "6.4.115"),
    ("03.0002", "loT", Pada::Parasmaipada, 3, "biBItAd", "7.1.35"),
    ("03.0002", "loT", Pada::Parasmaipada, 3, "biBitAd", "7.1.35+6.4.115"),
    ("03.0002", "loT", Pada::Parasmaipada, 3, "biBitAt", "7.1.35+6.4.115+8.4.56"),
    ("03.0002", "loT", Pada::Parasmaipada, 3, "biBItAt", "7.1.35+8.4.56"),
    ("03.0002", "loT", Pada::Parasmaipada, 4, "biBitam", "6.4.115"),
    ("03.0002", "loT", Pada::Parasmaipada, 5, "biBita", "6.4.115"),
    ("03.0002", "viDiliN", Pada::Parasmaipada, 0, "biBiyAd", "6.4.115"),
    ("03.0002", "viDiliN", Pada::Parasmaipada, 0, "biBiyAt", "6.4.115+8.4.56"),
    ("03.0002", "viDiliN", Pada::Parasmaipada, 0, "biBIyAt", "8.4.56"),
    ("03.0002", "viDiliN", Pada::Parasmaipada, 1, "biBiyAtAm", "6.4.115"),
    ("03.0002", "viDiliN", Pada::Parasmaipada, 2, "biBiyuH", "6.4.115"),
    ("03.0002", "viDiliN", Pada::Parasmaipada, 3, "biBiyAH", "6.4.115"),
    ("03.0002", "viDiliN", Pada::Parasmaipada, 4, "biBiyAtam", "6.4.115"),
    ("03.0002", "viDiliN", Pada::Parasmaipada, 5, "biBiyAta", "6.4.115"),
    ("03.0002", "viDiliN", Pada::Parasmaipada, 6, "biBiyAm", "6.4.115"),
    ("03.0002", "viDiliN", Pada::Parasmaipada, 7, "biBiyAva", "6.4.115"),
    ("03.0002", "viDiliN", Pada::Parasmaipada, 8, "biBiyAma", "6.4.115"),
    ("03.0003", "laN", Pada::Parasmaipada, 0, "ajihret", "8.4.56"),
    ("03.0003", "loT", Pada::Parasmaipada, 0, "jihrItAd", "7.1.35"),
    ("03.0003", "loT", Pada::Parasmaipada, 0, "jihrItAt", "7.1.35+8.4.56"),
    ("03.0003", "loT", Pada::Parasmaipada, 3, "jihrItAd", "7.1.35"),
    ("03.0003", "loT", Pada::Parasmaipada, 3, "jihrItAt", "7.1.35+8.4.56"),
    ("03.0003", "viDiliN", Pada::Parasmaipada, 0, "jihrIyAt", "8.4.56"),
```

- [ ] **Step 6: Update the corpus totals**

In `crates/panini/tests/paradigm/main.rs`:
- `assert_eq!(total_cells, 3564, ...)` becomes `3636`, and the message `"396 root×lakāra blocks × 9 cells each"` becomes `"404 root×lakāra blocks × 9 cells each"`.
- `assert_eq!(ALTERNATES.len(), 919, "ALTERNATES row count")` becomes `959`.
- `derivation_set_shape_matches_the_audited_numbers`: 3636 cells / 4595 forms / 81 roots.

- [ ] **Step 7: Run the full suite**

Run: `mise run test 2>&1 | tail -40` (foreground; expect ~39 minutes at 3636 cells)
Expected: PASS at 3636 cells / 4595 forms / 81 roots. Every 3564 prior unchanged.

If a new cell fails, the engine and the goldens disagree — read the failing form against the spec's appendix table before touching either. Do **not** edit a golden to match the engine; the goldens are vidyut's output and are the specification.

- [ ] **Step 8: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-data/src/lib.rs \
        crates/panini/tests/paradigm/data/juhotyadi.rs \
        crates/panini/tests/paradigm/main.rs
git commit -m "feat(data): √bhī and √hrī — juhotyādi at four of twenty-six rows

3564 → 3636 cells, 4483 → 4595 forms, 919 → 959 ALTERNATES, 79 → 81 roots.

√bhī's 70 forms are √hrī's 42 plus the 28 6.4.115 forks off, which is the
arithmetic check that the vikalpa is the whole difference between two
otherwise identical paradigms. 6.4.115 opens four ALTERNATES keys:
6.4.115 (23 rows), 7.1.35+6.4.115 (2), 7.1.35+6.4.115+8.4.56 (2) and
6.4.115+8.4.56 (1)."
```

---

## Task 6: The five trace pins

**Files:**
- Modify: `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes: `cell_trace`, `at` from `crate::helpers` (already imported in that file).

- [ ] **Step 1: Write the five pins**

Append to `crates/panini/tests/trace/juhotyadi.rs`:

```rust
#[test]
fn jihreti_trace_orders_haladih_shesha_before_hrasvah_before_kuhoshcuh() {
    // hrI laT P.E. The abhyāsa's whole shaping chain in pipeline order:
    // hrI → hI (7.4.60) → hi (7.4.59) → Ji (7.4.62) → ji (8.4.54).
    let (text, t) = cell_trace(
        "03.0003",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "jihreti", "got {t:?}");
    assert!(at(&t, "6.1.10") < at(&t, "7.4.60"), "got {t:?}");
    assert!(at(&t, "7.4.60") < at(&t, "7.4.59"), "got {t:?}");
    assert!(at(&t, "7.4.59") < at(&t, "7.4.62"), "got {t:?}");
    assert!(at(&t, "7.4.62") < at(&t, "8.4.54"), "got {t:?}");
}

#[test]
fn jihriyati_trace_credits_6_4_77_and_not_6_4_82() {
    // hrI laT P.B. The abhyasta span is `Ji` + `hrI`, so the two sounds
    // before the final I are r then h — saṁyogapūrva. 6.4.82 declines and
    // its utsarga 6.4.77 takes the cell.
    let (text, t) = cell_trace(
        "03.0003",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "jihriyati", "got {t:?}");
    assert!(t.contains(&"6.4.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.82".to_string()), "got {t:?}");
    assert!(at(&t, "7.1.4") < at(&t, "6.4.77"), "got {t:?}");
}

#[test]
fn bibhyati_trace_credits_6_4_82_on_a_long_i_and_no_6_4_115() {
    // BI laT P.B. 7.4.59 shortened the ABHYĀSA only, so 6.4.82 fires on a
    // long I. 6.4.115 is absent because `ati` is ajādi — the *hali*
    // clause's witness, and why this cell has exactly one form.
    let (text, t) = cell_trace(
        "03.0002",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "biByati", "got {t:?}");
    assert!(at(&t, "7.4.59") < at(&t, "6.4.82"), "got {t:?}");
    assert!(!t.contains(&"6.4.115".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.4.60".to_string()), "got {t:?}");
}

#[test]
fn bibhitah_trace_credits_6_4_115_after_the_abhyasa_is_shortened() {
    // BI laT P.D, the shortened arm. 7.4.59 shortens the abhyāsa (Bi) and
    // 6.4.115 then shortens the aṅga (Bi) — two different vowels, two
    // different rules, in that order.
    let (text, t) = cell_trace(
        "03.0002",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "biBItaH", "got {t:?}");
    assert!(!t.contains(&"6.4.115".to_string()), "got {t:?}");
    // The forked arm is the one 6.4.115 produces; `cell_trace` returns the
    // unforked derivation, so the fork itself is pinned by the ALTERNATES
    // row `("03.0002", "laT", Pada::Parasmaipada, 1, "biBitaH", "6.4.115")`
    // that Task 5 added.
    assert!(at(&t, "6.1.10") < at(&t, "7.4.59"), "got {t:?}");
}

#[test]
fn jihrayani_trace_credits_8_4_2_across_the_intervening_sounds() {
    // hrI loT U.E. The r of `hray`, then a, y, A, then ni → Ri. All three
    // interveners are aṭ, which `is_natva_intervener` already carries; this
    // is ṇatva's first abhyāsa-bearing word and it needed no edit.
    let (text, t) = cell_trace(
        "03.0003",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "jihrayARi", "got {t:?}");
    assert!(t.contains(&"8.4.2".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.1".to_string()), "got {t:?}");
    assert!(at(&t, "6.1.78") < at(&t, "8.4.2"), "got {t:?}");
}
```

- [ ] **Step 2: Run the trace suite**

Run: `mise exec -- cargo test -p panini --test trace 2>&1 | tail -30`
Expected: PASS. This binary is ~4s, so it can be run alone.

If `bibhitah_trace...`'s `assert_eq!(text, "biBItaH")` fails with `biBitaH`, `cell_trace` returns the forked arm rather than the base — swap the expectation and the `!t.contains("6.4.115")` assertion accordingly and record which arm it returns in the comment.

- [ ] **Step 3: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini/tests/trace/juhotyadi.rs
git commit -m "test(trace): five 3b pins — the abhyāsa chain, the 6.4.82/6.4.77 split, ṇatva

jihreti pins 7.4.60 < 7.4.59 < 7.4.62 < 8.4.54; jihriyati pins that
6.4.82 declines for the conjunct and 6.4.77 takes the cell; bibhyati pins
6.4.82 firing on a long I with no 6.4.115 (the hali clause's witness);
bibhitaḥ pins 7.4.59 on the abhyāsa before 6.4.115 on the aṅga;
jihrayāṇi pins 8.4.2 across a, y and A."
```

---

## Task 7: Audit, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (prose)

- [ ] **Step 1: Repoint the audit harness and run it**

`/tmp/vidyut-full/vidyut-prakriya/Cargo.toml`'s dev-deps hardcode an absolute path and currently point at a **deleted** worktree. Repoint them at this one first, or the audit silently checks the wrong tree (or fails to resolve at all):

```bash
sed -i 's#^panini = { path = .*#panini = { path = "'"$PWD"'/crates/panini" }#' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i 's#^panini-data = { path = .*#panini-data = { path = "'"$PWD"'/crates/panini-data" }#' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
```

Then copy the committed harness — never rewrite it — and run it:

```bash
cp tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
cd /tmp/vidyut-full/vidyut-prakriya && cargo run --release --example panini_full_audit 2>&1 | tail -30
```

Expected: **zero divergences across 3636 cells / 4595 forms / 81 roots**, with the `entry` negative control still reporting differences.

The harness asserts the totals itself, so update them there first (Step 2) or it will fail on the assertion before it reaches the comparison.

- [ ] **Step 2: Update the audit's asserted totals**

In `tools/audit/panini_full_audit.rs`:
- `assert_eq!(n_cells, 3564, ...)` → `3636`, message `"cells: 404 root×pada×lakāra blocks × 9"`.
- `assert_eq!(n_forms, 4483, "forms: 3564 cells + 919 ALTERNATES rows")` → `4595` / `"forms: 3636 cells + 959 ALTERNATES rows"`.
- The header comment's "79 roots, 3564 cells, 4483 forms" → "81 roots, 3636 cells, 4595 forms"; "919 `ALTERNATES` rows" → "959"; "the full 3564-cell table" → "3636".

In `tools/audit/README.md`, add a run record beside the existing ones: zero differences across 3636 cells / 4595 forms / 81 roots at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, and the totals arithmetic — `81 = 79 + 2` (√bhī, √hrī); `3636 = 3564 + 72` (2 roots × 1 pada × 4 lakāras × 9 cells, both parasmaipada-only); `4595 = 4483 + 72 + 40` new `ALTERNATES` rows (919 → 959).

- [ ] **Step 3: Update the fork distribution**

The buckets in `crates/panini/tests/paradigm/main.rs` (~line 287) and `README.md` (~line 63) both enumerate the same distribution. New values:

| forms per cell | old | new |
|---|---|---|
| one | 2882 | 2925 |
| two | 526 | 550 |
| three | 115 | 117 |
| four | 17 | 18 |
| five | 8 | 9 |
| six | 16 | 17 |

Check: `2925 + 550 + 117 + 18 + 9 + 17 = 3636`; `2925 + 1100 + 351 + 72 + 45 + 102 = 4595`. README's "682 of the 3564 cells hold more than one form" becomes "711 of the 3636 cells".

Both prose sites must also record **why** the six-form record moved, because it is a new mechanism rather than another instance of an old one: √bhī's loṭ parasmaipada madhyama eka (`biBIhi` / `biBihi` / `biBItAd` / `biBitAd` / `biBItAt` / `biBitAt`) reaches six by **7.1.35 / 6.4.115 / 8.4.56**, a third distinct k = 3 stack against the same 2³ bound of eight — beside rudhādi's 7.1.35/8.4.65/8.4.56 and tanādi's 7.1.35/7.3.86/8.4.56. The record stands at seventeen cells and three mechanisms. `docs/ARCHITECTURE.md`'s branch-count discussion is the third site.

√bhī's loṭ prathama eka joins the five-form bucket; its vidhiliṅ prathama eka the four-form bucket; √hrī's two loṭ eka cells the three-form bucket.

- [ ] **Step 4: Update the `ALTERNATES` key census**

`crates/panini/tests/paradigm/main.rs` (~line 315) enumerates every key. Three existing counts change and four keys are new:

- `8.4.56` 134 → 138, `7.1.35` 112 → 116, `7.1.35+8.4.56` 112 → 116.
- New: `6.4.115` 23, `7.1.35+6.4.115` 2, `7.1.35+6.4.115+8.4.56` 2, `6.4.115+8.4.56` 1.

Check: 12 rows on existing keys + 28 on new keys = 40.

- [ ] **Step 5: The nine-gaṇa / count sweep**

Grep for every stale claim. The counts alone are not enough — prior slices found that an English-only grep misses rule-scoped and wrapped numbers:

```bash
grep -rn "3564\|4483\|\b919\b\|\b79 roots\b\|682 of" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.md --include=*.rs | grep -v "^docs/superpowers/"
grep -rn "6\.4\.115\|7\.4\.59\|7\.4\.60\|6\.4\.77\|6\.4\.82\|hrasva_of" crates --include=*.rs | grep -i "slice 3b\|later slice\|not implemented\|no root in scope\|deferred"
grep -rn "two of its twenty-six\|2 of its 26\|juhotyādi" README.md AGENTS.md docs/ARCHITECTURE.md
```

Every comment that named 3b as the slice that would do a thing must now say it did, or say what is left. Specifically: `sound.rs`'s `deaspirate_of` comment ("`B -> b` is slice 3b's"), `abhyasa.rs`'s header ("7.4.59 *hrasvaḥ* (slice 3b) fires only on the long-vowel roots it names"), `guna.rs`'s 6.4.77 comment ("the iyaṅ case itself arrives with √hrī in slice 3b"), and `tripadi.rs`'s 8.4.54 comment ("in slice 3b BI → bI"). Also update the juhotyādi paragraph in README/ARCHITECTURE/AGENTS from two of twenty-six rows to four.

- [ ] **Step 6: Run the full suite once more**

Run: `mise run test 2>&1 | tail -30` (foreground)
Expected: PASS at 3636 cells. Doc changes should not move it; run it because the prose sites are inside doc comments on live test functions and a bad edit breaks compilation.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add -A
git commit -m "docs: 3b's counts, the audit record, and the comments it falsifies

3636 cells / 4595 forms / 81 roots / 959 ALTERNATES across README,
ARCHITECTURE, AGENTS, paradigm/main.rs and tools/audit. Audit at zero
divergence against 8da2f90b.

The six-form record moves to seventeen cells and THREE mechanisms:
√bhī's loṭ madhyama eka reaches six by 7.1.35/6.4.115/8.4.56, a third
k = 3 stack beside rudhādi's 8.4.65 route and tanādi's 7.3.86 route.

Four comments that named 3b as the slice that would do a thing now say
it did: deaspirate_of's B arm, abhyasa.rs's 7.4.59 note, 6.4.77's iyaṅ
deferral and 8.4.54's BI note."
```

---

## Task 8: The mutation gate

**Files:**
- Modify: `AGENTS.md` (timing record); `mise.toml` only if the cap must move

- [ ] **Step 1: Measure the uncontended floor at 3636 cells**

Run: `time mise run test 2>&1 | tail -5` (foreground, nothing else running)

Record paradigm, roundtrip and trace component times and the wall clock. **Do not scale the 3564 figure by cell count** — AGENTS.md's standing warning, earned twice in this series: 3a's cell count grew 2.06% and its floor grew 11.50%.

- [ ] **Step 2: Project the campaign cap before running it**

The 3564 floor is 2258.069s (paradigm 1003.42s, roundtrip 1249.42s, trace 4.03s), and slice 3a measured a caught-mutant margin of ~1.70×. Compute `measured_floor × 1.70` for 3636.

- If the projection exceeds **4800s**, raise the cap in `AGENTS.md` and `mise.toml` **together**, never silently — the standing rule.
- If it lands under 4800s but with a margin of tens of percent rather than multiples, say so in the AGENTS.md record. A cap that "clears" by 10% is adequate-but-tight, and a `timeout.txt` entry under it needs re-running alone before it is read as anything.

- [ ] **Step 3: Run the campaign**

Run through the task, not by reconstructing flags — `mise run mutants` already carries `-j 4 --timeout 4800`:

```bash
mise run mutants -- -o mutants.out.3b --iterate 2>&1 | tail -40
```

Two operational constraints, both learned the hard way in this series:
- **Always pass `-o`.** Every invocation rotates `mutants.out` → `mutants.out.old`, so a second probe run destroys a finished campaign's `outcomes.json`.
- **Background shells die at ~60 minutes.** Chunk with `--iterate` and `--exclude-re` for the known permanent pair rather than starting one long run.
- `cargo mutants` also reads `-j` from `CARGO_MUTANTS_JOBS`, so confirm the environment is not overriding the task's `-j 4`.

- [ ] **Step 4: Check both outcome files**

```bash
cat mutants.out.3b/missed.txt
cat mutants.out.3b/timeout.txt
```

Expected: `missed.txt` **empty**. `timeout.txt` holding **only** the known permanent entry — `tripadi.rs`'s ṇatva backward scan, whose `j -= 1` → `j /= 1` mutant makes the loop non-terminating. That one is a correct, permanent verdict; do not chase it with a bigger cap or a code change.

Any other `timeout.txt` entry must be **re-run alone** before being read as anything: under a cap that does not clear a full uncaught run, a real survivor is reclassified as a timeout and "0 missed" becomes vacuous.

- [ ] **Step 5: Fix any survivor, then re-run only the affected files**

A survivor in this slice's own code is most likely one of: 7.4.60's `is_vowel` guard, 7.4.59's no-op comparison, 6.4.115's `hali` or `kṅiti` clause, or 6.4.82's length test. Each has a named guard test in Tasks 2–4; a survivor means the test does not separate the mutant, not that the guard is wrong. Strengthen the test.

- [ ] **Step 6: Record the timing in AGENTS.md**

Append a slice-3b paragraph beside the 3a one: cell growth 3564 → 3636 (+2.02%), the measured floor and its percentage move, the per-binary breakdown, the campaign's mutant/caught/missed/unviable/timeout counts, and the caught-mutant margin. State whether the 4800s cap still clears a full uncaught run at `-j 4` **by measurement**, not by inheritance.

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md mise.toml
git commit -m "chore: 3b mutation gate — floor re-measured at 3636 cells

missed.txt empty; timeout.txt holds only the known permanent tripadi.rs
ṇatva-scan entry. Floor measured rather than scaled, per the standing
warning: 3a's 2.06% cell growth moved the floor 11.50%."
```

---

## Task 9: Finish the branch

- [ ] **Step 1: Confirm the whole gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -20
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin juhotyadi-3b
gh pr create --title "juhotyādi 3b — √bhī and √hrī" --body "$(cat <<'BODY'
Slice 3b curates √bhī (`03.0002`) and √hrī (`03.0003`), taking the golden
suite from 3564 to 3636 cells and juhotyādi from two of its twenty-six rows
to four.

**New rules:** 7.4.60 *halādiḥ śeṣaḥ* and 7.4.59 *hrasvaḥ* in `abhyasa.rs`;
6.4.115 *bhiyo'nyatarasyām* in `guna.rs`, the ninth vikalpa. **New arm:**
6.4.77's iyaṅ. **Widened:** 6.4.82 to a long `I`.

3a's pre-recorded bundle for this slice was wrong in both directions, and
re-probing all 72 cells is what found it: 8.4.54 needed no widening
(`deaspirate_of` already carried `B -> b`), while 6.4.82's long-`I` case and
a `pada_from_upadesha` defect were both unlisted. That defect — an initial
`Yi` treated as a ñit 1.3.72 reads — had been unfalsifiable since it was
written, because √bhī is the first curated root to reach it.

Deleting it made a second claim vacuous, so the slice pays for that too: no
upstream row satisfies both of `pada_from_upadesha`'s branch conditions once
the clause is gone, so the branch order moves from a comment to a data
invariant rather than becoming an unkillable mutant.

Audit at zero divergence against `8da2f90b`; `missed.txt` empty.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Per the standing instruction: auto-merge once green, verify the commits are on `main`, then delete the branch and the worktree.

---

## Self-Review

**Spec coverage.** Every spec section maps to a task: the `Yi` correction and the disjointness invariant → Task 1; 7.4.60/7.4.59 and `hrasva_of` → Task 2; 6.4.82 and 6.4.77 → Task 3; 6.4.115 → Task 4; root selection, counts, `ALTERNATES` keys → Task 5; the five trace pins → Task 6; the audit, the fork distribution and the doc sweep → Task 7; the mutation gate and its cap arithmetic → Task 8. The spec's "unchanged but newly witnessed" items (8.4.54, 8.4.2, 7.4.62) need no code and are witnessed by Task 5's goldens and Task 6's `jihrayani` pin.

**Type consistency.** `hrasva_of(char) -> Option<char>` is defined in Task 2 Step 3 and used in Task 2 Step 7 and nowhere else. `following_sarvadhatuka(&Prakriya) -> Option<&Term>` is pre-existing and used in Tasks 3 and 4 with the same signature. `bhi_slu_prakriya(&str, bool) -> Prakriya` is defined and used only within Task 4. `slu_prakriya(&str, &str) -> Prakriya` is pre-existing in `abhyasa.rs` and used only in Task 2. Rule ids are quoted consistently as `"7.4.60"`, `"7.4.59"`, `"6.4.115"` in the rule bodies, the two pinned lists, and the trace assertions.

**Known soft spot.** Task 6 Step 2 names the one assertion whose expected value depends on which arm `cell_trace` returns for a forked cell, and says what to do either way rather than guessing. Task 5 Step 7 says explicitly not to edit a golden to match the engine.
