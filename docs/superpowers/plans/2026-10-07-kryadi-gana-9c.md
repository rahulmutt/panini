# Kryādi gaṇa slice 9c Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate twenty-two kryādi rows that vidyut-prakriya matches cell for cell on the rules already in the pipeline, with no engine change. The golden suite goes from 38232 to 39312 cells, and kryādi from 6 to 28 of its 71 dhātus.

**Architecture:** Six tasks:
- **Task 1** checks the worktree and the baseline, adds the 22 rows, generates their goldens from the engine (asserting engine = vidyut cell by cell), raises the audit harness's totals, and runs the gate: the suite with `--no-fail-fast`, the `entry` negative control, the audit and a prior-trace dump. It does not commit: its suite is red by design, on four totals and roster assertions.
- **Task 2** moves every assertion the rows move (the row count, a kryādi row-list test, the census, the pada-ambiguous set, the √khac 8.4.40 roster), adds three trace pins and a goldens-derived `check()` test, and commits Tasks 1 and 2 together, green.
- **Task 3** is the doc sweep and the audit record.
- **Task 4** rebases onto `main` once svādi 5b has merged, and re-measures every total on the combined tree.
- **Task 5** is the mutation gate, run only when it is this slice's turn in the serial queue (svādi 5b, then kryādi 9c, then hetumaṇic).
- **Task 6** finishes the branch.

No production code changes. The one `panini-prakriya` edit is a corpus-size number in a `guna.rs` comment (Task 3), on the same line, so no mutant span moves.

**Tech Stack:** Rust 1.99.0, pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, in a PRIVATE copy at `/tmp/vidyut-9c` (other slices audit in parallel from their own copies; never edit `/tmp/vidyut-full`).

**Spec:** `docs/superpowers/specs/2026-10-07-kryadi-gana-9c-design.md`. Its Scope table is the slice's scope (22 rows; 616 / 39312 / 51116 / 11804). Its Decisions section explains why no engine or sibling-pair change is needed, the √khac roster widening and the homograph codes. Its Later slices section is 9d's and 9e's; nothing from it is in scope here.

**Workspace:** The spec and this plan are on the branch `kryadi-9c`, checked out at `/workspace/.worktrees/kryadi-9c`. Every path below is relative to that worktree unless it starts with `/`. `/workspace` stays on `main`.

**Provenance.**
- **The prototype.** A throwaway worktree, `/workspace/.worktrees/proto-kryadi` (branch `proto-kryadi`, uncommitted edits on `b2707e7`), curated all sixty-five remaining kryādi rows. On data alone its audit found 1863 differing cells, every one on the forty-three rows slices 9d and 9e take and none on this slice's twenty-two. Its engine edits are 9d's and 9e's; this plan takes nothing from them. It never ran the `entry` control, so Task 1 runs it before anything is recorded. Leave the prototype in place: 9d and 9e still need it.
- **The replay.** Every script below was run, in this plan's order, on a fresh detached worktree of the spec commit `f28879b`, against this branch's own engine:
  - the generator found all 1080 new cells equal to vidyut's (`1080 cells, 1212 forms, 132 alternates, 0 differences`);
  - before Task 2, the suite failed on exactly the four tests Task 1 Step 9 names, and nothing else;
  - after Task 2, the full suite passed with `--no-fail-fast`, and so did `fmt-check` and `lint`;
  - the audit at 616 roots / 39312 cells / 51116 forms showed zero differences with 6552 blocked branches, and the `entry` control failed on 36 cells;
  - a merge-base-vs-branch dump of every prior branch, blocked ones included, was byte-identical (56456 lines, 49904 live);
  - after Task 3, the stale-number grep printed exactly the history lines Task 3 Step 3 lists.
  Task 1 Step 9's and Task 2 Step 5's per-binary counts are the replay's.

**Throwaway scripts.** Everything under `/tmp/vidyut-9c/slice9c/` and the two vidyut examples (`kryadi_goldens_9c.rs`, `trace_dump_9c.rs`) never ship. Each is reproduced in full below with its sha256, so it can be recreated if `/tmp` was cleaned. Create a file only if it is missing, and check its hash either way (`sha256sum <file>`). If a hash differs, stop and report.

## Global Constraints

