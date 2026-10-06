# Curādi gaṇa slice 10l Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate eight curādi rows that bring rules of their own: `10.0023 urja~` (√ūrj), `10.0026 curRa~` (√cūrṇ), `10.0037 adwa~` (√aṭṭ), `10.0155 kFta~` (√kṝt), `10.0170 mleCa~` (√mlecch), `10.0180 gurda~` (√gūrd), `10.0270 u~Drasa~` (√dhras) and `10.0278 kfpa~` (√kṛp). They bring four new rules (6.1.75 *dīrghāt*, 7.1.101 *upadhāyāś ca*, 8.2.18 *kṛpo ro laḥ*, 8.2.78 *upadhāyāṃ ca*), a second arm for 8.4.41 *ṣṭunā ṣṭuḥ*, and one `OPTIONAL_NIC` entry. The golden suite goes from 37584 to 38160 cells, and curādi from 482 to 490 of its 509 rows.

**Architecture:** Six tasks:
- **Task 1** creates the worktree and checks the baseline.
- **Task 2** is the engine, green on its own. Nothing before this slice reaches any of the new rules: the prototype's main-vs-branch trace dump (below) shows every prior branch byte-identical. It adds:
  - 6.1.75 and 7.1.101 to `SANADI`, right after the sanādi 6.1.73 and before 7.3.86. 6.1.75 shares 6.1.73's tuk scan through a new `anga::tuk_before_che`;
  - 8.2.18 first in `TRIPADI`, keyed by row through a new `samjna::KRP` (the `CISPHUR` precedent);
  - 8.2.78 right after 8.2.77, reading the root inside a ṇijanta aṅga by stripping ṇic's `ay`;
  - 8.4.41's stu-before-ṭu arm, through a new `sound::shtutva_of`.
  The tests go in first and fail.
- **Task 3** lands the eight rows, √dhras's `OPTIONAL_NIC` entry and the initial-`u~` reading, 64 golden rows and 90 alternates, and every count, list, roster and `check()` assertion they move. The assertions go in first and fail; the rows and goldens make them pass.
- **Tasks 4–6** are the audit with the prior-trace diff and the doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.99.0, pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, checked out at `/tmp/vidyut-full`.

**Spec:** `docs/superpowers/specs/2026-10-05-curadi-gana-10l-design.md`. Its Scope table is the slice's scope. Its Decisions section explains:
- why 7.1.101 (not 7.1.100) runs before 7.3.86;
- why 6.1.75 is a sanādi entry only, sharing 6.1.73's scan;
- why 8.2.18 is keyed by row (`10.0408 kfpa` is another root);
- how 8.2.78 reads the root through ṇic's `ay`;
- 8.4.41's second arm and its full table;
- the initial `u~`, the one homograph, and the trailing space in `10.0026`'s artha.

**Workspace:** The spec and this plan are on the branch `curadi-10l`, which is checked out at `/workspace`. Task 1 puts `/workspace` back on `main` and checks `curadi-10l` out at `/workspace/.worktrees/curadi-10l`. Every path below is relative to that directory unless it starts with `/`.

**Provenance.** This slice was built end to end on a throwaway worktree, `/workspace/.worktrees/curadi-10l-proto` (branch `proto-10l`, on the spec commit `056e7f5`). Its throwaway commits, on branch `proto-10l-v2`, are `266e1c2` (Task 2's tests), `64a35ad` (engine), `71f83f2` (Task 3's assertions), `f005568` (rows and goldens) and `2a67ade` (audit and docs), and Task 6 deletes it. Every script below was generated from those commits' diffs (`/tmp/vidyut-full/slice10l/tools/diff2script.py`, a verification aid this plan does not need), or written and run against them, and was checked three ways:
- **Prototype checks.** The prototype's final state:
  - passed the full suite, clippy `-D warnings` and `fmt-check`;
  - its generator found all 576 new cells equal to vidyut's (`576 cells, 666 forms, 0 differences`);
  - the audit at 593 roots / 38160 cells / 49826 forms showed zero differences, with 6552 blocked branches and the `entry` control failing on 36 cells;
  - a main-vs-prototype dump of every prior branch, blocked ones included (55676 lines, 49160 of them live), was byte-identical.
- **Replay.** A replay ran the scripts below in this plan's order on a fresh worktree at `056e7f5` (`/workspace/.worktrees/curadi-10l-replay`, detached) and reproduced the prototype's tree after each of the five commits, byte for byte in every tracked file, the regenerated goldens' hashes included. Task 2 Step 2's and Task 3 Step 2's failing lists below are the prototype's at those identical trees.
- **Negative controls.** In the brainstorm's scoping prototype, switching each rule off in turn made the audit fail: 72 cells each for 6.1.75, 7.1.101, 8.2.18 and 8.4.41's new arm, and 270 for 8.2.78 (`urj`'s 54, and 72 each for `curR`, `gurd` and `kFt`).
- **Mutants.** The 51 mutants `--in-diff` lists for the slice's `panini-prakriya` diff were run on the prototype (`-j 4 --timeout 9240`, 29 minutes, load 17–79): 47 caught, 3 unviable and **1 missed**, `tripadi.rs:220:29: replace - with /` (8.2.78's `c[n - 2..]`): every root the 8.2.78 test then used had three or four letters, where `n - 2` is `n / 2`. The prototype was rebuilt with a five-letter witness (`stirpay`) in that test, and the mutant re-run alone was CAUGHT. The three unviable are 8.2.78's `+` → `-` and `+` → `*` on `String + &str` (`tripadi.rs:222:17`) and 8.4.41's let-chain `&&` → `||` (`tripadi.rs:1284:21`), which does not compile.

The prototype did **not** run the full mutation campaign or the `skip_nic` probe. Task 5's campaign numbers are expectations, derived from the measured mutant lists and the in-diff run above, and the campaign measures them.

**Throwaway scripts.** Everything under `/tmp/vidyut-full/slice10l/` and the two vidyut examples (`curadi_goldens_10l.rs`, `trace_dump_10l.rs`) never ship. Each is reproduced in full in this plan with its sha256, so it can be recreated if `/tmp` was cleaned. Recreate a file only if it is missing, and check its hash either way (`sha256sum <file>`). The scripts read nothing but the worktree, except `pin_ambiguous_10l.py` (the failing test's output) and `insert_goldens_10l.py` (the generator's output).

## Global Constraints

