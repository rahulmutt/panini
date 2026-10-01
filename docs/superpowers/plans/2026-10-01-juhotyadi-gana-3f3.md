# Juhotyādi gaṇa slice 3f3 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate √jan (`03.0025 jana~`, *jajanti*), parasmaipadī, the last juhotyādi row. The golden suite goes from 4608 to 4644 cells and the gaṇa closes at 26 of its 26 rows. The slice adds three rules in `guna.rs`: 6.4.98 *gamahanajanakhanaghasāṁ lopaḥ*, 6.4.42 *janasanakhanāṁ sañjhaloḥ*, and the engine's twelfth vikalpa, 6.4.43 *ye vibhāṣā*. It also gives 8.4.40 *stoḥ ścunā ścuḥ* its converse arm (a stu after a ścu), guarded by the 8.4.44 *śāt* exemption.

**Architecture:** Nine tasks.
- **Task 1** creates the worktree on the existing branch.
- **Tasks 2–4** are the whole engine change. None of it fires on a curated cell until Task 5 lands the row, so each is gated on **its unit tests plus the 4608 priors staying green**.
- **Task 5** lands the row and its goldens, turning 36 cells green.
- **Task 6** adds the trace pins and the corpus-wide fires-only tests.
- **Tasks 7–9** are the audit with the prior-trace diff and doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.99.0 pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, checked out at `/tmp/vidyut-full`.

**Spec:** `docs/superpowers/specs/2026-10-01-juhotyadi-gana-3f3-design.md`

**Workspace:** the branch `juhotyadi-3f3` already exists and holds the spec and this plan. Task 1 checks it out at `/workspace/.worktrees/juhotyadi-3f3`. Every path below is relative to that directory unless it starts with `/`.

**Provenance.** Every code block in this plan ran green on a throwaway worktree at `74eb975` (the spec commit). That run covered:
- the full suite (641 tests) and clippy `-D warnings`;
- the √jan goldens matching vidyut 36/36;
- the audit at 4644 / 5750 with zero differences, and its `entry` control failing on 36 cells;
- a byte-identical dump of all 5699 prior prakriyā logs, main vs prototype;
- a scoped mutation probe over the new code: 17 mutants, all caught.

## Global Constraints

- **The 4608 pre-existing cells must stay byte-identical, traces included.** Regenerate no golden and change no pinned trace.
- **Three new rules, one of them a vikalpa.** `tinanta_rule_order_is_pinned` grows from 126 to 129 ids, gaining exactly `6.4.98` (between `6.4.113` and `6.4.100`) and `6.4.42`, `6.4.43` (between `6.4.115` and `6.1.101`). `exactly_the_pinned_vikalpa_rules_are_optional` grows from eleven to twelve, gaining `6.4.43` between `6.4.115` and `6.4.107`.
- **All three are keyed by row number**, never by `ANGA.text == "jan"`. 6.4.98 keys `03.0025`. 6.4.42 and 6.4.43 key `JANA_SANA = ["03.0025", "08.0002"]`: √jan and tanādi's √san. √san must decline on its follower (the vikaraṇa `u`), not on a missing key.
- **6.4.42 and 6.4.43 sit after 6.4.115, at the end of `GUNA`**, below every ā-of-abhyasta rule (6.4.119, 6.4.118, 6.4.117, 6.4.116, 6.4.113, 6.4.112). Earlier, 6.4.112 or 6.4.113 eats the new ā (*jajtaH, *jajItaH).
- **8.4.40's 8.4.44 exemption is a guard (`left != 'S'`), not a rule.** No `8.4.44` id is added and no prior trace moves.
- **No unfalsifiable guard clauses.** A clause no cell and no guard test can make false is a mutation survivor. Witness it, delete it, or kill it with a direct guard test on a hand-built `Prakriya`.
- **Goldens are transcribed from this plan**: the prototype engine's output, checked cell by cell against vidyut at `8da2f90b` (36/36) on 2026-10-01. **ALTERNATES keys are this engine's** (its log ∩ `VIKALPA_RULES`). Never invent one. **Do not edit a golden to match the engine.**
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit.
- `mise run test` takes a few seconds. Run it in the **foreground** with a timeout of 600000 ms. Never background it and end a turn.
- `mise run test -- -p X` does not scope. Scope unit tests with `mise exec -- cargo test -p <crate> <filter>`.
- Rule ids and SLP1 names, verbatim: `6.4.98 gamahanajanaKanaGasAM lopaH kNityanaNi`, `6.4.42 janasanaKanAM saYJaloH`, `6.4.43 ye viBAzA`, `8.4.40 stoH ScunA ScuH`.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **√san, the other curated row in 6.4.42/6.4.43's key.** Its follower is the vikaraṇa `u`, which is not ṅit, so neither rule may fire. → Task 4, `janasana_and_ye_vibhasa_decline_on_san_where_u_intervenes`; Task 6, `gamahana_janasana_and_ye_vibhasa_are_credited_only_on_jan`.
2. **The new `jA` meeting the ā-of-abhyasta rules.** If 6.4.42 ever moves above them, 6.4.112/6.4.113 rewrite it. → Task 4, `the_abhyasta_rules_would_eat_janas_a_so_janasana_runs_after_them`; Task 6, `jajAtaH_trace_takes_the_a_without_the_abhyasta_rules`.
3. **A stu after a `S`** (√kliś's *kliSnAti*, √aś's *aSnoti*). The converse arm must not fire. → Task 2, the kliSnAti case in `shcutva_fires_on_either_side_of_a_shcu_and_declines_after_sha`; the 118 kliś/aś goldens.
4. **A new 8.4.40 firing on a prior row.** A trace step carries no direction, and a converse firing could leave a golden unchanged. → Task 6, `shcutva_off_jan_is_credited_exactly_as_before_3f3` (54 branches, `07.0003`/`07.0008` only, measured on `main`).
5. **`check()` on the new forms**, including the alternate *jajAyAt*. Each must resolve to √jan only. → Task 5, `jan_analyses_its_reduplicated_forms`.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | Task 2: 8.4.40's converse arm, its comment, its unit test |
| `crates/panini-prakriya/src/tinanta/guna.rs` | Task 3: 6.4.98 and its tests. Task 4: `JANA_SANA`, `janasanakhanam_a`, 6.4.42, 6.4.43, their tests, the module doc, 6.4.115's comment. Task 7: one stale count |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Tasks 3, 4: the rule-order and vikalpa pins |
| `crates/panini-prakriya/src/tinanta/anga.rs` | Task 7: one stale comment |
| `crates/panini-data/src/lib.rs` | Task 5: the row, counts, the gaṇa-row test |
| `crates/panini/tests/paradigm/data/juhotyadi.rs` | Task 5: 4 `PARADIGM` blocks, 15 `ALTERNATES` rows |
| `crates/panini/tests/paradigm/main.rs` | Task 5: `VIKALPA_RULES`, totals, buckets, key census, doc comments, the `check()` test. Task 7: audit prose |
| `crates/panini/tests/trace/juhotyadi.rs` | Task 5: the 7.3.87 and 8.3.24 allow-lists. Task 6: six pins, two corpus-wide tests |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md` | Task 7 |
| `AGENTS.md`, maybe `mise.toml` | Task 8 |

---

## Task 1: The worktree

**Files:** none.

**Interfaces:** none.

- [ ] **Step 1: Free the branch and create the worktree**

`/workspace` itself has `juhotyadi-3f3` checked out (the spec and plan commits). A branch can be checked out in only one worktree, so move `/workspace` back to `main` first. Follow superpowers:using-git-worktrees. From `/workspace`:

```bash
git -C /workspace status --short          # must be empty
git -C /workspace switch main
git -C /workspace worktree add .worktrees/juhotyadi-3f3 juhotyadi-3f3
cd /workspace/.worktrees/juhotyadi-3f3
git log --oneline -3                      # the plan commit, the spec commit, then 436a507
```

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -15
```

Run in the foreground, timeout 600000 ms. Expected: all pass at 4608 cells.

---

## Task 2: 8.4.40's converse arm, with *śāt*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs`: 8.4.40 (comment from about line 977, `apply` about line 1069), and its unit test in `mod tests` (about line 2660)

**Interfaces:**
- Consumes: `word_chars(p) -> Vec<(usize, usize, char)>` (term index, char index within the term, char), `set_char(p, term, idx, char)`, `is_shcu(char) -> bool` (`S c C j J Y`), `shcutva_of(char) -> Option<char>` (`s t T d D n` only). All are already imported in `tripadi.rs`.
- Produces: nothing later tasks call. Task 6's `shcutva_off_jan_is_credited_exactly_as_before_3f3` is named in this task's comment.

- [ ] **Step 1: Write the failing test**

Rename `shcutva_fires_on_stu_before_shcu_and_declines_after_sha` to `shcutva_fires_on_either_side_of_a_shcu_and_declines_after_sha`. (`grep -rn shcutva_fires_on_stu_before crates AGENTS.md docs/ARCHITECTURE.md` finds no other reference.) In its body, replace

```rust
        // 8.4.44 SAt: a stu FOLLOWING a `S` is exempt, and this engine
        // implements that exemption by not implementing the direction at
        // all. Fire here and √kliś surfaces *kliSYAti -- 41 invocations of
        // 8.4.44 on that one root in vidyut-prakriya over this corpus.
```

with

```rust
        // √jan laṭ prathama bahu after 6.4.98: the stu `n` FOLLOWS the ścu
        // `j` -- the converse arm (slice 3f3).
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("jn"), Term::new(""), Term::new("ati")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "ja".into();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "jajYati");

        // 8.4.44 SAt: a stu FOLLOWING a `S` is exempt, and the converse arm
        // carries that exemption as its guard. Without it √kliś surfaces
        // *kliSYAti -- 41 invocations of 8.4.44 on that one root in
        // vidyut-prakriya over this corpus.
```

The kliSnAti assertions that follow stay as they are.

- [ ] **Step 2: Run the test to verify it fails**

Run: `mise exec -- cargo test -p panini-prakriya shcutva_fires 2>&1 | tail -20`
Expected: FAIL at `assert!((rule.apply)(&mut p))` on the `jn` case: the one-direction rule sees no ścu after a stu.

- [ ] **Step 3: Replace the scan**

In 8.4.40's `apply` (`id: "8.4.40"`), replace

```rust
            let w = word_chars(p);
            for i in 0..w.len().saturating_sub(1) {
                if !is_shcu(w[i + 1].2) {
                    continue;
                }
                let Some(sub) = shcutva_of(w[i].2) else {
                    continue;
                };
                let (term, idx, _) = w[i];
```

with

```rust
            let w = word_chars(p);
            for i in 0..w.len().saturating_sub(1) {
                let (left, right) = (w[i].2, w[i + 1].2);
                let target = if is_shcu(right) {
                    i
                } else if is_shcu(left) && left != 'S' {
                    i + 1
                } else {
                    continue;
                };
                let Some(sub) = shcutva_of(w[target].2) else {
                    continue;
                };
                let (term, idx, _) = w[target];
```

The rest of `apply` (`let before = p.snapshot();` onward) is unchanged.

- [ ] **Step 4: Rewrite the comment**

Replace the rule's opening paragraph

