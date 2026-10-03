# Curādi gaṇa slice 10h Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate fifty of the fifty-one ādhṛṣīya rows (`10.0338 yu\ja~` … `10.0388 Dfza~`, all but `10.0368 za\da~`), whose ṇic the dhātupāṭha gaṇasūtra 10.0498 *ā dhṛṣād vā* makes optional, across laṭ / laṅ / loṭ / vidhiliṅ. The golden suite goes from 17964 to 21564 cells, and curādi from 208 to 258 of its 509 rows.

**Architecture:** Six tasks:
- **Task 1** creates the worktree and checks the baseline.
- **Task 2** is the engine, green on its own: no curated root before this slice is in the ādhṛṣīya, vowel-final before ṇic, √dhū, √prī or √mṛj, so every golden and every prior trace is unchanged. It adds:
  - the gaṇasūtra 10.0498, a fifth optional-ṇic vikalpa, first in `SANADI`;
  - `PadaAssignment::NicUbhayapada` with `Tag::NicUbhayapada`, which `skip_nic` turns into `Tag::Ubhayapadin` on the ṇic-less branch (1.3.72's);
  - 7.3.37.2, 7.2.115 and a sanādi 6.1.78 before ṇic;
  - 7.2.114 in `SANADI` and in `GUNA`, keyed on `Tag::Mrj` from the row list `MRJ`;
  - `vrddhi_of`'s `F` arm.
  The tests go in first and fail.
- **Task 3** lands the fifty rows, their `OPTIONAL_NIC` entries, 400 golden rows and 2772 alternates, and every count, list, roster, uniqueness and `check()` assertion they move. The assertions go in first and fail; the rows and goldens make them pass.
- **Tasks 4–6** are the audit with the prior-trace diff and the doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.99.0, pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, checked out at `/tmp/vidyut-full`.

**Spec:** `docs/superpowers/specs/2026-10-03-curadi-gana-10h-design.md`. Its row table is the slice's scope. Its Decisions section explains:
- 10.0498's placement;
- `NicUbhayapada`;
- 7.2.115 with its sanādi 6.1.78, 7.3.37.2 and 7.2.114;
- the range-aware helper and the narrowed exclusion;
- `10.0367 arha~`'s resolution;
- the homographs.

**Workspace:** The spec and this plan are on the branch `curadi-10h`. Task 1 puts `/workspace` back on `main` and checks `curadi-10h` out at `/workspace/.worktrees/curadi-10h`. Every path below is relative to that directory unless it starts with `/`.

**Provenance.** This slice was built end to end on a throwaway worktree, `/workspace/.worktrees/curadi-10h-proto` (branch `proto-10h`, on the spec commit `e28a74f`). Its throwaway commits are `0c712f1` (engine), `00d3d6c` (rows and assertions) and `9915b65` (audit and docs), and Task 6 deletes it. Every code block and script below is that prototype's code, and it was checked three ways:
- **Prototype checks.** The prototype's final state:
  - passed the full suite, clippy `-D warnings` and `fmt-check`;
  - its generator found all 3600 new cells equal to vidyut's (3600 cells, 6372 forms, 0 differences);
  - the audit at 361 roots / 21564 cells / 29096 forms showed zero differences, with 4320 blocked branches and the `entry` control failing on 36 cells;
  - a main-vs-prototype dump of all 22724 prior live branches was byte-identical.
- **Replay.** The scripts below, run in this plan's order on a fresh worktree at `e28a74f`, reproduced the prototype byte for byte in every tracked file. (The replay script is `/tmp/vidyut-full/slice10h/replay_10h.sh`, a verification aid this plan does not need.) Task 2 Step 2's and Task 3 Step 5's failing lists below are that replay's, and so are the per-binary counts.
- **Negative control.** Earlier, an exploratory prototype of the same design (since replaced) dropped √prī's nuk and saw 72 cells differ.

The prototype did **not** run the mutation campaign. Task 5's campaign numbers are expectations, derived from the measured mutant lists (below), and the campaign measures them.

**Throwaway scripts.** Everything under `/tmp/vidyut-full/slice10h/` and the two vidyut examples (`curadi_goldens_10h.rs`, `trace_dump_10h.rs`) never ship. Each is reproduced in full in this plan with its sha256, so it can be recreated if `/tmp` was cleaned. Recreate a file only if it is missing, and check its hash either way (`sha256sum <file>`).

## Global Constraints

- **The new grammar is exactly this:**
  - **ids:** 10.0498 (`SANADI`, first); the vārttika 7.3.37.2, 7.2.115, a second 6.1.78 and a 7.2.114 (`SANADI`, between 6.4.92 and the sanādi 7.3.86); a second 7.2.114 (`GUNA`, right before 7.3.84). The rule-order pin goes from 148 to 154.
  - **tags:** `Tag::NicUbhayapada`, `Tag::Mrj`.
  - **variant:** `PadaAssignment::NicUbhayapada`.
  - **const:** `samjna::MRJ = ["10.0386"]`.
  - **sound table:** `vrddhi_of`'s `f` arm becomes `'f' | 'F'`.
  Nothing else in the engine changes.
- **Rows:** exactly the spec's fifty, each `code` what `stored_form` computes. Six are `NicUbhayapada` (`10.0345`, `10.0365`, `10.0372`, `10.0373`, `10.0379`, `10.0387`) and the rest `Nic`. Every one's `OPTIONAL_NIC` id is `"10.0498"`. `OPTIONAL_NIC` has exactly 119 entries, in dhātupāṭha order.
- **Not modelled:** 7.1.58 as a credited step (the stored-num simplification stands), and 7.3.39 for curādi √lī (vidyut keeps it out too).
- **Out of scope:**
  - `10.0368 za\da~` (upasarga ā, 7.3.78);
  - every row in `10.0279..=10.0337` (āsvadīya);
  - √ci, √gṛ, √yu;
  - 2565, 2571, 2572;
  - the causative.
- **Pre-existing cells must stay byte-identical, traces included,** with no exception. Regenerate no prior golden.
- **Goldens come from the generator, which asserts engine = vidyut cell by cell, and their sha256 must match this plan's.** **Do not edit a golden to match the engine.** If the generator reports a difference, or a hash differs, stop and report.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit. Run `git branch --show-current` before every commit and the push: it must print `curadi-10h` (a detached HEAD strands commits).
- `mise run test` takes 2–3 minutes on this host under its usual external load (load 50–80). Run it in the **foreground** with a timeout of 600000 ms. Never background it and end a turn.
- `mise run test -- -p X` does not scope. Scope with `mise exec -- cargo test -p <crate> <filter>`. To see every failing binary at once, use `mise exec -- cargo test --workspace --no-fail-fast`.
- The `cargo-mutants` mise shim fails here. Use the real binary, `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`, under `mise exec --`.
- `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes its `panini` and `panini-data` dev-deps. Repoint them at this worktree before generating or auditing, and back at `/workspace/crates` after. Check with `grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml` every time.
- Never wait on a process with `pgrep -f` (it matches its own shell); use `pgrep -x cargo-mutants` or `kill -0 <pid>`.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **1.3.72 on the wrong branch.** A `NicUbhayapada` row's ṇic branch must credit 1.3.74, never 1.3.72. Its ṇic-less ātmanepada must be live and credit 1.3.72; every other optional-ṇic row's ṇic-less ātmanepada must block. Goldens hold the forms but not the credit. → Task 2 `a_nic_ubhayapada_roots_nic_branch_is_1_3_74s_not_1_3_72s` and `the_nicless_fork_makes_a_nic_ubhayapada_root_ubhayapadin`; Task 3 `no_nic_pada_rule_reaches_a_nicless_branch` (corpus-wide: no ṇic branch of any optional-ṇic row credits 1.3.72) and `the_optional_nic_ids_are_credited_only_on_their_rows`.
2. **The new sanādi rules firing off their rows.** 7.2.115 reads the ṇic term's ṇit tag, not the row, and the sanādi 6.1.78 any ec-final aṅga before ṇic; a future vowel-final curādi row meets both. → Task 3 `the_10h_vrddhi_and_nuk_rules_fire_only_on_their_rows`: 7.2.115 on exactly the eight vowel-final rows, the sanādi 6.1.78 on exactly six, 7.3.37.2 on two, 7.2.114 on one, in each stage. Also Task 4's prior-trace diff (all 22724 prior live branches byte-identical).
3. **Two ids, two entries each.** With 6.1.78 and 7.2.114 in two stages, `rules().find(|r| r.id == …)` returns the first. The guṇa stage's seven 6.1.78 unit tests would silently test the sanādi entry, and they fail that way. → Task 2's stage-scoped lookups (`GUNA.iter().find`), `tinanta_rule_order_is_pinned` (154 ids, both pairs placed), and `mrjer_vrddhih_takes_vrddhi_before_shap_on_the_nicless_mrj_only` (the `GUNA` 7.2.114 declines on a ṇijanta and before a ṅit sārvadhātuka).
4. **√dhū's and √prī's three-way fork.** The nuk branch must escape 7.2.115 and the vṛddhi branch must take no nuk. Each cell holds three readings, and the loṭ tātaṅ cells nine. → Task 2 `nuk_follows_dhu_and_pri_before_nic_only` and `aco_nniti_lengthens_an_ac_final_anga_before_nic` (`DUn` declines); Task 3 `DUnayati_and_DAvayati_take_nuk_or_vrddhi_never_both` and the census (`nines` 6, `threes` 525).
5. **The range-keyed verdict.** The helper's 10.0498 arm is keyed on position, so it would also accept `10.0368`, and the āsvadīya is still unread. It also now separates `10.0367 arha~` from the uncurated `10.0257 arha~`, which share gaṇa, stored form and artha. → Task 3 `optional_nic_matches_upadesha_markers` (no table row in `10.0279..=10.0337` or at `10.0368`; the helper's verdict for `10.0368` asserted). `dhatupatha_numbers_resolve_upstream` (`10.0367` resolves through the verdict-aware filter, no new pin). `curated_pada_agrees_with_upadesha_markers` (a marked curādi row is allowed only as `NicUbhayapada`, re-derived from the upadeśa's `~^`/`Y`).

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-prakriya/src/tinanta/sanadi.rs` | Task 2: module doc, `skip_nic`'s swap and doc, 10.0498, 7.3.37.2, 7.2.115, 6.1.78, 7.2.114, five unit tests and one widened |
| `crates/panini-prakriya/src/tinanta/guna.rs` | Task 2: 7.2.114 before 7.3.84, its unit test, seven stage-scoped 6.1.78 lookups. Task 4: one count |
| `crates/panini-prakriya/src/tinanta/samjna.rs` | Task 2: `MRJ`, 1.3.74's comment, the `NicUbhayapada` arm of `pada_anga`, two tests |
| `crates/panini-prakriya/src/tinanta/mod.rs` | Task 2: the `NicUbhayapada` arm, `Tag::Mrj` from `MRJ` |
| `crates/panini-prakriya/src/term.rs` | Task 2: `Tag::NicUbhayapada`, `Tag::Mrj`, `Nijanta`'s doc |
| `crates/panini-prakriya/src/tinanta/sound.rs` | Task 2: `vrddhi_of`'s `F`, its doc and test |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Task 2: order (154), vikalpa and bars pins |
| `crates/panini-data/src/lib.rs` | Task 2: the variant, `padas()`, one test line. Task 3: 50 rows, `OPTIONAL_NIC` (119) and its doc, the curādi row list, `Dhatu::pada`/`padas` docs, the range-aware helper, the marker, uniqueness and pada tests, the row count. Task 4: two doc comments |
| `crates/panini/tests/paradigm/main.rs` | Task 3: vikalpa list, census, keys, 10h paragraph, the ākusmīya and adanta homograph tests, the pada-ambiguous comment and set, `curadi_analyses_its_adhrsiya_forms`. Task 4: audit-chain prose |
| `crates/panini/tests/paradigm/data/curadi.rs` | Task 3: 400 golden rows, 2772 alternates |
| `crates/panini/tests/trace/curadi.rs`, `crates/panini/tests/trace/juhotyadi.rs` | Task 3: module doc, four moved rosters, one helper, two new tests |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, the 10g spec | Task 4 |
| `AGENTS.md`, maybe `mise.toml` | Task 5 |

---

## Task 1: The worktree and baseline

**Files:** none. **Interfaces:** none.

- [ ] **Step 1: Create the worktree**

```bash
cd /workspace
git status --short                   # empty
git log --oneline -2 curadi-10h      # the plan commit, e28a74f (spec)
git checkout main                    # curadi-10h can be checked out in one worktree only
git log --oneline -1                 # b19047b, the 10g merge
git worktree add .worktrees/curadi-10h curadi-10h
cd .worktrees/curadi-10h
git branch --show-current            # curadi-10h
```

`/workspace` stays on `main` for the whole slice: Task 4's prior-trace dump builds the main engine from `/workspace/crates`.

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: everything passes at 17964 cells. `panini-prakriya` reports 419 tests, the `trace` binary 210, `paradigm` 25 and `panini-data` 27 (and `panini` 7, `roundtrip` 1, `panini-analyze` 8, `cli` 5, `panini-lipi` 6).

---

## Task 2: The engine — 10.0498, `NicUbhayapada`, 7.3.37.2, 7.2.115, 6.1.78, 7.2.114

No curated root before this slice is ādhṛṣīya, vowel-final before ṇic, √dhū, √prī or √mṛj, and no row carries `NicUbhayapada` or `Tag::Mrj` yet. So every golden and every prior trace is unchanged, and the suite is green at the end of the task. Task 4's prior-trace diff is the corpus-wide proof.

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (the `NicUbhayapada` variant after `Nic`; `PadaAssignment::padas`; `padas_maps_each_assignment_to_its_derivable_padas`)
- Modify: `crates/panini-prakriya/src/term.rs` (`Tag::NicUbhayapada` after `Nic`, `Tag::Mrj` after `Ghu`, `Nijanta`'s doc)
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs` (`derive`'s pada match; `Tag::Mrj` after the `JNAPADI` block)
- Modify: `crates/panini-prakriya/src/tinanta/samjna.rs` (`MRJ` after `GHU`; 1.3.74's comment; `pada_anga`; two tests)
- Modify: `crates/panini-prakriya/src/tinanta/sanadi.rs` (module doc; `skip_nic`; five rules; tests)
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (the `vrddhi_of` import; 7.2.114 before 7.3.84; a test; seven 6.1.78 lookups)
- Modify: `crates/panini-prakriya/src/tinanta/sound.rs` (`vrddhi_of`'s doc and `F` arm; `vrddhi_of_ac_vowels_all_arms`)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (the three pins and the bars doc)

**Interfaces:**
- Produces:
  - `PadaAssignment::NicUbhayapada` (both padas);
  - `Tag::NicUbhayapada`, `Tag::Mrj`;
  - `samjna::MRJ: [&str; 1]`;
  - the rule ids 10.0498, 7.3.37.2, 7.2.115 and 7.2.114 (two entries), and a second 6.1.78;
  - `vrddhi_of('F') == Some("Ar")`.
- Consumes:
  - `skip_nic` (10f), `optional_nic`;
  - the sanādi tests' `with_nic(root, nic)`, `optional_nic_dhatu(number, root, tag)` and `rule(id)`;
  - the samjna tests' `nic_prakriya(pada)` and `upstream_upadesha(number)`;
  - `terms::following_sarvadhatuka`.

- [ ] **Step 1: The failing tests**

Create `/tmp/vidyut-full/slice10h/engine_10h.py` if it is missing (sha256 `fb4b2bbdd5d31e84739afa8f538bd265f2311b3f02c33d739ff70cc301f4eb2d`). It holds the declarations, the code and the tests. `--tests-only` applies the declarations (the variant, the two tags, `MRJ`, and the match arms that must name them) and the tests, but no rule:

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10h's engine edits — 10.0498, NicUbhayapada, 7.3.37.2,
7.2.115 with its sanādi 6.1.78, 7.2.114 in both stages — with their unit
tests and rule pins. Every `old` must occur exactly once in its file; nothing
is written if one fails. Run from the worktree root.
`--tests-only` applies the declarations (the new variant, tags, `MRJ` and the
match arms that must name them) and the tests, but no rule: TDD, see them fail
first."""
import sys
TESTS_ONLY = '--tests-only' in sys.argv
DATA = 'crates/panini-data/src/lib.rs'
TERM = 'crates/panini-prakriya/src/term.rs'
MOD = 'crates/panini-prakriya/src/tinanta/mod.rs'
SAMJNA = 'crates/panini-prakriya/src/tinanta/samjna.rs'
SANADI = 'crates/panini-prakriya/src/tinanta/sanadi.rs'
GUNA = 'crates/panini-prakriya/src/tinanta/guna.rs'
SOUND = 'crates/panini-prakriya/src/tinanta/sound.rs'
PINS = 'crates/panini-prakriya/src/tinanta/derivation_tests.rs'

DECLS = {DATA: [
("""    Nic,
    /// Ātmanepada only, sanctioned by the dhātupāṭha gaṇasūtra 10.0496""",
"""    Nic,
    /// Both padas derive on the ṇic branch, exactly as for `Nic` — and on the
    /// ṇic-LESS branch too, because the row's own upadeśa is svarita or ñit.
    /// One sūtra sanctions the ātmanepada on each branch: 1.3.74 *ṇicaś ca*
    /// where ṇic is taken, 1.3.72 *svaritañitaḥ* where it is not. vidyut-
    /// prakriya reads the svarita/ñit marker of the term it is handed, and on
    /// the ṇic branch that term is ṇic, which carries none: 1.3.72 never
    /// reaches a ṇic branch, and 1.3.74 never a ṇic-less one. Only a row whose
    /// ṇic is optional (`OPTIONAL_NIC`) can carry it.
    NicUbhayapada,
    /// Ātmanepada only, sanctioned by the dhātupāṭha gaṇasūtra 10.0496"""),
("""            | PadaAssignment::Nic => &[Pada::Parasmaipada, Pada::Atmanepada],""",
"""            | PadaAssignment::Nic
            | PadaAssignment::NicUbhayapada => &[Pada::Parasmaipada, Pada::Atmanepada],"""),
], TERM: [
("""    Nic,
    /// The dhātu's ātmanepada is sanctioned by the dhātupāṭha gaṇasūtra
    /// 10.0496""",
"""    Nic,
    /// The dhātu's ṇic-less branch is ubhayapadī by 1.3.72: the data layer's
    /// `PadaAssignment::NicUbhayapada`, carried beside `Nic`, which licenses
    /// the ṇic branch. Read only by the sanādi stage's optional-ṇic forks
    /// (`skip_nic`), which add `Ubhayapadin` when they strip `Nic` from a root
    /// carrying it. 1.3.72 still reads `Ubhayapadin` alone, so it fires on
    /// the ṇic-less branch only, and the ṇic branch's credit stays 1.3.74's.
    NicUbhayapada,
    /// The dhātu's ātmanepada is sanctioned by the dhātupāṭha gaṇasūtra
    /// 10.0496"""),
("""    /// recorded. Read by 6.4.113's *aghoḥ* and by 6.4.119.
    Ghu,""",
"""    /// recorded. Read by 6.4.113's *aghoḥ* and by 6.4.119.
    Ghu,
    /// 7.2.114 *mṛjer vṛddhiḥ*: the aṅga is √mṛj. Set by `tinanta::derive`
    /// from the row NUMBER (`tinanta::samjna::MRJ`), as `Ghu` is: the sūtra
    /// names a root, and a root is a dhātupāṭha row, not a spelling. A
    /// saṁjñā verdict, so no step is recorded. Read by the two 7.2.114
    /// entries, in `tinanta::sanadi` and `tinanta::guna`.
    Mrj,"""),
], SAMJNA: [
("""pub(crate) const GHU: [&str; 6] = [
    "01.1050", "01.1079", "01.1117", "03.0010", "03.0011", "04.0043",
];
""",
"""pub(crate) const GHU: [&str; 6] = [
    "01.1050", "01.1079", "01.1117", "03.0010", "03.0011", "04.0043",
];

/// 7.2.114 mṛjer vṛddhiḥ: the dhātupāṭha rows that are √mṛj. By number, as
/// `GHU` is: the sūtra names a root, and a root is a row. `10.0386 mfjU~`
/// (curādi) is the one curated; adādi's `02.0061 mfjU~` joins the list when
/// it is. Pinned to upstream by `mrj_is_the_curated_row_7_2_114_names`.
/// `super::derive` reads this to add `Tag::Mrj`.
pub(crate) const MRJ: [&str; 1] = ["10.0386"];
"""),
("""            PadaAssignment::Nic => t.add(Tag::Nic),
            PadaAssignment::Akusmiya => t.add(Tag::Akusmiya),
            PadaAssignment::AaGarviya => t.add(Tag::AaGarviya),
        }
        t
    }""",
"""            PadaAssignment::Nic => t.add(Tag::Nic),
            PadaAssignment::NicUbhayapada => {
                t.add(Tag::Nic);
                t.add(Tag::NicUbhayapada);
            }
            PadaAssignment::Akusmiya => t.add(Tag::Akusmiya),
            PadaAssignment::AaGarviya => t.add(Tag::AaGarviya),
        }
        t
    }"""),
], MOD: [
("""        PadaAssignment::Nic => t.add(Tag::Nic),
        PadaAssignment::Akusmiya => t.add(Tag::Akusmiya),""",
"""        PadaAssignment::Nic => t.add(Tag::Nic),
        PadaAssignment::NicUbhayapada => {
            t.add(Tag::Nic);
            t.add(Tag::NicUbhayapada);
        }
        PadaAssignment::Akusmiya => t.add(Tag::Akusmiya),"""),
]}

CODE = {TERM: [
("""    /// The aṅga is a ṇijanta: 3.1.32 *sanādyantā dhātavaḥ* folded ṇic into
    /// it, so its final `i` is ṇic's. A saṁjñā verdict, set by 3.1.32 and
    /// pinned by its unit test; no rule reads it yet. 6.4.51 *ṇer aniṭi*,
    /// in an ārdhadhātuka-lakāra slice, is the first rule that will.
    Nijanta,""",
"""    /// The aṅga is a ṇijanta: 3.1.32 *sanādyantā dhātavaḥ* folded ṇic into
    /// it, so its final `i` is ṇic's. A saṁjñā verdict, set by 3.1.32 and
    /// pinned by its unit test. Read only by the guṇa stage's 7.2.114, which
    /// declines on it: a ṇijanta √mṛj took its vṛddhi before ṇic, in
    /// `tinanta::sanadi`. 6.4.51 *ṇer aniṭi*, in an ārdhadhātuka-lakāra
    /// slice, will read it too.
    Nijanta,"""),
], MOD: [
("""    if dhatu.gana == Gana::Curadi && JNAPADI.contains(&dhatu.dhatupatha) {
        t.add(Tag::Mit);
    }
""",
"""    if dhatu.gana == Gana::Curadi && JNAPADI.contains(&dhatu.dhatupatha) {
        t.add(Tag::Mit);
    }
    // 7.2.114's √mṛj, likewise by row number (`samjna::MRJ`).
    if samjna::MRJ.contains(&dhatu.dhatupatha) {
        t.add(Tag::Mrj);
    }
"""),
], SAMJNA: [
("""    // kriyāphale* above). Affix-keyed: the guard is Tag::Nic, the data
    // layer's PadaAssignment::Nic, which every curated curādi row outside the
    // ākusmīya carries (those are 10.0496's, `Tag::Akusmiya`, and never reach
    // here).""",
"""    // kriyāphale* above). Affix-keyed: the guard is Tag::Nic, the data
    // layer's PadaAssignment::Nic or NicUbhayapada, which every curated curādi
    // row outside the ākusmīya carries (those are 10.0496's, `Tag::Akusmiya`,
    // and never reach here). On an optional-ṇic row's ṇic-less branch the
    // sanādi fork has removed it, so 1.3.74 declines there; a NicUbhayapada
    // row's ṇic-less ātmanepada is 1.3.72's."""),
], SOUND: [
("""/// Vṛddhi substitute of a vowel (1.1.1 vṛddhir ādaic; golden-reachable via
/// 6.1.90 for e/I (eD/Ikz), E (loṭ's 3.4.93), u (rudhādi 7d's √und,
/// `Onad`), and f (tanādi 8a's √ṛṇ laṅ, `ArRot`), and via 6.1.88 *vṛddhir
/// eci* for E as well (juhotyādi 3c's dadE/mimE/jihE/daDE) — the remaining
/// arms (a/A/U/o/O) are unit-test-only; see `vrddhi_of_ac_vowels_all_arms`
/// below for the full inventory).""",
"""/// Vṛddhi substitute of a vowel (1.1.1 vṛddhir ādaic; golden-reachable via
/// 6.1.90 for e/I (eD/Ikz), E (loṭ's 3.4.93), u (rudhādi 7d's √und,
/// `Onad`), and f (tanādi 8a's √ṛṇ laṅ, `ArRot`), via 6.1.88 *vṛddhir
/// eci* for E as well (juhotyādi 3c's dadE/mimE/jihE/daDE), and via 7.2.115
/// *aco ñṇiti* for I, U, f and F (curādi 10h's `lAyayati`, `BAvayati`,
/// `vArayati`, `jArayati`) and 7.2.114 for f (`mArjati`) — the remaining
/// arms (a/A/o/O) are unit-test-only; see `vrddhi_of_ac_vowels_all_arms`
/// below for the full inventory)."""),
("""        // 1.1.51 uraR raparaH: a vṛddhi substitute for f carries the r.
        'f' => Some("Ar"),""",
"""        // 1.1.51 uraR raparaH: a vṛddhi substitute for f or F carries the r.
        'f' | 'F' => Some("Ar"),"""),
], SANADI: [
("""//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, ṇic's
//! it-lopa (1.3.9), 3.4.114, 6.4.48, 7.2.116, 6.4.92, 7.3.86, 3.1.32 —
//! opened by four Kaumudī vikalpas (2564, 2570, 2573.1, 2573.3) that fork a
//! root whose ṇic is optional into its ṇic and ṇic-less branches, a fifth
//! (2573.2) that forks `pata`'s ṇic branch on its final `a`, and three
//! dhātupāṭha gaṇasūtras: 10.0496 and 10.0497, which settle an ākusmīya or
//! ā-garvīya root's pada, and 10.0493, which credits a jñapādi root's
//! mit-tva, all before ṇic is added.""",
"""//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, ṇic's
//! it-lopa (1.3.9), 3.4.114, 6.4.48, 7.2.116, 6.4.92, the vārttika 7.3.37.2,
//! 7.2.115, 6.1.78, 7.2.114, 7.3.86, 3.1.32 — opened by five vikalpas that
//! fork a root whose ṇic is optional into its ṇic and ṇic-less branches (the
//! dhātupāṭha gaṇasūtra 10.0498 and the Kaumudī's 2564, 2570, 2573.1,
//! 2573.3), a sixth (2573.2) that forks `pata`'s ṇic branch on its final
//! `a`, and three more dhātupāṭha gaṇasūtras: 10.0496 and 10.0497, which
//! settle an ākusmīya or ā-garvīya root's pada, and 10.0493, which credits a
//! jñapādi root's mit-tva, all before ṇic is added."""),
("""//! Every rule self-guards: the four optional-ṇic vikalpas on the row's
//! `OPTIONAL_NIC` entry, 2573.2 on `pata`'s, 10.0496 on `Tag::Akusmiya`, 10.0497 on
//! `Tag::AaGarviya`, 10.0493 on `Tag::Mit`, 3.1.25 on `Tag::Curadi`, the
//! rest on ṇic being present (6.4.48 on an `a`-final aṅga as well, 6.4.92 on
//! `Tag::Mit`, and 7.2.116 and 7.3.86 decline on 6.4.48's `Tag::AtLopa`).""",
"""//! Every rule self-guards: the five optional-ṇic vikalpas on the row's
//! `OPTIONAL_NIC` entry, 2573.2 on `pata`'s, 10.0496 on `Tag::Akusmiya`, 10.0497 on
//! `Tag::AaGarviya`, 10.0493 on `Tag::Mit`, 3.1.25 on `Tag::Curadi`, the
//! rest on ṇic being present (6.4.48 on an `a`-final aṅga as well, 6.4.92 on
//! `Tag::Mit`, 7.3.37.2 on √dhū's and √prī's text, 7.2.115 on an ac-final
//! aṅga, 6.1.78 on an ec-final one, 7.2.114 on `Tag::Mrj`, and 7.2.116 and
//! 7.3.86 decline on 6.4.48's `Tag::AtLopa`)."""),
("""use crate::tinanta::sound::{guna_of, hrasva_of};""",
"""use crate::tinanta::sound::{guna_of, hrasva_of, vrddhi_of};"""),
("""/// gone they decline on their own guards, and 1.3.78 finds a genuine śeṣa
/// (parasmaipada; ātmanepada blocks). 3.1.25 is barred by the caller's
/// `bars`, so no ṇic is ever added and every ṇic-reading rule below
/// declines. Changes no text.""",
"""/// gone they decline on their own guards, and 1.3.78 finds a genuine śeṣa
/// (parasmaipada; ātmanepada blocks) — unless the row is
/// `PadaAssignment::NicUbhayapada`, whose svarita or ñit upadeśa makes the
/// ṇic-less branch 1.3.72's: `Tag::NicUbhayapada` adds `Tag::Ubhayapadin`
/// here, and 1.3.72 sanctions the ātmanepada. 3.1.25 is barred by the
/// caller's `bars`, so no ṇic is ever added and every ṇic-reading rule below
/// declines. Changes no text."""),
("""    for tag in [Tag::Nic, Tag::Akusmiya, Tag::AaGarviya] {
        p.terms[ANGA].remove(tag);
    }
    p.record(id, name, before);""",
"""    for tag in [Tag::Nic, Tag::Akusmiya, Tag::AaGarviya] {
        p.terms[ANGA].remove(tag);
    }
    if p.terms[ANGA].has(Tag::NicUbhayapada) {
        p.terms[ANGA].add(Tag::Ubhayapadin);
    }
    p.record(id, name, before);"""),
("""pub(crate) static SANADI: &[Rule] = &[
    // Kaumudī 2564: an idit curādi root takes ṇic optionally (*cintayati* /
    // *cintati*). The first of four vikalpa rules that decide ṇic before the
    // pada gaṇasūtras, as vidyut-prakriya does ("First decide Ric-pratyaya,""",
"""pub(crate) static SANADI: &[Rule] = &[
    // 10.0498 ā dhṛṣād vā: the ādhṛṣīya — the curādi rows from `10.0338
    // yu\\ja~` to `10.0388 Dfza~` — take ṇic optionally (*yojayati* /
    // *yojati*). A dhātupāṭha gaṇasūtra, numbered as vidyut-prakriya numbers
    // it (`DP("10.0498")`), like 10.0496 and 10.0497. First of the five
    // optional-ṇic vikalpas because vidyut checks the ādhṛṣīya before idit or
    // udit (`dhatu_karya.rs`): `hisi~` and `kaWi~` are idit and `tanu~` and
    // `mfjU~` udit, and all four are 10.0498's. Keyed, like 2564 below, on
    // the row's `OPTIONAL_NIC` entry; the declined branch is the ṇic one and
    // stays index 0.
    Rule {
        id: "10.0498",
        name: "A DfzAd vA",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "10.0498", "A DfzAd vA"),
    },
    // Kaumudī 2564: an idit curādi root takes ṇic optionally (*cintayati* /
    // *cintati*). One of the five vikalpa rules that decide ṇic before the
    // pada gaṇasūtras, as vidyut-prakriya does ("First decide Ric-pratyaya,"""),
("""    // 7.3.86 pugantalaghūpadhasya ca, before ṇic: guṇa of a laghu ik upadhā""",
"""    // Vārttika 7.3.37.2 dhūñprīñor nug vaktavyaḥ: √dhū and √prī take the
    // augment nuk before ṇic (*dhūnayati*, *prīṇayati*; 8.4.2 makes the ṇ).
    // Optional, as vidyut-prakriya applies it (per Haradatta, on prīñ in the
    // Siddhānta-kaumudī), and numbered as vidyut numbers it
    // (`Varttika("7.3.37.2")`): the first vārttika id in this engine. Before
    // 7.2.115, so the nuk branch leaves the aṅga no final ac to lengthen;
    // declined, 7.2.115 gives *dhāvayati*, *prāyayati*. nuk is kit, so it is
    // the aṅga's final part (1.1.46 ādyantau ṭakitau): its `n` is appended to
    // the aṅga's text, its it-lopa unrecorded, as no rule reads the augment
    // as a term. Keyed on the root's text, as vidyut keys it: in this stage
    // the aṅga is still the bare root.
    Rule {
        id: "7.3.37.2",
        name: "DUYprIYor nug vaktavyaH",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit))
                || !matches!(p.terms[ANGA].text.as_str(), "DU" | "prI")
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text.push('n');
            p.record("7.3.37.2", "DUYprIYor nug vaktavyaH", before);
            true
        },
    },
    // 7.2.115 aco ñṇiti: vṛddhi of an ac-final aṅga before a ñit or ṇit
    // affix. Before ṇic: `lI` → `lE`, `BU` → `BO`, `vf` → `vAr`, `jF` →
    // `jAr` (1.1.51 uraṇ raparaḥ). Only ṇit has a carrier in this engine
    // (`Tag::Rit`), as for 7.2.116. vidyut-prakriya credits it here, before
    // 3.1.32. An `a`-final root never reaches it: 6.4.48 has deleted the `a`.
    Rule {
        id: "7.2.115",
        name: "aco YRiti",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) {
                return false;
            }
            let mut text = p.terms[ANGA].text.clone();
            let Some(v) = text.pop().and_then(vrddhi_of) else {
                return false;
            };
            let before = p.snapshot();
            text.push_str(v);
            p.terms[ANGA].text = text;
            p.record("7.2.115", "aco YRiti", before);
            true
        },
    },
    // 6.1.78 eco 'yavāyāvaḥ, before ṇic's `i`: 7.2.115's `E` and `O` become
    // `Ay` and `Av` (`lE` → `lAy`, `BO` → `BAv`) while ṇic is still its own
    // term, where vidyut-prakriya credits it, before 3.1.32 folds ṇic in. A
    // second entry under the guṇa stage's id: that one reads the aṅga against
    // a vikaraṇa or an ending, never against ṇic, so the two never meet.
    Rule {
        id: "6.1.78",
        name: "eco'yavAyAvaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) {
                return false;
            }
            let mut text = p.terms[ANGA].text.clone();
            let sub = match text.pop() {
                Some('e') => "ay",
                Some('o') => "av",
                Some('E') => "Ay",
                Some('O') => "Av",
                _ => return false,
            };
            let before = p.snapshot();
            text.push_str(sub);
            p.terms[ANGA].text = text;
            p.record("6.1.78", "eco'yavAyAvaH", before);
            true
        },
    },
    // 7.2.114 mṛjer vṛddhiḥ: √mṛj's ik takes vṛddhi where guṇa would come.
    // Before ārdhadhātuka ṇic, `mfj` → `mArj`, ahead of 7.3.86, which then
    // finds no laghu ik (*mārjayati*). Guarded on `Tag::Mrj`
    // (`super::samjna::MRJ`); the ṇic-less branch is the guṇa stage's entry.
    Rule {
        id: "7.2.114",
        name: "mfjer vfdDiH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Mrj)
                || !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Ardhadhatuka))
            {
                return false;
            }
            let text = &p.terms[ANGA].text;
            let Some(i) = text.find(['i', 'u', 'f', 'x']) else {
                return false;
            };
            let v = vrddhi_of(text[i..].chars().next().unwrap()).unwrap();
            let text = format!("{}{v}{}", &text[..i], &text[i + 1..]);
            let before = p.snapshot();
            p.terms[ANGA].text = text;
            p.record("7.2.114", "mfjer vfdDiH", before);
            true
        },
    },
    // 7.3.86 pugantalaghūpadhasya ca, before ṇic: guṇa of a laghu ik upadhā"""),
], GUNA: [
("""use crate::tinanta::sound::{guna_of, is_jhal, is_vowel};""",
"""use crate::tinanta::sound::{guna_of, is_jhal, is_vowel, vrddhi_of};"""),
("""    // 7.3.84 sārvadhātukārdhadhātukayoḥ: guṇa of the aṅga's final ik.
    Rule {
        id: "7.3.84",""",
"""    // 7.2.114 mṛjer vṛddhiḥ, on the ṇic-less branch: before the sārvadhātuka
    // śap, √mṛj's `f` takes vṛddhi in place of 7.3.86's guṇa (`mfj` →
    // `mArj`, *mārjati*), and 7.3.86 then finds no laghu ik. The ṇic branch
    // took its vṛddhi in `super::sanadi`, so a ṇijanta aṅga (`Tag::Nijanta`)
    // declines here, as does one before a ṅit sārvadhātuka (1.1.5 kṅiti ca,
    // as at 7.3.84 below). Guarded on `Tag::Mrj` (`super::samjna::MRJ`).
    Rule {
        id: "7.2.114",
        name: "mfjer vfdDiH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Mrj) || p.terms[ANGA].has(Tag::Nijanta) {
                return false;
            }
            if following_sarvadhatuka(p).is_some_and(|t| t.has(Tag::Ngit)) {
                return false;
            }
            let text = &p.terms[ANGA].text;
            let Some(i) = text.find(['i', 'u', 'f', 'x']) else {
                return false;
            };
            let v = vrddhi_of(text[i..].chars().next().unwrap()).unwrap();
            let text = format!("{}{v}{}", &text[..i], &text[i + 1..]);
            let before = p.snapshot();
            p.terms[ANGA].text = text;
            p.record("7.2.114", "mfjer vfdDiH", before);
            true
        },
    },
    // 7.3.84 sārvadhātukārdhadhātukayoḥ: guṇa of the aṅga's final ik.
    Rule {
        id: "7.3.84","""),
]}

