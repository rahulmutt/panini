# Kryādi Gaṇa (Slices 9a + 9b) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add kryādi (gaṇa 9) as the engine's fifth gaṇa — six roots, 216 new golden forms, 1080 → 1296 — landing the śnā vikaraṇa with its ā ~ ī ~ ∅ alternation and the engine's first ṇatva.

**Architecture:** Grammar lives in `TINANTA_RULES`, an ordered list of six stage arrays under `crates/panini-prakriya/src/tinanta/`. Every rule self-guards on shape and/or `Prakriya.ctx` and returns whether it fired; `derive` carries no grammar branches. This plan adds seven rules across three stage files, plus a prerequisite correction to the ṅit-tagging layer that the new rules depend on.

**Tech Stack:** Rust (pinned to 1.97.1 via `mise`), no external deps in the grammar crates. `#![forbid(unsafe_code)]` everywhere.

## Global Constraints

- Toolchain via `mise` only. Build/test with `mise run build | test | lint | fmt | fmt-check | mutants | audit`. Never install Rust globally.
- Scope a single crate during iteration with `mise exec -- cargo test -p panini-prakriya`. **`mise run test -- -p X` does NOT scope** — it runs everything.
- Run `cargo-mutants` by invoking the binary directly, not through the `mise run mutants` shim, which fails in background shells.
- SLP1 is the only internal representation. Transliteration lives in `panini-lipi` and is not touched by this plan.
- `#![forbid(unsafe_code)]` in every non-fuzz crate.
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, never as a branch inside `derive`.
- Which stage file a rule belongs to is decided by its **position relative to 3.1.68**, not by its sūtra family. Rules before 3.1.68 address the ending as `ENDING_PRE_SHAP` (index 1); rules after it use `ENDING` (index 2) and may use `SHAP` (index 1).
- `p.terms[SHAP].text` may be **empty** (2.4.72 luks śap for adādi). Any rule reading "the segment after the aṅga" must handle an empty string — use `chars().next()` as an `Option`, never `unwrap()`.
- Per-rule guard tests go beside the rule in its stage file. Tests asserting a surface form or a trace go in `tinanta/derivation_tests.rs`.
- Every rule added to `TINANTA_RULES` must be added to `tinanta_rule_order_is_pinned` in `derivation_tests.rs` **at the same position**, or the suite fails immediately.
- Sūtra ids and names in traces must match `vidyut-prakriya`'s `data/sutrapatha.tsv`. Names below are copied from it verbatim.
- **Never edit an existing golden form to make a test pass.** A moved surface form means the change is wrong. Escalate instead.

**Spec:** `docs/superpowers/specs/2026-07-28-kryadi-gana-design.md`

---

## Slice 9a — the apit layer and the vikaraṇa core

### Task 1: Complete 1.2.4 for parasmaipada apit endings

The first 1.2.4 in `samjna.rs` is gated on `Pada::Atmanepada`, so no parasmaipada ending is ever tagged ṅit. Every later rule in this plan depends on `tas` being ṅit while `tip` is not. The rule's own comment names this as the intended widening point.

To decline on pit endings, 1.2.4 needs to know which endings are pit. `tip` / `sip` / `mip` carry a `p` anubandha that 1.3.9 strips, and 1.3.9 runs *before* 1.2.4 — so 3.4.78 must record the fact when it still can.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/samjna.rs` (3.4.78, ~line 68; 1.2.4, ~line 107)
- Modify: `crates/panini/tests/trace.rs` (six re-pins)

**Interfaces:**
- Produces: `Tag::Pit` is set on the tiṅ ending by 3.4.78 iff its raw text ends in `p`. `Tag::Ngit` is set on every sārvadhātuka tiṅ ending that is not pit, except in loṭ uttama. Tasks 4, 5 and 6 consume `p.terms[ENDING].has(Tag::Ngit)`.

- [ ] **Step 1: Write the failing guard tests**

Add to the `mod tests` block at the bottom of `crates/panini-prakriya/src/tinanta/samjna.rs`:

```rust
    #[test]
    fn sarvadhatukam_apit_tags_parasmaipada_apit_endings() {
        // 1.2.4 is not pada-conditioned. `tas` is apit (no p-anubandha), so
        // it is Nid-vat in a parasmaipada derivation exactly as `ta` is in an
        // atmanepada one. This is the tag 6.4.112/6.4.113 fire on.
        for (purusha, vacana) in [
            (Purusha::Prathama, Vacana::Dvi),  // tas
            (Purusha::Prathama, Vacana::Bahu), // Ji
            (Purusha::Uttama, Vacana::Dvi),    // vas
            (Purusha::Uttama, Vacana::Bahu),   // mas
        ] {
            let mut p = Prakriya {
                ctx: Context::new(Lakara::Lat, Pada::Parasmaipada, purusha, vacana),
                ..Default::default()
            };
            p.terms.push(Term::new("kliS"));
            for id in ["3.4.78", "1.3.9", "1.2.4"] {
                let rule = rules().find(|r| r.id == id).unwrap();
                (rule.apply)(&mut p);
            }
            assert!(
                p.terms[ENDING_PRE_SHAP].has(Tag::Ngit),
                "{purusha:?} {vacana:?} should be Nit"
            );
        }
    }

    #[test]
    fn sarvadhatukam_apit_declines_for_pit_endings() {
        // tip/sip/mip carry the p-anubandha. They must stay untagged, or
        // 6.4.113 would fire on them and kliSnAti would surface as
        // *kliSnIti.
        for (purusha, vacana) in [
            (Purusha::Prathama, Vacana::Eka), // tip
            (Purusha::Madhyama, Vacana::Eka), // sip
            (Purusha::Uttama, Vacana::Eka),   // mip
        ] {
            let mut p = Prakriya {
                ctx: Context::new(Lakara::Lat, Pada::Parasmaipada, purusha, vacana),
                ..Default::default()
            };
            p.terms.push(Term::new("kliS"));
            for id in ["3.4.78", "1.3.9", "1.2.4"] {
                let rule = rules().find(|r| r.id == id).unwrap();
                (rule.apply)(&mut p);
            }
            assert!(p.terms[ENDING_PRE_SHAP].has(Tag::Pit));
            assert!(
                !p.terms[ENDING_PRE_SHAP].has(Tag::Ngit),
                "{purusha:?} {vacana:?} is pit and must not be Nit"
            );
        }
    }

    #[test]
    fn sarvadhatukam_apit_still_declines_for_lot_uttama_in_both_padas() {
        // 3.4.92 AD uttamasya pic ca makes the lot-uttama endings pit
        // outright. Tagging them Nit would let 7.2.81 rewrite the AT-Agama
        // and turn BavAva into *Baviyva. This exclusion is grammar, not
        // trace-minimalism -- it must survive the widening.
        for pada in [Pada::Parasmaipada, Pada::Atmanepada] {
            let mut p = Prakriya {
                ctx: Context::new(Lakara::Lot, pada, Purusha::Uttama, Vacana::Dvi),
                ..Default::default()
            };
            p.terms.push(Term::new("BU"));
            for id in ["3.4.78", "1.3.9", "1.2.4"] {
                let rule = rules().find(|r| r.id == id).unwrap();
                (rule.apply)(&mut p);
            }
            assert!(
                !p.terms[ENDING_PRE_SHAP].has(Tag::Ngit),
                "{pada:?} lot uttama must not be Nit"
            );
        }
    }
```

Make sure the test module's `use` list covers `Context`, `Lakara`, `Pada`, `Purusha`, `Vacana`, `Tag`, `Term`, `Prakriya`, `rules` and `ENDING_PRE_SHAP`. Follow the imports already used by `anga.rs`'s test module as the model.

- [ ] **Step 2: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya sarvadhatukam_apit`
Expected: `sarvadhatukam_apit_tags_parasmaipada_apit_endings` FAILS (`tas` is not tagged). `sarvadhatukam_apit_declines_for_pit_endings` FAILS on the `has(Tag::Pit)` assertion. The loṭ-uttama test PASSES already.

- [ ] **Step 3: Tag Pit in 3.4.78**

In `samjna.rs`, inside 3.4.78's `apply`, after `e.add(Tag::Sarvadhatuka);`:

```rust
            // 1.3.3 halantyam identifies the final `p` of tip/sip/mip as an
            // it; that anubandha is what makes those three endings pit. 1.3.9
            // strips it below, and 1.2.4 runs after that -- so the fact has to
            // be recorded here, while the raw text still carries it.
            if ending.ends_with('p') {
                e.add(Tag::Pit);
            }
```

- [ ] **Step 4: Widen 1.2.4**

In `samjna.rs`, replace 1.2.4's guard:

```rust
            if !matches!(p.ctx.pada, Pada::Atmanepada)
                || (matches!(p.ctx.lakara, Lakara::Lot) && matches!(p.ctx.purusha, Purusha::Uttama))
            {
                return false;
            }
```

with:

```rust
            if p.terms[ENDING_PRE_SHAP].has(Tag::Pit)
                || (matches!(p.ctx.lakara, Lakara::Lot) && matches!(p.ctx.purusha, Purusha::Uttama))
            {
                return false;
            }
```

and replace the first bullet of the rule's "Guard notes" comment (the one beginning "Ātmanepada only in this slice") with:

```rust
    // - Not pada-conditioned. Every apit sārvadhātuka is ṅid-vat, parasmaipada
    //   included: 6.4.112 / 6.4.113 read exactly this tag, and the whole
    //   kryādi paradigm is the pit/apit split (kliSnAti from pit tip against
    //   kliSnItaH from apit tas). Pit-ness comes from the `p` anubandha,
    //   recorded by 3.4.78 before 1.3.9 strips it.
```

- [ ] **Step 5: Run the guard tests to verify they pass**