- **No engine change.** No `Rule`, guard, constant or order in `panini-prakriya` changes; its only edit is `guna.rs`'s corpus size in a comment (594-root → 616-root, Task 3). If a test can only pass by changing engine code, stop and report.
- **Rows:** exactly the spec's twenty-two, inserted after `09.0045 vf` in dhātupāṭha order, with the spec's codes (`09.0005` → `si`, `09.0056` → `naB`) and padas (eight `Ubhayapada`, fourteen `Parasmaipada`). `dhatupatha_numbers_resolve_upstream` and `curated_pada_agrees_with_upadesha_markers` check each against the vendored upadeśa, unchanged.
- **Counts after the slice (before the Task 4 rebase):** 616 roots; 39312 cells (4368 blocks); 51116 forms; `ALTERNATES` 11804; one / two / three-form cells 30760 / 6834 / 971 (fours, fives, sixes, sevens, nines unchanged: 361, 10, 367, 1, 8); keys `8.4.56` 942, `7.1.35` 934, `7.1.35+8.4.56` 934, every other key unchanged; pada-ambiguous pinned surfaces 1667; blocked branches 6552; both-pada roots 476.
- **Pre-existing cells must stay byte-identical, traces included,** with no exception. Regenerate no prior golden. No sibling-pair test changes (the spec's Decisions).
- **Goldens come from the generator, which asserts engine = vidyut cell by cell, and their sha256 must match this plan's.** **Do not edit a golden to match the engine.** If the generator reports a difference, or a hash differs, stop and report.
- **The private vidyut copy.** `/tmp/vidyut-9c` is a copy of `/tmp/vidyut-full` made for this slice. Its `vidyut-prakriya/Cargo.toml` hardcodes the `panini` and `panini-data` dev-deps: point them at this worktree's `crates/` before generating or auditing (`grep -n '^panini' /tmp/vidyut-9c/vidyut-prakriya/Cargo.toml` every time), and at the base worktree only for the prior-trace dump's base half. Copy the committed harness `tools/audit/panini_full_audit.rs` into it; never rewrite the harness.
- **The suite runs in the FOREGROUND with the Bash tool timeout at 600000 ms.** Never background it yourself, and never end the turn while it runs. It takes 6–9 minutes under this host's load (the `trace` binary alone 350–400 s). Run it as:
  ```bash
  cd /workspace/.worktrees/kryadi-9c && bash -c 'echo $$ > /tmp/kryadi-9c-suite.pid; exec mise exec -- cargo test --workspace --no-fail-fast' > /tmp/kryadi-9c-suite.log 2>&1; echo "EXIT_CODE=$?" >> /tmp/kryadi-9c-suite.log; grep -E "^test .*FAILED|test result|^EXIT_CODE=" /tmp/kryadi-9c-suite.log | grep -v " 0 passed"
  ```
  If it outruns the timeout (the tool then moves it to the background), poll in the same turn, each call with timeout 600000 ms, until `EXIT_CODE=` prints:
  ```bash
  PID=$(cat /tmp/kryadi-9c-suite.pid); for i in $(seq 1 55); do kill -0 "$PID" 2>/dev/null || break; sleep 10; done; grep -E "^test .*FAILED|test result|^EXIT_CODE=" /tmp/kryadi-9c-suite.log | grep -v " 0 passed"
  ```
  Never wait with `pgrep -f "cargo test"` (it matches its own shell and loops forever), and never with a bare `pgrep -x cargo` (other slices run cargo on this host). Never pipe the suite through `tail`.
- `mise run test -- -p X` does not scope. Scope with `mise exec -- cargo test -p <crate> <filter>`.
- Commit only where a task says so. Run `mise run fmt` and `mise run lint` before each commit. Run `git branch --show-current` before every commit and before the push: it must print `kryadi-9c` (a detached HEAD strands commits).
- The `cargo-mutants` mise shim fails in background shells. Use the real binary, `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`. Mutation campaigns never run concurrently on this host: Task 5 waits for its turn in the serial queue.
- Review packages and diffs exclude the appended goldens (`crates/panini/tests/paradigm/data/kryadi.rs`) and use `git diff --diff-algorithm=histogram`: the default algorithm shows appended golden rows as fake deletions.
- Never write a session scratchpad path into a commit, a doc or a script. The throwaway files live under `/tmp/vidyut-9c` and `/tmp/kryadi-9c-*`.

## Review Focus

These are the inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **A homograph's forms answering as the wrong root.** Six of the twenty-two codes are also other gaṇas' rows' (`prI`, `mI`, `yu`, `Dras`, `puz` curādi; `viz` juhotyādi), and `check` reports a code, not a number. A collision would surface as a second analysis nobody asked for. → Task 2's `kryadi_analyses_its_9c_forms` walks every pinned form of the 22 rows (each analysis is that row's code, through 3.1.81's śnā) and of every partner row (no analysis through śnā), and derives the partner list from `dhatus()` so a new homograph (svādi 5b's `05.0002 si`) fails it until it is added (Task 4).
2. **A ñit row's parasmaipada cell crediting 1.3.72.** Both padas of an ubhayapadī row derive, and a trace that credits 1.3.72 on the parasmaipada side would be a wrong sūtra under a right form. → Task 2's `krinati_trace_takes_intervening_natva_on_a_nyit_root` (1.3.78, not 1.3.72) and `krinite_trace_is_the_ubhayapada_atmanepada_shna_path` (1.3.72, not 1.3.78).
3. **ṇatva at the wrong place.** √krī's `r` reaches śnā's `n` across the root's `I` (8.4.2, not 8.4.1), and √mṛd's `d` blocks ṇatva altogether. → Task 2's `krinati_…` pin (8.4.2 present, 8.4.1 absent) and `kryadi_analyses_its_9c_forms`'s Invalid forms `krInAti` and `mfdRAti`.
4. **8.4.40's converse arm over- or under-firing on √khac.** It must make śnā's `n` a `Y` after the `c` everywhere but before śānac. → Task 2's `khacnati_trace_takes_the_converse_shcutva` (`KacYAti` credits 8.4.40, `KacAna` does not), the widened roster (exactly 41 √khac branches) and the Invalid `KacnAti`.
5. **Pada-ambiguous surfaces.** The eight ñit roots' ātmanepada collides with their own parasmaipada in a shape no earlier root has (`krIRIta` is vidhiliṅ ātmanepada prathama eka and loṭ parasmaipada madhyama bahu). → Task 2 pins the measured 1667-surface set, and `kryadi_analyses_its_9c_forms` asserts each laṭ prathama eka has exactly one analysis in its own pada.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-data/src/lib.rs` | Task 1: the 22 rows. Task 2: the row count (616), `kryadi_rows_are_the_twenty_eight_curated_roots`, `Dhatu::pada`'s census (124 of 616) and the marker doc's counts (82 / 61). Task 4: `09.0005`'s comment if svādi's `si` landed |
| `crates/panini/tests/paradigm/data/kryadi.rs` | Task 1: 120 golden rows, 132 alternates |
| `crates/panini/tests/paradigm/main.rs` | Task 2: the census and its 9c paragraph, the pada-ambiguous set (1667) and its comment, `kryadi_analyses_its_9c_forms`. Task 3: doc counts, the alternates doc, the audit chain |
| `crates/panini/tests/trace/kryadi.rs` | Task 2: three trace pins |
| `crates/panini/tests/trace/juhotyadi.rs` | Task 2: the 8.4.40 roster gains `09.0067` |
| `tools/audit/panini_full_audit.rs` | Task 1: totals and header counts |
| `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini-prakriya/src/tinanta/guna.rs` (one comment number) | Task 3 |
| `AGENTS.md`, maybe `mise.toml` | Task 5 |

---

## Task 1: Baseline, rows, goldens and the gate

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (rows only)
- Modify: `crates/panini/tests/paradigm/data/kryadi.rs` (goldens only)
- Modify: `tools/audit/panini_full_audit.rs` (totals and header counts)

**Interfaces:**
- Consumes: `panini_data::dhatus()`, `Panini::derive`, the vidyut `Dhatupatha` and `Vyakarana`.
- Produces: the 22 `Dhatu` rows (in `DHATUS` after `09.0045`, in the order of the generator's `NEW`); 120 `PARADIGM` and 132 `ALTERNATES` rows in `kryadi.rs`; the audit date (`date -u +%F`) that Task 3 writes into the record.

- [ ] **Step 1: Check the worktree and where `main` stands**

```bash
cd /workspace/.worktrees/kryadi-9c
git branch --show-current            # kryadi-9c
git log --oneline -3                 # the plan commit, the spec commit f28879b, then b2707e7
git status --short                   # empty
git -C /workspace branch --show-current   # main
git fetch origin && git log --oneline -1 origin/main
```

If `origin/main` is still `b2707e7`, go on. If svādi 5b has already merged, run Task 4 Step 1 now (the rebase is conflict-free: the branch holds only the spec and plan commits), do Tasks 1–3 on the rebased tree, then Task 4 Steps 2–4. Every edit script below asserts its `old` text occurs exactly once and writes nothing otherwise; on a moved `main`, a script that refuses names each text it could not find. Re-derive those edits by hand with the same intent, record each in the commit message, and re-measure every count (the generator's hashes, the audit, the dump, the suite's census) instead of using this plan's.

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint
```

Then run the suite exactly as Global Constraints says (foreground, timeout 600000 ms, poll the PID in-turn if it outruns). Expected: `EXIT_CODE=0`, with `panini-prakriya` 447 tests, `trace` 217, `paradigm` 31, `panini-data` 29, `panini` 7, `roundtrip` 1, `panini-analyze` 8, `cli` 5, `panini-lipi` 6.

- [ ] **Step 3: Make the private vidyut copy**

```bash
test -d /tmp/vidyut-9c || cp -a /tmp/vidyut-full /tmp/vidyut-9c
mkdir -p /tmp/vidyut-9c/slice9c
WT=/workspace/.worktrees/kryadi-9c
V=/tmp/vidyut-9c/vidyut-prakriya
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml      # both must point at /workspace/.worktrees/kryadi-9c/crates
git -C /tmp/vidyut-9c log --oneline -1 # 8da2f90
```

The copy keeps `/tmp/vidyut-full`'s `target/`, so the first build is incremental. Never edit `/tmp/vidyut-full` itself.

- [ ] **Step 4: The rows**

Create `/tmp/vidyut-9c/slice9c/rows_9c.py` if it is missing (sha256 `126b47a9b491d6854c7fd109806966e43dfd388cfd3f8ada3e990716141ff9e9`). Each row's comment names its upadeśa, artha and root, its pada and the sūtra that gives it, any rule the row reaches that the reader would not expect, any homograph, and its laṭ parasmaipada prathama eka.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 9c — the twenty-two kryādi rows' `Dhatu` entries,
inserted after `09.0045 vfN`, the last kryādi row, in dhātupāṭha order. Run
from the worktree root."""
p = 'crates/panini-data/src/lib.rs'
s = open(p).read()
ROWS = """    Dhatu {
        // 09.0001 `qukrI\\Y` dravyavinimaye (√krī). Ubhayapadī by 1.3.72, the
        // ñit; the `qu` is a ḍu-it (1.3.5) and the `\\` the root vowel's accent.
        // The `r` reaches śnā's `n` across the root's `I`, so 8.4.2: krIRAti,
        // krIRIte. Slice 9c.
        dhatupatha: "09.0001",
        code: "krI",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "dravyavinimaye",
    },
    Dhatu {
        // 09.0002 `prI\\Y` tarpaRe kAntO ca (√prī). Ubhayapadī by 1.3.72; 8.4.2
        // as for √krī: prIRAti. Shares code and artha with curādi's `10.0373
        // prIY`, a distinct row, and no surface form of the two meets. Slice 9c.
        dhatupatha: "09.0002",
        code: "prI",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "tarpaRe kAntO ca",
    },
    Dhatu {
        // 09.0003 `SrI\\Y` pAke (√śrī). Ubhayapadī by 1.3.72; 8.4.2 as for √krī:
        // SrIRAti. Slice 9c.
        dhatupatha: "09.0003",
        code: "SrI",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "pAke",
    },
    Dhatu {
        // 09.0004 `mI\\Y` hiMsAyAm (√mī). Ubhayapadī by 1.3.72: mInAti, mInIte.
        // Shares its code with curādi's `10.0361 mI\\`, a distinct row, and no
        // surface form of the two meets. Slice 9c.
        dhatupatha: "09.0004",
        code: "mI",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 09.0005 `zi\\Y` banDane (√ṣi). Stored `si` per 6.1.64 dhātvādeḥ ṣaḥ
        // saḥ, as `stiG` is. Ubhayapadī by 1.3.72: sinAti, sinIte. Slice 9c.
        dhatupatha: "09.0005",
        code: "si",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "banDane",
    },
    Dhatu {
        // 09.0011 `yu\\Y` banDane (√yu). Ubhayapadī by 1.3.72: yunAti, yunIte.
        // Shares its code with the ākusmīya `10.0235 yu`, a distinct row, and
        // no surface form of the two meets. Slice 9c.
        dhatupatha: "09.0011",
        code: "yu",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "banDane",
    },
    Dhatu {
        // 09.0012 `knUY` Sabde (√knū). Ubhayapadī by 1.3.72: knUnAti, knUnIte.
        // Slice 9c.
        dhatupatha: "09.0012",
        code: "knU",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "Sabde",
    },
    Dhatu {
        // 09.0013 `drUY` hiMsAyAm (√drū). Ubhayapadī by 1.3.72; 8.4.2 across
        // the `U`: drURAti, drURIte. Slice 9c.
        dhatupatha: "09.0013",
        code: "drU",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 09.0041 `BrI\\` Baye (√bhrī). The `\\` is the root vowel's accent, not
        // an it, so parasmaipadī by 1.3.78. 8.4.2 as for √krī: BrIRAti.
        // Slice 9c.
        dhatupatha: "09.0041",
        code: "BrI",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "Baye",
    },
    Dhatu {
        // 09.0042 `kzI\\z` hiMsAyAm (√kṣīṣ). The final `z` is an it (1.3.3) and
        // marks no pada, so parasmaipadī by 1.3.78. 8.4.2 as for √krī:
        // kzIRAti. Slice 9c.
        dhatupatha: "09.0042",
        code: "kzI",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 09.0051 `mfda~` kzode (√mṛd). Parasmaipadī by 1.3.78. The `d` stands
        // between the `f` and śnā's `n`, so no ṇatva: mfdnAti. Slice 9c.
        dhatupatha: "09.0051",
        code: "mfd",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "kzode",
    },
    Dhatu {
        // 09.0054 `kuza~` nizkarze (√kuṣ). Parasmaipadī by 1.3.78; 8.4.1 on
        // the adjacent `z`, as for √muṣ: kuzRAti. Slice 9c.
        dhatupatha: "09.0054",
        code: "kuz",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "nizkarze",
    },
    Dhatu {
        // 09.0056 `RaBa~` hiMsAyAm (√ṇabh). Stored `naB`: the `R` → `n` of
        // 6.1.65 *ṇo naḥ* is the stored-form convention, as for √ṇij.
        // Parasmaipadī by 1.3.78: naBnAti. Slice 9c.
        dhatupatha: "09.0056",
        code: "naB",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 09.0057 `tuBa~` hiMsAyAm (√tubh). Parasmaipadī by 1.3.78: tuBnAti.
        // Slice 9c.
        dhatupatha: "09.0057",
        code: "tuB",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 09.0060 `u~Drasa~` uYCe (√dhras). Parasmaipadī by 1.3.78: DrasnAti.
        // Shares its upadeśa, code and artha with curādi's `10.0270`, whose
        // udit makes its ṇic optional; kryādi takes no ṇic, and no surface
        // form of the two meets. Slice 9c.
        dhatupatha: "09.0060",
        code: "Dras",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "uYCe",
    },
    Dhatu {
        // 09.0061 `iza~` ABIkzRye (√iṣ). Parasmaipadī by 1.3.78; 8.4.1 as for
        // √kuṣ: izRAti. Vowel-initial, so laṅ takes āṭ (6.4.72) and 6.1.90
        // merges it: EzRAt. Slice 9c.
        dhatupatha: "09.0061",
        code: "iz",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "ABIkzRye",
    },
    Dhatu {
        // 09.0062 `vi\\za~` viprayoge (√viṣ). The `\\` is the root vowel's
        // accent, so parasmaipadī by 1.3.78; 8.4.1 as for √kuṣ: vizRAti.
        // Shares its code with juhotyādi's `03.0014 vi\\zx~^`, a distinct row,
        // and no surface form of the two meets. Slice 9c.
        dhatupatha: "09.0062",
        code: "viz",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "viprayoge",
    },
    Dhatu {
        // 09.0063 `pruza~` snehanasevanapUraRezu (√pruṣ). Parasmaipadī by
        // 1.3.78; 8.4.1 as for √kuṣ: pruzRAti. Slice 9c.
        dhatupatha: "09.0063",
        code: "pruz",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "snehanasevanapUraRezu",
    },
    Dhatu {
        // 09.0064 `pluza~` snehanasevanapUraRezu (√pluṣ). Parasmaipadī by
        // 1.3.78; 8.4.1 as for √kuṣ: pluzRAti. Slice 9c.
        dhatupatha: "09.0064",
        code: "pluz",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "snehanasevanapUraRezu",
    },
    Dhatu {
        // 09.0065 `puza~` puzwO (√puṣ). Parasmaipadī by 1.3.78; 8.4.1 as for
        // √kuṣ: puzRAti. Shares its code with curādi's `10.0280 puza~`, a
        // distinct row, and no surface form of the two meets. Slice 9c.
        dhatupatha: "09.0065",
        code: "puz",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "puzwO",
    },
    Dhatu {
        // 09.0067 `Kaca~` BUtaprAdurBAve (√khac). Parasmaipadī by 1.3.78.
        // 8.4.40's converse arm makes śnā's `n` after the `c` a `Y`: KacYAti.
        // Before `hi`, 3.1.83's śānac leaves no `n` there: KacAna. Slice 9c.
        dhatupatha: "09.0067",
        code: "Kac",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "BUtaprAdurBAve",
    },
    Dhatu {
        // 09.0070 `svF` varaRe (√svṝ). Parasmaipadī by 1.3.78. 7.1.102 makes
        // the `F` after the labial `v` `ur`, 8.2.77 lengthens its `u` before
        // the `r`, and 8.4.1 reaches śnā's `n`: svUrRAti. Slice 9c.
        dhatupatha: "09.0070",
        code: "svF",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "varaRe",
    },
"""
anchor = """        dhatupatha: "09.0045",
        code: "vf",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Atmanepada,
        artha: "samBaktO",
    },
"""
assert s.count(anchor) == 1
s = s.replace(anchor, anchor + ROWS)
open(p, 'w').write(s)
print("inserted 22 rows")
```

```bash
cd /workspace/.worktrees/kryadi-9c
python3 /tmp/vidyut-9c/slice9c/rows_9c.py      # inserted 22 rows
```

- [ ] **Step 5: Generate the goldens**

Create `/tmp/vidyut-9c/vidyut-prakriya/examples/kryadi_goldens_9c.rs` if it is missing (sha256 `07101ca771ebd8fadc77d9b239c3569f54c8bcfca4238d40cb25fb16e0931a32`). It is 10m's generator with the row list changed and its paths made relative to the copy it runs in: it derives every cell of the 22 rows in both engines, asserts the form sets equal, and writes the rows the statics want (pinned form = first live branch; each further distinct form an `ALTERNATES` row keyed by its branch's vikalpa ids in log order). Its `VIKALPA_RULES` is `paradigm/main.rs`'s, unchanged by this slice.

```rust
//! THROWAWAY: slice 9c — emit the twenty-two kryādi rows' goldens from the
//! engine, asserting every cell's form set equals vidyut's first. Reads the
//! dhātupāṭha of the checkout it runs in and writes beside that checkout.
use panini::Panini;
use panini_data::{dhatus, Lakara as L, Pada, Purusha as P, Vacana as V};
use vidyut_prakriya::args::{DhatuPada, Lakara, Prayoga, Purusha, Tinanta, Vacana};
use vidyut_prakriya::{Dhatupatha, Vyakarana};
const VIKALPA_RULES: &[&str] = &[
    "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3", "2573.2", "6.1.54", "7.3.37.2", "7.1.35", "3.4.111", "7.3.86", "6.4.107",
    "8.2.74", "8.2.75", "8.4.65", "8.4.56", "6.4.115", "6.4.117", "6.4.116", "6.4.43",
];
/// The twenty-two rows, in the order they sit in `DHATUS`.
const NEW: [&str; 22] = [
    "09.0001", "09.0002", "09.0003", "09.0004", "09.0005", "09.0011", "09.0012", "09.0013",
    "09.0041", "09.0042", "09.0051", "09.0054", "09.0056", "09.0057", "09.0060", "09.0061",
    "09.0062", "09.0063", "09.0064", "09.0065", "09.0067", "09.0070",
];
fn main() {
    let here = env!("CARGO_MANIFEST_DIR");
    let dp = Dhatupatha::from_path(format!("{here}/data/dhatupatha.tsv")).unwrap();
    let v = Vyakarana::builder().build();
    let e = Panini::new();
    let laks = [(L::Lat, Lakara::Lat, "laT"), (L::Lan, Lakara::Lan, "laN"), (L::Lot, Lakara::Lot, "loT"), (L::VidhiLin, Lakara::VidhiLin, "viDiliN")];
    let cells = [(P::Prathama, Purusha::Prathama, V::Eka, Vacana::Eka), (P::Prathama, Purusha::Prathama, V::Dvi, Vacana::Dvi), (P::Prathama, Purusha::Prathama, V::Bahu, Vacana::Bahu),
                 (P::Madhyama, Purusha::Madhyama, V::Eka, Vacana::Eka), (P::Madhyama, Purusha::Madhyama, V::Dvi, Vacana::Dvi), (P::Madhyama, Purusha::Madhyama, V::Bahu, Vacana::Bahu),
                 (P::Uttama, Purusha::Uttama, V::Eka, Vacana::Eka), (P::Uttama, Purusha::Uttama, V::Dvi, Vacana::Dvi), (P::Uttama, Purusha::Uttama, V::Bahu, Vacana::Bahu)];
    let (mut par, mut alt) = (String::new(), String::new());
    let (mut ncells, mut nforms, mut nalt, mut ndiff) = (0, 0, 0, 0);
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
                    // The pinned form is the first live branch.
                    let first = live[0].text();
                    row.push(first.clone());
                    let mut seen = vec![first];
                    for br in &live {
                        let f = br.text();
                        if seen.contains(&f) { continue; }
                        seen.push(f.clone());
                        nalt += 1;
                        let key: Vec<&str> = br.log.iter().map(|s| s.sutra.as_str()).filter(|s| VIKALPA_RULES.contains(s)).collect();
                        alt.push_str(&format!("    (\n        \"{num}\",\n        \"{lname}\",\n        Pada::{pname},\n        {i},\n        \"{f}\",\n        \"{}\",\n    ),\n", key.join("+")));
                    }
                }
                par.push_str(&format!("    (\n        \"{num}\",\n        \"{lname}\",\n        Pada::{pname},\n        [\n{}        ],\n    ),\n", row.iter().map(|f| format!("            \"{f}\",\n")).collect::<String>()));
            }
        }
    }
    std::fs::write(format!("{here}/../goldens_9c_paradigm.rs"), par).unwrap();
    std::fs::write(format!("{here}/../goldens_9c_alternates.rs"), alt).unwrap();
    println!("{ncells} cells, {nforms} forms, {nalt} alternates, {ndiff} differences");
    assert_eq!(ndiff, 0);
}
```

```bash
cd /tmp/vidyut-9c/vidyut-prakriya
grep -n '^panini' Cargo.toml          # must point at the worktree
mise exec rust@1.99.0 -- cargo run -q --release --example kryadi_goldens_9c 2>/dev/null | tail -1
sha256sum /tmp/vidyut-9c/goldens_9c_paradigm.rs /tmp/vidyut-9c/goldens_9c_alternates.rs
```

Expected: `1080 cells, 1212 forms, 132 alternates, 0 differences`, then:
- `e5abf62001962a9473719412f2603d4a683d056ceed4ea7fe3aae7bb5a8e4785  /tmp/vidyut-9c/goldens_9c_paradigm.rs`
- `411496f02ddef80d8a98bc8392c7367d04df3e6a01927dc1c3bc612995804035  /tmp/vidyut-9c/goldens_9c_alternates.rs`

Use `mise exec rust@1.99.0 --` (the vidyut checkout has no `mise.toml`) and never `mise -C`. If a line differs, stop and report.

- [ ] **Step 6: Insert the goldens and check they landed verbatim**

Create `/tmp/vidyut-9c/slice9c/insert_goldens_9c.py` (sha256 `54ad6a968e8af1fa482fa27c29139f7e334d16d7910f7dd868785a62f9976333`) and `/tmp/vidyut-9c/slice9c/check_goldens_9c.py` (sha256 `0a0d0ac989c0ae31df1fa9443d89a9ddad5bd4be3aa017f802f900f5d2a50a2c`) if they are missing:

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 9c — insert the generator's goldens before each kryādi
static's closing `];`. Reads them from the private vidyut copy ($V9C,
default /tmp/vidyut-9c). Run from the worktree root."""
import os
v = os.environ.get('V9C', '/tmp/vidyut-9c')
p = 'crates/panini/tests/paradigm/data/kryadi.rs'
s = open(p).read()
par = open(f'{v}/goldens_9c_paradigm.rs').read()
alt = open(f'{v}/goldens_9c_alternates.rs').read()
assert s.count('pub const ALTERNATES') == 1
i = s.index('pub const ALTERNATES')
head, tail = s[:i], s[i:]
k = head.rindex('];'); head = head[:k] + par + head[k:]
k = tail.rindex('];'); tail = tail[:k] + alt + tail[k:]
open(p, 'w').write(head + tail)
print("inserted goldens")
```

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 9c — confirm every generator tuple landed in kryadi.rs
verbatim once rustfmt has run: whitespace is dropped and a trailing `,` before
`)` or `]` is ignored on both sides. Reads the private vidyut copy ($V9C,
default /tmp/vidyut-9c). Run from the worktree root."""
import os, re
v = os.environ.get('V9C', '/tmp/vidyut-9c')
def norm(t):
    t = re.sub(r'\s+', '', t)
    return t.replace(',)', ')').replace(',]', ']')
f = norm(open('crates/panini/tests/paradigm/data/kryadi.rs').read())
total = 0
for name in ('paradigm', 'alternates'):
    g = open(f'{v}/goldens_9c_{name}.rs').read()
    tuples = re.findall(r'\(\s*"09\.\d{4}".*?\n    \),\n', g, re.S)
    missing = [t for t in tuples if norm(t).rstrip(",") not in f]
    assert not missing, f"{name}: {len(missing)} missing, first {missing[0]!r}"
    total += len(tuples)
    print(name, len(tuples), "present")
assert total == 120 + 132, total
```

