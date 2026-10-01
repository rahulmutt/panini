# Juhotyādi gaṇa slice 3f Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate √kit, √tur, √dhiṣ and √dhan (`03.0021`–`03.0024`, *ciketti* / *tutorti* / *diDezwi* / *daDanti*), all parasmaipadī. This takes the golden suite from 4428 to 4572 cells and juhotyādi to 24 of its 26 rows. It adds no rule. It widens two: 8.2.75 *daś ca* loses its gaṇa test, and 8.3.24 *naś cāpadāntasya jhali* admits juhotyādi beside rudhādi.

**Architecture:** Seven tasks.
- **Tasks 1 and 2** are the whole engine change, both in `tripadi.rs`. Neither widening fires on a curated cell until Task 3 lands the rows, so both are gated on **guard tests plus the 4428 priors staying byte-identical**.
- **Task 3** lands the four rows and their goldens, turning 144 cells green, and extends 3e's corpus-wide 7.3.87 test, which the new rows would otherwise fail.
- **Task 4** adds the trace pins and the two corpus-wide fires-only tests.
- **Tasks 5–7** are the audit and doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.98.1 pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants | audit`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`.

**Spec:** `docs/superpowers/specs/2026-10-01-juhotyadi-gana-3f-design.md`

**Workspace:** the branch `juhotyadi-3f` already exists. It holds the spec and this plan and is checked out at `/workspace/.worktrees/juhotyadi-3f`. Every path below is relative to that directory.

## Global Constraints

- **The 4428 pre-existing cells must stay byte-identical, traces included.** Regenerate no golden and change no pinned trace.
- **No new rule and no new vikalpa.** The engine stays at eleven optional rules; `tinanta_rule_order_is_pinned` (124 ids) and `exactly_the_pinned_vikalpa_rules_are_optional` must not change.
- **8.2.75 keeps no gaṇa test. 8.3.24 keeps one** (`Rudhadi || Juhotyadi`): it stands in for *apadāntasya* and keeps the rule off 7.1.3's `n`. Dropping it fails four trace pins (*bhavanti*, *yanti*, *āpnuvanti*, *hinvanti*). Never drop it.
- **8.2.73 and 8.2.74 stay rudhādi-only.** Widening them is slice 3f2's (√bhas).
- **No unfalsifiable guard clauses.** A clause no cell and no guard test can make false is a mutation survivor. Witness it, delete it, or kill it with a direct guard test on a hand-built `Prakriya`.
- **Goldens are transcribed from this plan** (vidyut's output at `8da2f90b`, generated 2026-10-01 by `/tmp/vidyut-full/vidyut-prakriya/examples/juhotyadi_3f_probe.rs`). **ALTERNATES keys are this engine's** (its log ∩ `VIKALPA_RULES`, computed from HEAD's traces in the same probe). Never invent one. **Do not edit a golden to match the engine.**
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit.
- `mise run test` takes a few seconds. Run it in the **foreground** with a timeout of 600000 ms. Never background it and end a turn.
- `mise run test -- -p X` does not scope. Scope unit tests with `mise exec -- cargo test -p <crate> <filter>`.
- Rule ids and SLP1 names, verbatim from the code: `8.2.75 daSca`, `8.3.24 naScApadAntasya Jali`.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **A 7.1.3 `n` before a jhal in a third gaṇa** (bhvādi *bhavanti*: `Bav` + `a` + `nti`). 8.3.24 must decline, because that `n` is what the gaṇa test protects. → Task 2, `nas_capadantasya_declines_off_its_two_ganas_and_before_a_non_jhal`.
2. **Tip rather than sip on a pada-final `d`** (√kit laṅ prathama eka, *aciked*). 8.2.75 must not fork it into a visarga. → Task 1, `das_ca_declines_off_sip_off_d_and_before_a_live_ending`; Task 4, `aciked_trace_has_no_ru`.
3. **`is_sip()` true but the ending still live** (vidhiliṅ madhyama eka `yAd`-shaped, *cikityAH*). `is_sip` is lakāra-blind, so only `dhatu_is_pada_final` keeps 8.2.75 off it. → Task 1, same decline test (the `yAd` case).
4. **√dhan's `n` before a non-jhal** (`mi`, `vaH`, `yAt`: *daDanmi*, *daDanvaH*, *daDanyAt*). 8.3.24 must decline. → Task 2, the non-jhal half of the decline test.
5. **`check()` on the new forms.** *acikeH*, *daDaMhi* and *tutorti* must resolve to the 3f rows only. → Task 3, `kit_tur_dhish_dhan_analyse_their_reduplicated_forms`.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | Task 1: 8.2.75's guard, comment, `dhatu_is_pada_final`'s comment, unit tests. Task 2: 8.3.24's guard, comment, unit tests |
| `crates/panini-data/src/lib.rs` | Task 3: the four rows, counts, the gaṇa-row test |
| `crates/panini/tests/paradigm/data/juhotyadi.rs` | Task 3: 16 `PARADIGM` blocks, 25 `ALTERNATES` rows |
| `crates/panini/tests/paradigm/main.rs` | Task 3: totals, buckets, key census, the `check()` test. Task 5: audit prose |
| `crates/panini/tests/trace/juhotyadi.rs` | Task 3: the 7.3.87 corpus test's allowed rows. Task 4: five pins, two corpus-wide tests |
| `crates/panini-prakriya/src/tinanta/{guna,abhyasa,anga}.rs` | Task 5: stale "3f" comments |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, the 3a spec | Task 5 |
| `AGENTS.md`, maybe `mise.toml` | Task 6 |

---

## Task 1: 8.2.75 *daś ca* loses its gaṇa test

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (the 8.2.75 rule at about line 609; `dhatu_is_pada_final` at line 48; `mod tests`)

**Interfaces:**
- Consumes: `crate::context::Context::new(Lakara, Pada, Purusha, Vacana)`; `crate::tinanta::terms::with_slots(Vec<Term>) -> Vec<Term>` (prepends the empty AGAMA and ABHYASA slots, so the first term given lands at `ANGA`).
- Produces: the test helper `sip_prakriya(stem: &str, ending: &str, purusha: Purusha) -> Prakriya` in `tripadi.rs`'s `mod tests`, used only in this task.

- [ ] **Step 1: Write the failing tests**

In `tripadi.rs`'s `mod tests`, change `use panini_data::{Lakara, Purusha, Vacana, dhatus};` to `use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};` and add `use crate::context::Context;` beside the other `crate::` imports. At the end of `mod tests` (after `shcutva_fires_on_stu_before_shcu_and_declines_after_sha`), add:

```rust
    // --- 8.2.75 daś ca: gaṇa-free since slice 3f -------------------------

    /// A laṅ prakriyā at the tripādī with the whole stem in ANGA and no gaṇa
    /// tag: 8.2.75 reads none. `ending` "" is the 8.2.23-emptied sip.
    fn sip_prakriya(stem: &str, ending: &str, purusha: Purusha) -> Prakriya {
        Prakriya {
            terms: with_slots(vec![Term::new(stem), Term::new(""), Term::new(ending)]),
            ctx: Context::new(Lakara::Lan, Pada::Parasmaipada, purusha, Vacana::Eka),
            ..Default::default()
        }
    }

    #[test]
    fn das_ca_fires_on_any_ganas_pada_final_d_before_sip() {
        // √kit laṅ madhyama eka after 8.2.39: aciked → aciker, which 8.3.15
        // finishes to acikeH. No Rudhadi tag on the aṅga.
        let rule = rules().find(|r| r.id == "8.2.75").unwrap();
        let mut p = sip_prakriya("aciked", "", Purusha::Madhyama);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "aciker");
        assert_eq!(p.log.last().unwrap().sutra, "8.2.75");
    }

    #[test]
    fn das_ca_declines_off_sip_off_d_and_before_a_live_ending() {
        let rule = rules().find(|r| r.id == "8.2.75").unwrap();
        for (stem, ending, purusha, why) in [
            // tip: laṅ prathama eka keeps aciked.
            ("aciked", "", Purusha::Prathama, "tip"),
            // a pada-final `t`, not `d`.
            ("aciket", "", Purusha::Madhyama, "t-final"),
            // is_sip() is lakāra-blind, so a vidhiliṅ madhyama eka shape
            // passes it; only dhatu_is_pada_final keeps the rule off.
            ("cikit", "yAd", Purusha::Madhyama, "live ending"),
        ] {
            let mut p = sip_prakriya(stem, ending, purusha);
            assert!(!(rule.apply)(&mut p), "{why}");
            assert!(p.log.is_empty(), "{why}");
        }
    }
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya das_ca 2>&1 | tail -20`
Expected: `das_ca_fires_on_any_ganas_pada_final_d_before_sip` FAILS (the untagged aṅga hits the Rudhadi test). The decline test passes; it exists for the mutation gate.

- [ ] **Step 3: Drop the gaṇa test and rewrite the comment**

In the 8.2.75 rule, change

```rust
            if !p.terms[ANGA].has(Tag::Rudhadi) || !p.ctx.is_sip() {
```

to

```rust
            if !p.ctx.is_sip() {
```

The first of the two identical lines belongs to 8.2.74 (about line 579). Edit only the one inside `id: "8.2.75"`.

Replace the rule's leading comment, from `// 8.2.75 daś ca (vikalpa): and a final \`d\` likewise becomes ru before` down to and including `// witnesses.` just above `Rule {`, with:

```rust
    // 8.2.75 daś ca (vikalpa): and a final `d` likewise becomes ru before
    // sip. akfRad + s → akfRaH; aciked + s → acikeH (juhotyādi √kit, slice
    // 3f). The counterpart of 8.2.74 for a stem whose final is already a
    // stop, voiced by 8.2.39 just above.
    //
    // NO GAṆA TEST. Its guard is the sūtra's own: sip, the dhātu pada-final
    // (8.2.23 has eaten the ending), and a final `d`. Through slice 3e it
    // also required Tag::Rudhadi; slice 3f dropped that, and the whole suite
    // passed unchanged without it, so no curated root outside rudhādi and
    // √kit reaches this rule. `das_ca_is_credited_only_on_rudhadi_and_kit`
    // in `panini`'s trace suite holds that as a fact.
    //
    // ORDERED ABOVE 8.2.73 (7b Task 8), against sūtra order, for the same
    // structural reason as 8.2.74 just above: this rule needs to see the
    // dhātu's OWN `d`, not one 8.2.73 manufactured from an `s`. At this
    // position 8.2.73 has not run yet, so the guard rests on phonology:
    // √hiṃs presents `ahinas`, which fails `ends_with('d')` and falls
    // through to 8.2.73 unchanged; √kṛt presents `akfRad` and √kit
    // `aciked`, 8.2.39 having voiced their `t`, and this rule fires on them
    // directly. `shnams_ru_fires_on_the_dhatus_own_final` and the 7a laṅ
    // cell tests in `super::derivation_tests` are the witnesses.
```

In `dhatu_is_pada_final`'s comment (line 49), replace

```rust
    // Defensive rather than a bare `p.terms[ENDING..]`: every call site
    // guards on `Tag::Rudhadi` first, so a hand-built two-term `Prakriya`
    // never actually reaches here today, but this helper is file-scoped and
    // a future caller might not carry that guard. `p.terms.get(ENDING)` at
```

with

```rust
    // Defensive rather than a bare `p.terms[ENDING..]`: 8.2.73 and 8.2.74
    // guard on `Tag::Rudhadi` first, but 8.2.75 has had no gaṇa test since
    // slice 3f, so a hand-built prakriyā with any layout can reach here.
    // `p.terms.get(ENDING)` at
```

Leave the rest of that comment as it is.

- [ ] **Step 4: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS.

- [ ] **Step 5: Run the full suite (priors byte-identical)**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms)
Expected: PASS at 4428 cells, no golden or trace changed. The spec's probe ran exactly this change green. If a prior cell gains an 8.2.75 fork, stop and report it.

- [ ] **Step 6: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/tripadi.rs
git commit -m "feat(tripadi): 8.2.75 daś ca drops its rudhādi gaṇa test"
```

---

## Task 2: 8.3.24 *naś cāpadāntasya jhali* admits juhotyādi

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (the 8.3.24 rule at about line 777; `mod tests`)

**Interfaces:**
- Consumes: `Term::add(&mut self, Tag)`, `Tag::Juhotyadi`, `Tag::Rudhadi`. `Tag` reaches `mod tests` through `use super::*` (the file imports `crate::term::Tag`); if the compiler disagrees, add `use crate::term::Tag;` to the test module.
- Produces: nothing later tasks call.

- [ ] **Step 1: Write the failing tests**

At the end of `tripadi.rs`'s `mod tests`, add:

```rust
    // --- 8.3.24 naś cāpadāntasya jhali: rudhādi and juhotyādi ------------

    /// √dhan at the tripādī: abhyāsa `da`, aṅga `Dan`, an empty śap (ślu),
    /// and `ending`. The aṅga carries Tag::Juhotyadi.
    fn dhan_prakriya(ending: &str) -> Prakriya {
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Dan"), Term::new(""), Term::new(ending)]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "da".into();
        p.terms[ANGA].add(Tag::Juhotyadi);
        p
    }

    #[test]
    fn nas_capadantasya_fires_on_a_juhotyadi_n_before_a_jhal() {
        // daDaMsi, daDaMhi (`h` is a jhal), and daDaMtaH, which 8.4.58
        // takes back to daDantaH.
        let rule = rules().find(|r| r.id == "8.3.24").unwrap();
        for (ending, want) in [("si", "daDaMsi"), ("hi", "daDaMhi"), ("taH", "daDaMtaH")] {
            let mut p = dhan_prakriya(ending);
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.text(), want);
            assert_eq!(p.log.last().unwrap().sutra, "8.3.24");
        }
    }

    #[test]
    fn nas_capadantasya_declines_off_its_two_ganas_and_before_a_non_jhal() {
        let rule = rules().find(|r| r.id == "8.3.24").unwrap();
        // bhvādi bhavanti: 7.1.3's `n` before `t`. The gaṇa test is what
        // keeps the rule off it; *apadāntasya* is not modelled.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Bav"), Term::new("a"), Term::new("nti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p), "bhavanti");
        assert!(p.log.is_empty());
        // √dhan before m, v, y: none is a jhal (daDanmi, daDanvaH, daDanyAt).
        for ending in ["mi", "vaH", "yAt"] {
            let mut p = dhan_prakriya(ending);
            assert!(!(rule.apply)(&mut p), "{ending}");
            assert!(p.log.is_empty(), "{ending}");
        }
    }
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya nas_capadantasya 2>&1 | tail -20`
Expected: `nas_capadantasya_fires_on_a_juhotyadi_n_before_a_jhal` FAILS (the guard admits only rudhādi). The decline test passes.

- [ ] **Step 3: Widen the guard and rewrite the comment**

In the 8.3.24 rule, change

```rust
            if !p.terms[ANGA].has(Tag::Rudhadi) {
                return false;
            }
            let w = word_chars(p);
