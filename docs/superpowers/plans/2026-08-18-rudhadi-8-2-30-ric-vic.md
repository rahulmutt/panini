# 8.2.30 *coḥ kuḥ* generalised, and √ric and √vic — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace 8.2.30's hardcoded `j` → `g` pair with a real cu → ku substitution table read on both the match and the substitute side, then curate the two roots that unlocks — √ric (`07.0004`) and √vic (`07.0005`).

**Architecture:** A new `kutva_of` joins the substitution tables in `panini-prakriya`'s `sound.rs`, beside `jashtva_of` and `cartva_of`, and 8.2.30 reads it on both sides so its match and its substitute cannot disagree. The two `Dhatu` rows then land, the whole corpus is re-audited against vidyut-prakriya, and the goldens are generated from the engine the audit certified. No new sūtra, no rule reordering.

**Tech Stack:** Rust 1.97.1 pinned via `mise`; `cargo test` for the golden suite; `cargo-mutants` for the mutation gate; `vidyut-prakriya` (external, via the committed harness at `tools/audit/`) for the cross-implementation audit.

**Spec:** `docs/superpowers/specs/2026-08-18-rudhadi-8-2-30-ric-vic-design.md`

**Branch:** `rudhadi-8-2-30-ric-vic` already exists, with the spec committed as `3c6fef2`. Work continues on it; do not branch again.

## Global Constraints

- **Toolchain is pinned.** Use `mise run <task>` or `mise exec -- cargo …`. Never install Rust globally. `mise run test -- -p X` does **not** scope to a package; use `mise exec -- cargo test -p X`.
- **Goldens are generated, never hand-authored.** Every `PARADIGM` block and `ALTERNATES` row in this plan comes out of a throwaway generator run against the engine the audit certified. Expected surfaces appearing in this document exist to make a wrong result recognisable; they are not to be typed into a test.
- **The audit's negative control runs first.** A zero-difference result recorded without a verified-failing control proves nothing.
- **Run the golden suite in the FOREGROUND.** It takes ~10 minutes. Do not background it and do not end a turn while it runs; a backgrounded suite gets orphaned and its result is lost.
- **`mise run mutants` is `-j 4 --timeout 2400`.** Run the task; do not reconstruct the flags. `cargo-mutants` reads `-j` from `CARGO_MUTANTS_JOBS`, so the environment can defeat the cap.
- **SLP1 throughout.** `c C j J` are the palatals (cu-varga); `k K g G` the velars (ku-varga).
- **No new sūtra.** No `Rule` added, removed or reordered; no change to `TINANTA_RULES` or its pinned order; no change to any guard other than 8.2.30's own. The vikalpa set stays at seven.

## Numbers this slice changes

Old values, for the arithmetic in Tasks 3 and 5. Every one of these is asserted somewhere and will fail loudly if missed.

| quantity | old | new |
|---|---|---|
| `dhatus().len()` | 53 | **55** |
| `PARADIGM.len()` (blocks) | 240 | **256** |
| cells (`PARADIGM.len() * 9`) | 2160 | **2304** |
| rudhādi curated roots | 11 | **13** |
| ubhayapadī curated roots | 7 | **9** |
| rudhādi entries still out | 14 | **12** |
| `ALTERNATES.len()` | 336 | measured in Task 5 |
| forms (cells + alternates) | 2496 | measured in Task 4 |

Cell-multiplicity buckets in `derivation_set_shape_matches_the_audited_numbers`, old values: ones **1922**, twos **166**, threes **61**, fours **1**, fives **5**, sixes **5**.

`ALTERNATES` key counts, old values: `8.4.56` **75**, `7.1.35` **66**, `7.1.35+8.4.56` **66**, `3.4.111` **2**, `6.4.107` **8**, `8.4.65` **93**, `8.2.75` **5**, `8.2.74` **1**, `7.1.35+8.4.65` **10**, `7.1.35+8.4.65+8.4.56` **10**.

## File Structure

| file | responsibility | task |
|---|---|---|
| `crates/panini-prakriya/src/tinanta/sound.rs` | gains `kutva_of` + `kutva_of_cu_all_arms` | 1 |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | 8.2.30's `apply`, its comment block, its import line, and its unit test | 2 |
| `crates/panini-data/src/lib.rs` | two `Dhatu` rows; `rudhadi_rows_…` renamed and extended; `dhatus().len()` | 3 |
| `tools/audit/panini_full_audit.rs` | corpus-total assertions | 3, 4 |
| `crates/panini/tests/print_ricvic_goldens.rs` | throwaway generator, created and deleted in Task 5 | 5 |
| `crates/panini/tests/paradigm.rs` | `PARADIGM`, `ALTERNATES`, the audited-numbers test, the ambiguity test | 5, 6 |
| `crates/panini/tests/trace.rs` | the ṇatva contrast pins and the one-step 8.2.30 pin | 7 |
| `AGENTS.md`, `README.md`, `docs/ARCHITECTURE.md`, `tools/audit/README.md` | prose, counts, recorded results | 8, 9 |

**Expected-red window.** Task 3 adds two `Dhatu` rows with no `PARADIGM` blocks behind them, so `paradigm_covers_every_enumerable_cell` **fails from Task 3 until Task 5**. This is intended and is how every prior gaṇa slice sequenced: the roots must be derivable before the audit can certify them, and nothing is pinned before the audit. Do not "fix" it by hand-authoring blocks.

---

