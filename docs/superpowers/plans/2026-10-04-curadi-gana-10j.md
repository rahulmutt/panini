# Curādi gaṇa slice 10j Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate curādi's eight ajanta rows — `10.0058 zmiN` (√smiṅ), `10.0124 ciY` (√ci), `10.0152 Gf` (√ghṛ), `10.0231 gf` (√gṛ), `10.0235 yu` (√yu), `10.0258 jYA` (√jñā), `10.0275 cyu` (√cyu) and `10.0277 BU` (√bhū) — across laṭ / laṅ / loṭ / vidhiliṅ, with the three rules they bring: 6.1.54 *cisphuror ṇau*, 7.3.36's puk and Kaumudī 2567. The golden suite goes from 25956 to 26424 cells, and curādi from 319 to 327 of its 509 rows.

**Architecture:** Six tasks:
- **Task 1** creates the worktree and checks the baseline.
- **Task 2** is the engine, green on its own. No curated root before this slice is √ci's row, is ā-final before ṇic, or is a ṅit curādi row, and the two rules that move (10.0493, 6.4.92) fire only on the six mit `a`-roots, whose traces do not change. So every golden and every prior trace is unchanged. It adds:
  - 6.1.54 *cisphuror ṇau*, a vikalpa right after 6.4.48, keyed on the row number through a new `samjna::CISPHUR` (the `MRJ` precedent);
  - 7.3.36's puk right before 7.2.115, appending `p` to an ā-final aṅga's text (7.3.37.2's nuk precedent);
  - Kaumudī 2567 at the head of `SAMJNA`, guarded on `Tag::Curadi`, `Tag::Nijanta`, `Tag::Atmanepadin` and the ātmanepada pada;
  - 10.0493 moved to the head of `SANADI`, and 6.4.92 moved to right after the sanādi 6.1.78.
  The tests go in first and fail.
- **Task 3** lands the eight rows, √ci's `OPTIONAL_NIC` entry, 52 golden rows and 186 alternates, and every count, list, roster and `check()` assertion they move. The assertions go in first and fail; the rows and goldens make them pass.
- **Tasks 4–6** are the audit with the prior-trace diff and the doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.99.0, pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, checked out at `/tmp/vidyut-full`.

**Spec:** `docs/superpowers/specs/2026-10-04-curadi-gana-10j-design.md`. Its Scope table is the slice's scope. Its Decisions section explains:
- why 7.3.36 appends to the aṅga's text and runs before 7.2.115;
- why 6.1.54 is keyed by row number (`10.0325 ci` stores as the same `ci`);
- why 6.4.92 and 10.0493 move;
- how 2567 is guarded, and where ṅit-ness is held (the data layer);
- the homographs and the negative witnesses that become forms.

**Workspace:** The spec and this plan are on the branch `curadi-10j`. Task 1 puts `/workspace` back on `main` and checks `curadi-10j` out at `/workspace/.worktrees/curadi-10j`. Every path below is relative to that directory unless it starts with `/`.

**Provenance.** This slice was built end to end on a throwaway worktree, `/workspace/.worktrees/curadi-10j-proto` (branch `proto-10j`, on the spec commit `1b4b9ff`). Its throwaway commits are `3ba9bca` (Task 2's tests), `b094b95` (engine), `5e04ca5` (Task 3's assertions), `e7b640c` (rows and goldens) and `3f67d85` (audit and docs), and Task 6 deletes it. Every script below was generated from those commits' diffs, or written and run against them, and was checked three ways:
- **Prototype checks.** The prototype's final state:
  - passed the full suite, clippy `-D warnings` and `fmt-check`;
  - its generator found all 468 new cells equal to vidyut's (468 cells, 654 forms, 0 differences);
  - the audit at 430 roots / 26424 cells / 37070 forms showed zero differences, with 6516 blocked branches and the `entry` control failing on 36 cells;
  - a main-vs-prototype dump of every prior branch, blocked ones included (42932 lines, 36416 of them live), was byte-identical.
- **Replay.** A replay ran the scripts below in this plan's order on a fresh worktree at `1b4b9ff` and reproduced the prototype's final tree byte for byte in every tracked file (tree `657a0d66b02f`). The replay script is `/tmp/vidyut-full/slice10j/replay_10j.sh`, a verification aid this plan does not need. Task 2 Step 2's and Task 3 Step 2's failing lists below are that replay's, and so are the per-binary counts.
- **Negative controls.** The prototype's first 6.1.54, keyed on the text `ci` instead of the row, made the audit fail on 72 cells, all `10.0325`'s. Disabling 6.1.54 (its guard forced to decline) made it fail on 72 cells, all `10.0124`'s.
- **Mutants.** The 17 mutants `--in-diff` lists for the slice's `panini-prakriya` diff were run on the prototype (`-j 4 --timeout 5140 -o /tmp/vidyut-full/slice10j/mut/indiff`, 15 minutes, load 58–82): **17 caught**, none missed or timed out.

The prototype did **not** run the full mutation campaign. Task 5's campaign numbers are expectations, derived from the measured mutant lists and the in-diff run above, and the campaign measures them.

**Throwaway scripts.** Everything under `/tmp/vidyut-full/slice10j/` and the two vidyut examples (`curadi_goldens_10j.rs`, `trace_dump_10j.rs`) never ship. Each is reproduced in full in this plan with its sha256, so it can be recreated if `/tmp` was cleaned. Recreate a file only if it is missing, and check its hash either way (`sha256sum <file>`).

## Global Constraints

- **The new grammar is exactly this:**
  - **ids:** 6.1.54 (`SANADI`, right after 6.4.48); 7.3.36 (`SANADI`, right after 7.3.37.2, before 7.2.115); 2567 (`SAMJNA`, first, before 1.3.12). The rule-order pin goes from 159 to 162. 10.0493 moves to the head of `SANADI`; 6.4.92 moves to right after the sanādi 6.1.78.
  - **names:** 6.1.54 `"cisPuror RO"`; 7.3.36 `"artihrIvlIrIknUyIkzmAyyAtAM pug RO"`; 2567 `"NittvasyAvayave'caritArTatvAR RijantAt taN"` (the Siddhānta-kaumudī's text at `10.0058 zmiN`, «ङित्त्वस्यावयवेऽचरितार्थत्वाण्णिजन्तात्तङ्», read from `ashtadhyayi-com/data`'s `sutraani/kaumudi.txt`).
  - **const:** `samjna::CISPHUR: [&str; 1] = ["10.0124"]`, `pub(crate)`, read by the sanādi 6.1.54.
  - **vikalpa:** 6.1.54 only. 7.3.36 and 2567 are not. No rule has new `bars`.
  Nothing else in the engine changes.
- **Rows:** exactly the spec's eight, appended to `DHATUS` in dhātupāṭha order, each `code` what `stored_form` computes. `10.0058` is `Atmanepada`, `10.0124` `NicUbhayapada`, `10.0231` and `10.0235` `Akusmiya`, the other four `Nic`. `OPTIONAL_NIC` gains exactly `("10.0124", "2570")`, after `10.0114` (181 entries).
- **Not modelled:** 7.3.36's named roots (`f`, `hrI`, `vlI`, `rI`, `knUy`, `kzmAy`), 6.1.54 on `sPura~`, svādi `ci\Y` (all need a causative); 1.4.13, which vidyut credits in the sanādi stage and no rule here does.
- **Out of scope:** the ~165 plain hal-anta curādi rows, `10.0368 za\da~` and upasargas, `gupU~` / `paRa~\` / `pana~\`, the causative.
- **Pre-existing cells must stay byte-identical, traces included,** with no exception. Regenerate no prior golden.
- **Goldens come from the generator, which asserts engine = vidyut cell by cell, and their sha256 must match this plan's.** **Do not edit a golden to match the engine.** If the generator reports a difference, or a hash differs, stop and report.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit. Run `git branch --show-current` before every commit and the push: it must print `curadi-10j` (a detached HEAD strands commits).
- `mise run test` took 3–4 minutes in the prototype under external load. Run it in the **foreground** with a timeout of 600000 ms; if a run outgrows the 10-minute cap, start it detached with its output in a log file and an `EXIT_CODE=` sentinel, then wait in the same turn with `while kill -0 <PID>; do sleep 30; done`. Never background it and end a turn, never arm a Monitor and end a turn, and never pipe it through `tail`.
- `mise run test -- -p X` does not scope. Scope with `mise exec -- cargo test -p <crate> <filter>`. To see every failing binary at once, use `mise exec -- cargo test --workspace --no-fail-fast`.
- The `cargo-mutants` mise shim fails here. Use the real binary, `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`, under `mise exec --`.
- `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes its `panini` and `panini-data` dev-deps. Repoint them at this worktree before generating or auditing, and back at `/workspace/crates` after. Check with `grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml` every time. (The prototype left them pointing at itself; Task 3 Step 4 repoints them.)
- Never wait on a process with `pgrep -f` (it matches its own shell); use `pgrep -x cargo-mutants` or `kill -0 <pid>`.
- Review packages and diffs exclude the appended goldens (`crates/panini/tests/paradigm/data/curadi.rs`) and use `git diff --diff-algorithm=histogram`: the default algorithm shows appended golden rows as fake deletions.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **6.1.54 on the other `ci`.** `10.0325 ci` (āsvadīya, 10i) stores as the same `ci` as `10.0124 ciY`; a guard on text would give it *cāpayati* in 72 cells. → Task 2 `cisphuror_nau_gives_ci_its_a_on_its_row_only` (declines on `10.0325` and on no row); `cisphur_is_the_ciy_row_6_1_54_names` (the key pinned to the upstream upadeśa `ciY`, and `10.0325`'s `ci`); Task 3 `the_10j_rules_fire_only_on_their_rows` (6.1.54 on `10.0124` alone, corpus-wide).
2. **7.2.115 on an ā-final aṅga.** `vrddhi_of('A')` is `Some("A")`, so a 7.2.115 that ran before puk would record a no-op step vidyut does not credit; goldens would not notice. → Task 2 `puk_follows_an_a_final_anga_before_nit_nic` (7.2.115 declines on `jYAp`, `cAp`) and `jna_takes_puk_and_no_vrddhi_before_nic` (the exact sanādi trace); Task 3 `the_10h_vrddhi_and_nuk_rules_fire_only_on_their_rows` (7.2.115's roster omits `10.0258`).
3. **6.4.92 before the vowel it shortens exists.** On √ci the long vowel comes from 7.2.115/6.1.78 or 6.1.54/7.3.36; a 6.4.92 left after 7.2.116 would give *cāyayati*. → Task 2 `ci_forks_on_6_1_54_and_shortens_after_vrddhi_and_puk` (both ṇic branches' exact sanādi traces) and `mitam_hrasvah_…` (`cAy`, `cAp`); Task 3 `the_mit_rules_are_credited_on_exactly_the_jnapadi_cells` (702 / 624 credits).
4. **2567 on a row that is not ṅit.** An anudāttet curādi row (`za\da~`, once upasargas land) is ātmanepadī by its marker but takes 1.3.74 under ṇic; 2567 must never reach it. → Task 2 `nittva_keeps_a_nit_curadi_nijanta_atmanepadi` (every guard tag both ways, and 1.3.12 blocking the parasmaipada); Task 3 `curated_pada_agrees_with_upadesha_markers` (a curādi `Atmanepada` row must be ṅit, and `10.0058` is the one ṅit curādi row upstream) and `the_10j_rules_fire_only_on_their_rows` (2567 on `10.0058` alone).
5. **Homographs and negative witnesses.** `capayati` (√cap / √ci), `cayati` (`10.0325` / `10.0124`) and `BAvayati` (`10.0382` / `10.0277`) gain a second analysis, and `cayayati`, `jYApayati` and `jYApayate` stop being non-forms; `trace_for` answers with the first analysis. → Task 3 `curadi_analyses_its_ajanta_forms` (roots in analysis order, the slice row's credits in order, the shapes the slice rules out), and the edits to `curadi_analyses_its_jnapadi_forms` and `curadi_analyses_its_asvadiya_forms`.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-prakriya/src/tinanta/sanadi.rs` | Task 2: module doc, `CISPHUR` import, 10.0493 moved first, 6.1.54, 7.3.36, 6.4.92 moved, 7.2.115's comment, two new unit tests and two widened. Task 3: two tests gain √ci's row |
| `crates/panini-prakriya/src/tinanta/samjna.rs` | Task 2: module doc, `CISPHUR`, 2567, its unit test and `CISPHUR`'s pin. Task 3: the pin gains "curated" |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Task 2: order (162) and vikalpa pins, the order pin's doc, three hand-built-row derivation tests |
| `crates/panini-data/src/lib.rs` | Task 3: 8 rows, `OPTIONAL_NIC` (181), `JNAPADI`'s and `Dhatu::pada`'s docs, the row list, row count, marker and jñapādi tests. Task 4: one comment |
| `crates/panini/tests/paradigm/main.rs` | Task 3: vikalpa list, census and keys, the 10j paragraph, the jñapādi and āsvadīya witness tests, the new 10j witness test, the pada-ambiguous comment and set. Task 4: doc counts and audit-chain prose |
| `crates/panini/tests/paradigm/data/curadi.rs` | Task 3: 52 golden rows, 186 alternates |
| `crates/panini/tests/trace/curadi.rs` | Task 3: `opens_nicless`, five moved rosters, the window helper's pada-rule list, one new test. Task 4: module doc |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini-prakriya/src/tinanta/guna.rs`, the 10b–10i specs | Task 4 |
| `AGENTS.md`, maybe `mise.toml` | Task 5 |

---

## Task 1: The worktree and baseline

**Files:** none. **Interfaces:** none.

- [ ] **Step 1: Create the worktree**

```bash
cd /workspace
git status --short                   # empty
git log --oneline -3 curadi-10j      # the plan commit, then the spec commits, then 11b1ab4
git checkout main                    # curadi-10j can be checked out in one worktree only
git log --oneline -1                 # 11b1ab4, the 10i merge
git worktree add .worktrees/curadi-10j curadi-10j
cd .worktrees/curadi-10j
git branch --show-current            # curadi-10j
```

`/workspace` stays on `main` for the whole slice: Task 4's prior-trace dump builds the main engine from `/workspace/crates`.

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: everything passes at 25956 cells. `panini-prakriya` reports 430 tests, the `trace` binary 214, `paradigm` 27 and `panini-data` 28 (and `panini` 7, `roundtrip` 1, `panini-analyze` 8, `cli` 5, `panini-lipi` 6).

---

## Task 2: The engine — 6.1.54, 7.3.36's puk, Kaumudī 2567, and 10.0493 and 6.4.92 moved

No curated root before this slice is √ci's row (`CISPHUR`), meets ṇic with an ā-final aṅga, or is a curādi `Atmanepada` row. 10.0493 and 6.4.92 fire only on the six curated jñapādi `a`-roots, and on those every rule between 6.4.92's old and new positions (7.3.37.2, 7.3.36, 7.2.115, the sanādi 6.1.78) declines, since `jYAp`, `yAm`, … end in a consonant. So every golden and every prior trace is unchanged, and the suite is green at the end of the task. Task 4's prior-trace diff is the corpus-wide proof.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/sanadi.rs` (module doc; import; 10.0493 moved; 6.1.54; 7.3.36; 6.4.92 moved; 7.2.115's comment; tests)
- Modify: `crates/panini-prakriya/src/tinanta/samjna.rs` (module doc; `CISPHUR`; 2567; tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (the order and vikalpa pins and the order pin's doc; three derivation tests at the end)

**Interfaces:**
- Produces:
  - `pub(crate) const samjna::CISPHUR: [&str; 1] = ["10.0124"]`;
  - the rule ids 6.1.54 (vikalpa), 7.3.36 and 2567;
  - the samjna test helper `nijanta_prakriya(tags: &[Tag], pada: Pada) -> Prakriya`;
  - the derivation-test helpers `ajanta_row(dhatupatha, code, pada) -> Dhatu` and `sanadi_ids(p: &Prakriya) -> Vec<&str>`.
- Consumes:
  - the sanādi tests' `with_nic(root, nic)` and `rule(id)`; the samjna tests' `upstream_upadesha(number)`;
  - `derivation_tests`' `sole(branches)`; `derive`.

- [ ] **Step 1: The failing tests**

Create `/tmp/vidyut-full/slice10j/engine_tests_10j.py` if it is missing (sha256 `bc785866f73a725bcc331595561b7b3ddc83d9dc46791c5ddaa26a657f7a967b`). It adds:
- **`sanadi.rs`:** `puk_follows_an_a_final_anga_before_nit_nic` and `cisphuror_nau_gives_ci_its_a_on_its_row_only`; `mitam_hrasvah_…` gains `cAy` → `cay` and `cAp` → `cap`; `aco_nniti_…` gains `jYAp` among the hal-final aṅgas.
- **`samjna.rs`:** the `CISPHUR` declaration (a declaration, no rule, so the tests compile); `nijanta_prakriya`; `nittva_keeps_a_nit_curadi_nijanta_atmanepadi`; `cisphur_is_the_ciy_row_6_1_54_names`.
- **`derivation_tests.rs`:** the order pin (162) and its doc's 10j paragraph; the vikalpa pin (+6.1.54); `ajanta_row`, `sanadi_ids` and three tests on hand-built rows: `ci_forks_on_6_1_54_and_shortens_after_vrddhi_and_puk`, `jna_takes_puk_and_no_vrddhi_before_nic`, `smin_stays_atmanepadi_under_nic_by_2567`.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10j, Task 2's failing tests — 7.3.36's, 6.1.54's and 2567's
unit tests, the CISPHUR declaration and its upstream pin, the three
hand-built-row derivation tests, and the rule-order and vikalpa pins. Every
`old` must occur exactly once in its file; nothing is written if one fails.
Run from the worktree root."""
import sys
E = {
'crates/panini-prakriya/src/tinanta/sanadi.rs': [
("""            ("kUw", "kuw"),
            ("pFq", "pfq"),
        ] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
""",
"""            ("kUw", "kuw"),
            ("pFq", "pfq"),
            // √ci's two ṇic branches: 7.2.115 and 6.1.78's `cAy`, and 6.1.54
            // and 7.3.36's `cAp`.
            ("cAy", "cay"),
            ("cAp", "cap"),
        ] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
"""),
("""            assert_eq!(ids, ["7.2.115"]);
        }
        // Hal-final: a root (cur), and √dhū after its nuk (DUn).
        for root in ["cur", "DUn"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("7.2.115").apply)(&mut p), "{root}");
""",
"""            assert_eq!(ids, ["7.2.115"]);
        }
        // Hal-final: a root (cur), √dhū after its nuk (DUn), and √jñā after
        // its puk (jYAp).
        for root in ["cur", "DUn", "jYAp"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("7.2.115").apply)(&mut p), "{root}");
"""),
("""        assert!(!(rule("7.2.115").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "BU");
    }

