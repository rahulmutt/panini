# Adādi Vidhiliṅ Ungate — Slice 5b Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Teach the tiṅanta engine the athematic optative junction so adādi (√yā/√vā) derives vidhiliṅ correctly, then delete the slice-5a scope gate that declined those cells.

**Architecture:** Two text-shape rules on the fused ending term resolve the yāsuṭ-`ā` + vowel junction that śap-luk exposes for the first time: a new **6.1.96 *usyapadāntāt*** (`yAus` → `yus`) and a third arm on the existing **6.1.101 *akaḥ savarṇe dīrghaḥ*** (`yAam` → `yAm`). Both run after 7.2.80, so they are provably inert for the thematic gaṇas (whose liṅ endings are already `iy…`-shaped by then). Once both rules produce the right forms, the `Gana::Adadi × VidhiLin` decline-branch in `derive` is removed and the `GATED` list emptied in a single atomic flip.

**Tech Stack:** Rust workspace (`panini-data`, `panini-prakriya`, `panini`), `mise` task runner, SLP1 internal encoding, golden + trace + mutation tests.

## Global Constraints

- Toolchain via `mise` (rust 1.97.1); build/test with `mise run build | test`. Never install Rust globally.
- `#![forbid(unsafe_code)]` in every crate touched.
- SLP1 is the only internal representation; no transliteration outside `panini-lipi` (untouched here).
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, not as a branch inside `derive`.
- Sūtra ids/names in traces must match the reference (ashtadhyayi.com / the `ashtadhyayi-com/data` GitHub mirror). Task 1 verifies the one new name (6.1.96) before it is baked into `record()`.
- **The 918 existing golden forms and every pinned bhvādi/divādi/tudādi/adādi trace must never change.** Both new rules run after 7.2.80 and guard on shapes (`SHAP` empty + `yA`-initial ending; `a/A` immediately before `us`) that only adādi vidhiliṅ produces. Any change to a pre-existing golden/trace is a bug — run `mise run test` before and after each rule.
- **Scope of 5b (this slice):** ONLY the vidhiliṅ ungate for the two ā-final adādi roots already present (√yā, √vā). Do NOT add the consonant-final / voiced / ātmanepada roots (√ad, √vas, √ās, √śī) or their junction sandhi (8.4.55 cartva, 8.2.39 jaśtva, 7.4.21, 7.1.6) — those are separate follow-on slices. The spec (`docs/superpowers/specs/2026-07-23-adadi-vidhilin-5b-design.md`) is the shared design.

## Two facts that drive the task order

1. **The gate lives inside `derive`, before `run_pipeline`.** While the gate is up, `derive(adādi, VidhiLin, …)` returns a *blocked* prakriya, and `check` (which derives) never produces the adādi vidhiliṅ forms. So the two new rules **cannot** be observed through `derive`/`check`/`trace_for` until the gate is gone. Tasks 1–2 therefore test each new rule **in isolation**, by building a `Prakriya` by hand and calling `(rule.apply)(&mut p)` directly — the established single-rule unit-test idiom in `tinanta.rs` (see e.g. `jher_jus_replaces_ji_and_elides_the_j_marker`). This bypasses the gate, so the full suite stays green while the rules go in.

2. **The gate, the goldens, and the negatives must flip together.** Removing the gate turns the two wrong cells' output into `check`-visible forms; if the rules were absent that would make `yAyAuH`/`yAyAam` VALID and break the 5a negatives. Because Tasks 1–2 install the rules first, the flip in Task 3 is clean: `derive` now yields `yAyuH`/`yAyAm`, so `check("yAyAuH")` stays INVALID (nothing produces it) and the new goldens validate. Task 3 is the single atomic integration commit.

## Cell order convention

Every 9-cell paradigm array and trace list is ordered `[P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B]` — Prathama/Madhyama/Uttama × Eka/Dvi/Bahu (3sg, 3du, 3pl, 2sg, 2du, 2pl, 1sg, 1du, 1pl). The two cells this slice fixes are **P.B (3pl)** `yāyuḥ` and **U.E (1sg)** `yāyām`.

## Target paradigms (verified by running the ungated pipeline during planning)

