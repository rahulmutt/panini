# rudhādi gaṇa, slice 7a — Implementation Plan

> **Correction (post-merge fix wave, 2026-08-10):** This document cites
> `9b7adee` as the commit that removed 8.4.53 jaśtva as unreachable (see the
> occurrences below). That is the wrong hash — `9b7adee` is
> "fix(prakriya): delete 8.4.56's two unreachable guard arms". The commit
> that actually dropped 8.4.53 is `9fa8e5f`, "refactor(prakriya): drop
> 8.4.53 jaśtva — unreachable once 8.2.25 bleeds it". This is a historical
> planning artifact and is left uncorrected below to preserve the original
> record; all shipped-code citations were already fixed during the slice.

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Land rudhādi (gaṇa 7) on three roots — √kṛt, √hiṃs (parasmaipada) and √khid (ātmanepada) — teaching the engine its first *infix* vikaraṇa, śnam.

**Architecture:** 3.1.78 *ruḍhādibhyaḥ śnam* introduces a **mit** affix, which 1.1.47 *mid aco'ntyāt paraḥ* places after the root's last vowel. The pipeline's three fixed term slots `[ANGA, SHAP, ENDING]` have nowhere to put an infix, so the root is **split across the first two**: `ANGA` keeps the head up to and including its last vowel, `SHAP` holds śnam followed by the root's tail. `kft` becomes `[kf, nat, ti]`. Every subsequent rule in this slice is then an ordinary term-local edit.

**Tech Stack:** Rust (workspace pinned to 1.97.1 via `mise`), no new dependencies.

## Global Constraints

- Toolchain is pinned via `mise`. Never install Rust globally. Build/test with `mise exec -- cargo …` or the `mise run` tasks.
- `#![forbid(unsafe_code)]` holds in every crate touched here.
- SLP1 is the only internal representation. No transliteration outside `panini-lipi`.
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, **never** as a branch inside `derive`. The only gaṇa-conditioned logic in `derive` is aṅga tagging.
- Every new rule id must be added to `tinanta_rule_order_is_pinned` in `crates/panini-prakriya/src/tinanta/derivation_tests.rs`, in its pipeline position. Every new **optional** rule must also be added to `exactly_the_pinned_vikalpa_rules_are_optional`.
- Per-rule guard tests live beside the rule in its stage file. Tests asserting a surface form or trace live in `tinanta/derivation_tests.rs`.
- **Guards are written narrow, to the reachable slice**, per the discipline that landed 8.3.59 and 8.2.25. A mutation-test survivor means an arm has no witness: shrink the guard, do not grow the test.
- Sūtra ids and names must match `vidyut-prakriya`'s `data/sutrapatha.tsv`. The exact strings this slice needs are given verbatim in each task.

**Scoped test command:** `mise exec -- cargo test -p panini-prakriya`
(`mise run test -- -p panini-prakriya` does **not** scope — it runs the whole workspace.)

**Full gate:** `mise run test`

**Test-helper imports:** the stage files' `mod tests` blocks reach the shared helpers (`sole`, `declined`, `form_g`, `form_g_forked`) through `mod.rs`'s re-export — `anga.rs` and `tripadi.rs` already import them that way. Copy whichever import line the file you are editing already uses; do not invent a new path.

---

### Task 1: Data layer — `Gana::Rudhadi` and the three roots

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (the `Gana` enum, the `DHATUS` table)
- Modify: `crates/panini-prakriya/src/term.rs` (the `Tag` enum)
- Modify: `crates/panini-prakriya/src/tinanta/mod.rs` (the `match dhatu.gana` in `derive`)
- Modify: `data/dhatupatha.tsv` (reference mirror, not parsed by any code)
- Test: `crates/panini-data/src/lib.rs` (in-file `mod tests`)

**Interfaces:**
- Consumes: nothing.
- Produces: `Gana::Rudhadi`; `Tag::Rudhadi`; three `Dhatu` rows with ids `"kft"`, `"his"`, `"Kid"`. Every later task guards on `p.terms[ANGA].has(Tag::Rudhadi)`.

- [ ] **Step 1: Write the failing test**

Add to the existing `mod tests` in `crates/panini-data/src/lib.rs`:

```rust
    #[test]
    fn rudhadi_holds_exactly_the_slice_7a_roots() {
        // Three roots, and the pada split that decides which arm each
        // exercises. √hiṃs is stored `hins`, NOT `his`: see its row comment.
        let rows: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.gana == Gana::Rudhadi)
            .map(|d| (d.id, d.code, d.pada))
            .collect();
        assert_eq!(
            rows,
            vec![
                ("kft", "kft", Pada::Parasmaipada),
                ("his", "hins", Pada::Parasmaipada),
                ("Kid", "Kid", Pada::Atmanepada),
            ]
        );
    }

    #[test]
    fn slice_7a_ids_do_not_collide() {
        // rudhādi also holds `vi\da~\` and `o~vijI~`, which WOULD collide
        // with divādi's `vid` and tudādi's `vij` — neither is in 7a, and
        // when 7b lands them they need the `aS.5` qualification. These
        // three do not, so id == code for all of them.
        for id in ["kft", "his", "Kid"] {
            let d = dhatus().iter().find(|d| d.id == id).unwrap();
            assert_eq!(
                dhatus().iter().filter(|x| x.code == d.code).count(),
                1,
                "{id}: code {} is not unique",
                d.code
            );
        }
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `mise exec -- cargo test -p panini-data`
Expected: FAIL to compile — `no variant named 'Rudhadi' found for enum 'Gana'`.

- [ ] **Step 3: Add the enum variant, the tag, and the three rows**

In `crates/panini-data/src/lib.rs`, add to `Gana`:

```rust
    Rudhadi,
```

Append to `DHATUS`, after the svādi block:

```rust
    Dhatu {
        // 07.0010 kftI~ vezwane. rudhādi's √kṛt, distinct from tudādi's
        // √kṛnt — not in the root set, so no id qualification is needed.
        id: "kft",
        code: "kft",
        gana: Gana::Rudhadi,
        pada: Pada::Parasmaipada,
        artha: "vezwane",
    },
    Dhatu {
        // 07.0019 hisi~ hiMsAyAm. Stored post-7.1.58 idito num dhātoH: the
        // root is idit and takes num, but the engine models no it-markers
        // at all (every root here is stored post-it-elision), so 7.1.58 is
        // not derivable and the num is stored. A stated simplification, not
        // a derivation step — exactly as `stiG` is stored post-6.1.64.
        // This is the root that makes 6.4.23 SnAnnalopaH reachable: śnam
        // gives hinans, and 6.4.23 takes the root's own n back out.
        id: "his",
        code: "hins",
        gana: Gana::Rudhadi,
        pada: Pada::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 07.0012 Ki\da~\ dEnye. The gaṇa's ātmanepada arm. rudhādi offers
        // only three ānudātta roots (√indh, √khid, √vid); √khid is the one
        // that needs no rule beyond the gaṇa's own.
        id: "Kid",
        code: "Kid",
        gana: Gana::Rudhadi,
        pada: Pada::Atmanepada,
        artha: "dEnye",
    },
```

In `crates/panini-prakriya/src/term.rs`, add to `Tag` after `Svadi`:

```rust
    /// The dhātu belongs to rudhādi (gaṇa 7), whose vikaraṇa is śnam. Read
    /// by 3.1.78 alone. Mirrors Divadi/Tudadi/Adadi/Kryadi/Svadi.
    Rudhadi,
```

In `crates/panini-prakriya/src/tinanta/mod.rs`, add to the `match dhatu.gana` arm list:

```rust
            Gana::Rudhadi => t.add(Tag::Rudhadi),
