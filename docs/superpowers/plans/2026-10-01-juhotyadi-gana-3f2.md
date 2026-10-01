# Juhotyādi gaṇa slice 3f2 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate √bhas (`03.0019 Basa~`, *baBasti*), parasmaipadī. The golden suite goes from 4572 to 4608 cells and juhotyādi to 25 of its 26 rows. The slice adds two rules, 6.4.100 *ghasibhasor hali ca* and 8.2.26 *jhalo jhali*. It widens three: 8.2.73 and 8.2.74 lose their rudhādi gaṇa test, and 8.4.55 *khari ca* reads the whole word. It also lands the Rust 1.98.1 → 1.99.0 toolchain bump.

**Architecture:** Ten tasks.
- **Task 1** creates the branch and commits the toolchain bump on its own.
- **Tasks 2–5** are the whole engine change. None of it fires on a curated cell until Task 6 lands the row, so each is gated on **its guard tests plus the 4572 priors staying byte-identical**.
- **Task 6** lands the row and its goldens, turning 36 cells green.
- **Task 7** adds the trace pins and the corpus-wide fires-only tests.
- **Tasks 8–10** are the audit with the prior-trace diff and doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.99.0 pinned via `mise` (after Task 1). Tasks: `mise run build | test | lint | fmt | fmt-check | mutants | audit`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`.

**Spec:** `docs/superpowers/specs/2026-10-01-juhotyadi-gana-3f2-design.md`

**Workspace:** Task 1 creates the branch `juhotyadi-3f2`, checked out at `/workspace/.worktrees/juhotyadi-3f2`. Every path below is relative to that directory unless it starts with `/`.

**Provenance.** Every code block in this plan ran green on a throwaway worktree at `92f20a4`. That run included the full suite, clippy `-D warnings`, the √bhas probe at 36/36, the audit at 4608 / 5699 with zero differences and its `entry` control failing on 36 cells, a byte-identical trace dump of all 5805 prior prakriyās, and a scoped mutation probe over the three new rule bodies with every mutant caught.

## Global Constraints

- **The 4572 pre-existing cells must stay byte-identical, traces included.** Regenerate no golden and change no pinned trace.
- **Two new rules, no new vikalpa.** The engine stays at eleven optional rules: `exactly_the_pinned_vikalpa_rules_are_optional` must not change. `tinanta_rule_order_is_pinned` grows from 124 to 126 ids, gaining exactly `6.4.100` (between `6.4.113` and `6.4.112`) and `8.2.26` (between `8.2.25` and `8.2.30`).
- **6.4.100 is keyed by row number, `03.0019`.** Never test `ANGA.text == "Bas"`.
- **8.2.73 and 8.2.74 keep every clause but the gaṇa test.** Their order stays 8.2.74 → 8.2.75 → 8.2.73, against sūtra order.
- **No unfalsifiable guard clauses.** A clause no cell and no guard test can make false is a mutation survivor. Witness it, delete it, or kill it with a direct guard test on a hand-built `Prakriya`. This is why 8.4.55's new scan has no `is_jhal` test: `cartva_of` already returns `None` off the stops.
- **Goldens are transcribed from this plan**: vidyut's output at `8da2f90b`, generated 2026-10-01 by `/tmp/vidyut-full/vidyut-prakriya/examples/juhotyadi_3f2_probe.rs` and matched 36/36 by the prototype. **ALTERNATES keys are this engine's** (its log ∩ `VIKALPA_RULES`, from the prototype's traces). Never invent one. **Do not edit a golden to match the engine.**
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit.
- `mise run test` takes a few seconds. Run it in the **foreground** with a timeout of 600000 ms. Never background it and end a turn.
- `mise run test -- -p X` does not scope. Scope unit tests with `mise exec -- cargo test -p <crate> <filter>`.
- Rule ids and SLP1 names, verbatim: `6.4.100 GasiBasorhali ca`, `8.2.26 Jalo Jali`, `8.2.73 tipyanasteH`, `8.2.74 sipi DAto rurvA`, `8.4.55 Kari ca`.

## Review Focus

These are inputs the spec implies that no golden cell isolates. Each has its test in the owning task.

1. **A vowel on either side of the `s`** (*baBasti*: `a`+`s`+`t`; *bapsati*: `B`+`s`+`a`). 8.2.26 must decline, and it reads the whole word. → Task 4, `jhalo_jhali_declines_unless_both_neighbours_are_jhals`.
2. **A pit follower** (*baBasti*, *baBastu*, *abaBat*). 6.4.100 must keep the upadhā. → Task 5, `ghasibhasor_declines_before_a_pit_off_its_row_and_on_bs`; Task 7, `baBastu_trace_keeps_the_upadha_before_a_pit`.
3. **`is_sip()` true but the ending still live** (vidhiliṅ madhyama eka, *bapsyAH*). `is_sip` is lakāra-blind, so only `dhatu_is_pada_final` keeps the now gaṇa-free 8.2.73 and 8.2.74 off it. → Task 2, `tipy_anasteh_and_sipi_dhato_decline_before_a_live_ending_and_off_s`.
4. **A word-internal jhal + khar pair on a prior row.** 8.4.55 now scans the whole word, and a new firing on a prior row would change no golden when the stop is already voiceless. → Task 3, `khari_ca_records_nothing_on_an_already_voiceless_jhal`; Task 7, `khari_ca_off_bhas_is_credited_exactly_as_before_3f2` (455 branches, measured on `main`).
5. **`check()` on the new forms.** *bapsati*, *babDaH*, *abaBat* and *abaBaH* must resolve to √bhas only. → Task 6, `bhas_analyses_its_reduplicated_forms`.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `mise.toml` | Task 1: `rust = 1.99.0` |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | Task 2: 8.2.73/8.2.74 guards and comments, `dhatu_is_pada_final`'s comment. Task 3: 8.4.55's scan and comment, 8.3.13's note on it. Task 4: 8.2.26. Unit tests for all three |
| `crates/panini-prakriya/src/tinanta/guna.rs` | Task 5: 6.4.100 and its unit tests |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Tasks 4, 5: the rule-order pin |
| `crates/panini-data/src/lib.rs` | Task 6: the row, counts, the gaṇa-row test |
| `crates/panini/tests/paradigm/data/juhotyadi.rs` | Task 6: 4 `PARADIGM` blocks, 8 `ALTERNATES` rows |
| `crates/panini/tests/paradigm/main.rs` | Task 6: totals, buckets, key census, doc comments, the `check()` test. Task 8: audit prose |
| `crates/panini/tests/trace/juhotyadi.rs` | Task 6: the 7.3.87 corpus test. Task 7: six pins, three corpus-wide tests |
| `crates/panini-prakriya/src/tinanta/{guna,anga}.rs` | Task 8: stale "3f2" comments |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, the 3a and 3f specs | Task 8 |
| `AGENTS.md`, maybe `mise.toml` | Task 9 |

---

## Task 1: Branch and the toolchain bump

**Files:**
- Modify: `mise.toml` (one line)

**Interfaces:** none.

The `/workspace` checkout on `main` holds the bump as a **staged, uncommitted** change. It moves to the branch as the slice's first commit.

- [ ] **Step 1: Create the worktree**

Follow superpowers:using-git-worktrees. From `/workspace`:

```bash
git -C /workspace diff --cached mise.toml   # must show exactly: rust 1.98.1 → 1.99.0
git -C /workspace worktree add .worktrees/juhotyadi-3f2 -b juhotyadi-3f2 main
cd /workspace/.worktrees/juhotyadi-3f2
```

- [ ] **Step 2: Apply the bump on the branch**

In `mise.toml`, change

```toml
rust = { version = "1.98.1", components = "clippy,rustfmt" }
```

to

```toml
rust = { version = "1.99.0", components = "clippy,rustfmt" }
```

Then `git diff mise.toml` must equal the `/workspace` staged diff from Step 1, line for line.

- [ ] **Step 3: Verify on 1.99.0**

```bash
mise trust && mise install
mise exec -- rustc --version     # rustc 1.99.0
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -15
```

Run in the foreground, timeout 600000 ms. Expected: all pass at 4572 cells. This was checked on `main` while the spec was written.

- [ ] **Step 4: Commit, then clear the staged copy on `main`**

```bash
git add mise.toml
git commit -m "chore: bump rust 1.98.1 → 1.99.0"
git -C /workspace restore --staged --worktree mise.toml
git -C /workspace status --short    # mise.toml no longer listed
```

Only run the `restore` after the commit succeeds. The change then lives on the branch and reaches `main` at merge.

---

## Task 2: 8.2.73 and 8.2.74 lose their gaṇa test

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs`: `dhatu_is_pada_final` (line 48), 8.2.74 (about line 560), 8.2.73 (about line 650), and `mod tests`

**Interfaces:**
- Consumes: the existing test helper `sip_prakriya(stem: &str, ending: &str, purusha: Purusha) -> Prakriya` in `tripadi.rs`'s `mod tests` (from slice 3f). It builds a laṅ parasmaipada eka prakriyā with `stem` at ANGA, an empty SHAP, `ending` at ENDING, and no gaṇa tag.
- Produces: nothing later tasks call.

- [ ] **Step 1: Write the failing tests**

At the end of `tripadi.rs`'s `mod tests`, after `nas_capadantasya_declines_off_its_two_ganas_and_before_a_non_jhal`, add:

```rust

    // --- 8.2.73 tipy anasteḥ / 8.2.74 sipi dhāto rur vā: gaṇa-free since 3f2

    #[test]
    fn tipy_anasteh_and_sipi_dhato_fire_on_any_ganas_pada_final_s() {
        // √bhas laṅ after 8.2.23 has eaten the ending: abaBas, with no
        // Rudhadi tag on the aṅga. 8.2.73 writes the `d` at tip; 8.2.74 the
        // ru at sip, which 8.3.15 finishes to abaBaH.
        let r73 = rules().find(|r| r.id == "8.2.73").unwrap();
        let mut p = sip_prakriya("abaBas", "", Purusha::Prathama);
        assert!((r73.apply)(&mut p));
        assert_eq!(p.text(), "abaBad");
        assert_eq!(p.log.last().unwrap().sutra, "8.2.73");
        let r74 = rules().find(|r| r.id == "8.2.74").unwrap();
        let mut p = sip_prakriya("abaBas", "", Purusha::Madhyama);
        assert!((r74.apply)(&mut p));
        assert_eq!(p.text(), "abaBar");
        assert_eq!(p.log.last().unwrap().sutra, "8.2.74");
    }

    #[test]
    fn tipy_anasteh_and_sipi_dhato_decline_before_a_live_ending_and_off_s() {
        let r73 = rules().find(|r| r.id == "8.2.73").unwrap();
        let r74 = rules().find(|r| r.id == "8.2.74").unwrap();
        for (stem, ending, why) in [
            // is_sip() is lakāra-blind, so a vidhiliṅ madhyama eka shape
            // passes it; only dhatu_is_pada_final keeps both rules off.
            ("bapsyA", "s", "live ending"),
            // a pada-final `d` is 8.2.75's, not theirs.
            ("abaBad", "", "d-final"),
        ] {
            for rule in [r73, r74] {
                let mut p = sip_prakriya(stem, ending, Purusha::Madhyama);
                assert!(!(rule.apply)(&mut p), "{} {why}", rule.id);
                assert!(p.log.is_empty(), "{} {why}", rule.id);
            }
        }
        // 8.2.74 is sip-only: at tip the `s` is left for 8.2.73.
        let mut p = sip_prakriya("abaBas", "", Purusha::Prathama);
        assert!(!(r74.apply)(&mut p));
        assert!(p.log.is_empty());
    }
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya tipy_anasteh 2>&1 | tail -20`
Expected: `tipy_anasteh_and_sipi_dhato_fire_on_any_ganas_pada_final_s` FAILS, because the untagged aṅga hits the Rudhadi test. The decline test passes; it exists for the mutation gate.