√yā vidhiliṅ: `yAyAt, yAyAtAm, yAyuH, yAyAH, yAyAtam, yAyAta, yAyAm, yAyAva, yAyAma`
√vā vidhiliṅ: `vAyAt, vAyAtAm, vAyuH, vAyAH, vAyAtam, vAyAta, vAyAm, vAyAva, vAyAma`

Seven of nine already derive from the slice-2 chain once the gate is lifted; only P.B and U.E need the two new rules.

---

### Task 1: Add rule 6.1.96 *usyapadāntāt* (produces the 3pl `yus`)

Add the new rule to `TINANTA_RULES`, immediately after the 6.1.101 rule (they
target disjoint shapes, `…us` vs the `yA…` arm added in Task 2, so relative
order is free; grouping the two junction rules keeps them together). The rule
elides an `a`/`ā` standing immediately before the ending `us`. Tested in
isolation because the gate still declines this cell through `derive`.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (insert a `Rule` after the `id: "6.1.101"` rule, ~line 1044; add a unit test in the `#[cfg(test)] mod tests`)

**Interfaces:**
- Consumes: the `Rule { id, name, kind: RuleKind::Vidhi, apply }` shape; the `ENDING` index const; `p.terms[ENDING].text`, `p.snapshot()`, `p.record(id, name, before)`; `Term::new`, `Context::new`, and the `Prakriya { terms, log, ctx, blocked }` literal used by existing unit tests.
- Produces: a `TINANTA_RULES` entry `id: "6.1.96"` that rewrites an ending `…{a|A}us` → `…us`. Later tasks rely on this firing for adādi vidhiliṅ 3pl (`yAus` → `yus`).

- [ ] **Step 1: Write the failing unit test.** Add to the `#[cfg(test)] mod tests` block in `crates/panini-prakriya/src/tinanta.rs`:

```rust
    #[test]
    fn usyapadantat_drops_a_before_us_and_spares_iyus() {
        // Fires: after 7.2.79 the adādi liṅ 3pl ending is `yAus`; the ā
        // before `us` drops -> `yus`.
        let mut p = Prakriya {
            terms: vec![Term::new("yA"), Term::new(""), Term::new("yAus")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Bahu,
            ),
            blocked: false,
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "6.1.96").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "yus");

        // Declines: the thematic liṅ 3pl ending is `iyus` (7.2.80 rewrote yA
        // -> iy); the char before `us` is `y`, not a/ā, so nothing changes.
        let mut q = Prakriya {
            terms: vec![Term::new("Bav"), Term::new("a"), Term::new("iyus")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Bahu,
            ),
            blocked: false,
        };
        assert!(!(rule.apply)(&mut q));
        assert_eq!(q.terms[ENDING].text, "iyus");
    }
```

- [ ] **Step 2: Run it to confirm it fails.**

Run: `cargo test -p panini-prakriya usyapadantat_drops_a_before_us_and_spares_iyus -- --exact`
Expected: FAIL — `TINANTA_RULES.iter().find(|r| r.id == "6.1.96")` returns `None`, so `.unwrap()` panics ("no 6.1.96 rule yet").

- [ ] **Step 3: Verify the sūtra id/name against the reference.** Fetch the 6.1.96 entry from the `ashtadhyayi-com/data` mirror (or ashtadhyayi.com) and confirm the SLP1 name is `usyapadAntAt`. The reference implementation (vidyut-prakriya, `ac_sandhi.rs`) applies exactly this rule at the `a/A` + `us` junction (cited example *bhindyuḥ*). If the reference text differs, use the reference's SLP1 verbatim and note the correction in the commit message (as slice 2 did for 3.4.103).

- [ ] **Step 4: Add the rule.** Insert this `Rule` in `crates/panini-prakriya/src/tinanta.rs` immediately after the closing `},` of the `id: "6.1.101"` rule, before the `// 6.1.90 āṭaś ca:` comment:

```rust
    // 6.1.96 usyapadāntāt: an a/ā immediately before the ending `us` is
    // elided (a single substitution in the ekaḥ pūrvaparayoḥ section). Fires
    // only for adādi vidhiliṅ 3pl: after 7.2.79 strips yāsuṭ's s, the ending
    // is `yAus`, and here the ā before `us` drops -> `yus` -> yA + yuH.
    // Inert for the thematic gaṇas: 7.2.80 has already rewritten their liṅ
    // 3pl ending to `iyus`, whose segment before `us` is `y`, not a/ā.
    Rule {
        id: "6.1.96",
        name: "usyapadAntAt",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let e = &p.terms[ENDING].text;
            if !e.ends_with("us") {
                return false;
            }
            let n = e.chars().count();
            // the char immediately before the final `us` (None if the ending
            // is just "us", which wrapping_sub keeps panic-free)
            let pre = e.chars().nth(n.wrapping_sub(3));
            if !matches!(pre, Some('a') | Some('A')) {
                return false;
            }
            let before = p.snapshot();
            let kept: String = e.chars().take(n - 3).collect();
            p.terms[ENDING].text = format!("{kept}us");
            p.record("6.1.96", "usyapadAntAt", before);
            true
        },
    },
```

- [ ] **Step 5: Run the unit test to confirm it passes.**

Run: `cargo test -p panini-prakriya usyapadantat_drops_a_before_us_and_spares_iyus -- --exact`
Expected: PASS.

- [ ] **Step 6: Run the full suite — no existing form/trace may change.**

Run: `mise run test`
Expected: PASS (gate still up, so `check` behavior is unchanged; 918 goldens and all traces identical). The new rule is dormant for every currently-derivable form.

- [ ] **Step 7: Commit.**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "feat(prakriya): 6.1.96 usyapadAntAt — elide a/A before ending us"
```

---

### Task 2: Add the 6.1.101 third arm (produces the 1sg `yAm`)

Extend the existing 6.1.101 rule with a third arm: the yāsuṭ-internal `Aa`
(ending `yAam`, 1sg) coalesces to a single `A` (`yAm`). Tested in isolation for
the same gate reason as Task 1.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (the `apply` of the `id: "6.1.101"` rule and its doc comment; add a unit test)

**Interfaces:**
- Consumes: the existing 6.1.101 rule body (two arms), `Lakara::VidhiLin`, `p.ctx.lakara`, `SHAP`/`ENDING`/`ANGA` consts.
- Produces: 6.1.101 now also rewrites a `VidhiLin` ending `yA{a|A}…` → `yA…` (dropping the third char) when `SHAP` is empty. Later tasks rely on this for adādi vidhiliṅ 1sg (`yAam` → `yAm`).

- [ ] **Step 1: Write the failing unit test.** Add to the `#[cfg(test)] mod tests` block in `crates/panini-prakriya/src/tinanta.rs`:

```rust
    #[test]
    fn savarna_dirgha_adadi_lin_1sg_arm() {
        let rule = TINANTA_RULES.iter().find(|r| r.id == "6.1.101").unwrap();

        // Fires: adādi liṅ 1sg ending `yAam` (śap empty) -> `yAm`.
        let mut p = Prakriya {
            terms: vec![Term::new("yA"), Term::new(""), Term::new("yAam")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Uttama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "yAm");

        // Declines: the `yA` of `yAt` (2sg-shape) is followed by a consonant,
        // not a vowel, so no savarṇa coalescence.
        let mut q = Prakriya {
            terms: vec![Term::new("yA"), Term::new(""), Term::new("yAt")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        assert!(!(rule.apply)(&mut q));
        assert_eq!(q.terms[ENDING].text, "yAt");

        // Declines: thematic liṅ (śap = `a`, non-empty) is never touched by
        // this arm — the SHAP-empty guard is what scopes it to adādi.
        let mut r = Prakriya {
            terms: vec![Term::new("Bav"), Term::new("a"), Term::new("iyam")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Uttama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        assert!(!(rule.apply)(&mut r));
        assert_eq!(r.terms[ENDING].text, "iyam");
    }
```

- [ ] **Step 2: Run it to confirm it fails.**

Run: `cargo test -p panini-prakriya savarna_dirgha_adadi_lin_1sg_arm -- --exact`
Expected: FAIL — the first sub-case asserts `yAm`, but with no third arm the existing rule declines on `yAam` (ending starts with `y`, not a/ā), leaving `yAam`, so the `assert_eq!(…, "yAm")` panics.

- [ ] **Step 3: Add the third arm.** In `crates/panini-prakriya/src/tinanta.rs`, in the `id: "6.1.101"` rule's `apply`, insert this arm **before** the existing first `if` (the `SHAP.text.is_empty() && ANGA ends_with 'A' && ending starts a/A` block). Placing it first documents its precedence; the existing arm would decline on this shape anyway (the ending starts with `y`):