TESTS = {DATA: [
("""        assert_eq!(
            PadaAssignment::Nic.padas(),
            &[Pada::Parasmaipada, Pada::Atmanepada]
        );""",
"""        assert_eq!(
            PadaAssignment::Nic.padas(),
            &[Pada::Parasmaipada, Pada::Atmanepada]
        );
        assert_eq!(
            PadaAssignment::NicUbhayapada.padas(),
            &[Pada::Parasmaipada, Pada::Atmanepada]
        );"""),
], SOUND: [
("""        assert_eq!(vrddhi_of('f'), Some("Ar"));
        // Non-ac letters""",
"""        assert_eq!(vrddhi_of('f'), Some("Ar"));
        // 7.2.115 on √jṝ's ṇic branch (`jArayati`): F's vṛddhi carries the r too.
        assert_eq!(vrddhi_of('F'), Some("Ar"));
        // Non-ac letters"""),
], SAMJNA: [
("""    /// `pada_prakriya` for an ākusmīya root, hand-built as `nic_prakriya`
    /// is:""",
"""    /// `nic_prakriya` for a `PadaAssignment::NicUbhayapada` root's ṇic
    /// branch, tagged as `super::derive` tags it: √vṛ (`10.0345 vfY`).
    fn nic_ubhayapada_prakriya(pada: Pada) -> Prakriya {
        let mut p = nic_prakriya(pada);
        p.terms[ANGA].text = "vf".into();
        p.terms[ANGA].add(Tag::NicUbhayapada);
        p
    }

    #[test]
    fn a_nic_ubhayapada_roots_nic_branch_is_1_3_74s_not_1_3_72s() {
        // On the ṇic branch nothing has added `Tag::Ubhayapadin`: 1.3.72
        // declines in both padas, and 1.3.74 and 1.3.78 behave exactly as for
        // a `Nic` root. The ṇic-less branch is pinned in `super::sanadi`.
        let r1372 = SAMJNA.iter().find(|r| r.id == "1.3.72").unwrap();
        let r1374 = SAMJNA.iter().find(|r| r.id == "1.3.74").unwrap();
        let r1378 = SAMJNA.iter().find(|r| r.id == "1.3.78").unwrap();
        for pada in [Pada::Parasmaipada, Pada::Atmanepada] {
            let mut p = nic_ubhayapada_prakriya(pada);
            assert!(!(r1372.apply)(&mut p), "1.3.72 fired on vf {pada:?}");
            assert!(!p.blocked, "1.3.72 blocked vf {pada:?}");
            assert!(p.log.is_empty(), "1.3.72 recorded on vf {pada:?}");
        }
        let mut p = nic_ubhayapada_prakriya(Pada::Atmanepada);
        assert!((r1374.apply)(&mut p));
        assert!(!(r1378.apply)(&mut p), "1.3.78 fired on vf Atmanepada");
        assert!(!p.blocked);
        let mut p = nic_ubhayapada_prakriya(Pada::Parasmaipada);
        assert!(!(r1374.apply)(&mut p));
        assert!((r1378.apply)(&mut p), "1.3.78 declined vf Parasmaipada");
        assert!(!p.blocked);
    }

    /// `pada_prakriya` for an ākusmīya root, hand-built as `nic_prakriya`
    /// is:"""),
("""    #[test]
    fn ghu_is_exactly_the_six_rows_1_1_20_names() {""",
"""    #[test]
    fn mrj_is_the_curated_row_7_2_114_names() {
        // 7.2.114 mṛjer vṛddhiḥ. The curādi row is the one curated; adādi's
        // √mṛj has the same upadeśa and joins `MRJ` when it is curated.
        assert_eq!(MRJ, ["10.0386"]);
        assert_eq!(upstream_upadesha("10.0386"), Some("mfjU~"));
        assert_eq!(upstream_upadesha("02.0061"), Some("mfjU~"));
        assert!(!dhatus().iter().any(|d| d.dhatupatha == "02.0061"));
    }

    #[test]
    fn ghu_is_exactly_the_six_rows_1_1_20_names() {"""),
], SANADI: [
("""    #[test]
    fn sanadyanta_folds_nic_into_the_dhatu() {""",
"""    #[test]
    fn nuk_follows_dhu_and_pri_before_nic_only() {
        for (root, want) in [("DU", "DUn"), ("prI", "prIn")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("7.3.37.2").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["7.3.37.2"]);
            // No ṇic, or a pratyaya at `NIC` that is not ṇit: no nuk.
            let mut p = with_nic(root, None);
            assert!(!(rule("7.3.37.2").apply)(&mut p), "{root}");
            let mut p = with_nic(root, Some(&[Tag::Ardhadhatuka]));
            assert!(!(rule("7.3.37.2").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        // Other ī- and ū-final roots before ṇic: √lī, √bhū, √mī.
        for root in ["lI", "BU", "mI"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("7.3.37.2").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        assert!(rule("7.3.37.2").vikalpa);
    }

    #[test]
    fn aco_nniti_lengthens_an_ac_final_anga_before_nic() {
        for (root, want) in [
            ("lI", "lE"),
            ("BU", "BO"),
            ("jri", "jrE"),
            ("vf", "vAr"),
            ("jF", "jAr"),
        ] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("7.2.115").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["7.2.115"]);
        }
        // Hal-final: a root (cur), and √dhū after its nuk (DUn).
        for root in ["cur", "DUn"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("7.2.115").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        // No ṇic, or a pratyaya at `NIC` that is not ṇit.
        let mut p = with_nic("BU", None);
        assert!(!(rule("7.2.115").apply)(&mut p));
        let mut p = with_nic("BU", Some(&[Tag::Ardhadhatuka]));
        assert!(!(rule("7.2.115").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "BU");
    }

    #[test]
    fn eco_yavayavah_resolves_the_vrddhi_before_nic() {
        for (root, want) in [("lE", "lAy"), ("BO", "BAv"), ("ne", "nay"), ("Bo", "Bav")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("6.1.78").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["6.1.78"]);
        }
        // Not ec-final: `vAr` (7.2.115's f), `cur`.
        for root in ["vAr", "cur"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("6.1.78").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        // No ṇic, or a pratyaya at `NIC` that is not ṇit: the guṇa stage's
        // entry, not this one, reads an aṅga against a vikaraṇa.
        let mut p = with_nic("lE", None);
        assert!(!(rule("6.1.78").apply)(&mut p));
        let mut p = with_nic("Bo", None);
        p.terms.push(Term::new("a"));
        assert!(!(rule("6.1.78").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Bo");
    }

    #[test]
    fn mrjer_vrddhih_takes_vrddhi_before_ardhadhatuka_nic_on_mrj_only() {
        let mut p = with_nic("mfj", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        p.terms[ANGA].add(Tag::Mrj);
        assert!((rule("7.2.114").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "mArj");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["7.2.114"]);
        // Then 7.3.86 finds no laghu ik.
        assert!(!(rule("7.3.86").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "mArj");
        // The same shape without the tag is any other root: 7.3.86's guṇa.
        let mut p = with_nic("mfj", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!(!(rule("7.2.114").apply)(&mut p));
        assert!((rule("7.3.86").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "marj");
        // Tagged, but no ārdhadhātuka ṇic.
        for nic in [None, Some(&[Tag::Rit][..])] {
            let mut p = with_nic("mfj", nic);
            p.terms[ANGA].add(Tag::Mrj);
            assert!(!(rule("7.2.114").apply)(&mut p), "{nic:?}");
            assert_eq!(p.terms[ANGA].text, "mfj");
        }
    }

    #[test]
    fn sanadyanta_folds_nic_into_the_dhatu() {"""),
("""    #[test]
    fn an_optional_nic_rule_declines_off_its_rows() {""",
"""    #[test]
    fn the_nicless_fork_makes_a_nic_ubhayapada_root_ubhayapadin() {
        // `skip_nic`, through 2564 on its own row: with `Tag::NicUbhayapada`
        // the ṇic-less branch gains `Tag::Ubhayapadin` (1.3.72's); without it,
        // it gains nothing (1.3.78's śeṣa).
        for (extra, ubhaya) in [(Some(Tag::NicUbhayapada), true), (None, false)] {
            let mut p = optional_nic_dhatu("10.0193", "danS", Tag::Nic);
            if let Some(t) = extra {
                p.terms[ANGA].add(t);
            }
            assert!((rule("2564").apply)(&mut p));
            assert!(!p.terms[ANGA].has(Tag::Nic));
            assert_eq!(p.terms[ANGA].has(Tag::Ubhayapadin), ubhaya, "{extra:?}");
        }
        // The fork's rule declining leaves the ṇic branch untouched: no
        // `Tag::Ubhayapadin` there.
        let mut p = optional_nic_dhatu("10.0193", "danS", Tag::Nic);
        p.terms[ANGA].add(Tag::NicUbhayapada);
        assert!(!(rule("2570").apply)(&mut p));
        assert!(p.terms[ANGA].has(Tag::Nic));
        assert!(!p.terms[ANGA].has(Tag::Ubhayapadin));
    }

    #[test]
    fn an_optional_nic_rule_declines_off_its_rows() {"""),
("""    fn the_optional_nic_rules_are_vikalpas_that_bar_nic() {
        for id in ["2564", "2570", "2573.1", "2573.3"] {""",
"""    fn the_optional_nic_rules_are_vikalpas_that_bar_nic() {
        for id in ["10.0498", "2564", "2570", "2573.1", "2573.3"] {"""),
], GUNA: [
("""    #[test]
    fn pugantalaghupadhasya_one_char_anga_returns_false_without_panic() {""",
"""    #[test]
    fn mrjer_vrddhih_takes_vrddhi_before_shap_on_the_nicless_mrj_only() {
        // mfj + a + ti → mArj + a + ti; 7.3.86 then finds no laghu ik.
        let mrj = |text: &str| {
            let mut t = Term::new(text);
            t.add(Tag::Mrj);
            t
        };
        let mut p = Prakriya {
            terms: with_slots(vec![mrj("mfj"), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        let rule = GUNA.iter().find(|r| r.id == "7.2.114").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "mArj");
        let r7386 = GUNA.iter().find(|r| r.id == "7.3.86").unwrap();
        assert!(!(r7386.apply)(&mut p));
        // Untagged: 7.3.86's guṇa, as for any root.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("mfj"), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert!((r7386.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "marj");
        // A ṇijanta √mṛj took its vṛddhi before ṇic: declines.
        let mut anga = mrj("mfji");
        anga.add(Tag::Nijanta);
        let mut p = Prakriya {
            terms: with_slots(vec![anga, Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "mfji");
        // 1.1.5 kṅiti ca: a ṅit sārvadhātuka follower blocks it.
        let mut shap = Term::new("a");
        shap.add(Tag::Ngit);
        let mut p = Prakriya {
            terms: with_slots(vec![mrj("mfj"), shap, Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "mfj");
    }

    #[test]
    fn pugantalaghupadhasya_one_char_anga_returns_false_without_panic() {"""),
], PINS: [
("""        "2564", "2570", "2573.1", "2573.3", "2573.2", "10.0496", "10.0497", "10.0493", "3.1.25",
        "1.3.9", "3.4.114", "6.4.48", "7.2.116", "6.4.92", "7.3.86", "3.1.32", "1.3.12", "1.3.66",""",
"""        "10.0498", "2564", "2570", "2573.1", "2573.3", "2573.2", "10.0496", "10.0497", "10.0493",
        "3.1.25", "1.3.9", "3.4.114", "6.4.48", "7.2.116", "6.4.92", "7.3.37.2", "7.2.115",
        "6.1.78", "7.2.114", "7.3.86", "3.1.32", "1.3.12", "1.3.66","""),
("""        "7.3.87", "7.3.84", "7.3.86", "7.3.86", "7.3.92", "7.3.84", "7.1.102", "6.4.110",""",
"""        "7.3.87", "7.2.114", "7.3.84", "7.3.86", "7.3.86", "7.3.92", "7.3.84", "7.1.102", "6.4.110","""),
("""        "2564", "2570", "2573.1", "2573.3", "2573.2", "7.1.35", "3.4.111", "7.3.86", "6.4.117",
        "6.4.116", "6.4.115", "6.4.43", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56",""",
"""        "10.0498", "2564", "2570", "2573.1", "2573.3", "2573.2", "7.3.37.2", "7.1.35", "3.4.111",
        "7.3.86", "6.4.117", "6.4.116", "6.4.115", "6.4.43", "6.4.107", "8.2.74", "8.2.75",
        "8.4.65", "8.4.56","""),
("""/// Ids are not unique in the pipeline (7.3.84, 1.2.4 and 1.3.9 each appear
/// twice, 7.3.86 three times), so "runs after" means some later occurrence.""",
"""/// Ids are not unique in the pipeline (7.3.84, 1.2.4, 1.3.9, 6.1.78 and
/// 7.2.114 each appear twice, 7.3.86 three times), so "runs after" means some
/// later occurrence."""),
("""    let expected: Vec<(&str, &[&str])> = vec![
        ("2564", &["3.1.25", "2573.2"][..]),""",
"""    let expected: Vec<(&str, &[&str])> = vec![
        ("10.0498", &["3.1.25", "2573.2"][..]),
        ("2564", &["3.1.25", "2573.2"][..]),"""),
]}