- [ ] **Step 3: Drop both gaṇa tests**

In 8.2.74 (`id: "8.2.74"`), change

```rust
            if !p.terms[ANGA].has(Tag::Rudhadi) || !p.ctx.is_sip() {
```

to

```rust
            if !p.ctx.is_sip() {
```

In 8.2.73 (`id: "8.2.73"`), delete the first three lines of `apply`:

```rust
            if !p.terms[ANGA].has(Tag::Rudhadi) {
                return false;
            }
```

so that `apply` begins with `if !dhatu_is_pada_final(p) {`.

- [ ] **Step 4: Rewrite the comments**

**8.2.74.** Replace its first paragraph

```rust
    // 8.2.74 sipi dhāto rur vā (vikalpa): before sip, the dhātu's final
    // optionally becomes ru, which 8.3.15 then takes to a visarga.
    // ahinas + s → ahinaH.
```

with

```rust
    // 8.2.74 sipi dhāto rur vā (vikalpa): before sip, the dhātu's final
    // optionally becomes ru, which 8.3.15 then takes to a visarga.
    // ahinas + s → ahinaH; abaBas + s → abaBaH (juhotyādi √bhas, slice 3f2).
    //
    // NO GAṆA TEST since slice 3f2, as 8.2.75 since 3f: the guard is sip,
    // the dhātu pada-final (8.2.23 has eaten the ending), and a final `s`.
    // `tipy_anasteh_and_sipi_dhato_are_credited_only_on_rudhadi_and_bhas` in
    // `panini`'s trace suite holds that no curated root outside rudhādi and
    // √bhas reaches it.
```

Leave its ORDERED ABOVE 8.2.73 paragraph unchanged.

**8.2.73.** Replace its first paragraph

```rust
    // 8.2.73 tipy anasteḥ: before tip, a dhātu other than √as takes `d` for
    // its final. ahinas + t → ahinad.
```

with

```rust
    // 8.2.73 tipy anasteḥ: before tip, a dhātu other than √as takes `d` for
    // its final. ahinas + t → ahinad; abaBas + t → abaBad (juhotyādi √bhas,
    // slice 3f2). No gaṇa test since 3f2, and no √as clause: √as is not
    // curated.
```

Immediately after the paragraph that ends ``// `super::derivation_tests` are the witnesses.`` (the RE-VERIFIED (7b Task 8) paragraph), insert:

```rust
    //
    // RE-VERIFIED AGAIN (slice 3f2), when this rule and 8.2.74 dropped their
    // Tag::Rudhadi test. √bhas is the first non-rudhādi root to reach them,
    // and it empties `ENDING` at exactly laṅ prathama/madhyama eka — the same
    // slot family. With the gaṇa test gone, the whole suite and every prior
    // cell's trace were unchanged (the 3f2 spec's corpus-wide trace diff).
```

Replace its final paragraph

```rust
    // This rule is OBLIGATORY (`vikalpa: false`), so the hazard is only
    // narrowed, not closed: if a future slice's root set ever makes
    // `ENDING` empty at some other slot (a different saṁyoga shape, or
    // another rule that luks the ending), this guard would over-fire there
    // silently — no test failure until a golden happens to catch it.
    // Re-verify this invariant again before widening the root set further.
```

with

```rust
    // This rule is OBLIGATORY (`vikalpa: false`), so the hazard is only
    // narrowed, not closed: if a future slice's root set ever makes
    // `ENDING` empty at some other slot (a different saṁyoga shape, or
    // another rule that luks the ending), this guard would over-fire there.
    // Since slice 3f2 that is no longer silent:
    // `tipy_anasteh_and_sipi_dhato_are_credited_only_on_rudhadi_and_bhas`
    // fails on the first new root this rule reaches, and the slice adding it
    // re-verifies the invariant there.
```

**`dhatu_is_pada_final`** (line 49). Replace

```rust
    // Defensive rather than a bare `p.terms[ENDING..]`: 8.2.73 and 8.2.74
    // guard on `Tag::Rudhadi` first, but 8.2.75 has had no gaṇa test since
    // slice 3f, so a hand-built prakriyā with any layout can reach here.
```

with

```rust
    // Defensive rather than a bare `p.terms[ENDING..]`: none of 8.2.73,
    // 8.2.74 and 8.2.75 has a gaṇa test (8.2.75 since slice 3f, the other two
    // since 3f2), so a hand-built prakriyā with any layout can reach here.
```

Leave the rest of that comment as it is. If `Tag` is now unused in `tripadi.rs`, the compiler will say so. It is not: 8.3.24 still reads `Tag::Rudhadi`.

- [ ] **Step 5: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS. `the_ru_alternation_stays_off_the_new_roots` and `no_8_2_73_step_appears_for_bhanj_or_pish` in `derivation_tests.rs` still pass: they decline on phonology (√bhañj's `aBanag`, √piṣ's `apinaq`), not on the gaṇa test.

- [ ] **Step 6: Run the full suite (priors byte-identical)**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms)
Expected: PASS at 4572 cells, with no golden or trace changed. The spec's probe ran exactly this change green. If a prior cell gains an 8.2.73 or 8.2.74 step, stop and report it.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/tripadi.rs
git commit -m "feat(tripadi): 8.2.73 tipy anasteḥ and 8.2.74 sipi dhāto rur vā drop their rudhādi gaṇa test"
```

---

## Task 3: 8.4.55 *khari ca* reads the whole word

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs`: 8.4.55 (about line 1415), 8.3.13's equivalent-mutant note (about line 1182), and `mod tests`

**Interfaces:**
- Consumes: `word_chars(p) -> Vec<(usize, usize, char)>` (term index, char index within the term, char), `set_char(p, term, idx, char)`, `cartva_of(char) -> Option<char>` (stops only), `is_khar(char) -> bool`. All are already imported in `tripadi.rs`.
- Produces: the test helper `bhas_prakriya(anga: &str, ending: &str) -> Prakriya` in `tripadi.rs`'s `mod tests`, used again in Task 4.

- [ ] **Step 1: Write the failing tests**

At the end of `tripadi.rs`'s `mod tests`, after Task 2's tests, add:

```rust

    // --- 8.4.55 khari ca: the whole word since slice 3f2 -----------------

    /// √bhas at the tripādī: abhyāsa `Ba`, aṅga `anga` (`Bs` once 6.4.100
    /// has run), an empty śap (ślu), and `ending`. No gaṇa tag: neither rule
    /// tested with it reads one.
    fn bhas_prakriya(anga: &str, ending: &str) -> Prakriya {
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new(anga), Term::new(""), Term::new(ending)]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "Ba".into();
        p
    }

    #[test]
    fn khari_ca_fires_inside_the_anga_and_at_the_junction() {
        let rule = rules().find(|r| r.id == "8.4.55").unwrap();
        // Word-internal: `Bs` + ati → `ps` (bapsati, before 8.4.54 takes the
        // abhyāsa's `B`). The junction holds `s` + `a`, which is not khar.
        let mut p = bhas_prakriya("Bs", "ati");
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "Bapsati");
        assert_eq!(p.log.last().unwrap().sutra, "8.4.55");
        // At the junction, as the rule always read: √indh's inD + se → intse.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("inD"), Term::new(""), Term::new("se")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "intse");
    }

    #[test]
    fn khari_ca_records_nothing_on_an_already_voiceless_jhal() {
        // A `t` before `s` (atsi) is already its own car, and nothing else in
        // the word is a jhal before a khar. The khar differs from the jhal on
        // purpose: the no-op guard must compare the substitute with the jhal,
        // not with the khar after it.
        let rule = rules().find(|r| r.id == "8.4.55").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("at"), Term::new(""), Term::new("si")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert!(p.log.is_empty());
    }
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya khari_ca 2>&1 | tail -20`
Expected: `khari_ca_fires_inside_the_anga_and_at_the_junction` FAILS on its first assertion: the junction-only rule sees `s` + `a` and declines. The no-op test passes.

- [ ] **Step 3: Replace the rule's comment and body**

Replace 8.4.55's whole leading comment, from `// 8.4.55 khari ca (cartva): a jhal immediately before the ending, meeting` down to and including `// to exactly what they were.` just above `Rule {`, with:

```rust
    // 8.4.55 khari ca (cartva): a jhal immediately before a khar becomes its
    // car (voiceless unaspirated), anywhere the two sit adjacent in the word.
    // √ad's d before ti/tas/si/tha → t: atti, attaH, atsi, atTa; √bhas's
    // aṅga-internal `Bs` → `ps` before a vowel or `y`: bapsati, bapsyAt
    // (slice 3f2). General, reused by every later gaṇa/subanta slice. No
    // longer the pipeline's last rule — 8.4.65 and 8.4.56 both follow it now
    // — but still ordered after every other 8.3/8.4 rule that precedes it.
    //
    // READS THE WHOLE WORD since slice 3f2, through `word_chars`, as 8.2.40
    // and 8.4.53 do; the sūtra has no positional condition. Until then it
    // read only the aṅga/ending junction: the last non-empty term's final
    // char before `ENDING` against `ENDING`'s own first sound. That reading
    // was itself a fix (7a Task 7: rudhādi's śnam-split puts the root's own
    // tail in SHAP, and an ANGA-only read gave Kindte for Kintte), and the
    // whole-word scan subsumes it — every junction pair is a pair of
    // adjacent chars in the flattened word. What the junction reading could
    // not see was a pair inside one term: √bhas's `Bs`, once 6.4.100 has
    // elided the `a` between them. Widening it changed no prior cell's trace
    // (the 3f2 spec's corpus-wide diff), and
    // `khari_ca_off_bhas_is_credited_exactly_as_before_3f2` in `panini`'s
    // trace suite holds the count.
    //
    // The scan takes the FIRST pair in the word. Each rule fires once per
    // branch, so a word with two such pairs would devoice only the first;
    // no curated cell has two.
```

