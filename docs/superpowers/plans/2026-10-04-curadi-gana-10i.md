# Curādi gaṇa slice 10i Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate the fifty-nine āsvadīya rows (`10.0279 grasa~` … `10.0337 svAda~`), whose ṇic the dhātupāṭha gaṇasūtra 10.0499 *ā svadaḥ sakarmakāt* makes optional, together with the last two one-row optional-ṇic triggers, `10.0022 pF` (Kaumudī 2565) and `10.0251 Guzi~r` (Kaumudī 2571), across laṭ / laṅ / loṭ / vidhiliṅ. The golden suite goes from 21564 to 25956 cells, and curādi from 258 to 319 of its 509 rows.

**Architecture:** Six tasks:
- **Task 1** creates the worktree and checks the baseline.
- **Task 2** is the engine, green on its own: no curated root before this slice is āsvadīya, `pF`, `Guzi~r`, √dhūp or √vich, and no curated code has a short vowel before `C`, so every golden and every prior trace is unchanged. It adds:
  - the gaṇasūtra 10.0499 and the Kaumudī's 2565 and 2571, optional-ṇic vikalpas in vidyut's check order;
  - 3.1.28 *gupūdhūpavicchipaṇipanibhya āyaḥ* right after 3.1.25, keyed on `Tag::Aya` from the row list `panini_data::AYA`, with 3.4.114 and 3.1.32 taking its āya as they take ṇic (`sanadi_pratyaya`) and 3.1.32 setting `Tag::Nijanta` for ṇic only;
  - a second 6.1.73 *che ca*, in `SANADI` before the sanādi 7.3.86, whose apply is the aṅga stage's own, extracted as `anga::che_ca`.
  The tests go in first and fail.
- **Task 3** lands the sixty-one rows, their `OPTIONAL_NIC` entries, 488 golden rows and 2928 alternates, and every count, list, roster, uniqueness and `check()` assertion they move. The assertions go in first and fail; the rows and goldens make them pass.
- **Tasks 4–6** are the audit with the prior-trace diff and the doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.99.0, pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, checked out at `/tmp/vidyut-full`.

**Spec:** `docs/superpowers/specs/2026-10-04-curadi-gana-10i-design.md`. Its row table is the slice's scope. Its Decisions section explains:
- 10.0499, 2565 and 2571 and their placement;
- the helper's range and its two triggers;
- 3.1.28 and why it is keyed by row number (`AYA`);
- the sanādi 6.1.73 and why it shares the aṅga entry's apply;
- the `laGi~` pair;
- the homographs.

**Workspace:** The spec and this plan are on the branch `curadi-10i`. Task 1 puts `/workspace` back on `main` and checks `curadi-10i` out at `/workspace/.worktrees/curadi-10i`. Every path below is relative to that directory unless it starts with `/`.

**Provenance.** This slice was built end to end on a throwaway worktree, `/workspace/.worktrees/curadi-10i-proto` (branch `proto-10i`, on the spec commit `a55d993`). Its throwaway commits are `35145e0` (engine), `1c52532` (rows and assertions) and `279a853` (audit and docs), and Task 6 deletes it. Every code block and script below is that prototype's code, and it was checked three ways:
- **Prototype checks.** The prototype's final state:
  - passed the full suite, clippy `-D warnings` and `fmt-check`;
  - its generator found all 4392 new cells equal to vidyut's (4392 cells, 7320 forms, 0 differences);
  - the audit at 422 roots / 25956 cells / 36416 forms showed zero differences, with 6516 blocked branches and the `entry` control failing on 36 cells;
  - a main-vs-prototype dump of every prior branch, blocked ones included (33416 lines, 29096 of them live), was byte-identical.
- **Replay.** A first build applied Task 3's rows and goldens before its assertions; a replay ran the scripts below in this plan's order on a fresh worktree at `a55d993` and reproduced that build's final tree byte for byte in every tracked file (tree `ad032e2ae4ab`). The replay script is `/tmp/vidyut-full/slice10i/replay_10i.sh`, a verification aid this plan does not need. Task 2 Step 2's and Task 3 Step 5's failing lists below are that replay's, and so are the per-binary counts.
- **Negative controls.** Disabling the sanādi 6.1.73 (its apply returning `false`) made the audit fail on 72 cells, √vich's every cell in both padas. An earlier exploratory prototype of the same slice (since replaced) made 7.2.115 decline for `ji` and saw 72 cells differ, and its first run, before 3.1.28 and the sanādi 6.1.73 existed, saw 108.

The prototype did **not** run the mutation campaign. Task 5's campaign numbers are expectations, derived from the measured mutant lists (below), and the campaign measures them.

**Throwaway scripts.** Everything under `/tmp/vidyut-full/slice10i/` and the two vidyut examples (`curadi_goldens_10i.rs`, `trace_dump_10i.rs`) never ship. Each is reproduced in full in this plan with its sha256, so it can be recreated if `/tmp` was cleaned. Recreate a file only if it is missing, and check its hash either way (`sha256sum <file>`).

## Global Constraints

