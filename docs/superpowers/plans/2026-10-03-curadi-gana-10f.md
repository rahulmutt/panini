# Curādi gaṇa slice 10f Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Give curādi an optional ṇic and curate the ten rows earlier slices set aside for it, across laṭ / laṅ / loṭ / vidhiliṅ:
- six ākusmīya rows, `10.0193 daSi~`, `10.0194 dasi~`, `10.0198 tatri~`, `10.0199 matri~` (Kaumudī 2564, idit) and `10.0227 vancu~`, `10.0230 divu~` (2570, udit);
- four adanta rows: `10.0400 pata` (2573.1, with 2573.2's *pātayati*), and `10.0449 garva`, `10.0451 mUtra`, `10.0456 katra` (2573.3).

Each forks into a ṇic branch (index 0, as before) and a ṇic-less one, parasmaipada by 1.3.78. The golden suite goes from 12996 to 13716 cells, and curādi from 139 to 149 of its 509 rows.

**Architecture:** Six tasks:
- **Task 1** checks the worktree and the baseline.
- **Task 2** is the engine and the harness adaptations. It lands green on its own, because no curated row is in `OPTIONAL_NIC` yet:
  - `OPTIONAL_NIC`, `optional_nic()` and `Dhatu::padas()` in `panini-data`;
  - five Kaumudī vikalpa rules at the head of `SANADI` (2564, 2570, 2573.1, 2573.3 through the shared `skip_nic`, and 2573.2);
  - a second 6.1.97 entry and a second 6.1.101 entry at the head of `ADESHA`, for an aṅga-final `a` meeting śap;
  - every pada enumerator switched to `Dhatu::padas()`; `derivation_set_is_exactly_pinned` pinning the first live branch; `roundtrip` admitting blocked branches on optional-ṇic rows only;
  - the three prakriya pins, and unit tests.
- **Task 3** lands the ten rows, 80 golden rows and 264 alternates, and every count, list, trace and `check()` assertion they move. The assertions go in first and fail; the rows and goldens make them pass.
- **Tasks 4–6** are the audit with the prior-trace diff and the doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.99.0, pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, checked out at `/tmp/vidyut-full`.

**Spec:** `docs/superpowers/specs/2026-10-02-curadi-gana-10f-design.md`. Read its "(amendment)" paragraphs: the rule names, the aṅga–śap junction, the first-live-branch pin, the fork census, and the Prototype bullet under Evidence. All postdate the first draft.

**Workspace:** the branch `curadi-10f` is checked out at `/workspace/.worktrees/curadi-10f` and holds the spec and this plan. Every path below is relative to that directory unless it starts with `/`.

**Provenance.** This slice was built end to end on a throwaway worktree, `/workspace/.worktrees/curadi-10f-proto` (detached, throwaway commits `aad70dc`…`dc931c3`), which Task 6 deletes. Every code block and script below is that prototype's code, and was checked two ways:
- the prototype's final state passed the full suite, clippy `-D warnings` and `fmt-check`; its generator found all 720 new cells equal to vidyut's (720 cells, 984 forms, 0 differences); the audit at 252 roots / 13716 cells / 15644 forms showed zero differences, with the `entry` control failing on 36 cells; a main-vs-prototype dump of all 14660 prior live branches was byte-identical; `cargo mutants --in-diff` caught every new mutant (18 mutants across the three crates, 16 caught, 2 unviable, 0 missed);
- the scripts below, run in this plan's order on a fresh worktree at the spec commit (`/tmp/vidyut-full/slice10f/replay_10f.sh`, a verification aid this plan does not need), reproduced the prototype byte for byte in every tracked file but the spec. Task 2 alone, replayed, was green: `panini-prakriya` 418, `panini-data` 25, `trace` 203, `paradigm` 24.

**Throwaway scripts.** Everything under `/tmp/vidyut-full/slice10f/` and the two vidyut examples never ship. Each is reproduced in full in this plan with its sha256, so it can be recreated if `/tmp` was cleaned. Recreate a file only if it is missing, and check its hash either way.

## Global Constraints

- **Five new rule ids, no others:** `2564` (name `"iditkaraRaM RicaH pAkzikatve liNgam"`), `2570` (`"YitkaraRasAmarTyAdasya RijvikalpaH"`), `2573.1` (`"vA RijantaH"`), `2573.3` (`"adantatvasAmarTyARRijvikalpaH"`) and `2573.2` (`"vA'danta ityeke"`), all `RuleKind::Vidhi`, `vikalpa: true`, in `tinanta/sanadi.rs`. The two new `adesha.rs` entries reuse the ids `6.1.97` and `6.1.101`. The names are the Siddhānta-Kaumudī's text (spec, Decisions); do not paraphrase them.
- **Not modelled:** 7.1.58 (stays the stored-`code` simplification) and 7.3.63 (spec, Decisions).
- **Out of scope:** every other optional-ṇic row and trigger (10.0498, 10.0499, 2565, 2571, 2572, the other idit/udit rows). `OPTIONAL_NIC` lists exactly the ten.
- **Pre-existing cells must stay byte-identical, traces included,** with no exception. Regenerate no prior golden.
- **Goldens come from the generator, which asserts engine = vidyut cell by cell, and their sha256 must match this plan's.** **Do not edit a golden to match the engine.** If the generator reports a difference, or a hash differs, stop and report.
- **The aṅga–śap entries keep separate bodies.** `adesha.rs`'s module doc explains why this stage's arms repeat their lookups rather than share a helper. Do not factor the two new entries into one.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit.
- `mise run test` takes about 30 s on a quiet host. Run it in the **foreground** with a timeout of 600000 ms. Never background it and end a turn.
- `mise run test -- -p X` does not scope. Scope with `mise exec -- cargo test -p <crate> <filter>`. To see every failing binary at once, use `mise exec -- cargo test --workspace --no-fail-fast`.
- The `cargo-mutants` mise shim fails here. Use the real binary, `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`, under `mise exec --`.
- `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes its `panini` and `panini-data` dev-deps. Repoint them at this worktree before generating or auditing, and back at `/workspace/crates` after.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **An ākusmīya or ā-garvīya root asked for parasmaipada.** The ṇic branch must block with nothing recorded (not even the Kaumudī id: it is the declined branch), the ṇic-less branch must be live and credit 1.3.78, and the analyzer must never surface the blocked branch's partial text (`danS`). → Task 2 `roundtrip`'s blocked-branch rule; Task 3 `no_nic_pada_rule_reaches_a_nicless_branch`, `daMSati_trace_has_no_nic_and_no_a_kusmad`, and `check("daMSayati")` Invalid.
2. **The aṅga–śap entries reaching a root they should not.** Any aṅga ending in a short `a` before a thematic śap would fire them. → Task 2's guard tests (`ato_gune_merges_an_a_final_anga_into_shaps_a`, `savarna_dirgha_merges_an_a_final_anga_into_a_lengthened_shap`); Task 4's prior-trace diff, which is the corpus-wide check (`credited()` cannot tell an id's two entries apart).
3. **`OPTIONAL_NIC` drifting from the upadeśa,** or a curated idit / udit / ñit curādi row missing from it. → Task 3 `optional_nic_matches_upadesha_markers`, which also pins `Cidra` (a conjunct before its `a`, but not on the Kaumudī's list) out of the table.
4. **No ṇic in ātmanepada.** `mUtra`, `katra` and `pata` must derive only ṇic forms there; *mūtrate*, *patate*, *garvate*, *daṃśate* are Invalid. → Task 3 `the_optional_nic_ids_are_credited_only_on_their_rows` (every ṇic-less ātmanepada branch blocked) and the `check()` Invalid list.
5. **2573.2 leaking** onto the ṇic-less branch or onto another adanta row (*pātati*, *kāthayati*). → Task 2 `va_adanta_deletes_patas_a_before_nic_only_on_pata` and `exactly_the_pinned_bars`; `derivation_set_is_exactly_pinned` rejects any extra form.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-data/src/lib.rs` | Task 2: `OPTIONAL_NIC`, `optional_nic()`, `Dhatu::padas()`. Task 3: ten rows, row-list test, count, marker test, `padas` test, pada-agreement ṇic-less arm, `PadaAssignment` / `AA_GARVIYA` / `Dhatu::pada` docs. Task 4: one count |
| `crates/panini-prakriya/src/tinanta/sanadi.rs` | Task 2: `skip_nic`, the five Kaumudī rules, four tests. Task 4: module doc, 10.0497's comment |
| `crates/panini-prakriya/src/tinanta/adesha.rs` | Task 2: the two aṅga–śap entries, `junction()` for the existing tests, three tests |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Task 2: order, vikalpa and bars pins, the order pin's doc |
| `crates/panini-analyze/src/lib.rs`, `crates/panini/tests/trace/helpers.rs` | Task 2: `d.padas()` |
| `crates/panini/tests/roundtrip.rs` | Task 2: blocked branches on optional-ṇic rows only |
| `crates/panini/tests/paradigm/main.rs` | Task 2: `d.padas()`, first-live-branch pin. Task 3: `VIKALPA_RULES`, census, keys, pada-ambiguous set, `check()` witnesses. Task 4: audit-chain prose |
| `crates/panini/tests/paradigm/data/curadi.rs` | Task 3: 80 golden rows, 264 alternates |
| `crates/panini/tests/trace/curadi.rs`, `crates/panini/tests/trace/juhotyadi.rs` | Task 2: `d.padas()` in juhotyādi. Task 3: five moved assertions, five new tests |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `guna.rs` (one comment), the 10b, 10c and 10e specs | Task 4 |
| `AGENTS.md`, maybe `mise.toml` | Task 5 |

---

## Task 1: The worktree and baseline

**Files:** none. **Interfaces:** none.

- [ ] **Step 1: Confirm the worktree**

```bash
cd /workspace/.worktrees/curadi-10f
git status --short          # empty
git log --oneline -5        # the plan commit, 07c17dd (spec: separate bodies), 4a1c671 (spec amendments), 20f362b (spec), 6864f4f
```

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: everything passes at 12996 cells. `panini-prakriya` reports 411 tests, the `trace` binary 203, `paradigm` 24 and `panini-data` 25.

---

## Task 2: The engine — `OPTIONAL_NIC`, the five Kaumudī vikalpas, the aṅga–śap entries, and the harness

No curated row is in `OPTIONAL_NIC` yet, so no rule added here fires anywhere in the corpus, every golden is unchanged, and the suite is green at the end of the task.

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (after `pub const AA_GARVIYA`; after `struct Dhatu`)
- Modify: `crates/panini-prakriya/src/tinanta/sanadi.rs` (imports; the head of `SANADI`; the end of `mod tests`)
- Modify: `crates/panini-prakriya/src/tinanta/adesha.rs` (the head of `ADESHA`; `mod tests`)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (three pins, one doc)
- Modify: `crates/panini-analyze/src/lib.rs`, `crates/panini/tests/paradigm/main.rs`, `crates/panini/tests/trace/helpers.rs`, `crates/panini/tests/trace/juhotyadi.rs`, `crates/panini/tests/roundtrip.rs`

**Interfaces:**
- Produces:
  - `panini_data::OPTIONAL_NIC: &[(&str, &str)]` (number, Kaumudī id), ten entries;
  - `panini_data::optional_nic(dhatupatha: &str) -> Option<&'static str>`;
  - `Dhatu::padas(&self) -> &'static [Pada]`: the `PadaAssignment`'s padas, plus parasmaipada first when the row is in `OPTIONAL_NIC` and its assignment is ātmanepada-only;
  - rule ids `"2564"`, `"2570"`, `"2573.1"`, `"2573.3"`, `"2573.2"` (first in `SANADI`, in that order), and second `"6.1.97"` / `"6.1.101"` entries (first in `ADESHA`).
- Consumes: the sanādi tests' `pada_dhatu(root: &str, tags: &[Tag], pada: Pada) -> Prakriya`, `with_nic(root: &str, nic: Option<&[Tag]>) -> Prakriya` and `rule(id: &str) -> &'static Rule`.

Every edit in this task is one of the blocks below, in TDD order. The same edits, applied all at once, are what the Appendix's `engine_10f.py` does from the same blocks saved under `/tmp/vidyut-full/slice10f/seg/`; use it only to check your result (`git diff` against a scratch copy), not to skip Step 2's failing build.

- [ ] **Step 1: Write the failing tests**

**(a)** In `crates/panini-prakriya/src/tinanta/sanadi.rs`, at the end of `mod tests` (after `fn a_garvad_declines_without_the_a_garviya_licence() { … }`, before the module's closing `}`), add a blank line and then this block (`seg/sanadi_tests.rs`, sha256 `6faa0229c1be36b44a825dd0d87755fa4621e114473fe253bacac65a688e523a`):

```rust
    /// A curādi root at row `number`, with its ṇic-branch pada `tag`, as the
    /// sanādi stage first sees it.
    fn optional_nic_dhatu(number: &'static str, root: &str, tag: Tag) -> Prakriya {
        let mut p = pada_dhatu(root, &[Tag::Curadi, tag], Pada::Parasmaipada);
        p.ctx.dhatupatha = number;
        p
    }

    #[test]
    fn each_optional_nic_rule_takes_the_nicless_branch_on_its_own_rows() {
        // (rule, a row it owns, that row's root and ṇic-branch pada tag)
        for (id, number, root, tag) in [
            ("2564", "10.0193", "danS", Tag::Akusmiya),
            ("2570", "10.0230", "div", Tag::Akusmiya),
            ("2573.1", "10.0400", "pata", Tag::Nic),
            ("2573.3", "10.0449", "garva", Tag::AaGarviya),
            ("2573.3", "10.0451", "mUtra", Tag::Nic),
        ] {
            let mut p = optional_nic_dhatu(number, root, tag);
            assert!((rule(id).apply)(&mut p), "{id} {number}");
            // The pada tag that hangs on ṇic is gone; nothing else changes.
            for t in [Tag::Nic, Tag::Akusmiya, Tag::AaGarviya] {
                assert!(!p.terms[ANGA].has(t), "{id} {number} kept {t:?}");
            }
            assert!(p.terms[ANGA].has(Tag::Curadi), "{id} {number}");
            assert_eq!(p.terms[ANGA].text, root, "a choice, not an operation");
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, [id]);
            assert!(!p.blocked);
        }
    }

    #[test]
    fn an_optional_nic_rule_declines_off_its_rows() {
        // A sibling id's row, a curādi row outside the table (√cur, the
        // ākusmīya √cit, the adanta √kath), and a row with no number at all.
        for id in ["2564", "2570", "2573.1", "2573.3"] {
            for (number, root, tag) in [
                ("10.0193", "danS", Tag::Akusmiya),
                ("10.0230", "div", Tag::Akusmiya),
                ("10.0400", "pata", Tag::Nic),
                ("10.0451", "mUtra", Tag::Nic),
                ("10.0001", "cur", Tag::Nic),
                ("10.0192", "cit", Tag::Akusmiya),
                ("10.0389", "kaTa", Tag::Nic),
                ("", "cur", Tag::Nic),
            ] {
                if optional_nic(number) == Some(id) {
                    continue;
                }
                let mut p = optional_nic_dhatu(number, root, tag);
                assert!(!(rule(id).apply)(&mut p), "{id} on {number}");
                assert!(p.terms[ANGA].has(tag), "{id} on {number}");
                assert!(p.log.is_empty(), "{id} on {number}");
            }
        }
    }

    #[test]
    fn the_optional_nic_rules_are_vikalpas_that_bar_nic() {
        for id in ["2564", "2570", "2573.1", "2573.3"] {
            let r = rule(id);
            assert!(r.vikalpa, "{id}");
            assert_eq!(r.bars, ["3.1.25", "2573.2"], "{id}");
        }
        let r = rule("2573.2");
        assert!(r.vikalpa);
        assert!(r.bars.is_empty());
    }

    #[test]
    fn va_adanta_deletes_patas_a_before_nic_only_on_pata() {
        // `pata`, on the ṇic branch (nothing removed its `Nic` tag): the final
        // `a` goes, and no `Tag::AtLopa` is set, so 7.2.116 will lengthen.
        let mut p = optional_nic_dhatu("10.0400", "pata", Tag::Nic);
        assert!((rule("2573.2").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "pat");
        assert!(!p.terms[ANGA].has(Tag::AtLopa));
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["2573.2"]);
        // Then ṇic: 6.4.48 finds no `a`, and 7.2.116 lengthens.
        let mut q = with_nic("pat", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!(!(rule("6.4.48").apply)(&mut q));
        assert!((rule("7.2.116").apply)(&mut q));
        assert_eq!(q.terms[ANGA].text, "pAt");
        // Other adanta rows, optional-ṇic or not, keep their `a`.
        for (number, root) in [
            ("10.0451", "mUtra"),
            ("10.0449", "garva"),
            ("10.0389", "kaTa"),
        ] {
            let mut p = optional_nic_dhatu(number, root, Tag::Nic);
            assert!(!(rule("2573.2").apply)(&mut p), "{number}");
            assert_eq!(p.terms[ANGA].text, root);
            assert!(p.log.is_empty(), "{number}");
        }
    }
```

**(b)** In `crates/panini-prakriya/src/tinanta/adesha.rs`'s `mod tests`, immediately after the line `    use panini_data::{Pada, Purusha, Vacana, dhatus};` and its blank line, add this helper and a blank line (`seg/adesha_junction.rs`, sha256 `2317b425530a58697483349d247a8466cbd87d340fa48df42b2d0eef8ee548fc`):

```rust
    /// The stage's LAST entry under `id`. 6.1.97 and 6.1.101 each have two
    /// since slice 10f: an aṅga–śap entry at the head of the stage and the
    /// vikaraṇa–ending entry these tests were written for, which is the
    /// last. For every other id the stage has one entry, so this is it.
    fn junction(id: &str) -> &'static Rule {
        ADESHA.iter().rev().find(|r| r.id == id).unwrap()
    }