```rust
    // 8.4.40 stoH ScunA ScuH: a stu (`s` and the t-varga) in contact with a
    // ścu (`S` and the c-varga) takes its own ścu counterpart.
    // atCinad → acCinad; atCfRad → acCfRad.
```

with

```rust
    // 8.4.40 stoH ScunA ScuH: a stu (`s` and the t-varga) in contact with a
    // ścu (`S` and the c-varga) takes its own ścu counterpart, on either side
    // of it. Stu before ścu: atCinad → acCinad; atCfRad → acCfRad. Ścu before
    // stu: ja + jn + ati → jajYati, ajajYuH, jajYatu (√jan, once 6.4.98 has
    // elided its upadhā; slice 3f3).
```

Replace the two paragraphs from `// 8.4.41 next door scans the mirror image of this rule's search —` down to and including `// does not cover.` (the end of the ONE DIRECTION ONLY paragraph) with:

```rust
    // 8.4.41 next door scans for the same "a stu takes its neighbour's
    // class" pattern against the ṭu-varga instead of the c-varga, in the
    // trigger-then-target direction only — this rule's converse arm's
    // direction.
    //
    // BOTH DIRECTIONS since slice 3f3, and the converse arm carries 8.4.44
    // SAt as a guard: a stu that FOLLOWS a `S` is exempt. Until 3f3 the arm
    // was deliberately left out, because SAt was the only thing it would
    // ever meet — vidyut-prakriya credits 8.4.44 one hundred and eighteen
    // times over this corpus, every one an `S` before an `n`: aSnoti
    // (`05.0020`, 36; `09.0059`, 41) and kliSnAti (`09.0058`, 41). Shipped
    // without SAt it turns kliSnAti into *kliSYAti; shipped with SAt and no
    // witness it was code no cell could make fire. √jan's `jn` is the first
    // ścu-before-stu site SAt does not cover, so the two shipped together.
    // SAt is a guard here, not a rule of its own: a crediting 8.4.44 would
    // move those 118 prior traces for no change of form. Over the whole
    // corpus the converse arm fires on √jan alone
    // (`shcutva_off_jan_is_credited_exactly_as_before_3f3` in `panini`'s
    // trace suite).
    //
    // The forward arm is tried first at each position. The two cannot both
    // match one pair: the forward arm needs a ścu on the right, the converse
    // a stu there, and no sound is both.
```

The `// THIS CORPUS DOES present stu-immediately-before-ścu sites` paragraph that follows stays. Further down, in the paragraph beginning `// The rules below are inert on the site this one writes.`, replace

```rust
    // between — so it is not an 8.4.2 intervener question either.
```

with

```rust
    // between — so it is not an 8.4.2 intervener question either. The
    // converse arm's only output, √jan's `Y`, is a nasal: no jhal for 8.4.53,
    // 8.4.55 or 8.4.65, and not the dental `n` 8.4.1 retroflexes.
```

- [ ] **Step 5: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS.

- [ ] **Step 6: Run the full suite (priors unchanged)**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms)
Expected: PASS at 4608 cells. The arm cannot fire on a prior row: the spec's dump shows no ścu-before-stu site outside √kliś/√aś's `Sn`. If a prior golden fails, stop and report the cell.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/tripadi.rs
git commit -m "feat(tripadi): 8.4.40 stoḥ ścunā ścuḥ gains its converse arm, with 8.4.44 śāt as its guard"
```

---

## Task 3: 6.4.98 *gamahanajanakhanaghasāṁ lopaḥ*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (a new rule immediately before 6.4.100; `mod tests`)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: `following_sarvadhatuka(p) -> Option<&Term>`, `is_vowel(char) -> bool`, `Tag::Ngit`, `ANGA`, all already imported in `guna.rs`. The existing test helper `abhyasta_prakriya(abhyasa: &str, anga: &str, ghu: bool, ending: &str, ngit: bool) -> Prakriya` in `guna.rs`'s `mod tests`, which builds abhyāsa + aṅga (both tagged Abhyasta) + empty SHAP + ending (tagged Ngit when `ngit`). `Context::dhatupatha` is `&'static str`.
- Produces: the test helper `jan_prakriya(number: &'static str, anga: &str, ending: &str, ngit: bool) -> Prakriya`, reused in Task 4. The rule id `"6.4.98"`, read by Task 6.

- [ ] **Step 1: Write the failing tests**

At the end of `guna.rs`'s `mod tests`, after `ghasibhasor_declines_before_a_pit_off_its_row_and_on_bs`, add:

```rust

    // --- 6.4.98 / 6.4.42 / 6.4.43: √jan (slice 3f3) ------------------------

    /// √jan at the guṇa stage: abhyāsa `ja` (7.4.60 has run), aṅga `anga`, an
    /// empty śap, and `ending` (tagged Ngit when `ngit`), on the row `number`.
    fn jan_prakriya(number: &'static str, anga: &str, ending: &str, ngit: bool) -> Prakriya {
        let mut p = abhyasta_prakriya("ja", anga, false, ending, ngit);
        p.ctx.dhatupatha = number;
        p
    }

    #[test]
    fn gamahana_elides_the_upadha_before_a_vowel_initial_kngit() {
        // ati (jajYati), us (ajajYuH), atu (jajYatu).
        let rule = rules().find(|r| r.id == "6.4.98").unwrap();
        for ending in ["ati", "us", "atu"] {
            let mut p = jan_prakriya("03.0025", "jan", ending, true);
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ANGA].text, "jn", "{ending}");
            assert_eq!(p.log.last().unwrap().sutra, "6.4.98");
        }
    }

    #[test]
    fn gamahana_declines_off_a_vowel_initial_kngit_off_its_row_and_on_jn() {
        let rule = rules().find(|r| r.id == "6.4.98").unwrap();
        for (number, anga, ending, ngit, why) in [
            ("03.0025", "jan", "tas", true, "consonant-initial: 6.4.42's"),
            ("03.0025", "jan", "Ani", false, "pit āṭ: jajanAni"),
            ("03.0019", "jan", "ati", true, "another juhotyādi row"),
            ("", "jan", "ati", true, "a hand-built prakriyā names no row"),
            ("03.0025", "jn", "ati", true, "the upadhā is already gone"),
        ] {
            let mut p = jan_prakriya(number, anga, ending, ngit);
            assert!(!(rule.apply)(&mut p), "{why}");
            assert_eq!(p.terms[ANGA].text, anga, "{why}");
            assert!(p.log.is_empty(), "{why}");
        }
    }
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya gamahana 2>&1 | tail -20`
Expected: both FAIL, panicking on `unwrap()`: there is no rule `6.4.98`.

- [ ] **Step 3: Add the rule**

In `guna.rs`'s `GUNA`, insert this immediately before the comment `// 6.4.100 ghasibhasor hali ca:`:

```rust
    // 6.4.98 gamahanajanakhanaghasāṁ lopaḥ kṅity anaṅi: the upadhā `a` of
    // √gam, √han, √jan, √khan and √ghas is elided before a VOWEL-INITIAL
    // kṅit (*aci*, by anuvṛtti from 6.4.77). ja + jan + ati → ja + jn + ati,
    // which 8.4.40 finishes to jajYati; likewise ajajYuH (`us`) and jajYatu
    // (`atu`). The loṭ uttama endings are pit (3.4.92's āṭ), so jajanAni keeps
    // its `a`; a consonant-initial kṅit is 6.4.42's (jajAtaH). *anaṅi* is
    // vacuous here: aṅ is a luṅ vikaraṇa, and this engine derives no luṅ.
    //
    // KEYED BY ROW NUMBER, `03.0025 jana~`, as 6.4.100 keys √bhas: of the
    // sūtra's five roots only √jan is curated, and a √gam, √han, √khan or
    // √ghas row extends this key. The `an` suffix test is the operation
    // itself, and it declines on an already-elided `jn` or on 6.4.42's `jA`.
    //
    // PLACEMENT: beside 6.4.100, its sibling upadhā-lopa, in sūtra order. No
    // ā-rule in this block reads `jan` or `jn`, so no form depends on it;
    // `jajYati_trace_is_upadha_lopa_then_shcutva` in `panini`'s trace suite
    // pins it.
    Rule {
        id: "6.4.98",
        name: "gamahanajanaKanaGasAM lopaH kNityanaNi",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if p.ctx.dhatupatha != "03.0025" {
                return false;
            }
            let Some(follower) = following_sarvadhatuka(p) else {
                return false;
            };
            if !follower.has(Tag::Ngit) {
                return false;
            }
            if !follower.text.chars().next().is_some_and(is_vowel) {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix("an") else {
                return false;
            };
            let elided = format!("{stem}n");
            let before = p.snapshot();
            p.terms[ANGA].text = elided;
            p.record("6.4.98", "gamahanajanaKanaGasAM lopaH kNityanaNi", before);
            true
        },
    },
```

`elided` is computed before `p.snapshot()` because `stem` borrows `p.terms`.

- [ ] **Step 4: Pin the order**

In `derivation_tests.rs`'s `tinanta_rule_order_is_pinned`, change `"6.4.113", "6.4.100",` to `"6.4.113", "6.4.98", "6.4.100",`. `mise run fmt` rewraps the array.

- [ ] **Step 5: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS, including `tinanta_rule_order_is_pinned` and an unchanged `exactly_the_pinned_vikalpa_rules_are_optional`.