```bash
cd /workspace/.worktrees/kryadi-9c
python3 /tmp/vidyut-9c/slice9c/insert_goldens_9c.py      # inserted goldens
mise run fmt
python3 /tmp/vidyut-9c/slice9c/check_goldens_9c.py       # paradigm 120 present, alternates 132 present
```

rustfmt folds the short alternates onto one line and drops a trailing comma; the check ignores whitespace and a `,` before `)` or `]` on both sides, so a reflow is not a false "missing tuple".

- [ ] **Step 7: The harness totals, the `entry` control, then the audit**

Create `/tmp/vidyut-9c/slice9c/audit_9c.py` if it is missing (sha256 `9cfce6e3072d29f251d50ba761ce4afb4a9aab1c4da56e36bd8ee6c5f62d1927`). It raises the harness's asserted totals to 616 / 39312 / 51116 and its header's counts (476 both-pada roots, thirty-three of them by 1.3.72: twenty-five plus the eight ñit rows).

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 9c's edits to tools/audit/panini_full_audit.rs — the
asserted totals and the header's counts. Every `old` must occur exactly once;
nothing is written if one fails. Run from the worktree root."""
import sys
E = {
'tools/audit/panini_full_audit.rs': [
("""//! What it compares: for each of the 594 curated roots, for each pada the root
//! admits (`Dhatu::padas`; two apiece for the 468 roots that admit both padas —
//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 428 curādi
""",
"""//! What it compares: for each of the 616 curated roots, for each pada the root
//! admits (`Dhatu::padas`; two apiece for the 476 roots that admit both padas —
//! thirty-three ubhayapadī by 1.3.72, √bhuj by 1.3.66, 428 curādi
"""),
("""//! Corpus invariants, asserted: 594 roots, 38232 cells, 49904 forms. These are
//! facts about the repo, pinned by its own golden suite
//! (`derivation_set_shape_matches_the_audited_numbers`): 4248 root×pada×lakāra
//! blocks × 9 cells, plus 11672 `ALTERNATES` rows. If this harness's
""",
"""//! Corpus invariants, asserted: 616 roots, 39312 cells, 51116 forms. These are
//! facts about the repo, pinned by its own golden suite
//! (`derivation_set_shape_matches_the_audited_numbers`): 4368 root×pada×lakāra
//! blocks × 9 cells, plus 11804 `ALTERNATES` rows. If this harness's
"""),
("""//! Optionally dump the full 38232-cell table:
""",
"""//! Optionally dump the full 39312-cell table:
"""),
("""    assert_eq!(roots_seen.len(), 594, "curated roots");
    assert_eq!(n_cells, 38232, "cells: 4248 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 49904, "forms: 38232 cells + 11672 ALTERNATES rows");
""",
"""    assert_eq!(roots_seen.len(), 616, "curated roots");
    assert_eq!(n_cells, 39312, "cells: 4368 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 51116, "forms: 39312 cells + 11804 ALTERNATES rows");
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
cd /workspace/.worktrees/kryadi-9c
python3 /tmp/vidyut-9c/slice9c/audit_9c.py      # applied 4 edits
WT=/workspace/.worktrees/kryadi-9c
V=/tmp/vidyut-9c/vidyut-prakriya
cp tools/audit/panini_full_audit.rs $V/examples/
grep -n '^panini' $V/Cargo.toml                 # the worktree
(cd $V && PANINI_AUDIT_VIDYUT=/tmp/vidyut-9c PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run -q --release --example panini_full_audit 2>&1 | tail -2; echo "exit ${PIPESTATUS[0]}")
(cd $V && PANINI_AUDIT_VIDYUT=/tmp/vidyut-9c PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run -q --release --example panini_full_audit 2>&1 | tail -9)
date -u +%F
```

- The `entry` control runs first and is mandatory (the prototype skipped it). Expected: `AUDIT FAILED: 36 differing cells.` and a non-zero exit.
- The honest run. Expected: `roots : 616`, `cells : 39312`, `forms (set sizes): 51116`, `live branches : 51116`, `blocked branches : 6552`, `differing cells  : 0`, `AUDIT PASSED: 39312 cells, 51116 forms, zero differences.`

If the control does not fail, or the honest run shows any difference, stop and report, and record nothing. Keep the date `date -u +%F` prints: Task 3 takes it.

- [ ] **Step 8: The prior-trace dump**

Every prior branch, blocked ones included, with every step's before and after, must be byte-identical between the merge-base and this worktree (goldens ignore traces). Create `/tmp/vidyut-9c/vidyut-prakriya/examples/trace_dump_9c.rs` if it is missing (sha256 `6b2a89ac7acf49e8a33092287b476d877f914574a15c9e81b98a8dfd5b44dcf7`). It lists the 22 rows literally so that it also builds against the base:

```rust
//! THROWAWAY: slice 9c — dump every prior cell's branches, blocked ones
//! included, each with its credited-rule log and every step's before/after.
use panini::Panini;
use panini_data::{Lakara as L, Purusha as P, Vacana as V};
/// Slice 9c's twenty-two rows, listed literally so the dump also builds
/// against the base commit.
const NEW: &[&str] = &[
    "09.0001", "09.0002", "09.0003", "09.0004", "09.0005", "09.0011", "09.0012", "09.0013",
    "09.0041", "09.0042", "09.0051", "09.0054", "09.0056", "09.0057", "09.0060", "09.0061",
    "09.0062", "09.0063", "09.0064", "09.0065", "09.0067", "09.0070",
];
fn main() {
    assert_eq!(NEW.len(), 22);
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
WT=/workspace/.worktrees/kryadi-9c
V=/tmp/vidyut-9c/vidyut-prakriya
BASE=/tmp/kryadi-9c-base
test -d $BASE || git -C $WT worktree add --detach $BASE "$(git -C $WT merge-base HEAD origin/main)"
git -C $BASE log --oneline -1                     # the merge-base (b2707e7 unless main moved)
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_9c 2>/dev/null > /tmp/kryadi-9c-dump-branch.txt)
sed -i "s#^panini = { path = .*#panini = { path = \"$BASE/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$BASE/crates/panini-data\" }#" $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml                   # the base
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_9c 2>/dev/null > /tmp/kryadi-9c-dump-base.txt)
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml                   # the worktree again
wc -l < /tmp/kryadi-9c-dump-base.txt; wc -l < /tmp/kryadi-9c-dump-branch.txt   # 56456 and 56456
grep -c "blocked=false" /tmp/kryadi-9c-dump-base.txt                          # 49904
cmp /tmp/kryadi-9c-dump-base.txt /tmp/kryadi-9c-dump-branch.txt && echo PRIOR-TRACES-IDENTICAL
```

Expected: `56456`, `56456`, `49904`, `PRIOR-TRACES-IDENTICAL`. If `cmp` reports a difference, stop and report.

- [ ] **Step 9: Run the suite and read the failures**

Run the suite exactly as Global Constraints says. Expected, and nothing else: `EXIT_CODE=101`, with these four failures —
- `tests::curated_roots_have_expected_ganas_and_padas` (`panini-data`, 28 passed / 1 failed): `left: 616`, `right: 594`;
- `derivation_set_shape_matches_the_audited_numbers` (`paradigm`, 29 / 2): `left: 39312`, `right: 38232`;
- `pada_ambiguous_surfaces_are_exactly_these` (`paradigm`): the set grew by 32;
- `juhotyadi::shcutva_off_jan_is_credited_exactly_as_before_3f3` (`trace`, 216 / 1): `8.4.40 credited on 09.0067`.

Every other binary passes as in Step 2, `roundtrip` and `derivation_set_is_exactly_pinned` included. `dhatupatha_numbers_resolve_upstream` passes: none of the 22 is in a sibling pair. Any other failure means stop and report.

Do not commit: Task 2 commits these changes together with the assertions they move.

---

## Task 2: Every assertion the rows move

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`
- Modify: `crates/panini/tests/trace/kryadi.rs`, `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes: Task 1's rows and goldens; `PARADIGM`, `ALTERNATES`, `dhatus()`, `Panini::check`, `panini::Analysis` (`dhatu`, `pada`, `trace`); the trace helpers `trace_for` and `credited`.
- Produces: the tests `kryadi_rows_are_the_twenty_eight_curated_roots` (`panini-data`), `kryadi_analyses_its_9c_forms` (`paradigm`), `krinati_trace_takes_intervening_natva_on_a_nyit_root`, `krinite_trace_is_the_ubhayapada_atmanepada_shna_path` and `khacnati_trace_takes_the_converse_shcutva` (`trace`).

The spec asks for "the kryādi row-list test, renamed for its count". There is none to rename: kryādi's six rows were asserted only inside `curated_roots_have_expected_ganas_and_padas`. This task adds `kryadi_rows_are_the_twenty_eight_curated_roots` on the rudhādi, tanādi, juhotyādi and curādi pattern and leaves the older assertions as they are.

- [ ] **Step 1: The failing tests are Task 1's four**

Task 1 Step 9 ran them: the four tests above fail and nothing else does. That is this task's "red": three counts or rosters the rows moved, and one set the goldens grew.

- [ ] **Step 2: The assertions and the new tests**

Create `/tmp/vidyut-9c/slice9c/assertions_9c.py` if it is missing (sha256 `24f0587ef50e45d48ca3d3f63d254340300d94f3260e39c4adedec89891598fe`). By file:
- **`panini-data`:** `Dhatu::pada`'s census (124 of these 616: the non-curādi rows but √bhuj), the test-coverage sentence (616), the marker doc (82 of the 616 carry a `\`, 61 of those on a root vowel with no `~\`), the row count (616), and `kryadi_rows_are_the_twenty_eight_curated_roots`.
- **`paradigm/main.rs`:** the census (39312 / 4368; ones 30760, twos 6834, threes 971 with a 9c clause; `ALTERNATES` 11804; `8.4.56` / `7.1.35` / `7.1.35+8.4.56` 942 / 934 / 934) and its 9c paragraph; `kryadi_analyses_its_9c_forms`, appended.
- **`trace/kryadi.rs`:** the three pins, appended.
- **`trace/juhotyadi.rs`:** the 8.4.40 roster gains `09.0067`, its comment names √khac, and √khac's count is pinned at 41 (its 36 cells and six forks, less loṭ madhyama eka's śānac branch, `KacAna`).

`kryadi_analyses_its_9c_forms` reads its forms off the goldens, never off a hand list. It derives the partner rows (other rows with one of the 22 codes) from `dhatus()` and pins that list, so a homograph a later slice adds fails it until it is accounted for.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 9c, Task 2's assertions — every count, roster and list the
twenty-two rows move, the kryādi row-list test, the census and its 9c
paragraph, the √khac roster, three trace pins and the goldens-derived check()
test. Every `old` must occur exactly once in its file; nothing is written if
one fails. Run from the worktree root."""
import sys

KRYADI_ROWS_TEST = """
    #[test]
    fn kryadi_rows_are_the_twenty_eight_curated_roots() {
        // Slice 9a opened gaṇa 9 with √kliś, √gudh and √aś; slice 9b added
        // √muṣ, √vrī and √vṛṅ with 8.4.1 / 8.4.2. Slice 9c adds the twenty-two
        // rows vidyut-prakriya matches cell for cell on the rules already in
        // the pipeline: the eight ñit rows, ubhayapadī by 1.3.72, and fourteen
        // parasmaipadī by 1.3.78. Two codes are not the plain it-stripped
        // upadeśa: `zi\\Y` is stored `si` by 6.1.64, and `RaBa~` is stored `naB`
        // by the 6.1.65 convention √ṇij set. The gaṇa is OPEN at 28 of its 71
        // dhātupāṭha rows.
        let rows: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.gana == Gana::Kryadi)
            .map(|d| (d.dhatupatha, d.code, d.pada))
            .collect();
        assert_eq!(
            rows,
            vec![
                ("09.0058", "kliS", PadaAssignment::Parasmaipada),
                ("09.0053", "guD", PadaAssignment::Parasmaipada),
                ("09.0059", "aS", PadaAssignment::Parasmaipada),
                ("09.0066", "muz", PadaAssignment::Parasmaipada),
                ("09.0040", "vrI", PadaAssignment::Parasmaipada),
                ("09.0045", "vf", PadaAssignment::Atmanepada),
                ("09.0001", "krI", PadaAssignment::Ubhayapada),
                ("09.0002", "prI", PadaAssignment::Ubhayapada),
                ("09.0003", "SrI", PadaAssignment::Ubhayapada),
                ("09.0004", "mI", PadaAssignment::Ubhayapada),
                ("09.0005", "si", PadaAssignment::Ubhayapada),
                ("09.0011", "yu", PadaAssignment::Ubhayapada),
                ("09.0012", "knU", PadaAssignment::Ubhayapada),
                ("09.0013", "drU", PadaAssignment::Ubhayapada),
                ("09.0041", "BrI", PadaAssignment::Parasmaipada),
                ("09.0042", "kzI", PadaAssignment::Parasmaipada),
                ("09.0051", "mfd", PadaAssignment::Parasmaipada),
                ("09.0054", "kuz", PadaAssignment::Parasmaipada),
                ("09.0056", "naB", PadaAssignment::Parasmaipada),
                ("09.0057", "tuB", PadaAssignment::Parasmaipada),
                ("09.0060", "Dras", PadaAssignment::Parasmaipada),
                ("09.0061", "iz", PadaAssignment::Parasmaipada),
                ("09.0062", "viz", PadaAssignment::Parasmaipada),
                ("09.0063", "pruz", PadaAssignment::Parasmaipada),
                ("09.0064", "pluz", PadaAssignment::Parasmaipada),
                ("09.0065", "puz", PadaAssignment::Parasmaipada),
                ("09.0067", "Kac", PadaAssignment::Parasmaipada),
                ("09.0070", "svF", PadaAssignment::Parasmaipada),
            ]
        );
    }
"""

CHECK_TEST = """
/// Slice 9c's `check()` witnesses, read off the goldens rather than listed by
/// hand. Six of its codes are also other gaṇas' rows' (`prI`, `mI`, `yu`,
/// `Dras` and `puz` curādi, `viz` juhotyādi). Number keying keeps the rows
/// apart, but `check` answers with codes, so every form pinned for a 9c row
/// (`PARADIGM` and `ALTERNATES`) must analyse only as that row's code, through
/// 3.1.81's śnā, and no form pinned for a row sharing one of those codes may
/// analyse through śnā at all. The laṭ prathama eka of each row in each of
/// its padas has exactly one analysis, in that pada, and the forms a missed
/// rule would leave (no ṇatva, no ścutva, no 7.1.102, ṇatva across a `d`)
/// derive nothing.
#[test]
fn kryadi_analyses_its_9c_forms() {
    const ROWS: [&str; 22] = [
        "09.0001", "09.0002", "09.0003", "09.0004", "09.0005", "09.0011", "09.0012", "09.0013",
        "09.0041", "09.0042", "09.0051", "09.0054", "09.0056", "09.0057", "09.0060", "09.0061",
        "09.0062", "09.0063", "09.0064", "09.0065", "09.0067", "09.0070",
    ];
    let engine = Panini::new();
    let code_of = |n: &str| dhatus().iter().find(|d| d.dhatupatha == n).unwrap().code;
    let pinned = |n: &str| -> Vec<&'static str> {
        let mut forms: Vec<&'static str> = PARADIGM
            .iter()
            .filter(|(r, _, _, _)| *r == n)
            .flat_map(|(_, _, _, cells)| cells.iter().copied())
            .collect();
        forms.extend(
            ALTERNATES
                .iter()
                .filter(|(r, _, _, _, _, _)| *r == n)
                .map(|(_, _, _, _, f, _)| *f),
        );
        forms
    };
    let shna = |a: &panini::Analysis| a.trace.iter().any(|s| s.sutra == "3.1.81");
    let codes: Vec<&str> = ROWS.iter().map(|n| code_of(n)).collect();
    let mut partners: Vec<(&str, &str)> = dhatus()
        .iter()
        .filter(|d| !ROWS.contains(&d.dhatupatha) && codes.contains(&d.code))
        .map(|d| (d.dhatupatha, d.code))
        .collect();
    partners.sort_unstable();
    assert_eq!(
        partners,
        [
            ("03.0014", "viz"),
            ("10.0235", "yu"),
            ("10.0270", "Dras"),
            ("10.0280", "puz"),
            ("10.0361", "mI"),
            ("10.0373", "prI"),
        ]
    );
    for n in ROWS {
        let code = code_of(n);
        let forms = pinned(n);
        assert!(!forms.is_empty(), "{n} has no goldens");
        for form in forms {
            let r = engine.check(form);
            assert!(matches!(r.verdict, Verdict::Valid), "{n} {form}");
            for a in &r.analyses {
                assert_eq!(a.dhatu, code, "{n} {form}");
                assert!(shna(a), "{n} {form}");
            }
        }
    }
    for (n, code) in &partners {
        let forms = pinned(n);
        assert!(!forms.is_empty(), "{n} has no goldens");
        for form in forms {
            let r = engine.check(form);
            assert!(r.analyses.iter().all(|a| !shna(a)), "{n} {code} {form}");
        }
    }
    let p = Pada::Parasmaipada;
    let a = Pada::Atmanepada;
    for (form, code, pada) in [
        ("krIRAti", "krI", p),
        ("krIRIte", "krI", a),
        ("prIRAti", "prI", p),
        ("prIRIte", "prI", a),
        ("SrIRAti", "SrI", p),
        ("SrIRIte", "SrI", a),
        ("mInAti", "mI", p),
        ("mInIte", "mI", a),
        ("sinAti", "si", p),
        ("sinIte", "si", a),
        ("yunAti", "yu", p),
        ("yunIte", "yu", a),
        ("knUnAti", "knU", p),
        ("knUnIte", "knU", a),
        ("drURAti", "drU", p),
        ("drURIte", "drU", a),
        ("BrIRAti", "BrI", p),
        ("kzIRAti", "kzI", p),
        ("mfdnAti", "mfd", p),
        ("kuzRAti", "kuz", p),
        ("naBnAti", "naB", p),
        ("tuBnAti", "tuB", p),
        ("DrasnAti", "Dras", p),
        ("izRAti", "iz", p),
        ("vizRAti", "viz", p),
        ("pruzRAti", "pruz", p),
        ("pluzRAti", "pluz", p),
        ("puzRAti", "puz", p),
        ("KacYAti", "Kac", p),
        ("svUrRAti", "svF", p),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), 1, "{form}");
        assert_eq!(r.analyses[0].dhatu, code, "{form}");
        assert_eq!(r.analyses[0].pada, pada, "{form}");
    }
    for form in ["krInAti", "KacnAti", "svFRAti", "mfdRAti"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
"""

TRACE_PINS = """
#[test]
fn krinati_trace_takes_intervening_natva_on_a_nyit_root() {
    // krI P laT 3sg: the r, then the root's own I, then SnA's n -> 8.4.2, not
    // 8.4.1, as for vrIRAti. `qukrI\\Y` is Yit, but this is its parasmaipada
    // cell, so 1.3.78 credits and 1.3.72 does not.
    let t = trace_for("krIRAti");
    assert!(t.contains(&"3.1.81".to_string()), "got {t:?}");
    assert!(t.contains(&"8.4.2".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.1".to_string()), "got {t:?}");
    assert!(t.contains(&"1.3.78".to_string()), "got {t:?}");
    assert!(!t.contains(&"1.3.72".to_string()), "got {t:?}");
}

#[test]
fn krinite_trace_is_the_ubhayapada_atmanepada_shna_path() {
    // krI A laT 3sg: 1.3.72 sanctions the atmanepada, te is apit -> Nit
    // (1.2.4) and consonant-initial -> 6.4.113 gives nI, and 8.4.2 retroflexes
    // across the root's I.
    let t = trace_for("krIRIte");
    assert!(t.contains(&"1.3.72".to_string()), "got {t:?}");
    assert!(!t.contains(&"1.3.78".to_string()), "got {t:?}");
    assert!(t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(t.contains(&"8.4.2".to_string()), "got {t:?}");
}

#[test]
fn khacnati_trace_takes_the_converse_shcutva() {
    // Kac P laT 3sg: SnA's n follows the root's c, and 8.4.40's converse arm
    // (a Scu, then a stu) makes it Y. Before hi, 3.1.83's SAnac leaves no n
    // after the c, so KacAna takes no 8.4.40.
    let t = trace_for("KacYAti");
    assert!(t.contains(&"3.1.81".to_string()), "got {t:?}");
    assert!(t.contains(&"8.4.40".to_string()), "got {t:?}");
    let t = trace_for("KacAna");
    assert!(t.contains(&"3.1.83".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.40".to_string()), "got {t:?}");
}
"""

E = {
'crates/panini-data/src/lib.rs': [
("""    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 594
""",
"""    /// `curated_pada_agrees_with_upadesha_markers` re-derives 124 of these 616
"""),
("""    /// The test covers the 594 roots curated here, not the dhātupāṭha's 2259.
""",
"""    /// The test covers the 616 roots curated here, not the dhātupāṭha's 2259.
"""),
("""    /// vendored upadeśa: 73 of the 594 curated roots carry a `\\` at all, and 52
""",
"""    /// vendored upadeśa: 82 of the 616 curated roots carry a `\\` at all, and 61
"""),
("""        assert_eq!(dhatus().len(), 594);
""",
"""        assert_eq!(dhatus().len(), 616);
"""),
("""
    #[test]
    fn curadi_rows_are_the_four_hundred_ninety_one_curated_roots() {
""",
KRYADI_ROWS_TEST + """
    #[test]
    fn curadi_rows_are_the_four_hundred_ninety_one_curated_roots() {
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
/// Slice 9c curates twenty-two kryādi rows on data alone: the eight ñit rows,
/// ubhayapadī by 1.3.72, and fourteen parasmaipadī by 1.3.78. Each forks
/// exactly where √kliś does, laṅ and vidhiliṅ parasmaipada prathama eka on
/// 8.4.56 and the two loṭ tātaṅ cells three ways on 7.1.35/8.4.56, and no
/// ātmanepada cell forks. 1080 new cells, 132 new rows. Kryādi is OPEN at 28
/// of its 71 dhātus.
/// This test is what keeps the numbers true day to day.
"""),
("""    assert_eq!(total_cells, 38232, "4248 root×lakāra blocks × 9 cells each");
""",
"""    assert_eq!(total_cells, 39312, "4368 root×lakāra blocks × 9 cells each");
"""),
("""    assert_eq!(ones, 29768, "one-form cells");
    assert_eq!(twos, 6790, "two-form cells");
    assert_eq!(
        threes, 927,
""",
"""    assert_eq!(ones, 30760, "one-form cells");
    assert_eq!(twos, 6834, "two-form cells");
    assert_eq!(
        threes, 971,
"""),
("""         slice 10l — seven of its eight rows' the same way; and — new in slice 10m — √picc's"
""",
"""         slice 10l — seven of its eight rows' the same way; and — new in slice 10m — √picc's; \\
         and — new in slice 9c — the twenty-two kryādi rows' two loṭ tātaṅ cells, by \\
         7.1.35/8.4.56"
"""),
("""    assert_eq!(ALTERNATES.len(), 11672, "ALTERNATES row count");
""",
"""    assert_eq!(ALTERNATES.len(), 11804, "ALTERNATES row count");
"""),
("""    assert_eq!(key_count("8.4.56"), 898, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 890, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 890, "7.1.35+8.4.56 alternates");
""",
"""    assert_eq!(key_count("8.4.56"), 942, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 934, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 934, "7.1.35+8.4.56 alternates");
"""),
],
'crates/panini/tests/trace/juhotyadi.rs': [
("""    // slice 10i √vich (`viC`), whose root-internal tuk the sanādi 6.1.73
    // gives on every branch (vicCayati, vicCAyati), and slice 10k the two
    // ch-initial plain rows (`Card`, `Cuw`) the adanta way (acCardayat) and
    // √pich (`piC`) the √vich way (picCayati), and slice 10l √mlecch
    // (`mleC`), whose tuk the sanādi 6.1.75 gives (mlecCayati).
    let hits = credited("8.4.40");
""",
"""    // slice 10i √vich (`viC`), whose root-internal tuk the sanādi 6.1.73
    // gives on every branch (vicCayati, vicCAyati), and slice 10k the two
    // ch-initial plain rows (`Card`, `Cuw`) the adanta way (acCardayat) and
    // √pich (`piC`) the √vich way (picCayati), and slice 10l √mlecch
    // (`mleC`), whose tuk the sanādi 6.1.75 gives (mlecCayati). Slice 9c
    // gives the converse arm its second root, kryādi's √khac (`Kac`), whose
    // `c` makes śnā's `n` a `Y` (KacYAti).
    let hits = credited("8.4.40");
"""),
("""                "10.0171", "10.0352", "10.0354", "10.0370", "10.0304", "10.0078", "10.0462",
                "10.0061", "10.0170"
            ]
""",
"""                "10.0171", "10.0352", "10.0354", "10.0370", "10.0304", "10.0078", "10.0462",
                "10.0061", "10.0170", "09.0067"
            ]
"""),
("""    let curadi = off_jan.iter().filter(|(n, _)| n.starts_with("10.")).count();
    assert_eq!(off_jan.len() - curadi, 54);
""",
"""    let curadi = off_jan.iter().filter(|(n, _)| n.starts_with("10.")).count();
    let khac = off_jan.iter().filter(|(n, _)| *n == "09.0067").count();
    assert_eq!(off_jan.len() - curadi - khac, 54);
    // √khac: every live branch, its 36 cells and six forks, but loṭ madhyama
    // eka's śānac one (KacAna), where no `n` follows the `c`.
    assert_eq!(khac, 41);
"""),
],
}
APPEND = {
    'crates/panini/tests/paradigm/main.rs': CHECK_TEST,
    'crates/panini/tests/trace/kryadi.rs': TRACE_PINS,
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
for path, text in APPEND.items():
    s = out.get(path, open(path).read())
    if not s.endswith("}\n"):
        bad.append(f"{path}: does not end with a closing brace")
        continue
    out[path] = s + text
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
for path, s in out.items():
    open(path, 'w').write(s)
print(f"applied {sum(len(e) for e in E.values()) + len(APPEND)} edits")
```

```bash
cd /workspace/.worktrees/kryadi-9c
python3 /tmp/vidyut-9c/slice9c/assertions_9c.py      # applied 16 edits
mise run fmt
```

- [ ] **Step 3: Pin the measured pada-ambiguous set**

The set is measured, never hand-picked: run the test against the old set and read the real one off its failure. Create `/tmp/vidyut-9c/slice9c/pin_ambiguous_9c.py` if it is missing (sha256 `19f815a02394cb61a9d985b21f2629b332236a877d66d3ba3d7f23d8208cf3af`). It refuses a set whose size or hash differs from the replay's.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 9c — pin the measured pada-ambiguous set ($SET, the
failing test's `left:` JSON) into pada_ambiguous_surfaces_are_exactly_these,
and extend the comment that accounts for it. Run from the worktree root."""
import hashlib, json, os
amb = json.load(open(os.environ['SET']))
assert len(amb) == 1667, len(amb)
h = hashlib.sha256('\n'.join(amb).encode()).hexdigest()
assert h == '9da807257158cb88b6d1de75d67bdca4735e2a2bff914e16dadd795813e88f4c', h
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
old = """    // Slice 10m's √picc contributes the same four (`apiccayata`,
    // `piccayatAm`, `piccayetAm`, `piccayeta`), taking the set from 1631 to
    // 1635.
"""
assert s.count(old) == 1
s = s.replace(old, old + """    // Slice 9c's eight ñit kryādi roots, ubhayapadī by 1.3.72, contribute a
    // new four-surface shape each, √krī's: laṅ ātmanepada prathama eka =
    // parasmaipada madhyama bahu (`akrIRIta`), loṭ ātmanepada prathama eka =
    // parasmaipada prathama dvi (`krIRItAm`), vidhiliṅ ātmanepada prathama
    // eka = loṭ parasmaipada madhyama bahu (`krIRIta`) and vidhiliṅ
    // ātmanepada prathama dvi = parasmaipada prathama dvi (`krIRIyAtAm`) —
    // 32 more, taking the set from 1635 to 1667.
""")
i = s.index("fn pada_ambiguous_surfaces_are_exactly_these")
j = s.index("        both,\n        vec![", i)
k = s.index("        ]", j)
s = s[:j] + "        both,\n        vec![\n" + "".join(f'            "{x}",\n' for x in amb) + s[k:]
open(p, 'w').write(s)
print("pinned", len(amb))
```

```bash
cd /workspace/.worktrees/kryadi-9c
SET="$(mktemp)"
{ mise exec -- cargo test -q -p panini --test paradigm pada_ambiguous 2>&1 || true; } | grep "^  left:" | sed 's/^  left: //' > "$SET"
SET="$SET" python3 /tmp/vidyut-9c/slice9c/pin_ambiguous_9c.py      # pinned 1667
mise run fmt
```

The 32 new surfaces are the eight ñit roots' four each: `akrIRIta`, `krIRItAm`, `krIRIta` and `krIRIyAtAm`, and the same for `prIRI-`, `SrIRI-`, `mInI-`, `sinI-`, `yunI-`, `knUnI-` and `drURI-`.

- [ ] **Step 4: Grep the goldens for the `check()` witnesses**

The test enforces this too; the grep shows the witnesses were read off the goldens first.

```bash
cd /workspace/.worktrees/kryadi-9c
for f in krIRAti krIRIte prIRAti prIRIte SrIRAti SrIRIte mInAti mInIte sinAti sinIte yunAti yunIte knUnAti knUnIte drURAti drURIte \
         BrIRAti kzIRAti mfdnAti kuzRAti naBnAti tuBnAti DrasnAti izRAti vizRAti pruzRAti pluzRAti puzRAti KacYAti svUrRAti \
         krInAti KacnAti svFRAti mfdRAti; do
  printf "%s: %s\n" $f "$(grep -c "\"$f\"" crates/panini/tests/paradigm/data/*.rs | grep -v ':0' | sed 's#.*/##' | tr '\n' ' ')"; done
```

Expected: the first thirty print `kryadi.rs:1` and nothing else; the four Invalid shapes (`krInAti` … `mfdRAti`) print nothing.

- [ ] **Step 5: Run the full suite**

```bash
mise run fmt-check && mise run lint
```

Then the suite exactly as Global Constraints says. Expected: `EXIT_CODE=0`, with `panini-data` 30, `paradigm` 32, `trace` 220, and every other binary as in Task 1 Step 2.

- [ ] **Step 6: Commit**

```bash
git branch --show-current      # kryadi-9c
git add crates/panini-data/src/lib.rs crates/panini/tests/paradigm/data/kryadi.rs crates/panini/tests/paradigm/main.rs \
        crates/panini/tests/trace/kryadi.rs crates/panini/tests/trace/juhotyadi.rs tools/audit/panini_full_audit.rs
git status --short             # nothing else modified
git commit -m "feat(data): kryādi's twenty-two rows on data alone

38232 → 39312 cells, 49904 → 51116 forms, ALTERNATES 11672 → 11804, 594 →
616 roots; pada-ambiguous surfaces 1635 → 1667. No engine change: the eight
ñit rows are ubhayapadī by 1.3.72, fourteen parasmaipadī by 1.3.78, and
√khac's śnā takes 8.4.40's converse arm. Goldens generated cell-by-cell
equal to vidyut; the audit shows zero differences over 616 roots with the
entry control failing on 36 cells; every prior branch's trace is
byte-identical to the merge-base's."
```

---

## Task 3: The doc sweep and the audit record

**Files:**
- Modify: `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`
- Modify: `tools/audit/README.md`
- Modify: `crates/panini/tests/paradigm/main.rs` (doc counts, the alternates doc, the audit chain)
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (one comment number)

**Interfaces:**
- Consumes: Task 1 Step 7's audit date. Produces no symbols.

- [ ] **Step 1: The sweep**

Create `/tmp/vidyut-9c/slice9c/docsweep_9c.py` if it is missing (sha256 `e2db69d2990f6ec8d57a8e54de200f90c97fbcca671ed7ac8e310082c4c5df0d`). It takes the audit's date and writes it into the audit record and AGENTS.md's audit chain. It covers:
- **README.md:** the Scope opening (eight gaṇas fully, kryādi out of that list) and a kryādi sentence at 28 of 71 naming the 9c rows by group; the corpus (616 roots; 8552 of 39312 cells forked; 6834 / 971, with a 9c clause); the both-pada count (476, thirty-three by 1.3.72) and list; the pada-ambiguous count (1667) and its account.
- **AGENTS.md:** the cell count and "eight complete", kryādi's open state in the progress sentence, the fork census (6834 / 971; 11804 + 39312 = 51116), the kryādi paragraph (open at 28 of 71; √vṛṅ still the only ātmanepadī root; what 9d and 9e owe), the audit chain, and the `guna.rs` note's "39312 goldens". Not the mutation paragraphs: Task 5 rewrites those.
- **ARCHITECTURE.md:** the gaṇa coverage sentence (eight fully; kryādi open at 28 of 71); the 7.1.35 / 8.4.56 census (1084 cells across 542 parasmaipada columns, 476 both-pada roots, thirty-three by 1.3.72; 942 / 919 outright; 542 + 74 = 616).
- **`paradigm/main.rs` docs:** `ALTERNATES` 11804; the census doc (39312 / 4368 / 30760 / 6834 / 971, with a 9c clause); the alternates doc's key counts and a 9c fold clause; the audit chain.
- **`guna.rs`:** the 6.1.78 comment's 594-root → 616-root, same line.
- **`tools/audit/README.md`:** the asserted totals and the new top record.

The spec's Doc-sweep bullets map as follows: README Scope (28 of 71, the 9c rows by group) → the README edits; totals everywhere → every file above; ARCHITECTURE and AGENTS.md coverage sentences → the coverage edits; the floor/cap paragraph and the mutation record → Task 5; the sweep greps and anchors → Steps 3 and 4.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 9c's doc sweep — README, AGENTS.md, ARCHITECTURE.md, the
audit README's record and asserted totals, the paradigm docs and guna.rs's
corpus size. Takes the audit's date (`date -u +%F` on the day Task 1 Step 7
ran) as its one argument and writes it where the text says AUDITDATE. Every
`old` must occur exactly once; nothing is written if one fails. Run from the
worktree root."""
import re
import sys
E = {
'README.md': [
("""Finite verbs (*tiṅanta*), ten gaṇas covered, nine of them fully —
*bhvādi* (1, vikaraṇa śap), *divādi* (4, śyan), *tudādi* (6, śa), *adādi*
(2, śap luk'd), *kryādi* (9, śnā), *svādi* (5, śnu) and *rudhādi* (7,
śnam) — plus *tanādi*""",
"""Finite verbs (*tiṅanta*), ten gaṇas covered, eight of them fully —
*bhvādi* (1, vikaraṇa śap), *divādi* (4, śyan), *tudādi* (6, śa), *adādi*
(2, śap luk'd), *svādi* (5, śnu) and *rudhādi* (7,
śnam) — plus *tanādi*"""),
("""8.4.44 *śāt* exemption. *curādi* (10) is **open** at 491 of its 492
""",
"""8.4.44 *śāt* exemption. *kryādi* (9, śnā) is **open** at 28 of its 71
dhātus: √kliś, √gudh and √aś (slice 9a); √muṣ, √vrī and √vṛṅ (slice 9b, behind
8.4.1 / 8.4.2, the engine's first ṇatva); and twenty-two more that slice 9c
curated on the rules already in the pipeline — the eight ñit rows √krī
(*krīṇāti*, *krīṇīte*), √prī, √śrī, √mī, √ṣi (stored `si` by 6.1.64), √yu,
√knū and √drū, the gaṇa's first ubhayapadī roots, by 1.3.72, and fourteen
parasmaipadī by 1.3.78: √bhrī, √kṣīṣ, √mṛd, √kuṣ, √ṇabh (stored `naB`),
√tubh, √dhras, √iṣ, √viṣ, √pruṣ, √pluṣ, √puṣ, √khac (*khacñāti*, the second
root of 8.4.40's converse arm) and √svṝ (*svūrṇāti*, by 7.1.102 and 8.2.77).
*curādi* (10) is **open** at 491 of its 492
"""),
("""curated 594-root set, in four lakāras""",
"""curated 616-root set, in four lakāras"""),
("""both correct — and in fact 8464 of the 38232 cells hold more than one form: 6790
hold two, 927 hold three""",
"""both correct — and in fact 8552 of the 39312 cells hold more than one form: 6834
hold two, 971 hold three"""),
("""new in slice 10m — √picc's, each by
""",
"""new in slice 10m — √picc's, and — new in slice 9c — the twenty-two kryādi
rows', each by
"""),
("""padas — 468 roots that admit both padas in the curated set
(twenty-five ubhayapadī by 1.3.72: √nī, √tud, √rudh, √bhid, √kṣud, √yuj,
√tṛd, √ric, √vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛṇ, √tṛ, √ghṛ,
√kṛ, √dā, √dhā, √bhṛ, √ṇij, √vij and √viṣ; √bhuj by 1.3.66;""",
"""padas — 476 roots that admit both padas in the curated set
(thirty-three ubhayapadī by 1.3.72: √nī, √tud, √rudh, √bhid, √kṣud, √yuj,
√tṛd, √ric, √vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛṇ, √tṛ, √ghṛ,
√kṛ, √dā, √dhā, √bhṛ, √ṇij, √vij and √viṣ, and kryādi's √krī, √prī, √śrī,
√mī, √ṣi, √yu, √knū and √drū; √bhuj by 1.3.66;"""),
("""1635 of the pinned (`PARADIGM`) surfaces are pada-ambiguous""",
"""1667 of the pinned (`PARADIGM`) surfaces are pada-ambiguous"""),
("""homograph `10.0143 cUrRa~` already supplies; slice 10m's √picc adds four
more. The
""",
"""homograph `10.0143 cUrRa~` already supplies; slice 10m's √picc adds four
more; slice 9c's eight ñit kryādi roots add 32 more in a shape of their own,
four each: `akrIRIta` (laṅ ātmanepada prathama eka and parasmaipada madhyama
bahu), `krIRItAm` (loṭ ātmanepada prathama eka and parasmaipada prathama dvi),
`krIRIta` (vidhiliṅ ātmanepada prathama eka and loṭ parasmaipada madhyama
bahu) and `krIRIyAtAm` (vidhiliṅ ātmanepada and parasmaipada prathama dvi).
The
"""),
("""set, all 1635. It is therefore""",
"""set, all 1667. It is therefore"""),
],
'AGENTS.md': [
("""  (`crates/panini/tests/paradigm/`, 38232 cells, ten gaṇas, nine complete —
""",
"""  (`crates/panini/tests/paradigm/`, 39312 cells, ten gaṇas, eight complete —
"""),
("""at 491 after slice 10m curated √picc, narrowing 8.2.30 to a term-final cu, the one dhātu left being √ṣad (`10.0368`), which waits for upasargas —
""",
"""at 491 after slice 10m curated √picc, narrowing 8.2.30 to a term-final cu, the one dhātu left being √ṣad (`10.0368`), which waits for upasargas, and kryādi (9) open at 28 of its 71 dhātus after slice 9c curated twenty-two rows on data alone —
"""),
("""    other forms — a second (6790 cells), a third (927 cells), a fourth
""",
"""    other forms — a second (6834 cells), a third (971 cells), a fourth
"""),
("""    `ALTERNATES` (11672 rows in all, so 38232 + 11672 = 49904 forms total); √bhuj
""",
"""    `ALTERNATES` (11804 rows in all, so 39312 + 11804 = 51116 forms total); √bhuj
"""),
("""    rule analysis; kryādi (gaṇa 9) is now **complete** — six roots across all
    four lakāras, the first gaṇa whose vikaraṇa (śnā) is itself reshaped by
    the ending. √kliś, √gudh, √aś (parasmaipada) landed in slice 9a; √muṣ,
    √vrī (parasmaipada) and √vṛṅ (ātmanepada) landed in slice 9b along with
    8.4.1 / 8.4.2, the engine's first ṇatva. √vṛṅ is the gaṇa's **only**
    ātmanepadī root — every other ātmanepada form in kryādi belongs to an
    ubhayapadī root, and no kryādi ubhayapadī root is curated. The
    ubhayapada slice landed 1.3.72 *svaritañitaḥ* (with rudhādi's √rudh),
    so the pada model no longer stands in their way; whether any given one
    needs phonology of its own is a per-root question nobody has asked yet —
    see `docs/superpowers/specs/2026-07-28-kryadi-gana-design.md`; svādi
""",
"""    rule analysis; kryādi (gaṇa 9) is **open** at 28 of its 71 dhātus across
    all four lakāras, the first gaṇa whose vikaraṇa (śnā) is itself reshaped
    by the ending. √kliś, √gudh, √aś (parasmaipada) landed in slice 9a; √muṣ,
    √vrī (parasmaipada) and √vṛṅ (ātmanepada) landed in slice 9b along with
    8.4.1 / 8.4.2, the engine's first ṇatva. Slice 9c curated twenty-two
    more on the rules already in the pipeline: the eight ñit rows (√krī,
    √prī, √śrī, √mī, √ṣi, √yu, √knū, √drū), the gaṇa's first ubhayapadī
    roots, by 1.3.72, and fourteen parasmaipadī by 1.3.78, among them √khac,
    the second root of 8.4.40's converse arm (*khacñāti*). √vṛṅ is still the
    gaṇa's **only** ātmanepadī root — every other ātmanepada form in kryādi
    belongs to one of those eight. The other forty-three rows need rules:
    7.3.80 *pvādīnāṃ hrasvaḥ* and the other phonology and root specials of
    slice 9d, and 6.4.24 and 3.1.82 for slice 9e — see
    `docs/superpowers/specs/2026-07-28-kryadi-gana-design.md` and
    `docs/superpowers/specs/2026-10-07-kryadi-gana-9c-design.md`; svādi
"""),
("""  entry, 38232 cells / 49904 forms / 594 roots).
""",
"""  entry, 38232 cells / 49904 forms / 594 roots), and that by kryādi 9c's
  (`tools/audit/README.md`'s AUDITDATE 9c entry, 39312 cells / 51116 forms /
  616 roots).
"""),
("""not wrong in kind: 38232 goldens
""",
"""not wrong in kind: 39312 goldens
"""),
],
'docs/ARCHITECTURE.md': [
("""Ten gaṇas are covered, nine of them fully: bhvādi (1), divādi (4),
tudādi (6), adādi (2), kryādi (9), svādi (5), rudhādi (7) and tanādi (8),
""",
"""Ten gaṇas are covered, eight of them fully: bhvādi (1), divādi (4),
tudādi (6), adādi (2), svādi (5), rudhādi (7) and tanādi (8),
"""),
("""√jan; slice 3f3) — and curādi (10), **open** at 491 of its
""",
"""√jan; slice 3f3) — and kryādi (9), **open** at 28 of its 71 dhātus (√kliś, √gudh, √aś; slice 9a; √muṣ, √vrī, √vṛṅ; slice 9b; the eight ñit rows, ubhayapadī by 1.3.72, and fourteen parasmaipadī rows, on data alone; slice 9c) — and curādi (10), **open** at 491 of its
"""),
("""forking 1040 cells (loṭ
prathama and madhyama eka across the 520 roots with a parasmaipada column —
""",
"""forking 1084 cells (loṭ
prathama and madhyama eka across the 542 roots with a parasmaipada column —
"""),
("""roots never reach this guard, and the 468 roots that admit both
padas (twenty-five ubhayapadī by 1.3.72 — √rudh, √nī, √tud, √bhid, √kṣud,
√yuj, √tṛd, √ric, √vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛṇ, √tṛ, √ghṛṇ,
√kṛ, √dā, √dhā, √bhṛ, √ṇij, √vij and √viṣ — √bhuj by 1.3.66,""",
"""roots never reach this guard, and the 476 roots that admit both
padas (thirty-three ubhayapadī by 1.3.72 — √rudh, √nī, √tud, √bhid, √kṣud,
√yuj, √tṛd, √ric, √vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛṇ, √tṛ, √ghṛṇ,
√kṛ, √dā, √dhā, √bhṛ, √ṇij, √vij and √viṣ, and kryādi's √krī, √prī, √śrī,
√mī, √ṣi, √yu, √knū and √drū — √bhuj by 1.3.66,"""),
("""the parasmaipada columns of √bhṛ, √ṇij, √vij and √viṣ; 520 + 74 = the 594 curated roots)""",
"""the parasmaipada columns of √bhṛ, √ṇij, √vij and √viṣ; 542 + 74 = the 616 curated roots)"""),
("""utterance, forking 898 cells outright: laṅ and vidhiliṅ prathama eka across
those same 520 parasmaipada columns (875 of them""",
"""utterance, forking 942 cells outright: laṅ and vidhiliṅ prathama eka across
those same 542 parasmaipada columns (919 of them"""),
("""and 10l's other than √kṛp (likewise), √dhras on its ṇic branch, and 10m's √picc;""",
"""and 10l's other than √kṛp (likewise), √dhras on its ṇic branch, 10m's √picc, and 9c's twenty-two kryādi rows;"""),
("""forking a further 1040 (the same
""",
"""forking a further 1084 (the same
"""),
],
'crates/panini/tests/paradigm/main.rs': [
("""/// `ALTERNATES` is otherwise 11672 bare strings""",
"""/// `ALTERNATES` is otherwise 11804 bare strings"""),
("""/// 38232 cells total (4248 root×lakāra blocks × 9), of which 29768 hold exactly one form, 6790 hold two, 927 hold three (""",
"""/// 39312 cells total (4368 root×lakāra blocks × 9), of which 30760 hold exactly one form, 6834 hold two, 971 hold three ("""),
("""and √picc's,
/// new in slice 10m, each by
""",
"""and √picc's,
/// new in slice 10m, and the twenty-two kryādi rows', new in slice 9c, each by
"""),
("""/// itself has 11672 rows, keyed 898 `8.4.56`, 890 `7.1.35`, 890 `7.1.35+8.4.56`,
""",
"""/// itself has 11804 rows, keyed 942 `8.4.56`, 934 `7.1.35`, 934 `7.1.35+8.4.56`,
"""),
("""/// apiece into `8.4.56`, `7.1.35` and `7.1.35+8.4.56` — √kṛ (slice 8b) adds six more
""",
"""/// apiece into `8.4.56`, `7.1.35` and `7.1.35+8.4.56`, and slice 9c's twenty-two kryādi
/// rows open none and fold 44 apiece into the same three — √kṛ (slice 8b) adds six more
"""),
("""/// re-ran it at the same commit over all 38232 cells / 49904 forms / 594 roots
/// with zero differences, its `entry` negative control verified failing (36
/// √bhū cells). √tṛh joins none of the fork
""",
"""/// re-ran it at the same commit over all 38232 cells / 49904 forms / 594 roots
/// with zero differences, its `entry` negative control verified failing (36
/// √bhū cells), and kryādi 9c's re-ran it at the same commit over all 39312
/// cells / 51116 forms / 616 roots with zero differences, its `entry` negative
/// control verified failing (36 √bhū cells). √tṛh joins none of the fork
"""),
],
'crates/panini-prakriya/src/tinanta/guna.rs': [
("""    // 594-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)
""",
"""    // 616-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)
"""),
],
'tools/audit/README.md': [
("""**It asserts the corpus totals** (594 roots, 38232 cells, 49904 forms) rather than
""",
"""**It asserts the corpus totals** (616 roots, 39312 cells, 51116 forms) rather than
"""),
("""
## Last recorded result

2026-10-06, curādi 10m slice, vidyut
""",
"""
## Last recorded result

AUDITDATE, kryādi 9c slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 39312
cells / 51116 forms / 616 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole kryādi 9c slice: twenty-two rows on the rules
already in the pipeline, with no engine change — the eight ñit rows,
ubhayapadī by 1.3.72, and fourteen parasmaipadī by 1.3.78. Blocked branches
stay at 6552: no kryādi row takes ṇic. The throwaway prototype that scoped the
slice curated all sixty-five remaining kryādi rows; on data alone it found
1863 differing cells, every one on the forty-three rows slices 9d and 9e take,
and none on these twenty-two. A merge-base-vs-branch dump of every prior cell's
branches, blocked ones included, with every step's before and after text, was
byte-identical, all 56456 of them (49904 live).

Totals: 616 = 594 + 22; 39312 = 38232 + 1080 (120 root×pada×lakāra blocks ×
9); 51116 = 49904 + 1080 + 132 new `ALTERNATES` rows (11672 → 11804),
measured via the harness's corpus block, not assumed.

2026-10-06, curādi 10m slice, vidyut
"""),
],
}
if len(sys.argv) != 2 or not re.fullmatch(r'\d{4}-\d{2}-\d{2}', sys.argv[1]):
    sys.exit("usage: docsweep_9c.py YYYY-MM-DD (the audit's date)")
E = {p: [(o, n.replace('AUDITDATE', sys.argv[1])) for o, n in es] for p, es in E.items()}
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
cd /workspace/.worktrees/kryadi-9c
python3 /tmp/vidyut-9c/slice9c/docsweep_9c.py <the date Task 1 Step 7 printed>      # applied 33 edits
mise run fmt
```

- [ ] **Step 2: Check the sweep did not leave old text behind**

```bash
cd /workspace/.worktrees/kryadi-9c
grep -n -i -E "nine of them fully|nine complete|kryādi \(gaṇa 9\) is now \*\*complete\*\*|no kryādi ubhayapadī root" README.md AGENTS.md docs/ARCHITECTURE.md
```

Expected: nothing.

- [ ] **Step 3: Sweep for anything left stale**

Old numerals, spelled-out counts, wrapped counts and rule-scoped counts, across all of `crates/` (tests included) and the docs:

```bash
cd /workspace/.worktrees/kryadi-9c
grep -rn -i -E "\b(594|38232|49904|11672|4248|29768|6790|927|8464|898|890|1040|520|468|875|1635)\b|102 of these|73 of the|52 of those|nine complete|nine of them|ninety-four|six of its|is now \*\*complete\*\* — six|no kryādi ubhayapadī" \
  README.md AGENTS.md docs/ARCHITECTURE.md tools/audit crates --include=*.rs --include=*.md | grep -v "paradigm/data/" | grep -v 'dhatupatha: "'
grep -rn -i -E "kryādi|kryadi" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.rs --include=*.md | grep -v "paradigm/data" \
  | grep -i -E "complete|six roots|uncurated|not curated|no kryādi|later slice"
```

Expected from the first grep, exactly these, all history or Task 5's:
- `tools/audit/README.md`: the 9c record's own arithmetic (`(49904 live)`, `616 = 594 + 22; 39312 = 38232 + 1080`, `51116 = 49904 + 1080 + 132 … (11672 → 11804)`), 10m's record (`38232` / `49904 forms / 594 roots`, `594 = 593 + 1; 38232 = 38160 + 72`, `49904 = 49826 + …(11666 → 11672)`), 10l's `594 differing cells`, 10j's `468` lines and 10g's `4248` lines;
- `AGENTS.md`: the floor paragraph's `measured at 38232 cells` and `38232 cells against 38160` (Task 5 rewrites both), svādi's own `(gaṇa 5) is now **complete** — six roots` (not this slice's), and the audit chain's 10m clause (`38232 cells / 49904 forms / 594 roots), and that by kryādi 9c's`);
- `crates/panini/tests/paradigm/main.rs`: the audit chain's 10m clause (`38232 cells / 49904 forms / 594 roots`), the 3f3 paragraph's `twenty-six of its rows`, 10g's `4248 new cells`, 10j's `468 new cells`, the pada-ambiguous comment's `1635` (twice: 10m's sentence and 9c's), and `kryadi_analyses_its_9c_forms`'s doc (`Six of its codes`);
- `crates/panini/tests/trace/curadi.rs`: `468 in all` (a √ci cell count, not the corpus);
- `crates/panini-lipi/src/slp1.rs`: `\u{927}` (a code point).

