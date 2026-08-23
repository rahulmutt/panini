# rudhādi gaṇa slice 7e — √tṛh and its six rule changes — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate √tṛh (`07.0018`), the last reachable non-ubhayapadī rudhādi root, taking the gaṇa from 21 to 22 of its 25 dhātupāṭha roots.

**Architecture:** Six rule changes, in two groups with a hard gate between them. First the **three widenings** of rules the engine already has — 8.4.41's `z`-only trigger, 8.2.41's missing ḍh arm, 6.1.87's junction-only arm — verified inert against the *existing* 2592-cell corpus by a byte-for-byte dump diff before a single new row lands. Then the **three new sūtras** — 7.3.92 *tṛṇaha im*, 8.2.31 *ho ḍhaḥ*, 8.3.13 *ḍho ḍhe lopaḥ* — the root's data row, and goldens generated from the engine the cross-implementation audit certified. Because the widenings are proven inert first, a difference the audit finds after √tṛh arrives has exactly one candidate cause.

**Tech Stack:** Rust 1.98.0 pinned via `mise`; `cargo test` for the golden suite; `cargo-mutants` for the mutation gate; `vidyut-prakriya` (external, via the committed harness at `tools/audit/`) for the cross-implementation audit.

**Spec:** `docs/superpowers/specs/2026-08-23-rudhadi-gana-7e-design.md`

**Branch:** `rudhadi-gana-7e` already exists, with the spec committed as `a581504`. Work continues on it; do not branch again. `main` is at `2743fae`, unchanged.

## Global Constraints

- **Toolchain is pinned to rust 1.98.0** via `mise.toml`. Use `mise run <task>` or `mise exec -- cargo …`; never install Rust globally. `mise run test -- -p X` does **not** scope to a package — use `mise exec -- cargo test -p X`.
- **Run the golden suite in the FOREGROUND.** It takes ~12 minutes and grows in Task 7. Do not background it and do not end a turn while it runs; a backgrounded suite gets orphaned and its result is lost.
- **`mise run mutants` is `-j 4 --timeout 2400`.** Run the task; do not reconstruct the flags. `cargo-mutants` reads `-j` from `CARGO_MUTANTS_JOBS`, so the environment can defeat the cap.
- **Goldens are generated, never hand-authored.** Every `PARADIGM` block and `ALTERNATES` row comes out of the throwaway generator in Task 7, run against the engine the audit certified. Surfaces quoted in this document exist to make a wrong result recognisable; they are **not** to be typed into a test.
- **The audit's negative control runs first.** A zero-difference result recorded without a verified-failing control proves nothing.
- **`tools/audit/panini_full_audit.rs` is copied, never rewritten.** It is committed precisely so no slice reconstructs it.
- **SLP1 throughout.** `M` is anusvāra, `N` velar ṅ, `Y` palatal ñ, `R` retroflex ṇ, `z` retroflex ṣ, `q`/`Q` retroflex ḍ/ḍh, `f` vocalic ṛ.
- **The tag is `Tag::Ngit`, not `Tag::Nit`.** The spec's prose says "Nit"; the code name is `Ngit`.

## Numbers this slice changes

Old values, for the arithmetic in Tasks 3, 6 and 7. Every one is asserted somewhere and will fail loudly if missed.

| quantity | old | new |
|---|---|---|
| `dhatus().len()` | 63 | **64** |
| `PARADIGM.len()` (blocks) | 288 | **292** |
| cells (`PARADIGM.len() * 9`) | 2592 | **2628** |
| rudhādi curated roots | 21 | **22** |
| rudhādi entries still out | 4 | **3** |
| ubhayapadī curated roots | 9 | **9 (unchanged)** |
| `ALTERNATES.len()` | 422 | measured in Task 7 |
| forms (cells + alternates) | 3014 | measured in Task 6 |

Cell-multiplicity buckets in `derivation_set_shape_matches_the_audited_numbers`, old values: ones **2293**, twos **208**, threes **77**, fours **2**, fives **6**, sixes **6**.

`ALTERNATES` key counts, old values: `8.4.56` **102**, `7.1.35` **84**, `7.1.35+8.4.56` **84**, `3.4.111` **2**, `6.4.107` **8**, `8.4.65` **111**, `8.2.75` **6**, `8.2.74` **1**, `7.1.35+8.4.65` **12**, `7.1.35+8.4.65+8.4.56` **12**.

The spec projects 429 `ALTERNATES` rows and 3057 forms from a probe, with the new rows landing entirely in `8.4.56` (+3), `7.1.35` (+2) and `7.1.35+8.4.56` (+2). Those are **expectations to recognise a wrong result by**, not values to type in. Tasks 6 and 7 measure them.

## What √tṛh's paradigm looks like

Probed against vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` during design. Parasmaipada only — vidyut derives no ātmanepada forms for `tfha~`, so 36 cells, not 72. **Do not type these into a test**; they exist so a wrong generator output is recognisable at a glance.

| lakāra | the nine cells |
|---|---|
| laṭ | `tfReQi` `tfRQaH` `tfMhanti` `tfRekzi` `tfRQaH` `tfRQa` `tfRehmi` `tfMhvaH` `tfMhmaH` |
| laṅ | `atfReq`/`atfRew` `atfRQAm` `atfMhan` `atfReq`/`atfRew` `atfRQam` `atfRQa` `atfRaham` `atfMhva` `atfMhma` |
| loṭ | `tfReQu`/`tfRQAd`/`tfRQAt` `tfRQAm` `tfMhantu` `tfRQi`/`tfRQAd`/`tfRQAt` `tfRQam` `tfRQa` `tfRahAni` `tfRahAva` `tfRahAma` |
| vidhiliṅ | `tfMhyAt`/`tfMhyAd` `tfMhyAtAm` `tfMhyuH` `tfMhyAH` `tfMhyAtam` `tfMhyAta` `tfMhyAm` `tfMhyAva` `tfMhyAma` |

31 one-form cells, 3 two-form, 2 three-form. 43 forms over 36 cells → **7 new `ALTERNATES` rows**.

## File Structure

| file | responsibility | task |
|---|---|---|
| `crates/panini-prakriya/src/tinanta/sound.rs` | `is_shtu`, 8.4.41's trigger class, and its all-arms unit test | 1 |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | 8.4.41's widened trigger; 8.2.41's ḍh arm | 1 |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | the new 8.2.31 and 8.3.13 `Rule`s | 5 |
| `crates/panini-prakriya/src/tinanta/guna.rs` | the new 7.3.92 `Rule` | 4 |
| `crates/panini-prakriya/src/tinanta/adesha.rs` | 6.1.87's second (im) arm | 4 |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | `tinanta_rule_order_is_pinned`; the new rules' unit pins | 1, 4, 5 |
| `crates/panini-data/src/lib.rs` | the `Dhatu` row; `rudhadi_rows_…` renamed and extended; `dhatus().len()` | 3 |
| `crates/panini/tests/paradigm.rs` | `GATED`, then `PARADIGM`, `ALTERNATES`, the audited-numbers test and its doc comment | 3, 7 |
| `tools/audit/panini_full_audit.rs`, `tools/audit/README.md` | corpus totals; "Last recorded result" | 6 |
| `crates/panini/tests/trace.rs` | the three new pins | 8 |
| `AGENTS.md` | the mutation paragraph | 9 |
| `README.md`, `docs/ARCHITECTURE.md`, `AGENTS.md`, `data/ATTRIBUTION.md` | prose, counts, recorded results | 10 |

---

### Task 1: The two tripādī widenings

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/sound.rs` (add `is_shtu` after `is_jhash`)
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (8.4.41 at `:901`, its NARROW GUARD comment above it; 8.2.41 at `:485`)
- Test: `crates/panini-prakriya/src/tinanta/sound.rs` (unit test for `is_shtu`), `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: nothing.
- Produces: `pub(crate) fn is_shtu(c: char) -> bool` in `sound.rs`, read by 8.4.41. 8.2.41 gains no new symbol — its guard widens in place.

These two are grouped because they are the same kind of change (a hardcoded literal becoming a class) in the same file, and because Task 2's dump diff must cover both at once: two separate diffs against `main` would not prove that the *pair* is inert.

- [ ] **Step 1: Write the failing `is_shtu` unit test**

Append to the test module at the bottom of `crates/panini-prakriya/src/tinanta/sound.rs`:

```rust
    /// 8.4.41's trigger class, every arm. `R` has no golden witness — 8.4.1
    /// (ṇatva) runs BELOW 8.4.41 in `tripadi.rs`, so no `R` exists in the
    /// word when 8.4.41 scans — and it is in the table anyway, for the same
    /// reason `kutva_of` carries its witness-less `C`/`J` arms: the class is
    /// ṣ-and-ṭu, and a table that covers only what is currently reachable
    /// rots the moment reachability changes. This test is what keeps it.
    #[test]
    fn shtu_is_sha_plus_the_whole_tavarga() {
        for c in ['z', 'w', 'W', 'q', 'Q', 'R'] {
            assert!(is_shtu(c), "{c} is ṣṭu");
        }
        // The dentals are the TARGET class, never the trigger; `s` and `n`
        // in particular must not qualify, or 8.4.41 would fire on every
        // s-initial ending in the corpus.
        for c in ['t', 'T', 'd', 'D', 'n', 's', 'S', 'k', 'c', 'h'] {
            assert!(!is_shtu(c), "{c} is not ṣṭu");
        }
    }
