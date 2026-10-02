# Curādi gaṇa slice 10e Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate 92 of curādi's 96 adanta roots (`10.0108 mArga` and `10.0389 kaTa` … `10.0492 Deka`, minus the four optional-ṇic rows) across laṭ / laṅ / loṭ / vidhiliṅ. 83 are ubhayapadī by 1.3.74 and nine ā-garvīya rows are ātmanepadī. The work needs:
- **6.4.48** *ato lopaḥ ārdhadhātuke*, which deletes the final `a` before ṇic;
- a `Tag::AtLopa` carrying 1.1.57's sthānivadbhāva, so that 7.2.116 and 7.3.86 decline (*kathayati*, *kuhayate*);
- the gaṇasūtra **10.0497** *ā garvād ātmanepadinaḥ*;
- **8.3.24**, widened to a curādi root's own `n`.

The golden suite goes from 6696 to 12996 cells, and curādi from 47 to 139 of its 509 rows.

**Architecture:** Seven tasks:
- **Task 1** checks the worktree and the baseline.
- **Task 2** is the engine. It lands green on its own, because no curated row is adanta or ā-garvīya yet:
  - `AA_GARVIYA` and `PadaAssignment::AaGarviya`;
  - `Tag::AaGarviya` and `Tag::AtLopa`;
  - 10.0497 and 6.4.48 in the sanādi stage;
  - the `AtLopa` guards on 7.2.116 and the sanādi 7.3.86;
  - 1.3.78's `AaGarviya` decline;
  - unit tests and the order pin.
- **Task 3** widens 8.3.24 to a curādi root's own `n`. Its only witness so far is 10c's √gandh, which gains 8.3.24 → 8.4.58 on its 36 branches.
- **Task 4** lands the 92 rows, 700 golden rows and 498 alternates, plus every count, list, trace and `check()` assertion they move. These must land together: the suite is red between any of them.
- **Tasks 5–7** are the audit with the prior-trace diff and the doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.99.0, pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, checked out at `/tmp/vidyut-full`.

**Spec:** `docs/superpowers/specs/2026-10-02-curadi-gana-10e-design.md`. Read its two amendment paragraphs. One is the "8.3.24 widens" decision; the other is "Prototype (amendment)" under Evidence. Both postdate the first draft.

**Workspace:** the branch `curadi-10e` is checked out at `/workspace/.worktrees/curadi-10e` and holds the spec and this plan. Every path below is relative to that directory unless it starts with `/`.

**Provenance.** This slice was built end to end on a throwaway worktree, `/workspace/.worktrees/curadi-10e-proto`. It is detached at the throwaway commit chain `c21551d`…, and Task 7 deletes it. Every code block below is that prototype's code. Its **final** state was verified:
- the full suite, clippy `-D warnings` and `fmt-check`;
- the golden generator (Task 4) derived all 6300 new cells with the prototype engine and found each cell's derivation set equal to vidyut's: 6300 cells, 6798 forms, 0 differences;
- the audit at 242 roots / 12996 cells / 14660 forms showed **zero differences**, and the `entry` control failed on 36 cells;
- a main-vs-prototype dump of all 7862 prior live-branch logs was byte-identical except √gandh's 36 lines, each gaining exactly ` 8.3.24 8.4.58`;
- `cargo mutants --in-diff` over the production diff: 24 mutants, 23 caught, 1 unviable, 0 missed.

The intermediate states at the end of Tasks 2 and 3 were not run separately. Each task's own test step is the check.

**Throwaway scripts.** Four helpers live outside the repo and never ship. Each is reproduced in full below, so it can be recreated if `/tmp` was cleaned:
- `/tmp/vidyut-full/slice10e/gen_rows_10e.py` (Task 4);
- `/tmp/vidyut-full/vidyut-prakriya/examples/curadi_goldens_10e.rs` (Task 4);
- `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_10e.rs` (Task 5);
- `/tmp/vidyut-full/slice10e/docsweep_10e.py` (Task 5).

## Global Constraints

- **Two new rule ids, no others:** `10.0497` (`RuleKind::Vidhi`, name `"A garvAd AtmanepadinaH"`) and `6.4.48` (`RuleKind::Vidhi`, name `"ato lopaH"`), both in `tinanta/sanadi.rs`. Do not credit 1.1.57 and do not credit 8.3.110 (spec, Decisions).
- **Out of scope:** `10.0400 pata`, `10.0451 mUtra`, `10.0456 katra` and `10.0449 garva` (optional ṇic). `AA_GARVIYA` includes `10.0449` anyway.
- **Pre-existing cells must stay byte-identical, traces included,** with one named exception: √gandh (`10.0204`) gains 8.3.24 → 8.4.58 on its 36 branches. Regenerate no prior golden.
- **Goldens come from the generator, which asserts engine = vidyut cell by cell, and their sha256 must match this plan's.** **Do not edit a golden to match the engine.** If the generator reports a difference, or a hash differs, stop and report.
- **New rows are found by shape (curādi, `code` ending in `a`) and ā-garvīya rows by the positional `AA_GARVIYA` range** in trace tests, never by a hand list of numbers.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit.
- `mise run test` now takes about 30 s on a quiet host. Run it in the **foreground** with a timeout of 600000 ms. Never background it and end a turn.
- `mise run test -- -p X` does not scope. Scope with `mise exec -- cargo test -p <crate> <filter>`. To see every failing binary at once, use `mise exec -- cargo test --workspace --no-fail-fast`.
- The `cargo-mutants` mise shim fails here. Use the real binary, `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`, under `mise exec --`.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **The 7.1.3 `n` of a curādi 3pl** (*corayanti*, *kathayanti*) must not reach the widened 8.3.24. Its `n` sits in the tiṅ term, not `ANGA`. → Task 3/4 `nas_capadantasya_is_credited_only_on_rudhadi_dhan_jan_and_curadi_roots`, which pins the curādi credits to exactly eight rows and their branch count; Task 5's prior-trace diff.
2. **An ā-garvīya root asked for parasmaipada.** It must block, with nothing recorded, never fall through to 1.3.78. → Task 2 `a_garvad_sanctions_…_and_blocks_its_parasmaipada` and `every_pada_sutra_leaves_an_a_garviya_root_to_10_0497`; Task 4 `a_garvad_is_credited_on_exactly_the_a_garviya_cells`; `check("padayati")` Invalid.
3. **6.4.48 reaching a root that is not adanta, or firing before ṇic is ārdhadhātuka.** A consonant-final or `A`-final root, no ṇic, or a ṇic before 3.4.114 must decline and leave the aṅga untouched. → Task 2 `ato_lopa_deletes_an_adanta_roots_a_before_ardhadhatuka_nic`; Task 4 `ato_lopa_is_credited_on_exactly_the_adanta_cells`.
4. **A homograph surface in `check()`.** *rahayati*, *cahayati*, *kUwayate* and *vizkayate* must each yield exactly two analyses, the adanta row's and its twin's, with the right trace on each. → Task 4 `curadi_analyses_its_adanta_forms` and the updated `curadi_analyses_its_jnapadi_forms`.
5. **The shapes the slice prevents** (*kATayati*, *gARayati*, *kohayate*, *goRayati*, *sanketayati*) must be Invalid, not derivable another way. → Task 4 `curadi_analyses_its_adanta_forms`.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-data/src/lib.rs` | Task 2: `PadaAssignment::AaGarviya`, `AA_GARVIYA`, the upstream pin, the `padas()` test, the agreement test's curādi branch. Task 4: 92 rows, row-list test, `dhatus().len()`, the `pada` doc census. Task 5: two comments |
| `crates/panini-prakriya/src/term.rs` | Task 2: `Tag::AaGarviya`, `Tag::AtLopa` |
| `crates/panini-prakriya/src/tinanta/mod.rs` | Task 2: `derive` maps `AaGarviya` |
| `crates/panini-prakriya/src/tinanta/samjna.rs` | Task 2: 1.3.78's decline, its comment, the test helper's arm, one test |
| `crates/panini-prakriya/src/tinanta/sanadi.rs` | Task 2: 10.0497, 6.4.48, two guards, module doc, four tests |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Task 2: order pin and its doc |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | Task 3: 8.3.24's guard and two comments |
| `crates/panini/tests/trace/juhotyadi.rs` | Task 3: the 8.3.24 credit test. Task 4: it again, and the 8.4.40 test |
| `crates/panini/tests/paradigm/data/curadi.rs` | Task 4: 700 golden rows, 498 alternates |
| `crates/panini/tests/paradigm/main.rs` | Task 4: totals, alternates census, pada-ambiguous set, 10d's `check()` test, the new `check()` test. Task 5: audit-chain prose |
| `crates/panini/tests/trace/curadi.rs` | Task 4: module doc, 1.3.74 half, four new tests |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `guna.rs` (one comment), the 10d spec | Task 5 |
| `AGENTS.md`, maybe `mise.toml` | Task 6 |

---

## Task 1: The worktree and baseline

**Files:** none. **Interfaces:** none.

- [ ] **Step 1: Confirm the worktree**

```bash
cd /workspace/.worktrees/curadi-10e
git status --short          # empty
git log --oneline -5        # the plan commit, 78fd131 (kathayati), 0809f82 (spec amendment), 72a3eda (spec), c6811b1
```

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Run in the foreground, timeout 600000 ms. Expected: everything passes at 6696 cells. `panini-prakriya` reports 406 tests, the `trace` binary 199, `paradigm` 23 and `panini-data` 24.

---

## Task 2: The engine — `AA_GARVIYA`, the two tags, 10.0497, 6.4.48 and the 1.1.57 guards

No curated row is adanta or ā-garvīya yet. After this task 6.4.48 and 10.0497 fire nowhere in the corpus, and every golden is unchanged. The suite is green at the end of the task.

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (the `PadaAssignment` enum; after `pub const JNAPADI`; `padas()`; the `tests` module)
- Modify: `crates/panini-prakriya/src/term.rs` (`enum Tag`)
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs` (`derive`'s pada `match`)
- Modify: `crates/panini-prakriya/src/tinanta/samjna.rs` (1.3.78; `pada_anga`; one test)
- Modify: `crates/panini-prakriya/src/tinanta/sanadi.rs` (module doc, two `Rule`s, two guards, four tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (order pin and its doc)

**Interfaces:**
- Produces:
  - `panini_data::AA_GARVIYA: RangeInclusive<&'static str>` = `"10.0440"..="10.0449"`;
  - `PadaAssignment::AaGarviya`, whose `padas()` is `&[Pada::Atmanepada]`;
  - `Tag::AaGarviya` and `Tag::AtLopa`;
  - rule ids `"10.0497"` (second in `SANADI`) and `"6.4.48"` (right after `"3.4.114"`).
- Consumes:
  - the sanādi tests' helpers `with_nic(root: &str, nic: Option<&[Tag]>) -> Prakriya` and `pada_dhatu(root: &str, tags: &[Tag], pada: Pada) -> Prakriya`;
  - samjna's test helper `akusmiya_prakriya(pada: Pada) -> Prakriya`;
  - panini-data's test helper `upstream_rows() -> Vec<(&'static str, &'static str, &'static str)>`.

- [ ] **Step 1: Write the failing tests**

**(a)** In `crates/panini-data/src/lib.rs`'s `tests` module, immediately before `#[test] fn jnapadi_is_exactly_the_rows_10_0493_names()`, add:

```rust
    #[test]
    fn aa_garviya_is_exactly_the_rows_10_0497_names() {
        // The gaṇasūtra 10.0497 follows `10.0449 garva` and makes the ten
        // rows from `10.0440 pada` ātmanepadī. vidyut-prakriya lists the
        // same ten upadeśas (`AA_GARVIYA`). `garva` is not curated, so this
        // test — not a derivation — is what holds the range's upper end.
        let rows = upstream_rows();
        let in_range: Vec<&str> = rows
            .iter()
            .filter(|(n, _, _)| AA_GARVIYA.contains(n))
            .map(|(_, u, _)| *u)
            .collect();
        assert_eq!(
            in_range,
            ["pada", "gfha", "mfga", "kuha", "SUra", "vIra", "sTUla", "arTa", "satra", "garva"]
        );
        // The neighbours on either side exist upstream and fall outside.
        for n in ["10.0439", "10.0450"] {
            assert!(rows.iter().any(|(m, _, _)| *m == n), "{n}");
            assert!(!AA_GARVIYA.contains(&n), "{n}");
        }
    }

```

In the same module, immediately after the line `assert_eq!(PadaAssignment::Akusmiya.padas(), &[Pada::Atmanepada]);`, add:

```rust
        assert_eq!(PadaAssignment::AaGarviya.padas(), &[Pada::Atmanepada]);
```

**(b)** In `crates/panini-prakriya/src/tinanta/sanadi.rs`'s `tests` module, immediately before `#[test] fn ata_upadhayah_lengthens_an_a_upadha_before_nit()`, add:

```rust
    #[test]
    fn ato_lopa_deletes_an_adanta_roots_a_before_ardhadhatuka_nic() {
        for (root, want) in [("kaTa", "kaT"), ("kuha", "kuh"), ("Una", "Un")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("6.4.48").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
            assert!(p.terms[ANGA].has(Tag::AtLopa), "{root}");
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["6.4.48"]);
        }
        // Not `a`-final: a consonant (cur) or a long `A` (pA).
        for root in ["cur", "pA"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("6.4.48").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
            assert!(!p.terms[ANGA].has(Tag::AtLopa), "{root}");
        }
        // `a`-final, but no ārdhadhātuka follower: no ṇic at all, or a ṇic
        // that 3.4.114 has not reached yet.
        for nic in [None, Some(&[Tag::Rit][..])] {
            let mut p = with_nic("kaTa", nic);
            assert!(!(rule("6.4.48").apply)(&mut p), "{nic:?}");
            assert_eq!(p.terms[ANGA].text, "kaTa");
            assert!(!p.terms[ANGA].has(Tag::AtLopa), "{nic:?}");
        }
    }

    #[test]
    fn the_upadha_rules_decline_after_ato_lopa() {
        // 1.1.57: after 6.4.48, `kaT`'s `a` and `kuh`'s `u` sit at the upadhā,
        // but the deleted `a` still stands for 7.2.116 and 7.3.86.
        for (id, root) in [("7.2.116", "kaT"), ("7.3.86", "kuh")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            p.terms[ANGA].add(Tag::AtLopa);
            assert!(!(rule(id).apply)(&mut p), "{id} {root}");
            assert_eq!(p.terms[ANGA].text, root);
            assert!(p.log.is_empty(), "{id} {root}");
        }
        // The same shapes without the tag are ordinary roots, and both fire.
        for (id, root, want) in [("7.2.116", "kaT", "kAT"), ("7.3.86", "kuh", "koh")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule(id).apply)(&mut p), "{id} {root}");
            assert_eq!(p.terms[ANGA].text, want);
        }
    }

```

At the end of the same `tests` module, after `fn a_kusmad_declines_without_the_akusmiya_licence() { … }` and before the module's closing `}`, add:

```rust

    #[test]
    fn a_garvad_sanctions_an_a_garviya_roots_atmanepada_and_blocks_its_parasmaipada() {
        let mut p = pada_dhatu("kuha", &[Tag::Curadi, Tag::AaGarviya], Pada::Atmanepada);
        assert!((rule("10.0497").apply)(&mut p));
        assert!(!p.blocked);
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["10.0497"]);
        assert_eq!(p.terms[ANGA].text, "kuha", "a sanction, not an operation");
        let mut p = pada_dhatu("kuha", &[Tag::Curadi, Tag::AaGarviya], Pada::Parasmaipada);
        assert!(!(rule("10.0497").apply)(&mut p));
        assert!(p.blocked);
        assert!(p.log.is_empty());
    }

    #[test]
    fn a_garvad_declines_without_the_a_garviya_licence() {
        // An ākusmīya root is 10.0496's, not 10.0497's; √cur, √rudh, √bhū
        // and √ās are left alone too. Both padas: not recorded, not blocked.
        for tags in [
            &[Tag::Curadi, Tag::Akusmiya][..],
            &[Tag::Curadi, Tag::Nic][..],
            &[Tag::Ubhayapadin][..],
            &[][..],
            &[Tag::Atmanepadin][..],
        ] {
            for pada in [Pada::Parasmaipada, Pada::Atmanepada] {
                let mut p = pada_dhatu("x", tags, pada);
                assert!(!(rule("10.0497").apply)(&mut p), "{tags:?} {pada:?}");
                assert!(!p.blocked, "{tags:?} {pada:?}");
                assert!(p.log.is_empty(), "{tags:?} {pada:?}");
            }
        }
    }
```

**(c)** In `crates/panini-prakriya/src/tinanta/samjna.rs`'s `tests` module, immediately before `#[test] fn pada_sutras_are_order_independent()`, add:

```rust
    #[test]
    fn every_pada_sutra_leaves_an_a_garviya_root_to_10_0497() {
        // 10.0497 is 10.0496's twin: it has sanctioned an ā-garvīya root's
        // ātmanepada in `super::sanadi`, so no pada sūtra here may record on
        // it or block it — 1.3.78 declining on `Tag::AaGarviya`, the rest for
        // want of their tags. 1.3.78's parasmaipada arm is unreachable, as
        // for the ākusmīya: 10.0497 has already blocked that branch.
        let a_garviya = |pada| {
            let mut p = akusmiya_prakriya(pada);
            p.terms[ANGA] = Term::new("kuha");
            p.terms[ANGA].add(Tag::Dhatu);
            p.terms[ANGA].add(Tag::Curadi);
            p.terms[ANGA].add(Tag::AaGarviya);
            p
        };
        for id in ["1.3.12", "1.3.66", "1.3.72", "1.3.74", "1.3.78"] {
            let rule = SAMJNA.iter().find(|r| r.id == id).unwrap();
            let mut p = a_garviya(Pada::Atmanepada);
            assert!(!(rule.apply)(&mut p), "{id} fired on kuha Atmanepada");
            assert!(!p.blocked, "{id} blocked kuha Atmanepada");
            assert!(p.log.is_empty(), "{id} recorded on kuha Atmanepada");
        }
        for id in ["1.3.12", "1.3.66", "1.3.72", "1.3.74"] {
            let rule = SAMJNA.iter().find(|r| r.id == id).unwrap();
            let mut p = a_garviya(Pada::Parasmaipada);
            assert!(!(rule.apply)(&mut p), "{id} fired on kuha Parasmaipada");
            assert!(!p.blocked, "{id} blocked kuha Parasmaipada");
            assert!(p.log.is_empty(), "{id} recorded on kuha Parasmaipada");
        }
    }

```

**(d)** In `crates/panini-prakriya/src/tinanta/derivation_tests.rs`, in `tinanta_rule_order_is_pinned`, replace the opening of `expected`:

```rust
        "10.0496", "10.0493", "3.1.25", "1.3.9", "3.4.114", "7.2.116", "6.4.92", "7.3.86",
        "3.1.32",
```

with

```rust
        "10.0496", "10.0497", "10.0493", "3.1.25", "1.3.9", "3.4.114", "6.4.48", "7.2.116",
        "6.4.92", "7.3.86", "3.1.32",
```

`mise run fmt` rewraps the array.

- [ ] **Step 2: Run them to see them fail**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | grep -E "^error|cannot find|no variant" | head`
Expected: compile errors: `no variant … named AtLopa` / `AaGarviya`, and `cannot find value AA_GARVIYA`. Nothing exists yet. (`panini-data` fails the same way.)

- [ ] **Step 3: The data layer**

In `crates/panini-data/src/lib.rs`, the `PadaAssignment` enum: immediately after `    Akusmiya,` (its last variant) and before the enum's closing `}`, add:

```rust
    /// Ātmanepada only, sanctioned by the dhātupāṭha gaṇasūtra 10.0497
    /// *ā garvād ātmanepadinaḥ*: the curādi roots from `10.0440 pada` up to
    /// `10.0449 garva` (`AA_GARVIYA`) are ātmanepadī. Same standing as
    /// `Akusmiya`: neither a marker of the root's nor the affix's, so a root
    /// carrying it is credited 10.0497 and no pada sūtra at all: never
    /// 1.3.12, 1.3.74 or 1.3.78.
    ///
    /// Keyed on the row's position, as the gaṇasūtra is. vidyut-prakriya
    /// lists the same ten upadeśas (`AA_GARVIYA`). The gaṇasūtra applies only
    /// on the ṇic branch; `10.0449 garva`, whose ṇic is optional, is not
    /// curated yet, so every curated ā-garvīya row takes ṇic.
    AaGarviya,
