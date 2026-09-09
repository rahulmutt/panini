# Juhotyādi gaṇa 3a — the reduplication core Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Land `Gana::Juhotyadi` with ślu, dvitva and the abhyāsa core on √hu (`03.0001`) and √ki (`03.0020`): 72 new cells, 84 forms, suite 3492 → 3564, with 3492 priors and every pinned trace byte-identical.

**Architecture:** A new pipeline stage, `abhyasa.rs`, sits between `vikarana` and `anga` and owns dvitva (6.1.10) and the abhyāsa-shaping rule 7.4.62, so 6.4.71 always sees a finished abhyāsa in the `ABHYASA` slot the prep created and 7.1.4 reads a real `Tag::Abhyasta`. 2.4.75 elides śap by ślu and marks the empty term `Tag::Slu`, which 6.1.10 alone reads. Six other rules land in their pipeline positions — 3.4.109 (`tin.rs`), 7.1.4 (`anga.rs`), 7.3.83 and 6.4.82 (`guna.rs`), 8.4.54 (`tripadi.rs`) — and two existing rules grow their √hu arm (6.4.87, 6.4.101). Dvitva runs before guṇa (the Kaumudī order), a recorded trace divergence from vidyut with identical forms.

**Tech Stack:** Rust 1.98 via `mise`; `cargo test` golden suite; `cargo-mutants` gate; `vidyut-prakriya` at commit `8da2f90b` via the committed harness in `tools/audit/`.

**Spec:** `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md` — the "Slice 3a: the reduplication core" section, as amended in `0a29074` (6.4.101's hu arm, the two ordering pins, the augment-guard argument, the counts). The prep section is merged (PR #33). "Later slices" is out of scope.

**Branch:** `juhotyadi-3a`, already created at the spec commit `0a29074` on top of `main` (`bed84b2`). Execute in a worktree per `superpowers:using-git-worktrees`.

## Global Constraints

- **Toolchain:** rust 1.98 via `mise`. `mise run test -- -p X` does NOT scope — use `mise exec -- cargo test -p X`.
- **3492 priors byte-identical at every commit.** No existing golden or trace changes. Tasks 1–3 add rules guarded on tags no curated root carries (`Juhotyadi`, `Slu`, `Abhyasta`) and provably cannot move a golden: they run the unit suite and lint. Tasks 4, 5, 6 and 8 run the FULL suite (`mise run test`) in the foreground. A failing prior is a defect in that task; never regenerate a golden to make a task pass.
- **Run the golden suite in the FOREGROUND with the largest timeout the harness allows.** Prior floor: **2025.227s at 3492 cells** (~34 min; paradigm 895.91s, roundtrip 1124.11s, trace 3.74s). If the harness caps a single command below that, launch detached to a log (`nohup mise run test > /tmp/3a-suite.txt 2>&1 &`) and wait on it with a monitor loop until the final `test result:` lines appear; never end a turn while it runs. Wait on the PID with `kill -0`, not `pgrep -f "cargo test"` (which matches its own wrapper).
- **No unkillable clauses.** A guard clause no curated input can falsify is a guaranteed mutation survivor and is not written; the argument goes in the comment instead (the 7.4.21 / 6.4.72 precedent). This plan deviates from two of the spec's guard wordings for exactly that reason — see "What the planning-time reading settled".
- **Goldens are generated from the audited engine, never hand-authored.** Task 7 (audit) blocks Task 8 (goldens). The expected forms quoted in this plan are recognition tripwires from the vidyut probe, not values to type.
- **The audit's negative control runs first.** A zero-difference verdict without a verified-failing `entry` control proves nothing.
- **`mise run mutants` is `-j 4 --timeout 4800`.** The mise shim fails in background shells (`No version is set for shim: cargo-mutants`); run the `cargo-mutants` binary directly with the task's exact arguments. `CARGO_MUTANTS_JOBS` must be unset.
- **SLP1 throughout.** `M` anusvāra, `N` velar ṅ, `Y` palatal ñ, `R` retroflex ṇ, `z` retroflex ṣ, `f` vocalic ṛ, `E`/`O` ai/au, `J` jh, `D` dh, `B` bh.
- **Executing from a worktree:** `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml`'s `panini`/`panini-data` dev-dep paths currently point at `/workspace/crates` (repointed during planning). Task 7 sets them to THE CHECKOUT BEING AUDITED; a stale path audits the wrong engine.

## Numbers this plan changes

| number | before | after | where |
|---|---|---|---|
| curated roots (`dhatus().len()`) | 77 | 79 | `panini-data/src/lib.rs` |
| `PARADIGM` blocks / cells | 388 / 3492 | 396 / 3564 | `tests/paradigm/main.rs`, `tools/audit` |
| `ALTERNATES` rows / total forms | 907 / 4399 | 919 / 4483 | same |
| one/two/three-form cells | 2818 / 522 / 111 | 2882 / 526 / 115 | shape test |
| four/five/six-form cells | 17 / 8 / 16 | unchanged | shape test |
| `8.4.56` / `7.1.35` / `7.1.35+8.4.56` keys | 130 / 108 / 108 | 134 / 112 / 112 | shape test |
| pinned rule ids | 99 | 107 | `tinanta_rule_order_is_pinned` |
| pipeline stages | 7 | 8 | `TINANTA_RULES`, docs |
| `VIKALPA_RULES` | 8 | 8 | unchanged |
| pada-ambiguous surfaces | 44 | 44 | unchanged (both roots parasmaipadī) |

The +12 alternates are six per root: 8.4.56 on laṅ prathama eka and vidhiliṅ prathama eka; 7.1.35 and 7.1.35+8.4.56 on each of the two loṭ tātaṅ cells (prathama eka, madhyama eka). Distribution over the 72 new cells: 64 one-form, 4 two-form, 4 three-form. Measured by the generator in Task 8; if it prints anything else, stop and diagnose.

## What the planning-time reading settled

- **vidyut's 72 cells were re-derived** (all four lakāras, both roots, `/tmp/vidyut-full/vidyut-prakriya/examples/hu_ki_trace.rs`, throwaway). 42 + 42 forms, matching the spec's appendix. Every rule the spec lists is credited; the trace order there is 7.3.84 → 6.1.10 → 7.4.59 → 7.4.62 (guṇa first, copy, shorten), which this engine deliberately does NOT follow (dvitva first, no 7.4.59 needed for short-vowel roots). vidyut also runs 3.4.109 after dvitva on a real abhyasta tag; ours runs it in `tin.rs` on `Tag::Juhotyadi` (the spec's stand-in).
- **The ṅit plumbing already works on the ślu path.** The first 1.2.4 (`samjna.rs`) tags every apit ending ṅit, pada-independent; 3.4.87 and 7.1.35 re-tag `hi` and `tAt` ṅit; `following_sarvadhatuka` returns the ending when SHAP is empty. So *juhutaḥ*, *juhudhi*, *juhutāt* block guṇa with no new code, and *juhoti*, *ajuhot*, *ajuhavam*, *juhavāni* take it (pit `ti`/`t`/`am`/`Ani`). 6.1.78's athematic arm (SHAP empty, vowel-initial ending) delivers *hav-*/*kay-* before `us`, `am`, `Ani`.
- **Two spec guard wordings are relaxed on the unkillable-clause rule.** (1) The spec writes the √hu arms of 6.4.87 and 6.4.101 as `ANGA.text == "hu"` *with* `Tag::Juhotyadi`; no other curated root reads `hu`, so the tag clause can never be falsified — the guard is `ANGA.text == "hu"` alone, with the 7.4.21 reasoning in the comment. (2) 7.4.62 and 8.4.54 read the `ABHYASA` slot's text, not `Tag::Abhyasa`: the slot is non-empty exactly when 6.1.10 filled it, so a tag test there is equivalent and unfalsifiable. `Tag::Abhyasa` is still SET by 6.1.10 (it is 6.1.4's saṁjñā and 3b's abhyāsa rules will want it) and is pinned by a unit test, which is what keeps its `add` killable. Task 11 records (1) in the spec's two bullets.
- **6.4.101 needs its hu arm** (spec amendment): `adesha.rs`'s rule has only the jhal arm; `sound_before_ending` reads `u` for √hu and would leave *\*juhuhi*. vidyut credits 6.4.101 on exactly that cell.
- **6.4.82 must sit after 7.3.84** (spec amendment): loṭ uttama's `Ani` is pit, so *cikayāni* guṇates first; its slot before 6.4.77 (after 6.4.87) satisfies this. **8.4.54 needs 8.4.53's no-op guard**: √ki's abhyāsa `ci` is already car and vidyut records no step there.
- **7.3.83's "immediately following jus"** is `following_sarvadhatuka(p).text == "us"`: on the ślu path the follower is the ending, and `us` is the bare text 3.4.108/3.4.109 leave after 1.3.9. Vidhiliṅ's jus is `yAus` at this stage (yāsuṭ in front; 6.1.96 makes it `yus` later), so it never matches; the only other laṅ `us` in scope is 3.4.111's optional jus after an ā-final adādi aṅga, which declines on `guna_of('A')`. No lakāra clause is needed or written.
- **The tripādī scans now see the abhyāsa's characters** (`word_chars` walks every non-empty term). For √hu and √ki no scan fires wrongly: no `n` for ṇatva, no jhal–jhaṣ or jhal–khar junction inside `ju`/`ci`, and 8.3.59 still finds the `s`-initial affix after `ANGA` and the iṇ before it (`o`/`e`). Task 5's full-suite run and Task 7's audit are the proof.
- **The augment guards stay on `ANGA`** (spec subsection, decided in brainstorming): 6.4.71/6.4.72 keep reading `terms[ANGA]`'s first character; the class-preservation argument goes into both comments (Task 5). Slice 3d's √ṛ is the review checkpoint.
- **`Tag::Abhyasa` exists in `term.rs` already** (line 13, undocumented, unused). `ABHYASA` in `terms.rs` carries `#[allow(dead_code)]` with an instruction to remove it once a rule body reads the slot — Task 2 does.

## File Structure

| file | responsibility | task |
|---|---|---|
| `crates/panini-data/src/lib.rs` | `Gana::Juhotyadi`; prefix test arm; two `Dhatu` rows; row test; counts | 1, 6 |
| `crates/panini-prakriya/src/term.rs` | `Tag::Juhotyadi`, `Tag::Slu`, `Tag::Abhyasta`; doc on `Tag::Abhyasa` | 1 |
| `crates/panini-prakriya/src/tinanta/mod.rs` | gaṇa→tag arm; `mod abhyasa;` + stage wiring; module doc | 1, 2 |
| `crates/panini-prakriya/src/tinanta/vikarana.rs` | 2.4.75 after 2.4.72 | 1 |
| `crates/panini-prakriya/src/tinanta/abhyasa.rs` (new) | 6.1.10, 7.4.62 | 2 |
| `crates/panini-prakriya/src/tinanta/sound.rs` | `cutva_of`, `deaspirate_of` + all-arms tests | 2, 5 |
| `crates/panini-prakriya/src/tinanta/terms.rs` | drop `ABHYASA`'s `#[allow(dead_code)]`; "six stage files" → eight | 2, 11 |
| `crates/panini-prakriya/src/tinanta/tin.rs` | 3.4.109 after 3.4.108 | 3 |
| `crates/panini-prakriya/src/tinanta/anga.rs` | 7.1.4 before 7.1.3; 6.4.71/6.4.72 comment argument | 3, 5 |
| `crates/panini-prakriya/src/tinanta/guna.rs` | 7.3.83 before 7.3.84; 6.4.87 hu arm; 6.4.82 before 6.4.77 | 4 |
| `crates/panini-prakriya/src/tinanta/adesha.rs` | 6.4.101 hu arm | 5 |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | 8.4.54 after 8.4.53 | 5 |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | pinned order (each task); whole-derivation tests | 1–6 |
| `crates/panini/tests/paradigm/main.rs`, `data/mod.rs`, `data/juhotyadi.rs` (new) | GATED window; goldens; shape test | 6, 8 |
| `crates/panini/tests/trace/main.rs`, `trace/juhotyadi.rs` (new) | ordered-trace pins | 9 |
| `tools/audit/panini_full_audit.rs`, `tools/audit/README.md` | `gana_name` arm; totals; recorded result | 7 |
| `AGENTS.md`, `README.md`, `docs/ARCHITECTURE.md`, spec | timing paragraph; nine gaṇas; stage table; counts | 10, 11 |

---

### Task 1: `Gana::Juhotyadi`, the three tags, and 2.4.75 ślu

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (the `Gana` enum at the top; `gana_matches_dhatupatha_prefix` near line 1640)
- Modify: `crates/panini-prakriya/src/term.rs` (the `Tag` enum)
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs:88-97` (the gaṇa→tag match in `derive`)
- Modify: `crates/panini-prakriya/src/tinanta/vikarana.rs` (new rule after 2.4.72, before 3.4.111; module doc's rule list)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: nothing.
- Produces: `Gana::Juhotyadi` (panini-data); `Tag::Juhotyadi` (on `ANGA`, set by `derive`), `Tag::Slu` (on the empty `SHAP`, set by 2.4.75), `Tag::Abhyasta` (declared here, set in Task 2); rule `2.4.75`. Every later task relies on: after 2.4.75, `terms[SHAP].text == ""`, `SHAP` has `Vikarana` and `Slu` and NOT `Thematic`; `ENDING` is still index 4.

- [ ] **Step 1: Write the failing unit tests**

In `crates/panini-prakriya/src/tinanta/vikarana.rs`'s `tests` module, after `second_1_2_4_still_tags_the_shit_vikaranas`:

```rust
    #[test]
    fn juhotyadi_slu_empties_shap_in_place_and_tags_it_slu() {
        // 2.4.75 juhotyAdiByaH SluH. The same in-place emptying as 2.4.72,
        // plus the one thing that separates ślu from luk: Tag::Slu, which
        // 6.1.10 reads. Thematic must go — 6.4.105 would otherwise luk
        // juhuDi's hi behind a śap that no longer exists.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("hu"), Term::new("ti")]),
            ..Default::default()
        };
        p.terms[ANGA].add(Tag::Dhatu);
        p.terms[ANGA].add(Tag::Juhotyadi);
        let r_68 = rules().find(|r| r.id == "3.1.68").unwrap();
        assert!((r_68.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "a");
        let r_75 = rules().find(|r| r.id == "2.4.75").unwrap();
        assert!((r_75.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "");
        assert!(p.terms[SHAP].has(Tag::Slu));
        assert!(p.terms[SHAP].has(Tag::Vikarana));
        assert!(!p.terms[SHAP].has(Tag::Thematic));
        assert_eq!(p.terms.len(), ENDING + 1);
        assert_eq!(p.terms[ENDING].text, "ti");
        assert_eq!(p.text(), "huti");
        assert_eq!(p.log.last().unwrap().sutra, "2.4.75");
        // Nothing left to elide: a second application declines rather
        // than recording a vacuous step (this is what the non-empty test
        // in the guard is for).
        assert!(!(r_75.apply)(&mut p));
    }

    #[test]
    fn slu_declines_for_every_other_gana_and_luk_never_tags_slu() {
        // adādi's 2.4.72 and juhotyādi's 2.4.75 must not cross: √ad keeps
        // its luk with no Slu (so 6.1.10 stays silent — atti, not *atatti),
        // and a bhvādi śap is nobody's to elide.
        let r_68 = rules().find(|r| r.id == "3.1.68").unwrap();
        let r_72 = rules().find(|r| r.id == "2.4.72").unwrap();
        let r_75 = rules().find(|r| r.id == "2.4.75").unwrap();
        for (root, tag) in [("ad", Some(Tag::Adadi)), ("BU", None)] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new(root), Term::new("ti")]),
                ..Default::default()
            };
            if let Some(tag) = tag {
                p.terms[ANGA].add(tag);
            }
            assert!((r_68.apply)(&mut p));
            assert!(!(r_75.apply)(&mut p), "{root}");
            assert!(!p.terms[SHAP].has(Tag::Slu), "{root}");
        }
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ad"), Term::new("ti")]),
            ..Default::default()
        };
        p.terms[ANGA].add(Tag::Adadi);
        assert!((r_68.apply)(&mut p));
        assert!((r_72.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "");
        assert!(!p.terms[SHAP].has(Tag::Slu));
    }

    #[test]
    fn slu_single_term_anga_does_not_panic() {
        // The `len() > SHAP` boundary, pinned the way
        // `kartari_sap_single_term_anga_does_not_panic` pins 3.1.68's: a
        // three-term prakriya (aṅga only) must short-circuit before
        // indexing terms[SHAP].
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("hu")]),
            ..Default::default()
        };
        p.terms[ANGA].add(Tag::Juhotyadi);
        let r_75 = rules().find(|r| r.id == "2.4.75").unwrap();
        assert!(!(r_75.apply)(&mut p));
        assert_eq!(p.terms.len(), ANGA + 1);
    }