# The guṇa stage's seven 6.1.78 unit tests look the rule up by id; with a second
# 6.1.78 in `SANADI`, `rules()` would find that one first. Scope them to GUNA,
# as `sanadi::tests::rule` already scopes its own lookups.
SCOPE = (GUNA, 'let rule = rules().find(|r| r.id == "6.1.78").unwrap();',
         'let rule = GUNA.iter().find(|r| r.id == "6.1.78").unwrap();', 7)

def merge(*ds):
    out = {}
    for d in ds:
        for f, e in d.items():
            out.setdefault(f, []).extend(e)
    return out

FILES = merge(DECLS, TESTS) if TESTS_ONLY else merge(DECLS, CODE, TESTS)
out, bad = {}, []
for path in set(FILES) | {SCOPE[0]}:
    s = open(path).read()
    for old, new in FILES.get(path, []):
        n = s.count(old)
        if n != 1:
            bad.append(f"{path}: {n}x {old[:70]!r}")
            continue
        s = s.replace(old, new)
    if path == SCOPE[0]:
        n = s.count(SCOPE[1])
        if n != SCOPE[3]:
            bad.append(f"{path}: {n}x the 6.1.78 lookup")
        s = s.replace(SCOPE[1], SCOPE[2])
    out[path] = s
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
for path, s in out.items():
    open(path, 'w').write(s)
print(f"applied {sum(len(e) for e in FILES.values())} edits and the 6.1.78 scoping")
```

```bash
python3 /tmp/vidyut-full/slice10h/engine_10h.py --tests-only      # applied 20 edits and the 6.1.78 scoping
```

The tests:
- **`sanadi.rs`:**
  - `nuk_follows_dhu_and_pri_before_nic_only`: `DU` → `DUn`, `prI` → `prIn`; declines with no ṇic, before a non-ṇit pratyaya, and on `lI`, `BU`, `mI`; vikalpa.
  - `aco_nniti_lengthens_an_ac_final_anga_before_nic`: `lI` → `lE`, `BU` → `BO`, `jri` → `jrE`, `vf` → `vAr`, `jF` → `jAr`; declines on `cur`, on `DUn`, and with no ṇit ṇic.
  - `eco_yavayavah_resolves_the_vrddhi_before_nic`: `lE` → `lAy`, `BO` → `BAv`, `ne` → `nay`, `Bo` → `Bav`; declines on `vAr`, `cur`, with no ṇic, and before a vikaraṇa.
  - `mrjer_vrddhih_takes_vrddhi_before_ardhadhatuka_nic_on_mrj_only`: `mfj` → `mArj` under `Tag::Mrj`, then 7.3.86 declines; untagged, 7.3.86 gives `marj`; declines without an ārdhadhātuka ṇic.
  - `the_nicless_fork_makes_a_nic_ubhayapada_root_ubhayapadin`: 2564's `skip_nic` adds `Tag::Ubhayapadin` only under `Tag::NicUbhayapada`, and a declining fork adds nothing.
  - `the_optional_nic_rules_are_vikalpas_that_bar_nic` gains 10.0498.
- **`guna.rs`:** `mrjer_vrddhih_takes_vrddhi_before_shap_on_the_nicless_mrj_only`: `mfj` + `a` → `mArj`, after which 7.3.86 declines. Untagged, 7.3.86 gives `marj`. It declines on a `Tag::Nijanta` aṅga and before a ṅit śap. The seven 6.1.78 tests look the rule up in `GUNA`.
- **`samjna.rs`:**
  - `a_nic_ubhayapada_roots_nic_branch_is_1_3_74s_not_1_3_72s`: on the ṇic branch, 1.3.72 neither fires nor blocks nor records in either pada, 1.3.74 sanctions the ātmanepada, and 1.3.78 the parasmaipada.
  - `mrj_is_the_curated_row_7_2_114_names`: `MRJ == ["10.0386"]`, both √mṛj upadeśas upstream, and `02.0061` uncurated.
- **`sound.rs`:** `vrddhi_of('F') == Some("Ar")`.
- **`derivation_tests.rs`:** the order pin (154), the vikalpa pin (10.0498 and 7.3.37.2 added) and the bars pin (10.0498's).

- [ ] **Step 2: Run them to see them fail**

Run: `mise exec -- cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .*FAILED|test result: FAILED"`
Expected: `panini-prakriya` `416 passed; 11 failed`, every other binary passing:

- `tinanta::derivation_tests::exactly_the_pinned_bars`
- `tinanta::derivation_tests::exactly_the_pinned_vikalpa_rules_are_optional`
- `tinanta::derivation_tests::tinanta_rule_order_is_pinned`
- `tinanta::guna::tests::mrjer_vrddhih_takes_vrddhi_before_shap_on_the_nicless_mrj_only`
- `tinanta::sanadi::tests::aco_nniti_lengthens_an_ac_final_anga_before_nic`
- `tinanta::sanadi::tests::eco_yavayavah_resolves_the_vrddhi_before_nic`
- `tinanta::sanadi::tests::mrjer_vrddhih_takes_vrddhi_before_ardhadhatuka_nic_on_mrj_only`
- `tinanta::sanadi::tests::nuk_follows_dhu_and_pri_before_nic_only`
- `tinanta::sanadi::tests::the_nicless_fork_makes_a_nic_ubhayapada_root_ubhayapadin`
- `tinanta::sanadi::tests::the_optional_nic_rules_are_vikalpas_that_bar_nic`
- `tinanta::sound::tests::vrddhi_of_ac_vowels_all_arms`

`a_nic_ubhayapada_roots_nic_branch_is_1_3_74s_not_1_3_72s` and `mrj_is_the_curated_row_7_2_114_names` pass already. They characterise the declarations, which this step applies, and no rule is involved.

- [ ] **Step 3: The change**

```bash
git checkout -- crates
python3 /tmp/vidyut-full/slice10h/engine_10h.py      # applied 34 edits and the 6.1.78 scoping
mise run fmt
```

- [ ] **Step 4: Run the tests and the suite**

```bash
mise exec -- cargo test -q -p panini-prakriya 2>&1 | grep -E "FAILED|test result"      # 427 passed
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS, with `panini-prakriya` 427, `trace` 210, `paradigm` 25, `panini-data` 27.

- [ ] **Step 5: Commit**

```bash
mise run lint
git branch --show-current      # curadi-10h
git add -A
git commit -m "feat(sanadi): 10.0498, NicUbhayapada, 7.3.37.2, 7.2.115 with 6.1.78, 7.2.114

The ādhṛṣīya's gaṇasūtra joins the optional-ṇic forks, first; a
NicUbhayapada row's ṇic-less branch becomes 1.3.72's. Before ṇic, 7.2.115
lengthens an ac-final aṅga and 6.1.78 resolves it, √dhū and √prī may take
the vārttika's nuk instead, and √mṛj takes 7.2.114's vṛddhi, as it does on
the ṇic-less branch in the guṇa stage. No curated row reaches any of it yet."
```

---

## Task 3: The rows, their goldens, and every assertion they move

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`, `crates/panini/tests/paradigm/data/curadi.rs`
- Modify: `crates/panini/tests/trace/curadi.rs`, `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes:
  - Task 2's rules, variant and tags;
  - 10f's `OPTIONAL_NIC`, `optional_nic`, `Dhatu::padas`;
  - the trace helpers `at` and `credited`;
  - `panini_prakriya::derive`, `panini::Panini::check`.
- Produces: the totals Task 4 documents. Those are 361 roots, 21564 cells and 29096 forms, 7532 alternates, 839 pada-ambiguous surfaces, 238 both-pada roots, and 199 `Nic` plus 6 `NicUbhayapada` rows that credit 1.3.74.

Each script below asserts that every `old` occurs exactly once and writes nothing otherwise. If one prints `not applied:`, nothing was written: stop and report, rather than editing around it.

- [ ] **Step 1: The paradigm assertions (failing)**

Create `/tmp/vidyut-full/slice10h/paradigm_10h.py` if it is missing (sha256 `e6567f14d362eac7df5ac5c16aaa880103f7f6a85eec2e260dd722c65c5c5b5b`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10h's edits to crates/panini/tests/paradigm/main.rs (the
census, ALTERNATES keys, docs, the vikalpa list, and the two analyzer tests
whose homographs it adds to). Every `old` must occur exactly once; nothing is
written if one fails. Run from the worktree root."""
import sys
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
E = [
# the vikalpa list
("""/// this is an integration test and the rule table is crate-internal. Order
/// here is unconstrained, since the list below is read only via
/// `.contains()`: it is NOT the pipeline order — the pin has 6.4.117,
/// 6.4.116, 6.4.115 and 6.4.43 fourth to seventh, right after 7.3.86, not
/// last as they sit here.
const VIKALPA_RULES: &[&str] = &[
    "2564", "2570", "2573.1", "2573.3", "2573.2", "7.1.35", "3.4.111", "7.3.86", "6.4.107",
    "8.2.74", "8.2.75", "8.4.65", "8.4.56", "6.4.115", "6.4.117", "6.4.116", "6.4.43",
];""",
"""/// this is an integration test and the rule table is crate-internal. Order
/// here is unconstrained, since the list below is read only via
/// `.contains()`: it is NOT the pipeline order — the pin has 6.4.117,
/// 6.4.116, 6.4.115 and 6.4.43 right after 7.3.86, not last as they sit
/// here.
const VIKALPA_RULES: &[&str] = &[
    "10.0498", "2564", "2570", "2573.1", "2573.3", "2573.2", "7.3.37.2", "7.1.35", "3.4.111",
    "7.3.86", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56", "6.4.115", "6.4.117", "6.4.116",
    "6.4.43",
];"""),
("""/// `ALTERNATES` is otherwise 4760 bare strings,""", """/// `ALTERNATES` is otherwise 7532 bare strings,"""),
("""/// naming 7.3.86 does not by itself mean the rule was optional. Sixteen of the 23 `7.3.86+8.4.56` keys are the mandatory firing (ten juhotyādi: 3e's laṅ eka cells and 3f's √kit and √dhiṣ ones; six curādi: slice 10a's √cur laṅ and vidhiliṅ prathama eka, and slice 10g's √div and √śṛdh on their ṇic branch, where the firing is the sanādi entry before ṇic; the other seven are the tanādi vikalpa), and so is the 7.3.86 of the one `7.3.86+8.2.75` key (√kit's `acikeH`) and of the four `7.3.86+7.1.35`/`7.3.86+7.1.35+8.4.56` keys (√cur's loṭ tātaṅ cells — the only keys where 7.3.86 precedes 7.1.35, because the sanādi stage runs first), and of 10f's six `2570+…7.3.86…` keys (√div's ṇic-less guṇa).""",
"""/// naming 7.3.86 does not by itself mean the rule was optional. Forty-eight of the 55 `7.3.86+8.4.56` keys are the mandatory firing (ten juhotyādi: 3e's laṅ eka cells and 3f's √kit and √dhiṣ ones; thirty-eight curādi: slice 10a's √cur laṅ and vidhiliṅ prathama eka, and slice 10g's √div and √śṛdh and slice 10h's sixteen laghu-ik ādhṛṣīya rows on their ṇic branch, where the firing is the sanādi entry before ṇic; the other seven are the tanādi vikalpa), and so is the 7.3.86 of the one `7.3.86+8.2.75` key (√kit's `acikeH`), of every `7.3.86+7.1.35`/`7.3.86+7.1.35+8.4.56` key (the same curādi roots' loṭ tātaṅ cells — the only keys where 7.3.86 precedes 7.1.35, because the sanādi stage runs first), and of every `2570+…7.3.86…` and `10.0498+…7.3.86…` key (a ṇic-less root's guṇa)."""),
# the census doc
("""/// 17964 cells total (1996 root×lakāra blocks × 9), of which 14488 hold exactly one form, 2792 hold two, 389 hold three (""",
"""/// 21564 cells total (2396 root×lakāra blocks × 9), of which 16072 hold exactly one form, 4472 hold two, 525 hold three ("""),
("""/// optional-ṇic rows add 114 two-form and 46 three-form cells, and slice 10g's
/// fifty-nine add 1888 two-form cells, a ṇic and a ṇic-less reading each), 141 hold four (""",
"""/// optional-ṇic rows add 114 two-form and 46 three-form cells, and slice 10g's
/// fifty-nine add 1888 two-form cells, a ṇic and a ṇic-less reading each, and
/// slice 10h's fifty add 1680 two-form cells the same way, in parasmaipada and
/// in four `NicUbhayapada` rows' ātmanepada, and 136 three-form ones, √dhū's
/// and √prī's ṇic branch forking again on 7.3.37.2's nuk), 237 hold four ("""),
("""/// 10g — the fifty-nine optional-ṇic rows' laṅ and vidhiliṅ parasmaipada
/// prathama eka, the same way), and""",
"""/// 10g — the fifty-nine optional-ṇic rows' laṅ and vidhiliṅ parasmaipada
/// prathama eka, the same way; and — new in slice 10h — forty-eight of the
/// fifty ādhṛṣīya rows', the same way), and"""),
("""/// 141
/// hold six (""", """/// 241
/// hold six ("""),
("""/// slice 10g — the fifty-nine optional-ṇic rows' loṭ parasmaipada prathama
/// and madhyama eka, the same way), one — new in""",
"""/// slice 10g — the fifty-nine optional-ṇic rows' loṭ parasmaipada prathama
/// and madhyama eka, the same way; and — new in slice 10h — those forty-eight
/// ādhṛṣīya rows', the same way, and √dhū's and √prī's laṅ and vidhiliṅ
/// parasmaipada prathama eka, three readings × 8.4.56), one — new in"""),
("""/// readings before *hi* are not a 2^k product; and two — new in slice 10f, the
/// engine's record — hold NINE: `pata`'s loṭ parasmaipada prathama and madhyama
/// eka, its three readings (2573.1's ṇic-less *pata-*, 2573.2's *pāta-*, and
/// 6.4.48's *pata-* with ṇic) × the tātaṅ triple. No cell holds eight.""",
"""/// readings before *hi* are not a 2^k product; and six hold NINE, the engine's
/// record: `pata`'s loṭ parasmaipada prathama and madhyama eka (new in slice
/// 10f), its three readings (2573.1's ṇic-less *pata-*, 2573.2's *pāta-*, and
/// 6.4.48's *pata-* with ṇic) × the tātaṅ triple, and — new in slice 10h —
/// √dhū's and √prī's, theirs (10.0498's ṇic-less one, 7.3.37.2's nuk, and
/// 7.2.115's vṛddhi with ṇic) × the same triple. No cell holds eight."""),
("""/// itself has 4760 rows, keyed 468 `8.4.56`, 460 `7.1.35`, 460 `7.1.35+8.4.56`,""",
"""/// itself has 7532 rows, keyed 536 `8.4.56`, 528 `7.1.35`, 528 `7.1.35+8.4.56`,"""),
("""/// `7.3.86+6.4.107`, 23 `7.3.86+8.4.56` (sixteen of them name the MANDATORY
/// 7.3.86, through the id it shares with the tanādi vikalpa arm: ten juhotyādi, slice 3e's
/// laṅ prathama and madhyama eka cells and slice 3f's √kit and √dhiṣ ones, whose root guṇa 7.3.86 credits, and six curādi, slice 10a's √cur laṅ and vidhiliṅ prathama eka and slice 10g's √div and √śṛdh ṇic-branch ones, whose guṇa before ṇic the sanādi 7.3.86 credits; the other seven are the tanādi vikalpa),""",
"""/// `7.3.86+6.4.107`, 55 `7.3.86+8.4.56` (forty-eight of them name the MANDATORY
/// 7.3.86, through the id it shares with the tanādi vikalpa arm: ten juhotyādi, slice 3e's
/// laṅ prathama and madhyama eka cells and slice 3f's √kit and √dhiṣ ones, whose root guṇa 7.3.86 credits, and thirty-eight curādi, slice 10a's √cur laṅ and vidhiliṅ prathama eka and slice 10g's √div and √śṛdh and slice 10h's sixteen laghu-ik ādhṛṣīya rows' ṇic-branch ones, whose guṇa before ṇic the sanādi 7.3.86 credits; the other seven are the tanādi vikalpa),"""),
("""/// `7.1.35+6.4.116+8.4.56`, 9 `6.4.43` and 1 `6.4.43+8.4.56` (slice 3f3's √jan), 6 `7.3.86+7.1.35` and 6
/// `7.3.86+7.1.35+8.4.56` (slice 10a's √cur, its sanādi 7.3.86 ahead of 7.1.35, and slice 10g's √śṛdh and
/// √div on their ṇic branch), and slice 10f's twenty-one""",
"""/// `7.1.35+6.4.116+8.4.56`, 9 `6.4.43` and 1 `6.4.43+8.4.56` (slice 3f3's √jan), 38 `7.3.86+7.1.35` and 38
/// `7.3.86+7.1.35+8.4.56` (slice 10a's √cur, its sanādi 7.3.86 ahead of 7.1.35, and slice 10g's √śṛdh and
/// √div and slice 10h's sixteen laghu-ik ādhṛṣīya rows on their ṇic branch), and slice 10f's twenty-one"""),
("""/// `7.3.86+8.4.56`, `7.3.86+7.1.35` and `7.3.86+7.1.35+8.4.56` — √kṛ (slice 8b) adds six more""",
"""/// `7.3.86+8.4.56`, `7.3.86+7.1.35` and `7.3.86+7.1.35+8.4.56`. Slice 10h
/// opens twelve keys — 1404 `10.0498` and 612 `10.0498+7.3.86` (a ṇic-less
/// form beside the ṇic one, in parasmaipada and in a `NicUbhayapada` row's
/// ātmanepada; the second key's 7.3.86 is the MANDATORY guṇa of the sixteen
/// laghu-ik rows), 68 apiece `10.0498+8.4.56`, `10.0498+7.1.35` and
/// `10.0498+7.1.35+8.4.56`, 32 apiece `10.0498+7.3.86+8.4.56`,
/// `10.0498+7.1.35+7.3.86` and `10.0498+7.1.35+7.3.86+8.4.56`, 144
/// `7.3.37.2` and 4 apiece `7.3.37.2+8.4.56`, `7.3.37.2+7.1.35` and
/// `7.3.37.2+7.1.35+8.4.56` (√dhū's and √prī's nuk branch) — and folds 68
/// rows apiece into `8.4.56`, `7.1.35` and `7.1.35+8.4.56` and 32 apiece into
/// `7.3.86+8.4.56`, `7.3.86+7.1.35` and `7.3.86+7.1.35+8.4.56` — √kṛ (slice 8b) adds six more"""),
# the 10h paragraph
("""/// beside it. 4248 new cells, 2832 new rows. The gaṇa is OPEN at 208 of its
/// 509 rows.
/// This test is what keeps the numbers true day to day.""",
"""/// beside it. 4248 new cells, 2832 new rows. The gaṇa is OPEN at 208 of its
/// 509 rows.
///
/// Slice 10h curates fifty of the fifty-one ādhṛṣīya rows (the gaṇasūtra
/// 10.0498 *ā dhṛṣād vā*; `10.0368 za\\da~` waits for upasargas), forty-four
/// `Nic` and six `NicUbhayapada`. The ṇic branch is the pinned form; every
/// parasmaipada cell adds the ṇic-less reading, and so does every
/// ātmanepada cell of a `NicUbhayapada` row, 1.3.72's. √dhū's and √prī's ṇic
/// branch forks again on 7.3.37.2's nuk, three readings per cell. 3600 new
/// cells, 2772 new rows. The gaṇa is OPEN at 258 of its 509 rows.
/// This test is what keeps the numbers true day to day."""),
# the census asserts
("""    assert_eq!(total_cells, 17964, "1996 root×lakāra blocks × 9 cells each");""",
"""    assert_eq!(total_cells, 21564, "2396 root×lakāra blocks × 9 cells each");"""),
("""    assert_eq!(ones, 14488, "one-form cells");
    assert_eq!(twos, 2792, "two-form cells");
    assert_eq!(
        threes, 389,""",
"""    assert_eq!(ones, 16072, "one-form cells");
    assert_eq!(twos, 4472, "two-form cells");
    assert_eq!(
        threes, 525,"""),
("""         10f — the optional-ṇic rows' (2564/2570/2573.x beside 7.1.35/8.4.56)\"""",
"""         10f — the optional-ṇic rows' (2564/2570/2573.x beside 7.1.35/8.4.56); and — new in \\
         slice 10h — √dhū's and √prī's every cell that forks on neither 7.1.35 nor 8.4.56 \\
         (10.0498 and 7.3.37.2 beside the ṇic form)\""""),
("""        fours, 141,""", """        fours, 237,"""),
("""         rows' laṅ and vidhiliṅ parasmaipada prathama eka, 2564/2570 alongside 8.4.56\"""",
"""         rows' laṅ and vidhiliṅ parasmaipada prathama eka, 2564/2570 alongside 8.4.56; and — \\
         new in slice 10h — forty-eight ādhṛṣīya rows' the same cells, 10.0498 alongside 8.4.56\""""),
("""        sixes, 141,""", """        sixes, 241,"""),
("""         parasmaipada prathama and madhyama eka (2564/2570 beside 7.1.35/8.4.56)\"""",
"""         parasmaipada prathama and madhyama eka (2564/2570 beside 7.1.35/8.4.56); and — new in \\
         slice 10h — forty-eight ādhṛṣīya rows' the same cells (10.0498 beside 7.1.35/8.4.56), \\
         and √dhū's and √prī's laṅ and vidhiliṅ parasmaipada prathama eka (10.0498 and \\
         7.3.37.2 beside 8.4.56)\""""),
("""    assert_eq!(
        nines, 2,
        "nine-form cells — new in slice 10f, the engine's record: pata's loṭ parasmaipada \\
         prathama and madhyama eka, three readings (2573.1 ṇic-less, 2573.2 pAta-, 6.4.48 \\
         pata-) × the tātaṅ triple (7.1.35/8.4.56)"
    );""",
"""    assert_eq!(
        nines, 6,
        "nine-form cells — new in slice 10f, the engine's record: pata's loṭ parasmaipada \\
         prathama and madhyama eka, three readings (2573.1 ṇic-less, 2573.2 pAta-, 6.4.48 \\
         pata-) × the tātaṅ triple (7.1.35/8.4.56); and — new in slice 10h — √dhū's and \\
         √prī's, three readings (10.0498 ṇic-less, 7.3.37.2's nuk, 7.2.115's vṛddhi) × the \\
         same triple"
    );"""),
("""    assert_eq!(ALTERNATES.len(), 4760, "ALTERNATES row count");""",
"""    assert_eq!(ALTERNATES.len(), 7532, "ALTERNATES row count");"""),
("""    assert_eq!(key_count("8.4.56"), 468, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 460, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 460, "7.1.35+8.4.56 alternates");""",
"""    assert_eq!(key_count("8.4.56"), 536, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 528, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 528, "7.1.35+8.4.56 alternates");"""),
("""    assert_eq!(key_count("7.3.86+8.4.56"), 23, "7.3.86+8.4.56 alternates");""",
"""    assert_eq!(key_count("7.3.86+8.4.56"), 55, "7.3.86+8.4.56 alternates");"""),
("""    assert_eq!(key_count("7.3.86+7.1.35"), 6, "7.3.86+7.1.35 alternates");
    assert_eq!(
        key_count("7.3.86+7.1.35+8.4.56"),
        6,""",
"""    assert_eq!(key_count("7.3.86+7.1.35"), 38, "7.3.86+7.1.35 alternates");
    assert_eq!(
        key_count("7.3.86+7.1.35+8.4.56"),
        38,"""),
("""        ("2573.2+7.1.35+8.4.56", 2),
    ] {
        assert_eq!(key_count(key), n, "{key} alternates");
    }
}""",
"""        ("2573.2+7.1.35+8.4.56", 2),
    ] {
        assert_eq!(key_count(key), n, "{key} alternates");
    }
    // Slice 10h's two vikalpa ids, alone and stacked: the gaṇasūtra 10.0498's
    // ṇic-less branch (with the mandatory 7.3.86 on the sixteen laghu-ik
    // rows) and 7.3.37.2's nuk on √dhū's and √prī's ṇic branch.
    for (key, n) in [
        ("10.0498", 1404),
        ("10.0498+7.3.86", 612),
        ("10.0498+8.4.56", 68),
        ("10.0498+7.1.35", 68),
        ("10.0498+7.1.35+8.4.56", 68),
        ("10.0498+7.3.86+8.4.56", 32),
        ("10.0498+7.1.35+7.3.86", 32),
        ("10.0498+7.1.35+7.3.86+8.4.56", 32),
        ("7.3.37.2", 144),
        ("7.3.37.2+8.4.56", 4),
        ("7.3.37.2+7.1.35", 4),
        ("7.3.37.2+7.1.35+8.4.56", 4),
    ] {
        assert_eq!(key_count(key), n, "{key} alternates");
    }
}"""),
# the ākusmīya analyzer test: 10.0381 mAna~ shares mAnayate and amAnayata
("""/// `mAnayate` and `amAnayata` are the one pair two rows share (`10.0233
/// mAna~` unchanged before ṇic, `10.0234 mana~` by 7.2.116): exactly two
/// analyses, one per root, and only √man's credits 7.2.116. The
/// parasmaipada shapes derive nothing. The single-analysis and Invalid
/// assertions depend on the homograph partners being uncurated — `10.0189`,
/// `10.0438`, `10.0041`, `10.0006`, `10.0034`, `10.0381` — so a slice that
/// curates one must revisit this test (e.g. `mAnayati` becomes Valid once
/// `10.0381` is curated, `kuwwayatu` once `10.0034` is).""",
"""/// `mAnayate` and `amAnayata` are the one pair two ākusmīya rows share
/// (`10.0233 mAna~` unchanged before ṇic, `10.0234 mana~` by 7.2.116): one
/// analysis per root opening with 10.0496, and only √man's credits 7.2.116.
/// Slice 10h's ādhṛṣīya `10.0381 mAna~` shares them too, by 1.3.74 on its ṇic
/// branch — and `amAnayata` twice over, its parasmaipada madhyama bahu
/// as well — so `mAnayate` has three analyses and `amAnayata` four. The
/// parasmaipada shapes derive nothing. The single-analysis and Invalid
/// assertions depend on the homograph partners being uncurated — `10.0189`,
/// `10.0438`, `10.0041`, `10.0006`, `10.0034` — so a slice that curates one
/// must revisit this test (e.g. `kuwwayatu` becomes Valid once `10.0034`
/// is). `mAnayati` became Valid when slice 10h curated `10.0381`."""),
("""    for form in ["mAnayate", "amAnayata"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        let mut roots: Vec<&str> = r.analyses.iter().map(|a| a.dhatu.as_str()).collect();
        roots.sort_unstable();
        assert_eq!(roots, ["mAn", "man"], "{form}");
        for a in &r.analyses {
            assert_eq!(a.pada, Pada::Atmanepada, "{form}");
            let ids = ids_of(a);
            assert_eq!(ids[0], "10.0496", "{form}: {ids:?}");
            let lengthened = ids.iter().any(|i| i == "7.2.116");
            assert_eq!(lengthened, a.dhatu == "man", "{form} {}: {ids:?}", a.dhatu);
        }
    }
    for form in ["dAsayati", "vedayati", "mAnayati", "kuwwayatu"] {""",
"""    // (form, `10.0381 mAna~`'s padas for it)
    for (form, adhrsiya_padas) in [
        ("mAnayate", &[Pada::Atmanepada][..]),
        ("amAnayata", &[Pada::Atmanepada, Pada::Parasmaipada][..]),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), 2 + adhrsiya_padas.len(), "{form}");
        let mut akusmiya: Vec<&str> = Vec::new();
        let mut adhrsiya: Vec<Pada> = Vec::new();
        for a in &r.analyses {
            let ids = ids_of(a);
            if ids[0] == "10.0496" {
                assert_eq!(a.pada, Pada::Atmanepada, "{form}");
                let lengthened = ids.iter().any(|i| i == "7.2.116");
                assert_eq!(lengthened, a.dhatu == "man", "{form} {}: {ids:?}", a.dhatu);
                akusmiya.push(a.dhatu.as_str());
            } else {
                // `10.0381 mAna~`'s ṇic branch: 1.3.74 in ātmanepada, 1.3.78
                // in parasmaipada.
                assert_eq!(a.dhatu, "mAn", "{form}");
                assert!(ids.iter().any(|i| i == "3.1.25"), "{form}: {ids:?}");
                let sanction = match a.pada {
                    Pada::Atmanepada => "1.3.74",
                    Pada::Parasmaipada => "1.3.78",
                };
                assert!(ids.iter().any(|i| i == sanction), "{form}: {ids:?}");
                adhrsiya.push(a.pada);
            }
        }
        akusmiya.sort_unstable();
        assert_eq!(akusmiya, ["mAn", "man"], "{form}");
        assert_eq!(adhrsiya.len(), adhrsiya_padas.len(), "{form}");
        for pada in adhrsiya_padas {
            assert!(adhrsiya.contains(pada), "{form} {pada:?}");
        }
    }
    for form in ["dAsayati", "vedayati", "kuwwayatu"] {"""),