Run: `mise exec -- cargo test -p panini-prakriya sarvadhatukam_apit`
Expected: all three PASS.

- [ ] **Step 6: Verify no surface form moved**

Run: `mise run test`
Expected: `crates/panini/tests/paradigm.rs` PASSES unchanged — all 1080 forms. If any golden form differs, **stop**: the widening is wrong and the spec's no-delta argument has failed. Do not edit the golden.

`crates/panini/tests/trace.rs` will report exactly six failures. Confirm the failing set is precisely `aBavan`, `BavAmaH`, `Bavanti`, `BaveyuH`, `yAnti`, `yAyuH`, and that each diff is a single inserted `"1.2.4"` entry and nothing else. A seventh failure, or a diff of any other shape, is a bug — stop and escalate.

- [ ] **Step 7: Re-pin the six traces**

In `crates/panini/tests/trace.rs`, insert `"1.2.4"` into each of those six expected vectors at the position the test output reports (immediately after `"1.3.9"`, mirroring where it already appears in the ātmanepada traces such as `AsIta`). Add a comment above the first one:

```rust
// The `1.2.4` step below appears in every parasmaipada derivation whose
// ending is apit (tas, Ji, vas, mas). It was absent until kryādi needed the
// tag: 1.2.4 was gated on ātmanepada, so the atideśa that was already
// grammatically operative went unrecorded. Six traces gained this step and
// no surface form moved.
```

- [ ] **Step 8: Run the full suite**

Run: `mise run test`
Expected: PASS, all crates.

- [ ] **Step 9: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/samjna.rs crates/panini/tests/trace.rs
git commit -m "fix(prakriya): 1.2.4 is not pada-conditioned

Tag Pit on tip/sip/mip at 3.4.78, while the p-anubandha is still on the
raw text, and let 1.2.4 decline on that instead of on parasmaipada. The
lot-uttama exclusion stays: 3.4.92's own 'pic ca' makes those endings
pit, which is what keeps 7.2.81 off the AT-Agama.

No surface form moves. Six parasmaipada traces gain the 1.2.4 step that
was already grammatically operative but unrecorded."
```

---

### Task 2: Make `hi` apit and yāsuṭ ṅit

3.4.87 *ser hyapic ca* and 3.4.103 *yāsuṭ … ṅic ca* both state a tag in their own sūtra text that the implementations do not set. `vrIRIhi` and `kliSnIyAt` come out of 6.4.113 only if they do.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tin.rs` (3.4.87, ~line 154; 3.4.103)

**Interfaces:**
- Consumes: `Tag::Pit` from Task 1.
- Produces: the loṭ madhyama-eka ending `hi` carries `Tag::Ngit` and not `Tag::Pit`; the vidhiliṅ parasmaipada ending carries `Tag::Ngit`.

- [ ] **Step 1: Write the failing guard tests**

Add to `tin.rs`'s `mod tests`:

```rust
    #[test]
    fn ser_hyapic_ca_makes_hi_apit_and_ngit() {
        // "ser hi apit ca": the sutra names hi as apit in its own text. sip
        // arrives pit (3.4.78), so 3.4.87 must clear that and tag Nit --
        // otherwise 6.4.113 declines and vrIRIhi surfaces as *vrIRAhi.
        let mut p = Prakriya {
            ctx: Context::new(Lakara::Lot, Pada::Parasmaipada, Purusha::Madhyama, Vacana::Eka),
            ..Default::default()
        };
        p.terms.push(Term::new("vrI"));
        for id in ["3.4.78", "1.3.9", "1.2.4", "3.4.85", "3.4.87"] {
            let rule = rules().find(|r| r.id == id).unwrap();
            (rule.apply)(&mut p);
        }
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "hi");
        assert!(!p.terms[ENDING_PRE_SHAP].has(Tag::Pit));
        assert!(p.terms[ENDING_PRE_SHAP].has(Tag::Ngit));
    }

    #[test]
    fn yasut_is_ngit() {
        // 3.4.103's own name ends "Nic ca". Without the tag, 6.4.113 declines
        // in vidhilin and kliSnIyAt surfaces as *kliSnAyAt.
        let mut p = Prakriya {
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            ..Default::default()
        };
        p.terms.push(Term::new("kliS"));
        for id in ["3.4.78", "1.3.9", "1.2.4", "3.4.100", "3.4.103"] {
            let rule = rules().find(|r| r.id == id).unwrap();
            (rule.apply)(&mut p);
        }
        assert!(p.terms[ENDING_PRE_SHAP].has(Tag::Ngit));
    }
```

- [ ] **Step 2: Run to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya -- ser_hyapic_ca_makes_hi yasut_is_ngit`
Expected: both FAIL on the `has(Tag::Ngit)` assertion.

- [ ] **Step 3: Set the tags**

In 3.4.87's `apply`, after `p.terms[ENDING_PRE_SHAP].text = "hi".into();`:

```rust
            // "apit ca": hi is apit, so 1.2.4's atideśa reaches it. sip
            // arrived pit from 3.4.78; clear that before adding the ṅit, or
            // the term claims both. 1.2.4 has already run by now (samjna
            // stage), so the tag is set here rather than left to it.
            p.terms[ENDING_PRE_SHAP].remove(Tag::Pit);
            p.terms[ENDING_PRE_SHAP].add(Tag::Ngit);
```

In 3.4.103's `apply`, after the `format!("yAs{}", …)` line:

```rust
            // "Nic ca": yāsuṭ is ṅit, and the ending it augments is ṅit with
            // it. This is what 6.4.113 reads to give kliSnIyAt; tip's own pit
            // tag is left in place, since 1.1.5 asks only about ṅit.
            p.terms[ENDING_PRE_SHAP].add(Tag::Ngit);
```

`Term` has no `remove` method yet. Add one next to `add` in `crates/panini-prakriya/src/term.rs` — `tags` is a `HashSet<Tag>`, so this is a one-liner:

```rust
    pub fn remove(&mut self, tag: Tag) {
        self.tags.remove(&tag);
    }
```

- [ ] **Step 4: Run to verify they pass**

Run: `mise exec -- cargo test -p panini-prakriya -- ser_hyapic_ca_makes_hi yasut_is_ngit`
Expected: both PASS.

- [ ] **Step 5: Verify zero delta**

Run: `mise run test`
Expected: PASS with **no** test edits. Neither change adds a trace step — 3.4.87 and 3.4.103 already record — and neither moves a surface form: the only consumers of an ending's ṅit tag are 7.2.81 (which additionally needs an `A`-initial ending over an `a`-final śap) and, on the śap-luk'd path, 7.3.84 / 7.3.86 (which need an ik-final aṅga or a laghu ik upadhā — `yA`, `vA` and `ad` have neither). If anything fails, stop and escalate.

- [ ] **Step 6: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/tin.rs crates/panini-prakriya/src/term.rs
git commit -m "fix(prakriya): 3.4.87 and 3.4.103 set the tags their sutras name

'ser hyapic ca' says hi is apit; 'yAsuw ... Nic ca' says the augmented
ending is Nit. Neither was tagged. 6.4.113 reads both, so vrIRIhi and
kliSnIyAt depend on them. No form or trace delta."
```

---

### Task 3: Add gaṇa 9 to the data layer and rule 3.1.81