### Task 1: `kutva_of` in `sound.rs`

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/sound.rs`

**Interfaces:**
- Consumes: nothing.
- Produces: `pub(crate) fn kutva_of(c: char) -> Option<char>` — returns the ku (velar) counterpart of a cu sound (`c→k`, `C→K`, `j→g`, `J→G`), `None` for every other char. Task 2 calls it on both the match and the substitute side of 8.2.30.

- [ ] **Step 1: Write the failing test**

Add to the `mod tests` block at the bottom of `crates/panini-prakriya/src/tinanta/sound.rs`, directly after `jashtva_of_stops_all_arms`:

```rust
    #[test]
    fn kutva_of_cu_all_arms() {
        // 8.2.30 coH kuH: pin every arm of the cu -> ku substitution table
        // directly. Only `j -> g` (√bhañj, √yuj) and `c -> k` (√ric, √vic)
        // are reachable from the golden forms, so a mutant rewriting the
        // aspirate arms would be invisible to the whole suite without this.
        // Mirrors jashtva_of_stops_all_arms above.
        //
        // 1.1.50 sthAne'ntaratamaH picks the NEAREST velar, so voicing and
        // aspiration carry across: voiceless unaspirated `c` goes to the
        // voiceless unaspirated `k`, never to `g`.
        assert_eq!(kutva_of('c'), Some('k'));
        assert_eq!(kutva_of('C'), Some('K'));
        assert_eq!(kutva_of('j'), Some('g'));
        assert_eq!(kutva_of('J'), Some('G'));

        // The velars are already ku and are not cu; the rule must not
        // re-fire on its own output.
        for c in ['k', 'K', 'g', 'G', 'N'] {
            assert_eq!(kutva_of(c), None, "{c} is ku already, not cu");
        }
        // The palatal nasal and sibilant are not cu for this rule's
        // purposes -- 8.2.30's `coH` names the stops.
        for c in ['Y', 'S'] {
            assert_eq!(kutva_of(c), None, "{c} is not a cu stop");
        }
        // Off-domain sanity: a vowel and a dental.
        for c in ['a', 't'] {
            assert_eq!(kutva_of(c), None, "{c} should not kutva");
        }
    }
```

- [ ] **Step 2: Run it and confirm it fails**

```bash
mise exec -- cargo test -p panini-prakriya kutva_of_cu_all_arms
```

Expected: FAIL — a compile error, `cannot find function 'kutva_of' in this scope`. A compile failure is the correct red state here; the function does not exist yet.

- [ ] **Step 3: Write the implementation**

Insert into `crates/panini-prakriya/src/tinanta/sound.rs` immediately after `jashtva_of` (so the substitution tables stay together, and before `parasavarna_of`):

```rust
/// The *ku* (velar) counterpart of a cu sound — 8.2.30 coH kuH's substitute.
/// By 1.1.50 sthAne'ntaratamaH the nearest substitute preserves voicing and
/// aspiration, so `c` goes to `k` and `j` to `g`, never both to one letter.
///
/// `C` and `J` have no curated witness — no aspirate-cu-final root is in
/// scope — and are present anyway because the table is a total function of
/// place, the same reason `jashtva_of` carries its 1.1.50-derived `z -> q`
/// arm. `kutva_of_cu_all_arms` is what keeps them from rotting.
///
/// The velars are deliberately absent rather than mapped to themselves: they
/// are already ku, not cu, and `None` is what lets 8.2.30 use this single
/// lookup as its match test as well as its substitute.
pub(crate) fn kutva_of(c: char) -> Option<char> {
    Some(match c {
        'c' => 'k',
        'C' => 'K',
        'j' => 'g',
        'J' => 'G',
        _ => return None,
    })
}
```

- [ ] **Step 4: Run the test and confirm it passes**

```bash
mise exec -- cargo test -p panini-prakriya kutva_of_cu_all_arms
```

Expected: PASS (1 passed).

- [ ] **Step 5: Confirm nothing else moved**

```bash
mise run fmt && mise exec -- cargo test -p panini-prakriya && mise run lint
```

Expected: PASS. `kutva_of` has no callers yet, so `dead_code` would normally fire — it does not, because the test in the same crate uses it. If clippy complains anyway, do **not** add `#[allow(dead_code)]`; go straight to Task 2, which adds the real caller.

- [ ] **Step 6: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/sound.rs
git commit -m "feat(sound): the cu -> ku table 8.2.30 has been describing but not using

1.1.50 sthAne'ntaratamaH preserves voicing and aspiration, so c -> k and
j -> g. The aspirate arms have no curated witness and are pinned directly
by kutva_of_cu_all_arms rather than through a derivation, the way
jashtva_of's z -> q arm already is."
```

---

### Task 2: 8.2.30 reads `kutva_of` on both sides

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` — the import list (line ~10), the comment block above 8.2.30 (lines ~261-303), the rule's `apply` (lines ~305-321), and `coh_kuh_fires_only_word_finally_or_before_a_jhal` (line ~1611)

**Interfaces:**
- Consumes: `kutva_of` from Task 1.
- Produces: an 8.2.30 that fires on any cu sound. No signature changes; `Rule` is unchanged in id, name, kind, vikalpa flag and array position.

**Note on scope:** the spec's changed-files list names 8.2.30's `apply` and its comment block. The unit test `coh_kuh_fires_only_word_finally_or_before_a_jhal` is a third site in the same file, and its doc comment explicitly says "Only the `j` -> `g` arm is reachable this slice, so this pins that guard rather than the wider cu/ku set" — that sentence is now false and is covered here.

- [ ] **Step 1: Extend the rule's unit test with a `c` case**

In `crates/panini-prakriya/src/tinanta/tripadi.rs`, replace the doc comment above `coh_kuh_fires_only_word_finally_or_before_a_jhal` with:

```rust
    /// 8.2.30 velarises a cu sound that is word-final or immediately followed
    /// by a jhal, and declines otherwise. Both reachable arms are pinned
    /// here: `j -> g` (√bhañj, √yuj) and `c -> k` (√ric, √vic). The `c` case
    /// is the one that distinguishes a real 1.1.50 substitution from the
    /// literal 'g' this rule used to write -- see `kutva_of`.
```

and add these two cases inside the test body, immediately before the "before a vowel" case:

```rust
        // a `c` takes the VOICELESS velar `k`, not `g`. This is the case the
        // old hardcoded substitute got wrong while still reaching the right
        // surface: 8.4.55 khari ca would have devoiced a spurious `g` to `k`
        // downstream, hiding the error from every paradigm golden.
        //
        // The `n` is still DENTAL here: √ric's ṇatva is 8.4.2's, which runs
        // later in the tripādī than 8.2.30, so `riRakti`'s retroflex has not
        // happened yet at this rule's turn. These fixtures are the real
        // intermediates, not the finished surfaces.
        let mut p = Prakriya {
            terms: vec![Term::new("ri"), Term::new("nac"), Term::new("ti")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "rinakti");

        // word-final `c`, same arm.
        let mut p = Prakriya {
            terms: vec![Term::new("ari"), Term::new("nac")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "arinak");
```

- [ ] **Step 2: Run it and confirm it fails**

```bash
mise exec -- cargo test -p panini-prakriya coh_kuh_fires_only_word_finally_or_before_a_jhal
```