# the adanta analyzer test: 10.0384 mArg shares mArgayARi
("""/// grepped for every witness first: the single-analysis forms are their own
/// row's alone, and each homograph surface is exactly its two rows'.""",
"""/// grepped for every witness first: the single-analysis forms are their own
/// row's alone, and each homograph surface is exactly its two rows'.
/// `mArgayARi`, the adanta 8.4.2 witness, became a pair in slice 10h: the
/// ādhṛṣīya `10.0384 mArga~` is `mArg` before ṇic, the adanta `mArga` after
/// 6.4.48."""),
("""        ("mArgayARi", "mArga", Pada::Parasmaipada, Some("8.4.2")),
""", ""),
("""        ("vizkayate", "vizka", "vizk", "10.0496"),
    ] {""",
"""        ("vizkayate", "vizka", "vizk", "10.0496"),
        ("mArgayARi", "mArga", "mArg", "8.4.2"),
    ] {"""),
]
bad = []
for old, new in E:
    n = s.count(old)
    if n != 1:
        bad.append(f"{n}x {old[:80]!r}")
        continue
    s = s.replace(old, new)
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
open(p, 'w').write(s)
print(f"applied {len(E)} edits")
```

```bash
python3 /tmp/vidyut-full/slice10h/paradigm_10h.py      # applied 32 edits
```

What it changes, so a reviewer can check it against the spec and the goldens:
- **`VIKALPA_RULES`** gains 10.0498 and 7.3.37.2. The stale "fourth to seventh" ordinal goes.
- **The census:** 21564 cells (2396 blocks). Buckets are 16072 / 4472 / 525 / 237 / 10 / 241 / 1 / — / 6:
  - each `Nic` row other than √dhū's and √prī's adds 36 one-form ātmanepada cells, 32 two-form parasmaipada cells, two four-form cells (laṅ and vidhiliṅ prathama eka) and two six-form cells (loṭ prathama and madhyama eka);
  - the four other `NicUbhayapada` rows' ātmanepada adds 144 two-form cells;
  - √dhū and √prī add 68 three-form cells each, two six-form cells (laṅ and vidhiliṅ prathama eka, three readings × 8.4.56) and two nine-form cells (loṭ prathama and madhyama eka, three readings × the tātaṅ triple).
- **`ALTERNATES`:** 7532 rows.
  - `8.4.56` / `7.1.35` / `7.1.35+8.4.56` reach 536 / 528 / 528: 68 each from the thirty-four ṇic branches without a sanādi guṇa.
  - `7.3.86+8.4.56` reaches 55, and `7.3.86+7.1.35` / `7.3.86+7.1.35+8.4.56` reach 38: 32 each from the sixteen laghu-ik rows' ṇic branch.
  - Twelve new keys: `10.0498` 1404, `10.0498+7.3.86` 612, `10.0498+{8.4.56, 7.1.35, 7.1.35+8.4.56}` 68 each, `10.0498+{7.3.86+8.4.56, 7.1.35+7.3.86, 7.1.35+7.3.86+8.4.56}` 32 each, `7.3.37.2` 144, and `7.3.37.2+{8.4.56, 7.1.35, 7.1.35+8.4.56}` 4 each.
- **The doc** of `every_alternate_names_the_vikalpa_rules_that_produced_it`: forty-eight of the 55 `7.3.86+8.4.56` keys are the mandatory firing. It had also gone stale on the `7.3.86+7.1.35` count since 10g.
- **The 10h paragraph.**
- **`curadi_analyses_its_bulk_akusmiya_forms`.** `10.0381 mAna~` now shares `mAnayate` (three analyses) and `amAnayata` (four: its parasmaipada madhyama bahu too). An analysis that does not open with 10.0496 must be √mān's ṇic branch (3.1.25, then 1.3.74 or 1.3.78). `mAnayati` leaves the Invalid list, as the test's own doc foresaw.
- **`curadi_analyses_its_adanta_forms`.** `mArgayARi` moves to the homograph pairs (`mArga` / `mArg`, twin id 8.4.2).

The pada-ambiguous set itself is pinned in Step 6, from the measured failure.

- [ ] **Step 2: The trace assertions and tests (failing)**

Create `/tmp/vidyut-full/slice10h/trace_10h.py` if it is missing (sha256 `f2f9cd2be7ddab8816d6feb8678d8b6a9713fcabd5f27a85df1fd4406db7b46f`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10h's edits to crates/panini/tests/trace/{curadi,juhotyadi}.rs.
Every `old` must occur exactly once; nothing is written if one fails."""
import sys
CURADI = 'crates/panini/tests/trace/curadi.rs'
JUHO = 'crates/panini/tests/trace/juhotyadi.rs'
ADHRSIYA = ["10.0338", "10.0339", "10.0340", "10.0341", "10.0342", "10.0343", "10.0344",
            "10.0345", "10.0346", "10.0347", "10.0348", "10.0349", "10.0350", "10.0351",
            "10.0352", "10.0353", "10.0354", "10.0355", "10.0356", "10.0357", "10.0358",
            "10.0359", "10.0360", "10.0361", "10.0362", "10.0363", "10.0364", "10.0365",
            "10.0366", "10.0367", "10.0369", "10.0370", "10.0371", "10.0372", "10.0373",
            "10.0374", "10.0375", "10.0376", "10.0377", "10.0378", "10.0379", "10.0380",
            "10.0381", "10.0382", "10.0383", "10.0384", "10.0385", "10.0386", "10.0387",
            "10.0388"]
assert len(ADHRSIYA) == 50
NASAL = ["10.0362", "10.0366", "10.0369", "10.0374", "10.0375", "10.0385"]
CH = ["10.0352", "10.0354", "10.0370"]
def q(xs):
    return ", ".join(f'"{x}"' for x in xs)
NEW_TESTS = r'''

/// The rows whose live branches credit `sutra` — in the sanādi stage only
/// (before 3.1.32 folds ṇic in) when `sanadi` is set — sorted, each once.
fn rows_crediting(sutra: &str, sanadi: bool) -> Vec<&'static str> {
    let mut rows = Vec::new();
    for d in dhatus() {
        for lakara in [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin] {
            for &pada in d.padas() {
                for purusha in [Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama] {
                    for vacana in [Vacana::Eka, Vacana::Dvi, Vacana::Bahu] {
                        for p in derive(d, lakara, pada, purusha, vacana) {
                            if p.blocked {
                                continue;
                            }
                            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
                            let end = if sanadi {
                                ids.iter().position(|s| *s == "3.1.32").unwrap_or(0)
                            } else {
                                ids.len()
                            };
                            if ids[..end].contains(&sutra) {
                                rows.push(d.dhatupatha);
                            }
                        }
                    }
                }
            }
        }
    }
    rows.sort_unstable();
    rows.dedup();
    rows
}

#[test]
fn the_10h_vrddhi_and_nuk_rules_fire_only_on_their_rows() {
    // Goldens ignore traces, so this is what holds slice 10h's rules to
    // their rows across the corpus: 7.2.115 on exactly the eight vowel-final
    // ādhṛṣīya rows, the sanādi 6.1.78 on the six whose vṛddhi is an ec
    // (√vṛ and √jṝ reach `Ar` directly), 7.3.37.2 on √dhū and √prī, and
    // 7.2.114 on √mṛj, before ṇic and on its ṇic-less branch alike.
    assert_eq!(
        rows_crediting("7.2.115", false),
        ["10.0343", "10.0345", "10.0346", "10.0347", "10.0361", "10.0372", "10.0373", "10.0382"]
    );
    assert_eq!(
        rows_crediting("6.1.78", true),
        ["10.0343", "10.0347", "10.0361", "10.0372", "10.0373", "10.0382"]
    );
    assert_eq!(rows_crediting("7.3.37.2", false), ["10.0372", "10.0373"]);
    assert_eq!(rows_crediting("7.2.114", true), ["10.0386"]);
    assert_eq!(rows_crediting("7.2.114", false), ["10.0386"]);
}

#[test]
#[allow(non_snake_case)]
fn DUnayati_and_DAvayati_take_nuk_or_vrddhi_never_both() {
    // DU P laT P.E.: three live branches. The ṇic branch takes either the
    // vārttika's nuk or 7.2.115's vṛddhi (then 6.1.78) before 3.1.32, never
    // both; the ṇic-less branch opens with 10.0498 and takes neither (it has
    // no 3.1.32, so its 6.1.78 here is the guṇa stage's: Do + a → Dav).
    let d = dhatus().iter().find(|d| d.dhatupatha == "10.0372").unwrap();
    let mut got: Vec<(String, Vec<String>)> = Vec::new();
    for p in derive(
        d,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    ) {
        assert!(!p.blocked);
        let t: Vec<String> = p.log.iter().map(|s| s.sutra.clone()).collect();
        let sanadi = t
            .iter()
            .take_while(|s| *s != "3.1.32")
            .filter(|s| ["10.0498", "7.3.37.2", "7.2.115", "6.1.78"].contains(&s.as_str()))
            .cloned()
            .collect();
        got.push((p.text(), sanadi));
    }
    got.sort_unstable();
    let ids = |xs: &[&str]| xs.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    assert_eq!(
        got,
        [
            ("DAvayati".to_string(), ids(&["7.2.115", "6.1.78"])),
            ("DUnayati".to_string(), ids(&["7.3.37.2"])),
            ("Davati".to_string(), ids(&["10.0498", "6.1.78"])),
        ]
    );
}'''
EDITS = {CURADI: [
("""//! has a ṇic-less branch besides, opening with its Kaumudī id (2564, 2570,
//! 2573.1, 2573.3) and running the bhvādi path with 1.3.78.""",
"""//! has a ṇic-less branch besides, opening with the id that makes it so
//! (10.0498, 2564, 2570, 2573.1, 2573.3) and running the bhvādi path with
//! 1.3.78 — or, in a `NicUbhayapada` row's ātmanepada, 1.3.72. A
//! vowel-final ādhṛṣīya root's ṇic branch has 7.2.115 (and 6.1.78) before
//! 3.1.32, √dhū's and √prī's the vārttika 7.3.37.2 instead on a second
//! branch, and √mṛj's 7.2.114."""),
("""    // credit's number lies in the positional `AKUSMIYA` range. And 1.3.74
    // never reaches them: its credits stay on the 155 `Nic` rows, read from
    // the curated `pada` column (ten before slice 10e, listed literally until
    // then).""",
"""    // credit's number lies in the positional `AKUSMIYA` range. And 1.3.74
    // never reaches them: its credits stay on the 199 `Nic` rows and the six
    // `NicUbhayapada` rows' ṇic branch, read from the curated `pada` column
    // (ten before slice 10e, listed literally until then)."""),
("""        .filter(|d| d.gana == Gana::Curadi && d.pada == PadaAssignment::Nic)
        .map(|d| d.dhatupatha)
        .collect();
    assert_eq!(nic.len(), 155, "curated 1.3.74 rows");
    for (number, _) in credited("1.3.74") {
        assert!(nic.contains(&number), "1.3.74 credited on {number}");
    }""",
"""        .filter(|d| d.gana == Gana::Curadi && d.pada == PadaAssignment::Nic)
        .map(|d| d.dhatupatha)
        .collect();
    assert_eq!(nic.len(), 199, "curated 1.3.74 rows");
    let nic_ubhayapada: Vec<&str> = dhatus()
        .iter()
        .filter(|d| d.pada == PadaAssignment::NicUbhayapada)
        .map(|d| d.dhatupatha)
        .collect();
    assert_eq!(nic_ubhayapada.len(), 6, "curated 1.3.74-and-1.3.72 rows");
    for (number, _) in credited("1.3.74") {
        assert!(
            nic.contains(&number) || nic_ubhayapada.contains(&number),
            "1.3.74 credited on {number}"
        );
    }"""),
("""    // Each Kaumudī id fires only on the rows `OPTIONAL_NIC` gives it, and on
    // every cell of each: at least one ṇic-less branch per cell in each pada (live in
    // parasmaipada, blocked by 1.3.78 in ātmanepada). 2573.2 fires only on
    // `pata`. Goldens ignore traces, so this is also what holds all five
    // inert on every root outside `OPTIONAL_NIC`. The 2564 and 2570 rows are
    // the 10f and 10g specs' row tables, listed literally.
    for (id, rows) in [""",
"""    // Each optional-ṇic id fires only on the rows `OPTIONAL_NIC` gives it,
    // and on every cell of each: at least one ṇic-less branch per cell in
    // each pada (live in parasmaipada, blocked by 1.3.78 in ātmanepada but
    // for a `NicUbhayapada` row's, which 1.3.72 sanctions). 2573.2 fires only
    // on `pata`. Goldens ignore traces, so this is also what holds all six
    // inert on every root outside `OPTIONAL_NIC`. The 10.0498, 2564 and 2570
    // rows are the 10f, 10g and 10h specs' row tables, listed literally.
    for (id, rows) in [
        (
            "10.0498",
            &[""" + q(ADHRSIYA) + """][..],
        ),"""),
("""                        for p in &nicless {
                            assert_eq!(p.blocked, pada == Pada::Atmanepada, "{cell}");
                        }""",
"""                        let blocks = pada == Pada::Atmanepada
                            && d.pada != PadaAssignment::NicUbhayapada;
                        for p in &nicless {
                            assert_eq!(p.blocked, blocks, "{cell}");
                        }"""),
("""    // On a ṇic-less branch the root's ṇic-branch pada tag is gone: 10.0496,
    // 10.0497 and 1.3.74 never fire, 3.1.25 never adds ṇic, and the live
    // branches are 1.3.78's. On the ṇic branch of an ākusmīya or ā-garvīya
    // row, parasmaipada is blocked before anything is recorded — the ṇic
    // branch is the declined one, so not even the Kaumudī id is credited.""",
"""    // On a ṇic-less branch the root's ṇic-branch pada tag is gone: 10.0496,
    // 10.0497 and 1.3.74 never fire, 3.1.25 never adds ṇic, and the live
    // branches are 1.3.78's — or, in a `NicUbhayapada` row's ātmanepada,
    // 1.3.72's. A ṇic branch never credits 1.3.72. On the ṇic branch of an
    // ākusmīya or ā-garvīya row, parasmaipada is blocked before anything is
    // recorded — the ṇic branch is the declined one, so not even the
    // optional-ṇic id is credited."""),
("""                                if !p.blocked {
                                    assert!(ids.contains(&"1.3.78"), "{cell}: {ids:?}");
                                }
                            } else if p.blocked {""",
"""                                if !p.blocked {
                                    let sanction = if d.pada == PadaAssignment::NicUbhayapada
                                        && pada == Pada::Atmanepada
                                    {
                                        "1.3.72"
                                    } else {
                                        "1.3.78"
                                    };
                                    assert!(ids.contains(&sanction), "{cell}: {ids:?}");
                                }
                                continue;
                            }
                            assert!(!ids.contains(&"1.3.72"), "{cell}: {ids:?}");
                            if p.blocked {"""),
], JUHO: [
("""    // `mantr`, `vanc`) on both their branches, and since slice 10g the
    // fifty-three idit rows (num stored) and `10.0266 anc`, on every live
    // branch of both.
    const CURADI: [&str; 67] = [""",
"""    // `mantr`, `vanc`) on both their branches, and since slice 10g the
    // fifty-three idit rows (num stored) and `10.0266 anc`, on every live
    // branch of both, and since slice 10h six ādhṛṣīya rows with an `n`
    // before a jhal (`granT` twice, `hins`, `SunD`, `SranT`, `kanW`), on
    // every live branch of both.
    const CURADI: [&str; 73] = ["""),
("""        "10.0464", "10.0465", "10.0266",
    ];""",
"""        "10.0464", "10.0465", "10.0266", """ + q(NASAL) + """,
    ];"""),
("""    // ṇic ātmanepada), and slice 10g's fifty-four 120 each (every live
    // branch: 84 parasmaipada, ṇic and ṇic-less, and 36 ṇic ātmanepada).
    assert_eq!(curadi.len(), 36 + 7 * 78 + 5 * 78 + 54 * 120);""",
"""    // ṇic ātmanepada), and slice 10g's fifty-four and slice 10h's six 120
    // each (every live branch: 84 parasmaipada, ṇic and ṇic-less, and 36 ṇic
    // ātmanepada).
    assert_eq!(curadi.len(), 36 + 7 * 78 + 5 * 78 + 60 * 120);"""),
("""    // ch-initial optional-ṇic roots (`Cand`, `Canj`, `Canp`), on both branches
    // (acCandayat, acCandat).""",
"""    // ch-initial optional-ṇic roots (`Cand`, `Canj`, `Canp`), on both branches
    // (acCandayat, acCandat), and slice 10h the three ch-initial ādhṛṣīya
    // roots (`Cfd`, `Cfp`, `Cad`) the same way (acCardayat, acCardat)."""),
("""                "07.0003", "07.0008", "10.0469", "10.0480", "10.0481", "10.0062", "10.0114",
                "10.0171"
            ]""",
"""                "07.0003", "07.0008", "10.0469", "10.0480", "10.0481", "10.0062", "10.0114",
                "10.0171", """ + q(CH) + """
            ]"""),
("""    // 9 parasmaipada ones twice (ṇic and ṇic-less) plus their two 8.4.56 forks.
    assert_eq!(curadi, 3 * 19 + 3 * 29);""",
"""    // 9 parasmaipada ones twice (ṇic and ṇic-less) plus their two 8.4.56 forks,
    // slice 10g's three and slice 10h's three alike.
    assert_eq!(curadi, 3 * 19 + 6 * 29);"""),
]}
out, bad = {}, []
for path, edits in EDITS.items():
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
out[CURADI] = out[CURADI].rstrip('\n') + NEW_TESTS + '\n'
for path, s in out.items():
    open(path, 'w').write(s)
print(f"applied {sum(len(e) for e in EDITS.values())} edits and three items")
```

