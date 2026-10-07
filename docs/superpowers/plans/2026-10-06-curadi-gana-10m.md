# Curādi gaṇa slice 10m Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate `10.0175 picca~` (√picc, *piccayati*). To do that, narrow 8.2.30 *coḥ kuḥ* from a whole-word scan to vidyut-prakriya's reading: a **term-final** cu, before a jhal-initial affix or āgama, or pada-final. The golden suite goes from 38160 to 38232 cells, and curādi from 490 to 491 of its 509 rows.

**Architecture:** Six tasks:
- **Task 1** creates the worktree and checks the baseline.
- **Task 2** is the engine, green on its own. 8.2.30's `apply` stops reading `word_chars` and walks the terms. A term qualifies if its last sound is a cu, and the next non-empty term is not `ANGA` and begins with a jhal, or no non-empty term follows. The comment block is rewritten to match. Three neighbouring comments that described the old scan are corrected. Tests go in first and fail: four hand-built unit cases, a hand-built `picc` derivation, and a corpus-wide roster of 8.2.30's credits. The roster passes before and after, by design: it guards the narrowing against changing any prior credit.
- **Task 3** lands the row, 8 golden rows and 6 alternates, and every count, list, roster and `check()` assertion they move. The assertions go in first and fail; the row and goldens make them pass.
- **Tasks 4–6** are the audit with the prior-trace diff and the doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.99.0, pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, checked out at `/tmp/vidyut-full`.

**Spec:** `docs/superpowers/specs/2026-10-06-curadi-gana-10m-design.md`. Its Decisions section explains why 8.2.30 reads terms, why the next term must not be `ANGA` (and why that guard has no golden witness), why rudhādi is unaffected by construction, and which comment paragraphs stop being true.

**Workspace:** The spec and this plan are on the branch `curadi-10m`, which is checked out at `/workspace`. Task 1 puts `/workspace` back on `main` and checks `curadi-10m` out at `/workspace/.worktrees/curadi-10m`. Every path below is relative to that directory unless it starts with `/`.

**Provenance.** This slice was built end to end on a throwaway worktree, `/workspace/.worktrees/curadi-10m-proto` (branch `proto-10m`, on the spec commit `6b1ec29`). Its commits are `6e51267` (Task 2), `70617b9` (Task 3) and `cdfab27` (Task 4), and Task 6 deletes it. Every script below was run against it and checked four ways:
- **Prototype checks.** At `cdfab27`:
  - the full suite, clippy `-D warnings` and `fmt-check` passed;
  - the generator found all 72 new cells equal to vidyut's (`72 cells, 78 forms, 0 differences`);
  - the audit at 594 roots / 38232 cells / 49904 forms showed zero differences, with 6552 blocked branches and the `entry` control failing on 36 cells;
  - a main-vs-prototype dump of every prior branch, blocked ones included, with every step's before and after text (56378 lines, 49826 live), was byte-identical.
- **Replay.** The scripts below, run in this plan's order on a fresh detached worktree at `6b1ec29` (`/workspace/.worktrees/curadi-10m-replay`), reproduced the prototype's tree after each of the three commits, byte for byte in every tracked file.
- **Negative control.** In the brainstorm's scoping prototype, main's 8.2.30 with the row kept made the audit fail on 72 cells, all `10.0175`'s (*pikcayati*).
- **Guard reachability.** With the `j != ANGA` conjunct deleted, the full trace dump was byte-identical, so the guard is unreachable in the corpus. Only Task 2's hand-built abhyāsa case can see it.
- **Mutants.** `--in-diff` over the slice's `panini-prakriya` diff lists 7 mutants, all in 8.2.30's new `apply`. The prototype ran all 7 (`-j 4 --timeout 11110`, `--test-workspace=true`, 3 minutes, load 36–77): **all 7 CAUGHT**, none unviable. Test phases: 0.85 s and 1.01 s for `-` → `/` and `-` → `+` on `count() - 1` (an out-of-range index panics at once), 46–53 s for the other five.

The prototype did **not** run the full mutation campaign or the `skip_nic` probe. Task 5's campaign numbers are expectations derived from the measured mutant lists and the in-diff run, and the campaign measures them.

**Throwaway scripts.** Everything under `/tmp/vidyut-full/slice10m/` and the two vidyut examples (`curadi_goldens_10m.rs`, `trace_dump_10m.rs`) never ship. Each is reproduced in full in this plan with its sha256, so it can be recreated if `/tmp` was cleaned. Recreate a file only if it is missing, and check its hash either way (`sha256sum <file>`). The scripts read nothing but the worktree, except `pin_ambiguous_10m.py` (the failing test's output), `insert_goldens_10m.py` (the generator's output) and `docsweep_10m.py` (the `AUDIT_DATE` environment variable).

## Global Constraints

- **The engine change is exactly this:** 8.2.30's `apply` in `tinanta/tripadi.rs`, as Task 2 Step 3 writes it, and comments. No rule is added, removed, reordered or made optional. `tinanta_rule_order_is_pinned` (166 ids) and `exactly_the_pinned_vikalpa_rules_are_optional` are unchanged. `kutva_of` still drives both the match and the substitute.
- **Not ported:** vidyut's upadhā `Y` → `N` inside its 8.2.30. 8.2.31 *ho ḍhaḥ* keeps its whole-word scan; this slice narrows 8.2.30 alone.
- **Row:** exactly `Dhatu { dhatupatha: "10.0175", code: "picc", gana: Gana::Curadi, pada: PadaAssignment::Nic, artha: "kuwwane" }`, appended to `DHATUS` after 10l's block. `OPTIONAL_NIC` is unchanged (182): √picc's ṇic is obligatory.
- **Out of scope:** the other eighteen curādi rows, upasargas and `10.0368 za\da~`, the causative.
- **Pre-existing cells must stay byte-identical, traces included,** with no exception. Regenerate no prior golden.
- **Goldens come from the generator, which asserts engine = vidyut cell by cell, and their sha256 must match this plan's.** **Do not edit a golden to match the engine.** If the generator reports a difference, or a hash differs, stop and report.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit. Run `git branch --show-current` before every commit and the push: it must print `curadi-10m` (a detached HEAD strands commits).
- `mise run test` took 3–5 minutes in the prototype; the `trace` binary alone ran 147–262 s under external load. Run it in the **foreground** with a timeout of 600000 ms. If a run outgrows the 10-minute cap, start it detached with its output in a log file and an `EXIT_CODE=` sentinel, then wait in the same turn with `while kill -0 <PID>; do sleep 30; done`. Never background it and end a turn, never arm a Monitor and end a turn, and never pipe it through `tail`.
- `mise run test -- -p X` does not scope. Scope with `mise exec -- cargo test -p <crate> <filter>`. To see every failing binary at once, use `mise exec -- cargo test --workspace --no-fail-fast`.
- The `cargo-mutants` mise shim fails here. Use the real binary, `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`, under `mise exec --`.
- `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes its `panini` and `panini-data` dev-deps. Repoint them at this worktree before generating or auditing, and back at `/workspace/crates` after. Check with `grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml` every time.
- Never wait on a process with `pgrep -f` (it matches its own shell); use `pgrep -x cargo-mutants` or `kill -0 <pid>`.
- Review packages and diffs exclude the appended goldens (`crates/panini/tests/paradigm/data/curadi.rs`) and use `git diff --diff-algorithm=histogram`: the default algorithm shows appended golden rows as fake deletions.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **A prior 8.2.30 credit silently lost or gained.** The narrowing could drop a rudhādi or juhotyādi firing whose surface still matches (8.4.55 devoices a `g` and a `j` alike before `t`), or add one on a prior row. → Task 2 `coh_kuh_is_credited_on_exactly_its_twelve_rows` (every row and its live-branch count, sorted); Task 4's prior-trace diff (every step's before and after).
2. **The `ANGA` guard.** No abhyāsa or aṭ in the corpus ends in a consonant, so deleting `j != ANGA` changes nothing a golden can see. → Task 2 `coh_kuh_reads_only_a_terms_final_cu_before_an_affix`'s hand-built `jaj | da` case.
3. **An empty term between the cu and its jhal.** adādi's luk'd śap leaves `SHAP` empty. Reading `i + 1` instead of the next non-empty term would decline there. → Task 2's `vac | (empty) | ti` → `vakti` case.
4. **An earlier non-qualifying cu hiding a later one.** A `find` that stops at the first cu-final term, rather than the first qualifying one, would decline on `pac | anaj | ti`. → Task 2's `pacanagti` case.
5. **The witness forms.** *piccayati* and *piccayate* must each have exactly one analysis, and the old wrong form and a guṇated one none. → Task 3 `curadi_analyses_its_10m_forms`, and Step 7's goldens grep.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | Task 2: 8.2.30's `apply` and comment block, the 8.2.31 and 8.2.41-section comments, the old unit test's doc, the new unit test. Task 4: one comment in the 8.4.40 section |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Task 2: `picc_keeps_its_root_internal_cc` |
| `crates/panini/tests/trace/curadi.rs` | Task 2: `coh_kuh_is_credited_on_exactly_its_twelve_rows`. Task 3: its `10.0175` assertion, the 1.3.74 count (428) |
| `crates/panini-data/src/lib.rs` | Task 3: the row, `Dhatu::pada`'s and the marker doc's counts, the row count (594), the row-list test (renamed `…_four_hundred_ninety_one_…`) |
| `crates/panini/tests/paradigm/main.rs` | Task 3: census and keys and their 10m paragraph, the pada-ambiguous set (1635), `curadi_analyses_its_10m_forms`. Task 4: doc counts, the alternates doc, the audit chain |
| `crates/panini/tests/paradigm/data/curadi.rs` | Task 3: 8 golden rows, 6 alternates |
| `crates/panini-prakriya/src/tinanta/guna.rs` | Task 4: one comment's corpus size |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, the 10l spec | Task 4 |
| `AGENTS.md`, maybe `mise.toml` | Task 5 |

---

## Task 1: The worktree and baseline

**Files:** none. **Interfaces:** none.

- [ ] **Step 1: Create the worktree**

```bash
cd /workspace
git status --short                   # empty
git branch --show-current            # curadi-10m
git log --oneline -3                 # the plan commit, the spec commit 6b1ec29, then 2bb7a4f
git checkout main                    # curadi-10m can be checked out in one worktree only
git log --oneline -1                 # 2bb7a4f, the 10l merge
git worktree add .worktrees/curadi-10m curadi-10m
cd .worktrees/curadi-10m
git branch --show-current            # curadi-10m
```

`/workspace` stays on `main` for the whole slice: Task 4's prior-trace dump builds the main engine from `/workspace/crates`.

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: everything passes at 38160 cells. `panini-prakriya` reports 445 tests, the `trace` binary 216, `paradigm` 30 and `panini-data` 28 (and `panini` 7, `roundtrip` 1, `panini-analyze` 8, `cli` 5, `panini-lipi` 6).

---

## Task 2: The engine — 8.2.30 reads a term-final cu

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (8.2.30's `apply`, comments, tests)
- Test: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`, `crates/panini/tests/trace/curadi.rs`