```

In `data/dhatupatha.tsv`, append the three reference rows in the file's existing column format:

```
07.0010	kftI~	vezwane
07.0012	Ki\da~\	dEnye
07.0019	hisi~	hiMsAyAm
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `mise exec -- cargo test -p panini-data && mise exec -- cargo test -p panini-prakriya`
Expected: PASS. The prakriyā crate must still compile — the new `Gana` arm makes the `match` in `derive` exhaustive again.

- [ ] **Step 5: Commit**

```bash
git add crates/panini-data/src/lib.rs crates/panini-prakriya/src/term.rs \
        crates/panini-prakriya/src/tinanta/mod.rs data/dhatupatha.tsv
git commit -m "feat(data): rudhādi gaṇa and the three slice-7a roots"
```

---

### Task 2: 3.1.78 śnam, and the infix split

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/vikarana.rs` (add the rule + its guard tests)
- Modify: `crates/panini-prakriya/src/tinanta/terms.rs` (document the new `SHAP` caveat, add its unit test)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: `Tag::Rudhadi` from Task 1.
- Produces: the term split every later task edits — after 3.1.78, `terms[ANGA].text` is the root's head through its last vowel and `terms[SHAP].text` is `na` + the root's tail (`kf`/`nat`, `hi`/`nans`, `Ki`/`nad`).

- [ ] **Step 1: Write the failing tests**

Add to the `mod tests` at the bottom of `crates/panini-prakriya/src/tinanta/vikarana.rs`:

```rust
    #[test]
    fn shnam_lands_after_the_roots_last_vowel() {
        // 1.1.47's placement, enumerated. `RuleStep` records only the word
        // before and after, so assert on the stem the step produces: a
        // suffix model would give kftnati, not kfnatti.
        //
        // √hiṃs is the row that matters: its tail is TWO consonants, so a
        // rule that assumed a one-character tail passes on kft and Kid and
        // fails only here.
        for (id, stem) in [("kft", "kfnat"), ("his", "hinans"), ("Kid", "Kinad")] {
            let d = dhatus().iter().find(|d| d.id == id).unwrap();
            let p = sole(derive(d, Lakara::Lat, d.pada, Purusha::Prathama, Vacana::Eka));
            let step = p
                .log
                .iter()
                .find(|s| s.sutra == "3.1.78")
                .unwrap_or_else(|| panic!("{id}: 3.1.78 never fired"));
            assert!(
                step.after.starts_with(stem),
                "{id}: expected stem {stem}, got {}",
                step.after
            );
        }
    }

    #[test]
    fn shnam_declines_outside_rudhadi() {
        // The guard is a gaṇa tag, not a shape test. √kliś would split
        // perfectly well after its `i`, and must not.
        for id in ["BU", "kliS", "Ap", "ad"] {
            let d = dhatus().iter().find(|d| d.id == id).unwrap();
            let branches = derive(d, Lakara::Lat, d.pada, Purusha::Prathama, Vacana::Eka);
            for p in &branches {
                assert!(
                    !p.log.iter().any(|s| s.sutra == "3.1.78"),
                    "{id}: 3.1.78 fired outside rudhādi"
                );
            }
        }
    }