- **The new grammar is exactly this:**
  - **ids:** 10.0499 (`SANADI`, right after 10.0498); 2565 (right after 2564); 2571 (right after 2570); 3.1.28 (right after 3.1.25); a second 6.1.73 (`SANADI`, between the sanādi 7.2.114 and the sanādi 7.3.86). The rule-order pin goes from 154 to 159.
  - **names:** 10.0499 `"A svadaH sakarmakAt"`; 2565 `"dIrGoccAraRaM RicaH pAkzikatve liNgam"`; 2571 `"nizeDAlliNgAdanityo'sya Ric"` (both the Siddhānta-Kaumudī's text, at `10.0022 pF` and `10.0251 Guzi~r`, read from `ashtadhyayi-com/data`'s `sutraani/kaumudi.txt`); 3.1.28 `"gupUDUpavicCipaRipaniBya AyaH"`; the sanādi 6.1.73 `"Ce ca"`, as the aṅga entry's.
  - **tag:** `Tag::Aya`.
  - **const:** `panini_data::AYA = ["10.0303", "10.0304"]`.
  - **functions:** `anga::che_ca` (the aṅga entry's apply, extracted unchanged and shared by both 6.1.73 entries) and `sanadi::sanadi_pratyaya` (read by 3.4.114 and 3.1.32).
  Nothing else in the engine changes.
- **Rows:** exactly the spec's sixty-one, each `code` what `stored_form` computes, all `PadaAssignment::Nic`. `OPTIONAL_NIC` has exactly 180 entries, in dhātupāṭha order: `"10.0499"` for the fifty-nine `10.0279..=10.0337`, `"2565"` for `10.0022`, `"2571"` for `10.0251`.
- **Not modelled:** 10.0499's *sakarmakāt* (vidyut ignores it too), 3.1.31 *āyādaya ārdhadhātuke vā* (no ārdhadhātuka lakāra is in scope), 6.1.48 and 6.1.54 for the udātta `ji` and `ci` (vidyut keys them on `ji\` and `ciY` / `ci\Y`).
- **Out of scope:** `10.0368 za\da~` (upasarga ā, 7.3.78), 2572 (unreachable), 3.1.28's `gupU~`, `paRa~\` and `pana~\` (uncurated), √ci `10.0124`, √gṛ, √yu, √smiṅ, the causative.
- **Pre-existing cells must stay byte-identical, traces included,** with no exception. Regenerate no prior golden.
- **Goldens come from the generator, which asserts engine = vidyut cell by cell, and their sha256 must match this plan's.** **Do not edit a golden to match the engine.** If the generator reports a difference, or a hash differs, stop and report.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit. Run `git branch --show-current` before every commit and the push: it must print `curadi-10i` (a detached HEAD strands commits).
- `mise run test` took 3–7 minutes in the prototype, under external load (load 15–80): the `trace` binary alone ran 49–230 s. Run it in the **foreground** with a timeout of 600000 ms; if a run outgrows the 10-minute cap, start it detached with its output in a log file and an `EXIT_CODE=` sentinel, then wait in the same turn with `while kill -0 <PID>; do sleep 30; done`. Never background it and end a turn, never arm a Monitor and end a turn, and never pipe it through `tail`.
- `mise run test -- -p X` does not scope. Scope with `mise exec -- cargo test -p <crate> <filter>`. To see every failing binary at once, use `mise exec -- cargo test --workspace --no-fail-fast`.
- The `cargo-mutants` mise shim fails here. Use the real binary, `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`, under `mise exec --`.
- `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes its `panini` and `panini-data` dev-deps. Repoint them at this worktree before generating or auditing, and back at `/workspace/crates` after. Check with `grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml` every time.
- Never wait on a process with `pgrep -f` (it matches its own shell); use `pgrep -x cargo-mutants` or `kill -0 <pid>`.
- Review packages and diffs exclude the appended goldens (`crates/panini/tests/paradigm/data/curadi.rs`) and use `git diff --diff-algorithm=histogram`: the default algorithm shows appended golden rows as fake deletions.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **3.1.28 on the wrong branch or row.** āya must reach only √dhūp's and √vich's ṇic-less branches: never a ṇic branch, never another row, and 3.4.114 and 3.1.32 must reach a ṇic-less branch exactly where āya does. Goldens hold the forms but not where 3.1.28 is credited. → Task 2 `aya_follows_a_tagged_root_that_took_no_nic` (declines on a ṇic branch and on untagged `gup`); Task 3 `no_nic_pada_rule_reaches_a_nicless_branch` (corpus-wide: 3.1.28, 3.4.114 and 3.1.32 on a ṇic-less branch iff the row is in `AYA`, no ṇic branch credits 3.1.28) and `the_10i_aya_and_tuk_fire_only_on_their_rows`.
2. **The sanādi 6.1.73 firing off √vich.** Its apply is the whole-word scan with no guard but the sūtra's own, and the sanādi stage runs for every root of every gaṇa: any future code with a short vowel before `C` takes tuk there. → Task 3 `the_10i_aya_and_tuk_fire_only_on_their_rows` (the sanādi 6.1.73 on `10.0304` alone); Task 4's prior-trace dump (all 33416 prior branches byte-identical); Task 2 `che_ca_gives_vich_tuk_before_guna_can_read_its_upadha` (declines on `mUrC`, `Cid`, `cur`).
3. **Two entries under one id.** With 6.1.73 in two stages, `rules().find(|r| r.id == "6.1.73")` returns the sanādi entry, so `anga.rs`'s unit test would silently test it. → Task 2's stage-scoped lookup (`ANGA_RULES.iter().find`), `tinanta_rule_order_is_pinned` (159 ids, both 6.1.73s placed) and the bars pin's "ids are not unique" doc.
4. **`Tag::Nijanta` on an āya aṅga.** The guṇa stage's 7.2.114 reads `Nijanta`; an āya aṅga mis-tagged as ṇijanta would change what later stages see without changing a current form. → Task 2 `aya_folds_into_the_dhatu_without_nijanta` (`DUpAya` folds, no `Nijanta`) beside the existing `sanadyanta_folds_nic_into_the_dhatu` (ṇic sets it).
5. **Homograph analyses and their order.** `daMSati`, `laRqati`, `laYjati`, `tuYjati` and the rest now have analyses that open with different optional-ṇic ids, one per row, and `trace_for` answers with the first analysis. → Task 3 `curadi_analyses_its_optional_nic_forms` (the opening ids, one per analysis, sorted and compared) and `curadi_analyses_its_asvadiya_forms` (the earlier row's analysis first for `pArayati`, `jayati`, `BaYjanti`, `aYjanti` and `vartatAm`).

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-prakriya/src/tinanta/sanadi.rs` | Task 2: module doc, `sanadi_pratyaya`, 10.0499, 2565, 2571, 3.1.28, the sanādi 6.1.73, 3.4.114's and 3.1.32's guards and 3.1.32's `Nijanta`, `with_aya`, three unit tests and three widened. Task 3: two tests gain rows |
| `crates/panini-prakriya/src/tinanta/anga.rs` | Task 2: `che_ca` extracted, 6.1.73's comment, its test's stage-scoped lookup |
| `crates/panini-prakriya/src/tinanta/mod.rs` | Task 2: `Tag::Aya` from `AYA` |
| `crates/panini-prakriya/src/term.rs` | Task 2: `Tag::Aya` |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Task 2: order (159), vikalpa and bars pins, the pin's doc |
| `crates/panini-data/src/lib.rs` | Task 2: `AYA`. Task 3: 61 rows, `OPTIONAL_NIC` (180) and its doc, `AYA`'s doc, the curādi row list, `Dhatu::pada`'s census, the helper, the marker, uniqueness and `AYA` tests, the row count. Task 4: one comment |
| `crates/panini/tests/paradigm/main.rs` | Task 3: vikalpa list, census, keys, the 10i paragraph, 10g's optional-ṇic witness test, the new 10i witness test, the pada-ambiguous comment and set. Task 4: audit-chain prose |
| `crates/panini/tests/paradigm/data/curadi.rs` | Task 3: 488 golden rows, 2928 alternates |
| `crates/panini/tests/trace/curadi.rs`, `crates/panini/tests/trace/juhotyadi.rs` | Task 3: module doc, five moved rosters, one renamed test, two new tests |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini-prakriya/src/tinanta/guna.rs`, the 10g and 10h specs | Task 4 |
| `AGENTS.md`, maybe `mise.toml` | Task 5 |

---

## Task 1: The worktree and baseline

**Files:** none. **Interfaces:** none.

- [ ] **Step 1: Create the worktree**

```bash
cd /workspace
git status --short                   # empty
git log --oneline -3 curadi-10i      # the plan commit, then a55d993 (spec), then 757b4fe
git checkout main                    # curadi-10i can be checked out in one worktree only
git log --oneline -1                 # 757b4fe, the 10h merge
git worktree add .worktrees/curadi-10i curadi-10i
cd .worktrees/curadi-10i
git branch --show-current            # curadi-10i
```

`/workspace` stays on `main` for the whole slice: Task 4's prior-trace dump builds the main engine from `/workspace/crates`.

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: everything passes at 21564 cells. `panini-prakriya` reports 427 tests, the `trace` binary 212, `paradigm` 26 and `panini-data` 27 (and `panini` 7, `roundtrip` 1, `panini-analyze` 8, `cli` 5, `panini-lipi` 6).

---

## Task 2: The engine — 10.0499, 2565, 2571, 3.1.28's āya, the sanādi 6.1.73

No curated root before this slice is āsvadīya, `pF`, `Guzi~r`, √dhūp or √vich, no row is in `OPTIONAL_NIC` under the three new ids or in `AYA`'s rows, and no curated code has a short vowel before `C`. So every golden and every prior trace is unchanged, and the suite is green at the end of the task. Task 4's prior-trace diff is the corpus-wide proof.

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (`AYA` after `JNAPADI`)
- Modify: `crates/panini-prakriya/src/term.rs` (`Tag::Aya` after `Mrj`)
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs` (`AYA` import; `Tag::Aya` after the `MRJ` block)
- Modify: `crates/panini-prakriya/src/tinanta/anga.rs` (`che_ca` before `ANGA_RULES`; 6.1.73's comment and apply; its test)
- Modify: `crates/panini-prakriya/src/tinanta/sanadi.rs` (module doc; `sanadi_pratyaya`; five rules; 3.4.114 and 3.1.32; tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (the three pins and their docs)

**Interfaces:**
- Produces:
  - `panini_data::AYA: [&str; 2]`;
  - `Tag::Aya`;
  - `pub(crate) fn anga::che_ca(p: &mut Prakriya) -> bool`;
  - `fn sanadi::sanadi_pratyaya(p: &Prakriya) -> Option<&Term>`;
  - the rule ids 10.0499, 2565, 2571 and 3.1.28, and a second 6.1.73;
  - the sanādi test helper `with_aya(root: &str, tags: &[Tag]) -> Prakriya`.
- Consumes:
  - `skip_nic` (10f), `optional_nic`;
  - the sanādi tests' `with_nic(root, nic)`, `optional_nic_dhatu(number, root, tag)` and `rule(id)`;
  - `terms::{word_chars, insert_char}`, `sound::is_hrasva`.

- [ ] **Step 1: The failing tests**

Create `/tmp/vidyut-full/slice10i/engine_10i.py` if it is missing (sha256 `e8d466c09cf67b8d062be984d32621de332cd656018e8d1374ce2e8721aca97d`). It holds the declarations, the code and the tests. `--tests-only` applies the declarations (`AYA`, `Tag::Aya`) and the tests, but no rule:

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10i's engine edits — 10.0499, 2565, 2571, 3.1.28's āya
and a sanādi 6.1.73 sharing the aṅga stage's apply — with their unit tests
and rule pins. Every `old` must occur exactly once in its file; nothing is
written if one fails. Run from the worktree root.
`--tests-only` applies the declarations (`Tag::Aya`, `AYA`) and the tests,
but no rule: TDD, see them fail first."""
import sys
TESTS_ONLY = '--tests-only' in sys.argv
DATA = 'crates/panini-data/src/lib.rs'
TERM = 'crates/panini-prakriya/src/term.rs'
MOD = 'crates/panini-prakriya/src/tinanta/mod.rs'
ANGA = 'crates/panini-prakriya/src/tinanta/anga.rs'
SANADI = 'crates/panini-prakriya/src/tinanta/sanadi.rs'
PINS = 'crates/panini-prakriya/src/tinanta/derivation_tests.rs'

DECLS = {DATA: [
("""pub const JNAPADI: RangeInclusive<&str> = "10.0118"..="10.0124";
""",
"""pub const JNAPADI: RangeInclusive<&str> = "10.0118"..="10.0124";

/// The rows 3.1.28 *gupūdhūpavicchipaṇipanibhya āyaḥ* gives the pratyaya
/// āya: `10.0303 DUpa~` and `10.0304 viCa~`, which take it where their
/// optional ṇic is not taken (*dhūpāyati*, *vicchāyati*). Keyed by
/// dhātupāṭha number, as `JNAPADI` is, because a stored code cannot decide
/// it: `10.0302 gupa~` stores as `gup`, as the bhvādi `gupU~` the sūtra
/// names does, and takes no āya. vidyut-prakriya keys the sūtra on five
/// upadeśas in any gaṇa (`gupU~`, `DUpa~`, `viCa~`, `paRa~\\`, `pana~\\`);
/// each joins this list when its row is curated. The engine's `derive` tags
/// a listed root `Tag::Aya`, which 3.1.28 reads.
pub const AYA: [&str; 2] = ["10.0303", "10.0304"];
"""),
], TERM: [
("""    /// entries, in `tinanta::sanadi` and `tinanta::guna`.
    Mrj,""",
"""    /// entries, in `tinanta::sanadi` and `tinanta::guna`.
    Mrj,
    /// 3.1.28 *gupūdhūpavicchipaṇipanibhya āyaḥ*: the dhātu takes the
    /// pratyaya āya. Set by `tinanta::derive` from the row NUMBER
    /// (`panini_data::AYA`), as `Mrj` is: curādi `10.0302 gupa~` stores as
    /// `gup`, as the bhvādi `gupU~` the sūtra names does, and takes no āya.
    /// A saṁjñā verdict, so no step is recorded. Read only by 3.1.28, in
    /// `tinanta::sanadi`, which adds āya where no ṇic was taken.
    Aya,"""),
]}

CODE = {MOD: [
("""use panini_data::{Dhatu, Gana, JNAPADI, Lakara, Pada, PadaAssignment, Purusha, Vacana};""",
"""use panini_data::{AYA, Dhatu, Gana, JNAPADI, Lakara, Pada, PadaAssignment, Purusha, Vacana};"""),
("""    if samjna::MRJ.contains(&dhatu.dhatupatha) {
        t.add(Tag::Mrj);
    }
""",
"""    if samjna::MRJ.contains(&dhatu.dhatupatha) {
        t.add(Tag::Mrj);
    }
    // 3.1.28's āya, likewise by row number (`AYA`).
    if AYA.contains(&dhatu.dhatupatha) {
        t.add(Tag::Aya);
    }
"""),
], ANGA: [
("""use crate::rule::{Rule, RuleKind};
use crate::term::Tag;""",
"""use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::term::Tag;"""),
("""pub(crate) static ANGA_RULES: &[Rule] = &[""",
"""/// 6.1.73 Ce ca, applied: the first short vowel before a `C` anywhere in
/// the word takes tuk after it (the aṅga stage's entry explains the
/// whole-word scan). One function behind two entries: `ANGA_RULES`', for
/// the laṅ aṭ, and `super::sanadi`'s, for a root-internal site, taken there
/// before guṇa. Shared, so the two cannot drift apart.
pub(crate) fn che_ca(p: &mut Prakriya) -> bool {
    let w = word_chars(p);
    let Some(pos) = (1..w.len()).find(|i| w[*i].2 == 'C' && is_hrasva(w[i - 1].2)) else {
        return false;
    };
    let (term, idx, _) = w[pos - 1];
    let before = p.snapshot();
    insert_char(p, term, idx + 1, 't');
    p.record("6.1.73", "Ce ca", before);
    true
}

pub(crate) static ANGA_RULES: &[Rule] = &["""),
("""    // Below the 6.4.71/6.4.72 āgama pair (6.4.72 sits between this rule
    // and 6.4.71 in the array), and 6.4.71 is what manufactures the whole
    // of this rule's precondition: the only short vowel any curated root
    // presents before a `C` is the aṭ-āgama laṅ prefixes onto a C-initial
    // aṅga. Outside laṅ the `C` is word-initial and this rule has nothing
    // to sit after, which is why √chid's and √chṛd's laṭ, loṭ and
    // vidhiliṅ cells never take it.""",
"""    // Below the 6.4.71/6.4.72 āgama pair (6.4.72 sits between this rule
    // and 6.4.71 in the array), and 6.4.71 is what manufactures this
    // entry's one site: the aṭ-āgama laṅ prefixes onto a C-initial aṅga.
    // Outside laṅ that `C` is word-initial and the rule has nothing to sit
    // after, which is why √chid's and √chṛd's laṭ, loṭ and vidhiliṅ cells
    // never take it. A root-internal site (√vich, `viC`) is the sanādi
    // stage's: its entry under this id takes it before guṇa can read the
    // root's `i` (`super::sanadi`), and this one then finds that `C` after
    // the `t` and declines."""),
("""    // The tuk lands in whichever term holds the short vowel — `AGAMA`, for
    // the laṅ aṭ that is this corpus's only site — because `word_chars`""",
"""    // The tuk lands in whichever term holds the short vowel — `AGAMA`, for
    // the laṅ aṭ that is this entry's only site — because `word_chars`"""),
("""    // augment is obligatory. Implement it when an upasarga or a preceding
    // pada enters scope — and note it would be this engine's eighth vikalpa
    // rule, so `exactly_the_pinned_vikalpa_rules_are_optional` must change
    // with it.
    Rule {
        id: "6.1.73",
        name: "Ce ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let w = word_chars(p);
            let Some(pos) = (1..w.len()).find(|i| w[*i].2 == 'C' && is_hrasva(w[i - 1].2)) else {
                return false;
            };
            let (term, idx, _) = w[pos - 1];
            let before = p.snapshot();
            insert_char(p, term, idx + 1, 't');
            p.record("6.1.73", "Ce ca", before);
            true
        },
    },""",
"""    // augment is obligatory. Implement it when an upasarga or a preceding
    // pada enters scope — and note it would be a vikalpa rule, so
    // `exactly_the_pinned_vikalpa_rules_are_optional` must change with it.
    Rule {
        id: "6.1.73",
        name: "Ce ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: che_ca,
    },"""),
], SANADI: [
("""//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, ṇic's
//! it-lopa (1.3.9), 3.4.114, 6.4.48, 7.2.116, 6.4.92, the vārttika 7.3.37.2,
//! 7.2.115, 6.1.78, 7.2.114, 7.3.86, 3.1.32 — opened by five vikalpas that
//! fork a root whose ṇic is optional into its ṇic and ṇic-less branches (the
//! dhātupāṭha gaṇasūtra 10.0498 and the Kaumudī's 2564, 2570, 2573.1,
//! 2573.3), a sixth (2573.2) that forks `pata`'s ṇic branch on its final
//! `a`, and three more dhātupāṭha gaṇasūtras: 10.0496 and 10.0497, which
//! settle an ākusmīya or ā-garvīya root's pada, and 10.0493, which credits a
//! jñapādi root's mit-tva, all before ṇic is added.""",
"""//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, 3.1.28's
//! āya where no ṇic is taken, ṇic's it-lopa (1.3.9), 3.4.114, 6.4.48,
//! 7.2.116, 6.4.92, the vārttika 7.3.37.2, 7.2.115, 6.1.78, 7.2.114, 6.1.73,
//! 7.3.86, 3.1.32 — opened by eight vikalpas that fork a root whose ṇic is
//! optional into its ṇic and ṇic-less branches (the dhātupāṭha gaṇasūtras
//! 10.0498 and 10.0499 and the Kaumudī's 2564, 2565, 2570, 2571, 2573.1,
//! 2573.3), a ninth (2573.2) that forks `pata`'s ṇic branch on its final
//! `a`, and three more dhātupāṭha gaṇasūtras: 10.0496 and 10.0497, which
//! settle an ākusmīya or ā-garvīya root's pada, and 10.0493, which credits a
//! jñapādi root's mit-tva, all before ṇic is added."""),
("""//! is `[AGAMA, ABHYASA, ANGA, ṇic]`, ṇic at `NIC`; 3.1.32 folds ṇic into
//! `ANGA` and removes it, so `super::samjna` starts on the same
//! `[AGAMA, ABHYASA, ANGA]` every other gaṇa does. From there on the aṅga is
//! an ordinary i-final dhātu (`cori`), and 7.3.84 then 6.1.78 make `coray-`
//! exactly as they make √nī's `nay-`. See `super::terms`.""",
"""//! is `[AGAMA, ABHYASA, ANGA, ṇic]`, ṇic (or 3.1.28's āya) at `NIC`; 3.1.32
//! folds it into `ANGA` and removes it, so `super::samjna` starts on the
//! same `[AGAMA, ABHYASA, ANGA]` every other gaṇa does. From there on a
//! ṇijanta aṅga is an ordinary i-final dhātu (`cori`), and 7.3.84 then
//! 6.1.78 make `coray-` exactly as they make √nī's `nay-`; an āya aṅga is
//! a-final (`DUpAya`) and meets śap as an adanta root does. See
//! `super::terms`."""),
("""//! Every rule self-guards: the five optional-ṇic vikalpas on the row's
//! `OPTIONAL_NIC` entry, 2573.2 on `pata`'s, 10.0496 on `Tag::Akusmiya`, 10.0497 on
//! `Tag::AaGarviya`, 10.0493 on `Tag::Mit`, 3.1.25 on `Tag::Curadi`, the
//! rest on ṇic being present (6.4.48 on an `a`-final aṅga as well, 6.4.92 on
//! `Tag::Mit`, 7.3.37.2 on √dhū's and √prī's text, 7.2.115 on an ac-final
//! aṅga, 6.1.78 on an ec-final one, 7.2.114 on `Tag::Mrj`, and 7.2.116 and
//! 7.3.86 decline on 6.4.48's `Tag::AtLopa`).""",
"""//! Every rule self-guards: the eight optional-ṇic vikalpas on the row's
//! `OPTIONAL_NIC` entry, 2573.2 on `pata`'s, 10.0496 on `Tag::Akusmiya`,
//! 10.0497 on `Tag::AaGarviya`, 10.0493 on `Tag::Mit`, 3.1.25 on
//! `Tag::Curadi`, 3.1.28 on `Tag::Aya` with no ṇic taken, 6.1.73 on its own
//! saṁhitā condition, and the rest on the pratyaya at `NIC`: 3.4.114 and
//! 3.1.32 on ṇic or āya, 6.4.48, 7.2.114 and 7.3.86 on its being
//! ārdhadhātuka, the others on ṇic's ṇit (6.4.48 on an `a`-final aṅga as
//! well, 6.4.92 on `Tag::Mit`, 7.3.37.2 on √dhū's and √prī's text, 7.2.115
//! on an ac-final aṅga, 6.1.78 on an ec-final one, 7.2.114 on `Tag::Mrj`,
//! and 7.2.116 and 7.3.86 decline on 6.4.48's `Tag::AtLopa`)."""),
("""use crate::tinanta::sound::{guna_of, hrasva_of, vrddhi_of};""",
"""use crate::tinanta::anga::che_ca;
use crate::tinanta::sound::{guna_of, hrasva_of, vrddhi_of};"""),
("""    p.record(id, name, before);
    true
}
""",
"""    p.record(id, name, before);
    true
}

/// The sanādi pratyaya at `NIC`, once it is ready to fold into the dhātu:
/// ṇic after its it-lopa (ṇit, `Tag::Rit`), or 3.1.28's āya, which has no it
/// to lose. Read by 3.4.114 and 3.1.32, the two rules both pratyayas meet.
/// āya is known by its text, as 1.3.9 knows ṇic by `Ric`.
fn sanadi_pratyaya(p: &Prakriya) -> Option<&Term> {
    p.terms
        .get(NIC)
        .filter(|t| t.has(Tag::Rit) || t.text == "Aya")
}
"""),
("""    // it (`DP("10.0498")`), like 10.0496 and 10.0497. First of the five
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
    },""",
"""    // it (`DP("10.0498")`), like 10.0496 and 10.0497. First of the eight
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
    // 10.0499 ā svadaḥ sakarmakāt: the āsvadīya — the curādi rows from
    // `10.0279 grasa~` to `10.0337 svAda~` — take ṇic optionally
    // (*grāsayati* / *grasati*). A dhātupāṭha gaṇasūtra, numbered as
    // vidyut-prakriya numbers it (`DP("10.0499")`), like 10.0498. Its
    // *sakarmakāt* (only where the root takes an object) is not modelled:
    // vidyut makes ṇic optional on every row of the range, and so does this
    // engine. Second, right after 10.0498, because vidyut checks both
    // antargaṇas before any marker (`dhatu_karya.rs`): the idit `tuji~`,
    // `ahi~` and the rest, the udit `vftu~` and `vfDu~` and the īdit
    // `pUrI~` are all 10.0499's. Keyed on the row's `OPTIONAL_NIC` entry.
    Rule {
        id: "10.0499",
        name: "A svadaH sakarmakAt",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "10.0499", "A svadaH sakarmakAt"),
    },"""),
("""    // *cintati*). One of the five vikalpa rules that decide ṇic before the""",
"""    // *cintati*). One of the eight vikalpa rules that decide ṇic before the"""),
("""        apply: |p| skip_nic(p, "2564", "iditkaraRaM RicaH pAkzikatve liNgam"),
    },""",
"""        apply: |p| skip_nic(p, "2564", "iditkaraRaM RicaH pAkzikatve liNgam"),
    },
    // Kaumudī 2565: `10.0022 pF`'s ṇic is optional (*pārayati* / *parati*).
    // The Kaumudī reads the long vowel of its upadeśa as the sign, as 2564
    // reads the idit marker. vidyut-prakriya keys it on an F-final root,
    // after idit (`dhatu_karya.rs`); `10.0346 jF` is ādhṛṣīya, so 10.0498
    // takes it first, and `pF` is the one row 2565 reaches. Keyed on the
    // row's `OPTIONAL_NIC` entry.
    Rule {
        id: "2565",
        name: "dIrGoccAraRaM RicaH pAkzikatve liNgam",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "2565", "dIrGoccAraRaM RicaH pAkzikatve liNgam"),
    },"""),
("""        apply: |p| skip_nic(p, "2570", "YitkaraRasAmarTyAdasya RijvikalpaH"),
    },""",
"""        apply: |p| skip_nic(p, "2570", "YitkaraRasAmarTyAdasya RijvikalpaH"),
    },
    // Kaumudī 2571: `10.0251 Guzi~r`'s ṇic is not nitya (*ghoṣayati* /
    // *ghoṣati*). The Kaumudī reads that off the exclusion in 7.2.23
    // *ghuṣir aviśabdane* (*niṣedhāl liṅgāt*), as 2564 reads the idit
    // marker. vidyut-prakriya keys it on this one upadeśa. Its `i~r` is the
    // irit marker, not idit, so 2564 never reaches it. Keyed on the row's
    // `OPTIONAL_NIC` entry.
    Rule {
        id: "2571",
        name: "nizeDAlliNgAdanityo'sya Ric",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "2571", "nizeDAlliNgAdanityo'sya Ric"),
    },"""),
("""                "satyApapASarUpavIRAtUlaSlokasenAlomatvacavarmavarRacUrRacurAdiByo Ric",
                before,
            );
            true
        },
    },""",
"""                "satyApapASarUpavIRAtUlaSlokasenAlomatvacavarmavarRacUrRacurAdiByo Ric",
                before,
            );
            true
        },
    },
    // 3.1.28 gupūdhūpavicchipaṇipanibhya āyaḥ: √dhūp and √vich take the
    // pratyaya āya (*dhūpāyati*, *vicchāyati*). Only where ṇic was not
    // taken: on the ṇic branch 3.1.25 has put ṇic at `NIC`, and this
    // declines. vidyut-prakriya adds āya at this point, after its
    // optional-ṇic fork (trace `10.0499 → 3.1.28 → 3.4.114 → 3.1.32`).
    // Guarded on `Tag::Aya` (`panini_data::AYA`): a stored code cannot
    // decide it. Obligatory here: 3.1.31 *āyādaya ārdhadhātuke vā* makes
    // it optional only before an ārdhadhātuka ending, and no ārdhadhātuka
    // lakāra is in scope. āya has no it, so 1.3.9 passes it by; 3.4.114
    // makes it ārdhadhātuka and 3.1.32 folds it into the dhātu (`DUpAya`).
    Rule {
        id: "3.1.28",
        name: "gupUDUpavicCipaRipaniBya AyaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Aya) || p.terms.get(NIC).is_some() {
                return false;
            }
            let before = p.snapshot();
            let mut aya = Term::new("Aya");
            aya.add(Tag::Pratyaya);
            p.terms.push(aya);
            p.record("3.1.28", "gupUDUpavicCipaRipaniBya AyaH", before);
            true
        },
    },"""),
("""    // 3.4.114 ārdhadhātukaṃ śeṣaḥ: ṇic is neither tiṅ nor śit, so it is
    // ārdhadhātuka. Read by 7.3.86 below, whose *sārvadhātukārdhadhātukayoḥ*
    // is inherited from 7.3.84.
    Rule {
        id: "3.4.114",
        name: "ArDaDAtukaM SezaH",
        kind: RuleKind::Samjna,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) {
                return false;
            }""",
"""    // 3.4.114 ārdhadhātukaṃ śeṣaḥ: ṇic and āya are neither tiṅ nor śit, so
    // they are ārdhadhātuka. Read by 7.3.86 below, whose
    // *sārvadhātukārdhadhātukayoḥ* is inherited from 7.3.84.
    Rule {
        id: "3.4.114",
        name: "ArDaDAtukaM SezaH",
        kind: RuleKind::Samjna,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if sanadi_pratyaya(p).is_none() {
                return false;
            }"""),
("""    // 7.3.86 pugantalaghūpadhasya ca, before ṇic: guṇa of a laghu ik upadhā""",
"""    // 6.1.73 che ca, before ṇic or āya: √vich's `i` takes tuk here, before
    // the sanādi 7.3.86 below can read it as a laghu upadhā (`viC` →
    // `vitC`, so *vicchayati*, not *vechayati*; 8.4.40 later makes the `t`
    // a `c`). vidyut-prakriya credits it at this point ("tuk-Agama can block
    // guna", `angasya.rs`). A second entry under the aṅga stage's id, with
    // that entry's very apply (`super::anga::che_ca`): the whole-word scan,
    // not an aṅga-local copy, guarded on nothing but the sūtra's own
    // condition. On √vich's laṅ cells the aṅga stage's entry then finds the
    // `C` after the `t` and declines.
    Rule {
        id: "6.1.73",
        name: "Ce ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: che_ca,
    },
    // 7.3.86 pugantalaghūpadhasya ca, before ṇic: guṇa of a laghu ik upadhā"""),
("""    // 3.1.32 sanādyantā dhātavaḥ: root + ṇic is a dhātu. Fold ṇic's text into
    // `ANGA` and remove its term, so every later stage sees the three-slot
    // layout and an i-final dhātu. `Tag::Dhatu` stays; `Tag::Nijanta`
    // records that the final `i` is ṇic's.
    Rule {
        id: "3.1.32",
        name: "sanAdyantA DAtavaH",
        kind: RuleKind::Samjna,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) {
                return false;
            }
            let before = p.snapshot();
            let nic = p.terms.remove(NIC);
            p.terms[ANGA].text.push_str(&nic.text);
            p.terms[ANGA].add(Tag::Nijanta);""",
"""    // 3.1.32 sanādyantā dhātavaḥ: root + ṇic, or root + āya, is a dhātu.
    // Fold the pratyaya's text into `ANGA` and remove its term, so every
    // later stage sees the three-slot layout and an i-final (or, with āya,
    // a-final) dhātu. `Tag::Dhatu` stays; `Tag::Nijanta` records that the
    // final `i` is ṇic's, so an āya aṅga does not get it.
    Rule {
        id: "3.1.32",
        name: "sanAdyantA DAtavaH",
        kind: RuleKind::Samjna,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if sanadi_pratyaya(p).is_none() {
                return false;
            }
            let before = p.snapshot();
            let pratyaya = p.terms.remove(NIC);
            p.terms[ANGA].text.push_str(&pratyaya.text);
            if pratyaya.has(Tag::Rit) {
                p.terms[ANGA].add(Tag::Nijanta);
            }"""),
]}

TESTS = {ANGA: [
("""    fn che_ca_inserts_tuk_only_after_a_short_vowel() {
        let rule = rules().find(|r| r.id == "6.1.73").unwrap();

        // The one site this corpus reaches: 6.4.71's aṭ before a C-initial
        // aṅga.""",
"""    fn che_ca_inserts_tuk_only_after_a_short_vowel() {
        // This stage's entry: the sanādi stage has its own under this id.
        let rule = ANGA_RULES.iter().find(|r| r.id == "6.1.73").unwrap();

        // This entry's one site: 6.4.71's aṭ before a C-initial
        // aṅga."""),
("""        // to attach to. This is every laṭ, loṭ and vidhiliṅ cell of √chid
        // and √chṛd, and it is why the two new sūtras are laṅ-only.""",
"""        // to attach to. This is every laṭ, loṭ and vidhiliṅ cell of √chid
        // and √chṛd, and it is why this entry fires in laṅ only (√vich's
        // root-internal site, in every lakāra, is the sanādi entry's)."""),
], SANADI: [
("""    #[test]
    fn ardhadhatuka_sesah_tags_nic_only() {
        let mut p = with_nic("cur", Some(&[Tag::Rit]));
        assert!((rule("3.4.114").apply)(&mut p));
        assert!(p.terms[NIC].has(Tag::Ardhadhatuka));""",
"""    /// `root` followed by 3.1.28's āya, as that rule leaves it, carrying
    /// `tags` too.
    fn with_aya(root: &str, tags: &[Tag]) -> Prakriya {
        let mut p = with_nic(root, None);
        let mut aya = Term::new("Aya");
        aya.add(Tag::Pratyaya);
        for tag in tags {
            aya.add(*tag);
        }
        p.terms.push(aya);
        p
    }

    #[test]
    fn ardhadhatuka_sesah_tags_the_sanadi_pratyaya_only() {
        let mut p = with_nic("cur", Some(&[Tag::Rit]));
        assert!((rule("3.4.114").apply)(&mut p));
        assert!(p.terms[NIC].has(Tag::Ardhadhatuka));
        // 3.1.28's āya, which has no it and so no `Tag::Rit`.
        let mut p = with_aya("DUp", &[]);
        assert!((rule("3.4.114").apply)(&mut p));
        assert!(p.terms[NIC].has(Tag::Ardhadhatuka));"""),
("""    #[test]
    fn sanadyanta_folds_nic_into_the_dhatu() {""",
"""    #[test]
    fn aya_follows_a_tagged_root_that_took_no_nic() {
        for root in ["DUp", "viC"] {
            let mut p = with_nic(root, None);
            p.terms[ANGA].add(Tag::Aya);
            assert!((rule("3.1.28").apply)(&mut p), "{root}");
            assert_eq!(p.terms[NIC].text, "Aya");
            assert!(p.terms[NIC].has(Tag::Pratyaya));
            assert!(!p.terms[NIC].has(Tag::Rit), "āya is not ṇit");
            assert_eq!(p.terms[ANGA].text, root);
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["3.1.28"]);
        }
        // Tagged, but ṇic was taken: that branch is the ṇic one.
        let mut p = with_nic("DUp", Some(&[Tag::Rit]));
        p.terms[ANGA].add(Tag::Aya);
        assert!(!(rule("3.1.28").apply)(&mut p));
        assert_eq!(p.terms[NIC].text, "i");
        // Untagged: curādi √gup (`10.0302 gupa~`) takes no āya.
        let mut p = with_nic("gup", None);
        assert!(!(rule("3.1.28").apply)(&mut p));
        assert_eq!(p.terms.len(), NIC);
        assert!(p.log.is_empty());
    }

    #[test]
    fn aya_folds_into_the_dhatu_without_nijanta() {
        let mut p = with_nic("DUp", None);
        p.terms[ANGA].add(Tag::Aya);
        assert!((rule("3.1.28").apply)(&mut p));
        assert!(!(rule("1.3.9").apply)(&mut p), "āya has no it");
        assert!((rule("3.4.114").apply)(&mut p));
        assert!((rule("3.1.32").apply)(&mut p));
        assert_eq!(p.terms.len(), NIC, "the āya term is gone");
        assert_eq!(p.terms[ANGA].text, "DUpAya");
        assert!(p.terms[ANGA].has(Tag::Dhatu));
        assert!(!p.terms[ANGA].has(Tag::Nijanta), "an āya aṅga is no ṇijanta");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["3.1.28", "3.4.114", "3.1.32"]);
    }

    #[test]
    fn che_ca_gives_vich_tuk_before_guna_can_read_its_upadha() {
        // Before ṇic and before āya alike: tuk after the short `i`, and the
        // sanādi 7.3.86 then finds `t`, not a laghu ik, at the upadhā.
        for mut p in [
            with_nic("viC", Some(&[Tag::Rit, Tag::Ardhadhatuka])),
            with_aya("viC", &[Tag::Ardhadhatuka]),
        ] {
            assert!((rule("6.1.73").apply)(&mut p));
            assert_eq!(p.terms[ANGA].text, "vitC");
            assert!(!(rule("7.3.86").apply)(&mut p));
            assert_eq!(p.terms[ANGA].text, "vitC");
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["6.1.73"]);
        }
        // Without the tuk, 7.3.86 would read the `i`: *vechayati*.
        let mut p = with_nic("viC", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!((rule("7.3.86").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "veC");
        // No short vowel right before a `C`: a consonant (mUrC), nothing at
        // all (`C` word-initial, Cid), or no `C` (cur).
        for root in ["mUrC", "Cid", "cur"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("6.1.73").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
    }

    #[test]
    fn sanadyanta_folds_nic_into_the_dhatu() {"""),
("""        // `10.0368 zad`: inside 10.0498's range but uncurated, so
        // the verdict is keyed on the table, not on position alone.
        for id in ["10.0498", "2564", "2570", "2573.1", "2573.3"] {""",
"""        // `10.0368 zad`: inside 10.0498's range but uncurated, so
        // the verdict is keyed on the table, not on position alone.
        for id in [
            "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3",
        ] {"""),
("""    fn the_optional_nic_rules_are_vikalpas_that_bar_nic() {
        for id in ["10.0498", "2564", "2570", "2573.1", "2573.3"] {""",
"""    fn the_optional_nic_rules_are_vikalpas_that_bar_nic() {
        for id in [
            "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3",
        ] {"""),
], PINS: [
("""/// Slice 10f opened the list with five Kaumudī vikalpa rules, ahead of
/// 10.0496 (slice 10h puts the gaṇasūtra 10.0498 *ā dhṛṣād vā* first of
/// all, the same fork for the ādhṛṣīya rows), because vidyut-prakriya decides ṇic before the pada gaṇasūtras:""",
"""/// Slice 10f opened the list with five Kaumudī vikalpa rules, ahead of
/// 10.0496 (slice 10h puts the gaṇasūtra 10.0498 *ā dhṛṣād vā* first of
/// all, the same fork for the ādhṛṣīya rows, and slice 10i 10.0499 *ā
/// svadaḥ sakarmakāt* second, for the āsvadīya, and the Kaumudī's 2565 and
/// 2571 beside 2564 and 2570, in vidyut's check order), because
/// vidyut-prakriya decides ṇic before the pada gaṇasūtras:"""),
("""/// `tinanta/adesha.rs`. The ids here that are not Aṣṭādhyāyī sūtras are the
/// four gaṇasūtras (10.0493, 10.0496, 10.0497, 10.0498), the five Kaumudī ids
/// and the vārttika 7.3.37.2.""",
"""/// `tinanta/adesha.rs`. The ids here that are not Aṣṭādhyāyī sūtras are the
/// five gaṇasūtras (10.0493 and 10.0496 to 10.0499), the seven Kaumudī ids
/// and the vārttika 7.3.37.2.
///
/// Slice 10i adds 3.1.28 *gupūdhūpavicchipaṇipanibhya āyaḥ* right after
/// 3.1.25, where vidyut credits it: on a branch with no ṇic, √dhūp's and
/// √vich's āya stands where ṇic would, and 3.4.114 and 3.1.32 treat it as
/// they treat ṇic. It also adds a second 6.1.73 *che ca* right before the
/// sanādi 7.3.86, whose guṇa √vich's tuk must block."""),
("""        "10.0498", "2564", "2570", "2573.1", "2573.3", "2573.2", "10.0496", "10.0497", "10.0493",
        "3.1.25", "1.3.9", "3.4.114", "6.4.48", "7.2.116", "6.4.92", "7.3.37.2", "7.2.115",
        "6.1.78", "7.2.114", "7.3.86", "3.1.32", "1.3.12", "1.3.66", "1.3.72", "1.3.74", "1.3.78",""",
"""        "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3", "2573.2",
        "10.0496", "10.0497", "10.0493", "3.1.25", "3.1.28", "1.3.9", "3.4.114", "6.4.48",
        "7.2.116", "6.4.92", "7.3.37.2", "7.2.115", "6.1.78", "7.2.114", "6.1.73", "7.3.86",
        "3.1.32", "1.3.12", "1.3.66", "1.3.72", "1.3.74", "1.3.78","""),
("""        "10.0498", "2564", "2570", "2573.1", "2573.3", "2573.2", "7.3.37.2", "7.1.35", "3.4.111",
        "7.3.86", "6.4.117", "6.4.116", "6.4.115", "6.4.43", "6.4.107", "8.2.74", "8.2.75",
        "8.4.65", "8.4.56",""",
"""        "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3", "2573.2",
        "7.3.37.2", "7.1.35", "3.4.111", "7.3.86", "6.4.117", "6.4.116", "6.4.115", "6.4.43",
        "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56","""),
("""/// Ids are not unique in the pipeline (7.3.84, 1.2.4, 1.3.9, 6.1.78 and
/// 7.2.114 each appear twice, 7.3.86 three times), so "runs after" means some
/// later occurrence.""",
"""/// Ids are not unique in the pipeline (7.3.84, 1.2.4, 1.3.9, 6.1.78, 7.2.114
/// and 6.1.73 each appear twice, 7.3.86 three times), so "runs after" means
/// some later occurrence."""),
("""        ("10.0498", &["3.1.25", "2573.2"][..]),
        ("2564", &["3.1.25", "2573.2"][..]),
        ("2570", &["3.1.25", "2573.2"][..]),""",
"""        ("10.0498", &["3.1.25", "2573.2"][..]),
        ("10.0499", &["3.1.25", "2573.2"][..]),
        ("2564", &["3.1.25", "2573.2"][..]),
        ("2565", &["3.1.25", "2573.2"][..]),
        ("2570", &["3.1.25", "2573.2"][..]),
        ("2571", &["3.1.25", "2573.2"][..]),"""),
]}

def merge(*ds):
    out = {}
    for d in ds:
        for f, e in d.items():
            out.setdefault(f, []).extend(e)
    return out

FILES = merge(DECLS, TESTS) if TESTS_ONLY else merge(DECLS, CODE, TESTS)
out, bad = {}, []
for path, edits in FILES.items():
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
print(f"applied {sum(len(e) for e in FILES.values())} edits")
```

```bash
python3 /tmp/vidyut-full/slice10i/engine_10i.py --tests-only      # applied 14 edits
mise run fmt
```

The tests:
- **`sanadi.rs`:**
  - `with_aya(root, tags)`: a root followed by 3.1.28's āya, `Tag::Pratyaya` plus `tags`.
  - `ardhadhatuka_sesah_tags_the_sanadi_pratyaya_only` (renamed from `ardhadhatuka_sesah_tags_nic_only`, which stops being true): 3.4.114 tags ṇit ṇic and āya, and still declines with nothing at `NIC` or a tiṅ ending there.
  - `aya_follows_a_tagged_root_that_took_no_nic`: under `Tag::Aya`, `DUp` and `viC` with no ṇic get āya at `NIC` (`Tag::Pratyaya`, not ṇit); declines with ṇic present and on untagged `gup`.
  - `aya_folds_into_the_dhatu_without_nijanta`: 3.1.28, then 1.3.9 declines, 3.4.114 and 3.1.32 fold `DUpAya`, keeping `Tag::Dhatu`, without `Tag::Nijanta`.
  - `che_ca_gives_vich_tuk_before_guna_can_read_its_upadha`: before ṇic and before āya, `viC` → `vitC`, after which the sanādi 7.3.86 declines; without the tuk 7.3.86 gives `veC`; declines on `mUrC`, `Cid`, `cur`.
  - `an_optional_nic_rule_declines_off_its_rows` and `the_optional_nic_rules_are_vikalpas_that_bar_nic` gain 10.0499, 2565 and 2571.
- **`anga.rs`:** `che_ca_inserts_tuk_only_after_a_short_vowel` looks the rule up in `ANGA_RULES`, and its comments name this entry's one site.
- **`derivation_tests.rs`:** the order pin (159), the vikalpa pin and the bars pin (the three new vikalpas), the bars doc's "ids are not unique" list (6.1.73), and the order pin's doc (the gaṇasūtra and Kaumudī counts, a 10i paragraph).

- [ ] **Step 2: Run them to see them fail**

Run: `mise exec -- cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .*FAILED|test result: FAILED"`
Expected: `panini-prakriya` `421 passed; 9 failed`, every other binary passing:

- `tinanta::derivation_tests::exactly_the_pinned_bars`
- `tinanta::derivation_tests::exactly_the_pinned_vikalpa_rules_are_optional`
- `tinanta::derivation_tests::tinanta_rule_order_is_pinned`
- `tinanta::sanadi::tests::an_optional_nic_rule_declines_off_its_rows`
- `tinanta::sanadi::tests::ardhadhatuka_sesah_tags_the_sanadi_pratyaya_only`
- `tinanta::sanadi::tests::aya_folds_into_the_dhatu_without_nijanta`
- `tinanta::sanadi::tests::aya_follows_a_tagged_root_that_took_no_nic`
- `tinanta::sanadi::tests::che_ca_gives_vich_tuk_before_guna_can_read_its_upadha`
- `tinanta::sanadi::tests::the_optional_nic_rules_are_vikalpas_that_bar_nic`

`che_ca_inserts_tuk_only_after_a_short_vowel` passes already: with no sanādi 6.1.73 yet, the stage-scoped lookup finds the same entry `rules()` did.

- [ ] **Step 3: The change**

```bash
git checkout -- crates
python3 /tmp/vidyut-full/slice10i/engine_10i.py      # applied 34 edits
mise run fmt
```

- [ ] **Step 4: Run the tests and the suite**

```bash
mise exec -- cargo test -q -p panini-prakriya 2>&1 | grep -E "FAILED|test result"      # 430 passed
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS, with `panini-prakriya` 430, `trace` 212, `paradigm` 26, `panini-data` 27.

- [ ] **Step 5: Commit**

```bash
mise run lint
git branch --show-current      # curadi-10i
git add -A
git commit -m "feat(sanadi): 10.0499, 2565, 2571, 3.1.28 āya, a sanādi 6.1.73

The āsvadīya's gaṇasūtra joins the optional-ṇic forks after 10.0498, and
Kaumudī 2565 and 2571 join beside 2564 and 2570. Where √dhūp and √vich
take no ṇic, 3.1.28 gives them āya (Tag::Aya, from panini_data::AYA), which
3.4.114 and 3.1.32 treat as they treat ṇic, without Nijanta. 6.1.73's apply
is shared with a second entry before the sanādi 7.3.86, so √vich's tuk
blocks guṇa. No curated row reaches any of it yet."
```

---

## Task 3: The rows, their goldens, and every assertion they move

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini-prakriya/src/tinanta/sanadi.rs` (two tests)
- Modify: `crates/panini/tests/paradigm/main.rs`, `crates/panini/tests/paradigm/data/curadi.rs`
- Modify: `crates/panini/tests/trace/curadi.rs`, `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes:
  - Task 2's rules, `AYA` and `Tag::Aya`;
  - 10f's `OPTIONAL_NIC`, `optional_nic`, `Dhatu::padas`;
  - the trace helpers `at`, `credited` and `rows_crediting` (10h);
  - `panini_prakriya::derive`, `panini::Panini::check`.
- Produces: the totals Task 4 documents. Those are 422 roots, 25956 cells and 36416 forms, 10460 alternates, 1047 pada-ambiguous surfaces, 299 both-pada roots, and 260 `Nic` rows that credit 1.3.74.

Each script below asserts that every `old` occurs exactly once and writes nothing otherwise. If one prints `not applied:`, nothing was written: stop and report, rather than editing around it.

- [ ] **Step 1: The paradigm assertions (failing)**

Create `/tmp/vidyut-full/slice10i/paradigm_10i.py` if it is missing (sha256 `c133b16249893fc5365f6e7f4f246938deec05b8a99beb0d04a398aa1cacd6c3`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10i's edits to crates/panini/tests/paradigm/main.rs (the
census, ALTERNATES keys, docs, the vikalpa list, and 10g's optional-ṇic
witness test, whose homographs the slice grows). Every `old` must occur
exactly once; nothing is written if one fails. Run from the worktree root."""
import sys
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
E = [
("""const VIKALPA_RULES: &[&str] = &[
    "10.0498", "2564", "2570", "2573.1", "2573.3", "2573.2", "7.3.37.2", "7.1.35", "3.4.111",
    "7.3.86", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56", "6.4.115", "6.4.117", "6.4.116",
    "6.4.43",
];""",
"""const VIKALPA_RULES: &[&str] = &[
    "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3", "2573.2", "7.3.37.2",
    "7.1.35", "3.4.111", "7.3.86", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56", "6.4.115",
    "6.4.117", "6.4.116", "6.4.43",
];"""),
("""/// `ALTERNATES` is otherwise 7532 bare strings,""",
"""/// `ALTERNATES` is otherwise 10460 bare strings,"""),
("""/// naming 7.3.86 does not by itself mean the rule was optional. Forty-eight of the 55 `7.3.86+8.4.56` keys are the mandatory firing (ten juhotyādi: 3e's laṅ eka cells and 3f's √kit and √dhiṣ ones; thirty-eight curādi: slice 10a's √cur laṅ and vidhiliṅ prathama eka, and slice 10g's √div and √śṛdh and slice 10h's sixteen laghu-ik ādhṛṣīya rows on their ṇic branch, where the firing is the sanādi entry before ṇic; the other seven are the tanādi vikalpa), and so is the 7.3.86 of the one `7.3.86+8.2.75` key (√kit's `acikeH`), of every `7.3.86+7.1.35`/`7.3.86+7.1.35+8.4.56` key (the same curādi roots' loṭ tātaṅ cells — the only keys where 7.3.86 precedes 7.1.35, because the sanādi stage runs first), and of every `2570+…7.3.86…` and `10.0498+…7.3.86…` key (a ṇic-less root's guṇa).""",
"""/// naming 7.3.86 does not by itself mean the rule was optional. Seventy of the 77 `7.3.86+8.4.56` keys are the mandatory firing (ten juhotyādi: 3e's laṅ eka cells and 3f's √kit and √dhiṣ ones; sixty curādi: slice 10a's √cur laṅ and vidhiliṅ prathama eka, and slice 10g's √div and √śṛdh, slice 10h's sixteen laghu-ik ādhṛṣīya rows and slice 10i's eleven laghu-ik rows on their ṇic branch, where the firing is the sanādi entry before ṇic; the other seven are the tanādi vikalpa), and so is the 7.3.86 of the one `7.3.86+8.2.75` key (√kit's `acikeH`), of every `7.3.86+7.1.35`/`7.3.86+7.1.35+8.4.56` key (the same curādi roots' loṭ tātaṅ cells — the only keys where 7.3.86 precedes 7.1.35, because the sanādi stage runs first), and of every `2570+…7.3.86…`, `2571+…7.3.86…`, `10.0498+…7.3.86…` and `10.0499+…7.3.86…` key (a ṇic-less root's guṇa)."""),
("""/// 21564 cells total (2396 root×lakāra blocks × 9), of which 16072 hold exactly one form, 4472 hold two, 525 hold three (""",
"""/// 25956 cells total (2884 root×lakāra blocks × 9), of which 18268 hold exactly one form, 6424 hold two, 525 hold three ("""),
("""/// and √prī's ṇic branch forking again on 7.3.37.2's nuk), 237 hold four (piṣ's loṭ madhyama eka, the deepest""",
"""/// and √prī's ṇic branch forking again on 7.3.37.2's nuk, and slice 10i's
/// sixty-one add 1952 two-form cells, a ṇic and a ṇic-less reading each), 359 hold four (piṣ's loṭ madhyama eka, the deepest"""),
("""/// prathama eka, the same way; and — new in slice 10h — forty-eight of the
/// fifty ādhṛṣīya rows', the same way), and""",
"""/// prathama eka, the same way; and — new in slice 10h — forty-eight of the
/// fifty ādhṛṣīya rows', the same way; and — new in slice 10i — the
/// sixty-one āsvadīya, √pṝ and √ghuṣ rows', the same way), and"""),
("""/// 241
/// hold six (""",
"""/// 363
/// hold six ("""),
("""/// ādhṛṣīya rows', the same way, and √dhū's and √prī's laṅ and vidhiliṅ
/// parasmaipada prathama eka, three readings × 8.4.56), one — new in""",
"""/// ādhṛṣīya rows', the same way, and √dhū's and √prī's laṅ and vidhiliṅ
/// parasmaipada prathama eka, three readings × 8.4.56; and — new in slice
/// 10i — the sixty-one rows' loṭ parasmaipada prathama and madhyama eka,
/// the same way), one — new in"""),
("""/// itself has 7532 rows, keyed 536 `8.4.56`, 528 `7.1.35`, 528 `7.1.35+8.4.56`,""",
"""/// itself has 10460 rows, keyed 636 `8.4.56`, 628 `7.1.35`, 628 `7.1.35+8.4.56`,"""),
("""/// `7.3.86+6.4.107`, 55 `7.3.86+8.4.56` (forty-eight of them name the MANDATORY""",
"""/// `7.3.86+6.4.107`, 77 `7.3.86+8.4.56` (seventy of them name the MANDATORY"""),
("""and thirty-eight curādi, slice 10a's √cur laṅ and vidhiliṅ prathama eka and slice 10g's √div and √śṛdh and slice 10h's sixteen laghu-ik ādhṛṣīya rows' ṇic-branch ones,""",
"""and sixty curādi, slice 10a's √cur laṅ and vidhiliṅ prathama eka and slice 10g's √div and √śṛdh, slice 10h's sixteen laghu-ik ādhṛṣīya rows' and slice 10i's eleven laghu-ik rows' ṇic-branch ones,"""),
("""/// `7.1.35+6.4.116+8.4.56`, 9 `6.4.43` and 1 `6.4.43+8.4.56` (slice 3f3's √jan), 38 `7.3.86+7.1.35` and 38
/// `7.3.86+7.1.35+8.4.56` (slice 10a's √cur, its sanādi 7.3.86 ahead of 7.1.35, and slice 10g's √śṛdh and
/// √div and slice 10h's sixteen laghu-ik ādhṛṣīya rows on their ṇic branch), and slice 10f's twenty-one""",
"""/// `7.1.35+6.4.116+8.4.56`, 9 `6.4.43` and 1 `6.4.43+8.4.56` (slice 3f3's √jan), 60 `7.3.86+7.1.35` and 60
/// `7.3.86+7.1.35+8.4.56` (slice 10a's √cur, its sanādi 7.3.86 ahead of 7.1.35, and slice 10g's √śṛdh and
/// √div, slice 10h's sixteen laghu-ik ādhṛṣīya rows and slice 10i's eleven laghu-ik rows on their ṇic
/// branch), and slice 10f's twenty-one"""),
("""/// `7.3.86+8.4.56`, `7.3.86+7.1.35` and `7.3.86+7.1.35+8.4.56` — √kṛ (slice 8b) adds six more""",
"""/// `7.3.86+8.4.56`, `7.3.86+7.1.35` and `7.3.86+7.1.35+8.4.56`. Slice 10i
/// opens sixteen keys — 1764 `10.0499` and 360 `10.0499+7.3.86` (the second
/// key's 7.3.86 the MANDATORY guṇa of the ten laghu-ik āsvadīya rows), 98
/// apiece `10.0499+8.4.56`, `10.0499+7.1.35` and `10.0499+7.1.35+8.4.56`,
/// 20 apiece `10.0499+7.3.86+8.4.56`, `10.0499+7.1.35+7.3.86` and
/// `10.0499+7.1.35+7.3.86+8.4.56`, 36 `2565` and 2 apiece `2565+8.4.56`,
/// `2565+7.1.35` and `2565+7.1.35+8.4.56` (√pṝ), 36 `2571+7.3.86` and 2
/// apiece `2571+7.3.86+8.4.56`, `2571+7.1.35+7.3.86` and
/// `2571+7.1.35+7.3.86+8.4.56` (√ghuṣ, whose ṇic-less guṇa is the same
/// mandatory 7.3.86) — and folds 100 rows apiece into `8.4.56`, `7.1.35` and
/// `7.1.35+8.4.56` and 22 apiece into `7.3.86+8.4.56`, `7.3.86+7.1.35` and
/// `7.3.86+7.1.35+8.4.56` — √kṛ (slice 8b) adds six more"""),
("""/// cells, 2772 new rows. The gaṇa is OPEN at 258 of its 509 rows.
/// This test is what keeps the numbers true day to day.""",
"""/// cells, 2772 new rows. The gaṇa is OPEN at 258 of its 509 rows.
///
/// Slice 10i curates the fifty-nine āsvadīya rows (the gaṇasūtra 10.0499
/// *ā svadaḥ sakarmakāt*), `10.0022 pF` (Kaumudī 2565) and `10.0251 Guzi~r`
/// (2571), all `Nic`. The ṇic branch is the pinned form; every parasmaipada
/// cell adds the ṇic-less reading beside it — for √dhūp and √vich, 3.1.28's
/// āya stem (*dhūpāyati*, *vicchāyati*). 4392 new cells, 2928 new rows. The
/// gaṇa is OPEN at 319 of its 509 rows.
/// This test is what keeps the numbers true day to day."""),
("""    assert_eq!(total_cells, 21564, "2396 root×lakāra blocks × 9 cells each");""",
"""    assert_eq!(total_cells, 25956, "2884 root×lakāra blocks × 9 cells each");"""),
("""    assert_eq!(ones, 16072, "one-form cells");
    assert_eq!(twos, 4472, "two-form cells");""",
"""    assert_eq!(ones, 18268, "one-form cells");
    assert_eq!(twos, 6424, "two-form cells");"""),
("""        fours, 237,""",
"""        fours, 359,"""),
("""         new in slice 10h — forty-eight ādhṛṣīya rows' the same cells, 10.0498 alongside 8.4.56\"""",
"""         new in slice 10h — forty-eight ādhṛṣīya rows' the same cells, 10.0498 alongside 8.4.56; \\
         and — new in slice 10i — the sixty-one rows' the same cells, 10.0499/2565/2571 \\
         alongside 8.4.56\""""),
("""        sixes, 241,""",
"""        sixes, 363,"""),
("""         and √dhū's and √prī's laṅ and vidhiliṅ parasmaipada prathama eka (10.0498 and \\
         7.3.37.2 beside 8.4.56)\"""",
"""         and √dhū's and √prī's laṅ and vidhiliṅ parasmaipada prathama eka (10.0498 and \\
         7.3.37.2 beside 8.4.56); and — new in slice 10i — the sixty-one rows' loṭ \\
         parasmaipada prathama and madhyama eka (10.0499/2565/2571 beside 7.1.35/8.4.56)\""""),
("""    assert_eq!(ALTERNATES.len(), 7532, "ALTERNATES row count");""",
"""    assert_eq!(ALTERNATES.len(), 10460, "ALTERNATES row count");"""),
("""    assert_eq!(key_count("8.4.56"), 536, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 528, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 528, "7.1.35+8.4.56 alternates");""",
"""    assert_eq!(key_count("8.4.56"), 636, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 628, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 628, "7.1.35+8.4.56 alternates");"""),
("""    assert_eq!(key_count("7.3.86+8.4.56"), 55, "7.3.86+8.4.56 alternates");""",
"""    assert_eq!(key_count("7.3.86+8.4.56"), 77, "7.3.86+8.4.56 alternates");"""),
("""    assert_eq!(key_count("7.3.86+7.1.35"), 38, "7.3.86+7.1.35 alternates");
    assert_eq!(
        key_count("7.3.86+7.1.35+8.4.56"),
        38,""",
"""    assert_eq!(key_count("7.3.86+7.1.35"), 60, "7.3.86+7.1.35 alternates");
    assert_eq!(
        key_count("7.3.86+7.1.35+8.4.56"),
        60,"""),
("""        ("7.3.37.2+7.1.35+8.4.56", 4),
    ] {
        assert_eq!(key_count(key), n, "{key} alternates");
    }
}""",
"""        ("7.3.37.2+7.1.35+8.4.56", 4),
    ] {
        assert_eq!(key_count(key), n, "{key} alternates");
    }
    // Slice 10i's three vikalpa ids, alone and stacked: the gaṇasūtra
    // 10.0499's ṇic-less branch (with the mandatory 7.3.86 on the ten
    // laghu-ik rows), Kaumudī 2565's on √pṝ, and 2571's on √ghuṣ, whose
    // ṇic-less guṇa is the same mandatory 7.3.86.
    for (key, n) in [
        ("10.0499", 1764),
        ("10.0499+7.3.86", 360),
        ("10.0499+8.4.56", 98),
        ("10.0499+7.1.35", 98),
        ("10.0499+7.1.35+8.4.56", 98),
        ("10.0499+7.3.86+8.4.56", 20),
        ("10.0499+7.1.35+7.3.86", 20),
        ("10.0499+7.1.35+7.3.86+8.4.56", 20),
        ("2565", 36),
        ("2565+8.4.56", 2),
        ("2565+7.1.35", 2),
        ("2565+7.1.35+8.4.56", 2),
        ("2571+7.3.86", 36),
        ("2571+7.3.86+8.4.56", 2),
        ("2571+7.1.35+7.3.86", 2),
        ("2571+7.1.35+7.3.86+8.4.56", 2),
    ] {
        assert_eq!(key_count(key), n, "{key} alternates");
    }
}"""),
("""/// homograph pair its spec names. A ṇic-less analysis opens with its Kaumudī
/// id, credits 1.3.78 and never 3.1.25; a ṇic one credits 3.1.25 and never a
/// Kaumudī id but 2573.2.""",
"""/// homograph pair its spec names. A ṇic-less analysis opens with its
/// optional-ṇic id, credits 1.3.78 and never 3.1.25; a ṇic one credits
/// 3.1.25 and never an optional-ṇic id but 2573.2. Since slice 10i a
/// homograph's analyses may open with different ids, one per row: `daMSati`
/// is `10.0193`'s 2564 and the āsvadīya `10.0295`'s 10.0499."""),
("""    // (form, its roots, pada, the Kaumudī id that opens it, or None for ṇic)
    for (form, dhatus, pada, trigger) in [
        ("daMSati", &["danS"][..], Pada::Parasmaipada, Some("2564")),
        ("daMSayate", &["danS"], Pada::Atmanepada, None),
        ("vaYcati", &["vanc"], Pada::Parasmaipada, Some("2570")),
        // `10.0230` (10f, ākusmīya) and `10.0249` (10g, `Nic`) share these;
        // only `10.0249` derives the ṇic parasmaipada.
        ("devati", &["div", "div"], Pada::Parasmaipada, Some("2570")),
        ("devayate", &["div", "div"], Pada::Atmanepada, None),
        ("devayati", &["div"], Pada::Parasmaipada, None),
        ("garvati", &["garva"], Pada::Parasmaipada, Some("2573.3")),
        ("garvayate", &["garva"], Pada::Atmanepada, None),
        ("mUtrati", &["mUtra"], Pada::Parasmaipada, Some("2573.3")),
        ("mUtrayati", &["mUtra"], Pada::Parasmaipada, None),
        ("mUtrayate", &["mUtra"], Pada::Atmanepada, None),
        ("katrAmi", &["katra"], Pada::Parasmaipada, Some("2573.3")),
        ("patati", &["pata"], Pada::Parasmaipada, Some("2573.1")),
        ("patayati", &["pata"], Pada::Parasmaipada, None),
        ("pAtayati", &["pata"], Pada::Parasmaipada, None),
        ("patayate", &["pata"], Pada::Atmanepada, None),
        ("pAtayate", &["pata"], Pada::Atmanepada, None),
        // Slice 10g: one witness pair per new code shape.
        ("cintati", &["cint"], Pada::Parasmaipada, Some("2564")),
        ("cintayate", &["cint"], Pada::Atmanepada, None),
        ("sPuRwati", &["sPunw"], Pada::Parasmaipada, Some("2564")),
        ("sPuRwayate", &["sPunw"], Pada::Atmanepada, None),
        ("campati", &["canp"], Pada::Parasmaipada, Some("2564")),
        ("campayate", &["canp"], Pada::Atmanepada, None),
        ("daMhati", &["danh"], Pada::Parasmaipada, Some("2564")),
        ("daMhayate", &["danh"], Pada::Atmanepada, None),
        ("laRqati", &["lanq"], Pada::Parasmaipada, Some("2564")),
        ("laRqayate", &["lanq"], Pada::Atmanepada, None),
        ("kzampARi", &["kzanp"], Pada::Parasmaipada, Some("2564")),
        ("kzampayARi", &["kzanp"], Pada::Parasmaipada, None),
        ("SraRati", &["SraR"], Pada::Parasmaipada, Some("2570")),
        ("SrARayate", &["SraR"], Pada::Atmanepada, None),
        ("SarDati", &["SfD"], Pada::Parasmaipada, Some("2570")),
        ("SarDayate", &["SfD"], Pada::Atmanepada, None),
        ("aYcati", &["anc"], Pada::Parasmaipada, Some("2570")),
        ("aYcayate", &["anc"], Pada::Atmanepada, None),
        // Slice 10g's homographs: two rows each.
        (
            "laYjati",
            &["lanj", "lanj"],
            Pada::Parasmaipada,
            Some("2564"),
        ),
        (
            "vaRwati",
            &["vanw", "vanw"],
            Pada::Parasmaipada,
            Some("2564"),
        ),
        ("jasati", &["jas", "jas"], Pada::Parasmaipada, Some("2570")),
        // āṭ's vṛddhi gives `O` for both `o` and `u`: laṅ meets, laṭ does not.
        (
            "OlaRqad",
            &["olanq", "ulanq"],
            Pada::Parasmaipada,
            Some("2564"),
        ),
        ("olaRqati", &["olanq"], Pada::Parasmaipada, Some("2564")),
        ("ulaRqati", &["ulanq"], Pada::Parasmaipada, Some("2564")),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), dhatus.len(), "{form}");
        let mut got: Vec<&str> = r.analyses.iter().map(|a| a.dhatu.as_str()).collect();
        got.sort_unstable();
        let mut want = dhatus.to_vec();
        want.sort_unstable();
        assert_eq!(got, want, "{form}");
        for a in &r.analyses {
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
                    assert_eq!(
                        has(&ids, "2573.2"),
                        form.starts_with("pA"),
                        "{form}: {ids:?}"
                    );
                }
            }
        }
    }
""",
"""    // (form, its roots, pada, the optional-ṇic ids its analyses open with,
    // one per analysis, or none for ṇic)
    for (form, dhatus, pada, triggers) in [
        // Since slice 10i the āsvadīya `10.0295` is `danS` too: its
        // ṇic-less analysis opens with 10.0499.
        (
            "daMSati",
            &["danS", "danS"][..],
            Pada::Parasmaipada,
            &["2564", "10.0499"][..],
        ),
        ("daMSayate", &["danS", "danS"], Pada::Atmanepada, &[]),
        ("vaYcati", &["vanc"], Pada::Parasmaipada, &["2570"]),
        // `10.0230` (10f, ākusmīya) and `10.0249` (10g, `Nic`) share these;
        // only `10.0249` derives the ṇic parasmaipada.
        ("devati", &["div", "div"], Pada::Parasmaipada, &["2570", "2570"]),
        ("devayate", &["div", "div"], Pada::Atmanepada, &[]),
        ("devayati", &["div"], Pada::Parasmaipada, &[]),
        ("garvati", &["garva"], Pada::Parasmaipada, &["2573.3"]),
        ("garvayate", &["garva"], Pada::Atmanepada, &[]),
        ("mUtrati", &["mUtra"], Pada::Parasmaipada, &["2573.3"]),
        ("mUtrayati", &["mUtra"], Pada::Parasmaipada, &[]),
        ("mUtrayate", &["mUtra"], Pada::Atmanepada, &[]),
        ("katrAmi", &["katra"], Pada::Parasmaipada, &["2573.3"]),
        ("patati", &["pata"], Pada::Parasmaipada, &["2573.1"]),
        ("patayati", &["pata"], Pada::Parasmaipada, &[]),
        ("pAtayati", &["pata"], Pada::Parasmaipada, &[]),
        ("patayate", &["pata"], Pada::Atmanepada, &[]),
        ("pAtayate", &["pata"], Pada::Atmanepada, &[]),
        // Slice 10g: one witness pair per new code shape.
        ("cintati", &["cint"], Pada::Parasmaipada, &["2564"]),
        ("cintayate", &["cint"], Pada::Atmanepada, &[]),
        ("sPuRwati", &["sPunw"], Pada::Parasmaipada, &["2564"]),
        ("sPuRwayate", &["sPunw"], Pada::Atmanepada, &[]),
        ("campati", &["canp"], Pada::Parasmaipada, &["2564"]),
        ("campayate", &["canp"], Pada::Atmanepada, &[]),
        ("daMhati", &["danh"], Pada::Parasmaipada, &["2564"]),
        ("daMhayate", &["danh"], Pada::Atmanepada, &[]),
        // `lanq`, too, is the āsvadīya `10.0331` since slice 10i.
        (
            "laRqati",
            &["lanq", "lanq"],
            Pada::Parasmaipada,
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
            "laYjati",
            &["lanj", "lanj", "lanj"],
            Pada::Parasmaipada,
            &["2564", "2564", "10.0499"],
        ),
        (
            "vaRwati",
            &["vanw", "vanw"],
            Pada::Parasmaipada,
            &["2564", "2564"],
        ),
        (
            "jasati",
            &["jas", "jas"],
            Pada::Parasmaipada,
            &["2570", "2570"],
        ),
        // āṭ's vṛddhi gives `O` for both `o` and `u`: laṅ meets, laṭ does not.
        (
            "OlaRqad",
            &["olanq", "ulanq"],
            Pada::Parasmaipada,
            &["2564", "2564"],
        ),
        ("olaRqati", &["olanq"], Pada::Parasmaipada, &["2564"]),
        ("ulaRqati", &["ulanq"], Pada::Parasmaipada, &["2564"]),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), dhatus.len(), "{form}");
        let mut got: Vec<&str> = r.analyses.iter().map(|a| a.dhatu.as_str()).collect();
        got.sort_unstable();
        let mut want = dhatus.to_vec();
        want.sort_unstable();
        assert_eq!(got, want, "{form}");
        for a in &r.analyses {
            assert_eq!(a.pada, pada, "{form}");
            let ids = ids_of(a);
            if triggers.is_empty() {
                assert!(has(&ids, "3.1.25"), "{form}: {ids:?}");
                for id in ["10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3"] {
                    assert!(!has(&ids, id), "{form} {id}: {ids:?}");
                }
                assert_eq!(
                    has(&ids, "2573.2"),
                    form.starts_with("pA"),
                    "{form}: {ids:?}"
                );
            } else {
                assert!(has(&ids, "1.3.78"), "{form}: {ids:?}");
                assert!(!has(&ids, "3.1.25"), "{form}: {ids:?}");
            }
        }
        // A ṇic-less witness's analyses open with its ids, one apiece.
        if !triggers.is_empty() {
            let mut opened: Vec<String> = r.analyses.iter().map(|a| ids_of(a)[0].clone()).collect();
            opened.sort_unstable();
            let mut want = triggers.to_vec();
            want.sort_unstable();
            assert_eq!(opened, want, "{form}");
        }
    }
"""),
("""        "daMSayati",
        "garvayati",""",
"""        "tantrayati",
        "garvayati","""),
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
python3 /tmp/vidyut-full/slice10i/paradigm_10i.py      # applied 28 edits
```

What it changes, so a reviewer can check it against the spec and the goldens:
- **`VIKALPA_RULES`** gains 10.0499, 2565 and 2571.
- **The census:** 25956 cells (2884 blocks). Buckets are 18268 / 6424 / 525 / 359 / 10 / 363 / 1 / — / 6. Every one of the sixty-one rows adds 36 one-form ātmanepada cells (the ṇic branch; the ṇic-less ātmanepada blocks), 32 two-form parasmaipada cells, two four-form cells (laṅ and vidhiliṅ prathama eka, two readings × 8.4.56) and two six-form cells (loṭ prathama and madhyama eka, two readings × the tātaṅ triple).
- **`ALTERNATES`:** 10460 rows (+2928 = 61 × 48).
  - `8.4.56` / `7.1.35` / `7.1.35+8.4.56` reach 636 / 628 / 628: 100 each from the fifty ṇic branches without a sanādi guṇa.
  - `7.3.86+8.4.56` reaches 77, and `7.3.86+7.1.35` / `7.3.86+7.1.35+8.4.56` reach 60: 22 each from the eleven laghu-ik rows' ṇic branch (`puz`, `puw`, `luw`, `gup`, `puT`, `kup`, `vft`, `vfD`, `ruw`, `ruj`, `Guz`).
  - Sixteen new keys: `10.0499` 1764 (49 rows × 36), `10.0499+7.3.86` 360 (the ten āsvadīya laghu-ik rows), `10.0499+{8.4.56, 7.1.35, 7.1.35+8.4.56}` 98 each, `10.0499+{7.3.86+8.4.56, 7.1.35+7.3.86, 7.1.35+7.3.86+8.4.56}` 20 each, `2565` 36 and `2565+{…}` 2 each, `2571+7.3.86` 36 and `2571+{…}` 2 each.
- **The doc** of `every_alternate_names_the_vikalpa_rules_that_produced_it`: seventy of the 77 `7.3.86+8.4.56` keys are the mandatory firing; `2571+…7.3.86…` and `10.0499+…7.3.86…` keys are a ṇic-less root's guṇa.
- **The 10i paragraph.**
- **`curadi_analyses_its_optional_nic_forms`** (10g's). Its tuple's fourth field becomes the optional-ṇic ids its analyses open with, one per analysis, sorted and compared: `daMSati` (`2564`, `10.0499`), `laRqati` (the same), `laYjati` (`2564`, `2564`, `10.0499`), and `devati`, `vaRwati`, `jasati`, `OlaRqad` (their one id twice). `daMSayate` and `laRqayate` gain a second root. A ṇic witness credits none of 10.0499, 2565 or 2571 either. The negative witness `daMSayati` becomes `tantrayati`: the āsvadīya `10.0295` now derives *daṃśayati*, while √tantr, ākusmīya, still blocks its ṇic parasmaipada.

The pada-ambiguous set itself is pinned in Step 6, from the measured failure.

- [ ] **Step 2: The trace assertions and tests (failing)**

Create `/tmp/vidyut-full/slice10i/trace_10i.py` if it is missing (sha256 `a3e0e3f268f5f7caaaeb0f929d6df9eea4a34c8f7866f759ea503b8325f1aac1`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10i's edits to crates/panini/tests/trace/{curadi,juhotyadi}.rs.
Every `old` must occur exactly once; nothing is written if one fails."""
import sys
CURADI = 'crates/panini/tests/trace/curadi.rs'
JUHO = 'crates/panini/tests/trace/juhotyadi.rs'
NEW_TESTS = r'''

#[test]
fn the_10i_aya_and_tuk_fire_only_on_their_rows() {
    // Goldens ignore traces, so this is what holds slice 10i's two new
    // sanādi entries to their rows across the corpus: 3.1.28 on √dhūp and
    // √vich alone (`no_nic_pada_rule_reaches_a_nicless_branch` holds it to
    // their ṇic-less branches), and the sanādi 6.1.73 on √vich alone.
    assert_eq!(rows_crediting("3.1.28", true), ["10.0303", "10.0304"]);
    assert_eq!(rows_crediting("6.1.73", true), ["10.0304"]);
}

#[test]
#[allow(non_snake_case)]
fn vicCayati_and_vicCAyati_take_tuk_before_3_1_32() {
    // viC P laT P.E.: two live branches, the ṇic one at index 0. Both take
    // 6.1.73's tuk in the sanādi stage, after 3.4.114 and before 3.1.32, so
    // no sanādi 7.3.86 reads the `i`; 8.4.40 then makes the `t` a `c`. The
    // ṇic-less branch has 3.1.28's āya after 10.0499, whose `a` meets śap's
    // by 6.1.97's aṅga–śap entry.
    let d = dhatus().iter().find(|d| d.dhatupatha == "10.0304").unwrap();
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
    let core = ["3.1.32", "1.3.78", "3.4.78", "1.3.9", "3.1.68", "1.3.9"];
    let mut nic = vec!["3.1.25", "1.3.9", "3.4.114", "6.1.73"];
    nic.extend(core);
    nic.extend(["7.3.84", "6.1.78", "8.4.40"]);
    let mut aya = vec!["10.0499", "3.1.28", "3.4.114", "6.1.73"];
    aya.extend(core);
    aya.extend(["6.1.97", "8.4.40"]);
    assert_eq!(
        got,
        [("vicCayati".to_string(), nic), ("vicCAyati".to_string(), aya)],
    );
}'''
EDITS = {CURADI: [
("""//! (10.0498, 2564, 2570, 2573.1, 2573.3) and running the bhvādi path with
//! 1.3.78 — or, in a `NicUbhayapada` row's ātmanepada, 1.3.72. A
//! vowel-final ādhṛṣīya root's ṇic branch has 7.2.115 (and 6.1.78) before
//! 3.1.32, √dhū's and √prī's the vārttika 7.3.37.2 instead on a second
//! branch, and √mṛj's 7.2.114.""",
"""//! (10.0498, 10.0499, 2564, 2565, 2570, 2571, 2573.1, 2573.3) and running
//! the bhvādi path with 1.3.78 — or, in a `NicUbhayapada` row's ātmanepada,
//! 1.3.72. A vowel-final root's ṇic branch has 7.2.115 (and 6.1.78) before
//! 3.1.32, √dhū's and √prī's the vārttika 7.3.37.2 instead on a second
//! branch, and √mṛj's 7.2.114. √dhūp's and √vich's ṇic-less branch has
//! 3.1.28's āya, which 3.4.114 and 3.1.32 treat as they treat ṇic, and
//! √vich's every branch the sanādi 6.1.73's tuk before 3.1.32."""),
("""use panini_data::{
    AA_GARVIYA, AKUSMIYA, Gana, JNAPADI, Lakara, Pada, PadaAssignment, Purusha, Vacana, dhatus,
    optional_nic,
};""",
"""use panini_data::{
    AA_GARVIYA, AKUSMIYA, AYA, Gana, JNAPADI, Lakara, Pada, PadaAssignment, Purusha, Vacana,
    dhatus, optional_nic,
};"""),
("""    // never reaches them: its credits stay on the 199 `Nic` rows and the six""",
"""    // never reaches them: its credits stay on the 260 `Nic` rows and the six"""),
("""    assert_eq!(nic.len(), 199, "curated 1.3.74 rows");""",
"""    assert_eq!(nic.len(), 260, "curated 1.3.74 rows");"""),
("""    // on `pata`. Goldens ignore traces, so this is also what holds all six
    // inert on every root outside `OPTIONAL_NIC`. The 10.0498, 2564 and 2570
    // rows are the 10f, 10g and 10h specs' row tables, listed literally.""",
"""    // on `pata`. Goldens ignore traces, so this is also what holds all nine
    // inert on every root outside `OPTIONAL_NIC`. The 10.0498, 10.0499, 2564
    // and 2570 rows are the 10f to 10i specs' row tables, listed literally."""),
("""                "10.0388",
            ][..],
        ),
        (
            "2564",""",
"""                "10.0388",
            ][..],
        ),
        (
            "10.0499",
            &["10.0279", "10.0280", "10.0281", "10.0282", "10.0283", "10.0284", "10.0285", "10.0286", "10.0287", "10.0288", "10.0289", "10.0290", "10.0291", "10.0292", "10.0293", "10.0294", "10.0295", "10.0296", "10.0297", "10.0298", "10.0299", "10.0300", "10.0301", "10.0302", "10.0303", "10.0304", "10.0305", "10.0306", "10.0307", "10.0308", "10.0309", "10.0310", "10.0311", "10.0312", "10.0313", "10.0314", "10.0315", "10.0316", "10.0317", "10.0318", "10.0319", "10.0320", "10.0321", "10.0322", "10.0323", "10.0324", "10.0325", "10.0326", "10.0327", "10.0328", "10.0329", "10.0330", "10.0331", "10.0332", "10.0333", "10.0334", "10.0335", "10.0336", "10.0337"][..],
        ),
        (
            "2564","""),
("""        ("2573.1", &["10.0400"][..]),
        ("2573.2", &["10.0400"][..]),""",
"""        ("2565", &["10.0022"][..]),
        ("2571", &["10.0251"][..]),
        ("2573.1", &["10.0400"][..]),
        ("2573.2", &["10.0400"][..]),"""),
("""    // 1.3.72's. A ṇic branch never credits 1.3.72. On the ṇic branch of an
    // ākusmīya or ā-garvīya row, parasmaipada is blocked before anything is
    // recorded — the ṇic branch is the declined one, so not even the
    // optional-ṇic id is credited.
    let nic_only = [
        "10.0496", "10.0497", "1.3.74", "3.1.25", "3.4.114", "6.4.48", "7.2.116", "3.1.32",
    ];
    for d in optional_nic_rows() {
        let id = optional_nic(d.dhatupatha).unwrap();""",
"""    // 1.3.72's. A ṇic branch never credits 1.3.72. 3.4.114 and 3.1.32 reach
    // a ṇic-less branch only on √dhūp's and √vich's rows (`AYA`), for
    // 3.1.28's āya, which no ṇic branch credits. On the ṇic branch of an
    // ākusmīya or ā-garvīya row, parasmaipada is blocked before anything is
    // recorded — the ṇic branch is the declined one, so not even the
    // optional-ṇic id is credited.
    let nic_only = ["10.0496", "10.0497", "1.3.74", "3.1.25", "6.4.48", "7.2.116"];
    for d in optional_nic_rows() {
        let id = optional_nic(d.dhatupatha).unwrap();
        let aya = AYA.contains(&d.dhatupatha);"""),
("""                                for absent in nic_only {
                                    assert!(!ids.contains(&absent), "{cell} {absent}: {ids:?}");
                                }""",
"""                                for absent in nic_only {
                                    assert!(!ids.contains(&absent), "{cell} {absent}: {ids:?}");
                                }
                                // 3.1.28's āya is ārdhadhātuka and folds into
                                // the dhātu as ṇic does.
                                for sanadi in ["3.1.28", "3.4.114", "3.1.32"] {
                                    assert_eq!(
                                        ids.contains(&sanadi),
                                        aya,
                                        "{cell} {sanadi}: {ids:?}"
                                    );
                                }"""),
("""                            assert!(!ids.contains(&"1.3.72"), "{cell}: {ids:?}");""",
"""                            assert!(!ids.contains(&"1.3.72"), "{cell}: {ids:?}");
                            assert!(!ids.contains(&"3.1.28"), "{cell}: {ids:?}");"""),
("""fn natva_crosses_num_only_on_kzamp() {
    // 8.4.2 reads num as the anusvāra 8.3.24 has made of it (slice 10g).
    // Goldens ignore traces, so this is what holds that reading to √kṣamp's
    // cells: across the corpus, every live branch whose 8.4.2 crosses an `M`
    // is `10.0112 kzanp`'s loṭ parasmaipada uttama eka, one ṇic and one
    // ṇic-less, and both are found.""",
"""fn natva_crosses_num_only_on_kzamp_bfnh_and_ranh() {
    // 8.4.2 reads num as the anusvāra 8.3.24 has made of it (slice 10g).
    // Goldens ignore traces, so this is what holds that reading to its rows:
    // across the corpus, every live branch whose 8.4.2 crosses an `M` is the
    // loṭ parasmaipada uttama eka of `10.0112 kzanp` or, since slice 10i, of
    // the āsvadīya `10.0299 bfnh` and `10.0329 ranh`, one ṇic and one
    // ṇic-less branch each, and all six are found."""),
("""    assert_eq!(forms, ["kzampARi", "kzampayARi"], "{crossed:?}");
    for (number, lakara, pada, purusha, vacana, _) in &crossed {
        assert_eq!(*number, "10.0112");""",
"""    assert_eq!(
        forms,
        [
            "bfMhARi",
            "bfMhayARi",
            "kzampARi",
            "kzampayARi",
            "raMhARi",
            "raMhayARi"
        ],
        "{crossed:?}"
    );
    for (number, lakara, pada, purusha, vacana, _) in &crossed {
        assert!(
            ["10.0112", "10.0299", "10.0329"].contains(number),
            "{number}"
        );"""),
("""    // Goldens ignore traces, so this is what holds slice 10h's rules to
    // their rows across the corpus: 7.2.115 on exactly the eight vowel-final
    // ādhṛṣīya rows, the sanādi 6.1.78 on the six whose vṛddhi is an ec
    // (√vṛ and √jṝ reach `Ar` directly), 7.3.37.2 on √dhū and √prī, and
    // 7.2.114 on √mṛj, before ṇic and on its ṇic-less branch alike.
    assert_eq!(
        rows_crediting("7.2.115", false),
        [
            "10.0343", "10.0345", "10.0346", "10.0347", "10.0361", "10.0372", "10.0373", "10.0382"
        ]
    );
    assert_eq!(
        rows_crediting("6.1.78", true),
        [
            "10.0343", "10.0347", "10.0361", "10.0372", "10.0373", "10.0382"
        ]
    );""",
"""    // Goldens ignore traces, so this is what holds slice 10h's rules to
    // their rows across the corpus: 7.2.115 on exactly the eight vowel-final
    // ādhṛṣīya rows and, since slice 10i, √pṝ, √ji and √ci; the sanādi 6.1.78
    // on the six of those ādhṛṣīya rows whose vṛddhi is an ec (√vṛ and √jṝ
    // reach `Ar` directly, as √pṝ does) and on √ji and √ci; 7.3.37.2 on √dhū
    // and √prī; and 7.2.114 on √mṛj, before ṇic and on its ṇic-less branch
    // alike.
    assert_eq!(
        rows_crediting("7.2.115", false),
        [
            "10.0022", "10.0324", "10.0325", "10.0343", "10.0345", "10.0346", "10.0347",
            "10.0361", "10.0372", "10.0373", "10.0382"
        ]
    );
    assert_eq!(
        rows_crediting("6.1.78", true),
        [
            "10.0324", "10.0325", "10.0343", "10.0347", "10.0361", "10.0372", "10.0373",
            "10.0382"
        ]
    );"""),
], JUHO: [
("""    // fifty-three idit rows (num stored) and `10.0266 anc`, on every live
    // branch of both, and since slice 10h six ādhṛṣīya rows with an `n`
    // before a jhal (`granT` twice, `hins`, `SunD`, `SranT`, `kanW`), on
    // every live branch of both.
    const CURADI: [&str; 73] = [""",
"""    // fifty-three idit rows (num stored) and `10.0266 anc`, on every live
    // branch of both, and since slice 10h six ādhṛṣīya rows with an `n`
    // before a jhal (`granT` twice, `hins`, `SunD`, `SranT`, `kanW`), on
    // every live branch of both, and since slice 10i the twenty-seven idit
    // āsvadīya rows (num stored), on every live branch of both.
    const CURADI: [&str; 100] = ["""),
("""        "10.0464", "10.0465", "10.0266", "10.0362", "10.0366", "10.0369", "10.0374", "10.0375",
        "10.0385",
    ];""",
"""        "10.0464", "10.0465", "10.0266", "10.0362", "10.0366", "10.0369", "10.0374", "10.0375",
        "10.0385", "10.0285", "10.0286", "10.0287", "10.0289", "10.0290", "10.0291", "10.0292", "10.0293", "10.0294", "10.0295", "10.0296", "10.0298", "10.0299", "10.0315", "10.0316", "10.0317", "10.0318", "10.0319", "10.0321", "10.0322", "10.0323", "10.0326", "10.0327", "10.0328", "10.0329", "10.0330", "10.0331",
    ];"""),
("""    // ṇic ātmanepada), and slice 10g's fifty-four and slice 10h's six 120
    // each (every live branch: 84 parasmaipada, ṇic and ṇic-less, and 36 ṇic
    // ātmanepada).
    assert_eq!(curadi.len(), 36 + 7 * 78 + 5 * 78 + 60 * 120);""",
"""    // ṇic ātmanepada), and slice 10g's fifty-four, slice 10h's six and slice
    // 10i's twenty-seven 120 each (every live branch: 84 parasmaipada, ṇic
    // and ṇic-less, and 36 ṇic ātmanepada).
    assert_eq!(curadi.len(), 36 + 7 * 78 + 5 * 78 + 87 * 120);"""),
("""    // (acCandayat, acCandat), and slice 10h the three ch-initial ādhṛṣīya
    // roots (`Cfd`, `Cfp`, `Cad`) the same way (acCardayat, acCardat).""",
"""    // (acCandayat, acCandat), and slice 10h the three ch-initial ādhṛṣīya
    // roots (`Cfd`, `Cfp`, `Cad`) the same way (acCardayat, acCardat), and
    // slice 10i √vich (`viC`), whose root-internal tuk the sanādi 6.1.73
    // gives on every branch (vicCayati, vicCAyati)."""),
("""                "10.0171", "10.0352", "10.0354", "10.0370"
            ]""",
"""                "10.0171", "10.0352", "10.0354", "10.0370", "10.0304"
            ]"""),
("""    // 9 parasmaipada ones twice (ṇic and ṇic-less) plus their two 8.4.56 forks,
    // slice 10g's three and slice 10h's three alike.
    assert_eq!(curadi, 3 * 19 + 6 * 29);""",
"""    // 9 parasmaipada ones twice (ṇic and ṇic-less) plus their two 8.4.56 forks,
    // slice 10g's three and slice 10h's three alike. √vich: every live
    // branch, 84 parasmaipada (ṇic and ṇic-less) and 36 ṇic ātmanepada.
    assert_eq!(curadi, 3 * 19 + 6 * 29 + 120);"""),
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
print(f"applied {sum(len(e) for e in EDITS.values())} edits and two tests")
```

```bash
python3 /tmp/vidyut-full/slice10i/trace_10i.py      # applied 19 edits and two tests
```

It moves five rosters, each from the spec's row table and measured on the prototype:
- `a_kusmad_is_credited_on_exactly_the_akusmiya_cells`: 1.3.74's `Nic` rows go 199 → 260.
- `the_optional_nic_ids_are_credited_only_on_their_rows`: 10.0499's fifty-nine rows, listed literally, and 2565's and 2571's one row each. Without them the loop never looked at the three new ids.
- `no_nic_pada_rule_reaches_a_nicless_branch`: 3.4.114 and 3.1.32 leave the never-on-a-ṇic-less-branch list; instead 3.1.28, 3.4.114 and 3.1.32 appear on a ṇic-less branch exactly when the row is in `AYA`, and no ṇic branch credits 3.1.28.
- `nas_capadantasya_is_credited_only_on_rudhadi_dhan_jan_and_curadi_roots`: 8.3.24 gains the twenty-seven idit āsvadīya rows, 120 live branches each.
- `shcutva_off_jan_is_credited_exactly_as_before_3f3`: 8.4.40 gains `10.0304 viC`, on all 120 live branches.

It renames `natva_crosses_num_only_on_kzamp` to `natva_crosses_num_only_on_kzamp_bfnh_and_ranh`: 8.4.2 now crosses num on `10.0299 bfnh` and `10.0329 ranh` too (*bṛṃhāṇi* / *bṛṃhayāṇi*, *raṃhāṇi* / *raṃhayāṇi*), in the same loṭ parasmaipada uttama eka cell. It widens `the_10h_vrddhi_and_nuk_rules_fire_only_on_their_rows`: 7.2.115 gains `pF`, `ji` and `ci`, the sanādi 6.1.78 `ji` and `ci`. It updates the module doc and adds:
- `the_10i_aya_and_tuk_fire_only_on_their_rows`: 3.1.28 on `10.0303` and `10.0304`, the sanādi 6.1.73 on `10.0304`, nowhere else;
- `vicCayati_and_vicCAyati_take_tuk_before_3_1_32`: √vich's two laṭ prathama eka traces, in full.

- [ ] **Step 3: The data-layer assertions, docs and sanādi tests (failing)**

Create `/tmp/vidyut-full/slice10i/data_10i.py` if it is missing (sha256 `dc20aa6410ee54a8665c39390b4826490a3481ff0c28e657f111275e4f4a75ce`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10i's edits to crates/panini-data/src/lib.rs (tests and
docs, not the rows or the table) and to the sanādi unit tests that need the
rows. Every `old` must occur exactly once; nothing is written if one fails.
Run from the worktree root."""
import sys
DATA = 'crates/panini-data/src/lib.rs'
SANADI = 'crates/panini-prakriya/src/tinanta/sanadi.rs'
EDITS = {DATA: [
("""/// makes it so: the dhātupāṭha gaṇasūtra 10.0498 *ā dhṛṣād vā* for an
/// ādhṛṣīya row (`10.0338`–`10.0388`), which vidyut-prakriya decides before
/// any marker, and otherwise the Kaumudī's 2564 for an idit root, 2570 for a
/// ñit or udit root, 2573.1 for `pata`, and 2573.3 for the roots that rule
/// names (`mUtra`, `katra`, `garva`). Keyed by dhātupāṭha number, as""",
"""/// makes it so: the dhātupāṭha gaṇasūtras 10.0498 *ā dhṛṣād vā* for an
/// ādhṛṣīya row (`10.0338`–`10.0388`) and 10.0499 *ā svadaḥ sakarmakāt* for
/// an āsvadīya one (`10.0279`–`10.0337`), which vidyut-prakriya decides
/// before any marker, and otherwise the Kaumudī's 2564 for an idit root,
/// 2565 for `pF`, 2570 for a ñit or udit root, 2571 for `Guzi~r`, 2573.1
/// for `pata`, and 2573.3 for the roots that rule names (`mUtra`, `katra`,
/// `garva`). Keyed by dhātupāṭha number, as"""),
("""/// a listed root `Tag::Aya`, which 3.1.28 reads.
pub const AYA""",
"""/// a listed root `Tag::Aya`, which 3.1.28 reads.
/// `aya_is_exactly_the_curated_rows_3_1_28_names` pins it to upstream.
pub const AYA"""),
("""    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 361""",
"""    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 422"""),
("""    /// exception, 199 curādi rows' are 1.3.74's, six more 1.3.74's with ṇic""",
"""    /// exception, 260 curādi rows' are 1.3.74's, six more 1.3.74's with ṇic"""),
("""    /// the 119 curādi rows whose ṇic is optional this is the ṇic branch's""",
"""    /// the 180 curādi rows whose ṇic is optional this is the ṇic branch's"""),
("""    /// The test covers the 361 roots curated here, not the dhātupāṭha's 2259.""",
"""    /// The test covers the 422 roots curated here, not the dhātupāṭha's 2259."""),
("""        assert_eq!(dhatus().len(), 361);""",
"""        assert_eq!(dhatus().len(), 422);"""),
("""    fn curadi_rows_are_the_two_hundred_fifty_eight_curated_roots() {""",
"""    fn curadi_rows_are_the_three_hundred_nineteen_curated_roots() {"""),
("""        // `NicUbhayapada`, ubhayapadī by 1.3.72 without it too. The gaṇa is
        // OPEN at 258 of its 509 dhātupāṭha rows.""",
"""        // `NicUbhayapada`, ubhayapadī by 1.3.72 without it too. Slice 10i adds
        // the fifty-nine āsvadīya (10.0499), `10.0022 pF` (Kaumudī 2565) and
        // `10.0251 Guzi~r` (2571), ubhayapadī by 1.3.74 with ṇic. The gaṇa is
        // OPEN at 319 of its 509 dhātupāṭha rows."""),
("""    /// The id of the rule that makes curādi row `number`'s ṇic optional, read
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
        let u = upadesha.trim_end_matches(['\\\\', '^']);
        if u.ends_with("i~") {
            Some("2564")
        } else if u.ends_with("u~") || u.ends_with('Y') {
            Some("2570")
        } else if u == "pata" {""",
"""    /// The id of the rule that makes curādi row `number`'s ṇic optional, read
    /// as vidyut-prakriya reads it: first the two antargaṇas, by position
    /// (10.0498, rows `10.0338`–`10.0388`, and 10.0499, rows
    /// `10.0279`–`10.0337`, before any marker), then the upadeśa: 2564 for an
    /// idit root (last marker `i~`), 2565 for an `F`-final one, 2570 for a ñit
    /// or udit one (last marker `Y` or `u~`), 2571 for `Guzi~r`, 2573.1 for
    /// `pata`, and 2573.3 for the three roots the Kaumudī names there. 2573.3
    /// is a list, not a shape: `Cidra`, `sUtra` and the rest have a conjunct
    /// before their final `a` and take ṇic. vidyut's 2572 (īdit) has no arm:
    /// every īdit curādi row is in one of the two antargaṇas, which come
    /// first, as `optional_nic_matches_upadesha_markers` pins.
    fn optional_nic_from_upadesha(number: &str, upadesha: &str) -> Option<&'static str> {
        if ("10.0338"..="10.0388").contains(&number) {
            return Some("10.0498");
        }
        if ("10.0279"..="10.0337").contains(&number) {
            return Some("10.0499");
        }
        let u = upadesha.trim_end_matches(['\\\\', '^']);
        if u.ends_with("i~") {
            Some("2564")
        } else if u.ends_with('F') {
            Some("2565")
        } else if u.ends_with("u~") || u.ends_with('Y') {
            Some("2570")
        } else if u == "Guzi~r" {
            Some("2571")
        } else if u == "pata" {"""),
("""        assert_eq!(OPTIONAL_NIC.len(), 119);
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
        );""",
"""        assert_eq!(OPTIONAL_NIC.len(), 180);
        // `10.0368 za\\da~` stays out: the reading above makes it 10.0498's,
        // but vidyut derives it with the upasarga ā, which this engine does
        // not model.
        assert!(OPTIONAL_NIC.iter().all(|(n, _)| *n != "10.0368"));
        assert_eq!(
            optional_nic_from_upadesha("10.0368", upadesha("10.0368")),
            Some("10.0498")
        );
        // The one-row triggers reach one row each, and 2572 (īdit) none:
        // outside the two antargaṇas `pF` is the only `F`-final curādi row
        // (`10.0346 jF` is ādhṛṣīya), `Guzi~r` is one row, and every īdit
        // curādi row is inside.
        let antargana = |n: &str| ("10.0279"..="10.0388").contains(&n);
        let shape = |u: &str| u.trim_end_matches(['\\\\', '^']).to_string();
        let curadi: Vec<(&str, String)> = rows
            .iter()
            .filter(|(n, _, _)| n.starts_with("10."))
            .map(|(n, u, _)| (*n, shape(u)))
            .collect();
        let f_final: Vec<&str> = curadi
            .iter()
            .filter(|(n, u)| !antargana(n) && u.ends_with('F'))
            .map(|(n, _)| *n)
            .collect();
        assert_eq!(f_final, ["10.0022"]);
        let ghuz: Vec<&str> = curadi
            .iter()
            .filter(|(_, u)| u == "Guzi~r")
            .map(|(n, _)| *n)
            .collect();
        assert_eq!(ghuz, ["10.0251"]);
        let idit: Vec<&str> = curadi
            .iter()
            .filter(|(_, u)| u.ends_with("I~"))
            .map(|(n, _)| *n)
            .collect();
        assert!(!idit.is_empty());
        assert!(idit.iter().all(|n| antargana(n)), "{idit:?}");"""),
("""            // (none) is the case, pinned below, and so is `10.0367 arha~`
            // (10.0498) beside `10.0257 arha~` (none). The verdict only tells
            // curādi rows apart (the engine keys `OPTIONAL_NIC` by number
            // there alone), so every other gaṇa compares no verdict.""",
"""            // (none) is the case, pinned below, and so is `10.0367 arha~`
            // (10.0498) beside `10.0257 arha~` (none). The verdict only tells
            // curādi rows apart (the engine keys `OPTIONAL_NIC` by number
            // there alone), so every other gaṇa compares no verdict. The two
            // rows of `IDENTICAL_UPSTREAM_PAIR` differ in nothing, so each has
            // the other as its one allowed sibling."""),
("""            assert_eq!(
                siblings, 1,
                "{} is ambiguous: {siblings} rows in gaṇa {gana_prefix} share \\""",
"""            let allowed = if IDENTICAL_UPSTREAM_PAIR.contains(&d.dhatupatha) {
                2
            } else {
                1
            };
            assert_eq!(
                siblings, allowed,
                "{} is ambiguous: {siblings} rows in gaṇa {gana_prefix} share \\"""),
("""        assert_eq!(optional_nic("10.0174"), Some("2570"));
        assert_eq!(optional_nic("10.0063"), None);
    }""",
"""        assert_eq!(optional_nic("10.0174"), Some("2570"));
        assert_eq!(optional_nic("10.0063"), None);
        // `laGi~` twice: the pair is still identical upstream, and both rows
        // are curated.
        let (_, u1, a1) = row(IDENTICAL_UPSTREAM_PAIR[0]);
        let (_, u2, a2) = row(IDENTICAL_UPSTREAM_PAIR[1]);
        assert_eq!((u1, a1), ("laGi~", "BAzAyAm"));
        assert_eq!((u2, a2), (u1, a1));
        for n in IDENTICAL_UPSTREAM_PAIR {
            assert!(dhatus().iter().any(|d| d.dhatupatha == n), "{n}");
        }
    }

    #[test]
    fn aya_is_exactly_the_curated_rows_3_1_28_names() {
        // 3.1.28 names five roots, and vidyut-prakriya keys it on their
        // upadeśas in any gaṇa. `AYA` is exactly the curated rows that carry
        // one. Non-circular: the upadeśa comes from the vendored dhātupāṭha.
        let rows = upstream_rows();
        let named = ["gupU~", "DUpa~", "viCa~", "paRa~\\\\", "pana~\\\\"];
        for u in named {
            assert!(rows.iter().any(|(_, v, _)| *v == u), "{u} is not upstream");
        }
        let mut curated: Vec<&str> = dhatus()
            .iter()
            .filter(|d| {
                rows.iter()
                    .any(|(n, u, _)| *n == d.dhatupatha && named.contains(u))
            })
            .map(|d| d.dhatupatha)
            .collect();
        curated.sort_unstable();
        assert_eq!(curated, AYA);
        // Curādi `10.0302 gupa~` stores as `gup`, as the bhvādi `gupU~` does,
        // and is not named: why `AYA` is keyed by number.
        let gupa = rows.iter().find(|(n, _, _)| *n == "10.0302").unwrap().1;
        assert_eq!(stored_form(gupa), stored_form("gupU~"));
        assert!(!AYA.contains(&"10.0302"));
    }"""),
("""    #[test]
    fn dhatupatha_numbers_resolve_upstream() {""",
"""    /// The one pair of curated rows the dhātupāṭha lists twice, identically
    /// (upadeśa, artha and optional-ṇic verdict): `laGi~ BAzAyAm` at
    /// `10.0291` and `10.0327`, both āsvadīya. vidyut-prakriya derives both,
    /// so both are curated, and each is the other's one allowed sibling in
    /// `dhatupatha_numbers_resolve_upstream`.
    const IDENTICAL_UPSTREAM_PAIR: [&str; 2] = ["10.0291", "10.0327"];

    #[test]
    fn dhatupatha_numbers_resolve_upstream() {"""),
], SANADI: [
("""            // The idit √hiṃs is 10.0498's, not 2564's.
            ("10.0498", "10.0366", "hins", Tag::Nic),""",
"""            // The idit √hiṃs is 10.0498's, not 2564's.
            ("10.0498", "10.0366", "hins", Tag::Nic),
            ("10.0499", "10.0279", "gras", Tag::Nic),
            // The idit √tuñj, the udit √vṛt and the īdit √pūr are 10.0499's.
            ("10.0499", "10.0285", "tunj", Tag::Nic),
            ("10.0499", "10.0312", "vft", Tag::Nic),
            ("10.0499", "10.0334", "pUr", Tag::Nic),"""),
("""            ("2564", "10.0193", "danS", Tag::Akusmiya),
            ("2570", "10.0230", "div", Tag::Akusmiya),""",
"""            ("2564", "10.0193", "danS", Tag::Akusmiya),
            ("2565", "10.0022", "pF", Tag::Nic),
            ("2570", "10.0230", "div", Tag::Akusmiya),
            ("2571", "10.0251", "Guz", Tag::Nic),"""),
("""                ("10.0451", "mUtra", Tag::Nic),
                ("10.0001", "cur", Tag::Nic),""",
"""                ("10.0451", "mUtra", Tag::Nic),
                ("10.0279", "gras", Tag::Nic),
                ("10.0022", "pF", Tag::Nic),
                ("10.0251", "Guz", Tag::Nic),
                ("10.0001", "cur", Tag::Nic),"""),
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
for path, s in out.items():
    open(path, 'w').write(s)
print(f"applied {sum(len(e) for e in EDITS.values())} edits")
```

```bash
python3 /tmp/vidyut-full/slice10i/data_10i.py      # applied 18 edits
```

It updates:
- **`OPTIONAL_NIC`'s doc:** both gaṇasūtras, then 2564, 2565, 2570, 2571, 2573.1 and 2573.3.
- **`AYA`'s doc:** names its test.
- **`Dhatu::pada`'s census:** 422 rows; 260 `Nic`; 180 optional-ṇic rows.
- **The row count** (422), and the curādi list test's name (`…_three_hundred_nineteen_…`) and comment (319 of 509).
- **`optional_nic_from_upadesha`** answers `"10.0499"` for `10.0279..=10.0337` after the ādhṛṣīya range, `"2565"` for an `F`-final upadeśa after idit, and `"2571"` for `Guzi~r` after ñit/udit; its doc says why 2572 has no arm.
- **The marker test:** 180 entries; the exclusion is `10.0368` alone; and a pin that outside the two antargaṇas `pF` is the only `F`-final curādi row, `Guzi~r` is one row, and every īdit curādi row is inside them (the spec's 2572 claim).
- **`dhatupatha_numbers_resolve_upstream`:** `IDENTICAL_UPSTREAM_PAIR = ["10.0291", "10.0327"]`, each the other's one allowed sibling, and a pin that both are still `laGi~ BAzAyAm` upstream and both curated.
- **`aya_is_exactly_the_curated_rows_3_1_28_names`:** `AYA` is exactly the curated rows whose upstream upadeśa is one of vidyut's five (each asserted present upstream), and curādi `gupa~` stores as bhvādi `gupU~` does.
- **`sanadi.rs`:** `each_optional_nic_rule_takes_the_nicless_branch_on_its_own_rows` gains `gras`, the idit `tunj`, the udit `vft` and the īdit `pUr` (all 10.0499's), `pF` (2565) and `Guz` (2571); `an_optional_nic_rule_declines_off_its_rows` gains those three ids' rows.

- [ ] **Step 4: The `check()` witnesses (failing)**

Create `/tmp/vidyut-full/slice10i/check_10i.rs` (sha256 `eec21bda954d127ba2c200a079b3e723731d8072d74e8eef4c627dff72b8c86f`) and `/tmp/vidyut-full/slice10i/insert_check_10i.py` (sha256 `731c905aa5dfc88f2b290acd86363d593e422956b3849faa8240450ab7babf5e`) if they are missing:

```rust
/// Slice 10i's `check()` witnesses, from its spec's tables: a ṇic-less and a
/// ṇic witness per optional-ṇic id (10.0499, 2565, 2571), witnesses for each
/// new rule (3.1.28's āya, the sanādi 6.1.73) and each vowel-final row, and
/// every homograph the spec names. A ṇic-less analysis opens with its id,
/// credits 1.3.78 and no 3.1.25; a ṇic one credits 3.1.25 and none of the
/// three ids, and never 3.1.28. The goldens were grepped for every witness
/// first: a single-root witness is its own row's alone, and a homograph
/// witness is exactly its rows', one analysis each. Where the other row
/// came first in the table (`pArayati`'s adanta `10.0453 pAra`, and the
/// other gaṇas' `jayati`, `BaYjanti`, `aYjanti`, `vartatAm`), its analysis
/// comes first, so `trace_for` keeps answering with it; that order is pinned
/// here. The shapes the slice rules out — no ṇic in a `Nic` row's
/// ātmanepada, a ṇic-less √dhūp or √vich without āya, āya where ātmanepada
/// blocks it, guṇa before √vich's tuk, and guṇa where 7.2.115 gives vṛddhi
/// — derive nothing.
#[test]
fn curadi_analyses_its_asvadiya_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    let has = |ids: &[String], id: &str| ids.iter().any(|i| i == id);
    // (form, its curādi roots, pada, the optional-ṇic ids its analyses open
    // with, one per analysis, or none for ṇic, the ids every analysis credits)
    for (form, dhatus, pada, opens, credits) in [
        (
            "grasati",
            &["gras"][..],
            Pada::Parasmaipada,
            &["10.0499"][..],
            &["1.3.78"][..],
        ),
        ("grAsayati", &["gras"], Pada::Parasmaipada, &[], &["7.2.116"]),
        ("grAsayate", &["gras"], Pada::Atmanepada, &[], &["1.3.74"]),
        ("parati", &["pF"], Pada::Parasmaipada, &["2565"], &[]),
        ("Gozati", &["Guz"], Pada::Parasmaipada, &["2571"], &["7.3.86"]),
        ("Gozayati", &["Guz"], Pada::Parasmaipada, &[], &["7.3.86"]),
        (
            "DUpAyati",
            &["DUp"],
            Pada::Parasmaipada,
            &["10.0499"],
            &["3.1.28", "3.4.114", "3.1.32"],
        ),
        ("DUpayati", &["DUp"], Pada::Parasmaipada, &[], &[]),
        (
            "vicCAyati",
            &["viC"],
            Pada::Parasmaipada,
            &["10.0499"],
            &["3.1.28", "6.1.73", "8.4.40"],
        ),
        (
            "vicCayati",
            &["viC"],
            Pada::Parasmaipada,
            &[],
            &["6.1.73", "8.4.40"],
        ),
        (
            "jAyayati",
            &["ji"],
            Pada::Parasmaipada,
            &[],
            &["7.2.115", "6.1.78"],
        ),
        (
            "cAyayati",
            &["ci"],
            Pada::Parasmaipada,
            &[],
            &["7.2.115", "6.1.78"],
        ),
        ("cayati", &["ci"], Pada::Parasmaipada, &["10.0499"], &[]),
        // The īdit `pUrI~` and the udit `vftu~` are 10.0499's, not 2572's
        // or 2570's.
        ("pUrati", &["pUr"], Pada::Parasmaipada, &["10.0499"], &[]),
        (
            "vartati",
            &["vft"],
            Pada::Parasmaipada,
            &["10.0499"],
            &["7.3.86"],
        ),
        // Homographs: a curādi row of the same code before this slice, or
        // two in it.
        (
            "daMsati",
            &["dans", "dans"],
            Pada::Parasmaipada,
            &["2564", "10.0499"],
            &["8.3.24"],
        ),
        (
            "tuYjati",
            &["tunj", "tunj"],
            Pada::Parasmaipada,
            &["2564", "10.0499"],
            &["8.3.24"],
        ),
        (
            "piYjati",
            &["pinj", "pinj"],
            Pada::Parasmaipada,
            &["2564", "10.0499"],
            &["8.3.24"],
        ),
        (
            "luYjati",
            &["lunj", "lunj"],
            Pada::Parasmaipada,
            &["2564", "10.0499"],
            &["8.3.24"],
        ),
        (
            "SIkati",
            &["SIk", "SIk"],
            Pada::Parasmaipada,
            &["10.0498", "10.0499"],
            &[],
        ),
        ("SIkayati", &["SIk", "SIk"], Pada::Parasmaipada, &[], &[]),
        (
            "laNGati",
            &["lanG", "lanG"],
            Pada::Parasmaipada,
            &["10.0499", "10.0499"],
            &["8.3.24"],
        ),
        (
            "laNGayate",
            &["lanG", "lanG"],
            Pada::Atmanepada,
            &[],
            &["1.3.74"],
        ),
        (
            "svAdayati",
            &["svad", "svAd"],
            Pada::Parasmaipada,
            &[],
            &[],
        ),
        ("pArayati", &["pAra", "pF"], Pada::Parasmaipada, &[], &[]),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        let mut got: Vec<&str> = r.analyses.iter().map(|a| a.dhatu.as_str()).collect();
        got.sort_unstable();
        let mut want = dhatus.to_vec();
        want.sort_unstable();
        assert_eq!(got, want, "{form}");
        for a in &r.analyses {
            assert_eq!(a.pada, pada, "{form}");
            let ids = ids_of(a);
            if opens.is_empty() {
                assert!(has(&ids, "3.1.25"), "{form}: {ids:?}");
                for id in ["10.0499", "2565", "2571", "3.1.28"] {
                    assert!(!has(&ids, id), "{form} {id}: {ids:?}");
                }
            } else {
                assert!(has(&ids, "1.3.78"), "{form}: {ids:?}");
                assert!(!has(&ids, "3.1.25"), "{form}: {ids:?}");
            }
            for id in credits {
                assert!(has(&ids, id), "{form} {id}: {ids:?}");
            }
        }
        if !opens.is_empty() {
            let mut opened: Vec<String> = r.analyses.iter().map(|a| ids_of(a)[0].clone()).collect();
            opened.sort_unstable();
            let mut want = opens.to_vec();
            want.sort_unstable();
            assert_eq!(opened, want, "{form}");
        }
    }
    // √vich's tuk leaves guṇa nothing to read on either branch.
    for form in ["vicCAyati", "vicCayati"] {
        let ids = ids_of(&engine.check(form).analyses[0]);
        assert!(!has(&ids, "7.3.86"), "{form}: {ids:?}");
    }
    // `pArayati`: the adanta `pAra`'s analysis first (6.4.48), then √pṝ's
    // (7.2.115).
    let r = engine.check("pArayati");
    assert_eq!(r.analyses[0].dhatu, "pAra");
    assert!(has(&ids_of(&r.analyses[0]), "6.4.48"));
    assert_eq!(r.analyses[1].dhatu, "pF");
    assert!(has(&ids_of(&r.analyses[1]), "7.2.115"));
    // Homographs with other gaṇas: that gaṇa's row's analysis first, then
    // the āsvadīya row's ṇic-less one.
    for (form, dhatu, first_pada) in [
        ("jayati", "ji", Pada::Parasmaipada),
        ("BaYjanti", "Banj", Pada::Parasmaipada),
        ("aYjanti", "anj", Pada::Parasmaipada),
        ("vartatAm", "vft", Pada::Atmanepada),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), 2, "{form}");
        for a in &r.analyses {
            assert_eq!(a.dhatu, dhatu, "{form}");
        }
        let first = ids_of(&r.analyses[0]);
        let second = ids_of(&r.analyses[1]);
        assert_eq!(r.analyses[0].pada, first_pada, "{form}");
        assert!(!has(&first, "10.0499"), "{form}: {first:?}");
        assert_eq!(r.analyses[1].pada, Pada::Parasmaipada, "{form}");
        assert_eq!(second[0], "10.0499", "{form}: {second:?}");
    }
    for form in [
        "grasate", "parate", "Gozate", "DUpati", "vicCati", "DUpAyate", "veCayati", "jayayati",
        "cayayati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
```

```python
#!/usr/bin/env python3
"""THROWAWAY: append check_10i.rs's test, doc comment included, after 10h's
curadi_analyses_its_adhrsiya_forms, the last item in the file."""
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
assert s.count('fn curadi_analyses_its_adhrsiya_forms()') == 1
assert 'fn curadi_analyses_its_asvadiya_forms()' not in s
j = s.index('fn curadi_analyses_its_adhrsiya_forms()')
assert s.index('\n}\n', j) + 3 == len(s), "10h's test is no longer last"
s = s + '\n' + open('/tmp/vidyut-full/slice10i/check_10i.rs').read().rstrip('\n') + '\n'
open(p, 'w').write(s)
print("check witnesses appended")
```

```bash
python3 /tmp/vidyut-full/slice10i/insert_check_10i.py      # check witnesses appended
mise run fmt
```

It appends `curadi_analyses_its_asvadiya_forms` after 10h's `curadi_analyses_its_adhrsiya_forms`, the last item in the file. Its witnesses come from the spec's tables:
- **Per trigger:** `grasati` / `grAsayati` / `grAsayate` (10.0499), `parati` (2565), `Gozati` / `Gozayati` (2571).
- **Per new rule:** `DUpAyati` (3.1.28, 3.4.114, 3.1.32), `DUpayati`, `vicCAyati` (3.1.28, 6.1.73, 8.4.40), `vicCayati` (6.1.73, 8.4.40), neither √vich form crediting 7.3.86.
- **Per vowel-final row:** `jAyayati`, `cAyayati` (7.2.115, 6.1.78), `cayati`.
- **Markers 10.0499 overrides:** `pUrati` (īdit), `vartati` (udit).
- **Homographs:** `daMsati`, `tuYjati`, `piYjati`, `luYjati` (2564 and 10.0499), `SIkati` (10.0498 and 10.0499) and `SIkayati`, `laNGati` / `laNGayate` (the `laGi~` pair), `svAdayati` (`svad` and `svAd`), `pArayati` (`pAra`'s analysis first); and, across gaṇas, `jayati`, `BaYjanti`, `aYjanti`, `vartatAm`, the other gaṇa's analysis first. `daMSati`, `laRqati` and `laYjati` are 10g's test's (Step 1).
- **Invalid:** `grasate`, `parate`, `Gozate`, `DUpati`, `vicCati`, `DUpAyate`, `veCayati`, `jayayati`, `cayayati`.

- [ ] **Step 5: Run them to see them fail**

Run: `mise exec -- cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .*FAILED" | sort`
Expected failures (no row exists yet): `panini-data` 24 passed / 4 failed, `panini-prakriya` 429 passed / 1 failed, `paradigm` 24 passed / 3 failed, `trace` 207 passed / 7 failed:

- `curadi::a_kusmad_is_credited_on_exactly_the_akusmiya_cells`
- `curadi::natva_crosses_num_only_on_kzamp_bfnh_and_ranh`
- `curadi::the_10h_vrddhi_and_nuk_rules_fire_only_on_their_rows`
- `curadi::the_10i_aya_and_tuk_fire_only_on_their_rows`
- `curadi::vicCayati_and_vicCAyati_take_tuk_before_3_1_32`
- `curadi_analyses_its_asvadiya_forms`
- `curadi_analyses_its_optional_nic_forms`
- `derivation_set_shape_matches_the_audited_numbers`
- `juhotyadi::nas_capadantasya_is_credited_only_on_rudhadi_dhan_jan_and_curadi_roots`
- `juhotyadi::shcutva_off_jan_is_credited_exactly_as_before_3f3`
- `tests::aya_is_exactly_the_curated_rows_3_1_28_names`
- `tests::curated_roots_have_expected_ganas_and_padas`
- `tests::dhatupatha_numbers_resolve_upstream`
- `tests::optional_nic_matches_upadesha_markers`
- `tinanta::sanadi::tests::each_optional_nic_rule_takes_the_nicless_branch_on_its_own_rows`

Several tests still pass. `the_optional_nic_ids_are_credited_only_on_their_rows` and `no_nic_pada_rule_reaches_a_nicless_branch` hold vacuously with no new row; `every_alternate_names_the_vikalpa_rules_that_produced_it` and `pada_ambiguous_surfaces_are_exactly_these` are unchanged by the scripts; and `curated_pada_agrees_with_upadesha_markers` has no new row to read yet.

- [ ] **Step 6: The rows, the goldens, and the measured pada-ambiguous set**

Create `/tmp/vidyut-full/slice10i/gen_rows_10i.py` (sha256 `53d1c84dd099d04ac0ad8327c124bb8a2f9fc92869e7cab0dbe17c4c2121a015`) and `/tmp/vidyut-full/slice10i/insert_rows_10i.py` (sha256 `ffc0a53d525c901b52d61d042e0756f7bdff02a48a3dcdf3171a689e15b7107d`) if they are missing. The `ROWS` table is the spec's, with each row's laṭ parasmaipada prathama eka forms (ṇic-less, ṇic) for its comment, checked against the generated goldens:

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10i — emit the 61 rows' `Dhatu` entries (the 59
āsvadīya, `10.0022 pF` and `10.0251 Guzi~r`), the `curadi_rows_are_…` list
entries and the new `OPTIONAL_NIC` entries. Run from the worktree root;
writes rows.rs, rowlist.rs and table.rs into the directory given."""
import sys, textwrap

OUT = sys.argv[1]
# (number, code, optional-ṇic id, root, ṇic-less laṭ P 3sg, ṇic laṭ P 3sg)
ROWS = [
    ("10.0022", "pF", "2565", "pṝ", "parati", "pArayati"),
    ("10.0251", "Guz", "2571", "ghuṣ", "Gozati", "Gozayati"),
    ("10.0279", "gras", "10.0499", "gras", "grasati", "grAsayati"),
    ("10.0280", "puz", "10.0499", "puṣ", "pozati", "pozayati"),
    ("10.0281", "dal", "10.0499", "dal", "dalati", "dAlayati"),
    ("10.0282", "paw", "10.0499", "paṭ", "pawati", "pAwayati"),
    ("10.0283", "puw", "10.0499", "puṭ", "powati", "powayati"),
    ("10.0284", "luw", "10.0499", "luṭ", "lowati", "lowayati"),
    ("10.0285", "tunj", "10.0499", "tuñj", "tuYjati", "tuYjayati"),
    ("10.0286", "minj", "10.0499", "miñj", "miYjati", "miYjayati"),
    ("10.0287", "pinj", "10.0499", "piñj", "piYjati", "piYjayati"),
    ("10.0288", "lak", "10.0499", "lak", "lakati", "lAkayati"),
    ("10.0289", "lunj", "10.0499", "luñj", "luYjati", "luYjayati"),
    ("10.0290", "Banj", "10.0499", "bhañj", "BaYjati", "BaYjayati"),
    ("10.0291", "lanG", "10.0499", "laṅgh", "laNGati", "laNGayati"),
    ("10.0292", "trans", "10.0499", "traṃs", "traMsati", "traMsayati"),
    ("10.0293", "pins", "10.0499", "piṃs", "piMsati", "piMsayati"),
    ("10.0294", "kuns", "10.0499", "kuṃs", "kuMsati", "kuMsayati"),
    ("10.0295", "danS", "10.0499", "daṃś", "daMSati", "daMSayati"),
    ("10.0296", "kunS", "10.0499", "kuṃś", "kuMSati", "kuMSayati"),
    ("10.0297", "Gaw", "10.0499", "ghaṭ", "Gawati", "GAwayati"),
    ("10.0298", "Ganw", "10.0499", "ghaṇṭ", "GaRwati", "GaRwayati"),
    ("10.0299", "bfnh", "10.0499", "bṛṃh", "bfMhati", "bfMhayati"),
    ("10.0300", "barh", "10.0499", "barh", "barhati", "barhayati"),
    ("10.0301", "balh", "10.0499", "balh", "balhati", "balhayati"),
    ("10.0302", "gup", "10.0499", "gup", "gopati", "gopayati"),
    ("10.0303", "DUp", "10.0499", "dhūp", "DUpAyati", "DUpayati"),
    ("10.0304", "viC", "10.0499", "vich", "vicCAyati", "vicCayati"),
    ("10.0305", "cIv", "10.0499", "cīv", "cIvati", "cIvayati"),
    ("10.0306", "puT", "10.0499", "puth", "poTati", "poTayati"),
    ("10.0307", "lok", "10.0499", "lok", "lokati", "lokayati"),
    ("10.0308", "loc", "10.0499", "loc", "locati", "locayati"),
    ("10.0309", "nad", "10.0499", "nad", "nadati", "nAdayati"),
    ("10.0310", "kup", "10.0499", "kup", "kopati", "kopayati"),
    ("10.0311", "tark", "10.0499", "tark", "tarkati", "tarkayati"),
    ("10.0312", "vft", "10.0499", "vṛt", "vartati", "vartayati"),
    ("10.0313", "vfD", "10.0499", "vṛdh", "varDati", "varDayati"),
    ("10.0314", "ruw", "10.0499", "ruṭ", "rowati", "rowayati"),
    ("10.0315", "lanj", "10.0499", "lañj", "laYjati", "laYjayati"),
    ("10.0316", "anj", "10.0499", "añj", "aYjati", "aYjayati"),
    ("10.0317", "dans", "10.0499", "daṃs", "daMsati", "daMsayati"),
    ("10.0318", "BfnS", "10.0499", "bhṛṃś", "BfMSati", "BfMSayati"),
    ("10.0319", "runS", "10.0499", "ruṃś", "ruMSati", "ruMSayati"),
    ("10.0320", "SIk", "10.0499", "śīk", "SIkati", "SIkayati"),
    ("10.0321", "runs", "10.0499", "ruṃs", "ruMsati", "ruMsayati"),
    ("10.0322", "nanw", "10.0499", "naṇṭ", "naRwati", "naRwayati"),
    ("10.0323", "punw", "10.0499", "puṇṭ", "puRwati", "puRwayati"),
    ("10.0324", "ji", "10.0499", "ji", "jayati", "jAyayati"),
    ("10.0325", "ci", "10.0499", "ci", "cayati", "cAyayati"),
    ("10.0326", "ranD", "10.0499", "randh", "ranDati", "ranDayati"),
    ("10.0327", "lanG", "10.0499", "laṅgh", "laNGati", "laNGayati"),
    ("10.0328", "anh", "10.0499", "aṃh", "aMhati", "aMhayati"),
    ("10.0329", "ranh", "10.0499", "raṃh", "raMhati", "raMhayati"),
    ("10.0330", "manh", "10.0499", "maṃh", "maMhati", "maMhayati"),
    ("10.0331", "lanq", "10.0499", "laṇḍ", "laRqati", "laRqayati"),
    ("10.0332", "taq", "10.0499", "taḍ", "taqati", "tAqayati"),
    ("10.0333", "nal", "10.0499", "nal", "nalati", "nAlayati"),
    ("10.0334", "pUr", "10.0499", "pūr", "pUrati", "pUrayati"),
    ("10.0335", "ruj", "10.0499", "ruj", "rojati", "rojayati"),
    ("10.0336", "svad", "10.0499", "svad", "svadati", "svAdayati"),
    ("10.0337", "svAd", "10.0499", "svād", "svAdati", "svAdayati"),
]
assert len(ROWS) == 61
IDIT = {"10.0285", "10.0286", "10.0287", "10.0289", "10.0290", "10.0291", "10.0292",
        "10.0293", "10.0294", "10.0295", "10.0296", "10.0298", "10.0299", "10.0315",
        "10.0316", "10.0317", "10.0318", "10.0319", "10.0321", "10.0322", "10.0323",
        "10.0326", "10.0327", "10.0328", "10.0329", "10.0330", "10.0331"}
assert len(IDIT) == 27
# What a row needs beyond 10f's fork, said in its comment.
NOTE = {
    "10.0022": "7.2.115 *aco ñṇiti* lengthens the final before ṇic.",
    "10.0251": "Irit (`i~r`), not idit, so not 2564's.",
    "10.0291": "Idit (7.1.58's num stored). Listed twice in the dhātupāṭha, "
               "identically: `10.0291` and `10.0327`.",
    "10.0327": "Idit (7.1.58's num stored). Listed twice in the dhātupāṭha, "
               "identically: `10.0291` and `10.0327`.",
    "10.0303": "Without ṇic, 3.1.28 gives āya.",
    "10.0304": "Without ṇic, 3.1.28 gives āya; on both branches 6.1.73's tuk "
               "comes before guṇa.",
    "10.0312": "Udit, but āsvadīya: 10.0499's, not 2570's.",
    "10.0313": "Udit, but āsvadīya: 10.0499's, not 2570's.",
    "10.0324": "7.2.115 *aco ñṇiti* lengthens the final before ṇic.",
    "10.0325": "7.2.115 *aco ñṇiti* lengthens the final before ṇic.",
    "10.0334": "Īdit, but āsvadīya: 10.0499's, not 2572's.",
}
for n in IDIT:
    NOTE.setdefault(n, "Idit (7.1.58's num stored), but āsvadīya: 10.0499's, not 2564's.")
IAST = {'A': 'ā', 'I': 'ī', 'U': 'ū', 'f': 'ṛ', 'F': 'ṝ', 'x': 'ḷ', 'E': 'ai', 'O': 'au',
        'K': 'kh', 'G': 'gh', 'N': 'ṅ', 'C': 'ch', 'J': 'jh', 'Y': 'ñ', 'w': 'ṭ', 'W': 'ṭh',
        'q': 'ḍ', 'Q': 'ḍh', 'R': 'ṇ', 'T': 'th', 'D': 'dh', 'P': 'ph', 'B': 'bh', 'S': 'ś',
        'z': 'ṣ', 'M': 'ṃ', 'H': 'ḥ'}
def iast(s):
    return ''.join(IAST.get(c, c) for c in s)
up = {}
for line in open('data/dhatupatha.tsv'):
    f = line.rstrip('\n').split('\t')
    if len(f) >= 3:
        up[f[0]] = (f[1], f[2])
out, lst = [], []
for n, code, oid, root, nicless, nic in ROWS:
    u, artha = up[n]
    opt = ("Āsvadīya: ṇic optional by 10.0499." if oid == "10.0499"
           else f"Ṇic optional by Kaumudī {oid}.")
    p = (f"Ubhayapadī by 1.3.74 with ṇic (*{iast(nic)}*), parasmaipadī by 1.3.78 "
         f"without (*{iast(nicless)}*).")
    note = f" {NOTE[n]}" if n in NOTE else ""
    c = f"{n} `{u}` {artha} (√{root}). {opt} {p}{note} Slice 10i."
    com = '\n'.join('        // ' + l for l in textwrap.wrap(c, 72))
    out.append(f"    Dhatu {{\n{com}\n        dhatupatha: \"{n}\",\n        code: \"{code}\",\n"
               f"        gana: Gana::Curadi,\n        pada: PadaAssignment::Nic,\n"
               f"        artha: \"{artha}\",\n    }},\n")
    lst.append(f'                ("{n}", "{code}", PadaAssignment::Nic),\n')
table = ''.join(f'    ("{n}", "{oid}"),\n' for n, _, oid, *_ in ROWS)
open(f'{OUT}/rows.rs', 'w').write(''.join(out))
open(f'{OUT}/rowlist.rs', 'w').write(''.join(lst))
open(f'{OUT}/table.rs', 'w').write(table)
print(f"{len(ROWS)} rows")
```

```python
#!/usr/bin/env python3
"""THROWAWAY: insert gen_rows_10i.py's output ($GEN/rows.rs, rowlist.rs,
table.rs). The rows go after `10.0388 Dfz`, the last curādi row; the table
is rewritten in dhātupāṭha order."""
import os, re
GEN = os.environ['GEN']
p = 'crates/panini-data/src/lib.rs'
s = open(p).read()
a = '''        code: "Dfz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prasahane",
    },
];'''
assert s.count(a) == 1
s = s.replace(a, a[:-2] + open(f'{GEN}/rows.rs').read() + '];')
b = '''                ("10.0388", "Dfz", PadaAssignment::Nic),
'''
assert s.count(b) == 1
s = s.replace(b, b + open(f'{GEN}/rowlist.rs').read())
head = 'pub const OPTIONAL_NIC: &[(&str, &str)] = &[\n'
i = s.index(head) + len(head)
j = s.index('];', i)
entries = s[i:j] + open(f'{GEN}/table.rs').read()
lines = sorted(l for l in entries.splitlines(keepends=True) if l.strip())
assert len(lines) == 180, len(lines)
assert all(re.fullmatch(r'    \("10\.\d{4}", "[0-9.]+"\),\n', l) for l in lines)
s = s[:i] + ''.join(lines) + s[j:]
open(p, 'w').write(s)
print("inserted 61 rows; OPTIONAL_NIC has 180 entries")
```

```bash
GEN="$(mktemp -d)"
python3 /tmp/vidyut-full/slice10i/gen_rows_10i.py "$GEN"      # 61 rows
sha256sum "$GEN/rows.rs" "$GEN/rowlist.rs" "$GEN/table.rs"
GEN="$GEN" python3 /tmp/vidyut-full/slice10i/insert_rows_10i.py      # inserted 61 rows; OPTIONAL_NIC has 180 entries
```

Expected hashes:
- `rows.rs`: `349cff699673ac9ec0e26771ddbe5a7c985155da87f4c66437ee59777fe2350b`
- `rowlist.rs`: `9388cb03ddb73b3e1d5d08185d05aec11c9ee287de12ffd835a54f63857f2d66`
- `table.rs`: `db30b20f74dc777ba0722f032da28d79b0aca19163ed37b1cf51adc38a84c940`

If any differs, stop and report. The rows go after `10.0388 Dfz`, the last curādi row (the data layer orders curādi by slice); the table is rewritten in dhātupāṭha order.

Repoint the vidyut checkout's dev-deps at this worktree. Then create `/tmp/vidyut-full/vidyut-prakriya/examples/curadi_goldens_10i.rs` (sha256 `6d8e696eeae6b45e43a120ffdb5e57b3d3a2e81f6ad0fc2e2ce433f002299f95`) and `/tmp/vidyut-full/slice10i/insert_goldens_10i.py` (sha256 `f9d058f1547602da3576cd697f940a2b81a59c6ad32cd3c78821a73edf865187`) if they are missing:

```rust
//! THROWAWAY: slice 10i — emit the 61 rows (the āsvadīya, `pF`, `Guzi~r`)' goldens from the
//! engine, asserting every cell's form set equals vidyut's first.
use panini::Panini;
use panini_data::{dhatus, optional_nic, Lakara as L, Pada, Purusha as P, Vacana as V};
use vidyut_prakriya::args::{DhatuPada, Lakara, Prayoga, Purusha, Tinanta, Vacana};
use vidyut_prakriya::{Dhatupatha, Vyakarana};
const VIKALPA_RULES: &[&str] = &[
    "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3", "2573.2", "7.3.37.2", "7.1.35", "3.4.111", "7.3.86", "6.4.107",
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
    // The new rows: 10.0499's, 2565's and 2571's `OPTIONAL_NIC` entries.
    for d in dhatus().iter().filter(|d| matches!(optional_nic(d.dhatupatha), Some("10.0499" | "2565" | "2571"))) {
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
    std::fs::write("/tmp/vidyut-full/goldens_10i_paradigm.rs", par).unwrap();
    std::fs::write("/tmp/vidyut-full/goldens_10i_alternates.rs", alt).unwrap();
    println!("{ncells} cells, {nforms} forms, {ndiff} differences");
    assert_eq!(ndiff, 0);
}
```

```python
#!/usr/bin/env python3
"""THROWAWAY: insert the generator's goldens before each static's `];`."""
p = 'crates/panini/tests/paradigm/data/curadi.rs'
s = open(p).read()
par = open('/tmp/vidyut-full/goldens_10i_paradigm.rs').read()
alt = open('/tmp/vidyut-full/goldens_10i_alternates.rs').read()
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
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example curadi_goldens_10i 2>/dev/null | tail -3)
sha256sum /tmp/vidyut-full/goldens_10i_paradigm.rs /tmp/vidyut-full/goldens_10i_alternates.rs
python3 /tmp/vidyut-full/slice10i/insert_goldens_10i.py      # inserted goldens
mise run fmt
```

Expected:
- the generator prints `4392 cells, 7320 forms, 0 differences`;
- `goldens_10i_paradigm.rs`: `1be8d626362aa326f7ce74ec3fdec95e5f77e2ab0a7b18e01a9c919a27e570a0` (488 rows);
- `goldens_10i_alternates.rs`: `fff48b7294d0c69d0066665b024af4b7331336be9cbc925044535bce3f2951d5` (2928 rows).

Any `DIFF` line or other hash: stop and report, and edit nothing. Every new row's ṇic branch is live in both padas, so each pinned form is the ṇic one. Leave the dev-deps pointed at this worktree for Task 4.

Then pin the pada-ambiguous set from the measured failure (`pada_ambiguous_surfaces_are_exactly_these` says it is "measured (never hand-picked)"). Create `/tmp/vidyut-full/slice10i/pin_ambiguous_10i.py` if it is missing (sha256 `3ba607befca1570397cf446ea2313ed334ed28d125ae070f12b5b37f7907ba6f`):

```python
#!/usr/bin/env python3
"""THROWAWAY: pin the measured pada-ambiguous set ($SET, the failing test's
`left:` JSON) into pada_ambiguous_surfaces_are_exactly_these, and extend
the comment that accounts for it."""
import hashlib, json, os
amb = json.load(open(os.environ['SET']))
assert len(amb) == 1047, len(amb)
h = hashlib.sha256('\n'.join(amb).encode()).hexdigest()
assert h == '33dea2ecfc4a96992324e61218d8adf06e34cd01e2ce8f9713922327c3339386', h
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
old = """    // `10.0233 mAna~`, ākusmīya, has only their ātmanepada half. 184 more,
    // taking the set from 655 to 839.
"""
assert s.count(old) == 1
s = s.replace(old, old + """    // Slice 10i's sixty-one rows contribute the same four each from their
    // ṇic branch — 244 — less those already there: `10.0022 pF`'s four are
    // `10.0453 pAra`'s, and six rows share a code with a curated `Nic` row
    // whose four they already are (`tunj`, `pinj`, `lunj`, `lanj`, `lanq`, and
    // `SIk`, the ādhṛṣīya `10.0363`'s) (−28); and the two in-slice pairs,
    // `laGi~` twice and `svad` / `svAd`, share theirs (−8). `danS` and `dans`
    // add theirs: their earlier rows are ākusmīya, with only the ātmanepada
    // half. 208 more, taking the set from 839 to 1047.
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
SET="$SET" python3 /tmp/vidyut-full/slice10i/pin_ambiguous_10i.py      # pinned 1047
mise run fmt
```

If the count or hash assertion fails, stop and report. The script also extends the set's accounting comment. The 208 new surfaces are 61 × 4, less 28 for seven rows whose four another row already supplies (`pF`'s are `pAra`'s; `tunj`, `pinj`, `lunj`, `lanj`, `lanq` and `SIk` repeat a curated `Nic` row's code) and 8 for the two in-slice pairs (`laGi~` twice, `svad` / `svAd`). `danS` and `dans` add theirs, their earlier rows being ākusmīya with only the ātmanepada half.

- [ ] **Step 7: Run the full suite**

```bash
mise run fmt
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS at 25956 cells, with `panini-prakriya` 430, `trace` 214, `paradigm` 27, `panini-data` 28.

Grep the goldens for the `check()` witnesses, which the tests also enforce:

```bash
for f in grasati grAsayati grAsayate parati Gozati Gozayati DUpAyati DUpayati vicCAyati vicCayati jAyayati cAyayati cayati pUrati vartati daMsati tuYjati piYjati luYjati SIkati SIkayati laNGati laNGayate svAdayati pArayati jayati BaYjanti aYjanti vartatAm daMSati daMSayate laRqati laRqayate laYjati tantrayati grasate parate Gozate DUpati vicCati DUpAyate veCayati jayayati cayayati; do
  printf "%s: %s\n" $f "$(grep -c "\"$f\"" crates/panini/tests/paradigm/data/*.rs | grep -v ':0' | sed 's#.*/##' | tr '\n' ' ')"; done
```

Expected:
- `daMsati`, `tuYjati`, `piYjati`, `luYjati`, `SIkati`, `SIkayati`, `laNGati`, `laNGayate`, `svAdayati`, `pArayati`, `daMSati`, `daMSayate`, `laRqati` and `laRqayate` appear twice, and `laYjati` three times, in `curadi.rs` only.
- `jayati` and `vartatAm` appear once in `bhvadi.rs` and once in `curadi.rs`; `BaYjanti` and `aYjanti` once in `rudhadi.rs` and once in `curadi.rs`.
- Every other Valid witness appears once, in `curadi.rs` only.
- `tantrayati` and the nine Invalid shapes print nothing.

- [ ] **Step 8: Commit**

```bash
mise run lint
git branch --show-current      # curadi-10i
git add -A
git commit -m "feat(data): curādi's āsvadīya (10.0499), pF (2565) and Guzi~r (2571)

21564 → 25956 cells, 29096 → 36416 forms, ALTERNATES 7532 → 10460, 361 → 422
roots; pada-ambiguous surfaces 839 → 1047. OPTIONAL_NIC (180) reads the
āsvadīya by position, with 10.0368 its one exclusion; AYA holds √dhūp and
√vich; the two identical laGi~ rows are a named pair; goldens generated
cell-by-cell equal to vidyut."
```

---

## Task 4: Audit, prior-trace diff, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`
- Modify: `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`
- Modify: `crates/panini/tests/paradigm/main.rs` (audit prose), `crates/panini-prakriya/src/tinanta/guna.rs` (one comment), `crates/panini-data/src/lib.rs` (one comment)
- Modify: the 10g and 10h specs

**Interfaces:**
- Consumes: the finished engine, data and goldens. Produces no symbols.

- [ ] **Step 1: Update the audit harness**

Create `/tmp/vidyut-full/slice10i/audit_10i.py` if it is missing (sha256 `1b8d29581e4b749f59655b3f700d2eeea3fc6ed353dc8161e24d1dfc10f864be`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10i's edits to tools/audit/panini_full_audit.rs. Every
`old` must occur exactly once; nothing is written if one fails."""
import sys
p = 'tools/audit/panini_full_audit.rs'
s = open(p).read()
E = [
("""//! What it compares: for each of the 361 curated roots, for each pada the root
//! admits (`Dhatu::padas`; two apiece for the 238 roots that admit both padas —
//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 199 curādi
//! roots by 1.3.74,""",
"""//! What it compares: for each of the 422 curated roots, for each pada the root
//! admits (`Dhatu::padas`; two apiece for the 299 roots that admit both padas —
//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 260 curādi
//! roots by 1.3.74,"""),
("""//! Corpus invariants, asserted: 361 roots, 21564 cells, 29096 forms. These are""",
"""//! Corpus invariants, asserted: 422 roots, 25956 cells, 36416 forms. These are"""),
("""//! (`derivation_set_shape_matches_the_audited_numbers`): 2396 root×pada×lakāra
//! blocks × 9 cells, plus 7532 `ALTERNATES` rows.""",
"""//! (`derivation_set_shape_matches_the_audited_numbers`): 2884 root×pada×lakāra
//! blocks × 9 cells, plus 10460 `ALTERNATES` rows."""),
("""//! Optionally dump the full 21564-cell table:""",
"""//! Optionally dump the full 25956-cell table:"""),
("""    assert_eq!(roots_seen.len(), 361, "curated roots");
    assert_eq!(n_cells, 21564, "cells: 2396 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 29096, "forms: 21564 cells + 7532 ALTERNATES rows");""",
"""    assert_eq!(roots_seen.len(), 422, "curated roots");
    assert_eq!(n_cells, 25956, "cells: 2884 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 36416, "forms: 25956 cells + 10460 ALTERNATES rows");"""),
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
python3 /tmp/vidyut-full/slice10i/audit_10i.py      # applied 5 edits
```

The harness now asserts 422 / 25956 / 36416 and names 299 both-pada roots (238 + 61), 260 of them by 1.3.74.

- [ ] **Step 2: The prior-trace diff and the audit**

The dev-deps still point at this worktree from Task 3. Create `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_10i.rs` if it is missing (sha256 `559c67c2af7f3a445775cc3ca4a9b761344fa3785d9b9d37680d1511c045b8d0`). It lists 10i's rows literally so that it also builds against main, and it dumps every branch, blocked ones included:

```rust
//! THROWAWAY: slice 10i — dump every prior cell's branches, blocked ones
//! included, each with its credited-rule log.
use panini::Panini;
use panini_data::{Lakara as L, Purusha as P, Vacana as V};
/// Slice 10i's sixty-one rows, listed literally so the dump also builds
/// against main.
const NEW: &[&str] = &["10.0022", "10.0251", "10.0279", "10.0280", "10.0281", "10.0282", "10.0283", "10.0284", "10.0285", "10.0286", "10.0287", "10.0288", "10.0289", "10.0290", "10.0291", "10.0292", "10.0293", "10.0294", "10.0295", "10.0296", "10.0297", "10.0298", "10.0299", "10.0300", "10.0301", "10.0302", "10.0303", "10.0304", "10.0305", "10.0306", "10.0307", "10.0308", "10.0309", "10.0310", "10.0311", "10.0312", "10.0313", "10.0314", "10.0315", "10.0316", "10.0317", "10.0318", "10.0319", "10.0320", "10.0321", "10.0322", "10.0323", "10.0324", "10.0325", "10.0326", "10.0327", "10.0328", "10.0329", "10.0330", "10.0331", "10.0332", "10.0333", "10.0334", "10.0335", "10.0336", "10.0337"];
fn main() {
    assert_eq!(NEW.len(), 61);
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
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10i 2>/dev/null > "$DUMP/branch.txt")
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml   # must point at /workspace/crates
git -C /workspace branch --show-current   # main
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10i 2>/dev/null > "$DUMP/main.txt")
wc -l < "$DUMP/main.txt"; wc -l < "$DUMP/branch.txt"            # 33416 and 33416
grep -c "blocked=false" "$DUMP/main.txt"                        # 29096
cmp "$DUMP/main.txt" "$DUMP/branch.txt" && echo PRIOR-TRACES-IDENTICAL
```

Expected: `33416`, `33416`, `29096`, `PRIOR-TRACES-IDENTICAL`. This is the corpus-wide check that 10.0499, 2565, 2571, 3.1.28, `sanadi_pratyaya`'s guards and the sanādi 6.1.73 fire on no prior root, and change no blocked branch either (goldens ignore traces). `/workspace` must be on `main` (Task 1) for the second dump. If `cmp` reports a difference, stop and report.

Then the audit. Repoint at this worktree again, copy the committed harness (never rewrite it), and run:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
cp tools/audit/panini_full_audit.rs $V/examples/
(cd $V && PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -8)
(cd $V && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -2)
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml
```

- Expected from the honest run: `roots : 422`, `cells : 25956`, `forms (set sizes): 36416`, `live branches : 36416`, `blocked branches : 6516`, `differing cells  : 0`, `AUDIT PASSED: 25956 cells, 36416 forms, zero differences.`
- Expected from the `entry` control: `AUDIT FAILED: 36 differing cells.`

Do not use `mise -C`. If the honest run shows differences, stop and report, and edit nothing.

- [ ] **Step 3: The doc sweep, the audit record and the spec pointers**

Create `/tmp/vidyut-full/slice10i/docsweep_10i.py` if it is missing (sha256 `6c173bdadd7783fde69ad1cec21107ac31ddaaa7b83ba8f6f5dcf4a3785f8df1`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10i's doc sweep, run from the worktree root with the
audit date as argv[1]. Every `old` must occur exactly once in its file; the
script checks all of them before writing any file, and writes nothing if
one fails."""
import sys

README = [
("""8.4.44 *śāt* exemption. *curādi* (10) is **open** at 258 of its 509""",
"""8.4.44 *śāt* exemption. *curādi* (10) is **open** at 319 of its 509"""),
("""*prīṇayati*), and 7.2.114 *mṛjer vṛddhiḥ* gives √mṛj vṛddhi on both branches
(*mārjati*, *mārjayati*).
""",
"""*prīṇayati*), and 7.2.114 *mṛjer vṛddhiḥ* gives √mṛj vṛddhi on both branches
(*mārjati*, *mārjayati*). Slice 10i curated the fifty-nine āsvadīya rows,
whose ṇic the gaṇasūtra 10.0499 *ā svadaḥ sakarmakāt* makes optional (√gras,
*grāsayati* beside *grasati*), and the last two one-row optional-ṇic
triggers, Kaumudī 2565's √pṝ (*parati*) and 2571's √ghuṣ (*ghoṣati*). Where
√dhūp and √vich take no ṇic, 3.1.28 *gupūdhūpavicchipaṇipanibhya āyaḥ*
gives them āya (*dhūpāyati*, *vicchāyati*), and √vich's tuk (6.1.73 *che
ca*) comes before guṇa on every branch, so *vicchayati*, not *vechayati*.
"""),
("""curated 361-root set, in four lakāras:""",
"""curated 422-root set, in four lakāras:"""),
("""both correct — and in fact 5492 of the 21564 cells hold more than one form: 4472
hold two, 525 hold three""",
"""both correct — and in fact 7688 of the 25956 cells hold more than one form: 6424
hold two, 525 hold three"""),
("""√prī's ṇic branch forking again on 7.3.37.2's nuk),
237 hold four""",
"""√prī's ṇic branch forking again on 7.3.37.2's nuk, and slice 10i's
sixty-one add 1952 two-form cells the same way),
359 hold four"""),
("""and — new in slice 10h — forty-eight of the fifty ādhṛṣīya rows', the same
way),
ten hold""",
"""and — new in slice 10h — forty-eight of the fifty ādhṛṣīya rows', the same
way, and — new in slice 10i — the sixty-one rows', the same way),
ten hold"""),
("""7.1.35/6.4.116/8.4.56), and 241 hold six — the loṭ""",
"""7.1.35/6.4.116/8.4.56), and 363 hold six — the loṭ"""),
("""parasmaipada prathama eka, three readings × 8.4.56. One cell — new in slice 3c2 — holds seven:""",
"""parasmaipada prathama eka, three readings × 8.4.56; and, new in slice 10i,
122 more: the sixty-one rows' loṭ parasmaipada prathama and madhyama eka,
the same way. One cell — new in slice 3c2 — holds seven:"""),
("""padas — 238 roots that admit both padas in the curated set""",
"""padas — 299 roots that admit both padas in the curated set"""),
("""`pata`, slice 10g's fifty-nine optional-ṇic rows and slice 10h's fifty
ādhṛṣīya rows by 1.3.74 (the six svarita or ñit among the last by 1.3.72
too, without ṇic); and slice 10f's six""",
"""`pata`, slice 10g's fifty-nine optional-ṇic rows, slice 10h's fifty
ādhṛṣīya rows (the six svarita or ñit among them by 1.3.72 too, without
ṇic) and slice 10i's sixty-one rows by 1.3.74; and slice 10f's six"""),
("""839 of the pinned (`PARADIGM`) surfaces are pada-ambiguous""",
"""1047 of the pinned (`PARADIGM`) surfaces are pada-ambiguous"""),
("""`10.0384 mArga~`'s, which `10.0108 mArga` already supplies). The
enumeration is not""",
"""`10.0384 mArga~`'s, which `10.0108 mArga` already supplies); slice 10i's
sixty-one add 208 more, four each but where another row already supplies
them (`pF`'s are `pAra`'s; `tunj`, `pinj`, `lunj`, `lanj`, `lanq` and `SIk`
repeat earlier curādi codes) or an in-slice pair shares them (`laGi~`
twice, `svad` and `svAd`). The
enumeration is not"""),
("""set, all 839. It is therefore""",
"""set, all 1047. It is therefore"""),
]