**Interfaces:**
- Consumes: `kutva_of(char) -> Option<char>` and `is_jhal(char) -> bool` (`tinanta/sound.rs`, already imported in `tripadi.rs`); `ANGA` (`tinanta/terms.rs`, already imported); `with_slots(Vec<Term>) -> Vec<Term>` (prepends empty `AGAMA` and `ABHYASA`); `curadi_row(&'static str, &'static str, PadaAssignment) -> Dhatu` and `sole(Vec<Prakriya>) -> Prakriya` in `derivation_tests.rs`; `credited(&str) -> Vec<(&'static str, Gana)>` in `tests/trace/helpers.rs`, one entry per live branch crediting the id, in `dhatus()` order.
- Produces: the tests `coh_kuh_reads_only_a_terms_final_cu_before_an_affix`, `picc_keeps_its_root_internal_cc`, `coh_kuh_is_credited_on_exactly_its_twelve_rows`. Task 3 extends the last.

- [ ] **Step 1: The failing tests**

Create `/tmp/vidyut-full/slice10m/tests_10m.py` if it is missing (sha256 `b06eb0d8ff75ea50ee0151540cc73623ea1b877781fc950613ded8d2cd2c351a`). It adds:
- **`tripadi.rs`:** `coh_kuh_reads_only_a_terms_final_cu_before_an_affix`, after `coh_kuh_fires_only_word_finally_or_before_a_jhal`. Four hand-built cases: `piccay|a|ti` declines (the term-internal `cc`); `pac|anaj|ti` → `pacanagti` (an earlier non-qualifying cu does not hide a later one); `vac|(empty)|ti` → `vakti` (the next NON-EMPTY term); and `(empty)|jaj|da|a|ti` declines (an abhyāsa-final cu before the dhātu, the `ANGA` guard's only witness).
- **`derivation_tests.rs`:** `picc_keeps_its_root_internal_cc`, a hand-built `10.0175` deriving *piccayati* / *piccayate* with neither 8.2.30 nor 7.3.86 credited.
- **`trace/curadi.rs`:** `coh_kuh_is_credited_on_exactly_its_twelve_rows`, 8.2.30's live-branch credits per row, sorted. This one passes on main and must keep passing: it is the guard on the narrowing.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10m, Task 2's failing tests — 8.2.30's term-final unit
cases, the hand-built `picc` derivation, and the corpus-wide 8.2.30 roster.
Every `old` must occur exactly once in its file; nothing is written if one
fails. Run from the worktree root."""
import sys
E = {
'crates/panini-prakriya/src/tinanta/tripadi.rs': [
("""        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "Banjanti");
    }

""",
"""        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "Banjanti");
    }

    /// 8.2.30 reads a term's FINAL sound only, and the term after it must be
    /// an affix or āgama: the cu has to stand at a morpheme's end. vidyut
    /// fires it per term the same way. Main's whole-word scan velarised
    /// `picc`'s first `c` before the second (*pikcayati*); these pin the
    /// narrowing, each on a hand-built prakriyā.
    #[test]
    fn coh_kuh_reads_only_a_terms_final_cu_before_an_affix() {
        let rule = rules().find(|r| r.id == "8.2.30").unwrap();

        // a cu inside a term declines even before a jhal: √picc's real
        // tripādī intermediate, ṇic folded into the aṅga by 3.1.32 and made
        // `ay` by 6.1.78, then śap.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("piccay"), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "piccayati");

        // an earlier term-final cu that does not qualify (a vowel follows)
        // does not hide a later one that does, and only the later one moves.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("pac"), Term::new("anaj"), Term::new("ti")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "pacanagti");

        // an empty term is skipped: the jhal that conditions the cu is the
        // first sound of the next NON-EMPTY term (adādi's luk'd śap).
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("vac"), Term::new(""), Term::new("ti")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "vakti");

        // the dhātu is not an affix: an abhyāsa ending in a cu before an aṅga
        // beginning with a jhal declines. No abhyāsa in the corpus ends in a
        // consonant (7.4.60 halādiḥ śeṣaḥ), so this is the guard's only
        // witness.
        let mut p = Prakriya {
            terms: vec![
                Term::new(""),
                Term::new("jaj"),
                Term::new("da"),
                Term::new("a"),
                Term::new("ti"),
            ],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "jajdaati");
    }

"""),
],
'crates/panini-prakriya/src/tinanta/derivation_tests.rs': [
("""        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids.contains(&"8.2.78"), credits_8_2_78, "{number}");
        assert_eq!(ids.contains(&"8.2.18"), number == "10.0278", "{number}");
    }
}
""",
"""        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids.contains(&"8.2.78"), credits_8_2_78, "{number}");
        assert_eq!(ids.contains(&"8.2.18"), number == "10.0278", "{number}");
    }
}