```

to

```rust
            let anga = &p.terms[ANGA];
            if !anga.has(Tag::Rudhadi) && !anga.has(Tag::Juhotyadi) {
                return false;
            }
            let w = word_chars(p);
```

Replace the first paragraph of its comment:

```rust
    // 8.3.24 naścāpadāntasya jhali: a non-pada-final `n` becomes an
    // anusvāra before a jhal. In this suite that `n` is always śnam's, and
    // the jhal is whatever the weak stem's tail or the ending supplies.
```

with:

```rust
    // 8.3.24 naścāpadāntasya jhali: a non-pada-final `n` becomes an
    // anusvāra before a jhal. In this suite that `n` is śnam's (rudhādi) or,
    // in juhotyādi, the root's own (√dhan, slice 3f: daDaMsi, daDaMhi), and
    // the jhal is whatever the stem's tail or the ending supplies.
```

Replace the NARROW GUARD paragraph:

```rust
    // NARROW GUARD: rudhādi only. The `n` of 7.1.3 jho'ntaH (aBavan,
    // kfntan) is pada-final and out of scope by the sūtra's own
    // `apadāntasya`; guarding on the gaṇa keeps this rule away from it
    // without needing a pada-boundary notion the engine does not have.
```

with:

```rust
    // NARROW GUARD: rudhādi and juhotyādi only. The `n` of 7.1.3 jho'ntaH
    // (aBavan, kfntan) is pada-final and out of scope by the sūtra's own
    // `apadāntasya`; guarding on the gaṇa keeps this rule away from it
    // without needing a pada-boundary notion the engine does not have.
    // Dropping the gaṇa test entirely credits an 8.3.24 → 8.4.58 pair on
    // every 7.1.3 `n` (bhavanti, yanti, Apnuvanti, hinvanti: four trace pins
    // fail). Juhotyādi is safe under it: its aṅga is abhyasta, so 7.1.4 ad
    // abhyastāt takes the `J` (daDati) and laṅ takes jus (3.4.109), and no
    // juhotyādi `n` is 7.1.3's. Where √dhan's `n` meets `t`/`T`, 8.4.58
    // turns the anusvāra straight back (daDantaH), as in vidyut's trace.
```

Leave the paragraphs about 8.4.58 and the search inside the rule untouched.

- [ ] **Step 4: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS.

- [ ] **Step 5: Run the full suite (priors byte-identical)**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms)
Expected: PASS at 4428 cells. No curated juhotyādi row has an `n` before a jhal, so no prior trace changes. If one does, stop and report it.

- [ ] **Step 6: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/tripadi.rs
git commit -m "feat(tripadi): 8.3.24 naś cāpadāntasya jhali admits juhotyādi beside rudhādi"
```

