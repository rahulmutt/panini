# Svādi gaṇa slice 5b Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate svādi's other thirty-two dhātupāṭha rows and close the gaṇa at 38 of 38. Thirty derive on data alone; √dambh (`05.0026 danBu~`) needs 6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti*, landed as the general sūtra, and √tṛp (`05.0028 tfpa~`) needs 8.4.39 *kṣubhnādiṣu ca*, keyed by row. The golden suite goes from 38232 to 39744 cells.

**Architecture:** Six tasks:
- **Task 1 is the gate.** It applies the rows and then the production code to the worktree, and proves three things before anything is committed: the rows alone make the vidyut audit fail on exactly √dambh's and √tṛp's 72 cells; with the engine, a dump of every prior branch (blocked ones included, every step's before and after text) is byte-identical to the base's; and the audit over all 626 roots shows zero differences. If either check fails, the slice stops. Task 1 then reverts everything; nothing is committed.
- **Task 2** is the engine, test-first and green on its own (no rows): `IDIT` and `Tag::Idit` with its `derive` wiring, the general 6.4.24 after 6.4.23, 8.4.39 before 8.4.1 with `KSUBHNADI`, their guard tests, hand-built √dambh and √tṛp derivations, `IDIT`'s two-way data test, and the order and bars pins.
- **Task 3** lands the thirty-two rows, 168 golden rows and 308 alternates, and every count, list, trace pin, roster and `check()` assertion they move. The assertions go in first and fail; the rows and goldens make them pass.
- **Task 4** is the audit, the final base-vs-branch trace dump, and the doc sweep.
- **Task 5** is the mutation gate.
- **Task 6** is the final review and the branch finish.

**Tech Stack:** Rust 1.99.0, pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, used through a private copy of `/tmp/vidyut-full` at `/home/dev/vidyut-svadi-5b`.

**Spec:** `docs/superpowers/specs/2026-10-07-svadi-gana-5b-design.md`. Its Decisions section is why 6.4.24 is general and reads the next non-empty term's ṅit and an idit verdict, why that verdict is data (`IDIT`), why the idit guard has a hand-built witness only, and why 8.4.39 is keyed by row. Its Evidence section is the prototype this plan was measured against. Where this plan departs from the spec's letter, the Self-Review says so.

**Workspace:** The spec and this plan are on the branch `svadi-5b`, checked out at `/workspace/.worktrees/svadi-5b`. Every path below is relative to that directory unless it starts with `/`. Three paths outside it are this slice's own and nobody else's, because kryādi 9c and hetumaṇic run in parallel:
- `/home/dev/vidyut-svadi-5b` — a private copy of `/tmp/vidyut-full`. Its `vidyut-prakriya/Cargo.toml` is repointed freely; `/tmp/vidyut-full` is never touched.
- `/home/dev/vidyut-svadi-5b/slice5b/` — this plan's throwaway scripts.
- `/home/dev/svadi-5b-base` — a `git archive` of the branch before any code change, the base side of every trace dump. `/workspace` may move under the parallel slices, so the dump never builds from it.

**Provenance.** Everything below was measured on a throwaway replay of this plan, off the spec commit `7b8c338`, in a session scratchpad (not a git worktree; nothing was committed):
- **Gate.** With the general 6.4.24, `IDIT` and 8.4.39 on top of the thirty-two rows, the dump of every prior branch was byte-identical to the base's: 56456 lines, 49904 live, sha256 `ce7780cf9012e63d903924f24f597246fba396f39f88547819888fd929edb3ef` (the same bytes as the prototype's `trace-main.txt`). The audit over 626 roots / 39744 cells / 51724 forms showed zero differences, with 6552 blocked branches; the `entry` control failed on 36 cells. The prototype's rows-only audit failed on 72 cells, 36 each on `05.0026` and `05.0028`.
- **Task 2.** `tests_5b.py` on the base fails to compile (`IDIT`, `Tag::Idit`, `KSUBHNADI`); with `engine_5b.py`, `panini-prakriya` passes 455 and `panini-data` 30; clippy `-D warnings` and `fmt-check` pass.
- **Task 3.** With `assertions_5b.py` on Task 2's tree, the suite fails exactly the eleven tests Task 3 Step 2 lists. The generator found all 1512 new cells equal to vidyut's (`1512 cells, 1820 forms, 0 differences`), and its two outputs hash as Task 3 Step 4 says. After the rows, goldens and the measured pada-ambiguous set, the full suite passes (`panini-prakriya` 455, `trace` 222, `paradigm` 32, `panini-data` 31).
- **Task 4.** `audit_5b.py` and `docsweep_5b.py` applied cleanly (6 and 49 edits). On that final tree the full suite, clippy `-D warnings` and `fmt-check` passed (4m34s wall, load 33–111), the dump of every prior branch was again byte-identical to the base's, the audit passed at 626 / 39744 / 51724 with the `entry` control failing on 36 cells, and the sweep greps found only the lines Task 4 Step 5 lists.
- **Replay order.** The final replay ran every script in this plan's order on a fresh copy of `7b8c338`: `tests_5b.py`, `engine_5b.py`, `assertions_5b.py`, `rows_5b.py`, the generator, `insert_goldens_5b.py`, `check_goldens_5b.py`, `pin_ambiguous_5b.py`, `audit_5b.py`, `docsweep_5b.py`, with `cargo fmt` after each, as the steps say.
- **Mutants.** `--list` for `panini-prakriya` grows from 910 to 917: by name, none leave and seven arrive (four in 6.4.24, three in 8.4.39). `--in-diff` over the slice's production diff lists nine, the seven plus `derive`'s two whole-function mutants; over `panini-data` it lists none.

The replay did **not** run the mutation campaign or the `skip_nic` probe. Task 5's campaign numbers are expectations derived from the measured lists, and the campaign measures them.