```

In `derivation_tests.rs`'s `tinanta_rule_order_is_pinned`, insert `"2.4.75"` immediately after `"2.4.72"`.

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya slu 2>&1 | tail -20`
Expected: compile error — `no variant named Juhotyadi` / `Slu` in `Tag`.

- [ ] **Step 3: The enum variants**

`crates/panini-data/src/lib.rs`, the `Gana` enum:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gana {
    Bhvadi,
    Divadi,
    Tudadi,
    Adadi,
    Kryadi,
    Svadi,
    Rudhadi,
    Tanadi,
    Juhotyadi,
}
```

Same file, `gana_matches_dhatupatha_prefix` (near line 1640): add the arm `Gana::Juhotyadi => "03",` after `Gana::Kryadi => "09",` and change its comment "this engine covers eight of the ten gaṇas, so 03 and 10 have no `Gana` variant" to "this engine covers nine of the ten gaṇas, so only 10 has no `Gana` variant".

`crates/panini-prakriya/src/term.rs`: give the existing bare `Abhyasa,` variant a doc comment and add three variants after `Tanadi,`:

```rust
    /// 6.1.4 pūrvo'bhyāsaḥ: the earlier of the two copies dvitva makes —
    /// the reduplicant at `ABHYASA`. Set by 6.1.10. No rule reads it in
    /// slice 3a: the rules whose locative *abhyāse* names it (7.4.62,
    /// 8.4.54) read the slot's text instead, because the slot is non-empty
    /// exactly when 6.1.10 has filled it and a tag test there could never
    /// be falsified. The tag is the saṁjñā verdict itself, pinned by
    /// 6.1.10's unit test, for the abhyāsa rules slice 3b brings (7.4.59,
    /// 7.4.60).
    Abhyasa,
```

```rust
    /// The dhātu belongs to juhotyādi (gaṇa 3), the ślu gaṇa. Read by
    /// 2.4.75 once the vikaraṇa exists — and, before it, by 3.4.109 as the
    /// data-layer stand-in for *abhyasta* (see that rule's comment).
    /// Mirrors Divadi/Tudadi/Adadi/Kryadi/Svadi/Rudhadi/Tanadi.
    Juhotyadi,
    /// The śap at `SHAP` was elided by ŚLU (2.4.75), not luk (2.4.72). Both
    /// leave the term empty in place; only ślu triggers reduplication
    /// (6.1.10 *ślau*), and this tag is what 6.1.10 reads. Nothing else
    /// reads it.
    Slu,
    /// 6.1.5 ubhe abhyastam: the reduplicant AND the root it was copied
    /// from, together, are *abhyasta*. Set on both `ABHYASA` and `ANGA` by
    /// 6.1.10; read by 7.1.4 *ad abhyastāt* on `ANGA` — the aṅga before the
    /// tiṅ affix (1.4.13) is the pair, and its root half is the term the
    /// ending follows. `Abhyasa` marks the copy alone; this marks both.
    Abhyasta,
```

`crates/panini-prakriya/src/tinanta/mod.rs`, the match in `derive`: add `Gana::Juhotyadi => t.add(Tag::Juhotyadi),` after the `Tanadi` arm.

- [ ] **Step 4: 2.4.75**

In `vikarana.rs`, immediately after the 2.4.72 `Rule { ... }` and before the 3.4.110/3.4.111 comment block:

```rust
    // 2.4.75 juhotyādibhyaḥ śluḥ: juhotyādi (gaṇa 3) elides the śap that
    // 3.1.68 inserts by ŚLU, not luk. Same representation as 2.4.72 — the
    // term stays in place with empty text, so ENDING keeps its index and
    // every athematic arm downstream sees the empty SHAP it already handles
    // for adādi — and the same Tag::Thematic removal, for the same reason
    // (1.1.61: the vikaraṇa itself is gone, identity included).
    //
    // What ślu adds is Tag::Slu. 1.1.61 names luk, ślu and lup as three
    // kinds of adarśana, and the grammar tells them apart exactly once:
    // 6.1.10 *ślau* reduplicates the aṅga after ślu and after nothing else.
    // The tag IS that distinction, read by 6.1.10 alone; an "is SHAP empty
    // and the aṅga juhotyādi" test in its place would re-derive from two
    // terms the verdict this rule has already reached.
    Rule {
        id: "2.4.75",
        name: "juhotyAdiByaH SluH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Juhotyadi) {
                return false;
            }
            if !(p.terms.len() > SHAP
                && p.terms[SHAP].has(Tag::Vikarana)
                && !p.terms[SHAP].text.is_empty())
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = String::new();
            p.terms[SHAP].remove(Tag::Thematic);
            p.terms[SHAP].add(Tag::Slu);
            p.record("2.4.75", "juhotyAdiByaH SluH", before);
            true
        },
    },
```

Add `2.4.75` to the file's module-doc rule list (`//! 3.1.68, 2.4.72, 2.4.75, 3.1.83, 1.2.4.`) and to the 2.4.72 sentence there ("2.4.72 luks śap … — and 2.4.75 elides it by ślu the same way, tagging the term `Slu` for 6.1.10").

- [ ] **Step 5: Unit suite and lint**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -5` — Expected: all pass, including `tinanta_rule_order_is_pinned` (100 ids now).
Run: `mise exec -- cargo test -p panini-data 2>&1 | tail -5` — Expected: pass (no row carries the new gaṇa yet; the prefix test's new arm is exhaustive-match plumbing).
Run: `mise run lint && mise run fmt-check` — Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add crates/panini-data/src/lib.rs crates/panini-prakriya/src/term.rs crates/panini-prakriya/src/tinanta/mod.rs crates/panini-prakriya/src/tinanta/vikarana.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(engine): Gana::Juhotyadi and 2.4.75 slu — sap elided in place, tagged Slu for 6.1.10

Tag::Juhotyadi, Tag::Slu and Tag::Abhyasta declared; no root carries the
gaṇa yet, so 3492 priors are untouched."
```

---

### Task 2: the `abhyasa` stage — 6.1.10 and 7.4.62

**Files:**
- Create: `crates/panini-prakriya/src/tinanta/abhyasa.rs`
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs` (`mod abhyasa;`, `TINANTA_RULES`, module doc)
- Modify: `crates/panini-prakriya/src/tinanta/sound.rs` (`cutva_of` after `kutva_of`; test)
- Modify: `crates/panini-prakriya/src/tinanta/terms.rs:19-23` (drop `#[allow(dead_code)]` and its two-line note on `ABHYASA`)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (pinned order)

**Interfaces:**
- Consumes: `Tag::Slu` on `SHAP` (Task 1); `ABHYASA`, `ANGA`, `SHAP` from `crate::tinanta::terms`.
- Produces: after 6.1.10, `terms[ABHYASA].text` is a copy of the root, `ABHYASA` has `Abhyasa` + `Abhyasta`, `ANGA` has `Abhyasta`; after 7.4.62 the abhyāsa's initial ku/`h` is palatal (`hu` → `Ju`, `ki` → `ci`). `pub(crate) fn cutva_of(c: char) -> Option<char>` in `sound.rs`. `abhyasa::ABHYASA_RULES` sits between `vikarana::VIKARANA` and `anga::ANGA_RULES`.

- [ ] **Step 1: Write the failing tests**

In `sound.rs`'s `tests` module, after `kutva_of_cu_all_arms`:

```rust
    #[test]
    fn cutva_of_ku_and_h_all_arms() {
        // 7.4.62 kuhoś cuḥ: pin every arm directly. Only k -> c (√ki) and
        // h -> J (√hu) are reachable from the golden forms, so a mutant on
        // the other arms would be invisible without this. Mirrors
        // kutva_of_cu_all_arms, of which the four stop arms are the inverse.
        assert_eq!(cutva_of('k'), Some('c'));
        assert_eq!(cutva_of('K'), Some('C'));
        assert_eq!(cutva_of('g'), Some('j'));
        assert_eq!(cutva_of('G'), Some('J'));
        assert_eq!(cutva_of('N'), Some('Y'));
        assert_eq!(cutva_of('h'), Some('J'));
        for c in ['c', 'C', 'j', 'J', 'Y'] {
            assert_eq!(cutva_of(c), None, "{c} is cu already");
        }
        for c in ['t', 'p', 's', 'S', 'y', 'a', 'u'] {
            assert_eq!(cutva_of(c), None, "{c} is neither ku nor h");
        }
    }
```

Create `crates/panini-prakriya/src/tinanta/abhyasa.rs` with ONLY the test module for now (the rules come in Step 3):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::prakriya::Prakriya;
    use crate::term::Term;
    use crate::tinanta::rules;
    use crate::tinanta::terms::{ENDING, with_slots};

    /// The shape 2.4.75 leaves: root, an empty śap tagged Slu, the ending.
    fn slu_prakriya(root: &str, ending: &str) -> Prakriya {
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new(root), Term::new(""), Term::new(ending)]),
            ..Default::default()
        };
        p.terms[ANGA].add(Tag::Dhatu);
        p.terms[ANGA].add(Tag::Anga);
        p.terms[SHAP].add(Tag::Vikarana);
        p.terms[SHAP].add(Tag::Slu);
        p
    }

    #[test]
    fn slau_copies_the_anga_into_abhyasa_and_tags_both_terms() {
        // 6.1.10 SlO, with 6.1.4 and 6.1.5 as tags. All three tag writes are
        // asserted here: only ANGA's Abhyasta is read by a rule in this
        // slice (7.1.4), so the other two would otherwise be writes nothing
        // checks — and therefore unkillable under mutation.
        let mut p = slu_prakriya("hu", "ti");
        let rule = rules().find(|r| r.id == "6.1.10").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "hu");
        assert_eq!(p.terms[ANGA].text, "hu");
        assert!(p.terms[ABHYASA].has(Tag::Abhyasa));
        assert!(p.terms[ABHYASA].has(Tag::Abhyasta));
        assert!(p.terms[ANGA].has(Tag::Abhyasta));
        assert!(!p.terms[ANGA].has(Tag::Abhyasa));
        assert_eq!(p.terms[ENDING].text, "ti");
        assert_eq!(p.text(), "huhuti");
        assert_eq!(p.log.last().unwrap().sutra, "6.1.10");
    }

    #[test]
    fn slau_declines_for_luk_and_for_a_live_vikarana() {
        // adādi's empty śap is luk (2.4.72), not ślu: atti, not *atatti.
        let rule = rules().find(|r| r.id == "6.1.10").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ad"), Term::new(""), Term::new("ti")]),
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "");
        // A thematic vikaraṇa: nothing to do either.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("BU"), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "");
    }

    #[test]
    fn kuhos_cuh_palatalises_the_abhyasa_initial_only() {
        // hu → Ju (jh; 8.4.54 makes it j in the tripādī), ki → ci. The
        // root's own initial is untouched: the sūtra names the abhyāsa.
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        let r_62 = rules().find(|r| r.id == "7.4.62").unwrap();
        for (root, expected) in [("hu", "Ju"), ("ki", "ci")] {
            let mut p = slu_prakriya(root, "ti");
            assert!((r_10.apply)(&mut p));
            assert!((r_62.apply)(&mut p), "{root}");
            assert_eq!(p.terms[ABHYASA].text, expected, "{root}");
            assert_eq!(p.terms[ANGA].text, root, "{root}");
            assert_eq!(p.log.last().unwrap().sutra, "7.4.62");
        }
    }

    #[test]
    fn kuhos_cuh_declines_without_an_abhyasa_and_on_a_non_velar() {
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        let r_62 = rules().find(|r| r.id == "7.4.62").unwrap();
        // No abhyāsa (the slot is empty): every other gaṇa.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ki"), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(r_62.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "");
        // An abhyāsa whose initial is neither ku nor h (slice 3c's dA):
        // 6.1.10 fires, 7.4.62 has nothing to see.
        let mut p = slu_prakriya("dA", "ti");
        assert!((r_10.apply)(&mut p));
        assert!(!(r_62.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "dA");
    }
}
```

In `derivation_tests.rs`'s pinned order, insert `"6.1.10", "7.4.62"` immediately after the SECOND `"1.2.4"` (the one just before `"6.4.71"`).

- [ ] **Step 2: Run to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya cutva_of 2>&1 | tail -5` — Expected: compile error, `cutva_of` not found.
(The `abhyasa.rs` tests do not compile until Step 3 declares the module; that is expected.)

- [ ] **Step 3: `cutva_of`, the stage, and the wiring**

In `sound.rs`, after `kutva_of`:

```rust
/// The *cu* (palatal) counterpart of a *ku* sound or of `h` — 7.4.62 kuhoś
/// cuḥ's substitute, on the abhyāsa. The inverse of `kutva_of` for the four
/// stops (1.1.50 sthAne'ntaratamaH keeps voicing and aspiration: `k` to
/// `c`, `g` to `j`), plus the two arms `kutva_of` has no mirror for: `N`
/// (ṅ) to `Y` (ñ) — `ku~` is udit, so 1.1.69 pulls the nasal in — and `h`
/// to `J` (jh), the voiced aspirate `h` is nearest to; that is how
/// vidyut-prakriya writes it, and 8.4.54 then deaspirates it to `j`
/// (juhoti).
///
/// Only `k -> c` (√ki) and `h -> J` (√hu) have a curated witness; the other
/// four arms are present because the table covers the whole varga, the
/// same reason `kutva_of` carries `C`/`J`. `cutva_of_ku_and_h_all_arms`
/// keeps them from rotting. The palatals themselves are absent rather than
/// mapped to themselves, so `None` doubles as 7.4.62's match test.
pub(crate) fn cutva_of(c: char) -> Option<char> {
    Some(match c {
        'k' => 'c',
        'K' => 'C',
        'g' => 'j',
        'G' => 'J',
        'N' => 'Y',
        'h' => 'J',
        _ => return None,
    })
}
```

At the top of `abhyasa.rs`, above the test module:

```rust
//! Reduplication: 6.1.10, 7.4.62 — dvitva and the rules that reshape the
//! abhyāsa.
//!
//! Ordered AFTER 3.1.68 (ending at `ENDING`, śap at `SHAP` — empty on
//! exactly the path this stage cares about) and BEFORE `anga`, so 6.4.71
//! always sees a finished abhyāsa in `ABHYASA` and 7.1.4 reads a real
//! `Tag::Abhyasta`. See `super::terms`.
//!
//! DVITVA RUNS BEFORE GUṆA. vidyut-prakriya copies the already-guṇated
//! stem and shortens the copy back by 7.4.59 (*ho ho* → *hu ho*); this
//! engine follows the Kaumudī's order — ślu, dvitva, abhyāsa-kārya, then
//! guṇa — and copies the bare root, so 7.4.59 *hrasvaḥ* (slice 3b) fires
//! only on the long-vowel roots it names. The two orders give the same
//! forms for all 26 juhotyādi roots (the spec's appendix probe); the audit
//! compares form sets, not traces, so the divergence is visible only in
//! this engine's own trace pins, which pin THIS order (juhoti: 6.1.10 <
//! 7.4.62 < 7.3.84).
//!
//! 6.1.4 *pūrvo'bhyāsaḥ* and 6.1.5 *ubhe abhyastam* are saṁjñā verdicts
//! and live as tags set here (`Tag::Abhyasa`, `Tag::Abhyasta`), not as
//! steps — the 3.4.113 precedent.

use crate::rule::{Rule, RuleKind};
use crate::term::Tag;
use crate::tinanta::sound::cutva_of;
use crate::tinanta::terms::{ABHYASA, ANGA, SHAP};

pub(crate) static ABHYASA_RULES: &[Rule] = &[
    // 6.1.10 ślau: after ślu the aṅga is doubled (6.1.1 ekāco dve
    // prathamasya, the first ekāc). The copy goes into the permanent
    // `ABHYASA` slot, empty for every other gaṇa; it is tagged Abhyasa
    // (6.1.4 — the earlier of the two) and both it and the root are tagged
    // Abhyasta (6.1.5).
    //
    // NARROW: copies the whole aṅga text. Every juhotyādi root is a single
    // ekāc when this rule fires (hu, ki, BI, dA, … — 6.1.2 ajāder
    // dvitīyasya has no customer here either), so "the first ekāc" and
    // "the aṅga" coincide. liṭ is the first slice that reduplicates a
    // polysyllabic aṅga and must implement the ekāc cut then.
    //
    // Reads Tag::Slu, not "SHAP is empty": adādi's śap is empty too (2.4.72,
    // luk) and does not reduplicate — atti, not *atatti. SHAP is indexed
    // directly, as every post-3.1.68 rule does: the slot always exists here.
    Rule {
        id: "6.1.10",
        name: "SlO",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[SHAP].has(Tag::Slu) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ABHYASA].text = p.terms[ANGA].text.clone();
            p.terms[ABHYASA].add(Tag::Abhyasa);
            p.terms[ABHYASA].add(Tag::Abhyasta);
            p.terms[ANGA].add(Tag::Abhyasta);
            p.record("6.1.10", "SlO", before);
            true
        },
    },
    // 7.4.62 kuhoś cuḥ: the abhyāsa's initial velar or `h` becomes the
    // palatal — ku → cu by place, and h → J (jh), which 8.4.54 abhyāse car
    // ca deaspirates to `j` in the tripādī: hu → Ju → ju (juhoti), ki → ci
    // (ciketi).
    //
    // *abhyāse* (anuvṛtti from 7.4.59) is structurally satisfied: `ABHYASA`
    // is non-empty exactly when 6.1.10 has filled it — the slot is the
    // saṁjñā's whole extension — so a Tag::Abhyasa clause here could never
    // be falsified and is deliberately not written, the same reason 7.4.21
    // omits its *sārvadhātuke* test.
    Rule {
        id: "7.4.62",
        name: "kuhoScuH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let Some(first) = p.terms[ABHYASA].text.chars().next() else {
                return false;
            };
            let Some(cu) = cutva_of(first) else {
                return false;
            };
            let before = p.snapshot();
            let rest: String = p.terms[ABHYASA].text.chars().skip(1).collect();
            p.terms[ABHYASA].text = format!("{cu}{rest}");
            p.record("7.4.62", "kuhoScuH", before);
            true
        },
    },
];
```

In `mod.rs`: add `mod abhyasa;` as the first line of the alphabetical `mod` list (before `mod adesha;`); in `TINANTA_RULES` insert `abhyasa::ABHYASA_RULES,` between `vikarana::VIKARANA,` and `anga::ANGA_RULES,`; change the module doc's "seven ordered rule-stage modules" / "The seven stages — `samjna`, `tin`, `vikarana`, `anga`, `guna`, `adesha`, `tripadi`" to "eight ordered rule-stage modules" / "The eight stages — `samjna`, `tin`, `vikarana`, `abhyasa`, `anga`, `guna`, `adesha`, `tripadi`".

In `terms.rs`, delete the two comment lines and the attribute above `pub(crate) const ABHYASA: usize = 1;` (`// Unused outside tests until slice 3a (6.1.10) lands. Remove this allow` / `// once a rule body reads or writes `ABHYASA`.` / `#[allow(dead_code)]`), and change the doc's "(and, until slice 3a lands, for every derivation)" to "(6.1.10 fills it)".

- [ ] **Step 4: Unit suite and lint**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -5` — Expected: all pass (102 pinned ids).
Run: `mise run lint && mise run fmt-check` — Expected: clean (in particular no `dead_code` on `ABHYASA` and no unused import).

- [ ] **Step 5: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/abhyasa.rs crates/panini-prakriya/src/tinanta/mod.rs crates/panini-prakriya/src/tinanta/sound.rs crates/panini-prakriya/src/tinanta/terms.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(engine): the abhyasa stage — 6.1.10 slau fills ABHYASA, 7.4.62 kuhos cuh

Dvitva before guṇa (Kaumudī order; forms identical to vidyut's, trace order
not). Tag::Abhyasa / Tag::Abhyasta set as saṁjñā tags, pinned by test."
```

---

### Task 3: 3.4.109 jus in laṅ and 7.1.4 ad abhyastāt

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tin.rs` (imports; new rule after 3.4.108, before 3.4.105)
- Modify: `crates/panini-prakriya/src/tinanta/anga.rs` (new rule immediately before 7.1.3)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (pinned order)

**Interfaces:**
- Consumes: `Tag::Juhotyadi` on `ANGA` (Task 1), `Tag::Abhyasta` on `ANGA` (Task 2).
- Produces: in laṅ, a juhotyādi `Ji` at `ENDING_PRE_SHAP` becomes `us` (traced 3.4.109 then 1.3.9) before 3.4.100 can touch it; after 3.1.68, an abhyasta aṅga's `J`-initial ending becomes `at…` (`Ji` → `ati`, `Ju` → `atu`) and 7.1.3 declines.

- [ ] **Step 1: Write the failing tests**

In `tin.rs`'s `tests` module, after `jher_jus_leaves_lat_and_lot_ji_alone`:

```rust
    #[test]
    fn sijabhyasta_jus_replaces_lan_jhi_for_juhotyadi_only() {
        // 3.4.109. laṅ Ji → jus → us for a juhotyādi aṅga, traced with its
        // 1.3.9 like 3.4.108; laṭ and loṭ keep Ji (7.1.4 and 3.4.86 want
        // it), vidhiliṅ is 3.4.108's, and a non-juhotyādi laṅ Ji is
        // 3.4.100 + 7.1.3's business (aBavan).
        let rule = rules().find(|r| r.id == "3.4.109").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("hu"), Term::new("Ji")]),
            log: vec![],
            ctx: Context::new(Lakara::Lan, Pada::Parasmaipada, Purusha::Prathama, Vacana::Bahu),
            blocked: false,
        };
        p.terms[ANGA].add(Tag::Juhotyadi);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "us");
        assert!(p.log.iter().any(|s| s.sutra == "3.4.109"));
        assert!(p.log.iter().any(|s| s.sutra == "1.3.9"));

        for lakara in [Lakara::Lat, Lakara::Lot, Lakara::VidhiLin] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new("hu"), Term::new("Ji")]),
                log: vec![],
                ctx: Context::new(lakara, Pada::Parasmaipada, Purusha::Prathama, Vacana::Bahu),
                blocked: false,
            };
            p.terms[ANGA].add(Tag::Juhotyadi);
            assert!(!(rule.apply)(&mut p), "{lakara:?}");
            assert_eq!(p.terms[ENDING_PRE_SHAP].text, "Ji");
        }
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("BU"), Term::new("Ji")]),
            log: vec![],
            ctx: Context::new(Lakara::Lan, Pada::Parasmaipada, Purusha::Prathama, Vacana::Bahu),
            blocked: false,
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "Ji");
    }
```

In `anga.rs`'s `tests` module, after `aat_declines_by_the_angas_own_shape_with_no_log_lookup`:

```rust
    #[test]
    fn ad_abhyastat_preempts_jho_ntah_after_an_abhyasta_anga() {
        // Ji → ati (laṭ), Ju → atu (loṭ) when ANGA is abhyasta; 7.1.3 then
        // finds no J. Without the tag, 7.1.3's ant is the answer (Bavanti).
        let r_4 = rules().find(|r| r.id == "7.1.4").unwrap();
        let r_3 = rules().find(|r| r.id == "7.1.3").unwrap();
        for (ending, expected) in [("Ji", "ati"), ("Ju", "atu")] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new("hu"), Term::new(""), Term::new(ending)]),
                ..Default::default()
            };
            p.terms[ANGA].add(Tag::Abhyasta);
            assert!((r_4.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ENDING].text, expected);
            assert_eq!(p.log.last().unwrap().sutra, "7.1.4");
            assert!(!(r_3.apply)(&mut p), "7.1.3 must find no J after 7.1.4");
            assert_eq!(p.terms[ENDING].text, expected);
        }
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("BU"), Term::new("a"), Term::new("Ji")]),
            ..Default::default()
        };
        assert!(!(r_4.apply)(&mut p));
        assert!((r_3.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "anti");
        // An abhyasta aṅga before a J-less ending: nothing to do (juhoti).
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("hu"), Term::new(""), Term::new("ti")]),
            ..Default::default()
        };
        p.terms[ANGA].add(Tag::Abhyasta);
        assert!(!(r_4.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "ti");
    }
```

Pinned order: insert `"3.4.109"` after `"3.4.108"`, and `"7.1.4"` after `"7.1.6"` (before `"7.1.3"`).

- [ ] **Step 2: Run to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya sijabhyasta ad_abhyastat 2>&1 | tail -8`
Expected: the two new tests panic at `.find(...).unwrap()` (no such rule id); `tinanta_rule_order_is_pinned` fails on the two missing ids.

- [ ] **Step 3: 3.4.109**

`tin.rs`: change the terms import to `use crate::tinanta::terms::{ANGA, ENDING_PRE_SHAP};`. Insert immediately after the 3.4.108 `Rule { ... }`:

```rust
    // 3.4.109 sijabhyastavidibhyaś ca: after an abhyasta aṅga, laṅ's Ji is
    // replaced by jus — ajuhavuH, acikayuH — the laṅ counterpart of 3.4.108's
    // liṅ rule, and like it an apavāda to 3.4.100 itaś ca, hence ordered
    // before it. The j is elided and recorded as 1.3.9 exactly as 3.4.108
    // does. Guarded on Lakara::Lan itself, NOT ctx.is_ngit_like: loṭ is
    // laṅvat for 3.4.99–101 only, and its Ji must survive for 3.4.86 and
    // 7.1.4 (juhvatu).
    //
    // THE GUARD IS A STAND-IN. This rule runs before 3.1.68 — before 2.4.75
    // and 6.1.10 exist — so no term is abhyasta yet; it reads Tag::Juhotyadi
    // instead, which every cell of this gaṇa in these four lakāras entails
    // (ślu → dvitva, unconditionally). vidyut-prakriya applies the rule
    // after dvitva and reads a real abhyasta tag. The equivalence breaks the
    // moment a lakāra reduplicates CONDITIONALLY, or an aṅga outside gaṇa 3
    // is abhyasta in laṅ; liṭ is the slice that must revisit this (its
    // abhyasta aṅgas are everyone's), and the *sic* and *vid* arms are
    // that slice's too.
    Rule {
        id: "3.4.109",
        name: "sijaByastavidiByaSca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lan)
                || !p.terms[ANGA].has(Tag::Juhotyadi)
                || p.terms[ENDING_PRE_SHAP].text != "Ji"
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "jus".into();
            p.record("3.4.109", "sijaByastavidiByaSca", before);
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "us".into();
            p.record("1.3.9", "tasya lopaH", before);
            true
        },
    },
```

Add `3.4.109` to the module doc's first line ("3.4.85 … 3.4.102, 3.4.109, 7.1.35" — match the existing phrasing).

- [ ] **Step 4: 7.1.4**

`anga.rs`: insert immediately before the `// 7.1.3 jho'ntaḥ` comment:

```rust
    // 7.1.4 ad abhyastāt: after an abhyasta aṅga the jh of the ending is
    // replaced by `at`, not by 7.1.3's `ant` — juhvati, cikyati; juhvatu,
    // cikyatu. Apavāda to 7.1.3 and ordered before it; self-guarding in
    // the usual way, since once the J is gone 7.1.3 has nothing to match.
    // Reads Tag::Abhyasta on ANGA: the aṅga before the tiṅ affix (1.4.13)
    // is the abhyasta pair, and 6.1.10 tagged its root half.
    Rule {
        id: "7.1.4",
        name: "ad aByastAt",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Abhyasta) || !p.terms[ENDING].text.starts_with('J') {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("at{rest}");
            p.record("7.1.4", "ad aByastAt", before);
            true
        },
    },
```

- [ ] **Step 5: Unit suite and lint**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -5` — Expected: all pass (104 pinned ids).
Run: `mise run lint && mise run fmt-check` — Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/tin.rs crates/panini-prakriya/src/tinanta/anga.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(engine): 3.4.109 jus for lan after an abhyasta anga; 7.1.4 ad abhyastat before 7.1.3

3.4.109 guards on Tag::Juhotyadi as the pre-dvitva stand-in for abhyasta;
the comment names liṭ as the slice that must revisit it."
```

---