ARCH = [
("""| `sanadi.rs` | 10.0498, 2564, 2570, 2573.1, 2573.3, 2573.2, 10.0496, 10.0497, 10.0493, 3.1.25, 1.3.9, 3.4.114, 6.4.48, 7.2.116, 6.4.92, 7.3.37.2, 7.2.115, 6.1.78, 7.2.114, 7.3.86, 3.1.32 — the optional-ṇic fork, the ākusmīya and ā-garvīya pada and the jñapādi's mit-tva, then ṇic, the adanta root's final `a`, the root's vṛddhi or guṇa or √dhū's and √prī's nuk, and ṇic's folding into the dhātu (curādi only) |""",
"""| `sanadi.rs` | 10.0498, 10.0499, 2564, 2565, 2570, 2571, 2573.1, 2573.3, 2573.2, 10.0496, 10.0497, 10.0493, 3.1.25, 3.1.28, 1.3.9, 3.4.114, 6.4.48, 7.2.116, 6.4.92, 7.3.37.2, 7.2.115, 6.1.78, 7.2.114, 6.1.73, 7.3.86, 3.1.32 — the optional-ṇic fork, the ākusmīya and ā-garvīya pada and the jñapādi's mit-tva, then ṇic, or āya where √dhūp and √vich take none, the adanta root's final `a`, the root's vṛddhi or guṇa or √dhū's and √prī's nuk, √vich's tuk ahead of guṇa, and the pratyaya's folding into the dhātu (curādi only) |"""),
("""pins all 154 ids verbatim (72 pre-rudhādi, the twenty-one rudhādi added:""",
"""pins all 159 ids verbatim (72 pre-rudhādi, the twenty-one rudhādi added:"""),
("""right before 7.3.84 — 154 total).""",
"""right before 7.3.84 — 154 total — then curādi 10i's five: the gaṇasūtra
10.0499 *ā svadaḥ sakarmakāt* after 10.0498; the Kaumudī's 2565 and 2571
after 2564 and 2570; 3.1.28 *gupūdhūpavicchipaṇipanibhya āyaḥ* after
3.1.25; and a second entry for 6.1.73 *che ca*, before the sanādi 7.3.86 —
159 total)."""),
("""— and curādi (10), **open** at 258 of its""",
"""— and curādi (10), **open** at 319 of its"""),
("""fifty ādhṛṣīya rows, slice 10h). gaṇa""",
"""fifty ādhṛṣīya rows, slice 10h; the fifty-nine āsvadīya rows, √pṝ and √ghuṣ, slice 10i). gaṇa"""),
("""from `03.0009 o~hA\\k` (both `hA`) or ghu `03.0010` from `02.0054 dA\\p`
(both `dA`).""",
"""from `03.0009 o~hA\\k` (both `hA`) or ghu `03.0010` from `02.0054 dA\\p`
(both `dA`). 3.1.28's roots reach it the same way, as `Tag::Aya` from
`panini_data::AYA`, because curādi `10.0302 gupa~` and the bhvādi `gupU~`
the sūtra names share the text `gup`."""),
("""forking 580 cells (loṭ
prathama and madhyama eka across the 290 roots with a parasmaipada column —""",
"""forking 702 cells (loṭ
prathama and madhyama eka across the 351 roots with a parasmaipada column —"""),
("""roots never reach this guard, and the 238 roots that admit both""",
"""roots never reach this guard, and the 299 roots that admit both"""),
("""`pata`, slice 10g's fifty-nine optional-ṇic rows and slice 10h's fifty
ādhṛṣīya rows by 1.3.74 (six of the last by 1.3.72 too, without ṇic), and slice 10f's six""",
"""`pata`, slice 10g's fifty-nine optional-ṇic rows, slice 10h's fifty
ādhṛṣīya rows (six of them by 1.3.72 too, without ṇic) and slice 10i's
sixty-one rows by 1.3.74, and slice 10f's six"""),
("""290 + 71 = the 361 curated roots)""",
"""351 + 71 = the 422 curated roots)"""),
("""forking 536 cells outright: laṅ and vidhiliṅ prathama eka across
those same 290 parasmaipada columns (513 of them""",
"""forking 636 cells outright: laṅ and vidhiliṅ prathama eka across
those same 351 parasmaipada columns (613 of them"""),
("""and so do 10f's `mUtra`, `katra` and `pata`, 10g's rows other than √śṛdh and √div and 10h's other than its sixteen laghu-ik rows (whose ṇic branch keys on the sanādi 7.3.86 as well) on their ṇic branch; 10f's, 10g's and 10h's ṇic-less forks key on their optional-ṇic id as well — `2564+8.4.56`, `10.0498+8.4.56` and their siblings — as does √dhū's and √prī's nuk branch on 7.3.37.2, and sit outside this count),""",
"""and so do 10f's `mUtra`, `katra` and `pata`, 10g's rows other than √śṛdh and √div, 10h's other than its sixteen laghu-ik rows and 10i's other than its eleven (whose ṇic branch keys on the sanādi 7.3.86 as well) on their ṇic branch; 10f's to 10i's ṇic-less forks key on their optional-ṇic id as well — `2564+8.4.56`, `10.0498+8.4.56`, `10.0499+8.4.56` and their siblings — as does √dhū's and √prī's nuk branch on 7.3.37.2, and sit outside this count),"""),
("""forking a further 580 (the same""",
"""forking a further 702 (the same"""),
]