Expected: FAIL, on the first new case — `assertion failed: (rule.apply)(&mut p)`. The rule's match still reads a literal `'j'`, so it declines on `c` entirely.

- [ ] **Step 3: Add `kutva_of` to the import**

In the `use crate::tinanta::sound::{…}` list at the top of `tripadi.rs`, add `kutva_of` in alphabetical position (after `is_vowel`, before `jashtva_of`).

- [ ] **Step 4: Rewrite the rule's `apply`**

Replace the body of the `Rule` with `id: "8.2.30"`:

```rust
        apply: |p| {
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
            true
        },
```

Leave `id`, `name`, `kind` and `vikalpa` exactly as they are, and leave the rule where it sits in the array — between 8.2.25 and 8.2.39.

The second `kutva_of` lookup is redundant with the predicate by construction. It is written as a fallible binding rather than `unwrap` because an `unwrap` here would be the only one in the file, and because a mutant that breaks the correspondence should decline rather than panic.

- [ ] **Step 5: Rewrite the comment block above the rule**

Replace the paragraph beginning `// NARROW GUARD, by design, as with 8.2.39 just below:` and ending `…8.4.55 khari ca devoicing it to `k` afterwards.` with:

```rust
    // The substitute is `kutva_of`, and so is the MATCH -- one lookup governs
    // both halves, so they cannot drift apart. That matters more than it
    // looks: the rule previously tested a literal `j` and wrote a literal
    // 'g', and widening only the match would have substituted `g` for a `c`
    // and still reached the right surface, 8.4.55 khari ca devoicing it to
    // `k` afterwards. Every paradigm golden would have passed. Do not
    // "simplify" `kutva_of` back into a hardcoded char: only
    // `rinakti_trace_reaches_k_in_one_step` in `crates/panini/tests/trace.rs`
    // can tell the two implementations apart.
    //
    // The 1.1.50 sthAne'ntaratamaH account above is therefore a description
    // of this code, not only of the sūtra.
```

Then correct the paragraph that reads `No cell in this suite has two `j`s to distinguish: …` so it speaks of cu sounds generally. Replace that sentence with:

```rust
    // No cell in this suite has two cu sounds to distinguish: √ric and √vic
    // each carry exactly one `c`, no curated root mixes a `c` with a `j`, and
    // the j-bearing roots that decline (√ji, √juṣ, √vij, and √bhañj's own 3pl
    // `Banjanti`) decline because their `j` precedes a vowel.
```

Leave the three paragraphs about `word_chars`, cross-term adjacency, and the test living inside the search untouched — they describe properties this task preserves deliberately.

- [ ] **Step 6: Run the unit test and confirm it passes**

```bash
mise exec -- cargo test -p panini-prakriya coh_kuh_fires_only_word_finally_or_before_a_jhal
```

Expected: PASS.

- [ ] **Step 7: Run the full workspace suite — this is the regression gate**

```bash
mise run fmt && mise run lint && mise run test
```

Expected: PASS, entirely green. This is the most important check in the task: √bhañj and √yuj exercise the `j` path across their pinned cells and must be **byte-identical**, and no curated root carries a `c` today, so a fully green suite means the widening changed nothing it should not have.

Takes ~10 minutes. **Run it in the foreground and wait.**

If anything fails, stop and report rather than adjusting goldens. A changed form here means the widening reached a site it should not have, which is a finding about the rule, not about the golden.

- [ ] **Step 8: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/tripadi.rs
git commit -m "fix(tripadi): 8.2.30 substitutes the nearest velar instead of a literal g

Match and substitute now read one kutva_of lookup, so they cannot disagree.
The j path is byte-identical -- √bhañj and √yuj are unchanged across every
pinned cell -- and no curated root carries a c yet, so this widens the rule
without moving a single golden."
```

---

### Task 3: The two data rows

**Files:**
- Modify: `crates/panini-data/src/lib.rs` — the `DHATUS` table, `rudhadi_rows_are_the_eleven_curated_roots`, the rudhādi deferral comment, `dhatus().len()`
- Modify: `tools/audit/panini_full_audit.rs` — the roots and cells assertions

**Interfaces:**
- Consumes: the generalised 8.2.30 from Task 2.
- Produces: `Dhatu` rows keyed `"07.0004"` (code `ric`) and `"07.0005"` (code `vic`), both `Gana::Rudhadi`, both `PadaAssignment::Ubhayapada`. Tasks 5, 6 and 7 address them by those numbers.

- [ ] **Step 1: Add the two rows**

In `crates/panini-data/src/lib.rs`, append to the `DHATUS` table immediately after the `07.0009` (√tṛd) row, keeping rudhādi's rows contiguous:

```rust
    Dhatu {
        // 07.0004 ri\ci~^r virecane. Ubhayapadī by 1.3.72 svaritaYitaH; the
        // `\` is the root vowel's own accent, not an it. THE FIRST `c` EVER
        // TO REACH 8.2.30 coH kuH: riRakti's stem-final `c` takes the
        // voiceless velar `k` directly, where √bhañj's and √yuj's `j` takes
        // `g` and needs 8.4.55 Kari ca to devoice it afterwards. That
        // one-step/two-step contrast is what pins the substitute as a real
        // 1.1.50 nearest-velar map rather than the literal 'g' it used to
        // be. Also an 8.4.2 awkupvAGnumvyavAye'pi witness: the root's `r`
        // retroflexes śnam's `n` across the intervening `i`.
        dhatupatha: "07.0004",
        code: "ric",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "virecane",
    },
    Dhatu {
        // 07.0005 vi\ci~^r pfTagBAve. Ubhayapadī by 1.3.72. The MINIMAL
        // CONTRAST to √ric: same gaṇa, same c-final shape, same vikaraṇa,
        // same 8.2.30 application -- and no ṇatva trigger at all, so
        // vinakti keeps its dental `n`. The pair isolates 8.4.2 against a
        // controlled background, the way 7c used √kṣud and √tṛd to separate
        // 8.4.2 from 8.4.1.
        dhatupatha: "07.0005",
        code: "vic",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "pfTagBAve",
    },
```

- [ ] **Step 2: Rename and extend the rudhādi rows test**

Rename `rudhadi_rows_are_the_eleven_curated_roots` to `rudhadi_rows_are_the_thirteen_curated_roots`, and append these two entries to the end of its expected `vec![…]`:

```rust
                ("07.0004", "ric", PadaAssignment::Ubhayapada),
                ("07.0005", "vic", PadaAssignment::Ubhayapada),