Replace the rule's whole `apply` closure, from `apply: |p| {` through its closing `},` (the one just above `},` that ends the `Rule`), with:

```rust
        apply: |p| {
            let w = word_chars(p);
            for i in 1..w.len() {
                if !is_khar(w[i].2) {
                    continue;
                }
                // `cartva_of` is defined only on the stops, so it is the jhal
                // test as well: a sibilant is already its own car, and `h`
                // has none.
                let Some(sub) = cartva_of(w[i - 1].2) else {
                    continue;
                };
                // No-op guard: a stop that is already its own car records
                // nothing.
                if sub == w[i - 1].2 {
                    continue;
                }
                let (term, idx, _) = w[i - 1];
                let before = p.snapshot();
                set_char(p, term, idx, sub);
                p.record("8.4.55", "Kari ca", before);
                return true;
            }
            false
        },
```

Do **not** add an `is_jhal` test. It is subsumed by `cartva_of`, and a scoped mutation probe showed it survives as `replace - with /`.

`is_jhal` stays in use elsewhere in `tripadi.rs` (8.2.30, 8.2.31, 8.3.24). 8.4.56 already relies on `cartva_of`'s domain as its jhal test the same way; see its "no separate `is_jhal(last)` arm" note.

- [ ] **Step 4: Update 8.3.13's note on 8.4.55**

Inside 8.3.13's `apply` (the EQUIVALENT MUTANT note), replace

```rust
            // This engine has a dual representation — per-term text plus
            // the flattened `word_chars`/`p.text()` view — and two later
            // tripādī rules DO read term structure directly rather than
            // going through the flattened view: 8.4.55 (`Kari ca`) reads
            // `ENDING`'s own first char and walks `p.terms[..ENDING]` for
            // the last non-empty term before it; 8.4.56 (`vA'vasAne`, the
            // pipeline's last rule) does `p.terms.rposition(|t|
            // !t.text.is_empty())` and pops/pushes on that specific term.
            // Both were checked individually rather than assumed safe:
            // 8.4.55 is unaffected because `is_khar('Q')` is always false
            // (ḍh is a voiced aspirate, khar is voiceless) — the rule
            // declines to fire before it ever looks at which term holds
            // the surviving `Q`, whichever term that is. 8.4.56 is
            // unaffected because its `rposition` search for the last
```

with

```rust
            // This engine has a dual representation — per-term text plus
            // the flattened `word_chars`/`p.text()` view — and one later
            // tripādī rule DOES read term structure directly rather than
            // going through the flattened view: 8.4.56 (`vA'vasAne`, the
            // pipeline's last rule) does `p.terms.rposition(|t|
            // !t.text.is_empty())` and pops/pushes on that specific term.
            // (8.4.55 `Kari ca` did too, reading `ENDING`'s first char and
            // the last non-empty term before it, until slice 3f2 moved it to
            // the flattened view. It was unaffected either way: `is_khar('Q')`
            // is always false, since ḍh is a voiced aspirate and khar is
            // voiceless.) 8.4.56 was checked rather than assumed safe: it is
            // unaffected because its `rposition` search for the last
```

The next line (`// non-empty term is self-correcting: …`) continues unchanged. A few lines later the note says `re-running the` / `8.4.55/8.4.56 argument above`; change `8.4.55/8.4.56 argument` to `8.4.56 argument`.

- [ ] **Step 5: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS.

- [ ] **Step 6: Run the full suite (priors byte-identical)**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms)
Expected: PASS at 4572 cells. No prior cell has a word-internal jhal + khar pair that is not already car, so no trace changes. If a golden fails, stop and report the cell.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/tripadi.rs
git commit -m "feat(tripadi): 8.4.55 khari ca scans the whole word, not only the aṅga/ending junction"
```

---

## Task 4: 8.2.26 *jhalo jhali*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (a new rule right after 8.2.25; `mod tests`)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: Task 3's `bhas_prakriya(anga: &str, ending: &str) -> Prakriya`; `remove_char(p, term, idx)`, `is_jhal(char)`, `word_chars(p)`, all imported in `tripadi.rs`.
- Produces: the rule id `"8.2.26"`, which Task 7's trace tests read.

- [ ] **Step 1: Write the failing tests**

At the end of `tripadi.rs`'s `mod tests`, after Task 3's tests, add:

```rust

    // --- 8.2.26 jhalo jhali (slice 3f2) -----------------------------------

    #[test]
    fn jhalo_jhali_elides_an_s_between_two_jhals() {
        // √bhas after 6.4.100: the `s` of `Bs` before a `t`/`T`-initial
        // ending (babDaH, babDa, babDAt once 8.2.40 has run).
        let rule = rules().find(|r| r.id == "8.2.26").unwrap();
        for (ending, want) in [("tas", "BaBtas"), ("Ta", "BaBTa"), ("tAt", "BaBtAt")] {
            let mut p = bhas_prakriya("Bs", ending);
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.text(), want);
            assert_eq!(p.log.last().unwrap().sutra, "8.2.26");
        }
    }

    #[test]
    fn jhalo_jhali_declines_unless_both_neighbours_are_jhals() {
        let rule = rules().find(|r| r.id == "8.2.26").unwrap();
        for (anga, ending, why) in [
            // A vowel on the left: baBasti keeps its `s`.
            ("Bas", "ti", "a + s + t"),
            // A vowel on the right: bapsati keeps it, for 8.4.55.
            ("Bs", "ati", "B + s + a"),
            // A non-jhal on the right: bapsyAt keeps it too.
            ("Bs", "yAt", "B + s + y"),
        ] {
            let mut p = bhas_prakriya(anga, ending);
            assert!(!(rule.apply)(&mut p), "{why}");
            assert!(p.log.is_empty(), "{why}");
        }
    }
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya jhalo_jhali 2>&1 | tail -20`
Expected: both FAIL, panicking on `unwrap()`: there is no rule `8.2.26`.

- [ ] **Step 3: Add the rule**

In `tripadi.rs`'s `TRIPADI`, insert this immediately after 8.2.25's `Rule { … },` and before the comment `// 8.2.30 coH kuH:`:

```rust
    // 8.2.26 jhalo jhali: an `s` between two jhals is elided. Ba + Bs + tas
    // → Ba + B + tas, which 8.2.40 then takes to babDaH (√bhas, slice 3f2,
    // once 6.4.100 has elided the root's upadhā `a`).
    //
    // ORDERED AFTER 8.2.25 dhi ca, in sūtra order. On babDi the `s` stands
    // before `D`, and 8.2.25 — which needs no jhal on its left — takes it
    // first; this rule then finds no `s`. vidyut credits the same split.
    //
    // Reads the WHOLE WORD, as 8.2.40 and 8.4.53 do: the sūtra has no
    // positional condition. No curated cell outside √bhas presents jhal +
    // `s` + jhal anywhere (the 3f2 spec's corpus-wide trace diff), and
    // `ghasibhasor_and_jhalo_jhali_are_credited_only_on_bhas` in `panini`'s
    // trace suite holds that as a fact. An s-aorist would be the first
    // witness elsewhere, and this engine derives no luṅ.
    Rule {
        id: "8.2.26",
        name: "Jalo Jali",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let w = word_chars(p);
            for i in 1..w.len().saturating_sub(1) {
                if w[i].2 != 's' || !is_jhal(w[i - 1].2) || !is_jhal(w[i + 1].2) {
                    continue;
                }
                let (term, idx, _) = w[i];
                let before = p.snapshot();
                remove_char(p, term, idx);
                p.record("8.2.26", "Jalo Jali", before);
                return true;
            }
            false
        },
    },
```

- [ ] **Step 4: Pin the order**

In `derivation_tests.rs`'s `tinanta_rule_order_is_pinned`, change `"8.2.25", "8.2.30",` to `"8.2.25", "8.2.26", "8.2.30",`. `mise run fmt` rewraps the array.

- [ ] **Step 5: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS, including `tinanta_rule_order_is_pinned` and an unchanged `exactly_the_pinned_vikalpa_rules_are_optional`.

- [ ] **Step 6: Run the full suite (priors byte-identical)**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms)
Expected: PASS at 4572 cells. If a prior cell loses an `s`, stop and report it.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/tripadi.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(tripadi): 8.2.26 jhalo jhali — an s between two jhals is elided"
```

---

## Task 5: 6.4.100 *ghasibhasor hali ca*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (a new rule between 6.4.113 and 6.4.112; `mod tests`)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: `following_sarvadhatuka(p) -> Option<&Term>` (already imported in `guna.rs`), `Tag::Ngit`, `ANGA`. The existing test helper `abhyasta_prakriya(abhyasa, anga, ghu: bool, ending, ngit: bool) -> Prakriya` in `guna.rs`'s `mod tests`. `Context::dhatupatha` is `&'static str`.
- Produces: the rule id `"6.4.100"`, which Task 7 reads.

- [ ] **Step 1: Write the failing tests**

At the end of `guna.rs`'s `mod tests`, add:

```rust

    // --- 6.4.100 ghasibhasor hali ca (slice 3f2) ---------------------------

    /// √bhas at the guṇa stage: abhyāsa `Ba` (7.4.60 has run; 8.4.54's `b`
    /// comes later), aṅga `anga`, an empty śap, and `ending` (tagged Ngit
    /// when `ngit`), on the row `number`.
    fn bhas_prakriya(number: &'static str, anga: &str, ending: &str, ngit: bool) -> Prakriya {
        let mut p = abhyasta_prakriya("Ba", anga, false, ending, ngit);
        p.ctx.dhatupatha = number;
        p
    }

    #[test]
    fn ghasibhasor_elides_the_upadha_before_any_kngit() {
        // Hal-initial (tas: babDaH), vowel-initial (ati: bapsati — the *ca*'s
        // *aci*), and the ṅit yāsuṭ (yAt: bapsyAt).
        let rule = rules().find(|r| r.id == "6.4.100").unwrap();
        for ending in ["tas", "ati", "yAt"] {
            let mut p = bhas_prakriya("03.0019", "Bas", ending, true);
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ANGA].text, "Bs", "{ending}");
            assert_eq!(p.log.last().unwrap().sutra, "6.4.100");
        }
    }

    #[test]
    fn ghasibhasor_declines_before_a_pit_off_its_row_and_on_bs() {
        let rule = rules().find(|r| r.id == "6.4.100").unwrap();
        for (number, anga, ngit, why) in [
            ("03.0019", "Bas", false, "pit tip: baBasti"),
            ("03.0024", "Bas", true, "another juhotyādi row"),
            ("", "Bas", true, "a hand-built prakriyā names no row"),
            ("03.0019", "Bs", true, "the upadhā is already gone"),
        ] {
            let mut p = bhas_prakriya(number, anga, "ti", ngit);
            assert!(!(rule.apply)(&mut p), "{why}");
            assert_eq!(p.terms[ANGA].text, anga, "{why}");
            assert!(p.log.is_empty(), "{why}");
        }
    }
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya ghasibhasor 2>&1 | tail -20`
Expected: both FAIL, panicking on `unwrap()`: there is no rule `6.4.100`.