```

Immediately after `pub const JNAPADI: RangeInclusive<&str> = "10.0118"..="10.0124";`, add:

```rust

/// The ā-garvīya of curādi: dhātupāṭha rows `10.0440` (`pada`) through
/// `10.0449` (`garva`), the scope of the gaṇasūtra 10.0497 *ā garvād
/// ātmanepadinaḥ*. Compared as strings, like `AKUSMIYA`. A curated row is
/// `PadaAssignment::AaGarviya` exactly when it is curādi and in this range;
/// `curated_pada_agrees_with_upadesha_markers` holds both sides. Includes
/// `10.0449 garva`, which is ā-garvīya but not yet curated (its ṇic is
/// optional). `aa_garviya_is_exactly_the_rows_10_0497_names` pins the range
/// to upstream.
pub const AA_GARVIYA: RangeInclusive<&str> = "10.0440"..="10.0449";
```

In `padas()`, replace `            PadaAssignment::Akusmiya => &[Pada::Atmanepada],` with `            PadaAssignment::Akusmiya | PadaAssignment::AaGarviya => &[Pada::Atmanepada],`.

In `curated_pada_agrees_with_upadesha_markers`, replace

```rust
            // gaṇasūtra 10.0496 ā kusmād ātmanepadinaḥ (`Akusmiya`,
            // ātmanepada only); outside it, the affix's 1.3.74 ṇicaś ca
            // (`Nic`, both padas).
```

with

```rust
            // gaṇasūtra 10.0496 ā kusmād ātmanepadinaḥ (`Akusmiya`,
            // ātmanepada only); inside `AA_GARVIYA`, its twin 10.0497 ā
            // garvād ātmanepadinaḥ (`AaGarviya`); outside both, the affix's
            // 1.3.74 ṇicaś ca (`Nic`, both padas).
```

and replace

```rust
                    (PadaAssignment::Akusmiya, "ākusmīya, so 10.0496's")
                } else {
                    (PadaAssignment::Nic, "outside the ākusmīya, so 1.3.74's")
                };
```

with

```rust
                    (PadaAssignment::Akusmiya, "ākusmīya, so 10.0496's")
                } else if AA_GARVIYA.contains(&d.dhatupatha) {
                    (PadaAssignment::AaGarviya, "ā-garvīya, so 10.0497's")
                } else {
                    (
                        PadaAssignment::Nic,
                        "outside the ākusmīya and the ā-garvīya, so 1.3.74's",
                    )
                };
```

- [ ] **Step 4: The tags, `derive`, and 1.3.78**

In `crates/panini-prakriya/src/term.rs`, immediately after the `    Akusmiya,` variant, add:

```rust
    /// The dhātu's ātmanepada is sanctioned by the dhātupāṭha gaṇasūtra
    /// 10.0497 *ā garvād ātmanepadinaḥ*: the data layer's
    /// `PadaAssignment::AaGarviya`. Read only by 10.0497 in `tinanta::sanadi`,
    /// and by 1.3.78's ātmanepada arm, which declines rather than blocks
    /// when it is present. Same standing as `Akusmiya`.
    AaGarviya,
```

Immediately after the `    Nijanta,` variant (the one whose doc ends `in an ārdhadhātuka-lakāra slice, is the first rule that will.`), add:

```rust
    /// 6.4.48 *ato lopaḥ ārdhadhātuke* deleted the dhātu's final `a` (an
    /// adanta curādi root, before ṇic). Carries 1.1.57 *acaḥ parasmin
    /// pūrvavidhau*: the deleted `a` still counts for a rule about what
    /// precedes it, so 7.2.116 and the sanādi 7.3.86 decline on this tag
    /// rather than read the consonant now at the upadhā. Set by 6.4.48 in
    /// `tinanta::sanadi`; kept through 3.1.32's fold, for later stages that
    /// will read it as vidyut-prakriya reads its `FlagAtLopa`.
    AtLopa,
```

In `crates/panini-prakriya/src/tinanta/mod.rs` (`derive`) and in `crates/panini-prakriya/src/tinanta/samjna.rs` (`pada_anga`, the test helper), immediately after the arm `PadaAssignment::Akusmiya => t.add(Tag::Akusmiya),`, add the arm `PadaAssignment::AaGarviya => t.add(Tag::AaGarviya),` with the same indentation.

In `samjna.rs`, 1.3.78's ātmanepada arm, replace the comment lines

```rust
                // 1.3.74 (Nic) or the gaṇasūtra 10.0496 (Akusmiya, in `super::sanadi`)
                // has already sanctioned this cell, so decline instead of blocking.
                // Only the genuine śeṣa (no pada tag at all) blocks here. An
                // Akusmiya root never reaches the parasmaipada arm above: 10.0496
                // has already blocked that branch.
```

with

```rust
                // 1.3.74 (Nic) or the gaṇasūtras 10.0496 (Akusmiya) and 10.0497
                // (AaGarviya), both in `super::sanadi`, have already sanctioned this
                // cell, so decline instead of blocking. Only the genuine śeṣa (no
                // pada tag at all) blocks here. An Akusmiya or AaGarviya root never
                // reaches the parasmaipada arm above: its gaṇasūtra has already
                // blocked that branch.
