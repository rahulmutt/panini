# Juhotyādi gaṇa slice 3c Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate √dā (`03.0010`), √dhā (`03.0011`), √mā (`03.0007`) and √hā ātmanepada (`03.0008`) into the juhotyādi gaṇa, taking the golden suite from 3636 to 3852 cells, by carrying the dhātupāṭha number into the pipeline, landing 1.1.20 as `Tag::Ghu`, four new rules (7.4.76, 6.4.119, 6.1.88, 8.2.38), and four restructured ones (6.4.113 above 6.4.112 with abhyasta arms, 6.1.101's āṭ + ec decline, 8.2.40's *adhaḥ*).

**Architecture:** Eleven tasks. Task 1 is plumbing (`Context.dhatupatha`, `Tag::Ghu`, `GHU`) that moves no golden. Tasks 2–6 add or restructure engine rules that fire on no curated root yet, so each is gated on **per-rule guard tests plus the 3636 priors staying byte-identical**. Task 7 lands the four rows and their goldens and turns 216 cells green in one step; Task 8 pins nine traces. Tasks 9–11 are the audit and doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.98.1 pinned via `mise`. `mise run build | test | test-full | lint | fmt | fmt-check | mutants | audit`. Cross-implementation goldens from vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`.

**Spec:** `docs/superpowers/specs/2026-09-11-juhotyadi-gana-3c-design.md`

## Global Constraints

- **The 3636 pre-existing cells must stay byte-identical through Tasks 1–6.** No golden regenerated, no pinned trace changed. That is the evidence that the 6.4.113/6.4.112 swap, the vowel-test removal, 6.1.101's decline, 6.1.88 and 8.2.40's exclusion reach nothing they should not.
- **No unfalsifiable guard clauses.** A clause no cell and no guard test can make false is a mutation survivor: witness it, delete it, or kill it with a direct guard test on a hand-built `Prakriya`.
- **Row identity:** a sūtra naming particular roots matches `p.ctx.dhatupatha` by number, with a comment naming the upadeśa; a sūtra naming a class reads a saṁjñā tag (`Tag::Ghu`). Never key a 3c condition on `ANGA.text`: `03.0008` and `03.0009` are both `hA`, `03.0010` and `02.0054` both `dA`.
- Golden forms are **transcribed from the spec's appendix**, never invented. Do **not** edit a golden to match the engine.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit.
- **`mise run test` takes ~65 s** (AGENTS.md, 3636 cells). Run it in the **foreground** with an explicit timeout of at least 600000 ms; never background it and end a turn. `mise run test-full` takes ~25 minutes; same rule, timeout 3600000 ms.
- `mise run test -- -p X` does not scope. Scope unit tests with `mise exec -- cargo test -p <crate> <filter>`.
- Rule ids in the spec's appendix and trace table are authoritative; quote them identically in rule bodies, pinned lists and trace assertions.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-prakriya/src/context.rs` | Task 1: `Context.dhatupatha` |
| `crates/panini-prakriya/src/term.rs` | Task 1: `Tag::Ghu` |
| `crates/panini-prakriya/src/tinanta/samjna.rs` | Task 1: `GHU` and its upstream test |
| `crates/panini-prakriya/src/tinanta/mod.rs` | Task 1: `derive` stamps the number and adds `Tag::Ghu` |
| `crates/panini-prakriya/src/tinanta/abhyasa.rs` | Task 2: 7.4.76 |
| `crates/panini-prakriya/src/tinanta/guna.rs` | Task 3: 6.4.113 / 6.4.112 restructure; Task 4: 6.4.119 |
| `crates/panini-prakriya/src/tinanta/adesha.rs` | Task 5: 6.1.101's decline, 6.1.88 |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | Task 6: 8.2.40's *adhaḥ*, 8.2.38 |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Task 1's derive test; the pinned rule order (Tasks 2–6) |
| `crates/panini-data/src/lib.rs` | Task 7: four `Dhatu` rows, the eight-root test |
| `crates/panini/tests/paradigm/data/juhotyadi.rs` | Task 7: 24 `PARADIGM` blocks, 12 `ALTERNATES` rows |
| `crates/panini/tests/paradigm/main.rs` | Task 7: corpus totals and fork-distribution prose |
| `crates/panini/tests/trace/juhotyadi.rs` | Task 8: nine trace pins |
| `tools/audit/`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, 3a's spec | Task 9: counts, audit record, doc sweep |
| `AGENTS.md`, `mise.toml` | Task 10: mutation record; cap only if it must move |

---

## Task 1: Row identity reaches the pipeline

Plumbing only. Adds no rule; moves no golden.

**Files:**
- Modify: `crates/panini-prakriya/src/context.rs`
- Modify: `crates/panini-prakriya/src/term.rs` (`Tag`, after `Abhyasta`)
- Modify: `crates/panini-prakriya/src/tinanta/samjna.rs` (a `GHU` const above `SAMJNA`; a test)
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs` (`derive`)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (a test; the `panini_data` import line)

**Interfaces:**
- Consumes: nothing from earlier tasks.
- Produces: `Context.dhatupatha: &'static str` (`""` from `Context::new`/`Default`, the row number from `derive`); `Tag::Ghu` on the aṅga term when the row is in `samjna::GHU: [&str; 6]`. Tasks 2–6 read both.

- [ ] **Step 1: Write the failing tests**

In `context.rs`'s `tests` module:

```rust
    #[test]
    fn a_fresh_context_names_no_dhatupatha_row() {
        // Number-keyed guards (7.4.76, 8.2.38, 8.2.40) decline on "", so a
        // hand-built prakriyā can never trip one by accident. `derive` is
        // the only writer of a real number.
        let c = Context::new(
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert_eq!(c.dhatupatha, "");
        assert_eq!(Context::default().dhatupatha, "");
    }
```

In `samjna.rs`'s `tests` module:

```rust
    /// Upstream's dhātupāṭha, vendored, test-only — the `panini-data`
    /// precedent for holding a verdict to upstream rather than to itself.
    const UPSTREAM: &str = include_str!("../../../../data/dhatupatha.tsv");

    fn upstream_upadesha(number: &str) -> Option<&'static str> {
        UPSTREAM
            .lines()
            .filter(|l| !l.starts_with('#'))
            .find_map(|l| {
                let mut f = l.split('\t');
                if f.next() == Some(number) {
                    f.next()
                } else {
                    None
                }
            })
    }

    #[test]
    fn ghu_is_exactly_the_six_rows_1_1_20_names() {
        // 1.1.20 dādhā ghv adāp. Four of the six are not curated, so this
        // test — not a derivation — is what holds them.
        let named = [
            ("01.1050", "De\\w"),
            ("01.1079", "dA\\R"),
            ("01.1117", "de\\N"),
            ("03.0010", "qudA\\Y"),
            ("03.0011", "quDA\\Y"),
            ("04.0043", "do\\"),
        ];
        assert_eq!(GHU, named.map(|(n, _)| n));
        for (number, upadesha) in named {
            assert_eq!(upstream_upadesha(number), Some(upadesha), "{number}");
        }
        // *adāp*: dāp and daip are dā-shaped and NOT ghu. dāp is why the set
        // is keyed by number: it enters the derivation as `dA`, exactly as
        // ḍudāñ does.
        for (number, upadesha) in [("02.0054", "dA\\p"), ("01.1073", "dE\\p")] {
            assert_eq!(upstream_upadesha(number), Some(upadesha), "{number}");
            assert!(!GHU.contains(&number), "{number} is adāp");
        }
    }
```

In `derivation_tests.rs`, change the import line to `use panini_data::{Dhatu, Gana, Lakara, Pada, PadaAssignment, Purusha, Vacana, dhatus};` and add:

```rust
#[test]
fn derive_stamps_the_row_number_and_decides_ghu_by_it() {
    // Two hand-built rows whose aṅga text is identical: 03.0010 qudA\Y (ghu)
    // and 02.0054 dA\p (adāp, not ghu). Only the number separates them —
    // the reason Context carries it.
    let ghu = Dhatu {
        dhatupatha: "03.0010",
        code: "dA",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "dAne",
    };
    let dap = Dhatu {
        dhatupatha: "02.0054",
        code: "dA",
        gana: Gana::Adadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "lavane",
    };
    for (d, is_ghu) in [(ghu, true), (dap, false)] {
        let p = derive(
            &d,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        )
        .into_iter()
        .next()
        .unwrap();
        assert_eq!(p.ctx.dhatupatha, d.dhatupatha);
        assert_eq!(p.terms[ANGA].has(Tag::Ghu), is_ghu, "{}", d.dhatupatha);
    }
}
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya -- dhatupatha ghu 2>&1 | tail -20`
Expected: FAIL to compile — `no field dhatupatha on type Context`, `no variant Ghu`, `cannot find value GHU`.

- [ ] **Step 3: Add the field to `Context`**

In `context.rs`, after `is_ngit_like`:

```rust
    /// The dhātupāṭha entry number of the root being derived — the repo's
    /// unique key, `panini_data::Dhatu::dhatupatha` — or `""` for a
    /// hand-built prakriyā.
    ///
    /// Root TEXT cannot identify every root a sūtra names: `03.0008 o~hA\N`
    /// and `03.0009 o~hA\k` both enter the derivation as `hA`, and 7.4.76
    /// bhṛñām it names only the first. A sūtra that names particular roots
    /// matches this; a sūtra that names a class reads a saṁjñā tag instead
    /// (`Tag::Ghu`). Set by `tinanta::derive`; `Context::new` leaves it
    /// empty, so every number-keyed guard declines unless a test sets it.
    pub dhatupatha: &'static str,
```

and in `Context::new`'s struct literal, after `is_ngit_like: ...,`, add `dhatupatha: "",`.

- [ ] **Step 4: Add `Tag::Ghu`**

In `term.rs`, immediately after the `Abhyasta` variant:

```rust
    /// 1.1.20 dādhā ghv adāp: the aṅga is a *ghu* root — one of the six
    /// dā- and dhā-shaped dhātupāṭha rows, excluding dāp and daip. Set by
    /// `tinanta::derive` from the row NUMBER (`tinanta::samjna::GHU`),
    /// never from root text: `02.0054 dA\p` also enters the derivation as
    /// `dA` and is not ghu. A saṁjñā verdict, like `Abhyasta`, so no step is
    /// recorded. Read by 6.4.113's *aghoḥ* and by 6.4.119.
    Ghu,
```

- [ ] **Step 5: Add `GHU` to `samjna.rs`**

Immediately above `pub(crate) static SAMJNA: &[Rule] = &[`:

```rust
/// 1.1.20 dādhā ghv adāp: the dhātupāṭha rows that are *ghu* — the roots
/// of the form dā or dhā, except dāp (`02.0054 dA\p`, lavane) and daip
/// (`01.1073 dE\p`, śodhane). By number, because root text cannot express
/// the exclusion: dāp enters the derivation as `dA`, exactly as ḍudāñ does.
///
/// `01.1050 De\w` dheṭ, `01.1079 dA\R` dāṇ, `01.1117 de\N` deṅ,
/// `03.0010 qudA\Y` ḍudāñ, `03.0011 quDA\Y` ḍudhāñ, `04.0043 do\` do.
///
/// Only `03.0010` and `03.0011` are curated; all six are pinned to upstream
/// by `ghu_is_exactly_the_six_rows_1_1_20_names`. `super::derive` reads
/// this to add `Tag::Ghu` — a saṁjñā verdict, recorded as no step.
pub(crate) const GHU: [&str; 6] = [
    "01.1050", "01.1079", "01.1117", "03.0010", "03.0011", "04.0043",
];
```

Append to the module doc's first line so it reads `//! Saṃjñā, pada sanction and ending insertion: 1.1.20 (as the \`GHU\` set), 1.3.12, 1.3.66, 1.3.72,`.

- [ ] **Step 6: Stamp the number and the tag in `derive`**

In `tinanta/mod.rs`'s `derive`, replace:

```rust
    let mut p = Prakriya {
        ctx: Context::new(lakara, pada, purusha, vacana),
        ..Default::default()
    };
    let mut t = Term::new(dhatu.code);
    t.add(Tag::Dhatu);
```

with:

```rust
    let mut p = Prakriya {
        ctx: Context {
            dhatupatha: dhatu.dhatupatha,
            ..Context::new(lakara, pada, purusha, vacana)
        },
        ..Default::default()
    };
    let mut t = Term::new(dhatu.code);
    t.add(Tag::Dhatu);
    // 1.1.20 dādhā ghv adāp, decided by row number — see `samjna::GHU` for
    // why root text cannot decide it. A saṁjñā, like the gaṇa tags below:
    // it feeds guarded rules and substitutes for none.
    if samjna::GHU.contains(&dhatu.dhatupatha) {
        t.add(Tag::Ghu);
    }
```

- [ ] **Step 7: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS.

- [ ] **Step 8: Run the full suite; confirm the priors are byte-identical**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS, 3636 cells, no golden changed. No rule reads the new field or tag yet.

- [ ] **Step 9: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/context.rs crates/panini-prakriya/src/term.rs \
        crates/panini-prakriya/src/tinanta/samjna.rs crates/panini-prakriya/src/tinanta/mod.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(engine): the dhātupāṭha number reaches the pipeline; 1.1.20 as Tag::Ghu

Root text cannot identify the roots 3c's sūtras name: 03.0008 and 03.0009
are both hA, and 02.0054 dāp is dA like ghu 03.0010. Context gains the row
number, set by derive; GHU holds 1.1.20's six rows, pinned to the vendored
TSV with dāp and daip asserted absent, and derive adds Tag::Ghu from it.