- [ ] **Step 3: Add the rule**

In `guna.rs`'s `GUNA`, insert this immediately before the comment `// 6.4.112 śnābhyastayor ātaḥ:`, which comes right after 6.4.113's `Rule { … },`:

```rust
    // 6.4.100 ghasibhasor hali ca: the upadhā `a` of √ghas and √bhas is
    // elided before a kṅit. The sūtra says *hali* (a hal-initial follower)
    // and its *ca* carries 6.4.98's *aci* (a vowel-initial one), so the
    // follower's first sound does not matter and no test on it is written.
    // Bas + tas → Bs + tas (babDaH); Bas + ati → Bs + ati (bapsati); Bas +
    // yAt → Bs + yAt (bapsyAt); Bas + tAt → Bs + tAt (babDAt — 7.1.35's
    // tātaṅ is ṅit). Before a pit ending it declines: baBasti, baBastu,
    // abaBat.
    //
    // KEYED BY ROW NUMBER, `03.0019 Basa~`, as 8.2.40 keys its *adhaḥ* on
    // √dhā's row, not by an equality test on the aṅga's text. √ghas, the
    // sūtra's other root, is not curated; a √ghas row extends this key. The
    // `as` suffix test is the operation itself — the upadhā `a` before the
    // final `s` — and it declines on an already-elided `Bs`.
    //
    // PLACEMENT: in this stage, after dvitva (ANGA holds the doubled base,
    // ABHYASA the copy) and before adesha's 6.4.101 her dhiḥ, which is
    // vidyut's order on babDi. 6.4.101's own guard asks only that the stem
    // end in a jhal, which `Bas` and `Bs` both do, so no form depends on
    // the order; `babDi_trace_elides_the_upadha_before_her_dhih` in
    // `panini`'s trace suite pins it.
    Rule {
        id: "6.4.100",
        name: "GasiBasorhali ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if p.ctx.dhatupatha != "03.0019" {
                return false;
            }
            let Some(follower) = following_sarvadhatuka(p) else {
                return false;
            };
            if !follower.has(Tag::Ngit) {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix("as") else {
                return false;
            };
            let elided = format!("{stem}s");
            let before = p.snapshot();
            p.terms[ANGA].text = elided;
            p.record("6.4.100", "GasiBasorhali ca", before);
            true
        },
    },
```

`elided` is computed before `p.snapshot()` because `stem` borrows `p.terms`.

- [ ] **Step 4: Pin the order**

In `tinanta_rule_order_is_pinned`, change `"6.4.113", "6.4.112",` to `"6.4.113", "6.4.100", "6.4.112",`. `mise run fmt` rewraps the array. The pinned array then holds 126 ids.

- [ ] **Step 5: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -20`
Expected: PASS.

- [ ] **Step 6: Run the full suite (priors byte-identical)**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms)
Expected: PASS at 4572 cells. The rule is keyed to a row that is not curated yet, so it fires nowhere.

- [ ] **Step 7: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/guna.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(guna): 6.4.100 ghasibhasor hali ca — √bhas's upadhā before a kṅit"
```

---

## Task 6: The row and its paradigm goldens

This task turns the 36 new cells green.

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini/tests/paradigm/data/juhotyadi.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`
- Modify: `crates/panini/tests/trace/juhotyadi.rs` (the 7.3.87 corpus test only)
- Modify: `AGENTS.md` (one test-name reference)

**Interfaces:**
- Consumes: Tasks 2–5.
- Produces: the `dhatus()` row `03.0019` (`Bas`, `Gana::Juhotyadi`, `PadaAssignment::Parasmaipada`). Task 7 looks it up by number.

- [ ] **Step 1: Add the `Dhatu` row**

`DHATUS` is ordered by number. Insert this row immediately before the `03.0020` (`ki`) row, whose comment begins `// 03.0020 ki\ jYAne.`. The artha is verbatim from `data/dhatupatha.tsv:1283`.

```rust
    Dhatu {
        // 03.0019 `Basa~` BartsanadIptyoH (√bhasa). Parasmaipadī by 1.3.78.
        // 6.4.100 ghasibhasor hali ca elides the upadhā `a` before every
        // kṅit: 8.4.55 then devoices `Bs` to `ps` before a vowel or `y`
        // (bapsati, bapsyAt), and 8.2.26 jhalo jhali elides the `s` before a
        // `t`/`T` (babDaH). Its laṅ eka cells reach 8.2.73 (abaBat) and
        // 8.2.74 (abaBaH), the first outside rudhādi. Slice 3f2.
        dhatupatha: "03.0019",
        code: "Bas",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "BartsanadIptyoH",
    },
```

Counts in the same file:
- `Dhatu`'s `pada` doc: `re-derives 100 of these 101` → `re-derives 101 of these 102`; `The test covers the 101 roots curated here` → `The test covers the 102 roots curated here`.
- `pada_from_upadesha`'s doc: `66 of the 101 curated roots` → `66 of the 102 curated roots`. `Basa~` carries no `\`.
- `curated_roots_have_expected_ganas_and_padas`: `assert_eq!(dhatus().len(), 101);` → `102`.

- [ ] **Step 2: Extend the gaṇa-row test**

Rename `juhotyadi_rows_are_the_twenty_four_curated_roots` to `juhotyadi_rows_are_the_twenty_five_curated_roots`. `grep -rn "juhotyadi_rows_are_the_twenty_four" crates AGENTS.md docs` finds the other references: a comment near `lib.rs:1994` and `AGENTS.md:813`. Update each. (`rudhadi_rows_are_the_twenty_five_curated_roots` is a different test; leave it alone.)

In its comment, replace

```rust
        // (03.0021–03.0024), parasmaipadī by 1.3.78. The gaṇa is PARTIAL at
        // 24 of its 26 dhātupāṭha rows; slice 3f2 (√bhas, √jan) closes it.
```

with

```rust
        // (03.0021–03.0024), parasmaipadī by 1.3.78. Slice 3f2 adds √bhas
        // (03.0019), parasmaipadī by 1.3.78. The gaṇa is PARTIAL at 25 of its
        // 26 dhātupāṭha rows; slice 3f3 (√jan) closes it.
```

In its expected vector, insert after the `("03.0018", "sf", PadaAssignment::Parasmaipada),` line:

```rust
                ("03.0019", "Bas", PadaAssignment::Parasmaipada),
```

- [ ] **Step 3: Run the data tests**

Run: `mise exec -- cargo test -p panini-data 2>&1 | tail -20`
Expected: PASS, including `curated_pada_agrees_with_upadesha_markers` and `dhatupatha_numbers_resolve_upstream`.

- [ ] **Step 4: Add the `PARADIGM` blocks**

**Append** these to the end of `PARADIGM` in `crates/panini/tests/paradigm/data/juhotyadi.rs`, just before its closing `];` (the first `];` in the file). Blocks are ordered by slice, not by number. Index 0 of each multi-form cell is the declined derivation. `mise run fmt` rewraps.

```rust
    ("03.0019", "laT", Pada::Parasmaipada, ["baBasti", "babDaH", "bapsati", "baBassi", "babDaH", "babDa", "baBasmi", "bapsvaH", "bapsmaH"]),
    ("03.0019", "laN", Pada::Parasmaipada, ["abaBad", "ababDAm", "abapsuH", "abaBad", "ababDam", "ababDa", "abaBasam", "abapsva", "abapsma"]),
    ("03.0019", "loT", Pada::Parasmaipada, ["baBastu", "babDAm", "bapsatu", "babDi", "babDam", "babDa", "baBasAni", "baBasAva", "baBasAma"]),
    ("03.0019", "viDiliN", Pada::Parasmaipada, ["bapsyAd", "bapsyAtAm", "bapsyuH", "bapsyAH", "bapsyAtam", "bapsyAta", "bapsyAm", "bapsyAva", "bapsyAma"]),
```

- [ ] **Step 5: Add the `ALTERNATES` rows**

**Append** these to the end of `ALTERNATES`, just before its closing `];` (the second `];` in the file). Each row is (root, lakāra, pada, 0-based cell index in the order P.E P.D P.B M.E M.D M.B U.E U.D U.B, form, key). The key is the vikalpa-listed ids on the form's branch, in log order.

```rust
    ("03.0019", "laN", Pada::Parasmaipada, 0, "abaBat", "8.4.56"),
    ("03.0019", "laN", Pada::Parasmaipada, 3, "abaBat", "8.4.56"),
    ("03.0019", "laN", Pada::Parasmaipada, 3, "abaBaH", "8.2.74"),
    ("03.0019", "loT", Pada::Parasmaipada, 0, "babDAd", "7.1.35"),
    ("03.0019", "loT", Pada::Parasmaipada, 0, "babDAt", "7.1.35+8.4.56"),
    ("03.0019", "loT", Pada::Parasmaipada, 3, "babDAd", "7.1.35"),
    ("03.0019", "loT", Pada::Parasmaipada, 3, "babDAt", "7.1.35+8.4.56"),
    ("03.0019", "viDiliN", Pada::Parasmaipada, 0, "bapsyAt", "8.4.56"),
```

That is 8 rows over 5 cells, so 36 + 8 = 44 forms, vidyut's count. Laṅ madhyama eka's declined branch is *abaBad*: 8.2.74 declined, and 8.2.73's deliberate over-application to sip wrote the `d`. Laṅ keys carry no 7.3.86, because `Bas` has an `a` upadhā and 7.3.86 does not fire.

- [ ] **Step 6: Update `paradigm/main.rs`'s numbers**

In `derivation_set_shape_matches_the_audited_numbers`:
- `assert_eq!(total_cells, 4572, "508 root×lakāra blocks × 9 cells each");` → `assert_eq!(total_cells, 4608, "512 root×lakāra blocks × 9 cells each");`
- `ones` 3780 → `3811`; `twos` 594 → `596`; `threes` 152 → `155`.
- The `threes` message ends `√viṣ's, the same way; and — new in slice 3f — √kit's, √tur's, √dhiṣ's and √dhan's, \` / `the same way"`. Replace its final `the same way"` with `the same way; and — new in slice 3f2 — √bhas's laṅ madhyama eka (8.2.74 beside 8.4.56) \` / `and its two loṭ tātaṅ cells"`. Keep the `\` line-continuation style.
- `fours`, `fives`, `sixes` and `sevens` are unchanged.
- `ALTERNATES.len()` 1083 → `1091`.
- `key_count("8.4.56")` 160 → `163`; `key_count("7.1.35")` 152 → `154`; `key_count("7.1.35+8.4.56")` 152 → `154`; `key_count("8.2.74")` 1 → `2`.