No roots are added yet — this task lands the plumbing and the vikaraṇa rule, verified at rule level, so the golden grid stays at 1080 and the diff stays reviewable.

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (`Gana` enum)
- Modify: `crates/panini-prakriya/src/term.rs` (`Tag` enum)
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs` (`derive`'s gaṇa tagging)
- Modify: `crates/panini-prakriya/src/tinanta/vikarana.rs` (new rule + tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (order pin)

**Interfaces:**
- Produces: `Gana::Kryadi`; `Tag::Kryadi`; rule id `"3.1.81"` in `VIKARANA`, positioned immediately after `"3.1.77"`. After it fires, `p.terms[SHAP].text == "nA"` and the term carries `Tag::Vikarana` + `Tag::Sarvadhatuka` and no `Tag::Pit`.

- [ ] **Step 1: Write the failing guard tests**

Add to `vikarana.rs`'s `mod tests`:

```rust
    #[test]
    fn kryadibhyah_shna_inserts_shna_for_kryadi_only() {
        // 3.1.81 is an apavAda to 3.1.68, same shape as 3.1.69/3.1.77.
        // it-samjNa strips the S (1.3.8), leaving nA. No Tag::Pit: SnA is
        // apit, so the second 1.2.4 makes it Nit and 1.1.5 then blocks guNa
        // -- which is why kliS gives kliSnAti and not *kleSnAti.
        let mut anga = Term::new("kliS");
        anga.add(Tag::Kryadi);
        let mut p = Prakriya {
            terms: vec![anga, Term::new("ti")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "3.1.81").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nA");
        assert!(p.terms[SHAP].has(Tag::Vikarana));
        assert!(p.terms[SHAP].has(Tag::Sarvadhatuka));
        assert!(!p.terms[SHAP].has(Tag::Pit));
        assert!(p.terms[ANGA].has(Tag::Anga));
    }

    #[test]
    fn kryadibhyah_shna_declines_for_every_other_gana() {
        // bhvAdi carries no gana tag at all; the other three carry their own.
        // A mutant that drops the tag guard would give every root SnA.
        for tag in [None, Some(Tag::Divadi), Some(Tag::Tudadi), Some(Tag::Adadi)] {
            let mut anga = Term::new("BU");
            if let Some(t) = tag {
                anga.add(t);
            }
            let mut p = Prakriya {
                terms: vec![anga, Term::new("ti")],
                log: vec![],
                ..Default::default()
            };
            let rule = rules().find(|r| r.id == "3.1.81").unwrap();
            assert!(!(rule.apply)(&mut p), "fired for {tag:?}");
        }
    }
```

- [ ] **Step 2: Run to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya kryadibhyah_shna`
Expected: FAIL to compile — `Tag::Kryadi` does not exist.

- [ ] **Step 3: Add the enum variants and the tagging**

In `crates/panini-data/src/lib.rs`, add to `Gana`:

```rust
    Kryadi,
```

In `crates/panini-prakriya/src/term.rs`, add to `Tag` after `Adadi`:

```rust
    /// The dhātu belongs to kryādi (gaṇa 9), whose vikaraṇa is śnā. Read by
    /// 3.1.81 alone. Mirrors Divadi/Tudadi/Adadi.
    Kryadi,
```

In `crates/panini-prakriya/src/tinanta/mod.rs`, add to `derive`'s `match dhatu.gana`:

```rust
            Gana::Kryadi => t.add(Tag::Kryadi),
```

- [ ] **Step 4: Add rule 3.1.81**

In `vikarana.rs`, insert immediately after the 3.1.77 rule and before 3.1.68:

```rust
    // 3.1.81 kryādibhyaḥ śnā: kryādi (gaṇa 9) takes śnā, not śap. Apavāda to
    // 3.1.68, ordered before it, exactly as 3.1.69 and 3.1.77 are. śnā is
    // apit; the second 1.2.4 makes it ṅit and 1.1.5 then blocks guṇa — which
    // is what keeps kliS from guṇating to kleS under 7.3.86.
    //
    // Unlike adādi's śap, śnā is never luk'd: its text goes nA → nI (6.4.113)
    // or nA → n (6.4.112), and never to empty. No rule reading terms[SHAP]
    // can silently decline the way the athematic path made them decline.
    Rule {
        id: "3.1.81",
        name: "kryAdiByaH SnA",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Kryadi) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("SnA");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.81", "kryAdiByaH SnA", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S → nA
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
```

- [ ] **Step 5: Update the order pin**

In `derivation_tests.rs`, in `tinanta_rule_order_is_pinned`, change `"3.1.69", "3.1.77", "3.1.68"` to `"3.1.69", "3.1.77", "3.1.81", "3.1.68"`.

- [ ] **Step 6: Run the tests**

Run: `mise run test`
Expected: PASS. The two new guard tests pass; the 1080 goldens are untouched because no root carries `Gana::Kryadi` yet.

- [ ] **Step 7: Commit**

```bash
git add crates/panini-data/src/lib.rs crates/panini-prakriya/src/term.rs \
        crates/panini-prakriya/src/tinanta/mod.rs \
        crates/panini-prakriya/src/tinanta/vikarana.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 3.1.81 kryAdiByaH SnA + the kryadi gana tag"
```

---

### Task 4: Rules 6.4.112 and 6.4.113 — śnā's alternation

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/anga.rs` (append two rules at the **end** of `ANGA_RULES`, after 7.3.101, plus tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (order pin)

**Interfaces:**
- Consumes: `p.terms[SHAP].text == "nA"` from Task 3; `Tag::Ngit` on `p.terms[ENDING]` from Tasks 1 and 2.
- Produces: rule ids `"6.4.112"` and `"6.4.113"`, in that order, at the end of `ANGA_RULES`.

**Placement is load-bearing.** These go at the end of `anga.rs`, not in sūtra order:
- **after 7.1.3** *jho'ntaḥ*, which turns `Ji` into `anti` / `ant` — until then the 3pl endings are not vowel-initial and 6.4.112 cannot see them;
- **after 7.2.79** *liṅaḥ salopo'nantyasya* — the ātmanepada vidhiliṅ ending is `sIyta` until its `s` goes, and 6.4.113 would wrongly match the `s`;
- **before `adesha.rs`** — 6.1.87 *ād guṇaḥ* would otherwise coalesce `nA` + `Iyta` into `ne` and give \*`vfReta`. `anga.rs` runs entirely before `adesha.rs`, so this holds structurally.

- [ ] **Step 1: Write the failing guard tests**

Add to `anga.rs`'s `mod tests`:

```rust
    /// Build `[anga, SnA, ending]` with the ending's ṅit-ness set explicitly.
    fn shna_prakriya(anga: &str, ending: &str, ngit: bool) -> Prakriya {
        let mut vik = Term::new("nA");
        vik.add(Tag::Vikarana);
        vik.add(Tag::Sarvadhatuka);
        vik.add(Tag::Ngit);
        let mut end = Term::new(ending);
        end.add(Tag::Tin);
        end.add(Tag::Sarvadhatuka);
        if ngit {
            end.add(Tag::Ngit);
        }
        Prakriya {
            terms: vec![Term::new(anga), vik, end],
            log: vec![],
            ..Default::default()
        }
    }

    #[test]
    fn shnabhyastayor_atah_elides_a_before_ajadi_ngit() {
        // kliS + nA + anti -> kliS + n + anti -> kliSnanti.
        let mut p = shna_prakriya("kliS", "anti", true);
        let rule = rules().find(|r| r.id == "6.4.112").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "n");
        assert_eq!(p.text(), "kliSnanti");
    }

    #[test]
    fn shnabhyastayor_atah_declines_on_halali_and_on_non_ngit() {
        // Consonant-initial: 6.4.113's case, not this rule's.
        let mut p = shna_prakriya("kliS", "taH", true);
        let rule = rules().find(|r| r.id == "6.4.112").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nA");
        // Vowel-initial but PIT (lot 3pl would be the only ajadi pit ending
        // if 1.2.4 misfired): the A must survive.
        let mut p = shna_prakriya("kliS", "anti", false);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nA");
    }

    #[test]
    fn i_halyaghoh_replaces_a_with_i_before_halali_ngit() {
        // kliS + nA + taH -> kliS + nI + taH -> kliSnItaH.
        let mut p = shna_prakriya("kliS", "taH", true);
        let rule = rules().find(|r| r.id == "6.4.113").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nI");
        assert_eq!(p.text(), "kliSnItaH");
    }

    #[test]
    fn i_halyaghoh_declines_on_ajadi_and_on_non_ngit() {
        // Vowel-initial: 6.4.112's case.
        let mut p = shna_prakriya("kliS", "anti", true);
        let rule = rules().find(|r| r.id == "6.4.113").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nA");
        // The pit case is the whole paradigm split: kliSnAti, not *kliSnIti.
        let mut p = shna_prakriya("kliS", "ti", false);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nA");
        assert_eq!(p.text(), "kliSnAti");
    }

    #[test]
    fn shna_alternation_rules_ignore_other_vikaranas_and_short_prakriyas() {
        // The text guard is what keeps these off Sap/Syan/Sa and off the
        // Sanac that 3.1.83 substitutes ("Ana", not "nA").
        for vikarana in ["a", "ya", "Ana", ""] {
            let mut p = shna_prakriya("kliS", "taH", true);
            p.terms[SHAP].text = vikarana.to_string();
            for id in ["6.4.112", "6.4.113"] {
                let rule = rules().find(|r| r.id == id).unwrap();
                assert!(!(rule.apply)(&mut p), "{id} fired on {vikarana:?}");
            }
        }
        // A one-term prakriya must not panic indexing SHAP or ENDING.
        let mut p = Prakriya {
            terms: vec![Term::new("kliS")],
            log: vec![],
            ..Default::default()
        };
        for id in ["6.4.112", "6.4.113"] {
            let rule = rules().find(|r| r.id == id).unwrap();
            assert!(!(rule.apply)(&mut p));
        }
    }
```

- [ ] **Step 2: Run to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya -- shnabhyastayor i_halyaghoh shna_alternation`
Expected: FAIL — `rules().find(|r| r.id == "6.4.112")` returns `None` and the `unwrap()` panics.

- [ ] **Step 3: Add the two rules**

Append to `ANGA_RULES` in `anga.rs`, after the 7.3.101 rule:

```rust
    // --- śnā's alternation (6.4.112, 6.4.113) -----------------------------
    //
    // Placed at the END of this stage, not in sūtra order. Three constraints
    // fix the position and each fails visibly if broken:
    //   - AFTER 7.1.3 jho'ntaḥ, which makes `Ji` into `anti`/`ant`. Before it,
    //     the 3pl endings are not vowel-initial and 6.4.112 cannot see them.
    //   - AFTER 7.2.79 liṅaḥ salopo'nantyasya. The ātmanepada vidhiliṅ ending
    //     is `sIyta` until its s is elided; run earlier and 6.4.113 matches
    //     the s, giving *vfRIsIyta.
    //   - BEFORE adesha.rs, whose 6.1.87 ād guṇaḥ would coalesce nA + Iyta
    //     into ne and give *vfReta. This stage runs entirely before that one.
    //
    // Both read p.terms[ENDING] directly, NOT following_sarvadhatuka: the
    // helper answers "what follows the aṅga", which here is śnā itself — these
    // rules need what follows śnā.

    // 6.4.112 śnābhyastayor ātaḥ: śnā's `ā` is elided before a kṅit
    // sārvadhātuka beginning with a vowel. kliS + nA + anti → kliSnanti;
    // vf + nA + ate → vfRate; vf + nA + e → vfRe.
    //
    // The *abhyasta* half of the sūtra is out of scope — there is no
    // reduplication in this engine — so the guard is śnā's text alone. Widen
    // it when juhotyādi lands.
    Rule {
        id: "6.4.112",
        name: "SnA'ByastayorAtaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms.len() <= ENDING || p.terms[SHAP].text != "nA" {
                return false;
            }
            if !p.terms[ENDING].has(Tag::Ngit) {
                return false;
            }
            let Some(next) = p.terms[ENDING].text.chars().next() else {
                return false;
            };
            if !is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "n".into();
            p.record("6.4.112", "SnA'ByastayorAtaH", before);
            true
        },
    },
    // 6.4.113 ī halyaghoḥ: śnā's `ā` becomes `ī` before a kṅit sārvadhātuka
    // beginning with a consonant. kliS + nA + taH → kliSnItaH; kliS + nA +
    // yAt → kliSnIyAt; vrI + nA + hi → vrIRIhi.
    //
    // *aghoḥ* excludes the ghu roots (√dā, √dhā). They are juhotyādi, out of
    // scope, and no root that can reach this rule is one — so the exclusion is
    // recorded here rather than implemented. Implement it when gaṇa 3 lands.
    Rule {
        id: "6.4.113",
        name: "I halyaGoH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms.len() <= ENDING || p.terms[SHAP].text != "nA" {
                return false;
            }
            if !p.terms[ENDING].has(Tag::Ngit) {
                return false;
            }
            let Some(next) = p.terms[ENDING].text.chars().next() else {
                return false;
            };
            if is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "nI".into();
            p.record("6.4.113", "I halyaGoH", before);
            true
        },
    },
```

