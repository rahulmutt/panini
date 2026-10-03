# Curādi gaṇa slice 10g Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate the fifty-nine remaining optional-ṇic curādi rows that Kaumudī 2564 (idit, 53 rows) and 2570 (udit, 6 rows) open, across laṭ / laṅ / loṭ / vidhiliṅ, and let 8.4.2 read num as the anusvāra √kṣamp needs (*kṣampāṇi*). The golden suite goes from 13716 to 17964 cells, and curādi from 149 to 208 of its 509 rows.

**Architecture:** Six tasks:
- **Task 1** creates the worktree and checks the baseline.
- **Task 2** is the one engine change: 8.4.2's intervener set (`is_natva_intervener`) gains the anusvāra `M`, with its doc comment and two unit tests. It lands green on its own: no curated root before this slice has an anusvāra between a ṇatva trigger and its target.
- **Task 3** lands the fifty-nine rows, their `OPTIONAL_NIC` entries, 472 golden rows and 2832 alternates, and every count, list, roster, uniqueness and `check()` assertion they move. The assertions go in first and fail; the rows and goldens make them pass.
- **Tasks 4–6** are the audit with the prior-trace diff and the doc sweep, the mutation gate, and the branch finish.

No rule id, tag or `PadaAssignment` variant is added: 10f's sanādi fork (2564, 2570 at the head of `SANADI`, keyed on `OPTIONAL_NIC`) already handles every row.

**Tech Stack:** Rust 1.99.0, pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, checked out at `/tmp/vidyut-full`.

**Spec:** `docs/superpowers/specs/2026-10-03-curadi-gana-10g-design.md`. Its row tables are the slice's scope; its Decisions section explains the 8.4.2 reading, the range exclusion and the verdict-aware uniqueness filter.

**Workspace:** Task 1 creates the branch `curadi-10g` from `main` (which holds the spec and this plan) at `/workspace/.worktrees/curadi-10g`. Every path below is relative to that directory unless it starts with `/`.

**Provenance.** This slice was built end to end on a throwaway worktree, `/workspace/.worktrees/curadi-10g-proto` (detached, throwaway commits `bc2388f` engine, `eafc344` rows and assertions, `ff160f0` audit and docs, on the spec commit `cdf78e0`), which Task 6 deletes. Every code block and script below is that prototype's code, and was checked two ways:
- the prototype's final state passed the full suite, clippy `-D warnings` and `fmt-check`; its generator found all 4248 new cells equal to vidyut's (4248 cells, 7080 forms, 0 differences); the audit at 311 roots / 17964 cells / 22724 forms showed zero differences, with 2736 blocked branches and the `entry` control failing on 36 cells; a main-vs-prototype dump of all 15644 prior live branches was byte-identical;
- the scripts below, run in this plan's order on a fresh worktree at `cdf78e0` (`/tmp/vidyut-full/slice10g/replay_10g.sh`, a verification aid this plan does not need), reproduced the prototype byte for byte in every tracked file. Task 2 alone, replayed, was green: `panini-prakriya` 419, `panini-data` 27, `trace` 208, `paradigm` 25. Task 3 Step 5's failing list below is that replay's.

The prototype did **not** run the mutation campaign; Task 5's campaign numbers are expectations, derived from the unchanged mutant list (below), and the campaign measures them.

**Throwaway scripts.** Everything under `/tmp/vidyut-full/slice10g/` and the two vidyut examples (`curadi_goldens_10g.rs`, `trace_dump_10g.rs`) never ship. Each is reproduced in full in this plan with its sha256, so it can be recreated if `/tmp` was cleaned. Recreate a file only if it is missing, and check its hash either way (`sha256sum <file>`).

## Global Constraints

- **No new rule ids, tags or `PadaAssignment` variants.** The one engine change is `'M'` in `is_natva_intervener` (`crates/panini-prakriya/src/tinanta/sound.rs`), with its doc comment.
- **Rows:** exactly the spec's fifty-nine, every one `PadaAssignment::Nic`, each `code` what `stored_form` computes. `OPTIONAL_NIC` has exactly 69 entries, in dhātupāṭha order.
- **Not modelled:** 7.1.58 as a credited step (the stored-num simplification stands), 7.3.59 on √kṣañj, and 7.2.115 (so `10.0124 ciY` stays out).
- **Out of scope:** `10.0124 ciY`; every row in `10.0279..=10.0388` (āsvadīya, ādhṛṣīya); 2565, 2571, 2572; the causative.
- **Pre-existing cells must stay byte-identical, traces included,** with no exception. Regenerate no prior golden.
- **Goldens come from the generator, which asserts engine = vidyut cell by cell, and their sha256 must match this plan's.** **Do not edit a golden to match the engine.** If the generator reports a difference, or a hash differs, stop and report.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit. Run `git branch --show-current` before every commit and the push: it must print `curadi-10g` (a detached HEAD strands commits).
- `mise run test` takes about 100 s on this host under its usual external load (load 40–60). Run it in the **foreground** with a timeout of 600000 ms. Never background it and end a turn.
- `mise run test -- -p X` does not scope. Scope with `mise exec -- cargo test -p <crate> <filter>`. To see every failing binary at once, use `mise exec -- cargo test --workspace --no-fail-fast`.
- The `cargo-mutants` mise shim fails here. Use the real binary, `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`, under `mise exec --`.
- `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes its `panini` and `panini-data` dev-deps. Repoint them at this worktree before generating or auditing, and back at `/workspace/crates` after. Check with `grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml` every time.
- Never wait on a process with `pgrep -f` (it matches its own shell); use `pgrep -x cargo-mutants` or `kill -0 <pid>`.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **An anusvāra that is not num reaching 8.4.2.** Any `M` between a ṇatva trigger and an `n` now carries the scan; a prior root whose 8.3.24 anusvāra sat in that run would silently gain a `ṇ`. → Task 3 `natva_crosses_num_only_on_kzamp` (corpus-wide: every 8.4.2 step that crosses an `M` is `10.0112`'s loṭ parasmaipada uttama eka); Task 4's prior-trace diff (all 15644 prior live branches byte-identical).
2. **A homograph surface reporting the wrong number of roots.** `lanj`, `vanw` and `jas` have two rows each; `10.0249 div` shares 10f's `10.0230 div`'s forms but alone derives ṇic parasmaipada; `olanq` and `ulanq` meet only in laṅ. → Task 3 `curadi_analyses_its_optional_nic_forms` (one analysis per row, roots compared as a sorted list) and the pinned 655-surface pada-ambiguous set.
3. **No ṇic in ātmanepada, and the dental `n` across num.** *cintate*, *śardhate*, *kṣampāni*, *kṣampayāni* must be Invalid; every ṇic-less ātmanepada branch must block. → Task 3's Invalid list and `the_optional_nic_ids_are_credited_only_on_their_rows`.
4. **A future bulk slice putting an āsvadīya or ādhṛṣīya row in `OPTIONAL_NIC` under 2564 or 2570,** where vidyut's gaṇasūtra precedence makes the marker reading wrong. → Task 3 `optional_nic_matches_upadesha_markers`' `10.0279..=10.0388` exclusion.
5. **The verdict-aware uniqueness filter hiding a real ambiguity.** Two siblings agreeing on gaṇa, stored form, artha *and* verdict must still be rejected; only a verdict difference separates `10.0174 SraRu~` from `10.0063 SraRa~`. → Task 3 `dhatupatha_numbers_resolve_upstream` (the filter keeps every other clause, and the √śraṇ pin asserts both verdicts and both tables).

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-prakriya/src/tinanta/sound.rs` | Task 2: `'M'` in `is_natva_intervener`, its doc, the renamed intervener test |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | Task 2: `natva_fires_across_nums_anusvara_under_8_4_2` |
| `crates/panini-data/src/lib.rs` | Task 3: 59 rows, `OPTIONAL_NIC` (69), the curādi row list, `Dhatu::pada`'s census, the marker helper's doc, the marker test (69, range exclusion), the uniqueness filter and √śraṇ pin, the row count. Task 4: one count |
| `crates/panini/tests/paradigm/main.rs` | Task 3: census, keys, 10g paragraph, pada-ambiguous comment and set, `check()` witnesses. Task 4: audit-chain prose |
| `crates/panini/tests/paradigm/data/curadi.rs` | Task 3: 472 golden rows, 2832 alternates |
| `crates/panini/tests/trace/curadi.rs`, `crates/panini/tests/trace/juhotyadi.rs` | Task 3: four moved rosters, two new tests |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `guna.rs` (one comment), the 10f spec | Task 4 |
| `AGENTS.md`, maybe `mise.toml` | Task 5 |

---

## Task 1: The worktree and baseline

**Files:** none. **Interfaces:** none.

- [ ] **Step 1: Create the worktree**

```bash
cd /workspace
git status --short                   # empty
git log --oneline -3                 # the plan commit, cdf78e0 (spec), 6580978
git worktree add .worktrees/curadi-10g -b curadi-10g main
cd .worktrees/curadi-10g
git branch --show-current            # curadi-10g
```

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: everything passes at 13716 cells. `panini-prakriya` reports 418 tests, the `trace` binary 208, `paradigm` 25 and `panini-data` 27.

---

## Task 2: The engine — 8.4.2 reads num as the anusvāra