The second grep prints exactly one line, the new audit record's `no kryādi row takes ṇic`. Anything else is a miss: fix it, and say so in the commit message.

- [ ] **Step 4: Re-grep the recorded file:line anchors**

AGENTS.md cites two stale engine comments by line. Confirm both still sit where it says:

```bash
cd /workspace/.worktrees/kryadi-9c
grep -n "1872 goldens move" crates/panini-prakriya/src/tinanta/guna.rs      # 2641
grep -n "grammar's 1872 cells" crates/panini-prakriya/src/controller.rs     # 206
grep -n "guna.rs:2641\|controller.rs:206" AGENTS.md | head -3
```

Expected: `2641` and `206`, as AGENTS.md records (this slice's `guna.rs` edit keeps its line count). If either moved, update AGENTS.md's anchor and say so in the commit message.

- [ ] **Step 5: Run the full suite and commit**

```bash
mise run fmt-check && mise run lint
```

Then the suite exactly as Global Constraints says. Expected: as in Task 2 Step 5.

```bash
git branch --show-current      # kryadi-9c
git add -A
git status --short             # only the six files above
git commit -m "docs: 9c's counts, the audit record, and the sweep

Kryādi is open at 28 of its 71 dhātus. Audit: zero differences over 616
roots / 39312 cells / 51116 forms against vidyut 8da2f90, the entry control
failing on 36 cells; every prior branch's trace byte-identical between the
merge-base and the branch (56456, 49904 live)."
```

---

## Task 4: Rebase onto svādi 5b and re-measure

Run this when svādi 5b (`/workspace/.worktrees/svadi-5b`, spec `2026-10-07-svadi-gana-5b-design.md`) has merged to `main`; check with `git fetch origin && git log --oneline origin/main | head -5`. If hetumaṇic has merged too, the same procedure covers it. Svādi 5b adds thirty-two rows, 6.4.24 and 8.4.39's svādi arm, so its totals, docs and mutation record all move under this branch. **Never add the totals up by hand: measure them on the rebased tree.** The arithmetic in Step 3 is only a cross-check.

**Files:** every file Tasks 1–3 touched, as conflicts and re-measured counts require; `crates/panini/tests/paradigm/main.rs` and `crates/panini-data/src/lib.rs` for svādi's `si`.

- [ ] **Step 1: Rebase**

```bash
cd /workspace/.worktrees/kryadi-9c
git branch --show-current            # kryadi-9c
git fetch origin
git rebase origin/main
```

Resolve each conflict by keeping `main`'s text and re-applying this slice's intent on top of it: the rows and goldens are additive; every count becomes the measured combined count (Step 3); every prose clause this slice added is kept beside svādi's. Do not keep a pre-rebase number anywhere. Continue with `git rebase --continue` after each commit's conflicts; the result is the same three commits on top of `origin/main`.

- [ ] **Step 2: Account for svādi's `si`**

Svādi 5b curates `05.0002 zi\Y`, stored `si`, which is also `09.0005`'s code. Check it, then apply the edits below only if it printed `05.0002`:

```bash
grep -n 'dhatupatha: "05.0002"' -A1 crates/panini-data/src/lib.rs
```

Create `/tmp/vidyut-9c/slice9c/rebase_si_9c.py` if it is missing (sha256 `daea360d13ad68c1efdeaa90382fd08fede4f289c58b67c4ddf4203e3532dd4f`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 9c, Task 4 — account for svādi 5b's `05.0002 zi\\Y`,
stored `si` like `09.0005`: the goldens-derived check() test's partner list
and its doc, and `09.0005`'s row comment. Run only if `05.0002` is curated.
Every `old` must occur exactly once; nothing is written if one fails. Run from
the worktree root."""
import sys
E = {
'crates/panini/tests/paradigm/main.rs': [
("""/// hand. Six of its codes are also other gaṇas' rows' (`prI`, `mI`, `yu`,
/// `Dras` and `puz` curādi, `viz` juhotyādi). Number keying keeps the rows
""",
"""/// hand. Seven of its codes are also other gaṇas' rows' (`prI`, `mI`, `yu`,
/// `Dras` and `puz` curādi, `viz` juhotyādi, `si` svādi). Number keying keeps
/// the rows
"""),
("""            ("03.0014", "viz"),
            ("10.0235", "yu"),
""",
"""            ("03.0014", "viz"),
            ("05.0002", "si"),
            ("10.0235", "yu"),
"""),
],
'crates/panini-data/src/lib.rs': [
("""        // 09.0005 `zi\\Y` banDane (√ṣi). Stored `si` per 6.1.64 dhātvādeḥ ṣaḥ
        // saḥ, as `stiG` is. Ubhayapadī by 1.3.72: sinAti, sinIte. Slice 9c.
""",
"""        // 09.0005 `zi\\Y` banDane (√ṣi). Stored `si` per 6.1.64 dhātvādeḥ ṣaḥ
        // saḥ, as `stiG` is. Ubhayapadī by 1.3.72: sinAti, sinIte. Shares
        // upadeśa, code and artha with svādi's `05.0002`, a distinct row, and
        // no surface form of the two meets. Slice 9c.
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
python3 /tmp/vidyut-9c/slice9c/rebase_si_9c.py      # applied 3 edits
mise run fmt
```

- [ ] **Step 3: Re-measure every total**

1. **Goldens.** Re-run the generator (Task 1 Step 5's commands; the dev-deps still point at the worktree). Expected: `1080 cells, 1212 forms, 132 alternates, 0 differences` and the same two sha256s. A different hash means svādi's rules reach a 9c row: stop and report.
2. **The suite.** Run it exactly as Global Constraints says. Each failing count assertion prints the measured value (`left:`); set the assertion and every doc that states it to that value. The pada-ambiguous set is re-pinned from its failure the same way as Task 2 Step 3, by hand (the script's 1667 and hash are pre-rebase); keep both slices' comment paragraphs and make the "taking the set from … to …" chain continuous. Re-run until `EXIT_CODE=0`.
3. **The audit.** Read the harness's totals off a run (the corpus block prints before the asserts), set them in `tools/audit/panini_full_audit.rs` and its header, copy the harness into `/tmp/vidyut-9c` again, then run the `entry` control (must fail on 36 cells) and the honest run (zero differences), as in Task 1 Step 7. Keep the new date.
4. **The prior-trace dump** against the new merge-base: remove the old base worktree (`git worktree remove --force /tmp/kryadi-9c-base`), add it again at `git merge-base HEAD origin/main`, and repeat Task 1 Step 8. Expected: `PRIOR-TRACES-IDENTICAL`, with the live count equal to the forms total `main`'s own audit record states (51724 if svādi 5b merged at its spec's totals) and the line count that plus `main`'s blocked branches.

Cross-check, not a source: if svādi 5b merged at the totals its spec names (626 roots, 39744 cells, 51724 forms, 11980 `ALTERNATES`), this tree measures 648 roots, 40824 cells, 52936 forms and 12112 `ALTERNATES` rows. A mismatch means one of the two slices' measurements is wrong: stop and find which before recording anything.

- [ ] **Step 4: Sweep and record**

Update the audit record (`tools/audit/README.md`: the 9c entry moves to the top again with the new date, totals and dump counts, and its arithmetic is restated against svādi's record), AGENTS.md's audit chain, README, ARCHITECTURE and the `paradigm/main.rs` docs to the measured values. Then run Task 3 Step 3's two greps again with svādi's pre-9c totals substituted for 594 / 38232 / 49904 / 11672, and Task 3 Step 4's anchor check (svādi's engine change may have moved `guna.rs:2641`; if it did and svādi did not update AGENTS.md, update it). Run the suite once more (`EXIT_CODE=0`) and `mise run fmt-check && mise run lint`.

```bash
git branch --show-current      # kryadi-9c
git add -A
git commit -m "docs: 9c re-measured on svādi 5b

<the measured totals, the audit's date and result, the dump counts, and
every conflict resolved by hand>"
```

---

## Task 5: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and the current-record paragraph); `mise.toml` if the cap moves

Run this only when it is this slice's turn in the serial queue: svādi 5b's campaign has finished and its branch has merged, Task 4 is done, and no other `cargo-mutants` process is running on this host (`pgrep -x cargo-mutants` prints nothing). Campaigns never run concurrently here.

Follow AGENTS.md's cargo-mutants protocol. Hazards from this repo's record:
- **Measure, never scale.** The cap must exceed a full uncaught suite run at the parallelism used, under campaign load, and twice the slowest caught phase (the `skip_nic` blow-up). Take the floor by measurement, never by cell count.
- **Every invocation rotates `mutants.out`**, so always pass `-o`, give probes their own `-o`, and copy `outcomes.json` durably before any other invocation.
- **The mise shim fails in background shells.** Use the real binary: `CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`, launched with `eval "$(mise env -s bash)"` for the toolchain.
- **`pgrep -f` matches its own shell.** Wait on `pgrep -x cargo-mutants`.
- **Background shells die at about 60 minutes.** Launch probes and the campaign detached with `env -u CARGO_MUTANTS_JOBS setsid nohup`; the controller polls the detached run (Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`), never a foreground `sleep` loop and never a backgrounded shell. If a run dies, copy its `outcomes.json` aside and resume with the same command plus `--iterate`.

What to expect:
- **No mutant moves.** The slice changes no production code: `cargo mutants --list` for `panini-prakriya` is identical, spans included, to `origin/main`'s, so the non-caught set should equal svādi 5b's recorded set exactly. `--in-diff` over the slice's own diff lists none in `panini-data` or `panini-prakriya`.
- **The suite grows 2.7%** (1080 cells on svādi's), so the floor and the `skip_nic` pair are re-measured, and the cap follows AGENTS.md's rule.

- [ ] **Step 1: Confirm the mutant list is unchanged**

```bash
cd /workspace/.worktrees/kryadi-9c
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
OUT=/home/dev/mutants-records/kryadi-9c        # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
BASE="$(git merge-base HEAD origin/main)"
mise exec -- "$CM" mutants --package panini-prakriya --list -o "$OUT/list-branch" 2>/dev/null > "$OUT/list-branch.txt"
git worktree add --detach /tmp/kryadi-9c-mlist "$BASE"
(cd /tmp/kryadi-9c-mlist && mise trust -q && mise exec -- "$CM" mutants --package panini-prakriya --list -o "$OUT/list-base" 2>/dev/null > "$OUT/list-base.txt")
git worktree remove --force /tmp/kryadi-9c-mlist
wc -l < "$OUT/list-branch.txt"; cmp "$OUT/list-base.txt" "$OUT/list-branch.txt" && echo MUTANT-LIST-IDENTICAL
git diff "$BASE" -- crates/panini-data/src/lib.rs > "$OUT/data.diff"
mise exec -- "$CM" mutants --package panini-data --list --in-diff "$OUT/data.diff" -o "$OUT/data" 2>&1 | tail -1
git diff "$BASE" -- crates/panini-prakriya > "$OUT/prakriya.diff"
mise exec -- "$CM" mutants --package panini-prakriya --list --in-diff "$OUT/prakriya.diff" -o "$OUT/prakriya" 2>&1 | tail -1
```

Expected: `MUTANT-LIST-IDENTICAL`, and both `--in-diff` runs list no mutant (record their last lines verbatim; 10m's data run printed "No mutants to filter"). Anything else: stop and report.

- [ ] **Step 2: Measure the floor**

With nothing else of ours running, twice: `cat /proc/loadavg; time mise run test >/dev/null 2>&1; cat /proc/loadavg` (foreground, timeout 600000 ms; if it outruns, poll as Global Constraints says, with the `time` output in a log). Record both wall clocks, user and sys CPU, and the load averages. Read the previous floor from AGENTS.md's floor paragraph and keep its comparison chain.

- [ ] **Step 3: Locate and probe the uncaught equivalents and the `skip_nic` pair**

```bash
grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /=|skip_nic -> bool with true|in skip_nic" "$OUT/list-branch.txt"
sed -n '/\*\*Current record/,/^    The .* record it replaces/p' AGENTS.md | grep -E "missed.txt|timeout.txt" -A3
```

Take the two equivalents and the permanent hang from the current record's `missed.txt` and `timeout.txt` blocks, at their `--list` spans (before svādi 5b they were `adesha.rs:649:30`, `tripadi.rs:1416:38` in 8.3.13's `apply` — not the `tripadi.rs:217:38` that the same regex also prints — and `tripadi.rs:1729:23`; the `skip_nic` pair `sanadi.rs:60:5` and `60:39`). Write them as `<A>`, `<T1>`, `<T2>`, `<K>` and `<K2>`; never compute them. Then launch the probe detached:

```bash
PROBE=/home/dev/mutants-records/kryadi-9c-probe   # durable, its own -o
mkdir -p "$PROBE"
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 30000 -j 4 -o "$PROBE" \
  --re "adesha.rs:<A>:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" \
  --re "sanadi.rs:<K>:5: replace skip_nic -> bool with true" --re "sanadi.rs:<K2>:39: replace != with == in skip_nic" \
  > "$PROBE/probe.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$PROBE/started"; cat /proc/loadavg > "$PROBE/load.started"
```

The 30000 s probe cap is a measuring ceiling, not the campaign's cap. The regexes also match caught `mod.rs` `derive` mutants; that is expected. When `pgrep -x cargo-mutants` prints nothing, record the end time and load, copy `$PROBE/mutants.out/outcomes.json` to `$PROBE/probe-outcomes.durable.json`, and read each test phase from it. Both equivalents must be MISSED, not TIMEOUT; both `skip_nic` mutants must be CAUGHT. Set the provisional cap to max(the current cap in `mise.toml`, 6 × the longer equivalent, 2 × the longer `skip_nic` phase), rounded up to the next 10 s.

- [ ] **Step 4: Run the campaign detached**

```bash
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout <CAP> -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"; cat /proc/loadavg > "$OUT/load.started"
```

`<CAP>` is Step 3's provisional cap. Run nothing CPU-heavy of ours meanwhile. Expect 8–10 hours (10m's took 8h54m at 13520).

- [ ] **Step 5: Read the outcomes**

When `pgrep -x cargo-mutants` prints nothing:

```bash
date -u +"%F %T UTC" > "$OUT/finished"; cat /proc/loadavg > "$OUT/load.finished"
cp "$OUT/mutants.out/outcomes.json" "$OUT/outcomes.durable.json"
tail -5 "$OUT/campaign.log"
cat "$OUT/mutants.out/missed.txt" "$OUT/mutants.out/timeout.txt"
PREV="$(grep -o '/home/dev/mutants-records/[^ `]*/outcomes.durable.json' AGENTS.md | head -1)"; echo "$PREV"
```

`$PREV` is the record AGENTS.md names as current (svādi 5b's, after Task 4). Expected (exit code 3 is normal when a timeout is present): the per-package totals equal that record's; `missed.txt` holds exactly its two equivalents and `timeout.txt` exactly the permanent ṇatva hang. Then diff the non-caught set against it on the full record, with and without span lines:

```bash
python3 - "$OUT/outcomes.durable.json" "$PREV" <<'PY'
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