AGENTS = [
("""(`crates/panini/tests/paradigm/`, 21564 cells, ten gaṇas, nine complete —""",
"""(`crates/panini/tests/paradigm/`, 25956 cells, ten gaṇas, nine complete —"""),
("""at 258 after slice 10h curated the fifty ādhṛṣīya rows —""",
"""at 258 after slice 10h curated the fifty ādhṛṣīya rows, at 319 after slice 10i curated the fifty-nine āsvadīya rows, √pṝ and √ghuṣ —"""),
("""other forms — a second (4472 cells), a third (525 cells), a fourth
    (237""",
"""other forms — a second (6424 cells), a third (525 cells), a fourth
    (359"""),
("""    fifty-nine optional-ṇic rows', and — new in slice 10h — forty-eight
    ādhṛṣīya rows') and""",
"""    fifty-nine optional-ṇic rows', and — new in slice 10h — forty-eight
    ādhṛṣīya rows', and — new in slice 10i — the sixty-one āsvadīya, √pṝ
    and √ghuṣ rows') and"""),
("""    eka to 241 — a fourth""",
"""    eka to 241, and slice 10i's sixty-one rows (loṭ prathama and madhyama
    eka) to 363 — a fourth"""),
("""    `ALTERNATES` (7532 rows in all, so 21564 + 7532 = 29096 forms total); √bhuj""",
"""    `ALTERNATES` (10460 rows in all, so 25956 + 10460 = 36416 forms total); √bhuj"""),
("""  (`tools/audit/README.md`'s 2026-10-04 10h entry, 21564 cells / 29096 forms /
  361 roots).""",
"""  (`tools/audit/README.md`'s 2026-10-04 10h entry, 21564 cells / 29096 forms /
  361 roots), and that by curādi 10i's (`tools/audit/README.md`'s @DATE@ 10i
  entry, 25956 cells / 36416 forms / 422 roots)."""),
("""only in the ordinary corpus-size sense, not wrong in kind: 21564 goldens""",
"""only in the ordinary corpus-size sense, not wrong in kind: 25956 goldens"""),
]