---

## Task 3: The four rows and their paradigm goldens

This task turns the 144 new cells green.

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini/tests/paradigm/data/juhotyadi.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`
- Modify: `crates/panini/tests/trace/juhotyadi.rs` (the 7.3.87 corpus test only)

**Interfaces:**
- Consumes: Tasks 1–2.
- Produces: `dhatus()` rows `03.0021` (`kit`), `03.0022` (`tur`), `03.0023` (`Diz`) and `03.0024` (`Dan`), each `Gana::Juhotyadi` and `PadaAssignment::Parasmaipada`. Task 4 looks them up by number.

- [ ] **Step 1: Add the `Dhatu` rows**

`DHATUS` is ordered by number. Insert after the `03.0020` (`ki`) row and before `03.0026` (`gA`). Each artha is verbatim from `data/dhatupatha.tsv`:

```rust
    Dhatu {
        // 03.0021 `kita~` jYAne (√kita). Parasmaipadī by 1.3.78. 7.4.60 trims
        // the copy to `ki`, 7.4.62 makes it `ci`: ciketti. Laṅ madhyama eka
        // forks three ways, 8.2.75 daś ca taking the 8.2.39 `d` to ru
        // (acikeH beside aciked / aciket). 7.3.87 keeps the root's `i` before
        // the vowel-initial pit endings: cikitAni. Slice 3f.
        dhatupatha: "03.0021",
        code: "kit",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "jYAne",
    },
    Dhatu {
        // 03.0022 `tura~` tvaraRe (√tura). Parasmaipadī by 1.3.78. 7.3.86's
        // guṇa on the pit cells (tutorti); 8.2.77 hali ca lengthens the
        // upadhā before a consonant on the others (tutUrtaH, tutUrhi). Slice
        // 3f.
        dhatupatha: "03.0022",
        code: "tur",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "tvaraRe",
    },
    Dhatu {
        // 03.0023 `Diza~` Sabde (√dhiṣa). Parasmaipadī by 1.3.78. 8.4.54 makes
        // the abhyāsa `di`; the root's `z` reaches the existing tripādī rules
        // as √viṣ's does: 8.4.41 (diDezwi), 8.2.41 then 8.3.59 (diDekzi),
        // 8.4.41 then 8.4.53 (diDiqQi), 8.2.39 (adiDeq). Slice 3f.
        dhatupatha: "03.0023",
        code: "Diz",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "Sabde",
    },
    Dhatu {
        // 03.0024 `Dana~` DAnye (√dhana). Parasmaipadī by 1.3.78. 8.4.54 makes
        // the abhyāsa `da`. The root's `n` takes 8.3.24 before a jhal:
        // daDaMsi, daDaMhi; before `t`/`T` 8.4.58 restores it (daDantaH).
        // Slice 3f.
        dhatupatha: "03.0024",
        code: "Dan",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "DAnye",
    },
```

Counts in the same file:
- `Dhatu`'s `pada` doc: `re-derives 96 of these 97` → `re-derives 100 of these 101`; `The test covers the 97 roots curated here` → `The test covers the 101 roots curated here`.
- `pada_from_upadesha`'s doc: `66 of the 97 curated roots carry a \`\\\` at all, and 45` → `66 of the 101 curated roots carry a \`\\\` at all, and 45`. None of the four new upadeśas carries a `\`.
- `curated_roots_have_expected_ganas_and_padas`: `assert_eq!(dhatus().len(), 97);` → `101`.

- [ ] **Step 2: Extend the gaṇa-row test**

Rename `juhotyadi_rows_are_the_twenty_curated_roots` to `juhotyadi_rows_are_the_twenty_four_curated_roots`. `grep -rn "juhotyadi_rows_are_the_twenty_curated" crates AGENTS.md docs` finds the other references (a comment near `lib.rs:1944`, `AGENTS.md:812`); update each.

In its comment, replace:

```rust
        // by 1.3.78, the gaṇa's one vowel-initial ṛ-root. Slice 3e adds √ṇij,
        // √vij and √viṣ (03.0012–03.0014), ubhayapadī by 1.3.72, the three
        // roots 7.4.75 names. The gaṇa is PARTIAL at 20 of its 26 dhātupāṭha
        // rows; slice 3f closes it.
```

with:

```rust
        // by 1.3.78, the gaṇa's one vowel-initial ṛ-root. Slice 3e adds √ṇij,
        // √vij and √viṣ (03.0012–03.0014), ubhayapadī by 1.3.72, the three
        // roots 7.4.75 names. Slice 3f adds √kit, √tur, √dhiṣ and √dhan
        // (03.0021–03.0024), parasmaipadī by 1.3.78. The gaṇa is PARTIAL at
        // 24 of its 26 dhātupāṭha rows; slice 3f2 (√bhas, √jan) closes it.
```

If the wrapping differs, match on the sentence text. In its expected vector, insert after the `03.0020` line:

```rust
                ("03.0021", "kit", PadaAssignment::Parasmaipada),
                ("03.0022", "tur", PadaAssignment::Parasmaipada),
                ("03.0023", "Diz", PadaAssignment::Parasmaipada),
                ("03.0024", "Dan", PadaAssignment::Parasmaipada),
```

- [ ] **Step 3: Run the data tests**

Run: `mise exec -- cargo test -p panini-data 2>&1 | tail -20`
Expected: PASS, including `curated_pada_agrees_with_upadesha_markers` and `dhatupatha_numbers_resolve_upstream`.

- [ ] **Step 4: Add the `PARADIGM` blocks**

**Append** to the end of `PARADIGM` in `crates/panini/tests/paradigm/data/juhotyadi.rs`, just before its closing `];`. Blocks are ordered by slice, not by number. Index 0 of each multi-form cell is the declined derivation. `mise run fmt` re-wraps.

```rust
    ("03.0021", "laT", Pada::Parasmaipada, ["ciketti", "cikittaH", "cikitati", "ciketsi", "cikitTaH", "cikitTa", "ciketmi", "cikitvaH", "cikitmaH"]),
    ("03.0021", "laN", Pada::Parasmaipada, ["aciked", "acikittAm", "acikituH", "aciked", "acikittam", "acikitta", "acikitam", "acikitva", "acikitma"]),
    ("03.0021", "loT", Pada::Parasmaipada, ["cikettu", "cikittAm", "cikitatu", "cikidDi", "cikittam", "cikitta", "cikitAni", "cikitAva", "cikitAma"]),
    ("03.0021", "viDiliN", Pada::Parasmaipada, ["cikityAd", "cikityAtAm", "cikityuH", "cikityAH", "cikityAtam", "cikityAta", "cikityAm", "cikityAva", "cikityAma"]),
    ("03.0022", "laT", Pada::Parasmaipada, ["tutorti", "tutUrtaH", "tuturati", "tutorzi", "tutUrTaH", "tutUrTa", "tutormi", "tutUrvaH", "tutUrmaH"]),
    ("03.0022", "laN", Pada::Parasmaipada, ["atutoH", "atutUrtAm", "atuturuH", "atutoH", "atutUrtam", "atutUrta", "atuturam", "atutUrva", "atutUrma"]),
    ("03.0022", "loT", Pada::Parasmaipada, ["tutortu", "tutUrtAm", "tuturatu", "tutUrhi", "tutUrtam", "tutUrta", "tuturARi", "tuturAva", "tuturAma"]),
    ("03.0022", "viDiliN", Pada::Parasmaipada, ["tutUryAd", "tutUryAtAm", "tutUryuH", "tutUryAH", "tutUryAtam", "tutUryAta", "tutUryAm", "tutUryAva", "tutUryAma"]),
    ("03.0023", "laT", Pada::Parasmaipada, ["diDezwi", "diDizwaH", "diDizati", "diDekzi", "diDizWaH", "diDizWa", "diDezmi", "diDizvaH", "diDizmaH"]),
    ("03.0023", "laN", Pada::Parasmaipada, ["adiDeq", "adiDizwAm", "adiDizuH", "adiDeq", "adiDizwam", "adiDizwa", "adiDizam", "adiDizva", "adiDizma"]),
    ("03.0023", "loT", Pada::Parasmaipada, ["diDezwu", "diDizwAm", "diDizatu", "diDiqQi", "diDizwam", "diDizwa", "diDizARi", "diDizAva", "diDizAma"]),
    ("03.0023", "viDiliN", Pada::Parasmaipada, ["diDizyAd", "diDizyAtAm", "diDizyuH", "diDizyAH", "diDizyAtam", "diDizyAta", "diDizyAm", "diDizyAva", "diDizyAma"]),
    ("03.0024", "laT", Pada::Parasmaipada, ["daDanti", "daDantaH", "daDanati", "daDaMsi", "daDanTaH", "daDanTa", "daDanmi", "daDanvaH", "daDanmaH"]),
    ("03.0024", "laN", Pada::Parasmaipada, ["adaDan", "adaDantAm", "adaDanuH", "adaDan", "adaDantam", "adaDanta", "adaDanam", "adaDanva", "adaDanma"]),
    ("03.0024", "loT", Pada::Parasmaipada, ["daDantu", "daDantAm", "daDanatu", "daDaMhi", "daDantam", "daDanta", "daDanAni", "daDanAva", "daDanAma"]),
    ("03.0024", "viDiliN", Pada::Parasmaipada, ["daDanyAd", "daDanyAtAm", "daDanyuH", "daDanyAH", "daDanyAtam", "daDanyAta", "daDanyAm", "daDanyAva", "daDanyAma"]),