```

Extend that test's leading comment with a sentence naming the two roots and the reason they were out — 8.2.30's hardcoded pair, now generalised.

- [ ] **Step 3: Update the row count and the deferral comment**

Change `assert_eq!(dhatus().len(), 53);` to `55`.

In the rudhādi deferral comment (the block ending `…1.3.66 Bujo'navane forks its pada on sense.`), delete the √ric/√vic sentence and change `11 of its 25 dhātupāṭha roots, so FOURTEEN remain out` to `13 of its 25 dhātupāṭha roots, so TWELVE remain out`.

Also update the `Dhatu.pada` doc comment, which says "re-derives every one of these 53 verdicts" — it is 55 now.

- [ ] **Step 4: Update the audit harness's known totals**

In `tools/audit/panini_full_audit.rs`:

```rust
    assert_eq!(roots_seen.len(), 55, "curated roots");
    assert_eq!(n_cells, 2304, "cells: 256 root×pada×lakāra blocks × 9");
```

Leave the `n_forms` assertion at `2496` for now — the new figure is unknown until the audit runs, and Task 4 Step 4 sets it. Update the module header's "53 roots, 2160 cells, 2496 forms" line and its "the full 2160-cell table" mention to the new cell count in the same edit, leaving the form count for Task 4.

- [ ] **Step 5: Run the data crate's own tests**

```bash
mise run fmt && mise exec -- cargo test -p panini-data
```

Expected: PASS. Specifically these three must be green, and they are what makes the rows trustworthy without a hand-checked pada column:
- `rudhadi_rows_are_the_thirteen_curated_roots`
- `curated_pada_agrees_with_upadesha_markers` — re-derives both `Ubhayapada` verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78
- `dhatupatha_numbers_resolve_upstream` — it-strips `ri\ci~^r` and `vi\ci~^r` and checks they yield `ric` and `vic`

If `curated_pada_agrees_with_upadesha_markers` fails, stop: the pada column disagrees with the data that determines it, and that is a finding, not a value to adjust.

- [ ] **Step 6: Run the workspace suite and confirm the EXPECTED failure**

```bash
mise run test
```

Expected: **FAIL, and only in `paradigm_covers_every_enumerable_cell`**, reporting missing blocks for `07.0004` and `07.0005` across both padas and all four lakāras (16 blocks). This is the expected-red window described in File Structure above.

Any *other* failure is a real problem — in particular a failure in `roundtrip.rs` or in `derivation_set_is_exactly_pinned` means the new roots derive something the engine cannot re-analyse, which must be resolved before the audit runs.

Takes ~10 minutes. **Run it in the foreground.**

- [ ] **Step 7: Commit**

```bash
git add crates/panini-data/src/lib.rs tools/audit/panini_full_audit.rs
git commit -m "feat(data): ric and vic get their table rows

Both ubhayapadī by 1.3.72, both re-derived from the vendored upadeśa by
curated_pada_agrees_with_upadesha_markers rather than asserted. rudhādi 11
-> 13 roots, 53 -> 55 curated. paradigm_covers_every_enumerable_cell is
expected red until the audited goldens land."
```

---

### Task 4: The cross-implementation audit — the blocking gate

**Files:**
- Modify: `tools/audit/panini_full_audit.rs` (the `n_forms` assertion only)
- Read: `tools/audit/README.md`

**Interfaces:**
- Consumes: the rows from Task 3 and the rule change from Task 2.
- Produces: a recorded zero-difference verdict (or a sourced difference), and the corpus **forms** total that Task 5 cross-checks its pasted `ALTERNATES` against.

**This task blocks Task 5.** No golden is pinned until the audit certifies the derivations it would pin.

- [ ] **Step 1: Find the vendored commit and clone vidyut there**

```bash
head -20 /workspace/data/dhatupatha.tsv | grep -i commit
```

Read the commit from that header rather than trusting any README's copy of it, then:

```bash
cd /tmp && git clone --filter=blob:none https://github.com/ambuda-org/vidyut vidyut-full
cd /tmp/vidyut-full && git checkout <the commit from that header>
```

- [ ] **Step 2: Wire this repo's crates in and copy the harness**

Append to `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` under `[dev-dependencies]`:

```toml
panini = { path = "/workspace/crates/panini" }
panini-data = { path = "/workspace/crates/panini-data" }
```

Then:

```bash
cp /workspace/tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
```

**Copy the committed harness; do not rewrite it.** It encodes decisions that took three slices to get right — number-keyed entry selection with no fallback, derivation-set comparison rather than single-form, and blocked-prakriyā filtering.

- [ ] **Step 3: Run the negative control FIRST**

```bash
cd /tmp/vidyut-full/vidyut-prakriya
PANINI_AUDIT_PERTURB=entry mise exec rust@1.97.1 -- cargo run --release --example panini_full_audit
```

Expected: **exit 1**, printing 36 differing cells for √bhū — `Bavati` vs `paWati` and so on. This resolves `01.0001` against `01.0381` (√paṭh), a plausible wrong entry, so a control that passes here would mean the harness cannot see a difference at all.

If this does not fail, stop. Every result after it is worthless.

- [ ] **Step 4: Run the real audit and set the form total**

```bash
cd /tmp/vidyut-full/vidyut-prakriya
mise exec rust@1.97.1 -- cargo run --release --example panini_full_audit 2>&1 | tee /tmp/ricvic-audit.txt
```

The harness prints the corpus block before it asserts, so the first run tells you the form count even though it then panics on the stale `2496`:

```
=== corpus ===
roots            : 55
cells            : 2304
forms (set sizes): <N>
```

Take that `<N>`, and in **both** `/workspace/tools/audit/panini_full_audit.rs` and the copy in the vidyut checkout, set:

```rust
    assert_eq!(n_forms, <N>, "forms: 2304 cells + <N - 2304> ALTERNATES rows");
```

Update the module header's "53 roots, 2160 cells, 2496 forms" line to `55 roots, 2304 cells, <N> forms` at the same time. Then re-run the command above.

Expected on the re-run: `AUDIT PASSED: 2304 cells, <N> forms, zero differences.`

- [ ] **Step 5: If there ARE differences**