TOOLS_README = [
("""**It asserts the corpus totals** (361 roots, 21564 cells, 29096 forms) rather than""",
"""**It asserts the corpus totals** (422 roots, 25956 cells, 36416 forms) rather than"""),
("""## Last recorded result

""",
"""## Last recorded result

@DATE@, curādi 10i slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 25956
cells / 36416 forms / 422 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole curādi 10i slice: the fifty-nine āsvadīya rows,
whose ṇic the gaṇasūtra 10.0499 makes optional, and `10.0022 pF` and
`10.0251 Guzi~r`, whose ṇic Kaumudī 2565 and 2571 make optional, each derived
on its ṇic and its ṇic-less branch; and 3.1.28's āya on √dhūp's and √vich's
ṇic-less branch and the sanādi 6.1.73's tuk on √vich's every branch, which
the slice adds. Blocked branches rose from 4320 to 6516, the 2196 = 61 × 36
ṇic-less ātmanepada cells. Disabling the sanādi 6.1.73, as a control, made
72 cells differ. A main-vs-branch dump of every prior cell's branches,
blocked ones included, was byte-identical, all 33416 of them (29096 live).

Totals: 422 = 361 + 61; 25956 = 21564 + 4392 (488 root×pada×lakāra blocks ×
9); 36416 = 29096 + 4392 + 2928 new `ALTERNATES` rows (7532 → 10460),
measured via the harness's corpus block, not assumed.

"""),
]

