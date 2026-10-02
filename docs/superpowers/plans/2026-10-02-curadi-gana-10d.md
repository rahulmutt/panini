# Curādi gaṇa slice 10d Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate six of curādi's seven jñapādi mit roots (`10.0118`–`10.0123`: √jñap, √yam, √cah, √cap, √rah, √bal) in both padas. Add the gaṇasūtra 10.0493 *jñapādayo mitaḥ* and 6.4.92 *mitāṃ hrasvaḥ*, so that *jñapayati* derives, not *jñāpayati*. The golden suite goes from 6264 to 6696 cells, and curādi from 41 to 47 of its 509 rows.

**Architecture:** Six tasks:
- **Task 1** checks the worktree and baseline.
- **Task 2** is the engine, landing green on its own because no curated row is mit yet:
  - the data range `JNAPADI`, with its upstream pin;
  - `Tag::Mit`, set by `derive`;
  - the two sanādi rules, their unit tests, and the rule-order pin.
- **Task 3** lands the six rows, their 48 golden rows and 36 alternates, and every count, list, trace and `check()` assertion they move. These must land together: the suite is red between any of them.
- **Tasks 4–6** are the audit with the prior-trace diff and the doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.99.0, pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, checked out at `/tmp/vidyut-full`.

**Spec:** `docs/superpowers/specs/2026-10-02-curadi-gana-10d-design.md`. Read its "Prototype (amendment)" paragraph and the amended Tests section; they postdate the first draft.

**Workspace:** the branch `curadi-10d` is checked out at `/workspace/.worktrees/curadi-10d` and holds the spec and this plan. Every path below is relative to that directory unless it starts with `/`.

**Provenance.** Every code block in this plan ran green on a throwaway worktree, `/workspace/.worktrees/curadi-10d-proto`. It is detached at the throwaway commit `440746c`; delete it in Task 6. That run covered:
- the full suite, clippy `-D warnings` and `fmt-check`;
- the audit at 150 roots / 6696 cells / 7862 forms with **zero differences** against vidyut, the `entry` control failing on 36 cells;
- a golden generator (`/tmp/vidyut-full/vidyut-prakriya/examples/curadi_goldens_10d.rs`) that derived all 432 new cells with the prototype engine and asserted each cell's derivation set equal to vidyut's: 432 cells, 468 forms, all equal;
- a byte-identical dump of all 7394 prior live-branch logs, `main` against the prototype;
- `cargo mutants --in-diff` over the prototype's production diff: 8 mutants, 8 caught.

The golden rows below are that generator's output. Index 0 and the vikalpa keys are the engine's; the form sets are proven equal to vidyut's.

## Global Constraints

