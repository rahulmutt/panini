# Curādi gaṇa slice 10k Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate the 155 plain obligatory-ṇic curādi rows that need nothing new — ubhayapadī by 1.3.74, across laṭ / laṅ / loṭ / vidhiliṅ — with no engine change. The golden suite goes from 26424 to 37584 cells, and curādi from 327 to 482 of its 509 rows.

**Architecture:** Five tasks:
- **Task 1** checks the worktree and the baseline.
- **Task 2** lands the 155 rows, 1240 golden rows and 930 alternates, the `CONVERGENT_UPADESHA_PAIR` exception for `10.0051 zAntva~` / `10.0052 sAntva~`, and every count, list, roster and `check()` assertion they move. The assertions go in first and fail; the rows and goldens make them pass.
- **Task 3** is the audit, the main-vs-branch prior-trace diff, and the doc sweep.
- **Task 4** is the mutation gate. The mutant list is unchanged, but the suite grows 42%, so the floor and the `skip_nic` probe are re-measured and the cap is set from them.
- **Task 5** finishes the branch.

No production code changes: the only `panini-prakriya` edits are two comments in `sanadi.rs` (line counts kept, so no mutant span moves) and one in `guna.rs`.

**Tech Stack:** Rust 1.99.0, pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, checked out at `/tmp/vidyut-full`.

**Spec:** `docs/superpowers/specs/2026-10-05-curadi-gana-10k-design.md`. Its Scope lists are the slice's scope (37 rows by 7.3.86, 40 by 7.2.116, 78 unchanged). Its Decisions section explains the homographs (21 with earlier rows, 4 in-slice pairs) and the `sAntv` exception. Its Later slices section records what slice 10l owes; nothing from it is in scope here.

**Workspace:** The spec and this plan are on the branch `curadi-10k`, already checked out at `/workspace/.worktrees/curadi-10k`. `/workspace` stays on `main` for the whole slice: Task 3's prior-trace dump builds the main engine from `/workspace/crates`. Every path below is relative to the worktree unless it starts with `/`.

**Provenance.** This slice was built end to end on a throwaway worktree, `/workspace/.worktrees/curadi-10k-proto` (branch `proto-10k-assert`, on the spec commit `81b9834`). Its throwaway commits are `1f3f1d3` (Task 2's assertions), `0be3f51` (rows and goldens) and `b0115b1` (audit and docs); Task 5 deletes it. Every script below was generated from those commits' diffs, or written and run against them, and was checked three ways:
- **Prototype checks.** The prototype's final state:
  - passed the full suite, clippy `-D warnings` and `fmt-check`;
  - its generator found all 11160 new cells equal to vidyut's (`11160 cells, 12090 forms, 0 differences`);
  - the audit at 585 roots / 37584 cells / 49160 forms showed zero differences, with 6516 blocked branches, and the `entry` control failed on 36 cells;
  - a main-vs-prototype dump of every prior branch, blocked ones included (43586 lines, 37070 of them live), was byte-identical;
  - `cargo mutants --list` for `panini-prakriya` was identical to main's, spans included (863 lines), and `--in-diff` over the slice's diff listed no mutant in `panini-prakriya` or `panini-data`.
- **Replay.** A replay ran the scripts below in this plan's order on a fresh worktree at `81b9834` and reproduced the prototype's tree after each task, byte for byte in every tracked file. The replay script is `/tmp/vidyut-full/slice10k/replay_10k.sh`, a verification aid this plan does not need. Task 2 Step 2's failing list and Step 6's per-binary counts are that replay's.
- **Scoping.** The prototype that scoped the slice (spec, Evidence) curated all 164 remaining rows of the range: 594 cells differed, every one on the nine rows slice 10l takes.

The prototype did **not** run the mutation campaign or the `skip_nic` probe. Task 4's numbers are expectations, derived from 10j's record and the unchanged mutant list, and the campaign measures them.

**Throwaway scripts.** Everything under `/tmp/vidyut-full/slice10k/` and the two vidyut examples (`curadi_goldens_10k.rs`, `trace_dump_10k.rs`) never ship. Each is reproduced in full in this plan with its sha256, so it can be recreated if `/tmp` was cleaned. Recreate a file only if it is missing, and check its hash either way (`sha256sum <file>`). `rows_10k.py` reads nothing; `pin_ambiguous_10k.py` reads the failing test's output; the generator and the dump read the engine.

## Global Constraints

- **No engine change.** No `Rule`, guard, constant or order in `panini-prakriya` changes; its only edits are the two `sanadi.rs` comments (6.1.73's module-doc clause and entry comment, line counts unchanged) and `guna.rs`'s corpus size, all in Task 3's doc sweep. If a test can only pass by changing engine code, stop and report.
- **Rows:** exactly the spec's 155, appended to `DHATUS` after 10j's in dhātupāṭha order, each `pada: PadaAssignment::Nic`, each `code` what `stored_form` computes (`rows_10k.py` carries them literally). `OPTIONAL_NIC` does not change (181 entries). The nine 10l rows (`10.0023`, `10.0026`, `10.0037`, `10.0155`, `10.0170`, `10.0175`, `10.0180`, `10.0270`, `10.0278`) and `10.0368 za\da~` stay out.
- **`CONVERGENT_UPADESHA_PAIR = ["10.0051", "10.0052"]`**, a test-only const beside `IDENTICAL_UPSTREAM_PAIR` in `panini-data`'s tests; each is the other's one allowed sibling in `dhatupatha_numbers_resolve_upstream`, and the test pins both upadeśas (`zAntva~`, `sAntva~`), the shared artha (`sAmaprayoge`) and the shared stored form (`sAntv`).
- **Counts after the slice:** 585 roots; 37584 cells (4176 blocks); 49160 forms; `ALTERNATES` 11576; one / two / three-form cells 29188 / 6742 / 911 (fours, fives, sixes, sevens, nines unchanged: 359, 10, 365, 1, 8); keys `8.4.56` 882, `7.1.35` 874, `7.1.35+8.4.56` 874, `7.3.86+8.4.56` 151, `7.3.86+7.1.35` 134, `7.3.86+7.1.35+8.4.56` 134, every other key unchanged; 1.3.74 rows 419; both-pada roots 459; pada-ambiguous pinned surfaces 1603; blocked branches 6516.
- **Pre-existing cells must stay byte-identical, traces included,** with no exception. Regenerate no prior golden.
- **Goldens come from the generator, which asserts engine = vidyut cell by cell, and their sha256 must match this plan's.** **Do not edit a golden to match the engine.** If the generator reports a difference, or a hash differs, stop and report.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit. Run `git branch --show-current` before every commit and the push: it must print `curadi-10k` (a detached HEAD strands commits).
- `mise run test` took 3–5 minutes in the prototype under external load; the `trace` binary alone ran 117–244 s. Run it in the **foreground** with a timeout of 600000 ms; if a run outgrows the 10-minute cap, start it detached with its output in a log file and an `EXIT_CODE=` sentinel, then wait in the same turn with `while kill -0 <PID>; do sleep 30; done`. Never background it and end a turn, never arm a Monitor and end a turn, and never pipe it through `tail`.
- `mise run test -- -p X` does not scope. Scope with `mise exec -- cargo test -p <crate> <filter>`. To see every failing binary at once, use `mise exec -- cargo test --workspace --no-fail-fast` (plain `cargo test` stops at the first failing binary).
- The `cargo-mutants` mise shim fails here. Use the real binary, `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`, under `mise exec --`.
- `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes its `panini` and `panini-data` dev-deps. Repoint them at this worktree before generating or auditing, and back at `/workspace/crates` after. Check with `grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml` every time.
- Never wait on a process with `pgrep -f` (it matches its own shell); use `pgrep -x cargo-mutants` or `kill -0 <pid>`.
- Review packages and diffs exclude the appended goldens (`crates/panini/tests/paradigm/data/curadi.rs`) and use `git diff --diff-algorithm=histogram`: the default algorithm shows appended golden rows as fake deletions.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **The `sAntv` exception absorbing a real mis-keying.** A pinned sibling exception could silently accept a row whose number points at the wrong upadeśa. → Task 2 `dhatupatha_numbers_resolve_upstream` pins the pair's two upadeśas literally, their shared artha and stored form, and that both rows are curated; the exception admits exactly one sibling each, and only for those two numbers.
2. **Homograph answer order.** `trace_for` and the default `check` output answer with the first analysis, which is `dhatus()` order: an earlier slice's row before 10k's. A future reordering of `DHATUS` would silently change which root's trace users see. → Task 2 `curadi_analyses_its_plain_nic_forms` asserts the roots of every homograph witness in analysis order (`mArjayati` = √mṛj then `10.0150`, `pAlayati` = `pAl` then `pal`, …), and the ādhṛṣīya test pins `mArjayati`'s order and that only √mṛj's credits 7.2.114.
3. **Which 6.1.73 a row credits.** √pich takes the *sanādi* 6.1.73; √chard and √chuṭ take the *aṅga* stage's, on laṅ's aṭ. A roster read on the wrong stage would pass vacuously. → Task 2 `the_10i_aya_and_tuk_fire_only_on_their_rows` (`rows_crediting("6.1.73", true)` — the sanādi window — is exactly `10.0061`, `10.0304`) and `shcutva_off_jan_is_credited_exactly_as_before_3f3` (8.4.40 on `10.0061`, `10.0078`, `10.0462`, with the branch count `+ 2 * 19 + 78`).
4. **The ākusmīya partners' padas.** `10.0006`, `10.0034`, `10.0041`, `10.0189` and `10.0438` share only their ātmanepada with an ākusmīya row; their parasmaipada is theirs alone, and `kuwwayatu` stops being a non-form. → Task 2's rewritten `curadi_analyses_its_bulk_akusmiya_forms` (the partner's analyses by 1.3.74, plus `aqepayata`'s parasmaipada madhyama bahu by 1.3.78) and `curadi_analyses_its_plain_nic_forms` (`lakzayati`, `kuwwayati`, `SAWayati`, `qepayati`, `kURayati`: one analysis each).
5. **A plain row given an optional ṇic or the wrong pada.** Nothing in a golden says *why* a form is pinned. → Task 2 `a_kusmad_is_credited_on_exactly_the_akusmiya_cells` (1.3.74's credits on the 419 `Nic` rows and seven `NicUbhayapada` only), `curated_pada_agrees_with_upadesha_markers` (unchanged, now over 585 rows), and every `curadi_analyses_its_plain_nic_forms` analysis asserting 3.1.25 and 1.3.78 and no optional-ṇic id; the forms the slice rules out (`medati`, `jalati`, … no ṇic) are Invalid.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-data/src/lib.rs` | Task 2: 155 rows; `Dhatu::pada`'s and the marker doc's counts; the row count (585), the row-list test (renamed `…_four_hundred_eighty_two_…`), `CONVERGENT_UPADESHA_PAIR` and its pin, the √śraṇ comment |
| `crates/panini/tests/paradigm/data/curadi.rs` | Task 2: 1240 golden rows, 930 alternates |
| `crates/panini/tests/paradigm/main.rs` | Task 2: the census and keys and its 10k paragraph, the pada-ambiguous set (1603), three `check()` tests edited, `curadi_analyses_its_plain_nic_forms` new. Task 3: doc counts, the alternates doc, the audit chain |
| `crates/panini/tests/trace/curadi.rs` | Task 2: the 1.3.74 count, the sanādi 6.1.73 roster. Task 3: module doc |
| `crates/panini/tests/trace/juhotyadi.rs` | Task 2: the 8.3.24 and 8.4.40 rosters |
| `crates/panini-prakriya/src/tinanta/sanadi.rs`, `guna.rs` | Task 3: comments only |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, the 10c and 10j specs | Task 3 |
| `AGENTS.md`, maybe `mise.toml` | Task 4 |

---

## Task 1: The worktree and baseline

**Files:** none. **Interfaces:** none.

- [ ] **Step 1: Check the worktree**

```bash
cd /workspace/.worktrees/curadi-10k
git branch --show-current            # curadi-10k
git log --oneline -3                 # the plan commit, the spec commit 81b9834, then ca67ffd
git status --short                   # empty
git -C /workspace branch --show-current   # main
git -C /workspace log --oneline -1        # ca67ffd, the 10j merge
```

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: everything passes at 26424 cells. `panini-prakriya` reports 437 tests, the `trace` binary 215, `paradigm` 28 and `panini-data` 28 (and `panini` 7, `roundtrip` 1, `panini-analyze` 8, `cli` 5, `panini-lipi` 6).

---

## Task 2: The rows, their goldens, and every assertion they move

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`, `crates/panini/tests/paradigm/data/curadi.rs`
- Modify: `crates/panini/tests/trace/curadi.rs`, `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes: the trace helpers `credited`, `rows_crediting`; `Panini::check` and `panini::Analysis` (`dhatu`, `pada`, `trace`).
- Produces: the 155 `Dhatu` rows; the test-only `CONVERGENT_UPADESHA_PAIR: [&str; 2]`; the test `curadi_analyses_its_plain_nic_forms`.

- [ ] **Step 1: The assertions (failing)**

Create `/tmp/vidyut-full/slice10k/assertions_10k.py` if it is missing (sha256 `ac98e2f4f04e9bfad449b6810967e513909631e8bd656e510830be8b41847873`). By file:
- **`panini-data`:** `Dhatu::pada`'s census (585; 419 `Nic`), the marker doc (73 of the 585), the row count (585), the row-list test renamed `curadi_rows_are_the_four_hundred_eighty_two_curated_roots` with a 10k paragraph and the 155 tuples, `CONVERGENT_UPADESHA_PAIR` with its doc, the sibling check's allowance and comment, its pin, and the √śraṇ comment (`10.0063` is curated now).
- **`paradigm/main.rs`:** the census (37584 / 4176; ones 29188, twos 6742, threes 911; ALTERNATES 11576; `8.4.56` / `7.1.35` / `7.1.35+8.4.56` 882 / 874 / 874; `7.3.86+8.4.56` 151; `7.3.86+7.1.35` and `7.3.86+7.1.35+8.4.56` 134) and its 10k paragraph; the bulk ākusmīya witnesses (five gain their 10k partner's analyses; `kuwwayatu` leaves the non-forms); the optional-ṇic witnesses (`SrARayate` is both `SraR` rows'); the ādhṛṣīya witnesses (`mArjayati` leaves the table for its own two-analysis block); and `curadi_analyses_its_plain_nic_forms`.
- **`trace/curadi.rs`:** the 1.3.74 count (419) and the sanādi 6.1.73 roster (`10.0061`, `10.0304`).
- **`trace/juhotyadi.rs`:** the 8.3.24 list (110, + the ten rows) and its branch total (`+ 10 * 78`), and the 8.4.40 list (+ `10.0078`, `10.0462`, `10.0061`) and its count (`+ 2 * 19 + 78`).

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10k, Task 2's assertions — every count, roster, list,
uniqueness and check() assertion the 155 rows move, and the
`CONVERGENT_UPADESHA_PAIR` exception, before the rows exist. Every `old` must
occur exactly once in its file; nothing is written if one fails. Run from the
worktree root."""
import sys
E = {
'crates/panini-data/src/lib.rs': [
("""    /// Which pada(s) this engine derives for this root. Curated rather than
    /// read from the upadeśa's it-markers — but no longer a *deferral*:
    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 430
    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and
    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed
    /// exception, 264 curādi rows' are 1.3.74's, seven more 1.3.74's with ṇic
    /// and 1.3.72's without (`NicUbhayapada`), 45 ākusmīya rows'
    /// the gaṇasūtra 10.0496's, ten ā-garvīya rows' the gaṇasūtra
""",
"""    /// Which pada(s) this engine derives for this root. Curated rather than
    /// read from the upadeśa's it-markers — but no longer a *deferral*:
    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 585
    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and
    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed
    /// exception, 419 curādi rows' are 1.3.74's, seven more 1.3.74's with ṇic
    /// and 1.3.72's without (`NicUbhayapada`), 45 ākusmīya rows'
    /// the gaṇasūtra 10.0496's, ten ā-garvīya rows' the gaṇasūtra
"""),
("""    /// `docs/superpowers/specs/2026-08-16-pada-audit-design.md`.
    ///
    /// The test covers the 430 roots curated here, not the dhātupāṭha's 2259.
    /// It catches a mis-assigned pada on a root a future slice adds; it does
    /// not make the table self-maintaining.
""",
"""    /// `docs/superpowers/specs/2026-08-16-pada-audit-design.md`.
    ///
    /// The test covers the 585 roots curated here, not the dhātupāṭha's 2259.
    /// It catches a mis-assigned pada on a root a future slice adds; it does
    /// not make the table self-maintaining.
"""),
("""    #[test]
    fn curated_roots_have_expected_ganas_and_padas() {
        assert_eq!(dhatus().len(), 430);
        let bu = dhatus().iter().find(|d| d.dhatupatha == "01.0001").unwrap();
        assert!(matches!(bu.pada, PadaAssignment::Parasmaipada));
""",
"""    #[test]
    fn curated_roots_have_expected_ganas_and_padas() {
        assert_eq!(dhatus().len(), 585);
        let bu = dhatus().iter().find(|d| d.dhatupatha == "01.0001").unwrap();
        assert!(matches!(bu.pada, PadaAssignment::Parasmaipada));
"""),
("""
    #[test]
    fn curadi_rows_are_the_three_hundred_twenty_seven_curated_roots() {
        // Slice 10a opens gaṇa 10 with four roots that need only ṇic
        // (3.1.25), 3.1.32 and the guṇa/vṛddhi before ṇic: √cur (7.3.86),
""",
"""
    #[test]
    fn curadi_rows_are_the_four_hundred_eighty_two_curated_roots() {
        // Slice 10a opens gaṇa 10 with four roots that need only ṇic
        // (3.1.25), 3.1.32 and the guṇa/vṛddhi before ṇic: √cur (7.3.86),
"""),
("""        // ākusmīya, by 10.0496; √ci, the last jñapādi, `NicUbhayapada` with
        // ṇic optional by 2570; and √smiṅ, ātmanepadī under ṇic by Kaumudī
        // 2567. The gaṇa is OPEN at 327 of its 509 dhātupāṭha rows.
        let rows: Vec<_> = dhatus()
            .iter()
""",
"""        // ākusmīya, by 10.0496; √ci, the last jñapādi, `NicUbhayapada` with
        // ṇic optional by 2570; and √smiṅ, ātmanepadī under ṇic by Kaumudī
        // 2567. Slice 10k adds the 155 plain obligatory-ṇic rows that need
        // nothing new: thirty-seven by 7.3.86, forty by 7.2.116 and
        // seventy-eight unchanged before ṇic, all ubhayapadī by 1.3.74. The
        // gaṇa is OPEN at 482 of its 509 dhātupāṭha rows.
        let rows: Vec<_> = dhatus()
            .iter()
"""),
("""                ("10.0275", "cyu", PadaAssignment::Nic),
                ("10.0277", "BU", PadaAssignment::Nic),
            ]
        );
""",
"""                ("10.0275", "cyu", PadaAssignment::Nic),
                ("10.0277", "BU", PadaAssignment::Nic),
                ("10.0006", "lakz", PadaAssignment::Nic),
                ("10.0008", "kud", PadaAssignment::Nic),
                ("10.0012", "mid", PadaAssignment::Nic),
                ("10.0015", "jal", PadaAssignment::Nic),
                ("10.0016", "laj", PadaAssignment::Nic),
                ("10.0017", "pIq", PadaAssignment::Nic),
                ("10.0018", "naw", PadaAssignment::Nic),
                ("10.0019", "SraT", PadaAssignment::Nic),
                ("10.0020", "baD", PadaAssignment::Nic),
                ("10.0021", "banD", PadaAssignment::Nic),
                ("10.0024", "pakz", PadaAssignment::Nic),
                ("10.0025", "varR", PadaAssignment::Nic),
                ("10.0027", "praT", PadaAssignment::Nic),
                ("10.0028", "pfT", PadaAssignment::Nic),
                ("10.0029", "paT", PadaAssignment::Nic),
                ("10.0030", "sanb", PadaAssignment::Nic),
                ("10.0031", "Sanb", PadaAssignment::Nic),
                ("10.0032", "sAnb", PadaAssignment::Nic),
                ("10.0034", "kuww", PadaAssignment::Nic),
                ("10.0035", "puww", PadaAssignment::Nic),
                ("10.0036", "cuww", PadaAssignment::Nic),
                ("10.0038", "suww", PadaAssignment::Nic),
                ("10.0039", "lunw", PadaAssignment::Nic),
                ("10.0040", "lunW", PadaAssignment::Nic),
                ("10.0041", "SaW", PadaAssignment::Nic),
                ("10.0042", "SvaW", PadaAssignment::Nic),
                ("10.0044", "tuj", PadaAssignment::Nic),
                ("10.0046", "pij", PadaAssignment::Nic),
                ("10.0050", "pis", PadaAssignment::Nic),
                ("10.0051", "sAntv", PadaAssignment::Nic),
                ("10.0052", "sAntv", PadaAssignment::Nic),
                ("10.0053", "Svalk", PadaAssignment::Nic),
                ("10.0054", "valk", PadaAssignment::Nic),
                ("10.0055", "snih", PadaAssignment::Nic),
                ("10.0056", "sPiw", PadaAssignment::Nic),
                ("10.0057", "smiw", PadaAssignment::Nic),
                ("10.0059", "Sliz", PadaAssignment::Nic),
                ("10.0061", "piC", PadaAssignment::Nic),
                ("10.0063", "SraR", PadaAssignment::Nic),
                ("10.0064", "taq", PadaAssignment::Nic),
                ("10.0065", "Kaq", PadaAssignment::Nic),
                ("10.0078", "Card", PadaAssignment::Nic),
                ("10.0079", "pust", PadaAssignment::Nic),
                ("10.0080", "bust", PadaAssignment::Nic),
                ("10.0081", "cud", PadaAssignment::Nic),
                ("10.0082", "nakk", PadaAssignment::Nic),
                ("10.0083", "Dakk", PadaAssignment::Nic),
                ("10.0084", "cakk", PadaAssignment::Nic),
                ("10.0085", "cukk", PadaAssignment::Nic),
                ("10.0086", "kzal", PadaAssignment::Nic),
                ("10.0087", "tal", PadaAssignment::Nic),
                ("10.0088", "tul", PadaAssignment::Nic),
                ("10.0089", "dul", PadaAssignment::Nic),
                ("10.0090", "pul", PadaAssignment::Nic),
                ("10.0091", "cul", PadaAssignment::Nic),
                ("10.0092", "mUl", PadaAssignment::Nic),
                ("10.0093", "kal", PadaAssignment::Nic),
                ("10.0094", "vil", PadaAssignment::Nic),
                ("10.0095", "bil", PadaAssignment::Nic),
                ("10.0096", "til", PadaAssignment::Nic),
                ("10.0097", "cal", PadaAssignment::Nic),
                ("10.0098", "pAl", PadaAssignment::Nic),
                ("10.0099", "pal", PadaAssignment::Nic),
                ("10.0100", "lUz", PadaAssignment::Nic),
                ("10.0101", "Sulb", PadaAssignment::Nic),
                ("10.0102", "SUrp", PadaAssignment::Nic),
                ("10.0103", "cuw", PadaAssignment::Nic),
                ("10.0104", "muw", PadaAssignment::Nic),
                ("10.0109", "vraj", PadaAssignment::Nic),
                ("10.0110", "Sulk", PadaAssignment::Nic),
                ("10.0115", "Svart", PadaAssignment::Nic),
                ("10.0116", "svart", PadaAssignment::Nic),
                ("10.0117", "SvaBr", PadaAssignment::Nic),
                ("10.0125", "Gaww", PadaAssignment::Nic),
                ("10.0126", "must", PadaAssignment::Nic),
                ("10.0127", "Kaww", PadaAssignment::Nic),
                ("10.0128", "saww", PadaAssignment::Nic),
                ("10.0129", "sPiww", PadaAssignment::Nic),
                ("10.0131", "pul", PadaAssignment::Nic),
                ("10.0132", "pUrR", PadaAssignment::Nic),
                ("10.0133", "puR", PadaAssignment::Nic),
                ("10.0134", "puns", PadaAssignment::Nic),
                ("10.0136", "vyap", PadaAssignment::Nic),
                ("10.0137", "vyay", PadaAssignment::Nic),
                ("10.0138", "pUl", PadaAssignment::Nic),
                ("10.0139", "DUs", PadaAssignment::Nic),
                ("10.0140", "DUz", PadaAssignment::Nic),
                ("10.0141", "DUS", PadaAssignment::Nic),
                ("10.0142", "kIw", PadaAssignment::Nic),
                ("10.0143", "cUrR", PadaAssignment::Nic),
                ("10.0144", "pUj", PadaAssignment::Nic),
                ("10.0145", "ark", PadaAssignment::Nic),
                ("10.0146", "SuW", PadaAssignment::Nic),
                ("10.0148", "juq", PadaAssignment::Nic),
                ("10.0149", "gaj", PadaAssignment::Nic),
                ("10.0150", "mArj", PadaAssignment::Nic),
                ("10.0151", "marc", PadaAssignment::Nic),
                ("10.0154", "tij", PadaAssignment::Nic),
                ("10.0156", "varD", PadaAssignment::Nic),
                ("10.0161", "hlap", PadaAssignment::Nic),
                ("10.0162", "klap", PadaAssignment::Nic),
                ("10.0163", "hrap", PadaAssignment::Nic),
                ("10.0165", "brIs", PadaAssignment::Nic),
                ("10.0167", "il", PadaAssignment::Nic),
                ("10.0168", "mrakz", PadaAssignment::Nic),
                ("10.0169", "ast", PadaAssignment::Nic),
                ("10.0172", "brUs", PadaAssignment::Nic),
                ("10.0173", "barh", PadaAssignment::Nic),
                ("10.0176", "bul", PadaAssignment::Nic),
                ("10.0177", "garj", PadaAssignment::Nic),
                ("10.0178", "gard", PadaAssignment::Nic),
                ("10.0179", "garD", PadaAssignment::Nic),
                ("10.0181", "pUrv", PadaAssignment::Nic),
                ("10.0183", "Iq", PadaAssignment::Nic),
                ("10.0186", "parT", PadaAssignment::Nic),
                ("10.0187", "ruz", PadaAssignment::Nic),
                ("10.0188", "ruw", PadaAssignment::Nic),
                ("10.0189", "qip", PadaAssignment::Nic),
                ("10.0190", "stup", PadaAssignment::Nic),
                ("10.0191", "stUp", PadaAssignment::Nic),
                ("10.0237", "carc", PadaAssignment::Nic),
                ("10.0238", "bukk", PadaAssignment::Nic),
                ("10.0239", "Sabd", PadaAssignment::Nic),
                ("10.0240", "kaR", PadaAssignment::Nic),
                ("10.0242", "sUd", PadaAssignment::Nic),
                ("10.0244", "paS", PadaAssignment::Nic),
                ("10.0245", "am", PadaAssignment::Nic),
                ("10.0246", "caw", PadaAssignment::Nic),
                ("10.0247", "sPuw", PadaAssignment::Nic),
                ("10.0248", "Gaw", PadaAssignment::Nic),
                ("10.0250", "arj", PadaAssignment::Nic),
                ("10.0252", "krand", PadaAssignment::Nic),
                ("10.0253", "las", PadaAssignment::Nic),
                ("10.0256", "mokz", PadaAssignment::Nic),
                ("10.0257", "arh", PadaAssignment::Nic),
                ("10.0259", "Baj", PadaAssignment::Nic),
                ("10.0261", "yat", PadaAssignment::Nic),
                ("10.0262", "rak", PadaAssignment::Nic),
                ("10.0263", "lag", PadaAssignment::Nic),
                ("10.0264", "raG", PadaAssignment::Nic),
                ("10.0265", "rag", PadaAssignment::Nic),
                ("10.0268", "mud", PadaAssignment::Nic),
                ("10.0269", "tras", PadaAssignment::Nic),
                ("10.0271", "uDras", PadaAssignment::Nic),
                ("10.0272", "muc", PadaAssignment::Nic),
                ("10.0273", "vas", PadaAssignment::Nic),
                ("10.0274", "car", PadaAssignment::Nic),
                ("10.0276", "cyus", PadaAssignment::Nic),
                ("10.0397", "raNg", PadaAssignment::Nic),
                ("10.0414", "Keq", PadaAssignment::Nic),
                ("10.0438", "kUR", PadaAssignment::Nic),
                ("10.0457", "kart", PadaAssignment::Nic),
                ("10.0462", "Cuw", PadaAssignment::Nic),
                ("10.0470", "karR", PadaAssignment::Nic),
                ("10.0491", "ruW", PadaAssignment::Nic),
            ]
        );
"""),
("""    /// `~^` a svarita it — whereas a `\\` sitting directly on a vowel elsewhere
    /// is the ROOT's own accent and says nothing about pada. Counted off the
    /// vendored upadeśa: 73 of the 430 curated roots carry a `\\` at all, and 52
    /// of those carry one on a root vowel — `01.0642 ji\\`, `01.1082 smf\\` and
    /// `02.0001 a\\da~` among them — so conflating the two does not fail
""",
"""    /// `~^` a svarita it — whereas a `\\` sitting directly on a vowel elsewhere
    /// is the ROOT's own accent and says nothing about pada. Counted off the
    /// vendored upadeśa: 73 of the 585 curated roots carry a `\\` at all, and 52
    /// of those carry one on a root vowel — `01.0642 ji\\`, `01.1082 smf\\` and
    /// `02.0001 a\\da~` among them — so conflating the two does not fail
"""),
("""    const IDENTICAL_UPSTREAM_PAIR: [&str; 2] = ["10.0291", "10.0327"];

    #[test]
    fn dhatupatha_numbers_resolve_upstream() {
""",
"""    const IDENTICAL_UPSTREAM_PAIR: [&str; 2] = ["10.0291", "10.0327"];

    /// The one pair of curated rows whose upadeśas differ but which store the
    /// same code with the same artha: `zAntva~` (`10.0051`) and `sAntva~`
    /// (`10.0052`), both *sAmaprayoge*. 6.1.64 dhātvādeḥ ṣaḥ saḥ makes the
    /// first `sAntv` too. vidyut-prakriya derives both, so both are curated
    /// (slice 10k), and each is the other's one allowed sibling in
    /// `dhatupatha_numbers_resolve_upstream`.
    const CONVERGENT_UPADESHA_PAIR: [&str; 2] = ["10.0051", "10.0052"];

    #[test]
    fn dhatupatha_numbers_resolve_upstream() {
"""),
("""            // curādi rows apart (the engine keys `OPTIONAL_NIC` by number
            // there alone), so every other gaṇa compares no verdict. The two
            // rows of `IDENTICAL_UPSTREAM_PAIR` differ in nothing, so each has
            // the other as its one allowed sibling.
            let gana_prefix = &d.dhatupatha[..2];
            let verdict = |n: &str, u: &str| {
""",
"""            // curādi rows apart (the engine keys `OPTIONAL_NIC` by number
            // there alone), so every other gaṇa compares no verdict. The two
            // rows of `IDENTICAL_UPSTREAM_PAIR` differ in nothing, and the two
            // of `CONVERGENT_UPADESHA_PAIR` only in what 6.1.64 levels, so each
            // has the other as its one allowed sibling.
            let gana_prefix = &d.dhatupatha[..2];
            let verdict = |n: &str, u: &str| {
"""),
("""                })
                .count();
            let allowed = if IDENTICAL_UPSTREAM_PAIR.contains(&d.dhatupatha) {
                2
            } else {
""",
"""                })
                .count();
            let allowed = if IDENTICAL_UPSTREAM_PAIR.contains(&d.dhatupatha)
                || CONVERGENT_UPADESHA_PAIR.contains(&d.dhatupatha)
            {
                2
            } else {
"""),
("""            );
        }
        // √śraṇ: `10.0174 SraRu~` and the uncurated `10.0063 SraRa~` share
        // gaṇa, stored form and artha. Only the optional-ṇic verdict tells
        // them apart, so `10.0063` stays a distinct row if it is curated.
        let row = |n: &str| *rows.iter().find(|(m, _, _)| *m == n).unwrap();
        let (_, sran_a, artha_a) = row("10.0063");
""",
"""            );
        }
        // √śraṇ: `10.0174 SraRu~` and `10.0063 SraRa~` (curated since slice
        // 10k) share gaṇa, stored form and artha. Only the optional-ṇic
        // verdict tells them apart, so the two stay distinct rows.
        let row = |n: &str| *rows.iter().find(|(m, _, _)| *m == n).unwrap();
        let (_, sran_a, artha_a) = row("10.0063");