No curated root before this slice has an `M` between a ṇatva trigger and its target, so every golden and every prior trace is unchanged, and the suite is green at the end of the task (Task 4's prior-trace diff is the corpus-wide proof).

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/sound.rs` (the doc above `is_natva_intervener`; its `matches!`; `natva_intervener_is_at_ku_pu_and_nothing_else`)
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (`mod tests`, before `fn natva_declines_word_finally_per_8_4_37`)

**Interfaces:**
- Produces: `is_natva_intervener('M') == true`. No new symbol.
- Consumes: the tripadi tests' `natva_prakriya(anga: &str, vikarana: &str, ending: &str) -> Prakriya` and `rules()`.

- [ ] **Step 1: The failing tests**

Create `/tmp/vidyut-full/slice10g/engine_10g.py` if it is missing (sha256 `1edf2378459f15c118afd6f41c274df98bdc1947e29d4abbae19a534b771ded3`). It holds both the code edit and the two test edits; `--tests-only` applies the tests alone:

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10g's engine edit — 8.4.2's intervener set gains the
anusvāra — with its unit tests. Every `old` must occur exactly once in its
file; nothing is written if one fails. Run from the worktree root.
`--tests-only` applies only the two test edits (TDD: see them fail first)."""
import sys
TESTS_ONLY = '--tests-only' in sys.argv
SOUND = 'crates/panini-prakriya/src/tinanta/sound.rs'
TRIPADI = 'crates/panini-prakriya/src/tinanta/tripadi.rs'
CODE = {SOUND: [
("""/// 8.4.2's intervention set: aṭ (the vowels plus `h y v r`), ku (`k K g G N`)
/// and pu (`p P b B m`).
///
/// The sūtra also names **āṅ** and **num**, which are morphemes rather than
/// varṇa classes. Ṇatva runs in the tripādī over assembled text, where
/// morpheme identity is gone — and neither is a loss: āṅ is the upasarga `ā`,
/// already an aṭ vowel, and num's nasal cannot occur in the intervening
/// position for any form in the covered grammar (no num-infixing root is in
/// scope, and upasargas are out of scope entirely). Revisit when either
/// enters scope.
///""",
"""/// 8.4.2's intervention set: aṭ (the vowels plus `h y v r`), ku (`k K g G N`),
/// pu (`p P b B m`), and the anusvāra `M`, standing in for num.
///
/// The sūtra also names **āṅ** and **num**, which are morphemes rather than
/// varṇa classes. Ṇatva runs in the tripādī over assembled text, where
/// morpheme identity is gone. āṅ is no loss: it is the upasarga `ā`, already
/// an aṭ vowel (and upasargas are out of scope entirely). num is read from
/// its sound: a root's num is stored as `n` (7.1.58 is the stored-`code`
/// simplification), 8.3.24 has made it the anusvāra `M` before a jhal by the
/// time ṇatva scans, and 8.4.58 turns it into a pu-class `m` only after. In
/// the covered grammar a root-internal `M` comes from num alone, so `M` is
/// num's textual reading here — the approximation slice 10g's √kṣamp
/// (`10.0112 kzanp`, *kṣampāṇi*) needs.
///"""),
("""            'h' | 'y' | 'v' | 'r' | 'k' | 'K' | 'g' | 'G' | 'N' | 'p' | 'P' | 'b' | 'B' | 'm'
        )""",
"""            'h' | 'y' | 'v' | 'r' | 'k' | 'K' | 'g' | 'G' | 'N' | 'p' | 'P' | 'b' | 'B' | 'm' | 'M'
        )"""),
]}
TESTS = {SOUND: [
("""    fn natva_intervener_is_at_ku_pu_and_nothing_else() {""",
 """    fn natva_intervener_is_at_ku_pu_num_and_nothing_else() {"""),
("""        for c in ['p', 'P', 'b', 'B', 'm'] {
            assert!(is_natva_intervener(c), "pu member {c} should intervene");
        }""",
"""        for c in ['p', 'P', 'b', 'B', 'm'] {
            assert!(is_natva_intervener(c), "pu member {c} should intervene");
        }
        // num, read as the anusvāra 8.3.24 has made of it by the time ṇatva
        // scans (√kṣamp's kzaMpAni → kzaMpARi).
        assert!(is_natva_intervener('M'), "num's anusvāra should intervene");"""),
], TRIPADI: [
("""    #[test]
    fn natva_declines_word_finally_per_8_4_37() {""",
"""    #[test]
    fn natva_fires_across_nums_anusvara_under_8_4_2() {
        // kzaMp + Ani (√kṣamp's loṭ uttama eka, 10.0112): z, the aw vowel a,
        // num's anusvāra M (8.3.24 has run), the pu p, the aw vowel A, then n.
        // 8.4.2 names num; read as M, it carries the scan to the z.
        let mut p = natva_prakriya("kzaMp", "", "Ani");
        let r841 = rules().find(|r| r.id == "8.4.1").unwrap();
        assert!(!(r841.apply)(&mut p), "8.4.1 must not fire non-adjacently");
        let r842 = rules().find(|r| r.id == "8.4.2").unwrap();
        assert!((r842.apply)(&mut p));
        assert_eq!(p.text(), "kzaMpARi");
        // The dental n of a root without num stays: kzap + Ani has the same
        // run with no M, and 8.4.2 fires there too — the pu p and the aw
        // vowels already intervene. What M adds is exactly one link.
        let mut p = natva_prakriya("kzap", "", "Ani");
        assert!((r842.apply)(&mut p));
        assert_eq!(p.text(), "kzapARi");
    }

    #[test]
    fn natva_declines_word_finally_per_8_4_37() {"""),
]}
FILES = dict(TESTS) if TESTS_ONLY else {f: CODE.get(f, []) + TESTS.get(f, []) for f in set(CODE) | set(TESTS)}
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
python3 /tmp/vidyut-full/slice10g/engine_10g.py --tests-only      # applied 3 edits
```

The tests: `natva_intervener_is_at_ku_pu_num_and_nothing_else` (renamed from `…_at_ku_pu_and_nothing_else`) asserts `is_natva_intervener('M')`; `natva_fires_across_nums_anusvara_under_8_4_2` runs 8.4.1 and 8.4.2 on `kzaMp` + `` + `Ani` (8.4.1 declines, 8.4.2 gives `kzaMpARi`) and, as contrast, on `kzap` + `Ani` (`kzapARi`, no `M` needed).

- [ ] **Step 2: Run them to see them fail**

Run: `mise exec -- cargo test -q -p panini-prakriya natva 2>&1 | grep -E "FAILED|test result"`
Expected: `tinanta::sound::tests::natva_intervener_is_at_ku_pu_num_and_nothing_else` and `tinanta::tripadi::tests::natva_fires_across_nums_anusvara_under_8_4_2` FAIL; `6 passed; 2 failed`. This red step is also the mutation evidence for `'M'`: cargo-mutants generates no mutant inside a `matches!` pattern (Task 5).

- [ ] **Step 3: The change**

```bash
git checkout -- crates
python3 /tmp/vidyut-full/slice10g/engine_10g.py      # applied 5 edits
mise run fmt
```

It rewrites the doc above `is_natva_intervener` (the set now names the anusvāra `M` standing in for num; the "num's nasal cannot occur … Revisit" deferral, which this slice falsifies, is replaced by the textual reading; āṅ's half stays) and adds `| 'M'` to its `matches!`.

- [ ] **Step 4: Run the tests and the suite**

```bash
mise exec -- cargo test -q -p panini-prakriya natva 2>&1 | grep -E "FAILED|test result"      # 8 passed
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS, with `panini-prakriya` 419, `trace` 208, `paradigm` 25, `panini-data` 27.

- [ ] **Step 5: Commit**

```bash
mise run lint
git branch --show-current      # curadi-10g
git add -A
git commit -m "feat(tripadi): 8.4.2 reads num as the anusvāra

is_natva_intervener gains 'M': a root's stored num reaches ṇatva as the
anusvāra 8.3.24 made of it, and 8.4.58 turns it into m only afterwards.
No curated root yet has an M in a ṇatva run; √kṣamp (10g) will."
```

---

## Task 3: The rows, their goldens, and every assertion they move

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`, `crates/panini/tests/paradigm/data/curadi.rs`
- Modify: `crates/panini/tests/trace/curadi.rs`, `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes: Task 2's `'M'`; 10f's `OPTIONAL_NIC`, `optional_nic`, `Dhatu::padas`; the trace helpers `at` and `credited`; `panini_prakriya::{derive, RuleStep}`; `panini::Panini::check`.
- Produces: the totals Task 4 documents (311 / 17964 / 22724; 4760 alternates; 655 pada-ambiguous surfaces; 188 both-pada roots).

Each script below asserts that every `old` occurs exactly once and writes nothing otherwise. If one prints `not applied:`, nothing was written: stop and report, rather than editing around it.

- [ ] **Step 1: The paradigm assertions (failing)**

Create `/tmp/vidyut-full/slice10g/paradigm_10g.py` if it is missing (sha256 `f3b86b89fa1811d2cd4899efaa2abf76ad24d871bd5fc3478b1712c131576520`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10g's edits to crates/panini/tests/paradigm/main.rs (the
census, ALTERNATES keys, docs). Every `old` must occur exactly once; nothing
is written if one fails. Run from the worktree root."""
import sys
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
E = [
("/// `ALTERNATES` is otherwise 1928 bare strings,", "/// `ALTERNATES` is otherwise 4760 bare strings,"),
("/// 13716 cells total (1524 root×lakāra blocks × 9), of which 12364 hold exactly one form, 904 hold two, 389 hold three (",
 "/// 17964 cells total (1996 root×lakāra blocks × 9), of which 14488 hold exactly one form, 2792 hold two, 389 hold three ("),
("/// optional-ṇic rows add 114 two-form and 46 three-form cells), twenty-three hold four (",
 "/// optional-ṇic rows add 114 two-form and 46 three-form cells, and slice 10g's\n"
 "/// fifty-nine add 1888 two-form cells, a ṇic and a ṇic-less reading each), 141 hold four ("),
("/// prathama eka, their ṇic and ṇic-less readings × 8.4.56), and\n",
 "/// prathama eka, their ṇic and ṇic-less readings × 8.4.56; and — new in slice\n"
 "/// 10g — the fifty-nine optional-ṇic rows' laṅ and vidhiliṅ parasmaipada\n"
 "/// prathama eka, the same way), and\n"),
("/// twenty-three\n/// hold six (", "/// 141\n/// hold six ("),
("/// prathama and madhyama eka (two readings × the tātaṅ triple)), one — new in\n",
 "/// prathama and madhyama eka (two readings × the tātaṅ triple); and — new in\n"
 "/// slice 10g — the fifty-nine optional-ṇic rows' loṭ parasmaipada prathama\n"
 "/// and madhyama eka, the same way), one — new in\n"),
("/// itself has 1928 rows, keyed 354 `8.4.56`, 346 `7.1.35`, 346 `7.1.35+8.4.56`,",
 "/// itself has 4760 rows, keyed 468 `8.4.56`, 460 `7.1.35`, 460 `7.1.35+8.4.56`,"),
("/// `7.3.86+6.4.107`, 19 `7.3.86+8.4.56` (twelve of them",
 "/// `7.3.86+6.4.107`, 23 `7.3.86+8.4.56` (twelve of them"),
("/// `7.1.35+6.4.116+8.4.56`, 9 `6.4.43` and 1 `6.4.43+8.4.56` (slice 3f3's √jan), 2 `7.3.86+7.1.35` and 2\n"
 "/// `7.3.86+7.1.35+8.4.56` (slice 10a's √cur, its sanādi 7.3.86 ahead of 7.1.35), and slice 10f's twenty-one\n"
 "/// keys on its five Kaumudī vikalpa ids: 36 `2573.1`, 72 `2573.2`, 72 `2573.3`, 8 apiece\n"
 "/// `2564+8.4.56` / `2564+7.1.35` / `2564+7.1.35+8.4.56`, 6 apiece `2573.3+8.4.56` /\n"
 "/// `2573.3+7.1.35` / `2573.3+7.1.35+8.4.56`, and 2 apiece `2570+8.4.56`,\n"
 "/// `2570+7.1.35`, `2570+7.1.35+8.4.56`, `2570+7.3.86+8.4.56`, `2570+7.1.35+7.3.86`,\n"
 "/// `2570+7.1.35+7.3.86+8.4.56` (√div's ṇic-less branch, whose root guṇa the\n"
 "/// MANDATORY 7.3.86 credits), `2573.1+8.4.56`, `2573.1+7.1.35`,\n"
 "/// `2573.1+7.1.35+8.4.56`, `2573.2+8.4.56`, `2573.2+7.1.35` and\n"
 "/// `2573.2+7.1.35+8.4.56`; its other eighteen rows fold into `8.4.56`,\n"
 "/// `7.1.35` and `7.1.35+8.4.56`, six apiece — √kṛ (slice 8b) adds six more",
 "/// `7.1.35+6.4.116+8.4.56`, 9 `6.4.43` and 1 `6.4.43+8.4.56` (slice 3f3's √jan), 6 `7.3.86+7.1.35` and 6\n"
 "/// `7.3.86+7.1.35+8.4.56` (slice 10a's √cur, its sanādi 7.3.86 ahead of 7.1.35, and slice 10g's √śṛdh and\n"
 "/// √div on their ṇic branch), and slice 10f's twenty-one\n"
 "/// keys on its five Kaumudī vikalpa ids, at their counts as of slice 10g: 36 `2573.1`, 72 `2573.2`, 72 `2573.3`, 114 apiece\n"
 "/// `2564+8.4.56` / `2564+7.1.35` / `2564+7.1.35+8.4.56` (8 from 10f's four idit rows, 106 from 10g's\n"
 "/// fifty-three), 6 apiece `2573.3+8.4.56` /\n"
 "/// `2573.3+7.1.35` / `2573.3+7.1.35+8.4.56`, 10 apiece `2570+8.4.56`,\n"
 "/// `2570+7.1.35` and `2570+7.1.35+8.4.56` (10f's √vañc, 10g's √śraṇ, both √jas and √añc), 6 apiece\n"
 "/// `2570+7.3.86+8.4.56`, `2570+7.1.35+7.3.86` and\n"
 "/// `2570+7.1.35+7.3.86+8.4.56` (10f's √div, 10g's √div and √śṛdh: the ṇic-less branch, whose root guṇa the\n"
 "/// MANDATORY 7.3.86 credits), and 2 apiece `2573.1+8.4.56`, `2573.1+7.1.35`,\n"
 "/// `2573.1+7.1.35+8.4.56`, `2573.2+8.4.56`, `2573.2+7.1.35` and\n"
 "/// `2573.2+7.1.35+8.4.56`; its other eighteen rows fold into `8.4.56`,\n"
 "/// `7.1.35` and `7.1.35+8.4.56`, six apiece. Slice 10g opens three keys,\n"
 "/// 1908 `2564`, 144 `2570` and 72 `2570+7.3.86` — a ṇic-less form beside\n"
 "/// the ṇic one in a cell that otherwise does not fork — and folds 114 rows\n"
 "/// apiece into `8.4.56`, `7.1.35` and `7.1.35+8.4.56` and four apiece into\n"
 "/// `7.3.86+8.4.56`, `7.3.86+7.1.35` and `7.3.86+7.1.35+8.4.56` — √kṛ (slice 8b) adds six more"),
("/// first live branch, the ṇic-less one. 720 new cells, 264 new rows. The\n/// gaṇa is OPEN at 149 of its 509 rows.\n/// This test is what keeps the numbers true day to day.",
 "/// first live branch, the ṇic-less one. 720 new cells, 264 new rows. The\n/// gaṇa is OPEN at 149 of its 509 rows.\n///\n"
 "/// Slice 10g curates fifty-nine more optional-ṇic rows: every idit (Kaumudī\n"
 "/// 2564) and udit (2570) curādi row outside the āsvadīya and ādhṛṣīya but\n"
 "/// `10.0124 ciY`, all `Nic`. The ṇic branch is live in both padas, so it is\n"
 "/// the pinned form; every parasmaipada cell adds the ṇic-less reading\n"
 "/// beside it. 4248 new cells, 2832 new rows. The gaṇa is OPEN at 208 of its\n"
 "/// 509 rows.\n"
 "/// This test is what keeps the numbers true day to day."),
('    assert_eq!(total_cells, 13716, "1524 root×lakāra blocks × 9 cells each");',
 '    assert_eq!(total_cells, 17964, "1996 root×lakāra blocks × 9 cells each");'),
('    assert_eq!(ones, 12364, "one-form cells");\n    assert_eq!(twos, 904, "two-form cells");',
 '    assert_eq!(ones, 14488, "one-form cells");\n    assert_eq!(twos, 2792, "two-form cells");'),
("        fours, 23,", "        fours, 141,"),
("         and — new in slice 10f — mUtra's and katra's laṅ and vidhiliṅ parasmaipada prathama \\\n         eka, 2573.3 alongside 8.4.56\"",
 "         and — new in slice 10f — mUtra's and katra's laṅ and vidhiliṅ parasmaipada prathama \\\n         eka, 2573.3 alongside 8.4.56; and — new in slice 10g — the fifty-nine optional-ṇic \\\n         rows' laṅ and vidhiliṅ parasmaipada prathama eka, 2564/2570 alongside 8.4.56\""),
("        sixes, 23,", "        sixes, 141,"),
("         mUtra's and katra's loṭ parasmaipada prathama and madhyama eka (2573.3 beside \\\n         7.1.35/8.4.56)\"",
 "         mUtra's and katra's loṭ parasmaipada prathama and madhyama eka (2573.3 beside \\\n         7.1.35/8.4.56); and — new in slice 10g — the fifty-nine optional-ṇic rows' loṭ \\\n         parasmaipada prathama and madhyama eka (2564/2570 beside 7.1.35/8.4.56)\""),
('    assert_eq!(ALTERNATES.len(), 1928, "ALTERNATES row count");',
 '    assert_eq!(ALTERNATES.len(), 4760, "ALTERNATES row count");'),
('    assert_eq!(key_count("8.4.56"), 354, "8.4.56-only alternates");\n    assert_eq!(key_count("7.1.35"), 346, "7.1.35-only alternates");\n    assert_eq!(key_count("7.1.35+8.4.56"), 346, "7.1.35+8.4.56 alternates");',
 '    assert_eq!(key_count("8.4.56"), 468, "8.4.56-only alternates");\n    assert_eq!(key_count("7.1.35"), 460, "7.1.35-only alternates");\n    assert_eq!(key_count("7.1.35+8.4.56"), 460, "7.1.35+8.4.56 alternates");'),
('    assert_eq!(key_count("7.3.86+8.4.56"), 19, "7.3.86+8.4.56 alternates");',
 '    assert_eq!(key_count("7.3.86+8.4.56"), 23, "7.3.86+8.4.56 alternates");'),
('    assert_eq!(key_count("7.3.86+7.1.35"), 2, "7.3.86+7.1.35 alternates");\n    assert_eq!(\n        key_count("7.3.86+7.1.35+8.4.56"),\n        2,',
 '    assert_eq!(key_count("7.3.86+7.1.35"), 6, "7.3.86+7.1.35 alternates");\n    assert_eq!(\n        key_count("7.3.86+7.1.35+8.4.56"),\n        6,'),
('''    // Slice 10f's five Kaumudī vikalpa ids, alone and stacked.
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
        ("2570+7.1.35+7.3.86+8.4.56", 2),''',
'''    // Slice 10f's five Kaumudī vikalpa ids, alone and stacked, with slice
    // 10g's fifty-nine rows: 2564 alone, 2570 alone and 2570+7.3.86 are 10g's.
    for (key, n) in [
        ("2564", 1908),
        ("2570", 144),
        ("2570+7.3.86", 72),
        ("2573.1", 36),
        ("2573.2", 72),
        ("2573.3", 72),
        ("2564+8.4.56", 114),
        ("2564+7.1.35", 114),
        ("2564+7.1.35+8.4.56", 114),
        ("2573.3+8.4.56", 6),
        ("2573.3+7.1.35", 6),
        ("2573.3+7.1.35+8.4.56", 6),
        ("2570+8.4.56", 10),
        ("2570+7.1.35", 10),
        ("2570+7.1.35+8.4.56", 10),
        ("2570+7.3.86+8.4.56", 6),
        ("2570+7.1.35+7.3.86", 6),
        ("2570+7.1.35+7.3.86+8.4.56", 6),'''),
("    // twelve more, taking the set from 420 to 432. The ākusmīya rows and\n    // `garva` derive each pada on a different branch (ṇic-less parasmaipada,\n    // ṇic ātmanepada), and those surfaces never meet: they contribute nothing.",
 "    // twelve more, taking the set from 420 to 432. The ākusmīya rows and\n    // `garva` derive each pada on a different branch (ṇic-less parasmaipada,\n    // ṇic ātmanepada), and those surfaces never meet: they contribute nothing.\n"
 "    // Slice 10g's fifty-nine `Nic` rows contribute the same four each from\n"
 "    // their ṇic branch — 236 — less the homographs': `lanj`, `vanw` and `jas`\n"
 "    // each have two rows sharing all four (−12), and `10.0014 olanq` and\n"
 "    // `10.0105 ulanq` share their laṅ one, `OlaRqayata` (−1). `10.0249 div`'s\n"
 "    // four are new: 10f's `10.0230 div` has only their ātmanepada half. 223\n"
 "    // more, taking the set from 432 to 655."),
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
python3 /tmp/vidyut-full/slice10g/paradigm_10g.py      # applied 22 edits
```

What it changes, so a reviewer can check it against the spec and the goldens:
- the census: 17964 cells (1996 blocks); buckets 14488 / 2792 / 389 / 141 / 10 / 141 / 1 / — / 2 (each new row adds 36 one-form ātmanepada cells, 32 two-form parasmaipada cells, two four-form (laṅ and vidhiliṅ prathama eka) and two six-form (loṭ prathama and madhyama eka));
- `ALTERNATES`: 4760 rows; `8.4.56` / `7.1.35` / `7.1.35+8.4.56` 468 / 460 / 460 (114 each from the fifty-seven ṇic branches without a sanādi guṇa); `7.3.86+8.4.56` 23 and `7.3.86+7.1.35` / `7.3.86+7.1.35+8.4.56` 6 (√śṛdh's and √div's ṇic branch, four each); 10f's Kaumudī keys raised (`2564+…` 114, `2570+…` 10, `2570+7.3.86`-stacked 6) and three new keys, `2564` 1908 (53 × 36), `2570` 144 (4 × 36), `2570+7.3.86` 72 (2 × 36);
- the 10g paragraph and the pada-ambiguous comment (432 → 655: 59 × 4 = 236, less 12 for the `lanj` / `vanw` / `jas` pairs and 1 for `OlaRqayata`). The set itself is pinned in Step 6, from the measured failure.

- [ ] **Step 2: The trace assertions and tests (failing)**

Create `/tmp/vidyut-full/slice10g/trace_10g.py` if it is missing (sha256 `a50480bd46dc4989fd68b08e8b6bd471767e9a9cd7c1e76206379978a7e07587`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10g's edits to crates/panini/tests/trace/{curadi,juhotyadi}.rs.
Every `old` must occur exactly once; nothing is written if one fails."""
import sys
CURADI = 'crates/panini/tests/trace/curadi.rs'
JUHO = 'crates/panini/tests/trace/juhotyadi.rs'
IDIT = ["10.0002", "10.0003", "10.0004", "10.0005", "10.0007", "10.0009", "10.0011", "10.0013",
        "10.0014", "10.0043", "10.0045", "10.0047", "10.0048", "10.0049", "10.0060", "10.0062",
        "10.0066", "10.0067", "10.0068", "10.0069", "10.0070", "10.0071", "10.0072", "10.0073",
        "10.0074", "10.0075", "10.0076", "10.0077", "10.0105", "10.0106", "10.0107", "10.0111",
        "10.0112", "10.0113", "10.0114", "10.0130", "10.0135", "10.0147", "10.0153", "10.0157",
        "10.0158", "10.0159", "10.0160", "10.0164", "10.0166", "10.0171", "10.0182", "10.0185",
        "10.0241", "10.0254", "10.0267", "10.0464", "10.0465"]
UDIT = ["10.0174", "10.0184", "10.0243", "10.0249", "10.0260", "10.0266"]
assert len(IDIT) == 53 and len(UDIT) == 6
def q(xs):
    return ", ".join(f'"{x}"' for x in xs)
NEW_TESTS = r'''

/// The places 8.4.2 retroflexes an `n` across num's anusvāra `M`: walk back
/// from the changed `n` in the step's `before` text to its trigger, and
/// report whether an `M` lay between.
fn natva_crossed_num(step: &panini_prakriya::RuleStep) -> bool {
    let before: Vec<char> = step.before.chars().collect();
    let after: Vec<char> = step.after.chars().collect();
    let i = before
        .iter()
        .zip(&after)
        .position(|(b, a)| b != a)
        .expect("8.4.2 changed nothing");
    assert_eq!((before[i], after[i]), ('n', 'R'), "{step:?}");
    let trigger = before[..i]
        .iter()
        .rposition(|c| matches!(c, 'r' | 'z' | 'f' | 'F'))
        .expect("8.4.2 without a trigger");
    before[trigger..i].contains(&'M')
}

#[test]
fn natva_crosses_num_only_on_kzamp() {
    // 8.4.2 reads num as the anusvāra 8.3.24 has made of it (slice 10g).
    // Goldens ignore traces, so this is what holds that reading to √kṣamp's
    // cells: across the corpus, every live branch whose 8.4.2 crosses an `M`
    // is `10.0112 kzanp`'s loṭ parasmaipada uttama eka, one ṇic and one
    // ṇic-less, and both are found.
    let mut crossed = Vec::new();
    for d in dhatus() {
        for lakara in [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin] {
            for &pada in d.padas() {
                for purusha in [Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama] {
                    for vacana in [Vacana::Eka, Vacana::Dvi, Vacana::Bahu] {
                        for p in derive(d, lakara, pada, purusha, vacana) {
                            if p.blocked {
                                continue;
                            }
                            for s in p.log.iter().filter(|s| s.sutra == "8.4.2") {
                                if natva_crossed_num(s) {
                                    crossed.push((
                                        d.dhatupatha,
                                        lakara,
                                        pada,
                                        purusha,
                                        vacana,
                                        p.text(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut forms: Vec<String> = crossed.iter().map(|c| c.5.clone()).collect();
    forms.sort_unstable();
    assert_eq!(forms, ["kzampARi", "kzampayARi"], "{crossed:?}");
    for (number, lakara, pada, purusha, vacana, _) in &crossed {
        assert_eq!(*number, "10.0112");
        assert_eq!(
            (*lakara, *pada, *purusha, *vacana),
            (
                Lakara::Lot,
                Pada::Parasmaipada,
                Purusha::Uttama,
                Vacana::Eka
            )
        );
    }
}

#[test]
#[allow(non_snake_case)]
fn kzampARi_traces_8_4_2_across_num_on_both_branches() {
    // kzanp P loT U.E. On both branches 8.3.24 makes the stored num an
    // anusvāra, 8.4.2 retroflexes the ending's `n` across it, and only then
    // does 8.4.58 make it `m`.
    let d = dhatus().iter().find(|d| d.dhatupatha == "10.0112").unwrap();
    let mut got: Vec<String> = Vec::new();
    for p in derive(
        d,
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    ) {
        if p.blocked {
            continue;
        }
        let t: Vec<String> = p.log.iter().map(|s| s.sutra.clone()).collect();
        assert!(at(&t, "8.3.24") < at(&t, "8.4.2"), "{t:?}");
        assert!(at(&t, "8.4.2") < at(&t, "8.4.58"), "{t:?}");
        assert!(!t.contains(&"8.4.1".to_string()), "{t:?}");
        got.push(p.text());
    }
    got.sort_unstable();
    assert_eq!(got, ["kzampARi", "kzampayARi"]);
}'''
EDITS = {CURADI: [
("    // never reaches them: its credits stay on the 96 `Nic` rows, read from\n",
 "    // never reaches them: its credits stay on the 155 `Nic` rows, read from\n"),
('    assert_eq!(nic.len(), 96, "curated 1.3.74 rows");',
 '    assert_eq!(nic.len(), 155, "curated 1.3.74 rows");'),
("    // `pata`. Goldens ignore traces, so this is also what holds all five\n    // inert on the 242 prior roots.\n",
 "    // `pata`. Goldens ignore traces, so this is also what holds all five\n    // inert on every root outside `OPTIONAL_NIC`. The 2564 and 2570 rows are\n    // the 10f and 10g specs' row tables, listed literally.\n"),
('        ("2564", &["10.0193", "10.0194", "10.0198", "10.0199"][..]),\n        ("2570", &["10.0227", "10.0230"][..]),',
 '        (\n            "2564",\n            &["10.0193", "10.0194", "10.0198", "10.0199", ' + q(IDIT) + '][..],\n        ),\n'
 '        (\n            "2570",\n            &["10.0227", "10.0230", ' + q(UDIT) + '][..],\n        ),'),
], JUHO: [
("    // 10c's √gandh and seven adanta roots, on every live branch, and since\n    // slice 10f five optional-ṇic ākusmīya roots (`danS`, `dans`, `tantr`,\n    // `mantr`, `vanc`) on both their branches.\n    const CURADI: [&str; 13] = [\n        \"10.0204\", \"10.0433\", \"10.0460\", \"10.0467\", \"10.0471\", \"10.0472\", \"10.0473\", \"10.0474\",\n        \"10.0193\", \"10.0194\", \"10.0198\", \"10.0199\", \"10.0227\",\n    ];",
 "    // 10c's √gandh and seven adanta roots, on every live branch, and since\n    // slice 10f five optional-ṇic ākusmīya roots (`danS`, `dans`, `tantr`,\n    // `mantr`, `vanc`) on both their branches, and since slice 10g the\n"
 "    // fifty-three idit rows (num stored) and `10.0266 anc`, on every live\n    // branch of both.\n"
 "    const CURADI: [&str; 67] = [\n        \"10.0204\", \"10.0433\", \"10.0460\", \"10.0467\", \"10.0471\", \"10.0472\", \"10.0473\", \"10.0474\",\n        \"10.0193\", \"10.0194\", \"10.0198\", \"10.0199\", \"10.0227\", " + q(IDIT) + ", \"10.0266\",\n    ];"),
("    // and the five optional-ṇic roots' 78 each (42 ṇic-less parasmaipada, 36\n    // ṇic ātmanepada).\n    assert_eq!(curadi.len(), 36 + 7 * 78 + 5 * 78);",
 "    // the five optional-ṇic roots' 78 each (42 ṇic-less parasmaipada, 36\n    // ṇic ātmanepada), and slice 10g's fifty-four 120 each (every live\n    // branch: 84 parasmaipada, ṇic and ṇic-less, and 36 ṇic ātmanepada).\n    assert_eq!(curadi.len(), 36 + 7 * 78 + 5 * 78 + 54 * 120);"),
("    // three ch-initial adanta curādi roots (`Cidra`, `Ceda`, `Cada`), whose laṅ\n    // aṭ takes the same forward-arm tuk (acCidrayat).\n",
 "    // three ch-initial adanta curādi roots (`Cidra`, `Ceda`, `Cada`), whose laṅ\n    // aṭ takes the same forward-arm tuk (acCidrayat), and slice 10g the three\n    // ch-initial optional-ṇic roots (`Cand`, `Canj`, `Canp`), on both branches\n    // (acCandayat, acCandat).\n"),
('            ["07.0003", "07.0008", "10.0469", "10.0480", "10.0481"].contains(number),',
 '            [\n                "07.0003", "07.0008", "10.0469", "10.0480", "10.0481", "10.0062", "10.0114",\n                "10.0171"\n            ]\n            .contains(number),'),
("    // Each ch-initial adanta root: laṅ's 18 cells plus its one 8.4.56 fork.\n    assert_eq!(adanta, 3 * 19);",
 "    // Each ch-initial adanta root: laṅ's 18 cells plus its one 8.4.56 fork.\n"
 "    // Each ch-initial optional-ṇic root: laṅ's 9 ātmanepada cells, and its\n"
 "    // 9 parasmaipada ones twice (ṇic and ṇic-less) plus their two 8.4.56 forks.\n"
 "    assert_eq!(adanta, 3 * 19 + 3 * 29);"),
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
python3 /tmp/vidyut-full/slice10g/trace_10g.py      # applied 9 edits and two tests
```

It moves four rosters, each from the spec's row tables:
- `a_kusmad_is_credited_on_exactly_the_akusmiya_cells`: 1.3.74's `Nic` rows 96 → 155;
- `the_optional_nic_ids_are_credited_only_on_their_rows`: 2564's rows gain the 53, 2570's the 6;
- `nas_capadantasya_is_credited_only_on_rudhadi_dhan_jan_and_curadi_roots`: 8.3.24 gains the 53 idit rows (num stored) and `10.0266 anc`, 120 live branches each (every live branch: 84 parasmaipada and 36 ātmanepada), `36 + 7 * 78 + 5 * 78 + 54 * 120`;
- `shcutva_off_jan_is_credited_exactly_as_before_3f3`: 8.4.40 gains `Cand`, `Canj`, `Canp`, 29 each (laṅ: 9 ātmanepada, 9 parasmaipada twice, two 8.4.56 forks).

And adds two: `natva_crosses_num_only_on_kzamp` (corpus-wide; every 8.4.2 step whose run from trigger to `n` holds an `M` is `10.0112`'s loṭ parasmaipada uttama eka, both branches) and `kzampARi_traces_8_4_2_across_num_on_both_branches` (8.3.24 < 8.4.2 < 8.4.58, no 8.4.1, forms {`kzampARi`, `kzampayARi`}).

- [ ] **Step 3: The data-layer assertions and docs (failing)**

Create `/tmp/vidyut-full/slice10g/data_10g.py` if it is missing (sha256 `db55c4d35ac7e33ca4894e1cf5a0674b280b07d55d9019f6221c0f4f725988c3`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10g's edits to crates/panini-data/src/lib.rs (tests and
docs, not the rows or the table). Every `old` must occur exactly once;
nothing is written if one fails. Run from the worktree root."""
import sys
p = 'crates/panini-data/src/lib.rs'
s = open(p).read()
E = [
# Dhatu::pada doc census
("    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 252\n",
 "    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 311\n"),
("    /// exception, ninety-six curādi rows' are 1.3.74's, 43 ākusmīya rows'\n",
 "    /// exception, 155 curādi rows' are 1.3.74's, 43 ākusmīya rows'\n"),
("    /// `dhatupatha_numbers_resolve_upstream` holds `code` to upstream. For\n    /// the ten curādi rows whose ṇic is optional this is the ṇic branch's\n",
 "    /// `dhatupatha_numbers_resolve_upstream` holds `code` to upstream. For\n    /// the sixty-nine curādi rows whose ṇic is optional this is the ṇic branch's\n"),
("    /// The test covers the 252 roots curated here, not the dhātupāṭha's 2259.",
 "    /// The test covers the 311 roots curated here, not the dhātupāṭha's 2259."),
("        assert_eq!(dhatus().len(), 252);", "        assert_eq!(dhatus().len(), 311);"),
# the curādi row-list test
("    fn curadi_rows_are_the_one_hundred_forty_nine_curated_roots() {",
 "    fn curadi_rows_are_the_two_hundred_eight_curated_roots() {"),
("        // `mUtra`, `katra` and `pata`, each curated with its ṇic branch's\n        // pada. The gaṇa is OPEN at 149 of its 509 dhātupāṭha rows.",
 "        // `mUtra`, `katra` and `pata`, each curated with its ṇic branch's\n"
 "        // pada. Slice 10g adds fifty-nine more, every idit (Kaumudī 2564) and\n"
 "        // udit (2570) row outside the āsvadīya and ādhṛṣīya but `10.0124 ciY`,\n"
 "        // all ubhayapadī by 1.3.74 with ṇic. The gaṇa is OPEN at 208 of its\n"
 "        // 509 dhātupāṭha rows."),
# the marker helper's doc: the ranges it does not know
("    /// have a conjunct before their final `a` and take ṇic. Only the triggers\n    /// slice 10f curates; the ādhṛṣīya / āsvadīya gaṇasūtras (10.0498,\n    /// 10.0499) and 2565 / 2571 / 2572 are later slices'.",
 "    /// have a conjunct before their final `a` and take ṇic. Only the triggers\n"
 "    /// slices 10f and 10g curate; the ādhṛṣīya / āsvadīya gaṇasūtras (10.0498,\n"
 "    /// 10.0499) and 2565 / 2571 / 2572 are later slices'. vidyut decides\n"
 "    /// those two gaṇasūtras BEFORE idit or udit, so inside their rows\n"
 "    /// (`10.0279`–`10.0388`) this reading would be wrong;\n"
 "    /// `optional_nic_matches_upadesha_markers` keeps the table out of them."),
# the marker test: 69 entries and the range exclusion
("        assert_eq!(OPTIONAL_NIC.len(), 10);\n",
 "        assert_eq!(OPTIONAL_NIC.len(), 69);\n"
 "        // vidyut's `dhatu_karya.rs` checks the āsvadīya (10.0499, rows\n"
 "        // 279–337) and ādhṛṣīya (10.0498, rows 338–388) before idit or udit,\n"
 "        // and the reading above knows neither. No table row may fall in\n"
 "        // those ranges until a slice teaches it both gaṇasūtras.\n"
 "        for (n, _) in OPTIONAL_NIC {\n"
 "            assert!(\n"
 "                !(\"10.0279\"..=\"10.0388\").contains(n),\n"
 "                \"{n} is āsvadīya or ādhṛṣīya\"\n"
 "            );\n"
 "        }\n"),
# uniqueness: the optional-ṇic verdict tells siblings apart
("            // number's two-digit prefix, per `gana_matches_dhatupatha_prefix`)\n            // because upstream reuses (code, artha) pairs across gaṇas too.\n            let gana_prefix = &d.dhatupatha[..2];\n            let siblings = rows\n                .iter()\n                .filter(|(n, u, a)| {\n                    n.starts_with(gana_prefix) && stored_form(u) == stripped && *a == *artha\n                })\n                .count();",
 "            // number's two-digit prefix, per `gana_matches_dhatupatha_prefix`)\n"
 "            // because upstream reuses (code, artha) pairs across gaṇas too.\n"
 "            // Siblings that differ in their optional-ṇic verdict are distinct\n"
 "            // roots to this engine, which reads that verdict by number\n"
 "            // (`OPTIONAL_NIC`): `10.0174 SraRu~` (2570) beside `10.0063 SraRa~`\n"
 "            // (none) is the case, pinned below.\n"
 "            let gana_prefix = &d.dhatupatha[..2];\n"
 "            let verdict = optional_nic_from_upadesha(upadesha);\n"
 "            let siblings = rows\n"
 "                .iter()\n"
 "                .filter(|(n, u, a)| {\n"
 "                    n.starts_with(gana_prefix)\n"
 "                        && stored_form(u) == stripped\n"
 "                        && *a == *artha\n"
 "                        && optional_nic_from_upadesha(u) == verdict\n"
 "                })\n"
 "                .count();"),
("                \"{} is ambiguous: {siblings} rows in gaṇa {gana_prefix} share \\\n                 ({stripped}, {artha})\",\n                d.dhatupatha\n            );\n        }\n    }",
 "                \"{} is ambiguous: {siblings} rows in gaṇa {gana_prefix} share \\\n                 ({stripped}, {artha})\",\n                d.dhatupatha\n            );\n        }\n"
 "        // √śraṇ: `10.0174 SraRu~` and the uncurated `10.0063 SraRa~` share\n"
 "        // gaṇa, stored form and artha. Only the optional-ṇic verdict tells\n"
 "        // them apart, so `10.0063` stays a distinct row if it is curated.\n"
 "        let row = |n: &str| *rows.iter().find(|(m, _, _)| *m == n).unwrap();\n"
 "        let (_, sran_a, artha_a) = row(\"10.0063\");\n"
 "        let (_, sran_u, artha_u) = row(\"10.0174\");\n"
 "        assert_eq!(stored_form(sran_a), \"SraR\");\n"
 "        assert_eq!(stored_form(sran_u), \"SraR\");\n"
 "        assert_eq!(artha_a, artha_u);\n"
 "        assert_eq!(optional_nic_from_upadesha(sran_u), Some(\"2570\"));\n"
 "        assert_eq!(optional_nic_from_upadesha(sran_a), None);\n"
 "        assert_eq!(optional_nic(\"10.0174\"), Some(\"2570\"));\n"
 "        assert_eq!(optional_nic(\"10.0063\"), None);\n"
 "    }"),
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
python3 /tmp/vidyut-full/slice10g/data_10g.py      # applied 11 edits
```

It updates `Dhatu::pada`'s census (311; 155 `Nic`; sixty-nine optional-ṇic rows), the row count (311), the curādi list test's name (`…_two_hundred_eight_…`) and comment (208 of 509), the marker helper's doc (the ranges it cannot read), the marker test (69 entries; no entry in `10.0279..=10.0388`), and `dhatupatha_numbers_resolve_upstream` (siblings must also share `optional_nic_from_upadesha`'s verdict; the √śraṇ pin: both store `SraR`, share their artha, and differ in verdict and in `optional_nic`).

- [ ] **Step 4: The `check()` witnesses (failing)**

Create `/tmp/vidyut-full/slice10g/check_10g.rs` (sha256 `f2ce3adbff5f26ba62f7491c2f94e48b7db4612114a22bbe535c43f24dfe1bb5`) and `/tmp/vidyut-full/slice10g/insert_check_10g.py` (sha256 `01f948471c0a4f58cb7ceed000a0666dfafed91e703a08331139f0020e9fadfe`) if they are missing:

```rust
/// Slices 10f's and 10g's `check()` witnesses: 10f's one per row class ×
/// pada of its spec's Forms table, and 10g's one ṇic-less and one ṇic
/// witness per new code shape (num before `w W`, `p b B` and `h`, the
/// stripped `o~`, √kṣamp's 8.4.2, and the four udit shapes), plus every
/// homograph pair its spec names. A ṇic-less analysis opens with its Kaumudī
/// id, credits 1.3.78 and never 3.1.25; a ṇic one credits 3.1.25 and never a
/// Kaumudī id but 2573.2. The goldens were grepped for every witness first:
/// a single-root witness is its own row's alone, and a homograph witness is
/// exactly its rows', one analysis each (the `cah` / `rah` precedent). The
/// shapes the slices rule out — ṇic in an ākusmīya or ā-garvīya root's
/// parasmaipada, no ṇic in any ātmanepada, the unmerged aṅga–śap junction,
/// and the dental `n` 8.4.2 retroflexes across num — derive nothing.
#[test]
fn curadi_analyses_its_optional_nic_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    let has = |ids: &[String], id: &str| ids.iter().any(|i| i == id);
    // (form, its roots, pada, the Kaumudī id that opens it, or None for ṇic)
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
        ("laYjati", &["lanj", "lanj"], Pada::Parasmaipada, Some("2564")),
        ("vaRwati", &["vanw", "vanw"], Pada::Parasmaipada, Some("2564")),
        ("jasati", &["jas", "jas"], Pada::Parasmaipada, Some("2570")),
        // āṭ's vṛddhi gives `O` for both `o` and `u`: laṅ meets, laṭ does not.
        ("OlaRqad", &["olanq", "ulanq"], Pada::Parasmaipada, Some("2564")),
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
    for form in [
        "daMSayati",
        "garvayati",
        "daMSate",
        "garvate",
        "mUtrate",
        "patate",
        "katraati",
        "pataAmi",
        // Slice 10g: no ṇic in ātmanepada, and 8.4.2 across num.
        "cintate",
        "SarDate",
        "kzampAni",
        "kzampayAni",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
```

```python
#!/usr/bin/env python3
"""THROWAWAY: replace 10f's curadi_analyses_its_optional_nic_forms, doc
comment included, with check_10g.rs."""
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
i = s.index("/// Slice 10f's `check()` witnesses")
j = s.index('fn curadi_analyses_its_optional_nic_forms()', i)
k = s.index('\n}\n', j) + 3
assert s.count('fn curadi_analyses_its_optional_nic_forms()') == 1
s = s[:i] + open('/tmp/vidyut-full/slice10g/check_10g.rs').read() + s[k:]
open(p, 'w').write(s)
print("check witnesses replaced")
```

```bash
python3 /tmp/vidyut-full/slice10g/insert_check_10g.py      # check witnesses replaced
mise run fmt
```

It replaces 10f's `curadi_analyses_its_optional_nic_forms` (doc comment included). 10f's witnesses keep their rows; each tuple now names its roots, and a homograph witness expects one analysis per row. `devati` / `devayate` expect two (`10.0230`, `10.0249`), and `devayati` leaves the Invalid list for a one-analysis witness (`10.0249`'s ṇic parasmaipada). The 10g witnesses are the spec's: one ṇic-less and one ṇic form per new code shape (`cint`, `sPunw`, `canp`, `danh`, `lanq`, `kzanp`, `SraR`, `SfD`, `anc`) and every homograph pair (`lanj`, `vanw`, `jas`; `olanq` / `ulanq` in laṅ, each alone in laṭ). The Invalid list gains `cintate`, `SarDate`, `kzampAni`, `kzampayAni`.

- [ ] **Step 5: Run them to see them fail**

Run: `mise exec -- cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .*FAILED" | sort`
Expected failures (no row exists yet):

- `curadi::a_kusmad_is_credited_on_exactly_the_akusmiya_cells`
- `curadi_analyses_its_optional_nic_forms`
- `curadi::kzampARi_traces_8_4_2_across_num_on_both_branches`
- `curadi::natva_crosses_num_only_on_kzamp`
- `derivation_set_shape_matches_the_audited_numbers`
- `juhotyadi::nas_capadantasya_is_credited_only_on_rudhadi_dhan_jan_and_curadi_roots`
- `juhotyadi::shcutva_off_jan_is_credited_exactly_as_before_3f3`
- `tests::curated_roots_have_expected_ganas_and_padas`
- `tests::dhatupatha_numbers_resolve_upstream`
- `tests::optional_nic_matches_upadesha_markers`

`the_optional_nic_ids_are_credited_only_on_their_rows` and `pada_ambiguous_surfaces_are_exactly_these` still pass: with no new row they hold vacuously, or are unchanged.

- [ ] **Step 6: The rows, the goldens, and the measured pada-ambiguous set**

Create `/tmp/vidyut-full/slice10g/gen_rows_10g.py` (sha256 `3337aff80af176cf2c62d269261003cd0861581ff7db8ac4447eb1dbc9b10cc1`) and `/tmp/vidyut-full/slice10g/insert_rows_10g.py` (sha256 `4123f88035805311cfdbda24014b6f142ca07880097bd6686bec96363f1fc8bf`) if they are missing. The `ROWS` table is the spec's, with each row's laṭ parasmaipada prathama eka forms (ṇic-less, ṇic) for its comment, read from the generated goldens:

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10g — emit the 59 optional-ṇic `Dhatu` rows, the
`curadi_rows_are_…` list entries and the new `OPTIONAL_NIC` entries. Run
from the worktree root; writes rows.rs, rowlist.rs and table.rs into the
directory given."""
import sys, textwrap

OUT = sys.argv[1]
# (number, code, Kaumudī id, root, ṇic-less laṭ P 3sg, ṇic laṭ P 3sg)
ROWS = [
    ("10.0002", "cint", "2564", "cint", "cintati", "cintayati"),
    ("10.0003", "yantr", "2564", "yantr", "yantrati", "yantrayati"),
    ("10.0004", "sPunq", "2564", "sphuṇḍ", "sPuRqati", "sPuRqayati"),
    ("10.0005", "sPunw", "2564", "sphuṇṭ", "sPuRwati", "sPuRwayati"),
    ("10.0007", "kundr", "2564", "kundr", "kundrati", "kundrayati"),
    ("10.0009", "spunq", "2564", "spuṇḍ", "spuRqati", "spuRqayati"),
    ("10.0011", "mind", "2564", "mind", "mindati", "mindayati"),
    ("10.0013", "lanq", "2564", "laṇḍ", "laRqati", "laRqayati"),
    ("10.0014", "olanq", "2564", "olaṇḍ", "olaRqati", "olaRqayati"),
    ("10.0043", "SvanW", "2564", "śvaṇṭh", "SvaRWati", "SvaRWayati"),
    ("10.0045", "tunj", "2564", "tuñj", "tuYjati", "tuYjayati"),
    ("10.0047", "pinj", "2564", "piñj", "piYjati", "piYjayati"),
    ("10.0048", "lanj", "2564", "lañj", "laYjati", "laYjayati"),
    ("10.0049", "lunj", "2564", "luñj", "luYjati", "luYjayati"),
    ("10.0060", "panT", "2564", "panth", "panTati", "panTayati"),
    ("10.0062", "Cand", "2564", "chand", "Candati", "Candayati"),
    ("10.0066", "Kanq", "2564", "khaṇḍ", "KaRqati", "KaRqayati"),
    ("10.0067", "kanq", "2564", "kaṇḍ", "kaRqati", "kaRqayati"),
    ("10.0068", "kunq", "2564", "kuṇḍ", "kuRqati", "kuRqayati"),
    ("10.0069", "gunq", "2564", "guṇḍ", "guRqati", "guRqayati"),
    ("10.0070", "kunW", "2564", "kuṇṭh", "kuRWati", "kuRWayati"),
    ("10.0071", "gunW", "2564", "guṇṭh", "guRWati", "guRWayati"),
    ("10.0072", "Kunq", "2564", "khuṇḍ", "KuRqati", "KuRqayati"),
    ("10.0073", "vanw", "2564", "vaṇṭ", "vaRwati", "vaRwayati"),
    ("10.0074", "vanq", "2564", "vaṇḍ", "vaRqati", "vaRqayati"),
    ("10.0075", "canq", "2564", "caṇḍ", "caRqati", "caRqayati"),
    ("10.0076", "manq", "2564", "maṇḍ", "maRqati", "maRqayati"),
    ("10.0077", "Banq", "2564", "bhaṇḍ", "BaRqati", "BaRqayati"),
    ("10.0105", "ulanq", "2564", "ulaṇḍ", "ulaRqati", "ulaRqayati"),
    ("10.0106", "panq", "2564", "paṇḍ", "paRqati", "paRqayati"),
    ("10.0107", "pans", "2564", "paṃs", "paMsati", "paMsayati"),
    ("10.0111", "canp", "2564", "camp", "campati", "campayati"),
    ("10.0112", "kzanp", "2564", "kṣamp", "kzampati", "kzampayati"),
    ("10.0113", "kzanj", "2564", "kṣañj", "kzaYjati", "kzaYjayati"),
    ("10.0114", "Canj", "2564", "chañj", "CaYjati", "CaYjayati"),
    ("10.0130", "cunb", "2564", "cumb", "cumbati", "cumbayati"),
    ("10.0135", "wank", "2564", "ṭaṅk", "waNkati", "waNkayati"),
    ("10.0147", "SunW", "2564", "śuṇṭh", "SuRWati", "SuRWayati"),
    ("10.0153", "panc", "2564", "pañc", "paYcati", "paYcayati"),
    ("10.0157", "kunb", "2564", "kumb", "kumbati", "kumbayati"),
    ("10.0158", "kunB", "2564", "kumbh", "kumBati", "kumBayati"),
    ("10.0159", "lunb", "2564", "lumb", "lumbati", "lumbayati"),
    ("10.0160", "tunb", "2564", "tumb", "tumbati", "tumbayati"),
    ("10.0164", "cunw", "2564", "cuṇṭ", "cuRwati", "cuRwayati"),
    ("10.0166", "danh", "2564", "daṃh", "daMhati", "daMhayati"),
    ("10.0171", "Canp", "2564", "champ", "Campati", "Campayati"),
    ("10.0174", "SraR", "2570", "śraṇ", "SraRati", "SrARayati"),
    ("10.0182", "jans", "2564", "jaṃs", "jaMsati", "jaMsayati"),
    ("10.0184", "jas", "2570", "jas", "jasati", "jAsayati"),
    ("10.0185", "pinq", "2564", "piṇḍ", "piRqati", "piRqayati"),
    ("10.0241", "janB", "2564", "jambh", "jamBati", "jamBayati"),
    ("10.0243", "jas", "2570", "jas", "jasati", "jAsayati"),
    ("10.0249", "div", "2570", "div", "devati", "devayati"),
    ("10.0254", "tans", "2564", "taṃs", "taMsati", "taMsayati"),
    ("10.0260", "SfD", "2570", "śṛdh", "SarDati", "SarDayati"),
    ("10.0266", "anc", "2570", "añc", "aYcati", "aYcayati"),
    ("10.0267", "ling", "2564", "liṅg", "liNgati", "liNgayati"),
    ("10.0464", "vanw", "2564", "vaṇṭ", "vaRwati", "vaRwayati"),
    ("10.0465", "lanj", "2564", "lañj", "laYjati", "laYjayati"),
]
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
for n, code, trig, root, nicless, nic in ROWS:
    u, artha = up[n]
    kind = {"2564": "Idit", "2570": "Udit"}[trig]
    num = " (7.1.58's num stored)" if trig == "2564" else ""
    c = (f"{n} `{u}` {artha} (√{root}). {kind}: ṇic optional by Kaumudī {trig}{num}. "
         f"Ubhayapadī by 1.3.74 with ṇic (*{iast(nic)}*), parasmaipadī by 1.3.78 "
         f"without (*{iast(nicless)}*). Slice 10g.")
    com = '\n'.join('        // ' + l for l in textwrap.wrap(c, 72))
    out.append(f"    Dhatu {{\n{com}\n        dhatupatha: \"{n}\",\n        code: \"{code}\",\n"
               f"        gana: Gana::Curadi,\n        pada: PadaAssignment::Nic,\n"
               f"        artha: \"{artha}\",\n    }},\n")
    lst.append(f'                ("{n}", "{code}", PadaAssignment::Nic),\n')
table = ''.join(f'    ("{n}", "{trig}"),\n' for n, _, trig, *_ in ROWS)
open(f'{OUT}/rows.rs', 'w').write(''.join(out))
open(f'{OUT}/rowlist.rs', 'w').write(''.join(lst))
open(f'{OUT}/table.rs', 'w').write(table)
print(f"{len(ROWS)} rows")
```

```python
#!/usr/bin/env python3
"""THROWAWAY: insert gen_rows_10g.py's output ($GEN/rows.rs, rowlist.rs,
table.rs). The rows go after `10.0456 katra`, the last curādi row; the
table is rewritten in dhātupāṭha order."""
import os, re
GEN = os.environ['GEN']
p = 'crates/panini-data/src/lib.rs'
s = open(p).read()
a = '''        artha: "SETilye",
    },
];'''
assert s.count(a) == 1
s = s.replace(a, '''        artha: "SETilye",
    },
''' + open(f'{GEN}/rows.rs').read() + '];')
b = '''                ("10.0456", "katra", PadaAssignment::Nic),
'''
assert s.count(b) == 1
s = s.replace(b, b + open(f'{GEN}/rowlist.rs').read())
head = 'pub const OPTIONAL_NIC: &[(&str, &str)] = &[\n'
i = s.index(head) + len(head)
j = s.index('];', i)
entries = s[i:j] + open(f'{GEN}/table.rs').read()
lines = sorted(l for l in entries.splitlines(keepends=True) if l.strip())
assert len(lines) == 69, len(lines)
assert all(re.fullmatch(r'    \("10\.\d{4}", "[0-9.]+"\),\n', l) for l in lines)
s = s[:i] + ''.join(lines) + s[j:]
open(p, 'w').write(s)
print("inserted 59 rows; OPTIONAL_NIC has 69 entries")
```

```bash
GEN="$(mktemp -d)"
python3 /tmp/vidyut-full/slice10g/gen_rows_10g.py "$GEN"      # 59 rows
sha256sum "$GEN/rows.rs" "$GEN/rowlist.rs" "$GEN/table.rs"
GEN="$GEN" python3 /tmp/vidyut-full/slice10g/insert_rows_10g.py      # inserted 59 rows; OPTIONAL_NIC has 69 entries
```

Expected hashes: `rows.rs` `e96f9fdf392c11f252b6f10a2b26f88700c8ab05d3af05d357f79c39e459a7bd`, `rowlist.rs` `28af70cbb275ec9ab90198640668b4b74afefe4dcad1aa2963bf034b01d5f61e`, `table.rs` `15b8ca5922c5e078bc5c4725cfb2f824c524db700dc6b49c14adaf3c3920340e`. If any differs, stop and report. The rows go after `10.0456 katra`, the last curādi row (the data layer orders curādi by slice); the table is rewritten in dhātupāṭha order.

Repoint the vidyut checkout's dev-deps at this worktree, and create `/tmp/vidyut-full/vidyut-prakriya/examples/curadi_goldens_10g.rs` (sha256 `a00ce9089238602b4ec3617e814bb8a0b5f8d6ded0716af13123c2ddc2529d49`) and `/tmp/vidyut-full/slice10g/insert_goldens_10g.py` (sha256 `25a62aa96b6e1f7f4a1f1d0839c92157d57890aea98edfbfa3e7b9a34779f07d`) if they are missing:

```rust
//! THROWAWAY: slice 10g — emit the 59 optional-ṇic rows' goldens from the
//! engine, asserting every cell's form set equals vidyut's first.
use panini::Panini;
use panini_data::{dhatus, optional_nic, Lakara as L, Pada, Purusha as P, Vacana as V};
use vidyut_prakriya::args::{DhatuPada, Lakara, Prayoga, Purusha, Tinanta, Vacana};
use vidyut_prakriya::{Dhatupatha, Vyakarana};
const VIKALPA_RULES: &[&str] = &[
    "2564", "2570", "2573.1", "2573.3", "2573.2", "7.1.35", "3.4.111", "7.3.86", "6.4.107",
    "8.2.74", "8.2.75", "8.4.65", "8.4.56", "6.4.115", "6.4.117", "6.4.116", "6.4.43",
];
/// Slice 10f's ten optional-ṇic rows, whose goldens are already in.
const TENF: &[&str] = &["10.0193", "10.0194", "10.0198", "10.0199", "10.0227", "10.0230", "10.0400", "10.0449", "10.0451", "10.0456"];
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
    // The new rows: the `OPTIONAL_NIC` table less 10f's ten.
    for d in dhatus().iter().filter(|d| optional_nic(d.dhatupatha).is_some() && !TENF.contains(&d.dhatupatha)) {
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
    std::fs::write("/tmp/vidyut-full/goldens_10g_paradigm.rs", par).unwrap();
    std::fs::write("/tmp/vidyut-full/goldens_10g_alternates.rs", alt).unwrap();
    println!("{ncells} cells, {nforms} forms, {ndiff} differences");
    assert_eq!(ndiff, 0);
}
```

```python
#!/usr/bin/env python3
"""THROWAWAY: insert the generator's goldens before each static's `];`."""
p = 'crates/panini/tests/paradigm/data/curadi.rs'
s = open(p).read()
par = open('/tmp/vidyut-full/goldens_10g_paradigm.rs').read()
alt = open('/tmp/vidyut-full/goldens_10g_alternates.rs').read()
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
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example curadi_goldens_10g 2>/dev/null | tail -3)
sha256sum /tmp/vidyut-full/goldens_10g_paradigm.rs /tmp/vidyut-full/goldens_10g_alternates.rs
python3 /tmp/vidyut-full/slice10g/insert_goldens_10g.py      # inserted goldens
mise run fmt
```

Expected:
- the generator prints `4248 cells, 7080 forms, 0 differences`;
- `goldens_10g_paradigm.rs`: `f601c5e068d48ed7ca7223cafb7291471b72e594296ee9e30341962fa9eeacd5` (472 rows);
- `goldens_10g_alternates.rs`: `6c4cc435c70980107eb4f3a81381b94c5103676df502500e1ee344462b9162fb` (2832 rows).

Any `DIFF` line or other hash: stop and report, and edit nothing. Every new row's ṇic branch is live in both padas, so each pinned form is the ṇic one; the ṇic-less parasmaipada reading is an alternate keyed on its Kaumudī id. Leave the dev-deps pointed at this worktree for Task 4.

Then pin the pada-ambiguous set from the measured failure (`pada_ambiguous_surfaces_are_exactly_these` says it is "measured (never hand-picked)"). Create `/tmp/vidyut-full/slice10g/pin_ambiguous_10g.py` if it is missing (sha256 `ea33f67120fc9efa266de3b535b025e22cb28482a052fcdbeebfe67575aa8358`):

```python
#!/usr/bin/env python3
"""THROWAWAY: pin the measured pada-ambiguous set ($SET, the failing test's
`left:` JSON) into pada_ambiguous_surfaces_are_exactly_these."""
import hashlib, json, os
amb = json.load(open(os.environ['SET']))
assert len(amb) == 655, len(amb)
h = hashlib.sha256('\n'.join(amb).encode()).hexdigest()
assert h == '9da9a9aeed331b225e7e0dc6ecb546a264b55eec8eebf6de5f3a073e71b85fe4', h
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
SET="$SET" python3 /tmp/vidyut-full/slice10g/pin_ambiguous_10g.py      # pinned 655
mise run fmt
```

If the count or hash assertion fails, stop and report. The 223 new surfaces include `OlaRqayata` (once, for both `olanq` and `ulanq`) and `10.0249 div`'s `adevayata` / `devayatAm` / `devayetAm` / `devayeta`.

- [ ] **Step 7: Run the full suite**

```bash
mise run fmt
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS at 17964 cells, with `panini-data` 27, `panini-prakriya` 419, `trace` 210 and `paradigm` 25.

Grep the goldens for the `check()` witnesses, which the test also enforces:

```bash
for f in cintati cintayate sPuRwati sPuRwayate campati campayate daMhati daMhayate laRqati laRqayate kzampARi kzampayARi SraRati SrARayate SarDati SarDayate aYcati aYcayate laYjati vaRwati jasati OlaRqad olaRqati ulaRqati devati devayate devayati cintate SarDate kzampAni kzampayAni; do
  printf "%s: %s\n" $f "$(grep -c "\"$f\"" crates/panini/tests/paradigm/data/*.rs | grep -v ':0' | tr '\n' ' ')"; done
```

Expected: `laYjati`, `vaRwati`, `jasati`, `OlaRqad`, `devati` and `devayate` appear twice, in `curadi.rs` only; every other Valid witness once, in `curadi.rs` only; the four Invalid shapes print nothing.

- [ ] **Step 8: Commit**

```bash
mise run lint
git branch --show-current      # curadi-10g
git add -A
git commit -m "feat(data): curādi's fifty-nine idit and udit optional-ṇic rows

13716 → 17964 cells, 15644 → 22724 forms, ALTERNATES 1928 → 4760, 252 → 311
roots; pada-ambiguous surfaces 432 → 655. OPTIONAL_NIC (69) is held to the
vendored upadeśa and kept out of the āsvadīya/ādhṛṣīya; √śraṇ resolves by
its optional-ṇic verdict; goldens generated cell-by-cell equal to vidyut."
```

---

## Task 4: Audit, prior-trace diff, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (audit prose), `crates/panini-prakriya/src/tinanta/guna.rs` (one comment), `crates/panini-data/src/lib.rs` (one comment), the 10f spec

**Interfaces:**
- Consumes: the finished engine, data and goldens. Produces no symbols.

- [ ] **Step 1: Update the audit harness**

Create `/tmp/vidyut-full/slice10g/audit_10g.py` if it is missing (sha256 `95f73c36841caa6c5465b7f01afc3fac995562d49ab6251891082b37146841fd`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10g's edits to tools/audit/panini_full_audit.rs. Every
`old` must occur exactly once; nothing is written if one fails."""
import sys
p = 'tools/audit/panini_full_audit.rs'
s = open(p).read()
E = [
("//! What it compares: for each of the 252 curated roots, for each pada the root\n//! admits (`Dhatu::padas`; two apiece for the 129 roots that admit both padas —\n//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, ninety-six curādi\n//! roots by 1.3.74, and seven",
 "//! What it compares: for each of the 311 curated roots, for each pada the root\n//! admits (`Dhatu::padas`; two apiece for the 188 roots that admit both padas —\n//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 155 curādi\n//! roots by 1.3.74, and seven"),
("//! Corpus invariants, asserted: 252 roots, 13716 cells, 15644 forms. These are",
 "//! Corpus invariants, asserted: 311 roots, 17964 cells, 22724 forms. These are"),
("//! (`derivation_set_shape_matches_the_audited_numbers`): 1524 root×pada×lakāra\n//! blocks × 9 cells, plus 1928 `ALTERNATES` rows.",
 "//! (`derivation_set_shape_matches_the_audited_numbers`): 1996 root×pada×lakāra\n//! blocks × 9 cells, plus 4760 `ALTERNATES` rows."),
("//! Optionally dump the full 13716-cell table:", "//! Optionally dump the full 17964-cell table:"),
('    assert_eq!(roots_seen.len(), 252, "curated roots");\n    assert_eq!(n_cells, 13716, "cells: 1524 root×pada×lakāra blocks × 9");\n    assert_eq!(n_forms, 15644, "forms: 13716 cells + 1928 ALTERNATES rows");',
 '    assert_eq!(roots_seen.len(), 311, "curated roots");\n    assert_eq!(n_cells, 17964, "cells: 1996 root×pada×lakāra blocks × 9");\n    assert_eq!(n_forms, 22724, "forms: 17964 cells + 4760 ALTERNATES rows");'),
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
python3 /tmp/vidyut-full/slice10g/audit_10g.py      # applied 5 edits
```

The harness now asserts 311 / 17964 / 22724 and names 188 both-pada roots (129 + 59) and 155 1.3.74 rows.

- [ ] **Step 2: The prior-trace diff and the audit**

The dev-deps still point at this worktree from Task 3. Create `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_10g.rs` if it is missing (sha256 `d3cce3c0f544b88fdc0c01901459327511c2c9e0df5b8bae7c5e08d89f7db414`). It lists 10g's rows literally so that it also builds against main:

```rust
//! THROWAWAY: slice 10g — dump every prior cell's live-branch credited-rule log.
use panini::Panini;
use panini_data::{Lakara as L, Purusha as P, Vacana as V};
/// Slice 10g's fifty-nine rows, listed literally so the dump also builds
/// against main.
const NEW: &[&str] = &["10.0002", "10.0003", "10.0004", "10.0005", "10.0007", "10.0009", "10.0011", "10.0013", "10.0014", "10.0043", "10.0045", "10.0047", "10.0048", "10.0049", "10.0060", "10.0062", "10.0066", "10.0067", "10.0068", "10.0069", "10.0070", "10.0071", "10.0072", "10.0073", "10.0074", "10.0075", "10.0076", "10.0077", "10.0105", "10.0106", "10.0107", "10.0111", "10.0112", "10.0113", "10.0114", "10.0130", "10.0135", "10.0147", "10.0153", "10.0157", "10.0158", "10.0159", "10.0160", "10.0164", "10.0166", "10.0171", "10.0174", "10.0182", "10.0184", "10.0185", "10.0241", "10.0243", "10.0249", "10.0254", "10.0260", "10.0266", "10.0267", "10.0464", "10.0465"];
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
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10g 2>/dev/null > "$DUMP/branch.txt")
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml   # must point at /workspace/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10g 2>/dev/null > "$DUMP/main.txt")
wc -l < "$DUMP/main.txt"; wc -l < "$DUMP/branch.txt"            # 15644 and 15644
cmp "$DUMP/main.txt" "$DUMP/branch.txt" && echo PRIOR-TRACES-IDENTICAL
```

Expected: `15644`, `15644`, `PRIOR-TRACES-IDENTICAL`. This is the corpus-wide check that the `'M'` intervener fires on no prior root (goldens ignore traces). `/workspace` must be on `main` at the plan commit for the second dump. If `cmp` reports a difference, stop and report.

Then the audit. Repoint at this worktree again, copy the committed harness (never rewrite it), and run:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
cp tools/audit/panini_full_audit.rs $V/examples/
(cd $V && PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -8)
(cd $V && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -2)
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml
```

- Expected from the honest run: `roots : 311`, `cells : 17964`, `forms (set sizes): 22724`, `blocked branches : 2736`, `differing cells  : 0`, `AUDIT PASSED: 17964 cells, 22724 forms, zero differences.`
- Expected from the `entry` control: `AUDIT FAILED: 36 differing cells.`

Do not use `mise -C`. If the honest run shows differences, stop and report, and edit nothing.

- [ ] **Step 3: The doc sweep, the audit record and the spec pointer**

Create `/tmp/vidyut-full/slice10g/docsweep_10g.py` if it is missing (sha256 `3b00b65a74b33ae99611dcc7bdac871b6d4c498f368c9be09b11d4763bd99ad9`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10g's doc sweep, run from the worktree root with the
audit date as argv[1]. Every `old` must occur exactly once in its file; the
script checks all of them before writing any file, and writes nothing if
one fails."""
import sys

README = [
("8.4.44 *śāt* exemption. *curādi* (10) is **open** at 149 of its 509",
 "8.4.44 *śāt* exemption. *curādi* (10) is **open** at 208 of its 509"),
("by 6.1.97 or, after 7.3.101, 6.1.101 (*mūtrati*, *patāmi*); `pata` has a\nthird reading, 2573.2's *pātayati*.\n",
 "by 6.1.97 or, after 7.3.101, 6.1.101 (*mūtrati*, *patāmi*); `pata` has a\n"
 "third reading, 2573.2's *pātayati*. Slice 10g curated fifty-nine more:\n"
 "every idit (2564) and udit (2570) curādi row outside the āsvadīya and\n"
 "ādhṛṣīya but √ci, all ubhayapadī by 1.3.74 with ṇic (√cint, *cintayati*\n"
 "beside *cintati*). One of them, √kṣamp, made 8.4.2 *aṭkupvāṅnumvyavāye 'pi*\n"
 "read its *num*: the root's stored `n` reaches ṇatva as the anusvāra 8.3.24\n"
 "made of it, and the anusvāra now intervenes (*kṣampāṇi*).\n"),
("curated 252-root set, in four lakāras:", "curated 311-root set, in four lakāras:"),
("both correct — and in fact 1352 of the 13716 cells hold more than one form: 904\nhold two, 389 hold three",
 "both correct — and in fact 3476 of the 17964 cells hold more than one form: 2792\nhold two, 389 hold three"),
("optional-ṇic rows add 114 two-form and 46 three-form cells),\ntwenty-three hold four",
 "optional-ṇic rows add 114 two-form and 46 three-form cells, and slice 10g's\nfifty-nine add 1888 two-form cells, a ṇic and a ṇic-less reading each),\n141 hold four"),
("eka, two readings × 8.4.56), ten hold\nfive",
 "eka, two readings × 8.4.56, and — new in slice 10g — the fifty-nine\noptional-ṇic rows' laṅ and vidhiliṅ parasmaipada prathama eka, the same way),\nten hold\nfive"),
("and twenty-three hold six — the loṭ", "and 141 hold six — the loṭ"),
("`katra`'s loṭ parasmaipada prathama and madhyama eka (two readings × the\ntātaṅ triple). One cell",
 "`katra`'s loṭ parasmaipada prathama and madhyama eka (two readings × the\n"
 "tātaṅ triple); and, new in slice 10g, 118 more: the fifty-nine optional-ṇic\n"
 "rows' loṭ parasmaipada prathama and madhyama eka, the same way. One cell"),
("padas — 129 roots that admit both padas in the curated set",
 "padas — 188 roots that admit both padas in the curated set"),
("eighty-three ubhayapadī adanta roots and slice 10f's `mUtra`, `katra` and\n`pata` by 1.3.74; and slice 10f's six",
 "eighty-three ubhayapadī adanta roots, slice 10f's `mUtra`, `katra` and\n`pata` and slice 10g's fifty-nine optional-ṇic rows by 1.3.74; and slice 10f's six"),
("432 surfaces are pada-ambiguous, each of them a pinned cell in both padas",
 "655 surfaces are pada-ambiguous, each of them a pinned cell in both padas"),
("derive their two padas on different branches and add none. The\nenumeration is not",
 "derive their two padas on different branches and add none; slice 10g's\n"
 "fifty-nine rows add 223 more on their ṇic branch, four each but where a\n"
 "homograph pair shares them (`lanj`, `vanw` and `jas`, and the laṅ\n"
 "`OlaRqayata` of `olanq` and `ulanq`). The\nenumeration is not"),
("set, all 432. It is therefore", "set, all 655. It is therefore"),
]

ARCH = [
("— and curādi (10), **open** at 149 of its", "— and curādi (10), **open** at 208 of its"),
("ninety-two adanta roots, slice 10e; the ten optional-ṇic rows, slice 10f). gaṇa",
 "ninety-two adanta roots, slice 10e; the ten optional-ṇic rows, slice 10f; fifty-nine more, slice 10g). gaṇa"),
("are the goldens that pin it.\n",
 "are the goldens that pin it. Since slice 10g, 8.4.2's intervener set\n"
 "(`sound.rs`'s `is_natva_intervener`) also counts the anusvāra `M` as num's\n"
 "textual reading: a root's stored num reaches ṇatva as the anusvāra 8.3.24\n"
 "has made of it, and 8.4.58 turns it into a pu `m` only afterwards\n"
 "(√kṣamp's *kṣampāṇi*, `10.0112`).\n"),
("forking 362 cells (loṭ\nprathama and madhyama eka across the 181 roots with a parasmaipada column —",
 "forking 480 cells (loṭ\nprathama and madhyama eka across the 240 roots with a parasmaipada column —"),
("roots never reach this guard, and the 129 roots that admit both",
 "roots never reach this guard, and the 188 roots that admit both"),
("eighty-three ubhayapadī adanta roots and slice 10f's `mUtra`, `katra` and\n`pata` by 1.3.74, and slice 10f's six",
 "eighty-three ubhayapadī adanta roots, slice 10f's `mUtra`, `katra` and\n`pata` and slice 10g's fifty-nine optional-ṇic rows by 1.3.74, and slice 10f's six"),
("181 + 71 = the 252 curated roots)", "240 + 71 = the 311 curated roots)"),
("forking 354 cells outright: laṅ and vidhiliṅ prathama eka across\nthose same 181 parasmaipada columns (331 of them",
 "forking 468 cells outright: laṅ and vidhiliṅ prathama eka across\nthose same 240 parasmaipada columns (445 of them"),
("and so do 10f's `mUtra`, `katra` and `pata` on their ṇic branch; 10f's ṇic-less forks key on their Kaumudī id as well — `2564+8.4.56` and its siblings — and sit outside this count),",
 "and so do 10f's `mUtra`, `katra` and `pata` and 10g's rows other than √śṛdh and √div (whose ṇic branch keys on the sanādi 7.3.86 as well) on their ṇic branch; 10f's and 10g's ṇic-less forks key on their Kaumudī id as well — `2564+8.4.56` and its siblings — and sit outside this count),"),
("forking a further 362 (the same", "forking a further 480 (the same"),
]

AGENTS = [
("(`crates/panini/tests/paradigm/`, 13716 cells, ten gaṇas, nine complete —",
 "(`crates/panini/tests/paradigm/`, 17964 cells, ten gaṇas, nine complete —"),
("at 149 after slice 10f curated the ten optional-ṇic rows —",
 "at 149 after slice 10f curated the ten optional-ṇic rows, at 208 after slice 10g curated fifty-nine more —"),
("other forms — a second (904 cells), a third (389 cells), a fourth\n    (twenty-three",
 "other forms — a second (2792 cells), a third (389 cells), a fourth\n    (141"),
("    eka, and — new in slice 3f3 — √jan's, and — new in slice 10f — `mUtra`'s\n    and `katra`'s laṅ and vidhiliṅ prathama eka) and",
 "    eka, and — new in slice 3f3 — √jan's, and — new in slice 10f — `mUtra`'s\n    and `katra`'s laṅ and vidhiliṅ prathama eka, and — new in slice 10g — the\n    fifty-nine optional-ṇic rows') and"),
("    eka) to twenty-three — a fourth",
 "    eka) to twenty-three, and slice 10g's fifty-nine optional-ṇic rows (loṭ\n    prathama and madhyama eka) to 141 — a fourth"),
("    `ALTERNATES` (1928 rows in all, so 13716 + 1928 = 15644 forms total); √bhuj",
 "    `ALTERNATES` (4760 rows in all, so 17964 + 4760 = 22724 forms total); √bhuj"),
("  (`tools/audit/README.md`'s 2026-10-03 10f entry, 13716 cells / 15644 forms /\n  252 roots).",
 "  (`tools/audit/README.md`'s 2026-10-03 10f entry, 13716 cells / 15644 forms /\n  252 roots), and that by curādi 10g's (`tools/audit/README.md`'s @DATE@ 10g\n  entry, 17964 cells / 22724 forms / 311 roots)."),
("only in the ordinary corpus-size sense, not wrong in kind: 13716 goldens",
 "only in the ordinary corpus-size sense, not wrong in kind: 17964 goldens"),
]

TOOLS_README = [
("**It asserts the corpus totals** (252 roots, 13716 cells, 15644 forms) rather than",
 "**It asserts the corpus totals** (311 roots, 17964 cells, 22724 forms) rather than"),
("## Last recorded result\n\n",
 "## Last recorded result\n\n"
 "@DATE@, curādi 10g slice, vidyut\n"
 "`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 17964\n"
 "cells / 22724 forms / 311 roots**, with the `entry` negative control verified\n"
 "failing (36 √bhū cells).\n\n"
 "The verdict covers the whole curādi 10g slice: fifty-nine more rows whose ṇic\n"
 "is optional, every idit (Kaumudī 2564) and udit (2570) curādi row outside the\n"
 "āsvadīya and ādhṛṣīya but `10.0124 ciY`, each derived on its ṇic and its\n"
 "ṇic-less branch. Blocked branches rose from 612 to 2736, the 2124 = 59 × 36\n"
 "ṇic-less ātmanepada cells. Before 8.4.2 counted the anusvāra as an intervener\n"
 "the slice's only difference was one cell, `10.0112 kzanp`'s loṭ parasmaipada\n"
 "uttama eka (*kṣampāni* for *kṣampāṇi*). A main-vs-branch dump of every prior\n"
 "cell's traces was byte-identical, all 15644 live branches.\n\n"
 "Totals: 311 = 252 + 59; 17964 = 13716 + 4248 (472 root×pada×lakāra blocks ×\n"
 "9); 22724 = 15644 + 4248 + 2832 new `ALTERNATES` rows (1928 → 4760), measured\n"
 "via the harness's corpus block, not assumed.\n\n"),
]

MAIN_RS = [
("/// negative control verified failing (36 √bhū cells), and curādi 10f's\n/// re-ran it at the same commit over all 13716 cells / 15644 forms / 252\n/// roots with zero differences, its `entry` negative control verified\n/// failing (36 √bhū cells). √tṛh joins none of the fork",
 "/// negative control verified failing (36 √bhū cells), and curādi 10f's\n"
 "/// re-ran it at the same commit over all 13716 cells / 15644 forms / 252\n"
 "/// roots with zero differences, its `entry` negative control verified\n"
 "/// failing (36 √bhū cells), and curādi 10g's re-ran it at the same commit\n"
 "/// over all 17964 cells / 22724 forms / 311 roots with zero differences,\n"
 "/// its `entry` negative control verified failing (36 √bhū cells). √tṛh joins none of the fork"),
]

GUNA = [
("    // 252-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)",
 "    // 311-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)"),
]

DATA = [
("    /// vendored upadeśa: 66 of the 252 curated roots carry a `\\` at all, and 45",
 "    /// vendored upadeśa: 66 of the 311 curated roots carry a `\\` at all, and 45"),
]

SPEC_10F = [
("- Bulk optional-ṇic rows: the 57 idit (2564) and 10 ñit/udit (2570) rows,\n  then 10.0498 ādhṛṣīya",
 "- Bulk optional-ṇic rows: the 57 idit (2564) and 10 ñit/udit (2570) rows\n"
 "  (taken by slice 10g, all but √ci, see `2026-10-03-curadi-gana-10g-design.md`),\n"
 "  then 10.0498 ādhṛṣīya"),
]

FILES = {
    "README.md": README,
    "docs/ARCHITECTURE.md": ARCH,
    "AGENTS.md": AGENTS,
    "tools/audit/README.md": TOOLS_README,
    "crates/panini/tests/paradigm/main.rs": MAIN_RS,
    "crates/panini-prakriya/src/tinanta/guna.rs": GUNA,
    "crates/panini-data/src/lib.rs": DATA,
    "docs/superpowers/specs/2026-10-02-curadi-gana-10f-design.md": SPEC_10F,
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
python3 /tmp/vidyut-full/slice10g/docsweep_10g.py "$(date -u +%F)"      # applied 37 edits to 8 files
```

If it prints `not applied:`, nothing was written. Edit the paragraph named to the same facts, rather than skipping it, and re-run.

Notes on the numbers, re-derived from the census rather than assumed:
- 3476 = 17964 − 14488; 2792 / 389 / 141 / 10 / 141 / 1 / 2 are the census buckets, and no cell holds eight.
- 188 = 129 + 59 roots that admit both padas, all fifty-nine by 1.3.74 with ṇic. 240 + 71 = 311: every new row has a parasmaipada column; the ātmanepada-only count is unchanged.
- 480 = 240 × 2 (7.1.35's loṭ prathama and madhyama eka, and 8.4.56's "further" fork of the same cells). 468 = `key_count("8.4.56")` = 445 + 22 + 1, where 445 = 331 + 114: the fifty-seven new rows without a sanādi guṇa fork laṅ and vidhiliṅ prathama eka on their ṇic branch; √śṛdh's and √div's key on 7.3.86 as well, and the ṇic-less forks key on their Kaumudī id, so neither sits in it.
- The rule-order pin stays at 148 entries: 8.4.2 is already pinned and no id is added. Change nothing there.
- `Dhatu::pada`'s "66 of the 311 curated roots carry a `\`": none of the fifty-nine upadeśas has one (measured).

- [ ] **Step 4: The AGENTS.md stale-comment ledger**

Create `/tmp/vidyut-full/slice10g/ledger_10g.py` if it is missing (sha256 `dfd9fee2450fe6eb44bf53e0e1db29eb475bf67e4023941ae345885f040f88ee`). It measures both anchors by grep at this commit; it never computes them:

```python
#!/usr/bin/env python3
"""THROWAWAY: add 10g's sentence to AGENTS.md's stale-comment ledger, with
both anchors measured by grep now."""
import re, subprocess
g = subprocess.run(['grep', '-n', '1872 goldens move', 'crates/panini-prakriya/src/tinanta/guna.rs'], capture_output=True, text=True).stdout.split(':')[0]
c = subprocess.run(['grep', '-n', 'only 8 cells fire', 'crates/panini-prakriya/src/controller.rs'], capture_output=True, text=True).stdout.split(':')[0]
assert g and c
p = 'AGENTS.md'
s = open(p).read()
m = re.search(r"Curādi 10f touched neither comment either; the corpus stands at 13716 cells as of 10f \(`guna\.rs:2565`'s claim anchored at `guna\.rs:\d+`, `controller\.rs:206`'s at `controller\.rs:\d+`; both lines measured by grep at this commit\)\.", s)
assert m
new = m.group(0) + f" Curādi 10g touched neither comment either; the corpus stands at 17964 cells as of 10g (`guna.rs:2565`'s claim anchored at `guna.rs:{g}`, `controller.rs:206`'s at `controller.rs:{c}`; both lines measured by grep at this commit)."
s = s[:m.start()] + new + s[m.end():]
open(p, 'w').write(s)
print(f"ledger: guna.rs:{g} controller.rs:{c}")
```

```bash
python3 /tmp/vidyut-full/slice10g/ledger_10g.py      # ledger: guna.rs:2565 controller.rs:206
```

AGENTS.md's floor paragraph (`measured at 13716 cells`) and the current mutation record belong to Task 5.

- [ ] **Step 5: Sweep for anything left stale**

```bash
grep -rn -i -E "\b13716\b|\b15644\b|\b1928\b|\b1524\b|252 roots|252-root|of these 252|of the 252|252 curated|149 of|\b129 roots|ninety-six curādi|\b432\b|\b12364\b|\b904\b|181 \+ 71|forking 362|\b354 cells|\b612\b|twenty-three hold|ten optional|cannot occur in the intervening|Revisit when either|one_hundred_forty_nine" README.md AGENTS.md docs/ARCHITECTURE.md tools/audit/README.md crates --include=*.md --include=*.rs | grep -v "paradigm/data/"
grep -rn -i -E "exactly the ten|the ten (curated )?rows|ten optional|OPTIONAL_NIC.{0,40}ten|ten.{0,40}OPTIONAL_NIC|173 optional|uncurated.{0,60}(idit|udit|2564|2570)|(idit|udit|2564|2570).{0,60}(uncurated|not (yet )?curated|later slice|bulk slice)" README.md AGENTS.md docs/ARCHITECTURE.md tools crates --include=*.md --include=*.rs | grep -v "paradigm/data/"
for code in cint yantr sPunq sPunw kundr spunq mind lanq olanq SvanW tunj pinj lanj lunj panT Cand Kanq kanq kunq gunq kunW gunW Kunq vanw vanq canq manq Banq ulanq panq pans canp kzanp kzanj Canj cunb wank SunW panc kunb kunB lunb tunb cunw danh Canp jans pinq janB tans ling SraR jas SfD anc; do
  grep -rln "\"$code\"" crates --include=*.rs | grep -v "paradigm/data/" | LC_ALL=C sort | tr '\n' ' ' | sed "s#^#$code: #"; echo; done | grep -v -E "^[A-Za-z]+: crates/panini-data/src/lib.rs $|^[A-Za-z]+: crates/panini-data/src/lib.rs crates/panini/tests/paradigm/main.rs $"
```

Expected residue from the first grep, all of it dated history that stays true:
- `tools/audit/README.md`'s 10g entry's own `Totals:` and blocked-branch lines, and the 10f and older entries;
- AGENTS.md:37's floor paragraph (Task 5), its curādi chain's 10f clause ("the ten optional-ṇic rows"), its audit chain (the 10f clause) and its stale-comment ledger;
- `paradigm/main.rs`'s audit chain (the 10f clause), its 10d and 10f paragraphs ("432 new cells", "the ten optional-ṇic rows", "OPEN at 149"), and the pada-ambiguous comment's "420 to 432" / "432 to 655";
- `docs/ARCHITECTURE.md`'s "the ten optional-ṇic rows, slice 10f".

The second grep may print only dated 10f history (README's and `tools/audit/README.md`'s "Slice 10f curated the ten rows…", `paradigm/main.rs`'s 10f paragraph, ARCHITECTURE's and AGENTS.md's 10f clauses). The third loop must print nothing: every new code appears only in `panini-data` and, for the witnesses, `paradigm/main.rs`. Any other hit from any of the three is a miss. Edit it.

- [ ] **Step 6: Run the full suite and commit**

Run: `mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"` (foreground, timeout 600000 ms). Expected: PASS at 17964 cells.

```bash
mise run fmt && mise run lint
git branch --show-current      # curadi-10g
git add -A
git commit -m "docs: 10g's counts, the audit record, and the sweep

17964 cells / 22724 forms / 311 roots across README, ARCHITECTURE, AGENTS,
paradigm/main.rs and tools/audit; curādi open at 208/509; 188 both-pada
roots; 655 pada-ambiguous surfaces; 8.4.2's num reading documented. Audit
at zero divergence against 8da2f90b with 2736 blocked branches; prior traces
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
- **The mutant list is unchanged:** 830 in panini-prakriya (`--list` on the prototype equals main's count), 12 in panini-analyze. `--in-diff` over the slice's production diff lists only `sound.rs:147:5: replace is_natva_intervener -> bool with true` and `… with false`, which predate the slice (the function moved three lines down; both were caught in 10f). cargo-mutants generates no mutant inside a `matches!` pattern, so `'M'` has none of its own: Task 2's red step (both tests fail without it) and Task 3's `natva_crosses_num_only_on_kzamp` / `check("kzampAni")` are its coverage. The data crate's diff is data and test code only: `--in-diff` prints `No mutants to filter`.
- **The three documented non-caught entries do not move:** `adesha.rs:647:30`, `tripadi.rs:1303:38` and `tripadi.rs:1616:23` (Task 2's tripadi test sits below them; `sound.rs` holds no non-caught entry). Confirm all three by `--list`; never compute them.
- **The floor at 17964 cells:** two `mise run test` runs took 1m47.395s and 1m41.325s wall clock (user 3m46.1s and 3m42.0s), load averages `43.78 50.06 60.47` before, `58.81 53.25 60.44` between, `61.95 55.55 60.50` after, on 24 cores (external load).
- **The isolated `-j 4` uncaught probe** (`--timeout 1210`, load 61–73): `adesha.rs:647:30` MISSED with a 131.33s test phase, `tripadi.rs:1303:38` MISSED with 130.86s. 6 × 131.33s = 788s, under the 1210 cap in force, so the provisional cap stays **1210**.

- [ ] **Step 1: Measure the floor**

With nothing else of ours running, run this twice: `cat /proc/loadavg; time mise run test >/dev/null 2>&1; cat /proc/loadavg` (foreground, timeout 600000 ms). Record both wall clocks, user CPU and the load averages. Read 10f's floor from AGENTS.md's floor paragraph and keep the comparison chain.

- [ ] **Step 2: Locate and probe the two uncaught equivalents at `-j 4`**

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | wc -l      # 830
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /="
```

Expect `647` (adesha), `1303` (tripadi, inside 8.3.13's `apply`) and `1616` (the ṇatva hang). Write them as `<A>`, `<T1>` and `<T2>`. Then run in the foreground with timeout 600000 ms:

```bash
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 1210 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:<A>:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" 2>&1 | tail -6
```

The two regexes also match two caught `mod.rs` mutants; that is expected. Both equivalents must be MISSED, not TIMEOUT. Read each test-phase duration from `$SCRATCH/mutants.out/outcomes.json`. Set the provisional cap: max(1210, 6 × the longer of the two, rounded up to the next 10 s).

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/curadi-10g"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout <CAP> -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"; cat /proc/loadavg > "$OUT/load.started"
```

`<CAP>` is Step 2's provisional cap. Run nothing CPU-heavy meanwhile. 10f's campaign took 72 minutes with a suite half this one's uncaught length; expect longer. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

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
- **842 mutants: 791 caught, 48 unviable, 2 missed, 1 timeout.**
  - panini-prakriya: 830 / 783 / 44 / 2 / 1.
  - panini-analyze: 12 / 8 / 4 / 0 / 0.
- `missed.txt` holds exactly `adesha.rs:<A>:30: replace + with *` and `tripadi.rs:<T1>:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:<T2>:23: replace -= with /=`.

Then diff the non-caught set against 10f's on the full record:

```bash
python3 - "$OUT/outcomes.durable.json" /home/dev/mutants-records/curadi-10f/outcomes.durable.json <<'PY'
import json, sys
def non(p):
    out = set()
    for o in json.load(open(p))["outcomes"]:
        sc = o["scenario"]
        if isinstance(sc, dict) and "Mutant" in sc and o["summary"] != "CaughtMutant":
            m = sc["Mutant"]; s = m["span"]["start"]
            out.add((m["package"], m["file"], s["line"], s["column"], m["replacement"], (m.get("function") or {}).get("function_name"), m["genre"], o["summary"]))
    return out
a, b = non(sys.argv[1]), non(sys.argv[2])
print(len(a), len(b)); print("new:", sorted(a - b)); print("gone:", sorted(b - a))
PY
```

Expected: `51 51`, `new: []`, `gone: []`: the slice moves no non-caught span.

If not:
- Any **other timeout** is a suspect survivor that the larger suite pushed past the cap. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant in `is_natva_intervener` is a gap in Task 2's tests; add the test that kills it. Any other missed mutant means a test that caught it at 13716 cells no longer does; stop and report.

**Step 4b: the data-crate mutants.** Run the slice's diff for `panini-data` alone:

```bash
git diff cdf78e0 -- crates/panini-data/src/lib.rs > "$OUT/data.diff"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-data --test-workspace=true \
  --in-diff "$OUT/data.diff" --timeout <CAP> -j 4 -o "$OUT/data" 2>&1 | tail -3
```

Expected: `No mutants to filter` (the diff is rows, the table, docs and test code).

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
- If that is 1210, the cap stays.
- Otherwise change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together (`grep -n "1210" AGENTS.md mise.toml`).

- [ ] **Step 6: Record it in AGENTS.md**

- Rewrite the paragraph that opens `**The floor behind the 1210s cap, measured at 13716 cells on Rust 1.99.0,`. Use Step 1's and Step 2's numbers at 17964 cells, the load averages, and the cap Step 5 chose. Keep the comparison chain to earlier floors, with 10f's (47.778s / 43.068s at 13716 cells, isolated probe 59.32s / 58.53s) joining it.
- Replace the `**Current record (curādi 10f, …).**` paragraph with `**Current record (curādi 10g, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**;
  - the non-caught set diffed against 10f's on the full record (Step 4's script output);
  - that the slice adds no mutant: the `'M'` arm sits inside `matches!`, the only in-diff mutants are `sound.rs:147:5`'s two pre-existing ones (name their outcomes), and Step 4b's data diff has none;
  - the campaign-load phases and margins;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`.

  End it with a pointer to the record it replaces. Run `git rev-parse --short HEAD` before committing, and write ``The curādi 10f record it replaces: `git show <that hash>:AGENTS.md`.``

- [ ] **Step 7: Commit**

```bash
git branch --show-current      # curadi-10g
git add AGENTS.md mise.toml
git commit -m "chore: 10g mutation gate — floor and uncaught run re-measured at 17964 cells

No new mutant (8.4.2's 'M' sits in a matches! pattern; Task 2's red step
covers it); missed.txt holds only the two documented equivalents and
timeout.txt only the permanent ṇatva-scan entry, at unmoved spans."
```

---

## Task 6: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -20
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # back at /workspace/crates
git branch --show-current      # curadi-10g
git log --oneline main..HEAD   # the four task commits
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin curadi-10g
gh pr create --title "curādi 10g — bulk optional ṇic (2564, 2570)" --body "$(cat <<'BODY'
Slice 10g curates the fifty-nine remaining optional-ṇic curādi rows that
Kaumudī 2564 (idit, 53) and 2570 (udit, 6) open: every one outside the
āsvadīya and ādhṛṣīya but √ci, all ubhayapadī by 1.3.74 with ṇic and
parasmaipadī by 1.3.78 without (*cintayati* beside *cintati*).

- 10f's mechanism carries every row; no rule id is added.
- 8.4.2 reads num as the anusvāra 8.3.24 made of it: √kṣamp's *kṣampāṇi*.
- `OPTIONAL_NIC` (69) is kept out of 10.0279–10.0388, where vidyut's
  gaṇasūtras precede the marker reading.
- The upstream uniqueness check tells √śraṇ (`10.0174 SraRu~`) from the
  uncurated `10.0063 SraRa~` by their optional-ṇic verdict.
- Homographs (`lanj`, `vanw`, `jas`, `div`, `olanq`/`ulanq`) report one
  analysis per row.

The golden suite goes from 13716 to 17964 cells; curādi is open at 208 of
509. The audit shows zero divergence against `8da2f90b`, a main-vs-branch
dump of every prior cell's traces is byte-identical, and the mutation gate
finds the documented non-caught set unchanged.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`:
   - run `git worktree remove .worktrees/curadi-10g`;
   - run `git worktree remove --force .worktrees/curadi-10g-proto` (the throwaway);
   - delete the local and remote `curadi-10g` branch;
   - run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| 53 idit + 6 udit rows, codes, all `Nic` | 3 Step 6 |
| `OPTIONAL_NIC` 10 → 69 | 3 Steps 3, 6 |
| New code shapes decided by `stored_form` (num before `w W`, `p b B`, `h`; stripped `o~`) | 3 Step 6 (the uniqueness test re-derives every code) |
| 8.4.2 `'M'`, doc rewritten, āṅ note kept | 2 |
| Witness *kṣampāṇi* / *kṣampayāṇi*, 8.4.2 before 8.4.58 on both branches | 3 Steps 2, 4 |
| Corpus-wide "fires across an anusvāra only on `kzanp`"; trace diff main ↔ HEAD | 3 Step 2; 4 Step 2 |
| Marker helper stays marker-only; `10.0279..=10.0388` exclusion | 3 Step 3 |
| Verdict-aware uniqueness; √śraṇ pin | 3 Step 3 |
| Homographs (cah/rah precedent): `div`, `lanj`, `vanw`, `jas`, `olanq`/`ulanq` | 3 Step 4; pada-ambiguous pin, Step 6 |
| Goldens via the harness; census 311 / 17964 / 22724; blocked +2124 | 3 Steps 1, 6; 4 Step 2 |
| 1.3.74 count 96 → 155 | 3 Steps 2, 3 |
| Rosters 8.3.24, 8.4.40, 2564, 2570 from the row tables | 3 Step 2 |
| Witness per new code shape, ṇic-less (trigger first, 1.3.78, no 3.1.25) and ṇic (3.1.25, no trigger) | 3 Step 4 |
| Grep goldens before asserting counts | 3 Step 7 |
| Mutation gate: scoped, floor re-measured, `-o`, durable copy, chunking, non-caught named verbatim | 5 |
| AGENTS.md counts, census, audit, mutation record | 4 Steps 3–4; 5 |
| ARCHITECTURE fork counts; pin count unchanged (148) | 4 Step 3 |
| `tools/audit/README.md` totals | 4 Steps 1, 3 |
| `panini-data`: `OPTIONAL_NIC` doc, `Dhatu::pada` census, "ten" phrasing | 3 Step 3 (census); the `OPTIONAL_NIC` doc names no count and stays; 4 Step 5's second grep finds no "ten" claim |
| `sound.rs` doc | 2 |
| 10f spec's "Later slices" pointer | 4 Step 3 |
| Sweep greps: root-shape literals, all of `crates/`, spelled-out and wrapped counts, counts re-derived | 4 Steps 3, 5 |

**Type consistency.**
- `natva_crossed_num(step: &panini_prakriya::RuleStep) -> bool` reads `RuleStep.before` / `.after` (`String`), both public; it compiled in the prototype.
- `curadi_analyses_its_optional_nic_forms`' tuples are `(&str, &[&str], Pada, Option<&str>)`; the first tuple's `&["danS"][..]` fixes the slice type for the rest.
- `dhatupatha_numbers_resolve_upstream` compares `Option<&'static str>` verdicts from `optional_nic_from_upadesha`, defined above it in the same `mod tests`.
- `("10.0279"..="10.0388").contains(n)` takes the `&&str` the `OPTIONAL_NIC` iterator yields, as `AKUSMIYA.contains` does elsewhere.
- The golden tuple shapes match `ParadigmRow` and `AlternateRow`; the generator is 10f's with the row filter changed.

**Known soft spots.**
- **Doc strings in Task 4** were read at the prototype's state. If the script reports an `old` not found exactly once, edit that paragraph to the same facts rather than skip it.
- **Task 5's campaign numbers are expectations**, not prototype measurements; the floor and cap depend on host load. Record the load beside every timing.
- **A pre-existing stale claim, left alone:** `docs/ARCHITECTURE.md`'s ṇatva paragraph and `tripadi.rs`'s `is_natva_target` doc still say the engine has "no anusvāra machinery", which 8.3.24 has falsified since slice 3f. This slice's added sentence states the current fact beside it; retiring the old claim belongs to whichever slice retires `is_natva_target`'s jhal guard.
- **`gen_rows_10g.py`'s comment forms** were read from the generated goldens (a second pass), so the rows' comments and the goldens agree by construction; the hashes pin both.