Add `ENDING` to `anga.rs`'s `use crate::tinanta::terms::{…}` line if it is not already imported, and `is_vowel` to its `use crate::tinanta::sound::{…}` line.

- [ ] **Step 4: Update the order pin**

In `derivation_tests.rs`, change `"6.1.78", "7.3.101",` to `"6.1.78", "7.3.101", "6.4.112", "6.4.113",`.

- [ ] **Step 5: Run the tests**

Run: `mise run test`
Expected: PASS. The 1080 goldens are untouched — no existing vikaraṇa has the text `nA`.

- [ ] **Step 6: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/anga.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 6.4.112 + 6.4.113, SnA's a ~ I ~ null alternation"
```

---

### Task 5: Rule 3.1.83 — śānac before `hi`

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/vikarana.rs` (new rule between 2.4.72 and 1.2.4, plus tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (order pin)

**Interfaces:**
- Consumes: `p.terms[SHAP].text == "nA"` from Task 3; the `hi` that 3.4.87 produces (Task 2).
- Produces: rule id `"3.1.83"`, positioned between `"2.4.72"` and the second `"1.2.4"`. After it fires, `p.terms[SHAP].text == "Ana"`.

**Both ordering constraints are load-bearing:**
- **Before 6.4.113** (which lives in the later `anga.rs` stage), or śnā's `ā` becomes `ī` and `kliSAna` surfaces as \*`kliSnIhi`.
- **Before the second 1.2.4** (a few lines below in this same file), so that śānac — which is apit — gets tagged ṅit. Without the tag, 7.3.86 guṇates kliś's laghu upadhā and `kliSAna` surfaces as \*`kleSAna`.

- [ ] **Step 1: Write the failing guard tests**

Add to `vikarana.rs`'s `mod tests`:

```rust
    /// `[anga, SnA, ending]`, the shape 3.1.83 inspects.
    fn shna_before(anga: &str, ending: &str) -> Prakriya {
        let mut vik = Term::new("nA");
        vik.add(Tag::Vikarana);
        vik.add(Tag::Sarvadhatuka);
        Prakriya {
            terms: vec![Term::new(anga), vik, Term::new(ending)],
            log: vec![],
            ..Default::default()
        }
    }

    #[test]
    fn halah_shnah_shanac_replaces_shna_after_a_consonant_final_root() {
        // kliS + nA + hi -> kliS + Ana + hi; 6.4.105 ato heH (adesha stage)
        // then drops the hi, giving kliSAna.
        let mut p = shna_before("kliS", "hi");
        let rule = rules().find(|r| r.id == "3.1.83").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "Ana");
        assert!(p.terms[SHAP].has(Tag::Vikarana));
        assert!(!p.terms[SHAP].has(Tag::Pit)); // apit: the next 1.2.4 tags it
    }

    #[test]
    fn halah_shnah_shanac_declines_after_a_vowel_final_root() {
        // "halaH" is the whole condition. vrI is vowel-final, so it keeps SnA
        // and takes 6.4.113 instead: vrIRIhi, not *vrIRAna. This pair is the
        // rule's shape guard.
        let mut p = shna_before("vrI", "hi");
        let rule = rules().find(|r| r.id == "3.1.83").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nA");
    }

    #[test]
    fn halah_shnah_shanac_declines_for_endings_other_than_hi() {
        // The sutra is conditioned on hi alone. A mutant dropping this would
        // rewrite the entire consonant-final paradigm as *kliSAnati.
        for ending in ["ti", "taH", "anti", "tAt"] {
            let mut p = shna_before("kliS", ending);
            let rule = rules().find(|r| r.id == "3.1.83").unwrap();
            assert!(!(rule.apply)(&mut p), "fired on {ending}");
            assert_eq!(p.terms[SHAP].text, "nA");
        }
    }

    #[test]
    fn halah_shnah_shanac_ignores_other_vikaranas_and_short_prakriyas() {
        for vikarana in ["a", "ya", ""] {
            let mut p = shna_before("kliS", "hi");
            p.terms[SHAP].text = vikarana.to_string();
            let rule = rules().find(|r| r.id == "3.1.83").unwrap();
            assert!(!(rule.apply)(&mut p), "fired on {vikarana:?}");
        }
        // A one-term prakriya must not panic indexing SHAP or ENDING.
        let mut p = Prakriya {
            terms: vec![Term::new("kliS")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "3.1.83").unwrap();
        assert!(!(rule.apply)(&mut p));
    }
```

- [ ] **Step 2: Run to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya halah_shnah_shanac`
Expected: FAIL — the `unwrap()` on `find(|r| r.id == "3.1.83")` panics.

- [ ] **Step 3: Add the rule**

In `vikarana.rs`, insert between the 2.4.72 rule and the 1.2.4 rule:

```rust
    // 3.1.83 halaḥ śnaḥ śānac: after a CONSONANT-final root, with `hi`
    // following, śnā is replaced wholesale by śānac. it-samjña strips the
    // leading S (1.3.8) and the final c (1.3.3), leaving `Ana`; the existing
    // 6.4.105 ato heḥ then elides the hi after śāna's short `a`, giving
    // kliSAna. No new rule is needed for the hi-lopa.
    //
    // Placement carries two constraints, both failing visibly:
    //   - BEFORE 6.4.113 (anga stage, later): that rule would otherwise turn
    //     śnā's ā into ī before the consonant-initial ṅit `hi` and give
    //     *kliSnIhi. As an apavāda, 3.1.83 must remove śnā first.
    //   - BEFORE the second 1.2.4, immediately below: śānac is apit and must
    //     be tagged ṅit, or 7.3.86 guṇates kliS's laghu upadhā and the form
    //     surfaces as *kleSAna.
    //
    // Vowel-final roots fall outside "halaḥ" and keep śnā, taking 6.4.113 to
    // vrIRIhi. That pair — kliSAna against vrIRIhi — is the rule's pin.
    //
    // Its id is 3.1.x but it lives after the 3.1.68 boundary, so it addresses
    // the ending as ENDING (index 2). Stage placement is by pipeline position,
    // not sūtra family; see `super::terms`. The `hi` it reads already exists:
    // 3.4.87 ser hyapic ca runs in the earlier `tin` stage.
    Rule {
        id: "3.1.83",
        name: "halaH SnaH SAnajJO",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms.len() <= ENDING || p.terms[SHAP].text != "nA" {
                return false;
            }
            if p.terms[ENDING].text != "hi" {
                return false;
            }
            let Some(last) = p.terms[ANGA].text.chars().last() else {
                return false;
            };
            if is_vowel(last) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("SAnac");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms[SHAP] = s;
            p.record("3.1.83", "halaH SnaH SAnajJO", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S, 1.3.3 strips c → Ana
            p.terms[SHAP] = s;
            true
        },
    },
```

Add `ANGA`, `ENDING` to `vikarana.rs`'s `use crate::tinanta::terms::{…}` line as needed, and `use crate::tinanta::sound::is_vowel;`.

Note the rule `name` is `"halaH SnaH SAnajJO"` — copied verbatim from `sutrapatha.tsv`, which is the repo's citation reference. Do not "correct" it to a reconstructed reading.

- [ ] **Step 4: Update the order pin**

In `derivation_tests.rs`, change `"3.1.68", "2.4.72", "1.2.4",` to `"3.1.68", "2.4.72", "3.1.83", "1.2.4",`.

- [ ] **Step 5: Run the tests**

Run: `mise run test`
Expected: PASS, 1080 goldens untouched.

- [ ] **Step 6: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/vikarana.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 3.1.83 halaH SnaH SAnac — the lot 2sg SAnac path"
```

---

### Task 6: Land the three slice-9a roots and their 108 goldens

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (`DHATUS`)
- Modify: `data/dhatupatha.tsv`, `data/ATTRIBUTION.md`
- Modify: `crates/panini/tests/paradigm.rs` (+12 entries)
- Modify: `crates/panini/tests/trace.rs` (+5 traces)

**Interfaces:**
- Consumes: `Gana::Kryadi` (Task 3) and rules 3.1.81, 3.1.83, 6.4.112, 6.4.113 (Tasks 3–5).
- Produces: golden coverage for `kliS`, `guD`, `aS` across all four lakāras; grid at 1188.

- [ ] **Step 1: Add the goldens (the failing test)**

Append to `PARADIGM` in `crates/panini/tests/paradigm.rs`, following the existing `(root, lakara_label, [9 forms])` shape and the established non-tāt loṭ convention:

```rust
    ("kliS", "laT", ["kliSnAti", "kliSnItaH", "kliSnanti", "kliSnAsi", "kliSnITaH", "kliSnITa", "kliSnAmi", "kliSnIvaH", "kliSnImaH"]),
    ("kliS", "laN", ["akliSnAt", "akliSnItAm", "akliSnan", "akliSnAH", "akliSnItam", "akliSnIta", "akliSnAm", "akliSnIva", "akliSnIma"]),
    ("kliS", "loT", ["kliSnAtu", "kliSnItAm", "kliSnantu", "kliSAna", "kliSnItam", "kliSnIta", "kliSnAni", "kliSnAva", "kliSnAma"]),
    ("kliS", "viDiliN", ["kliSnIyAt", "kliSnIyAtAm", "kliSnIyuH", "kliSnIyAH", "kliSnIyAtam", "kliSnIyAta", "kliSnIyAm", "kliSnIyAva", "kliSnIyAma"]),
    ("guD", "laT", ["guDnAti", "guDnItaH", "guDnanti", "guDnAsi", "guDnITaH", "guDnITa", "guDnAmi", "guDnIvaH", "guDnImaH"]),
    ("guD", "laN", ["aguDnAt", "aguDnItAm", "aguDnan", "aguDnAH", "aguDnItam", "aguDnIta", "aguDnAm", "aguDnIva", "aguDnIma"]),
    ("guD", "loT", ["guDnAtu", "guDnItAm", "guDnantu", "guDAna", "guDnItam", "guDnIta", "guDnAni", "guDnAva", "guDnAma"]),
    ("guD", "viDiliN", ["guDnIyAt", "guDnIyAtAm", "guDnIyuH", "guDnIyAH", "guDnIyAtam", "guDnIyAta", "guDnIyAm", "guDnIyAva", "guDnIyAma"]),
    ("aS", "laT", ["aSnAti", "aSnItaH", "aSnanti", "aSnAsi", "aSnITaH", "aSnITa", "aSnAmi", "aSnIvaH", "aSnImaH"]),
    ("aS", "laN", ["ASnAt", "ASnItAm", "ASnan", "ASnAH", "ASnItam", "ASnIta", "ASnAm", "ASnIva", "ASnIma"]),
    ("aS", "loT", ["aSnAtu", "aSnItAm", "aSnantu", "aSAna", "aSnItam", "aSnIta", "aSnAni", "aSnAva", "aSnAma"]),
    ("aS", "viDiliN", ["aSnIyAt", "aSnIyAtAm", "aSnIyuH", "aSnIyAH", "aSnIyAtam", "aSnIyAta", "aSnIyAm", "aSnIyAva", "aSnIyAma"]),
```

Match the exact formatting `rustfmt` produces for the existing entries — run `mise run fmt` before committing rather than hand-wrapping.

If `paradigm.rs` carries a hard-coded total (e.g. an assertion that the grid is 1080 forms), update it to 1188.

- [ ] **Step 2: Run to verify it fails**

Run: `mise exec -- cargo test -p panini --test paradigm`
Expected: FAIL — the three roots are not in `DHATUS`, so `derive` has nothing to produce. The exact failure depends on how `paradigm.rs` looks roots up; a lookup panic or an empty-form mismatch are both expected here.

- [ ] **Step 3: Add the roots**

Append to `DHATUS` in `crates/panini-data/src/lib.rs`:

```rust
    Dhatu {
        code: "kliS",
        gana: Gana::Kryadi,
        pada: Pada::Parasmaipada,
        artha: "vibADane",
    },
    Dhatu {
        code: "guD",
        gana: Gana::Kryadi,
        pada: Pada::Parasmaipada,
        artha: "roze",
    },
    Dhatu {
        code: "aS",
        gana: Gana::Kryadi,
        pada: Pada::Parasmaipada,
        artha: "Bojane",
    },
```

Append to `data/dhatupatha.tsv` (tab-separated, no header):

```
kliS	kryadi	parasmaipada	vibADane
guD	kryadi	parasmaipada	roze
aS	kryadi	parasmaipada	Bojane
```

In `data/ATTRIBUTION.md`, extend the cross-reference note to record that the kryādi entries correspond to Dhātupāṭha numbers 09.0058 (`kliSU~`), 09.0053 (`guDa~`) and 09.0059 (`aSa~`).

- [ ] **Step 4: Run the goldens**

Run: `mise exec -- cargo test -p panini --test paradigm`
Expected: PASS, 1188 forms, including `paradigm_covers_every_enumerable_cell`.

If a form differs, **do not edit the golden**. The goldens were generated from `vidyut-prakriya` and verified; a mismatch means a rule is wrong. The most likely culprits, in order: the ending is not tagged ṅit (Tasks 1–2), 6.4.112/6.4.113 are placed too early in `anga.rs` (Task 4), or 3.1.83 is ordered after 6.4.113 (Task 5).

- [ ] **Step 5: Add the traces**

Add to `crates/panini/tests/trace.rs`:

```rust
#[test]
fn klishnati_trace_is_the_shna_pit_path() {
    // kliS kryAdi lat 3sg: tip is PIT, so neither 6.4.112 nor 6.4.113 fires
    // and SnA's A survives -- this is the baseline the whole paradigm splits
    // away from. 7.3.86 does NOT appear: SnA is apit, the second 1.2.4 makes
    // it Nit, and 1.1.5 blocks guNa of kliS's laghu upadha `i`.
    let t = trace_for("kliSnAti");
    assert!(t.contains(&"3.1.81".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.112".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.86".to_string()), "got {t:?}");
}

#[test]
fn klishnitah_trace_takes_i_halyaghoh() {
    // tas is apit -> Nit (1.2.4) and consonant-initial -> 6.4.113 gives nI.
    let t = trace_for("kliSnItaH");
    assert!(t.contains(&"1.2.4".to_string()), "got {t:?}");
    assert!(t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.112".to_string()), "got {t:?}");
}

#[test]
fn klishnanti_trace_takes_shnabhyastayor_atah() {
    // Ji -> anti (7.1.3) must precede 6.4.112, or the ending is not yet
    // vowel-initial and the A survives as *kliSnAnti.
    let t = trace_for("kliSnanti");
    let i713 = t.iter().position(|r| r == "7.1.3").expect("7.1.3 present");
    let i6412 = t
        .iter()
        .position(|r| r == "6.4.112")
        .expect("6.4.112 present");
    assert!(i713 < i6412, "7.1.3 must precede 6.4.112: {t:?}");
    assert!(!t.contains(&"6.4.113".to_string()), "got {t:?}");
}

#[test]
fn klishana_trace_is_shanac_then_ato_heh() {
    // lot 2sg after a consonant-final root: 3.1.83 replaces SnA by SAnac,
    // then the existing 6.4.105 ato heH drops the hi. 6.4.113 must NOT
    // appear -- it would have given *kliSnIhi -- and 7.3.86 must not fire.
    let t = trace_for("kliSAna");
    let i3183 = t.iter().position(|r| r == "3.1.83").expect("3.1.83 present");
    let i64105 = t
        .iter()
        .position(|r| r == "6.4.105")
        .expect("6.4.105 present");
    assert!(i3183 < i64105, "3.1.83 must precede 6.4.105: {t:?}");
    assert!(!t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.86".to_string()), "got {t:?}");
}

#[test]
fn ashnat_trace_takes_the_vowel_initial_anga_augment() {
    // aS is vowel-initial, so lan takes AT (6.4.72) rather than aT, and
    // 6.1.101 merges A + a into A: ASnAt.
    let t = trace_for("ASnAt");
    assert!(t.contains(&"6.4.72".to_string()), "got {t:?}");
    assert!(t.contains(&"6.1.101".to_string()), "got {t:?}");
    assert!(t.contains(&"3.1.81".to_string()), "got {t:?}");
}
```

- [ ] **Step 6: Run the full suite**

Run: `mise run test`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
mise run fmt
git add crates/panini-data/src/lib.rs data/dhatupatha.tsv data/ATTRIBUTION.md \
        crates/panini/tests/paradigm.rs crates/panini/tests/trace.rs
git commit -m "feat: kryadi slice 9a — kliS, guD, aS across four lakaras (1080 -> 1188)"
```

---

### Task 7: Slice 9a docs and verification gate

**Files:**
- Modify: `docs/ARCHITECTURE.md`, `AGENTS.md`

- [ ] **Step 1: Update `docs/ARCHITECTURE.md`**

In the stage table, note that `vikarana.rs` now holds `3.1.69, 3.1.77, 3.1.81, 3.1.68, 2.4.72, 3.1.83, 1.2.4` and that `anga.rs` now runs `6.4.71 … 7.3.101, 6.4.112, 6.4.113`. Change "Four gaṇas are covered" to five, adding kryādi (9) and `Tag::Kryadi`, and add a paragraph:

```markdown
kryādi (gaṇa 9) is thematic — śnā occupies the same `SHAP` slot as śap, śyan
and śa — but it is the first gaṇa whose vikaraṇa is itself reshaped by the
ending: 6.4.112 elides its `ā` before a vowel-initial kṅit sārvadhātuka
(kliSnanti), 6.4.113 turns it into `ī` before a consonant-initial one
(kliSnItaH), and 3.1.83 replaces it wholesale with śānac before `hi` after a
consonant-final root (kliSAna). That split is driven by 1.2.4, which as of
this slice tags parasmaipada apit endings ṅit as well as ātmanepada ones —
the distinction between pit `tip` (kliSnAti) and apit `tas` (kliSnItaH) is
the whole paradigm.
```

- [ ] **Step 2: Update `AGENTS.md`**

Extend the golden-paradigm bullet: 1188 forms, five gaṇas, kryādi's three parasmaipadī roots landed in slice 9a, and a pointer to `docs/superpowers/specs/2026-07-28-kryadi-gana-design.md`.

- [ ] **Step 3: Run the full gate**

```bash
mise run fmt-check
mise run lint
mise run audit
mise run test
```
Expected: all PASS.

- [ ] **Step 4: Run mutation testing**

Invoke the `cargo-mutants` binary directly (the `mise run mutants` shim fails in background shells):

```bash
mise exec -- cargo mutants --package panini-prakriya --test-workspace=true
```
Expected: **zero survivors**. If a mutant survives in one of the new rules, add a rule-level test that kills it. Before concluding a mutant is unkillable, check whether a *downstream* rule is repairing its output — that has masked a real gap in this repo before, and only an ordered-trace pin could see it.

- [ ] **Step 5: Commit**

```bash
git add docs/ARCHITECTURE.md AGENTS.md
git commit -m "docs: ARCHITECTURE.md and AGENTS.md cover kryadi and the apit layer"
```

---

## Slice 9b — ṇatva and the ātmanepada root

### Task 8: The ṇatva sound classifiers

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/sound.rs`

**Interfaces:**
- Produces: `pub(crate) fn is_natva_trigger(c: char) -> bool` and `pub(crate) fn is_natva_intervener(c: char) -> bool`, consumed by Task 9.

- [ ] **Step 1: Write the failing tests**

Add to `sound.rs`'s `mod tests`:

```rust
    #[test]
    fn natva_trigger_is_ra_sha_and_the_r_vowels() {
        // 8.4.1 "razAByAm": r and z. f/F (R/RR) count too -- they contain the
        // r-sound by 1.1.51 uraN raparaH, and that is the ONLY reason vfN
        // retroflexes (vf + nIte -> vfRIte).
        for c in ['r', 'z', 'f', 'F'] {
            assert!(is_natva_trigger(c), "{c} should trigger Natva");
        }
        // S is NOT z. varSanti keeps its dental n precisely because of this.
        for c in ['S', 's', 'n', 'a', 'l', 'v'] {
            assert!(!is_natva_trigger(c), "{c} should not trigger Natva");
        }
    }

    #[test]
    fn natva_intervener_is_at_ku_pu_and_nothing_else() {
        // 8.4.2 aw-ku-pu-AN-num-vyavAye'pi. aw = the vowels plus h y v r.
        for c in [
            'a', 'A', 'i', 'I', 'u', 'U', 'f', 'F', 'x', 'X', 'e', 'E', 'o', 'O', 'h', 'y', 'v',
            'r',
        ] {
            assert!(is_natva_intervener(c), "aw member {c} should intervene");
        }
        // ku = k K g G N
        for c in ['k', 'K', 'g', 'G', 'N'] {
            assert!(is_natva_intervener(c), "ku member {c} should intervene");
        }
        // pu = p P b B m
        for c in ['p', 'P', 'b', 'B', 'm'] {
            assert!(is_natva_intervener(c), "pu member {c} should intervene");
        }
        // Everything else BREAKS the intervention. These three are the ones
        // that protect existing goldens: S (varSanti), t (avartanta), and the
        // retroflex R itself (amuzRan's final n stays dental).
        for c in ['S', 's', 'z', 't', 'T', 'd', 'D', 'n', 'R', 'c', 'j', 'w', 'q', 'l'] {
            assert!(!is_natva_intervener(c), "{c} must break intervention");
        }
    }
```

- [ ] **Step 2: Run to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya natva_`
Expected: FAIL to compile — the functions do not exist.

- [ ] **Step 3: Implement the classifiers**

Add to `sound.rs`, after `is_khar`:

```rust
/// 8.4.1's trigger set: `r`, `z`, and the r-vowels `f`/`F`, which contain the
/// r-sound by 1.1.51 *uraṇ raparaḥ*. `S` is deliberately absent — it is not
/// `z`, which is why varSanti keeps its dental `n`.
pub(crate) fn is_natva_trigger(c: char) -> bool {
    matches!(c, 'r' | 'z' | 'f' | 'F')
}

/// 8.4.2's intervention set: aṭ (the vowels plus `h y v r`), ku (`k K g G N`)
/// and pu (`p P b B m`).
///
/// The sūtra also names **āṅ** and **num**, which are morphemes rather than
/// varṇa classes. Ṇatva runs in the tripādī over assembled text, where
/// morpheme identity is gone — and neither is a loss: āṅ is the upasarga `ā`,
/// already an aṭ vowel, and num's nasal cannot occur in the intervening
/// position for any form in the covered grammar (no num-infixing root is in
/// scope, and upasargas are out of scope entirely). Revisit when either
/// enters scope.
///
/// Note `r` and the r-vowels are BOTH triggers and interveners. Callers must
/// test for a trigger first; see 8.4.2's backward scan.
pub(crate) fn is_natva_intervener(c: char) -> bool {
    is_vowel(c) || matches!(c, 'h' | 'y' | 'v' | 'r' | 'k' | 'K' | 'g' | 'G' | 'N' | 'p' | 'P' | 'b' | 'B' | 'm')
}
```

- [ ] **Step 4: Run to verify they pass**

Run: `mise exec -- cargo test -p panini-prakriya natva_`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
mise run fmt
git add crates/panini-prakriya/src/tinanta/sound.rs
git commit -m "feat(prakriya): Natva trigger and intervention classifiers for 8.4.1/8.4.2"
```

---

### Task 9: Rules 8.4.1 and 8.4.2

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (helper + two rules at the end of `TRIPADI`, plus tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (order pin)

**Interfaces:**
- Consumes: `is_natva_trigger`, `is_natva_intervener` (Task 8), `is_jhal` (existing).
- Produces: rule ids `"8.4.1"` then `"8.4.2"`, appended after `"8.4.55"`.

- [ ] **Step 1: Write the failing tests**

Add to `tripadi.rs`'s `mod tests`:

```rust
    fn natva_prakriya(anga: &str, vikarana: &str, ending: &str) -> Prakriya {
        Prakriya {
            terms: vec![
                Term::new(anga),
                Term::new(vikarana),
                Term::new(ending),
            ],
            log: vec![],
            ..Default::default()
        }
    }

    #[test]
    fn natva_fires_adjacent_under_8_4_1() {
        // muz + nA + ti: z directly precedes the n.
        let mut p = natva_prakriya("muz", "nA", "ti");
        let rule = rules().find(|r| r.id == "8.4.1").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "muzRAti");
        // vf + nI + te: the r-vowel triggers it (1.1.51).
        let mut p = natva_prakriya("vf", "nI", "te");
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "vfRIte");
    }

    #[test]
    fn natva_fires_across_intervention_under_8_4_2() {
        // vrI + nA + ti: r, then the aw vowel I, then n. 8.4.1 must DECLINE
        // here (not adjacent) and 8.4.2 must fire.
        let mut p = natva_prakriya("vrI", "nA", "ti");
        let r841 = rules().find(|r| r.id == "8.4.1").unwrap();
        assert!(!(r841.apply)(&mut p), "8.4.1 must not fire non-adjacently");
        let r842 = rules().find(|r| r.id == "8.4.2").unwrap();
        assert!((r842.apply)(&mut p));
        assert_eq!(p.text(), "vrIRAti");
        // muz + Ana (the SAnac form): z, the aw vowel A, then n.
        let mut p = natva_prakriya("muz", "Ana", "");
        assert!((r842.apply)(&mut p));
        assert_eq!(p.text(), "muzARa");
    }

    #[test]
    fn natva_declines_word_finally_per_8_4_37() {
        // asmaran: r, the aw vowel a, then a WORD-FINAL n. 8.4.37 padAntasya
        // forbids Natva there. This is an existing golden -- a mutant that
        // drops this guard breaks the 1080, not just this test.
        assert_eq!(
            form_g("smf", Lakara::Lan, Purusha::Prathama, Vacana::Bahu),
            "asmaran"
        );
        let mut p = natva_prakriya("a", "smar", "an");
        for id in ["8.4.1", "8.4.2"] {
            let rule = rules().find(|r| r.id == id).unwrap();
            assert!(!(rule.apply)(&mut p), "{id} fired word-finally");
        }
        assert_eq!(p.text(), "asmaran");
    }

    #[test]
    fn natva_declines_before_a_jhal_because_8_3_24_bleeds_it() {
        // BAzante: z, the aw vowel a, then n -- but the n is followed by the
        // jhal `t`. In the full grammar 8.3.24 naS cApadAntasya jhali has
        // already made that n an anusvAra by the time 8.4.1 runs, and 8.4.58
        // restores it afterwards. This engine has no anusvAra machinery, so
        // the bleeding is encoded as this guard. Another existing golden.
        assert_eq!(
            form_g("BAz", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "BAzante"
        );
        let mut p = natva_prakriya("BAz", "a", "nte");
        for id in ["8.4.1", "8.4.2"] {
            let rule = rules().find(|r| r.id == id).unwrap();
            assert!(!(rule.apply)(&mut p), "{id} fired before a jhal");
        }
        assert_eq!(p.text(), "BAzante");
    }

    #[test]
    fn natva_declines_when_a_non_intervener_breaks_the_run() {
        // varSanti: S is not z and not an aw member, so it breaks the run
        // between r and n. avartanta: t likewise. Both are existing goldens.
        for (anga, vikarana, ending) in [("varS", "a", "nti"), ("a", "varta", "nta")] {
            let mut p = natva_prakriya(anga, vikarana, ending);
            let before = p.text();
            for id in ["8.4.1", "8.4.2"] {
                let rule = rules().find(|r| r.id == id).unwrap();
                assert!(!(rule.apply)(&mut p), "{id} fired on {before}");
            }
            assert_eq!(p.text(), before);
        }
    }
```

- [ ] **Step 2: Run to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya natva_`
Expected: the four new `natva_*` rule tests FAIL on the `unwrap()` (rules absent). The Task-8 classifier tests still pass.

- [ ] **Step 3: Add the helper and the rules**

At the top of `tripadi.rs`, after the `use` lines:

```rust
/// The assembled word as `(term index, char index, char)`, so a tripādī rule
/// can reason over the whole pada and still write back into the right term.
fn word_chars(p: &Prakriya) -> Vec<(usize, usize, char)> {
    let mut out = Vec::new();
    for (ti, t) in p.terms.iter().enumerate() {
        for (ci, c) in t.text.chars().enumerate() {
            out.push((ti, ci, c));
        }
    }
    out
}

/// Replace one character of one term, addressed as `word_chars` reports it.
fn set_char(p: &mut Prakriya, term: usize, idx: usize, to: char) {
    let mut s: Vec<char> = p.terms[term].text.chars().collect();
    s[idx] = to;
    p.terms[term].text = s.into_iter().collect();
}

/// Shared precondition for 8.4.1 and 8.4.2: the `n` at `i` is a legal target.
///
/// Two sūtras are folded in here as guards rather than modelled as rules,
/// which is this slice's one stated simplification:
///   - **8.4.37 padāntasya**: ṇatva never applies to a word-final n
///     (asmaran, not *asmaraR).
///   - **8.3.24 naś cāpadāntasya jhali**: a non-padānta n before a jhal has
///     ALREADY become an anusvāra by the time the 8.4 rules run, and 8.4.58
///     restores it afterwards — so no such n can be a target (BAzante, not
///     *BAzaRte). This engine has no anusvāra machinery; the condition below
///     is exactly equivalent within tripādī order.
/// Retire both in favour of the real rules when liṭ/luṅ bring 8.3.24 in.
fn is_natva_target(w: &[(usize, usize, char)], i: usize) -> bool {
    if w[i].2 != 'n' {
        return false;
    }
    if i + 1 == w.len() {
        return false; // 8.4.37 padAntasya
    }
    !is_jhal(w[i + 1].2) // 8.3.24 has already bled this case
}
```

Then append to `TRIPADI`, after 8.4.55:

```rust
    // 8.4.1 raṣābhyāṁ no ṇaḥ samānapade: `n` → `ṇ` when `r`/`ṣ` DIRECTLY
    // precedes it within the same pada. muz + nAti → muzRAti; vf + nIte →
    // vfRIte (the r-vowel triggers it by 1.1.51 uraṇ raparaḥ).
    //
    // The engine's first ṇatva. Kept disjoint from 8.4.2 — adjacency here,
    // intervention there — so a trace names the sūtra that actually applied.
    Rule {
        id: "8.4.1",
        name: "razAByAM no RaH samAnapade",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let w = word_chars(p);
            for i in 0..w.len() {
                if !is_natva_target(&w, i) || i == 0 {
                    continue;
                }
                if !is_natva_trigger(w[i - 1].2) {
                    continue;
                }
                let before = p.snapshot();
                set_char(p, w[i].0, w[i].1, 'R');
                p.record("8.4.1", "razAByAM no RaH samAnapade", before);
                return true;
            }
            false
        },
    },
    // 8.4.2 aṭkupvāṅnumvyavāye'pi: 8.4.1 applies even when aṭ, ku or pu
    // intervene. vrI + nAti → vrIRAti (the aṭ vowel `I`); muz + Ana → muzARa
    // (the aṭ vowel `A`).
    //
    // The backward scan takes the NEAREST trigger, and must test for a
    // trigger BEFORE testing for an intervener: `r` and the r-vowels are in
    // both sets, so a greedy intervener scan would walk straight past the `r`
    // of `vrI` and find nothing.
    //
    // `j == i` means nothing intervened — that is 8.4.1's case, and this rule
    // declines so the trace credits the right sūtra.
    Rule {
        id: "8.4.2",
        name: "awkupvANnumvyavAye'pi",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let w = word_chars(p);
            for i in 0..w.len() {
                if !is_natva_target(&w, i) {
                    continue;
                }
                let mut j = i;
                let fired = loop {
                    if j == 0 {
                        break false;
                    }
                    let c = w[j - 1].2;
                    if is_natva_trigger(c) {
                        break j < i;
                    }
                    if !is_natva_intervener(c) {
                        break false;
                    }
                    j -= 1;
                };
                if !fired {
                    continue;
                }
                let before = p.snapshot();
                set_char(p, w[i].0, w[i].1, 'R');
                p.record("8.4.2", "awkupvANnumvyavAye'pi", before);
                return true;
            }
            false
        },
    },
