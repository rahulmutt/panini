# Juhotyādi gaṇa slice 3d2 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate √ṛ (`03.0017 f\`, *iyarti* / *EyaH*), juhotyādi's one vowel-initial ṛ-root. This takes the golden suite from 4176 to 4212 cells and the gaṇa to 17 of its 26 rows. It adds one sūtra (6.4.78 *abhyāsasyāsavarṇe*), widens two (7.4.60, 7.4.77), and closes 3a's augment-guard checkpoint.

**Architecture:** Seven tasks.
- **Task 1** is the whole engine change, all in the abhyāsa stage (`abhyasa.rs`). It fires on no curated root yet, so it is gated on **guard tests plus the 4176 priors staying byte-identical**.
- **Task 2** lands the row and its goldens, turning 36 cells green.
- **Task 3** closes the 6.4.71/6.4.72 checkpoint (comments and a live-row test).
- **Task 4** adds the trace pins.
- **Tasks 5–7** are the audit and doc sweep, the mutation gate, and the branch finish.

**Tech Stack:** Rust 1.98.1 pinned via `mise`. Tasks: `mise run build | test | lint | fmt | fmt-check | mutants | audit`. The cross-implementation reference is vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`.

**Spec:** `docs/superpowers/specs/2026-09-30-juhotyadi-gana-3d2-design.md`

**Workspace:** the branch `juhotyadi-3d2` already exists (it holds the spec and this plan) and is checked out in `/workspace`. Before Task 1, move it into a worktree:

```bash
cd /workspace && git switch main && git worktree add .worktrees/juhotyadi-3d2 juhotyadi-3d2
```

Every path below is relative to `/workspace/.worktrees/juhotyadi-3d2`.

## Global Constraints

- **The 4176 pre-existing cells must stay byte-identical, traces included.** Regenerate no golden and change no pinned trace.
- **No unfalsifiable guard clauses.** A clause no cell and no guard test can make false is a mutation survivor: witness it, delete it, or kill it with a direct guard test on a hand-built `Prakriya`.
- **Rules that name roots key on `p.ctx.dhatupatha`**, with a comment naming the upadeśa (7.4.77 gains `03.0017`). **Rules that name sounds guard on text** (6.4.78).
- **6.4.71 / 6.4.72 keep reading `ANGA`.** Their code does not change; only comments and a test.
- **Engine order differs from vidyut's on purpose.** This engine copies the bare root before guṇa, so 7.4.66 fires on every √ṛ cell and 6.4.78 runs in the abhyāsa stage before 6.4.72 and 7.3.84. Pins hold THIS order. Never "fix" a pin toward vidyut's trace.
- **Goldens are transcribed from this plan** (vidyut's output at `8da2f90b`, generated 2026-09-30 by `/tmp/vidyut-full/vidyut-prakriya/examples/juhotyadi_3d2_probe.rs`), never invented. **Do not edit a golden to match the engine.**
- **No new vikalpa.** The engine stays at eleven optional rules; `exactly_the_pinned_vikalpa_rules_are_optional` must not change.
- Commit after every task. Run `mise run fmt` and `mise run lint` before each commit.
- `mise run test` takes a few seconds. Run it in the **foreground** with a timeout of 600000 ms; never background it and end a turn.
- `mise run test -- -p X` does not scope. Scope unit tests with `mise exec -- cargo test -p <crate> <filter>`.
- The new rule's id and SLP1 name are copied verbatim from vidyut's `sutrapatha.tsv`: `6.4.78 aByAsasyAsavarRe`. Existing rules keep their `name` strings (`halAdiH SezaH`, `artipipartyoSca`).

## Review Focus

Inputs the spec implies that no golden cell exercises. Each has its test in the owning task.

1. **A vowel-initial root of another gaṇa.** adādi's √ad (laṅ *Adat*) has an empty `ABHYASA` slot and a vowel-initial `ANGA`; 6.4.78 must not fire on the empty slot. → Task 1, the `("", "ad")` row of `abhyasasyasavarne_declines_…`.
2. **A long savarṇa follower.** `i` before `I`, `u` before `U` are savarṇa; the rule must decline, not produce *iy·I*. → Task 1, same test.
3. **A consonant-initial abhyāsa ending in `i` before a vowel.** Only the final vowel changes (`ki` → `kiy`), never the whole text. → Task 1, the `("ki", "a", "kiy")` row of `abhyasasyasavarne_gives_…`.
4. **Laṅ's āṭ lands on the abhyāsa, not the aṅga** (*EyaH*, not *iyAraH* or *AiyaH*). → Task 3, `the_vowel_initial_abhyasta_takes_at_and_merges_it_into_the_abhyasa`.
5. **`check()` on the corpus's only single-letter, vowel-initial root.** *iyarti*, *EyaH*, *iyrati*, *iyfhi* must analyse to √ṛ and nothing else. → Task 2, `r_root_analyses_its_reduplicated_forms`.

---

## File Structure

| file | responsibility in this slice |
|---|---|
| `crates/panini-prakriya/src/tinanta/abhyasa.rs` | Task 1: 7.4.60 widened, 7.4.77 widened, 6.4.78 new; module doc |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | Task 1: `tinanta_rule_order_is_pinned` |
| `crates/panini-data/src/lib.rs` | Task 2: the `03.0017` row, counts, the gaṇa-row test |
| `crates/panini/tests/paradigm/data/juhotyadi.rs` | Task 2: 4 `PARADIGM` blocks, 5 `ALTERNATES` rows |
| `crates/panini/tests/paradigm/main.rs` | Task 2: totals, buckets, key census, spot check; Task 5: audit prose |
| `crates/panini-prakriya/src/tinanta/anga.rs` | Task 3: 6.4.71/6.4.72 comments, the live-row test |
| `crates/panini-prakriya/src/tinanta/adesha.rs` | Task 3: 6.1.90 test comment |
| `crates/panini/tests/trace/juhotyadi.rs` | Task 4: four pins |
| `tools/audit/*`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, the 3a spec | Task 5 |
| `AGENTS.md`, maybe `mise.toml` | Task 6 |

---

## Task 1: 6.4.78 *abhyāsasyāsavarṇe*, and 7.4.60 / 7.4.77 widened

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/abhyasa.rs`
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: the existing test helpers in `abhyasa.rs`'s `mod tests`: `slu_prakriya(root: &str, ending: &str) -> Prakriya` and `run_abhyasa_stage(root: &str, number: &'static str) -> Prakriya`.
- Produces: a `Rule` with `id: "6.4.78"`, `name: "aByAsasyAsavarRe"`, last in `ABHYASA_RULES` (after 7.4.78). Tasks 3 and 4 assert its position in traces.

- [ ] **Step 1: Rewrite the 7.4.60 vowel-initial test so it fails**

In `abhyasa.rs`'s `mod tests`, replace the whole `haladih_shesha_declines_for_a_vowel_initial_abhyasa` test with:

```rust
    #[test]
    fn haladih_shesha_trims_a_vowel_initial_abhyasa() {
        // 7.4.60 on √ṛ (slice 3d2): the abhyāsa `ar` (7.4.66's) has no ādi
        // hal to keep, so every consonant goes — ar → a, then 7.4.77 makes it
        // `i` (iyarti). vidyut's trace on 03.0017 credits exactly this. `ap`
        // is the same shape with a consonant 7.4.66 did not put there. `f`
        // alone is the no-op case: nothing to drop, nothing recorded.
        let r_60 = rules().find(|r| r.id == "7.4.60").unwrap();
        for (abhyasa, want) in [("ar", "a"), ("ap", "a")] {
            let mut p = slu_prakriya("f", "ti");
            p.terms[ABHYASA].text = abhyasa.into();
            assert!((r_60.apply)(&mut p), "{abhyasa}");
            assert_eq!(p.terms[ABHYASA].text, want, "{abhyasa}");
            assert_eq!(p.terms[ANGA].text, "f", "{abhyasa}: the aṅga is untouched");
            assert_eq!(p.log.last().unwrap().sutra, "7.4.60");
        }
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        let mut p = slu_prakriya("f", "ti");
        assert!((r_10.apply)(&mut p));
        p.log.clear();
        assert!(!(r_60.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "f");
        assert!(p.log.is_empty());
    }
```

- [ ] **Step 2: Add the 7.4.77 and 6.4.78 guard tests and the stage row**

In `arti_pipartyos_ca_makes_the_abhyasa_vowel_i_for_pr_and_prr`: rename it to `arti_pipartyos_ca_makes_the_abhyasa_vowel_i_for_its_three_rows`, change its comment's first line to `// 7.4.77: *arti* is √ṛ (03.0017) and *piparti* both pf\ (03.0005) and`, and change its loop to:

```rust
        for (root, number, abhyasa) in [
            ("pf", "03.0005", "pa"),
            ("pF", "03.0004", "pa"),
            ("f", "03.0017", "a"),
        ] {
            let mut p = slu_prakriya(root, "ti");
            p.ctx.dhatupatha = number;
            p.terms[ABHYASA].text = abhyasa.into();
            assert!((rule.apply)(&mut p), "{number}");
            assert_eq!(
                p.terms[ABHYASA].text,
                abhyasa.replace('a', "i"),
                "{number}"
            );
            assert_eq!(p.log.last().unwrap().sutra, "7.4.77");
        }
```

In `arti_pipartyos_ca_declines_off_its_rows`, delete the comment sentence `√ṛ (03.0017)` / `// is the sūtra's other root and arrives with its witness in 3d2.` so the comment ends at `names no row.`.

After `arti_pipartyos_ca_declines_off_its_rows`, add:

```rust
    #[test]
    fn abhyasasyasavarne_gives_iyan_and_uvan_before_a_dissimilar_vowel() {
        // 6.4.78, with *aci* and *yvor iyaṅuvaṅau* continued from 6.4.77.
        // √ṛ's abhyāsa `i` before the root's `f` is the corpus witness
        // (iyarti). The `u` arm and the long vowels are the sūtra's too, and
        // no juhotyādi row reaches them. `ki` + `a` checks that only the
        // final vowel is replaced.
        let rule = rules().find(|r| r.id == "6.4.78").unwrap();
        for (abhyasa, root, want) in [
            ("i", "f", "iy"),
            ("I", "a", "iy"),
            ("u", "f", "uv"),
            ("U", "i", "uv"),
            ("ki", "a", "kiy"),
        ] {
            let mut p = slu_prakriya(root, "ti");
            p.terms[ABHYASA].text = abhyasa.into();
            assert!((rule.apply)(&mut p), "{abhyasa}+{root}");
            assert_eq!(p.terms[ABHYASA].text, want, "{abhyasa}+{root}");
            assert_eq!(p.terms[ANGA].text, root, "{abhyasa}+{root}");
            assert_eq!(p.log.last().unwrap().sutra, "6.4.78");
        }
    }

    #[test]
    fn abhyasasyasavarne_declines_before_a_savarna_vowel_a_consonant_or_no_abhyasa() {
        // *asavarṇe*: i before i/I, u before U. *aci*: every 3a–3d abhyāsa
        // meets a consonant-initial root (ci·ki, Ju·hu). *yvoḥ*: an
        // `a`-final abhyāsa. And the empty slot of every other gaṇa, here
        // adādi's vowel-initial √ad, which must never reach iyaṅ.
        let rule = rules().find(|r| r.id == "6.4.78").unwrap();
        for (abhyasa, root) in [
            ("i", "i"),
            ("i", "I"),
            ("u", "U"),
            ("ci", "ki"),
            ("Ju", "hu"),
            ("a", "f"),
            ("", "ad"),
        ] {
            let mut p = slu_prakriya(root, "ti");
            p.terms[ABHYASA].text = abhyasa.into();
            assert!(!(rule.apply)(&mut p), "{abhyasa:?}+{root}");
            assert_eq!(p.terms[ABHYASA].text, abhyasa, "{abhyasa:?}+{root}");
            assert!(p.log.is_empty(), "{abhyasa:?}+{root}");
        }
    }
```

In `the_r_roots_reach_their_abhyasa_through_ur_at_then_haladih_shesha`, change the comment's first line to `// Each of 3d's six rows and 3d2's √ṛ through the whole stage: exactly the`, and append this entry after the `sf` row:

```rust
            (
                "f",
                "03.0017",
                "iy",
                vec!["6.1.10", "7.4.66", "7.4.60", "7.4.77", "6.4.78"],
            ),
```

- [ ] **Step 3: Pin the new rule order**

In `derivation_tests.rs`'s `tinanta_rule_order_is_pinned`, change `"7.4.62", "7.4.76", "7.4.77", "7.4.78", "6.4.71",` to `"7.4.62", "7.4.76", "7.4.77", "7.4.78", "6.4.78", "6.4.71",`. `mise run fmt` re-wraps the array.

- [ ] **Step 4: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -30`
Expected FAIL:
- `haladih_shesha_trims_a_vowel_initial_abhyasa` (7.4.60 declines on `ar`)
- `arti_pipartyos_ca_makes_the_abhyasa_vowel_i_for_its_three_rows` (on `03.0017`)
- both `abhyasasyasavarne_*` tests (panic at `rules().find(...).unwrap()`: no 6.4.78)
- `the_r_roots_reach_their_abhyasa_through_ur_at_then_haladih_shesha` (on `03.0017`)
- `tinanta_rule_order_is_pinned`

- [ ] **Step 5: Widen 7.4.60**

Replace 7.4.60's rule comment (from `// 7.4.60 halādiḥ śeṣaḥ: of the abhyāsa's consonants only the first` down to `// every √hu, √ki and √bhī trace grows a step and the priors break.`) with:

```rust
    // 7.4.60 halādiḥ śeṣaḥ: of the abhyāsa's consonants only the first
    // remains. Three witnesses: √hrī's initial cluster (hrI → hI, which 7.4.59
    // then shortens to hi and 7.4.62 palatalizes to Ji: jihreti), the `r`
    // 7.4.66 leaves on every consonant-initial ṛ-root's abhyāsa (Bar → Ba:
    // bibharti, pa: piparti, Ga: jagharti; slice 3d), and √ṛ's vowel-initial
    // `ar` (slice 3d2), which has no ādi hal to keep and so loses its `r` too
    // (ar → a, then 7.4.77's `i`: iyarti).
    //
    // The sūtra as written: keep the first character if it is a consonant,
    // keep every vowel, drop every other consonant. For a single-ekāc abhyāsa
    // (6.1.10's NARROW note) that is the whole rule. A vowel first character
    // is kept by the same `once(first)` — it is a vowel — so the vowel-initial
    // case needs no arm of its own.
    //
    // The no-op guard is 8.4.53's: for a single-consonant open abhyāsa (and
    // for √ṛ's bare `f`) the result equals the input, and the rule must record
    // nothing there or every √hu, √ki and √bhī trace grows a step and the
    // priors break.
```

In the rule body, delete these lines entirely:

```rust
            // *halādiḥ* names a consonant-initial abhyāsa. vidyut elides √ṛ's
            // `r` too (ar → a on 03.0017, its 7.4.60), so whether this
            // fall-through survives is slice 3d2's to decide with that
            // witness; no curated row reaches it today.
            if is_vowel(first) {
                return false;
            }
```

The `once(first).chain(s[1..].iter().copied().filter(|c| is_vowel(*c)))` computation and the `t == p.terms[ABHYASA].text` no-op check stay unchanged.

- [ ] **Step 6: Widen 7.4.77**

Replace 7.4.77's rule comment with:

```rust
    // 7.4.77 arti-pipartyoś ca: the abhyāsa of √ṛ (*arti*) and √pṛ
    // (*piparti*) takes `i` too. vidyut applies *piparti* to both pf\ and pF,
    // the two rows that spell it. After 7.4.66 and 7.4.60: pa → pi, and √ṛ's
    // a → i (then 6.4.78: iy, iyarti).
    //
    // KEYED BY ROW NUMBER, like 7.4.76 above it: by this stage `ANGA.text`
    // still reads `pf`/`pF`/`f`, but the sūtra names roots, and the precedent
    // is the number.
    //   03.0004 pF  √pṝ
    //   03.0005 pf\ √pṛ
    //   03.0017 f\  √ṛ
    //
    // After 7.4.76: the two name disjoint roots, so only the trace pins decide
    // the order, and they follow vidyut's.
```

and change its guard to:

```rust
            if !matches!(p.ctx.dhatupatha, "03.0004" | "03.0005" | "03.0017") {
```

- [ ] **Step 7: Add 6.4.78**

Append this rule to `ABHYASA_RULES`, after 7.4.78's closing `},` and before the array's `];`:

```rust
    // 6.4.78 abhyāsasyāsavarṇe: an abhyāsa-final i/u (short or long) becomes
    // iyaṅ/uvaṅ before a vowel that is not savarṇa with it — *aci* and *yvor
    // iyaṅuvaṅau* continue from 6.4.77. √ṛ's abhyāsa, `i` after 7.4.77, meets
    // the root's `f`: iyarti; in laṅ 6.1.90 then merges the āṭ into it,
    // A+iy → Ey (EyaH).
    //
    // HERE, not beside 6.4.77 in `guna`: every edit to the abhyāsa's text
    // lives in this stage, vidyut applies it straight after 7.4.77 on every
    // √ṛ prakriyā, and running before `anga` puts it ahead of 6.4.72/6.1.90
    // (laṅ) and 6.1.77 (iy·f·ati → iyrati, not *irati) by construction.
    //
    // The follower is ANGA's first character. At this stage that is the bare
    // root's (`f`); guṇa later makes it `a` on pit cells, asavarṇa with `i`
    // either way. The savarṇa test is local because `sound::is_savarna` knows
    // stop series only.
    //
    // Guarded on the sound, not the row, like 7.4.66: the sūtra names sounds.
    // √ṛ is the only row that reaches it — every other abhyāsa meets a
    // consonant-initial root, and every other gaṇa's slot is empty — so the
    // `u` arm and the savarṇa clause are held by synthetic guard tests.
    Rule {
        id: "6.4.78",
        name: "aByAsasyAsavarRe",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let (glide, savarna) = match p.terms[ABHYASA].text.chars().last() {
                Some('i' | 'I') => ("iy", ['i', 'I']),
                Some('u' | 'U') => ("uv", ['u', 'U']),
                _ => return false,
            };
            let Some(next) = p.terms[ANGA].text.chars().next() else {
                return false;
            };
            if !is_vowel(next) || savarna.contains(&next) {
                return false;
            }
            let before = p.snapshot();
            let mut t = p.terms[ABHYASA].text.clone();
            t.pop();
            t.push_str(glide);
            p.terms[ABHYASA].text = t;
            p.record("6.4.78", "aByAsasyAsavarRe", before);
            true
        },
    },
```

Update the module doc's first two lines to:

```rust
//! Reduplication: 6.1.10, 7.4.66, 7.4.60, 7.4.59, 7.4.62, 7.4.76, 7.4.77,
//! 7.4.78, 6.4.78 — dvitva and the rules that reshape the abhyāsa.
```

- [ ] **Step 8: Run the unit tests**

Run: `mise exec -- cargo test -p panini-prakriya 2>&1 | tail -30`
Expected: PASS.

- [ ] **Step 9: Run the full suite (priors byte-identical)**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms)
Expected: PASS at 4176 cells, no golden or trace changed. √ṛ is not curated yet, so nothing in the corpus reaches the new behaviour.

- [ ] **Step 10: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/abhyasa.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(abhyasa): 6.4.78 abhyāsasyāsavarṇe; 7.4.60 trims a vowel-initial abhyāsa; 7.4.77 gains √ṛ"
```

---

## Task 2: The √ṛ row and its paradigm goldens

This task turns the 36 new cells green.

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `crates/panini/tests/paradigm/data/juhotyadi.rs`
- Modify: `crates/panini/tests/paradigm/main.rs`

**Interfaces:**
- Consumes: Task 1.
- Produces: `dhatus()` row `03.0017`, code `f`, `Gana::Juhotyadi`, `PadaAssignment::Parasmaipada`. Tasks 3 and 4 look it up by number.

- [ ] **Step 1: Add the `Dhatu` row**

`DHATUS` is ordered by number. Insert after the `03.0016` row (artha verbatim from `data/dhatupatha.tsv` line 1281):

```rust
    Dhatu {
        // 03.0017 `f\` gatO (√ṛ). Parasmaipadī by 1.3.78 (the `\` is the root
        // vowel's accent). The gaṇa's one vowel-initial ṛ-root: 7.4.66 gives
        // the abhyāsa `ar`, 7.4.60 trims it to `a` (no ādi hal to keep),
        // 7.4.77 makes it `i`, keyed by this number, and 6.4.78 makes that
        // `iy` before the root's vowel: iyarti. In laṅ 6.4.72's āṭ merges
        // into the abhyāsa by 6.1.90: EyaH. Slice 3d2.
        dhatupatha: "03.0017",
        code: "f",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "gatO",
    },
```

Counts in the same file:
- `Dhatu`'s doc: `re-derives 92 of these 93` → `re-derives 93 of these 94`; `The test covers the 93 roots curated here` → `The test covers the 94 roots curated here`.
- `pada_from_upadesha`'s doc: `62 of the 93 curated roots carry a \`\\\` at all, and 41` → `63 of the 94 curated roots carry a \`\\\` at all, and 42` (`f\`'s one `\` sits on the root vowel).
- `curated_roots_have_expected_ganas_and_padas`: `assert_eq!(dhatus().len(), 93);` → `94`.

- [ ] **Step 2: Extend the gaṇa-row test**

Rename `juhotyadi_rows_are_the_sixteen_curated_roots` to `juhotyadi_rows_are_the_seventeen_curated_roots`, and update the other reference (`grep -rn "juhotyadi_rows_are_the_sixteen" crates` finds it, in a comment near line 1887).

In its comment, replace:

```rust
        // ñit), √ghṛ, √hṛ and √sṛ. The gaṇa is PARTIAL at 16 of its 26
        // dhātupāṭha rows; slices 3d2, 3e and 3f close it.
```

with:

```rust
        // ñit), √ghṛ, √hṛ and √sṛ. Slice 3d2 adds √ṛ (03.0017), parasmaipadī
        // by 1.3.78, the gaṇa's one vowel-initial ṛ-root. The gaṇa is PARTIAL
        // at 17 of its 26 dhātupāṭha rows; slices 3e and 3f close it.
```

In its expected vector, insert after the `03.0016` line:

```rust
                ("03.0017", "f", PadaAssignment::Parasmaipada),
```

In the same test's final comment (the root-keyed list beginning `// Every root-specific rule of slices 3c, 3c2 and 3d`), change `slices 3c, 3c2 and 3d` → `slices 3c, 3c2, 3d and 3d2` and `` `pf`, `pF` and `Bf` `` → `` `pf`, `pF`, `Bf` and `f` ``. After `(a ṛ-vowel; a labial before ṝ), not roots.` add ` Slice 3d2's 6.4.78 names sounds too (an abhyāsa-final i/u before a dissimilar vowel).` If the comment's wording differs, keep its form and make the same additions.

- [ ] **Step 3: Run the data tests**

Run: `mise exec -- cargo test -p panini-data 2>&1 | tail -20`
Expected: PASS, including `curated_pada_agrees_with_upadesha_markers` and `dhatupatha_numbers_resolve_upstream`.

- [ ] **Step 4: Add the `PARADIGM` blocks**

**Append** to the end of `PARADIGM` in `crates/panini/tests/paradigm/data/juhotyadi.rs` (blocks are ordered by slice, not number). Index 0 of each multi-form cell is the declined derivation (`-tu`, `-hi`, `-yAd`). `mise run fmt` re-wraps.

```rust
    ("03.0017", "laT", Pada::Parasmaipada, ["iyarti", "iyftaH", "iyrati", "iyarzi", "iyfTaH", "iyfTa", "iyarmi", "iyfvaH", "iyfmaH"]),
    ("03.0017", "laN", Pada::Parasmaipada, ["EyaH", "EyftAm", "EyaruH", "EyaH", "Eyftam", "Eyfta", "Eyaram", "Eyfva", "Eyfma"]),
    ("03.0017", "loT", Pada::Parasmaipada, ["iyartu", "iyftAm", "iyratu", "iyfhi", "iyftam", "iyfta", "iyarARi", "iyarAva", "iyarAma"]),
    ("03.0017", "viDiliN", Pada::Parasmaipada, ["iyfyAd", "iyfyAtAm", "iyfyuH", "iyfyAH", "iyfyAtam", "iyfyAta", "iyfyAm", "iyfyAva", "iyfyAma"]),
```

- [ ] **Step 5: Add the `ALTERNATES` rows**

**Append** to the end of `ALTERNATES` (index = 0-based cell, P.E P.D P.B M.E M.D M.B U.E U.D U.B; key = the optional rules behind the form, in pipeline order):

```rust
    ("03.0017", "loT", Pada::Parasmaipada, 0, "iyftAd", "7.1.35"),
    ("03.0017", "loT", Pada::Parasmaipada, 0, "iyftAt", "7.1.35+8.4.56"),
    ("03.0017", "loT", Pada::Parasmaipada, 3, "iyftAd", "7.1.35"),
    ("03.0017", "loT", Pada::Parasmaipada, 3, "iyftAt", "7.1.35+8.4.56"),
    ("03.0017", "viDiliN", Pada::Parasmaipada, 0, "iyfyAt", "8.4.56"),
```

Five rows: 36 cells + 5 = 41 forms, vidyut's count. Laṅ prathama eka does not fork: its `r` goes to visarga (`EyaH`).

- [ ] **Step 6: Update `paradigm/main.rs`**

In `every_alternate_names_the_vikalpa_rules_that_produced_it`'s doc comment: `otherwise 1032 bare strings` → `otherwise 1037 bare strings`.

In `derivation_set_shape_matches_the_audited_numbers`:
- `assert_eq!(total_cells, 4176, "464 root×lakāra blocks × 9 cells each")` → `4212`, `"468 root×lakāra blocks × 9 cells each"`.
- `ones` 3418 → `3451`; `twos` 577 → `578`.
- `threes` 135 → `137`; in its message change the final `the six ṛ-roots', the same \` / `way"` to `the six ṛ-roots', the same way; and — new in slice 3d2 — √ṛ's, the same way"` (keep the string's `\` line-continuation style).
- `fours`, `fives`, `sixes`, `sevens` unchanged.
- `ALTERNATES.len()` 1032 → `1037`.
- `key_count("8.4.56")` 152 → `153`; `key_count("7.1.35")` 136 → `138`; `key_count("7.1.35+8.4.56")` 136 → `138`.

Check: 3451 + 578 + 137 + 18 + 10 + 17 + 1 = 4212 cells; 3451 + 1156 + 411 + 72 + 50 + 102 + 7 = 5249 forms = 4212 + 1037.

In the doc comment above that test:
- `4176 cells total (464 root×lakāra blocks × 9), of which 3418 hold exactly` / `one form, 577 hold two, 135 hold three (` → `4212 cells total (468 root×lakāra blocks × 9), of which 3451 hold exactly one form, 578 hold two, 137 hold three (`.
- In that three-form parenthesis, after `and the six ṛ-roots', new in slice 3d`, add `, and √ṛ's, new in slice 3d2`.
- `itself has 1032 rows, keyed 152 \`8.4.56\`, 136 \`7.1.35\`, 136 \`7.1.35+8.4.56\`,` → `itself has 1037 rows, keyed 153 \`8.4.56\`, 138 \`7.1.35\`, 138 \`7.1.35+8.4.56\`,`.
- After the slice-3d paragraph (ending `pre-existing keys. The gaṇa is PARTIAL at 16 of its 26 rows.`), add:

```rust
///
/// Slice 3d2 curates √ṛ (`03.0017`), the gaṇa's one vowel-initial ṛ-root,
/// bringing it to seventeen of its twenty-six rows. Its machinery (6.4.78,
/// 7.4.60 on a vowel-initial abhyāsa, 7.4.77's √ṛ row) adds no vikalpa and no
/// fork kind: vidhiliṅ prathama eka forks on 8.4.56 (`iyfyAd`/`iyfyAt`) and
/// the two loṭ tātaṅ cells three ways (`iyartu`/`iyftAd`/`iyftAt`,
/// `iyfhi`/`iyftAd`/`iyftAt`); laṅ prathama eka ends in visarga (`EyaH`) and
/// does not fork. Five new rows, all in the three pre-existing keys. The gaṇa
/// is PARTIAL at 17 of its 26 rows.
```

`pada_ambiguous_surfaces_are_exactly_these` is unchanged: √ṛ is parasmaipada-only.

After `both_pr_roots_analyse_their_shared_forms`, add the Review Focus test:

```rust
/// √ṛ (`03.0017 f\`) is the corpus's only single-letter root and its only
/// vowel-initial abhyasta: every surface starts with the abhyāsa (`iy-`) or
/// the āṭ merged into it (`Ey-`), never with the root. `check` must still find
/// √ṛ, and only √ṛ.
#[test]
fn r_root_analyses_its_reduplicated_forms() {
    let engine = Panini::new();
    for form in ["iyarti", "EyaH", "iyrati", "iyfhi"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        let mut v: Vec<String> = r.analyses.iter().map(|a| a.dhatu.clone()).collect();
        v.sort_unstable();
        v.dedup();
        assert_eq!(v, vec!["f"], "{form}");
    }
}
```

- [ ] **Step 7: Run the full suite**

Run: `mise run test 2>&1 | tail -40` (foreground, timeout 600000 ms)
Expected: PASS at 4212 cells, every 4176 prior unchanged.

If a √ṛ cell fails, read it against the spec before touching anything, using superpowers:systematic-debugging. **Do not edit a golden to match the engine.** Likely causes by symptom:
- `ar-`/`Ar-` abhyāsa: 7.4.60 still declines (Task 1 Step 5).
- `a-`/`Ay-`: 7.4.77 did not fire (Task 1 Step 6).
- `ir-`/`Er-` or `irati`: 6.4.78 did not fire, or ran after 6.1.77.
- `AiyaH`/`iyAraH`: 6.1.90 did not merge into `ABHYASA` — Task 3's subject; stop and report.

- [ ] **Step 8: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini-data/src/lib.rs crates/panini/tests/paradigm/data/juhotyadi.rs crates/panini/tests/paradigm/main.rs
git commit -m "feat(data): √ṛ (03.0017) — juhotyādi at seventeen of twenty-six rows

4176 → 4212 cells, 5208 → 5249 forms, 1032 → 1037 ALTERNATES, 93 → 94 roots."
```

---

## Task 3: Close the augment-guard checkpoint

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/anga.rs` (6.4.71's and 6.4.72's comments; one test)
- Modify: `crates/panini-prakriya/src/tinanta/adesha.rs` (a test comment only)

**Interfaces:**
- Consumes: the `03.0017` row (Task 2); `derive`, `sole` and `dhatus` already imported by `anga.rs`'s `mod tests`.
- Produces: nothing later tasks use.

- [ ] **Step 1: Write the live-row test**

In `anga.rs`'s `mod tests`, change `use crate::tinanta::terms::{AGAMA, with_slots};` to `use crate::tinanta::terms::{ABHYASA, AGAMA, with_slots};`, and after `at_augment_declines_outside_lan_and_for_vowel_initial_angas` add:

```rust
    #[test]
    fn the_vowel_initial_abhyasta_takes_at_and_merges_it_into_the_abhyasa() {
        // √ṛ (03.0017) laṅ prathama eka, EyaH: the witness 6.4.71's comment
        // names. 6.4.72 reads ANGA's `f`, the abhyāsa reads `iy` — both
        // vowel-initial — so āṭ, not aṭ; then 6.1.90 writes the vṛddhi into
        // the abhyāsa, the first non-empty term after the augment.
        let d = dhatus().iter().find(|d| d.dhatupatha == "03.0017").unwrap();
        let p = sole(derive(
            d,
            Lakara::Lan,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        ));
        assert_eq!(p.text(), "EyaH");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert!(ids.contains(&"6.4.72"), "{ids:?}");
        assert!(!ids.contains(&"6.4.71"), "{ids:?}");
        assert_eq!(p.terms[AGAMA].text, "");
        assert_eq!(p.terms[ABHYASA].text, "Ey");
    }
```

- [ ] **Step 2: Run it**

Run: `mise exec -- cargo test -p panini-prakriya the_vowel_initial_abhyasta 2>&1 | tail -15`
Expected: PASS. This is a pin of behaviour Tasks 1–2 already produce, not a test of new code; it holds the checkpoint's verdict under mutation.

- [ ] **Step 3: Rewrite the 6.4.71 comment**

Replace the `READS \`ANGA\`, NOT THE FIRST NON-EMPTY TERM AFTER \`AGAMA\`.` paragraph above 6.4.71 with:

```rust
    // READS `ANGA`, NOT THE FIRST NON-EMPTY TERM AFTER `AGAMA`. Grammatically
    // the augment precedes the whole aṅga, abhyāsa included, and 6.1.90
    // already merges the āṭ into the first non-empty term after the slot.
    // The consonant/vowel verdict is the same either way for every
    // juhotyādi row: the abhyāsa is a copy of the root's first ekāc, and no
    // rule of the abhyāsa stage changes its initial's class — 7.4.60 keeps
    // the first consonant (or, vowel-initial, the vowel), 7.4.62 substitutes
    // consonant for consonant, 7.4.66 and 7.4.77 vowel for vowel, and 6.4.78
    // only appends a glide. √ṛ (03.0017, slice 3d2) is the vowel-initial
    // witness: ANGA reads `f`, the abhyāsa `iy`, and both reads give āṭ
    // (EyaH). Reading the abhyāsa instead would add a clause no row can
    // falsify (3e's and 3f's roots are consonant-initial), so the ANGA read
    // stays.
```

6.4.72's one-line pointer (`// Reads ANGA's own initial for the same reason 6.4.71 does — see its comment.`) stays as it is.

- [ ] **Step 4: Update the 6.1.90 test comment**

In `adesha.rs`'s `awas_ca_anga_arm_merges_the_agama_into_the_first_non_empty_term`, replace:

```rust
        // "First non-empty term after AGAMA", not "ANGA": an abhyāsa in
        // front of the aṅga is what meets the āṭ (slice 3d2's √ṛ, A+iy+ar →
        // Eyar). No curated root reaches 6.1.90 with a filled ABHYASA until
        // slice 3d2's √ṛ, so this case stays synthetic until then; this pins
        // the arm's addressing so 3d2 inherits it rather than re-deriving it.
```

with:

```rust
        // "First non-empty term after AGAMA", not "ANGA": an abhyāsa in
        // front of the aṅga is what meets the āṭ (√ṛ, 03.0017, A+iy+ar →
        // Eyar, EyaH). This is the arm's addressing in isolation; the live
        // row is pinned in anga.rs's
        // `the_vowel_initial_abhyasta_takes_at_and_merges_it_into_the_abhyasa`.
```

- [ ] **Step 5: Run the full suite and commit**

Run: `mise run test 2>&1 | tail -20` (foreground, timeout 600000 ms). Expected: PASS.

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/anga.rs crates/panini-prakriya/src/tinanta/adesha.rs
git commit -m "test(anga): √ṛ closes the augment-guard checkpoint — 6.4.72 on ANGA, 6.1.90 into the abhyāsa"
```

---

## Task 4: The trace pins

**Files:**
- Modify: `crates/panini/tests/trace/juhotyadi.rs`

**Interfaces:**
- Consumes: `crate::helpers::{at, cell_trace}`. `cell_trace(number, lakara, pada, purusha, vacana) -> (String, Vec<String>)` returns index 0's text and trace; `at(&[String], &str) -> usize` panics if the sūtra is absent. Every cell pinned below is single-form.

- [ ] **Step 1: Update the module doc**

In the file's `//!` doc, after `(*pipūrtaḥ*'s` / `//! abhyāsa is copied from \`pF\`).`, add: `//! The same order runs 6.4.78 (slice 3d2's √ṛ) inside the abhyāsa stage, before laṅ's 6.4.72 and before guṇa, where vidyut guṇates first and reaches 6.4.78 later.` Reflow to the file's wrap.

- [ ] **Step 2: Add the pins**

At the end of the file, add:

```rust
#[test]
fn iyarti_trace_is_ur_at_haladih_shesha_arti_then_abhyasasyasavarne() {
    // f P laT P.E. A pit cell: 7.4.66 fires on the bare copy here (vidyut
    // copies the guṇated `ar` and skips it), and 6.4.78 precedes guṇa.
    let (text, t) = cell_trace(
        "03.0017",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "iyarti", "got {t:?}");
    assert!(at(&t, "6.1.10") < at(&t, "7.4.66"), "got {t:?}");
    assert!(at(&t, "7.4.66") < at(&t, "7.4.60"), "got {t:?}");
    assert!(at(&t, "7.4.60") < at(&t, "7.4.77"), "got {t:?}");
    assert!(at(&t, "7.4.77") < at(&t, "6.4.78"), "got {t:?}");
    assert!(at(&t, "6.4.78") < at(&t, "7.3.84"), "got {t:?}");
}

#[test]
fn iyftah_trace_has_abhyasasyasavarne_and_no_guna() {
    // f P laT P.D. kṅit: the aṅga keeps its `f`, which is what 6.4.78 reads.
    let (text, t) = cell_trace(
        "03.0017",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "iyftaH", "got {t:?}");
    assert!(at(&t, "7.4.77") < at(&t, "6.4.78"), "got {t:?}");
    assert!(!t.contains(&"7.3.84".to_string()), "got {t:?}");
}

#[test]
fn iyrati_trace_is_abhyasasyasavarne_before_iko_yan_aci() {
    // f P laT P.B. 6.4.78 must see the root's vowel before 6.1.77 turns it
    // into `r`, or the abhyāsa would stay `i` (*irati).
    let (text, t) = cell_trace(
        "03.0017",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "iyrati", "got {t:?}");
    assert!(at(&t, "6.4.78") < at(&t, "7.1.4"), "got {t:?}");
    assert!(at(&t, "7.1.4") < at(&t, "6.1.77"), "got {t:?}");
}

#[test]
fn eyaruh_trace_is_abhyasasyasavarne_then_at_then_awas_ca_on_the_abhyasa() {
    // f P laN P.B. 6.4.72 (āṭ, not 6.4.71's aṭ) reads ANGA's vowel initial;
    // 7.3.83 guṇates before jus; 6.1.90 merges A+iy → Ey into the abhyāsa.
    let (text, t) = cell_trace(
        "03.0017",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "EyaruH", "got {t:?}");
    assert!(at(&t, "6.4.78") < at(&t, "6.4.72"), "got {t:?}");
    assert!(at(&t, "6.4.72") < at(&t, "7.3.83"), "got {t:?}");
    assert!(at(&t, "7.3.83") < at(&t, "6.1.90"), "got {t:?}");
    assert!(!t.contains(&"6.4.71".to_string()), "got {t:?}");
}
```

- [ ] **Step 3: Run the suite**

Run: `mise run test 2>&1 | tail -20` (foreground, timeout 600000 ms)
Expected: PASS. If a pin fails on ORDER, the engine's order is the one to report, not to "fix" toward vidyut's; stop and report the trace.

- [ ] **Step 4: Commit**

```bash
mise run fmt && mise run lint
git add crates/panini/tests/trace/juhotyadi.rs
git commit -m "test(trace): 3d2 pins — iyarti, iyftaH, iyrati, EyaruH"
```

---

## Task 5: Audit, counts and the doc sweep

**Files:**
- Modify: `tools/audit/panini_full_audit.rs`, `tools/audit/README.md`, `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, `crates/panini/tests/paradigm/main.rs` (audit prose), `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md` (one note), any stale engine comment

**Interfaces:**
- Consumes: the finished engine and goldens (Tasks 1–4). Produces no symbols.

- [ ] **Step 1: Update the audit harness's asserted totals**

In `tools/audit/panini_full_audit.rs`:
- header `for each of the 93 curated roots` → `94`
- `93 roots, 4176 cells, 5208 forms` → `94 roots, 4212 cells, 5249 forms`
- `464 root×pada×lakāra` / `blocks × 9 cells, plus 1032` → `468 … plus 1037`
- `the full 4176-cell table` → `4212-cell`
- `assert_eq!(roots_seen.len(), 93, …)` → `94`
- `assert_eq!(n_cells, 4176, "cells: 464 root×pada×lakāra blocks × 9")` → `4212`, `"cells: 468 root×pada×lakāra blocks × 9"`
- `assert_eq!(n_forms, 5208, "forms: 4176 cells + 1032 ALTERNATES rows")` → `5249`, `"forms: 4212 cells + 1037 ALTERNATES rows"`

`twenty-two ubhayapadī by 1.3.72` is unchanged (√ṛ is parasmaipadī).

In `tools/audit/README.md`, `(93 roots, 4176 cells, 5208 forms)` → `(94 roots, 4212 cells, 5249 forms)`.

- [ ] **Step 2: Repoint vidyut's dev-deps at THIS worktree and run the audit**

`/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` hardcodes absolute dev-dep paths to `/workspace/crates` — the `main` checkout, which does not have this slice. Auditing without repointing checks the pre-slice engine and passes vacuously.

```bash
WT="$(git rev-parse --show-toplevel)"
sed -i "s#^panini = { path = .*#panini = { path = \"$WT/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"$WT/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
grep -n '^panini' /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
cp tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" mise exec rust@1.98.1 -- cargo run --release --example panini_full_audit 2>&1 | tail -15)
(cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_REPO="$WT" PANINI_AUDIT_PERTURB=entry mise exec rust@1.98.1 -- cargo run --release --example panini_full_audit 2>&1 | tail -8)
```

Copy the committed harness; never rewrite it.
- Expected from the honest run: `AUDIT PASSED: 4212 cells, 5249 forms, zero differences.`
- Expected from the `entry` control: exit 1 with 36 √bhū cells.

If the honest run shows differences, stop and report; edit nothing. After both runs, restore the dev-deps so later probes from `main` work:

```bash
sed -i "s#^panini = { path = .*#panini = { path = \"/workspace/crates/panini\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
sed -i "s#^panini-data = { path = .*#panini-data = { path = \"/workspace/crates/panini-data\" }#" /tmp/vidyut-full/vidyut-prakriya/Cargo.toml
```

- [ ] **Step 3: Record the audit**

In `tools/audit/README.md`, immediately under `## Last recorded result`, add a new dated entry above the 3d one (today's date):

```markdown
YYYY-MM-DD, juhotyādi 3d2 slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 4212
cells / 5249 forms / 94 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole juhotyādi 3d2 slice: 6.4.78
*abhyāsasyāsavarṇe*, 7.4.60 on a vowel-initial abhyāsa, and 7.4.77's √ṛ row,
added for √ṛ (`03.0017`), with 6.4.72 and 6.1.90 reaching the abhyāsa
unchanged.

Totals: 94 = 93 + 1; 4212 = 4176 + 36 (4 root×pada×lakāra blocks × 9); 5249
= 5208 + 36 + 5 new `ALTERNATES` rows (1032 → 1037), measured via the
harness's corpus block, not assumed.
```

In `crates/panini/tests/paradigm/main.rs`'s audit-chain doc comment, before the final `.` of the sentence ending `…and juhotyādi 3d's re-ran it at the same commit over all 4176` / `cells / 5208 forms / 93 roots with zero differences, its \`entry\` negative` / `control verified failing (36 √bhū cells)`, add `, and juhotyādi 3d2's re-ran it at the same commit over all 4212 cells / 5249 forms / 94 roots with zero differences, its \`entry\` negative control verified failing (36 √bhū cells)`.

- [ ] **Step 4: README.md**

- `**partial** at 16 of its 26 dhātupāṭha rows` → `**partial** at 17 of its 26 dhātupāṭha rows`.
- After the sentence ending `given an \`r\` arm (*bibharṣi*).`, insert: `Slice 3d2 added √ṛ (\`03.0017\`, *iyarti*), the gaṇa's one vowel-initial ṛ-root, behind 6.4.78 *abhyāsasyāsavarṇe*, with 7.4.60 trimming a vowel-initial abhyāsa (\`ar\` → \`a\`) and 7.4.77 given its √ṛ row; in laṅ the āṭ merges into the abhyāsa (*aiyaḥ*, \`EyaH\`).`
- `curated 93-root set` → `curated 94-root set`.
- Multi-form paragraph: `758 of the 4176 cells hold more than one form: 577` / `hold two, 135 hold three (` → `761 of the 4212 cells hold more than one form: 578 hold two, 137 hold three (`; after `and — new in slice 3d —` / `the six ṛ-roots'` add `, and — new in slice 3d2 — √ṛ's`. Check: 578 + 137 + 18 + 10 + 17 + 1 = 761.

Reflow edited paragraphs to the file's ~80-column wrap.

- [ ] **Step 5: docs/ARCHITECTURE.md**

- Stage table, `abhyasa.rs` row: `6.1.10, 7.4.66, 7.4.60, 7.4.59, 7.4.62, 7.4.76, 7.4.77, 7.4.78 — dvitva and the abhyāsa's shape` → `…, 7.4.78, 6.4.78 — dvitva and the abhyāsa's shape`.
- `pins all 121 ids verbatim` → `pins all 122 ids verbatim`. After `not added) — 121 total` add ` — then juhotyādi 3d2's one: 6.4.78 *abhyāsasyāsavarṇe* (7.4.60 and 7.4.77 widened) — 122 total`. Confirm 122 by counting `expected` in `tinanta_rule_order_is_pinned`.
- `**partial** at 16 of its 26 rows (… √sṛ; slice 3d)` → `**partial** at 17 of its 26 rows (… √sṛ; slice 3d; √ṛ; slice 3d2)`.
- The 7.1.35 / 8.4.56 paragraph (`grep -n "forking 136 cells" docs/ARCHITECTURE.md`):
  - `forking 136 cells` → `138`
  - `across the 68 roots with a parasmaipada column` → `69`
  - juhotyādi list `√pṝ, √pṛ, √ghṛ (\`Gf\`), √hṛ and √sṛ (all parasmaipada-only)` → `√pṝ, √pṛ, √ghṛ (\`Gf\`), √hṛ, √sṛ and √ṛ (all parasmaipada-only)`
  - `68 + 25 = the 93 curated roots` → `69 + 25 = the 94 curated roots`
  - `forking 152 cells outright` → `153`
  - `those same 68 parasmaipada columns (130 of them` → `69 … (131 of them`
  - `the six juhotyādi 3d ṛ-roots` / `contribute only their vidhiliṅ cell, since their laṅ prathama eka ends in` / `the aṅga's \`r\`, turned to visarga, not in a jaś: \`abiBaH\`)` → `the seven juhotyādi ṛ-roots (3d's six and 3d2's √ṛ) contribute only their vidhiliṅ cell, since their laṅ prathama eka ends in the aṅga's \`r\`, turned to visarga, not in a jaś: \`abiBaH\`, \`EyaH\`)`

  If wording differs, keep its form and make the same substitution. What must be right: 1 new parasmaipada column, 2 new `7.1.35` rows, 1 new `8.4.56` row.
- The checkpoint sentence (`grep -n "3d2's √ṛ is where that argument" docs/ARCHITECTURE.md`): replace `6.4.71/6.4.72 still read \`ANGA\`'s initial rather` … `and 3d2's √ṛ is where that argument meets a` / `vowel-initial witness.` with:

```markdown
6.4.71/6.4.72 read `ANGA`'s initial rather than the abhyāsa's: a
reduplicant's initial is always the same class as its root's (7.4.60 keeps
the first consonant, or the vowel of a vowel-initial copy; 7.4.62, 7.4.66,
7.4.77 substitute within class; 6.4.78 only appends a glide). Slice 3d2's √ṛ
is the vowel-initial witness that closed the question: `ANGA` reads `f`, the
abhyāsa `iy`, both give āṭ, and 6.1.90 writes the vṛddhi into the abhyāsa
(*aiyaḥ*, `EyaH`). 6.4.78 *abhyāsasyāsavarṇe* runs in the abhyāsa stage,
straight after 7.4.77 as in vidyut, so it precedes 6.4.72/6.1.90 and 6.1.77
(*iyrati*, not \**irati*) by construction.
```

- [ ] **Step 6: AGENTS.md**

- Rules of the codebase: `(\`crates/panini/tests/paradigm/\`, 4176 cells,` → `4212 cells`. In the juhotyādi clause, `and now at 16 of its 26 after slice 3d curated √pṝ, √pṛ, √bhṛ, √ghṛ, √hṛ` / `and √sṛ —` → `at 16 after slice 3d curated √pṝ, √pṛ, √bhṛ, √ghṛ, √hṛ and √sṛ, and now at 17 of its 26 after slice 3d2 curated √ṛ —`. `(1032 rows in all, so 4176 + 1032 = 5208 forms total)` → `(1037 rows in all, so 4212 + 1037 = 5249 forms total)`. Its bucket counts `a second (577 cells), a third (135 cells)` → `(578 cells)`, `(137 cells)`.
- The audit record: after `and that by juhotyādi 3d's (\`tools/audit/README.md\`'s 2026-09-29` / `entry, 4176 cells / 5208 forms / 93 roots)` add `, and that by juhotyādi 3d2's (\`tools/audit/README.md\`'s YYYY-MM-DD entry, 4212 cells / 5249 forms / 94 roots)`.
- The stale-comment paragraph: `4176 goldens` / `would move today` → `4212 goldens`. After `…both lines measured by grep at this commit).` (the 3d sentence's end) add ` Juhotyādi 3d2 touched neither comment either; the corpus stands at 4212 cells as of 3d2 (\`guna.rs:2226\`'s claim still at \`guna.rs:<G>\`, \`controller.rs:206\`'s at \`controller.rs:<C>\`: 3d2 did not touch \`guna.rs\` or \`controller.rs\`; both lines measured by grep at this commit).` Measure `<G>` with `grep -n "1872 goldens" crates/panini-prakriya/src/tinanta/guna.rs` and `<C>` with `grep -n "1800 goldens" crates/panini-prakriya/src/controller.rs`; expect 2226 and 206.

- [ ] **Step 7: Note the slice in 3a's spec**

In `docs/superpowers/specs/2026-09-05-juhotyadi-gana-design.md`, after the slice-3d note beneath the "Later slices" table, add:

```markdown
> Slice 3d2 (`2026-09-30-juhotyadi-gana-3d2-design.md`) took √ṛ, 36 cells:
> 6.4.78, 7.4.60 on a vowel-initial abhyāsa and 7.4.77's √ṛ row. It closed
> this spec's augment-guard checkpoint by argument plus witness — 6.4.71 /
> 6.4.72 keep reading `ANGA`.
```

- [ ] **Step 8: Sweep for anything left stale**

```bash
grep -rn "4176\|5208\|\b1032\b\|93 roots\|93-root\|of these 93\|of the 93\|\b464\b\|758 of\|16 of its 26\|sixteen of its twenty-six\|121 ids\|121 total\|forking 136\|forking 152\|130 of them\|68 + 25\|\b577\b\|135 hold\|(135 cells)" README.md AGENTS.md docs/ARCHITECTURE.md crates tools --include=*.md --include=*.rs
grep -rn "3d2" crates tools README.md AGENTS.md docs/ARCHITECTURE.md
grep -rn "\bar\b\|\biy\b\|\bEy\b\|f\\\\\|vowel-initial" crates/panini-prakriya/src --include=*.rs | grep "//"
```

Expected residue:
- AGENTS.md's dated mutation record and anything naming 3d's numbers explicitly as 3d's (history; never rewrite).
- "3d2" where it now records what 3d2 *did* (Tasks 1–5's own new comments).

Every "3d2" that promises something *will* happen is now false: rewrite it as done or delete it. For every root-shape hit, check the comment is still true with √ṛ curated.

- [ ] **Step 9: Run the full suite and commit**

Run: `mise run test 2>&1 | tail -30` (foreground, timeout 600000 ms). Expected: PASS at 4212 cells.

```bash
mise run fmt && mise run lint
git add -A
git commit -m "docs: 3d2's counts, the audit record, and the checkpoint closed

4212 cells / 5249 forms / 94 roots / 1037 ALTERNATES across README,
ARCHITECTURE, AGENTS, paradigm/main.rs and tools/audit. Audit at zero
divergence against 8da2f90b."
```

---

## Task 6: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the floor paragraph and the current-record paragraph); `mise.toml` only if the cap moves

Follow AGENTS.md's cargo-mutants protocol. Hazards from this repo's record:
- **Measure, never scale.**
- **Every invocation rotates `mutants.out`**, so always pass `-o`.
- **The mise shim fails in background shells.** Use the real binary: `/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants`.
- **`pgrep -f` matches its own shell.** Wait on `pgrep -x cargo-mutants`.

- [ ] **Step 1: Measure the floor**

With nothing else running, twice: `time mise run test 2>&1 | tail -3` (foreground). Record both wall clocks and `cat /proc/loadavg`. The 4176-cell floor was 5.299 s / 5.332 s.

- [ ] **Step 2: Probe the two uncaught equivalents at `-j 4`**

This slice does not touch lines above `adesha.rs:588` or anywhere in `tripadi.rs`, so both should sit where AGENTS.md records them. Confirm, then run them (foreground, timeout 600000 ms):

```bash
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
mise exec -- "$CM" mutants --package panini-prakriya --list 2>/dev/null | grep -E "adesha.rs:588:30: replace \+ with \*|tripadi.rs:1217:38: replace - with /"
SCRATCH="$(mktemp -d)"
mise exec -- env -u CARGO_MUTANTS_JOBS "$CM" mutants --package panini-prakriya --test-workspace=true \
  --timeout 60 -j 4 -o "$SCRATCH" \
  --re "adesha.rs:588:30: replace \+ with \*" --re "tripadi.rs:1217:38: replace - with /" 2>&1 | tail -10
```

Both must be MISSED, not TIMEOUT. Read each test-phase duration from `$SCRATCH/mutants.out/outcomes.json`. If `--list` shows either at another position, use the listed position here and below.

- [ ] **Step 3: Run the campaign detached**

```bash
OUT="$HOME/mutants-records/juhotyadi-3d2"   # durable: outside the repo and any scratchpad
mkdir -p "$OUT"
eval "$(mise env -s bash)"
CM=/home/dev/.local/share/mise/installs/cargo-cargo-mutants/27.1.0/bin/cargo-mutants
env -u CARGO_MUTANTS_JOBS setsid nohup "$CM" mutants --package panini-prakriya --package panini-analyze \
  --test-workspace=true --timeout 60 -j 4 -o "$OUT" > "$OUT/campaign.log" 2>&1 < /dev/null &
date -u +"%F %T UTC" > "$OUT/started"
```

The last campaign took 12 minutes. Run nothing CPU-heavy meanwhile. Wait with a Monitor or ScheduleWakeup on `pgrep -x cargo-mutants`, never a foreground `sleep` loop.

- [ ] **Step 4: Read the outcomes**

When `pgrep -x cargo-mutants` returns nothing:

```bash
date -u +"%F %T UTC" > "$OUT/finished"
tail -5 "$OUT/campaign.log"
cat "$OUT/mutants.out/missed.txt" "$OUT/mutants.out/timeout.txt"
```

Expected (exit code 3 is normal when a timeout is present):
- `missed.txt` holds exactly `adesha.rs:588:30: replace + with *` and `tripadi.rs:1217:38: replace - with /`.
- `timeout.txt` holds exactly the permanent ṇatva `tripadi.rs:1543:23: replace -= with /=`.
- panini-analyze: 0 missed, 0 timeout.

If not:
- Any **other timeout** is a suspect survivor. Re-run it alone with its own `-o` and `--re` before concluding anything.
- Any **missed** mutant in `abhyasa.rs` means a Task 1 guard test does not separate it. Likeliest: 6.4.78's `||`, `!`, a deleted match arm, or `savarna.contains`; 7.4.77's `"03.0017"`. Strengthen the named test, commit, re-run only those mutants with `--re` and a fresh `-o`.

- [ ] **Step 5: Margins**

Inspect one record to see how `duration` is stored:

```bash
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["outcomes"][1])' "$OUT/mutants.out/outcomes.json"
```

Compute: the two equivalents' test phases under campaign load, the slowest caught phase, and caught min/median/p90/max. Margins are 60 ÷ the longest equivalent phase and 60 ÷ the slowest caught phase.
- If the first margin is ≥ 5, the cap stays 60.
- Otherwise the new cap is 6 × that phase, rounded up to the next 10 s. Change `mise.toml`'s `--timeout` and every AGENTS.md mention of the current cap together. A higher cap can only turn timeouts into outcomes, and the only timeout is the permanent hang, so the campaign's outcomes stand without a re-run.

- [ ] **Step 6: Record it in AGENTS.md**

- Rewrite `**The floor behind the 60s cap, measured at 4176 cells on 2026-09-30.**` with Step 1's and Step 2's numbers at 4212 cells, and the cap Step 5 chose.
- Replace the `**Current record (check-form-index, 2026-09-30).**` paragraph with `**Current record (juhotyādi 3d2, YYYY-MM-DD).**` in the same style: flags, `-o` path, window, **mutants / caught / unviable / missed / timeout** per package summing to the total, `missed.txt` and `timeout.txt` **named verbatim**, the non-caught set diffed against check-form-index's (the clean result is identical), campaign-load phases and margins, that no 6.4.78 / 7.4.60 / 7.4.77 mutant is missed, timed out or unviable-without-reason, and that `outcomes.json` is kept at `$OUT/mutants.out/outcomes.json`. End it with a pointer to the record it replaces: run `git rev-parse --short HEAD` before committing and write `The check-form-index record it replaces: \`git show <that hash>:AGENTS.md\`.`

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md mise.toml
git commit -m "chore: 3d2 mutation gate — floor and uncaught run re-measured at 4212 cells

missed.txt holds only the two documented equivalents and timeout.txt only
the permanent ṇatva-scan entry; every 6.4.78 clause is killed."
```

---

## Task 7: Finish the branch

- [ ] **Step 1: Confirm the gate is green**

```bash
mise run fmt-check && mise run lint && mise run test 2>&1 | tail -20
```

- [ ] **Step 2: Open the PR**

```bash
git push -u origin juhotyadi-3d2
gh pr create --title "juhotyādi 3d2 — √ṛ, the vowel-initial ṛ-root" --body "$(cat <<'BODY'
Slice 3d2 curates √ṛ (`03.0017 f\`, *iyarti* / *aiyaḥ*), taking the golden
suite from 4176 to 4212 cells and juhotyādi to seventeen of its twenty-six rows.

**New rule:** 6.4.78 *abhyāsasyāsavarṇe* — an abhyāsa-final i/u becomes
iyaṅ/uvaṅ before a dissimilar vowel (`i` → `iy`), in the abhyāsa stage after
7.4.78, so it precedes laṅ's 6.4.72/6.1.90 and 6.1.77 by construction.

**Widened:** 7.4.60 now trims a vowel-initial abhyāsa (`ar` → `a`; its
fall-through is gone), and 7.4.77 gains √ṛ's row.

**Checkpoint closed:** 6.4.71/6.4.72 keep reading `ANGA`. √ṛ is the
vowel-initial witness: both reads give āṭ, and 6.1.90 writes the vṛddhi into
the abhyāsa (`EyaH`), pinned by a live-row test.

No new vikalpa. Re-probed against vidyut and HEAD cell by cell before the
spec (0/36 on HEAD, one cause); audit at zero divergence against `8da2f90b`;
mutation gate clean.
BODY
)"
```

- [ ] **Step 3: Merge and clean up**

Per the standing instruction:
1. Once checks are green, `gh pr merge --merge --auto`.
2. After `git fetch origin`, `git branch -r --contains "$(git rev-parse HEAD)"` must list `origin/main`.
3. From `/workspace`: `git worktree remove .worktrees/juhotyadi-3d2`, then delete the local and remote branch, and `git pull` on `main`.

---

## Self-Review

**Spec coverage.**

| spec item | task |
|---|---|
| 7.4.60 fall-through retired; `ar`/`ap` → `a`; `f` no-op | 1 |
| 7.4.77 gains `03.0017`; comment table; decline-test comment | 1 |
| 6.4.78 after 7.4.78, sound-guarded, both *yvoḥ* arms, local savarṇa, synthetic killers | 1 |
| Stage-order row for `03.0017`; `tinanta_rule_order_is_pinned` | 1 |
| Row, counts, gaṇa-row test, 4 blocks, 5 `ALTERNATES`, buckets and keys | 2 |
| Spot check (*iyarti*, *EyaH*, *iyrati*) | 2 |
| 6.4.71/6.4.72 comments; live laṅ test; 6.1.90 test comment | 3 |
| Trace pins *iyarti*, *iyftaH*, *iyrati*, *EyaruH* | 4 |
| Audit with repoint and negative control; README/ARCHITECTURE/AGENTS; 3a-spec note; "3d2" promise sweep | 5 |
| Floor, uncaught probe, campaign, verbatim non-caught record | 6 |

**Type consistency.** `slu_prakriya(&str, &str)` and `run_abhyasa_stage(&str, &'static str)` are the existing helpers. Rule id `"6.4.78"` and name `"aByAsasyAsavarRe"` match across the rule, `p.record`, the order pin, the unit tests and the trace pins. `ABHYASA` is added to `anga.rs`'s test imports in Task 3 Step 1.

**Known soft spots.**
- The `threes` message string in `paradigm/main.rs` uses `\` continuations; edit it without breaking them.
- Task 3's test passes on first run by design: it pins a verdict, not new code.