MAIN_RS = [
("""/// curādi 10h's re-ran it at the same commit over all 21564 cells / 29096
/// forms / 361 roots with zero differences, its `entry` negative control
/// verified failing (36 √bhū cells). √tṛh joins none of the fork""",
"""/// curādi 10h's re-ran it at the same commit over all 21564 cells / 29096
/// forms / 361 roots with zero differences, its `entry` negative control
/// verified failing (36 √bhū cells), and curādi 10i's re-ran it at the same
/// commit over all 25956 cells / 36416 forms / 422 roots with zero
/// differences, its `entry` negative control verified failing (36 √bhū
/// cells). √tṛh joins none of the fork"""),
]

GUNA = [
("""    // 361-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)""",
"""    // 422-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)"""),
]

DATA = [
("""    /// vendored upadeśa: 73 of the 361 curated roots carry a `\\` at all, and 52""",
"""    /// vendored upadeśa: 73 of the 422 curated roots carry a `\\` at all, and 52"""),
]

SPEC_10G = [
("""  exclusion assertion. (Slice 10h took fifty of the ādhṛṣīya, all but
  `10.0368 za\\da~`: see `2026-10-03-curadi-gana-10h-design.md`.)
- The one-row triggers 2565 (`pF`), 2571 (`Guzi~r`) and 2572 (īdit).""",
"""  exclusion assertion. (Slice 10h took fifty of the ādhṛṣīya, all but
  `10.0368 za\\da~`: see `2026-10-03-curadi-gana-10h-design.md`. Slice 10i
  took the āsvadīya: see `2026-10-04-curadi-gana-10i-design.md`.)
- The one-row triggers 2565 (`pF`) and 2571 (`Guzi~r`), both taken by slice
  10i. 2572 (īdit) is unreachable: every īdit curādi row is āsvadīya or
  ādhṛṣīya, and vidyut checks both antargaṇas first."""),
]

