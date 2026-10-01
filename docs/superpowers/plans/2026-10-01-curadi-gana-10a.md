# Curādi gaṇa slice 10a Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Open curādi (gaṇa 10) with √cur (`10.0001`), √laḍ (`10.0010`), √bhakṣ (`10.0033`) and √bhūṣ (`10.0255`), all ubhayapadī by 1.3.74 *ṇicaś ca*. The golden suite goes from 4644 to 4932 cells. ṇic (3.1.25) enters through a new first pipeline stage, `sanadi`, and 3.1.32 *sanādyantā dhātavaḥ* folds it into the aṅga, so every later stage sees an ordinary i-final dhātu (`cori`).

**Architecture:** Nine tasks.
- **Task 1** checks the worktree and baseline.
- **Task 2** adds the enum variants and tags, wired through `derive`, with no rows yet.
- **Task 3** adds 1.3.74 and its 1.3.78 arm.
- **Task 4** adds the `sanadi` stage and makes the stage tests look rules up stage-locally.
- Tasks 2–4 fire on no curated cell. Each is gated on **its unit tests plus the 4644 priors staying green**.
- **Task 5** lands the four rows and their goldens (288 cells).
- **Task 6** adds the trace pins and the corpus-wide fires-only tests.
- **Tasks 7–9** are the audit with the prior-trace diff and doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.99.0 pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, checked out at `/tmp/vidyut-full`.

**Spec:** `docs/superpowers/specs/2026-10-01-curadi-gana-10a-design.md`

**Workspace:** the branch `curadi-10a` is checked out at `/workspace/.worktrees/curadi-10a` and holds the spec and this plan. Every path below is relative to that directory unless it starts with `/`.

**Provenance.** Every code block in this plan ran green on a throwaway worktree, `/workspace/.worktrees/curadi-10a-proto`, commit `2e7c3ef` (detached; delete it in Task 9). That run covered:
- the full suite and clippy `-D warnings`;
- the audit at 107 roots / 4932 cells / 6062 forms with **zero differences** against vidyut;
- a byte-identical dump of all 5750 prior live-branch logs, `main` vs prototype;
- a scoped mutation probe over the new code: 31 mutants (`sanadi.rs`, 1.3.74, 1.3.78's new arm, `derive`'s mapping), all caught.

## Global Constraints

- **The 4644 pre-existing cells must stay byte-identical, traces included.** Regenerate no golden and change no pinned trace.
- **Six new pipeline entries in a new first stage plus one in `samjna`.** `tinanta_rule_order_is_pinned` grows from 129 to 136 ids. It opens with `"3.1.25", "1.3.9", "3.4.114", "7.2.116", "7.3.86", "3.1.32"` and gains `"1.3.74"` between `"1.3.72"` and `"1.3.78"`. **No new vikalpa**: `exactly_the_pinned_vikalpa_rules_are_optional` is unchanged.
- **Ids are not unique.** After Task 4, `1.3.9` appears twice and `7.3.86` three times. A stage's unit test must look a rule up in **its own stage static** (`SANADI`, `SAMJNA`, `GUNA`, `TIN`), never with `rules().find(…)`, or it gets the sanādi entry.
- **Every sanādi rule guards on ṇic's identity**, never on "a term exists at index 3":
  - 1.3.9 on text `Ric`;
  - 3.4.114, 7.2.116 and 3.1.32 on `Tag::Rit`;
  - 7.3.86 on `Tag::Ardhadhatuka`.
  
  Index 3 is the ending's slot once `samjna` runs.
- **Pada is row-driven**: `PadaAssignment::Nic` maps to `Tag::Nic`, read only by 1.3.74 and 1.3.78. `Tag::Nijanta` (set by 3.1.32) has no reader in 10a.
- **No unfalsifiable guard clauses.** A clause no cell and no guard test can make false is a mutation survivor.
- **Goldens are transcribed from this plan**: the prototype engine's output, matched against vidyut at `8da2f90b` (zero differences). **ALTERNATES keys are this engine's** (its log ∩ `VIKALPA_RULES`); never invent one. **Do not edit a golden to match the engine.**
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit.
- `mise run test` takes about 10 s. Run it in the **foreground** with a timeout of 600000 ms. Never background it and end a turn.
- `mise run test -- -p X` does not scope. Scope unit tests with `mise exec -- cargo test -p <crate> <filter>`.
- Rule ids and SLP1 names, verbatim: `3.1.25 satyApapASarUpavIRAtUlaSlokasenAlomatvacavarmavarRacUrRacurAdiByo Ric`, `1.3.9 tasya lopaH`, `3.4.114 ArDaDAtukaM SezaH`, `7.2.116 ata upaDAyAH`, `7.3.86 pugantalaGUpaDasya ca`, `3.1.32 sanAdyantA DAtavaH`, `1.3.74 RicaS ca`.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **A stage test silently exercising the sanādi entry.** Ten `guna.rs` lookups, one `samjna.rs` test and one `tin.rs` chain test found `7.3.86`/`1.3.9` through `rules()`. In the prototype, before the fix, seven of them failed and the rest passed by testing the wrong rule. → Task 4, Steps 4–5.
2. **A tiṅ ending sitting at index 3.** The sanādi rules must decline on it, or a hand-built chain (or a reordered stage) rewrites `tip` to `i`. → Task 4, `nic_it_lopa_leaves_a_nit_i`, `ardhadhatuka_sesah_tags_nic_only`, `sanadyanta_folds_nic_into_the_dhatu`.
3. **A prior row reaching a new rule.** Goldens ignore traces. → Task 6, `the_sanadi_rules_and_nicas_ca_are_credited_only_on_curadi`, and `pugantalaghupadhasya_off_curadi_is_credited_exactly_as_before_10a` (384, measured on `main`); Task 7 Step 2's prior-trace diff.
4. **Pada-ambiguous curādi surfaces** (`corayatAm` is loṭ P.D parasmaipada *and* loṭ P.E ātmanepada). → Task 5, `pada_ambiguous_surfaces_are_exactly_these` gains 16.
5. **`check()` on the new forms**, including the ātmanepada *corayate* and an alternate-free ṇatva form. → Task 5, `curadi_analyses_its_nijanta_forms`.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-data/src/lib.rs` | Task 2: `Gana::Curadi`, `PadaAssignment::Nic`, `padas()`, the prefix test, the pada tests. Task 5: four rows, the row test, the pada-marker exception, the count |
| `crates/panini-prakriya/src/term.rs` | Task 2: `Tag::Curadi`, `Nic`, `Nijanta`, `Rit` |
| `crates/panini-prakriya/src/tinanta/mod.rs` | Task 2: `derive`'s two arms. Task 4: `mod sanadi`, `SANADI` first in `TINANTA_RULES`, module doc |
| `crates/panini-prakriya/src/tinanta/samjna.rs` | Task 2: test helper arm. Task 3: 1.3.74, 1.3.78's arm, tests. Task 4: one stage-local lookup |
| `crates/panini-prakriya/src/tinanta/terms.rs` | Task 4: `NIC` |
| `crates/panini-prakriya/src/tinanta/sanadi.rs` | Task 4: new — the stage and its tests |
| `crates/panini-prakriya/src/tinanta/guna.rs`, `tin.rs` | Task 4: stage-local lookups in tests |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Tasks 3, 4: the rule-order pin and its doc |
| `crates/panini/tests/paradigm/data/{curadi,mod}.rs` | Task 5 |
| `crates/panini/tests/paradigm/main.rs` | Task 5: totals, buckets, key census, `VIKALPA_RULES` doc, pada-ambiguous list, `check()` test. Task 7: audit prose |
| `crates/panini/tests/trace/{helpers,juhotyadi,curadi,main}.rs` | Task 6 |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, the 3f3 spec | Task 7 |
| `AGENTS.md`, maybe `mise.toml` | Task 8 |

---

## Task 1: The worktree and baseline

**Files:** none. **Interfaces:** none.

- [ ] **Step 1: Confirm the worktree**

```bash
cd /workspace/.worktrees/curadi-10a
git status --short          # empty
git log --oneline -3        # the plan commit, the spec commit (1a4275f), then 4dd7b53
```

- [ ] **Step 2: Verify the baseline**

```bash
mise trust && mise install
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -15
```

Run in the foreground, timeout 600000 ms. Expected: all pass at 4644 cells.

---

## Task 2: The variants and tags

Adds every new enum variant and tag, and the `derive` arms that map the data layer onto the tags. No row uses them yet, so no derivation changes.

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini-prakriya/src/term.rs`
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs`
- Modify: `crates/panini-prakriya/src/tinanta/samjna.rs` (test helper only)

**Interfaces:**
- Produces: `panini_data::Gana::Curadi`; `panini_data::PadaAssignment::Nic`, whose `padas()` is `&[Pada::Parasmaipada, Pada::Atmanepada]`; `Tag::Curadi`, `Tag::Nic`, `Tag::Nijanta`, `Tag::Rit`. `derive` adds `Tag::Curadi` for `Gana::Curadi` and `Tag::Nic` for `PadaAssignment::Nic`.

- [ ] **Step 1: Write the failing tests**

In `crates/panini-data/src/lib.rs`'s test module, extend the two `padas()` tests. In the test whose last assertion is

```rust
        assert_eq!(
            PadaAssignment::UbhayapadaAnavane.padas(),
            &[Pada::Parasmaipada, Pada::Atmanepada]
        );
    }
```

replace that with

```rust
        assert_eq!(
            PadaAssignment::UbhayapadaAnavane.padas(),
            &[Pada::Parasmaipada, Pada::Atmanepada]
        );
        assert_eq!(
            PadaAssignment::Nic.padas(),
            &[Pada::Parasmaipada, Pada::Atmanepada]
        );
    }
```

and in `ubhayapada_padas_are_parasmaipada_first`, replace

```rust
        assert_eq!(
            PadaAssignment::UbhayapadaAnavane.padas()[0],
            Pada::Parasmaipada
        );
    }
```

with

```rust
        assert_eq!(
            PadaAssignment::UbhayapadaAnavane.padas()[0],
            Pada::Parasmaipada
        );
        assert_eq!(PadaAssignment::Nic.padas()[0], Pada::Parasmaipada);
    }
```

In `gana_matches_dhatupatha_prefix`, replace the comment

```rust
        // Mapped variant → prefix, not the inverse: this engine covers nine
        // of the ten gaṇas, so only 10 has no `Gana` variant.
```

with

```rust
        // Mapped variant → prefix, not the inverse, so a new variant must
        // name its prefix here before it compiles.
```

and add the arm `Gana::Curadi => "10",` after `Gana::Kryadi => "09",`.

- [ ] **Step 2: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-data 2>&1 | tail -5`
Expected: compile errors, because `PadaAssignment::Nic` and `Gana::Curadi` do not exist.

- [ ] **Step 3: Add the variants**

In `crates/panini-data/src/lib.rs`, add `Curadi,` after `Juhotyadi,` in `enum Gana`. In `enum PadaAssignment`, after `UbhayapadaAnavane,` (and its doc comment), add:

```rust
    /// Both padas derive, exactly as for `Ubhayapada` — but the ātmanepada
    /// arm is sanctioned by 1.3.74 *ṇicaś ca*, a sūtra keyed on the affix
    /// ṇic, rather than by any marker of the root's. Every curādi root takes
    /// ṇic (3.1.25), so a curādi row with no pada marker of its own is
    /// curated with this. Like `UbhayapadaAnavane`, a root carrying it must
    /// never reach 1.3.72, or the trace credits the wrong sūtra.
    ///
    /// Row-driven for now: 1.3.74 reads the pada licence the data layer
    /// supplies, not the presence of ṇic. When the causative (ṇic after a
    /// root of any gaṇa) is implemented, 1.3.74 should key on the ṇijanta
    /// stem instead, and this variant retires.
    Nic,
```

In `padas()`, replace

```rust
            PadaAssignment::Ubhayapada | PadaAssignment::UbhayapadaAnavane => {
                &[Pada::Parasmaipada, Pada::Atmanepada]
            }
```

with

```rust
            PadaAssignment::Ubhayapada
            | PadaAssignment::UbhayapadaAnavane
            | PadaAssignment::Nic => &[Pada::Parasmaipada, Pada::Atmanepada],
```

- [ ] **Step 4: Add the tags**

In `crates/panini-prakriya/src/term.rs`, immediately above `/// The dhātu belongs to juhotyādi (gaṇa 3), the ślu gaṇa.`, insert:

```rust
    /// The dhātu belongs to curādi (gaṇa 10). Read by 3.1.25 alone, which
    /// adds ṇic. Mirrors Divadi/Tudadi/Adadi/Kryadi/Svadi/Rudhadi/Tanadi/
    /// Juhotyadi.
    Curadi,
    /// The dhātu's ātmanepada is sanctioned by 1.3.74 *ṇicaś ca*: the
    /// data layer's `PadaAssignment::Nic`. Read only by 1.3.74, and by
    /// 1.3.78's ātmanepada arm, which declines rather than blocks when it
    /// is present. Distinct from `Nijanta`, which says what the stem IS;
    /// this says what licenses its pada. Same standing as `Anavane`: a
    /// pada licence keyed to a sūtra, so the trace credits that sūtra and
    /// never 1.3.72.
    Nic,
    /// The aṅga is a ṇijanta: 3.1.32 *sanādyantā dhātavaḥ* folded ṇic into
    /// it, so its final `i` is ṇic's. A saṁjñā verdict, set by 3.1.32 and
    /// pinned by its unit test; no rule reads it yet. 6.4.51 *ṇer aniṭi*,
    /// in an ārdhadhātuka-lakāra slice, is the first rule that will.
    Nijanta,
    /// The pratyaya carries the ṇ-anubandha (ṇit), SLP1 `R` as `Ngit`'s
    /// `N` is ṅ. Set on ṇic by its it-lopa in `super::sanadi`; read by
    /// 7.2.116 *ata upadhāyāḥ*, whose following ñit/ṇit it is.
    Rit,
```