Do not expand the slice. The spec fixes the posture in advance: **ship what passes, defer the rest with a sourced note.**

- Record which root, which cells, and both engines' forms, from the `DIFF` lines.
- If one root is clean and the other is not, keep the clean one and drop the other's row (reverting that part of Task 3), then carry it as a deferral naming the actual sūtra at fault — measured, not guessed — in the same form the 6.1.73 / 8.4.40 deferral takes today.
- The rule change lands either way; it is correct on its own terms and Task 2 proved it moves nothing on the `j` path.
- Every count in this plan then reflects one root, not two. Recompute rather than carrying these numbers forward, and say so in the prose: a partial slice states its own partiality.

Stop and report before continuing to Task 5 in this case.

- [ ] **Step 6: Record the result and commit**

Update `tools/audit/README.md`'s "Last recorded result" section, replacing it with this slice's run — the date, the branch's slice name, the vidyut commit, the cell/form/root totals, and the fact that the `entry` control was verified failing first. Keep the shape of the existing entry.

```bash
cd /workspace
git add tools/audit/panini_full_audit.rs tools/audit/README.md
git commit -m "test(audit): ric and vic are byte-identical to vidyut

Whole corpus, 55 roots / 2304 cells / <N> forms, zero differences at vidyut
<commit>, with the entry negative control verified failing first (exit 1, 36
√bhū cells). The generalised 8.2.30 is sufficient for both roots; no further
sūtra is needed."
```

---

### Task 5: The goldens, generated

**Files:**
- Create then delete: `crates/panini/tests/print_ricvic_goldens.rs`
- Modify: `crates/panini/tests/paradigm.rs` (`PARADIGM`, `ALTERNATES`, `derivation_set_shape_matches_the_audited_numbers` and its doc comment)

**Interfaces:**
- Consumes: the two `Dhatu` rows (Task 3); the audit verdict and the form total `<N>` (Task 4).
- Produces: 16 `PARADIGM` blocks and their `ALTERNATES` rows; the measured multiplicity distribution and per-key counts that Task 9's prose quotes.

- [ ] **Step 1: Write the throwaway generator**

Create `crates/panini/tests/print_ricvic_goldens.rs`:

```rust
//! THROWAWAY — this slice only. Prints the two new roots' `PARADIGM` blocks
//! and `ALTERNATES` rows as Rust source, plus the distribution counts
//! `derivation_set_shape_matches_the_audited_numbers` asserts. Deleted in
//! the same task that pastes its output: goldens are generated from the
//! engine the audit certified, never hand-authored.
//!
//! Run with:
//!   mise exec -- cargo test -p panini --test print_ricvic_goldens -- --nocapture

mod common;

use common::{CELLS, LAKARA_BY_NAME};
use panini_data::{Pada, dhatus};
use panini_prakriya::derive;

const NEW_ROOTS: [&str; 2] = ["07.0004", "07.0005"];

/// Mirrors `VIKALPA_RULES` in `paradigm.rs`. An alternate's key is the
/// `+`-joined list of optional rules its branch actually applied, which is
/// what `every_alternate_names_the_vikalpa_rules_that_produced_it` checks.
const VIKALPA_RULES: &[&str] = &[
    "7.1.35", "3.4.111", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56",
];

#[test]
fn print_ricvic_goldens() {
    // (rendered row, key) — the key is kept alongside rather than parsed
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
                    // Index 0 is the declined derivation — the one with no
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

- [ ] **Step 2: Run the generator**

```bash
mise exec -- cargo test -p panini --test print_ricvic_goldens -- --nocapture 2>&1 | tee /tmp/ricvic-goldens.txt
```

Expected: PASS, printing 16 `PARADIGM` block lines, the `ALTERNATES` rows, the multiplicity distribution over **144** new cells, and per-key counts.

Sanity-check two strings against the spec's expectations before trusting the rest — these are the laṭ prathama eka **parasmaipada** cells: `riRakti` (`07.0004`) and `vinakti` (`07.0005`). If either differs, stop: Task 4 passed, so a mismatch here means the generator addresses cells wrongly, not that the engine is wrong.

- [ ] **Step 3: Paste the blocks and rows into `paradigm.rs`**

Append the 16 printed `PARADIGM` block lines to the end of the `PARADIGM` array (before its closing `];`), and the printed `ALTERNATES` rows to the end of the `ALTERNATES` array. Paste them verbatim from `/tmp/ricvic-goldens.txt`; do not retype. Formatting is `rustfmt`'s job.

- [ ] **Step 4: Update the audited-numbers assertions**

In `derivation_set_shape_matches_the_audited_numbers`:

```rust
    assert_eq!(total_cells, 2304, "256 root×lakāra blocks × 9 cells each");
```

Then for each multiplicity bucket, `new = old + <that bucket from Step 2>`, using the old values in "Numbers this slice changes" (ones 1922, twos 166, threes 61, fours 1, fives 5, sixes 5). Likewise `ALTERNATES.len()` becomes `336 + <ALTERNATES row count from Step 2>`, and each `key_count(...)` becomes its old value plus the generator's `key <k>` count. Keys the generator does not list are unchanged.

Update the `fives` and `sixes` assertion messages only if the generator actually reports new cells in those buckets. Based on √yuj — also ubhayapadī, also velar-against-dental at the junction, so 8.4.65 never applies — √ric and √vic are **not** expected to reach five or six forms. If they do, that is a real finding: say so in the message and note it for Task 9, because `README.md` and `docs/ARCHITECTURE.md` both name the record-holding cells.

**Cross-check before moving on:** `2304 + ALTERNATES.len()` must equal the `<N>` forms figure from Task 4 Step 4. If it does not, the goldens and the audit disagree about the corpus and something was pasted wrong. Resolve it here, not later.

If the generator reports a bucket the test has no arm for (a 7-form cell), stop and report it: that would be a sharper fork than anything in the repo and needs discussion, not a silently added `assert_eq!`.

- [ ] **Step 5: Update the doc comment above that test**

That comment enumerates which roots fork where and carries the audited numbers. Correct `2160 cells total (240 root×lakāra blocks × 9)` and the bucket figures to the new values, correct `ALTERNATES itself has 336 rows` and its key list, and extend the audit-provenance sentence with this slice's run (vidyut commit, 2304 cells, `<N>` forms, 55 roots, negative control verified).

Note the comment already contains a passage about √yuj taking no 8.4.65 branch because 8.2.30 gives it a velar. √ric and √vic are the same shape for the same reason — extend that passage rather than writing a competing one.

- [ ] **Step 6: Delete the generator**

```bash
rm crates/panini/tests/print_ricvic_goldens.rs
```

It is throwaway by construction: it duplicates `VIKALPA_RULES`, and a second copy that can drift is exactly what this repo deletes rather than maintains.

- [ ] **Step 7: Run the full workspace suite**

```bash
mise run fmt && mise run test
```

Expected: PASS, fully green — the expected-red window closes here. These five are the tests that judge the goldens:
- `paradigm_covers_every_enumerable_cell` — green again; every (root, lakāra, pada) triple is pinned
- `derivation_set_is_exactly_pinned` — each cell's derivation set must be **exactly** golden + alternates
- `every_alternate_names_the_vikalpa_rules_that_produced_it`
- `every_alternate_names_a_real_paradigm_block`
- `derivation_set_shape_matches_the_audited_numbers`

Takes ~10 minutes. **Foreground.**

- [ ] **Step 8: Commit**

```bash
git add crates/panini/tests/paradigm.rs
git commit -m "test(paradigm): ric and vic get their audited paradigms