```

Then, in the same module, replace all **eight** occurrences of `rules().find(|r| r.id == "6.1.101").unwrap()` with `junction("6.1.101")`, and in `atas_ca_then_vrddhir_eci_is_the_dade_path`'s loop replace

```rust
        for (id, fires) in [("6.1.101", false), ("6.1.90", true), ("6.1.88", true)] {
            let r = rules().find(|r| r.id == id).unwrap();
```

with

```rust
        for (id, fires) in [("6.1.101", false), ("6.1.90", true), ("6.1.88", true)] {
            let r = junction(id);
```

These tests were written for the four-armed 6.1.101; after Step 5 a `rules()` lookup would find the new head entry first.

At the end of `mod tests` (before its closing `}`), add a blank line and this block (`seg/adesha_tests.rs`, sha256 `6f9987b4486e04b7cfc51a3d7da445c2a8fc1afa8f127444511ed40184bf8c0d`):

```rust
    // --- slice 10f: the aṅga–śap entries of 6.1.97 and 6.1.101 -------------

    /// The stage's FIRST entry under `id`: for 6.1.97 and 6.1.101, the
    /// aṅga–śap entry at the head of the stage.
    fn anga_shap(id: &str) -> &'static Rule {
        ADESHA.iter().find(|r| r.id == id).unwrap()
    }

    /// `anga` + a śap spelled `shap` (thematic unless `thematic` is false)
    /// + `ending`.
    fn anga_shap_prakriya(anga: &str, shap: &str, thematic: bool, ending: &str) -> Prakriya {
        let mut s = Term::new(shap);
        if thematic {
            s.add(Tag::Thematic);
        }
        Prakriya {
            terms: with_slots(vec![Term::new(anga), s, Term::new(ending)]),
            ..Default::default()
        }
    }

    #[test]
    fn the_anga_shap_entries_head_the_stage() {
        let ids: Vec<&str> = ADESHA.iter().take(2).map(|r| r.id).collect();
        assert_eq!(ids, ["6.1.97", "6.1.101"]);
        assert_eq!(ADESHA.iter().filter(|r| r.id == "6.1.97").count(), 2);
        assert_eq!(ADESHA.iter().filter(|r| r.id == "6.1.101").count(), 2);
    }

    #[test]
    fn ato_gune_merges_an_a_final_anga_into_shaps_a() {
        // katra + a + ti → katr + a + ti: the aṅga loses its `a`, the śap
        // spells the para-rūpa vowel, the ending is untouched.
        let mut p = anga_shap_prakriya("katra", "a", true, "ti");
        assert!((anga_shap("6.1.97").apply)(&mut p));
        assert_eq!(p.text(), "katrati");
        assert_eq!(p.terms[ANGA].text, "katr");
        assert_eq!(p.terms[SHAP].text, "a");
        assert_eq!(p.terms[ENDING].text, "ti");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["6.1.97"]);
        // Declines: a consonant-final aṅga (Bav), a long `A`-final one (pA),
        // a lengthened śap (`A`, 6.1.101's), and a non-thematic vikaraṇa.
        for (anga, shap, thematic) in [
            ("Bav", "a", true),
            ("pA", "a", true),
            ("katra", "A", true),
            ("katra", "a", false),
        ] {
            let mut p = anga_shap_prakriya(anga, shap, thematic, "ti");
            assert!(
                !(anga_shap("6.1.97").apply)(&mut p),
                "{anga} {shap} {thematic}"
            );
            assert_eq!(p.terms[ANGA].text, anga);
            assert!(p.log.is_empty());
        }
    }

    #[test]
    fn savarna_dirgha_merges_an_a_final_anga_into_a_lengthened_shap() {
        // pata + A + mi (7.3.101) → pat + A + mi.
        let mut p = anga_shap_prakriya("pata", "A", true, "mi");
        assert!((anga_shap("6.1.101").apply)(&mut p));
        assert_eq!(p.text(), "patAmi");
        assert_eq!(p.terms[ANGA].text, "pat");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["6.1.101"]);
        // Declines on an unlengthened śap (6.1.97's), a consonant-final
        // aṅga, and a non-thematic `A` vikaraṇa (kryādi's nA ends in `A` but
        // is not śap).
        for (anga, shap, thematic) in [
            ("pata", "a", true),
            ("Bav", "A", true),
            ("pata", "A", false),
        ] {
            let mut p = anga_shap_prakriya(anga, shap, thematic, "mi");
            assert!(
                !(anga_shap("6.1.101").apply)(&mut p),
                "{anga} {shap} {thematic}"
            );
            assert_eq!(p.terms[ANGA].text, anga);
            assert!(p.log.is_empty());
        }
    }
```

**(c)** In `crates/panini-prakriya/src/tinanta/derivation_tests.rs`:

- The order pin's doc: replace

```rust
/// `Tag::AtLopa` (1.1.57). 10.0496, 10.0497 and 10.0493 are the only ids
/// here that are not Aṣṭādhyāyī sūtras.
```

  with

```rust
/// `Tag::AtLopa` (1.1.57).
///
/// Slice 10f opens the list with five Kaumudī vikalpa rules, ahead of
/// 10.0496, because vidyut-prakriya decides ṇic before the pada gaṇasūtras:
/// 2564, 2570, 2573.1 and 2573.3 fork a root whose ṇic is optional into its
/// ṇic and ṇic-less branches, and 2573.2 then forks `pata`'s ṇic branch on
/// its final `a`. It also adds a second 6.1.97 and a second 6.1.101 at the
/// head of the adesha stage, for an aṅga-final `a` meeting śap, which only
/// the ṇic-less adanta branch reaches; see their comments in
/// `tinanta/adesha.rs`. 10.0496, 10.0497, 10.0493 and the five Kaumudī ids
/// are the only ids here that are not Aṣṭādhyāyī sūtras.
```

- In `tinanta_rule_order_is_pinned`, replace the opening of `expected`

```rust
        "10.0496", "10.0497", "10.0493", "3.1.25",
```

  with

```rust
        "2564", "2570", "2573.1", "2573.3", "2573.2", "10.0496", "10.0497", "10.0493", "3.1.25",
```

- and, further down the same array,

```rust
"6.4.42", "6.4.43", "6.1.101",
        "6.1.96",
```

  with

```rust
"6.4.42", "6.4.43", "6.1.97",
        "6.1.101", "6.1.101", "6.1.96",
