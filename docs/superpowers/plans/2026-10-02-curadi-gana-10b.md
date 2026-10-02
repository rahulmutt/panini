# Curādi gaṇa slice 10b Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate four ākusmīya curādi roots — √cit (`10.0192`), √vṛṣ (`10.0228`), √mad (`10.0229`), √kusm (`10.0236`) — ātmanepadī by the dhātupāṭha gaṇasūtra **10.0496 *ā kusmād ātmanepadinaḥ***, credited by a new first rule of the `sanadi` stage. The golden suite goes from 4932 to 5076 cells.

**Architecture:** Eight tasks.
- **Task 1** checks the worktree and baseline.
- **Task 2** adds `PadaAssignment::Akusmiya`, the `AKUSMIYA` range and `Tag::Akusmiya`, wired through `derive`, with no rows yet.
- **Task 3** adds the 10.0496 rule at the head of `SANADI`, and 1.3.78's ātmanepada decline for an ākusmīya aṅga.
- Tasks 2–3 fire on no curated cell. Each is gated on **its unit tests plus the 4932 priors staying green**.
- **Task 4** lands the four rows and their goldens (144 cells, ātmanepada only).
- **Task 5** adds the trace pins and the fires-only tests.
- **Tasks 6–8** are the audit with the prior-trace diff and doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.99.0 pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, checked out at `/tmp/vidyut-full`.

**Spec:** `docs/superpowers/specs/2026-10-02-curadi-gana-10b-design.md`

**Workspace:** the branch `curadi-10b` is checked out at `/workspace/.worktrees/curadi-10b` and holds the spec and this plan. Every path below is relative to that directory unless it starts with `/`.

**Provenance.** Every code block in this plan ran green on a throwaway worktree, `/workspace/.worktrees/curadi-10b-proto` (detached; delete it in Task 8). That run covered:
- the full suite and clippy `-D warnings`;
- the audit at 111 roots / 5076 cells / 6206 forms with **zero differences** against vidyut, the `entry` control failing on 36 √bhū cells;
- a throwaway golden probe: all 144 ātmanepada cells equal to vidyut's sets, no cell with a second form, and all 144 parasmaipada cells empty in vidyut and blocked here;
- a byte-identical dump of all 6062 prior live-branch logs, `main` vs prototype;
- a scoped mutation probe over the new code: 6 mutants (10.0496's `delete !`, 1.3.78's three `||` → `&&`), all caught.

## Global Constraints

- **The 4932 pre-existing cells must stay byte-identical, traces included.** Regenerate no golden and change no pinned trace.
- **One new pipeline entry, first of all:** `tinanta_rule_order_is_pinned` grows from 136 to 137 ids and opens `"10.0496", "3.1.25", …`. **No new vikalpa**: `exactly_the_pinned_vikalpa_rules_are_optional` is unchanged.
- **10.0496 is a gaṇasūtra id, not an Aṣṭādhyāyī sūtra.** Spell it exactly `"10.0496"`, name `"A kusmAd AtmanepadinaH"`. No code parses rule ids; do not add any that does.
- **No pada sūtra may be credited on an ākusmīya cell**: not 1.3.12, not 1.3.74, not 1.3.78. 10.0496 sanctions ātmanepada and **blocks** parasmaipada without recording.
- **Pada is row-driven**: `PadaAssignment::Akusmiya` maps to `Tag::Akusmiya`, read only by 10.0496 and by 1.3.78's ātmanepada decline. Membership is held to `AKUSMIYA` (`"10.0192"..="10.0236"`) by `curated_pada_agrees_with_upadesha_markers`.
- **No unfalsifiable guard clauses.** A clause no cell and no guard test can make false is a mutation survivor.
- **Goldens are transcribed from this plan**: the prototype's output, matched against vidyut at `8da2f90b`. **Do not edit a golden to match the engine.**
- A stage's unit test looks a rule up in **its own stage static** (`SANADI`, `SAMJNA`), never `rules().find(…)`.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit.
- `mise run test` takes about 10 s. Run it in the **foreground** with a timeout of 600000 ms. Never background it and end a turn.
- `mise run test -- -p X` does not scope. Scope unit tests with `mise exec -- cargo test -p <crate> <filter>`.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **A parasmaipada request for an ākusmīya root** (`derive(…, Parasmaipada, …)` or `check("cetayati")`). Must produce only blocked branches, recording nothing, and `check` must say Invalid. → Task 3 `a_kusmad_blocks_an_akusmiya_roots_parasmaipada`; Task 4 `curadi_analyses_its_akusmiya_forms`; Task 5 `an_akusmiya_roots_parasmaipada_is_blocked_by_a_kusmad_alone`.
2. **1.3.78 blocking an ākusmīya ātmanepada cell** (its guard is `!Atmanepadin`, which admits the root). Would erase all 144 cells. → Task 3 `every_pada_sutra_leaves_an_akusmiya_root_to_10_0496`.
3. **1.3.74 or 1.3.12 being credited on an ākusmīya cell** (the misframing the 10a spec carried). → Task 3 (same test); Task 5 `a_kusmad_is_credited_on_exactly_the_akusmiya_cells` and the `check()` trace assertions in Task 4.
4. **A row on the wrong side of the range boundary** (`10.0191`/`10.0237` curated `Akusmiya`, or an in-range row curated `Nic`). → Task 4, the split curādi arm of `curated_pada_agrees_with_upadesha_markers`.
5. **A prior row reaching 10.0496.** Goldens ignore traces. → Task 5 `a_kusmad_is_credited_on_exactly_the_akusmiya_cells` (exactly 144 credits, all on the four rows); Task 6 Step 2's prior-trace diff.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-data/src/lib.rs` | Task 2: `PadaAssignment::Akusmiya`, `AKUSMIYA`, `padas()`, the `padas` test, the `Nic` doc. Task 4: four rows, the gaṇa-row test, the split pada-marker arm, the counts |
| `crates/panini-prakriya/src/term.rs` | Task 2: `Tag::Akusmiya` |
| `crates/panini-prakriya/src/tinanta/mod.rs` | Task 2: `derive`'s arm |
| `crates/panini-prakriya/src/tinanta/samjna.rs` | Task 2: test helper arm. Task 3: module doc, 1.3.74's comment, 1.3.78's arm, the ākusmīya test |
| `crates/panini-prakriya/src/tinanta/sanadi.rs` | Task 3: 10.0496 and its tests |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Task 3: the rule-order pin and its doc |
| `crates/panini/tests/paradigm/data/curadi.rs` | Task 4: sixteen golden rows |
| `crates/panini/tests/paradigm/main.rs` | Task 4: totals, slice paragraph, `check()` test. Task 6: audit prose |
| `crates/panini/tests/trace/curadi.rs` | Task 5 |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini-prakriya/src/tinanta/guna.rs` (one comment), the 10a spec | Task 6 |
| `AGENTS.md`, maybe `mise.toml` | Task 7 |

---

## Task 1: The worktree and baseline

**Files:** none. **Interfaces:** none.

- [ ] **Step 1: Confirm the worktree**

```bash
cd /workspace/.worktrees/curadi-10b
git status --short          # empty
git log --oneline -3        # the plan commit, the spec commit (6fdeb85), then d3972dd
```

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | head -20
```

Run in the foreground, timeout 600000 ms. Expected: all pass at 4932 cells; `panini-prakriya` reports 399 tests, the `trace` binary 192, `paradigm` 20, `panini-data` 22.

---

## Task 2: The variant, the range and the tag

Adds `PadaAssignment::Akusmiya`, `AKUSMIYA` and `Tag::Akusmiya`, and the `derive` arm. No row uses them yet, so no derivation changes.

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini-prakriya/src/term.rs`
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs`
- Modify: `crates/panini-prakriya/src/tinanta/samjna.rs` (test helper only)

**Interfaces:**
- Produces: `panini_data::PadaAssignment::Akusmiya`, whose `padas()` is `&[Pada::Atmanepada]`; `pub const panini_data::AKUSMIYA: std::ops::RangeInclusive<&str>` = `"10.0192"..="10.0236"`; `Tag::Akusmiya`. `derive` adds `Tag::Akusmiya` for `PadaAssignment::Akusmiya`.

- [ ] **Step 1: Write the failing test**

In `crates/panini-data/src/lib.rs`'s `padas_maps_each_assignment_to_its_derivable_padas`, replace its closing

```rust
        assert_eq!(
            PadaAssignment::Nic.padas(),
            &[Pada::Parasmaipada, Pada::Atmanepada]
        );
    }
```

with

```rust
        assert_eq!(
            PadaAssignment::Nic.padas(),
            &[Pada::Parasmaipada, Pada::Atmanepada]
        );
        assert_eq!(PadaAssignment::Akusmiya.padas(), &[Pada::Atmanepada]);
    }
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `mise exec -- cargo test -p panini-data 2>&1 | tail -5`
Expected: a compile error, because `PadaAssignment::Akusmiya` does not exist.

- [ ] **Step 3: Add the variant and the range**

In `crates/panini-data/src/lib.rs`, after `#![forbid(unsafe_code)]` and its blank line, add:

```rust
use std::ops::RangeInclusive;

```

In the `Nic` variant's doc comment, replace

```rust
    /// ṇic (3.1.25), so a curādi row with no pada marker of its own is
    /// curated with this. Like `UbhayapadaAnavane`, a root carrying it must
```

with

```rust
    /// ṇic (3.1.25), so a curādi row with no pada marker of its own is
    /// curated with this — unless it is ākusmīya (`AKUSMIYA`), which is
    /// `Akusmiya`. Like `UbhayapadaAnavane`, a root carrying it must
```