(`super::sanadi` names a module Task 4 creates. That is a doc comment, not a path, so it compiles now.)

- [ ] **Step 5: Wire `derive` and the samjna test helper**

In `crates/panini-prakriya/src/tinanta/mod.rs`'s `derive`, add `PadaAssignment::Nic => t.add(Tag::Nic),` after the `PadaAssignment::UbhayapadaAnavane => t.add(Tag::Anavane),` arm, and `Gana::Curadi => t.add(Tag::Curadi),` after the `Gana::Juhotyadi => t.add(Tag::Juhotyadi),` arm.

In `crates/panini-prakriya/src/tinanta/samjna.rs`'s test helper `pada_anga`, add `PadaAssignment::Nic => t.add(Tag::Nic),` after its `UbhayapadaAnavane` arm.

- [ ] **Step 6: Run the tests**

```bash
mise exec -- cargo test -p panini-data 2>&1 | tail -3     # 21 passed
mise run test 2>&1 | grep -E "FAILED|test result" | head   # all pass at 4644 cells
```

`tools/audit/panini_full_audit.rs`'s `gana_name` also matches `Gana` exhaustively. It is not a workspace member, so Task 7 Step 1 edits it.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-data/src/lib.rs crates/panini-prakriya/src/term.rs crates/panini-prakriya/src/tinanta/mod.rs crates/panini-prakriya/src/tinanta/samjna.rs
git commit -m "feat(data,prakriya): Gana::Curadi, PadaAssignment::Nic and the four curādi tags

No row uses them yet; derive maps the data layer onto Tag::Curadi / Tag::Nic."
```

---

## Task 3: 1.3.74 *ṇicaś ca*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/samjna.rs`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: `Tag::Nic` (Task 2).
- Produces: the `SAMJNA` rule `"1.3.74"`, placed between `"1.3.72"` and `"1.3.78"`.

- [ ] **Step 1: Write the failing tests**

In `samjna.rs`'s test module, immediately above `#[test]` / `fn svaritanit_reports_firing_only_on_atmanepada()`, add the helper:

```rust
    /// `pada_prakriya` for a curādi root, hand-built: the curādi rows land in
    /// the data task after this one, and 1.3.74 reads only `Tag::Nic`, so the
    /// term is constructed directly, as `anavane_prakriya` is.
    fn nic_prakriya(pada: Pada) -> Prakriya {
        let mut t = Term::new("cur");
        t.add(Tag::Dhatu);
        t.add(Tag::Nic);
        let mut p = Prakriya {
            ctx: Context::new(Lakara::Lat, pada, Purusha::Prathama, Vacana::Eka),
            ..Default::default()
        };
        p.terms = with_slots(vec![t]);
        p
    }
```

Immediately above `#[test]` / `fn pada_sutras_are_order_independent()`, add:

```rust
    #[test]
    fn nicas_ca_reports_firing_only_on_atmanepada() {
        // Same shape as 1.3.72's and 1.3.66's: 1.3.74 sanctions a Nic root's
        // ātmanepada reading and DECLINES its parasmaipada one, without
        // blocking — 1.3.78 sanctions that.
        let rule = SAMJNA.iter().find(|r| r.id == "1.3.74").unwrap();
        for (pada, fires) in [(Pada::Atmanepada, true), (Pada::Parasmaipada, false)] {
            let mut p = nic_prakriya(pada);
            assert_eq!((rule.apply)(&mut p), fires, "1.3.74 on {pada:?}");
            assert!(!p.blocked, "1.3.74 must never block, {pada:?}");
        }
    }

    #[test]
    fn nicas_ca_declines_for_roots_without_the_nic_licence() {
        // The guard is Tag::Nic and nothing else: √rudh (1.3.72's), √bhū
        // (1.3.78's), √khid (1.3.12's) and √bhuj (1.3.66's) are left alone in
        // both padas, without recording and without blocking.
        let rule = SAMJNA.iter().find(|r| r.id == "1.3.74").unwrap();
        for number in ["07.0001", "01.0001", "07.0012", "07.0017"] {
            for pada in [Pada::Parasmaipada, Pada::Atmanepada] {
                let mut p = pada_prakriya(number, pada);
                assert!(!(rule.apply)(&mut p), "1.3.74 fired on {number} {pada:?}");
                assert!(!p.blocked, "1.3.74 blocked {number} {pada:?}");
                assert!(p.log.is_empty(), "1.3.74 recorded on {number} {pada:?}");
            }
        }
    }

    #[test]
    fn the_other_pada_sutras_leave_a_nic_root_to_1_3_74_and_1_3_78() {
        // The wrong-sūtra-credit case: a curādi root carries no marker, so
        // 1.3.12, 1.3.66 and 1.3.72 must never fire on it. 1.3.78 fires on
        // its parasmaipada and DECLINES its ātmanepada, which 1.3.74 has
        // sanctioned — blocking there would erase the ātmanepada column.
        for id in ["1.3.12", "1.3.66", "1.3.72"] {
            let rule = SAMJNA.iter().find(|r| r.id == id).unwrap();
            for pada in [Pada::Parasmaipada, Pada::Atmanepada] {
                let mut p = nic_prakriya(pada);
                assert!(!(rule.apply)(&mut p), "{id} fired on cur {pada:?}");
                assert!(!p.blocked, "{id} blocked cur {pada:?}");
            }
        }
        let rule = SAMJNA.iter().find(|r| r.id == "1.3.78").unwrap();
        let mut p = nic_prakriya(Pada::Atmanepada);
        assert!(!(rule.apply)(&mut p), "1.3.78 fired on cur Atmanepada");
        assert!(!p.blocked, "1.3.78 blocked cur Atmanepada");
        let mut p = nic_prakriya(Pada::Parasmaipada);
        assert!((rule.apply)(&mut p), "1.3.78 declined cur Parasmaipada");
        assert!(!p.blocked);
    }
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya samjna 2>&1 | grep -E "FAILED|panicked" | head`
Expected: the two `nicas_ca_*` tests panic on `unwrap()` (there is no `1.3.74`), and `the_other_pada_sutras_…` fails on `1.3.78 blocked cur Atmanepada`.

- [ ] **Step 3: Add the rule and 1.3.78's arm**

In the module doc (line 2), `//! 1.3.12, 1.3.66, 1.3.72, 1.3.78, 3.4.78, 1.3.9, 1.2.4.` → `//! 1.3.12, 1.3.66, 1.3.72, 1.3.74, 1.3.78, 3.4.78, 1.3.9, 1.2.4.`

Immediately above `    // 1.3.78 śeṣāt kartari parasmaipadam: everything else takes parasmaipada.`, insert:

```rust
    // 1.3.74 ṇicaś ca: a ṇijanta takes ātmanepada (when the fruit accrues
    // to the agent — unmodelled, exactly as 1.3.72's *kartrabhiprāye
    // kriyāphale* above). Affix-keyed: the guard is Tag::Nic, the data
    // layer's PadaAssignment::Nic, which every curated curādi row carries.
    // Not keyed on Tag::Nijanta yet — see that variant's doc comment.
    //
    // The parasmaipada arm DECLINES rather than blocks, for 1.3.72's and
    // 1.3.66's reason: 1.3.78 below sanctions it. Structural twin of both;
    // only the guard tag and the credited sūtra differ.
    Rule {
        id: "1.3.74",
        name: "RicaS ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Nic) {
                return false;
            }
            match p.ctx.pada {
                Pada::Atmanepada => {
                    let before = p.snapshot();
                    p.record("1.3.74", "RicaS ca", before);
                    true
                }
                Pada::Parasmaipada => false,
            }
        },
    },
```

In 1.3.78's ātmanepada arm, replace

```rust
                // they split on ctx.pada: 1.3.72 (Ubhayapadin) or 1.3.66 (Anavane) has
                // already sanctioned this cell, so decline instead of blocking. Only the
                // genuine śeṣa (no pada tag at all) blocks here.
                Pada::Atmanepada => {
                    if p.terms[ANGA].has(Tag::Ubhayapadin) || p.terms[ANGA].has(Tag::Anavane) {
```

with

```rust
                // they split on ctx.pada: 1.3.72 (Ubhayapadin), 1.3.66 (Anavane) or
                // 1.3.74 (Nic) has already sanctioned this cell, so decline instead of
                // blocking. Only the genuine śeṣa (no pada tag at all) blocks here.
                Pada::Atmanepada => {
                    if p.terms[ANGA].has(Tag::Ubhayapadin)
                        || p.terms[ANGA].has(Tag::Anavane)
                        || p.terms[ANGA].has(Tag::Nic)
                    {
```

- [ ] **Step 4: Pin the order**

In `derivation_tests.rs`'s `tinanta_rule_order_is_pinned`, change the first line of `expected` from

```rust
        "1.3.12", "1.3.66", "1.3.72", "1.3.78", "3.4.78", "1.3.9", "1.2.4", "3.4.85", "3.4.108",
```

to

```rust
        "1.3.12", "1.3.66", "1.3.72", "1.3.74", "1.3.78", "3.4.78", "1.3.9", "1.2.4", "3.4.85",
        "3.4.108",
```

(`mise run fmt` rewraps the array.)

- [ ] **Step 5: Run the tests**

```bash
mise exec -- cargo test -p panini-prakriya 2>&1 | grep -E "FAILED|test result"
mise run test 2>&1 | grep -E "FAILED|test result" | head
```

Expected: PASS, with the 4644 priors unchanged. No curated row carries `Tag::Nic` yet.

- [ ] **Step 6: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/samjna.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 1.3.74 ṇicaś ca — the ṇijanta's ātmanepada, 1.3.78 declining on Tag::Nic"
```

---

## Task 4: The sanādi stage

**Files:**
- Create: `crates/panini-prakriya/src/tinanta/sanadi.rs`
- Modify: `crates/panini-prakriya/src/tinanta/terms.rs`, `mod.rs`, `derivation_tests.rs`, `guna.rs`, `samjna.rs`, `tin.rs`

**Interfaces:**
- Consumes: `Tag::Curadi`, `Tag::Rit`, `Tag::Nijanta` (Task 2); `crate::tinanta::sound::guna_of(char) -> Option<&'static str>`; `crate::tinanta::terms::{ANGA, with_slots}`.
- Produces: `pub(crate) const NIC: usize = 3` in `terms.rs`; `pub(crate) static SANADI: &[Rule]`, the first entry of `TINANTA_RULES`.

- [ ] **Step 1: Add `NIC`**

In `terms.rs`, immediately above `/// Index of the tiṅ ending *before* śap is inserted (3.1.68).`, insert:

```rust
/// Index of ṇic (3.1.25) while `super::sanadi` runs — and only then.
/// 3.1.32 folds ṇic into `ANGA` and removes this term before `super::samjna`
/// starts, so by the time 3.4.78 pushes the ending this index is free again
/// and the ending lands at `ENDING_PRE_SHAP`, the same value. No tiṅ exists
/// while ṇic holds it. Present only for curādi; for every other gaṇa
/// `sanadi` adds nothing and the slot is never occupied.
pub(crate) const NIC: usize = 3;

```

- [ ] **Step 2: Create `sanadi.rs` with its tests**

Create `crates/panini-prakriya/src/tinanta/sanadi.rs` with exactly this content:

```rust
//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, ṇic's
//! it-lopa (1.3.9), 3.4.114, 7.2.116, 7.3.86, 3.1.32.
//!
//! First in the pipeline, before any lakāra or tiṅ exists. The layout here
//! is `[AGAMA, ABHYASA, ANGA, ṇic]`, ṇic at `NIC`; 3.1.32 folds ṇic into
//! `ANGA` and removes it, so `super::samjna` starts on the same
//! `[AGAMA, ABHYASA, ANGA]` every other gaṇa does. From there on the aṅga is
//! an ordinary i-final dhātu (`cori`), and 7.3.84 then 6.1.78 make `coray-`
//! exactly as they make √nī's `nay-`. See `super::terms`.
//!
//! Every rule self-guards: 3.1.25 on `Tag::Curadi`, the rest on ṇic being
//! present. For gaṇas 1–9 the stage adds nothing and records nothing.

use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use crate::tinanta::sound::guna_of;
use crate::tinanta::terms::{ANGA, NIC};

pub(crate) static SANADI: &[Rule] = &[
    // 3.1.25 satyāpapāśarūpavīṇātūlaślokasenālomatvacavarmavarṇacūrṇa-
    // curādibhyo ṇic: ṇic after a curādi root. The sūtra's nominal bases
    // (satya, pāśa, …) are not in scope; the gaṇa is.
    Rule {
        id: "3.1.25",
        name: "satyApapASarUpavIRAtUlaSlokasenAlomatvacavarmavarRacUrRacurAdiByo Ric",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Curadi) {
                return false;
            }
            let before = p.snapshot();
            let mut nic = Term::new("Ric");
            nic.add(Tag::Pratyaya);
            p.terms.push(nic);
            p.record(
                "3.1.25",
                "satyApapASarUpavIRAtUlaSlokasenAlomatvacavarmavarRacUrRacurAdiByo Ric",
                before,
            );
            true
        },
    },
    // 1.3.9 tasya lopaḥ, on ṇic: 1.3.7 cuṭū makes the initial ṇ an it, 1.3.3
    // halantyam the final c, and both go, leaving `i`. Local to ṇic, not
    // `it_samjna::run_it_samjna`: that helper has no 1.3.7 arm, and a
    // general one would strip the jh of the tiṅ ending `Ji`, which the
    // tradition exempts. The ṇ is recorded as `Tag::Rit` for 7.2.116. One
    // `1.3.9` step, the engine's it-lopa trace convention.
    Rule {
        id: "1.3.9",
        name: "tasya lopaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let Some(nic) = p.terms.get(NIC).filter(|t| t.text == "Ric") else {
                return false;
            };
            let before = p.snapshot();
            let mut nic = nic.clone();
            nic.text = "i".to_string();
            nic.add(Tag::Rit);
            p.terms[NIC] = nic;
            p.record("1.3.9", "tasya lopaH", before);
            true
        },
    },
    // 3.4.114 ārdhadhātukaṃ śeṣaḥ: ṇic is neither tiṅ nor śit, so it is
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
            }
            let before = p.snapshot();
            p.terms[NIC].add(Tag::Ardhadhatuka);
            p.record("3.4.114", "ArDaDAtukaM SezaH", before);
            true
        },
    },
    // 7.2.116 ata upadhāyāḥ: vṛddhi of an `a` upadhā before a ñit or ṇit
    // affix. `laq` → `lAq` before ṇic. Only ṇit has a carrier in this engine
    // (`Tag::Rit`); no ñit affix is in scope.
    Rule {
        id: "7.2.116",
        name: "ata upaDAyAH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) {
                return false;
            }
            let mut chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let n = chars.len();
            if n < 2 || chars[n - 2] != 'a' {
                return false;
            }
            let before = p.snapshot();
            chars[n - 2] = 'A';
            p.terms[ANGA].text = chars.into_iter().collect();
            p.record("7.2.116", "ata upaDAyAH", before);
            true
        },
    },
    // 7.3.86 pugantalaghūpadhasya ca, before ṇic: guṇa of a laghu ik upadhā
    // before an ārdhadhātuka. `cur` → `cor`. A third entry under this id —
    // the guṇa-stage pair (nitya, and tanādi's vikalpa) reads the aṅga
    // before the vikaraṇa, by which point a curādi aṅga is the vowel-final
    // `cori` and their "final-vowel aṅgas are 7.3.84's business" early
    // return declines. This entry is the one occasion that sees the root
    // with ṇic beside it.
    Rule {
        id: "7.3.86",
        name: "pugantalaGUpaDasya ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Ardhadhatuka)) {
                return false;
            }
            let chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let n = chars.len();
            if n < 2 || !matches!(chars[n - 2], 'i' | 'u' | 'f' | 'x') {
                return false;
            }
            let Some(g) = guna_of(chars[n - 2]) else {
                return false;
            };
            let before = p.snapshot();
            let mut s: String = chars[..n - 2].iter().collect();
            s.push_str(g);
            s.push(chars[n - 1]);
            p.terms[ANGA].text = s;
            p.record("7.3.86", "pugantalaGUpaDasya ca", before);
            true
        },
    },
    // 3.1.32 sanādyantā dhātavaḥ: root + ṇic is a dhātu. Fold ṇic's text into
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
            p.terms[ANGA].add(Tag::Nijanta);
            p.record("3.1.32", "sanAdyantA DAtavaH", before);
            true
        },
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prakriya::Prakriya;
    use crate::tinanta::terms::with_slots;

    /// The sanādi stage's own entry for `id` — looked up in `SANADI`, not
    /// `rules()`, so it can never be another stage's 1.3.9 or 7.3.86.
    fn rule(id: &str) -> &'static Rule {
        SANADI.iter().find(|r| r.id == id).unwrap()
    }

    /// `root`, as a curādi dhātu, alone or followed by ṇic as it stands
    /// after its it-lopa (`i`, ṇit), optionally already ārdhadhātuka.
    fn with_nic(root: &str, nic: Option<&[Tag]>) -> Prakriya {
        let mut anga = Term::new(root);
        anga.add(Tag::Dhatu);
        anga.add(Tag::Curadi);
        let mut terms = vec![anga];
        if let Some(tags) = nic {
            let mut t = Term::new("i");
            t.add(Tag::Pratyaya);
            for tag in tags {
                t.add(*tag);
            }
            terms.push(t);
        }
        Prakriya {
            terms: with_slots(terms),
            ..Default::default()
        }
    }

    #[test]
    fn nic_is_added_after_a_curadi_root_only() {
        let mut p = with_nic("cur", None);
        assert!((rule("3.1.25").apply)(&mut p));
        assert_eq!(p.terms[NIC].text, "Ric");
        assert!(p.terms[NIC].has(Tag::Pratyaya));
        // √bhū: no Curadi tag, no ṇic.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("BU")]),
            ..Default::default()
        };
        assert!(!(rule("3.1.25").apply)(&mut p));
        assert_eq!(p.terms.len(), NIC);
        assert!(p.log.is_empty());
    }

    #[test]
    fn nic_it_lopa_leaves_a_nit_i() {
        let mut p = with_nic("cur", None);
        (rule("3.1.25").apply)(&mut p);
        assert!((rule("1.3.9").apply)(&mut p));
        assert_eq!(p.terms[NIC].text, "i");
        assert!(p.terms[NIC].has(Tag::Rit));
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["3.1.25", "1.3.9"]);
        // Nothing at NIC, or something that is not ṇic: decline. The second
        // case is a tiṅ ending where ṇic would sit — the layout `samjna`
        // builds — which this rule must never touch.
        let mut p = with_nic("BU", None);
        assert!(!(rule("1.3.9").apply)(&mut p));
        let mut p = with_nic("BU", None);
        p.terms.push(Term::new("tip"));
        assert!(!(rule("1.3.9").apply)(&mut p));
        assert_eq!(p.terms[NIC].text, "tip");
    }

    #[test]
    fn ardhadhatuka_sesah_tags_nic_only() {
        let mut p = with_nic("cur", Some(&[Tag::Rit]));
        assert!((rule("3.4.114").apply)(&mut p));
        assert!(p.terms[NIC].has(Tag::Ardhadhatuka));
        let mut p = with_nic("cur", None);
        assert!(!(rule("3.4.114").apply)(&mut p));
        let mut p = with_nic("cur", None);
        p.terms.push(Term::new("ti"));
        assert!(!(rule("3.4.114").apply)(&mut p));
        assert!(!p.terms[NIC].has(Tag::Ardhadhatuka));
    }

    #[test]
    fn ata_upadhayah_lengthens_an_a_upadha_before_nit() {
        for (root, want) in [("laq", "lAq"), ("aw", "Aw")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("7.2.116").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
        }
        // Not an `a` upadhā (Bakz: k; cur: u), too short to have one (a), or
        // no ṇit follower.
        for root in ["Bakz", "cur", "a"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("7.2.116").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        let mut p = with_nic("laq", Some(&[Tag::Ardhadhatuka]));
        assert!(!(rule("7.2.116").apply)(&mut p));
    }

    #[test]
    fn pugantalaghupadhasya_gunates_a_laghu_ik_upadha_before_nic() {
        for (root, want) in [("cur", "cor"), ("uz", "oz"), ("kft", "kart")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("7.3.86").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
        }
        // Guru upadhā (BUz: long U; Bakz: k), too short (u), or a ṇic not
        // (yet) ārdhadhātuka.
        for root in ["BUz", "Bakz", "u"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("7.3.86").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        let mut p = with_nic("cur", Some(&[Tag::Rit]));
        assert!(!(rule("7.3.86").apply)(&mut p));
    }

    #[test]
    fn sanadyanta_folds_nic_into_the_dhatu() {
        let mut p = with_nic("cor", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!((rule("3.1.32").apply)(&mut p));
        assert_eq!(p.terms.len(), ANGA + 1, "the ṇic term is gone");
        assert_eq!(p.terms[ANGA].text, "cori");
        assert!(p.terms[ANGA].has(Tag::Dhatu));
        assert!(p.terms[ANGA].has(Tag::Nijanta));
        let mut p = with_nic("BU", None);
        assert!(!(rule("3.1.32").apply)(&mut p));
        let mut p = with_nic("BU", None);
        p.terms.push(Term::new("ti"));
        assert!(!(rule("3.1.32").apply)(&mut p));
        assert_eq!(p.terms.len(), NIC + 1);
    }
}
```

The two-character (`aw`, `uz`) and one-character (`a`, `u`) roots are hypothetical. They exist to witness the `n < 2` boundary, which no curated root reaches: without them, `<` → `<=` and `<` → `==` survive. The prototype's mutation probe caught all 26 `sanadi.rs` mutants with these tests.

- [ ] **Step 3: Wire the stage**

In `crates/panini-prakriya/src/tinanta/mod.rs`:
- Module doc: replace the first four lines

```rust
//! The tiṅanta pipeline, as eight ordered rule-stage modules plus two support
//! layers.
//!
//! The eight stages — `samjna`, `tin`, `vikarana`, `abhyasa`, `anga`, `guna`, `adesha`,
//! `tripadi` — are declared below in *pipeline* order in `TINANTA_RULES`,
```

  with

```rust
//! The tiṅanta pipeline, as nine ordered rule-stage modules plus two support
//! layers.
//!
//! The nine stages — `sanadi`, `samjna`, `tin`, `vikarana`, `abhyasa`, `anga`, `guna`,
//! `adesha`, `tripadi` — are declared below in *pipeline* order in `TINANTA_RULES`,
```

- Add `mod sanadi;` after `mod samjna;`.
- In `TINANTA_RULES`, add `sanadi::SANADI,` as the first entry, above `samjna::SAMJNA,`.