No rule reads either yet; the 3636 priors are byte-identical."
```

---

## Task 2: 7.4.76 *bhṛñām it*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/abhyasa.rs` (new rule after 7.4.62; guard tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: `Context.dhatupatha` (Task 1); `ABHYASA`; `is_vowel`.
- Produces: rule `"7.4.76"` in `ABHYASA_RULES`, immediately after `"7.4.62"`.

- [ ] **Step 1: Write the failing guard tests**

Add to `abhyasa.rs`'s `tests` module (`slu_prakriya` already exists there):

```rust
    #[test]
    fn bhrnam_it_makes_the_abhyasa_vowel_i_for_the_two_rows_it_names() {
        // 7.4.76. √māṅ (03.0007) and √āṅhāṅ (03.0008), after 7.4.59 and
        // 7.4.62: ma → mi (mimIte), Ja → Ji (jihIte).
        let rule = rules().find(|r| r.id == "7.4.76").unwrap();
        for (root, number, abhyasa, want) in [("mA", "03.0007", "ma", "mi"), ("hA", "03.0008", "Ja", "Ji")] {
            let mut p = slu_prakriya(root, "te");
            p.ctx.dhatupatha = number;
            p.terms[ABHYASA].text = abhyasa.into();
            assert!((rule.apply)(&mut p), "{number}");
            assert_eq!(p.terms[ABHYASA].text, want, "{number}");
            assert_eq!(p.log.last().unwrap().sutra, "7.4.76");
        }
    }

    #[test]
    fn bhrnam_it_declines_for_the_other_ha_row_despite_identical_text() {
        // 03.0009 o~hA\k enters the derivation as `hA`, exactly like
        // 03.0008, and takes no 7.4.76: jahAti, not *jihAti. This is the test
        // that the number, not the text, decides.
        let rule = rules().find(|r| r.id == "7.4.76").unwrap();
        for number in ["03.0009", ""] {
            let mut p = slu_prakriya("hA", "ti");
            p.ctx.dhatupatha = number;
            p.terms[ABHYASA].text = "Ja".into();
            assert!(!(rule.apply)(&mut p), "{number:?}");
            assert_eq!(p.terms[ABHYASA].text, "Ja", "{number:?}");
            assert!(p.log.is_empty(), "{number:?}");
        }
    }
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya bhrnam 2>&1 | tail -20`
Expected: FAIL — `rules().find(|r| r.id == "7.4.76").unwrap()` panics on `None`.

- [ ] **Step 3: Add the rule after 7.4.62**

Insert immediately after the `7.4.62` rule's closing `},` in `ABHYASA_RULES`:

```rust
    // 7.4.76 bhṛñām it: the abhyāsa of √bhṛñ, √māṅ and √āṅhāṅ takes `i`.
    // ma → mi (mimIte), and — after 7.4.59 and 7.4.62 — Ja → Ji (jihIte).
    // vidyut's order is 7.4.59 → 7.4.62 → 7.4.76, which this stage's
    // sequence already is.
    //
    // KEYED BY ROW NUMBER (`ctx.dhatupatha`), not by `ANGA.text`: the sūtra
    // names three roots, and `03.0009 o~hA\k` enters the derivation as `hA`
    // exactly like `03.0008 o~hA\N` while taking no 7.4.76 (jahAti).
    //   03.0007 mA\N   √māṅ
    //   03.0008 o~hA\N √āṅhāṅ
    // `03.0006 quBf\Y` (√bhṛñ) is the third, and slice 3d adds it with its
    // witness; an unwitnessed number here would be a mutation survivor.
    //
    // Every abhyāsa this reaches is a single ekāc (6.1.10's NARROW note), so
    // mapping each vowel to `i` is mapping THE vowel to `i`.
    Rule {
        id: "7.4.76",
        name: "BfYAm it",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !matches!(p.ctx.dhatupatha, "03.0007" | "03.0008") {
                return false;
            }
            let t: String = p.terms[ABHYASA]
                .text
                .chars()
                .map(|c| if is_vowel(c) { 'i' } else { c })
                .collect();
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("7.4.76", "BfYAm it", before);
            true
        },
    },
```

Update the module doc's first line to `//! Reduplication: 6.1.10, 7.4.60, 7.4.59, 7.4.62, 7.4.76 — dvitva and the rules that`.

- [ ] **Step 4: Update the pinned rule order**

In `derivation_tests.rs`'s `tinanta_rule_order_is_pinned`, change `"7.4.59", "7.4.62",` to `"7.4.59", "7.4.62", "7.4.76",`.

- [ ] **Step 5: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS.

- [ ] **Step 6: Run the full suite; confirm the priors are byte-identical**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS, 3636 cells. No curated row is `03.0007`/`03.0008` yet.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/abhyasa.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(engine): 7.4.76 bhṛñām it, keyed by row number

The abhyāsa of √māṅ and √āṅhāṅ takes i (mimIte, jihIte). Keyed on
ctx.dhatupatha because 03.0009 o~hA\\k is also hA and takes no 7.4.76
(jahAti); the guard test asserts exactly that. √bhṛñ's number waits for 3d
and its witness."
```

---

## Task 3: 6.4.113 above 6.4.112, both with abhyasta arms

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (the `--- śnā's alternation` block; guard tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: `Tag::Ghu` (Task 1); `Tag::Abhyasta`, `Tag::Ngit`; `ABHYASA`, `ANGA`, `SHAP`, `ENDING`, `following_sarvadhatuka`; `is_vowel`.
- Produces: `"6.4.113"` now immediately before `"6.4.112"` in `GUNA`. Test helper `abhyasta_prakriya(abhyasa: &str, anga: &str, ghu: bool, ending: &str, ngit: bool) -> Prakriya` in `guna.rs`'s `tests` module, reused by Task 4.

- [ ] **Step 1: Write the failing guard tests**

In `guna.rs`'s `tests` module, **replace** `shnabhyastayor_atah_declines_on_halali_and_on_non_ngit` (it asserts 6.4.112 declines on a hal-initial ending in isolation, which stops being true) with the first test below, and add the rest beside it:

```rust
    #[test]
    fn shnabhyastayor_atah_declines_after_6_4_113_and_on_non_ngit() {
        // Consonant-initial: 6.4.113's case — by ORDER now, not by a test in
        // this rule. 6.4.113 runs first and leaves `nI`, which this rule's
        // `nA` test declines.
        let r_113 = rules().find(|r| r.id == "6.4.113").unwrap();
        let r_112 = rules().find(|r| r.id == "6.4.112").unwrap();
        let mut p = shna_prakriya("kliS", "taH", true);
        assert!((r_113.apply)(&mut p));
        assert!(!(r_112.apply)(&mut p));
        assert_eq!(p.text(), "kliSnItaH");
        // Vowel-initial but PIT: the A must survive.
        let mut p = shna_prakriya("kliS", "anti", false);
        assert!(!(r_112.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nA");
    }

    /// An ā-final abhyasta aṅga under ślu: `abhyasa` in `ABHYASA`, `anga`
    /// tagged Abhyasta (and Ghu when `ghu`), an empty śap, and `ending`
    /// (tagged Ngit when `ngit`).
    fn abhyasta_prakriya(abhyasa: &str, anga: &str, ghu: bool, ending: &str, ngit: bool) -> Prakriya {
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new(anga), Term::new(""), Term::new(ending)]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = abhyasa.into();
        p.terms[ABHYASA].add(Tag::Abhyasta);
        p.terms[ANGA].add(Tag::Abhyasta);
        if ghu {
            p.terms[ANGA].add(Tag::Ghu);
        }
        if ngit {
            p.terms[ENDING].add(Tag::Ngit);
        }
        p
    }

    #[test]
    fn i_halyaghoh_abhyasta_arm_gives_i_before_a_hal_initial_kngit() {
        // mi + mA + te → mi + mI + te (mimIte).
        let rule = rules().find(|r| r.id == "6.4.113").unwrap();
        let mut p = abhyasta_prakriya("mi", "mA", false, "te", true);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "mI");
        assert_eq!(p.log.last().unwrap().sutra, "6.4.113");
        // vidhiliṅ: yāsuṭ sits on the ending's own text, Ngit by 3.4.103.
        let mut p = abhyasta_prakriya("mi", "mA", false, "yAt", true);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "mI");
    }

    #[test]
    fn i_halyaghoh_abhyasta_arm_declines_for_ghu_ajadi_pit_and_non_abhyasta() {
        let rule = rules().find(|r| r.id == "6.4.113").unwrap();
        // *aghoḥ*: √dā's dattaH, not *dadItaH.
        let mut p = abhyasta_prakriya("da", "dA", true, "tas", true);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "dA");
        // Vowel-initial: 6.4.112's cell (mimate).
        let mut p = abhyasta_prakriya("mi", "mA", false, "ate", true);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "mA");
        // Pit: the ā survives.
        let mut p = abhyasta_prakriya("mi", "mA", false, "ti", false);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "mA");
        // Not ā-final: √hu's juhutaH.
        let mut p = abhyasta_prakriya("Ju", "hu", false, "tas", true);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "hu");
        // Not abhyasta: adādi's √yā, whose yAtaH must keep its ā.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("yA"), Term::new(""), Term::new("tas")]),
            ..Default::default()
        };
        p.terms[ENDING].add(Tag::Ngit);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "yA");
        assert!(p.log.is_empty());
    }

    #[test]
    fn shnabhyastayor_atah_abhyasta_arm_elides_before_any_kngit() {
        let rule = rules().find(|r| r.id == "6.4.112").unwrap();
        // Vowel-initial: mi + mA + ate → mimate.
        let mut p = abhyasta_prakriya("mi", "mA", false, "ate", true);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "mimate");
        // Consonant-initial on a ghu root, which 6.4.113 left alone: dattaH's
        // `d` before 8.4.55 makes it `t`.
        let mut p = abhyasta_prakriya("da", "dA", true, "tas", true);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "d");
        assert_eq!(p.log.last().unwrap().sutra, "6.4.112");
    }

    #[test]
    fn shnabhyastayor_atah_abhyasta_arm_declines_on_pit_and_non_abhyasta() {
        let rule = rules().find(|r| r.id == "6.4.112").unwrap();
        // Pit: dadAti.
        let mut p = abhyasta_prakriya("da", "dA", true, "ti", false);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "dA");
        // Not abhyasta: √yā's yAnti keeps its ā for 6.1.101.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("yA"), Term::new(""), Term::new("anti")]),
            ..Default::default()
        };
        p.terms[ENDING].add(Tag::Ngit);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "yA");
        assert!(p.log.is_empty());
    }

    #[test]
    fn i_halyaghoh_then_shnabhyastayor_atah_split_a_hal_initial_kngit_by_ghu() {
        // The swap at unit level: in pipeline order, a non-ghu abhyasta aṅga
        // takes 6.4.113 and 6.4.112 declines on the `I` it left; a ghu one
        // is declined by 6.4.113 and elided by 6.4.112.
        let r_113 = rules().find(|r| r.id == "6.4.113").unwrap();
        let r_112 = rules().find(|r| r.id == "6.4.112").unwrap();
        let mut p = abhyasta_prakriya("mi", "mA", false, "te", true);
        assert!((r_113.apply)(&mut p));
        assert!(!(r_112.apply)(&mut p));
        assert_eq!(p.text(), "mimIte");
        let mut p = abhyasta_prakriya("da", "dA", true, "tas", true);
        assert!(!(r_113.apply)(&mut p));
        assert!((r_112.apply)(&mut p));
        assert_eq!(p.text(), "dadtas");
    }
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya -- halyaghoh shnabhyastayor 2>&1 | tail -30`
Expected: FAIL — both rules decline on every `abhyasta_prakriya` (SHAP is not `nA`), and `shnabhyastayor_atah_abhyasta_arm_elides_before_any_kngit` fails first.

- [ ] **Step 3: Replace the śnā-alternation block**

In `guna.rs`, replace everything from the line `    // --- śnā's alternation (6.4.112, 6.4.113) -----------------------------` through the closing `    },` of the `6.4.113` rule (the line before `    // 6.4.115 bhiyo'nyatarasyām`) with:

```rust
    // --- 6.4.113 / 6.4.112: the final ā of śnā and of an abhyasta aṅga ----
    //
    // Placed at the END of this stage, not in sūtra order. Three constraints
    // fix the position and each fails visibly if broken:
    //   - AFTER 7.1.3 jho'ntaḥ and 7.1.4 / 7.1.5, which make `Ji`/`Ja` into
    //     `anti`/`ati`/`ate`. Before them the plural endings are not
    //     vowel-initial and 6.4.113 would read them as hal-initial.
    //   - AFTER 7.2.79 liṅaḥ salopo'nantyasya. The ātmanepada vidhiliṅ ending
    //     is `sIyta` until its s is elided; run earlier and 6.4.113 matches
    //     the s, giving *vfRIsIyta.
    //   - BEFORE adesha.rs, whose 6.1.87 ād guṇaḥ would coalesce nA + Iyta
    //     into ne and give *vfReta, and whose 6.1.101 / 6.1.96 would take
    //     dA + Ate and dA + us (dadAte, adaduH) away from 6.4.112.
    //
    // 6.4.113 RUNS FIRST, as the apavāda it is. 6.4.112 carries no *aci*:
    // it elides the ā before ANY kṅit sārvadhātuka, and 6.4.113 takes the
    // consonant-initial ones — except for ghu roots (*aghoḥ*), where 6.4.112
    // then elides before consonants too (dattaH, dadyAt). Slice 3c swapped
    // the pair and deleted 6.4.112's vowel-initial test, which had only ever
    // stood in for "6.4.113 takes the rest".
    //
    // Two arms each. The śnā arm reads p.terms[SHAP] and p.terms[ENDING]
    // directly, NOT following_sarvadhatuka: the helper answers "what follows
    // the aṅga", which there is śnā itself. The abhyasta arm reads the aṅga's
    // own final ā, so its follower IS what follows the aṅga — under ślu the
    // empty śap makes that the ending, which in vidhiliṅ carries yāsuṭ on its
    // text and Ngit from 3.4.103 (6.4.115's reasoning, below).

    // 6.4.113 ī halyaghoḥ: the ā of śnā or of an abhyasta aṅga becomes `ī`
    // before a kṅit sārvadhātuka beginning with a consonant, except for the
    // ghu roots. kliS + nA + taH → kliSnItaH; mi + mA + te → mimIte.
    //
    // *aghoḥ* reads `Tag::Ghu` (1.1.20, decided by row number in derive); its
    // witness is √dā's dattaH, which would otherwise be *dadItaH. The
    // abhyasta test's witness is adādi's √yā — an ā-final aṅga with an empty
    // śap before a kṅit consonant-initial ending — whose yAtaH must keep its ā.
    Rule {
        id: "6.4.113",
        name: "I halyaGoH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if p.terms.len() <= ENDING {
                return false;
            }
            // śnā arm.
            if p.terms[SHAP].text == "nA" {
                if !p.terms[ENDING].has(Tag::Ngit) {
                    return false;
                }
                let Some(next) = p.terms[ENDING].text.chars().next() else {
                    return false;
                };
                if is_vowel(next) {
                    return false;
                }
                let before = p.snapshot();
                p.terms[SHAP].text = "nI".into();
                p.record("6.4.113", "I halyaGoH", before);
                return true;
            }
            // abhyasta arm.
            if !p.terms[ANGA].has(Tag::Abhyasta) || p.terms[ANGA].has(Tag::Ghu) {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix('A') else {
                return false;
            };
            let stem = format!("{stem}I");
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
            p.terms[ANGA].text = stem;
            p.record("6.4.113", "I halyaGoH", before);
            true
        },
    },
    // 6.4.112 śnābhyastayor ātaḥ: the ā of śnā or of an abhyasta aṅga is
    // elided before a kṅit sārvadhātuka. kliS + nA + anti → kliSnanti;
    // vf + nA + e → vfRe; da + dA + tas → da + d + tas (dattaH);
    // a + da + dA + us → adaduH; mi + mA + ate → mimate.
    //
    // No vowel-initial test: 6.4.113 above has already taken every follower
    // it can, so what reaches here is vowel-initial, or consonant-initial on
    // a ghu root. kryādi's kliSnItaH is the prior that fails loudly if the
    // order is ever reversed — 6.4.112 would make it *kliSntaH.
    //
    // The abhyasta test's witness is adādi's √yā again: yAnti keeps its ā
    // for 6.1.101.
    Rule {
        id: "6.4.112",
        name: "SnA'ByastayorAtaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if p.terms.len() <= ENDING {
                return false;
            }
            // śnā arm.
            if p.terms[SHAP].text == "nA" {
                if !p.terms[ENDING].has(Tag::Ngit) {
                    return false;
                }
                let before = p.snapshot();
                p.terms[SHAP].text = "n".into();
                p.record("6.4.112", "SnA'ByastayorAtaH", before);
                return true;
            }
            // abhyasta arm.
            if !p.terms[ANGA].has(Tag::Abhyasta) {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix('A') else {
                return false;
            };
            let stem = stem.to_string();
            let Some(follower) = following_sarvadhatuka(p) else {
                return false;
            };
            if !follower.has(Tag::Ngit) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = stem;
            p.record("6.4.112", "SnA'ByastayorAtaH", before);
            true
        },
    },
```

Update `guna.rs`'s module doc first line to `//! Vowel gradation and vikaraṇa reshaping: 7.4.21 … 6.4.113, 6.4.112, 6.4.115.`

- [ ] **Step 4: Update the pinned rule order**

In `tinanta_rule_order_is_pinned`, change `"7.3.101", "6.4.112", "6.4.113", "6.4.115",` to `"7.3.101", "6.4.113", "6.4.112", "6.4.115",`.

- [ ] **Step 5: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS, including the untouched `shna_alternation_rules_ignore_other_vikaranas_and_short_prakriyas` (`kliS` carries no `Abhyasta`, and the one-term prakriya fails `len() <= ENDING`).

- [ ] **Step 6: Run the full suite; confirm the priors are byte-identical**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS, 3636 cells, no golden changed. **This step is the proof the swap and the vowel-test removal are safe.** If a kryādi golden moves, stop: the order in `GUNA` is wrong, not the guard.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/guna.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(engine): 6.4.113 above 6.4.112, both with abhyasta arms

6.4.112 carries no aci; its vowel-initial test only ever stood in for
'6.4.113 takes the consonant-initial cases'. aghoḥ removes 6.4.113 for ghu
roots, so 6.4.112 must elide before consonants there (dattaH, dadyAt). The
apavāda now runs first and the stand-in test is gone.

Both rules gain an abhyasta arm on an ā-final aṅga, read through
following_sarvadhatuka; 6.4.113's excludes Tag::Ghu. The 3636 priors are
byte-identical: no kryādi cell fires both."
```

---

## Task 4: 6.4.119 *ghvasor eddhāv abhyāsalopaś ca*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (new rule immediately above the 6.4.113 / 6.4.112 block; guard tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: `Tag::Ghu` (Task 1); `abhyasta_prakriya` (Task 3's test helper); `ABHYASA`, `ANGA`, `ENDING`.
- Produces: rule `"6.4.119"` in `GUNA`, immediately before `"6.4.113"`.

- [ ] **Step 1: Write the failing guard tests**

In `guna.rs`'s `tests` module:

```rust
    #[test]
    fn ghvasor_eddhav_gives_e_and_elides_the_abhyasa_before_hi() {
        // da + dA + hi → de + hi (dehi); Da + DA + hi → Dehi.
        let rule = rules().find(|r| r.id == "6.4.119").unwrap();
        for (abhyasa, anga, want) in [("da", "dA", "dehi"), ("Da", "DA", "Dehi")] {
            let mut p = abhyasta_prakriya(abhyasa, anga, true, "hi", true);
            assert!((rule.apply)(&mut p), "{anga}");
            assert_eq!(p.terms[ABHYASA].text, "", "{anga}");
            assert_eq!(p.text(), want);
            assert_eq!(p.log.last().unwrap().sutra, "6.4.119");
        }
    }

    #[test]
    fn ghvasor_eddhav_declines_off_hi_off_ghu_and_on_a_short_prakriya() {
        let rule = rules().find(|r| r.id == "6.4.119").unwrap();
        // The tātaṅ branch: dattAt is 6.4.112's, not this rule's.
        let mut p = abhyasta_prakriya("da", "dA", true, "tAt", true);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "dadAtAt");
        // An ā-final aṅga that is not ghu: adādi's yAhi, not *yehi.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("yA"), Term::new(""), Term::new("hi")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "yAhi");
        // No ending term: must not panic indexing ENDING.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("dA")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert!(p.log.is_empty());
    }
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya ghvasor 2>&1 | tail -20`
Expected: FAIL — `rules().find(|r| r.id == "6.4.119").unwrap()` panics on `None`.

- [ ] **Step 3: Add the rule above the 6.4.113 / 6.4.112 block**

Insert immediately before the line `    // --- 6.4.113 / 6.4.112: the final ā of śnā and of an abhyasta aṅga ----`:

```rust
    // 6.4.119 ghvasor eddhāv abhyāsalopaś ca: before `hi`, a ghu root's ā
    // becomes `e` and the abhyāsa is elided. da + dA + hi → de + hi (dehi);
    // Da + DA + hi → Dehi.
    //
    // MUST PRECEDE 6.4.112, which would elide the same ā first and leave
    // dad + hi for 6.4.101 hujhalbhyo her dhiḥ to make *daddhi. The loṭ
    // madhyama eka tātaṅ branch (7.1.35, forked in the tiṅ stage) presents
    // `tAt`, not `hi`, so this rule declines there and 6.4.112 gives dattAt.
    // 6.4.101 later sees `e`, not a jhal, before `hi` and declines unedited.
    //
    // *ghu* reads `Tag::Ghu`; its witness is adādi's √yā, an ā-final aṅga
    // before the same `hi` (yAhi, not *yehi). The √as arm (*asoḥ*: edhi) has
    // no curated root and is not written.
    //
    // The elision empties `ABHYASA`'s text in place — the permanent-slot
    // idiom 2.4.72 set — so later readers of the slot (8.4.54) find nothing.
    Rule {
        id: "6.4.119",
        name: "GvasoreddhAvaByAsalopaSca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if p.terms.len() <= ENDING || p.terms[ENDING].text != "hi" {
                return false;
            }
            if !p.terms[ANGA].has(Tag::Ghu) {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix('A') else {
                return false;
            };
            let stem = format!("{stem}e");
            let before = p.snapshot();
            p.terms[ANGA].text = stem;
            p.terms[ABHYASA].text.clear();
            p.record("6.4.119", "GvasoreddhAvaByAsalopaSca", before);
            true
        },
    },
```

Update `guna.rs`'s module doc first line to `//! Vowel gradation and vikaraṇa reshaping: 7.4.21 … 6.4.119, 6.4.113, 6.4.112, 6.4.115.`

- [ ] **Step 4: Update the pinned rule order**

In `tinanta_rule_order_is_pinned`, change `"7.3.101", "6.4.113",` to `"7.3.101", "6.4.119", "6.4.113",`.

- [ ] **Step 5: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS.

- [ ] **Step 6: Run the full suite; confirm the priors are byte-identical**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS, 3636 cells. No curated aṅga carries `Tag::Ghu` yet.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/guna.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(engine): 6.4.119 ghvasor eddhāv abhyāsalopaś ca