```

- [ ] **Step 5: Add the `ALTERNATES` rows**

**Append** to the end of `ALTERNATES`, just before its closing `];`. Each row is (root, lakāra, pada, 0-based cell index in the order P.E P.D P.B M.E M.D M.B U.E U.D U.B, form, key). The key is the vikalpa-listed ids on the form's branch, in log order. The laṅ keys carry 7.3.86 because the engine credits the root's (mandatory) guṇa under an id that is also in `VIKALPA_RULES`, exactly as 3e's laṅ rows do.

```rust
    ("03.0021", "laN", Pada::Parasmaipada, 0, "aciket", "7.3.86+8.4.56"),
    ("03.0021", "laN", Pada::Parasmaipada, 3, "aciket", "7.3.86+8.4.56"),
    ("03.0021", "laN", Pada::Parasmaipada, 3, "acikeH", "7.3.86+8.2.75"),
    ("03.0021", "loT", Pada::Parasmaipada, 0, "cikittAd", "7.1.35"),
    ("03.0021", "loT", Pada::Parasmaipada, 0, "cikittAt", "7.1.35+8.4.56"),
    ("03.0021", "loT", Pada::Parasmaipada, 3, "cikittAd", "7.1.35"),
    ("03.0021", "loT", Pada::Parasmaipada, 3, "cikittAt", "7.1.35+8.4.56"),
    ("03.0021", "viDiliN", Pada::Parasmaipada, 0, "cikityAt", "8.4.56"),
    ("03.0022", "loT", Pada::Parasmaipada, 0, "tutUrtAd", "7.1.35"),
    ("03.0022", "loT", Pada::Parasmaipada, 0, "tutUrtAt", "7.1.35+8.4.56"),
    ("03.0022", "loT", Pada::Parasmaipada, 3, "tutUrtAd", "7.1.35"),
    ("03.0022", "loT", Pada::Parasmaipada, 3, "tutUrtAt", "7.1.35+8.4.56"),
    ("03.0022", "viDiliN", Pada::Parasmaipada, 0, "tutUryAt", "8.4.56"),
    ("03.0023", "laN", Pada::Parasmaipada, 0, "adiDew", "7.3.86+8.4.56"),
    ("03.0023", "laN", Pada::Parasmaipada, 3, "adiDew", "7.3.86+8.4.56"),
    ("03.0023", "loT", Pada::Parasmaipada, 0, "diDizwAd", "7.1.35"),
    ("03.0023", "loT", Pada::Parasmaipada, 0, "diDizwAt", "7.1.35+8.4.56"),
    ("03.0023", "loT", Pada::Parasmaipada, 3, "diDizwAd", "7.1.35"),
    ("03.0023", "loT", Pada::Parasmaipada, 3, "diDizwAt", "7.1.35+8.4.56"),
    ("03.0023", "viDiliN", Pada::Parasmaipada, 0, "diDizyAt", "8.4.56"),
    ("03.0024", "loT", Pada::Parasmaipada, 0, "daDantAd", "7.1.35"),
    ("03.0024", "loT", Pada::Parasmaipada, 0, "daDantAt", "7.1.35+8.4.56"),
    ("03.0024", "loT", Pada::Parasmaipada, 3, "daDantAd", "7.1.35"),
    ("03.0024", "loT", Pada::Parasmaipada, 3, "daDantAt", "7.1.35+8.4.56"),
    ("03.0024", "viDiliN", Pada::Parasmaipada, 0, "daDanyAt", "8.4.56"),
```

That is 25 rows over 16 cells, so 144 + 25 = 169 forms, vidyut's count. √tur's laṅ eka ends in visarga (`atutoH`) and √dhan's in `n` (`adaDan`), so neither forks there.

- [ ] **Step 6: Update `paradigm/main.rs`**

In `every_alternate_names_the_vikalpa_rules_that_produced_it`'s doc comment:
- `otherwise 1058 bare strings` → `otherwise 1083 bare strings`.
- `Six of the 13` / `` `7.3.86+8.4.56` keys are the mandatory firing (3e's laṅ eka cells).`` → `Ten of the 17 \`7.3.86+8.4.56\` keys are the mandatory firing (3e's laṅ eka cells and 3f's √kit and √dhiṣ ones), and so is the 7.3.86 of the one \`7.3.86+8.2.75\` key (√kit's \`acikeH\`).`

In `derivation_set_shape_matches_the_audited_numbers`:
- `assert_eq!(total_cells, 4428, "492 root×lakāra blocks × 9 cells each")` → `4572`, `"508 root×lakāra blocks × 9 cells each"`.
- `ones` 3652 → `3780`; `twos` 587 → `594`.
- `threes` 143 → `152`. In its message, change the final `and — new in slice 3e — √ṇij's, √vij's and \` / `√viṣ's, the same way"` to `and — new in slice 3e — √ṇij's, √vij's and \` / `√viṣ's, the same way; and — new in slice 3f — √kit's, √tur's, √dhiṣ's and √dhan's, \` / `the same way"`. Keep the string's `\` line-continuation style.
- `fours`, `fives`, `sixes`, `sevens` unchanged.
- `ALTERNATES.len()` 1058 → `1083`.
- `key_count("8.4.56")` 156 → `160`; `key_count("7.1.35")` 144 → `152`; `key_count("7.1.35+8.4.56")` 144 → `152`; `key_count("7.3.86+8.4.56")` 13 → `17`.
- After the `7.3.86+8.4.56` line, add: `assert_eq!(key_count("7.3.86+8.2.75"), 1, "7.3.86+8.2.75 alternates");`

Check: 3780 + 594 + 152 + 18 + 10 + 17 + 1 = 4572 cells, and 3780 + 1188 + 456 + 72 + 50 + 102 + 7 = 5655 forms = 4572 + 1083. The key census stays exhaustive: 1058 + 4 + 8 + 8 + 4 + 1 = 1083.

In the doc comment above that test:
- `4428 cells total (492 root×lakāra blocks × 9), of which 3652 hold exactly` / `one form, 587 hold two, 143 hold three (` → `4572 cells total (508 root×lakāra blocks × 9), of which 3780 hold exactly one form, 594 hold two, 152 hold three (`.
- In that three-form parenthesis, after `and √ṇij's, √vij's and √viṣ's, new in slice 3e`, add `, and √kit's, √tur's, √dhiṣ's and √dhan's, new in slice 3f`.
- `itself has 1058 rows, keyed 156 \`8.4.56\`, 144 \`7.1.35\`, 144 \`7.1.35+8.4.56\`,` → `itself has 1083 rows, keyed 160 \`8.4.56\`, 152 \`7.1.35\`, 152 \`7.1.35+8.4.56\`,`. `13 \`7.3.86+8.4.56\` (six of them name the MANDATORY` → `17 \`7.3.86+8.4.56\` (ten of them name the MANDATORY`, and after `slice 3e's` / `laṅ prathama and madhyama eka cells, whose root guṇa 7.3.86 credits)` change that parenthesis's end to `slice 3e's laṅ prathama and madhyama eka cells and slice 3f's √kit and √dhiṣ ones, whose root guṇa 7.3.86 credits), 1 \`7.3.86+8.2.75\` (√kit's acikeH, the same mandatory 7.3.86),`.
- After the slice-3e paragraph (ending `the four loṭ tātaṅ rows per root in \`7.1.35\` and \`7.1.35+8.4.56\`. The gaṇa` / `is PARTIAL at 20 of its 26 rows.`), add:

```rust
///
/// Slice 3f curates √kit (`03.0021`), √tur (`03.0022`), √dhiṣ (`03.0023`) and
/// √dhan (`03.0024`), parasmaipadī by 1.3.78, bringing the gaṇa to twenty-four
/// of its twenty-six rows. It adds no rule and no vikalpa: 8.2.75 *daś ca*
/// loses its rudhādi gaṇa test and 8.3.24 admits juhotyādi. Every root forks
/// its vidhiliṅ prathama eka on 8.4.56 and its two loṭ tātaṅ cells three
/// ways. √kit and √dhiṣ also fork laṅ prathama and madhyama eka on 8.4.56
/// (`aciked`/`aciket`, `adiDeq`/`adiDew`), keyed `7.3.86+8.4.56` like 3e's,
/// and √kit's laṅ madhyama eka forks a third way on 8.2.75 (`acikeH`, keyed
/// `7.3.86+8.2.75`, the only such key). √tur's laṅ eka ends in visarga and
/// √dhan's in `n`, so neither forks there. Twenty-five new rows. The gaṇa is
/// PARTIAL at 24 of its 26 rows.
```

After `nij_roots_analyse_their_reduplicated_forms`, add the Review Focus test:

```rust
/// Slice 3f's four rows through `check`. Each code is unique among the
/// curated roots, so every analysis must name the 3f row. *acikeH* is 8.2.75's
/// first non-rudhādi form and *daDaMhi* 8.3.24's first juhotyādi one.
#[test]
fn kit_tur_dhish_dhan_analyse_their_reduplicated_forms() {
    let engine = Panini::new();
    for (form, root, sutra) in [
        ("acikeH", "kit", "8.2.75"),
        ("ciketti", "kit", "7.4.62"),
        ("tutorti", "tur", "7.3.86"),
        ("diDezwi", "Diz", "8.4.41"),
        ("daDaMhi", "Dan", "8.3.24"),
        ("daDaMsi", "Dan", "8.3.24"),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert!(!r.analyses.is_empty(), "{form}");
        for a in &r.analyses {
            assert_eq!(a.dhatu, root, "{form}");
            assert!(
                a.trace.iter().any(|s| s.sutra == sutra),
                "{form}: {:?}",
                a.trace
            );
        }
    }
}
```

- [ ] **Step 7: Extend 3e's corpus-wide 7.3.87 test**

HEAD credits 7.3.87 on all four new rows' laṅ and loṭ uttama cells (*acikitam*, *cikitAni*, *tuturARi*, *diDizARi*, and on √dhan's a-upadhā *daDanAni* as a no-op, as vidyut does). 3e's spec predicted this. In `crates/panini/tests/trace/juhotyadi.rs`:
- Rename `nabhyastasyaci_is_credited_only_on_the_nij_vij_vish_rows` → `nabhyastasyaci_is_credited_only_on_the_3e_and_3f_rows`. `grep -rn nabhyastasyaci_is_credited_only crates` finds the comment in `juhavani_trace_has_no_nabhyastasyaci` that cites it; update that too.
- Replace its first comment lines:

```rust
    // Corpus-wide: every branch of every curated root x lakāra x pada x cell
    // whose log carries 7.3.87 belongs to √ṇij, √vij or √viṣ. A new rule that
    // credits itself on prior rows' traces (forms unchanged) fails here.
    // 3f extends the allowed list.
    const ALLOWED: [&str; 3] = ["03.0012", "03.0013", "03.0014"];
```

with:

```rust
    // Corpus-wide: every branch of every curated root x lakāra x pada x cell
    // whose log carries 7.3.87 belongs to √ṇij, √vij, √viṣ (3e) or √kit,
    // √tur, √dhiṣ, √dhan (3f; a credited no-op on √dhan's a-upadhā). A new
    // rule that credits itself on prior rows' traces (forms unchanged) fails
    // here. 3f2 extends the allowed list with √bhas and √jan.
    const ALLOWED: [&str; 7] = [
        "03.0012", "03.0013", "03.0014", "03.0021", "03.0022", "03.0023", "03.0024",
    ];
```

- [ ] **Step 8: Run the full suite**

Run: `mise run test 2>&1 | tail -40` (foreground, timeout 600000 ms)
Expected: PASS at 4572 cells, every 4428 prior unchanged.

If a 3f cell fails, read it against the spec before touching anything, using superpowers:systematic-debugging. **Do not edit a golden to match the engine.** Likely causes by symptom:
- `daDansi` / `daDanhi`: 8.3.24 declined. Check Task 2's guard.
- `daDaMtaH` (anusvāra surviving before `t`): 8.4.58 did not restore it. Stop and report the trace; the spec expects 8.4.58 to fire.
- `acikeH` missing from laṅ madhyama eka: 8.2.75 declined. Check Task 1.
- A key mismatch in `every_alternate_names_the_vikalpa_rules_that_produced_it`: read the branch's actual log. The keys above were computed from HEAD's traces; only correct a key if the form is right and the log really differs, and say so in the commit message.

- [ ] **Step 9: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-data/src/lib.rs crates/panini/tests/paradigm/data/juhotyadi.rs crates/panini/tests/paradigm/main.rs crates/panini/tests/trace/juhotyadi.rs AGENTS.md
git commit -m "feat(data): √kit, √tur, √dhiṣ, √dhan (03.0021–03.0024) — juhotyādi at twenty-four of twenty-six rows

4428 → 4572 cells, 5486 → 5655 forms, 1058 → 1083 ALTERNATES, 97 → 101 roots."
```

---

## Task 4: The trace pins and the fires-only tests

**Files:**
- Modify: `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes: `crate::helpers::{at, cell_trace}`. `cell_trace(number, lakara, pada, purusha, vacana) -> (String, Vec<String>)` returns branch 0's (the declined derivation's) text and trace. `at(&[String], &str) -> usize` panics if the sūtra is absent. `panini_prakriya::derive` and `panini_data::dhatus` are already imported.

- [ ] **Step 1: Add the pins and the two corpus-wide tests**

Change the file's `use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};` to `use panini_data::{Gana, Lakara, Pada, Purusha, Vacana, dhatus};`. At the end of the file, add:

```rust
/// The derivation of `form` in one cell, for the pins on a non-declined
/// branch (cell_trace reads branch 0 only).
fn branch_trace(
    number: &str,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
    form: &str,
) -> Vec<String> {
    let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
    let p = derive(d, lakara, Pada::Parasmaipada, purusha, vacana)
        .into_iter()
        .find(|p| !p.blocked && p.text() == form)
        .unwrap_or_else(|| panic!("no branch derives {form}"));
    p.log.iter().map(|s| s.sutra.clone()).collect()
}

#[test]
#[allow(non_snake_case)]
fn acikeH_trace_is_jashtva_then_das_ca() {
    // kit P laN M.E. 7.3.86's guṇa, 8.2.23 eats the sip, 8.2.39 voices the
    // `t`, then 8.2.75 takes the `d` to ru. 8.2.73 never runs on it: that
    // rule is still rudhādi-only and wants an `s`.
    let t = branch_trace("03.0021", Lakara::Lan, Purusha::Madhyama, Vacana::Eka, "acikeH");
    assert!(at(&t, "7.3.86") < at(&t, "8.2.39"), "got {t:?}");
    assert!(at(&t, "8.2.39") < at(&t, "8.2.75"), "got {t:?}");
    assert!(!t.contains(&"8.2.73".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.56".to_string()), "got {t:?}");
}

#[test]
fn aciked_trace_has_no_ru() {
    // kit P laN P.E. Tip, not sip: 8.2.75 declines, and the cell forks only
    // on 8.4.56.
    let (text, t) = cell_trace(
        "03.0021",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "aciked", "got {t:?}");
    assert!(!t.contains(&"8.2.75".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.2.73".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn daDaMsi_trace_keeps_its_anusvara() {
    // Dan P laT M.E. 8.3.24 before `s`; 8.4.58 needs a following yay and
    // `s` is not one, so the anusvāra stays.
    let (text, t) = cell_trace(
        "03.0024",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "daDaMsi", "got {t:?}");
    assert!(t.contains(&"8.3.24".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.58".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn daDaMhi_trace_reaches_nas_capadantasya_before_h() {
    // Dan P loT M.E. The prep spec's open question: `h` is a jhal, and
    // 8.3.24 reaches it. 6.4.101 does not fire: `n` is not a jhal.
    let (text, t) = cell_trace(
        "03.0024",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "daDaMhi", "got {t:?}");
    assert!(t.contains(&"8.3.24".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.101".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn daDantaH_trace_is_the_anusvara_round_trip() {
    // Dan P laT P.D. 8.3.24 makes the `n` an anusvāra before `t`, and 8.4.58
    // turns it straight back: vidyut's trace too.
    let (text, t) = cell_trace(
        "03.0024",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "daDantaH", "got {t:?}");
    assert!(at(&t, "8.3.24") < at(&t, "8.4.58"), "got {t:?}");
}

const ALL_CELLS: [(Purusha, Vacana); 9] = [
    (Purusha::Prathama, Vacana::Eka),
    (Purusha::Prathama, Vacana::Dvi),
    (Purusha::Prathama, Vacana::Bahu),
    (Purusha::Madhyama, Vacana::Eka),
    (Purusha::Madhyama, Vacana::Dvi),
    (Purusha::Madhyama, Vacana::Bahu),
    (Purusha::Uttama, Vacana::Eka),
    (Purusha::Uttama, Vacana::Dvi),
    (Purusha::Uttama, Vacana::Bahu),
];

/// Every (root, lakāra, pada, cell, branch) of the curated corpus whose log
/// carries `sutra`, as (dhatupatha, gaṇa).
fn credited(sutra: &str) -> Vec<(&'static str, Gana)> {
    let mut out = Vec::new();
    for d in dhatus() {
        for lakara in [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin] {
            for &pada in d.pada.padas() {
                for (purusha, vacana) in ALL_CELLS {
                    for p in derive(d, lakara, pada, purusha, vacana) {
                        if !p.blocked && p.log.iter().any(|s| s.sutra == sutra) {
                            out.push((d.dhatupatha, d.gana));
                        }
                    }
                }
            }
        }
    }
    out
}

#[test]
fn das_ca_is_credited_only_on_rudhadi_and_kit() {
    // 8.2.75 has no gaṇa test since slice 3f. Goldens ignore traces, so this
    // is what holds "no other root reaches it".
    let hits = credited("8.2.75");
    for (number, gana) in &hits {
        assert!(
            *gana == Gana::Rudhadi || *number == "03.0021",
            "8.2.75 credited on {number}"
        );
    }
    assert!(hits.iter().any(|(n, _)| *n == "03.0021"), "√kit no longer witnesses 8.2.75");
}

#[test]
fn nas_capadantasya_is_credited_only_on_rudhadi_and_dhan() {
    // 8.3.24 admits juhotyādi since slice 3f; only √dhan has an `n` before
    // a jhal there.
    let hits = credited("8.3.24");
    for (number, gana) in &hits {
        assert!(
            *gana == Gana::Rudhadi || *number == "03.0024",
            "8.3.24 credited on {number}"
        );
    }
    assert!(hits.iter().any(|(n, _)| *n == "03.0024"), "√dhan no longer witnesses 8.3.24");
}
```