- [ ] **Step 4: Run the unit tests and see the lookups break**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | grep -E "FAILED" | head -20`

Expected: these eight fail, because `rules().find` now returns the sanādi entry of a duplicated id (the prototype's run):
- `tinanta_rule_order_is_pinned`;
- in `guna.rs`: `pugantalaghupadhasya_applies_guna_when_luk_shap_ending_is_not_ngit`, `pugantalaghupadhasya_single_term_still_applies_guna`, `pugantalaghupadhasya_two_char_ik_penult_fires`, `pugantalaghupadhasya_uses_n_minus_2_not_n_over_2` and `pugantalaghupadhasya_tanadi_arm_is_vikalpa_and_ngit_blind`;
- `it_samjna_rule_reports_when_ending_is_reduced`;
- `ser_hyapic_ca_makes_hi_apit_and_ngit`.

The order pin is the only failure the grammar intends. The rest are tests reaching the wrong entry. Worse, the `guna.rs` tests that still pass, the decline cases, pass by testing the sanādi entry, which also declines. Step 5 fixes all ten `guna.rs` lookup sites, not just the failing ones.

- [ ] **Step 5: Make the stage tests look up stage-locally**

`guna.rs`, inside `mod tests` only:
- replace every `rules().find(|r| r.id == "7.3.86")` with `GUNA.iter().find(|r| r.id == "7.3.86")` (7 sites);
- replace every `rules().filter(|r| r.id == "7.3.86")` with `GUNA.iter().filter(|r| r.id == "7.3.86")` (3 sites).

Leave the `7.3.84` lookups alone: the sanādi stage has no 7.3.84.

`samjna.rs`, in `it_samjna_rule_reports_when_ending_is_reduced`: `rules().find(|r| r.id == "1.3.9")` → `SAMJNA.iter().find(|r| r.id == "1.3.9")`.

`tin.rs`, in `ser_hyapic_ca_makes_hi_apit_and_ngit`, replace

```rust
        for id in ["3.4.78", "1.3.9", "1.2.4", "3.4.85", "3.4.87"] {
            let rule = rules().find(|r| r.id == id).unwrap();
```

with

```rust
        // Looked up in the two stages the chain spans, not in `rules()`:
        // `super::sanadi` runs first and has its own 1.3.9 (ṇic's it-lopa).
        let stages = || super::super::samjna::SAMJNA.iter().chain(TIN.iter());
        for id in ["3.4.78", "1.3.9", "1.2.4", "3.4.85", "3.4.87"] {
            let rule = stages().find(|r| r.id == id).unwrap();
```

Then check that no other stage test still searches globally for a duplicated id:

```bash
grep -rn 'rules().find(|r| r.id == "\(7.3.86\|1.3.9\)")\|rules().filter(|r| r.id == "\(7.3.86\|1.3.9\)")' crates/panini-prakriya/src
```

Expected: no output.

- [ ] **Step 6: Pin the order**

In `derivation_tests.rs`'s `tinanta_rule_order_is_pinned`, change the opening of `expected` from

```rust
        "1.3.12", "1.3.66", "1.3.72", "1.3.74", "1.3.78",
```

(Task 3's state, however fmt wrapped it) to start

```rust
        "3.1.25", "1.3.9", "3.4.114", "7.2.116", "7.3.86", "3.1.32", "1.3.12", "1.3.66", "1.3.72",
        "1.3.74", "1.3.78",
```

with everything after `"1.3.78"` unchanged. In the test's doc comment, replace the paragraph

```rust
/// 7.3.86 also appears twice, for the same reason 7.3.84 and 1.2.4 do: one
/// sūtra id, two distinct occasions. The nitya entry states the gaṇasūtra
/// (never gaṇa 8); the vikalpa entry immediately after it is gaṇa 8's own
/// Kaumudī 2547.1 optional guṇa, kept under the Pāṇinian id. Both entries
/// are real; do not deduplicate.
```

with

```rust
/// 7.3.86 also appears twice in the guṇa stage, for the same reason 7.3.84
/// and 1.2.4 do: one sūtra id, two distinct occasions. The nitya entry
/// states the gaṇasūtra (never gaṇa 8); the vikalpa entry immediately after
/// it is gaṇa 8's own Kaumudī 2547.1 optional guṇa, kept under the Pāṇinian
/// id. Both entries are real; do not deduplicate.
///
/// The sanādi stage opens the list (slice 10a): 3.1.25 adds curādi's ṇic,
/// 1.3.9 strips its anubandhas, 3.4.114 makes it ārdhadhātuka, 7.2.116 and
/// a THIRD 7.3.86 entry act on the root before it, and 3.1.32 folds it into
/// the dhātu. So 1.3.9 appears twice and 7.3.86 three times. A stage
/// test that looks a rule up by id must search its own stage's static
/// (`GUNA`, `SAMJNA`, …), not `rules()`, or it finds the sanādi entry first.
```

- [ ] **Step 7: Run the unit tests and the full suite**

```bash
mise exec -- cargo test -p panini-prakriya 2>&1 | grep -E "FAILED|test result"   # 399 passed (390 at baseline + 3 from Task 3 + 6 here)
mise run test 2>&1 | grep -E "FAILED|test result" | head                         # priors unchanged at 4644
```

The stage is inert on every curated row, because none carries `Tag::Curadi` yet.

- [ ] **Step 8: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/
git commit -m "feat(prakriya): the sanādi stage — 3.1.25 ṇic, its it-lopa, 3.4.114, 7.2.116, 7.3.86, 3.1.32

A new first stage, inert outside curādi. Stage tests now look duplicated ids
(1.3.9, 7.3.86) up in their own stage static, not rules()."
```

---

## Task 5: The rows and their paradigm goldens

This task turns the 288 new cells green.

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Create: `crates/panini/tests/paradigm/data/curadi.rs`
- Modify: `crates/panini/tests/paradigm/data/mod.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`

**Interfaces:**
- Consumes: Tasks 2–4.
- Produces: the `dhatus()` rows `10.0001` (`cur`), `10.0010` (`laq`), `10.0033` (`Bakz`), `10.0255` (`BUz`), each `Gana::Curadi`, `PadaAssignment::Nic`. Task 6 looks them up by number.

- [ ] **Step 1: Add the four `Dhatu` rows**

Append these to the end of `DHATUS`, after the `03.0026` (`gA`) row and before the closing `];` that precedes `pub fn dhatus()`. The upadeśa and artha are verbatim from `data/dhatupatha.tsv` (`10.0001 cura~ steye`, `10.0010 laqa~ upasevAyAm`, `10.0033 Bakza~ adane`, `10.0255 BUza~ alaNkaraRe`).

```rust
    Dhatu {
        // 10.0001 `cura~` steye (√cur). Curādi's eponym: 3.1.25 adds ṇic,
        // 7.3.86 guṇates the laghu upadhā before it (cor-i), and 3.1.32 makes
        // `cori` the dhātu. Ubhayapadī by 1.3.74 ṇicaś ca. Slice 10a.
        dhatupatha: "10.0001",
        code: "cur",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "steye",
    },
    Dhatu {
        // 10.0010 `laqa~` upasevAyAm (√laḍ). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (lAq-i). Ubhayapadī by 1.3.74. Slice
        // 10a.
        dhatupatha: "10.0010",
        code: "laq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "upasevAyAm",
    },
    Dhatu {
        // 10.0033 `Bakza~` adane (√bhakṣ). Guru upadhā: neither 7.3.86 nor
        // 7.2.116 touches it before ṇic. Ubhayapadī by 1.3.74. Slice 10a.
        dhatupatha: "10.0033",
        code: "Bakz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "adane",
    },
    Dhatu {
        // 10.0255 `BUza~` alaNkaraRe (√bhūṣ). Long upadhā, so guru: unchanged
        // before ṇic. Ubhayapadī by 1.3.74. Slice 10a.
        dhatupatha: "10.0255",
        code: "BUz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "alaNkaraRe",
    },
```

Counts in the same file:
- `curated_roots_have_expected_ganas_and_padas`: `assert_eq!(dhatus().len(), 103);` → `107`.
- `Dhatu`'s `pada` doc: replace `` `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 103 `` / `verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and` / ``requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed`` / `exception, asserted explicitly from both sides, the same way` with `` `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 107 `` / `verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and` / ``requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed`` / `exception and the four curādi rows' are 1.3.74's, each asserted` / `explicitly from both sides, the same way`, and `The test covers the 103 roots curated here` → `The test covers the 107 roots curated here`.
- `pada_from_upadesha`'s doc says `66 of the 103 curated roots`. Recount with `grep -c '\\' <(grep -P "^(10\.0001|10\.0010|10\.0033|10\.0255)\t" data/dhatupatha.tsv)`: none of the four carries a `\`, so this becomes `66 of the 107`, and the 45 stands.

- [ ] **Step 2: Add the gaṇa-row test**

Immediately above `#[test]` / `fn juhotyadi_rows_are_the_twenty_six_curated_roots()`, add:

```rust
    #[test]
    fn curadi_rows_are_the_four_curated_roots() {
        // Slice 10a opens gaṇa 10 with four roots that need only ṇic
        // (3.1.25), 3.1.32 and the guṇa/vṛddhi before ṇic: √cur (7.3.86),
        // √laḍ (7.2.116), √bhakṣ and √bhūṣ (neither). None carries a pada
        // marker; all four are ubhayapadī by 1.3.74 ṇicaś ca. The gaṇa is
        // OPEN at 4 of its 509 dhātupāṭha rows.
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
            ]
        );
    }

```

- [ ] **Step 3: The pada-marker exception**

In `curated_pada_agrees_with_upadesha_markers`, immediately after the √bhuj block's closing `continue;` / `}` and before `if derived != d.pada {`, insert:

```rust
            // 1.3.74 ṇicaś ca: a curādi root's ātmanepada comes from the
            // affix 3.1.25 adds, not from any marker of its own, so its
            // upadeśa CORRECTLY derives parasmaipada and the curated column
            // says `Nic`. Asserted both ways, like √bhuj above. A curādi row
            // that DOES carry a marker (the ātmanepadī ākusmīya roots) is a
            // later slice's, and will fail here until that slice decides how
            // 1.3.12 and 1.3.74 meet.
            if d.gana == Gana::Curadi {
                assert_eq!(
                    derived,
                    PadaAssignment::Parasmaipada,
                    "{} {upadesha}: a marked curādi row needs its own pada decision",
                    d.dhatupatha
                );
                assert_eq!(
                    d.pada,
                    PadaAssignment::Nic,
                    "{} is curādi; its pada is 1.3.74's, curated as Nic",
                    d.dhatupatha
                );
                continue;
            }
```

- [ ] **Step 4: Run the data tests**

Run: `mise exec -- cargo test -p panini-data 2>&1 | tail -3`
Expected: PASS (22 tests), including `dhatupatha_numbers_resolve_upstream` (all four resolve uniquely) and `curated_pada_agrees_with_upadesha_markers`.

- [ ] **Step 5: Create the golden file**

Create `crates/panini/tests/paradigm/data/curadi.rs` with exactly this content (`mise run fmt` rewraps the arrays):

```rust
//! curadi's golden rows. See `super` (`data/mod.rs`) for the row
//! contracts and the concatenated `PARADIGM` / `ALTERNATES` statics.

use panini_data::Pada;

use super::{AlternateRow, ParadigmRow};

pub const PARADIGM: &[ParadigmRow] = &[
    ("10.0001", "laT", Pada::Parasmaipada, ["corayati", "corayataH", "corayanti", "corayasi", "corayaTaH", "corayaTa", "corayAmi", "corayAvaH", "corayAmaH"]),
    ("10.0001", "laN", Pada::Parasmaipada, ["acorayad", "acorayatAm", "acorayan", "acorayaH", "acorayatam", "acorayata", "acorayam", "acorayAva", "acorayAma"]),
    ("10.0001", "loT", Pada::Parasmaipada, ["corayatu", "corayatAm", "corayantu", "coraya", "corayatam", "corayata", "corayARi", "corayAva", "corayAma"]),
    ("10.0001", "viDiliN", Pada::Parasmaipada, ["corayed", "corayetAm", "corayeyuH", "corayeH", "corayetam", "corayeta", "corayeyam", "corayeva", "corayema"]),
    ("10.0001", "laT", Pada::Atmanepada, ["corayate", "corayete", "corayante", "corayase", "corayeTe", "corayaDve", "coraye", "corayAvahe", "corayAmahe"]),
    ("10.0001", "laN", Pada::Atmanepada, ["acorayata", "acorayetAm", "acorayanta", "acorayaTAH", "acorayeTAm", "acorayaDvam", "acoraye", "acorayAvahi", "acorayAmahi"]),
    ("10.0001", "loT", Pada::Atmanepada, ["corayatAm", "corayetAm", "corayantAm", "corayasva", "corayeTAm", "corayaDvam", "corayE", "corayAvahE", "corayAmahE"]),
    ("10.0001", "viDiliN", Pada::Atmanepada, ["corayeta", "corayeyAtAm", "corayeran", "corayeTAH", "corayeyATAm", "corayeDvam", "corayeya", "corayevahi", "corayemahi"]),
    ("10.0010", "laT", Pada::Parasmaipada, ["lAqayati", "lAqayataH", "lAqayanti", "lAqayasi", "lAqayaTaH", "lAqayaTa", "lAqayAmi", "lAqayAvaH", "lAqayAmaH"]),
    ("10.0010", "laN", Pada::Parasmaipada, ["alAqayad", "alAqayatAm", "alAqayan", "alAqayaH", "alAqayatam", "alAqayata", "alAqayam", "alAqayAva", "alAqayAma"]),
    ("10.0010", "loT", Pada::Parasmaipada, ["lAqayatu", "lAqayatAm", "lAqayantu", "lAqaya", "lAqayatam", "lAqayata", "lAqayAni", "lAqayAva", "lAqayAma"]),
    ("10.0010", "viDiliN", Pada::Parasmaipada, ["lAqayed", "lAqayetAm", "lAqayeyuH", "lAqayeH", "lAqayetam", "lAqayeta", "lAqayeyam", "lAqayeva", "lAqayema"]),
    ("10.0010", "laT", Pada::Atmanepada, ["lAqayate", "lAqayete", "lAqayante", "lAqayase", "lAqayeTe", "lAqayaDve", "lAqaye", "lAqayAvahe", "lAqayAmahe"]),
    ("10.0010", "laN", Pada::Atmanepada, ["alAqayata", "alAqayetAm", "alAqayanta", "alAqayaTAH", "alAqayeTAm", "alAqayaDvam", "alAqaye", "alAqayAvahi", "alAqayAmahi"]),
    ("10.0010", "loT", Pada::Atmanepada, ["lAqayatAm", "lAqayetAm", "lAqayantAm", "lAqayasva", "lAqayeTAm", "lAqayaDvam", "lAqayE", "lAqayAvahE", "lAqayAmahE"]),
    ("10.0010", "viDiliN", Pada::Atmanepada, ["lAqayeta", "lAqayeyAtAm", "lAqayeran", "lAqayeTAH", "lAqayeyATAm", "lAqayeDvam", "lAqayeya", "lAqayevahi", "lAqayemahi"]),
    ("10.0033", "laT", Pada::Parasmaipada, ["Bakzayati", "BakzayataH", "Bakzayanti", "Bakzayasi", "BakzayaTaH", "BakzayaTa", "BakzayAmi", "BakzayAvaH", "BakzayAmaH"]),
    ("10.0033", "laN", Pada::Parasmaipada, ["aBakzayad", "aBakzayatAm", "aBakzayan", "aBakzayaH", "aBakzayatam", "aBakzayata", "aBakzayam", "aBakzayAva", "aBakzayAma"]),
    ("10.0033", "loT", Pada::Parasmaipada, ["Bakzayatu", "BakzayatAm", "Bakzayantu", "Bakzaya", "Bakzayatam", "Bakzayata", "BakzayARi", "BakzayAva", "BakzayAma"]),
    ("10.0033", "viDiliN", Pada::Parasmaipada, ["Bakzayed", "BakzayetAm", "BakzayeyuH", "BakzayeH", "Bakzayetam", "Bakzayeta", "Bakzayeyam", "Bakzayeva", "Bakzayema"]),
    ("10.0033", "laT", Pada::Atmanepada, ["Bakzayate", "Bakzayete", "Bakzayante", "Bakzayase", "BakzayeTe", "BakzayaDve", "Bakzaye", "BakzayAvahe", "BakzayAmahe"]),
    ("10.0033", "laN", Pada::Atmanepada, ["aBakzayata", "aBakzayetAm", "aBakzayanta", "aBakzayaTAH", "aBakzayeTAm", "aBakzayaDvam", "aBakzaye", "aBakzayAvahi", "aBakzayAmahi"]),
    ("10.0033", "loT", Pada::Atmanepada, ["BakzayatAm", "BakzayetAm", "BakzayantAm", "Bakzayasva", "BakzayeTAm", "BakzayaDvam", "BakzayE", "BakzayAvahE", "BakzayAmahE"]),
    ("10.0033", "viDiliN", Pada::Atmanepada, ["Bakzayeta", "BakzayeyAtAm", "Bakzayeran", "BakzayeTAH", "BakzayeyATAm", "BakzayeDvam", "Bakzayeya", "Bakzayevahi", "Bakzayemahi"]),
    ("10.0255", "laT", Pada::Parasmaipada, ["BUzayati", "BUzayataH", "BUzayanti", "BUzayasi", "BUzayaTaH", "BUzayaTa", "BUzayAmi", "BUzayAvaH", "BUzayAmaH"]),
    ("10.0255", "laN", Pada::Parasmaipada, ["aBUzayad", "aBUzayatAm", "aBUzayan", "aBUzayaH", "aBUzayatam", "aBUzayata", "aBUzayam", "aBUzayAva", "aBUzayAma"]),
    ("10.0255", "loT", Pada::Parasmaipada, ["BUzayatu", "BUzayatAm", "BUzayantu", "BUzaya", "BUzayatam", "BUzayata", "BUzayARi", "BUzayAva", "BUzayAma"]),
    ("10.0255", "viDiliN", Pada::Parasmaipada, ["BUzayed", "BUzayetAm", "BUzayeyuH", "BUzayeH", "BUzayetam", "BUzayeta", "BUzayeyam", "BUzayeva", "BUzayema"]),
    ("10.0255", "laT", Pada::Atmanepada, ["BUzayate", "BUzayete", "BUzayante", "BUzayase", "BUzayeTe", "BUzayaDve", "BUzaye", "BUzayAvahe", "BUzayAmahe"]),
    ("10.0255", "laN", Pada::Atmanepada, ["aBUzayata", "aBUzayetAm", "aBUzayanta", "aBUzayaTAH", "aBUzayeTAm", "aBUzayaDvam", "aBUzaye", "aBUzayAvahi", "aBUzayAmahi"]),
    ("10.0255", "loT", Pada::Atmanepada, ["BUzayatAm", "BUzayetAm", "BUzayantAm", "BUzayasva", "BUzayeTAm", "BUzayaDvam", "BUzayE", "BUzayAvahE", "BUzayAmahE"]),
    ("10.0255", "viDiliN", Pada::Atmanepada, ["BUzayeta", "BUzayeyAtAm", "BUzayeran", "BUzayeTAH", "BUzayeyATAm", "BUzayeDvam", "BUzayeya", "BUzayevahi", "BUzayemahi"]),
];

pub const ALTERNATES: &[AlternateRow] = &[
    ("10.0001", "laN", Pada::Parasmaipada, 0, "acorayat", "7.3.86+8.4.56"),
    ("10.0001", "loT", Pada::Parasmaipada, 0, "corayatAd", "7.3.86+7.1.35"),
    ("10.0001", "loT", Pada::Parasmaipada, 0, "corayatAt", "7.3.86+7.1.35+8.4.56"),
    ("10.0001", "loT", Pada::Parasmaipada, 3, "corayatAd", "7.3.86+7.1.35"),
    ("10.0001", "loT", Pada::Parasmaipada, 3, "corayatAt", "7.3.86+7.1.35+8.4.56"),
    ("10.0001", "viDiliN", Pada::Parasmaipada, 0, "corayet", "7.3.86+8.4.56"),
    ("10.0010", "laN", Pada::Parasmaipada, 0, "alAqayat", "8.4.56"),
    ("10.0010", "loT", Pada::Parasmaipada, 0, "lAqayatAd", "7.1.35"),
    ("10.0010", "loT", Pada::Parasmaipada, 0, "lAqayatAt", "7.1.35+8.4.56"),
    ("10.0010", "loT", Pada::Parasmaipada, 3, "lAqayatAd", "7.1.35"),
    ("10.0010", "loT", Pada::Parasmaipada, 3, "lAqayatAt", "7.1.35+8.4.56"),
    ("10.0010", "viDiliN", Pada::Parasmaipada, 0, "lAqayet", "8.4.56"),
    ("10.0033", "laN", Pada::Parasmaipada, 0, "aBakzayat", "8.4.56"),
    ("10.0033", "loT", Pada::Parasmaipada, 0, "BakzayatAd", "7.1.35"),
    ("10.0033", "loT", Pada::Parasmaipada, 0, "BakzayatAt", "7.1.35+8.4.56"),
    ("10.0033", "loT", Pada::Parasmaipada, 3, "BakzayatAd", "7.1.35"),
    ("10.0033", "loT", Pada::Parasmaipada, 3, "BakzayatAt", "7.1.35+8.4.56"),
    ("10.0033", "viDiliN", Pada::Parasmaipada, 0, "Bakzayet", "8.4.56"),
    ("10.0255", "laN", Pada::Parasmaipada, 0, "aBUzayat", "8.4.56"),
    ("10.0255", "loT", Pada::Parasmaipada, 0, "BUzayatAd", "7.1.35"),
    ("10.0255", "loT", Pada::Parasmaipada, 0, "BUzayatAt", "7.1.35+8.4.56"),
    ("10.0255", "loT", Pada::Parasmaipada, 3, "BUzayatAd", "7.1.35"),
    ("10.0255", "loT", Pada::Parasmaipada, 3, "BUzayatAt", "7.1.35+8.4.56"),
    ("10.0255", "viDiliN", Pada::Parasmaipada, 0, "BUzayet", "8.4.56"),
];
```

That is 24 rows over 16 cells, so 288 + 24 = 312 forms, which is vidyut's count. **√cur's keys open with 7.3.86.** That is its mandatory sanādi guṇa, which `VIKALPA_RULES` cannot tell apart from tanādi's optional entry because they share the id. It sits *ahead of* 7.1.35 because the sanādi stage runs first. The other three roots key exactly like √bhū.

- [ ] **Step 6: Register the file**

In `crates/panini/tests/paradigm/data/mod.rs`:
- module doc: `//! kryādi. Row order` → `//! kryādi, 10 curādi. Row order`;
- add `pub mod curadi;` after `pub mod bhvadi;`;
- add `curadi::PARADIGM,` after `juhotyadi::PARADIGM,` in the `PARADIGM` concat, and `curadi::ALTERNATES,` after `juhotyadi::ALTERNATES,` in the `ALTERNATES` concat.

- [ ] **Step 7: Update `paradigm/main.rs`'s counts and census**

In `derivation_set_shape_matches_the_audited_numbers`:
- `assert_eq!(total_cells, 4644, "516 root×lakāra blocks × 9 cells each");` → `assert_eq!(total_cells, 4932, "548 root×lakāra blocks × 9 cells each");`
- `ones` 3836 → `4108`; `twos` 604 → `612`; `threes` 157 → `165`. `fours`…`sevens` are unchanged.
- The `threes` message ends ``and its two loṭ tātaṅ cells; and — new in slice 3f3 — √jan's two loṭ tātaṅ cells, \`` / `` by 7.1.35/8.4.56"``. Replace that ending with ``and its two loṭ tātaṅ cells; and — new in slice 3f3 — √jan's two loṭ tātaṅ cells; \`` / ``and — new in slice 10a — the four curādi roots' two loṭ tātaṅ cells each, by \`` / ``7.1.35/8.4.56"``.
- `ALTERNATES.len()` 1106 → `1130`.
- `key_count("8.4.56")` 164 → `170`; `key_count("7.1.35")` 156 → `162`; `key_count("7.1.35+8.4.56")` 156 → `162`; `key_count("7.3.86+8.4.56")` 17 → `19`.
- After the final `key_count("6.4.43+8.4.56")` assertion, add:

```rust
    assert_eq!(key_count("7.3.86+7.1.35"), 2, "7.3.86+7.1.35 alternates");
    assert_eq!(
        key_count("7.3.86+7.1.35+8.4.56"),
        2,
        "7.3.86+7.1.35+8.4.56 alternates"
    );
```

Checks:
- Cells: 4108 + 612 + 165 + 19 + 10 + 17 + 1 = 4932.
- Forms: 4932 + 1130 = 6062.
- The key census stays exhaustive: 1106 + 6 + 6 + 6 + 2 + 2 + 2 = 1130.

Doc comments:
- Above `every_alternate_names_the_vikalpa_rules_that_produced_it`: `` `ALTERNATES` is otherwise 1106 bare strings, `` → ``1130``. In the `VIKALPA_RULES` paragraph below it, replace ``Ten of the 17 `7.3.86+8.4.56` keys are the mandatory firing (3e's laṅ eka cells and 3f's √kit and √dhiṣ ones), and so is the 7.3.86 of the one `7.3.86+8.2.75` key (√kit's `acikeH`).`` with ``Twelve of the 19 `7.3.86+8.4.56` keys are the mandatory firing (3e's laṅ eka cells, 3f's √kit and √dhiṣ ones, and slice 10a's √cur laṅ and vidhiliṅ prathama eka, where the firing is the sanādi entry before ṇic), and so is the 7.3.86 of the one `7.3.86+8.2.75` key (√kit's `acikeH`) and of the four `7.3.86+7.1.35`/`7.3.86+7.1.35+8.4.56` keys (√cur's loṭ tātaṅ cells — the only keys where 7.3.86 precedes 7.1.35, because the sanādi stage runs first).``
- Above `derivation_set_shape_matches_the_audited_numbers`:
  - `4644 cells total (516 root×lakāra blocks × 9), of which 3836 hold exactly one form, 604 hold two, 157 hold three (` → `4932 cells total (548 root×lakāra blocks × 9), of which 4108 hold exactly one form, 612 hold two, 165 hold three (`
  - `and √bhas's, new in slice 3f2, and √jan's, new in slice 3f3, each by` → `and √bhas's, new in slice 3f2, and √jan's, new in slice 3f3, and the four curādi roots', new in slice 10a, each by`
  - ``itself has 1106 rows, keyed 164 `8.4.56`, 156 `7.1.35`, 156 `7.1.35+8.4.56`,`` → ``itself has 1130 rows, keyed 170 `8.4.56`, 162 `7.1.35`, 162 `7.1.35+8.4.56`,``
  - ``17 `7.3.86+8.4.56` (ten of them name the MANDATORY`` → ``19 `7.3.86+8.4.56` (twelve of them name the MANDATORY``. Append ``, and slice 10a's √cur laṅ and vidhiliṅ prathama eka, whose guṇa before ṇic the sanādi 7.3.86 credits`` after ``whose root guṇa 7.3.86 credits``, inside the same parenthesis.
  - ``9 `6.4.43` and 1 `6.4.43+8.4.56` (slice 3f3's √jan) — √kṛ`` → ``9 `6.4.43` and 1 `6.4.43+8.4.56` (slice 3f3's √jan), 2 `7.3.86+7.1.35` and 2`` / ``/// `7.3.86+7.1.35+8.4.56` (slice 10a's √cur, its sanādi 7.3.86 ahead of 7.1.35) — √kṛ``
  - After the slice-3f3 paragraph (ending `three ways on 7.1.35/8.4.56. Fifteen new rows. The gaṇa is COMPLETE.`), add:

```rust
///
/// Slice 10a opens curādi (gaṇa 10) with √cur, √laḍ, √bhakṣ and √bhūṣ
/// (`10.0001`, `10.0010`, `10.0033`, `10.0255`), ubhayapadī by 1.3.74. No new
/// vikalpa rule: each root's parasmaipada forks only where every thematic
/// root does — laṅ and vidhiliṅ prathama eka on 8.4.56, the two loṭ tātaṅ
/// cells three ways on 7.1.35/8.4.56. √cur's keys carry its mandatory
/// sanādi 7.3.86 in front, opening `7.3.86+7.1.35` and
/// `7.3.86+7.1.35+8.4.56`. Twenty-four new rows. The gaṇa is OPEN at 4 of
/// its 509 rows.
```

- [ ] **Step 8: The pada-ambiguous surfaces**

The four roots are thematic and ubhayapadī, so each contributes √nī's four-surface shape. In `pada_ambiguous_surfaces_are_exactly_these`, after the comment line `// Measured: no 3e form equals any pre-slice form in either pada.`, add:

```rust
    // Slice 10a's four curādi roots, ubhayapadī by 1.3.74, are thematic like
    // √nī and √tud and contribute the same four-surface shape each: laṅ
    // ātmanepada prathama eka = parasmaipada madhyama bahu (`acorayata`), loṭ
    // ātmanepada prathama eka = parasmaipada prathama dvi (`corayatAm`), and
    // the two vidhiliṅ ones (`corayetAm`, `corayeta`) — sixteen more, taking
    // the set from fifty-six to seventy-two, with no new collision against
    // any pre-slice surface.
```

and replace the expected vector with these 72, in this order (`mise run fmt` packs them):

```rust
            "ArRuta", "BUzayatAm", "BUzayetAm", "BUzayeta", "BakzayatAm", "BakzayetAm",
            "Bakzayeta", "BinttAm", "BuNktAm", "CfnttAm", "CinttAm", "DattAm", "GfRutAm",
            "aBUzayata", "aBakzayata", "aBintta", "aBuNkta", "aDatta", "aGfRuta", "abiBfta",
            "acCfntta", "acCintta", "acorayata", "adatta", "akuruta", "akzaRuta", "akziRuta",
            "akzuntta", "alAqayata", "anayata", "anenikta", "ariNkta", "arundDa", "asanuta",
            "atanuta", "atfRuta", "atfntta", "atudata", "avevikta", "avevizwa", "aviNkta",
            "ayuNkta", "biBftAm", "corayatAm", "corayetAm", "corayeta", "dattAm", "fRutAm",
            "kurutAm", "kzaRutAm", "kziRutAm", "kzunttAm", "lAqayatAm", "lAqayetAm", "lAqayeta",
            "nayatAm", "nayetAm", "nayeta", "neniktAm", "riNktAm", "rundDAm", "sanutAm",
            "tanutAm", "tfRutAm", "tfnttAm", "tudatAm", "tudetAm", "tudeta", "veviktAm",
            "vevizwAm", "viNktAm", "yuNktAm",
```

- [ ] **Step 9: Add the `check()` test**

Run `grep -rn '"corayati"\|"corayate"\|"lAqayati"\|"BakzayARi"\|"aBUzayat"' crates/panini/tests/paradigm/data/ | grep -v curadi.rs` first. It must print nothing. Then, at the end of `paradigm/main.rs`, add:

```rust

/// Slice 10a's rows through `check`. None of these surfaces is a prior row's,
/// so every analysis must name the curādi root and carry the sanādi rule that
/// shaped its stem — and `corayate`, ātmanepada only, must carry 1.3.74.
#[test]
fn curadi_analyses_its_nijanta_forms() {
    let engine = Panini::new();
    for (form, dhatu, sutra) in [
        ("corayati", "cur", "7.3.86"),
        ("corayate", "cur", "1.3.74"),
        ("lAqayati", "laq", "7.2.116"),
        ("BakzayARi", "Bakz", "3.1.32"),
        ("aBUzayat", "BUz", "3.1.25"),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert!(!r.analyses.is_empty(), "{form}");
        for a in &r.analyses {
            assert_eq!(a.dhatu, dhatu, "{form}");
            assert!(
                a.trace.iter().any(|s| s.sutra == sutra),
                "{form}: {:?}",
                a.trace
            );
        }
    }
}
```

- [ ] **Step 10: Run the full suite**

Run: `mise run test 2>&1 | grep -E "FAILED|panicked|test result" | head -20` (foreground, timeout 600000 ms)
Expected: PASS at 4932 cells, with the 4644 priors unchanged.

If a curādi cell fails, read it against the spec before touching anything, using superpowers:systematic-debugging. **Do not edit a golden to match the engine.** Likely causes by symptom:
- `curati` / `Bakzati` (no `-ay-`): 3.1.25 never fired. Check `derive`'s `Gana::Curadi` arm and the stage's place in `TINANTA_RULES`.
- `curayati` (no guṇa): the sanādi 7.3.86 declined. Check that 3.4.114 tags `Ardhadhatuka`.
- `laqayati`: 7.2.116 declined. Check that 1.3.9 tags `Rit`.
- a panic indexing `terms[4]`, or the ending at the wrong index: 3.1.32 did not remove the ṇic term.
- an ātmanepada cell blocked (missing from the derivation set): 1.3.78's `Tag::Nic` arm.

- [ ] **Step 11: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-data/src/lib.rs crates/panini/tests/paradigm/
git commit -m "feat(data): curādi opens — √cur, √laḍ, √bhakṣ, √bhūṣ (10.0001/0010/0033/0255), ubhayapadī by 1.3.74

4644 → 4932 cells, 5750 → 6062 forms, 1106 → 1130 ALTERNATES, 103 → 107 roots."
```

---

## Task 6: The trace pins and the fires-only tests

**Files:**
- Modify: `crates/panini/tests/trace/helpers.rs`, `juhotyadi.rs`, `main.rs`
- Create: `crates/panini/tests/trace/curadi.rs`

**Interfaces:**
- Consumes: `crate::helpers::{at, cell_trace}` (existing). `cell_trace(number, lakara, pada, purusha, vacana) -> (String, Vec<String>)` returns branch 0's text and trace. `at(&[String], &str) -> usize` panics if the sūtra is absent.
- Produces: `pub fn credited(sutra: &str) -> Vec<(&'static str, Gana)>` in `helpers.rs`, moved from `juhotyadi.rs` with its `ALL_CELLS`. It returns one entry per non-blocked branch, over every curated root, lakāra, pada and cell, whose log carries `sutra`.

- [ ] **Step 1: Move `credited` into the helpers**

In `crates/panini/tests/trace/juhotyadi.rs`, cut the `const ALL_CELLS: [(Purusha, Vacana); 9] = [ … ];` block and the `credited` function together with its `///` doc comment. They are adjacent, just above `fn das_ca_is_credited_only_on_rudhadi_and_kit`. Change that file's import to `use crate::helpers::{at, cell_trace, credited};`.

Paste both at the end of `crates/panini/tests/trace/helpers.rs`, making the function `pub fn credited`, and change its import to `use panini_data::{Gana, Lakara, Pada, Purusha, Vacana, dhatus};`.

Run: `mise exec -- cargo test -p panini --test trace 2>&1 | grep -E "warning|error|test result"`
Expected: `184 passed`, no warnings. If `Gana` or `Purusha` is now unused in `juhotyadi.rs`, the warning names it; drop it from that file's import.

- [ ] **Step 2: Add the curādi trace module**

In `crates/panini/tests/trace/main.rs`, add `mod curadi;` after `mod bhvadi;`, and change `then its eight stage files in that order` to `then its nine stage files in that order`.

Create `crates/panini/tests/trace/curadi.rs`:

```rust
//! curadi's ordered-trace witnesses. Helpers live in `crate::helpers`; the
//! module doc governing this suite is in `main.rs`.
//!
//! Every curādi trace opens with the sanādi stage — 3.1.25 ṇic, its 1.3.9,
//! 3.4.114, then 7.2.116 or 7.3.86 where the root's upadhā takes one, then
//! 3.1.32 — before the pada sūtra, where every other gaṇa's trace opens.

use crate::helpers::{at, cell_trace, credited};
use panini_data::{Lakara, Pada, Purusha, Vacana};

#[test]
fn corayati_trace_is_nic_guna_sanadyanta_then_the_thematic_core() {
    // cur P laT P.E. 7.3.86 guṇates `cur` before ṇic, 3.1.32 makes `cori`
    // the dhātu, and only then does the pada sūtra run. 7.3.84 guṇates ṇic's
    // `i` before śap and 6.1.78 makes it `ay`.
    let (text, t) = cell_trace(
        "10.0001",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "corayati", "got {t:?}");
    assert_eq!(
        t,
        [
            "3.1.25", "1.3.9", "3.4.114", "7.3.86", "3.1.32", "1.3.78", "3.4.78", "1.3.9",
            "3.1.68", "1.3.9", "7.3.84", "6.1.78",
        ],
    );
}

#[test]
fn corayate_trace_credits_nicas_ca() {
    // cur A laT P.E. 1.3.74 sanctions the ātmanepada, where 1.3.78 stands in
    // corayati; neither 1.3.72 nor 1.3.78 is credited.
    let (text, t) = cell_trace(
        "10.0001",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "corayate", "got {t:?}");
    assert!(at(&t, "3.1.32") < at(&t, "1.3.74"), "got {t:?}");
    assert!(at(&t, "1.3.74") < at(&t, "3.4.78"), "got {t:?}");
    for absent in ["1.3.72", "1.3.78", "1.3.12"] {
        assert!(!t.contains(&absent.to_string()), "{absent}: got {t:?}");
    }
}

#[test]
#[allow(non_snake_case)]
fn lAqayati_trace_takes_ata_upadhayah_not_guna() {
    // laq P laT P.E. The `a` upadhā takes vṛddhi before ṇit ṇic; 7.3.86 has
    // nothing to guṇate.
    let (text, t) = cell_trace(
        "10.0010",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "lAqayati", "got {t:?}");
    assert!(at(&t, "3.4.114") < at(&t, "7.2.116"), "got {t:?}");
    assert!(at(&t, "7.2.116") < at(&t, "3.1.32"), "got {t:?}");
    assert!(!t.contains(&"7.3.86".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn Bakzayati_and_BUzayati_traces_touch_no_upadha() {
    // Bakz and BUz have guru upadhās: ṇic goes on unchanged roots.
    for (number, form) in [("10.0033", "Bakzayati"), ("10.0255", "BUzayati")] {
        let (text, t) = cell_trace(
            number,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert_eq!(text, form, "got {t:?}");
        assert!(at(&t, "3.4.114") < at(&t, "3.1.32"), "got {t:?}");
        for absent in ["7.2.116", "7.3.86"] {
            assert!(!t.contains(&absent.to_string()), "{form} {absent}: got {t:?}");
        }
    }
}

#[test]
#[allow(non_snake_case)]
fn BakzayARi_trace_takes_natva_across_the_stem() {
    // Bakz P loT U.E. The ṣ of the root reaches the ending's `n` across
    // `ayA` (8.4.2); √laḍ, with no trigger, keeps lAqayAni.
    let (text, t) = cell_trace(
        "10.0033",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "BakzayARi", "got {t:?}");
    assert!(at(&t, "6.1.101") < at(&t, "8.4.2"), "got {t:?}");
    let (text, t) = cell_trace(
        "10.0010",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "lAqayAni", "got {t:?}");
    assert!(!t.contains(&"8.4.2".to_string()), "got {t:?}");
}

#[test]
fn acorayad_trace_puts_the_augment_on_the_merged_anga() {
    // cur P laN P.E. The aṭ comes after 3.1.32, in front of the ṇijanta.
    let (text, t) = cell_trace(
        "10.0001",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "acorayad", "got {t:?}");
    assert!(at(&t, "3.1.32") < at(&t, "6.4.71"), "got {t:?}");
    assert!(at(&t, "6.4.71") < at(&t, "7.3.84"), "got {t:?}");
}

#[test]
fn the_sanadi_rules_and_nicas_ca_are_credited_only_on_curadi() {
    // Every curated branch whose log carries one of these belongs to a
    // `10.x` row. Goldens ignore traces, so this is what holds the new
    // stage inert on the 103 prior roots.
    for sutra in ["3.1.25", "3.4.114", "7.2.116", "3.1.32", "1.3.74"] {
        let hits = credited(sutra);
        assert!(!hits.is_empty(), "curādi no longer witnesses {sutra}");
        for (number, _) in &hits {
            assert!(number.starts_with("10."), "{sutra} credited on {number}");
        }
    }
}

#[test]
fn pugantalaghupadhasya_off_curadi_is_credited_exactly_as_before_10a() {
    // 7.3.86 gains a third entry in slice 10a, the sanādi one. A trace step
    // carries no entry, so "the guṇa-stage entries are untouched" is held as
    // the credit count off curādi: 384 branches, measured on `main` before
    // the slice. A new firing on a prior row, from any entry, changes it.
    let hits = credited("7.3.86");
    assert!(
        hits.iter().any(|(n, _)| *n == "10.0001"),
        "√cur no longer witnesses 7.3.86"
    );
    let off = hits.iter().filter(|(n, _)| !n.starts_with("10.")).count();
    assert_eq!(off, 384);
}
```

`384` was measured on `main` at `4dd7b53` from a dump of every curated cell's live branches, the same count `credited()` takes. Task 7 Step 2 re-measures it.

- [ ] **Step 3: Run the suite**

Run: `mise run test 2>&1 | grep -E "FAILED|panicked|test result" | head -20` (foreground, timeout 600000 ms)
Expected: PASS (192 trace tests).
- If a pin fails on ORDER, report the engine's order; do not "fix" it toward vidyut's. vidyut credits 1.4.13 / 1.4.14 and the individual it rules, which this engine does not record.
- If a fires-only test names a non-`10.x` root, or the 7.3.86 count is not 384, stop and report it: that is the new code over-firing on a prior row.

- [ ] **Step 4: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini/tests/trace/
git commit -m "test(trace): 10a pins — corayati, corayate, lAqayati, Bakzayati/BUzayati, BakzayARi, acorayad; sanādi rules and 1.3.74 fire only on curādi, 7.3.86 off curādi pinned at 384"
```

---

## Task 7: Audit, prior-trace diff, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (audit prose), `docs/superpowers/specs/2026-10-01-juhotyadi-gana-3f3-design.md` (one pointer)

**Interfaces:**
- Consumes: the finished engine and goldens (Tasks 2–6). Produces no symbols.

- [ ] **Step 1: Update the audit harness**

In `tools/audit/panini_full_audit.rs`:
- In `gana_name`, add `PGana::Curadi => "Curadi",` after `PGana::Juhotyadi => "Juhotyadi",`. `gana_of_number` already maps `"10"`.
- `for each of the 103 curated roots` → `107`.
- `Corpus invariants, asserted: 103 roots, 4644 cells, 5750 forms.` → `107 roots, 4932 cells, 6062 forms.`
- `` 516 root×pada×lakāra `` / `` blocks × 9 cells, plus 1106 `ALTERNATES` rows. `` → `548 …` / `plus 1130`.
- `the full 4644-cell table` → `4932-cell`.
- `assert_eq!(roots_seen.len(), 103, "curated roots");` → `107`.
- `assert_eq!(n_cells, 4644, "cells: 516 root×pada×lakāra blocks × 9");` → `4932`, `"cells: 548 root×pada×lakāra blocks × 9"`.
- `assert_eq!(n_forms, 5750, "forms: 4644 cells + 1106 ALTERNATES rows");` → `6062`, `"forms: 4932 cells + 1130 ALTERNATES rows"`.

- The both-pada clause: `` //! admits (two apiece for the twenty-six roots that admit both padas — `` / `//! twenty-five ubhayapadī by 1.3.72, plus √bhuj by 1.3.66), for each of the four` → `` //! admits (two apiece for the thirty roots that admit both padas — `` / `//! twenty-five ubhayapadī by 1.3.72, √bhuj by 1.3.66, and four curādi roots by` / `//! 1.3.74), for each of the four`.

In `tools/audit/README.md`, `(103 roots, 4644 cells, 5750 forms)` → `(107 roots, 4932 cells, 6062 forms)`.

- [ ] **Step 2: Repoint vidyut's dev-deps at THIS worktree, run the prior-trace diff and the audit**

`/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes absolute dev-dep paths to `/workspace/crates`, the `main` checkout, which does not have this slice. Auditing without repointing checks the pre-slice engine and passes vacuously.

The throwaway example `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_10a.rs` exists from the plan's prototype. If it is missing, recreate it:

```rust
//! THROWAWAY: slice 10a — dump every non-curādi cell's live-branch credited-rule log.
use panini::Panini;
use panini_data::{Lakara as L, Purusha as P, Vacana as V};
fn main() {
    let panini = Panini::new();
    for d in panini_data::dhatus() {
        if d.dhatupatha.starts_with("10.") { continue; }
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
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # must point at /workspace/crates
(cd /tmp/vidyut-full/vidyut-prakriya && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10a > "$DUMP/main.txt")
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # must point at $WT/crates
(cd /tmp/vidyut-full/vidyut-prakriya && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_10a > "$DUMP/branch.txt")
wc -l "$DUMP/main.txt"                                          # 5750
diff "$DUMP/main.txt" "$DUMP/branch.txt" && echo PRIORS-IDENTICAL
grep -cE ' 7\.3\.86( |$)' "$DUMP/main.txt"                       # 384, the Task 6 pin
```

Expected: `PRIORS-IDENTICAL`, and `384`. If the diff shows any line, stop and report it: a prior cell's trace was changed by the slice.

Then the audit. Copy the committed harness; never rewrite it.

```bash
cp tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -15)
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -8)
```

- Expected from the honest run: `AUDIT PASSED: 4932 cells, 6062 forms, zero differences.` (as in the prototype).
- Expected from the `entry` control: `AUDIT FAILED: 36 differing cells.` (the √bhū cells).

Do not use `mise -C`: it changes cargo's working directory, and cargo then looks for the example in the wrong package. If the honest run shows differences, stop and report, and edit nothing. After all runs, restore the dev-deps:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"/workspace/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"/workspace/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
```

- [ ] **Step 3: Record the audit**

In `tools/audit/README.md`, immediately under `## Last recorded result` and its blank line, add a new entry above the 3f3 one, with `<DATE>` = `date -u +%F`:

```markdown
<DATE>, curādi 10a slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 4932
cells / 6062 forms / 107 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole curādi 10a slice: the first gaṇa-10 rows, √cur,
√laḍ, √bhakṣ and √bhūṣ (`10.0001`, `10.0010`, `10.0033`, `10.0255`), ubhayapadī
by 1.3.74 *ṇicaś ca*, behind a new first pipeline stage — 3.1.25 ṇic, its
it-lopa, 3.4.114, 7.2.116 *ata upadhāyāḥ* (*lāḍayati*), a pre-ṇic 7.3.86
(*corayati*) and 3.1.32 *sanādyantā dhātavaḥ*. A main-vs-branch dump of every
prior cell's traces was byte-identical.

Totals: 107 = 103 + 4; 4932 = 4644 + 288 (32 root×pada×lakāra blocks × 9);
6062 = 5750 + 288 + 24 new `ALTERNATES` rows (1106 → 1130), measured via the
harness's corpus block, not assumed.

```

In `crates/panini/tests/paradigm/main.rs`'s audit-chain doc comment, find the sentence ending `and juhotyādi 3f3's re-ran it at` / `the same commit over all 4644 cells / 5750 forms / 103 roots with zero` / `differences, its \`entry\` negative control verified failing (36 √bhū` / `cells).` (`grep -n "juhotyādi 3f3's re-ran" crates/panini/tests/paradigm/main.rs`). Replace its final `cells).` with `cells), and curādi 10a's re-ran it at the same commit over all 4932 cells /` / `/// 6062 forms / 107 roots with zero differences, its \`entry\` negative` / `/// control verified failing (36 √bhū cells).`

- [ ] **Step 4: README.md**

Each `old` → `new` below occurs exactly once. Confirm with `grep -c` before editing.
- `Finite verbs (*tiṅanta*), nine gaṇas covered, all nine fully —` → `Finite verbs (*tiṅanta*), ten gaṇas covered, nine of them fully —`
- Replace `8.4.44 *śāt* exemption. rudhādi is` with:

```markdown
8.4.44 *śāt* exemption. *curādi* (10) is **open** at 4 of its 509
dhātupāṭha rows: √cur (`10.0001`, *corayati*), √laḍ (`10.0010`,
*lāḍayati*), √bhakṣ (`10.0033`) and √bhūṣ (`10.0255`), curated in slice 10a,
all ubhayapadī by 1.3.74 *ṇicaś ca*. Every curādi root takes ṇic (3.1.25)
before the vikaraṇa; a new first pipeline stage adds it, guṇates or
lengthens the root before it (7.3.86, 7.2.116 *ata upadhāyāḥ*), and folds it
into the dhātu by 3.1.32 *sanādyantā dhātavaḥ*, so √cur's stem is `cori` and
derives on through 7.3.84 and 6.1.78 like √nī's. rudhādi is
```

- `curated 103-root set` → `curated 107-root set`
- `808 of the 4644 cells hold more than one form: 604` / `hold two, 157 hold three (` → `824 of the 4932 cells hold more than one form: 612` / `hold two, 165 hold three (`
- `and √dhan's, and — new in slice 3f2 — √bhas's, and — new in slice 3f3 —` / `√jan's, each by 7.1.35/8.4.56,` → `and √dhan's, and — new in slice 3f2 — √bhas's, and — new in slice 3f3 —` / `√jan's, and — new in slice 10a — the four curādi roots', each by 7.1.35/8.4.56,`
- `padas — twenty-six roots that admit both padas in the curated set` → `padas — thirty roots that admit both padas in the curated set`
- `√kṛ, √dā, √dhā, √bhṛ, √ṇij, √vij and √viṣ; and √bhuj by 1.3.66) derive a full` → `√kṛ, √dā, √dhā, √bhṛ, √ṇij, √vij and √viṣ; √bhuj by 1.3.66; and curādi's √cur,` / `√laḍ, √bhakṣ and √bhūṣ by 1.3.74) derive a full`
- `Fifty-six surfaces are pada-ambiguous,` → `Seventy-two surfaces are pada-ambiguous,`. Then rebuild the backticked list after it from the 72-entry vector in Task 5 Step 8, in the same order, keeping the `` `viNktAm` and `` / `` `yuNktAm` `` ending. After `` `anenikta`/`neniktAm`, `avevikta`/`veviktAm` and `avevizwa`/`vevizwAm`. `` add: `` Slice 10a's four curādi roots, thematic like √nī, contribute √nī's four-surface shape each: `acorayata`/`corayatAm`/`corayetAm`/`corayeta` and the same for `lAqaya-`, `Bakzaya-` and `BUzaya-`. ``

Check: 612 + 165 + 19 + 10 + 17 + 1 = 824.

- [ ] **Step 5: docs/ARCHITECTURE.md**

Each `old` → `new` occurs exactly once.
- ``ordered `&[&[Rule]]` — eight pipeline stages, each in its own file — covering`` → ``ordered `&[&[Rule]]` — nine pipeline stages, each in its own file — covering``
- Stage table: insert as the first data row, above the `samjna.rs` row:

```markdown
| `sanadi.rs` | 3.1.25, 1.3.9, 3.4.114, 7.2.116, 7.3.86, 3.1.32 — ṇic and its folding into the dhātu (curādi only) | before 3.1.68, before any tiṅ |
```

  and in the `samjna.rs` row `| 1.3.12, 1.3.72, 1.3.78, 3.4.78, 1.3.9, 1.2.4 |` → `| 1.3.12, 1.3.66, 1.3.72, 1.3.74, 1.3.78, 3.4.78, 1.3.9, 1.2.4 |`.
- `pins all 129 ids verbatim` → `pins all 136 ids verbatim`
- `*janasanakhanāṁ sañjhaloḥ* and 6.4.43 *ye vibhāṣā* (8.4.40 widened) — 129` / `total).` → `*janasanakhanāṁ sañjhaloḥ* and 6.4.43 *ye vibhāṣā* (8.4.40 widened) — 129` / `total — then curādi 10a's seven: the sanādi stage's 3.1.25, 1.3.9 (ṇic's),` / `3.4.114, 7.2.116, 7.3.86 (a third entry) and 3.1.32, and 1.3.74 *ṇicaś ca* in` / `` `samjna.rs` — 136 total).``
- `Nine gaṇas are covered, all nine fully: bhvādi (1), divādi (4),` → `Ten gaṇas are covered, nine of them fully: bhvādi (1), divādi (4),`
- `√bhas; slice 3f2; √jan; slice 3f3). gaṇa` → `√bhas; slice 3f2; √jan; slice 3f3) — and curādi (10), **open** at 4 of its` / `509 rows (√cur, √laḍ, √bhakṣ, √bhūṣ; slice 10a). gaṇa`
- `` `Tag::Juhotyadi`, mirroring how `` → `` `Tag::Juhotyadi` / `Tag::Curadi`, mirroring how ``, and in the same sentence `3.1.73, 3.1.77, 3.1.78, 3.1.79, 3.1.81, 2.4.72 and 2.4.75.` → `3.1.73, 3.1.77, 3.1.78, 3.1.79, 3.1.81, 2.4.72, 2.4.75 and 3.1.25.`
- The 7.1.35 / 8.4.56 paragraph:
  - `forking 156 cells (loṭ` / `prathama and madhyama eka across the 78 roots` → `forking 164 cells (loṭ` / `prathama and madhyama eka across the 82 roots`
  - `the twenty-six roots that admit both` → `the thirty roots that admit both`
  - `√kṛ, √dā, √dhā, √bhṛ, √ṇij, √vij and √viṣ — and √bhuj by 1.3.66) reach it in their` → `√kṛ, √dā, √dhā, √bhṛ, √ṇij, √vij and √viṣ — √bhuj by 1.3.66, and curādi's √cur,` / `√laḍ, √bhakṣ and √bhūṣ by 1.3.74) reach it in their`
  - `78 + 25 = the 103 curated roots` → `82 + 25 = the 107 curated roots`
  - `forking 164 cells outright` → `forking 172 cells outright`
  - `those same 78 parasmaipada columns (141 of them` → `those same 82 parasmaipada columns (149 of them`
  - `` its 6.4.43 branch's `jajAyAd ~ jajAyAt` keys on 6.4.43 as well), `` → `` its 6.4.43 branch's `jajAyAd ~ jajAyAt` keys on 6.4.43 as well; 10a's four curādi roots contribute both cells, √cur's keyed on its mandatory sanādi 7.3.86 as well), ``
  - `forking a further 156 (the same` → `forking a further 164 (the same`

  Check: 149 + 22 + 1 = 172, and 156 + 8 = 164. What must be right: 4 new parasmaipada columns, 16 new `7.1.35`-keyed rows (6 `7.1.35`, 6 `7.1.35+8.4.56`, 2 `7.3.86+7.1.35`, 2 `7.3.86+7.1.35+8.4.56`), and 8 new 8.4.56 rows (6 plain, 2 `7.3.86+8.4.56`).

If any `old` string is not found exactly once, read the paragraph and edit it to the same facts. Never leave a count stale.

- [ ] **Step 6: AGENTS.md**

Each `old` → `new` occurs exactly once.
- `(\`crates/panini/tests/paradigm/\`, 4644 cells, nine gaṇas, all complete —` → `(\`crates/panini/tests/paradigm/\`, 4932 cells, ten gaṇas, nine complete —`
- `after slice 3f2 curated √bhas, and closing at 26 of 26 in slice 3f3 with √jan —` → `after slice 3f2 curated √bhas, and closing at 26 of 26 in slice 3f3 with √jan,` / `and curādi (10) opened in slice 10a at 4 of its 509 rows (√cur, √laḍ, √bhakṣ,` / `√bhūṣ) —`
- `a second (604 cells), a third (157 cells), a fourth` → `a second (612 cells), a third (165 cells), a fourth`
- `(1106 rows in all, so 4644 + 1106 = 5750 forms total)` → `(1130 rows in all, so 4932 + 1130 = 6062 forms total)`
- The audit record: `` (`tools/audit/README.md`'s 2026-10-01 entry, 4644 cells / 5750 forms / 103 `` / `roots).` → `` (`tools/audit/README.md`'s 2026-10-01 entry, 4644 cells / 5750 forms / 103 `` / `` roots), and that by curādi 10a's (`tools/audit/README.md`'s <DATE> entry, 4932 `` / `cells / 6062 forms / 107 roots).`
- `4644 goldens` → `4932 goldens`
- `` `TINANTA_RULES` is a list of eight stage arrays, `` → `` `TINANTA_RULES` is a list of nine stage arrays, ``
- Immediately after `in its stage` / `file; tests asserting a surface form or trace go in` / `` `tinanta/derivation_tests.rs`. ``, insert: `` A guard test looks its rule up in its own stage's static (`GUNA.iter().find(…)`, `SAMJNA.iter()…`), never with `rules().find(…)`: ids repeat across stages — 1.3.9 twice and 7.3.86 three times since curādi 10a — and `rules()` returns the first, which is the sanādi stage's. ``
- The stale-comment ledger: the 3f3 sentence is ``Juhotyādi 3f3 touched neither comment either; the corpus stands at 4644 cells as of 3f3 (`guna.rs:2433`'s claim anchored at `guna.rs:2565`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit).`` After it, insert ``Curādi 10a touched neither comment either; the corpus stands at 4932 cells as of 10a (`guna.rs:2433`'s claim anchored at `guna.rs:<G>`, `controller.rs:206`'s at `controller.rs:<C>`; both lines measured by grep at this commit).`` Measure `<G>` with `grep -n "1872 goldens move" crates/panini-prakriya/src/tinanta/guna.rs` (2565 on `main`; Task 4 edits only test code, below it) and `<C>` with `grep -n "only 8 cells fire" crates/panini-prakriya/src/controller.rs` (206). Measure; never compute.

- [ ] **Step 7: The 3f3 spec's out-of-scope line**

In `docs/superpowers/specs/2026-10-01-juhotyadi-gana-3f3-design.md`, the line `6.4.42 name; none is curated); 8.4.44 as a rule of its own; curādi.` → `6.4.42 name; none is curated); 8.4.44 as a rule of its own; curādi (opened in` / `` slice 10a — `2026-10-01-curadi-gana-10a-design.md`). ``

- [ ] **Step 8: Sweep for anything left stale**

```bash
grep -rn "4644\|5750\|\b1106\b\|103 roots\|103-root\|of these 103\|of the 103\|\b516\b\|808 of\|\b604\b\|157 hold\|(157 cells)\|forking 156\|forking 164\|141 of them\|78 + 25\|78 parasmaipada\|78 roots with\|129 ids\|nine gaṇas\|all nine fully\|all complete\|eight stage\|eight pipeline\|eight ordered\|twenty-six roots that admit\|Fifty-six surfaces\|fifty-six" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.md --include=*.rs
grep -rn "10a" crates tools README.md AGENTS.md docs/ARCHITECTURE.md
grep -rn "\bcur\b\|\bcori\b\|\blAqi\b\|\bBakzi\b\|\bBUzi\b\|\blaq\b\|3\.1\.25\|3\.1\.32\|7\.2\.116\|3\.4\.114\|1\.3\.74\|Nijanta\|\bRit\b" crates/panini-prakriya/src --include=*.rs | grep "//"
grep -rn "rules().find\|rules().filter" crates/panini-prakriya/src
```

Expected residue:
- `AGENTS.md:37`'s floor paragraph (`measured at 4644 cells`), which Task 8 rewrites.
- AGENTS.md's dated mutation record, `tools/audit/README.md`'s older entries, the audit chain in `paradigm/main.rs`, and the 3f3 paragraphs. That is history; never rewrite it.
- `pada_ambiguous_surfaces_are_exactly_these`'s comment `taking the set from fifty-six to seventy-two`.
- "10a" wherever it records what 10a *did*.
- In the last grep: `rules().find`/`filter` sites only for ids that occur once (any id but `1.3.9` and `7.3.86`), or for 7.3.84/1.2.4 through their existing `.nth()` idiom, which `sanadi` does not shift.

For every root-shape and rule-id hit, check the comment is still true with the sanādi stage in place.

- [ ] **Step 9: Run the full suite and commit**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms). Expected: PASS at 4932 cells.

```bash
mise run fmt && mise run lint
git add -A
git commit -m "docs: 10a's counts, the audit record, and the sweep

4932 cells / 6062 forms / 107 roots / 1130 ALTERNATES across README,
ARCHITECTURE, AGENTS, paradigm/main.rs and tools/audit; curādi open at 4/509.
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
- **This slice edits `samjna.rs`, `guna.rs` (tests only), `tin.rs` (tests only), `terms.rs` and `mod.rs`**, not `adesha.rs` or `tripadi.rs`. So the three documented non-caught entries (`adesha.rs:589:30`, `tripadi.rs:1289:38`, `tripadi.rs:1602:23`) should not move. Confirm them by `--list`, never by assumption.

- [ ] **Step 1: Measure the floor**

With nothing else running, run this twice: `time mise run test 2>&1 | tail -3` (foreground). Record both wall clocks and `cat /proc/loadavg`. The 4644-cell floor was 9.077s / 8.855s.

- [ ] **Step 2: Locate and probe the two uncaught equivalents at `-j 4`**

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /="
```

Write the positions as `<A>` (adesha), `<T1>` (the tripadi equivalent, inside 8.3.13's `apply`) and `<T2>` (the ṇatva hang). Then run in the foreground with timeout 600000 ms:

```bash
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 150 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:<A>:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" 2>&1 | tail -10
```

Both must be MISSED, not TIMEOUT. Read each test-phase duration from `$SCRATCH/mutants.out/outcomes.json`.

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/curadi-10a"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout 150 -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
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
- `missed.txt` holds exactly `adesha.rs:<A>:30: replace + with *` and `tripadi.rs:<T1>:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:<T2>:23: replace -= with /=`.
- panini-analyze: 0 missed, 0 timeout.

A scoped probe while this plan was written already ran every mutant in the new code: 26 in `sanadi.rs`, 1.3.74's `delete !`, 1.3.78's two new `||` → `&&`, and `derive`'s two field deletions. All 31 were caught.

If not:
- Any **other timeout** is a suspect survivor. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant in `sanadi.rs` or 1.3.74 means a Task 3–4 test does not separate it. Strengthen the named test, commit, and re-run only those mutants with `--re` and a fresh `-o`.

- [ ] **Step 5: Margins**

Inspect one record to see how `duration` is stored:

```bash
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["outcomes"][1])' "$OUT/mutants.out/outcomes.json"
```

Compute the two equivalents' test phases under campaign load, and the caught min/median/p90/max. The margin is 150 ÷ the longest equivalent phase.
- If the margin is ≥ 5, the cap stays 150.
- Otherwise the new cap is 6 × that phase, rounded up to the next 10 s. Change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together.

- [ ] **Step 6: Record it in AGENTS.md**

- Rewrite `**The floor behind the 150s cap, measured at 4644 cells on Rust 1.99.0,` / `2026-10-01.**` with Step 1's and Step 2's numbers at 4932 cells, and the cap Step 5 chose.
- Replace the `**Current record (juhotyādi 3f3, 2026-10-01).**` paragraph with `**Current record (curādi 10a, <DATE>).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**;
  - the non-caught set diffed against 3f3's (the clean result is identical);
  - the campaign-load phases and margins;
  - that no `sanadi.rs` or 1.3.74 mutant is missed, timed out or unviable-without-reason, with each one's `file:line` range and mutant count;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`.

  End it with a pointer to the record it replaces: run `git rev-parse --short HEAD` before committing and write `The juhotyādi 3f3 record it replaces: \`git show <that hash>:AGENTS.md\`.`

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md mise.toml
git commit -m "chore: 10a mutation gate — floor and uncaught run re-measured at 4932 cells

missed.txt holds only the two documented equivalents and timeout.txt only
the permanent ṇatva-scan entry; every sanādi-stage and 1.3.74 mutant is caught."
```

---

## Task 9: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -20
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin curadi-10a
gh pr create --title "curādi 10a — ṇic, 3.1.32 and 1.3.74; gaṇa 10 opens" --body "$(cat <<'BODY'
Slice 10a opens curādi (gaṇa 10), the one gaṇa the engine had not touched,
with √cur (*corayati*), √laḍ (*lāḍayati*), √bhakṣ and √bhūṣ — all ubhayapadī
by 1.3.74 *ṇicaś ca*. The golden suite goes from 4644 to 4932 cells.

**A new first pipeline stage, `sanadi`.** 3.1.25 adds ṇic; its it-lopa leaves
a ṇit `i`; 3.4.114 makes it ārdhadhātuka; 7.2.116 *ata upadhāyāḥ* or a pre-ṇic
7.3.86 acts on the root beside it; and 3.1.32 *sanādyantā dhātavaḥ* folds it
into the aṅga and removes its term. Every later stage then sees an ordinary
i-final dhātu (`cori`) in the usual three-slot layout, and derives it exactly
as it derives √nī — no existing rule's `ANGA` read changes.

**1.3.74 in `samjna`**, the structural twin of 1.3.66, keyed on a new
`PadaAssignment::Nic`.

Duplicate ids (1.3.9 twice, 7.3.86 three times) meant stage tests that found
rules by `rules().find` silently reached the sanādi entry; they now search
their own stage's static.

The audit shows zero divergence against `8da2f90b`, a main-vs-branch dump of
every prior cell's traces is byte-identical, and the mutation gate is clean.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`: `git worktree remove .worktrees/curadi-10a` and `git worktree remove --force .worktrees/curadi-10a-proto` (the throwaway). Then delete the local and remote `curadi-10a` branch, and run `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| `Gana::Curadi`, `PadaAssignment::Nic`, four rows, prefix and pada-marker tests | 2, 5 |
| `Tag::Curadi`/`Nic`/`Nijanta`/`Rit`; `derive` arms | 2 |
| 1.3.74 beside 1.3.66/1.3.72, declining parasmaipada; 1.3.78's `Nic` arm; 1.3.72 never on a `Nic` row | 3 |
| `sanadi` stage first; 3.1.25, ṇic's local it-lopa (one 1.3.9), 3.4.114, 7.2.116, pre-ṇic 7.3.86, 3.1.32 removing the term and tagging `Nijanta`; `NIC` note in `terms.rs` | 4 |
| Guard unit tests for every sanādi rule and 1.3.74 | 3, 4 |
| Rule-order pin | 3, 4 |
| 288 goldens, 24 `ALTERNATES` over 16 cells, totals 107 / 4932 / 6062 | 5 |
| Trace pins *corayati*, *corayate*, *lAqayati*, *Bakzayati*/*BUzayati*, *BakzayARi*, *acorayad* | 6 |
| Fires-only: sanādi rules and 1.3.74 on `10.x` only; 7.3.86 off curādi at 384 | 6 |
| Prior traces byte-identical, main ↔ branch | 2–4 (suite at 4644), 7 Step 2 (dump diff) |
| Allow-list "curādi open at 4 of 509" | 5 (`curadi_rows_are_the_four_curated_roots`) |
| Spot check (`check()`) | 5 |
| Audit with repoint and negative control; README / ARCHITECTURE / AGENTS; 3f3 spec pointer | 7 |
| Floor, uncaught probe, campaign, verbatim non-caught record | 8 |

**Spec deviations, recorded** (the spec is corrected alongside this plan):
- **Stage-local test lookups.** The spec did not foresee that existing guard tests find rules with `rules().find`. Task 4 Step 5 changes nine of them, and AGENTS.md records the rule (Task 7 Step 6).
- **The sanādi guards key ṇic's identity** (`Ric` / `Tag::Rit` / `Tag::Ardhadhatuka`), not "a term exists at `NIC`". The spec said "on the ṇic term being present". The stricter form is what keeps a hand-built chain with a tiṅ at index 3 safe.
- **ALTERNATES keys.** √cur's keys carry its mandatory sanādi 7.3.86 (`7.3.86+8.4.56`, and the new `7.3.86+7.1.35` and `7.3.86+7.1.35+8.4.56`). This is the documented id-sharing artifact of `VIKALPA_RULES`, not a new vikalpa.
- **Pada-ambiguous surfaces** gain sixteen, and README's both-pada count moves from twenty-six to thirty. The spec's doc-sweep list did not name either.
- **`credited` moves to `trace/helpers.rs`**, because curādi is its second consumer.
- **1.3.74's position** is between 1.3.72 and 1.3.78 (sūtra order), not literally beside 1.3.66.

**Type consistency.**
- `NIC` (`terms.rs`) is used in `sanadi.rs` and its tests.
- `SANADI` is in `sanadi.rs` and `mod.rs`.
- `nic_prakriya(Pada) -> Prakriya` is in `samjna.rs` tests (Task 3). `with_nic(&str, Option<&[Tag]>) -> Prakriya` and `rule(&str) -> &'static Rule` are in `sanadi.rs` tests.
- `credited(&str) -> Vec<(&'static str, Gana)>` is in `trace/helpers.rs`.
- The rule names are as listed in Global Constraints, everywhere.

**Known soft spots.**
- **The 384 pin.** It was measured from the throwaway dump at `4dd7b53`, counting live branches as `credited()` does. Task 7 Step 2 re-measures it on `main` directly.
- **Doc strings in Task 7** were read from `main` at `4dd7b53`. If one is not found exactly once, edit the paragraph to the same facts rather than skip it.
- **`Tag::Nijanta` has no reader.** It is pinned only by `sanadyanta_folds_nic_into_the_dhatu`, on `Tag::Abhyasa`'s precedent.