### Task 4: guṇa-stage rules — 7.3.83, 6.4.87's hu arm, 6.4.82

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (imports; 7.3.83 before the first 7.3.84; 6.4.87's body and comment; 6.4.82 between 6.4.87 and 6.4.77)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (pinned order)

**Interfaces:**
- Consumes: `following_sarvadhatuka`, `guna_of`, `is_vowel`, `ABHYASA`/`ANGA`/`ENDING`/`SHAP`.
- Produces: `hu` + `us` → `ho` (7.3.83) so 6.1.78's athematic arm can give `hav`; `hu` + vowel-initial ending → `hv` (6.4.87, hu arm, ANGA rewritten — the śnu arm still rewrites SHAP); `ci`+`ki` + vowel-initial ending → `ci`+`ky` (6.4.82).

- [ ] **Step 1: Write the failing tests**

In `guna.rs`'s `tests` module, after `shnu_vowel_rules_decline_before_a_consonant_ending`:

```rust
    // --- 7.3.83 jusi ca ----------------------------------------------------

    #[test]
    fn jusi_ca_gunates_before_a_bare_us_and_7_3_84_then_declines() {
        // hu + "" + us → ho + us (ajuhavuH after 6.1.78). us is ṅit, so
        // 7.3.84 alone would leave *ajuhuvuH; and once 7.3.83 has written
        // `o`, 7.3.84 must not record a second step.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("hu"), Term::new(""), Term::new("us")]),
            ..Default::default()
        };
        p.terms[ENDING].add(Tag::Ngit);
        let r_83 = rules().find(|r| r.id == "7.3.83").unwrap();
        assert!((r_83.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "ho");
        assert_eq!(p.log.last().unwrap().sutra, "7.3.83");
        let r_84 = rules().find(|r| r.id == "7.3.84").unwrap();
        assert!(!(r_84.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "ho");
    }

    #[test]
    fn jusi_ca_wants_jus_immediately_after_the_anga() {
        let rule = rules().find(|r| r.id == "7.3.83").unwrap();
        for (root, shap, ending, why) in [
            ("hu", "", "yAus", "vidhiliṅ: yāsuṭ intervenes (juhuyuH)"),
            ("hu", "a", "us", "a live vikaraṇa is the follower"),
            ("yA", "", "us", "3.4.111's jus after an ā-final aṅga: no ik"),
            ("hu", "", "taH", "not jus at all"),
        ] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new(root), Term::new(shap), Term::new(ending)]),
                ..Default::default()
            };
            assert!(!(rule.apply)(&mut p), "{why}");
            assert_eq!(p.terms[ANGA].text, root, "{why}");
        }
    }

    // --- 6.4.87's hu arm ---------------------------------------------------

    #[test]
    fn hushnuvoh_hu_arm_writes_yan_into_the_root_before_a_vowel() {
        // hu + "" + ati → hv + ati (juhvati). Consonant-initial endings, the
        // guṇated `ho` (juhavAni's shape here) and any other u-final root
        // with an empty śap decline; the śnu arm is untouched.
        let rule = rules().find(|r| r.id == "6.4.87").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("hu"), Term::new(""), Term::new("ati")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "hv");
        assert_eq!(p.terms[SHAP].text, "");
        assert_eq!(p.log.last().unwrap().sutra, "6.4.87");
        for (root, ending) in [("hu", "taH"), ("ho", "Ani"), ("su", "ati")] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new(root), Term::new(""), Term::new(ending)]),
                ..Default::default()
            };
            assert!(!(rule.apply)(&mut p), "{root}+{ending}");
            assert_eq!(p.terms[ANGA].text, root);
        }
    }

    // --- 6.4.82 er anekāco'saṁyogapūrvasya ---------------------------------

    #[test]
    fn er_anekaco_yan_for_the_reduplicated_i_final_anga() {
        // ci + ki + "" + ati → ci + ky + ati (cikyati). 6.4.77's śnu arm
        // must not be what fires: SHAP is empty, not `nu`.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ki"), Term::new(""), Term::new("ati")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "ci".into();
        let rule = rules().find(|r| r.id == "6.4.82").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "ky");
        assert_eq!(p.text(), "cikyati");
        assert_eq!(p.log.last().unwrap().sutra, "6.4.82");
        let utsarga = rules().find(|r| r.id == "6.4.77").unwrap();
        assert!(!(utsarga.apply)(&mut p));
    }

    #[test]
    fn er_anekaco_declines_on_each_of_its_conditions() {
        let rule = rules().find(|r| r.id == "6.4.82").unwrap();
        for (abhyasa, anga, ending, why) in [
            ("", "ci", "ati", "ekāc: one vowel over the whole aṅga"),
            ("ci", "kri", "ati", "saṁyogapūrva: kr before the i"),
            ("ci", "ki", "yAt", "yāsuṭ, consonant-initial, is the follower"),
            ("ci", "ki", "taH", "consonant-initial ending"),
            ("ci", "ke", "ati", "not i-final (guṇa already applied)"),
        ] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new(anga), Term::new(""), Term::new(ending)]),
                ..Default::default()
            };
            p.terms[ABHYASA].text = abhyasa.into();
            assert!(!(rule.apply)(&mut p), "{why}");
            assert_eq!(p.terms[ANGA].text, anga, "{why}");
        }
        // The vowel-before-i arm of asaṁyogapūrva, pinned on its own: no
        // curated root reaches it (every juhotyādi i-final root has a
        // consonant before its i), so it is exercised synthetically, as
        // `vikarana_u_asamyogapurva`'s own test does for its vowel arm.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("i"), Term::new(""), Term::new("ati")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "a".into();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "y");
    }
```

Pinned order: insert `"7.3.83"` after `"7.4.21"` (before the first `"7.3.84"`), and `"6.4.82"` after `"6.4.87"` (before `"6.4.77"`).

- [ ] **Step 2: Run to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya jusi_ca hushnuvoh_hu er_anekaco 2>&1 | tail -8`
Expected: `jusi_ca_*` and `er_anekaco_*` panic on the missing rule id; `hushnuvoh_hu_arm_*` fails its first `assert!((rule.apply)(&mut p))` (the śnu-only guard declines on an empty SHAP).

- [ ] **Step 3: 7.3.83**

`guna.rs`: change the terms import to `use crate::tinanta::terms::{ABHYASA, ANGA, ENDING, SHAP, following_sarvadhatuka, vikarana_u_asamyogapurva};`. Insert immediately before the `// 7.3.84 sārvadhātukārdhadhātukayoḥ: guṇa of the aṅga's final ik.` comment (the FIRST 7.3.84, after 7.4.21):

```rust
    // 7.3.83 jusi ca: guṇa of the aṅga's final ik before jus — ajuhavuH,
    // acikayuH — overriding the 1.1.5 block that jus, apit and so ṅit by
    // 1.2.4, would impose on 7.3.84. Ordered immediately before 7.3.84 as
    // its apavāda, the 7.4.21 shape: the trace credits the guṇa to the
    // sūtra that licenses it, and 7.3.84 then declines on the guṇated vowel
    // by its own shape guard.
    //
    // "Before jus" means IMMEDIATELY before: the follower is read with
    // `following_sarvadhatuka`, and its text must be the bare `us` that
    // 3.4.108 / 3.4.109 leave after 1.3.9. Vidhiliṅ's jus never matches —
    // yāsuṭ sits in front of it (`yAus` at this point; 6.1.96 makes it
    // `yus` later), so juhuyuH, not *juhoyuH. The one other laṅ `us` in
    // scope, 3.4.111's optional jus after an ā-final adādi aṅga (ayuH), has
    // no ik to guṇate and declines on `guna_of`. No lakāra clause: the
    // follower's text already separates every case, and a clause no cell
    // could falsify would be a mutation survivor.
    Rule {
        id: "7.3.83",
        name: "jusi ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !following_sarvadhatuka(p).is_some_and(|t| t.text == "us") {
                return false;
            }
            let last = p.terms[ANGA].text.chars().last().unwrap();
            let Some(g) = guna_of(last) else {
                return false;
            };
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ANGA].text.chars().collect();
            s.pop();
            p.terms[ANGA].text = s.into_iter().collect::<String>() + g;
            p.record("7.3.83", "jusi ca", before);
            true
        },
    },
```

- [ ] **Step 4: 6.4.87's hu arm**

In 6.4.87's comment, replace the paragraph "The √hu arm is not implemented: √hu is juhotyādi, out of scope. Widen when gaṇa 3 lands." with:

```rust
    // The √hu arm (slice 3a): the sūtra names the root, and the yaṇ goes
    // into the ROOT's own final — hu + ati → hv + ati, juhvati — where the
    // śnu arm rewrites the vikaraṇa. `ANGA.text == "hu"` alone identifies
    // it: no other curated root reads `hu`, so a Tag::Juhotyadi clause
    // beside it could never be falsified (the 7.4.21 reasoning) and is
    // deliberately not written. `==`, not `ends_with`: ANGA is the root's
    // own text (terms.rs). Ordered after 7.3.84, so the pit cells have
    // already guṇated to `ho` and decline here (juhavAni goes to 6.1.78).
```

Replace the start of its `apply` body so the whole closure reads:

```rust
        apply: |p| {
            if p.terms[ANGA].text == "hu" {
                let Some(next) = p.terms[ENDING].text.chars().next() else {
                    return false;
                };
                if !is_vowel(next) {
                    return false;
                }
                let before = p.snapshot();
                p.terms[ANGA].text = "hv".into();
                p.record("6.4.87", "huSnuvoH sArvaDAtuke", before);
                return true;
            }
            // The śnu arm. The sūtra names hu and śnu; tanādi's bare `u`
            // (which the shared asaṁyogapūrva helper now also admits) is
            // 6.1.77's business below — without this test 6.4.87 would
            // write śnu's `nv` over a vikaraṇa that has no `n`.
            if p.terms[SHAP].text != "nu" {
                return false;
            }
            if !vikarana_u_asamyogapurva(p) {
                return false;
            }
            let Some(next) = p.terms[ENDING].text.chars().next() else {
                return false;
            };
            if !is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "nv".into();
            p.record("6.4.87", "huSnuvoH sArvaDAtuke", before);
            true
        },
```

- [ ] **Step 5: 6.4.82**

Insert between the 6.4.87 `Rule { ... }` and the `// 6.4.77 aci śnudhātubhruvāṁ` comment:

```rust
    // 6.4.82 er anekāco'saṁyogapūrvasya: a final `i` of a polysyllabic
    // aṅga, not preceded by a conjunct, becomes `y` (yaṇ) before a vowel —
    // cikyati, cikyatu — where 6.4.77's dhātu arm (iyaṅ) would otherwise
    // apply (as it will for √hrī's conjunct-preceded ī, jihriyati, in slice
    // 3b). Apavāda to 6.4.77, ordered before it.
    //
    // *Anekāc* is counted over ABHYASA + ANGA together: after 6.1.10 the
    // aṅga before the affix is the abhyasta pair (1.4.13), and ci-ki has two
    // vowels where ki alone has one. *Asaṁyogapūrva*: the sound before the
    // final `i` is a single consonant, itself after a vowel (the k of
    // ci-ki) — or a vowel. The follower is the first non-empty term after
    // ANGA, which on the ślu path is the ending itself; a yāsuṭ (`yA…`) or
    // a consonant-initial ending declines it (cikiyAt, cikitaH).
    //
    // ORDER: after 7.3.84, or the loṭ uttama cells break — Ani is pit, so
    // cikayAni takes guṇa (ke) and then 6.1.78 (kay); a 6.4.82 that saw
    // the `i` first would write *cikyAni. The cikayAni trace pin holds this.
    Rule {
        id: "6.4.82",
        name: "er anekAco'saMyogapUrvasya",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let mut anga: Vec<char> = p.terms[ANGA].text.chars().collect();
            if anga.last() != Some(&'i') {
                return false;
            }
            let stem: Vec<char> = p.terms[ABHYASA]
                .text
                .chars()
                .chain(p.terms[ANGA].text.chars())
                .collect();
            if stem.iter().filter(|&&c| is_vowel(c)).count() < 2 {
                return false;
            }
            // The two sounds before the final i, nearest first.
            let n = stem.len();
            let before_i = n.checked_sub(2).and_then(|i| stem.get(i)).copied();
            let before_that = n.checked_sub(3).and_then(|i| stem.get(i)).copied();
            let asamyogapurva = match (before_i, before_that) {
                (Some(v), _) if is_vowel(v) => true,
                (Some(c), Some(v)) if !is_vowel(c) && is_vowel(v) => true,
                _ => false,
            };
            if !asamyogapurva {
                return false;
            }
            let Some(next) = p.terms[ANGA + 1..]
                .iter()
                .find(|t| !t.text.is_empty())
                .and_then(|t| t.text.chars().next())
            else {
                return false;
            };
            if !is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            anga.pop();
            anga.push('y');
            p.terms[ANGA].text = anga.into_iter().collect();
            p.record("6.4.82", "er anekAco'saMyogapUrvasya", before);
            true
        },
    },
```

Also update the 6.4.77 comment's "The *dhātu* arm (ī/ū-final roots) and the *bhrū* arm have no root in scope" to add ": 6.4.82 above is the dhātu arm's apavāda for the asaṁyogapūrva i-final case (√ki); the iyaṅ case itself arrives with √hrī in slice 3b".

- [ ] **Step 6: Unit suite, lint, FULL suite**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -5` — Expected: all pass (106 pinned ids).
Run: `mise run lint && mise run fmt-check` — Expected: clean.
Run: `mise run test` (foreground, ~34 min). Expected: PASS; `git status --short crates/panini/tests` empty. This is the first task with general-shape guards (7.3.83's `us` follower, 6.4.82's vowel count); a moved prior is a guard that is wider than argued above — fix the guard, never the golden.

- [ ] **Step 7: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/guna.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(engine): 7.3.83 jusi ca, 6.4.87's hu arm, 6.4.82 er anekaco — the guna-stage core for juhoti/juhvati/cikyati

3492 priors byte-identical (full suite)."
```

---

### Task 5: 6.4.101's hu arm, 8.4.54 abhyāse car ca, and the augment-guard argument

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/adesha.rs` (6.4.101's comment and guard; import `ANGA` if not already imported)
- Modify: `crates/panini-prakriya/src/tinanta/sound.rs` (`deaspirate_of` after `cutva_of`; test)
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (imports; 8.4.54 after 8.4.53)
- Modify: `crates/panini-prakriya/src/tinanta/anga.rs` (6.4.71 and 6.4.72 comments only)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (pinned order)

**Interfaces:**
- Consumes: `sound_before_ending`, `is_jhal`, `ABHYASA`.
- Produces: `hu` + `hi` → `Di` (6.4.101); an aspirate in `ABHYASA` deaspirated (`Ju` → `ju`) with no step when nothing changes (8.4.54); `pub(crate) fn deaspirate_of(c: char) -> Option<char>`.

- [ ] **Step 1: Write the failing tests**

In `sound.rs`'s tests, after `cutva_of_ku_and_h_all_arms`:

```rust
    #[test]
    fn deaspirate_of_aspirate_stops_all_arms() {
        // 8.4.54 abhyāse car ca: the abhyāsa's jhal becomes car (and jaś by
        // 8.4.53's anuvṛtti) — observable exactly on the ten aspirates,
        // since an unaspirated stop's car/jaś is itself. Only J -> j (√hu)
        // is reachable from the golden forms in slice 3a, B -> b in 3b; the
        // rest are pinned here so they cannot rot.
        for (from, to) in [
            ('K', 'k'), ('G', 'g'), ('C', 'c'), ('J', 'j'), ('W', 'w'),
            ('Q', 'q'), ('T', 't'), ('D', 'd'), ('P', 'p'), ('B', 'b'),
        ] {
            assert_eq!(deaspirate_of(from), Some(to), "{from}");
        }
        // Already car/jaś, or not a stop at all: no substitute, so 8.4.54
        // can use this as its match test too.
        for c in ['k', 'g', 'c', 'j', 't', 'd', 'p', 'b', 's', 'S', 'z', 'h', 'n', 'a', 'i', 'u'] {
            assert_eq!(deaspirate_of(c), None, "{c}");
        }
    }
```

In `adesha.rs`'s tests, after `her_dhih_declines_for_kryadi_shni`:

```rust
    #[test]
    fn her_dhih_hu_arm_fires_on_the_named_root_and_not_on_ki() {
        // hu + "" + hi → hu + Di (juhuDi): `u` is no jhal, so this is the
        // sūtra's own *hu* arm. √ki keeps hi (cikihi) — i is no jhal and the
        // root is not √hu.
        let rule = rules().find(|r| r.id == "6.4.101").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("hu"), Term::new(""), Term::new("hi")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "Di");
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ki"), Term::new(""), Term::new("hi")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "hi");
    }