Replace the end of `enum PadaAssignment`

```rust
    /// stem instead, and this variant retires.
    Nic,
}
```

with

```rust
    /// stem instead, and this variant retires.
    Nic,
    /// Ātmanepada only, sanctioned by the dhātupāṭha gaṇasūtra 10.0496
    /// *ā kusmād ātmanepadinaḥ*: the curādi roots from `10.0192 cita~` up to
    /// `10.0236 kusma~` (`AKUSMIYA`) are ātmanepadī. Not a marker of the
    /// root's — their upadeśas carry none, so 1.3.12 never reaches them — and
    /// not the affix's, so 1.3.74 must not credit them either. A root
    /// carrying it is credited 10.0496 and no pada sūtra at all: never
    /// 1.3.12, 1.3.74 or 1.3.78.
    ///
    /// Keyed on the row's position, as the gaṇasūtra is; vidyut-prakriya
    /// reads the same range (`maybe_find_antargana`). The gaṇasūtra applies
    /// only on the ṇic branch; this engine has no optional-ṇic roots yet, so
    /// every curated ākusmīya row takes ṇic.
    Akusmiya,
}
/// The ākusmīya antargaṇa of curādi: dhātupāṭha rows `10.0192` (`cita~`)
/// through `10.0236` (`kusma~`), the scope of the gaṇasūtra 10.0496 *ā
/// kusmād ātmanepadinaḥ*. Compared as strings, which the zero-padded
/// numbering makes order-correct. A curated row is
/// `PadaAssignment::Akusmiya` exactly when it is curādi and in this range;
/// `curated_pada_agrees_with_upadesha_markers` holds both sides.
pub const AKUSMIYA: RangeInclusive<&str> = "10.0192"..="10.0236";
```

In `padas()`, after the arm ending `| PadaAssignment::Nic => &[Pada::Parasmaipada, Pada::Atmanepada],`, add:

```rust
            PadaAssignment::Akusmiya => &[Pada::Atmanepada],
```

- [ ] **Step 4: Add the tag**

In `crates/panini-prakriya/src/term.rs`, immediately after the `Nic,` variant (whose doc comment ends `/// never 1.3.72.`), insert:

```rust
    /// The dhātu's ātmanepada is sanctioned by the dhātupāṭha gaṇasūtra
    /// 10.0496 *ā kusmād ātmanepadinaḥ*: the data layer's
    /// `PadaAssignment::Akusmiya`. Read only by 10.0496 in `tinanta::sanadi`,
    /// and by 1.3.78's ātmanepada arm, which declines rather than
    /// blocks when it is present. Same standing as `Nic` and `Anavane`: a
    /// pada licence keyed to the rule that grants it, so the trace credits
    /// that rule and no pada sūtra.
    Akusmiya,
```

- [ ] **Step 5: Wire `derive` and the samjna test helper**

In `crates/panini-prakriya/src/tinanta/mod.rs`'s `derive`, add `PadaAssignment::Akusmiya => t.add(Tag::Akusmiya),` after the `PadaAssignment::Nic => t.add(Tag::Nic),` arm.

In `crates/panini-prakriya/src/tinanta/samjna.rs`'s test helper `pada_anga`, add the same arm after its `PadaAssignment::Nic => t.add(Tag::Nic),`.

- [ ] **Step 6: Run the tests**

```bash
mise exec -- cargo test -p panini-data 2>&1 | grep "test result" | head -1   # 22 passed
mise run test 2>&1 | grep -E "FAILED|test result" | head                       # all pass at 4932 cells
```

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-data/src/lib.rs crates/panini-prakriya/src/term.rs crates/panini-prakriya/src/tinanta/mod.rs crates/panini-prakriya/src/tinanta/samjna.rs
git commit -m "feat(data,prakriya): PadaAssignment::Akusmiya, the AKUSMIYA range and Tag::Akusmiya

No row uses them yet; derive maps the data layer onto Tag::Akusmiya."
```

---

## Task 3: 10.0496 *ā kusmād ātmanepadinaḥ*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/sanadi.rs`
- Modify: `crates/panini-prakriya/src/tinanta/samjna.rs`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: `Tag::Akusmiya` (Task 2); `crate::context::Context::new(Lakara, Pada, Purusha, Vacana)`; `crate::tinanta::terms::{ANGA, with_slots}`.
- Produces: the `SANADI` rule `"10.0496"`, first in the stage and so first in `TINANTA_RULES`.

- [ ] **Step 1: Write the failing sanādi tests**

In `sanadi.rs`'s `mod tests`, replace the imports

```rust
    use super::*;
    use crate::prakriya::Prakriya;
    use crate::tinanta::terms::with_slots;
```

with

```rust
    use super::*;
    use crate::context::Context;
    use crate::prakriya::Prakriya;
    use crate::tinanta::terms::with_slots;
    use panini_data::{Lakara, Purusha, Vacana};
```

and append, after the last test (`sanadyanta_folds_nic_into_the_dhatu`) and before the module's closing `}`:

```rust

    /// A bare dhātu carrying `tags`, in a laṭ prathama eka context for `pada`
    /// — all 10.0496 reads.
    fn pada_dhatu(root: &str, tags: &[Tag], pada: Pada) -> Prakriya {
        let mut anga = Term::new(root);
        anga.add(Tag::Dhatu);
        for tag in tags {
            anga.add(*tag);
        }
        Prakriya {
            ctx: Context::new(Lakara::Lat, pada, Purusha::Prathama, Vacana::Eka),
            terms: with_slots(vec![anga]),
            ..Default::default()
        }
    }

    #[test]
    fn a_kusmad_sanctions_an_akusmiya_roots_atmanepada() {
        let mut p = pada_dhatu("cit", &[Tag::Curadi, Tag::Akusmiya], Pada::Atmanepada);
        assert!((rule("10.0496").apply)(&mut p));
        assert!(!p.blocked);
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["10.0496"]);
        assert_eq!(p.terms[ANGA].text, "cit", "a sanction, not an operation");
    }

    #[test]
    fn a_kusmad_blocks_an_akusmiya_roots_parasmaipada() {
        let mut p = pada_dhatu("cit", &[Tag::Curadi, Tag::Akusmiya], Pada::Parasmaipada);
        assert!(!(rule("10.0496").apply)(&mut p));
        assert!(p.blocked);
        assert!(p.log.is_empty());
    }

    #[test]
    fn a_kusmad_declines_without_the_akusmiya_licence() {
        // √cur (1.3.74's), √rudh (1.3.72's), √bhū (1.3.78's) and √as
        // (1.3.12's) are left alone in both padas: not recorded, not blocked.
        for tags in [
            &[Tag::Curadi, Tag::Nic][..],
            &[Tag::Ubhayapadin][..],
            &[][..],
            &[Tag::Atmanepadin][..],
        ] {
            for pada in [Pada::Parasmaipada, Pada::Atmanepada] {
                let mut p = pada_dhatu("x", tags, pada);
                assert!(!(rule("10.0496").apply)(&mut p), "{tags:?} {pada:?}");
                assert!(!p.blocked, "{tags:?} {pada:?}");
                assert!(p.log.is_empty(), "{tags:?} {pada:?}");
            }
        }
    }
```

`pada_dhatu` names `Pada`, which the test module reaches through `use super::*` once Step 4 imports it at the top of the file; until then the module does not compile, which is the expected failure.

- [ ] **Step 2: Write the failing samjna test**

In `samjna.rs`'s `mod tests`, immediately above `#[test]` / `fn pada_sutras_are_order_independent()`, add:

```rust
    /// `pada_prakriya` for an ākusmīya root, hand-built as `nic_prakriya`
    /// is: √cit, tagged as `super::derive` tags a `PadaAssignment::Akusmiya`
    /// row, without running the sanādi stage.
    fn akusmiya_prakriya(pada: Pada) -> Prakriya {
        let mut t = Term::new("cit");
        t.add(Tag::Dhatu);
        t.add(Tag::Curadi);
        t.add(Tag::Akusmiya);
        let mut p = Prakriya {
            ctx: Context::new(Lakara::Lat, pada, Purusha::Prathama, Vacana::Eka),
            ..Default::default()
        };
        p.terms = with_slots(vec![t]);
        p
    }

    #[test]
    fn every_pada_sutra_leaves_an_akusmiya_root_to_10_0496() {
        // 10.0496 has sanctioned an ākusmīya root's ātmanepada in
        // `super::sanadi`, so no pada sūtra of this stage may record on it
        // or block it: 1.3.12, 1.3.66, 1.3.72 and 1.3.74 for want of their
        // tags, 1.3.78 by declining on `Tag::Akusmiya`. Parasmaipada is not
        // tested here: 10.0496 has already blocked that branch, and the
        // controller runs no later rule on a blocked branch.
        for id in ["1.3.12", "1.3.66", "1.3.72", "1.3.74", "1.3.78"] {
            let rule = SAMJNA.iter().find(|r| r.id == id).unwrap();
            let mut p = akusmiya_prakriya(Pada::Atmanepada);
            assert!(!(rule.apply)(&mut p), "{id} fired on cit Atmanepada");
            assert!(!p.blocked, "{id} blocked cit Atmanepada");
            assert!(p.log.is_empty(), "{id} recorded on cit Atmanepada");
        }
    }

```