- [ ] **Step 6: Run the full suite**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms)
Expected: PASS at 4608 cells. The rule is keyed to a row that is not curated yet, so it fires nowhere.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/guna.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(guna): 6.4.98 gamahanajanakhanaghasāṁ lopaḥ — √jan's upadhā before a vowel-initial kṅit"
```

---

## Task 4: 6.4.42 *janasanakhanāṁ sañjhaloḥ* and the vikalpa 6.4.43 *ye vibhāṣā*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (the module doc, imports, a const and a helper above `GUNA`, two rules at the end of `GUNA`, 6.4.115's comment, `mod tests`)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (both pins)

**Interfaces:**
- Consumes: Task 3's `jan_prakriya(number, anga, ending, ngit) -> Prakriya`. `is_jhal(char) -> bool` from `crate::tinanta::sound`, and `Prakriya` from `crate::prakriya`, both newly imported here. In `mod tests`, `with_slots`, `Term`, `ENDING`, `Tag` and `Prakriya` are already in scope.
- Produces: `const JANA_SANA: &[&str]` and `fn janasanakhanam_a(p: &mut Prakriya, id: &str, name: &str, takes: fn(char) -> bool) -> bool`, both private to `guna.rs`. The rule ids `"6.4.42"` and `"6.4.43"`, read by Tasks 5 and 6.

- [ ] **Step 1: Write the failing tests**

At the end of `guna.rs`'s `mod tests`, after Task 3's tests, add:

```rust

    #[test]
    fn janasana_takes_a_before_a_jhal_initial_kngit() {
        // tas (jajAtaH), Ta (jajATa), hi (jajAhi), tAt (jajAtAt).
        let rule = rules().find(|r| r.id == "6.4.42").unwrap();
        for ending in ["tas", "Ta", "hi", "tAt"] {
            let mut p = jan_prakriya("03.0025", "jan", ending, true);
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ANGA].text, "jA", "{ending}");
            assert_eq!(p.log.last().unwrap().sutra, "6.4.42");
        }
    }

    #[test]
    fn ye_vibhasa_takes_a_before_a_y_initial_kngit() {
        let rule = rules().find(|r| r.id == "6.4.43").unwrap();
        let mut p = jan_prakriya("03.0025", "jan", "yAt", true);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "jA");
        assert_eq!(p.log.last().unwrap().sutra, "6.4.43");
    }

    #[test]
    fn janasana_and_ye_vibhasa_decline_off_their_follower_class_row_and_shape() {
        let r42 = rules().find(|r| r.id == "6.4.42").unwrap();
        let r43 = rules().find(|r| r.id == "6.4.43").unwrap();
        for (rule, number, anga, ending, ngit, why) in [
            (
                r42,
                "03.0025",
                "jan",
                "yAt",
                true,
                "6.4.42 before y: 6.4.43's",
            ),
            (
                r42,
                "03.0025",
                "jan",
                "ati",
                true,
                "6.4.42 before a vowel: 6.4.98's",
            ),
            (
                r43,
                "03.0025",
                "jan",
                "tas",
                true,
                "6.4.43 before a jhal: 6.4.42's",
            ),
            (r42, "03.0025", "jan", "ti", false, "pit tip: jajanti"),
            (r43, "03.0025", "jan", "yAt", false, "a y-initial pit"),
            (r42, "03.0024", "jan", "tas", true, "another juhotyādi row"),
            (r43, "03.0024", "jan", "yAt", true, "another juhotyādi row"),
            (r42, "03.0025", "jA", "tas", true, "already lengthened"),
        ] {
            let mut p = jan_prakriya(number, anga, ending, ngit);
            assert!(!(rule.apply)(&mut p), "{why}");
            assert_eq!(p.terms[ANGA].text, anga, "{why}");
            assert!(p.log.is_empty(), "{why}");
        }
    }

    #[test]
    fn janasana_and_ye_vibhasa_decline_on_san_where_u_intervenes() {
        // Tanādi √san (`08.0002`) is in the key, but its follower is the
        // vikaraṇa `u`: not ṅit, not a jhal, not `y`. sanutaH, sanuyAt.
        for (id, ending) in [("6.4.42", "tas"), ("6.4.43", "yAt")] {
            let rule = rules().find(|r| r.id == id).unwrap();
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new("san"), Term::new("u"), Term::new(ending)]),
                ..Default::default()
            };
            p.ctx.dhatupatha = "08.0002";
            p.terms[ENDING].add(Tag::Ngit);
            assert!(!(rule.apply)(&mut p), "{id}");
            assert_eq!(p.terms[ANGA].text, "san", "{id}");
            assert!(p.log.is_empty(), "{id}");
        }
    }

    #[test]
    fn the_abhyasta_rules_would_eat_janas_a_so_janasana_runs_after_them() {
        // 6.4.22 asiddhavat atrābhāt, as placement: `jA` is an `A`-final
        // abhyasta aṅga, and before a ṅit `tas` 6.4.113 would make it *jajItaH
        // and 6.4.112 *jajtaH. Both sit above 6.4.42 and have already run on
        // `jan` by the time it writes the ā.
        for (id, want) in [("6.4.113", "jI"), ("6.4.112", "j")] {
            let rule = rules().find(|r| r.id == id).unwrap();
            let mut p = jan_prakriya("03.0025", "jA", "tas", true);
            assert!((rule.apply)(&mut p), "{id}");
            assert_eq!(p.terms[ANGA].text, want, "{id}");
        }
        let ids: Vec<&str> = rules().map(|r| r.id).collect();
        let at = |id: &str| ids.iter().position(|r| *r == id).unwrap();
        for id in ["6.4.42", "6.4.43"] {
            for abhyasta_rule in ["6.4.118", "6.4.113", "6.4.112"] {
                assert!(at(abhyasta_rule) < at(id), "{abhyasta_rule} before {id}");
            }
        }
    }
```

(The tuple rows are already in the shape `mise run fmt` produces.)

- [ ] **Step 2: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya -- janasana ye_vibhasa abhyasta_rules_would 2>&1 | tail -20`
Expected: all five FAIL, panicking on `unwrap()`: there is no rule `6.4.42` or `6.4.43`.

- [ ] **Step 3: Imports, the module doc, the key and the shared body**

Change the module doc's first line

```rust
//! Vowel gradation and vikaraṇa reshaping: 7.4.21 … 6.4.119, 6.4.118 … 6.4.116, 6.4.113, 6.4.112, 6.4.115.
```

to

```rust
//! Vowel gradation and vikaraṇa reshaping: 7.4.21 … 6.4.119, 6.4.118 … 6.4.116, 6.4.113, 6.4.98,
//! 6.4.100, 6.4.112, 6.4.115, 6.4.42, 6.4.43.
```

In the imports, add `use crate::prakriya::Prakriya;` above `use crate::rule::{Rule, RuleKind};`, and change `use crate::tinanta::sound::{guna_of, is_vowel};` to `use crate::tinanta::sound::{guna_of, is_jhal, is_vowel};`.

Immediately above `pub(crate) static GUNA: &[Rule] = &[`, insert:

```rust
/// The rows 6.4.42 and 6.4.43 name: √jan (`03.0025`) and tanādi's √san
/// (`08.0002`). √khan, the sūtra's third root, is not curated; a √khan row
/// extends this list.
///
/// √san is here although it never takes the `ā` in this corpus: its
/// follower is the vikaraṇa `u`, which is neither ṅit (1.2.4's second arm
/// excludes it) nor a jhal nor a `y`, so both rules decline on every √san
/// cell (sanutaH, sanuyAt) on the grammar rather than on a missing key, and
/// √san's goldens hold that. 6.4.107's u-lopa runs later, in `adesha.rs`.
const JANA_SANA: &[&str] = &["03.0025", "08.0002"];

/// 6.4.42 and 6.4.43's shared body: on a `JANA_SANA` row, before a kṅit
/// follower whose first sound satisfies `takes`, the aṅga's final `n`
/// becomes `ā` (`jan` → `jA`). The follower is read through
/// `following_sarvadhatuka`, as 6.4.100 reads it: under ślu SHAP is empty,
/// so it is the ending, whose `Tag::Ngit` already encodes 1.2.4, 7.1.35 and
/// 3.4.103.
fn janasanakhanam_a(p: &mut Prakriya, id: &str, name: &str, takes: fn(char) -> bool) -> bool {
    if !JANA_SANA.contains(&p.ctx.dhatupatha) {
        return false;
    }
    let Some(follower) = following_sarvadhatuka(p) else {
        return false;
    };
    if !follower.has(Tag::Ngit) {
        return false;
    }
    if !follower.text.chars().next().is_some_and(takes) {
        return false;
    }
    let Some(stem) = p.terms[ANGA].text.strip_suffix("an") else {
        return false;
    };
    let lengthened = format!("{stem}A");
    let before = p.snapshot();
    p.terms[ANGA].text = lengthened;
    p.record(id, name, before);
    true
}

```

- [ ] **Step 4: Add the two rules**

At the very end of `GUNA`, after 6.4.115's `Rule { … },` and before the closing `];` (the one directly above `#[cfg(test)]`), insert:

```rust
    // --- 6.4.42 / 6.4.43: √jan's `ā` -----------------------------------------
    //
    // AFTER THE Ā-OF-ABHYASTA BLOCK, at the end of this stage. Both rules
    // turn `jan` into `jA`, an `A`-final abhyasta aṅga, which is exactly what
    // that block reads: run first, 6.4.112 would elide the new ā (*jajtaH),
    // 6.4.113 would make it ī (*jajItaH), and a widened 6.4.118 would elide
    // it before `y` (*jajyAt). Textually this is 6.4.22 *asiddhavat
    // atrābhāt*: these and 6.4.112/6.4.113 are all ābhīya rules, so the ā is
    // asiddha to them, and running after them implements that.
    // `jajAtaH_trace_takes_the_a_without_the_abhyasta_rules` and
    // `jajAyAt_trace_forks_on_ye_vibhasa` in `panini`'s trace suite pin it.
    //
    // vidyut runs 6.4.42 BEFORE dvitva and reaches the same forms: its
    // abhyāsa copies `jaA` and 7.4.60 reduces it to `ja`, where here dvitva
    // copies `jan` and 7.4.60 reduces that to `ja`. vidyut also writes `jaA`
    // and credits 6.1.101 for the merge; this engine writes `jA` in one step.
    //
    // KEYED BY ROW NUMBER, `JANA_SANA`, and the two rules share
    // `janasanakhanam_a`. See both.

    // 6.4.42 janasanakhanāṁ sañjhaloḥ: √jan, √san and √khan take `ā` for
    // their final before a JHAL-INITIAL kṅit (the *san* arm needs a
    // desiderative, out of scope). ja + jan + tas → ja + jA + tas: jajAtaH,
    // jajAhi, jajAtAt, ajajAtAm. Before a pit ending it declines: jajanti,
    // jajantu.
    Rule {
        id: "6.4.42",
        name: "janasanaKanAM saYJaloH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| janasanakhanam_a(p, "6.4.42", "janasanaKanAM saYJaloH", is_jhal),
    },
    // 6.4.43 ye vibhāṣā: the same `ā`, optionally, before a Y-INITIAL kṅit.
    // In this corpus that is yāsuṭ, so all nine vidhiliṅ cells fork: jajanyAt
    // (declined) ~ jajAyAt. The engine's twelfth vikalpa.
    Rule {
        id: "6.4.43",
        name: "ye viBAzA",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &[],
        apply: |p| janasanakhanam_a(p, "6.4.43", "ye viBAzA", |c| c == 'y'),
    },
```

In 6.4.115's comment, change

```rust
    // engine's ninth vikalpa (6.4.117 and 6.4.116, slice 3c2, are the tenth
    // and eleventh).
```

to

```rust
    // engine's ninth vikalpa (6.4.117 and 6.4.116, slice 3c2, are the tenth
    // and eleventh; 6.4.43, slice 3f3, the twelfth).
```

- [ ] **Step 5: Pin the order and the vikalpa set**

In `derivation_tests.rs`:
- `tinanta_rule_order_is_pinned`: change `"6.4.115", "6.1.101",` to `"6.4.115", "6.4.42", "6.4.43", "6.1.101",`.
- `exactly_the_pinned_vikalpa_rules_are_optional`: change `"6.4.115", "6.4.107",` to `"6.4.115", "6.4.43", "6.4.107",`.

`mise run fmt` rewraps both arrays. The order pin now holds 129 ids, and the vikalpa pin twelve.

- [ ] **Step 6: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS (390 tests in the prototype).

- [ ] **Step 7: Run the full suite**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms)
Expected: PASS at 4608 cells. √jan's row is not curated yet, and on √san both rules decline. If a √san cell fails, the key is reaching it through the follower test: stop and report.