Before hi a ghu root's ā becomes e and the abhyāsa is elided: dehi, Dehi.
Above 6.4.112, whose elision would otherwise feed 6.4.101 into *daddhi; the
tātaṅ branch's tAt declines it, leaving dattAt to 6.4.112. Ghu's witness is
adādi's yAhi. The √as arm has no curated root and is not written."
```

---

## Task 5: 6.1.101 leaves āṭ + ec to 6.1.90; 6.1.88 *vṛddhir eci*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/adesha.rs` (6.1.101's adādi arm; new rule after 6.1.90; guard tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: `ABHYASA`, `ANGA`, `SHAP`, `ENDING`; `vrddhi_of` (already imported in `adesha.rs`).
- Produces: rule `"6.1.88"` in `ADESHA`, immediately after `"6.1.90"`.

- [ ] **Step 1: Write the failing guard tests**

In `adesha.rs`'s `tests` module:

```rust
    #[test]
    fn akah_savarne_dirghah_adadi_arm_leaves_an_at_before_an_ec_to_6_1_90() {
        // da + dA + AE: the āṭ merges with its own ending first (6.1.90,
        // AE → E) and only then does the root's ā meet it (6.1.88) — dadE, the
        // Kaumudī's path and vidyut's. FORM-NEUTRAL: 6.1.101 then 6.1.88 would
        // spell dadE too, so this decline is held only by this test and the
        // dadE trace pin.
        let rule = rules().find(|r| r.id == "6.1.101").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("dA"), Term::new(""), Term::new("AE")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "da".into();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "AE");
        assert!(p.log.is_empty());
    }

    #[test]
    fn akah_savarne_dirghah_adadi_arm_still_fires_where_no_at_ec_follows() {
        // dadAni / dadAvahE: the āṭ is followed by a consonant, so 6.1.90 has
        // no ekādeśa to make and 6.1.101 takes dA + A as before. `aE` is the
        // `starts_with('A')` clause's only witness: an `a` before an ec is not
        // the āṭ, is still savarṇa with the ā, and must still coalesce.
        let rule = rules().find(|r| r.id == "6.1.101").unwrap();
        for (ending, want) in [("Ani", "ni"), ("AvahE", "vahE"), ("aE", "E")] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new("dA"), Term::new(""), Term::new(ending)]),
                ..Default::default()
            };
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ENDING].text, want, "{ending}");
        }
    }

    #[test]
    fn vrddhir_eci_merges_an_a_final_anga_into_the_endings_ec() {
        // da + dA + E → da + d + E: dadE, after 6.1.90 made AE into E.
        let rule = rules().find(|r| r.id == "6.1.88").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("dA"), Term::new(""), Term::new("E")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "da".into();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "d");
        assert_eq!(p.text(), "dadE");
        assert_eq!(p.log.last().unwrap().sutra, "6.1.88");
    }

    #[test]
    fn vrddhir_eci_declines_with_a_vikarana_between_no_ec_or_no_ending() {
        let rule = rules().find(|r| r.id == "6.1.88").unwrap();
        // A non-empty SHAP separates the ā from the ec: the SHAP test's witness.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("dA"), Term::new("a"), Term::new("E")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        // Not an ec: dadAni's `n`, after 6.1.101.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("dA"), Term::new(""), Term::new("ni")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        // Not ā-final: √ās's AsE is 6.1.90's alone.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("As"), Term::new(""), Term::new("E")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        // No ending term: must not panic indexing ENDING.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("dA"), Term::new("")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert!(p.log.is_empty());
    }

    #[test]
    fn atas_ca_then_vrddhir_eci_is_the_dade_path() {
        // The three rules in pipeline order on da + dA + AE: 6.1.101 declines,
        // 6.1.90's athematic arm makes AE into E, 6.1.88 merges dA + E.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("dA"), Term::new(""), Term::new("AE")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "da".into();
        for (id, fires) in [("6.1.101", false), ("6.1.90", true), ("6.1.88", true)] {
            let r = rules().find(|r| r.id == id).unwrap();
            assert_eq!((r.apply)(&mut p), fires, "{id}");
        }
        assert_eq!(p.text(), "dadE");
    }
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya -- akah_savarne vrddhir atas_ca_then 2>&1 | tail -30`
Expected: FAIL — 6.1.101 fires on `AE`; `rules().find(|r| r.id == "6.1.88").unwrap()` panics.

- [ ] **Step 3: Make 6.1.101's adādi arm decline on āṭ + ec**

In the `6.1.101` rule, replace the adādi arm's comment and condition:

```rust
            // adādi (śap luk'd by 2.4.72): the aṅga's own final ā meets an
            // a/ā-initial ending directly (no vikaraṇa buffer). ā + a/ā are
            // savarṇa → a single long ā. Keep the aṅga's ā, drop the ending's
            // initial vowel: yA + anti → yAnti, yA + Ani (āṭ) → yAni.
            if p.terms.len() > ENDING
                && p.terms[SHAP].text.is_empty()
                && p.terms[ANGA].text.ends_with('A')
                && matches!(p.terms[ENDING].text.chars().next(), Some('a') | Some('A'))
            {
```

with:

```rust
            // adādi (śap luk'd by 2.4.72): the aṅga's own final ā meets an
            // a/ā-initial ending directly (no vikaraṇa buffer). ā + a/ā are
            // savarṇa → a single long ā. Keep the aṅga's ā, drop the ending's
            // initial vowel: yA + anti → yAnti, yA + Ani (āṭ) → yAni, and — an
            // empty ślu'd śap looks the same — da + dA + Ani → dadAni.
            //
            // DECLINES on the āṭ followed by an ec (loṭ uttama eka ātmanepada,
            // `AE`). There the āṭ merges with its own ending first, 6.1.90
            // āṭaś ca (AE → E), and the root's ā meets the result by 6.1.88
            // vṛddhir eci: dadE, the Kaumudī's *dadai* and vidyut's trace
            // (juhotyādi 3c). Form-neutral — this arm then 6.1.88 would spell
            // dadE too — so only the dadE trace pin holds it. No prior reaches
            // it: √yā and √vā have no ātmanepada.
            if p.terms.len() > ENDING
                && p.terms[SHAP].text.is_empty()
                && p.terms[ANGA].text.ends_with('A')
                && matches!(p.terms[ENDING].text.chars().next(), Some('a') | Some('A'))
                && !(p.terms[ENDING].text.starts_with('A')
                    && matches!(
                        p.terms[ENDING].text.chars().nth(1),
                        Some('e' | 'E' | 'o' | 'O')
                    ))
            {
```

- [ ] **Step 4: Add 6.1.88 after 6.1.90**

Insert immediately after the `6.1.90` rule's closing `    },` (before `    // 6.1.97 ato guṇe`):

```rust
    // 6.1.88 vṛddhir eci: an a-varṇa followed by an ec coalesces into the
    // ec's vṛddhi. da + dA + E → da + d + E (dadE; mimE, jihE, daDE), after
    // 6.1.90's athematic arm has merged the loṭ uttama āṭ into its ending.
    //
    // The ā is the aṅga's own and the ec begins the ending, with the empty
    // ślu'd śap as the adjacency. A non-empty SHAP puts a vikaraṇa between
    // them: the thematic and kryādi `A` + ec junctions are 6.1.90's, whose
    // `A` is the 6.1.101 ekādeśa that contains the āṭ, and they keep that
    // attribution. `vrddhir_eci_declines_with_a_vikarana_between_no_ec_or_no_ending`
    // is the SHAP test's witness.
    //
    // Reaches no prior: √yā and √vā are the only other ā-final aṅgas with an
    // empty śap, and no parasmaipada ending begins with an ec.
    Rule {
        id: "6.1.88",
        name: "vfdDireci",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if p.terms.len() <= ENDING || !p.terms[SHAP].text.is_empty() {
                return false;
            }
            let Some(first) = p.terms[ENDING].text.chars().next() else {
                return false;
            };
            if !matches!(first, 'e' | 'E' | 'o' | 'O') {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix('A') else {
                return false;
            };
            let stem = stem.to_string();
            let before = p.snapshot();
            p.terms[ANGA].text = stem;
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("{}{rest}", vrddhi_of(first).unwrap());
            p.record("6.1.88", "vfdDireci", before);
            true
        },
    },
```

- [ ] **Step 5: Update the pinned rule order**

In `tinanta_rule_order_is_pinned`, change `"6.1.90", "6.1.97",` to `"6.1.90", "6.1.88", "6.1.97",`.

- [ ] **Step 6: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS, including every existing `akah_savarne_dirghah_*` and `atash_ca_athematic_*` test.

- [ ] **Step 7: Run the full suite; confirm the priors are byte-identical**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS, 3636 cells.

- [ ] **Step 8: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/adesha.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(engine): 6.1.88 vṛddhir eci, and 6.1.101 leaves āṭ + ec to 6.1.90

dadE takes the Kaumudī's path, as vidyut does: the āṭ merges with its own
ending first (6.1.90, AE → E), then the root's ā with the result (6.1.88).
6.1.101's adādi arm declines on āṭ + ec so it cannot take dA + A first.

The decline is form-neutral — 6.1.101 then 6.1.88 would spell dadE too — so
the guard test and 3c's dadE trace pin are what hold it. No prior reaches
either change."
```

---

## Task 6: 8.2.40's *adhaḥ* and 8.2.38 *dadhas tathoś ca*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (8.2.40's guard and comment; 8.4.53's and 8.4.54's comments; new rule between 8.4.54 and 8.4.55; guard tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: `Context.dhatupatha` (Task 1); `ABHYASA`, `ANGA`, `ENDING`.
- Produces: rule `"8.2.38"` in `TRIPADI`, immediately after `"8.4.54"`.

- [ ] **Step 1: Write the failing guard tests**

In `tripadi.rs`'s `tests` module:

```rust
    #[test]
    fn jhashas_tathor_dhodhah_declines_after_dha_by_adhah() {
        // *adhaḥ*. √dhā's D meets the t of tas once 6.4.112 has elided its ā:
        // DattaH, not *DadDaH. The identical shape on any other row fires.
        let rule = rules().find(|r| r.id == "8.2.40").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("D"), Term::new(""), Term::new("taH")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "da".into();
        p.ctx.dhatupatha = "03.0011";
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "daDtaH");
        assert!(p.log.is_empty());
        p.ctx.dhatupatha = "";
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "daDDaH");
    }

    /// √dhā after 6.4.112 and 8.4.54: abhyāsa `da`, the aṅga reduced to one
    /// consonant, an empty śap, `ending`, and row 03.0011.
    fn dadh_prakriya(anga: &str, ending: &str) -> Prakriya {
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new(anga), Term::new(""), Term::new(ending)]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "da".into();
        p.ctx.dhatupatha = "03.0011";
        p
    }

    #[test]
    fn dadhas_tathos_ca_aspirates_the_abhyasa_before_t_th_s_and_dhv() {
        // DattaH, DatTaH, Datse, and DadDve (whose root D 8.4.53 has already
        // made d). Only the abhyāsa changes; the root's sound is 8.4.53's or
        // 8.4.55's business.
        let rule = rules().find(|r| r.id == "8.2.38").unwrap();
        for (anga, ending) in [("D", "taH"), ("D", "TaH"), ("D", "se"), ("d", "Dve")] {
            let mut p = dadh_prakriya(anga, ending);
            assert!((rule.apply)(&mut p), "{anga}+{ending}");
            assert_eq!(p.terms[ABHYASA].text, "Da", "{anga}+{ending}");
            assert_eq!(p.terms[ANGA].text, anga, "{anga}+{ending}");
            assert_eq!(p.log.last().unwrap().sutra, "8.2.38");
        }
    }

    #[test]
    fn dadhas_tathos_ca_declines_with_the_a_present_before_other_sounds_and_off_dha() {
        let rule = rules().find(|r| r.id == "8.2.38").unwrap();
        // The ā survives before a pit ending: daDAti, not *DaDAti.
        let mut p = dadh_prakriya("DA", "ti");
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "da");
        // A vowel or another consonant follows: daDati, daDIDvam, daDvaH.
        for ending in ["ati", "IDvam", "vaH"] {
            let mut p = dadh_prakriya("D", ending);
            assert!(!(rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ABHYASA].text, "da", "{ending}");
        }
        // √dā has the identical shape and takes no 8.2.38: dattaH, not *DattaH.
        for number in ["03.0010", ""] {
            let mut p = dadh_prakriya("d", "taH");
            p.ctx.dhatupatha = number;
            assert!(!(rule.apply)(&mut p), "{number:?}");
            assert_eq!(p.terms[ABHYASA].text, "da", "{number:?}");
            assert!(p.log.is_empty(), "{number:?}");
        }
        // No ending term: must not panic.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("D"), Term::new("")]),
            ..Default::default()
        };
        p.ctx.dhatupatha = "03.0011";
        assert!(!(rule.apply)(&mut p));
    }
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya -- jhashas_tathor dadhas 2>&1 | tail -30`
Expected: FAIL — 8.2.40 fires on row `03.0011`; `rules().find(|r| r.id == "8.2.38").unwrap()` panics.

- [ ] **Step 3: Add *adhaḥ* to 8.2.40 and correct its comment**

In the 8.2.40 comment, replace this paragraph:

```rust
    // The ONLY NEW source of a D-initial ending in this suite besides the
    // pre-existing 6.4.101 her dhiḥ (which already supplies one: √hiṃs's
    // hinDi) — see 8.4.53's comment below for why that bounds its
    // widening: no root in the suite besides √indh ever presents a jhaṣ
    // immediately before its ending, because every OTHER gaṇa represented
    // here inserts a real vikaraṇa syllable between the two — bhvādi's
    // laBate, divādi's yuDyate, svādi's stiG, kryādi's guDnAti — so this
    // rule, and hence a fresh D-initial ending, is reachable only through
    // √indh.
```

with:

```rust
    // The ONLY NEW source of a D-initial ending in this suite besides the
    // pre-existing 6.4.101 her dhiḥ (which already supplies one: √hiṃs's
    // hinDi) — see 8.4.53's comment below for why that bounds its
    // widening. Two roots present a jhaṣ immediately before their ending:
    // √indh, and — once 6.4.112 has elided its ā, slice 3c — √dhā
    // (da + D + tas). Every other root represented here either inserts a
    // real vikaraṇa syllable between the two — bhvādi's laBate, divādi's
    // yuDyate, svādi's stiG, kryādi's guDnAti — or ends in no jhaṣ. The
    // sūtra's own *adhaḥ* excludes √dhā (DattaH, not *DadDaH), so this
    // rule, and hence a fresh D-initial ending, is still reachable only
    // through √indh.
    //
    // *adhaḥ* is KEYED BY ROW NUMBER, `03.0011 quDA\Y`. √dhā's only jhaṣ
    // before a `t`/`T` in any of its cells is its own final `D`, so a
    // narrower "the jhaṣ belongs to the aṅga" clause could never be
    // falsified and is not written.
```

and at the top of the 8.2.40 rule's `apply` body, before `let w = word_chars(p);`, add:

```rust
            if p.ctx.dhatupatha == "03.0011" {
                return false;
            }
```

- [ ] **Step 4: Correct 8.4.53's comment**

Replace:

```rust
    // 8.2.40 (7b Task 7) is the only NEW source of a D-initial ending —
    // besides the pre-existing 6.4.101 her dhiḥ — and it requires a jhaṣ
    // already abutting the ending, which no root in the suite besides
    // √indh ever presents: every OTHER gaṇa represented here inserts a
    // real vikaraṇa syllable between the root and the ending — bhvādi's
    // laBate, divādi's yuDyate, svādi's stiG, kryādi's guDnAti — so this
    // rule — and hence a fresh D-initial ending — is reachable only
    // through √indh.
```

with:

```rust
    // 8.2.40 (7b Task 7) is the only NEW source of a D-initial ending —
    // besides the pre-existing 6.4.101 her dhiḥ — and it requires a jhaṣ
    // already abutting the ending. Only √indh and √dhā present one (see
    // 8.2.40's comment), and 8.2.40's *adhaḥ* excludes √dhā, so a fresh
    // D-initial ending is still reachable only through √indh.
```

and replace:

```rust
    // about the STEMS that reach them, not the endings: every Dve/Dvam
    // cell pinned in this suite puts either a vowel (laBaDve, ADve,
    // vaDve, AsIDvam, laBaDvam) or an already-jaś `d` (KindDve)
    // immediately before the `D` — never an untreated jhal — so this rule
    // either has nothing to see or the no-op guard declines it. √indh's
    // own indDve/indDvam (7b Task 7) are the one cell where a stem-final
    // jhaṣ genuinely meets this native `D`, and that is exactly where
    // this rule is supposed to fire.
```

with:

```rust
    // about the STEMS that reach them, not the endings: every Dve/Dvam
    // cell pinned in this suite puts either a vowel (laBaDve, ADve,
    // vaDve, AsIDvam, laBaDvam) or an already-jaś `d` (KindDve, √dā's
    // dadDve) immediately before the `D` — never an untreated jhal — so
    // this rule either has nothing to see or the no-op guard declines it.
    // √indh's own indDve/indDvam (7b Task 7) and √dhā's DadDve, aDadDvam
    // and DadDvam (slice 3c, the root's `D` bared by 6.4.112) are the
    // cells where a stem-final jhaṣ genuinely meets this native `D`, and
    // that is exactly where this rule is supposed to fire.
```

- [ ] **Step 5: Extend 8.4.54's comment**

Replace:

```rust
    // 8.4.54 on √hu's 42 forms, on √bhī's and √hrī's own forms too, and on
    // none of √ki's.
```

with:

```rust
    // 8.4.54 on √hu's 42 forms, on √bhī's and √hrī's own forms too, on
    // √hā's and √dhā's (slice 3c: Ja → ja, Da → da — a `da` that 8.2.38,
    // just below, re-aspirates before t/th/s/dhv), and on none of √ki's,
    // √dā's or √mā's.