```

- [ ] **Step 2: Run it to verify it fails**

Run: `mise exec -- cargo test -p panini-prakriya shtu_is_sha_plus_the_whole_tavarga`
Expected: FAIL to compile — `cannot find function is_shtu in this scope`.

- [ ] **Step 3: Add `is_shtu`**

Insert into `crates/panini-prakriya/src/tinanta/sound.rs`, immediately after `is_jhash`:

```rust
/// 8.4.41's conditioning class — *ṣṭunā*, the ṣ-and-ṭu the sūtra names on
/// its trigger side: `z` (ṣ) plus the whole ṭ-varga.
///
/// This was a bare `z` literal inside 8.4.41 until rudhādi 7e. 8.2.31 ho
/// ḍhaḥ produces a `Q`, which must retroflex the `D` that 8.2.40 puts after
/// it (tfneQ + Di → tfneQ + Qi), and a `z`-only trigger cannot see it.
pub(crate) fn is_shtu(c: char) -> bool {
    matches!(c, 'z' | 'w' | 'W' | 'q' | 'Q' | 'R')
}
```

- [ ] **Step 4: Run it to verify it passes**

Run: `mise exec -- cargo test -p panini-prakriya shtu_is_sha_plus_the_whole_tavarga`
Expected: PASS.

- [ ] **Step 5: Widen 8.4.41's trigger**

In `crates/panini-prakriya/src/tinanta/tripadi.rs`, add `is_shtu` to the `use crate::tinanta::sound::{…}` list at the top of the file (alphabetical position: after `is_natva_trigger`, before `is_savarna`).

In 8.4.41's `apply`, replace:

```rust
                if w[i - 1].2 != 'z' {
                    continue;
                }
```

with:

```rust
                if !is_shtu(w[i - 1].2) {
                    continue;
                }
```

- [ ] **Step 6: Rewrite the NARROW GUARD comment**

The comment block above 8.4.41 (`tripadi.rs`, the paragraph beginning `// NARROW GUARD, by design, matching 8.3.59's discipline just above:`) is now false on its trigger half. Replace that whole paragraph with:

```rust
    // TRIGGER: the full ṣṭu class (`is_shtu`), widened from a bare `z`
    // literal in rudhādi 7e. That literal was, as the paragraph above says,
    // the ONE narrowing left holding this rule and 8.4.53 apart under a
    // reordering — so widening it does not weaken the pair, it removes the
    // pair's last order-dependence. 8.2.31 ho ḍhaḥ is what made the wider
    // class reachable: it produces a `Q` that must retroflex 8.2.40's `D`
    // (tfneQ + Di → tfneQ + Qi → 8.3.13 → tfneQi). Verified inert against
    // the pre-7e 2592-cell corpus by a byte-for-byte dump diff before any
    // new root was curated.
    //
    // The CORRESPONDENCE side stays narrow, and deliberately: only t/T/D
    // have a witness. √tṛh reaches `D` → `Q` and nothing wider, so d/n/s
    // are still absent. Widen that half the moment a junction reaches it —
    // it is a separate claim from the trigger's, with separate evidence.
```

- [ ] **Step 7: Widen 8.2.41's guard**

In 8.2.41's `apply`, replace:

```rust
                if w[i].2 != 's' || w[i - 1].2 != 'z' {
                    continue;
                }
```

with:

```rust
                if w[i].2 != 's' || !matches!(w[i - 1].2, 'z' | 'Q') {
                    continue;
                }
```

and insert this paragraph immediately above the `Rule {` line for 8.2.41:

```rust
    // BOTH SOUNDS THE SŪTRA NAMES. *ṣaḍhoḥ* is a dvandva — ṣ **and** ḍh —
    // and until rudhādi 7e the guard read `z` alone, because √piṣ was the
    // only root that reached the rule and it presents a `z`. √tṛh presents
    // the other: 8.2.31 ho ḍhaḥ turns its `h` into a `Q`, and tfneQ + si
    // must become tfnek + si (→ tfRekzi by 8.3.59 and 8.4.1). Same shape as
    // the 8.2.30 episode — a rule whose own name promised two cases and
    // whose code implemented one — caught here before an audit had to.
```

- [ ] **Step 8: Run the prakriyā crate's own tests**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS. In particular `pish_lot_madhyama_eka_is_pinddhi` (whose `piRqQi` is the cell the 8.4.41/8.4.53 ordering argument turns on) and `shadhoh_kah_si_declines_when_8_2_23_ate_the_s_first` must both still pass. If either fails, **stop** — the widening is not inert and Task 2's gate has already been answered.

- [ ] **Step 9: Run the full golden suite in the foreground**

Run: `mise run test`
Expected: PASS, ~12 minutes. Do not background it.