""",
"""        assert!(!(rule("7.2.115").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "BU");
    }

    #[test]
    fn puk_follows_an_a_final_anga_before_nit_nic() {
        // √jñā (`10.0258 jYA`), and √ci's 6.1.54 branch (`cA`).
        for (root, want) in [("jYA", "jYAp"), ("cA", "cAp")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("7.3.36").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["7.3.36"]);
            // 7.2.115 then finds the `p`, not an ac, at the end.
            assert!(!(rule("7.2.115").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
        }
        // Not ā-final: a short `a` (kaTa, before 6.4.48), an ik (ci, BU), a
        // consonant (cur).
        for root in ["kaTa", "ci", "BU", "cur"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("7.3.36").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        // No ṇic, or a pratyaya at `NIC` that is not ṇit.
        let mut p = with_nic("jYA", None);
        assert!(!(rule("7.3.36").apply)(&mut p));
        let mut p = with_nic("jYA", Some(&[Tag::Ardhadhatuka]));
        assert!(!(rule("7.3.36").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "jYA");
        assert!(!rule("7.3.36").vikalpa);
    }

    #[test]
    fn cisphuror_nau_gives_ci_its_a_on_its_row_only() {
        let mut p = with_nic("ci", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        p.ctx.dhatupatha = "10.0124";
        assert!((rule("6.1.54").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "cA");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["6.1.54"]);
        // `10.0325 ci` (bhāṣāyām) stores as the same `ci` but is not `ciY`,
        // and a row with no number is no row at all.
        for number in ["10.0325", ""] {
            let mut p = with_nic("ci", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            p.ctx.dhatupatha = number;
            assert!(!(rule("6.1.54").apply)(&mut p), "{number}");
            assert_eq!(p.terms[ANGA].text, "ci");
            assert!(p.log.is_empty(), "{number}");
        }
        // On its row, but no ṇic, or a pratyaya at `NIC` that is not ṇit.
        for nic in [None, Some(&[Tag::Ardhadhatuka][..])] {
            let mut p = with_nic("ci", nic);
            p.ctx.dhatupatha = "10.0124";
            assert!(!(rule("6.1.54").apply)(&mut p), "{nic:?}");
            assert_eq!(p.terms[ANGA].text, "ci");
        }
        let r = rule("6.1.54");
        assert!(r.vikalpa);
        assert!(r.bars.is_empty());
    }

"""),
],
'crates/panini-prakriya/src/tinanta/samjna.rs': [
("""/// `super::derive` reads this to add `Tag::Mrj`.
pub(crate) const MRJ: [&str; 1] = ["10.0386"];

pub(crate) static SAMJNA: &[Rule] = &[
""",
"""/// `super::derive` reads this to add `Tag::Mrj`.
pub(crate) const MRJ: [&str; 1] = ["10.0386"];

/// 6.1.54 cisphuror ṇau: the dhātupāṭha rows that are the √ci the sūtra
/// names. By number, as `MRJ` is: vidyut-prakriya keys the sūtra on the
/// upadeśas `ciY` and `ci\\Y`, and curādi `10.0325 ci` (*bhāṣāyām*) stores
/// as the same `ci`. `10.0124 ciY` is the row that meets ṇic; svādi
/// `05.0005 ci\\Y` would need a causative, as would the sūtra's other root,
/// tudādi `06.0121 sPura~`. Pinned to upstream by
/// `cisphur_is_the_ciy_row_6_1_54_names`. The sanādi 6.1.54 reads it.
pub(crate) const CISPHUR: [&str; 1] = ["10.0124"];

pub(crate) static SAMJNA: &[Rule] = &[
"""),
("""    }

    #[test]
    fn nicas_ca_reports_firing_only_on_atmanepada() {
""",
"""    }

    /// A curādi ṇijanta carrying `tags`, as the sanādi stage leaves √smiṅ
    /// (`smAyi`), in `pada`. Hand-built, as `nic_prakriya` is: 2567 reads
    /// only tags.
    fn nijanta_prakriya(tags: &[Tag], pada: Pada) -> Prakriya {
        let mut t = Term::new("smAyi");
        t.add(Tag::Dhatu);
        for tag in tags {
            t.add(*tag);
        }
        let mut p = Prakriya {
            ctx: Context::new(Lakara::Lat, pada, Purusha::Prathama, Vacana::Eka),
            ..Default::default()
        };
        p.terms = with_slots(vec![t]);
        p
    }

    #[test]
    fn nittva_keeps_a_nit_curadi_nijanta_atmanepadi() {
        let rule = SAMJNA.iter().find(|r| r.id == "2567").unwrap();
        let r1312 = SAMJNA.iter().find(|r| r.id == "1.3.12").unwrap();
        let smin = [Tag::Curadi, Tag::Atmanepadin, Tag::Nijanta];
        let mut p = nijanta_prakriya(&smin, Pada::Atmanepada);
        assert!((rule.apply)(&mut p));
        assert!((r1312.apply)(&mut p));
        assert!(!p.blocked);
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["2567", "1.3.12"]);
        assert_eq!(p.terms[ANGA].text, "smAyi", "a sanction, not an operation");
        // Parasmaipada: 2567 declines and records nothing; 1.3.12 blocks, as
        // for any ātmanepadī root.
        let mut p = nijanta_prakriya(&smin, Pada::Parasmaipada);
        assert!(!(rule.apply)(&mut p));
        assert!(!p.blocked);
        assert!(p.log.is_empty());
        assert!(!(r1312.apply)(&mut p));
        assert!(p.blocked);
        // One guard tag missing: not curādi, not ātmanepadī (√cur's `Nic`, an
        // ākusmīya root's `Akusmiya`), or no ṇic taken.
        for tags in [
            &[Tag::Atmanepadin, Tag::Nijanta][..],
            &[Tag::Curadi, Tag::Nic, Tag::Nijanta][..],
            &[Tag::Curadi, Tag::Akusmiya, Tag::Nijanta][..],
            &[Tag::Curadi, Tag::Atmanepadin][..],
        ] {
            for pada in [Pada::Parasmaipada, Pada::Atmanepada] {
                let mut p = nijanta_prakriya(tags, pada);
                assert!(!(rule.apply)(&mut p), "{tags:?} {pada:?}");
                assert!(!p.blocked, "{tags:?} {pada:?}");
                assert!(p.log.is_empty(), "{tags:?} {pada:?}");
            }
        }
    }

    #[test]
    fn nicas_ca_reports_firing_only_on_atmanepada() {
"""),
("""
    #[test]
    fn ghu_is_exactly_the_six_rows_1_1_20_names() {
        // 1.1.20 dādhā ghv adāp. Four of the six are not curated, so this
""",
"""
    #[test]
    fn cisphur_is_the_ciy_row_6_1_54_names() {
        // 6.1.54 cisphuror ṇau. vidyut-prakriya keys it on the upadeśas `ciY`
        // and `ci\\Y`, and on `sPura~`. Curādi `10.0325 ci` stores as the
        // same `ci` and is not √ci of 6.1.54; svādi `ci\\Y` and tudādi
        // `sPura~` meet ṇic only in a causative.
        assert_eq!(CISPHUR, ["10.0124"]);
        assert_eq!(upstream_upadesha("10.0124"), Some("ciY"));
        assert_eq!(upstream_upadesha("10.0325"), Some("ci"));
        for (number, upadesha) in [("05.0005", "ci\\\\Y"), ("06.0121", "sPura~")] {
            assert_eq!(upstream_upadesha(number), Some(upadesha), "{number}");
            assert!(!dhatus().iter().any(|d| d.dhatupatha == number), "{number}");
        }
    }

    #[test]
    fn ghu_is_exactly_the_six_rows_1_1_20_names() {
        // 1.1.20 dādhā ghv adāp. Four of the six are not curated, so this
"""),
],
'crates/panini-prakriya/src/tinanta/derivation_tests.rs': [
("""/// √vich's āya stands where ṇic would, and 3.4.114 and 3.1.32 treat it as
/// they treat ṇic. It also adds a second 6.1.73 *che ca* right before the
/// sanādi 7.3.86, whose guṇa √vich's tuk must block.
///
/// 6.4.106/6.4.107 sit BELOW 6.1.96 but ABOVE 6.1.90, against sūtra order
/// and against where Task 3 first placed them (after 6.4.105, below all
/// four of 6.1.90/97/87/66). 6.4.107's move is load-bearing: laṅ's āṭ-
/// vṛddhi ekādeśa (6.1.90's aṅga arm) can rewrite a vowel-initial tanādi
""",
"""/// √vich's āya stands where ṇic would, and 3.4.114 and 3.1.32 treat it as
/// they treat ṇic. It also adds a second 6.1.73 *che ca* right before the
/// sanādi 7.3.86, whose guṇa √vich's tuk must block.
///
/// Slice 10j moves 10.0493 to the very top, ahead of the optional-ṇic
/// vikalpas, where vidyut tags a jñapādi root mit (`dhatu_karya.rs`): √ci
/// (`10.0124`) is mit and its ṇic is optional. It adds 6.1.54 *cisphuror
/// ṇau* right after 6.4.48, 7.3.36's puk right before 7.2.115, and Kaumudī
/// 2567 right before 1.3.12. And it moves 6.4.92 from after 7.2.116 to after
/// the sanādi 6.1.78: √ci's long vowel comes from 7.2.115 or 6.1.54, so
/// 6.4.92 must read the aṅga after both.
///
/// 6.4.106/6.4.107 sit BELOW 6.1.96 but ABOVE 6.1.90, against sūtra order
/// and against where Task 3 first placed them (after 6.4.105, below all
/// four of 6.1.90/97/87/66). 6.4.107's move is load-bearing: laṅ's āṭ-
/// vṛddhi ekādeśa (6.1.90's aṅga arm) can rewrite a vowel-initial tanādi
"""),
("""/// comment on the block for the full argument.
#[test]
fn tinanta_rule_order_is_pinned() {
    let expected = [
        "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3", "2573.2",
        "10.0496", "10.0497", "10.0493", "3.1.25", "3.1.28", "1.3.9", "3.4.114", "6.4.48",
        "7.2.116", "6.4.92", "7.3.37.2", "7.2.115", "6.1.78", "7.2.114", "6.1.73", "7.3.86",
        "3.1.32", "1.3.12", "1.3.66", "1.3.72", "1.3.74", "1.3.78", "3.4.78", "1.3.9", "1.2.4",
        "3.4.85", "3.4.108", "3.4.109", "3.4.105", "3.4.106", "3.4.101", "3.4.99", "3.4.87",
        "3.4.89", "3.4.86", "3.4.100", "3.4.80", "3.4.79", "3.4.91", "3.4.93", "3.4.90", "3.4.92",
        "3.4.103", "3.4.102", "7.1.35", "3.1.69", "3.1.73", "3.1.77", "3.1.78", "3.1.79", "3.1.81",
        "3.1.68", "2.4.72", "2.4.75", "3.4.111", "3.1.83", "1.2.4", "6.1.10", "7.4.66", "7.4.60",
        "7.4.59", "7.4.62", "7.4.75", "7.4.76", "7.4.77", "7.4.78", "6.4.78", "6.4.71", "6.4.72",
        "6.1.73", "7.3.100", "7.1.5", "7.1.6", "7.1.4", "7.1.3", "7.2.79", "7.2.80", "7.2.81",
        "6.4.23", "7.4.21", "7.3.83", "7.3.87", "7.2.114", "7.3.84", "7.3.86", "7.3.86", "7.3.92",
        "7.3.84", "7.1.102", "6.4.110", "6.4.108", "6.4.109", "6.4.87", "6.4.82", "6.4.77",
        "6.1.77", "6.1.78", "7.3.101", "6.4.119", "6.4.118", "6.4.117", "6.4.116", "6.4.113",
        "6.4.98", "6.4.100", "6.4.112", "6.4.115", "6.4.42", "6.4.43", "6.1.97", "6.1.101",
        "6.1.101", "6.1.96", "6.4.106", "6.4.107", "6.1.90", "6.1.88", "6.1.97", "6.1.87",
        "6.1.66", "6.4.105", "6.4.101", "6.4.111", "8.2.77", "8.2.23", "8.2.25", "8.2.26",
        "8.2.30", "8.2.31", "8.2.39", "8.2.40", "8.2.41", "8.2.74", "8.2.75", "8.2.73", "8.3.15",
        "8.3.24", "8.3.59", "8.4.40", "8.4.41", "8.3.13", "8.4.53", "8.4.54", "8.2.38", "8.4.55",
        "8.4.1", "8.4.2", "8.4.58", "8.4.65", "8.4.56",
    ];
    let actual: Vec<&str> = rules().map(|r| r.id).collect();
    assert_eq!(actual, expected);
}
""",
"""/// comment on the block for the full argument.
#[test]
fn tinanta_rule_order_is_pinned() {
    let expected = [
        "10.0493", "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3",
        "2573.2", "10.0496", "10.0497", "3.1.25", "3.1.28", "1.3.9", "3.4.114", "6.4.48", "6.1.54",
        "7.2.116", "7.3.37.2", "7.3.36", "7.2.115", "6.1.78", "6.4.92", "7.2.114", "6.1.73",
        "7.3.86", "3.1.32", "2567", "1.3.12", "1.3.66", "1.3.72", "1.3.74", "1.3.78", "3.4.78",
        "1.3.9", "1.2.4", "3.4.85", "3.4.108", "3.4.109", "3.4.105", "3.4.106", "3.4.101",
        "3.4.99", "3.4.87", "3.4.89", "3.4.86", "3.4.100", "3.4.80", "3.4.79", "3.4.91", "3.4.93",
        "3.4.90", "3.4.92", "3.4.103", "3.4.102", "7.1.35", "3.1.69", "3.1.73", "3.1.77", "3.1.78",
        "3.1.79", "3.1.81", "3.1.68", "2.4.72", "2.4.75", "3.4.111", "3.1.83", "1.2.4", "6.1.10",
        "7.4.66", "7.4.60", "7.4.59", "7.4.62", "7.4.75", "7.4.76", "7.4.77", "7.4.78", "6.4.78",
        "6.4.71", "6.4.72", "6.1.73", "7.3.100", "7.1.5", "7.1.6", "7.1.4", "7.1.3", "7.2.79",
        "7.2.80", "7.2.81", "6.4.23", "7.4.21", "7.3.83", "7.3.87", "7.2.114", "7.3.84", "7.3.86",
        "7.3.86", "7.3.92", "7.3.84", "7.1.102", "6.4.110", "6.4.108", "6.4.109", "6.4.87",
        "6.4.82", "6.4.77", "6.1.77", "6.1.78", "7.3.101", "6.4.119", "6.4.118", "6.4.117",
        "6.4.116", "6.4.113", "6.4.98", "6.4.100", "6.4.112", "6.4.115", "6.4.42", "6.4.43",
        "6.1.97", "6.1.101", "6.1.101", "6.1.96", "6.4.106", "6.4.107", "6.1.90", "6.1.88",
        "6.1.97", "6.1.87", "6.1.66", "6.4.105", "6.4.101", "6.4.111", "8.2.77", "8.2.23",
        "8.2.25", "8.2.26", "8.2.30", "8.2.31", "8.2.39", "8.2.40", "8.2.41", "8.2.74", "8.2.75",
        "8.2.73", "8.3.15", "8.3.24", "8.3.59", "8.4.40", "8.4.41", "8.3.13", "8.4.53", "8.4.54",
        "8.2.38", "8.4.55", "8.4.1", "8.4.2", "8.4.58", "8.4.65", "8.4.56",
    ];
    let actual: Vec<&str> = rules().map(|r| r.id).collect();
    assert_eq!(actual, expected);
}
"""),
("""fn exactly_the_pinned_vikalpa_rules_are_optional() {
    let actual: Vec<&str> = rules().filter(|r| r.vikalpa).map(|r| r.id).collect();
    let expected = [
        "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3", "2573.2",
        "7.3.37.2", "7.1.35", "3.4.111", "7.3.86", "6.4.117", "6.4.116", "6.4.115", "6.4.43",
        "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56",
    ];
    assert_eq!(actual, expected);
}

""",
"""fn exactly_the_pinned_vikalpa_rules_are_optional() {
    let actual: Vec<&str> = rules().filter(|r| r.vikalpa).map(|r| r.id).collect();
    let expected = [
        "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3", "2573.2",
        "6.1.54", "7.3.37.2", "7.1.35", "3.4.111", "7.3.86", "6.4.117", "6.4.116", "6.4.115",
        "6.4.43", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56",
    ];
    assert_eq!(actual, expected);
}

"""),
("""            d.gana
        );
    }
}
""",
"""            d.gana
        );
    }
}

/// A curādi row slice 10j's rules reach, hand-built: `derive` reads only the
/// row number, the code, the gaṇa and the pada assignment, and the rules
/// under test need nothing else.
fn ajanta_row(dhatupatha: &'static str, code: &'static str, pada: PadaAssignment) -> Dhatu {
    Dhatu {
        dhatupatha,
        code,
        gana: Gana::Curadi,
        pada,
        artha: "",
    }
}

/// The ids a branch credits in the sanādi stage: everything up to and
/// including 3.1.32, or the whole log if 3.1.32 never ran.
fn sanadi_ids(p: &Prakriya) -> Vec<&str> {
    let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
    match ids.iter().position(|id| *id == "3.1.32") {
        Some(i) => ids[..=i].to_vec(),
        None => ids,
    }
}

#[test]
fn ci_forks_on_6_1_54_and_shortens_after_vrddhi_and_puk() {
    // √ci's ṇic branches: 6.1.54 declined, then 7.2.115 and 6.1.78 make the
    // long `cAy`; taken, 7.3.36 makes `cAp`. 6.4.92 shortens both, so it must
    // run after all four. The ṇic-less branch (2570's, once the row is in
    // `OPTIONAL_NIC`) is filtered out: it never reaches 3.1.25.
    let ci = ajanta_row("10.0124", "ci", PadaAssignment::NicUbhayapada);
    let branches = derive(
        &ci,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    let mut nic: Vec<(String, Vec<&str>)> = Vec::new();
    for p in &branches {
        let ids = sanadi_ids(p);
        if ids.contains(&"3.1.25") {
            assert!(!p.blocked);
            nic.push((p.text(), ids));
        }
    }
    assert_eq!(
        nic,
        [
            (
                "cayayati".to_string(),
                vec![
                    "10.0493", "3.1.25", "1.3.9", "3.4.114", "7.2.115", "6.1.78", "6.4.92",
                    "3.1.32",
                ]
            ),
            (
                "capayati".to_string(),
                vec![
                    "10.0493", "3.1.25", "1.3.9", "3.4.114", "6.1.54", "7.3.36", "6.4.92",
                    "3.1.32",
                ]
            ),
        ]
    );
}

#[test]
fn jna_takes_puk_and_no_vrddhi_before_nic() {
    // `10.0258 jYA` is not mit: puk, no 7.2.115 (it would find the `p`), and
    // no 6.4.92, so the `A` stays long.
    let jna = ajanta_row("10.0258", "jYA", PadaAssignment::Nic);
    let p = sole(derive(
        &jna,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    ));
    assert_eq!(p.text(), "jYApayati");
    assert_eq!(
        sanadi_ids(&p),
        ["3.1.25", "1.3.9", "3.4.114", "7.3.36", "3.1.32"]
    );
}

#[test]
fn smin_stays_atmanepadi_under_nic_by_2567() {
    // √smiṅ (`10.0058 zmiN`, stored `smi`) is ṅit: under ṇic, Kaumudī 2567
    // keeps it ātmanepadī and 1.3.12 credits, not 1.3.74.
    let smin = ajanta_row("10.0058", "smi", PadaAssignment::Atmanepada);
    let p = sole(derive(
        &smin,
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    ));
    assert_eq!(p.text(), "smAyayate");
    let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
    assert_eq!(
        ids[..8],
        [
            "3.1.25", "1.3.9", "3.4.114", "7.2.115", "6.1.78", "3.1.32", "2567", "1.3.12"
        ]
    );
    assert!(!ids.contains(&"1.3.74"));
    let p = sole(derive(
        &smin,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    ));
    assert!(p.blocked);
}
"""),
],
}
out, bad = {}, []
for path, edits in E.items():
    s = open(path).read()
    for old, new in edits:
        n = s.count(old)
        if n != 1:
            bad.append(f"{path}: {n}x {old[:70]!r}")
            continue
        s = s.replace(old, new)
    out[path] = s
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
for path, s in out.items():
    open(path, 'w').write(s)
print(f"applied {sum(len(e) for e in E.values())} edits")
```

```bash
python3 /tmp/vidyut-full/slice10j/engine_tests_10j.py      # applied 10 edits
mise run fmt
```

- [ ] **Step 2: Run them to see them fail**

Run: `mise exec -- cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .*FAILED|test result: FAILED"`
Expected: `panini-prakriya` `429 passed; 8 failed`, every other binary passing:

- `tinanta::derivation_tests::ci_forks_on_6_1_54_and_shortens_after_vrddhi_and_puk`
- `tinanta::derivation_tests::exactly_the_pinned_vikalpa_rules_are_optional`
- `tinanta::derivation_tests::jna_takes_puk_and_no_vrddhi_before_nic`
- `tinanta::derivation_tests::smin_stays_atmanepadi_under_nic_by_2567`
- `tinanta::derivation_tests::tinanta_rule_order_is_pinned`
- `tinanta::samjna::tests::nittva_keeps_a_nit_curadi_nijanta_atmanepadi`
- `tinanta::sanadi::tests::cisphuror_nau_gives_ci_its_a_on_its_row_only`
- `tinanta::sanadi::tests::puk_follows_an_a_final_anga_before_nit_nic`

`mitam_hrasvah_…` and `aco_nniti_…` pass already: their guards read the aṅga's shape, which the new cases exercise without a new rule.

- [ ] **Step 3: The change**

Create `/tmp/vidyut-full/slice10j/engine_code_10j.py` if it is missing (sha256 `c99daaff66030addf36eb8c21856f4412507b963dc6ca3ee20bbe2094c2a1430`). Its 6.4.92 hunks are a move: the block leaves its place after 7.2.116 and arrives, with a rewritten comment, after the sanādi 6.1.78; 10.0493's block moves to the head of `SANADI` the same way.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10j, Task 2's engine — 10.0493 to the head of SANADI,
6.1.54 after 6.4.48, 7.3.36 before 7.2.115, 6.4.92 after the sanādi 6.1.78,
Kaumudī 2567 before 1.3.12, and the module docs. Every `old` must occur
exactly once in its file; nothing is written if one fails. Run from the
worktree root, after engine_tests_10j.py."""
import sys
E = {
'crates/panini-prakriya/src/tinanta/sanadi.rs': [
("""//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, 3.1.28's
//! āya where no ṇic is taken, ṇic's it-lopa (1.3.9), 3.4.114, 6.4.48,
//! 7.2.116, 6.4.92, the vārttika 7.3.37.2, 7.2.115, 6.1.78, 7.2.114, 6.1.73,
//! 7.3.86, 3.1.32 — opened by eight vikalpas that fork a root whose ṇic is
//! optional into its ṇic and ṇic-less branches (the dhātupāṭha gaṇasūtras
//! 10.0498 and 10.0499 and the Kaumudī's 2564, 2565, 2570, 2571, 2573.1,
//! 2573.3), a ninth (2573.2) that forks `pata`'s ṇic branch on its final
//! `a`, and three more dhātupāṭha gaṇasūtras: 10.0496 and 10.0497, which
//! settle an ākusmīya or ā-garvīya root's pada, and 10.0493, which credits a
//! jñapādi root's mit-tva, all before ṇic is added.
//!
//! First in the pipeline, before any lakāra or tiṅ exists. The layout here
""",
"""//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, 3.1.28's
//! āya where no ṇic is taken, ṇic's it-lopa (1.3.9), 3.4.114, 6.4.48,
//! 6.1.54, 7.2.116, the vārttika 7.3.37.2, 7.3.36, 7.2.115, 6.1.78, 6.4.92,
//! 7.2.114, 6.1.73, 7.3.86, 3.1.32 — opened by the dhātupāṭha gaṇasūtra
//! 10.0493, which credits a jñapādi root's mit-tva, then eight vikalpas that
//! fork a root whose ṇic is optional into its ṇic and ṇic-less branches (the
//! gaṇasūtras 10.0498 and 10.0499 and the Kaumudī's 2564, 2565, 2570, 2571,
//! 2573.1, 2573.3), a ninth (2573.2) that forks `pata`'s ṇic branch on its
//! final `a`, and two more gaṇasūtras, 10.0496 and 10.0497, which settle an
//! ākusmīya or ā-garvīya root's pada, all before ṇic is added.
//!
//! First in the pipeline, before any lakāra or tiṅ exists. The layout here
"""),
("""//! 3.1.32 on ṇic or āya, 6.4.48, 7.2.114 and 7.3.86 on its being
//! ārdhadhātuka, the others on ṇic's ṇit (6.4.48 on an `a`-final aṅga as
//! well, 6.4.92 on `Tag::Mit`, 7.3.37.2 on √dhū's and √prī's text, 7.2.115
//! on an ac-final aṅga, 6.1.78 on an ec-final one, 7.2.114 on `Tag::Mrj`,
//! and 7.2.116 and 7.3.86 decline on 6.4.48's `Tag::AtLopa`).
//! 6.1.73 and 3.1.28 carry no gaṇa guard, only their own conditions (3.1.28
//! reads the āya rows); on today's corpus the gaṇas 1–9 add nothing and
""",
"""//! 3.1.32 on ṇic or āya, 6.4.48, 7.2.114 and 7.3.86 on its being
//! ārdhadhātuka, the others on ṇic's ṇit (6.4.48 on an `a`-final aṅga as
//! well, 6.1.54 on √ci's row (`super::samjna::CISPHUR`), 6.4.92 on
//! `Tag::Mit`, 7.3.37.2 on √dhū's and √prī's text, 7.3.36 on an `A`-final
//! aṅga, 7.2.115 on an ac-final one, 6.1.78 on an ec-final one, 7.2.114 on
//! `Tag::Mrj`, and 7.2.116 and 7.3.86 decline on 6.4.48's `Tag::AtLopa`).
//! 6.1.73 and 3.1.28 carry no gaṇa guard, only their own conditions (3.1.28
//! reads the āya rows); on today's corpus the gaṇas 1–9 add nothing and
"""),
("""use crate::term::{Tag, Term};
use crate::tinanta::anga::che_ca;
use crate::tinanta::sound::{guna_of, hrasva_of, vrddhi_of};
use crate::tinanta::terms::{ANGA, NIC};
""",
"""use crate::term::{Tag, Term};
use crate::tinanta::anga::che_ca;
use crate::tinanta::samjna::CISPHUR;
use crate::tinanta::sound::{guna_of, hrasva_of, vrddhi_of};
use crate::tinanta::terms::{ANGA, NIC};
"""),
("""
pub(crate) static SANADI: &[Rule] = &[
    // 10.0498 ā dhṛṣād vā: the ādhṛṣīya — the curādi rows from `10.0338
    // yu\\ja~` to `10.0388 Dfza~` — take ṇic optionally (*yojayati* /
""",
"""
pub(crate) static SANADI: &[Rule] = &[
    // 10.0493, the gaṇasūtra closing the jñapādi (`10.0118 jYapa~` …
    // `10.0124 ciY`): these roots are mit. The verdict is `Tag::Mit`, which
    // `super::derive` sets from the data layer's `JNAPADI` range as it sets
    // `Tag::Ghu`; this entry credits it and changes no text, as 10.0496
    // credits `Tag::Akusmiya`. First in the stage because vidyut-prakriya
    // tags a jñapādi root mit there, as soon as the dhātu is identified and
    // before any optional-ṇic fork (`dhatu_karya.rs`): √ci (`10.0124`) is mit
    // and its ṇic is optional (2570), so 10.0493 is credited on both its
    // branches, ahead of 2570. 6.4.92 below is what the tag feeds.
    //
    // Not here: 01.0934 (am-final roots are mit) and 10.0494 *nānye mito
    // 'hetau* (no other curādi root is mit, outside the causative). With no
    // causative in this engine, 01.0934 could reach only curādi am-final
    // roots, where 10.0494 always blocks it: neither could fire or fail a
    // test. Both wait for a causative slice.
    Rule {
        id: "10.0493",
        name: "jYapAdayo mitaH",
        kind: RuleKind::Samjna,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Mit) {
                return false;
            }
            let before = p.snapshot();
            p.record("10.0493", "jYapAdayo mitaH", before);
            true
        },
    },
    // 10.0498 ā dhṛṣād vā: the ādhṛṣīya — the curādi rows from `10.0338
    // yu\\ja~` to `10.0388 Dfza~` — take ṇic optionally (*yojayati* /
"""),
("""    // dhātupāṭha. Rule ids are opaque strings everywhere they are read.
    //
    // First in the stage because vidyut credits it there, as soon as the
    // dhātu is identified and before 3.1.25. It settles the pada outright, so
    // no pada sūtra in `super::samjna` is credited after it: 1.3.12, 1.3.66,
    // 1.3.72 and 1.3.74 decline on their own guards, 1.3.78 on this tag. The
""",
"""    // dhātupāṭha. Rule ids are opaque strings everywhere they are read.
    //
    // After the optional-ṇic vikalpas and before 3.1.25, because vidyut
    // credits it there, once ṇic is decided. It settles the pada outright, so
    // no pada sūtra in `super::samjna` is credited after it: 1.3.12, 1.3.66,
    // 1.3.72 and 1.3.74 decline on their own guards, 1.3.78 on this tag. The
"""),
("""                }
            }
        },
    },
    // 10.0493, the gaṇasūtra closing the jñapādi (`10.0118 jYapa~` …
    // `10.0124 ciY`): these roots are mit. The verdict is `Tag::Mit`, which
    // `super::derive` sets from the data layer's `JNAPADI` range as it sets
    // `Tag::Ghu`; this entry credits it and changes no text, as 10.0496
    // credits `Tag::Akusmiya`. Second in the stage because vidyut-prakriya
    // credits it there, once the dhātu is identified and before 3.1.25.
    // 6.4.92 below is what the tag feeds.
    //
    // Not here: 01.0934 (am-final roots are mit) and 10.0494 *nānye mito
    // 'hetau* (no other curādi root is mit, outside the causative). With no
    // causative in this engine, 01.0934 could reach only curādi am-final
    // roots, where 10.0494 always blocks it: neither could fire or fail a
    // test. Both wait for a causative slice.
    Rule {
        id: "10.0493",
        name: "jYapAdayo mitaH",
        kind: RuleKind::Samjna,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Mit) {
                return false;
            }
            let before = p.snapshot();
            p.record("10.0493", "jYapAdayo mitaH", before);
            true
        },
    },
""",
"""                }
            }
        },
    },
"""),
("""        },
    },
    // 7.2.116 ata upadhāyāḥ: vṛddhi of an `a` upadhā before a ñit or ṇit
    // affix. `laq` → `lAq` before ṇic. Only ṇit has a carrier in this engine
""",
"""        },
    },
    // 6.1.54 cisphuror ṇau: before ṇic, √ci's `i` optionally becomes `A`
    // (`ci` → `cA`). Taken, 7.3.36 below adds puk (`cAp`) and 6.4.92 shortens
    // it (*capayati*); declined, 7.2.115 and 6.1.78 make `cAy` and 6.4.92
    // shortens that (*cayayati*). vidyut-prakriya credits it at this point,
    // right after 3.4.114. Keyed on the row number (`super::samjna::CISPHUR`),
    // as vidyut keys it on the upadeśa `ciY`: curādi `10.0325 ci` stores as
    // the same `ci` and takes no `A`. The sūtra's `sphur` meets ṇic only in a
    // causative.
    Rule {
        id: "6.1.54",
        name: "cisPuror RO",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &[],
        apply: |p| {
            if !CISPHUR.contains(&p.ctx.dhatupatha)
                || !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit))
            {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix('i') else {
                return false;
            };
            let text = format!("{stem}A");
            let before = p.snapshot();
            p.terms[ANGA].text = text;
            p.record("6.1.54", "cisPuror RO", before);
            true
        },
    },
    // 7.2.116 ata upadhāyāḥ: vṛddhi of an `a` upadhā before a ñit or ṇit
    // affix. `laq` → `lAq` before ṇic. Only ṇit has a carrier in this engine
"""),
("""            p.terms[ANGA].text = chars.into_iter().collect();
            p.record("7.2.116", "ata upaDAyAH", before);
            true
        },
    },
    // 6.4.92 mitāṃ hrasvaḥ: a mit root's upadhā is shortened before ṇi.
    // `jYAp` → `jYap`: here it undoes the 7.2.116 vṛddhi just above, so
    // √jñap makes *jñapayati*, not *jñāpayati*. Guarded on `Tag::Mit`
    // (10.0493's verdict) and on ṇit ṇic; a short upadhā declines.
    //
    // vidyut-prakriya credits 6.4.92 later, among its asiddhavat rules, after
    // 7.3.84 guṇates ṇic's `i`. It sits here, while root and ṇic are still
    // separate terms and the upadhā is one character read; after 3.1.32 the
    // vowel would be inside `jYApe`. The forms agree for every root curated
    // here: the one intervening upadhā-reader, 7.3.86, needs a laghu ik and
    // declines on these six `a` roots. A long-ik mit root (√ci) must revisit.
    Rule {
        id: "6.4.92",
        name: "mitAM hrasvaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Mit) || !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) {
                return false;
            }
            let mut chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let Some(upadha) = chars.len().checked_sub(2) else {
                return false;
            };
            let Some(short) = hrasva_of(chars[upadha]) else {
                return false;
            };
            let before = p.snapshot();
            chars[upadha] = short;
            p.terms[ANGA].text = chars.into_iter().collect();
            p.record("6.4.92", "mitAM hrasvaH", before);
            true
        },
""",
"""            p.terms[ANGA].text = chars.into_iter().collect();
            p.record("7.2.116", "ata upaDAyAH", before);
            true
        },
"""),
("""        },
    },
    // 7.2.115 aco ñṇiti: vṛddhi of an ac-final aṅga before a ñit or ṇit
    // affix. Before ṇic: `lI` → `lE`, `BU` → `BO`, `vf` → `vAr`, `jF` →
""",
"""        },
    },
    // 7.3.36 artihrīvlīrīknūyīkṣmāyyātāṃ puk ṇau: an ā-final aṅga takes the
    // augment puk before ṇic (*jñāpayati*; √ci's 6.1.54 branch `cA` →
    // `cAp`). puk is kit, so, like 7.3.37.2's nuk above, its `p` is appended
    // to the aṅga's text (1.1.46 ādyantau ṭakitau), its it-lopa unrecorded.
    // vidyut-prakriya inserts a term; one here, between `ANGA` and `NIC`,
    // would move ṇic off the index every rule in this stage reads. Before
    // 7.2.115: `vrddhi_of` maps `A` to itself, so on a bare `jYA` 7.2.115
    // would record a step vidyut does not credit; after puk it finds the `p`
    // and declines. The sūtra's named roots (`f`, `hrI`, `vlI`, `rI`,
    // `knUy`, `kzmAy`) meet ṇic only in a causative, so the ā-final arm is
    // the one here.
    Rule {
        id: "7.3.36",
        name: "artihrIvlIrIknUyIkzmAyyAtAM pug RO",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit))
                || !p.terms[ANGA].text.ends_with('A')
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text.push('p');
            p.record("7.3.36", "artihrIvlIrIknUyIkzmAyyAtAM pug RO", before);
            true
        },
    },
    // 7.2.115 aco ñṇiti: vṛddhi of an ac-final aṅga before a ñit or ṇit
    // affix. Before ṇic: `lI` → `lE`, `BU` → `BO`, `vf` → `vAr`, `jF` →
"""),
("""    // (`Tag::Rit`), as for 7.2.116. vidyut-prakriya credits it here, before
    // 3.1.32. An `a`-final root never reaches it: 6.4.48 has deleted the `a`.
    Rule {
        id: "7.2.115",
""",
"""    // (`Tag::Rit`), as for 7.2.116. vidyut-prakriya credits it here, before
    // 3.1.32. An `a`-final root never reaches it: 6.4.48 has deleted the `a`.
    // Nor does an `A`-final one: 7.3.36 has added its `p`.
    Rule {
        id: "7.2.115",
"""),
("""            p.terms[ANGA].text = text;
            p.record("6.1.78", "eco'yavAyAvaH", before);
            true
        },
""",
"""            p.terms[ANGA].text = text;
            p.record("6.1.78", "eco'yavAyAvaH", before);
            true
        },
    },
    // 6.4.92 mitāṃ hrasvaḥ: a mit root's upadhā is shortened before ṇi.
    // `jYAp` → `jYap`: it undoes 7.2.116's vṛddhi, so √jñap makes
    // *jñapayati*, not *jñāpayati*. On √ci it undoes 7.2.115's and 6.1.78's
    // `cAy` (*cayayati*) and 6.1.54's and 7.3.36's `cAp` (*capayati*), so it
    // sits after all of them. Guarded on `Tag::Mit` (10.0493's verdict) and
    // on ṇit ṇic; a short upadhā declines.
    //
    // vidyut-prakriya credits 6.4.92 later, among its asiddhavat rules, after
    // 7.3.84 guṇates ṇic's `i`. It sits here, while root and ṇic are still
    // separate terms and the upadhā is one character read; after 3.1.32 the
    // vowel would be inside `jYApe`. The forms agree for every root curated
    // here: the upadhā-readers after it, 7.2.114 and 7.3.86, need √mṛj or a
    // laghu ik, and decline on the six `a` roots and on `cay` and `cap`.
    Rule {
        id: "6.4.92",
        name: "mitAM hrasvaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Mit) || !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) {
                return false;
            }
            let mut chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let Some(upadha) = chars.len().checked_sub(2) else {
                return false;
            };
            let Some(short) = hrasva_of(chars[upadha]) else {
                return false;
            };
            let before = p.snapshot();
            chars[upadha] = short;
            p.terms[ANGA].text = chars.into_iter().collect();
            p.record("6.4.92", "mitAM hrasvaH", before);
            true
        },
"""),
],
'crates/panini-prakriya/src/tinanta/samjna.rs': [
("""//! Saṃjñā, pada sanction and ending insertion: 1.1.20 (as the `GHU` set),
//! 1.3.12, 1.3.66, 1.3.72, 1.3.74, 1.3.78, 3.4.78, 1.3.9, 1.2.4.
//!
//! One pada sanction is settled before this stage: the dhātupāṭha gaṇasūtra
""",
"""//! Saṃjñā, pada sanction and ending insertion: 1.1.20 (as the `GHU` set),
//! Kaumudī 2567, 1.3.12, 1.3.66, 1.3.72, 1.3.74, 1.3.78, 3.4.78, 1.3.9, 1.2.4.
//!
//! One pada sanction is settled before this stage: the dhātupāṭha gaṇasūtra
"""),
("""
pub(crate) static SAMJNA: &[Rule] = &[
    // 1.3.12 anudāttaṅita ātmanepadam: a root carrying the anudātta/ṅit
    // marker (here: the data-layer Atmanepadin tag) takes ātmanepada.
""",
"""
pub(crate) static SAMJNA: &[Rule] = &[
    // Kaumudī 2567: a ṅit curādi root stays ātmanepadī under ṇic. With ṇic,
    // 1.3.74 ṇicaś ca would govern and 1.3.78 admit parasmaipada too; the
    // Kaumudī reads the ṅit as having a purpose only if the ṇijanta takes
    // the ātmanepada (*ṅittvasyāvayave 'caritārthatvāṇ ṇijantāt taṅ*,
    // *smāyayate*, on `10.0058 zmiN`). vidyut-prakriya steps
    // `Kaumudi("2567")` and then runs 1.3.12 (`atmanepada.rs`), which
    // sanctions or blocks below. Guarded on `Tag::Curadi`, `Tag::Nijanta`
    // and `Tag::Atmanepadin` (the data layer's `PadaAssignment::Atmanepada`,
    // which a curādi row carries only with a ṅit upadeśa:
    // `curated_pada_agrees_with_upadesha_markers`). Records in the ātmanepada
    // only; on a parasmaipada request it declines, and 1.3.12 blocks. Not a
    // sūtra of the Aṣṭādhyāyī: numbered as vidyut numbers it.
    Rule {
        id: "2567",
        name: "NittvasyAvayave'caritArTatvAR RijantAt taN",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let anga = &p.terms[ANGA];
            if !anga.has(Tag::Curadi)
                || !anga.has(Tag::Nijanta)
                || !anga.has(Tag::Atmanepadin)
                || p.ctx.pada != Pada::Atmanepada
            {
                return false;
            }
            let before = p.snapshot();
            p.record("2567", "NittvasyAvayave'caritArTatvAR RijantAt taN", before);
            true
        },
    },
    // 1.3.12 anudāttaṅita ātmanepadam: a root carrying the anudātta/ṅit
    // marker (here: the data-layer Atmanepadin tag) takes ātmanepada.
"""),
],
}
out, bad = {}, []
for path, edits in E.items():
    s = open(path).read()
    for old, new in edits:
        n = s.count(old)
        if n != 1:
            bad.append(f"{path}: {n}x {old[:70]!r}")
            continue
        s = s.replace(old, new)
    out[path] = s
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
for path, s in out.items():
    open(path, 'w').write(s)
print(f"applied {sum(len(e) for e in E.values())} edits")
```

```bash
python3 /tmp/vidyut-full/slice10j/engine_code_10j.py      # applied 13 edits
mise run fmt
```

- [ ] **Step 4: Run the tests and the suite**

```bash
mise exec -- cargo test -q -p panini-prakriya 2>&1 | grep -E "FAILED|test result"      # 437 passed
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS, with `panini-prakriya` 437, `trace` 214, `paradigm` 27, `panini-data` 28.

- [ ] **Step 5: Commit**

```bash
mise run lint
git branch --show-current      # curadi-10j
git add -A
git commit -m "feat(sanadi): 6.1.54, 7.3.36 puk, Kaumudī 2567; 10.0493 and 6.4.92 moved

6.1.54 cisphuror ṇau gives √ci its optional A before ṇic, keyed on the
row (samjna::CISPHUR), since 10.0325 ci stores as the same code. 7.3.36
appends puk's p to an ā-final aṅga before 7.2.115, which then declines.
Kaumudī 2567 keeps a ṅit curādi ṇijanta ātmanepadī ahead of 1.3.12.
10.0493 moves to the head of the stage, where vidyut credits it, and
6.4.92 after the sanādi 6.1.78, so it reads √ci's long vowel. No curated
row reaches any of it yet."
```

---

## Task 3: The rows, their goldens, and every assertion they move

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini-prakriya/src/tinanta/sanadi.rs`, `crates/panini-prakriya/src/tinanta/samjna.rs` (tests only)
- Modify: `crates/panini/tests/paradigm/main.rs`, `crates/panini/tests/paradigm/data/curadi.rs`
- Modify: `crates/panini/tests/trace/curadi.rs`

**Interfaces:**
- Consumes: Task 2's rules and `CISPHUR`; `skip_nic` through `OPTIONAL_NIC`; the trace helpers `credited`, `rows_crediting`, `optional_nic_rows`.
- Produces: the eight `Dhatu` rows; `("10.0124", "2570")`; the trace helper `opens_nicless(ids: &[&str], id: &str) -> bool`; the test `curadi_analyses_its_ajanta_forms`.

- [ ] **Step 1: The assertions (failing)**

Create `/tmp/vidyut-full/slice10j/assertions_10j.py` if it is missing (sha256 `fe4bde6c72cddc32e9797911270a9bd8f88ea0602622f3bb35080dff57fbfdf6`). By file:
- **`panini-data`:** `JNAPADI`'s doc (all seven curated); `Dhatu::pada`'s census (430; 264 `Nic`, seven `NicUbhayapada`, 45 ākusmīya, √smiṅ's 2567; 181 optional-ṇic); the row count (430); the row-list test, renamed `curadi_rows_are_the_three_hundred_twenty_seven_curated_roots`, with a 10j paragraph and the eight tuples; `OPTIONAL_NIC.len()` 181; the marker test's ṅit arm (a curādi `Atmanepada` row is ṅit, Kaumudī 2567's) and its pin that `10.0058` is the one ṅit curādi row upstream; the jñapādi test's comment.
- **`sanadi.rs` / `samjna.rs` tests:** √ci joins `each_optional_nic_rule_takes_…` (2570's) and `an_optional_nic_rule_declines_off_its_rows`; `cisphur_is_the_ciy_row_6_1_54_names` asserts `10.0124` is curated.
- **`paradigm/main.rs`:** `VIKALPA_RULES` (+6.1.54); the census (26424 / 2936; ones 18648, twos 6432, threes 601, sixes 365, nines 8; ALTERNATES 10646; 8.4.56 / 7.1.35 / 7.1.35+8.4.56 646 / 638 / 638; 2570 216 and its stacks 12; the four 6.1.54 keys) and its 10j paragraph; the jñapādi witnesses (√cap's surfaces are homographs; `jYApayati` / `jYApayate` leave the non-forms); the āsvadīya witnesses (`cayati` is both √ci rows'; the opening id skips 10.0493; `cayayati` leaves the non-forms); and `curadi_analyses_its_ajanta_forms`.
- **`trace/curadi.rs`:** `opens_nicless` and its two callers; 2570's literal rows (+`10.0124`); ākusmīya 39 / 45 × 36, `Nic` 264, `NicUbhayapada` 7; the mit-rule credits 702 / 624; `rows_crediting`'s window (+2567); the 7.2.115 and sanādi 6.1.78 rosters; `the_10j_rules_fire_only_on_their_rows`.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10j, Task 3's assertions — every count, roster, list,
uniqueness and check() assertion the eight rows move, before the rows exist.
Every `old` must occur exactly once in its file; nothing is written if one
fails. Run from the worktree root, after Task 2."""
import sys
E = {
'crates/panini-data/src/lib.rs': [
("""/// `10.0124` (`ciY`), the roots the gaṇasūtra 10.0493 makes mit. Compared as
/// strings, like `AKUSMIYA`. The engine's `derive` tags a curādi root in this
/// range `Tag::Mit`, which 10.0493 and 6.4.92 *mitāṃ hrasvaḥ* read. Includes
/// `10.0124 ciY`, which is mit but not yet curated: 7.2.115 is in since slice
/// 10h, but √ci still waits on its own slice, for 6.1.54 and 7.3.36.
/// `jnapadi_is_exactly_the_rows_10_0493_names` pins the range to upstream.
pub const JNAPADI: RangeInclusive<&str> = "10.0118"..="10.0124";
""",
"""/// `10.0124` (`ciY`), the roots the gaṇasūtra 10.0493 makes mit. Compared as
/// strings, like `AKUSMIYA`. The engine's `derive` tags a curādi root in this
/// range `Tag::Mit`, which 10.0493 and 6.4.92 *mitāṃ hrasvaḥ* read. All seven
/// are curated; `10.0124 ciY`, the last, since slice 10j.
/// `jnapadi_is_exactly_the_rows_10_0493_names` pins the range to upstream.
pub const JNAPADI: RangeInclusive<&str> = "10.0118"..="10.0124";
"""),
("""    /// Which pada(s) this engine derives for this root. Curated rather than
    /// read from the upadeśa's it-markers — but no longer a *deferral*:
    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 422
    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and
    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed
    /// exception, 260 curādi rows' are 1.3.74's, six more 1.3.74's with ṇic
    /// and 1.3.72's without (`NicUbhayapada`), 43 ākusmīya rows'
    /// the gaṇasūtra 10.0496's and ten ā-garvīya rows' the gaṇasūtra
    /// 10.0497's, each asserted explicitly from both sides, the same way
    /// `dhatupatha_numbers_resolve_upstream` holds `code` to upstream. For
    /// the 180 curādi rows whose ṇic is optional this is the ṇic branch's
    /// pada; the test also re-derives their ṇic-less branch's from the
    /// upadeśa: 1.3.72's for the six `NicUbhayapada` rows, 1.3.78's for the
    /// rest.
    ///
""",
"""    /// Which pada(s) this engine derives for this root. Curated rather than
    /// read from the upadeśa's it-markers — but no longer a *deferral*:
    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 430
    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and
    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed
    /// exception, 264 curādi rows' are 1.3.74's, seven more 1.3.74's with ṇic
    /// and 1.3.72's without (`NicUbhayapada`), 45 ākusmīya rows'
    /// the gaṇasūtra 10.0496's, ten ā-garvīya rows' the gaṇasūtra
    /// 10.0497's and `10.0058 zmiN`'s Kaumudī 2567's and 1.3.12's, each
    /// asserted explicitly from both sides, the same way
    /// `dhatupatha_numbers_resolve_upstream` holds `code` to upstream. For
    /// the 181 curādi rows whose ṇic is optional this is the ṇic branch's
    /// pada; the test also re-derives their ṇic-less branch's from the
    /// upadeśa: 1.3.72's for the seven `NicUbhayapada` rows, 1.3.78's for the
    /// rest.
    ///
"""),
("""    /// `docs/superpowers/specs/2026-08-16-pada-audit-design.md`.
    ///
    /// The test covers the 422 roots curated here, not the dhātupāṭha's 2259.
    /// It catches a mis-assigned pada on a root a future slice adds; it does
    /// not make the table self-maintaining.
""",
"""    /// `docs/superpowers/specs/2026-08-16-pada-audit-design.md`.
    ///
    /// The test covers the 430 roots curated here, not the dhātupāṭha's 2259.
    /// It catches a mis-assigned pada on a root a future slice adds; it does
    /// not make the table self-maintaining.
"""),
("""    #[test]
    fn curated_roots_have_expected_ganas_and_padas() {
        assert_eq!(dhatus().len(), 422);
        let bu = dhatus().iter().find(|d| d.dhatupatha == "01.0001").unwrap();
        assert!(matches!(bu.pada, PadaAssignment::Parasmaipada));
""",
"""    #[test]
    fn curated_roots_have_expected_ganas_and_padas() {
        assert_eq!(dhatus().len(), 430);
        let bu = dhatus().iter().find(|d| d.dhatupatha == "01.0001").unwrap();
        assert!(matches!(bu.pada, PadaAssignment::Parasmaipada));
"""),
("""
    #[test]
    fn curadi_rows_are_the_three_hundred_nineteen_curated_roots() {
        // Slice 10a opens gaṇa 10 with four roots that need only ṇic
        // (3.1.25), 3.1.32 and the guṇa/vṛddhi before ṇic: √cur (7.3.86),
""",
"""
    #[test]
    fn curadi_rows_are_the_three_hundred_twenty_seven_curated_roots() {
        // Slice 10a opens gaṇa 10 with four roots that need only ṇic
        // (3.1.25), 3.1.32 and the guṇa/vṛddhi before ṇic: √cur (7.3.86),
"""),
("""        // `NicUbhayapada`, ubhayapadī by 1.3.72 without it too. Slice 10i adds
        // the fifty-nine āsvadīya (10.0499), `10.0022 pF` (Kaumudī 2565) and
        // `10.0251 Guzi~r` (2571), ubhayapadī by 1.3.74 with ṇic. The gaṇa is
        // OPEN at 319 of its 509 dhātupāṭha rows.
        let rows: Vec<_> = dhatus()
            .iter()
""",
"""        // `NicUbhayapada`, ubhayapadī by 1.3.72 without it too. Slice 10i adds
        // the fifty-nine āsvadīya (10.0499), `10.0022 pF` (Kaumudī 2565) and
        // `10.0251 Guzi~r` (2571), ubhayapadī by 1.3.74 with ṇic. Slice 10j
        // adds the eight ajanta rows 7.2.115, 6.1.54 and 7.3.36 reach: √ghṛ,
        // √jñā, √cyu and √bhū, ubhayapadī by 1.3.74; √gṛ and √yu, the last two
        // ākusmīya, by 10.0496; √ci, the last jñapādi, `NicUbhayapada` with
        // ṇic optional by 2570; and √smiṅ, ātmanepadī under ṇic by Kaumudī
        // 2567. The gaṇa is OPEN at 327 of its 509 dhātupāṭha rows.
        let rows: Vec<_> = dhatus()
            .iter()
"""),
("""                ("10.0336", "svad", PadaAssignment::Nic),
                ("10.0337", "svAd", PadaAssignment::Nic),
            ]
        );
""",
"""                ("10.0336", "svad", PadaAssignment::Nic),
                ("10.0337", "svAd", PadaAssignment::Nic),
                ("10.0058", "smi", PadaAssignment::Atmanepada),
                ("10.0124", "ci", PadaAssignment::NicUbhayapada),
                ("10.0152", "Gf", PadaAssignment::Nic),
                ("10.0231", "gf", PadaAssignment::Akusmiya),
                ("10.0235", "yu", PadaAssignment::Akusmiya),
                ("10.0258", "jYA", PadaAssignment::Nic),
                ("10.0275", "cyu", PadaAssignment::Nic),
                ("10.0277", "BU", PadaAssignment::Nic),
            ]
        );
"""),
("""            );
        }
        assert_eq!(OPTIONAL_NIC.len(), 180);
        // `10.0368 za\\da~` stays out: the reading above makes it 10.0498's,
        // but vidyut derives it with the upasarga ā, which this engine does
""",
"""            );
        }
        assert_eq!(OPTIONAL_NIC.len(), 181);
        // `10.0368 za\\da~` stays out: the reading above makes it 10.0498's,
        // but vidyut derives it with the upasarga ā, which this engine does
"""),
("""        // The gaṇasūtra 10.0493 follows `10.0124 ciY` and makes the seven
        // rows from `10.0118 jYapa~` mit. vidyut-prakriya lists the same
        // seven upadeśas (`JNAP_ADI`). `ciY` is not curated, so this test —
        // not a derivation — is what holds the range's upper end.
        let rows = upstream_rows();
        let in_range: Vec<&str> = rows
""",
"""        // The gaṇasūtra 10.0493 follows `10.0124 ciY` and makes the seven
        // rows from `10.0118 jYapa~` mit. vidyut-prakriya lists the same
        // seven upadeśas (`JNAP_ADI`). All seven are curated since slice 10j,
        // so derivations credit 10.0493 at both ends of the range; this test
        // holds the range itself to upstream.
        let rows = upstream_rows();
        let in_range: Vec<&str> = rows
"""),
("""            // only where its ṇic is optional and the marker is svarita or ñit:
            // its ṇic-less branch is 1.3.72's (`NicUbhayapada`, slice 10h's six
            // ādhṛṣīya). Any other marked curādi row (`10.0058 zmiN`, ṅit) is a
            // later slice's, and fails the first assertion until that slice
            // decides how 1.3.12 meets ṇic.
            if d.gana == Gana::Curadi {
                let nicless_ubhaya =
                    optional_nic(d.dhatupatha).is_some() && derived == PadaAssignment::Ubhayapada;
                if !nicless_ubhaya {
                    assert_eq!(
                        derived,
""",
"""            // only where its ṇic is optional and the marker is svarita or ñit:
            // its ṇic-less branch is 1.3.72's (`NicUbhayapada`, slice 10h's six
            // ādhṛṣīya, and √ci). One marked row decides its ṇic branch too: a
            // ṅit row stays ātmanepadī under ṇic by Kaumudī 2567, and 1.3.12
            // credits (`10.0058 zmiN`, the one ṅit curādi row upstream). The
            // check reads the ṅ, not the verdict: an anudāttet row (`za\\da~`)
            // marks ātmanepada too, but takes 1.3.74 under ṇic. Any other
            // marked curādi row fails the first assertion until a slice
            // decides its pada.
            if d.gana == Gana::Curadi {
                let ngit = upadesha.trim_end_matches(['\\\\', '^']).ends_with('N');
                let nicless_ubhaya =
                    optional_nic(d.dhatupatha).is_some() && derived == PadaAssignment::Ubhayapada;
                if ngit {
                    assert_eq!(
                        derived,
                        PadaAssignment::Atmanepada,
                        "{} {upadesha}: ṅit, so 1.3.12's",
                        d.dhatupatha
                    );
                } else if !nicless_ubhaya {
                    assert_eq!(
                        derived,
"""),
("""                } else if AA_GARVIYA.contains(&d.dhatupatha) {
                    (PadaAssignment::AaGarviya, "ā-garvīya, so 10.0497's")
                } else if nicless_ubhaya {
                    (
""",
"""                } else if AA_GARVIYA.contains(&d.dhatupatha) {
                    (PadaAssignment::AaGarviya, "ā-garvīya, so 10.0497's")
                } else if ngit {
                    (PadaAssignment::Atmanepada, "ṅit, so Kaumudī 2567's")
                } else if nicless_ubhaya {
                    (
"""),
("""            wrong.join("\\n  ")
        );
    }

""",
"""            wrong.join("\\n  ")
        );
        // The ṅit arm above reaches one row: `10.0058 zmiN` is the one ṅit
        // curādi row upstream.
        let ngit_curadi: Vec<&str> = rows
            .iter()
            .filter(|(n, u, _)| {
                n.starts_with("10.") && u.trim_end_matches(['\\\\', '^']).ends_with('N')
            })
            .map(|(n, _, _)| *n)
            .collect();
        assert_eq!(ngit_curadi, ["10.0058"]);
    }

"""),
],
'crates/panini-prakriya/src/tinanta/sanadi.rs': [
("""            ("2565", "10.0022", "pF", Tag::Nic),
            ("2570", "10.0230", "div", Tag::Akusmiya),
            ("2571", "10.0251", "Guz", Tag::Nic),
            ("2573.1", "10.0400", "pata", Tag::Nic),
""",
"""            ("2565", "10.0022", "pF", Tag::Nic),
            ("2570", "10.0230", "div", Tag::Akusmiya),
            // √ci, the one mit row whose ṇic is optional.
            ("2570", "10.0124", "ci", Tag::Nic),
            ("2571", "10.0251", "Guz", Tag::Nic),
            ("2573.1", "10.0400", "pata", Tag::Nic),
"""),
("""                ("10.0022", "pF", Tag::Nic),
                ("10.0251", "Guz", Tag::Nic),
                ("10.0001", "cur", Tag::Nic),
                ("10.0192", "cit", Tag::Akusmiya),
""",
"""                ("10.0022", "pF", Tag::Nic),
                ("10.0251", "Guz", Tag::Nic),
                ("10.0124", "ci", Tag::Nic),
                ("10.0001", "cur", Tag::Nic),
                ("10.0192", "cit", Tag::Akusmiya),
"""),
],
'crates/panini-prakriya/src/tinanta/samjna.rs': [
("""        assert_eq!(CISPHUR, ["10.0124"]);
        assert_eq!(upstream_upadesha("10.0124"), Some("ciY"));
        assert_eq!(upstream_upadesha("10.0325"), Some("ci"));
        for (number, upadesha) in [("05.0005", "ci\\\\Y"), ("06.0121", "sPura~")] {
""",
"""        assert_eq!(CISPHUR, ["10.0124"]);
        assert_eq!(upstream_upadesha("10.0124"), Some("ciY"));
        assert!(dhatus().iter().any(|d| d.dhatupatha == "10.0124"));
        assert_eq!(upstream_upadesha("10.0325"), Some("ci"));
        for (number, upadesha) in [("05.0005", "ci\\\\Y"), ("06.0121", "sPura~")] {
"""),
],
'crates/panini/tests/paradigm/main.rs': [
("""/// 6.4.116, 6.4.115 and 6.4.43 right after 7.3.86, not last as they sit
/// here.
const VIKALPA_RULES: &[&str] = &[
    "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3", "2573.2", "7.3.37.2",
    "7.1.35", "3.4.111", "7.3.86", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56", "6.4.115",
    "6.4.117", "6.4.116", "6.4.43",
];

/// `ALTERNATES` is otherwise 10460 bare strings, and a string can be right for
/// the wrong reason — `BavatAt` is a real form whether or not 8.4.56 is what
/// produced it. This ties each row to the grammar: find the branch that
/// derives the row's form, intersect its log with the optional-rule set, and
""",
"""/// 6.4.116, 6.4.115 and 6.4.43 right after 7.3.86, not last as they sit
/// here.
const VIKALPA_RULES: &[&str] = &[
    "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3", "2573.2", "6.1.54",
    "7.3.37.2", "7.1.35", "3.4.111", "7.3.86", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56",
    "6.4.115", "6.4.117", "6.4.116", "6.4.43",
];

/// `ALTERNATES` is otherwise 10646 bare strings, and a string can be right for
/// the wrong reason — `BavatAt` is a real form whether or not 8.4.56 is what
/// produced it. This ties each row to the grammar: find the branch that
/// derives the row's form, intersect its log with the optional-rule set, and
"""),
("""/// cell adds the ṇic-less reading beside it — for √dhūp and √vich, 3.1.28's
/// āya stem (*dhūpāyati*, *vicchāyati*). 4392 new cells, 2928 new rows. The
/// gaṇa is OPEN at 319 of its 509 rows.
/// This test is what keeps the numbers true day to day.
#[test]
fn derivation_set_shape_matches_the_audited_numbers() {
    let total_cells = PARADIGM.len() * 9;
    assert_eq!(total_cells, 25956, "2884 root×lakāra blocks × 9 cells each");

    let mut ones = 0usize;
    let mut twos = 0usize;
""",
"""/// cell adds the ṇic-less reading beside it — for √dhūp and √vich, 3.1.28's
/// āya stem (*dhūpāyati*, *vicchāyati*). 4392 new cells, 2928 new rows. The
/// gaṇa is OPEN at 319 of its 509 rows.
///
/// Slice 10j curates the eight ajanta rows 7.2.115, 6.1.54 and 7.3.36 reach:
/// √ghṛ, √jñā, √cyu and √bhū (`Nic`), which fork exactly where √cur does;
/// √gṛ and √yu, ākusmīya, and √smiṅ, ātmanepadī under ṇic by Kaumudī 2567,
/// one form per cell; and √ci (`NicUbhayapada`), whose every cell holds
/// three readings: the ṇic branch declined (*cayayati*, the pinned form),
/// 6.1.54's (*capayati*) and 2570's ṇic-less one (*cayati*), in both padas.
/// 468 new cells, 186 new rows. The gaṇa is OPEN at 327 of its 509 rows.
/// This test is what keeps the numbers true day to day.
#[test]
fn derivation_set_shape_matches_the_audited_numbers() {
    let total_cells = PARADIGM.len() * 9;
    assert_eq!(total_cells, 26424, "2936 root×lakāra blocks × 9 cells each");

    let mut ones = 0usize;
    let mut twos = 0usize;
"""),
("""            }
        }
    }
    assert_eq!(ones, 18268, "one-form cells");
    assert_eq!(twos, 6424, "two-form cells");
    assert_eq!(
        threes, 525,
        "three-form cells — new in slice 3b — √hrī's loṭ prathama and madhyama eka, each by \\
         7.1.35/8.4.56; and — new in slice 3c — √dā's and √dhā's, the same way; and — new in \\
         slice 3c2 — √gā's, the same way; and — new in slice 3d — the six ṛ-roots', the same way; \\
""",
"""            }
        }
    }
    assert_eq!(ones, 18648, "one-form cells");
    assert_eq!(twos, 6432, "two-form cells");
    assert_eq!(
        threes, 601,
        "three-form cells — new in slice 3b — √hrī's loṭ prathama and madhyama eka, each by \\
         7.1.35/8.4.56; and — new in slice 3c — √dā's and √dhā's, the same way; and — new in \\
         slice 3c2 — √gā's, the same way; and — new in slice 3d — the six ṛ-roots', the same way; \\
"""),
("""         in slice 10e — the eighty-three ubhayapadī adanta roots', the same way; and — new in slice \\
         10f — the optional-ṇic rows' (2564/2570/2573.x beside 7.1.35/8.4.56); and — new in \\
         slice 10h — √dhū's and √prī's every cell that forks on neither 7.1.35 nor 8.4.56 \\
         (10.0498 and 7.3.37.2 beside the ṇic form)"
    );
    assert_eq!(
        fours, 359,
""",
"""         in slice 10e — the eighty-three ubhayapadī adanta roots', the same way; and — new in slice \\
         10f — the optional-ṇic rows' (2564/2570/2573.x beside 7.1.35/8.4.56); and — new in \\
         slice 10h — √dhū's and √prī's every cell that forks on neither 7.1.35 nor 8.4.56 \\
         (10.0498 and 7.3.37.2 beside the ṇic form); and — new in slice 10j — √ghṛ's, √jñā's, \\
         √cyu's and √bhū's two loṭ tātaṅ cells, by 7.1.35/8.4.56, and √ci's every cell that \\
         forks on neither (6.1.54 and 2570 beside the ṇic form)"
    );
    assert_eq!(
        fours, 359,
"""),
("""         prathama eka, forking on 7.1.35/6.4.116/8.4.56"
    );
    assert_eq!(
        sixes, 363,
        "six-form cells — kft loṭ madhyama eka, ruD loṭ parasmaipada madhyama eka, Bid, kzud \\
         and tfd's loṭ parasmaipada madhyama eka, und's (slice 7d), Cid's and Cfd's loṭ \\
         parasmaipada madhyama eka (slice 7f), and — new in slice 8a — kziR, fR, tfR and GfR's \\
""",
"""         prathama eka, forking on 7.1.35/6.4.116/8.4.56"
    );
    assert_eq!(
        sixes, 365,
        "six-form cells — kft loṭ madhyama eka, ruD loṭ parasmaipada madhyama eka, Bid, kzud \\
         and tfd's loṭ parasmaipada madhyama eka, und's (slice 7d), Cid's and Cfd's loṭ \\
         parasmaipada madhyama eka (slice 7f), and — new in slice 8a — kziR, fR, tfR and GfR's \\
"""),
("""         slice 10h — forty-eight ādhṛṣīya rows' the same cells (10.0498 beside 7.1.35/8.4.56), \\
         and √dhū's and √prī's laṅ and vidhiliṅ parasmaipada prathama eka (10.0498 and \\
         7.3.37.2 beside 8.4.56); and — new in slice 10i — the sixty-one rows' loṭ \\
         parasmaipada prathama and madhyama eka (10.0499/2565/2571 beside 7.1.35/8.4.56)"
    );
    assert_eq!(
        sevens, 1,
""",
"""         slice 10h — forty-eight ādhṛṣīya rows' the same cells (10.0498 beside 7.1.35/8.4.56), \\
         and √dhū's and √prī's laṅ and vidhiliṅ parasmaipada prathama eka (10.0498 and \\
         7.3.37.2 beside 8.4.56); and — new in slice 10i — the sixty-one rows' loṭ \\
         parasmaipada prathama and madhyama eka (10.0499/2565/2571 beside 7.1.35/8.4.56); and \\
         — new in slice 10j — √ci's laṅ and vidhiliṅ parasmaipada prathama eka (6.1.54 and 2570 \\
         beside 8.4.56)"
    );
    assert_eq!(
        sevens, 1,
"""),
("""    );

    assert_eq!(
        nines, 6,
        "nine-form cells — new in slice 10f, the engine's record: pata's loṭ parasmaipada \\
         prathama and madhyama eka, three readings (2573.1 ṇic-less, 2573.2 pAta-, 6.4.48 \\
         pata-) × the tātaṅ triple (7.1.35/8.4.56); and — new in slice 10h — √dhū's and \\
         √prī's, three readings (10.0498 ṇic-less, 7.3.37.2's nuk, 7.2.115's vṛddhi) × the \\
         same triple"
    );

    assert_eq!(ALTERNATES.len(), 10460, "ALTERNATES row count");
    let key_count = |key: &str| {
        ALTERNATES
            .iter()
            .filter(|(_, _, _, _, _, k)| *k == key)
            .count()
    };
    assert_eq!(key_count("8.4.56"), 636, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 628, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 628, "7.1.35+8.4.56 alternates");
    assert_eq!(key_count("3.4.111"), 2, "3.4.111 alternates");
    assert_eq!(key_count("6.4.107"), 72, "6.4.107 alternates");
    assert_eq!(key_count("8.4.65"), 145, "8.4.65-only alternates");
""",
"""    );

    assert_eq!(
        nines, 8,
        "nine-form cells — new in slice 10f, the engine's record: pata's loṭ parasmaipada \\
         prathama and madhyama eka, three readings (2573.1 ṇic-less, 2573.2 pAta-, 6.4.48 \\
         pata-) × the tātaṅ triple (7.1.35/8.4.56); and — new in slice 10h — √dhū's and \\
         √prī's, three readings (10.0498 ṇic-less, 7.3.37.2's nuk, 7.2.115's vṛddhi) × the \\
         same triple; and — new in slice 10j — √ci's, three readings (2570 ṇic-less, 6.1.54's \\
         `cA` with 7.3.36's puk, 7.2.115's vṛddhi) × the same triple"
    );

    assert_eq!(ALTERNATES.len(), 10646, "ALTERNATES row count");
    let key_count = |key: &str| {
        ALTERNATES
            .iter()
            .filter(|(_, _, _, _, _, k)| *k == key)
            .count()
    };
    assert_eq!(key_count("8.4.56"), 646, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 638, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 638, "7.1.35+8.4.56 alternates");
    assert_eq!(key_count("3.4.111"), 2, "3.4.111 alternates");
    assert_eq!(key_count("6.4.107"), 72, "6.4.107 alternates");
    assert_eq!(key_count("8.4.65"), 145, "8.4.65-only alternates");
"""),
("""    );
    // Slice 10f's five Kaumudī vikalpa ids, alone and stacked, with slice
    // 10g's fifty-nine rows: 2564 alone, 2570 alone and 2570+7.3.86 are 10g's.
    for (key, n) in [
        ("2564", 1908),
        ("2570", 144),
        ("2570+7.3.86", 72),
        ("2573.1", 36),
        ("2573.2", 72),
""",
"""    );
    // Slice 10f's five Kaumudī vikalpa ids, alone and stacked, with slice
    // 10g's fifty-nine rows: 2564 alone, 2570 alone and 2570+7.3.86 are 10g's.
    // Slice 10j's √ci adds 72 to 2570 alone, its ṇic-less branch in both
    // padas, and two to each of 2570's 8.4.56 and 7.1.35 stacks.
    for (key, n) in [
        ("2564", 1908),
        ("2570", 216),
        ("2570+7.3.86", 72),
        ("2573.1", 36),
        ("2573.2", 72),
"""),
("""        ("2573.3+8.4.56", 6),
        ("2573.3+7.1.35", 6),
        ("2573.3+7.1.35+8.4.56", 6),
        ("2570+8.4.56", 10),
        ("2570+7.1.35", 10),
        ("2570+7.1.35+8.4.56", 10),
        ("2570+7.3.86+8.4.56", 6),
        ("2570+7.1.35+7.3.86", 6),
        ("2570+7.1.35+7.3.86+8.4.56", 6),
""",
"""        ("2573.3+8.4.56", 6),
        ("2573.3+7.1.35", 6),
        ("2573.3+7.1.35+8.4.56", 6),
        ("2570+8.4.56", 12),
        ("2570+7.1.35", 12),
        ("2570+7.1.35+8.4.56", 12),
        ("2570+7.3.86+8.4.56", 6),
        ("2570+7.1.35+7.3.86", 6),
        ("2570+7.1.35+7.3.86+8.4.56", 6),
"""),
("""        ("2571+7.3.86+8.4.56", 2),
        ("2571+7.1.35+7.3.86", 2),
        ("2571+7.1.35+7.3.86+8.4.56", 2),
    ] {
        assert_eq!(key_count(key), n, "{key} alternates");
    }
""",
"""        ("2571+7.3.86+8.4.56", 2),
        ("2571+7.1.35+7.3.86", 2),
        ("2571+7.1.35+7.3.86+8.4.56", 2),
    ] {
        assert_eq!(key_count(key), n, "{key} alternates");
    }
    // Slice 10j's vikalpa id, alone and stacked: 6.1.54 on √ci's ṇic branch,
    // in both padas.
    for (key, n) in [
        ("6.1.54", 72),
        ("6.1.54+8.4.56", 2),
        ("6.1.54+7.1.35", 2),
        ("6.1.54+7.1.35+8.4.56", 2),
    ] {
        assert_eq!(key_count(key), n, "{key} alternates");
    }
"""),
("""/// root and pada and crediting 10.0493 first, then 7.2.116, then
/// 6.4.92 — except that since slice 10e √cah's and √rah's surfaces are
/// shared with the adanta homographs `10.0405 caha` and `10.0396 raha`
/// (see `curadi_analyses_its_adanta_forms`): two analyses, of which the mit
/// one is checked here. `ajYapayata` is pada-ambiguous within √jñap (laṅ parasmaipada
/// madhyama bahu = ātmanepada prathama eka): two analyses, one per pada, both
/// mit. The 7.2.116-only shapes (`jYApayati`, …) — what this engine derived
/// before 6.4.92 — derive nothing.
#[test]
fn curadi_analyses_its_jnapadi_forms() {
    let engine = Panini::new();
""",
"""/// root and pada and crediting 10.0493 first, then 7.2.116, then
/// 6.4.92 — except that since slice 10e √cah's and √rah's surfaces are
/// shared with the adanta homographs `10.0405 caha` and `10.0396 raha`
/// (see `curadi_analyses_its_adanta_forms`), and since slice 10j √cap's with
/// √ci's 6.1.54 branch (`10.0124`, see `curadi_analyses_its_ajanta_forms`):
/// two analyses, of which the √cap one, 7.2.116's, is checked here.
/// `ajYapayata` is pada-ambiguous within √jñap (laṅ parasmaipada
/// madhyama bahu = ātmanepada prathama eka): two analyses, one per pada, both
/// mit. The 7.2.116-only shapes (`yAmayati`, …) — what this engine derived
/// before 6.4.92 — derive nothing; √jñap's, `jYApayati`, is √jñā's
/// (`10.0258`, 7.3.36) since slice 10j.
#[test]
fn curadi_analyses_its_jnapadi_forms() {
    let engine = Panini::new();
"""),
("""        for (form, pada) in [(parasmai, Pada::Parasmaipada), (atmane, Pada::Atmanepada)] {
            let r = engine.check(form);
            assert!(matches!(r.verdict, Verdict::Valid), "{form}");
            let homograph = matches!(dhatu, "cah" | "rah");
            assert_eq!(r.analyses.len(), if homograph { 2 } else { 1 }, "{form}");
            let a = r.analyses.iter().find(|a| a.dhatu == dhatu).unwrap();
            assert_eq!(a.pada, pada, "{form}");
""",
"""        for (form, pada) in [(parasmai, Pada::Parasmaipada), (atmane, Pada::Atmanepada)] {
            let r = engine.check(form);
            assert!(matches!(r.verdict, Verdict::Valid), "{form}");
            let homograph = matches!(dhatu, "cah" | "rah" | "cap");
            assert_eq!(r.analyses.len(), if homograph { 2 } else { 1 }, "{form}");
            let a = r.analyses.iter().find(|a| a.dhatu == dhatu).unwrap();
            assert_eq!(a.pada, pada, "{form}");
"""),
("""        assert_eq!(a.dhatu, "jYap");
        assert_mit("ajYapayata", &ids_of(a));
    }
    for form in [
        "jYApayati",
        "yAmayati",
        "cAhayati",
        "cApayati",
        "rAhayati",
        "bAlayati",
        "jYApayate",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
""",
"""        assert_eq!(a.dhatu, "jYap");
        assert_mit("ajYapayata", &ids_of(a));
    }
    for form in ["yAmayati", "cAhayati", "cApayati", "rAhayati", "bAlayati"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
"""),
("""            &[],
            &["7.2.115", "6.1.78"],
        ),
        ("cayati", &["ci"], Pada::Parasmaipada, &["10.0499"], &[]),
        // The īdit `pUrI~` and the udit `vftu~` are 10.0499's, not 2572's
        // or 2570's.
        ("pUrati", &["pUr"], Pada::Parasmaipada, &["10.0499"], &[]),
""",
"""            &[],
            &["7.2.115", "6.1.78"],
        ),
        // √ci (`10.0124`, slice 10j) shares the ṇic-less `cayati`, 2570's.
        (
            "cayati",
            &["ci", "ci"],
            Pada::Parasmaipada,
            &["10.0499", "2570"],
            &[],
        ),
        // The īdit `pUrI~` and the udit `vftu~` are 10.0499's, not 2572's
        // or 2570's.
        ("pUrati", &["pUr"], Pada::Parasmaipada, &["10.0499"], &[]),
"""),
("""            }
        }
        if !opens.is_empty() {
            let mut opened: Vec<String> = r.analyses.iter().map(|a| ids_of(a)[0].clone()).collect();
            opened.sort_unstable();
            let mut want = opens.to_vec();
            want.sort_unstable();
""",
"""            }
        }
        if !opens.is_empty() {
            // The id each analysis opens with, after 10.0493 on a mit row.
            let mut opened: Vec<String> = r
                .analyses
                .iter()
                .map(|a| ids_of(a).into_iter().find(|i| i != "10.0493").unwrap())
                .collect();
            opened.sort_unstable();
            let mut want = opens.to_vec();
            want.sort_unstable();
"""),
("""    }
    for form in [
        "grasate", "parate", "Gozate", "DUpati", "vicCati", "DUpAyate", "veCayati", "jayayati",
        "cayayati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
""",
"""    }
    for form in [
        "grasate", "parate", "Gozate", "DUpati", "vicCati", "DUpAyate", "veCayati", "jayayati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}

/// Slice 10j's `check()` witnesses, from its spec's tables: each row's laṭ
/// prathama eka in every pada it admits, √ci's three readings in each, the
/// parasmaipada `smi`, `gf` and `yu` block, and every homograph the spec
/// names. The goldens were grepped for every witness first: a single-root
/// witness is its own row's alone, and a homograph witness is exactly its
/// rows', one analysis each. The other row came first in the table
/// (`capayati`'s √cap, `cayati`'s āsvadīya `10.0325 ci`, `BAvayati`'s
/// ādhṛṣīya `10.0382 BU`), so its analysis comes first and `trace_for` keeps
/// answering with it; that order is pinned here, and the slice's row's
/// analysis, the last, must credit the listed ids in order. The shapes the
/// slice rules out derive nothing.
#[test]
fn curadi_analyses_its_ajanta_forms() {
    let engine = Panini::new();
    // (form, its roots in analysis order, pada, the ids the slice's row's
    // analysis credits, in order)
    for (form, dhatus, pada, credits) in [
        (
            "smAyayate",
            &["smi"][..],
            Pada::Atmanepada,
            &["3.1.25", "7.2.115", "6.1.78", "3.1.32", "2567", "1.3.12"][..],
        ),
        (
            "cayayati",
            &["ci"],
            Pada::Parasmaipada,
            &["10.0493", "3.1.25", "7.2.115", "6.1.78", "6.4.92", "1.3.78"],
        ),
        (
            "cayayate",
            &["ci"],
            Pada::Atmanepada,
            &["10.0493", "3.1.25", "7.2.115", "6.1.78", "6.4.92", "1.3.74"],
        ),
        (
            "capayati",
            &["cap", "ci"],
            Pada::Parasmaipada,
            &["10.0493", "3.1.25", "6.1.54", "7.3.36", "6.4.92", "1.3.78"],
        ),
        (
            "capayate",
            &["cap", "ci"],
            Pada::Atmanepada,
            &["10.0493", "3.1.25", "6.1.54", "7.3.36", "6.4.92", "1.3.74"],
        ),
        (
            "cayati",
            &["ci", "ci"],
            Pada::Parasmaipada,
            &["10.0493", "2570", "1.3.78"],
        ),
        (
            "cayate",
            &["ci"],
            Pada::Atmanepada,
            &["10.0493", "2570", "1.3.72"],
        ),
        (
            "GArayati",
            &["Gf"],
            Pada::Parasmaipada,
            &["7.2.115", "1.3.78"],
        ),
        (
            "GArayate",
            &["Gf"],
            Pada::Atmanepada,
            &["7.2.115", "1.3.74"],
        ),
        (
            "gArayate",
            &["gf"],
            Pada::Atmanepada,
            &["10.0496", "3.1.25", "7.2.115"],
        ),
        (
            "yAvayate",
            &["yu"],
            Pada::Atmanepada,
            &["10.0496", "3.1.25", "7.2.115", "6.1.78"],
        ),
        (
            "jYApayati",
            &["jYA"],
            Pada::Parasmaipada,
            &["3.1.25", "7.3.36", "1.3.78"],
        ),
        (
            "jYApayate",
            &["jYA"],
            Pada::Atmanepada,
            &["3.1.25", "7.3.36", "1.3.74"],
        ),
        (
            "cyAvayati",
            &["cyu"],
            Pada::Parasmaipada,
            &["7.2.115", "6.1.78", "1.3.78"],
        ),
        (
            "cyAvayate",
            &["cyu"],
            Pada::Atmanepada,
            &["7.2.115", "6.1.78", "1.3.74"],
        ),
        (
            "BAvayati",
            &["BU", "BU"],
            Pada::Parasmaipada,
            &["7.2.115", "6.1.78", "1.3.78"],
        ),
        (
            "BAvayate",
            &["BU", "BU"],
            Pada::Atmanepada,
            &["7.2.115", "6.1.78", "1.3.74"],
        ),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        let got: Vec<&str> = r.analyses.iter().map(|a| a.dhatu.as_str()).collect();
        assert_eq!(got, dhatus, "{form}");
        for a in &r.analyses {
            assert_eq!(a.pada, pada, "{form}");
        }
        let ids: Vec<&str> = r
            .analyses
            .last()
            .unwrap()
            .trace
            .iter()
            .map(|s| s.sutra.as_str())
            .collect();
        let mut at = 0;
        for id in credits {
            let next = ids[at..].iter().position(|i| i == id);
            assert!(next.is_some(), "{form} {id} out of order: {ids:?}");
            at += next.unwrap() + 1;
        }
        // Only √ci's 6.1.54 branch and √jñā take puk, and neither takes
        // 7.2.115 then.
        if credits.contains(&"7.3.36") {
            assert!(!ids.contains(&"7.2.115"), "{form}: {ids:?}");
        }
    }
    // The parasmaipada of an ātmanepadī root (√smiṅ by 2567 and 1.3.12, √gṛ
    // and √yu by 10.0496), guṇa where 7.2.115 gives vṛddhi, √ci's 6.1.54
    // branch without 6.4.92, and √jñā without its puk.
    for form in [
        "smAyayati",
        "gArayati",
        "yAvayati",
        "smayayate",
        "Garayati",
        "cyavayati",
        "cApayati",
        "jYAyayati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
"""),
],
'crates/panini/tests/trace/curadi.rs': [
("""                && optional_nic(d.dhatupatha).is_none()
        })
        .collect();
    assert_eq!(rows.len(), 37, "curated ākusmīya rows with ṇic");
    for d in rows {
        let number = d.dhatupatha;
        for lakara in [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin] {
""",
"""                && optional_nic(d.dhatupatha).is_none()
        })
        .collect();
    assert_eq!(rows.len(), 39, "curated ākusmīya rows with ṇic");
    for d in rows {
        let number = d.dhatupatha;
        for lakara in [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin] {
"""),
("""
#[test]
fn a_kusmad_is_credited_on_exactly_the_akusmiya_cells() {
    // 10.0496 fires on every ātmanepada cell of the 43 curated ākusmīya
    // rows — 43 roots × 4 lakāras × 9 cells, one branch each, the six
    // optional-ṇic rows' on their ṇic branch — and nowhere else: every
    // credit's number lies in the positional `AKUSMIYA` range. And 1.3.74
    // never reaches them: its credits stay on the 260 `Nic` rows and the six
    // `NicUbhayapada` rows' ṇic branch, read from the curated `pada` column
    // (ten before slice 10e, listed literally until then).
    let hits = credited("10.0496");
    assert_eq!(hits.len(), 43 * 36);
    for (number, _) in &hits {
        assert!(AKUSMIYA.contains(number), "10.0496 credited on {number}");
    }
""",
"""
#[test]
fn a_kusmad_is_credited_on_exactly_the_akusmiya_cells() {
    // 10.0496 fires on every ātmanepada cell of the 45 curated ākusmīya
    // rows (the whole range, since slice 10j) — 45 roots × 4 lakāras × 9
    // cells, one branch each, the six optional-ṇic rows' on their ṇic branch
    // — and nowhere else: every credit's number lies in the positional
    // `AKUSMIYA` range. And 1.3.74 never reaches them: its credits stay on the
    // 264 `Nic` rows and the seven `NicUbhayapada` rows' ṇic branch, read from
    // the curated `pada` column (ten before slice 10e, listed literally until
    // then).
    let hits = credited("10.0496");
    assert_eq!(hits.len(), 45 * 36);
    for (number, _) in &hits {
        assert!(AKUSMIYA.contains(number), "10.0496 credited on {number}");
    }
"""),
("""        .filter(|d| d.gana == Gana::Curadi && d.pada == PadaAssignment::Nic)
        .map(|d| d.dhatupatha)
        .collect();
    assert_eq!(nic.len(), 260, "curated 1.3.74 rows");
    let nic_ubhayapada: Vec<&str> = dhatus()
        .iter()
        .filter(|d| d.pada == PadaAssignment::NicUbhayapada)
        .map(|d| d.dhatupatha)
        .collect();
    assert_eq!(nic_ubhayapada.len(), 6, "curated 1.3.74-and-1.3.72 rows");
    for (number, _) in credited("1.3.74") {
        assert!(
            nic.contains(&number) || nic_ubhayapada.contains(&number),
""",
"""        .filter(|d| d.gana == Gana::Curadi && d.pada == PadaAssignment::Nic)
        .map(|d| d.dhatupatha)
        .collect();
    assert_eq!(nic.len(), 264, "curated 1.3.74 rows");
    let nic_ubhayapada: Vec<&str> = dhatus()
        .iter()
        .filter(|d| d.pada == PadaAssignment::NicUbhayapada)
        .map(|d| d.dhatupatha)
        .collect();
    assert_eq!(nic_ubhayapada.len(), 7, "curated 1.3.74-and-1.3.72 rows");
    for (number, _) in credited("1.3.74") {
        assert!(
            nic.contains(&number) || nic_ubhayapada.contains(&number),
"""),
("""
#[test]
fn the_mit_rules_are_credited_on_exactly_the_jnapadi_cells() {
    // 10.0493 and 6.4.92 fire on every branch of the six curated jñapādi
    // rows — 42 parasmaipada and 36 ātmanepada branches each, 468 in all —
    // and nowhere else: every credit's number lies in the positional
    // `JNAPADI` range. Goldens ignore traces, so this is what holds both
    // rules inert on the 144 prior roots.
    for sutra in ["10.0493", "6.4.92"] {
        let hits = credited(sutra);
        assert_eq!(hits.len(), 468, "{sutra}");
        for (number, _) in &hits {
            assert!(JNAPADI.contains(number), "{sutra} credited on {number}");
        }
""",
"""
#[test]
fn the_mit_rules_are_credited_on_exactly_the_jnapadi_cells() {
    // 10.0493 fires on every branch of the seven curated jñapādi rows, and
    // 6.4.92 on every ṇic branch: 42 parasmaipada and 36 ātmanepada branches
    // for each of the six that take ṇic, 468 in all; and for √ci
    // (`10.0124`), whose ṇic is optional, 126 and 108 branches, two thirds of
    // them ṇic branches (6.1.54 forks each in two), so 10.0493 has 234 more
    // and 6.4.92 156. Nowhere else: every credit's number lies in the
    // positional `JNAPADI` range. Goldens ignore traces, so this is what
    // holds both rules inert on every other root.
    for (sutra, n) in [("10.0493", 702), ("6.4.92", 624)] {
        let hits = credited(sutra);
        assert_eq!(hits.len(), n, "{sutra}");
        for (number, _) in &hits {
            assert!(JNAPADI.contains(number), "{sutra} credited on {number}");
        }
"""),
("""        .collect()
}

#[test]
#[allow(non_snake_case)]
fn daMSati_trace_has_no_nic_and_no_a_kusmad() {
""",
"""        .collect()
}

/// Whether a branch crediting `ids` is the ṇic-less one `id` opens: `id` is
/// its first credit, or its second after 10.0493, which is credited ahead of
/// the fork on a mit row (√ci, `10.0124`), as vidyut credits it.
fn opens_nicless(ids: &[&str], id: &str) -> bool {
    let ids = ids.strip_prefix(&["10.0493"][..]).unwrap_or(ids);
    ids.first() == Some(&id)
}

#[test]
#[allow(non_snake_case)]
fn daMSati_trace_has_no_nic_and_no_a_kusmad() {
"""),
("""    // for a `NicUbhayapada` row's, which 1.3.72 sanctions). 2573.2 fires only
    // on `pata`. Goldens ignore traces, so this is also what holds all nine
    // inert on every root outside `OPTIONAL_NIC`. The 10.0498, 10.0499, 2564
    // and 2570 rows are the 10f to 10i specs' row tables, listed literally.
    for (id, rows) in [
        (
            "10.0498",
""",
"""    // for a `NicUbhayapada` row's, which 1.3.72 sanctions). 2573.2 fires only
    // on `pata`. Goldens ignore traces, so this is also what holds all nine
    // inert on every root outside `OPTIONAL_NIC`. The 10.0498, 10.0499, 2564
    // and 2570 rows are the 10f to 10j specs' row tables, listed literally.
    for (id, rows) in [
        (
            "10.0498",
"""),
("""            "2570",
            &[
                "10.0227", "10.0230", "10.0174", "10.0184", "10.0243", "10.0249", "10.0260",
                "10.0266",
            ][..],
        ),
        ("2565", &["10.0022"][..]),
""",
"""            "2570",
            &[
                "10.0227", "10.0230", "10.0174", "10.0184", "10.0243", "10.0249", "10.0260",
                "10.0266", "10.0124",
            ][..],
        ),
        ("2565", &["10.0022"][..]),
"""),
("""                        );
                        let nicless: Vec<_> = derive(d, lakara, pada, purusha, vacana)
                            .into_iter()
                            .filter(|p| p.log.first().is_some_and(|s| s.sutra == id))
                            .collect();
                        assert!(!nicless.is_empty(), "{cell}: no ṇic-less branch");
                        let blocks =
""",
"""                        );
                        let nicless: Vec<_> = derive(d, lakara, pada, purusha, vacana)
                            .into_iter()
                            .filter(|p| {
                                let ids: Vec<&str> =
                                    p.log.iter().map(|s| s.sutra.as_str()).collect();
                                opens_nicless(&ids, id)
                            })
                            .collect();
                        assert!(!nicless.is_empty(), "{cell}: no ṇic-less branch");
                        let blocks =
"""),
("""                        );
                        for p in derive(d, lakara, pada, purusha, vacana) {
                            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
                            if ids.first() == Some(&id) {
                                for absent in nic_only {
                                    assert!(!ids.contains(&absent), "{cell} {absent}: {ids:?}");
                                }
""",
"""                        );
                        for p in derive(d, lakara, pada, purusha, vacana) {
                            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
                            if opens_nicless(&ids, id) {
                                for absent in nic_only {
                                    assert!(!ids.contains(&absent), "{cell} {absent}: {ids:?}");
                                }
"""),
("""/// The rows whose live branches credit `sutra` — in the sanādi stage only
/// when `sanadi` is set — sorted, each once. The stage ends where the next
/// opens, at the first of 3.1.32 (which closes it where it fires) and the pada
/// rules 1.3.12, 1.3.66, 1.3.72, 1.3.74, 1.3.78 (which open the stage after
/// it, so a branch without 3.1.32 still has a bounded window); 3.4.78, the tiṅ
/// substitution that every live branch credits, is the backstop. The aṅga
/// stage's later credits stay outside.
fn rows_crediting(sutra: &str, sanadi: bool) -> Vec<&'static str> {
""",
"""/// The rows whose live branches credit `sutra` — in the sanādi stage only
/// when `sanadi` is set — sorted, each once. The stage ends where the next
/// opens, at the first of 3.1.32 (which closes it where it fires) and the pada
/// rules 2567, 1.3.12, 1.3.66, 1.3.72, 1.3.74, 1.3.78 (which open the stage
/// after it, so a branch without 3.1.32 still has a bounded window); 3.4.78, the tiṅ
/// substitution that every live branch credits, is the backstop. The aṅga
/// stage's later credits stay outside.
fn rows_crediting(sutra: &str, sanadi: bool) -> Vec<&'static str> {
"""),
("""                                ids.iter()
                                    .position(|s| {
                                        [
                                            "3.1.32", "1.3.12", "1.3.66", "1.3.72", "1.3.74",
                                            "1.3.78", "3.4.78",
                                        ]
                                        .contains(s)
                                    })
""",
"""                                ids.iter()
                                    .position(|s| {
                                        [
                                            "3.1.32", "2567", "1.3.12", "1.3.66", "1.3.72",
                                            "1.3.74", "1.3.78", "3.4.78",
                                        ]
                                        .contains(s)
                                    })
"""),
("""fn the_10h_vrddhi_and_nuk_rules_fire_only_on_their_rows() {
    // Goldens ignore traces, so this is what holds slice 10h's rules to
    // their rows across the corpus: 7.2.115 on exactly the eight vowel-final
    // ādhṛṣīya rows and, since slice 10i, √pṝ, √ji and √ci; the sanādi 6.1.78
    // on the six of those ādhṛṣīya rows whose vṛddhi is an ec (√vṛ and √jṝ
    // reach `Ar` directly, as √pṝ does) and on √ji and √ci; 7.3.37.2 on √dhū
    // and √prī; and 7.2.114 on √mṛj, before ṇic and on its ṇic-less branch
    // alike.
    assert_eq!(
        rows_crediting("7.2.115", false),
        [
            "10.0022", "10.0324", "10.0325", "10.0343", "10.0345", "10.0346", "10.0347", "10.0361",
            "10.0372", "10.0373", "10.0382"
        ]
    );
    assert_eq!(
        rows_crediting("6.1.78", true),
        [
            "10.0324", "10.0325", "10.0343", "10.0347", "10.0361", "10.0372", "10.0373", "10.0382"
        ]
    );
    assert_eq!(rows_crediting("7.3.37.2", false), ["10.0372", "10.0373"]);
""",
"""fn the_10h_vrddhi_and_nuk_rules_fire_only_on_their_rows() {
    // Goldens ignore traces, so this is what holds slice 10h's rules to
    // their rows across the corpus: 7.2.115 on exactly the eight vowel-final
    // ādhṛṣīya rows, since slice 10i √pṝ, √ji and √ci (`10.0325`), and since
    // slice 10j √smiṅ, √ci (`10.0124`), √ghṛ, √gṛ, √yu, √cyu and √bhū — never
    // √jñā, whose puk comes first; the sanādi 6.1.78 on the rows whose
    // vṛddhi is an ec (√vṛ, √jṝ, √pṝ, √ghṛ and √gṛ reach `Ar` directly);
    // 7.3.37.2 on √dhū and √prī; and 7.2.114 on √mṛj, before ṇic and on its
    // ṇic-less branch alike.
    assert_eq!(
        rows_crediting("7.2.115", false),
        [
            "10.0022", "10.0058", "10.0124", "10.0152", "10.0231", "10.0235", "10.0275", "10.0277",
            "10.0324", "10.0325", "10.0343", "10.0345", "10.0346", "10.0347", "10.0361", "10.0372",
            "10.0373", "10.0382"
        ]
    );
    assert_eq!(
        rows_crediting("6.1.78", true),
        [
            "10.0058", "10.0124", "10.0235", "10.0275", "10.0277", "10.0324", "10.0325", "10.0343",
            "10.0347", "10.0361", "10.0372", "10.0373", "10.0382"
        ]
    );
    assert_eq!(rows_crediting("7.3.37.2", false), ["10.0372", "10.0373"]);
"""),
("""}

#[test]
#[allow(non_snake_case)]
fn vicCayati_and_vicCAyati_take_tuk_before_3_1_32() {
    // viC P laT P.E.: two live branches, the ṇic one at index 0. Both take
""",
"""}

#[test]
fn the_10j_rules_fire_only_on_their_rows() {
    // Goldens ignore traces, so this is what holds slice 10j's three new
    // rules to their rows across the corpus: 6.1.54 on √ci (`10.0124`) alone,
    // never on the āsvadīya `10.0325 ci`; 7.3.36's puk on √ci's 6.1.54 branch
    // and √jñā, the two ā-final aṅgas before ṇic; and Kaumudī 2567 on √smiṅ,
    // the one ṅit curādi row.
    assert_eq!(rows_crediting("6.1.54", true), ["10.0124"]);
    assert_eq!(rows_crediting("7.3.36", true), ["10.0124", "10.0258"]);
    assert_eq!(rows_crediting("2567", false), ["10.0058"]);
}

#[test]
#[allow(non_snake_case)]
fn vicCayati_and_vicCAyati_take_tuk_before_3_1_32() {
    // viC P laT P.E.: two live branches, the ṇic one at index 0. Both take
"""),
],
}
out, bad = {}, []
for path, edits in E.items():
    s = open(path).read()
    for old, new in edits:
        n = s.count(old)
        if n != 1:
            bad.append(f"{path}: {n}x {old[:70]!r}")
            continue
        s = s.replace(old, new)
    out[path] = s
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
for path, s in out.items():
    open(path, 'w').write(s)
print(f"applied {sum(len(e) for e in E.values())} edits")
```

```bash
python3 /tmp/vidyut-full/slice10j/assertions_10j.py      # applied 44 edits
mise run fmt
```

- [ ] **Step 2: Run them to see them fail**

Run: `mise exec -- cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .*FAILED|test result: FAILED"`
Expected: `panini-data` 25 passed / 3 failed, `panini-prakriya` 435 passed / 2 failed, `paradigm` 24 passed / 4 failed, `trace` 210 passed / 5 failed, every other binary passing:

- `curadi::a_kusmad_is_credited_on_exactly_the_akusmiya_cells`
- `curadi::an_akusmiya_roots_parasmaipada_is_blocked_by_a_kusmad_alone`
- `curadi::the_10h_vrddhi_and_nuk_rules_fire_only_on_their_rows`
- `curadi::the_10j_rules_fire_only_on_their_rows`
- `curadi::the_mit_rules_are_credited_on_exactly_the_jnapadi_cells`
- `curadi_analyses_its_ajanta_forms`
- `curadi_analyses_its_asvadiya_forms`
- `curadi_analyses_its_jnapadi_forms`
- `derivation_set_shape_matches_the_audited_numbers`
- `tests::curadi_rows_are_the_three_hundred_twenty_seven_curated_roots`
- `tests::curated_roots_have_expected_ganas_and_padas`
- `tests::optional_nic_matches_upadesha_markers`
- `tinanta::samjna::tests::cisphur_is_the_ciy_row_6_1_54_names`
- `tinanta::sanadi::tests::each_optional_nic_rule_takes_the_nicless_branch_on_its_own_rows`

`curated_pada_agrees_with_upadesha_markers`, `the_optional_nic_ids_are_credited_only_on_their_rows` and `no_nic_pada_rule_reaches_a_nicless_branch` pass already: with no √smiṅ or √ci row their new arms have nothing to read.

- [ ] **Step 3: The rows**

Create `/tmp/vidyut-full/slice10j/rows_10j.py` if it is missing (sha256 `8e3bbdbdfbd36e21784a5a8d978b1199d90f0b694b3a2d5f9da717ca141975ec`). The codes are `stored_form`'s output; `dhatupatha_numbers_resolve_upstream` checks each.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10j — the eight rows' `Dhatu` entries, appended to
`DHATUS`, and √ci's `OPTIONAL_NIC` entry. Run from the worktree root."""
p = 'crates/panini-data/src/lib.rs'
s = open(p).read()
ROWS = """    Dhatu {
        // 10.0058 `zmiN` anAdare (√smiṅ). Ṅit, so ātmanepadī, and under ṇic
        // Kaumudī 2567 keeps it so: 1.3.12 credits, not 1.3.74 (*smāyayate*).
        // 7.2.115 *aco ñṇiti* and the sanādi 6.1.78 make `smAy`. Stored per
        // 6.1.64. Slice 10j.
        dhatupatha: "10.0058",
        code: "smi",
        gana: Gana::Curadi,
        pada: PadaAssignment::Atmanepada,
        artha: "anAdare",
    },
    Dhatu {
        // 10.0124 `ciY` cayane (√ci). Jñapādi, so mit (10.0493); ñit, so ṇic
        // optional by Kaumudī 2570. Ubhayapadī by 1.3.74 with ṇic, where 6.1.54
        // *cisphuror ṇau* optionally gives `cA` and 7.3.36 its puk
        // (*capayati*), or 7.2.115 and 6.1.78 give `cAy` (*cayayati*), and
        // 6.4.92 shortens both; ubhayapadī by 1.3.72 without (*cayati* /
        // *cayate*). Slice 10j.
        dhatupatha: "10.0124",
        code: "ci",
        gana: Gana::Curadi,
        pada: PadaAssignment::NicUbhayapada,
        artha: "cayane",
    },
    Dhatu {
        // 10.0152 `Gf` prasravaRe (√ghṛ). Ubhayapadī by 1.3.74 (*ghārayati*).
        // 7.2.115 *aco ñṇiti* makes `GAr` (1.1.51). Slice 10j.
        dhatupatha: "10.0152",
        code: "Gf",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prasravaRe",
    },
    Dhatu {
        // 10.0231 `gf` vijYAne (√gṛ). Ākusmīya: ātmanepadī by 10.0496
        // (*gārayate*). 7.2.115 makes `gAr`. Slice 10j.
        dhatupatha: "10.0231",
        code: "gf",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "vijYAne",
    },
    Dhatu {
        // 10.0235 `yu` jugupsAyAm (√yu). Ākusmīya: ātmanepadī by 10.0496
        // (*yāvayate*). 7.2.115 and the sanādi 6.1.78 make `yAv`. Slice 10j.
        dhatupatha: "10.0235",
        code: "yu",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "jugupsAyAm",
    },
    Dhatu {
        // 10.0258 `jYA` niyoge (√jñā). Ubhayapadī by 1.3.74 (*jñāpayati*).
        // 7.3.36 adds puk; outside the jñapādi, so not mit, and the `A` stays
        // long. Slice 10j.
        dhatupatha: "10.0258",
        code: "jYA",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "niyoge",
    },
    Dhatu {
        // 10.0275 `cyu` sahane hasane ca (√cyu). Ubhayapadī by 1.3.74
        // (*cyāvayati*). 7.2.115 and the sanādi 6.1.78 make `cyAv`. Slice 10j.
        dhatupatha: "10.0275",
        code: "cyu",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sahane hasane ca",
    },
    Dhatu {
        // 10.0277 `BU` avakalkane (√bhū). Ubhayapadī by 1.3.74 (*bhāvayati*),
        // as the ādhṛṣīya `10.0382 BU` is on its ṇic branch. 7.2.115 and the
        // sanādi 6.1.78 make `BAv`. Slice 10j.
        dhatupatha: "10.0277",
        code: "BU",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "avakalkane",
    },
"""
end = "];\n\npub fn dhatus() -> &'static [Dhatu] {"
assert s.count(end) == 1
s = s.replace(end, ROWS + end)
old = '    ("10.0114", "2564"),\n'
assert s.count(old) == 1
s = s.replace(old, old + '    ("10.0124", "2570"),\n')
open(p, 'w').write(s)
print("inserted 8 rows and 1 OPTIONAL_NIC entry")
```

```bash
python3 /tmp/vidyut-full/slice10j/rows_10j.py      # inserted 8 rows and 1 OPTIONAL_NIC entry
```

- [ ] **Step 4: Generate the goldens**

Create `/tmp/vidyut-full/vidyut-prakriya/examples/curadi_goldens_10j.rs` if it is missing (sha256 `8ed95a9ebf286743c4b81543b675e4252cdc5f46a80df6b1944af9d80de04ae1`). It is 10i's generator with the row selection and the vikalpa list changed: it derives every cell of the eight rows in both engines, asserts the form sets equal, and writes the rows the static wants (pinned form = first live branch; each further distinct form an `ALTERNATES` row keyed by its branch's vikalpa ids in log order).

```rust
//! THROWAWAY: slice 10j — emit the eight ajanta rows' goldens from the engine,
//! asserting every cell's form set equals vidyut's first.
use panini::Panini;
use panini_data::{dhatus, Lakara as L, Pada, Purusha as P, Vacana as V};
use vidyut_prakriya::args::{DhatuPada, Lakara, Prayoga, Purusha, Tinanta, Vacana};
use vidyut_prakriya::{Dhatupatha, Vyakarana};
const VIKALPA_RULES: &[&str] = &[
    "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3", "2573.2", "6.1.54", "7.3.37.2", "7.1.35", "3.4.111", "7.3.86", "6.4.107",
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
    // The new rows, in dhātupāṭha order.
    const NEW: [&str; 8] = ["10.0058", "10.0124", "10.0152", "10.0231", "10.0235", "10.0258", "10.0275", "10.0277"];
    for num in NEW {
        let d = dhatus().iter().find(|d| d.dhatupatha == num).unwrap();
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
    std::fs::write("/tmp/vidyut-full/goldens_10j_paradigm.rs", par).unwrap();
    std::fs::write("/tmp/vidyut-full/goldens_10j_alternates.rs", alt).unwrap();
    println!("{ncells} cells, {nforms} forms, {ndiff} differences");
    assert_eq!(ndiff, 0);
}
```

```bash
WT="$(git rev-parse --show-toplevel)"
V=/tmp/vidyut-full/vidyut-prakriya
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml      # must point at $WT/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example curadi_goldens_10j 2>/dev/null | tail -1)
sha256sum /tmp/vidyut-full/goldens_10j_paradigm.rs /tmp/vidyut-full/goldens_10j_alternates.rs
```

Expected: `468 cells, 654 forms, 0 differences`, then:
- `4ea7518b02ac8b952e230ca594c6859d0ee734e1aa0967314dc7079fc4e1b7a5  /tmp/vidyut-full/goldens_10j_paradigm.rs`
- `7068aed414ddfdf0d78d69dd4277a8706283fecf148225e00de3e44651ad2182  /tmp/vidyut-full/goldens_10j_alternates.rs`

Leave the dev-deps pointing at this worktree; Task 4 uses them. If a line differs, stop and report.

- [ ] **Step 5: Insert the goldens**

Create `/tmp/vidyut-full/slice10j/insert_goldens_10j.py` if it is missing (sha256 `cc36d6fab09b9c525a7d968f6eb80d06d1f149e8039a2522650bf4805455d9f6`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10j — insert the generator's goldens before each
static's `];`. Run from the worktree root."""
p = 'crates/panini/tests/paradigm/data/curadi.rs'
s = open(p).read()
par = open('/tmp/vidyut-full/goldens_10j_paradigm.rs').read()
alt = open('/tmp/vidyut-full/goldens_10j_alternates.rs').read()
i = s.index('pub const ALTERNATES')
head, tail = s[:i], s[i:]
k = head.rindex('];'); head = head[:k] + par + head[k:]
k = tail.rindex('];'); tail = tail[:k] + alt + tail[k:]
open(p, 'w').write(head + tail)
print("inserted goldens")
```

```bash
python3 /tmp/vidyut-full/slice10j/insert_goldens_10j.py      # inserted goldens
mise run fmt
```

- [ ] **Step 6: The measured pada-ambiguous set**

The set is measured, never hand-picked: run the test against the old set and read the real one off its failure. Create `/tmp/vidyut-full/slice10j/pin_ambiguous_10j.py` if it is missing (sha256 `354c19bb7d72f9dc8b294aa3046aa911aeb3f9fe53098dec7c1ab9ef8c967aa2`). It refuses a set whose size or hash differs from the prototype's.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10j — pin the measured pada-ambiguous set ($SET, the
failing test's `left:` JSON) into pada_ambiguous_surfaces_are_exactly_these,
and extend the comment that accounts for it. Run from the worktree root."""
import hashlib, json, os
amb = json.load(open(os.environ['SET']))
assert len(amb) == 1063, len(amb)
h = hashlib.sha256('\n'.join(amb).encode()).hexdigest()
assert h == '7598ba6c6c7bb5e138442d63993f36701c8d8a19e96ff3f3f98fdfbaa13e97cf', h
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
old = """    // half. 208 more, taking the set from 839 to 1047.
"""
assert s.count(old) == 1
s = s.replace(old, old + """    // Slice 10j's √ghṛ, √jñā, √cyu and √ci (its declined ṇic branch, the
    // pinned form) contribute the same four each — 16 more, taking the set
    // from 1047 to 1063. √bhū's four are the ādhṛṣīya `10.0382 BU`'s already;
    // √gṛ, √yu and √smiṅ are ātmanepadī only.
""")
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
SET="$SET" python3 /tmp/vidyut-full/slice10j/pin_ambiguous_10j.py      # pinned 1063
mise run fmt
```

The sixteen new surfaces are √ghṛ's, √jñā's, √cyu's and √ci's four each (`aGArayata` / `GArayatAm` / `GArayetAm` / `GArayeta`, …); √bhū's four are the ādhṛṣīya `10.0382 BU`'s already.

- [ ] **Step 7: Run the full suite**

```bash
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS at 26424 cells, with `panini-prakriya` 437, `trace` 215, `paradigm` 28, `panini-data` 28.

Grep the goldens for the `check()` witnesses, which the tests also enforce:

```bash
for f in smAyayate cayayati cayayate capayati capayate cayati cayate GArayati GArayate gArayate yAvayate jYApayati jYApayate cyAvayati cyAvayate BAvayati BAvayate smAyayati gArayati yAvayati smayayate Garayati cyavayati cApayati jYAyayati; do
  printf "%s: %s\n" $f "$(grep -c "\"$f\"" crates/panini/tests/paradigm/data/*.rs | grep -v ':0' | sed 's#.*/##' | tr '\n' ' ')"; done
```

Expected:
- `capayati`, `capayate`, `cayati`, `BAvayati` and `BAvayate` appear twice, in `curadi.rs` only.
- Every other Valid witness appears once, in `curadi.rs` only.
- The eight Invalid shapes (`smAyayati` … `jYAyayati`) print nothing.

- [ ] **Step 8: Commit**

```bash
mise run lint
git branch --show-current      # curadi-10j
git add -A
git commit -m "feat(data): curādi's eight ajanta rows — √smiṅ, √ci, √ghṛ, √gṛ, √yu, √jñā, √cyu, √bhū

25956 → 26424 cells, 36416 → 37070 forms, ALTERNATES 10460 → 10646, 422 →
430 roots; pada-ambiguous surfaces 1047 → 1063. √ci joins OPTIONAL_NIC
(181) by 2570 and forks three ways; √gṛ and √yu complete the ākusmīya; a
curādi Atmanepada row must be ṅit (2567's). Goldens generated
cell-by-cell equal to vidyut."
```

---

## Task 4: Audit, prior-trace diff, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`
- Modify: `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`
- Modify: `crates/panini/tests/paradigm/main.rs` (doc counts, audit prose), `crates/panini/tests/trace/curadi.rs` (module doc), `crates/panini-prakriya/src/tinanta/guna.rs` (one comment), `crates/panini-data/src/lib.rs` (one comment)
- Modify: the 10b–10i specs

**Interfaces:**
- Consumes: the finished engine, data and goldens. Produces no symbols.

- [ ] **Step 1: Update the audit harness**

Create `/tmp/vidyut-full/slice10j/audit_10j.py` if it is missing (sha256 `8d3b809d83d0183224abe224244618ff2ec9b5121ea02d951093bcb5965c7645`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10j's edits to tools/audit/panini_full_audit.rs. Every
`old` must occur exactly once; nothing is written if one fails."""
import sys
p = 'tools/audit/panini_full_audit.rs'
s = open(p).read()
E = [
("""//! What it compares: for each of the 422 curated roots, for each pada the root
//! admits (`Dhatu::padas`; two apiece for the 299 roots that admit both padas —
//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 260 curādi
//! roots by 1.3.74, six more by 1.3.74 with ṇic and 1.3.72 without, and seven optional-ṇic ākusmīya or ā-garvīya roots by""",
"""//! What it compares: for each of the 430 curated roots, for each pada the root
//! admits (`Dhatu::padas`; two apiece for the 304 roots that admit both padas —
//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 264 curādi
//! roots by 1.3.74, seven more by 1.3.74 with ṇic and 1.3.72 without, and seven optional-ṇic ākusmīya or ā-garvīya roots by"""),
("""//! Corpus invariants, asserted: 422 roots, 25956 cells, 36416 forms. These are""",
"""//! Corpus invariants, asserted: 430 roots, 26424 cells, 37070 forms. These are"""),
("""//! (`derivation_set_shape_matches_the_audited_numbers`): 2884 root×pada×lakāra
//! blocks × 9 cells, plus 10460 `ALTERNATES` rows.""",
"""//! (`derivation_set_shape_matches_the_audited_numbers`): 2936 root×pada×lakāra
//! blocks × 9 cells, plus 10646 `ALTERNATES` rows."""),
("""//! Optionally dump the full 25956-cell table:""",
"""//! Optionally dump the full 26424-cell table:"""),
("""    assert_eq!(roots_seen.len(), 422, "curated roots");
    assert_eq!(n_cells, 25956, "cells: 2884 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 36416, "forms: 25956 cells + 10460 ALTERNATES rows");""",
"""    assert_eq!(roots_seen.len(), 430, "curated roots");
    assert_eq!(n_cells, 26424, "cells: 2936 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 37070, "forms: 26424 cells + 10646 ALTERNATES rows");"""),
]
bad = []
for old, new in E:
    n = s.count(old)
    if n != 1:
        bad.append(f"{n}x {old[:70]!r}")
        continue
    s = s.replace(old, new)
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
open(p, 'w').write(s)
print(f"applied {len(E)} edits")
```

```bash
python3 /tmp/vidyut-full/slice10j/audit_10j.py      # applied 5 edits
```

The harness now asserts 430 / 26424 / 37070 and names 304 both-pada roots (299 + √ci, √ghṛ, √jñā, √cyu, √bhū), 264 of them `Nic`.

- [ ] **Step 2: The prior-trace diff and the audit**

The dev-deps still point at this worktree from Task 3. Create `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_10j.rs` if it is missing (sha256 `b4d4e7bd7afd945c00bbedc07b5662a70cf6a70f74dca93ba7dd530c457a88f0`). It lists 10j's rows literally so that it also builds against main, and it dumps every branch, blocked ones included:

```rust
//! THROWAWAY: slice 10j — dump every prior cell's branches, blocked ones
//! included, each with its credited-rule log.
use panini::Panini;
use panini_data::{Lakara as L, Purusha as P, Vacana as V};
/// Slice 10j's eight rows, listed literally so the dump also builds against
/// main.
const NEW: &[&str] = &["10.0058", "10.0124", "10.0152", "10.0231", "10.0235", "10.0258", "10.0275", "10.0277"];
fn main() {
    assert_eq!(NEW.len(), 8);
    let panini = Panini::new();
    for d in panini_data::dhatus() {
        if NEW.contains(&d.dhatupatha) { continue; }
        for pada in d.padas() {
            for l in [L::Lat, L::Lan, L::Lot, L::VidhiLin] { for pu in [P::Prathama, P::Madhyama, P::Uttama] { for va in [V::Eka, V::Dvi, V::Bahu] {
                for b in panini.derive(d, l, *pada, pu, va) {
                    let ids: Vec<&str> = b.log.iter().map(|s| s.sutra.as_str()).collect();
                    println!("{} {:?} {:?} {:?} {:?} blocked={} {} : {}", d.dhatupatha, pada, l, pu, va, b.blocked, b.text(), ids.join(" "));
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
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10j 2>/dev/null > "$DUMP/branch.txt")
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml   # must point at /workspace/crates
git -C /workspace branch --show-current   # main
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10j 2>/dev/null > "$DUMP/main.txt")
wc -l < "$DUMP/main.txt"; wc -l < "$DUMP/branch.txt"            # 42932 and 42932
grep -c "blocked=false" "$DUMP/main.txt"                        # 36416
cmp "$DUMP/main.txt" "$DUMP/branch.txt" && echo PRIOR-TRACES-IDENTICAL
```

Expected: `42932`, `42932`, `36416`, `PRIOR-TRACES-IDENTICAL`. This is the corpus-wide check that 6.1.54, 7.3.36, 2567 and the two moves change no prior root's trace, blocked branches included (goldens ignore traces). `/workspace` must be on `main` (Task 1) for the second dump. If `cmp` reports a difference, stop and report.

Then the audit. Repoint at this worktree again, copy the committed harness (never rewrite it), and run:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
cp tools/audit/panini_full_audit.rs $V/examples/
(cd $V && PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -8)
(cd $V && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -2)
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml
```

- Expected from the honest run: `roots : 430`, `cells : 26424`, `forms (set sizes): 37070`, `live branches : 37070`, `blocked branches : 6516`, `differing cells  : 0`, `AUDIT PASSED: 26424 cells, 37070 forms, zero differences.`
- Expected from the `entry` control: `AUDIT FAILED: 36 differing cells.`

Do not use `mise -C`. If the honest run shows differences, stop and report, and edit nothing.

- [ ] **Step 3: The doc sweep, the audit record and the spec pointers**

Create `/tmp/vidyut-full/slice10j/docsweep_10j.py` if it is missing (sha256 `8ddb44ef44f92c38bbac784a8e1b4c1f65eed269740527cffc62485bf7e4edbe`). It takes the audit's date (`date -u +%F` on the day Step 2 ran) and writes it into the audit record and AGENTS.md's audit chain. It covers:
- **ARCHITECTURE.md:** the stage table's `sanadi.rs` and `samjna.rs` rows; the pin count (162) and its history; the gaṇa coverage line (327); the pada paragraph (10.0496's place, √smiṅ's 2567); the 7.1.35 / 8.4.56 census (356 parasmaipada columns, 74 ātmanepada-only, 304 both-pada, 712 / 646 / 623 cells; √ci's 6.1.54 keys); the nine-form record (eight cells).
- **README.md:** the curādi line (327) and a 10j paragraph; the corpus (430 roots; 7776 of 26424 cells forked; 6432 / 601 / 365; eight nine-form cells); the both-pada list (304); the pada-ambiguous count (1063) and its account.
- **AGENTS.md:** the cell count, the progress line (327), the fork census (6432 / 601; the six-form record to 365; eight nine-form cells; 10646 + 26424 = 37070), the audit chain, and the `guna.rs:1233` note's corpus size.
- **`tools/audit/README.md`:** the asserted totals and the new top record.
- **Code:** `pada_from_upadesha`'s and `guna.rs`'s 6.1.78 corpus size (430); the paradigm doc block's counts, the 10j key paragraph and the audit chain; `trace/curadi.rs`'s module doc (√ci's 6.4.92 sources, the ṇic-less opening after 10.0493, √jñā's puk, √smiṅ's 2567).
- **Earlier specs:** the 10b–10i "Later slices" items that name √smiṅ, √gṛ, √yu or √ci gain a pointer to this slice's spec.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10j's doc sweep — counts, the curādi progress lines, the
stage table and pin history, the audit record, the census prose and the
earlier specs' "Later slices" pointers. Every `old` must occur exactly once
in its file; nothing is written if one fails. Run from the worktree root
with the audit's date as the only argument."""
import sys
DATE = sys.argv[1]
PTR = "(Slice 10j took √smiṅ, √gṛ, √yu and √ci: see `2026-10-04-curadi-gana-10j-design.md`.)"
E = {}
def add(path, old, new):
    E.setdefault(path, []).append((old, new))

# ---- docs/ARCHITECTURE.md
A = 'docs/ARCHITECTURE.md'
add(A, """| `sanadi.rs` | 10.0498, 10.0499, 2564, 2565, 2570, 2571, 2573.1, 2573.3, 2573.2, 10.0496, 10.0497, 10.0493, 3.1.25, 3.1.28, 1.3.9, 3.4.114, 6.4.48, 7.2.116, 6.4.92, 7.3.37.2, 7.2.115, 6.1.78, 7.2.114, 6.1.73, 7.3.86, 3.1.32 — the optional-ṇic fork, the ākusmīya and ā-garvīya pada and the jñapādi's mit-tva, then ṇic, or āya where √dhūp and √vich take none, the adanta root's final `a`, the root's vṛddhi or guṇa or √dhū's and √prī's nuk, √vich's tuk ahead of guṇa, and the pratyaya's folding into the dhātu""",
"""| `sanadi.rs` | 10.0493, 10.0498, 10.0499, 2564, 2565, 2570, 2571, 2573.1, 2573.3, 2573.2, 10.0496, 10.0497, 3.1.25, 3.1.28, 1.3.9, 3.4.114, 6.4.48, 6.1.54, 7.2.116, 7.3.37.2, 7.3.36, 7.2.115, 6.1.78, 6.4.92, 7.2.114, 6.1.73, 7.3.86, 3.1.32 — the jñapādi's mit-tva, the optional-ṇic fork and the ākusmīya and ā-garvīya pada, then ṇic, or āya where √dhūp and √vich take none, the adanta root's final `a`, √ci's optional `A`, the root's vṛddhi or guṇa or √dhū's and √prī's nuk or an ā-final aṅga's puk, the mit root's short upadhā, √vich's tuk ahead of guṇa, and the pratyaya's folding into the dhātu""")
add(A, """| `samjna.rs` | 1.3.12, 1.3.66, 1.3.72, 1.3.74, 1.3.78, 3.4.78, 1.3.9, 1.2.4 | before 3.1.68 |""",
"""| `samjna.rs` | 2567, 1.3.12, 1.3.66, 1.3.72, 1.3.74, 1.3.78, 3.4.78, 1.3.9, 1.2.4 | before 3.1.68 |""")
add(A, """pins all 159 ids verbatim""", """pins all 162 ids verbatim""")
add(A, """3.1.25; and a second entry for 6.1.73 *che ca*, before the sanādi 7.3.86 —
159 total).""",
"""3.1.25; and a second entry for 6.1.73 *che ca*, before the sanādi 7.3.86 —
159 total — then curādi 10j's three: 6.1.54 *cisphuror ṇau* after 6.4.48,
7.3.36 *artihrīvlīrīknūyīkṣmāyyātāṃ puk ṇau* before 7.2.115, and Kaumudī
2567 at the head of `samjna.rs`; 10j also moved 10.0493 to the head of
`sanadi.rs` and 6.4.92 to after the sanādi 6.1.78 — 162 total).""")
add(A, """slice 3f3) — and curādi (10), **open** at 319 of its""",
"""slice 3f3) — and curādi (10), **open** at 327 of its""")
add(A, """the fifty-nine āsvadīya rows, √pṝ and √ghuṣ, slice 10i). gaṇa""",
"""the fifty-nine āsvadīya rows, √pṝ and √ghuṣ, slice 10i; √smiṅ, √ci, √ghṛ, √gṛ, √yu, √jñā, √cyu and √bhū, slice 10j). gaṇa""")
add(A, """earlier, by the dhātupāṭha gaṇasūtra 10.0496 *ā kusmād ātmanepadinaḥ* at the
head of `sanadi.rs`: it sanctions ātmanepada and blocks parasmaipada exactly""",
"""earlier, by the dhātupāṭha gaṇasūtra 10.0496 *ā kusmād ātmanepadinaḥ* in
`sanadi.rs`, after the optional-ṇic fork: it sanctions ātmanepada and blocks parasmaipada exactly""")
add(A, """one: the fork that drops ṇic adds `Tag::Ubhayapadin` as it strips
`Tag::Nic`. `INVALID` means""",
"""one: the fork that drops ṇic adds `Tag::Ubhayapadin` as it strips
`Tag::Nic`. √smiṅ (`10.0058 zmiN`, slice 10j), the one ṅit curādi root, stays
ātmanepadī under ṇic by Kaumudī 2567 at the head of `samjna.rs`, and 1.3.12
then sanctions or blocks as for any ātmanepadī root. `INVALID` means""")
add(A, """8.2.39 obligatorily voices its final `t` to `d`), forking 702 cells (loṭ
prathama and madhyama eka across the 351 roots with a parasmaipada column —
`tu`/`hi` are parasmaipada endings, so the curated set's 71 ātmanepada-only
roots never reach this guard, and the 299 roots that admit both""",
"""8.2.39 obligatorily voices its final `t` to `d`), forking 712 cells (loṭ
prathama and madhyama eka across the 356 roots with a parasmaipada column —
`tu`/`hi` are parasmaipada endings, so the curated set's 74 ātmanepada-only
roots never reach this guard, and the 304 roots that admit both""")
add(A, """ādhṛṣīya rows (six of them by 1.3.72 too, without ṇic) and slice 10i's
sixty-one rows by 1.3.74, and slice 10f's six optional-ṇic ākusmīya roots and""",
"""ādhṛṣīya rows (six of them by 1.3.72 too, without ṇic), slice 10i's
sixty-one rows and slice 10j's √ghṛ, √jñā, √cyu, √bhū and √ci (by 1.3.72 too,
without ṇic) by 1.3.74, and slice 10f's six optional-ṇic ākusmīya roots and""")
add(A, """the parasmaipada columns of √bhṛ, √ṇij, √vij and √viṣ; 351 + 71 = the 422 curated roots)""",
"""the parasmaipada columns of √bhṛ, √ṇij, √vij and √viṣ; 356 + 74 = the 430 curated roots)""")
add(A, """utterance, forking 636 cells outright: laṅ and vidhiliṅ prathama eka across
those same 351 parasmaipada columns (613 of them""",
"""utterance, forking 646 cells outright: laṅ and vidhiliṅ prathama eka across
those same 356 parasmaipada columns (623 of them""")
add(A, """10h's other than its sixteen laghu-ik rows and 10i's other than its eleven (whose ṇic branch keys on the sanādi 7.3.86 as well) on their ṇic branch; 10f's to 10i's ṇic-less forks key on their optional-ṇic id as well""",
"""10h's other than its sixteen laghu-ik rows and 10i's other than its eleven (whose ṇic branch keys on the sanādi 7.3.86 as well) on their ṇic branch, and 10j's √ghṛ, √jñā, √cyu and √bhū, and √ci on its declined ṇic branch; √ci's 6.1.54 branch keys on 6.1.54 as well (`6.1.54+8.4.56`), and 10f's to 10j's ṇic-less forks key on their optional-ṇic id""")
add(A, """forking a further 702 (the same
loṭ cells 7.1.35 just forked)""",
"""forking a further 712 (the same
loṭ cells 7.1.35 just forked)""")

add(A, """slice 10h's √dhū and √prī loṭ tātaṅ cells, six nine-form cells in all, now hold the record)""",
"""slice 10h's √dhū and √prī and slice 10j's √ci loṭ tātaṅ cells, eight nine-form cells in all, now hold the record)""")

add('crates/panini/tests/trace/curadi.rs', """//! step earlier, with the gaṇasūtra 10.0493, and 6.4.92 follows its
//! 7.2.116. An adanta root's has 6.4.48 after 3.4.114 and neither 7.2.116""",
"""//! step earlier, with the gaṇasūtra 10.0493, and 6.4.92 follows its
//! 7.2.116 (on √ci, its 7.2.115 and 6.1.78, or its 6.1.54 and 7.3.36's puk).
//! An adanta root's has 6.4.48 after 3.4.114 and neither 7.2.116""")
add('crates/panini/tests/trace/curadi.rs', """//! has a ṇic-less branch besides, opening with the id that makes it so
//! (10.0498, 10.0499, 2564, 2565, 2570, 2571, 2573.1, 2573.3) and running""",
"""//! has a ṇic-less branch besides, opening with the id that makes it so
//! (10.0498, 10.0499, 2564, 2565, 2570, 2571, 2573.1, 2573.3; after 10.0493 on
//! √ci) and running""")
add('crates/panini/tests/trace/curadi.rs', """//! 3.1.32, √dhū's and √prī's the vārttika 7.3.37.2 instead on a second
//! branch, and √mṛj's 7.2.114.""",
"""//! 3.1.32, √dhū's and √prī's the vārttika 7.3.37.2 instead on a second
//! branch, √jñā's 7.3.36's puk instead, and √mṛj's 7.2.114. √smiṅ's pada
//! sūtra is Kaumudī 2567, then 1.3.12.""")

# ---- README.md
R = 'README.md'
add(R, """8.4.44 *śāt* exemption. *curādi* (10) is **open** at 319 of its 509""",
"""8.4.44 *śāt* exemption. *curādi* (10) is **open** at 327 of its 509""")
add(R, """ca*) comes before guṇa on every branch, so *vicchayati*, not *vechayati*.
Every curādi root""",
"""ca*) comes before guṇa on every branch, so *vicchayati*, not *vechayati*.
Slice 10j curated the eight ajanta rows: √ghṛ, √jñā, √cyu and √bhū
(*bhāvayati*), ubhayapadī by 1.3.74; √gṛ and √yu, the last two ākusmīya
rows; √smiṅ, ṅit, which Kaumudī 2567 keeps ātmanepadī under ṇic
(*smāyayate*); and √ci, the last jñapādi root, whose ṇic is optional (2570)
and which forks three ways: 6.1.54 *cisphuror ṇau* optionally gives `cA`,
7.3.36 *artihrīvlīrīknūyīkṣmāyyātāṃ puk ṇau* adds puk to that and to √jñā
(*jñāpayati*), and 6.4.92 shortens both mit branches (*capayati*,
*cayayati*, beside *cayati*).
Every curādi root""")
add(R, """curated 422-root set, in four lakāras""", """curated 430-root set, in four lakāras""")
add(R, """both correct — and in fact 7688 of the 25956 cells hold more than one form: 6424
hold two, 525 hold three""",
"""both correct — and in fact 7776 of the 26424 cells hold more than one form: 6432
hold two, 601 hold three""")
add(R, """√prī's ṇic branch forking again on 7.3.37.2's nuk, and slice 10i's
sixty-one add 1952 two-form cells the same way),""",
"""√prī's ṇic branch forking again on 7.3.37.2's nuk, and slice 10i's
sixty-one add 1952 two-form cells the same way, and slice 10j's √ghṛ, √jñā,
√cyu and √bhū add eight two-form and eight three-form cells as √cur's do,
and √ci 68 three-form cells, its three readings where neither 7.1.35 nor
8.4.56 forks),""")
add(R, """7.1.35/6.4.116/8.4.56), and 363 hold six""", """7.1.35/6.4.116/8.4.56), and 365 hold six""")
add(R, """122 more: the sixty-one rows' loṭ parasmaipada prathama and madhyama eka,
the same way. One cell""",
"""122 more: the sixty-one rows' loṭ parasmaipada prathama and madhyama eka,
the same way; and, new in slice 10j, √ci's laṅ and vidhiliṅ parasmaipada
prathama eka, three readings × 8.4.56. One cell""")
add(R, """barring the rules that would change it. Six cells hold **nine**, the record:""",
"""barring the rules that would change it. Eight cells hold **nine**, the record:""")
add(R, """`Davatu` / `DavatAd` / `DavatAt` for √dhū).""",
"""`Davatu` / `DavatAd` / `DavatAt` for √dhū), and — new in slice 10j — √ci's
(`cayayatu` / `capayatu` / `cayatu` and their tātaṅ pairs).""")
add(R, """padas — 299 roots that admit both padas in the curated set""",
"""padas — 304 roots that admit both padas in the curated set""")
add(R, """ṇic) and slice 10i's sixty-one rows by 1.3.74; and slice 10f's six optional-ṇic ākusmīya roots and""",
"""ṇic), slice 10i's sixty-one rows and slice 10j's √ghṛ, √jñā, √cyu, √bhū and
√ci (by 1.3.72 too, without ṇic) by 1.3.74; and slice 10f's six optional-ṇic ākusmīya roots and""")
add(R, """1047 of the pinned (`PARADIGM`) surfaces are pada-ambiguous""",
"""1063 of the pinned (`PARADIGM`) surfaces are pada-ambiguous""")
add(R, """twice, `svad` and `svAd`). The
enumeration is not""",
"""twice, `svad` and `svAd`); slice 10j's √ghṛ, √jñā, √cyu and √ci add 16 more,
four each, √bhū's being the ādhṛṣīya `10.0382 BU`'s already. The
enumeration is not""")
add(R, """set, all 1047. It is therefore""", """set, all 1063. It is therefore""")

# ---- AGENTS.md
G = 'AGENTS.md'
add(G, """  (`crates/panini/tests/paradigm/`, 25956 cells, ten gaṇas, nine complete —""",
"""  (`crates/panini/tests/paradigm/`, 26424 cells, ten gaṇas, nine complete —""")
add(G, """at 319 after slice 10i curated the fifty-nine āsvadīya rows, √pṝ and √ghuṣ —""",
"""at 319 after slice 10i curated the fifty-nine āsvadīya rows, √pṝ and √ghuṣ, at 327 after slice 10j curated the eight ajanta rows √smiṅ, √ci, √ghṛ, √gṛ, √yu, √jñā, √cyu and √bhū —""")
add(G, """    other forms — a second (6424 cells), a third (525 cells), a fourth""",
"""    other forms — a second (6432 cells), a third (601 cells), a fourth""")
add(G, """    eka to 241, and slice 10i's sixty-one rows (loṭ prathama and madhyama
    eka) to 363 — a fourth""",
"""    eka to 241, and slice 10i's sixty-one rows (loṭ prathama and madhyama
    eka) to 363, and slice 10j's √ci (laṅ and vidhiliṅ prathama eka) to 365 —
    a fourth""")
add(G, """    seven-form cell, or up to a ninth for slice 10f's `pata` and slice 10h's
    √dhū and √prī loṭ parasmaipada prathama and madhyama eka, the six
    nine-form cells and the record — in
    `ALTERNATES` (10460 rows in all, so 25956 + 10460 = 36416 forms total); √bhuj""",
"""    seven-form cell, or up to a ninth for slice 10f's `pata`, slice 10h's
    √dhū and √prī and slice 10j's √ci loṭ parasmaipada prathama and madhyama
    eka, the eight nine-form cells and the record — in
    `ALTERNATES` (10646 rows in all, so 26424 + 10646 = 37070 forms total); √bhuj""")
add(G, """  361 roots), and that by curādi 10i's (`tools/audit/README.md`'s 2026-10-04 10i
  entry, 25956 cells / 36416 forms / 422 roots).""",
"""  361 roots), and that by curādi 10i's (`tools/audit/README.md`'s 2026-10-04 10i
  entry, 25956 cells / 36416 forms / 422 roots), and that by curādi 10j's
  (`tools/audit/README.md`'s """ + DATE + """ 10j entry, 26424 cells / 37070 forms /
  430 roots).""")
add(G, """  only in the ordinary corpus-size sense, not wrong in kind: 25956 goldens
  would move today.""",
"""  only in the ordinary corpus-size sense, not wrong in kind: 26424 goldens
  would move today.""")

# ---- tools/audit/README.md
T = 'tools/audit/README.md'
add(T, """**It asserts the corpus totals** (422 roots, 25956 cells, 36416 forms) rather than""",
"""**It asserts the corpus totals** (430 roots, 26424 cells, 37070 forms) rather than""")
add(T, """## Last recorded result

""", """## Last recorded result

""" + DATE + """, curādi 10j slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 26424
cells / 37070 forms / 430 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole curādi 10j slice: the eight ajanta rows √smiṅ,
√ci, √ghṛ, √gṛ, √yu, √jñā, √cyu and √bhū, and the three rules they bring:
6.1.54 *cisphuror ṇau* (√ci's optional `A`), 7.3.36's puk (√ci's 6.1.54
branch and √jñā) and Kaumudī 2567 (√smiṅ's ātmanepada under ṇic), with
10.0493 and 6.4.92 moved for √ci. Blocked branches stay at 6516: the three
ātmanepadī rows admit only ātmanepada, and √ci admits both padas on every
branch. On the throwaway prototype, disabling 6.1.54 as a control made 72
cells differ, all √ci's. A main-vs-branch dump of every prior cell's
branches, blocked ones included, was byte-identical, all 42932 of them
(36416 live).

Totals: 430 = 422 + 8; 26424 = 25956 + 468 (52 root×pada×lakāra blocks ×
9); 37070 = 36416 + 468 + 186 new `ALTERNATES` rows (10460 → 10646),
measured via the harness's corpus block, not assumed.

""")

# ---- code comments
add('crates/panini-data/src/lib.rs', """    /// vendored upadeśa: 73 of the 422 curated roots carry a `\\` at all, and 52""",
"""    /// vendored upadeśa: 73 of the 430 curated roots carry a `\\` at all, and 52""")
add('crates/panini-prakriya/src/tinanta/guna.rs', """    // 422-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)""",
"""    // 430-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)""")
P = 'crates/panini/tests/paradigm/main.rs'
add(P, """/// 25956 cells total (2884 root×lakāra blocks × 9), of which 18268 hold exactly one form, 6424 hold two, 525 hold three""",
"""/// 26424 cells total (2936 root×lakāra blocks × 9), of which 18648 hold exactly one form, 6432 hold two, 601 hold three""")
add(P, """/// sixty-one add 1952 two-form cells, a ṇic and a ṇic-less reading each), 359 hold four""",
"""/// sixty-one add 1952 two-form cells, a ṇic and a ṇic-less reading each, and
/// slice 10j's √ghṛ, √jñā, √cyu and √bhū eight two-form and eight three-form
/// cells as √cur's, and √ci 68 three-form ones), 359 hold four""")
add(P, """/// prathama eka, forking on 7.1.35/6.4.116/8.4.56) and
/// 363
/// hold six""", """/// prathama eka, forking on 7.1.35/6.4.116/8.4.56) and
/// 365
/// hold six""")
add(P, """/// 10i — the sixty-one rows' loṭ parasmaipada prathama and madhyama eka,
/// the same way), one — new in""", """/// 10i — the sixty-one rows' loṭ parasmaipada prathama and madhyama eka,
/// the same way; and — new in slice 10j — √ci's laṅ and vidhiliṅ parasmaipada
/// prathama eka, three readings × 8.4.56), one — new in""")
add(P, """/// readings before *hi* are not a 2^k product; and six hold NINE, the engine's""",
"""/// readings before *hi* are not a 2^k product; and eight hold NINE, the engine's""")
add(P, """/// 7.2.115's vṛddhi with ṇic) × the same triple. No cell holds eight.""",
"""/// 7.2.115's vṛddhi with ṇic) × the same triple, and — new in slice 10j —
/// √ci's (2570's ṇic-less one, 6.1.54's `cA` with puk, and the declined
/// ṇic branch) × the same triple. No cell holds eight.""")
add(P, """/// itself has 10460 rows, keyed 636 `8.4.56`, 628 `7.1.35`, 628 `7.1.35+8.4.56`,""",
"""/// itself has 10646 rows, keyed 646 `8.4.56`, 638 `7.1.35`, 638 `7.1.35+8.4.56`,""")
add(P, """/// `7.3.86+7.1.35+8.4.56` — √kṛ (slice 8b) adds six more""",
"""/// `7.3.86+7.1.35+8.4.56`. Slice 10j opens four keys — 72 `6.1.54` and 2
/// apiece `6.1.54+8.4.56`, `6.1.54+7.1.35` and `6.1.54+7.1.35+8.4.56` (√ci's
/// 6.1.54 branch) — adds 72 to `2570` and 2 apiece to `2570+8.4.56`,
/// `2570+7.1.35` and `2570+7.1.35+8.4.56` (√ci's ṇic-less branch), and folds
/// 10 rows apiece into `8.4.56`, `7.1.35` and `7.1.35+8.4.56` — √kṛ (slice 8b) adds six more""")
add(P, """/// commit over all 25956 cells / 36416 forms / 422 roots with zero
/// differences, its `entry` negative control verified failing (36 √bhū
/// cells).""", """/// commit over all 25956 cells / 36416 forms / 422 roots with zero
/// differences, its `entry` negative control verified failing (36 √bhū
/// cells), and curādi 10j's re-ran it at the same commit over all 26424
/// cells / 37070 forms / 430 roots with zero differences, its `entry`
/// negative control verified failing (36 √bhū cells).""")

# ---- earlier specs' "Later slices"
S = 'docs/superpowers/specs/'
add(S + '2026-10-02-curadi-gana-10b-design.md', """√smiṅ, and with it 7.2.115 before ṇic, rides whichever slice first needs
7.2.115.""", """√smiṅ, and with it 7.2.115 before ṇic, rides whichever slice first needs
7.2.115. """ + PTR)
add(S + '2026-10-02-curadi-gana-10c-design.md', """ākusmīya rows that wait on it.""", """ākusmīya rows that wait on it. """ + PTR)
add(S + '2026-10-02-curadi-gana-10d-design.md', """brings 6.1.54, 7.3.36 and its optional ṇic.""", """brings 6.1.54, 7.3.36 and its optional ṇic. """ + PTR)
add(S + '2026-10-02-curadi-gana-10e-design.md', """  That slice takes √gṛ (`10.0231`), √yu (`10.0235`) and √ci (`10.0124`).
- A causative""", """  That slice takes √gṛ (`10.0231`), √yu (`10.0235`) and √ci (`10.0124`).
  """ + PTR + """
- A causative""")
add(S + '2026-10-02-curadi-gana-10f-design.md', """  √ci's optional ṇic is 2570, and it will reuse this slice's mechanism.""",
"""  √ci's optional ṇic is 2570, and it will reuse this slice's mechanism.
  """ + PTR)
add(S + '2026-10-03-curadi-gana-10g-design.md', """  are curation; √ci also needs 6.1.54 and 7.3.36.""",
"""  are curation; √ci also needs 6.1.54 and 7.3.36. """ + PTR)
add(S + '2026-10-03-curadi-gana-10h-design.md', """  curation slices. √ci (`10.0124`) also needs 6.1.54 and 7.3.36.""",
"""  curation slices. √ci (`10.0124`) also needs 6.1.54 and 7.3.36.
  """ + PTR)
add(S + '2026-10-04-curadi-gana-10i-design.md', """  in. √ci (`10.0124`) also needs 6.1.54 and 7.3.36.""",
"""  in. √ci (`10.0124`) also needs 6.1.54 and 7.3.36.
  """ + PTR)

out, bad = {}, []
for path, edits in E.items():
    s = open(path).read()
    for old, new in edits:
        n = s.count(old)
        if n != 1:
            bad.append(f"{path}: {n}x {old[:70]!r}")
            continue
        s = s.replace(old, new)
    out[path] = s
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
for path, s in out.items():
    open(path, 'w').write(s)
print(f"applied {sum(len(e) for e in E.values())} edits")
```

```bash
python3 /tmp/vidyut-full/slice10j/docsweep_10j.py "$(date -u +%F)"      # applied 60 edits
mise run fmt
```

If an `old` is not found exactly once, a paragraph drifted since the prototype: edit that paragraph to the same facts by hand rather than skip it, and say so in the commit message.

- [ ] **Step 4: Sweep for anything left stale**

```bash
grep -rnE "\b(422|25956|36416|10460|2884|1047|299|159|351)\b|\b319\b|\b180\b|\b71 ātmanepada|\b260\b|six nine-form|Six cells hold" --include=*.md --include=*.rs . \
  | grep -v "paradigm/data/\|^./target\|docs/superpowers/plans\|mutants.out\|specs/2026-0[89]\|specs/2026-10-0[1-3]\|10i-design"
grep -rn "not yet curated\|must revisit" crates --include=*.rs | grep -i "ci\b\|ciY\|√ci"
grep -rn -i "three hundred nineteen\|319 curādi" --include=*.rs --include=*.md . | grep -v "^./target"
```

Expected from the first grep, every hit a history line or Task 5's, and nothing else:
- `docs/ARCHITECTURE.md`: the pin history (`159 total — then curādi 10j's three`).
- `AGENTS.md`: the mutation floor paragraph (`25956 cells`, Task 5 rewrites it), its cap history (`260 from 10d`), the 10i record's spans (`351:16`, `351:45`), the progress line (`at 319 after slice 10i`), the audit chain (10i's `25956 cells / 36416 forms / 422 roots` before 10j's) and the juhotyādi 3f note.
- `crates/panini/tests/paradigm/main.rs`: the audit chain's 10i entry, the 10i census paragraph (`OPEN at 319`), and the pada-ambiguous account (`839 to 1047`, `1047 to 1063`).
- `tools/audit/README.md`: the new record's arithmetic (`422 + 8`, `25956 + 468`, `36416 + 468`, `10460 → 10646`, `36416 live`) and the 10i record.
- This slice's spec: its Evidence and Documentation lines.

The second grep prints nothing. The third prints only this slice's spec (its sweep-grep bullet quotes "319 curādi").

- [ ] **Step 5: Run the full suite and commit**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
git branch --show-current      # curadi-10j
git add -A
git commit -m "docs: 10j's counts, the audit record, and the sweep

26424 cells / 37070 forms / 430 roots across README, ARCHITECTURE, AGENTS,
paradigm/main.rs and tools/audit; curādi open at 327/509; 304 both-pada
roots; 1063 pada-ambiguous surfaces; 162 pinned rule ids. Audit at zero
divergence against 8da2f90b with 6516 blocked branches; prior traces
byte-identical to main. The 10b–10i specs point at 10j for √smiṅ, √gṛ,
√yu and √ci."
```

---

## Task 5: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and the current-record paragraph); `mise.toml` if the cap moves

Follow AGENTS.md's cargo-mutants protocol. Hazards from this repo's record:
- **Measure, never scale.** The cap must exceed a full uncaught suite run at the parallelism used, under campaign load, and twice the slowest caught phase (the `skip_nic` blow-up, AGENTS.md's floor paragraph).
- **Every invocation rotates `mutants.out`**, so always pass `-o`, and copy `outcomes.json` durably before any other invocation.
- **The mise shim fails.** Use the real binary: `CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`.
- **`pgrep -f` matches its own shell.** Wait on `pgrep -x cargo-mutants`.
- **Background shells die at about 60 minutes.** Launch detached with `setsid nohup`, as below, and resume with `--iterate` if it dies.

The prototype measured these, which are the values to expect:
- **The mutant list grows by thirteen.** `panini-prakriya` lists 863 mutants (850 on main); `panini-analyze` 12 and `panini-data` 12, both unchanged. By name (spans ignored) nothing leaves; the thirteen that arrive are 2567's guard (`samjna.rs`: three `delete !`, one `!=` → `==`, three `||` → `&&`), 6.1.54's (`sanadi.rs`: two `delete !`, one `||` → `&&`) and 7.3.36's (two `delete !`, one `||` → `&&`).
- **`--in-diff` over the slice's production diff for `panini-prakriya`** lists 17: those thirteen, 10.0493's moved guard (one `delete !`) and 6.4.92's moved guard (two `delete !`, one `||` → `&&`). The prototype ran all 17: **every one CAUGHT**. Test phases under load 58–82: 50–133 s for fourteen, and 443–498 s for 2567's three `||` → `&&` mutants (`samjna.rs:77:17`, `78:17`, `79:17`), which let 2567 credit every curādi cell before a test fails. All are far under the 5140 cap; record their campaign-load phases in Step 5 all the same.
- **`--in-diff` over the data crate's diff** lists none: the slice changes only constants, tests and docs there.
- **The documented non-caught entries.** The two equivalents are at `adesha.rs:649:30` and `tripadi.rs:1305:38`, the permanent hang at `tripadi.rs:1618:23`, on main and on this branch alike. AGENTS.md's current record was taken at `2f79dd0`, before 10i's final-review commit `d5b225c`, which moved several spans on main: `adesha.rs` +2 lines from 434 on (nine unviables and the equivalent, 647 → 649), `terms.rs` +1 (two unviables, 121 → 122 and 156 → 157), and `sanadi_pratyaya`'s unviable 71 → 74. This slice moves that unviable on to 76, and `skip_nic` from 55 to 57. Confirm `<A>`, `<T1>` and `<T2>` by `--list`; never compute them.
- **The `skip_nic` mutants move to `sanadi.rs:57`** (from 55 on main, 52 in AGENTS.md's record: the import and the 10.0493 move push them down). 6.1.54 is a new vikalpa, but keyed on one row, so `skip_nic -> true` gains only √ci's extra fork; re-measure it alone anyway, as AGENTS.md requires whenever a vikalpa is added.
- **The floor at 26424 cells** was not measured on a quiet host. Measure it in Step 1. The cap in force is 5140.

- [ ] **Step 1: Measure the floor**

With nothing else of ours running, run this twice: `cat /proc/loadavg; time mise run test >/dev/null 2>&1; cat /proc/loadavg` (foreground, timeout 600000 ms). Record both wall clocks, user CPU and the load averages. Read 10i's floor from AGENTS.md's floor paragraph and keep the comparison chain.

- [ ] **Step 2: Locate and probe the uncaught equivalents and the `skip_nic` pair**

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
SCRATCH="$(mktemp -d)"
mise exec -- "$CM" mutants --package panini-prakriya --list -o "$SCRATCH" 2>/dev/null | wc -l      # 863
mise exec -- "$CM" mutants --package panini-prakriya --list -o "$SCRATCH" 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /=|skip_nic -> bool with true|in skip_nic"
```

Expect `649` (adesha), `1305` (tripadi, inside 8.3.13's `apply`), `1618` (the ṇatva hang), and `sanadi.rs:57:5` (`skip_nic -> bool with true`) and `57:39` (`!=` → `==` in `skip_nic`). Write them as `<A>`, `<T1>`, `<T2>`, `<K>` and `<K2>`. Then run, detached with a log and an `EXIT_CODE=` sentinel, waiting on its PID in the same turn:

```bash
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 5140 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:<A>:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" \
  --re "sanadi.rs:<K>:5: replace skip_nic -> bool with true" --re "sanadi.rs:<K2>:39: replace != with == in skip_nic" \
  > "$SCRATCH/probe.log" 2>&1; echo "EXIT_CODE=$?" >> "$SCRATCH/probe.log"
```

The regexes also match caught `mod.rs` `derive` mutants; that is expected. Both equivalents must be MISSED, not TIMEOUT; both `skip_nic` mutants must be CAUGHT. Read each test-phase duration from `$SCRATCH/mutants.out/outcomes.json`. Set the provisional cap to max(5140, 6 × the longer equivalent, 2 × the longer `skip_nic` phase), rounded up to the next 10 s.

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/curadi-10j"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout <CAP> -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"; cat /proc/loadavg > "$OUT/load.started"
```

`<CAP>` is Step 2's provisional cap. Run nothing CPU-heavy meanwhile. 10i's campaign took 2h53m at 25956 cells and a 1810 cap; at 5140 the permanent hang alone holds one `-j 4` slot for 86 minutes, so expect longer. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

If the process dies before finishing, copy `$OUT/mutants.out/outcomes.json` aside, then resume with the same command plus `--iterate`. That re-tests only what has no outcome yet.

- [ ] **Step 4: Read the outcomes**

When `pgrep -x cargo-mutants` returns nothing:

```bash
date -u +"%F %T UTC" > "$OUT/finished"; cat /proc/loadavg > "$OUT/load.finished"
cp "$OUT/mutants.out/outcomes.json" "$OUT/outcomes.durable.json"
tail -5 "$OUT/campaign.log"
cat "$OUT/mutants.out/missed.txt" "$OUT/mutants.out/timeout.txt"
```

Expected (exit code 3 is normal when a timeout is present):
- **875 mutants: 823 caught, 49 unviable, 2 missed, 1 timeout.**
  - panini-prakriya: 863 / 815 / 45 / 2 / 1.
  - panini-analyze: 12 / 8 / 4 / 0 / 0.
- `missed.txt` holds exactly `adesha.rs:<A>:30: replace + with *` and `tripadi.rs:<T1>:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:<T2>:23: replace -= with /=`. (10i's campaign also timed out `skip_nic -> true` at its 1810 cap; at 5140 it should be CAUGHT.)

Then diff the non-caught set against 10i's on the full record, both with and without span lines:

```bash
python3 - "$OUT/outcomes.durable.json" /home/dev/mutants-records/curadi-10i/outcomes.durable.json <<'PY'
import json, sys
def non(p, lines):
    out = set()
    for o in json.load(open(p))["outcomes"]:
        sc = o["scenario"]
        if isinstance(sc, dict) and "Mutant" in sc and o["summary"] != "CaughtMutant":
            m = sc["Mutant"]; s = m["span"]["start"]
            out.add((m["package"], m["file"], s["line"] if lines else None, s["column"], m["replacement"], (m.get("function") or {}).get("function_name"), m["genre"], o["summary"]))
    return out
for lines in (False, True):
    a, b = non(sys.argv[1], lines), non(sys.argv[2], lines)
    print("lines" if lines else "no lines", len(a), len(b)); print(" new:", sorted(a - b)); print(" gone:", sorted(b - a))
PY
```

Expected (computed from 10i's `outcomes.durable.json` and this branch's `--list`):
- **Without lines:** `25 26`, `new: []`, `gone:` exactly 10i's `skip_nic` `true` Timeout, CAUGHT now under the 5140 cap.
- **With lines:** `52 53`; `new:` thirteen entries, `gone:` fourteen — the same thirteen at their 10i spans, plus the `skip_nic` Timeout. The thirteen are the moves above: nine `adesha.rs` unviables and the `adesha.rs` equivalent (+2, `d5b225c`), the two `terms.rs` unviables (+1, `d5b225c`), and `sanadi_pratyaya`'s unviable (71 → 76: `d5b225c`'s +3 and this slice's +2). No other span moves. Write down exactly what prints.

If not:
- Any **other timeout** is a suspect survivor that the larger suite pushed past the cap. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant among the new ones is a gap in Task 2's tests; add the test that kills it. Any other missed mutant means a test that caught it at 25956 cells no longer does; stop and report.

**Step 4b: the data-crate mutants.** Confirm the slice's diff for `panini-data` holds none:

```bash
git diff 1b4b9ff -- crates/panini-data/src/lib.rs > "$OUT/data.diff"
mise exec -- "$CM" mutants --package panini-data --list --in-diff "$OUT/data.diff" -o "$OUT/data" 2>&1 | tail -1      # No mutants to filter
```

- [ ] **Step 5: Margins**

```bash
python3 - "$OUT/outcomes.durable.json" <<'PY'
import json, statistics, sys
oc = json.load(open(sys.argv[1]))["outcomes"]
def test_phase(o):
    return next((p["duration"] for p in o["phase_results"] if p["phase"] == "Test"), None)
caught = sorted(t for o in oc if o["summary"] == "CaughtMutant" and (t := test_phase(o)) is not None)
print("caught", len(caught), "min", caught[0], "median", statistics.median(caught), "p90", caught[int(0.9 * len(caught))], "max", caught[-1])
for o in oc:
    if o["summary"] in ("MissedMutant", "Timeout") or (o["summary"] == "CaughtMutant" and (test_phase(o) or 0) > 600):
        m = o["scenario"]["Mutant"]; print(o["summary"], m["file"], m["span"]["start"]["line"], m["replacement"], test_phase(o))
PY
```

The cap is max(430, 6 × the longest campaign-load equivalent phase, 2 × the longest campaign-load caught phase), rounded up to the next 10 s.
- If that is 5140 or less, the cap stays 5140.
- Otherwise change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together (`grep -n "5140" AGENTS.md mise.toml`).

- [ ] **Step 6: Record it in AGENTS.md**

- **The floor paragraph.** Rewrite the paragraph that opens `**The floor behind the 5140s cap, measured at 25956 cells on Rust 1.99.0,`. Use Step 1's and Step 2's numbers at 26424 cells, the load averages, and the cap Step 5 chose. Keep the comparison chain to earlier floors, with 10i's joining it: 2m16.234s / 2m12.659s at 25956 cells, isolated probe 188.69s / 188.85s, campaign-load 208.34s / 186.62s, `skip_nic` `true` 1559.33s alone and `!=` → `==` 1762.02s in the campaign.
- **The current record.** Replace the `**Current record (curādi 10i, …).**` paragraph with `**Current record (curādi 10j, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**;
  - the non-caught set diffed against 10i's on the full record, both ways (Step 4's script output), naming what moved and why (the `d5b225c` span; the `skip_nic` timeout now caught);
  - the thirteen new mutants, every one CAUGHT, by name, and the four moved ones;
  - the `skip_nic` pair's phases;
  - Step 4b's empty data-crate list;
  - the campaign-load phases and margins;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`.

  End it with a pointer to the record it replaces. Run `git rev-parse --short HEAD` before committing, and write ``The curādi 10i record it replaces: `git show <that hash>:AGENTS.md`.``

- [ ] **Step 7: Commit**

```bash
git branch --show-current      # curadi-10j
git add AGENTS.md mise.toml
git commit -m "chore: 10j mutation gate — floor and uncaught run re-measured at 26424 cells

Every new mutant caught; missed.txt holds only the two documented
equivalents and timeout.txt only the permanent ṇatva-scan entry. The
skip_nic blow-up, a timeout at 10i's 1810 cap, is caught under 5140."
```

---

## Task 6: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # back at /workspace/crates
git branch --show-current      # curadi-10j
git log --oneline main..HEAD   # the spec commits, the plan and the four task commits
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin curadi-10j
gh pr create --title "curādi 10j — the eight ajanta rows: 6.1.54, 7.3.36 puk, Kaumudī 2567" --body "$(cat <<'BODY'
Slice 10j curates curādi's eight ajanta rows — √smiṅ (`10.0058`), √ci
(`10.0124`), √ghṛ (`10.0152`), √gṛ (`10.0231`), √yu (`10.0235`), √jñā
(`10.0258`), √cyu (`10.0275`) and √bhū (`10.0277`) — closing the ākusmīya
and the jñapādi.

- 6.1.54 *cisphuror ṇau* gives √ci its optional `A` before ṇic, keyed on
  the row (`samjna::CISPHUR`): `10.0325 ci` stores as the same code.
- 7.3.36's puk appends `p` to an ā-final aṅga before ṇic (*jñāpayati*, and
  √ci's 6.1.54 branch), ahead of 7.2.115, which then declines.
- Kaumudī 2567 keeps √smiṅ ātmanepadī under ṇic (*smāyayate*); a curādi
  `Atmanepada` row must be ṅit, which the data test holds.
- 10.0493 moves to the head of the sanādi stage, where vidyut credits it,
  and 6.4.92 after the sanādi 6.1.78, so √ci's three readings (*cayayati*,
  *capayati*, *cayati*) come out right.

The golden suite goes from 25956 to 26424 cells; curādi is open at 327 of
509. The audit shows zero divergence against `8da2f90b`, a main-vs-branch
dump of every prior branch (blocked ones included) is byte-identical, and
the mutation gate finds every new mutant caught.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`:
   - run `git worktree remove .worktrees/curadi-10j`;
   - run `git worktree remove --force .worktrees/curadi-10j-proto` and `git branch -D proto-10j proto-10j-t3final` (the throwaway);
   - run `git worktree remove --force .worktrees/curadi-10j-replay` if the replay worktree is still there (detached; no branch);
   - run `git worktree remove --force /tmp/claude-1000/-workspace/ab7ae8d8-531a-4aca-a8a2-0d18eb7b5bf4/scratchpad/main-10j` if it is still registered (the brainstorm's detached main dump; `git worktree prune` if the path is gone);
   - delete the local and remote `curadi-10j` branch;
   - run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| 8 rows, codes, padas; `OPTIONAL_NIC` + √ci by 2570 | 3 Step 3 |
| 7.3.36 appends `p`, before 7.2.115; ā-final arm only | 2 |
| 6.1.54 vikalpa keyed by row (`CISPHUR`), pinned to `ciY` upstream; placement after 6.4.48; branch order | 2; 3 Step 1 |
| 6.4.92 after the sanādi 6.1.78, comment rewritten | 2 |
| 10.0493 at the head of `SANADI` | 2 |
| 2567 in `SAMJNA` before 1.3.12, tag guard, ātmanepada only; ṅit-ness held by the data test; the Kaumudī's wording | 2; 3 Step 1 |
| √gṛ, √yu ākusmīya; stored codes need nothing new | 3 Steps 3, 7 |
| Homographs (`capayati`, `cayati`, `BAvayati`) and the negative witnesses that become forms | 3 Steps 1, 7 |
| Goldens via the generator; census 430 / 26424 / 37070; blocked 6516; forms-per-cell; ALTERNATES 10646 and keys | 3 Steps 1, 4–7; 4 Step 2 |
| Other counts: row count 430, 1.3.74 +5 (264 `Nic` + 7 `NicUbhayapada`), pada-ambiguous re-derived (1063) | 3 Steps 1, 6 |
| Rule pins: order (+6.1.54, 7.3.36, 2567; two moves), vikalpa (+6.1.54), `the_optional_nic_ids_…` (+`10.0124`) | 2; 3 Step 1 |
| Data tests: `optional_nic_matches_…` (181), the marker test's ṅit arm, the jñapādi comment, the 6.1.54 key pin | 2; 3 Step 1 |
| Unit tests: 7.3.36, 6.1.54, 6.4.92 at its new position, 2567, 7.2.115 on `jYAp` | 2 |
| Corpus-wide: 6.1.54 / 7.3.36 / 2567 fire only on their rows; 7.2.115, sanādi 6.1.78, 6.4.92, 10.0493 rosters | 3 Step 1 |
| Prior-trace diff main ↔ HEAD | 4 Step 2 |
| Witnesses from the spec's tables, per row, per √ci branch, per blocked pada, per homograph | 3 Steps 1, 7 |
| Mutation gate: scoped, floor re-measured, `skip_nic` alone, `-o`, durable copy, chunking, non-caught named verbatim | 5 |
| AGENTS.md, ARCHITECTURE, `tools/audit/README.md`, `JNAPADI` doc, sanādi and samjna module docs, earlier specs' pointers, sweep greps | 2; 3 Step 1; 4 Steps 1, 3, 4; 5 |

**Type consistency.**
- `CISPHUR: [&str; 1]`, read as `CISPHUR.contains(&p.ctx.dhatupatha)` in `sanadi.rs` (`use crate::tinanta::samjna::CISPHUR`) and compared with an array literal in its pin.
- 6.1.54, 7.3.36 and 2567 are closures in `Rule { apply: |p| … }`, as every rule in both stages; 2567 compares `p.ctx.pada != Pada::Atmanepada` (`Pada: PartialEq`).
- `opens_nicless(ids: &[&str], id: &str) -> bool` uses `<[&str]>::strip_prefix(&["10.0493"][..])`; both callers pass `&ids` built as `Vec<&str>`.
- `sanadi_ids(p: &Prakriya) -> Vec<&str>` borrows the log; the √ci test collects `(String, Vec<&str>)` tuples and compares against array literals of the same shape.
- `curadi_analyses_its_ajanta_forms`' tuples are `(&str, &[&str], Pada, &[&str])`; the first tuple's `[..]` on both slices fixes the types.
- The golden tuple shapes match `ParadigmRow` and `AlternateRow`; the generator is 10i's, with the row selection and the vikalpa list changed.

**Known soft spots.**
- **Doc strings in Task 4** were read at the prototype's state. If the script reports an `old` not found exactly once, edit that paragraph to the same facts rather than skip it.
- **Task 5's campaign numbers are expectations**, not prototype measurements; the in-diff run above is the one measured part. The floor and cap depend on host load; record the load beside every timing. The Step 4 diff expectation is the least certain line in the plan: write down what prints.
- **Where this plan goes beyond the spec's letter**, each to make the spec's decisions work:
  - **6.1.54's key is a `samjna` const (`CISPHUR`)**, beside `MRJ` and `GHU`, the spec's first option.
  - **`opens_nicless`** replaces two tests' "first credit is the optional-ṇic id" with "first, or second after 10.0493": the 10.0493 move puts the mit credit ahead of 2570 on √ci.
  - **`rows_crediting`'s window** gains 2567 among the pada rules that open the stage after sanādi.
  - **The marker test pins upstream** that `10.0058` is the one ṅit curādi row, so 2567's data-side guard reaches nothing else.
  - **The row-list test is renamed** (`…_three_hundred_twenty_seven_…`), since its name stops being true.
  - **The 10.0496 comment's "First in the stage"**, stale since 10f, is corrected where the 10.0493 move rewrites its neighbour; so is ARCHITECTURE's "at the head of `sanadi.rs`" for 10.0496.
  - **The mit-rule credit test's "144 prior roots"**, stale since 10e, becomes "every other root".
- **The pada-ambiguous count's arithmetic** (16 = 4 × 4) is derived after the fact. The set itself is the measured one, hash-pinned by the script.