16 blocks, 144 cells, generated from the engine the vidyut audit certified
in the previous commit rather than hand-authored. PARADIGM 240 -> 256
blocks, 2160 -> 2304 cells."
```

---

### Task 6: The cross-pada ambiguity test

**Files:**
- Modify: `crates/panini/tests/paradigm.rs` (`pada_ambiguous_surfaces_are_exactly_these`, line ~4844)

**Interfaces:**
- Consumes: the 16 new `PARADIGM` blocks (Task 5).
- Produces: the measured list of ambiguous surfaces that Task 9 copies into `README.md`.

Two new ubhayapadī roots mean new surfaces that are pinned cells in **both** padas at once. The test computes the set itself but asserts against a hardcoded list, so the list must be re-measured — never hand-extended.

- [ ] **Step 1: Blank the expected list to measure the real one**

Temporarily replace the `assert_eq!(both, vec![…])` expectation with `Vec::<&str>::new()`, then:

```bash
mise exec -- cargo test -p panini --test paradigm pada_ambiguous_surfaces_are_exactly_these
```

Expected: FAIL, printing the full measured set on the left. This is the documented way this list is produced — the existing comment says so explicitly.

- [ ] **Step 2: Paste the measured set back**

Restore the assertion with the measured list, verbatim from the failure output.

**Check before accepting it:** all eighteen pre-slice surfaces must still be present. If any disappeared, this slice disturbed an existing root's paradigm and that is a regression, not a list update — stop and report.

- [ ] **Step 3: Extend the comment**

Add a sentence naming the new roots' contributions, in the style of the existing 7c sentence — which surfaces, which root, which number. Keep the existing explanation of how the list is measured; it is what stops the next contributor hand-picking it.

- [ ] **Step 4: Run it and confirm it passes**

```bash
mise run fmt && mise exec -- cargo test -p panini --test paradigm
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/panini/tests/paradigm.rs
git commit -m "test(paradigm): the new ubhayapadī surfaces join the ambiguity set