Expected: `new: []` and `gone: []` both ways. Write down exactly what prints. If not:
- Any **other timeout** is a suspect survivor the larger suite pushed past the cap. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant the previous record caught means a test that caught it no longer does. Stop and report.

- [ ] **Step 6: Margins and the cap**

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

The cap is max(430, 6 × the longest campaign-load equivalent phase, 2 × the longest caught phase), rounded up to the next 10 s, over both readings of the `skip_nic` `true` mutant (the probe's and the campaign's); it is never lowered on the quieter one, and never below the cap the campaign ran at.
- If that equals the current cap, the cap stays.
- Otherwise change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together (`grep -n "<the current cap>" AGENTS.md mise.toml`).

- [ ] **Step 7: Record it in AGENTS.md**

- **The floor paragraph.** Rewrite the paragraph that opens `**The floor behind the <cap>s cap, measured at <cells> cells`, with Step 2's and Step 3's numbers at the rebased cell count, the load averages, and the cap Step 6 chose. Keep the comparison chain to earlier floors, with the previous record's readings joining it.
- **The current record.** Replace the `**Current record (…).**` paragraph with `**Current record (kryādi 9c, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**;
  - the non-caught set diffed against the previous record on the full record, both ways (Step 5's output);
  - that the slice adds no mutant (Step 1's identical `--list`) and both `--in-diff` outputs verbatim;
  - the `skip_nic` pair's probe and campaign phases;
  - the campaign-load phases and margins;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`, and the probe's at `/home/dev/mutants-records/kryadi-9c-probe/probe-outcomes.durable.json`.

  End it with a pointer to the record it replaces: run `git rev-parse --short HEAD` before committing and write ``The <previous slice> record it replaces: `git show <that hash>:AGENTS.md`.``