```

In `tripadi.rs`'s tests, after `jhalam_jash_jhashi_fires_anywhere_not_just_word_finally`:

```rust
    #[test]
    fn abhyase_car_ca_deaspirates_the_abhyasa_and_nothing_else() {
        // Ju + ho + "" + ti → ju + ho + ti (juhoti): the abhyāsa's J goes to
        // j; the root's own h is not an abhyāsa sound and stays.
        let rule = rules().find(|r| r.id == "8.4.54").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ho"), Term::new(""), Term::new("ti")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "Ju".into();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "ju");
        assert_eq!(p.text(), "juhoti");
        assert_eq!(p.log.last().unwrap().sutra, "8.4.54");
        // Already car (√ki's ci) and no abhyāsa at all: no step recorded.
        for abhyasa in ["ci", ""] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new("ke"), Term::new(""), Term::new("ti")]),
                ..Default::default()
            };
            p.terms[ABHYASA].text = abhyasa.into();
            assert!(!(rule.apply)(&mut p), "{abhyasa:?}");
            assert_eq!(p.terms[ABHYASA].text, abhyasa);
            assert!(p.log.is_empty(), "{abhyasa:?}");
        }
    }
```

Pinned order: insert `"8.4.54"` after `"8.4.53"`.

- [ ] **Step 2: Run to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya deaspirate her_dhih_hu abhyase_car 2>&1 | tail -8`
Expected: compile error on `deaspirate_of`; once that is stubbed, `her_dhih_hu_arm_*` fails its first assert and `abhyase_car_ca_*` panics on the missing id.

- [ ] **Step 3: `deaspirate_of`**

In `sound.rs`, after `cutva_of`:

```rust
/// The unaspirated stop of the same place and voicing — 8.4.54 abhyāse car
/// ca's substitute. The sūtra says the abhyāsa's jhal becomes car, and jaś
/// by 8.4.53's anuvṛtti; for an unaspirated stop or a sibilant that
/// substitute is the sound itself, so the change is observable exactly on
/// the ten aspirates, and `None` for everything else lets 8.4.54 use this
/// one lookup as its match test as well (the `kutva_of` / `jashtva_of`
/// idiom). `h` is a jhal too but never reaches 8.4.54 in an abhyāsa: 7.4.62
/// has already made it `J`, which is why `J -> j` is the one arm √hu
/// exercises (juhoti); `B -> b` is slice 3b's (√bhī).
pub(crate) fn deaspirate_of(c: char) -> Option<char> {
    Some(match c {
        'K' => 'k',
        'G' => 'g',
        'C' => 'c',
        'J' => 'j',
        'W' => 'w',
        'Q' => 'q',
        'T' => 't',
        'D' => 'd',
        'P' => 'p',
        'B' => 'b',
        _ => return None,
    })
}
```

- [ ] **Step 4: 6.4.101's hu arm**

In `adesha.rs`, replace 6.4.101's comment with:

```rust
    // 6.4.101 hujhalbhyo her dhiḥ: the loṭ 2sg `hi` becomes `Di` after a
    // jhal-final aṅga and after √hu. √ad: 6.4.105 ato heḥ declined (its aṅga
    // ends in `d`, not a short `a`), so `hi` survives to here → adDi. √hu:
    // juhuDi — the root is named by the sūtra and `u` is no jhal, so it is
    // its own arm, `ANGA.text == "hu"`, the same root-keyed test 6.4.87's hu
    // arm uses and for the same reason with no gaṇa clause beside it (no
    // other curated root reads `hu`; see 7.4.21). Thematic roots never
    // reach this — their `hi` is luk'd by 6.4.105 behind śap's `a` — and
    // √ki's stays (cikihi: `i` is no jhal and the root is not √hu).
```

and its `apply` body with:

```rust
        apply: |p| {
            if p.terms[ENDING].text != "hi" {
                return false;
            }
            if p.terms[ANGA].text != "hu" {
                // NOT terms[ANGA] for the jhal test. The jhal this sūtra
                // tests is the sound the ending attaches to, which for a
                // gaṇa with a live vikaraṇa is the vikaraṇa's final, not
                // the root's. Reading ANGA fired on √āp's `p` and √śak's
                // `k` and gave *ApnuDi / *SaknuDi, even though śnu's `u`
                // sits between. adādi still reaches the root because its
                // śap is empty and the helper walks past it.
                let Some(last) = sound_before_ending(p) else {
                    return false;
                };
                if !is_jhal(last) {
                    return false;
                }
            }
            let before = p.snapshot();
            p.terms[ENDING].text = "Di".into();
            p.record("6.4.101", "her DiH", before);
            true
        },
```

Make sure `ANGA` is in `adesha.rs`'s `use crate::tinanta::terms::{...}` import (it already is if 6.1.101's rule body compiles against it; add it otherwise).

- [ ] **Step 5: 8.4.54**

`tripadi.rs`: add `ABHYASA` to the terms import and `deaspirate_of` to the sound import. Insert immediately after the 8.4.53 `Rule { ... }` and before the `// 8.4.55 khari ca (cartva)` comment:

```rust
    // 8.4.54 abhyāse car ca: a jhal in the abhyāsa becomes its car — and its
    // jaś, by 8.4.53's anuvṛtti (jhalāṁ jaś) — i.e. the abhyāsa loses its
    // aspiration: Ju → ju (juhoti; the `J` 7.4.62 wrote for h), and in slice
    // 3b BI → bI (bibheti). Reads the ABHYASA slot directly and whole:
    // *abhyāse* is the slot (non-empty exactly when 6.1.10 filled it — see
    // 7.4.62 in abhyasa.rs), and every aspirate in it is deaspirated in one
    // step. The no-op guard is 8.4.53's: √ki's abhyāsa `ci` is already car
    // and the rule must record nothing there — vidyut-prakriya credits
    // 8.4.54 on √hu's 42 forms and on none of √ki's.
    Rule {
        id: "8.4.54",
        name: "aByAse car ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let s: Vec<char> = p.terms[ABHYASA].text.chars().collect();
            let t: Vec<char> = s.iter().map(|&c| deaspirate_of(c).unwrap_or(c)).collect();
            if t == s {
                return false;
            }
            let before = p.snapshot();
            p.terms[ABHYASA].text = t.into_iter().collect();
            p.record("8.4.54", "aByAse car ca", before);
            true
        },
    },
```

Add `8.4.54` to the file's module doc range if it enumerates ids (it says "8.2.77 … 8.4.56"; leave it).

- [ ] **Step 6: The augment-guard argument (comments only)**

In `anga.rs`, append to 6.4.71's comment block (after "…see their comments.)"):

```rust
    //
    // READS `ANGA`, NOT THE FIRST NON-EMPTY TERM AFTER `AGAMA`. Grammatically
    // the augment precedes the whole aṅga, abhyāsa included, and 6.1.90
    // already merges the āṭ into the first non-empty term after the slot.
    // The consonant/vowel verdict is the same either way for every
    // juhotyādi row: the abhyāsa is a copy of the root's first ekāc, and no
    // rule in 7.4.59–7.4.78 changes its initial's class — 7.4.60 keeps the
    // first consonant, 7.4.62 substitutes consonant for consonant, 7.4.66
    // and 7.4.77 vowel for vowel. Reading the abhyāsa instead would add a
    // clause no 3a root can falsify (both are consonant-initial), so the
    // ANGA read stays; slice 3d's √ṛ (iyarti, aiyaḥ) is the vowel-initial
    // row that re-checks this argument against a live witness.
```

Append to 6.4.72's comment block one sentence: `// Reads ANGA's own initial for the same reason 6.4.71 does — see its comment.`

- [ ] **Step 7: Unit suite, lint, FULL suite**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -5` — Expected: all pass (107 pinned ids).
Run: `mise run lint && mise run fmt-check` — Expected: clean.
Run: `mise run test` (foreground). Expected: PASS; `git status --short crates/panini/tests` empty. 8.4.54 is a tripādī rule reading a slot that is empty for every prior, and 6.4.101's new arm keys on a root text no prior carries; a moved prior here is a defect.

- [ ] **Step 8: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/adesha.rs crates/panini-prakriya/src/tinanta/sound.rs crates/panini-prakriya/src/tinanta/tripadi.rs crates/panini-prakriya/src/tinanta/anga.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(engine): 6.4.101's hu arm (juhuDi), 8.4.54 abhyase car ca; the augment guards argue their ANGA read

Eight new rules and two widened arms are now in place; 3492 priors
byte-identical (full suite). No root carries the gaṇa yet."
```

---

### Task 6: the two data rows, whole-derivation tests, and the GATED window

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (two `Dhatu` rows appended after `08.0010`; `curated_roots_have_expected_ganas_and_padas`; new row test; the two doc-comment counts near lines 99 and 112)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (whole-derivation tests)
- Modify: `crates/panini/tests/paradigm/main.rs` (`GATED` in `paradigm_covers_every_enumerable_cell`)

**Interfaces:**
- Consumes: every rule from Tasks 1–5.
- Produces: rows addressable as `03.0001` (`hu`) and `03.0020` (`ki`), both `PadaAssignment::Parasmaipada`, `Gana::Juhotyadi`; `dhatus().len() == 79`. Tasks 7–9 resolve them through `dhatus()`.

- [ ] **Step 1: The failing row test and count**

In `panini-data/src/lib.rs`'s tests, after `tanadi_rows_are_the_ten_curated_roots`:

```rust
    #[test]
    fn juhotyadi_rows_are_the_two_curated_roots() {
        // Slice 3a: the ślu gaṇa opens with its eponym √hu and √ki, the two
        // roots that exercise dvitva, 7.4.62, 7.1.4, 3.4.109/7.3.83, 6.4.82,
        // 6.4.87's and 6.4.101's hu arms and 8.4.54 with nothing else. Both
        // parasmaipadī by 1.3.78: the `\` in `hu\` / `ki\` sits on the root
        // vowel (svara), not on an it. The gaṇa is PARTIAL at 2 of its 26
        // dhātupāṭha rows; slices 3b–3f close it (spec, "Later slices").
        let rows: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.gana == Gana::Juhotyadi)
            .map(|d| (d.dhatupatha, d.code, d.pada))
            .collect();
        assert_eq!(
            rows,
            vec![
                ("03.0001", "hu", PadaAssignment::Parasmaipada),
                ("03.0020", "ki", PadaAssignment::Parasmaipada),
            ]
        );
    }
```

In `curated_roots_have_expected_ganas_and_padas`, change `assert_eq!(dhatus().len(), 77);` to `79`.

- [ ] **Step 2: Run to verify they fail**

Run: `mise exec -- cargo test -p panini-data juhotyadi_rows curated_roots_have 2>&1 | tail -8`
Expected: both FAIL (empty vec; 77 ≠ 79).

- [ ] **Step 3: The rows**

Append to `DHATUS` after the `08.0010` entry (before the closing `];`):

```rust
    Dhatu {
        // 03.0001 hu\ dAnAdAnayoH AdAne prIRane ca. The ślu gaṇa's eponym
        // (juhoti): 2.4.75 ślu, 6.1.10 dvitva, 7.4.62 kuhoś cuḥ on the
        // abhyāsa (hu → Ju, then 8.4.54 → ju), 6.4.87's hu arm (juhvati),
        // 6.4.101's hu arm (juhuDi), 3.4.109 + 7.3.83 in laṅ (ajuhavuH).
        // The `\` sits on the root vowel — svara, not an anubandha — so
        // 1.3.78 → parasmaipadī. Slice 3a.
        dhatupatha: "03.0001",
        code: "hu",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "dAnAdAnayoH AdAne prIRane ca",
    },
    Dhatu {
        // 03.0020 ki\ jYAne. ciketi / cikyati — the i-final witness for
        // 6.4.82 er anekāco'saṁyogapūrvasya (ci-ki is anekāc, k is no
        // conjunct) and the abhyāsa that 8.4.54 must leave alone (ci is
        // already car). Parasmaipadī by 1.3.78 as √hu. Slice 3a.
        dhatupatha: "03.0020",
        code: "ki",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "jYAne",
    },