Check: 3811 + 596 + 155 + 18 + 10 + 17 + 1 = 4608 cells. 3811 + 1192 + 465 + 72 + 50 + 102 + 7 = 5699 forms = 4608 + 1091. The key census stays exhaustive: 1083 + 3 + 2 + 2 + 1 = 1091.

Doc comments in the same file:
- Above `every_alternate_names_the_vikalpa_rules_that_produced_it`: `otherwise 1083 bare strings` → `otherwise 1091 bare strings`.
- Above `derivation_set_shape_matches_the_audited_numbers`:
  - `4572 cells total (508 root×lakāra blocks × 9), of which 3780 hold exactly one form, 594 hold two, 152 hold three (` → `4608 cells total (512 root×lakāra blocks × 9), of which 3811 hold exactly one form, 596 hold two, 155 hold three (`.
  - In that three-form parenthesis, replace `and √kit's, √tur's, √dhiṣ's and √dhan's, new in slice 3f, each by` / `/// 7.1.35/8.4.56),` with `and √kit's, √tur's, √dhiṣ's and √dhan's, new in slice 3f, and √bhas's, new in slice 3f2, each by` / `/// 7.1.35/8.4.56, plus √bhas's laṅ madhyama eka, by 8.2.74/8.4.56),`.
  - `itself has 1083 rows, keyed 160 \`8.4.56\`, 152 \`7.1.35\`, 152 \`7.1.35+8.4.56\`,` → `itself has 1091 rows, keyed 163 \`8.4.56\`, 154 \`7.1.35\`, 154 \`7.1.35+8.4.56\`,`.
  - `8 \`8.2.75\`, 1 \`8.2.74\`,` → `8 \`8.2.75\`, 2 \`8.2.74\` (√hiṃs's ahinaH and, new in slice 3f2, √bhas's abaBaH),`.
  - After the slice-3f paragraph (ending `√dhan's in \`n\`, so neither forks there. Twenty-five new rows. The gaṇa is` / `/// PARTIAL at 24 of its 26 rows.`), add:

```rust
///
/// Slice 3f2 curates √bhas (`03.0019`), parasmaipadī by 1.3.78, bringing the
/// gaṇa to twenty-five of its twenty-six rows. It adds 6.4.100 *ghasibhasor
/// hali ca* and 8.2.26 *jhalo jhali*, drops the rudhādi gaṇa test from 8.2.73
/// and 8.2.74, and lets 8.4.55 read the whole word; no new vikalpa. √bhas
/// forks its vidhiliṅ and laṅ prathama eka on 8.4.56 (`bapsyAd`/`bapsyAt`,
/// `abaBad`/`abaBat`), its two loṭ tātaṅ cells three ways, and its laṅ
/// madhyama eka three ways on 8.2.74 and 8.4.56 (`abaBad`/`abaBat`/`abaBaH`)
/// — the second `8.2.74` key, after √hiṃs's. Eight new rows. The gaṇa is
/// PARTIAL at 25 of its 26 rows.
```

- [ ] **Step 7: Add the `check()` test**

At the end of `paradigm/main.rs`, add:

```rust

/// Slice 3f2's row through `check`. `Bas` is unique among the curated roots,
/// and none of these surfaces is a prior row's, so every analysis must name
/// √bhas and carry the rule that shaped the form.
#[test]
fn bhas_analyses_its_reduplicated_forms() {
    let engine = Panini::new();
    for (form, sutra) in [
        ("bapsati", "6.4.100"),
        ("babDaH", "8.2.26"),
        ("abaBat", "8.2.73"),
        ("abaBaH", "8.2.74"),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert!(!r.analyses.is_empty(), "{form}");
        for a in &r.analyses {
            assert_eq!(a.dhatu, "Bas", "{form}");
            assert!(
                a.trace.iter().any(|s| s.sutra == sutra),
                "{form}: {:?}",
                a.trace
            );
        }
    }
}
```

Before relying on the "no prior row" claim, run `grep -rn '"bapsati"\|"babDaH"\|"abaBat"\|"abaBaH"' crates/panini/tests/paradigm/data/`. It must find only the lines this task added.

- [ ] **Step 8: Extend the corpus-wide 7.3.87 test**

The engine credits 7.3.87 on √bhas's laṅ and loṭ uttama cells (*abaBasam*, *baBasAni*, …) as a no-op on the `a` upadhā, as it does for √dhan and as vidyut does. In `crates/panini/tests/trace/juhotyadi.rs`, replace

```rust
fn nabhyastasyaci_is_credited_only_on_the_3e_and_3f_rows() {
    // Corpus-wide: every branch of every curated root x lakāra x pada x cell
    // whose log carries 7.3.87 belongs to √ṇij, √vij, √viṣ (3e) or √kit,
    // √tur, √dhiṣ, √dhan (3f; a credited no-op on √dhan's a-upadhā). A new
    // rule that credits itself on prior rows' traces (forms unchanged) fails
    // here. 3f2 extends the allowed list with √bhas and √jan.
    const ALLOWED: [&str; 7] = [
        "03.0012", "03.0013", "03.0014", "03.0021", "03.0022", "03.0023", "03.0024",
    ];
```

with

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

`grep -rn nabhyastasyaci_is_credited_only crates` finds any comment that cites the old name; update each.

- [ ] **Step 9: Run the full suite**

Run: `mise run test 2>&1 | tail -40` (foreground, timeout 600000 ms)
Expected: PASS at 4608 cells, with all 4572 priors unchanged.

If a 3f2 cell fails, read it against the spec before touching anything, using superpowers:systematic-debugging. **Do not edit a golden to match the engine.** Likely causes by symptom:
- `baBastaH`, `baBasati` and the like (upadhā kept before a ṅit): 6.4.100 declined. Check its row key and the follower's `Tag::Ngit`.
- `babstaH` / `babsTa` (`s` kept): 8.2.26 declined.
- `babsati` (`B` voiced before `s`): 8.4.55 did not reach the aṅga-internal pair.
- `abaBaH` alone in laṅ eka: Task 2's guards are not dropped.
- A key mismatch in `every_alternate_names_the_vikalpa_rules_that_produced_it`: read the branch's actual log. Only correct a key if the form is right and the log really differs, and say so in the commit message.

- [ ] **Step 10: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-data/src/lib.rs crates/panini/tests/paradigm/data/juhotyadi.rs crates/panini/tests/paradigm/main.rs crates/panini/tests/trace/juhotyadi.rs AGENTS.md
git commit -m "feat(data): √bhas (03.0019) — juhotyādi at twenty-five of twenty-six rows

4572 → 4608 cells, 5655 → 5699 forms, 1083 → 1091 ALTERNATES, 101 → 102 roots."
```

---

## Task 7: The trace pins and the fires-only tests

**Files:**
- Modify: `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes: `crate::helpers::{at, cell_trace}`. `cell_trace(number, lakara, pada, purusha, vacana) -> (String, Vec<String>)` returns branch 0's (the declined derivation's) text and trace. `at(&[String], &str) -> usize` panics if the sūtra is absent. From slice 3f, already in this file: `branch_trace(number, lakara, purusha, vacana, form) -> Vec<String>` (parasmaipada, the branch deriving `form`), `ALL_CELLS`, and `credited(sutra) -> Vec<(&'static str, Gana)>` (one entry per non-blocked branch whose log carries `sutra`).

- [ ] **Step 1: Fix the stale 8.2.73 note in the *acikeH* pin**

In `acikeH_trace_is_jashtva_then_das_ca`, replace

```rust
    // `t`, then 8.2.75 takes the `d` to ru. 8.2.73 never runs on it: that
    // rule is still rudhādi-only and wants an `s`.
```

with

```rust
    // `t`, then 8.2.75 takes the `d` to ru. 8.2.73 never runs on it: that
    // rule wants an `s`.
```

- [ ] **Step 2: Add the pins and the corpus-wide tests**

At the end of the file, add:

```rust

#[test]
fn bapsati_trace_elides_the_upadha_then_devoices_inside_the_anga() {
    // Bas P laT P.B. 6.4.100 before the ṅit `ati` (7.1.4's), then 8.4.55
    // turns the aṅga's own `B` to `p` before its `s`: a word-internal pair the
    // junction-only reading never saw.
    let (text, t) = cell_trace(
        "03.0019",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "bapsati", "got {t:?}");
    assert!(at(&t, "7.1.4") < at(&t, "6.4.100"), "got {t:?}");
    assert!(at(&t, "6.4.100") < at(&t, "8.4.55"), "got {t:?}");
    assert!(!t.contains(&"8.2.26".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn babDaH_trace_is_upadha_lopa_then_jhalo_jhali_then_jhashas_tathoh() {
    // Bas P laT P.D. Bs + tas: 8.2.26 elides the `s` between `B` and `t`,
    // 8.2.40 voices the `t` after the jhaṣ, and 8.4.53 takes `B` to `b`.
    let (text, t) = cell_trace(
        "03.0019",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "babDaH", "got {t:?}");
    assert!(at(&t, "6.4.100") < at(&t, "8.2.26"), "got {t:?}");
    assert!(at(&t, "8.2.26") < at(&t, "8.2.40"), "got {t:?}");
    assert!(at(&t, "8.2.40") < at(&t, "8.4.53"), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn babDi_trace_elides_the_upadha_before_her_dhih() {
    // Bas P loT M.E. 6.4.100 runs before 6.4.101 (vidyut's order), and the
    // `s` before `D` is 8.2.25's, not 8.2.26's.
    let (text, t) = cell_trace(
        "03.0019",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "babDi", "got {t:?}");
    assert!(at(&t, "6.4.100") < at(&t, "6.4.101"), "got {t:?}");
    assert!(at(&t, "6.4.101") < at(&t, "8.2.25"), "got {t:?}");
    assert!(!t.contains(&"8.2.26".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn baBastu_trace_keeps_the_upadha_before_a_pit() {
    // Bas P loT P.E., declined branch: `tu` is pit, so 6.4.100 declines.
    let (text, t) = cell_trace(
        "03.0019",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "baBastu", "got {t:?}");
    assert!(!t.contains(&"6.4.100".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn abaBat_trace_is_tipy_anasteh_then_car() {
    // Bas P laN P.E. 8.2.23 eats the tip, 8.2.73 writes the `d`, 8.4.56
    // devoices it. The first 8.2.73 outside rudhādi.
    let t = branch_trace(
        "03.0019",
        Lakara::Lan,
        Purusha::Prathama,
        Vacana::Eka,
        "abaBat",
    );
    assert!(at(&t, "8.2.23") < at(&t, "8.2.73"), "got {t:?}");
    assert!(at(&t, "8.2.73") < at(&t, "8.4.56"), "got {t:?}");
    assert!(!t.contains(&"6.4.100".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn abaBaH_trace_is_sipi_dhatoh_without_tipy_anasteh() {
    // Bas P laN M.E., 8.2.74's branch: the `s` goes to ru and 8.3.15 to a
    // visarga, and 8.2.73 finds no `s` left.
    let t = branch_trace(
        "03.0019",
        Lakara::Lan,
        Purusha::Madhyama,
        Vacana::Eka,
        "abaBaH",
    );
    assert!(at(&t, "8.2.74") < at(&t, "8.3.15"), "got {t:?}");
    assert!(!t.contains(&"8.2.73".to_string()), "got {t:?}");
}

#[test]
fn ghasibhasor_and_jhalo_jhali_are_credited_only_on_bhas() {
    // 6.4.100 is keyed to √bhas's row; 8.2.26 reads the whole word with no
    // key at all. Goldens ignore traces, so this is what holds "no other
    // root reaches 8.2.26".
    for sutra in ["6.4.100", "8.2.26"] {
        let hits = credited(sutra);
        assert!(!hits.is_empty(), "√bhas no longer witnesses {sutra}");
        for (number, _) in &hits {
            assert_eq!(*number, "03.0019", "{sutra} credited on {number}");
        }
    }
}

#[test]
fn tipy_anasteh_and_sipi_dhato_are_credited_only_on_rudhadi_and_bhas() {
    // 8.2.73 and 8.2.74 have no gaṇa test since slice 3f2.
    for sutra in ["8.2.73", "8.2.74"] {
        let hits = credited(sutra);
        for (number, gana) in &hits {
            assert!(
                *gana == Gana::Rudhadi || *number == "03.0019",
                "{sutra} credited on {number}"
            );
        }
        assert!(
            hits.iter().any(|(n, _)| *n == "03.0019"),
            "√bhas no longer witnesses {sutra}"
        );
    }
}

#[test]
fn khari_ca_off_bhas_is_credited_exactly_as_before_3f2() {
    // 8.4.55 reads the whole word since slice 3f2. A trace step carries no
    // term boundaries, so "it fires only at the junction off √bhas" is held
    // as the count of crediting branches outside √bhas, measured on `main`
    // before the widening: a new word-internal firing on a prior row would
    // add one. Update it only when a slice adds rows that credit 8.4.55.
    let off_bhas = credited("8.4.55")
        .into_iter()
        .filter(|(number, _)| *number != "03.0019")
        .count();
    assert_eq!(off_bhas, 455);
}
```