```

- In `exactly_the_pinned_vikalpa_rules_are_optional`, replace

```rust
    let expected = [
        "7.1.35", "3.4.111", "7.3.86",
```

  with

```rust
    let expected = [
        "2564", "2570", "2573.1", "2573.3", "2573.2", "7.1.35", "3.4.111", "7.3.86",
```

- In `exactly_the_pinned_bars`, replace

```rust
    let expected: Vec<(&str, &[&str])> = vec![
        ("7.3.87",
```

  with

```rust
    let expected: Vec<(&str, &[&str])> = vec![
        ("2564", &["3.1.25", "2573.2"][..]),
        ("2570", &["3.1.25", "2573.2"][..]),
        ("2573.1", &["3.1.25", "2573.2"][..]),
        ("2573.3", &["3.1.25", "2573.2"][..]),
        ("7.3.87",
```


`mise run fmt` rewraps the order pin's array; the result is the prototype's.

- [ ] **Step 2: Run them to see them fail**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | grep -E "^error" | sort | uniq -c | head`
Expected: compile errors only — `cannot find value OPTIONAL_NIC`/`optional_nic`, no rule `2573.2` … (nothing exists yet). After Steps 3–5 the same command compiles.

- [ ] **Step 3: The data layer**

In `crates/panini-data/src/lib.rs`, replace the line

```rust
pub const AA_GARVIYA: RangeInclusive<&str> = "10.0440"..="10.0449";
```

with that line, a blank line, and this block (`seg/lib_table.rs`, sha256 `ce6a8980d36688b5b43d84dacadc2f1b496c3fe93eeab6545ad6f2e620a6a40f`):

```rust
/// The curādi rows whose ṇic is optional, each with the Kaumudī id that
/// makes it so: 2564 for an idit root, 2570 for a ñit or udit root, 2573.1
/// for `pata`, and 2573.3 for the roots that rule names (`mUtra`, `katra`,
/// `garva`). Keyed by dhātupāṭha number, as `JNAPADI` is: the engine never
/// sees an upadeśa's markers, so `danS` and `div` cannot say they are idit
/// or udit. The engine's sanādi stage forks each listed row into a ṇic
/// branch and a ṇic-less one (`Dhatu::padas`).
///
/// Lists curated rows only. `optional_nic_matches_upadesha_markers`
/// re-derives every entry from the vendored upadeśa and holds the table to
/// exactly the curated rows those markers select.
pub const OPTIONAL_NIC: &[(&str, &str)] = &[
    ("10.0193", "2564"),
    ("10.0194", "2564"),
    ("10.0198", "2564"),
    ("10.0199", "2564"),
    ("10.0227", "2570"),
    ("10.0230", "2570"),
    ("10.0400", "2573.1"),
    ("10.0449", "2573.3"),
    ("10.0451", "2573.3"),
    ("10.0456", "2573.3"),
];

/// The Kaumudī id that makes row `dhatupatha`'s ṇic optional, if any
/// (`OPTIONAL_NIC`).
pub fn optional_nic(dhatupatha: &str) -> Option<&'static str> {
    OPTIONAL_NIC
        .iter()
        .find(|(n, _)| *n == dhatupatha)
        .map(|(_, id)| *id)
}
```

Immediately after the closing `}` of `pub struct Dhatu { … }` (whose last field is `pub artha: &'static str,`), add a blank line and this block (`seg/lib_impl.rs`, sha256 `639bbb421f6323d11635d858ab1dd7b2e3404d4f64ffc1d5ac557b7da3566c4d`):

```rust
impl Dhatu {
    /// The padas this root derives. `pada` is the ṇic branch's verdict; a
    /// root whose ṇic is optional (`OPTIONAL_NIC`) also derives its ṇic-less
    /// branch, parasmaipada by 1.3.78 for every row listed there. So an
    /// ākusmīya or ā-garvīya row in the table admits both padas,
    /// parasmaipada first, as `PadaAssignment::padas` orders every
    /// two-pada assignment.
    pub fn padas(&self) -> &'static [Pada] {
        match (optional_nic(self.dhatupatha), self.pada.padas()) {
            (Some(_), [Pada::Atmanepada]) => &[Pada::Parasmaipada, Pada::Atmanepada],
            (_, padas) => padas,
        }
    }
}
```

- [ ] **Step 4: The five Kaumudī rules**

In `crates/panini-prakriya/src/tinanta/sanadi.rs`, add `use crate::prakriya::Prakriya;` as the first `use` line (above `use crate::rule::{Rule, RuleKind};`), and replace

```rust
use panini_data::Pada;

pub(crate) static SANADI: &[Rule] = &[
```

with `use panini_data::{Pada, optional_nic};`, a blank line, and this block (`seg/sanadi_head.rs`, sha256 `6d1b443ff42ca2ce73f5eece05f55bb7b4c42bb18f9cd09bb6a2e33ccd42a04c`), which ends just before the existing 10.0496 comment:

```rust
/// The ṇic-less branch of a root whose ṇic is optional: fires only when
/// `id` is the row's `OPTIONAL_NIC` entry. The root's ṇic-branch pada tag
/// goes, because ṇic is what 1.3.74, 10.0496 and 10.0497 hang on: with it
/// gone they decline on their own guards, and 1.3.78 finds a genuine śeṣa
/// (parasmaipada; ātmanepada blocks). 3.1.25 is barred by the caller's
/// `bars`, so no ṇic is ever added and every ṇic-reading rule below
/// declines. Changes no text.
fn skip_nic(p: &mut Prakriya, id: &'static str, name: &'static str) -> bool {
    if optional_nic(p.ctx.dhatupatha) != Some(id) {
        return false;
    }
    let before = p.snapshot();
    for tag in [Tag::Nic, Tag::Akusmiya, Tag::AaGarviya] {
        p.terms[ANGA].remove(tag);
    }
    p.record(id, name, before);
    true
}

pub(crate) static SANADI: &[Rule] = &[
    // Kaumudī 2564: an idit curādi root takes ṇic optionally (*cintayati* /
    // *cintati*). The first of four vikalpa rules that decide ṇic before the
    // pada gaṇasūtras, as vidyut-prakriya does ("First decide Ric-pratyaya,
    // since this affects the scope of AkusmIya"): an ākusmīya root is
    // ātmanepadī only on its ṇic branch. Keyed on the row's `OPTIONAL_NIC`
    // entry, since idit-ness is a marker `code` no longer carries. The
    // declined branch is the ṇic one and stays index 0. Not a sūtra of the
    // Aṣṭādhyāyī: numbered as vidyut numbers it (`Kaumudi("2564")`).
    Rule {
        id: "2564",
        name: "iditkaraRaM RicaH pAkzikatve liNgam",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "2564", "iditkaraRaM RicaH pAkzikatve liNgam"),
    },
    // Kaumudī 2570: a ñit or udit curādi root takes ṇic optionally
    // (*vañcayate* / *vañcati*). 2564's twin, for its own rows.
    Rule {
        id: "2570",
        name: "YitkaraRasAmarTyAdasya RijvikalpaH",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "2570", "YitkaraRasAmarTyAdasya RijvikalpaH"),
    },
    // Kaumudī 2573.1: `10.0400 pata` takes ṇic optionally (*patati*). Its
    // ṇic branch forks again at 2573.2 below.
    Rule {
        id: "2573.1",
        name: "vA RijantaH",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "2573.1", "vA RijantaH"),
    },
    // Kaumudī 2573.3: `mUtra`, `katra` and `garva` take ṇic optionally
    // (*mūtrati*, *garvati*). vidyut-prakriya keys it on those upadeśas (and
    // `karta`, which has no curādi row), not on their shape.
    Rule {
        id: "2573.3",
        name: "adantatvasAmarTyARRijvikalpaH",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "2573.3", "adantatvasAmarTyARRijvikalpaH"),
    },
    // Kaumudī 2573.2: on `pata`'s ṇic branch, some treat the root as not
    // adanta: its final `a` goes before ṇic exists, so 6.4.48 finds none and
    // sets no `Tag::AtLopa`, and 7.2.116 lengthens the upadhā (*pātayati*).
    // Declined, 6.4.48 deletes the `a` and 7.2.116 declines (*patayati*).
    // The ṇic-less branch bars it (2573.1's `bars`), as vidyut tries it only
    // once ṇic is taken.
    Rule {
        id: "2573.2",
        name: "vA'danta ityeke",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &[],
        apply: |p| {
            if optional_nic(p.ctx.dhatupatha) != Some("2573.1") {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix('a') else {
                return false;
            };
            let stem = stem.to_string();
            let before = p.snapshot();
            p.terms[ANGA].text = stem;
            p.record("2573.2", "vA'danta ityeke", before);
            true
        },
    },
```

- [ ] **Step 5: The two aṅga–śap entries**

In `crates/panini-prakriya/src/tinanta/adesha.rs`, replace the line `pub(crate) static ADESHA: &[Rule] = &[` with this block (`seg/adesha_head.rs`, sha256 `bce002796d3291d7d15eaddce486a699b110f23d31acffebedfda515adb01d8d`), which ends just before the existing four-armed 6.1.101 comment:

```rust
pub(crate) static ADESHA: &[Rule] = &[
    // 6.1.97 ato guṇe, at the aṅga–śap junction: an aṅga ending in a short
    // `a` before śap's `a` gives para-rūpa, katra + a + ti → katrati. Only
    // the ṇic-less branch of an adanta curādi root (Kaumudī 2573.1 / 2573.3:
    // `pata`, `mUtra`, `katra`, `garva`) reaches śap with that `a` still in
    // place; with ṇic, 6.4.48 has deleted it. A second entry under this id,
    // FIRST in the stage because vidyut-prakriya resolves this junction
    // before the vikaraṇa meets the ending: *patāni* credits 6.1.97 here,
    // then 6.1.101's bhvādi arm; *katranti* credits 6.1.97 twice, this entry
    // then the junction entry below. The aṅga loses its final `a`; śap
    // already spells the single vowel that results. The 6.1.101 entry below
    // repeats this body for śap's long `A` rather than share a helper, as
    // this stage's arms do, so each keeps its own mutation pin.
    Rule {
        id: "6.1.97",
        name: "ato guRe",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[SHAP].has(Tag::Thematic) || !p.terms[SHAP].text.starts_with('a') {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix('a') else {
                return false;
            };
            let stem = stem.to_string();
            let before = p.snapshot();
            p.terms[ANGA].text = stem;
            p.record("6.1.97", "ato guRe", before);
            true
        },
    },
    // 6.1.101 akaḥ savarṇe dīrghaḥ, at the same junction once 7.3.101 *ato
    // dīrgho yañi* has lengthened śap before a yañ-initial ending: pata + A +
    // mi → patAmi. The `a` and the `A` are savarṇa, so the long vowel the
    // śap spells is the result. 6.1.97 above declines here: `A` is no guṇa
    // vowel. A second entry under this id, beside the four-armed one below,
    // which reads the vikaraṇa–ending junction instead.
    Rule {
        id: "6.1.101",
        name: "akaH savarRe dIrGaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[SHAP].has(Tag::Thematic) || !p.terms[SHAP].text.starts_with('A') {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix('a') else {
                return false;
            };
            let stem = stem.to_string();
            let before = p.snapshot();
            p.terms[ANGA].text = stem;
            p.record("6.1.101", "akaH savarRe dIrGaH", before);
            true
        },
    },
```

- [ ] **Step 6: The harness adaptations**

**(a)** Every enumerator of a root's derivable padas switches to `Dhatu::padas()`:
- in `crates/panini-analyze/src/lib.rs`, `crates/panini/tests/paradigm/main.rs`, `crates/panini/tests/trace/helpers.rs` and `crates/panini/tests/trace/juhotyadi.rs`, `for &pada in d.pada.padas() {` → `for &pada in d.padas() {` (one occurrence each);
- in `crates/panini-analyze/src/lib.rs` and `crates/panini/tests/paradigm/main.rs`, `d.pada.padas().len()` → `d.padas().len()` (one occurrence each).

Leave every `d.pada.padas()[0]` call site alone: it picks one derivable pada, and for every row it still names one.

**(b)** In `crates/panini/tests/paradigm/main.rs`'s `derivation_set_is_exactly_pinned`, replace

```rust
            let branches = derive(d, lak, *row_pada, pu, va);
            assert_eq!(
                branches[0].text(),
                *expected,
                "index 0 must be the declined derivation for {root} {lakara} cell {cell}"
            );
```

with

```rust
            let branches = derive(d, lak, *row_pada, pu, va);
            // The pinned form is the first LIVE branch, the declined
            // derivation among those that survive. Before slice 10f that was
            // always index 0. An optional-ṇic root's ṇic branch is index 0
            // and, in an ākusmīya or ā-garvīya root's parasmaipada,
            // blocked by 10.0496 / 10.0497: the ṇic-less branch is the cell.
            let first_live = branches
                .iter()
                .find(|p| !p.blocked)
                .unwrap_or_else(|| panic!("no live branch for {root} {lakara} cell {cell}"));
            assert_eq!(
                first_live.text(),
                *expected,
                "the first live branch must be the declined derivation for {root} {lakara} cell {cell}"
            );
```

**(c)** In `crates/panini/tests/roundtrip.rs`, replace `use panini_data::{Lakara, Pada, Purusha, Vacana};` with `use panini_data::{Lakara, Pada, Purusha, Vacana, optional_nic};`, and in `fn roundtrip` replace

```rust
        for p in engine.derive(d, lakara, pada, purusha, vacana) {
            // The cross-product only ever asks for padas the root admits, so
            // nothing here should be blocked. Assert it rather than
            // filtering: a blocked branch appearing would mean `padas()` and
            // the pada-sanction rules (1.3.12 / 1.3.78 / 10.0496) had come apart.
            assert!(
                !p.blocked,
                "{} {} {:?} {:?} {:?} derived a blocked branch",
                d.code,
                panini::lakara_name(lakara),
                pada,
                purusha,
                vacana
            );
```

with

```rust
        let branches = engine.derive(d, lakara, pada, purusha, vacana);
        // The cross-product only ever asks for padas the root admits, so
        // every cell has a live branch, and only a root whose ṇic is
        // optional (`OPTIONAL_NIC`) blocks one: its ṇic and ṇic-less
        // branches can differ in pada (10.0496 / 10.0497 / 1.3.78). Any
        // other blocked branch would mean `padas()` and the pada-sanction
        // rules had come apart.
        let cell = format!(
            "{} {} {:?} {:?} {:?}",
            d.code,
            panini::lakara_name(lakara),
            pada,
            purusha,
            vacana
        );
        assert!(
            branches.iter().any(|p| !p.blocked),
            "{cell} derived no live branch"
        );
        for p in &branches {
            assert!(
                !p.blocked || optional_nic(d.dhatupatha).is_some(),
                "{cell} derived a blocked branch"
            );
        }
        for p in branches.iter().filter(|p| !p.blocked) {
```

The loop body after it (`let form = p.text(); …`) is unchanged and now runs over live branches only.

- [ ] **Step 7: Run the tests**

```bash
mise run fmt
mise exec -- cargo test -p panini-data 2>&1 | grep "test result" | head -1
mise exec -- cargo test -p panini-prakriya 2>&1 | grep "test result" | head -1
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: `panini-data` 25 and `panini-prakriya` 418 (411 + four sanādi + three adesha tests), all green. `trace` 203 and `paradigm` 24 are unchanged and green.

- [ ] **Step 8: Commit**

```bash
mise run lint
git add -A
git commit -m "feat(engine): optional ṇic — Kaumudī 2564/2570/2573.1/2573.3 fork a ṇic-less branch, 2573.2 on pata, and 6.1.97/6.1.101 at the aṅga–śap junction

OPTIONAL_NIC (row → Kaumudī id) and Dhatu::padas in panini-data. The four
triggers lead the sanādi stage, ahead of 10.0496; the applied branch drops
the ṇic-branch pada tag and bars 3.1.25, so 1.3.78 finds a śeṣa. The pinned
form is the first live branch, and roundtrip admits blocked branches on
optional-ṇic rows only. No curated row is optional-ṇic yet, so every golden
is unchanged."
```

---

## Task 3: The rows, their goldens, and every assertion they move

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`, `crates/panini/tests/paradigm/data/curadi.rs`
- Modify: `crates/panini/tests/trace/curadi.rs`, `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes: everything from Task 2; the trace helpers `cell_trace`, `at` and `credited`; `panini_prakriya::derive`; `panini::Panini::check`.
- Produces: the totals Task 4 documents (252 / 13716 / 15644; 1928 alternates; 432 pada-ambiguous surfaces).

Each script below asserts that every `old` occurs exactly once and writes nothing otherwise. If one prints `not applied:`, nothing was written: stop and report, rather than editing around it.

- [ ] **Step 1: The paradigm assertions (failing)**

Create `/tmp/vidyut-full/slice10f/paradigm_10f.py` if it is missing (sha256 `dd68258eb8f6e818bd05224e373af2ccc2275248a081f99b9e4459034d6cd6ba`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10f's edits to crates/panini/tests/paradigm/main.rs (the
census, ALTERNATES keys, docs). Every `old` must occur exactly once; nothing
is written if one fails. Run from the worktree root."""
import sys
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
E = [
("const VIKALPA_RULES: &[&str] = &[\n    \"7.1.35\",",
 "const VIKALPA_RULES: &[&str] = &[\n    \"2564\", \"2570\", \"2573.1\", \"2573.3\", \"2573.2\", \"7.1.35\","),
("/// `ALTERNATES` is otherwise 1664 bare strings,", "/// `ALTERNATES` is otherwise 1928 bare strings,"),
("/// 12996 cells total (1444 root×lakāra blocks × 9), of which 11816 hold exactly one form, 790 hold two, 343 hold three (",
 "/// 13716 cells total (1524 root×lakāra blocks × 9), of which 12364 hold exactly one form, 904 hold two, 389 hold three ("),
("and the eighty-three ubhayapadī adanta roots', new in slice 10e, each by\n/// 7.1.35/8.4.56, plus √bhas's laṅ madhyama eka, by 8.2.74/8.4.56), nineteen hold four (",
 "and the eighty-three ubhayapadī adanta roots', new in slice 10e, each by\n/// 7.1.35/8.4.56, plus √bhas's laṅ madhyama eka, by 8.2.74/8.4.56; slice 10f's\n/// optional-ṇic rows add 114 two-form and 46 three-form cells), twenty-three hold four ("),
("/// slice 3f3 — √jan's vidhiliṅ prathama eka, forking on 6.4.43 alongside 8.4.56), and",
 "/// slice 3f3 — √jan's vidhiliṅ prathama eka, forking on 6.4.43 alongside 8.4.56; and —\n/// new in slice 10f — `mUtra`'s and `katra`'s laṅ and vidhiliṅ parasmaipada\n/// prathama eka, their ṇic and ṇic-less readings × 8.4.56), and"),
("/// seventeen\n/// hold six (", "/// twenty-three\n/// hold six ("),
("/// seventeen cells, not sixteen, and at three mechanisms, not two), and one — new in slice 3c2 — holds SEVEN,\n/// the engine's new record: √hā's loṭ parasmaipada madhyama eka, where 6.4.117\n/// is the first optional rule to bar others (`Rule.bars`), so its three\n/// readings before *hi* are not a 2^k product.",
 "/// seventeen cells, not sixteen, and at three mechanisms, not two; and — new in\n/// slice 10f — six more: `pata`'s laṅ and vidhiliṅ parasmaipada prathama eka\n/// (three readings × 8.4.56) and `mUtra`'s and `katra`'s loṭ parasmaipada\n/// prathama and madhyama eka (two readings × the tātaṅ triple)), one — new in\n/// slice 3c2 — holds SEVEN: √hā's loṭ parasmaipada madhyama eka, where 6.4.117\n/// is the first optional rule to bar others (`Rule.bars`), so its three\n/// readings before *hi* are not a 2^k product; and two — new in slice 10f, the\n/// engine's record — hold NINE: `pata`'s loṭ parasmaipada prathama and madhyama\n/// eka, its three readings (2573.1's ṇic-less *pata-*, 2573.2's *pāta-*, and\n/// 6.4.48's *pata-* with ṇic) × the tātaṅ triple. No cell holds eight."),
("/// itself has 1664 rows, keyed 348 `8.4.56`, 340 `7.1.35`, 340 `7.1.35+8.4.56`,",
 "/// itself has 1928 rows, keyed 354 `8.4.56`, 346 `7.1.35`, 346 `7.1.35+8.4.56`,"),
("/// `7.3.86+7.1.35+8.4.56` (slice 10a's √cur, its sanādi 7.3.86 ahead of 7.1.35) — √kṛ (slice 8b) adds six more",
 "/// `7.3.86+7.1.35+8.4.56` (slice 10a's √cur, its sanādi 7.3.86 ahead of 7.1.35), and slice 10f's twenty-one\n"
 "/// keys on its five Kaumudī vikalpa ids: 36 `2573.1`, 72 `2573.2`, 72 `2573.3`, 8 apiece\n"
 "/// `2564+8.4.56` / `2564+7.1.35` / `2564+7.1.35+8.4.56`, 6 apiece `2573.3+8.4.56` /\n"
 "/// `2573.3+7.1.35` / `2573.3+7.1.35+8.4.56`, and 2 apiece `2570+8.4.56`,\n"
 "/// `2570+7.1.35`, `2570+7.1.35+8.4.56`, `2570+7.3.86+8.4.56`, `2570+7.1.35+7.3.86`,\n"
 "/// `2570+7.1.35+7.3.86+8.4.56` (√div's ṇic-less branch, whose root guṇa the\n"
 "/// MANDATORY 7.3.86 credits), `2573.1+8.4.56`, `2573.1+7.1.35`,\n"
 "/// `2573.1+7.1.35+8.4.56`, `2573.2+8.4.56`, `2573.2+7.1.35` and\n"
 "/// `2573.2+7.1.35+8.4.56`; its other eighteen rows fold into `8.4.56`,\n"
 "/// `7.1.35` and `7.1.35+8.4.56`, six apiece — √kṛ (slice 8b) adds six more"),
("/// cell. 6300 new cells, 498 new rows. The gaṇa is OPEN at 139 of its 509\n/// rows.\n/// This test is what keeps the numbers true day to day.",
 "/// cell. 6300 new cells, 498 new rows. The gaṇa is OPEN at 139 of its 509\n/// rows.\n///\n"
 "/// Slice 10f curates the ten optional-ṇic rows (Kaumudī 2564, 2570, 2573.1,\n"
 "/// 2573.3): six ākusmīya, `garva`, `mUtra`, `katra` and `pata`. Each forks\n"
 "/// into a ṇic branch, at index 0, and a ṇic-less one, parasmaipada by 1.3.78;\n"
 "/// `pata`'s ṇic branch forks again on 2573.2. In an ākusmīya or ā-garvīya\n"
 "/// root's parasmaipada the ṇic branch blocks, so the pinned form is the\n"
 "/// first live branch, the ṇic-less one. 720 new cells, 264 new rows. The\n"
 "/// gaṇa is OPEN at 149 of its 509 rows.\n"
 "/// This test is what keeps the numbers true day to day."),
('    assert_eq!(total_cells, 12996, "1444 root×lakāra blocks × 9 cells each");',
 '    assert_eq!(total_cells, 13716, "1524 root×lakāra blocks × 9 cells each");'),
("    let mut sevens = 0usize;\n", "    let mut sevens = 0usize;\n    let mut nines = 0usize;\n"),
("                7 => sevens += 1,\n", "                7 => sevens += 1,\n                9 => nines += 1,\n"),
('    assert_eq!(ones, 11816, "one-form cells");\n    assert_eq!(twos, 790, "two-form cells");',
 '    assert_eq!(ones, 12364, "one-form cells");\n    assert_eq!(twos, 904, "two-form cells");'),
("        threes, 343,", "        threes, 389,"),
("in slice 10e — the eighty-three ubhayapadī adanta roots', the same way\"",
 "in slice 10e — the eighty-three ubhayapadī adanta roots', the same way; and — new in slice \\\n         10f — the optional-ṇic rows' (2564/2570/2573.x beside 7.1.35/8.4.56)\""),
("        fours, 19,", "        fours, 23,"),
("new in slice 3f3 — √jan's vidhiliṅ prathama eka, forking on 6.4.43 alongside 8.4.56\"",
 "new in slice 3f3 — √jan's vidhiliṅ prathama eka, forking on 6.4.43 alongside 8.4.56; \\\n         and — new in slice 10f — mUtra's and katra's laṅ and vidhiliṅ parasmaipada prathama \\\n         eka, 2573.3 alongside 8.4.56\""),
("        sixes, 17,", "        sixes, 23,"),
("7.1.35/8.4.65/8.4.56 and tanādi's 7.1.35/7.3.86/8.4.56\"",
 "7.1.35/8.4.65/8.4.56 and tanādi's 7.1.35/7.3.86/8.4.56; and — new in slice 10f — \\\n         pata's laṅ and vidhiliṅ parasmaipada prathama eka (2573.1/2573.2 beside 8.4.56) and \\\n         mUtra's and katra's loṭ parasmaipada prathama and madhyama eka (2573.3 beside \\\n         7.1.35/8.4.56)\""),
("        \"seven-form cells — new in slice 3c2, the engine's record: √hā's",
 "        \"seven-form cells — new in slice 3c2, the record until slice 10f: √hā's"),
('    assert_eq!(ALTERNATES.len(), 1664, "ALTERNATES row count");',
 '    assert_eq!(\n'
 '        nines, 2,\n'
 '        "nine-form cells — new in slice 10f, the engine\'s record: pata\'s loṭ parasmaipada \\\n'
 '         prathama and madhyama eka, three readings (2573.1 ṇic-less, 2573.2 pAta-, 6.4.48 \\\n'
 '         pata-) × the tātaṅ triple (7.1.35/8.4.56)"\n'
 '    );\n\n'
 '    assert_eq!(ALTERNATES.len(), 1928, "ALTERNATES row count");'),
('    assert_eq!(key_count("8.4.56"), 348, "8.4.56-only alternates");\n    assert_eq!(key_count("7.1.35"), 340, "7.1.35-only alternates");\n    assert_eq!(key_count("7.1.35+8.4.56"), 340, "7.1.35+8.4.56 alternates");',
 '    assert_eq!(key_count("8.4.56"), 354, "8.4.56-only alternates");\n    assert_eq!(key_count("7.1.35"), 346, "7.1.35-only alternates");\n    assert_eq!(key_count("7.1.35+8.4.56"), 346, "7.1.35+8.4.56 alternates");'),
('''        "7.3.86+7.1.35+8.4.56 alternates"
    );
}''', '''        "7.3.86+7.1.35+8.4.56 alternates"
    );
    // Slice 10f's five Kaumudī vikalpa ids, alone and stacked.
    for (key, n) in [
        ("2573.1", 36),
        ("2573.2", 72),
        ("2573.3", 72),
        ("2564+8.4.56", 8),
        ("2564+7.1.35", 8),
        ("2564+7.1.35+8.4.56", 8),
        ("2573.3+8.4.56", 6),
        ("2573.3+7.1.35", 6),
        ("2573.3+7.1.35+8.4.56", 6),
        ("2570+8.4.56", 2),
        ("2570+7.1.35", 2),
        ("2570+7.1.35+8.4.56", 2),
        ("2570+7.3.86+8.4.56", 2),
        ("2570+7.1.35+7.3.86", 2),
        ("2570+7.1.35+7.3.86+8.4.56", 2),
        ("2573.1+8.4.56", 2),
        ("2573.1+7.1.35", 2),
        ("2573.1+7.1.35+8.4.56", 2),
        ("2573.2+8.4.56", 2),
        ("2573.2+7.1.35", 2),
        ("2573.2+7.1.35+8.4.56", 2),
    ] {
        assert_eq!(key_count(key), n, "{key} alternates");
    }
}'''),
("    // ninety-six to 420. The nine ā-garvīya roots, ātmanepada-only, contribute\n    // nothing; nor do `kUwa` and `vizka`, whose ātmanepada the ākusmīya `kUwa~`\n    // and `vizka~` share, beyond their own four.",
 "    // ninety-six to 420. The nine ā-garvīya roots, ātmanepada-only, contribute\n    // nothing; nor do `kUwa` and `vizka`, whose ātmanepada the ākusmīya `kUwa~`\n    // and `vizka~` share, beyond their own four.\n"
 "    // Slice 10f's `mUtra`, `katra` and `pata` contribute the same four each from\n"
 "    // their ṇic branch (`amUtrayata`, …), which alone derives in both padas —\n"
 "    // twelve more, taking the set from 420 to 432. The ākusmīya rows and\n"
 "    // `garva` derive each pada on a different branch (ṇic-less parasmaipada,\n"
 "    // ṇic ātmanepada), and those surfaces never meet: they contribute nothing."),
]
bad = []
for old, new in E:
    n = s.count(old)
    if n != 1:
        bad.append(f"{n}× {old[:80]!r}")
        continue
    s = s.replace(old, new)
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
open(p, 'w').write(s)
print(f"applied {len(E)} edits")
```

```bash
python3 /tmp/vidyut-full/slice10f/paradigm_10f.py      # applied 25 edits
```

What it changes, so a reviewer can check it against the spec:
- `VIKALPA_RULES` gains the five Kaumudī ids;
- the census: 13716 cells (1524 blocks), buckets 12364 / 904 / 389 / 23 / 10 / 23 / 1 and a new nine-form arm (2, `pata`'s loṭ parasmaipada prathama and madhyama eka), with their prose;
- `ALTERNATES`: 1928 rows; `8.4.56` / `7.1.35` / `7.1.35+8.4.56` 354 / 346 / 346; twenty-one new keys on the Kaumudī ids;
- the 10f paragraph, and the pada-ambiguous comment (420 → 432). The set itself is pinned in Step 6, from the measured failure.

- [ ] **Step 2: The trace assertions and tests (failing)**

Create `/tmp/vidyut-full/slice10f/trace_10f.py` if it is missing (sha256 `acc880e5de791cb7920598b54978f4c08cc6a98f20860dee56d557797b99de50`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10f's edits to crates/panini/tests/trace/{curadi,juhotyadi}.rs.
Every `old` must occur exactly once; nothing is written if one fails."""
import sys
CURADI = 'crates/panini/tests/trace/curadi.rs'
JUHO = 'crates/panini/tests/trace/juhotyadi.rs'
NEW_TESTS = r'''

/// The curated rows whose ṇic is optional: exactly `OPTIONAL_NIC`'s.
fn optional_nic_rows() -> Vec<&'static panini_data::Dhatu> {
    dhatus()
        .iter()
        .filter(|d| optional_nic(d.dhatupatha).is_some())
        .collect()
}

#[test]
#[allow(non_snake_case)]
fn daMSati_trace_has_no_nic_and_no_a_kusmad() {
    // danS P laT P.E. Kaumudī 2564 takes the ṇic-less branch first, before
    // any pada rule; with no ṇic, 10.0496 declines and 1.3.78 sanctions the
    // parasmaipada. Then the ordinary thematic core, and 8.3.24 on the root's
    // own `n`. Index 0 is the ṇic branch, which 10.0496 blocked with nothing
    // recorded; the cell is the first live branch.
    let d = dhatus().iter().find(|d| d.dhatupatha == "10.0193").unwrap();
    let branches = derive(
        d,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert!(branches[0].blocked && branches[0].log.is_empty());
    let live: Vec<_> = branches.iter().filter(|p| !p.blocked).collect();
    assert_eq!(live.len(), 1);
    let t: Vec<&str> = live[0].log.iter().map(|s| s.sutra.as_str()).collect();
    assert_eq!(live[0].text(), "daMSati", "got {t:?}");
    assert_eq!(
        t,
        ["2564", "1.3.78", "3.4.78", "1.3.9", "3.1.68", "1.3.9", "8.3.24"],
    );
}

#[test]
fn pata_traces_its_three_readings() {
    // pata P laT P.E. Index 0 is the ṇic branch with 2573.2 declined (6.4.48
    // deletes the `a`; 7.2.116 declines). 2573.2 deletes it before ṇic, so
    // 7.2.116 lengthens. 2573.1 takes no ṇic, and the aṅga's `a` meets śap's
    // by 6.1.97's aṅga–śap entry.
    let d = dhatus().iter().find(|d| d.dhatupatha == "10.0400").unwrap();
    let branches = derive(
        d,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    let got: Vec<(String, Vec<&str>)> = branches
        .iter()
        .map(|p| {
            assert!(!p.blocked, "{}", p.text());
            (p.text(), p.log.iter().map(|s| s.sutra.as_str()).collect())
        })
        .collect();
    let core = ["1.3.78", "3.4.78", "1.3.9", "3.1.68", "1.3.9"];
    let with_nic = |head: &[&'static str], upadha: &'static str| {
        let mut v: Vec<&str> = head.to_vec();
        v.extend(["3.1.25", "1.3.9", "3.4.114", upadha, "3.1.32"]);
        v.extend(core);
        v.extend(["7.3.84", "6.1.78"]);
        v
    };
    let mut nicless = vec!["2573.1"];
    nicless.extend(core);
    nicless.push("6.1.97");
    assert_eq!(
        got,
        [
            ("patayati".to_string(), with_nic(&[], "6.4.48")),
            ("pAtayati".to_string(), with_nic(&["2573.2"], "7.2.116")),
            ("patati".to_string(), nicless),
        ],
    );
}

#[test]
fn the_optional_nic_ids_are_credited_only_on_their_rows() {
    // Each Kaumudī id fires only on the rows `OPTIONAL_NIC` gives it, and on
    // every cell of each: one ṇic-less branch per cell in each pada (live in
    // parasmaipada, blocked by 1.3.78 in ātmanepada). 2573.2 fires only on
    // `pata`. Goldens ignore traces, so this is also what holds all five
    // inert on the 242 prior roots.
    for (id, rows) in [
        ("2564", &["10.0193", "10.0194", "10.0198", "10.0199"][..]),
        ("2570", &["10.0227", "10.0230"][..]),
        ("2573.1", &["10.0400"][..]),
        ("2573.2", &["10.0400"][..]),
        ("2573.3", &["10.0449", "10.0451", "10.0456"][..]),
    ] {
        for (number, _) in credited(id) {
            assert!(rows.contains(&number), "{id} credited on {number}");
        }
    }
    for d in optional_nic_rows() {
        let id = optional_nic(d.dhatupatha).unwrap();
        for lakara in [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin] {
            for purusha in [Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama] {
                for vacana in [Vacana::Eka, Vacana::Dvi, Vacana::Bahu] {
                    for pada in [Pada::Parasmaipada, Pada::Atmanepada] {
                        let cell =
                            format!("{} {pada:?} {lakara:?} {purusha:?} {vacana:?}", d.dhatupatha);
                        let nicless: Vec<_> = derive(d, lakara, pada, purusha, vacana)
                            .into_iter()
                            .filter(|p| p.log.first().is_some_and(|s| s.sutra == id))
                            .collect();
                        assert!(!nicless.is_empty(), "{cell}: no ṇic-less branch");
                        for p in &nicless {
                            assert_eq!(p.blocked, pada == Pada::Atmanepada, "{cell}");
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn no_nic_pada_rule_reaches_a_nicless_branch() {
    // On a ṇic-less branch the root's ṇic-branch pada tag is gone: 10.0496,
    // 10.0497 and 1.3.74 never fire, 3.1.25 never adds ṇic, and the live
    // branches are 1.3.78's. On the ṇic branch of an ākusmīya or ā-garvīya
    // row, parasmaipada is blocked before anything is recorded — the ṇic
    // branch is the declined one, so not even the Kaumudī id is credited.
    let nic_only = [
        "10.0496", "10.0497", "1.3.74", "3.1.25", "3.4.114", "6.4.48", "7.2.116", "3.1.32",
    ];
    for d in optional_nic_rows() {
        let id = optional_nic(d.dhatupatha).unwrap();
        for lakara in [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin] {
            for purusha in [Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama] {
                for vacana in [Vacana::Eka, Vacana::Dvi, Vacana::Bahu] {
                    for pada in [Pada::Parasmaipada, Pada::Atmanepada] {
                        let cell =
                            format!("{} {pada:?} {lakara:?} {purusha:?} {vacana:?}", d.dhatupatha);
                        for p in derive(d, lakara, pada, purusha, vacana) {
                            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
                            if ids.first() == Some(&id) {
                                for absent in nic_only {
                                    assert!(!ids.contains(&absent), "{cell} {absent}: {ids:?}");
                                }
                                if !p.blocked {
                                    assert!(ids.contains(&"1.3.78"), "{cell}: {ids:?}");
                                }
                            } else if p.blocked {
                                assert!(
                                    matches!(d.pada, PadaAssignment::Akusmiya | PadaAssignment::AaGarviya)
                                        && pada == Pada::Parasmaipada,
                                    "{cell}: a ṇic branch blocked"
                                );
                                assert!(p.log.is_empty(), "{cell}: {ids:?}");
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn the_anga_shap_junction_resolves_before_the_ending() {
    // The ṇic-less branch of an adanta optional-ṇic row reaches śap with its
    // final `a`. Where 7.3.101 has lengthened śap (uttama), 6.1.101's aṅga–śap
    // entry merges the two (mUtrAmi); before `anti`, 6.1.97 fires twice, its
    // aṅga–śap entry and then its vikaraṇa–ending one (mUtranti). That these
    // head-of-stage entries fire on no prior root is the main↔branch trace
    // dump's finding; their guards are pinned in `adesha.rs`.
    let d = dhatus().iter().find(|d| d.dhatupatha == "10.0451").unwrap();
    let branches = derive(d, Lakara::Lat, Pada::Parasmaipada, Purusha::Uttama, Vacana::Eka);
    let nicless = branches.iter().find(|p| p.text() == "mUtrAmi").unwrap();
    let ids: Vec<String> = nicless.log.iter().map(|s| s.sutra.clone()).collect();
    assert!(at(&ids, "7.3.101") < at(&ids, "6.1.101"), "got {ids:?}");
    let branches = derive(d, Lakara::Lat, Pada::Parasmaipada, Purusha::Prathama, Vacana::Bahu);
    let nicless = branches.iter().find(|p| p.text() == "mUtranti").unwrap();
    let n = nicless.log.iter().filter(|s| s.sutra == "6.1.97").count();
    assert_eq!(n, 2, "aṅga–śap, then śap–anti");
}
'''
C = [
("//! nor 7.3.86; an ā-garvīya root's also opens with the gaṇasūtra 10.0497,\n//! and has no pada sūtra.\n",
 "//! nor 7.3.86; an ā-garvīya root's also opens with the gaṇasūtra 10.0497,\n//! and has no pada sūtra. A root whose ṇic is optional (`OPTIONAL_NIC`)\n"
 "//! has a ṇic-less branch besides, opening with its Kaumudī id (2564, 2570,\n//! 2573.1, 2573.3) and running the bhvādi path with 1.3.78.\n"),
("use panini_data::{\n    AA_GARVIYA, AKUSMIYA, Gana, JNAPADI, Lakara, Pada, PadaAssignment, Purusha, Vacana, dhatus,\n};",
 "use panini_data::{\n    AA_GARVIYA, AKUSMIYA, Gana, JNAPADI, Lakara, Pada, PadaAssignment, Purusha, Vacana, dhatus,\n    optional_nic,\n};"),
("    // no later rule ran on the branch. The rows are found by the positional\n    // `AKUSMIYA` range, not by the curated `pada` column.\n    let rows: Vec<_> = dhatus()\n        .iter()\n        .filter(|d| d.gana == Gana::Curadi && AKUSMIYA.contains(&d.dhatupatha))\n        .collect();\n    assert_eq!(rows.len(), 37, \"curated ākusmīya rows\");",
 "    // no later rule ran on the branch. The rows are found by the positional\n    // `AKUSMIYA` range, not by the curated `pada` column. The six whose ṇic is\n    // optional (slice 10f) derive their parasmaipada on the ṇic-less branch;\n    // `no_nic_pada_rule_reaches_a_nicless_branch` holds them.\n    let rows: Vec<_> = dhatus()\n        .iter()\n        .filter(|d| {\n            d.gana == Gana::Curadi\n                && AKUSMIYA.contains(&d.dhatupatha)\n                && optional_nic(d.dhatupatha).is_none()\n        })\n        .collect();\n    assert_eq!(rows.len(), 37, \"curated ākusmīya rows with ṇic\");"),
("    // 10.0496 fires on every ātmanepada cell of the 37 curated ākusmīya\n    // rows — 37 roots × 4 lakāras × 9 cells, one branch each — and nowhere\n    // else: every credit's number lies in the positional `AKUSMIYA` range.\n    // And 1.3.74 never reaches them: its credits stay on the 93 `Nic` rows,\n    // read from the curated `pada` column (ten before slice 10e, listed\n    // literally until then).\n    let hits = credited(\"10.0496\");\n    assert_eq!(hits.len(), 1332);",
 "    // 10.0496 fires on every ātmanepada cell of the 43 curated ākusmīya\n    // rows — 43 roots × 4 lakāras × 9 cells, one branch each, the six\n    // optional-ṇic rows' on their ṇic branch — and nowhere else: every\n    // credit's number lies in the positional `AKUSMIYA` range. And 1.3.74\n    // never reaches them: its credits stay on the 96 `Nic` rows, read from\n    // the curated `pada` column (ten before slice 10e, listed literally until\n    // then).\n    let hits = credited(\"10.0496\");\n    assert_eq!(hits.len(), 43 * 36);"),
("    assert_eq!(nic.len(), 93, \"curated 1.3.74 rows\");", "    assert_eq!(nic.len(), 96, \"curated 1.3.74 rows\");"),
("    // 6.4.48 fires on every live branch of the 92 adanta rows — 83\n    // ubhayapadī roots × 78 branches and 9 ā-garvīya roots × 36 — and\n    // nowhere else. Neither 7.2.116 nor 7.3.86 is credited on any of them:\n    // that is 1.1.57's block, held corpus-wide. Goldens ignore traces, so\n    // this is also what holds 6.4.48 inert on the 150 prior roots.\n    let adanta = adanta_rows();\n    assert_eq!(adanta.len(), 92);\n    let hits = credited(\"6.4.48\");\n    assert_eq!(hits.len(), 83 * 78 + 9 * 36);\n    for (number, _) in &hits {\n        assert!(adanta.contains(number), \"6.4.48 credited on {number}\");\n    }",
 "    // 6.4.48 fires on every live branch of the 92 adanta rows with ṇic — 83\n    // ubhayapadī roots × 78 branches and 9 ā-garvīya roots × 36 — and, since\n    // slice 10f, on the ṇic branches of the four adanta rows whose ṇic is\n    // optional (`pata`'s with 2573.2 declined), and nowhere else. Neither\n    // 7.2.116 nor 7.3.86 is credited on any of the 92: that is 1.1.57's\n    // block, held corpus-wide. Goldens ignore traces, so this is also what\n    // holds 6.4.48 inert on the 150 roots before 10e.\n    let adanta = adanta_rows();\n    assert_eq!(adanta.len(), 92);\n    let hits = credited(\"6.4.48\");\n    let (with_nic, optional): (Vec<_>, Vec<_>) =\n        hits.iter().partition(|(n, _)| adanta.contains(n));\n    assert_eq!(with_nic.len(), 83 * 78 + 9 * 36);\n    for (number, _) in &optional {\n        assert!(\n            [\"10.0400\", \"10.0449\", \"10.0451\", \"10.0456\"].contains(number),\n            \"6.4.48 credited on {number}\"\n        );\n    }"),
("    // the positional `AA_GARVIYA` range. Their parasmaipada derives only\n    // blocked branches, with nothing recorded: the block is 10.0497's.\n    let hits = credited(\"10.0497\");\n    assert_eq!(hits.len(), 9 * 36);",
 "    // the positional `AA_GARVIYA` range — ten rows since slice 10f, `garva`'s\n    // on its ṇic branch. The nine with ṇic derive only blocked branches in\n    // parasmaipada, with nothing recorded: the block is 10.0497's. `garva`\n    // derives its parasmaipada on the ṇic-less branch.\n    let hits = credited(\"10.0497\");\n    assert_eq!(hits.len(), 10 * 36);"),
("        .filter(|d| d.gana == Gana::Curadi && AA_GARVIYA.contains(&d.dhatupatha))\n        .collect();\n    assert_eq!(rows.len(), 9, \"curated ā-garvīya rows\");",
 "        .filter(|d| {\n            d.gana == Gana::Curadi\n                && AA_GARVIYA.contains(&d.dhatupatha)\n                && optional_nic(d.dhatupatha).is_none()\n        })\n        .collect();\n    assert_eq!(rows.len(), 9, \"curated ā-garvīya rows with ṇic\");"),
("/// The curated adanta rows: curādi, with an `a`-final code. No row curated\n/// before slice 10e has one, so this is exactly 10e's ninety-two.\nfn adanta_rows() -> Vec<&'static str> {\n    dhatus()\n        .iter()\n        .filter(|d| d.gana == Gana::Curadi && d.code.ends_with('a'))",
 "/// The curated adanta rows that take ṇic: curādi, with an `a`-final code,\n/// and not in `OPTIONAL_NIC`. No row curated before slice 10e has an\n/// `a`-final code, so this is exactly 10e's ninety-two; 10f's four adanta\n/// rows (`pata`, `garva`, `mUtra`, `katra`) are optional-ṇic.\nfn adanta_rows() -> Vec<&'static str> {\n    dhatus()\n        .iter()\n        .filter(|d| {\n            d.gana == Gana::Curadi\n                && d.code.ends_with('a')\n                && optional_nic(d.dhatupatha).is_none()\n        })"),
]
J = [
("    // Since slice 10e it also admits a curādi root's own `n` before a jhal:\n    // 10c's √gandh and seven adanta roots, on every live branch.\n    const CURADI: [&str; 8] = [\n        \"10.0204\", \"10.0433\", \"10.0460\", \"10.0467\", \"10.0471\", \"10.0472\", \"10.0473\", \"10.0474\",\n    ];",
 "    // Since slice 10e it also admits a curādi root's own `n` before a jhal:\n    // 10c's √gandh and seven adanta roots, on every live branch, and since\n    // slice 10f five optional-ṇic ākusmīya roots (`danS`, `dans`, `tantr`,\n    // `mantr`, `vanc`) on both their branches.\n    const CURADI: [&str; 13] = [\n        \"10.0204\", \"10.0433\", \"10.0460\", \"10.0467\", \"10.0471\", \"10.0472\", \"10.0473\", \"10.0474\",\n        \"10.0193\", \"10.0194\", \"10.0198\", \"10.0199\", \"10.0227\",\n    ];"),
("    // √gandh's 36 ātmanepada branches and the seven ubhayapadī roots' 78 each.\n    assert_eq!(curadi.len(), 36 + 7 * 78);",
 "    // √gandh's 36 ātmanepada branches, the seven ubhayapadī roots' 78 each,\n    // and the five optional-ṇic roots' 78 each (42 ṇic-less parasmaipada, 36\n    // ṇic ātmanepada).\n    assert_eq!(curadi.len(), 36 + 7 * 78 + 5 * 78);"),
]
out = {}
bad = []
for path, edits in [(CURADI, C), (JUHO, J)]:
    s = open(path).read()
    for old, new in edits:
        n = s.count(old)
        if n != 1:
            bad.append(f"{path}: {n}× {old[:70]!r}")
            continue
        s = s.replace(old, new)
    out[path] = s
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
out[CURADI] += NEW_TESTS
for path, s in out.items():
    open(path, 'w').write(s)
print(f"applied {len(C) + len(J)} edits and five tests")
```

```bash
python3 /tmp/vidyut-full/slice10f/trace_10f.py      # applied 11 edits and five tests
```

It moves five existing assertions (the ākusmīya and ā-garvīya block tests now skip the optional-ṇic rows, 10.0496 on 43 × 36, 96 `Nic` rows, 6.4.48 on the 92 plus the four optional adanta rows' ṇic branches, 10.0497 on 10 × 36, 8.3.24 on five more ākusmīya roots × 78) and adds five tests: `daMSati_trace_has_no_nic_and_no_a_kusmad`, `pata_traces_its_three_readings`, `the_optional_nic_ids_are_credited_only_on_their_rows`, `no_nic_pada_rule_reaches_a_nicless_branch`, `the_anga_shap_junction_resolves_before_the_ending`.

- [ ] **Step 3: The data-layer assertions and docs (failing)**

Create `/tmp/vidyut-full/slice10f/data_10f.py` if it is missing (sha256 `26a913680feaa654cea8bd55b44c7884fbeb606e7362c768fc60103275dbb5fb`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10f's edits to crates/panini-data/src/lib.rs (tests and
docs, not the rows or the table). Every `old` must occur exactly once."""
import sys
p = 'crates/panini-data/src/lib.rs'
s = open(p).read()
E = [
# PadaAssignment docs that 10f falsifies
("    /// reads the same range (`maybe_find_antargana`). The gaṇasūtra applies\n    /// only on the ṇic branch; this engine has no optional-ṇic roots yet, so\n    /// every curated ākusmīya row takes ṇic.\n    Akusmiya,",
 "    /// reads the same range (`maybe_find_antargana`). The gaṇasūtra applies\n    /// only on the ṇic branch: a row whose ṇic is optional (`OPTIONAL_NIC`,\n    /// slice 10f's six) is still `Akusmiya`, the ṇic branch's verdict, and\n    /// `Dhatu::padas` adds the ṇic-less branch's parasmaipada.\n    Akusmiya,"),
("    /// lists the same ten upadeśas (`AA_GARVIYA`). The gaṇasūtra applies only\n    /// on the ṇic branch; `10.0449 garva`, whose ṇic is optional, is not\n    /// curated yet, so every curated ā-garvīya row takes ṇic.\n    AaGarviya,",
 "    /// lists the same ten upadeśas (`AA_GARVIYA`). The gaṇasūtra applies only\n    /// on the ṇic branch: `10.0449 garva`, whose ṇic is optional (`OPTIONAL_NIC`),\n    /// is still `AaGarviya`, and `Dhatu::padas` adds its ṇic-less parasmaipada.\n    AaGarviya,"),
("/// `curated_pada_agrees_with_upadesha_markers` holds both sides. Includes\n/// `10.0449 garva`, which is ā-garvīya but not yet curated (its ṇic is\n/// optional). `aa_garviya_is_exactly_the_rows_10_0497_names` pins the range\n/// to upstream.",
 "/// `curated_pada_agrees_with_upadesha_markers` holds both sides. Includes\n/// `10.0449 garva`, whose ṇic is optional (`OPTIONAL_NIC`): 10.0497 makes\n/// only its ṇic branch ātmanepadī. `aa_garviya_is_exactly_the_rows_10_0497_names`\n/// pins the range to upstream."),
# Dhatu::pada doc census
("    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 242\n    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and\n    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed\n    /// exception, ninety-three curādi rows' are 1.3.74's, 37 ākusmīya rows'\n    /// the gaṇasūtra 10.0496's and nine ā-garvīya rows' the gaṇasūtra\n    /// 10.0497's, each asserted explicitly from both sides, the same way\n    /// `dhatupatha_numbers_resolve_upstream` holds `code` to upstream.",
 "    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 252\n    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and\n    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed\n    /// exception, ninety-six curādi rows' are 1.3.74's, 43 ākusmīya rows'\n    /// the gaṇasūtra 10.0496's and ten ā-garvīya rows' the gaṇasūtra\n    /// 10.0497's, each asserted explicitly from both sides, the same way\n    /// `dhatupatha_numbers_resolve_upstream` holds `code` to upstream. For\n    /// the ten curādi rows whose ṇic is optional this is the ṇic branch's\n    /// pada; the test also re-derives their ṇic-less branch's, 1.3.78's."),
("    /// The test covers the 242 roots curated here, not the dhātupāṭha's 2259.",
 "    /// The test covers the 252 roots curated here, not the dhātupāṭha's 2259."),
("        assert_eq!(dhatus().len(), 242);", "        assert_eq!(dhatus().len(), 252);"),
("    fn curadi_rows_are_the_one_hundred_thirty_nine_curated_roots() {",
 "    fn curadi_rows_are_the_one_hundred_forty_nine_curated_roots() {"),
("        // (`AA_GARVIYA`) are ātmanepadī by 10.0497. The gaṇa is OPEN at 139\n        // of its 509 dhātupāṭha rows.",
 "        // (`AA_GARVIYA`) are ātmanepadī by 10.0497. Slice 10f adds the ten\n        // rows whose ṇic is optional (`OPTIONAL_NIC`): six ākusmīya, `garva`,\n        // `mUtra`, `katra` and `pata`, each curated with its ṇic branch's\n        // pada. The gaṇa is OPEN at 149 of its 509 dhātupāṭha rows."),
# padas() pin: Dhatu::padas
("    #[test]\n    fn ubhayapada_padas_are_parasmaipada_first() {",
 "    #[test]\n    fn dhatu_padas_add_the_nicless_parasmaipada() {\n"
 "        // A root whose ṇic is optional derives its ṇic branch's padas plus the\n"
 "        // ṇic-less branch's parasmaipada, parasmaipada first. One row per\n"
 "        // ṇic-branch assignment, and a row outside the table for contrast.\n"
 "        let padas = |n: &str| dhatus().iter().find(|d| d.dhatupatha == n).unwrap().padas();\n"
 "        let both = &[Pada::Parasmaipada, Pada::Atmanepada];\n"
 "        assert_eq!(padas(\"10.0193\"), both, \"ākusmīya daSi~\");\n"
 "        assert_eq!(padas(\"10.0449\"), both, \"ā-garvīya garva\");\n"
 "        assert_eq!(padas(\"10.0451\"), both, \"1.3.74 mUtra\");\n"
 "        assert_eq!(padas(\"10.0192\"), &[Pada::Atmanepada], \"ākusmīya cita~, ṇic only\");\n"
 "        assert_eq!(padas(\"10.0440\"), &[Pada::Atmanepada], \"ā-garvīya pada, ṇic only\");\n"
 "        assert_eq!(padas(\"01.0001\"), &[Pada::Parasmaipada]);\n"
 "        for d in dhatus() {\n"
 "            if optional_nic(d.dhatupatha).is_none() {\n"
 "                assert_eq!(d.padas(), d.pada.padas(), \"{}\", d.dhatupatha);\n"
 "            }\n"
 "        }\n"
 "    }\n\n"
 "    #[test]\n    fn ubhayapada_padas_are_parasmaipada_first() {"),
# pada agreement: ṇic-less branch
("                assert_eq!(d.pada, want, \"{} is curādi and {why}\", d.dhatupatha);\n                continue;",
 "                assert_eq!(d.pada, want, \"{} is curādi and {why}\", d.dhatupatha);\n"
 "                // That is the ṇic branch's pada. A row whose ṇic is optional\n"
 "                // also derives without it, where its own markers decide by\n"
 "                // 1.3.12 / 1.3.72 / 1.3.78: parasmaipada for every row listed.\n"
 "                if optional_nic(d.dhatupatha).is_some() {\n"
 "                    assert_eq!(\n"
 "                        derived,\n"
 "                        PadaAssignment::Parasmaipada,\n"
 "                        \"{} {upadesha}: its ṇic-less branch is 1.3.78's\",\n"
 "                        d.dhatupatha\n"
 "                    );\n"
 "                    assert!(d.padas().contains(&Pada::Parasmaipada), \"{}\", d.dhatupatha);\n"
 "                }\n"
 "                continue;"),
# the marker test, before aa_garviya_is_exactly...
("    #[test]\n    fn aa_garviya_is_exactly_the_rows_10_0497_names() {",
 "    /// The Kaumudī id that makes a curādi upadeśa's ṇic optional, read from\n"
 "    /// the upadeśa as vidyut-prakriya reads it: 2564 for an idit root (last\n"
 "    /// marker `i~`), 2570 for a ñit or udit one (last marker `Y` or `u~`),\n"
 "    /// 2573.1 for `pata`, and 2573.3 for the three roots the Kaumudī names\n"
 "    /// there. 2573.3 is a list, not a shape: `Cidra`, `sUtra` and the rest\n"
 "    /// have a conjunct before their final `a` and take ṇic. Only the triggers\n"
 "    /// slice 10f curates; the ādhṛṣīya / āsvadīya gaṇasūtras (10.0498,\n"
 "    /// 10.0499) and 2565 / 2571 / 2572 are later slices'.\n"
 "    fn optional_nic_from_upadesha(upadesha: &str) -> Option<&'static str> {\n"
 "        let u = upadesha.trim_end_matches(['\\\\', '^']);\n"
 "        if u.ends_with(\"i~\") {\n"
 "            Some(\"2564\")\n"
 "        } else if u.ends_with(\"u~\") || u.ends_with('Y') {\n"
 "            Some(\"2570\")\n"
 "        } else if u == \"pata\" {\n"
 "            Some(\"2573.1\")\n"
 "        } else if [\"mUtra\", \"katra\", \"garva\"].contains(&u) {\n"
 "            Some(\"2573.3\")\n"
 "        } else {\n"
 "            None\n"
 "        }\n"
 "    }\n\n"
 "    #[test]\n"
 "    fn optional_nic_matches_upadesha_markers() {\n"
 "        // Every table entry is what its upadeśa says, and every curated\n"
 "        // curādi row whose upadeśa says so is in the table: the table is\n"
 "        // exactly the curated rows those markers select. Non-circular, as\n"
 "        // `dhatupatha_numbers_resolve_upstream` is: the upadeśa comes from\n"
 "        // the vendored dhātupāṭha, not from the table.\n"
 "        let rows = upstream_rows();\n"
 "        let upadesha = |n: &str| rows.iter().find(|(m, _, _)| *m == n).unwrap().1;\n"
 "        for (n, id) in OPTIONAL_NIC {\n"
 "            assert_eq!(optional_nic_from_upadesha(upadesha(n)), Some(*id), \"{n}\");\n"
 "            assert!(dhatus().iter().any(|d| d.dhatupatha == *n), \"{n} is not curated\");\n"
 "        }\n"
 "        for d in dhatus().iter().filter(|d| d.gana == Gana::Curadi) {\n"
 "            assert_eq!(\n"
 "                optional_nic(d.dhatupatha),\n"
 "                optional_nic_from_upadesha(upadesha(d.dhatupatha)),\n"
 "                \"{} {}\",\n"
 "                d.dhatupatha,\n"
 "                upadesha(d.dhatupatha)\n"
 "            );\n"
 "        }\n"
 "        assert_eq!(OPTIONAL_NIC.len(), 10);\n"
 "        // 2573.3 is the Kaumudī's list: a conjunct-before-`a` curādi root\n"
 "        // outside it (`10.0469 Cidra`, curated in 10e) is no optional-ṇic row.\n"
 "        assert_eq!(optional_nic_from_upadesha(\"Cidra\"), None);\n"
 "        assert_eq!(optional_nic(\"10.0469\"), None);\n"
 "    }\n\n"
 "    #[test]\n    fn aa_garviya_is_exactly_the_rows_10_0497_names() {"),
("        // same ten upadeśas (`AA_GARVIYA`). `garva` is not curated, so this\n        // test — not a derivation — is what holds the range's upper end.",
 "        // same ten upadeśas (`AA_GARVIYA`). `garva`'s ṇic is optional, so this\n        // test — not only its ṇic branch's derivations — holds the range's\n        // upper end."),
]
bad = []
for old, new in E:
    n = s.count(old)
    if n != 1:
        bad.append(f"{n}× {old[:80]!r}")
        continue
    s = s.replace(old, new)
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
open(p, 'w').write(s)
print(f"applied {len(E)} edits")
```

```bash
python3 /tmp/vidyut-full/slice10f/data_10f.py      # applied 12 edits
```

It updates the three docs 10f falsifies (`PadaAssignment::Akusmiya`, `::AaGarviya`, `AA_GARVIYA`) and `Dhatu::pada`'s census (252; 96 / 43 / ten), the row count (252) and the curādi list test's name and comment, and adds `dhatu_padas_add_the_nicless_parasmaipada`, `optional_nic_from_upadesha` with `optional_nic_matches_upadesha_markers`, and the pada-agreement test's ṇic-less arm.

- [ ] **Step 4: The `check()` witnesses (failing)**

Create `/tmp/vidyut-full/slice10f/check_10f.rs` (sha256 `b997e285d95140ead39756aa83e96de316fe8c0ffac17a7d61626759cf1f4bdc`) and `/tmp/vidyut-full/slice10f/insert_check_10f.py` (sha256 `2e4cf26fe2c38a2cae37be213b72e78dca4ac698ea922aefb5192327e7181093`) if they are missing:

```rust

/// Slice 10f's `check()` witnesses, one per row class × pada of the spec's
/// Forms table. A ṇic-less analysis opens with its Kaumudī id, credits
/// 1.3.78 and never 3.1.25; a ṇic one credits 3.1.25 and never a Kaumudī
/// id but 2573.2. The goldens were grepped for every witness first: each is
/// its own row's alone. The shapes the slice rules out — ṇic in an ākusmīya
/// or ā-garvīya root's parasmaipada, no ṇic in any ātmanepada, and the
/// unmerged aṅga–śap junction — derive nothing.
#[test]
fn curadi_analyses_its_optional_nic_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    let has = |ids: &[String], id: &str| ids.iter().any(|i| i == id);
    // (form, root, pada, the Kaumudī id that opens it, or None for ṇic)
    for (form, dhatu, pada, trigger) in [
        ("daMSati", "danS", Pada::Parasmaipada, Some("2564")),
        ("daMSayate", "danS", Pada::Atmanepada, None),
        ("vaYcati", "vanc", Pada::Parasmaipada, Some("2570")),
        ("devati", "div", Pada::Parasmaipada, Some("2570")),
        ("devayate", "div", Pada::Atmanepada, None),
        ("garvati", "garva", Pada::Parasmaipada, Some("2573.3")),
        ("garvayate", "garva", Pada::Atmanepada, None),
        ("mUtrati", "mUtra", Pada::Parasmaipada, Some("2573.3")),
        ("mUtrayati", "mUtra", Pada::Parasmaipada, None),
        ("mUtrayate", "mUtra", Pada::Atmanepada, None),
        ("katrAmi", "katra", Pada::Parasmaipada, Some("2573.3")),
        ("patati", "pata", Pada::Parasmaipada, Some("2573.1")),
        ("patayati", "pata", Pada::Parasmaipada, None),
        ("pAtayati", "pata", Pada::Parasmaipada, None),
        ("patayate", "pata", Pada::Atmanepada, None),
        ("pAtayate", "pata", Pada::Atmanepada, None),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), 1, "{form}");
        let a = &r.analyses[0];
        assert_eq!(a.dhatu, dhatu, "{form}");
        assert_eq!(a.pada, pada, "{form}");
        let ids = ids_of(a);
        match trigger {
            Some(id) => {
                assert_eq!(ids[0], id, "{form}: {ids:?}");
                assert!(has(&ids, "1.3.78"), "{form}: {ids:?}");
                assert!(!has(&ids, "3.1.25"), "{form}: {ids:?}");
            }
            None => {
                assert!(has(&ids, "3.1.25"), "{form}: {ids:?}");
                for id in ["2564", "2570", "2573.1", "2573.3"] {
                    assert!(!has(&ids, id), "{form} {id}: {ids:?}");
                }
                assert_eq!(has(&ids, "2573.2"), form.starts_with("pA"), "{form}: {ids:?}");
            }
        }
    }
    for form in [
        "daMSayati",
        "devayati",
        "garvayati",
        "daMSate",
        "garvate",
        "mUtrate",
        "patate",
        "katraati",
        "pataAmi",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
```

```python
#!/usr/bin/env python3
"""THROWAWAY: insert check_10f.rs right after curadi_analyses_its_adanta_forms."""
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
i = s.index('fn curadi_analyses_its_adanta_forms()')
j = s.index('\n}\n', i) + 3
s = s[:j] + open('/tmp/vidyut-full/slice10f/check_10f.rs').read() + s[j:]
open(p, 'w').write(s)
print("check witnesses inserted")
```

```bash
python3 /tmp/vidyut-full/slice10f/insert_check_10f.py      # check witnesses inserted
mise run fmt
```

The witnesses are the spec's Forms table, one per class × pada, not hand-picked; the Invalid list is the shapes the slice rules out.

- [ ] **Step 5: Run them to see them fail**

Run: `mise exec -- cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .*FAILED" | sort`
Expected failures (no row exists yet):

- `curadi::a_garvad_is_credited_on_exactly_the_a_garviya_cells `
- `curadi::a_kusmad_is_credited_on_exactly_the_akusmiya_cells `
- `curadi_analyses_its_optional_nic_forms `
- `curadi::daMSati_trace_has_no_nic_and_no_a_kusmad `
- `curadi::pata_traces_its_three_readings `
- `curadi::the_anga_shap_junction_resolves_before_the_ending `
- `derivation_set_shape_matches_the_audited_numbers `
- `juhotyadi::nas_capadantasya_is_credited_only_on_rudhadi_dhan_jan_and_curadi_roots `
- `tests::curated_roots_have_expected_ganas_and_padas `
- `tests::dhatu_padas_add_the_nicless_parasmaipada `
- `tests::optional_nic_matches_upadesha_markers `

`the_optional_nic_ids_are_credited_only_on_their_rows`, `no_nic_pada_rule_reaches_a_nicless_branch`, `an_akusmiya_roots_parasmaipada_is_blocked_by_a_kusmad_alone`, `ato_lopa_is_credited_on_exactly_the_adanta_cells` and `pada_ambiguous_surfaces_are_exactly_these` still pass: with no row in the table they hold vacuously, or are unchanged.

- [ ] **Step 6: The rows, the goldens, and the measured pada-ambiguous set**

Create `/tmp/vidyut-full/slice10f/gen_rows_10f.py` (sha256 `209a9dac30c1df528d262c2fdcaf4ac840d1768a712fc34af51aa66db300f207`) and `/tmp/vidyut-full/slice10f/insert_rows_10f.py` (sha256 `a8ea23edef803588aa0b31e261d3ec23a8efb1de631464091274d041fd7ac818`) if they are missing:

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10f — emit the ten optional-ṇic `Dhatu` rows and the
matching `curadi_rows_are_…` list entries. Run from the worktree root;
writes rows.rs and rowlist.rs into the directory given."""
import sys, textwrap

OUT = sys.argv[1]
ROWS = [
    # (number, code, PadaAssignment, trigger, comment body)
    ("10.0193", "danS", "Akusmiya", "2564",
     "√daṃś. Idit, so its ṇic is optional by Kaumudī 2564 (7.1.58's num stored, as for `hins`). "
     "Ātmanepadī by the gaṇasūtra 10.0496 on its ṇic branch (*daṃśayate*); parasmaipadī by "
     "1.3.78 on its ṇic-less branch (*daṃśati*)."),
    ("10.0194", "dans", "Akusmiya", "2564",
     "√daṃs. Idit: ṇic optional by Kaumudī 2564. Ātmanepadī by 10.0496 with ṇic "
     "(*daṃsayate*), parasmaipadī by 1.3.78 without (*daṃsati*)."),
    ("10.0198", "tantr", "Akusmiya", "2564",
     "√tantr. Idit: ṇic optional by Kaumudī 2564. Ātmanepadī by 10.0496 with ṇic "
     "(*tantrayate*), parasmaipadī by 1.3.78 without (*tantrati*)."),
    ("10.0199", "mantr", "Akusmiya", "2564",
     "√mantr. Idit: ṇic optional by Kaumudī 2564. Ātmanepadī by 10.0496 with ṇic "
     "(*mantrayate*), parasmaipadī by 1.3.78 without (*mantrati*)."),
    ("10.0227", "vanc", "Akusmiya", "2570",
     "√vañc. Udit: ṇic optional by Kaumudī 2570. Ātmanepadī by 10.0496 with ṇic "
     "(*vañcayate*), parasmaipadī by 1.3.78 without (*vañcati*). vidyut also runs a "
     "text-free 7.3.63 here, which this engine does not model."),
    ("10.0230", "div", "Akusmiya", "2570",
     "√div. Udit: ṇic optional by Kaumudī 2570. Ātmanepadī by 10.0496 with ṇic "
     "(*devayate*), parasmaipadī by 1.3.78 without (*devati*, 7.3.86's guṇa before śap)."),
    ("10.0400", "pata", "Nic", "2573.1",
     "√pat. Adanta, with ṇic optional by Kaumudī 2573.1 (*patati*). On the ṇic branch "
     "2573.2 optionally deletes the final `a` first, so 7.2.116 lengthens (*pātayati*); "
     "otherwise 6.4.48 deletes it and 7.2.116 declines (*patayati*). Ubhayapadī by 1.3.74 "
     "with ṇic, parasmaipadī by 1.3.78 without."),
    ("10.0449", "garva", "AaGarviya", "2573.3",
     "√garv. Adanta and ā-garvīya, with ṇic optional by Kaumudī 2573.3. Ātmanepadī by "
     "10.0497 with ṇic (*garvayate*), parasmaipadī by 1.3.78 without (*garvati*)."),
    ("10.0451", "mUtra", "Nic", "2573.3",
     "√mūtr. Adanta, with ṇic optional by Kaumudī 2573.3. Ubhayapadī by 1.3.74 with ṇic "
     "(*mūtrayati*), parasmaipadī by 1.3.78 without (*mūtrati*)."),
    ("10.0456", "katra", "Nic", "2573.3",
     "√katr. Adanta, with ṇic optional by Kaumudī 2573.3. Ubhayapadī by 1.3.74 with ṇic "
     "(*katrayati*), parasmaipadī by 1.3.78 without (*katrati*)."),
]
up = {}
for line in open('data/dhatupatha.tsv'):
    f = line.rstrip('\n').split('\t')
    if len(f) >= 3:
        up[f[0]] = (f[1], f[2])
out, lst = [], []
for n, code, pada, trig, body in ROWS:
    u, artha = up[n]
    root, rest = body.split(". ", 1)
    c = f"{n} `{u}` {artha} ({root}). {rest} Slice 10f."
    com = '\n'.join('        // ' + l for l in textwrap.wrap(c, 72))
    out.append(f"    Dhatu {{\n{com}\n        dhatupatha: \"{n}\",\n        code: \"{code}\",\n"
               f"        gana: Gana::Curadi,\n        pada: PadaAssignment::{pada},\n"
               f"        artha: \"{artha}\",\n    }},\n")
    lst.append(f'                ("{n}", "{code}", PadaAssignment::{pada}),\n')
open(f'{OUT}/rows.rs', 'w').write(''.join(out))
open(f'{OUT}/rowlist.rs', 'w').write(''.join(lst))
print(f"{len(ROWS)} rows")
```

```python
#!/usr/bin/env python3
"""THROWAWAY: insert gen_rows_10f.py's output ($GEN/rows.rs, rowlist.rs)."""
import os
GEN = os.environ['GEN']
p = 'crates/panini-data/src/lib.rs'
s = open(p).read()
a = '''        artha: "darSane",
    },
];'''
assert s.count(a) == 1
s = s.replace(a, '''        artha: "darSane",
    },
''' + open(f'{GEN}/rows.rs').read() + '];')
b = '''                ("10.0492", "Deka", PadaAssignment::Nic),
'''
assert s.count(b) == 1
s = s.replace(b, b + open(f'{GEN}/rowlist.rs').read())
open(p, 'w').write(s)
print("inserted 10 rows")
```

```bash
GEN="$(mktemp -d)"
python3 /tmp/vidyut-full/slice10f/gen_rows_10f.py "$GEN"      # 10 rows
sha256sum "$GEN/rows.rs" "$GEN/rowlist.rs"
GEN="$GEN" python3 /tmp/vidyut-full/slice10f/insert_rows_10f.py      # inserted 10 rows
```

Expected hashes: `rows.rs` `f5e0a0193d9489bae49f7dc9d26da51638e5164e32c0b0dd57c2c9c3f7910ce7`, `rowlist.rs` `3fd2fce3a3cf8b4a45a9860fa14d1b732fa73d16bd33055576b3b0205d33e4a0`. If either differs, stop and report. The rows go after `10.0492 Deka`, the last curādi row (the data layer orders curādi by slice).

Repoint the vidyut checkout's dev-deps at this worktree, and create `/tmp/vidyut-full/vidyut-prakriya/examples/curadi_goldens_10f.rs` (sha256 `b76defa330fd08465f070731f9da59dddf61eafd7313effd02f9f0d2ae6fea33`) and `/tmp/vidyut-full/slice10f/insert_goldens_10f.py` (sha256 `3f75d778c62c262b27d4fa724f02ee62006d191806e221153af07eb753676426`) if they are missing:

```rust
//! THROWAWAY: slice 10f — emit the ten optional-ṇic rows' goldens from the
//! engine, asserting every cell's form set equals vidyut's first.
use panini::Panini;
use panini_data::{dhatus, optional_nic, Lakara as L, Pada, Purusha as P, Vacana as V};
use vidyut_prakriya::args::{DhatuPada, Lakara, Prayoga, Purusha, Tinanta, Vacana};
use vidyut_prakriya::{Dhatupatha, Vyakarana};
const VIKALPA_RULES: &[&str] = &[
    "2564", "2570", "2573.1", "2573.3", "2573.2", "7.1.35", "3.4.111", "7.3.86", "6.4.107",
    "8.2.74", "8.2.75", "8.4.65", "8.4.56", "6.4.115", "6.4.117", "6.4.116", "6.4.43",
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
    // The new rows: exactly the `OPTIONAL_NIC` table's.
    for d in dhatus().iter().filter(|d| optional_nic(d.dhatupatha).is_some()) {
        let num = d.dhatupatha;
        let vd = dp.get(num).unwrap();
        for (pada, vpada, pname) in [(Pada::Parasmaipada, DhatuPada::Parasmaipada, "Parasmaipada"), (Pada::Atmanepada, DhatuPada::Atmanepada, "Atmanepada")] {
            if !d.padas().contains(&pada) { continue; }
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
                    // The pinned form is the first LIVE branch: in a cell whose ṇic
                    // branch blocks, that is the ṇic-less one.
                    let first = live[0].text();
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
    std::fs::write("/tmp/vidyut-full/goldens_10f_paradigm.rs", par).unwrap();
    std::fs::write("/tmp/vidyut-full/goldens_10f_alternates.rs", alt).unwrap();
    println!("{ncells} cells, {nforms} forms, {ndiff} differences");
    assert_eq!(ndiff, 0);
}
```

```python
#!/usr/bin/env python3
"""THROWAWAY: insert the generator's goldens before each static's `];`."""
p = 'crates/panini/tests/paradigm/data/curadi.rs'
s = open(p).read()
par = open('/tmp/vidyut-full/goldens_10f_paradigm.rs').read()
alt = open('/tmp/vidyut-full/goldens_10f_alternates.rs').read()
i = s.index('pub const ALTERNATES')
head, tail = s[:i], s[i:]
k = head.rindex('];'); head = head[:k] + par + head[k:]
k = tail.rindex('];'); tail = tail[:k] + alt + tail[k:]
open(p, 'w').write(head + tail)
print("inserted goldens")
```

```bash
WT="$(git rev-parse --show-toplevel)"
V=/tmp/vidyut-full/vidyut-prakriya
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml   # must point at $WT/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example curadi_goldens_10f 2>/dev/null | tail -3)
sha256sum /tmp/vidyut-full/goldens_10f_paradigm.rs /tmp/vidyut-full/goldens_10f_alternates.rs
python3 /tmp/vidyut-full/slice10f/insert_goldens_10f.py      # inserted goldens
mise run fmt
```

Expected:
- the generator prints `720 cells, 984 forms, 0 differences`;
- `goldens_10f_paradigm.rs`: `b365743d9a50079f722efc462c8322a063c74d6bc260d44a6c8b6da6dd458001` (80 rows);
- `goldens_10f_alternates.rs`: `e42a17a8e589b562ac46f7560b445343759ea290eab7f5fdbff6b5a1186cf46a` (264 rows).

Any `DIFF` line or other hash: stop and report, and edit nothing. The pinned form is each cell's first **live** branch: in the ākusmīya and `garva` parasmaipada cells `branches[0]` is the blocked ṇic branch. Leave the dev-deps pointed at this worktree for Task 4.

Then pin the pada-ambiguous set from the measured failure (`pada_ambiguous_surfaces_are_exactly_these` says it is "measured (never hand-picked)"). Create `/tmp/vidyut-full/slice10f/pin_ambiguous_10f.py` if it is missing (sha256 `4c87a0a11c6c06402d1a670636b60ae8a110f04eb916b4013450c834e83374c2`):

```python
#!/usr/bin/env python3
"""THROWAWAY: pin the measured pada-ambiguous set ($SET, the failing test's
`left:` JSON) into pada_ambiguous_surfaces_are_exactly_these."""
import hashlib, json, os
amb = json.load(open(os.environ['SET']))
assert len(amb) == 432, len(amb)
h = hashlib.sha256('\n'.join(amb).encode()).hexdigest()
assert h == 'a6fbbbb8492e047097c55ce8eb59c58e948dbf012c465fecdf669f4c66ad7a8b', h
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
i = s.index("fn pada_ambiguous_surfaces_are_exactly_these")
j = s.index("        both,\n        vec![", i)
k = s.index("        ]", j)
s = s[:j] + "        both,\n        vec![\n" + "".join(f'            "{x}",\n' for x in amb) + s[k:]
open(p, 'w').write(s)
print("pinned", len(amb))
```

```bash
SET="$(mktemp)"
{ mise exec -- cargo test -q -p panini --test paradigm pada_ambiguous 2>&1 || true; } | grep "^  left:" | sed 's/^  left: //' > "$SET"
SET="$SET" python3 /tmp/vidyut-full/slice10f/pin_ambiguous_10f.py      # pinned 432
mise run fmt
```

If the count or hash assertion fails, stop and report.

- [ ] **Step 7: Run the full suite**

```bash
mise run fmt
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS at 13716 cells, with `panini-data` 27, `panini-prakriya` 418, `trace` 208 and `paradigm` 25.

Grep the goldens for the `check()` witnesses, which the test also enforces:

```bash
for f in daMSati daMSayate vaYcati devati devayate garvati garvayate mUtrati mUtrayati mUtrayate katrAmi patati patayati pAtayati patayate pAtayate daMSayati devayati garvayati daMSate garvate mUtrate patate katraati pataAmi; do
  printf "%s: %s\n" $f "$(grep -c "\"$f\"" crates/panini/tests/paradigm/data/*.rs | grep -v ':0' | tr '\n' ' ')"; done
```

Expected: the sixteen Valid witnesses appear only in `curadi.rs`, once each; the nine Invalid shapes print nothing.

- [ ] **Step 8: Commit**

```bash
mise run lint
git add -A
git commit -m "feat(data): curādi's ten optional-ṇic rows — six ākusmīya, garva, mUtra, katra, pata

12996 → 13716 cells, 14660 → 15644 forms, ALTERNATES 1664 → 1928, 242 → 252
roots; pada-ambiguous surfaces 420 → 432; pata's loṭ parasmaipada prathama
and madhyama eka are the new nine-form record. OPTIONAL_NIC is held to the
vendored upadeśa; goldens generated cell-by-cell equal to vidyut."
```

---

## Task 4: Audit, prior-trace diff, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (audit prose), `crates/panini-prakriya/src/tinanta/sanadi.rs` (module doc, 10.0497's comment), `crates/panini-prakriya/src/tinanta/guna.rs` (one comment), `crates/panini-data/src/lib.rs` (one comment), the 10b, 10c and 10e specs

**Interfaces:**
- Consumes: the finished engine, data and goldens. Produces no symbols.

- [ ] **Step 1: Update the audit harness**

Create `/tmp/vidyut-full/slice10f/audit_10f.py` if it is missing (sha256 `4e1a844f11800e62d6d8c87d8ff9183d6862634f729806110744a6d7b555964a`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10f's edits to tools/audit/panini_full_audit.rs. Every
`old` must occur exactly once; nothing is written if one fails."""
import sys
p = 'tools/audit/panini_full_audit.rs'
s = open(p).read()
E = [
("//! What it compares: for each of the 242 curated roots, for each pada the root\n//! admits (two apiece for the 119 roots that admit both padas —\n//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, and ninety-three curādi\n//! roots by 1.3.74), for each of the four",
 "//! What it compares: for each of the 252 curated roots, for each pada the root\n//! admits (`Dhatu::padas`; two apiece for the 129 roots that admit both padas —\n//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, ninety-six curādi\n//! roots by 1.3.74, and seven optional-ṇic ākusmīya or ā-garvīya roots by\n//! 10.0496 / 10.0497 with ṇic and 1.3.78 without), for each of the four"),
("//! Corpus invariants, asserted: 242 roots, 12996 cells, 14660 forms. These are",
 "//! Corpus invariants, asserted: 252 roots, 13716 cells, 15644 forms. These are"),
("//! (`derivation_set_shape_matches_the_audited_numbers`): 1444 root×pada×lakāra\n//! blocks × 9 cells, plus 1664 `ALTERNATES` rows.",
 "//! (`derivation_set_shape_matches_the_audited_numbers`): 1524 root×pada×lakāra\n//! blocks × 9 cells, plus 1928 `ALTERNATES` rows."),
("//! Optionally dump the full 12996-cell table:", "//! Optionally dump the full 13716-cell table:"),
("/// No cell IN the corpus is blocked — every (root, pada) pair the corpus\n/// enumerates comes from `d.pada.padas()`, i.e. a pada the root is sanctioned\n/// in — so this probe steps deliberately OUTSIDE the corpus",
 "/// Every cell IN the corpus has a live branch — every (root, pada) pair the\n/// corpus enumerates comes from `d.padas()`, a pada the root is sanctioned in —\n/// and since slice 10f an optional-ṇic root's cells also hold a blocked branch\n/// (its ṇic branch in parasmaipada, or its ṇic-less one in ātmanepada, where\n/// the two differ in pada). This probe steps deliberately OUTSIDE the corpus"),
("            d.pada\n                .padas()\n                .iter()", "            d.padas()\n                .iter()"),
("        for pada in d.pada.padas() {", "        for pada in d.padas() {"),
('    assert_eq!(roots_seen.len(), 242, "curated roots");\n    assert_eq!(n_cells, 12996, "cells: 1444 root×pada×lakāra blocks × 9");\n    assert_eq!(n_forms, 14660, "forms: 12996 cells + 1664 ALTERNATES rows");',
 '    assert_eq!(roots_seen.len(), 252, "curated roots");\n    assert_eq!(n_cells, 13716, "cells: 1524 root×pada×lakāra blocks × 9");\n    assert_eq!(n_forms, 15644, "forms: 13716 cells + 1928 ALTERNATES rows");'),
]
bad = []
for old, new in E:
    n = s.count(old)
    if n != 1:
        bad.append(f"{n}× {old[:70]!r}")
        continue
    s = s.replace(old, new)
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
open(p, 'w').write(s)
print(f"applied {len(E)} edits")
```

```bash
python3 /tmp/vidyut-full/slice10f/audit_10f.py      # applied 8 edits
```

The harness now enumerates `d.padas()` and asserts 252 / 13716 / 15644.

- [ ] **Step 2: The prior-trace diff and the audit**

The dev-deps still point at this worktree from Task 3. Create `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_10f.rs` if it is missing (sha256 `eb5d5c99c62003bb37fbe900fc66f8ec8b9f25fc013d0c6e687f209fa00735fc`). It lists 10f's rows literally so that it also builds against main:

```rust
//! THROWAWAY: slice 10f — dump every prior cell's live-branch credited-rule log.
use panini::Panini;
use panini_data::{Lakara as L, Purusha as P, Vacana as V};
fn main() {
    let panini = Panini::new();
    for d in panini_data::dhatus() {
        // 10f's rows are exactly the `OPTIONAL_NIC` table's; listed here
        // literally so the dump also builds against main.
        if ["10.0193", "10.0194", "10.0198", "10.0199", "10.0227", "10.0230", "10.0400", "10.0449", "10.0451", "10.0456"].contains(&d.dhatupatha) { continue; }
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
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10f 2>/dev/null > "$DUMP/branch.txt")
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml   # must point at /workspace/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10f 2>/dev/null > "$DUMP/main.txt")
wc -l < "$DUMP/main.txt"; wc -l < "$DUMP/branch.txt"            # 14660 and 14660
cmp "$DUMP/main.txt" "$DUMP/branch.txt" && echo PRIOR-TRACES-IDENTICAL
```

Expected: `14660`, `14660`, `PRIOR-TRACES-IDENTICAL`. This is the corpus-wide check that the aṅga–śap entries and the five Kaumudī rules fire on no prior root. If `cmp` reports a difference, stop and report.

Then the audit. Repoint at this worktree again, copy the committed harness (never rewrite it), and run:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
cp tools/audit/panini_full_audit.rs $V/examples/
(cd $V && PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -8)
(cd $V && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -2)
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml
```

- Expected from the honest run: `blocked branches : 612`, `differing cells  : 0`, `AUDIT PASSED: 13716 cells, 15644 forms, zero differences.`
- Expected from the `entry` control: `AUDIT FAILED: 36 differing cells.`

Do not use `mise -C`. If the honest run shows differences, stop and report, and edit nothing.

- [ ] **Step 3: The doc sweep, the audit record and the spec pointers**

Create `/tmp/vidyut-full/slice10f/docsweep_10f.py` if it is missing (sha256 `29112adcb27e36b0de43e8b6431defcf144b8366f30d0e355d851e96747748a0`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10f's doc sweep, run from the worktree root with the
audit date as argv[1]. Every `old` must occur exactly once in its file; the
script checks all of them before writing any file, and writes nothing if
one fails."""
import sys

README = [
("8.4.44 *śāt* exemption. *curādi* (10) is **open** at 139 of its 509",
 "8.4.44 *śāt* exemption. *curādi* (10) is **open** at 149 of its 509"),
("*naś cāpadāntasya jhali*, until then rudhādi's and juhotyādi's, now reaches a\ncurādi root's own `n` too.\n",
 "*naś cāpadāntasya jhali*, until then rudhādi's and juhotyādi's, now reaches a\n"
 "curādi root's own `n` too. Slice 10f curated the ten rows whose ṇic is\n"
 "optional (Kaumudī 2564, 2570, 2573.1 and 2573.3, the engine's first\n"
 "Kaumudī vikalpas): six ākusmīya roots (√daṃś, *daṃśati* beside\n"
 "*daṃśayate*), `garva`, `mUtra`, `katra` and `pata`. Each derives a ṇic\n"
 "branch and a ṇic-less one. The ṇic-less branch is parasmaipada by 1.3.78\n"
 "and runs the bhvādi path, where an adanta root's own `a` merges with śap's\n"
 "by 6.1.97 or, after 7.3.101, 6.1.101 (*mūtrati*, *patāmi*); `pata` has a\n"
 "third reading, 2573.2's *pātayati*.\n"),
("curated 242-root set, in four lakāras:", "curated 252-root set, in four lakāras:"),
("both correct — and in fact 1180 of the 12996 cells hold more than one form: 790\nhold two, 343 hold three",
 "both correct — and in fact 1352 of the 13716 cells hold more than one form: 904\nhold two, 389 hold three"),
("ubhayapadī adanta roots', each by\n7.1.35/8.4.56, √bhas's laṅ madhyama eka by 8.2.74/8.4.56),\nnineteen hold four",
 "ubhayapadī adanta roots', each by\n7.1.35/8.4.56, √bhas's laṅ madhyama eka by 8.2.74/8.4.56; slice 10f's\noptional-ṇic rows add 114 two-form and 46 three-form cells),\ntwenty-three hold four"),
("vidhiliṅ prathama eka, forking on 6.4.43 alongside 8.4.56), ten hold\nfive",
 "vidhiliṅ prathama eka, forking on 6.4.43 alongside 8.4.56, and — new in\nslice 10f — `mUtra`'s and `katra`'s laṅ and vidhiliṅ parasmaipada prathama\neka, two readings × 8.4.56), ten hold\nfive"),
("and seventeen hold six — the loṭ", "and twenty-three hold six — the loṭ"),
("stack against the same 2³ bound of eight, beside rudhādi's 8.4.65 route and\ntanādi's 7.3.86 route. One cell",
 "stack against the same 2³ bound of eight, beside rudhādi's 8.4.65 route and\n"
 "tanādi's 7.3.86 route; and, new in slice 10f, six more: `pata`'s laṅ and\n"
 "vidhiliṅ parasmaipada prathama eka (three readings × 8.4.56) and `mUtra`'s and\n"
 "`katra`'s loṭ parasmaipada prathama and madhyama eka (two readings × the\n"
 "tātaṅ triple). One cell"),
("barring the rules that would change it. Nothing forks deeper than seven.",
 "barring the rules that would change it. Two cells — new in slice 10f — hold\n"
 "**nine**, the record: `pata`'s loṭ parasmaipada prathama and madhyama eka,\n"
 "three readings × the tātaṅ triple (`patayatu` / `patayatAd` / `patayatAt` /\n"
 "`pAtayatu` / `pAtayatAd` / `pAtayatAt` / `patatu` / `patatAd` / `patatAt`).\n"
 "No cell holds eight."),
("padas — 119 roots that admit both padas in the curated set",
 "padas — 129 roots that admit both padas in the curated set"),
("√laḍ, √bhakṣ, √bhūṣ, √jñap, √yam, √cah, √cap, √rah, √bal and the\neighty-three ubhayapadī adanta roots by 1.3.74) derive a full",
 "√laḍ, √bhakṣ, √bhūṣ, √jñap, √yam, √cah, √cap, √rah, √bal, the\n"
 "eighty-three ubhayapadī adanta roots and slice 10f's `mUtra`, `katra` and\n"
 "`pata` by 1.3.74; and slice 10f's six optional-ṇic ākusmīya roots and\n"
 "`garva`, ātmanepadī by 10.0496 / 10.0497 with ṇic and parasmaipadī by\n"
 "1.3.78 without) derive a full"),
("420 surfaces are pada-ambiguous, each of them a pinned cell in both padas",
 "432 surfaces are pada-ambiguous, each of them a pinned cell in both padas"),
("whose surfaces 10d's √rah and √cah already supply. The enumeration is not",
 "whose surfaces 10d's √rah and √cah already supply, and slice 10f's `mUtra`,\n"
 "`katra` and `pata` the same again on their ṇic branch (`mUtraya-`,\n"
 "`katraya-`, `pataya-`); the six optional-ṇic ākusmīya roots and `garva`\n"
 "derive their two padas on different branches and add none. The\n"
 "enumeration is not"),
("set, all 420. It is therefore", "set, all 432. It is therefore"),
]

ARCH = [
("| `sanadi.rs` | 10.0496, 10.0497, 10.0493, 3.1.25,",
 "| `sanadi.rs` | 2564, 2570, 2573.1, 2573.3, 2573.2, 10.0496, 10.0497, 10.0493, 3.1.25,"),
("— the ākusmīya and ā-garvīya pada and the jñapādi's mit-tva, then ṇic,",
 "— the optional-ṇic fork, the ākusmīya and ā-garvīya pada and the jñapādi's mit-tva, then ṇic,"),
("| `adesha.rs` | 6.1.101 … 6.1.96,",
 "| `adesha.rs` | 6.1.97, 6.1.101 (their aṅga–śap entries), 6.1.101 … 6.1.96,"),
("pins all 141 ids verbatim", "pins all 148 ids verbatim"),
("7.2.116 and 7.3.86 decline on (1.1.57 *acaḥ parasmin pūrvavidhau*) — 141\ntotal).",
 "7.2.116 and 7.3.86 decline on (1.1.57 *acaḥ parasmin pūrvavidhau*) — 141\n"
 "total — then curādi 10f's seven: the Kaumudī vikalpas 2564, 2570, 2573.1\n"
 "and 2573.3, which fork a root whose ṇic is optional into its ṇic and\n"
 "ṇic-less branches, and 2573.2, `pata`'s optional `a`-lopa before ṇic, all\n"
 "five ahead of 10.0496, and second entries for 6.1.97 *ato guṇe* and 6.1.101\n"
 "*akaḥ savarṇe dīrghaḥ* at the head of `adesha.rs`, where an adanta root's\n"
 "ṇic-less `a` meets śap — 148 total)."),
("curādi (10), **open** at 139 of its\n509 rows", "curādi (10), **open** at 149 of its\n509 rows"),
("ninety-two adanta roots, slice 10e). gaṇa",
 "ninety-two adanta roots, slice 10e; the ten optional-ṇic rows, slice 10f). gaṇa"),
("forking 342 cells (loṭ\nprathama and madhyama eka across the 171 roots with a parasmaipada column —",
 "forking 362 cells (loṭ\nprathama and madhyama eka across the 181 roots with a parasmaipada column —"),
("roots never reach this guard, and the 119 roots that admit both",
 "roots never reach this guard, and the 129 roots that admit both"),
("√laḍ, √bhakṣ, √bhūṣ, √jñap, √yam, √cah, √cap, √rah, √bal and slice 10e's\neighty-three ubhayapadī adanta roots by 1.3.74) reach it in their",
 "√laḍ, √bhakṣ, √bhūṣ, √jñap, √yam, √cah, √cap, √rah, √bal, slice 10e's\n"
 "eighty-three ubhayapadī adanta roots and slice 10f's `mUtra`, `katra` and\n"
 "`pata` by 1.3.74, and slice 10f's six optional-ṇic ākusmīya roots and\n"
 "`garva`, by 1.3.78 on their ṇic-less branch) reach it in their"),
("171 + 71 = the 242 curated roots)", "181 + 71 = the 252 curated roots)"),
("forking 348 cells outright: laṅ and vidhiliṅ prathama eka across\nthose same 171 parasmaipada columns (325 of them",
 "forking 354 cells outright: laṅ and vidhiliṅ prathama eka across\nthose same 181 parasmaipada columns (331 of them"),
("10d's six jñapādi roots and 10e's eighty-three adanta roots contribute both cells),",
 "10d's six jñapādi roots and 10e's eighty-three adanta roots contribute both cells, and so do 10f's `mUtra`, `katra` and `pata` on their ṇic branch; 10f's ṇic-less forks key on their Kaumudī id as well — `2564+8.4.56` and its siblings — and sit outside this count),"),
("forking a further 342 (the same", "forking a further 362 (the same"),
]

AGENTS = [
("(`crates/panini/tests/paradigm/`, 12996 cells, ten gaṇas, nine complete —",
 "(`crates/panini/tests/paradigm/`, 13716 cells, ten gaṇas, nine complete —"),
("at 139 after slice 10e curated ninety-two adanta roots —",
 "at 139 after slice 10e curated ninety-two adanta roots, at 149 after slice 10f curated the ten optional-ṇic rows —"),
("other forms — a second (790 cells), a third (343 cells), a fourth\n    (nineteen",
 "other forms — a second (904 cells), a third (389 cells), a fourth\n    (twenty-three"),
("    eka, and — new in slice 3f3 — √jan's) and",
 "    eka, and — new in slice 3f3 — √jan's, and — new in slice 10f — `mUtra`'s\n    and `katra`'s laṅ and vidhiliṅ prathama eka) and"),
("    madhyama eka took it to seventeen — a fourth",
 "    madhyama eka took it to seventeen, and slice 10f's `pata` (laṅ and\n    vidhiliṅ prathama eka) and `mUtra` and `katra` (loṭ prathama and madhyama\n    eka) to twenty-three — a fourth"),
("    seventh for slice 3c2's √hā (`03.0009`) loṭ madhyama eka, the one\n    seven-form cell — in",
 "    seventh for slice 3c2's √hā (`03.0009`) loṭ madhyama eka, the one\n    seven-form cell, or up to a ninth for slice 10f's `pata` loṭ\n    parasmaipada prathama and madhyama eka, the two nine-form cells and the\n    record — in"),
("    `ALTERNATES` (1664 rows in all, so 12996 + 1664 = 14660 forms total); √bhuj",
 "    `ALTERNATES` (1928 rows in all, so 13716 + 1928 = 15644 forms total); √bhuj"),
("  entry, 12996 cells / 14660 forms / 242 roots).",
 "  entry, 12996 cells / 14660 forms / 242 roots), and that by curādi 10f's\n  (`tools/audit/README.md`'s @DATE@ 10f entry, 13716 cells / 15644 forms /\n  252 roots)."),
("only in the ordinary corpus-size sense, not wrong in kind: 12996 goldens",
 "only in the ordinary corpus-size sense, not wrong in kind: 13716 goldens"),
]

TOOLS_README = [
("**It asserts the corpus totals** (242 roots, 12996 cells, 14660 forms) rather than",
 "**It asserts the corpus totals** (252 roots, 13716 cells, 15644 forms) rather than"),
("## Last recorded result\n\n",
 "## Last recorded result\n\n"
 "@DATE@, curādi 10f slice, vidyut\n"
 "`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 13716\n"
 "cells / 15644 forms / 252 roots**, with the `entry` negative control verified\n"
 "failing (36 √bhū cells).\n\n"
 "The verdict covers the whole curādi 10f slice: the ten rows whose ṇic is\n"
 "optional (Kaumudī 2564, 2570, 2573.1, 2573.3), each derived on its ṇic and\n"
 "its ṇic-less branch. The harness now enumerates `Dhatu::padas`, which adds\n"
 "the ṇic-less parasmaipada to an ākusmīya or ā-garvīya row's ātmanepada, so\n"
 "for the first time a corpus cell holds blocked branches beside its live\n"
 "ones: 612 of them (each optional-ṇic row's ṇic or ṇic-less branch in the\n"
 "pada it does not admit). Before 6.1.97's and 6.1.101's aṅga–śap entries the\n"
 "four adanta rows' ṇic-less parasmaipada were the slice's only differences,\n"
 "144 cells (*katraati* for *katrati*). A main-vs-branch dump of every prior\n"
 "cell's traces was byte-identical, all 14660 live branches.\n\n"
 "Totals: 252 = 242 + 10; 13716 = 12996 + 720 (80 root×pada×lakāra blocks ×\n"
 "9); 15644 = 14660 + 720 + 264 new `ALTERNATES` rows (1664 → 1928), measured\n"
 "via the harness's corpus block, not assumed.\n\n"),
]

MAIN_RS = [
("/// cells), and curādi 10e's re-ran it at the same commit over all 12996\n/// cells / 14660 forms / 242 roots with zero differences, its `entry`\n/// negative control verified failing (36 √bhū cells). √tṛh joins none of the fork",
 "/// cells), and curādi 10e's re-ran it at the same commit over all 12996\n"
 "/// cells / 14660 forms / 242 roots with zero differences, its `entry`\n"
 "/// negative control verified failing (36 √bhū cells), and curādi 10f's\n"
 "/// re-ran it at the same commit over all 13716 cells / 15644 forms / 252\n"
 "/// roots with zero differences, its `entry` negative control verified\n"
 "/// failing (36 √bhū cells). √tṛh joins none of the fork"),
]

GUNA = [
("    // 242-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)",
 "    // 252-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)"),
]

DATA = [
("    /// vendored upadeśa: 66 of the 242 curated roots carry a `\\` at all, and 45",
 "    /// vendored upadeśa: 66 of the 252 curated roots carry a `\\` at all, and 45"),
]

SANADI = [
("//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, ṇic's\n//! it-lopa (1.3.9), 3.4.114, 6.4.48, 7.2.116, 6.4.92, 7.3.86, 3.1.32 —\n//! opened by three dhātupāṭha gaṇasūtras: 10.0496 and 10.0497, which settle\n//! an ākusmīya or ā-garvīya root's pada, and 10.0493, which credits a\n//! jñapādi root's mit-tva, all before ṇic is added.",
 "//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, ṇic's\n"
 "//! it-lopa (1.3.9), 3.4.114, 6.4.48, 7.2.116, 6.4.92, 7.3.86, 3.1.32 —\n"
 "//! opened by four Kaumudī vikalpas (2564, 2570, 2573.1, 2573.3) that fork a\n"
 "//! root whose ṇic is optional into its ṇic and ṇic-less branches, a fifth\n"
 "//! (2573.2) that forks `pata`'s ṇic branch on its final `a`, and three\n"
 "//! dhātupāṭha gaṇasūtras: 10.0496 and 10.0497, which settle an ākusmīya or\n"
 "//! ā-garvīya root's pada, and 10.0493, which credits a jñapādi root's\n"
 "//! mit-tva, all before ṇic is added."),
("//! Every rule self-guards: 10.0496 on `Tag::Akusmiya`, 10.0497 on",
 "//! Every rule self-guards: the four optional-ṇic vikalpas on the row's\n"
 "//! `OPTIONAL_NIC` entry, 2573.2 on `pata`'s, 10.0496 on `Tag::Akusmiya`, 10.0497 on"),
("    // 3.1.25. The range includes `10.0449 garva`, whose ṇic is optional and\n    // which is not curated yet: the gaṇasūtra applies only on its ṇic branch.",
 "    // 3.1.25. The range includes `10.0449 garva`, whose ṇic is optional\n"
 "    // (Kaumudī 2573.3): the gaṇasūtra applies only on its ṇic branch, and on\n"
 "    // the ṇic-less one 2573.3 has removed `Tag::AaGarviya`, so this declines."),
]

SPEC_10B = [
("roots (with the ā-garvīya list, 10.0497), optional ṇic (which takes the\noptional-ṇic ākusmīya rows, since 10.0496 applies only on the ṇic branch).",
 "roots (with the ā-garvīya list, 10.0497), optional ṇic (which takes the\noptional-ṇic ākusmīya rows, since 10.0496 applies only on the ṇic branch;\ntaken by slice 10f, see `2026-10-02-curadi-gana-10f-design.md`)."),
]
SPEC_10C = [
("ā-garvīya list, 10.0497), then optional ṇic (taking the six optional-ṇic\nākusmīya rows).",
 "ā-garvīya list, 10.0497), then optional ṇic (taking the six optional-ṇic\nākusmīya rows; taken by slice 10f, see `2026-10-02-curadi-gana-10f-design.md`)."),
]
SPEC_10E = [
("- Optional ṇic, next. It takes the six optional-ṇic ākusmīya rows and the",
 "- Optional ṇic, next (taken by slice 10f, see\n  `2026-10-02-curadi-gana-10f-design.md`). It takes the six optional-ṇic ākusmīya rows and the"),
]

FILES = {
    "README.md": README,
    "docs/ARCHITECTURE.md": ARCH,
    "AGENTS.md": AGENTS,
    "tools/audit/README.md": TOOLS_README,
    "crates/panini/tests/paradigm/main.rs": MAIN_RS,
    "crates/panini-prakriya/src/tinanta/guna.rs": GUNA,
    "crates/panini-data/src/lib.rs": DATA,
    "crates/panini-prakriya/src/tinanta/sanadi.rs": SANADI,
    "docs/superpowers/specs/2026-10-02-curadi-gana-10b-design.md": SPEC_10B,
    "docs/superpowers/specs/2026-10-02-curadi-gana-10c-design.md": SPEC_10C,
    "docs/superpowers/specs/2026-10-02-curadi-gana-10e-design.md": SPEC_10E,
}

def main():
    date = sys.argv[1]
    out, bad = {}, []
    for path, edits in FILES.items():
        s = open(path).read()
        for old, new in edits:
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
python3 /tmp/vidyut-full/slice10f/docsweep_10f.py "$(date -u +%F)"      # applied 48 edits to 11 files
```

If it prints `not applied:`, nothing was written. Edit the paragraph named to the same facts, rather than skipping it, and re-run.

Notes on the numbers, re-derived from the census rather than assumed:
- 1352 = 13716 − 12364; 904 / 389 / 23 / 23 are the census buckets, and no cell holds eight.
- 129 = 119 + 10 roots that admit both padas: `mUtra`, `katra` and `pata` by 1.3.74, the six ākusmīya roots and `garva` by their gaṇasūtra with ṇic and 1.3.78 without. 181 + 71 = 252: the ten new rows all have a parasmaipada column, and the ātmanepada-only count is unchanged.
- 362 = 181 × 2 (7.1.35's loṭ prathama and madhyama eka). 354 = 331 + 22 + 1 = `key_count("8.4.56")`: the ṇic-less forks key on their Kaumudī id as well and sit outside it, as √cur's 7.3.86 ones do.
- 148 = 141 + 7 rule entries: five ids and two second entries.

- [ ] **Step 4: The AGENTS.md stale-comment ledger**

Create `/tmp/vidyut-full/slice10f/ledger_10f.py` if it is missing (sha256 `558af50f9122ffae28c75db0ac4145eca1443e844931c35b43e8e73d82253b9e`). It measures both anchors by grep at this commit; it never computes them:

```python
#!/usr/bin/env python3
"""THROWAWAY: add 10f's sentence to AGENTS.md's stale-comment ledger, with
both anchors measured by grep now."""
import re, subprocess
g = subprocess.run(['grep', '-n', '1872 goldens move', 'crates/panini-prakriya/src/tinanta/guna.rs'], capture_output=True, text=True).stdout.split(':')[0]
c = subprocess.run(['grep', '-n', 'only 8 cells fire', 'crates/panini-prakriya/src/controller.rs'], capture_output=True, text=True).stdout.split(':')[0]
assert g and c
p = 'AGENTS.md'
s = open(p).read()
m = re.search(r"Curādi 10e touched neither comment either; the corpus stands at 12996 cells as of 10e \(`guna\.rs:2565`'s claim anchored at `guna\.rs:\d+`, `controller\.rs:206`'s at `controller\.rs:\d+`; both lines measured by grep at this commit\)\.", s)
assert m
new = m.group(0) + f" Curādi 10f touched neither comment either; the corpus stands at 13716 cells as of 10f (`guna.rs:2565`'s claim anchored at `guna.rs:{g}`, `controller.rs:206`'s at `controller.rs:{c}`; both lines measured by grep at this commit)."
s = s[:m.start()] + new + s[m.end():]
open(p, 'w').write(s)
print(f"ledger: guna.rs:{g} controller.rs:{c}")
```

```bash
python3 /tmp/vidyut-full/slice10f/ledger_10f.py      # ledger: guna.rs:2565 controller.rs:206
```

AGENTS.md's floor paragraph (`measured at 12996 cells`) and the current mutation record belong to Task 5.

- [ ] **Step 5: Sweep for anything left stale**

```bash
grep -rn -i -E "\b12996\b|\b14660\b|\b1664\b|\b1444\b|242 roots|242-root|of these 242|of the 242|242 curated|139 of|\b119 roots|ninety-three curādi|\b37 ākusmīya|nine ā-garvīya rows'|\b420\b|\b11816\b|\b790\b|\b343\b|171 \+ 71|forking 342|\b348 cells|141 total\)|141 ids|not curated yet|no optional-ṇic roots|Nothing forks deeper|the one seven-form|engine's record|merge_anga_a" README.md AGENTS.md docs/ARCHITECTURE.md tools/audit/README.md crates --include=*.md --include=*.rs | grep -v "paradigm/data/"
grep -rn -i -E "(pata|mUtra|katra|garva|daSi|dasi~|tatri|matri|vancu|divu).{0,80}(not (yet )?curated|optional-ṇic slice|go to the|deferred|is out)|(not (yet )?curated|optional-ṇic slice|deferred).{0,80}(pata|mUtra|katra|garva)" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.md --include=*.rs | grep -v "paradigm/data/"
```

Expected residue, all of it dated history that stays true:
- `tools/audit/README.md`'s 10f entry's own `Totals:` line and the 10e entry;
- `paradigm/main.rs`'s audit chain (the 10e clause), its 10e paragraph ("OPEN at 139"), the pada-ambiguous comment's "ninety-six to 420" / "420 to 432", and the nine-form record's "the engine's record" (which is 10f's own claim);
- AGENTS.md:37's floor paragraph (Task 5), its audit chain (the 10e clause) and its stale-comment ledger.

The second grep, for any comment that still calls one of the ten rows deferred or uncurated, must print nothing. Any other hit from either grep is a miss. Edit it.

- [ ] **Step 6: Run the full suite and commit**

Run: `mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"` (foreground, timeout 600000 ms). Expected: PASS at 13716 cells.

```bash
mise run fmt && mise run lint
git add -A
git commit -m "docs: 10f's counts, the audit record, and the sweep

13716 cells / 15644 forms / 252 roots across README, ARCHITECTURE, AGENTS,
paradigm/main.rs and tools/audit; curādi open at 149/509; 148 rule entries;
129 both-pada roots; 432 pada-ambiguous surfaces; the nine-form record. Audit
at zero divergence against 8da2f90b with 612 blocked branches; prior traces
byte-identical to main."
```

---

## Task 5: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and the current-record paragraph); `mise.toml` if the cap moves

Follow AGENTS.md's cargo-mutants protocol. Hazards from this repo's record:
- **Measure, never scale.**
- **Every invocation rotates `mutants.out`**, so always pass `-o`.
- **The mise shim fails.** Use the real binary: `CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`.
- **`pgrep -f` matches its own shell.** Wait on `pgrep -x cargo-mutants`.
- **Background shells die at about 60 minutes.** Launch detached with `setsid nohup`, as below.

The prototype measured these, which are the values to expect:
- **The mutant list** goes 820 → **830** in panini-prakriya, while panini-analyze stays at 12. The ten new:
  - `adesha.rs:78:50: replace || with &&`
  - `adesha.rs:78:16: delete !`
  - `adesha.rs:78:53: delete !`
  - `adesha.rs:104:50: replace || with &&`
  - `adesha.rs:104:16: delete !`
  - `adesha.rs:104:53: delete !`
  - `sanadi.rs:39:5: replace skip_nic -> bool with true`
  - `sanadi.rs:39:5: replace skip_nic -> bool with false`
  - `sanadi.rs:39:39: replace != with == in skip_nic`
  - `sanadi.rs:111:47: replace != with ==`

  All were caught in the prototype's `--in-diff` run.
- **The three documented non-caught entries** are at **`adesha.rs:647:30`** (from `589:30`: the two head entries added 58 lines above it), **`tripadi.rs:1303:38`** and **`tripadi.rs:1616:23`**. The two `tripadi.rs` positions are **already** one line below AGENTS.md's recorded `1302:38` / `1615:23` on main: 10e's final-review commit `fab7acc` edited a comment above them after its campaign. 10f does not touch `tripadi.rs`. Confirm all three by `--list`; never compute them.
- **The data-crate mutants** (`optional_nic`, `Dhatu::padas`) are outside the campaign's packages, as `panini-data` always has been. Step 4b covers them with `--in-diff`.

- [ ] **Step 1: Measure the floor**

With nothing else running, run this twice: `time mise run test 2>&1 | tail -3` (foreground). Record both wall clocks and `cat /proc/loadavg`. Read 10e's floor from AGENTS.md's floor paragraph and keep the comparison chain.

- [ ] **Step 2: Locate and probe the two uncaught equivalents at `-j 4`**

```bash
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /="
```

Expect `647` (adesha), `1303` (tripadi, inside 8.3.13's `apply`) and `1616` (the ṇatva hang). Write them as `<A>`, `<T1>` and `<T2>`. Then run in the foreground with timeout 600000 ms:

```bash
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 600 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:<A>:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" 2>&1 | tail -10
```

Both must be MISSED, not TIMEOUT. Read each test-phase duration from `$SCRATCH/mutants.out/outcomes.json`. Set a provisional cap: max(430, 6 × the longer of the two, rounded up to the next 10 s).

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/curadi-10f"   # durable: outside the repo and any scratchpad
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
- **842 mutants: 791 caught, 48 unviable, 2 missed, 1 timeout.**
  - panini-prakriya: 830 / 783 / 44 / 2 / 1.
  - panini-analyze: 12 / 8 / 4 / 0 / 0.
- `missed.txt` holds exactly `adesha.rs:<A>:30: replace + with *` and `tripadi.rs:<T1>:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:<T2>:23: replace -= with /=`.

If not:
- Any **other timeout** is a suspect survivor that the larger suite pushed past the cap. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant in `sanadi.rs` or the two `adesha.rs` head entries is a gap in Task 2's tests; add the test that kills it. Any other missed mutant means a test that caught it at 12996 cells no longer does; stop and report.

**Step 4b: the data-crate mutants.** Run the slice's diff for `panini-data` alone:

```bash
git diff 6864f4f -- crates/panini-data/src/lib.rs > "$OUT/data.diff"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-data --test-workspace=true \
  --in-diff "$OUT/data.diff" --timeout <CAP> -j 4 -o "$OUT/data" 2>&1 | tail -3
```

Expected: 6 mutants (four on `optional_nic`, two on `Dhatu::padas`), 5 caught, 1 unviable (`Vec::leak(vec![Default::default()])`), 0 missed.

- [ ] **Step 5: Margins**

```bash
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["outcomes"][1])' "$OUT/mutants.out/outcomes.json"
```

Compute two things:
- the two equivalents' test phases under campaign load;
- the caught phases' min / median / p90 / max.

The cap is max(430, 6 × the longest campaign-load equivalent phase, rounded up to the next 10 s).
- If that is 430, the cap stays.
- Otherwise change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together.

- [ ] **Step 6: Record it in AGENTS.md**

- Rewrite the paragraph that opens `**The floor behind the 430s cap, measured at 12996 cells on Rust 1.99.0,`. Use Step 1's and Step 2's numbers at 13716 cells, the load averages, and the cap Step 5 chose. Keep the comparison chain to earlier floors, with 10e's joining it.
- Replace the `**Current record (curādi 10e, …).**` paragraph with `**Current record (curādi 10f, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**;
  - the non-caught set diffed against 10e's on the full record (package, span, replacement, function, genre, outcome). The clean result is that the 51 entries are identical except `adesha.rs:589:30` → `647:30` (58 lines added above it) and the two `tripadi.rs` spans, which sit one line below 10e's recorded positions because of `fab7acc`, not 10f — say so, since the 10e record's positions are stale on main;
  - the ten new-code mutants, each named with its outcome, and Step 4b's six data-crate mutants;
  - the campaign-load phases and margins;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`.

  End it with a pointer to the record it replaces. Run `git rev-parse --short HEAD` before committing, and write ``The curādi 10e record it replaces: `git show <that hash>:AGENTS.md`.``

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md mise.toml
git commit -m "chore: 10f mutation gate — floor and uncaught run re-measured at 13716 cells

All ten new-code mutants (skip_nic, 2573.2, the two aṅga–śap entries) and
the data crate's optional_nic / Dhatu::padas mutants caught; missed.txt holds
only the two documented equivalents and timeout.txt only the permanent
ṇatva-scan entry."
```

---

## Task 6: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -20
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # back at /workspace/crates
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin curadi-10f
gh pr create --title "curādi 10f — optional ṇic" --body "$(cat <<'BODY'
Slice 10f gives curādi an optional ṇic and curates the ten rows earlier
slices set aside for it: six ākusmīya rows (Kaumudī 2564 idit, 2570 udit),
`pata` (2573.1, with 2573.2's *pātayati*), and `garva`, `mUtra`, `katra`
(2573.3).

- Four Kaumudī vikalpas lead the sanādi stage, ahead of 10.0496, keyed on
  a row-numbered `OPTIONAL_NIC` table. The ṇic-less branch drops the
  ṇic-branch pada tag and bars 3.1.25, so 1.3.78 finds a śeṣa: *daṃśati*
  beside *daṃśayate*, *mūtrati* beside *mūtrayati*.
- 6.1.97 and 6.1.101 gain an aṅga–śap entry each, for the adanta root's own
  `a` meeting śap without ṇic (*katrati*, *patāmi*), where vidyut resolves it.
- Rule names are the Siddhānta-Kaumudī's own text.
- The pinned form of a cell is its first live branch; `roundtrip` admits
  blocked branches on optional-ṇic rows only.

The golden suite goes from 12996 to 13716 cells; curādi is open at 149 of
509; `pata`'s loṭ parasmaipada prathama and madhyama eka are the new
nine-form record. The audit shows zero divergence against `8da2f90b`, a
main-vs-branch dump of every prior cell's traces is byte-identical, and the
mutation gate catches every new mutant.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`:
   - run `git worktree remove .worktrees/curadi-10f`;
   - run `git worktree remove --force .worktrees/curadi-10f-proto` and `git worktree remove --force .worktrees/curadi-10f-replay` (the throwaways);
   - delete the local and remote `curadi-10f` branch;
   - run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| `OPTIONAL_NIC` keyed by number; `optional_nic()` | 2 |
| `Dhatu::padas()`; every enumerator switched | 2 |
| 2564, 2570, 2573.1, 2573.3 at the head of `SANADI`, `bars: ["3.1.25", "2573.2"]`, dropping the ṇic-branch pada tag | 2 |
| 2573.2 on `pata`'s ṇic branch | 2 |
| SK rule names (amendment) | 2 |
| 6.1.97 / 6.1.101 aṅga–śap entries, separate bodies (amendment) | 2 |
| First-live-branch pin; `roundtrip` (amendment) | 2 |
| Order / vikalpa / bars pins; unit tests with negative controls | 2 |
| 7.1.58 and 7.3.63 not modelled | Global Constraints |
| Ten rows, codes, comments | 3 |
| 80 goldens + 264 alternates, engine = vidyut per cell; totals 252 / 13716 / 15644; buckets (amended census); nine-form arm; keys | 3 |
| Pada-ambiguous 420 → 432 | 3 |
| `optional_nic_matches_upadesha_markers` (list, not shape; `Cidra`) | 3 |
| Pada agreement 96 / 43 / 10 and the ṇic-less arm; `Dhatu::padas` pin | 3 |
| Trace pins (*daṃśati*, `pata`'s three readings); ids only on their rows; no ṇic pada rule on a ṇic-less branch; junction order | 3 |
| `check()`: Forms-table witnesses and the ruled-out shapes | 3 |
| Prior traces byte-identical | 4 Step 2 |
| Audit with repoint and negative control; record | 4 |
| README / ARCHITECTURE (148, 362 / 354 / 331, 181 + 71, 129) / AGENTS; `sanadi.rs`, `panini-data` docs; 10b, 10c, 10e pointers | 3 Step 3, 4 |
| Floor, uncaught probe, campaign, verbatim non-caught record, new-code mutants named | 5 |

**Type consistency.**
- `optional_nic` takes `&str` and returns `Option<&'static str>`; `OPTIONAL_NIC` is `&[(&str, &str)]`. Call sites pass `p.ctx.dhatupatha` (`&'static str`), `d.dhatupatha`, and `number: &str` in the tests; all compiled in the prototype.
- `Dhatu::padas` returns `&'static [Pada]`, the same type as `PadaAssignment::padas`, so `for &pada in d.padas()` and `.len()` are drop-in replacements.
- `skip_nic(p: &mut Prakriya, id: &'static str, name: &'static str) -> bool` is called from four `apply` closures with literal ids.
- The golden tuple shapes match `ParadigmRow` and `AlternateRow`.

**Known soft spots.**
- **Doc strings in Task 4** were read at the prototype's state. If the script reports an `old` not found exactly once, edit that paragraph to the same facts rather than skip it.
- **Task 3's intermediate state** (Step 5, assertions in, rows not) was replayed; the failing list above is that replay's.
- **The floor and cap in Task 5** depend on host load. Record the load beside every timing.

## Appendix: `engine_10f.py`

The Task 2 edits as one script, for checking a hand-applied result. It reads the blocks above from `/tmp/vidyut-full/slice10f/seg/` (`sanadi_tests.rs`, `adesha_junction.rs`, `adesha_tests.rs`, `lib_table.rs`, `lib_impl.rs`, `sanadi_head.rs`, `adesha_head.rs`). sha256 `aafd13c22f1b1d63d22055e2762ec3dfc367047854f1692e63299a4c3a1edf85`.

```python
#!/usr/bin/env python3
"""THROWAWAY: replay slice 10f's Task 2 (engine + harness) edits on a base
checkout, from the verbatim segments in seg/. Mirrors the plan's Task 2
steps one for one. Every `old` must occur exactly once; nothing is written
if one fails. Run from the worktree root, then `mise run fmt`."""
import sys
SEG = '/tmp/vidyut-full/slice10f/seg'
seg = lambda n: open(f'{SEG}/{n}.rs').read()
APPEND = object()  # (path, APPEND, text): insert before the file's final "}\n"
E = [
('crates/panini-data/src/lib.rs',
 'pub const AA_GARVIYA: RangeInclusive<&str> = "10.0440"..="10.0449";\n',
 'pub const AA_GARVIYA: RangeInclusive<&str> = "10.0440"..="10.0449";\n\n' + seg('lib_table')),
('crates/panini-data/src/lib.rs',
 "    pub pada: PadaAssignment,\n    pub artha: &'static str,\n}\n",
 "    pub pada: PadaAssignment,\n    pub artha: &'static str,\n}\n\n" + seg('lib_impl')),
('crates/panini-prakriya/src/tinanta/sanadi.rs',
 'use crate::rule::{Rule, RuleKind};\nuse crate::term::{Tag, Term};\n',
 'use crate::prakriya::Prakriya;\nuse crate::rule::{Rule, RuleKind};\nuse crate::term::{Tag, Term};\n'),
('crates/panini-prakriya/src/tinanta/sanadi.rs',
 'use panini_data::Pada;\n\npub(crate) static SANADI: &[Rule] = &[\n',
 'use panini_data::{Pada, optional_nic};\n\n' + seg('sanadi_head')),
('crates/panini-prakriya/src/tinanta/sanadi.rs', APPEND, seg('sanadi_tests')),
('crates/panini-prakriya/src/tinanta/adesha.rs',
 'pub(crate) static ADESHA: &[Rule] = &[\n', seg('adesha_head')),
('crates/panini-prakriya/src/tinanta/adesha.rs',
 '    use panini_data::{Pada, Purusha, Vacana, dhatus};\n\n',
 '    use panini_data::{Pada, Purusha, Vacana, dhatus};\n\n' + seg('adesha_junction') + '\n'),
('crates/panini-prakriya/src/tinanta/adesha.rs', APPEND, seg('adesha_tests')),
]
# adesha: the existing tests' 6.1.101 lookups move to junction().
REPL = [
('crates/panini-prakriya/src/tinanta/adesha.rs', 'rules().find(|r| r.id == "6.1.101").unwrap()', 'junction("6.1.101")', 8),
('crates/panini-prakriya/src/tinanta/adesha.rs',
 '        for (id, fires) in [("6.1.101", false), ("6.1.90", true), ("6.1.88", true)] {\n            let r = rules().find(|r| r.id == id).unwrap();',
 '        for (id, fires) in [("6.1.101", false), ("6.1.90", true), ("6.1.88", true)] {\n            let r = junction(id);', 1),
('crates/panini-analyze/src/lib.rs', 'for &pada in d.pada.padas() {', 'for &pada in d.padas() {', 1),
('crates/panini-analyze/src/lib.rs', 'd.pada.padas().len()', 'd.padas().len()', 1),
('crates/panini/tests/paradigm/main.rs', 'for &pada in d.pada.padas() {', 'for &pada in d.padas() {', 1),
('crates/panini/tests/paradigm/main.rs', 'd.pada.padas().len()', 'd.padas().len()', 1),
('crates/panini/tests/trace/helpers.rs', 'for &pada in d.pada.padas() {', 'for &pada in d.padas() {', 1),
('crates/panini/tests/trace/juhotyadi.rs', 'for &pada in d.pada.padas() {', 'for &pada in d.padas() {', 1),
]
DT = 'crates/panini-prakriya/src/tinanta/derivation_tests.rs'
E += [
(DT, '''/// `Tag::AtLopa` (1.1.57). 10.0496, 10.0497 and 10.0493 are the only ids
/// here that are not Aṣṭādhyāyī sūtras.
''', '''/// `Tag::AtLopa` (1.1.57).
///
/// Slice 10f opens the list with five Kaumudī vikalpa rules, ahead of
/// 10.0496, because vidyut-prakriya decides ṇic before the pada gaṇasūtras:
/// 2564, 2570, 2573.1 and 2573.3 fork a root whose ṇic is optional into its
/// ṇic and ṇic-less branches, and 2573.2 then forks `pata`'s ṇic branch on
/// its final `a`. It also adds a second 6.1.97 and a second 6.1.101 at the
/// head of the adesha stage, for an aṅga-final `a` meeting śap, which only
/// the ṇic-less adanta branch reaches; see their comments in
/// `tinanta/adesha.rs`. 10.0496, 10.0497, 10.0493 and the five Kaumudī ids
/// are the only ids here that are not Aṣṭādhyāyī sūtras.
'''),
(DT, '''        "10.0496", "10.0497", "10.0493", "3.1.25",''',
 '''        "2564", "2570", "2573.1", "2573.3", "2573.2", "10.0496", "10.0497", "10.0493", "3.1.25",'''),
(DT, '''"6.4.42", "6.4.43", "6.1.101",
        "6.1.96",''', '''"6.4.42", "6.4.43", "6.1.97",
        "6.1.101", "6.1.101", "6.1.96",'''),
(DT, '''    let expected = [
        "7.1.35", "3.4.111", "7.3.86",''', '''    let expected = [
        "2564", "2570", "2573.1", "2573.3", "2573.2", "7.1.35", "3.4.111", "7.3.86",'''),
(DT, '''    let expected: Vec<(&str, &[&str])> = vec![
        ("7.3.87",''', '''    let expected: Vec<(&str, &[&str])> = vec![
        ("2564", &["3.1.25", "2573.2"][..]),
        ("2570", &["3.1.25", "2573.2"][..]),
        ("2573.1", &["3.1.25", "2573.2"][..]),
        ("2573.3", &["3.1.25", "2573.2"][..]),
        ("7.3.87",'''),
('crates/panini/tests/paradigm/main.rs', '''            let branches = derive(d, lak, *row_pada, pu, va);
            assert_eq!(
                branches[0].text(),
                *expected,
                "index 0 must be the declined derivation for {root} {lakara} cell {cell}"
            );
''', '''            let branches = derive(d, lak, *row_pada, pu, va);
            // The pinned form is the first LIVE branch, the declined
            // derivation among those that survive. Before slice 10f that was
            // always index 0. An optional-ṇic root's ṇic branch is index 0
            // and, in an ākusmīya or ā-garvīya root's parasmaipada,
            // blocked by 10.0496 / 10.0497: the ṇic-less branch is the cell.
            let first_live = branches
                .iter()
                .find(|p| !p.blocked)
                .unwrap_or_else(|| panic!("no live branch for {root} {lakara} cell {cell}"));
            assert_eq!(
                first_live.text(),
                *expected,
                "the first live branch must be the declined derivation for {root} {lakara} cell {cell}"
            );
'''),
('crates/panini/tests/roundtrip.rs', 'use panini_data::{Lakara, Pada, Purusha, Vacana};',
 'use panini_data::{Lakara, Pada, Purusha, Vacana, optional_nic};'),
('crates/panini/tests/roundtrip.rs', '''        for p in engine.derive(d, lakara, pada, purusha, vacana) {
            // The cross-product only ever asks for padas the root admits, so
            // nothing here should be blocked. Assert it rather than
            // filtering: a blocked branch appearing would mean `padas()` and
            // the pada-sanction rules (1.3.12 / 1.3.78 / 10.0496) had come apart.
            assert!(
                !p.blocked,
                "{} {} {:?} {:?} {:?} derived a blocked branch",
                d.code,
                panini::lakara_name(lakara),
                pada,
                purusha,
                vacana
            );
''', '''        let branches = engine.derive(d, lakara, pada, purusha, vacana);
        // The cross-product only ever asks for padas the root admits, so
        // every cell has a live branch, and only a root whose ṇic is
        // optional (`OPTIONAL_NIC`) blocks one: its ṇic and ṇic-less
        // branches can differ in pada (10.0496 / 10.0497 / 1.3.78). Any
        // other blocked branch would mean `padas()` and the pada-sanction
        // rules had come apart.
        let cell = format!(
            "{} {} {:?} {:?} {:?}",
            d.code,
            panini::lakara_name(lakara),
            pada,
            purusha,
            vacana
        );
        assert!(
            branches.iter().any(|p| !p.blocked),
            "{cell} derived no live branch"
        );
        for p in &branches {
            assert!(
                !p.blocked || optional_nic(d.dhatupatha).is_some(),
                "{cell} derived a blocked branch"
            );
        }
        for p in branches.iter().filter(|p| !p.blocked) {
'''),
]
files, bad = {}, []
def get(p):
    if p not in files: files[p] = open(p).read()
    return files[p]
for p, old, new in E:
    s = get(p)
    if old is APPEND:
        if not s.endswith('    }\n}\n'): bad.append(f'{p}: no final test'); continue
        files[p] = s[:-2] + '\n' + new + '}\n'; continue
    n = s.count(old)
    if n != 1: bad.append(f'{p}: {n}× {old[:60]!r}'); continue
    files[p] = s.replace(old, new)
for p, old, new, want in REPL:
    s = get(p); n = s.count(old)
    if n != want: bad.append(f'{p}: {n}× (want {want}) {old[:60]!r}'); continue
    files[p] = s.replace(old, new)
if bad: sys.exit('not applied:\n  ' + '\n  '.join(bad))
for p, s in files.items(): open(p, 'w').write(s)
print(f'applied {len(E) + len(REPL)} edits to {len(files)} files')
```