- [ ] **Step 8: Commit**

```bash
git branch --show-current      # kryadi-9c
git add AGENTS.md mise.toml
git commit -m "chore: 9c mutation gate — floor and uncaught run re-measured at <cells> cells

<the cap, missed.txt and timeout.txt verbatim, and the non-caught diff
against the previous record>"
```

---

## Task 6: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
cd /workspace/.worktrees/kryadi-9c
mise run fmt-check && mise run lint
```

Then the suite exactly as Global Constraints says (`EXIT_CODE=0`).

```bash
git branch --show-current      # kryadi-9c
git log --oneline origin/main..HEAD   # the spec, the plan, and the task commits
```

- [ ] **Step 2: The final review package**

The whole-branch review reads the slice's diff without the appended goldens, with the histogram algorithm:

```bash
git diff --diff-algorithm=histogram "$(git merge-base HEAD origin/main)"..HEAD -- . ':(exclude)crates/panini/tests/paradigm/data/kryadi.rs' > /tmp/kryadi-9c-review.diff
wc -l /tmp/kryadi-9c-review.diff
```

Give the reviewer that file, the spec and this plan. Check the goldens separately with Task 1 Step 6's `check_goldens_9c.py`.

- [ ] **Step 3: Open the PR**

```bash
git branch --show-current      # kryadi-9c
git push -u origin kryadi-9c
gh pr create --title "kryādi 9c — twenty-two rows on data alone" --body "$(cat <<'BODY'
Slice 9c curates twenty-two kryādi rows that vidyut-prakriya matches cell
for cell on the rules already in the pipeline. No engine change.