- [ ] **Step 10: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/sound.rs crates/panini-prakriya/src/tinanta/tripadi.rs
git commit -m "fix(tripadi): 8.4.41 reads the whole stu class, 8.2.41 both its sounds"
```

---

### Task 2: The dump diff — the first blocking gate

**Files:**
- Create then delete: two dump files under the scratchpad directory (not the repo)

**Interfaces:**
- Consumes: Task 1's two widenings.
- Produces: the evidence that the widenings perturb no existing cell — the fact every later task's attribution argument rests on.

The spec's whole reason for allowing six changes in one slice is this gate. Task 1 changed two rules that every ṣṭutva and every s-initial junction in the corpus passes through. If they are inert on the existing 2592 cells, then any difference Task 6's audit reports belongs to √tṛh or the three new sūtras, and nowhere else. **Do not skip this because Task 1's suite went green:** the golden suite pins the *declined* branch of every cell plus its alternates, and the dump is the full derivation set, cell by cell — a strictly wider object.

- [ ] **Step 1: Set up the vidyut checkout, if it is not already present**

```bash
head -20 data/dhatupatha.tsv | grep commit
```
Expected: the commit the corpus was vendored from — `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`.

```bash
ls /tmp/vidyut-full 2>/dev/null || (cd /tmp && git clone --filter=blob:none https://github.com/ambuda-org/vidyut vidyut-full && cd vidyut-full && git checkout 8da2f90bee3ce1c07505fa432fc3729e3f7e02ea)
```

Copy the committed harness in, per `tools/audit/README.md`:

```bash
cp tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
```

- [ ] **Step 2: Dump the corpus as `main` derives it**

```bash
git stash
cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_DUMP=/tmp/claude-1000/-workspace/*/scratchpad/before.tsv cargo run --release --example panini_full_audit
git stash pop
```

Expected: the harness asserts 63 roots / 2592 cells / 3014 forms and writes `before.tsv`. Use the literal scratchpad path this session was given rather than a glob if the shell does not expand it.

- [ ] **Step 3: Dump the corpus as the widened engine derives it**

```bash
cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_DUMP=/tmp/claude-1000/-workspace/*/scratchpad/after.tsv cargo run --release --example panini_full_audit
```

- [ ] **Step 4: Diff them**

```bash
diff /tmp/claude-1000/-workspace/*/scratchpad/before.tsv /tmp/claude-1000/-workspace/*/scratchpad/after.tsv && echo "INERT"
```

Expected: no output, then `INERT`.

**If the diff is non-empty, stop and do not continue to Task 3.** A widening perturbed an existing cell. Record which cells and which rule, then revert the offending widening and re-approach it — the spec's posture is that a widening that is not inert is reverted, not patched around. Note that a *correct-looking* changed form is still a failure here: the pre-7e corpus was audited byte-for-byte against vidyut, so any change is a regression by construction.

- [ ] **Step 5: Record the result in the commit message and clean up**

```bash
rm -f /tmp/claude-1000/-workspace/*/scratchpad/before.tsv /tmp/claude-1000/-workspace/*/scratchpad/after.tsv
git commit --allow-empty -m "test(audit): the 8.4.41 and 8.2.41 widenings are inert on 2592 cells

Full derivation-set dump before and after Task 1's two widenings, over the
whole pre-7e corpus (63 roots / 2592 cells / 3014 forms): byte-identical.
Every attribution argument in the rest of this slice rests on this."
```

---

### Task 3: The √tṛh data row

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (the end of `DHATUS`; `rudhadi_rows_are_the_twenty_one_curated_roots`; the `dhatus().len()` assertion at `:750`)
- Modify: `crates/panini/tests/paradigm.rs` (the `GATED` constant, currently `&[]`)

**Interfaces:**
- Consumes: nothing.
- Produces: one curated root addressable as `Dhatu { dhatupatha: "07.0018", code: "tfh", … }`, which Tasks 4, 5, 6, 7 and 8 all resolve through `dhatus()`.

The row lands **before** the new sūtras so Tasks 4 and 5 can TDD against real derivations rather than synthetic term vectors. Its forms will be wrong until Task 5 completes — that is expected, and `GATED` is how the suite stays green in the meantime.

- [ ] **Step 1: Write the failing table test**

In `crates/panini-data/src/lib.rs`, rename `rudhadi_rows_are_the_twenty_one_curated_roots` to `rudhadi_rows_are_the_twenty_two_curated_roots`, and append one entry to its expected vector, after `("07.0025", "pfc", PadaAssignment::Parasmaipada),` and in the position Step 3 inserts the row (the assertion compares against `DHATUS` order):

```rust
                ("07.0018", "tfh", PadaAssignment::Parasmaipada),
```

Append to that test's comment block:

```rust
        // Slice 7e adds √tṛh, the ninth and last of the "reachable
        // non-ubhayapadī" roots 7d's probe separated out. It was the one
        // that did NOT come free: 7.3.92 tfRaha im, 8.2.31 ho QaH and
        // 8.3.13 Qo Qe lopaH are all new in this slice, and 8.4.41, 8.2.41
        // and 6.1.87 all had to widen to carry it. Parasmaipada: `tfha~`
        // carries no anudātta and no ñi, so 1.3.78 SezAt kartari
        // parasmaipadam settles it, and vidyut-prakriya derives no
        // ātmanepada forms for the entry.
        //
        // Three of rudhādi's 25 are still out after this: √chid and √chṛd
        // (6.1.73 Ce ca with 8.4.40 stoH ScunA ScuH) and √bhuj (1.3.66
        // Bujo'navane, which forks its pada on sense).
```

- [ ] **Step 2: Run it to verify it fails**

Run: `mise exec -- cargo test -p panini-data rudhadi_rows_are_the_twenty_two_curated_roots`
Expected: FAIL — the actual vector is 21 entries against an expected 22.

- [ ] **Step 3: Add the `Dhatu` row**

Insert at the end of `DHATUS` in `crates/panini-data/src/lib.rs`, after the `07.0013` (`vid`) row:

```rust
    Dhatu {
        // 07.0018 tfha~ hiMsAyAm. The gaṇa's ninth reachable
        // non-ubhayapadī root and the only one that needed sūtras this
        // engine lacked: 7.3.92 tfRaha im puts the *im* āgama into the
        // stem (tfnah -> tfnaih -> tfneh by 6.1.87), 8.2.31 ho QaH takes
        // the root's `h` to `Q`, and 8.3.13 Qo Qe lopaH elides it before
        // the `Q` that 8.4.41 produces -- tfReQi.
        //
        // The im is conditioned on a HAL-INITIAL PIT sārvadhātuka, which
        // is why this one root's paradigm splits three ways rather than
        // two: tfReQi/tfRekzi/tfRehmi take it, tfRQaH/tfMhanti do not
        // (apit, hence ṅit by 1.2.4), and atfRaham does not either
        // (`am` is vowel-initial).
        dhatupatha: "07.0018",
        code: "tfh",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
```

- [ ] **Step 4: Bump `dhatus().len()`**

At `crates/panini-data/src/lib.rs:750`, change `assert_eq!(dhatus().len(), 63);` to `assert_eq!(dhatus().len(), 64);`.

- [ ] **Step 5: Update the two 63-root prose sites in the same file**

`crates/panini-data/src/lib.rs` states the root count in two more places, both of which the count change falsifies:

- `:94` — "The test covers the 63 roots curated here, not the dhātupāṭha's 2259." → 64.
- `:1102` and `:1298` — the backslash census, "42 of the 63 curated roots carry a `\` at all, and 29 …". `tfha~` carries **no** backslash, so the denominator moves and the numerators do not: **42 of the 64**, 29 unchanged. Re-run the test rather than trusting that arithmetic — it asserts the numerators too.

- [ ] **Step 6: Run the data crate's tests**

Run: `mise exec -- cargo test -p panini-data`
Expected: PASS, including `dhatupatha_numbers_resolve_upstream` — which it-strips `tfha~` and compares against `code`, so a wrong `code` fails here rather than in the audit.

- [ ] **Step 7: Gate the four golden triples**

√tṛh now has no `PARADIGM` block, which `every enumerable (root, lakara, pada) triple needs golden rows` fails on. That test carries a `GATED` escape hatch for exactly this. In `crates/panini/tests/paradigm.rs`, replace `const GATED: &[(&str, &str, Pada)] = &[];` with:

```rust
    // TEMPORARY, slice 7e: √tṛh's data row lands before its sūtras (so
    // 7.3.92, 8.2.31 and 8.3.13 can be TDD'd against a real derivation)
    // and before its goldens (which are generated from the engine the
    // audit certifies, never hand-authored). Task 7 empties this back to
    // `&[]` in the same commit that pastes the blocks. If this constant is
    // still non-empty when the slice ships, a root shipped uncovered.
    const GATED: &[(&str, &str, Pada)] = &[
        ("07.0018", "laT", Pada::Parasmaipada),
        ("07.0018", "laN", Pada::Parasmaipada),
        ("07.0018", "loT", Pada::Parasmaipada),
        ("07.0018", "viDiliN", Pada::Parasmaipada),
    ];
```

- [ ] **Step 8: Run the full golden suite in the foreground**

Run: `mise run test`
Expected: PASS, ~12 minutes. Do not background it.

- [ ] **Step 9: Commit**

```bash
git add crates/panini-data/src/lib.rs crates/panini/tests/paradigm.rs
git commit -m "feat(data): tfh, the rudhadi root that needed three sutras"
```

---

### Task 4: 7.3.92 and 6.1.87's im arm

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/guna.rs` (new `Rule`, between 7.3.86 at `:107` and the second 7.3.84 at `:169`)
- Modify: `crates/panini-prakriya/src/tinanta/adesha.rs` (6.1.87 at `:349`)
- Test: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: the `07.0018` row (Task 3).
- Produces: the rule id `"7.3.92"` in `TINANTA_RULES`, positioned between the first `"7.3.86"` and the second `"7.3.84"`; 6.1.87 gains an arm that fires only when `7.3.92` is in `p.log`.

The two ship together because 6.1.87's new arm is gated on 7.3.92 having fired, so it cannot be written or tested first.

- [ ] **Step 1: Write the failing derivation test**

Append to `crates/panini-prakriya/src/tinanta/derivation_tests.rs`:

```rust
#[test]
fn trh_takes_the_im_agama_only_before_a_hal_initial_pit_sarvadhatuka() {
    // 7.3.92 tfRaha im, all four conjuncts of its guard, each with the cell
    // that would break if it were dropped.
    //
    // Asserted on the LOG rather than on a surface or a stem, deliberately.
    // Both are already rewritten by the time `derive` returns -- 8.4.1 has
    // taken Snam's `n` to `R` -- and neither settles until 8.2.31 and
    // 8.3.13 land in the next task. Whether the Agama fired is the claim,
    // and the log states it directly. The surfaces get asserted next task,
    // in `trh_lat_reaches_its_three_shapes`.
    //
    // 6.1.87 is asserted alongside because for this root only its im arm
    // can fire: SHAP holds Snam, which is not Thematic, so the junction arm
    // declines. The two rules stand or fall together.
    fn fired(la: Lakara, pu: Purusha, va: Vacana) -> (bool, bool) {
        let d = dhatus()
            .iter()
            .find(|d| d.dhatupatha == "07.0018")
            .expect("07.0018 is curated");
        let p = derive(d, la, Pada::Parasmaipada, pu, va)
            .into_iter()
            .next()
            .expect("every enumerable cell derives at least one branch");
        let has = |id: &str| p.log.iter().any(|step| step.sutra == id);
        (has("7.3.92"), has("6.1.87"))
    }

    // FIRES: hal-initial, pit, sArvadhAtuka, not Ngit.
    for (la, pu, va, why) in [
        (Lakara::Lat, Purusha::Prathama, Vacana::Eka, "ti"),
        (Lakara::Lat, Purusha::Madhyama, Vacana::Eka, "si"),
        (Lakara::Lat, Purusha::Uttama, Vacana::Eka, "mi"),
        // laN tip's apRkta `t`. 8.2.23 saMyogAntasya lopaH eats it, but not
        // until the tripAdI -- one stage BELOW this rule -- so the hal test
        // still sees it here. That ordering is pinned again in trace.rs.
        (Lakara::Lan, Purusha::Prathama, Vacana::Eka, "t"),
    ] {
        assert_eq!(fired(la, pu, va), (true, true), "{why}");
    }

    // DECLINES on the hal conjunct: the ending is vowel-initial.
    for (la, pu, va, why) in [
        (Lakara::Lan, Purusha::Uttama, Vacana::Eka, "am -> atfRaham"),
        (Lakara::Lot, Purusha::Uttama, Vacana::Eka, "Ani -> tfRahAni"),
    ] {
        assert_eq!(fired(la, pu, va), (false, false), "{why}");
    }

    // DECLINES on the pit conjunct: 1.2.4 makes tas/Ta/vas Ngit, and
    // 6.4.111 takes Snam's `a` instead of the Agama going in.
    assert_eq!(
        fired(Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
        (false, false),
        "tas -> tfRQaH"
    );

    // DECLINES on the Ngit conjunct, which the pit conjunct does NOT cover:
    // under yAsuT the ending's own `t` is still pit, and it is the Agama
    // that carries the N. The ending is hal-initial too, so this cell
    // isolates the fourth conjunct exactly.
    assert_eq!(
        fired(Lakara::VidhiLin, Purusha::Prathama, Vacana::Eka),
        (false, false),
        "yAsuT -> tfMhyAt"
    );
}
```

The test's imports (`dhatus`, `derive`, `Lakara`, `Pada`, `Purusha`, `Vacana`) are already in `derivation_tests.rs`'s prelude; add none.

- [ ] **Step 2: Run it to verify it fails**

Run: `mise exec -- cargo test -p panini-prakriya trh_takes_the_im_agama`
Expected: FAIL — the four firing cells come back `(false, false)`, because no rule inserts the āgama yet.

- [ ] **Step 3: Add 7.3.92 to `guna.rs`**

Insert into `GUNA` in `crates/panini-prakriya/src/tinanta/guna.rs`, immediately after 7.3.86's closing `},` and before the second 7.3.84's comment block:

```rust
    // 7.3.92 tṛṇaha im: √tṛh takes the *im* āgama before a hal-initial pit
    // sārvadhātuka. tfnah + ti → tfnaih, which 6.1.87 ād guṇaḥ (in
    // `super::adesha`) then coalesces to tfneh → tfReQi.
    //
    // The āgama is **mit**, so 1.1.47 mid aco'ntyāt paraḥ places it after
    // the last vowel of what it attaches to. 1.1.47 is cited here, not
    // implemented as its own Rule — the treatment 3.1.78 already gives
    // śnam, and 1.4.13 and 1.1.5 get elsewhere. This is the engine's first
    // ĀGAMA placed that way; the placement itself is not new.
    //
    // REPRESENTATION. The aṅga is `tfnah`, but 3.1.78 splits the rudhādi
    // stem across ANGA and SHAP, so it is held as [tf, nah]. The last vowel
    // of the COMBINED stem is śnam's own `a`, which lives in SHAP — so both
    // this insertion and 6.1.87's coalescence are SHAP-internal and neither
    // touches ANGA. The guard reads the combined text rather than the two
    // slots separately on purpose: the split is an implementation artifact,
    // `tfnah` is what the sūtra names. `ends_with` rather than `==` because
    // 6.4.71 has already prefixed the laṅ aṭ-augment onto ANGA (atf) by
    // this point — the same allowance 7.4.21's guard makes.
    //
    // FOUR CONJUNCTS, each with a negative control among √tṛh's own 36
    // golden cells. Drop any one and a golden breaks:
    //   - the stem is tfnah        every other rudhādi root
    //   - hal-initial follower     `am` → atfRaham; loṭ uttama Ani/Ava/Ama
    //   - pit sārvadhātuka         tas/Ta/vas, ṅit by 1.2.4 → tfRQaH
    //   - NOT ṅit                  tātaṅ (7.1.35) → tfRQAt; yāsuṭ → tfMhyAt
    // The fourth is not redundant with the third: under yāsuṭ the ending's
    // own `t` is still pit, and it is the ĀGAMA that carries the ṅ.
    //
    // The sārvadhātuka clause is a real guard here, not a structural
    // always-true as at 7.3.84: it is read off the ending directly, and
    // costs nothing to state.
    Rule {
        id: "7.3.92",
        name: "tfRaha im",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let stem = format!("{}{}", p.terms[ANGA].text, p.terms[SHAP].text);
            if !stem.ends_with("tfnah") {
                return false;
            }
            let ending = &p.terms[ENDING];
            if !ending.has(Tag::Pit) || ending.has(Tag::Ngit) || !ending.has(Tag::Sarvadhatuka) {
                return false;
            }
            // *hali*: the ending must lead with a consonant. "Not a vowel"
            // is exact here — every ending in scope is vowel- or
            // consonant-initial, with no third case.
            let Some(first) = ending.text.chars().next() else {
                return false;
            };
            if is_vowel(first) {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[SHAP].text.chars().collect();
            let last = s
                .pop()
                .expect("ends_with(\"tfnah\") implies a non-empty SHAP");
            s.push('i');
            s.push(last);
            p.terms[SHAP].text = s.into_iter().collect();
            p.record("7.3.92", "tfRaha im", before);
            true
        },
    },
```

Then correct the stale inventory in the **second** 7.3.84's comment, a few lines below in the same file. It reads "The complete inventory of SHAP texts reaching this point is `a` (śap/śa), `ya` (śyan), `` (adādi luk), `Ana` (śānac), `nA`/`n` (śnā, 6.4.112) and `nI` (śnā, 6.4.113)" — which already omitted rudhādi's śnam-plus-tail shapes before this slice, and which 7.3.92 now adds one more to. Replace that sentence with:

```rust
    // NO DELTA on any pre-existing form, by guard rather than by argument.
    // The complete inventory of SHAP texts reaching this point is `a`
    // (śap/śa), `ya` (śyan), `` (adādi luk), `Ana` (śānac), `nA`/`n` (śnā,
    // 6.4.112), `nI` (śnā, 6.4.113), and — for rudhādi, where SHAP holds
    // śnam followed by the root's own tail (3.1.78) — `nat`, `nah`, `nans`
    // and their kin, plus `naih` once 7.3.92 above has put the im in.
    // Only `nI` is ik-final; every rudhādi shape is consonant-final, so
    // `guna_of` returns None for all of them. 6.4.113 produces `nI` ONLY
    // before a ṅit ending — so the 1.1.5 test below declines there. Two
    // tests pin both halves.
```

- [ ] **Step 4: Add 6.1.87's im arm**

In `crates/panini-prakriya/src/tinanta/adesha.rs`, insert at the very top of 6.1.87's `apply`, before the existing `let first = …` line:

```rust
            // ARM 2, the 7.3.92 im (rudhādi 7e). The āgama put an `i`
            // inside SHAP immediately after śnam's `a` — tfnah → tfnaih —
            // and guṇa coalesces that `a i` into `e`, still inside SHAP,
            // consuming nothing from the ending. That is what makes it a
            // separate arm rather than a widening of the junction arm
            // below: the two operations differ in what they consume, not
            // just in where they look.
            //
            // Gated on 7.3.92 having FIRED IN THIS DERIVATION rather than
            // on sniffing SHAP for an `ai`: the āgama IS the condition, not
            // a proxy for it, and the gate makes the arm structurally
            // unable to fire for a root that does not take it. Same idiom
            // 6.4.72 and 7.1.6 use to read the log for a prior rule.
            if p.log.iter().any(|s| s.sutra == "7.3.92") {
                let chars: Vec<char> = p.terms[SHAP].text.chars().collect();
                let Some(pos) = chars.windows(2).position(|w| w == ['a', 'i']) else {
                    return false;
                };
                let before = p.snapshot();
                let mut s = chars;
                s.remove(pos + 1);
                s[pos] = 'e';
                p.terms[SHAP].text = s.into_iter().collect();
                p.record("6.1.87", "Ad guRaH", before);
                return true;
            }
```

Extend 6.1.87's comment block above the `Rule {` line with:

```rust
    // TWO ARMS since rudhādi 7e. The junction arm (below, and the original)
    // coalesces śap's `a` with the ending's initial `i`/`I` and eats that
    // initial. The im arm coalesces an `a i` that sits wholly inside SHAP,
    // put there by 7.3.92, and eats nothing. Both are ād guṇaḥ; they differ
    // in what the `i` belongs to.
```

- [ ] **Step 5: Update the pinned rule order**

In `crates/panini-prakriya/src/tinanta/derivation_tests.rs`, in `tinanta_rule_order_is_pinned`, insert `"7.3.92"` between the first `"7.3.86"` and the following `"7.3.84"`:

```rust
        "6.4.23", "7.4.21", "7.3.84", "7.3.86", "7.3.92", "7.3.84", "6.4.87", "6.4.77", "6.1.78",
        "7.3.101",
```

Append to that test's doc comment:

```rust
/// 7.3.92 sits between the two 7.3.84 applications: in sūtra order, and
/// necessarily above 6.1.87 in `tinanta/adesha.rs`, which coalesces the
/// āgama it inserts.
```

- [ ] **Step 6: Run the tests**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS, including the new `trh_takes_the_im_agama_only_before_a_hal_initial_pit_sarvadhatuka` and `tinanta_rule_order_is_pinned`.

If a firing cell still comes back `(false, false)`, check whether 3.4.86 (loṭ `tu`) and 3.4.100 (laṅ `t`) preserve `Tag::Pit` when they rewrite the ending — 3.4.87 explicitly clears it for `hi`, and the others must not. The fix is to preserve the tag there, **not** to weaken 7.3.92's third conjunct.

- [ ] **Step 7: Run the full golden suite in the foreground**

Run: `mise run test`
Expected: PASS, ~12 minutes. No existing cell may move: 7.3.92's stem guard admits only √tṛh, and 6.1.87's new arm is gated on 7.3.92 having fired.

- [ ] **Step 8: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/guna.rs crates/panini-prakriya/src/tinanta/adesha.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(guna): 7.3.92 trnaha im, and the arm of 6.1.87 that finishes it"
```

---

### Task 5: 8.2.31 and 8.3.13

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (new `Rule`s: 8.2.31 between 8.2.30 at `:308` and 8.2.39 at `:378`; 8.3.13 immediately after 8.4.41)
- Test: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: the `07.0018` row (Task 3); 7.3.92 (Task 4).
- Produces: the rule ids `"8.2.31"` and `"8.3.13"` in `TINANTA_RULES`, positioned after `"8.2.30"` and after `"8.4.41"` respectively.

- [ ] **Step 1: Write the failing surface test**

Append to `crates/panini-prakriya/src/tinanta/derivation_tests.rs`:

```rust
#[test]
fn trh_lat_reaches_its_three_shapes() {
    // The three tails √tṛh's laṭ splits into, one assertion each, and the
    // reason 8.2.31's *jhali* condition has to be a real guard:
    //
    //   tfReQi   `h` before the jhal `t`     -> 8.2.31, then 8.3.13
    //   tfRekzi  `h` before `s`              -> 8.2.31, then 8.2.41's Q arm
    //   tfRehmi  `h` before `m`, NOT a jhal  -> 8.2.31 declines, `h` stays
    //
    // tfRehmi is the load-bearing one: an 8.2.31 that fired on every `h`
    // would give *tfReQmi, a form that looks no less Sanskrit than the
    // right one.
    // Every laṭ cell of this root is single-form, so `form_g` (which goes
    // through `sole`) is the right helper: a cell that unexpectedly gains
    // an optional branch fails loudly here rather than having its first
    // branch read silently.
    let lat = |pu, va| form_g("07.0018", Lakara::Lat, pu, va);

    assert_eq!(lat(Purusha::Prathama, Vacana::Eka), "tfReQi");
    assert_eq!(lat(Purusha::Madhyama, Vacana::Eka), "tfRekzi");
    assert_eq!(lat(Purusha::Uttama, Vacana::Eka), "tfRehmi");
    // The apit cells, where 6.4.111 runs instead of the āgama and 8.3.13
    // still fires -- on the `Q` that 8.4.41 makes out of 8.2.40's `D`.
    assert_eq!(lat(Purusha::Prathama, Vacana::Dvi), "tfRQaH");
    // And the one where nothing retroflexes at all: `h` before a vowel.
    assert_eq!(lat(Purusha::Prathama, Vacana::Bahu), "tfMhanti");
}
```

- [ ] **Step 2: Run it to verify it fails**

Run: `mise exec -- cargo test -p panini-prakriya trh_lat_reaches_its_three_shapes`
Expected: FAIL — `tfReQi` comes back as something ending in `hti`, because no rule takes `h` to `Q` yet.

- [ ] **Step 3: Add 8.2.31**

Insert into `TRIPADI` in `crates/panini-prakriya/src/tinanta/tripadi.rs`, immediately after 8.2.30's closing `},`:

```rust
    // 8.2.31 ho ḍhaḥ: `h` becomes `Q` (ḍh). The *jhali* and *padasya*
    // conditions come by anuvṛtti from the same place 8.2.30 coH kuH reads
    // them, so the guard is written the same way — find the first `h` that
    // is genuinely word-final or jhal-followed, rather than the first `h`
    // in the word, so a non-applicable `h` earlier can never hide a later
    // applicable one.
    //
    // tfneh + ti → tfneQ + ti; tfnh + tas → tfnQ + tas; atfneh → atfneQ
    // (pada-final, the laṅ arm, after 8.2.23 above has eaten tip's `t`).
    //
    // It must DECLINE before `m` and `v` — neither is a jhal — which is
    // exactly what leaves tfRehmi and tfMhvaH their `h`, and before a
    // vowel, which leaves tfMhanti its own. `is_jhal` already carries `h`
    // itself, so an `h h` junction would qualify; none arises here (6.4.101
    // has already taken loṭ's `hi` to `Di` by this point), and the general
    // form is kept rather than special-cased.
    Rule {
        id: "8.2.31",
        name: "ho QaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let w = word_chars(p);
            let Some(pos) = w.iter().enumerate().position(|(i, (_, _, c))| {
                *c == 'h' && w.get(i + 1).is_none_or(|(_, _, next)| is_jhal(*next))
            }) else {
                return false;
            };
            let (term, idx, _) = w[pos];
            let before = p.snapshot();
            set_char(p, term, idx, 'Q');
            p.record("8.2.31", "ho QaH", before);
            true
        },
    },