`Gana` derives `PartialEq`, so `*gana == Gana::Rudhadi` compiles.

- [ ] **Step 2: Run the suite**

Run: `mise run test 2>&1 | tail -20` (foreground, timeout 600000 ms)
Expected: PASS. If a pin fails on ORDER, report the engine's order; do not "fix" it toward vidyut's. If a pin fails on an id this plan assumed, read the actual trace and correct the pin to the engine's real credit only when the form is right and the spec does not name that id. Say so in the commit message. If a fires-only test names a root outside the allowed set, stop and report it: that is a widening over-firing on a prior row.

- [ ] **Step 3: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini/tests/trace/juhotyadi.rs
git commit -m "test(trace): 3f pins — acikeH, aciked, daDaMsi, daDaMhi, daDantaH; 8.2.75 and 8.3.24 fire only on their rows"
```

---

## Task 5: Audit, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (audit prose), `crates/panini-prakriya/src/tinanta/{guna,abhyasa,anga}.rs` (comments), `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md` (one note)

**Interfaces:**
- Consumes: the finished engine and goldens (Tasks 1–4). Produces no symbols.

- [ ] **Step 1: Update the audit harness's asserted totals**

In `tools/audit/panini_full_audit.rs`:
- `for each of the 97 curated roots` → `101`
- `97 roots, 4428 cells, 5486 forms` → `101 roots, 4572 cells, 5655 forms`
- `492 root×pada×lakāra` / `blocks × 9 cells, plus 1058` → `508 … plus 1083`
- `the full 4428-cell table` → `4572-cell`
- `assert_eq!(roots_seen.len(), 97, …)` → `101`
- `assert_eq!(n_cells, 4428, "cells: 492 root×pada×lakāra blocks × 9")` → `4572`, `"cells: 508 root×pada×lakāra blocks × 9"`
- `assert_eq!(n_forms, 5486, "forms: 4428 cells + 1058 ALTERNATES rows")` → `5655`, `"forms: 4572 cells + 1083 ALTERNATES rows"`

The both-pada clause (`twenty-six` / `twenty-five ubhayapadī`) does not change: the four new roots are parasmaipadī.

In `tools/audit/README.md`, `(97 roots, 4428 cells, 5486 forms)` → `(101 roots, 4572 cells, 5655 forms)`.

- [ ] **Step 2: Repoint vidyut's dev-deps at THIS worktree and run the audit**

`/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes absolute dev-dep paths to `/workspace/crates`, the `main` checkout, which does not have this slice. Auditing without repointing checks the pre-slice engine and passes vacuously.

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
- Expected from the honest run: `AUDIT PASSED: 4572 cells, 5655 forms, zero differences.`
- Expected from the `entry` control: exit 1 with 36 √bhū cells.

If the honest run shows differences, stop and report, and edit nothing. After both runs, restore the dev-deps so later probes from `main` work:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"/workspace/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"/workspace/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
```

- [ ] **Step 3: Record the audit**

In `tools/audit/README.md`, immediately under `## Last recorded result`, add a new dated entry above the 3e one, using today's date:

```markdown
YYYY-MM-DD, juhotyādi 3f slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 4572
cells / 5655 forms / 101 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole juhotyādi 3f slice: √kit (`03.0021`), √tur
(`03.0022`), √dhiṣ (`03.0023`) and √dhan (`03.0024`), with no new rule —
8.2.75 *daś ca* without its rudhādi gaṇa test (√kit's *acikeH*) and 8.3.24
*naś cāpadāntasya jhali* admitting juhotyādi (√dhan's *daDaMsi*, *daDaMhi*).

Totals: 101 = 97 + 4; 4572 = 4428 + 144 (16 root×pada×lakāra blocks × 9); 5655
= 5486 + 144 + 25 new `ALTERNATES` rows (1058 → 1083), measured via the
harness's corpus block, not assumed.
```

In `crates/panini/tests/paradigm/main.rs`'s audit-chain doc comment, the sentence ends `…and juhotyādi 3e's re-ran it at the same commit over all 4428` / `cells / 5486 forms / 97 roots with zero differences, its \`entry\` negative` / `control verified failing (36 √bhū cells).` Before its final `.`, add `, and juhotyādi 3f's re-ran it at the same commit over all 4572 cells / 5655 forms / 101 roots with zero differences, its \`entry\` negative control verified failing (36 √bhū cells)`.

- [ ] **Step 4: README.md**

- `**partial** at 20 of its 26 dhātupāṭha rows` → `**partial** at 24 of its 26 dhātupāṭha rows`.
- After the sentence ending `the stored-form convention covering` / `6.1.65.`, insert: `Slice 3f added √kit (\`03.0021\`, *ciketti*), √tur (\`03.0022\`, *tutorti*), √dhiṣ (\`03.0023\`, *didheṣṭi*) and √dhan (\`03.0024\`, *dadhanti*), all parasmaipadī, with no new rule: 8.2.75 *daś ca* dropped its rudhādi gaṇa test (*acikeḥ*), and 8.3.24 *naś cāpadāntasya jhali* now admits juhotyādi beside rudhādi (*dadhaṁsi*, *dadhaṁhi*).`
- `curated 97-root set` → `curated 101-root set`.
- Multi-form paragraph: `776 of the 4428 cells hold more than one form: 587` / `hold two, 143 hold three (` → `792 of the 4572 cells hold more than one form: 594 hold two, 152 hold three (`. After `and — new in slice 3e — √ṇij's, √vij's and` / `√viṣ's` add `, and — new in slice 3f — √kit's, √tur's, √dhiṣ's and √dhan's`. Check: 594 + 152 + 18 + 10 + 17 + 1 = 792.
- The both-pada and pada-ambiguity paragraphs do not change.

Reflow edited paragraphs to the file's ~80-column wrap.

- [ ] **Step 5: docs/ARCHITECTURE.md**