```

Add to the `mod tests` at the bottom of `crates/panini-prakriya/src/tinanta/terms.rs`:

```rust
    #[test]
    fn shap_holds_shnam_plus_the_roots_tail_for_rudhadi() {
        // The load-bearing consequence of the infix representation: for
        // rudhādi, terms[SHAP].text is NOT the vikaraṇa's own text. Any
        // rule that reads SHAP expecting `na` must guard on the gaṇa.
        let mut p = Prakriya {
            terms: vec![Term::new("hi"), Term::new("nans"), Term::new("ti")],
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        assert_ne!(p.terms[SHAP].text, "na");
        assert!(p.terms[SHAP].text.starts_with("na"));
        assert_eq!(sound_before_ending(&p), Some('s'));
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: FAIL — `3.1.78 never fired` for all three roots.

- [ ] **Step 3: Add the rule**

In `crates/panini-prakriya/src/tinanta/vikarana.rs`, insert **between the 3.1.77 and 3.1.81 rules** (numeric order among the mutually-exclusive sibling vikaraṇas, all apavādas to 3.1.68 and all ordered before it):

```rust
    // 3.1.78 rudhādibhyaḥ śnam: rudhādi (gaṇa 7) takes śnam, not śap.
    // Apavāda to 3.1.68, ordered before it exactly as 3.1.69, 3.1.73,
    // 3.1.77 and 3.1.81 are.
    //
    // Unlike every other vikaraṇa, śnam is NOT a suffix. It is **mit**, and
    // 1.1.47 mid aco'ntyāt paraḥ places a mit affix after the last vowel of
    // what it attaches to: kft + śnam is `kfnat`, not kft + na. 1.1.47 is a
    // paribhāṣā and is cited here rather than implemented as its own Rule —
    // the treatment 1.4.13 and 1.1.5 already get, and what vidyut-prakriya's
    // trace does (it emits 3.1.78 and never 1.1.47).
    //
    // REPRESENTATION, load-bearing. The pipeline's three fixed slots
    // [ANGA, SHAP, ENDING] have nowhere to put an infix, so the root is
    // split across the first two: ANGA keeps the head through its last
    // vowel, SHAP holds śnam followed by whatever the root had after it.
    // kft → [kf, nat, ti]; hins → [hi, nans, ti].
    //
    // The consequence — recorded in `super::terms` too — is that
    // terms[SHAP].text is no longer purely the vikaraṇa for this gaṇa.
    // 6.4.23 deletes a nasal that came from the ROOT but now lives in SHAP,
    // and 6.4.111 deletes śnam's own `a` from the same term.
    //
    // The alternative — ANGA holding the whole infixed stem with SHAP empty,
    // the adādi śap-luk shape — was rejected: it forces 6.4.23 and 6.4.111
    // to locate a character by position inside a merged string, which is the
    // failure mode this file's header exists to warn about.
    //
    // ORDERING WITHIN THE RULE: the it-saṁjñā runs BEFORE the root's tail is
    // appended. With the tail already in place, 1.3.3 halantyam would strip
    // the ROOT's final consonant instead of śnam's mit `m`. That is why this
    // rule records 3.1.78 after run_it_samjna rather than before it, unlike
    // its siblings — the recorded step is the whole operation, split and all.
    Rule {
        id: "3.1.78",
        name: "ruDAdiByaH Snam",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Rudhadi) {
                return false;
            }
            let root: Vec<char> = p.terms[ANGA].text.chars().collect();
            let Some(last_vowel) = root.iter().rposition(|c| is_vowel(*c)) else {
                return false;
            };
            let head: String = root[..=last_vowel].iter().collect();
            let tail: String = root[last_vowel + 1..].iter().collect();
            let before = p.snapshot();
            let mut s = Term::new("Snam");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            // 1.3.8 laSakvataddhite strips S; 1.3.3 halantyam strips the
            // mit m. Leaves `na`.
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP);
            p.terms[SHAP] = s;
            // 1.1.47, cited above.
            p.terms[ANGA].text = head;
            p.terms[SHAP].text.push_str(&tail);
            p.terms[ANGA].add(Tag::Anga);
            p.record("3.1.78", "ruDAdiByaH Snam", before);
            true
        },
    },
```

In `crates/panini-prakriya/src/tinanta/terms.rs`, extend the header comment after the existing 2.4.72 paragraph:

```rust
// A SECOND caveat since rudhādi (gaṇa 7) landed: `terms[SHAP].text` is not
// always the vikaraṇa's own text. śnam is an infix (3.1.78 with 1.1.47), and
// the only way to seat an infix in a three-slot layout is to split the root
// across ANGA and SHAP: ANGA keeps the head through its last vowel, SHAP
// holds `na` followed by the root's tail. So for rudhādi, SHAP reads `nat`
// (kft), `nans` (hins) or `nad` (Kid) — never a bare `na`. A rule that reads
// SHAP expecting the vikaraṇa alone must guard on the gaṇa. This is a
// stronger form of a hazard the file already carries: 6.4.107 leaves
// `terms[SHAP].text == "n"` for svādi, which is why `shnu_asamyogapurva` and
// `sound_before_ending` both have ordering constraints written around them.
```

In `crates/panini-prakriya/src/tinanta/derivation_tests.rs`, add `"3.1.78"` to `expected` in `tinanta_rule_order_is_pinned`, between `"3.1.77"` and `"3.1.81"`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS. `kfRatti` is not asserted yet — Task 3 does that.

- [ ] **Step 5: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/vikarana.rs \
        crates/panini-prakriya/src/tinanta/terms.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 3.1.78 ruDAdiByaH Snam and the infix split"
```

---

### Task 3: The strong stem — `kfRatti`, `KinadE`, and 6.4.23 for `hinasti`

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/anga.rs` (add 6.4.23 + its guard tests)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: the `[head, na+tail, ending]` split from Task 2.
- Produces: correct strong (non-kṅit) cells for all three roots. Nothing later depends on new names.

- [ ] **Step 1: Write the failing tests**

Add to `crates/panini-prakriya/src/tinanta/derivation_tests.rs`:

```rust
#[test]
fn rudhadi_strong_cells() {
    // The strong stem is śnam with its `a` intact. kft needs no new rule at
    // all beyond 3.1.78 — 8.4.1 ṇatva already fires across the ANGA/SHAP
    // junction, exactly as it does for kryādi's vf + nA → vfRAti.
    assert_eq!(
        form_g("kft", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "kfRatti"
    );
    assert_eq!(
        form_g("kft", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "kfRatmi"
    );
    // √hiṃs needs 6.4.23: hins + śnam is hinans, and the root's own n comes
    // back out.
    assert_eq!(
        form_g("his", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "hinasti"
    );
    assert_eq!(
        form_g("his", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
        "hinasAni"
    );
    // The ātmanepada arm's strong cells keep śnam's `a` too.
    assert_eq!(
        form_g("Kid", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
        "KinadE"
    );
}
```

Add to the `mod tests` at the bottom of `crates/panini-prakriya/src/tinanta/anga.rs`:

```rust
    #[test]
    fn nalopa_removes_only_the_roots_own_nasal() {
        // 6.4.23 deletes the nasal the ROOT contributed, which after 3.1.78
        // sits in SHAP behind śnam's `na`. It must not touch śnam's own `n`:
        // hinans → hinas, never *hias or *hins.
        let d = dhatus().iter().find(|d| d.id == "his").unwrap();
        let p = sole(derive(d, Lakara::Lat, Pada::Parasmaipada, Purusha::Prathama, Vacana::Eka));
        let step = p.log.iter().find(|s| s.sutra == "6.4.23").unwrap();
        assert_eq!(step.before, "hinansti");
        assert_eq!(step.after, "hinasti");
    }

    #[test]
    fn nalopa_declines_where_the_tail_has_no_nasal() {
        // kft's tail is `t` and Kid's is `d`. A guard that fired on any
        // rudhādi root would produce *kfRatti from a mangled stem.
        for id in ["kft", "Kid"] {
            let d = dhatus().iter().find(|d| d.id == id).unwrap();
            let branches = derive(d, Lakara::Lat, d.pada, Purusha::Prathama, Vacana::Eka);
            for p in &branches {
                assert!(
                    !p.log.iter().any(|s| s.sutra == "6.4.23"),
                    "{id}: 6.4.23 fired with no root nasal"
                );
            }
        }
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: FAIL — `rudhadi_strong_cells` reports `hinansti` where `hinasti` is expected. `kfRatti` and `KinadE` may already pass; that is fine and is the point of asserting them here.

- [ ] **Step 3: Add 6.4.23**

In `crates/panini-prakriya/src/tinanta/anga.rs`, append to the `ANGA_RULES` array:

```rust
    // 6.4.23 śnān nalopaḥ: after śnam, the root's own nasal is elided.
    // hins + śnam gives hinans (3.1.78 seats `na` after the last vowel and
    // pushes the root's `ns` behind it); 6.4.23 takes the root's `n` out,
    // leaving hinas, whence hinasti.
    //
    // NARROW GUARD, by design. The nasal this rule deletes lives in SHAP,
    // not ANGA — an artefact of the infix representation (see
    // `super::vikarana`'s 3.1.78 and `super::terms`), and the reason the
    // rule reads SHAP at all. In 7a the only reachable witness is √hiṃs,
    // whose tail is `ns`; the guard therefore looks for a nasal immediately
    // after śnam's own `na` and does nothing otherwise. 7b widens it for
    // √bhañj, √und and √indh, whose tails are `fj`, `nd` and `nD`.
    //
    // Ordered before 6.4.111: the trace order is 6.4.23 then 6.4.111, and
    // reversing them elides śnam's `a` first, after which this rule can no
    // longer tell śnam's `n` from the root's.
    Rule {
        id: "6.4.23",
        name: "SnAnnalopaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Rudhadi) {
                return false;
            }
            let Some(shap) = p.terms.get(SHAP) else {
                return false;
            };
            // śnam's own `na`, then the root's tail. Only a tail whose first
            // sound is `n` is in scope in 7a.
            let rest: String = shap.text.chars().skip(2).collect();
            if !rest.starts_with('n') {
                return false;
            }
            let before = p.snapshot();
            let head: String = p.terms[SHAP].text.chars().take(2).collect();
            p.terms[SHAP].text = format!("{head}{}", &rest[1..]);
            p.record("6.4.23", "SnAnnalopaH", before);
            true
        },
    },
```

Add `"6.4.23"` to `tinanta_rule_order_is_pinned`'s `expected`, at the **end of the `ANGA` block** — i.e. immediately after `"7.4.21"`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/anga.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 6.4.23 SnAnnalopaH and the rudhādi strong stem"
```

---