```

and in its condition, after `                        || p.terms[ANGA].has(Tag::Akusmiya)`, add the line `                        || p.terms[ANGA].has(Tag::AaGarviya)`.

- [ ] **Step 5: The sanādi rules and guards**

In `crates/panini-prakriya/src/tinanta/sanadi.rs`, replace the module doc's first paragraph

```rust
//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, ṇic's
//! it-lopa (1.3.9), 3.4.114, 7.2.116, 6.4.92, 7.3.86, 3.1.32 — opened by
//! two dhātupāṭha gaṇasūtras: 10.0496, which settles an ākusmīya root's
//! pada, and 10.0493, which credits a jñapādi root's mit-tva, both before
//! ṇic is added.
```

with

```rust
//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, ṇic's
//! it-lopa (1.3.9), 3.4.114, 6.4.48, 7.2.116, 6.4.92, 7.3.86, 3.1.32 —
//! opened by three dhātupāṭha gaṇasūtras: 10.0496 and 10.0497, which settle
//! an ākusmīya or ā-garvīya root's pada, and 10.0493, which credits a
//! jñapādi root's mit-tva, all before ṇic is added.
```

and its last paragraph

```rust
//! Every rule self-guards: 10.0496 on `Tag::Akusmiya`, 10.0493 on
//! `Tag::Mit`, 3.1.25 on `Tag::Curadi`, the rest on ṇic being present
//! (6.4.92 on `Tag::Mit` as well). For gaṇas 1–9 the stage
//! adds nothing and records nothing.
```

with

```rust
//! Every rule self-guards: 10.0496 on `Tag::Akusmiya`, 10.0497 on
//! `Tag::AaGarviya`, 10.0493 on `Tag::Mit`, 3.1.25 on `Tag::Curadi`, the
//! rest on ṇic being present (6.4.48 on an `a`-final aṅga as well, 6.4.92 on
//! `Tag::Mit`, and 7.2.116 and 7.3.86 decline on 6.4.48's `Tag::AtLopa`).
//! For gaṇas 1–9 the stage adds nothing and records nothing.
```

Immediately before the comment line `    // 10.0493, the gaṇasūtra closing the jñapādi` (so, right after 10.0496's `Rule { … },`), add:

```rust
    // 10.0497 ā garvād ātmanepadinaḥ: the curādi roots from `10.0440 pada` up
    // to `10.0449 garva` are ātmanepadī — the data layer's `AA_GARVIYA`
    // range, carried here as `Tag::AaGarviya`. 10.0496's twin in every
    // respect but the range: it settles the pada outright, so no pada sūtra
    // in `super::samjna` is credited after it, and the parasmaipada branch
    // BLOCKS. vidyut-prakriya credits it where it credits 10.0496, before
    // 3.1.25. The range includes `10.0449 garva`, whose ṇic is optional and
    // which is not curated yet: the gaṇasūtra applies only on its ṇic branch.
    Rule {
        id: "10.0497",
        name: "A garvAd AtmanepadinaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::AaGarviya) {
                return false;
            }
            match p.ctx.pada {
                Pada::Atmanepada => {
                    let before = p.snapshot();
                    p.record("10.0497", "A garvAd AtmanepadinaH", before);
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

Immediately before the comment line `    // 7.2.116 ata upadhāyāḥ: vṛddhi of an `a` upadhā` (so, right after 3.4.114's `Rule { … },`), add:

```rust
    // 6.4.48 ato lopaḥ ārdhadhātuke: an aṅga's final `a` is deleted before an
    // ārdhadhātuka affix. The adanta curādi roots (`10.0389 kaTa` …) meet it
    // here, before ṇic: `kaTa` → `kaT`. vidyut-prakriya credits it at this
    // very point, after 3.4.114 and before 3.1.32.
    //
    // The lopa is not the end of the `a`. By 1.1.57 acaḥ parasmin
    // pūrvavidhau, a vowel replaced because of what follows still stands for
    // a rule about what precedes it: 7.2.116 and 7.3.86 below would read
    // `kaT`'s `a` and `kuh`'s `u` as the upadhā, but the deleted `a` is still
    // there for them, so *kathayati*, not *kāthayati*, and *kuhayate*, not
    // *kohayate*. `Tag::AtLopa` carries that, and both decline on it. 1.1.57
    // is a paribhāṣā and is not credited, as vidyut does not credit it. The
    // tag outlives 3.1.32, for later stages that must know the `a` was there.
    Rule {
        id: "6.4.48",
        name: "ato lopaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Ardhadhatuka)) {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix('a') else {
                return false;
            };
            let stem = stem.to_string();
            let before = p.snapshot();
            p.terms[ANGA].text = stem;
            p.terms[ANGA].add(Tag::AtLopa);
            p.record("6.4.48", "ato lopaH", before);
            true
        },
    },
```

In 7.2.116's `apply` (the entry named `"ata upaDAyAH"`), replace its first guard

```rust
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) {
                return false;
            }
```

with

```rust
            // 1.1.57: 6.4.48's deleted `a` still stands; see 6.4.48 above.
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) || p.terms[ANGA].has(Tag::AtLopa)
            {
                return false;
            }
```

In 7.3.86's `apply` (the entry named `"pugantalaGUpaDasya ca"`), replace its first guard

```rust
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Ardhadhatuka)) {
                return false;
            }
```

with

```rust
            // 1.1.57: 6.4.48's deleted `a` still stands; see 6.4.48 above.
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Ardhadhatuka))
                || p.terms[ANGA].has(Tag::AtLopa)
            {
                return false;
            }
```

Both `old` guards occur more than once in the file (3.1.32 and 6.4.48 share them), so edit by the rule's `name`, not by a blind search-and-replace.

- [ ] **Step 6: The order pin's doc**

In `derivation_tests.rs`, replace

```rust
/// here. 10.0496 and 10.0493 are the only ids here that are not
/// Aṣṭādhyāyī sūtras.
```

with

```rust
/// here.
///
/// Slice 10e adds the gaṇasūtra 10.0497 *ā garvād ātmanepadinaḥ* right after
/// 10.0496, its twin, and 6.4.48 *ato lopaḥ ārdhadhātuke* right after
/// 3.4.114, where vidyut credits it: the adanta root's final `a` goes as soon
/// as ṇic is ārdhadhātuka. 7.2.116 and 7.3.86 then decline on its
/// `Tag::AtLopa` (1.1.57). 10.0496, 10.0497 and 10.0493 are the only ids
/// here that are not Aṣṭādhyāyī sūtras.
```

- [ ] **Step 7: Run the tests**

```bash
mise run fmt
mise exec -- cargo test -p panini-data 2>&1 | grep "test result" | head -1
mise exec -- cargo test -p panini-prakriya 2>&1 | grep "test result" | head -1
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: `panini-data` 25 and `panini-prakriya` 411, all green. `trace` 199 and `paradigm` 23 are unchanged and green.

- [ ] **Step 8: Commit**

```bash
mise run lint
git add -A
git commit -m "feat(engine): 6.4.48 ato lopaḥ and 10.0497 ā garvād ātmanepadinaḥ in the sanādi stage