"""),
("""        assert_eq!((u2, a2), (u1, a1));
        for n in IDENTICAL_UPSTREAM_PAIR {
            assert!(dhatus().iter().any(|d| d.dhatupatha == n), "{n}");
        }
""",
"""        assert_eq!((u2, a2), (u1, a1));
        for n in IDENTICAL_UPSTREAM_PAIR {
            assert!(dhatus().iter().any(|d| d.dhatupatha == n), "{n}");
        }
        // `zAntva~` and `sAntva~`: the upadeśas differ only in their first
        // sound, the `z` 6.1.64 makes `s`, and the artha is shared, so both
        // store `sAntv`. Both rows are curated.
        let (_, z, artha_z) = row(CONVERGENT_UPADESHA_PAIR[0]);
        let (_, s, artha_s) = row(CONVERGENT_UPADESHA_PAIR[1]);
        assert_eq!((z, s), ("zAntva~", "sAntva~"));
        assert_eq!(z.strip_prefix('z'), s.strip_prefix('s'));
        assert_eq!((artha_z, artha_s), ("sAmaprayoge", "sAmaprayoge"));
        assert_eq!(stored_form(z), "sAntv");
        assert_eq!(stored_form(s), "sAntv");
        for n in CONVERGENT_UPADESHA_PAIR {
            assert!(dhatus().iter().any(|d| d.dhatupatha == n), "{n}");
        }
"""),
],
'crates/panini/tests/paradigm/main.rs': [
("""/// Slice 10j curates the eight ajanta rows 7.2.115, 6.1.54 and 7.3.36 reach:
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
    let mut threes = 0usize;
    let mut fours = 0usize;
    let mut fives = 0usize;
    let mut sixes = 0usize;
""",
"""/// Slice 10j curates the eight ajanta rows 7.2.115, 6.1.54 and 7.3.36 reach:
/// √ghṛ, √jñā, √cyu and √bhū (`Nic`), which fork exactly where √cur does;
/// √gṛ and √yu, ākusmīya, and √smiṅ, ātmanepadī under ṇic by Kaumudī 2567,
/// one form per cell; and √ci (`NicUbhayapada`), whose every cell holds
/// three readings: the ṇic branch declined (*cayayati*, the pinned form),
/// 6.1.54's (*capayati*) and 2570's ṇic-less one (*cayati*), in both padas.
/// 468 new cells, 186 new rows. The gaṇa is OPEN at 327 of its 509 rows.
///
/// Slice 10k curates the 155 plain obligatory-ṇic rows that need nothing new
/// (thirty-seven take 7.3.86 before ṇic, forty 7.2.116, seventy-eight
/// neither), all `Nic`, which fork exactly where √cur does: laṅ and vidhiliṅ
/// parasmaipada prathama eka on 8.4.56, the two loṭ tātaṅ cells three ways on
/// 7.1.35/8.4.56, the thirty-seven's keys carrying their sanādi 7.3.86 in
/// front. 11160 new cells, 930 new rows. The gaṇa is OPEN at 482 of its 509
/// rows.
/// This test is what keeps the numbers true day to day.
#[test]
fn derivation_set_shape_matches_the_audited_numbers() {
    let total_cells = PARADIGM.len() * 9;
    assert_eq!(total_cells, 37584, "4176 root×lakāra blocks × 9 cells each");

    let mut ones = 0usize;
    let mut twos = 0usize;
    let mut threes = 0usize;
    let mut fours = 0usize;
    let mut fives = 0usize;
    let mut sixes = 0usize;
"""),
("""                6 => sixes += 1,
                7 => sevens += 1,
                9 => nines += 1,
                n => panic!("unexpected {n}-form cell in ({root}, {lakara}, {cell})"),
            }
        }
    }
    assert_eq!(ones, 18648, "one-form cells");
    assert_eq!(twos, 6432, "two-form cells");
    assert_eq!(
        threes, 601,
        "three-form cells — new in slice 3b — √hrī's loṭ prathama and madhyama eka, each by \\
         7.1.35/8.4.56; and — new in slice 3c — √dā's and √dhā's, the same way; and — new in \\
         slice 3c2 — √gā's, the same way; and — new in slice 3d — the six ṛ-roots', the same way; \\
         and — new in slice 3d2 — √ṛ's, the same way; and — new in slice 3e — √ṇij's, √vij's and \\
         √viṣ's, the same way; and — new in slice 3f — √kit's, √tur's, √dhiṣ's and √dhan's, \\
         the same way; and — new in slice 3f2 — √bhas's laṅ madhyama eka (8.2.74 beside 8.4.56) \\
         and its two loṭ tātaṅ cells; and — new in slice 3f3 — √jan's two loṭ tātaṅ cells; \\
         and — new in slice 10a — the four curādi roots' two loṭ tātaṅ cells each, by \\
         7.1.35/8.4.56; and — new in slice 10d — the six jñapādi roots', the same way; and — new \\
         in slice 10e — the eighty-three ubhayapadī adanta roots', the same way; and — new in slice \\
         10f — the optional-ṇic rows' (2564/2570/2573.x beside 7.1.35/8.4.56); and — new in \\
         slice 10h — √dhū's and √prī's every cell that forks on neither 7.1.35 nor 8.4.56 \\
         (10.0498 and 7.3.37.2 beside the ṇic form); and — new in slice 10j — √ghṛ's, √jñā's, \\
         √cyu's and √bhū's two loṭ tātaṅ cells, by 7.1.35/8.4.56, and √ci's every cell that \\
         forks on neither (6.1.54 and 2570 beside the ṇic form)"
    );
    assert_eq!(
        fours, 359,
        "four-form cells — piṣ's loṭ madhyama eka, Siz's loṭ parasmaipada madhyama eka (slice \\
         7d), and — new in slice 8a — fifteen more across the four ik-upadhā roots (kziR, fR, \\
         tfR, GfR), each forking on 7.3.86 alongside the pre-existing rules; and — new in \\
         slice 3b — √bhī's vidhiliṅ prathama eka, forking on 6.4.115 alongside 8.4.56; and — \\
""",
"""                6 => sixes += 1,
                7 => sevens += 1,
                9 => nines += 1,
                n => panic!("unexpected {n}-form cell in ({root}, {lakara}, {cell})"),
            }
        }
    }
    assert_eq!(ones, 29188, "one-form cells");
    assert_eq!(twos, 6742, "two-form cells");
    assert_eq!(
        threes, 911,
        "three-form cells — new in slice 3b — √hrī's loṭ prathama and madhyama eka, each by \\
         7.1.35/8.4.56; and — new in slice 3c — √dā's and √dhā's, the same way; and — new in \\
         slice 3c2 — √gā's, the same way; and — new in slice 3d — the six ṛ-roots', the same way; \\
         and — new in slice 3d2 — √ṛ's, the same way; and — new in slice 3e — √ṇij's, √vij's and \\
         √viṣ's, the same way; and — new in slice 3f — √kit's, √tur's, √dhiṣ's and √dhan's, \\
         the same way; and — new in slice 3f2 — √bhas's laṅ madhyama eka (8.2.74 beside 8.4.56) \\
         and its two loṭ tātaṅ cells; and — new in slice 3f3 — √jan's two loṭ tātaṅ cells; \\
         and — new in slice 10a — the four curādi roots' two loṭ tātaṅ cells each, by \\
         7.1.35/8.4.56; and — new in slice 10d — the six jñapādi roots', the same way; and — new \\
         in slice 10e — the eighty-three ubhayapadī adanta roots', the same way; and — new in slice \\
         10f — the optional-ṇic rows' (2564/2570/2573.x beside 7.1.35/8.4.56); and — new in \\
         slice 10h — √dhū's and √prī's every cell that forks on neither 7.1.35 nor 8.4.56 \\
         (10.0498 and 7.3.37.2 beside the ṇic form); and — new in slice 10j — √ghṛ's, √jñā's, \\
         √cyu's and √bhū's two loṭ tātaṅ cells, by 7.1.35/8.4.56, and √ci's every cell that \\
         forks on neither (6.1.54 and 2570 beside the ṇic form); and — new in slice 10k — the \\
         155 plain obligatory-ṇic rows' two loṭ tātaṅ cells, by 7.1.35/8.4.56"
    );
    assert_eq!(
        fours, 359,
        "four-form cells — piṣ's loṭ madhyama eka, Siz's loṭ parasmaipada madhyama eka (slice \\
         7d), and — new in slice 8a — fifteen more across the four ik-upadhā roots (kziR, fR, \\
         tfR, GfR), each forking on 7.3.86 alongside the pre-existing rules; and — new in \\
         slice 3b — √bhī's vidhiliṅ prathama eka, forking on 6.4.115 alongside 8.4.56; and — \\
"""),
("""         prathama and madhyama eka, three readings (2573.1 ṇic-less, 2573.2 pAta-, 6.4.48 \\
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
    assert_eq!(key_count("8.2.75"), 8, "8.2.75-only alternates");
    assert_eq!(key_count("8.2.74"), 2, "8.2.74-only alternates");
    assert_eq!(key_count("7.1.35+8.4.65"), 16, "7.1.35+8.4.65 alternates");
    assert_eq!(
""",
"""         prathama and madhyama eka, three readings (2573.1 ṇic-less, 2573.2 pAta-, 6.4.48 \\
         pata-) × the tātaṅ triple (7.1.35/8.4.56); and — new in slice 10h — √dhū's and \\
         √prī's, three readings (10.0498 ṇic-less, 7.3.37.2's nuk, 7.2.115's vṛddhi) × the \\
         same triple; and — new in slice 10j — √ci's, three readings (2570 ṇic-less, 6.1.54's \\
         `cA` with 7.3.36's puk, 7.2.115's vṛddhi) × the same triple"
    );

    assert_eq!(ALTERNATES.len(), 11576, "ALTERNATES row count");
    let key_count = |key: &str| {
        ALTERNATES
            .iter()
            .filter(|(_, _, _, _, _, k)| *k == key)
            .count()
    };
    assert_eq!(key_count("8.4.56"), 882, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 874, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 874, "7.1.35+8.4.56 alternates");
    assert_eq!(key_count("3.4.111"), 2, "3.4.111 alternates");
    assert_eq!(key_count("6.4.107"), 72, "6.4.107 alternates");
    assert_eq!(key_count("8.4.65"), 145, "8.4.65-only alternates");
    assert_eq!(key_count("8.2.75"), 8, "8.2.75-only alternates");
    assert_eq!(key_count("8.2.74"), 2, "8.2.74-only alternates");
    assert_eq!(key_count("7.1.35+8.4.65"), 16, "7.1.35+8.4.65 alternates");
    assert_eq!(
"""),
("""    assert_eq!(key_count("7.1.35+7.3.86"), 8, "7.1.35+7.3.86 alternates");
    assert_eq!(
        key_count("7.1.35+7.3.86+8.4.56"),
        8,
        "7.1.35+7.3.86+8.4.56 alternates"
    );
    assert_eq!(key_count("7.3.86+6.4.107"), 8, "7.3.86+6.4.107 alternates");
    assert_eq!(key_count("7.3.86+8.4.56"), 77, "7.3.86+8.4.56 alternates");
    assert_eq!(key_count("7.3.86+8.2.75"), 1, "7.3.86+8.2.75 alternates");
    assert_eq!(key_count("6.4.115"), 23, "6.4.115-only alternates");
    assert_eq!(key_count("7.1.35+6.4.115"), 2, "7.1.35+6.4.115 alternates");
    assert_eq!(
        key_count("7.1.35+6.4.115+8.4.56"),
        2,
        "7.1.35+6.4.115+8.4.56 alternates"
""",
"""    assert_eq!(key_count("7.1.35+7.3.86"), 8, "7.1.35+7.3.86 alternates");
    assert_eq!(
        key_count("7.1.35+7.3.86+8.4.56"),
        8,
        "7.1.35+7.3.86+8.4.56 alternates"
    );
    assert_eq!(key_count("7.3.86+6.4.107"), 8, "7.3.86+6.4.107 alternates");
    assert_eq!(key_count("7.3.86+8.4.56"), 151, "7.3.86+8.4.56 alternates");
    assert_eq!(key_count("7.3.86+8.2.75"), 1, "7.3.86+8.2.75 alternates");
    assert_eq!(key_count("6.4.115"), 23, "6.4.115-only alternates");
    assert_eq!(key_count("7.1.35+6.4.115"), 2, "7.1.35+6.4.115 alternates");
    assert_eq!(
        key_count("7.1.35+6.4.115+8.4.56"),
        2,
        "7.1.35+6.4.115+8.4.56 alternates"
"""),
("""    assert_eq!(
        key_count("7.1.35+6.4.116+8.4.56"),
        2,
        "7.1.35+6.4.116+8.4.56 alternates"
    );
    assert_eq!(key_count("6.4.43"), 9, "6.4.43-only alternates");
    assert_eq!(key_count("6.4.43+8.4.56"), 1, "6.4.43+8.4.56 alternates");
    assert_eq!(key_count("7.3.86+7.1.35"), 60, "7.3.86+7.1.35 alternates");
    assert_eq!(
        key_count("7.3.86+7.1.35+8.4.56"),
        60,
        "7.3.86+7.1.35+8.4.56 alternates"
    );
    // Slice 10f's five Kaumudī vikalpa ids, alone and stacked, with slice
    // 10g's fifty-nine rows: 2564 alone, 2570 alone and 2570+7.3.86 are 10g's.
    // Slice 10j's √ci adds 72 to 2570 alone, its ṇic-less branch in both
    // padas, and two to each of 2570's 8.4.56 and 7.1.35 stacks.
    for (key, n) in [
""",
"""    assert_eq!(
        key_count("7.1.35+6.4.116+8.4.56"),
        2,
        "7.1.35+6.4.116+8.4.56 alternates"
    );
    assert_eq!(key_count("6.4.43"), 9, "6.4.43-only alternates");
    assert_eq!(key_count("6.4.43+8.4.56"), 1, "6.4.43+8.4.56 alternates");
    assert_eq!(key_count("7.3.86+7.1.35"), 134, "7.3.86+7.1.35 alternates");
    assert_eq!(
        key_count("7.3.86+7.1.35+8.4.56"),
        134,
        "7.3.86+7.1.35+8.4.56 alternates"
    );
    // Slice 10f's five Kaumudī vikalpa ids, alone and stacked, with slice
    // 10g's fifty-nine rows: 2564 alone, 2570 alone and 2570+7.3.86 are 10g's.
    // Slice 10j's √ci adds 72 to 2570 alone, its ṇic-less branch in both
    // padas, and two to each of 2570's 8.4.56 and 7.1.35 stacks.
    for (key, n) in [
"""),
("""/// that root, ātmanepada, opening with 10.0496 and crediting no pada sūtra.
/// `mAnayate` and `amAnayata` are the one pair two ākusmīya rows share
/// (`10.0233 mAna~` unchanged before ṇic, `10.0234 mana~` by 7.2.116): one
/// analysis per root opening with 10.0496, and only √man's credits 7.2.116.
/// Slice 10h's ādhṛṣīya `10.0381 mAna~` shares them too, by 1.3.74 on its ṇic
/// branch — and `amAnayata` twice over, its parasmaipada madhyama bahu
/// as well — so `mAnayate` has three analyses and `amAnayata` four. The
/// parasmaipada shapes derive nothing. The single-analysis and Invalid
/// assertions depend on the homograph partners being uncurated — `10.0189`,
/// `10.0438`, `10.0041`, `10.0006`, `10.0034` — so a slice that curates one
/// must revisit this test (e.g. `kuwwayatu` becomes Valid once `10.0034`
/// is). `mAnayati` became Valid when slice 10h curated `10.0381`.
#[test]
fn curadi_analyses_its_bulk_akusmiya_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    for (form, dhatu) in [
        ("dAsayate", "das"),
        ("aqepayata", "qip"),
        ("spASayatAm", "spaS"),
        ("tarjayeta", "tarj"),
        ("trowayaDve", "truw"),
        ("vedayate", "vid"),
        ("kURayasva", "kUR"),
        ("SAWayate", "SaW"),
        ("SAmayeran", "Sam"),
        ("syAmayate", "syam"),
        ("alakzayanta", "lakz"),
        ("kuwwayate", "kuww"),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), 1, "{form}");
        let a = &r.analyses[0];
        assert_eq!(a.dhatu, dhatu, "{form}");
        assert_eq!(a.pada, Pada::Atmanepada, "{form}");
        let ids = ids_of(a);
        assert_eq!(ids[0], "10.0496", "{form}: {ids:?}");
        for absent in ["1.3.12", "1.3.66", "1.3.72", "1.3.74", "1.3.78"] {
            assert!(!ids.iter().any(|i| i == absent), "{form} {absent}: {ids:?}");
        }
    }
    // (form, `10.0381 mAna~`'s padas for it)
    for (form, adhrsiya_padas) in [
        ("mAnayate", &[Pada::Atmanepada][..]),
        ("amAnayata", &[Pada::Atmanepada, Pada::Parasmaipada][..]),
    ] {
""",
"""/// that root, ātmanepada, opening with 10.0496 and crediting no pada sūtra.
/// `mAnayate` and `amAnayata` are the one pair two ākusmīya rows share
/// (`10.0233 mAna~` unchanged before ṇic, `10.0234 mana~` by 7.2.116): one
/// analysis per root opening with 10.0496, and only √man's credits 7.2.116.
/// Slice 10h's ādhṛṣīya `10.0381 mAna~` shares them too, by 1.3.74 on its ṇic
/// branch — and `amAnayata` twice over, its parasmaipada madhyama bahu
/// as well — so `mAnayate` has three analyses and `amAnayata` four. The
/// parasmaipada shapes derive nothing. Slice 10k curated the five
/// ubhayapadī homograph partners — `10.0189`, `10.0438`, `10.0041`,
/// `10.0006`, `10.0034` — so their witnesses here gain the partner's
/// analyses by 1.3.74 (and `aqepayata` its parasmaipada madhyama bahu too),
/// and `kuwwayatu` is `10.0034`'s and no longer Invalid. `mAnayati` became
/// Valid when slice 10h curated `10.0381`.
#[test]
fn curadi_analyses_its_bulk_akusmiya_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    // (form, root, the ubhayapadī partner's padas for it — none, or the
    // padas of slice 10k's homograph row)
    for (form, dhatu, partner_padas) in [
        ("dAsayate", "das", &[][..]),
        (
            "aqepayata",
            "qip",
            &[Pada::Atmanepada, Pada::Parasmaipada][..],
        ),
        ("spASayatAm", "spaS", &[][..]),
        ("tarjayeta", "tarj", &[][..]),
        ("trowayaDve", "truw", &[][..]),
        ("vedayate", "vid", &[][..]),
        ("kURayasva", "kUR", &[Pada::Atmanepada][..]),
        ("SAWayate", "SaW", &[Pada::Atmanepada][..]),
        ("SAmayeran", "Sam", &[][..]),
        ("syAmayate", "syam", &[][..]),
        ("alakzayanta", "lakz", &[Pada::Atmanepada][..]),
        ("kuwwayate", "kuww", &[Pada::Atmanepada][..]),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), 1 + partner_padas.len(), "{form}");
        let mut partner: Vec<Pada> = Vec::new();
        for a in &r.analyses {
            assert_eq!(a.dhatu, dhatu, "{form}");
            let ids = ids_of(a);
            if ids[0] == "10.0496" {
                assert_eq!(a.pada, Pada::Atmanepada, "{form}");
                for absent in ["1.3.12", "1.3.66", "1.3.72", "1.3.74", "1.3.78"] {
                    assert!(!ids.iter().any(|i| i == absent), "{form} {absent}: {ids:?}");
                }
            } else {
                // The slice 10k partner: 1.3.74 in ātmanepada, 1.3.78 in
                // parasmaipada.
                let sanction = match a.pada {
                    Pada::Atmanepada => "1.3.74",
                    Pada::Parasmaipada => "1.3.78",
                };
                assert!(ids.iter().any(|i| i == sanction), "{form}: {ids:?}");
                assert!(!ids.iter().any(|i| i == "10.0496"), "{form}: {ids:?}");
                partner.push(a.pada);
            }
        }
        assert_eq!(partner.len(), partner_padas.len(), "{form}");
        for pada in partner_padas {
            assert!(partner.contains(pada), "{form} {pada:?}");
        }
    }
    // (form, `10.0381 mAna~`'s padas for it)
    for (form, adhrsiya_padas) in [
        ("mAnayate", &[Pada::Atmanepada][..]),
        ("amAnayata", &[Pada::Atmanepada, Pada::Parasmaipada][..]),
    ] {
"""),
("""        akusmiya.sort_unstable();
        assert_eq!(akusmiya, ["mAn", "man"], "{form}");
        assert_eq!(adhrsiya.len(), adhrsiya_padas.len(), "{form}");
        for pada in adhrsiya_padas {
            assert!(adhrsiya.contains(pada), "{form} {pada:?}");
        }
    }
    for form in ["dAsayati", "vedayati", "kuwwayatu"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}

/// Slice 10d's `check()` witnesses: all six jñapādi rows, both padas. The
""",
"""        akusmiya.sort_unstable();
        assert_eq!(akusmiya, ["mAn", "man"], "{form}");
        assert_eq!(adhrsiya.len(), adhrsiya_padas.len(), "{form}");
        for pada in adhrsiya_padas {
            assert!(adhrsiya.contains(pada), "{form} {pada:?}");
        }
    }
    for form in ["dAsayati", "vedayati"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}

/// Slice 10d's `check()` witnesses: all six jñapādi rows, both padas. The
"""),
("""            Pada::Parasmaipada,
            &["2564", "10.0499"],
        ),
        ("laRqayate", &["lanq", "lanq"], Pada::Atmanepada, &[]),
        ("kzampARi", &["kzanp"], Pada::Parasmaipada, &["2564"]),
        ("kzampayARi", &["kzanp"], Pada::Parasmaipada, &[]),
        ("SraRati", &["SraR"], Pada::Parasmaipada, &["2570"]),
        ("SrARayate", &["SraR"], Pada::Atmanepada, &[]),
        ("SarDati", &["SfD"], Pada::Parasmaipada, &["2570"]),
        ("SarDayate", &["SfD"], Pada::Atmanepada, &[]),
        ("aYcati", &["anc"], Pada::Parasmaipada, &["2570"]),
        ("aYcayate", &["anc"], Pada::Atmanepada, &[]),
        // Slice 10g's homographs: two rows each, and a third `lanj`, the
        // āsvadīya `10.0315`, since slice 10i.
        (
""",
"""            Pada::Parasmaipada,
            &["2564", "10.0499"],
        ),
        ("laRqayate", &["lanq", "lanq"], Pada::Atmanepada, &[]),
        ("kzampARi", &["kzanp"], Pada::Parasmaipada, &["2564"]),
        ("kzampayARi", &["kzanp"], Pada::Parasmaipada, &[]),
        ("SraRati", &["SraR"], Pada::Parasmaipada, &["2570"]),
        // Since slice 10k `10.0063 SraRa~` (7.2.116, obligatory ṇic) shares
        // `10.0174`'s every ṇic form.
        ("SrARayate", &["SraR", "SraR"], Pada::Atmanepada, &[]),
        ("SarDati", &["SfD"], Pada::Parasmaipada, &["2570"]),
        ("SarDayate", &["SfD"], Pada::Atmanepada, &[]),
        ("aYcati", &["anc"], Pada::Parasmaipada, &["2570"]),
        ("aYcayate", &["anc"], Pada::Atmanepada, &[]),
        // Slice 10g's homographs: two rows each, and a third `lanj`, the
        // āsvadīya `10.0315`, since slice 10i.
        (
"""),
("""            "prIRayate",
            &["prI"],
            Pada::Atmanepada,
            false,
            &["7.3.37.2", "1.3.74"],
        ),
        ("mArjati", &["mfj"], Pada::Parasmaipada, true, &["7.2.114"]),
        (
            "mArjayati",
            &["mfj"],
            Pada::Parasmaipada,
            false,
            &["7.2.114"],
        ),
        ("hiMsati", &["hins"], Pada::Parasmaipada, true, &["8.3.24"]),
        ("kaRWati", &["kanW"], Pada::Parasmaipada, true, &["8.4.58"]),
        ("mAnayati", &["mAn"], Pada::Parasmaipada, false, &["1.3.78"]),
        // Homographs inside the slice: two rows each.
        (
            "tarpati",
            &["tfp", "tfp"],
""",
"""            "prIRayate",
            &["prI"],
            Pada::Atmanepada,
            false,
            &["7.3.37.2", "1.3.74"],
        ),
        ("mArjati", &["mfj"], Pada::Parasmaipada, true, &["7.2.114"]),
        ("hiMsati", &["hins"], Pada::Parasmaipada, true, &["8.3.24"]),
        ("kaRWati", &["kanW"], Pada::Parasmaipada, true, &["8.4.58"]),
        ("mAnayati", &["mAn"], Pada::Parasmaipada, false, &["1.3.78"]),
        // Homographs inside the slice: two rows each.
        (
            "tarpati",
            &["tfp", "tfp"],
"""),
("""                assert!(has(&ids, "3.1.25"), "{form}: {ids:?}");
                assert!(!has(&ids, "10.0498"), "{form}: {ids:?}");
            }
            for id in credits {
                assert!(has(&ids, id), "{form} {id}: {ids:?}");
            }
        }
    }
    // √mṛj's vṛddhi replaces guṇa on both branches; √dhū's nuk replaces vṛddhi.
    for form in ["mArjati", "mArjayati"] {
        let ids = ids_of(&engine.check(form).analyses[0]);
        assert!(!has(&ids, "7.3.86"), "{form}: {ids:?}");
    }
    let ids = ids_of(&engine.check("DUnayati").analyses[0]);