- **Two new rule ids, no others:** `10.0493` (`RuleKind::Samjna`, name `"jYapAdayo mitaH"`) and `6.4.92` (`RuleKind::Vidhi`, name `"mitAM hrasvaH"`). Both go in `tinanta/sanadi.rs`, and both guard on `Tag::Mit`.
- **No 01.0934 and no 10.0494.** Without a causative, 01.0934 can never fire in this engine. Both wait for a causative slice; the code comment and the √syam/√śam data comments say so.
- **No `10.0124 ciY`.** It is inside `JNAPADI` but stays uncurated (it needs 7.2.115).
- **The 6264 pre-existing cells must stay byte-identical, traces included.** Regenerate no prior golden, and change no prior pinned trace.
- **Goldens are transcribed from this plan.** **Do not edit a golden to match the engine.** If a new golden fails, stop and report.
- **Jñapādi rows are found by the positional `JNAPADI` range** in trace tests, never by a hand list of numbers.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit.
- `mise run test` takes 10–25 s depending on host load. Run it in the **foreground** with a timeout of 600000 ms. Never background it and end a turn.
- `mise run test -- -p X` does not scope. Scope with `mise exec -- cargo test -p <crate> <filter>`. To see every failing binary at once, use `mise exec -- cargo test --workspace --no-fail-fast`.
- The `cargo-mutants` mise shim fails here (`No version is set for shim`). Use the real binary, `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`, under `mise exec --`.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **A mit root reaching 6.4.92 without ṇic, or with a short or non-vowel upadhā.** It must decline and leave the aṅga untouched. → Task 2 `mitam_hrasvah_shortens_a_mit_roots_long_upadha_before_nit`.
2. **An am-final curādi root outside the jñapādi** (√syam, √śam). It must keep 7.2.116's vṛddhi, since 01.0934's mit-tva is not modelled. → Task 2 (unit: `SAm` declines); Task 3 `syAmayate_and_SAmayate_keep_their_vrddhi`.
3. **`Tag::Mit` leaking onto a non-curādi root numbered in range, or onto a curādi root out of range.** → Task 2 `derive_tags_mit_on_curadi_rows_in_jnapadi_only`; Task 3 `the_mit_rules_are_credited_on_exactly_the_jnapadi_cells`, which holds the corpus to exactly 468 credits per rule, all in range.
4. **A pada-ambiguous new surface in `check()`.** `ajYapayata` must yield two analyses, one per pada, both mit. → Task 3 `curadi_analyses_its_jnapadi_forms`.
5. **The pre-6.4.92 forms** (*jYApayati*, …) must be Invalid, not quietly derivable by another path. → Task 3 `curadi_analyses_its_jnapadi_forms`.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-data/src/lib.rs` | Task 2: `JNAPADI` and its upstream pin. Task 3: six rows, row-list test, `dhatus().len()`, the `pada` doc census, the √syam/√śam comments |
| `crates/panini-prakriya/src/term.rs` | Task 2: `Tag::Mit` |
| `crates/panini-prakriya/src/tinanta/mod.rs` | Task 2: `derive` tags `Tag::Mit` |
| `crates/panini-prakriya/src/tinanta/sanadi.rs` | Task 2: 10.0493, 6.4.92, module doc, unit tests |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Task 2: order pin, its doc, the `Tag::Mit` derive test |
| `crates/panini/tests/paradigm/data/curadi.rs` | Task 3: 48 golden rows, 36 alternates |
| `crates/panini/tests/paradigm/main.rs` | Task 3: totals, alternates census, pada-ambiguous set, `check()` test. Task 4: audit-chain prose |
| `crates/panini/tests/trace/curadi.rs` | Task 3: module doc, 1.3.74 list, three new tests |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `guna.rs` (one comment), the 10c spec | Task 4 |
| `AGENTS.md`, maybe `mise.toml` | Task 5 |

---

## Task 1: The worktree and baseline

**Files:** none. **Interfaces:** none.

- [ ] **Step 1: Confirm the worktree**

```bash
cd /workspace/.worktrees/curadi-10d
git status --short          # empty
git log --oneline -4        # the plan commit, cafcb76 (spec amendment), 9a97615 (spec), e594fe1
```

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint && mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Run in the foreground, timeout 600000 ms. Expected: everything passes at 6264 cells. `panini-prakriya` reports 403 tests, the `trace` binary 196, `paradigm` 22 and `panini-data` 23.

---

## Task 2: The engine — `JNAPADI`, `Tag::Mit`, 10.0493 and 6.4.92

No curated row is in `JNAPADI` yet, so after this task both rules fire nowhere in the corpus, and every golden is unchanged. The suite is green at the end of the task.

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (after `pub const AKUSMIYA`; the `tests` module)
- Modify: `crates/panini-prakriya/src/term.rs` (`enum Tag`, after `Akusmiya`)
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs` (the `panini_data` import; `derive`, after the `GHU` read)
- Modify: `crates/panini-prakriya/src/tinanta/sanadi.rs` (module doc, import, two `Rule`s, two tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (order pin and its doc; one new test)

**Interfaces:**
- Produces: `panini_data::JNAPADI: RangeInclusive<&'static str>` = `"10.0118"..="10.0124"`; `Tag::Mit`; rule ids `"10.0493"` and `"6.4.92"` in `SANADI`, at positions 2 and 7.
- Consumes: `crate::tinanta::sound::hrasva_of(char) -> Option<char>` (existing: `A/I/U/F/X` → short, else `None`); the sanādi tests' existing helper `with_nic(root: &str, nic: Option<&[Tag]>) -> Prakriya`; panini-data's test helper `upstream_rows() -> Vec<(&'static str, &'static str, &'static str)>`.

- [ ] **Step 1: Write the failing tests**

**(a)** In `crates/panini-data/src/lib.rs`'s `tests` module, immediately before `#[test] fn dhatupatha_numbers_resolve_upstream()`, add:

```rust
    #[test]
    fn jnapadi_is_exactly_the_rows_10_0493_names() {
        // The gaṇasūtra 10.0493 follows `10.0124 ciY` and makes the seven
        // rows from `10.0118 jYapa~` mit. vidyut-prakriya lists the same
        // seven upadeśas (`JNAP_ADI`). `ciY` is not curated, so this test —
        // not a derivation — is what holds the range's upper end.
        let rows = upstream_rows();
        let in_range: Vec<&str> = rows
            .iter()
            .filter(|(n, _, _)| JNAPADI.contains(n))
            .map(|(_, u, _)| *u)
            .collect();
        assert_eq!(
            in_range,
            ["jYapa~", "yama~", "caha~", "capa~", "raha~", "bala~", "ciY"]
        );
        // The neighbours on either side exist upstream and fall outside.
        for n in ["10.0117", "10.0125"] {
            assert!(rows.iter().any(|(m, _, _)| *m == n), "{n}");
            assert!(!JNAPADI.contains(&n), "{n}");
        }
    }

```

**(b)** In `crates/panini-prakriya/src/tinanta/sanadi.rs`'s `tests` module, immediately before `#[test] fn pugantalaghupadhasya_gunates_a_laghu_ik_upadha_before_nic()`, add:

```rust
    #[test]
    fn jnapadayo_mitah_credits_a_mit_root_and_changes_nothing() {
        let mut p = with_nic("jYap", None);
        p.terms[ANGA].add(Tag::Mit);
        assert!((rule("10.0493").apply)(&mut p));
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["10.0493"]);
        assert_eq!(p.terms[ANGA].text, "jYap", "a saṁjñā, not an operation");
        // √cur is curādi but not mit.
        let mut p = with_nic("cur", None);
        assert!(!(rule("10.0493").apply)(&mut p));
        assert!(p.log.is_empty());
    }

    #[test]
    fn mitam_hrasvah_shortens_a_mit_roots_long_upadha_before_nit() {
        // `jYAp` is 7.2.116's output for √jñap; the other three are
        // hand-built long upadhās, one per remaining `hrasva_of` arm that a
        // root can carry there.
        for (root, want) in [
            ("jYAp", "jYap"),
            ("cIl", "cil"),
            ("kUw", "kuw"),
            ("pFq", "pfq"),
        ] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            p.terms[ANGA].add(Tag::Mit);
            assert!((rule("6.4.92").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["6.4.92"]);
        }
        // A short upadhā (jYap, never lengthened), a non-vowel upadhā
        // (yakz: k), or too short to have one (A): nothing to shorten.
        for root in ["jYap", "yakz", "A"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            p.terms[ANGA].add(Tag::Mit);
            assert!(!(rule("6.4.92").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        // Not mit: √śam's `SAm` keeps 7.2.116's vṛddhi.
        let mut p = with_nic("SAm", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!(!(rule("6.4.92").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "SAm");
        // Mit, but no ṇit follower: neither a bare root nor a non-ṇit
        // pratyaya at `NIC`.
        let mut p = with_nic("jYAp", None);
        p.terms[ANGA].add(Tag::Mit);
        assert!(!(rule("6.4.92").apply)(&mut p));
        let mut p = with_nic("jYAp", Some(&[Tag::Ardhadhatuka]));
        p.terms[ANGA].add(Tag::Mit);
        assert!(!(rule("6.4.92").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "jYAp");
    }

```

**(c)** In `crates/panini-prakriya/src/tinanta/derivation_tests.rs`:
- In `tinanta_rule_order_is_pinned`, replace the first line of `expected`,
  `"10.0496", "3.1.25", "1.3.9", "3.4.114", "7.2.116", "7.3.86", "3.1.32", "1.3.12", "1.3.66",`, with
  `"10.0496", "10.0493", "3.1.25", "1.3.9", "3.4.114", "7.2.116", "6.4.92", "7.3.86", "3.1.32", "1.3.12", "1.3.66",`. `mise run fmt` rewraps the array.
- Append at the end of the file, after `derive_stamps_the_row_number_and_decides_ghu_by_it`:

```rust

#[test]
fn derive_tags_mit_on_curadi_rows_in_jnapadi_only() {
    // 10.0493's verdict, by row number and gaṇa: √jñap and √yam (in range)
    // are mit; √śam (am-final, curādi, outside the range) and √cur are not.
    // A non-curādi dhātu numbered inside the range is impossible upstream,
    // so the gaṇa check is pinned with a hand-built row.
    let row = |dhatupatha, code, gana| Dhatu {
        dhatupatha,
        code,
        gana,
        pada: if gana == Gana::Curadi {
            PadaAssignment::Nic
        } else {
            PadaAssignment::Parasmaipada
        },
        artha: "",
    };
    for (d, is_mit) in [
        (row("10.0118", "jYap", Gana::Curadi), true),
        (row("10.0119", "yam", Gana::Curadi), true),
        (row("10.0218", "Sam", Gana::Curadi), false),
        (row("10.0001", "cur", Gana::Curadi), false),
        (row("10.0118", "jYap", Gana::Bhvadi), false),
    ] {
        let p = derive(
            &d,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        )
        .into_iter()
        .next()
        .unwrap();
        assert_eq!(
            p.terms[ANGA].has(Tag::Mit),
            is_mit,
            "{} {:?}",
            d.dhatupatha,
            d.gana
        );
    }
}
```

- [ ] **Step 2: Run them to see them fail**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | grep -E "^error|cannot find" | head`
Expected: compile errors, `no variant ... named Mit` and `cannot find value JNAPADI`. Nothing exists yet. (`panini-data` fails the same way.)

- [ ] **Step 3: Add `JNAPADI`**

In `crates/panini-data/src/lib.rs`, immediately after `pub const AKUSMIYA: RangeInclusive<&str> = "10.0192"..="10.0236";`, add:

```rust

/// The jñapādi of curādi: dhātupāṭha rows `10.0118` (`jYapa~`) through
/// `10.0124` (`ciY`), the roots the gaṇasūtra 10.0493 makes mit. Compared as
/// strings, like `AKUSMIYA`. The engine's `derive` tags a curādi root in this
/// range `Tag::Mit`, which 10.0493 and 6.4.92 *mitāṃ hrasvaḥ* read. Includes
/// `10.0124 ciY`, which is mit but not yet curated (it waits on 7.2.115).
/// `jnapadi_is_exactly_the_rows_10_0493_names` pins the range to upstream.
pub const JNAPADI: RangeInclusive<&str> = "10.0118"..="10.0124";
```

- [ ] **Step 4: Add `Tag::Mit` and tag it in `derive`**

In `crates/panini-prakriya/src/term.rs`, immediately after the `Akusmiya,` variant (before the `/// The aṅga is a ṇijanta` doc of `Nijanta`), add:

```rust
    /// The dhātu is mit by the dhātupāṭha gaṇasūtra 10.0493: a curādi root
    /// in the data layer's `JNAPADI` range, tagged by `tinanta::derive`.
    /// Read only by 10.0493, which credits it, and 6.4.92 *mitāṃ hrasvaḥ*,
    /// which shortens the root's upadhā before ṇic, both in
    /// `tinanta::sanadi`. A saṁjñā verdict like `Ghu`, not a pada licence.
    Mit,
```

In `crates/panini-prakriya/src/tinanta/mod.rs`:
- Replace `use panini_data::{Dhatu, Gana, Lakara, Pada, PadaAssignment, Purusha, Vacana};` with `use panini_data::{Dhatu, Gana, JNAPADI, Lakara, Pada, PadaAssignment, Purusha, Vacana};`.
- In `derive`, immediately after the block `if samjna::GHU.contains(&dhatu.dhatupatha) { t.add(Tag::Ghu); }`, add:

```rust
    // 10.0493's mit-tva, likewise by row number: the jñapādi are a run of
    // curādi rows (`JNAPADI`), and 10.0493 and 6.4.92 read the tag.
    if dhatu.gana == Gana::Curadi && JNAPADI.contains(&dhatu.dhatupatha) {
        t.add(Tag::Mit);
    }
```

- [ ] **Step 5: Add the two rules**

In `crates/panini-prakriya/src/tinanta/sanadi.rs`:

Replace the module doc's first paragraph and its last paragraph:

```rust
//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, ṇic's
//! it-lopa (1.3.9), 3.4.114, 7.2.116, 7.3.86, 3.1.32 — opened by the
//! dhātupāṭha gaṇasūtra 10.0496, which settles an ākusmīya root's pada
//! before ṇic is added.
```
→
```rust
//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, ṇic's
//! it-lopa (1.3.9), 3.4.114, 7.2.116, 6.4.92, 7.3.86, 3.1.32 — opened by
//! two dhātupāṭha gaṇasūtras: 10.0496, which settles an ākusmīya root's
//! pada, and 10.0493, which credits a jñapādi root's mit-tva, both before
//! ṇic is added.
```

```rust
//! Every rule self-guards: 10.0496 on `Tag::Akusmiya`, 3.1.25 on
//! `Tag::Curadi`, the rest on ṇic being present. For gaṇas 1–9 the stage
//! adds nothing and records nothing.
```
→
```rust
//! Every rule self-guards: 10.0496 on `Tag::Akusmiya`, 10.0493 on
//! `Tag::Mit`, 3.1.25 on `Tag::Curadi`, the rest on ṇic being present
//! (6.4.92 on `Tag::Mit` as well). For gaṇas 1–9 the stage
//! adds nothing and records nothing.
```

Replace `use crate::tinanta::sound::guna_of;` with `use crate::tinanta::sound::{guna_of, hrasva_of};`.

Immediately before the comment line `// 3.1.25 satyāpapāśarūpavīṇātūlaślokasenālomatvacavarmavarṇacūrṇa-` (so, right after 10.0496's `Rule { … },`), add:

```rust
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
```

Immediately before the comment line `// 7.3.86 pugantalaghūpadhasya ca, before ṇic: guṇa of a laghu ik upadhā` (so, right after 7.2.116's `Rule { … },`), add:

```rust
    // 6.4.92 mitāṃ hrasvaḥ: a mit root's upadhā is shortened before ṇi.
    // `jYAp` → `jYap`: here it undoes the 7.2.116 vṛddhi just above, so
    // √jñap makes *jñapayati*, not *jñāpayati*. Guarded on `Tag::Mit`
    // (10.0493's verdict) and on ṇit ṇic; a short upadhā declines.
    //
    // vidyut-prakriya credits 6.4.92 later, among its asiddhavat rules,
    // after 7.3.84 has guṇated ṇic's `i`. It sits here instead, while the
    // root and ṇic are still separate terms, so the upadhā is one character
    // read. After 3.1.32 folds ṇic into `ANGA`, the root's vowel would have
    // to be found inside `jYApe`. The forms agree: no rule between the two
    // positions reads the root's vowel.
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
    },
```

Compute the upadhā index once, as written. Do not write `chars[n - 2]` twice: the prototype's first draft did, and cargo-mutants' `n - 2` → `n / 2` survived, because the two are equal for every 3- and 4-letter witness.

- [ ] **Step 6: Update the order pin's doc**

In `derivation_tests.rs`, replace

```rust
/// credits it: it settles an ākusmīya root's pada before ṇic exists. It is
/// the only id here that is not an Aṣṭādhyāyī sūtra.
```

with

```rust
/// credits it: it settles an ākusmīya root's pada before ṇic exists.
///
/// Slice 10d adds the gaṇasūtra 10.0493 *jñapādayo mitaḥ* right after it,
/// again where vidyut credits it, and 6.4.92 *mitāṃ hrasvaḥ* right after
/// 7.2.116, whose vṛddhi it undoes on a mit root. vidyut credits 6.4.92
/// later, after 7.3.84; see its comment in `sanadi.rs` for why it sits
/// here. 10.0496 and 10.0493 are the only ids here that are not
/// Aṣṭādhyāyī sūtras.
```

- [ ] **Step 7: Run the tests**

```bash
mise run fmt
mise exec -- cargo test -p panini-data jnapadi 2>&1 | grep "test result"
mise exec -- cargo test -p panini-prakriya 2>&1 | grep "test result" | head -1
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: `panini-data` 24 tests, `panini-prakriya` 406, all green; `trace` 196 and `paradigm` 22 unchanged and green. No curated row is mit yet.

- [ ] **Step 8: Commit**

```bash
mise run lint
git add -A
git commit -m "feat(engine): 10.0493 jñapādayo mitaḥ and 6.4.92 mitāṃ hrasvaḥ in the sanādi stage

JNAPADI (10.0118–10.0124) pinned to upstream; derive tags Tag::Mit on a
curādi root in range; 6.4.92 shortens the upadhā 7.2.116 lengthened before
ṇic. No curated row is mit yet, so every golden is unchanged."
```

---

## Task 3: The rows, their goldens, and every assertion they move

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini/tests/paradigm/data/curadi.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`
- Modify: `crates/panini/tests/trace/curadi.rs`

**Interfaces:**
- Consumes: `JNAPADI`, `Tag::Mit` and both rules from Task 2; the trace helpers `cell_trace`, `at` and `credited(&str) -> Vec<(&'static str, Gana)>`, which counts **live branches**, not cells.
- Produces: nothing new for later tasks; the totals Task 4 documents.

- [ ] **Step 1: Update the data-layer assertions (failing)**

In `crates/panini-data/src/lib.rs`'s `tests` module:
- `assert_eq!(dhatus().len(), 144);` → `assert_eq!(dhatus().len(), 150);`
- Rename `fn curadi_rows_are_the_forty_one_curated_roots` to `fn curadi_rows_are_the_forty_seven_curated_roots`. In its comment, replace

```rust
        // new: six by 7.3.86, ten by 7.2.116, seventeen unchanged before ṇic.
        // The gaṇa is OPEN at 41 of its 509 dhātupāṭha rows.
```

with

```rust
        // new: six by 7.3.86, ten by 7.2.116, seventeen unchanged before ṇic.
        // Slice 10d adds six of the seven jñapādi (`JNAPADI`), mit by the
        // gaṇasūtra 10.0493, whose upadhā 6.4.92 shortens back after
        // 7.2.116: √jñap, √yam, √cah, √cap, √rah, √bal, ubhayapadī by 1.3.74.
        // The gaṇa is OPEN at 47 of its 509 dhātupāṭha rows.
```

  In its expected list, immediately after `("10.0255", "BUz", PadaAssignment::Nic),`, insert:

```rust
                ("10.0118", "jYap", PadaAssignment::Nic),
                ("10.0119", "yam", PadaAssignment::Nic),
                ("10.0120", "cah", PadaAssignment::Nic),
                ("10.0121", "cap", PadaAssignment::Nic),
                ("10.0122", "rah", PadaAssignment::Nic),
                ("10.0123", "bal", PadaAssignment::Nic),
```

- [ ] **Step 2: Update the trace tests (failing)**

In `crates/panini/tests/trace/curadi.rs`:
- Replace the module doc's last line, `//! 10.0496, and has no pada sūtra at all.`, with

```rust
//! 10.0496, and has no pada sūtra at all. A jñapādi root's also opens one
//! step earlier, with the gaṇasūtra 10.0493, and 6.4.92 follows its
//! 7.2.116.
```

- `use panini_data::{AKUSMIYA, Gana, Lakara, Pada, Purusha, Vacana, dhatus};` → `use panini_data::{AKUSMIYA, Gana, JNAPADI, Lakara, Pada, Purusha, Vacana, dhatus};`
- In `a_kusmad_is_credited_on_exactly_the_akusmiya_cells`:
  - `// And 1.3.74 never reaches them: its credits stay on the four `Nic` rows.` → `… on the ten `Nic` rows.`
  - Replace `["10.0001", "10.0010", "10.0033", "10.0255"].contains(&number),` with

```rust
            [
                "10.0001", "10.0010", "10.0033", "10.0255", "10.0118", "10.0119", "10.0120",
                "10.0121", "10.0122", "10.0123",
            ]
            .contains(&number),
```

- Append at the end of the file:

```rust

#[test]
fn jnapayati_trace_lengthens_then_shortens_the_upadha() {
    // jYap P laT P.E. 10.0493 credits the mit-tva before ṇic exists; 7.2.116
    // lengthens the `a` upadhā before ṇit ṇic (`jYAp`), and 6.4.92 mitāṃ
    // hrasvaḥ shortens it back (`jYap`) before 3.1.32 folds ṇic in.
    let (text, t) = cell_trace(
        "10.0118",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "jYapayati", "got {t:?}");
    assert_eq!(
        t,
        [
            "10.0493", "3.1.25", "1.3.9", "3.4.114", "7.2.116", "6.4.92", "3.1.32", "1.3.78",
            "3.4.78", "1.3.9", "3.1.68", "1.3.9", "7.3.84", "6.1.78",
        ],
    );
}

#[test]
#[allow(non_snake_case)]
fn syAmayate_and_SAmayate_keep_their_vrddhi() {
    // √syam and √śam are am-final curādi roots, but not jñapādi: neither
    // 10.0493 nor 6.4.92 reaches them, and 7.2.116's vṛddhi stands.
    for (number, form) in [("10.0216", "syAmayate"), ("10.0218", "SAmayate")] {
        let (text, t) = cell_trace(
            number,
            Lakara::Lat,
            Pada::Atmanepada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert_eq!(text, form, "got {t:?}");
        assert!(t.contains(&"7.2.116".to_string()), "{form}: got {t:?}");
        for absent in ["10.0493", "6.4.92"] {
            assert!(
                !t.contains(&absent.to_string()),
                "{form} {absent}: got {t:?}"
            );
        }
    }
}

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
    }
}
```

- [ ] **Step 3: Update the paradigm assertions and add the `check()` test (failing)**

In `crates/panini/tests/paradigm/main.rs` (each `old` occurs exactly once; confirm with `grep -c`):

- `/// `ALTERNATES` is otherwise 1130 bare strings` → `1166`.
- `/// itself has 1130 rows, keyed 170 `8.4.56`, 162 `7.1.35`, 162 `7.1.35+8.4.56`,` → `/// itself has 1166 rows, keyed 182 `8.4.56`, 174 `7.1.35`, 174 `7.1.35+8.4.56`,`
- `/// 6264 cells total (696 root×lakāra blocks × 9), of which 5440 hold exactly one form, 612 hold two, 165 hold three` → `/// 6696 cells total (744 root×lakāra blocks × 9), of which 5848 hold exactly one form, 624 hold two, 177 hold three`
- `and the four curādi roots', new in slice 10a, each by` → `and the four curādi roots', new in slice 10a, and the six jñapādi roots', new in slice 10d, each by`
- After the 10c paragraph's last line, `/// OPEN at 41 of its 509 rows.`, insert:

```rust
///
/// Slice 10d curates six of the seven jñapādi roots, √jñap, √yam, √cah,
/// √cap, √rah and √bal (`10.0118` through `10.0123`), mit by the gaṇasūtra
/// 10.0493 and ubhayapadī by 1.3.74. 6.4.92 *mitāṃ hrasvaḥ* shortens back
/// the upadhā 7.2.116 lengthened before ṇic (*jñapayati*). No new vikalpa
/// rule: each root forks only where √bhūṣ does — laṅ and vidhiliṅ
/// parasmaipada prathama eka on 8.4.56, the two loṭ tātaṅ cells three ways
/// on 7.1.35/8.4.56 — since neither 7.2.116 nor 6.4.92 is a vikalpa key.
/// 432 new cells, thirty-six new rows. The gaṇa is OPEN at 47 of its 509
/// rows.
```

- In `derivation_set_shape_matches_the_audited_numbers`:
  - `assert_eq!(total_cells, 6264, "696 root×lakāra blocks × 9 cells each");` → `assert_eq!(total_cells, 6696, "744 root×lakāra blocks × 9 cells each");`
  - `assert_eq!(ones, 5440, "one-form cells");` → `5848`
  - `assert_eq!(twos, 612, "two-form cells");` → `624`
  - `        threes, 165,` → `        threes, 177,`
  - In the threes message, replace `7.1.35/8.4.56"` (the message's final line, after `the four curādi roots' two loṭ tātaṅ cells each, by \`) with `7.1.35/8.4.56; and — new in slice 10d — the six jñapādi roots', the same way"`
  - `assert_eq!(ALTERNATES.len(), 1130, "ALTERNATES row count");` → `1166`
  - `assert_eq!(key_count("8.4.56"), 170, "8.4.56-only alternates");` → `182`
  - `assert_eq!(key_count("7.1.35"), 162, "7.1.35-only alternates");` → `174`
  - `assert_eq!(key_count("7.1.35+8.4.56"), 162, "7.1.35+8.4.56 alternates");` → `174`
- In `pada_ambiguous_surfaces_are_exactly_these`:
  - After the comment lines `    // the set from fifty-six to seventy-two, with no new collision against` / `    // any pre-slice surface.`, add (same comment block):

```rust
    // Slice 10d's six jñapādi roots, ubhayapadī by
    // 1.3.74, contribute the same four-surface shape each (`ajYapayata`,
    // `jYapayatAm`, `jYapayetAm`, `jYapayeta`) — twenty-four more, taking the
    // set from seventy-two to ninety-six, again with no collision against
    // any pre-slice surface. The ākusmīya roots, ātmanepada-only, contribute
    // nothing.
```

  - In the expected `vec![…]`, insert these 24 strings at their sorted positions. The list is sorted by byte order, and the test fails on any misplacement:
    - `"abalayata"` after `"aGfRuta"`;
    - `"acahayata"`, `"acapayata"` after `"acCintta"`;
    - `"ajYapayata"` after `"adatta"`;
    - `"arahayata"` after `"anenikta"`;
    - `"ayamayata"` after `"aviNkta"`;
    - `"balayatAm"`, `"balayetAm"`, `"balayeta"` after `"ayuNkta"`;
    - `"cahayatAm"`, `"cahayetAm"`, `"cahayeta"`, `"capayatAm"`, `"capayetAm"`, `"capayeta"` after `"biBftAm"`;
    - `"jYapayatAm"`, `"jYapayetAm"`, `"jYapayeta"` after `"fRutAm"`;
    - `"rahayatAm"`, `"rahayetAm"`, `"rahayeta"` after `"neniktAm"`;
    - `"yamayatAm"`, `"yamayetAm"`, `"yamayeta"` after `"viNktAm"`.

    The resulting list has 96 entries.
- Immediately after the closing `}` of `fn curadi_analyses_its_bulk_akusmiya_forms`, add:

```rust

/// Slice 10d's `check()` witnesses: all six jñapādi rows, both padas. The
/// goldens were grepped first — each laṭ prathama eka surface below is its
/// own row's alone, so each must yield exactly one analysis, naming that
/// root and pada and crediting 10.0493 and 6.4.92 in that order, after
/// 7.2.116. `ajYapayata` is pada-ambiguous within √jñap (laṅ parasmaipada
/// madhyama bahu = ātmanepada prathama eka): two analyses, one per pada, both
/// mit. The 7.2.116-only shapes (`jYApayati`, …) — what this engine derived
/// before 6.4.92 — derive nothing.
#[test]
fn curadi_analyses_its_jnapadi_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    let assert_mit = |form: &str, ids: &[String]| {
        assert_eq!(ids[0], "10.0493", "{form}: {ids:?}");
        let pos = |id: &str| ids.iter().position(|i| i == id);
        let (lengthen, shorten) = (pos("7.2.116"), pos("6.4.92"));
        assert!(
            lengthen.is_some() && shorten.is_some() && lengthen < shorten,
            "{form}: {ids:?}"
        );
    };
    for (dhatu, parasmai, atmane) in [
        ("jYap", "jYapayati", "jYapayate"),
        ("yam", "yamayati", "yamayate"),
        ("cah", "cahayati", "cahayate"),
        ("cap", "capayati", "capayate"),
        ("rah", "rahayati", "rahayate"),
        ("bal", "balayati", "balayate"),
    ] {
        for (form, pada) in [(parasmai, Pada::Parasmaipada), (atmane, Pada::Atmanepada)] {
            let r = engine.check(form);
            assert!(matches!(r.verdict, Verdict::Valid), "{form}");
            assert_eq!(r.analyses.len(), 1, "{form}");
            let a = &r.analyses[0];
            assert_eq!(a.dhatu, dhatu, "{form}");
            assert_eq!(a.pada, pada, "{form}");
            assert_mit(form, &ids_of(a));
        }
    }
    let r = engine.check("ajYapayata");
    assert!(matches!(r.verdict, Verdict::Valid));
    let mut padas: Vec<Pada> = r.analyses.iter().map(|a| a.pada).collect();
    padas.sort_by_key(|p| *p == Pada::Atmanepada);
    assert_eq!(padas, [Pada::Parasmaipada, Pada::Atmanepada]);
    for a in &r.analyses {
        assert_eq!(a.dhatu, "jYap");
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
    }
}
```

Grep the goldens for every witness before trusting the single-analysis assertions:

```bash
for f in jYapayati yamayati cahayati capayati rahayati balayati jYapayate yamayate cahayate capayate rahayate balayate jYApayati yAmayati cAhayati cApayati rAhayati bAlayati jYApayate; do
  printf "%s: %s\n" $f "$(grep -rl "\"$f\"" crates/panini/tests/paradigm/data/ | tr '\n' ' ')"; done
```

Expected now (rows absent): every line empty. After Step 6: the twelve real forms print `crates/panini/tests/paradigm/data/curadi.rs` only, and the seven lengthened shapes print nothing.

- [ ] **Step 4: Run the tests to see them fail**

Run: `mise exec -- cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .*FAILED|panicked at" | head -20`
Expected failures:
- `curated_roots_have_expected_ganas_and_padas` (144 vs 150);
- `curadi_rows_are_the_forty_seven_curated_roots`;
- `derivation_set_shape_matches_the_audited_numbers` (6264 vs 6696);
- `pada_ambiguous_surfaces_are_exactly_these`;
- `curadi_analyses_its_jnapadi_forms` (`jYapayati` Invalid);
- `jnapayati_trace_lengthens_then_shortens_the_upadha` (not a curated root);
- `the_mit_rules_are_credited_on_exactly_the_jnapadi_cells` (0 vs 468).

- [ ] **Step 5: Add the six `Dhatu` rows and fix the doc counts**

In `crates/panini-data/src/lib.rs`, immediately after the `10.0255` (`BUz`) row's closing `},` (before the `10.0192` row), insert:

```rust
    Dhatu {
        // 10.0118 `jYapa~` jYAne jYApane ca (√jñap). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic and 6.4.92 mitāṃ hrasvaḥ
        // shortens it back (mit by the gaṇasūtra 10.0493). Ubhayapadī by
        // 1.3.74. Slice 10d.
        dhatupatha: "10.0118",
        code: "jYap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "jYAne jYApane ca",
    },
    Dhatu {
        // 10.0119 `yama~` parivezaRe (√yam). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic and 6.4.92 mitāṃ hrasvaḥ shortens it
        // back (mit by the gaṇasūtra 10.0493). Am-final, so 01.0934 would also
        // make it mit; this engine has no 01.0934 (it waits for a causative
        // slice), and 10.0493 alone gives *yamayati*. Ubhayapadī by 1.3.74.
        // Slice 10d.
        dhatupatha: "10.0119",
        code: "yam",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "parivezaRe",
    },
    Dhatu {
        // 10.0120 `caha~` parikalkane (√cah). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic and 6.4.92 mitāṃ hrasvaḥ shortens it
        // back (mit by the gaṇasūtra 10.0493). Ubhayapadī by 1.3.74. Slice
        // 10d.
        dhatupatha: "10.0120",
        code: "cah",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "parikalkane",
    },
    Dhatu {
        // 10.0121 `capa~` parikalpane (√cap). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic and 6.4.92 mitāṃ hrasvaḥ shortens it
        // back (mit by the gaṇasūtra 10.0493). Ubhayapadī by 1.3.74. Slice
        // 10d.
        dhatupatha: "10.0121",
        code: "cap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "parikalpane",
    },
    Dhatu {
        // 10.0122 `raha~` tyAge (√rah). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic and 6.4.92 mitāṃ hrasvaḥ shortens it back
        // (mit by the gaṇasūtra 10.0493). Ubhayapadī by 1.3.74. Slice 10d.
        dhatupatha: "10.0122",
        code: "rah",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "tyAge",
    },
    Dhatu {
        // 10.0123 `bala~` prARane (√bal). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic and 6.4.92 mitāṃ hrasvaḥ shortens it back
        // (mit by the gaṇasūtra 10.0493). Ubhayapadī by 1.3.74. Slice 10d.
        dhatupatha: "10.0123",
        code: "bal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prARane",
    },
```

Then, in the same file (each `old` occurs once):
- The `pada` field doc: `re-derives 102 of these 144` → `re-derives 102 of these 150`, and `exception, four curādi rows' are 1.3.74's and 37 ākusmīya rows' are` → `exception, ten curādi rows' are 1.3.74's and 37 ākusmīya rows' are`. The census is 102 + 1 + 10 + 37 = 150.
- `/// The test covers the 144 roots curated here` → `150`.
- `vendored upadeśa: 66 of the 144 curated roots carry a` → `66 of the 150`. None of the six upadeśas carries a `\`, so the 66 and the 45 stand.
- The `10.0216` (√syam) comment: replace

```rust
        // mittva 01.0934; this engine has neither, and the forms agree. The
        // mit slice inherits this row as a witness: once it adds 01.0934, this
        // golden fails unless 10.0494 comes with it. Ātmanepadī by 10.0496.
        // Slice 10c.
```

  with

```rust
        // mittva 01.0934; this engine has neither, and the forms agree. Not
        // jñapādi, so 10.0493 and 6.4.92 pass it by (slice 10d). A causative
        // slice inherits this row as a witness: once it adds 01.0934, this
        // golden fails unless 10.0494 comes with it. Ātmanepadī by 10.0496.
        // Slice 10c.
```

- The `10.0218` (√śam) comment: replace

```rust
        // `a` upadhā before ṇit ṇic (SAm-i). vidyut also credits 10.0494 here,
        // as on `10.0216`; a witness for the mit slice. Ātmanepadī by 10.0496.
        // Slice 10c.
```

  with

```rust
        // `a` upadhā before ṇit ṇic (SAm-i). vidyut also credits 10.0494 here,
        // as on `10.0216`; a witness for a causative slice. Ātmanepadī by
        // 10.0496. Slice 10c.
```

- [ ] **Step 6: Add the golden rows**

In `crates/panini/tests/paradigm/data/curadi.rs`, `mise run fmt` rewraps the one-line rows below into the file's style.

**(a) `PARADIGM`:** immediately before the first `10.0192` row (the line `    (` followed by `        "10.0192",`), and so after `10.0255`'s `viDiliN` ātmanepada row, insert:

```rust
    ("10.0118", "laT", Pada::Parasmaipada, ["jYapayati", "jYapayataH", "jYapayanti", "jYapayasi", "jYapayaTaH", "jYapayaTa", "jYapayAmi", "jYapayAvaH", "jYapayAmaH"]),
    ("10.0118", "laN", Pada::Parasmaipada, ["ajYapayad", "ajYapayatAm", "ajYapayan", "ajYapayaH", "ajYapayatam", "ajYapayata", "ajYapayam", "ajYapayAva", "ajYapayAma"]),
    ("10.0118", "loT", Pada::Parasmaipada, ["jYapayatu", "jYapayatAm", "jYapayantu", "jYapaya", "jYapayatam", "jYapayata", "jYapayAni", "jYapayAva", "jYapayAma"]),
    ("10.0118", "viDiliN", Pada::Parasmaipada, ["jYapayed", "jYapayetAm", "jYapayeyuH", "jYapayeH", "jYapayetam", "jYapayeta", "jYapayeyam", "jYapayeva", "jYapayema"]),
    ("10.0118", "laT", Pada::Atmanepada, ["jYapayate", "jYapayete", "jYapayante", "jYapayase", "jYapayeTe", "jYapayaDve", "jYapaye", "jYapayAvahe", "jYapayAmahe"]),
    ("10.0118", "laN", Pada::Atmanepada, ["ajYapayata", "ajYapayetAm", "ajYapayanta", "ajYapayaTAH", "ajYapayeTAm", "ajYapayaDvam", "ajYapaye", "ajYapayAvahi", "ajYapayAmahi"]),
    ("10.0118", "loT", Pada::Atmanepada, ["jYapayatAm", "jYapayetAm", "jYapayantAm", "jYapayasva", "jYapayeTAm", "jYapayaDvam", "jYapayE", "jYapayAvahE", "jYapayAmahE"]),
    ("10.0118", "viDiliN", Pada::Atmanepada, ["jYapayeta", "jYapayeyAtAm", "jYapayeran", "jYapayeTAH", "jYapayeyATAm", "jYapayeDvam", "jYapayeya", "jYapayevahi", "jYapayemahi"]),
    ("10.0119", "laT", Pada::Parasmaipada, ["yamayati", "yamayataH", "yamayanti", "yamayasi", "yamayaTaH", "yamayaTa", "yamayAmi", "yamayAvaH", "yamayAmaH"]),
    ("10.0119", "laN", Pada::Parasmaipada, ["ayamayad", "ayamayatAm", "ayamayan", "ayamayaH", "ayamayatam", "ayamayata", "ayamayam", "ayamayAva", "ayamayAma"]),
    ("10.0119", "loT", Pada::Parasmaipada, ["yamayatu", "yamayatAm", "yamayantu", "yamaya", "yamayatam", "yamayata", "yamayAni", "yamayAva", "yamayAma"]),
    ("10.0119", "viDiliN", Pada::Parasmaipada, ["yamayed", "yamayetAm", "yamayeyuH", "yamayeH", "yamayetam", "yamayeta", "yamayeyam", "yamayeva", "yamayema"]),
    ("10.0119", "laT", Pada::Atmanepada, ["yamayate", "yamayete", "yamayante", "yamayase", "yamayeTe", "yamayaDve", "yamaye", "yamayAvahe", "yamayAmahe"]),
    ("10.0119", "laN", Pada::Atmanepada, ["ayamayata", "ayamayetAm", "ayamayanta", "ayamayaTAH", "ayamayeTAm", "ayamayaDvam", "ayamaye", "ayamayAvahi", "ayamayAmahi"]),
    ("10.0119", "loT", Pada::Atmanepada, ["yamayatAm", "yamayetAm", "yamayantAm", "yamayasva", "yamayeTAm", "yamayaDvam", "yamayE", "yamayAvahE", "yamayAmahE"]),
    ("10.0119", "viDiliN", Pada::Atmanepada, ["yamayeta", "yamayeyAtAm", "yamayeran", "yamayeTAH", "yamayeyATAm", "yamayeDvam", "yamayeya", "yamayevahi", "yamayemahi"]),
    ("10.0120", "laT", Pada::Parasmaipada, ["cahayati", "cahayataH", "cahayanti", "cahayasi", "cahayaTaH", "cahayaTa", "cahayAmi", "cahayAvaH", "cahayAmaH"]),
    ("10.0120", "laN", Pada::Parasmaipada, ["acahayad", "acahayatAm", "acahayan", "acahayaH", "acahayatam", "acahayata", "acahayam", "acahayAva", "acahayAma"]),
    ("10.0120", "loT", Pada::Parasmaipada, ["cahayatu", "cahayatAm", "cahayantu", "cahaya", "cahayatam", "cahayata", "cahayAni", "cahayAva", "cahayAma"]),
    ("10.0120", "viDiliN", Pada::Parasmaipada, ["cahayed", "cahayetAm", "cahayeyuH", "cahayeH", "cahayetam", "cahayeta", "cahayeyam", "cahayeva", "cahayema"]),
    ("10.0120", "laT", Pada::Atmanepada, ["cahayate", "cahayete", "cahayante", "cahayase", "cahayeTe", "cahayaDve", "cahaye", "cahayAvahe", "cahayAmahe"]),
    ("10.0120", "laN", Pada::Atmanepada, ["acahayata", "acahayetAm", "acahayanta", "acahayaTAH", "acahayeTAm", "acahayaDvam", "acahaye", "acahayAvahi", "acahayAmahi"]),
    ("10.0120", "loT", Pada::Atmanepada, ["cahayatAm", "cahayetAm", "cahayantAm", "cahayasva", "cahayeTAm", "cahayaDvam", "cahayE", "cahayAvahE", "cahayAmahE"]),
    ("10.0120", "viDiliN", Pada::Atmanepada, ["cahayeta", "cahayeyAtAm", "cahayeran", "cahayeTAH", "cahayeyATAm", "cahayeDvam", "cahayeya", "cahayevahi", "cahayemahi"]),
    ("10.0121", "laT", Pada::Parasmaipada, ["capayati", "capayataH", "capayanti", "capayasi", "capayaTaH", "capayaTa", "capayAmi", "capayAvaH", "capayAmaH"]),
    ("10.0121", "laN", Pada::Parasmaipada, ["acapayad", "acapayatAm", "acapayan", "acapayaH", "acapayatam", "acapayata", "acapayam", "acapayAva", "acapayAma"]),
    ("10.0121", "loT", Pada::Parasmaipada, ["capayatu", "capayatAm", "capayantu", "capaya", "capayatam", "capayata", "capayAni", "capayAva", "capayAma"]),
    ("10.0121", "viDiliN", Pada::Parasmaipada, ["capayed", "capayetAm", "capayeyuH", "capayeH", "capayetam", "capayeta", "capayeyam", "capayeva", "capayema"]),
    ("10.0121", "laT", Pada::Atmanepada, ["capayate", "capayete", "capayante", "capayase", "capayeTe", "capayaDve", "capaye", "capayAvahe", "capayAmahe"]),
    ("10.0121", "laN", Pada::Atmanepada, ["acapayata", "acapayetAm", "acapayanta", "acapayaTAH", "acapayeTAm", "acapayaDvam", "acapaye", "acapayAvahi", "acapayAmahi"]),
    ("10.0121", "loT", Pada::Atmanepada, ["capayatAm", "capayetAm", "capayantAm", "capayasva", "capayeTAm", "capayaDvam", "capayE", "capayAvahE", "capayAmahE"]),
    ("10.0121", "viDiliN", Pada::Atmanepada, ["capayeta", "capayeyAtAm", "capayeran", "capayeTAH", "capayeyATAm", "capayeDvam", "capayeya", "capayevahi", "capayemahi"]),
    ("10.0122", "laT", Pada::Parasmaipada, ["rahayati", "rahayataH", "rahayanti", "rahayasi", "rahayaTaH", "rahayaTa", "rahayAmi", "rahayAvaH", "rahayAmaH"]),
    ("10.0122", "laN", Pada::Parasmaipada, ["arahayad", "arahayatAm", "arahayan", "arahayaH", "arahayatam", "arahayata", "arahayam", "arahayAva", "arahayAma"]),
    ("10.0122", "loT", Pada::Parasmaipada, ["rahayatu", "rahayatAm", "rahayantu", "rahaya", "rahayatam", "rahayata", "rahayARi", "rahayAva", "rahayAma"]),
    ("10.0122", "viDiliN", Pada::Parasmaipada, ["rahayed", "rahayetAm", "rahayeyuH", "rahayeH", "rahayetam", "rahayeta", "rahayeyam", "rahayeva", "rahayema"]),
    ("10.0122", "laT", Pada::Atmanepada, ["rahayate", "rahayete", "rahayante", "rahayase", "rahayeTe", "rahayaDve", "rahaye", "rahayAvahe", "rahayAmahe"]),
    ("10.0122", "laN", Pada::Atmanepada, ["arahayata", "arahayetAm", "arahayanta", "arahayaTAH", "arahayeTAm", "arahayaDvam", "arahaye", "arahayAvahi", "arahayAmahi"]),
    ("10.0122", "loT", Pada::Atmanepada, ["rahayatAm", "rahayetAm", "rahayantAm", "rahayasva", "rahayeTAm", "rahayaDvam", "rahayE", "rahayAvahE", "rahayAmahE"]),
    ("10.0122", "viDiliN", Pada::Atmanepada, ["rahayeta", "rahayeyAtAm", "rahayeran", "rahayeTAH", "rahayeyATAm", "rahayeDvam", "rahayeya", "rahayevahi", "rahayemahi"]),
    ("10.0123", "laT", Pada::Parasmaipada, ["balayati", "balayataH", "balayanti", "balayasi", "balayaTaH", "balayaTa", "balayAmi", "balayAvaH", "balayAmaH"]),
    ("10.0123", "laN", Pada::Parasmaipada, ["abalayad", "abalayatAm", "abalayan", "abalayaH", "abalayatam", "abalayata", "abalayam", "abalayAva", "abalayAma"]),
    ("10.0123", "loT", Pada::Parasmaipada, ["balayatu", "balayatAm", "balayantu", "balaya", "balayatam", "balayata", "balayAni", "balayAva", "balayAma"]),
    ("10.0123", "viDiliN", Pada::Parasmaipada, ["balayed", "balayetAm", "balayeyuH", "balayeH", "balayetam", "balayeta", "balayeyam", "balayeva", "balayema"]),
    ("10.0123", "laT", Pada::Atmanepada, ["balayate", "balayete", "balayante", "balayase", "balayeTe", "balayaDve", "balaye", "balayAvahe", "balayAmahe"]),
    ("10.0123", "laN", Pada::Atmanepada, ["abalayata", "abalayetAm", "abalayanta", "abalayaTAH", "abalayeTAm", "abalayaDvam", "abalaye", "abalayAvahi", "abalayAmahi"]),
    ("10.0123", "loT", Pada::Atmanepada, ["balayatAm", "balayetAm", "balayantAm", "balayasva", "balayeTAm", "balayaDvam", "balayE", "balayAvahE", "balayAmahE"]),
    ("10.0123", "viDiliN", Pada::Atmanepada, ["balayeta", "balayeyAtAm", "balayeran", "balayeTAH", "balayeyATAm", "balayeDvam", "balayeya", "balayevahi", "balayemahi"]),
```

**(b) `ALTERNATES`:** immediately before the closing `];` of `pub const ALTERNATES`, after the last `10.0255` row, insert:

```rust
    ("10.0118", "laN", Pada::Parasmaipada, 0, "ajYapayat", "8.4.56"),
    ("10.0118", "loT", Pada::Parasmaipada, 0, "jYapayatAd", "7.1.35"),
    ("10.0118", "loT", Pada::Parasmaipada, 0, "jYapayatAt", "7.1.35+8.4.56"),
    ("10.0118", "loT", Pada::Parasmaipada, 3, "jYapayatAd", "7.1.35"),
    ("10.0118", "loT", Pada::Parasmaipada, 3, "jYapayatAt", "7.1.35+8.4.56"),
    ("10.0118", "viDiliN", Pada::Parasmaipada, 0, "jYapayet", "8.4.56"),
    ("10.0119", "laN", Pada::Parasmaipada, 0, "ayamayat", "8.4.56"),
    ("10.0119", "loT", Pada::Parasmaipada, 0, "yamayatAd", "7.1.35"),
    ("10.0119", "loT", Pada::Parasmaipada, 0, "yamayatAt", "7.1.35+8.4.56"),
    ("10.0119", "loT", Pada::Parasmaipada, 3, "yamayatAd", "7.1.35"),
    ("10.0119", "loT", Pada::Parasmaipada, 3, "yamayatAt", "7.1.35+8.4.56"),
    ("10.0119", "viDiliN", Pada::Parasmaipada, 0, "yamayet", "8.4.56"),
    ("10.0120", "laN", Pada::Parasmaipada, 0, "acahayat", "8.4.56"),
    ("10.0120", "loT", Pada::Parasmaipada, 0, "cahayatAd", "7.1.35"),
    ("10.0120", "loT", Pada::Parasmaipada, 0, "cahayatAt", "7.1.35+8.4.56"),
    ("10.0120", "loT", Pada::Parasmaipada, 3, "cahayatAd", "7.1.35"),
    ("10.0120", "loT", Pada::Parasmaipada, 3, "cahayatAt", "7.1.35+8.4.56"),
    ("10.0120", "viDiliN", Pada::Parasmaipada, 0, "cahayet", "8.4.56"),
    ("10.0121", "laN", Pada::Parasmaipada, 0, "acapayat", "8.4.56"),
    ("10.0121", "loT", Pada::Parasmaipada, 0, "capayatAd", "7.1.35"),
    ("10.0121", "loT", Pada::Parasmaipada, 0, "capayatAt", "7.1.35+8.4.56"),
    ("10.0121", "loT", Pada::Parasmaipada, 3, "capayatAd", "7.1.35"),
    ("10.0121", "loT", Pada::Parasmaipada, 3, "capayatAt", "7.1.35+8.4.56"),
    ("10.0121", "viDiliN", Pada::Parasmaipada, 0, "capayet", "8.4.56"),
    ("10.0122", "laN", Pada::Parasmaipada, 0, "arahayat", "8.4.56"),
    ("10.0122", "loT", Pada::Parasmaipada, 0, "rahayatAd", "7.1.35"),
    ("10.0122", "loT", Pada::Parasmaipada, 0, "rahayatAt", "7.1.35+8.4.56"),
    ("10.0122", "loT", Pada::Parasmaipada, 3, "rahayatAd", "7.1.35"),
    ("10.0122", "loT", Pada::Parasmaipada, 3, "rahayatAt", "7.1.35+8.4.56"),
    ("10.0122", "viDiliN", Pada::Parasmaipada, 0, "rahayet", "8.4.56"),
    ("10.0123", "laN", Pada::Parasmaipada, 0, "abalayat", "8.4.56"),
    ("10.0123", "loT", Pada::Parasmaipada, 0, "balayatAd", "7.1.35"),
    ("10.0123", "loT", Pada::Parasmaipada, 0, "balayatAt", "7.1.35+8.4.56"),
    ("10.0123", "loT", Pada::Parasmaipada, 3, "balayatAd", "7.1.35"),
    ("10.0123", "loT", Pada::Parasmaipada, 3, "balayatAt", "7.1.35+8.4.56"),
    ("10.0123", "viDiliN", Pada::Parasmaipada, 0, "balayet", "8.4.56"),
```

Optional cross-check of the transcription: the throwaway generator `/tmp/vidyut-full/vidyut-prakriya/examples/curadi_goldens_10d.rs` re-emits both blocks to `/tmp/vidyut-full/goldens_10d_{paradigm,alternates}.rs` in rustfmt layout. Run it only with the dev-deps repointed at this worktree (Task 4 Step 2), since it reads the curated rows.

- [ ] **Step 7: Run the full suite**

```bash
mise run fmt
mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"
```

Foreground, timeout 600000 ms. Expected: PASS at 6696 cells, with `panini-data` 24, `panini-prakriya` 406, `trace` 199 and `paradigm` 23. Then re-run Step 3's grep loop: twelve forms print `curadi.rs` only, and the seven lengthened shapes print nothing.

- [ ] **Step 8: Commit**

```bash
mise run lint
git add -A
git commit -m "feat(data): curādi's jñapādi mit roots — six rows (10.0118–10.0123), ubhayapadī by 1.3.74

6264 → 6696 cells, 7394 → 7862 forms, ALTERNATES 1130 → 1166, 144 → 150
roots; pada-ambiguous surfaces 72 → 96. 10.0493 and 6.4.92 are credited on
exactly the 468 jñapādi branches; √syam/√śam keep their vṛddhi and pass to a
causative slice as 10.0494's witnesses."
```

---

## Task 4: Audit, prior-trace diff, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (audit prose), `crates/panini-prakriya/src/tinanta/guna.rs` (one comment), `docs/superpowers/specs/2026-10-02-curadi-gana-10c-design.md`

**Interfaces:**
- Consumes: the finished engine, data and goldens. Produces no symbols.

- [ ] **Step 1: Update the audit harness**

In `tools/audit/panini_full_audit.rs` (each `old` occurs once):
- Replace

```rust
//! What it compares: for each of the 144 curated roots, for each pada the root
//! admits (two apiece for the thirty roots that admit both padas —
//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, and four curādi roots by
//! 1.3.74), for each of the four
```

  with

```rust
//! What it compares: for each of the 150 curated roots, for each pada the root
//! admits (two apiece for the thirty-six roots that admit both padas —
//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, and ten curādi roots by
//! 1.3.74), for each of the four
```

- `//! Corpus invariants, asserted: 144 roots, 6264 cells, 7394 forms. These are` → `150 roots, 6696 cells, 7862 forms.`
- ``//! (`derivation_set_shape_matches_the_audited_numbers`): 696 root×pada×lakāra`` → `744`
- `//! blocks × 9 cells, plus 1130 `ALTERNATES` rows.` → `1166`
- `//! Optionally dump the full 6264-cell table:` → `6696-cell`
- `assert_eq!(roots_seen.len(), 144, "curated roots");` → `150`
- `assert_eq!(n_cells, 6264, "cells: 696 root×pada×lakāra blocks × 9");` → `assert_eq!(n_cells, 6696, "cells: 744 root×pada×lakāra blocks × 9");`
- `assert_eq!(n_forms, 7394, "forms: 6264 cells + 1130 ALTERNATES rows");` → `assert_eq!(n_forms, 7862, "forms: 6696 cells + 1166 ALTERNATES rows");`

In `tools/audit/README.md`, `(144 roots, 6264 cells, 7394 forms)` → `(150 roots, 6696 cells, 7862 forms)`.

- [ ] **Step 2: Repoint vidyut's dev-deps at THIS worktree, run the prior-trace diff and the audit**

`/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes absolute dev-dep paths to `/workspace/crates`, the `main` checkout, which lacks this slice. An audit without repointing checks the pre-slice engine and fails on all 432 new cells (if the harness totals are raised) or passes vacuously (if not).

The throwaway example `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_10d.rs` exists from the prototype. If it is missing, recreate it:

```rust
//! THROWAWAY: slice 10d — dump every prior cell's live-branch credited-rule log.
use panini::Panini;
use panini_data::{Lakara as L, Purusha as P, Vacana as V};
const NEW: [&str; 6] = ["10.0118", "10.0119", "10.0120", "10.0121", "10.0122", "10.0123"];
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
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10d 2>/dev/null > "$DUMP/main.txt")
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#; s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml   # must point at $WT/crates
(cd $V && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10d 2>/dev/null > "$DUMP/branch.txt")
wc -l < "$DUMP/main.txt"          # 7394
diff "$DUMP/main.txt" "$DUMP/branch.txt" && echo PRIORS-IDENTICAL
```

Expected: `7394`, then `PRIORS-IDENTICAL`. If the diff prints any line, stop and report.

Then the audit. Copy the committed harness; never rewrite it.

```bash
cp tools/audit/panini_full_audit.rs $V/examples/
(cd $V && PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -6)
(cd $V && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -4)
```

- Expected from the honest run: `AUDIT PASSED: 6696 cells, 7862 forms, zero differences.`
- Expected from the `entry` control: `AUDIT FAILED: 36 differing cells.`

Do not use `mise -C`. If the honest run shows differences, stop and report, and edit nothing. After all runs, restore the dev-deps:

```bash
sed -i 's#^panini = { path = .*#panini = { path = "/workspace/crates/panini" }#; s#^panini-data = { path = .*#panini-data = { path = "/workspace/crates/panini-data" }#' $V/Cargo.toml
grep -n '^panini' $V/Cargo.toml
```

- [ ] **Step 3: Record the audit**

In `tools/audit/README.md`, immediately under `## Last recorded result` and its blank line, add a new entry above the 10c one. Set `<DATE>` from `date -u +%F`:

```markdown
<DATE>, curādi 10d slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 6696
cells / 7862 forms / 150 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole curādi 10d slice: six of the seven jñapādi rows
(`10.0118` through `10.0123`), ubhayapadī by 1.3.74, and the two rules that
derive them — the gaṇasūtra 10.0493 *jñapādayo mitaḥ*, which credits their
mit-tva, and 6.4.92 *mitāṃ hrasvaḥ*, which shortens back the upadhā 7.2.116
lengthened before ṇic. vidyut credits 6.4.92 later in its pipeline, after
7.3.84; the forms agree, and the harness compares forms. Before the slice
this engine derived all 432 new cells with the unshortened upadhā
(*jñāpayati*), every one a difference. A main-vs-branch dump of every prior
cell's traces was byte-identical.

Totals: 150 = 144 + 6; 6696 = 6264 + 432 (48 root×pada×lakāra blocks × 9);
7862 = 7394 + 432 + 36 new `ALTERNATES` rows (1130 → 1166), measured via
the harness's corpus block, not assumed.

```

In `crates/panini/tests/paradigm/main.rs`'s audit-chain doc comment, replace

```rust
/// cells), and curādi 10c's re-ran it at the same commit over all 6264 cells
/// / 7394 forms / 144 roots with zero differences, its `entry` negative
/// control verified failing (36 √bhū cells). √tṛh
```

with

```rust
/// cells), and curādi 10c's re-ran it at the same commit over all 6264 cells
/// / 7394 forms / 144 roots with zero differences, its `entry` negative
/// control verified failing (36 √bhū cells), and curādi 10d's re-ran it at
/// the same commit over all 6696 cells / 7862 forms / 150 roots with zero
/// differences, its `entry` negative control verified failing (36 √bhū
/// cells). √tṛh
```

- [ ] **Step 4: README.md**

Confirm each `old` occurs exactly once with `grep -c` before editing.
- `8.4.44 *śāt* exemption. *curādi* (10) is **open** at 41 of its 509` → `… at 47 of its 509`
- `ātmanepadinaḥ* — the engine's one rule that is not an Aṣṭādhyāyī sūtra.` → `ātmanepadinaḥ* — the engine's first rule that is not an Aṣṭādhyāyī sūtra.`
- Replace

```markdown
`10.0234`), among them √vid (*vedayate*), √śam (*śāmayate*) and the pair √mān
/ √man, which share every form (*mānayate*).
```

  with

```markdown
`10.0234`), among them √vid (*vedayate*), √śam (*śāmayate*) and the pair √mān
/ √man, which share every form (*mānayate*). Slice 10d curated six of the
seven jñapādi (`10.0118` through `10.0123`: √jñap, √yam, √cah, √cap, √rah,
√bal), ubhayapadī by 1.3.74 and mit by the gaṇasūtra 10.0493 *jñapādayo
mitaḥ*, the engine's second non-Aṣṭādhyāyī rule: 6.4.92 *mitāṃ hrasvaḥ*
shortens back the upadhā 7.2.116 lengthened before ṇic, so *jñapayati*, not
*jñāpayati*.
```

- `curated 144-root set,` → `curated 150-root set,`
- `both correct — and in fact 824 of the 6264 cells hold more than one form: 612` / `hold two, 165 hold three` → `both correct — and in fact 848 of the 6696 cells hold more than one form: 624` / `hold two, 177 hold three`
- `√jan's, and — new in slice 10a — the four curādi roots', each by` → `√jan's, and — new in slice 10a — the four curādi roots', and — new in slice` / `10d — the six jñapādi roots', each by`
- `padas — thirty roots that admit both padas in the curated set` → `padas — thirty-six roots that admit both padas in the curated set`
- `√laḍ, √bhakṣ and √bhūṣ by 1.3.74) derive a full` → `√laḍ, √bhakṣ, √bhūṣ, √jñap, √yam, √cah, √cap, √rah and √bal by 1.3.74) derive a full`
- `the same for `lAqaya-`, `Bakzaya-` and `BUzaya-`. That enumeration is no` → `the same for `lAqaya-`, `Bakzaya-` and `BUzaya-`, and slice 10d's six` / `jñapādi roots the same for `jYapaya-`, `yamaya-`, `cahaya-`, `capaya-`,` / `` `rahaya-` and `balaya-`. That enumeration is no``

Check: 848 = 6696 − 5848.

- [ ] **Step 5: docs/ARCHITECTURE.md**

Each `old` occurs exactly once.
- The stage table: `` | `sanadi.rs` | 10.0496, 3.1.25, 1.3.9, 3.4.114, 7.2.116, 7.3.86, 3.1.32 — the ākusmīya pada, then ṇic and its folding into the dhātu (curādi only) | `` → `` | `sanadi.rs` | 10.0496, 10.0493, 3.1.25, 1.3.9, 3.4.114, 7.2.116, 6.4.92, 7.3.86, 3.1.32 — the ākusmīya pada and the jñapādi's mit-tva, then ṇic and its folding into the dhātu (curādi only) | ``
- The rule census: replace `10.0496 *ā kusmād ātmanepadinaḥ*, first in `sanadi.rs` and the only id that` / `is not an Aṣṭādhyāyī sūtra — 137 total).` with `10.0496 *ā kusmād ātmanepadinaḥ*, first in `sanadi.rs` and the first id that` / `is not an Aṣṭādhyāyī sūtra — 137 total — then curādi 10d's two: the` / `gaṇasūtra 10.0493 *jñapādayo mitaḥ*, second in `sanadi.rs`, and 6.4.92` / `*mitāṃ hrasvaḥ*, right after 7.2.116 — 139 total).`
- `open** at 41 of its` / `509 rows (… slice 10b; thirty-three more ākusmīya roots, slice 10c). gaṇa` → `open** at 47 of its` / `509 rows (… slice 10b; thirty-three more ākusmīya roots, slice 10c; √jñap, √yam, √cah, √cap, √rah, √bal, slice 10d). gaṇa`
- `forking 164 cells (loṭ` / `prathama and madhyama eka across the 82 roots with a parasmaipada column —` → `forking 176 cells (loṭ` / `prathama and madhyama eka across the 88 roots with a parasmaipada column —`
- `roots never reach this guard, and the thirty roots that admit both` → `… the thirty-six roots that admit both`
- `√laḍ, √bhakṣ and √bhūṣ by 1.3.74) reach it in their` → `√laḍ, √bhakṣ, √bhūṣ, √jñap, √yam, √cah, √cap, √rah and √bal by 1.3.74) reach it in their`
- `82 + 62 = the 144 curated roots)` → `88 + 62 = the 150 curated roots)`
- `forking 170 cells outright: laṅ and vidhiliṅ prathama eka across` / `those same 82 parasmaipada columns (147 of them` → `forking 182 cells outright: …` / `those same 88 parasmaipada columns (159 of them`
- `10a's four curādi roots contribute both cells, except √cur, whose two key on its mandatory sanādi 7.3.86 as well, see below),` → `…, see below; 10d's six jñapādi roots contribute both cells),`
- `8.4.56 goes on` / `forking a further 164 (the same` → `forking a further 176 (the same`

Checks: 182 = 159 + 22 + 1 (`key_count("8.4.56")`); 176 = 164 + 12; 88 + 62 = 150. `grep -c "pada: PadaAssignment::Atmanepada,\|pada: PadaAssignment::Akusmiya," crates/panini-data/src/lib.rs` still prints `62`.

- [ ] **Step 6: AGENTS.md**

Each `old` occurs exactly once.
- `` (`crates/panini/tests/paradigm/`, 6264 cells, ten gaṇas, nine complete — `` → `6696 cells`
- `at 41 after slice 10c curated thirty-three more ākusmīya roots —` → `at 41 after slice 10c curated thirty-three more ākusmīya roots, at 47 after slice 10d curated the jñapādi √jñap, √yam, √cah, √cap, √rah and √bal —`
- `other forms — a second (612 cells), a third (165 cells), a fourth` → `(624 cells)`, `(177 cells)`
- `` `ALTERNATES` (1130 rows in all, so 6264 + 1130 = 7394 forms total); √bhuj `` → `` `ALTERNATES` (1166 rows in all, so 6696 + 1166 = 7862 forms total); √bhuj ``
- The audit chain: replace `  6264 cells / 7394 forms / 144 roots).` (the line ending the 10c link) with `  6264 cells / 7394 forms / 144 roots), and that by curādi 10d's` / `` (`tools/audit/README.md`'s <DATE> 10d entry, 6696 cells / 7862 forms / `` / `  150 roots).`
- `not wrong in kind: 6264 goldens` → `not wrong in kind: 6696 goldens`
- The stale-comment ledger. After the sentence ``Curādi 10c touched neither comment either; the corpus stands at 6264 cells as of 10c (`guna.rs:2565`'s claim anchored at `guna.rs:2565`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit).``, insert ``Curādi 10d touched neither comment either; the corpus stands at 6696 cells as of 10d (`guna.rs:2565`'s claim anchored at `guna.rs:<G>`, `controller.rs:206`'s at `controller.rs:<C>`; both lines measured by grep at this commit).`` Measure `<G>` with `grep -n "1872 goldens move" crates/panini-prakriya/src/tinanta/guna.rs` and `<C>` with `grep -n "only 8 cells fire" crates/panini-prakriya/src/controller.rs`. Both were 2565 and 206 in the prototype. Measure; never compute.

AGENTS.md's floor paragraph (`measured at 6264 cells`) and the current mutation record belong to Task 5.

- [ ] **Step 7: The engine comment and the 10c spec**

`crates/panini-prakriya/src/tinanta/guna.rs`, 6.1.78's comment: `144-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)` → `150-root × …`. The claim still holds: every new aṅga ends in ṇic's `i`. This is an in-place edit; no line shifts.

`docs/superpowers/specs/2026-10-02-curadi-gana-10c-design.md`:
- After the out-of-scope bullet that ends `  roots generally, and everything else on 10a's and 10b's out-of-scope lists.`, append to the bullet: `` (Slice 10d took the mit roots — see `2026-10-02-curadi-gana-10d-design.md` — `` / `and moved 01.0934 and 10.0494 to a causative slice: without a causative,` / `01.0934 can never fire.)`
- After `these goldens fail unless it also adds 10.0494.` append ` (Slice 10d, the mit slice,` / `did not add 01.0934; the witnesses pass to a causative slice.)`
- In "Later slices", `10.0494, with √syam and √śam as the witnesses), adanta roots` → `10.0494, with √syam and √śam as the witnesses; slice 10d took the jñapādi and` / `deferred both gaṇasūtras to a causative slice), adanta roots`

- [ ] **Step 8: Sweep for anything left stale**

```bash
grep -rn -E "\b6264\b|\b7394\b|\b696\b|\b1130\b|144 roots|144-root|of these 144|of the 144|144 curated|41 of|thirty roots|four curādi roots by|82 \+ 62|\b5440\b|the only id|one rule that is not|the mit slice|seventy-two" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.md --include=*.rs | grep -v "paradigm/data/"
grep -rn "√cur, √laḍ, √bhakṣ and √bhūṣ\|1.3.74" README.md AGENTS.md docs/ARCHITECTURE.md crates/panini-prakriya/src crates/panini-data/src --include=*.md --include=*.rs
```

Expected residue in the first grep:
- AGENTS.md's floor paragraph (`measured at 6264 cells`), which Task 5 rewrites;
- dated history, which must never be rewritten: AGENTS.md's audit chain and mutation record; `tools/audit/README.md`'s older entries and the 10d entry's own `144 + 6` / `6264 + 432` / `7394 + 432` totals; the audit chain and the 10c paragraph in `paradigm/main.rs` ("OPEN at 41"); the pada-ambiguous comment's "fifty-six to seventy-two" and "seventy-two to ninety-six"; `lan_a_form("01.1130"` and other `01.1130` row numbers.

Each of these describes an earlier step and stays true.

For every hit of the second grep, check that a sentence enumerating "the 1.3.74 roots" as 10a's four is either explicitly 10a's or updated.

- [ ] **Step 9: Run the full suite and commit**

Run: `mise run test 2>&1 | grep -E "FAILED|test result" | grep -v " 0 passed"` (foreground, timeout 600000 ms). Expected: PASS at 6696 cells.

```bash
mise run fmt && mise run lint
git add -A
git commit -m "docs: 10d's counts, the audit record, and the sweep

6696 cells / 7862 forms / 150 roots across README, ARCHITECTURE, AGENTS,
paradigm/main.rs and tools/audit; curādi open at 47/509; 139 rules; 36
both-pada roots. Audit at zero divergence against 8da2f90b; prior traces
byte-identical to main."
```

---

## Task 5: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and the current-record paragraph); `mise.toml` only if the cap moves

Follow AGENTS.md's cargo-mutants protocol. Hazards from this repo's record:
- **Measure, never scale.**
- **Every invocation rotates `mutants.out`**, so always pass `-o`.
- **The mise shim fails.** Use the real binary: `CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`.
- **`pgrep -f` matches its own shell.** Wait on `pgrep -x cargo-mutants`.
- **Background shells die at about 60 minutes.** Launch detached with `setsid nohup`, as below.

This slice adds mutated code (`sanadi.rs`, `mod.rs`) but touches neither `adesha.rs` nor `tripadi.rs`. So the three documented non-caught entries (`adesha.rs:589:30`, `tripadi.rs:1289:38`, `tripadi.rs:1602:23`) should not move; the prototype's `--list` confirmed all three. Confirm them by `--list`, never by assumption.

Expected totals, from the prototype's `--list` of 821 mutants:
- 815 + 6 new: `mod.rs`'s `&&`→`||` and `==`→`!=` in the `Tag::Mit` read, 10.0493's `delete !`, and 6.4.92's `||`→`&&` and two `delete !`.
- All 6 new mutants were caught in the prototype's `--in-diff` run.

The cap is 200.

- [ ] **Step 1: Measure the floor**

With nothing else running, run this twice: `time mise run test 2>&1 | tail -3` (foreground). Record both wall clocks and `cat /proc/loadavg`. The 6264-cell floor was 16.008s / 15.686s, under host load of about 16-27.

- [ ] **Step 2: Locate and probe the two uncaught equivalents at `-j 4`**

```bash
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /="
```

Expect positions `589` (adesha), `1289` (tripadi, inside 8.3.13's `apply`) and `1602` (the ṇatva hang). Write them as `<A>`, `<T1>` and `<T2>`. Then run in the foreground with timeout 600000 ms:

```bash
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 200 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:<A>:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" 2>&1 | tail -10
```

Both must be MISSED, not TIMEOUT. Read each test-phase duration from `$SCRATCH/mutants.out/outcomes.json`.

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/curadi-10d"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout 200 -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"
```

The last campaign took about 26 minutes. Run nothing CPU-heavy meanwhile. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

- [ ] **Step 4: Read the outcomes**

When `pgrep -x cargo-mutants` returns nothing:

```bash
date -u +"%F %T UTC" > "$OUT/finished"
cp "$OUT/mutants.out/outcomes.json" "$OUT/outcomes.durable.json"
tail -5 "$OUT/campaign.log"
cat "$OUT/mutants.out/missed.txt" "$OUT/mutants.out/timeout.txt"
```

Expected (exit code 3 is normal when a timeout is present):
- **821 mutants: 770 caught, 48 unviable, 2 missed, 1 timeout.**
  - panini-prakriya: 809 / 762 / 44 / 2 / 1.
  - panini-analyze: 12 / 8 / 4 / 0 / 0.
- `missed.txt` holds exactly `adesha.rs:<A>:30: replace + with *` and `tripadi.rs:<T1>:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:<T2>:23: replace -= with /=`.

If not:
- Any **other timeout** is a suspect survivor that the larger suite pushed past the cap. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant in `sanadi.rs` or `mod.rs` is a gap in Task 2's tests; add the test that kills it. Any other missed mutant means a test that caught it at 6264 cells no longer does; stop and report.

- [ ] **Step 5: Margins**

```bash
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["outcomes"][1])' "$OUT/mutants.out/outcomes.json"
```

Compute the two equivalents' test phases under campaign load, and the caught min/median/p90/max. The cap is max(200, 6 × the longest campaign-load equivalent phase, rounded up to the next 10 s).
- If that is 200, the cap stays.
- Otherwise change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together.

- [ ] **Step 6: Record it in AGENTS.md**

- Rewrite the paragraph that opens `**The floor behind the 200s cap, measured at 6264 cells on Rust 1.99.0,` / `2026-10-02.**`. Use Step 1's and Step 2's numbers at 6696 cells, the load averages, and the cap Step 5 chose. Keep the comparison chain to earlier floors, with 10c's 16.008s / 15.686s at 6264 cells joining it.
- Replace the `**Current record (curādi 10c, 2026-10-02).**` paragraph with `**Current record (curādi 10d, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**;
  - the non-caught set diffed against 10c's (the clean result is identical, lines and columns included, since no mutated line in `adesha.rs` or `tripadi.rs` moved);
  - the six new-code mutants, each named with its outcome (all caught);
  - the campaign-load phases and margins;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`.

  End it with a pointer to the record it replaces. Run `git rev-parse --short HEAD` before committing, and write ``The curādi 10c record it replaces: `git show <that hash>:AGENTS.md`.``

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md mise.toml
git commit -m "chore: 10d mutation gate — floor and uncaught run re-measured at 6696 cells

All six new-code mutants (10.0493, 6.4.92, the Tag::Mit read) caught;
missed.txt holds only the two documented equivalents and timeout.txt only
the permanent ṇatva-scan entry, identical to 10c's."
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
git push -u origin curadi-10d
gh pr create --title "curādi 10d — the jñapādi mit roots" --body "$(cat <<'BODY'
Slice 10d curates six of curādi's seven jñapādi roots (`10.0118`–`10.0123`:
√jñap, √yam, √cah, √cap, √rah, √bal), ubhayapadī by 1.3.74, and adds the two
rules they need, both in the sanādi stage:

- **10.0493 *jñapādayo mitaḥ*** credits the roots' mit-tva. `derive` tags
  `Tag::Mit` from the new `JNAPADI` range, which is pinned to upstream.
- **6.4.92 *mitāṃ hrasvaḥ*** shortens back the upadhā that 7.2.116
  lengthened before ṇic: *jñapayati*, not *jñāpayati*. vidyut credits it
  later (after 7.3.84); the forms agree, and the placement is argued in its
  comment.

**Deferred:**
- √ci (`10.0124`) waits for 7.2.115.
- 01.0934 and 10.0494 wait for a causative slice; without one, 01.0934
  can never fire.

The golden suite goes from 6264 to 6696 cells; curādi is open at 47 of 509.
The audit shows zero divergence against `8da2f90b`; a main-vs-branch dump of
every prior cell's traces is byte-identical; the mutation gate catches every
new mutant.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`, run `git worktree remove .worktrees/curadi-10d` and `git worktree remove --force .worktrees/curadi-10d-proto` (the throwaway). Then delete the local and remote `curadi-10d` branch, and run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| `JNAPADI` range incl. `ciY`, pinned to upstream | 2 |
| `Tag::Mit`, set in `derive` for curādi rows in range | 2 |
| 10.0493 (record-only, after 10.0496) and 6.4.92 (after 7.2.116, `hrasva_of`, index computed once) | 2 |
| Order pin and its doc; unit tests; `Tag::Mit` derive test | 2 |
| Six rows, `Nic`, comments (√yam's am-final note) | 3 |
| √syam/√śam comments → causative slice | 3 |
| 48 goldens + 36 alternates; totals 150 / 6696 / 7862; buckets 5848 / 624 / 177; ALTERNATES 1166 and key counts | 3 |
| Pada-ambiguous 72 → 96 | 3 |
| 1.3.74 list → ten; √jñap trace; √syam/√śam keep vṛddhi; 468 credits per rule, all in range | 3 |
| `check()`: all six rows both padas, `ajYapayata` two analyses, lengthened shapes Invalid | 3 |
| Prior traces byte-identical | 4 Step 2 |
| Audit with repoint and negative control; record | 4 |
| README / ARCHITECTURE (139 rules, 176 / 182 / 159, 88 + 62) / AGENTS; 10c spec pointers | 4 |
| Floor, uncaught probe, campaign, verbatim non-caught record, new-code mutants named | 5 |

**Type consistency.**
- `JNAPADI: RangeInclusive<&str>` is read as `JNAPADI.contains(&dhatu.dhatupatha)` in `derive` (`&&str`), as `JNAPADI.contains(n)` with `n: &&str` in the upstream pin's filter, and as `JNAPADI.contains(number)` with `number: &&'static str` in the credit loop. All compiled in the prototype.
- `hrasva_of(char) -> Option<char>` is existing and unchanged.
- The golden tuple shapes match `ParadigmRow` and `AlternateRow`.

**Known soft spots.**
- **Doc strings in Task 4** were read at `e594fe1` plus this slice's code. If one is not found exactly once, edit the paragraph to the same facts rather than skip it.
- **The floor in Task 5** depends on host load. Record the load beside every timing.