- **The new grammar is exactly this:**
  - **ids:** 6.1.75 and 7.1.101 (`SANADI`, in that order, right after the sanādi 6.1.73 and before 7.3.86); 8.2.18 (first in `TRIPADI`); 8.2.78 (`TRIPADI`, right after 8.2.77). The rule-order pin goes from 162 to 166.
  - **names** (vidyut's `data/sutrapatha.tsv`): 6.1.75 `"dIrGAt"`; 7.1.101 `"upaDAyASca"`; 8.2.18 `"kfpo ro laH"`; 8.2.78 `"upaDAyAM ca"`.
  - **consts and helpers:** `samjna::KRP: [&str; 1] = ["10.0278"]`, `pub(crate)`, read by 8.2.18; `sound::shtutva_of(char) -> Option<char>`; `anga::dirghat` and the private `anga::tuk_before_che(p, vowel: fn(char) -> bool, sutra, name)`, which `che_ca` now calls.
  - **8.4.41** gains the stu-before-ṭu arm (trigger `w W q Q R`, not `z`); its ṣṭu-first arm is unchanged.
  - **vikalpa:** none of the new rules. No rule has new `bars`.
  Nothing else in the engine changes.
- **Rows:** exactly the spec's eight, appended to `DHATUS` after 10k's in dhātupāṭha order, each `pada: PadaAssignment::Nic`, each `code` what `stored_form` computes. `10.0026`'s artha is `"preraRe "`, with upstream's trailing space. `OPTIONAL_NIC` gains exactly `("10.0270", "2570")`, after `10.0267` (182 entries), and `optional_nic_from_upadesha`'s 2570 arm reads an initial `u~` too.
- **Not modelled:** an aṅga-stage 6.1.75 (no laṅ aṭ is long); a guṇa-stage 7.1.101 (no other curated root has an `F` upadhā); 8.4.43 *toḥ ṣi* as a rule (it is the new arm's trigger set); 1.1.51, which writes `ir` uncredited, as 7.1.102's `ur` is.
- **Out of scope:** `10.0175 picca~` and the 8.2.30 narrowing (slice 10m), `10.0368 za\da~` and upasargas, the causative.
- **Pre-existing cells must stay byte-identical, traces included,** with no exception. Regenerate no prior golden.
- **Goldens come from the generator, which asserts engine = vidyut cell by cell, and their sha256 must match this plan's.** **Do not edit a golden to match the engine.** If the generator reports a difference, or a hash differs, stop and report.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit. Run `git branch --show-current` before every commit and the push: it must print `curadi-10l` (a detached HEAD strands commits).
- `mise run test` took 3–5 minutes in the prototype; the `trace` binary alone ran 93–183 s. Run it in the **foreground** with a timeout of 600000 ms; if a run outgrows the 10-minute cap, start it detached with its output in a log file and an `EXIT_CODE=` sentinel, then wait in the same turn with `while kill -0 <PID>; do sleep 30; done`. Never background it and end a turn, never arm a Monitor and end a turn, and never pipe it through `tail`.
- `mise run test -- -p X` does not scope. Scope with `mise exec -- cargo test -p <crate> <filter>`. To see every failing binary at once, use `mise exec -- cargo test --workspace --no-fail-fast`.
- The `cargo-mutants` mise shim fails here. Use the real binary, `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`, under `mise exec --`.
- `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes its `panini` and `panini-data` dev-deps. Repoint them at this worktree before generating or auditing, and back at `/workspace/crates` after. Check with `grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml` every time.
- Never wait on a process with `pgrep -f` (it matches its own shell); use `pgrep -x cargo-mutants` or `kill -0 <pid>`.
- Review packages and diffs exclude the appended goldens (`crates/panini/tests/paradigm/data/curadi.rs`) and use `git diff --diff-algorithm=histogram`: the default algorithm shows appended golden rows as fake deletions.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **8.2.18 on the other `kfp`.** `10.0408 kfpa` (adanta, 10e) is a different upadeśa from `kfpa~`, and a guard on text would rewrite its `r`-less aṅga or a future `kfp`-shaped row. → Task 2 `krpo_ro_lah_turns_krps_r_to_l_on_its_row_only` (declines on `10.0408` and on no row) and `krp_is_the_row_8_2_18_names` (the key pinned to the upstream upadeśa `kfpa~`, `10.0408`'s `kfpa`, and the two uncurated bhvādi rows); Task 3 `the_10l_rules_fire_only_on_their_rows` (8.2.18 on `10.0278` alone, corpus-wide).
2. **8.2.78 reading the wrong span of a ṇijanta aṅga.** Read whole, `urjay` ends `jay` and the rule never fires; read without the ṇijanta check, a plain aṅga ending in `ay` loses its last two letters. A prior ṇijanta aṅga not ending in `ay` would panic on the `expect`. → Task 2 `upadhayam_ca_lengthens_the_ik_before_a_roots_r_or_v_upadha` (all four ik arms through `ay`, the `v` upadhā, a five-letter root where `n - 2` is not `n / 2`, a plain `urj`, and a non-ṇijanta `urjay` that must decline) and `the_10l_rules_give_vidyuts_forms` (laṅ *aurjayat* declines); Task 3's full suite (every ṇijanta cell reaches the `expect`) and `the_10l_rules_fire_only_on_their_rows` (exactly four rows); Task 4's prior-trace diff.
3. **8.4.41's new arm on prior rows.** `n`/`s`/`t` before a ṭu is common in curādi root text (`kuww`, `lunw`, `Gaww`), most of it already retroflex in storage, and a new firing on a prior row would add a step no golden sees. → Task 2 `shtutva_retroflexes_a_stu_before_a_tu_too` (every trigger, every target, `z` excluded, adjacency) and `shtutva_of_all_arms`; Task 3 `the_10l_rules_fire_only_on_their_rows` (8.4.41's whole roster: the five rudhādi/juhotyādi rows plus `10.0037`) and `khari_ca_off_bhas_is_credited_exactly_as_before_3f2` (8.4.55's off-√bhas count unchanged but for `10.0037`'s 78); Task 4's prior-trace diff.
4. **Tuk twice.** 6.1.75 runs after the sanādi 6.1.73; on `piC` or `viC` it must not add a second `t` (`pittC`), and on a root with a long vowel and a short one before two `C`s each rule must take its own. → Task 2 `dirghat_gives_mlecch_tuk_after_its_long_vowel` (declines on `mletC`, `viC`, a consonant and nothing before the `C`; every dīrgha); Task 3 `the_10l_rules_fire_only_on_their_rows` (6.1.75 on `10.0170` alone) and `shcutva_off_jan_is_credited_exactly_as_before_3f3` (8.4.40's curādi count gains exactly √mlecch's 78).
5. **The homograph and the optional ṇic.** *cūrṇayati* now has two analyses, and `trace_for` answers with the first (`10.0143`'s, `dhatus()` order); √dhras's *dhrasati* exists only on the ṇic-less branch. → Task 3 `curadi_analyses_its_10l_forms` (roots in analysis order, each reading's credits and absences, eleven ruled-out shapes Invalid) and `the_optional_nic_ids_are_credited_only_on_their_rows` (2570 on `10.0270` and every cell of it).

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-prakriya/src/tinanta/anga.rs` | Task 2: `tuk_before_che`, `che_ca` through it, `dirghat` |
| `crates/panini-prakriya/src/tinanta/sanadi.rs` | Task 2: import, 6.1.75, 7.1.101, two unit tests. Task 3: 2570's test row list gains `10.0270`. Task 4: module doc |
| `crates/panini-prakriya/src/tinanta/samjna.rs` | Task 2: `KRP` and its upstream pin. Task 3: the pin gains "curated" |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | Task 2: module doc range, imports, 8.2.18, 8.2.78, 8.4.41's arm and comments, the 8.4.40 comment, three unit tests |
| `crates/panini-prakriya/src/tinanta/sound.rs` | Task 2: `shtutva_of`, `shtutva_of_all_arms` |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Task 2: order pin (166) and its doc, `ajanta_row` renamed `curadi_row`, `the_10l_rules_give_vidyuts_forms` |
| `crates/panini-prakriya/src/tinanta/guna.rs` | Task 4: one comment's corpus size |
| `crates/panini-data/src/lib.rs` | Task 3: 8 rows, `10.0143`'s comment, `OPTIONAL_NIC` (182) and its doc, `optional_nic_from_upadesha`, `Dhatu::pada`'s and the marker doc's counts, the row count (593), the row-list test (renamed `…_four_hundred_ninety_…`) |
| `crates/panini/tests/paradigm/main.rs` | Task 3: census and keys and its 10l paragraph, the pada-ambiguous set (1631), `curadi_analyses_its_10l_forms`. Task 4: doc counts, the alternates doc, the audit chain |
| `crates/panini/tests/paradigm/data/curadi.rs` | Task 3: 64 golden rows, 90 alternates |
| `crates/panini/tests/trace/curadi.rs` | Task 3: the 1.3.74 count, 2570's rows, `the_10l_rules_fire_only_on_their_rows`. Task 4: module doc |
| `crates/panini/tests/trace/juhotyadi.rs` | Task 3: the 8.4.55 and 8.4.40 rosters |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, the 10k spec | Task 4 |
| `AGENTS.md`, maybe `mise.toml` | Task 5 |

---

## Task 1: The worktree and baseline

**Files:** none. **Interfaces:** none.

- [ ] **Step 1: Create the worktree**

```bash
cd /workspace
git status --short                   # empty
git branch --show-current            # curadi-10l
git log --oneline -3                 # the plan commit, the spec commit 056e7f5, then e47201c
git checkout main                    # curadi-10l can be checked out in one worktree only
git log --oneline -1                 # e47201c, the 10k merge
git worktree add .worktrees/curadi-10l curadi-10l
cd .worktrees/curadi-10l
git branch --show-current            # curadi-10l
```

`/workspace` stays on `main` for the whole slice: Task 4's prior-trace dump builds the main engine from `/workspace/crates`.

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: everything passes at 37584 cells. `panini-prakriya` reports 437 tests, the `trace` binary 215, `paradigm` 29 and `panini-data` 28 (and `panini` 7, `roundtrip` 1, `panini-analyze` 8, `cli` 5, `panini-lipi` 6).

---

## Task 2: The engine — 6.1.75, 7.1.101, 8.2.18, 8.2.78 and 8.4.41's second arm

No curated root before this slice has an `F` upadhā, a long vowel before a `C`, a row in `KRP`, a short ik before an `r`/`v` upadhā and a final hal, or a stu right before a ṭu once the tripādī reaches 8.4.41. So every golden and every prior trace is unchanged, and the suite is green at the end of the task. Task 4's prior-trace diff is the corpus-wide proof.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/anga.rs` (the shared tuk scan, `dirghat`)
- Modify: `crates/panini-prakriya/src/tinanta/sanadi.rs` (import; 6.1.75; 7.1.101; tests)
- Modify: `crates/panini-prakriya/src/tinanta/samjna.rs` (`KRP`; its pin)
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (module doc; imports; 8.2.18; 8.2.78; 8.4.41's arm and comments; tests)
- Modify: `crates/panini-prakriya/src/tinanta/sound.rs` (`shtutva_of`; its arms test)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (the order pin and its doc; `curadi_row`; the end-to-end test)

**Interfaces:**
- Produces:
  - `pub(crate) const samjna::KRP: [&str; 1] = ["10.0278"]`;
  - `pub(crate) fn sound::shtutva_of(c: char) -> Option<char>`;
  - `pub(crate) fn anga::dirghat(p: &mut Prakriya) -> bool`, and the private `fn tuk_before_che(p: &mut Prakriya, vowel: fn(char) -> bool, sutra: &str, name: &str) -> bool` behind it and `che_ca`;
  - the rule ids 6.1.75, 7.1.101, 8.2.18, 8.2.78;
  - the derivation-test helper `curadi_row(dhatupatha, code, pada) -> Dhatu` (10j's `ajanta_row`, renamed).
- Consumes:
  - the sanādi tests' `with_nic(root, nic)` and `rule(id)`; the samjna tests' `upstream_upadesha(number)`;
  - `derivation_tests`' `sole(branches)`, `declined(branches, n)` and `sanadi_ids(p)`; `derive`; `rules()`.

- [ ] **Step 1: The failing tests**

Create `/tmp/vidyut-full/slice10l/engine_tests_10l.py` if it is missing (sha256 `f7083e061ff82c25cdae87977de69306ec1b1ed3bc11f83533e0f105ba2b5f9f`). It adds:
- **`sanadi.rs`:** `dirghat_gives_mlecch_tuk_after_its_long_vowel` and `upadhayas_ca_makes_an_f_upadha_ir_before_nic_and_bars_its_guna`.
- **`samjna.rs`:** the `KRP` declaration (a declaration, no rule, so the tests compile) and `krp_is_the_row_8_2_18_names`.
- **`tripadi.rs`:** `krpo_ro_lah_turns_krps_r_to_l_on_its_row_only`, `upadhayam_ca_lengthens_the_ik_before_a_roots_r_or_v_upadha` and `shtutva_retroflexes_a_stu_before_a_tu_too`.
- **`derivation_tests.rs`:** `ajanta_row` renamed `curadi_row` (four uses) with its doc; the order pin (166) and its doc's 10l paragraph; `the_10l_rules_give_vidyuts_forms`.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10l, Task 2's failing tests — the unit tests for 6.1.75,
7.1.101, 8.2.18, 8.2.78 and 8.4.41's new arm, the KRP declaration and its
upstream pin, the end-to-end derivation test on hand-built rows, the
rule-order pin, and the hand-built-row helper renamed curadi_row.
Every `old` must occur exactly once in its file; nothing is written if one
fails. Run from the worktree root."""
import sys
E = {
'crates/panini-prakriya/src/tinanta/derivation_tests.rs': [
("""///
/// 6.4.106/6.4.107 sit BELOW 6.1.96 but ABOVE 6.1.90, against sūtra order
""",
"""///
/// Slice 10l adds 6.1.75 *dīrghāt* and 7.1.101 *upadhāyāś ca* right after
/// the sanādi 6.1.73, before 7.3.86, whose guṇa √kṝt's `ir` must block;
/// 8.2.18 *kṛpo ro laḥ* first in the tripādī, as vidyut runs it ahead of the
/// rest of 8.2; and 8.2.78 *upadhāyāṃ ca* right after 8.2.77.
///
/// 6.4.106/6.4.107 sit BELOW 6.1.96 but ABOVE 6.1.90, against sūtra order
"""),
("""        "7.2.116", "7.3.37.2", "7.3.36", "7.2.115", "6.1.78", "6.4.92", "7.2.114", "6.1.73",
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
""",
"""        "7.2.116", "7.3.37.2", "7.3.36", "7.2.115", "6.1.78", "6.4.92", "7.2.114", "6.1.73",
        "6.1.75", "7.1.101", "7.3.86", "3.1.32", "2567", "1.3.12", "1.3.66", "1.3.72", "1.3.74",
        "1.3.78", "3.4.78", "1.3.9", "1.2.4", "3.4.85", "3.4.108", "3.4.109", "3.4.105", "3.4.106",
        "3.4.101", "3.4.99", "3.4.87", "3.4.89", "3.4.86", "3.4.100", "3.4.80", "3.4.79", "3.4.91",
        "3.4.93", "3.4.90", "3.4.92", "3.4.103", "3.4.102", "7.1.35", "3.1.69", "3.1.73", "3.1.77",
        "3.1.78", "3.1.79", "3.1.81", "3.1.68", "2.4.72", "2.4.75", "3.4.111", "3.1.83", "1.2.4",
        "6.1.10", "7.4.66", "7.4.60", "7.4.59", "7.4.62", "7.4.75", "7.4.76", "7.4.77", "7.4.78",
        "6.4.78", "6.4.71", "6.4.72", "6.1.73", "7.3.100", "7.1.5", "7.1.6", "7.1.4", "7.1.3",
        "7.2.79", "7.2.80", "7.2.81", "6.4.23", "7.4.21", "7.3.83", "7.3.87", "7.2.114", "7.3.84",
        "7.3.86", "7.3.86", "7.3.92", "7.3.84", "7.1.102", "6.4.110", "6.4.108", "6.4.109",
        "6.4.87", "6.4.82", "6.4.77", "6.1.77", "6.1.78", "7.3.101", "6.4.119", "6.4.118",
        "6.4.117", "6.4.116", "6.4.113", "6.4.98", "6.4.100", "6.4.112", "6.4.115", "6.4.42",
        "6.4.43", "6.1.97", "6.1.101", "6.1.101", "6.1.96", "6.4.106", "6.4.107", "6.1.90",
        "6.1.88", "6.1.97", "6.1.87", "6.1.66", "6.4.105", "6.4.101", "6.4.111", "8.2.18",
        "8.2.77", "8.2.78", "8.2.23", "8.2.25", "8.2.26", "8.2.30", "8.2.31", "8.2.39", "8.2.40",
        "8.2.41", "8.2.74", "8.2.75", "8.2.73", "8.3.15", "8.3.24", "8.3.59", "8.4.40", "8.4.41",
        "8.3.13", "8.4.53", "8.4.54", "8.2.38", "8.4.55", "8.4.1", "8.4.2", "8.4.58", "8.4.65",
        "8.4.56",
    ];
"""),
("""
/// A curādi row slice 10j's rules reach, hand-built: `derive` reads only the
/// row number, the code, the gaṇa and the pada assignment, and the rules
/// under test need nothing else.
fn ajanta_row(dhatupatha: &'static str, code: &'static str, pada: PadaAssignment) -> Dhatu {
    Dhatu {
""",
"""
/// A curādi row a slice's rules reach, hand-built (10j's and 10l's):
/// `derive` reads only the row number, the code, the gaṇa and the pada
/// assignment, and the rules under test need nothing else.
fn curadi_row(dhatupatha: &'static str, code: &'static str, pada: PadaAssignment) -> Dhatu {
    Dhatu {
"""),
("""    // `OPTIONAL_NIC`) is filtered out: it never reaches 3.1.25.
    let ci = ajanta_row("10.0124", "ci", PadaAssignment::NicUbhayapada);
    let branches = derive(
""",
"""    // `OPTIONAL_NIC`) is filtered out: it never reaches 3.1.25.
    let ci = curadi_row("10.0124", "ci", PadaAssignment::NicUbhayapada);
    let branches = derive(
"""),
("""    // no 6.4.92, so the `A` stays long.
    let jna = ajanta_row("10.0258", "jYA", PadaAssignment::Nic);
    let p = sole(derive(
""",
"""    // no 6.4.92, so the `A` stays long.
    let jna = curadi_row("10.0258", "jYA", PadaAssignment::Nic);
    let p = sole(derive(
"""),
("""    // keeps it ātmanepadī and 1.3.12 credits, not 1.3.74.
    let smin = ajanta_row("10.0058", "smi", PadaAssignment::Atmanepada);
    let p = sole(derive(
""",
"""    // keeps it ātmanepadī and 1.3.12 credits, not 1.3.74.
    let smin = curadi_row("10.0058", "smi", PadaAssignment::Atmanepada);
    let p = sole(derive(
"""),
("""    assert!(p.blocked);
}
""",
"""    assert!(p.blocked);
}

#[test]
fn the_10l_rules_give_vidyuts_forms() {
    // Slice 10l's rows, hand-built, laṭ parasmaipada prathama eka: each form
    // is vidyut-prakriya's, and each rule is credited where vidyut credits
    // it (the sanādi stage, or the tripādī after 6.1.78).
    for (number, code, form, sanadi, tripadi) in [
        (
            "10.0023",
            "urj",
            "Urjayati",
            &["3.1.25", "1.3.9", "3.4.114", "3.1.32"][..],
            &["8.2.78"][..],
        ),
        (
            "10.0026",
            "curR",
            "cUrRayati",
            &["3.1.25", "1.3.9", "3.4.114", "3.1.32"],
            &["8.2.78"],
        ),
        (
            "10.0037",
            "adw",
            "awwayati",
            &["3.1.25", "1.3.9", "3.4.114", "3.1.32"],
            &["8.4.41", "8.4.55"],
        ),
        (
            "10.0155",
            "kFt",
            "kIrtayati",
            &["3.1.25", "1.3.9", "3.4.114", "7.1.101", "3.1.32"],
            &["8.2.78"],
        ),
        (
            "10.0170",
            "mleC",
            "mlecCayati",
            &["3.1.25", "1.3.9", "3.4.114", "6.1.75", "3.1.32"],
            &["8.4.40"],
        ),
        (
            "10.0180",
            "gurd",
            "gUrdayati",
            &["3.1.25", "1.3.9", "3.4.114", "3.1.32"],
            &["8.2.78"],
        ),
        (
            "10.0278",
            "kfp",
            "kalpayati",
            &["3.1.25", "1.3.9", "3.4.114", "7.3.86", "3.1.32"],
            &["8.2.18"],
        ),
    ] {
        let row = curadi_row(number, code, PadaAssignment::Nic);
        let p = sole(derive(
            &row,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        ));
        assert_eq!(p.text(), form, "{number}");
        assert_eq!(sanadi_ids(&p), sanadi, "{number}");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        let after_ayadi = ids.iter().rposition(|id| *id == "6.1.78").unwrap();
        assert_eq!(ids[after_ayadi + 1..], *tripadi, "{number}");
    }
    // laṅ: the āṭ leaves √ūrj no ik to lengthen (*aurjayat*), and √kṛp's
    // aṭ-initial *akalpayat* still takes 8.2.18.
    for (number, code, form, credits_8_2_78) in [
        ("10.0023", "urj", "Orjayad", false),
        ("10.0278", "kfp", "akalpayad", false),
    ] {
        let row = curadi_row(number, code, PadaAssignment::Nic);
        let p = declined(
            derive(
                &row,
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            2,
        );
        assert_eq!(p.text(), form, "{number}");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids.contains(&"8.2.78"), credits_8_2_78, "{number}");
        assert_eq!(ids.contains(&"8.2.18"), number == "10.0278", "{number}");
    }
}
"""),
],
'crates/panini-prakriya/src/tinanta/samjna.rs': [
("""pub(crate) const CISPHUR: [&str; 1] = ["10.0124"];

""",
"""pub(crate) const CISPHUR: [&str; 1] = ["10.0124"];

/// 8.2.18 kṛpo ro laḥ: the dhātupāṭha rows that are the √kṛp the sūtra
/// names. By number, as `CISPHUR` is: vidyut-prakriya keys the sūtra on the
/// upadeśas `kfpU~\\`, `kfpa~\\` and `kfpa~`, and curādi `10.0408 kfpa`
/// (*daurbalye*), an adanta root, is none of them. `10.0278 kfpa~` is the
/// one curated; bhvādi `01.0866 kfpU~\\` and `01.0875 kfpa~\\` join the list
/// when they are. Pinned to upstream by `krp_is_the_row_8_2_18_names`. The
/// tripādī 8.2.18 reads it.
pub(crate) const KRP: [&str; 1] = ["10.0278"];

"""),
("""    #[test]
    fn ghu_is_exactly_the_six_rows_1_1_20_names() {
""",
"""    #[test]
    fn krp_is_the_row_8_2_18_names() {
        // 8.2.18 kṛpo ro laḥ. vidyut-prakriya keys it on the upadeśas
        // `kfpU~\\`, `kfpa~\\` and `kfpa~`. Curādi `10.0408 kfpa` is another
        // root; the two bhvādi rows are not curated.
        assert_eq!(KRP, ["10.0278"]);
        assert_eq!(upstream_upadesha("10.0278"), Some("kfpa~"));
        assert_eq!(upstream_upadesha("10.0408"), Some("kfpa"));
        for (number, upadesha) in [("01.0866", "kfpU~\\\\"), ("01.0875", "kfpa~\\\\")] {
            assert_eq!(upstream_upadesha(number), Some(upadesha), "{number}");
            assert!(!dhatus().iter().any(|d| d.dhatupatha == number), "{number}");
        }
    }

    #[test]
    fn ghu_is_exactly_the_six_rows_1_1_20_names() {
"""),
],
'crates/panini-prakriya/src/tinanta/sanadi.rs': [
("""    #[test]
    fn sanadyanta_folds_nic_into_the_dhatu() {
""",
"""    #[test]
    fn dirghat_gives_mlecch_tuk_after_its_long_vowel() {
        // √mlecch: tuk after the long `e`, where 6.1.73 (short vowels only)
        // declines; 8.4.40 later makes the `t` a `c` (*mlecchayati*).
        let mut p = with_nic("mleC", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!(!(rule("6.1.73").apply)(&mut p));
        assert!((rule("6.1.75").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "mletC");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["6.1.75"]);
        // Every long vowel is the sūtra's dīrgha, not only `e`.
        for v in ['A', 'I', 'U', 'F', 'X', 'e', 'E', 'o', 'O'] {
            let mut p = with_nic(&format!("m{v}C"), Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("6.1.75").apply)(&mut p), "{v}");
            assert_eq!(p.terms[ANGA].text, format!("m{v}tC"));
        }
        // A short vowel before the `C` is 6.1.73's (viC); a consonant (mUrC,
        // and mletC once the tuk is in), nothing at all (Cid), or no `C`
        // (cur): decline.
        for root in ["viC", "mUrC", "mletC", "Cid", "cur"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("6.1.75").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
            assert!(p.log.is_empty(), "{root}");
        }
        assert!(!rule("6.1.75").vikalpa);
    }

    #[test]
    fn upadhayas_ca_makes_an_f_upadha_ir_before_nic_and_bars_its_guna() {
        // √kṝt: `kFt` → `kirt`. The `i` is guru before `rt`, so 7.3.86
        // declines (*kīrtayati*, not *kartayati*).
        let mut p = with_nic("kFt", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!((rule("7.1.101").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kirt");
        assert!(!(rule("7.3.86").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kirt");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["7.1.101"]);
        // Without it, 7.3.86 would guṇate the `F`: *kartayati*.
        let mut p = with_nic("kFt", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!((rule("7.3.86").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kart");
        // An `F` that is final (pF, 2565's row), a short `f` upadhā (kfp,
        // 7.3.86's), a one-letter aṅga (F): decline.
        for root in ["pF", "kfp", "F"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("7.1.101").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        // No ṇic, or a pratyaya at `NIC` that is not ṇit.
        for nic in [None, Some(&[Tag::Ardhadhatuka][..])] {
            let mut p = with_nic("kFt", nic);
            assert!(!(rule("7.1.101").apply)(&mut p), "{nic:?}");
            assert_eq!(p.terms[ANGA].text, "kFt");
        }
        assert!(!rule("7.1.101").vikalpa);
    }

    #[test]
    fn sanadyanta_folds_nic_into_the_dhatu() {
"""),
],
'crates/panini-prakriya/src/tinanta/tripadi.rs': [
("""    #[test]
    fn shatva_declines_for_every_pre_existing_junction() {
""",
"""    #[test]
    fn krpo_ro_lah_turns_krps_r_to_l_on_its_row_only() {
        let rule = rules().find(|r| r.id == "8.2.18").unwrap();
        let krp = |row: &'static str, anga: &str| {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new(anga), Term::new("a"), Term::new("ti")]),
                ..Default::default()
            };
            p.ctx.dhatupatha = row;
            p
        };
        // `karp` (7.3.86's guṇa before ṇic) → `kalp`: *kalpayati*.
        let mut p = krp("10.0278", "karpay");
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kalpay");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["8.2.18"]);
        // The `f` arm: an unguṇated `kfp` → `kxp`.
        let mut p = krp("10.0278", "kfp");
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kxp");
        // `10.0408 kfpa` is another root, and a row with no number is none.
        for row in ["10.0408", ""] {
            let mut p = krp(row, "karpay");
            assert!(!(rule.apply)(&mut p), "{row}");
            assert_eq!(p.terms[ANGA].text, "karpay");
            assert!(p.log.is_empty(), "{row}");
        }
        // On its row with no `r` or `f` left: nothing to record.
        let mut p = krp("10.0278", "kalpay");
        assert!(!(rule.apply)(&mut p));
        assert!(p.log.is_empty());
        assert!(!rule.vikalpa);
    }

    #[test]
    fn upadhayam_ca_lengthens_the_ik_before_a_roots_r_or_v_upadha() {
        let rule = rules().find(|r| r.id == "8.2.78").unwrap();
        let with_anga = |anga: &str, nijanta: bool| {
            let mut t = Term::new(anga);
            if nijanta {
                t.add(Tag::Nijanta);
            }
            Prakriya {
                terms: with_slots(vec![t, Term::new("a"), Term::new("ti")]),
                ..Default::default()
            }
        };
        // A ṇijanta aṅga is read through ṇic's `ay`: each short ik, before
        // an `r` or a `v` upadhā.
        for (anga, want) in [
            ("urjay", "Urjay"),
            ("curRay", "cUrRay"),
            ("gurday", "gUrday"),
            ("kirtay", "kIrtay"),
            ("pfrkay", "pFrkay"),
            ("kxvpay", "kXvpay"),
            ("divkay", "dIvkay"),
            // A five-letter root, where the upadhā's index `n - 2` is not
            // `n / 2`.
            ("stirpay", "stIrpay"),
        ] {
            let mut p = with_anga(anga, true);
            assert!((rule.apply)(&mut p), "{anga}");
            assert_eq!(p.terms[ANGA].text, want);
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["8.2.78"]);
        }
        // Any other aṅga is the root's own text.
        let mut p = with_anga("urj", false);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Urj");
        // Decline: laṅ's `Orjay` (6.1.90 left no ik), an `a` before the `r`
        // (arjay), a vowel-final root (uri), a two-letter root (rj), and an
        // `r` that is the root's last sound (kur, 8.2.77's shape). A
        // non-ṇijanta `urjay` is read whole and ends in `ay`.
        for (anga, nijanta) in [
            ("Orjay", true),
            ("arjay", true),
            ("uri", false),
            ("rjay", true),
            ("kur", false),
            ("urjay", false),
        ] {
            let mut p = with_anga(anga, nijanta);
            assert!(!(rule.apply)(&mut p), "{anga}");
            assert_eq!(p.terms[ANGA].text, anga);
            assert!(p.log.is_empty(), "{anga}");
        }
        assert!(!rule.vikalpa);
    }

    #[test]
    fn shtutva_retroflexes_a_stu_before_a_tu_too() {
        let rule = rules().find(|r| r.id == "8.4.41").unwrap();
        let word = |anga: &str| Prakriya {
            terms: with_slots(vec![Term::new(anga), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        // √aṭṭ: `adway` → `aqway` (8.4.55 then makes the `q` a `w`).
        let mut p = word("adway");
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "aqwayati");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["8.4.41"]);
        // Every ṭu after it triggers, and every stu before one takes its ṣṭu.
        for (anga, want) in [
            ("adW", "aqW"),
            ("adq", "aqq"),
            ("adQ", "aqQ"),
            ("adR", "aqR"),
            ("atw", "aww"),
            ("aTw", "aWw"),
            ("aDw", "aQw"),
            ("anw", "aRw"),
            ("asw", "azw"),
        ] {
            let mut p = word(anga);
            assert!((rule.apply)(&mut p), "{anga}");
            assert_eq!(p.terms[ANGA].text, want);
        }
        // 8.4.43 toḥ ṣi: a tu before `z` stays. A sound between them, or a
        // non-stu before the ṭu, and nothing happens.
        for anga in ["adz", "adaw", "akw"] {
            let mut p = word(anga);
            assert!(!(rule.apply)(&mut p), "{anga}");
            assert_eq!(p.terms[ANGA].text, anga);
        }
    }

    #[test]
    fn shatva_declines_for_every_pre_existing_junction() {
"""),
],
}
bad, out = [], {}
for p, edits in E.items():
    s = open(p).read()
    for old, new in edits:
        n = s.count(old)
        if n != 1:
            bad.append(f"{p}: {n}x {old[:70]!r}")
            continue
        s = s.replace(old, new)
    out[p] = s
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
for p, s in out.items():
    open(p, 'w').write(s)
print(f"applied {sum(len(e) for e in E.values())} edits to {len(E)} files")
```

```bash
python3 /tmp/vidyut-full/slice10l/engine_tests_10l.py      # applied 11 edits to 4 files
mise run fmt
```

- [ ] **Step 2: Run them to see them fail**

```bash
mise exec -- cargo test -p panini-prakriya 2>&1 | grep -E "^    tinanta|test result"
```

Expected: `437 passed; 7 failed`, the seven being:
- `derivation_tests::the_10l_rules_give_vidyuts_forms` and `derivation_tests::tinanta_rule_order_is_pinned`;
- `sanadi::tests::dirghat_gives_mlecch_tuk_after_its_long_vowel` and `sanadi::tests::upadhayas_ca_makes_an_f_upadha_ir_before_nic_and_bars_its_guna` (`rule(id)` finds no entry);
- `tripadi::tests::krpo_ro_lah_turns_krps_r_to_l_on_its_row_only`, `tripadi::tests::shtutva_retroflexes_a_stu_before_a_tu_too` and `tripadi::tests::upadhayam_ca_lengthens_the_ik_before_a_roots_r_or_v_upadha`.

`krp_is_the_row_8_2_18_names` passes already: it pins the declaration to upstream.

- [ ] **Step 3: The engine**

Create `/tmp/vidyut-full/slice10l/engine_10l.py` if it is missing (sha256 `cd0dbe9b94388a58e0f67618d762db1af3ad10a7dbad1ece8c4afdbaf03b57de`). It writes:
- **`sound.rs`:** `shtutva_of` and `shtutva_of_all_arms`.
- **`anga.rs`:** `tuk_before_che`, `che_ca` as its short-vowel caller, and `dirghat` as its long-vowel one.
- **`sanadi.rs`:** the import, then 6.1.75 (`apply: dirghat`) and 7.1.101 right after the sanādi 6.1.73; and the 7.1.101 test's two comments that described 7.3.86 wrongly before the prototype ran it (7.3.86 reads only a laghu ik, so the `F` stays: *kṝtayati*).
- **`tripadi.rs`:** the module doc's range, the `shtutva_of` and `KRP` imports, 8.2.18 first, 8.2.78 after 8.2.77, the stu-before-ṭu arm at the top of 8.4.41's scan, and the comments (8.4.40's "trigger-then-target direction only", 8.4.41's header and its CORRESPONDENCE paragraph).

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10l, Task 2's engine — 6.1.75 and 7.1.101 in the sanādi
stage, 8.2.18 and 8.2.78 in the tripādī, 8.4.41's stu-before-ṭu arm with
shtutva_of and its arms test, che_ca's scan shared with dirghat, and the
comments that described 8.4.41 one way only.
Every `old` must occur exactly once in its file; nothing is written if one
fails. Run from the worktree root."""
import sys
E = {
'crates/panini-prakriya/src/tinanta/anga.rs': [
("""pub(crate) fn che_ca(p: &mut Prakriya) -> bool {
    let w = word_chars(p);
    let Some(pos) = (1..w.len()).find(|i| w[*i].2 == 'C' && is_hrasva(w[i - 1].2)) else {
        return false;
""",
"""pub(crate) fn che_ca(p: &mut Prakriya) -> bool {
    tuk_before_che(p, is_hrasva, "6.1.73", "Ce ca")
}

/// 6.1.75 dIrGAt, applied: the first long vowel before a `C` takes tuk
/// after it (√mlecch: `mleC` → `mletC`). Behind the sanādi stage's entry
/// only: a laṅ aṭ is short, so an aṅga-stage entry could never fire.
pub(crate) fn dirghat(p: &mut Prakriya) -> bool {
    tuk_before_che(p, |c| is_vowel(c) && !is_hrasva(c), "6.1.75", "dIrGAt")
}

/// The one scan behind 6.1.73 and 6.1.75, as vidyut-prakriya runs the two
/// sūtras in one (`angasya.rs`, `try_add_tuk_agama`): the first vowel that
/// passes `vowel`, right before a `C` anywhere in the word, takes tuk after
/// it.
fn tuk_before_che(p: &mut Prakriya, vowel: fn(char) -> bool, sutra: &str, name: &str) -> bool {
    let w = word_chars(p);
    let Some(pos) = (1..w.len()).find(|i| w[*i].2 == 'C' && vowel(w[i - 1].2)) else {
        return false;
"""),
("""    insert_char(p, term, idx + 1, 't');
    p.record("6.1.73", "Ce ca", before);
    true
""",
"""    insert_char(p, term, idx + 1, 't');
    p.record(sutra, name, before);
    true
"""),
],
'crates/panini-prakriya/src/tinanta/sanadi.rs': [
("""use crate::term::{Tag, Term};
use crate::tinanta::anga::che_ca;
use crate::tinanta::samjna::CISPHUR;
""",
"""use crate::term::{Tag, Term};
use crate::tinanta::anga::{che_ca, dirghat};
use crate::tinanta::samjna::CISPHUR;
"""),
("""    },
    // 7.3.86 pugantalaghūpadhasya ca, before ṇic: guṇa of a laghu ik upadhā
""",
"""    },
    // 6.1.75 dīrghāt, before ṇic: √mlecch's long `e` takes tuk before its
    // `C` (`mleC` → `mletC`; 8.4.40 later makes the `t` a `c`, so
    // *mlecchayati*), where vidyut-prakriya credits it, in the same scan as
    // 6.1.73 (`super::anga::dirghat`). The sūtra's own condition is its
    // only guard, as 6.1.73's is. No aṅga-stage entry: a laṅ aṭ is short.
    Rule {
        id: "6.1.75",
        name: "dIrGAt",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: dirghat,
    },
    // 7.1.101 upadhāyāś ca, before ṇic: an `F` upadhā becomes `ir`, the `i`
    // by the sūtra and the `r` by 1.1.51 uraṇ raparaḥ, uncredited as
    // 7.1.102's `ur` is. √kṝt: `kFt` → `kirt`. Before 7.3.86, where
    // vidyut-prakriya credits it: the `i` is guru before `rt`, so 7.3.86
    // finds no laghu ik and the root takes no guṇa, and the tripādī 8.2.78
    // lengthens the `i` (*kīrtayati*). Guarded on ṇic's
    // ṇit at `NIC`. No other curated root has an `F` upadhā, so the guṇa
    // stage has no entry.
    Rule {
        id: "7.1.101",
        name: "upaDAyASca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) {
                return false;
            }
            let mut chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let Some(upadha) = chars.len().checked_sub(2) else {
                return false;
            };
            if chars[upadha] != 'F' {
                return false;
            }
            let before = p.snapshot();
            chars.splice(upadha..=upadha, ['i', 'r']);
            p.terms[ANGA].text = chars.into_iter().collect();
            p.record("7.1.101", "upaDAyASca", before);
            true
        },
    },
    // 7.3.86 pugantalaghūpadhasya ca, before ṇic: guṇa of a laghu ik upadhā
"""),
("""        // √kṝt: `kFt` → `kirt`. The `i` is guru before `rt`, so 7.3.86
        // declines (*kīrtayati*, not *kartayati*).
        let mut p = with_nic("kFt", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
""",
"""        // √kṝt: `kFt` → `kirt`. The `i` is guru before `rt`, so 7.3.86
        // declines, and the tripādī 8.2.78 lengthens it (*kīrtayati*).
        let mut p = with_nic("kFt", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
"""),
("""        assert_eq!(ids, ["7.1.101"]);
        // Without it, 7.3.86 would guṇate the `F`: *kartayati*.
        let mut p = with_nic("kFt", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!((rule("7.3.86").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kart");
        // An `F` that is final (pF, 2565's row), a short `f` upadhā (kfp,
""",
"""        assert_eq!(ids, ["7.1.101"]);
        // Without it the long `F` stays, since 7.3.86 reads only a laghu ik:
        // *kṝtayati*, the form 10k's prototype derived.
        let mut p = with_nic("kFt", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!(!(rule("7.3.86").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kFt");
        // An `F` that is final (pF, 2565's row), a short `f` upadhā (kfp,
"""),
],
'crates/panini-prakriya/src/tinanta/sound.rs': [
("""
/// The homorganic nasal of a *yay*. Covers only the stops — yay's
""",
"""
/// 8.4.41 ṣṭunā ṣṭuḥ's stu → ṣṭu substitution, by place of articulation:
/// `s` → `z` and the t-varga → the ṭ-varga. Read by 8.4.41's stu-first arm,
/// a stu before a ṭu. Only `d` → `q` is reached by a curated row (`10.0037
/// adwa~`, *aṭṭayati*), so `shtutva_of_all_arms` pins the other five. The
/// ṣṭu-first arm keeps its own narrow match; see its comment.
pub(crate) fn shtutva_of(c: char) -> Option<char> {
    Some(match c {
        's' => 'z',
        't' => 'w',
        'T' => 'W',
        'd' => 'q',
        'D' => 'Q',
        'n' => 'R',
        _ => return None,
    })
}

/// The homorganic nasal of a *yay*. Covers only the stops — yay's
"""),
("""        assert_eq!(shcutva_of('z'), None);
    }
""",
"""        assert_eq!(shcutva_of('z'), None);
    }

    #[test]
    fn shtutva_of_all_arms() {
        // 8.4.41 zwunA zwuH, stu before Swu: pin every arm of the stu -> Swu
        // table directly. Only `d -> q` is reachable from any golden (√aṭṭ's
        // `dw`), so without this test a mutant rewriting any of the other
        // five arms would be invisible to the whole suite.
        assert_eq!(shtutva_of('s'), Some('z'));
        assert_eq!(shtutva_of('t'), Some('w'));
        assert_eq!(shtutva_of('T'), Some('W'));
        assert_eq!(shtutva_of('d'), Some('q'));
        assert_eq!(shtutva_of('D'), Some('Q'));
        assert_eq!(shtutva_of('n'), Some('R'));
        // Already Swu, so not stu.
        for c in ['z', 'w', 'W', 'q', 'Q', 'R'] {
            assert_eq!(shtutva_of(c), None, "{c} is Swu, not stu");
        }
        // Not stu at all: a velar, and the palatal sibilant.
        assert_eq!(shtutva_of('k'), None);
        assert_eq!(shtutva_of('S'), None);
    }
"""),
],
'crates/panini-prakriya/src/tinanta/tripadi.rs': [
("""//! Tripādī: 8.2.77 … 8.4.56.
//!
""",
"""//! Tripādī: 8.2.18 … 8.4.56.
//!
"""),
("""use crate::term::Tag;
use crate::tinanta::sound::{
""",
"""use crate::term::Tag;
use crate::tinanta::samjna::KRP;
use crate::tinanta::sound::{
"""),
("""    is_savarna, is_shcu, is_shtu, is_vowel, jashtva_of, kutva_of, parasavarna_of, shcutva_of,
};
""",
"""    is_savarna, is_shcu, is_shtu, is_vowel, jashtva_of, kutva_of, parasavarna_of, shcutva_of,
    shtutva_of,
};
"""),
("""pub(crate) static TRIPADI: &[Rule] = &[
    // 8.2.77 hali ca: a root ending in `r`/`v` with a short ik upadhā
""",
"""pub(crate) static TRIPADI: &[Rule] = &[
    // 8.2.18 kṛpo ro laḥ: √kṛp's `r` becomes `l`, and an `f` an `x`.
    // 7.3.86's `karp` before ṇic → `kalp` (*kalpayati*). Keyed by row
    // (`super::samjna::KRP`), as vidyut-prakriya keys it by upadeśa:
    // `10.0408 kfpa` is another root. It rewrites the whole aṅga, ṇic's `ay`
    // included, which has no `r`; vidyut rewrites the dhātu term alone.
    // First in the tripādī, as vidyut runs it ahead of the rest of 8.2.
    Rule {
        id: "8.2.18",
        name: "kfpo ro laH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !KRP.contains(&p.ctx.dhatupatha) {
                return false;
            }
            let text = p.terms[ANGA].text.replace('f', "x").replace('r', "l");
            if text == p.terms[ANGA].text {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = text;
            p.record("8.2.18", "kfpo ro laH", before);
            true
        },
    },
    // 8.2.77 hali ca: a root ending in `r`/`v` with a short ik upadhā
"""),
("""            p.record("8.2.77", "hali ca", before);
            true
""",
"""            p.record("8.2.77", "hali ca", before);
            true
        },
    },
    // 8.2.78 upadhāyāṃ ca: a dhātu whose upadhā is `r` or `v` before a final
    // hal lengthens the short ik before that upadhā (8.2.76's dīrghaḥ and
    // 8.2.77's `r`/`v` by anuvṛtti). `urj` → `Urj` (*ūrjayati*), `curR` →
    // `cUrR`, `gurd` → `gUrd`, and √kṝt's `kirt` (7.1.101) → `kIrt`.
    // vidyut-prakriya reads the dhātu term; here ṇic is folded into `ANGA`,
    // and a ṇijanta aṅga reaches the tripādī as root + `ay` (7.3.84 then
    // 6.1.78 on ṇic's `i`, in all four lakāras), so the root is the text
    // before that `ay`. Any other aṅga is the root's own text. In laṅ √ūrj
    // declines: 6.1.90 has made its `u` part of an `O` (*aurjayat*). 8.2.79
    // na bhakurchurām needs no carve-out here: `kur` and `Cur` end in their
    // `r`, which is 8.2.77's shape, not this one's.
    Rule {
        id: "8.2.78",
        name: "upaDAyAM ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let anga = &p.terms[ANGA];
            let root = if anga.has(Tag::Nijanta) {
                anga.text
                    .strip_suffix("ay")
                    .expect("a ṇijanta aṅga reaches the tripādī as root + `ay`")
            } else {
                anga.text.as_str()
            };
            let c: Vec<char> = root.chars().collect();
            let n = c.len();
            if n < 3 || !matches!(c[n - 2], 'r' | 'v') || is_vowel(c[n - 1]) {
                return false;
            }
            let long = match c[n - 3] {
                'i' => 'I',
                'u' => 'U',
                'f' => 'F',
                'x' => 'X',
                _ => return false,
            };
            let text: String = c[..n - 3]
                .iter()
                .chain(std::iter::once(&long))
                .chain(&c[n - 2..])
                .collect::<String>()
                + &anga.text[root.len()..];
            let before = p.snapshot();
            p.terms[ANGA].text = text;
            p.record("8.2.78", "upaDAyAM ca", before);
            true
"""),
("""    // 8.4.41 next door scans for the same "a stu takes its neighbour's
    // class" pattern against the ṭu-varga instead of the c-varga, in the
    // trigger-then-target direction only — this rule's converse arm's
    // direction.
    //
""",
"""    // 8.4.41 next door scans for the same "a stu takes its neighbour's
    // class" pattern against the ṭu-varga instead of the c-varga, in both
    // directions since slice 10l, as this rule does since 3f3.
    //
"""),
("""    // carries it the rest of the way to the paradigm's finished piRqQi
    // (7b Task 6).
    //
""",
"""    // carries it the rest of the way to the paradigm's finished piRqQi
    // (7b Task 6). Since slice 10l it also fires the other way round, on a
    // stu BEFORE a ṭu: √aṭṭ's `adw` → `aqw`, which 8.4.55 then makes
    // *aṭṭayati*.
    //
"""),
("""    // CORRESPONDENCE staying narrow can go stale but cannot go
    // inconsistent the way 8.2.30 did.
    Rule {
""",
"""    // CORRESPONDENCE staying narrow can go stale but cannot go
    // inconsistent the way 8.2.30 did. The stu-before-ṭu arm (slice 10l)
    // took 8.4.40's route instead: its target is the sound BEFORE the ṭu,
    // a separate claim, and it reads the full table (`shtutva_of`), with
    // `shtutva_of_all_arms` pinning the five arms √aṭṭ's `d` does not reach.
    Rule {
"""),
("""            for i in 1..w.len() {
                if !is_shtu(w[i - 1].2) {
""",
"""            for i in 1..w.len() {
                // The sūtra's other order: a stu before a ṭu takes its ṣṭu
                // (√aṭṭ: `dw` → `qw`). The trigger is the ṭ-varga only, not
                // `z`: 8.4.43 toḥ ṣi keeps a tu before `z`.
                if matches!(w[i].2, 'w' | 'W' | 'q' | 'Q' | 'R')
                    && let Some(sub) = shtutva_of(w[i - 1].2)
                {
                    let (term, idx, _) = w[i - 1];
                    let before = p.snapshot();
                    set_char(p, term, idx, sub);
                    p.record("8.4.41", "zwunA zwuH", before);
                    return true;
                }
                if !is_shtu(w[i - 1].2) {
"""),
],
}
bad, out = [], {}
for p, edits in E.items():
    s = open(p).read()
    for old, new in edits:
        n = s.count(old)
        if n != 1:
            bad.append(f"{p}: {n}x {old[:70]!r}")
            continue
        s = s.replace(old, new)
    out[p] = s
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
for p, s in out.items():
    open(p, 'w').write(s)
print(f"applied {sum(len(e) for e in E.values())} edits to {len(E)} files")
```

```bash
python3 /tmp/vidyut-full/slice10l/engine_10l.py      # applied 17 edits to 4 files
mise run fmt
```

- [ ] **Step 4: Run the full suite**

```bash
mise run lint
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS, with `panini-prakriya` 445 and every other binary as in Task 1. The goldens are untouched, so this is also the first check that no prior cell moved.

- [ ] **Step 5: Commit**

```bash
git branch --show-current      # curadi-10l
git add -A
git commit -m "feat(prakriya): 6.1.75, 7.1.101, 8.2.18, 8.2.78 and 8.4.41's stu-before-ṭu arm

6.1.75 dīrghāt and 7.1.101 upadhāyāś ca run in the sanādi stage before
7.3.86, 6.1.75 sharing 6.1.73's tuk scan; 8.2.18 kṛpo ro laḥ heads the
tripādī, keyed by row (samjna::KRP); 8.2.78 upadhāyāṃ ca follows 8.2.77
and reads the root through ṇic's ay; 8.4.41 also retroflexes a stu before
a ṭu, through sound::shtutva_of. No curated row reaches any of them yet."
```

---

## Task 3: The rows, their goldens, and every assertion they move

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini-prakriya/src/tinanta/sanadi.rs`, `crates/panini-prakriya/src/tinanta/samjna.rs` (tests only)
- Modify: `crates/panini/tests/paradigm/main.rs`, `crates/panini/tests/paradigm/data/curadi.rs`
- Modify: `crates/panini/tests/trace/curadi.rs`, `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes: Task 2's rules and `KRP`; `skip_nic` through `OPTIONAL_NIC`; the trace helpers `credited`, `rows_crediting`, `optional_nic_rows`.
- Produces: the eight `Dhatu` rows; `("10.0270", "2570")`; the tests `the_10l_rules_fire_only_on_their_rows` and `curadi_analyses_its_10l_forms`.

- [ ] **Step 1: The assertions (failing)**

Create `/tmp/vidyut-full/slice10l/assertions_10l.py` if it is missing (sha256 `ad2a921a8947d6192321f279b393a0a698699e5a2421d07cc5288b48fa08ce37`). By file:
- **`panini-data`:** `OPTIONAL_NIC`'s doc and `optional_nic_from_upadesha`'s doc and 2570 arm (an initial `u~`); `Dhatu::pada`'s census (593; 427 `Nic`; 182 optional-ṇic) and the marker doc's 593; the row count (593); the row-list test, renamed `curadi_rows_are_the_four_hundred_ninety_curated_roots`, with a 10l paragraph and the eight tuples; `OPTIONAL_NIC.len()` 182.
- **`sanadi.rs` / `samjna.rs` tests:** √dhras joins `each_optional_nic_rule_takes_the_nicless_branch_on_its_own_rows` (2570's); `krp_is_the_row_8_2_18_names` asserts `10.0278` is curated.
- **`paradigm/main.rs`:** the census (38160 / 4240; ones 29700, twos 6788, threes 925, fours 361, sixes 367; `ALTERNATES` 11666; `8.4.56` / `7.1.35` / `7.1.35+8.4.56` 896 / 888 / 888; `7.3.86+8.4.56` 153, `7.3.86+7.1.35` and its 8.4.56 stack 136; 2570 252 and its three stacks 14) and its 10l paragraph and bucket messages; `curadi_analyses_its_10l_forms`.
- **`trace/curadi.rs`:** the 1.3.74 count (427); 2570's literal rows (+`10.0270`); `the_10l_rules_fire_only_on_their_rows`.
- **`trace/juhotyadi.rs`:** 8.4.55's off-√bhas count held at 455 with `10.0037`'s 78 separated out; 8.4.40's curādi list (+`10.0170`) and count (+78).

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10l, Task 3's assertions — every count, roster, list and
check() assertion the eight rows move, and the initial-u~ reading, before
the rows exist.
Every `old` must occur exactly once in its file; nothing is written if one
fails. Run from the worktree root."""
import sys
E = {
'crates/panini-data/src/lib.rs': [
("""/// before any marker, and otherwise the Kaumudī's 2564 for an idit root,
/// 2565 for `pF`, 2570 for a ñit or udit root, 2571 for `Guzi~r`, 2573.1
/// for `pata`, and 2573.3 for the roots that rule names (`mUtra`, `katra`,
""",
"""/// before any marker, and otherwise the Kaumudī's 2564 for an idit root,
/// 2565 for `pF`, 2570 for a ñit or udit root (udit by a final `u~`, or by
/// an initial one, as `10.0270 u~Drasa~` is), 2571 for `Guzi~r`, 2573.1
/// for `pata`, and 2573.3 for the roots that rule names (`mUtra`, `katra`,
"""),
("""    /// read from the upadeśa's it-markers — but no longer a *deferral*:
    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 585
    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and
    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed
    /// exception, 419 curādi rows' are 1.3.74's, seven more 1.3.74's with ṇic
    /// and 1.3.72's without (`NicUbhayapada`), 45 ākusmīya rows'
""",
"""    /// read from the upadeśa's it-markers — but no longer a *deferral*:
    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 593
    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and
    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed
    /// exception, 427 curādi rows' are 1.3.74's, seven more 1.3.74's with ṇic
    /// and 1.3.72's without (`NicUbhayapada`), 45 ākusmīya rows'
"""),
("""    /// `dhatupatha_numbers_resolve_upstream` holds `code` to upstream. For
    /// the 181 curādi rows whose ṇic is optional this is the ṇic branch's
    /// pada; the test also re-derives their ṇic-less branch's from the
""",
"""    /// `dhatupatha_numbers_resolve_upstream` holds `code` to upstream. For
    /// the 182 curādi rows whose ṇic is optional this is the ṇic branch's
    /// pada; the test also re-derives their ṇic-less branch's from the
"""),
("""    ///
    /// The test covers the 585 roots curated here, not the dhātupāṭha's 2259.
    /// It catches a mis-assigned pada on a root a future slice adds; it does
""",
"""    ///
    /// The test covers the 593 roots curated here, not the dhātupāṭha's 2259.
    /// It catches a mis-assigned pada on a root a future slice adds; it does
"""),
("""    fn curated_roots_have_expected_ganas_and_padas() {
        assert_eq!(dhatus().len(), 585);
        let bu = dhatus().iter().find(|d| d.dhatupatha == "01.0001").unwrap();
""",
"""    fn curated_roots_have_expected_ganas_and_padas() {
        assert_eq!(dhatus().len(), 593);
        let bu = dhatus().iter().find(|d| d.dhatupatha == "01.0001").unwrap();
"""),
("""    #[test]
    fn curadi_rows_are_the_four_hundred_eighty_two_curated_roots() {
        // Slice 10a opens gaṇa 10 with four roots that need only ṇic
""",
"""    #[test]
    fn curadi_rows_are_the_four_hundred_ninety_curated_roots() {
        // Slice 10a opens gaṇa 10 with four roots that need only ṇic
"""),
("""        // nothing new: thirty-seven by 7.3.86, forty by 7.2.116 and
        // seventy-eight unchanged before ṇic, all ubhayapadī by 1.3.74. The
        // gaṇa is OPEN at 482 of its 509 dhātupāṭha rows.
        let rows: Vec<_> = dhatus()
""",
"""        // nothing new: thirty-seven by 7.3.86, forty by 7.2.116 and
        // seventy-eight unchanged before ṇic, all ubhayapadī by 1.3.74. Slice
        // 10l adds eight rows that bring rules of their own: √ūrj, √cūrṇ and
        // √gūrd (8.2.78), √aṭṭ (8.4.41's stu-before-ṭu arm), √kṝt (7.1.101,
        // then 8.2.78), √mlecch (6.1.75), √dhras (udit by its initial `u~`,
        // ṇic optional by 2570) and √kṛp (8.2.18), all ubhayapadī by 1.3.74.
        // The gaṇa is OPEN at 490 of its 509 dhātupāṭha rows.
        let rows: Vec<_> = dhatus()
"""),
("""                ("10.0491", "ruW", PadaAssignment::Nic),
            ]
""",
"""                ("10.0491", "ruW", PadaAssignment::Nic),
                ("10.0023", "urj", PadaAssignment::Nic),
                ("10.0026", "curR", PadaAssignment::Nic),
                ("10.0037", "adw", PadaAssignment::Nic),
                ("10.0155", "kFt", PadaAssignment::Nic),
                ("10.0170", "mleC", PadaAssignment::Nic),
                ("10.0180", "gurd", PadaAssignment::Nic),
                ("10.0270", "Dras", PadaAssignment::Nic),
                ("10.0278", "kfp", PadaAssignment::Nic),
            ]
"""),
("""    /// is the ROOT's own accent and says nothing about pada. Counted off the
    /// vendored upadeśa: 73 of the 585 curated roots carry a `\\` at all, and 52
    /// of those carry one on a root vowel — `01.0642 ji\\`, `01.1082 smf\\` and
""",
"""    /// is the ROOT's own accent and says nothing about pada. Counted off the
    /// vendored upadeśa: 73 of the 593 curated roots carry a `\\` at all, and 52
    /// of those carry one on a root vowel — `01.0642 ji\\`, `01.1082 smf\\` and
"""),
("""    /// idit root (last marker `i~`), 2565 for an `F`-final one, 2570 for a ñit
    /// or udit one (last marker `Y` or `u~`), 2571 for `Guzi~r`, 2573.1 for
    /// `pata`, and 2573.3 for the three roots the Kaumudī names there. 2573.3
""",
"""    /// idit root (last marker `i~`), 2565 for an `F`-final one, 2570 for a ñit
    /// or udit one (last marker `Y` or `u~`, or an initial `u~`, which marks
    /// an udit root as vidyut's it-reading tags any `u~`: `10.0270
    /// u~Drasa~`), 2571 for `Guzi~r`, 2573.1 for
    /// `pata`, and 2573.3 for the three roots the Kaumudī names there. 2573.3
"""),
("""            Some("2565")
        } else if u.ends_with("u~") || u.ends_with('Y') {
            Some("2570")
""",
"""            Some("2565")
        } else if u.ends_with("u~") || u.starts_with("u~") || u.ends_with('Y') {
            Some("2570")
"""),
("""        }
        assert_eq!(OPTIONAL_NIC.len(), 181);
        // `10.0368 za\\da~` stays out: the reading above makes it 10.0498's,
""",
"""        }
        assert_eq!(OPTIONAL_NIC.len(), 182);
        // `10.0368 za\\da~` stays out: the reading above makes it 10.0498's,
"""),
],
'crates/panini-prakriya/src/tinanta/samjna.rs': [
("""        assert_eq!(upstream_upadesha("10.0278"), Some("kfpa~"));
        assert_eq!(upstream_upadesha("10.0408"), Some("kfpa"));
""",
"""        assert_eq!(upstream_upadesha("10.0278"), Some("kfpa~"));
        assert!(dhatus().iter().any(|d| d.dhatupatha == "10.0278"));
        assert_eq!(upstream_upadesha("10.0408"), Some("kfpa"));
"""),
],
'crates/panini-prakriya/src/tinanta/sanadi.rs': [
("""            ("2570", "10.0124", "ci", Tag::Nic),
            ("2571", "10.0251", "Guz", Tag::Nic),
""",
"""            ("2570", "10.0124", "ci", Tag::Nic),
            // √dhras, udit by its initial `u~`.
            ("2570", "10.0270", "Dras", Tag::Nic),
            ("2571", "10.0251", "Guz", Tag::Nic),
"""),
],
'crates/panini/tests/paradigm/main.rs': [
("""/// rows.
/// This test is what keeps the numbers true day to day.
""",
"""/// rows.
///
/// Slice 10l curates eight rows that bring rules of their own: 8.2.78 (√ūrj,
/// √cūrṇ, √gūrd, and √kṝt after 7.1.101), 8.2.18 (√kṛp), 6.1.75 (√mlecch)
/// and 8.4.41's stu-before-ṭu arm (√aṭṭ), all `Nic`, which fork exactly
/// where √cur does; and √dhras, whose ṇic is optional (2570), so its every
/// parasmaipada cell adds the ṇic-less reading beside the pinned ṇic form.
/// 576 new cells, 90 new rows. The gaṇa is OPEN at 490 of its 509 rows.
/// This test is what keeps the numbers true day to day.
"""),
("""    let total_cells = PARADIGM.len() * 9;
    assert_eq!(total_cells, 37584, "4176 root×lakāra blocks × 9 cells each");

""",
"""    let total_cells = PARADIGM.len() * 9;
    assert_eq!(total_cells, 38160, "4240 root×lakāra blocks × 9 cells each");

"""),
("""    }
    assert_eq!(ones, 29188, "one-form cells");
    assert_eq!(twos, 6742, "two-form cells");
    assert_eq!(
        threes, 911,
        "three-form cells — new in slice 3b — √hrī's loṭ prathama and madhyama eka, each by \\
""",
"""    }
    assert_eq!(ones, 29700, "one-form cells");
    assert_eq!(twos, 6788, "two-form cells");
    assert_eq!(
        threes, 925,
        "three-form cells — new in slice 3b — √hrī's loṭ prathama and madhyama eka, each by \\
"""),
("""         forks on neither (6.1.54 and 2570 beside the ṇic form); and — new in slice 10k — the \\
         155 plain obligatory-ṇic rows' two loṭ tātaṅ cells, by 7.1.35/8.4.56"
    );
    assert_eq!(
        fours, 359,
        "four-form cells — piṣ's loṭ madhyama eka, Siz's loṭ parasmaipada madhyama eka (slice \\
""",
"""         forks on neither (6.1.54 and 2570 beside the ṇic form); and — new in slice 10k — the \\
         155 plain obligatory-ṇic rows' two loṭ tātaṅ cells, by 7.1.35/8.4.56; and — new in \\
         slice 10l — seven of its eight rows' the same way"
    );
    assert_eq!(
        fours, 361,
        "four-form cells — piṣ's loṭ madhyama eka, Siz's loṭ parasmaipada madhyama eka (slice \\
"""),
("""         and — new in slice 10i — the sixty-one rows' the same cells, 10.0499/2565/2571 \\
         alongside 8.4.56"
    );
""",
"""         and — new in slice 10i — the sixty-one rows' the same cells, 10.0499/2565/2571 \\
         alongside 8.4.56; and — new in slice 10l — √dhras's the same cells, 2570 alongside \\
         8.4.56"
    );
"""),
("""    assert_eq!(
        sixes, 365,
        "six-form cells — kft loṭ madhyama eka, ruD loṭ parasmaipada madhyama eka, Bid, kzud \\
""",
"""    assert_eq!(
        sixes, 367,
        "six-form cells — kft loṭ madhyama eka, ruD loṭ parasmaipada madhyama eka, Bid, kzud \\
"""),
("""         — new in slice 10j — √ci's laṅ and vidhiliṅ parasmaipada prathama eka (6.1.54 and 2570 \\
         beside 8.4.56)"
    );
""",
"""         — new in slice 10j — √ci's laṅ and vidhiliṅ parasmaipada prathama eka (6.1.54 and 2570 \\
         beside 8.4.56); and — new in slice 10l — √dhras's loṭ parasmaipada prathama and \\
         madhyama eka (2570 beside 7.1.35/8.4.56)"
    );
"""),
("""
    assert_eq!(ALTERNATES.len(), 11576, "ALTERNATES row count");
    let key_count = |key: &str| {
""",
"""
    assert_eq!(ALTERNATES.len(), 11666, "ALTERNATES row count");
    let key_count = |key: &str| {
"""),
("""    };
    assert_eq!(key_count("8.4.56"), 882, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 874, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 874, "7.1.35+8.4.56 alternates");
    assert_eq!(key_count("3.4.111"), 2, "3.4.111 alternates");
""",
"""    };
    assert_eq!(key_count("8.4.56"), 896, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 888, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 888, "7.1.35+8.4.56 alternates");
    assert_eq!(key_count("3.4.111"), 2, "3.4.111 alternates");
"""),
("""    assert_eq!(key_count("7.3.86+6.4.107"), 8, "7.3.86+6.4.107 alternates");
    assert_eq!(key_count("7.3.86+8.4.56"), 151, "7.3.86+8.4.56 alternates");
    assert_eq!(key_count("7.3.86+8.2.75"), 1, "7.3.86+8.2.75 alternates");
""",
"""    assert_eq!(key_count("7.3.86+6.4.107"), 8, "7.3.86+6.4.107 alternates");
    assert_eq!(key_count("7.3.86+8.4.56"), 153, "7.3.86+8.4.56 alternates");
    assert_eq!(key_count("7.3.86+8.2.75"), 1, "7.3.86+8.2.75 alternates");
"""),
("""    assert_eq!(key_count("6.4.43+8.4.56"), 1, "6.4.43+8.4.56 alternates");
    assert_eq!(key_count("7.3.86+7.1.35"), 134, "7.3.86+7.1.35 alternates");
    assert_eq!(
        key_count("7.3.86+7.1.35+8.4.56"),
        134,
        "7.3.86+7.1.35+8.4.56 alternates"
""",
"""    assert_eq!(key_count("6.4.43+8.4.56"), 1, "6.4.43+8.4.56 alternates");
    assert_eq!(key_count("7.3.86+7.1.35"), 136, "7.3.86+7.1.35 alternates");
    assert_eq!(
        key_count("7.3.86+7.1.35+8.4.56"),
        136,
        "7.3.86+7.1.35+8.4.56 alternates"
"""),
("""    // Slice 10j's √ci adds 72 to 2570 alone, its ṇic-less branch in both
    // padas, and two to each of 2570's 8.4.56 and 7.1.35 stacks.
    for (key, n) in [
        ("2564", 1908),
        ("2570", 216),
        ("2570+7.3.86", 72),
""",
"""    // Slice 10j's √ci adds 72 to 2570 alone, its ṇic-less branch in both
    // padas, and two to each of 2570's 8.4.56 and 7.1.35 stacks; slice 10l's
    // √dhras 36, its ṇic-less parasmaipada, and two to each stack.
    for (key, n) in [
        ("2564", 1908),
        ("2570", 252),
        ("2570+7.3.86", 72),
"""),
("""        ("2573.3+7.1.35+8.4.56", 6),
        ("2570+8.4.56", 12),
        ("2570+7.1.35", 12),
        ("2570+7.1.35+8.4.56", 12),
        ("2570+7.3.86+8.4.56", 6),
""",
"""        ("2573.3+7.1.35+8.4.56", 6),
        ("2570+8.4.56", 14),
        ("2570+7.1.35", 14),
        ("2570+7.1.35+8.4.56", 14),
        ("2570+7.3.86+8.4.56", 6),
"""),
("""        "piCayati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
""",
"""        "piCayati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}

/// Slice 10l's `check()` witnesses, from its spec's table: each row's laṭ
/// parasmaipada prathama eka with the rules the slice brings for it, √ūrj's
/// laṅ (*aurjayat*, where 8.2.78 declines), and √dhras's ṇic-less *dhrasati*
/// (2570). The goldens were grepped for every witness first. *cūrṇayati* is
/// also the 10k row `10.0143 cUrRa~`'s, which answers first, `dhatus()`
/// order. Every witness is parasmaipada, so 1.3.78 credits. Each form the
/// slice's rules rule out (the same row with the rule declined) derives
/// nothing.
#[test]
fn curadi_analyses_its_10l_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    let has = |ids: &[String], id: &str| ids.iter().any(|i| i == id);
    // (form, [(root, ids it must credit, ids it must not)], in analysis order)
    type Reading<'a> = (&'a str, &'a [&'a str], &'a [&'a str]);
    let witnesses: &[(&str, &[Reading])] = &[
        (
            "Urjayati",
            &[("urj", &["3.1.25", "8.2.78"][..], &["2570"][..])],
        ),
        ("Orjayat", &[("urj", &["3.1.25", "6.4.72"], &["8.2.78"])]),
        (
            "cUrRayati",
            &[
                ("cUrR", &["3.1.25"], &["8.2.78"]),
                ("curR", &["3.1.25", "8.2.78"], &[]),
            ],
        ),
        ("gUrdayati", &[("gurd", &["3.1.25", "8.2.78"], &[])]),
        (
            "kIrtayati",
            &[("kFt", &["3.1.25", "7.1.101", "8.2.78"], &["7.3.86"])],
        ),
        (
            "kalpayati",
            &[("kfp", &["3.1.25", "7.3.86", "8.2.18"], &[])],
        ),
        (
            "mlecCayati",
            &[("mleC", &["3.1.25", "6.1.75", "8.4.40"], &["6.1.73"])],
        ),
        ("awwayati", &[("adw", &["3.1.25", "8.4.41", "8.4.55"], &[])]),
        ("DrAsayati", &[("Dras", &["3.1.25", "7.2.116"], &["2570"])]),
        ("Drasati", &[("Dras", &["2570"], &["3.1.25", "7.2.116"])]),
    ];
    for (form, readings) in witnesses {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        let roots: Vec<&str> = r.analyses.iter().map(|a| a.dhatu.as_str()).collect();
        let want: Vec<&str> = readings.iter().map(|(root, _, _)| *root).collect();
        assert_eq!(roots, want, "{form}");
        for (a, (root, credits, absent)) in r.analyses.iter().zip(readings.iter()) {
            assert_eq!(a.pada, Pada::Parasmaipada, "{form} {root}");
            let ids = ids_of(a);
            for id in ["1.3.78"].iter().chain(credits.iter()) {
                assert!(has(&ids, id), "{form} {root} {id}: {ids:?}");
            }
            for id in absent.iter() {
                assert!(!has(&ids, id), "{form} {root} {id}: {ids:?}");
            }
        }
    }
    for form in [
        "urjayati",
        "curRayati",
        "gurdayati",
        "kirtayati",
        "kFtayati",
        "karpayati",
        "mleCayati",
        "adwayati",
        "aqwayati",
        "Drasayati",
        "DrAsati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
"""),
],
'crates/panini/tests/trace/curadi.rs': [
("""    // `AKUSMIYA` range. And 1.3.74 never reaches them: its credits stay on the
    // 419 `Nic` rows and the seven `NicUbhayapada` rows' ṇic branch, read from
    // the curated `pada` column (ten before slice 10e, listed literally until
""",
"""    // `AKUSMIYA` range. And 1.3.74 never reaches them: its credits stay on the
    // 427 `Nic` rows and the seven `NicUbhayapada` rows' ṇic branch, read from
    // the curated `pada` column (ten before slice 10e, listed literally until
"""),
("""        .collect();
    assert_eq!(nic.len(), 419, "curated 1.3.74 rows");
    let nic_ubhayapada: Vec<&str> = dhatus()
""",
"""        .collect();
    assert_eq!(nic.len(), 427, "curated 1.3.74 rows");
    let nic_ubhayapada: Vec<&str> = dhatus()
"""),
("""    // inert on every root outside `OPTIONAL_NIC`. The 10.0498, 10.0499, 2564
    // and 2570 rows are the 10f to 10j specs' row tables, listed literally.
    for (id, rows) in [
""",
"""    // inert on every root outside `OPTIONAL_NIC`. The 10.0498, 10.0499, 2564
    // and 2570 rows are the 10f to 10l specs' row tables, listed literally.
    for (id, rows) in [
"""),
("""                "10.0227", "10.0230", "10.0174", "10.0184", "10.0243", "10.0249", "10.0260",
                "10.0266", "10.0124",
            ][..],
""",
"""                "10.0227", "10.0230", "10.0174", "10.0184", "10.0243", "10.0249", "10.0260",
                "10.0266", "10.0124", "10.0270",
            ][..],
"""),
("""
#[test]
#[allow(non_snake_case)]
fn vicCayati_and_vicCAyati_take_tuk_before_3_1_32() {
""",
"""
#[test]
fn the_10l_rules_fire_only_on_their_rows() {
    // Goldens ignore traces, so this is what holds slice 10l's rules to their
    // rows across the corpus, in every stage: 6.1.75 on √mlecch and 7.1.101 on
    // √kṝt alone; 8.2.18 on √kṛp (`10.0278`) alone, never on the adanta
    // `10.0408 kfpa`; 8.2.78 on the four roots with a short ik before an `r`
    // upadhā and a final hal; and 8.4.41 on rudhādi's and juhotyādi's ṣṭu-
    // first sites as before, plus √aṭṭ, its stu-before-ṭu arm's one site.
    assert_eq!(rows_crediting("6.1.75", false), ["10.0170"]);
    assert_eq!(rows_crediting("7.1.101", false), ["10.0155"]);
    assert_eq!(rows_crediting("8.2.18", false), ["10.0278"]);
    assert_eq!(
        rows_crediting("8.2.78", false),
        ["10.0023", "10.0026", "10.0155", "10.0180"]
    );
    assert_eq!(
        rows_crediting("8.4.41", false),
        [
            "03.0014", "03.0023", "07.0014", "07.0015", "07.0018", "10.0037"
        ]
    );
}

#[test]
#[allow(non_snake_case)]
fn vicCayati_and_vicCAyati_take_tuk_before_3_1_32() {
"""),
],
'crates/panini/tests/trace/juhotyadi.rs': [
("""    // before the widening: a new word-internal firing on a prior row would
    // add one. Update it only when a slice adds rows that credit 8.4.55.
    let off_bhas = credited("8.4.55")
        .into_iter()
        .filter(|(number, _)| *number != "03.0019")
        .count();
    assert_eq!(off_bhas, 455);
}
""",
"""    // before the widening: a new word-internal firing on a prior row would
    // add one. Update it only when a slice adds rows that credit 8.4.55. Slice
    // 10l's √aṭṭ (`10.0037`) credits it on every live branch, 78: 8.4.41's
    // `q` before the `w`, word-internal.
    let hits = credited("8.4.55");
    let off_bhas = hits
        .iter()
        .filter(|(number, _)| *number != "03.0019" && *number != "10.0037")
        .count();
    assert_eq!(off_bhas, 455);
    let att = hits
        .iter()
        .filter(|(number, _)| *number == "10.0037")
        .count();
    assert_eq!(att, 78);
}
"""),
("""    // ch-initial plain rows (`Card`, `Cuw`) the adanta way (acCardayat) and
    // √pich (`piC`) the √vich way (picCayati).
    let hits = credited("8.4.40");
""",
"""    // ch-initial plain rows (`Card`, `Cuw`) the adanta way (acCardayat) and
    // √pich (`piC`) the √vich way (picCayati), and slice 10l √mlecch
    // (`mleC`), whose tuk the sanādi 6.1.75 gives (mlecCayati).
    let hits = credited("8.4.40");
"""),
("""                "10.0171", "10.0352", "10.0354", "10.0370", "10.0304", "10.0078", "10.0462",
                "10.0061"
            ]
""",
"""                "10.0171", "10.0352", "10.0354", "10.0370", "10.0304", "10.0078", "10.0462",
                "10.0061", "10.0170"
            ]
"""),
("""    // Slice 10k's two ch-initial rows count as the adanta ones do, and √pich
    // every live branch, 42 parasmaipada and 36 ātmanepada.
    assert_eq!(curadi, 3 * 19 + 6 * 29 + 120 + 2 * 19 + 78);
}
""",
"""    // Slice 10k's two ch-initial rows count as the adanta ones do, and √pich
    // every live branch, 42 parasmaipada and 36 ātmanepada; slice 10l's
    // √mlecch the same 78.
    assert_eq!(curadi, 3 * 19 + 6 * 29 + 120 + 2 * 19 + 78 + 78);
}
"""),
],
}
bad, out = [], {}
for p, edits in E.items():
    s = open(p).read()
    for old, new in edits:
        n = s.count(old)
        if n != 1:
            bad.append(f"{p}: {n}x {old[:70]!r}")
            continue
        s = s.replace(old, new)
    out[p] = s
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
for p, s in out.items():
    open(p, 'w').write(s)
print(f"applied {sum(len(e) for e in E.values())} edits to {len(E)} files")
```

```bash
python3 /tmp/vidyut-full/slice10l/assertions_10l.py      # applied 37 edits to 6 files
mise run fmt
```

- [ ] **Step 2: Run them to see them fail**

```bash
mise exec -- cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .*FAILED|test result: FAILED"
```

Foreground, timeout 600000 ms. Expected: eleven failures, in four binaries:
- `paradigm`: `curadi_analyses_its_10l_forms`, `derivation_set_shape_matches_the_audited_numbers`;
- `trace`: `curadi::a_kusmad_is_credited_on_exactly_the_akusmiya_cells`, `curadi::the_10l_rules_fire_only_on_their_rows`, `juhotyadi::khari_ca_off_bhas_is_credited_exactly_as_before_3f2`, `juhotyadi::shcutva_off_jan_is_credited_exactly_as_before_3f3`;
- `panini-data`: `tests::curadi_rows_are_the_four_hundred_ninety_curated_roots`, `tests::curated_roots_have_expected_ganas_and_padas`, `tests::optional_nic_matches_upadesha_markers`;
- `panini-prakriya`: `tinanta::samjna::tests::krp_is_the_row_8_2_18_names`, `tinanta::sanadi::tests::each_optional_nic_rule_takes_the_nicless_branch_on_its_own_rows`.

`the_optional_nic_ids_are_credited_only_on_their_rows` passes already (a row allowed, not yet present), and `pada_ambiguous_surfaces_are_exactly_these` does too, until the rows land (Step 6).

- [ ] **Step 3: The rows**

Create `/tmp/vidyut-full/slice10l/rows_10l.py` if it is missing (sha256 `dd13636c0ab4e6379ca60a6391663b94610e0ba583569507f94ba44e603fa46f`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10l — append the eight rows to `DHATUS`, add
`("10.0270", "2570")` to `OPTIONAL_NIC`, and give `10.0143`'s comment its
homograph pointer. Run from the worktree root."""
p = 'crates/panini-data/src/lib.rs'
s = open(p).read()
ROWS = '''    Dhatu {
        // 10.0023 `urja~` balaprARanayoH (√ūrj). Guru upadhā (the conjunct
        // `rj`), so unchanged before ṇic; the tripādī 8.2.78 upadhāyāṃ ca
        // lengthens the `u` before the `r` upadhā (*ūrjayati*), but not in
        // laṅ, where 6.1.90 has merged it into the āṭ (*aurjayat*).
        // Ubhayapadī by 1.3.74. Slice 10l.
        dhatupatha: "10.0023",
        code: "urj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "balaprARanayoH",
    },
    Dhatu {
        // 10.0026 `curRa~` preraRe (√cūrṇ). Guru upadhā (the conjunct `rR`),
        // so unchanged before ṇic; 8.2.78 lengthens the `u` (*cūrṇayati*).
        // Homograph of the ubhayapadī `10.0143 cUrRa~`: every form of this
        // row is also that row's. Ubhayapadī by 1.3.74. The artha keeps
        // upstream's trailing space. Slice 10l.
        dhatupatha: "10.0026",
        code: "curR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "preraRe ",
    },
    Dhatu {
        // 10.0037 `adwa~` anAdare (√aṭṭ). Guru upadhā (the conjunct `dw`),
        // so unchanged before ṇic; 8.4.41 retroflexes the `d` before the
        // `w`, and 8.4.55 makes it `w` (*aṭṭayati*). Ubhayapadī by 1.3.74.
        // Slice 10l.
        dhatupatha: "10.0037",
        code: "adw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "anAdare",
    },
    Dhatu {
        // 10.0155 `kFta~` saMSabdane (√kṝt). 7.1.101 upadhāyāś ca makes the
        // `F` upadhā `ir` before ṇic (kirt-i), guru before `rt`, so 7.3.86
        // declines; 8.2.78 lengthens the `i` (*kīrtayati*). Ubhayapadī by
        // 1.3.74. Slice 10l.
        dhatupatha: "10.0155",
        code: "kFt",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMSabdane",
    },
    Dhatu {
        // 10.0170 `mleCa~` avyaktAyAM vAci (√mlecch). 6.1.75 dīrghāt adds tuk
        // after the long `e` before ṇic (mletC-i), and 8.4.40 makes it `c`
        // (*mlecchayati*). Ubhayapadī by 1.3.74. Slice 10l.
        dhatupatha: "10.0170",
        code: "mleC",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "avyaktAyAM vAci",
    },
    Dhatu {
        // 10.0180 `gurda~` pUrvaniketane (√gūrd). Guru upadhā (the conjunct
        // `rd`), so unchanged before ṇic; 8.2.78 lengthens the `u`
        // (*gūrdayati*). Ubhayapadī by 1.3.74. Slice 10l.
        dhatupatha: "10.0180",
        code: "gurd",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pUrvaniketane",
    },
    Dhatu {
        // 10.0270 `u~Drasa~` uYCe (√dhras). Udit by its initial `u~`, so its
        // ṇic is optional by Kaumudī 2570 (`OPTIONAL_NIC`). With ṇic, 7.2.116
        // lengthens the `a` upadhā (*dhrāsayati*), ubhayapadī by 1.3.74;
        // without, parasmaipadī by 1.3.78 (*dhrasati*). The next row,
        // `10.0271 uDrasa~`, has no `u~` and takes ṇic. Slice 10l.
        dhatupatha: "10.0270",
        code: "Dras",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "uYCe",
    },
    Dhatu {
        // 10.0278 `kfpa~` avakalkane (√kṛp). 7.3.86 guṇates the `f` before
        // ṇic (karp-i), and the tripādī 8.2.18 kṛpo ro laḥ makes it `kalp`
        // (*kalpayati*), keyed by row (`KRP`): `10.0408 kfpa` is another
        // root. Ubhayapadī by 1.3.74. Slice 10l.
        dhatupatha: "10.0278",
        code: "kfp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "avakalkane",
    },
'''
end = "];\n\npub fn dhatus() -> &'static [Dhatu] {"
assert s.count(end) == 1
s = s.replace(end, ROWS + end)
for old, new in [
    ('    ("10.0267", "2564"),\n', '    ("10.0267", "2564"),\n    ("10.0270", "2570"),\n'),
    ('''        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*cūrṇayati*). Slice
        // 10k.
        dhatupatha: "10.0143",''',
     '''        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*cūrṇayati*).
        // Homograph of `10.0026 curRa~` (slice 10l), which 8.2.78 lengthens to
        // the same `cUrR`: every form of that row is also this one's. Slice
        // 10k.
        dhatupatha: "10.0143",'''),
]:
    assert s.count(old) == 1, old
    s = s.replace(old, new)
open(p, 'w').write(s)
print("inserted 8 rows and 1 OPTIONAL_NIC entry")
```

```bash
python3 /tmp/vidyut-full/slice10l/rows_10l.py      # inserted 8 rows and 1 OPTIONAL_NIC entry
mise run fmt
```

- [ ] **Step 4: Generate the goldens**

Create `/tmp/vidyut-full/vidyut-prakriya/examples/curadi_goldens_10l.rs` if it is missing (sha256 `7c288f3ef4123c4be21ab6aa338d4c1e16ef14df5106310db761080cdd774853`). It is 10j's generator with the row selection changed: it derives every cell of the eight rows in both engines, asserts the form sets equal, and writes the rows the static wants (pinned form = first live branch; each further distinct form an `ALTERNATES` row keyed by its branch's vikalpa ids in log order).

```rust
//! THROWAWAY: slice 10l — emit the eight rule-bearing rows' goldens from the engine,
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
    const NEW: [&str; 8] = ["10.0023", "10.0026", "10.0037", "10.0155", "10.0170", "10.0180", "10.0270", "10.0278"];
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
    std::fs::write("/tmp/vidyut-full/goldens_10l_paradigm.rs", par).unwrap();
    std::fs::write("/tmp/vidyut-full/goldens_10l_alternates.rs", alt).unwrap();
    println!("{ncells} cells, {nforms} forms, {ndiff} differences");
    assert_eq!(ndiff, 0);
}
```

```bash
WT="$(git rev-parse --show-toplevel)"
V=/tmp/vidyut-full/vidyut-prakriya
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml      # must point at $WT/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example curadi_goldens_10l 2>/dev/null | tail -1)
sha256sum /tmp/vidyut-full/goldens_10l_paradigm.rs /tmp/vidyut-full/goldens_10l_alternates.rs
```

Expected: `576 cells, 666 forms, 0 differences`, then:
- `b5deb8cba77557533b112c4b8a68ec4b8c8402f3e2248c6e684990096da47ddf  /tmp/vidyut-full/goldens_10l_paradigm.rs`
- `259014210fd3630abd644eca325b833602677ca337f49fdf13793348cd6e4f01  /tmp/vidyut-full/goldens_10l_alternates.rs`

Leave the dev-deps pointing at this worktree; Task 4 uses them. If a line differs, stop and report.

- [ ] **Step 5: Insert the goldens**

Create `/tmp/vidyut-full/slice10l/insert_goldens_10l.py` if it is missing (sha256 `a672592b41fdacdffdc7cf0f4979f118fda384b0b09d57bb11bf791b66d39065`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10l — insert the generator's goldens before each
static's `];`. Run from the worktree root."""
p = 'crates/panini/tests/paradigm/data/curadi.rs'
s = open(p).read()
par = open('/tmp/vidyut-full/goldens_10l_paradigm.rs').read()
alt = open('/tmp/vidyut-full/goldens_10l_alternates.rs').read()
i = s.index('pub const ALTERNATES')
head, tail = s[:i], s[i:]
k = head.rindex('];'); head = head[:k] + par + head[k:]
k = tail.rindex('];'); tail = tail[:k] + alt + tail[k:]
open(p, 'w').write(head + tail)
print("inserted goldens")
```

```bash
python3 /tmp/vidyut-full/slice10l/insert_goldens_10l.py      # inserted goldens
mise run fmt
```

- [ ] **Step 6: The measured pada-ambiguous set**

The set is measured, never hand-picked: run the test against the old set and read the real one off its failure. Create `/tmp/vidyut-full/slice10l/pin_ambiguous_10l.py` if it is missing (sha256 `39ee477fbb5ef67374ba1cbf4b1c94d195bad98ef8814adde65c7d5703e4f59d`). It refuses a set whose size or hash differs from the prototype's.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10l — pin the measured pada-ambiguous set ($SET, the
failing test's `left:` JSON) into pada_ambiguous_surfaces_are_exactly_these,
and extend the comment that accounts for it. Run from the worktree root."""
import hashlib, json, os
amb = json.load(open(os.environ['SET']))
assert len(amb) == 1631, len(amb)
h = hashlib.sha256('\n'.join(amb).encode()).hexdigest()
assert h == '079759cd23e05ff445f4bab7af74634d176ce8b71e1188361c35210d3801ce97', h
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
old = """    // 540 more, taking the set from 1063 to 1603.
"""
assert s.count(old) == 1
s = s.replace(old, old + """    // Slice 10l's eight rows contribute the same four each, less `10.0026
    // curRa~`'s, which its homograph `10.0143 cUrRa~` already holds (−4);
    // √dhras's ṇic-less branch is parasmaipada only. 28 more, taking the set
    // from 1603 to 1631.
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
SET="$SET" python3 /tmp/vidyut-full/slice10l/pin_ambiguous_10l.py      # pinned 1631
mise run fmt
```

The 28 new surfaces are seven rows' four each (`-ayata` / `-ayatAm` / `-ayetAm` / `-ayeta` on the ṇic branch); `10.0026 curRa~`'s four are `10.0143 cUrRa~`'s already.

- [ ] **Step 7: Run the full suite**

```bash
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS at 38160 cells, with `panini-prakriya` 445, `trace` 216, `paradigm` 30, `panini-data` 28.

Grep the goldens for the `check()` witnesses, which the test also enforces:

```bash
for f in Urjayati Orjayat cUrRayati gUrdayati kIrtayati kalpayati mlecCayati awwayati DrAsayati Drasati urjayati curRayati gurdayati kirtayati kFtayati karpayati mleCayati adwayati aqwayati Drasayati DrAsati; do
  printf "%s: %s\n" $f "$(grep -c "\"$f\"" crates/panini/tests/paradigm/data/*.rs | grep -v ':0' | sed 's#.*/##' | tr '\n' ' ')"; done
```

Expected:
- `cUrRayati` appears twice, in `curadi.rs` only.
- Every other Valid witness (`Urjayati` … `Drasati`) appears once, in `curadi.rs` only.
- The eleven Invalid shapes (`urjayati` … `DrAsati`) print nothing.

- [ ] **Step 8: Commit**

```bash
mise run lint
git branch --show-current      # curadi-10l
git add -A
git commit -m "feat(data): curādi's eight rule-bearing rows — √ūrj, √cūrṇ, √aṭṭ, √kṝt, √mlecch, √gūrd, √dhras, √kṛp

37584 → 38160 cells, 49160 → 49826 forms, ALTERNATES 11576 → 11666, 585 →
593 roots; pada-ambiguous surfaces 1603 → 1631. √dhras joins OPTIONAL_NIC
(182) by 2570, udit by its initial u~. Goldens generated cell-by-cell
equal to vidyut."
```

---

## Task 4: Audit, prior-trace diff, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`
- Modify: `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`
- Modify: `crates/panini/tests/paradigm/main.rs` (doc counts, audit prose), `crates/panini/tests/trace/curadi.rs` (module doc), `crates/panini-prakriya/src/tinanta/sanadi.rs` (module doc), `crates/panini-prakriya/src/tinanta/guna.rs` (one comment)
- Modify: `docs/superpowers/specs/2026-10-05-curadi-gana-10k-design.md`

**Interfaces:**
- Consumes: the finished engine, data and goldens. Produces no symbols.

- [ ] **Step 1: Update the audit harness**

Create `/tmp/vidyut-full/slice10l/audit_10l.py` if it is missing (sha256 `7c70f41a489b7bb4b0f9b00270d3ea5ecb3e39d75657f1244c4315b9888e2adf`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10l's edits to tools/audit/panini_full_audit.rs.
Every `old` must occur exactly once in its file; nothing is written if one
fails. Run from the worktree root."""
import sys
E = {
'tools/audit/panini_full_audit.rs': [
("""//!
//! What it compares: for each of the 585 curated roots, for each pada the root
//! admits (`Dhatu::padas`; two apiece for the 459 roots that admit both padas —
//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 419 curādi
//! roots by 1.3.74, seven more by 1.3.74 with ṇic and 1.3.72 without, and seven optional-ṇic ākusmīya or ā-garvīya roots by
""",
"""//!
//! What it compares: for each of the 593 curated roots, for each pada the root
//! admits (`Dhatu::padas`; two apiece for the 467 roots that admit both padas —
//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 427 curādi
//! roots by 1.3.74, seven more by 1.3.74 with ṇic and 1.3.72 without, and seven optional-ṇic ākusmīya or ā-garvīya roots by
"""),
("""//!
//! Corpus invariants, asserted: 585 roots, 37584 cells, 49160 forms. These are
//! facts about the repo, pinned by its own golden suite
//! (`derivation_set_shape_matches_the_audited_numbers`): 4176 root×pada×lakāra
//! blocks × 9 cells, plus 11576 `ALTERNATES` rows. If this harness's
//! enumeration disagrees, the harness is wrong.
""",
"""//!
//! Corpus invariants, asserted: 593 roots, 38160 cells, 49826 forms. These are
//! facts about the repo, pinned by its own golden suite
//! (`derivation_set_shape_matches_the_audited_numbers`): 4240 root×pada×lakāra
//! blocks × 9 cells, plus 11666 `ALTERNATES` rows. If this harness's
//! enumeration disagrees, the harness is wrong.
"""),
("""//!
//! Optionally dump the full 37584-cell table:
//!
""",
"""//!
//! Optionally dump the full 38160-cell table:
//!
"""),
("""
    assert_eq!(roots_seen.len(), 585, "curated roots");
    assert_eq!(n_cells, 37584, "cells: 4176 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 49160, "forms: 37584 cells + 11576 ALTERNATES rows");
    assert_eq!(
""",
"""
    assert_eq!(roots_seen.len(), 593, "curated roots");
    assert_eq!(n_cells, 38160, "cells: 4240 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 49826, "forms: 38160 cells + 11666 ALTERNATES rows");
    assert_eq!(
"""),
],
}
bad, out = [], {}
for p, edits in E.items():
    s = open(p).read()
    for old, new in edits:
        n = s.count(old)
        if n != 1:
            bad.append(f"{p}: {n}x {old[:70]!r}")
            continue
        s = s.replace(old, new)
    out[p] = s
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
for p, s in out.items():
    open(p, 'w').write(s)
print(f"applied {sum(len(e) for e in E.values())} edits to {len(E)} files")
```

```bash
python3 /tmp/vidyut-full/slice10l/audit_10l.py      # applied 4 edits to 1 files
```

The harness now asserts 593 / 38160 / 49826 and names 467 both-pada roots, 427 of them `Nic`.

- [ ] **Step 2: The prior-trace diff and the audit**

The dev-deps still point at this worktree from Task 3. Create `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_10l.rs` if it is missing (sha256 `36feced69614ef1ae3e583b28db18077986abaeb8f841c6db9b29782ca78ce88`). It lists 10l's rows literally so that it also builds against main, and it dumps every branch, blocked ones included:

```rust
//! THROWAWAY: slice 10l — dump every prior cell's branches, blocked ones
//! included, each with its credited-rule log.
use panini::Panini;
use panini_data::{Lakara as L, Purusha as P, Vacana as V};
/// Slice 10l's eight rows, listed literally so the dump also builds against
/// main.
const NEW: &[&str] = &["10.0023", "10.0026", "10.0037", "10.0155", "10.0170", "10.0180", "10.0270", "10.0278"];
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
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10l 2>/dev/null > "$DUMP/branch.txt")
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml   # must point at /workspace/crates
git -C /workspace branch --show-current   # main
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10l 2>/dev/null > "$DUMP/main.txt")
wc -l < "$DUMP/main.txt"; wc -l < "$DUMP/branch.txt"            # 55676 and 55676
grep -c "blocked=false" "$DUMP/main.txt"                        # 49160
cmp "$DUMP/main.txt" "$DUMP/branch.txt" && echo PRIOR-TRACES-IDENTICAL
```

Expected: `55676`, `55676`, `49160`, `PRIOR-TRACES-IDENTICAL`. This is the corpus-wide check that the four new rules and 8.4.41's arm change no prior root's trace, blocked branches included (goldens ignore traces). `/workspace` must be on `main` (Task 1) for the second dump. If `cmp` reports a difference, stop and report.

Then the audit. Repoint at this worktree again, copy the committed harness (never rewrite it), and run:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
cp tools/audit/panini_full_audit.rs $V/examples/
(cd $V && PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -8)
(cd $V && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -2)
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml
date -u +%F
```

- Expected from the honest run: `roots : 593`, `cells : 38160`, `forms (set sizes): 49826`, `live branches : 49826`, `blocked branches : 6552`, `differing cells  : 0`, `AUDIT PASSED: 38160 cells, 49826 forms, zero differences.`
- Expected from the `entry` control: `AUDIT FAILED: 36 differing cells.`

Do not use `mise -C`. If the honest run shows differences, stop and report, and edit nothing. Write down the date the last command prints: it is the audit's date for Step 3.

- [ ] **Step 3: The doc sweep, the audit record and the spec pointer**

Create `/tmp/vidyut-full/slice10l/docsweep_10l.py` if it is missing (sha256 `34a3ea55abf615f4079f2c22ff1b4cf7d16a6784f8507252cb319adf9bb895e6`). It takes the audit's date (`AUDIT_DATE`, Step 2's `date -u +%F`) and writes it into the audit record and AGENTS.md's audit chain. It covers:
- **ARCHITECTURE.md:** the stage table's `sanadi.rs` and `tripadi.rs` rows (and 6.1.73's stale "fires only on √vich"); the pin count (166) and its history; the gaṇa coverage line (490); the 7.1.35 / 8.4.56 census (519 parasmaipada columns, 74 ātmanepada-only, 467 both-pada, 1038 / 896 / 873 cells; 10l's rows in the 8.4.56 roster).
- **README.md:** the curādi line (490) and a 10l paragraph; the corpus (593 roots; 8460 of 38160 cells forked; 6788 / 925 / 361 / 367); the both-pada count (467); the pada-ambiguous count (1631) and its account.
- **AGENTS.md:** the cell count, the progress line (490), the fork census (6788 / 925 / 361; the six-form record to 367; 11666 + 38160 = 49826), the audit chain, and the `guna.rs:1233` note's corpus size.
- **`tools/audit/README.md`:** the asserted totals and the new top record.
- **`paradigm/main.rs`:** the `ALTERNATES` doc (11666), the mandatory-7.3.86 counts (146 of 153; 136 curādi, √kṛp's), the census doc's buckets and keys and 10l's key paragraph, and the audit chain.
- **`sanadi.rs`'s and `trace/curadi.rs`'s module docs; `guna.rs`'s corpus size (593); 10k's spec's 10l bullet,** pointing here and recording the three corrections.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10l's doc sweep — README, AGENTS.md, ARCHITECTURE.md,
the audit README's record, paradigm/main.rs's doc counts and audit chain,
the sanādi and trace module docs, guna.rs's corpus size and the 10k spec's
pointer. The audit date comes from $AUDIT_DATE.
Every `old` must occur exactly once in its file; nothing is written if one
fails. Run from the worktree root."""
import sys
E = {
'AGENTS.md': [
("""- Grammar changes are gated by the golden paradigm test
  (`crates/panini/tests/paradigm/`, 37584 cells, ten gaṇas, nine complete —
  tanādi closing at 10/10 in slice 8b (nine of its ten dhātupāṭha rows
""",
"""- Grammar changes are gated by the golden paradigm test
  (`crates/panini/tests/paradigm/`, 38160 cells, ten gaṇas, nine complete —
  tanādi closing at 10/10 in slice 8b (nine of its ten dhātupāṭha rows
"""),
("""and curādi (10) opened in slice 10a at 4 of its 509 rows (√cur, √laḍ, √bhakṣ,
√bhūṣ), at 8 after slice 10b curated the ākusmīya √cit, √vṛṣ, √mad and √kusm, at 41 after slice 10c curated thirty-three more ākusmīya roots, at 47 after slice 10d curated the jñapādi √jñap, √yam, √cah, √cap, √rah and √bal, at 139 after slice 10e curated ninety-two adanta roots, at 149 after slice 10f curated the ten optional-ṇic rows, at 208 after slice 10g curated fifty-nine more, at 258 after slice 10h curated the fifty ādhṛṣīya rows, at 319 after slice 10i curated the fifty-nine āsvadīya rows, √pṝ and √ghuṣ, at 327 after slice 10j curated the eight ajanta rows √smiṅ, √ci, √ghṛ, √gṛ, √yu, √jñā, √cyu and √bhū, at 482 after slice 10k curated the 155 plain obligatory-ṇic rows —
  `PARADIGM`
    stays one-form-per-cell: a cell forked by an optional rule keeps its
    other forms — a second (6742 cells), a third (911 cells), a fourth
    (359
    cells, rudhādi's √piṣ and — new in slice 7d — √śiṣ loṭ madhyama eka, and
""",
"""and curādi (10) opened in slice 10a at 4 of its 509 rows (√cur, √laḍ, √bhakṣ,
√bhūṣ), at 8 after slice 10b curated the ākusmīya √cit, √vṛṣ, √mad and √kusm, at 41 after slice 10c curated thirty-three more ākusmīya roots, at 47 after slice 10d curated the jñapādi √jñap, √yam, √cah, √cap, √rah and √bal, at 139 after slice 10e curated ninety-two adanta roots, at 149 after slice 10f curated the ten optional-ṇic rows, at 208 after slice 10g curated fifty-nine more, at 258 after slice 10h curated the fifty ādhṛṣīya rows, at 319 after slice 10i curated the fifty-nine āsvadīya rows, √pṝ and √ghuṣ, at 327 after slice 10j curated the eight ajanta rows √smiṅ, √ci, √ghṛ, √gṛ, √yu, √jñā, √cyu and √bhū, at 482 after slice 10k curated the 155 plain obligatory-ṇic rows, at 490 after slice 10l curated the eight rule-bearing rows √ūrj, √cūrṇ, √aṭṭ, √kṝt, √mlecch, √gūrd, √dhras and √kṛp —
  `PARADIGM`
    stays one-form-per-cell: a cell forked by an optional rule keeps its
    other forms — a second (6788 cells), a third (925 cells), a fourth
    (361
    cells, rudhādi's √piṣ and — new in slice 7d — √śiṣ loṭ madhyama eka, and
"""),
("""    ādhṛṣīya rows', and — new in slice 10i — the sixty-one āsvadīya, √pṝ
    and √ghuṣ rows') and
    — the loṭ parasmaipada cells of
""",
"""    ādhṛṣīya rows', and — new in slice 10i — the sixty-one āsvadīya, √pṝ
    and √ghuṣ rows', and — new in slice 10l — √dhras's) and
    — the loṭ parasmaipada cells of
"""),
("""    eka to 241, and slice 10i's sixty-one rows (loṭ prathama and madhyama
    eka) to 363, and slice 10j's √ci (laṅ and vidhiliṅ prathama eka) to 365 —
    a fourth
""",
"""    eka to 241, and slice 10i's sixty-one rows (loṭ prathama and madhyama
    eka) to 363, and slice 10j's √ci (laṅ and vidhiliṅ prathama eka) to 365,
    and slice 10l's √dhras (loṭ prathama and madhyama eka) to 367 —
    a fourth
"""),
("""    eka, the eight nine-form cells and the record — in
    `ALTERNATES` (11576 rows in all, so 37584 + 11576 = 49160 forms total); √bhuj
    joins neither fork record — its forks stack only 7.1.35 and 8.4.56, the
""",
"""    eka, the eight nine-form cells and the record — in
    `ALTERNATES` (11666 rows in all, so 38160 + 11666 = 49826 forms total); √bhuj
    joins neither fork record — its forks stack only 7.1.35 and 8.4.56, the
"""),
("""  430 roots), and that by curādi 10k's (`tools/audit/README.md`'s 2026-10-05 10k
  entry, 37584 cells / 49160 forms / 585 roots).
  Three new `Rule`s are behind it, all root-keyed to √kṛ and all in
""",
"""  430 roots), and that by curādi 10k's (`tools/audit/README.md`'s 2026-10-05 10k
  entry, 37584 cells / 49160 forms / 585 roots), and that by curādi 10l's
  (`tools/audit/README.md`'s 2026-10-05 10l entry, 38160 cells / 49826 forms /
  593 roots).
  Three new `Rule`s are behind it, all root-keyed to √kṛ and all in
"""),
("""  landed. `guna.rs:1233`'s own claim ("1872 goldens move") stays stale
  only in the ordinary corpus-size sense, not wrong in kind: 37584 goldens
  would move today. Neither comment was touched by tanādi 8a or 8b, consistent
""",
"""  landed. `guna.rs:1233`'s own claim ("1872 goldens move") stays stale
  only in the ordinary corpus-size sense, not wrong in kind: 38160 goldens
  would move today. Neither comment was touched by tanādi 8a or 8b, consistent
"""),
],
'README.md': [
("""*stoḥ ścunā ścuḥ* gaining its converse arm, a stu after a ścu, guarded by the
8.4.44 *śāt* exemption. *curādi* (10) is **open** at 482 of its 509
dhātupāṭha rows: √cur (`10.0001`, *corayati*), √laḍ (`10.0010`,
""",
"""*stoḥ ścunā ścuḥ* gaining its converse arm, a stu after a ścu, guarded by the
8.4.44 *śāt* exemption. *curādi* (10) is **open** at 490 of its 509
dhātupāṭha rows: √cur (`10.0001`, *corayati*), √laḍ (`10.0010`,
"""),
("""are the same root once 6.1.64 makes the `z` an `s`; both are curated.
Every curādi root takes ṇic (3.1.25) before the vikaraṇa; a new first
""",
"""are the same root once 6.1.64 makes the `z` an `s`; both are curated.
Slice 10l curated eight rows that bring rules of their own, all ubhayapadī
by 1.3.74: 8.2.78 *upadhāyāṃ ca* lengthens an ik before an `r` upadhā and a
final hal (*ūrjayati*, *cūrṇayati*, *gūrdayati*); 7.1.101 *upadhāyāś ca*
makes √kṝt's `F` upadhā `ir` before ṇic, ahead of guṇa (*kīrtayati*);
8.2.18 *kṛpo ro laḥ* gives √kṛp its `l` (*kalpayati*); 6.1.75 *dīrghāt*
adds tuk after √mlecch's long `e` (*mlecchayati*); and 8.4.41 *ṣṭunā ṣṭuḥ*
now also retroflexes a stu before a ṭu (*aṭṭayati*). √dhras (`10.0270
u~Drasa~`) is udit by its initial `u~`, so its ṇic is optional
(*dhrāsayati* beside *dhrasati*).
Every curādi root takes ṇic (3.1.25) before the vikaraṇa; a new first
"""),
("""(which padas a root admits is a curated verdict on its table row), over a
curated 585-root set, in four lakāras: *laṭ* (present), *laṅ* (imperfect), *loṭ*
(imperative), and *vidhiliṅ* (optative). A cell may have more than one valid
form where an optional (*vikalpa*) sūtra applies — `hinvaH` and `hinuvaH` are
both correct — and in fact 8396 of the 37584 cells hold more than one form: 6742
hold two, 911 hold three (`Bavatu`, `BavatAd`, `BavatAt`, and — new in
slice 3b — √hrī's loṭ prathama and madhyama eka, and — new in slice 3c —
""",
"""(which padas a root admits is a curated verdict on its table row), over a
curated 593-root set, in four lakāras: *laṭ* (present), *laṅ* (imperfect), *loṭ*
(imperative), and *vidhiliṅ* (optative). A cell may have more than one valid
form where an optional (*vikalpa*) sūtra applies — `hinvaH` and `hinuvaH` are
both correct — and in fact 8460 of the 38160 cells hold more than one form: 6788
hold two, 925 hold three (`Bavatu`, `BavatAd`, `BavatAt`, and — new in
slice 3b — √hrī's loṭ prathama and madhyama eka, and — new in slice 3c —
"""),
("""ubhayapadī adanta roots', and — new in slice 10k — the 155 plain
obligatory-ṇic rows', each by
7.1.35/8.4.56, √bhas's laṅ madhyama eka by 8.2.74/8.4.56; slice 10f's
""",
"""ubhayapadī adanta roots', and — new in slice 10k — the 155 plain
obligatory-ṇic rows', and — new in slice 10l — seven of its eight rows', each by
7.1.35/8.4.56, √bhas's laṅ madhyama eka by 8.2.74/8.4.56; slice 10f's
"""),
("""and √ci 68 three-form cells, its three readings where neither 7.1.35 nor
8.4.56 forks),
359 hold four
(rudhādi's √piṣ loṭ madhyama eka, and — new in slice 7d — √śiṣ's, and — new
""",
"""and √ci 68 three-form cells, its three readings where neither 7.1.35 nor
8.4.56 forks, and slice 10l's √dhras 32 two-form cells, a ṇic and a
ṇic-less reading each),
361 hold four
(rudhādi's √piṣ loṭ madhyama eka, and — new in slice 7d — √śiṣ's, and — new
"""),
("""and — new in slice 10h — forty-eight of the fifty ādhṛṣīya rows', the same
way, and — new in slice 10i — the sixty-one rows', the same way),
ten hold
""",
"""and — new in slice 10h — forty-eight of the fifty ādhṛṣīya rows', the same
way, and — new in slice 10i — the sixty-one rows', the same way, and — new
in slice 10l — √dhras's, the same way),
ten hold
"""),
("""7.1.35/6.4.115/8.4.56, and — new in slice 3c2 — √hā's, forking on
7.1.35/6.4.116/8.4.56), and 365 hold six — the loṭ
parasmaipada madhyama eka of rudhādi's √kṛt, √rudh, √bhid, √kṣud, √tṛd, √und
""",
"""7.1.35/6.4.115/8.4.56, and — new in slice 3c2 — √hā's, forking on
7.1.35/6.4.116/8.4.56), and 367 hold six — the loṭ
parasmaipada madhyama eka of rudhādi's √kṛt, √rudh, √bhid, √kṣud, √tṛd, √und
"""),
("""the same way; and, new in slice 10j, √ci's laṅ and vidhiliṅ parasmaipada
prathama eka, three readings × 8.4.56. One cell — new in slice 3c2 — holds seven: √hā's (`03.0009`) loṭ
parasmaipada madhyama eka, `jahIhi` / `jahihi` / `jahAhi` / `jahItAd` /
""",
"""the same way; and, new in slice 10j, √ci's laṅ and vidhiliṅ parasmaipada
prathama eka, three readings × 8.4.56; and, new in slice 10l, √dhras's loṭ
parasmaipada prathama and madhyama eka, two readings × the tātaṅ triple. One cell — new in slice 3c2 — holds seven: √hā's (`03.0009`) loṭ
parasmaipada madhyama eka, `jahIhi` / `jahihi` / `jahAhi` / `jahItAd` /
"""),
("""roots take. A root may also admit **both**
padas — 459 roots that admit both padas in the curated set
(twenty-five ubhayapadī by 1.3.72: √nī, √tud, √rudh, √bhid, √kṣud, √yuj,
""",
"""roots take. A root may also admit **both**
padas — 467 roots that admit both padas in the curated set
(twenty-five ubhayapadī by 1.3.72: √nī, √tud, √rudh, √bhid, √kṣud, √yuj,
"""),
("""ṇic), slice 10i's sixty-one rows, slice 10j's √ghṛ, √jñā, √cyu, √bhū and
√ci (by 1.3.72 too, without ṇic) and slice 10k's 155 rows by 1.3.74; and slice 10f's six optional-ṇic ākusmīya roots and
`garva`, ātmanepadī by 10.0496 / 10.0497 with ṇic and parasmaipadī by
""",
"""ṇic), slice 10i's sixty-one rows, slice 10j's √ghṛ, √jñā, √cyu, √bhū and
√ci (by 1.3.72 too, without ṇic), slice 10k's 155 rows and slice 10l's eight by 1.3.74; and slice 10f's six optional-ṇic ākusmīya roots and
`garva`, ātmanepadī by 10.0496 / 10.0497 with ṇic and parasmaipadī by
"""),
("""engine's √van has no parasmaipada branch to collide against.
1603 of the pinned (`PARADIGM`) surfaces are pada-ambiguous, each of them a
pinned cell in both padas at once — `rundDAm`, for instance, is √rudh's loṭ
""",
"""engine's √van has no parasmaipada branch to collide against.
1631 of the pinned (`PARADIGM`) surfaces are pada-ambiguous, each of them a
pinned cell in both padas at once — `rundDAm`, for instance, is √rudh's loṭ
"""),
("""already supplies them (sixteen homographs) or an in-slice pair shares them
(`pfT` and `parT`, `pul` twice, `pAl` and `pal`, `sAntv` twice). The
enumeration is not
""",
"""already supplies them (sixteen homographs) or an in-slice pair shares them
(`pfT` and `parT`, `pul` twice, `pAl` and `pal`, `sAntv` twice); slice
10l's eight add 28 more, four each but `10.0026 curRa~`'s, which its
homograph `10.0143 cUrRa~` already supplies. The
enumeration is not
"""),
("""`crates/panini/tests/paradigm/main.rs` walks `PARADIGM` and asserts the whole
set, all 1603. It is therefore a list of ambiguous **pinned cells**, not of every
pada-ambiguous surface: since slice 10h's ṇic-less branches are live in both
""",
"""`crates/panini/tests/paradigm/main.rs` walks `PARADIGM` and asserts the whole
set, all 1631. It is therefore a list of ambiguous **pinned cells**, not of every
pada-ambiguous surface: since slice 10h's ṇic-less branches are live in both
"""),
],
'crates/panini-prakriya/src/tinanta/guna.rs': [
("""    // E/O → Ay/Av, but those two arms are dropped here: within the current
    // 585-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)
    // at the point this rule runs. `vrddhi_of` (the only source of E/O in
""",
"""    // E/O → Ay/Av, but those two arms are dropped here: within the current
    // 593-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)
    // at the point this rule runs. `vrddhi_of` (the only source of E/O in
"""),
],
'crates/panini-prakriya/src/tinanta/sanadi.rs': [
("""//! 6.1.54, 7.2.116, the vārttika 7.3.37.2, 7.3.36, 7.2.115, 6.1.78, 6.4.92,
//! 7.2.114, 6.1.73, 7.3.86, 3.1.32 — opened by the dhātupāṭha gaṇasūtra
//! 10.0493, which credits a jñapādi root's mit-tva, then eight vikalpas that
//! fork a root whose ṇic is optional into its ṇic and ṇic-less branches (the
//! gaṇasūtras 10.0498 and 10.0499 and the Kaumudī's 2564, 2565, 2570, 2571,
//! 2573.1, 2573.3), a ninth (2573.2) that forks `pata`'s ṇic branch on its
//! final `a`, and two more gaṇasūtras, 10.0496 and 10.0497, which settle an
//! ākusmīya or ā-garvīya root's pada, all before ṇic is added.
//!
""",
"""//! 6.1.54, 7.2.116, the vārttika 7.3.37.2, 7.3.36, 7.2.115, 6.1.78, 6.4.92,
//! 7.2.114, 6.1.73, 6.1.75, 7.1.101, 7.3.86, 3.1.32 — opened by the
//! dhātupāṭha gaṇasūtra 10.0493, which credits a jñapādi root's mit-tva,
//! then eight vikalpas that fork a root whose ṇic is optional into its ṇic
//! and ṇic-less branches (the gaṇasūtras 10.0498 and 10.0499 and the
//! Kaumudī's 2564, 2565, 2570, 2571, 2573.1, 2573.3), a ninth (2573.2) that
//! forks `pata`'s ṇic branch on its final `a`, and two more gaṇasūtras,
//! 10.0496 and 10.0497, which settle an ākusmīya or ā-garvīya root's pada,
//! all before ṇic is added.
//!
"""),
("""//! 10.0497 on `Tag::AaGarviya`, 10.0493 on `Tag::Mit`, 3.1.25 on
//! `Tag::Curadi`, 3.1.28 on `Tag::Aya` with no ṇic taken, 6.1.73 on its own
//! saṁhitā condition, and the rest on the pratyaya at `NIC`: 3.4.114 and
//! 3.1.32 on ṇic or āya, 6.4.48, 7.2.114 and 7.3.86 on its being
//! ārdhadhātuka, the others on ṇic's ṇit (6.4.48 on an `a`-final aṅga as
""",
"""//! 10.0497 on `Tag::AaGarviya`, 10.0493 on `Tag::Mit`, 3.1.25 on
//! `Tag::Curadi`, 3.1.28 on `Tag::Aya` with no ṇic taken, 6.1.73 and 6.1.75
//! on their own saṁhitā condition, and the rest on the pratyaya at `NIC`:
//! 3.4.114 and 3.1.32 on ṇic or āya, 6.4.48, 7.2.114 and 7.3.86 on its being
//! ārdhadhātuka, the others on ṇic's ṇit (6.4.48 on an `a`-final aṅga as
"""),
("""//! aṅga, 7.2.115 on an ac-final one, 6.1.78 on an ec-final one, 7.2.114 on
//! `Tag::Mrj`, and 7.2.116 and 7.3.86 decline on 6.4.48's `Tag::AtLopa`).
//! 6.1.73 and 3.1.28 carry no gaṇa guard, only their own conditions (3.1.28
//! reads the āya rows); on today's corpus the gaṇas 1–9 add nothing and
//! record nothing, and 6.1.73 fires only on √vich and √pich (slice 10k),
//! which `the_10i_aya_and_tuk_fire_only_on_their_rows` pins.

""",
"""//! aṅga, 7.2.115 on an ac-final one, 6.1.78 on an ec-final one, 7.2.114 on
//! `Tag::Mrj`, 7.1.101 on an `F` upadhā, and 7.2.116 and 7.3.86 decline on
//! 6.4.48's `Tag::AtLopa`). 6.1.73, 6.1.75 and 3.1.28 carry no gaṇa guard,
//! only their own conditions (3.1.28 reads the āya rows); on today's corpus
//! the gaṇas 1–9 add nothing and record nothing, 6.1.73 fires only on √vich
//! and √pich (slice 10k), which `the_10i_aya_and_tuk_fire_only_on_their_rows`
//! pins, and 6.1.75 only on √mlecch and 7.1.101 only on √kṝt (slice 10l),
//! which `the_10l_rules_fire_only_on_their_rows` pins.

"""),
],
'crates/panini/tests/paradigm/main.rs': [
("""
/// `ALTERNATES` is otherwise 11576 bare strings, and a string can be right for
/// the wrong reason — `BavatAt` is a real form whether or not 8.4.56 is what
""",
"""
/// `ALTERNATES` is otherwise 11666 bare strings, and a string can be right for
/// the wrong reason — `BavatAt` is a real form whether or not 8.4.56 is what
"""),
("""/// since slice 10a, as the sanādi entry before ṇic, so a key
/// naming 7.3.86 does not by itself mean the rule was optional. 144 of the 151 `7.3.86+8.4.56` keys are the mandatory firing (ten juhotyādi: 3e's laṅ eka cells and 3f's √kit and √dhiṣ ones; 134 curādi: slice 10a's √cur laṅ and vidhiliṅ prathama eka, and slice 10g's √div and √śṛdh, slice 10h's sixteen laghu-ik ādhṛṣīya rows, slice 10i's eleven laghu-ik rows on their ṇic branch and slice 10k's thirty-seven laghu-ik rows, where the firing is the sanādi entry before ṇic; the other seven are the tanādi vikalpa), and so is the 7.3.86 of the one `7.3.86+8.2.75` key (√kit's `acikeH`), of every `7.3.86+7.1.35`/`7.3.86+7.1.35+8.4.56` key (the same curādi roots' loṭ tātaṅ cells — the only keys where 7.3.86 precedes 7.1.35, because the sanādi stage runs first), and of every `2570+…7.3.86…`, `2571+…7.3.86…`, `10.0498+…7.3.86…` and `10.0499+…7.3.86…` key (a ṇic-less root's guṇa).
#[test]
""",
"""/// since slice 10a, as the sanādi entry before ṇic, so a key
/// naming 7.3.86 does not by itself mean the rule was optional. 146 of the 153 `7.3.86+8.4.56` keys are the mandatory firing (ten juhotyādi: 3e's laṅ eka cells and 3f's √kit and √dhiṣ ones; 136 curādi: slice 10a's √cur laṅ and vidhiliṅ prathama eka, and slice 10g's √div and √śṛdh, slice 10h's sixteen laghu-ik ādhṛṣīya rows, slice 10i's eleven laghu-ik rows on their ṇic branch, slice 10k's thirty-seven laghu-ik rows and slice 10l's √kṛp, where the firing is the sanādi entry before ṇic; the other seven are the tanādi vikalpa), and so is the 7.3.86 of the one `7.3.86+8.2.75` key (√kit's `acikeH`), of every `7.3.86+7.1.35`/`7.3.86+7.1.35+8.4.56` key (the same curādi roots' loṭ tātaṅ cells — the only keys where 7.3.86 precedes 7.1.35, because the sanādi stage runs first), and of every `2570+…7.3.86…`, `2571+…7.3.86…`, `10.0498+…7.3.86…` and `10.0499+…7.3.86…` key (a ṇic-less root's guṇa).
#[test]
"""),
("""/// with the same k = 3 against the 2³ bound of eight:
/// 37584 cells total (4176 root×lakāra blocks × 9), of which 29188 hold exactly one form, 6742 hold two, 911 hold three (√hrī's loṭ prathama and madhyama
/// eka, new in slice 3b, √dā's and √dhā's, new in slice 3c, and √gā's, new in
/// slice 3c2, and the six ṛ-roots', new in slice 3d, and √ṛ's, new in slice 3d2, and √ṇij's, √vij's and √viṣ's, new in slice 3e, and √kit's, √tur's, √dhiṣ's and √dhan's, new in slice 3f, and √bhas's, new in slice 3f2, and √jan's, new in slice 3f3, and the four curādi roots', new in slice 10a, and the six jñapādi roots', new in slice 10d, and the eighty-three ubhayapadī adanta roots', new in slice 10e, and the 155 plain obligatory-ṇic rows', new in slice 10k, each by
/// 7.1.35/8.4.56, plus √bhas's laṅ madhyama eka, by 8.2.74/8.4.56; slice 10f's
""",
"""/// with the same k = 3 against the 2³ bound of eight:
/// 38160 cells total (4240 root×lakāra blocks × 9), of which 29700 hold exactly one form, 6788 hold two, 925 hold three (√hrī's loṭ prathama and madhyama
/// eka, new in slice 3b, √dā's and √dhā's, new in slice 3c, and √gā's, new in
/// slice 3c2, and the six ṛ-roots', new in slice 3d, and √ṛ's, new in slice 3d2, and √ṇij's, √vij's and √viṣ's, new in slice 3e, and √kit's, √tur's, √dhiṣ's and √dhan's, new in slice 3f, and √bhas's, new in slice 3f2, and √jan's, new in slice 3f3, and the four curādi roots', new in slice 10a, and the six jñapādi roots', new in slice 10d, and the eighty-three ubhayapadī adanta roots', new in slice 10e, and the 155 plain obligatory-ṇic rows', new in slice 10k, and seven of the eight rule-bearing rows', new in slice 10l, each by
/// 7.1.35/8.4.56, plus √bhas's laṅ madhyama eka, by 8.2.74/8.4.56; slice 10f's
"""),
("""/// slice 10j's √ghṛ, √jñā, √cyu and √bhū eight two-form and eight three-form
/// cells as √cur's, and √ci 68 three-form ones), 359 hold four (piṣ's loṭ madhyama eka, the deepest
/// fork added in 7b, Siz's loṭ parasmaipada madhyama eka (slice 7d), and — new in
""",
"""/// slice 10j's √ghṛ, √jñā, √cyu and √bhū eight two-form and eight three-form
/// cells as √cur's, and √ci 68 three-form ones, and slice 10l's √dhras 32
/// two-form cells, a ṇic and a ṇic-less reading each), 361 hold four (piṣ's loṭ madhyama eka, the deepest
/// fork added in 7b, Siz's loṭ parasmaipada madhyama eka (slice 7d), and — new in
"""),
("""/// fifty ādhṛṣīya rows', the same way; and — new in slice 10i — the
/// sixty-one āsvadīya, √pṝ and √ghuṣ rows', the same way), and
/// — the sharpest branch-count witnesses in
""",
"""/// fifty ādhṛṣīya rows', the same way; and — new in slice 10i — the
/// sixty-one āsvadīya, √pṝ and √ghuṣ rows', the same way; and — new in slice
/// 10l — √dhras's, the same way), and
/// — the sharpest branch-count witnesses in
"""),
("""/// prathama eka, forking on 7.1.35/6.4.116/8.4.56) and
/// 365
/// hold six (√kṛt's loṭ madhyama eka, `kfndDi`/`kfnDi`'s cell, ruD's loṭ
""",
"""/// prathama eka, forking on 7.1.35/6.4.116/8.4.56) and
/// 367
/// hold six (√kṛt's loṭ madhyama eka, `kfndDi`/`kfnDi`'s cell, ruD's loṭ
"""),
("""/// the same way; and — new in slice 10j — √ci's laṅ and vidhiliṅ parasmaipada
/// prathama eka, three readings × 8.4.56), one — new in
/// slice 3c2 — holds SEVEN: √hā's loṭ parasmaipada madhyama eka, where 6.4.117
""",
"""/// the same way; and — new in slice 10j — √ci's laṅ and vidhiliṅ parasmaipada
/// prathama eka, three readings × 8.4.56; and — new in slice 10l — √dhras's
/// loṭ parasmaipada prathama and madhyama eka, two readings × the tātaṅ
/// triple), one — new in
/// slice 3c2 — holds SEVEN: √hā's loṭ parasmaipada madhyama eka, where 6.4.117
"""),
("""/// `ALTERNATES`
/// itself has 11576 rows, keyed 882 `8.4.56`, 874 `7.1.35`, 874 `7.1.35+8.4.56`,
/// 2 `3.4.111`, 72 `6.4.107`, 145 `8.4.65`, 8 `8.2.75`, 2 `8.2.74` (√hiṃs's ahinaH and, new in slice 3f2, √bhas's abaBaH), 16
""",
"""/// `ALTERNATES`
/// itself has 11666 rows, keyed 896 `8.4.56`, 888 `7.1.35`, 888 `7.1.35+8.4.56`,
/// 2 `3.4.111`, 72 `6.4.107`, 145 `8.4.65`, 8 `8.2.75`, 2 `8.2.74` (√hiṃs's ahinaH and, new in slice 3f2, √bhas's abaBaH), 16
"""),
("""/// ik-upadhā fork), 8 `7.1.35+7.3.86`, 8 `7.1.35+7.3.86+8.4.56`, 8
/// `7.3.86+6.4.107`, 151 `7.3.86+8.4.56` (144 of them name the MANDATORY
/// 7.3.86, through the id it shares with the tanādi vikalpa arm: ten juhotyādi, slice 3e's
/// laṅ prathama and madhyama eka cells and slice 3f's √kit and √dhiṣ ones, whose root guṇa 7.3.86 credits, and 134 curādi, slice 10a's √cur laṅ and vidhiliṅ prathama eka and slice 10g's √div and √śṛdh, slice 10h's sixteen laghu-ik ādhṛṣīya rows', slice 10i's eleven laghu-ik rows' ṇic-branch ones and slice 10k's thirty-seven laghu-ik rows', whose guṇa before ṇic the sanādi 7.3.86 credits; the other seven are the tanādi vikalpa), 1 `7.3.86+8.2.75` (√kit's acikeH, the same mandatory 7.3.86), 23 `6.4.115`, 2 `7.1.35+6.4.115`,
/// 2 `7.1.35+6.4.115+8.4.56`, and 1 `6.4.115+8.4.56`, 14 `6.4.116`, 1 `6.4.117`, 2 `7.1.35+6.4.116` and 2
/// `7.1.35+6.4.116+8.4.56`, 9 `6.4.43` and 1 `6.4.43+8.4.56` (slice 3f3's √jan), 134 `7.3.86+7.1.35` and 134
/// `7.3.86+7.1.35+8.4.56` (slice 10a's √cur, its sanādi 7.3.86 ahead of 7.1.35, and slice 10g's √śṛdh and
/// √div, slice 10h's sixteen laghu-ik ādhṛṣīya rows and slice 10i's eleven laghu-ik rows on their ṇic
/// branch, and slice 10k's thirty-seven laghu-ik rows), and slice 10f's twenty-one
/// keys on its five Kaumudī vikalpa ids, at their counts as of slice 10g: 36 `2573.1`, 72 `2573.2`, 72 `2573.3`, 114 apiece
""",
"""/// ik-upadhā fork), 8 `7.1.35+7.3.86`, 8 `7.1.35+7.3.86+8.4.56`, 8
/// `7.3.86+6.4.107`, 153 `7.3.86+8.4.56` (146 of them name the MANDATORY
/// 7.3.86, through the id it shares with the tanādi vikalpa arm: ten juhotyādi, slice 3e's
/// laṅ prathama and madhyama eka cells and slice 3f's √kit and √dhiṣ ones, whose root guṇa 7.3.86 credits, and 136 curādi, slice 10a's √cur laṅ and vidhiliṅ prathama eka and slice 10g's √div and √śṛdh, slice 10h's sixteen laghu-ik ādhṛṣīya rows', slice 10i's eleven laghu-ik rows' ṇic-branch ones, slice 10k's thirty-seven laghu-ik rows' and slice 10l's √kṛp's, whose guṇa before ṇic the sanādi 7.3.86 credits; the other seven are the tanādi vikalpa), 1 `7.3.86+8.2.75` (√kit's acikeH, the same mandatory 7.3.86), 23 `6.4.115`, 2 `7.1.35+6.4.115`,
/// 2 `7.1.35+6.4.115+8.4.56`, and 1 `6.4.115+8.4.56`, 14 `6.4.116`, 1 `6.4.117`, 2 `7.1.35+6.4.116` and 2
/// `7.1.35+6.4.116+8.4.56`, 9 `6.4.43` and 1 `6.4.43+8.4.56` (slice 3f3's √jan), 136 `7.3.86+7.1.35` and 136
/// `7.3.86+7.1.35+8.4.56` (slice 10a's √cur, its sanādi 7.3.86 ahead of 7.1.35, and slice 10g's √śṛdh and
/// √div, slice 10h's sixteen laghu-ik ādhṛṣīya rows and slice 10i's eleven laghu-ik rows on their ṇic
/// branch, and slice 10k's thirty-seven laghu-ik rows, and slice 10l's √kṛp), and slice 10f's twenty-one
/// keys on its five Kaumudī vikalpa ids, at their counts as of slice 10g: 36 `2573.1`, 72 `2573.2`, 72 `2573.3`, 114 apiece
"""),
("""/// `7.1.35+8.4.56` and 74 apiece into `7.3.86+8.4.56`, `7.3.86+7.1.35` and
/// `7.3.86+7.1.35+8.4.56` (its thirty-seven laghu-ik rows) — √kṛ (slice 8b) adds six more
/// rows, all folded into the pre-existing `8.4.56`/`7.1.35`/`7.1.35+8.4.56`
""",
"""/// `7.1.35+8.4.56` and 74 apiece into `7.3.86+8.4.56`, `7.3.86+7.1.35` and
/// `7.3.86+7.1.35+8.4.56` (its thirty-seven laghu-ik rows). Slice 10l opens
/// no key either: it adds 36 to `2570` and 2 apiece to `2570+8.4.56`,
/// `2570+7.1.35` and `2570+7.1.35+8.4.56` (√dhras's ṇic-less branch), and
/// folds 14 rows apiece into `8.4.56`, `7.1.35` and `7.1.35+8.4.56` and 2
/// apiece into `7.3.86+8.4.56`, `7.3.86+7.1.35` and `7.3.86+7.1.35+8.4.56`
/// (√kṛp, its guṇa before ṇic) — √kṛ (slice 8b) adds six more
/// rows, all folded into the pre-existing `8.4.56`/`7.1.35`/`7.1.35+8.4.56`
"""),
("""/// roots with zero differences, its `entry` negative control verified
/// failing (36 √bhū cells). √tṛh joins none of the fork
/// records: its deepest cells hold three forms, because 8.3.13 Qo Qe lopaH
""",
"""/// roots with zero differences, its `entry` negative control verified
/// failing (36 √bhū cells), and curādi 10l's re-ran it at the same commit
/// over all 38160 cells / 49826 forms / 593 roots with zero differences, its
/// `entry` negative control verified failing (36 √bhū cells). √tṛh joins none of the fork
/// records: its deepest cells hold three forms, because 8.3.13 Qo Qe lopaH
"""),
],
'crates/panini/tests/trace/curadi.rs': [
("""//! √vich's every branch the sanādi 6.1.73's tuk before 3.1.32, as √pich's
//! (slice 10k) ṇic branch does.

""",
"""//! √vich's every branch the sanādi 6.1.73's tuk before 3.1.32, as √pich's
//! (slice 10k) ṇic branch does, and √mlecch's the sanādi 6.1.75's (slice
//! 10l). √kṝt's ṇic branch has 7.1.101 before 3.1.32, and no 7.3.86.

"""),
],
'docs/ARCHITECTURE.md': [
("""|---|---|---|
| `sanadi.rs` | 10.0493, 10.0498, 10.0499, 2564, 2565, 2570, 2571, 2573.1, 2573.3, 2573.2, 10.0496, 10.0497, 3.1.25, 3.1.28, 1.3.9, 3.4.114, 6.4.48, 6.1.54, 7.2.116, 7.3.37.2, 7.3.36, 7.2.115, 6.1.78, 6.4.92, 7.2.114, 6.1.73, 7.3.86, 3.1.32 — the jñapādi's mit-tva, the optional-ṇic fork and the ākusmīya and ā-garvīya pada, then ṇic, or āya where √dhūp and √vich take none, the adanta root's final `a`, √ci's optional `A`, the root's vṛddhi or guṇa or √dhū's and √prī's nuk or an ā-final aṅga's puk, the mit root's short upadhā, √vich's tuk ahead of guṇa, and the pratyaya's folding into the dhātu (curādi only today: 6.1.73 has no gaṇa guard, and fires only on √vich) | before 3.1.68, before any tiṅ |
| `samjna.rs` | 2567, 1.3.12, 1.3.66, 1.3.72, 1.3.74, 1.3.78, 3.4.78, 1.3.9, 1.2.4 | before 3.1.68 |
""",
"""|---|---|---|
| `sanadi.rs` | 10.0493, 10.0498, 10.0499, 2564, 2565, 2570, 2571, 2573.1, 2573.3, 2573.2, 10.0496, 10.0497, 3.1.25, 3.1.28, 1.3.9, 3.4.114, 6.4.48, 6.1.54, 7.2.116, 7.3.37.2, 7.3.36, 7.2.115, 6.1.78, 6.4.92, 7.2.114, 6.1.73, 6.1.75, 7.1.101, 7.3.86, 3.1.32 — the jñapādi's mit-tva, the optional-ṇic fork and the ākusmīya and ā-garvīya pada, then ṇic, or āya where √dhūp and √vich take none, the adanta root's final `a`, √ci's optional `A`, the root's vṛddhi or guṇa or √dhū's and √prī's nuk or an ā-final aṅga's puk, the mit root's short upadhā, √vich's, √pich's and √mlecch's tuk and √kṝt's `ir` ahead of guṇa, and the pratyaya's folding into the dhātu (curādi only today: 6.1.73 and 6.1.75 have no gaṇa guard, and fire only on √vich and √pich, and on √mlecch) | before 3.1.68, before any tiṅ |
| `samjna.rs` | 2567, 1.3.12, 1.3.66, 1.3.72, 1.3.74, 1.3.78, 3.4.78, 1.3.9, 1.2.4 | before 3.1.68 |
"""),
("""| `adesha.rs` | 6.1.97, 6.1.101 (their aṅga–śap entries), 6.1.101 … 6.1.96, 6.4.106, 6.4.107, 6.1.90, 6.1.88 … 6.4.101, 6.4.111 | after 3.1.68 |
| `tripadi.rs` | 8.2.77, 8.2.23, 8.2.25, 8.2.26, 8.2.30, 8.2.31, 8.2.39, 8.2.40, 8.2.41, 8.2.74, 8.2.75, 8.2.73, 8.3.15 … 8.3.59, 8.4.40, 8.4.41, 8.3.13, 8.4.53, 8.4.54, 8.2.38, 8.4.55, 8.4.1, 8.4.2, 8.4.58, 8.4.65, 8.4.56 | after 3.1.68 |

""",
"""| `adesha.rs` | 6.1.97, 6.1.101 (their aṅga–śap entries), 6.1.101 … 6.1.96, 6.4.106, 6.4.107, 6.1.90, 6.1.88 … 6.4.101, 6.4.111 | after 3.1.68 |
| `tripadi.rs` | 8.2.18, 8.2.77, 8.2.78, 8.2.23, 8.2.25, 8.2.26, 8.2.30, 8.2.31, 8.2.39, 8.2.40, 8.2.41, 8.2.74, 8.2.75, 8.2.73, 8.3.15 … 8.3.59, 8.4.40, 8.4.41, 8.3.13, 8.4.53, 8.4.54, 8.2.38, 8.4.55, 8.4.1, 8.4.2, 8.4.58, 8.4.65, 8.4.56 | after 3.1.68 |

"""),
("""what matters, and `tinanta_rule_order_is_pinned` in `derivation_tests.rs`
pins all 162 ids verbatim (72 pre-rudhādi, the twenty-one rudhādi added:
3.1.78, 6.4.23, 6.4.111, 8.2.74, 8.2.75, 8.2.73, 8.3.24, 8.4.53, 8.4.58 and
""",
"""what matters, and `tinanta_rule_order_is_pinned` in `derivation_tests.rs`
pins all 166 ids verbatim (72 pre-rudhādi, the twenty-one rudhādi added:
3.1.78, 6.4.23, 6.4.111, 8.2.74, 8.2.75, 8.2.73, 8.3.24, 8.4.53, 8.4.58 and
"""),
("""2567 at the head of `samjna.rs`; 10j also moved 10.0493 to the head of
`sanadi.rs` and 6.4.92 to after the sanādi 6.1.78 — 162 total).
`tinanta/terms.rs` holds the term-index constants and the reason 3.1.68
""",
"""2567 at the head of `samjna.rs`; 10j also moved 10.0493 to the head of
`sanadi.rs` and 6.4.92 to after the sanādi 6.1.78 — 162 total — then curādi
10l's four: 6.1.75 *dīrghāt* and 7.1.101 *upadhāyāś ca* after the sanādi
6.1.73, 8.2.18 *kṛpo ro laḥ* at the head of `tripadi.rs` and 8.2.78
*upadhāyāṃ ca* after 8.2.77 — 166 total).
`tinanta/terms.rs` holds the term-index constants and the reason 3.1.68
"""),
("""√ki; slice 3a; √bhī, √hrī; slice 3b; √dā, √dhā, √mā, √hā; slice 3c; √hā
parasmaipada, √gā; slice 3c2; √pṝ, √pṛ, √bhṛ, √ghṛ, √hṛ, √sṛ; slice 3d; √ṛ; slice 3d2; √ṇij, √vij, √viṣ; slice 3e; √kit, √tur, √dhiṣ, √dhan; slice 3f; √bhas; slice 3f2; √jan; slice 3f3) — and curādi (10), **open** at 482 of its
509 rows (√cur, √laḍ, √bhakṣ, √bhūṣ; slice 10a; √cit, √vṛṣ, √mad, √kusm; slice 10b; thirty-three more ākusmīya roots, slice 10c; √jñap, √yam, √cah, √cap, √rah, √bal, slice 10d; ninety-two adanta roots, slice 10e; the ten optional-ṇic rows, slice 10f; fifty-nine more, slice 10g; fifty ādhṛṣīya rows, slice 10h; the fifty-nine āsvadīya rows, √pṝ and √ghuṣ, slice 10i; √smiṅ, √ci, √ghṛ, √gṛ, √yu, √jñā, √cyu and √bhū, slice 10j; the 155 plain obligatory-ṇic rows, slice 10k). gaṇa
is carried as a tag on the aṅga term (`Tag::Divadi` / `Tag::Tudadi` / `Tag::Adadi` /
""",
"""√ki; slice 3a; √bhī, √hrī; slice 3b; √dā, √dhā, √mā, √hā; slice 3c; √hā
parasmaipada, √gā; slice 3c2; √pṝ, √pṛ, √bhṛ, √ghṛ, √hṛ, √sṛ; slice 3d; √ṛ; slice 3d2; √ṇij, √vij, √viṣ; slice 3e; √kit, √tur, √dhiṣ, √dhan; slice 3f; √bhas; slice 3f2; √jan; slice 3f3) — and curādi (10), **open** at 490 of its
509 rows (√cur, √laḍ, √bhakṣ, √bhūṣ; slice 10a; √cit, √vṛṣ, √mad, √kusm; slice 10b; thirty-three more ākusmīya roots, slice 10c; √jñap, √yam, √cah, √cap, √rah, √bal, slice 10d; ninety-two adanta roots, slice 10e; the ten optional-ṇic rows, slice 10f; fifty-nine more, slice 10g; fifty ādhṛṣīya rows, slice 10h; the fifty-nine āsvadīya rows, √pṝ and √ghuṣ, slice 10i; √smiṅ, √ci, √ghṛ, √gṛ, √yu, √jñā, √cyu and √bhū, slice 10j; the 155 plain obligatory-ṇic rows, slice 10k; the eight rule-bearing rows, slice 10l). gaṇa
is carried as a tag on the aṅga term (`Tag::Divadi` / `Tag::Tudadi` / `Tag::Adadi` /
"""),
("""7.1.35 optionally replaces the loṭ endings `tu`/`hi` with tātaṅ (then
8.2.39 obligatorily voices its final `t` to `d`), forking 1022 cells (loṭ
prathama and madhyama eka across the 511 roots with a parasmaipada column —
`tu`/`hi` are parasmaipada endings, so the curated set's 74 ātmanepada-only
roots never reach this guard, and the 459 roots that admit both
padas (twenty-five ubhayapadī by 1.3.72 — √rudh, √nī, √tud, √bhid, √kṣud,
""",
"""7.1.35 optionally replaces the loṭ endings `tu`/`hi` with tātaṅ (then
8.2.39 obligatorily voices its final `t` to `d`), forking 1038 cells (loṭ
prathama and madhyama eka across the 519 roots with a parasmaipada column —
`tu`/`hi` are parasmaipada endings, so the curated set's 74 ātmanepada-only
roots never reach this guard, and the 467 roots that admit both
padas (twenty-five ubhayapadī by 1.3.72 — √rudh, √nī, √tud, √bhid, √kṣud,
"""),
("""sixty-one rows, slice 10j's √ghṛ, √jñā, √cyu, √bhū and √ci (by 1.3.72 too,
without ṇic) and slice 10k's 155 rows by 1.3.74, and slice 10f's six optional-ṇic ākusmīya roots and
`garva`, by 1.3.78 on their ṇic-less branch) reach it in their
""",
"""sixty-one rows, slice 10j's √ghṛ, √jñā, √cyu, √bhū and √ci (by 1.3.72 too,
without ṇic), slice 10k's 155 rows and slice 10l's eight by 1.3.74, and slice 10f's six optional-ṇic ākusmīya roots and
`garva`, by 1.3.78 on their ṇic-less branch) reach it in their
"""),
("""(`03.0009`), √gā, √pṝ, √pṛ, √ghṛ (`Gf`), √hṛ, √sṛ, √ṛ, √kit, √tur, √dhiṣ, √dhan, √bhas and √jan (all parasmaipada-only), and
the parasmaipada columns of √bhṛ, √ṇij, √vij and √viṣ; 511 + 74 = the 585 curated roots) — `Bavatu ~
BavatAd`, `Bava ~ BavatAd`. 8.4.56 optionally devoices a pada-final jaś
(produced by the now-obligatory 8.2.39) back to its car at the end of an
utterance, forking 882 cells outright: laṅ and vidhiliṅ prathama eka across
those same 511 parasmaipada columns (859 of them — 8.2.39's `d` is a
parasmaipada-ending artifact, ātmanepada's laṅ/vidhiliṅ prathama eka endings
""",
"""(`03.0009`), √gā, √pṝ, √pṛ, √ghṛ (`Gf`), √hṛ, √sṛ, √ṛ, √kit, √tur, √dhiṣ, √dhan, √bhas and √jan (all parasmaipada-only), and
the parasmaipada columns of √bhṛ, √ṇij, √vij and √viṣ; 519 + 74 = the 593 curated roots) — `Bavatu ~
BavatAd`, `Bava ~ BavatAd`. 8.4.56 optionally devoices a pada-final jaś
(produced by the now-obligatory 8.2.39) back to its car at the end of an
utterance, forking 896 cells outright: laṅ and vidhiliṅ prathama eka across
those same 519 parasmaipada columns (873 of them — 8.2.39's `d` is a
parasmaipada-ending artifact, ātmanepada's laṅ/vidhiliṅ prathama eka endings
"""),
("""prathama eka ends in the aṅga's `r`, turned to visarga, not in a jaś: `abiBaH`,
`EyaH`; 3f's four contribute only their vidhiliṅ cell too: √tur's laṅ ends in visarga, √dhan's in `n`, and √kit's and √dhiṣ's laṅ forks key on 7.3.86 as well, see below; 3f2's √bhas contributes both, its laṅ `abaBad` coming from 8.2.73; 3f3's √jan contributes only its vidhiliṅ cell, its laṅ ending in `n`, and its 6.4.43 branch's `jajAyAd ~ jajAyAt` keys on 6.4.43 as well; 10a's four curādi roots contribute both cells, except √cur, whose two key on its mandatory sanādi 7.3.86 as well, see below; 10d's six jñapādi roots and 10e's eighty-three adanta roots contribute both cells, and so do 10f's `mUtra`, `katra` and `pata`, 10g's rows other than √śṛdh and √div, 10h's other than its sixteen laghu-ik rows and 10i's other than its eleven (whose ṇic branch keys on the sanādi 7.3.86 as well) on their ṇic branch, and 10j's √ghṛ, √jñā, √cyu and √bhū, and √ci on its declined ṇic branch, and 10k's rows other than its thirty-seven laghu-ik rows (whose two key on the sanādi 7.3.86 as well); √ci's 6.1.54 branch keys on 6.1.54 as well (`6.1.54+8.4.56`), and 10f's to 10j's ṇic-less forks key on their optional-ṇic id — `2564+8.4.56`, `10.0498+8.4.56`, `10.0499+8.4.56` and their siblings — as does √dhū's and √prī's nuk branch on 7.3.37.2, and sit outside this count),
plus twenty-two rudhādi laṅ *madhyama* eka cells (√kṛt, √hiṃs, √bhañj, √piṣ,
""",
"""prathama eka ends in the aṅga's `r`, turned to visarga, not in a jaś: `abiBaH`,
`EyaH`; 3f's four contribute only their vidhiliṅ cell too: √tur's laṅ ends in visarga, √dhan's in `n`, and √kit's and √dhiṣ's laṅ forks key on 7.3.86 as well, see below; 3f2's √bhas contributes both, its laṅ `abaBad` coming from 8.2.73; 3f3's √jan contributes only its vidhiliṅ cell, its laṅ ending in `n`, and its 6.4.43 branch's `jajAyAd ~ jajAyAt` keys on 6.4.43 as well; 10a's four curādi roots contribute both cells, except √cur, whose two key on its mandatory sanādi 7.3.86 as well, see below; 10d's six jñapādi roots and 10e's eighty-three adanta roots contribute both cells, and so do 10f's `mUtra`, `katra` and `pata`, 10g's rows other than √śṛdh and √div, 10h's other than its sixteen laghu-ik rows and 10i's other than its eleven (whose ṇic branch keys on the sanādi 7.3.86 as well) on their ṇic branch, and 10j's √ghṛ, √jñā, √cyu and √bhū, and √ci on its declined ṇic branch, and 10k's rows other than its thirty-seven laghu-ik rows (whose two key on the sanādi 7.3.86 as well), and 10l's other than √kṛp (likewise), √dhras on its ṇic branch; √ci's 6.1.54 branch keys on 6.1.54 as well (`6.1.54+8.4.56`), and 10f's to 10l's ṇic-less forks key on their optional-ṇic id — `2564+8.4.56`, `10.0498+8.4.56`, `10.0499+8.4.56` and their siblings — as does √dhū's and √prī's nuk branch on 7.3.37.2, and sit outside this count),
plus twenty-two rudhādi laṅ *madhyama* eka cells (√kṛt, √hiṃs, √bhañj, √piṣ,
"""),
("""`7.3.86+8.4.56` bucket beside tanādi's and not in this count. 8.4.56 goes on
forking a further 1022 (the same
loṭ cells 7.1.35 just forked) by devoicing the tātaṅ branch's `BavatAd` to
""",
"""`7.3.86+8.4.56` bucket beside tanādi's and not in this count. 8.4.56 goes on
forking a further 1038 (the same
loṭ cells 7.1.35 just forked) by devoicing the tātaṅ branch's `BavatAd` to
"""),
],
'docs/superpowers/specs/2026-10-05-curadi-gana-10k-design.md': [
("""- **10l: the nine rule-bearing rows.** The prototype found exactly what each
  needs. Re-prototype off 10k's HEAD before planning.

""",
"""- **10l: the nine rule-bearing rows.** The prototype found exactly what each
  needs. Re-prototype off 10k's HEAD before planning. (Done:
  `2026-10-05-curadi-gana-10l-design.md` takes eight of them; `picca~` and
  the 8.2.30 narrowing went to 10m. Its re-prototype corrected this table:
  `kFta~` takes 7.1.101 *upadhāyāś ca*, not 7.1.100; `adwa~` also takes
  8.4.55; and `u~Drasa~` needs the marker reader to see an initial `u~`.)

"""),
],
'tools/audit/README.md': [
("""
**It asserts the corpus totals** (585 roots, 37584 cells, 49160 forms) rather than
reporting whatever it enumerated. Those totals are corroborated by
""",
"""
**It asserts the corpus totals** (593 roots, 38160 cells, 49826 forms) rather than
reporting whatever it enumerated. Those totals are corroborated by
"""),
("""## Last recorded result

""",
"""## Last recorded result

2026-10-05, curādi 10l slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 38160
cells / 49826 forms / 593 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole curādi 10l slice: eight rows that bring rules of
their own, all ubhayapadī by 1.3.74. 8.2.78 *upadhāyāṃ ca* lengthens √ūrj's,
√cūrṇ's, √gūrd's and √kṝt's ik before the `r` upadhā; 7.1.101 *upadhāyāś
ca* gives √kṝt its `ir` ahead of guṇa; 8.2.18 *kṛpo ro laḥ* gives √kṛp its
`l`, keyed by row; 6.1.75 *dīrghāt* gives √mlecch its tuk; 8.4.41's
stu-before-ṭu arm, then 8.4.55, gives √aṭṭ its `ww`; and √dhras's ṇic is
optional by Kaumudī 2570, read off its initial `u~`. Blocked branches rose
from 6516 to 6552, √dhras's 36 ṇic-less ātmanepada cells. On the throwaway
prototype that scoped the slice, switching each rule off in turn made the
audit fail: 72 cells for each single-row rule and 270 for 8.2.78. A
main-vs-branch dump of every prior cell's branches, blocked ones included,
was byte-identical, all 55676 of them (49160 live).

Totals: 593 = 585 + 8; 38160 = 37584 + 576 (64 root×pada×lakāra blocks ×
9); 49826 = 49160 + 576 + 90 new `ALTERNATES` rows (11576 → 11666),
measured via the harness's corpus block, not assumed.

"""),
],
}
import os
D = os.environ['AUDIT_DATE']
assert len(D) == 10 and D[4] == '-' and D[7] == '-', D
for p, edits in E.items():
    E[p] = [(o, n.replace('2026-10-05, curādi 10l slice', f'{D}, curādi 10l slice')
                 .replace("2026-10-05 10l entry", f"{D} 10l entry")) for o, n in edits]
bad, out = [], {}
for p, edits in E.items():
    s = open(p).read()
    for old, new in edits:
        n = s.count(old)
        if n != 1:
            bad.append(f"{p}: {n}x {old[:70]!r}")
            continue
        s = s.replace(old, new)
    out[p] = s
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
for p, s in out.items():
    open(p, 'w').write(s)
print(f"applied {sum(len(e) for e in E.values())} edits to {len(E)} files")
```

```bash
AUDIT_DATE=<Step 2's date> python3 /tmp/vidyut-full/slice10l/docsweep_10l.py      # applied 49 edits to 9 files
mise run fmt
```

- [ ] **Step 4: Sweep greps**

The script covers what the prototype's greps found. Re-run the greps on this tree, and fix any hit that is not historical (a past slice's record, or 10k's own paragraphs):

```bash
grep -rni -E "four hundred eighty|five hundred eighty|four hundred nineteen|one hundred eighty-one|four hundred fifty-nine" README.md AGENTS.md docs/ARCHITECTURE.md tools crates --include=*.rs --include=*.md | grep -v "paradigm/data"
grep -rn -E "\b(482|585|419|181|6516|1022|459|162|37584|49160|11576|1603)\b" README.md AGENTS.md docs/ARCHITECTURE.md crates/*/src crates/panini/tests/*.rs crates/panini/tests/trace crates/panini/tests/paradigm/main.rs tools/audit/panini_full_audit.rs | grep -v '"10\.0\|"0[0-9]\.0'
grep -rn -E "ñit or udit|last marker|trigger-then-target|one direction|fires only on √vich\b" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.rs --include=*.md | grep -v "paradigm/data"
```

Expected (the prototype's output at this point): no spelled-out hits. The numeral grep prints only historical lines and Task 5's: ARCHITECTURE's "162 total" in the pin history; AGENTS.md's floor paragraph (lines with 37584, 459.65s twice, and the 155-row sentence), its progress line (482, in 10k's clause) and its 10k audit-chain line; `paradigm/main.rs`'s 10k audit-chain line, its 10k census paragraph (482), and the pada-ambiguous comment's 1603 (10k's line and 10l's). The phrasing grep prints only lines that stay true: `sanadi.rs`'s module doc ("6.1.73 fires only on √vich and √pich"), its 2570 comment ("a ñit or udit curādi root"), and `panini-data`'s two 2570 docs, which now name the initial `u~`.

- [ ] **Step 5: Run the full suite and commit**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
git branch --show-current      # curadi-10l
git add -A
git commit -m "docs: 10l's counts, the audit record, and the sweep

38160 cells / 49826 forms / 593 roots across README, ARCHITECTURE, AGENTS,
paradigm/main.rs and tools/audit; curādi open at 490/509; 467 both-pada
roots; 1631 pada-ambiguous surfaces; 166 pinned rule ids. Audit at zero
divergence against 8da2f90b with 6552 blocked branches; prior traces
byte-identical to main (55676 branches, 49160 live). The 10k spec points
at 10l and records its three corrections."
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

What to expect (the prototype measured the lists):
- **The mutant list grows by 43.** `panini-prakriya` lists 906 mutants (863 on main); `panini-analyze` 12 and `panini-data` 12, both unchanged. By name (spans ignored) eight leave: `che_ca`'s scan mutants (`&&`, `==`, two `-` → `+`, two `-` → `/`, `+` → `-`, `+` → `*`), which move into `tuk_before_che` under that name. Fifty-one arrive: those eight under `tuk_before_che` and its two return-value mutants; `dirghat`'s four; 7.1.101's two (`sanadi.rs`: `delete !`, `!=` → `==`); `shtutva_of`'s eight (two return values, six match arms); 8.2.18's two (`delete !`, `==` → `!=`); 8.2.78's twenty (two `||` → `&&`, three on `<`, `delete !`, four match arms, ten on `-`/`+`); 8.4.41's arm's five (`&&` → `||`, four on `-`).
- **`--in-diff` over the slice's production diff for `panini-prakriya`** lists those 51. The prototype ran all 51 (above): 48 CAUGHT once the five-letter witness was in, 3 UNVIABLE (`tripadi.rs:222:17` `+` → `-` and `+` → `*`, `tripadi.rs:1284:21` `&&` → `||`). Test phases under load 17–79: 0.4–171 s for most, and 567–604 s for three of `shtutva_of`'s match-arm deletions (`sound.rs:330`, `331`, `333`), which only `shtutva_of_all_arms`, in the last binary run, can see. All far under the 9240 cap; record their campaign-load phases in Step 5 all the same.
- **`--in-diff` over the data crate's diff** lists none ("No mutants to filter", measured on the prototype): the slice's changes there are rows, constants, docs and test code.
- **The documented non-caught entries move.** The equivalents are at `adesha.rs:649:30` (unmoved) and `tripadi.rs:1399:38` (8.3.13's `apply`, from 1305: 8.2.18 and 8.2.78 land above it), the permanent hang at `tripadi.rs:1712:23` (from 1618). The `skip_nic` pair moves to `sanadi.rs:60:5` and `60:39` (from 57: the import and the module doc). **A new `tripadi.rs:217:38: replace - with /` (8.2.78's `c[n - 2]`) also matches the old probe regex `tripadi.rs:[0-9]+:38: replace - with /`, and it is CAUGHT; anchor the probe on the line `--list` gives inside 8.3.13.** Confirm every span by `--list`; never compute them.
- **The cap will probably hold at 9240.** The slice adds no optional-ṇic id, so `skip_nic -> true` still forks 2^8 ways; the corpus grows 1.5% (38160 cells against 37584). Re-measure it alone anyway, as AGENTS.md requires on corpus growth.

- [ ] **Step 1: Measure the floor**

With nothing else of ours running, run this twice: `cat /proc/loadavg; time mise run test >/dev/null 2>&1; cat /proc/loadavg` (foreground, timeout 600000 ms). Record both wall clocks, user CPU and the load averages. Read 10k's floor from AGENTS.md's floor paragraph and keep the comparison chain.

- [ ] **Step 2: Locate and probe the uncaught equivalents and the `skip_nic` pair**

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
PROBE=/home/dev/mutants-records/curadi-10l-probe   # durable
mkdir -p "$PROBE"
mise exec -- "$CM" mutants --package panini-prakriya --list -o "$PROBE/list" 2>/dev/null | wc -l      # 906
mise exec -- "$CM" mutants --package panini-prakriya --list -o "$PROBE/list" 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /=|skip_nic -> bool with true|in skip_nic"
```

Expect `adesha.rs:649:30`, `tripadi.rs:217:38` (8.2.78's, caught; not the equivalent), `tripadi.rs:1399:38` (the equivalent, inside 8.3.13's `apply`), `tripadi.rs:1712:23` (the ṇatva hang), and `sanadi.rs:60:5` (`skip_nic -> bool with true`) and `60:39` (`!=` → `==` in `skip_nic`). Write them as `<A>` (649), `<T1>` (1399), `<T2>` (1712), `<K>` and `<K2>` (60). The probe runs past the 60-minute shell limit, so launch it detached and wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`:

```bash
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 20000 -j 4 -o "$PROBE" \
  --re "adesha.rs:<A>:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" \
  --re "sanadi.rs:<K>:5: replace skip_nic -> bool with true" --re "sanadi.rs:<K2>:39: replace != with == in skip_nic" \
  > "$PROBE/probe.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$PROBE/started"; cat /proc/loadavg > "$PROBE/load.started"
```

The 20000s probe cap is a ceiling for measurement, not the campaign's cap. The regexes also match caught `mod.rs` `derive` mutants; that is expected. When it ends, copy `$PROBE/mutants.out/outcomes.json` to `$PROBE/probe-outcomes.durable.json` and record the end time and load. Both equivalents must be MISSED, not TIMEOUT; both `skip_nic` mutants must be CAUGHT. Read each test-phase duration from the outcomes. Set the provisional cap to max(9240, 6 × the longer equivalent, 2 × the longer `skip_nic` phase), rounded up to the next 10 s.

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/curadi-10l"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout <CAP> -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"; cat /proc/loadavg > "$OUT/load.started"
```

`<CAP>` is Step 2's provisional cap. Run nothing CPU-heavy meanwhile. 10k's campaign took 6h57m at 9240; expect about the same. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

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
- **918 mutants: 863 caught, 52 unviable, 2 missed, 1 timeout.**
  - panini-prakriya: 906 / 855 / 48 / 2 / 1.
  - panini-analyze: 12 / 8 / 4 / 0 / 0.
- `missed.txt` holds exactly `adesha.rs:<A>:30: replace + with *` and `tripadi.rs:<T1>:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:<T2>:23: replace -= with /=`.

Then diff the non-caught set against 10k's on the full record, both with and without span lines:

```bash
python3 - "$OUT/outcomes.durable.json" /home/dev/mutants-records/curadi-10k/outcomes.durable.json <<'PY'
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

Expected:
- **Without lines:** `28 25`, `new:` exactly the three new unviables (`tripadi.rs` col 17 `+` → `-` and `+` → `*` in 8.2.78's `apply`, col 21 `&&` → `||` in 8.4.41's), `gone: []`.
- **With lines:** `55 52`; `new:` those three plus three moved entries, and `gone:` the same three moved entries at their 10k spans — the `tripadi.rs` equivalent (1305 → 1399, 8.2.18 and 8.2.78 above it), the hang (1618 → 1712), and `sanadi_pratyaya`'s unviable (`sanadi.rs` 76 → 79, the module doc and the import). The prototype mapped every other entry of 10k's non-caught set to an unmoved span in this branch's `--list`. Write down exactly what prints, and name each moved entry in the record.

If not:
- Any **other timeout** is a suspect survivor that the larger suite pushed past the cap. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant among the 51 new ones is a gap in Task 2's tests; add the test that kills it. Any other missed mutant means a test that caught it at 37584 cells no longer does; stop and report.

**Step 4b: the data-crate mutants.** Confirm the slice's diff for `panini-data` holds none:

```bash
git diff 056e7f5 -- crates/panini-data/src/lib.rs > "$OUT/data.diff"
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

The cap is max(430, 6 × the longest campaign-load equivalent phase, 2 × the longest campaign-load caught phase), rounded up to the next 10 s, over both readings of the `skip_nic` `true` mutant (the probe's and the campaign's); it is not lowered on the quieter one.
- If that is 9240 or less, the cap stays 9240.
- Otherwise change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together (`grep -n "9240" AGENTS.md mise.toml`).

- [ ] **Step 6: Record it in AGENTS.md**

- **The floor paragraph.** Rewrite the paragraph that opens `**The floor behind the 9240s cap, measured at 37584 cells on Rust 1.99.0,`. Use Step 1's and Step 2's numbers at 38160 cells, the load averages, and the cap Step 5 chose. Keep the comparison chain to earlier floors, with 10k's joining it: 9m31.852s / 6m9.420s at 37584 cells under load 63–122 (user 12m50.9s / 12m27.1s), isolated probe 450.11s / 459.65s, campaign-load 411.94s / 260.17s, `skip_nic` `true` 4617.70s in the probe and 2846.36s in the campaign, `!=` → `==` 3695.74s and 2191.20s.
- **The current record.** Replace the `**Current record (curādi 10k, …).**` paragraph with `**Current record (curādi 10l, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**;
  - the non-caught set diffed against 10k's on the full record, both ways (Step 4's script output), naming what moved and why;
  - the 51 new mutants by rule, 48 CAUGHT and 3 UNVIABLE (named), and the eight that moved into `tuk_before_che`;
  - the `skip_nic` pair's probe and campaign phases;
  - Step 4b's empty data-crate list;
  - the campaign-load phases and margins;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`, and the probe's at `/home/dev/mutants-records/curadi-10l-probe/probe-outcomes.durable.json`.

  End it with a pointer to the record it replaces. Run `git rev-parse --short HEAD` before committing, and write ``The curādi 10k record it replaces: `git show <that hash>:AGENTS.md`.``

- [ ] **Step 7: Commit**

```bash
git branch --show-current      # curadi-10l
git add AGENTS.md mise.toml
git commit -m "chore: 10l mutation gate — floor and uncaught run re-measured at 38160 cells

Every viable new mutant caught; missed.txt holds only the two documented
equivalents and timeout.txt only the permanent ṇatva-scan entry, at
their moved tripadi.rs spans."
```

Adjust the message to what Step 4 and Step 5 actually found (the cap, any difference).

---

## Task 6: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # back at /workspace/crates
git branch --show-current      # curadi-10l
git log --oneline main..HEAD   # the spec commit, the plan and the four task commits
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin curadi-10l
gh pr create --title "curādi 10l — eight rule-bearing rows: 6.1.75, 7.1.101, 8.2.18, 8.2.78, 8.4.41's second arm" --body "$(cat <<'BODY'
Slice 10l curates eight curādi rows that bring rules of their own: √ūrj
(`10.0023`), √cūrṇ (`10.0026`), √aṭṭ (`10.0037`), √kṝt (`10.0155`), √mlecch
(`10.0170`), √gūrd (`10.0180`), √dhras (`10.0270`) and √kṛp (`10.0278`).

- 8.2.78 *upadhāyāṃ ca* lengthens an ik before an `r` upadhā and a final
  hal (*ūrjayati*, *cūrṇayati*, *gūrdayati*), reading the root inside a
  ṇijanta aṅga through ṇic's `ay`.
- 7.1.101 *upadhāyāś ca* makes √kṝt's `F` upadhā `ir` before ṇic, ahead of
  7.3.86, so the `i` is guru and takes no guṇa; 8.2.78 then gives
  *kīrtayati*.
- 8.2.18 *kṛpo ro laḥ* gives √kṛp its `l` (*kalpayati*), keyed by row
  (`samjna::KRP`): `10.0408 kfpa` is another root.
- 6.1.75 *dīrghāt* adds tuk after √mlecch's long `e` (*mlecchayati*), in the
  same scan as the sanādi 6.1.73.
- 8.4.41 *ṣṭunā ṣṭuḥ* now also retroflexes a stu before a ṭu
  (*aṭṭayati*), through a full `shtutva_of` table.
- √dhras is udit by its initial `u~`, so its ṇic is optional by Kaumudī
  2570 (*dhrāsayati* beside *dhrasati*).

The golden suite goes from 37584 to 38160 cells; curādi is open at 490 of
509. The audit shows zero divergence against `8da2f90b`, a main-vs-branch
dump of every prior branch (blocked ones included) is byte-identical, and
the mutation gate finds every viable new mutant caught. `10.0175 picca~` and the
8.2.30 narrowing are slice 10m's.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`:
   - run `git worktree remove .worktrees/curadi-10l`;
   - run `git worktree remove --force .worktrees/curadi-10l-proto` and `git branch -D proto-10l proto-10l-v2 proto-10l-t3combined` (the throwaway);
   - run `git worktree remove --force .worktrees/curadi-10l-replay` if the replay worktree is still there (detached; no branch);
   - delete the local and remote `curadi-10l` branch;
   - run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| 8 rows, codes, `Nic`; `10.0026`'s trailing space; `OPTIONAL_NIC` + √dhras by 2570 | 3 Step 3 |
| The initial-`u~` reading in `optional_nic_from_upadesha` | 3 Step 1 |
| 7.1.101 in the sanādi stage before 7.3.86, `ir` uncredited 1.1.51, no guṇa-stage entry | 2 |
| 6.1.75 in the sanādi stage, sharing 6.1.73's scan; no aṅga-stage entry | 2 |
| 8.2.18 first in the tripādī, keyed by `KRP`, pinned to upstream | 2; 3 Step 1 |
| 8.2.78 after 8.2.77, reading the root through `ay`; 8.2.79 needs no carve-out | 2 |
| 8.4.41's stu-before-ṭu arm, `shtutva_of` and its arms test, `z` excluded, the CORRESPONDENCE paragraph | 2 |
| The homograph `10.0026` / `10.0143`, both comments | 3 Steps 1, 3 |
| Goldens via the generator; census 593 / 38160 / 49826; blocked 6552; 90 `ALTERNATES` and keys | 3 Steps 1, 4–7; 4 Step 2 |
| Other counts: row count 593, 1.3.74 427, `OPTIONAL_NIC` 182, pada-ambiguous re-derived (1631) | 3 Steps 1, 6 |
| Unit tests per rule or arm, each with a near-miss | 2 Step 1 |
| `tinanta_rule_order_is_pinned` (166); vikalpa pin unchanged | 2 Step 1 |
| Rosters: 1.3.74, 2570, 8.4.55 (533 = 455 + 78), 8.4.40 (+`10.0170`) | 3 Step 1 |
| `the_10l_rules_fire_only_on_their_rows` | 3 Step 1 |
| `curadi_analyses_its_10l_forms` from the spec's table, `cUrRayati` two analyses | 3 Steps 1, 7 |
| Prior-trace diff main ↔ HEAD, all 55676 | 4 Step 2 |
| Audit (negative control, then 593 / 38160 / 49826, 6552 blocked) | 4 Steps 1–2 |
| Mutation gate: every viable new mutant caught, floor and `skip_nic` re-measured, cap by rule, `-o`, durable copy, non-caught named verbatim | 5 |
| Doc sweep: counts, curādi 490, rule inventories, ARCHITECTURE census, audit record, sanādi doc, 8.4.41's comments, 10k spec pointer, greps | 2 Step 3; 4 Steps 3–4 |
| Later slices (10m) | no task |

**Placeholder scan.** `<A>`, `<T1>`, `<T2>`, `<K>`, `<K2>`, `<CAP>`, `<DATE>` and the audit date are values the executor measures, each with the command that produces it and its expected value where one is known. No step says "update the tests" without the edit.

**Type consistency.**
- `KRP: [&str; 1]`, read as `KRP.contains(&p.ctx.dhatupatha)` in `tripadi.rs` (`use crate::tinanta::samjna::KRP`) and compared with an array literal in its pin.
- `shtutva_of(char) -> Option<char>` is read by 8.4.41's `if matches!(…) && let Some(sub) = shtutva_of(…)` (edition 2024 let-chains, as the crate already uses) and pinned arm by arm.
- `tuk_before_che(p, vowel: fn(char) -> bool, sutra: &str, name: &str)`: `che_ca` passes `is_hrasva`, `dirghat` a non-capturing closure, which coerces to `fn(char) -> bool`; `Prakriya::record` takes `&str`.
- `curadi_row` keeps `ajanta_row`'s signature; its four 10j callers are renamed with it.
- `curadi_analyses_its_10l_forms`' tuples are `(&str, &[Reading])` with `Reading<'a> = (&'a str, &'a [&'a str], &'a [&'a str])`; the first witness's `[..]` fixes the slice types.
- The golden tuple shapes match `ParadigmRow` and `AlternateRow`; the generator is 10j's with the row selection changed.

**Where this plan goes beyond the spec's letter**, each to make the spec's decisions work:
- **`ajanta_row` is renamed `curadi_row`**, since 10l's hand-built rows are not ajanta.
- **The end-to-end `the_10l_rules_give_vidyuts_forms`** derives each row before it is curated, so Task 2 is tested on whole words, not only on hand-built prakriyās.
- **The pada-ambiguous set (1603 → 1631)** moves, which the spec did not name; Task 3 Step 6 pins it from measurement.
- **8.4.55's roster test** keeps its 455 and pins `10.0037`'s 78 separately, so a new prior-row firing still shows.
- **8.4.40's comment** said 8.4.41 runs "in the trigger-then-target direction only"; it is corrected with 8.4.41's own. ARCHITECTURE's stale "6.1.73 … fires only on √vich" (stale since 10k) is corrected where the sanādi row changes.

**Known soft spots.**
- **Doc strings in Task 4** were read at the prototype's state. If the script reports an `old` not found exactly once, edit that paragraph to the same facts rather than skip it.
- **Task 5's campaign numbers are expectations**, not prototype measurements; the in-diff run is the one measured part. The floor and cap depend on host load; record the load beside every timing. The Step 4 span diff is the least certain line in the plan: write down what prints.