""",
"""                assert!(has(&ids, "3.1.25"), "{form}: {ids:?}");
                assert!(!has(&ids, "10.0498"), "{form}: {ids:?}");
            }
            for id in credits {
                assert!(has(&ids, id), "{form} {id}: {ids:?}");
            }
        }
    }
    // Since slice 10k `10.0150 mArja~`, unchanged before ṇic, shares √mṛj's
    // every ṇic form. √mṛj's analysis, 7.2.114's, comes first.
    let r = engine.check("mArjayati");
    assert!(matches!(r.verdict, Verdict::Valid), "mArjayati");
    let roots: Vec<&str> = r.analyses.iter().map(|a| a.dhatu.as_str()).collect();
    assert_eq!(roots, ["mfj", "mArj"], "mArjayati");
    for (a, vrddhi) in r.analyses.iter().zip([true, false]) {
        assert_eq!(a.pada, Pada::Parasmaipada, "mArjayati");
        let ids = ids_of(a);
        assert!(has(&ids, "3.1.25"), "mArjayati: {ids:?}");
        assert!(!has(&ids, "10.0498"), "mArjayati: {ids:?}");
        assert_eq!(has(&ids, "7.2.114"), vrddhi, "mArjayati: {ids:?}");
    }
    // √mṛj's vṛddhi replaces guṇa on both branches; √dhū's nuk replaces vṛddhi.
    for form in ["mArjati", "mArjayati"] {
        let ids = ids_of(&engine.check(form).analyses[0]);
        assert!(!has(&ids, "7.3.86"), "{form}: {ids:?}");
    }
    let ids = ids_of(&engine.check("DUnayati").analyses[0]);
"""),
("""        "jYAyayati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
""",
"""        "jYAyayati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}

/// Slice 10k's `check()` witnesses, from its spec's lists: one per change
/// before ṇic (7.3.86 on `i`, `u` and `f`, 7.2.116, none), the existing rules
/// the slice reaches in new places (8.3.24 with and without 8.4.58, the sanādi
/// 6.1.73 on √pich, the laṅ aṭ's tuk on √chard), each in-slice homograph pair,
/// and each homograph of a row curated earlier. The goldens were grepped for
/// every witness first. A homograph's analyses come in `dhatus()` order, so
/// the row curated earlier answers first. Every analysis is a ṇic branch: it
/// credits 3.1.25 and 1.3.78 (these are parasmaipada surfaces) and no
/// optional-ṇic id. The five ākusmīya partners' ātmanepada surfaces are
/// `curadi_analyses_its_bulk_akusmiya_forms`'s; their parasmaipada ones are
/// this slice's rows' alone. The shapes the slice rules out — no ṇic, no
/// guṇa or vṛddhi before it, no anusvāra, no tuk — derive nothing.
#[test]
fn curadi_analyses_its_plain_nic_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    let has = |ids: &[String], id: &str| ids.iter().any(|i| i == id);
    // (form, [(root, ids it must credit, ids it must not)], in analysis order)
    type Reading<'a> = (&'a str, &'a [&'a str], &'a [&'a str]);
    let pre_nic: &[&str] = &["7.3.86", "7.2.116"];
    let witnesses: &[(&str, &[Reading])] = &[
        // One per change before ṇic.
        ("medayati", &[("mid", &["7.3.86"], &["7.2.116"])]),
        ("codayati", &[("cud", &["7.3.86"], &["7.2.116"])]),
        ("jAlayati", &[("jal", &["7.2.116"], &["7.3.86"])]),
        ("pIqayati", &[("pIq", &[], pre_nic)]),
        // Existing rules in new places.
        ("sambayati", &[("sanb", &["8.3.24", "8.4.58"], pre_nic)]),
        ("puMsayati", &[("puns", &["8.3.24"], &["8.4.58", "7.3.86"])]),
        ("picCayati", &[("piC", &["6.1.73"], pre_nic)]),
        (
            "acCardayat",
            &[
                ("Cfd", &["6.1.73", "7.3.86"], &[]),
                ("Card", &["6.1.73"], pre_nic),
            ],
        ),
        // In-slice homograph pairs.
        (
            "parTayati",
            &[("pfT", &["7.3.86"], &[]), ("parT", &[], pre_nic)],
        ),
        (
            "polayati",
            &[("pul", &["7.3.86"], &[]), ("pul", &["7.3.86"], &[])],
        ),
        (
            "pAlayati",
            &[("pAl", &[], pre_nic), ("pal", &["7.2.116"], &[])],
        ),
        (
            "sAntvayati",
            &[
                ("sAntv", &["8.3.24", "8.4.58"], &[]),
                ("sAntv", &["8.3.24", "8.4.58"], &[]),
            ],
        ),
        // Homographs of rows curated earlier: theirs first, then slice 10k's.
        (
            "SrATayati",
            &[("SraT", &["7.2.116"], &[]), ("SraT", &["7.2.116"], &[])],
        ),
        (
            "varRayati",
            &[("varRa", &["6.4.48"], &[]), ("varR", &[], pre_nic)],
        ),
        (
            "valkayati",
            &[("valka", &["6.4.48"], &[]), ("valk", &[], pre_nic)],
        ),
        (
            "SrARayati",
            &[("SraR", &["7.2.116"], &[]), ("SraR", &["7.2.116"], &[])],
        ),
        (
            "tAqayati",
            &[("taq", &["7.2.116"], &[]), ("taq", &["7.2.116"], &[])],
        ),
        (
            "Cardayati",
            &[("Cfd", &["7.3.86"], &[]), ("Card", &[], pre_nic)],
        ),
        (
            "kAlayati",
            &[("kAla", &["6.4.48"], &[]), ("kal", &["7.2.116"], &[])],
        ),
        (
            "velayati",
            &[("vela", &["6.4.48"], &[]), ("vil", &["7.3.86"], &[])],
        ),
        (
            "mArjayati",
            &[("mfj", &["7.2.114"], &[]), ("mArj", &[], &["7.2.114"])],
        ),
        (
            "varDayati",
            &[("vfD", &["7.3.86"], &[]), ("varD", &[], pre_nic)],
        ),
        (
            "barhayati",
            &[("barh", &[], pre_nic), ("barh", &[], pre_nic)],
        ),
        (
            "rowayati",
            &[("ruw", &["7.3.86"], &[]), ("ruw", &["7.3.86"], &[])],
        ),
        (
            "GAwayati",
            &[("Gaw", &["7.2.116"], &[]), ("Gaw", &["7.2.116"], &[])],
        ),
        ("arhayati", &[("arh", &[], pre_nic), ("arh", &[], pre_nic)]),
        (
            "BAjayati",
            &[("BAja", &["6.4.48"], &[]), ("Baj", &["7.2.116"], &[])],
        ),
        (
            "vAsayati",
            &[("vAsa", &["6.4.48"], &[]), ("vas", &["7.2.116"], &[])],
        ),
        // The ākusmīya partners' parasmaipada: slice 10k's row alone.
        ("lakzayati", &[("lakz", &[], pre_nic)]),
        ("kuwwayati", &[("kuww", &[], pre_nic)]),
        ("SAWayati", &[("SaW", &["7.2.116"], &[])]),
        ("qepayati", &[("qip", &["7.3.86"], &[])]),
        ("kURayati", &[("kUR", &[], pre_nic)]),
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
            for id in ["3.1.25", "1.3.78"].iter().chain(credits.iter()) {
                assert!(has(&ids, id), "{form} {root} {id}: {ids:?}");
            }
            for id in [
                "10.0496", "10.0497", "10.0498", "10.0499", "2564", "2565", "2570", "2571",
            ]
            .iter()
            .chain(absent.iter())
            {
                assert!(!has(&ids, id), "{form} {root} {id}: {ids:?}");
            }
        }
    }
    for form in [
        "medati",
        "codati",
        "jalati",
        "pIqati",
        "midayati",
        "jalayati",
        "sanbayati",
        "punsayati",
        "piCayati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
"""),
],
'crates/panini/tests/trace/curadi.rs': [
("""    // — and nowhere else: every credit's number lies in the positional
    // `AKUSMIYA` range. And 1.3.74 never reaches them: its credits stay on the
    // 264 `Nic` rows and the seven `NicUbhayapada` rows' ṇic branch, read from
    // the curated `pada` column (ten before slice 10e, listed literally until
    // then).
""",
"""    // — and nowhere else: every credit's number lies in the positional
    // `AKUSMIYA` range. And 1.3.74 never reaches them: its credits stay on the
    // 419 `Nic` rows and the seven `NicUbhayapada` rows' ṇic branch, read from
    // the curated `pada` column (ten before slice 10e, listed literally until
    // then).
"""),
("""        .map(|d| d.dhatupatha)
        .collect();
    assert_eq!(nic.len(), 264, "curated 1.3.74 rows");
    let nic_ubhayapada: Vec<&str> = dhatus()
        .iter()
""",
"""        .map(|d| d.dhatupatha)
        .collect();
    assert_eq!(nic.len(), 419, "curated 1.3.74 rows");
    let nic_ubhayapada: Vec<&str> = dhatus()
        .iter()
"""),
("""    // sanādi entries to their rows across the corpus: 3.1.28 on √dhūp and
    // √vich alone (`no_nic_pada_rule_reaches_a_nicless_branch` holds it to
    // their ṇic-less branches), and the sanādi 6.1.73 on √vich alone.
    assert_eq!(rows_crediting("3.1.28", true), ["10.0303", "10.0304"]);
    assert_eq!(rows_crediting("6.1.73", true), ["10.0304"]);
}

""",
"""    // sanādi entries to their rows across the corpus: 3.1.28 on √dhūp and
    // √vich alone (`no_nic_pada_rule_reaches_a_nicless_branch` holds it to
    // their ṇic-less branches), and the sanādi 6.1.73 on √vich and, since
    // slice 10k, √pich (`10.0061 piC`, picCayati) alone.
    assert_eq!(rows_crediting("3.1.28", true), ["10.0303", "10.0304"]);
    assert_eq!(rows_crediting("6.1.73", true), ["10.0061", "10.0304"]);
}

"""),
],
'crates/panini/tests/trace/juhotyadi.rs': [
("""    // before a jhal (`granT` twice, `hins`, `SunD`, `SranT`, `kanW`), on
    // every live branch of both, and since slice 10i the twenty-seven idit
    // āsvadīya rows (num stored), on every live branch of both.
    const CURADI: [&str; 100] = [
        "10.0204", "10.0433", "10.0460", "10.0467", "10.0471", "10.0472", "10.0473", "10.0474",
        "10.0193", "10.0194", "10.0198", "10.0199", "10.0227", "10.0002", "10.0003", "10.0004",
""",
"""    // before a jhal (`granT` twice, `hins`, `SunD`, `SranT`, `kanW`), on
    // every live branch of both, and since slice 10i the twenty-seven idit
    // āsvadīya rows (num stored), on every live branch of both, and since
    // slice 10k ten plain obligatory-ṇic rows with an `n` before a jhal
    // (`banD`, `sanb`, `Sanb`, `sAnb`, `lunw`, `lunW`, `sAntv` twice, `puns`,
    // `krand`), on every live branch.
    const CURADI: [&str; 110] = [
        "10.0204", "10.0433", "10.0460", "10.0467", "10.0471", "10.0472", "10.0473", "10.0474",
        "10.0193", "10.0194", "10.0198", "10.0199", "10.0227", "10.0002", "10.0003", "10.0004",
"""),
("""        "10.0293", "10.0294", "10.0295", "10.0296", "10.0298", "10.0299", "10.0315", "10.0316",
        "10.0317", "10.0318", "10.0319", "10.0321", "10.0322", "10.0323", "10.0326", "10.0327",
        "10.0328", "10.0329", "10.0330", "10.0331",
    ];
    let hits = credited("8.3.24");
""",
"""        "10.0293", "10.0294", "10.0295", "10.0296", "10.0298", "10.0299", "10.0315", "10.0316",
        "10.0317", "10.0318", "10.0319", "10.0321", "10.0322", "10.0323", "10.0326", "10.0327",
        "10.0328", "10.0329", "10.0330", "10.0331", "10.0021", "10.0030", "10.0031", "10.0032",
        "10.0039", "10.0040", "10.0051", "10.0052", "10.0134", "10.0252",
    ];
    let hits = credited("8.3.24");