#[test]
fn picc_keeps_its_root_internal_cc() {
    // Slice 10m's `10.0175 picca~`, hand-built, laṭ prathama eka in both
    // padas: vidyut-prakriya's forms. The first `c` stands before a jhal (the
    // second `c`) but not at a term's end, so 8.2.30 declines; the guru
    // upadhā (`cc`) leaves 7.3.86 nothing to do.
    let row = curadi_row("10.0175", "picc", PadaAssignment::Nic);
    for (pada, form) in [
        (Pada::Parasmaipada, "piccayati"),
        (Pada::Atmanepada, "piccayate"),
    ] {
        let p = sole(derive(
            &row,
            Lakara::Lat,
            pada,
            Purusha::Prathama,
            Vacana::Eka,
        ));
        assert_eq!(p.text(), form);
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert!(!ids.contains(&"8.2.30"), "{form}: {ids:?}");
        assert!(!ids.contains(&"7.3.86"), "{form}: {ids:?}");
    }
}
"""),
],
'crates/panini/tests/trace/curadi.rs': [
("""
#[test]
#[allow(non_snake_case)]
fn vicCayati_and_vicCAyati_take_tuk_before_3_1_32() {
""",
"""
#[test]
fn coh_kuh_is_credited_on_exactly_its_twelve_rows() {
    // Slice 10m narrowed 8.2.30 to a term-final cu before an affix or āgama,
    // or pada-final. Goldens ignore traces, so this holds its credits to the
    // rows it fired on before the narrowing, branch for branch: juhotyādi's
    // √ṇij and √vij, and rudhādi's ten cu-final roots (30 live branches where
    // both padas derive, 21 where only parasmaipada does). No curādi row
    // credits it.
    let mut got: Vec<(&str, usize)> = Vec::new();
    for (number, _) in credited("8.2.30") {
        match got.last_mut() {
            Some((n, c)) if *n == number => *c += 1,
            _ => got.push((number, 1)),
        }
    }
    got.sort();
    assert_eq!(
        got,
        [
            ("03.0012", 30),
            ("03.0013", 30),
            ("07.0004", 30),
            ("07.0005", 30),
            ("07.0007", 30),
            ("07.0016", 21),
            ("07.0017", 30),
            ("07.0021", 21),
            ("07.0022", 21),
            ("07.0023", 21),
            ("07.0024", 21),
            ("07.0025", 21),
        ]
    );
}

#[test]
#[allow(non_snake_case)]
fn vicCayati_and_vicCAyati_take_tuk_before_3_1_32() {
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
python3 /tmp/vidyut-full/slice10m/tests_10m.py      # applied 3 edits to 3 files
mise run fmt
```

- [ ] **Step 2: Run them to see them fail**

```bash
mise exec -- cargo test -q -p panini-prakriya coh_kuh 2>&1 | grep -E "^test .*FAILED|panicked|test result"
mise exec -- cargo test -q -p panini-prakriya picc_keeps 2>&1 | grep -E "panicked|left|right|test result"
mise exec -- cargo test -q -p panini --test trace coh_kuh_is_credited 2>&1 | grep -E "panicked|test result"
```

Expected:
- `coh_kuh_reads_only_a_terms_final_cu_before_an_affix` FAILS at its first assertion (main's 8.2.30 fires on `piccayati`); the old `coh_kuh_fires_only_word_finally_or_before_a_jhal` passes.
- `picc_keeps_its_root_internal_cc` FAILS with `left: "pikcayati"`, `right: "piccayati"`.
- `coh_kuh_is_credited_on_exactly_its_twelve_rows` PASSES (it pins main's credits).

- [ ] **Step 3: The engine**

Create `/tmp/vidyut-full/slice10m/engine_10m.py` if it is missing (sha256 `b11347677b4a7580f938f5f5934f40444f6a22b342d625461c885612bb1cd2b7`). It rewrites 8.2.30's `apply` and its comment block. It also fixes three comments that described the old scan: 8.2.31's guard paragraph, the `word_chars` paragraph in the 8.2.41 section, and the old unit test's doc and first-case comment.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10m, Task 2's engine — 8.2.30 reads a term-final cu
before an affix or āgama, or pada-final, and its comment block says so.
Every `old` must occur exactly once in its file; nothing is written if one
fails. Run from the worktree root."""
import sys
E = {
'crates/panini-prakriya/src/tinanta/tripadi.rs': [
("""    // 8.2.30 coH kuH: a cu stop (c C j J) is replaced by its ku counterpart
    // (the nearest velar by 1.1.50 sthāne'ntaratamaḥ, so voicing and
    // aspiration are preserved) when it is either word-final or immediately
    // followed by a jhal. Banaj + ti -> Banag + ti (before the jhal `t`,
    // then 8.4.55 khari ca devoices to Banakti); aBanaj -> aBanag
    // word-finally.
""",
"""    // 8.2.30 coH kuH: a cu stop (c C j J) is replaced by its ku counterpart
    // (the nearest velar by 1.1.50 sthāne'ntaratamaḥ, so voicing and
    // aspiration are preserved) when it ends a term and either the next
    // non-empty term is an affix or āgama beginning with a jhal, or nothing
    // follows it in the pada. Banaj + ti -> Banag + ti (before the jhal `t`,
    // then 8.4.55 khari ca devoices to Banakti); aBanaj -> aBanag
    // word-finally.
"""),
("""    // The 1.1.50 sthAne'ntaratamaH account above is therefore a description
    // of this code, not only of the sūtra.
    //
    // Read via `word_chars`, not a term-boundary check: the target `j` sits
    // at the END of a non-final term (śnam's infix leaves the root's own
    // tail — the `j` — in `SHAP`, one term short of the actual word end,
    // e.g. `Ba | naj | ti`), so the jhal that conditions it can be the
    // FIRST character of the NEXT term rather than anything in the bearing
    // term itself. `word_chars` already flattens exactly this cross-term
    // adjacency for the same reason 8.3.24 further down this array reads
    // it. Word-final falls
    // out of the same scan for free — there is simply no next entry to test
    // (`w.get(i + 1)` is `None`) after 8.2.23 has eaten tip/sip's own letter,
    // leaving `ENDING` empty and the dhātu's `j` as the last entry
    // `word_chars` reports.
    //
    // The word-final / jhal test lives INSIDE the search, not after it, the
    // way 8.3.24's and 8.4.58's own searches further down this array do: the
    // rule finds the first `j` that is genuinely word-final or jhal-followed
    // rather than the first `j` in the word full stop, so a non-applicable
    // `j` earlier in the word can never hide a later, applicable one.
    // No cell in this suite has two cu sounds to distinguish: √ric and √vic
    // each carry exactly one `c`, no curated root mixes a `c` with a `j`,
    // √ji, √juṣ and √vij always present theirs before a vowel, and the
    // weak-stem cells of √bhañj, √yuj, √ric and √vic (`Banjanti`,
    // `yuYjanti`, `riYcanti`) decline for the same reason.
    // The scan is therefore deliberately NOT narrowed to √bhañj's known
    // position: this is hardening against an ordering no witness here
    // exercises, not a fix for one observed.
""",
"""    // The 1.1.50 sthAne'ntaratamaH account above is therefore a description
    // of this code, not only of the sūtra.
    //
    // The rule reads TERMS, the way vidyut-prakriya's does: the cu must be a
    // term's LAST sound. Slice 10m narrowed it from a whole-word scan, which
    // velarised any cu before a jhal and so turned curādi `picc`'s first `c`
    // (before the second) into `k` (*pikcayati* for *piccayati*). A root-
    // internal cu is not at a morpheme's end, and the sūtra's jhal is the
    // initial of what follows the term. The test
    // `coh_kuh_reads_only_a_terms_final_cu_before_an_affix` pins the case.
    //
    // The jhal is the first sound of the next NON-EMPTY term: śnam's infix
    // leaves the root's own tail — the `j` — in `SHAP` (`Ba | naj | ti`), so
    // the conditioning `t` is `ENDING`'s first sound, and adādi's luk'd śap
    // leaves an empty `SHAP` between a root and its ending. Pada-final is the
    // case where no non-empty term follows at all: 8.2.23 has eaten tip/sip's
    // own letter, leaving `ENDING` empty and the dhātu's `j` last.
    //
    // That next term must be an affix or āgama, as vidyut requires. This
    // engine does not tag terms by kind, but its slots fix it: every term
    // after `ANGA` is one, and the only other term that can follow is `ANGA`
    // itself, the dhātu, after `AGAMA` or `ABHYASA`. Hence `j != ANGA`. No
    // abhyāsa or aṭ in the corpus ends in a consonant (7.4.60 halādiḥ
    // śeṣaḥ), so the guard has one witness, the hand-built one in the test
    // above, and no golden.
    //
    // The test lives INSIDE the search, not after it, the way 8.3.24's and
    // 8.4.58's own searches further down this array do: the rule finds the
    // first term whose final cu qualifies, so a term-final cu earlier in the
    // word that does not (a vowel follows) never hides a later one that does.
"""),
("""        apply: |p| {
            let w = word_chars(p);
            let Some(pos) = w.iter().enumerate().position(|(i, (_, _, c))| {
                kutva_of(*c).is_some() && w.get(i + 1).is_none_or(|(_, _, next)| is_jhal(*next))
            }) else {
                return false;
            };
            let (term, idx, found) = w[pos];
            let Some(to) = kutva_of(found) else {
                return false;
            };
            let before = p.snapshot();
            set_char(p, term, idx, to);
            p.record("8.2.30", "coH kuH", before);
""",
"""        apply: |p| {
            let hit = (0..p.terms.len()).find_map(|i| {
                let to = kutva_of(p.terms[i].text.chars().last()?)?;
                let next = (i + 1..p.terms.len()).find(|&j| !p.terms[j].text.is_empty());
                let qualifies = match next {
                    Some(j) => j != ANGA && p.terms[j].text.chars().next().is_some_and(is_jhal),
                    None => true,
                };
                qualifies.then(|| (i, p.terms[i].text.chars().count() - 1, to))
            });
            let Some((term, idx, to)) = hit else {
                return false;
            };
            let before = p.snapshot();
            set_char(p, term, idx, to);
            p.record("8.2.30", "coH kuH", before);
"""),
("""    // 8.2.31 ho ḍhaḥ: `h` becomes `Q` (ḍh). The *jhali* and *padasya*
    // conditions come by anuvṛtti from the same place 8.2.30 coH kuH reads
    // them, so the guard is written the same way — find the first `h` that
    // is genuinely word-final or jhal-followed, rather than the first `h`
    // in the word, so a non-applicable `h` earlier can never hide a later
    // applicable one.
""",
"""    // 8.2.31 ho ḍhaḥ: `h` becomes `Q` (ḍh). The *jhali* and *padasya*
    // conditions come by anuvṛtti from the same place 8.2.30 coH kuH reads
    // them. The guard is the whole-word scan 8.2.30 used until slice 10m
    // narrowed that rule alone to a term-final cu — find the first `h` that
    // is genuinely word-final or jhal-followed, rather than the first `h`
    // in the word, so a non-applicable `h` earlier can never hide a later
    // applicable one.
"""),
("""    // Read via `word_chars`, not a term-boundary check, for the same reason
    // 8.2.30/8.4.41 do: śnam's infix leaves √piṣ's own tail — the `z` — at
""",
"""    // Read via `word_chars`, not a term-boundary check, for the same reason
    // 8.4.41 does: śnam's infix leaves √piṣ's own tail — the `z` — at
"""),
("""    /// 8.2.30 velarises a cu sound that is word-final or immediately followed
    /// by a jhal, and declines otherwise. Both reachable arms are pinned
""",
"""    /// 8.2.30 velarises a term-final cu sound that is pada-final or followed
    /// by a jhal-initial affix, and declines otherwise. Both reachable arms are pinned
"""),
("""        // before a jhal: the `j` sits at the end of a non-final term (śnam's
        // infix leaves it in SHAP), and the jhal that conditions it is the
        // first character of the term after — the cross-term adjacency
        // `word_chars` exists for.
""",
"""        // before a jhal: the `j` sits at the end of a non-final term (śnam's
        // infix leaves it in SHAP), and the jhal that conditions it is the
        // first character of the term after.
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
python3 /tmp/vidyut-full/slice10m/engine_10m.py      # applied 7 edits to 1 files
mise run fmt
```

The new `apply`, for reference (the script writes it):

```rust
        apply: |p| {
            let hit = (0..p.terms.len()).find_map(|i| {
                let to = kutva_of(p.terms[i].text.chars().last()?)?;
                let next = (i + 1..p.terms.len()).find(|&j| !p.terms[j].text.is_empty());
                let qualifies = match next {
                    Some(j) => j != ANGA && p.terms[j].text.chars().next().is_some_and(is_jhal),
                    None => true,
                };
                qualifies.then(|| (i, p.terms[i].text.chars().count() - 1, to))
            });
            let Some((term, idx, to)) = hit else {
                return false;
            };
            let before = p.snapshot();
            set_char(p, term, idx, to);
            p.record("8.2.30", "coH kuH", before);
            true
        },
```

`word_chars` keeps its other callers (8.3.24, 8.4.41, 8.2.41 and the rest), so its import stays.

- [ ] **Step 4: Run the full suite**

```bash
mise run lint
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS, with `panini-prakriya` 447, `trace` 217, `paradigm` 30, `panini-data` 28.

- [ ] **Step 5: Commit**

```bash
git branch --show-current      # curadi-10m
git add -A
git commit -m "feat(engine): 8.2.30 coH kuH reads a term-final cu before an affix

The whole-word scan velarised any cu before a jhal, so curādi picc's
root-internal cc became kc (pikcayati). 8.2.30 now reads terms, as
vidyut-prakriya does: a term's last sound, before a jhal-initial affix or
āgama (never the dhātu itself), or pada-final. Every prior credit is
unchanged, pinned row by row by coh_kuh_is_credited_on_exactly_its_twelve_rows."
```

---

## Task 3: The row, its goldens, and every assertion it moves

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`, `crates/panini/tests/paradigm/data/curadi.rs`
- Modify: `crates/panini/tests/trace/curadi.rs`

**Interfaces:**
- Consumes: Task 2's narrowed 8.2.30 and `coh_kuh_is_credited_on_exactly_its_twelve_rows`; `Panini::check(&str) -> CheckResult { verdict, analyses }`, each `Analysis { dhatu: String, pada: Pada, trace: Vec<RuleStep> }`.
- Produces: the `10.0175` `Dhatu` row; the test `curadi_analyses_its_10m_forms`.

- [ ] **Step 1: The assertions (failing)**

Create `/tmp/vidyut-full/slice10m/assertions_10m.py` if it is missing (sha256 `4f0123cd1f042e6f8db9f598323a319ec6d5c4c4a981a995df050236248c881a`). By file:
- **`panini-data`:** `Dhatu::pada`'s census (594; 428 `Nic`) and the marker doc's 594; the row count (594); the row-list test, renamed `curadi_rows_are_the_four_hundred_ninety_one_curated_roots`, with a 10m sentence and the tuple.
- **`paradigm/main.rs`:** the census (38232 / 4248; ones 29768, twos 6790, threes 927; `ALTERNATES` 11672; `8.4.56` / `7.1.35` / `7.1.35+8.4.56` 898 / 890 / 890), its 10m paragraph and the three-form message; `curadi_analyses_its_10m_forms`.
- **`trace/curadi.rs`:** the 1.3.74 count (428) and its comment; `coh_kuh_is_credited_on_exactly_its_twelve_rows` asserts `10.0175` is curated.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10m, Task 3's assertions — every count, roster, list and
check() assertion `10.0175 picca~` moves, before the row exists.
Every `old` must occur exactly once in its file; nothing is written if one
fails. Run from the worktree root."""
import sys
E = {
'crates/panini-data/src/lib.rs': [
("""    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 593
    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and
    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed
    /// exception, 427 curādi rows' are 1.3.74's, seven more 1.3.74's with ṇic
""",
"""    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 594
    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and
    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed
    /// exception, 428 curādi rows' are 1.3.74's, seven more 1.3.74's with ṇic
"""),
("""    /// The test covers the 593 roots curated here, not the dhātupāṭha's 2259.
""",
"""    /// The test covers the 594 roots curated here, not the dhātupāṭha's 2259.
"""),
("""        assert_eq!(dhatus().len(), 593);
""",
"""        assert_eq!(dhatus().len(), 594);
"""),
("""    fn curadi_rows_are_the_four_hundred_ninety_curated_roots() {
""",
"""    fn curadi_rows_are_the_four_hundred_ninety_one_curated_roots() {
"""),
("""        // ṇic optional by 2570) and √kṛp (8.2.18), all ubhayapadī by 1.3.74.
        // The gaṇa is OPEN at 490 of its 509 dhātupāṭha rows.
""",
"""        // ṇic optional by 2570) and √kṛp (8.2.18), all ubhayapadī by 1.3.74.
        // Slice 10m adds √picc, whose root-internal `cc` 8.2.30 no longer
        // velarises, ubhayapadī by 1.3.74. The gaṇa is OPEN at 491 of its 509
        // dhātupāṭha rows.
"""),
("""                ("10.0278", "kfp", PadaAssignment::Nic),
            ]
""",
"""                ("10.0278", "kfp", PadaAssignment::Nic),
                ("10.0175", "picc", PadaAssignment::Nic),
            ]
"""),
("""    /// vendored upadeśa: 73 of the 593 curated roots carry a `\\` at all, and 52
""",
"""    /// vendored upadeśa: 73 of the 594 curated roots carry a `\\` at all, and 52
"""),
],
'crates/panini/tests/paradigm/main.rs': [
("""/// 576 new cells, 90 new rows. The gaṇa is OPEN at 490 of its 509 rows.
/// This test is what keeps the numbers true day to day.
""",
"""/// 576 new cells, 90 new rows. The gaṇa is OPEN at 490 of its 509 rows.
///
/// Slice 10m curates √picc, `Nic`, which forks exactly where √cur does, once
/// 8.2.30 stopped velarising its root-internal `cc`. 72 new cells, 6 new
/// rows. The gaṇa is OPEN at 491 of its 509 rows.
/// This test is what keeps the numbers true day to day.
"""),
("""    assert_eq!(total_cells, 38160, "4240 root×lakāra blocks × 9 cells each");
""",
"""    assert_eq!(total_cells, 38232, "4248 root×lakāra blocks × 9 cells each");
"""),
("""    assert_eq!(ones, 29700, "one-form cells");
    assert_eq!(twos, 6788, "two-form cells");
    assert_eq!(
        threes, 925,
""",
"""    assert_eq!(ones, 29768, "one-form cells");
    assert_eq!(twos, 6790, "two-form cells");
    assert_eq!(
        threes, 927,
"""),
("""         slice 10l — seven of its eight rows' the same way"
""",
"""         slice 10l — seven of its eight rows' the same way; and — new in slice 10m — √picc's"
"""),
("""    assert_eq!(ALTERNATES.len(), 11666, "ALTERNATES row count");
""",
"""    assert_eq!(ALTERNATES.len(), 11672, "ALTERNATES row count");
"""),
("""    assert_eq!(key_count("8.4.56"), 896, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 888, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 888, "7.1.35+8.4.56 alternates");
""",
"""    assert_eq!(key_count("8.4.56"), 898, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 890, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 890, "7.1.35+8.4.56 alternates");
"""),
("""        "Drasayati",
        "DrAsati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
""",
"""        "Drasayati",
        "DrAsati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}

/// Slice 10m's `check()` witnesses: √picc's laṭ prathama eka in each pada,
/// each with exactly one analysis (the goldens were grepped first), crediting
/// ṇic and neither 8.2.30 nor 7.3.86. The form main's whole-word 8.2.30 gave
/// (*pikcayati*) and a guṇated one derive nothing.
#[test]
fn curadi_analyses_its_10m_forms() {
    let engine = Panini::new();
    for (form, pada) in [
        ("piccayati", Pada::Parasmaipada),
        ("piccayate", Pada::Atmanepada),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), 1, "{form}");
        let a = &r.analyses[0];
        assert_eq!(a.dhatu, "picc", "{form}");
        assert_eq!(a.pada, pada, "{form}");
        let ids: Vec<&str> = a.trace.iter().map(|s| s.sutra.as_str()).collect();
        assert!(ids.contains(&"3.1.25"), "{form}: {ids:?}");
        assert!(!ids.contains(&"8.2.30"), "{form}: {ids:?}");
        assert!(!ids.contains(&"7.3.86"), "{form}: {ids:?}");
    }
    for form in ["pikcayati", "pikcayate", "peccayati"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
"""),
],
'crates/panini/tests/trace/curadi.rs': [
("""    // 427 `Nic` rows and the seven `NicUbhayapada` rows' ṇic branch, read from
""",
"""    // 428 `Nic` rows and the seven `NicUbhayapada` rows' ṇic branch, read from
"""),
("""    assert_eq!(nic.len(), 427, "curated 1.3.74 rows");
""",
"""    assert_eq!(nic.len(), 428, "curated 1.3.74 rows");
"""),
("""    // credits it.
    let mut got: Vec<(&str, usize)> = Vec::new();
""",
"""    // credits it, `10.0175 picca~` included, whose `cc` is root-internal.
    assert!(dhatus().iter().any(|d| d.dhatupatha == "10.0175"));
    let mut got: Vec<(&str, usize)> = Vec::new();
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
python3 /tmp/vidyut-full/slice10m/assertions_10m.py      # applied 17 edits to 3 files
mise run fmt
```

- [ ] **Step 2: Run them to see them fail**

```bash
mise exec -- cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .* FAILED|test result: FAILED"
```

Foreground, timeout 600000 ms. Expected: exactly six failures, two each in three binaries:
- `paradigm`: `curadi_analyses_its_10m_forms`, `derivation_set_shape_matches_the_audited_numbers`
- `trace`: `curadi::coh_kuh_is_credited_on_exactly_its_twelve_rows`, `curadi::a_kusmad_is_credited_on_exactly_the_akusmiya_cells`
- `panini-data`: `tests::curated_roots_have_expected_ganas_and_padas`, `tests::curadi_rows_are_the_four_hundred_ninety_one_curated_roots`

- [ ] **Step 3: The row**

Create `/tmp/vidyut-full/slice10m/rows_10m.py` if it is missing (sha256 `911af30eb4d088bba9447d00ec4aed54980d8020f66d274bfe870e89e597475d`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10m, Task 3's row — `10.0175 picca~`, appended to
`DHATUS` after slice 10l's block. Run from the worktree root."""
import sys
p = 'crates/panini-data/src/lib.rs'
old = """        // root. Ubhayapadī by 1.3.74. Slice 10l.
        dhatupatha: "10.0278",
        code: "kfp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "avakalkane",
    },
];
"""
new = """        // root. Ubhayapadī by 1.3.74. Slice 10l.
        dhatupatha: "10.0278",
        code: "kfp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "avakalkane",
    },
    Dhatu {
        // 10.0175 `picca~` kuwwane (√picc). Guru upadhā (the conjunct `cc`),
        // so unchanged before ṇic. 8.2.30 coH kuH declines on the first `c`:
        // it stands before a jhal, the second `c`, but inside the root, not
        // at a term's end (*piccayati*, not *pikcayati*). Ubhayapadī by
        // 1.3.74. Slice 10m.
        dhatupatha: "10.0175",
        code: "picc",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kuwwane",
    },
];
"""
s = open(p).read()
if s.count(old) != 1:
    sys.exit(f"not applied: {s.count(old)}x the 10l block's end")
open(p, 'w').write(s.replace(old, new))
print("appended 1 row")
```

```bash
python3 /tmp/vidyut-full/slice10m/rows_10m.py      # appended 1 row
```

- [ ] **Step 4: Generate the goldens**

Create `/tmp/vidyut-full/vidyut-prakriya/examples/curadi_goldens_10m.rs` if it is missing (sha256 `853caf5233b20a5292b45f2139475d22813862a822c1b906e287d86dd4d2020a`). It is 10l's generator with the row selection changed: it derives every cell of the row in both engines, asserts the form sets equal, and writes the rows the statics want (pinned form = first live branch; each further distinct form an `ALTERNATES` row keyed by its branch's vikalpa ids in log order).

```rust
//! THROWAWAY: slice 10m — emit √picc's goldens from the engine,
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
    // The new row.
    const NEW: [&str; 1] = ["10.0175"];
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
    std::fs::write("/tmp/vidyut-full/goldens_10m_paradigm.rs", par).unwrap();
    std::fs::write("/tmp/vidyut-full/goldens_10m_alternates.rs", alt).unwrap();
    println!("{ncells} cells, {nforms} forms, {ndiff} differences");
    assert_eq!(ndiff, 0);
}
```

```bash
WT="$(git rev-parse --show-toplevel)"
V=/tmp/vidyut-full/vidyut-prakriya
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml      # must point at $WT/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example curadi_goldens_10m 2>/dev/null | tail -1)
sha256sum /tmp/vidyut-full/goldens_10m_paradigm.rs /tmp/vidyut-full/goldens_10m_alternates.rs
```

Expected: `72 cells, 78 forms, 0 differences`, then:
- `8e1e993e33b3d02d4ef72f00b4a4cfb5c82ae9bb1ae6cd2240f8e929390db9f1  /tmp/vidyut-full/goldens_10m_paradigm.rs`
- `d2cdf48b0e1995cf617eb527c165200e86ce6c50b0bf4a68f89117a9a19543a3  /tmp/vidyut-full/goldens_10m_alternates.rs`

Leave the dev-deps pointing at this worktree; Task 4 uses them. If a line differs, stop and report.

- [ ] **Step 5: Insert the goldens**

Create `/tmp/vidyut-full/slice10m/insert_goldens_10m.py` if it is missing (sha256 `ba6781ecdc35a977403801438f44a2ca54724cb414eab7191031030ab1248535`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10m — insert the generator's goldens before each
static's `];`. Run from the worktree root."""
p = 'crates/panini/tests/paradigm/data/curadi.rs'
s = open(p).read()
par = open('/tmp/vidyut-full/goldens_10m_paradigm.rs').read()
alt = open('/tmp/vidyut-full/goldens_10m_alternates.rs').read()
i = s.index('pub const ALTERNATES')
head, tail = s[:i], s[i:]
k = head.rindex('];'); head = head[:k] + par + head[k:]
k = tail.rindex('];'); tail = tail[:k] + alt + tail[k:]
open(p, 'w').write(head + tail)
print("inserted goldens")
```

```bash
python3 /tmp/vidyut-full/slice10m/insert_goldens_10m.py      # inserted goldens
mise run fmt
```

- [ ] **Step 6: The measured pada-ambiguous set**

The set is measured, never hand-picked: run the test against the old set and read the real one off its failure. Create `/tmp/vidyut-full/slice10m/pin_ambiguous_10m.py` if it is missing (sha256 `8e89c9e48c8c2ee7a252afe8fd52e01bda9ea61b0216a10cb8366fc7e0ac839b`). It refuses a set whose size or hash differs from the prototype's.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10m — pin the measured pada-ambiguous set ($SET, the
failing test's `left:` JSON) into pada_ambiguous_surfaces_are_exactly_these,
and extend the comment that accounts for it. Run from the worktree root."""
import hashlib, json, os
amb = json.load(open(os.environ['SET']))
assert len(amb) == 1635, len(amb)
h = hashlib.sha256('\n'.join(amb).encode()).hexdigest()
assert h == '2498d97096840e09f25e6ce91074c9b6dfe8241c4f2011ad013dbbbf298d629e', h
p = 'crates/panini/tests/paradigm/main.rs'
s = open(p).read()
old = """    // from 1603 to 1631.
"""
assert s.count(old) == 1
s = s.replace(old, old + """    // Slice 10m's √picc contributes the same four (`apiccayata`,
    // `piccayatAm`, `piccayetAm`, `piccayeta`), taking the set from 1631 to
    // 1635.
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
SET="$SET" python3 /tmp/vidyut-full/slice10m/pin_ambiguous_10m.py      # pinned 1635
mise run fmt
```

The four new surfaces are `apiccayata`, `piccayatAm`, `piccayetAm` and `piccayeta`, the same `-ayata` / `-ayatAm` / `-ayetAm` / `-ayeta` four every ubhayapadī curādi row adds.

- [ ] **Step 7: Run the full suite**

```bash
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS at 38232 cells, with `panini-prakriya` 447, `trace` 217, `paradigm` 31, `panini-data` 28.

Grep the goldens for the `check()` witnesses, which the test also enforces:

```bash
for f in piccayati piccayate pikcayati pikcayate peccayati; do
  printf "%s: %s\n" $f "$(grep -c "\"$f\"" crates/panini/tests/paradigm/data/*.rs | grep -v ':0' | sed 's#.*/##' | tr '\n' ' ')"; done
```

Expected: `piccayati: curadi.rs:1` and `piccayate: curadi.rs:1`; the three Invalid shapes print nothing.

- [ ] **Step 8: Commit**

```bash
mise run lint
git branch --show-current      # curadi-10m
git add -A
git commit -m "feat(data): curādi's √picc (10.0175 picca~)

38160 → 38232 cells, 49826 → 49904 forms, ALTERNATES 11666 → 11672, 593 →
594 roots; pada-ambiguous surfaces 1631 → 1635. Its root-internal cc keeps
its c now that 8.2.30 reads a term-final cu (piccayati). Goldens generated
cell-by-cell equal to vidyut."
```

---

## Task 4: Audit, prior-trace diff, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`
- Modify: `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`
- Modify: `crates/panini/tests/paradigm/main.rs` (doc counts, audit prose), `crates/panini-prakriya/src/tinanta/guna.rs` (one comment), `crates/panini-prakriya/src/tinanta/tripadi.rs` (one comment in the 8.4.40 section)
- Modify: `docs/superpowers/specs/2026-10-05-curadi-gana-10l-design.md`

**Interfaces:**
- Consumes: the finished engine, data and goldens. Produces no symbols.

- [ ] **Step 1: Update the audit harness**

Create `/tmp/vidyut-full/slice10m/audit_10m.py` if it is missing (sha256 `f7f7f5fabcb144a27651a9e8a1214ad7705b35c3e6f2609d1d3175bc626a5f18`):

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10m's edits to tools/audit/panini_full_audit.rs.
Every `old` must occur exactly once; nothing is written if one fails.
Run from the worktree root."""
import sys
p = 'tools/audit/panini_full_audit.rs'
E = [
("//! What it compares: for each of the 593 curated roots, for each pada the root\n",
 "//! What it compares: for each of the 594 curated roots, for each pada the root\n"),
("//! admits (`Dhatu::padas`; two apiece for the 467 roots that admit both padas —\n",
 "//! admits (`Dhatu::padas`; two apiece for the 468 roots that admit both padas —\n"),
("//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 427 curādi\n",
 "//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, 428 curādi\n"),
("//! Corpus invariants, asserted: 593 roots, 38160 cells, 49826 forms. These are\n",
 "//! Corpus invariants, asserted: 594 roots, 38232 cells, 49904 forms. These are\n"),
("//! (`derivation_set_shape_matches_the_audited_numbers`): 4240 root×pada×lakāra\n//! blocks × 9 cells, plus 11666 `ALTERNATES` rows. If this harness's\n",
 "//! (`derivation_set_shape_matches_the_audited_numbers`): 4248 root×pada×lakāra\n//! blocks × 9 cells, plus 11672 `ALTERNATES` rows. If this harness's\n"),
("//! Optionally dump the full 38160-cell table:\n",
 "//! Optionally dump the full 38232-cell table:\n"),
("""    assert_eq!(roots_seen.len(), 593, "curated roots");
    assert_eq!(n_cells, 38160, "cells: 4240 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 49826, "forms: 38160 cells + 11666 ALTERNATES rows");
""",
"""    assert_eq!(roots_seen.len(), 594, "curated roots");
    assert_eq!(n_cells, 38232, "cells: 4248 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 49904, "forms: 38232 cells + 11672 ALTERNATES rows");
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

```bash
python3 /tmp/vidyut-full/slice10m/audit_10m.py      # applied 7 edits to 1 files
```

The harness now asserts 594 / 38232 / 49904 and names 468 both-pada roots, 428 of them `Nic`.

- [ ] **Step 2: The prior-trace diff and the audit**

The dev-deps still point at this worktree from Task 3. Create `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_10m.rs` if it is missing (sha256 `d21014181cf52ce4bd3b5d893f80cf5814e048621ee2efaa947b24e0bd431b15`). It lists 10m's row literally so that it also builds against main, and it dumps every branch, blocked ones included, with every step's before and after text:

```rust
//! THROWAWAY: slice 10m — dump every prior cell's branches, blocked ones
//! included, each with its credited-rule log and every step's before/after.
use panini::Panini;
use panini_data::{Lakara as L, Purusha as P, Vacana as V};
/// Slice 10m's row, listed literally so the dump also builds against main.
const NEW: &[&str] = &["10.0175"];
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
WT="$(git rev-parse --show-toplevel)"
DUMP="$(mktemp -d)"
V=/tmp/vidyut-full/vidyut-prakriya
grep -n '^panini' $V/Cargo.toml   # must point at $WT/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10m 2>/dev/null > "$DUMP/branch.txt")
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml   # must point at /workspace/crates
git -C /workspace branch --show-current   # main
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10m 2>/dev/null > "$DUMP/main.txt")
wc -l < "$DUMP/main.txt"; wc -l < "$DUMP/branch.txt"            # 56378 and 56378
grep -c "blocked=false" "$DUMP/main.txt"                        # 49826
cmp "$DUMP/main.txt" "$DUMP/branch.txt" && echo PRIOR-TRACES-IDENTICAL
```

Expected: `56378`, `56378`, `49826`, `PRIOR-TRACES-IDENTICAL`. This is the corpus-wide check that the narrowing changes no prior root's trace, step text included, blocked branches included (goldens ignore traces). `/workspace` must be on `main` (Task 1) for the second dump. If `cmp` reports a difference, stop and report.

Then the audit. Repoint at this worktree again, copy the committed harness (never rewrite it), and run:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
cp tools/audit/panini_full_audit.rs $V/examples/
(cd $V && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -2)
(cd $V && PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -8)
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml
date -u +%F
```

- Expected from the `entry` control, run first: `AUDIT FAILED: 36 differing cells.`
- Expected from the honest run: `roots : 594`, `cells : 38232`, `forms (set sizes): 49904`, `live branches : 49904`, `blocked branches : 6552`, `differing cells  : 0`, `AUDIT PASSED: 38232 cells, 49904 forms, zero differences.`

Do not use `mise -C`. If the honest run shows differences, stop and report, and edit nothing. Write down the date the last command prints: it is the audit's date for Step 3.

- [ ] **Step 3: The doc sweep, the audit record and the spec pointer**

Create `/tmp/vidyut-full/slice10m/docsweep_10m.py` if it is missing (sha256 `ca0273e25877fc8945d09701c5eaa2fcbbed9d17d1ff8b5b9a8346d28a8712c6`). It takes the audit's date (`AUDIT_DATE`, Step 2's `date -u +%F`) and writes it into the audit record and AGENTS.md's audit chain. It covers:
- **README:** curādi 491; a 10m sentence after 10l's; 594 roots; 8464 of 38232 multi-form cells (6790 two, 927 three); √picc in the three-form and both-pada lists; 468 both-pada roots; 1635 pada-ambiguous surfaces and √picc's four.
- **ARCHITECTURE:** curādi 491 and √picc in the progress line; 7.1.35's fork 1040 cells across 520 parasmaipada columns; 468 both-pada roots; 520 + 74 = 594; 8.4.56's outright fork 898 = 875 + 22 + 1, with √picc among its contributors; the tātaṅ fork's further 1040.
- **AGENTS.md:** 38232 cells; the gaṇa history's 10m clause; 6790 / 927; 11672 `ALTERNATES` and 49904 forms; the audit chain; the 38232 goldens.
- **tools/audit/README.md:** the totals line and a 10m "Last recorded result" entry.
- **paradigm/main.rs** docs: 11672; the 38232-cell census line; √picc in the three-form list; the `ALTERNATES` key counts (898 / 890 / 890) and √picc's fold; the audit chain.
- **guna.rs:** the 594-root corpus comment. **tripadi.rs:** the 8.4.40-section comment's account of 8.2.30.
- **The 10l spec:** its "Later slices" 10m bullet points at this slice's spec.

```python
#!/usr/bin/env python3
"""THROWAWAY: slice 10m's doc sweep — counts, curādi 491, the audit record and
chain, the 10l spec pointer. Takes AUDIT_DATE (Task 4 Step 2's `date -u +%F`).
Every `old` must occur exactly once in its file; nothing is written if one
fails. Run from the worktree root."""
import os, re, sys
D = os.environ['AUDIT_DATE']
assert re.fullmatch(r'\d{4}-\d{2}-\d{2}', D), D
BOTH = ("slice 10k's 155 rows and slice 10l's eight by 1.3.74",
        "slice 10k's 155 rows, slice 10l's eight and slice 10m's √picc by 1.3.74")
E = {
'README.md': [
("*curādi* (10) is **open** at 490 of its 509\n",
 "*curādi* (10) is **open** at 491 of its 509\n"),
("""u~Drasa~`) is udit by its initial `u~`, so its ṇic is optional
(*dhrāsayati* beside *dhrasati*).
""",
"""u~Drasa~`) is udit by its initial `u~`, so its ṇic is optional
(*dhrāsayati* beside *dhrasati*). Slice 10m curated √picc (`10.0175
picca~`, *piccayati*), ubhayapadī by 1.3.74, once 8.2.30 *coḥ kuḥ* was
narrowed, as vidyut-prakriya reads it, to a term-final cu before a
jhal-initial affix or āgama, or pada-final: the whole-word scan before it
velarised the root-internal `cc` (*pikcayati*).
"""),
("curated 593-root set, in four lakāras",
 "curated 594-root set, in four lakāras"),
("""both correct — and in fact 8460 of the 38160 cells hold more than one form: 6788
hold two, 925 hold three""",
"""both correct — and in fact 8464 of the 38232 cells hold more than one form: 6790
hold two, 927 hold three"""),
("obligatory-ṇic rows', and — new in slice 10l — seven of its eight rows', each by\n",
 "obligatory-ṇic rows', and — new in slice 10l — seven of its eight rows', and —\nnew in slice 10m — √picc's, each by\n"),
("padas — 467 roots that admit both padas in the curated set\n",
 "padas — 468 roots that admit both padas in the curated set\n"),
BOTH,
("1631 of the pinned (`PARADIGM`) surfaces are pada-ambiguous",
 "1635 of the pinned (`PARADIGM`) surfaces are pada-ambiguous"),
("set, all 1631. It is therefore",
 "set, all 1635. It is therefore"),
("""homograph `10.0143 cUrRa~` already supplies. The
""",
"""homograph `10.0143 cUrRa~` already supplies; slice 10m's √picc adds four
more. The
"""),
],
'docs/ARCHITECTURE.md': [
("curādi (10), **open** at 490 of its\n",
 "curādi (10), **open** at 491 of its\n"),
("the eight rule-bearing rows, slice 10l). gaṇa\n",
 "the eight rule-bearing rows, slice 10l; √picc, slice 10m). gaṇa\n"),
("""forking 1038 cells (loṭ
prathama and madhyama eka across the 519 roots with a parasmaipada column —""",
"""forking 1040 cells (loṭ
prathama and madhyama eka across the 520 roots with a parasmaipada column —"""),
("roots never reach this guard, and the 467 roots that admit both\n",
 "roots never reach this guard, and the 468 roots that admit both\n"),
BOTH,
("519 + 74 = the 593 curated roots",
 "520 + 74 = the 594 curated roots"),
('utterance, forking 896 cells outright: laṅ and vidhiliṅ prathama eka across\nthose same 519 parasmaipada columns (873 of them',
 'utterance, forking 898 cells outright: laṅ and vidhiliṅ prathama eka across\nthose same 520 parasmaipada columns (875 of them'),
("and 10l's other than √kṛp (likewise), √dhras on its ṇic branch;",
 "and 10l's other than √kṛp (likewise), √dhras on its ṇic branch, and 10m's √picc;"),
('forking a further 1038 (the same\n',
 'forking a further 1040 (the same\n'),
],
'AGENTS.md': [
("(`crates/panini/tests/paradigm/`, 38160 cells, ten gaṇas, nine complete —",
 "(`crates/panini/tests/paradigm/`, 38232 cells, ten gaṇas, nine complete —"),
("at 490 after slice 10l curated the eight rule-bearing rows √ūrj, √cūrṇ, √aṭṭ, √kṝt, √mlecch, √gūrd, √dhras and √kṛp —",
 "at 490 after slice 10l curated the eight rule-bearing rows √ūrj, √cūrṇ, √aṭṭ, √kṝt, √mlecch, √gūrd, √dhras and √kṛp, at 491 after slice 10m curated √picc, narrowing 8.2.30 to a term-final cu —"),
("other forms — a second (6788 cells), a third (925 cells), a fourth",
 "other forms — a second (6790 cells), a third (927 cells), a fourth"),
("`ALTERNATES` (11666 rows in all, so 38160 + 11666 = 49826 forms total); √bhuj",
 "`ALTERNATES` (11672 rows in all, so 38232 + 11672 = 49904 forms total); √bhuj"),
("""  (`tools/audit/README.md`'s 2026-10-06 10l entry, 38160 cells / 49826 forms /
  593 roots).
""",
f"""  (`tools/audit/README.md`'s 2026-10-06 10l entry, 38160 cells / 49826 forms /
  593 roots), and that by curādi 10m's (`tools/audit/README.md`'s {D} 10m
  entry, 38232 cells / 49904 forms / 594 roots).
"""),
("only in the ordinary corpus-size sense, not wrong in kind: 38160 goldens",
 "only in the ordinary corpus-size sense, not wrong in kind: 38232 goldens"),
],
'tools/audit/README.md': [
("**It asserts the corpus totals** (593 roots, 38160 cells, 49826 forms) rather than",
 "**It asserts the corpus totals** (594 roots, 38232 cells, 49904 forms) rather than"),
("""## Last recorded result

2026-10-06, curādi 10l slice, vidyut
""",
f"""## Last recorded result

{D}, curādi 10m slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 38232
cells / 49904 forms / 594 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole curādi 10m slice: `10.0175 picca~` (√picc),
ubhayapadī by 1.3.74, and 8.2.30 *coḥ kuḥ* narrowed, as vidyut-prakriya
reads it, to a term-final cu before a jhal-initial affix or āgama, or
pada-final. Blocked branches stay at 6552: √picc's ṇic is obligatory. On the
throwaway prototype that scoped the slice, main's whole-word 8.2.30 with the
row kept made 72 cells differ, all √picc's (*pikcayati*). A main-vs-branch
dump of every prior cell's branches, blocked ones included, with every
step's before and after text, was byte-identical, all 56378 of them (49826
live).

Totals: 594 = 593 + 1; 38232 = 38160 + 72 (8 root×pada×lakāra blocks × 9);
49904 = 49826 + 72 + 6 new `ALTERNATES` rows (11666 → 11672), measured via
the harness's corpus block, not assumed.

2026-10-06, curādi 10l slice, vidyut
"""),
],
'crates/panini/tests/paradigm/main.rs': [
("/// `ALTERNATES` is otherwise 11666 bare strings, and a string can be right for\n",
 "/// `ALTERNATES` is otherwise 11672 bare strings, and a string can be right for\n"),
("/// 38160 cells total (4240 root×lakāra blocks × 9), of which 29700 hold exactly one form, 6788 hold two, 925 hold three",
 "/// 38232 cells total (4248 root×lakāra blocks × 9), of which 29768 hold exactly one form, 6790 hold two, 927 hold three"),
("and seven of the eight rule-bearing rows', new in slice 10l, each by\n",
 "and seven of the eight rule-bearing rows', new in slice 10l, and √picc's,\n/// new in slice 10m, each by\n"),
("/// itself has 11666 rows, keyed 896 `8.4.56`, 888 `7.1.35`, 888 `7.1.35+8.4.56`,\n",
 "/// itself has 11672 rows, keyed 898 `8.4.56`, 890 `7.1.35`, 890 `7.1.35+8.4.56`,\n"),
("/// (√kṛp, its guṇa before ṇic) — √kṛ (slice 8b) adds six more\n",
 "/// (√kṛp, its guṇa before ṇic); slice 10m's √picc opens none and folds 2 rows\n/// apiece into `8.4.56`, `7.1.35` and `7.1.35+8.4.56` — √kṛ (slice 8b) adds six more\n"),
("""/// over all 38160 cells / 49826 forms / 593 roots with zero differences, its
/// `entry` negative control verified failing (36 √bhū cells). √tṛh joins none of the fork
""",
"""/// over all 38160 cells / 49826 forms / 593 roots with zero differences, its
/// `entry` negative control verified failing (36 √bhū cells), and curādi 10m's
/// re-ran it at the same commit over all 38232 cells / 49904 forms / 594 roots
/// with zero differences, its `entry` negative control verified failing (36
/// √bhū cells). √tṛh joins none of the fork
"""),
],
'crates/panini-prakriya/src/tinanta/guna.rs': [
("593-root × 4-lakāra", "594-root × 4-lakāra"),
],
'crates/panini-prakriya/src/tinanta/tripadi.rs': [
("""    // kuH is NOT independently sufficient: it turns a word-final or
    // jhal-followed `j`/`c` into its velar, but its own comment above (and
""",
"""    // kuH is NOT independently sufficient: it turns a term-final `j`/`c`
    // that is pada-final or before a jhal-initial affix into its velar, but
    // its own comment above (and
"""),
],
'docs/superpowers/specs/2026-10-05-curadi-gana-10l-design.md': [
("""  every prior trace before planning.
""",
"""  every prior trace before planning. Specified in
  `2026-10-06-curadi-gana-10m-design.md`.
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
AUDIT_DATE=<Step 2's date> python3 /tmp/vidyut-full/slice10m/docsweep_10m.py      # applied 36 edits to 8 files
mise run fmt
```

- [ ] **Step 4: Sweep greps**

The script covers what the prototype's greps found. Re-run the greps on this tree, and fix any hit that is not historical (a past slice's record, or 10l's own paragraphs):

```bash
grep -rni -E "four hundred ninety\b|five hundred ninety-three|four hundred twenty-seven|four hundred sixty-seven" README.md AGENTS.md docs/ARCHITECTURE.md tools crates --include=*.rs --include=*.md | grep -v "paradigm/data"
grep -rn -E "\b(490|593|427|467|519|873|896|1038|1631|38160|49826|11666|8460)\b" README.md AGENTS.md docs/ARCHITECTURE.md crates/*/src crates/panini/tests/*.rs crates/panini/tests/trace crates/panini/tests/paradigm/main.rs tools/audit/panini_full_audit.rs | grep -v '"10\.0\|"0[0-9]\.0'
grep -rn -E "word-final or|jhal-followed|two cu sounds|cross-term adjacency" crates --include=*.rs | grep -v paradigm/data
```

Expected (the prototype's output at this point):
- **Spelled-out grep:** no hits.
- **Numeral grep:** only historical lines and Task 5's: AGENTS.md's floor paragraph (lines 37 and 86, both 38160, which Task 5 rewrites), its gaṇa-history line (490, in 10l's clause) and its 10l audit-chain line (38160 / 49826 / 593); `paradigm/main.rs`'s 10l audit-chain line (555), its 10l census paragraph (867, 490), and the pada-ambiguous comment's 1631 (10l's line and 10m's).
- **Phrasing grep:** exactly two lines, both still true. `tripadi.rs`'s 8.2.31 comment ("is genuinely word-final or jhal-followed") describes 8.2.31's own scan, which this slice keeps. `tripadi.rs`'s 8.2.41 unit test ("the cross-term adjacency `word_chars` exists for") is about 8.2.41, which still reads `word_chars`.

- [ ] **Step 5: Run the full suite and commit**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
git branch --show-current      # curadi-10m
git add -A
git commit -m "docs: 10m's counts, the audit record, and the sweep

38232 cells / 49904 forms / 594 roots across README, ARCHITECTURE, AGENTS,
paradigm/main.rs and tools/audit; curādi open at 491/509; 468 both-pada
roots; 1635 pada-ambiguous surfaces. Audit at zero divergence against
8da2f90b with 6552 blocked branches; prior traces byte-identical to main
(56378 branches, 49826 live). The 10l spec points at 10m."
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
- **The mutant list grows by 4.** `panini-prakriya` lists 910 mutants (906 on main); `panini-analyze` 12 and `panini-data` 12, both unchanged. By name (spans ignored) none leave and four arrive in `tripadi.rs`: `delete !`, `!=` → `==`, `-` → `+` and `-` → `/`. The old `apply`'s `+` → `-`, `+` → `*` and `&&` → `||` re-attach by name to the new one's `i + 1` and `&&`.
- **`--in-diff` over the slice's production diff for `panini-prakriya`** lists 7, all in 8.2.30's new `apply` (`tripadi.rs` 427–432): `+` → `-` and `+` → `*` on `i + 1`, `delete !` on the empty-term test, `&&` → `||`, `!=` → `==` on the `ANGA` guard, `-` → `+` and `-` → `/` on `count() - 1`. The prototype ran all 7 (`-j 4 --timeout 11110`, `--test-workspace=true`, 3 minutes, load 36–77): **all 7 CAUGHT**, none unviable. Test phases: 0.85 s and 1.01 s for `-` → `/` and `-` → `+` on `count() - 1` (an out-of-range index panics at once), 46–53 s for the other five.
- **`--in-diff` over the data crate's diff** lists none ("No mutants to filter", measured on the prototype): the slice's changes there are a row, docs and test code.
- **The documented non-caught entries move.** Two equivalents: `adesha.rs:649:30` (unmoved) and `tripadi.rs:1405:38` (8.3.13's `apply`, from 1399: the 8.2.30 comment grew six lines). The permanent hang is `tripadi.rs:1718:23` (from 1712). The `skip_nic` pair stays at `sanadi.rs:60:5` and `60:39`. `tripadi.rs:217:38: replace - with /` (8.2.78's, CAUGHT) also matches the probe regex `tripadi.rs:[0-9]+:38: replace - with /`, so anchor the probe on the line `--list` gives inside 8.3.13. Confirm every span by `--list`; never compute them.
- **The cap will probably hold at 11110.** The slice adds no optional-ṇic id, so `skip_nic -> true` still forks 2^8 ways, and the corpus grows 0.19% (38232 cells against 38160). Re-measure it all the same, as AGENTS.md requires on corpus growth.

- [ ] **Step 1: Measure the floor**

With nothing else of ours running, run this twice: `cat /proc/loadavg; time mise run test >/dev/null 2>&1; cat /proc/loadavg` (foreground, timeout 600000 ms). Record both wall clocks, user CPU and the load averages. Read 10l's floor from AGENTS.md's floor paragraph and keep the comparison chain.

- [ ] **Step 2: Locate and probe the uncaught equivalents and the `skip_nic` pair**

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
PROBE=/home/dev/mutants-records/curadi-10m-probe   # durable
mkdir -p "$PROBE"
mise exec -- "$CM" mutants --package panini-prakriya --list -o "$PROBE/list" 2>/dev/null | wc -l      # 910
mise exec -- "$CM" mutants --package panini-prakriya --list -o "$PROBE/list" 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /=|skip_nic -> bool with true|in skip_nic"
```

Expect `adesha.rs:649:30`, `tripadi.rs:217:38` (8.2.78's, caught; not the equivalent), `tripadi.rs:1405:38` (the equivalent, inside 8.3.13's `apply`), `tripadi.rs:1718:23` (the ṇatva hang), and `sanadi.rs:60:5` (`skip_nic -> bool with true`) and `60:39` (`!=` → `==` in `skip_nic`). Write them as `<A>` (649), `<T1>` (1405), `<T2>` (1718), `<K>` and `<K2>` (60). The probe runs past the 60-minute shell limit, so launch it detached and wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`:

```bash
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 20000 -j 4 -o "$PROBE" \
  --re "adesha.rs:<A>:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" \
  --re "sanadi.rs:<K>:5: replace skip_nic -> bool with true" --re "sanadi.rs:<K2>:39: replace != with == in skip_nic" \
  > "$PROBE/probe.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$PROBE/started"; cat /proc/loadavg > "$PROBE/load.started"
```

The 20000s probe cap is a ceiling for measurement, not the campaign's cap. The regexes also match caught `mod.rs` `derive` mutants; that is expected. When it ends, copy `$PROBE/mutants.out/outcomes.json` to `$PROBE/probe-outcomes.durable.json` and record the end time and load. Both equivalents must be MISSED, not TIMEOUT; both `skip_nic` mutants must be CAUGHT. Read each test-phase duration from the outcomes. Set the provisional cap to max(11110, 6 × the longer equivalent, 2 × the longer `skip_nic` phase), rounded up to the next 10 s.

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/curadi-10m"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout <CAP> -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"; cat /proc/loadavg > "$OUT/load.started"
```

`<CAP>` is Step 2's provisional cap. Run nothing CPU-heavy meanwhile. 10l's campaign took 7h46m; expect about the same. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

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
- **922 mutants: 867 caught, 52 unviable, 2 missed, 1 timeout.**
  - panini-prakriya: 910 / 859 / 48 / 2 / 1.
  - panini-analyze: 12 / 8 / 4 / 0 / 0.
- `missed.txt` holds exactly `adesha.rs:<A>:30: replace + with *` and `tripadi.rs:<T1>:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:<T2>:23: replace -= with /=`.

Then diff the non-caught set against 10l's on the full record, both with and without span lines:

```bash
python3 - "$OUT/outcomes.durable.json" /home/dev/mutants-records/curadi-10l/outcomes.durable.json <<'PY'
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
- **With lines:** `55 55`; `new:` and `gone:` list the same `tripadi.rs` entries at their 10m and 10l spans, every one below 8.2.30's comment and six lines further down: at least the equivalent (1399 → 1405) and the hang (1712 → 1718), and 8.4.41's unviable `&&` → `||` (10l's `1284:21`). The prototype did not measure this list. Write down exactly what prints, and name each moved entry in the record.

If not:
- Any **other timeout** is a suspect survivor that the larger suite pushed past the cap. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant among the 7 new ones is a gap in Task 2's tests; add the test that kills it. Any other missed mutant means a test that caught it at 38160 cells no longer does; stop and report.

**Step 4b: the data-crate mutants.** Confirm the slice's diff for `panini-data` holds none:

```bash
git diff 6b1ec29 -- crates/panini-data/src/lib.rs > "$OUT/data.diff"
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
- If that is 11110 or less, the cap stays 11110.
- Otherwise change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together (`grep -n "11110" AGENTS.md mise.toml`).

- [ ] **Step 6: Record it in AGENTS.md**

- **The floor paragraph.** Rewrite the paragraph that opens `**The floor behind the 11110s cap, measured at 38160 cells on Rust 1.99.0,`. Use Step 1's and Step 2's numbers at 38232 cells, the load averages, and the cap Step 5 chose. Keep the comparison chain to earlier floors, with 10l's joining it: 2m46.688s / 2m47.456s at 38160 cells on a quiet host (load 3.7–11.2, user 8m38.8s / 8m43.5s), the `skip_nic` `true` mutant 2242.64s in the probe and 5553.65s in the campaign, `!=` → `==` 1883.83s and 3708.11s, the equivalents' campaign-load phases 349.19s and 341.93s.
- **The current record.** Replace the `**Current record (curādi 10l, 2026-10-06).**` paragraph with `**Current record (curādi 10m, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**;
  - the non-caught set diffed against 10l's on the full record, both ways (Step 4's script output), naming what moved and why;
  - the 7 in-diff mutants by site and outcome, and the by-name list change (4 arrive, none leave);
  - the `skip_nic` pair's probe and campaign phases;
  - Step 4b's empty data-crate list;
  - the campaign-load phases and margins;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`, and the probe's at `/home/dev/mutants-records/curadi-10m-probe/probe-outcomes.durable.json`.

  End it with a pointer to the record it replaces. Run `git rev-parse --short HEAD` before committing, and write ``The curādi 10l record it replaces: `git show <that hash>:AGENTS.md`.``

- [ ] **Step 7: Commit**

```bash
git branch --show-current      # curadi-10m
git add AGENTS.md mise.toml
git commit -m "chore: 10m mutation gate — floor and uncaught run re-measured at 38232 cells

Every new mutant caught; missed.txt holds only the two documented
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
git branch --show-current      # curadi-10m
git log --oneline main..HEAD   # the spec commit, the plan and the four task commits
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin curadi-10m
gh pr create --title "curādi 10m — √picc and 8.2.30 read as a term-final cu" --body "$(cat <<'BODY'
Slice 10m curates `10.0175 picca~` (√picc, *piccayati*), ubhayapadī by
1.3.74, and narrows 8.2.30 *coḥ kuḥ* to make it derivable.

- 8.2.30 scanned the whole word for a cu before a jhal, so √picc's
  root-internal `cc` became `kc` (*pikcayati*). It now reads terms, as
  vidyut-prakriya does: a term's final cu, before a jhal-initial affix or
  āgama, or pada-final. The dhātu itself never counts as the following
  affix; that guard has no corpus witness, so a hand-built unit test pins it.
- Every prior 8.2.30 credit is unchanged, pinned row by row (twelve rows,
  306 live branches) by a new corpus-wide roster test.

The golden suite goes from 38160 to 38232 cells; curādi is open at 491 of
509. The audit shows zero divergence against `8da2f90b`, a main-vs-branch
dump of every prior branch (blocked ones included, every step's text) is
byte-identical, and the mutation gate finds every new mutant caught.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`:
   - run `git worktree remove .worktrees/curadi-10m`;
   - run `git worktree remove --force .worktrees/curadi-10m-proto` and `git branch -D proto-10m` (the throwaway);
   - run `git worktree remove --force .worktrees/curadi-10m-replay` if the replay worktree is still there (detached; no branch);
   - delete the local and remote `curadi-10m` branch;
   - run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| 8.2.30 reads terms: term-final cu, next non-empty term not `ANGA` and jhal-initial, or none | 2 Step 3 |
| `kutva_of` drives match and substitute; the comment explaining why survives | 2 Step 3 |
| The comment block's stale paragraphs rewritten ("Read via `word_chars`", "No cell … has two cu sounds") | 2 Step 3 |
| Unit tests: term-internal declines, earlier non-qualifying cu, empty term skipped, `ANGA` guard | 2 Step 1 |
| Row `10.0175 picc`, `Nic`, comment per spec | 3 Step 3 |
| Row-list test renamed `…_four_hundred_ninety_one_…`; row count 594; `pada` doc 428 / 594 | 3 Step 1 |
| Goldens via the generator: 8 rows, 6 alternates | 3 Steps 4–5 |
| Census 38232 / 49904; buckets re-derived; 10m paragraph | 3 Step 1 |
| `a_kusmad_is_credited_on_exactly_the_akusmiya_cells` 427 → 428 | 3 Step 1 |
| 8.2.30 roster test, twelve rows with counts, `10.0175` none | 2 Step 1; 3 Step 1 |
| `curadi_analyses_its_10m_forms` with *piccayati* / *piccayate*, goldens grepped | 3 Steps 1, 7 |
| Prior-trace diff main ↔ HEAD, all 56378 | 4 Step 2 |
| Audit (negative control first, then 594 / 38232 / 49904, 6552 blocked) | 4 Steps 1–2 |
| Mutation gate: every new mutant caught, floor and `skip_nic` re-measured, cap by rule, `-o`, durable copy, non-caught named verbatim | 5 |
| Doc sweep: counts, curādi 491, 8.2.30's descriptions re-read, ARCHITECTURE census, audit record, 10l spec pointer, greps | 2 Step 3; 4 Steps 3–4 |
| Later slices | no task |

**Placeholder scan.** `<A>`, `<T1>`, `<T2>`, `<K>`, `<K2>`, `<CAP>`, `<DATE>` and the audit date are values the executor measures, each with the command that produces it and its expected value where one is known. No step says "update the tests" without the edit.

**Type consistency.**
- 8.2.30's closure returns `Option<(usize, usize, char)>`, destructured as `(term, idx, to)` for `set_char(&mut Prakriya, usize, usize, char)`, the signature it already used.
- `credited` yields `(&'static str, Gana)`; the roster test folds it into `Vec<(&str, usize)>`, sorts, and compares with an array of `(&str, usize)` tuples.
- `curadi_row` and `sole` are the 10l helpers, unchanged.
- `curadi_analyses_its_10m_forms` reads `Analysis.dhatu` (`String`, compared with `&str`), `.pada` (`Pada`) and `.trace` (`Vec<RuleStep>`, `.sutra: String`).
- The golden tuple shapes match `ParadigmRow` and `AlternateRow`; the generator is 10l's with the row selection changed.

**Where this plan goes beyond the spec's letter**, each to make the spec's decisions work:
- **Three neighbouring comments** described 8.2.30's old scan: 8.2.31's guard paragraph, the 8.2.41 section's `word_chars` paragraph, and the 8.4.40 section's account of 8.2.30. Each is corrected.
- **The pada-ambiguous set (1631 → 1635)** moves, which the spec did not name; Task 3 Step 6 pins it from measurement.
- **ARCHITECTURE's 7.1.35 and 8.4.56 fork counts** (1038 → 1040, 896 → 898, 873 → 875, 519 → 520 parasmaipada columns) are re-derived, beyond the spec's count list.
- **The `check()` test** asserts three Invalid shapes (*pikcayati*, *pikcayate*, *peccayati*) beside the spec's two Valid witnesses.

**Known soft spots.**
- **Task 5's campaign numbers are expectations**, not prototype measurements; the in-diff run is the one measured part. The floor and cap depend on host load; record the load beside every timing. The Step 4 span diff is the least certain line in the plan: write down what prints.