- [ ] **Step 3: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | grep -E "error\[|FAILED|panicked" | head`
Expected: a compile error in `sanadi.rs`'s tests — `Pada` is not in scope until Step 4 imports it. (In the prototype, with the import but without the rule and 1.3.78's arm, the three `a_kusmad_*` tests panicked on `unwrap()` and `every_pada_sutra_leaves_…` failed on `1.3.78 blocked cit Atmanepada`.)

- [ ] **Step 4: Add the rule**

In `sanadi.rs`, replace the module doc's opening and closing paragraphs

```rust
//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, ṇic's
//! it-lopa (1.3.9), 3.4.114, 7.2.116, 7.3.86, 3.1.32.
```

```rust
//! Every rule self-guards: 3.1.25 on `Tag::Curadi`, the rest on ṇic being
//! present. For gaṇas 1–9 the stage adds nothing and records nothing.
```

with

```rust
//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, ṇic's
//! it-lopa (1.3.9), 3.4.114, 7.2.116, 7.3.86, 3.1.32 — opened by the
//! dhātupāṭha gaṇasūtra 10.0496, which settles an ākusmīya root's pada
//! before ṇic is added.
```

```rust
//! Every rule self-guards: 10.0496 on `Tag::Akusmiya`, 3.1.25 on
//! `Tag::Curadi`, the rest on ṇic being present. For gaṇas 1–9 the stage
//! adds nothing and records nothing.
```

After `use crate::tinanta::terms::{ANGA, NIC};` add `use panini_data::Pada;`. Then, immediately after `pub(crate) static SANADI: &[Rule] = &[`, insert as the first entry:

```rust
    // 10.0496 ā kusmād ātmanepadinaḥ: the curādi roots up to kusm are
    // ātmanepadī — the data layer's `AKUSMIYA` range, carried here as
    // `Tag::Akusmiya`. A gaṇasūtra of the dhātupāṭha, not a sūtra of the
    // Aṣṭādhyāyī, and the first such id in this engine: numbered as
    // vidyut-prakriya numbers it (`DP("10.0496")`), by its position in the
    // dhātupāṭha. Rule ids are opaque strings everywhere they are read.
    //
    // First in the stage because vidyut credits it there, as soon as the
    // dhātu is identified and before 3.1.25. It settles the pada outright, so
    // no pada sūtra in `super::samjna` is credited after it: 1.3.12 and
    // 1.3.74 decline on their own guards, 1.3.78 on this tag. The wrong pada
    // BLOCKS, as it does under 1.3.12 — derivation, not the analyzer, is the
    // source of truth for pada.
    Rule {
        id: "10.0496",
        name: "A kusmAd AtmanepadinaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Akusmiya) {
                return false;
            }
            match p.ctx.pada {
                Pada::Atmanepada => {
                    let before = p.snapshot();
                    p.record("10.0496", "A kusmAd AtmanepadinaH", before);
                    true
                }
                Pada::Parasmaipada => {
                    p.blocked = true;
                    false
                }
            }
        },
    },
```

- [ ] **Step 5: 1.3.78's decline, and the samjna comments**

In `samjna.rs`, after the module doc's first paragraph (`//! 1.3.12, 1.3.66, 1.3.72, 1.3.74, 1.3.78, 3.4.78, 1.3.9, 1.2.4.`), insert:

```rust
//!
//! One pada sanction is settled before this stage: the dhātupāṭha gaṇasūtra
//! 10.0496 *ā kusmād ātmanepadinaḥ*, first in `super::sanadi`, for the
//! ākusmīya curādi roots (`Tag::Akusmiya`). Every pada sūtra here leaves
//! such a root alone.
```

In 1.3.74's comment, replace `    // layer's PadaAssignment::Nic, which every curated curādi row carries.` with

```rust
    // layer's PadaAssignment::Nic, which every curated curādi row outside the
    // ākusmīya carries (those are 10.0496's, `Tag::Akusmiya`, and never reach
    // here).