```rust
            // adādi vidhiliṅ 1sg: after 7.2.79 the ending is `yAam` (yāsuṭ ā +
            // the uttama-eka `am`). 7.2.80 would have rewritten `yA`->`iy` for
            // a thematic gaṇa, but śap is luk'd so it declined; the yāsuṭ ā and
            // the ending a are savarṇa -> a single ā: yAam -> yAm. Guard is
            // tight: VidhiLin + empty śap + a `yA`+vowel ending (never `yAt`/
            // `yAs`/... whose yA is followed by a consonant).
            if p.terms.len() > ENDING
                && matches!(p.ctx.lakara, Lakara::VidhiLin)
                && p.terms[SHAP].text.is_empty()
                && p.terms[ENDING].text.starts_with("yA")
                && matches!(p.terms[ENDING].text.chars().nth(2), Some('a') | Some('A'))
            {
                let before = p.snapshot();
                // drop the ending's third char (the a/A after `yA`)
                let kept: String = p.terms[ENDING]
                    .text
                    .chars()
                    .enumerate()
                    .filter(|&(i, _)| i != 2)
                    .map(|(_, c)| c)
                    .collect();
                p.terms[ENDING].text = kept;
                p.record("6.1.101", "akaH savarRe dIrGaH", before);
                return true;
            }
```

- [ ] **Step 4: Update the 6.1.101 doc comment.** Replace the rule's leading comment (the `// 6.1.101 akaḥ savarṇe dīrghaḥ ... Two arms:` block and its two bullet lines) with a three-arm version:

```rust
    // 6.1.101 akaḥ savarṇe dīrghaḥ: an ak vowel followed by a savarṇa vowel
    // coalesces into the corresponding long vowel. Three arms:
    //   - adādi vidhiliṅ 1sg (śap luk'd, 7.2.80 declined): the yāsuṭ ā + the
    //     ending a coalesce inside the ending, yAam → yAm (→ yAyAm);
    //   - adādi (śap luk'd by 2.4.72): the aṅga's own final `A` meets an
    //     a/ā-initial ending, yA + anti → yAnti, yA + Ani → yAni;
    //   - bhvādi &c.: śap `a` + the ending's initial `A` (from 3.4.92 āḍ),
    //     Bav + a + Ani → BavAni.
```

- [ ] **Step 5: Run the unit test to confirm it passes.**

Run: `cargo test -p panini-prakriya savarna_dirgha_adadi_lin_1sg_arm -- --exact`
Expected: PASS.

- [ ] **Step 6: Run the full suite — no existing form/trace may change.**

Run: `mise run test`
Expected: PASS (gate still up; 918 goldens and all traces identical; the new arm only fires on the empty-śap `yA`+vowel shape no currently-derivable form reaches).

- [ ] **Step 7: Commit.**

```bash
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "feat(prakriya): 6.1.101 adadi vidhilin 1sg arm (yAam -> yAm)"
```

---

### Task 3: Flip the gate — ungate, goldens, negatives, roundtrip (atomic)

The integration commit. With both rules in place, remove the gate, pin the 18
forms as `check`-based goldens, add the two full-derivation traces, empty
`GATED`, retarget the 5a negatives comment, and drop the now-dead `blocked` arm
in roundtrip — all together, because they only stay green as a unit.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` (delete the gate block)
- Modify: `crates/panini/tests/paradigm.rs` (2 PARADIGM blocks; empty + simplify GATED guard; retarget negatives comment)
- Modify: `crates/panini/tests/trace.rs` (2 trace tests)
- Modify: `crates/panini/tests/roundtrip.rs` (remove dead `blocked` arm)

**Interfaces:**
- Consumes: `PARADIGM` (`&[(&str, &str, [&str; 9])]`), `paradigm_covers_every_enumerable_cell`, the `trace_for` helper, the negatives loop, the roundtrip loop.
- Produces: `check("yAyuH")`/`check("yAyAm")` (and vā) VALID and attributed to √yā/√vā vidhiliṅ; `derive` has no gana×lakāra branch; `GATED` empty; every enumerable cell pinned in `PARADIGM` (26 roots × 4 lakāras = 104 blocks; `every_form_validates_and_matches` now walks 936 forms).

- [ ] **Step 1: Write the failing trace tests.** Add both to `crates/panini/tests/trace.rs`. These use the file's standard `trace_for` helper (works once the gate is gone):

```rust
#[test]
fn yayuh_trace_is_the_adadi_us_junction_path() {
    // √yā adādi vidhiliṅ 3pl: Ji -> jus (3.4.108) -> us, śap inserted
    // (3.1.68) then luk'd (2.4.72), yāsuṭ's s elided (7.2.79) -> yAus, the ā
    // before us drops (6.1.96) -> yus, word-final s -> visarga (8.3.15):
    // yA + yuH -> yAyuH.
    assert_eq!(
        trace_for("yAyuH"),
        vec![
            "1.3.78", "3.4.78", "3.4.108", "1.3.9", "3.4.103", "3.1.68", "1.3.9",
            "2.4.72", "7.2.79", "6.1.96", "8.3.15"
        ]
    );
}