- [ ] **Step 8: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/guna.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(guna): 6.4.42 janasanakhanāṁ sañjhaloḥ and the vikalpa 6.4.43 ye vibhāṣā — √jan's ā, after the ā-of-abhyasta block"
```

---

## Task 5: The row and its paradigm goldens

This task turns the 36 new cells green.

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini/tests/paradigm/data/juhotyadi.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`
- Modify: `crates/panini/tests/trace/juhotyadi.rs` (the 7.3.87 and 8.3.24 corpus tests, and one comment)
- Modify: `AGENTS.md` (one test-name reference)

**Interfaces:**
- Consumes: Tasks 2–4.
- Produces: the `dhatus()` row `03.0025` (`jan`, `Gana::Juhotyadi`, `PadaAssignment::Parasmaipada`). Task 6 looks it up by number.

- [ ] **Step 1: Add the `Dhatu` row**

`DHATUS` is ordered by number. Insert this row immediately before the `03.0026` (`gA`) row, whose comment begins ``// 03.0026 `gA\` stutO (√gā).``. The upadeśa and artha are verbatim from `data/dhatupatha.tsv:1289` (`03.0025	jana~	janane`; `janI~\` is divādi's `04.0044`).

```rust
    Dhatu {
        // 03.0025 `jana~` janane (√jana). Parasmaipadī by 1.3.78. Before a
        // vowel-initial kṅit 6.4.98 elides its upadhā and 8.4.40 palatalizes
        // the `n` after the `j` (jajYati, ajajYuH); before a jhal-initial kṅit
        // 6.4.42 makes the `n` `ā` (jajAtaH, jajAhi), and before yāsuṭ 6.4.43
        // does so optionally (jajanyAt ~ jajAyAt). Slice 3f3, which closes the
        // gaṇa.
        dhatupatha: "03.0025",
        code: "jan",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "janane",
    },
```

Counts in the same file:
- `Dhatu`'s `pada` doc: `re-derives 101 of these 102` → `re-derives 102 of these 103`; `The test covers the 102 roots curated here` → `The test covers the 103 roots curated here`.
- `pada_from_upadesha`'s doc: `66 of the 102 curated roots` → `66 of the 103 curated roots`. `jana~` carries no `\`, so 66 and 45 stand.
- `curated_roots_have_expected_ganas_and_padas`: `assert_eq!(dhatus().len(), 102);` → `103`.

- [ ] **Step 2: Extend the gaṇa-row test**

Rename `juhotyadi_rows_are_the_twenty_five_curated_roots` to `juhotyadi_rows_are_the_twenty_six_curated_roots`. `grep -rn "juhotyadi_rows_are_the_twenty_five" crates AGENTS.md docs/ARCHITECTURE.md` finds the other references: a comment near `lib.rs:2009` and `AGENTS.md:820`. Update each. (`rudhadi_rows_are_the_twenty_five_curated_roots` is a different test; leave it alone.)

In its comment, replace

```rust
        // (03.0019), parasmaipadī by 1.3.78. The gaṇa is PARTIAL at 25 of its
        // 26 dhātupāṭha rows; slice 3f3 (√jan) closes it.
```

with

```rust
        // (03.0019), parasmaipadī by 1.3.78. Slice 3f3 adds √jan (03.0025),
        // parasmaipadī by 1.3.78, and closes the gaṇa at all 26 of its
        // dhātupāṭha rows.
```

In its expected vector, insert after the `("03.0024", "Dan", PadaAssignment::Parasmaipada),` line:

```rust
                ("03.0025", "jan", PadaAssignment::Parasmaipada),
```

- [ ] **Step 3: Run the data tests**

Run: `mise exec -- cargo test -p panini-data 2>&1 | tail -20`
Expected: PASS (21 tests), including `curated_pada_agrees_with_upadesha_markers`, `dhatupatha_numbers_resolve_upstream`, and the test's `code`-uniqueness tripwire (`jan` is unique).

- [ ] **Step 4: Add the `PARADIGM` blocks**

**Append** these to the end of `PARADIGM` in `crates/panini/tests/paradigm/data/juhotyadi.rs`, just before its closing `];` (the first `];` in the file). Blocks are ordered by slice, not by number. Index 0 of each multi-form cell is the declined derivation. `mise run fmt` rewraps.

```rust
    ("03.0025", "laT", Pada::Parasmaipada, ["jajanti", "jajAtaH", "jajYati", "jajaMsi", "jajATaH", "jajATa", "jajanmi", "jajanvaH", "jajanmaH"]),
    ("03.0025", "laN", Pada::Parasmaipada, ["ajajan", "ajajAtAm", "ajajYuH", "ajajan", "ajajAtam", "ajajAta", "ajajanam", "ajajanva", "ajajanma"]),
    ("03.0025", "loT", Pada::Parasmaipada, ["jajantu", "jajAtAm", "jajYatu", "jajAhi", "jajAtam", "jajAta", "jajanAni", "jajanAva", "jajanAma"]),
    ("03.0025", "viDiliN", Pada::Parasmaipada, ["jajanyAd", "jajanyAtAm", "jajanyuH", "jajanyAH", "jajanyAtam", "jajanyAta", "jajanyAm", "jajanyAva", "jajanyAma"]),
```

- [ ] **Step 5: Add the `ALTERNATES` rows**

**Append** these to the end of `ALTERNATES`, just before its closing `];` (the second `];` in the file). Each row is (root, lakāra, pada, 0-based cell index in the order P.E P.D P.B M.E M.D M.B U.E U.D U.B, form, key). The key is the vikalpa-listed ids on the form's branch, in log order.

```rust
    ("03.0025", "loT", Pada::Parasmaipada, 0, "jajAtAd", "7.1.35"),
    ("03.0025", "loT", Pada::Parasmaipada, 0, "jajAtAt", "7.1.35+8.4.56"),
    ("03.0025", "loT", Pada::Parasmaipada, 3, "jajAtAd", "7.1.35"),
    ("03.0025", "loT", Pada::Parasmaipada, 3, "jajAtAt", "7.1.35+8.4.56"),
    ("03.0025", "viDiliN", Pada::Parasmaipada, 0, "jajanyAt", "8.4.56"),
    ("03.0025", "viDiliN", Pada::Parasmaipada, 0, "jajAyAd", "6.4.43"),
    ("03.0025", "viDiliN", Pada::Parasmaipada, 0, "jajAyAt", "6.4.43+8.4.56"),
    ("03.0025", "viDiliN", Pada::Parasmaipada, 1, "jajAyAtAm", "6.4.43"),
    ("03.0025", "viDiliN", Pada::Parasmaipada, 2, "jajAyuH", "6.4.43"),
    ("03.0025", "viDiliN", Pada::Parasmaipada, 3, "jajAyAH", "6.4.43"),
    ("03.0025", "viDiliN", Pada::Parasmaipada, 4, "jajAyAtam", "6.4.43"),
    ("03.0025", "viDiliN", Pada::Parasmaipada, 5, "jajAyAta", "6.4.43"),
    ("03.0025", "viDiliN", Pada::Parasmaipada, 6, "jajAyAm", "6.4.43"),
    ("03.0025", "viDiliN", Pada::Parasmaipada, 7, "jajAyAva", "6.4.43"),
    ("03.0025", "viDiliN", Pada::Parasmaipada, 8, "jajAyAma", "6.4.43"),
```

That is 15 rows over 11 cells, so 36 + 15 = 51 forms, vidyut's count. Laṅ prathama eka (*ajajan*) does not fork: it ends in `n`, not a jaś.

- [ ] **Step 6: Update `paradigm/main.rs`**

`VIKALPA_RULES`: change

```rust
    "6.4.117", "6.4.116",
];
```

to

```rust
    "6.4.117", "6.4.116", "6.4.43",
];
```

and in its doc comment change

```rust
/// `.contains()`: it is NOT the pipeline order — the pin has 6.4.117,
/// 6.4.116 and 6.4.115 fourth to sixth, right after 7.3.86, not last as they sit here.
```

to

```rust
/// `.contains()`: it is NOT the pipeline order — the pin has 6.4.117,
/// 6.4.116, 6.4.115 and 6.4.43 fourth to seventh, right after 7.3.86, not
/// last as they sit here.
```

Above `every_alternate_names_the_vikalpa_rules_that_produced_it`: `otherwise 1091 bare strings` → `otherwise 1106 bare strings`.

In `derivation_set_shape_matches_the_audited_numbers`:
- `assert_eq!(total_cells, 4608, "512 root×lakāra blocks × 9 cells each");` → `assert_eq!(total_cells, 4644, "516 root×lakāra blocks × 9 cells each");`
- `ones` 3811 → `3836`; `twos` 596 → `604`; `threes` 155 → `157`; `fours` 18 → `19`. `fives`, `sixes` and `sevens` are unchanged.
- The `threes` message ends `and its two loṭ tātaṅ cells"`. Replace that ending with `and its two loṭ tātaṅ cells; and — new in slice 3f3 — √jan's two loṭ tātaṅ cells, \` / `by 7.1.35/8.4.56"`.
- The `fours` message ends `slice 3b — √bhī's vidhiliṅ prathama eka, forking on 6.4.115 alongside 8.4.56"`. Replace that ending with `slice 3b — √bhī's vidhiliṅ prathama eka, forking on 6.4.115 alongside 8.4.56; and — \` / `new in slice 3f3 — √jan's vidhiliṅ prathama eka, forking on 6.4.43 alongside 8.4.56"`.
- Keep the `\` line-continuation style in both messages.
- `ALTERNATES.len()` 1091 → `1106`.
- `key_count("8.4.56")` 163 → `164`; `key_count("7.1.35")` 154 → `156`; `key_count("7.1.35+8.4.56")` 154 → `156`.
- After the final `key_count("7.1.35+6.4.116+8.4.56")` assertion, add:

```rust
    assert_eq!(key_count("6.4.43"), 9, "6.4.43-only alternates");
    assert_eq!(key_count("6.4.43+8.4.56"), 1, "6.4.43+8.4.56 alternates");