- `**partial** at 20 of its 26 rows (… √ṇij, √vij, √viṣ; slice 3e)` → `**partial** at 24 of its 26 rows (… √ṇij, √vij, √viṣ; slice 3e; √kit, √tur, √dhiṣ, √dhan; slice 3f)`.
- The vikalpa paragraph: after `8.2.74 and 8.2.75 the *ru* alternation on` / `√hiṃs's own final before sip,` add ` (8.2.75 gaṇa-free since juhotyādi 3f, which gave it √kit's *acikeḥ*)`.
- The 7.1.35 / 8.4.56 paragraph (`grep -n "forking 144 cells" docs/ARCHITECTURE.md`):
  - `forking 144 cells (loṭ` → `forking 152 cells (loṭ`
  - `across the 72 roots with a parasmaipada column` → `76`
  - in the juhotyādi list `√gā, √pṝ, √pṛ, √ghṛ (\`Gf\`), √hṛ, √sṛ and √ṛ (all parasmaipada-only)` → `√gā, √pṝ, √pṛ, √ghṛ (\`Gf\`), √hṛ, √sṛ, √ṛ, √kit, √tur, √dhiṣ and √dhan (all parasmaipada-only)`
  - `72 + 25 = the 97` / `curated roots)` → `76 + 25 = the 101 curated roots)`
  - `forking 156 cells outright` → `160`
  - `those same 72 parasmaipada columns (134 of them` → `those same 76 parasmaipada columns (138 of them`. In that parenthesis, after the ṛ-roots clause ending `` `abiBaH`, `` / `` `EyaH`) ``, change `` `EyaH`) `` to `` `EyaH`; 3f's four contribute only their vidhiliṅ cell too: √tur's laṅ ends in visarga, √dhan's in `n`, and √kit's and √dhiṣ's laṅ forks key on 7.3.86 as well, see below) ``.
  - `Juhotyādi 3e's six laṅ prathama and` / `madhyama eka cells (\`aneneg ~ anenek\` etc.) also fork on 8.4.56` → `Juhotyādi 3e's six laṅ prathama and madhyama eka cells (\`aneneg ~ anenek\` etc.) and 3f's four (√kit's and √dhiṣ's: \`aciked ~ aciket\`, \`adiDeq ~ adiDew\`) also fork on 8.4.56`.
  - `forking a further 144 (the same` → `forking a further 152 (the same`

  Check: 138 + 22 = 160. If wording differs, keep its form and make the same substitution. What must be right: 4 new parasmaipada columns, 8 new `7.1.35` rows, 4 new plain `8.4.56` rows, 4 new `7.3.86+8.4.56` rows.
- Where the 8.3.24 discussion near line 382 (`grep -n "8.3.24" docs/ARCHITECTURE.md`) calls 8.3.24 rudhādi-only or says its `n` is always śnam's, add "or juhotyādi √dhan's". If it says neither, leave it.

- [ ] **Step 6: AGENTS.md**

- Rules of the codebase: `(\`crates/panini/tests/paradigm/\`, 4428 cells,` → `4572 cells`. In the juhotyādi clause, `at 17 after slice 3d2 curated √ṛ, and now at 20 of its 26 after slice 3e` / `curated √ṇij, √vij and √viṣ —` → `at 17 after slice 3d2 curated √ṛ, at 20 after slice 3e curated √ṇij, √vij and √viṣ, and now at 24 of its 26 after slice 3f curated √kit, √tur, √dhiṣ and √dhan —`. `(1058 rows in all, so 4428 + 1058 = 5486 forms total)` → `(1083 rows in all, so 4572 + 1083 = 5655 forms total)`. Bucket counts: `a second (587 cells), a third (143 cells)` → `(594 cells)`, `(152 cells)`.
- The 7b paragraph's `**8.2.74 and 8.2.75 above 8.2.73**` sentence stays true. After its parenthesis ending `which made its \`p.log\` read unreachable and let` / `it be deleted).`, add ` Since juhotyādi 3f, 8.2.75 carries no gaṇa test (√kit's *acikeH*); 8.2.73 and 8.2.74 are still rudhādi-only.`
- The audit record: after `and that by juhotyādi 3e's (\`tools/audit/README.md\`'s 2026-10-01` / `entry, 4428 cells / 5486 forms / 97 roots)` add `, and that by juhotyādi 3f's (\`tools/audit/README.md\`'s YYYY-MM-DD entry, 4572 cells / 5655 forms / 101 roots)`.
- The stale-comment ledger: `4428 goldens` / `would move today` → `4572 goldens`. After the 3e sentence's end (`…both lines measured by grep at this commit).`, before ` A third,`), add: ` Juhotyādi 3f touched neither comment either; the corpus stands at 4572 cells as of 3f (\`guna.rs:2384\`'s claim anchored at \`guna.rs:<G>\`, \`controller.rs:206\`'s at \`controller.rs:<C>\`; both lines measured by grep at this commit).` Measure `<G>` with `grep -n "1872 goldens move" crates/panini-prakriya/src/tinanta/guna.rs` and `<C>` with `grep -n "only 8 cells fire" crates/panini-prakriya/src/controller.rs`, after Step 7's `guna.rs` edits. Measure; never compute.

- [ ] **Step 7: The stale engine comments, and the 3a spec note**

Each of these promised something of "3f". Rewrite so it is true with 3f curated and √bhas / √jan still to come in 3f2:
- `crates/panini-prakriya/src/tinanta/guna.rs`, 7.3.87's comment: replace `` `a` is included because vidyut credits this rule on 3f's `` / `a-upadhā √bhas, √dhan and √jan, where 7.3.86 has nothing to guṇate` / `anyway. 3f inherits the rule unchanged. It changes forms there on` / `03.0021 (cikitAni), 03.0022 (tuturARi) and 03.0023 (diDizARi).` with `` `a` is included because vidyut credits this rule on the a-upadhā √dhan (slice 3f) and √bhas and √jan (slice 3f2), where 7.3.86 has nothing to guṇate anyway. Slice 3f inherited the rule unchanged; it changes forms on 03.0021 (cikitAni), 03.0022 (tuturARi) and 03.0023 (diDizARi). ``
- `guna.rs`, the unit test comment `// 3f's a-upadhā shape (√bhas, 03.0019).` → `// The a-upadhā shape of √bhas (03.0019, slice 3f2) and √dhan (03.0024, slice 3f).`
- `crates/panini-prakriya/src/tinanta/abhyasa.rs`, `` `Bas` is √bhas's (slice 3f) `` → `` `Bas` is √bhas's (slice 3f2) ``.
- `crates/panini-prakriya/src/tinanta/anga.rs`, 6.4.71's comment: `can falsify (3e's curated roots and 3f's are all consonant-initial), so` → `can falsify (3e's and 3f's curated roots, and 3f2's, are all consonant-initial), so`. Reflow if the line grows past the file's wrap.

In `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md`, after the slice-3e note beneath the "Later slices" table, add:

```markdown
> Slice 3f (`2026-10-01-juhotyadi-gana-3f-design.md`) split this table's 3f
> row: 3f took √kit, √tur, √dhiṣ and √dhan, 144 cells, with no new rule —
> 8.2.75 dropped its rudhādi gaṇa test and 8.3.24 admitted juhotyādi (the `h`
> flag above is answered: 8.3.24 reaches *daDaMhi*). √bhas and √jan, with
> 6.4.98, 6.4.100, 6.4.42, 6.4.43 and 8.2.26 and the 8.2.73 / 8.2.74 / 8.4.55
> / 8.4.40 widenings, became slice 3f2.
```

- [ ] **Step 8: Sweep for anything left stale**

```bash
grep -rn "4428\|5486\|\b1058\b\|97 roots\|97-root\|of these 97\|of the 97\|\b492\b\|776 of\|20 of its 26\|twenty of its twenty-six\|forking 144\|forking 156\|134 of them\|72 + 25\|\b587\b\|143 hold\|(143 cells)\|72 parasmaipada\|72 roots with\|juhotyadi_rows_are_the_twenty_curated\|nij_vij_vish_rows" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.md --include=*.rs
grep -rn "\b3f\b\|slice 3f\|3f's" crates tools README.md AGENTS.md docs/ARCHITECTURE.md
grep -rn "rudhādi only\|rudhādi-only\|always śnam's\|Tag::Rudhadi. first\|guards on .Tag::Rudhadi" crates/panini-prakriya/src docs/ARCHITECTURE.md AGENTS.md
grep -rn "\bkit\b\|\btur\b\|\bDiz\b\|\bDan\b\|8\.2\.75\|8\.3\.24" crates/panini-prakriya/src --include=*.rs | grep "//"
```

Expected residue:
- AGENTS.md's dated mutation record and anything naming 3e's numbers explicitly as 3e's. That is history; never rewrite it.
- "3f" where it now records what 3f *did*, and "3f2" promises.
- The 8.2.73 / 8.2.74 "rudhādi" guard comments: still true.

Every "3f" that promises something *will* happen is now false: rewrite it as done, or retarget it to 3f2 when it is √bhas's or √jan's. For every root-shape and rule-id hit, check the comment is still true with 3f curated.

- [ ] **Step 9: Run the full suite and commit**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms). Expected: PASS at 4572 cells.

```bash
mise run fmt && mise run lint
git add -A
git commit -m "docs: 3f's counts, the audit record, and the sweep

4572 cells / 5655 forms / 101 roots / 1083 ALTERNATES across README,
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
- **This slice edits `tripadi.rs` above both tripadi entries of the non-caught set**, so their line numbers move. Find them by `--list` and their shape, never by the recorded line.

- [ ] **Step 1: Measure the floor**

