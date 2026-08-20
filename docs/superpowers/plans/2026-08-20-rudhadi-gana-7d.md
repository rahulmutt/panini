# rudhādi gaṇa slice 7d — the eight curation-only roots — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate the eight rudhādi roots that need no new sūtra — √śiṣ, √und, √añj, √tañc, √vij, √vṛj, √pṛc and √vid — taking the gaṇa from 13 to 21 of its 25 dhātupāṭha roots.

**Architecture:** No engine change of any kind. Eight `Dhatu` rows in `panini-data`, then the whole corpus is re-audited against `vidyut-prakriya`, then 32 `PARADIGM` blocks and their `ALTERNATES` rows are generated from the engine the audit certified. Three trace pins, a re-measured mutation gate, and a documentation sweep. Because no `Rule` changes, a difference the audit finds has exactly one candidate cause — the engine being wrong about a rule it already has.

**Tech Stack:** Rust 1.98.0 pinned via `mise`; `cargo test` for the golden suite; `cargo-mutants` for the mutation gate; `vidyut-prakriya` (external, via the committed harness at `tools/audit/`) for the cross-implementation audit.

**Spec:** `docs/superpowers/specs/2026-08-20-rudhadi-gana-7d-design.md`

**Branch:** `rudhadi-gana-7d` already exists, with the spec committed as `1cbe282`. Work continues on it; do not branch again.

## Global Constraints

- **Toolchain is pinned, and it moved.** `mise.toml` pins **rust 1.98.0** as of commit `57f886f`, not the 1.97.1 every prior plan names. Use `mise run <task>` or `mise exec -- cargo …`; never install Rust globally. `mise run test -- -p X` does **not** scope to a package — use `mise exec -- cargo test -p X`.
- **Every timing figure in `AGENTS.md` was measured under 1.97.1.** This slice's floor measurement is the first under 1.98.0, so a shift is not necessarily suite growth. Say so when recording it.
- **No engine code changes.** No file under `crates/panini-prakriya/src/tinanta/` gains, loses, or alters a `Rule`. `TINANTA_RULES` and its pinned order are untouched; the vikalpa set stays at seven. The one permitted edit under `panini-prakriya` is a stale *comment* in `sound.rs`'s unit test (Task 7).
- **Goldens are generated, never hand-authored.** Every `PARADIGM` block and `ALTERNATES` row comes out of the throwaway generator in Task 3, run against the engine the audit certified. Surfaces quoted in this document exist to make a wrong result recognisable; they are **not** to be typed into a test.
- **The audit's negative control runs first.** A zero-difference result recorded without a verified-failing control proves nothing.
- **Run the golden suite in the FOREGROUND.** It takes ~12 minutes and will take longer after Task 3. Do not background it and do not end a turn while it runs; a backgrounded suite gets orphaned and its result is lost.
- **`mise run mutants` is `-j 4 --timeout 2400`.** Run the task; do not reconstruct the flags. `cargo-mutants` reads `-j` from `CARGO_MUTANTS_JOBS`, so the environment can defeat the cap.
- **SLP1 throughout.** `M` is anusvāra, `N` velar ṅ, `Y` palatal ñ, `R` retroflex ṇ, `z` retroflex ṣ, `q`/`Q` retroflex ḍ/ḍh.

## Numbers this slice changes

Old values, for the arithmetic in Tasks 1, 2 and 3. Every one of these is asserted somewhere and will fail loudly if missed.

| quantity | old | new |
|---|---|---|
| `dhatus().len()` | 55 | **63** |
| `PARADIGM.len()` (blocks) | 256 | **288** |
| cells (`PARADIGM.len() * 9`) | 2304 | **2592** |
| rudhādi curated roots | 13 | **21** |
| rudhādi entries still out | 12 | **4** |
| ubhayapadī curated roots | 9 | **9 (unchanged)** |
| `ALTERNATES.len()` | 350 | measured in Task 3 |
| forms (cells + alternates) | 2654 | measured in Task 2 |

Cell-multiplicity buckets in `derivation_set_shape_matches_the_audited_numbers`, old values: ones **2056**, twos **172**, threes **65**, fours **1**, fives **5**, sixes **5**.

`ALTERNATES` key counts, old values: `8.4.56` **81**, `7.1.35` **70**, `7.1.35+8.4.56` **70**, `3.4.111` **2**, `6.4.107` **8**, `8.4.65` **93**, `8.2.75` **5**, `8.2.74` **1**, `7.1.35+8.4.65` **10**, `7.1.35+8.4.65+8.4.56` **10**.

The spec projects 422 `ALTERNATES` rows and 3014 forms from a probe. Those are **expectations to recognise a wrong result by**, not values to type in. Task 2 and Task 3 measure them.

## File Structure

| file | responsibility | task |
|---|---|---|
| `crates/panini-data/src/lib.rs` | eight `Dhatu` rows; `rudhadi_rows_…` renamed and extended; `dhatus().len()`; the deferral comment | 1 |
| `tools/audit/panini_full_audit.rs` | corpus-total assertions and the module header's totals | 2 |
| `tools/audit/README.md` | "Last recorded result" | 2 |
| `crates/panini/tests/print_7d_goldens.rs` | throwaway generator, created and deleted in Task 3 | 3 |
| `crates/panini/tests/paradigm.rs` | `PARADIGM`, `ALTERNATES`, the audited-numbers test and its doc comment | 3 |
| `crates/panini/tests/paradigm.rs` | `pada_ambiguous_surfaces_are_exactly_these` | 4 |
| `crates/panini/tests/trace.rs` | the three new pins | 5 |
| `AGENTS.md` | the mutation paragraph | 6 |
| `crates/panini-prakriya/src/tinanta/sound.rs` | the stale "never a/A/u/U/o/O" comment | 7 |
| `README.md`, `docs/ARCHITECTURE.md`, `AGENTS.md`, `data/ATTRIBUTION.md` | prose, counts, recorded results | 7 |

**Expected-red window.** Task 1 adds eight `Dhatu` rows with no `PARADIGM` blocks behind them, so `paradigm_covers_every_enumerable_cell` **fails from Task 1 until Task 3**. This is intended and is how every prior gaṇa slice sequenced: the roots must be derivable before the audit can certify them, and nothing is pinned before the audit certifies it. Do not "fix" it by hand-authoring blocks.