```

- [ ] **Step 6: Add 8.2.38 between 8.4.54 and 8.4.55**

Insert immediately after the `8.4.54` rule's closing `    },`:

```rust
    // 8.2.38 dadhas tathoś ca: the reduplicated √dhā — *dadh*, its ā gone —
    // takes bhaṣ for the abhyāsa's baś before `t`, `th`, `s` or `dhv`: the
    // abhyāsa's `d` becomes `D`. da + D + tas → Da + D + tas (DattaH, once
    // 8.4.55 devoices the root's `D`); Datse; DadDve.
    //
    // OUT OF SŪTRA ORDER, after 8.4.54 and before 8.4.55. The sūtra names
    // *dadh* — the stem after 8.4.54 has already deaspirated the abhyāsa to
    // `da` — so it must see that output. In sūtra position it would find
    // `Da`, decline, and 8.4.54 would then produce *dattaH / *dadDve.
    // vidyut-prakriya orders it the same way (8.4.54 < 8.2.38 < 8.4.55), and
    // the DattaH and DadDve trace pins hold it here.
    //
    // Only the abhyāsa changes. The root's own `D` is left to 8.4.53 (before
    // `Dv`: DadDve) and 8.4.55 (before `t`/`s`: DattaH, Datse), so the
    // credited rules match vidyut's step for step.
    //
    // KEYED BY ROW NUMBER, `03.0011 quDA\Y`: √dā (`03.0010`) has the
    // identical shape — da + d + tas — and takes no 8.2.38 (dattaH). The
    // single-consonant aṅga test is *dadh*'s lost ā: daDAti keeps its ā
    // before the pit `ti` and must not become *DaDAti.
    Rule {
        id: "8.2.38",
        name: "daDastaToSca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if p.ctx.dhatupatha != "03.0011" {
                return false;
            }
            let Some(e) = p.terms.get(ENDING).map(|t| t.text.as_str()) else {
                return false;
            };
            if !(e.starts_with(['t', 'T', 's']) || e.starts_with("Dv")) {
                return false;
            }
            if p.terms[ANGA].text.chars().count() != 1 {
                return false;
            }
            let Some(rest) = p.terms[ABHYASA].text.strip_prefix('d') else {
                return false;
            };
            let t = format!("D{rest}");
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("8.2.38", "daDastaToSca", before);
            true
        },
    },
```

- [ ] **Step 7: Update the pinned rule order**

In `tinanta_rule_order_is_pinned`, change `"8.4.53", "8.4.54", "8.4.55",` to `"8.4.53", "8.4.54", "8.2.38", "8.4.55",`.

- [ ] **Step 8: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS, including the existing `jhashas_tathor_dhodhah_fires_only_on_a_dental_after_a_jhash` (its prakriyās carry `dhatupatha: ""`).

- [ ] **Step 9: Run the full suite; confirm the priors are byte-identical**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS, 3636 cells.

- [ ] **Step 10: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/tripadi.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(engine): 8.2.38 dadhas tathoś ca after 8.4.54, and 8.2.40's adhaḥ

√dhā loses its ā to 6.4.112 and bares a jhaṣ before t/th/s/dhv. 8.2.40
must not voice the t (DattaH, not *DadDaH): adhaḥ, keyed on 03.0011.

8.2.38 re-aspirates the abhyāsa's d. The sūtra names dadh, the stem after
8.4.54's deaspiration, so it runs after 8.4.54 — vidyut's order — or 8.4.54
would undo it. √dā's identical da + d + tas takes no 8.2.38, hence the row
number. 8.2.40's and 8.4.53's 'only √indh' comments are corrected."
```

---

## Task 7: The four rows and their paradigm goldens

This is the task that turns 216 new cells green.

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (four `Dhatu` rows; the gaṇa-row test; three doc-comment counts)
- Modify: `crates/panini/tests/paradigm/data/juhotyadi.rs` (24 `PARADIGM` blocks, 12 `ALTERNATES` rows)
- Modify: `crates/panini/tests/paradigm/main.rs` (corpus totals, bucket counts, key census, prose)

**Interfaces:**
- Consumes: everything from Tasks 1–6.
- Produces: `dhatus()` rows `03.0007` (`mA`, `Atmanepada`), `03.0008` (`hA`, `Atmanepada`), `03.0010` (`dA`, `Ubhayapada`), `03.0011` (`DA`, `Ubhayapada`), all `Gana::Juhotyadi`.

- [ ] **Step 1: Add the four `Dhatu` rows**

`DHATUS` is ordered by number. Insert these after the `03.0003` row and before the `03.0020` row, in this order:

```rust
    Dhatu {
        // 03.0007 `mA\N` mAne Sabde ca (√māṅ). Ātmanepadī by 1.3.12 (final ṅ).
        // 7.4.76 bhṛñām it names it, so its abhyāsa is `mi` (mimIte) — keyed
        // by this number, not by `mA`. Not ghu: 6.4.113 gives its ī before a
        // consonant (mimIte), 6.4.112 elides its ā before a vowel (mimate).
        // Slice 3c.
        dhatupatha: "03.0007",
        code: "mA",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Atmanepada,
        artha: "mAne Sabde ca",
    },
    Dhatu {
        // 03.0008 `o~hA\N` gatO (√āṅhāṅ). The `o~` is an it by 1.3.2;
        // ātmanepadī by 1.3.12. Enters the derivation as `hA`, exactly like
        // 03.0009 `o~hA\k` (jahāti; slice 3c2) — which is why 7.4.76, naming
        // this row and not that one, keys on the NUMBER: jihIte. Slice 3c.
        dhatupatha: "03.0008",
        code: "hA",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Atmanepada,
        artha: "gatO",
    },
    Dhatu {
        // 03.0010 `qudA\Y` dAne (√ḍudāñ). The ḍu is 1.3.5's; ubhayapadī by
        // 1.3.72 (final ñ). Ghu by 1.1.20 — decided by this number in
        // `derive`, because `02.0054 dA\p` is `dA` too and is not ghu — so
        // 6.4.113 skips it, 6.4.112 elides its ā before consonants as well
        // (dattaH, dadyAt), and 6.4.119 gives dehi. Slice 3c.
        dhatupatha: "03.0010",
        code: "dA",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "dAne",
    },
    Dhatu {
        // 03.0011 `quDA\Y` DAraRapozaRayoH (√ḍudhāñ). Ghu like √dā. Its bared
        // `D` is what 8.2.40's *adhaḥ* excludes (DattaH, not *DadDaH) and what
        // 8.2.38 dadhas tathoś ca reads, re-aspirating the abhyāsa after
        // 8.4.54 (DattaH, Datse, DadDve). Both keyed by this number. Slice 3c.
        dhatupatha: "03.0011",
        code: "DA",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "DAraRapozaRayoH",
    },
```

In the `Dhatu` struct's doc comments, change `re-derives 80 of these 81` to `re-derives 84 of these 85` and `The test covers the 81 roots curated here` to `The test covers the 85 roots curated here`. In `pada_from_upadesha`'s doc comment, change the words `51 of the 81 curated roots` to `55 of the 85 curated roots`, and the `30` that begins the next line to `34` — all four new upadeśas carry a backslash on the root vowel and none on an it.

- [ ] **Step 2: Extend the gaṇa-row test**

Rename `juhotyadi_rows_are_the_four_curated_roots` to `juhotyadi_rows_are_the_eight_curated_roots`. Replace the comment's last sentence (`The gaṇa is PARTIAL at 4 of its 26 dhātupāṭha rows; slices 3c–3f close it (spec, "Later slices").`) with:

```rust
        // Slice 3c adds √dā and √dhā (ubhayapadī by 1.3.72, ghu by 1.1.20)
        // and √mā and √hā (ātmanepadī by 1.3.12). The gaṇa is PARTIAL at 8 of
        // its 26 dhātupāṭha rows; slices 3c2 and 3d–3f close it.
```

Change the expected vector to:

```rust
            vec![
                ("03.0001", "hu", PadaAssignment::Parasmaipada),
                ("03.0002", "BI", PadaAssignment::Parasmaipada),
                ("03.0003", "hrI", PadaAssignment::Parasmaipada),
                ("03.0007", "mA", PadaAssignment::Atmanepada),
                ("03.0008", "hA", PadaAssignment::Atmanepada),
                ("03.0010", "dA", PadaAssignment::Ubhayapada),
                ("03.0011", "DA", PadaAssignment::Ubhayapada),
                ("03.0020", "ki", PadaAssignment::Parasmaipada),
            ]
```

After the existing `BI` uniqueness assertion, add this comment (no assertion):

```rust
        // Slice 3c's root-specific rules (7.4.76, 8.2.38, 8.2.40's adhaḥ) and
        // 1.1.20's Tag::Ghu key on the dhātupāṭha NUMBER, so `hA`, `dA` and
        // `DA` need no uniqueness tripwire — and `hA` must not get one: slice
        // 3c2's 03.0009 `o~hA\k` enters the derivation as `hA` too.
```

- [ ] **Step 3: Run the data tests**