AA_GARVIYA (10.0440–10.0449) pinned to upstream, PadaAssignment::AaGarviya
and Tag::AaGarviya mirror 10.0496's ākusmīya. 6.4.48 deletes an adanta
root's final a before ārdhadhātuka ṇic and sets Tag::AtLopa, on which
7.2.116 and the sanādi 7.3.86 decline (1.1.57). No curated row is adanta
yet, so every golden is unchanged."
```

---

## Task 3: Widen 8.3.24 to a curādi root's own `n`

The only prior row that reaches the widened guard is 10c's √gandh (`ganD`). Its 36 branches gain 8.3.24 → 8.4.58, the pair vidyut also credits, and its forms are unchanged.

**Files:**
- Modify: `crates/panini/tests/trace/juhotyadi.rs` (`nas_capadantasya_is_credited_only_on_rudhadi_dhan_and_jan`)
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (8.3.24's comment and guard; the 8.4.58 comment; the 8.4.40 comment that names 8.3.24's guard)

**Interfaces:**
- Consumes: the trace helper `credited(&str) -> Vec<(&'static str, Gana)>`, which counts **live branches**.
- Produces: no symbols. Task 4 extends this test's `CURADI` list.

- [ ] **Step 1: Write the failing test**

In `crates/panini/tests/trace/juhotyadi.rs`, replace the whole of

```rust
fn nas_capadantasya_is_credited_only_on_rudhadi_dhan_and_jan() {
    // 8.3.24 admits juhotyādi since slice 3f; only √dhan and √jan (3f3) have
    // an `n` before a jhal there — √jan only before a pit ending (jajanti,
    // jajaMsi, jajantu), since 6.4.42 takes the `n` before a kṅit one.
    let hits = credited("8.3.24");
    for (number, gana) in &hits {
        assert!(
            *gana == Gana::Rudhadi || *number == "03.0024" || *number == "03.0025",
            "8.3.24 credited on {number}"
        );
    }
```

with

```rust
fn nas_capadantasya_is_credited_only_on_rudhadi_dhan_jan_and_curadi_roots() {
    // 8.3.24 admits juhotyādi since slice 3f; only √dhan and √jan (3f3) have
    // an `n` before a jhal there — √jan only before a pit ending (jajanti,
    // jajaMsi, jajantu), since 6.4.42 takes the `n` before a kṅit one.
    // Since slice 10e it also admits a curādi root's own `n` before a jhal:
    // 10c's √gandh.
    const CURADI: [&str; 1] = ["10.0204"];
    let hits = credited("8.3.24");
    for (number, gana) in &hits {
        assert!(
            *gana == Gana::Rudhadi
                || *number == "03.0024"
                || *number == "03.0025"
                || CURADI.contains(number),
            "8.3.24 credited on {number}"
        );
    }
    let curadi: Vec<_> = hits.iter().filter(|(_, g)| *g == Gana::Curadi).collect();
    // √gandh's 36 ātmanepada branches.
    assert_eq!(curadi.len(), 36);
    for number in CURADI {
        assert!(
            curadi.iter().any(|(n, _)| *n == number),
            "{number} no longer witnesses 8.3.24"
        );
    }
```

(The rest of the function, the √dhan witness assertion, is unchanged.)

- [ ] **Step 2: Run it to see it fail**

Run: `mise exec -- cargo test -p panini --test trace nas_capadantasya 2>&1 | grep -E "panicked|left|right|test result"`
Expected: FAIL, `left: 0` / `right: 36`.

- [ ] **Step 3: Widen the guard**

In `crates/panini-prakriya/src/tinanta/tripadi.rs`, 8.3.24's comment block: immediately after the line `    // without needing a pada-boundary notion the engine does not have.`, add:

```rust
    //
    // CURĀDI, ROOT-INTERNAL ONLY (slice 10e). Six adanta curādi roots carry
    // their own `n` before a jhal (`sanketa`, `ansa`, `sangrAma`, `danqa`,
    // `anka`, `anga`: *saṅketayati*, *aṃsayati*); 10c's √gandh (`ganD`)
    // does too, and vidyut credits the pair on it as on them. For a curādi
    // root the search is confined to `ANGA`'s own characters: that `n` is
    // inside the dhātu, so `apadāntasya` holds by construction, and the
    // 7.1.3 `n` of *corayanti* — in the tiṅ term, never `ANGA` — stays out
    // of reach exactly as for every other gaṇa.
```

In 8.3.24's `apply`, replace

```rust
            let anga = &p.terms[ANGA];
            if !anga.has(Tag::Rudhadi) && !anga.has(Tag::Juhotyadi) {
                return false;
            }
            let w = word_chars(p);
            let Some(pos) = w.iter().enumerate().position(|(i, (_, _, c))| {
                *c == 'n' && w.get(i + 1).is_some_and(|(_, _, next)| is_jhal(*next))
            }) else {
```

with

```rust
            let anga = &p.terms[ANGA];
            let root_only = anga.has(Tag::Curadi);
            if !anga.has(Tag::Rudhadi) && !anga.has(Tag::Juhotyadi) && !root_only {
                return false;
            }
            let w = word_chars(p);
            let Some(pos) = w.iter().enumerate().position(|(i, (term, _, c))| {
                *c == 'n'
                    && (!root_only || *term == ANGA)
                    && w.get(i + 1).is_some_and(|(_, _, next)| is_jhal(*next))
            }) else {
```

In 8.4.58's comment, replace

```rust
    // Retire the fold, and this constraint with it, when a slice widens
    // 8.3.24 past rudhādi and juhotyādi.
```

with

```rust
    // Retire the fold, and this constraint with it, when a slice widens
    // 8.3.24 past rudhādi and juhotyādi. Slice 10e's curādi widening does
    // not: it reaches only a curādi root's own `n`, so the 7.1.3 `n` of
    // every non-rudhādi, non-juhotyādi root still needs the fold.
```

In 8.4.40's comment, replace

```rust
    // from what follows `j`/`c`. 8.3.24's guard is `Tag::Rudhadi` or
    // `Tag::Juhotyadi`, gaṇa tags rather than a grammatical predicate, so
```

with

```rust
    // from what follows `j`/`c`. 8.3.24's guard is `Tag::Rudhadi` or
    // `Tag::Juhotyadi` (or `Tag::Curadi`, for a root-internal `n` only),
    // gaṇa tags rather than a grammatical predicate, so
```

- [ ] **Step 4: Run the suite**

```bash
mise run fmt
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: all green, with counts unchanged from Task 2 (`trace` 199). √gandh's goldens hold: 8.4.58 turns the anusvāra straight back to `n` before `D`.

- [ ] **Step 5: Commit**

```bash
mise run lint
git add -A
git commit -m "feat(engine): 8.3.24 reaches a curādi root's own n

The search is confined to ANGA for a curādi root, so the 7.1.3 n of
corayanti stays out of reach. 10c's √gandh gains 8.3.24 → 8.4.58 on its 36
branches, as vidyut credits; its forms are unchanged."
```

---

## Task 4: The rows, their goldens, and every assertion they move

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini/tests/paradigm/data/curadi.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`
- Modify: `crates/panini/tests/trace/curadi.rs`
- Modify: `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes: everything from Tasks 2 and 3; the trace helpers `cell_trace`, `at` and `credited`; `panini_prakriya::derive`.
- Produces: the totals Task 5 documents (242 / 12996 / 14660).

- [ ] **Step 1: Update the data-layer assertions (failing)**

In `crates/panini-data/src/lib.rs`'s `tests` module:
- `assert_eq!(dhatus().len(), 150);` → `assert_eq!(dhatus().len(), 242);`
- Rename `fn curadi_rows_are_the_forty_seven_curated_roots` to `fn curadi_rows_are_the_one_hundred_thirty_nine_curated_roots`. In its comment, replace `        // The gaṇa is OPEN at 47 of its 509 dhātupāṭha rows.` with

```rust
        // Slice 10e adds ninety-two adanta roots (`10.0108 mArga` and
        // `10.0389 kaTa` … `10.0492 Deka`): 6.4.48 deletes the final `a`
        // before ṇic, and 7.2.116 / 7.3.86 decline on its `Tag::AtLopa`.
        // Eighty-three are ubhayapadī by 1.3.74; the nine ā-garvīya
        // (`AA_GARVIYA`) are ātmanepadī by 10.0497. The gaṇa is OPEN at 139
        // of its 509 dhātupāṭha rows.
```

  The 92 entries of its expected list are inserted in Step 6.

Also in `lib.rs`, the `pada` field doc: replace `re-derives 102 of these 150` with `re-derives 102 of these 242`, and replace

```rust
    /// exception, ten curādi rows' are 1.3.74's and 37 ākusmīya rows' are
    /// the gaṇasūtra 10.0496's, each asserted explicitly from both sides, the
    /// same way
```

with

```rust
    /// exception, ninety-three curādi rows' are 1.3.74's, 37 ākusmīya rows'
    /// the gaṇasūtra 10.0496's and nine ā-garvīya rows' the gaṇasūtra
    /// 10.0497's, each asserted explicitly from both sides, the same way
```

Check: 102 + 1 (√bhuj) + 93 + 37 + 9 = 242.

- [ ] **Step 2: Update the trace tests (failing)**

**(a)** In `crates/panini/tests/trace/juhotyadi.rs`, in the test Task 3 renamed, replace

```rust
    // Since slice 10e it also admits a curādi root's own `n` before a jhal:
    // 10c's √gandh.
    const CURADI: [&str; 1] = ["10.0204"];
```

with

```rust
    // Since slice 10e it also admits a curādi root's own `n` before a jhal:
    // 10c's √gandh and seven adanta roots, on every live branch.
    const CURADI: [&str; 8] = [
        "10.0204", "10.0433", "10.0460", "10.0467", "10.0471", "10.0472", "10.0473", "10.0474",
    ];
```

and replace

```rust
    // √gandh's 36 ātmanepada branches.
    assert_eq!(curadi.len(), 36);
```

with

```rust
    // √gandh's 36 ātmanepada branches and the seven ubhayapadī roots' 78 each.
    assert_eq!(curadi.len(), 36 + 7 * 78);
```

In `shcutva_off_jan_is_credited_exactly_as_before_3f3`, replace

```rust
    // does would change its form, which the goldens hold.
```

with

```rust
    // does would change its form, which the goldens hold. Slice 10e adds the
    // three ch-initial adanta curādi roots (`Cidra`, `Ceda`, `Cada`), whose laṅ
    // aṭ takes the same forward-arm tuk (acCidrayat).
```

and replace

```rust
            *number == "07.0003" || *number == "07.0008",
            "8.4.40 credited on {number}"
        );
    }
    assert_eq!(off_jan.len(), 54);
```

with

```rust
            ["07.0003", "07.0008", "10.0469", "10.0480", "10.0481"].contains(number),
            "8.4.40 credited on {number}"
        );
    }
    let adanta = off_jan.iter().filter(|(n, _)| n.starts_with("10.")).count();
    assert_eq!(off_jan.len() - adanta, 54);
    // Each ch-initial adanta root: laṅ's 18 cells plus its one 8.4.56 fork.
    assert_eq!(adanta, 3 * 19);
```

**(b)** In `crates/panini/tests/trace/curadi.rs`:
- Replace the module doc's last line, `//! 7.2.116.`, with

```rust
//! 7.2.116. An adanta root's has 6.4.48 after 3.4.114 and neither 7.2.116
//! nor 7.3.86; an ā-garvīya root's also opens with the gaṇasūtra 10.0497,
//! and has no pada sūtra.
```

- Replace `use panini_data::{AKUSMIYA, Gana, JNAPADI, Lakara, Pada, Purusha, Vacana, dhatus};` with

```rust
use panini_data::{
    AA_GARVIYA, AKUSMIYA, Gana, JNAPADI, Lakara, Pada, PadaAssignment, Purusha, Vacana, dhatus,
};
```

- In `a_kusmad_is_credited_on_exactly_the_akusmiya_cells`, replace `    // And 1.3.74 never reaches them: its credits stay on the ten `Nic` rows.` with

```rust
    // And 1.3.74 never reaches them: its credits stay on the 93 `Nic` rows,
    // read from the curated `pada` column (ten before slice 10e, listed
    // literally until then).
```

  and replace

```rust
    for (number, _) in credited("1.3.74") {
        assert!(
            [
                "10.0001", "10.0010", "10.0033", "10.0255", "10.0118", "10.0119", "10.0120",
                "10.0121", "10.0122", "10.0123",
            ]
            .contains(&number),
            "1.3.74 credited on {number}"
        );
    }
```

  with

```rust
    let nic: Vec<&str> = dhatus()
        .iter()
        .filter(|d| d.gana == Gana::Curadi && d.pada == PadaAssignment::Nic)
        .map(|d| d.dhatupatha)
        .collect();
    assert_eq!(nic.len(), 93, "curated 1.3.74 rows");
    for (number, _) in credited("1.3.74") {
        assert!(nic.contains(&number), "1.3.74 credited on {number}");
    }
```

- Append at the end of the file:

```rust

/// The curated adanta rows: curādi, with an `a`-final code. No row curated
/// before slice 10e has one, so this is exactly 10e's ninety-two.
fn adanta_rows() -> Vec<&'static str> {
    dhatus()
        .iter()
        .filter(|d| d.gana == Gana::Curadi && d.code.ends_with('a'))
        .map(|d| d.dhatupatha)
        .collect()
}

#[test]
#[allow(non_snake_case)]
fn kaTayati_trace_deletes_the_a_and_skips_the_vrddhi() {
    // kaTa P laT P.E. 6.4.48 deletes the final `a` once 3.4.114 has made ṇic
    // ārdhadhātuka (`kaT`); by 1.1.57 the deleted `a` still stands, so
    // 7.2.116 does not lengthen `kaT`'s `a` (not *kATayati*).
    let (text, t) = cell_trace(
        "10.0389",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "kaTayati", "got {t:?}");
    assert_eq!(
        t,
        [
            "3.1.25", "1.3.9", "3.4.114", "6.4.48", "3.1.32", "1.3.78", "3.4.78", "1.3.9",
            "3.1.68", "1.3.9", "7.3.84", "6.1.78",
        ],
    );
    assert!(at(&t, "3.4.114") < at(&t, "6.4.48"), "got {t:?}");
    assert!(at(&t, "6.4.48") < at(&t, "3.1.32"), "got {t:?}");
}

#[test]
fn kuhayate_trace_opens_with_a_garvad() {
    // kuha A laT P.E. The gaṇasūtra 10.0497 settles the pada before ṇic
    // exists; 6.4.48 deletes the `a`, and 7.3.86 does not guṇate `kuh`'s
    // laghu `u` (not *kohayate*). No pada sūtra is credited.
    let (text, t) = cell_trace(
        "10.0443",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "kuhayate", "got {t:?}");
    assert_eq!(
        t,
        [
            "10.0497", "3.1.25", "1.3.9", "3.4.114", "6.4.48", "3.1.32", "3.4.78", "1.2.4",
            "3.4.79", "3.1.68", "1.3.9", "7.3.84", "6.1.78",
        ],
    );
}

#[test]
fn ato_lopa_is_credited_on_exactly_the_adanta_cells() {
    // 6.4.48 fires on every live branch of the 92 adanta rows — 83
    // ubhayapadī roots × 78 branches and 9 ā-garvīya roots × 36 — and
    // nowhere else. Neither 7.2.116 nor 7.3.86 is credited on any of them:
    // that is 1.1.57's block, held corpus-wide. Goldens ignore traces, so
    // this is also what holds 6.4.48 inert on the 150 prior roots.
    let adanta = adanta_rows();
    assert_eq!(adanta.len(), 92);
    let hits = credited("6.4.48");
    assert_eq!(hits.len(), 83 * 78 + 9 * 36);
    for (number, _) in &hits {
        assert!(adanta.contains(number), "6.4.48 credited on {number}");
    }
    for sutra in ["7.2.116", "7.3.86"] {
        for (number, _) in credited(sutra) {
            assert!(!adanta.contains(&number), "{sutra} credited on {number}");
        }
    }
}

#[test]
fn a_garvad_is_credited_on_exactly_the_a_garviya_cells() {
    // 10.0497 fires on every ātmanepada cell of the nine curated ā-garvīya
    // rows, one branch each, and nowhere else: every credit's number lies in
    // the positional `AA_GARVIYA` range. Their parasmaipada derives only
    // blocked branches, with nothing recorded: the block is 10.0497's.
    let hits = credited("10.0497");
    assert_eq!(hits.len(), 9 * 36);
    for (number, _) in &hits {
        assert!(AA_GARVIYA.contains(number), "10.0497 credited on {number}");
    }
    let rows: Vec<_> = dhatus()
        .iter()
        .filter(|d| d.gana == Gana::Curadi && AA_GARVIYA.contains(&d.dhatupatha))
        .collect();
    assert_eq!(rows.len(), 9, "curated ā-garvīya rows");
    for d in rows {
        for lakara in [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin] {
            for purusha in [Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama] {
                for vacana in [Vacana::Eka, Vacana::Dvi, Vacana::Bahu] {
                    let ps = derive(d, lakara, Pada::Parasmaipada, purusha, vacana);
                    let cell = format!("{} {lakara:?} {purusha:?} {vacana:?}", d.dhatupatha);
                    assert!(!ps.is_empty(), "{cell}");
                    for p in &ps {
                        assert!(p.blocked, "{cell}: {}", p.text());
                        assert!(p.log.is_empty(), "{cell}: {:?}", p.log);
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 3: Update the paradigm assertions and the `check()` tests (failing)**

In `crates/panini/tests/paradigm/main.rs` (each `old` occurs exactly once; confirm with `grep -c`):

- `/// `ALTERNATES` is otherwise 1166 bare strings` → `1664`.
- `/// 6696 cells total (744 root×lakāra blocks × 9), of which 5848 hold exactly one form, 624 hold two, 177 hold three` → `/// 12996 cells total (1444 root×lakāra blocks × 9), of which 11816 hold exactly one form, 790 hold two, 343 hold three`
- `and the six jñapādi roots', new in slice 10d, each by` → `and the six jñapādi roots', new in slice 10d, and the eighty-three ubhayapadī adanta roots', new in slice 10e, each by`
- `/// itself has 1166 rows, keyed 182 `8.4.56`, 174 `7.1.35`, 174 `7.1.35+8.4.56`,` → `/// itself has 1664 rows, keyed 348 `8.4.56`, 340 `7.1.35`, 340 `7.1.35+8.4.56`,`
- After the 10d paragraph's last lines, `/// 432 new cells, thirty-six new rows. The gaṇa is OPEN at 47 of its 509` / `/// rows.`, insert:

```rust
///
/// Slice 10e curates ninety-two adanta roots (`10.0108 mArga` and `10.0389
/// kaTa` … `10.0492 Deka`). 6.4.48 *ato lopaḥ ārdhadhātuke* deletes the final
/// `a` before ṇic, and by 1.1.57 neither 7.2.116 nor 7.3.86 reads the
/// upadhā (*kathayati*, *kuhayate*). Eighty-three are ubhayapadī by 1.3.74
/// and fork exactly where √cur does; the nine ā-garvīya (`10.0440 pada` …
/// `10.0448 satra`) are ātmanepadī by the gaṇasūtra 10.0497, one form per
/// cell. 6300 new cells, 498 new rows. The gaṇa is OPEN at 139 of its 509
/// rows.
```

- In `derivation_set_shape_matches_the_audited_numbers`:
  - `assert_eq!(total_cells, 6696, "744 root×lakāra blocks × 9 cells each");` → `assert_eq!(total_cells, 12996, "1444 root×lakāra blocks × 9 cells each");`
  - `assert_eq!(ones, 5848, "one-form cells");` → `11816`
  - `assert_eq!(twos, 624, "two-form cells");` → `790`
  - `        threes, 177,` → `        threes, 343,`
  - In the threes message, replace `7.1.35/8.4.56; and — new in slice 10d — the six jñapādi roots', the same way"` with `7.1.35/8.4.56; and — new in slice 10d — the six jñapādi roots', the same way; and — new \` / `         in slice 10e — the eighty-three ubhayapadī adanta roots', the same way"`
  - `assert_eq!(ALTERNATES.len(), 1166, "ALTERNATES row count");` → `1664`
  - `assert_eq!(key_count("8.4.56"), 182, "8.4.56-only alternates");` → `348`
  - `assert_eq!(key_count("7.1.35"), 174, "7.1.35-only alternates");` → `340`
  - `assert_eq!(key_count("7.1.35+8.4.56"), 174, "7.1.35+8.4.56 alternates");` → `340`
- In `pada_ambiguous_surfaces_are_exactly_these`, after the comment lines `    // any pre-slice surface. The ākusmīya roots, ātmanepada-only, contribute` / `    // nothing.`, add (same comment block):

```rust
    // Slice 10e's eighty-three ubhayapadī adanta roots contribute the same
    // four-surface shape each (`akaTayata`, `kaTayatAm`, `kaTayetAm`,
    // `kaTayeta`), except `10.0396 raha` and `10.0405 caha`, whose surfaces
    // 10d's √rah and √cah already contributed — 324 more, taking the set from
    // ninety-six to 420. The nine ā-garvīya roots, ātmanepada-only, contribute
    // nothing; nor do `kUwa` and `vizka`, whose ātmanepada the ākusmīya `kUwa~`
    // and `vizka~` share, beyond their own four.
```

  The expected list itself is replaced in Step 8, from the measured set.
- 10d's `curadi_analyses_its_jnapadi_forms`: in its doc, replace `/// 6.4.92. `ajYapayata` is pada-ambiguous within √jñap (laṅ parasmaipada` with

```rust
/// 6.4.92 — except that since slice 10e √cah's and √rah's surfaces are
/// shared with the adanta homographs `10.0405 caha` and `10.0396 raha`
/// (see `curadi_analyses_its_adanta_forms`): two analyses, of which the mit
/// one is checked here. `ajYapayata` is pada-ambiguous within √jñap (laṅ parasmaipada
```

  and in its body replace

```rust
            let r = engine.check(form);
            assert!(matches!(r.verdict, Verdict::Valid), "{form}");
            assert_eq!(r.analyses.len(), 1, "{form}");
            let a = &r.analyses[0];
            assert_eq!(a.dhatu, dhatu, "{form}");
            assert_eq!(a.pada, pada, "{form}");
            assert_mit(form, &ids_of(a));
```

  with

```rust
            let r = engine.check(form);
            assert!(matches!(r.verdict, Verdict::Valid), "{form}");
            let homograph = matches!(dhatu, "cah" | "rah");
            assert_eq!(r.analyses.len(), if homograph { 2 } else { 1 }, "{form}");
            let a = r.analyses.iter().find(|a| a.dhatu == dhatu).unwrap();
            assert_eq!(a.pada, pada, "{form}");
            assert_mit(form, &ids_of(a));
```

- Immediately after the closing `}` of `fn curadi_analyses_its_jnapadi_forms`, add:

```rust

/// Slice 10e's `check()` witnesses, one per shape class of the spec's
/// enumeration plus every homograph pair. Each adanta analysis credits
/// 6.4.48 and neither 7.2.116 nor 7.3.86 (1.1.57's block); each ā-garvīya
/// one opens with 10.0497 and credits no pada sūtra. The goldens were
/// grepped for every witness first: the single-analysis forms are their own
/// row's alone, and each homograph surface is exactly its two rows'.
/// The shapes the block prevents, and the anusvāra rows' pre-8.3.24 shape,
/// derive nothing.
#[test]
fn curadi_analyses_its_adanta_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    let has = |ids: &[String], id: &str| ids.iter().any(|i| i == id);
    let assert_adanta = |form: &str, ids: &[String]| {
        assert!(has(ids, "6.4.48"), "{form}: {ids:?}");
        assert!(!has(ids, "7.2.116"), "{form}: {ids:?}");
        assert!(!has(ids, "7.3.86"), "{form}: {ids:?}");
    };
    // (form, root, pada, an id the class adds beyond 6.4.48)
    for (form, dhatu, pada, extra) in [
        ("kaTayati", "kaTa", Pada::Parasmaipada, None),
        ("gaRayati", "gaRa", Pada::Parasmaipada, None),
        ("guRayati", "guRa", Pada::Parasmaipada, None),
        ("kuRayate", "kuRa", Pada::Atmanepada, None),
        ("mArgayARi", "mArga", Pada::Parasmaipada, Some("8.4.2")),
        ("Onayan", "Una", Pada::Parasmaipada, Some("6.4.72")),
        ("acCidrayan", "Cidra", Pada::Parasmaipada, Some("6.1.73")),
        ("saNketayati", "sanketa", Pada::Parasmaipada, Some("8.4.58")),
        ("daRqayate", "danqa", Pada::Atmanepada, Some("8.4.58")),
        ("aMsayati", "ansa", Pada::Parasmaipada, Some("8.3.24")),
        ("padayate", "pada", Pada::Atmanepada, Some("10.0497")),
        ("gfhayate", "gfha", Pada::Atmanepada, Some("10.0497")),
        ("kuhayate", "kuha", Pada::Atmanepada, Some("10.0497")),
        ("ArTayata", "arTa", Pada::Atmanepada, Some("10.0497")),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), 1, "{form}");
        let a = &r.analyses[0];
        assert_eq!(a.dhatu, dhatu, "{form}");
        assert_eq!(a.pada, pada, "{form}");
        let ids = ids_of(a);
        assert_adanta(form, &ids);
        if let Some(id) = extra {
            assert!(has(&ids, id), "{form} {id}: {ids:?}");
        }
        if extra == Some("10.0497") {
            assert_eq!(ids[0], "10.0497", "{form}: {ids:?}");
            for absent in ["1.3.12", "1.3.66", "1.3.72", "1.3.74", "1.3.78"] {
                assert!(!has(&ids, absent), "{form} {absent}: {ids:?}");
            }
        }
    }
    // The homograph pairs: the adanta row and its curated twin.
    for (form, adanta, twin, twin_id) in [
        ("rahayati", "raha", "rah", "6.4.92"),
        ("cahayati", "caha", "cah", "6.4.92"),
        ("kUwayate", "kUwa", "kUw", "10.0496"),
        ("vizkayate", "vizka", "vizk", "10.0496"),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        let mut roots: Vec<&str> = r.analyses.iter().map(|a| a.dhatu.as_str()).collect();
        roots.sort_unstable();
        let mut want = [adanta, twin];
        want.sort_unstable();
        assert_eq!(roots, want, "{form}");
        for a in &r.analyses {
            let ids = ids_of(a);
            if a.dhatu == adanta {
                assert_adanta(form, &ids);
            } else {
                assert!(has(&ids, twin_id), "{form} {twin}: {ids:?}");
                assert!(!has(&ids, "6.4.48"), "{form} {twin}: {ids:?}");
            }
        }
    }
    for form in [
        "kATayati",
        "gARayati",
        "kohayate",
        "goRayati",
        "sanketayati",
        "padayati",
        "kuhayati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
```

- [ ] **Step 4: Run the tests to see them fail**

Run: `mise exec -- cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .*FAILED" | head -20`
Expected failures:
- `curated_roots_have_expected_ganas_and_padas` (150 vs 242);
- `curadi_rows_are_the_one_hundred_thirty_nine_curated_roots`;
- `derivation_set_shape_matches_the_audited_numbers`;
- `curadi_analyses_its_adanta_forms`;
- `kaTayati_trace_deletes_the_a_and_skips_the_vrddhi`;
- `kuhayate_trace_opens_with_a_garvad`;
- `ato_lopa_is_credited_on_exactly_the_adanta_cells`;
- `a_garvad_is_credited_on_exactly_the_a_garviya_cells`;
- `a_kusmad_is_credited_on_exactly_the_akusmiya_cells` (10 vs 93);
- `nas_capadantasya_is_credited_only_on_rudhadi_dhan_jan_and_curadi_roots`;
- `shcutva_off_jan_is_credited_exactly_as_before_3f3`;
- `curadi_analyses_its_jnapadi_forms` (*rahayati* has one analysis, not yet two).

`pada_ambiguous_surfaces_are_exactly_these` still passes: no row exists yet.

- [ ] **Step 5: Generate the 92 rows**

Create `/tmp/vidyut-full/slice10e/gen_rows_10e.py` if it is missing:

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10e — emit the 92 adanta `Dhatu` rows and the matching
`curadi_rows_are_…` list entries from the vendored dhātupāṭha. Run from the
worktree root; writes rows.rs and rowlist.rs into the directory given."""
import sys, textwrap

OUT = sys.argv[1]
SLP = {'a':'a','A':'ā','i':'i','I':'ī','u':'u','U':'ū','f':'ṛ','F':'ṝ','x':'ḷ','e':'e','E':'ai','o':'o','O':'au','M':'ṃ','H':'ḥ',
       'k':'k','K':'kh','g':'g','G':'gh','N':'ṅ','c':'c','C':'ch','j':'j','J':'jh','Y':'ñ','w':'ṭ','W':'ṭh','q':'ḍ','Q':'ḍh','R':'ṇ',
       't':'t','T':'th','d':'d','D':'dh','n':'n','p':'p','P':'ph','b':'b','B':'bh','m':'m','y':'y','r':'r','l':'l','v':'v','S':'ś','z':'ṣ','s':'s','h':'h'}
iast = lambda s: ''.join(SLP[c] for c in s)
OPTIONAL_NIC = {'10.0400', '10.0451', '10.0456', '10.0449'}  # pata, mUtra, katra, garva
HOMOGRAPH = {
    '10.0396': "Shares every form with the mit `10.0122 raha~` (slice 10d).",
    '10.0405': "Shares every form with the mit `10.0120 caha~` (slice 10d).",
    '10.0432': "Its ātmanepada shares every form with the ākusmīya `10.0225 kUwa~`.",
    '10.0486': "Its ātmanepada shares every form with the ākusmīya `10.0207 vizka~`.",
}
rows = []
for line in open('data/dhatupatha.tsv'):
    f = line.rstrip('\n').split('\t')
    if len(f) < 3 or not f[0].startswith('10.'):
        continue
    n, u, artha = f[0], f[1], f[2]
    if u.endswith('a') and not u.endswith('a~') and n not in OPTIONAL_NIC:
        rows.append((n, u, artha))
assert len(rows) == 92, len(rows)
out, lst = [], []
for n, u, artha in rows:
    ag = "10.0440" <= n <= "10.0449"
    pada = "AaGarviya" if ag else "Nic"
    c = (f"{n} `{u}` {artha} (√{iast(u)}). Adanta: 6.4.48 ato lopaḥ deletes the "
         "final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still "
         "stands for 7.2.116 and 7.3.86, which decline. ")
    c += "Ātmanepadī by the gaṇasūtra 10.0497." if ag else "Ubhayapadī by 1.3.74."
    if n in HOMOGRAPH:
        c += " " + HOMOGRAPH[n]
    c += " Slice 10e."
    com = '\n'.join('        // ' + l for l in textwrap.wrap(c, 72))
    out.append(f"    Dhatu {{\n{com}\n        dhatupatha: \"{n}\",\n        code: \"{u}\",\n"
               f"        gana: Gana::Curadi,\n        pada: PadaAssignment::{pada},\n"
               f"        artha: \"{artha}\",\n    }},\n")
    lst.append(f'                ("{n}", "{u}", PadaAssignment::{pada}),\n')
open(f'{OUT}/rows.rs', 'w').write(''.join(out))
open(f'{OUT}/rowlist.rs', 'w').write(''.join(lst))
print(f"{len(rows)} rows, {sum(1 for r in rows if '10.0440' <= r[0] <= '10.0449')} ā-garvīya")
```

```bash
GEN="$(mktemp -d)"
python3 /tmp/vidyut-full/slice10e/gen_rows_10e.py "$GEN"      # 92 rows, 9 ā-garvīya
sha256sum "$GEN/rows.rs" "$GEN/rowlist.rs"
```

Expected hashes:
- `rows.rs`: `458292bb5bb22b80460df1fb7621596ab0db4fad002c9d2cccd0d3781905eeb2`;
- `rowlist.rs`: `a96a630b9dbbebf3231f4943021c85afb1dabf8504cef2817873f31c1a960c31`.

If either differs, stop and report.

- [ ] **Step 6: Insert the rows and the list entries**

```bash
GEN="$GEN" python3 - <<'EOF'
import os
GEN = os.environ['GEN']
p = 'crates/panini-data/src/lib.rs'
s = open(p).read()
a = '''        artha: "kutsitasmaye",
    },
];'''
assert s.count(a) == 1
s = s.replace(a, '''        artha: "kutsitasmaye",
    },
''' + open(f'{GEN}/rows.rs').read() + '];')
b = '''                ("10.0236", "kusm", PadaAssignment::Akusmiya),
'''
assert s.count(b) == 1
s = s.replace(b, b + open(f'{GEN}/rowlist.rs').read())
open(p, 'w').write(s)
EOF
mise exec -- cargo test -p panini-data 2>&1 | grep "test result" | head -1
```

Expected: `panini-data` 25 passed. The rows go after `10.0236 kusm`, the last curādi row, in dhātupāṭha order among themselves (the data layer orders curādi by slice). `curated_pada_agrees_with_upadesha_markers` and `dhatupatha_numbers_resolve_upstream` now cover all 92.

- [ ] **Step 7: Generate the goldens and insert them**

`/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes its `panini` and `panini-data` dev-deps at `/workspace/crates`, the `main` checkout. Repoint them at this worktree before generating, or the generator reads the pre-slice rows.

Create `/tmp/vidyut-full/vidyut-prakriya/examples/curadi_goldens_10e.rs` if it is missing:

```rust
//! THROWAWAY: slice 10e — emit the 92 adanta rows' goldens from the
//! engine, asserting every cell's form set equals vidyut's first.
use panini::Panini;
use panini_data::{dhatus, Lakara as L, Pada, Purusha as P, Vacana as V};
use vidyut_prakriya::args::{DhatuPada, Lakara, Prayoga, Purusha, Tinanta, Vacana};
use vidyut_prakriya::{Dhatupatha, Vyakarana};
const VIKALPA_RULES: &[&str] = &[
    "7.1.35", "3.4.111", "7.3.86", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56", "6.4.115",
    "6.4.117", "6.4.116", "6.4.43",
];
fn main() {
    let dp = Dhatupatha::from_path("/tmp/vidyut-full/vidyut-prakriya/data/dhatupatha.tsv").unwrap();
    let v = Vyakarana::builder().build();
    let e = Panini::new();
    let laks = [(L::Lat, Lakara::Lat, "laT"), (L::Lan, Lakara::Lan, "laN"), (L::Lot, Lakara::Lot, "loT"), (L::VidhiLin, Lakara::VidhiLin, "viDiliN")];
    let cells = [(P::Prathama, Purusha::Prathama, V::Eka, Vacana::Eka), (P::Prathama, Purusha::Prathama, V::Dvi, Vacana::Dvi), (P::Prathama, Purusha::Prathama, V::Bahu, Vacana::Bahu),
                 (P::Madhyama, Purusha::Madhyama, V::Eka, Vacana::Eka), (P::Madhyama, Purusha::Madhyama, V::Dvi, Vacana::Dvi), (P::Madhyama, Purusha::Madhyama, V::Bahu, Vacana::Bahu),
                 (P::Uttama, Purusha::Uttama, V::Eka, Vacana::Eka), (P::Uttama, Purusha::Uttama, V::Dvi, Vacana::Dvi), (P::Uttama, Purusha::Uttama, V::Bahu, Vacana::Bahu)];
    let (mut par, mut alt) = (String::new(), String::new());
    let (mut ncells, mut nforms, mut ndiff) = (0, 0, 0);
    // The new rows: curādi, and an `a`-final code (no pre-slice code ends in `a`).
    for d in dhatus().iter().filter(|d| d.gana == panini_data::Gana::Curadi && d.code.ends_with('a')) {
        let num = d.dhatupatha;
        let vd = dp.get(num).unwrap();
        for (pada, vpada, pname) in [(Pada::Parasmaipada, DhatuPada::Parasmaipada, "Parasmaipada"), (Pada::Atmanepada, DhatuPada::Atmanepada, "Atmanepada")] {
            if !d.pada.padas().contains(&pada) { continue; }
            for (el, vl, lname) in laks {
                let mut row = vec![];
                for (i, (ep, vp, ev, vv)) in cells.iter().enumerate() {
                    ncells += 1;
                    let bs = e.derive(d, el, pada, *ep, *ev);
                    let live: Vec<_> = bs.iter().filter(|b| !b.blocked).collect();
                    let mut a: Vec<String> = live.iter().map(|b| b.text()).collect();
                    a.sort(); a.dedup();
                    let t = Tinanta::builder().dhatu(vd.clone()).prayoga(Prayoga::Kartari).purusha(*vp).vacana(*vv).lakara(vl).pada(vpada).build().unwrap();
                    let mut b: Vec<String> = v.derive_tinantas(&t).iter().map(|p| p.text()).collect();
                    b.sort(); b.dedup();
                    if a != b { ndiff += 1; println!("DIFF {num} {} {pname} {lname} {i}: engine={a:?} vidyut={b:?}", d.code); }
                    nforms += a.len();
                    let first = bs[0].text();
                    row.push(first.clone());
                    let mut seen = vec![first];
                    for br in &live {
                        let f = br.text();
                        if seen.contains(&f) { continue; }
                        seen.push(f.clone());
                        let key: Vec<&str> = br.log.iter().map(|s| s.sutra.as_str()).filter(|s| VIKALPA_RULES.contains(s)).collect();
                        alt.push_str(&format!("    (\n        \"{num}\",\n        \"{lname}\",\n        Pada::{pname},\n        {i},\n        \"{f}\",\n        \"{}\",\n    ),\n", key.join("+")));
                    }
                }
                par.push_str(&format!("    (\n        \"{num}\",\n        \"{lname}\",\n        Pada::{pname},\n        [\n{}        ],\n    ),\n", row.iter().map(|f| format!("            \"{f}\",\n")).collect::<String>()));
            }
        }
    }
    std::fs::write("/tmp/vidyut-full/goldens_10e_paradigm.rs", par).unwrap();
    std::fs::write("/tmp/vidyut-full/goldens_10e_alternates.rs", alt).unwrap();
    println!("{ncells} cells, {nforms} forms, {ndiff} differences");
    assert_eq!(ndiff, 0);
}
```

```bash
WT="$(git rev-parse --show-toplevel)"
V=/tmp/vidyut-full/vidyut-prakriya
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml   # must point at $WT/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example curadi_goldens_10e 2>/dev/null | tail -3)
sha256sum /tmp/vidyut-full/goldens_10e_paradigm.rs /tmp/vidyut-full/goldens_10e_alternates.rs
```

Expected:
- the generator prints `6300 cells, 6798 forms, 0 differences`;
- `goldens_10e_paradigm.rs`: `d1d499297eefac453476bf7601f9721d7ef365111c6f9c57d89fb58cdd9e92a4` (700 rows);
- `goldens_10e_alternates.rs`: `ca4170419ca1042445ff0e1b9647f541d80a5f6a192ad46cf44c66525b2dadca` (498 rows).

Any `DIFF` line or other hash: stop and report, and edit nothing. Leave the dev-deps pointed at this worktree for Task 5; Task 5 restores them.

Insert both blocks just before each static's closing `];`:

```bash
python3 - <<'EOF'
p = 'crates/panini/tests/paradigm/data/curadi.rs'
s = open(p).read()
par = open('/tmp/vidyut-full/goldens_10e_paradigm.rs').read()
alt = open('/tmp/vidyut-full/goldens_10e_alternates.rs').read()
i = s.index('pub const ALTERNATES')
head, tail = s[:i], s[i:]
k = head.rindex('];'); head = head[:k] + par + head[k:]
k = tail.rindex('];'); tail = tail[:k] + alt + tail[k:]
open(p, 'w').write(head + tail)
EOF
mise run fmt
```

- [ ] **Step 8: Measure the pada-ambiguous set and pin it**

The test's own comment says the set is "measured (never hand-picked)". Read it off the failure:

```bash
SET="$(mktemp)"
mise exec -- cargo test -p panini --test paradigm pada_ambiguous 2>&1 | grep "^  left:" | sed 's/^  left: //' > "$SET"
SET="$SET" python3 - <<'EOF'
import hashlib, json, os
amb = json.load(open(os.environ['SET']))
assert len(amb) == 420, len(amb)
h = hashlib.sha256('\n'.join(amb).encode()).hexdigest()
assert h == '8f78d23fdf37a0e5521b167e199347b3ea4a33c5a8a5300ffab71da7db448a5b', h
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
i = s.index("fn pada_ambiguous_surfaces_are_exactly_these")
j = s.index("        both,\n        vec![", i)
k = s.index("        ]", j)
s = s[:j] + "        both,\n        vec![\n" + "".join(f'            "{x}",\n' for x in amb) + s[k:]
open(p, 'w').write(s)
print("pinned", len(amb))
EOF
```

Expected: `pinned 420`. If the count or the hash assertion fails, stop and report.

- [ ] **Step 9: Run the full suite**

```bash
mise run fmt
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS at 12996 cells, with `panini-data` 25, `panini-prakriya` 411, `trace` 203 and `paradigm` 24.

Grep the goldens for the `check()` witnesses, which the test also enforces:

```bash
for f in kaTayati gaRayati guRayati kuRayate mArgayARi Onayan acCidrayan saNketayati daRqayate aMsayati padayate gfhayate kuhayate ArTayata rahayati cahayati kUwayate vizkayate kATayati gARayati kohayate goRayati sanketayati padayati kuhayati; do
  printf "%s: %s\n" $f "$(grep -c "\"$f\"" crates/panini/tests/paradigm/data/*.rs | grep -v ':0' | tr '\n' ' ')"; done
```

Expected:
- the fourteen single-analysis witnesses appear only in `curadi.rs`, once each;
- *rahayati*, *cahayati*, *kUwayate* and *vizkayate* appear twice in `curadi.rs`;
- the seven Invalid shapes print nothing.

- [ ] **Step 10: Commit**

```bash
mise run lint
git add -A
git commit -m "feat(data): curādi's adanta roots — ninety-two rows, 83 ubhayapadī by 1.3.74, nine ā-garvīya by 10.0497

6696 → 12996 cells, 7862 → 14660 forms, ALTERNATES 1166 → 1664, 150 → 242
roots; pada-ambiguous surfaces 96 → 420. 6.4.48 is credited on exactly
the 6798 adanta branches and 10.0497 on the 324 ā-garvīya ones; 7.2.116 and
7.3.86 on none. Goldens generated cell-by-cell equal to vidyut."
```

---

## Task 5: Audit, prior-trace diff, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (audit prose), `crates/panini-prakriya/src/tinanta/guna.rs` (one comment), `crates/panini-data/src/lib.rs` (two comments), `docs/superpowers/specs/2026-10-02-curadi-gana-10d-design.md`

**Interfaces:**
- Consumes: the finished engine, data and goldens. Produces no symbols.

- [ ] **Step 1: Update the audit harness**

In `tools/audit/panini_full_audit.rs` (each `old` occurs once):
- Replace

```rust
//! What it compares: for each of the 150 curated roots, for each pada the root
//! admits (two apiece for the thirty-six roots that admit both padas —
//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, and ten curādi roots by
//! 1.3.74), for each of the four
```

  with

```rust
//! What it compares: for each of the 242 curated roots, for each pada the root
//! admits (two apiece for the 119 roots that admit both padas —
//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, and ninety-three curādi
//! roots by 1.3.74), for each of the four
```

- `//! Corpus invariants, asserted: 150 roots, 6696 cells, 7862 forms. These are` → `242 roots, 12996 cells, 14660 forms.`
- ``//! (`derivation_set_shape_matches_the_audited_numbers`): 744 root×pada×lakāra`` → `1444`
- `//! blocks × 9 cells, plus 1166 `ALTERNATES` rows.` → `1664`
- `//! Optionally dump the full 6696-cell table:` → `12996-cell`
- `assert_eq!(roots_seen.len(), 150, "curated roots");` → `242`
- `assert_eq!(n_cells, 6696, "cells: 744 root×pada×lakāra blocks × 9");` → `assert_eq!(n_cells, 12996, "cells: 1444 root×pada×lakāra blocks × 9");`
- `assert_eq!(n_forms, 7862, "forms: 6696 cells + 1166 ALTERNATES rows");` → `assert_eq!(n_forms, 14660, "forms: 12996 cells + 1664 ALTERNATES rows");`

- [ ] **Step 2: The prior-trace diff and the audit**

The dev-deps still point at this worktree from Task 4 Step 7. Create `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_10e.rs` if it is missing:

```rust
//! THROWAWAY: slice 10e — dump every prior cell's live-branch credited-rule log.
use panini::Panini;
use panini_data::{Gana, Lakara as L, Purusha as P, Vacana as V};
fn main() {
    let panini = Panini::new();
    for d in panini_data::dhatus() {
        // 10e's rows: curādi with an `a`-final code. No prior code ends in `a`.
        if d.gana == Gana::Curadi && d.code.ends_with('a') { continue; }
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

```bash
WT="$(git rev-parse --show-toplevel)"
DUMP="$(mktemp -d)"
V=/tmp/vidyut-full/vidyut-prakriya
grep -n '^panini' $V/Cargo.toml   # must point at $WT/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10e 2>/dev/null > "$DUMP/branch.txt")
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml   # must point at /workspace/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10e 2>/dev/null > "$DUMP/main.txt")
wc -l < "$DUMP/main.txt"; wc -l < "$DUMP/branch.txt"            # 7862 and 7862
diff <(grep -v "^10.0204 " "$DUMP/main.txt") <(grep -v "^10.0204 " "$DUMP/branch.txt") && echo OTHERS-IDENTICAL
diff <(grep "^10.0204 " "$DUMP/main.txt" | sed 's/$/ 8.3.24 8.4.58/') <(grep "^10.0204 " "$DUMP/branch.txt") && echo GANDH-GAINS-THE-PAIR
grep -c "^10.0204 " "$DUMP/main.txt"                             # 36
```

Expected: `7862`, `7862`, `OTHERS-IDENTICAL`, `GANDH-GAINS-THE-PAIR`, `36`. If any diff prints a line, stop and report.

Then the audit. Repoint at this worktree again, copy the committed harness (never rewrite it), and run:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
cp tools/audit/panini_full_audit.rs $V/examples/
(cd $V && PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -3)
(cd $V && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -2)
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml
```

- Expected from the honest run: `AUDIT PASSED: 12996 cells, 14660 forms, zero differences.`
- Expected from the `entry` control: `AUDIT FAILED: 36 differing cells.`

Do not use `mise -C`. If the honest run shows differences, stop and report, and edit nothing.

- [ ] **Step 3: Record the audit**

In `tools/audit/README.md`, immediately under `## Last recorded result` and its blank line, add this entry above the 10d one. Set `<DATE>` from `date -u +%F`:

```markdown
<DATE>, curādi 10e slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 12996
cells / 14660 forms / 242 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole curādi 10e slice: ninety-two adanta rows
(`10.0108 mArga` and `10.0389 kaTa` … `10.0492 Deka`), eighty-three
ubhayapadī by 1.3.74 and nine ātmanepadī by the gaṇasūtra 10.0497 *ā garvād
ātmanepadinaḥ*. 6.4.48 *ato lopaḥ ārdhadhātuke* deletes their final `a`
before ṇic, and by 1.1.57 7.2.116 and 7.3.86 decline (*kathayati*, not
*kāthayati*). 8.3.24 *naś cāpadāntasya jhali* now reaches a curādi root's own
`n`: before that widening, the six anusvāra rows (*saṅketayati*, …) were the
slice's only differences, 432 cells. A main-vs-branch dump of every prior
cell's traces was byte-identical except √gandh's (`10.0204`) 36 branches,
each gaining the 8.3.24 → 8.4.58 pair vidyut also credits.

Totals: 242 = 150 + 92; 12996 = 6696 + 6300 (700 root×pada×lakāra blocks ×
9); 14660 = 7862 + 6300 + 498 new `ALTERNATES` rows (1166 → 1664), measured
via the harness's corpus block, not assumed.

```

- [ ] **Step 4: The doc sweep**

Create `/tmp/vidyut-full/slice10e/docsweep_10e.py` if it is missing. Its sha256 in the prototype was `9accd4821f42d48945b78afeec31b721a916c5f272a04a34deeb32209e5660f4`:

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10e's doc sweep, run from the worktree root. Every `old`
must occur exactly once in its file; the script checks all of them before
writing any file, and writes nothing if one fails."""
import sys

SPAN = object()  # (SPAN, start, end, new): replace start..end, keep end

ARCH = [
("| `sanadi.rs` | 10.0496, 10.0493, 3.1.25, 1.3.9, 3.4.114, 7.2.116, 6.4.92, 7.3.86, 3.1.32 — the ākusmīya pada and the jñapādi's mit-tva, then ṇic and its folding into the dhātu (curādi only) |",
 "| `sanadi.rs` | 10.0496, 10.0497, 10.0493, 3.1.25, 1.3.9, 3.4.114, 6.4.48, 7.2.116, 6.4.92, 7.3.86, 3.1.32 — the ākusmīya and ā-garvīya pada and the jñapādi's mit-tva, then ṇic, the adanta root's final `a`, and ṇic's folding into the dhātu (curādi only) |"),
("*mitāṃ hrasvaḥ*, right after 7.2.116 — 139 total).",
 "*mitāṃ hrasvaḥ*, right after 7.2.116 — 139 total — then curādi 10e's two:\n"
 "the gaṇasūtra 10.0497 *ā garvād ātmanepadinaḥ*, right after 10.0496, and\n"
 "6.4.48 *ato lopaḥ ārdhadhātuke*, right after 3.4.114, whose `Tag::AtLopa`\n"
 "7.2.116 and 7.3.86 decline on (1.1.57 *acaḥ parasmin pūrvavidhau*) — 141\n"
 "total)."),
("and curādi (10), **open** at 47 of its", "and curādi (10), **open** at 139 of its"),
("√jñap, √yam, √cah, √cap, √rah, √bal, slice 10d). gaṇa",
 "√jñap, √yam, √cah, √cap, √rah, √bal, slice 10d; ninety-two adanta roots, slice 10e). gaṇa"),
("forking 176 cells (loṭ\nprathama and madhyama eka across the 88 roots with a parasmaipada column —",
 "forking 342 cells (loṭ\nprathama and madhyama eka across the 171 roots with a parasmaipada column —"),
("`tu`/`hi` are parasmaipada endings, so the curated set's 62 ātmanepada-only",
 "`tu`/`hi` are parasmaipada endings, so the curated set's 71 ātmanepada-only"),
("roots never reach this guard, and the thirty-six roots that admit both",
 "roots never reach this guard, and the 119 roots that admit both"),
("√laḍ, √bhakṣ, √bhūṣ, √jñap, √yam, √cah, √cap, √rah and √bal by 1.3.74) reach it in their",
 "√laḍ, √bhakṣ, √bhūṣ, √jñap, √yam, √cah, √cap, √rah, √bal and slice 10e's\n"
 "eighty-three ubhayapadī adanta roots by 1.3.74) reach it in their"),
("88 + 62 = the 150 curated roots)", "171 + 71 = the 242 curated roots)"),
("forking 182 cells outright: laṅ and vidhiliṅ prathama eka across\nthose same 88 parasmaipada columns (159 of them",
 "forking 348 cells outright: laṅ and vidhiliṅ prathama eka across\nthose same 171 parasmaipada columns (325 of them"),
("10d's six jñapādi roots contribute both cells),",
 "10d's six jñapādi roots and 10e's eighty-three adanta roots contribute both cells),"),
("forking a further 176 (the same", "forking a further 342 (the same"),
]

README = [
(SPAN, "Ninety-six surfaces are pada-ambiguous, each of them a pinned cell in both padas\nat once: ",
 " — `rundDAm`, for instance,",
 "420 surfaces are pada-ambiguous, each of them a pinned cell in both padas\nat once"),
("`rahaya-` and `balaya-`. That enumeration is no\nlonger maintained by hand:\n`pada_ambiguous_surfaces_are_exactly_these` in\n`crates/panini/tests/paradigm/main.rs` walks `PARADIGM` and asserts exactly this\nset.",
 "`rahaya-` and `balaya-`, and slice 10e's eighty-three ubhayapadī adanta roots\n"
 "the same for `kaTaya-` and the rest, except `10.0396 raha` and `10.0405 caha`,\n"
 "whose surfaces 10d's √rah and √cah already supply. The enumeration is not\n"
 "maintained by hand: `pada_ambiguous_surfaces_are_exactly_these` in\n"
 "`crates/panini/tests/paradigm/main.rs` walks `PARADIGM` and asserts the whole\n"
 "set, all 420."),
("8.4.44 *śāt* exemption. *curādi* (10) is **open** at 47 of its 509",
 "8.4.44 *śāt* exemption. *curādi* (10) is **open** at 139 of its 509"),
("shortens back the upadhā 7.2.116 lengthened before ṇic, so *jñapayati*, not\n*jñāpayati*.\n",
 "shortens back the upadhā 7.2.116 lengthened before ṇic, so *jñapayati*, not\n"
 "*jñāpayati*. Slice 10e curated ninety-two adanta roots (`10.0108 mArga` and\n"
 "`10.0389 kaTa` … `10.0492 Deka`), whose upadeśa ends in a bare `a`: 6.4.48\n"
 "*ato lopaḥ ārdhadhātuke* deletes it before ṇic, and by 1.1.57 *acaḥ parasmin\n"
 "pūrvavidhau* the deleted `a` still stands for 7.2.116 and 7.3.86, so\n"
 "*kathayati*, not *kāthayati*, and *kuhayate*, not *kohayate*. Eighty-three are\n"
 "ubhayapadī by 1.3.74; nine are ātmanepadī by the gaṇasūtra 10.0497 *ā garvād\n"
 "ātmanepadinaḥ*, the engine's third non-Aṣṭādhyāyī rule. Six of them\n"
 "(√saṅketa, *saṅketayati*) carry their own `n` before a jhal, and 8.3.24\n"
 "*naś cāpadāntasya jhali*, until then rudhādi's and juhotyādi's, now reaches a\n"
 "curādi root's own `n` too.\n"),
("curated 150-root set,", "curated 242-root set,"),
("both correct — and in fact 848 of the 6696 cells hold more than one form: 624\nhold two, 177 hold three",
 "both correct — and in fact 1180 of the 12996 cells hold more than one form: 790\nhold two, 343 hold three"),
("√jan's, and — new in slice 10a — the four curādi roots', and — new in slice\n10d — the six jñapādi roots', each by",
 "√jan's, and — new in slice 10a — the four curādi roots', and — new in slice\n10d — the six jñapādi roots', and — new in slice 10e — the eighty-three\nubhayapadī adanta roots', each by"),
("padas — thirty-six roots that admit both padas in the curated set",
 "padas — 119 roots that admit both padas in the curated set"),
("√laḍ, √bhakṣ, √bhūṣ, √jñap, √yam, √cah, √cap, √rah and √bal by 1.3.74) derive a full",
 "√laḍ, √bhakṣ, √bhūṣ, √jñap, √yam, √cah, √cap, √rah, √bal and the\neighty-three ubhayapadī adanta roots by 1.3.74) derive a full"),
]

AGENTS = [
("(`crates/panini/tests/paradigm/`, 6696 cells, ten gaṇas, nine complete —",
 "(`crates/panini/tests/paradigm/`, 12996 cells, ten gaṇas, nine complete —"),
("at 47 after slice 10d curated the jñapādi √jñap, √yam, √cah, √cap, √rah and √bal —",
 "at 47 after slice 10d curated the jñapādi √jñap, √yam, √cah, √cap, √rah and √bal, at 139 after slice 10e curated ninety-two adanta roots —"),
("other forms — a second (624 cells), a third (177 cells), a fourth",
 "other forms — a second (790 cells), a third (343 cells), a fourth"),
("`ALTERNATES` (1166 rows in all, so 6696 + 1166 = 7862 forms total); √bhuj",
 "`ALTERNATES` (1664 rows in all, so 12996 + 1664 = 14660 forms total); √bhuj"),
("  (`tools/audit/README.md`'s 2026-10-02 10d entry, 6696 cells / 7862 forms /\n  150 roots).",
 "  (`tools/audit/README.md`'s 2026-10-02 10d entry, 6696 cells / 7862 forms /\n  150 roots), and that by curādi 10e's (`tools/audit/README.md`'s @DATE@ 10e\n  entry, 12996 cells / 14660 forms / 242 roots)."),
("only in the ordinary corpus-size sense, not wrong in kind: 6696 goldens",
 "only in the ordinary corpus-size sense, not wrong in kind: 12996 goldens"),
]

TOOLS_README = [
("**It asserts the corpus totals** (150 roots, 6696 cells, 7862 forms) rather than",
 "**It asserts the corpus totals** (242 roots, 12996 cells, 14660 forms) rather than"),
]

MAIN_RS = [
("/// differences, its `entry` negative control verified failing (36 √bhū\n/// cells). √tṛh joins none of the fork",
 "/// differences, its `entry` negative control verified failing (36 √bhū\n"
 "/// cells), and curādi 10e's re-ran it at the same commit over all 12996\n"
 "/// cells / 14660 forms / 242 roots with zero differences, its `entry`\n"
 "/// negative control verified failing (36 √bhū cells). √tṛh joins none of the fork"),
]

GUNA = [
("    // 150-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)",
 "    // 242-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)"),
]

DATA = [
("    /// The test covers the 150 roots curated here, not the dhātupāṭha's 2259.",
 "    /// The test covers the 242 roots curated here, not the dhātupāṭha's 2259."),
("    /// vendored upadeśa: 66 of the 150 curated roots carry a `\\` at all, and 45",
 "    /// vendored upadeśa: 66 of the 242 curated roots carry a `\\` at all, and 45"),
]

SPEC_10D = [
("- adanta roots, with the ā-garvīya list (10.0497);",
 "- adanta roots, with the ā-garvīya list (10.0497) (taken by slice 10e, see\n"
 "  `2026-10-02-curadi-gana-10e-design.md`, all but the four optional-ṇic\n"
 "  adanta rows);"),
]

FILES = {
    "docs/ARCHITECTURE.md": ARCH,
    "README.md": README,
    "AGENTS.md": AGENTS,
    "tools/audit/README.md": TOOLS_README,
    "crates/panini-prakriya/src/tinanta/guna.rs": GUNA,
    "crates/panini-data/src/lib.rs": DATA,
    "crates/panini/tests/paradigm/main.rs": MAIN_RS,
    "docs/superpowers/specs/2026-10-02-curadi-gana-10d-design.md": SPEC_10D,
}

def main():
    date = sys.argv[1]
    out, bad = {}, []
    for path, edits in FILES.items():
        s = open(path).read()
        for edit in edits:
            if edit[0] is SPAN:
                _, start, end, new = edit
                if s.count(start) != 1:
                    bad.append(f"{path}: {s.count(start)}× span start {start[:70]!r}")
                    continue
                i = s.index(start)
                j = s.find(end, i)
                if j < 0:
                    bad.append(f"{path}: span end {end[:70]!r} not after start")
                    continue
                s = s[:i] + new + s[j:]
                continue
            old, new = edit
            n = s.count(old)
            if n != 1:
                bad.append(f"{path}: {n}× {old[:70]!r}")
                continue
            s = s.replace(old, new.replace("@DATE@", date))
        out[path] = s
    if bad:
        sys.exit("not applied:\n  " + "\n  ".join(bad))
    for path, s in out.items():
        open(path, "w").write(s)
    print(f"applied {sum(len(e) for e in FILES.values())} edits to {len(FILES)} files")

main()
```

```bash
python3 /tmp/vidyut-full/slice10e/docsweep_10e.py "$(date -u +%F)"
```

Expected: `applied 33 edits to 8 files`. If it prints `not applied:`, nothing was written. Edit the paragraph named to the same facts, rather than skipping it, and re-run.

Notes on the numbers, re-derived from the census rather than assumed:
- 1180 = 12996 − 11816.
- 342 = 176 + 2 × 83.
- 348 = 325 + 22 + 1, which is `key_count("8.4.56")`.
- 171 + 71 = 242.
- 119 = 36 + 83.
- `grep -c "pada: PadaAssignment::Atmanepada,\|pada: PadaAssignment::Akusmiya,\|pada: PadaAssignment::AaGarviya," crates/panini-data/src/lib.rs` prints `71`.

- [ ] **Step 5: The AGENTS.md stale-comment ledger**

After the sentence ``Curādi 10d touched neither comment either; the corpus stands at 6696 cells as of 10d (`guna.rs:2565`'s claim anchored at `guna.rs:2565`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit).``, insert:

``Curādi 10e touched neither comment either; the corpus stands at 12996 cells as of 10e (`guna.rs:2565`'s claim anchored at `guna.rs:<G>`, `controller.rs:206`'s at `controller.rs:<C>`; both lines measured by grep at this commit).``

Measure `<G>` with `grep -n "1872 goldens move" crates/panini-prakriya/src/tinanta/guna.rs` and `<C>` with `grep -n "only 8 cells fire" crates/panini-prakriya/src/controller.rs`. Both were 2565 and 206 in the prototype. Measure; never compute.

AGENTS.md's floor paragraph (`measured at 6696 cells`) and the current mutation record belong to Task 6.

- [ ] **Step 6: Sweep for anything left stale**

```bash
grep -rn -i -E "\b6696\b|\b7862\b|\b744\b|\b1166\b|150 roots|150-root|of these 150|of the 150|150 curated|47 of|thirty-six roots|ten curādi roots|88 \+ 62|\b5848\b|139 total\)|ninety-six|forty-seven|only ids here|10\.0496, 10\.0493|the adanta slice|\b176\b|\b182\b|\b159\b" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.md --include=*.rs | grep -v "paradigm/data/"
```

Expected residue, all of it dated history that stays true:
- AGENTS.md:37's floor paragraph (`measured at 6696 cells`), which Task 6 rewrites;
- AGENTS.md's audit chain (the 10d entry);
- AGENTS.md's stale-comment ledger;
- `tools/audit/README.md`'s 10d entry and its `150 = 144 + 6` / `6696 = 6264 + 432` / `7862 = 7394 + 432` totals;
- `paradigm/main.rs`'s audit chain (the 10d clause), its 10d paragraph ("OPEN at 47"), and the pada-ambiguous comment's "seventy-two to ninety-six" and "ninety-six to 420".

Any other hit is a miss. Edit it.

- [ ] **Step 7: Run the full suite and commit**

Run: `mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"` (foreground, timeout 600000 ms). Expected: PASS at 12996 cells.

```bash
mise run fmt && mise run lint
git add -A
git commit -m "docs: 10e's counts, the audit record, and the sweep

12996 cells / 14660 forms / 242 roots across README, ARCHITECTURE, AGENTS,
paradigm/main.rs and tools/audit; curādi open at 139/509; 141 rules; 119
both-pada roots; 420 pada-ambiguous surfaces. Audit at zero divergence
against 8da2f90b; prior traces byte-identical to main but for √gandh's
8.3.24 → 8.4.58 pair."
```

---

## Task 6: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and the current-record paragraph); `mise.toml` if the cap moves

Follow AGENTS.md's cargo-mutants protocol. Hazards from this repo's record:
- **Measure, never scale.**
- **Every invocation rotates `mutants.out`**, so always pass `-o`.
- **The mise shim fails.** Use the real binary: `CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`.
- **`pgrep -f` matches its own shell.** Wait on `pgrep -x cargo-mutants`.
- **Background shells die at about 60 minutes.** Launch detached with `setsid nohup`, as below. This campaign may run longer than 10d's 27 minutes, because the suite is about twice as slow.

The prototype measured these, which are the values to expect:
- **The suite** took about 30 s on a quiet host (10d's floor was 16 s), so the cap of **260** will probably move.
- **The mutant list** goes 809 → **820** in panini-prakriya, while panini-analyze stays at 12. The 11 new are:
  - `samjna.rs:211:25` (`||`→`&&`, 1.3.78's `AaGarviya` arm);
  - `sanadi.rs:78:16` (`delete !`, 10.0497);
  - `sanadi.rs:211:16` (`delete !`, 6.4.48);
  - `sanadi.rs:236:67` and `sanadi.rs:303:17` (`||`→`&&`, the two `AtLopa` guards);
  - six in 8.3.24's widened guard: `tripadi.rs:879:69` (`&&`→`||`), `879:72` (`delete !`), `885:21` (`&&`→`||`), `885:25` (`delete !`), `885:36` (`||`→`&&`) and `885:45` (`==`→`!=`).

  All were caught in the prototype's `--in-diff` run.
- **The three documented non-caught entries:**
  - `adesha.rs:589:30` is unchanged;
  - the two `tripadi.rs` entries move down by the lines Task 3 added, from `1289:38` to **`1302:38`** and from `1602:23` to **`1615:23`**.

  Confirm all positions by `--list`. Never compute them.

- [ ] **Step 1: Measure the floor**

With nothing else running, run this twice: `time mise run test 2>&1 | tail -3` (foreground). Record both wall clocks and `cat /proc/loadavg`. The 6696-cell floor was 16.860s / 15.610s.

- [ ] **Step 2: Locate and probe the two uncaught equivalents at `-j 4`**

```bash
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /="
```

Expect `589` (adesha), `1302` (tripadi, inside 8.3.13's `apply`) and `1615` (the ṇatva hang). Write them as `<A>`, `<T1>` and `<T2>`. Then run in the foreground with timeout 600000 ms:

```bash
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 600 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:<A>:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" 2>&1 | tail -10
```

Both must be MISSED, not TIMEOUT. Read each test-phase duration from `$SCRATCH/mutants.out/outcomes.json`. Set a provisional cap: max(260, 6 × the longer of the two, rounded up to the next 10 s).

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/curadi-10e"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout <CAP> -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"
```

`<CAP>` is Step 2's provisional cap. Run nothing CPU-heavy meanwhile. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

If the process dies before finishing, resume with the same command plus `--iterate`. That re-tests only what has no outcome yet.

- [ ] **Step 4: Read the outcomes**

When `pgrep -x cargo-mutants` returns nothing:

```bash
date -u +"%F %T UTC" > "$OUT/finished"
cp "$OUT/mutants.out/outcomes.json" "$OUT/outcomes.durable.json"
tail -5 "$OUT/campaign.log"
cat "$OUT/mutants.out/missed.txt" "$OUT/mutants.out/timeout.txt"
```

Expected (exit code 3 is normal when a timeout is present):
- **832 mutants: 781 caught, 48 unviable, 2 missed, 1 timeout.**
  - panini-prakriya: 820 / 773 / 44 / 2 / 1.
  - panini-analyze: 12 / 8 / 4 / 0 / 0.
- `missed.txt` holds exactly `adesha.rs:<A>:30: replace + with *` and `tripadi.rs:<T1>:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:<T2>:23: replace -= with /=`.

If not:
- Any **other timeout** is a suspect survivor that the larger suite pushed past the cap. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant in `sanadi.rs`, `samjna.rs`, `mod.rs` or 8.3.24 is a gap in Task 2's or 3's tests; add the test that kills it. Any other missed mutant means a test that caught it at 6696 cells no longer does; stop and report.

- [ ] **Step 5: Margins**

```bash
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["outcomes"][1])' "$OUT/mutants.out/outcomes.json"
```

Compute two things:
- the two equivalents' test phases under campaign load;
- the caught phases' min / median / p90 / max.

The cap is max(260, 6 × the longest campaign-load equivalent phase, rounded up to the next 10 s).
- If that is 260, the cap stays.
- Otherwise change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together.

- [ ] **Step 6: Record it in AGENTS.md**

- Rewrite the paragraph that opens `**The floor behind the 260s cap, measured at 6696 cells on Rust 1.99.0,`. Use Step 1's and Step 2's numbers at 12996 cells, the load averages, and the cap Step 5 chose. Keep the comparison chain to earlier floors, with 10d's 16.860s / 15.610s at 6696 cells joining it.
- Replace the `**Current record (curādi 10d, 2026-10-02).**` paragraph with `**Current record (curādi 10e, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**;
  - the non-caught set diffed against 10d's on the full record (package, span, replacement, function, genre, outcome). The clean result is that the 51 entries are identical except the two `tripadi.rs` spans, which moved down by exactly the line count Task 3 added;
  - the eleven new-code mutants, each named with its outcome (all caught);
  - the campaign-load phases and margins;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`.

  End it with a pointer to the record it replaces. Run `git rev-parse --short HEAD` before committing, and write ``The curādi 10d record it replaces: `git show <that hash>:AGENTS.md`.``

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md mise.toml
git commit -m "chore: 10e mutation gate — floor and uncaught run re-measured at 12996 cells

All eleven new-code mutants (10.0497, 6.4.48, the AtLopa guards, 1.3.78's
AaGarviya arm, 8.3.24's widened guard) caught; missed.txt holds only the two
documented equivalents and timeout.txt only the permanent ṇatva-scan entry."
```

---

## Task 7: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -20
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # back at /workspace/crates
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin curadi-10e
gh pr create --title "curādi 10e — the adanta roots and the ā-garvīya" --body "$(cat <<'BODY'
Slice 10e curates 92 of curādi's 96 adanta roots (`10.0108 mArga` and
`10.0389 kaTa` … `10.0492 Deka`): 83 ubhayapadī by 1.3.74 and the nine
ā-garvīya, ātmanepadī by the gaṇasūtra **10.0497** *ā garvād ātmanepadinaḥ*.

- **6.4.48 *ato lopaḥ ārdhadhātuke*** deletes the final `a` before ṇic
  (`kaTa` → `kaT`), where vidyut runs it. It sets `Tag::AtLopa`, on which
  7.2.116 and the sanādi 7.3.86 decline: 1.1.57's sthānivadbhāva, so
  *kathayati*, not *kāthayati*, and *kuhayate*, not *kohayate*.
- **8.3.24** now reaches a curādi root's own `n` (*saṅketayati*), searching
  only `ANGA` so the 7.1.3 `n` of *corayanti* stays out. 10c's √gandh gains
  the 8.3.24 → 8.4.58 pair vidyut also credits; its forms are unchanged.

**Deferred:** `pata`, `mUtra`, `katra` and `garva` go to the optional-ṇic slice.

The golden suite goes from 6696 to 12996 cells; curādi is open at 139 of 509.
The audit shows zero divergence against `8da2f90b`. A main-vs-branch dump of
every prior cell's traces is byte-identical but for √gandh's pair, and the
mutation gate catches every new mutant.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`:
   - run `git worktree remove .worktrees/curadi-10e`;
   - run `git worktree remove --force .worktrees/curadi-10e-proto` (the throwaway);
   - delete the local and remote `curadi-10e` branch;
   - run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| `AA_GARVIYA` incl. `garva`, pinned to upstream; `PadaAssignment::AaGarviya` | 2 |
| `Tag::AaGarviya`, `Tag::AtLopa`; `derive` arm | 2 |
| 10.0497 (after 10.0496, blocks parasmaipada); 1.3.78 declines on it | 2 |
| 6.4.48 (after 3.4.114); 7.2.116 / 7.3.86 decline on `AtLopa`; 1.1.57 not credited | 2 |
| Order pin and its doc; unit tests | 2 |
| 8.3.24 widened to a curādi root's own `n` (amendment); √gandh's pair | 3 |
| 92 rows (83 `Nic`, 9 `AaGarviya`), homograph comments | 4 |
| 700 goldens + 498 alternates, engine = vidyut per cell; totals 242 / 12996 / 14660; buckets 11816 / 790 / 343; ALTERNATES 1664 and key counts | 4 |
| Pada-ambiguous 96 → 420 | 4 |
| 1.3.74 half reads the `pada` column (93); 6.4.48 6798 credits; 10.0497 324 credits and blocked parasmaipada; √kath and √kuha traces | 4 |
| juhotyādi 8.3.24 (582) and 8.4.40 (57) pins (amendment) | 3, 4 |
| `check()`: class witnesses, four homograph pairs, prevented shapes Invalid; 10d's test adjusted | 4 |
| Prior traces byte-identical but for √gandh | 5 Step 2 |
| Audit with repoint and negative control; record | 5 |
| README / ARCHITECTURE (141 rules, 342 / 348 / 325, 171 + 71, 119) / AGENTS; 10d spec pointer | 5 |
| Floor, uncaught probe, campaign, verbatim non-caught record, new-code mutants named | 6 |

**Type consistency.**
- `AA_GARVIYA: RangeInclusive<&str>` is read three ways:
  - as `AA_GARVIYA.contains(&d.dhatupatha)` (`&&str`) in the agreement test and the trace test;
  - as `AA_GARVIYA.contains(n)` with `n: &&str` in the upstream pin;
  - as `AA_GARVIYA.contains(number)` with `number: &&'static str` in the credit loop.

  All three compiled in the prototype.
- `credited` returns `Vec<(&'static str, Gana)>`. `CURADI.contains(number)` takes `number: &&str`.
- The golden tuple shapes match `ParadigmRow` and `AlternateRow`.

**Known soft spots.**
- **Doc strings in Task 5** were read at the prototype's state. If the script reports an `old` not found exactly once, edit that paragraph to the same facts rather than skip it.
- **The intermediate states** after Tasks 2 and 3 were not run in the prototype. Their test steps are the check.
- **The floor and cap in Task 6** depend on host load. Record the load beside every timing.