---

### Task 1: The eight data rows

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (the end of `DHATUS`, `rudhadi_rows_are_the_thirteen_curated_roots`, the `dhatus().len()` assertion at line 655)

**Interfaces:**
- Consumes: nothing.
- Produces: eight curated roots addressable by `Dhatu::dhatupatha` — `"07.0014"`, `"07.0020"`, `"07.0021"`, `"07.0022"`, `"07.0023"`, `"07.0024"`, `"07.0025"`, `"07.0013"` — which Tasks 2, 3 and 5 all resolve through `dhatus()`.

- [ ] **Step 1: Write the failing table test**

Rename `rudhadi_rows_are_the_thirteen_curated_roots` to `rudhadi_rows_are_the_twenty_one_curated_roots` and replace its comment block and its expected vector. Keep the existing thirteen entries in their current order and append the eight; the assertion compares against `DHATUS` order, so the appended rows must appear in the order Step 2 inserts them.

Replace the comment block that begins `// √rudh, the gaṇa's eponym, arrived with 1.3.72` and ends `// 1.3.66 Bujo'navane forks its pada on sense.` with:

```rust
        // √rudh, the gaṇa's eponym, arrived with 1.3.72 svaritaYitaH and
        // PadaAssignment::Ubhayapada. Slice 7c added √bhid, √kṣud, √yuj and
        // √tṛd; the 8.2.30/8.2.39 generalization slice added √ric and √vic
        // once 8.2.30 stopped hardcoding its `j` -> `g` pair.
        //
        // Slice 7d adds the eight roots that a probe against
        // vidyut-prakriya showed need NO sūtra this engine lacks: √śiṣ,
        // √und, √añj, √tañc, √vij, √vṛj, √pṛc and √vid. The probe compared
        // the sūtras each root's derivations invoke against this engine's
        // implemented set; `tools/audit/README.md`'s recorded result is
        // what turned that into a byte-for-byte verdict.
        //
        // `vi\da~\` and `o~vijI~` are the two entries whose SLP1 surfaces
        // WOULD have collided with divādi's `vid` and tudādi's `vij` under
        // the retired `id` scheme. Both are curated here, and under number
        // keying the question does not arise: `07.0013` and `07.0023` are
        // distinct from `04.0067` and `06.0009` whether or not their
        // surfaces agree. This is the slice that would have paid for the
        // retired scheme, and does not.
        //
        // The gaṇa is still PARTIAL: 21 of its 25 dhātupāṭha roots, so FOUR
        // remain out, and they do not all cost the same. √tṛh wants 7.3.92
        // tfRaha im with 8.2.31 ho QaH and 8.3.13 Qo Qe lopaH -- slice 7e.
        // √chid and √chṛd want 6.1.73 Ce ca plus 8.4.40 stoH ScunA ScuH.
        // And √bhuj is out on different grounds again: 1.3.66 Bujo'navane
        // forks its pada on sense.
```

Then append these eight tuples to the expected vector, after the `("07.0005", "vic", PadaAssignment::Ubhayapada),` line:

```rust
                ("07.0014", "Siz", PadaAssignment::Parasmaipada),
                ("07.0020", "und", PadaAssignment::Parasmaipada),
                ("07.0021", "anj", PadaAssignment::Parasmaipada),
                ("07.0022", "tanc", PadaAssignment::Parasmaipada),
                ("07.0023", "vij", PadaAssignment::Parasmaipada),
                ("07.0024", "vfj", PadaAssignment::Parasmaipada),
                ("07.0025", "pfc", PadaAssignment::Parasmaipada),
                ("07.0013", "vid", PadaAssignment::Atmanepada),
```

- [ ] **Step 2: Run it to verify it fails**

```bash
mise exec -- cargo test -p panini-data rudhadi_rows
```

Expected: FAIL — the left vector has thirteen entries, the right twenty-one.

- [ ] **Step 3: Add the eight rows**

Insert into `DHATUS` immediately after the `07.0005` (√vic) row, before the closing `];`:

```rust
    Dhatu {
        // 07.0014 Si\zx~ viSezaRe. Structurally √piṣ (07.0015) with a
        // different head: both are z-final, so both drive 8.4.41 zwutva
        // (Sinazwi, the dental of `ti` retroflexed next to the root's z)
        // and 8.2.41 (the z replaced by k before an s-initial affix).
        // Curated as the witness that the z path is not piṣ-specific.
        dhatupatha: "07.0014",
        code: "Siz",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "viSezaRe",
    },
    Dhatu {
        // 07.0020 undI~ kledane. VOWEL-INITIAL and u-headed, which is what
        // makes it worth curating: its laN takes AT (6.4.72) and then
        // 6.1.90 AwaS ca, whose `u` -> `O` arm no curated root had ever
        // reached -- `vrddhi_of_ac_vowels_all_arms` in panini-prakriya's
        // sound.rs says in as many words that only e/I/E inputs occur.
        // Onad is the counterexample. The root's own `n` is 6.4.23's, and
        // 6.4.111 then takes śnam's `a`, exactly as for √bhañj.
        dhatupatha: "07.0020",
        code: "und",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "kledane",
    },
    Dhatu {
        // 07.0021 anjU~ vyaktimrakzaRakAntigatizu. Vowel-initial like
        // √und, and the 8.2.30 witness among the four nasal-tailed roots
        // here: anaj -> anj (6.4.111) -> ang (8.2.30) -> aNk, the `j` arm
        // of kutva_of on a stem whose nasal 6.4.23 has already thinned.
        dhatupatha: "07.0021",
        code: "anj",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyaktimrakzaRakAntigatizu",
    },
    Dhatu {
        // 07.0022 tancU~ saNkocane. The consonant-initial contrast to
        // √añj: same nasal tail, same 6.4.23, and a `c` rather than a `j`
        // for 8.2.30 -- so kutva_of's two cu arms are both driven by roots
        // of the same shape, differing only in voicing.
        dhatupatha: "07.0022",
        code: "tanc",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "saNkocane",
    },
    Dhatu {
        // 07.0023 o~vijI~ BayacalanayoH. The first `o~`-initial upadeśa in
        // the table. Nothing new is needed for it: 1.3.2's anunāsika-it
        // loop in strip_anubandhas takes `o~` like any other vowel + `~`
        // pair, and `curated_pada_agrees_with_upadesha_markers` checks the
        // verdict rather than trusting it.
        dhatupatha: "07.0023",
        code: "vij",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "BayacalanayoH",
    },
    Dhatu {
        // 07.0024 vfjI~ varjane. f-headed, so śnam's own `n` retroflexes
        // by 8.4.1 raSAByAM no RaH -- vfRakti. The minimal contrast to
        // √pṛc below is the tail, not the trigger: both take ṇatva, and
        // only one of them also drives 8.2.30 on a `c`.
        dhatupatha: "07.0024",
        code: "vfj",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "varjane",
    },
    Dhatu {
        // 07.0025 pfcI~ samparke. ṇatva by 8.4.1 like √vṛj, and 8.2.30 on
        // a `c` like √tañc -- the one curated root that stacks both, so it
        // pins that the ṇatva trigger and the kutva substitution do not
        // interfere. pfRakti / pfNktaH.
        dhatupatha: "07.0025",
        code: "pfc",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "samparke",
    },
    Dhatu {
        // 07.0013 vi\da~\ vicAraRe. Ātmanepada by 1.3.12 on its trailing
        // `~\`, and the gaṇa's second pure-ātmanepadī root after √khid and
        // √indh. Distinct from divādi's `vid` (04.0067) and every other
        // √vid by dhātupāṭha number, not by surface. 8.4.65 Jaro Jari
        // savarRe forks nearly every cell it has (vinte / vintte).
        dhatupatha: "07.0013",
        code: "vid",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vicAraRe",
    },
```