Run: `mise exec -- cargo test -p panini-data 2>&1 | tail -20`
Expected: PASS, including `curated_pada_agrees_with_upadesha_markers` and `dhatupatha_numbers_resolve_upstream` (the arthas above are upstream's verbatim).

- [ ] **Step 4: Append the `PARADIGM` blocks**

Append to the end of `PARADIGM` in `crates/panini/tests/paradigm/data/juhotyadi.rs`. Transcribed from the spec's appendix; index 0 is the declined derivation (`adadAd`, `dadAtu`, `dehi`, `dadyAd`). `mise run fmt` re-wraps the arrays.

```rust
    ("03.0010", "laT", Pada::Parasmaipada, ["dadAti", "dattaH", "dadati", "dadAsi", "datTaH", "datTa", "dadAmi", "dadvaH", "dadmaH"]),
    ("03.0010", "laN", Pada::Parasmaipada, ["adadAd", "adattAm", "adaduH", "adadAH", "adattam", "adatta", "adadAm", "adadva", "adadma"]),
    ("03.0010", "loT", Pada::Parasmaipada, ["dadAtu", "dattAm", "dadatu", "dehi", "dattam", "datta", "dadAni", "dadAva", "dadAma"]),
    ("03.0010", "viDiliN", Pada::Parasmaipada, ["dadyAd", "dadyAtAm", "dadyuH", "dadyAH", "dadyAtam", "dadyAta", "dadyAm", "dadyAva", "dadyAma"]),
    ("03.0010", "laT", Pada::Atmanepada, ["datte", "dadAte", "dadate", "datse", "dadATe", "dadDve", "dade", "dadvahe", "dadmahe"]),
    ("03.0010", "laN", Pada::Atmanepada, ["adatta", "adadAtAm", "adadata", "adatTAH", "adadATAm", "adadDvam", "adadi", "adadvahi", "adadmahi"]),
    ("03.0010", "loT", Pada::Atmanepada, ["dattAm", "dadAtAm", "dadatAm", "datsva", "dadATAm", "dadDvam", "dadE", "dadAvahE", "dadAmahE"]),
    ("03.0010", "viDiliN", Pada::Atmanepada, ["dadIta", "dadIyAtAm", "dadIran", "dadITAH", "dadIyATAm", "dadIDvam", "dadIya", "dadIvahi", "dadImahi"]),
    ("03.0011", "laT", Pada::Parasmaipada, ["daDAti", "DattaH", "daDati", "daDAsi", "DatTaH", "DatTa", "daDAmi", "daDvaH", "daDmaH"]),
    ("03.0011", "laN", Pada::Parasmaipada, ["adaDAd", "aDattAm", "adaDuH", "adaDAH", "aDattam", "aDatta", "adaDAm", "adaDva", "adaDma"]),
    ("03.0011", "loT", Pada::Parasmaipada, ["daDAtu", "DattAm", "daDatu", "Dehi", "Dattam", "Datta", "daDAni", "daDAva", "daDAma"]),
    ("03.0011", "viDiliN", Pada::Parasmaipada, ["daDyAd", "daDyAtAm", "daDyuH", "daDyAH", "daDyAtam", "daDyAta", "daDyAm", "daDyAva", "daDyAma"]),
    ("03.0011", "laT", Pada::Atmanepada, ["Datte", "daDAte", "daDate", "Datse", "daDATe", "DadDve", "daDe", "daDvahe", "daDmahe"]),
    ("03.0011", "laN", Pada::Atmanepada, ["aDatta", "adaDAtAm", "adaData", "aDatTAH", "adaDATAm", "aDadDvam", "adaDi", "adaDvahi", "adaDmahi"]),
    ("03.0011", "loT", Pada::Atmanepada, ["DattAm", "daDAtAm", "daDatAm", "Datsva", "daDATAm", "DadDvam", "daDE", "daDAvahE", "daDAmahE"]),
    ("03.0011", "viDiliN", Pada::Atmanepada, ["daDIta", "daDIyAtAm", "daDIran", "daDITAH", "daDIyATAm", "daDIDvam", "daDIya", "daDIvahi", "daDImahi"]),
    ("03.0007", "laT", Pada::Atmanepada, ["mimIte", "mimAte", "mimate", "mimIze", "mimATe", "mimIDve", "mime", "mimIvahe", "mimImahe"]),
    ("03.0007", "laN", Pada::Atmanepada, ["amimIta", "amimAtAm", "amimata", "amimITAH", "amimATAm", "amimIDvam", "amimi", "amimIvahi", "amimImahi"]),
    ("03.0007", "loT", Pada::Atmanepada, ["mimItAm", "mimAtAm", "mimatAm", "mimIzva", "mimATAm", "mimIDvam", "mimE", "mimAvahE", "mimAmahE"]),
    ("03.0007", "viDiliN", Pada::Atmanepada, ["mimIta", "mimIyAtAm", "mimIran", "mimITAH", "mimIyATAm", "mimIDvam", "mimIya", "mimIvahi", "mimImahi"]),
    ("03.0008", "laT", Pada::Atmanepada, ["jihIte", "jihAte", "jihate", "jihIze", "jihATe", "jihIDve", "jihe", "jihIvahe", "jihImahe"]),
    ("03.0008", "laN", Pada::Atmanepada, ["ajihIta", "ajihAtAm", "ajihata", "ajihITAH", "ajihATAm", "ajihIDvam", "ajihi", "ajihIvahi", "ajihImahi"]),
    ("03.0008", "loT", Pada::Atmanepada, ["jihItAm", "jihAtAm", "jihatAm", "jihIzva", "jihATAm", "jihIDvam", "jihE", "jihAvahE", "jihAmahE"]),
    ("03.0008", "viDiliN", Pada::Atmanepada, ["jihIta", "jihIyAtAm", "jihIran", "jihITAH", "jihIyATAm", "jihIDvam", "jihIya", "jihIvahi", "jihImahi"]),
```

- [ ] **Step 5: Append the `ALTERNATES` rows**

Append to the end of `ALTERNATES` in the same file. The index is the 0-based cell (P.E P.D P.B M.E M.D M.B U.E U.D U.B); the key names the optional rules behind the form, in pipeline order.

```rust
    ("03.0010", "laN", Pada::Parasmaipada, 0, "adadAt", "8.4.56"),
    ("03.0010", "loT", Pada::Parasmaipada, 0, "dattAd", "7.1.35"),
    ("03.0010", "loT", Pada::Parasmaipada, 0, "dattAt", "7.1.35+8.4.56"),
    ("03.0010", "loT", Pada::Parasmaipada, 3, "dattAd", "7.1.35"),
    ("03.0010", "loT", Pada::Parasmaipada, 3, "dattAt", "7.1.35+8.4.56"),
    ("03.0010", "viDiliN", Pada::Parasmaipada, 0, "dadyAt", "8.4.56"),
    ("03.0011", "laN", Pada::Parasmaipada, 0, "adaDAt", "8.4.56"),
    ("03.0011", "loT", Pada::Parasmaipada, 0, "DattAd", "7.1.35"),
    ("03.0011", "loT", Pada::Parasmaipada, 0, "DattAt", "7.1.35+8.4.56"),
    ("03.0011", "loT", Pada::Parasmaipada, 3, "DattAd", "7.1.35"),
    ("03.0011", "loT", Pada::Parasmaipada, 3, "DattAt", "7.1.35+8.4.56"),
    ("03.0011", "viDiliN", Pada::Parasmaipada, 0, "daDyAt", "8.4.56"),
```

- [ ] **Step 6: Update `paradigm/main.rs`'s totals and prose**

In `derivation_set_shape_matches_the_audited_numbers`:
- `assert_eq!(total_cells, 3636, "404 root×lakāra blocks × 9 cells each")` → `3852`, `"428 root×lakāra blocks × 9 cells each"`.
- `assert_eq!(ones, 2925, ...)` → `3133`; `assert_eq!(twos, 550, ...)` → `554`.
- `assert_eq!(threes, 117, ...)` → `121`, and append to its message: `; and — new in slice 3c — √dā's and √dhā's, the same way`.
- `fours` 18, `fives` 9, `sixes` 17 are unchanged.
- `assert_eq!(ALTERNATES.len(), 959, ...)` → `971`.
- `key_count("8.4.56")` 138 → `142`; `key_count("7.1.35")` 116 → `120`; `key_count("7.1.35+8.4.56")` 116 → `120`.

Check: 3133 + 554 + 121 + 18 + 9 + 17 = 3852 cells; 3133 + 1108 + 363 + 72 + 45 + 102 = 4823 forms = 3852 + 971.

In the test's doc comment, immediately before `/// This test is what keeps the numbers true day to day.`, add:

```rust
///
/// Slice 3c curates four more juhotyādi roots — √dā (`03.0010`) and √dhā
/// (`03.0011`), ubhayapadī by 1.3.72, and √mā (`03.0007`) and √hā
/// (`03.0008`), ātmanepadī by 1.3.12 — bringing the gaṇa to eight of its
/// twenty-six rows. Its machinery (1.1.20 as `Tag::Ghu`, 7.4.76, 6.4.119,
/// 6.1.88, 8.2.38, the 6.4.113/6.4.112 abhyasta arms, 8.2.40's *adhaḥ*)
/// adds no vikalpa and no fork kind: √dā and √dhā fork exactly where every
/// -oti parasmaipada root does — laṅ and vidhiliṅ prathama eka on 8.4.56
/// (`adadAd`/`adadAt`, `dadyAd`/`dadyAt`) and the two loṭ tātaṅ cells three
/// ways (`dadAtu`/`dattAd`/`dattAt`, `dehi`/`dattAd`/`dattAt`) — and their
/// ātmanepada columns, √mā and √hā fork nowhere. Twelve new `ALTERNATES`
/// rows, four apiece on `8.4.56` (138→142), `7.1.35` (116→120) and
/// `7.1.35+8.4.56` (116→120). The gaṇa is PARTIAL at 8 of its 26 rows.
```

In the fork-distribution doc comment above it (~line 287), change `3636 cells total (404 root×lakāra blocks × 9), of which 2925 hold exactly` / `one form, 550 hold two, 117 hold three (√hrī's loṭ prathama and madhyama` / `eka, new in slice 3b, each by 7.1.35/8.4.56)` so it reads `3852 cells total (428 root×lakāra blocks × 9), of which 3133 hold exactly one form, 554 hold two, 121 hold three (√hrī's loṭ prathama and madhyama eka, new in slice 3b, and √dā's and √dhā's, new in slice 3c, each by 7.1.35/8.4.56)`. In the key census (~line 324), change `itself has 959 rows, keyed 138 \`8.4.56\`, 116 \`7.1.35\`, 116 \`7.1.35+8.4.56\`,` to `itself has 971 rows, keyed 142 \`8.4.56\`, 120 \`7.1.35\`, 120 \`7.1.35+8.4.56\`,`, and after `keys since 7.3.86 —` add `slice 3c (√dā, √dhā, √mā, √hā) adds twelve more, all in those same three keys, four apiece (\`8.4.56\` 138→142, \`7.1.35\` 116→120, \`7.1.35+8.4.56\` 116→120) —`.

- [ ] **Step 7: Run the full suite**

Run: `mise run test 2>&1 | tail -40` (foreground, timeout ≥ 600000 ms)
Expected: PASS at 3852 cells; every 3636 prior unchanged.

If a new cell fails, the engine and the goldens disagree. Read the failing form against the spec's appendix and the "traces that decide the slice" before touching either, and use superpowers:systematic-debugging. **Do not edit a golden to match the engine** — the goldens are vidyut's output and are the specification.

- [ ] **Step 8: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-data/src/lib.rs crates/panini/tests/paradigm/data/juhotyadi.rs crates/panini/tests/paradigm/main.rs
git commit -m "feat(data): √dā, √dhā, √mā and √hā — juhotyādi at eight of twenty-six rows

3636 → 3852 cells, 4595 → 4823 forms, 959 → 971 ALTERNATES, 81 → 85 roots.

No new fork kind: √dā and √dhā fork where every -oti parasmaipada root
does, twelve rows in the 8.4.56 / 7.1.35 / 7.1.35+8.4.56 keys, and √mā's
and √hā's columns fork nowhere."
```

---

## Task 8: The nine trace pins

**Files:**
- Modify: `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes: `cell_trace(number, lakara, pada, purusha, vacana) -> (String, Vec<String>)` and `at(&[String], &str) -> usize` from `crate::helpers` (already imported). `cell_trace` returns branch 0 — the declined derivation — so a forked cell's pin reads its no-optional-rule form (`dehi`, not `dattAd`).

- [ ] **Step 1: Write the nine pins**

Append to `crates/panini/tests/trace/juhotyadi.rs`:

```rust
#[test]
fn mimite_trace_orders_hrasvah_bhrnam_it_then_i_halyaghoh() {
    // mA Ā laT P.E. 7.4.59 shortens the abhyāsa (ma), 7.4.76 makes it mi,
    // and 6.4.113's abhyasta arm turns the aṅga's ā to ī before the kṅit
    // hal-initial te. No 6.4.112 (a consonant follows and √mā is not ghu)
    // and no 7.4.62 (m is no velar).
    let (text, t) = cell_trace(
        "03.0007",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "mimIte", "got {t:?}");
    assert!(at(&t, "7.4.59") < at(&t, "7.4.76"), "got {t:?}");
    assert!(at(&t, "7.4.76") < at(&t, "6.4.113"), "got {t:?}");
    assert!(!t.contains(&"6.4.112".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.4.62".to_string()), "got {t:?}");
}

#[test]
fn dehi_trace_credits_6_4_119_and_neither_6_4_112_nor_6_4_101() {
    // dA P loT M.E, branch 0 (no tātaṅ). 6.4.119 gives de and elides the
    // abhyāsa; 6.4.112 never sees an ā, and 6.4.101 sees `e`, not a jhal,
    // before hi.
    let (text, t) = cell_trace(
        "03.0010",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "dehi", "got {t:?}");
    assert!(at(&t, "6.1.10") < at(&t, "6.4.119"), "got {t:?}");
    for absent in ["6.4.112", "6.4.113", "6.4.101", "7.1.35"] {
        assert!(!t.contains(&absent.to_string()), "{absent} in {t:?}");
    }
}

#[test]
fn dattah_trace_credits_6_4_112_and_not_6_4_113_by_aghoh() {
    // dA P laT P.D. √dā is ghu, so 6.4.113 declines before the hal-initial
    // tas and 6.4.112 elides the ā; 8.4.55 then devoices d before t. No
    // 8.2.38: the identical shape is √dhā's alone.
    let (text, t) = cell_trace(
        "03.0010",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "dattaH", "got {t:?}");
    assert!(at(&t, "6.4.112") < at(&t, "8.4.55"), "got {t:?}");
    assert!(!t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.2.38".to_string()), "got {t:?}");
}

#[test]
fn dhattah_trace_orders_car_ca_then_dadhas_tathos_ca_then_khari_ca() {
    // DA P laT P.D. 8.4.54 deaspirates the abhyāsa, 8.2.38 — out of sūtra
    // order — re-aspirates it, 8.4.55 devoices the root's D. 8.2.40's adhaḥ
    // keeps the t a t.
    let (text, t) = cell_trace(
        "03.0011",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "DattaH", "got {t:?}");
    assert!(at(&t, "6.4.112") < at(&t, "8.4.54"), "got {t:?}");
    assert!(at(&t, "8.4.54") < at(&t, "8.2.38"), "got {t:?}");
    assert!(at(&t, "8.2.38") < at(&t, "8.4.55"), "got {t:?}");
    assert!(!t.contains(&"8.2.40".to_string()), "got {t:?}");
}

#[test]
fn dhaddhve_trace_orders_jas_jhasi_then_car_ca_then_dadhas_tathos_ca() {
    // DA Ā laT M.B. 8.4.53 voices the root's D before Dve, 8.4.54
    // deaspirates the abhyāsa, 8.2.38 re-aspirates it before dhv.
    let (text, t) = cell_trace(
        "03.0011",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Madhyama,
        Vacana::Bahu,
    );
    assert_eq!(text, "DadDve", "got {t:?}");
    assert!(at(&t, "8.4.53") < at(&t, "8.4.54"), "got {t:?}");
    assert!(at(&t, "8.4.54") < at(&t, "8.2.38"), "got {t:?}");
}

#[test]
fn dadhati_trace_carries_no_dadhas_tathos_ca() {
    // DA P laT P.E. The ti is pit, so the ā survives and there is no dadh:
    // 8.2.38's single-consonant test declines. 8.4.54 still deaspirates.
    let (text, t) = cell_trace(
        "03.0011",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "daDAti", "got {t:?}");
    assert!(t.contains(&"8.4.54".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.2.38".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.112".to_string()), "got {t:?}");
}

#[test]
fn dadai_trace_is_atas_ca_then_vrddhir_eci_not_akah_savarne() {
    // dA Ā loT U.E. The Kaumudī's path: the āṭ merges with its own ending
    // (6.1.90), then the root's ā with that (6.1.88). 6.1.101 must NOT
    // appear: its adādi arm declines on āṭ + ec. The form is the same either
    // way, so this pin is what holds the decline.
    let (text, t) = cell_trace(
        "03.0010",
        Lakara::Lot,
        Pada::Atmanepada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "dadE", "got {t:?}");
    assert!(at(&t, "6.1.90") < at(&t, "6.1.88"), "got {t:?}");
    assert!(!t.contains(&"6.1.101".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.112".to_string()), "got {t:?}");
}

#[test]
fn adaduh_trace_credits_6_4_112_not_usy_apadantat() {
    // dA P laN P.B. 3.4.109 makes jhi into us, which keeps 1.2.4's ṅit, so
    // 6.4.112 elides the ā before it. 6.1.96's junction arm spells the same
    // form and must not fire.
    let (text, t) = cell_trace(
        "03.0010",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "adaduH", "got {t:?}");
    assert!(at(&t, "3.4.109") < at(&t, "6.4.112"), "got {t:?}");
    assert!(!t.contains(&"6.1.96".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.83".to_string()), "got {t:?}");
}

#[test]
fn dadate_trace_credits_6_4_112_not_akah_savarne() {
    // dA Ā laT P.D. The ā goes by 6.4.112 before the kṅit Ate; 6.1.101
    // would spell the same dadAte, which is why this is pinned.
    let (text, t) = cell_trace(
        "03.0010",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "dadAte", "got {t:?}");
    assert!(t.contains(&"6.4.112".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.1.101".to_string()), "got {t:?}");
}
```

- [ ] **Step 2: Run the trace suite**

Run: `mise exec -- cargo test -p panini --test trace 2>&1 | tail -30`
Expected: PASS (~5 s).

A failing pin here with a passing paradigm golden means the form is right but the path is not. That is exactly what these pins exist to catch — find which rule fired instead (print `t`) and fix the engine's guard or order, not the pin.

- [ ] **Step 3: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini/tests/trace/juhotyadi.rs
git commit -m "test(trace): nine 3c pins — ghu, dadh, dadai and the rules that spell the same form

mimIte pins 7.4.59 < 7.4.76 < 6.4.113; dehi pins 6.4.119 with no 6.4.112 or
6.4.101; dattaH pins aghoḥ; DattaH and DadDve pin 8.2.38 after 8.4.54 and
8.2.40's adhaḥ; daDAti pins 8.2.38's lost-ā test. dadE, adaduH and dadAte
pin the rule that fires where another would spell the same form: 6.1.90 +
6.1.88 over 6.1.101, 6.4.112 over 6.1.96 and over 6.1.101."
```

---

## Task 9: Audit, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (audit prose), `crates/panini-prakriya/src/tinanta/{abhyasa,guna,sound}.rs` (stale comments), `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md` (one note)

**Interfaces:**
- Consumes: the finished engine and goldens (Tasks 1–8). Produces no symbols.

- [ ] **Step 1: Update the audit harness's asserted totals**

In `tools/audit/panini_full_audit.rs`:
- Header: `for each of the 81 curated roots` → `85`; `two apiece for the twenty roots that admit both padas — nineteen ubhayapadī by 1.3.72, plus √bhuj by 1.3.66` → `two apiece for the twenty-two roots that admit both padas — twenty-one ubhayapadī by 1.3.72, plus √bhuj by 1.3.66`; `81 roots, 3636 cells, 4595 forms` → `85 roots, 3852 cells, 4823 forms`; `404 root×pada×lakāra blocks × 9 cells, plus 959` → `428 … plus 971`; `the full 3636-cell table` → `3852-cell`.
- Asserts: `assert_eq!(roots_seen.len(), 81, …)` → `85`; `assert_eq!(n_cells, 3636, "cells: 404 root×pada×lakāra blocks × 9")` → `3852`, `"cells: 428 root×pada×lakāra blocks × 9"`; `assert_eq!(n_forms, 4595, "forms: 3636 cells + 959 ALTERNATES rows")` → `4823`, `"forms: 3852 cells + 971 ALTERNATES rows"`.

In `tools/audit/README.md`, `(81 roots, 3636 cells, 4595 forms)` → `(85 roots, 3852 cells, 4823 forms)`.

- [ ] **Step 2: Repoint vidyut's dev-deps at THIS worktree and run the audit**

`/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes absolute dev-dep paths and currently points at `/workspace/crates` — the `main` checkout, which does not have this slice. Auditing without repointing checks the pre-slice engine and fails on its totals.

```bash
WT="$(git rev-parse --show-toplevel)"
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
cp tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" cargo run --release --example panini_full_audit 2>&1 | tail -15)
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry cargo run --release --example panini_full_audit 2>&1 | tail -8)
```

Copy the committed harness — never rewrite it. Expected: `AUDIT PASSED: 3852 cells, 4823 forms, zero differences.`; the `entry` control fails with exit 1 and 36 √bhū cells. If the honest run shows differences, stop: the goldens passed, so the engine and vidyut disagree on a form the goldens do not pin — report it rather than editing anything.

- [ ] **Step 3: Record the audit**

In `tools/audit/README.md`, immediately under `## Last recorded result`, add a new dated entry above the 3b one (use today's date):

```markdown
YYYY-MM-DD, juhotyādi 3c slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 3852
cells / 4823 forms / 85 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole juhotyādi 3c slice: the dhātupāṭha number
reaching the pipeline (`Context.dhatupatha`, with 1.1.20 as `Tag::Ghu`), four
new rules (7.4.76 *bhṛñām it*, 6.4.119 *ghvasor eddhāv abhyāsalopaś ca*,
6.1.88 *vṛddhir eci*, 8.2.38 *dadhas tathoś ca*), 6.4.113 moved above 6.4.112
with both given abhyasta arms, 6.1.101's āṭ + ec decline and 8.2.40's *adhaḥ* —
added for √dā (`03.0010`), √dhā (`03.0011`), √mā (`03.0007`) and √hā
(`03.0008`).

Totals: 85 = 81 + 4; 3852 = 3636 + 216 (24 root×pada×lakāra blocks × 9 —
√dā and √dhā two padas × four lakāras each, √mā and √hā one × four); 4823 =
4595 + 216 + 12 new `ALTERNATES` rows (959 → 971), measured via the
harness's corpus block, not assumed.
```

In `crates/panini/tests/paradigm/main.rs`'s audit-chain doc comment, replace `forms / 81 roots with zero differences, its \`entry\` negative control` / `verified failing (36 √bhū cells). √tṛh` so the sentence continues: `… 3636 cells / 4595 forms / 81 roots with zero differences, its \`entry\` negative control verified failing (36 √bhū cells), and juhotyādi 3c's cross-implementation audit re-ran the same probe against vidyut-prakriya at the same commit \`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea\` over all 3852 cells / 4823 forms / 85 roots with zero differences, its \`entry\` negative control verified failing (36 √bhū cells). √tṛh`.

- [ ] **Step 4: README.md**

- `**partial** at 4 of its 26 dhātupāṭha rows` → `**partial** at 8 of its 26 dhātupāṭha rows`.
- Replace `landing as the engine's ninth vikalpa rule — root-keyed to √bhī, forking` / `across all four lakāras, and stacking with 7.1.35 in loṭ. rudhādi is` so the juhotyādi sentence ends: `… and stacking with 7.1.35 in loṭ; and √dā (\`03.0010\`, *dadāti*) and √dhā (\`03.0011\`, *dadhāti*), both ubhayapadī, and √mā (\`03.0007\`, *mimīte*) and √hā (\`03.0008\`, *jihīte*), both ātmanepadī, curated in slice 3c behind the dhātupāṭha number reaching the pipeline — 1.1.20 *dādhā ghv adāp* becomes a saṁjñā decided by row, since √dāp is \`dA\` too — and 7.4.76 *bhṛñām it*, 6.4.119 *ghvasor eddhāv abhyāsalopaś ca*, 6.1.88 *vṛddhir eci* and 8.2.38 *dadhas tathoś ca* (run after 8.4.54, out of sūtra order), with 6.4.113 moved above 6.4.112 and both given abhyasta arms, and 8.2.40 given its *adhaḥ*. rudhādi is`.
- `over a` / `curated 81-root set` → `curated 85-root set`.
- `711 of the 3636 cells hold more than one form: 550` / `hold two, 117 hold three (… and — new in` / `slice 3b — √hrī's loṭ prathama and madhyama eka, each by 7.1.35/8.4.56),` → `719 of the 3852 cells hold more than one form: 554 hold two, 121 hold three (\`Bavatu\`, \`BavatAd\`, \`BavatAt\`, and — new in slice 3b — √hrī's loṭ prathama and madhyama eka, and — new in slice 3c — √dā's and √dhā's, each by 7.1.35/8.4.56),`.

Reflow the paragraph to the file's existing ~80-column wrap.

- [ ] **Step 5: docs/ARCHITECTURE.md**

- Stage table: `abhyasa.rs` row → `6.1.10, 7.4.60, 7.4.59, 7.4.62, 7.4.76`; `guna.rs` row ending `7.3.101, 6.4.112, 6.4.113, 6.4.115` → `7.3.101, 6.4.119, 6.4.113, 6.4.112, 6.4.115`; `adesha.rs` row `6.1.90 … 6.4.101` → `6.1.90, 6.1.88 … 6.4.101`; `tripadi.rs` row `8.4.53, 8.4.54, 8.4.55` → `8.4.53, 8.4.54, 8.2.38, 8.4.55`.
- `pins all 110 ids verbatim` → `pins all 114 ids verbatim` (confirm by counting the `expected` array entries in `tinanta_rule_order_is_pinned`), and after `this slice widened them rather than adding ids)` insert ` — 110 total — then juhotyādi 3c's four: 7.4.76 *bhṛñām it*, 6.4.119 *ghvasor eddhāv abhyāsalopaś ca*, 6.1.88 *vṛddhir eci* and 8.2.38 *dadhas tathoś ca* — 114 total`.
- After the paragraph ending `The exact ordered traces in` / `` `crates/panini/tests/trace/` are what pin them. `` add:

```markdown
Two orderings from juhotyādi 3c are worth naming here. **8.2.38 *dadhas
tathoś ca* runs after 8.4.54**, against sūtra order: it names *dadh*, the
reduplicated √dhā after 8.4.54 has deaspirated its abhyāsa, so in sūtra
position 8.4.54 would undo it (*DattaH*, not \**dattaH*; vidyut orders it the
same way). **6.4.113 *ī halyaghoḥ* runs above 6.4.112 *śnābhyastayor ātaḥ***
as its apavāda, which is what lets 6.4.112 carry no vowel-initial test and
still elide a ghu root's ā before consonants (*dattaH*). The *DattaH*,
*DadDve* and *dattaH* trace pins hold both.
```

- `**partial** at 4 of its 26 rows (√hu,` / `√ki; slice 3a; √bhī, √hrī; slice 3b)` → `**partial** at 8 of its 26 rows (√hu, √ki; slice 3a; √bhī, √hrī; slice 3b; √dā, √dhā, √mā, √hā; slice 3c)`.
- After `read by 3.1.69,` / `3.1.73, 3.1.77, 3.1.78, 3.1.79, 3.1.81, 2.4.72 and 2.4.75.` add: `Root identity beyond text reaches the pipeline as \`Context.dhatupatha\`, the row number \`derive\` stamps (slice 3c). A sūtra naming particular roots matches it (7.4.76, 8.2.38, 8.2.40's *adhaḥ*); a sūtra naming a class becomes a saṁjñā tag decided from it — \`Tag::Ghu\`, 1.1.20's six rows in \`tinanta/samjna.rs\`'s \`GHU\` — because text cannot separate \`03.0008 o~hA\N\` from \`03.0009 o~hA\k\` (both \`hA\`) or ghu \`03.0010\` from \`02.0054 dA\p\` (both \`dA\`).`
- The 7.1.35 / 8.4.56 accounting: `forking 116 cells` → `120`; `across the 58 roots with a parasmaipada column` → `60`; `the curated set's 23 ātmanepada-only` → `25`; `the twenty roots that admit both padas` → `the twenty-two`; `(nineteen ubhayapadī by 1.3.72 —` → `(twenty-one ubhayapadī by 1.3.72 —`; `√ghṛ and √kṛ —` → `√ghṛ, √kṛ, √dā and √dhā —`; `58 + 23 = the 81 curated` → `60 + 25 = the 85 curated`; `forking 138 cells outright` → `142`; `across those same 58 parasmaipada columns (116 of` → `60 … (120 of`.

- [ ] **Step 6: AGENTS.md**

- Rules of the codebase: `3636 cells, nine gaṇas` → `3852 cells`; `opened in slice 3a at 2 of its 26 rows, now at 4 of its 26 after slice` / `3b curated √bhī and √hrī —` → `opened in slice 3a at 2 of its 26 rows, at 4 after slice 3b curated √bhī and √hrī, and now at 8 of its 26 after slice 3c curated √dā, √dhā, √mā and √hā (ātmanepada) —`; `(959 rows in all, so 3636 + 959 = 4595 forms total)` → `(971 rows in all, so 3852 + 971 = 4823 forms total)`.
- The audit record: after `itself superseded by juhotyādi 3b's` / `own audit (… 2026-09-09 entry, 3636 cells / 4595` / `forms / 81 roots)` add `, and that by juhotyādi 3c's (\`tools/audit/README.md\`'s YYYY-MM-DD entry, 3852 cells / 4823 forms / 85 roots)`.
- The stale-comment paragraph: `3636 goldens` / `would move today` → `3852`. After `The corpus` / `stands at 3636 cells as of juhotyādi 3b.` add `Juhotyādi 3c touched neither comment either; the corpus stands at 3852 cells as of 3c.` Then recompute both anchors rather than guessing: `grep -n "1872 goldens" crates/panini-prakriya/src/tinanta/guna.rs` gives the new `guna.rs` line (3c adds rules and tests above it), and `git diff main -- crates/panini-prakriya/src/controller.rs` is empty, so `controller.rs:153` stands. Write the measured `guna.rs` line in.
- Add a Rules-of-the-codebase bullet immediately after the `When a rule's substitute set widens…` bullet:

```markdown
- **A guard that names particular roots keys on the dhātupāṭha number
  wherever the root text is ambiguous.** `ctx.dhatupatha` carries the row
  `derive` was called with, or `""` on a hand-built prakriyā, which every
  such guard declines. 6.4.87, 6.4.101 and 6.4.115 still key on `ANGA.text`
  (`hu`, `BI`), safe only because `juhotyadi_rows_are_the_eight_curated_roots`
  asserts those codes stay unique; 7.4.76, 8.2.38 and 8.2.40's *adhaḥ* key on
  numbers, because `03.0008` and `03.0009` share `hA`. A sūtra naming a
  class of roots becomes a saṁjñā tag decided from the number in `derive` —
  `Tag::Ghu` from `tinanta/samjna.rs`'s `GHU` — pinned to the vendored TSV
  rather than to a derivation, so its uncurated members are held too.
```

- [ ] **Step 7: Stale engine comments**

- `abhyasa.rs`: both `the 3636 priors break` → `the 3852 priors break`.
- `guna.rs`: in 6.4.77's comment, `The 3636 byte-identical` → `The 3852 byte-identical`.
- `sound.rs`, `hrasva_of_long_vowels_all_arms`: replace `Only the \`I\` arm has a` / `cell in slice 3b (√bhī's \`BI\` and √hrī's \`hI\`); \`A\` arrives with` / `√dā in 3c and \`F\` with √pṝ in 3d.` with `The \`I\` arm has cells from slice 3b (√bhī's \`BI\`, √hrī's \`hI\`) and the \`A\` arm from slice 3c (√dā, √dhā, √mā, √hā); \`F\` arrives with √pṝ in 3d.`
- `sound.rs`, `deaspirate_of_aspirate_stops_all_arms`: `Only J -> j (√hu)` / `is reachable from the golden forms in slice 3a, B -> b in 3b;` → `J -> j (√hu) is reachable from the golden forms in slice 3a, B -> b in 3b and D -> d in 3c (√dhā);`.

- [ ] **Step 8: Note the split in 3a's spec**

In `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md`, immediately after the "Later slices" table, add:

```markdown
> Slice 3c's spec (`2026-09-11-juhotyadi-gana-3c-design.md`) split this
> table's 3c row: 3c took √dā, √dhā, √mā and √hā (ātmanepada), 216 cells;
> √hā (parasmaipada) and √gā became slice 3c2.
```

- [ ] **Step 9: Sweep for anything left stale**

```bash
grep -rn "3636\|4595\|\b959\b\|81 roots\|of these 81\|\b404\b\|711 of\|4 of its 26\|4 of 26\|58 + 23\|110 ids" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.md --include=*.rs
grep -rn "slice 3c\|3c–3f\|arrive in slice 3c\|arrives in 3c\|in 3c\b" crates --include=*.rs
grep -rn "\bdA\b\|\bDA\b\|\bmA\b\|\bhA\b\|√dā\|√dhā\|√mā\|√hā" crates/panini-prakriya/src --include=*.rs | grep "//"
grep -rn "6\.4\.112\|6\.4\.113\|8\.2\.40\|6\.1\.101\|6\.1\.96" crates/panini-prakriya/src --include=*.rs | grep -i "defer\|later slice\|not implemented\|no curated\|not yet\|sole witness\|only √indh"
```

Expected residue only: AGENTS.md's dated mutation-record entries (history — never rewrite them) and prose that names 3b's numbers explicitly as 3b's. Every comment that says 3c *will* do something must now say it did. Fix any other hit.

- [ ] **Step 10: Run the full suite**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout ≥ 600000 ms)
Expected: PASS at 3852 cells. The prose sites sit in doc comments on live test functions, so a bad edit breaks compilation.

- [ ] **Step 11: Commit**

```bash
mise run fmt && mise run lint
git add -A
git commit -m "docs: 3c's counts, the audit record, and the comments it falsifies

3852 cells / 4823 forms / 85 roots / 971 ALTERNATES across README,
ARCHITECTURE, AGENTS, paradigm/main.rs and tools/audit. Audit at zero
divergence against 8da2f90b.

ARCHITECTURE names the two 3c orderings (8.2.38 after 8.4.54; 6.4.113 above
6.4.112) and how root identity reaches the pipeline; AGENTS gains the rule
for number-keyed guards. The hrasva_of and deaspirate_of table comments now
name 3c's witnesses."
```

---

## Task 10: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and a dated campaign entry); `mise.toml` only if the cap must move

Follow AGENTS.md's cargo-mutants protocol exactly. Three hazards from this repo's record apply: **measure, never scale**; **every invocation rotates `mutants.out`, so always pass `-o`**; **the mise shim fails in background shells and background shells die at ~60 minutes**, so the campaign runs detached on the real binary.

- [ ] **Step 1: Run the exhaustive tier once**

Run: `mise run test-full 2>&1 | tail -20` (foreground, timeout 3600000 ms; ~25 minutes)
Expected: PASS, including `roundtrip_exhaustive` over all 4823 forms.

- [ ] **Step 2: Measure the uncontended floor**

With nothing else running: `time mise run test 2>&1 | grep -E "Running|finished in|real"` (foreground). Record the wall clock and the paradigm, roundtrip and trace binaries' times. Compare with the 3636-cell floor (65.01 s: paradigm 27.60 s, roundtrip 31.30 s, trace 4.72 s). `roundtrip_sampled` is Θ(N²) in roots, so expect it to grow faster than the cell count.

- [ ] **Step 3: Measure a full UNCAUGHT run at `-j 4`**

The two documented equivalent mutants run the suite to completion uncaught. Their line numbers have moved; find them by source text, not by number:

```bash
git show main:crates/panini-prakriya/src/tinanta/adesha.rs | sed -n 519p   # the `s.remove(pos + 1);` line
git show main:crates/panini-prakriya/src/tinanta/tripadi.rs | sed -n 1176p # the `- 1` line
grep -n "s.remove(pos + 1);" crates/panini-prakriya/src/tinanta/adesha.rs
grep -nF "$(git show main:crates/panini-prakriya/src/tinanta/tripadi.rs | sed -n 1176p)" crates/panini-prakriya/src/tinanta/tripadi.rs
CM="$(mise which cargo-mutants)"
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:<NEW_LINE>:30: replace \+ with \*|tripadi.rs:<NEW_LINE>:38: replace - with /"
```

Substitute the two new line numbers the greps print for `<NEW_LINE>`, confirm the `--list` output shows exactly those two mutants, then run them (foreground, timeout 1800000 ms):

```bash
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 600 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:<NEW_LINE>:30: replace \+ with \*" --re "tripadi.rs:<NEW_LINE>:38: replace - with /" 2>&1 | tail -10
```

Both must be MISSED (not TIMEOUT). Read each one's test-phase duration from `$SCRATCH/mutants.out/outcomes.json`. **Cap rule:** keep `--timeout 600` if 600 ÷ the longer test phase is at least 5×; otherwise set the cap to 6 × the longer phase rounded up to the next 100 s and change `mise.toml` and AGENTS.md's floor paragraph together.

- [ ] **Step 4: Launch the campaign detached**

```bash
OUT="$HOME/mutants-records/juhotyadi-3c"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
CM="$(mise which cargo-mutants)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 600 -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"
```

(Use the cap from Step 3 if it moved.) The 3636-cell campaign took 1 h 22 min. Check progress with `tail -3 "$OUT/campaign.log"` and liveness with `pgrep -x cargo-mutants` — **not** `pgrep -f`, which matches its own shell. Run nothing CPU-heavy in the meantime; contention inflates test phases toward the cap.

- [ ] **Step 5: Read both outcome files**

When `pgrep -x cargo-mutants` returns nothing:

```bash
date -u +"%F %T UTC" > "$OUT/finished"
tail -5 "$OUT/campaign.log"
cat "$OUT/mutants.out/missed.txt" "$OUT/mutants.out/timeout.txt"
```

Expected: `missed.txt` holds exactly the two documented equivalents at their Step 3 positions; `timeout.txt` holds exactly the permanent `tripadi.rs` ṇatva-scan `replace -= with /=`. `cargo-mutants` exits 3 when timeouts are present — expected.

- Any **other timeout** is a suspect survivor: re-run it alone with its own `-o` directory and `--re` before concluding anything.
- Any **other missed** mutant in 3c's code means its guard test does not separate the mutant. Likeliest: 6.1.101's `starts_with('A')` sub-clause, 6.1.88's `SHAP.is_empty()`, 8.2.38's single-consonant test, 6.4.119's `len() <= ENDING`, 6.4.113's `Ghu` test. Strengthen the named guard test (Tasks 2–6), commit it, and re-run only those mutants with `--re` and a fresh `-o`.

- [ ] **Step 6: Keep the outcomes and compute the margins**

`$OUT` is already durable; do not run another `cargo-mutants` against it. Inspect one record first (`python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["outcomes"][1])' "$OUT/mutants.out/outcomes.json"`) to see how `duration` is stored, then compute the caught mutants' test-phase min / median / p90 / p99 / max and the two margins AGENTS.md records: 600 ÷ the longest uncaught test phase (Step 3, and the two missed mutants' phases in this campaign), and 600 ÷ the slowest caught mutant.