**Throwaway scripts.** Everything under `/home/dev/vidyut-svadi-5b/slice5b/` and the two vidyut examples (`trace_dump_5b.rs`, `svadi_goldens_5b.rs`) never ship. Each is reproduced in full below with its sha256. Create a file only if it is missing, and check its hash either way (`sha256sum <file>`); a mismatch means the file is not this plan's, so stop and report. The scripts read nothing but the worktree, except `insert_goldens_5b.py` and `check_goldens_5b.py` (the generator's output, via `V5B`), `pin_ambiguous_5b.py` (the failing test's output, via `SET`) and `docsweep_5b.py` (the audit date, via `AUDIT_DATE`).

## Global Constraints

- **The engine change is exactly this:** `IDIT` in `panini-data`; `Tag::Idit` and its `derive` wiring; 6.4.24 in `ANGA_RULES` right after 6.4.23; `KSUBHNADI` in `samjna.rs` and 8.4.39 in `TRIPADI` right before 8.4.1, barring `["8.4.1", "8.4.2"]`; and comments (6.4.23's, `Rule.bars`'s). No other rule is added, removed, reordered or made optional. `tinanta_rule_order_is_pinned` goes from 166 to 168 ids; `exactly_the_pinned_vikalpa_rules_are_optional` is unchanged.
- **6.4.24 is general, never svādi-gated:** it declines on `Tag::Idit`, reads the next non-empty term after `ANGA` for `Tag::Ngit`, and needs `ANGA` to end in a hal with `n` as its penultimate sound. Do not copy the prototype's `Tag::Svadi` guard.
- **Rows:** exactly the spec's thirty-two, in `rows_5b.py`'s order, inserted after `05.0021 stiG`: ten `Ubhayapada` (`05.0001`–`05.0010`) and twenty-two `Parasmaipada`. `OPTIONAL_NIC` (182), `AYA`, `CISPHUR` and `KRP` do not change. `IDIT` lists 87 rows, none svādi.
- **Name:** this slice is **svādi 5b** in code and docs, never bare "slice 5b": the optional-rules slice (6.4.107) and adādi's vidhiliṅ slice were both "slice 5b" before it.
- **Counts after the slice:** 626 roots; 39744 cells (4416 blocks); 51724 forms; `ALTERNATES` 11980; one / two / three-form cells 31036 / 6970 / 991 (fours, fives, sixes, sevens, nines unchanged: 361, 10, 367, 1, 8); keys `8.4.56` 962, `7.1.35` 954, `7.1.35+8.4.56` 954, `6.4.107` 188, every other key unchanged; both-pada roots 478 (thirty-five by 1.3.72); parasmaipada columns 552; pada-ambiguous pinned surfaces 1655; blocked branches 6552.
- **Pre-existing cells must stay byte-identical, traces included,** with no exception. Regenerate no prior golden.
- **Goldens come from the generator, which asserts engine = vidyut cell by cell, and their sha256 must match this plan's.** **Do not edit a golden to match the engine.** If the generator reports a difference, or a hash differs, stop and report.
- Commit after every task that changes the tree (Task 1 commits nothing). Run `mise run fmt` and `mise run lint` before each commit. Run `git branch --show-current` before every commit and before the push: it must print `svadi-5b` (a detached HEAD strands commits).
- **Every suite run is in the FOREGROUND, with the Bash tool timeout at 600000 ms. Never background it, never end the turn while it runs.** The suite takes 6–8 minutes on a quiet host; under the load this host often carries, the `trace` binary alone ran 309–713 s in the replay. If a run will outgrow one 600000 ms call, start it detached with its output in a log and an exit sentinel, and wait in the same turn on its PID:
  ```bash
  LOG="$(mktemp)"; setsid nohup bash -c 'mise run test > "$0" 2>&1; echo "EXIT_CODE=$?" >> "$0"' "$LOG" < /dev/null & PID=$!; echo "$PID $LOG"
  while kill -0 "$PID" 2>/dev/null; do sleep 30; done; grep -E "FAILED|test result|EXIT_CODE" "$LOG" | grep -v " 0 passed"
  ```
  Re-issue the `while kill -0` line (timeout 600000 ms) until it returns. Never use `pgrep -f "cargo test"`: it matches its own `bash -c` wrapper and loops forever. Never pipe a live suite through `tail`.
- `mise run test -- -p X` does not scope. Scope with `mise exec -- cargo test -p <crate> <filter>`. To see every failing binary at once, use `mise exec -- cargo test --workspace --no-fail-fast` (plain `cargo test` stops at the first failing binary).
- **The private vidyut copy's dev-deps must point at the tree being measured.** `bash /home/dev/vidyut-svadi-5b/slice5b/repoint_5b.sh <tree>` sets them and prints them; read the printed paths every time before a build. The audit harness is always the committed `tools/audit/panini_full_audit.rs`, copied into the private copy's `examples/`; never rewrite it.
- The `cargo-mutants` mise shim fails in background shells. Use the real binary, `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`, under `mise exec --` or after `eval "$(mise env -s bash)"`.
- **Mutation campaigns never run concurrently on this host.** Svādi 5b's is first in the serial queue (svādi 5b, then kryādi 9c, then hetumaṇic). Before launching the probe or the campaign, `pgrep -x cargo-mutants` must print nothing.
- Review packages and diffs exclude the appended goldens (`crates/panini/tests/paradigm/data/svadi.rs`) and use `git diff --diff-algorithm=histogram`: the default algorithm shows appended golden rows as fake deletions.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **A prior branch reached by the general 6.4.24.** Any adādi or juhotyādi aṅga with a nasal upadhā before a luk'd or ślu'd śap, or a kryādi, divādi or tudādi one before its ṅit vikaraṇa, would now lose its nasal, and goldens ignore traces. → Task 1's gate dump and Task 4's final dump (every prior branch, every step); Task 3 `anidit_nalopa_and_ksubhnadi_fire_only_on_their_rows` (6.4.24 credited on `05.0026`'s 42 live branches and nowhere else).
2. **The idit guard.** No curated idit root meets a ṅit affix with a nasal upadhā, so deleting the `Tag::Idit` test changes no golden. → Task 2 `anidit_upadha_nalopa_spares_an_idit_root`, hand-built; `derive_tags_idit_on_the_rows_idit_lists` for the wiring; `idit_matches_upadesha_markers` for the table, both directions.
3. **Which affix decides *kṅiti*.** Reading `ENDING` (or any later term) instead of the next non-empty one, or failing to skip an empty `SHAP`, passes every golden. → Task 2 `anidit_upadha_nalopa_reads_only_the_next_affixs_ngit` (a pit śap before a ṅit ending declines) and the empty-`SHAP` case in `anidit_upadha_nalopa_elides_the_nasal_before_a_ngit_affix`.
4. **8.4.39 on the other `tfpa~` rows.** Curādi `10.0351` and `10.0355` store the same `tfp`; a text-keyed guard would bar their ṇatva-free forms harmlessly and pass, but would bar a real ṇatva on any future `tfp` + `n`. → Task 2 `ksubhnadi_records_on_its_row_before_snu_only` (`10.0351`, `10.0355`, `""` decline; `05.0028` without śnu declines); Task 3's fires-only test.
5. **Homograph answer order.** The spec expected no collision, but every form of `05.0033 kzi` is one of tanādi `08.0004 kziR`'s, and each ñit row's loṭ ātmanepada prathama eka is its parasmaipada prathama dvi. → Task 3 `svadi_analyses_its_5b_forms` asserts both readings of *kziRoti* and *sunutAm*, in analysis order; Task 3 Step 7's whole-corpus collision check.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-data/src/lib.rs` | Task 2: `IDIT`, `is_idit` (shared by three test readers), `idit_matches_upadesha_markers`. Task 3: the thirty-two rows, `Dhatu::pada`'s and the marker doc's counts, the row count (626), `svadi_rows_are_the_thirty_eight_curated_roots` |
| `crates/panini-prakriya/src/term.rs` | Task 2: `Tag::Idit` |
| `crates/panini-prakriya/src/tinanta/mod.rs` | Task 2: `derive` sets `Tag::Idit` |
| `crates/panini-prakriya/src/tinanta/anga.rs` | Task 2: 6.4.24, 6.4.23's comment, four guard tests |
| `crates/panini-prakriya/src/tinanta/samjna.rs` | Task 2: `KSUBHNADI`, its upstream pin. Task 3: the pin's curated assertion; the reworded `cisphur_is_the_ciy_row_6_1_54_names` |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | Task 2: 8.4.39 and its guard test |
| `crates/panini-prakriya/src/rule.rs` | Task 2: `Rule.bars`'s doc names its third user |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Task 2: the order and bars pins, `derive_tags_idit_on_the_rows_idit_lists`, `dambh_loses_its_nasal_and_trp_keeps_snus_dental_n` |
| `crates/panini/tests/trace/svadi.rs` | Task 3: four trace pins and the fires-only roster |
| `crates/panini/tests/paradigm/main.rs` | Task 3: the census and keys and their svādi 5b paragraph, the pada-ambiguous set (1655), `svadi_analyses_its_5b_forms`. Task 4: doc counts, the alternates doc, the audit chain |
| `crates/panini/tests/paradigm/data/svadi.rs` | Task 3: 168 golden rows, 308 alternates |
| `crates/panini/tests/trace/rudhadi.rs`, `crates/panini-prakriya/src/tinanta/guna.rs` | Task 4: one comment each |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, the svādi gaṇa spec | Task 4 |
| `AGENTS.md`, maybe `mise.toml` | Task 5 |

---

## Task 1: The worktree, the baseline, and the gate

**Files:** none committed. The gate edits the worktree and reverts it.

**Interfaces:** none. Produces the private vidyut copy, the base archive, the throwaway scripts, and the gate's evidence in `/home/dev/svadi-5b-gate/`.

- [ ] **Step 1: Check the worktree**

```bash
cd /workspace/.worktrees/svadi-5b
git status --short                   # empty
git branch --show-current            # svadi-5b
git log --oneline -3                 # the plan commit, the spec commit 7b8c338, then b2707e7
```

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms (Global Constraints for the detached fallback). Expected: everything passes at 38232 cells. `panini-prakriya` reports 447 tests, the `trace` binary 217, `paradigm` 31 and `panini-data` 29 (and `panini` 7, `roundtrip` 1, `panini-analyze` 8, `cli` 5, `panini-lipi` 6).

- [ ] **Step 3: The private vidyut copy, the base archive and the scripts**

```bash
test -d /home/dev/vidyut-svadi-5b || cp -a /tmp/vidyut-full /home/dev/vidyut-svadi-5b
git -C /home/dev/vidyut-svadi-5b log --oneline -1          # 8da2f90 ...
mkdir -p /home/dev/vidyut-svadi-5b/slice5b /home/dev/svadi-5b-gate
rm -rf /home/dev/svadi-5b-base && mkdir -p /home/dev/svadi-5b-base
git archive HEAD | tar -x -C /home/dev/svadi-5b-base
```

The base archive is HEAD before any code change: the spec and plan commits touch only `docs/`, so its crates are `main`'s at `b2707e7`. If Task 6 rebases the branch, re-archive the new base before Task 4's dump is re-run.

Create these five files if they are missing, and check their hashes. `/home/dev/vidyut-svadi-5b/slice5b/repoint_5b.sh` (sha256 `2731ff70681f1a9555ae17783b04c439b0e79dce0add7d2976395cd041df7cb3`):

```bash
#!/usr/bin/env bash
# THROWAWAY: point the private vidyut copy's `panini` and `panini-data`
# dev-dependencies at the crates under $1 (a worktree or the base archive),
# then print them. Usage: bash repoint_5b.sh <tree>
set -euo pipefail
T="$(cd "$1" && pwd)"
C=/home/dev/vidyut-svadi-5b/vidyut-prakriya/Cargo.toml
sed -i "s#^panini = { path = .*#panini = { path = \"$T/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$T/crates/panini-data\" }#" "$C"
grep -n '^panini' "$C"
```

`/home/dev/vidyut-svadi-5b/slice5b/rows_5b.py` (sha256 `8c4a21fbb9f4691172183a295fd080060ebaf71a51b9a9a3f77c79121d4d998d`) inserts the thirty-two rows after `05.0021 stiG`, or with `--revert` takes them out:

```python
#!/usr/bin/env python3
"""THROWAWAY: svādi 5b's thirty-two svādi rows, inserted into `DHATUS` after
`05.0021 stiG` (the end of the svādi block). `--revert` takes them out again
(Task 1's gate applies and reverts them). Run from the worktree root."""
import sys
p = 'crates/panini-data/src/lib.rs'
anchor = """        dhatupatha: "05.0021",
        code: "stiG",
        gana: Gana::Svadi,
        pada: PadaAssignment::Atmanepada,
        artha: "Askandane",
    },
"""
rows = """    Dhatu {
        // 05.0001 `zu\\Y` aBizave (√su). Stored post-6.1.64, as `stiG` is.
        // Ubhayapadī by 1.3.72 (the ñ it), as are the nine after it. Svādi 5b.
        dhatupatha: "05.0001",
        code: "su",
        gana: Gana::Svadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "aBizave",
    },
    Dhatu {
        // 05.0002 `zi\\Y` banDane (√si). Stored post-6.1.64. Svādi 5b.
        dhatupatha: "05.0002",
        code: "si",
        gana: Gana::Svadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "banDane",
    },
    Dhatu {
        dhatupatha: "05.0003",
        code: "Si",
        gana: Gana::Svadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "niSAne",
    },
    Dhatu {
        // 05.0004 `qumi\\Y` prakzepaRe (√mi). The initial `qu` is an it by
        // 1.3.5. Svādi 5b.
        dhatupatha: "05.0004",
        code: "mi",
        gana: Gana::Svadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "prakzepaRe",
    },
    Dhatu {
        // 05.0005 `ci\\Y` cayane (√ci). 6.1.54 cisphuror ṇau names this root
        // too, but only before ṇic, which this row never takes: `CISPHUR`
        // holds curādi's `10.0124` alone. Svādi 5b.
        dhatupatha: "05.0005",
        code: "ci",
        gana: Gana::Svadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "cayane",
    },
    Dhatu {
        dhatupatha: "05.0006",
        code: "stf",
        gana: Gana::Svadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "AcCAdane",
    },
    Dhatu {
        dhatupatha: "05.0007",
        code: "kf",
        gana: Gana::Svadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        dhatupatha: "05.0008",
        code: "vf",
        gana: Gana::Svadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "varaRe",
    },
    Dhatu {
        dhatupatha: "05.0009",
        code: "Du",
        gana: Gana::Svadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "kampane",
    },
    Dhatu {
        dhatupatha: "05.0010",
        code: "DU",
        gana: Gana::Svadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "kampane",
    },
    Dhatu {
        // 05.0011 `wudu\\` upatApe (√du). The initial `wu` is an it by 1.3.5,
        // and the `\\` is the root vowel's own accent, not an it, so 1.3.78
        // makes it parasmaipadī, as it does the twenty-one after it. Svādi 5b.
        dhatupatha: "05.0011",
        code: "du",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "upatApe",
    },
    Dhatu {
        dhatupatha: "05.0013",
        code: "pf",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "prItO",
    },
    Dhatu {
        dhatupatha: "05.0014",
        code: "spf",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "prItipAlanayoH prIticalanayoSca",
    },
    Dhatu {
        dhatupatha: "05.0015",
        code: "smf",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "prItibalanayoH",
    },
    Dhatu {
        dhatupatha: "05.0018",
        code: "rAD",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "saMsidDO",
    },
    Dhatu {
        dhatupatha: "05.0019",
        code: "sAD",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "saMsidDO",
    },
    Dhatu {
        dhatupatha: "05.0022",
        code: "tik",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "Askandane gatO ca",
    },
    Dhatu {
        dhatupatha: "05.0023",
        code: "tig",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "Askandane gatO ca",
    },
    Dhatu {
        // 05.0024 `zaGa~` hiMsAyAm (√ṣagh). Stored post-6.1.64. Svādi 5b.
        dhatupatha: "05.0024",
        code: "saG",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 05.0025 `YiDfzA~` prAgalBye (√dhṛṣ). The initial `Yi` is an it by
        // 1.3.5, not the ñ it 1.3.72 reads. Svādi 5b.
        dhatupatha: "05.0025",
        code: "Dfz",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "prAgalBye",
    },
    Dhatu {
        // 05.0026 `danBu~` damBane (√dambh). Udit, not idit, so its nasal is
        // its own, and 6.4.24 aniditAM hala upaDAyAH kNiti elides it before
        // the ṅit śnu: daBnoti. Svādi 5b.
        dhatupatha: "05.0026",
        code: "danB",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "damBane",
    },
    Dhatu {
        dhatupatha: "05.0027",
        code: "fD",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vfdDO",
    },
    Dhatu {
        // 05.0028 `tfpa~` prIRane (√tṛp). 8.4.39 kzuBnAdizu ca keeps śnu's
        // `n` dental: tfpnoti, not *tfpRoti. Keyed by this row
        // (`KSUBHNADI`): curādi's `10.0351` and `10.0355` are `tfpa~` too.
        // Svādi 5b.
        dhatupatha: "05.0028",
        code: "tfp",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "prIRane",
    },
    Dhatu {
        dhatupatha: "05.0029",
        code: "ah",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyAptO",
    },
    Dhatu {
        dhatupatha: "05.0030",
        code: "daG",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "GAtane pAlane ca",
    },
    Dhatu {
        dhatupatha: "05.0031",
        code: "cam",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "BakzaRe",
    },
    Dhatu {
        dhatupatha: "05.0033",
        code: "kzi",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 05.0034 `ciri` hiMsAyAm. No anubandha: the final `i` is the
        // root's, as `jiri`'s and `fkzi`'s are below. Svādi 5b.
        dhatupatha: "05.0034",
        code: "ciri",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        dhatupatha: "05.0035",
        code: "jiri",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        dhatupatha: "05.0036",
        code: "dAS",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        dhatupatha: "05.0037",
        code: "df",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        dhatupatha: "05.0038",
        code: "fkzi",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
"""
s = open(p).read()
if '--revert' in sys.argv:
    old, new = anchor + rows, anchor
else:
    old, new = anchor, anchor + rows
if s.count(old) != 1:
    sys.exit(f"not applied: {s.count(old)}x the anchor")
open(p, 'w').write(s.replace(old, new))
print(("reverted" if '--revert' in sys.argv else "inserted") + " 32 rows")
```

`/home/dev/vidyut-svadi-5b/slice5b/engine_5b.py` (sha256 `9d864479613eab761c427c6cc3c15ed8f6e8d9d656337eac126ad6581113bb5a`) writes all the production code: `IDIT`, `Tag::Idit` and its wiring, 6.4.24 after 6.4.23 with 6.4.23's comment corrected, `KSUBHNADI`, 8.4.39 before 8.4.1, and `Rule.bars`'s doc. Task 2 applies it again, after its tests:

```python
#!/usr/bin/env python3
"""THROWAWAY: svādi 5b's production engine and data code — `IDIT` and
`Tag::Idit` with their `derive` wiring, 6.4.24 aniditAM hala upaDAyAH kNiti
after 6.4.23, 8.4.39 kzuBnAdizu ca before 8.4.1 with `KSUBHNADI`, and the
comments they make stale. Every `old` must occur exactly once in its file;
nothing is written if one fails. Run from the worktree root."""
import sys
E = {
'crates/panini-data/src/lib.rs': [
("""pub const AYA: [&str; 2] = ["10.0303", "10.0304"];
""",
"""pub const AYA: [&str; 2] = ["10.0303", "10.0304"];

/// The curated rows whose upadeśa is idit: it carries the `i~` it-marker,
/// as `hisi~` and `citi~` do. 7.1.58 *idito num dhātoḥ* gives such a root
/// its num, and this table stores the root with the num already in
/// (`07.0019 hins` for `hisi~`), a stated simplification. So a stored code
/// cannot say whether a nasal is the root's own or 7.1.58's: only the
/// upadeśa can. 6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti* needs that verdict:
/// it elides the nasal upadhā of an ANIDIT root before a kit or ṅit affix,
/// and spares an idit root's. Keyed by dhātupāṭha number, as `AYA` is. The
/// engine's `derive` tags a listed root `Tag::Idit`, which 6.4.24 reads.
///
/// Lists curated rows only: rudhādi's √hiṃs and eighty-six curādi rows.
/// `idit_matches_upadesha_markers` re-derives the table from the vendored
/// upadeśa and holds it to exactly the curated rows that carry the marker.
pub const IDIT: &[&str] = &[
    "07.0019", "10.0002", "10.0003", "10.0004", "10.0005", "10.0007", "10.0009", "10.0011",
    "10.0013", "10.0014", "10.0043", "10.0045", "10.0047", "10.0048", "10.0049", "10.0060",
    "10.0062", "10.0066", "10.0067", "10.0068", "10.0069", "10.0070", "10.0071", "10.0072",
    "10.0073", "10.0074", "10.0075", "10.0076", "10.0077", "10.0105", "10.0106", "10.0107",
    "10.0111", "10.0112", "10.0113", "10.0114", "10.0130", "10.0135", "10.0147", "10.0153",
    "10.0157", "10.0158", "10.0159", "10.0160", "10.0164", "10.0166", "10.0171", "10.0182",
    "10.0185", "10.0193", "10.0194", "10.0198", "10.0199", "10.0241", "10.0254", "10.0267",
    "10.0285", "10.0286", "10.0287", "10.0289", "10.0290", "10.0291", "10.0292", "10.0293",
    "10.0294", "10.0295", "10.0296", "10.0298", "10.0299", "10.0315", "10.0316", "10.0317",
    "10.0318", "10.0319", "10.0321", "10.0322", "10.0323", "10.0326", "10.0327", "10.0328",
    "10.0329", "10.0330", "10.0331", "10.0366", "10.0385", "10.0464", "10.0465",
];
"""),
],
'crates/panini-prakriya/src/term.rs': [
("""    /// `tinanta::sanadi`, which adds āya where no ṇic was taken.
    Aya,
""",
"""    /// `tinanta::sanadi`, which adds āya where no ṇic was taken.
    Aya,
    /// The dhātu is idit: its upadeśa carries the `i~` it-marker, so 7.1.58
    /// *idito num dhātoḥ* gave it its num, which the data layer stores
    /// already in (`hins`). Set by `tinanta::derive` from the row NUMBER
    /// (`panini_data::IDIT`), as `Aya` is: a stored code cannot say which
    /// nasal is 7.1.58's. A saṁjñā verdict, so no step is recorded. Read
    /// only by 6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti*, in `tinanta::anga`,
    /// which spares an idit root's nasal.
    Idit,
"""),
],
'crates/panini-prakriya/src/tinanta/mod.rs': [
("""use panini_data::{AYA, Dhatu, Gana, JNAPADI, Lakara, Pada, PadaAssignment, Purusha, Vacana};
""",
"""use panini_data::{AYA, Dhatu, Gana, IDIT, JNAPADI, Lakara, Pada, PadaAssignment, Purusha, Vacana};
"""),
("""    if AYA.contains(&dhatu.dhatupatha) {
        t.add(Tag::Aya);
    }
""",
"""    if AYA.contains(&dhatu.dhatupatha) {
        t.add(Tag::Aya);
    }
    // 7.1.58's idit, likewise by row number (`IDIT`): 6.4.24 reads it.
    if IDIT.contains(&dhatu.dhatupatha) {
        t.add(Tag::Idit);
    }
"""),
],
'crates/panini-prakriya/src/tinanta/anga.rs': [
("""    // This is also why 6.4.24 aniditāṁ hala upadhāyāḥ kṅiti is not needed in
    // this slice: it governs the PENULTIMATE nasal of roots like √añj and
    // √tañc (out of scope here), whereas the nasal 6.4.23 removes sits
    // immediately behind śnam's `na` and is already this rule's by its own
    // terms.
""",
"""    // 6.4.24 aniditāṁ hala upadhāyāḥ kṅiti, just below, elides a nasal too,
    // but the PENULTIMATE sound of a hal-final ANGA. After 3.1.78's split no
    // rudhādi ANGA is hal-final — it ends in the root's last vowel, and the
    // root's nasal sits behind śnam's `na`, in SHAP — so that nasal is this
    // rule's by its own terms, never 6.4.24's.
"""),
("""            let head: String = p.terms[SHAP].text.chars().take(2).collect();
            p.terms[SHAP].text = format!("{head}{}", &rest[1..]);
            p.record("6.4.23", "SnAnnalopaH", before);
            true
        },
    },
];
""",
"""            let head: String = p.terms[SHAP].text.chars().take(2).collect();
            p.terms[SHAP].text = format!("{head}{}", &rest[1..]);
            p.record("6.4.23", "SnAnnalopaH", before);
            true
        },
    },
    // 6.4.24 aniditāṁ hala upadhāyāḥ kṅiti: an anidit root's nasal upadhā is
    // elided before a kit or ṅit affix, when the aṅga ends in a hal. danB +
    // nu → daB + nu, whence daBnoti (√dambh, svādi 5b).
    //
    // *kṅiti* reads the NEXT NON-EMPTY TERM after ANGA: the affix the aṅga
    // stands before. 1.2.4's second application tags an apit sārvadhātuka
    // vikaraṇa ṅit (śnu, śnā, śyan, śa); where the vikaraṇa is luk'd or
    // ślu'd, SHAP is empty and the affix is the ending, which 1.2.4's first
    // application tags. śap is pit and never ṅit, so no bhvādi aṅga meets
    // this rule. vidyut-prakriya reads the same next non-empty term.
    //
    // *aniditām* reads `Tag::Idit` (the data layer's `IDIT`): 7.1.58 idito num
    // dhātoḥ is stored, not derived, so an idit root's num already sits in
    // its code (`hins`), and only the upadeśa can say the nasal is 7.1.58's.
    // No curated idit root reaches this rule with a nasal upadhā before a ṅit
    // affix — curādi's meet ṇic or śap, and √hiṃs's nasal is 6.4.23's — so
    // the guard's one witness is the hand-built
    // `anidit_upadha_nalopa_spares_an_idit_root`.
    //
    // No rudhādi aṅga reaches it either (6.4.23's comment above). vidyut
    // credits this rule on √und's `unad → und`, which is 6.4.23's deletion
    // here; `unantas_trace_orders_6_4_23_before_6_4_111` in `panini`'s trace
    // suite keeps that credit out.
    //
    // After 6.4.23 and before the guṇa stage's 7.3.84, as vidyut runs it.
    Rule {
        id: "6.4.24",
        name: "aniditAM hala upaDAyAH kNiti",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if p.terms[ANGA].has(Tag::Idit) {
                return false;
            }
            let Some(next) = p.terms[ANGA + 1..].iter().find(|t| !t.text.is_empty()) else {
                return false;
            };
            if !next.has(Tag::Ngit) {
                return false;
            }
            let chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let [head @ .., 'n', last] = chars.as_slice() else {
                return false;
            };
            if is_vowel(*last) {
                return false;
            }
            let text: String = head.iter().chain([last]).collect();
            let before = p.snapshot();
            p.terms[ANGA].text = text;
            p.record("6.4.24", "aniditAM hala upaDAyAH kNiti", before);
            true
        },
    },
];
"""),
],
'crates/panini-prakriya/src/tinanta/samjna.rs': [
("""pub(crate) const KRP: [&str; 1] = ["10.0278"];
""",
"""pub(crate) const KRP: [&str; 1] = ["10.0278"];

/// 8.4.39 kṣubhnādiṣu ca: the dhātupāṭha rows whose ṇatva the sūtra's
/// kṣubhnādi blocks. By number, as `KRP` is: vidyut-prakriya keys the sūtra
/// on the upadeśa `tfpa~` before śnu, and curādi `10.0351` and `10.0355`
/// (and tudādi `06.0028`) are `tfpa~` too, stored as the same `tfp`, and
/// take no śnu. `05.0028 tfpa~` (*tṛpnoti*) is the one curated; kryādi
/// `09.0055 kzuBa~` (*kṣubhnāti*) joins the list when it is. Pinned to
/// upstream by `ksubhnadi_is_the_row_8_4_39_names`. The tripādī 8.4.39 reads
/// it.
pub(crate) const KSUBHNADI: [&str; 1] = ["05.0028"];
"""),
],
'crates/panini-prakriya/src/tinanta/tripadi.rs': [
("""use crate::tinanta::samjna::KRP;
""",
"""use crate::tinanta::samjna::{KRP, KSUBHNADI};
"""),
("""    // 8.4.1 raṣābhyāṁ no ṇaḥ samānapade: `n` → `ṇ` when `r`/`ṣ` DIRECTLY
""",
"""    // 8.4.39 kṣubhnādiṣu ca: no ṇatva in the kṣubhnādi. √tṛp's `f` would
    // otherwise reach śnu's `n` across the pu-varga `p` by 8.4.2 (*tfpRoti*
    // for tfpnoti). Keyed by row (`super::samjna::KSUBHNADI`), as vidyut-
    // prakriya keys it by upadeśa: curādi's `10.0351` and `10.0355` are
    // `tfpa~`, stored `tfp`, too. vidyut also requires śnu after the dhātu;
    // every svādi aṅga stands before śnu (3.1.73), so `Tag::Svadi` is that
    // test.
    //
    // Changes no text: it records and bars 8.4.1 and 8.4.2, as 6.4.117 ā ca
    // hau bars the rules that would change its `A`. Placed just above 8.4.1,
    // the first rule it bars.
    Rule {
        id: "8.4.39",
        name: "kzuBnAdizu ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &["8.4.1", "8.4.2"],
        apply: |p| {
            if !KSUBHNADI.contains(&p.ctx.dhatupatha) || !p.terms[ANGA].has(Tag::Svadi) {
                return false;
            }
            let before = p.snapshot();
            p.record("8.4.39", "kzuBnAdizu ca", before);
            true
        },
    },
    // 8.4.1 raṣābhyāṁ no ṇaḥ samānapade: `n` → `ṇ` when `r`/`ṣ` DIRECTLY
"""),
],
'crates/panini-prakriya/src/rule.rs': [
("""    /// aṅga's laghu upadhā before a vowel-initial pit ending (nenijAni).
""",
"""    /// aṅga's laghu upadhā before a vowel-initial pit ending (nenijAni).
    /// 8.4.39 *kṣubhnādiṣu ca* is the third: it changes no text and bars 8.4.1
    /// and 8.4.2, keeping √tṛp's śnu dental (tfpnoti).
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

`/home/dev/vidyut-svadi-5b/slice5b/audit_5b.py` (sha256 `5ad64d197cafe9e514d8f3c8742ef70d92fea174398f65de5591e82fd0abf22f`) raises the committed harness's asserted totals and its doc to 626 / 39744 / 51724; Task 4 applies it again for the commit:

```python
#!/usr/bin/env python3
"""THROWAWAY: svādi 5b's edits to tools/audit/panini_full_audit.rs. Every
`old` must occur exactly once; nothing is written if one fails. Run from the
worktree root."""
import sys
p = 'tools/audit/panini_full_audit.rs'
E = [
("//! What it compares: for each of the 594 curated roots, for each pada the root\n",
 "//! What it compares: for each of the 626 curated roots, for each pada the root\n"),
("//! admits (`Dhatu::padas`; two apiece for the 468 roots that admit both padas —\n//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 428 curādi\n",
 "//! admits (`Dhatu::padas`; two apiece for the 478 roots that admit both padas —\n//! thirty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 428 curādi\n"),
("//! Corpus invariants, asserted: 594 roots, 38232 cells, 49904 forms. These are\n",
 "//! Corpus invariants, asserted: 626 roots, 39744 cells, 51724 forms. These are\n"),
("//! (`derivation_set_shape_matches_the_audited_numbers`): 4248 root×pada×lakāra\n//! blocks × 9 cells, plus 11672 `ALTERNATES` rows. If this harness's\n",
 "//! (`derivation_set_shape_matches_the_audited_numbers`): 4416 root×pada×lakāra\n//! blocks × 9 cells, plus 11980 `ALTERNATES` rows. If this harness's\n"),
("//! Optionally dump the full 38232-cell table:\n",
 "//! Optionally dump the full 39744-cell table:\n"),
("""    assert_eq!(roots_seen.len(), 594, "curated roots");
    assert_eq!(n_cells, 38232, "cells: 4248 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 49904, "forms: 38232 cells + 11672 ALTERNATES rows");
""",
"""    assert_eq!(roots_seen.len(), 626, "curated roots");
    assert_eq!(n_cells, 39744, "cells: 4416 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 51724, "forms: 39744 cells + 11980 ALTERNATES rows");
"""),
]
s = open(p).read()
bad = [o[:70] for o, _ in E if s.count(o) != 1]
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
for o, n in E:
    s = s.replace(o, n)
open(p, 'w').write(s)
print(f"applied {len(E)} edits to 1 files")
```

`/home/dev/vidyut-svadi-5b/vidyut-prakriya/examples/trace_dump_5b.rs` (sha256 `94a73d89c2cb8957493a15a4412d7d6f61d415a5e6d3b73a4bd6d03c0aa9dbfb`) dumps every prior branch, blocked ones included, with every step's before and after text. It lists the new rows literally, so it builds against the base too:

```rust
//! THROWAWAY: slice 5b — dump every prior cell's branches, blocked ones
//! included, each with its credited-rule log and every step's before/after.
use panini::Panini;
use panini_data::{Lakara as L, Purusha as P, Vacana as V};
/// Slice 5b's thirty-two rows, listed literally so the dump also builds
/// against the base, which does not have them.
const NEW: &[&str] = &[
    "05.0001", "05.0002", "05.0003", "05.0004", "05.0005", "05.0006", "05.0007", "05.0008",
    "05.0009", "05.0010", "05.0011", "05.0013", "05.0014", "05.0015", "05.0018", "05.0019",
    "05.0022", "05.0023", "05.0024", "05.0025", "05.0026", "05.0027", "05.0028", "05.0029",
    "05.0030", "05.0031", "05.0033", "05.0034", "05.0035", "05.0036", "05.0037", "05.0038",
];
fn main() {
    let panini = Panini::new();
    for d in panini_data::dhatus() {
        if NEW.contains(&d.dhatupatha) { continue; }
        for pada in d.padas() {
            for l in [L::Lat, L::Lan, L::Lot, L::VidhiLin] { for pu in [P::Prathama, P::Madhyama, P::Uttama] { for va in [V::Eka, V::Dvi, V::Bahu] {
                for b in panini.derive(d, l, *pada, pu, va) {
                    let steps: Vec<String> = b.log.iter().map(|s| format!("{}:{}>{}", s.sutra, s.before, s.after)).collect();
                    println!("{} {:?} {:?} {:?} {:?} blocked={} {} : {}", d.dhatupatha, pada, l, pu, va, b.blocked, b.text(), steps.join(" "));
                }
            }}}
        }
    }
}
```

```bash
cd /home/dev/vidyut-svadi-5b && sha256sum slice5b/repoint_5b.sh slice5b/rows_5b.py slice5b/engine_5b.py slice5b/audit_5b.py vidyut-prakriya/examples/trace_dump_5b.rs
```

- [ ] **Step 4: The gate's negative control — the rows alone fail the audit on exactly √dambh and √tṛp**

```bash
cd /workspace/.worktrees/svadi-5b
S=/home/dev/vidyut-svadi-5b/slice5b; V=/home/dev/vidyut-svadi-5b/vidyut-prakriya; G=/home/dev/svadi-5b-gate
python3 $S/rows_5b.py                  # inserted 32 rows
python3 $S/audit_5b.py                 # applied 6 edits to 1 files
cp tools/audit/panini_full_audit.rs $V/examples/
bash $S/repoint_5b.sh .                # both lines must name /workspace/.worktrees/svadi-5b/crates
(cd $V && PANINI_AUDIT_VIDYUT=/home/dev/vidyut-svadi-5b PANINI_AUDIT_REPO=/workspace/.worktrees/svadi-5b mise exec rust@1.99.0 -- cargo run -q --release --example panini_full_audit > $G/audit-rows-only.txt 2>&1); echo "exit $?"
grep "^DIFF" $G/audit-rows-only.txt | awk '{print $2, $3}' | sort | uniq -c
tail -1 $G/audit-rows-only.txt
```

Foreground, timeout 600000 ms (the first build of the private copy takes a few minutes). Expected: `exit 1`; `36 05.0026 (danB)` and `36 05.0028 (tfp)` and nothing else; `AUDIT FAILED: 72 differing cells.` This proves the audit sees exactly the two rows the rules are for, and that the other thirty match on data alone. Any other row in the list, or any other count: stop and report.

- [ ] **Step 5: The engine, then the gate**

```bash
cd /workspace/.worktrees/svadi-5b
S=/home/dev/vidyut-svadi-5b/slice5b; V=/home/dev/vidyut-svadi-5b/vidyut-prakriya; G=/home/dev/svadi-5b-gate
python3 $S/engine_5b.py                # applied 10 edits to 7 files
mise run fmt
mise exec -- cargo build -q --workspace
# 1. Every prior branch, base against this tree.
bash $S/repoint_5b.sh /home/dev/svadi-5b-base      # both lines must name /home/dev/svadi-5b-base/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_5b 2>/dev/null > $G/dump-base.txt)
bash $S/repoint_5b.sh .                            # both lines must name /workspace/.worktrees/svadi-5b/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_5b 2>/dev/null > $G/dump-head.txt)
wc -l < $G/dump-base.txt; wc -l < $G/dump-head.txt     # 56456 and 56456
grep -c "blocked=false" $G/dump-base.txt                # 49904
sha256sum $G/dump-base.txt                              # ce7780cf9012e63d903924f24f597246fba396f39f88547819888fd929edb3ef
cmp $G/dump-base.txt $G/dump-head.txt && echo PRIOR-TRACES-IDENTICAL
# 2. The audit over all 626 roots: the negative control first, then the honest run.
(cd $V && PANINI_AUDIT_VIDYUT=/home/dev/vidyut-svadi-5b PANINI_AUDIT_REPO=/workspace/.worktrees/svadi-5b PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run -q --release --example panini_full_audit 2>&1 | tail -1)
(cd $V && PANINI_AUDIT_VIDYUT=/home/dev/vidyut-svadi-5b PANINI_AUDIT_REPO=/workspace/.worktrees/svadi-5b mise exec rust@1.99.0 -- cargo run -q --release --example panini_full_audit > $G/audit-gate.txt 2>&1); echo "exit $?"
tail -9 $G/audit-gate.txt
```

Foreground, timeout 600000 ms per command. Expected:
- `56456`, `56456`, `49904`, the hash above, then `PRIOR-TRACES-IDENTICAL`. The general 6.4.24 and 8.4.39 touch no prior branch, not one step's text.
- The `entry` control: `AUDIT FAILED: 36 differing cells.`
- The honest run: `exit 0`, then `roots : 626`, `cells : 39744`, `forms (set sizes): 51724`, `live branches : 51724`, `blocked branches : 6552`, `differing cells  : 0`, and `AUDIT PASSED: 39744 cells, 51724 forms, zero differences.`

**If `cmp` reports any difference, or the honest audit shows any difference: STOP.** Do not continue to Task 2 and do not edit anything to make it pass. Report the first differing dump lines (`diff $G/dump-base.txt $G/dump-head.txt | head -20`) or the audit's `DIFF` lines; the spec says the slice stops and is amended.

- [ ] **Step 6: Revert the worktree**

```bash
cd /workspace/.worktrees/svadi-5b
git checkout -- . && git status --short      # empty
ls /home/dev/svadi-5b-gate                    # audit-gate.txt audit-rows-only.txt dump-base.txt dump-head.txt
```

Nothing is committed in this task. Tasks 2 and 3 apply the same scripts again, test-first.

---

## Task 2: The engine — `IDIT`, 6.4.24 and 8.4.39, test-first

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (`IDIT`; `is_idit` and `idit_matches_upadesha_markers` in its tests)
- Modify: `crates/panini-prakriya/src/term.rs`, `src/rule.rs`, `src/tinanta/mod.rs`, `src/tinanta/anga.rs`, `src/tinanta/samjna.rs`, `src/tinanta/tripadi.rs`
- Test: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` and the stage files' test modules

**Interfaces:**
- Consumes: `ANGA`, `ENDING` and `with_slots(Vec<Term>) -> Vec<Term>` (`tinanta/terms.rs`; `with_slots` prepends the empty `AGAMA` and `ABHYASA`); `is_vowel(char) -> bool` (`tinanta/sound.rs`, already imported in `anga.rs`); `Prakriya { terms, ctx: Context { dhatupatha: &'static str, .. }, log: Vec<RuleStep>, .. }` with `snapshot()` / `record(id, name, before)` / `text()`; `sole(Vec<Prakriya>) -> Prakriya` in `derivation_tests.rs`; `upstream_upadesha(&str) -> Option<&'static str>` in `samjna.rs`'s tests; `upstream_rows() -> Vec<(&str, &str, &str)>` in `panini-data`'s tests.
- Produces: `pub const IDIT: &[&str]` (87 entries) in `panini_data`; `Tag::Idit`; `pub(crate) const KSUBHNADI: [&str; 1]` in `tinanta::samjna`; the rules `"6.4.24"` (in `ANGA_RULES`) and `"8.4.39"` (in `TRIPADI`, `bars: &["8.4.1", "8.4.2"]`); the test-only `fn is_idit(&str) -> bool` and `fn svadi_row(&'static str, &'static str) -> Dhatu`. Task 3 extends `ksubhnadi_is_the_row_8_4_39_names` and rewrites `cisphur_is_the_ciy_row_6_1_54_names`.

- [ ] **Step 1: The failing tests**

Create `/home/dev/vidyut-svadi-5b/slice5b/tests_5b.py` if it is missing (sha256 `aedf028b940dca6c64ec4e63ba0bdc233884ebd53db1d5d018ace6ba761abdde`). It adds, by file:
- **`anga.rs`:** a `before_affix` helper and four guard tests for 6.4.24, each looking the rule up in `ANGA_RULES`: it fires on `danB` before a ṅit śnu, on the five-sound `granT` (the penultimate sound, wherever the length puts it) and across an empty `SHAP` to a ṅit ending; it spares a `Tag::Idit` aṅga (the guard's only witness); it declines before a pit śap, and before a pit śap followed by a ṅit ending (the next non-empty term decides); it declines on `mnA` (vowel-final), `tfp` (no nasal upadhā) and `Ba` (a rudhādi split aṅga).
- **`tripadi.rs`:** `ksubhnadi_records_on_its_row_before_snu_only`, looking the rule up in `TRIPADI`: it records on `05.0028` with no text change, declines on `10.0351`, `10.0355` and `""` even before śnu, and on `05.0028` when the aṅga is not svādi.
- **`samjna.rs`:** `ksubhnadi_is_the_row_8_4_39_names`, the upstream pin.
- **`derivation_tests.rs`:** the order pin gains `6.4.24` after `6.4.23` and `8.4.39` before `8.4.1`; the bars pin gains `("8.4.39", ["8.4.1", "8.4.2"])`; `derive_tags_idit_on_the_rows_idit_lists`; and `dambh_loses_its_nasal_and_trp_keeps_snus_dental_n`, hand-built rows deriving *daBnoti* / *daBnutaH* (6.4.24 once) and *tfpnoti* / *tfpnutaH* (8.4.39, no 8.4.1 or 8.4.2).
- **`panini-data`:** `is_idit`, the one idit reading that `stored_form`, `IDIT` and 2564's arm of `optional_nic_from_upadesha` now share (two of them had their own copy), and `idit_matches_upadesha_markers`, both directions.

```python
#!/usr/bin/env python3
"""THROWAWAY: svādi 5b, Task 2's failing tests — 6.4.24's and 8.4.39's guard
tests, `KSUBHNADI`'s upstream pin, `Tag::Idit`'s `derive` wiring, hand-built
√dambh and √tṛp derivations, `IDIT`'s two-way data test, and the rule-order
and bars pins. Every `old` must occur exactly once in its file; nothing is
written if one fails. Run from the worktree root."""
import sys
E = {
'crates/panini-prakriya/src/tinanta/anga.rs': [
("""        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "atCid");
    }
}
""",
"""        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "atCid");
    }

    /// A hand-built aṅga before a vikaraṇa and an ending, the vikaraṇa tagged
    /// as 1.2.4 leaves it: `Ngit` (śnu, śnā) or `Pit` (śap).
    fn before_affix(anga: &str, vikarana: &str, tag: Tag, ending: &str) -> Prakriya {
        let mut v = Term::new(vikarana);
        v.add(tag);
        Prakriya {
            terms: with_slots(vec![Term::new(anga), v, Term::new(ending)]),
            ..Default::default()
        }
    }

    #[test]
    fn anidit_upadha_nalopa_elides_the_nasal_before_a_ngit_affix() {
        // This stage's entry, looked up in its own static.
        let rule = ANGA_RULES.iter().find(|r| r.id == "6.4.24").unwrap();
        assert!(!rule.vikalpa);

        // √dambh before the ṅit śnu: danB → daB (daBnoti).
        let mut p = before_affix("danB", "nu", Tag::Ngit, "ti");
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "daB");
        assert_eq!(p.text(), "daBnuti");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["6.4.24"]);

        // A five-sound aṅga: the elided `n` is the penultimate sound, not a
        // fixed index (√granth before śnā, kryādi's graTnAti).
        let mut p = before_affix("granT", "nA", Tag::Ngit, "ti");
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "graT");

        // An empty vikaraṇa (luk, ślu) is skipped: the affix is the ending,
        // which 1.2.4's first application tags ṅit.
        let mut ending = Term::new("tas");
        ending.add(Tag::Ngit);
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("danB"), Term::new(""), ending]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "daBtas");
    }

    #[test]
    fn anidit_upadha_nalopa_spares_an_idit_root() {
        // *aniditām*: an idit root's nasal is 7.1.58's num, stored in its code
        // (`hins`), and stays. No curated idit root meets a ṅit affix with a
        // nasal upadhā, so this hand-built aṅga is the guard's only witness.
        let rule = ANGA_RULES.iter().find(|r| r.id == "6.4.24").unwrap();
        let mut p = before_affix("danB", "nu", Tag::Ngit, "ti");
        p.terms[ANGA].add(Tag::Idit);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "danBnuti");
        assert!(p.log.is_empty());
    }

    #[test]
    fn anidit_upadha_nalopa_reads_only_the_next_affixs_ngit() {
        let rule = ANGA_RULES.iter().find(|r| r.id == "6.4.24").unwrap();
        // śap is pit and never ṅit: a bhvādi aṅga keeps its nasal.
        let mut p = before_affix("danB", "a", Tag::Pit, "ti");
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "danBati");

        // The affix the aṅga stands before decides, not a later one: a
        // non-ṅit vikaraṇa before a ṅit ending declines.
        let mut p = before_affix("danB", "a", Tag::Pit, "tas");
        p.terms[ENDING].add(Tag::Ngit);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "danBatas");
        assert!(p.log.is_empty());
    }

    #[test]
    fn anidit_upadha_nalopa_needs_a_hal_final_anga_with_a_nasal_upadha() {
        let rule = ANGA_RULES.iter().find(|r| r.id == "6.4.24").unwrap();
        for (anga, why) in [
            // A nasal upadhā, but the aṅga ends in a vowel.
            ("mnA", "vowel-final"),
            // Hal-final, with no nasal upadhā (√tṛp).
            ("tfp", "no nasal upadhā"),
            // A rudhādi ANGA after 3.1.78's split ends in the root's last
            // vowel; its nasal is in SHAP, 6.4.23's.
            ("Ba", "rudhādi split"),
        ] {
            let mut p = before_affix(anga, "nu", Tag::Ngit, "ti");
            assert!(!(rule.apply)(&mut p), "{why}");
            assert_eq!(p.terms[ANGA].text, anga, "{why}");
            assert!(p.log.is_empty(), "{why}");
        }
    }
}
"""),
],
'crates/panini-prakriya/src/tinanta/tripadi.rs': [
("""            let mut p = bhas_prakriya(anga, ending);
            assert!(!(rule.apply)(&mut p), "{why}");
            assert!(p.log.is_empty(), "{why}");
        }
    }
}
""",
"""            let mut p = bhas_prakriya(anga, ending);
            assert!(!(rule.apply)(&mut p), "{why}");
            assert!(p.log.is_empty(), "{why}");
        }
    }

    #[test]
    fn ksubhnadi_records_on_its_row_before_snu_only() {
        // This stage's entry, looked up in its own static.
        let rule = TRIPADI.iter().find(|r| r.id == "8.4.39").unwrap();
        assert!(!rule.vikalpa);
        assert_eq!(rule.bars, ["8.4.1", "8.4.2"]);
        let trp = |row: &'static str, gana: Tag| {
            let mut anga = Term::new("tfp");
            anga.add(gana);
            let mut p = Prakriya {
                terms: with_slots(vec![anga, Term::new("no"), Term::new("ti")]),
                ..Default::default()
            };
            p.ctx.dhatupatha = row;
            p
        };

        // √tṛp before śnu: recorded, no text changed. `run_pipeline` then
        // skips the barred 8.4.1 and 8.4.2 on the branch;
        // `tfpnoti_trace_credits_8_4_39_and_no_natva` in `panini`'s trace
        // suite pins that effect.
        let mut p = trp("05.0028", Tag::Svadi);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "tfpnoti");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["8.4.39"]);

        // Another row declines even before śnu: curādi's `tfpa~` rows store
        // the same `tfp`, and a hand-built row has no number.
        for row in ["10.0351", "10.0355", ""] {
            let mut p = trp(row, Tag::Svadi);
            assert!(!(rule.apply)(&mut p), "{row}");
            assert!(p.log.is_empty(), "{row}");
        }

        // Its row, but no śnu after it: an aṅga that is not svādi declines.
        let mut p = trp("05.0028", Tag::Kryadi);
        assert!(!(rule.apply)(&mut p));
        assert!(p.log.is_empty());
    }
}
"""),
],
'crates/panini-prakriya/src/tinanta/samjna.rs': [
("""    #[test]
    fn ghu_is_exactly_the_six_rows_1_1_20_names() {
""",
"""    #[test]
    fn ksubhnadi_is_the_row_8_4_39_names() {
        // 8.4.39 kṣubhnādiṣu ca. vidyut-prakriya keys its śnu arm on the
        // upadeśa `tfpa~`. Tudādi's and curādi's two `tfpa~` rows store the
        // same `tfp` and take no śnu; kryādi's `kzuBa~` joins when its row
        // is curated.
        assert_eq!(KSUBHNADI, ["05.0028"]);
        for number in ["05.0028", "06.0028", "10.0351", "10.0355"] {
            assert_eq!(upstream_upadesha(number), Some("tfpa~"), "{number}");
        }
        assert_eq!(upstream_upadesha("09.0055"), Some("kzuBa~"));
        assert!(!dhatus().iter().any(|d| d.dhatupatha == "09.0055"));
    }

    #[test]
    fn ghu_is_exactly_the_six_rows_1_1_20_names() {
"""),
],
'crates/panini-prakriya/src/tinanta/derivation_tests.rs': [
("""        "7.2.79", "7.2.80", "7.2.81", "6.4.23", "7.4.21",""",
"""        "7.2.79", "7.2.80", "7.2.81", "6.4.23", "6.4.24", "7.4.21","""),
("""        "8.3.13", "8.4.53", "8.4.54", "8.2.38", "8.4.55", "8.4.1", "8.4.2",""",
"""        "8.3.13", "8.4.53", "8.4.54", "8.2.38", "8.4.55", "8.4.39", "8.4.1", "8.4.2","""),
("""        ("6.4.117", &["6.4.116", "6.4.113", "6.4.112"][..]),
    ];
""",
"""        ("6.4.117", &["6.4.116", "6.4.113", "6.4.112"][..]),
        ("8.4.39", &["8.4.1", "8.4.2"][..]),
    ];
"""),
("""        assert!(!ids.contains(&"8.2.30"), "{form}: {ids:?}");
        assert!(!ids.contains(&"7.3.86"), "{form}: {ids:?}");
    }
}
""",
"""        assert!(!ids.contains(&"8.2.30"), "{form}: {ids:?}");
        assert!(!ids.contains(&"7.3.86"), "{form}: {ids:?}");
    }
}

#[test]
fn derive_tags_idit_on_the_rows_idit_lists() {
    // 7.1.58's verdict, by row number (`panini_data::IDIT`), as `Aya` is:
    // √hiṃs and curādi √cint are idit; √kṛt and √cur are not, nor is svādi
    // √dambh (hand-built), whose upadeśa `danBu~` is udit.
    let curated = |number: &str| *dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
    let dambh = Dhatu {
        dhatupatha: "05.0026",
        code: "danB",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "",
    };
    for (d, is_idit) in [
        (curated("07.0019"), true),
        (curated("10.0002"), true),
        (curated("07.0010"), false),
        (curated("10.0001"), false),
        (dambh, false),
    ] {
        let p = derive(
            &d,
            Lakara::Lat,
            d.padas()[0],
            Purusha::Prathama,
            Vacana::Eka,
        )
        .into_iter()
        .next()
        .unwrap();
        assert_eq!(p.terms[ANGA].has(Tag::Idit), is_idit, "{}", d.dhatupatha);
    }
}

/// A svādi row a slice's rules reach, hand-built (svādi 5b's): `derive` reads only
/// the row number, the code, the gaṇa and the pada assignment.
fn svadi_row(dhatupatha: &'static str, code: &'static str) -> Dhatu {
    Dhatu {
        dhatupatha,
        code,
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "",
    }
}

#[test]
fn dambh_loses_its_nasal_and_trp_keeps_snus_dental_n() {
    // Svādi 5b's `05.0026 danBu~` and `05.0028 tfpa~`, hand-built, laṭ
    // prathama eka and dvi: vidyut-prakriya's forms. 6.4.24 elides √dambh's
    // nasal before the ṅit śnu, once; 8.4.39 keeps √tṛp's śnu dental, so
    // neither ṇatva rule is credited.
    let dambh = svadi_row("05.0026", "danB");
    let trp = svadi_row("05.0028", "tfp");
    for (vacana, dambh_form, trp_form) in [
        (Vacana::Eka, "daBnoti", "tfpnoti"),
        (Vacana::Dvi, "daBnutaH", "tfpnutaH"),
    ] {
        let p = sole(derive(
            &dambh,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            vacana,
        ));
        assert_eq!(p.text(), dambh_form);
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(
            ids.iter().filter(|i| **i == "6.4.24").count(),
            1,
            "{dambh_form}: {ids:?}"
        );

        let p = sole(derive(
            &trp,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            vacana,
        ));
        assert_eq!(p.text(), trp_form);
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert!(ids.contains(&"8.4.39"), "{trp_form}: {ids:?}");
        assert!(!ids.contains(&"8.4.1"), "{trp_form}: {ids:?}");
        assert!(!ids.contains(&"8.4.2"), "{trp_form}: {ids:?}");
    }
}
"""),
],
'crates/panini-data/src/lib.rs': [
("""    /// The form this repo stores as `Dhatu::code`, derived from an upstream
    /// upadeśa.
    fn stored_form(upadesha: &str) -> String {
""",
"""    /// Whether an upstream upadeśa is idit: its LAST marker is `i~`, accents
    /// aside (`hisi~`, `citi~`, `aci~^`). A non-final `i~` belongs to another
    /// marker — irit `i~r` (`ru\\Di~^r`, `Guzi~r`) or `cakzi~N`. The one
    /// reading `stored_form`, `IDIT` (`idit_matches_upadesha_markers`) and
    /// 2564's arm of `optional_nic_from_upadesha` share.
    fn is_idit(upadesha: &str) -> bool {
        upadesha.trim_end_matches(['\\\\', '^']).ends_with("i~")
    }

    /// The form this repo stores as `Dhatu::code`, derived from an upstream
    /// upadeśa.
    fn stored_form(upadesha: &str) -> String {
"""),
("""        let idit = upadesha.trim_end_matches(['\\\\', '^']).ends_with("i~");
        match s.rfind(|c: char| !is_hal(c)) {
""",
"""        let idit = is_idit(upadesha);
        match s.rfind(|c: char| !is_hal(c)) {
"""),
("""        let u = upadesha.trim_end_matches(['\\\\', '^']);
        if u.ends_with("i~") {
            Some("2564")
""",
"""        let u = upadesha.trim_end_matches(['\\\\', '^']);
        if is_idit(upadesha) {
            Some("2564")
"""),
("""    #[test]
    fn optional_nic_matches_upadesha_markers() {
""",
"""    #[test]
    fn idit_matches_upadesha_markers() {
        // Both directions, as `optional_nic_matches_upadesha_markers` holds
        // its table: every entry's upadeśa is idit and its row curated, and
        // every curated row whose upadeśa is idit is an entry. Non-circular:
        // the upadeśa comes from the vendored dhātupāṭha, not from the table.
        let rows = upstream_rows();
        let upadesha = |n: &str| rows.iter().find(|(m, _, _)| *m == n).unwrap().1;
        for n in IDIT {
            assert!(is_idit(upadesha(n)), "{n} {}", upadesha(n));
            assert!(
                dhatus().iter().any(|d| d.dhatupatha == *n),
                "{n} is not curated"
            );
        }
        for d in dhatus() {
            assert_eq!(
                IDIT.contains(&d.dhatupatha),
                is_idit(upadesha(d.dhatupatha)),
                "{} {}",
                d.dhatupatha,
                upadesha(d.dhatupatha)
            );
        }
        assert!(IDIT.windows(2).all(|w| w[0] < w[1]), "sorted, no repeats");
        assert_eq!(IDIT.len(), 87);
        // √hiṃs is the one entry outside curādi: its stored `hins` holds
        // 7.1.58's num.
        assert_eq!(upadesha("07.0019"), "hisi~");
        assert_eq!(IDIT.iter().filter(|n| !n.starts_with("10.")).count(), 1);
        // The irit `i~r` is not idit, nor is a root's own final `i`.
        assert!(!is_idit("Guzi~r"));
        assert!(!is_idit("ciri"));
    }

    #[test]
    fn optional_nic_matches_upadesha_markers() {
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
python3 /home/dev/vidyut-svadi-5b/slice5b/tests_5b.py      # applied 11 edits to 5 files
mise run fmt
```

`mise run fmt` reflows the order pin's lines; that is expected.

- [ ] **Step 2: Run them to see them fail**

```bash
mise exec -- cargo test -q -p panini-prakriya --no-run 2>&1 | grep -E "^error" | sort | uniq -c
mise exec -- cargo test -q -p panini-data --no-run 2>&1 | grep -E "^error" | sort | uniq -c
```

Expected: neither compiles. `panini-prakriya` names `KSUBHNADI` (E0425) and `Tag::Idit` (E0599); `panini-data` names `IDIT` (E0425, five times).

- [ ] **Step 3: The engine**

```bash
python3 /home/dev/vidyut-svadi-5b/slice5b/engine_5b.py      # applied 10 edits to 7 files
mise run fmt
```

The two rules, as `engine_5b.py` writes them (for reference):

```rust
        apply: |p| {
            if p.terms[ANGA].has(Tag::Idit) {
                return false;
            }
            let Some(next) = p.terms[ANGA + 1..].iter().find(|t| !t.text.is_empty()) else {
                return false;
            };
            if !next.has(Tag::Ngit) {
                return false;
            }
            let chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let [head @ .., 'n', last] = chars.as_slice() else {
                return false;
            };
            if is_vowel(*last) {
                return false;
            }
            let text: String = head.iter().chain([last]).collect();
            let before = p.snapshot();
            p.terms[ANGA].text = text;
            p.record("6.4.24", "aniditAM hala upaDAyAH kNiti", before);
            true
        },
```

```rust
        apply: |p| {
            if !KSUBHNADI.contains(&p.ctx.dhatupatha) || !p.terms[ANGA].has(Tag::Svadi) {
                return false;
            }
            let before = p.snapshot();
            p.record("8.4.39", "kzuBnAdizu ca", before);
            true
        },
```

6.4.24 elides the nasal with a slice pattern, not by index arithmetic, so there is no `n - 2` for an index mutant to hit; its unit tests still carry a five-sound aṅga (`granT`), which would separate `n - 2` from `n / 2` if the code ever moves to an index. 8.4.39 reads `Tag::Svadi` as "śnu follows the aṅga": śnu is svādi's only vikaraṇa (3.1.73) and is never emptied, and kryādi 9d widens the test when `09.0055 kzuBa~` (before śnā) joins `KSUBHNADI`.

- [ ] **Step 4: Run the tests**

```bash
mise exec -- cargo test -q -p panini-prakriya 2>&1 | grep -E "test result|FAILED|panicked" | grep -v " 0 passed"
mise exec -- cargo test -q -p panini-data 2>&1 | grep -E "test result|FAILED|panicked" | grep -v " 0 passed"
mise run lint
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms each. Expected: `panini-prakriya` 455 passed, `panini-data` 30 passed, lint clean, and the full suite green with `trace` 217 and `paradigm` 31 (no row reaches either rule yet; Task 1's dump is the proof that no prior branch moves).

- [ ] **Step 5: Commit**

```bash
git branch --show-current      # svadi-5b
git add -A
git commit -m "feat(engine): 6.4.24 aniditAM hala upaDAyAH kNiti, general; 8.4.39 kzuBnAdizu ca

6.4.24 elides an anidit root's nasal upadhā before the next non-empty
term when it is ṅit: śnu, śnā, śyan, śa, or a luk'd or ślu'd gaṇa's ending.
Its aniditām is Tag::Idit, set by derive from panini_data::IDIT (the 87
curated idit rows, held to the vendored upadeśa both ways), since 7.1.58's
num is stored, not derived. 8.4.39 bars 8.4.1 and 8.4.2 for 05.0028 (√tṛp)
before śnu, keyed by row (KSUBHNADI) because curādi's 10.0351 and 10.0355
store the same tfp. No prior branch moves (the gate's dump of every prior
branch is byte-identical); the rows that need the rules land next."
```

---

## Task 3: The thirty-two rows, their goldens, and every assertion they move

**Files:**
- Modify: `crates/panini-data/src/lib.rs`, `crates/panini-prakriya/src/tinanta/samjna.rs`
- Modify: `crates/panini/tests/trace/svadi.rs`, `crates/panini/tests/paradigm/main.rs`, `crates/panini/tests/paradigm/data/svadi.rs`

**Interfaces:**
- Consumes: Task 2's rules and tests; `credited(&str) -> Vec<(&'static str, Gana)>` and `trace_for(&str) -> Vec<String>` in `tests/trace/helpers.rs` (`credited` yields one entry per live branch crediting the id, in `dhatus()` order); `Panini::check(&str) -> CheckResult { verdict, analyses }`, each `Analysis { dhatu: String, pada: Pada, trace: Vec<RuleStep>, .. }`, analyses in `dhatus()` order.
- Produces: the thirty-two `Dhatu` rows; the tests `svadi_rows_are_the_thirty_eight_curated_roots`, `svadi_analyses_its_5b_forms`, `anidit_nalopa_and_ksubhnadi_fire_only_on_their_rows` and the four trace pins.

- [ ] **Step 1: The assertions (failing)**

Create `/home/dev/vidyut-svadi-5b/slice5b/assertions_5b.py` if it is missing (sha256 `57ee863a5dc283db600f312801abf5d6991439874b833098468455ebffba75f2`). By file:
- **`panini-data`:** the row count (626); `Dhatu::pada`'s doc (134 of 626 verdicts re-derived; 626 roots); `pada_from_upadesha`'s doc (90 of the 626 carry a `\`, 69 on a root vowel); `svadi_rows_are_the_thirty_eight_curated_roots`, new, with the full upstream comparison. The spec calls this test "renamed"; no svādi row-list test existed, so it is added beside the rudhādi one.
- **`samjna.rs`:** `cisphur_is_the_ciy_row_6_1_54_names` reworded, not weakened: `05.0005` is now curated and outside `CISPHUR`, and none of its branches credits 6.1.54. `ksubhnadi_is_the_row_8_4_39_names` asserts `05.0028` is curated.
- **`trace/svadi.rs`:** *daBnoti* (6.4.24 between 1.2.4 and 7.3.84), *tfpnoti* (8.4.39, no ṇatva), *sunute* (1.3.72), *fkziRoti* (8.4.2, not 8.4.1: the root's `i` stands between `z` and `n`), each the whole trace; and `anidit_nalopa_and_ksubhnadi_fire_only_on_their_rows`.
- **`paradigm/main.rs`:** the census and the svādi 5b paragraph; `svadi_analyses_its_5b_forms`.

```python
#!/usr/bin/env python3
"""THROWAWAY: svādi 5b, Task 3's assertions — every count, list, trace pin,
roster and check() assertion the thirty-two svādi rows move, before the rows
exist. Every `old` must occur exactly once in its file; nothing is written if
one fails. Run from the worktree root."""
import sys
E = {
'crates/panini-data/src/lib.rs': [
("""    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 594
""",
"""    /// `curated_pada_agrees_with_upadesha_markers` re-derives 134 of these 626
"""),
("""    /// The test covers the 594 roots curated here, not the dhātupāṭha's 2259.
""",
"""    /// The test covers the 626 roots curated here, not the dhātupāṭha's 2259.
"""),
("""        assert_eq!(dhatus().len(), 594);
""",
"""        assert_eq!(dhatus().len(), 626);
"""),
("""    /// vendored upadeśa: 73 of the 594 curated roots carry a `\\` at all, and 52
""",
"""    /// vendored upadeśa: 90 of the 626 curated roots carry a `\\` at all, and 69
"""),
("""    #[test]
    fn rudhadi_rows_are_the_twenty_five_curated_roots() {
""",
"""    #[test]
    fn svadi_rows_are_the_thirty_eight_curated_roots() {
        // Svādi opened with √āp, √śak, √hi and √ri, parasmaipadī, and √aś
        // (`05.0020`) and √ṣṭigh, ātmanepadī. Svādi 5b curates the other
        // thirty-two and closes the gaṇa at all 38 of its dhātupāṭha rows:
        // the ten ñit rows, ubhayapadī by 1.3.72, then twenty-two
        // parasmaipadī by 1.3.78. Thirty derive on rules already in the
        // pipeline; √dambh needs 6.4.24 aniditAM hala upaDAyAH kNiti and
        // √tṛp 8.4.39 kzuBnAdizu ca.
        let rows: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.gana == Gana::Svadi)
            .map(|d| (d.dhatupatha, d.code, d.pada))
            .collect();
        assert_eq!(
            rows,
            vec![
                ("05.0016", "Ap", PadaAssignment::Parasmaipada),
                ("05.0017", "Sak", PadaAssignment::Parasmaipada),
                ("05.0012", "hi", PadaAssignment::Parasmaipada),
                ("05.0032", "ri", PadaAssignment::Parasmaipada),
                ("05.0020", "aS", PadaAssignment::Atmanepada),
                ("05.0021", "stiG", PadaAssignment::Atmanepada),
                ("05.0001", "su", PadaAssignment::Ubhayapada),
                ("05.0002", "si", PadaAssignment::Ubhayapada),
                ("05.0003", "Si", PadaAssignment::Ubhayapada),
                ("05.0004", "mi", PadaAssignment::Ubhayapada),
                ("05.0005", "ci", PadaAssignment::Ubhayapada),
                ("05.0006", "stf", PadaAssignment::Ubhayapada),
                ("05.0007", "kf", PadaAssignment::Ubhayapada),
                ("05.0008", "vf", PadaAssignment::Ubhayapada),
                ("05.0009", "Du", PadaAssignment::Ubhayapada),
                ("05.0010", "DU", PadaAssignment::Ubhayapada),
                ("05.0011", "du", PadaAssignment::Parasmaipada),
                ("05.0013", "pf", PadaAssignment::Parasmaipada),
                ("05.0014", "spf", PadaAssignment::Parasmaipada),
                ("05.0015", "smf", PadaAssignment::Parasmaipada),
                ("05.0018", "rAD", PadaAssignment::Parasmaipada),
                ("05.0019", "sAD", PadaAssignment::Parasmaipada),
                ("05.0022", "tik", PadaAssignment::Parasmaipada),
                ("05.0023", "tig", PadaAssignment::Parasmaipada),
                ("05.0024", "saG", PadaAssignment::Parasmaipada),
                ("05.0025", "Dfz", PadaAssignment::Parasmaipada),
                ("05.0026", "danB", PadaAssignment::Parasmaipada),
                ("05.0027", "fD", PadaAssignment::Parasmaipada),
                ("05.0028", "tfp", PadaAssignment::Parasmaipada),
                ("05.0029", "ah", PadaAssignment::Parasmaipada),
                ("05.0030", "daG", PadaAssignment::Parasmaipada),
                ("05.0031", "cam", PadaAssignment::Parasmaipada),
                ("05.0033", "kzi", PadaAssignment::Parasmaipada),
                ("05.0034", "ciri", PadaAssignment::Parasmaipada),
                ("05.0035", "jiri", PadaAssignment::Parasmaipada),
                ("05.0036", "dAS", PadaAssignment::Parasmaipada),
                ("05.0037", "df", PadaAssignment::Parasmaipada),
                ("05.0038", "fkzi", PadaAssignment::Parasmaipada),
            ]
        );
        // Complete: every svādi row upstream is curated.
        let mut ours: Vec<&str> = rows.iter().map(|(n, _, _)| *n).collect();
        ours.sort_unstable();
        let upstream: Vec<&str> = upstream_rows()
            .iter()
            .map(|(n, _, _)| *n)
            .filter(|n| n.starts_with("05."))
            .collect();
        assert_eq!(ours, upstream);
    }

    #[test]
    fn rudhadi_rows_are_the_twenty_five_curated_roots() {
"""),
],
'crates/panini-prakriya/src/tinanta/samjna.rs': [
("""    #[test]
    fn cisphur_is_the_ciy_row_6_1_54_names() {
        // 6.1.54 cisphuror ṇau. vidyut-prakriya keys it on the upadeśas `ciY`
        // and `ci\\Y`, and on `sPura~`. Curādi `10.0325 ci` stores as the
        // same `ci` and is not √ci of 6.1.54; svādi `ci\\Y` and tudādi
        // `sPura~` meet ṇic only in a causative.
        assert_eq!(CISPHUR, ["10.0124"]);
        assert_eq!(upstream_upadesha("10.0124"), Some("ciY"));
        assert!(dhatus().iter().any(|d| d.dhatupatha == "10.0124"));
        assert_eq!(upstream_upadesha("10.0325"), Some("ci"));
        for (number, upadesha) in [("05.0005", "ci\\\\Y"), ("06.0121", "sPura~")] {
            assert_eq!(upstream_upadesha(number), Some(upadesha), "{number}");
            assert!(!dhatus().iter().any(|d| d.dhatupatha == number), "{number}");
        }
    }
""",
"""    #[test]
    fn cisphur_is_the_ciy_row_6_1_54_names() {
        // 6.1.54 cisphuror ṇau. vidyut-prakriya keys it on the upadeśas `ciY`
        // and `ci\\Y`, and on `sPura~`. Curādi `10.0325 ci` stores as the
        // same `ci` and is not √ci of 6.1.54. Svādi `05.0005 ci\\Y` is that
        // √ci, curated since svādi 5b, but it meets ṇic only in a causative:
        // its śnu forms carry no 6.1.54 credit. Tudādi `sPura~` is uncurated.
        use crate::tinanta::derive;
        assert_eq!(CISPHUR, ["10.0124"]);
        assert_eq!(upstream_upadesha("10.0124"), Some("ciY"));
        assert!(dhatus().iter().any(|d| d.dhatupatha == "10.0124"));
        assert_eq!(upstream_upadesha("10.0325"), Some("ci"));
        assert_eq!(upstream_upadesha("05.0005"), Some("ci\\\\Y"));
        let ci = dhatus()
            .iter()
            .find(|d| d.dhatupatha == "05.0005")
            .expect("svādi √ci is curated");
        assert!(!CISPHUR.contains(&ci.dhatupatha));
        for &pada in ci.padas() {
            for lakara in [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin] {
                for purusha in [Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama] {
                    for vacana in [Vacana::Eka, Vacana::Dvi, Vacana::Bahu] {
                        for p in derive(ci, lakara, pada, purusha, vacana) {
                            assert!(!p.log.iter().any(|s| s.sutra == "6.1.54"), "{}", p.text());
                        }
                    }
                }
            }
        }
        assert_eq!(upstream_upadesha("06.0121"), Some("sPura~"));
        assert!(!dhatus().iter().any(|d| d.dhatupatha == "06.0121"));
    }
"""),
("""        assert_eq!(KSUBHNADI, ["05.0028"]);
""",
"""        assert_eq!(KSUBHNADI, ["05.0028"]);
        assert!(dhatus().iter().any(|d| d.dhatupatha == "05.0028"));
"""),
],
'crates/panini/tests/trace/svadi.rs': [
("""use crate::helpers::trace_for;
""",
"""use crate::helpers::{credited, trace_for};
"""),
("""#[test]
fn apnutat_trace_shows_tatan_blocking_the_vikarana_guna() {
""",
"""#[test]
fn dabhnoti_trace_pins_6_4_24_before_the_vikarana_guna() {
    // danB prathama eka (svādi 5b). śnu is ṅit by 1.2.4's second
    // application, so 6.4.24 aniditAM hala upaDAyAH kNiti elides the root's
    // nasal upadhā, and only then does the second 7.3.84 guṇate śnu's `u`:
    // vidyut-prakriya's order.
    assert_eq!(
        trace_for("daBnoti"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.1.73", "1.3.9", "1.2.4", "6.4.24", "7.3.84"
        ]
    );
}

#[test]
fn tfpnoti_trace_credits_8_4_39_and_no_natva() {
    // tfp prathama eka (svādi 5b). The `f` would reach śnu's `n` across the
    // pu-varga `p` by 8.4.2 (*tfpRoti*); 8.4.39 kzuBnAdizu ca records and
    // bars both ṇatva rules, so neither appears.
    assert_eq!(
        trace_for("tfpnoti"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.1.73", "1.3.9", "1.2.4", "7.3.84", "8.4.39"
        ]
    );
}

#[test]
fn sunute_trace_is_the_nit_atmanepada_of_1_3_72() {
    // su prathama eka, ātmanepada (svādi 5b): `zu\\Y`'s ñ it makes the root
    // ubhayapadī, so 1.3.72 svaritaYitaH sanctions the ātmanepada, and the
    // apit `ta` leaves śnu unguṇated.
    assert_eq!(
        trace_for("sunute"),
        vec![
            "1.3.72", "3.4.78", "1.2.4", "3.4.79", "3.1.73", "1.3.9", "1.2.4"
        ]
    );
}

#[test]
fn fkziroti_trace_takes_natva_across_its_i_by_8_4_2() {
    // fkzi prathama eka (svādi 5b). The trigger `z` and śnu's `n` are not
    // adjacent — the root's own `i` stands between them — so 8.4.2
    // awkupvANnumvyavAye'pi is credited, not 8.4.1.
    assert_eq!(
        trace_for("fkziRoti"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.1.73", "1.3.9", "1.2.4", "7.3.84", "8.4.2"
        ]
    );
}

#[test]
fn anidit_nalopa_and_ksubhnadi_fire_only_on_their_rows() {
    // Goldens ignore traces, so this holds svādi 5b's two rules to the rows
    // they were added for, branch for branch: 6.4.24 on every live branch
    // of √dambh's, 8.4.39 on every one of √tṛp's (36 cells and 6 alternates
    // each), and nowhere else in the corpus. Kryādi 9e's ten 6.4.24 rows
    // widen the first.
    for (sutra, row) in [("6.4.24", "05.0026"), ("8.4.39", "05.0028")] {
        let got = credited(sutra);
        assert!(got.iter().all(|(n, _)| *n == row), "{sutra}: {got:?}");
        assert_eq!(got.len(), 42, "{sutra}");
    }
}

#[test]
fn apnutat_trace_shows_tatan_blocking_the_vikarana_guna() {
"""),
],
'crates/panini/tests/paradigm/main.rs': [
("""/// Slice 10m curates √picc, `Nic`, which forks exactly where √cur does, once
/// 8.2.30 stopped velarising its root-internal `cc`. 72 new cells, 6 new
/// rows. The gaṇa is OPEN at 491 of its 492 rows.
/// This test is what keeps the numbers true day to day.
""",
"""/// Slice 10m curates √picc, `Nic`, which forks exactly where √cur does, once
/// 8.2.30 stopped velarising its root-internal `cc`. 72 new cells, 6 new
/// rows. The gaṇa is OPEN at 491 of its 492 rows.
///
/// Svādi 5b curates the gaṇa's other thirty-two rows, ten ubhayapadī by 1.3.72
/// and twenty-two parasmaipadī, which fork exactly where √āp does: laṅ and
/// vidhiliṅ parasmaipada prathama eka on 8.4.56, the two loṭ tātaṅ cells
/// three ways on 7.1.35/8.4.56, and the nineteen whose śnu is asaṁyogapūrva
/// on 6.4.107, four cells per pada, as √hi and √ri do. 1512 new cells, 308
/// new rows. Svādi is complete at 38 of its 38 rows.
/// This test is what keeps the numbers true day to day.
"""),
("""    assert_eq!(total_cells, 38232, "4248 root×lakāra blocks × 9 cells each");
""",
"""    assert_eq!(total_cells, 39744, "4416 root×lakāra blocks × 9 cells each");
"""),
("""    assert_eq!(ones, 29768, "one-form cells");
    assert_eq!(twos, 6790, "two-form cells");
    assert_eq!(
        threes, 927,
""",
"""    assert_eq!(ones, 31036, "one-form cells");
    assert_eq!(twos, 6970, "two-form cells");
    assert_eq!(
        threes, 991,
"""),
("""         slice 10l — seven of its eight rows' the same way; and — new in slice 10m — √picc's"
""",
"""         slice 10l — seven of its eight rows' the same way; and — new in slice 10m — √picc's; \\
         and — new in svādi 5b — its thirty-two rows', the same way"
"""),
("""    assert_eq!(ALTERNATES.len(), 11672, "ALTERNATES row count");
""",
"""    assert_eq!(ALTERNATES.len(), 11980, "ALTERNATES row count");
"""),
("""    assert_eq!(key_count("8.4.56"), 898, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 890, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 890, "7.1.35+8.4.56 alternates");
    assert_eq!(key_count("3.4.111"), 2, "3.4.111 alternates");
    assert_eq!(key_count("6.4.107"), 72, "6.4.107 alternates");
""",
"""    assert_eq!(key_count("8.4.56"), 962, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 954, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 954, "7.1.35+8.4.56 alternates");
    assert_eq!(key_count("3.4.111"), 2, "3.4.111 alternates");
    assert_eq!(key_count("6.4.107"), 188, "6.4.107 alternates");
"""),
("""    for form in ["pikcayati", "pikcayate", "peccayati"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
""",
"""    for form in ["pikcayati", "pikcayate", "peccayati"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}

/// Svādi 5b's `check()` witnesses, every analysis count read off the goldens
/// first: √dambh's and √tṛp's laṭ prathama eka with their rules credited,
/// √su's ātmanepada by 1.3.72, and √ṛkṣ's ṇatva across its `i`, one analysis
/// each; then the two homograph shapes the slice brings. `05.0033 kzi`'s
/// every form is one of tanādi `08.0004 kziR`'s (*kziRoti*), and each ñit
/// row's loṭ ātmanepada prathama eka is its parasmaipada prathama dvi
/// (*sunutAm*). The forms the two rules forbid derive nothing.
#[test]
fn svadi_analyses_its_5b_forms() {
    let engine = Panini::new();
    for (form, dhatu, pada, credits, absent) in [
        (
            "daBnoti",
            "danB",
            Pada::Parasmaipada,
            &["6.4.24"][..],
            &[][..],
        ),
        (
            "tfpnoti",
            "tfp",
            Pada::Parasmaipada,
            &["8.4.39"][..],
            &["8.4.1", "8.4.2"][..],
        ),
        (
            "sunute",
            "su",
            Pada::Atmanepada,
            &["1.3.72"][..],
            &["1.3.78"][..],
        ),
        (
            "fkziRoti",
            "fkzi",
            Pada::Parasmaipada,
            &["8.4.2"][..],
            &["8.4.1"][..],
        ),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), 1, "{form}");
        let a = &r.analyses[0];
        assert_eq!(a.dhatu, dhatu, "{form}");
        assert_eq!(a.pada, pada, "{form}");
        let ids: Vec<&str> = a.trace.iter().map(|s| s.sutra.as_str()).collect();
        for id in credits {
            assert!(ids.contains(id), "{form}: {ids:?}");
        }
        for id in absent {
            assert!(!ids.contains(id), "{form}: {ids:?}");
        }
    }
    for (form, readings) in [
        (
            "kziRoti",
            [("kzi", Pada::Parasmaipada), ("kziR", Pada::Parasmaipada)],
        ),
        (
            "sunutAm",
            [("su", Pada::Atmanepada), ("su", Pada::Parasmaipada)],
        ),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        let got: Vec<(&str, Pada)> = r
            .analyses
            .iter()
            .map(|a| (a.dhatu.as_str(), a.pada))
            .collect();
        assert_eq!(got, readings, "{form}");
    }
    for form in ["danBnoti", "tfpRoti", "tfpRuvanti"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
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
python3 /home/dev/vidyut-svadi-5b/slice5b/assertions_5b.py      # applied 16 edits to 4 files
mise run fmt
```

- [ ] **Step 2: Run them to see them fail**

```bash
mise exec -- cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .* FAILED|test result: FAILED"
```

Foreground, timeout 600000 ms (detached fallback per Global Constraints). Expected: exactly eleven failures:
- `paradigm`: `derivation_set_shape_matches_the_audited_numbers`, `svadi_analyses_its_5b_forms`
- `trace`: `svadi::dabhnoti_trace_pins_6_4_24_before_the_vikarana_guna`, `svadi::tfpnoti_trace_credits_8_4_39_and_no_natva`, `svadi::sunute_trace_is_the_nit_atmanepada_of_1_3_72`, `svadi::fkziroti_trace_takes_natva_across_its_i_by_8_4_2`, `svadi::anidit_nalopa_and_ksubhnadi_fire_only_on_their_rows`
- `panini-data`: `tests::curated_roots_have_expected_ganas_and_padas`, `tests::svadi_rows_are_the_thirty_eight_curated_roots`
- `panini-prakriya`: `tinanta::samjna::tests::ksubhnadi_is_the_row_8_4_39_names`, `tinanta::samjna::tests::cisphur_is_the_ciy_row_6_1_54_names`

- [ ] **Step 3: The rows**

```bash
python3 /home/dev/vidyut-svadi-5b/slice5b/rows_5b.py      # inserted 32 rows (the script is in Task 1 Step 3)
```

- [ ] **Step 4: Generate the goldens**

Create `/home/dev/vidyut-svadi-5b/vidyut-prakriya/examples/svadi_goldens_5b.rs` if it is missing (sha256 `48a4dc9995f41b2798f97c2a686bda31c0c581a5afc597cbe458c11aa53313f9`). It is 10m's generator with the row selection changed and its paths taken from the copy it is built in: it derives every cell of the thirty-two rows in both engines, asserts the form sets equal, and writes the rows the statics want (pinned form = first live branch; each further distinct form an `ALTERNATES` row keyed by its branch's vikalpa ids in log order).

```rust
//! THROWAWAY: slice 5b — emit the thirty-two svādi rows' goldens from the
//! engine, asserting every cell's form set equals vidyut's first.
use panini::Panini;
use panini_data::{dhatus, Lakara as L, Pada, Purusha as P, Vacana as V};
use vidyut_prakriya::args::{DhatuPada, Lakara, Prayoga, Purusha, Tinanta, Vacana};
use vidyut_prakriya::{Dhatupatha, Vyakarana};
const VIKALPA_RULES: &[&str] = &[
    "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3", "2573.2", "6.1.54", "7.3.37.2", "7.1.35", "3.4.111", "7.3.86", "6.4.107",
    "8.2.74", "8.2.75", "8.4.65", "8.4.56", "6.4.115", "6.4.117", "6.4.116", "6.4.43",
];
/// The new rows, in `DHATUS` order.
const NEW: [&str; 32] = [
    "05.0001", "05.0002", "05.0003", "05.0004", "05.0005", "05.0006", "05.0007", "05.0008",
    "05.0009", "05.0010", "05.0011", "05.0013", "05.0014", "05.0015", "05.0018", "05.0019",
    "05.0022", "05.0023", "05.0024", "05.0025", "05.0026", "05.0027", "05.0028", "05.0029",
    "05.0030", "05.0031", "05.0033", "05.0034", "05.0035", "05.0036", "05.0037", "05.0038",
];
fn main() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let dp = Dhatupatha::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/data/dhatupatha.tsv")).unwrap();
    let v = Vyakarana::builder().build();
    let e = Panini::new();
    let laks = [(L::Lat, Lakara::Lat, "laT"), (L::Lan, Lakara::Lan, "laN"), (L::Lot, Lakara::Lot, "loT"), (L::VidhiLin, Lakara::VidhiLin, "viDiliN")];
    let cells = [(P::Prathama, Purusha::Prathama, V::Eka, Vacana::Eka), (P::Prathama, Purusha::Prathama, V::Dvi, Vacana::Dvi), (P::Prathama, Purusha::Prathama, V::Bahu, Vacana::Bahu),
                 (P::Madhyama, Purusha::Madhyama, V::Eka, Vacana::Eka), (P::Madhyama, Purusha::Madhyama, V::Dvi, Vacana::Dvi), (P::Madhyama, Purusha::Madhyama, V::Bahu, Vacana::Bahu),
                 (P::Uttama, Purusha::Uttama, V::Eka, Vacana::Eka), (P::Uttama, Purusha::Uttama, V::Dvi, Vacana::Dvi), (P::Uttama, Purusha::Uttama, V::Bahu, Vacana::Bahu)];
    let (mut par, mut alt) = (String::new(), String::new());
    let (mut ncells, mut nforms, mut ndiff) = (0, 0, 0);
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
                    // The pinned form is the first LIVE branch.
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
    std::fs::write(format!("{root}/goldens_5b_paradigm.rs"), par).unwrap();
    std::fs::write(format!("{root}/goldens_5b_alternates.rs"), alt).unwrap();
    println!("{ncells} cells, {nforms} forms, {ndiff} differences");
    assert_eq!(ndiff, 0);
}
```

```bash
bash /home/dev/vidyut-svadi-5b/slice5b/repoint_5b.sh .     # both lines must name /workspace/.worktrees/svadi-5b/crates
(cd /home/dev/vidyut-svadi-5b/vidyut-prakriya && mise exec rust@1.99.0 -- cargo run -q --release --example svadi_goldens_5b 2>/dev/null | tail -1)
sha256sum /home/dev/vidyut-svadi-5b/goldens_5b_paradigm.rs /home/dev/vidyut-svadi-5b/goldens_5b_alternates.rs
```

Expected: `1512 cells, 1820 forms, 0 differences`, then:
- `905f402f335a365b1ba35f18bdd54f6e7fbeec9165b1fc645db0ece46713b6a3  /home/dev/vidyut-svadi-5b/goldens_5b_paradigm.rs`
- `96511085aa219494a4cf2bfb119ef1f86bb4c24ec987614f3bcd70bee6c424c4  /home/dev/vidyut-svadi-5b/goldens_5b_alternates.rs`

If a line differs, stop and report.

- [ ] **Step 5: Insert the goldens and check they landed verbatim**

Create `/home/dev/vidyut-svadi-5b/slice5b/insert_goldens_5b.py` (sha256 `85e852e88cf905cea2a0a6f0ddf07e397d5b80ad7951187666bc025fabebb7e2`) and `/home/dev/vidyut-svadi-5b/slice5b/check_goldens_5b.py` (sha256 `4802e80285f14ce8f92b46860c619e6bcd749a8a5696c06373406ec1975fc7f7`) if they are missing:

```python
#!/usr/bin/env python3
"""THROWAWAY: svādi 5b — insert the generator's goldens before each static's
`];` in svadi.rs. Reads them from $V5B (the private vidyut copy). Run from the
worktree root."""
import os
v = os.environ['V5B']
p = 'crates/panini/tests/paradigm/data/svadi.rs'
s = open(p).read()
par = open(f'{v}/goldens_5b_paradigm.rs').read()
alt = open(f'{v}/goldens_5b_alternates.rs').read()
i = s.index('pub const ALTERNATES')
head, tail = s[:i], s[i:]
k = head.rindex('];'); head = head[:k] + par + head[k:]
k = tail.rindex('];'); tail = tail[:k] + alt + tail[k:]
open(p, 'w').write(head + tail)
print("inserted goldens")
```

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 5b — confirm every generated golden tuple landed in
svadi.rs verbatim, modulo rustfmt (whitespace, and the trailing comma it
drops before a closing parenthesis). Reads $V5B's generator output. Run from
the worktree root."""
import os, re
v = os.environ['V5B']
norm = lambda t: re.sub(r',\)', ')', re.sub(r'\s+', '', t))
have = norm(open('crates/panini/tests/paradigm/data/svadi.rs').read())
total = 0
for name in ('paradigm', 'alternates'):
    src = norm(open(f'{v}/goldens_5b_{name}.rs').read())
    tuples = re.findall(r'\("0[^()]*(?:\[[^\]]*\])?[^()]*\)', src)
    missing = [t for t in tuples if t not in have]
    total += len(tuples)
    print(name, len(tuples), "missing", len(missing))
    assert not missing, missing[:3]
print("all", total, "landed")
```

```bash
V5B=/home/dev/vidyut-svadi-5b python3 /home/dev/vidyut-svadi-5b/slice5b/insert_goldens_5b.py   # inserted goldens
mise run fmt
V5B=/home/dev/vidyut-svadi-5b python3 /home/dev/vidyut-svadi-5b/slice5b/check_goldens_5b.py    # paradigm 168 missing 0 / alternates 308 missing 0 / all 476 landed
```

`check_goldens_5b.py` compares modulo rustfmt (whitespace, and the `,)` it rewrites to `)`); a raw text comparison reports false missing tuples.

- [ ] **Step 6: The measured pada-ambiguous set**

The set is measured, never hand-picked: run the test against the old set and read the real one off its failure. Create `/home/dev/vidyut-svadi-5b/slice5b/pin_ambiguous_5b.py` if it is missing (sha256 `d40de2644a613fa8a27a924769211b5e88625d64da1f1310b97848dca9eca007`). It refuses a set whose size or hash differs from the replay's.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 5b — pin the measured pada-ambiguous set ($SET, the
failing test's `left:` JSON) into pada_ambiguous_surfaces_are_exactly_these,
and extend the comment that accounts for it. Run from the worktree root."""
import hashlib, json, os
amb = json.load(open(os.environ['SET']))
assert len(amb) == 1655, len(amb)
h = hashlib.sha256('\n'.join(amb).encode()).hexdigest()
assert h == 'f3dbcc32a2563c21d5475cf59d1b5e150766f13abc6eb060102895bc43510a67', h
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
old = """    // `piccayatAm`, `piccayetAm`, `piccayeta`), taking the set from 1631 to
    // 1635.
"""
assert s.count(old) == 1
s = s.replace(old, old + """    // Svādi 5b's ten ñit rows contribute two each, the tanādi shape: laṅ
    // ātmanepada prathama eka = parasmaipada madhyama bahu (`asunuta`) and
    // loṭ ātmanepada prathama eka = parasmaipada prathama dvi (`sunutAm`).
    // Its twenty-two parasmaipadī rows add none: `05.0033 kzi`'s surfaces
    // are tanādi `08.0004 kziR`'s parasmaipada ones, already in the set
    // where they meet its ātmanepada. 20 more, taking the set from 1635 to
    // 1655.
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
SET="$SET" python3 /home/dev/vidyut-svadi-5b/slice5b/pin_ambiguous_5b.py      # pinned 1655
mise run fmt
```

The twenty new surfaces are the ten ñit rows' `a-…-uta` / `…-utAm` pairs: `asunuta`/`sunutAm`, `asinuta`/`sinutAm`, `aSinuta`/`SinutAm`, `aminuta`/`minutAm`, `acinuta`/`cinutAm`, `astfRuta`/`stfRutAm`, `akfRuta`/`kfRutAm`, `avfRuta`/`vfRutAm`, `aDunuta`/`DunutAm`, `aDUnuta`/`DUnutAm`.

- [ ] **Step 7: Run the full suite, and check the homographs against the goldens**

```bash
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms (detached fallback per Global Constraints). Expected: PASS at 39744 cells, with `panini-prakriya` 455, `trace` 222, `paradigm` 32, `panini-data` 31.

The `check()` witnesses' analysis counts must match the goldens, which the test also enforces:

```bash
for f in daBnoti tfpnoti sunute fkziRoti kziRoti sunutAm danBnoti tfpRoti tfpRuvanti; do
  printf "%s: %s\n" $f "$(grep -c "\"$f\"" crates/panini/tests/paradigm/data/*.rs | grep -v ':0' | sed 's#.*/##' | tr '\n' ' ')"; done
```

Expected: `daBnoti`, `tfpnoti`, `sunute` and `fkziRoti` `svadi.rs:1`; `kziRoti` `svadi.rs:1 tanadi.rs:1`; `sunutAm` `svadi.rs:2` (its parasmaipada and ātmanepada rows); the three Invalid shapes print nothing.

Then derive the whole-corpus collision list from the goldens, not from a hand list:

```bash
python3 - <<'PY'
import re, glob, collections
def forms(text):
    out = []
    for m in re.finditer(r'\(\s*"([0-9.]+)",\s*"\w+",\s*Pada::\w+,\s*\[([^\]]*)\]', text):
        out += [(m.group(1), f) for f in re.findall(r'"([^"]+)"', m.group(2))]
    for m in re.finditer(r'\(\s*"([0-9.]+)",\s*"\w+",\s*Pada::\w+,\s*\d+,\s*"([^"]*)",\s*"[^"]*",?\s*\)', text):
        out.append((m.group(1), m.group(2)))
    return out
roots = collections.defaultdict(set)
for f in glob.glob('crates/panini/tests/paradigm/data/*.rs'):
    for n, form in forms(open(f).read()):
        roots[form].add(n)
hits = collections.Counter()
for form, ns in roots.items():
    new = {n for n in ns if n.startswith('05.') and n not in ('05.0012', '05.0016', '05.0017', '05.0020', '05.0021', '05.0032')}
    if new and len(ns) > 1:
        hits[tuple(sorted(ns))] += 1
print(dict(hits))
PY
```

Expected: `{('05.0033', '08.0004'): 44}` — every form of `05.0033 kzi` is one of tanādi `08.0004 kziR`'s, and no other new row shares a surface with any other root. `svadi_analyses_its_5b_forms` pins the pair on *kziRoti*, svādi's reading first.

- [ ] **Step 8: Commit**

```bash
mise run lint
git branch --show-current      # svadi-5b
git add -A
git commit -m "feat(data): svādi's last thirty-two rows — the gaṇa at 38 of 38

38232 → 39744 cells, 49904 → 51724 forms, ALTERNATES 11672 → 11980, 594 →
626 roots; pada-ambiguous surfaces 1635 → 1655. Ten ñit rows ubhayapadī by
1.3.72, twenty-two parasmaipadī; √dambh takes 6.4.24 (daBnoti) and √tṛp
8.4.39 (tfpnoti), each on its own row's branches only. Goldens generated
cell by cell equal to vidyut."
```

---

## Task 4: The audit, the final trace dump, and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`
- Modify: `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `docs/superpowers/specs/2026-07-29-svadi-gana-design.md`
- Modify: `crates/panini/tests/paradigm/main.rs` (doc counts, alternates doc, audit chain), `crates/panini/tests/trace/rudhadi.rs` (one comment), `crates/panini-prakriya/src/tinanta/guna.rs` (one comment)

**Interfaces:** Consumes the finished engine, data and goldens. Produces no symbols.

- [ ] **Step 1: The harness**

```bash
python3 /home/dev/vidyut-svadi-5b/slice5b/audit_5b.py      # applied 6 edits to 1 files (the script is in Task 1 Step 3)
```

The committed harness now asserts 626 / 39744 / 51724 and names 478 both-pada roots, thirty-five of them by 1.3.72.

- [ ] **Step 2: The final base-vs-branch dump and the audit**

```bash
S=/home/dev/vidyut-svadi-5b/slice5b; V=/home/dev/vidyut-svadi-5b/vidyut-prakriya; G=/home/dev/svadi-5b-gate
WT="$(git rev-parse --show-toplevel)"
bash $S/repoint_5b.sh "$WT"                        # both lines must name $WT/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_5b 2>/dev/null > $G/dump-final.txt)
wc -l < $G/dump-final.txt                          # 56456
cmp $G/dump-base.txt $G/dump-final.txt && echo PRIOR-TRACES-IDENTICAL
cp tools/audit/panini_full_audit.rs $V/examples/
(cd $V && PANINI_AUDIT_VIDYUT=/home/dev/vidyut-svadi-5b PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run -q --release --example panini_full_audit 2>&1 | tail -1)
(cd $V && PANINI_AUDIT_VIDYUT=/home/dev/vidyut-svadi-5b PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run -q --release --example panini_full_audit 2>&1 | tail -9)
date -u +%F
```

`dump-base.txt` is Task 1's, from `/home/dev/svadi-5b-base`; if it is missing, rebuild it first with `repoint_5b.sh /home/dev/svadi-5b-base` and the same `cargo run`, and check its hash (`ce7780cf…`). Expected:
- `56456`, then `PRIOR-TRACES-IDENTICAL`: the committed slice moves no prior branch.
- The `entry` control, run first: `AUDIT FAILED: 36 differing cells.`
- The honest run: `roots : 626`, `cells : 39744`, `forms (set sizes): 51724`, `live branches : 51724`, `blocked branches : 6552`, `differing cells  : 0`, `AUDIT PASSED: 39744 cells, 51724 forms, zero differences.`

Do not use `mise -C`. If the dump differs or the honest run shows differences, stop and report, and edit nothing. Write down the date the last command prints: it is the audit's date for Step 3.

- [ ] **Step 3: The doc sweep and the audit record**

Create `/home/dev/vidyut-svadi-5b/slice5b/docsweep_5b.py` if it is missing (sha256 `24caf54e6affd7d8fcfea7510c157822d2390ec0f8b7ef03696e492ce579245d`). It covers:
- **README:** svādi complete at 38 of 38 with 6.4.24 and 8.4.39 described; 626 roots; 8708 of 39744 multi-form cells (6970 two, 991 three); svādi 5b in the three-form list and its 180 two-form cells; 478 both-pada roots, thirty-five by 1.3.72 with the ten new ones named; 1655 pada-ambiguous surfaces and the twenty new ones' shape.
- **ARCHITECTURE:** 6.4.24 and 8.4.39 in the stage table; 168 pinned ids with svādi 5b's two; a svādi 5b paragraph after the gaṇa's, placing 6.4.24 beside 6.4.23 and naming `IDIT`, `Tag::Idit` and `KSUBHNADI`; 6.4.107's 188 cells across thirty roots (the old "√hi and √ri, the gaṇa's only asaṁyogapūrva roots" is no longer true); 7.1.35's 1104 cells across 552 parasmaipada columns; 478 both-pada roots; 552 + 74 = 626; 8.4.56's 962 = 939 + 22 + 1, with svādi 5b among its contributors; the tātaṅ fork's further 1104.
- **AGENTS.md:** 39744 cells; svādi closing at 38 of 38 in the gaṇa history; 6970 / 991; 11980 `ALTERNATES` and 51724 forms; the svādi paragraph, which called the gaṇa "complete — six roots"; the √und paragraph's "does not implement 6.4.24 at all"; the audit chain; the 6.4.107 ledger (72 cells as of 8b, 188 now); the 39744 goldens; 8.4.39 in the number-keyed-guards list and `Tag::Idit` beside `Tag::Ghu`; 8.4.39 as `Rule.bars`'s third user.
- **tools/audit/README.md:** the totals line and a svādi 5b "Last recorded result" entry.
- **paradigm/main.rs** docs: 11980; the 39744-cell census line; svādi 5b in the three-form list and its 180 two-form cells; the `ALTERNATES` key counts (962 / 954 / 954 / 188) and svādi 5b's fold; the audit chain.
- **guna.rs:** the 626-root corpus comment. **trace/rudhadi.rs:** `unantas_trace_orders_6_4_23_before_6_4_111`'s comment, which said this engine does not implement 6.4.24.
- **The svādi gaṇa spec** (`2026-07-29`): its out-of-scope ñit bullet points at this slice's spec, and its √stṛ / √kṛ bullet records that they needed neither 7.1.100 nor the kṛ specials.

```python
#!/usr/bin/env python3
"""THROWAWAY: svādi 5b's doc sweep — counts, svādi at 38 of 38, 6.4.24 and
8.4.39 where the docs list rules, the audit record and chain, the svādi
spec's pointer. Takes AUDIT_DATE (Task 4 Step 2's `date -u +%F`). Every
`old` must occur exactly once in its file; nothing is written if one fails.
Run from the worktree root."""
import os, re, sys
D = os.environ['AUDIT_DATE']
assert re.fullmatch(r'\d{4}-\d{2}-\d{2}', D), D
SVADI_NYIT = "√su, √si, √śi, √mi, √ci, √stṛ, √kṛ, √vṛ, √dhu and √dhū"
E = {
'README.md': [
("""(2, śap luk'd), *kryādi* (9, śnā), *svādi* (5, śnu) and *rudhādi* (7,
śnam) — plus""",
"""(2, śap luk'd), *kryādi* (9, śnā), *svādi* (5, śnu), **complete** at all
38 of its dhātupāṭha rows since svādi 5b curated the thirty-two beyond its
first six, behind 6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti* (√dambh's
*dabhnoti*, its *aniditām* read from the curated `IDIT` rows) and 8.4.39
*kṣubhnādiṣu ca* (√tṛp's *tṛpnoti*, kept free of ṇatva), and *rudhādi* (7,
śnam) — plus"""),
("curated 594-root set, in four lakāras",
 "curated 626-root set, in four lakāras"),
("""both correct — and in fact 8464 of the 38232 cells hold more than one form: 6790
hold two, 927 hold three""",
"""both correct — and in fact 8708 of the 39744 cells hold more than one form: 6970
hold two, 991 hold three"""),
("""new in slice 10m — √picc's, each by
""",
"""new in slice 10m — √picc's, and — new in svādi 5b — its thirty-two rows', each by
"""),
("""8.4.56 forks, and slice 10l's √dhras 32 two-form cells, a ṇic and a
ṇic-less reading each),
""",
"""8.4.56 forks, and slice 10l's √dhras 32 two-form cells, a ṇic and a
ṇic-less reading each, and svādi 5b's thirty-two rows 180 two-form cells,
64 on 8.4.56 and 116 on 6.4.107),
"""),
("""padas — 468 roots that admit both padas in the curated set
(twenty-five ubhayapadī by 1.3.72: √nī, √tud, √rudh, √bhid, √kṣud, √yuj,
√tṛd, √ric, √vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛṇ, √tṛ, √ghṛ,
√kṛ, √dā, √dhā, √bhṛ, √ṇij, √vij and √viṣ; √bhuj by 1.3.66;""",
f"""padas — 478 roots that admit both padas in the curated set
(thirty-five ubhayapadī by 1.3.72: √nī, √tud, √rudh, √bhid, √kṣud, √yuj,
√tṛd, √ric, √vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛṇ, √tṛ, √ghṛ,
√kṛ, √dā, √dhā, √bhṛ, √ṇij, √vij and √viṣ, and svādi 5b's {SVADI_NYIT};
√bhuj by 1.3.66;"""),
("1635 of the pinned (`PARADIGM`) surfaces are pada-ambiguous",
 "1655 of the pinned (`PARADIGM`) surfaces are pada-ambiguous"),
("""homograph `10.0143 cUrRa~` already supplies; slice 10m's √picc adds four
more. The
""",
"""homograph `10.0143 cUrRa~` already supplies; slice 10m's √picc adds four
more; and svādi 5b's ten ñit rows add twenty, two each in the tanādi shape
(`asunuta`, `sunutAm`). The
"""),
("set, all 1635. It is therefore",
 "set, all 1655. It is therefore"),
],
'docs/ARCHITECTURE.md': [
("| `anga.rs` | 6.4.71 … 6.1.73 … 7.1.4 … 7.2.81, 6.4.23 | after 3.1.68 |",
 "| `anga.rs` | 6.4.71 … 6.1.73 … 7.1.4 … 7.2.81, 6.4.23, 6.4.24 | after 3.1.68 |"),
("8.4.53, 8.4.54, 8.2.38, 8.4.55, 8.4.1, 8.4.2, 8.4.58, 8.4.65, 8.4.56 | after 3.1.68 |",
 "8.4.53, 8.4.54, 8.2.38, 8.4.55, 8.4.39, 8.4.1, 8.4.2, 8.4.58, 8.4.65, 8.4.56 | after 3.1.68 |"),
("pins all 166 ids verbatim (72 pre-rudhādi",
 "pins all 168 ids verbatim (72 pre-rudhādi"),
("""*upadhāyāṃ ca* after 8.2.77 — 166 total).
""",
"""*upadhāyāṃ ca* after 8.2.77 — 166 total — then svādi 5b's two: 6.4.24
*aniditāṁ hala upadhāyāḥ kṅiti* after 6.4.23, and 8.4.39 *kṣubhnādiṣu ca*
before 8.4.1 — 168 total).
"""),
("""*utaś ca pratyayād asaṁyogapūrvāt* splits `hinu` from `Apnuhi` at `hi`.
""",
"""*utaś ca pratyayād asaṁyogapūrvāt* splits `hinu` from `Apnuhi` at `hi`.

Svādi is **complete** at all 38 of its dhātupāṭha rows: svādi 5b curated the
thirty-two beyond the gaṇa slice's six, thirty on rules already in the
pipeline. √dambh (`05.0026 danBu~`) brought 6.4.24 *aniditāṁ hala upadhāyāḥ
kṅiti*, in `anga.rs` beside 6.4.23 *śnān nalopaḥ*: both elide a nasal, but
6.4.23 the root's own nasal that 3.1.78 has moved into `SHAP` behind śnam's
`na`, and 6.4.24 the penultimate `n` of a hal-final `ANGA` before a ṅit
affix, which it reads as the next non-empty term after `ANGA` (*dabhnoti*).
After 3.1.78's split no rudhādi `ANGA` is hal-final, so the two never meet
the same nasal. 6.4.24's *aniditām* is the data layer's `IDIT`, the curated
idit rows, set as `Tag::Idit` by `derive`: 7.1.58 *idito num dhātoḥ* is
stored, not derived, so an idit root's num already sits in its code (`hins`)
and only the upadeśa can tell it from a root's own nasal. √tṛp (`05.0028
tfpa~`) brought 8.4.39 *kṣubhnādiṣu ca*, keyed by row (`KSUBHNADI`) because
curādi's `10.0351` and `10.0355` are `tfpa~` too; it changes no text and bars
8.4.1 and 8.4.2 (*tṛpnoti*), the third rule to use `Rule.bars`.
"""),
("""*asaṁyogapūrva*, forking 72 cells across eleven roots: svādi's √hi and √ri
(the gaṇa's only asaṁyogapūrva roots) in laṭ and laṅ uttama dvi/bahu, whose""",
"""*asaṁyogapūrva*, forking 188 cells across thirty roots: svādi's √hi and √ri
(and, new in svādi 5b, the nineteen more of its roots whose śnu follows a
single non-conjunct sound) in laṭ and laṅ uttama dvi/bahu, whose"""),
("""svādi's `nu` alone. Eight of those 72 cells — the four ik-upadhā roots'""",
 """svādi's `nu` alone. Eight of those 188 cells — the four ik-upadhā roots'"""),
("""keyed `7.3.86+6.4.107` on top of the 72 keyed plain `6.4.107` (both counts""",
 """keyed `7.3.86+6.4.107` on top of the 188 keyed plain `6.4.107` (both counts"""),
("""gains a 6.4.107-keyed alternate — the 72-cell/eleven-root count above is
unchanged by √kṛ.""",
"""gains a 6.4.107-keyed alternate — the 188-cell/thirty-root count above is
unchanged by √kṛ."""),
("""8.2.39 obligatorily voices its final `t` to `d`), forking 1040 cells (loṭ
prathama and madhyama eka across the 520 roots with a parasmaipada column —""",
"""8.2.39 obligatorily voices its final `t` to `d`), forking 1104 cells (loṭ
prathama and madhyama eka across the 552 roots with a parasmaipada column —"""),
("""roots never reach this guard, and the 468 roots that admit both
padas (twenty-five ubhayapadī by 1.3.72 — √rudh, √nī, √tud, √bhid, √kṣud,
√yuj, √tṛd, √ric, √vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛṇ, √tṛ, √ghṛṇ,
√kṛ, √dā, √dhā, √bhṛ, √ṇij, √vij and √viṣ — √bhuj by 1.3.66,""",
f"""roots never reach this guard, and the 478 roots that admit both
padas (thirty-five ubhayapadī by 1.3.72 — √rudh, √nī, √tud, √bhid, √kṣud,
√yuj, √tṛd, √ric, √vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛṇ, √tṛ, √ghṛṇ,
√kṛ, √dā, √dhā, √bhṛ, √ṇij, √vij and √viṣ, and svādi 5b's {SVADI_NYIT}
— √bhuj by 1.3.66,"""),
("520 + 74 = the 594 curated roots",
 "552 + 74 = the 626 curated roots"),
("""utterance, forking 898 cells outright: laṅ and vidhiliṅ prathama eka across
those same 520 parasmaipada columns (875 of them""",
"""utterance, forking 962 cells outright: laṅ and vidhiliṅ prathama eka across
those same 552 parasmaipada columns (939 of them"""),
("√dhras on its ṇic branch, and 10m's √picc;",
 "√dhras on its ṇic branch, and 10m's √picc, and svādi 5b's thirty-two rows;"),
("""forking a further 1040 (the same
""",
"""forking a further 1104 (the same
"""),
],
'AGENTS.md': [
("(`crates/panini/tests/paradigm/`, 38232 cells, ten gaṇas, nine complete —",
 "(`crates/panini/tests/paradigm/`, 39744 cells, ten gaṇas, nine complete —"),
("at 491 after slice 10m curated √picc, narrowing 8.2.30 to a term-final cu, the one dhātu left being √ṣad (`10.0368`), which waits for upasargas —",
 "at 491 after slice 10m curated √picc, narrowing 8.2.30 to a term-final cu, the one dhātu left being √ṣad (`10.0368`), which waits for upasargas, and svādi (5) closing at 38 of 38 in svādi 5b, which curated the thirty-two rows beyond its first six behind 6.4.24 and 8.4.39 —"),
("other forms — a second (6790 cells), a third (927 cells), a fourth",
 "other forms — a second (6970 cells), a third (991 cells), a fourth"),
("`ALTERNATES` (11672 rows in all, so 38232 + 11672 = 49904 forms total); √bhuj",
 "`ALTERNATES` (11980 rows in all, so 39744 + 11980 = 51724 forms total); √bhuj"),
("""    (gaṇa 5) is now **complete** — six roots across all four lakāras: √āp,
    √śak, √hi and √ri (parasmaipada), √aś (`05.0020`, distinct from kryādi's
    `09.0059`) and √ṣṭigh (`stiG`) (ātmanepada). Its vikaraṇa is śnu (3.1.73),""",
"""    (gaṇa 5) is **complete** at all 38 of its dhātupāṭha rows — first six
    roots across all four lakāras: √āp, √śak, √hi and √ri (parasmaipada),
    √aś (`05.0020`, distinct from kryādi's `09.0059`) and √ṣṭigh (`stiG`)
    (ātmanepada); then, in svādi 5b, the other thirty-two, ten ubhayapadī by
    1.3.72 and twenty-two parasmaipadī, behind 6.4.24 *aniditāṁ hala
    upadhāyāḥ kṅiti* (√dambh's *dabhnoti*; its *aniditām* is `Tag::Idit`,
    from the data layer's `IDIT`) and 8.4.39 *kṣubhnādiṣu ca* (√tṛp's
    *tṛpnoti*, keyed by row). Its vikaraṇa is śnu (3.1.73),"""),
("""    credits 6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti* for √und's `unad → und`
    step; this engine rejects that credit, does not implement 6.4.24 at
    all, and pins the rejection in `tests/trace/`.)""",
"""    credits 6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti* for √und's `unad → und`
    step; this engine rejects that credit — its 6.4.24, landed in svādi 5b,
    never reaches a rudhādi aṅga, which 3.1.78 leaves vowel-final — and pins
    the rejection in `tests/trace/`.)"""),
("""(`tools/audit/README.md`'s 2026-10-06 10m
  entry, 38232 cells / 49904 forms / 594 roots).
""",
f"""(`tools/audit/README.md`'s 2026-10-06 10m
  entry, 38232 cells / 49904 forms / 594 roots), and that by svādi 5b's
  (`tools/audit/README.md`'s {D} svādi 5b entry, 39744 cells / 51724 forms /
  626 roots).
"""),
("""  add to this count — 6.4.108 empties its `u` first), so 6.4.107 now fires
  on **72**
  cells across eleven roots (`key_count("6.4.107") == 72`, the same
  test), not 8 — the "8 cells" figure was never re-derived when the gaṇa
  landed.""",
"""  add to this count — 6.4.108 empties its `u` first), so 6.4.107 fired
  on **72**
  cells across eleven roots as of tanādi 8b, and fires on **188** across
  thirty since svādi 5b's nineteen asaṁyogapūrva roots
  (`key_count("6.4.107") == 188`, the same test), not 8 — the "8 cells"
  figure was never re-derived when the gaṇa landed."""),
("only in the ordinary corpus-size sense, not wrong in kind: 38232 goldens",
 "only in the ordinary corpus-size sense, not wrong in kind: 39744 goldens"),
("""  `hA`, and 7.4.77 (`03.0004`, `03.0005`, `03.0017`) follows that precedent.""",
"""  `hA`, and 7.4.77 (`03.0004`, `03.0005`, `03.0017`) follows that precedent. 8.4.39 (`KSUBHNADI`: `05.0028`, svādi 5b) does too: curādi's `10.0351` and `10.0355` are also `tfpa~`, stored `tfp`."""),
("""  `Tag::Ghu` from `tinanta/samjna.rs`'s `GHU` — pinned to the vendored TSV
  rather than to a derivation, so its uncurated members are held too.""",
"""  `Tag::Ghu` from `tinanta/samjna.rs`'s `GHU` — pinned to the vendored TSV
  rather than to a derivation, so its uncurated members are held too. A
  verdict the upadeśa carries but the stored code cannot is decided the same
  way: 6.4.24's *aniditām* reads `Tag::Idit`, from `panini_data::IDIT`,
  which `idit_matches_upadesha_markers` holds to the vendored upadeśa in
  both directions."""),
("""  no text and bars 7.3.86, so 7.3.86's guard carries no abhyasta exception.
""",
"""  no text and bars 7.3.86, so 7.3.86's guard carries no abhyasta exception.
  8.4.39 *kṣubhnādiṣu ca* (svādi 5b) is the third: it changes no text and
  bars 8.4.1 and 8.4.2, so ṇatva's guards carry no √tṛp exception.
"""),
],
'tools/audit/README.md': [
("**It asserts the corpus totals** (594 roots, 38232 cells, 49904 forms) rather than",
 "**It asserts the corpus totals** (626 roots, 39744 cells, 51724 forms) rather than"),
("""## Last recorded result

2026-10-06, curādi 10m slice, vidyut
""",
f"""## Last recorded result

{D}, svādi 5b slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 39744
cells / 51724 forms / 626 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole svādi 5b slice: svādi's other thirty-two rows,
closing the gaṇa at 38 of 38, ten ubhayapadī by 1.3.72 and twenty-two
parasmaipadī by 1.3.78; 6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti*, general
(the next non-empty term after the aṅga ṅit, the root not idit by the
data layer's `IDIT`), for √dambh; and 8.4.39 *kṣubhnādiṣu ca*, keyed by
row, for √tṛp. Blocked branches stay at 6552. With the rows and neither
rule, 72 cells differed, √dambh's 36 and √tṛp's 36. A base-vs-branch dump
of every prior cell's branches, blocked ones included, with every step's
before and after text, was byte-identical, all 56456 of them (49904 live).

Totals: 626 = 594 + 32; 39744 = 38232 + 1512 (168 root×pada×lakāra blocks ×
9: ten rows × 2 padas × 4 lakāras, twenty-two × 4); 51724 = 49904 + 1512 +
308 new `ALTERNATES` rows (11672 → 11980), measured via the harness's
corpus block, not assumed.

2026-10-06, curādi 10m slice, vidyut
"""),
],
'crates/panini/tests/paradigm/main.rs': [
("/// `ALTERNATES` is otherwise 11672 bare strings, and a string can be right for\n",
 "/// `ALTERNATES` is otherwise 11980 bare strings, and a string can be right for\n"),
("/// 38232 cells total (4248 root×lakāra blocks × 9), of which 29768 hold exactly one form, 6790 hold two, 927 hold three",
 "/// 39744 cells total (4416 root×lakāra blocks × 9), of which 31036 hold exactly one form, 6970 hold two, 991 hold three"),
("""and √picc's,
/// new in slice 10m, each by
""",
"""and √picc's,
/// new in slice 10m, and svādi's thirty-two rows', new in svādi 5b, each by
"""),
("""/// cells as √cur's, and √ci 68 three-form ones, and slice 10l's √dhras 32
/// two-form cells, a ṇic and a ṇic-less reading each), 361 hold four""",
"""/// cells as √cur's, and √ci 68 three-form ones, and slice 10l's √dhras 32
/// two-form cells, a ṇic and a ṇic-less reading each, and svādi 5b's
/// thirty-two rows 180 two-form cells, 64 on 8.4.56 and 116 on 6.4.107), 361 hold four"""),
("""/// itself has 11672 rows, keyed 898 `8.4.56`, 890 `7.1.35`, 890 `7.1.35+8.4.56`,
/// 2 `3.4.111`, 72 `6.4.107`,""",
"""/// itself has 11980 rows, keyed 962 `8.4.56`, 954 `7.1.35`, 954 `7.1.35+8.4.56`,
/// 2 `3.4.111`, 188 `6.4.107`,"""),
("""/// (√kṛp, its guṇa before ṇic); slice 10m's √picc opens none and folds 2 rows
/// apiece into `8.4.56`, `7.1.35` and `7.1.35+8.4.56` — √kṛ (slice 8b) adds six more
""",
"""/// (√kṛp, its guṇa before ṇic); slice 10m's √picc opens none and folds 2 rows
/// apiece into `8.4.56`, `7.1.35` and `7.1.35+8.4.56`; svādi 5b opens none
/// either and folds 64 rows apiece into `8.4.56`, `7.1.35` and
/// `7.1.35+8.4.56` and 116 into `6.4.107` — √kṛ (slice 8b) adds six more
"""),
("""/// re-ran it at the same commit over all 38232 cells / 49904 forms / 594 roots
/// with zero differences, its `entry` negative control verified failing (36
/// √bhū cells). √tṛh joins none of the fork
""",
"""/// re-ran it at the same commit over all 38232 cells / 49904 forms / 594 roots
/// with zero differences, its `entry` negative control verified failing (36
/// √bhū cells), and svādi 5b's re-ran it at the same commit over all 39744
/// cells / 51724 forms / 626 roots with zero differences, its `entry`
/// negative control verified failing (36 √bhū cells). √tṛh joins none of the fork
"""),
],
'crates/panini-prakriya/src/tinanta/guna.rs': [
("594-root × 4-lakāra", "626-root × 4-lakāra"),
],
'crates/panini/tests/trace/rudhadi.rs': [
("""    // upadhā is `a` -- so this engine does not implement 6.4.24 at all and
    // must not be "corrected" toward vidyut's history here.
""",
"""    // upadhā is `a` -- so this engine's 6.4.24 (svādi 5b), which reads a
    // hal-final ANGA, never reaches a rudhādi aṅga (3.1.78 leaves it
    // vowel-final), and must not be "corrected" toward vidyut's history here.
"""),
],
'docs/superpowers/specs/2026-07-29-svadi-gana-design.md': [
("""  not the final ñ that 1.3.72 reads, and it is parasmaipadī.)
""",
"""  not the final ñ that 1.3.72 reads, and it is parasmaipadī.) All ten, and
  the gaṇa's other twenty-two, are curated in svādi 5b
  (`2026-10-07-svadi-gana-5b-design.md`).
"""),
("""  and the ñit roots √stṛ / √kṛ, which additionally want 7.1.100 and the 6.4.10x
  kṛ-specials. The root set below avoids every one.
""",
"""  and the ñit roots √stṛ / √kṛ, which additionally want 7.1.100 and the 6.4.10x
  kṛ-specials. The root set below avoids every one. (Svādi 5b found that
  √stṛ and √kṛ want neither: vidyut-prakriya agrees with them on data
  alone.)
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
AUDIT_DATE=<Step 2's date> python3 /home/dev/vidyut-svadi-5b/slice5b/docsweep_5b.py      # applied 49 edits to 8 files
mise run fmt
```

- [ ] **Step 4: Re-anchor the recorded file:line anchors**

AGENTS.md's ledger cites `guna.rs:2641` and `controller.rs:206`. This slice adds no line above either (`guna.rs`'s edit keeps its line count; `controller.rs` is untouched), but check rather than trust:

```bash
grep -n "626-root × 4-lakāra" crates/panini-prakriya/src/tinanta/guna.rs      # the corpus comment, at its own line
sed -n 2641p crates/panini-prakriya/src/tinanta/guna.rs; sed -n 206p crates/panini-prakriya/src/controller.rs
git diff b2707e7 --stat -- crates/panini-prakriya/src/controller.rs           # empty
```

Expected: line 2641 of `guna.rs` is still the "1872 goldens move" comment and line 206 of `controller.rs` the "only 8 cells" one. If either moved, update the ledger's anchor in AGENTS.md in the same commit, measured by grep.

- [ ] **Step 5: Sweep greps**

The script covers what the replay's greps found. Re-run them on this tree, and fix any hit that is not historical (a past slice's record or paragraph) or Task 5's:

```bash
grep -rni -E "five hundred ninety-four|four hundred sixty-eight|five hundred twenty\b|twenty-five ubhayapad|eleven roots|thirty-eight thousand|forty-nine thousand" README.md AGENTS.md docs/ARCHITECTURE.md tools crates --include=*.rs --include=*.md | grep -v "paradigm/data"
grep -rn -E "\b(594|38232|49904|11672|4248|468|520|8464|6790|927|1635|898|875|1040|890|166)\b" README.md AGENTS.md docs/ARCHITECTURE.md crates/*/src crates/panini/tests/*.rs crates/panini/tests/trace crates/panini/tests/paradigm/main.rs tools/audit/panini_full_audit.rs | grep -v '"10\.0\|"0[0-9]\.0'
grep -rn -i -E "6 of its 38|svādi[^.]{0,40}\*\*open|is not needed|not implement 6\.4\.24|6\.4\.24 at all|complete\*\* — six roots|only asaṁyogapūrva|\bslice 5b\b" README.md AGENTS.md docs/ARCHITECTURE.md tools crates --include=*.rs --include=*.md | grep -v paradigm/data
```

Expected (the replay's output at this point):
- **Spelled-out grep:** one line, AGENTS.md's 6.4.107 ledger ("cells across eleven roots as of tanādi 8b"), historical.
- **Numeral grep:** only historical lines and Task 5's: AGENTS.md's floor paragraph (lines 37 and 92, both 38232, which Task 5 rewrites) and its 10m audit-chain line 717 (38232 / 49904 / 594); `paradigm/main.rs`'s 10m audit-chain line 560, its 10g and 10j census paragraphs (833: 4248; 857: 468), and the pada-ambiguous comment's 1635 (1704, 10m's line, and 1710, svādi 5b's own "from 1635 to 1655"); `trace/curadi.rs:356`'s 468 (√ci's branches); `panini-lipi`'s `\u{927}`; ARCHITECTURE's line 107, "166 total" (the running count before svādi 5b's two).
- **Phrase grep:** two lines, both someone else's: AGENTS.md:311, kryādi's "**complete** — six roots" (kryādi's own claim, out of scope), and `paradigm/main.rs:1154`, adādi's historical "ungated in slice 5b". Nothing for svādi, 6.4.24 or a bare "slice 5b" of this slice.

- [ ] **Step 6: Run the full suite and commit**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
git branch --show-current      # svadi-5b
git add -A
git commit -m "docs: svādi 5b's counts, the audit record, and the sweep

39744 cells / 51724 forms / 626 roots across README, ARCHITECTURE, AGENTS,
paradigm/main.rs and tools/audit; svādi complete at 38 of 38; 478 both-pada
roots; 1655 pada-ambiguous surfaces; 6.4.107 on 188 cells across thirty
roots. Audit at zero divergence against 8da2f90b with 6552 blocked branches;
prior traces byte-identical to the base (56456 branches, 49904 live)."
```

Foreground, timeout 600000 ms (detached fallback per Global Constraints).

---

## Task 5: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and the current-record paragraph); `mise.toml` if the cap moves

Follow AGENTS.md's cargo-mutants protocol. Hazards from this repo's record:
- **Campaigns never run concurrently on this host.** Svādi 5b's is first in the serial queue: svādi 5b, then kryādi 9c, then hetumaṇic. Before the probe and before the campaign, `pgrep -x cargo-mutants` must print nothing; if it prints a PID, another slice's run is live, so wait for it and do not launch. The probe and the campaign run one after the other, never together.
- **Measure, never scale.** The cap must clear a full uncaught suite run at the parallelism used, under campaign load, and twice the slowest caught phase (the `skip_nic` blow-up; AGENTS.md's floor paragraph).
- **Every invocation rotates `mutants.out`**, so pass `-o` always, a separate one for every probe, and copy `outcomes.json` durably before any other invocation.
- **The mise shim fails in background shells.** Use the real binary: `CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`, after `eval "$(mise env -s bash)"`.
- **Background shells die at about 60 minutes.** Launch detached with `env -u CARGO_MUTANTS_JOBS setsid nohup`, as below. The controller polls the detached run (`pgrep -x cargo-mutants`, or `kill -0 <PID>`) from its own turns; never wait on `pgrep -f`, which matches its own shell. If the run dies, copy `outcomes.json` aside and resume with the same command plus `--iterate`.

What to expect (the replay measured the lists):
- **The mutant list grows by 7.** `panini-prakriya` lists 917 (910 on main); `panini-analyze` 12 and `panini-data` 12, both unchanged. By name, none leave and seven arrive: in `anga.rs` `delete !` twice, `+` → `-` and `+` → `*` (6.4.24); in `tripadi.rs` `delete !` twice and `||` → `&&` (8.4.39).
- **`--in-diff` over the slice's production diff for `panini-prakriya`** lists 9: those seven, at `anga.rs:494:43` (`+` → `-`, `+` → `*` on `ANGA + 1`), `494:66` (`delete !`, the empty-term test), `497:16` (`delete !`, the ṅit test), `tripadi.rs:1683:16` and `1683:58` (`delete !` on each conjunct) and `1683:55` (`||` → `&&`), plus `mod.rs:82:5`'s two whole-`derive` mutants. Every one should be CAUGHT: each `ANGA + 1` mutant starts the search at a term with no `Ngit`, and each 8.4.39 mutant either drops √tṛp's bar (*tfpRoti*) or bars ṇatva on every svādi row (*stfnoti*). Confirm the spans by `--list`; never compute them.
- **`--in-diff` over the data crate's diff** lists none ("No mutants to filter"): `IDIT` is a constant, and the rest is rows and test code.
- **One documented non-caught entry moves.** The permanent ṇatva hang goes from `tripadi.rs:1729:23` to `1755:23` (8.4.39 and its comment sit above it, 26 lines). The equivalents stay at `adesha.rs:649:30` and `tripadi.rs:1416:38`; the `skip_nic` pair at `sanadi.rs:60:5` and `60:39`. `tripadi.rs:217:38: replace - with /` (8.2.78's, CAUGHT) also matches a `tripadi.rs:[0-9]+:38: replace - with /` regex, so anchor the probe on `1416`.
- **The cap probably holds at 13520.** The slice adds no optional-ṇic id, so `skip_nic -> true` still forks 2^8 ways; the corpus grows 4.0% (39744 cells against 38232). Re-measure all the same, as AGENTS.md requires on corpus growth.

- [ ] **Step 1: Measure the floor**

With nothing else of ours running, run this twice, each in the foreground with timeout 600000 ms: `cat /proc/loadavg; time mise run test >/dev/null 2>&1; cat /proc/loadavg`. If a run outgrows the timeout, use the detached form in Global Constraints with `time` inside it. Record both wall clocks, user and sys CPU, and the four load-average readings. Keep AGENTS.md's comparison chain, with 10m's joining it: 12m41.146s / 7m42.564s at 38232 cells under heavy external load (load 19.53–109.07, user 14m1.8s / 14m8.6s).

- [ ] **Step 2: Locate and probe the uncaught equivalents and the `skip_nic` pair**

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
PROBE=/home/dev/mutants-records/svadi-5b-probe     # durable, its own -o
mkdir -p "$PROBE"
mise exec -- "$CM" mutants --package panini-prakriya --list -o "$PROBE/list" 2>/dev/null | wc -l      # 917
mise exec -- "$CM" mutants --package panini-prakriya --list -o "$PROBE/list" 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /=|skip_nic"
git diff b2707e7 -- crates/panini-prakriya/src > "$PROBE/prakriya.diff"
mise exec -- "$CM" mutants --package panini-prakriya --list --in-diff "$PROBE/prakriya.diff" -o "$PROBE/in-diff" 2>/dev/null
```

Expect `adesha.rs:649:30`, `tripadi.rs:217:38` (8.2.78's, caught; not the equivalent), `tripadi.rs:1416:38` (the equivalent, inside 8.3.13's `apply`), `tripadi.rs:1755:23` (the ṇatva hang), and `sanadi.rs:60:5` (`skip_nic -> bool with true` and `with false`) and `60:39` (`!=` → `==` in `skip_nic`); then the nine in-diff mutants above. Write the lines as `<A>` (649), `<T1>` (1416), `<T2>` (1755), `<K>` and `<K2>` (60). The probe runs past the 60-minute shell limit, so launch it detached, once `pgrep -x cargo-mutants` prints nothing:

```bash
pgrep -x cargo-mutants || echo "no campaign running"
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 20000 -j 4 -o "$PROBE" \
  --re "adesha.rs:<A>:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" \
  --re "sanadi.rs:<K>:5: replace skip_nic -> bool with true" --re "sanadi.rs:<K2>:39: replace != with == in skip_nic" \
  > "$PROBE/probe.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$PROBE/started"; cat /proc/loadavg > "$PROBE/load.started"
```

The 20000 s probe cap is a ceiling for measurement, not the campaign's cap. The regexes also match caught `mod.rs` `derive` mutants; that is expected. Poll from the controller until `pgrep -x cargo-mutants` prints nothing. Then record the end time and load, and copy `$PROBE/mutants.out/outcomes.json` to `$PROBE/probe-outcomes.durable.json` before anything else. Both equivalents must be MISSED, not TIMEOUT; both `skip_nic` mutants must be CAUGHT. Read each test-phase duration from the outcomes. The provisional cap is max(13520, 6 × the longer equivalent, 2 × the longer `skip_nic` phase), rounded up to the next 10 s.

- [ ] **Step 3: Run the campaign detached**

```bash
pgrep -x cargo-mutants || echo "no campaign running"      # must print the second
OUT=/home/dev/mutants-records/svadi-5b      # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout <CAP> -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"; cat /proc/loadavg > "$OUT/load.started"
```

`<CAP>` is Step 2's provisional cap. Run nothing CPU-heavy of ours meanwhile. 10m's campaign took 8h54m at 13520; expect about the same. The controller polls `pgrep -x cargo-mutants` across turns; never a foreground `sleep` loop that outlives a turn.

- [ ] **Step 4: Read the outcomes**

When `pgrep -x cargo-mutants` prints nothing:

```bash
date -u +"%F %T UTC" > "$OUT/finished"; cat /proc/loadavg > "$OUT/load.finished"
cp "$OUT/mutants.out/outcomes.json" "$OUT/outcomes.durable.json"
tail -5 "$OUT/campaign.log"
cat "$OUT/mutants.out/missed.txt" "$OUT/mutants.out/timeout.txt"
```

Expected (exit code 3 is normal when a timeout is present):
- **929 mutants: 874 caught, 52 unviable, 2 missed, 1 timeout.**
  - panini-prakriya: 917 / 866 / 48 / 2 / 1.
  - panini-analyze: 12 / 8 / 4 / 0 / 0.
- `missed.txt` holds exactly `crates/panini-prakriya/src/tinanta/adesha.rs:<A>:30: replace + with *` and `crates/panini-prakriya/src/tinanta/tripadi.rs:<T1>:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `crates/panini-prakriya/src/tinanta/tripadi.rs:<T2>:23: replace -= with /=`.
- All nine in-diff mutants CAUGHT; read each one's test phase.

Then diff the non-caught set against 10m's on the full record, both with and without span lines:

```bash
python3 - "$OUT/outcomes.durable.json" /home/dev/mutants-records/curadi-10m/outcomes.durable.json <<'PY'
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
- **Without lines:** `28 28`, `new: []`, `gone: []`.
- **With lines:** `55 55`; `new:` and `gone:` list the same entries at their svādi 5b and 10m spans. At least the hang moves (10m's durable record has it at `1718:23`, the line its campaign ran on, before 10m's last comment edits; this campaign's is `1755:23`). Write down exactly what prints, and name each moved entry and its cause in the record.

If not:
- Any **other timeout** is a suspect survivor the larger suite pushed past the cap. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant among the nine in-diff ones is a gap in Task 2's tests; add the test that kills it, then re-run that mutant alone with its own `-o`. Any other missed mutant means a test that caught it at 38232 cells no longer does; stop and report.

**Step 4b: the data-crate mutants.** Confirm the slice's `panini-data` diff holds none:

```bash
git diff b2707e7 -- crates/panini-data/src/lib.rs > "$OUT/data.diff"
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

The cap is max(430, 6 × the longest campaign-load equivalent phase, 2 × the longest caught phase), rounded up to the next 10 s, over both readings of the `skip_nic` `true` mutant (the probe's and the campaign's); it is never lowered on the quieter reading.
- If that is 13520 or less, the cap stays 13520.
- Otherwise change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together (`grep -n "13520" AGENTS.md mise.toml`).

- [ ] **Step 6: Record it in AGENTS.md**

- **The floor paragraph.** Rewrite the paragraph that opens `**The floor behind the 13520s cap, measured at 38232 cells on Rust 1.99.0,`. Use Step 1's and Step 2's numbers at 39744 cells, the load averages, and the cap Step 5 chose. Keep the comparison chain to earlier floors, with 10m's joining it: 12m41.146s / 7m42.564s at 38232 cells under heavy external load (user 14m1.8s / 14m8.6s), the `skip_nic` `true` mutant 6759.15s in the probe and 5329.63s in the campaign, `!=` → `==` 5547.69s and 4272.82s, the equivalents' campaign-load phases 388.14s and 457.45s. State that svādi 5b adds no optional-ṇic id (2^8 forks unchanged) and grows the corpus 4.0%.
- **The current record.** Replace the `**Current record (curādi 10m, 2026-10-07).**` paragraph with `**Current record (svādi 5b, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window, with the load at launch and end;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**, in code blocks;
  - the non-caught set diffed against 10m's on the full record, both ways (Step 4's script output), naming what moved and why;
  - the nine in-diff mutants by site and outcome, with test phases, and the by-name list change (7 arrive, none leave);
  - the `skip_nic` pair's probe and campaign phases;
  - Step 4b's empty data-crate list;
  - the campaign-load phases and margins;
  - that `outcomes.json` is kept at `/home/dev/mutants-records/svadi-5b/mutants.out/outcomes.json`, with the durable copy at `/home/dev/mutants-records/svadi-5b/outcomes.durable.json`, and the probe's at `/home/dev/mutants-records/svadi-5b-probe/probe-outcomes.durable.json`.

  End it with a pointer to the record it replaces. Run `git rev-parse --short HEAD` before committing, and write ``The curādi 10m record it replaces: `git show <that hash>:AGENTS.md`.``

- [ ] **Step 7: Commit**

```bash
git branch --show-current      # svadi-5b
git add AGENTS.md mise.toml
git commit -m "chore: svādi 5b mutation gate — floor and uncaught run re-measured at 39744 cells

Every new mutant caught; missed.txt holds only the two documented
equivalents and timeout.txt only the permanent ṇatva-scan entry, at its
moved tripadi.rs span."
```

Adjust the message to what Steps 4 and 5 actually found (the cap, any difference).

---

## Task 6: Final review and finish

- [ ] **Step 1: Confirm the gate is green and the base has not moved**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
git fetch origin
git log --oneline HEAD..origin/main      # empty: main has not moved since the base
git branch --show-current                # svadi-5b
git log --oneline b2707e7..HEAD          # the spec, the plan and the four task commits
```

Foreground, timeout 600000 ms. If `origin/main` has moved (kryādi 9c or hetumaṇic merged first), stop: the spec requires a rebase and a re-measure of every total, the goldens' neighbours, the dump against a fresh base archive and the audit. Report rather than adding the totals up by hand.

- [ ] **Step 2: The final review package**

```bash
git diff --diff-algorithm=histogram b2707e7..HEAD -- . ':(exclude)crates/panini/tests/paradigm/data/svadi.rs' > /home/dev/svadi-5b-gate/review.diff
git diff --stat b2707e7..HEAD
wc -l /home/dev/svadi-5b-gate/review.diff
```

Dispatch the whole-branch reviewer with `review.diff`, the spec, this plan, and `/home/dev/svadi-5b-gate/` (the gate's dumps and audits). The goldens are excluded on purpose: they are generator output, hash-checked in Task 3 Step 4 and verified landed in Step 5; the default diff algorithm shows appended golden rows as fake deletions. Ask the reviewer to probe-derive off HEAD rather than trust comments, in particular: 6.4.24 on a hand-built adādi or juhotyādi nasal-upadhā aṅga, and 8.4.39 on `10.0351`. Fix what it finds in a new commit, re-running the suite (foreground, 600000 ms).

- [ ] **Step 3: Open the PR**

```bash
git branch --show-current      # svadi-5b — a detached HEAD strands commits
git push -u origin svadi-5b
gh pr create --title "svādi 5b — the last thirty-two rows, 6.4.24 and 8.4.39" --body "$(cat <<'BODY'
Svādi 5b curates svādi's other thirty-two dhātupāṭha rows and closes the
gaṇa at 38 of 38: ten ñit rows ubhayapadī by 1.3.72, twenty-two
parasmaipadī.

- 6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti* lands as the general sūtra: an
  anidit root's nasal upadhā goes before the next non-empty term when that
  term is ṅit (*dabhnoti*). Its *aniditām* is `Tag::Idit`, from the data
  layer's `IDIT`, because 7.1.58's num is stored, not derived. Kryādi 9e's
  ten rows reuse it.
- 8.4.39 *kṣubhnādiṣu ca* bars ṇatva on √tṛp before śnu (*tṛpnoti*), keyed
  by row because curādi's two `tfpa~` rows store the same `tfp`.
- The other thirty match vidyut-prakriya on data alone. `05.0033 kzi`'s
  every form is also tanādi √kṣiṇ's; the `check()` test pins both readings.

The golden suite goes from 38232 to 39744 cells. The audit shows zero
divergence against `8da2f90b`, a base-vs-branch dump of every prior branch
(blocked ones included, every step's text) is byte-identical, and the
mutation gate finds every new mutant caught.
BODY
)"
```

- [ ] **Step 4: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`:
   - run `git worktree remove .worktrees/svadi-5b`;
   - run `git worktree remove --force .worktrees/proto-svadi` and `git branch -D proto-svadi` (the throwaway prototype);
   - delete the local and remote `svadi-5b` branch;
   - run `git pull` on `main`.
4. Keep `/home/dev/vidyut-svadi-5b`, `/home/dev/svadi-5b-gate` and `/home/dev/mutants-records/svadi-5b*`: AGENTS.md's record points at the last.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| The thirty-two rows with their artha; ten `Ubhayapada`, twenty-two `Parasmaipada` | 1 Step 3 (`rows_5b.py`), 3 Step 3 |
| 6.4.24 general: `!Tag::Idit`, hal-final `ANGA`, penultimate `n`, next non-empty term `Ngit`; after 6.4.23; records once | 2 Step 3 (`engine_5b.py`) |
| 6.4.23's comment drops "not needed" and points to 6.4.24 | 2 Step 3 |
| `IDIT` with its doc; `Tag::Idit` set in `derive` | 2 Step 3 |
| `idit_matches_upadesha_markers`, both directions, the 2564 arm's reading | 2 Step 1 (`is_idit` now shared by all three readers) |
| 8.4.39 before 8.4.1, `bars` 8.4.1 / 8.4.2, `KSUBHNADI` = `05.0028`, no text change | 2 Step 3 |
| Pins: `exactly_the_pinned_bars`, `tinanta_rule_order_is_pinned` | 2 Step 1 |
| 6.4.24 unit tests: fires; declines on `Idit`; on a pit or non-ṅit term; on vowel-final or non-`n`-upadhā; length ≥ 5 witness | 2 Step 1 (`granT`; the code has no index to mutate) |
| 8.4.39 unit tests: `05.0028` only; declines on `10.0351`'s `tfp` | 2 Step 1 |
| Trace pins *dabhnoti*, *tṛpnoti*, *sunute*, *ṛkṣiṇoti* (which ṇatva rule: 8.4.2) | 3 Step 1 |
| Fires-only-on-rows, corpus-wide | 3 Step 1 (42 live branches each) |
| The CISPHUR test reworded, not weakened | 3 Step 1 |
| Row count, `Dhatu::pada` census, `curated_pada_agrees_with_upadesha_markers` over 32 new verdicts | 3 Step 1 (the last re-derives every row already; its doc's 134 of 626) |
| The svādi row-list test | 3 Step 1 (new: none existed to rename) |
| Goldens: 32 roots' rows and 308 alternates; census 5b paragraph and totals | 3 Steps 1, 4–6 |
| Prior-trace stability, every branch, every step, byte-identical | 1 Step 5 (gate), 4 Step 2 (final) |
| Audit: private copy, repointed, committed harness copied, totals raised, `entry` control, README record | 1 Steps 4–5, 4 Steps 1–3 |
| Mutation gate: `--in-diff`; first in the serial queue; floor and `-j 4` probe of equivalents and `skip_nic`; cap by rule; `mise.toml` and AGENTS together; non-caught named verbatim; durable `outcomes.json` | 5 |
| Doc sweep: README Scope, totals, multi-form counts, ARCHITECTURE svādi entry and 6.4.24 beside 6.4.23, AGENTS coverage / floor / record, the sweep greps, anchors re-grepped | 4 Steps 3–5, 5 Step 6 |
| Later slices (9e, 9d) | no task; 8.4.39's and 6.4.24's comments name them |

Unit-test bullets in the spec, one by one: each has a step with code above. Credited rows are counted separately from form-changing rows: 6.4.24 and 8.4.39 each change or bar on exactly one row's 42 live branches (36 cells + 6 alternates), and no prior row is credited (the fires-only test and the dump).

**Where this plan departs from the spec's letter, and why.**
- **The spec says no new surface equals another root's.** The goldens say otherwise: all 44 of `05.0033 kzi`'s forms are tanādi `08.0004 kziR`'s. Nothing breaks (homographs are analyses, not collisions in the data), but the `check()` test pins *kziRoti*'s two readings, and Task 3 Step 7 derives the collision list from the goldens.
- **"The svādi row-list test, renamed"** — there was none. `svadi_rows_are_the_thirty_eight_curated_roots` is new.
- **The sweep phrases "6 of its 38" and "svādi … open" do not occur.** The docs already called svādi complete at six roots ("**complete** — six roots"); the sweep rewrites that claim instead.
- **8.4.39's "next non-empty term after ANGA is śnu"** is `Tag::Svadi` on `ANGA`: śnu is the gaṇa's only vikaraṇa and is never emptied, so the two are the same test, and a term-shape test (`n…`) could not tell śnu from śnā when 9d joins. The guard test pins the conjunct with a non-svādi `05.0028`.
- **Beyond the spec's sweep list:** the 6.4.107 counts (188 cells, thirty roots; the "only asaṁyogapūrva roots" claim), the pada-ambiguous set (1635 → 1655, measured), 168 pinned ids, `Rule.bars`'s third user, the rudhādi trace comment and the AGENTS sentence that said 6.4.24 is unimplemented, and the 2026-07-29 svādi spec's pointers.
- **"svādi 5b", not "slice 5b"**, everywhere, because two earlier slices already carry that name.

**Placeholder scan.** `<A>`, `<T1>`, `<T2>`, `<K>`, `<K2>`, `<CAP>`, `<DATE>` and the audit date are values the executor measures, each with the command that produces it and its expected value where one is known. No step says "update the tests" without the edit.

**Type consistency.**
- `IDIT: &[&str]` is read by `derive` (`IDIT.contains(&dhatu.dhatupatha)`) and by `idit_matches_upadesha_markers`; `KSUBHNADI: [&str; 1]` by 8.4.39 (`KSUBHNADI.contains(&p.ctx.dhatupatha)`) and its pin.
- 6.4.24's `apply` binds `next: &Term`, then `head: &[char]`, `last: &char` from `chars.as_slice()`, and collects `head.iter().chain([last])` (`&char` items) into a `String`.
- `before_affix(&str, &str, Tag, &str) -> Prakriya` and `svadi_row(&'static str, &'static str) -> Dhatu` are test-local. `credited` yields `(&'static str, Gana)`; the fires-only test reads `.0`. `svadi_analyses_its_5b_forms` compares `Vec<(&str, Pada)>` with `[(&str, Pada); 2]`.
- The golden tuple shapes match `ParadigmRow` and `AlternateRow`; the generator is 10m's with the row list and paths changed.

**Review Focus.** Five lines, each with its test in Task 1, 2 or 3.

**Known soft spots.**
- **Task 5's campaign numbers are expectations**, not measurements; only the lists are measured. Record the load beside every timing. The span diff in Step 4 is the least certain line: write down what prints.
- **The suite's wall clock under this host's load** ran past one 600000 ms call in the replay (`trace` 713 s). The detached form in Global Constraints is the fallback; never end a turn on a live run.