- [ ] **Step 4: Update the row-count assertion**

At `crates/panini-data/src/lib.rs:655`:

```rust
        assert_eq!(dhatus().len(), 63);
```

- [ ] **Step 5: Run the whole `panini-data` suite**

```bash
mise exec -- cargo test -p panini-data
```

Expected: PASS, all of it. Three tests other than the renamed one are load-bearing here and must pass **without being edited**:

- `dhatupatha_numbers_resolve_upstream` — it-strips each upstream upadeśa and compares against `code`. If `Siz`, `und`, `anj`, `tanc`, `vij`, `vfj`, `pfc` or `vid` is rejected, the `code` is wrong, not the test.
- `curated_pada_agrees_with_upadesha_markers` — re-derives all 63 pada verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78. `07.0013`'s ātmanepada and the other seven's parasmaipada must fall out of it.
- the uniqueness and well-formedness checks on `dhatupatha`.

If any of the three fails, **stop**. A `code` or `pada` that has to be argued for is a finding, not a fix: record it and report before going further.

- [ ] **Step 6: Confirm the expected-red window is exactly one test**

```bash
mise exec -- cargo test -p panini --test paradigm 2>&1 | tail -30
```

Expected: `paradigm_covers_every_enumerable_cell` **FAILS**, listing 32 unpinned `(root, lakara, pada)` triples — the eight new roots × four lakāras × one pada each. Everything else passes.

If any *other* test fails, that is not the expected-red window and must be understood before continuing.

- [ ] **Step 7: Commit**

```bash
git add crates/panini-data/src/lib.rs
git commit -m "feat(data): the eight rudhadi roots that need no new sutra

SiZ, und, aYj, taYc, vij, vfj, pfc and vid, taking the gaNa to 21 of 25 and
the table to 63 roots. paradigm_covers_every_enumerable_cell is red from
here until the goldens land, by design: nothing is pinned before the audit
certifies it."
```

---

### Task 2: The cross-implementation audit — the blocking gate

**Files:**
- Modify: `tools/audit/panini_full_audit.rs` (module header totals, the `n_roots` / `n_cells` / `n_forms` assertions, the `PANINI_AUDIT_DUMP` line)
- Modify: `tools/audit/README.md` ("What it commits to" totals, "Last recorded result")

**Interfaces:**
- Consumes: the eight rows from Task 1.
- Produces: a recorded zero-difference verdict (or a sourced difference), and the corpus **forms** total that Task 3 cross-checks its pasted `ALTERNATES` against.

**This task blocks Task 3.** No golden is pinned until the audit certifies the derivations it would pin.

- [ ] **Step 1: Find the vendored commit and get vidyut there**

```bash
head -20 /workspace/data/dhatupatha.tsv | grep -i commit
```

Read the commit from that header rather than trusting any README's copy of it. If `/tmp/vidyut-full` already exists, confirm it is at that commit (`git -C /tmp/vidyut-full log --oneline -1`); otherwise:

```bash
cd /tmp && git clone --filter=blob:none https://github.com/ambuda-org/vidyut vidyut-full
cd /tmp/vidyut-full && git checkout <the commit from that header>
```

- [ ] **Step 2: Wire this repo's crates in and copy the harness**