Measured off the assertion against an empty vec, not hand-extended. All
eighteen pre-slice surfaces are still present, so nothing was disturbed."
```

---

### Task 7: The trace pins

**Files:**
- Modify: `crates/panini/tests/trace.rs`

**Interfaces:**
- Consumes: the audited goldens (Task 5); `cell_trace`, `trace_for` and `at`, already in the file.
- Produces: `rinakti_trace_reaches_k_in_one_step`, named by the comment Task 2 wrote into `tripadi.rs`.

These three tests are the slice's load-bearing assertions. Under a lazy fix — match widened, substitute left literal — **every `PARADIGM` cell in Task 5 still passes**, because 8.4.55 launders the wrong `g` into the right `k`. The trace does not launder.

- [ ] **Step 1: Write the three tests**

Append to `crates/panini/tests/trace.rs`:

```rust
#[test]
fn rinakti_trace_reaches_k_in_one_step() {
    // THE pin that distinguishes a correct 8.2.30 from an accidentally
    // correct one. √bhañj's `j` takes the VOICED velar `g` and needs 8.4.55
    // Kari ca to devoice it afterwards -- that two-step path is pinned by
    // bhanakti_trace_shows_8_2_30_then_8_4_55 above. √ric's `c` is already
    // voiceless, so 1.1.50's nearest velar IS `k` and 8.2.30 reaches it in
    // one step, leaving 8.4.55 nothing to do.
    //
    // A substitute hardcoded to 'g' would produce riRagti here and let
    // 8.4.55 devoice it to the same riRakti surface. Every paradigm golden
    // would still pass; only this test fails. Do not weaken it to a
    // presence check on 8.2.30.
    let (text, t) = cell_trace(
        "07.0004",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "riRakti", "got {t:?}");
    assert!(t.contains(&"8.2.30".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.55".to_string()), "8.4.55 must not fire: {t:?}");
}

#[test]
fn rinakti_trace_takes_intervening_natva() {
    // r, then the aw vowel i, then śnam's n -> 8.4.2, not 8.4.1. Structurally
    // √rudh's arm (ruRadDi) reached through an `r` trigger, inside the one
    // gaṇa where 8.3.24 naScApadAntasya Jali competes on the weak stem.
    let (text, t) = cell_trace(
        "07.0004",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "riRakti", "got {t:?}");
    assert!(t.contains(&"8.4.2".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.1".to_string()), "got {t:?}");
}

#[test]
fn vinakti_trace_takes_no_natva_at_all() {
    // The minimal contrast to √ric: same gaṇa, same c-final shape, same
    // vikaraṇa, same 8.2.30 application -- and no r/z/f trigger anywhere, so
    // śnam's `n` stays dental. A NEGATIVE pin, and the point of the pair:
    // it is what stops a widened ṇatva guard passing unnoticed.
    let (text, t) = cell_trace(
        "07.0005",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "vinakti", "got {t:?}");
    assert!(t.contains(&"8.2.30".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.1".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.2".to_string()), "got {t:?}");
}
```

`cell_trace` addresses a cell by coordinates rather than by surface, which matters here: both roots are ubhayapadī and `trace_for` resolves a *word*, which can be ambiguous across padas.

- [ ] **Step 2: Reconcile the expected surfaces against the pinned goldens**

The two `assert_eq!(text, …)` strings are the spec's *expectations*. Open the `07.0004` and `07.0005` laṭ `Pada::Parasmaipada` blocks in `PARADIGM` and confirm cell 0 of each matches. **If a golden differs, the golden wins** — the audit certified it and this document did not. Correct the test and note the divergence for Task 9's prose.

- [ ] **Step 3: Run them**

```bash
mise exec -- cargo test -p panini --test trace
```

Expected: PASS, all three plus the existing pins.

If `rinakti_trace_reaches_k_in_one_step` fails on the `8.4.55` assertion, there are two possible causes and they need telling apart before anything is edited:

1. **8.2.30 wrote a `g`.** The substitute is not being read from `kutva_of`, and 8.4.55 then fired to devoice it. This is the test detecting exactly what it exists to detect — fix Task 2, do not relax the assertion.
2. **8.4.55 recorded vacuously.** The rule has a no-op guard (see the "No-op guard" comment in `tripadi.rs`), so it is expected to decline outright on an unchanged `k` — but if the run shows it recording without changing the text, that is a fact about 8.4.55, not a defect in this slice. In that case keep the one-step property and assert it directly instead: that the text after 8.2.30 already ends in `k`, and that 8.4.55 left it unchanged.

Distinguish them by reading the trace and the derivation's own log rather than by guessing. Cause 1 shows a `g` in the intermediate; cause 2 does not.

- [ ] **Step 4: Commit**

```bash
git add crates/panini/tests/trace.rs
git commit -m "test(trace): the c arm reaches k in one step, and the natva pair

riRakti pins 8.2.30 substituting a voiceless velar directly, with 8.4.55
declining -- the only assertion in the suite that a hardcoded 'g' substitute
would fail, since 8.4.55 would otherwise launder it to the same surface.
vinakti is the no-trigger contrast that isolates 8.4.2."
```

---

### Task 8: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the cargo-mutants paragraph)

**Interfaces:**
- Consumes: everything above.
- Produces: the measured floor and campaign figures Task 9's prose and `mise.toml`'s comment reference.

- [ ] **Step 1: Measure the uncontended floor**

```bash
time mise run test 2>&1 | tee /tmp/ricvic-floor.txt
```

Run it alone — no mutation campaign, no other load. Record the per-binary times (`paradigm`, `roundtrip`, `trace`) and their total.

**Measure; do not scale.** Cell count has failed as a multiplier in both directions — flat from 1800 to 1872 cells, then +38% for +15% growth into 7c's 610.73s (paradigm 276.99s, roundtrip 331.81s, trace 1.93s). This slice adds 144 cells (+6.7%); the floor may not move proportionally.

- [ ] **Step 2: Sanity-check the cap against the floor**

The `-j 4` contention factor measured by the pada audit is **1.70×**. Multiply Step 1's total by it: that is the projected worst case for an **uncaught** mutant, which is the figure that governs whether a "0 missed" is vacuous — a genuine survivor runs the suite to completion, whereas a caught run can abort the moment it is detected.

If that projection exceeds **2400s**, stop and report before running the campaign: the cap needs raising, and that is a decision to record in `AGENTS.md`, not to make silently. At 7c's numbers the projection was ~1040s (a 2.31× margin), so a 6.7% growth should leave it comfortable.

- [ ] **Step 3: Run the campaign**

```bash
cd /workspace && mise run mutants
```

This runs `cargo mutants --package panini-prakriya --test-workspace=true --timeout 2400 -j 4`. It takes hours. **Foreground; do not background it.**

If the `cargo mutants` shim misbehaves, invoke the binary directly rather than through the shim — but keep the same flags.

- [ ] **Step 4: Check BOTH `missed.txt` and `timeout.txt`**

```bash
cat mutants.out/missed.txt; echo "--- timeouts ---"; cat mutants.out/timeout.txt; wc -l mutants.out/*.txt
```

Expected:
- `missed.txt` — **empty**. Any entry is a real survivor and must be resolved, not accepted.
- `timeout.txt` — **exactly one entry**, and it must be the known-permanent one: the ṇatva backward-scan mutant in `tripadi.rs` that turns `j -= 1` into `j /= 1`, making the loop non-terminating. No assertion can ever catch it — the mutated run never reaches one — so the cap itself is the detection mechanism and this is the correct verdict at any cap.

**Identify it by that shape, not by its line number.** `AGENTS.md` records it as `tripadi.rs:1140:23`, and this slice added lines to `tripadi.rs` above it, so the number **will** have moved. A shifted line number is not a new timeout. Confirm by reading the mutant's diff:

```bash
grep -n "j /= 1" mutants.out/timeout.txt mutants.out/mutants.json
```

Any *other* timeout must be re-run alone at the same cap before any conclusion is drawn — under contention a real survivor can be misreported as a timeout, which is what makes a careless "0 missed" vacuous.

Expect roughly **530 mutants** (up from 522): `kutva_of` contributes two function-replacement and four arm-deletion mutants, all killed by `kutva_of_cu_all_arms` against the ~2s baseline suite rather than the ~10min golden suite.

- [ ] **Step 5: Extract the duration distribution**

```bash
cd /workspace && python3 - <<'PY'
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

The over-600s count is the number to watch across slices — it went 4 → 44 between the pada audit and 7c while the max barely moved.

- [ ] **Step 6: Record it in `AGENTS.md`**

Add a paragraph to the cargo-mutants section in the same shape as the existing "Slice 7c re-measured both at 2160 cells" one: the new cell count, the per-binary floor and its total, how it compared to what scaling would have predicted, the campaign's mutant/caught/missed/unviable/timeout tallies, the duration distribution, and both margins — the measured one against the worst caught mutant, and the projected one against the uncaught floor, each labelled as measured or projected.

Do not delete the 7c paragraph. The series is the evidence that cell count is not a multiplier.

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md
git commit -m "test: mutation gate at 2304 cells, with the floor re-measured

Campaign at -j 4 --timeout 2400, 0 missed, and the one known-permanent
tripadi.rs non-terminating-loop timeout whose line number moved with this
slice's edits. The floor was measured, not scaled from the cell count."
```

---

### Task 9: The documentation sweep

**Files:**
- Modify: `README.md`, `docs/ARCHITECTURE.md`, `AGENTS.md`, `crates/panini-data/src/lib.rs`, `crates/panini/tests/paradigm.rs`, `data/ATTRIBUTION.md` (conditionally)

**Interfaces:**
- Consumes: every measured number from Tasks 4-8.
- Produces: a repo whose prose matches its tests.

Six sites state the √ric/√vic deferral or the corpus totals. Treat this as a checklist, not a sweep — past slices have shipped with counts stale in one file. Several were already touched in earlier tasks; the boxes below are what remains.

- [ ] **Step 1: `README.md`**

- rudhādi "eleven of its roots … of 25" → thirteen, adding √ric and √vic to the curated list with their slice attribution
- "**14 of the 25 remain out**" → **12**
- Delete the √ric/√vic deferral sentence — the 8.2.30 clause about the hardcoded `j` → `g` pair and its substitute. It is done.
- "over a curated 53-root set" → 55
- cell and form totals → 2304 and `<N>`
- the ubhayapadī list (√nī, √tud, √rudh, √bhid, √kṣud, √yuj, √tṛd) → add √ric and √vic; "all seven ubhayapadī roots" → nine
- the multi-form cell distribution sentence, from Task 5's measured buckets
- the pada-ambiguous surface enumeration ("Eighteen surfaces are…") → the measured list from Task 6, with its new count

- [ ] **Step 2: `docs/ARCHITECTURE.md`**

- the rudhādi paragraph: "carries eleven roots" → thirteen, naming √ric and √vic
- "nine of rudhādi's 25 dhātupāṭha roots are ubhayapadī, five of the nine now curated" → **seven** of the nine
- **rewrite the whole √ric/√vic passage.** It currently reads as a pending caveat — "they therefore need no new sūtra but more than a widened guard: the substitute has to be generalised alongside the match" — and must become a record of what was done and why the substitute is a map.
- "**14 of the 25 in all**" → 12
- if Task 5 found a new deepest fork, the "sharpest branch-count witnesses" sentence

- [ ] **Step 3: `AGENTS.md`**

- the rudhādi section (~line 220-260): the root list and count, and the √ric/√vic paragraph, which must stop describing pending work
- suite-size figures (~line 165, 173): 2160 → 2304, 336 → the new `ALTERNATES` count, 2496 → `<N>`
- the recorded audit results (~line 249, ~373): add this slice's run
- the mutation paragraph is already done (Task 8)

- [ ] **Step 4: `crates/panini-data/src/lib.rs` and `paradigm.rs`**

Already updated in Tasks 3 and 5. Re-read both to confirm no count was missed — in particular `panini-data`'s comment about `07.0013`/`07.0023` colliding under the retired `id` scheme, which sits inside the block Task 3 edited.

- [ ] **Step 5: `data/ATTRIBUTION.md` — conditional**

That file records per-entry discrepancies against upstream for `07.0010`, `07.0019` and `07.0012`. Check whether `07.0004` and `07.0005` need an entry:

```bash
grep -E '^07\.000[45]' /workspace/data/dhatupatha.tsv
```

If the vendored upadeśa for each matches upstream unmodified and `code` is a plain it-strip of it (which `dhatupatha_numbers_resolve_upstream` already verified in Task 3 Step 5), **no entry is needed** — that file records deviations, not routine rows. Add nothing in that case.

- [ ] **Step 6: Verify the prose against the tests**

```bash
cd /workspace && grep -rn "2160\|2496\|53 roots\|eleven\|14 of the 25\|FOURTEEN" \
  README.md docs/ARCHITECTURE.md AGENTS.md tools/audit/README.md \
  crates/panini-data/src/lib.rs crates/panini/tests/paradigm.rs
```

Expected: every remaining hit is a deliberate historical reference (a past slice's recorded measurement, e.g. 7c's floor paragraph), not a current claim. Anything stating a present-tense fact must have been updated.

- [ ] **Step 7: Full suite, lint, format**

```bash
mise run fmt-check && mise run lint && mise run test && mise run audit
```

Expected: all PASS. **Foreground.**

- [ ] **Step 8: Commit**

```bash
git add -A
git commit -m "docs: rudhadi carries thirteen roots, and 8.2.30 is no longer a caveat

The ric/vic deferral is discharged in all six places that stated it, and the
counts reach every file: 55 roots, 2304 cells, and the ubhayapadī list grows
to nine."
```

---

### Task 10: Push, PR, and finish the branch

**Files:** none.

- [ ] **Step 1: Confirm the tree is clean and the branch is coherent**

```bash
cd /workspace && git status --short && git log --oneline main..HEAD
```

Expected: clean tree, and a commit series running spec → `kutva_of` → 8.2.30 → rows → audit → goldens → ambiguity → traces → mutants → docs.

- [ ] **Step 2: Push and open the PR**

```bash
git push -u origin rudhadi-8-2-30-ric-vic
gh pr create --fill
```

The PR body should state the audit's verdict (vidyut commit, 55 roots / 2304 cells / `<N>` forms, zero differences, negative control verified), the mutation result (mutants, caught, 0 missed, the one known-permanent timeout), and the one-sentence version of what the slice fixed: 8.2.30's substitute was a literal `'g'` that reached the right surface for `j` and would have silently corrupted the intermediate for `c`.

- [ ] **Step 3: Finish the branch**

Use the `superpowers:finishing-a-development-branch` skill: wait for CI, merge the green PR, verify the commits are on `main`, then delete the branch.

---

## Deferred, and why

Carried forward unchanged. None of these is blocked by anything this slice did.

- **6.1.73 *che ca* and 8.4.40 *stoḥ ścunā ścuḥ***, and with them √chid and √chṛd. Two sūtras the engine does not implement, not a generalisation of one it does.
- **The nine reachable non-ubhayapadī rudhādi roots** — √śiṣ, √tṛh, √und, √añj, √tañc, √vij, √vṛj, √pṛc, √vid.
- **√bhuj** (`07.0017`), whose 1.3.66 *bhujo'navane* forks its pada on sense rather than on an axis this engine models.
- **It-stripping as real sūtras** (1.3.2, 1.3.3, 1.3.5, 1.3.9). Two more curated rows make the case slightly stronger without pre-empting the design.
- **8.2.39's own narrow guard**, and every other narrow guard in `tripadi.rs`. 8.2.30 was generalised here because this slice's roots demanded it, not because narrow guards are being retired as a class.