`455` was measured on `main` at `92f20a4` from a dump of every curated cell's live branches, the same count `credited()` takes. Task 8 Step 2 re-measures it on `main` independently and diffs every prior trace.

- [ ] **Step 3: Run the suite**

Run: `mise run test 2>&1 | tail -20` (foreground, timeout 600000 ms)
Expected: PASS.
- If a pin fails on ORDER, report the engine's order; do not "fix" it toward vidyut's.
- If a pin fails on an id this plan assumed, read the actual trace. Correct the pin to the engine's real credit only when the form is right and the spec does not name that id, and say so in the commit message.
- If a fires-only test names a root outside the allowed set, or the 8.4.55 count is not 455, stop and report it: that is a widening over-firing on a prior row.

- [ ] **Step 4: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini/tests/trace/juhotyadi.rs
git commit -m "test(trace): 3f2 pins — bapsati, babDaH, babDi, baBastu, abaBat, abaBaH; 6.4.100, 8.2.26, 8.2.73/74 fire only on their rows, 8.4.55 off √bhas pinned at 455"
```

---

## Task 8: Audit, prior-trace diff, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (audit prose), `crates/panini-prakriya/src/tinanta/{guna,anga}.rs` (comments), `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md` and `docs/superpowers/specs/2026-10-01-juhotyadi-gana-3f-design.md` (one note each)

**Interfaces:**
- Consumes: the finished engine and goldens (Tasks 2–7). Produces no symbols.

- [ ] **Step 1: Update the audit harness's asserted totals**

In `tools/audit/panini_full_audit.rs`:
- `for each of the 101 curated roots` → `102`
- `101 roots, 4572 cells, 5655 forms` → `102 roots, 4608 cells, 5699 forms`
- `508 root×pada×lakāra` / `blocks × 9 cells, plus 1083` → `512 … plus 1091`
- `the full 4572-cell table` → `4608-cell`
- `assert_eq!(roots_seen.len(), 101, …)` → `102`
- `assert_eq!(n_cells, 4572, "cells: 508 root×pada×lakāra blocks × 9")` → `4608`, `"cells: 512 root×pada×lakāra blocks × 9"`
- `assert_eq!(n_forms, 5655, "forms: 4572 cells + 1083 ALTERNATES rows")` → `5699`, `"forms: 4608 cells + 1091 ALTERNATES rows"`

The both-pada clause (`twenty-six` / `twenty-five ubhayapadī`) does not change, because √bhas is parasmaipadī.

In `tools/audit/README.md`, `(101 roots, 4572 cells, 5655 forms)` → `(102 roots, 4608 cells, 5699 forms)`.

- [ ] **Step 2: Repoint vidyut's dev-deps at THIS worktree, run the audit and the prior-trace diff**

`/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes absolute dev-dep paths to `/workspace/crates`, the `main` checkout, which does not have this slice. Auditing without repointing checks the pre-slice engine and passes vacuously.

First dump every prior cell's traces from `main` **before** repointing. Save this throwaway example as `/tmp/vidyut-full/vidyut-prakriya/examples/trace_dump_3f2.rs` (it may already exist from the spec's probe; overwrite it):

```rust
//! THROWAWAY: dump every curated cell's derivation logs, for a main-vs-branch trace diff.
use panini::Panini;
use panini_data::{dhatus, Lakara, Purusha, Vacana};
fn main() {
    let panini = Panini::new();
    for d in dhatus() {
        for pada in d.pada.padas() {
            for l in [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin] {
                for pu in [Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama] {
                    for va in [Vacana::Eka, Vacana::Dvi, Vacana::Bahu] {
                        for p in panini.derive(d, l, *pada, pu, va).iter().filter(|p| !p.blocked) {
                            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
                            println!("{} {:?} {:?} {:?} {:?} {} : {}", d.dhatupatha, pada, l, pu, va, p.text(), ids.join(" "));
                        }
                    }
                }
            }
        }
    }
}
```

```bash
WT="$(git rev-parse --show-toplevel)"
DUMP="$(mktemp -d)"
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # must point at /workspace/crates
(cd /tmp/vidyut-full/vidyut-prakriya && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_3f2 > "$DUMP/main.txt")
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml   # must point at $WT/crates
(cd /tmp/vidyut-full/vidyut-prakriya && mise exec rust@1.99.0 -- cargo run -q --release --example trace_dump_3f2 > "$DUMP/branch.txt")
wc -l "$DUMP/main.txt"                                          # 5805 at 92f20a4
diff "$DUMP/main.txt" <(grep -v '^03.0019 ' "$DUMP/branch.txt") && echo PRIORS-IDENTICAL
grep -cE ' 8\.4\.55( |$)' "$DUMP/main.txt"                       # 455, the Task 7 pin
```

Expected: `PRIORS-IDENTICAL`, and `455`. If the diff shows any line, stop and report it: that is a prior cell's trace changed by the slice.

Then the audit. Copy the committed harness; never rewrite it.

```bash
cp tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -15)
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit 2>&1 | tail -8)
```

- Expected from the honest run: `AUDIT PASSED: 4608 cells, 5699 forms, zero differences.`
- Expected from the `entry` control: exit 1 with 36 √bhū cells.

`mise exec rust@1.99.0 --` runs cargo on this branch's toolchain from vidyut's directory. Do not use `mise -C`: it changes cargo's working directory too, and cargo then looks for the example in the wrong package.

If the honest run shows differences, stop and report, and edit nothing. After all runs, restore the dev-deps so later probes from `main` work:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"/workspace/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"/workspace/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
```

- [ ] **Step 3: Record the audit**

In `tools/audit/README.md`, immediately under `## Last recorded result`, add a new dated entry above the 3f one, using today's date:

```markdown
YYYY-MM-DD, juhotyādi 3f2 slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 4608
cells / 5699 forms / 102 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole juhotyādi 3f2 slice: √bhas (`03.0019`), with two
new rules — 6.4.100 *ghasibhasor hali ca* (*bapsati*) and 8.2.26 *jhalo jhali*
(*babDaH*) — 8.2.73 and 8.2.74 without their rudhādi gaṇa test (*abaBat*,
*abaBaH*), and 8.4.55 *khari ca* reading the whole word (`Bs` → `ps`). A
main-vs-branch dump of every prior cell's traces was byte-identical.

Totals: 102 = 101 + 1; 4608 = 4572 + 36 (4 root×pada×lakāra blocks × 9); 5699
= 5655 + 36 + 8 new `ALTERNATES` rows (1083 → 1091), measured via the
harness's corpus block, not assumed.
```

In `crates/panini/tests/paradigm/main.rs`'s audit-chain doc comment, the sentence ends `…and juhotyādi 3f's re-ran it at` / `the same commit over all 4572 cells / 5655 forms / 101 roots with zero` / `differences, its \`entry\` negative control verified failing (36 √bhū` / `cells).` Before its final `.`, add `, and juhotyādi 3f2's re-ran it at the same commit over all 4608 cells / 5699 forms / 102 roots with zero differences, its \`entry\` negative control verified failing (36 √bhū cells)`.

- [ ] **Step 4: README.md**

- `**partial** at 24 of its 26 dhātupāṭha rows` → `**partial** at 25 of its 26 dhātupāṭha rows`.
- After the sentence ending `*naś cāpadāntasya jhali* now admits juhotyādi beside rudhādi (*dadhaṁsi*,` / `*dadhaṁhi*).`, insert: `Slice 3f2 added √bhas (\`03.0019\`, *babhasti*), parasmaipadī, behind two new sūtras — 6.4.100 *ghasibhasor hali ca*, which elides its upadhā \`a\` before every kṅit (*bapsati*), and 8.2.26 *jhalo jhali* (*babdhaḥ*) — with 8.2.73 *tipy anasteḥ* and 8.2.74 *sipi dhāto rur vā* dropping their rudhādi gaṇa test (*ababhat*, *ababhaḥ*) and 8.4.55 *khari ca* reading the whole word.`
- `curated 101-root set` → `curated 102-root set`.
- Multi-form paragraph: `792 of the 4572 cells hold more than one form: 594` / `hold two, 152 hold three (` → `797 of the 4608 cells hold more than one form: 596 hold two, 155 hold three (`. After `and — new in slice 3f — √kit's, √tur's, √dhiṣ's` / `and √dhan's` add `, and — new in slice 3f2 — √bhas's`, and change the following `, each by 7.1.35/8.4.56),` to `, each by 7.1.35/8.4.56, √bhas's laṅ madhyama eka by 8.2.74/8.4.56),`. Check: 596 + 155 + 18 + 10 + 17 + 1 = 797.
- The both-pada and pada-ambiguity paragraphs do not change.