```

- [ ] **Step 4: Add 8.3.13**

Insert into `TRIPADI` immediately after 8.4.41's closing `},` and before 8.4.53's comment block:

```rust
    // 8.3.13 ḍho ḍhe lopaḥ: a `Q` is elided before a `Q`.
    // tfReQ + Qi → tfRe + Qi; tfMQ + QaH → tfM + QaH.
    //
    // OUT OF SŪTRA ORDER, immediately below 8.4.41, and this is
    // load-bearing twice over.
    //
    // First, the condition. The SECOND ḍh does not exist until ṣṭutva has
    // run: 8.2.31 makes the stem-final `Q`, 8.2.40 makes the ending's `t`
    // into `D`, and only 8.4.41 above turns that `D` into the `Q` this rule
    // needs. Placed in numeric order it would see tfneQ + Di, decline, and
    // the cell would surface *tfReQQi. The file already orders by operation
    // where the derivation demands it — 8.2.73 sits below 8.2.75, and
    // 8.4.56 sits last, below 8.4.65.
    //
    // Second, the fork count. √tṛh reaches loṭ madhyama eka in the same
    // kfnt + Di shape that makes every other stop-final rudhādi root a
    // SIX-form cell (8.4.53 voices, 8.4.65 optionally elides, 7.1.35 and
    // 8.4.56 multiply). √tṛh's is a three-former, because this rule
    // obligatorily eats the very ḍh 8.4.65 would have forked on. Move this
    // rule below 8.4.65 and the cell silently grows to six forms —
    // `trnaddhi_trace_has_8_3_13_and_no_8_4_65` in `panini`'s trace suite
    // is the pin, and the ALTERNATES count is the second alarm.
    //
    // 6.3.111 ḍhralope pūrvasya dīrgho'ṇaḥ does NOT follow this elision
    // here, and its absence is deliberate rather than an omission: it
    // lengthens a preceding **aṇ**, and in every √tṛh cell the sound before
    // the elided ḍh is `e` (tfRe + Qi) or `M` (tfM + QaH), neither of which
    // is one. vidyut-prakriya's traces do not emit it either. Implement it
    // when a root presents a short a/i/u there — this comment is the note
    // that says why there is nothing to implement yet.
    Rule {
        id: "8.3.13",
        name: "Qo Qe lopaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let w = word_chars(p);
            let Some(i) = (1..w.len()).find(|&i| w[i - 1].2 == 'Q' && w[i].2 == 'Q') else {
                return false;
            };
            let (term, idx, _) = w[i - 1];
            let before = p.snapshot();
            remove_char(p, term, idx);
            p.record("8.3.13", "Qo Qe lopaH", before);
            true
        },
    },
