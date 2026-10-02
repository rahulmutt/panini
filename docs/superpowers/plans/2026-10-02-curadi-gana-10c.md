# Curādi gaṇa slice 10c Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate 33 more ākusmīya curādi rows (`10.0195`–`10.0234`), ātmanepadī by the gaṇasūtra 10.0496 that slice 10b built. The golden suite goes from 5076 to 6264 cells, and curādi from 8 to 41 of its 509 rows.

**Architecture:** This slice changes no engine code. Six tasks:
- **Task 1** checks the worktree and baseline.
- **Task 2** generalizes a test-only helper, `stored_form`, to model 7.1.58 for every idit upadeśa. Without it, `10.0195 dasa~` reads as ambiguous against `10.0194 dasi~`.
- **Task 3** lands the 33 rows, their 132 golden rows, and every count, list and `check()` assertion they move. They must land together: the suite is red between any of them.
- **Tasks 4–6** are the audit with the prior-trace diff and the doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.99.0 pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, checked out at `/tmp/vidyut-full`.

**Spec:** `docs/superpowers/specs/2026-10-02-curadi-gana-10c-design.md`. Read its "√das and the sibling check (amendment)" section; it postdates the first draft.

**Workspace:** the branch `curadi-10c` is checked out at `/workspace/.worktrees/curadi-10c` and holds the spec and this plan. Every path below is relative to that directory unless it starts with `/`.

**Provenance.** Every code block in this plan ran green on a throwaway worktree, `/workspace/.worktrees/curadi-10c-proto`. It is detached at the throwaway commit `7aa26e1`; delete it in Task 6. That run covered:
- the full suite, clippy `-D warnings` and `fmt-check`;
- the audit at 144 roots / 6264 cells / 7394 forms with **zero differences** against vidyut, the `entry` control failing on 36 cells;
- an engine-vs-vidyut probe over all 33 rows: 1188 cells, 0 differences, every cell one form;
- a byte-identical dump of all 6206 prior live-branch logs, `main` vs prototype.

The golden rows below are vidyut's output, emitted by the throwaway example `/tmp/vidyut-full/vidyut-prakriya/examples/curadi_goldens_10c.rs`. They are not the engine's.

## Global Constraints

- **No production code changes.** `crates/panini-prakriya/src` and `crates/panini/src` gain no code. The only edit there is one comment in `guna.rs`, changing a number on the same line. `crates/panini-data/src/lib.rs` gains data rows and test-module code only.
- **No new rule ids.** In particular, **no 10.0494**: vidyut credits it on √syam and √śam, but it blocks a rule (01.0934) this engine lacks. The rows' comments say so.
- **The 5076 pre-existing cells must stay byte-identical, traces included.** Regenerate no golden and change no pinned trace.
- **Goldens are transcribed from this plan.** **Do not edit a golden to match the engine.** If a new golden fails, stop and report.
- **Homographs stay in.** `10.0197 qipa~`, `10.0211 kURa~`, `10.0214 SaWa~`, `10.0219 lakza~`, `10.0226 kuwwa~` and `10.0233 mAna~` are curated, and each comment names its ubhayapadī partner. `10.0233` and `10.0234` share every form.
- **Ākusmīya rows are found by the positional `AKUSMIYA` range** in trace tests, never by a hand list of numbers and never by the curated `pada` column.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit.
- `mise run test` takes 10–25 s depending on host load. Run it in the **foreground** with a timeout of 600000 ms. Never background it and end a turn.
- `mise run test -- -p X` does not scope. Scope with `mise exec -- cargo test -p <crate> <filter>`. To see every failing binary at once, use `mise exec -- cargo test --workspace --no-fail-fast`.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **A curated number that does not resolve uniquely upstream.** `10.0195 dasa~` vs `10.0194 dasi~` share artha and, under the old helper, it-stripped form. A wrong idit rule (irit `i~r`, `cakzi~N`) would silently change other rows' stored forms. → Task 2 `stored_form_inserts_num_for_exactly_the_idit_upadeshas`, plus `dhatupatha_numbers_resolve_upstream` over all 144 rows.
2. **`check()` on a surface two rows share.** *mAnayate* must yield exactly two analyses, √mān and √man, not one merged or deduplicated analysis. → Task 3 `curadi_analyses_its_bulk_akusmiya_forms`.
3. **A parasmaipada request for any new ākusmīya root.** It must yield only blocked branches, recording nothing. → Task 3, `an_akusmiya_roots_parasmaipada_is_blocked_by_a_kusmad_alone` widened to all 37 rows, plus the parasmaipada shapes in the `check()` test.
4. **A curated ākusmīya row the range walk misses.** A typo'd number outside `10.0192..=10.0236` would be curated `Akusmiya` but skipped. → Task 3: the walk asserts it found exactly 37 rows, and `curated_pada_agrees_with_upadesha_markers` already rejects `Akusmiya` outside the range.
5. **A prior row's trace moving.** Goldens ignore traces. → Task 3 `a_kusmad_is_credited_on_exactly_the_akusmiya_cells` (1332 credits, all in range); Task 4 Step 2's prior-trace diff.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-data/src/lib.rs` | Task 2: `stored_form` and its unit test. Task 3: 33 rows, the curādi row-list test, `dhatus().len()`, the `pada` and `pada_from_upadesha` doc counts |
| `crates/panini/tests/paradigm/data/curadi.rs` | Task 3: 132 golden rows |
| `crates/panini/tests/paradigm/main.rs` | Task 3: totals, doc counts, slice paragraph, `check()` test. Task 4: audit-chain prose |
| `crates/panini/tests/trace/curadi.rs` | Task 3: two tests switched to the `AKUSMIYA` range |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini-prakriya/src/tinanta/guna.rs` (one comment), the 10b spec | Task 4 |
| `AGENTS.md`, maybe `mise.toml` | Task 5 |

---

## Task 1: The worktree and baseline

**Files:** none. **Interfaces:** none.

- [ ] **Step 1: Confirm the worktree**

```bash
cd /workspace/.worktrees/curadi-10c
git status --short          # empty
git log --oneline -4        # the plan commit, 3cdc130 (spec amendment), f1c96c4 (spec), 54a5fcb
```

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | head -20
```

Run in the foreground, timeout 600000 ms. Expected: everything passes at 5076 cells. `panini-prakriya` reports 403 tests, the `trace` binary 196, `paradigm` 21 and `panini-data` 22.

---

## Task 2: `stored_form` models 7.1.58 for every idit upadeśa

This is a test-only change in `panini-data`'s `#[cfg(test)]` module. It must land before the rows: Task 3's `10.0195` fails `dhatupatha_numbers_resolve_upstream` without it.

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (the `tests` module: `stored_form`, one new test)

**Interfaces:**
- Consumes: the existing test helpers `strip_anubandhas(&str) -> String`, `dhatvadeh_sha_sa(String) -> String` and `is_hal(char) -> bool`.
- Produces: `stored_form(&str) -> String`, same signature, new behaviour. For an upadeśa whose last marker is `i~` (after trailing `\`/`^` accents), it inserts `n` after the last vowel of the it-stripped form. Otherwise it returns the it-stripped form.

- [ ] **Step 1: Write the failing test**

In `crates/panini-data/src/lib.rs`, immediately after the closing `}` of `fn stored_form` and before `#[test] fn dhatupatha_numbers_resolve_upstream()`, add:

```rust

    #[test]
    fn stored_form_inserts_num_for_exactly_the_idit_upadeshas() {
        // idit: the num lands after the last vowel.
        assert_eq!(stored_form("hisi~"), "hins");
        assert_eq!(stored_form("dasi~"), "dans");
        assert_eq!(stored_form("tatri~"), "tantr");
        assert_eq!(stored_form("aci~^"), "anc");
        // A non-final `i~` is another marker's: no num.
        assert_eq!(stored_form("ru\\Di~^r"), "ruD");
        assert_eq!(stored_form("ca\\kzi~\\N"), "cakz");
        // No `i~` at all: the plain it-stripped form.
        assert_eq!(stored_form("dasa~"), "das");
        assert_eq!(stored_form("kusma~"), "kusm");
    }
```

- [ ] **Step 2: Run it to see it fail**

Run: `mise exec -- cargo test -p panini-data stored_form_inserts 2>&1 | grep -E "panicked|left|right" | head`
Expected: FAIL at the `dasi~` assertion, `left: "das"`, `right: "dans"`. The old helper handles only `his`.

- [ ] **Step 3: Generalize the helper**

In `fn stored_form`, replace

```rust
        let s = dhatvadeh_sha_sa(strip_anubandhas(upadesha));
        // 7.1.58 idito num dhātoḥ is not derivable here, so √hiṃs is stored
        // with the num already inserted. This is the single deviation between
        // an it-stripped upadeśa and a stored `code`, and it is the same one
        // the retired `Dhatu::id` doc comment recorded.
        if s == "his" { "hins".to_string() } else { s }
    }
```

with

```rust
        let s = dhatvadeh_sha_sa(strip_anubandhas(upadesha));
        // 7.1.58 idito num dhātoḥ is not derivable here, so an idit root is
        // stored with the num already inserted, after its last vowel (1.1.47
        // mid aco 'ntyāt paraḥ): `hisi~` stores as `hins`. An upadeśa is idit
        // when its LAST marker is `i~`; a non-final `i~` belongs to another
        // marker — irit `i~r` (`ru\Di~^r`) or `cakzi~N`. This is the single
        // deviation between an it-stripped upadeśa and a stored `code`. It is
        // applied to every upadeśa, not only curated ones, because the
        // sibling check below compares a curated row against its uncurated
        // neighbours: `10.0194 dasi~` must store as `dans`, not collide with
        // `10.0195 dasa~`'s `das`.
        let idit = upadesha.trim_end_matches(['\\', '^']).ends_with("i~");
        match s.rfind(|c: char| !is_hal(c)) {
            Some(i) if idit => format!("{}n{}", &s[..=i], &s[i + 1..]),
            _ => s,
        }
    }
```

The idit test is "the upadeśa, with trailing accent marks trimmed, ends in `i~`". The vendored dhātupāṭha's only non-final `i~` shapes are irit `…i~r` and `cakzi~N`. Confirm this with:

```bash
awk -F'\t' '{print $2}' data/dhatupatha.tsv | tr -d '\\^' | grep 'i~' | grep -v 'i~$' | sed 's/.*i~//' | sort | uniq -c
```

Expected: `27 r` and `1 N`.

- [ ] **Step 4: Run the data tests**

Run: `mise exec -- cargo test -p panini-data 2>&1 | grep -E "FAILED|test result" | head -2`
Expected: PASS, 23 tests. `dhatupatha_numbers_resolve_upstream` and `curated_pada_agrees_with_upadesha_markers` are unchanged in outcome; `07.0019 hisi~` still stores as `hins`.

- [ ] **Step 5: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-data/src/lib.rs
git commit -m "test(data): stored_form models 7.1.58 for every idit upadeśa

dasi~ now stores as dans, so 10.0195 dasa~ (Task 3) resolves uniquely
against its same-artha neighbour 10.0194; hisi~ -> hins is unchanged."
```

---

## Task 3: The rows, their goldens, and every assertion they move

This task takes the suite from 5076 to 6264 cells. The assertions come first and are seen failing; then the rows and goldens make them pass.

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini/tests/paradigm/data/curadi.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`
- Modify: `crates/panini/tests/trace/curadi.rs`