Reflow edited paragraphs to the file's ~80-column wrap.

- [ ] **Step 5: docs/ARCHITECTURE.md**

- The stage table (`grep -n "^| \`guna.rs\`\|^| \`tripadi.rs\`" docs/ARCHITECTURE.md`): in the `guna.rs` row, `6.4.113, 6.4.112` → `6.4.113, 6.4.100, 6.4.112`; in the `tripadi.rs` row, `8.2.25, 8.2.30` → `8.2.25, 8.2.26, 8.2.30`.
- The order-pin paragraph: `pins all 124 ids verbatim` → `pins all 126 ids verbatim`. Its last clause ends `juhotyādi 3e's two: 7.4.75 *nijāṁ trayāṇāṁ guṇaḥ ślau* and 7.3.87` / `*nābhyastasyāci piti sārvadhātuke* — 124 total).` Change that ending to `*nābhyastasyāci piti sārvadhātuke* — 124 total — then, after juhotyādi 3f added none, juhotyādi 3f2's two: 6.4.100 *ghasibhasor hali ca* and 8.2.26 *jhalo jhali* (8.2.73, 8.2.74 and 8.4.55 widened) — 126 total).`
- `**partial** at 24 of its 26 rows (… √kit, √tur, √dhiṣ, √dhan; slice 3f)` → `**partial** at 25 of its 26 rows (… √kit, √tur, √dhiṣ, √dhan; slice 3f; √bhas; slice 3f2)`.
- The vikalpa paragraph: `(8.2.75 gaṇa-free since juhotyādi 3f, which gave it √kit's *acikeḥ*)` → `(8.2.75 gaṇa-free since juhotyādi 3f, which gave it √kit's *acikeḥ*, and 8.2.74 since 3f2, which gave it √bhas's *ababhaḥ*)`.
- The 7.1.35 / 8.4.56 paragraph (`grep -n "forking 152 cells" docs/ARCHITECTURE.md`):
  - `forking 152 cells (loṭ` → `forking 154 cells (loṭ`
  - `across the 76 roots with a parasmaipada column` → `77`
  - in the juhotyādi list, `√ṛ, √kit, √tur, √dhiṣ and √dhan (all parasmaipada-only)` → `√ṛ, √kit, √tur, √dhiṣ, √dhan and √bhas (all parasmaipada-only)`
  - `76 + 25 = the 101 curated roots)` → `77 + 25 = the 102 curated roots)`
  - `forking 160 cells outright` → `163`
  - `those same 76 parasmaipada columns (138 of them` → `those same 77 parasmaipada columns (140 of them`. In that parenthesis, change the clause ending `` √kit's and √dhiṣ's laṅ forks key on 7.3.86 as well, see below) `` to `` √kit's and √dhiṣ's laṅ forks key on 7.3.86 as well, see below; 3f2's √bhas contributes both, its laṅ `abaBad` coming from 8.2.73) ``.
  - After `√vṛj, √pṛc, √tṛh, √chid, √chṛd and √bhuj — every rudhādi root with a` / `parasmaipada column; tanādi's own aṅga is vikaraṇa-vowel-final rather than` / `consonant-final at this junction, so none of its roots add to this` / `bucket)`, add `, and — new in slice 3f2 — √bhas's laṅ madhyama eka (\`abaBad ~ abaBat\`, the first outside rudhādi)`.

  Check: 140 + 22 + 1 = 163, and 152 + 2 = 154. If wording differs, keep its form and make the same substitution. What must be right: 1 new parasmaipada column, 4 new `7.1.35`-keyed rows (2 `7.1.35`, 2 `7.1.35+8.4.56`), and 3 new plain `8.4.56` rows.
- If any other sentence calls 8.2.73 or 8.2.74 rudhādi-only, or describes 8.4.55 as reading only the junction (`grep -n "8.2.73\|8.2.74\|8.4.55" docs/ARCHITECTURE.md`), make it true. The "junction witnesses: 8.4.55 cartva (√ad)" history at line ~263 stays.

- [ ] **Step 6: AGENTS.md**

- Rules of the codebase: `(\`crates/panini/tests/paradigm/\`, 4572 cells,` → `4608 cells`. In the juhotyādi clause, `and √viṣ, and now at 24 of its 26 after slice 3f curated √kit, √tur, √dhiṣ` / `and √dhan —` → `and √viṣ, at 24 after slice 3f curated √kit, √tur, √dhiṣ and √dhan, and now at 25 of its 26 after slice 3f2 curated √bhas —`. `(1083 rows in all, so 4572 + 1083 = 5655 forms total)` → `(1091 rows in all, so 4608 + 1091 = 5699 forms total)`. Bucket counts: `a second (594 cells), a third (152 cells)` → `(596 cells)`, `(155 cells)`.
- The 7b paragraph: `Since juhotyādi 3f, 8.2.75 carries no gaṇa test (√kit's` / `*acikeH*); 8.2.73 and 8.2.74 are still rudhādi-only.` → `Since juhotyādi 3f, 8.2.75 carries no gaṇa test (√kit's *acikeH*), and since 3f2 neither do 8.2.73 and 8.2.74 (√bhas's *abaBat*, *abaBaH*).`
- The audit record: after `and that by juhotyādi 3f's` / `(\`tools/audit/README.md\`'s 2026-10-01 entry, 4572 cells / 5655 forms / 101` / `roots)` add `, and that by juhotyādi 3f2's (\`tools/audit/README.md\`'s YYYY-MM-DD entry, 4608 cells / 5699 forms / 102 roots)`.
- The root-keyed-guard paragraph (`grep -n "keys on the dhātupāṭha number" AGENTS.md`): after the sentence `7.4.75 (\`03.0012\`–\`03.0014\`) does too: \`vij\` is also \`06.0009\` and \`07.0023\`.` insert ` 6.4.100 (\`03.0019\`, slice 3f2) does too, so a curated √ghas extends its key rather than sharing a text test.`
- The stale-comment ledger: `4572 goldens` / `would move today` → `4608 goldens`. After the 3f sentence's end (`…both lines measured by grep at this commit).`, before ` A third,`), add: ` Juhotyādi 3f2 touched neither comment either; the corpus stands at 4608 cells as of 3f2 (\`guna.rs:2386\`'s claim anchored at \`guna.rs:<G>\`, \`controller.rs:206\`'s at \`controller.rs:<C>\`; both lines measured by grep at this commit).` Measure `<G>` with `grep -n "1872 goldens move" crates/panini-prakriya/src/tinanta/guna.rs` and `<C>` with `grep -n "only 8 cells fire" crates/panini-prakriya/src/controller.rs`. Task 5 added lines to `guna.rs`, so `<G>` has moved. Measure; never compute.

- [ ] **Step 7: The stale engine comments, and the spec notes**

Each of these promised something of "3f2" that is now √jan's (3f3):
- `crates/panini-prakriya/src/tinanta/guna.rs`, 7.3.87's comment: `a-upadhā √dhan (slice 3f) and √bhas and √jan (slice 3f2), where 7.3.86` → `a-upadhā √dhan (slice 3f), √bhas (slice 3f2) and √jan (slice 3f3), where 7.3.86`. Reflow the paragraph if the line grows past the file's wrap.
- `crates/panini-prakriya/src/tinanta/anga.rs`, 6.4.71's comment: `can falsify (3e's and 3f's curated roots, and 3f2's, are all` / `consonant-initial)` → `can falsify (3e's, 3f's and 3f2's curated roots, and 3f3's √jan, are all consonant-initial)`. Reflow.
- `abhyasa.rs:744` (`` `Bas` is √bhas's (slice 3f2) ``) and `guna.rs`'s unit-test comment `The a-upadhā shape of √bhas (03.0019, slice 3f2)` stay: both are now records of what 3f2 did.

In `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md`, after the slice-3f note beneath the "Later slices" table (ending `became slice 3f2.`), add:

```markdown
> Slice 3f2 (`2026-10-01-juhotyadi-gana-3f2-design.md`) split again: 3f2 took
> √bhas alone, 36 cells, with 6.4.100 and 8.2.26 new and 8.2.73 / 8.2.74 /
> 8.4.55 widened. √jan, with 6.4.98, 6.4.42, the vikalpa 6.4.43 and the 8.4.40
> widening, became slice 3f3, which closes the gaṇa.
```

In `docs/superpowers/specs/2026-10-01-juhotyadi-gana-3f-design.md`, at the end of its "Later slices" bullet (ending `Re-probe before writing its spec.`), add a sentence: ` (3f2's re-probe split it: √bhas is 3f2, √jan is 3f3.)`

- [ ] **Step 8: Sweep for anything left stale**

```bash
grep -rn "4572\|5655\|\b1083\b\|101 roots\|101-root\|of these 101\|of the 101\|\b508\b\|792 of\|24 of its 26\|twenty-four of\|forking 152\|forking 160\|138 of them\|76 + 25\|\b594\b\|152 hold\|(152 cells)\|76 parasmaipada\|76 roots with\|124 ids\|twenty_four_curated\|3e_and_3f_rows" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.md --include=*.rs
grep -rn "3f2\|3f3" crates tools README.md AGENTS.md docs/ARCHITECTURE.md
grep -rn "rudhādi only\|rudhādi-only\|Tag::Rudhadi. first\|guard on .Tag::Rudhadi\|reads only the aṅga/ending\|junction-only\|ENDING.'s own first char" crates/panini-prakriya/src docs/ARCHITECTURE.md AGENTS.md
grep -rn "\bBas\b\|\bBs\b\|8\.2\.26\|6\.4\.100\|8\.2\.7[34]\|8\.4\.55" crates/panini-prakriya/src --include=*.rs | grep "//"
```

Expected residue:
- AGENTS.md's dated mutation record and anything naming 3f's numbers explicitly as 3f's. That is history; never rewrite it.
- "3f2" where it now records what 3f2 *did*, and "3f3" promises for √jan.
- 8.3.24's "rudhādi and juhotyādi" guard comments, which are still true.

Every "3f2" that promises something *will* happen is now false: rewrite it as done, or retarget it to 3f3 when it is √jan's. For every root-shape and rule-id hit, check the comment is still true with √bhas curated and 8.4.55 scanning the word.

- [ ] **Step 9: Run the full suite and commit**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms). Expected: PASS at 4608 cells.

```bash
mise run fmt && mise run lint
git add -A
git commit -m "docs: 3f2's counts, the audit record, and the sweep

4608 cells / 5699 forms / 102 roots / 1091 ALTERNATES across README,
ARCHITECTURE, AGENTS, paradigm/main.rs and tools/audit. Audit at zero
divergence against 8da2f90b; prior traces byte-identical to main."
```