```bash
python3 /tmp/vidyut-full/slice10h/trace_10h.py      # applied 13 edits and three items
```

It moves four rosters, each from the spec's row table and measured on the prototype:
- `a_kusmad_is_credited_on_exactly_the_akusmiya_cells`: 1.3.74's `Nic` rows go 155 → 199, with the six `NicUbhayapada` rows (1.3.74's on their ṇic branch) asserted beside them.
- `the_optional_nic_ids_are_credited_only_on_their_rows`: 10.0498's fifty rows, listed literally. A `NicUbhayapada` row's ṇic-less ātmanepada is live, not blocked.
- `no_nic_pada_rule_reaches_a_nicless_branch`: such a branch credits 1.3.72, not 1.3.78, and no ṇic branch credits 1.3.72.
- `nas_capadantasya_is_credited_only_on_rudhadi_dhan_jan_and_curadi_roots`: 8.3.24 gains `granT` (both rows), `hins`, `SunD`, `SranT` and `kanW`, 120 live branches each.
- `shcutva_off_jan_is_credited_exactly_as_before_3f3`: 8.4.40 gains `Cfd`, `Cfp` and `Cad`, 29 each, as for 10g's three ch-initial rows.

The spec names only `granT` and `Cfd`; the measurement found six and three rows. It also updates the module doc and adds:
- `rows_crediting(sutra, sanadi)`, a helper;
- `the_10h_vrddhi_and_nuk_rules_fire_only_on_their_rows`;
- `DUnayati_and_DAvayati_take_nuk_or_vrddhi_never_both`.

- [ ] **Step 3: The data-layer assertions and docs (failing)**

Create `/tmp/vidyut-full/slice10h/data_10h.py` if it is missing (sha256 `f362410cd3054246af78620e26beb70267e68e14a6464fc0420973820f3e87e8`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10h's edits to crates/panini-data/src/lib.rs (tests and
docs, not the rows or the table). Every `old` must occur exactly once;
nothing is written if one fails. Run from the worktree root."""
import sys
p = 'crates/panini-data/src/lib.rs'
s = open(p).read()
E = [
# OPTIONAL_NIC's doc: five ids, 10.0498 first
("""/// The curādi rows whose ṇic is optional, each with the Kaumudī id that
/// makes it so: 2564 for an idit root, 2570 for a ñit or udit root, 2573.1
/// for `pata`, and 2573.3 for the roots that rule names (`mUtra`, `katra`,
/// `garva`). Keyed by dhātupāṭha number, as `JNAPADI` is: the engine never""",
"""/// The curādi rows whose ṇic is optional, each with the id of the rule that
/// makes it so: the dhātupāṭha gaṇasūtra 10.0498 *ā dhṛṣād vā* for an
/// ādhṛṣīya row (`10.0338`–`10.0388`), which vidyut-prakriya decides before
/// any marker, and otherwise the Kaumudī's 2564 for an idit root, 2570 for a
/// ñit or udit root, 2573.1 for `pata`, and 2573.3 for the roots that rule
/// names (`mUtra`, `katra`, `garva`). Keyed by dhātupāṭha number, as
/// `JNAPADI` is: the engine never"""),
# Dhatu::pada doc census
("""    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 311
    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and
    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed
    /// exception, 155 curādi rows' are 1.3.74's, 43 ākusmīya rows'
    /// the gaṇasūtra 10.0496's and ten ā-garvīya rows' the gaṇasūtra
    /// 10.0497's, each asserted explicitly from both sides, the same way
    /// `dhatupatha_numbers_resolve_upstream` holds `code` to upstream. For
    /// the sixty-nine curādi rows whose ṇic is optional this is the ṇic branch's
    /// pada; the test also re-derives their ṇic-less branch's, 1.3.78's.""",
"""    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 361
    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and
    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed
    /// exception, 199 curādi rows' are 1.3.74's, six more 1.3.74's with ṇic
    /// and 1.3.72's without (`NicUbhayapada`), 43 ākusmīya rows'
    /// the gaṇasūtra 10.0496's and ten ā-garvīya rows' the gaṇasūtra
    /// 10.0497's, each asserted explicitly from both sides, the same way
    /// `dhatupatha_numbers_resolve_upstream` holds `code` to upstream. For
    /// the 119 curādi rows whose ṇic is optional this is the ṇic branch's
    /// pada; the test also re-derives their ṇic-less branch's from the
    /// upadeśa: 1.3.72's for the six `NicUbhayapada` rows, 1.3.78's for the
    /// rest."""),
("""    /// The test covers the 311 roots curated here, not the dhātupāṭha's 2259.""",
"""    /// The test covers the 361 roots curated here, not the dhātupāṭha's 2259."""),
# Dhatu::padas doc
("""    /// The padas this root derives. `pada` is the ṇic branch's verdict; a
    /// root whose ṇic is optional (`OPTIONAL_NIC`) also derives its ṇic-less
    /// branch, parasmaipada by 1.3.78 for every row listed there. So an
    /// ākusmīya or ā-garvīya row in the table admits both padas,
    /// parasmaipada first, as `PadaAssignment::padas` orders every
    /// two-pada assignment.""",
"""    /// The padas this root derives. `pada` is the ṇic branch's verdict; a
    /// root whose ṇic is optional (`OPTIONAL_NIC`) also derives its ṇic-less
    /// branch: parasmaipada by 1.3.78, or, for a `NicUbhayapada` row, both
    /// padas by 1.3.72, which that assignment already admits. So an
    /// ākusmīya or ā-garvīya row in the table admits both padas,
    /// parasmaipada first, as `PadaAssignment::padas` orders every
    /// two-pada assignment."""),
# the row count
("""        assert_eq!(dhatus().len(), 311);""", """        assert_eq!(dhatus().len(), 361);"""),
# the curādi row-list test
("""    fn curadi_rows_are_the_two_hundred_eight_curated_roots() {""",
"""    fn curadi_rows_are_the_two_hundred_fifty_eight_curated_roots() {"""),
("""        // ñit/udit (2570) row outside the āsvadīya and ādhṛṣīya but `10.0124 ciY`,
        // all ubhayapadī by 1.3.74 with ṇic. The gaṇa is OPEN at 208 of its
        // 509 dhātupāṭha rows.""",
"""        // ñit/udit (2570) row outside the āsvadīya and ādhṛṣīya but `10.0124 ciY`,
        // all ubhayapadī by 1.3.74 with ṇic. Slice 10h adds fifty of the
        // fifty-one ādhṛṣīya (10.0498, all but `10.0368 za\\da~`), ubhayapadī by
        // 1.3.74 with ṇic; the six svarita or ñit among them are
        // `NicUbhayapada`, ubhayapadī by 1.3.72 without it too. The gaṇa is
        // OPEN at 258 of its 509 dhātupāṭha rows."""),
# the marker helper: range-aware
("""    /// The Kaumudī id that makes a curādi upadeśa's ṇic optional, read from
    /// the upadeśa as vidyut-prakriya reads it: 2564 for an idit root (last
    /// marker `i~`), 2570 for a ñit or udit one (last marker `Y` or `u~`),
    /// 2573.1 for `pata`, and 2573.3 for the three roots the Kaumudī names
    /// there. 2573.3 is a list, not a shape: `Cidra`, `sUtra` and the rest
    /// have a conjunct before their final `a` and take ṇic. Only the triggers
    /// slices 10f and 10g curate; the ādhṛṣīya / āsvadīya gaṇasūtras (10.0498,
    /// 10.0499) and 2565 / 2571 / 2572 are later slices'. vidyut decides
    /// those two gaṇasūtras BEFORE idit or udit, so inside their rows
    /// (`10.0279`–`10.0388`) this reading would be wrong;
    /// `optional_nic_matches_upadesha_markers` keeps the table out of them.
    fn optional_nic_from_upadesha(upadesha: &str) -> Option<&'static str> {
        let u = upadesha.trim_end_matches(['\\\\', '^']);""",
"""    /// The id of the rule that makes curādi row `number`'s ṇic optional, read
    /// as vidyut-prakriya reads it: first the ādhṛṣīya, by position (10.0498,
    /// rows `10.0338`–`10.0388`, before any marker), then the upadeśa: 2564
    /// for an idit root (last marker `i~`), 2570 for a ñit or udit one (last
    /// marker `Y` or `u~`), 2573.1 for `pata`, and 2573.3 for the three roots
    /// the Kaumudī names there. 2573.3 is a list, not a shape: `Cidra`,
    /// `sUtra` and the rest have a conjunct before their final `a` and take
    /// ṇic. Only the triggers slices 10f to 10h curate; the āsvadīya
    /// gaṇasūtra (10.0499) and 2565 / 2571 / 2572 are later slices'. vidyut
    /// decides the āsvadīya BEFORE idit or udit too, so inside its rows
    /// (`10.0279`–`10.0337`) this reading would be wrong;
    /// `optional_nic_matches_upadesha_markers` keeps the table out of them.
    fn optional_nic_from_upadesha(number: &str, upadesha: &str) -> Option<&'static str> {
        if ("10.0338"..="10.0388").contains(&number) {
            return Some("10.0498");
        }
        let u = upadesha.trim_end_matches(['\\\\', '^']);"""),
# the marker test: 119 entries and the narrowed exclusion
("""        for (n, id) in OPTIONAL_NIC {
            assert_eq!(optional_nic_from_upadesha(upadesha(n)), Some(*id), "{n}");""",
"""        for (n, id) in OPTIONAL_NIC {
            assert_eq!(optional_nic_from_upadesha(n, upadesha(n)), Some(*id), "{n}");"""),
("""                optional_nic(d.dhatupatha),
                optional_nic_from_upadesha(upadesha(d.dhatupatha)),""",
"""                optional_nic(d.dhatupatha),
                optional_nic_from_upadesha(d.dhatupatha, upadesha(d.dhatupatha)),"""),
("""        assert_eq!(OPTIONAL_NIC.len(), 69);
        // vidyut's `dhatu_karya.rs` checks the āsvadīya (10.0499, rows
        // 279–337) and ādhṛṣīya (10.0498, rows 338–388) before idit or udit,
        // and the reading above knows neither. No table row may fall in
        // those ranges until a slice teaches it both gaṇasūtras.
        for (n, _) in OPTIONAL_NIC {
            assert!(
                !("10.0279"..="10.0388").contains(n),
                "{n} is āsvadīya or ādhṛṣīya"
            );
        }""",
"""        assert_eq!(OPTIONAL_NIC.len(), 119);
        // vidyut's `dhatu_karya.rs` checks the āsvadīya (10.0499, rows
        // 279–337) before idit or udit too, and the reading above does not
        // know it. No table row may fall in that range until a slice teaches
        // it the gaṇasūtra. Nor may `10.0368 za\\da~`: the reading above makes
        // it 10.0498's, but vidyut derives it with the upasarga ā, which this
        // engine does not model.
        for (n, _) in OPTIONAL_NIC {
            assert!(
                !("10.0279"..="10.0337").contains(n) && *n != "10.0368",
                "{n} is āsvadīya, or the upasarga-bound `10.0368 za\\\\da~`"
            );
        }
        assert_eq!(
            optional_nic_from_upadesha("10.0368", upadesha("10.0368")),
            Some("10.0498")
        );"""),
("""        assert_eq!(optional_nic_from_upadesha("Cidra"), None);""",
"""        assert_eq!(optional_nic_from_upadesha("10.0469", "Cidra"), None);"""),
# uniqueness: the verdict reads the row's number too
("""            // (none) is the case, pinned below. The verdict only tells curādi
            // rows apart (the engine keys `OPTIONAL_NIC` by number there
            // alone), so every other gaṇa compares no verdict.
            let gana_prefix = &d.dhatupatha[..2];
            let verdict = |u: &str| {
                if gana_prefix == "10" {
                    optional_nic_from_upadesha(u)
                } else {
                    None
                }
            };
            let own_verdict = verdict(upadesha);""",
"""            // (none) is the case, pinned below, and so is `10.0367 arha~`
            // (10.0498) beside `10.0257 arha~` (none). The verdict only tells
            // curādi rows apart (the engine keys `OPTIONAL_NIC` by number
            // there alone), so every other gaṇa compares no verdict.
            let gana_prefix = &d.dhatupatha[..2];
            let verdict = |n: &str, u: &str| {
                if gana_prefix == "10" {
                    optional_nic_from_upadesha(n, u)
                } else {
                    None
                }
            };
            let own_verdict = verdict(d.dhatupatha, upadesha);"""),
("""                        && verdict(u) == own_verdict""",
"""                        && verdict(n, u) == own_verdict"""),
("""        assert_eq!(optional_nic_from_upadesha(sran_u), Some("2570"));
        assert_eq!(optional_nic_from_upadesha(sran_a), None);""",
"""        assert_eq!(optional_nic_from_upadesha("10.0174", sran_u), Some("2570"));
        assert_eq!(optional_nic_from_upadesha("10.0063", sran_a), None);"""),
# the pada test: a NicUbhayapada arm
("""            // garvād ātmanepadinaḥ (`AaGarviya`); outside both, the affix's
            // 1.3.74 ṇicaś ca (`Nic`, both padas). Asserted both ways, like √bhuj above, so a
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
                } else if AA_GARVIYA.contains(&d.dhatupatha) {
                    (PadaAssignment::AaGarviya, "ā-garvīya, so 10.0497's")
                } else {
                    (
                        PadaAssignment::Nic,
                        "outside the ākusmīya and the ā-garvīya, so 1.3.74's",
                    )
                };
                assert_eq!(d.pada, want, "{} is curādi and {why}", d.dhatupatha);
                // That is the ṇic branch's pada. A row whose ṇic is optional
                // also derives without it, where its own markers decide by
                // 1.3.12 / 1.3.72 / 1.3.78: parasmaipada for every row listed.
                if optional_nic(d.dhatupatha).is_some() {
                    assert_eq!(
                        derived,
                        PadaAssignment::Parasmaipada,
                        "{} {upadesha}: its ṇic-less branch is 1.3.78's",
                        d.dhatupatha
                    );
                    assert!(d.padas().contains(&Pada::Parasmaipada), "{}", d.dhatupatha);
                }
                continue;
            }""",
"""            // garvād ātmanepadinaḥ (`AaGarviya`); outside both, the affix's
            // 1.3.74 ṇicaś ca (`Nic`, both padas). Asserted both ways, like √bhuj above, so a
            // row on the wrong side of the range boundary fails here. A marker
            // decides only a ṇic-less branch, so a marked curādi row is allowed
            // only where its ṇic is optional and the marker is svarita or ñit:
            // its ṇic-less branch is 1.3.72's (`NicUbhayapada`, slice 10h's six
            // ādhṛṣīya). Any other marked curādi row (`10.0058 zmiN`, ṅit) is a
            // later slice's, and fails the first assertion until that slice
            // decides how 1.3.12 meets ṇic.
            if d.gana == Gana::Curadi {
                let nicless_ubhaya = optional_nic(d.dhatupatha).is_some()
                    && derived == PadaAssignment::Ubhayapada;
                if !nicless_ubhaya {
                    assert_eq!(
                        derived,
                        PadaAssignment::Parasmaipada,
                        "{} {upadesha}: a marked curādi row needs its own pada decision",
                        d.dhatupatha
                    );
                }
                let (want, why) = if AKUSMIYA.contains(&d.dhatupatha) {
                    (PadaAssignment::Akusmiya, "ākusmīya, so 10.0496's")
                } else if AA_GARVIYA.contains(&d.dhatupatha) {
                    (PadaAssignment::AaGarviya, "ā-garvīya, so 10.0497's")
                } else if nicless_ubhaya {
                    (
                        PadaAssignment::NicUbhayapada,
                        "svarita or ñit with optional ṇic, so 1.3.74's and 1.3.72's",
                    )
                } else {
                    (
                        PadaAssignment::Nic,
                        "outside the ākusmīya and the ā-garvīya, so 1.3.74's",
                    )
                };
                assert_eq!(d.pada, want, "{} is curādi and {why}", d.dhatupatha);
                // That is the ṇic branch's pada. A row whose ṇic is optional
                // also derives without it, where its own markers decide by
                // 1.3.12 / 1.3.72 / 1.3.78: ubhayapadī for a `NicUbhayapada`
                // row, parasmaipada for every other row listed.
                if optional_nic(d.dhatupatha).is_some() {
                    let (nicless, by) = if d.pada == PadaAssignment::NicUbhayapada {
                        (PadaAssignment::Ubhayapada, "1.3.72")
                    } else {
                        (PadaAssignment::Parasmaipada, "1.3.78")
                    };
                    assert_eq!(
                        derived, nicless,
                        "{} {upadesha}: its ṇic-less branch is {by}'s",
                        d.dhatupatha
                    );
                    assert!(d.padas().contains(&Pada::Parasmaipada), "{}", d.dhatupatha);
                }
                continue;
            }"""),
]
bad = []
for old, new in E:
    n = s.count(old)
    if n != 1:
        bad.append(f"{n}x {old[:80]!r}")
        continue
    s = s.replace(old, new)
if bad:
    sys.exit("not applied:\n  " + "\n  ".join(bad))