```

- [ ] **Step 5: Update the pinned rule order**

In `tinanta_rule_order_is_pinned`, insert `"8.2.31"` after `"8.2.30"` and `"8.3.13"` after `"8.4.41"`:

```rust
        "8.2.30", "8.2.31", "8.2.39", "8.2.40", "8.2.41", "8.2.74", "8.2.75", "8.2.73", "8.3.15",
        "8.3.24", "8.3.59", "8.4.41", "8.3.13", "8.4.53", "8.4.55", "8.4.1", "8.4.2", "8.4.58",
        "8.4.65", "8.4.56",
```

Append to that test's doc comment:

```rust
/// 8.3.13 sits BELOW 8.4.41, against sūtra order: the second ḍh it needs
/// is 8.4.41's own output. See its comment in `tinanta/tripadi.rs`.
```

- [ ] **Step 6: Run the tests**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS, including `trh_lat_reaches_its_three_shapes` and `tinanta_rule_order_is_pinned`.

- [ ] **Step 7: Run the full golden suite in the foreground**

Run: `mise run test`
Expected: PASS, ~12 minutes. No existing cell may move: no curated root before √tṛh has an `h` in a jhal-followed or pada-final position, and none produces two adjacent `Q`.

- [ ] **Step 8: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/tripadi.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(tripadi): 8.2.31 ho QaH, and 8.3.13 below the stutva that feeds it"
```