- [ ] **Step 7: Record it in AGENTS.md**

- Rewrite the `**The floor behind the 600s cap, measured at 3636 cells.**` paragraph with Step 2's and Step 3's measured numbers at 3852 cells (or the new cap, with `mise.toml` changed in the same commit).
- Append a dated entry after the `2026-09-11 — the floor series has a hinge here` entry, in its style: cell growth 3636 → 3852 (+5.94%) and the floor's measured move; the campaign window (`started`/`finished`) and wall clock; **mutants / caught / unviable / missed / timeout** counts summing to the total; `missed.txt` and `timeout.txt` **named verbatim with their new positions**; the caught-mutant duration distribution; both margins; that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`; and the non-caught set diffed against the N² slice's (identical set, drifted positions, is the clean result).

- [ ] **Step 8: Commit**

```bash
git add AGENTS.md mise.toml
git commit -m "chore: 3c mutation gate — floor and uncaught run re-measured at 3852 cells

missed.txt holds only the two documented equivalents and timeout.txt only
the permanent ṇatva-scan entry, at drifted positions; the cap was checked
against a measured -j 4 uncaught run, not scaled."
```

---

## Task 11: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -20
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin juhotyadi-3c
gh pr create --title "juhotyādi 3c — √dā, √dhā, √mā, √hā (ātmanepada)" --body "$(cat <<'BODY'
Slice 3c curates √dā (`03.0010`), √dhā (`03.0011`), √mā (`03.0007`) and √hā
ātmanepada (`03.0008`), taking the golden suite from 3636 to 3852 cells and
juhotyādi from four of its twenty-six rows to eight.