Append to `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` under `[dev-dependencies]`, if not already present:

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
PANINI_AUDIT_PERTURB=entry mise exec rust@1.98.0 -- cargo run --release --example panini_full_audit
```

Expected: **exit 1**, printing 36 differing cells for √bhū — `Bavati` vs `paWati` and so on. This resolves `01.0001` against `01.0381` (√paṭh), a plausible wrong entry, so a control that passes here would mean the harness cannot see a difference at all.

If this does not fail, stop. Every result after it is worthless.

- [ ] **Step 4: Run the real audit and set the totals**

```bash
cd /tmp/vidyut-full/vidyut-prakriya
mise exec rust@1.98.0 -- cargo run --release --example panini_full_audit 2>&1 | tee /tmp/7d-audit.txt
```

The harness prints the corpus block before it asserts, so the first run tells you the form count even though it then panics on the stale totals:

```
=== corpus ===
roots            : 63
cells            : 2592
forms (set sizes): <N>
```

Take that `<N>`, and in **both** `/workspace/tools/audit/panini_full_audit.rs` and the copy in the vidyut checkout, set the three assertions near line 577:

```rust
    assert_eq!(roots_seen.len(), 63, "curated roots");
    assert_eq!(n_cells, 2592, "cells: 288 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, <N>, "forms: 2592 cells + <N - 2592> ALTERNATES rows");
```

Note the first is `roots_seen.len()`, not an `n_roots` binding. Update the module header at the same time — line 24's `55 roots, 2304 cells, 2654 forms` becomes `63 roots, 2592 cells, <N> forms`, line 27's `plus 350 ALTERNATES rows` becomes `<N - 2592>`, and line 54's `Optionally dump the full 2304-cell table` becomes `2592`.

Then re-run the command above.

Expected on the re-run: `AUDIT PASSED: 2592 cells, <N> forms, zero differences.`

Record `<N>` and `<N> - 2592` — Task 3 needs both.

- [ ] **Step 5: If there ARE differences**

Do not expand the slice. The spec fixes the posture in advance: **ship what passes, defer the rest with a sourced note.**

- Record which root, which cells, and both engines' forms, from the `DIFF` lines.
- Drop the differing root's row (reverting that part of Task 1) and carry it as a deferral naming the actual sūtra at fault — measured from the two traces, not guessed — in the same form the 6.1.73 / 8.4.40 deferral takes today.
- Every count in this plan then reflects seven roots, or six, not eight. **Recompute rather than carrying these numbers forward**, and say so in the prose: a partial slice states its own partiality.
- Because no `Rule` changed in this slice, a difference means the engine is wrong about a rule it already has. That is a finding worth its own investigation and possibly its own slice — not something to patch inside this one.
- The likeliest single candidate is √und or √añj, if vidyut's 6.4.24 turns out to do something our 6.4.111 does not.

Stop and report before continuing to Task 3 in this case.

- [ ] **Step 6: Record the result and commit**

Update `tools/audit/README.md`: line 30's `(55 roots, 2304 cells, 2654 forms)` to the new triple, and replace the "Last recorded result" section with this slice's run — the date, the slice name, the vidyut commit, the cell/form/root totals, and the fact that the `entry` control was verified failing first. Keep the shape of the existing entry. State plainly that this slice changed no `Rule`, which is what makes the verdict a statement about the eight rows alone.

```bash
cd /workspace
git add tools/audit/panini_full_audit.rs tools/audit/README.md
git commit -m "test(audit): the eight roots are byte-identical to vidyut

Whole corpus, 63 roots / 2592 cells / <N> forms, zero differences at vidyut
<commit>, with the entry negative control verified failing first (exit 1, 36
BU cells). No Rule changed in this slice, so the verdict is about the eight
rows and nothing else."
```

---

### Task 3: The goldens, generated

**Files:**
- Create then delete: `crates/panini/tests/print_7d_goldens.rs`
- Modify: `crates/panini/tests/paradigm.rs` (`PARADIGM`, `ALTERNATES`, `derivation_set_shape_matches_the_audited_numbers` and its doc comment)

**Interfaces:**
- Consumes: the eight `Dhatu` rows (Task 1); the audit verdict and form total `<N>` (Task 2).
- Produces: 32 `PARADIGM` blocks and their `ALTERNATES` rows; the measured cell-multiplicity distribution and per-key counts that Task 7's prose quotes.

- [ ] **Step 1: Write the throwaway generator**

Create `crates/panini/tests/print_7d_goldens.rs`:

```rust
//! THROWAWAY -- slice 7d only. Prints the eight new roots' `PARADIGM`
//! blocks and `ALTERNATES` rows as Rust source, plus the distribution
//! counts `derivation_set_shape_matches_the_audited_numbers` asserts.
//! Deleted in the same task that pastes its output: goldens are generated
//! from the engine the audit certified, never hand-authored.
//!
//! Run with:
//!   mise exec -- cargo test -p panini --test print_7d_goldens -- --nocapture

mod common;

use common::{CELLS, LAKARA_BY_NAME};
use panini_data::{Pada, dhatus};
use panini_prakriya::derive;

const NEW_ROOTS: [&str; 8] = [
    "07.0014", "07.0020", "07.0021", "07.0022", "07.0023", "07.0024", "07.0025", "07.0013",
];

/// Mirrors `VIKALPA_RULES` in `paradigm.rs`. An alternate's key is the
/// `+`-joined list of optional rules its branch actually applied, which is
/// what `every_alternate_names_the_vikalpa_rules_that_produced_it` checks.
const VIKALPA_RULES: &[&str] = &[
    "7.1.35", "3.4.111", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56",
];

#[test]
fn print_7d_goldens() {
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

- [ ] **Step 2: Run the generator**

```bash
mise exec -- cargo test -p panini --test print_7d_goldens -- --nocapture 2>&1 | tee /tmp/7d-goldens.txt
```

Expected: PASS, printing 32 `PARADIGM` block lines, the `ALTERNATES` rows, the multiplicity distribution over 288 new cells, and per-key counts.

Sanity-check eight strings before trusting the rest. These are **single-form cells**, so the index-0 branch is the only branch and the comparison is unambiguous — the first seven are laṭ prathama eka parasmaipada, the last is laṭ prathama bahu ātmanepada:

| root | cell | expected |
|---|---|---|
| `07.0014` | laṭ P.E parasmaipada | `Sinazwi` |
| `07.0020` | laṭ P.E parasmaipada | `unatti` |
| `07.0021` | laṭ P.E parasmaipada | `anakti` |
| `07.0022` | laṭ P.E parasmaipada | `tanakti` |
| `07.0023` | laṭ P.E parasmaipada | `vinakti` |
| `07.0024` | laṭ P.E parasmaipada | `vfRakti` |
| `07.0025` | laṭ P.E parasmaipada | `pfRakti` |
| `07.0013` | laṭ P.B ātmanepada | `vindate` |

If any differs, stop: Task 2 passed, so a mismatch here means the generator addresses cells wrongly, not that the engine is wrong.

Do **not** sanity-check the forked cells against the spec. `derivation_set_is_exactly_pinned` wants the *ruleless* branch in `PARADIGM`, which for `07.0013` laṭ prathama eka is `vintte` and not the alphabetically-first `vinte` — the spec quotes derivation sets, the generator emits index 0, and they legitimately differ.

- [ ] **Step 3: Paste the blocks and rows into `paradigm.rs`**

Append the 32 printed `PARADIGM` block lines to the end of the `PARADIGM` array (before its closing `];`), and the printed `ALTERNATES` rows to the end of the `ALTERNATES` array. Paste them verbatim from `/tmp/7d-goldens.txt`; do not retype. Formatting is `rustfmt`'s job, not yours.

- [ ] **Step 4: Update the audited-numbers assertions**

In `derivation_set_shape_matches_the_audited_numbers`:

```rust
    assert_eq!(total_cells, 2592, "288 root×lakāra blocks × 9 cells each");
```

Then, for each multiplicity `n`, `new_total = old_total + <n-form cells from Step 2>`, using the old totals from "Numbers this slice changes" (2056, 172, 65, 1, 5, 5). Likewise `ALTERNATES.len()` becomes `350 + <ALTERNATES row count from Step 2>`, and each `key_count(...)` becomes its old value plus the generator's `key <k>` count (keys the generator does not list are unchanged).

The `fives` and `sixes` assertion messages name the roots that hold those cells. If the generator puts a new cell in either bucket — the spec expects √und's loṭ madhyama eka to join the six-form bucket — extend the message to name it. Do not leave a count that no longer matches its own description.

**Cross-check before moving on:** `2592 + ALTERNATES.len()` must equal `<N>`, the forms figure Task 2 Step 4 recorded. If it does not, the goldens and the audit disagree about the corpus and something was pasted wrong — resolve it here, not later.

If the generator reports a multiplicity bucket that does not currently exist in the test (a 7-form cell), stop and report it: that would be a sharper fork than anything in the repo and needs its own discussion, not a silently added `assert_eq!`.

- [ ] **Step 5: Update the doc comment above that test**

The long comment above `derivation_set_shape_matches_the_audited_numbers` (around lines 4700–4775) states the corpus totals, enumerates which roots fork where, names the record-holding cells, and records the audit verdict. Update all four:

- line ~4742's `2304 cells total (256 root×lakāra blocks × 9), of which 2056 hold exactly one form` → the new figures
- line ~4755's `itself has 350 rows, keyed 81 8.4.56, 70 7.1.35, …` → the new row count and key counts
- line ~4771's `all 2304 cells / 2654 forms / 55 roots with zero differences` → this slice's audit verdict
- line ~4704's `every one of the thirteen rudhādi roots forks in both loṭ and laṅ` — this is a **claim to re-check, not a number to bump.** Confirm against Step 2's output that each of the eight new roots forks in both loṭ and laṅ before writing twenty-one. If any of them does not, the sentence needs rewording, not renumbering.

If the generator's distribution shows a new six-form cell, the sentence naming the sharpest branch-count witnesses must name it too — and `README.md` and `docs/ARCHITECTURE.md` say the same thing, so note it for Task 7.

- [ ] **Step 6: Delete the generator**

```bash
rm crates/panini/tests/print_7d_goldens.rs
```

It is throwaway by construction: it duplicates `VIKALPA_RULES`, and a second copy that can drift is exactly the kind of thing this repo deletes rather than maintains.

- [ ] **Step 7: Run the full golden suite**

```bash
mise exec -- cargo test -p panini
```

Expected: PASS, including `paradigm_covers_every_enumerable_cell`, which closes the expected-red window opened in Task 1. **Foreground; ~12 minutes or more.** Do not background it and do not end the turn while it runs.

- [ ] **Step 8: Commit**

```bash
git add crates/panini/tests/paradigm.rs
git commit -m "test(paradigm): the eight roots get their audited paradigms

32 blocks, 288 cells, generated from the engine the vidyut audit certified
and pasted verbatim. The corpus reaches 2592 cells and 63 roots."
```

---

### Task 4: The pada-ambiguity set, re-derived

**Files:**
- Modify: `crates/panini/tests/paradigm.rs` (`pada_ambiguous_surfaces_are_exactly_these`)

**Interfaces:**
- Consumes: the pasted `PARADIGM` blocks (Task 3).
- Produces: the measured ambiguous-surface list and its count, which Task 7's `README.md` prose quotes.

None of the eight roots is ubhayapadī, so none contributes an ambiguity against *itself*. But the test buckets across **all** roots, not per root, so `07.0013`'s ātmanepada surfaces are now compared against every parasmaipada surface in the corpus. Whether that produces a new entry is a measurement, not a deduction.

- [ ] **Step 1: Run it and see whether it moved**

```bash
mise exec -- cargo test -p panini --test paradigm pada_ambiguous
```

Expected: **PASS** if no cross-root collision appeared, in which case Steps 2 and 3 are no-ops — record "unchanged at twenty-two surfaces" for Task 7 and move on.

If it FAILS, the failure prints the real set. Do not hand-edit toward it from the diff; do Step 2.

- [ ] **Step 2: Re-derive the set from scratch (only if Step 1 failed)**

Temporarily replace the expected vector in the assertion with `Vec::<&str>::new()`, run the test again, and read the complete real set off the failure output. That is the measurement; the diff-derived guess is not. Restore the assertion with the printed set.

- [ ] **Step 3: Extend the comment (only if the set moved)**

The comment above the assertion records where each surface came from, slice by slice. Add a sentence naming which new surface came from which root and why — a cross-root collision between an ātmanepada-only root and a parasmaipada cell elsewhere would be the first of its kind in the suite, and is worth stating as such rather than appending silently.

- [ ] **Step 4: Commit (only if anything changed)**

```bash
git add crates/panini/tests/paradigm.rs
git commit -m "test(paradigm): the pada-ambiguous set, re-measured at 63 roots"
```

If nothing changed, skip the commit and say so; an empty commit is not evidence of a check having run.

---

### Task 5: The trace pins

**Files:**
- Modify: `crates/panini/tests/trace.rs`

**Interfaces:**
- Consumes: the audited goldens (Task 3); `cell_trace` and `at`, already in the file.
- Produces: `unantas_trace_orders_6_4_23_before_6_4_111`, `aunat_trace_takes_the_u_vrddhi_arm`, `anaktas_trace_is_the_kutva_path_on_a_vowel_initial_root`.

The first is the pin the spec calls load-bearing: vidyut credits **6.4.24** for a step this engine credits to **6.4.111**, and nothing in the suite currently asserts that 6.4.23 runs before 6.4.111 — the `anga.rs` comment says reversing them makes the rule unable to tell śnam's `n` from the root's, and no test held it to that.

- [ ] **Step 1: Write the three tests**

Append to `crates/panini/tests/trace.rs`:

```rust
#[test]
fn unantas_trace_orders_6_4_23_before_6_4_111() {
    // und laT prathama dvi. The ORDER pin 6.4.23's own comment in anga.rs
    // asks for and nothing asserted until this slice: 6.4.23 SnAnnalopaH
    // takes the root's `n` out of unand, and only then does 6.4.111
    // SnasorallopaH take śnam's `a`. Reversed, 6.4.111 fires first and
    // 6.4.23 can no longer tell śnam's `n` from the root's.
    //
    // This is also where vidyut-prakriya credits 6.4.24 aniditAM hala
    // upaDAyAH kNiti for the same unad -> und step. It is the wrong credit
    // -- 6.4.24 deletes a nasal upadhā, and after 6.4.23 has run, unad's
    // upadhā is `a` -- so this engine does not implement 6.4.24 at all and
    // must not be "corrected" toward vidyut's history here.
    let (_text, t) = cell_trace(
        "07.0020",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert!(at(&t, "6.4.23") < at(&t, "6.4.111"), "got {t:?}");
    assert!(!t.contains(&"6.4.24".to_string()), "got {t:?}");
}

#[test]
fn aunat_trace_takes_the_u_vrddhi_arm() {
    // und laN prathama eka: AT (6.4.72) then 6.1.90 AwaS ca on a
    // vowel-initial aNga whose first vowel is `u`, so vrddhi_of returns
    // `O`. Every curated root before this slice drove 6.1.90 with e/I/E
    // only -- sound.rs's vrddhi_of_ac_vowels_all_arms says so in as many
    // words -- and this is the first golden derivation to reach the `u`
    // arm. A vrddhi table that mapped `u` to anything else would still
    // pass every unit test in sound.rs and fail here.
    let (text, t) = cell_trace(
        "07.0020",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "Onad", "got {t:?}");
    assert!(at(&t, "6.4.72") < at(&t, "6.1.90"), "got {t:?}");
}

#[test]
fn anaktas_trace_is_the_kutva_path_on_a_vowel_initial_root() {
    // aYj laT prathama dvi: 6.4.23 thins the root's nasal to anaj, 6.4.111
    // takes śnam's `a` to anj, 8.2.30 coH kuH substitutes the velar for the
    // `j`, 8.3.24 nasalises to aMg, 8.4.55 Kari ca devoices to aMk, and
    // 8.4.58 anusvArasya yayi parasavarRaH gives the velar nasal: aNktaH.
    // The whole tripAdi tail on a vowel-initial root, in order.
    let (text, t) = cell_trace(
        "07.0021",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "aNktaH", "got {t:?}");
    assert!(at(&t, "6.4.23") < at(&t, "6.4.111"), "got {t:?}");
    assert!(at(&t, "6.4.111") < at(&t, "8.2.30"), "got {t:?}");
    assert!(at(&t, "8.2.30") < at(&t, "8.3.24"), "got {t:?}");
    assert!(at(&t, "8.3.24") < at(&t, "8.4.55"), "got {t:?}");
    assert!(at(&t, "8.4.55") < at(&t, "8.4.58"), "got {t:?}");
}
```

`cell_trace` addresses a cell by coordinates rather than by surface, which is what these need: `trace_for` resolves a *word*, and `aNktaH` is not guaranteed unique across the corpus.

- [ ] **Step 2: Reconcile the expected surfaces against the pinned goldens**

The two `assert_eq!(text, …)` strings — `Onad` and `aNktaH` — are this document's *expectations*, taken from a probe. Open the `07.0020` laṅ and `07.0021` laṭ `Pada::Parasmaipada` blocks in `PARADIGM` and confirm cell 0 and cell 1 respectively match. **If a golden differs, the golden wins** — the audit certified it and this document did not. Correct the test and note the divergence for Task 7's prose.

Note `Onad`, not `Onat`: `PARADIGM` holds the ruleless branch and 8.4.56 *vāvasāne* is optional, exactly as √bhū's laṅ is pinned `aBavad` with `aBavat` in `ALTERNATES`.

- [ ] **Step 3: Run them**

```bash
mise exec -- cargo test -p panini --test trace
```

Expected: PASS, all three plus the existing 90.

If `unantas_trace_orders_6_4_23_before_6_4_111` panics inside `at` rather than failing an assertion, the named sūtra is absent from the trace entirely — which for 6.4.23 would mean the narrow guard declined this root. That is a real finding about the guard, not a reason to weaken the test: `und`'s SHAP tail is `nd`, which starts with `n` and is squarely inside the guard's stated scope.

- [ ] **Step 4: Commit**

```bash
git add crates/panini/tests/trace.rs
git commit -m "test(trace): 6.4.23 before 6.4.111, and the u arm of vrddhi

The order 6.4.23's own comment demands had never been asserted; und pins it,
and pins that this engine credits 6.4.111 where vidyut credits 6.4.24. Onad
is the first golden derivation to drive 6.1.90 with a `u`."
```

---

### Task 6: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the cargo-mutants paragraph)

**Interfaces:**
- Consumes: everything above.
- Produces: the measured floor and campaign figures Task 7's prose references.

- [ ] **Step 1: Measure the uncontended floor**

```bash
cd /workspace && time mise run test 2>&1 | tee /tmp/7d-floor.txt
```

Run it alone — no mutation campaign, no other load. Record the per-binary times (`paradigm`, `roundtrip`, `trace`) and their total. **Foreground.**

**Measure; do not scale.** Cell count has failed as a multiplier three times: flat from 1800 to 1872 cells, +38% for +15% into 7c's 610.73s, and +13.8% for +6.7% into the last slice's 695.15s (paradigm 321.34s, roundtrip 371.81s, trace 2.00s). This slice adds 288 cells (+12.5%); the spec projects ~850–880s as a tripwire, not as a substitute for the measurement.

**One confound to name explicitly:** this is the first floor measured under **rust 1.98.0**. Every figure in the series above was taken under 1.97.1. If the total lands oddly relative to the trend, the compiler is a candidate cause and the record must say the toolchain changed, not silently attribute the move to cell growth.

- [ ] **Step 2: Sanity-check the cap against the floor**

The `-j 4` contention factor measured by the pada audit is **1.70×**. Multiply Step 1's total by it: that is the projected worst case for an **uncaught** mutant, which is the figure that governs whether a "0 missed" is vacuous — a genuine survivor runs the suite to completion, whereas a caught run can abort the moment it is detected.

If that projection exceeds **2400s**, stop and report before running the campaign. The cap needs raising, and per the spec that is a decision to record in `AGENTS.md` and in `mise.toml` together, not to make silently. The last slice's projection was ~1182s (a 2.03× margin); the spec expects this one near ~1450–1500s (~1.6×).

- [ ] **Step 3: Run the campaign**

```bash
cd /workspace && mise run mutants
```

This runs `cargo mutants --package panini-prakriya --test-workspace=true --timeout 2400 -j 4`. It takes hours. **Foreground; do not background it.**

If the `cargo mutants` mise shim misbehaves, invoke the `cargo-mutants` binary directly rather than through the shim — but keep the same flags.

Expect roughly **527 mutants**, unchanged from the last slice: this slice adds no `panini-prakriya` code, so the mutant population should be identical. A materially different count means something under `crates/panini-prakriya/src/` changed that this slice said it would not.

- [ ] **Step 4: Check BOTH `missed.txt` and `timeout.txt`**

```bash
cd /workspace && cat mutants.out/missed.txt; echo "--- timeouts ---"; cat mutants.out/timeout.txt; wc -l mutants.out/*.txt
```

Expected:
- `missed.txt` — **empty**. Any entry is a real survivor and must be resolved, not accepted.
- `timeout.txt` — **exactly one entry**, and it must be the known-permanent one: the ṇatva backward-scan mutant in `tripadi.rs` that turns `j -= 1` into `j /= 1`, making the loop non-terminating. No assertion can ever catch it — the mutated run never reaches one — so the cap itself is the detection mechanism and this is the correct verdict at any cap.

**Identify it by that shape, not by its line number.** `AGENTS.md` records it as `tripadi.rs:1156:23`. This slice adds no lines to `tripadi.rs`, so it should be unmoved — but confirm by reading the mutant's diff rather than by matching the number:

```bash
grep -n "j /= 1" mutants.out/timeout.txt mutants.out/mutants.json
```

Any *other* timeout must be re-run alone at the same cap before any conclusion is drawn — under contention a real survivor can be misreported as a timeout, which is what makes a careless "0 missed" vacuous.

A **new survivor in `missed.txt` is possible even though no engine code changed**, and is worth stating rather than dismissing: 288 new golden cells can newly kill a mutant, never newly spare one, so a survivor here would mean a mutant that the old corpus caught and the new one does not — which would point at a golden that changed, not at the engine. Investigate before accepting.

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

The over-600s count is the number to watch across slices — it went 4 → 44 → 46 across the pada audit, 7c and the last slice, while the max moved much less.

- [ ] **Step 6: Record it in `AGENTS.md`**

Add a paragraph to the cargo-mutants section in the same shape as the existing "This slice (ric/vic, 8.2.30) re-measured both at 2304 cells" one: the new cell count, the per-binary floor and its total, how it compared to what scaling would have predicted, **that the toolchain moved to 1.98.0 and the series before it was measured under 1.97.1**, the campaign's mutant/caught/missed/unviable/timeout tallies, the duration distribution, and both margins — the measured one against the worst caught mutant, and the projected one against the uncaught floor, each labelled as measured or projected.

Do not delete the earlier paragraphs. The series is the evidence that cell count is not a multiplier.

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md
git commit -m "test: mutation gate at 2592 cells, floor re-measured under 1.98.0

Campaign at -j 4 --timeout 2400, 0 missed, and the one known-permanent
tripadi.rs non-terminating-loop timeout. First floor taken under rust
1.98.0, which the record now says explicitly."
```

---

### Task 7: The documentation sweep

**Files:**
- Modify: `README.md`, `docs/ARCHITECTURE.md`, `AGENTS.md`, `crates/panini-prakriya/src/tinanta/sound.rs`, `data/ATTRIBUTION.md`

**Interfaces:**
- Consumes: every measured number from Tasks 2–6.
- Produces: a repo whose prose matches its tests.

A checklist, not a sweep — past slices have shipped with counts stale in exactly one file. `crates/panini-data/src/lib.rs`, `crates/panini/tests/paradigm.rs`, `tools/audit/panini_full_audit.rs` and `tools/audit/README.md` were already updated in Tasks 1–3; the boxes below are what remains.

- [ ] **Step 1: `README.md`**

- line 18: "thirteen of its roots" → twenty-one, listing the eight with their slice attribution
- line 24: "**12 of the 25 remain out**" → **4**, and the reason list becomes: √tṛh (7.3.92 with 8.2.31 and 8.3.13), √chid and √chṛd (6.1.73 with 8.4.40), √bhuj (1.3.66)
- **delete the "Nine reachable non-ubhayapadī roots are likewise not curated yet" sentence.** It is down to one root, and that root has a named sūtra cost — it belongs in the named-deferral list above, not in a "simply not curated" bucket.
- "over a curated 55-root set" → 63
- cell and form totals → 2592 and `<N>`; the "248 of the 2304 cells hold more than one form" distribution → Task 3 Step 2's measured buckets
- the six-form cell enumeration gains √und's loṭ madhyama eka if Task 3 measured it there; the "nothing forks deeper than six" claim survives unless Task 3 Step 4 found a seven-form cell
- the ubhayapadī list is **unchanged** at nine — none of the eight is ubhayapadī. Do not renumber it.
- the pada-ambiguous surface enumeration → whatever Task 4 measured, including "unchanged"

- [ ] **Step 2: `docs/ARCHITECTURE.md`**

- line 83: "The gaṇa carries thirteen roots" → twenty-one, naming the eight and their slice
- line 133: "rudhādi is already past it at thirteen" → twenty-one
- line 136: "**12 of the 25 in all**" → 4
- **rewrite the "Nine further reachable non-ubhayapadī roots … are simply not curated yet" sentence.** This is the claim the slice most directly falsifies, and the honest replacement says what the probe found: eight of the nine needed no sūtra this engine lacks, and the ninth, √tṛh, needs three.
- "nine of rudhādi's 25 dhātupāṭha roots are ubhayapadī, seven of the nine now curated" is **unchanged** — 7d curates no ubhayapadī root.
- if Task 3 found a new six-form cell, the sharpest-fork sentence

- [ ] **Step 3: `AGENTS.md`**

- the rudhādi section (around lines 268 and 332): the root list and count, and the "12 of the 25 remain out" figure
- suite-size figures around line 207: `2304 + 350 = 2654` → the new triple
- the recorded audit results around lines 298 and 442: add this slice's run
- the mutation paragraph is already done (Task 6)

- [ ] **Step 4: `crates/panini-prakriya/src/tinanta/sound.rs`**

The comment above `vrddhi_of_ac_vowels_all_arms` claims:

> the curated roots only ever drive `vrddhi_of` through 6.1.90 with e/I/E inputs (never a/A/u/U/o/O), leaving those arms unreachable via golden derivations

√und's `Onad` falsifies it for `u`. Amend it to say the `u` arm is now reached by a golden derivation (naming `aunat_trace_takes_the_u_vrddhi_arm`), while the remaining arms stay unit-test-only. **Keep the test itself** — its point is total coverage of the table, and it is still the only thing covering `a`/`A`/`U`/`o`/`O`.

This is a comment change, not a code change; the "no engine code changes" constraint stands.

- [ ] **Step 5: `data/ATTRIBUTION.md`**

Two edits.

First, add a cross-reference bullet for the eight, in the shape the 7a and 7b bullets take, naming all eight dhātupāṭha numbers with their upadeśas. State that none of the eight stores a `code` differing from a plain it-strip of its vendored upadeśa — unlike `07.0019`, stored post-7.1.58 — so the slice records no per-entry deviation. Confirm that before writing it:

```bash
grep -E '^07\.00(13|14|20|21|22|23|24|25)' /workspace/data/dhatupatha.tsv
```

`dhatupatha_numbers_resolve_upstream` already proved it in Task 1 Step 5; this bullet records it.

Second, the "The gaṇa is **partial**" bullet at the end. Its ubhayapadī arithmetic is untouched by this slice, but its framing — which currently treats the ubhayapadī axis as the whole story — should name what 7d changed: the non-ubhayapadī roots are now curated but for √tṛh.

- [ ] **Step 6: Verify the prose against the tests**

```bash
cd /workspace && grep -rn "2304\|2654\|55 roots\|thirteen\|12 of the 25\|Nine further reachable\|Nine reachable" \
  README.md docs/ARCHITECTURE.md AGENTS.md tools/audit/README.md \
  crates/panini-data/src/lib.rs crates/panini/tests/paradigm.rs \
  crates/panini-prakriya/src/tinanta/sound.rs data/ATTRIBUTION.md
```

Expected: every remaining hit is a deliberate historical reference — a past slice's recorded measurement, such as the 7c or ric/vic floor paragraphs in `AGENTS.md` — not a current claim. Anything stating a present-tense fact must have been updated.

- [ ] **Step 7: Full suite, lint, format**

```bash
cd /workspace && mise run fmt-check && mise run lint && mise run test && mise run audit
```

Expected: all PASS. **Foreground.**

- [ ] **Step 8: Commit**

```bash
git add -A
git commit -m "docs: rudhadi carries twenty-one roots, and the nine become one

The 'nine reachable roots simply not curated' claim is retired in every file
that made it: eight needed no sutra this engine lacks, and tfh needs three.
Counts reach 63 roots and 2592 cells, and sound.rs stops claiming the u arm
of vrddhi_of is unreachable."
```

---

### Task 8: Push, PR, and finish the branch

**Files:** none.

- [ ] **Step 1: Confirm the tree is clean and the branch is coherent**

```bash
cd /workspace && git status --short && git log --oneline main..HEAD
```

Expected: clean tree, and a commit series running spec → rows → audit → goldens → (ambiguity) → traces → mutants → docs.

- [ ] **Step 2: Push and open the PR**

```bash
git push -u origin rudhadi-gana-7d
gh pr create --fill
```

The PR body should state the audit's verdict (vidyut commit, 63 roots / 2592 cells / `<N>` forms, zero differences, negative control verified), the mutation result (mutants, caught, 0 missed, the one known-permanent timeout), and the one-sentence version of what the slice found: eight of the nine "reachable but uncurated" rudhādi roots needed no sūtra this engine lacks, which is why the slice changed no `Rule` at all.

- [ ] **Step 3: Finish the branch**

Use the `superpowers:finishing-a-development-branch` skill: wait for CI, merge the green PR, verify the commits are on `main`, then delete the branch.

---

## Deferred, and why

- **√tṛh (`07.0018`)**, and with it **7.3.92 *tṛṇaha im*, 8.2.31 *ho ḍhaḥ* and 8.3.13 *ḍho ḍhe lopaḥ*.** Slice 7e. 7.3.92 is a mit-āgama placed by 1.1.47 and would be the engine's first āgama of that kind, which is why it is not folded in here.
- **6.1.73 *che ca* and 8.4.40 *stoḥ ścunā ścuḥ***, and with them √chid and √chṛd. Unchanged by this slice.
- **√bhuj (`07.0017`)**, whose 1.3.66 *bhujo'navane* forks its pada on sense rather than on an axis this engine models.
- **6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti ca*.** Deliberately not implemented. Task 5's first pin asserts its absence. If a future root genuinely needs it — one whose nasal upadhā survives to a kṅit affix without śnam having reordered the stem — that is the slice to add it in, with a witness.
- **Splitting `crates/panini/tests/paradigm.rs`**, now past 5,400 lines with `ALTERNATES` around 422 rows. Worth doing, and its own slice: a large mechanical diff inside a curation slice would sit directly next to the data the audit exists to validate.
- **It-stripping as real sūtras** (1.3.2, 1.3.3, 1.3.5, 1.3.9). `o~vijI~` makes the case slightly stronger without pre-empting the design.