---

### Task 6: The cross-implementation audit — the second blocking gate

**Files:**
- Modify: `tools/audit/panini_full_audit.rs` (the corpus-total assertions at `:577–579`, and the header's totals at `:12`, `:24`, `:27`, `:54`)
- Modify: `tools/audit/README.md` ("Last recorded result")

**Interfaces:**
- Consumes: the complete engine (Tasks 1, 4, 5) and the `07.0018` row (Task 3).
- Produces: the verdict Task 7's goldens are generated under, and the measured form total `<N>` that Task 7 and Task 10 both quote.

- [ ] **Step 1: Update the harness's corpus totals**

In `tools/audit/panini_full_audit.rs`, at `:577–579`:

```rust
    assert_eq!(roots_seen.len(), 64, "curated roots");
    assert_eq!(n_cells, 2628, "cells: 292 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 3057, "forms: 2628 cells + 429 ALTERNATES rows");
```

and update the four header mentions — `:12` (`for each of the 63 curated roots` → 64), `:24` (`63 roots, 2592 cells, 3014 forms` → `64 roots, 2628 cells, 3057 forms`), `:27` (`288 root×lakāra blocks × 9 cells, plus 422 ALTERNATES rows` → `292 … plus 429 …`), `:54` (`the full 2592-cell table` → 2628).

If the run reports a different form total than 3057, **the harness's number is the measurement and 3057 was the projection** — change the assertion to what it measured, and carry that value forward into Tasks 7 and 10. A differing *cell* count, by contrast, means a pada or lakāra is miscounted; investigate rather than adjust.

- [ ] **Step 2: Copy the harness into the vidyut checkout**

```bash
cp tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
```

- [ ] **Step 3: Run the `entry` negative control FIRST**

```bash
cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_PERTURB=entry cargo run --release --example panini_full_audit; echo "exit=$?"
```

Expected: **exit 1**, with 36 √bhū cells flagged. A zero-difference result recorded without this proves nothing.

- [ ] **Step 4: Run the `form` negative control**

```bash
cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_PERTURB=form cargo run --release --example panini_full_audit; echo "exit=$?"
```

Expected: **exit 1**.

- [ ] **Step 5: Run the real audit**

```bash
cd /tmp/vidyut-full/vidyut-prakriya && cargo run --release --example panini_full_audit; echo "exit=$?"
```

Expected: **exit 0**, 64 roots / 2628 cells / `<N>` forms, zero differences.

**If any cell differs, stop.** Task 2 already proved the widenings inert on the pre-7e corpus, so a difference belongs to √tṛh — to one of the three new sūtras or to 7.3.92's guard. The guard's four conjuncts each have a named negative control in Task 4's test, so the failing cell identifies the conjunct: a wrong `tfRahAni` implicates the hal test, a wrong `tfRQaH` the pit test, a wrong `tfMhyAt` the ṅit test. The spec's posture is fixed: fix the rule, do not widen the slice.

- [ ] **Step 6: Record the result**

In `tools/audit/README.md`, update "Last recorded result" to name: vidyut commit `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, 64 roots / 2628 cells / `<N>` forms, zero differences, both negative controls verified failing, and that this is the first audit run after 7e's six rule changes.

- [ ] **Step 7: Commit**

```bash
git add tools/audit/panini_full_audit.rs tools/audit/README.md
git commit -m "test(audit): tfh is byte-identical to vidyut across 2628 cells"
```

---

### Task 7: The goldens, generated

**Files:**
- Create then delete: `crates/panini/tests/print_7e_goldens.rs`
- Modify: `crates/panini/tests/paradigm.rs` (`PARADIGM`, `ALTERNATES`, `GATED` back to `&[]`, `derivation_set_shape_matches_the_audited_numbers` and its doc comment)

**Interfaces:**
- Consumes: the `07.0018` row (Task 3); the audit verdict and form total `<N>` (Task 6).
- Produces: 4 `PARADIGM` blocks and their `ALTERNATES` rows; the measured cell-multiplicity distribution and per-key counts Task 10's prose quotes.

- [ ] **Step 1: Write the throwaway generator**

Create `crates/panini/tests/print_7e_goldens.rs`:

```rust
//! THROWAWAY -- slice 7e only. Prints √tṛh's `PARADIGM` blocks and
//! `ALTERNATES` rows as Rust source, plus the distribution counts
//! `derivation_set_shape_matches_the_audited_numbers` asserts. Deleted in
//! the same task that pastes its output: goldens are generated from the
//! engine the audit certified, never hand-authored.
//!
//! Run with:
//!   mise exec -- cargo test -p panini --test print_7e_goldens -- --nocapture

mod common;

use common::{CELLS, LAKARA_BY_NAME};
use panini_data::{Pada, dhatus};
use panini_prakriya::derive;

const NEW_ROOTS: [&str; 1] = ["07.0018"];

/// Mirrors `VIKALPA_RULES` in `paradigm.rs`. An alternate's key is the
/// `+`-joined list of optional rules its branch actually applied, which is
/// what `every_alternate_names_the_vikalpa_rules_that_produced_it` checks.
/// 7e adds no optional rule, so this list is unchanged from 7d.
const VIKALPA_RULES: &[&str] = &[
    "7.1.35", "3.4.111", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56",
];

#[test]
fn print_7e_goldens() {
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
                                "    ({:?}, {:?}, Pada::{:?}, {}, {:?}, {:?}),",
                                number,
                                lak_name,
                                pada,
                                cell,
                                p.text(),
                                key
                            ),
                            key,
                        ));
                        n_alts += 1;
                    }
                    multiplicity[n_alts + 1] += 1;
                    goldens.push(format!("{golden:?}"));
                }
                println!("    (");
                println!("        {number:?},");
                println!("        {lak_name:?},");
                println!("        Pada::{pada:?},");
                println!("        [");
                println!("            {},", goldens.join(", "));
                println!("        ],");
                println!("    ),");
            }
        }
    }

    println!("\n// ==== ALTERNATES rows ====");
    // Deduplicate: two branches can reach the same surface by different
    // optional-rule paths, and ALTERNATES holds one row per (cell, form).
    let mut seen = std::collections::BTreeSet::new();
    let mut per_key: std::collections::BTreeMap<String, usize> = Default::default();
    for (row, key) in &alternates {
        if seen.insert(row.clone()) {
            println!("{row}");
            *per_key.entry(key.clone()).or_default() += 1;
        }
    }

    println!("\n// ==== counts ====");
    println!("// new ALTERNATES rows: {}", seen.len());
    for (key, n) in &per_key {
        println!("// key {key:?}: +{n}");
    }
    for (n, count) in multiplicity.iter().enumerate() {
        if *count > 0 {
            println!("// cells with {n} form(s): +{count}");
        }
    }
}
```

- [ ] **Step 2: Run the generator**

Run: `mise exec -- cargo test -p panini --test print_7e_goldens -- --nocapture`
Expected: four `PARADIGM` blocks, **7** `ALTERNATES` rows, and counts reading `key "8.4.56": +3`, `key "7.1.35": +2`, `key "7.1.35+8.4.56": +2`, `cells with 1 form(s): +31`, `cells with 2 form(s): +3`, `cells with 3 form(s): +2`.

If the counts differ from those, the derivation differs from the audited one — which cannot happen, since Task 6 certified it. Re-run Task 6 rather than adjusting the numbers.

- [ ] **Step 3: Paste the blocks into `PARADIGM`**

Append the four printed blocks to the end of `PARADIGM` in `crates/panini/tests/paradigm.rs`, after the `07.0013` blocks.

- [ ] **Step 4: Paste the rows into `ALTERNATES`**

Append the seven printed rows to the end of `ALTERNATES`.

- [ ] **Step 5: Empty `GATED`**

Restore `const GATED: &[(&str, &str, Pada)] = &[];` — deleting the temporary comment block Task 3 added along with the four triples. √tṛh now has golden coverage; nothing in this slice ships gated.

- [ ] **Step 6: Update the audited-numbers assertions**

In `derivation_set_shape_matches_the_audited_numbers`:

```rust
    assert_eq!(total_cells, 2628, "292 root×lakāra blocks × 9 cells each");