```

Copy the two `name` strings verbatim from `sutrapatha.tsv` — `razAByAM no RaH samAnapade` and `awkupvANnumvyavAye'pi`. Use the same string in the `Rule` field and in the `p.record` call; a mismatch between them is a silent trace bug.

Extend `tripadi.rs`'s `use crate::tinanta::sound::{…}` to bring in `is_natva_trigger` and `is_natva_intervener`.

- [ ] **Step 4: Update the order pin**

In `derivation_tests.rs`, change the tail `"8.3.59", "8.4.55",` to `"8.3.59", "8.4.55", "8.4.1", "8.4.2",`.

- [ ] **Step 5: Run the tests**

Run: `mise run test`
Expected: PASS, all 1188 goldens unchanged. The four existing goldens named in the new tests — `asmaran`, `BAzante`, `varSanti`, `avartanta` — are the tripwires; if any moved, the guards are wrong. Do not edit a golden.

- [ ] **Step 6: Commit**

```bash
mise run fmt
git add crates/panini-prakriya/src/tinanta/tripadi.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 8.4.1 + 8.4.2 Natva, guarded against padanta and pre-jhal n

8.4.37 padAntasya and 8.3.24's anusvAra bleeding are encoded as guards
rather than modelled, since this engine has no anusvAra machinery. Both
are named in is_natva_target and both are pinned by existing goldens
(asmaran, BAzante)."
```