---

## Task 9: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and the current-record paragraph); `mise.toml` only if the cap moves

Follow AGENTS.md's cargo-mutants protocol. Hazards from this repo's record:
- **Measure, never scale.**
- **Every invocation rotates `mutants.out`**, so always pass `-o`.
- **The mise shim fails in background shells.** Use the real binary: `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`.
- **`pgrep -f` matches its own shell.** Wait on `pgrep -x cargo-mutants`.
- **This slice edits `tripadi.rs` above both tripadi entries of the non-caught set**, so their line numbers move. Find them by `--list` and by their shape, never by the recorded line.
- **Rust is now 1.99.0.** The floor is measured on the new toolchain; do not carry 3f's numbers over.

- [ ] **Step 1: Measure the floor**

With nothing else running, run this twice: `time mise run test 2>&1 | tail -3` (foreground). Record both wall clocks and `cat /proc/loadavg`. The 4572-cell floor on 1.98.1 was 8.019s / 8.348s under a heavily loaded host.

- [ ] **Step 2: Locate and probe the two uncaught equivalents at `-j 4`**

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:[0-9]+:30: replace \+ with \*|tripadi.rs:[0-9]+:38: replace - with /|tripadi.rs:[0-9]+:23: replace -= with /="
```

`adesha.rs` is untouched, so its entry should still be `adesha.rs:589:30`. The `tripadi.rs` `- with /` entry (was `1223:38`, inside 8.3.13's `apply`, the `w[i - 1]` of `let Some(i) = (1..w.len()).find(…)`) and the permanent `-= with /=` ṇatva hang (was `1549:23`) move under Tasks 2–4. If `--list` shows several `- with /` candidates at column 38, pick the one inside 8.3.13. Write the new positions as `<T1>` (the equivalent) and `<T2>` (the hang), then run in the foreground with timeout 600000 ms:

```bash
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 80 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:589:30: replace \+ with \*" --re "tripadi.rs:<T1>:38: replace - with /" 2>&1 | tail -10
```

Both must be MISSED, not TIMEOUT. Read each test-phase duration from `$SCRATCH/mutants.out/outcomes.json`.

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/juhotyadi-3f2"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout 80 -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"
```

The last campaign took about 19 minutes under heavy host load. Run nothing CPU-heavy meanwhile. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

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

A scoped probe while this plan was written already ran every mutant in 6.4.100's, 8.2.26's and the new 8.4.55's bodies, and all were caught once 8.4.55's redundant `is_jhal` clause was dropped.

If not:
- Any **other timeout** is a suspect survivor. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant in 6.4.100, 8.2.26 or 8.4.55 means a Task 3–5 test does not separate it. Strengthen the named test, commit, and re-run only those mutants with `--re` and a fresh `-o`.

- [ ] **Step 5: Margins**

Inspect one record to see how `duration` is stored:

```bash
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["outcomes"][1])' "$OUT/mutants.out/outcomes.json"
```

Compute the two equivalents' test phases under campaign load, and the caught min/median/p90/max. The margin is 80 ÷ the longest equivalent phase.
- If the margin is ≥ 5, the cap stays 80.
- Otherwise the new cap is 6 × that phase, rounded up to the next 10 s. Change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together. A higher cap can only turn timeouts into outcomes, and the only timeout is the permanent hang, so the campaign's outcomes stand without a re-run.

- [ ] **Step 6: Record it in AGENTS.md**

- Rewrite `**The floor behind the 80s cap, measured at 4572 cells on` / `2026-10-01.**` with Step 1's and Step 2's numbers at 4608 cells on Rust 1.99.0, the new `tripadi.rs` positions, and the cap Step 5 chose.
- Replace the `**Current record (juhotyādi 3f, 2026-10-01).**` paragraph with `**Current record (juhotyādi 3f2, YYYY-MM-DD).**` in the same style. Include:
  - the flags, the `-o` path and the window;
  - **mutants / caught / unviable / missed / timeout** per package, summing to the total;
  - `missed.txt` and `timeout.txt` **named verbatim**, with the moved `tripadi.rs` positions;
  - the non-caught set diffed against 3f's (the clean result is identical up to the two moved `tripadi.rs` lines);
  - the campaign-load phases and margins;
  - that no 6.4.100, 8.2.26 or 8.4.55 mutant is missed, timed out or unviable-without-reason, with each rule's `file:line` range and mutant count;
  - that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`, with the durable copy at `$OUT/outcomes.durable.json`.

  End it with a pointer to the record it replaces: run `git rev-parse --short HEAD` before committing and write `The juhotyādi 3f record it replaces: \`git show <that hash>:AGENTS.md\`.`
- Update every other mention of the `tripadi.rs:1223:38` and `tripadi.rs:1549:23` positions in AGENTS.md (`grep -n "1223:38\|1549:23" AGENTS.md`) outside dated history.

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md mise.toml
git commit -m "chore: 3f2 mutation gate — floor and uncaught run re-measured at 4608 cells on rust 1.99.0

missed.txt holds only the two documented equivalents and timeout.txt only
the permanent ṇatva-scan entry; every 6.4.100, 8.2.26 and 8.4.55 mutant is caught."
```

---

## Task 10: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -20
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin juhotyadi-3f2
gh pr create --title "juhotyādi 3f2 — √bhas; rust 1.99.0" --body "$(cat <<'BODY'
Slice 3f2 curates √bhas (`03.0019`, *babhasti*), parasmaipadī. This takes the
golden suite from 4572 to 4608 cells and juhotyādi to twenty-five of its
twenty-six rows. It also bumps the pinned Rust toolchain from 1.98.1 to 1.99.0
(first commit).

**Two new rules:**
- 6.4.100 *ghasibhasor hali ca* (`guna.rs`, keyed to row `03.0019`) elides
  √bhas's upadhā `a` before every kṅit: *bapsati*, *babDaH*, *bapsyAt*.
- 8.2.26 *jhalo jhali* (`tripadi.rs`) elides an `s` between two jhals: *babDaH*.

**Three widenings in `tripadi.rs`:**
- 8.2.73 *tipy anasteḥ* and 8.2.74 *sipi dhāto rur vā* drop their rudhādi gaṇa
  test (*abaBat*, *abaBaH*). A corpus-wide test holds that only rudhādi and
  √bhas reach them.
- 8.4.55 *khari ca* scans the whole word instead of only the aṅga/ending
  junction (`Bs` → `ps`). Its credits off √bhas are pinned at 455, the count
  on `main`.

The 3f spec's 3f2 was split after a re-probe against vidyut (20/72 on HEAD):
√jan, with 6.4.98 / 6.4.42 / 6.4.43 and the 8.4.40 widening, is slice 3f3. The
audit shows zero divergence against `8da2f90b`, a main-vs-branch dump of every
prior cell's traces is byte-identical, and the mutation gate is clean.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Follow the standing instruction:
1. Watch `gh pr checks <N>` until nothing is pending. This repo has no required checks, so `--auto` merges immediately and must not be used. Once the checks are green, run `gh pr merge <N> --merge`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`: `git worktree remove .worktrees/juhotyadi-3f2`, then delete the local and remote branch, and run `git pull` on `main`. `git -C /workspace show HEAD:mise.toml | head -2` must now show `1.99.0`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| Rust 1.99.0 bump, first commit, gates pass on it | 1 |
| 8.2.73 / 8.2.74: gaṇa test dropped, order unchanged, hazard re-verified, comments | 2 |
| 8.2.73 / 8.2.74 guard tests: fire untagged; decline on live ending, `d`, tip (8.2.74) | 2 |
| 8.4.55 whole-word scan, comment, 8.3.13's note | 3 |
| 8.4.55 tests: internal + junction; no-op on already-car | 3 |
| 8.2.26 after 8.2.25, whole word; tests; order pin | 4 |
| 6.4.100 in `guna.rs` before 6.4.112, row-keyed, `as` suffix; tests; order pin | 5 |
| Row, counts, gaṇa-row test, 4 blocks, 8 `ALTERNATES`, buckets, keys | 6 |
| `check()` spot test (*bapsati*, *babDaH*, *abaBat*, *abaBaH*) | 6 |
| 7.3.87 corpus test extended to √bhas | 6 |
| Trace pins *bapsati*, *babDaH*, *babDi*, *baBastu*, *abaBat*, *abaBaH* | 7 |
| Fires-only: 6.4.100, 8.2.26 on √bhas; 8.2.73/74 on rudhādi + √bhas; 8.4.55 off-√bhas count | 7 |
| Prior traces byte-identical, main ↔ branch | 2–5 (suite at 4572), 8 Step 2 (dump diff) |
| Audit with repoint and negative control; README/ARCHITECTURE/AGENTS; spec notes; "3f2" sweep | 8 |
| Floor on 1.99.0, uncaught probe, campaign, verbatim non-caught record | 9 |

**Spec deviation, recorded.** The spec lists a fires-only test for "8.4.55 credited at a non-junction position only on `03.0019`". A `RuleStep` holds only flat `before`/`after` strings, so the `panini` crate cannot tell a junction firing from an internal one. Task 7 instead pins the off-√bhas credit count (455, measured on `main`). Any new firing on a prior row raises it, and a moved firing would change that row's golden. Task 8 Step 2's dump diff is the one-off confirmation. The spec also names a guard-test case "8.2.26 declines with a vowel on either side"; Task 4 adds the non-jhal-consonant case (`y`) as well.

**Type consistency.** Rule ids and names: `"6.4.100"` / `"GasiBasorhali ca"`, `"8.2.26"` / `"Jalo Jali"`, `"8.2.73"` / `"tipyanasteH"`, `"8.2.74"` / `"sipi DAto rurvA"`, `"8.4.55"` / `"Kari ca"`, everywhere. `tripadi.rs`'s `bhas_prakriya(&str, &str)` is defined in Task 3 and used in Tasks 3–4. `guna.rs`'s `bhas_prakriya(&'static str, &str, &str, bool)` is defined in Task 5 and used only there; the two live in different test modules, so the shared name does not clash. `branch_trace`, `credited` and `ALL_CELLS` already exist in `trace/juhotyadi.rs` from 3f.

**Known soft spots.**
- **The 455 pin.** It was measured from the throwaway dump at `92f20a4`, counting live branches as `credited()` does. Task 8 Step 2 re-measures it on `main` directly.
- **`check()` field names.** Task 6 uses `a.dhatu`, `a.trace` and `s.sutra`, as the 3f test beside it does.
- **The `threes` message and the `\` continuations.** Edit without breaking them.
- **`tripadi.rs` line drift.** Task 9 finds the two moved entries by `--list` and by shape.
- **The `/workspace` staged `mise.toml`.** Task 1 Step 4 clears it only after the branch commit exists.