```

Checks:
- Cells: 3836 + 604 + 157 + 19 + 10 + 17 + 1 = 4644.
- Forms: 3836 + 1208 + 471 + 76 + 50 + 102 + 7 = 5750 = 4644 + 1106.
- The key census stays exhaustive: 1091 + 1 + 2 + 2 + 9 + 1 = 1106.

Doc comments above `derivation_set_shape_matches_the_audited_numbers`:
- `4608 cells total (512 root×lakāra blocks × 9), of which 3811 hold exactly one form, 596 hold two, 155 hold three (` → `4644 cells total (516 root×lakāra blocks × 9), of which 3836 hold exactly one form, 604 hold two, 157 hold three (`.
- `and √bhas's, new in slice 3f2, each by` / `/// 7.1.35/8.4.56, plus √bhas's laṅ madhyama eka, by 8.2.74/8.4.56), eighteen hold four (` → `and √bhas's, new in slice 3f2, and √jan's, new in slice 3f3, each by` / `/// 7.1.35/8.4.56, plus √bhas's laṅ madhyama eka, by 8.2.74/8.4.56), nineteen hold four (`.
- `/// 3b — √bhī's vidhiliṅ prathama eka, forking on 6.4.115 alongside 8.4.56), and` → `/// 3b — √bhī's vidhiliṅ prathama eka, forking on 6.4.115 alongside 8.4.56; and — new in` / `/// slice 3f3 — √jan's vidhiliṅ prathama eka, forking on 6.4.43 alongside 8.4.56), and`.
- `itself has 1091 rows, keyed 163 \`8.4.56\`, 154 \`7.1.35\`, 154 \`7.1.35+8.4.56\`,` → `itself has 1106 rows, keyed 164 \`8.4.56\`, 156 \`7.1.35\`, 156 \`7.1.35+8.4.56\`,`.
- `` /// `7.1.35+6.4.116+8.4.56` — √kṛ (slice 8b) `` → `` /// `7.1.35+6.4.116+8.4.56`, 9 `6.4.43` and 1 `6.4.43+8.4.56` (slice 3f3's √jan) — √kṛ (slice 8b) ``.
- After the slice-3f2 paragraph (ending `— the second \`8.2.74\` key, after √hiṃs's. Eight new rows. The gaṇa is` / `/// PARTIAL at 25 of its 26 rows.`), add:

```rust
///
/// Slice 3f3 curates √jan (`03.0025`), parasmaipadī by 1.3.78, and closes the
/// gaṇa at all twenty-six of its rows. It adds 6.4.98 *gamahanajanakhanaghasāṁ
/// lopaḥ*, 6.4.42 *janasanakhanāṁ sañjhaloḥ* and the engine's twelfth vikalpa,
/// 6.4.43 *ye vibhāṣā*, and gives 8.4.40 its converse arm. 6.4.43 forks all
/// nine vidhiliṅ cells (`jajanyAm`/`jajAyAm`), the prathama eka four ways with
/// 8.4.56 — the first `6.4.43` keys — and √jan's two loṭ tātaṅ cells fork
/// three ways on 7.1.35/8.4.56. Fifteen new rows. The gaṇa is COMPLETE.
```

- [ ] **Step 7: Add the `check()` test**

Run `grep -rn '"jajYati"\|"ajajYuH"\|"jajAtaH"\|"jajAyAt"' crates/panini/tests/paradigm/data/` first. It must find only the lines this task added. Then, at the end of `paradigm/main.rs`, add:

```rust

/// Slice 3f3's row through `check`. `jan` is unique among the curated roots,
/// and none of these surfaces is a prior row's, so every analysis must name
/// √jan and carry the rule that shaped the form. `jajAyAt` is an alternate:
/// `check` reaches it through 6.4.43's branch.
#[test]
fn jan_analyses_its_reduplicated_forms() {
    let engine = Panini::new();
    for (form, sutra) in [
        ("jajYati", "6.4.98"),
        ("ajajYuH", "8.4.40"),
        ("jajAtaH", "6.4.42"),
        ("jajAyAt", "6.4.43"),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert!(!r.analyses.is_empty(), "{form}");
        for a in &r.analyses {
            assert_eq!(a.dhatu, "jan", "{form}");
            assert!(
                a.trace.iter().any(|s| s.sutra == sutra),
                "{form}: {:?}",
                a.trace
            );
        }
    }
}
```

- [ ] **Step 8: Extend the two corpus-wide allow-lists**

The engine credits 7.3.87 on √jan's pit vowel-initial cells (*ajajanam*, *jajanAni*, …) as a no-op on the `a` upadhā, as it does for √dhan and √bhas and as vidyut does. It credits 8.3.24 on *jajanti*, *jajaMsi* and *jajantu*, where the `n` meets a jhal before a pit ending; vidyut credits it on the same three. In `crates/panini/tests/trace/juhotyadi.rs`, replace

```rust
fn nabhyastasyaci_is_credited_only_on_the_3e_3f_and_3f2_rows() {
    // Corpus-wide: every branch of every curated root x lakāra x pada x cell
    // whose log carries 7.3.87 belongs to √ṇij, √vij, √viṣ (3e), √kit, √tur,
    // √dhiṣ, √dhan (3f) or √bhas (3f2) — a credited no-op on √dhan's and
    // √bhas's a-upadhā. A new rule that credits itself on prior rows' traces
    // (forms unchanged) fails here. 3f3 extends the allowed list with √jan.
    const ALLOWED: [&str; 8] = [
        "03.0012", "03.0013", "03.0014", "03.0019", "03.0021", "03.0022", "03.0023", "03.0024",
    ];
```

with

```rust
fn nabhyastasyaci_is_credited_only_on_the_3e_3f_3f2_and_3f3_rows() {
    // Corpus-wide: every branch of every curated root x lakāra x pada x cell
    // whose log carries 7.3.87 belongs to √ṇij, √vij, √viṣ (3e), √kit, √tur,
    // √dhiṣ, √dhan (3f), √bhas (3f2) or √jan (3f3) — a credited no-op on
    // √dhan's, √bhas's and √jan's a-upadhā. A new rule that credits itself on
    // prior rows' traces (forms unchanged) fails here.
    const ALLOWED: [&str; 9] = [
        "03.0012", "03.0013", "03.0014", "03.0019", "03.0021", "03.0022", "03.0023", "03.0024",
        "03.0025",
    ];
```

In `juhavani_trace_has_no_nabhyastasyaci`'s comment, change `` `nabhyastasyaci_is_credited_only_on_the_3e_3f_and_3f2_rows`. `` to `` `nabhyastasyaci_is_credited_only_on_the_3e_3f_3f2_and_3f3_rows`. ``.

Replace

```rust
fn nas_capadantasya_is_credited_only_on_rudhadi_and_dhan() {
    // 8.3.24 admits juhotyādi since slice 3f; only √dhan has an `n` before
    // a jhal there.
    let hits = credited("8.3.24");
    for (number, gana) in &hits {
        assert!(
            *gana == Gana::Rudhadi || *number == "03.0024",
            "8.3.24 credited on {number}"
        );
    }
```

with

```rust
fn nas_capadantasya_is_credited_only_on_rudhadi_dhan_and_jan() {
    // 8.3.24 admits juhotyādi since slice 3f; only √dhan and √jan (3f3) have
    // an `n` before a jhal there — √jan only before a pit ending (jajanti,
    // jajaMsi, jajantu), since 6.4.42 takes the `n` before a kṅit one.
    let hits = credited("8.3.24");
    for (number, gana) in &hits {
        assert!(
            *gana == Gana::Rudhadi || *number == "03.0024" || *number == "03.0025",
            "8.3.24 credited on {number}"
        );
    }
```

Its trailing `√dhan no longer witnesses 8.3.24` assertion stays. `grep -rn "nabhyastasyaci_is_credited_only_on_the_3e_3f_and_3f2\|nas_capadantasya_is_credited_only_on_rudhadi_and_dhan" crates AGENTS.md docs/ARCHITECTURE.md` must then find nothing.

- [ ] **Step 9: Run the full suite**

Run: `mise run test 2>&1 | tail -40` (foreground, timeout 600000 ms)
Expected: PASS at 4644 cells, with all 4608 priors unchanged.

If a 3f3 cell fails, read it against the spec before touching anything, using superpowers:systematic-debugging. **Do not edit a golden to match the engine.** Likely causes by symptom:
- `jajanati` / `ajajanuH` (upadhā kept before a vowel-initial ṅit): 6.4.98 declined. Check its row key and the follower's `Tag::Ngit`.
- `jajnati` (`n` not palatalized): Task 2's converse arm did not fire.
- `jajantaH` / `jajaMhi` (`n` kept before a jhal-initial ṅit): 6.4.42 declined.
- `jajtaH` / `jajItaH`: 6.4.42 runs above 6.4.112/6.4.113. Check the placement.
- vidhiliṅ with one form: 6.4.43's `vikalpa` flag, or its `y` test.
- A key mismatch in `every_alternate_names_the_vikalpa_rules_that_produced_it`: read the branch's actual log. Only correct a key if the form is right and the log really differs, and say so in the commit message.

- [ ] **Step 10: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-data/src/lib.rs crates/panini/tests/paradigm/data/juhotyadi.rs crates/panini/tests/paradigm/main.rs crates/panini/tests/trace/juhotyadi.rs AGENTS.md
git commit -m "feat(data): √jan (03.0025) — juhotyādi complete at twenty-six of twenty-six rows

4608 → 4644 cells, 5699 → 5750 forms, 1091 → 1106 ALTERNATES, 102 → 103 roots."
```

---

## Task 6: The trace pins and the fires-only tests

**Files:**
- Modify: `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes: `crate::helpers::{at, cell_trace}`. `cell_trace(number, lakara, pada, purusha, vacana) -> (String, Vec<String>)` returns branch 0's (the declined derivation's) text and trace. `at(&[String], &str) -> usize` panics if the sūtra is absent. Already in this file: `branch_trace(number, lakara, purusha, vacana, form) -> Vec<String>` (parasmaipada, the branch deriving `form`) and `credited(sutra) -> Vec<(&'static str, Gana)>` (one entry per non-blocked branch whose log carries `sutra`).

- [ ] **Step 1: Add the pins and the corpus-wide tests**

At the end of the file, add:

```rust

#[test]
#[allow(non_snake_case)]
fn jajYati_trace_is_upadha_lopa_then_shcutva() {
    // jan P laT P.B. 6.4.98 elides the upadhā before the ṅit `ati` (7.1.4's),
    // and 8.4.40's converse arm takes the `n` after the `j` to `Y`.
    let (text, t) = cell_trace(
        "03.0025",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "jajYati", "got {t:?}");
    assert!(at(&t, "7.1.4") < at(&t, "6.4.98"), "got {t:?}");
    assert!(at(&t, "6.4.98") < at(&t, "8.4.40"), "got {t:?}");
    assert!(!t.contains(&"6.4.42".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn ajajYuH_trace_is_at_then_upadha_lopa_then_shcutva() {
    // jan P laN P.B. The aṭ, then 6.4.98 before `us`, then 8.4.40.
    let (text, t) = cell_trace(
        "03.0025",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "ajajYuH", "got {t:?}");
    assert!(at(&t, "6.4.71") < at(&t, "6.4.98"), "got {t:?}");
    assert!(at(&t, "6.4.98") < at(&t, "8.4.40"), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn jajAtaH_trace_takes_the_a_without_the_abhyasta_rules() {
    // jan P laT P.D. 6.4.42 writes `jA` after 6.4.112 and 6.4.113 have run
    // on `jan` and declined (6.4.22), so neither elides nor raises the ā.
    let (text, t) = cell_trace(
        "03.0025",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "jajAtaH", "got {t:?}");
    assert!(t.contains(&"6.4.42".to_string()), "got {t:?}");
    for absent in ["6.4.112", "6.4.113", "6.4.98", "8.3.24"] {
        assert!(!t.contains(&absent.to_string()), "{absent}: got {t:?}");
    }
}

#[test]
#[allow(non_snake_case)]
fn jajAhi_trace_takes_the_a_and_keeps_hi() {
    // jan P loT M.E. `hi` is jhal-initial and ṅit: 6.4.42. The stem `jA` is
    // not jhal-final, so 6.4.101 hu-jhalbhyo her dhiḥ declines.
    let (text, t) = cell_trace(
        "03.0025",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "jajAhi", "got {t:?}");
    assert!(t.contains(&"6.4.42".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.101".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn jajAyAt_trace_forks_on_ye_vibhasa() {
    // jan P viDiliN P.E. 6.4.43's branch takes the ā before yāsuṭ, and no
    // ā-of-abhyasta rule touches it; the declined branch keeps the `n`.
    let t = branch_trace(
        "03.0025",
        Lakara::VidhiLin,
        Purusha::Prathama,
        Vacana::Eka,
        "jajAyAt",
    );
    assert!(at(&t, "7.2.79") < at(&t, "6.4.43"), "got {t:?}");
    for absent in ["6.4.112", "6.4.118", "6.4.42"] {
        assert!(!t.contains(&absent.to_string()), "{absent}: got {t:?}");
    }
    let t = branch_trace(
        "03.0025",
        Lakara::VidhiLin,
        Purusha::Prathama,
        Vacana::Eka,
        "jajanyAt",
    );
    assert!(!t.contains(&"6.4.43".to_string()), "got {t:?}");
}

#[test]
fn jajanti_trace_takes_none_of_the_jan_rules_before_a_pit() {
    // jan P laT P.E. tip is pit: no 6.4.98, 6.4.42 or 6.4.43, and the `n`
    // before `t` goes through 8.3.24 and back by 8.4.58.
    let (text, t) = cell_trace(
        "03.0025",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "jajanti", "got {t:?}");
    for absent in ["6.4.98", "6.4.42", "6.4.43"] {
        assert!(!t.contains(&absent.to_string()), "{absent}: got {t:?}");
    }
    assert!(at(&t, "8.3.24") < at(&t, "8.4.58"), "got {t:?}");
}

#[test]
fn gamahana_janasana_and_ye_vibhasa_are_credited_only_on_jan() {
    // All three are keyed by row number; 6.4.42 and 6.4.43's key holds √san
    // too, which must never take them (its follower is the vikaraṇa `u`).
    for sutra in ["6.4.98", "6.4.42", "6.4.43"] {
        let hits = credited(sutra);
        assert!(!hits.is_empty(), "√jan no longer witnesses {sutra}");
        for (number, _) in &hits {
            assert_eq!(*number, "03.0025", "{sutra} credited on {number}");
        }
    }
}

#[test]
fn shcutva_off_jan_is_credited_exactly_as_before_3f3() {
    // 8.4.40 has a converse arm (a ścu, then a stu) since slice 3f3. A trace
    // step carries no direction, so "the converse arm fires only on √jan" is
    // held as the credits off √jan: exactly √chid's and √chṛd's tuk (the
    // forward arm), 54 branches, measured on `main` before the widening. A
    // new firing on a prior row, in either direction, changes the count.
    let hits = credited("8.4.40");
    assert!(
        hits.iter().any(|(n, _)| *n == "03.0025"),
        "√jan no longer witnesses 8.4.40"
    );
    let off_jan: Vec<_> = hits.iter().filter(|(n, _)| *n != "03.0025").collect();
    for (number, _) in &off_jan {
        assert!(
            *number == "07.0003" || *number == "07.0008",
            "8.4.40 credited on {number}"
        );
    }
    assert_eq!(off_jan.len(), 54);
}
```

`54` was measured on `main` at `436a507` from a dump of every curated cell's live branches (27 on `07.0003`, 27 on `07.0008`), the same count `credited()` takes. Task 7 Step 2 re-measures it.

- [ ] **Step 2: Run the suite**

Run: `mise run test 2>&1 | tail -20` (foreground, timeout 600000 ms)
Expected: PASS (184 trace tests in the prototype).
- If a pin fails on ORDER, report the engine's order; do not "fix" it toward vidyut's.
- If a pin fails on an id this plan assumed, read the actual trace. Correct the pin to the engine's real credit only when the form is right and the spec does not name that id, and say so in the commit message.
- If a fires-only test names a root outside the allowed set, or the 8.4.40 count is not 54, stop and report it: that is the new code over-firing on a prior row.

- [ ] **Step 3: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini/tests/trace/juhotyadi.rs
git commit -m "test(trace): 3f3 pins — jajYati, ajajYuH, jajAtaH, jajAhi, jajAyAt, jajanti; 6.4.98/6.4.42/6.4.43 fire only on √jan, 8.4.40 off √jan pinned at 54"
```

---

## Task 7: Audit, prior-trace diff, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (audit prose), `crates/panini-prakriya/src/tinanta/{guna,anga}.rs` (comments)

**Interfaces:**
- Consumes: the finished engine and goldens (Tasks 2–6). Produces no symbols.

- [ ] **Step 1: Update the audit harness's asserted totals**

In `tools/audit/panini_full_audit.rs`:
- `for each of the 102 curated roots` → `103`
- `Corpus invariants, asserted: 102 roots, 4608 cells, 5699 forms.` → `103 roots, 4644 cells, 5750 forms.`
- `` (`derivation_set_shape_matches_the_audited_numbers`): 512 root×pada×lakāra `` / `` //! blocks × 9 cells, plus 1091 `ALTERNATES` rows. `` → `516 …` / `plus 1106`
- `the full 4608-cell table` → `4644-cell`
- `assert_eq!(roots_seen.len(), 102, "curated roots");` → `103`
- `assert_eq!(n_cells, 4608, "cells: 512 root×pada×lakāra blocks × 9");` → `4644`, `"cells: 516 root×pada×lakāra blocks × 9"`
- `assert_eq!(n_forms, 5699, "forms: 4608 cells + 1091 ALTERNATES rows");` → `5750`, `"forms: 4644 cells + 1106 ALTERNATES rows"`

The both-pada clause (`twenty-six` / `twenty-five ubhayapadī`) does not change, because √jan is parasmaipadī.

In `tools/audit/README.md`, `(102 roots, 4608 cells, 5699 forms)` → `(103 roots, 4644 cells, 5750 forms)`.

- [ ] **Step 2: Repoint vidyut's dev-deps at THIS worktree, run the audit and the prior-trace diff**

`/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes absolute dev-dep paths to `/workspace/crates`, the `main` checkout, which does not have this slice. Auditing without repointing checks the pre-slice engine and passes vacuously.

First dump every prior cell's traces from `main` **before** repointing. The throwaway example `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_3f3.rs` exists from the spec's probe. If it is missing, recreate it:

```rust
//! THROWAWAY: slice 3f3 — dump every curated cell's live-branch credited-rule log.
use panini::Panini;
use panini_data::{Lakara as L, Purusha as P, Vacana as V};
fn main() {
    let panini = Panini::new();
    for d in panini_data::dhatus() {
        if d.dhatupatha == "03.0025" { continue; }
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

It skips `03.0025`, so the branch dump holds exactly the prior cells.

```bash
WT="$(git rev-parse --show-toplevel)"
DUMP="$(mktemp -d)"
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # must point at /workspace/crates
(cd /tmp/vidyut-full/vidyut-prakriya && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_3f3 > "$DUMP/main.txt")
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # must point at $WT/crates
(cd /tmp/vidyut-full/vidyut-prakriya && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_3f3 > "$DUMP/branch.txt")
wc -l "$DUMP/main.txt"                                          # 5699 at 436a507
diff "$DUMP/main.txt" "$DUMP/branch.txt" && echo PRIORS-IDENTICAL
grep -cE ' 8\.4\.40( |$)' "$DUMP/main.txt"                       # 54, the Task 6 pin
```

Expected: `PRIORS-IDENTICAL`, and `54`. If the diff shows any line, stop and report it: that is a prior cell's trace changed by the slice.

Then the audit. Copy the committed harness; never rewrite it.

```bash
cp tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -15)
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -8)
```

- Expected from the honest run: `AUDIT PASSED: 4644 cells, 5750 forms, zero differences.`
- Expected from the `entry` control: `AUDIT FAILED: 36 differing cells.` (the √bhū cells).

`mise exec rust@1.99.0 --` runs cargo on this branch's toolchain from vidyut's directory. Do not use `mise -C`: it changes cargo's working directory too, and cargo then looks for the example in the wrong package.

If the honest run shows differences, stop and report, and edit nothing. After all runs, restore the dev-deps so later probes from `main` work:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"/workspace/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"/workspace/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
```