#[test]
fn yayam_trace_is_the_adadi_am_junction_path() {
    // √yā adādi vidhiliṅ 1sg: mip -> am (3.4.101), yāsuṭ prefixed (3.4.103)
    // -> yAsam, śap inserted (3.1.68) then luk'd (2.4.72), yāsuṭ's s elided
    // (7.2.79) -> yAam, then yāsuṭ ā + ending a coalesce (6.1.101 new arm)
    // -> yAm: yA + yAm -> yAyAm. No 8.3.15 (ends in m).
    assert_eq!(
        trace_for("yAyAm"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.101", "3.4.103", "3.1.68", "1.3.9",
            "2.4.72", "7.2.79", "6.1.101"
        ]
    );
}
```

- [ ] **Step 2: Run them to confirm they fail.**

Run: `cargo test -p panini --test trace -- yayuh_trace yayam_trace`
Expected: FAIL — `trace_for` calls `check`, which derives through the still-active gate, so no analysis produces `yAyuH`/`yAyAm`; the `.expect("expected an analysis…")` panics.

- [ ] **Step 3: Remove the gate in `derive`.** In `crates/panini-prakriya/src/tinanta.rs`, delete the entire `// ---- SLICE 5a SCOPE BOUNDARY …` comment block and the `if matches!(dhatu.gana, Gana::Adadi) && matches!(lakara, Lakara::VidhiLin) { p.blocked = true; return p; }` block (~lines 1308–1323). The tail of `derive` becomes:

```rust
        match dhatu.gana {
            Gana::Divadi => t.add(Tag::Divadi),
            Gana::Tudadi => t.add(Tag::Tudadi),
            Gana::Adadi => t.add(Tag::Adadi),
            Gana::Bhvadi => {}
        }
        t
    });
    run_pipeline(&mut p, TINANTA_RULES);
    p
}
```

- [ ] **Step 4: Add the two golden blocks.** In `crates/panini/tests/paradigm.rs`, insert into the `PARADIGM` array immediately after the `("vA", "loT", [...])` block (the last adādi block, ~line 1043), before the closing `];`:

```rust
    (
        "yA",
        "viDiliN",
        [
            "yAyAt", "yAyAtAm", "yAyuH", "yAyAH", "yAyAtam", "yAyAta", "yAyAm", "yAyAva", "yAyAma",
        ],
    ),
    (
        "vA",
        "viDiliN",
        [
            "vAyAt", "vAyAtAm", "vAyuH", "vAyAH", "vAyAtam", "vAyAta", "vAyAm", "vAyAva", "vAyAma",
        ],
    ),
```

- [ ] **Step 5: Empty GATED and simplify its guard.** In `crates/panini/tests/paradigm.rs`, in `paradigm_covers_every_enumerable_cell`, replace the whole preamble — the slice-5a comment, the `const GATED` line, the machine-guard comment, the `assert!(GATED.len() <= 2, …)`, and the `for &(root, lakara) in GATED { … }` loop — with:

```rust
    // adādi × vidhiliṅ was gated in slice 5a and ungated in slice 5b; there
    // are no gated cells any more. This constant stays (empty) so the two
    // assertions below keep documenting that EVERY enumerable (root, lakara)
    // pair must be pinned in PARADIGM — a future partial slice may repopulate
    // it, but it must never silently hide a missing golden block.
    const GATED: &[(&str, &str)] = &[];
```