### Task 4: 6.4.111 śnasor allopaḥ — the weak stem

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/adesha.rs`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: Tasks 2 and 3.
- Produces: the weak stem — `SHAP` with śnam's `a` gone (`kf`/`nt`, `hi`/`ns`, `Ki`/`nd`).

- [ ] **Step 1: Write the failing tests**

Add to `crates/panini-prakriya/src/tinanta/derivation_tests.rs`:

```rust
#[test]
fn rudhadi_weak_cells_lose_shnams_a() {
    // 6.4.111 fires before a kṅit sārvadhātuka and makes the strong/weak
    // split visible. These are the cells 8.4.65 does NOT fork, so they are
    // safe to assert with `form_g` at this stage.
    assert_eq!(
        form_g("kft", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "kfntanti"
    );
    assert_eq!(
        form_g("kft", Lakara::Lat, Purusha::Uttama, Vacana::Dvi),
        "kfntvaH"
    );
    assert_eq!(
        form_g("Kid", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "Kinde"
    );
    assert_eq!(
        form_g("Kid", Lakara::VidhiLin, Purusha::Prathama, Vacana::Eka),
        "KindIta"
    );
}
```

Add to the `mod tests` at the bottom of `crates/panini-prakriya/src/tinanta/adesha.rs`:

```rust
    #[test]
    fn shnasor_allopah_fires_only_before_a_knit_sarvadhatuka() {
        // Strong cell (tip is pit, not ṅit) keeps the `a`; weak cell (Ji is
        // apit → ṅit by 1.2.4) loses it. A guard that ignored ṅitva would
        // derive *kfnttanti and *kfRatvaH, both plausible-looking.
        let d = dhatus().iter().find(|d| d.id == "kft").unwrap();
        let strong = sole(derive(d, Lakara::Lat, Pada::Parasmaipada, Purusha::Prathama, Vacana::Eka));
        assert!(!strong.log.iter().any(|s| s.sutra == "6.4.111"));
        let weak = sole(derive(d, Lakara::Lat, Pada::Parasmaipada, Purusha::Prathama, Vacana::Bahu));
        let step = weak.log.iter().find(|s| s.sutra == "6.4.111").unwrap();
        assert_eq!(step.before, "kfnatanti");
        assert_eq!(step.after, "kfntanti");
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: FAIL — `kfnatanti` where `kfntanti` is expected.

- [ ] **Step 3: Add 6.4.111**

In `crates/panini-prakriya/src/tinanta/adesha.rs`, append to `ADESHA` — **after 6.4.101**, which is currently the last entry:

```rust
    // 6.4.111 śnasor allopaḥ: śnam's `a` is elided before a kṅit
    // sārvadhātuka. This is what produces rudhādi's weak stem, and the
    // strong/weak split the gaṇa is built around: kfnat + ti (strong,
    // tip is pit) against kfnt + anti (weak, Ji is ṅit by 1.2.4).
    //
    // The `a` deleted is always SHAP's second character — śnam's own — never
    // a vowel of the root, because 3.1.78 put everything of the root that
    // follows its last vowel behind śnam. That is the whole payoff of the
    // representation: this is a term-local edit rather than a positional
    // search inside a merged string.
    //
    // PLACEMENT, pinned by `hinDi`: 6.4.101 her dhiH runs FIRST and rewrites
    // the ending hi → Di, and only then does this rule strip the `a`
    // (hinas + Di → hins + Di → 8.2.25 → hinDi). Ordered last in this stage
    // for that reason; the sūtra number is not what decides it.
    //
    // The sūtra's `sa` — the `a` of √as — is out of scope: √as is not in the
    // root set. Guarded to the rudhādi arm accordingly, per the narrow-guard
    // discipline that landed 8.3.59 and 8.2.25.
    Rule {
        id: "6.4.111",
        name: "SnasorallopaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Rudhadi) {
                return false;
            }
            // 1.1.5 kṅiti ca: the sārvadhātuka that immediately follows must
            // be ṅit. For rudhādi SHAP is never empty, so the follower is
            // always the ending.
            let Some(ending) = p.terms.get(ENDING) else {
                return false;
            };
            if !ending.has(Tag::Ngit) {
                return false;
            }
            let shap: Vec<char> = p.terms[SHAP].text.chars().collect();
            if shap.get(1) != Some(&'a') {
                return false;
            }
            let before = p.snapshot();
            let mut s = shap;
            s.remove(1);
            p.terms[SHAP].text = s.into_iter().collect();
            p.record("6.4.111", "SnasorallopaH", before);
            true
        },
    },
```

Add `"6.4.111"` to `tinanta_rule_order_is_pinned`'s `expected`, immediately after `"6.4.101"`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/adesha.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 6.4.111 SnasorallopaH and the rudhādi weak stem"
```

---

### Task 5: 8.3.24 and 8.4.58 — the anusvāra round trip

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: Task 4's weak stem.
- Produces: `hiMstaH` (anusvāra retained) and `kfnttaH` (anusvāra resolved).

- [ ] **Step 1: Write the failing tests**

Add to `crates/panini-prakriya/src/tinanta/derivation_tests.rs`:

```rust
#[test]
fn rudhadi_anusvara_round_trip_is_conditional() {
    // 8.3.24 turns śnam's n into an anusvāra before a jhal; 8.4.58 turns it
    // back into the following sound's homorganic nasal — but only before a
    // YAY. √hiṃs is the witness that the return leg is conditional: the
    // anusvāra there is followed by the ROOT's own `s`, which is śal, not
    // yay, so it survives. Folding the two rules into one operation would
    // derive *hintaH.
    assert_eq!(
        form_g("his", Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
        "hiMstaH"
    );
    assert_eq!(
        form_g("his", Lakara::VidhiLin, Purusha::Prathama, Vacana::Bahu),
        "hiMsyuH"
    );
    assert_eq!(
        form_g("kft", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "kfntanti"
    );
}
```

Add to the `mod tests` at the bottom of `crates/panini-prakriya/src/tinanta/tripadi.rs`:

```rust
    #[test]
    fn parasavarna_requires_a_yay() {
        // Enumerated rather than golden-driven: a predicate that fired
        // unconditionally still produces plausible Sanskrit for two of the
        // three 7a roots, and only √hiṃs catches it.
        for (id, la, pu, va, has_anusvara) in [
            ("his", Lakara::Lat, Purusha::Prathama, Vacana::Dvi, true),
            ("kft", Lakara::Lat, Purusha::Prathama, Vacana::Bahu, false),
        ] {
            let d = dhatus().iter().find(|d| d.id == id).unwrap();
            let p = sole(derive(d, la, d.pada, pu, va));
            assert!(
                p.log.iter().any(|s| s.sutra == "8.3.24"),
                "{id}: 8.3.24 should always fire on a weak rudhādi cell"
            );
            assert_eq!(
                p.text().contains('M'),
                has_anusvara,
                "{id}: anusvāra retention"
            );
        }
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: FAIL — `hinstaH` where `hiMstaH` is expected.

- [ ] **Step 3: Add both rules**

In `crates/panini-prakriya/src/tinanta/tripadi.rs`, insert 8.3.24 **between the 8.3.15 and 8.3.59 entries**:

```rust
    // 8.3.24 naścāpadāntasya jhali: a non-pada-final `n` becomes an
    // anusvāra before a jhal. In this suite that `n` is always śnam's, and
    // the jhal is whatever the weak stem's tail or the ending supplies.
    //
    // Paired with 8.4.58 below, which usually turns the anusvāra straight
    // back into the same `n`. The pair is not a no-op, and √hiṃs is why:
    // hiMs + taH stops here, because 8.4.58 needs a YAY to follow and what
    // follows is the root's own `s`, which is śal. hiMstaH keeps its
    // anusvāra where kfntaH does not.
    //
    // NARROW GUARD: rudhādi only. The `n` of 7.1.3 jho'ntaH (aBavan,
    // kfntan) is pada-final and out of scope by the sūtra's own
    // `apadāntasya`; guarding on the gaṇa keeps this rule away from it
    // without needing a pada-boundary notion the engine does not have.
    Rule {
        id: "8.3.24",
        name: "naScApadAntasya Jali",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Rudhadi) {
                return false;
            }
            let w = word_chars(p);
            let Some(pos) = w.iter().position(|(_, _, c)| *c == 'n') else {
                return false;
            };
            // `apadāntasya`: something must follow, and it must be a jhal.
            let Some((_, _, next)) = w.get(pos + 1) else {
                return false;
            };
            if !is_jhal(*next) {
                return false;
            }
            let (term, idx, _) = w[pos];
            let before = p.snapshot();
            set_char(p, term, idx, 'M');
            p.record("8.3.24", "naScApadAntasya Jali", before);
            true
        },
    },
```

Insert 8.4.58 **immediately after the 8.4.55 entry** (see the caveat step below):

```rust
    // 8.4.58 anusvārasya yayi parasavarṇaḥ: an anusvāra becomes the
    // following sound's homorganic nasal, before a YAY only. This is the
    // return leg of the 8.3.24 pair — kfMt → kfnt — and it declines for
    // hiMs + taH, whose anusvāra is followed by śal `s`.
    //
    // ORDERED AFTER 8.4.1 / 8.4.2, and this is constrained — contrary to
    // what the spec assumed. `is_natva_target` in this file FOLDS 8.3.24 in
    // as a guard ("a non-padānta n before a jhal has ALREADY become an
    // anusvāra by the time the 8.4 rules run"), a simplification taken when
    // the engine had no anusvāra machinery. It does now, but only for
    // rudhādi: 8.3.24 above is gaṇa-guarded, so BAzante's `n` is still an
    // `n` when ṇatva runs and the fold is still load-bearing for every
    // other root. The fold therefore stays.
    //
    // Given that, this rule must run AFTER ṇatva. Placed before it, kfMt
    // would already be kfnt when 8.4.1 looks, and the weak stem would
    // decline only by falling through the stale fold rather than because
    // its nasal is genuinely an anusvāra. Placed here, kfntaH declines for
    // the right reason (`M` is not `n`) while kfRatti — whose `n` precedes
    // a vowel, so 8.3.24 never fired — still takes ṇatva.
    //
    // Retire the fold, and this constraint with it, when a slice widens
    // 8.3.24 past rudhādi.
    Rule {
        id: "8.4.58",
        name: "anusvArasya yayi parasavarRaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let w = word_chars(p);
            let Some(pos) = w.iter().position(|(_, _, c)| *c == 'M') else {
                return false;
            };
            let Some((_, _, next)) = w.get(pos + 1) else {
                return false;
            };
            let Some(nasal) = parasavarna_of(*next) else {
                return false;
            };
            let (term, idx, _) = w[pos];
            let before = p.snapshot();
            set_char(p, term, idx, nasal);
            p.record("8.4.58", "anusvArasya yayi parasavarRaH", before);
            true
        },
    },
```

`is_jhal` already exists in `crates/panini-prakriya/src/tinanta/sound.rs` and is already imported by `tripadi.rs` — **do not redefine it**. Add only the new classifier:

```rust
/// The homorganic nasal of a *yay* — every stop and semivowel. Returns
/// `None` for a sound outside yay (the sibilants and `h`), which is exactly
/// 8.4.58's declining case.
pub(crate) fn parasavarna_of(c: char) -> Option<char> {
    Some(match c {
        'k' | 'K' | 'g' | 'G' | 'N' => 'N',
        'c' | 'C' | 'j' | 'J' | 'Y' => 'Y',
        'w' | 'W' | 'q' | 'Q' | 'R' => 'R',
        't' | 'T' | 'd' | 'D' | 'n' => 'n',
        'p' | 'P' | 'b' | 'B' | 'm' => 'm',
        _ => return None,
    })
}
```

`tripadi.rs` already provides the two helpers these rules need — **use them rather than writing new ones**:

- `fn word_chars(p: &Prakriya) -> Vec<(usize, usize, char)>` — the assembled word as `(term index, char index, char)`, so a rule can reason over the whole pada and still write back into the right term. This is what lets 8.3.24 and 8.4.58 act on a sound that may sit in `ANGA`, `SHAP` or `ENDING` depending on the root.
- `fn set_char(p: &mut Prakriya, term: usize, idx: usize, to: char)` — replace one character, addressed as `word_chars` reports it.

Add `"8.3.24"` (between `"8.3.15"` and `"8.3.59"`) and `"8.4.58"` (immediately after `"8.4.2"`) to `tinanta_rule_order_is_pinned`'s `expected`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/tripadi.rs \
        crates/panini-prakriya/src/tinanta/sound.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 8.3.24 and 8.4.58, the rudhādi anusvāra round trip"
```

---

### Task 6: 8.4.53 jhalāṁ jaś jhaśi — restored

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: Task 4.
- Produces: `kfndDi`, the base branch of loṭ madhyama eka. Task 7 forks it.

- [ ] **Step 1: Write the failing test**

Add to `crates/panini-prakriya/src/tinanta/derivation_tests.rs`:

```rust
#[test]
fn rudhadi_lot_madhyama_eka_takes_jashtva() {
    // 6.4.101 her DiH makes the ending `Di`; the weak stem's final `t`
    // meets it and is voiced-aspirated to `d` by 8.4.53. This is the rule
    // commit 9b7adee deleted as unreachable once 8.2.25 dhi ca replaced
    // slice 5d's analysis — √kṛt is its first genuine witness.
    assert_eq!(
        form_g("kft", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
        "kfndDi"
    );
    // √hiṃs reaches the same cell through 8.2.25 instead: its stem-final
    // `s` is ELIDED before the Dh-initial ending, not voiced. Third witness
    // for that rule, on a stem shape it has not seen.
    assert_eq!(
        form_g("his", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
        "hinDi"
    );
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: FAIL — `kfntDi` where `kfndDi` is expected.

- [ ] **Step 3: Restore 8.4.53**

In `crates/panini-prakriya/src/tinanta/tripadi.rs`, insert **before the 8.4.55 entry**:

```rust
    // 8.4.53 jhalāṁ jaś jhaśi: a jhal becomes its jaś before a jhaś (a
    // voiced aspirate). kfnt + Di → kfnd + Di → kfndDi.
    //
    // RESTORED, not reverted. This rule was removed in 9b7adee as
    // unreachable: slice 5d had analysed the ās/vas junction as jaśtva and
    // shipped *AdDve, and 8.2.25 dhi ca — which ELIDES the `s` rather than
    // voicing it, and sits in 8.2, asiddha to all of 8.4 — bled it
    // completely. Nothing else in the suite reached it. rudhādi does: √kṛt's
    // stem-final `t` is not an `s`, so 8.2.25 declines and this junction is
    // genuinely jaśtva's.
    //
    // 8.2.25 still bleeds it for √hiṃs, which is why hinDi and kfndDi differ
    // in shape — the same cell of the same gaṇa, reached by two different
    // rules. Both are asserted in `super::derivation_tests`.
    Rule {
        id: "8.4.53",
        name: "JalAM jaS JaSi",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let w = word_chars(p);
            let Some(pos) = w.len().checked_sub(2) else {
                return false;
            };
            // The jhaś that conditions it: in this suite always the `D` of
            // 6.4.101's Di, at the last position.
            if w.last().map(|(_, _, c)| *c) != Some('i') || w[pos].2 != 'D' {
                return false;
            }
            let Some(target) = pos.checked_sub(1) else {
                return false;
            };
            let Some(jash) = jashtva_of(w[target].2) else {
                return false;
            };
            let (term, idx, _) = w[target];
            let before = p.snapshot();
            set_char(p, term, idx, jash);
            p.record("8.4.53", "JalAM jaS JaSi", before);
            true
        },
    },
```

Add to `crates/panini-prakriya/src/tinanta/sound.rs`, beside the existing `cartva_of`:

```rust
/// The *jaś* (voiced unaspirated) counterpart of a jhal, by place of
/// articulation. `None` for a sound with no jaś — the sibilants and `h`.
pub(crate) fn jashtva_of(c: char) -> Option<char> {
    Some(match c {
        'k' | 'K' | 'g' | 'G' => 'g',
        'c' | 'C' | 'j' | 'J' => 'j',
        'w' | 'W' | 'q' | 'Q' => 'q',
        't' | 'T' | 'd' | 'D' => 'd',
        'p' | 'P' | 'b' | 'B' => 'b',
        _ => return None,
    })
}
```

Add `"8.4.53"` to `tinanta_rule_order_is_pinned`'s `expected`, immediately before `"8.4.55"`.

- [ ] **Step 4: Run test to verify it passes**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/tripadi.rs \
        crates/panini-prakriya/src/tinanta/sound.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): restore 8.4.53 JalAM jaS JaSi, with kfndDi as its witness"
```

---

### Task 7: 8.4.65 jharo jhari savarṇe — the fifth optional rule

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (including `exactly_the_pinned_vikalpa_rules_are_optional`)

**Interfaces:**
- Consumes: Tasks 4–6.
- Produces: forked cells. From here on, affected cells must be asserted with `form_g_forked(.., branches)`, not `form_g`.

- [ ] **Step 1: Write the failing tests**

Add to `crates/panini-prakriya/src/tinanta/derivation_tests.rs`:

```rust
#[test]
fn rudhadi_savarna_elision_forks() {
    // The declined branch keeps both consonants and is index 0.
    assert_eq!(
        form_g_forked("kft", Lakara::Lat, Purusha::Prathama, Vacana::Dvi, 2),
        "kfnttaH"
    );
    assert_eq!(
        form_g_forked("Kid", Lakara::Lat, Purusha::Prathama, Vacana::Eka, 2),
        "Kintte"
    );
    // √hiṃs never forks here: `s` and `t` are not savarṇa.
    assert_eq!(
        form_g("his", Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
        "hiMstaH"
    );
}

#[test]
fn rudhadi_savarna_elision_derives_both_members() {
    let d = dhatus().iter().find(|d| d.id == "kft").unwrap();
    let forms: Vec<String> = derive(d, Lakara::Lat, Pada::Parasmaipada, Purusha::Prathama, Vacana::Dvi)
        .iter()
        .map(|p| p.text())
        .collect();
    assert_eq!(forms, vec!["kfnttaH".to_string(), "kfntaH".to_string()]);
}
```

Update `exactly_the_pinned_vikalpa_rules_are_optional`'s `expected` to:

```rust
    let expected = ["7.1.35", "3.4.111", "6.4.107", "8.4.65", "8.4.56"];
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: FAIL — `expected 2 branches, got 1`.

- [ ] **Step 3: Add 8.4.65**

In `crates/panini-prakriya/src/tinanta/tripadi.rs`, insert **between the 8.4.58 and 8.4.56 entries**:

```rust
    // 8.4.65 jharo jhari savarṇe (vikalpa): a jhar is optionally elided
    // before a savarṇa jhar. kfnttaH ~ kfntaH, kfndDi ~ kfnDi,
    // Kintte ~ Kinte.
    //
    // PLACEMENT AGAINST 8.4.56 IS LOAD-BEARING and unenforceable by the
    // compiler. Both rules are optional and both sit at the end of the
    // tripādī. This one must run FIRST: 8.4.56 vāvasāne forks a pada-final
    // `d` to `t` at pause, and if it ran first only one of this rule's two
    // branches would receive that fork — kfnttAt would never be derived.
    // The `kfntAt` trace pin in `super::derivation_tests` is the guard.
    //
    // It is also the rule that takes √kṛt's loṭ eka cells to five and six
    // forms, stacking with 7.1.35 and 8.4.56. That is the deepest fork the
    // engine produces, and the witness for ARCHITECTURE.md's branch-count
    // claim: k = 3 gives six branches, not eight, because 8.4.56 declines on
    // the vowel-final non-tātaṅ branch.
    Rule {
        id: "8.4.65",
        name: "Jaro Jari savarRe",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            let w = word_chars(p);
            let Some(pos) = (0..w.len().saturating_sub(1)).find(|i| {
                is_jhal(w[*i].2) && is_jhal(w[i + 1].2) && is_savarna(w[*i].2, w[i + 1].2)
            }) else {
                return false;
            };
            let (term, idx, _) = w[pos];
            let before = p.snapshot();
            remove_char(p, term, idx);
            p.record("8.4.65", "Jaro Jari savarRe", before);
            true
        },
    },
```

Add to `crates/panini-prakriya/src/tinanta/sound.rs`:

```rust
/// Are the two sounds *savarṇa* — same place and same manner of closure?
/// For 8.4.65's purposes that reduces to sharing a stop series: `t` and
/// `T` are savarṇa, `s` and `t` are not.
pub(crate) fn is_savarna(a: char, b: char) -> bool {
    fn series(c: char) -> Option<u8> {
        Some(match c {
            'k' | 'K' | 'g' | 'G' => 0,
            'c' | 'C' | 'j' | 'J' => 1,
            'w' | 'W' | 'q' | 'Q' => 2,
            't' | 'T' | 'd' | 'D' => 3,
            'p' | 'P' | 'b' | 'B' => 4,
            _ => return None,
        })
    }
    match (series(a), series(b)) {
        (Some(x), Some(y)) => x == y,
        _ => false,
    }
}
```

Add a `remove_char` helper to `tripadi.rs`, directly beside the existing `set_char` and in the same idiom:

```rust
/// Delete one character of one term, addressed as `word_chars` reports it.
/// Companion to `set_char`, for the rules that elide rather than substitute.
fn remove_char(p: &mut Prakriya, term: usize, idx: usize) {
    let mut s: Vec<char> = p.terms[term].text.chars().collect();
    s.remove(idx);
    p.terms[term].text = s.into_iter().collect();
}
```

Add `"8.4.65"` to `tinanta_rule_order_is_pinned`'s `expected`, immediately before `"8.4.56"` (so after `"8.4.58"`).

- [ ] **Step 4: Run tests to verify they pass**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/tripadi.rs \
        crates/panini-prakriya/src/tinanta/sound.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 8.4.65 Jaro Jari savarRe, the fifth optional rule"
```

---

### Task 8: 8.2.74, 8.2.73 and 8.2.75 — the ru alternation

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: Tasks 3–7.
- Produces: laṅ prathama/madhyama eka for all three roots. This is the last grammar task.

**Background the implementer needs.** `akfRat` + tip `t` gives a final conjunct, which the existing **8.2.23** *saṁyogāntasya lopaḥ* already reduces (its guard reads the word's last two characters and needs no change), and the existing **8.2.39** then voices the survivor to `akfRad`. `ahinas` reduces the same way — but **8.2.39 declines**, because its guard is deliberately narrow to a final `t`: a final `s` must become a visarga through 8.2.66 / 8.3.15, which is 8.2.39's apavāda. So √hiṃs needs 8.2.73 to supply its `d`, and without it the engine derives `*ahinaH` for laṅ prathama eka.

- [ ] **Step 1: Write the failing tests**

Add to `crates/panini-prakriya/src/tinanta/derivation_tests.rs`:

```rust
#[test]
fn rudhadi_lan_eka_cells() {
    // prathama eka: √kṛt's `d` comes from the existing 8.2.39, √hiṃs's from
    // the new 8.2.73 — 8.2.39 declines on a final `s` by design.
    assert_eq!(
        form_g_forked("kft", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2),
        "akfRad"
    );
    assert_eq!(
        form_g_forked("his", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2),
        "ahinad"
    );
    // madhyama eka forks three ways: the stop, its pausal variant, and ru.
    assert_eq!(
        form_g_forked("kft", Lakara::Lan, Purusha::Madhyama, Vacana::Eka, 3),
        "akfRad"
    );
    assert_eq!(
        form_g_forked("his", Lakara::Lan, Purusha::Madhyama, Vacana::Eka, 3),
        "ahinad"
    );
}

#[test]
fn ru_branch_derives_the_visarga_forms() {
    for (id, expected) in [("kft", "akfRaH"), ("his", "ahinaH")] {
        let d = dhatus().iter().find(|d| d.id == id).unwrap();
        let forms: Vec<String> =
            derive(d, Lakara::Lan, Pada::Parasmaipada, Purusha::Madhyama, Vacana::Eka)
                .iter()
                .map(|p| p.text())
                .collect();
        assert!(forms.contains(&expected.to_string()), "{id}: {forms:?}");
    }
}

#[test]
fn shnams_ru_fires_on_the_dhatus_own_final() {
    // 8.2.74 must see `ahinas`, not the `ahinad` 8.2.73 would already have
    // produced — which is why it is ordered ABOVE 8.2.73, against sūtra
    // order. Assert the order, not just the surface: numeric order still
    // derives ahinad on both branches, it simply never derives ahinaH.
    let d = dhatus().iter().find(|d| d.id == "his").unwrap();
    let p = derive(d, Lakara::Lan, Pada::Parasmaipada, Purusha::Madhyama, Vacana::Eka)
        .into_iter()
        .find(|p| p.text() == "ahinaH")
        .expect("ahinaH branch");
    let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
    let ru = ids.iter().position(|s| *s == "8.2.74").expect("8.2.74");
    assert!(
        !ids[..ru].contains(&"8.2.73"),
        "8.2.73 must not precede 8.2.74: {ids:?}"
    );
}
```

Update `exactly_the_pinned_vikalpa_rules_are_optional`'s `expected`. The test filters `rules()`, which yields **pipeline order**, and 8.2.74 / 8.2.75 sit in the tripādī before 8.4.65:

```rust
    let expected = ["7.1.35", "3.4.111", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56"];
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: FAIL — `ahinaH` where `ahinad` is expected for laṅ prathama eka.

- [ ] **Step 3: Add the three rules**

In `crates/panini-prakriya/src/tinanta/tripadi.rs`, insert all three **between the 8.2.39 and 8.3.15 entries**, in this order — 8.2.74 first:

```rust
    // 8.2.74 sipi dhāto rur vā (vikalpa): before sip, the dhātu's final
    // optionally becomes ru, which 8.3.15 then takes to a visarga.
    // ahinas + s → ahinaH.
    //
    // ORDERED ABOVE 8.2.73, against sūtra order, and this is load-bearing.
    // This rule replaces the DHĀTU'S OWN FINAL — the `s` — so it must see
    // `ahinas`. Below 8.2.73 it would find `ahinad` and have no `s` to act
    // on, and ahinaH would never be derived. Nothing in the code enforces
    // the order; `shnams_ru_fires_on_the_dhatus_own_final` in
    // `super::derivation_tests` is the guard, and it asserts the ORDER,
    // because the wrong one still produces a real word.
    Rule {
        id: "8.2.74",
        name: "sipi DAto rurvA",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Rudhadi) || !p.ctx.is_sip() {
                return false;
            }
            if !p.text().ends_with('s') {
                return false;
            }
            let before = p.snapshot();
            let Some(idx) = p.terms.iter().rposition(|t| !t.text.is_empty()) else {
                return false;
            };
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            s.push('r');
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.2.74", "sipi DAto rurvA", before);
            true
        },
    },
    // 8.2.73 tipy anasteḥ: before tip, a dhātu other than √as takes `d` for
    // its final. ahinas + t → ahinad.
    //
    // This is what fills the hole 8.2.39 leaves. 8.2.39 jhalāṁ jaśo'nte is
    // guarded narrowly to a final `t`, and correctly so — a final `s` is
    // 8.2.66 / 8.3.15's, not jaśtva's — so without this rule √hiṃs would
    // surface as *ahinaH in laṅ prathama eka. √kṛt needs nothing here: its
    // final really is a `t` and 8.2.39 handles it.
    //
    // DELIBERATE OVER-APPLICATION, recorded so it is not later read as a
    // bug: the sūtra says *tipi*, and this guard covers sip as well. The
    // reason is structural — 8.2.74 above is optional *against* the `d`,
    // so its declined branch has to be able to reach one. Same treatment
    // the previous slice gave 7.1.35's āśiṣi condition.
    Rule {
        id: "8.2.73",
        name: "tipyanasteH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Rudhadi) {
                return false;
            }
            if !(p.ctx.is_tip() || p.ctx.is_sip()) {
                return false;
            }
            if !p.text().ends_with('s') {
                return false;
            }
            let before = p.snapshot();
            let Some(idx) = p.terms.iter().rposition(|t| !t.text.is_empty()) else {
                return false;
            };
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            s.push('d');
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.2.73", "tipyanasteH", before);
            true
        },
    },
    // 8.2.75 daś ca (vikalpa): and a final `d` likewise becomes ru before
    // sip. akfRad + s → akfRaH. The counterpart of 8.2.74 for a stem whose
    // final is already a stop — √kṛt's, voiced by 8.2.39 just above.
    Rule {
        id: "8.2.75",
        name: "daSca",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Rudhadi) || !p.ctx.is_sip() {
                return false;
            }
            if !p.text().ends_with('d') {
                return false;
            }
            let before = p.snapshot();
            let Some(idx) = p.terms.iter().rposition(|t| !t.text.is_empty()) else {
                return false;
            };
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            s.push('r');
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.2.75", "daSca", before);
            true
        },
    },
```

`p.ctx.is_tip()` / `is_sip()` may not exist. If not, add them to `crates/panini-prakriya/src/context.rs` beside the existing accessors — tip is parasmaipada prathama eka, sip is parasmaipada madhyama eka:

```rust
    /// Is the ending tip — parasmaipada prathama eka?
    pub(crate) fn is_tip(&self) -> bool {
        matches!(self.pada, Pada::Parasmaipada)
            && matches!(self.purusha, Purusha::Prathama)
            && matches!(self.vacana, Vacana::Eka)
    }

    /// Is the ending sip — parasmaipada madhyama eka?
    pub(crate) fn is_sip(&self) -> bool {
        matches!(self.pada, Pada::Parasmaipada)
            && matches!(self.purusha, Purusha::Madhyama)
            && matches!(self.vacana, Vacana::Eka)
    }
```

Add `"8.2.74"`, `"8.2.73"`, `"8.2.75"` to `tinanta_rule_order_is_pinned`'s `expected` in that order, between `"8.2.39"` and `"8.3.15"`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/tripadi.rs \
        crates/panini-prakriya/src/context.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 8.2.74, 8.2.73 and 8.2.75, the rudhādi ru alternation"
```

---

### Task 9: The goldens — PARADIGM, ALTERNATES and the trace pins

**Files:**
- Modify: `crates/panini/tests/paradigm.rs`
- Modify: `crates/panini/tests/trace.rs`

**Interfaces:**
- Consumes: Tasks 1–8. No new code, only goldens.
- Produces: the slice's actual gate.

The exact tables are in the spec, `docs/superpowers/specs/2026-08-10-rudhadi-gana-7a-design.md`, under "What PARADIGM and ALTERNATES become". **Copy them verbatim** — they were generated from vidyut-prakriya, not transcribed by hand, with the single documented correction to √hiṃs laṅ madhyama eka.

- [ ] **Step 1: Add the twelve PARADIGM blocks**

Append the twelve `(root, lakara, [9 forms])` blocks from the spec to `PARADIGM` in `crates/panini/tests/paradigm.rs`, after the svādi rows. `PARADIGM` goes 1512 → 1620 cells and 42 → 45 roots.

- [ ] **Step 2: Add the 37 ALTERNATES rows**

Append the 37 rows from the spec to `ALTERNATES`. It goes 154 → 191 rows.

- [ ] **Step 3: Run the paradigm suite to verify it passes**

Run: `mise exec -- cargo test -p panini --test paradigm`
Expected: PASS, including `derivation_set_is_exactly_pinned` (each cell's derivation set is exactly `PARADIGM ∪ ALTERNATES`) and `every_alternate_names_the_vikalpa_rules_that_produced_it`.

If `derivation_set_is_exactly_pinned` fails, the engine and the goldens disagree — **fix the engine, not the golden**. The tables came from the reference implementation.

- [ ] **Step 4: Re-run the audit probe over 45 roots**

Rebuild the `examples/panini_full_audit.rs` probe the previous slice used: for every root × laṭ/laṅ/loṭ/vidhiliṅ × nine cells, derive the **complete set** of forms in both this engine and `vidyut-prakriya`, and diff the sets. The claim it sustains is the one the previous slice established — the engine's derivation set equals vidyut's in every cell, with no over-generation and no under-generation.

`vidyut-prakriya` is not a workspace dependency; clone it beside the repo and drive it from its own `examples/` directory, as the previous slice did. Note that `mise exec -- cargo` will not resolve there (no `mise.toml`); use `mise exec rust@1.97.1 -- cargo …`.

Expected: zero differences in all 1620 cells, with one documented exception — √hiṃs laṅ madhyama eka, where the *sets* match but vidyut treats `ahinaH` as its ruleless branch and this engine treats `ahinad` as index 0. The audit compares sets, so it passes; the difference lives only in `ALTERNATES`' key column, which is checked against this engine's own log.

- [ ] **Step 5: Add the five trace pins**

`trace.rs` already provides `fn trace_for(word: &str) -> Vec<String>`, which runs `Panini::check`, picks the analysis whose `form_slp1` equals the word exactly — so it addresses one fork branch, not the cell — and returns that branch's sūtra ids in order. There is no ordered/absent assertion helper; add this one beside `trace_for`:

```rust
/// Index of a sūtra in a trace, for the pins that must assert ORDER rather
/// than mere presence.
fn at(trace: &[String], sutra: &str) -> usize {
    trace
        .iter()
        .position(|s| s == sutra)
        .unwrap_or_else(|| panic!("{sutra} absent from {trace:?}"))
}
```

Then add the four pins. Each asserts order or absence, because in every case the wrong ordering still yields plausible Sanskrit:

```rust
#[test]
fn krnatti_trace_shows_the_infix_then_natva() {
    // 3.1.78 splits the root; 8.4.1 then fires across the ANGA/SHAP
    // junction it created, exactly as it does for kryādi's vf + nA.
    let t = trace_for("kfRatti");
    assert!(at(&t, "3.1.78") < at(&t, "8.4.1"), "got {t:?}");
}

#[test]
fn hindi_trace_shows_dhi_ca_bleeding_jashtva() {
    // 6.4.101 rewrites the ending BEFORE 6.4.111 strips śnam's `a`; 8.2.25
    // then ELIDES the stem-final `s` rather than voicing it, which is why
    // this cell reaches no 8.4.53 where its sibling kfndDi does.
    let t = trace_for("hinDi");
    assert!(at(&t, "6.4.101") < at(&t, "6.4.111"), "got {t:?}");
    assert!(at(&t, "6.4.111") < at(&t, "8.2.25"), "got {t:?}");
    assert!(!t.contains(&"8.4.53".to_string()), "got {t:?}");
}

#[test]
fn krntat_trace_shows_savarna_elision_above_pausal() {
    // Three optional rules on one branch. The reverse of 8.4.65 / 8.4.56
    // derives kfnttAt and kfntAd but never this form, so the surface alone
    // does not catch it.
    let t = trace_for("kfntAt");
    assert!(at(&t, "7.1.35") < at(&t, "8.4.65"), "got {t:?}");
    assert!(at(&t, "8.4.65") < at(&t, "8.4.56"), "got {t:?}");
}

#[test]
fn ahinah_trace_shows_ru_fires_on_the_dhatus_own_final() {
    // 8.2.74 must act on `ahinas`. Below 8.2.73 it would find `ahinad` and
    // this branch would not exist at all.
    let t = trace_for("ahinaH");
    assert!(t.contains(&"8.2.74".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.2.73".to_string()), "got {t:?}");
}

#[test]
fn kndhi_trace_shows_jashtva_where_dhi_ca_declines() {
    // The counterpart to hinDi: kft's stem-final `t` is not an `s`, so
    // 8.2.25 declines and the junction is genuinely 8.4.53's.
    let t = trace_for("kfndDi");
    assert!(t.contains(&"8.4.53".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.2.25".to_string()), "got {t:?}");
}
```

- [ ] **Step 6: Run the full gate and commit**

Run: `mise run test`
Expected: PASS, including `crates/panini/tests/roundtrip.rs`, which walks whatever `PARADIGM` holds and needs no edit.

```bash
git add crates/panini/tests/paradigm.rs crates/panini/tests/trace.rs
git commit -m "test: pin the rudhādi 7a paradigm, alternates and rule order"
```

---

### Task 10: Documentation and the mutation gate

**Files:**
- Modify: `AGENTS.md`
- Modify: `README.md`
- Modify: `docs/ARCHITECTURE.md`

**Interfaces:**
- Consumes: Tasks 1–9.
- Produces: nothing code-facing.

- [ ] **Step 1: Run the mutation gate**

Run: `mise run mutants`

This runs `cargo mutants --package panini-prakriya --test-workspace=true --timeout 300`. The explicit generous timeout is required: cargo-mutants calibrates from a baseline that runs only `panini-prakriya`'s unit tests (~2s), while each mutant runs the full `panini` golden suite (now ~100s at 1620 cells). Under a short cap a harmless mutant is recorded as a **timeout rather than a survivor**, and a zero-survivor run is vacuous.

Expected: zero survivors. A survivor means one of this slice's guard arms has no witness — **shrink the guard**, do not add a test to cover it.

- [ ] **Step 2: Update AGENTS.md**

In the golden-suite paragraph, record that:
- rudhādi (gaṇa 7, vikaraṇa śnam) is **partial**, not complete — the first gaṇa described that way. Say why: nine of its 25 roots are ubhayapadī and 1.3.72 is still deferred, so √rudh, √bhid, √chid and √yuj are absent and the gaṇa lacks its own eponymous root; 7b (√bhañj, √piṣ, √indh) closes what is reachable.
- śnam is the engine's first infix, and `terms[SHAP].text` for rudhādi is śnam plus the root's tail, not the vikaraṇa alone.
- 8.4.53 is restored, with `kfndDi` as its witness, after `9b7adee` removed it as unreachable.
- The vikalpa set is now **seven** rules, in pipeline order: 7.1.35, 3.4.111, 6.4.107, 8.2.74, 8.2.75, 8.4.65, 8.4.56.
- 8.4.65 must be ordered above 8.4.56, and 8.2.74 above 8.2.73 — both unenforceable, both pinned only by a trace test.

- [ ] **Step 3: Update README.md**

The scope paragraph becomes six complete gaṇas plus rudhādi partial. Update the cell counts: 1620 cells, of which 143 hold more than one form. Add the six-form case to the multi-form sentence — `kfndDi` / `kfnDi` / `kfnttAd` / `kfntAd` / `kfnttAt` / `kfntAt` is a single cell.

- [ ] **Step 4: Update docs/ARCHITECTURE.md**

- The stage table's rule ranges gain the ten new ids.
- The branch-count paragraph gains √kṛt's loṭ madhyama eka as its witness: k = 3, six branches against a 2³ bound of eight, because 8.4.56 declines on the vowel-final non-tātaṅ branch.
- Note the `SHAP` representation for rudhādi where the term layout is described.

- [ ] **Step 5: Run the full gate and commit**

Run: `mise run test && mise run lint && mise run fmt-check`

```bash
git add AGENTS.md README.md docs/ARCHITECTURE.md
git commit -m "docs: rudhādi 7a — śnam, the infix representation, and the seven vikalpa rules"
```

---

## Notes for the reviewer

Three decisions in this slice are invisible to any surface-form assertion and are pinned only by ordering tests. If a reviewer is going to push back anywhere, it should be here:

1. **The `SHAP` representation** (Task 2). Every rudhādi rule in 7a and 7b is written against it. The alternative — `ANGA` holding the whole infixed stem, `SHAP` empty — is recorded in the rule comment with the reason it was rejected.
2. **8.4.65 above 8.4.56** (Task 7) and **8.2.74 above 8.2.73** (Task 8). Both are against numeric order, both are deliberate, and the wrong order in each case produces real Sanskrit words — just not the ones the paradigm needs.
3. **8.2.73's guard covers sip as well as its sūtra's stated *tipi*** (Task 8). A stated over-application, in the shape the previous slice used for 7.1.35's *āśiṣi*.

**8.4.58 must sit after ṇatva** (Task 5), and the reason is a fold rather than a sūtra: `is_natva_target` in `tripadi.rs` folds 8.3.24 in as a guard, a simplification from when the engine had no anusvāra machinery, and that fold is still load-bearing for every non-rudhādi root because the real 8.3.24 this slice adds is gaṇa-guarded. Ordering 8.4.58 before ṇatva would make rudhādi's weak stems decline through the stale fold instead of because their nasal is genuinely an anusvāra. The fold stays until a slice widens 8.3.24 past rudhādi.