```

In 1.3.78's ātmanepada arm, replace

```rust
                // they split on ctx.pada: 1.3.72 (Ubhayapadin), 1.3.66 (Anavane) or
                // 1.3.74 (Nic) has already sanctioned this cell, so decline instead of
                // blocking. Only the genuine śeṣa (no pada tag at all) blocks here.
                Pada::Atmanepada => {
                    if p.terms[ANGA].has(Tag::Ubhayapadin)
                        || p.terms[ANGA].has(Tag::Anavane)
                        || p.terms[ANGA].has(Tag::Nic)
                    {
```

with

```rust
                // they split on ctx.pada: 1.3.72 (Ubhayapadin), 1.3.66 (Anavane),
                // 1.3.74 (Nic) or the gaṇasūtra 10.0496 (Akusmiya, in `super::sanadi`)
                // has already sanctioned this cell, so decline instead of blocking.
                // Only the genuine śeṣa (no pada tag at all) blocks here. An
                // Akusmiya root never reaches the parasmaipada arm above: 10.0496
                // has already blocked that branch.
                Pada::Atmanepada => {
                    if p.terms[ANGA].has(Tag::Ubhayapadin)
                        || p.terms[ANGA].has(Tag::Anavane)
                        || p.terms[ANGA].has(Tag::Nic)
                        || p.terms[ANGA].has(Tag::Akusmiya)
                    {
```

- [ ] **Step 6: Pin the order**

In `derivation_tests.rs`'s `tinanta_rule_order_is_pinned`, the `expected` array opens `"3.1.25", "1.3.9", …`. Insert `"10.0496", ` before that first `"3.1.25"`; `mise run fmt` rewraps the array. Everything else is unchanged.

In the test's doc comment, after the paragraph ending `` /// (`GUNA`, `SAMJNA`, …), not `rules()`, or it finds the sanādi entry first. `` and its `///` blank line, insert:

```rust
/// Slice 10b puts the dhātupāṭha gaṇasūtra 10.0496 *ā kusmād
/// ātmanepadinaḥ* at the very top, ahead of 3.1.25, where vidyut-prakriya
/// credits it: it settles an ākusmīya root's pada before ṇic exists. It is
/// the only id here that is not an Aṣṭādhyāyī sūtra.
///
```

- [ ] **Step 7: Run the unit tests and the full suite**

```bash
mise exec -- cargo test -p panini-prakriya 2>&1 | grep -E "FAILED|test result" | head -2   # 403 passed (399 + 4)
mise run test 2>&1 | grep -E "FAILED|test result" | head                                   # priors unchanged at 4932
```

No curated row carries `Tag::Akusmiya` yet, so the rule is inert on every cell.

- [ ] **Step 8: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/
git commit -m "feat(prakriya): 10.0496 ā kusmād ātmanepadinaḥ — the ākusmīya pada, first in sanadi

The engine's first non-Aṣṭādhyāyī rule id. Sanctions ātmanepada, blocks
parasmaipada; 1.3.78 declines on Tag::Akusmiya."
```

---

## Task 4: The rows and their paradigm goldens

This task turns the 144 new cells green.

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini/tests/paradigm/data/curadi.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`

**Interfaces:**
- Consumes: Tasks 2–3.
- Produces: the `dhatus()` rows `10.0192` (`cit`), `10.0228` (`vfz`), `10.0229` (`mad`), `10.0236` (`kusm`), each `Gana::Curadi`, `PadaAssignment::Akusmiya`. Task 5 looks them up by number.

- [ ] **Step 1: Split the pada-marker arm (the failing test comes with Step 2)**

In `curated_pada_agrees_with_upadesha_markers`, replace the whole curādi block

```rust
            // 1.3.74 ṇicaś ca: a curādi root's ātmanepada comes from the
            // affix 3.1.25 adds, not from any marker of its own, so its
            // upadeśa CORRECTLY derives parasmaipada and the curated column
            // says `Nic`. Asserted both ways, like √bhuj above. A curādi row
            // that DOES carry a marker (the ātmanepadī ākusmīya roots) is a
            // later slice's, and will fail here until that slice decides how
            // 1.3.12 and 1.3.74 meet.
            if d.gana == Gana::Curadi {
                assert_eq!(
                    derived,
                    PadaAssignment::Parasmaipada,
                    "{} {upadesha}: a marked curādi row needs its own pada decision",
                    d.dhatupatha
                );
                assert_eq!(
                    d.pada,
                    PadaAssignment::Nic,
                    "{} is curādi; its pada is 1.3.74's, curated as Nic",
                    d.dhatupatha
                );
                continue;
            }
```

with

```rust
            // Curādi: no curated row's pada comes from a marker of its own,
            // so every upadeśa CORRECTLY derives parasmaipada, and the curated
            // column names the sanction instead. Inside `AKUSMIYA` it is the
            // gaṇasūtra 10.0496 ā kusmād ātmanepadinaḥ (`Akusmiya`,
            // ātmanepada only); outside it, the affix's 1.3.74 ṇicaś ca
            // (`Nic`, both padas). Asserted both ways, like √bhuj above, so a
            // row on the wrong side of the range boundary fails here. A
            // curādi row that DOES carry a marker (`10.0058 zmiN`, ṅit) is a
            // later slice's, and fails the first assertion until that slice
            // decides how 1.3.12 meets ṇic.
            if d.gana == Gana::Curadi {
                assert_eq!(
                    derived,
                    PadaAssignment::Parasmaipada,
                    "{} {upadesha}: a marked curādi row needs its own pada decision",
                    d.dhatupatha
                );
                let (want, why) = if AKUSMIYA.contains(&d.dhatupatha) {
                    (PadaAssignment::Akusmiya, "ākusmīya, so 10.0496's")
                } else {
                    (PadaAssignment::Nic, "outside the ākusmīya, so 1.3.74's")
                };
                assert_eq!(d.pada, want, "{} is curādi and {why}", d.dhatupatha);
                continue;
            }
```

This is the only test holding membership to the range. The spec described a separate "iff" test; this arm already asserts it both ways over every curated curādi row, so a second test would duplicate it.

- [ ] **Step 2: Update the gaṇa-row test**

Replace `curadi_rows_are_the_four_curated_roots` (its header comment and expected vector) so it reads:

```rust
    #[test]
    fn curadi_rows_are_the_eight_curated_roots() {
        // Slice 10a opens gaṇa 10 with four roots that need only ṇic
        // (3.1.25), 3.1.32 and the guṇa/vṛddhi before ṇic: √cur (7.3.86),
        // √laḍ (7.2.116), √bhakṣ and √bhūṣ (neither). None carries a pada
        // marker; all four are ubhayapadī by 1.3.74 ṇicaś ca. Slice 10b adds
        // four ākusmīya roots, ātmanepadī by 10.0496 ā kusmād ātmanepadinaḥ:
        // √cit and √vṛṣ (7.3.86), √mad (7.2.116), √kusm (neither). The gaṇa
        // is OPEN at 8 of its 509 dhātupāṭha rows.
        let rows: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.gana == Gana::Curadi)
            .map(|d| (d.dhatupatha, d.code, d.pada))
            .collect();
        assert_eq!(
            rows,
            vec![
                ("10.0001", "cur", PadaAssignment::Nic),
                ("10.0010", "laq", PadaAssignment::Nic),
                ("10.0033", "Bakz", PadaAssignment::Nic),
                ("10.0255", "BUz", PadaAssignment::Nic),
                ("10.0192", "cit", PadaAssignment::Akusmiya),
                ("10.0228", "vfz", PadaAssignment::Akusmiya),
                ("10.0229", "mad", PadaAssignment::Akusmiya),
                ("10.0236", "kusm", PadaAssignment::Akusmiya),
            ]
        );
    }
```

And in `curated_roots_have_expected_ganas_and_padas`: `assert_eq!(dhatus().len(), 107);` → `111`.

Run: `mise exec -- cargo test -p panini-data 2>&1 | grep -E "FAILED|panicked" | head`
Expected: FAIL — `curadi_rows_are_the_eight_curated_roots` (four rows missing) and `curated_roots_have_expected_ganas_and_padas` (107 ≠ 111).

- [ ] **Step 3: Add the four `Dhatu` rows**

Append these to the end of `DHATUS`, after the `10.0255` (`BUz`) row and before the closing `];` that precedes `pub fn dhatus()`. Upadeśa and artha are verbatim from `data/dhatupatha.tsv` (`10.0192 cita~ saYcetane`, `10.0228 vfza~ SaktibanDane`, `10.0229 mada~ tfptiyoge`, `10.0236 kusma~ kutsitasmaye`).

```rust
    Dhatu {
        // 10.0192 `cita~` saYcetane (√cit). The first ākusmīya row: 7.3.86
        // guṇates the laghu upadhā before ṇic (cet-i). Ātmanepadī by 10.0496
        // ā kusmād ātmanepadinaḥ. Slice 10b.
        dhatupatha: "10.0192",
        code: "cit",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "saYcetane",
    },
    Dhatu {
        // 10.0228 `vfza~` SaktibanDane (√vṛṣ). 7.3.86's ṛ arm before ṇic: the
        // laghu ṛ upadhā takes guṇa `ar` (varz-i). Ātmanepadī by 10.0496.
        // Slice 10b.
        dhatupatha: "10.0228",
        code: "vfz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "SaktibanDane",
    },
    Dhatu {
        // 10.0229 `mada~` tfptiyoge (√mad). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (mAd-i). Ātmanepadī by 10.0496. Slice
        // 10b.
        dhatupatha: "10.0229",
        code: "mad",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "tfptiyoge",
    },
    Dhatu {
        // 10.0236 `kusma~` kutsitasmaye (√kusm). The last ākusmīya row, the
        // one 10.0496 names: guru upadhā (the conjunct `sm`), so unchanged
        // before ṇic. Ātmanepadī by 10.0496. Slice 10b.
        dhatupatha: "10.0236",
        code: "kusm",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "kutsitasmaye",
    },
```

Counts in the same file:
- `Dhatu`'s `pada` doc: replace

```rust
    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 107
    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and
    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed
    /// exception and the four curādi rows' are 1.3.74's, each asserted
    /// explicitly from both sides, the same way
```

  with

```rust
    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 111
    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and
    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed
    /// exception, four curādi rows' are 1.3.74's and four ākusmīya rows' are
    /// the gaṇasūtra 10.0496's, each asserted explicitly from both sides, the
    /// same way
```

  and `The test covers the 107 roots curated here` → `The test covers the 111 roots curated here`.
- `pada_from_upadesha`'s doc: `66 of the 107 curated roots` → `66 of the 111 curated roots`. None of the four upadeśas carries a `\`, so 66 and 45 stand.

- [ ] **Step 4: Run the data tests**

Run: `mise exec -- cargo test -p panini-data 2>&1 | grep -E "FAILED|test result" | head -2`
Expected: PASS (22 tests), including `dhatupatha_numbers_resolve_upstream` (all four resolve uniquely) and `curated_pada_agrees_with_upadesha_markers` (all four in range, all four `Akusmiya`).

- [ ] **Step 5: Add the golden rows**

In `crates/panini/tests/paradigm/data/curadi.rs`, append these sixteen rows to the end of `PARADIGM`, after the last `10.0255` row and before its closing `];`. `ALTERNATES` is unchanged: no cell forks. `mise run fmt` rewraps them.

```rust
    ("10.0192", "laT", Pada::Atmanepada, ["cetayate", "cetayete", "cetayante", "cetayase", "cetayeTe", "cetayaDve", "cetaye", "cetayAvahe", "cetayAmahe"]),
    ("10.0192", "laN", Pada::Atmanepada, ["acetayata", "acetayetAm", "acetayanta", "acetayaTAH", "acetayeTAm", "acetayaDvam", "acetaye", "acetayAvahi", "acetayAmahi"]),
    ("10.0192", "loT", Pada::Atmanepada, ["cetayatAm", "cetayetAm", "cetayantAm", "cetayasva", "cetayeTAm", "cetayaDvam", "cetayE", "cetayAvahE", "cetayAmahE"]),
    ("10.0192", "viDiliN", Pada::Atmanepada, ["cetayeta", "cetayeyAtAm", "cetayeran", "cetayeTAH", "cetayeyATAm", "cetayeDvam", "cetayeya", "cetayevahi", "cetayemahi"]),
    ("10.0228", "laT", Pada::Atmanepada, ["varzayate", "varzayete", "varzayante", "varzayase", "varzayeTe", "varzayaDve", "varzaye", "varzayAvahe", "varzayAmahe"]),
    ("10.0228", "laN", Pada::Atmanepada, ["avarzayata", "avarzayetAm", "avarzayanta", "avarzayaTAH", "avarzayeTAm", "avarzayaDvam", "avarzaye", "avarzayAvahi", "avarzayAmahi"]),
    ("10.0228", "loT", Pada::Atmanepada, ["varzayatAm", "varzayetAm", "varzayantAm", "varzayasva", "varzayeTAm", "varzayaDvam", "varzayE", "varzayAvahE", "varzayAmahE"]),
    ("10.0228", "viDiliN", Pada::Atmanepada, ["varzayeta", "varzayeyAtAm", "varzayeran", "varzayeTAH", "varzayeyATAm", "varzayeDvam", "varzayeya", "varzayevahi", "varzayemahi"]),
    ("10.0229", "laT", Pada::Atmanepada, ["mAdayate", "mAdayete", "mAdayante", "mAdayase", "mAdayeTe", "mAdayaDve", "mAdaye", "mAdayAvahe", "mAdayAmahe"]),
    ("10.0229", "laN", Pada::Atmanepada, ["amAdayata", "amAdayetAm", "amAdayanta", "amAdayaTAH", "amAdayeTAm", "amAdayaDvam", "amAdaye", "amAdayAvahi", "amAdayAmahi"]),
    ("10.0229", "loT", Pada::Atmanepada, ["mAdayatAm", "mAdayetAm", "mAdayantAm", "mAdayasva", "mAdayeTAm", "mAdayaDvam", "mAdayE", "mAdayAvahE", "mAdayAmahE"]),
    ("10.0229", "viDiliN", Pada::Atmanepada, ["mAdayeta", "mAdayeyAtAm", "mAdayeran", "mAdayeTAH", "mAdayeyATAm", "mAdayeDvam", "mAdayeya", "mAdayevahi", "mAdayemahi"]),
    ("10.0236", "laT", Pada::Atmanepada, ["kusmayate", "kusmayete", "kusmayante", "kusmayase", "kusmayeTe", "kusmayaDve", "kusmaye", "kusmayAvahe", "kusmayAmahe"]),
    ("10.0236", "laN", Pada::Atmanepada, ["akusmayata", "akusmayetAm", "akusmayanta", "akusmayaTAH", "akusmayeTAm", "akusmayaDvam", "akusmaye", "akusmayAvahi", "akusmayAmahi"]),
    ("10.0236", "loT", Pada::Atmanepada, ["kusmayatAm", "kusmayetAm", "kusmayantAm", "kusmayasva", "kusmayeTAm", "kusmayaDvam", "kusmayE", "kusmayAvahE", "kusmayAmahE"]),
    ("10.0236", "viDiliN", Pada::Atmanepada, ["kusmayeta", "kusmayeyAtAm", "kusmayeran", "kusmayeTAH", "kusmayeyATAm", "kusmayeDvam", "kusmayeya", "kusmayevahi", "kusmayemahi"]),
```

There are no parasmaipada rows: `padas()` gives `Akusmiya` only ātmanepada, so `paradigm_covers_every_enumerable_cell` enumerates none.

- [ ] **Step 6: Update `paradigm/main.rs`'s counts**

In `derivation_set_shape_matches_the_audited_numbers`:
- `assert_eq!(total_cells, 4932, "548 root×lakāra blocks × 9 cells each");` → `assert_eq!(total_cells, 5076, "564 root×lakāra blocks × 9 cells each");`
- `assert_eq!(ones, 4108, "one-form cells");` → `4252`. `twos` … `sevens`, `ALTERNATES.len()` (1130) and every `key_count` are unchanged.

Check: 4252 + 612 + 165 + 19 + 10 + 17 + 1 = 5076 cells; 5076 + 1130 = 6206 forms.

In the doc comment above that test:
- `/// 4932 cells total (548 root×lakāra blocks × 9), of which 4108 hold exactly one form,` → `/// 5076 cells total (564 root×lakāra blocks × 9), of which 4252 hold exactly one form,`
- After the slice-10a paragraph (ending `` /// `7.3.86+7.1.35+8.4.56`. Twenty-four new rows. The gaṇa is OPEN at 4 of `` / `/// its 509 rows.`), add:

```rust
///
/// Slice 10b curates four ākusmīya roots, √cit, √vṛṣ, √mad and √kusm
/// (`10.0192`, `10.0228`, `10.0229`, `10.0236`), ātmanepadī by the
/// dhātupāṭha gaṇasūtra 10.0496 *ā kusmād ātmanepadinaḥ*. Ātmanepada only,
/// and the thematic ātmanepada paradigm forks nowhere, so all 144 new cells
/// hold one form. No new rows. The gaṇa is OPEN at 8 of its 509 rows.
```

`pada_ambiguous_surfaces_are_exactly_these` needs no change: the four roots are ātmanepada-only and none of their surfaces equals any other cell's (the prototype's run).

- [ ] **Step 7: Add the `check()` test**

First run

```bash
grep -rn '"cetayate"\|"avarzayata"\|"mAdayaDvam"\|"kusmayeta"\|"cetayati"\|"varzayati"\|"amAdayat"\|"kusmayatu"' crates/panini/tests/paradigm/data/ | grep -v curadi.rs
```

It must print nothing. Then, at the end of `paradigm/main.rs`, add:

```rust

/// Slice 10b's ākusmīya rows through `check`. None of these surfaces is a
/// prior row's, so every analysis must name the ākusmīya root, be
/// ātmanepada, and credit the gaṇasūtra 10.0496 with no pada sūtra beside
/// it. The parasmaipada shapes (`cetayati` …) derive nothing: 10.0496 blocks
/// every parasmaipada cell, and `check` must not report a blocked branch.
#[test]
fn curadi_analyses_its_akusmiya_forms() {
    let engine = Panini::new();
    for (form, dhatu) in [
        ("cetayate", "cit"),
        ("avarzayata", "vfz"),
        ("mAdayaDvam", "mad"),
        ("kusmayeta", "kusm"),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert!(!r.analyses.is_empty(), "{form}");
        for a in &r.analyses {
            assert_eq!(a.dhatu, dhatu, "{form}");
            assert_eq!(a.pada, Pada::Atmanepada, "{form}");
            let ids: Vec<&str> = a.trace.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids[0], "10.0496", "{form}: {ids:?}");
            for absent in ["1.3.12", "1.3.74", "1.3.78"] {
                assert!(!ids.contains(&absent), "{form} {absent}: {ids:?}");
            }
        }
    }
    for form in ["cetayati", "varzayati", "amAdayat", "kusmayatu"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
```

- [ ] **Step 8: Run the full suite**

Run: `mise run test 2>&1 | grep -E "FAILED|panicked|test result" | head -20` (foreground, timeout 600000 ms)
Expected: PASS at 5076 cells, `paradigm` at 21 tests, with the 4932 priors unchanged.

If an ākusmīya cell fails, read it against the spec before touching anything, using superpowers:systematic-debugging. **Do not edit a golden to match the engine.** Likely causes by symptom:
- every ākusmīya cell missing from the derivation set (blocked): 1.3.78's `Tag::Akusmiya` decline, or `derive`'s `Akusmiya` arm.
- `citayate` / `vfzayate` (no guṇa): the sanādi 7.3.86 declined — it is unchanged by this slice, so look at the row's `code`.
- `madayate`: 7.2.116 declined — same.
- a `check()` trace whose first step is `3.1.25`: 10.0496 is not first in `SANADI`.

- [ ] **Step 9: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-data/src/lib.rs crates/panini/tests/paradigm/
git commit -m "feat(data): curādi's ākusmīya — √cit, √vṛṣ, √mad, √kusm (10.0192/0228/0229/0236), ātmanepadī by 10.0496

4932 → 5076 cells, 6062 → 6206 forms, ALTERNATES unchanged at 1130, 107 → 111 roots."
```

---

## Task 5: The trace pins and the fires-only tests

**Files:**
- Modify: `crates/panini/tests/trace/curadi.rs`

**Interfaces:**
- Consumes: `crate::helpers::{at, cell_trace, credited}` (existing). `cell_trace(number, lakara, pada, purusha, vacana) -> (String, Vec<String>)` returns branch 0's text and trace. `credited(sutra) -> Vec<(&'static str, Gana)>` lists one entry per non-blocked branch, over every curated cell, whose log carries `sutra`. `panini_prakriya::derive(&Dhatu, Lakara, Pada, Purusha, Vacana) -> Vec<Prakriya>`.

- [ ] **Step 1: Extend the module header**

In `crates/panini/tests/trace/curadi.rs`, replace

```rust
//! 3.1.32 — before the pada sūtra, where every other gaṇa's trace opens.

use crate::helpers::{at, cell_trace, credited};
use panini_data::{Lakara, Pada, Purusha, Vacana};
```

with

```rust
//! 3.1.32 — before the pada sūtra, where every other gaṇa's trace opens.
//! An ākusmīya root's trace opens one step earlier, with the gaṇasūtra
//! 10.0496, and has no pada sūtra at all.

use crate::helpers::{at, cell_trace, credited};
use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};
use panini_prakriya::derive;
```

- [ ] **Step 2: Add the tests**

Append to the end of the file:

```rust

#[test]
fn cetayate_trace_opens_with_a_kusmad_and_credits_no_pada_sutra() {
    // cit A laT P.E. 10.0496 settles the pada before ṇic exists; 7.3.86
    // guṇates `cit` before ṇic, 3.1.32 makes `ceti` the dhātu, and 3.4.78
    // follows 3.1.32 directly — no 1.3.12, 1.3.74 or 1.3.78 between them.
    let (text, t) = cell_trace(
        "10.0192",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "cetayate", "got {t:?}");
    assert_eq!(
        t,
        [
            "10.0496", "3.1.25", "1.3.9", "3.4.114", "7.3.86", "3.1.32", "3.4.78", "1.2.4",
            "3.4.79", "3.1.68", "1.3.9", "7.3.84", "6.1.78",
        ],
    );
}

#[test]
#[allow(non_snake_case)]
fn varzayate_mAdayate_and_kusmayate_take_their_pre_nic_change_or_none() {
    // √vṛṣ: 7.3.86's ṛ arm (`vfz` → `varz`). √mad: 7.2.116, and no 7.3.86
    // (`a` is not ik). √kusm: guru upadhā, neither. Each opens with 10.0496.
    for (number, form, present, absent) in [
        ("10.0228", "varzayate", &["7.3.86"][..], &["7.2.116"][..]),
        ("10.0229", "mAdayate", &["7.2.116"][..], &["7.3.86"][..]),
        ("10.0236", "kusmayate", &[][..], &["7.2.116", "7.3.86"][..]),
    ] {
        let (text, t) = cell_trace(
            number,
            Lakara::Lat,
            Pada::Atmanepada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert_eq!(text, form, "got {t:?}");
        assert_eq!(t[0], "10.0496", "{form}: got {t:?}");
        for sutra in present {
            assert!(at(&t, "3.4.114") < at(&t, sutra), "{form}: got {t:?}");
            assert!(at(&t, sutra) < at(&t, "3.1.32"), "{form}: got {t:?}");
        }
        for sutra in absent {
            assert!(!t.contains(&sutra.to_string()), "{form} {sutra}: got {t:?}");
        }
    }
}

#[test]
fn an_akusmiya_roots_parasmaipada_is_blocked_by_a_kusmad_alone() {
    // Every parasmaipada cell of the four rows derives only blocked
    // branches, and the block is 10.0496's: nothing is recorded, so no later
    // rule ran on the branch.
    for number in ["10.0192", "10.0228", "10.0229", "10.0236"] {
        let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
        for lakara in [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin] {
            let ps = derive(
                d,
                lakara,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            );
            assert!(!ps.is_empty(), "{number} {lakara:?}");
            for p in &ps {
                assert!(p.blocked, "{number} {lakara:?}: {}", p.text());
                assert!(p.log.is_empty(), "{number} {lakara:?}: {:?}", p.log);
            }
        }
    }
}

#[test]
fn a_kusmad_is_credited_on_exactly_the_akusmiya_cells() {
    // 10.0496 fires on every ātmanepada cell of the four ākusmīya rows —
    // 4 roots × 4 lakāras × 9 cells, one branch each — and nowhere else. And
    // 1.3.74 never reaches them: its credits stay on the four `Nic` rows.
    let hits = credited("10.0496");
    assert_eq!(hits.len(), 144);
    for (number, _) in &hits {
        assert!(
            ["10.0192", "10.0228", "10.0229", "10.0236"].contains(number),
            "10.0496 credited on {number}"
        );
    }
    for (number, _) in credited("1.3.74") {
        assert!(
            ["10.0001", "10.0010", "10.0033", "10.0255"].contains(&number),
            "1.3.74 credited on {number}"
        );
    }
}
```

The *cetayate* pin is the prototype engine's order. vidyut's trace for the same cell credits the same rules in the same relative order, plus it-saṁjñā and 1.4.13/1.4.14 steps this engine does not record.

- [ ] **Step 3: Run the suite**

Run: `mise run test 2>&1 | grep -E "FAILED|panicked|test result" | head -20` (foreground, timeout 600000 ms)
Expected: PASS (`trace` at 196 tests).
- If a pin fails on ORDER, report the engine's order; do not "fix" it toward vidyut's.
- If `credited("10.0496")` is not 144, or names another row, stop and report it: that is the rule over- or under-firing.

- [ ] **Step 4: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini/tests/trace/curadi.rs
git commit -m "test(trace): 10b pins — cetayate, varzayate/mAdayate/kusmayate; parasmaipada blocked by 10.0496 alone; 10.0496 on exactly the 144 ākusmīya cells"
```

---

## Task 6: Audit, prior-trace diff, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (audit prose), `crates/panini-prakriya/src/tinanta/guna.rs` (one comment), `docs/superpowers/specs/2026-10-01-curadi-gana-10a-design.md`

**Interfaces:**
- Consumes: the finished engine and goldens (Tasks 2–5). Produces no symbols.

- [ ] **Step 1: Update the audit harness**

In `tools/audit/panini_full_audit.rs` (each `old` occurs once):
- `//! What it compares: for each of the 107 curated roots,` → `111`.
- `//! Corpus invariants, asserted: 107 roots, 4932 cells, 6062 forms.` → `111 roots, 5076 cells, 6206 forms.`
- ``(`derivation_set_shape_matches_the_audited_numbers`): 548 root×pada×lakāra`` → `564`.
- `//! Optionally dump the full 4932-cell table:` → `5076-cell`.
- `assert_eq!(roots_seen.len(), 107, "curated roots");` → `111`.
- `assert_eq!(n_cells, 4932, "cells: 548 root×pada×lakāra blocks × 9");` → `assert_eq!(n_cells, 5076, "cells: 564 root×pada×lakāra blocks × 9");`
- `assert_eq!(n_forms, 6062, "forms: 4932 cells + 1130 ALTERNATES rows");` → `assert_eq!(n_forms, 6206, "forms: 5076 cells + 1130 ALTERNATES rows");`

The both-pada clause (thirty roots) is unchanged: the four new roots are ātmanepada-only. `gana_name` already has `Curadi`.

In `tools/audit/README.md`, `(107 roots, 4932 cells, 6062 forms)` → `(111 roots, 5076 cells, 6206 forms)`.

- [ ] **Step 2: Repoint vidyut's dev-deps at THIS worktree, run the prior-trace diff, the parasmaipada probe and the audit**

`/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes absolute dev-dep paths to `/workspace/crates`, the `main` checkout, which does not have this slice. Auditing without repointing checks the pre-slice engine and passes vacuously.

Two throwaway examples exist from the plan's prototype: `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_10b.rs` and `curadi_golden_10b.rs`. If either is missing, recreate it:

```rust
//! THROWAWAY: slice 10b — dump every prior cell's live-branch credited-rule log.
use panini::Panini;
use panini_data::{Lakara as L, Purusha as P, Vacana as V};
const NEW: [&str; 4] = ["10.0192", "10.0228", "10.0229", "10.0236"];
fn main() {
    let panini = Panini::new();
    for d in panini_data::dhatus() {
        if NEW.contains(&d.dhatupatha) { continue; }
        for pada in d.pada.padas() {
            for l in [L::Lat, L::Lan, L::Lot, L::VidhiLin] { for pu in [P::Prathama, P::Madhyama, P::Uttama] { for va in [V::Eka, V::Dvi, V::Bahu] {
                for b in panini.derive(d, l, *pada, pu, va).iter().filter(|b| !b.blocked) {
                    let ids: Vec<&str> = b.log.iter().map(|s| s.sutra.as_str()).collect();
                    println!("{} {:?} {:?} {:?} {:?} {} : {}", d.dhatupatha, pada, l, pu, va, b.text(), ids.join(" "));
                }
            }}}
        }
    }
}
```

```rust
//! THROWAWAY: slice 10b — check the four ākusmīya rows against vidyut in BOTH
//! padas: ātmanepada sets equal, parasmaipada empty on both sides.
use panini::Panini;
use panini_data::{dhatus, Lakara as PL, Pada as PP, Purusha as PPu, Vacana as PV};
use vidyut_prakriya::args::{DhatuPada, Lakara, Prayoga, Purusha, Tinanta, Vacana};
use vidyut_prakriya::{Dhatupatha, Vyakarana};
fn main() {
    let dp = Dhatupatha::from_path("/tmp/vidyut-full/vidyut-prakriya/data/dhatupatha.tsv").unwrap();
    let v = Vyakarana::builder().build();
    let panini = Panini::new();
    let (mut ok, mut extra, mut empty) = (0, 0, 0);
    for num in ["10.0192", "10.0228", "10.0229", "10.0236"] {
        let d = dhatus().iter().find(|d| d.dhatupatha == num).unwrap();
        let up = dp.get(num).unwrap().clone();
        for (pp, vp) in [(PP::Atmanepada, DhatuPada::Atmanepada), (PP::Parasmaipada, DhatuPada::Parasmaipada)] {
            for (pl, vl) in [(PL::Lat, Lakara::Lat), (PL::Lan, Lakara::Lan), (PL::Lot, Lakara::Lot), (PL::VidhiLin, Lakara::VidhiLin)] {
                for (ppu, vpu) in [(PPu::Prathama, Purusha::Prathama), (PPu::Madhyama, Purusha::Madhyama), (PPu::Uttama, Purusha::Uttama)] {
                    for (pva, vva) in [(PV::Eka, Vacana::Eka), (PV::Dvi, Vacana::Dvi), (PV::Bahu, Vacana::Bahu)] {
                        let mut ours: Vec<String> = panini.derive(d, pl, pp, ppu, pva).iter().filter(|b| !b.blocked).map(|b| b.text()).collect();
                        let t = Tinanta::builder().dhatu(up.clone()).prayoga(Prayoga::Kartari).purusha(vpu).vacana(vva).lakara(vl).pada(vp).build().unwrap();
                        let mut theirs: Vec<String> = v.derive_tinantas(&t).iter().map(|p| p.text()).collect();
                        if ours.len() > 1 { extra += 1 }
                        ours.sort(); ours.dedup(); theirs.sort(); theirs.dedup();
                        if pp == PP::Parasmaipada {
                            if ours.is_empty() && theirs.is_empty() { empty += 1 } else { eprintln!("P {num}: {ours:?} vs {theirs:?}") }
                        } else if ours == theirs { ok += 1 } else { eprintln!("A {num}: {ours:?} vs {theirs:?}") }
                    }
                }
            }
        }
    }
    println!("A cells matching vidyut: {ok}/144; multi-form cells: {extra}; P cells empty both sides: {empty}/144");
}
```

```bash
WT="$(git rev-parse --show-toplevel)"
DUMP="$(mktemp -d)"
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # must point at /workspace/crates
(cd /tmp/vidyut-full/vidyut-prakriya && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10b > "$DUMP/main.txt")
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # must point at $WT/crates
(cd /tmp/vidyut-full/vidyut-prakriya && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10b > "$DUMP/branch.txt")
wc -l < "$DUMP/main.txt"                                        # 6062
diff "$DUMP/main.txt" "$DUMP/branch.txt" && echo PRIORS-IDENTICAL
(cd /tmp/vidyut-full/vidyut-prakriya && mise exec rust@1.99.0 -- cargo run -q --release --example curadi_golden_10b 2>/dev/null)
```

Expected: `PRIORS-IDENTICAL`, then `A cells matching vidyut: 144/144; multi-form cells: 0; P cells empty both sides: 144/144`. If the diff shows any line, or the probe prints a `P`/`A` line, stop and report it.

Then the audit. Copy the committed harness; never rewrite it.

```bash
cp tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -15)
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -8)
```

- Expected from the honest run: `AUDIT PASSED: 5076 cells, 6206 forms, zero differences.`
- Expected from the `entry` control: `AUDIT FAILED: 36 differing cells.`

Do not use `mise -C`. If the honest run shows differences, stop and report, and edit nothing. After all runs, restore the dev-deps:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"/workspace/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"/workspace/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
```

- [ ] **Step 3: Record the audit**

In `tools/audit/README.md`, immediately under `## Last recorded result` and its blank line, add a new entry above the 10a one, with `<DATE>` = `date -u +%F`:

```markdown
<DATE>, curādi 10b slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 5076
cells / 6206 forms / 111 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole curādi 10b slice: four roots of the ākusmīya
antargaṇa, √cit, √vṛṣ, √mad and √kusm (`10.0192`, `10.0228`, `10.0229`,
`10.0236`), ātmanepadī by the dhātupāṭha gaṇasūtra 10.0496 *ā kusmād
ātmanepadinaḥ*, which a new first entry of the sanādi stage credits. The
harness compares only the padas a root admits, so it never asks about an
ākusmīya parasmaipada cell; a throwaway probe did, and found all 144 empty
in vidyut and blocked here. A main-vs-branch dump of every prior cell's
traces was byte-identical.

Totals: 111 = 107 + 4; 5076 = 4932 + 144 (16 root×pada×lakāra blocks × 9);
6206 = 6062 + 144, with no new `ALTERNATES` rows (1130), measured via the
harness's corpus block, not assumed.

```

In `crates/panini/tests/paradigm/main.rs`'s audit-chain doc comment (`grep -n "curādi 10a's re-ran" crates/panini/tests/paradigm/main.rs`), replace

```rust
/// 6062 forms / 107 roots with zero differences, its `entry` negative
/// control verified failing (36 √bhū cells). √tṛh
```

with

```rust
/// 6062 forms / 107 roots with zero differences, its `entry` negative
/// control verified failing (36 √bhū cells), and curādi 10b's re-ran it at the
/// same commit over all 5076 cells / 6206 forms / 111 roots with zero
/// differences, its `entry` negative control verified failing (36 √bhū
/// cells). √tṛh
```

- [ ] **Step 4: README.md**

Each `old` occurs exactly once; confirm with `grep -c` before editing.
- Replace

```markdown
8.4.44 *śāt* exemption. *curādi* (10) is **open** at 4 of its 509
dhātupāṭha rows: √cur (`10.0001`, *corayati*), √laḍ (`10.0010`,
*lāḍayati*), √bhakṣ (`10.0033`) and √bhūṣ (`10.0255`), curated in slice 10a,
all ubhayapadī by 1.3.74 *ṇicaś ca*.
```

  with

```markdown
8.4.44 *śāt* exemption. *curādi* (10) is **open** at 8 of its 509
dhātupāṭha rows: √cur (`10.0001`, *corayati*), √laḍ (`10.0010`,
*lāḍayati*), √bhakṣ (`10.0033`) and √bhūṣ (`10.0255`), curated in slice 10a,
all ubhayapadī by 1.3.74 *ṇicaś ca*; and four roots of the ākusmīya
antargaṇa, √cit (`10.0192`, *cetayate*), √vṛṣ (`10.0228`, *varṣayate*), √mad
(`10.0229`, *mādayate*) and √kusm (`10.0236`), curated in slice 10b,
ātmanepadī by the dhātupāṭha's own gaṇasūtra 10.0496 *ā kusmād
ātmanepadinaḥ* — the engine's one rule that is not an Aṣṭādhyāyī sūtra.
```

- `curated 107-root set,` → `curated 111-root set,`
- `824 of the 4932 cells hold more than one form` → `824 of the 5076 cells hold more than one form`

The both-pada list (thirty roots) and the pada-ambiguous list (seventy-two) are unchanged.

- [ ] **Step 5: docs/ARCHITECTURE.md**

Each `old` occurs exactly once.
- Stage table: the `sanadi.rs` row's `| 3.1.25, 1.3.9, 3.4.114, 7.2.116, 7.3.86, 3.1.32 — ṇic and its folding into the dhātu (curādi only) |` → `| 10.0496, 3.1.25, 1.3.9, 3.4.114, 7.2.116, 7.3.86, 3.1.32 — the ākusmīya pada, then ṇic and its folding into the dhātu (curādi only) |`
- `pins all 136 ids verbatim` → `pins all 137 ids verbatim`
- `` `samjna.rs` — 136 total). `` → `` `samjna.rs` — 136 total — then curādi 10b's one: the dhātupāṭha gaṇasūtra `` / ``10.0496 *ā kusmād ātmanepadinaḥ*, first in `sanadi.rs` and the only id that`` / `is not an Aṣṭādhyāyī sūtra — 137 total).`
- `**open** at 4 of its` / `509 rows (√cur, √laḍ, √bhakṣ, √bhūṣ; slice 10a). gaṇa` → `**open** at 8 of its` / `509 rows (√cur, √laḍ, √bhakṣ, √bhūṣ; slice 10a; √cit, √vṛṣ, √mad, √kusm; slice 10b). gaṇa`
- In the pada-sanction paragraph, replace `1.3.78's ātmanepada arm declines rather than blocks for such a root, so both` / `` padas derive. `INVALID` `` with:

```markdown
1.3.78's ātmanepada arm declines rather than blocks for such a root, so both
padas derive. The ākusmīya curādi roots (`Tag::Akusmiya`) settle their pada
earlier, by the dhātupāṭha gaṇasūtra 10.0496 *ā kusmād ātmanepadinaḥ* at the
head of `sanadi.rs`: it sanctions ātmanepada and blocks parasmaipada exactly
as 1.3.12 does, and every pada sūtra in `samjna.rs` then leaves the root
alone. `INVALID`
```

- The tātaṅ paragraph: `the curated set's 25 ātmanepada-only` → `the curated set's 29 ātmanepada-only`; `82 + 25 = the 107 curated roots` → `82 + 29 = the 111 curated roots`. Its 82-root / 164-cell parasmaipada census and the thirty both-pada roots are unchanged.

Check: `grep -c "pada: PadaAssignment::Atmanepada,\|pada: PadaAssignment::Akusmiya," crates/panini-data/src/lib.rs` prints `29`.

- [ ] **Step 6: AGENTS.md**

Each `old` occurs exactly once.
- `` (`crates/panini/tests/paradigm/`, 4932 cells, ten gaṇas, nine complete — `` → `5076 cells`
- `and curādi (10) opened in slice 10a at 4 of its 509 rows (√cur, √laḍ, √bhakṣ,` / `√bhūṣ) —` → `and curādi (10) opened in slice 10a at 4 of its 509 rows (√cur, √laḍ, √bhakṣ,` / `√bhūṣ), at 8 after slice 10b curated the ākusmīya √cit, √vṛṣ, √mad and √kusm —`
- `so 4932 + 1130 = 6062 forms total` → `so 5076 + 1130 = 6206 forms total`
- The audit chain: `` roots), and that by curādi 10a's (`tools/audit/README.md`'s 2026-10-02 entry, 4932 `` / `  cells / 6062 forms / 107 roots).` → keep both lines, replacing the final `roots).` with `` roots), and that by curādi 10b's `` / `` (`tools/audit/README.md`'s <DATE> 10b entry, 5076 cells / 6206 forms / 111 `` / `roots).`
- `not wrong in kind: 4932 goldens` → `not wrong in kind: 5076 goldens`
- The stale-comment ledger: after the sentence ``Curādi 10a touched neither comment either; the corpus stands at 4932 cells as of 10a (`guna.rs:2565`'s claim anchored at `guna.rs:2565`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit).`` insert ``Curādi 10b touched neither comment either; the corpus stands at 5076 cells as of 10b (`guna.rs:2565`'s claim anchored at `guna.rs:<G>`, `controller.rs:206`'s at `controller.rs:<C>`; both lines measured by grep at this commit).`` Measure `<G>` with `grep -n "1872 goldens move" crates/panini-prakriya/src/tinanta/guna.rs` and `<C>` with `grep -n "only 8 cells fire" crates/panini-prakriya/src/controller.rs` (2565 and 206 in the prototype). Measure; never compute.

AGENTS.md's floor paragraph (`measured at 4932 cells`) and the current mutation record are Task 7's.

- [ ] **Step 7: The engine comment and the 10a spec**

`crates/panini-prakriya/src/tinanta/guna.rs`, 6.1.78's comment: `107-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)` → `111-root × 4-lakāra grammar, …`. The claim still holds: the four new aṅgas end in ṇic's `i` (`ceti`, `varzi`, `mAdi`, `kusmi`).

`docs/superpowers/specs/2026-10-01-curadi-gana-10a-design.md`:
- Replace `- ātmanepadī (ākusmīya) roots such as √cit *cetayate* — how 1.3.12 and 1.3.74` / `  meet on one row;` with

```markdown
- ātmanepadī (ākusmīya) roots such as √cit *cetayate* — how 1.3.12 and 1.3.74
  meet on one row (corrected in slice 10b,
  `2026-10-02-curadi-gana-10b-design.md`: the ākusmīya rows carry no marker;
  their ātmanepada is the dhātupāṭha gaṇasūtra 10.0496's, and no pada sūtra
  is credited);
```

- In "Later slices", after `ātmanepadī roots, mit roots, adanta roots, then optional ṇic.` add ` (10b took the` / `ātmanepadī class as the ākusmīya roots and gaṇasūtra 10.0496 — see` / `` `2026-10-02-curadi-gana-10b-design.md`.) ``

- [ ] **Step 8: Sweep for anything left stale**

```bash
grep -rn "4932\|6062\|\b548\b\|4108\|107 roots\|107-root\|of these 107\|of the 107\|107 curated\|136 ids\|136 total)\|4 of 509\|4 of its 509\|25 ātmanepada\|82 + 25\|every curated curādi row carries\|curādi row with no pada marker of its own is$" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.md --include=*.rs
grep -rn "\bcit\b\|\bceti\b\|\bvfz\b\|\bvarzi\b\|\bmad\b\|\bmAdi\b\|\bkusm\b\|\bkusmi\b\|10\.0496\|1\.3\.74\|Akusmiya\|ākusmīya" crates/panini-prakriya/src crates/panini-data/src --include=*.rs | grep "//"
```

Expected residue in the first grep:
- AGENTS.md's floor paragraph (`measured at 4932 cells`), which Task 7 rewrites;
- dated history: AGENTS.md's mutation record and audit chain, `tools/audit/README.md`'s older entries and the 10b entry's own `107 + 4` / `4932 + 144` / `6062 + 144` totals, the audit chain in `paradigm/main.rs`, the 10a paragraphs. Never rewrite history.

For every hit of the second grep, check the comment is still true with 10.0496 in place — in particular any comment presenting 1.3.74 as the sanction of *every* curādi row.

- [ ] **Step 9: Run the full suite and commit**

Run: `mise run test 2>&1 | grep -E "FAILED|test result" | head -20` (foreground, timeout 600000 ms). Expected: PASS at 5076 cells.

```bash
mise run fmt && mise run lint
git add -A
git commit -m "docs: 10b's counts, the audit record, and the sweep

5076 cells / 6206 forms / 111 roots across README, ARCHITECTURE, AGENTS,
paradigm/main.rs and tools/audit; curādi open at 8/509; 10.0496 documented.
Audit at zero divergence against 8da2f90b; prior traces byte-identical to main."
```

---

## Task 7: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and the current-record paragraph); `mise.toml` only if the cap moves

Follow AGENTS.md's cargo-mutants protocol. Hazards from this repo's record:
- **Measure, never scale.**
- **Every invocation rotates `mutants.out`**, so always pass `-o`.
- **The mise shim fails in background shells.** Use the real binary: `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`.
- **`pgrep -f` matches its own shell.** Wait on `pgrep -x cargo-mutants`.
- **This slice edits `sanadi.rs`, `samjna.rs`, `mod.rs`, `term.rs` and one comment line in `guna.rs` (in place, no line shift)**, not `adesha.rs` or `tripadi.rs`. So the three documented non-caught entries (`adesha.rs:589:30`, `tripadi.rs:1289:38`, `tripadi.rs:1602:23`) should not move. Confirm them by `--list`, never by assumption.

- [ ] **Step 1: Measure the floor**

With nothing else running, run this twice: `time mise run test 2>&1 | tail -3` (foreground). Record both wall clocks and `cat /proc/loadavg`. The 4932-cell floor was 10.336s / 9.781s.

- [ ] **Step 2: Locate and probe the two uncaught equivalents at `-j 4`**

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /="
```

Write the positions as `<A>` (adesha), `<T1>` (the tripadi equivalent, inside 8.3.13's `apply`) and `<T2>` (the ṇatva hang). Then run in the foreground with timeout 600000 ms:

```bash
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 150 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:<A>:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" 2>&1 | tail -10
```

Both must be MISSED, not TIMEOUT. Read each test-phase duration from `$SCRATCH/mutants.out/outcomes.json`.

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/curadi-10b"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout 150 -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"
```

The last campaign took about 20 minutes. Run nothing CPU-heavy meanwhile. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

- [ ] **Step 4: Read the outcomes**

When `pgrep -x cargo-mutants` returns nothing:

```bash
date -u +"%F %T UTC" > "$OUT/finished"
cp "$OUT/mutants.out/outcomes.json" "$OUT/outcomes.durable.json"
tail -5 "$OUT/campaign.log"
cat "$OUT/mutants.out/missed.txt" "$OUT/mutants.out/timeout.txt"
```

Expected (exit code 3 is normal when a timeout is present):
- `missed.txt` holds exactly `adesha.rs:<A>:30: replace + with *` and `tripadi.rs:<T1>:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:<T2>:23: replace -= with /=`.
- panini-analyze: 0 missed, 0 timeout.

The prototype's scoped probe already ran every mutant cargo-mutants generates in the new code — `sanadi.rs`'s 10.0496 `delete !` and 1.3.78's three `||` → `&&` (6 in all, counting duplicates) — and all were caught. cargo-mutants generates no arm-deletion mutant for the exhaustive `match` arms in 10.0496, `derive` or `padas()`.

If not:
- Any **other timeout** is a suspect survivor. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant in 10.0496 or 1.3.78 means a Task 3 test does not separate it. Strengthen the named test, commit, and re-run only those mutants with `--re` and a fresh `-o`.

- [ ] **Step 5: Margins**

```bash
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["outcomes"][1])' "$OUT/mutants.out/outcomes.json"
```

Compute the two equivalents' test phases under campaign load, and the caught min/median/p90/max. The margin is 150 ÷ the longest equivalent phase.
- If the margin is ≥ 5, the cap stays 150.
- Otherwise the new cap is 6 × that phase, rounded up to the next 10 s. Change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together.

- [ ] **Step 6: Record it in AGENTS.md**

- Rewrite `**The floor behind the 150s cap, measured at 4932 cells on Rust 1.99.0,` / `2026-10-02.**` and its paragraph with Step 1's and Step 2's numbers at 5076 cells, and the cap Step 5 chose, keeping the comparison chain to earlier floors.
- Replace the `**Current record (curādi 10a, 2026-10-02).**` paragraph with `**Current record (curādi 10b, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**;
  - the non-caught set diffed against 10a's (the clean result is identical);
  - the campaign-load phases and margins;
  - that no 10.0496 or 1.3.78-`Akusmiya` mutant is missed, timed out or unviable-without-reason, with each one's `file:line` and count;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`.

  End it with a pointer to the record it replaces: run `git rev-parse --short HEAD` before committing and write `The curādi 10a record it replaces: \`git show <that hash>:AGENTS.md\`.`

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md mise.toml
git commit -m "chore: 10b mutation gate — floor and uncaught run re-measured at 5076 cells

missed.txt holds only the two documented equivalents and timeout.txt only
the permanent ṇatva-scan entry; every 10.0496 and 1.3.78-Akusmiya mutant is caught."
```

---

## Task 8: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -20
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin curadi-10b
gh pr create --title "curādi 10b — the ākusmīya roots and gaṇasūtra 10.0496" --body "$(cat <<'BODY'
Slice 10b curates four roots of curādi's ākusmīya antargaṇa — √cit
(*cetayate*), √vṛṣ (*varṣayate*), √mad (*mādayate*) and √kusm (*kusmayate*) —
ātmanepadī by the dhātupāṭha gaṇasūtra **10.0496 *ā kusmād ātmanepadinaḥ***.
The golden suite goes from 4932 to 5076 cells; curādi is open at 8 of 509.

**The engine's first non-Aṣṭādhyāyī rule id.** The 10a spec framed this
class as "how 1.3.12 and 1.3.74 meet on one row". The vidyut probe showed
otherwise: these upadeśas carry no marker, vidyut credits the gaṇasūtra
before 3.1.25, and no pada sūtra after it. 10.0496 now opens the `sanadi`
stage at that position, keyed on a new `PadaAssignment::Akusmiya` (held to
the dhātupāṭha range `10.0192..=10.0236` by the pada-marker test). It
sanctions ātmanepada and blocks parasmaipada; 1.3.78 declines on the tag,
and 1.3.12 / 1.3.74 never see it.

The audit shows zero divergence against `8da2f90b`; a throwaway probe
confirmed every ākusmīya parasmaipada cell is empty in vidyut and blocked
here; a main-vs-branch dump of every prior cell's traces is byte-identical;
the mutation gate is clean.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`: `git worktree remove .worktrees/curadi-10b` and `git worktree remove --force .worktrees/curadi-10b-proto` (the throwaway). Then delete the local and remote `curadi-10b` branch, and run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| `PadaAssignment::Akusmiya` (ātmanepada only), `AKUSMIYA` range, `Nic` doc | 2 |
| `Tag::Akusmiya`; `derive` and test-helper arms | 2 |
| 10.0496 first in `SANADI`, guarded on the tag, recording on ātmanepada, blocking parasmaipada; comment naming it the first gaṇasūtra id | 3 |
| 1.3.78's `Akusmiya` decline; 1.3.12/1.3.66/1.3.72/1.3.74 untouched and declining | 3 |
| Rule-order pin and its doc | 3 |
| Four rows; membership held to the range both ways; pada-marker arm corrected | 4 |
| 144 goldens, single-form; totals 111 / 5076 / 6206; ALTERNATES 1130 | 4 |
| `check()` on the new forms, and parasmaipada shapes Invalid | 4 |
| Trace pins: *cetayate* full, √vṛṣ/√mad/√kusm pre-ṇic shapes | 5 |
| Fires-only: 10.0496 on exactly the 144 cells; 1.3.74 never on them; parasmaipada blocked by 10.0496 alone | 5 |
| Prior traces byte-identical, main ↔ branch | 2–3 (suite at 4932), 6 Step 2 (dump diff) |
| Audit with repoint and negative control; parasmaipada emptiness probe | 6 |
| README / ARCHITECTURE / AGENTS counts, 25 → 29 ātmanepada-only, 10a spec correction | 6 |
| Floor, uncaught probe, campaign, verbatim non-caught record | 7 |

**Spec deviations, recorded:**
- **No separate "iff" test for `AKUSMIYA`.** The split curādi arm of `curated_pada_agrees_with_upadesha_markers` asserts membership both ways over every curated curādi row, so a second test would duplicate it.
- **The audit harness gains no parasmaipada check.** It compares only `padas()`; Task 6's throwaway probe covers the ākusmīya parasmaipada cells against vidyut, and the audit record says so.
- **Two code comments the spec did not list** become false and are corrected: `PadaAssignment::Nic`'s doc and 1.3.74's comment, both of which said every curādi row is `Nic` (Tasks 2, 3). And `guna.rs`'s 6.1.78 corpus-size comment moves 107 → 111 (Task 6).

**Type consistency.**
- `AKUSMIYA: RangeInclusive<&str>` is defined in Task 2 and read in Task 4 as `AKUSMIYA.contains(&d.dhatupatha)`.
- `pada_dhatu(&str, &[Tag], Pada) -> Prakriya` is in `sanadi.rs` tests; `akusmiya_prakriya(Pada) -> Prakriya` in `samjna.rs` tests.
- The rule id `"10.0496"` and name `"A kusmAd AtmanepadinaH"` are identical in the rule, its `record` call, the order pin and every test.

**Known soft spots.**
- **Doc strings in Task 6** were read from `main` at `d3972dd`. If one is not found exactly once, edit the paragraph to the same facts rather than skip it.
- **The *cetayate* order pin** is this engine's order, not vidyut's; the two agree on every rule both record.