**Row identity reaches the pipeline.** Root text cannot tell `03.0008` from
`03.0009` (both `hA`) or ghu `03.0010` from √dāp `02.0054` (both `dA`), so
`Context` carries the dhātupāṭha number and 1.1.20 lands as `Tag::Ghu`,
pinned to the vendored TSV.

**New rules:** 7.4.76 *bhṛñām it*, 6.4.119 *ghvasor eddhāv abhyāsalopaś ca*,
6.1.88 *vṛddhir eci*, 8.2.38 *dadhas tathoś ca* (after 8.4.54, out of sūtra
order). **Restructured:** 6.4.113 above 6.4.112 with abhyasta arms and
*aghoḥ*; 6.1.101 leaves āṭ + ec to 6.1.90; 8.2.40 gains *adhaḥ*.

The pre-recorded bundle was re-probed against vidyut and against this
engine's own HEAD, cell by cell: 8.2.40's exclusion and the 6.4.113/6.4.112
swap were unlisted. Fourteen cells reach the right form by a different rule
on HEAD, so nine trace pins hold the paths as well as the forms.

Audit at zero divergence against `8da2f90b`; mutation gate clean.

https://claude.ai/code/session_01GNZF73bdEJkA2hEgJw5D9s
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Per the standing instruction: once checks are green, merge (`gh pr merge --merge --auto`), confirm the branch's commits are on `main` (`git fetch origin && git branch -r --contains "$(git rev-parse HEAD)"` lists `origin/main`), then delete the remote and local branch and remove the worktree (`git worktree remove .worktrees/juhotyadi-3c` from the main checkout).

---

## Self-Review

**Spec coverage.** Row identity (`Context.dhatupatha`, `Tag::Ghu`, `GHU` and its upstream test) → Task 1. 7.4.76 → Task 2. 6.4.113 above 6.4.112, both abhyasta arms, *aghoḥ*, the vowel-test removal → Task 3. 6.4.119 → Task 4. 6.1.101's āṭ + ec decline and 6.1.88 → Task 5. 8.2.40's *adhaḥ*, 8.2.38 after 8.4.54, and the falsified "only √indh" comments → Task 6. Root selection, counts, the 12 `ALTERNATES` rows, the eight-root test → Task 7. The nine trace pins → Task 8. The audit, the doc sweep, ARCHITECTURE's two named orderings and root-identity note, AGENTS's number-keyed-guard rule, the 3a spec note → Task 9. `test-full`, the re-measured floor and uncaught run, the campaign, the verbatim non-caught record → Task 10. The spec's "existing machinery" (8.4.53, 8.4.54, `hrasva_of`, 3.4.109, 7.1.4, 7.1.5, 6.1.66, 6.1.90's athematic arm, 6.1.96's ending arm, 8.3.59, 8.4.55, the forks, the data layer) needs no code and is witnessed by Task 7's goldens and Task 8's pins. 3c2 is recorded in the spec and has no task.

**Type consistency.** `Context.dhatupatha: &'static str` (Task 1) is read as `p.ctx.dhatupatha` in Tasks 2 and 6 and set in their tests. `Tag::Ghu` (Task 1) is read in Tasks 3 and 4. `samjna::GHU: [&str; 6]` is used only by `derive` and its own test. `abhyasta_prakriya(&str, &str, bool, &str, bool) -> Prakriya` is defined in Task 3 and reused in Task 4. `dadh_prakriya(&str, &str) -> Prakriya` is defined and used only in Task 6. Rule ids `"7.4.76"`, `"6.4.119"`, `"6.4.113"`, `"6.4.112"`, `"6.1.88"`, `"8.2.38"` match across rule bodies, the pinned order edits (Tasks 2–6, applied cumulatively) and Task 8's assertions.

**Known soft spots.** Two decisions are form-neutral — 6.1.101's decline and 8.2.38's position — and are held by guard tests plus Task 8's `dadE`, `DattaH` and `DadDve` pins, which is why Task 8's Step 2 says to fix the engine, never the pin. Task 9's audit depends on repointing vidyut's dev-deps at this worktree first. Task 10's Step 3 locates the two equivalent mutants by source text because their line numbers have drifted.