With nothing else running, run this twice: `time mise run test 2>&1 | tail -3` (foreground). Record both wall clocks and `cat /proc/loadavg`. The 4428-cell floor was 5.418 s / 5.419 s.

- [ ] **Step 2: Locate and probe the two uncaught equivalents at `-j 4`**

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /="
```

`adesha.rs` is untouched, so its entry should still be `adesha.rs:589:30`. The `tripadi.rs` `- with /` entry (was `1217:38`) and the permanent `-= with /=` ṇatva hang (was `1543:23`) move down by the lines Tasks 1–2 added. If `--list` shows several `- with /` candidates at column 38, pick the one in the same function as before: `git show main:crates/panini-prakriya/src/tinanta/tripadi.rs | sed -n 1210,1220p` shows its context. Write the new positions as `<T1>` (the equivalent) and `<T2>` (the hang), then (foreground, timeout 600000 ms):

```bash
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 60 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:589:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" 2>&1 | tail -10
```

Both must be MISSED, not TIMEOUT. Read each test-phase duration from `$SCRATCH/mutants.out/outcomes.json`.

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/juhotyadi-3f"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout 60 -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"
```

The last campaign took about 11 minutes. Run nothing CPU-heavy meanwhile. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

- [ ] **Step 4: Read the outcomes**

When `pgrep -x cargo-mutants` returns nothing:

```bash
date -u +"%F %T UTC" > "$OUT/finished"
cp "$OUT/mutants.out/outcomes.json" "$OUT/outcomes.durable.json"
tail -5 "$OUT/campaign.log"
cat "$OUT/mutants.out/missed.txt" "$OUT/mutants.out/timeout.txt"
```

Expected (exit code 3 is normal when a timeout is present):
- `missed.txt` holds exactly `adesha.rs:589:30: replace + with *` and `tripadi.rs:<T1>:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:<T2>:23: replace -= with /=`.
- panini-analyze: 0 missed, 0 timeout.

If not:
- Any **other timeout** is a suspect survivor. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant in 8.2.75's or 8.3.24's guard means a Task 1 or Task 2 test does not separate it. The likeliest are 8.3.24's `&&` → `||` and a deleted `!` on either gaṇa test. Strengthen the named test, commit, and re-run only those mutants with `--re` and a fresh `-o`.

- [ ] **Step 5: Margins**

Inspect one record to see how `duration` is stored:

```bash
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["outcomes"][1])' "$OUT/mutants.out/outcomes.json"
```

Compute the two equivalents' test phases under campaign load, the slowest caught phase, and the caught min/median/p90/max. The margins are 60 ÷ the longest equivalent phase and 60 ÷ the slowest caught phase.
- If the first margin is ≥ 5, the cap stays 60.
- Otherwise the new cap is 6 × that phase, rounded up to the next 10 s. Change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together. A higher cap can only turn timeouts into outcomes, and the only timeout is the permanent hang, so the campaign's outcomes stand without a re-run.

- [ ] **Step 6: Record it in AGENTS.md**

- Rewrite `**The floor behind the 60s cap, measured at 4428 cells on` / `2026-10-01.**` with Step 1's and Step 2's numbers at 4572 cells, the new `tripadi.rs` positions, and the cap Step 5 chose.
- Replace the `**Current record (juhotyādi 3e, 2026-10-01).**` paragraph, including its **Addendum**, with `**Current record (juhotyādi 3f, YYYY-MM-DD).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**, with the moved `tripadi.rs` positions;
  - the non-caught set diffed against 3e's (the clean result is identical up to the two moved `tripadi.rs` lines);
  - the campaign-load phases and margins;
  - that no 8.2.75 or 8.3.24 mutant is missed, timed out or unviable-without-reason;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`.

  End it with a pointer to the record it replaces: run `git rev-parse --short HEAD` before committing and write `The juhotyādi 3e record it replaces: \`git show <that hash>:AGENTS.md\`.`
- Update every other mention of the `tripadi.rs:1217:38` and `tripadi.rs:1543:23` positions in AGENTS.md (`grep -n "1217:38\|1543:23" AGENTS.md`) outside dated history.

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md mise.toml
git commit -m "chore: 3f mutation gate — floor and uncaught run re-measured at 4572 cells

missed.txt holds only the two documented equivalents and timeout.txt only
the permanent ṇatva-scan entry; every 8.2.75 and 8.3.24 guard clause is killed."
```

---

## Task 7: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -20
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin juhotyadi-3f
gh pr create --title "juhotyādi 3f — √kit, √tur, √dhiṣ, √dhan" --body "$(cat <<'BODY'
Slice 3f curates √kit, √tur, √dhiṣ and √dhan (`03.0021`–`03.0024`, *ciketti* /
*tutorti* / *didheṣṭi* / *dadhanti*), all parasmaipadī. This takes the golden
suite from 4428 to 4572 cells and juhotyādi to twenty-four of its twenty-six
rows.

**No new rule. Two widenings in `tripadi.rs`:**
- 8.2.75 *daś ca* drops its rudhādi gaṇa test; its guard is now the sūtra's own
  (sip, dhātu pada-final, final `d`). Gives √kit's *acikeH*. A corpus-wide test
  holds that only rudhādi and √kit reach it.
- 8.3.24 *naś cāpadāntasya jhali* admits juhotyādi beside rudhādi. Gives √dhan's
  *daDaMsi* and *daDaMhi*. The gaṇa test stays: it stands in for *apadāntasya*,
  and dropping it credits 8.3.24 on every 7.1.3 `n`.

The prep spec's 3f row was split after a re-probe against vidyut (160/216 on
HEAD): √bhas and √jan, with 6.4.98 / 6.4.100 / 6.4.42 / 6.4.43 / 8.2.26 and four
more widenings, are slice 3f2. The audit shows zero divergence against
`8da2f90b`, and the mutation gate is clean.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`: `git worktree remove .worktrees/juhotyadi-3f`, then delete the local and remote branch, and run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| 8.2.75: gaṇa test dropped, sūtra guard kept, order unchanged, comment rewritten | 1 |
| 8.2.75 guard tests: fires untagged on sip `d`; declines on tip, `t`, live ending | 1 |
| 8.3.24: `Rudhadi \|\| Juhotyadi`, comment (śnam's or √dhan's; why juhotyādi is safe; why the test stays) | 2 |
| 8.3.24 guard tests: fires on `s`/`h`/`t`; declines on bhavanti and before `m`/`v`/`y` | 2 |
| Four rows, counts, gaṇa-row test, 16 blocks, 25 `ALTERNATES`, buckets, keys | 3 |
| `check()` spot test (*acikeH*, *daDaMhi*, *tutorti* and more) | 3 |
| 3e's 7.3.87 corpus test extended to the 3f rows | 3 |
| Trace pins *acikeH*, *aciked*, *daDaMsi*, *daDaMhi*, *daDantaH* | 4 |
| Corpus-wide fires-only tests for 8.2.75 and 8.3.24 | 4 |
| Prior traces byte-identical | 1, 2 (suite at 4428), 4 (fires-only tests) |
| Audit with repoint and negative control; README/ARCHITECTURE/AGENTS; 3a-spec note; "3f" sweep | 5 |
| Floor, uncaught probe, campaign, verbatim non-caught record | 6 |

**Spec deviation, recorded.** The spec's success criterion says "the 4428 priors byte-identical, traces included" and its test list names a main↔HEAD trace diff. This plan holds that through Tasks 1–2's suite runs at 4428 (no trace pin moves) and Task 4's two fires-only tests, which are the permanent form of the same check for the only two rules that changed. It does not script a one-off trace dump diff.

**Type consistency.** Rule ids and names are `"8.2.75"` / `"daSca"` and `"8.3.24"` / `"naScApadAntasya Jali"` across the rules, unit tests, pins and fires-only tests. `sip_prakriya(&str, &str, Purusha)` and `dhan_prakriya(&str)` are defined in Tasks 1 and 2 and used only there. `branch_trace(&str, Lakara, Purusha, Vacana, &str) -> Vec<String>`, `ALL_CELLS` and `credited(&str) -> Vec<(&'static str, Gana)>` are defined in Task 4 Step 1 and used only there.

**Known soft spots.**
- **ALTERNATES keys.** Computed from HEAD's traces before Tasks 1–2. The `acikeH` key (`7.3.86+8.2.75`) is inferred, since HEAD could not derive the form yet. `every_alternate_names_the_vikalpa_rules_that_produced_it` will say so if it is wrong.
- **`check()` field names.** Task 3 uses `a.dhatu`, `a.trace` and `s.sutra`, as the 3e test beside it does.
- **The `threes` message and the `\` continuations.** Edit without breaking them.
- **`tripadi.rs` line drift.** Task 6 finds the two moved entries by `--list` and shape.