Leave the rest of the function (the `pinned`/`unpinned`/`gated` comparison and the `PARADIGM.len() + GATED.len()` count assertion) unchanged. Deleting the `matches!(d.gana, Gana::Adadi)` guard removes the only use of `Gana` in this file (it appears only at that line and in the import), so change the top import from `use panini_data::{Gana, Lakara, Pada, Purusha, Vacana, dhatus};` to `use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};`.

- [ ] **Step 6: Retarget the negatives comment.** In `crates/panini/tests/paradigm.rs`, replace the comment block above the four strings `yAyAuH`/`yAyAam`/`vAyAuH`/`vAyAam` (the `// adādi × vidhiliṅ is GATED UNTIL SLICE 5b …` paragraph, ~lines 1215–1225) with:

```rust
        // These four are the non-words the pre-5b pipeline emitted for adādi
        // vidhiliṅ before 6.1.96 / the 6.1.101 arm reduced the yāsuṭ-ā + vowel
        // junction. They stay pinned INVALID as the regression that the
        // reduction actually RAN: the real forms are yAyuH / yAyAm (and the vā
        // pair), now pinned as goldens in PARADIGM. If any of these four ever
        // validates, the junction reduction regressed.
```

Leave the four string literals and their trailing inline comments as-is.

- [ ] **Step 7: Remove the dead `blocked` arm in roundtrip.** In `crates/panini/tests/roundtrip.rs`, inside `generate_then_check_recovers_inputs`, delete the entire `if p.blocked { … continue; }` block. The loop always derives with `d.pada`, so 1.3.12/1.3.78 never block, and the adādi vidhiliṅ gate is gone — nothing is ever blocked here. The plain roundtrip assertion (`assert!(r.analyses.iter().any(|a| a.dhatu == d.code && a.form_slp1 == form && a.lakara == lakara), …)`) then runs for every cell, including the four new adādi vidhiliṅ ones.

- [ ] **Step 8: Run the targeted tests.**

Run: `cargo test -p panini --test trace -- yayuh_trace yayam_trace` then `cargo test -p panini --test paradigm` then `cargo test -p panini --test roundtrip`
Expected: PASS. The two traces match; `every_form_validates_and_matches` validates 936 forms; `paradigm_covers_every_enumerable_cell` passes with empty `GATED` (no unpinned cells); the four negatives remain INVALID; roundtrip recovers all 936 forms.

- [ ] **Step 9: Run the full suite.**

Run: `mise run test`
Expected: PASS across all crates.

- [ ] **Step 10: Commit.**

```bash
git add crates/panini-prakriya/src/tinanta.rs crates/panini/tests/paradigm.rs crates/panini/tests/trace.rs crates/panini/tests/roundtrip.rs
git commit -m "feat: ungate adadi vidhilin — 936 goldens, yAyuH/yAyAm traces, empty GATED"
```

---

### Task 4: Update the prose docs (AGENTS.md + stale slice comments)

The gate is gone, so the AGENTS.md "one adjudicated exception" note and the
`slice 5b` forward-references in the source are stale. This is the
documentation task the slice exists to discharge.

**Files:**
- Modify: `AGENTS.md`
- Modify: `crates/panini-prakriya/src/tinanta.rs` (residual `5a`/`5b` comments)
- Modify: `crates/panini/tests/paradigm.rs`, `crates/panini/tests/roundtrip.rs` (residual slice mentions)

**Interfaces:**
- Consumes: nothing (prose only — no code or guard changes).
- Produces: docs describing adādi as having uniform four-lakāra coverage for √yā/√vā and no scope gate.

- [ ] **Step 1: Update the AGENTS.md golden-count + coverage line.** In `AGENTS.md`, replace the paradigm-test bullet (currently `918 forms; … slice 5a covers √yā/√vā in laṭ/laṅ/loṭ; the consonant-final/ātmanepada roots and vidhiliṅ land in slice 5b`) with:

```
- Grammar changes are gated by the golden paradigm test
  (`crates/panini/tests/paradigm.rs`, 936 forms; bhvādi/divādi/tudādi are
    complete across laṭ/laṅ/loṭ/vidhiliṅ × parasmaipada/ātmanepada, and adādi
    (gaṇa 2) is being added — √yā/√vā are complete across all four lakāras
    (laṭ/laṅ/loṭ/vidhiliṅ); the consonant-final, voiced-junction, and
    ātmanepada roots (√ad, √vas, √ās, √śī) land in later slices)
```