---

### Task 10: Land the three slice-9b roots and their 108 goldens

**Files:**
- Modify: `crates/panini-data/src/lib.rs`, `data/dhatupatha.tsv`, `data/ATTRIBUTION.md`
- Modify: `crates/panini/tests/paradigm.rs` (+12 entries), `crates/panini/tests/trace.rs` (+5 traces)

**Interfaces:**
- Consumes: everything from Tasks 3–9.
- Produces: grid at 1296.

- [ ] **Step 1: Add the goldens (the failing test)**

Append to `PARADIGM`:

```rust
    ("muz", "laT", ["muzRAti", "muzRItaH", "muzRanti", "muzRAsi", "muzRITaH", "muzRITa", "muzRAmi", "muzRIvaH", "muzRImaH"]),
    ("muz", "laN", ["amuzRAt", "amuzRItAm", "amuzRan", "amuzRAH", "amuzRItam", "amuzRIta", "amuzRAm", "amuzRIva", "amuzRIma"]),
    ("muz", "loT", ["muzRAtu", "muzRItAm", "muzRantu", "muzARa", "muzRItam", "muzRIta", "muzRAni", "muzRAva", "muzRAma"]),
    ("muz", "viDiliN", ["muzRIyAt", "muzRIyAtAm", "muzRIyuH", "muzRIyAH", "muzRIyAtam", "muzRIyAta", "muzRIyAm", "muzRIyAva", "muzRIyAma"]),
    ("vrI", "laT", ["vrIRAti", "vrIRItaH", "vrIRanti", "vrIRAsi", "vrIRITaH", "vrIRITa", "vrIRAmi", "vrIRIvaH", "vrIRImaH"]),
    ("vrI", "laN", ["avrIRAt", "avrIRItAm", "avrIRan", "avrIRAH", "avrIRItam", "avrIRIta", "avrIRAm", "avrIRIva", "avrIRIma"]),
    ("vrI", "loT", ["vrIRAtu", "vrIRItAm", "vrIRantu", "vrIRIhi", "vrIRItam", "vrIRIta", "vrIRAni", "vrIRAva", "vrIRAma"]),
    ("vrI", "viDiliN", ["vrIRIyAt", "vrIRIyAtAm", "vrIRIyuH", "vrIRIyAH", "vrIRIyAtam", "vrIRIyAta", "vrIRIyAm", "vrIRIyAva", "vrIRIyAma"]),
    ("vf", "laT", ["vfRIte", "vfRAte", "vfRate", "vfRIze", "vfRATe", "vfRIDve", "vfRe", "vfRIvahe", "vfRImahe"]),
    ("vf", "laN", ["avfRIta", "avfRAtAm", "avfRata", "avfRITAH", "avfRATAm", "avfRIDvam", "avfRi", "avfRIvahi", "avfRImahi"]),
    ("vf", "loT", ["vfRItAm", "vfRAtAm", "vfRatAm", "vfRIzva", "vfRATAm", "vfRIDvam", "vfRE", "vfRAvahE", "vfRAmahE"]),
    ("vf", "viDiliN", ["vfRIta", "vfRIyAtAm", "vfRIran", "vfRITAH", "vfRIyATAm", "vfRIDvam", "vfRIya", "vfRIvahi", "vfRImahi"]),
```