open(p, 'w').write(s)
print(f"applied {len(E)} edits")
```

```bash
python3 /tmp/vidyut-full/slice10h/data_10h.py      # applied 16 edits
```

It updates:
- **`OPTIONAL_NIC`'s doc:** five ids, 10.0498 first.
- **`Dhatu::pada`'s census:** 361 rows; 199 `Nic`; six `NicUbhayapada`; 119 optional-ṇic rows, whose ṇic-less branch is re-derived as 1.3.72's or 1.3.78's.
- **`Dhatu::padas`'s doc.**
- **The row count** (361), and the curādi list test's name (`…_two_hundred_fifty_eight_…`) and comment (258 of 509).
- **`optional_nic_from_upadesha`** now takes the row number and answers `"10.0498"` for `10.0338..=10.0388` before reading markers.
- **The marker test:** 119 entries. The exclusion narrows to `10.0279..=10.0337` plus `10.0368`, whose helper verdict is asserted.
- **`dhatupatha_numbers_resolve_upstream`:** the verdict reads the number. `10.0367` resolves with no new pin (its verdict `10.0498` differs from `10.0257`'s `None`), and the comment names it.
- **`curated_pada_agrees_with_upadesha_markers`:** a marked curādi row passes only as `NicUbhayapada` (optional ṇic, and a `~^`/`Y` upadeśa deriving `Ubhayapada`). The ṇic-less branch is re-derived as 1.3.72's for those rows and 1.3.78's for the rest.

- [ ] **Step 4: The `check()` witnesses (failing)**

Create `/tmp/vidyut-full/slice10h/check_10h.rs` (sha256 `890ea439187906224194faf0227587ad02ef7fa79124506e2befd126131cd276`) and `/tmp/vidyut-full/slice10h/insert_check_10h.py` (sha256 `671c35384b9d743d0dcfcc813834af9b39861aab9e8cf6d3a918cdb1e3449a52`) if they are missing:

```rust
/// Slice 10h's `check()` witnesses, from its spec's tables: one ṇic-less and
/// one ṇic witness per pada assignment and per new rule, plus every homograph
/// the spec names. A ṇic-less analysis opens with 10.0498 and credits no
/// 3.1.25, and its pada sūtra is 1.3.78 — or 1.3.72, in a `NicUbhayapada`
/// row's ātmanepada; a ṇic one credits 3.1.25 and 1.3.74 or 1.3.78, and no
/// 10.0498. The goldens were grepped for every witness first: a single-root
/// witness is its own row's alone, and a homograph witness is exactly its
/// rows', one analysis each (the `cah` / `rah` precedent). `Bavati` and
/// `vadati` are bhvādi's too, and the bhvādi analysis comes first, so
/// `trace_for` keeps answering with it; that order is pinned here. The
/// shapes the slice rules out — no ṇic in a `Nic` row's ātmanepada, a ṇic
/// branch without its vṛddhi, √mṛj's guṇa, and nuk without ṇic — derive
/// nothing.
#[test]
fn curadi_analyses_its_adhrsiya_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    let has = |ids: &[String], id: &str| ids.iter().any(|i| i == id);
    // (form, its ādhṛṣīya roots, pada, ṇic-less?, the ids it must credit)
    for (form, dhatus, pada, nicless, credits) in [
        ("yojati", &["yuj"][..], Pada::Parasmaipada, true, &["1.3.78"][..]),
        ("yojayate", &["yuj"], Pada::Atmanepada, false, &["1.3.74"]),
        ("layati", &["lI"], Pada::Parasmaipada, true, &["1.3.78"]),
        ("lAyayati", &["lI"], Pada::Parasmaipada, false, &["7.2.115", "6.1.78"]),
        ("varate", &["vf"], Pada::Atmanepada, true, &["1.3.72"]),
        ("vArayate", &["vf"], Pada::Atmanepada, false, &["7.2.115", "1.3.74"]),
        ("DUnayati", &["DU"], Pada::Parasmaipada, false, &["7.3.37.2"]),
        ("DAvayati", &["DU"], Pada::Parasmaipada, false, &["7.2.115", "6.1.78"]),
        ("Davate", &["DU"], Pada::Atmanepada, true, &["1.3.72"]),
        ("prIRayate", &["prI"], Pada::Atmanepada, false, &["7.3.37.2", "1.3.74"]),
        ("mArjati", &["mfj"], Pada::Parasmaipada, true, &["7.2.114"]),
        ("mArjayati", &["mfj"], Pada::Parasmaipada, false, &["7.2.114"]),
        ("hiMsati", &["hins"], Pada::Parasmaipada, true, &["8.3.24"]),
        ("kaRWati", &["kanW"], Pada::Parasmaipada, true, &["8.4.58"]),
        ("mAnayati", &["mAn"], Pada::Parasmaipada, false, &["1.3.78"]),
        // Homographs inside the slice: two rows each.
        ("tarpati", &["tfp", "tfp"], Pada::Parasmaipada, true, &["7.3.86"]),
        ("darBati", &["dfB", "dfB"], Pada::Parasmaipada, true, &["7.3.86"]),
        ("granTati", &["granT", "granT"], Pada::Parasmaipada, true, &["8.3.24"]),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), dhatus.len(), "{form}");
        for a in &r.analyses {
            assert!(dhatus.contains(&a.dhatu.as_str()), "{form}: {}", a.dhatu);
            assert_eq!(a.pada, pada, "{form}");
            let ids = ids_of(a);
            if nicless {
                assert_eq!(ids[0], "10.0498", "{form}: {ids:?}");
                assert!(!has(&ids, "3.1.25"), "{form}: {ids:?}");
            } else {
                assert!(has(&ids, "3.1.25"), "{form}: {ids:?}");
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
    assert!(!has(&ids, "7.2.115"), "DUnayati: {ids:?}");
    // Homographs with bhvādi: the bhvādi row's analysis first, then the
    // ādhṛṣīya row's ṇic-less one.
    for form in ["Bavati", "vadati"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), 2, "{form}");
        let first = ids_of(&r.analyses[0]);
        let second = ids_of(&r.analyses[1]);
        assert!(!has(&first, "10.0498"), "{form}: {first:?}");
        assert_eq!(second[0], "10.0498", "{form}: {second:?}");
        assert_eq!(r.analyses[0].dhatu, r.analyses[1].dhatu, "{form}");
    }
    for form in ["yojate", "layayati", "marjati", "marjayati", "DUnati"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
```

```python
#!/usr/bin/env python3
"""THROWAWAY: append check_10h.rs's test, doc comment included, after 10g's
curadi_analyses_its_optional_nic_forms, the last item in the file."""
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
assert s.count('fn curadi_analyses_its_optional_nic_forms()') == 1
assert 'fn curadi_analyses_its_adhrsiya_forms()' not in s
j = s.index('fn curadi_analyses_its_optional_nic_forms()')
assert s.index('\n}\n', j) + 3 == len(s), "10g's test is no longer last"
s = s + '\n' + open('/tmp/vidyut-full/slice10h/check_10h.rs').read().rstrip('\n') + '\n'
open(p, 'w').write(s)
print("check witnesses appended")
```

```bash
python3 /tmp/vidyut-full/slice10h/insert_check_10h.py      # check witnesses appended
mise run fmt
```

It appends `curadi_analyses_its_adhrsiya_forms` after 10g's `curadi_analyses_its_optional_nic_forms`, the last item in the file. Its witnesses come from the spec's tables:
- **Per pada assignment:** `yojati` / `yojayate`, and `varate` / `vArayate` with `Davate`.
- **Per new rule:** `layati` / `lAyayati` (7.2.115, 6.1.78), `DUnayati` / `DAvayati` / `prIRayate` (7.3.37.2 or 7.2.115), `mArjati` / `mArjayati` (7.2.114, no 7.3.86), `hiMsati`, `kaRWati`, and `mAnayati`, which the curation makes Valid.
- **Homographs:** `tarpati`, `darBati` and `granTati` have two rows each. `Bavati` and `vadati` have the bhvādi analysis first and the ādhṛṣīya one second, the order `trace_for` relies on.
- **Invalid:** `yojate`, `layayati`, `marjati`, `marjayati` and `DUnati`.

- [ ] **Step 5: Run them to see them fail**

Run: `mise exec -- cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .*FAILED" | sort`
Expected failures (no row exists yet): `trace` 207 passed / 5 failed, `paradigm` 22 / 4, `panini-data` 25 / 2:

- `curadi::DUnayati_and_DAvayati_take_nuk_or_vrddhi_never_both`
- `curadi::a_kusmad_is_credited_on_exactly_the_akusmiya_cells`
- `curadi::the_10h_vrddhi_and_nuk_rules_fire_only_on_their_rows`
- `curadi_analyses_its_adanta_forms`
- `curadi_analyses_its_adhrsiya_forms`
- `curadi_analyses_its_bulk_akusmiya_forms`
- `derivation_set_shape_matches_the_audited_numbers`
- `juhotyadi::nas_capadantasya_is_credited_only_on_rudhadi_dhan_jan_and_curadi_roots`
- `juhotyadi::shcutva_off_jan_is_credited_exactly_as_before_3f3`
- `tests::curated_roots_have_expected_ganas_and_padas`
- `tests::optional_nic_matches_upadesha_markers`

Several tests still pass. `the_optional_nic_ids_are_credited_only_on_their_rows`, `no_nic_pada_rule_reaches_a_nicless_branch`, `every_alternate_names_the_vikalpa_rules_that_produced_it` and `pada_ambiguous_surfaces_are_exactly_these` hold vacuously or are unchanged. `curated_pada_agrees_with_upadesha_markers` and `dhatupatha_numbers_resolve_upstream` have no new row to read yet.

- [ ] **Step 6: The rows, the goldens, and the measured pada-ambiguous set**

Create `/tmp/vidyut-full/slice10h/gen_rows_10h.py` (sha256 `bd9d36dc0f66b923e0194f72244a96e6631ab7fddd01a6666a7b8b89ada72bf5`) and `/tmp/vidyut-full/slice10h/insert_rows_10h.py` (sha256 `4e745ce6cf1c14653e39fa482c7149316ec5dfcbc0482608956ef34382dae3eb`) if they are missing. The `ROWS` table is the spec's, with each row's laṭ prathama eka forms (ṇic-less, ṇic parasmaipada) for its comment, checked against the generated goldens:

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10h — emit the 50 ādhṛṣīya `Dhatu` rows, the
`curadi_rows_are_…` list entries and the new `OPTIONAL_NIC` entries. Run
from the worktree root; writes rows.rs, rowlist.rs and table.rs into the
directory given."""
import sys, textwrap

OUT = sys.argv[1]
# (number, code, pada, root, ṇic-less laṭ 3sg, ṇic laṭ P 3sg); a
# NicUbhayapada row's ṇic-less form is its parasmaipada / ātmanepada pair.
ROWS = [
    ("10.0338", "yuj", "Nic", "yuj", "yojati", "yojayati"),
    ("10.0339", "pfc", "Nic", "pṛc", "parcati", "parcayati"),
    ("10.0340", "arc", "Nic", "arc", "arcati", "arcayati"),
    ("10.0341", "sah", "Nic", "sah", "sahati", "sAhayati"),
    ("10.0342", "Ir", "Nic", "īr", "Irati", "Irayati"),
    ("10.0343", "lI", "Nic", "lī", "layati", "lAyayati"),
    ("10.0344", "vfj", "Nic", "vṛj", "varjati", "varjayati"),
    ("10.0345", "vf", "NicUbhayapada", "vṛ", "varati / varate", "vArayati"),
    ("10.0346", "jF", "Nic", "jṝ", "jarati", "jArayati"),
    ("10.0347", "jri", "Nic", "jri", "jrayati", "jrAyayati"),
    ("10.0348", "ric", "Nic", "ric", "recati", "recayati"),
    ("10.0349", "Siz", "Nic", "śiṣ", "Sezati", "Sezayati"),
    ("10.0350", "tap", "Nic", "tap", "tapati", "tApayati"),
    ("10.0351", "tfp", "Nic", "tṛp", "tarpati", "tarpayati"),
    ("10.0352", "Cfd", "Nic", "chṛd", "Cardati", "Cardayati"),
    ("10.0353", "cfp", "Nic", "cṛp", "carpati", "carpayati"),
    ("10.0354", "Cfp", "Nic", "chṛp", "Carpati", "Carpayati"),
    ("10.0355", "tfp", "Nic", "tṛp", "tarpati", "tarpayati"),
    ("10.0356", "dfp", "Nic", "dṛp", "darpati", "darpayati"),
    ("10.0357", "dfB", "Nic", "dṛbh", "darBati", "darBayati"),
    ("10.0358", "dfB", "Nic", "dṛbh", "darBati", "darBayati"),
    ("10.0359", "law", "Nic", "laṭ", "lawati", "lAwayati"),
    ("10.0360", "SraT", "Nic", "śrath", "SraTati", "SrATayati"),
    ("10.0361", "mI", "Nic", "mī", "mayati", "mAyayati"),
    ("10.0362", "granT", "Nic", "granth", "granTati", "granTayati"),
    ("10.0363", "SIk", "Nic", "śīk", "SIkati", "SIkayati"),
    ("10.0364", "cIk", "Nic", "cīk", "cIkati", "cIkayati"),
    ("10.0365", "ard", "NicUbhayapada", "ard", "ardati / ardate", "ardayati"),
    ("10.0366", "hins", "Nic", "hiṃs", "hiMsati", "hiMsayati"),
    ("10.0367", "arh", "Nic", "arh", "arhati", "arhayati"),
    ("10.0369", "SunD", "Nic", "śundh", "SunDati", "SunDayati"),
    ("10.0370", "Cad", "Nic", "chad", "Cadati", "CAdayati"),
    ("10.0371", "juz", "Nic", "juṣ", "jozati", "jozayati"),
    ("10.0372", "DU", "NicUbhayapada", "dhū", "Davati / Davate", "DAvayati"),
    ("10.0373", "prI", "NicUbhayapada", "prī", "prayati / prayate", "prAyayati"),
    ("10.0374", "SranT", "Nic", "śranth", "SranTati", "SranTayati"),
    ("10.0375", "granT", "Nic", "granth", "granTati", "granTayati"),
    ("10.0376", "Ap", "Nic", "āp", "Apati", "Apayati"),
    ("10.0377", "tan", "Nic", "tan", "tanati", "tAnayati"),
    ("10.0378", "can", "Nic", "can", "canati", "cAnayati"),
    ("10.0379", "vad", "NicUbhayapada", "vad", "vadati / vadate", "vAdayati"),
    ("10.0380", "vac", "Nic", "vac", "vacati", "vAcayati"),
    ("10.0381", "mAn", "Nic", "mān", "mAnati", "mAnayati"),
    ("10.0382", "BU", "Nic", "bhū", "Bavati", "BAvayati"),
    ("10.0383", "garh", "Nic", "garh", "garhati", "garhayati"),
    ("10.0384", "mArg", "Nic", "mārg", "mArgati", "mArgayati"),
    ("10.0385", "kanW", "Nic", "kaṇṭh", "kaRWati", "kaRWayati"),
    ("10.0386", "mfj", "Nic", "mṛj", "mArjati", "mArjayati"),
    ("10.0387", "mfz", "NicUbhayapada", "mṛṣ", "marzati / marzate", "marzayati"),
    ("10.0388", "Dfz", "Nic", "dhṛṣ", "Darzati", "Darzayati"),
]
assert len(ROWS) == 50
# What a row needs beyond 10f's fork, said in its comment.
NOTE = {
    "10.0343": "7.2.115 *aco ñṇiti* lengthens the final before ṇic.",
    "10.0345": "7.2.115 *aco ñṇiti* lengthens the final before ṇic.",
    "10.0346": "7.2.115 *aco ñṇiti* lengthens the final before ṇic.",
    "10.0347": "7.2.115 *aco ñṇiti* lengthens the final before ṇic.",
    "10.0361": "7.2.115 *aco ñṇiti* lengthens the final before ṇic.",
    "10.0382": "7.2.115 *aco ñṇiti* lengthens the final before ṇic.",
    "10.0366": "Idit (7.1.58's num stored).",
    "10.0385": "Idit (7.1.58's num stored).",
    "10.0372": "Before ṇic, 7.2.115 *aco ñṇiti* or the vārttika 7.3.37.2's nuk (*dhūnayati*).",
    "10.0373": "Before ṇic, 7.2.115 *aco ñṇiti* or the vārttika 7.3.37.2's nuk (*prīṇayati*).",
    "10.0386": "7.2.114 *mṛjer vṛddhiḥ* on both branches.",
}
IAST = {'A': 'ā', 'I': 'ī', 'U': 'ū', 'f': 'ṛ', 'F': 'ṝ', 'x': 'ḷ', 'E': 'ai', 'O': 'au',
        'K': 'kh', 'G': 'gh', 'N': 'ṅ', 'C': 'ch', 'J': 'jh', 'Y': 'ñ', 'w': 'ṭ', 'W': 'ṭh',
        'q': 'ḍ', 'Q': 'ḍh', 'R': 'ṇ', 'T': 'th', 'D': 'dh', 'P': 'ph', 'B': 'bh', 'S': 'ś',
        'z': 'ṣ', 'M': 'ṃ', 'H': 'ḥ'}
def iast(s):
    return ''.join(IAST.get(c, c) for c in s)
def forms(s):
    return ' / '.join(f"*{iast(f)}*" for f in s.split(' / '))
up = {}
for line in open('data/dhatupatha.tsv'):
    f = line.rstrip('\n').split('\t')
    if len(f) >= 3:
        up[f[0]] = (f[1], f[2])
out, lst = [], []
for n, code, pada, root, nicless, nic in ROWS:
    u, artha = up[n]
    if pada == "Nic":
        p = (f"Ubhayapadī by 1.3.74 with ṇic ({forms(nic)}), parasmaipadī by 1.3.78 "
             f"without ({forms(nicless)}).")
    else:
        p = (f"Ubhayapadī by 1.3.74 with ṇic ({forms(nic)}), and by 1.3.72 without, "
             f"its upadeśa being svarita or ñit ({forms(nicless)}).")
    note = f" {NOTE[n]}" if n in NOTE else ""
    c = f"{n} `{u}` {artha} (√{root}). Ādhṛṣīya: ṇic optional by 10.0498. {p}{note} Slice 10h."
    com = '\n'.join('        // ' + l for l in textwrap.wrap(c, 72))
    out.append(f"    Dhatu {{\n{com}\n        dhatupatha: \"{n}\",\n        code: \"{code}\",\n"
               f"        gana: Gana::Curadi,\n        pada: PadaAssignment::{pada},\n"
               f"        artha: \"{artha}\",\n    }},\n")
    lst.append(f'                ("{n}", "{code}", PadaAssignment::{pada}),\n')
table = ''.join(f'    ("{n}", "10.0498"),\n' for n, *_ in ROWS)
open(f'{OUT}/rows.rs', 'w').write(''.join(out))
open(f'{OUT}/rowlist.rs', 'w').write(''.join(lst))
open(f'{OUT}/table.rs', 'w').write(table)
print(f"{len(ROWS)} rows")
```

```python
#!/usr/bin/env python3
"""THROWAWAY: insert gen_rows_10h.py's output ($GEN/rows.rs, rowlist.rs,
table.rs). The rows go after `10.0465 lanj`, the last curādi row; the
table is rewritten in dhātupāṭha order."""
import os, re
GEN = os.environ['GEN']
p = 'crates/panini-data/src/lib.rs'
s = open(p).read()
a = '''        code: "lanj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prakASane",
    },
];'''
assert s.count(a) == 1
s = s.replace(a, a[:-2] + open(f'{GEN}/rows.rs').read() + '];')
b = '''                ("10.0465", "lanj", PadaAssignment::Nic),
'''
assert s.count(b) == 1
s = s.replace(b, b + open(f'{GEN}/rowlist.rs').read())
head = 'pub const OPTIONAL_NIC: &[(&str, &str)] = &[\n'
i = s.index(head) + len(head)
j = s.index('];', i)
entries = s[i:j] + open(f'{GEN}/table.rs').read()
lines = sorted(l for l in entries.splitlines(keepends=True) if l.strip())
assert len(lines) == 119, len(lines)
assert all(re.fullmatch(r'    \("10\.\d{4}", "[0-9.]+"\),\n', l) for l in lines)
s = s[:i] + ''.join(lines) + s[j:]
open(p, 'w').write(s)
print("inserted 50 rows; OPTIONAL_NIC has 119 entries")
```

```bash
GEN="$(mktemp -d)"
python3 /tmp/vidyut-full/slice10h/gen_rows_10h.py "$GEN"      # 50 rows
sha256sum "$GEN/rows.rs" "$GEN/rowlist.rs" "$GEN/table.rs"
GEN="$GEN" python3 /tmp/vidyut-full/slice10h/insert_rows_10h.py      # inserted 50 rows; OPTIONAL_NIC has 119 entries
```

Expected hashes:
- `rows.rs`: `37d9664fa7839376d9d633150c319f9a56d74134c5fd9d9c3744758065c7f947`
- `rowlist.rs`: `cf14c19db09c3d5da09c62b0475e200beabd9fa5da0bd8fc060c5785eed9c35d`
- `table.rs`: `86f8ce322fd1987b2a30fb0faf7eb04ac4427a749c91f66ad77469bab0cb442c`

If any differs, stop and report. The rows go after `10.0465 lanj`, the last curādi row (the data layer orders curādi by slice); the table is rewritten in dhātupāṭha order.

Repoint the vidyut checkout's dev-deps at this worktree. Then create `/tmp/vidyut-full/vidyut-prakriya/examples/curadi_goldens_10h.rs` (sha256 `04512e7489f505bc3363425cff53bc8b46a58d197ba46e74dd2796b28c4dc793`) and `/tmp/vidyut-full/slice10h/insert_goldens_10h.py` (sha256 `a86a93fa7daa3155df3336a54ba9e48409dd7bbc0ebe1c890f2c9fbf04a8513d`) if they are missing:

```rust
//! THROWAWAY: slice 10h — emit the 50 ādhṛṣīya rows' goldens from the
//! engine, asserting every cell's form set equals vidyut's first.
use panini::Panini;
use panini_data::{dhatus, optional_nic, Lakara as L, Pada, Purusha as P, Vacana as V};
use vidyut_prakriya::args::{DhatuPada, Lakara, Prayoga, Purusha, Tinanta, Vacana};
use vidyut_prakriya::{Dhatupatha, Vyakarana};
const VIKALPA_RULES: &[&str] = &[
    "10.0498", "2564", "2570", "2573.1", "2573.3", "2573.2", "7.3.37.2", "7.1.35", "3.4.111", "7.3.86", "6.4.107",
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
    // The new rows: the ādhṛṣīya, 10.0498's `OPTIONAL_NIC` entries.
    for d in dhatus().iter().filter(|d| optional_nic(d.dhatupatha) == Some("10.0498")) {
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
    std::fs::write("/tmp/vidyut-full/goldens_10h_paradigm.rs", par).unwrap();
    std::fs::write("/tmp/vidyut-full/goldens_10h_alternates.rs", alt).unwrap();
    println!("{ncells} cells, {nforms} forms, {ndiff} differences");
    assert_eq!(ndiff, 0);
}
```

```python
#!/usr/bin/env python3
"""THROWAWAY: insert the generator's goldens before each static's `];`."""
p = 'crates/panini/tests/paradigm/data/curadi.rs'
s = open(p).read()
par = open('/tmp/vidyut-full/goldens_10h_paradigm.rs').read()
alt = open('/tmp/vidyut-full/goldens_10h_alternates.rs').read()
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
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example curadi_goldens_10h 2>/dev/null | tail -3)
sha256sum /tmp/vidyut-full/goldens_10h_paradigm.rs /tmp/vidyut-full/goldens_10h_alternates.rs
python3 /tmp/vidyut-full/slice10h/insert_goldens_10h.py      # inserted goldens
mise run fmt
```

Expected:
- the generator prints `3600 cells, 6372 forms, 0 differences`;
- `goldens_10h_paradigm.rs`: `f3f04c669e3035cfd9688d96f9220798dd1e363d33831c093629799acd3efb22` (400 rows);
- `goldens_10h_alternates.rs`: `062cea8e257f3c1a267860b7e54fb8f416c9319f03ce65696317abba360bd229` (2772 rows).

Any `DIFF` line or other hash: stop and report, and edit nothing. Every new row's ṇic branch is live in both padas, so each pinned form is the ṇic one; for √dhū and √prī it is the 7.2.115 branch (`DAvayati`), since 7.3.37.2's declined branch is index 0. Leave the dev-deps pointed at this worktree for Task 4.

Then pin the pada-ambiguous set from the measured failure (`pada_ambiguous_surfaces_are_exactly_these` says it is "measured (never hand-picked)"). Create `/tmp/vidyut-full/slice10h/pin_ambiguous_10h.py` if it is missing (sha256 `f14c902311e1ae602daf4a01b97558494c9369e33ed986532ae5e145b1815de8`):

```python
#!/usr/bin/env python3
"""THROWAWAY: pin the measured pada-ambiguous set ($SET, the failing test's
`left:` JSON) into pada_ambiguous_surfaces_are_exactly_these, and extend
the comment that accounts for it."""
import hashlib, json, os
amb = json.load(open(os.environ['SET']))
assert len(amb) == 839, len(amb)
h = hashlib.sha256('\n'.join(amb).encode()).hexdigest()
assert h == '312b79e625f2957e6d0d2b690aa9f7e30a2f79bf5a4e2213a87b22bd2d7f2826', h
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
old = """    // four are new: 10f's `10.0230 div` has only their ātmanepada half. 223
    // more, taking the set from 432 to 655.
"""
assert s.count(old) == 1
s = s.replace(old, old + """    // Slice 10h's fifty ādhṛṣīya rows contribute the same four each from
    // their ṇic branch — 200 — less the homographs': `tfp`, `dfB` and `granT`
    // each have two rows sharing all four (−12), and `10.0384 mArga~`'s four
    // are `10.0108 mArga`'s already (−4). `10.0381 mAna~`'s four are new:
    // `10.0233 mAna~`, ākusmīya, has only their ātmanepada half. 184 more,
    // taking the set from 655 to 839.
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
SET="$SET" python3 /tmp/vidyut-full/slice10h/pin_ambiguous_10h.py      # pinned 839
mise run fmt
```

If the count or hash assertion fails, stop and report. The script also extends the set's accounting comment. The 184 new surfaces are 50 × 4, less 12 for the three in-slice homograph pairs and 4 for `10.0384 mArga~`'s, which `10.0108 mArga` already holds. They include `amAnayata`, √mān's four being new because ākusmīya `10.0233 mAna~` has only their ātmanepada half.

- [ ] **Step 7: Run the full suite**

```bash
mise run fmt
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS at 21564 cells, with `panini-data` 27, `panini-prakriya` 427, `trace` 212 and `paradigm` 26.

Grep the goldens for the `check()` witnesses, which the test also enforces:

```bash
for f in yojati yojayate layati lAyayati varate vArayate Davate DUnayati DAvayati prIRayate mArjati mArjayati hiMsati kaRWati mAnayati tarpati darBati granTati Bavati vadati mArgayARi mAnayate amAnayata yojate layayati marjati marjayati DUnati; do
  printf "%s: %s\n" $f "$(grep -c "\"$f\"" crates/panini/tests/paradigm/data/*.rs | grep -v ':0' | sed 's#.*/##' | tr '\n' ' ')"; done
```

Expected:
- `tarpati`, `darBati`, `granTati` and `mArgayARi` appear twice, in `curadi.rs` only.
- `Bavati` and `vadati` appear once in `bhvadi.rs` and once in `curadi.rs`.
- `mAnayate` appears three times and `amAnayata` four, in `curadi.rs`.
- Every other Valid witness appears once, in `curadi.rs` only.
- The five Invalid shapes print nothing.

- [ ] **Step 8: Commit**

```bash
mise run lint
git branch --show-current      # curadi-10h
git add -A
git commit -m "feat(data): curādi's fifty ādhṛṣīya rows (10.0498)

17964 → 21564 cells, 22724 → 29096 forms, ALTERNATES 4760 → 7532, 311 → 361
roots; pada-ambiguous surfaces 655 → 839. OPTIONAL_NIC (119) reads the
ādhṛṣīya by position and stays out of the āsvadīya and 10.0368; six
svarita/ñit rows are NicUbhayapada; √arh resolves by its optional-ṇic
verdict; goldens generated cell-by-cell equal to vidyut."
```