```

Update the `pada` field's doc comment: "re-derives 76 of these 77 verdicts" → "78 of these 79"; "The test covers the 77 roots curated here" → "79 roots".

- [ ] **Step 4: panini-data suite**

Run: `mise exec -- cargo test -p panini-data 2>&1 | tail -5`
Expected: all pass — in particular `dhatupatha_numbers_resolve_upstream` (`hu\` it-strips to `hu`, `ki\` to `ki`; each unique in gaṇa 03 with its artha — `03.0021 kita~ jYAne` stores as `kit`, so `ki`/`jYAne` is unique) and `curated_pada_agrees_with_upadesha_markers` (root-vowel `\` derives Parasmaipada, the `ji\` precedent).

- [ ] **Step 5: Whole-derivation tests**

In `derivation_tests.rs`, after `her_dhih_gives_addhi_for_consonant_root`:

```rust
#[test]
fn juhotyadi_hu_core_forms() {
    // √hu, the ślu gaṇa's eponym. Every rule this slice adds is on one of
    // these surfaces; the per-rule guards are pinned beside the rules and
    // the ordered traces in crates/panini/tests/trace/juhotyadi.rs.
    //
    // laṭ: ślu, dvitva, kuhoś cuḥ and car ca on the abhyāsa; guṇa before
    // pit ti, blocked before ṅit taH (1.1.5 via the first 1.2.4); 6.4.87's
    // hu arm before 7.1.4's ati; 8.3.59 for juhozi.
    assert_eq!(form_g("03.0001", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "juhoti");
    assert_eq!(form_g("03.0001", Lakara::Lat, Purusha::Prathama, Vacana::Dvi), "juhutaH");
    assert_eq!(form_g("03.0001", Lakara::Lat, Purusha::Prathama, Vacana::Bahu), "juhvati");
    assert_eq!(form_g("03.0001", Lakara::Lat, Purusha::Madhyama, Vacana::Eka), "juhozi");
    assert_eq!(form_g("03.0001", Lakara::Lat, Purusha::Uttama, Vacana::Bahu), "juhumaH");
    // laṅ: aṭ in AGAMA; 3.4.109's jus with 7.3.83's guṇa and 6.1.78's av;
    // the pit am guṇates too.
    assert_eq!(
        form_g_forked("03.0001", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2),
        "ajuhod"
    );
    assert_eq!(form_g("03.0001", Lakara::Lan, Purusha::Prathama, Vacana::Bahu), "ajuhavuH");
    assert_eq!(form_g("03.0001", Lakara::Lan, Purusha::Madhyama, Vacana::Eka), "ajuhoH");
    assert_eq!(form_g("03.0001", Lakara::Lan, Purusha::Uttama, Vacana::Eka), "ajuhavam");
    // loṭ: 6.4.101's hu arm (a three-form cell with the tātaṅ pair); 7.1.4
    // again; the pit āṭ-ending guṇates.
    assert_eq!(
        form_g_forked("03.0001", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "juhuDi"
    );
    assert_eq!(form_g("03.0001", Lakara::Lot, Purusha::Prathama, Vacana::Bahu), "juhvatu");
    assert_eq!(form_g("03.0001", Lakara::Lot, Purusha::Uttama, Vacana::Eka), "juhavAni");
    // vidhiliṅ: yāsuṭ keeps 7.3.83 off the jus.
    assert_eq!(
        form_g_forked("03.0001", Lakara::VidhiLin, Purusha::Prathama, Vacana::Eka, 2),
        "juhuyAd"
    );
    assert_eq!(form_g("03.0001", Lakara::VidhiLin, Purusha::Prathama, Vacana::Bahu), "juhuyuH");
}

#[test]
fn juhotyadi_ki_core_forms() {
    // √ki: the i-final root. 6.4.82 before ati/atu, and NOT before Ani
    // (guṇa first: cikayAni) or yāsuṭ (cikiyAt); no 8.4.54 step on ci.
    assert_eq!(form_g("03.0020", Lakara::Lat, Purusha::Prathama, Vacana::Eka), "ciketi");
    assert_eq!(form_g("03.0020", Lakara::Lat, Purusha::Prathama, Vacana::Dvi), "cikitaH");
    assert_eq!(form_g("03.0020", Lakara::Lat, Purusha::Prathama, Vacana::Bahu), "cikyati");
    assert_eq!(form_g("03.0020", Lakara::Lat, Purusha::Madhyama, Vacana::Eka), "cikezi");
    assert_eq!(form_g("03.0020", Lakara::Lan, Purusha::Prathama, Vacana::Bahu), "acikayuH");
    assert_eq!(form_g("03.0020", Lakara::Lan, Purusha::Uttama, Vacana::Eka), "acikayam");
    assert_eq!(
        form_g_forked("03.0020", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "cikihi"
    );
    assert_eq!(form_g("03.0020", Lakara::Lot, Purusha::Prathama, Vacana::Bahu), "cikyatu");
    assert_eq!(form_g("03.0020", Lakara::Lot, Purusha::Uttama, Vacana::Eka), "cikayAni");
    assert_eq!(
        form_g_forked("03.0020", Lakara::VidhiLin, Purusha::Prathama, Vacana::Eka, 2),
        "cikiyAd"
    );
    assert_eq!(form_g("03.0020", Lakara::VidhiLin, Purusha::Prathama, Vacana::Bahu), "cikiyuH");
}

#[test]
fn ajuhavuh_lays_the_augment_abhyasa_and_root_in_their_own_slots() {
    // The five-slot layout doing the job the prep built it for: augment,
    // abhyāsa and root are three terms, and the tripādī found the
    // abhyāsa's J by term, not by guessing an offset into the root.
    let d = dhatus().iter().find(|d| d.dhatupatha == "03.0001").unwrap();
    let p = sole(derive(
        d,
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    ));
    assert_eq!(p.text(), "ajuhavuH");
    assert_eq!(p.terms[AGAMA].text, "a");
    assert_eq!(p.terms[ABHYASA].text, "ju");
    assert_eq!(p.terms[ANGA].text, "hav");
    assert_eq!(p.terms[SHAP].text, "");
    assert_eq!(p.terms[ENDING].text, "uH");
    assert!(p.terms[ABHYASA].has(Tag::Abhyasa));
    assert!(p.terms[ABHYASA].has(Tag::Abhyasta));
    assert!(p.terms[ANGA].has(Tag::Abhyasta));
    assert!(p.terms[SHAP].has(Tag::Slu));
}
```

Run: `mise exec -- cargo test -p panini-prakriya juhotyadi ajuhavuh 2>&1 | tail -30`
Expected: all three PASS. If a surface differs, the engine is wrong, not the expectation: these are vidyut's forms (`hu_ki_trace.txt` from planning). Diagnose from the branch's `log` (print `p.log.iter().map(|s| (&s.sutra, &s.after))`) against the rule the trace pin for that cell expects, fix the guard in its task's file with a unit test, and re-run that task's full suite.

- [ ] **Step 6: The GATED window**

In `paradigm/main.rs`, `paradigm_covers_every_enumerable_cell`:

```rust
    // Slice 3a gates √hu's and √ki's eight (root, lakāra, pada) triples —
    // 2 roots × 1 pada × 4 lakāras — for one commit, between their Dhatu
    // rows landing and their cross-implementation-audited goldens
    // arriving, the same window 7f and the Buj slice used.
    const GATED: &[(&str, &str, Pada)] = &[
        ("03.0001", "laT", Pada::Parasmaipada),
        ("03.0001", "laN", Pada::Parasmaipada),
        ("03.0001", "loT", Pada::Parasmaipada),
        ("03.0001", "viDiliN", Pada::Parasmaipada),
        ("03.0020", "laT", Pada::Parasmaipada),
        ("03.0020", "laN", Pada::Parasmaipada),
        ("03.0020", "loT", Pada::Parasmaipada),
        ("03.0020", "viDiliN", Pada::Parasmaipada),
    ];
```

(Keep the existing history comment above it; append this paragraph to it.)

- [ ] **Step 7: FULL suite**

Run: `mise run test` (foreground). Expected: PASS. `roundtrip` now derives the 72 new cells and checks each through the analyzer; `paradigm_covers_every_enumerable_cell` passes on the gate; `derivation_set_shape_matches_the_audited_numbers` is unchanged (no golden yet). `git status --short crates/panini/tests` shows only `main.rs`.

- [ ] **Step 8: Commit**

```bash
git add crates/panini-data/src/lib.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs crates/panini/tests/paradigm/main.rs
git commit -m "feat(data): juhotyadi 3a rows — hu (03.0001) and ki (03.0020); whole-derivation pins; goldens gated for one commit

79 roots. The eight new triples are GATED until the audit certifies them."
```

---

### Task 7: the cross-implementation audit — the blocking gate

**Files:**
- Modify: `tools/audit/panini_full_audit.rs` (`gana_name` arm; the three total assertions; module-header totals)
- Modify: `tools/audit/README.md` ("What it commits to" totals; "Last recorded result")

**Interfaces:**
- Consumes: Tasks 1–6.
- Produces: a recorded zero-difference verdict at 79 / 3564 / `<N>`, where `<N>` is the harness's measured form total (projected 4483). Task 8 cross-checks `3564 + ALTERNATES.len() == <N>`.

**This task blocks Task 8.** No golden is pinned before the audit certifies it.

- [ ] **Step 1: Vidyut checkout at the vendored commit**

```bash
head -20 /workspace/data/dhatupatha.tsv | grep -i commit
git -C /tmp/vidyut-full log --oneline -1
```

Both must show `8da2f90…`. If `/tmp/vidyut-full` is missing, clone and checkout as `tools/audit/README.md` describes.

- [ ] **Step 2: Point the dev-deps at THIS checkout and copy the harness**

In `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml`, set the `panini` and `panini-data` dev-dependency paths to the checkout being audited — `<checkout>/crates/panini` and `<checkout>/crates/panini-data`; verify with `git -C <checkout> branch --show-current` printing `juhotyadi-3a`. (They point at `/workspace/crates` after planning, which is `main`, not this branch.) First make the one structural edit in the repo copy, `<checkout>/tools/audit/panini_full_audit.rs`: `gana_name` gains

```rust
        PGana::Juhotyadi => "Juhotyadi",
```

(the compiler forces it — the match is exhaustive). Then:

```bash
cp <checkout>/tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
```

Copy; do not rewrite.

- [ ] **Step 3: Negative control FIRST**

```bash
cd /tmp/vidyut-full/vidyut-prakriya
PANINI_AUDIT_PERTURB=entry mise exec rust@1.98 -- cargo run --release --example panini_full_audit
```

Expected: **exit 1**, 36 differing √bhū cells (the control targets `01.0001`/`01.0381`, outside this slice). If it passes, stop — every later result is worthless.

- [ ] **Step 4: Real run; set the totals**

```bash
mise exec rust@1.98 -- cargo run --release --example panini_full_audit 2>&1 | tee /tmp/3a-audit.txt
```

The corpus block prints before the assertions panic on the stale totals. Read `roots: 79`, `cells: 3564`, `forms: <N>`; set the three assertions in BOTH copies (the repo's and the example's):

```rust
    assert_eq!(roots_seen.len(), 79, "curated roots");
    assert_eq!(n_cells, 3564, "cells: 396 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, <N>, "forms: 3564 cells + <N - 3564> ALTERNATES rows");
```

Update the module header in the same edit: "for each of the 77 curated roots" → 79; "Corpus invariants, asserted: 77 roots, 3492 cells, 4399 forms" → 79 / 3564 / `<N>`; "388 root×pada×lakāra blocks × 9 cells, plus 907 `ALTERNATES` rows" → 396 / `<N - 3564>`; "dump the full 3492-cell table" → 3564. Re-run: expected `AUDIT PASSED: 3564 cells, <N> forms, zero differences.` The probe projects `<N>` = **4483** (12 new alternates) — a recognition tripwire, not a value to type.

- [ ] **Step 5: If there ARE differences**

Stop and diagnose; never patch golden data toward either engine. Read the `DIFF` lines' two derivation sets; the planning probe (`hu_ki_trace.txt`) names the rule vidyut credits per cell. An engine bug in a rule Tasks 1–5 added → fix in that task's file with a unit test, re-run the FULL suite (priors!) and this audit. A difference that is a genuine modelling disagreement (none is expected: the spec's appendix and the probe agree on all 84 forms) → record it in the spec and this plan before deciding anything.

- [ ] **Step 6: Record and commit**

`tools/audit/README.md`: "It asserts the corpus totals (77 roots, 3492 cells, 4399 forms)" → 79 / 3564 / `<N>`; add a new "Last recorded result" entry ABOVE the prep's, in the existing shape — date, `juhotyādi 3a slice`, vidyut `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, zero differences across 3564 cells / `<N>` forms / 79 roots, `entry` control verified failing first (exit 1, 36 √bhū cells), and a paragraph naming what the verdict now covers: the eight new rules (2.4.75, 6.1.10, 7.4.62, 3.4.109, 7.1.4, 7.3.83, 6.4.82, 8.4.54), the two widened arms (6.4.87, 6.4.101), the new `abhyasa` stage, and the dvitva-before-guṇa order (forms identical, trace order not). Close with the totals arithmetic: 79 = 77 + 2; 3564 = 3492 + 72 (2 roots × 1 pada × 4 lakāras × 9); `<N>` = 4399 + 72 + `<N - 3564 - 907>` new `ALTERNATES` rows.

```bash
cd <checkout>
git add tools/audit/
git commit -m "test(audit): juhotyadi 3a is byte-identical to vidyut

79 roots / 3564 cells / <N> forms, zero differences at vidyut 8da2f90,
entry control verified failing first."
```

---

### Task 8: the goldens, generated

**Files:**
- Create then delete: `crates/panini/tests/print_3a_goldens.rs`
- Create: `crates/panini/tests/paradigm/data/juhotyadi.rs`
- Modify: `crates/panini/tests/paradigm/data/mod.rs` (mod line + both concat arrays + mod doc)
- Modify: `crates/panini/tests/paradigm/main.rs` (shape test + its doc comment; `GATED` back to empty; `pada_ambiguous_surfaces_are_exactly_these` comment)

**Interfaces:**
- Consumes: the audit verdict and `<N>` (Task 7); `common::{CELLS, LAKARA_BY_NAME}`.
- Produces: 8 `PARADIGM` blocks + 12 `ALTERNATES` rows in `data/juhotyadi.rs`; the measured buckets and key counts Tasks 9–11 quote.

- [ ] **Step 1: The throwaway generator**

Create `crates/panini/tests/print_3a_goldens.rs`:

```rust
//! THROWAWAY -- slice 3a only. Prints the two new roots' `PARADIGM` blocks
//! and `ALTERNATES` rows as Rust source, plus the distribution counts
//! `derivation_set_shape_matches_the_audited_numbers` asserts. Deleted in
//! the same task that pastes its output: goldens are generated from the
//! engine the audit certified, never hand-authored.
//!
//! Run with:
//!   mise exec -- cargo test -p panini --test print_3a_goldens -- --nocapture

mod common;

use common::{CELLS, LAKARA_BY_NAME};
use panini_data::dhatus;
use panini_prakriya::derive;

const NEW_ROOTS: [&str; 2] = ["03.0001", "03.0020"];

/// Mirrors `VIKALPA_RULES` in `paradigm/main.rs`. An alternate's key is the
/// `+`-joined list of optional rules its branch actually applied, which is
/// what `every_alternate_names_the_vikalpa_rules_that_produced_it` checks.
const VIKALPA_RULES: &[&str] = &[
    "7.1.35", "3.4.111", "7.3.86", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56",
];

#[test]
fn print_3a_goldens() {
    // (rendered row, key) -- the key is kept alongside rather than parsed
    // back out of the rendered string.
    let mut alternates: Vec<(String, String)> = Vec::new();
    // Indexed by forms-per-cell. Sized well past the repo's deepest fork
    // (six) so an unexpectedly sharp cell prints instead of panicking.
    let mut multiplicity = [0usize; 12];

    println!("\n// ==== PARADIGM blocks ====");
    for number in NEW_ROOTS {
        let d = dhatus()
            .iter()
            .find(|d| d.dhatupatha == number)
            .unwrap_or_else(|| panic!("{number} is not a curated root"));
        for &pada in d.pada.padas() {
            for (lak_name, lak) in LAKARA_BY_NAME {
                let mut goldens: Vec<String> = Vec::new();
                for (cell, &(pu, va)) in CELLS.iter().enumerate() {
                    let branches = derive(d, lak, pada, pu, va);
                    // Index 0 is the declined derivation -- the one with no
                    // optional rule applied. `derivation_set_is_exactly_pinned`
                    // requires PARADIGM to hold exactly this.
                    let golden = branches[0].text();
                    let mut n_alts = 0usize;
                    for p in branches.iter().filter(|p| !p.blocked && p.text() != golden) {
                        let key: Vec<&str> = p
                            .log
                            .iter()
                            .map(|s| s.sutra.as_str())
                            .filter(|s| VIKALPA_RULES.contains(s))
                            .collect();
                        let key = key.join("+");
                        alternates.push((
                            format!(
                                "({:?}, {:?}, Pada::{:?}, {}, {:?}, {:?}),",
                                number,
                                lak_name,
                                pada,
                                cell,
                                p.text(),
                                key,
                            ),
                            key,
                        ));
                        n_alts += 1;
                    }
                    multiplicity[1 + n_alts] += 1;
                    goldens.push(format!("{golden:?}"));
                }
                println!(
                    "({:?}, {:?}, Pada::{:?}, [{}]),",
                    number,
                    lak_name,
                    pada,
                    goldens.join(", "),
                );
            }
        }
    }

    println!("\n// ==== ALTERNATES rows ({}) ====", alternates.len());
    for (row, _) in &alternates {
        println!("{row}");
    }

    println!(
        "\n// ==== distribution of the {} new cells ====",
        multiplicity.iter().sum::<usize>()
    );
    for (n, count) in multiplicity.iter().enumerate().skip(1) {
        if *count > 0 {
            println!("// {n}-form cells: {count}");
        }
    }

    println!("\n// ==== new ALTERNATES rows per key ====");
    let mut keys: Vec<&str> = alternates.iter().map(|(_, k)| k.as_str()).collect();
    keys.sort_unstable();
    keys.dedup();
    for key in keys {
        let n = alternates.iter().filter(|(_, k)| k == key).count();
        println!("// key {key}: {n}");
    }
}
```

- [ ] **Step 2: Run it**

```bash
mise exec -- cargo test -p panini --test print_3a_goldens -- --nocapture 2>&1 | tee /tmp/3a-goldens.txt
```

Expected: PASS, printing 8 block lines, 12 `ALTERNATES` rows, `1-form cells: 64`, `2-form cells: 4`, `3-form cells: 4`, and per-key `7.1.35: 4`, `7.1.35+8.4.56: 4`, `8.4.56: 4`. Sanity-check these single-form strings against the probe (do NOT type them into the golden file; paste the generator's output):

| block | expected |
|---|---|
| `03.0001` laT | `juhoti juhutaH juhvati juhozi juhuTaH juhuTa juhomi juhuvaH juhumaH` |
| `03.0001` laN | `ajuhod ajuhutAm ajuhavuH ajuhoH ajuhutam ajuhuta ajuhavam ajuhuva ajuhuma` |
| `03.0001` loT | `juhotu juhutAm juhvatu juhuDi juhutam juhuta juhavAni juhavAva juhavAma` |
| `03.0001` viDiliN | `juhuyAd juhuyAtAm juhuyuH juhuyAH juhuyAtam juhuyAta juhuyAm juhuyAva juhuyAma` |
| `03.0020` laT | `ciketi cikitaH cikyati cikezi cikiTaH cikiTa cikemi cikivaH cikimaH` |
| `03.0020` laN | `aciked acikitAm acikayuH acikeH acikitam acikita acikayam acikiva acikima` |
| `03.0020` loT | `ciketu cikitAm cikyatu cikihi cikitam cikita cikayAni cikayAva cikayAma` |
| `03.0020` viDiliN | `cikiyAd cikiyAtAm cikiyuH cikiyAH cikiyAtam cikiyAta cikiyAm cikiyAva cikiyAma` |

Alternates per root: `laN` cell 0 `ajuhot`/`aciket` keyed `8.4.56`; `loT` cells 0 and 3 each `…tAd` keyed `7.1.35` and `…tAt` keyed `7.1.35+8.4.56` (`juhutAd`/`juhutAt`, `cikitAd`/`cikitAt`); `viDiliN` cell 0 `juhuyAt`/`cikiyAt` keyed `8.4.56`. If anything differs, stop: the audit passed, so a mismatch is generator addressing or a plan expectation being wrong — diagnose which before pasting.

- [ ] **Step 3: Create `data/juhotyadi.rs` and wire it**

Create the file with the header shape of `data/tanadi.rs`:

```rust
//! juhotyadi's golden rows. See `super` (`data/mod.rs`) for the row
//! contracts and the concatenated `PARADIGM` / `ALTERNATES` statics.

use panini_data::Pada;

use super::{AlternateRow, ParadigmRow};

pub const PARADIGM: &[ParadigmRow] = &[
    // paste the 8 block lines from /tmp/3a-goldens.txt
];

pub const ALTERNATES: &[AlternateRow] = &[
    // paste the 12 rows from /tmp/3a-goldens.txt
];
```

Paste verbatim; `rustfmt` owns the formatting. In `data/mod.rs`: add `pub mod juhotyadi;` to the alphabetical mod list (between `divadi` and `kryadi`), append `juhotyadi::PARADIGM,` / `juhotyadi::ALTERNATES,` as the LAST entry of each concat array (the mod doc's own instruction), and extend the mod doc's enumeration: "01 bhvādi, 02 adādi, 03 juhotyādi, 04 divādi, …".

- [ ] **Step 4: The shape test**

In `paradigm/main.rs`, `derivation_set_shape_matches_the_audited_numbers`: `total_cells` 3492 → 3564 ("396 root×lakāra blocks × 9 cells each"); `ones` 2818 → 2882; `twos` 522 → 526; `threes` 111 → 115; `ALTERNATES.len()` 907 → 919; `key_count("8.4.56")` 130 → 134; `key_count("7.1.35")` 108 → 112; `key_count("7.1.35+8.4.56")` 108 → 112 — each replaced by the generator's printed number, not by this plan's. **Cross-check: `3564 + ALTERNATES.len() == <N>`** from Task 7; a mismatch is a paste error, resolved here.

Append a paragraph to the test's long doc comment:

> Slice 3a opens the ninth gaṇa, juhotyādi (3), the ślu gaṇa, with its eponym √hu (`03.0001`) and √ki (`03.0020`), both parasmaipadī by 1.3.78 — the first roots whose derivations put a term in the `ABHYASA` slot. Eight new sūtras (2.4.75 *ślu*, 6.1.10 *ślau* dvitva, 7.4.62, 3.4.109, 7.1.4, 7.3.83, 6.4.82, 8.4.54) and two widened arms (6.4.87's and 6.4.101's *hu*), yet no new fork kind: each root forks exactly where every -oti parasmaipada root already does — laṅ prathama eka and vidhiliṅ prathama eka on 8.4.56 (`ajuhod`/`ajuhot`, `juhuyAd`/`juhuyAt`), and the two loṭ tātaṅ cells three ways (`juhotu`/`juhutAd`/`juhutAt`, `juhuDi`/`juhutAd`/`juhutAt`) — six `ALTERNATES` rows per root, all in pre-existing keys. The gaṇa is PARTIAL at 2 of its 26 rows.

Set `GATED` back to `&[]` and fold Task 6's sentence into the history comment above it ("…were likewise gated for one commit in slice 3a").

- [ ] **Step 5: Delete the generator; re-check the ambiguity set**

```bash
rm crates/panini/tests/print_3a_goldens.rs
mise exec -- cargo test -p panini --test paradigm pada_ambiguous
```

Expected: PASS unchanged (both roots are parasmaipada-only, so they never enter the bucket). Append one sentence to that test's comment: "Slice 3a's two roots, √hu and √ki, are parasmaipada-only and contribute nothing; the set stands at forty-four."

- [ ] **Step 6: Full suite, foreground**

```bash
mise run test
```

Expected: PASS; `paradigm_covers_every_enumerable_cell` closes the window; `every_form_validates_and_matches` and `every_alternate_*` accept all 84 new forms; `derivation_set_is_exactly_pinned` agrees. Budget 35+ minutes.

- [ ] **Step 7: Commit**

```bash
git add crates/panini/tests/paradigm/
git commit -m "test(paradigm): juhotyadi 3a goldens, generated from the audited engine

8 blocks / 72 cells in data/juhotyadi.rs, 12 ALTERNATES rows; the corpus
reaches 79 roots, 3564 cells, <N> forms. GATED window closed."
```

---

### Task 9: the trace pins

**Files:**
- Create: `crates/panini/tests/trace/juhotyadi.rs`
- Modify: `crates/panini/tests/trace/main.rs` (mod line)

**Interfaces:**
- Consumes: `crate::helpers::{at, cell_trace}`; `panini_prakriya::derive`; the pinned goldens (Task 8).
- Produces: the gaṇa's ordered-trace witnesses — every pin the spec's Testing section names.

- [ ] **Step 1: Write the module**

Create `crates/panini/tests/trace/juhotyadi.rs`:

```rust
//! juhotyadi's ordered-trace witnesses. Helpers live in
//! `crate::helpers`; the module doc governing this suite is in
//! `main.rs`.
//!
//! The order these pins hold is THIS engine's: dvitva (6.1.10) and the
//! abhyāsa rules run before guṇa (7.3.84), the Kaumudī sequence, where
//! vidyut-prakriya guṇates first, copies, and shortens the copy back by
//! 7.4.59. Forms agree; the traces do not, and these pins are what make
//! the engine's own order a checked fact rather than an accident.

use crate::helpers::{at, cell_trace};
use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};
use panini_prakriya::derive;

#[test]
fn juhoti_trace_is_slu_dvitva_cutva_guna_then_car_ca() {
    // hu laT P.E. The reduplication core in pipeline order, and two
    // load-bearing absences: no 1.2.4 (ti is pit; the ślu'd śap is pit
    // too, so the second 1.2.4 declines), and no 2.4.72 (ślu, not luk).
    let (text, t) = cell_trace(
        "03.0001",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "juhoti", "got {t:?}");
    assert!(at(&t, "3.1.68") < at(&t, "2.4.75"), "got {t:?}");
    assert!(at(&t, "2.4.75") < at(&t, "6.1.10"), "got {t:?}");
    assert!(at(&t, "6.1.10") < at(&t, "7.4.62"), "got {t:?}");
    assert!(at(&t, "7.4.62") < at(&t, "7.3.84"), "got {t:?}");
    assert!(at(&t, "7.3.84") < at(&t, "8.4.54"), "got {t:?}");
    assert!(!t.contains(&"1.2.4".to_string()), "got {t:?}");
    assert!(!t.contains(&"2.4.72".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.4.59".to_string()), "got {t:?}");
}

#[test]
fn juhvati_trace_credits_7_1_4_and_the_hu_arm_of_6_4_87() {
    // hu laT P.B. The jh goes by 7.1.4 (at), not 7.1.3 (ant); the yaṇ is
    // 6.4.87's hu arm on the ROOT, not 6.1.77 (tanādi's u) nor 6.4.77.
    let (text, t) = cell_trace(
        "03.0001",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "juhvati", "got {t:?}");
    assert!(at(&t, "7.1.4") < at(&t, "6.4.87"), "got {t:?}");
    assert!(t.contains(&"1.2.4".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.1.3".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.1.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.84".to_string()), "got {t:?}");
}

#[test]
fn ajuhavuh_trace_credits_3_4_109_and_7_3_83_with_the_augment_in_agama() {
    // hu laN P.B. jus by 3.4.109 (not 3.4.108, not 7.1.3), guṇa by 7.3.83
    // (not 7.3.84, which the ṅit us blocks), av by 6.1.78; and the augment
    // is its own term, the abhyāsa another.
    let d = dhatus().iter().find(|d| d.dhatupatha == "03.0001").unwrap();
    let branches: Vec<_> = derive(
        d,
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    )
    .into_iter()
    .filter(|p| !p.blocked)
    .collect();
    assert_eq!(branches.len(), 1, "ajuhavuH does not fork");
    let p = &branches[0];
    let t: Vec<String> = p.log.iter().map(|s| s.sutra.clone()).collect();
    assert_eq!(p.text(), "ajuhavuH", "got {t:?}");
    assert!(at(&t, "3.4.109") < at(&t, "3.1.68"), "got {t:?}");
    assert!(at(&t, "6.4.71") < at(&t, "7.3.83"), "got {t:?}");
    assert!(at(&t, "7.3.83") < at(&t, "6.1.78"), "got {t:?}");
    assert!(!t.contains(&"3.4.108".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.84".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.1.3".to_string()), "got {t:?}");
    assert_eq!(p.terms[0].text, "a", "AGAMA");
    assert_eq!(p.terms[1].text, "ju", "ABHYASA");
    assert_eq!(p.terms[2].text, "hav", "ANGA");
}

#[test]
fn juhudhi_trace_credits_6_4_101_and_cikihi_does_not() {
    // hu loT M.E. — a three-form cell (tātaṅ pair); branch 0 is the
    // declined juhuDi, whose Di is 6.4.101's hu arm. ki's cikihi keeps hi:
    // i is no jhal and the root is not √hu.
    let (text, t) = cell_trace(
        "03.0001",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "juhuDi", "got {t:?}");
    assert!(at(&t, "3.4.87") < at(&t, "6.4.101"), "got {t:?}");
    assert!(!t.contains(&"6.4.105".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.106".to_string()), "got {t:?}");
    let (text, t) = cell_trace(
        "03.0020",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "cikihi", "got {t:?}");
    assert!(!t.contains(&"6.4.101".to_string()), "got {t:?}");
}

#[test]
fn cikyati_trace_credits_6_4_82_not_6_4_77() {
    // ki laT P.B. The i → y is 6.4.82 (anekāc over ci-ki, asaṁyogapūrva),
    // the apavāda, not 6.4.77's iyaṅ; and ci records no 8.4.54 (already car).
    let (text, t) = cell_trace(
        "03.0020",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "cikyati", "got {t:?}");
    assert!(at(&t, "7.1.4") < at(&t, "6.4.82"), "got {t:?}");
    assert!(!t.contains(&"6.4.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.54".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.84".to_string()), "got {t:?}");
}

#[test]
fn cikayani_trace_gunates_before_6_4_82_could_see_the_i() {
    // ki loT U.E. Ani is pit: 7.3.84 (ke) then 6.1.78 (kay), and 6.4.82
    // must NOT have fired — the ordering pin the spec names.
    let (text, t) = cell_trace(
        "03.0020",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "cikayAni", "got {t:?}");
    assert!(at(&t, "3.4.92") < at(&t, "7.3.84"), "got {t:?}");
    assert!(at(&t, "7.3.84") < at(&t, "6.1.78"), "got {t:?}");
    assert!(!t.contains(&"6.4.82".to_string()), "got {t:?}");
}

#[test]
fn ciketi_trace_carries_no_8_4_54_and_juhuyuh_no_7_3_83() {
    // Two absences vidyut's credits confirm: ki's abhyāsa ci is already
    // car, and vidhiliṅ's jus sits behind yāsuṭ, out of 7.3.83's reach.
    let (text, t) = cell_trace(
        "03.0020",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "ciketi", "got {t:?}");
    assert!(t.contains(&"7.4.62".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.54".to_string()), "got {t:?}");
    let (text, t) = cell_trace(
        "03.0001",
        Lakara::VidhiLin,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "juhuyuH", "got {t:?}");
    assert!(t.contains(&"3.4.108".to_string()), "got {t:?}");
    assert!(t.contains(&"6.1.96".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.83".to_string()), "got {t:?}");
    assert!(!t.contains(&"3.4.109".to_string()), "got {t:?}");
}
```

In `trace/main.rs`, add `mod juhotyadi;` to the alphabetical mod list (between `divadi` and `kryadi`).

- [ ] **Step 2: Run the trace binary**

```bash
mise exec -- cargo test -p panini --test trace juhotyadi 2>&1 | tail -15
```

Expected: 7 passed. A failing ORDER assertion is a real finding (the rule sits in the wrong slot); a failing ABSENCE means a guard is wider than argued — fix the rule in its task's file, and re-run that task's full suite.

- [ ] **Step 3: Lint and commit**

Run: `mise run lint && mise run fmt-check` — Expected: clean.

```bash
git add crates/panini/tests/trace/
git commit -m "test(trace): juhotyadi 3a ordered-trace pins — dvitva before guna, 7.1.4, 3.4.109/7.3.83, 6.4.82, 6.4.101, 8.4.54"
```

---

### Task 10: the mutation gate

**Files:**
- Modify: `AGENTS.md` (the cargo-mutants paragraph — append an entry after the prep's)

**Interfaces:**
- Consumes: everything above.
- Produces: the measured floor and campaign record.

- [ ] **Step 1: Measure the uncontended floor**

```bash
cd <checkout> && time mise run test 2>&1 | tee /tmp/3a-floor.txt
```

Alone, foreground, nothing else running. Record per-binary times and the wall clock. Prior floor: **2025.227s at 3492 cells** (paradigm 895.91s, roundtrip 1124.11s, trace 3.74s). Cells grow +2.06%; the series says the floor does not scale by cell count — measure, do not extrapolate.

- [ ] **Step 2: Sanity-check the cap**

Multiply the measured total by the recorded `-j 4` contention range (**1.02×–1.43×**). If the projection exceeds **4800s**, stop and report — cap changes are recorded in `AGENTS.md` and `mise.toml` together, never silently. (A floor near 2100s projects ~2140–3000s: margin expected.)

- [ ] **Step 3: Run the campaign**

```bash
env | grep CARGO_MUTANTS   # must print nothing
cd <checkout> && nohup cargo-mutants mutants --package panini-prakriya --test-workspace=true --timeout 4800 -j 4 > /tmp/3a-mutants.txt 2>&1 &
```

(The `cargo-mutants` binary directly, with `mise.toml`'s exact arguments: the mise shim fails in background shells. Confirm with `ps` that four workers are building.) Expect ~19–20 hours (the prep's ran 18h31m for 618 mutants; this slice adds roughly 30–40 mutants across `abhyasa.rs`, the two new `sound.rs` tables, 2.4.75, 3.4.109, 7.1.4, 7.3.83, 6.4.82, 6.4.87's arm, 6.4.101's arm and 8.4.54). Wait on `/tmp/3a-mutants.txt` and `mutants.out/` with a monitor loop; background shells die at ~60 minutes, so re-arm the wait each time and never end the turn as "done" while it runs. Mutants to expect and where they must be caught: the `cutva_of` / `deaspirate_of` arm deletions (all-arms unit tests), `6.4.82`'s `< 2` → `< 1`/`<= 2` and the two `checked_sub` offsets (`er_anekaco_declines_on_each_of_its_conditions` and the cikyati golden), `8.4.54`'s `t == s` → `!=` (the `ci`/empty no-op cases), `7.3.83`'s `== "us"` (ajuhavuH), the `Tag` writes in 6.1.10 (`slau_copies_…` asserts all three), 3.4.109's lakāra test (the loṭ/laṭ loop), 7.1.4's `starts_with('J')` (juhoti's negative case).

- [ ] **Step 4: Check BOTH `missed.txt` and `timeout.txt`**

Expected: `missed.txt` holds exactly the two documented equivalents carried since 7e (`adesha.rs` 6.1.87's `s.remove(pos + 1)` → `s.remove(pos)`; `tripadi.rs` 8.3.13's `w[i - 1]` → `w[i]`), confirmed unchanged at their in-place comments — line numbers will have moved; identify by diff shape. `timeout.txt` holds exactly the one known-permanent ṇatva-scan entry (`j -= 1` → `j /= 1` in `tripadi.rs`, 8.4.2's backward scan). Any OTHER timeout: re-run it alone at the same cap before concluding anything. Any OTHER survivor: resolve, don't accept — a survivor in this slice's code means a unit test above is weaker than it looks, and the fix is a test (or a deleted clause), never a wider comment.

- [ ] **Step 5: Extract the duration distribution**

```bash
cd <checkout> && python3 - <<'PY'
import json
d = json.load(open('mutants.out/outcomes.json'))
xs = sorted(
    sum(p['duration'] for p in o.get('phase_results', []) if p['phase'] == 'Test')
    for o in d['outcomes'] if o.get('summary') == 'CaughtMutant'
)
n = len(xs)
pick = lambda q: xs[min(n - 1, int(q * n))]
print(f"caught={n} median={pick(.5):.1f} p90={pick(.9):.1f} p99={pick(.99):.1f} max={xs[-1]:.1f}")
print("over 600s:", sum(x > 600 for x in xs), " over 1200s:", sum(x > 1200 for x in xs))
PY
```

Also record the two missed mutants' own test-phase durations (they ran the suite to completion uncaught — a direct uncaught-run measurement).

- [ ] **Step 6: Record in `AGENTS.md`**

Append a paragraph in the established series shape, immediately after the prep's entry: "**Slice 3a (juhotyādi, √hu and √ki, eight new sūtras, first cells in `ABHYASA`) re-measured both at 3564 cells.**" — per-binary floor + wall clock, the move against the prep's 2025.227s with +2.06% cells, the cap sanity check (×1.02–1.43), campaign tallies (mutants / caught / missed / unviable / timeouts, and that the sum reconciles), the distribution from Step 5, the site-by-site account of this slice's new mutants, both margins labelled measured/projected (against the worst caught and the worst uncaught), and the ruling on the cap against the prep's 1.70× trigger reference. Keep every earlier paragraph verbatim.

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md
git commit -m "test: mutation gate at 3564 cells with the abhyasa stage, floor re-measured"
```

Do not commit `mutants.out/` (it was untracked by `2376f08`; check `git status` before `git add`).

---

### Task 11: documentation sweep, PR, finish

No rule body changes. Every sentence that says eight gaṇas, seven stages, 3492 cells, 77 roots, 907 alternates or 99 pinned ids is now false.

**Files:**
- Modify: `README.md` (Scope paragraph)
- Modify: `docs/ARCHITECTURE.md` (stage table and count; pinned-id sentence; "Eight gaṇas" paragraph; tag list; vikaraṇa-selection sentence; a juhotyādi paragraph; ALTERNATES/forks prose if it quotes 907/4399)
- Modify: `AGENTS.md:924` (the golden-suite sentence), `:1403` ("seven stage arrays")
- Modify: `crates/panini-prakriya/src/tinanta/terms.rs:54` ("six stage files" → "eight"), `crates/panini/tests/trace/main.rs:18` (same), `crates/panini-prakriya/src/tinanta/mod.rs` (already "eight" from Task 2 — verify)
- Modify: `crates/panini-data/src/lib.rs` (`gana_matches_dhatupatha_prefix` comment — done in Task 1; verify)
- Modify: `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md` (the 6.4.87 and 6.4.101 bullets: guard wording)

- [ ] **Step 1: `README.md`**

"eight gaṇas covered, all of them fully — *bhvādi* … plus *tanādi* (8, …)" → "nine gaṇas covered, eight of them fully — … plus *tanādi* (8, …) … — and *juhotyādi* (3, the ślu gaṇa: 2.4.75 elides śap by ślu and 6.1.10 reduplicates the root into the `ABHYASA` slot), **partial** at 2 of its 26 dhātupāṭha rows, √hu (`03.0001`, *juhoti*) and √ki (`03.0020`, *ciketi*), curated in slice 3a behind 7.4.62 *kuhoś cuḥ*, 7.1.4 *ad abhyastāt*, 3.4.109 with 7.3.83 *jusi ca*, 6.4.82 *er anekāco'saṁyogapūrvasya* and 8.4.54 *abhyāse car ca*, with 6.4.87 and 6.4.101 grown their √hu arms." Then: "over a curated 77-root set" → 79; "674 of the 3492 cells hold more than one form: 522 hold two, 111 hold three" → "682 of the 3564 cells …: 526 hold two, 115 hold three" (four/five/six unchanged). The pada-ambiguous list is unchanged (44).

- [ ] **Step 2: `docs/ARCHITECTURE.md`**

- "seven pipeline stages, each in its own file" → eight. Add a table row after `vikarana.rs`: `| `abhyasa.rs` | 6.1.10, 7.4.62 — dvitva and the abhyāsa's shape | after 3.1.68 |`. Add `2.4.75` to `vikarana.rs`'s row, `3.4.109` to `tin.rs`'s, `7.1.4` to `anga.rs`'s, `7.3.83` and `6.4.82` to `guna.rs`'s (in position), `8.4.54` to `tripadi.rs`'s.
- The pinned-ids sentence: "pins all 99 ids verbatim (… all three in `guna.rs`)" → "pins all 107 ids verbatim (… all three in `guna.rs` — 99 total — then juhotyādi 3a's eight: 2.4.75, 6.1.10, 7.4.62, 3.4.109, 7.1.4, 7.3.83, 6.4.82 and 8.4.54)".
- "Eight gaṇas are covered, all of them fully: … tanādi (8), **complete** at all ten of its dhātupāṭha rows (√kṛ, `08.0010`, curated in slice 8b)." → "Nine gaṇas are covered, eight of them fully: … and juhotyādi (3), **partial** at 2 of its 26 rows (√hu, √ki; slice 3a)." Add `Tag::Juhotyadi` to the tag list and "2.4.75" to "read by 3.1.69, …, 3.1.81, 2.4.72 and 2.4.75". In the vikaraṇa-selection sentence add "juhotyādi takes śap by 3.1.68 and loses it to ślu by 2.4.75".
- After the rudhādi/tanādi paragraphs, a juhotyādi paragraph:

> juhotyādi (gaṇa 3) is the first gaṇa whose vikaraṇa leaves a trace after it is gone. **2.4.75 *juhotyādibhyaḥ śluḥ*** empties śap in place like 2.4.72 but tags the term `Slu`, and **6.1.10 *ślau*** reads that tag alone: it copies the root into the `ABHYASA` slot the prep created, tagging the copy `Abhyasa` (6.1.4) and both terms `Abhyasta` (6.1.5). Everything that reshapes the abhyāsa lives in its own stage, `abhyasa.rs`, between `vikarana` and `anga` — 7.4.62 *kuhoś cuḥ* in 3a (hu → Ju, ki → ci); 7.4.59/7.4.60 arrive with 3b — so 6.4.71 always sees a finished abhyāsa and 7.1.4 *ad abhyastāt* (before 7.1.3) reads a real tag. **Dvitva runs before guṇa**, the Kaumudī order; vidyut-prakriya guṇates first, copies, and shortens the copy by 7.4.59. Forms agree for all 26 roots and the audit compares form sets, so the divergence shows only in this engine's trace pins (`tests/trace/juhotyadi.rs`). 3.4.109 gives laṅ its jus on `Tag::Juhotyadi` as a pre-dvitva stand-in for *abhyasta* (liṭ must revisit it), 7.3.83 *jusi ca* guṇates before that jus over 1.1.5's block, 6.4.82 turns ci-ki's final i to y before a vowel (after 7.3.84, so *cikayāni* guṇates first), 8.4.54 *abhyāse car ca* deaspirates the abhyāsa in the tripādī (Ju → ju), and 6.4.87 and 6.4.101 carry the √hu arms their sūtras name. 6.4.71/6.4.72 still read `ANGA`'s initial rather than the abhyāsa's: a reduplicant's initial is always the same class as its root's (7.4.60 keeps the first consonant; 7.4.62, 7.4.66, 7.4.77 substitute within class), and 3d's √ṛ is where that argument meets a vowel-initial witness.

- Any sentence quoting "3492", "4399", "907 rows", "77 roots" in this file → the new figures.

- [ ] **Step 3: `AGENTS.md`, `terms.rs`, `trace/main.rs`, the spec**

- `AGENTS.md:924`: "(`crates/panini/tests/paradigm/`, 3492 cells, eight gaṇas — all complete, tanādi closing at 10/10 in slice 8b …" → "3564 cells, nine gaṇas — eight complete, tanādi closing at 10/10 in slice 8b …, and juhotyādi (3) opened in slice 3a at 2 of its 26 rows". Further down that bullet, "(522 cells)" → 526, "(111 cells)" → 115, "907 rows in all, so 3492 + 907 = 4399 forms total" → "919 rows in all, so 3564 + 919 = 4483 forms total".
- `AGENTS.md:1403`: "a list of seven stage arrays" → eight.
- `terms.rs:54` and `trace/main.rs:18`: "six stage files" → "eight stage files".
- The spec's 6.4.87 and 6.4.101 bullets: change "(`ANGA.text == "hu"` with `Tag::Juhotyadi`)" / "the same `ANGA.text == "hu"` with `Tag::Juhotyadi` test as 6.4.87" to "(`ANGA.text == "hu"` alone — a gaṇa clause beside it could never be falsified, the 7.4.21 reasoning)" / "the same root-keyed `ANGA.text == "hu"` test as 6.4.87, likewise with no gaṇa clause".

- [ ] **Step 4: Verify prose against code**

```bash
grep -rn "eight gaṇas\|seven stage\|seven ordered\|seven pipeline\|six stage\|3492\|4399\|907 rows\|77 roots\|77-root\|99 ids\|all 99" README.md docs/ARCHITECTURE.md AGENTS.md crates/panini-prakriya/src crates/panini/tests crates/panini-data/src tools/audit/README.md | grep -v "docs/superpowers" | grep -v "was\|were\|before\|until\|prior\|8b\|prep\|as of\|from"
```

Every remaining hit must be a deliberate historical reference (the AGENTS.md timing series keeps its old figures). Fix the rest. Also `grep -rn "hu\b.*out of scope\|hu arm is not implemented\|Widen when gaṇa 3" crates/` must print nothing.

- [ ] **Step 5: Full verification and commit**

Run: `mise run fmt-check && mise run lint && mise run test` (foreground). Expected: all PASS, `git status --short crates/panini/tests` empty.

```bash
git add README.md docs/ARCHITECTURE.md AGENTS.md crates/panini-prakriya/src crates/panini/tests docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md
git commit -m "docs: nine gaṇas — juhotyādi 3a in README, ARCHITECTURE and AGENTS; eight pipeline stages; counts at 3564 cells"
```

- [ ] **Step 6: Push and PR**

```bash
cd <checkout> && mise run audit
git push -u origin juhotyadi-3a
gh pr create --title "feat(engine): juhotyādi 3a — ślu, dvitva and the abhyāsa core on √hu and √ki" --body "$(cat <<'MD'
Slice 3a of docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md.

- `Gana::Juhotyadi`; 2.4.75 ślu (śap emptied in place, tagged `Slu`); a new `abhyasa` stage between `vikarana` and `anga` with 6.1.10 dvitva (into the prep's `ABHYASA` slot) and 7.4.62.
- 3.4.109, 7.1.4, 7.3.83, 6.4.82, 8.4.54 new; 6.4.87 and 6.4.101 grow their √hu arms. Dvitva before guṇa (Kaumudī order; forms identical to vidyut, trace order pinned).
- √hu (`03.0001`) and √ki (`03.0020`): 72 cells, 84 forms; 3492 priors and every pinned trace byte-identical. Suite 3564 cells / 79 roots / <N> forms.
- Audit: zero differences at vidyut 8da2f90, entry control verified failing first.
- Mutation gate: <tallies from Task 10>; floor <measured>s at 3564 cells.

https://claude.ai/code/session_01EspCVnjFH8iQEjdz1JTgvo
MD
)"
```

- [ ] **Step 7: Finish the branch**

Per the standing instruction (`branch-finish-auto-merge`): wait for CI green, merge with a merge commit (the repo's convention), `git fetch`, verify the commits are ancestors of `origin/main`, and only then delete the branch and the worktree. Slice 3b's spec starts from the merged `main` and the spec's "Later slices" table.

---

## Deferred, and why

- **3b–3f** (√bhī, √hrī; the ā-roots; the ṛ-roots; √nij/√vij/√viṣ; the a-roots): each its own spec and plan from the design's "Later slices" table. 7.4.59/7.4.60 (3b) are the next abhyāsa rules and the first readers of `Tag::Abhyasa`.
- **Reading the abhyāsa's initial in 6.4.71/6.4.72:** argued equivalent in Task 5's comments; 3d's √ṛ (*aiyaḥ*) is the checkpoint, and moving both guards onto 6.1.90's "first non-empty term after `AGAMA`" walk is a one-commit change with a live pin if that review disagrees.
- **A real *abhyasta* test in 3.4.109:** needs dvitva to precede the tiṅ substitutions or a saṁjñā pre-pass; liṭ's problem, recorded in the rule's comment.
- **The multi-ekāc cut (6.1.1 / 6.1.2) in 6.1.10:** no juhotyādi root needs it; liṭ's.
- **`Tag::Abhyasa` as a guard:** deliberately unread in 3a (equivalent to the slot being non-empty, hence unfalsifiable); 3b's abhyāsa rules decide whether any of them has a reason to read the tag rather than the slot.