- [ ] **Step 2: Remove the AGENTS.md "adjudicated exception" note.** The gate it describes no longer exists. Replace the bullet (currently `New grammar goes in TINANTA_RULES … The one adjudicated exception is the slice 5a scope gate … Do not "fix" it back into TINANTA_RULES.`) with:

```
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, not as a
  branch inside `derive`. `derive` carries no grammar branches: the only
  gana-conditioned logic there is aṅga tagging (`Tag::Adadi` &c.), which feeds
  the guarded rules rather than substituting for them.
```

- [ ] **Step 3: Reword residual slice labels in tinanta.rs.** The tinanta.rs header caveat about `terms[SHAP].text` possibly being EMPTY (~lines 71–76) is still accurate — leave it. Find the remaining slice-labelled comments:

Run: `grep -n "5b\|5a\|SLICE" crates/panini-prakriya/src/tinanta.rs`
Expected hits (after Task 3 deleted the gate block): the 6.1.78 comment (~line 964, `5b generalizes this …`) and the ~line 1220 comment (`Unreachable in 5a … 5b must generalize …`). For each, replace the slice number with a scope-neutral phrase (e.g. "a later slice" / "when the consonant-final and ātmanepada adādi roots land"). **Comments only — do not touch any code or guard.** If the grep shows any surviving `SLICE 5a SCOPE BOUNDARY` text, the gate block was not fully deleted in Task 3 — go back and remove it.

- [ ] **Step 4: Sweep the test files for residual slice mentions.**

Run: `grep -rn "slice 5b\|slice 5a\|GATED UNTIL\|until slice" crates/panini/tests/`
Expected: only intentional history mentions remain. Reword any that still imply adādi vidhiliṅ is gated or pending (Tasks 3 handled the main ones; this is the backstop). Comments only.

- [ ] **Step 5: Verify the whole build + docs are consistent.**

Run: `mise run build && mise run test && mise run lint && mise run fmt-check`
Expected: all PASS.

- [ ] **Step 6: Commit.**

```bash
git add AGENTS.md crates/panini-prakriya/src/tinanta.rs crates/panini/tests/paradigm.rs crates/panini/tests/roundtrip.rs
git commit -m "docs: adadi vidhilin complete for yA/vA; retire scope-gate exception note"
```

---

### Task 5: Mutation testing — confirm the two new guards are pinned

**Files:**
- No source changes expected. If mutants survive, add targeted unit tests/negatives to kill them.

**Interfaces:**
- Consumes: `mise run mutants`.
- Produces: evidence the 6.1.96 `a/A`-before-`us` guard and the 6.1.101 `yA`+vowel arm are killed by the unit/golden/trace/negative tests.

- [ ] **Step 1: Run mutation testing.**

Run: `MISE_ENV=dev mise install && mise run mutants`
Expected: no survivors in the new 6.1.96 rule or the 6.1.101 third arm. (The `usyapadantat_…` unit test + `yayuh_trace` + the `yAyuH` golden kill 6.1.96 mutants; the `savarna_dirgha_adadi_lin_1sg_arm` unit test + `yayam_trace` + the `yAyAm` golden kill 6.1.101-arm mutants; the four retained negatives kill a reduction that fails to run.)

- [ ] **Step 2: If a new-rule mutant survives, kill it.** Read what the mutant changed (e.g. the `Some('a') | Some('A')` guard mutated to always-true, `ends_with("us")` negated, or the `nth(2)` index changed) and add the smallest unit assertion in `tinanta.rs`'s test mod (or negative in paradigm.rs) that distinguishes the mutant from the real rule. Re-run `mise run mutants` until the new-rule mutants are caught. Commit any added tests:

```bash
git add crates/panini-prakriya/src/tinanta.rs crates/panini/tests/
git commit -m "test(prakriya): pin adadi vidhilin junction guards against surviving mutants"
```

- [ ] **Step 3: Final full verification.**

Run: `mise run build && mise run test && mise run lint && mise run fmt-check && mise run audit`
Expected: all PASS. Slice complete: 936 goldens, adādi √yā/√vā complete across four lakāras, no scope gate.