- [ ] **Step 3: Record the audit**

In `tools/audit/README.md`, immediately under `## Last recorded result` and its blank line, add a new entry above the 3f2 one, with `<DATE>` = `date -u +%F`:

```markdown
<DATE>, juhotyādi 3f3 slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 4644
cells / 5750 forms / 103 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole juhotyādi 3f3 slice: √jan (`03.0025`), the
gaṇa's last row, with three new rules — 6.4.98 *gamahanajanakhanaghasāṁ lopaḥ*
(*jajYati*), 6.4.42 *janasanakhanāṁ sañjhaloḥ* (*jajAtaH*) and the vikalpa
6.4.43 *ye vibhāṣā* (*jajanyAt* ~ *jajAyAt*) — and 8.4.40 *stoḥ ścunā ścuḥ*'s
converse arm (`jn` → `jY`), guarded by 8.4.44 *śāt*. A main-vs-branch dump of
every prior cell's traces was byte-identical.

Totals: 103 = 102 + 1; 4644 = 4608 + 36 (4 root×pada×lakāra blocks × 9); 5750
= 5699 + 36 + 15 new `ALTERNATES` rows (1091 → 1106), measured via the
harness's corpus block, not assumed.

```

In `crates/panini/tests/paradigm/main.rs`'s audit-chain doc comment, replace

```rust
/// cells), and juhotyādi 3f2's re-ran it at the same commit over all 4608
/// cells / 5699 forms / 102 roots with zero differences, its `entry` negative
/// control verified failing (36 √bhū cells).
```

with

```rust
/// cells), and juhotyādi 3f2's re-ran it at the same commit over all 4608
/// cells / 5699 forms / 102 roots with zero differences, its `entry` negative
/// control verified failing (36 √bhū cells), and juhotyādi 3f3's re-ran it at
/// the same commit over all 4644 cells / 5750 forms / 103 roots with zero
/// differences, its `entry` negative control verified failing (36 √bhū
/// cells).
```

- [ ] **Step 4: README.md**

Each `old` → `new` below occurs exactly once.
- `nine gaṇas covered, eight of them fully —` → `nine gaṇas covered, all nine fully —`
- `slot), **partial** at 25 of its 26 dhātupāṭha rows, √hu` → `slot), **complete** at all 26 of its dhātupāṭha rows, √hu`
- Replace `(*ababhat*, *ababhaḥ*) and 8.4.55 *khari ca* reading the whole word. rudhādi is` with:

```markdown
(*ababhat*, *ababhaḥ*) and 8.4.55 *khari ca* reading the whole word.
Slice 3f3 added √jan (`03.0025`, *jajanti*), parasmaipadī, the gaṇa's last
row, behind three new sūtras — 6.4.98 *gamahanajanakhanaghasāṁ lopaḥ*
(*jajñati*), 6.4.42 *janasanakhanāṁ sañjhaloḥ* (*jajātaḥ*) and the engine's
twelfth vikalpa, 6.4.43 *ye vibhāṣā* (*jajanyāt* ~ *jajāyāt*) — with 8.4.40
*stoḥ ścunā ścuḥ* gaining its converse arm, a stu after a ścu, guarded by the
8.4.44 *śāt* exemption. rudhādi is
```