- The eight ñit rows (√krī, √prī, √śrī, √mī, √ṣi, √yu, √knū, √drū) are the
  gaṇa's first ubhayapadī roots, by 1.3.72 (*krīṇāti*, *krīṇīte*).
- Fourteen are parasmaipadī by 1.3.78. √khac's śnā takes 8.4.40's converse
  arm (*khacñāti*), so the 8.4.40 roster gains it; √svṝ reaches 7.1.102 and
  8.2.77 (*svūrṇāti*).
- Six codes are other gaṇas' too (and `si` svādi 5b's); a goldens-derived
  `check()` test pins that no form of either side answers as the other.

Kryādi is open at 28 of its 71 dhātus. The audit shows zero divergence
against `8da2f90b` with the entry control failing, a merge-base-vs-branch
dump of every prior branch (blocked ones included) is byte-identical, and
the mutation gate's non-caught set equals the previous record's. The other
forty-three rows need rules: slices 9d and 9e.
BODY
)"
```

- [ ] **Step 4: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`:
   - run `git worktree remove .worktrees/kryadi-9c`;
   - run `git worktree remove --force /tmp/kryadi-9c-base` (`git worktree prune` if the path is gone);
   - delete the local and remote `kryadi-9c` branch;
   - run `rm -rf /tmp/vidyut-9c /tmp/kryadi-9c-*`;
   - run `git pull` on `main`.
   Leave `/workspace/.worktrees/proto-kryadi` and its branch: slices 9d and 9e still need them.