**Interfaces:**
- Consumes: Task 2's `stored_form`. From 10b: `PadaAssignment::Akusmiya`, `panini_data::AKUSMIYA: RangeInclusive<&str>` (`"10.0192"..="10.0236"`), and the trace helpers `credited(sutra) -> Vec<(&'static str, Gana)>` (one entry per non-blocked branch, over every curated cell, whose log carries `sutra`). Also `panini::Panini::check(&str) -> CheckResult`, with `verdict: Verdict` and `analyses: Vec<panini::Analysis>`. `Analysis` has `dhatu: String` (the `code`), `pada`, and `trace: Vec<RuleStep>`, where `RuleStep` has `sutra: String`.
- Produces: 33 `dhatus()` rows, each `Gana::Curadi`, `PadaAssignment::Akusmiya`. Task 4 counts them.

- [ ] **Step 1: Update the data-layer assertions (failing)**

In `crates/panini-data/src/lib.rs`:
- In `curated_roots_have_expected_ganas_and_padas`, change `assert_eq!(dhatus().len(), 111);` to `assert_eq!(dhatus().len(), 144);`.
- Replace the whole test `curadi_rows_are_the_eight_curated_roots` (from its `#[test]` through its closing `}`) with:

```rust
    #[test]
    fn curadi_rows_are_the_forty_one_curated_roots() {
        // Slice 10a opens gaṇa 10 with four roots that need only ṇic
        // (3.1.25), 3.1.32 and the guṇa/vṛddhi before ṇic: √cur (7.3.86),
        // √laḍ (7.2.116), √bhakṣ and √bhūṣ (neither). None carries a pada
        // marker; all four are ubhayapadī by 1.3.74 ṇicaś ca. Slice 10b adds
        // four ākusmīya roots, ātmanepadī by 10.0496 ā kusmād ātmanepadinaḥ:
        // √cit and √vṛṣ (7.3.86), √mad (7.2.116), √kusm (neither). Slice 10c
        // adds thirty-three more ākusmīya rows that need nothing
        // new: six by 7.3.86, ten by 7.2.116, seventeen unchanged before ṇic.
        // The gaṇa is OPEN at 41 of its 509 dhātupāṭha rows.
        let rows: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.gana == Gana::Curadi)
            .map(|d| (d.dhatupatha, d.code, d.pada))
            .collect();
        assert_eq!(
            rows,
            vec![
                ("10.0001", "cur", PadaAssignment::Nic),
                ("10.0010", "laq", PadaAssignment::Nic),
                ("10.0033", "Bakz", PadaAssignment::Nic),
                ("10.0255", "BUz", PadaAssignment::Nic),
                ("10.0192", "cit", PadaAssignment::Akusmiya),
                ("10.0195", "das", PadaAssignment::Akusmiya),
                ("10.0196", "qap", PadaAssignment::Akusmiya),
                ("10.0197", "qip", PadaAssignment::Akusmiya),
                ("10.0200", "spaS", PadaAssignment::Akusmiya),
                ("10.0201", "tarj", PadaAssignment::Akusmiya),
                ("10.0202", "Barts", PadaAssignment::Akusmiya),
                ("10.0203", "bast", PadaAssignment::Akusmiya),
                ("10.0204", "ganD", PadaAssignment::Akusmiya),
                ("10.0205", "kil", PadaAssignment::Akusmiya),
                ("10.0206", "pil", PadaAssignment::Akusmiya),
                ("10.0207", "vizk", PadaAssignment::Akusmiya),
                ("10.0208", "hizk", PadaAssignment::Akusmiya),
                ("10.0209", "nizk", PadaAssignment::Akusmiya),
                ("10.0210", "lal", PadaAssignment::Akusmiya),
                ("10.0211", "kUR", PadaAssignment::Akusmiya),
                ("10.0212", "tUR", PadaAssignment::Akusmiya),
                ("10.0213", "BrUR", PadaAssignment::Akusmiya),
                ("10.0214", "SaW", PadaAssignment::Akusmiya),
                ("10.0215", "yakz", PadaAssignment::Akusmiya),
                ("10.0216", "syam", PadaAssignment::Akusmiya),
                ("10.0217", "gUr", PadaAssignment::Akusmiya),
                ("10.0218", "Sam", PadaAssignment::Akusmiya),
                ("10.0219", "lakz", PadaAssignment::Akusmiya),
                ("10.0220", "kuts", PadaAssignment::Akusmiya),
                ("10.0221", "truw", PadaAssignment::Akusmiya),
                ("10.0222", "kuw", PadaAssignment::Akusmiya),
                ("10.0223", "gal", PadaAssignment::Akusmiya),
                ("10.0224", "Bal", PadaAssignment::Akusmiya),
                ("10.0225", "kUw", PadaAssignment::Akusmiya),
                ("10.0226", "kuww", PadaAssignment::Akusmiya),
                ("10.0228", "vfz", PadaAssignment::Akusmiya),
                ("10.0229", "mad", PadaAssignment::Akusmiya),
                ("10.0232", "vid", PadaAssignment::Akusmiya),
                ("10.0233", "mAn", PadaAssignment::Akusmiya),
                ("10.0234", "man", PadaAssignment::Akusmiya),
                ("10.0236", "kusm", PadaAssignment::Akusmiya),
            ]
        );
    }
```

- [ ] **Step 2: Update the trace tests (failing)**

In `crates/panini/tests/trace/curadi.rs`, replace

```rust
use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};
```

with

```rust
use panini_data::{AKUSMIYA, Gana, Lakara, Pada, Purusha, Vacana, dhatus};
```

In `an_akusmiya_roots_parasmaipada_is_blocked_by_a_kusmad_alone`, replace

```rust
    // Every parasmaipada cell of the four rows derives only blocked
    // branches, and the block is 10.0496's: nothing is recorded, so no later
    // rule ran on the branch.
    for number in ["10.0192", "10.0228", "10.0229", "10.0236"] {
        let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
```

with

```rust
    // Every parasmaipada cell of every curated ākusmīya row derives only
    // blocked branches, and the block is 10.0496's: nothing is recorded, so
    // no later rule ran on the branch. The rows are found by the positional
    // `AKUSMIYA` range, not by the curated `pada` column.
    let rows: Vec<_> = dhatus()
        .iter()
        .filter(|d| d.gana == Gana::Curadi && AKUSMIYA.contains(&d.dhatupatha))
        .collect();
    assert_eq!(rows.len(), 37, "curated ākusmīya rows");
    for d in rows {
        let number = d.dhatupatha;
```

The loop body is unchanged. It still uses `number` in its `cell` message.

In `a_kusmad_is_credited_on_exactly_the_akusmiya_cells`, replace

```rust
    // 10.0496 fires on every ātmanepada cell of the four ākusmīya rows —
    // 4 roots × 4 lakāras × 9 cells, one branch each — and nowhere else. And
    // 1.3.74 never reaches them: its credits stay on the four `Nic` rows.
    let hits = credited("10.0496");
    assert_eq!(hits.len(), 144);
    for (number, _) in &hits {
        assert!(
            ["10.0192", "10.0228", "10.0229", "10.0236"].contains(number),
            "10.0496 credited on {number}"
        );
    }
```

with

```rust
    // 10.0496 fires on every ātmanepada cell of the 37 curated ākusmīya
    // rows — 37 roots × 4 lakāras × 9 cells, one branch each — and nowhere
    // else: every credit's number lies in the positional `AKUSMIYA` range.
    // And 1.3.74 never reaches them: its credits stay on the four `Nic` rows.
    let hits = credited("10.0496");
    assert_eq!(hits.len(), 1332);
    for (number, _) in &hits {
        assert!(AKUSMIYA.contains(number), "10.0496 credited on {number}");
    }
```

The 1.3.74 half of that test is unchanged.

- [ ] **Step 3: Update the paradigm totals and add the `check()` test (failing)**

In `crates/panini/tests/paradigm/main.rs`, in `derivation_set_shape_matches_the_audited_numbers`:
- `assert_eq!(total_cells, 5076, "564 root×lakāra blocks × 9 cells each");` → `assert_eq!(total_cells, 6264, "696 root×lakāra blocks × 9 cells each");`
- `assert_eq!(ones, 4252, "one-form cells");` → `assert_eq!(ones, 5440, "one-form cells");`

`twos` … `sevens`, `ALTERNATES.len()` (1130) and every `key_count` are unchanged. Check: 5440 + 612 + 165 + 19 + 10 + 17 + 1 = 6264 cells, and 6264 + 1130 = 7394 forms.

In the doc comment above that test:
- `/// 5076 cells total (564 root×lakāra blocks × 9), of which 4252 hold exactly one form,` → `/// 6264 cells total (696 root×lakāra blocks × 9), of which 5440 hold exactly one form,`. The rest of that line is unchanged.
- After the slice-10b paragraph, which ends `/// hold one form. No new rows. The gaṇa is OPEN at 8 of its 509 rows.`, add:

```rust
///
/// Slice 10c curates thirty-three more ākusmīya roots (`10.0195` through
/// `10.0234`; six take 7.3.86 before ṇic, ten 7.2.116, seventeen neither),
/// again ātmanepada only and one form per cell: 1188 new cells, no new
/// rows. `10.0233 mAna~` and `10.0234 mana~` share every form. The gaṇa is
/// OPEN at 41 of its 509 rows.
```

Before adding the `check()` test, grep the goldens. Every witness must be its own row's alone, except the shared pair:

```bash
for f in dAsayate aqepayata spASayatAm tarjayeta trowayaDve vedayate kURayasva SAmayeran syAmayate alakzayanta kuwwayate mAnayate amAnayata dAsayati vedayati mAnayati kuwwayatu; do
  echo "$f $(grep -rlw "\"$f\"" crates/panini/tests/paradigm/data/ | tr '\n' ' ')"; done
```

Expected: every form prints no file at this point; the rows land in Step 5. After Step 5, re-run it: the first thirteen print only `curadi.rs`, and the last four print nothing.

Then append to the end of `crates/panini/tests/paradigm/main.rs`:

```rust

/// Slice 10c's ākusmīya rows through `check`: one witness per pre-ṇic shape
/// and per homograph row. The goldens were grepped first — each surface below
/// is its own row's alone, so each must yield exactly one analysis, naming
/// that root, ātmanepada, opening with 10.0496 and crediting no pada sūtra.
/// `mAnayate` and `amAnayata` are the one pair two rows share (`10.0233
/// mAna~` unchanged before ṇic, `10.0234 mana~` by 7.2.116): exactly two
/// analyses, one per root, and only √man's credits 7.2.116. The
/// parasmaipada shapes derive nothing.
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
    for form in ["mAnayate", "amAnayata"] {
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
    for form in ["dAsayati", "vedayati", "mAnayati", "kuwwayatu"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
```

- [ ] **Step 4: Run the tests to see them fail**

Run: `mise exec -- cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .*FAILED|left:|right:" | head -30`

Expected failures, and only these:
- `curadi_rows_are_the_forty_one_curated_roots` (8 rows found, not 41);
- `curated_roots_have_expected_ganas_and_padas` (111 ≠ 144);
- `derivation_set_shape_matches_the_audited_numbers` (5076 ≠ 6264);
- `curadi_analyses_its_bulk_akusmiya_forms` (`dAsayate` Invalid);
- `an_akusmiya_roots_parasmaipada_is_blocked_by_a_kusmad_alone` (4 ≠ 37);
- `a_kusmad_is_credited_on_exactly_the_akusmiya_cells` (144 ≠ 1332).

- [ ] **Step 5: Add the 33 `Dhatu` rows**

In `DHATUS`, 10b's four ākusmīya rows are the last four entries: `10.0192`, `10.0228`, `10.0229`, `10.0236`. Insert the new rows so that the ākusmīya block runs in dhātupāṭha order. Upadeśa and artha are verbatim from `data/dhatupatha.tsv`.

**(a)** After the `10.0192` (`cit`) row's closing `},` and before the `10.0228` (`vfz`) row's `Dhatu {`, insert these thirty rows:

```rust
    Dhatu {
        // 10.0195 `dasa~` darSanadaMSanayoH (√das). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (dAs-i). Ātmanepadī by
        // 10.0496. Slice 10c.
        dhatupatha: "10.0195",
        code: "das",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "darSanadaMSanayoH",
    },
    Dhatu {
        // 10.0196 `qapa~` saNGAte (√ḍap). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (qAp-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0196",
        code: "qap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0197 `qipa~` saNGAte (√ḍip). 7.3.86 guṇates the laghu upadhā
        // before ṇic (qep-i). Homograph of the ubhayapadī curādi row `10.0189
        // qipa~`. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0197",
        code: "qip",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0200 `spaSa~` grahaRasaMSlezaRayoH (√spaś). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (spAS-i). Ātmanepadī by
        // 10.0496. Slice 10c.
        dhatupatha: "10.0200",
        code: "spaS",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "grahaRasaMSlezaRayoH",
    },
    Dhatu {
        // 10.0201 `tarja~` tarjane (√tarj). Guru upadhā (the conjunct `rj`),
        // so unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0201",
        code: "tarj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "tarjane",
    },
    Dhatu {
        // 10.0202 `Bartsa~` tarjane (√bharts). Guru upadhā (the conjunct
        // `rts`), so unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0202",
        code: "Barts",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "tarjane",
    },
    Dhatu {
        // 10.0203 `basta~` ardane (√bast). Guru upadhā (the conjunct `st`), so
        // unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0203",
        code: "bast",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "ardane",
    },
    Dhatu {
        // 10.0204 `ganDa~` ardane (√gandh). Guru upadhā (the conjunct `nD`),
        // so unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0204",
        code: "ganD",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "ardane",
    },
    Dhatu {
        // 10.0205 `kila~` kzepe (√kil). 7.3.86 guṇates the laghu upadhā before
        // ṇic (kel-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0205",
        code: "kil",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "kzepe",
    },
    Dhatu {
        // 10.0206 `pila~` kzepe (√pil). 7.3.86 guṇates the laghu upadhā before
        // ṇic (pel-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0206",
        code: "pil",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "kzepe",
    },
    Dhatu {
        // 10.0207 `vizka~` hiMsAyAm (√viṣk). Guru upadhā (the conjunct `zk`),
        // so unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0207",
        code: "vizk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0208 `hizka~` hiMsAyAm (√hiṣk). Guru upadhā (the conjunct `zk`),
        // so unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0208",
        code: "hizk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0209 `nizka~` parimARe (√niṣk). Guru upadhā (the conjunct `zk`),
        // so unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0209",
        code: "nizk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "parimARe",
    },
    Dhatu {
        // 10.0210 `lala~` IpsAyAm (√lal). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (lAl-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0210",
        code: "lal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "IpsAyAm",
    },
    Dhatu {
        // 10.0211 `kURa~` saNkoce (√kūṇ). Long upadhā vowel `U`, so unchanged
        // before ṇic. Homograph of the ubhayapadī curādi row `10.0438 kURa~`.
        // Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0211",
        code: "kUR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "saNkoce",
    },
    Dhatu {
        // 10.0212 `tURa~` pUraRe (√tūṇ). Long upadhā vowel `U`, so unchanged
        // before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0212",
        code: "tUR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "pUraRe",
    },
    Dhatu {
        // 10.0213 `BrURa~` ASAviSaNkayoH (√bhrūṇ). Long upadhā vowel `U`, so
        // unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0213",
        code: "BrUR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "ASAviSaNkayoH",
    },
    Dhatu {
        // 10.0214 `SaWa~` SlAGAyAm (√śaṭh). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (SAW-i). Homograph of the ubhayapadī
        // curādi row `10.0041 SaWa~`. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0214",
        code: "SaW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "SlAGAyAm",
    },
    Dhatu {
        // 10.0215 `yakza~` pUjAyAm (√yakṣ). Guru upadhā (the conjunct `kz`),
        // so unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0215",
        code: "yakz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "pUjAyAm",
    },
    Dhatu {
        // 10.0216 `syama~` vitarke (√syam). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (syAm-i). vidyut also credits the
        // gaṇasūtra 10.0494 nānye mito 'hetau here, in place of the am-final
        // mittva 01.0934; this engine has neither, and the forms agree. The
        // mit slice inherits this row as a witness: once it adds 01.0934, this
        // golden fails unless 10.0494 comes with it. Ātmanepadī by 10.0496.
        // Slice 10c.
        dhatupatha: "10.0216",
        code: "syam",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "vitarke",
    },
    Dhatu {
        // 10.0217 `gUra~` udyamane (√gūr). Long upadhā vowel `U`, so unchanged
        // before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0217",
        code: "gUr",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "udyamane",
    },
    Dhatu {
        // 10.0218 `Sama~` Alocane (√śam). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (SAm-i). vidyut also credits 10.0494 here,
        // as on `10.0216`; a witness for the mit slice. Ātmanepadī by 10.0496.
        // Slice 10c.
        dhatupatha: "10.0218",
        code: "Sam",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "Alocane",
    },
    Dhatu {
        // 10.0219 `lakza~` Alocane (√lakṣ). Guru upadhā (the conjunct `kz`),
        // so unchanged before ṇic. Homograph of the ubhayapadī curādi row
        // `10.0006 lakza~`. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0219",
        code: "lakz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "Alocane",
    },
    Dhatu {
        // 10.0220 `kutsa~` avakzepaRe nindane ca (√kuts). Guru upadhā (the
        // conjunct `ts`), so unchanged before ṇic. Ātmanepadī by 10.0496.
        // Slice 10c.
        dhatupatha: "10.0220",
        code: "kuts",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "avakzepaRe nindane ca",
    },
    Dhatu {
        // 10.0221 `truwa~` Cedane (√truṭ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (trow-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0221",
        code: "truw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "Cedane",
    },
    Dhatu {
        // 10.0222 `kuwa~` Cedane (√kuṭ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (kow-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0222",
        code: "kuw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "Cedane",
    },
    Dhatu {
        // 10.0223 `gala~` sravaRe (√gal). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (gAl-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0223",
        code: "gal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "sravaRe",
    },
    Dhatu {
        // 10.0224 `Bala~` ABaRqane (√bal). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (BAl-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0224",
        code: "Bal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "ABaRqane",
    },
    Dhatu {
        // 10.0225 `kUwa~` ApradAne avasAdane ca (√kūṭ). Long upadhā vowel `U`,
        // so unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0225",
        code: "kUw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "ApradAne avasAdane ca",
    },
    Dhatu {
        // 10.0226 `kuwwa~` pratApane (√kuṭṭ). Guru upadhā (the conjunct `ww`),
        // so unchanged before ṇic. Homograph of the ubhayapadī curādi row
        // `10.0034 kuwwa~`. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0226",
        code: "kuww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "pratApane",
    },
```

**(b)** After the `10.0229` (`mad`) row's closing `},` and before the `10.0236` (`kusm`) row's `Dhatu {`, insert these three rows:

```rust
    Dhatu {
        // 10.0232 `vida~` cetanAKyAnanivAsezu (√vid). 7.3.86 guṇates the laghu
        // upadhā before ṇic (ved-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0232",
        code: "vid",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "cetanAKyAnanivAsezu",
    },
    Dhatu {
        // 10.0233 `mAna~` stamBe (√mān). Long upadhā vowel `A`, so unchanged
        // before ṇic. Homograph of the ubhayapadī curādi row `10.0381 mAna~`;
        // and every form is also `10.0234`'s. Ātmanepadī by 10.0496. Slice
        // 10c.
        dhatupatha: "10.0233",
        code: "mAn",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "stamBe",
    },
    Dhatu {
        // 10.0234 `mana~` stamBe (√man). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (mAn-i). Every form is also `10.0233`'s
        // (mAnayate …). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0234",
        code: "man",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "stamBe",
    },
```

Counts in the same file (each `old` occurs once):
- In `Dhatu`'s `pada` doc, replace

```rust
    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 111
    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and
    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed
    /// exception, four curādi rows' are 1.3.74's and four ākusmīya rows' are
    /// the gaṇasūtra 10.0496's, each asserted explicitly from both sides, the
    /// same way
```

  with

```rust
    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 144
    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and
    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed
    /// exception, four curādi rows' are 1.3.74's and 37 ākusmīya rows' are
    /// the gaṇasūtra 10.0496's, each asserted explicitly from both sides, the
    /// same way
```

  (102 + 1 + 4 + 37 = 144.)
- `/// The test covers the 111 roots curated here, not the dhātupāṭha's 2259.` → `/// The test covers the 144 roots curated here, not the dhātupāṭha's 2259.`
- In `pada_from_upadesha`'s doc, `66 of the 111 curated roots` → `66 of the 144 curated roots`. None of the 33 upadeśas carries a `\`, so 66 and 45 still hold.

- [ ] **Step 6: Add the golden rows**

In `crates/panini/tests/paradigm/data/curadi.rs`, 10b's sixteen rows end `PARADIGM`: four lakāras each for `10.0192`, `10.0228`, `10.0229` and `10.0236`. `ALTERNATES` is unchanged. `mise run fmt` rewraps the one-line rows below into the file's style.

**(a)** After the `("10.0192", "viDiliN", …)` row and before the `("10.0228", "laT", …)` row, insert:

```rust
    ("10.0195", "laT", Pada::Atmanepada, ["dAsayate", "dAsayete", "dAsayante", "dAsayase", "dAsayeTe", "dAsayaDve", "dAsaye", "dAsayAvahe", "dAsayAmahe"]),
    ("10.0195", "laN", Pada::Atmanepada, ["adAsayata", "adAsayetAm", "adAsayanta", "adAsayaTAH", "adAsayeTAm", "adAsayaDvam", "adAsaye", "adAsayAvahi", "adAsayAmahi"]),
    ("10.0195", "loT", Pada::Atmanepada, ["dAsayatAm", "dAsayetAm", "dAsayantAm", "dAsayasva", "dAsayeTAm", "dAsayaDvam", "dAsayE", "dAsayAvahE", "dAsayAmahE"]),
    ("10.0195", "viDiliN", Pada::Atmanepada, ["dAsayeta", "dAsayeyAtAm", "dAsayeran", "dAsayeTAH", "dAsayeyATAm", "dAsayeDvam", "dAsayeya", "dAsayevahi", "dAsayemahi"]),
    ("10.0196", "laT", Pada::Atmanepada, ["qApayate", "qApayete", "qApayante", "qApayase", "qApayeTe", "qApayaDve", "qApaye", "qApayAvahe", "qApayAmahe"]),
    ("10.0196", "laN", Pada::Atmanepada, ["aqApayata", "aqApayetAm", "aqApayanta", "aqApayaTAH", "aqApayeTAm", "aqApayaDvam", "aqApaye", "aqApayAvahi", "aqApayAmahi"]),
    ("10.0196", "loT", Pada::Atmanepada, ["qApayatAm", "qApayetAm", "qApayantAm", "qApayasva", "qApayeTAm", "qApayaDvam", "qApayE", "qApayAvahE", "qApayAmahE"]),
    ("10.0196", "viDiliN", Pada::Atmanepada, ["qApayeta", "qApayeyAtAm", "qApayeran", "qApayeTAH", "qApayeyATAm", "qApayeDvam", "qApayeya", "qApayevahi", "qApayemahi"]),
    ("10.0197", "laT", Pada::Atmanepada, ["qepayate", "qepayete", "qepayante", "qepayase", "qepayeTe", "qepayaDve", "qepaye", "qepayAvahe", "qepayAmahe"]),
    ("10.0197", "laN", Pada::Atmanepada, ["aqepayata", "aqepayetAm", "aqepayanta", "aqepayaTAH", "aqepayeTAm", "aqepayaDvam", "aqepaye", "aqepayAvahi", "aqepayAmahi"]),
    ("10.0197", "loT", Pada::Atmanepada, ["qepayatAm", "qepayetAm", "qepayantAm", "qepayasva", "qepayeTAm", "qepayaDvam", "qepayE", "qepayAvahE", "qepayAmahE"]),
    ("10.0197", "viDiliN", Pada::Atmanepada, ["qepayeta", "qepayeyAtAm", "qepayeran", "qepayeTAH", "qepayeyATAm", "qepayeDvam", "qepayeya", "qepayevahi", "qepayemahi"]),
    ("10.0200", "laT", Pada::Atmanepada, ["spASayate", "spASayete", "spASayante", "spASayase", "spASayeTe", "spASayaDve", "spASaye", "spASayAvahe", "spASayAmahe"]),
    ("10.0200", "laN", Pada::Atmanepada, ["aspASayata", "aspASayetAm", "aspASayanta", "aspASayaTAH", "aspASayeTAm", "aspASayaDvam", "aspASaye", "aspASayAvahi", "aspASayAmahi"]),
    ("10.0200", "loT", Pada::Atmanepada, ["spASayatAm", "spASayetAm", "spASayantAm", "spASayasva", "spASayeTAm", "spASayaDvam", "spASayE", "spASayAvahE", "spASayAmahE"]),
    ("10.0200", "viDiliN", Pada::Atmanepada, ["spASayeta", "spASayeyAtAm", "spASayeran", "spASayeTAH", "spASayeyATAm", "spASayeDvam", "spASayeya", "spASayevahi", "spASayemahi"]),
    ("10.0201", "laT", Pada::Atmanepada, ["tarjayate", "tarjayete", "tarjayante", "tarjayase", "tarjayeTe", "tarjayaDve", "tarjaye", "tarjayAvahe", "tarjayAmahe"]),
    ("10.0201", "laN", Pada::Atmanepada, ["atarjayata", "atarjayetAm", "atarjayanta", "atarjayaTAH", "atarjayeTAm", "atarjayaDvam", "atarjaye", "atarjayAvahi", "atarjayAmahi"]),
    ("10.0201", "loT", Pada::Atmanepada, ["tarjayatAm", "tarjayetAm", "tarjayantAm", "tarjayasva", "tarjayeTAm", "tarjayaDvam", "tarjayE", "tarjayAvahE", "tarjayAmahE"]),
    ("10.0201", "viDiliN", Pada::Atmanepada, ["tarjayeta", "tarjayeyAtAm", "tarjayeran", "tarjayeTAH", "tarjayeyATAm", "tarjayeDvam", "tarjayeya", "tarjayevahi", "tarjayemahi"]),
    ("10.0202", "laT", Pada::Atmanepada, ["Bartsayate", "Bartsayete", "Bartsayante", "Bartsayase", "BartsayeTe", "BartsayaDve", "Bartsaye", "BartsayAvahe", "BartsayAmahe"]),
    ("10.0202", "laN", Pada::Atmanepada, ["aBartsayata", "aBartsayetAm", "aBartsayanta", "aBartsayaTAH", "aBartsayeTAm", "aBartsayaDvam", "aBartsaye", "aBartsayAvahi", "aBartsayAmahi"]),
    ("10.0202", "loT", Pada::Atmanepada, ["BartsayatAm", "BartsayetAm", "BartsayantAm", "Bartsayasva", "BartsayeTAm", "BartsayaDvam", "BartsayE", "BartsayAvahE", "BartsayAmahE"]),
    ("10.0202", "viDiliN", Pada::Atmanepada, ["Bartsayeta", "BartsayeyAtAm", "Bartsayeran", "BartsayeTAH", "BartsayeyATAm", "BartsayeDvam", "Bartsayeya", "Bartsayevahi", "Bartsayemahi"]),
    ("10.0203", "laT", Pada::Atmanepada, ["bastayate", "bastayete", "bastayante", "bastayase", "bastayeTe", "bastayaDve", "bastaye", "bastayAvahe", "bastayAmahe"]),
    ("10.0203", "laN", Pada::Atmanepada, ["abastayata", "abastayetAm", "abastayanta", "abastayaTAH", "abastayeTAm", "abastayaDvam", "abastaye", "abastayAvahi", "abastayAmahi"]),
    ("10.0203", "loT", Pada::Atmanepada, ["bastayatAm", "bastayetAm", "bastayantAm", "bastayasva", "bastayeTAm", "bastayaDvam", "bastayE", "bastayAvahE", "bastayAmahE"]),
    ("10.0203", "viDiliN", Pada::Atmanepada, ["bastayeta", "bastayeyAtAm", "bastayeran", "bastayeTAH", "bastayeyATAm", "bastayeDvam", "bastayeya", "bastayevahi", "bastayemahi"]),
    ("10.0204", "laT", Pada::Atmanepada, ["ganDayate", "ganDayete", "ganDayante", "ganDayase", "ganDayeTe", "ganDayaDve", "ganDaye", "ganDayAvahe", "ganDayAmahe"]),
    ("10.0204", "laN", Pada::Atmanepada, ["aganDayata", "aganDayetAm", "aganDayanta", "aganDayaTAH", "aganDayeTAm", "aganDayaDvam", "aganDaye", "aganDayAvahi", "aganDayAmahi"]),
    ("10.0204", "loT", Pada::Atmanepada, ["ganDayatAm", "ganDayetAm", "ganDayantAm", "ganDayasva", "ganDayeTAm", "ganDayaDvam", "ganDayE", "ganDayAvahE", "ganDayAmahE"]),
    ("10.0204", "viDiliN", Pada::Atmanepada, ["ganDayeta", "ganDayeyAtAm", "ganDayeran", "ganDayeTAH", "ganDayeyATAm", "ganDayeDvam", "ganDayeya", "ganDayevahi", "ganDayemahi"]),
    ("10.0205", "laT", Pada::Atmanepada, ["kelayate", "kelayete", "kelayante", "kelayase", "kelayeTe", "kelayaDve", "kelaye", "kelayAvahe", "kelayAmahe"]),
    ("10.0205", "laN", Pada::Atmanepada, ["akelayata", "akelayetAm", "akelayanta", "akelayaTAH", "akelayeTAm", "akelayaDvam", "akelaye", "akelayAvahi", "akelayAmahi"]),
    ("10.0205", "loT", Pada::Atmanepada, ["kelayatAm", "kelayetAm", "kelayantAm", "kelayasva", "kelayeTAm", "kelayaDvam", "kelayE", "kelayAvahE", "kelayAmahE"]),
    ("10.0205", "viDiliN", Pada::Atmanepada, ["kelayeta", "kelayeyAtAm", "kelayeran", "kelayeTAH", "kelayeyATAm", "kelayeDvam", "kelayeya", "kelayevahi", "kelayemahi"]),
    ("10.0206", "laT", Pada::Atmanepada, ["pelayate", "pelayete", "pelayante", "pelayase", "pelayeTe", "pelayaDve", "pelaye", "pelayAvahe", "pelayAmahe"]),
    ("10.0206", "laN", Pada::Atmanepada, ["apelayata", "apelayetAm", "apelayanta", "apelayaTAH", "apelayeTAm", "apelayaDvam", "apelaye", "apelayAvahi", "apelayAmahi"]),
    ("10.0206", "loT", Pada::Atmanepada, ["pelayatAm", "pelayetAm", "pelayantAm", "pelayasva", "pelayeTAm", "pelayaDvam", "pelayE", "pelayAvahE", "pelayAmahE"]),
    ("10.0206", "viDiliN", Pada::Atmanepada, ["pelayeta", "pelayeyAtAm", "pelayeran", "pelayeTAH", "pelayeyATAm", "pelayeDvam", "pelayeya", "pelayevahi", "pelayemahi"]),
    ("10.0207", "laT", Pada::Atmanepada, ["vizkayate", "vizkayete", "vizkayante", "vizkayase", "vizkayeTe", "vizkayaDve", "vizkaye", "vizkayAvahe", "vizkayAmahe"]),
    ("10.0207", "laN", Pada::Atmanepada, ["avizkayata", "avizkayetAm", "avizkayanta", "avizkayaTAH", "avizkayeTAm", "avizkayaDvam", "avizkaye", "avizkayAvahi", "avizkayAmahi"]),
    ("10.0207", "loT", Pada::Atmanepada, ["vizkayatAm", "vizkayetAm", "vizkayantAm", "vizkayasva", "vizkayeTAm", "vizkayaDvam", "vizkayE", "vizkayAvahE", "vizkayAmahE"]),
    ("10.0207", "viDiliN", Pada::Atmanepada, ["vizkayeta", "vizkayeyAtAm", "vizkayeran", "vizkayeTAH", "vizkayeyATAm", "vizkayeDvam", "vizkayeya", "vizkayevahi", "vizkayemahi"]),
    ("10.0208", "laT", Pada::Atmanepada, ["hizkayate", "hizkayete", "hizkayante", "hizkayase", "hizkayeTe", "hizkayaDve", "hizkaye", "hizkayAvahe", "hizkayAmahe"]),
    ("10.0208", "laN", Pada::Atmanepada, ["ahizkayata", "ahizkayetAm", "ahizkayanta", "ahizkayaTAH", "ahizkayeTAm", "ahizkayaDvam", "ahizkaye", "ahizkayAvahi", "ahizkayAmahi"]),
    ("10.0208", "loT", Pada::Atmanepada, ["hizkayatAm", "hizkayetAm", "hizkayantAm", "hizkayasva", "hizkayeTAm", "hizkayaDvam", "hizkayE", "hizkayAvahE", "hizkayAmahE"]),
    ("10.0208", "viDiliN", Pada::Atmanepada, ["hizkayeta", "hizkayeyAtAm", "hizkayeran", "hizkayeTAH", "hizkayeyATAm", "hizkayeDvam", "hizkayeya", "hizkayevahi", "hizkayemahi"]),
    ("10.0209", "laT", Pada::Atmanepada, ["nizkayate", "nizkayete", "nizkayante", "nizkayase", "nizkayeTe", "nizkayaDve", "nizkaye", "nizkayAvahe", "nizkayAmahe"]),
    ("10.0209", "laN", Pada::Atmanepada, ["anizkayata", "anizkayetAm", "anizkayanta", "anizkayaTAH", "anizkayeTAm", "anizkayaDvam", "anizkaye", "anizkayAvahi", "anizkayAmahi"]),
    ("10.0209", "loT", Pada::Atmanepada, ["nizkayatAm", "nizkayetAm", "nizkayantAm", "nizkayasva", "nizkayeTAm", "nizkayaDvam", "nizkayE", "nizkayAvahE", "nizkayAmahE"]),
    ("10.0209", "viDiliN", Pada::Atmanepada, ["nizkayeta", "nizkayeyAtAm", "nizkayeran", "nizkayeTAH", "nizkayeyATAm", "nizkayeDvam", "nizkayeya", "nizkayevahi", "nizkayemahi"]),
    ("10.0210", "laT", Pada::Atmanepada, ["lAlayate", "lAlayete", "lAlayante", "lAlayase", "lAlayeTe", "lAlayaDve", "lAlaye", "lAlayAvahe", "lAlayAmahe"]),
    ("10.0210", "laN", Pada::Atmanepada, ["alAlayata", "alAlayetAm", "alAlayanta", "alAlayaTAH", "alAlayeTAm", "alAlayaDvam", "alAlaye", "alAlayAvahi", "alAlayAmahi"]),
    ("10.0210", "loT", Pada::Atmanepada, ["lAlayatAm", "lAlayetAm", "lAlayantAm", "lAlayasva", "lAlayeTAm", "lAlayaDvam", "lAlayE", "lAlayAvahE", "lAlayAmahE"]),
    ("10.0210", "viDiliN", Pada::Atmanepada, ["lAlayeta", "lAlayeyAtAm", "lAlayeran", "lAlayeTAH", "lAlayeyATAm", "lAlayeDvam", "lAlayeya", "lAlayevahi", "lAlayemahi"]),
    ("10.0211", "laT", Pada::Atmanepada, ["kURayate", "kURayete", "kURayante", "kURayase", "kURayeTe", "kURayaDve", "kURaye", "kURayAvahe", "kURayAmahe"]),
    ("10.0211", "laN", Pada::Atmanepada, ["akURayata", "akURayetAm", "akURayanta", "akURayaTAH", "akURayeTAm", "akURayaDvam", "akURaye", "akURayAvahi", "akURayAmahi"]),
    ("10.0211", "loT", Pada::Atmanepada, ["kURayatAm", "kURayetAm", "kURayantAm", "kURayasva", "kURayeTAm", "kURayaDvam", "kURayE", "kURayAvahE", "kURayAmahE"]),
    ("10.0211", "viDiliN", Pada::Atmanepada, ["kURayeta", "kURayeyAtAm", "kURayeran", "kURayeTAH", "kURayeyATAm", "kURayeDvam", "kURayeya", "kURayevahi", "kURayemahi"]),
    ("10.0212", "laT", Pada::Atmanepada, ["tURayate", "tURayete", "tURayante", "tURayase", "tURayeTe", "tURayaDve", "tURaye", "tURayAvahe", "tURayAmahe"]),
    ("10.0212", "laN", Pada::Atmanepada, ["atURayata", "atURayetAm", "atURayanta", "atURayaTAH", "atURayeTAm", "atURayaDvam", "atURaye", "atURayAvahi", "atURayAmahi"]),
    ("10.0212", "loT", Pada::Atmanepada, ["tURayatAm", "tURayetAm", "tURayantAm", "tURayasva", "tURayeTAm", "tURayaDvam", "tURayE", "tURayAvahE", "tURayAmahE"]),
    ("10.0212", "viDiliN", Pada::Atmanepada, ["tURayeta", "tURayeyAtAm", "tURayeran", "tURayeTAH", "tURayeyATAm", "tURayeDvam", "tURayeya", "tURayevahi", "tURayemahi"]),
    ("10.0213", "laT", Pada::Atmanepada, ["BrURayate", "BrURayete", "BrURayante", "BrURayase", "BrURayeTe", "BrURayaDve", "BrURaye", "BrURayAvahe", "BrURayAmahe"]),
    ("10.0213", "laN", Pada::Atmanepada, ["aBrURayata", "aBrURayetAm", "aBrURayanta", "aBrURayaTAH", "aBrURayeTAm", "aBrURayaDvam", "aBrURaye", "aBrURayAvahi", "aBrURayAmahi"]),
    ("10.0213", "loT", Pada::Atmanepada, ["BrURayatAm", "BrURayetAm", "BrURayantAm", "BrURayasva", "BrURayeTAm", "BrURayaDvam", "BrURayE", "BrURayAvahE", "BrURayAmahE"]),
    ("10.0213", "viDiliN", Pada::Atmanepada, ["BrURayeta", "BrURayeyAtAm", "BrURayeran", "BrURayeTAH", "BrURayeyATAm", "BrURayeDvam", "BrURayeya", "BrURayevahi", "BrURayemahi"]),
    ("10.0214", "laT", Pada::Atmanepada, ["SAWayate", "SAWayete", "SAWayante", "SAWayase", "SAWayeTe", "SAWayaDve", "SAWaye", "SAWayAvahe", "SAWayAmahe"]),
    ("10.0214", "laN", Pada::Atmanepada, ["aSAWayata", "aSAWayetAm", "aSAWayanta", "aSAWayaTAH", "aSAWayeTAm", "aSAWayaDvam", "aSAWaye", "aSAWayAvahi", "aSAWayAmahi"]),
    ("10.0214", "loT", Pada::Atmanepada, ["SAWayatAm", "SAWayetAm", "SAWayantAm", "SAWayasva", "SAWayeTAm", "SAWayaDvam", "SAWayE", "SAWayAvahE", "SAWayAmahE"]),
    ("10.0214", "viDiliN", Pada::Atmanepada, ["SAWayeta", "SAWayeyAtAm", "SAWayeran", "SAWayeTAH", "SAWayeyATAm", "SAWayeDvam", "SAWayeya", "SAWayevahi", "SAWayemahi"]),
    ("10.0215", "laT", Pada::Atmanepada, ["yakzayate", "yakzayete", "yakzayante", "yakzayase", "yakzayeTe", "yakzayaDve", "yakzaye", "yakzayAvahe", "yakzayAmahe"]),
    ("10.0215", "laN", Pada::Atmanepada, ["ayakzayata", "ayakzayetAm", "ayakzayanta", "ayakzayaTAH", "ayakzayeTAm", "ayakzayaDvam", "ayakzaye", "ayakzayAvahi", "ayakzayAmahi"]),
    ("10.0215", "loT", Pada::Atmanepada, ["yakzayatAm", "yakzayetAm", "yakzayantAm", "yakzayasva", "yakzayeTAm", "yakzayaDvam", "yakzayE", "yakzayAvahE", "yakzayAmahE"]),
    ("10.0215", "viDiliN", Pada::Atmanepada, ["yakzayeta", "yakzayeyAtAm", "yakzayeran", "yakzayeTAH", "yakzayeyATAm", "yakzayeDvam", "yakzayeya", "yakzayevahi", "yakzayemahi"]),
    ("10.0216", "laT", Pada::Atmanepada, ["syAmayate", "syAmayete", "syAmayante", "syAmayase", "syAmayeTe", "syAmayaDve", "syAmaye", "syAmayAvahe", "syAmayAmahe"]),
    ("10.0216", "laN", Pada::Atmanepada, ["asyAmayata", "asyAmayetAm", "asyAmayanta", "asyAmayaTAH", "asyAmayeTAm", "asyAmayaDvam", "asyAmaye", "asyAmayAvahi", "asyAmayAmahi"]),
    ("10.0216", "loT", Pada::Atmanepada, ["syAmayatAm", "syAmayetAm", "syAmayantAm", "syAmayasva", "syAmayeTAm", "syAmayaDvam", "syAmayE", "syAmayAvahE", "syAmayAmahE"]),
    ("10.0216", "viDiliN", Pada::Atmanepada, ["syAmayeta", "syAmayeyAtAm", "syAmayeran", "syAmayeTAH", "syAmayeyATAm", "syAmayeDvam", "syAmayeya", "syAmayevahi", "syAmayemahi"]),
    ("10.0217", "laT", Pada::Atmanepada, ["gUrayate", "gUrayete", "gUrayante", "gUrayase", "gUrayeTe", "gUrayaDve", "gUraye", "gUrayAvahe", "gUrayAmahe"]),
    ("10.0217", "laN", Pada::Atmanepada, ["agUrayata", "agUrayetAm", "agUrayanta", "agUrayaTAH", "agUrayeTAm", "agUrayaDvam", "agUraye", "agUrayAvahi", "agUrayAmahi"]),
    ("10.0217", "loT", Pada::Atmanepada, ["gUrayatAm", "gUrayetAm", "gUrayantAm", "gUrayasva", "gUrayeTAm", "gUrayaDvam", "gUrayE", "gUrayAvahE", "gUrayAmahE"]),
    ("10.0217", "viDiliN", Pada::Atmanepada, ["gUrayeta", "gUrayeyAtAm", "gUrayeran", "gUrayeTAH", "gUrayeyATAm", "gUrayeDvam", "gUrayeya", "gUrayevahi", "gUrayemahi"]),
    ("10.0218", "laT", Pada::Atmanepada, ["SAmayate", "SAmayete", "SAmayante", "SAmayase", "SAmayeTe", "SAmayaDve", "SAmaye", "SAmayAvahe", "SAmayAmahe"]),
    ("10.0218", "laN", Pada::Atmanepada, ["aSAmayata", "aSAmayetAm", "aSAmayanta", "aSAmayaTAH", "aSAmayeTAm", "aSAmayaDvam", "aSAmaye", "aSAmayAvahi", "aSAmayAmahi"]),
    ("10.0218", "loT", Pada::Atmanepada, ["SAmayatAm", "SAmayetAm", "SAmayantAm", "SAmayasva", "SAmayeTAm", "SAmayaDvam", "SAmayE", "SAmayAvahE", "SAmayAmahE"]),
    ("10.0218", "viDiliN", Pada::Atmanepada, ["SAmayeta", "SAmayeyAtAm", "SAmayeran", "SAmayeTAH", "SAmayeyATAm", "SAmayeDvam", "SAmayeya", "SAmayevahi", "SAmayemahi"]),
    ("10.0219", "laT", Pada::Atmanepada, ["lakzayate", "lakzayete", "lakzayante", "lakzayase", "lakzayeTe", "lakzayaDve", "lakzaye", "lakzayAvahe", "lakzayAmahe"]),
    ("10.0219", "laN", Pada::Atmanepada, ["alakzayata", "alakzayetAm", "alakzayanta", "alakzayaTAH", "alakzayeTAm", "alakzayaDvam", "alakzaye", "alakzayAvahi", "alakzayAmahi"]),
    ("10.0219", "loT", Pada::Atmanepada, ["lakzayatAm", "lakzayetAm", "lakzayantAm", "lakzayasva", "lakzayeTAm", "lakzayaDvam", "lakzayE", "lakzayAvahE", "lakzayAmahE"]),
    ("10.0219", "viDiliN", Pada::Atmanepada, ["lakzayeta", "lakzayeyAtAm", "lakzayeran", "lakzayeTAH", "lakzayeyATAm", "lakzayeDvam", "lakzayeya", "lakzayevahi", "lakzayemahi"]),
    ("10.0220", "laT", Pada::Atmanepada, ["kutsayate", "kutsayete", "kutsayante", "kutsayase", "kutsayeTe", "kutsayaDve", "kutsaye", "kutsayAvahe", "kutsayAmahe"]),
    ("10.0220", "laN", Pada::Atmanepada, ["akutsayata", "akutsayetAm", "akutsayanta", "akutsayaTAH", "akutsayeTAm", "akutsayaDvam", "akutsaye", "akutsayAvahi", "akutsayAmahi"]),
    ("10.0220", "loT", Pada::Atmanepada, ["kutsayatAm", "kutsayetAm", "kutsayantAm", "kutsayasva", "kutsayeTAm", "kutsayaDvam", "kutsayE", "kutsayAvahE", "kutsayAmahE"]),
    ("10.0220", "viDiliN", Pada::Atmanepada, ["kutsayeta", "kutsayeyAtAm", "kutsayeran", "kutsayeTAH", "kutsayeyATAm", "kutsayeDvam", "kutsayeya", "kutsayevahi", "kutsayemahi"]),
    ("10.0221", "laT", Pada::Atmanepada, ["trowayate", "trowayete", "trowayante", "trowayase", "trowayeTe", "trowayaDve", "trowaye", "trowayAvahe", "trowayAmahe"]),
    ("10.0221", "laN", Pada::Atmanepada, ["atrowayata", "atrowayetAm", "atrowayanta", "atrowayaTAH", "atrowayeTAm", "atrowayaDvam", "atrowaye", "atrowayAvahi", "atrowayAmahi"]),
    ("10.0221", "loT", Pada::Atmanepada, ["trowayatAm", "trowayetAm", "trowayantAm", "trowayasva", "trowayeTAm", "trowayaDvam", "trowayE", "trowayAvahE", "trowayAmahE"]),
    ("10.0221", "viDiliN", Pada::Atmanepada, ["trowayeta", "trowayeyAtAm", "trowayeran", "trowayeTAH", "trowayeyATAm", "trowayeDvam", "trowayeya", "trowayevahi", "trowayemahi"]),
    ("10.0222", "laT", Pada::Atmanepada, ["kowayate", "kowayete", "kowayante", "kowayase", "kowayeTe", "kowayaDve", "kowaye", "kowayAvahe", "kowayAmahe"]),
    ("10.0222", "laN", Pada::Atmanepada, ["akowayata", "akowayetAm", "akowayanta", "akowayaTAH", "akowayeTAm", "akowayaDvam", "akowaye", "akowayAvahi", "akowayAmahi"]),
    ("10.0222", "loT", Pada::Atmanepada, ["kowayatAm", "kowayetAm", "kowayantAm", "kowayasva", "kowayeTAm", "kowayaDvam", "kowayE", "kowayAvahE", "kowayAmahE"]),
    ("10.0222", "viDiliN", Pada::Atmanepada, ["kowayeta", "kowayeyAtAm", "kowayeran", "kowayeTAH", "kowayeyATAm", "kowayeDvam", "kowayeya", "kowayevahi", "kowayemahi"]),
    ("10.0223", "laT", Pada::Atmanepada, ["gAlayate", "gAlayete", "gAlayante", "gAlayase", "gAlayeTe", "gAlayaDve", "gAlaye", "gAlayAvahe", "gAlayAmahe"]),
    ("10.0223", "laN", Pada::Atmanepada, ["agAlayata", "agAlayetAm", "agAlayanta", "agAlayaTAH", "agAlayeTAm", "agAlayaDvam", "agAlaye", "agAlayAvahi", "agAlayAmahi"]),
    ("10.0223", "loT", Pada::Atmanepada, ["gAlayatAm", "gAlayetAm", "gAlayantAm", "gAlayasva", "gAlayeTAm", "gAlayaDvam", "gAlayE", "gAlayAvahE", "gAlayAmahE"]),
    ("10.0223", "viDiliN", Pada::Atmanepada, ["gAlayeta", "gAlayeyAtAm", "gAlayeran", "gAlayeTAH", "gAlayeyATAm", "gAlayeDvam", "gAlayeya", "gAlayevahi", "gAlayemahi"]),
    ("10.0224", "laT", Pada::Atmanepada, ["BAlayate", "BAlayete", "BAlayante", "BAlayase", "BAlayeTe", "BAlayaDve", "BAlaye", "BAlayAvahe", "BAlayAmahe"]),
    ("10.0224", "laN", Pada::Atmanepada, ["aBAlayata", "aBAlayetAm", "aBAlayanta", "aBAlayaTAH", "aBAlayeTAm", "aBAlayaDvam", "aBAlaye", "aBAlayAvahi", "aBAlayAmahi"]),
    ("10.0224", "loT", Pada::Atmanepada, ["BAlayatAm", "BAlayetAm", "BAlayantAm", "BAlayasva", "BAlayeTAm", "BAlayaDvam", "BAlayE", "BAlayAvahE", "BAlayAmahE"]),
    ("10.0224", "viDiliN", Pada::Atmanepada, ["BAlayeta", "BAlayeyAtAm", "BAlayeran", "BAlayeTAH", "BAlayeyATAm", "BAlayeDvam", "BAlayeya", "BAlayevahi", "BAlayemahi"]),
    ("10.0225", "laT", Pada::Atmanepada, ["kUwayate", "kUwayete", "kUwayante", "kUwayase", "kUwayeTe", "kUwayaDve", "kUwaye", "kUwayAvahe", "kUwayAmahe"]),
    ("10.0225", "laN", Pada::Atmanepada, ["akUwayata", "akUwayetAm", "akUwayanta", "akUwayaTAH", "akUwayeTAm", "akUwayaDvam", "akUwaye", "akUwayAvahi", "akUwayAmahi"]),
    ("10.0225", "loT", Pada::Atmanepada, ["kUwayatAm", "kUwayetAm", "kUwayantAm", "kUwayasva", "kUwayeTAm", "kUwayaDvam", "kUwayE", "kUwayAvahE", "kUwayAmahE"]),
    ("10.0225", "viDiliN", Pada::Atmanepada, ["kUwayeta", "kUwayeyAtAm", "kUwayeran", "kUwayeTAH", "kUwayeyATAm", "kUwayeDvam", "kUwayeya", "kUwayevahi", "kUwayemahi"]),
    ("10.0226", "laT", Pada::Atmanepada, ["kuwwayate", "kuwwayete", "kuwwayante", "kuwwayase", "kuwwayeTe", "kuwwayaDve", "kuwwaye", "kuwwayAvahe", "kuwwayAmahe"]),
    ("10.0226", "laN", Pada::Atmanepada, ["akuwwayata", "akuwwayetAm", "akuwwayanta", "akuwwayaTAH", "akuwwayeTAm", "akuwwayaDvam", "akuwwaye", "akuwwayAvahi", "akuwwayAmahi"]),
    ("10.0226", "loT", Pada::Atmanepada, ["kuwwayatAm", "kuwwayetAm", "kuwwayantAm", "kuwwayasva", "kuwwayeTAm", "kuwwayaDvam", "kuwwayE", "kuwwayAvahE", "kuwwayAmahE"]),
    ("10.0226", "viDiliN", Pada::Atmanepada, ["kuwwayeta", "kuwwayeyAtAm", "kuwwayeran", "kuwwayeTAH", "kuwwayeyATAm", "kuwwayeDvam", "kuwwayeya", "kuwwayevahi", "kuwwayemahi"]),
```

**(b)** After the `("10.0229", "viDiliN", …)` row and before the `("10.0236", "laT", …)` row, insert:

```rust
    ("10.0232", "laT", Pada::Atmanepada, ["vedayate", "vedayete", "vedayante", "vedayase", "vedayeTe", "vedayaDve", "vedaye", "vedayAvahe", "vedayAmahe"]),
    ("10.0232", "laN", Pada::Atmanepada, ["avedayata", "avedayetAm", "avedayanta", "avedayaTAH", "avedayeTAm", "avedayaDvam", "avedaye", "avedayAvahi", "avedayAmahi"]),
    ("10.0232", "loT", Pada::Atmanepada, ["vedayatAm", "vedayetAm", "vedayantAm", "vedayasva", "vedayeTAm", "vedayaDvam", "vedayE", "vedayAvahE", "vedayAmahE"]),
    ("10.0232", "viDiliN", Pada::Atmanepada, ["vedayeta", "vedayeyAtAm", "vedayeran", "vedayeTAH", "vedayeyATAm", "vedayeDvam", "vedayeya", "vedayevahi", "vedayemahi"]),
    ("10.0233", "laT", Pada::Atmanepada, ["mAnayate", "mAnayete", "mAnayante", "mAnayase", "mAnayeTe", "mAnayaDve", "mAnaye", "mAnayAvahe", "mAnayAmahe"]),
    ("10.0233", "laN", Pada::Atmanepada, ["amAnayata", "amAnayetAm", "amAnayanta", "amAnayaTAH", "amAnayeTAm", "amAnayaDvam", "amAnaye", "amAnayAvahi", "amAnayAmahi"]),
    ("10.0233", "loT", Pada::Atmanepada, ["mAnayatAm", "mAnayetAm", "mAnayantAm", "mAnayasva", "mAnayeTAm", "mAnayaDvam", "mAnayE", "mAnayAvahE", "mAnayAmahE"]),
    ("10.0233", "viDiliN", Pada::Atmanepada, ["mAnayeta", "mAnayeyAtAm", "mAnayeran", "mAnayeTAH", "mAnayeyATAm", "mAnayeDvam", "mAnayeya", "mAnayevahi", "mAnayemahi"]),
    ("10.0234", "laT", Pada::Atmanepada, ["mAnayate", "mAnayete", "mAnayante", "mAnayase", "mAnayeTe", "mAnayaDve", "mAnaye", "mAnayAvahe", "mAnayAmahe"]),
    ("10.0234", "laN", Pada::Atmanepada, ["amAnayata", "amAnayetAm", "amAnayanta", "amAnayaTAH", "amAnayeTAm", "amAnayaDvam", "amAnaye", "amAnayAvahi", "amAnayAmahi"]),
    ("10.0234", "loT", Pada::Atmanepada, ["mAnayatAm", "mAnayetAm", "mAnayantAm", "mAnayasva", "mAnayeTAm", "mAnayaDvam", "mAnayE", "mAnayAvahE", "mAnayAmahE"]),
    ("10.0234", "viDiliN", Pada::Atmanepada, ["mAnayeta", "mAnayeyAtAm", "mAnayeran", "mAnayeTAH", "mAnayeyATAm", "mAnayeDvam", "mAnayeya", "mAnayevahi", "mAnayemahi"]),
```

Optional cross-check of the transcription: the throwaway example `/tmp/vidyut-full/vidyut-prakriya/examples/curadi_goldens_10c.rs` re-emits all 148 ākusmīya golden rows (10b's 16 and these 132) from vidyut. Its stdout's sha256 was `7813612803ee8364382129efa4ea8e9d17db380545af48e129c4a60900897ec9` in the prototype.

- [ ] **Step 7: Run the full suite**

```bash
mise run fmt
mise run test 2>&1 | grep -E "FAILED|test result" | head -20
```

Foreground, timeout 600000 ms. Expected: PASS at 6264 cells. `panini-data` 23 tests, `paradigm` 22, `trace` 196, `panini-prakriya` 403. Then re-run Step 3's grep loop: thirteen forms print `curadi.rs` only, and the four parasmaipada shapes print nothing.

`pada_ambiguous_surfaces_are_exactly_these` needs no change. Every new form is ātmanepada, and none equals any prior golden; the prototype grepped all 1188.

- [ ] **Step 8: Commit**

```bash
mise run lint
git add -A
git commit -m "feat(data): curādi's remaining ākusmīya bulk — 33 roots (10.0195–10.0234), ātmanepadī by 10.0496

5076 → 6264 cells, 6206 → 7394 forms, ALTERNATES unchanged at 1130, 111 → 144
roots. Homograph rows name their ubhayapadī partners; √mān/√man share every
form; √syam/√śam are the mit slice's 10.0494 witnesses. Trace tests now walk
the AKUSMIYA range (37 rows, 1332 credits)."
```

---

## Task 4: Audit, prior-trace diff, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (audit prose), `crates/panini-prakriya/src/tinanta/guna.rs` (one comment), `docs/superpowers/specs/2026-10-02-curadi-gana-10b-design.md`

**Interfaces:**
- Consumes: the finished data and goldens from Task 3. Produces no symbols.

- [ ] **Step 1: Update the audit harness**

In `tools/audit/panini_full_audit.rs` (each `old` occurs once):
- `//! What it compares: for each of the 111 curated roots,` → `144`.
- `//! Corpus invariants, asserted: 111 roots, 5076 cells, 6206 forms.` → `144 roots, 6264 cells, 7394 forms.`
- ``(`derivation_set_shape_matches_the_audited_numbers`): 564 root×pada×lakāra`` → `696`.
- `//! Optionally dump the full 5076-cell table:` → `6264-cell`.
- `assert_eq!(roots_seen.len(), 111, "curated roots");` → `144`.
- `assert_eq!(n_cells, 5076, "cells: 564 root×pada×lakāra blocks × 9");` → `assert_eq!(n_cells, 6264, "cells: 696 root×pada×lakāra blocks × 9");`
- `assert_eq!(n_forms, 6206, "forms: 5076 cells + 1130 ALTERNATES rows");` → `assert_eq!(n_forms, 7394, "forms: 6264 cells + 1130 ALTERNATES rows");`

The both-pada clause (thirty roots) is unchanged, because all 33 new roots are ātmanepada-only.

In `tools/audit/README.md`, `(111 roots, 5076 cells, 6206 forms)` → `(144 roots, 6264 cells, 7394 forms)`.

- [ ] **Step 2: Repoint vidyut's dev-deps at THIS worktree, run the prior-trace diff, the engine-vs-vidyut probe and the audit**

`/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes absolute dev-dep paths to `/workspace/crates`, the `main` checkout, which lacks this slice. An audit without repointing checks the pre-slice engine and passes vacuously.

Two throwaway examples exist from the prototype: `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_10c.rs` and `curadi_engine_10c.rs`. If `trace_dump_10c.rs` is missing, recreate it:

```rust
//! THROWAWAY: slice 10c — dump every prior cell's live-branch credited-rule log.
use panini::Panini;
use panini_data::{Lakara as L, Purusha as P, Vacana as V};
const NEW: [&str; 33] = [
    "10.0195", "10.0196", "10.0197", "10.0200", "10.0201", "10.0202", "10.0203", "10.0204",
    "10.0205", "10.0206", "10.0207", "10.0208", "10.0209", "10.0210", "10.0211", "10.0212",
    "10.0213", "10.0214", "10.0215", "10.0216", "10.0217", "10.0218", "10.0219", "10.0220",
    "10.0221", "10.0222", "10.0223", "10.0224", "10.0225", "10.0226", "10.0232", "10.0233",
    "10.0234",
];
fn main() {
    let panini = Panini::new();
    for d in panini_data::dhatus() {
        if NEW.contains(&d.dhatupatha) { continue; }
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
grep -n '^panini' $V/Cargo.toml   # must point at /workspace/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10c 2>/dev/null > "$DUMP/main.txt")
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml   # must point at $WT/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10c 2>/dev/null > "$DUMP/branch.txt")
wc -l < "$DUMP/main.txt"          # 6206
diff "$DUMP/main.txt" "$DUMP/branch.txt" && echo PRIORS-IDENTICAL
```

Expected: `6206`, then `PRIORS-IDENTICAL`. If the diff prints any line, stop and report.

If `curadi_engine_10c.rs` exists, run it too (`cargo run -q --release --example curadi_engine_10c 2>/dev/null | grep -v ^FORM | tail -3`). Expected last line: `33 rows, 1188 cells, 0 differences`. It builds its own `Dhatu` values, so it is independent of the repoint; it re-confirms the probe the spec cites.

Then the audit. Copy the committed harness; never rewrite it.

```bash
cp tools/audit/panini_full_audit.rs $V/examples/
(cd $V && PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -6)
(cd $V && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -4)
```

- Expected from the honest run: `AUDIT PASSED: 6264 cells, 7394 forms, zero differences.`
- Expected from the `entry` control: `AUDIT FAILED: 36 differing cells.`

Do not use `mise -C`. If the honest run shows differences, stop and report, and edit nothing. After all runs, restore the dev-deps:

```bash
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml
```

- [ ] **Step 3: Record the audit**

In `tools/audit/README.md`, immediately under `## Last recorded result` and its blank line, add a new entry above the 10b one. Set `<DATE>` from `date -u +%F`:

```markdown
<DATE>, curādi 10c slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 6264
cells / 7394 forms / 144 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole curādi 10c slice: thirty-three more rows of the
ākusmīya antargaṇa (`10.0195` through `10.0234`), ātmanepadī by 10.0496,
with no engine change. It includes six rows homographic with uncurated
ubhayapadī curādi rows and the pair `10.0233 mAna~` / `10.0234 mana~`, which
share every form. vidyut credits the gaṇasūtra 10.0494 on `10.0216` and
`10.0218`. It changes no form there, and this engine does not implement it;
the harness compares forms, so it agrees. A main-vs-branch dump of every prior
cell's traces was byte-identical.

Totals: 144 = 111 + 33; 6264 = 5076 + 1188 (132 root×pada×lakāra blocks ×
9); 7394 = 6206 + 1188, with no new `ALTERNATES` rows (1130), measured via
the harness's corpus block, not assumed.

```

In `crates/panini/tests/paradigm/main.rs`'s audit-chain doc comment, replace

```rust
/// same commit over all 5076 cells / 6206 forms / 111 roots with zero
/// differences, its `entry` negative control verified failing (36 √bhū
/// cells). √tṛh joins none of the fork
```

with

```rust
/// same commit over all 5076 cells / 6206 forms / 111 roots with zero
/// differences, its `entry` negative control verified failing (36 √bhū
/// cells), and curādi 10c's re-ran it at the same commit over all 6264 cells
/// / 7394 forms / 144 roots with zero differences, its `entry` negative
/// control verified failing (36 √bhū cells). √tṛh joins none of the fork
```

- [ ] **Step 4: README.md**

Confirm each `old` occurs exactly once with `grep -c` before editing.
- `8.4.44 *śāt* exemption. *curādi* (10) is **open** at 8 of its 509` → `8.4.44 *śāt* exemption. *curādi* (10) is **open** at 41 of its 509`
- Replace `ātmanepadinaḥ* — the engine's one rule that is not an Aṣṭādhyāyī sūtra.` with:

```markdown
ātmanepadinaḥ* — the engine's one rule that is not an Aṣṭādhyāyī sūtra.
Slice 10c curated thirty-three more ākusmīya rows in bulk (`10.0195` through
`10.0234`), among them √vid (*vedayate*), √śam (*śāmayate*) and the pair √mān
/ √man, which share every form (*mānayate*).
```

- `curated 111-root set,` → `curated 144-root set,`
- `824 of the 5076 cells hold more than one form` → `824 of the 6264 cells hold more than one form`

The both-pada list (thirty roots) and the pada-ambiguous list (seventy-two) are unchanged.

- [ ] **Step 5: docs/ARCHITECTURE.md**

Each `old` occurs exactly once.
- `` **open** at 8 of its `` / `509 rows (√cur, √laḍ, √bhakṣ, √bhūṣ; slice 10a; √cit, √vṛṣ, √mad, √kusm; slice 10b). gaṇa` → `**open** at 41 of its` / `509 rows (√cur, √laḍ, √bhakṣ, √bhūṣ; slice 10a; √cit, √vṛṣ, √mad, √kusm; slice 10b; thirty-three more ākusmīya roots, slice 10c). gaṇa`
- `the curated set's 29 ātmanepada-only` → `the curated set's 62 ātmanepada-only`
- `82 + 29 = the 111 curated roots` → `82 + 62 = the 144 curated roots`

Check: `grep -c "pada: PadaAssignment::Atmanepada,\|pada: PadaAssignment::Akusmiya," crates/panini-data/src/lib.rs` prints `62`. The 82-root / 164-cell parasmaipada census and the thirty both-pada roots are unchanged.

- [ ] **Step 6: AGENTS.md**

Each `old` occurs exactly once.
- `` (`crates/panini/tests/paradigm/`, 5076 cells, ten gaṇas, nine complete — `` → `6264 cells`
- `√bhūṣ), at 8 after slice 10b curated the ākusmīya √cit, √vṛṣ, √mad and √kusm —` → `√bhūṣ), at 8 after slice 10b curated the ākusmīya √cit, √vṛṣ, √mad and √kusm, at 41 after slice 10c curated thirty-three more ākusmīya roots —`
- `so 5076 + 1130 = 6206 forms total` → `so 6264 + 1130 = 7394 forms total`
- The audit chain. Replace `` (`tools/audit/README.md`'s 2026-10-02 10b entry, 5076 cells / 6206 forms / 111 `` / `  roots).` with `` (`tools/audit/README.md`'s 2026-10-02 10b entry, 5076 cells / 6206 forms / 111 `` / `` roots), and that by curādi 10c's (`tools/audit/README.md`'s <DATE> 10c entry, `` / `6264 cells / 7394 forms / 144 roots).`
- `not wrong in kind: 5076 goldens` → `not wrong in kind: 6264 goldens`
- The stale-comment ledger. After the sentence ``Curādi 10b touched neither comment either; the corpus stands at 5076 cells as of 10b (`guna.rs:2565`'s claim anchored at `guna.rs:2565`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit).``, insert ``Curādi 10c touched neither comment either; the corpus stands at 6264 cells as of 10c (`guna.rs:2565`'s claim anchored at `guna.rs:<G>`, `controller.rs:206`'s at `controller.rs:<C>`; both lines measured by grep at this commit).`` Measure `<G>` with `grep -n "1872 goldens move" crates/panini-prakriya/src/tinanta/guna.rs` and `<C>` with `grep -n "only 8 cells fire" crates/panini-prakriya/src/controller.rs`; both were 2565 and 206 in the prototype. Measure; never compute.

AGENTS.md's floor paragraph (`measured at 5076 cells`) and the current mutation record belong to Task 5.

- [ ] **Step 7: The engine comment and the 10b spec**

`crates/panini-prakriya/src/tinanta/guna.rs`, 6.1.78's comment: `111-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)` → `144-root × 4-lakāra grammar, …`. The claim still holds: every new aṅga ends in ṇic's `i`. This is an in-place edit; no line shifts.

`docs/superpowers/specs/2026-10-02-curadi-gana-10b-design.md`:
- After the out-of-scope bullet ending `` (`10.0231 gf`, `10.0235 yu`); ``, which begins `- the other ākusmīya rows.`, append to that bullet: ` Slice 10c curated thirty-three of them in bulk — see` / `` `2026-10-02-curadi-gana-10c-design.md`. ``
- Append to the `` - `10.0219 lakza~`, deliberately: `` bullet: ` (Reversed in slice 10c: rows are keyed by number, so the homograph needs` / `no structural support, and 10c curated it.)`
- In "Later slices", after `The remaining ākusmīya rows in bulk;` insert ` (taken by slice 10c, all but the optional-ṇic and 7.2.115 rows)`.

- [ ] **Step 8: Sweep for anything left stale**

```bash
grep -rn "\b5076\b\|\b6206\b\|\b564\b\|\b4252\b\|111 roots\|111-root\|of these 111\|of the 111\|111 curated\|8 of 509\|8 of its 509\|29 ātmanepada\|82 + 29\|four ākusmīya" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.md --include=*.rs | grep -v "paradigm/data/"
grep -rn "√cit, √vṛṣ, √mad\|cit.*vfz.*mad.*kusm\|ākusmīya" README.md AGENTS.md docs/ARCHITECTURE.md crates/panini-prakriya/src crates/panini-data/src --include=*.md --include=*.rs
```

Expected residue in the first grep:
- AGENTS.md's floor paragraph (`measured at 5076 cells`), which Task 5 rewrites;
- dated history, which must never be rewritten: AGENTS.md's mutation record and audit chain; `tools/audit/README.md`'s older entries and the 10c entry's own `111 + 33` / `5076 + 1188` / `6206 + 1188` totals; the audit chain and the 10b paragraph in `paradigm/main.rs`; the 10b test comment "four ākusmīya roots" in `curadi_rows_are_the_forty_one_curated_roots` and in the curādi block comment in `lib.rs`. Each of these describes 10b's step and stays true.

For every hit of the second grep, check that a sentence enumerating "the ākusmīya roots" as 10b's four is either explicitly 10b's or updated.

- [ ] **Step 9: Run the full suite and commit**

Run: `mise run test 2>&1 | grep -E "FAILED|test result" | head -20` (foreground, timeout 600000 ms). Expected: PASS at 6264 cells.

```bash
mise run fmt && mise run lint
git add -A
git commit -m "docs: 10c's counts, the audit record, and the sweep

6264 cells / 7394 forms / 144 roots across README, ARCHITECTURE, AGENTS,
paradigm/main.rs and tools/audit; curādi open at 41/509; 62 ātmanepada-only.
Audit at zero divergence against 8da2f90b; prior traces byte-identical to main."
```

---

## Task 5: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and the current-record paragraph); `mise.toml` only if the cap moves

Follow AGENTS.md's cargo-mutants protocol. Hazards from this repo's record:
- **Measure, never scale.**
- **Every invocation rotates `mutants.out`**, so always pass `-o`.
- **The mise shim fails in background shells.** Use the real binary: `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`.
- **`pgrep -f` matches its own shell.** Wait on `pgrep -x cargo-mutants`.
- **Background shells die at about 60 minutes.** Launch detached with `setsid nohup`, as below.
- **This slice edits no mutated source except one in-place comment in `guna.rs`.** So the three documented non-caught entries (`adesha.rs:589:30`, `tripadi.rs:1289:38`, `tripadi.rs:1602:23`) should not move. Confirm them by `--list`, never by assumption.

What can move is the **floor**: the suite has 1188 more cells, so an uncaught full run takes longer. The cap is 170.

- [ ] **Step 1: Measure the floor**

With nothing else running, run this twice: `time mise run test 2>&1 | tail -3` (foreground). Record both wall clocks and `cat /proc/loadavg`. The 5076-cell floor was 23.322s / 22.548s, under host load of about 60.

- [ ] **Step 2: Locate and probe the two uncaught equivalents at `-j 4`**

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /="
```

Expect positions `589` (adesha), `1289` (tripadi, inside 8.3.13's `apply`) and `1602` (the ṇatva hang). Write them as `<A>`, `<T1>` and `<T2>`. Then run in the foreground with timeout 600000 ms:

```bash
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 170 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:<A>:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" 2>&1 | tail -10
```

Both must be MISSED, not TIMEOUT. Read each test-phase duration from `$SCRATCH/mutants.out/outcomes.json`.

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/curadi-10c"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout 170 -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"
```

The last campaign took about 27 minutes under heavy host load. Run nothing CPU-heavy meanwhile. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

- [ ] **Step 4: Read the outcomes**

When `pgrep -x cargo-mutants` returns nothing:

```bash
date -u +"%F %T UTC" > "$OUT/finished"
cp "$OUT/mutants.out/outcomes.json" "$OUT/outcomes.durable.json"
tail -5 "$OUT/campaign.log"
cat "$OUT/mutants.out/missed.txt" "$OUT/mutants.out/timeout.txt"
```

Expected (exit code 3 is normal when a timeout is present):
- **815 mutants**, the same count as 10b's: the mutated sources are unchanged.
- `missed.txt` holds exactly `adesha.rs:<A>:30: replace + with *` and `tripadi.rs:<T1>:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:<T2>:23: replace -= with /=`.
- panini-analyze: 0 missed, 0 timeout.

If not:
- Any **other timeout** is a suspect survivor that the larger suite pushed past the cap. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant means a test that caught it at 5076 cells no longer does. That should be impossible in a data-only slice; stop and report.

- [ ] **Step 5: Margins**

```bash
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["outcomes"][1])' "$OUT/mutants.out/outcomes.json"
```

Compute the two equivalents' test phases under campaign load, and the caught min/median/p90/max. The cap is max(170, 6 × the longest campaign-load equivalent phase, rounded up to the next 10 s).
- If that is 170, the cap stays.
- Otherwise change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together.

- [ ] **Step 6: Record it in AGENTS.md**

- Rewrite the paragraph that opens `**The floor behind the 170s cap, measured at 5076 cells on Rust 1.99.0,` / `2026-10-02.**`. Use Step 1's and Step 2's numbers at 6264 cells, the load averages, and the cap Step 5 chose. Keep the comparison chain to earlier floors, with 10b's 23.322s / 22.548s at 5076 cells joining it.
- Replace the `**Current record (curādi 10b, 2026-10-02).**` paragraph with `**Current record (curādi 10c, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**;
  - the non-caught set diffed against 10b's (the clean result is identical, lines and columns included, since no mutated source line moved);
  - the campaign-load phases and margins;
  - that this slice changed no mutated code, so it has no new-code mutants to report;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`.

  End it with a pointer to the record it replaces. Run `git rev-parse --short HEAD` before committing, and write ``The curādi 10b record it replaces: `git show <that hash>:AGENTS.md`.``

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md mise.toml
git commit -m "chore: 10c mutation gate — floor and uncaught run re-measured at 6264 cells

missed.txt holds only the two documented equivalents and timeout.txt only
the permanent ṇatva-scan entry; the non-caught set is identical to 10b's."
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
git push -u origin curadi-10c
gh pr create --title "curādi 10c — the ākusmīya roots in bulk" --body "$(cat <<'BODY'
Slice 10c curates thirty-three more rows of curādi's ākusmīya antargaṇa
(`10.0195`–`10.0234`), ātmanepadī by the gaṇasūtra 10.0496 that 10b built.
It is a data-only slice: no engine change and no new rule id. The golden
suite goes from 5076 to 6264 cells; curādi is open at 41 of 509.

- **Homographs are in.** Six rows share an upadeśa with an uncurated
  ubhayapadī curādi row; this reverses 10b's hold on `lakza~`. √mān and
  √man share every form, and `check("mAnayate")` returns both.
- **√syam and √śam come without 10.0494.** vidyut credits it, but it only
  blocks the mittva rule 01.0934, which this engine lacks. These rows are
  the witnesses that will force the mit slice to add it.
- **One test-helper fix.** `stored_form` now models 7.1.58 for every idit
  upadeśa, so `10.0195 dasa~` resolves uniquely against `10.0194 dasi~`.

The audit shows zero divergence against `8da2f90b`; a main-vs-branch dump
of every prior cell's traces is byte-identical; the mutation gate is clean.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`, run `git worktree remove .worktrees/curadi-10c` and `git worktree remove --force .worktrees/curadi-10c-proto` (the throwaway). Then delete the local and remote `curadi-10c` branch, and run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| 33 rows, `Akusmiya`, comments (pre-ṇic shape, homograph partner, 10.0494 witness, √mān/√man) | 3 |
| Rows and goldens in dhātupāṭha order, interleaved with 10b's | 3 |
| 1188 goldens from vidyut, single-form; totals 144 / 6264 / 7394; ALTERNATES 1130 | 3 |
| `curated_roots…` 144, the curādi row-list test at 41, `pada` doc 102 of 144 / 37 ākusmīya | 3 |
| Trace tests on the `AKUSMIYA` range: 1332 credits, 37-row parasmaipada block | 3 |
| `check()` witnesses incl. the √mān/√man pair; parasmaipada shapes Invalid | 3 |
| `stored_form` generalized to every idit upadeśa, with its unit test (amendment) | 2 |
| No 10.0494; no engine change | Global Constraints; 3 (comments) |
| Prior traces byte-identical, main ↔ branch | 4 Step 2 |
| Audit with repoint and negative control | 4 |
| README / ARCHITECTURE / AGENTS counts, 29 → 62 ātmanepada-only, 10b spec pointers | 4 |
| Floor, uncaught probe, campaign, verbatim non-caught record | 5 |

**Spec deviations, recorded:**
- **The curādi row-list test** (`curadi_rows_are_the_eight_curated_roots`) is renamed `…_forty_one_…` and lists all 41 rows. The spec named only `curated_roots_have_expected_ganas_and_padas`.
- **The `check()` test name** is `curadi_analyses_its_bulk_akusmiya_forms`, alongside 10b's `curadi_analyses_its_akusmiya_forms`.

**Type consistency.**
- `AKUSMIYA: RangeInclusive<&str>` is read as `AKUSMIYA.contains(&d.dhatupatha)` (a `&&str`) in the walk, and `AKUSMIYA.contains(number)` with `number: &&'static str` in the credit loop. Both compiled in the prototype.
- `stored_form(&str) -> String` keeps its signature.
- The golden tuple shape `(&str, &str, Pada, [&str; 9])` matches `ParadigmRow`.

**Known soft spots.**
- **Doc strings in Task 4** were read at `54a5fcb`. If one is not found exactly once, edit the paragraph to the same facts rather than skip it.
- **The floor in Task 5** depends on host load. 10b's was measured at a load average of about 60. Record the load beside every timing.