- `curated 102-root set` → `curated 103-root set`
- `797 of the 4608 cells hold more than one form: 596` / `hold two, 155 hold three (` → `808 of the 4644 cells hold more than one form: 604` / `hold two, 157 hold three (`
- `and √dhan's, and — new in slice 3f2 — √bhas's, each by 7.1.35/8.4.56, √bhas's` / `laṅ madhyama eka by 8.2.74/8.4.56),` / `eighteen hold four` → `and √dhan's, and — new in slice 3f2 — √bhas's, and — new in slice 3f3 —` / `√jan's, each by 7.1.35/8.4.56, √bhas's laṅ madhyama eka by 8.2.74/8.4.56),` / `nineteen hold four`
- `forking on 6.4.115 alongside 8.4.56), ten hold` → `forking on 6.4.115 alongside 8.4.56, and — new in slice 3f3 — √jan's` / `vidhiliṅ prathama eka, forking on 6.4.43 alongside 8.4.56), ten hold`

Check: 604 + 157 + 19 + 10 + 17 + 1 = 808. The both-pada and pada-ambiguity paragraphs do not change.

- [ ] **Step 5: docs/ARCHITECTURE.md**

Each `old` → `new` occurs exactly once.
- Stage table, `guna.rs` row: `6.4.113, 6.4.100, 6.4.112, 6.4.115 — vowel` → `6.4.113, 6.4.98, 6.4.100, 6.4.112, 6.4.115, 6.4.42, 6.4.43 — vowel`
- `pins all 126 ids verbatim` → `pins all 129 ids verbatim`
- `*jhalo jhali* (8.2.73, 8.2.74 and 8.4.55 widened) — 126 total).` → `*jhalo jhali* (8.2.73, 8.2.74 and 8.4.55 widened) — 126 total — then` / `juhotyādi 3f3's three: 6.4.98 *gamahanajanakhanaghasāṁ lopaḥ*, 6.4.42` / `*janasanakhanāṁ sañjhaloḥ* and 6.4.43 *ye vibhāṣā* (8.4.40 widened) — 129` / `total).`
- `Nine gaṇas are covered, eight of them fully:` → `Nine gaṇas are covered, all nine fully:`
- `and juhotyādi (3), **partial** at 25 of its 26 rows (√hu,` → `and juhotyādi (3), **complete** at all 26 of its rows (√hu,`
- `√bhas; slice 3f2). gaṇa` → `√bhas; slice 3f2; √jan; slice 3f3). gaṇa`
- `8.2.40's *adhaḥ*, 6.4.100, 6.4.116–6.4.118)` → `8.2.40's *adhaḥ*, 6.4.98, 6.4.100, 6.4.42, 6.4.43, 6.4.116–6.4.118)`
- `Eleven rules are optional, in pipeline order:` → `Twelve rules are optional, in pipeline order:`
- `**6.4.115** *bhiyo'nyatarasyām*, **6.4.107** *lopaś` → `**6.4.115** *bhiyo'nyatarasyām*, **6.4.43** *ye vibhāṣā*, **6.4.107** *lopaś`
- `Three of the` / `eleven arrived` → `Three of the` / `twelve arrived`
- ``by `Rule.bars`. See`` / `` `exactly_the_pinned_vikalpa_rules_are_optional` `` → ``by `Rule.bars`. Slice 3f3 added 6.4.43, keyed to √jan's row (and √san's, which`` / `never reaches it), forking all nine of its vidhiliṅ cells. See` / `` `exactly_the_pinned_vikalpa_rules_are_optional` ``
- The 7.1.35 / 8.4.56 paragraph:
  - `forking 154 cells (loṭ` / `prathama and madhyama eka across the 77 roots` → `forking 156 cells (loṭ` / `prathama and madhyama eka across the 78 roots`
  - `√kit, √tur, √dhiṣ, √dhan and √bhas (all parasmaipada-only)` → `√kit, √tur, √dhiṣ, √dhan, √bhas and √jan (all parasmaipada-only)`
  - `77 + 25 = the 102 curated roots` → `78 + 25 = the 103 curated roots`
  - `forking 163 cells outright` → `forking 164 cells outright`
  - `those same 77 parasmaipada columns (140 of them` → `those same 78 parasmaipada columns (141 of them`
  - `` its laṅ `abaBad` coming from 8.2.73), `` → `` its laṅ `abaBad` coming from 8.2.73; 3f3's √jan contributes only its vidhiliṅ cell, its laṅ ending in `n`, and its 6.4.43 branch's `jajAyAd ~ jajAyAt` keys on 6.4.43 as well), ``
  - `forking a further 154 (the same` → `forking a further 156 (the same`

  Check: 141 + 22 + 1 = 164, and 154 + 2 = 156. What must be right: 1 new parasmaipada column, 4 new `7.1.35`-keyed rows (2 `7.1.35`, 2 `7.1.35+8.4.56`), and 1 new plain `8.4.56` row.

- [ ] **Step 6: AGENTS.md**

Each `old` → `new` occurs exactly once.
- `4608 cells, nine gaṇas — eight complete,` → `4644 cells, nine gaṇas, all complete —`
- `and now` / `at 25 of its 26 after slice 3f2 curated √bhas —` → `at 25` / `after slice 3f2 curated √bhas, and closing at 26 of 26 in slice 3f3 with √jan —`
- `a second (596 cells), a third (155 cells), a fourth` / `(eighteen` → `a second (604 cells), a third (157 cells), a fourth` / `(nineteen`
- `— new in slice 3b — √bhī's vidhiliṅ prathama` / `eka) and` → `— new in slice 3b — √bhī's vidhiliṅ prathama` / `eka, and — new in slice 3f3 — √jan's) and`
- `(1091 rows in all, so 4608 + 1091 = 5699 forms total)` → `(1106 rows in all, so 4644 + 1106 = 5750 forms total)`
- The audit record: `entry, 4608 cells / 5699 forms / 102 roots).` → `` entry, 4608 cells / 5699 forms / 102 roots), and that by juhotyādi 3f3's `` / `` (`tools/audit/README.md`'s <DATE> entry, 4644 cells / 5750 forms / 103 `` / `roots).`
- `4608 goldens` / `would move today` → `4644 goldens` / `would move today`
- The stale-comment ledger: the 3f2 sentence ends `` `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit). A third, ``. Before ` A third,` insert: `` Juhotyādi 3f3 touched neither comment either; the corpus stands at 4644 cells as of 3f3 (`guna.rs:2433`'s claim anchored at `guna.rs:<G>`, `controller.rs:206`'s at `controller.rs:<C>`; both lines measured by grep at this commit). `` Measure `<G>` with `grep -n "1872 goldens move" crates/panini-prakriya/src/tinanta/guna.rs` (2565 in the prototype) and `<C>` with `grep -n "only 8 cells fire" crates/panini-prakriya/src/controller.rs` (206). Tasks 3–4 added lines to `guna.rs`, so `<G>` has moved. Measure; never compute.
- `**Eleven rules are optional today, in pipeline order:` / `7.1.35, 3.4.111, 7.3.86, 6.4.117, 6.4.116, 6.4.115, 6.4.107,` → `**Twelve rules are optional today, in pipeline order:` / `7.1.35, 3.4.111, 7.3.86, 6.4.117, 6.4.116, 6.4.115, 6.4.43, 6.4.107,`
- `Slice 3c2's 6.4.117 and 6.4.116 are the` / `tenth and eleventh.**` → `Slice 3c2's 6.4.117 and 6.4.116 are the` / `tenth and eleventh, and slice 3f3's 6.4.43, keyed to √jan's row and forking` / `its nine vidhiliṅ cells, the twelfth.**`
- The root-keyed-guard paragraph: after `so a curated √ghas extends its key rather than sharing a text test.` insert `` 6.4.98 (`03.0025`) and 6.4.42 / 6.4.43 (`JANA_SANA`: `03.0025`, `08.0002`; slice 3f3) do too; the latter key both curated roots their sūtra names, and √san declines on its follower, the vikaraṇa `u`, not on a missing key. ``

- [ ] **Step 7: The stale engine comments**

- `crates/panini-prakriya/src/tinanta/anga.rs`, 6.4.71's comment: `// can falsify (3e's, 3f's and 3f2's curated roots, and 3f3's √jan, are` / `// all consonant-initial), so the ANGA read stays.` → `// can falsify (3e's, 3f's, 3f2's and 3f3's curated roots are all` / `// consonant-initial), so the ANGA read stays.`
- `crates/panini-prakriya/src/tinanta/guna.rs`, 6.1.78's comment: `// 102-root × 4-lakāra grammar, ANGA can never end` → `// 103-root × 4-lakāra grammar, ANGA can never end`. The claim still holds: no √jan form ends its aṅga in E/O.
- `guna.rs`'s 7.3.87 comment (`a-upadhā √dhan (slice 3f), √bhas (slice 3f2) and √jan (slice 3f3)`) is already a record of fact; leave it.

- [ ] **Step 8: Sweep for anything left stale**

```bash
grep -rn "4608\|5699\|\b1091\b\|102 roots\|102-root\|of these 102\|of the 102\|\b512\b\|797 of\|25 of its 26\|twenty-five of\|forking 154\|forking 163\|140 of them\|77 + 25\|\b596\b\|155 hold\|(155 cells)\|77 parasmaipada\|77 roots with\|126 ids\|twenty_five_curated\|3e_3f_and_3f2\|rudhadi_and_dhan()\|Eleven rules\|eight of them\|eight complete" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.md --include=*.rs
grep -rn "3f3" crates tools README.md AGENTS.md docs/ARCHITECTURE.md
grep -rn "ONE DIRECTION\|one direction\|stu before ścu, never\|eleventh vikalpa\|ninth, tenth and eleventh\|PARTIAL" crates/panini-prakriya/src crates/panini/tests docs/ARCHITECTURE.md AGENTS.md README.md
grep -rn "\bjan\b\|\bjn\b\|\bjA\b\|\bjY\b\|6\.4\.98\|6\.4\.4[23]\|8\.4\.40\|8\.4\.44" crates/panini-prakriya/src --include=*.rs | grep "//"
```

Expected residue:
- `AGENTS.md:37`'s floor paragraph (`measured at 4608 cells`), which Task 8 rewrites.
- AGENTS.md's dated mutation record, `tools/audit/README.md`'s older entries, the audit chain in `paradigm/main.rs`, and the 3f2 paragraph (`twenty-five of its twenty-six`, `PARTIAL at 25 of its 26`). That is history; never rewrite it.
- `rudhadi_rows_are_the_twenty_five_curated_roots` and README's `complete at all twenty-five of its` (rudhādi).
- "3f3" wherever it records what 3f3 *did*.

For every root-shape and rule-id hit, check the comment is still true with √jan curated and 8.4.40 two-directional.

- [ ] **Step 9: Run the full suite and commit**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms). Expected: PASS at 4644 cells.

```bash
mise run fmt && mise run lint
git add -A
git commit -m "docs: 3f3's counts, the audit record, and the sweep

4644 cells / 5750 forms / 103 roots / 1106 ALTERNATES across README,
ARCHITECTURE, AGENTS, paradigm/main.rs and tools/audit; juhotyādi complete.
Audit at zero divergence against 8da2f90b; prior traces byte-identical to main."
```