SPEC_10H = [
("""- 10.0499 āsvadīya (59 rows, 279–337). Teaches the helper its range and
  drops the rest of the exclusion assertion.
- The one-row triggers 2565 (`pF`), 2571 (`Guzi~r`) and 2572 (īdit).""",
"""- 10.0499 āsvadīya (59 rows, 279–337). Teaches the helper its range and
  drops the rest of the exclusion assertion. (Slice 10i took it: see
  `2026-10-04-curadi-gana-10i-design.md`.)
- The one-row triggers 2565 (`pF`) and 2571 (`Guzi~r`), both taken by slice
  10i. 2572 (īdit) is unreachable: every īdit curādi row is āsvadīya or
  ādhṛṣīya, and vidyut checks both antargaṇas first."""),
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
    "docs/superpowers/specs/2026-10-03-curadi-gana-10h-design.md": SPEC_10H,
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
python3 /tmp/vidyut-full/slice10i/docsweep_10i.py "$(date -u +%F)"      # applied 41 edits to 9 files
```

If it prints `not applied:`, nothing was written. Edit the paragraph named to the same facts, rather than skipping it, and re-run. (The prototype and its replay ran it with `2026-10-04`; another date changes only the two dated lines.)

Notes on the numbers, re-derived from the census rather than assumed:
- **Multi-form cells:** 7688 = 25956 − 18268. The buckets are 6424 / 525 / 359 / 10 / 363 / 1 / 6, and no cell holds eight.
- **Both-pada roots:** 299 = 238 + 61. Every new row has a parasmaipada column, so 351 + 71 = 422: the ātmanepada-only count is unchanged.
- **7.1.35 forks:** 702 = 351 × 2, its loṭ prathama and madhyama eka; 8.4.56's "further" fork covers the same cells.
- **8.4.56 forks:** 636 = `key_count("8.4.56")` = 613 + 22 + 1, where 613 = 513 + 100. Fifty new rows' ṇic branches fork laṅ and vidhiliṅ prathama eka on 8.4.56 alone; the eleven laghu-ik rows key on 7.3.86 as well, and the ṇic-less forks on their optional-ṇic id, so neither sits in it.
- **The rule-order pin:** 159 = 154 + 5: 10.0499, 2565, 2571, 3.1.28 and the sanādi 6.1.73.
- **`pada_from_upadesha`'s doc:** "73 of the 422 curated roots carry a `\` at all, and 52": no new row's upadeśa carries a `\` (measured), so only the total moves.
- **The ARCHITECTURE root-identity paragraph** gains 3.1.28's `Tag::Aya` beside 1.1.20's `Tag::Ghu`: both are root-naming sūtras keyed by row number.

- [ ] **Step 4: The AGENTS.md stale-comment ledger**

Create `/tmp/vidyut-full/slice10i/ledger_10i.py` if it is missing (sha256 `936ba3da111341c9ca61b6bceb69bfd2a866c3aed9490e7daa9818812987f22b`). It measures both anchors by grep at this commit; it never computes them:

```python
#!/usr/bin/env python3
"""THROWAWAY: add 10i's sentence to AGENTS.md's stale-comment ledger, with
both anchors measured by grep now."""
import re, subprocess
g = subprocess.run(['grep', '-n', '1872 goldens move', 'crates/panini-prakriya/src/tinanta/guna.rs'], capture_output=True, text=True).stdout.split(':')[0]
c = subprocess.run(['grep', '-n', 'only 8 cells fire', 'crates/panini-prakriya/src/controller.rs'], capture_output=True, text=True).stdout.split(':')[0]
assert g and c
p = 'AGENTS.md'
s = open(p).read()
m = re.search(r"Curādi 10h touched neither comment either, though its guṇa-stage 7\.2\.114 moved the first; the corpus stands at 21564 cells as of 10h \(`guna\.rs:2565`'s claim anchored at `guna\.rs:\d+`, `controller\.rs:206`'s at `controller\.rs:\d+`; both lines measured by grep at this commit\)\.", s)
assert m
new = m.group(0) + f" Curādi 10i touched neither comment either; the corpus stands at 25956 cells as of 10i (`guna.rs:2641`'s claim anchored at `guna.rs:{g}`, `controller.rs:206`'s at `controller.rs:{c}`; both lines measured by grep at this commit)."
s = s[:m.start()] + new + s[m.end():]
open(p, 'w').write(s)
print(f"ledger: guna.rs:{g} controller.rs:{c}")
```

```bash
python3 /tmp/vidyut-full/slice10i/ledger_10i.py      # ledger: guna.rs:2641 controller.rs:206
```

Neither anchor moves: the slice adds nothing to `guna.rs` above line 2641 or to `controller.rs`. AGENTS.md's floor paragraph (`measured at 21564 cells`) and the current mutation record belong to Task 5.

- [ ] **Step 5: Sweep for anything left stale**

```bash
grep -rn -i -E "\b21564\b|\b29096\b|\b7532\b|\b2396\b|361 roots|361-root|of these 361|of the 361|361 curated|258 of|\b238 roots|\b199 curādi|199 \`Nic\`|\b839\b|\b16072\b|\b4472\b|290 \+ 71|forking 580|\b536 cells|\b4320\b|\b154 ids|154 total|237 hold|241 hold|\b119\b|two hundred fifty-eight|all six inert" README.md AGENTS.md docs/ARCHITECTURE.md tools/audit/README.md crates --include=*.md --include=*.rs | grep -v "paradigm/data/"
grep -rn -i -E "2564, 2570, 2573|10\.0498, 2564|five (optional-ṇic|vikalpa)|four (Kaumudī )?vikalpa|four gaṇasūtras|five Kaumudī ids|2565 / 2571|later slices'|āsvadīya.{0,40}(later|not yet|uncurated|keeps the table out)|laṅ-only|only site|this corpus's only|only on kzamp|natva_crosses_num_only_on_kzamp\b|no curated root presents|eighth vikalpa" README.md AGENTS.md docs/ARCHITECTURE.md tools crates --include=*.md --include=*.rs | grep -v "paradigm/data/"
for code in pF Guz gras puz dal paw puw luw tunj minj pinj lak lunj Banj lanG trans pins kuns danS kunS Gaw Ganw bfnh barh balh gup DUp viC cIv puT lok loc nad kup tark vft vfD ruw lanj anj dans BfnS runS SIk runs nanw punw ji ci ranD anh ranh manh lanq taq nal pUr ruj svad svAd; do
  r=$(grep -rln "\"$code\"" crates --include=*.rs | grep -v "paradigm/data/" | LC_ALL=C sort | tr '\n' ' '); echo "$code: $r"; done | grep -v -E "^[A-Za-z]+: crates/panini-data/src/lib.rs $|^[A-Za-z]+: crates/panini-data/src/lib.rs crates/panini/tests/paradigm/main.rs $"
```

Expected residue from the first grep, all of it dated history that stays true, plus the numbers that merely share digits:
- `tools/audit/README.md`'s 10i entry's own `Totals:` and dump lines, and the 10h and older entries;
- AGENTS.md:37's floor paragraph (Task 5), its audit chain (the 10h clause) and its stale-comment ledger;
- `paradigm/main.rs`'s audit chain (the 10h clause), its 10h paragraph ("OPEN at 258") and the pada-ambiguous comment's "655 to 839" / "839 to 1047";
- ARCHITECTURE's pin chain ("— 154 total — then curādi 10i's five");
- every `6.4.119` hit (the sūtra id, not a count).

The second grep prints only dated history and unrelated phrasings, twelve lines: `README.md`'s and `tools/audit/README.md`'s 10f sentences ("Kaumudī 2564, 2570, 2573.1 …"), `paradigm/main.rs`'s 10f paragraph, ARCHITECTURE's 10f pin-chain clause, `derivation_tests.rs`'s 10f order-pin sentence ("2564, 2570, 2573.1 and 2573.3 fork …") and its "the ekādeśa is laṅ-only", `juhotyadi.rs`'s two 8.3.24 notes on 10f's "five optional-ṇic … roots", `adesha.rs`'s two "laṅ-only" āṭ notes, and `anga.rs`'s "this entry's only site" and "No curated root presents this shape today" (the across-term-boundary case, still true).

The third loop prints the new codes Task 2's and Task 3's tests construct (`pF`, `Guz`, `gras`, `tunj`, `danS`, `gup`, `DUp`, `viC`, `vft`, `pUr`, in `sanadi.rs`) and codes pre-existing roots already use in unit tests (`pF` in `abhyasa.rs` and `guna.rs`, `nad` in `anga.rs` and `tripadi.rs`, `vft` in `guna.rs`, `ji` in `abhyasa.rs` and `derivation_tests.rs`, `ci` in `abhyasa.rs`, `guna.rs` and `tripadi.rs`, `pUr` in `tripadi.rs`). Any other hit from any of the three is a miss. Edit it.

- [ ] **Step 6: Run the full suite and commit**

Run: `mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"` (foreground, timeout 600000 ms). Expected: PASS at 25956 cells.

```bash
mise run fmt && mise run lint
git branch --show-current      # curadi-10i
git add -A
git commit -m "docs: 10i's counts, the audit record, and the sweep

25956 cells / 36416 forms / 422 roots across README, ARCHITECTURE, AGENTS,
paradigm/main.rs and tools/audit; curādi open at 319/509; 299 both-pada
roots; 1047 pada-ambiguous surfaces; 159 pinned rule ids. Audit at zero
divergence against 8da2f90b with 6516 blocked branches; prior traces
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
- **The mutant list grows by six.** `panini-prakriya` lists 850 mutants (844 on main); `panini-analyze` 12 and `panini-data` 12, both unchanged. By name, nine entries leave and fifteen arrive: the eight mutants inside 6.1.73's closure in `anga.rs` reappear inside `che_ca`, and one `sanadi.rs` `delete !` goes net (3.4.114's and 3.1.32's guards lose theirs; 3.1.28's guard brings one). The seven others are new: `che_ca -> bool` with `true` and with `false`; `sanadi_pratyaya -> Option<&Term>` with `None` and with `Some(Box::leak(Box::new(Default::default())))` (UNVIABLE: `Term` has no `Default`), its `||` → `&&` and `==` → `!=`; and 3.1.28's `||` → `&&`.
- **`--in-diff` over the slice's production diff for `panini-prakriya`** lists 18, all expected CAUGHT but the one unviable: `derive` (two, `mod.rs:82`), `che_ca` (ten, `anga.rs:19`–`25`), `sanadi_pratyaya` (four, `sanadi.rs:71`–`73`), and 3.1.28's guard (`sanadi.rs:351:16` delete `!`, `351:45` `||` → `&&`). Each new one is killed by a Task 2 unit test (`aya_follows_a_tagged_root_that_took_no_nic`, `aya_folds_into_the_dhatu_without_nijanta`, `ardhadhatuka_sesah_tags_the_sanadi_pratyaya_only`, `che_ca_gives_vich_tuk_before_guna_can_read_its_upadha`) and by the goldens.
- **`--in-diff` over the data crate's diff** lists none: the slice changes only constants, tests and docs there.
- **The 51 documented non-caught entries are unchanged, spans included.** `adesha.rs:647:30` (MISSED), `tripadi.rs:1305:38` (MISSED) and `tripadi.rs:1618:23` (the permanent TIMEOUT) are where main has them, and no `guna.rs` line moves. Confirm `<A>`, `<T1>` and `<T2>` by `--list`; never compute them.
- **The floor at 25956 cells** was not measured on a quiet host: the prototype's full suite ran with the `trace` binary at 49–230 s under external load. Measure it in Step 1. The cap in force is 1810.

- [ ] **Step 1: Measure the floor**

With nothing else of ours running, run this twice: `cat /proc/loadavg; time mise run test >/dev/null 2>&1; cat /proc/loadavg` (foreground, timeout 600000 ms). Record both wall clocks, user CPU and the load averages. Read 10h's floor from AGENTS.md's floor paragraph and keep the comparison chain.

- [ ] **Step 2: Locate and probe the two uncaught equivalents at `-j 4`**

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
SCRATCH="$(mktemp -d)"
mise exec -- "$CM" mutants --package panini-prakriya --list -o "$SCRATCH" 2>/dev/null | wc -l      # 850
mise exec -- "$CM" mutants --package panini-prakriya --list -o "$SCRATCH" 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /="
```

Expect `647` (adesha), `1305` (tripadi, inside 8.3.13's `apply`) and `1618` (the ṇatva hang). Write them as `<A>`, `<T1>` and `<T2>`. Then run in the foreground with timeout 600000 ms:

```bash
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 1810 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:<A>:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" 2>&1 | tail -6
```

The two regexes also match two caught `mod.rs` mutants; that is expected. Both equivalents must be MISSED, not TIMEOUT. Read each test-phase duration from `$SCRATCH/mutants.out/outcomes.json`. Set the provisional cap to max(1810, 6 × the longer of the two, rounded up to the next 10 s). If the probe outgrows the 10-minute foreground cap, rerun it detached with a log and an `EXIT_CODE=` sentinel and wait on its PID in the same turn.

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/curadi-10i"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout <CAP> -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"; cat /proc/loadavg > "$OUT/load.started"
```

`<CAP>` is Step 2's provisional cap. Run nothing CPU-heavy meanwhile. 10h's campaign took 2h48m at 21564 cells; expect longer. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

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
- **862 mutants: 810 caught, 49 unviable, 2 missed, 1 timeout.**
  - panini-prakriya: 850 / 802 / 45 / 2 / 1.
  - panini-analyze: 12 / 8 / 4 / 0 / 0.
- `missed.txt` holds exactly `adesha.rs:<A>:30: replace + with *` and `tripadi.rs:<T1>:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:<T2>:23: replace -= with /=`.

Then diff the non-caught set against 10h's on the full record, both with and without span lines:

```bash
python3 - "$OUT/outcomes.durable.json" /home/dev/mutants-records/curadi-10h/outcomes.durable.json <<'PY'
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
- **Without lines:** `25 24`, `new:` exactly the `sanadi_pratyaya` `Some(Box::leak(Box::new(Default::default())))` UNVIABLE, `gone: []`.
- **With lines:** `52 51`, `new:` that same entry alone, `gone: []`. No non-caught span moves.

If not:
- Any **other timeout** is a suspect survivor that the larger suite pushed past the cap. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant among the new ones is a gap in Task 2's tests; add the test that kills it. Any other missed mutant means a test that caught it at 21564 cells no longer does; stop and report.

**Step 4b: the data-crate mutants.** Confirm the slice's diff for `panini-data` holds none:

```bash
git diff a55d993 -- crates/panini-data/src/lib.rs > "$OUT/data.diff"
mise exec -- "$CM" mutants --package panini-data --list --in-diff "$OUT/data.diff" -o "$OUT/data" 2>/dev/null | wc -l      # 0
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

- **The floor paragraph.** Rewrite the paragraph that opens `**The floor behind the 1810s cap, measured at 21564 cells on Rust 1.99.0,`. Use Step 1's and Step 2's numbers at 25956 cells, the load averages, and the cap Step 5 chose. Keep the comparison chain to earlier floors, with 10h's joining it: 1m27.729s / 2m20.140s at 21564 cells, isolated probe 160.99s / 160.28s, campaign-load 142.46s / 171.58s.
- **The current record.** Replace the `**Current record (curādi 10h, …).**` paragraph with `**Current record (curādi 10i, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**;
  - the non-caught set diffed against 10h's on the full record, both ways (Step 4's script output), naming the one new unviable;
  - the seven new viable mutants, every one CAUGHT, by name, and the eight that moved into `che_ca`;
  - Step 4b's empty data-crate list;
  - the campaign-load phases and margins;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`.

  End it with a pointer to the record it replaces. Run `git rev-parse --short HEAD` before committing, and write ``The curādi 10h record it replaces: `git show <that hash>:AGENTS.md`.``

- [ ] **Step 7: Commit**

```bash
git branch --show-current      # curadi-10i
git add AGENTS.md mise.toml
git commit -m "chore: 10i mutation gate — floor and uncaught run re-measured at 25956 cells

Every new viable mutant caught; missed.txt holds only the two documented
equivalents and timeout.txt only the permanent ṇatva-scan entry. The
non-caught set is 10h's plus sanadi_pratyaya's one unviable Default mutant,
no span moved."
```

---

## Task 6: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # back at /workspace/crates
git branch --show-current      # curadi-10i
git log --oneline main..HEAD   # the spec, the plan and the four task commits
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin curadi-10i
gh pr create --title "curādi 10i — the āsvadīya (10.0499), √pṝ (2565), √ghuṣ (2571)" --body "$(cat <<'BODY'
Slice 10i curates the fifty-nine āsvadīya rows (`10.0279 grasa~` …
`10.0337 svAda~`), whose ṇic the dhātupāṭha gaṇasūtra 10.0499 *ā svadaḥ
sakarmakāt* makes optional, and the last two one-row optional-ṇic triggers,
`10.0022 pF` (Kaumudī 2565) and `10.0251 Guzi~r` (Kaumudī 2571). 2572 (īdit)
is unreachable in curādi and is pinned so.

- 10.0499 joins the optional-ṇic forks right after 10.0498, 2565 and 2571
  beside 2564 and 2570, as vidyut checks them.
- 3.1.28 *gupūdhūpavicchipaṇipanibhya āyaḥ* gives √dhūp and √vich āya where
  they take no ṇic (*dhūpāyati*, *vicchāyati*), keyed by row number
  (`panini_data::AYA`); 3.4.114 and 3.1.32 treat āya as ṇic, without
  `Nijanta`.
- A second 6.1.73 *che ca*, sharing the aṅga entry's apply, gives √vich its
  tuk before guṇa (*vicchayati*, not *vechayati*).
- `OPTIONAL_NIC` (180) reads the āsvadīya by position; the two identical
  `laGi~` rows are a named pair in the uniqueness test.

The golden suite goes from 21564 to 25956 cells; curādi is open at 319 of
509. The audit shows zero divergence against `8da2f90b`, a main-vs-branch
dump of every prior branch (blocked ones included) is byte-identical, and
the mutation gate finds every new mutant caught and the documented
non-caught set unchanged.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`:
   - run `git worktree remove .worktrees/curadi-10i`;
   - run `git worktree remove --force .worktrees/curadi-10i-proto` and `git branch -D proto-10i` (the throwaway);
   - run `git worktree remove --force .worktrees/curadi-10i-replay` if the replay worktree is still there (detached; no branch);
   - run `git worktree remove --force .claude/worktrees/agent-aaca91506ee5b6189` and `git branch -D worktree-agent-aaca91506ee5b6189` (the exploratory prototype the spec's Evidence cites);
   - delete the local and remote `curadi-10i` branch;
   - run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| 61 rows, codes, all `Nic`; `10.0368` out | 3 Step 6 |
| `OPTIONAL_NIC` 119 → 180: 10.0499 × 59, 2565, 2571 | 3 Steps 3, 6 |
| 10.0499 / 2565 / 2571 as `skip_nic` vikalpas, in vidyut's order, 2564's bars | 2 |
| helper: the range after 10.0498, `F` → 2565, `Guzi~r` → 2571; exclusion narrowed to `10.0368` | 3 Step 3 |
| 2572 unreachable (Out) | 3 Step 3 (marker pin); 4 Step 3 (10g/10h specs) |
| 3.1.28 on the ṇic-less branch, `Tag::Aya` from `AYA`, data test from vidyut's five upadeśas | 2; 3 Step 3 |
| 3.4.114 and 3.1.32 take āya; `Nijanta` for ṇic only | 2 |
| 3.1.28 not a vikalpa (3.1.31 out) | 2 (order pin, not the vikalpa pin) |
| sanādi 6.1.73 before the sanādi 7.3.86, sharing `che_ca`; stage-scoped lookups | 2 |
| `laGi~` pair: named exception and identity pin | 3 Step 3 |
| Stored codes need nothing new | 3 Step 6 (`dhatupatha_numbers_resolve_upstream`) |
| Homographs (codes gaining rows, in-slice pairs, other gaṇas), `daMSayati` negative witness gone | 3 Steps 1, 4, 7 |
| Goldens via the harness; census 422 / 25956 / 36416; blocked 4320 → 6516 | 3 Steps 1, 6; 4 Step 2 |
| 1.3.74 count 199 → 260; `curated_roots_have_expected_ganas_and_padas` 422; pada-ambiguous 839 → 1047 | 3 Steps 2, 3, 6 |
| Rule pins: order (+10.0499, 2565, 2571, 3.1.28, sanādi 6.1.73), vikalpa and bars (+3 ids) | 2 |
| `the_optional_nic_ids_…` gains all three; `no_nic_pada_rule_…` allows 3.4.114 and 3.1.32 on the āya branch | 3 Step 2 |
| Unit tests: each guard both ways (the three ids, 3.1.28, 3.1.32's `Nijanta`, the sanādi 6.1.73) | 2; 3 Step 3 (the three ids on their rows) |
| Corpus-wide: 3.1.28 on exactly `10.0303`/`10.0304`, ṇic-less only; rosters 7.2.115, sanādi 6.1.78, 8.3.24, 6.1.73/8.4.40; 8.4.2-across-num three rows | 3 Step 2 |
| Prior-trace diff main ↔ HEAD | 4 Step 2 |
| Witnesses from the spec's tables; 10g's test credits its trigger per row | 3 Steps 1, 4, 7 |
| Mutation gate: scoped, floor re-measured (trace binary note), `-o`, durable copy, chunking, non-caught named verbatim | 5 |
| AGENTS.md (progress line, corpus counts, fork/blocked census, audit and mutation records), ARCHITECTURE (pin count, sanādi row, fork counts), `tools/audit/README.md`, `panini-data` docs (`OPTIONAL_NIC`, helper, `AYA`), sanādi module doc, `anga.rs`'s 6.1.73 comment | 2; 3 Step 3; 4 Steps 1, 3, 4; 5 |
| 10g's and 10h's "Later slices": pointer, drop 2572 | 4 Step 3 |
| Sweep greps: root-shape literals, all of `crates/`, sibling enumerations, 6.1.73 phrasings, spelled-out and wrapped counts | 4 Steps 3, 5 |

**Type consistency.**
- `panini_data::AYA: [&str; 2]`, read by `tinanta::derive` (`AYA.contains(&dhatu.dhatupatha)`) and compared with a `Vec<&str>` in `aya_is_exactly_the_curated_rows_3_1_28_names`.
- `che_ca(p: &mut Prakriya) -> bool` is a plain `fn`, so both entries name it as `apply: che_ca` (`Rule::apply` is `fn(&mut Prakriya) -> bool`).
- `sanadi_pratyaya(p: &Prakriya) -> Option<&Term>`; its two callers test `.is_none()`.
- `optional_nic_from_upadesha(number: &str, upadesha: &str) -> Option<&'static str>` is unchanged in signature.
- `curadi_analyses_its_optional_nic_forms`' tuples are `(&str, &[&str], Pada, &[&str])`; the first tuple's `[..]` on both slices fixes the types. `curadi_analyses_its_asvadiya_forms`' are `(&str, &[&str], Pada, &[&str], &[&str])`, likewise.
- `IDENTICAL_UPSTREAM_PAIR: [&str; 2]` is iterated by value and indexed.
- The golden tuple shapes match `ParadigmRow` and `AlternateRow`; the generator is 10h's, with the row filter and the vikalpa list changed.

**Known soft spots.**
- **Doc strings in Task 4** were read at the prototype's state. If the script reports an `old` not found exactly once, edit that paragraph to the same facts rather than skip it.
- **Task 5's campaign numbers are expectations**, not prototype measurements. The floor and cap depend on host load; record the load beside every timing.
- **Where this plan goes beyond the spec's letter**, each to make the spec's decisions work:
  - **`sanadi_pratyaya` knows āya by its text** (`"Aya"`), as 1.3.9 knows ṇic by `"Ric"`; ṇic by `Tag::Rit`.
  - **3.1.28's guard is `Tag::Aya` with nothing at `NIC`**, with no gaṇa test: `AYA` holds row numbers, and a future bhvādi `gupU~` takes āya with no ṇic in play.
  - **The sanādi 6.1.73's unit test asserts `vitC`**, the sanādi-stage text; the spec's "(`vicC`, not `veC`)" is the surface after 8.4.40, which `vicCayati_and_vicCAyati_take_tuk_before_3_1_32` and the witnesses hold.
  - **The marker test pins the spec's Out claim** that 2565 and 2571 reach one row each and 2572 none.
  - **`natva_crosses_num_only_on_kzamp` is renamed** (`…_kzamp_bfnh_and_ranh`), since its name stops being true.
  - **`ardhadhatuka_sesah_tags_nic_only` is renamed** (`…_the_sanadi_pratyaya_only`), for the same reason.
  - **10g's witness test's trigger becomes a list**, one id per analysis, and its negative witness `daMSayati` becomes `tantrayati`.
  - **`anga.rs`'s 6.1.76 note loses "eighth vikalpa"**, stale long before this slice, in the paragraph the slice rewrites.
  - **The order pin's doc gains a 10i paragraph and its non-Aṣṭādhyāyī census** (five gaṇasūtras, seven Kaumudī ids).
  - **The prior-trace dump includes blocked branches** (33416 lines, not 10h's 29096 live ones).
  - **ARCHITECTURE's root-identity paragraph names `Tag::Aya`** beside `Tag::Ghu`.
- **The pada-ambiguous count's arithmetic** (244 − 28 − 8) is derived after the fact. The set itself is the measured one, hash-pinned by the script.