"""),
("""    // ṇic ātmanepada), and slice 10g's fifty-four, slice 10h's six and slice
    // 10i's twenty-seven 120 each (every live branch: 84 parasmaipada, ṇic
    // and ṇic-less, and 36 ṇic ātmanepada).
    assert_eq!(curadi.len(), 36 + 7 * 78 + 5 * 78 + 87 * 120);
    for number in CURADI {
        assert!(
""",
"""    // ṇic ātmanepada), and slice 10g's fifty-four, slice 10h's six and slice
    // 10i's twenty-seven 120 each (every live branch: 84 parasmaipada, ṇic
    // and ṇic-less, and 36 ṇic ātmanepada), and slice 10k's ten 78 each.
    assert_eq!(curadi.len(), 36 + 7 * 78 + 5 * 78 + 87 * 120 + 10 * 78);
    for number in CURADI {
        assert!(
"""),
("""    // roots (`Cfd`, `Cfp`, `Cad`) the same way (acCardayat, acCardat), and
    // slice 10i √vich (`viC`), whose root-internal tuk the sanādi 6.1.73
    // gives on every branch (vicCayati, vicCAyati).
    let hits = credited("8.4.40");
    assert!(
""",
"""    // roots (`Cfd`, `Cfp`, `Cad`) the same way (acCardayat, acCardat), and
    // slice 10i √vich (`viC`), whose root-internal tuk the sanādi 6.1.73
    // gives on every branch (vicCayati, vicCAyati), and slice 10k the two
    // ch-initial plain rows (`Card`, `Cuw`) the adanta way (acCardayat) and
    // √pich (`piC`) the √vich way (picCayati).
    let hits = credited("8.4.40");
    assert!(
"""),
("""            [
                "07.0003", "07.0008", "10.0469", "10.0480", "10.0481", "10.0062", "10.0114",
                "10.0171", "10.0352", "10.0354", "10.0370", "10.0304"
            ]
            .contains(number),
""",
"""            [
                "07.0003", "07.0008", "10.0469", "10.0480", "10.0481", "10.0062", "10.0114",
                "10.0171", "10.0352", "10.0354", "10.0370", "10.0304", "10.0078", "10.0462",
                "10.0061"
            ]
            .contains(number),
"""),
("""    // slice 10g's three and slice 10h's three alike. √vich: every live
    // branch, 84 parasmaipada (ṇic and ṇic-less) and 36 ṇic ātmanepada.
    assert_eq!(curadi, 3 * 19 + 6 * 29 + 120);
}
""",
"""    // slice 10g's three and slice 10h's three alike. √vich: every live
    // branch, 84 parasmaipada (ṇic and ṇic-less) and 36 ṇic ātmanepada.
    // Slice 10k's two ch-initial rows count as the adanta ones do, and √pich
    // every live branch, 42 parasmaipada and 36 ātmanepada.
    assert_eq!(curadi, 3 * 19 + 6 * 29 + 120 + 2 * 19 + 78);
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
python3 /tmp/vidyut-full/slice10k/assertions_10k.py      # applied 32 edits
mise run fmt
```

- [ ] **Step 2: Run them to see them fail**

Run: `mise exec -- cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .*FAILED|test result: FAILED"`
Expected: `panini-data` 25 passed / 3 failed, `paradigm` 24 passed / 5 failed, `trace` 211 passed / 4 failed, every other binary passing:

- `curadi::a_kusmad_is_credited_on_exactly_the_akusmiya_cells`
- `curadi::the_10i_aya_and_tuk_fire_only_on_their_rows`
- `curadi_analyses_its_adhrsiya_forms`
- `curadi_analyses_its_bulk_akusmiya_forms`
- `curadi_analyses_its_optional_nic_forms`
- `curadi_analyses_its_plain_nic_forms`
- `derivation_set_shape_matches_the_audited_numbers`
- `juhotyadi::nas_capadantasya_is_credited_only_on_rudhadi_dhan_jan_and_curadi_roots`
- `juhotyadi::shcutva_off_jan_is_credited_exactly_as_before_3f3`
- `tests::curadi_rows_are_the_four_hundred_eighty_two_curated_roots`
- `tests::curated_roots_have_expected_ganas_and_padas`
- `tests::dhatupatha_numbers_resolve_upstream`

`pada_ambiguous_surfaces_are_exactly_these` and `paradigm_covers_every_enumerable_cell` pass for now: with no new rows, the old set and coverage are still exact. `dhatupatha_numbers_resolve_upstream` fails on `CONVERGENT_UPADESHA_PAIR`'s "both rows are curated".

- [ ] **Step 3: The rows**

Create `/tmp/vidyut-full/slice10k/rows_10k.py` if it is missing (sha256 `b6220ae968a22379b61c76295d717112caec1d07da818ffd8af299031292e7ed`). The codes are `stored_form`'s output; `dhatupatha_numbers_resolve_upstream` checks each. Each comment names the change before ṇic (or why there is none), any existing rule the row newly reaches, its homograph partner, and its laṭ parasmaipada prathama eka.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10k — the 155 rows' `Dhatu` entries, appended to
`DHATUS` after 10j's, in dhātupāṭha order. Run from the worktree root."""
p = 'crates/panini-data/src/lib.rs'
s = open(p).read()
ROWS = """    Dhatu {
        // 10.0006 `lakza~` darSanANkanayoH (√lakṣ). Guru upadhā (the conjunct
        // `kz`), so unchanged before ṇic. Shares its ātmanepada forms with the
        // ākusmīya `10.0219 lakza~`. Ubhayapadī by 1.3.74 (*lakṣayati*). Slice
        // 10k.
        dhatupatha: "10.0006",
        code: "lakz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "darSanANkanayoH",
    },
    Dhatu {
        // 10.0008 `kudf~` anftaBAzaRe (√kud). 7.3.86 guṇates the laghu upadhā
        // before ṇic (kod-i). Ubhayapadī by 1.3.74 (*kodayati*). Slice 10k.
        dhatupatha: "10.0008",
        code: "kud",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "anftaBAzaRe",
    },
    Dhatu {
        // 10.0012 `mida~` snehane (√mid). 7.3.86 guṇates the laghu upadhā
        // before ṇic (med-i). Ubhayapadī by 1.3.74 (*medayati*). Slice 10k.
        dhatupatha: "10.0012",
        code: "mid",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "snehane",
    },
    Dhatu {
        // 10.0015 `jala~` apavAraRe (√jal). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (jAl-i). Ubhayapadī by 1.3.74 (*jālayati*).
        // Slice 10k.
        dhatupatha: "10.0015",
        code: "jal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "apavAraRe",
    },
    Dhatu {
        // 10.0016 `laja~` apavAraRe (√laj). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (lAj-i). Ubhayapadī by 1.3.74 (*lājayati*).
        // Slice 10k.
        dhatupatha: "10.0016",
        code: "laj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "apavAraRe",
    },
    Dhatu {
        // 10.0017 `pIqa~` avagAhane (√pīḍ). Long upadhā vowel `I`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*pīḍayati*). Slice 10k.
        dhatupatha: "10.0017",
        code: "pIq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "avagAhane",
    },
    Dhatu {
        // 10.0018 `nawa~` avaspandane (√naṭ). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (nAw-i). Ubhayapadī by 1.3.74
        // (*nāṭayati*). Slice 10k.
        dhatupatha: "10.0018",
        code: "naw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "avaspandane",
    },
    Dhatu {
        // 10.0019 `SraTa~` prayatne (√śrath). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (SrAT-i). Homograph of the ubhayapadī
        // curādi row `10.0360 SraTa~`: they share every form. Ubhayapadī by
        // 1.3.74 (*śrāthayati*). Slice 10k.
        dhatupatha: "10.0019",
        code: "SraT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prayatne",
    },
    Dhatu {
        // 10.0020 `baDa~` saMyamane (√badh). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (bAD-i). Ubhayapadī by 1.3.74
        // (*bādhayati*). Slice 10k.
        dhatupatha: "10.0020",
        code: "baD",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMyamane",
    },
    Dhatu {
        // 10.0021 `banDa~` saMyamane (√bandh). Guru upadhā (the conjunct `nD`),
        // so unchanged before ṇic. 8.3.24 makes its `n` an anusvāra before `D`,
        // and 8.4.58 restores `n`. Ubhayapadī by 1.3.74 (*bandhayati*). Slice
        // 10k.
        dhatupatha: "10.0021",
        code: "banD",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMyamane",
    },
    Dhatu {
        // 10.0024 `pakza~` parigrahe (√pakṣ). Guru upadhā (the conjunct `kz`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*pakṣayati*). Slice
        // 10k.
        dhatupatha: "10.0024",
        code: "pakz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "parigrahe",
    },
    Dhatu {
        // 10.0025 `varRa~` preraRe (√varṇ). Guru upadhā (the conjunct `rR`), so
        // unchanged before ṇic. Homograph of the ubhayapadī curādi row `10.0484
        // varRa`: they share every form. Ubhayapadī by 1.3.74 (*varṇayati*).
        // Slice 10k.
        dhatupatha: "10.0025",
        code: "varR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "preraRe",
    },
    Dhatu {
        // 10.0027 `praTa~` praKyAne (√prath). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (prAT-i). Ubhayapadī by 1.3.74
        // (*prāthayati*). Slice 10k.
        dhatupatha: "10.0027",
        code: "praT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "praKyAne",
    },
    Dhatu {
        // 10.0028 `pfTa~` prakzepe (√pṛth). 7.3.86 guṇates the laghu upadhā `f`
        // to `ar` (1.1.51) before ṇic (parT-i). Shares every form with `10.0186
        // parTa~`. Ubhayapadī by 1.3.74 (*parthayati*). Slice 10k.
        dhatupatha: "10.0028",
        code: "pfT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prakzepe",
    },
    Dhatu {
        // 10.0029 `paTa~` prakzepe (√path). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (pAT-i). Ubhayapadī by 1.3.74
        // (*pāthayati*). Slice 10k.
        dhatupatha: "10.0029",
        code: "paT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prakzepe",
    },
    Dhatu {
        // 10.0030 `zanba~` sambanDane (√sanb). Guru upadhā (the conjunct `nb`),
        // so unchanged before ṇic. Stored per 6.1.64. 8.3.24 makes its `n` an
        // anusvāra, and 8.4.58 makes that `m` before `b`. Ubhayapadī by 1.3.74
        // (*sambayati*). Slice 10k.
        dhatupatha: "10.0030",
        code: "sanb",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sambanDane",
    },
    Dhatu {
        // 10.0031 `Sanba~` sambanDane (√śanb). Guru upadhā (the conjunct `nb`),
        // so unchanged before ṇic. 8.3.24 makes its `n` an anusvāra, and 8.4.58
        // makes that `m` before `b`. Ubhayapadī by 1.3.74 (*śambayati*). Slice
        // 10k.
        dhatupatha: "10.0031",
        code: "Sanb",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sambanDane",
    },
    Dhatu {
        // 10.0032 `sAnba~` sambanDane (√sānb). Guru upadhā (the conjunct `nb`),
        // so unchanged before ṇic. 8.3.24 makes its `n` an anusvāra, and 8.4.58
        // makes that `m` before `b`. Ubhayapadī by 1.3.74 (*sāmbayati*). Slice
        // 10k.
        dhatupatha: "10.0032",
        code: "sAnb",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sambanDane",
    },
    Dhatu {
        // 10.0034 `kuwwa~` CedanaBartsanayoH (√kuṭṭ). Guru upadhā (the conjunct
        // `ww`), so unchanged before ṇic. Shares its ātmanepada forms with the
        // ākusmīya `10.0226 kuwwa~`. Ubhayapadī by 1.3.74 (*kuṭṭayati*). Slice
        // 10k.
        dhatupatha: "10.0034",
        code: "kuww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "CedanaBartsanayoH",
    },
    Dhatu {
        // 10.0035 `puwwa~` alpIBAve (√puṭṭ). Guru upadhā (the conjunct `ww`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*puṭṭayati*). Slice
        // 10k.
        dhatupatha: "10.0035",
        code: "puww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "alpIBAve",
    },
    Dhatu {
        // 10.0036 `cuwwa~` alpIBAve (√cuṭṭ). Guru upadhā (the conjunct `ww`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*cuṭṭayati*). Slice
        // 10k.
        dhatupatha: "10.0036",
        code: "cuww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "alpIBAve",
    },
    Dhatu {
        // 10.0038 `zuwwa~` anAdare (√suṭṭ). Guru upadhā (the conjunct `ww`), so
        // unchanged before ṇic. Stored per 6.1.64. Ubhayapadī by 1.3.74
        // (*suṭṭayati*). Slice 10k.
        dhatupatha: "10.0038",
        code: "suww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "anAdare",
    },
    Dhatu {
        // 10.0039 `lunwa~` steye (√lunṭ). Guru upadhā (the conjunct `nw`), so
        // unchanged before ṇic. 8.3.24 makes its `n` an anusvāra, and 8.4.58
        // makes that `R` before `w`. Ubhayapadī by 1.3.74 (*luṇṭayati*). Slice
        // 10k.
        dhatupatha: "10.0039",
        code: "lunw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "steye",
    },
    Dhatu {
        // 10.0040 `lunWa~` steye (√lunṭh). Guru upadhā (the conjunct `nW`), so
        // unchanged before ṇic. 8.3.24 makes its `n` an anusvāra, and 8.4.58
        // makes that `R` before `W`. Ubhayapadī by 1.3.74 (*luṇṭhayati*). Slice
        // 10k.
        dhatupatha: "10.0040",
        code: "lunW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "steye",
    },
    Dhatu {
        // 10.0041 `SaWa~` asaMskAragatyoH (√śaṭh). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (SAW-i). Shares its
        // ātmanepada forms with the ākusmīya `10.0214 SaWa~`. Ubhayapadī by
        // 1.3.74 (*śāṭhayati*). Slice 10k.
        dhatupatha: "10.0041",
        code: "SaW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "asaMskAragatyoH",
    },
    Dhatu {
        // 10.0042 `SvaWa~` asaMskAragatyoH (√śvaṭh). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (SvAW-i). Ubhayapadī by
        // 1.3.74 (*śvāṭhayati*). Slice 10k.
        dhatupatha: "10.0042",
        code: "SvaW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "asaMskAragatyoH",
    },
    Dhatu {
        // 10.0044 `tuja~` hiMsAbalAdAnaniketanezu (√tuj). 7.3.86 guṇates the
        // laghu upadhā before ṇic (toj-i). Ubhayapadī by 1.3.74 (*tojayati*).
        // Slice 10k.
        dhatupatha: "10.0044",
        code: "tuj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAbalAdAnaniketanezu",
    },
    Dhatu {
        // 10.0046 `pija~` hiMsAbalAdAnaniketanezu (√pij). 7.3.86 guṇates the
        // laghu upadhā before ṇic (pej-i). Ubhayapadī by 1.3.74 (*pejayati*).
        // Slice 10k.
        dhatupatha: "10.0046",
        code: "pij",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAbalAdAnaniketanezu",
    },
    Dhatu {
        // 10.0050 `pisa~` gatO (√pis). 7.3.86 guṇates the laghu upadhā before
        // ṇic (pes-i). Ubhayapadī by 1.3.74 (*pesayati*). Slice 10k.
        dhatupatha: "10.0050",
        code: "pis",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gatO",
    },
    Dhatu {
        // 10.0051 `zAntva~` sAmaprayoge (√sāntv). Guru upadhā (the conjunct
        // `ntv`), so unchanged before ṇic. Stored per 6.1.64. 8.3.24 makes its
        // `n` an anusvāra before `t`, and 8.4.58 restores `n`. Shares every
        // form with `10.0052 sAntva~`. `CONVERGENT_UPADESHA_PAIR` lets the two
        // resolve. Ubhayapadī by 1.3.74 (*sāntvayati*). Slice 10k.
        dhatupatha: "10.0051",
        code: "sAntv",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sAmaprayoge",
    },
    Dhatu {
        // 10.0052 `sAntva~` sAmaprayoge (√sāntv). Guru upadhā (the conjunct
        // `ntv`), so unchanged before ṇic. 8.3.24 makes its `n` an anusvāra
        // before `t`, and 8.4.58 restores `n`. Shares every form with `10.0051
        // zAntva~`. `CONVERGENT_UPADESHA_PAIR` lets the two resolve. Ubhayapadī
        // by 1.3.74 (*sāntvayati*). Slice 10k.
        dhatupatha: "10.0052",
        code: "sAntv",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sAmaprayoge",
    },
    Dhatu {
        // 10.0053 `Svalka~` pariBAzaRe (√śvalk). Guru upadhā (the conjunct
        // `lk`), so unchanged before ṇic. Ubhayapadī by 1.3.74 (*śvalkayati*).
        // Slice 10k.
        dhatupatha: "10.0053",
        code: "Svalk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pariBAzaRe",
    },
    Dhatu {
        // 10.0054 `valka~` pariBAzaRe (√valk). Guru upadhā (the conjunct `lk`),
        // so unchanged before ṇic. Homograph of the ubhayapadī curādi row
        // `10.0458 valka`: they share every form. Ubhayapadī by 1.3.74
        // (*valkayati*). Slice 10k.
        dhatupatha: "10.0054",
        code: "valk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pariBAzaRe",
    },
    Dhatu {
        // 10.0055 `zRiha~` snehane (√snih). 7.3.86 guṇates the laghu upadhā
        // before ṇic (sneh-i). Stored per 6.1.64. Ubhayapadī by 1.3.74
        // (*snehayati*). Slice 10k.
        dhatupatha: "10.0055",
        code: "snih",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "snehane",
    },
    Dhatu {
        // 10.0056 `sPiwa~` hiMsAyAm (√sphiṭ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (sPew-i). Ubhayapadī by 1.3.74 (*spheṭayati*). Slice 10k.
        dhatupatha: "10.0056",
        code: "sPiw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0057 `smiwa~` anAdare (√smiṭ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (smew-i). Ubhayapadī by 1.3.74 (*smeṭayati*). Slice 10k.
        dhatupatha: "10.0057",
        code: "smiw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "anAdare",
    },
    Dhatu {
        // 10.0059 `Sliza~` SlezaRe (√śliṣ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (Slez-i). Ubhayapadī by 1.3.74 (*śleṣayati*). Slice 10k.
        dhatupatha: "10.0059",
        code: "Sliz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SlezaRe",
    },
    Dhatu {
        // 10.0061 `piCa~` kuwwane (√pich). The sanādi 6.1.73 che ca gives tuk
        // before its `C` (picC), so the upadhā is guru and unchanged before
        // ṇic. Ubhayapadī by 1.3.74 (*picchayati*). Slice 10k.
        dhatupatha: "10.0061",
        code: "piC",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kuwwane",
    },
    Dhatu {
        // 10.0063 `SraRa~` dAne (√śraṇ). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (SrAR-i). Homograph of the ubhayapadī
        // curādi row `10.0174 SraRu~`: they share every form. Ubhayapadī by
        // 1.3.74 (*śrāṇayati*). Slice 10k.
        dhatupatha: "10.0063",
        code: "SraR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "dAne",
    },
    Dhatu {
        // 10.0064 `taqa~` AGAte (√taḍ). 7.2.116 ata upadhāyāḥ lengthens the `a`
        // upadhā before ṇit ṇic (tAq-i). Homograph of the ubhayapadī curādi row
        // `10.0332 taqa~`: they share every form. Ubhayapadī by 1.3.74
        // (*tāḍayati*). Slice 10k.
        dhatupatha: "10.0064",
        code: "taq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AGAte",
    },
    Dhatu {
        // 10.0065 `Kaqa~` Bedane (√khaḍ). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (KAq-i). Ubhayapadī by 1.3.74
        // (*khāḍayati*). Slice 10k.
        dhatupatha: "10.0065",
        code: "Kaq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Bedane",
    },
    Dhatu {
        // 10.0078 `Carda~` vamane (√chard). Guru upadhā (the conjunct `rd`), so
        // unchanged before ṇic. In laṅ the aṭ takes 6.1.73's tuk (acC-).
        // Homograph of the ubhayapadī curādi row `10.0352 CfdI~`: they share
        // every form. Ubhayapadī by 1.3.74 (*chardayati*). Slice 10k.
        dhatupatha: "10.0078",
        code: "Card",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vamane",
    },
    Dhatu {
        // 10.0079 `pusta~` AdarAnAdarayoH (√pust). Guru upadhā (the conjunct
        // `st`), so unchanged before ṇic. Ubhayapadī by 1.3.74 (*pustayati*).
        // Slice 10k.
        dhatupatha: "10.0079",
        code: "pust",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AdarAnAdarayoH",
    },
    Dhatu {
        // 10.0080 `busta~` AdarAnAdarayoH (√bust). Guru upadhā (the conjunct
        // `st`), so unchanged before ṇic. Ubhayapadī by 1.3.74 (*bustayati*).
        // Slice 10k.
        dhatupatha: "10.0080",
        code: "bust",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AdarAnAdarayoH",
    },
    Dhatu {
        // 10.0081 `cuda~` saYcodane (√cud). 7.3.86 guṇates the laghu upadhā
        // before ṇic (cod-i). Ubhayapadī by 1.3.74 (*codayati*). Slice 10k.
        dhatupatha: "10.0081",
        code: "cud",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saYcodane",
    },
    Dhatu {
        // 10.0082 `nakka~` nASane (√nakk). Guru upadhā (the conjunct `kk`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*nakkayati*). Slice 10k.
        dhatupatha: "10.0082",
        code: "nakk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "nASane",
    },
    Dhatu {
        // 10.0083 `Dakka~` nASane (√dhakk). Guru upadhā (the conjunct `kk`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*dhakkayati*). Slice 10k.
        dhatupatha: "10.0083",
        code: "Dakk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "nASane",
    },
    Dhatu {
        // 10.0084 `cakka~` vyaTane (√cakk). Guru upadhā (the conjunct `kk`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*cakkayati*). Slice 10k.
        dhatupatha: "10.0084",
        code: "cakk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vyaTane",
    },
    Dhatu {
        // 10.0085 `cukka~` vyaTane (√cukk). Guru upadhā (the conjunct `kk`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*cukkayati*). Slice 10k.
        dhatupatha: "10.0085",
        code: "cukk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vyaTane",
    },
    Dhatu {
        // 10.0086 `kzala~` SOcakarmaRi (√kṣal). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (kzAl-i). Ubhayapadī by 1.3.74
        // (*kṣālayati*). Slice 10k.
        dhatupatha: "10.0086",
        code: "kzal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SOcakarmaRi",
    },
    Dhatu {
        // 10.0087 `tala~` pratizWAyAm (√tal). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (tAl-i). Ubhayapadī by 1.3.74
        // (*tālayati*). Slice 10k.
        dhatupatha: "10.0087",
        code: "tal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pratizWAyAm",
    },
    Dhatu {
        // 10.0088 `tula~` unmAne (√tul). 7.3.86 guṇates the laghu upadhā before
        // ṇic (tol-i). Ubhayapadī by 1.3.74 (*tolayati*). Slice 10k.
        dhatupatha: "10.0088",
        code: "tul",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "unmAne",
    },
    Dhatu {
        // 10.0089 `dula~` utkzepe (√dul). 7.3.86 guṇates the laghu upadhā
        // before ṇic (dol-i). Ubhayapadī by 1.3.74 (*dolayati*). Slice 10k.
        dhatupatha: "10.0089",
        code: "dul",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "utkzepe",
    },
    Dhatu {
        // 10.0090 `pula~` mahattve (√pul). 7.3.86 guṇates the laghu upadhā
        // before ṇic (pol-i). Shares every form with `10.0131 pula~`.
        // Ubhayapadī by 1.3.74 (*polayati*). Slice 10k.
        dhatupatha: "10.0090",
        code: "pul",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "mahattve",
    },
    Dhatu {
        // 10.0091 `cula~` samucCrAye (√cul). 7.3.86 guṇates the laghu upadhā
        // before ṇic (col-i). Ubhayapadī by 1.3.74 (*colayati*). Slice 10k.
        dhatupatha: "10.0091",
        code: "cul",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "samucCrAye",
    },
    Dhatu {
        // 10.0092 `mUla~` rohaRe (√mūl). Long upadhā vowel `U`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*mūlayati*). Slice 10k.
        dhatupatha: "10.0092",
        code: "mUl",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "rohaRe",
    },
    Dhatu {
        // 10.0093 `kala~` kzepe (√kal). 7.2.116 ata upadhāyāḥ lengthens the `a`
        // upadhā before ṇit ṇic (kAl-i). Homograph of the ubhayapadī curādi row
        // `10.0422 kAla`: they share every form. Ubhayapadī by 1.3.74
        // (*kālayati*). Slice 10k.
        dhatupatha: "10.0093",
        code: "kal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kzepe",
    },
    Dhatu {
        // 10.0094 `vila~` kzepe (√vil). 7.3.86 guṇates the laghu upadhā before
        // ṇic (vel-i). Homograph of the ubhayapadī curādi row `10.0421 vela`:
        // they share every form. Ubhayapadī by 1.3.74 (*velayati*). Slice 10k.
        dhatupatha: "10.0094",
        code: "vil",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kzepe",
    },
    Dhatu {
        // 10.0095 `bila~` Bedane (√bil). 7.3.86 guṇates the laghu upadhā before
        // ṇic (bel-i). Ubhayapadī by 1.3.74 (*belayati*). Slice 10k.
        dhatupatha: "10.0095",
        code: "bil",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Bedane",
    },
    Dhatu {
        // 10.0096 `tila~` snehane (√til). 7.3.86 guṇates the laghu upadhā
        // before ṇic (tel-i). Ubhayapadī by 1.3.74 (*telayati*). Slice 10k.
        dhatupatha: "10.0096",
        code: "til",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "snehane",
    },
    Dhatu {
        // 10.0097 `cala~` BftO (√cal). 7.2.116 ata upadhāyāḥ lengthens the `a`
        // upadhā before ṇit ṇic (cAl-i). Ubhayapadī by 1.3.74 (*cālayati*).
        // Slice 10k.
        dhatupatha: "10.0097",
        code: "cal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BftO",
    },
    Dhatu {
        // 10.0098 `pAla~` rakzaRe (√pāl). Long upadhā vowel `A`, so unchanged
        // before ṇic. Shares every form with `10.0099 pala~`. Ubhayapadī by
        // 1.3.74 (*pālayati*). Slice 10k.
        dhatupatha: "10.0098",
        code: "pAl",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "rakzaRe",
    },
    Dhatu {
        // 10.0099 `pala~` rakzaRe (√pal). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (pAl-i). Shares every form with `10.0098
        // pAla~`. Ubhayapadī by 1.3.74 (*pālayati*). Slice 10k.
        dhatupatha: "10.0099",
        code: "pal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "rakzaRe",
    },
    Dhatu {
        // 10.0100 `lUza~` hiMsAyAm (√lūṣ). Long upadhā vowel `U`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*lūṣayati*). Slice 10k.
        dhatupatha: "10.0100",
        code: "lUz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0101 `Sulba~` mAne (√śulb). Guru upadhā (the conjunct `lb`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*śulbayati*). Slice 10k.
        dhatupatha: "10.0101",
        code: "Sulb",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "mAne",
    },
    Dhatu {
        // 10.0102 `SUrpa~` mAne (√śūrp). Guru upadhā (the conjunct `rp`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*śūrpayati*). Slice 10k.
        dhatupatha: "10.0102",
        code: "SUrp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "mAne",
    },
    Dhatu {
        // 10.0103 `cuwa~` Cedane (√cuṭ). 7.3.86 guṇates the laghu upadhā before
        // ṇic (cow-i). Ubhayapadī by 1.3.74 (*coṭayati*). Slice 10k.
        dhatupatha: "10.0103",
        code: "cuw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Cedane",
    },
    Dhatu {
        // 10.0104 `muwa~` saYcUrRane (√muṭ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (mow-i). Ubhayapadī by 1.3.74 (*moṭayati*). Slice 10k.
        dhatupatha: "10.0104",
        code: "muw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saYcUrRane",
    },
    Dhatu {
        // 10.0109 `vraja~` saMskAragatyoH (√vraj). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (vrAj-i). Ubhayapadī by
        // 1.3.74 (*vrājayati*). Slice 10k.
        dhatupatha: "10.0109",
        code: "vraj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMskAragatyoH",
    },
    Dhatu {
        // 10.0110 `Sulka~` atisparSane (√śulk). Guru upadhā (the conjunct
        // `lk`), so unchanged before ṇic. Ubhayapadī by 1.3.74 (*śulkayati*).
        // Slice 10k.
        dhatupatha: "10.0110",
        code: "Sulk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "atisparSane",
    },
    Dhatu {
        // 10.0115 `Svarta~` gatyAm (√śvart). Guru upadhā (the conjunct `rt`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*śvartayati*). Slice
        // 10k.
        dhatupatha: "10.0115",
        code: "Svart",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gatyAm",
    },
    Dhatu {
        // 10.0116 `svarta~` kfcCrajIvane, gatyAm (√svart). Guru upadhā (the
        // conjunct `rt`), so unchanged before ṇic. Ubhayapadī by 1.3.74
        // (*svartayati*). Slice 10k.
        dhatupatha: "10.0116",
        code: "svart",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kfcCrajIvane, gatyAm",
    },
    Dhatu {
        // 10.0117 `SvaBra~` gatyAm (√śvabhr). Guru upadhā (the conjunct `Br`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*śvabhrayati*). Slice
        // 10k.
        dhatupatha: "10.0117",
        code: "SvaBr",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gatyAm",
    },
    Dhatu {
        // 10.0125 `Gawwa~` calane (√ghaṭṭ). Guru upadhā (the conjunct `ww`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*ghaṭṭayati*). Slice 10k.
        dhatupatha: "10.0125",
        code: "Gaww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "calane",
    },
    Dhatu {
        // 10.0126 `musta~` saNGAte (√must). Guru upadhā (the conjunct `st`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*mustayati*). Slice 10k.
        dhatupatha: "10.0126",
        code: "must",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0127 `Kawwa~` saMvaraRe (√khaṭṭ). Guru upadhā (the conjunct `ww`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*khaṭṭayati*). Slice
        // 10k.
        dhatupatha: "10.0127",
        code: "Kaww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMvaraRe",
    },
    Dhatu {
        // 10.0128 `zawwa~` hiMsAyAm (√saṭṭ). Guru upadhā (the conjunct `ww`),
        // so unchanged before ṇic. Stored per 6.1.64. Ubhayapadī by 1.3.74
        // (*saṭṭayati*). Slice 10k.
        dhatupatha: "10.0128",
        code: "saww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0129 `sPiwwa~` hiMsAyAm (√sphiṭṭ). Guru upadhā (the conjunct
        // `ww`), so unchanged before ṇic. Ubhayapadī by 1.3.74 (*sphiṭṭayati*).
        // Slice 10k.
        dhatupatha: "10.0129",
        code: "sPiww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0131 `pula~` saNGAte (√pul). 7.3.86 guṇates the laghu upadhā
        // before ṇic (pol-i). Shares every form with `10.0090 pula~`.
        // Ubhayapadī by 1.3.74 (*polayati*). Slice 10k.
        dhatupatha: "10.0131",
        code: "pul",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0132 `pUrRa~` saNGAte (√pūrṇ). Guru upadhā (the conjunct `rR`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*pūrṇayati*). Slice 10k.
        dhatupatha: "10.0132",
        code: "pUrR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0133 `puRa~` saNGAte (√puṇ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (poR-i). Ubhayapadī by 1.3.74 (*poṇayati*). Slice 10k.
        dhatupatha: "10.0133",
        code: "puR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0134 `punsa~` aBivarDane (√puns). Guru upadhā (the conjunct `ns`),
        // so unchanged before ṇic. 8.3.24 makes its `n` an anusvāra before `s`,
        // which stays. Ubhayapadī by 1.3.74 (*puṃsayati*). Slice 10k.
        dhatupatha: "10.0134",
        code: "puns",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "aBivarDane",
    },
    Dhatu {
        // 10.0136 `vyapa~` kzaye (√vyap). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (vyAp-i). Ubhayapadī by 1.3.74
        // (*vyāpayati*). Slice 10k.
        dhatupatha: "10.0136",
        code: "vyap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kzaye",
    },
    Dhatu {
        // 10.0137 `vyaya~` kzaye (√vyay). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (vyAy-i). Ubhayapadī by 1.3.74
        // (*vyāyayati*). Slice 10k.
        dhatupatha: "10.0137",
        code: "vyay",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kzaye",
    },
    Dhatu {
        // 10.0138 `pUla~` saNGAte (√pūl). Long upadhā vowel `U`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*pūlayati*). Slice 10k.
        dhatupatha: "10.0138",
        code: "pUl",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0139 `DUsa~` kAntikaraRe (√dhūs). Long upadhā vowel `U`, so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*dhūsayati*). Slice 10k.
        dhatupatha: "10.0139",
        code: "DUs",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kAntikaraRe",
    },
    Dhatu {
        // 10.0140 `DUza~` kAntikaraRe (√dhūṣ). Long upadhā vowel `U`, so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*dhūṣayati*). Slice 10k.
        dhatupatha: "10.0140",
        code: "DUz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kAntikaraRe",
    },
    Dhatu {
        // 10.0141 `DUSa~` kAntikaraRe (√dhūś). Long upadhā vowel `U`, so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*dhūśayati*). Slice 10k.
        dhatupatha: "10.0141",
        code: "DUS",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kAntikaraRe",
    },
    Dhatu {
        // 10.0142 `kIwa~` varRe (√kīṭ). Long upadhā vowel `I`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*kīṭayati*). Slice 10k.
        dhatupatha: "10.0142",
        code: "kIw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "varRe",
    },
    Dhatu {
        // 10.0143 `cUrRa~` saNkocane (√cūrṇ). Guru upadhā (the conjunct `rR`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*cūrṇayati*). Slice
        // 10k.
        dhatupatha: "10.0143",
        code: "cUrR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNkocane",
    },
    Dhatu {
        // 10.0144 `pUja~` pUjAyAm (√pūj). Long upadhā vowel `U`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*pūjayati*). Slice 10k.
        dhatupatha: "10.0144",
        code: "pUj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pUjAyAm",
    },
    Dhatu {
        // 10.0145 `arka~` stavane (√ark). Guru upadhā (the conjunct `rk`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*arkayati*). Slice 10k.
        dhatupatha: "10.0145",
        code: "ark",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "stavane",
    },
    Dhatu {
        // 10.0146 `SuWa~` Alasye (√śuṭh). 7.3.86 guṇates the laghu upadhā
        // before ṇic (SoW-i). Ubhayapadī by 1.3.74 (*śoṭhayati*). Slice 10k.
        dhatupatha: "10.0146",
        code: "SuW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Alasye",
    },
    Dhatu {
        // 10.0148 `juqa~` preraRe (√juḍ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (joq-i). Ubhayapadī by 1.3.74 (*joḍayati*). Slice 10k.
        dhatupatha: "10.0148",
        code: "juq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "preraRe",
    },
    Dhatu {
        // 10.0149 `gaja~` SabdArTe (√gaj). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (gAj-i). Ubhayapadī by 1.3.74 (*gājayati*).
        // Slice 10k.
        dhatupatha: "10.0149",
        code: "gaj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SabdArTe",
    },
    Dhatu {
        // 10.0150 `mArja~` SabdArTe (√mārj). Guru upadhā (the conjunct `rj`),
        // so unchanged before ṇic. Homograph of the ubhayapadī curādi row
        // `10.0386 mfjU~`: they share every form. Ubhayapadī by 1.3.74
        // (*mārjayati*). Slice 10k.
        dhatupatha: "10.0150",
        code: "mArj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SabdArTe",
    },
    Dhatu {
        // 10.0151 `marca~` SabdArTe (√marc). Guru upadhā (the conjunct `rc`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*marcayati*). Slice
        // 10k.
        dhatupatha: "10.0151",
        code: "marc",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SabdArTe",
    },
    Dhatu {
        // 10.0154 `tija~` niSAne (√tij). 7.3.86 guṇates the laghu upadhā before
        // ṇic (tej-i). Ubhayapadī by 1.3.74 (*tejayati*). Slice 10k.
        dhatupatha: "10.0154",
        code: "tij",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "niSAne",
    },
    Dhatu {
        // 10.0156 `varDa~` CedanapUraRayoH (√vardh). Guru upadhā (the conjunct
        // `rD`), so unchanged before ṇic. Homograph of the ubhayapadī curādi
        // row `10.0313 vfDu~`: they share every form. Ubhayapadī by 1.3.74
        // (*vardhayati*). Slice 10k.
        dhatupatha: "10.0156",
        code: "varD",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "CedanapUraRayoH",
    },
    Dhatu {
        // 10.0161 `hlapa~` vyaktAyAM vAci (√hlap). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (hlAp-i). Ubhayapadī by
        // 1.3.74 (*hlāpayati*). Slice 10k.
        dhatupatha: "10.0161",
        code: "hlap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        // 10.0162 `klapa~` vyaktAyAM vAci (√klap). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (klAp-i). Ubhayapadī by
        // 1.3.74 (*klāpayati*). Slice 10k.
        dhatupatha: "10.0162",
        code: "klap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        // 10.0163 `hrapa~` vyaktAyAM vAci (√hrap). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (hrAp-i). Ubhayapadī by
        // 1.3.74 (*hrāpayati*). Slice 10k.
        dhatupatha: "10.0163",
        code: "hrap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        // 10.0165 `brIsa~` hiMsAyAm (√brīs). Long upadhā vowel `I`, so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*brīsayati*). Slice 10k.
        dhatupatha: "10.0165",
        code: "brIs",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0167 `ila~` preraRe (√il). 7.3.86 guṇates the laghu upadhā before
        // ṇic (el-i). Ubhayapadī by 1.3.74 (*elayati*). Slice 10k.
        dhatupatha: "10.0167",
        code: "il",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "preraRe",
    },
    Dhatu {
        // 10.0168 `mrakza~` mlecCane (√mrakṣ). Guru upadhā (the conjunct `kz`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*mrakṣayati*). Slice
        // 10k.
        dhatupatha: "10.0168",
        code: "mrakz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "mlecCane",
    },
    Dhatu {
        // 10.0169 `asta~` saNGAte (√ast). Guru upadhā (the conjunct `st`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*astayati*). Slice 10k.
        dhatupatha: "10.0169",
        code: "ast",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0172 `brUsa~` hiMsAyAm (√brūs). Long upadhā vowel `U`, so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*brūsayati*). Slice 10k.
        dhatupatha: "10.0172",
        code: "brUs",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0173 `barha~` hiMsAyAm (√barh). Guru upadhā (the conjunct `rh`),
        // so unchanged before ṇic. Homograph of the ubhayapadī curādi row
        // `10.0300 barha~`: they share every form. Ubhayapadī by 1.3.74
        // (*barhayati*). Slice 10k.
        dhatupatha: "10.0173",
        code: "barh",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0176 `bula~` nimajjane (√bul). 7.3.86 guṇates the laghu upadhā
        // before ṇic (bol-i). Ubhayapadī by 1.3.74 (*bolayati*). Slice 10k.
        dhatupatha: "10.0176",
        code: "bul",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "nimajjane",
    },
    Dhatu {
        // 10.0177 `garja~` Sabde (√garj). Guru upadhā (the conjunct `rj`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*garjayati*). Slice 10k.
        dhatupatha: "10.0177",
        code: "garj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Sabde",
    },
    Dhatu {
        // 10.0178 `garda~` Sabde (√gard). Guru upadhā (the conjunct `rd`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*gardayati*). Slice 10k.
        dhatupatha: "10.0178",
        code: "gard",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Sabde",
    },
    Dhatu {
        // 10.0179 `garDa~` aBikANkzAyAm (√gardh). Guru upadhā (the conjunct
        // `rD`), so unchanged before ṇic. Ubhayapadī by 1.3.74 (*gardhayati*).
        // Slice 10k.
        dhatupatha: "10.0179",
        code: "garD",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "aBikANkzAyAm",
    },
    Dhatu {
        // 10.0181 `pUrva~` niketane (√pūrv). Guru upadhā (the conjunct `rv`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*pūrvayati*). Slice
        // 10k.
        dhatupatha: "10.0181",
        code: "pUrv",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "niketane",
    },
    Dhatu {
        // 10.0183 `Iqa~` stutO (√īḍ). Long upadhā vowel `I`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*īḍayati*). Slice 10k.
        dhatupatha: "10.0183",
        code: "Iq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "stutO",
    },
    Dhatu {
        // 10.0186 `parTa~` prakzepe (√parth). Guru upadhā (the conjunct `rT`),
        // so unchanged before ṇic. Shares every form with `10.0028 pfTa~`.
        // Ubhayapadī by 1.3.74 (*parthayati*). Slice 10k.
        dhatupatha: "10.0186",
        code: "parT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prakzepe",
    },
    Dhatu {
        // 10.0187 `ruza~` roze (√ruṣ). 7.3.86 guṇates the laghu upadhā before
        // ṇic (roz-i). Ubhayapadī by 1.3.74 (*roṣayati*). Slice 10k.
        dhatupatha: "10.0187",
        code: "ruz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "roze",
    },
    Dhatu {
        // 10.0188 `ruwa~` roze (√ruṭ). 7.3.86 guṇates the laghu upadhā before
        // ṇic (row-i). Homograph of the ubhayapadī curādi row `10.0314 ruwa~`:
        // they share every form. Ubhayapadī by 1.3.74 (*roṭayati*). Slice 10k.
        dhatupatha: "10.0188",
        code: "ruw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "roze",
    },
    Dhatu {
        // 10.0189 `qipa~` kzepe (√ḍip). 7.3.86 guṇates the laghu upadhā before
        // ṇic (qep-i). Shares its ātmanepada forms with the ākusmīya `10.0197
        // qipa~`. Ubhayapadī by 1.3.74 (*ḍepayati*). Slice 10k.
        dhatupatha: "10.0189",
        code: "qip",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kzepe",
    },
    Dhatu {
        // 10.0190 `zwupa~` samucCrAye (√stup). 7.3.86 guṇates the laghu upadhā
        // before ṇic (stop-i). Stored per 6.1.64. Ubhayapadī by 1.3.74
        // (*stopayati*). Slice 10k.
        dhatupatha: "10.0190",
        code: "stup",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "samucCrAye",
    },
    Dhatu {
        // 10.0191 `zwUpa~` samucCrAye (√stūp). Long upadhā vowel `U`, so
        // unchanged before ṇic. Stored per 6.1.64. Ubhayapadī by 1.3.74
        // (*stūpayati*). Slice 10k.
        dhatupatha: "10.0191",
        code: "stUp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "samucCrAye",
    },
    Dhatu {
        // 10.0237 `carca~` aDyayane (√carc). Guru upadhā (the conjunct `rc`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*carcayati*). Slice
        // 10k.
        dhatupatha: "10.0237",
        code: "carc",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "aDyayane",
    },
    Dhatu {
        // 10.0238 `bukka~` BazaRe (√bukk). Guru upadhā (the conjunct `kk`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*bukkayati*). Slice 10k.
        dhatupatha: "10.0238",
        code: "bukk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BazaRe",
    },
    Dhatu {
        // 10.0239 `Sabda~` AvizkAre, BazaRe (√śabd). Guru upadhā (the conjunct
        // `bd`), so unchanged before ṇic. Ubhayapadī by 1.3.74 (*śabdayati*).
        // Slice 10k.
        dhatupatha: "10.0239",
        code: "Sabd",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AvizkAre, BazaRe",
    },
    Dhatu {
        // 10.0240 `kaRa~` nimIlane (√kaṇ). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (kAR-i). Ubhayapadī by 1.3.74 (*kāṇayati*).
        // Slice 10k.
        dhatupatha: "10.0240",
        code: "kaR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "nimIlane",
    },
    Dhatu {
        // 10.0242 `zUda~` kzaraRe AsravaRe ApravaRe GAte ca (√sūd). Long upadhā
        // vowel `U`, so unchanged before ṇic. Stored per 6.1.64. Ubhayapadī by
        // 1.3.74 (*sūdayati*). Slice 10k.
        dhatupatha: "10.0242",
        code: "sUd",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kzaraRe AsravaRe ApravaRe GAte ca",
    },
    Dhatu {
        // 10.0244 `paSa~` banDane (√paś). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (pAS-i). Ubhayapadī by 1.3.74 (*pāśayati*).
        // Slice 10k.
        dhatupatha: "10.0244",
        code: "paS",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "banDane",
    },
    Dhatu {
        // 10.0245 `ama~` roge (√am). 7.2.116 ata upadhāyāḥ lengthens the `a`
        // upadhā before ṇit ṇic (Am-i). Ubhayapadī by 1.3.74 (*āmayati*). Slice
        // 10k.
        dhatupatha: "10.0245",
        code: "am",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "roge",
    },
    Dhatu {
        // 10.0246 `cawa~` Bedane (√caṭ). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (cAw-i). Ubhayapadī by 1.3.74 (*cāṭayati*).
        // Slice 10k.
        dhatupatha: "10.0246",
        code: "caw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Bedane",
    },
    Dhatu {
        // 10.0247 `sPuwa~` Bedane (√sphuṭ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (sPow-i). Ubhayapadī by 1.3.74 (*sphoṭayati*). Slice 10k.
        dhatupatha: "10.0247",
        code: "sPuw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Bedane",
    },
    Dhatu {
        // 10.0248 `Gawa~` saNGAte (√ghaṭ). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (GAw-i). Homograph of the ubhayapadī curādi
        // row `10.0297 Gawa~`: they share every form. Ubhayapadī by 1.3.74
        // (*ghāṭayati*). Slice 10k.
        dhatupatha: "10.0248",
        code: "Gaw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0250 `arja~` pratiyatne (√arj). Guru upadhā (the conjunct `rj`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*arjayati*). Slice
        // 10k.
        dhatupatha: "10.0250",
        code: "arj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pratiyatne",
    },
    Dhatu {
        // 10.0252 `kranda~` sAtatye (√krand). Guru upadhā (the conjunct `nd`),
        // so unchanged before ṇic. 8.3.24 makes its `n` an anusvāra before `d`,
        // and 8.4.58 restores `n`. Ubhayapadī by 1.3.74 (*krandayati*). Slice
        // 10k.
        dhatupatha: "10.0252",
        code: "krand",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sAtatye",
    },
    Dhatu {
        // 10.0253 `lasa~` Silpayoge (√las). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (lAs-i). Ubhayapadī by 1.3.74 (*lāsayati*).
        // Slice 10k.
        dhatupatha: "10.0253",
        code: "las",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Silpayoge",
    },
    Dhatu {
        // 10.0256 `mokza~` mocane (√mokṣ). Guru upadhā (the conjunct `kz`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*mokṣayati*). Slice 10k.
        dhatupatha: "10.0256",
        code: "mokz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "mocane",
    },
    Dhatu {
        // 10.0257 `arha~` pUjAyAm (√arh). Guru upadhā (the conjunct `rh`), so
        // unchanged before ṇic. Homograph of the ubhayapadī curādi row `10.0367
        // arha~`: they share every form. Ubhayapadī by 1.3.74 (*arhayati*).
        // Slice 10k.
        dhatupatha: "10.0257",
        code: "arh",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pUjAyAm",
    },
    Dhatu {
        // 10.0259 `Baja~` viSrARane (√bhaj). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (BAj-i). Homograph of the ubhayapadī
        // curādi row `10.0428 BAja`: they share every form. Ubhayapadī by
        // 1.3.74 (*bhājayati*). Slice 10k.
        dhatupatha: "10.0259",
        code: "Baj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "viSrARane",
    },
    Dhatu {
        // 10.0261 `yata~` nikAropaskArayoH (√yat). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (yAt-i). Ubhayapadī by 1.3.74
        // (*yātayati*). Slice 10k.
        dhatupatha: "10.0261",
        code: "yat",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "nikAropaskArayoH",
    },
    Dhatu {
        // 10.0262 `raka~` AsvAdane (√rak). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (rAk-i). Ubhayapadī by 1.3.74 (*rākayati*).
        // Slice 10k.
        dhatupatha: "10.0262",
        code: "rak",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AsvAdane",
    },
    Dhatu {
        // 10.0263 `laga~` AsvAdane (√lag). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (lAg-i). Ubhayapadī by 1.3.74 (*lāgayati*).
        // Slice 10k.
        dhatupatha: "10.0263",
        code: "lag",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AsvAdane",
    },
    Dhatu {
        // 10.0264 `raGa~` AsvAdane (√ragh). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (rAG-i). Ubhayapadī by 1.3.74
        // (*rāghayati*). Slice 10k.
        dhatupatha: "10.0264",
        code: "raG",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AsvAdane",
    },
    Dhatu {
        // 10.0265 `raga~` AsvAdane (√rag). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (rAg-i). Ubhayapadī by 1.3.74 (*rāgayati*).
        // Slice 10k.
        dhatupatha: "10.0265",
        code: "rag",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AsvAdane",
    },
    Dhatu {
        // 10.0268 `muda~` saMsarge (√mud). 7.3.86 guṇates the laghu upadhā
        // before ṇic (mod-i). Ubhayapadī by 1.3.74 (*modayati*). Slice 10k.
        dhatupatha: "10.0268",
        code: "mud",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMsarge",
    },
    Dhatu {
        // 10.0269 `trasa~` DAraRe grahaRe vAraRe ca (√tras). 7.2.116 ata
        // upadhāyāḥ lengthens the `a` upadhā before ṇit ṇic (trAs-i).
        // Ubhayapadī by 1.3.74 (*trāsayati*). Slice 10k.
        dhatupatha: "10.0269",
        code: "tras",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "DAraRe grahaRe vAraRe ca",
    },
    Dhatu {
        // 10.0271 `uDrasa~` uYCe (√udhras). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (uDrAs-i). Ubhayapadī by 1.3.74
        // (*udhrāsayati*). Slice 10k.
        dhatupatha: "10.0271",
        code: "uDras",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "uYCe",
    },
    Dhatu {
        // 10.0272 `muca~` pramocane modane ca (√muc). 7.3.86 guṇates the laghu
        // upadhā before ṇic (moc-i). Ubhayapadī by 1.3.74 (*mocayati*). Slice
        // 10k.
        dhatupatha: "10.0272",
        code: "muc",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pramocane modane ca",
    },
    Dhatu {
        // 10.0273 `vasa~` snehacCedApaharaRezu (√vas). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (vAs-i). Homograph of the
        // ubhayapadī curādi row `10.0426 vAsa`: they share every form.
        // Ubhayapadī by 1.3.74 (*vāsayati*). Slice 10k.
        dhatupatha: "10.0273",
        code: "vas",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "snehacCedApaharaRezu",
    },
    Dhatu {
        // 10.0274 `cara~` saMSaye (√car). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (cAr-i). Ubhayapadī by 1.3.74 (*cārayati*).
        // Slice 10k.
        dhatupatha: "10.0274",
        code: "car",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMSaye",
    },
    Dhatu {
        // 10.0276 `cyusa~` sahane hasane ca (√cyus). 7.3.86 guṇates the laghu
        // upadhā before ṇic (cyos-i). Ubhayapadī by 1.3.74 (*cyosayati*). Slice
        // 10k.
        dhatupatha: "10.0276",
        code: "cyus",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sahane hasane ca",
    },
    Dhatu {
        // 10.0397 `raNga~` gatO (√raṅg). Guru upadhā (the conjunct `Ng`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*raṅgayati*). Slice 10k.
        dhatupatha: "10.0397",
        code: "raNg",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gatO",
    },
    Dhatu {
        // 10.0414 `Keqa~` BakzaRe (√kheḍ). Long upadhā vowel `e`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*kheḍayati*). Slice 10k.
        dhatupatha: "10.0414",
        code: "Keq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BakzaRe",
    },
    Dhatu {
        // 10.0438 `kURa~` saNkocane (√kūṇ). Long upadhā vowel `U`, so unchanged
        // before ṇic. Shares its ātmanepada forms with the ākusmīya `10.0211
        // kURa~`. Ubhayapadī by 1.3.74 (*kūṇayati*). Slice 10k.
        dhatupatha: "10.0438",
        code: "kUR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNkocane",
    },
    Dhatu {
        // 10.0457 `karta~` SETilye (√kart). Guru upadhā (the conjunct `rt`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*kartayati*). Slice 10k.
        dhatupatha: "10.0457",
        code: "kart",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SETilye",
    },
    Dhatu {
        // 10.0462 `Cuwa~` Cedane (√chuṭ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (Cow-i). In laṅ the aṭ takes 6.1.73's tuk (acC-).
        // Ubhayapadī by 1.3.74 (*choṭayati*). Slice 10k.
        dhatupatha: "10.0462",
        code: "Cuw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Cedane",
    },
    Dhatu {
        // 10.0470 `karRa~` Bedane (√karṇ). Guru upadhā (the conjunct `rR`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*karṇayati*). Slice 10k.
        dhatupatha: "10.0470",
        code: "karR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Bedane",
    },
    Dhatu {
        // 10.0491 `ruWa~` BAzAyAm (√ruṭh). 7.3.86 guṇates the laghu upadhā
        // before ṇic (roW-i). Ubhayapadī by 1.3.74 (*roṭhayati*). Slice 10k.
        dhatupatha: "10.0491",
        code: "ruW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
"""
end = "];\n\npub fn dhatus() -> &'static [Dhatu] {"
assert s.count(end) == 1
s = s.replace(end, ROWS + end)
open(p, 'w').write(s)
print("inserted 155 rows")
```

```bash
python3 /tmp/vidyut-full/slice10k/rows_10k.py      # inserted 155 rows
```

- [ ] **Step 4: Generate the goldens**

Create `/tmp/vidyut-full/vidyut-prakriya/examples/curadi_goldens_10k.rs` if it is missing (sha256 `94fe49ef9c3f0dcb0ae97d0f0a6d83327c6cf66c5adfc747122d9fc7fb7d2ce4`). It is 10j's generator with the row list changed: it derives every cell of the 155 rows in both engines, asserts the form sets equal, and writes the rows the static wants (pinned form = first live branch; each further distinct form an `ALTERNATES` row keyed by its branch's vikalpa ids in log order). Its `VIKALPA_RULES` is `paradigm/main.rs`'s, unchanged by this slice.

```rust
//! THROWAWAY: slice 10k — emit the 155 plain obligatory-ṇic rows' goldens from the engine,
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
    const NEW: [&str; 155] = [
        "10.0006", "10.0008", "10.0012", "10.0015", "10.0016", "10.0017", "10.0018", "10.0019",
        "10.0020", "10.0021", "10.0024", "10.0025", "10.0027", "10.0028", "10.0029", "10.0030",
        "10.0031", "10.0032", "10.0034", "10.0035", "10.0036", "10.0038", "10.0039", "10.0040",
        "10.0041", "10.0042", "10.0044", "10.0046", "10.0050", "10.0051", "10.0052", "10.0053",
        "10.0054", "10.0055", "10.0056", "10.0057", "10.0059", "10.0061", "10.0063", "10.0064",
        "10.0065", "10.0078", "10.0079", "10.0080", "10.0081", "10.0082", "10.0083", "10.0084",
        "10.0085", "10.0086", "10.0087", "10.0088", "10.0089", "10.0090", "10.0091", "10.0092",
        "10.0093", "10.0094", "10.0095", "10.0096", "10.0097", "10.0098", "10.0099", "10.0100",
        "10.0101", "10.0102", "10.0103", "10.0104", "10.0109", "10.0110", "10.0115", "10.0116",
        "10.0117", "10.0125", "10.0126", "10.0127", "10.0128", "10.0129", "10.0131", "10.0132",
        "10.0133", "10.0134", "10.0136", "10.0137", "10.0138", "10.0139", "10.0140", "10.0141",
        "10.0142", "10.0143", "10.0144", "10.0145", "10.0146", "10.0148", "10.0149", "10.0150",
        "10.0151", "10.0154", "10.0156", "10.0161", "10.0162", "10.0163", "10.0165", "10.0167",
        "10.0168", "10.0169", "10.0172", "10.0173", "10.0176", "10.0177", "10.0178", "10.0179",
        "10.0181", "10.0183", "10.0186", "10.0187", "10.0188", "10.0189", "10.0190", "10.0191",
        "10.0237", "10.0238", "10.0239", "10.0240", "10.0242", "10.0244", "10.0245", "10.0246",
        "10.0247", "10.0248", "10.0250", "10.0252", "10.0253", "10.0256", "10.0257", "10.0259",
        "10.0261", "10.0262", "10.0263", "10.0264", "10.0265", "10.0268", "10.0269", "10.0271",
        "10.0272", "10.0273", "10.0274", "10.0276", "10.0397", "10.0414", "10.0438", "10.0457",
        "10.0462", "10.0470", "10.0491",
    ];
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
    std::fs::write("/tmp/vidyut-full/goldens_10k_paradigm.rs", par).unwrap();
    std::fs::write("/tmp/vidyut-full/goldens_10k_alternates.rs", alt).unwrap();
    println!("{ncells} cells, {nforms} forms, {ndiff} differences");
    assert_eq!(ndiff, 0);
}
```

```bash
WT="$(git rev-parse --show-toplevel)"
V=/tmp/vidyut-full/vidyut-prakriya
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml      # must point at $WT/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example curadi_goldens_10k 2>/dev/null | tail -1)
sha256sum /tmp/vidyut-full/goldens_10k_paradigm.rs /tmp/vidyut-full/goldens_10k_alternates.rs
```

Expected: `11160 cells, 12090 forms, 0 differences`, then:
- `459473ea60c74df2f93ff4c91750d39d0b5bed6a7c0bf7b8f93a5d5c20aef110  /tmp/vidyut-full/goldens_10k_paradigm.rs`
- `8e8ba0a6e3ebddb1105b12159cba1d6b3fc38066a780dbf5b52cd0ceb5799c90  /tmp/vidyut-full/goldens_10k_alternates.rs`

Leave the dev-deps pointing at this worktree; Task 3 uses them. If a line differs, stop and report.

- [ ] **Step 5: Insert the goldens and pin the measured pada-ambiguous set**

Create `/tmp/vidyut-full/slice10k/insert_goldens_10k.py` if it is missing (sha256 `48856307c4a215f8a8c998442f9b6a9789d16ef1b36b95deb586ab748aead0a4`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10k — insert the generator's goldens before each
static's `];`. Run from the worktree root."""
p = 'crates/panini/tests/paradigm/data/curadi.rs'
s = open(p).read()
par = open('/tmp/vidyut-full/goldens_10k_paradigm.rs').read()
alt = open('/tmp/vidyut-full/goldens_10k_alternates.rs').read()
i = s.index('pub const ALTERNATES')
head, tail = s[:i], s[i:]
k = head.rindex('];'); head = head[:k] + par + head[k:]
k = tail.rindex('];'); tail = tail[:k] + alt + tail[k:]
open(p, 'w').write(head + tail)
print("inserted goldens")
```

The pada-ambiguous set is measured, never hand-picked: run the test against the old set and read the real one off its failure. Create `/tmp/vidyut-full/slice10k/pin_ambiguous_10k.py` if it is missing (sha256 `5d7d1ad79e9e2fe14bd5354fc884b29721cab454fe85b88f38ab43f2cdfaec89`). It refuses a set whose size or hash differs from the prototype's.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10k — pin the measured pada-ambiguous set ($SET, the
failing test's `left:` JSON) into pada_ambiguous_surfaces_are_exactly_these,
and extend the comment that accounts for it. Run from the worktree root."""
import hashlib, json, os
amb = json.load(open(os.environ['SET']))
assert len(amb) == 1603, len(amb)
h = hashlib.sha256('\n'.join(amb).encode()).hexdigest()
assert h == '5825d3ed642b5c6f8ee93fc07d0ed64a2ad412f1c0c44245ddb76ebcb088d399', h
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
old = """    // √gṛ, √yu and √smiṅ are ātmanepadī only.
"""
assert s.count(old) == 1
s = s.replace(old, old + """    // Slice 10k's 155 rows contribute the same four each (−ayata, −ayatAm,
    // −ayetAm, −ayeta), less the sixteen whose partner is an ubhayapadī row
    // curated earlier and already holds the four (−64), and less one row of
    // each in-slice pair, `pfT` / `parT`, `pul` twice, `pAl` / `pal` and
    // `sAntv` twice (−16). The five whose partner is ākusmīya add theirs.
    // 540 more, taking the set from 1063 to 1603.
""")
i = s.index("fn pada_ambiguous_surfaces_are_exactly_these")
j = s.index("        both,\n        vec![", i)
k = s.index("        ]", j)
s = s[:j] + "        both,\n        vec![\n" + "".join(f'            "{x}",\n' for x in amb) + s[k:]
open(p, 'w').write(s)
print("pinned", len(amb))
```

```bash
python3 /tmp/vidyut-full/slice10k/insert_goldens_10k.py      # inserted goldens
mise run fmt
SET="$(mktemp)"
{ mise exec -- cargo test -q -p panini --test paradigm pada_ambiguous 2>&1 || true; } | grep "^  left:" | sed 's/^  left: //' > "$SET"
SET="$SET" python3 /tmp/vidyut-full/slice10k/pin_ambiguous_10k.py      # pinned 1603
mise run fmt
```

The 540 new surfaces are the 155 rows' four each (`-ayata`, `-ayatAm`, `-ayetAm`, `-ayeta`), less the sixteen rows whose ubhayapadī partner already holds them (−64) and one row of each in-slice pair (−16).

- [ ] **Step 6: Run the full suite**

```bash
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS at 37584 cells, with `panini-prakriya` 437, `trace` 215, `paradigm` 29, `panini-data` 28.

Grep the goldens for the `check()` witnesses, which the tests also enforce:

```bash
for f in medayati codayati jAlayati pIqayati sambayati puMsayati picCayati lakzayati kuwwayati SAWayati qepayati kURayati \
         acCardayat parTayati polayati pAlayati sAntvayati SrATayati varRayati valkayati SrARayati tAqayati Cardayati kAlayati velayati mArjayati varDayati barhayati rowayati GAwayati arhayati BAjayati vAsayati \
         medati codati jalati pIqati midayati jalayati sanbayati punsayati piCayati; do
  printf "%s: %s\n" $f "$(grep -c "\"$f\"" crates/panini/tests/paradigm/data/*.rs | grep -v ':0' | sed 's#.*/##' | tr '\n' ' ')"; done
```

Expected:
- The first twelve appear once, in `curadi.rs` only.
- The next twenty-one (`acCardayat` … `vAsayati`) appear twice, in `curadi.rs` only.
- The nine Invalid shapes (`medati` … `piCayati`) print nothing.

- [ ] **Step 7: Commit**

```bash
mise run lint
git branch --show-current      # curadi-10k
git add -A
git commit -m "feat(data): curādi's 155 plain obligatory-ṇic rows

26424 → 37584 cells, 37070 → 49160 forms, ALTERNATES 10646 → 11576, 430 →
585 roots; pada-ambiguous surfaces 1063 → 1603. No engine change: 37 rows
by 7.3.86, 40 by 7.2.116, 78 unchanged before ṇic, all ubhayapadī by 1.3.74.
zAntva~ and sAntva~ converge on sAntv by 6.1.64 and are a pinned sibling
pair. Goldens generated cell-by-cell equal to vidyut."
```

---

## Task 3: Audit, prior-trace diff, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`
- Modify: `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`
- Modify: `crates/panini/tests/paradigm/main.rs` (doc counts, alternates doc, audit chain), `crates/panini/tests/trace/curadi.rs` (module doc), `crates/panini-prakriya/src/tinanta/sanadi.rs` (two comments), `crates/panini-prakriya/src/tinanta/guna.rs` (one comment)
- Modify: the 10c and 10j specs

**Interfaces:**
- Consumes: the finished data and goldens. Produces no symbols.

- [ ] **Step 1: Update the audit harness**

Create `/tmp/vidyut-full/slice10k/audit_10k.py` if it is missing (sha256 `c4d262b917f4af9b18722f0c1d40214c242813cf6b53270233d7c7c0ffcf588a`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10k's edits to tools/audit/panini_full_audit.rs — the
asserted totals and the header's counts. Every `old` must occur exactly once;
nothing is written if one fails. Run from the worktree root."""
import sys
E = {
'tools/audit/panini_full_audit.rs': [
("""//! removes that circularity.
//!
//! What it compares: for each of the 430 curated roots, for each pada the root
//! admits (`Dhatu::padas`; two apiece for the 304 roots that admit both padas —
//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 264 curādi
//! roots by 1.3.74, seven more by 1.3.74 with ṇic and 1.3.72 without, and seven optional-ṇic ākusmīya or ā-garvīya roots by
//! 10.0496 / 10.0497 with ṇic and 1.3.78 without), for each of the four
""",
"""//! removes that circularity.
//!
//! What it compares: for each of the 585 curated roots, for each pada the root
//! admits (`Dhatu::padas`; two apiece for the 459 roots that admit both padas —
//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 419 curādi
//! roots by 1.3.74, seven more by 1.3.74 with ṇic and 1.3.72 without, and seven optional-ṇic ākusmīya or ā-garvīya roots by
//! 10.0496 / 10.0497 with ṇic and 1.3.78 without), for each of the four
"""),
("""//! (often the bare root code), not a surface form.
//!
//! Corpus invariants, asserted: 430 roots, 26424 cells, 37070 forms. These are
//! facts about the repo, pinned by its own golden suite
//! (`derivation_set_shape_matches_the_audited_numbers`): 2936 root×pada×lakāra
//! blocks × 9 cells, plus 10646 `ALTERNATES` rows. If this harness's
//! enumeration disagrees, the harness is wrong.
//!
""",
"""//! (often the bare root code), not a surface form.
//!
//! Corpus invariants, asserted: 585 roots, 37584 cells, 49160 forms. These are
//! facts about the repo, pinned by its own golden suite
//! (`derivation_set_shape_matches_the_audited_numbers`): 4176 root×pada×lakāra
//! blocks × 9 cells, plus 11576 `ALTERNATES` rows. If this harness's
//! enumeration disagrees, the harness is wrong.
//!
"""),
("""//!     PANINI_AUDIT_PERTURB=entry cargo run --release --example panini_full_audit
//!
//! Optionally dump the full 26424-cell table:
//!
//!     PANINI_AUDIT_DUMP=/path/to/table.tsv cargo run --release --example panini_full_audit
""",
"""//!     PANINI_AUDIT_PERTURB=entry cargo run --release --example panini_full_audit
//!
//! Optionally dump the full 37584-cell table:
//!
//!     PANINI_AUDIT_DUMP=/path/to/table.tsv cargo run --release --example panini_full_audit
"""),
("""    println!("differing cells  : {}", diffs.len());

    assert_eq!(roots_seen.len(), 430, "curated roots");
    assert_eq!(n_cells, 26424, "cells: 2936 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 37070, "forms: 26424 cells + 10646 ALTERNATES rows");
    assert_eq!(
        n_branches, n_forms,
""",
"""    println!("differing cells  : {}", diffs.len());

    assert_eq!(roots_seen.len(), 585, "curated roots");
    assert_eq!(n_cells, 37584, "cells: 4176 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 49160, "forms: 37584 cells + 11576 ALTERNATES rows");
    assert_eq!(
        n_branches, n_forms,
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
python3 /tmp/vidyut-full/slice10k/audit_10k.py      # applied 4 edits
```

The harness now asserts 585 / 37584 / 49160 and names 459 both-pada roots (304 + 155), 419 of them `Nic`.

- [ ] **Step 2: The prior-trace diff and the audit**

The dev-deps still point at this worktree from Task 2. Create `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_10k.rs` if it is missing (sha256 `e6346e0d0e737e72f66505e4b8635b8a0057bc0f3d6fddf5a80aaf334adeb5c0`). It lists 10k's rows literally so that it also builds against main, and it dumps every branch, blocked ones included:

```rust
//! THROWAWAY: slice 10k — dump every prior cell's branches, blocked ones
//! included, each with its credited-rule log.
use panini::Panini;
use panini_data::{Lakara as L, Purusha as P, Vacana as V};
/// Slice 10k's 155 rows, listed literally so the dump also builds against
/// main.
const NEW: &[&str] = &[
    "10.0006", "10.0008", "10.0012", "10.0015", "10.0016", "10.0017", "10.0018", "10.0019",
    "10.0020", "10.0021", "10.0024", "10.0025", "10.0027", "10.0028", "10.0029", "10.0030",
    "10.0031", "10.0032", "10.0034", "10.0035", "10.0036", "10.0038", "10.0039", "10.0040",
    "10.0041", "10.0042", "10.0044", "10.0046", "10.0050", "10.0051", "10.0052", "10.0053",
    "10.0054", "10.0055", "10.0056", "10.0057", "10.0059", "10.0061", "10.0063", "10.0064",
    "10.0065", "10.0078", "10.0079", "10.0080", "10.0081", "10.0082", "10.0083", "10.0084",
    "10.0085", "10.0086", "10.0087", "10.0088", "10.0089", "10.0090", "10.0091", "10.0092",
    "10.0093", "10.0094", "10.0095", "10.0096", "10.0097", "10.0098", "10.0099", "10.0100",
    "10.0101", "10.0102", "10.0103", "10.0104", "10.0109", "10.0110", "10.0115", "10.0116",
    "10.0117", "10.0125", "10.0126", "10.0127", "10.0128", "10.0129", "10.0131", "10.0132",
    "10.0133", "10.0134", "10.0136", "10.0137", "10.0138", "10.0139", "10.0140", "10.0141",
    "10.0142", "10.0143", "10.0144", "10.0145", "10.0146", "10.0148", "10.0149", "10.0150",
    "10.0151", "10.0154", "10.0156", "10.0161", "10.0162", "10.0163", "10.0165", "10.0167",
    "10.0168", "10.0169", "10.0172", "10.0173", "10.0176", "10.0177", "10.0178", "10.0179",
    "10.0181", "10.0183", "10.0186", "10.0187", "10.0188", "10.0189", "10.0190", "10.0191",
    "10.0237", "10.0238", "10.0239", "10.0240", "10.0242", "10.0244", "10.0245", "10.0246",
    "10.0247", "10.0248", "10.0250", "10.0252", "10.0253", "10.0256", "10.0257", "10.0259",
    "10.0261", "10.0262", "10.0263", "10.0264", "10.0265", "10.0268", "10.0269", "10.0271",
    "10.0272", "10.0273", "10.0274", "10.0276", "10.0397", "10.0414", "10.0438", "10.0457",
    "10.0462", "10.0470", "10.0491",
];
fn main() {
    assert_eq!(NEW.len(), 155);
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
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10k 2>/dev/null > "$DUMP/branch.txt")
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml   # must point at /workspace/crates
git -C /workspace branch --show-current   # main
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10k 2>/dev/null > "$DUMP/main.txt")
wc -l < "$DUMP/main.txt"; wc -l < "$DUMP/branch.txt"            # 43586 and 43586
grep -c "blocked=false" "$DUMP/main.txt"                        # 37070
cmp "$DUMP/main.txt" "$DUMP/branch.txt" && echo PRIOR-TRACES-IDENTICAL
```

Expected: `43586`, `43586`, `37070`, `PRIOR-TRACES-IDENTICAL`. This is the corpus-wide check that no prior root's trace moves, blocked branches included (goldens ignore traces). `/workspace` must be on `main` for the second dump. If `cmp` reports a difference, stop and report.

Then the audit. Repoint at this worktree again, copy the committed harness (never rewrite it), and run:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
cp tools/audit/panini_full_audit.rs $V/examples/
(cd $V && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -2)
(cd $V && PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -9)
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml
date -u +%F
```

- Expected from the `entry` control, run first: `AUDIT FAILED: 36 differing cells.`
- Expected from the honest run: `roots : 585`, `cells : 37584`, `forms (set sizes): 49160`, `live branches : 49160`, `blocked branches : 6516`, `differing cells  : 0`, `AUDIT PASSED: 37584 cells, 49160 forms, zero differences.`

Do not use `mise -C`. If the honest run shows differences, stop and report, and edit nothing. Keep the date `date -u +%F` prints: Step 3 takes it.

- [ ] **Step 3: The doc sweep, the audit record and the spec pointers**

Create `/tmp/vidyut-full/slice10k/docsweep_10k.py` if it is missing (sha256 `3d68fc6c2efc6fb4ca9c27c5b784a3498b3e0695c74a4a6dd7bfdecc8c06b5dd`). It takes the audit's date (Step 2's `date -u +%F`) and writes it into the audit record and AGENTS.md's audit chain. It covers:
- **README.md:** the curādi line (482) and a 10k paragraph; the corpus (585 roots; 8396 of 37584 cells forked; 6742 / 911); the both-pada count (459) and list; the pada-ambiguous count (1603) and its account.
- **AGENTS.md:** the cell count, the progress line (482), the fork census (6742 / 911; 11576 + 37584 = 49160), the audit chain, and the `guna.rs:1233` note's corpus size. Not the mutation paragraphs: Task 4 rewrites those.
- **ARCHITECTURE.md:** the gaṇa coverage line (482); the 7.1.35 / 8.4.56 census (1022 cells across 511 parasmaipada columns, 74 ātmanepada-only, 459 both-pada; 882 / 859 outright; the thirty-seven laghu-ik rows in `7.3.86+8.4.56`; 511 + 74 = 585).
- **`paradigm/main.rs` docs:** `ALTERNATES` 11576; the `7.3.86+8.4.56` account (144 of 151 mandatory; 134 curādi); the census doc (37584 / 4176 / 29188 / 6742 / 911); the alternates doc's key counts and a 10k sentence; the audit chain.
- **`sanadi.rs`:** the module doc's and the 6.1.73 entry's "√vich" become √vich and √pich, without changing either comment's line count (so no mutant span moves).
- **`trace/curadi.rs`:** the module doc's √vich clause gains √pich. **`guna.rs`:** 430-root → 585-root.
- **`tools/audit/README.md`:** the asserted totals and the new top record.
- **The 10j spec's** first Later-slices bullet and **the 10c spec's** homograph decision gain pointers to the 10k spec.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10k's doc sweep — README, AGENTS.md, ARCHITECTURE.md, the
audit README's record, the paradigm and trace docs, the 6.1.73 comments,
guna.rs's corpus size, and the 10c and 10j spec pointers. Takes the audit's
date (`date -u +%F` on the day Task 3 Step 2 ran) as its one argument and
writes it where the text says AUDITDATE. Every `old` must occur exactly once;
nothing is written if one fails. Run from the worktree root."""
import sys
E = {
'AGENTS.md': [
("""  `#![no_main]` plus the libfuzzer harness macro).
- Grammar changes are gated by the golden paradigm test
  (`crates/panini/tests/paradigm/`, 26424 cells, ten gaṇas, nine complete —
  tanādi closing at 10/10 in slice 8b (nine of its ten dhātupāṭha rows
  curated in slice 8a; √kṛ, the tenth and last, in 8b), and juhotyādi (3)
""",
"""  `#![no_main]` plus the libfuzzer harness macro).
- Grammar changes are gated by the golden paradigm test
  (`crates/panini/tests/paradigm/`, 37584 cells, ten gaṇas, nine complete —
  tanādi closing at 10/10 in slice 8b (nine of its ten dhātupāṭha rows
  curated in slice 8a; √kṛ, the tenth and last, in 8b), and juhotyādi (3)
"""),
("""  after slice 3f2 curated √bhas, and closing at 26 of 26 in slice 3f3 with √jan,
and curādi (10) opened in slice 10a at 4 of its 509 rows (√cur, √laḍ, √bhakṣ,
√bhūṣ), at 8 after slice 10b curated the ākusmīya √cit, √vṛṣ, √mad and √kusm, at 41 after slice 10c curated thirty-three more ākusmīya roots, at 47 after slice 10d curated the jñapādi √jñap, √yam, √cah, √cap, √rah and √bal, at 139 after slice 10e curated ninety-two adanta roots, at 149 after slice 10f curated the ten optional-ṇic rows, at 208 after slice 10g curated fifty-nine more, at 258 after slice 10h curated the fifty ādhṛṣīya rows, at 319 after slice 10i curated the fifty-nine āsvadīya rows, √pṝ and √ghuṣ, at 327 after slice 10j curated the eight ajanta rows √smiṅ, √ci, √ghṛ, √gṛ, √yu, √jñā, √cyu and √bhū —
  `PARADIGM`
    stays one-form-per-cell: a cell forked by an optional rule keeps its
    other forms — a second (6432 cells), a third (601 cells), a fourth
    (359
    cells, rudhādi's √piṣ and — new in slice 7d — √śiṣ loṭ madhyama eka, and
""",
"""  after slice 3f2 curated √bhas, and closing at 26 of 26 in slice 3f3 with √jan,
and curādi (10) opened in slice 10a at 4 of its 509 rows (√cur, √laḍ, √bhakṣ,
√bhūṣ), at 8 after slice 10b curated the ākusmīya √cit, √vṛṣ, √mad and √kusm, at 41 after slice 10c curated thirty-three more ākusmīya roots, at 47 after slice 10d curated the jñapādi √jñap, √yam, √cah, √cap, √rah and √bal, at 139 after slice 10e curated ninety-two adanta roots, at 149 after slice 10f curated the ten optional-ṇic rows, at 208 after slice 10g curated fifty-nine more, at 258 after slice 10h curated the fifty ādhṛṣīya rows, at 319 after slice 10i curated the fifty-nine āsvadīya rows, √pṝ and √ghuṣ, at 327 after slice 10j curated the eight ajanta rows √smiṅ, √ci, √ghṛ, √gṛ, √yu, √jñā, √cyu and √bhū, at 482 after slice 10k curated the 155 plain obligatory-ṇic rows —
  `PARADIGM`
    stays one-form-per-cell: a cell forked by an optional rule keeps its
    other forms — a second (6742 cells), a third (911 cells), a fourth
    (359
    cells, rudhādi's √piṣ and — new in slice 7d — √śiṣ loṭ madhyama eka, and
"""),
("""    √dhū and √prī and slice 10j's √ci loṭ parasmaipada prathama and madhyama
    eka, the eight nine-form cells and the record — in
    `ALTERNATES` (10646 rows in all, so 26424 + 10646 = 37070 forms total); √bhuj
    joins neither fork record — its forks stack only 7.1.35 and 8.4.56, the
    same two-deep profile as √yuj — but the √bhuj/1.3.66 slice adds two
""",
"""    √dhū and √prī and slice 10j's √ci loṭ parasmaipada prathama and madhyama
    eka, the eight nine-form cells and the record — in
    `ALTERNATES` (11576 rows in all, so 37584 + 11576 = 49160 forms total); √bhuj
    joins neither fork record — its forks stack only 7.1.35 and 8.4.56, the
    same two-deep profile as √yuj — but the √bhuj/1.3.66 slice adds two
"""),
("""  entry, 25956 cells / 36416 forms / 422 roots), and that by curādi 10j's
  (`tools/audit/README.md`'s 2026-10-04 10j entry, 26424 cells / 37070 forms /
  430 roots).
  Three new `Rule`s are behind it, all root-keyed to √kṛ and all in
  `guna.rs` — 6.4.110 *ata ut sārvadhātuke*, 6.4.108 *nityaṁ karoteḥ* and
""",
"""  entry, 25956 cells / 36416 forms / 422 roots), and that by curādi 10j's
  (`tools/audit/README.md`'s 2026-10-04 10j entry, 26424 cells / 37070 forms /
  430 roots), and that by curādi 10k's (`tools/audit/README.md`'s AUDITDATE 10k
  entry, 37584 cells / 49160 forms / 585 roots).
  Three new `Rule`s are behind it, all root-keyed to √kṛ and all in
  `guna.rs` — 6.4.110 *ata ut sārvadhātuke*, 6.4.108 *nityaṁ karoteḥ* and
"""),
("""  test), not 8 — the "8 cells" figure was never re-derived when the gaṇa
  landed. `guna.rs:1233`'s own claim ("1872 goldens move") stays stale
  only in the ordinary corpus-size sense, not wrong in kind: 26424 goldens
  would move today. Neither comment was touched by tanādi 8a or 8b, consistent
  with every slice since 7c. Rudhādi 7d touched neither comment — its one permitted
""",
"""  test), not 8 — the "8 cells" figure was never re-derived when the gaṇa
  landed. `guna.rs:1233`'s own claim ("1872 goldens move") stays stale
  only in the ordinary corpus-size sense, not wrong in kind: 37584 goldens
  would move today. Neither comment was touched by tanādi 8a or 8b, consistent
  with every slice since 7c. Rudhādi 7d touched neither comment — its one permitted
"""),
],
'README.md': [
("""twelfth vikalpa, 6.4.43 *ye vibhāṣā* (*jajanyāt* ~ *jajāyāt*) — with 8.4.40
*stoḥ ścunā ścuḥ* gaining its converse arm, a stu after a ścu, guarded by the
8.4.44 *śāt* exemption. *curādi* (10) is **open** at 327 of its 509
dhātupāṭha rows: √cur (`10.0001`, *corayati*), √laḍ (`10.0010`,
*lāḍayati*), √bhakṣ (`10.0033`) and √bhūṣ (`10.0255`), curated in slice 10a,
""",
"""twelfth vikalpa, 6.4.43 *ye vibhāṣā* (*jajanyāt* ~ *jajāyāt*) — with 8.4.40
*stoḥ ścunā ścuḥ* gaining its converse arm, a stu after a ścu, guarded by the
8.4.44 *śāt* exemption. *curādi* (10) is **open** at 482 of its 509
dhātupāṭha rows: √cur (`10.0001`, *corayati*), √laḍ (`10.0010`,
*lāḍayati*), √bhakṣ (`10.0033`) and √bhūṣ (`10.0255`), curated in slice 10a,
"""),
("""7.3.36 *artihrīvlīrīknūyīkṣmāyyātāṃ puk ṇau* adds puk to that and to √jñā
(*jñāpayati*), and 6.4.92 shortens both mit branches (*capayati*,
*cayayati*, beside *cayati*).
Every curādi root takes ṇic (3.1.25) before the vikaraṇa; a new first
pipeline stage adds it, guṇates or
""",
"""7.3.36 *artihrīvlīrīknūyīkṣmāyyātāṃ puk ṇau* adds puk to that and to √jñā
(*jñāpayati*), and 6.4.92 shortens both mit branches (*capayati*,
*cayayati*, beside *cayati*). Slice 10k curated the 155 plain
obligatory-ṇic rows that need nothing new, all ubhayapadī by 1.3.74: 7.3.86
or 7.2.116 before ṇic or neither (*codayati*, *jālayati*, *pīḍayati*), and
existing rules on new rows: 8.3.24 and 8.4.58 on a root's own `n`
(*sambayati*, *puṃsayati*) and the sanādi 6.1.73's tuk on √pich
(*picchayati*). Two of its rows, `10.0051 zAntva~` and `10.0052 sAntva~`,
are the same root once 6.1.64 makes the `z` an `s`; both are curated.
Every curādi root takes ṇic (3.1.25) before the vikaraṇa; a new first
pipeline stage adds it, guṇates or
"""),
("""since neither engine models sense. *parasmaipada* and *ātmanepada*
(which padas a root admits is a curated verdict on its table row), over a
curated 430-root set, in four lakāras: *laṭ* (present), *laṅ* (imperfect), *loṭ*
(imperative), and *vidhiliṅ* (optative). A cell may have more than one valid
form where an optional (*vikalpa*) sūtra applies — `hinvaH` and `hinuvaH` are
both correct — and in fact 7776 of the 26424 cells hold more than one form: 6432
hold two, 601 hold three (`Bavatu`, `BavatAd`, `BavatAt`, and — new in
slice 3b — √hrī's loṭ prathama and madhyama eka, and — new in slice 3c —
√dā's and √dhā's, and — new in slice 3c2 — √gā's, and — new in slice 3d —
""",
"""since neither engine models sense. *parasmaipada* and *ātmanepada*
(which padas a root admits is a curated verdict on its table row), over a
curated 585-root set, in four lakāras: *laṭ* (present), *laṅ* (imperfect), *loṭ*
(imperative), and *vidhiliṅ* (optative). A cell may have more than one valid
form where an optional (*vikalpa*) sūtra applies — `hinvaH` and `hinuvaH` are
both correct — and in fact 8396 of the 37584 cells hold more than one form: 6742
hold two, 911 hold three (`Bavatu`, `BavatAd`, `BavatAt`, and — new in
slice 3b — √hrī's loṭ prathama and madhyama eka, and — new in slice 3c —
√dā's and √dhā's, and — new in slice 3c2 — √gā's, and — new in slice 3d —
"""),
("""√jan's, and — new in slice 10a — the four curādi roots', and — new in slice
10d — the six jñapādi roots', and — new in slice 10e — the eighty-three
ubhayapadī adanta roots', each by
7.1.35/8.4.56, √bhas's laṅ madhyama eka by 8.2.74/8.4.56; slice 10f's
optional-ṇic rows add 114 two-form and 46 three-form cells, and slice 10g's
""",
"""√jan's, and — new in slice 10a — the four curādi roots', and — new in slice
10d — the six jñapādi roots', and — new in slice 10e — the eighty-three
ubhayapadī adanta roots', and — new in slice 10k — the 155 plain
obligatory-ṇic rows', each by
7.1.35/8.4.56, √bhas's laṅ madhyama eka by 8.2.74/8.4.56; slice 10f's
optional-ṇic rows add 114 two-form and 46 three-form cells, and slice 10g's
"""),
("""`t`/`D` that follows, so it never reaches the 8.4.65 branch the dental-final
roots take. A root may also admit **both**
padas — 304 roots that admit both padas in the curated set
(twenty-five ubhayapadī by 1.3.72: √nī, √tud, √rudh, √bhid, √kṣud, √yuj,
√tṛd, √ric, √vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛṇ, √tṛ, √ghṛ,
""",
"""`t`/`D` that follows, so it never reaches the 8.4.65 branch the dental-final
roots take. A root may also admit **both**
padas — 459 roots that admit both padas in the curated set
(twenty-five ubhayapadī by 1.3.72: √nī, √tud, √rudh, √bhid, √kṣud, √yuj,
√tṛd, √ric, √vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛṇ, √tṛ, √ghṛ,
"""),
("""`pata`, slice 10g's fifty-nine optional-ṇic rows, slice 10h's fifty
ādhṛṣīya rows (the six svarita or ñit among them by 1.3.72 too, without
ṇic), slice 10i's sixty-one rows and slice 10j's √ghṛ, √jñā, √cyu, √bhū and
√ci (by 1.3.72 too, without ṇic) by 1.3.74; and slice 10f's six optional-ṇic ākusmīya roots and
`garva`, ātmanepadī by 10.0496 / 10.0497 with ṇic and parasmaipadī by
1.3.78 without) derive a full
""",
"""`pata`, slice 10g's fifty-nine optional-ṇic rows, slice 10h's fifty
ādhṛṣīya rows (the six svarita or ñit among them by 1.3.72 too, without
ṇic), slice 10i's sixty-one rows, slice 10j's √ghṛ, √jñā, √cyu, √bhū and
√ci (by 1.3.72 too, without ṇic) and slice 10k's 155 rows by 1.3.74; and slice 10f's six optional-ṇic ākusmīya roots and
`garva`, ātmanepadī by 10.0496 / 10.0497 with ṇic and parasmaipadī by
1.3.78 without) derive a full
"""),
("""here, not modelled, on 1.3.72's own sense-restriction precedent, so this
engine's √van has no parasmaipada branch to collide against.
1063 of the pinned (`PARADIGM`) surfaces are pada-ambiguous, each of them a
pinned cell in both padas at once — `rundDAm`, for instance, is √rudh's loṭ
parasmaipada prathama dvi *and* its loṭ ātmanepada prathama eka, and tanādi's seven ubhayapadī roots
""",
"""here, not modelled, on 1.3.72's own sense-restriction precedent, so this
engine's √van has no parasmaipada branch to collide against.
1603 of the pinned (`PARADIGM`) surfaces are pada-ambiguous, each of them a
pinned cell in both padas at once — `rundDAm`, for instance, is √rudh's loṭ
parasmaipada prathama dvi *and* its loṭ ātmanepada prathama eka, and tanādi's seven ubhayapadī roots
"""),
("""repeat earlier curādi codes) or an in-slice pair shares them (`laGi~`
twice, `svad` and `svAd`); slice 10j's √ghṛ, √jñā, √cyu and √ci add 16 more,
four each, √bhū's being the ādhṛṣīya `10.0382 BU`'s already. The
enumeration is not
maintained by hand: `pada_ambiguous_surfaces_are_exactly_these` in
`crates/panini/tests/paradigm/main.rs` walks `PARADIGM` and asserts the whole
set, all 1063. It is therefore a list of ambiguous **pinned cells**, not of every
pada-ambiguous surface: since slice 10h's ṇic-less branches are live in both
padas, some surfaces that only `ALTERNATES` pins (`avadata`) are ambiguous too. An *alternate* form
""",
"""repeat earlier curādi codes) or an in-slice pair shares them (`laGi~`
twice, `svad` and `svAd`); slice 10j's √ghṛ, √jñā, √cyu and √ci add 16 more,
four each, √bhū's being the ādhṛṣīya `10.0382 BU`'s already; slice 10k's 155
rows add 540 more, four each but where an ubhayapadī row curated earlier
already supplies them (sixteen homographs) or an in-slice pair shares them
(`pfT` and `parT`, `pul` twice, `pAl` and `pal`, `sAntv` twice). The
enumeration is not
maintained by hand: `pada_ambiguous_surfaces_are_exactly_these` in
`crates/panini/tests/paradigm/main.rs` walks `PARADIGM` and asserts the whole
set, all 1603. It is therefore a list of ambiguous **pinned cells**, not of every
pada-ambiguous surface: since slice 10h's ṇic-less branches are live in both
padas, some surfaces that only `ALTERNATES` pins (`avadata`) are ambiguous too. An *alternate* form
"""),
],
'crates/panini-prakriya/src/tinanta/guna.rs': [
("""    // 6.1.78 eco'yavāyāvaḥ: e/o before a vowel → ay/av. The sūtra also covers
    // E/O → Ay/Av, but those two arms are dropped here: within the current
    // 430-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)
    // at the point this rule runs. `vrddhi_of` (the only source of E/O in
    // this engine) is called by 6.1.90 (three) and 6.1.88 (one) in
""",
"""    // 6.1.78 eco'yavāyāvaḥ: e/o before a vowel → ay/av. The sūtra also covers
    // E/O → Ay/Av, but those two arms are dropped here: within the current
    // 585-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)
    // at the point this rule runs. `vrddhi_of` (the only source of E/O in
    // this engine) is called by 6.1.90 (three) and 6.1.88 (one) in
"""),
],
'crates/panini-prakriya/src/tinanta/sanadi.rs': [
("""//! 6.1.73 and 3.1.28 carry no gaṇa guard, only their own conditions (3.1.28
//! reads the āya rows); on today's corpus the gaṇas 1–9 add nothing and
//! record nothing, and 6.1.73 fires only on √vich, which
//! `the_10i_aya_and_tuk_fire_only_on_their_rows` pins.

use crate::prakriya::Prakriya;
""",
"""//! 6.1.73 and 3.1.28 carry no gaṇa guard, only their own conditions (3.1.28
//! reads the āya rows); on today's corpus the gaṇas 1–9 add nothing and
//! record nothing, and 6.1.73 fires only on √vich and √pich (slice 10k),
//! which `the_10i_aya_and_tuk_fire_only_on_their_rows` pins.

use crate::prakriya::Prakriya;
"""),
("""        },
    },
    // 6.1.73 che ca, before ṇic or āya: √vich's `i` takes tuk here, before
    // the sanādi 7.3.86 below can read it as a laghu upadhā (`viC` →
    // `vitC`, so *vicchayati*, not *vechayati*; 8.4.40 later makes the `t`
    // a `c`). vidyut-prakriya credits it at this point ("tuk-Agama can block
""",
"""        },
    },
    // 6.1.73 che ca, before ṇic or āya: √vich's `i` (√pich's too, slice 10k)
    // takes tuk here, before the sanādi 7.3.86 reads it as a laghu upadhā (`viC` →
    // `vitC`, so *vicchayati*, not *vechayati*; 8.4.40 later makes the `t`
    // a `c`). vidyut-prakriya credits it at this point ("tuk-Agama can block
"""),
],
'crates/panini/tests/paradigm/main.rs': [
("""];

/// `ALTERNATES` is otherwise 10646 bare strings, and a string can be right for
/// the wrong reason — `BavatAt` is a real form whether or not 8.4.56 is what
/// produced it. This ties each row to the grammar: find the branch that
""",
"""];

/// `ALTERNATES` is otherwise 11576 bare strings, and a string can be right for
/// the wrong reason — `BavatAt` is a real form whether or not 8.4.56 is what
/// produced it. This ties each row to the grammar: find the branch that
"""),
("""/// as the mandatory guṇa-stage laghūpadha guṇa, as the tanādi vikalpa entry and,
/// since slice 10a, as the sanādi entry before ṇic, so a key
/// naming 7.3.86 does not by itself mean the rule was optional. Seventy of the 77 `7.3.86+8.4.56` keys are the mandatory firing (ten juhotyādi: 3e's laṅ eka cells and 3f's √kit and √dhiṣ ones; sixty curādi: slice 10a's √cur laṅ and vidhiliṅ prathama eka, and slice 10g's √div and √śṛdh, slice 10h's sixteen laghu-ik ādhṛṣīya rows and slice 10i's eleven laghu-ik rows on their ṇic branch, where the firing is the sanādi entry before ṇic; the other seven are the tanādi vikalpa), and so is the 7.3.86 of the one `7.3.86+8.2.75` key (√kit's `acikeH`), of every `7.3.86+7.1.35`/`7.3.86+7.1.35+8.4.56` key (the same curādi roots' loṭ tātaṅ cells — the only keys where 7.3.86 precedes 7.1.35, because the sanādi stage runs first), and of every `2570+…7.3.86…`, `2571+…7.3.86…`, `10.0498+…7.3.86…` and `10.0499+…7.3.86…` key (a ṇic-less root's guṇa).
#[test]
fn every_alternate_names_the_vikalpa_rules_that_produced_it() {
""",
"""/// as the mandatory guṇa-stage laghūpadha guṇa, as the tanādi vikalpa entry and,
/// since slice 10a, as the sanādi entry before ṇic, so a key
/// naming 7.3.86 does not by itself mean the rule was optional. 144 of the 151 `7.3.86+8.4.56` keys are the mandatory firing (ten juhotyādi: 3e's laṅ eka cells and 3f's √kit and √dhiṣ ones; 134 curādi: slice 10a's √cur laṅ and vidhiliṅ prathama eka, and slice 10g's √div and √śṛdh, slice 10h's sixteen laghu-ik ādhṛṣīya rows, slice 10i's eleven laghu-ik rows on their ṇic branch and slice 10k's thirty-seven laghu-ik rows, where the firing is the sanādi entry before ṇic; the other seven are the tanādi vikalpa), and so is the 7.3.86 of the one `7.3.86+8.2.75` key (√kit's `acikeH`), of every `7.3.86+7.1.35`/`7.3.86+7.1.35+8.4.56` key (the same curādi roots' loṭ tātaṅ cells — the only keys where 7.3.86 precedes 7.1.35, because the sanādi stage runs first), and of every `2570+…7.3.86…`, `2571+…7.3.86…`, `10.0498+…7.3.86…` and `10.0499+…7.3.86…` key (a ṇic-less root's guṇa).
#[test]
fn every_alternate_names_the_vikalpa_rules_that_produced_it() {
"""),
("""/// cell), and its loṭ parasmaipada madhyama eka ties the six-form record
/// with the same k = 3 against the 2³ bound of eight:
/// 26424 cells total (2936 root×lakāra blocks × 9), of which 18648 hold exactly one form, 6432 hold two, 601 hold three (√hrī's loṭ prathama and madhyama
/// eka, new in slice 3b, √dā's and √dhā's, new in slice 3c, and √gā's, new in
/// slice 3c2, and the six ṛ-roots', new in slice 3d, and √ṛ's, new in slice 3d2, and √ṇij's, √vij's and √viṣ's, new in slice 3e, and √kit's, √tur's, √dhiṣ's and √dhan's, new in slice 3f, and √bhas's, new in slice 3f2, and √jan's, new in slice 3f3, and the four curādi roots', new in slice 10a, and the six jñapādi roots', new in slice 10d, and the eighty-three ubhayapadī adanta roots', new in slice 10e, each by
/// 7.1.35/8.4.56, plus √bhas's laṅ madhyama eka, by 8.2.74/8.4.56; slice 10f's
/// optional-ṇic rows add 114 two-form and 46 three-form cells, and slice 10g's
""",
"""/// cell), and its loṭ parasmaipada madhyama eka ties the six-form record
/// with the same k = 3 against the 2³ bound of eight:
/// 37584 cells total (4176 root×lakāra blocks × 9), of which 29188 hold exactly one form, 6742 hold two, 911 hold three (√hrī's loṭ prathama and madhyama
/// eka, new in slice 3b, √dā's and √dhā's, new in slice 3c, and √gā's, new in
/// slice 3c2, and the six ṛ-roots', new in slice 3d, and √ṛ's, new in slice 3d2, and √ṇij's, √vij's and √viṣ's, new in slice 3e, and √kit's, √tur's, √dhiṣ's and √dhan's, new in slice 3f, and √bhas's, new in slice 3f2, and √jan's, new in slice 3f3, and the four curādi roots', new in slice 10a, and the six jñapādi roots', new in slice 10d, and the eighty-three ubhayapadī adanta roots', new in slice 10e, and the 155 plain obligatory-ṇic rows', new in slice 10k, each by
/// 7.1.35/8.4.56, plus √bhas's laṅ madhyama eka, by 8.2.74/8.4.56; slice 10f's
/// optional-ṇic rows add 114 two-form and 46 three-form cells, and slice 10g's
"""),
("""/// ṇic branch) × the same triple. No cell holds eight.
/// `ALTERNATES`
/// itself has 10646 rows, keyed 646 `8.4.56`, 638 `7.1.35`, 638 `7.1.35+8.4.56`,
/// 2 `3.4.111`, 72 `6.4.107`, 145 `8.4.65`, 8 `8.2.75`, 2 `8.2.74` (√hiṃs's ahinaH and, new in slice 3f2, √bhas's abaBaH), 16
/// `7.1.35+8.4.65`, 16 `7.1.35+8.4.65+8.4.56`, 270 `7.3.86` (tanādi 8a's
/// ik-upadhā fork), 8 `7.1.35+7.3.86`, 8 `7.1.35+7.3.86+8.4.56`, 8
/// `7.3.86+6.4.107`, 77 `7.3.86+8.4.56` (seventy of them name the MANDATORY
/// 7.3.86, through the id it shares with the tanādi vikalpa arm: ten juhotyādi, slice 3e's
/// laṅ prathama and madhyama eka cells and slice 3f's √kit and √dhiṣ ones, whose root guṇa 7.3.86 credits, and sixty curādi, slice 10a's √cur laṅ and vidhiliṅ prathama eka and slice 10g's √div and √śṛdh, slice 10h's sixteen laghu-ik ādhṛṣīya rows' and slice 10i's eleven laghu-ik rows' ṇic-branch ones, whose guṇa before ṇic the sanādi 7.3.86 credits; the other seven are the tanādi vikalpa), 1 `7.3.86+8.2.75` (√kit's acikeH, the same mandatory 7.3.86), 23 `6.4.115`, 2 `7.1.35+6.4.115`,
/// 2 `7.1.35+6.4.115+8.4.56`, and 1 `6.4.115+8.4.56`, 14 `6.4.116`, 1 `6.4.117`, 2 `7.1.35+6.4.116` and 2
/// `7.1.35+6.4.116+8.4.56`, 9 `6.4.43` and 1 `6.4.43+8.4.56` (slice 3f3's √jan), 60 `7.3.86+7.1.35` and 60
/// `7.3.86+7.1.35+8.4.56` (slice 10a's √cur, its sanādi 7.3.86 ahead of 7.1.35, and slice 10g's √śṛdh and
/// √div, slice 10h's sixteen laghu-ik ādhṛṣīya rows and slice 10i's eleven laghu-ik rows on their ṇic
/// branch), and slice 10f's twenty-one
/// keys on its five Kaumudī vikalpa ids, at their counts as of slice 10g: 36 `2573.1`, 72 `2573.2`, 72 `2573.3`, 114 apiece
/// `2564+8.4.56` / `2564+7.1.35` / `2564+7.1.35+8.4.56` (8 from 10f's four idit rows, 106 from 10g's
""",
"""/// ṇic branch) × the same triple. No cell holds eight.
/// `ALTERNATES`
/// itself has 11576 rows, keyed 882 `8.4.56`, 874 `7.1.35`, 874 `7.1.35+8.4.56`,
/// 2 `3.4.111`, 72 `6.4.107`, 145 `8.4.65`, 8 `8.2.75`, 2 `8.2.74` (√hiṃs's ahinaH and, new in slice 3f2, √bhas's abaBaH), 16
/// `7.1.35+8.4.65`, 16 `7.1.35+8.4.65+8.4.56`, 270 `7.3.86` (tanādi 8a's
/// ik-upadhā fork), 8 `7.1.35+7.3.86`, 8 `7.1.35+7.3.86+8.4.56`, 8
/// `7.3.86+6.4.107`, 151 `7.3.86+8.4.56` (144 of them name the MANDATORY
/// 7.3.86, through the id it shares with the tanādi vikalpa arm: ten juhotyādi, slice 3e's
/// laṅ prathama and madhyama eka cells and slice 3f's √kit and √dhiṣ ones, whose root guṇa 7.3.86 credits, and 134 curādi, slice 10a's √cur laṅ and vidhiliṅ prathama eka and slice 10g's √div and √śṛdh, slice 10h's sixteen laghu-ik ādhṛṣīya rows', slice 10i's eleven laghu-ik rows' ṇic-branch ones and slice 10k's thirty-seven laghu-ik rows', whose guṇa before ṇic the sanādi 7.3.86 credits; the other seven are the tanādi vikalpa), 1 `7.3.86+8.2.75` (√kit's acikeH, the same mandatory 7.3.86), 23 `6.4.115`, 2 `7.1.35+6.4.115`,
/// 2 `7.1.35+6.4.115+8.4.56`, and 1 `6.4.115+8.4.56`, 14 `6.4.116`, 1 `6.4.117`, 2 `7.1.35+6.4.116` and 2
/// `7.1.35+6.4.116+8.4.56`, 9 `6.4.43` and 1 `6.4.43+8.4.56` (slice 3f3's √jan), 134 `7.3.86+7.1.35` and 134
/// `7.3.86+7.1.35+8.4.56` (slice 10a's √cur, its sanādi 7.3.86 ahead of 7.1.35, and slice 10g's √śṛdh and
/// √div, slice 10h's sixteen laghu-ik ādhṛṣīya rows and slice 10i's eleven laghu-ik rows on their ṇic
/// branch, and slice 10k's thirty-seven laghu-ik rows), and slice 10f's twenty-one
/// keys on its five Kaumudī vikalpa ids, at their counts as of slice 10g: 36 `2573.1`, 72 `2573.2`, 72 `2573.3`, 114 apiece
/// `2564+8.4.56` / `2564+7.1.35` / `2564+7.1.35+8.4.56` (8 from 10f's four idit rows, 106 from 10g's
"""),
("""/// 6.1.54 branch) — adds 72 to `2570` and 2 apiece to `2570+8.4.56`,
/// `2570+7.1.35` and `2570+7.1.35+8.4.56` (√ci's ṇic-less branch), and folds
/// 10 rows apiece into `8.4.56`, `7.1.35` and `7.1.35+8.4.56` — √kṛ (slice 8b) adds six more
/// rows, all folded into the pre-existing `8.4.56`/`7.1.35`/`7.1.35+8.4.56`
/// keys above, two apiece: `8.4.56` gains `akarot` (laṅ parasmaipada
""",
"""/// 6.1.54 branch) — adds 72 to `2570` and 2 apiece to `2570+8.4.56`,
/// `2570+7.1.35` and `2570+7.1.35+8.4.56` (√ci's ṇic-less branch), and folds
/// 10 rows apiece into `8.4.56`, `7.1.35` and `7.1.35+8.4.56`. Slice 10k opens
/// no key: it folds 236 rows apiece into `8.4.56`, `7.1.35` and
/// `7.1.35+8.4.56` and 74 apiece into `7.3.86+8.4.56`, `7.3.86+7.1.35` and
/// `7.3.86+7.1.35+8.4.56` (its thirty-seven laghu-ik rows) — √kṛ (slice 8b) adds six more
/// rows, all folded into the pre-existing `8.4.56`/`7.1.35`/`7.1.35+8.4.56`
/// keys above, two apiece: `8.4.56` gains `akarot` (laṅ parasmaipada
"""),
("""/// cells), and curādi 10j's re-ran it at the same commit over all 26424
/// cells / 37070 forms / 430 roots with zero differences, its `entry`
/// negative control verified failing (36 √bhū cells). √tṛh joins none of the fork
/// records: its deepest cells hold three forms, because 8.3.13 Qo Qe lopaH
/// obligatorily elides the ḍh that 8.4.65 forks on for every other
""",
"""/// cells), and curādi 10j's re-ran it at the same commit over all 26424
/// cells / 37070 forms / 430 roots with zero differences, its `entry`
/// negative control verified failing (36 √bhū cells), and curādi 10k's
/// re-ran it at the same commit over all 37584 cells / 49160 forms / 585
/// roots with zero differences, its `entry` negative control verified
/// failing (36 √bhū cells). √tṛh joins none of the fork
/// records: its deepest cells hold three forms, because 8.3.13 Qo Qe lopaH
/// obligatorily elides the ḍh that 8.4.65 forks on for every other
"""),
],
'crates/panini/tests/trace/curadi.rs': [
("""//! sūtra is Kaumudī 2567, then 1.3.12. √dhūp's and √vich's ṇic-less branch has
//! 3.1.28's āya, which 3.4.114 and 3.1.32 treat as they treat ṇic, and
//! √vich's every branch the sanādi 6.1.73's tuk before 3.1.32.

use crate::helpers::{at, cell_trace, credited};
""",
"""//! sūtra is Kaumudī 2567, then 1.3.12. √dhūp's and √vich's ṇic-less branch has
//! 3.1.28's āya, which 3.4.114 and 3.1.32 treat as they treat ṇic, and
//! √vich's every branch the sanādi 6.1.73's tuk before 3.1.32, as √pich's
//! (slice 10k) ṇic branch does.

use crate::helpers::{at, cell_trace, credited};
"""),
],
'docs/ARCHITECTURE.md': [
("""in slice 8b) — and juhotyādi (3), **complete** at all 26 of its rows (√hu,
√ki; slice 3a; √bhī, √hrī; slice 3b; √dā, √dhā, √mā, √hā; slice 3c; √hā
parasmaipada, √gā; slice 3c2; √pṝ, √pṛ, √bhṛ, √ghṛ, √hṛ, √sṛ; slice 3d; √ṛ; slice 3d2; √ṇij, √vij, √viṣ; slice 3e; √kit, √tur, √dhiṣ, √dhan; slice 3f; √bhas; slice 3f2; √jan; slice 3f3) — and curādi (10), **open** at 327 of its
509 rows (√cur, √laḍ, √bhakṣ, √bhūṣ; slice 10a; √cit, √vṛṣ, √mad, √kusm; slice 10b; thirty-three more ākusmīya roots, slice 10c; √jñap, √yam, √cah, √cap, √rah, √bal, slice 10d; ninety-two adanta roots, slice 10e; the ten optional-ṇic rows, slice 10f; fifty-nine more, slice 10g; fifty ādhṛṣīya rows, slice 10h; the fifty-nine āsvadīya rows, √pṝ and √ghuṣ, slice 10i; √smiṅ, √ci, √ghṛ, √gṛ, √yu, √jñā, √cyu and √bhū, slice 10j). gaṇa
is carried as a tag on the aṅga term (`Tag::Divadi` / `Tag::Tudadi` / `Tag::Adadi` /
`Tag::Kryadi` / `Tag::Svadi` / `Tag::Rudhadi` / `Tag::Tanadi` /
""",
"""in slice 8b) — and juhotyādi (3), **complete** at all 26 of its rows (√hu,
√ki; slice 3a; √bhī, √hrī; slice 3b; √dā, √dhā, √mā, √hā; slice 3c; √hā
parasmaipada, √gā; slice 3c2; √pṝ, √pṛ, √bhṛ, √ghṛ, √hṛ, √sṛ; slice 3d; √ṛ; slice 3d2; √ṇij, √vij, √viṣ; slice 3e; √kit, √tur, √dhiṣ, √dhan; slice 3f; √bhas; slice 3f2; √jan; slice 3f3) — and curādi (10), **open** at 482 of its
509 rows (√cur, √laḍ, √bhakṣ, √bhūṣ; slice 10a; √cit, √vṛṣ, √mad, √kusm; slice 10b; thirty-three more ākusmīya roots, slice 10c; √jñap, √yam, √cah, √cap, √rah, √bal, slice 10d; ninety-two adanta roots, slice 10e; the ten optional-ṇic rows, slice 10f; fifty-nine more, slice 10g; fifty ādhṛṣīya rows, slice 10h; the fifty-nine āsvadīya rows, √pṝ and √ghuṣ, slice 10i; √smiṅ, √ci, √ghṛ, √gṛ, √yu, √jñā, √cyu and √bhū, slice 10j; the 155 plain obligatory-ṇic rows, slice 10k). gaṇa
is carried as a tag on the aṅga term (`Tag::Divadi` / `Tag::Tudadi` / `Tag::Adadi` /
`Tag::Kryadi` / `Tag::Svadi` / `Tag::Rudhadi` / `Tag::Tanadi` /
"""),
("""
7.1.35 optionally replaces the loṭ endings `tu`/`hi` with tātaṅ (then
8.2.39 obligatorily voices its final `t` to `d`), forking 712 cells (loṭ
prathama and madhyama eka across the 356 roots with a parasmaipada column —
`tu`/`hi` are parasmaipada endings, so the curated set's 74 ātmanepada-only
roots never reach this guard, and the 304 roots that admit both
padas (twenty-five ubhayapadī by 1.3.72 — √rudh, √nī, √tud, √bhid, √kṣud,
√yuj, √tṛd, √ric, √vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛṇ, √tṛ, √ghṛṇ,
""",
"""
7.1.35 optionally replaces the loṭ endings `tu`/`hi` with tātaṅ (then
8.2.39 obligatorily voices its final `t` to `d`), forking 1022 cells (loṭ
prathama and madhyama eka across the 511 roots with a parasmaipada column —
`tu`/`hi` are parasmaipada endings, so the curated set's 74 ātmanepada-only
roots never reach this guard, and the 459 roots that admit both
padas (twenty-five ubhayapadī by 1.3.72 — √rudh, √nī, √tud, √bhid, √kṣud,
√yuj, √tṛd, √ric, √vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛṇ, √tṛ, √ghṛṇ,
"""),
("""`pata`, slice 10g's fifty-nine optional-ṇic rows, slice 10h's fifty
ādhṛṣīya rows (six of them by 1.3.72 too, without ṇic), slice 10i's
sixty-one rows and slice 10j's √ghṛ, √jñā, √cyu, √bhū and √ci (by 1.3.72 too,
without ṇic) by 1.3.74, and slice 10f's six optional-ṇic ākusmīya roots and
`garva`, by 1.3.78 on their ṇic-less branch) reach it in their
parasmaipada cells only, joined by juhotyādi's √hu, √ki, √bhī, √hrī, √hā
(`03.0009`), √gā, √pṝ, √pṛ, √ghṛ (`Gf`), √hṛ, √sṛ, √ṛ, √kit, √tur, √dhiṣ, √dhan, √bhas and √jan (all parasmaipada-only), and
the parasmaipada columns of √bhṛ, √ṇij, √vij and √viṣ; 356 + 74 = the 430 curated roots) — `Bavatu ~
BavatAd`, `Bava ~ BavatAd`. 8.4.56 optionally devoices a pada-final jaś
(produced by the now-obligatory 8.2.39) back to its car at the end of an
utterance, forking 646 cells outright: laṅ and vidhiliṅ prathama eka across
those same 356 parasmaipada columns (623 of them — 8.2.39's `d` is a
parasmaipada-ending artifact, ātmanepada's laṅ/vidhiliṅ prathama eka endings
are vowel-final and never reach a jhal, and the seven juhotyādi ṛ-roots
(3d's six and 3d2's √ṛ) contribute only their vidhiliṅ cell, since their laṅ
prathama eka ends in the aṅga's `r`, turned to visarga, not in a jaś: `abiBaH`,
`EyaH`; 3f's four contribute only their vidhiliṅ cell too: √tur's laṅ ends in visarga, √dhan's in `n`, and √kit's and √dhiṣ's laṅ forks key on 7.3.86 as well, see below; 3f2's √bhas contributes both, its laṅ `abaBad` coming from 8.2.73; 3f3's √jan contributes only its vidhiliṅ cell, its laṅ ending in `n`, and its 6.4.43 branch's `jajAyAd ~ jajAyAt` keys on 6.4.43 as well; 10a's four curādi roots contribute both cells, except √cur, whose two key on its mandatory sanādi 7.3.86 as well, see below; 10d's six jñapādi roots and 10e's eighty-three adanta roots contribute both cells, and so do 10f's `mUtra`, `katra` and `pata`, 10g's rows other than √śṛdh and √div, 10h's other than its sixteen laghu-ik rows and 10i's other than its eleven (whose ṇic branch keys on the sanādi 7.3.86 as well) on their ṇic branch, and 10j's √ghṛ, √jñā, √cyu and √bhū, and √ci on its declined ṇic branch; √ci's 6.1.54 branch keys on 6.1.54 as well (`6.1.54+8.4.56`), and 10f's to 10j's ṇic-less forks key on their optional-ṇic id — `2564+8.4.56`, `10.0498+8.4.56`, `10.0499+8.4.56` and their siblings — as does √dhū's and √prī's nuk branch on 7.3.37.2, and sit outside this count),
plus twenty-two rudhādi laṅ *madhyama* eka cells (√kṛt, √hiṃs, √bhañj, √piṣ,
√rudh, √bhid, √kṣud, √yuj, √tṛd, √ric, √vic, √śiṣ, √und, √añj, √tañc, √vij,
""",
"""`pata`, slice 10g's fifty-nine optional-ṇic rows, slice 10h's fifty
ādhṛṣīya rows (six of them by 1.3.72 too, without ṇic), slice 10i's
sixty-one rows, slice 10j's √ghṛ, √jñā, √cyu, √bhū and √ci (by 1.3.72 too,
without ṇic) and slice 10k's 155 rows by 1.3.74, and slice 10f's six optional-ṇic ākusmīya roots and
`garva`, by 1.3.78 on their ṇic-less branch) reach it in their
parasmaipada cells only, joined by juhotyādi's √hu, √ki, √bhī, √hrī, √hā
(`03.0009`), √gā, √pṝ, √pṛ, √ghṛ (`Gf`), √hṛ, √sṛ, √ṛ, √kit, √tur, √dhiṣ, √dhan, √bhas and √jan (all parasmaipada-only), and
the parasmaipada columns of √bhṛ, √ṇij, √vij and √viṣ; 511 + 74 = the 585 curated roots) — `Bavatu ~
BavatAd`, `Bava ~ BavatAd`. 8.4.56 optionally devoices a pada-final jaś
(produced by the now-obligatory 8.2.39) back to its car at the end of an
utterance, forking 882 cells outright: laṅ and vidhiliṅ prathama eka across
those same 511 parasmaipada columns (859 of them — 8.2.39's `d` is a
parasmaipada-ending artifact, ātmanepada's laṅ/vidhiliṅ prathama eka endings
are vowel-final and never reach a jhal, and the seven juhotyādi ṛ-roots
(3d's six and 3d2's √ṛ) contribute only their vidhiliṅ cell, since their laṅ
prathama eka ends in the aṅga's `r`, turned to visarga, not in a jaś: `abiBaH`,
`EyaH`; 3f's four contribute only their vidhiliṅ cell too: √tur's laṅ ends in visarga, √dhan's in `n`, and √kit's and √dhiṣ's laṅ forks key on 7.3.86 as well, see below; 3f2's √bhas contributes both, its laṅ `abaBad` coming from 8.2.73; 3f3's √jan contributes only its vidhiliṅ cell, its laṅ ending in `n`, and its 6.4.43 branch's `jajAyAd ~ jajAyAt` keys on 6.4.43 as well; 10a's four curādi roots contribute both cells, except √cur, whose two key on its mandatory sanādi 7.3.86 as well, see below; 10d's six jñapādi roots and 10e's eighty-three adanta roots contribute both cells, and so do 10f's `mUtra`, `katra` and `pata`, 10g's rows other than √śṛdh and √div, 10h's other than its sixteen laghu-ik rows and 10i's other than its eleven (whose ṇic branch keys on the sanādi 7.3.86 as well) on their ṇic branch, and 10j's √ghṛ, √jñā, √cyu and √bhū, and √ci on its declined ṇic branch, and 10k's rows other than its thirty-seven laghu-ik rows (whose two key on the sanādi 7.3.86 as well); √ci's 6.1.54 branch keys on 6.1.54 as well (`6.1.54+8.4.56`), and 10f's to 10j's ṇic-less forks key on their optional-ṇic id — `2564+8.4.56`, `10.0498+8.4.56`, `10.0499+8.4.56` and their siblings — as does √dhū's and √prī's nuk branch on 7.3.37.2, and sit outside this count),
plus twenty-two rudhādi laṅ *madhyama* eka cells (√kṛt, √hiṃs, √bhañj, √piṣ,
√rudh, √bhid, √kṣud, √yuj, √tṛd, √ric, √vic, √śiṣ, √und, √añj, √tañc, √vij,
"""),
("""stack 7.3.86, which credits the root's guṇa, so they sit in the
`7.3.86+8.4.56` bucket beside tanādi's and not in this count. 8.4.56 goes on
forking a further 712 (the same
loṭ cells 7.1.35 just forked) by devoicing the tātaṅ branch's `BavatAd` to
`BavatAt`, which is
""",
"""stack 7.3.86, which credits the root's guṇa, so they sit in the
`7.3.86+8.4.56` bucket beside tanādi's and not in this count. 8.4.56 goes on
forking a further 1022 (the same
loṭ cells 7.1.35 just forked) by devoicing the tātaṅ branch's `BavatAd` to
`BavatAt`, which is
"""),
],
'docs/superpowers/specs/2026-10-02-curadi-gana-10c-design.md': [
("""support. When a partner is curated later, its ātmanepada cells will repeat
these forms. That is expected, the same as any two rows sharing a surface.
Each homograph row's comment names its partner number.

**√syam and √śam are in, without 10.0494.** vidyut credits 10.0494 on
""",
"""support. When a partner is curated later, its ātmanepada cells will repeat
these forms. That is expected, the same as any two rows sharing a surface.
Each homograph row's comment names its partner number. (Slice 10k curated
the five ubhayapadī partners `10.0006`, `10.0034`, `10.0041`, `10.0189` and
`10.0438`: see `2026-10-05-curadi-gana-10k-design.md`.)

**√syam and √śam are in, without 10.0494.** vidyut credits 10.0494 on
"""),
],
'docs/superpowers/specs/2026-10-04-curadi-gana-10j-design.md': [
("""  (`10.0397`, `10.0414`, `10.0438`, `10.0457`, `10.0462`, `10.0470`,
  `10.0491`). Prototype-audit them first: no spec has examined them, and
  each batch may hide rules.
- Upasargas, and with them `10.0368 za\\da~` (7.3.78) and 6.1.76 *padāntād
  vā*.
""",
"""  (`10.0397`, `10.0414`, `10.0438`, `10.0457`, `10.0462`, `10.0470`,
  `10.0491`). Prototype-audit them first: no spec has examined them, and
  each batch may hide rules. (The prototype found 164; slice 10k took the
  155 that need nothing new, and slice 10l takes the nine that need rules:
  see `2026-10-05-curadi-gana-10k-design.md`.)
- Upasargas, and with them `10.0368 za\\da~` (7.3.78) and 6.1.76 *padāntād
  vā*.
"""),
],
'tools/audit/README.md': [
("""bare root code — not a surface form.

**It asserts the corpus totals** (430 roots, 26424 cells, 37070 forms) rather than
reporting whatever it enumerated. Those totals are corroborated by
`derivation_set_shape_matches_the_audited_numbers` in
""",
"""bare root code — not a surface form.

**It asserts the corpus totals** (585 roots, 37584 cells, 49160 forms) rather than
reporting whatever it enumerated. Those totals are corroborated by
`derivation_set_shape_matches_the_audited_numbers` in
"""),
("""
## Last recorded result

2026-10-04, curādi 10j slice, vidyut
""",
"""
## Last recorded result

AUDITDATE, curādi 10k slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 37584
cells / 49160 forms / 585 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole curādi 10k slice: the 155 plain obligatory-ṇic
rows that need nothing new, ubhayapadī by 1.3.74 (thirty-seven by 7.3.86
before ṇic, forty by 7.2.116, seventy-eight unchanged), with no engine change.
Ten of them reach 8.3.24 on their own `n`, √pich the sanādi 6.1.73, and
`10.0051 zAntva~` and `10.0052 sAntva~` are one root once 6.1.64 applies.
Blocked branches stay at 6516: no row's ṇic is optional. The throwaway
prototype that scoped the slice curated all 164 remaining rows of the range
and found 594 differing cells, every one on the nine rows slice 10l takes. A
main-vs-branch dump of every prior cell's branches, blocked ones included,
was byte-identical, all 43586 of them (37070 live).

Totals: 585 = 430 + 155; 37584 = 26424 + 11160 (1240 root×pada×lakāra blocks
× 9); 49160 = 37070 + 11160 + 930 new `ALTERNATES` rows (10646 → 11576),
measured via the harness's corpus block, not assumed.

2026-10-04, curādi 10j slice, vidyut
"""),
],
}
import re
if len(sys.argv) != 2 or not re.fullmatch(r'\d{4}-\d{2}-\d{2}', sys.argv[1]):
    sys.exit("usage: docsweep_10k.py YYYY-MM-DD (the audit's date)")
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
python3 /tmp/vidyut-full/slice10k/docsweep_10k.py <the date Step 2 printed>      # applied 31 edits
mise run fmt
```

- [ ] **Step 4: Sweep for anything left stale**

```bash
grep -rn -i -E "\b(430|26424|37070|10646|2936|304 roots|356|712|646|623|1063|18648|6432|327 of)\b|three hundred twenty-seven|four hundred thirty|two hundred sixty-four" \
  README.md AGENTS.md docs/ARCHITECTURE.md tools/audit crates --include=*.rs --include=*.md | grep -v "paradigm/data/" \
  | grep -v 'dhatupatha: "\|^\S*:\s*[0-9]*:\s*("10' \
  | grep -v "10j entry\|10j slice\|10j's re-ran\|430 = 422\|26424 = 25956\|37070 = 36416\|10460 → 10646\|= 26424 + 11160\|= 37070 +\|10646 → 11576\|585 = 430\|1063 to 1603\|1047 to 1063"
grep -rn -i -E "uncurated|not yet curated|not curated|later slice" crates README.md AGENTS.md docs/ARCHITECTURE.md --include=*.rs --include=*.md | grep -v "paradigm/data/" \
  | grep -E "10\.00[0-9]{2}|10\.01[0-9]{2}|10\.02[0-7][0-9]|10\.0(397|414|438|457|462|470|491)|plain|hal-anta"
```

Expected from the first grep, exactly these, all history or Task 4's:
- `tools/audit/README.md`: 10j's record (`26424` / `37070 forms / 430 roots`) and the 10k record's `(37070 live)`;
- `AGENTS.md`: the floor paragraph's `measured at 26424 cells`, the cap formula's `max(430, …` and `430 from 10e`, and the audit chain's 10j clause (`430 roots), and that by curādi 10k's`);
- `crates/panini/tests/paradigm/main.rs`: the audit chain's 10j clause (`37070 forms / 430 roots`) and 10j's paragraph (`OPEN at 327 of its 509 rows`).

The second prints nothing. Anything else is a miss: fix it, and say so in the commit message.

- [ ] **Step 5: Run the full suite and commit**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
git branch --show-current      # curadi-10k
git add -A
git commit -m "docs: 10k's counts, the audit record, and the sweep

Audit: zero differences over 585 roots / 37584 cells / 49160 forms against
vidyut 8da2f90, the entry control failing on 36 cells; every prior branch's
trace byte-identical between main and the branch (43586, 37070 live)."
```

Expected: the suite passes as in Task 2 Step 6.

---

## Task 4: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and the current-record paragraph); `mise.toml` if the cap moves

Follow AGENTS.md's cargo-mutants protocol. Hazards from this repo's record:
- **Measure, never scale.** The cap must exceed a full uncaught suite run at the parallelism used, under campaign load, and twice the slowest caught phase (the `skip_nic` blow-up, AGENTS.md's floor paragraph).
- **Every invocation rotates `mutants.out`**, so always pass `-o`, and copy `outcomes.json` durably before any other invocation.
- **The mise shim fails.** Use the real binary: `CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`.
- **`pgrep -f` matches its own shell.** Wait on `pgrep -x cargo-mutants`.
- **Background shells die at about 60 minutes.** Launch detached with `setsid nohup`, as below, and resume with `--iterate` if it dies.

What to expect:
- **The mutant list is unchanged, spans included.** `panini-prakriya` lists 863 mutants, identical line for line to main's (the prototype diffed the two `--list` outputs); `panini-analyze` 12 and `panini-data` 12. `--in-diff` over this slice's diff lists none in `panini-prakriya` or `panini-data` ("No mutants to filter"). So the non-caught set should equal 10j's exactly, with and without span lines.
- **The documented non-caught entries stay where 10j recorded them:** the equivalents at `adesha.rs:649:30` and `tripadi.rs:1305:38`, the permanent hang at `tripadi.rs:1618:23`, and the `skip_nic` pair at `sanadi.rs:57:5` and `57:39`. Confirm by `--list`; never compute them.
- **The cap will probably rise.** `skip_nic -> true` forks every derivation 2^8 ways, so its test phase grows with the corpus: 10j's probe caught it in 3885.06s at 26424 cells (the `==` sibling in 2740.27s), and the corpus is 42% larger. Expect a probe reading near 5000–6000s and a cap near twice that. Only the probe sets it; do not scale 10j's number.
- **The campaign will be long.** 10j's took 5h32m at a 7780 cap; the permanent hang alone holds one `-j 4` slot for a full cap. Expect 7–9 hours.

- [ ] **Step 1: Measure the floor**

With nothing else of ours running, run this twice: `cat /proc/loadavg; time mise run test >/dev/null 2>&1; cat /proc/loadavg` (foreground, timeout 600000 ms). Record both wall clocks, user CPU and the load averages. Read 10j's floor from AGENTS.md's floor paragraph and keep the comparison chain.

- [ ] **Step 2: Locate and probe the uncaught equivalents and the `skip_nic` pair**

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
PROBE=/home/dev/mutants-records/curadi-10k-probe   # durable
mkdir -p "$PROBE"
mise exec -- "$CM" mutants --package panini-prakriya --list -o "$PROBE/list" 2>/dev/null | wc -l      # 863
mise exec -- "$CM" mutants --package panini-prakriya --list -o "$PROBE/list" 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /=|skip_nic -> bool with true|in skip_nic"
```

Expect `649` (adesha), `1305` (tripadi, inside 8.3.13's `apply`), `1618` (the ṇatva hang), and `sanadi.rs:57:5` (`skip_nic -> bool with true`) and `57:39` (`!=` → `==` in `skip_nic`). Write them as `<A>`, `<T1>`, `<T2>`, `<K>` and `<K2>`. The probe runs past the 60-minute shell limit, so launch it detached and wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`:

```bash
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 20000 -j 4 -o "$PROBE" \
  --re "adesha.rs:<A>:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" \
  --re "sanadi.rs:<K>:5: replace skip_nic -> bool with true" --re "sanadi.rs:<K2>:39: replace != with == in skip_nic" \
  > "$PROBE/probe.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$PROBE/started"; cat /proc/loadavg > "$PROBE/load.started"
```

The 20000s probe cap is a ceiling for measurement, not the campaign's cap. The regexes also match caught `mod.rs` `derive` mutants; that is expected. When it ends, copy `$PROBE/mutants.out/outcomes.json` to `$PROBE/probe-outcomes.durable.json` and record the end time and load. Both equivalents must be MISSED, not TIMEOUT; both `skip_nic` mutants must be CAUGHT. Read each test-phase duration from the outcomes. Set the provisional cap to max(7780, 6 × the longer equivalent, 2 × the longer `skip_nic` phase), rounded up to the next 10 s.

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/curadi-10k"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout <CAP> -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"; cat /proc/loadavg > "$OUT/load.started"
```

`<CAP>` is Step 2's provisional cap. Run nothing CPU-heavy meanwhile. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

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
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:<T2>:23: replace -= with /=`.

Then diff the non-caught set against 10j's on the full record, both with and without span lines:

```bash
python3 - "$OUT/outcomes.durable.json" /home/dev/mutants-records/curadi-10j/outcomes.durable.json <<'PY'
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

Expected: `no lines 25 25`, `new: []`, `gone: []`; `lines 52 52`, `new: []`, `gone: []`. Write down exactly what prints.

If not:
- Any **other timeout** is a suspect survivor that the larger suite pushed past the cap. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant that 10j caught means a test that caught it at 26424 cells no longer does. Stop and report.

**Step 4b: the data-crate mutants.** Confirm the slice's diff for `panini-data` holds none:

```bash
git diff 81b9834 -- crates/panini-data/src/lib.rs > "$OUT/data.diff"
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
- If that is 7780 or less, the cap stays 7780.
- Otherwise change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together (`grep -n "7780" AGENTS.md mise.toml`).

- [ ] **Step 6: Record it in AGENTS.md**

- **The floor paragraph.** Rewrite the paragraph that opens `**The floor behind the 7780s cap, measured at 26424 cells on Rust 1.99.0,`. Use Step 1's and Step 2's numbers at 37584 cells, the load averages, and the cap Step 5 chose. Keep the comparison chain to earlier floors, with 10j's joining it: 7m6.732s / 4m29.176s at 26424 cells under load 65–90 (and 2m13.130s on a quieter host), isolated probe 315.46s / 309.36s, campaign-load 298.02s / 250.01s, `skip_nic` `true` 3885.06s in the probe and 3232.46s in the campaign, `!=` → `==` 2740.27s and 2287.81s.
- **The current record.** Replace the `**Current record (curādi 10j, …).**` paragraph with `**Current record (curādi 10k, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**;
  - the non-caught set diffed against 10j's on the full record, both ways (Step 4's script output);
  - that the slice adds no mutant (`--list` identical to main's, spans included) and Step 4b's empty data-crate list;
  - the `skip_nic` pair's probe and campaign phases;
  - the campaign-load phases and margins;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`, and the probe's at `/home/dev/mutants-records/curadi-10k-probe/probe-outcomes.durable.json`.

  End it with a pointer to the record it replaces. Run `git rev-parse --short HEAD` before committing, and write ``The curādi 10j record it replaces: `git show <that hash>:AGENTS.md`.``

- [ ] **Step 7: Commit**

```bash
git branch --show-current      # curadi-10k
git add AGENTS.md mise.toml
git commit -m "chore: 10k mutation gate — floor and uncaught run re-measured at 37584 cells

The mutant list is unchanged; missed.txt holds only the two documented
equivalents and timeout.txt only the permanent ṇatva-scan entry, and the
non-caught set equals 10j's, spans included."
```

Adjust the message to what Step 4 and Step 5 actually found (the cap, any difference).

---

## Task 5: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # back at /workspace/crates
git branch --show-current      # curadi-10k
git log --oneline main..HEAD   # the spec commit, the plan and the three task commits
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin curadi-10k
gh pr create --title "curādi 10k — the 155 plain obligatory-ṇic rows" --body "$(cat <<'BODY'
Slice 10k curates the 155 plain obligatory-ṇic curādi rows that need
nothing new: ubhayapadī by 1.3.74, thirty-seven taking 7.3.86 before ṇic,
forty 7.2.116 and seventy-eight neither. No engine change.

- Existing rules reach new rows: 8.3.24 / 8.4.58 on a root's own `n`
  (*sambayati*, *puṃsayati*) and the sanādi 6.1.73's tuk on √pich
  (*picchayati*).
- `10.0051 zAntva~` and `10.0052 sAntva~` are one root once 6.1.64 applies;
  both are curated, as a pinned sibling pair in the data test.
- Twenty-one rows are homographs of earlier rows and four pairs of each
  other; the `check()` tests pin every one, in analysis order.

The golden suite goes from 26424 to 37584 cells; curādi is open at 482 of
509. The audit shows zero divergence against `8da2f90b`, a main-vs-branch
dump of every prior branch (blocked ones included) is byte-identical, and
the mutation gate's non-caught set equals 10j's. The nine rows that need
new rules (8.2.78, 7.1.100, 8.2.18, 6.1.75, 8.4.41, 8.2.30, and udit
`u~Drasa~`) are slice 10l's.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`:
   - run `git worktree remove .worktrees/curadi-10k`;
   - run `git worktree remove --force .worktrees/curadi-10k-proto` and `git branch -D proto-10k proto-10k-assert` (the throwaway);
   - run `git worktree remove --force .worktrees/curadi-10k-replay` if the replay worktree is still there (detached; no branch);
   - run `git worktree remove --force /tmp/claude-1000/-workspace/3b8703e6-f103-44f5-a253-6b0f93057c8e/scratchpad/proto10k` and `git branch -D proto-10k-throwaway` (the brainstorm's scoping prototype; `git worktree prune` if the path is gone);
   - delete the local and remote `curadi-10k` branch;
   - run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**
- Scope (155 rows, `Nic`, four lakāras, both padas; 11160 cells, 12090 forms; blocked 6516) → Task 2 Steps 3–6; Global Constraints' counts.
- Decisions: homographs with no structural support, each partner named in its row comment → `rows_10k.py`; the 21 cross-row and 4 in-slice homographs as `check()` witnesses → `curadi_analyses_its_plain_nic_forms` and the three edited tests. The `sAntv` exception and its upadeśa pin, seen failing first → Task 2 Steps 1–2.
- Changes, data: 155 rows in a 10k block; the row-list test renamed with its paragraph; 430 → 585; `pada` doc 264 → 419; `CONVERGENT_UPADESHA_PAIR` → Task 2 Step 1, Step 3.
- Goldens: 1240 + 930, generated, hashed, reviewed with histogram → Task 2 Steps 4–5; Global Constraints.
- Tests: the census; the coverage test; the four rosters with their branch counts; the `check()` tests; the prior-trace dump → Task 2 Steps 1–6, Task 3 Step 2. The spec missed one moved test, `pada_ambiguous_surfaces_are_exactly_these` (1063 → 1603); Task 2 Step 5 pins it from measurement.
- Audit (negative control first, then 585 / 37584 / 49160) → Task 3 Steps 1–2.
- Mutation gate (re-measure floor and `skip_nic`, cap by AGENTS.md's rule, `-o`, durable copy, non-caught set named) → Task 4.
- Doc sweep (counts, curādi 482, ARCHITECTURE census, audit record, the 6.1.73 comments, the 8.3.24 and 8.4.40 roster comments, the 10j and 10c spec pointers, greps for numerals, spelled-out counts and "uncurated" phrasings) → Task 3 Steps 3–4; the roster comments are in Task 2's script with their assertions.
- Later slices → no task (10l is its own slice).

**Placeholder scan.** `<A>`, `<T1>`, `<T2>`, `<K>`, `<K2>`, `<CAP>`, `<DATE>` and the audit date are values the executor measures, each with the command that produces it and its expected value where one is known. No step says "update the tests" without the edit.

**Type consistency.** `CONVERGENT_UPADESHA_PAIR: [&str; 2]` (Task 2) is read only in `panini-data`'s tests; `curadi_analyses_its_plain_nic_forms` uses `Reading<'a> = (&'a str, &'a [&'a str], &'a [&'a str])` locally. The scripts' file names and hashes match between their creation and use.

**Review Focus.** Five lines, each with its test in Task 2.