---

## Task 8: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and the current-record paragraph); `mise.toml` only if the cap moves

Follow AGENTS.md's cargo-mutants protocol. Hazards from this repo's record:
- **Measure, never scale.**
- **Every invocation rotates `mutants.out`**, so always pass `-o`.
- **The mise shim fails in background shells.** Use the real binary: `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`.
- **`pgrep -f` matches its own shell.** Wait on `pgrep -x cargo-mutants`.
- **This slice edits `tripadi.rs` above both tripadi entries of the non-caught set**, so their line numbers move. Find them by `--list` and by their shape, never by the recorded line.

- [ ] **Step 1: Measure the floor**

With nothing else running, run this twice: `time mise run test 2>&1 | tail -3` (foreground). Record both wall clocks and `cat /proc/loadavg`. The 4608-cell floor was 7.848s / 7.864s at load about 16–17.

- [ ] **Step 2: Locate and probe the two uncaught equivalents at `-j 4`**

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /="
```

On the prototype this listed `adesha.rs:589:30`, `tripadi.rs:1289:38` and `tripadi.rs:1602:23`. `adesha.rs` is untouched, so its entry stays at 589. The `tripadi.rs` `- with /` entry (was `1270:38`, inside 8.3.13's `apply`, the `w[i - 1]` of `let Some(i) = (1..w.len()).find(…)`) and the permanent `-= with /=` ṇatva hang (was `1583:23`) moved down 19 lines under Task 2. If `--list` shows several `- with /` candidates at column 38, pick the one inside 8.3.13. Write the positions as `<T1>` (the equivalent) and `<T2>` (the hang), then run in the foreground with timeout 600000 ms:

```bash
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 110 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:589:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" 2>&1 | tail -10
```

Both must be MISSED, not TIMEOUT. Read each test-phase duration from `$SCRATCH/mutants.out/outcomes.json`.

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/juhotyadi-3f3"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout 110 -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"
```

The last campaign took about 20 minutes. Run nothing CPU-heavy meanwhile. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

- [ ] **Step 4: Read the outcomes**

When `pgrep -x cargo-mutants` returns nothing:

```bash
date -u +"%F %T UTC" > "$OUT/finished"
cp "$OUT/mutants.out/outcomes.json" "$OUT/outcomes.durable.json"
tail -5 "$OUT/campaign.log"
cat "$OUT/mutants.out/missed.txt" "$OUT/mutants.out/timeout.txt"
```

Expected (exit code 3 is normal when a timeout is present):
- `missed.txt` holds exactly `adesha.rs:589:30: replace + with *` and `tripadi.rs:<T1>:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:<T2>:23: replace -= with /=`.
- panini-analyze: 0 missed, 0 timeout.

A scoped probe while this plan was written already ran every mutant in the new code: `janasanakhanam_a`'s three guards and both return-value replacements, 6.4.98's three guards, 6.4.43's `== 'y'`, and 8.4.40's new arm (`&&`, `!= 'S'`, both `i + 1`). All 17 were caught.

If not:
- Any **other timeout** is a suspect survivor. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant in 6.4.98, `janasanakhanam_a`, 6.4.42/6.4.43 or 8.4.40 means a Task 2–4 test does not separate it. Strengthen the named test, commit, and re-run only those mutants with `--re` and a fresh `-o`.

- [ ] **Step 5: Margins**

Inspect one record to see how `duration` is stored:

```bash
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["outcomes"][1])' "$OUT/mutants.out/outcomes.json"
```

Compute the two equivalents' test phases under campaign load, and the caught min/median/p90/max. The margin is 110 ÷ the longest equivalent phase.
- If the margin is ≥ 5, the cap stays 110.
- Otherwise the new cap is 6 × that phase, rounded up to the next 10 s. Change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together. A higher cap can only turn timeouts into outcomes, and the only timeout is the permanent hang, so the campaign's outcomes stand without a re-run.

- [ ] **Step 6: Record it in AGENTS.md**

- Rewrite `**The floor behind the 110s cap, measured at 4608 cells on Rust 1.99.0,` / `2026-10-01.**` with Step 1's and Step 2's numbers at 4644 cells, the new `tripadi.rs` positions, and the cap Step 5 chose. Also update the `(8.6-18.2s as measured in 3f2: …)` parenthesis a few lines above it with this slice's isolated and campaign-load phases.
- Replace the `**Current record (juhotyādi 3f2, 2026-10-01).**` paragraph with `**Current record (juhotyādi 3f3, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**, with the moved `tripadi.rs` positions;
  - the non-caught set diffed against 3f2's (the clean result is identical up to the two moved `tripadi.rs` lines);
  - the campaign-load phases and margins;
  - that no 6.4.98, `janasanakhanam_a` / 6.4.42 / 6.4.43 or 8.4.40 mutant is missed, timed out or unviable-without-reason, with each one's `file:line` range and mutant count;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`.

  End it with a pointer to the record it replaces: run `git rev-parse --short HEAD` before committing and write `The juhotyādi 3f2 record it replaces: \`git show <that hash>:AGENTS.md\`.`
- Update every other mention of the `tripadi.rs:1270:38` and `tripadi.rs:1583:23` positions in AGENTS.md and `mise.toml`'s comments (`grep -n "1270:38\|1583:23" AGENTS.md mise.toml`) outside dated history.

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md mise.toml
git commit -m "chore: 3f3 mutation gate — floor and uncaught run re-measured at 4644 cells

missed.txt holds only the two documented equivalents and timeout.txt only
the permanent ṇatva-scan entry; every 6.4.98, 6.4.42/6.4.43 and 8.4.40 mutant is caught."
```

---

## Task 9: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -20
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin juhotyadi-3f3
gh pr create --title "juhotyādi 3f3 — √jan; the gaṇa closes" --body "$(cat <<'BODY'
Slice 3f3 curates √jan (`03.0025 jana~`, *jajanti*), parasmaipadī, the last
juhotyādi row. The golden suite goes from 4608 to 4644 cells and juhotyādi is
complete at twenty-six of twenty-six rows: all nine gaṇas are now complete.

**Three new rules in `guna.rs`, all keyed by row number:**
- 6.4.98 *gamahanajanakhanaghasāṁ lopaḥ* elides √jan's upadhā before a
  vowel-initial kṅit: *jajYati*, *ajajYuH*.
- 6.4.42 *janasanakhanāṁ sañjhaloḥ* turns its `n` into `ā` before a
  jhal-initial kṅit: *jajAtaH*, *jajAhi*. Its key holds tanādi's √san too,
  which declines on its follower, the vikaraṇa `u`.
- 6.4.43 *ye vibhāṣā*, the engine's twelfth vikalpa, does the same before
  yāsuṭ: *jajanyAt* ~ *jajAyAt*.

6.4.42 and 6.4.43 run after the ā-of-abhyasta rules (6.4.22 *asiddhavat*), or
6.4.112/6.4.113 would eat the new ā.

**One widening in `tripadi.rs`:** 8.4.40 *stoḥ ścunā ścuḥ* gains its converse
arm (`jn` → `jY`), with the 8.4.44 *śāt* exemption as a guard so √kliś and √aś
keep their `Sn`. Its credits off √jan are pinned at 54, the count on `main`.

The audit shows zero divergence against `8da2f90b`, a main-vs-branch dump of
every prior cell's traces is byte-identical, and the mutation gate is clean.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`: `git worktree remove .worktrees/juhotyadi-3f3`, then delete the local and remote branch, and run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| 8.4.40 converse arm, *śāt* as a guard, comment rewritten, order unchanged | 2 |
| 8.4.40 unit test: converse firing, kliSnAti guard | 2 |
| 6.4.98 beside 6.4.100, row-keyed, vowel-initial kṅit, `an` suffix; tests; order pin | 3 |
| 6.4.42 / 6.4.43 after 6.4.115, `JANA_SANA` key, √san declines on its follower, `jA` in one step; tests; order and vikalpa pins | 4 |
| Placement guard test (6.4.112/6.4.113 would eat `jA`) | 4 |
| Row `03.0025 jana~`, counts, gaṇa-row test, 4 blocks, 15 `ALTERNATES`, buckets, keys, `VIKALPA_RULES` | 5 |
| `check()` spot test (*jajYati*, *ajajYuH*, *jajAtaH*, *jajAyAt*) | 5 |
| 7.3.87 and 8.3.24 allow-lists gain √jan | 5 |
| Trace pins *jajYati*, *ajajYuH*, *jajAtaH*, *jajAhi*, *jajAyAt* ~ *jajanyAt*, *jajanti* | 6 |
| Fires-only: 6.4.98/6.4.42/6.4.43 on √jan; 8.4.40 off √jan at 54 on `07.0003`/`07.0008` | 6 |
| Prior traces byte-identical, main ↔ branch | 2–4 (suite at 4608), 7 Step 2 (dump diff) |
| Audit with repoint and negative control; README/ARCHITECTURE/AGENTS; "3f3" sweep | 7 |
| Floor, uncaught probe, campaign, verbatim non-caught record | 8 |

**Spec deviations, recorded.**
- The spec's draft named the upadeśa `janI~`. The dhātupāṭha row is `jana~ janane`; `janI~\` is divādi's `04.0044`. The spec was corrected alongside this plan.
- The spec asks for a guard test "the order pin: 6.4.112 and 6.4.113 run on `jan` and decline". Task 4's `the_abhyasta_rules_would_eat_janas_a_so_janasana_runs_after_them` tests the stronger half: on `jA` both rules *would* fire. It then asserts the array order directly. The trace pin `jajAtaH_trace_takes_the_a_without_the_abhyasta_rules` covers "decline on `jan`" end to end.
- The 8.4.40 fires-only test is a credit count plus a row set (54 on `07.0003`/`07.0008`), not "credited with a ścu on the left". A `RuleStep` carries no direction. The spec was updated to match.

**Type consistency.**
- Rule ids and names: `"6.4.98"` / `"gamahanajanaKanaGasAM lopaH kNityanaNi"`, `"6.4.42"` / `"janasanaKanAM saYJaloH"`, `"6.4.43"` / `"ye viBAzA"`, `"8.4.40"` / `"stoH ScunA ScuH"`, everywhere.
- `jan_prakriya(&'static str, &str, &str, bool)` is defined in Task 3 and used in Tasks 3–4.
- `JANA_SANA` and `janasanakhanam_a` are defined in Task 4 and used only there.
- `branch_trace`, `credited` and `cell_trace` already exist in `trace/juhotyadi.rs` / `helpers`.

**Known soft spots.**
- **The 54 pin.** It was measured from the throwaway dump at `436a507`, counting live branches as `credited()` does. Task 7 Step 2 re-measures it on `main` directly.
- **`check()` on an alternate.** `jajAyAt` is not a `PARADIGM` form. The prototype ran `jan_analyses_its_reduplicated_forms` green, so `check` resolves alternates through their branch.
- **The `threes`/`fours` messages and their `\` continuations.** Edit without breaking them.
- **`tripadi.rs` line drift.** Task 8 finds the two moved entries by `--list` and by shape.