---

## Task 4: Audit, prior-trace diff, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`
- Modify: `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`
- Modify: `crates/panini/tests/paradigm/main.rs` (audit prose), `crates/panini-prakriya/src/tinanta/guna.rs` (one comment), `crates/panini-data/src/lib.rs` (two comments)
- Modify: the 10g spec

**Interfaces:**
- Consumes: the finished engine, data and goldens. Produces no symbols.

- [ ] **Step 1: Update the audit harness**

Create `/tmp/vidyut-full/slice10h/audit_10h.py` if it is missing (sha256 `f15cd6f77e7b00a31e35d520d238ac0d095f8f599962c590edbcb26f76a538d3`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10h's edits to tools/audit/panini_full_audit.rs. Every
`old` must occur exactly once; nothing is written if one fails."""
import sys
p = 'tools/audit/panini_full_audit.rs'
s = open(p).read()
E = [
("//! What it compares: for each of the 311 curated roots, for each pada the root\n//! admits (`Dhatu::padas`; two apiece for the 188 roots that admit both padas —\n//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 155 curādi\n//! roots by 1.3.74, and seven",
 "//! What it compares: for each of the 361 curated roots, for each pada the root\n//! admits (`Dhatu::padas`; two apiece for the 238 roots that admit both padas —\n//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 199 curādi\n//! roots by 1.3.74, six more by 1.3.74 with ṇic and 1.3.72 without, and seven"),
("//! Corpus invariants, asserted: 311 roots, 17964 cells, 22724 forms. These are",
 "//! Corpus invariants, asserted: 361 roots, 21564 cells, 29096 forms. These are"),
("//! (`derivation_set_shape_matches_the_audited_numbers`): 1996 root×pada×lakāra\n//! blocks × 9 cells, plus 4760 `ALTERNATES` rows.",
 "//! (`derivation_set_shape_matches_the_audited_numbers`): 2396 root×pada×lakāra\n//! blocks × 9 cells, plus 7532 `ALTERNATES` rows."),
("//! Optionally dump the full 17964-cell table:", "//! Optionally dump the full 21564-cell table:"),
('    assert_eq!(roots_seen.len(), 311, "curated roots");\n    assert_eq!(n_cells, 17964, "cells: 1996 root×pada×lakāra blocks × 9");\n    assert_eq!(n_forms, 22724, "forms: 17964 cells + 4760 ALTERNATES rows");',
 '    assert_eq!(roots_seen.len(), 361, "curated roots");\n    assert_eq!(n_cells, 21564, "cells: 2396 root×pada×lakāra blocks × 9");\n    assert_eq!(n_forms, 29096, "forms: 21564 cells + 7532 ALTERNATES rows");'),
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
python3 /tmp/vidyut-full/slice10h/audit_10h.py      # applied 5 edits
```

The harness now asserts 361 / 21564 / 29096 and names 238 both-pada roots: 188 + 50, with 199 by 1.3.74 and six more by 1.3.74 with ṇic and 1.3.72 without.

- [ ] **Step 2: The prior-trace diff and the audit**

The dev-deps still point at this worktree from Task 3. Create `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_10h.rs` if it is missing (sha256 `a0a7ebf4434e1629b904dd76b8c183ca1fd755507566edc1019e3b34f4a4f320`). It lists 10h's rows literally so that it also builds against main:

```rust
//! THROWAWAY: slice 10h — dump every prior cell's live-branch credited-rule log.
use panini::Panini;
use panini_data::{Lakara as L, Purusha as P, Vacana as V};
/// Slice 10h's fifty rows, listed literally so the dump also builds
/// against main.
const NEW: &[&str] = &["10.0338", "10.0339", "10.0340", "10.0341", "10.0342", "10.0343", "10.0344", "10.0345", "10.0346", "10.0347", "10.0348", "10.0349", "10.0350", "10.0351", "10.0352", "10.0353", "10.0354", "10.0355", "10.0356", "10.0357", "10.0358", "10.0359", "10.0360", "10.0361", "10.0362", "10.0363", "10.0364", "10.0365", "10.0366", "10.0367", "10.0369", "10.0370", "10.0371", "10.0372", "10.0373", "10.0374", "10.0375", "10.0376", "10.0377", "10.0378", "10.0379", "10.0380", "10.0381", "10.0382", "10.0383", "10.0384", "10.0385", "10.0386", "10.0387", "10.0388"];
fn main() {
    let panini = Panini::new();
    for d in panini_data::dhatus() {
        if NEW.contains(&d.dhatupatha) { continue; }
        for pada in d.padas() {
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
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10h 2>/dev/null > "$DUMP/branch.txt")
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml   # must point at /workspace/crates
git -C /workspace branch --show-current   # main
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10h 2>/dev/null > "$DUMP/main.txt")
wc -l < "$DUMP/main.txt"; wc -l < "$DUMP/branch.txt"            # 22724 and 22724
cmp "$DUMP/main.txt" "$DUMP/branch.txt" && echo PRIOR-TRACES-IDENTICAL
```

Expected: `22724`, `22724`, `PRIOR-TRACES-IDENTICAL`. This is the corpus-wide check that 10.0498, `NicUbhayapada`, 7.3.37.2, 7.2.115, the sanādi 6.1.78, both 7.2.114 entries and `vrddhi_of`'s `F` fire on no prior root (goldens ignore traces). `/workspace` must be on `main` (Task 1) for the second dump. If `cmp` reports a difference, stop and report.

Then the audit. Repoint at this worktree again, copy the committed harness (never rewrite it), and run:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
cp tools/audit/panini_full_audit.rs $V/examples/
(cd $V && PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -8)
(cd $V && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -2)
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml
```

- Expected from the honest run: `roots : 361`, `cells : 21564`, `forms (set sizes): 29096`, `live branches : 29096`, `blocked branches : 4320`, `differing cells  : 0`, `AUDIT PASSED: 21564 cells, 29096 forms, zero differences.`
- Expected from the `entry` control: `AUDIT FAILED: 36 differing cells.`

Do not use `mise -C`. If the honest run shows differences, stop and report, and edit nothing.

- [ ] **Step 3: The doc sweep, the audit record and the spec pointer**

Create `/tmp/vidyut-full/slice10h/docsweep_10h.py` if it is missing (sha256 `49fbd4bc1615f9bf6f170b2b8ca49a8aa7c025f5096655ac15e8a8362fef7a08`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10h's doc sweep, run from the worktree root with the
audit date as argv[1]. Every `old` must occur exactly once in its file; the
script checks all of them before writing any file, and writes nothing if
one fails."""
import sys

README = [
("8.4.44 *śāt* exemption. *curādi* (10) is **open** at 208 of its 509",
 "8.4.44 *śāt* exemption. *curādi* (10) is **open** at 258 of its 509"),
("read its *num*: the root's stored `n` reaches ṇatva as the anusvāra 8.3.24\nmade of it, and the anusvāra now intervenes (*kṣampāṇi*).\n",
 "read its *num*: the root's stored `n` reaches ṇatva as the anusvāra 8.3.24\n"
 "made of it, and the anusvāra now intervenes (*kṣampāṇi*). Slice 10h curated\n"
 "fifty of the fifty-one ādhṛṣīya rows, whose ṇic the dhātupāṭha gaṇasūtra\n"
 "10.0498 *ā dhṛṣād vā* makes optional (√yuj, *yojayati* beside *yojati*);\n"
 "`10.0368 za\\da~` waits for upasargas. The six svarita or ñit among them are\n"
 "ubhayapadī without ṇic too, by 1.3.72 (√vṛ, *varate*). Before ṇic, 7.2.115\n"
 "*aco ñṇiti* lengthens a final vowel (*lāyayati*, *bhāvayati*), the vārttika\n"
 "7.3.37.2 gives √dhū and √prī an optional nuk instead (*dhūnayati*,\n"
 "*prīṇayati*), and 7.2.114 *mṛjer vṛddhiḥ* gives √mṛj vṛddhi on both branches\n"
 "(*mārjati*, *mārjayati*).\n"),
("curated 311-root set, in four lakāras:", "curated 361-root set, in four lakāras:"),
("both correct — and in fact 3476 of the 17964 cells hold more than one form: 2792\nhold two, 389 hold three",
 "both correct — and in fact 5492 of the 21564 cells hold more than one form: 4472\nhold two, 525 hold three"),
("fifty-nine add 1888 two-form cells, a ṇic and a ṇic-less reading each),\n141 hold four",
 "fifty-nine add 1888 two-form cells, a ṇic and a ṇic-less reading each, and\n"
 "slice 10h's fifty add 1680 two-form and 136 three-form cells, √dhū's and\n"
 "√prī's ṇic branch forking again on 7.3.37.2's nuk),\n237 hold four"),
("optional-ṇic rows' laṅ and vidhiliṅ parasmaipada prathama eka, the same way),\nten hold",
 "optional-ṇic rows' laṅ and vidhiliṅ parasmaipada prathama eka, the same way,\n"
 "and — new in slice 10h — forty-eight of the fifty ādhṛṣīya rows', the same\nway),\nten hold"),
("7.1.35/6.4.116/8.4.56), and 141 hold six — the loṭ",
 "7.1.35/6.4.116/8.4.56), and 241 hold six — the loṭ"),
("rows' loṭ parasmaipada prathama and madhyama eka, the same way. One cell — new in slice 3c2 — holds seven:",
 "rows' loṭ parasmaipada prathama and madhyama eka, the same way; and, new in\n"
 "slice 10h, 100 more: forty-eight ādhṛṣīya rows' loṭ parasmaipada prathama\n"
 "and madhyama eka, the same way, and √dhū's and √prī's laṅ and vidhiliṅ\n"
 "parasmaipada prathama eka, three readings × 8.4.56. One cell — new in slice 3c2 — holds seven:"),
("barring the rules that would change it. Two cells — new in slice 10f — hold\n**nine**, the record: `pata`'s loṭ parasmaipada prathama and madhyama eka,\nthree readings × the tātaṅ triple (`patayatu` / `patayatAd` / `patayatAt` /\n`pAtayatu` / `pAtayatAd` / `pAtayatAt` / `patatu` / `patatAd` / `patatAt`).",
 "barring the rules that would change it. Six cells hold **nine**, the record:\n"
 "`pata`'s loṭ parasmaipada prathama and madhyama eka (new in slice 10f),\n"
 "three readings × the tātaṅ triple (`patayatu` / `patayatAd` / `patayatAt` /\n"
 "`pAtayatu` / `pAtayatAd` / `pAtayatAt` / `patatu` / `patatAd` / `patatAt`),\n"
 "and — new in slice 10h — √dhū's and √prī's, the same way (`DAvayatu` /\n"
 "`DAvayatAd` / `DAvayatAt` / `DUnayatu` / `DUnayatAd` / `DUnayatAt` /\n"
 "`Davatu` / `DavatAd` / `DavatAt` for √dhū)."),
("padas — 188 roots that admit both padas in the curated set",
 "padas — 238 roots that admit both padas in the curated set"),
("`pata` and slice 10g's fifty-nine optional-ṇic rows by 1.3.74; and slice 10f's six",
 "`pata`, slice 10g's fifty-nine optional-ṇic rows and slice 10h's fifty\n"
 "ādhṛṣīya rows by 1.3.74 (the six svarita or ñit among the last by 1.3.72\n"
 "too, without ṇic); and slice 10f's six"),
("655 surfaces are pada-ambiguous, each of them a pinned cell in both padas",
 "839 surfaces are pada-ambiguous, each of them a pinned cell in both padas"),
("`OlaRqayata` of `olanq` and `ulanq`). The\nenumeration is not",
 "`OlaRqayata` of `olanq` and `ulanq`); slice 10h's fifty add 184 more, four\n"
 "each but where a homograph shares them (`tfp`, `dfB` and `granT`, and\n"
 "`10.0384 mArga~`'s, which `10.0108 mArga` already supplies). The\nenumeration is not"),
("set, all 655. It is therefore", "set, all 839. It is therefore"),
]

ARCH = [
("| `sanadi.rs` | 2564, 2570, 2573.1, 2573.3, 2573.2, 10.0496, 10.0497, 10.0493, 3.1.25, 1.3.9, 3.4.114, 6.4.48, 7.2.116, 6.4.92, 7.3.86, 3.1.32 — the optional-ṇic fork, the ākusmīya and ā-garvīya pada and the jñapādi's mit-tva, then ṇic, the adanta root's final `a`, and ṇic's folding into the dhātu (curādi only) |",
 "| `sanadi.rs` | 10.0498, 2564, 2570, 2573.1, 2573.3, 2573.2, 10.0496, 10.0497, 10.0493, 3.1.25, 1.3.9, 3.4.114, 6.4.48, 7.2.116, 6.4.92, 7.3.37.2, 7.2.115, 6.1.78, 7.2.114, 7.3.86, 3.1.32 — the optional-ṇic fork, the ākusmīya and ā-garvīya pada and the jñapādi's mit-tva, then ṇic, the adanta root's final `a`, the root's vṛddhi or guṇa or √dhū's and √prī's nuk, and ṇic's folding into the dhātu (curādi only) |"),
("pins all 148 ids verbatim (72 pre-rudhādi, the twenty-one rudhādi added:",
 "pins all 154 ids verbatim (72 pre-rudhādi, the twenty-one rudhādi added:"),
("ṇic-less `a` meets śap — 148 total).",
 "ṇic-less `a` meets śap — 148 total — then curādi 10h's six: the dhātupāṭha\n"
 "gaṇasūtra 10.0498 *ā dhṛṣād vā*, first in `sanadi.rs`; the vārttika 7.3.37.2\n"
 "*dhūñprīñor nug vaktavyaḥ* (the first vārttika id), 7.2.115 *aco ñṇiti*, a\n"
 "second entry for 6.1.78 *eco 'yavāyāvaḥ* and 7.2.114 *mṛjer vṛddhiḥ*,\n"
 "between 6.4.92 and the sanādi 7.3.86; and a second 7.2.114 in `guna.rs`,\n"
 "right before 7.3.84 — 154 total)."),
("— and curādi (10), **open** at 208 of its", "— and curādi (10), **open** at 258 of its"),
("the ten optional-ṇic rows, slice 10f; fifty-nine more, slice 10g). gaṇa",
 "the ten optional-ṇic rows, slice 10f; fifty-nine more, slice 10g; fifty ādhṛṣīya rows, slice 10h). gaṇa"),
("as 1.3.12 does, and every pada sūtra in `samjna.rs` then leaves the root\nalone.",
 "as 1.3.12 does, and every pada sūtra in `samjna.rs` then leaves the root\n"
 "alone. A `PadaAssignment::NicUbhayapada` root (slice 10h's six svarita or ñit\n"
 "ādhṛṣīya rows) is 1.3.74's on its ṇic branch and 1.3.72's on its ṇic-less\n"
 "one: the fork that drops ṇic adds `Tag::Ubhayapadin` as it strips\n"
 "`Tag::Nic`."),
("forking 480 cells (loṭ\nprathama and madhyama eka across the 240 roots with a parasmaipada column —",
 "forking 580 cells (loṭ\nprathama and madhyama eka across the 290 roots with a parasmaipada column —"),
("roots never reach this guard, and the 188 roots that admit both",
 "roots never reach this guard, and the 238 roots that admit both"),
("`pata` and slice 10g's fifty-nine optional-ṇic rows by 1.3.74, and slice 10f's six",
 "`pata`, slice 10g's fifty-nine optional-ṇic rows and slice 10h's fifty\n"
 "ādhṛṣīya rows by 1.3.74 (six of the last by 1.3.72 too, without ṇic), and slice 10f's six"),
("240 + 71 = the 311 curated roots)", "290 + 71 = the 361 curated roots)"),
("forking 468 cells outright: laṅ and vidhiliṅ prathama eka across\nthose same 240 parasmaipada columns (445 of them",
 "forking 536 cells outright: laṅ and vidhiliṅ prathama eka across\nthose same 290 parasmaipada columns (513 of them"),
("and so do 10f's `mUtra`, `katra` and `pata` and 10g's rows other than √śṛdh and √div (whose ṇic branch keys on the sanādi 7.3.86 as well) on their ṇic branch; 10f's and 10g's ṇic-less forks key on their Kaumudī id as well — `2564+8.4.56` and its siblings — and sit outside this count),",
 "and so do 10f's `mUtra`, `katra` and `pata`, 10g's rows other than √śṛdh and √div and 10h's other than its sixteen laghu-ik rows (whose ṇic branch keys on the sanādi 7.3.86 as well) on their ṇic branch; 10f's, 10g's and 10h's ṇic-less forks key on their optional-ṇic id as well — `2564+8.4.56`, `10.0498+8.4.56` and their siblings — as does √dhū's and √prī's nuk branch on 7.3.37.2, and sit outside this count),"),
("forking a further 480 (the same", "forking a further 580 (the same"),
]

AGENTS = [
("(`crates/panini/tests/paradigm/`, 17964 cells, ten gaṇas, nine complete —",
 "(`crates/panini/tests/paradigm/`, 21564 cells, ten gaṇas, nine complete —"),
("at 208 after slice 10g curated fifty-nine more —",
 "at 208 after slice 10g curated fifty-nine more, at 258 after slice 10h curated the fifty ādhṛṣīya rows —"),
("other forms — a second (2792 cells), a third (389 cells), a fourth\n    (141",
 "other forms — a second (4472 cells), a third (525 cells), a fourth\n    (237"),
("    and `katra`'s laṅ and vidhiliṅ prathama eka, and — new in slice 10g — the\n    fifty-nine optional-ṇic rows') and",
 "    and `katra`'s laṅ and vidhiliṅ prathama eka, and — new in slice 10g — the\n"
 "    fifty-nine optional-ṇic rows', and — new in slice 10h — forty-eight\n"
 "    ādhṛṣīya rows') and"),
("    prathama and madhyama eka) to 141 — a fourth",
 "    prathama and madhyama eka) to 141, and slice 10h's forty-eight ādhṛṣīya\n"
 "    rows (the same cells) and √dhū's and √prī's laṅ and vidhiliṅ prathama\n"
 "    eka to 241 — a fourth"),
("    seven-form cell, or up to a ninth for slice 10f's `pata` loṭ\n    parasmaipada prathama and madhyama eka, the two nine-form cells and the\n    record — in",
 "    seven-form cell, or up to a ninth for slice 10f's `pata` and slice 10h's\n"
 "    √dhū and √prī loṭ parasmaipada prathama and madhyama eka, the six\n"
 "    nine-form cells and the record — in"),
("    `ALTERNATES` (4760 rows in all, so 17964 + 4760 = 22724 forms total); √bhuj",
 "    `ALTERNATES` (7532 rows in all, so 21564 + 7532 = 29096 forms total); √bhuj"),
("  252 roots), and that by curādi 10g's (`tools/audit/README.md`'s 2026-10-03 10g\n  entry, 17964 cells / 22724 forms / 311 roots).",
 "  252 roots), and that by curādi 10g's (`tools/audit/README.md`'s 2026-10-03 10g\n"
 "  entry, 17964 cells / 22724 forms / 311 roots), and that by curādi 10h's\n"
 "  (`tools/audit/README.md`'s @DATE@ 10h entry, 21564 cells / 29096 forms /\n"
 "  361 roots)."),
("only in the ordinary corpus-size sense, not wrong in kind: 17964 goldens",
 "only in the ordinary corpus-size sense, not wrong in kind: 21564 goldens"),
]

TOOLS_README = [
("**It asserts the corpus totals** (311 roots, 17964 cells, 22724 forms) rather than",
 "**It asserts the corpus totals** (361 roots, 21564 cells, 29096 forms) rather than"),
("## Last recorded result\n\n",
 "## Last recorded result\n\n"
 "@DATE@, curādi 10h slice, vidyut\n"
 "`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 21564\n"
 "cells / 29096 forms / 361 roots**, with the `entry` negative control verified\n"
 "failing (36 √bhū cells).\n\n"
 "The verdict covers the whole curādi 10h slice: fifty of the fifty-one\n"
 "ādhṛṣīya rows, whose ṇic the gaṇasūtra 10.0498 makes optional (all but\n"
 "`10.0368 za\\da~`, which vidyut derives with the upasarga ā), each derived on\n"
 "its ṇic and its ṇic-less branch; the six svarita or ñit rows' ṇic-less branch\n"
 "in both padas, by 1.3.72; and 7.2.115, the sanādi 6.1.78, the vārttika\n"
 "7.3.37.2 and 7.2.114, which the slice adds. Blocked branches rose from 2736\n"
 "to 4320, the 1584 = 44 × 36 ṇic-less ātmanepada cells of the `Nic` rows. A\n"
 "throwaway prototype's audit was the same, zero differences, from its first\n"
 "run; dropping √prī's nuk, as a control, made 72 cells differ. A\n"
 "main-vs-branch dump of every prior cell's traces was byte-identical, all\n"
 "22724 live branches.\n\n"
 "Totals: 361 = 311 + 50; 21564 = 17964 + 3600 (400 root×pada×lakāra blocks ×\n"
 "9); 29096 = 22724 + 3600 + 2772 new `ALTERNATES` rows (4760 → 7532), measured\n"
 "via the harness's corpus block, not assumed.\n\n"),
]

MAIN_RS = [
("/// failing (36 √bhū cells), and curādi 10g's re-ran it at the same commit\n/// over all 17964 cells / 22724 forms / 311 roots with zero differences,\n/// its `entry` negative control verified failing (36 √bhū cells). √tṛh joins none of the fork",
 "/// failing (36 √bhū cells), and curādi 10g's re-ran it at the same commit\n"
 "/// over all 17964 cells / 22724 forms / 311 roots with zero differences,\n"
 "/// its `entry` negative control verified failing (36 √bhū cells), and\n"
 "/// curādi 10h's re-ran it at the same commit over all 21564 cells / 29096\n"
 "/// forms / 361 roots with zero differences, its `entry` negative control\n"
 "/// verified failing (36 √bhū cells). √tṛh joins none of the fork"),
]

GUNA = [
("    // 311-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)",
 "    // 361-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)"),
]

DATA = [
("    /// vendored upadeśa: 66 of the 311 curated roots carry a `\\` at all, and 45",
 "    /// vendored upadeśa: 73 of the 361 curated roots carry a `\\` at all, and 52"),
("/// `10.0124 ciY`, which is mit but not yet curated (it waits on 7.2.115).",
 "/// `10.0124 ciY`, which is mit but not yet curated: 7.2.115 is in since slice\n"
 "/// 10h, but √ci still waits on its own slice, for 6.1.54 and 7.3.36."),
]

SPEC_10G = [
("- 10.0498 ādhṛṣīya (51 rows), then 10.0499 āsvadīya (59 rows). Each\n"
 "  teaches `optional_nic_from_upadesha` its range and drops this slice's\n"
 "  exclusion assertion.",
 "- 10.0498 ādhṛṣīya (51 rows), then 10.0499 āsvadīya (59 rows). Each\n"
 "  teaches `optional_nic_from_upadesha` its range and drops this slice's\n"
 "  exclusion assertion. (Slice 10h took fifty of the ādhṛṣīya, all but\n"
 "  `10.0368 za\\da~`: see `2026-10-03-curadi-gana-10h-design.md`.)"),
("- √ci (`10.0124`) rides the 7.2.115 slice with √gṛ and √yu.",
 "- √ci (`10.0124`), √gṛ and √yu. 7.2.115 landed in slice 10h, so √gṛ and √yu\n"
 "  are curation; √ci also needs 6.1.54 and 7.3.36."),
]

FILES = {
    "README.md": README,
    "docs/ARCHITECTURE.md": ARCH,
    "AGENTS.md": AGENTS,
    "tools/audit/README.md": TOOLS_README,
    "crates/panini/tests/paradigm/main.rs": MAIN_RS,
    "crates/panini-prakriya/src/tinanta/guna.rs": GUNA,
    "crates/panini-data/src/lib.rs": DATA,
    "docs/superpowers/specs/2026-10-03-curadi-gana-10g-design.md": SPEC_10G,
}

def main():
    date = sys.argv[1]
    out, bad = {}, []
    for path, edits in FILES.items():
        s = open(path).read()
        for old, new in edits:
            n = s.count(old)
            if n != 1:
                bad.append(f"{path}: {n}x {old[:70]!r}")
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
python3 /tmp/vidyut-full/slice10h/docsweep_10h.py "$(date -u +%F)"      # applied 44 edits to 8 files
```

If it prints `not applied:`, nothing was written. Edit the paragraph named to the same facts, rather than skipping it, and re-run. (The prototype and its replay ran it with `2026-10-03`; another date changes only the two dated lines.)

Notes on the numbers, re-derived from the census rather than assumed:
- **Multi-form cells:** 5492 = 21564 − 16072. The buckets are 4472 / 525 / 237 / 10 / 241 / 1 / 6, and no cell holds eight.
- **Both-pada roots:** 238 = 188 + 50. Every new row has a parasmaipada column, so 290 + 71 = 361: the ātmanepada-only count is unchanged.
- **7.1.35 forks:** 580 = 290 × 2, its loṭ prathama and madhyama eka; 8.4.56's "further" fork covers the same cells.
- **8.4.56 forks:** 536 = `key_count("8.4.56")` = 513 + 22 + 1, where 513 = 445 + 68. Thirty-four new rows' ṇic branches fork laṅ and vidhiliṅ prathama eka on 8.4.56 alone, √dhū's and √prī's vṛddhi branch among them. The sixteen laghu-ik rows key on 7.3.86 as well, and the ṇic-less and nuk forks key on 10.0498 and 7.3.37.2, so neither sits in it.
- **The rule-order pin:** 154 = 148 + 6. The new entries are 10.0498, 7.3.37.2, 7.2.115, 6.1.78 (second), 7.2.114 (sanādi) and 7.2.114 (guṇa).
- **`pada_from_upadesha`'s doc:** "73 of the 361 curated roots carry a `\` at all, and 52" without a `~\`. The seven new ones are `yu\ja~`, `jri\`, `ri\ca~`, `Si\za~`, `ta\pa~`, `mI\` and `va\ca~`, all on a root vowel (measured).
- **`JNAPADI`'s doc:** √ci now waits on its own slice (6.1.54, 7.3.36), not on 7.2.115.

- [ ] **Step 4: The AGENTS.md stale-comment ledger**

Create `/tmp/vidyut-full/slice10h/ledger_10h.py` if it is missing (sha256 `f5ac4380d68e3be76a2780fda8077a41576bc84b9b12b6de3b35a51de1aac418`). It measures both anchors by grep at this commit; it never computes them:

```python
#!/usr/bin/env python3
"""THROWAWAY: add 10h's sentence to AGENTS.md's stale-comment ledger, with
both anchors measured by grep now."""
import re, subprocess
g = subprocess.run(['grep', '-n', '1872 goldens move', 'crates/panini-prakriya/src/tinanta/guna.rs'], capture_output=True, text=True).stdout.split(':')[0]
c = subprocess.run(['grep', '-n', 'only 8 cells fire', 'crates/panini-prakriya/src/controller.rs'], capture_output=True, text=True).stdout.split(':')[0]
assert g and c
p = 'AGENTS.md'
s = open(p).read()
m = re.search(r"Curādi 10g touched neither comment either; the corpus stands at 17964 cells as of 10g \(`guna\.rs:2565`'s claim anchored at `guna\.rs:\d+`, `controller\.rs:206`'s at `controller\.rs:\d+`; both lines measured by grep at this commit\)\.", s)
assert m
new = m.group(0) + f" Curādi 10h touched neither comment either, though its guṇa-stage 7.2.114 moved the first; the corpus stands at 21564 cells as of 10h (`guna.rs:2565`'s claim anchored at `guna.rs:{g}`, `controller.rs:206`'s at `controller.rs:{c}`; both lines measured by grep at this commit)."
s = s[:m.start()] + new + s[m.end():]
open(p, 'w').write(s)
print(f"ledger: guna.rs:{g} controller.rs:{c}")
```

```bash
python3 /tmp/vidyut-full/slice10h/ledger_10h.py      # ledger: guna.rs:2641 controller.rs:206
```

`guna.rs`'s anchor moves 2565 → 2641 because of Task 2's guṇa 7.2.114 and its unit test; the comment's text is untouched. AGENTS.md's floor paragraph (`measured at 17964 cells`) and the current mutation record belong to Task 5.

- [ ] **Step 5: Sweep for anything left stale**

```bash
grep -rn -i -E "\b17964\b|\b22724\b|\b4760\b|\b1996\b|311 roots|311-root|of these 311|of the 311|311 curated|208 of|\b188 roots|\b155 curādi|155 \`Nic\`|\b655\b|\b14488\b|\b2792\b|240 \+ 71|forking 480|\b468 cells|\b2736\b|sixty-nine|\b148 ids|148 total|twenty-three hold|two nine-form|Two cells — new in slice 10f" README.md AGENTS.md docs/ARCHITECTURE.md tools/audit/README.md crates --include=*.md --include=*.rs | grep -v "paradigm/data/"
grep -rn -i -E "waits on 7\.2\.115|whichever slice first needs|7\.2\.115.{0,40}(defer|later|not yet)|(defer|later|not yet).{0,40}7\.2\.115|four (Kaumudī )?vikalpa|four optional-ṇic|ādhṛṣīya.{0,40}(later slice|uncurated|not yet)|10\.0279..=\"10\.0388|parasmaipada by 1\.3\.78 for every row" README.md AGENTS.md docs/ARCHITECTURE.md tools crates --include=*.md --include=*.rs | grep -v "paradigm/data/"
for code in yuj pfc arc sah Ir lI vfj vf jF jri ric Siz tap tfp Cfd cfp Cfp dfp dfB law SraT mI granT SIk cIk ard hins arh SunD Cad juz DU prI SranT Ap tan can vad vac mAn BU garh mArg kanW mfj mfz Dfz; do
  grep -rln "\"$code\"" crates --include=*.rs | grep -v "paradigm/data/" | LC_ALL=C sort | tr '\n' ' ' | sed "s#^#$code: #"; echo; done | grep -v -E "^[A-Za-z]+: crates/panini-data/src/lib.rs $|^[A-Za-z]+: crates/panini-data/src/lib.rs crates/panini/tests/paradigm/main.rs $"
```

Expected residue from the first grep, all of it dated history that stays true:
- `tools/audit/README.md`'s 10h entry's own `Totals:`, blocked-branch and prior-dump lines, and the 10g and older entries;
- ARCHITECTURE's pin chain ("— 148 total — then curādi 10h's six");
- AGENTS.md:37's floor paragraph (Task 5), its audit chain (the 10g clause) and its stale-comment ledger;
- `paradigm/main.rs`'s audit chain (the 10g clause), its 10g paragraph ("OPEN at 208") and the pada-ambiguous comment's "432 to 655" / "655 to 839".

The second grep prints exactly `crates/panini-data/src/lib.rs:108`, JNAPADI's new "7.2.115 is in since slice 10h" sentence.

The third loop prints exactly eleven lines, each a pre-existing root's code or a Task 2 unit test:
- **Codes Task 2's unit tests construct:** `lI`, `jF`, `jri`, `DU`, `prI` and `mfj`. Some of these also appear as `check()` witnesses in `paradigm/main.rs`.
- **Codes pre-existing roots already use:** `vf` (√vṛ's tripadi and adesha pins, plus `samjna.rs`'s new test), `mI` (kryādi), `Ap` and `tan` (svādi, tanādi) and `BU` (bhvādi). `sanadi.rs`'s tests now use `mI` and `BU` too.

Any other hit from any of the three is a miss. Edit it.

- [ ] **Step 6: Run the full suite and commit**

Run: `mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"` (foreground, timeout 600000 ms). Expected: PASS at 21564 cells.

```bash
mise run fmt && mise run lint
git branch --show-current      # curadi-10h
git add -A
git commit -m "docs: 10h's counts, the audit record, and the sweep

21564 cells / 29096 forms / 361 roots across README, ARCHITECTURE, AGENTS,
paradigm/main.rs and tools/audit; curādi open at 258/509; 238 both-pada
roots; 839 pada-ambiguous surfaces; 154 pinned rule ids. Audit at zero
divergence against 8da2f90b with 4320 blocked branches; prior traces
byte-identical to main."
```

---

## Task 5: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and the current-record paragraph); `mise.toml` if the cap moves

Follow AGENTS.md's cargo-mutants protocol. Hazards from this repo's record:
- **Measure, never scale.** The cap must exceed a full uncaught suite run at the parallelism used, under campaign load.
- **Every invocation rotates `mutants.out`**, so always pass `-o`, and copy `outcomes.json` durably before any other invocation.
- **The mise shim fails.** Use the real binary: `CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`.
- **`pgrep -f` matches its own shell.** Wait on `pgrep -x cargo-mutants`.
- **Background shells die at about 60 minutes.** Launch detached with `setsid nohup`, as below, and resume with `--iterate` if it dies.

The prototype measured these, which are the values to expect:
- **The mutant list grows by fourteen.** `panini-prakriya` lists 844 mutants (830 on main), `panini-analyze` 12 and `panini-data` 12, both unchanged.
- **`--in-diff` over the slice's production diff for `panini-prakriya`** lists 23. All are expected CAUGHT:
  - **Fourteen new:**
    - the guṇa 7.2.114 guard (`guna.rs:237`, two);
    - 7.3.37.2's guard (`sanadi.rs:429`/`430`, three);
    - 7.2.115's guard (`sanadi.rs:452`);
    - the sanādi 6.1.78's guard and its four match arms (`sanadi.rs:478`, `483`–`486`);
    - the sanādi 7.2.114's guard (`sanadi.rs:507`/`508`, three).
    Each is killed by its Task 2 unit test: both directions per guard, and `ne` / `Bo` for the `e` / `o` arms.
  - **Nine in functions the slice touched:** `derive` (two), `skip_nic` (two), and `vrddhi_of` (five, including deleting its `'f' | 'F'` and `'u' | …` arms). These were caught in 10g.
- **`--in-diff` over the data crate's diff** lists `lib.rs`'s two `PadaAssignment::padas` mutants (`Vec::leak(Vec::new())`, `Vec::leak(vec![Default::default()])`). The function's match gained an arm, and `padas_maps_each_assignment_to_its_derivable_padas` kills both.
- **The 51 documented non-caught entries are unchanged, modulo span lines.** `adesha.rs:647:30` (MISSED) stays put. `tripadi.rs:1305:38` (MISSED) and `tripadi.rs:1618:23` (the permanent TIMEOUT) are where main has them. They moved +2 in 22eb329, 10g's final-review fixes, after the 10g record (which says 1303/1616) was written. Seventeen `guna.rs` unviables move +31 lines, behind Task 2's guṇa 7.2.114 (250 → 281, 519 → 550, 945 → 976, … 1077 → 1108). Confirm `<A>`, `<T1>` and `<T2>` by `--list`; never compute them.
- **The floor at 21564 cells:** two `mise run test` runs took 2m18.492s and 3m13.070s wall clock (user 5m37.9s and 5m43.7s). Load averages were `54.48 57.31 42.96` before, `52.12 54.20 43.67` between and `80.23 66.22 50.28` after, on 24 cores (external load). The cap in force is 1810.

- [ ] **Step 1: Measure the floor**

With nothing else of ours running, run this twice: `cat /proc/loadavg; time mise run test >/dev/null 2>&1; cat /proc/loadavg` (foreground, timeout 600000 ms). Record both wall clocks, user CPU and the load averages. Read 10g's floor from AGENTS.md's floor paragraph and keep the comparison chain.

- [ ] **Step 2: Locate and probe the two uncaught equivalents at `-j 4`**

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | wc -l      # 844
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /="
```

Expect `647` (adesha), `1305` (tripadi, inside 8.3.13's `apply`) and `1618` (the ṇatva hang). Write them as `<A>`, `<T1>` and `<T2>`. Then run in the foreground with timeout 600000 ms:

```bash
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 1810 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:<A>:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" 2>&1 | tail -6
```

The two regexes also match two caught `mod.rs` mutants; that is expected. Both equivalents must be MISSED, not TIMEOUT. Read each test-phase duration from `$SCRATCH/mutants.out/outcomes.json`. Set the provisional cap to max(1810, 6 × the longer of the two, rounded up to the next 10 s).

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/curadi-10h"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout <CAP> -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"; cat /proc/loadavg > "$OUT/load.started"
```

`<CAP>` is Step 2's provisional cap. Run nothing CPU-heavy meanwhile. 10g's campaign took two hours at 17964 cells; expect longer. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

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
- **856 mutants: 805 caught, 48 unviable, 2 missed, 1 timeout.**
  - panini-prakriya: 844 / 797 / 44 / 2 / 1.
  - panini-analyze: 12 / 8 / 4 / 0 / 0.
- `missed.txt` holds exactly `adesha.rs:<A>:30: replace + with *` and `tripadi.rs:<T1>:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:<T2>:23: replace -= with /=`.

Then diff the non-caught set against 10g's on the full record, both with and without span lines:

```bash
python3 - "$OUT/outcomes.durable.json" /home/dev/mutants-records/curadi-10g/outcomes.durable.json <<'PY'
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
- **Without lines:** `51 51`, `new: []`, `gone: []`.
- **With lines:** nineteen entries on each side, exactly the moves listed above. Seventeen `guna.rs` unviables move +31, and `tripadi.rs` 1303 → 1305 and 1616 → 1618 move +2. The slice moves no other non-caught span.

If not:
- Any **other timeout** is a suspect survivor that the larger suite pushed past the cap. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant among the fourteen new ones is a gap in Task 2's tests; add the test that kills it. Any other missed mutant means a test that caught it at 17964 cells no longer does; stop and report.

**Step 4b: the data-crate mutants.** Run the slice's diff for `panini-data` alone:

```bash
git diff e28a74f -- crates/panini-data/src/lib.rs > "$OUT/data.diff"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-data --test-workspace=true \
  --in-diff "$OUT/data.diff" --timeout <CAP> -j 4 -o "$OUT/data" 2>&1 | tail -3
```

Expected: `2 mutants tested`, both caught (`PadaAssignment::padas`).

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
    if o["summary"] in ("MissedMutant", "Timeout"):
        m = o["scenario"]["Mutant"]; print(o["summary"], m["file"], m["span"]["start"]["line"], test_phase(o))
PY
```

Compute two things:
- the two equivalents' test phases under campaign load;
- the caught phases' min / median / p90 / max.

The cap is max(430, 6 × the longest campaign-load equivalent phase, rounded up to the next 10 s).
- If that is 1810 or less, the cap stays 1810.
- Otherwise change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together (`grep -n "1810" AGENTS.md mise.toml`).

- [ ] **Step 6: Record it in AGENTS.md**

- **The floor paragraph.** Rewrite the paragraph that opens `**The floor behind the 1810s cap, measured at 17964 cells on Rust 1.99.0,`. Use Step 1's and Step 2's numbers at 21564 cells, the load averages, and the cap Step 5 chose. Keep the comparison chain to earlier floors, with 10g's joining it: 1m45.672s / 1m40.061s at 17964 cells, isolated probe 160.20s / 163.19s, campaign-load 300.16s / 107.56s.
- **The current record.** Replace the `**Current record (curādi 10g, …).**` paragraph with `**Current record (curādi 10h, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**;
  - the non-caught set diffed against 10g's on the full record, both ways (Step 4's script output), naming the nineteen span moves and their two causes;
  - the fourteen new mutants, every one CAUGHT, by name;
  - Step 4b's two data-crate mutants and their outcome;
  - the campaign-load phases and margins;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`.

  End it with a pointer to the record it replaces. Run `git rev-parse --short HEAD` before committing, and write ``The curādi 10g record it replaces: `git show <that hash>:AGENTS.md`.``

- [ ] **Step 7: Commit**

```bash
git branch --show-current      # curadi-10h
git add AGENTS.md mise.toml
git commit -m "chore: 10h mutation gate — floor and uncaught run re-measured at 21564 cells

Fourteen new mutants, all caught; missed.txt holds only the two documented
equivalents and timeout.txt only the permanent ṇatva-scan entry. The
non-caught set is 10g's modulo span lines (guna.rs +31 behind the guṇa
7.2.114, tripadi.rs +2 from 22eb329)."
```

---

## Task 6: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -20
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # back at /workspace/crates
git branch --show-current      # curadi-10h
git log --oneline main..HEAD   # the spec, the plan and the four task commits
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin curadi-10h
gh pr create --title "curādi 10h — the ādhṛṣīya (10.0498)" --body "$(cat <<'BODY'
Slice 10h curates fifty of the fifty-one ādhṛṣīya rows (`10.0338 yu\ja~` …
`10.0388 Dfza~`, all but `10.0368 za\da~`, which waits for upasargas),
whose ṇic the dhātupāṭha gaṇasūtra 10.0498 *ā dhṛṣād vā* makes optional.

- 10.0498 joins the optional-ṇic forks, first, as vidyut checks it.
- `PadaAssignment::NicUbhayapada`: the six svarita/ñit rows are 1.3.74's
  with ṇic and 1.3.72's without (*varate*); 1.3.72 never reaches a ṇic
  branch.
- 7.2.115 *aco ñṇiti* before ṇic, with a sanādi 6.1.78 (*lāyayati*,
  *bhāvayati*); the vārttika 7.3.37.2's optional nuk for √dhū and √prī
  (*dhūnayati*, three readings per cell); 7.2.114 *mṛjer vṛddhiḥ* on both
  of √mṛj's branches.
- `OPTIONAL_NIC` (119) reads the ādhṛṣīya by position and stays out of the
  āsvadīya and `10.0368`; √arh resolves by its optional-ṇic verdict.

The golden suite goes from 17964 to 21564 cells; curādi is open at 258 of
509. The audit shows zero divergence against `8da2f90b`, a main-vs-branch
dump of every prior cell's traces is byte-identical, and the mutation gate
finds every new mutant caught and the documented non-caught set unchanged.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`:
   - run `git worktree remove .worktrees/curadi-10h`;
   - run `git worktree remove --force .worktrees/curadi-10h-proto` and `git branch -D proto-10h` (the throwaway);
   - run `git worktree remove --force .worktrees/curadi-10h-replay` if the replay worktree is still there (detached; no branch);
   - delete the local and remote `curadi-10h` branch;
   - run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| 50 rows, codes, 44 `Nic` + 6 `NicUbhayapada`; `10.0368` out | 3 Step 6 |
| `OPTIONAL_NIC` 69 → 119, every new entry `"10.0498"` | 3 Steps 3, 6 |
| 10.0498 as a sanādi vikalpa, first, 2564's bars | 2 |
| `optional_nic_from_upadesha` range-aware; exclusion narrowed to `10.0279..=10.0337` + `10.0368` | 3 Step 3 |
| `NicUbhayapada`: `Tag::Nic` + `Tag::NicUbhayapada`; `skip_nic` adds `Ubhayapadin`; doc on `UbhayapadaAnavane`'s model | 2 |
| `Dhatu::padas` unchanged (the variant joins the two-pada arm) | 2 |
| 7.2.115 after 6.4.92 and 7.3.37.2, before 7.3.86; eight rows | 2; 3 Step 2 |
| Sanādi 6.1.78 after 7.2.115; stage-scoped lookups | 2 |
| 7.3.37.2, optional, before 7.2.115; three-way fork | 2; 3 Steps 2, 4 |
| 7.2.114 in `SANADI` and `GUNA`, `Tag::Mrj` from `MRJ = ["10.0386"]` | 2 |
| `10.0367 arha~` resolves by verdict, no pin | 3 Step 3 (`dhatupatha_numbers_resolve_upstream` passes with no pin) |
| Homographs: `tfp`, `dfB`, `granT`; `mArgayARi`, `mAnayate`; `Bavati` / `vadati` order pinned | 3 Steps 1, 4 |
| Goldens via the harness; census 361 / 21564 / 29096; blocked +1584 | 3 Steps 1, 6; 4 Step 2 |
| 1.3.74 count 155 → 199 (`Nic`), the six `NicUbhayapada` rows asserted beside them | 3 Steps 2, 3 |
| Rule pins: order, bars, vikalpa; `the_optional_nic_ids_…` gains 10.0498 | 2; 3 Step 2 |
| Data tests: 119, exclusion, `NicUbhayapada` pada arm (non-circular) | 3 Step 3 |
| Unit tests in both directions per guard | 2 |
| Corpus-wide fires-only-on-rows; trace diff main ↔ HEAD | 3 Step 2; 4 Step 2 |
| `check()` witnesses from the spec's tables; goldens grepped first | 3 Steps 4, 7 |
| Mutation gate: scoped, floor re-measured, `-o`, durable copy, chunking, non-caught named verbatim | 5 |
| AGENTS.md, ARCHITECTURE (pin count 154, sanādi description, forks), `tools/audit/README.md`, `panini-data` docs, sanādi module doc | 2 (module doc); 3 Step 3; 4 Steps 3–4; 5 |
| 10g spec's "Later slices" pointer and √ci bullet | 4 Step 3 |
| Sweep greps: root-shape literals, all of `crates/`, sibling enumerations, "7.2.115", spelled-out and wrapped counts | 4 Steps 3, 5 |

**Type consistency.**
- `PadaAssignment::NicUbhayapada` is matched exhaustively in `tinanta::derive` and in `samjna::tests::pada_anga`, both arms adding `Tag::Nic` and `Tag::NicUbhayapada`.
- `optional_nic_from_upadesha(number: &str, upadesha: &str) -> Option<&'static str>`. All five call sites pass a `&str` row number: the marker test, its `upadesha` closure, the uniqueness closure `verdict(n, u)`, the √śraṇ pin and the `Cidra` assertion.
- `rows_crediting(sutra: &str, sanadi: bool) -> Vec<&'static str>` compares against `[&str; N]` arrays.
- `curadi_analyses_its_adhrsiya_forms`' tuples are `(&str, &[&str], Pada, bool, &[&str])`. The first tuple's `&["yuj"][..]` fixes the slice types.
- The ākusmīya test's `adhrsiya_padas` is `&[Pada]`, compared with `Vec::contains`, since `Pada` has no `Ord`.
- The golden tuple shapes match `ParadigmRow` and `AlternateRow`; the generator is 10g's, with the row filter and the vikalpa list changed.

**Known soft spots.**
- **Doc strings in Task 4** were read at the prototype's state. If the script reports an `old` not found exactly once, edit that paragraph to the same facts rather than skip it.
- **Task 5's campaign numbers are expectations**, not prototype measurements. The floor and cap depend on host load; record the load beside every timing. The prototype's two suite runs (2m18s and 3m13s) are well above 10g's (1m46s and 1m40s), partly from load.
- **Where this plan goes beyond the spec's letter**, each to make the spec's decisions work:
  - **`vrddhi_of` gains an `F` arm.** √jṝ's 7.2.115 needs it, and the spec names `jF → jAr`.
  - **`Tag::Nijanta`'s doc changes.** "No rule reads it yet" stops being true: the spec's "the `GUNA` entry declines on a ṇijanta aṅga" reads it.
  - **7.3.37.2 is keyed on the aṅga's text** (`DU` | `prI`), as vidyut keys it. In the sanādi stage the aṅga is the bare root, before any aṭ.
  - **`MRJ`'s rationale is worded "the sūtra names a root, and a root is a row".** The spec wrote "root text cannot decide it once the code is `mfj`".
  - **The 8.3.24 and 8.4.40 rosters gain six and three rows.** The spec's Evidence named one row each.
- **The pada-ambiguous count's arithmetic** (200 − 12 − 4) is derived after the fact. The set itself is the measured one, hash-pinned by the script.