```

```rust
    assert_eq!(ALTERNATES.len(), 429, "ALTERNATES row count");
```

and update the multiplicity buckets to ones **2324**, twos **211**, threes **79**, fours **2**, fives **6**, sixes **6**, and the per-key counts to `8.4.56` **105**, `7.1.35` **86**, `7.1.35+8.4.56` **86**, with the other seven keys unchanged.

- [ ] **Step 7: Update that test's doc comment**

The doc comment at `:5343–5380` carries the fork census and the audit history as prose. Update the cell/form/root totals to 2628 / `<N>` / 64 and the bucket figures to match Step 6, and append:

```rust
/// and rudhādi 7e's cross-implementation audit re-ran the same probe
/// against vidyut-prakriya at commit `8da2f90` over all 2628 cells /
/// <N> forms / 64 roots with zero differences, its `entry` negative
/// control verified failing. √tṛh joins none of the fork records: its
/// deepest cells hold three forms, because 8.3.13 Qo Qe lopaH
/// obligatorily elides the ḍh that 8.4.65 forks on for every other
/// stop-final rudhādi root.
```

- [ ] **Step 8: Delete the generator**

```bash
rm crates/panini/tests/print_7e_goldens.rs
```

- [ ] **Step 9: Run the full golden suite in the foreground**

Run: `mise run test`
Expected: PASS, ~13 minutes. Do not background it.

- [ ] **Step 10: Commit**

```bash
git add crates/panini/tests/paradigm.rs
git commit -m "test(paradigm): tfh gets its audited paradigm"
```

---

### Task 8: The trace pins

**Files:**
- Modify: `crates/panini/tests/trace.rs`

**Interfaces:**
- Consumes: the audited goldens (Task 7); `cell_trace` and `at`, already in the file.
- Produces: `trneddhi_trace_puts_8_3_13_below_8_4_41`, `trnaddhi_trace_has_8_3_13_and_no_8_4_65`, `atrned_trace_takes_the_im_before_8_2_23_eats_tips_t`.

Each pins an ordering that a plausible-looking reordering would break while still producing real Sanskrit — which is why a golden alone is not enough.

- [ ] **Step 1: Write the three tests**

Append to `crates/panini/tests/trace.rs`:

```rust
#[test]
fn trneddhi_trace_puts_8_3_13_below_8_4_41() {
    // tfh laT prathama eka, the whole im path in one cell: 7.3.92 inserts
    // the Agama, 6.1.87 coalesces it (tfnaih -> tfneh), 8.2.31 takes the
    // `h` to `Q`, 8.2.40 takes ti's `t` to `D`, 8.4.41 retroflexes that
    // `D` to `Q`, and only THEN can 8.3.13 elide the first of the two.
    //
    // The order assertion is the point. 8.3.13's second ḍh is 8.4.41's own
    // output, so in sūtra order the rule would see tfneQ + Di, decline,
    // and the cell would surface *tfReQQi.
    let (text, t) = cell_trace(
        "07.0018",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "tfReQi", "got {t:?}");
    assert!(at(&t, "7.3.92") < at(&t, "6.1.87"), "got {t:?}");
    assert!(at(&t, "8.2.31") < at(&t, "8.2.40"), "got {t:?}");
    assert!(at(&t, "8.4.41") < at(&t, "8.3.13"), "got {t:?}");
}