Update any hard-coded grid total to 1296.

- [ ] **Step 2: Run to verify it fails**

Run: `mise exec -- cargo test -p panini --test paradigm`
Expected: FAIL — the roots are not in `DHATUS`.

- [ ] **Step 3: Add the roots**

Append to `DHATUS`:

```rust
    Dhatu {
        code: "muz",
        gana: Gana::Kryadi,
        pada: Pada::Parasmaipada,
        artha: "steye",
    },
    Dhatu {
        code: "vrI",
        gana: Gana::Kryadi,
        pada: Pada::Parasmaipada,
        artha: "varaRe",
    },
    Dhatu {
        code: "vf",
        gana: Gana::Kryadi,
        pada: Pada::Atmanepada,
        artha: "samBaktO",
    },
```

Append to `data/dhatupatha.tsv`:

```
muz	kryadi	parasmaipada	steye
vrI	kryadi	parasmaipada	varaRe
vf	kryadi	atmanepada	samBaktO
```

Extend `data/ATTRIBUTION.md` with Dhātupāṭha numbers 09.0066 (`muza~`), 09.0040 (`vrI\`) and 09.0045 (`vfN`). Note there that **√vṛṅ is the only pure-ātmanepadī root in the gaṇa** — every other ātmanepada form in kryādi belongs to an ubhayapadī root, which the one-pada-per-root model does not carry.

- [ ] **Step 4: Run the goldens**

Run: `mise exec -- cargo test -p panini --test paradigm`
Expected: PASS, 1296 forms.

If `vfRIta` (vidhiliṅ 3sg) comes out as \*`vfReta`, 6.4.112 is running after `adesha.rs`'s 6.1.87 — check Task 4's placement. If it comes out as \*`vfRIsIyta`, 6.4.112/6.4.113 are running before 7.2.79. Do not edit the golden.

- [ ] **Step 5: Add the traces**

```rust
#[test]
fn mushnati_trace_takes_adjacent_natva() {
    // z directly precedes SnA's n -> 8.4.1, not 8.4.2.
    let t = trace_for("muzRAti");
    assert!(t.contains(&"8.4.1".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.2".to_string()), "got {t:?}");
}

#[test]
fn vrinati_trace_takes_intervening_natva() {
    // r, then the aw vowel I, then n -> 8.4.2, not 8.4.1.
    let t = trace_for("vrIRAti");
    assert!(t.contains(&"8.4.2".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.1".to_string()), "got {t:?}");
}

#[test]
fn mushana_trace_is_shanac_plus_intervening_natva() {
    // lot 2sg: 3.1.83 gives Ana, 6.4.105 drops the hi, and 8.4.2 then
    // retroflexes across the A. Both rules in one derivation.
    let t = trace_for("muzARa");
    assert!(t.contains(&"3.1.83".to_string()), "got {t:?}");
    assert!(t.contains(&"6.4.105".to_string()), "got {t:?}");
    assert!(t.contains(&"8.4.2".to_string()), "got {t:?}");
}

#[test]
fn vrinite_trace_is_the_atmanepada_shna_path() {
    // vf + SnA + te: te is apit -> Nit (1.2.4), consonant-initial -> 6.4.113,
    // and the r-vowel triggers 8.4.1.
    let t = trace_for("vfRIte");
    assert!(t.contains(&"3.1.81".to_string()), "got {t:?}");
    assert!(t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(t.contains(&"8.4.1".to_string()), "got {t:?}");
}

#[test]
fn vrinishva_trace_reaches_the_existing_shatva() {
    // lot 2sg atmanepada: 6.4.113 gives nI, and the existing 8.3.59
    // AdeSapratyayayoH then retroflexes sva's s after that I -> vfRIzva.
    let t = trace_for("vfRIzva");
    assert!(t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(t.contains(&"8.3.59".to_string()), "got {t:?}");
}
```

`vfRIzva` and `vfRIze` will **fail** at this point, and the fix is required — do it as Steps 5a–5c below before re-running.

8.3.59 locates the affix as "the first non-empty term after the aṅga" and then reads `p.terms[ANGA]`'s last char as the preceding sound. For kryādi that first non-empty term is śnā (`nI`), not `sva`, so the rule declines and `vfRIzva` surfaces as \*`vfRIsva`. The retroflexion trigger here is śnā's `ī`, which is not in the aṅga at all — the assumption that the aṅga directly precedes the affix was true for every earlier gaṇa and is false for the first time here.

- [ ] **Step 5a: Widen 8.3.59 to read the actual preceding sound**

In `crates/panini-prakriya/src/tinanta/tripadi.rs`, replace 8.3.59's two lookups:

```rust
            let next_idx = p
                .terms
                .iter()
                .enumerate()
                .skip(ANGA + 1)
                .find(|(_, t)| !t.text.is_empty())
                .map(|(i, _)| i);
            let Some(next_idx) = next_idx else {
                return false;
            };
            if !p.terms[next_idx].text.starts_with('s') {
                return false;
            }
            let Some(anga_last) = p.terms[ANGA].text.chars().last() else {
                return false;
            };
            if !is_vowel(anga_last) || matches!(anga_last, 'a' | 'A') {
                return false;
            }
```

with:

```rust
            // The affix whose s retroflexes: the first s-initial term after
            // the aṅga. Searching for the s-initial term — rather than taking
            // the first non-empty one and testing it — is what lets a
            // non-empty vikaraṇa sit between the aṅga and the affix.
            let next_idx = p
                .terms
                .iter()
                .enumerate()
                .skip(ANGA + 1)
                .find(|(_, t)| t.text.starts_with('s'))
                .map(|(i, _)| i);
            let Some(next_idx) = next_idx else {
                return false;
            };
            // The iṇ-koḥ trigger is the sound IMMEDIATELY before that affix —
            // the last char of the nearest non-empty preceding term, which is
            // the aṅga only when nothing intervenes. For kryādi it is śnā's
            // `ī` (vf + nI + sva → vfRIzva); reading ANGA here would ask
            // about `f` and miss the rule entirely.
            let Some(prev) = p.terms[..next_idx]
                .iter()
                .rev()
                .find_map(|t| t.text.chars().last())
            else {
                return false;
            };
            if !is_vowel(prev) || matches!(prev, 'a' | 'A') {
                return false;
            }
```

`ANGA` may become unused in this rule; leave the import if other rules in the file still use it, and drop it from the `use` line only if `cargo lint` complains.

- [ ] **Step 5b: Add the inside/outside test**

Add to `tripadi.rs`'s `mod tests`:

```rust
    #[test]
    fn shatva_reads_the_sound_before_the_affix_not_the_anga() {
        // vf + nI + sva: the iN trigger is SnA's I, not the anga's f. The
        // pre-kryadi guard read ANGA and would have declined here.
        let mut p = Prakriya {
            terms: vec![Term::new("vf"), Term::new("nI"), Term::new("sva")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "8.3.59").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "vfnIzva");
        // And the thematic case still declines on the vikaraNa's `a`, which
        // is what keeps laBasva intact.
        let mut p = Prakriya {
            terms: vec![Term::new("laB"), Term::new("a"), Term::new("sva")],
            log: vec![],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "laBasva");
    }
```

- [ ] **Step 5c: Confirm the five existing 8.3.59 goldens are unmoved**

Run: `mise exec -- cargo test -p panini-prakriya shatva`
Expected: PASS, including the existing `shatva_declines_for_every_pre_existing_junction`, which pins `Assva` (aṅga-final `s`, not a vowel), `vasse` (same), and `laBasva` (the śap's `a`). `Seze` and `Sezva` are pinned by the goldens and by `trace.rs`.

Then continue with the trace additions above.

- [ ] **Step 6: Run the full suite**

Run: `mise run test`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
mise run fmt
git add crates/panini-data/src/lib.rs data/dhatupatha.tsv data/ATTRIBUTION.md \
        crates/panini/tests/paradigm.rs crates/panini/tests/trace.rs \
        crates/panini-prakriya/src/tinanta/tripadi.rs
git commit -m "feat: kryadi slice 9b — muz, vrI, vfN with Natva (1188 -> 1296)"
```

---

### Task 11: Slice 9b docs and the final gate

**Files:**
- Modify: `docs/ARCHITECTURE.md`, `AGENTS.md`

- [ ] **Step 1: Update the docs**

In `docs/ARCHITECTURE.md`, extend the `tripadi.rs` stage row to `8.2.77 … 8.4.55, 8.4.1, 8.4.2` and add:

```markdown
8.4.1 / 8.4.2 are the engine's first ṇatva. They are guarded to skip an `n`
that is word-final or immediately followed by a jhal — the effect of 8.4.37
*padāntasya* and of 8.3.24 *naś cāpadāntasya jhali* bleeding the rule, neither
of which is modelled here because the engine has no anusvāra machinery. The
guard is exactly equivalent within tripādī order; it costs trace fidelity, and
it is the first thing liṭ and luṅ will want retired. `asmaran` and `BAzante`
are the goldens that pin it.
```

In `AGENTS.md`, update the golden-paradigm bullet to 1296 forms and note that kryādi is complete: six roots, √vṛṅ being the gaṇa's only ātmanepadī root, and ubhayapada (1.3.72) still deferred.

- [ ] **Step 2: Run the full gate**

```bash
mise run fmt-check
mise run lint
mise run audit
mise run test
```
Expected: all PASS.

- [ ] **Step 3: Run mutation testing**

```bash
mise exec -- cargo mutants --package panini-prakriya --test-workspace=true
```
Expected: **zero survivors**. The likeliest survivors are in `is_natva_intervener`'s membership list and in 8.4.2's backward-scan loop conditions; kill them with targeted classifier tests and constructed prakriyas rather than by adding goldens.

- [ ] **Step 4: Commit**

```bash
git add docs/ARCHITECTURE.md AGENTS.md
git commit -m "docs: kryadi complete — 1296 goldens, five ganas, first Natva"
```

---

## Verification summary

| gate | after 9a (Task 7) | after 9b (Task 11) |
|---|---|---|
| golden forms | 1188, all `VALID` | 1296, all `VALID` |
| pre-existing surfaces | unmoved | unmoved |
| pre-existing traces | 6 re-pinned (Task 1), no others | no further change |
| rule order pin | 60 ids | 62 ids |
| `fmt-check` / `lint` / `audit` | clean | clean |
| `cargo mutants` | zero survivors | zero survivors |