---

## Self-Review

**Spec coverage.**
- Scope (22 rows, codes and padas; 616 / 39312 / 51116 / 11804; blocked 6552) → Task 1 Steps 4–9; Global Constraints' counts.
- Decisions: no engine change and no sibling-pair change → Global Constraints, Task 1 Step 9 (`dhatupatha_numbers_resolve_upstream` passes untouched); the √khac roster → Task 2 Step 2; codes repeating across gaṇas, homograph assertions derived from the goldens, every homograph row grepped → `kryadi_analyses_its_9c_forms` (partner rows derived from `dhatus()` and pinned) and Task 2 Step 4; `si` if 5b lands first → Task 4 Step 2.
- Evidence: the `entry` control the prototype skipped → Task 1 Step 7, before anything is recorded; the prior-trace dump → Task 1 Step 8; the suite's expected failures → Task 1 Step 9.
- Changes, data: rows with artha, the kryādi row-list test (added, not renamed: none existed; see Task 2), `Dhatu::pada`'s census and the row count, the marker check covering all 22 (it walks `dhatus()`) → Tasks 1–2.
- Changes, goldens: paradigm rows and 132 alternates; the census's 9c paragraph and totals → Task 1 Steps 5–6, Task 2 Step 2.
- Changes, tests: *krīṇāti* (8.4.2 across the `I`) and *krīṇīte* (ātmanepada, 6.4.113) trace pins; *khacñāti*; the roster; the goldens-derived homograph checks; the main↔HEAD dump → Task 2 Steps 2–4, Task 1 Step 8.
- Audit (private copy, repointed dev-deps, totals raised, `entry` control, README record) → Task 1 Steps 3, 7; Task 3 Step 1.
- Mutation gate (`--in-diff` data-only; full campaign after svādi 5b on the rebased tree; floor and `skip_nic` re-measured; cap by AGENTS.md's rule; `mise.toml` and AGENTS.md together; every missed and timeout mutant verbatim) → Task 4, Task 5.
- Doc sweep (README Scope 28 of 71 by group; totals everywhere; ARCHITECTURE and AGENTS.md coverage sentences, the floor/cap paragraph and the record; greps for old and spelled-out numerals, wrapped counts and "6 of its 71", all of `crates/`; anchors at final HEAD) → Task 3 Steps 1–4, Task 4 Step 4, Task 5 Step 7.
- Success criteria → Task 1 Steps 5, 7, 8; Task 2 Step 5; Task 5; Task 3 Step 1 (28 of 71).
- Later slices → no task.

**Placeholder scan.** `<A>`, `<T1>`, `<T2>`, `<K>`, `<K2>`, `<CAP>`, `<DATE>`, `<N>`, `<cells>`, `<the current cap>` and the audit date are values the executor measures, each with the command that produces it; Task 4's commit body and re-measured counts are the measurements themselves. No step says "update the tests" without the edit.

**Type consistency.** `kryadi_analyses_its_9c_forms` uses `PARADIGM`/`ALTERNATES` rows as `data/mod.rs` types them, `panini::Analysis`'s `dhatu: String`, `pada: Pada` and `trace: Vec<RuleStep>`; the trace pins use `trace_for(&str) -> Vec<String>`; the roster uses `credited(&str) -> Vec<(&'static str, Gana)>`. The scripts' file names and hashes match between their creation and use; `rebase_si_9c.py`'s `old` strings are the text `assertions_9c.py` and `rows_9c.py` write.

**Review Focus.** Five lines, each with its test in Task 2.