#[test]
fn trnaddhi_trace_has_8_3_13_and_no_8_4_65() {
    // tfh loT madhyama eka. Every other stop-final rudhAdi root makes this
    // cell a SIX-former: 8.4.53 voices, 8.4.65 Jaro Jari savarRe optionally
    // elides, and 7.1.35 and 8.4.56 multiply that by three. √tṛh's holds
    // three forms, because 8.3.13 obligatorily eats the very ḍh 8.4.65
    // would have forked on.
    //
    // The negative half is the pin: move 8.3.13 below 8.4.65 in
    // `tripadi.rs` and this cell silently grows to six forms, every one of
    // them a plausible word. The ALTERNATES count is the second alarm; this
    // is the one that says why.
    let (text, t) = cell_trace(
        "07.0018",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "tfRQi", "got {t:?}");
    assert!(t.contains(&"8.3.13".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.65".to_string()), "got {t:?}");
    // 6.4.101 huJalByo her DiH supplies the `Di` that 8.4.41 retroflexes;
    // without it there is no second ḍh and 8.3.13 has nothing to elide.
    assert!(at(&t, "6.4.101") < at(&t, "8.4.41"), "got {t:?}");
}

#[test]
fn atrned_trace_takes_the_im_before_8_2_23_eats_tips_t() {
    // tfh laN prathama eka. A cross-STAGE ordering fact that nothing else
    // records: 7.3.92 lives in the `guna` stage and 8.2.23 in `tripadi`,
    // so when 7.3.92 asks whether the following affix is hal-initial, laN
    // tip's apRkta `t` is STILL THERE. Let 8.2.23 saMyogAntasya lopaH run
    // first and ENDING is empty, the hal test fails, and the cell derives
    // *atfRah.
    //
    // vidyut-prakriya credits 6.1.68 hal NyAb Byo dIrGAt sutisyapRktaM hal
    // with that same deletion, here and for every curated rudhAdi root
    // (akfRat, aBinat, apinaw, aBanak). This engine has no 6.1.68 and
    // reaches the same surface by 8.2.23; the divergence predates √tṛh and
    // is audited clean, so it is not this slice's to correct.
    let (text, t) = cell_trace(
        "07.0018",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "atfReq", "got {t:?}");
    assert!(at(&t, "7.3.92") < at(&t, "8.2.23"), "got {t:?}");
    assert!(!t.contains(&"6.1.68".to_string()), "got {t:?}");
}
```

- [ ] **Step 2: Run them**

Run: `mise exec -- cargo test -p panini --test trace`
Expected: PASS, all three.

If `atrned_trace_…` reports `atfRew` rather than `atfReq`, `cell_trace` took the 8.4.56 branch: index 0 is the declined derivation, so `atfReq` (jaśtva-voiced, 8.4.56 not applied) is correct and a `w` means 8.4.56's vikalpa flag or ordering moved. Investigate rather than changing the expected string.

- [ ] **Step 3: Commit**

```bash
git add crates/panini/tests/trace.rs
git commit -m "test(trace): the im path, the fork 8.3.13 pre-empts, and the stage boundary"
```

---

### Task 9: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the mutation paragraph)

**Interfaces:**
- Consumes: the complete slice (Tasks 1–8).
- Produces: the measured floor and campaign numbers Task 10's prose quotes.

- [ ] **Step 1: Measure the uncontended floor**

Run, in the foreground: `mise run test`

Record the per-binary wall-clock: paradigm, roundtrip, trace. **Measure it; do not scale the 2592-cell figure by cell count** — `AGENTS.md` records that the floor has not tracked cell count across slices. Growth here is +1.4%, the smallest in several slices, so a near-flat result is expected but is not a substitute for the measurement.

- [ ] **Step 2: Check the floor against the cap**

`mise run mutants` carries `--timeout 2400`. Confirm the measured uncaught floor (paradigm + roundtrip + trace) times a `-j 4` contention factor of 2.1–2.5× still clears it with margin. At the pada audit's 1872 cells the uncontended floor was ~443s; if this slice measures near that, 2400s has roughly 2× margin and needs no change.

- [ ] **Step 3: Run the campaign**

Run, in the foreground: `mise run mutants`

Expected: **0 missed**. Check `mutants.out/timeout.txt` **as well as** `mutants.out/missed.txt` — a run reported clean on `missed.txt` alone is vacuous if the cap reclassified survivors as timeouts.

Exactly one timeout is expected and correct: `tripadi.rs`'s ṇatva backward scan, whose `j -= 1` mutates to `j /= 1` and never terminates. No assertion can catch it; the cap **is** the detection. Do not chase it with a bigger `--timeout`.

**Any other timeout must be re-run alone** at `-j 1` before any conclusion is drawn — it is a survivor misreported, until proven otherwise.

- [ ] **Step 4: Fix any survivor**

A survivor in the new code is a missing test, not a mutation-tool artifact. The likely shapes, and what they mean:

- A mutated conjunct of **7.3.92**'s guard surviving means the negative control for that conjunct is not actually reachable in a golden — check Task 4's test covers it and that the corresponding cell is in `PARADIGM`.
- A mutated **`is_shtu`** arm surviving is expected for `R` and is exactly why `shtu_is_sha_plus_the_whole_tavarga` exists; if the unit test does not kill it, the test is wrong.
- A mutated **8.3.13** position (`w[i - 1]` → `w[i]`) surviving means no cell distinguishes eliding the first ḍh from the second — `tfReQi` should, since eliding the second gives `tfReQ`.

- [ ] **Step 5: Record the numbers in `AGENTS.md`**

Append to the mutation paragraph: the cell count (2628), the measured floor per binary, and the campaign result (mutants, caught, 0 missed, unviable, and the one known-permanent timeout). Note explicitly whether the floor moved against the 2592-cell measurement, since the paragraph's standing advice is that it does not track cell count.

- [ ] **Step 6: Commit**

```bash
git add AGENTS.md
git commit -m "test: mutation gate at 2628 cells, floor re-measured"
```

---

### Task 10: The documentation sweep

**Files:**
- Modify: `README.md`, `docs/ARCHITECTURE.md`, `AGENTS.md`, `data/ATTRIBUTION.md`

**Interfaces:**
- Consumes: the audit verdict (Task 6), the measured counts (Task 7), the campaign numbers (Task 9).
- Produces: nothing downstream. This is the last content task.

A checklist, not a sweep: past slices have shipped with counts stale in exactly one file. The five sites already touched in earlier tasks (`panini-data`, `paradigm.rs`, `tripadi.rs`, `tools/audit`, `AGENTS.md`'s mutation paragraph) are done; these four are what remain.

- [ ] **Step 1: `README.md`**

- `:18` — rudhādi carries **twenty-two** roots, not twenty-one; add √tṛh to the enumeration.
- `:26` — "**3 of the 25 remain out**", not 4. The three are √chid, √chṛd and √bhuj.
- `:36` — 2592 → **2628** cells, and the fork census: **304** cells hold more than one form, not 299; **211** hold two, **79** hold three; the four/five/six enumerations are unchanged, and √tṛh joins none of them.
- The root total, 63 → **64**, and the form total → `<N>`.

- [ ] **Step 2: `docs/ARCHITECTURE.md`**

In the rudhādi paragraph: move √tṛh out of the deferral list and into the curated enumeration, noting it arrived with 7.3.92, 8.2.31 and 8.3.13 and with the widenings of 8.4.41, 8.2.41 and 6.1.87. Change "twenty-one roots" to twenty-two and the remaining count to three. The sentence beginning "**√tṛh** ... needs three sūtras the engine does not have" must go — it is the claim this slice falsifies.

- [ ] **Step 3: `AGENTS.md`**

In the rudhādi paragraph: twenty-one → twenty-two; "4 of the 25 remain out" → 3; record what 7e found, in the form the 7c/7d entries take. Two things belong in it that are not obvious from the counts:

- 7d's deferral named three sūtras and **undercounted**: three rules the engine already had were too narrow to carry the root, and 8.4.41's own comment had predicted its own widening.
- √tṛh's deepest cells hold **three** forms, not the six every other stop-final rudhādi root reaches, because 8.3.13 obligatorily elides the ḍh 8.4.65 forks on.

Also note the standing 6.1.68/8.2.23 divergence in the form the other cross-implementation notes take, and that 6.3.111 is deliberately unimplemented with its reason recorded at 8.3.13.

- [ ] **Step 4: `data/ATTRIBUTION.md`**

Add a 7e entry in the form the 7a–7d entries take: `07.0018 tfha~` stores `code` `tfh`, which is its it-stripped upadeśa, so no per-entry deviation needs recording.

- [ ] **Step 5: Check for stale counts the checklist might have missed**

```bash
grep -rn "2592\|3014\|twenty-one\|4 of the 25\| 63 roots\|422" README.md AGENTS.md docs/ tools/ crates/ --include=*.md --include=*.rs | grep -v "docs/superpowers/"
```

Expected: no hits outside `docs/superpowers/` (specs and plans are historical records and are **not** updated). A hit anywhere else is a site this checklist missed — fix it and note which file, since the checklist is meant to be complete.

Note that a grep for bare numbers cannot match a count that is wrapped across lines or stated rule-scoped; read the rudhādi paragraphs in all three of `README.md`, `AGENTS.md` and `docs/ARCHITECTURE.md` end to end rather than trusting the grep alone.

- [ ] **Step 6: Run the full suite one last time in the foreground**

Run: `mise run test && mise run lint && mise run fmt-check`
Expected: all PASS.

- [ ] **Step 7: Commit**

```bash
git add README.md docs/ARCHITECTURE.md AGENTS.md data/ATTRIBUTION.md
git commit -m "docs: rudhadi carries twenty-two roots, and tfh is no longer deferred"
```

---

### Task 11: Push, PR, and finish the branch

- [ ] **Step 1: Push and open the PR**

```bash
git push -u origin rudhadi-gana-7e
gh pr create --fill
```

The PR body should state the audit's verdict (vidyut commit, 64 roots / 2628 cells / `<N>` forms, zero differences, negative controls verified), the dump-diff result that gates the widenings, the mutation result (mutants, caught, 0 missed, the one known-permanent timeout), and the one-sentence version of what the slice found: 7d's deferral named three sūtras for √tṛh and undercounted by three, because 8.4.41, 8.2.41 and 6.1.87 were each too narrow to carry it.

- [ ] **Step 2: Finish the branch**

Use the `superpowers:finishing-a-development-branch` skill: wait for CI, merge the green PR, verify the commits are on `main`, then delete the branch.

---

## Deferred, and why

- **√chid and √chṛd (`07.0003`, `07.0008`)**, and with them **6.1.73 *che ca*** (the tuk augment before a `C` after a short vowel) and **8.4.40 *stoḥ ścunā ścuḥ***. Without them their laṅ cells surface `aCinat` where vidyut has `acCinat`. Unchanged by this slice.
- **√bhuj (`07.0017`)**, whose 1.3.66 *bhujo'navane* forks its pada on **sense**, not on an axis this engine models.
- **6.3.111 *ḍhralope pūrvasya dīrgho'ṇaḥ***. Not implemented, and 8.3.13's comment records why: it lengthens a preceding aṇ, and every √tṛh cell presents `e` or `M` there. The slice that curates a root with a short a/i/u before an elided ḍh is the one to add it in, with a witness.
- **6.1.68 *hal ṅyāb bhyo dīrghāt su-ti-sy-apṛktaṁ hal***. Deliberately not implemented; 8.2.23 reaches every surface it would. Verified during design to be a divergence that predates √tṛh, across every curated rudhādi root.
- **8.4.41's correspondence side.** The trigger widened in Task 1; the substitute map still covers only t/T/D, because d/n/s have no witness. A separate claim with separate evidence.
- **Splitting `crates/panini/tests/paradigm.rs`**, now past 5,780 lines with `ALTERNATES` around 429 rows. Worth doing, and its own slice.
- **It-stripping as real sūtras** (1.3.2, 1.3.3, 1.3.5, 1.3.9).
