# rudhādi slice 7c — the four curation-only roots — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate rudhādi's four curation-only ubhayapadī roots — √bhid, √kṣud, √yuj, √tṛd — taking the corpus from 49 to 53 roots, and in doing so run the vidyut audit that three documents already assert the result of.

**Architecture:** No engine change of any kind. Four `Dhatu` rows in `panini-data`, 32 generated `PARADIGM` blocks with their `ALTERNATES` rows, two ṇatva trace tests, one cross-pada ambiguity test, and a documentation sweep. The audit runs **before** any golden is pinned, so a root that turns out not to be byte-identical costs a table row rather than a suite of wrong strings.

**Tech Stack:** Rust 1.97.1 pinned via `mise`; `cargo-mutants` for the mutation gate; `vidyut-prakriya` as the cross-implementation oracle.

**Spec:** `docs/superpowers/specs/2026-08-17-rudhadi-gana-7c-design.md`

**Branch:** `rudhadi-gana-7c` (already created; the spec is committed at `0da8ae5`).

## Global Constraints

- **Toolchain:** rust 1.97.1 via `mise`. Never install Rust globally. Scoped test runs use `mise exec -- cargo test -p <crate>`; `mise run test -- -p <crate>` does **not** scope and runs the whole workspace.
- **No grammar change whatsoever.** No new sūtra, no `Rule` added or reordered, no change to `TINANTA_RULES` or its pinned order, no guard widened. `crates/panini-prakriya/src` is not edited by any task in this plan. If a task seems to require it, stop — the premise has broken; see Task 2's divergence protocol.
- **The vikalpa set stays at seven:** `7.1.35`, `3.4.111`, `6.4.107`, `8.2.74`, `8.2.75`, `8.4.65`, `8.4.56`.
- **Goldens are never hand-authored.** Every `PARADIGM` string and `ALTERNATES` row in this slice comes from the generator in Task 3. Do not retype them from anywhere, including from this plan.
- **Audited vidyut commit:** read it from `data/dhatupatha.tsv`'s own header. It is `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` as of writing; the file is authoritative, not this plan.
- **Mutation gate:** `-j 4 --timeout 2400`, and `timeout.txt` is checked alongside `missed.txt`. `CARGO_MUTANTS_JOBS` in the environment can override `-j`; verify it is unset or ≤ 4.
- **Target counts on a clean audit:** 53 roots, 240 `PARADIGM` blocks, 2160 cells. The `ALTERNATES` row count and the form total are **measured in Task 3**, not predicted here.

## Numbers this slice changes

Current values, for reference when editing. Every one of these is asserted somewhere and will fail loudly if missed.

| Quantity | Current |
|---|---|
| curated roots | 49 |
| `PARADIGM` blocks | 208 |
| cells | 1872 |
| `ALTERNATES` rows | 242 |
| forms | 2114 |
| one-form cells | 1702 |
| two-form cells | 109 |
| three-form cells | 56 |
| four-form cells | 1 |
| five-form cells | 2 |
| six-form cells | 2 |
| key `8.4.56` | 63 |
| key `7.1.35` | 58 |
| key `7.1.35+8.4.56` | 58 |
| key `3.4.111` | 2 |
| key `6.4.107` | 8 |
| key `8.4.65` | 42 |
| key `8.2.75` | 2 |
| key `8.2.74` | 1 |
| key `7.1.35+8.4.65` | 4 |
| key `7.1.35+8.4.65+8.4.56` | 4 |

## File Structure

**Modified:**
- `crates/panini-data/src/lib.rs` — four `Dhatu` rows in `DHATUS`; the `rudhadi_rows_are_the_seven_curated_roots` test (renamed, vector extended, comment rewritten).
- `crates/panini/tests/paradigm.rs` — 32 `PARADIGM` blocks, new `ALTERNATES` rows, updated assertions in `derivation_set_shape_matches_the_audited_numbers`, and the new cross-pada ambiguity test.
- `crates/panini/tests/trace.rs` — a `cell_trace` helper and two ṇatva tests.
- `tools/audit/panini_full_audit.rs` — pinned totals (lines 577–579) and doc comments (lines 12, 24, 54).
- `tools/audit/README.md` — "Last recorded result".
- `mise.toml` — the `mutants` task's `run` line and its comment.
- `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md` — the documentation sweep.

**Created then deleted within Task 3:**
- `crates/panini/tests/print_7c_goldens.rs` — the throwaway golden generator.

---

### Task 1: The four data rows

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (the `DHATUS` array, after the `07.0001` entry that currently ends it; and the `rudhadi_rows_are_the_seven_curated_roots` test)

**Interfaces:**
- Consumes: nothing.
- Produces: four `Dhatu` rows keyed `"07.0002"`, `"07.0006"`, `"07.0007"`, `"07.0009"`, each `Gana::Rudhadi` and `PadaAssignment::Ubhayapada`, with `code` values `"Bid"`, `"kzud"`, `"yuj"`, `"tfd"`. Every later task addresses cells by these dhātupāṭha numbers.

- [ ] **Step 1: Run the panini-data suite to confirm it is green before the change**

```bash
mise exec -- cargo test -p panini-data
```

Expected: PASS. This is the baseline — `curated_pada_agrees_with_upadesha_markers` and `dhatupatha_numbers_resolve_upstream` are the two tests that will judge the new rows, and they must be known-good first.

- [ ] **Step 2: Add the four rows**

In `crates/panini-data/src/lib.rs`, append these four entries to the `DHATUS` array, immediately after the `07.0001` (`ruD`) entry that currently ends it and before the closing `];`.

```rust
    Dhatu {
        // 07.0002 Bi\di~^r vidAraRe. Ubhayapadī by 1.3.72 svaritaYitaH: the
        // `~^` svarita it, with no trailing `~\` for 1.3.12 to pre-empt it.
        // The plainest of slice 7c's four roots — it reaches no rule the
        // gaṇa had not already reached, and is here for coverage rather
        // than as a witness. Coverage is a sufficient reason for a root to
        // exist; the audit in that slice is what earns it its place.
        dhatupatha: "07.0002",
        code: "Bid",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "vidAraRe",
    },
    Dhatu {
        // 07.0006 kzu\di~^r sampezaRe. Ubhayapadī by 1.3.72. Witnesses
        // 8.4.2 awkupvANnumvyavAye'pi under a SIBILANT trigger: in
        // kzuRatti the trigger is the `z` of `kz`, the target is Snam's
        // `n`, and the root's own aw vowel `u` separates them. That is
        // √rudh's shape (ruRadDi, r-u-n) reached through z rather than r,
        // and it makes this the second root to show the strong/weak ṇatva
        // split inside rudhādi -- the one gaṇa where 8.3.24
        // naScApadAntasya Jali is live and bleeds ṇatva off the weak stem.
        // 8.4.2's other curated witnesses (vrIRAti, muzARa) are kryādi,
        // where 8.3.24 never competes.
        dhatupatha: "07.0006",
        code: "kzud",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "sampezaRe",
    },
    Dhatu {
        // 07.0007 yu\ji~^r yoge. Ubhayapadī by 1.3.72. The root that earns
        // its place structurally: it is j-final, so its strong stem reaches
        // 8.2.30 coH kuH (yunagti -> 8.4.55 Kari ca -> yunakti), and 8.2.30
        // is exactly what a later slice must generalise -- in its match AND
        // its substitute, which is a literal 'g' rather than the 1.1.50
        // nearest velar its own comment claims -- to reach √ric and √vic.
        // √bhañj has been that rule's only witness since 7b. Pinning √yuj's
        // 72 cells here gives the generalisation slice a second independent
        // anchor it does not have to build as part of the change it is
        // trying to validate.
        dhatupatha: "07.0007",
        code: "yuj",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "yoge",
    },
    Dhatu {
        // 07.0009 u~tfdi~^r hiMsAnAdarayoH. Ubhayapadī by 1.3.72. The
        // leading `u~` is an it by 1.3.2 upadeSe'j anunAsika it; it is
        // neither anudātta nor Nit, so it never reaches 1.3.12, and udit's
        // own consequence (7.2.56 udito vA, optional iw before ktvA) is not
        // a tiṅanta rule and so cannot touch these four lakāras.
        // Structurally √kṛt: ṇatva here is the ADJACENT 8.4.1 razAByAM no
        // RaH, not √kṣud's 8.4.2 -- tfRatti's trigger `f` sits directly
        // against the `n` with nothing intervening -- and it leans on
        // is_natva_trigger's `f | F` arm, the r-vowels counting as triggers
        // by 1.1.51 uraR raparaH.
        dhatupatha: "07.0009",
        code: "tfd",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "hiMsAnAdarayoH",
    },
```

- [ ] **Step 3: Rewrite the rudhādi row test**

In the same file, replace the whole `rudhadi_rows_are_the_seven_curated_roots` test — attribute, comment, name and body — with this.

```rust
    #[test]
    fn rudhadi_rows_are_the_eleven_curated_roots() {
        // √rudh, the gaṇa's eponym, arrived with 1.3.72 svaritaYitaH and
        // PadaAssignment::Ubhayapada. Slice 7c then added the four roots
        // that needed nothing but a table row and an audit -- √bhid,
        // √kṣud, √yuj and √tṛd, all four ubhayapadī by the same sūtra.
        // That claim ("the engine already derives all 72 cells of each,
        // byte-identical to vidyut") had been repeated in three files and
        // sourced in none until 7c actually ran the audit; see
        // `tools/audit/README.md`'s recorded result.
        //
        // rudhādi also holds `vi\da~\` and `o~vijI~`, whose SLP1 surfaces
        // WOULD have collided with divādi's `vid` and tudādi's `vij` under
        // the retired `id` scheme. Neither is curated, and under number
        // keying the question no longer arises: `07.0013` and `07.0023`
        // would be distinct from `04.0067` and `06.0009` whether or not
        // their surfaces agree.
        //
        // The gaṇa is still PARTIAL: 11 of its 25 dhātupāṭha roots, so
        // FOURTEEN remain out, and they do not all cost the same. √ric and
        // √vic want 8.2.30 coH kuH generalised past the hardcoded `j` ->
        // `g` -- its match reads `j` alone AND its substitute is a literal
        // 'g', so the substitute has to be generalised alongside the match.
        // √chid and √chṛd want 6.1.73 Ce ca plus 8.4.40 stoH ScunA ScuH,
        // which this engine does not have. Nine reachable non-ubhayapadī
        // roots (√śiṣ, √tṛh, √und, √añj, √tañc, √vij, √vṛj, √pṛc, √vid) are
        // simply not curated. And √bhuj is out on different grounds again:
        // 1.3.66 Bujo'navane forks its pada on sense.
        let rows: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.gana == Gana::Rudhadi)
            .map(|d| (d.dhatupatha, d.code, d.pada))
            .collect();
        assert_eq!(
            rows,
            vec![
                ("07.0010", "kft", PadaAssignment::Parasmaipada),
                ("07.0019", "hins", PadaAssignment::Parasmaipada),
                ("07.0012", "Kid", PadaAssignment::Atmanepada),
                ("07.0016", "Banj", PadaAssignment::Parasmaipada),
                ("07.0015", "piz", PadaAssignment::Parasmaipada),
                ("07.0011", "inD", PadaAssignment::Atmanepada),
                ("07.0001", "ruD", PadaAssignment::Ubhayapada),
                ("07.0002", "Bid", PadaAssignment::Ubhayapada),
                ("07.0006", "kzud", PadaAssignment::Ubhayapada),
                ("07.0007", "yuj", PadaAssignment::Ubhayapada),
                ("07.0009", "tfd", PadaAssignment::Ubhayapada),
            ]
        );
    }
```

- [ ] **Step 4: Run the panini-data suite**

```bash
mise exec -- cargo fmt --all && mise exec -- cargo test -p panini-data
```

Expected: PASS, and three tests in particular must pass without being touched:
- `curated_pada_agrees_with_upadesha_markers` — re-derives all four new pada verdicts from the vendored upadeśa. **If this fails, a `pada` field is wrong; fix the row, never the test.**
- `dhatupatha_numbers_resolve_upstream` — it-strips each row's upadeśa and compares against `code`. A failure means a `code` is wrong.
- `rudhadi_rows_are_the_eleven_curated_roots`.

- [ ] **Step 5: Confirm the workspace gate is RED, and red for exactly one reason**

```bash
mise exec -- cargo test -p panini --test paradigm 2>&1 | tail -40
```

Expected: **FAIL** — `paradigm_covers_every_enumerable_cell` reports 32 unpinned (root, lakāra, pada) triples, being the four new roots × 4 lakāras × 2 padas. `derivation_set_shape_matches_the_audited_numbers` still passes at this point (it counts `PARADIGM`, which has not grown).

This red is expected and stays red until Task 3. It is not a licence to skip the audit: Task 2 runs against exactly this state.

- [ ] **Step 6: Commit**

```bash
git add crates/panini-data/src/lib.rs
git commit -m "feat(data): rudhadi's four curation-only roots get their table rows

√bhid, √kṣud, √yuj and √tṛd, all four ubhayapadī by 1.3.72 and all four
verified against the vendored upadeśa by
curated_pada_agrees_with_upadesha_markers. No engine change.

The panini golden suite is expected RED until the goldens land: the four
roots are enumerable but unpinned."
```

---

### Task 2: The cross-implementation audit — the blocking gate

**Files:**
- Modify: `tools/audit/panini_full_audit.rs:577-579` (pinned totals) and its doc comments at lines 12, 24, 54
- Modify: `tools/audit/README.md` ("Last recorded result")

**Interfaces:**
- Consumes: the four `Dhatu` rows from Task 1.
- Produces: a recorded audit verdict, and the measured **roots / cells / forms** totals that Task 3's assertions and Task 7's prose both depend on.

This task decides whether the slice's premise holds. **No golden is written until it passes.**

- [ ] **Step 1: Read the audited commit from the vendored data, not from any doc**

```bash
head -20 /workspace/data/dhatupatha.tsv | grep commit
```

Expected: a line naming the commit (`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` as of writing). Use whatever it prints; if it differs from this plan, the file wins.

- [ ] **Step 2: Clone vidyut at that commit**

```bash
cd /tmp && git clone --filter=blob:none https://github.com/ambuda-org/vidyut vidyut-full
cd /tmp/vidyut-full && git checkout <the commit from Step 1>
```

- [ ] **Step 3: Wire this repo's crates in as dev-dependencies of `vidyut-prakriya`**

Add to `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` under `[dev-dependencies]`:

```toml
panini = { path = "/workspace/crates/panini" }
panini-data = { path = "/workspace/crates/panini-data" }
```

- [ ] **Step 4: Copy the harness in**

```bash
cp /workspace/tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
```

Copy it — do not rewrite it. It is committed precisely so no slice rebuilds it again.

- [ ] **Step 5: Run it once with the OLD totals, to read the new ones**

```bash
cd /tmp/vidyut-full/vidyut-prakriya
mise exec rust@1.97.1 -- cargo run --release --example panini_full_audit 2>&1 | tail -40
```

Expected: the `=== differences ===` section prints **first** and should say `none`; the `=== corpus ===` block then prints the real `roots` / `cells` / `forms` / `live branches` counts; and only then does an assertion panic on the stale `49` / `1872` / `2114`. That panic is expected on this run — the totals print before they are asserted, which is why one run yields both the verdict and the numbers.

**Record the printed `roots`, `cells`, `forms` and `live branches`.** Expected on a clean audit: roots 53, cells 2160. The forms figure is new information — write it down; Task 3 and Task 7 both need it.

**If `differences` is not `none`:** stop. Do not proceed to Step 6. Apply the divergence protocol at the end of this task.

- [ ] **Step 6: Update the harness's pinned totals**

In the copy at `/workspace/tools/audit/panini_full_audit.rs` (the repo's, not the one in `/tmp`), replace lines 577–579:

```rust
    assert_eq!(roots_seen.len(), 53, "curated roots");
    assert_eq!(n_cells, 2160, "cells: 240 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, <FORMS from Step 5>, "forms: 2160 cells + <ALTERNATES> ALTERNATES rows");
```

where `<FORMS>` is the number Step 5 printed and `<ALTERNATES>` is `<FORMS> - 2160`. Both are measurements, not choices.

Also update the three doc comments that repeat the totals: line 12 (`each of the 49 curated roots` → `53`), line 24 (`49 roots, 1872 cells, 2114 forms` → the new triple), line 54 (`Optionally dump the full 1872-cell table` → `2160`).

- [ ] **Step 7: Re-copy and re-run, expecting a clean pass**

```bash
cp /workspace/tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
cd /tmp/vidyut-full/vidyut-prakriya
mise exec rust@1.97.1 -- cargo run --release --example panini_full_audit 2>&1 | tail -20
```

Expected: `AUDIT PASSED: 2160 cells, <FORMS> forms, zero differences.` and exit 0.

- [ ] **Step 8: Prove the harness can fail — the negative control**

```bash
cd /tmp/vidyut-full/vidyut-prakriya
PANINI_AUDIT_PERTURB=entry mise exec rust@1.97.1 -- cargo run --release --example panini_full_audit 2>&1 | tail -20
```

Expected: **exit 1**, with all 36 of √bhū's cells flagged `Bavati` vs `paWati`.

This step is not optional and not a formality. The entire reason this slice exists is that a clean-audit claim was recorded without a control behind it; recording a new one the same way would reproduce the exact fault being fixed.

- [ ] **Step 9: Record the result in the harness README**

Replace the "Last recorded result" section of `tools/audit/README.md`:

```markdown
## Last recorded result

2026-08-17, rudhādi slice 7c, vidyut `8da2f90`:
**zero differences across 2160 cells / <FORMS> forms / 53 roots**, with the
`entry` negative control verified failing. This is the run that first sourced
the "√bhid, √kṣud, √yuj and √tṛd derive byte-identically" claim, which until
7c appeared in three files and rested on no recorded audit.
```

Substitute the measured `<FORMS>`, and the commit from Step 1 if it differs.

- [ ] **Step 10: Commit**

```bash
git add tools/audit/panini_full_audit.rs tools/audit/README.md
git commit -m "test(audit): the four roots' byte-identity stops being an unsourced claim

Three files asserted that √bhid, √kṣud, √yuj and √tṛd derive all 72 cells
byte-identically to vidyut. No spec, plan or commit recorded the run. This
is that run: zero differences across 2160 cells / 53 roots at vidyut
8da2f90, with the entry negative control verified failing."
```

**Divergence protocol — if Step 5 reports any difference.** Do not widen a guard, do not add a sūtra, and do not adjust a golden to match. Instead:

1. Identify which root the differing cells belong to and remove **that root's** `Dhatu` row (Task 1), leaving the others.
2. Re-run Steps 5–8 with the reduced set; the expected roots/cells drop by 1 and 72 respectively per dropped root.
3. Carry the finding into Task 7's documentation sweep: the dropped root's entry in the "curation-only" prose is replaced by the specific sūtra or guard it actually needs, sourced to this audit run.
4. Continue the plan with the reduced root set. Every subsequent count changes accordingly; they are all measured, never assumed. Shipping three roots is a success.

---

### Task 3: The goldens, generated

**Files:**
- Create then delete: `crates/panini/tests/print_7c_goldens.rs`
- Modify: `crates/panini/tests/paradigm.rs` (`PARADIGM`, `ALTERNATES`, and `derivation_set_shape_matches_the_audited_numbers`)

**Interfaces:**
- Consumes: the four `Dhatu` rows (Task 1); the audit verdict and form total (Task 2).
- Produces: 32 `PARADIGM` blocks and their `ALTERNATES` rows; the measured cell-multiplicity distribution and per-key counts that Task 7's prose quotes.

- [ ] **Step 1: Write the throwaway generator**

Create `crates/panini/tests/print_7c_goldens.rs`:

```rust
//! THROWAWAY — slice 7c only. Prints the four new roots' `PARADIGM` blocks
//! and `ALTERNATES` rows as Rust source, plus the distribution counts
//! `derivation_set_shape_matches_the_audited_numbers` asserts. Deleted in
//! the same task that pastes its output: goldens are generated from the
//! engine the audit certified, never hand-authored.
//!
//! Run with:
//!   mise exec -- cargo test -p panini --test print_7c_goldens -- --nocapture

mod common;

use common::{CELLS, LAKARA_BY_NAME};
use panini_data::{Pada, dhatus};
use panini_prakriya::derive;

const NEW_ROOTS: [&str; 4] = ["07.0002", "07.0006", "07.0007", "07.0009"];

/// Mirrors `VIKALPA_RULES` in `paradigm.rs`. An alternate's key is the
/// `+`-joined list of optional rules its branch actually applied, which is
/// what `every_alternate_names_the_vikalpa_rules_that_produced_it` checks.
const VIKALPA_RULES: &[&str] = &[
    "7.1.35", "3.4.111", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56",
];

#[test]
fn print_7c_goldens() {
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
mise exec -- cargo test -p panini --test print_7c_goldens -- --nocapture 2>&1 | tee /tmp/7c-goldens.txt
```

Expected: PASS, printing 32 `PARADIGM` block lines, the `ALTERNATES` rows, the multiplicity distribution over 288 new cells, and per-key counts.

Sanity-check four strings against the spec's expectations before trusting the rest — these are the laṭ prathama eka parasmaipada cells: `Binatti` (`07.0002`), `kzuRatti` (`07.0006`), `yunakti` (`07.0007`), `tfRatti` (`07.0009`). If any differs, stop: Task 2 passed, so a mismatch here means the generator addresses cells wrongly, not that the engine is wrong.

- [ ] **Step 3: Paste the blocks and rows into `paradigm.rs`**

Append the 32 printed `PARADIGM` block lines to the end of the `PARADIGM` array (before its closing `];`), and the printed `ALTERNATES` rows to the end of the `ALTERNATES` array. Paste them verbatim from `/tmp/7c-goldens.txt`; do not retype. Formatting is `rustfmt`'s job, not yours.

- [ ] **Step 4: Update the audited-numbers assertions**

In `derivation_set_shape_matches_the_audited_numbers`, replace each assertion's expected value with the old value plus the generator's corresponding new count:

```rust
    assert_eq!(total_cells, 2160, "240 root×lakāra blocks × 9 cells each");
```

then, for each multiplicity `n`, `new_total = old_total + <n-form cells from Step 2>` using the table in "Numbers this slice changes" for the old totals (1702, 109, 56, 1, 2, 2). Likewise `ALTERNATES.len()` becomes `242 + <ALTERNATES row count from Step 2>`, and each `key_count(...)` becomes its old value plus the generator's `key <k>` count (keys the generator does not list are unchanged).

**Cross-check before moving on:** `2160 + ALTERNATES.len()` must equal the `forms` figure Task 2 Step 5 printed. If it does not, the goldens and the audit disagree about the corpus and something was pasted wrong — resolve it here, not later.

If the generator reports a multiplicity bucket that does not currently exist in the test (a 7-form cell), stop and report it: that would be a sharper fork than anything in the repo and needs its own discussion, not a silently added `assert_eq!`.

- [ ] **Step 5: Update the doc comment above that test**

The long comment above `derivation_set_shape_matches_the_audited_numbers` enumerates which roots fork where and names the record-holding cells. Extend it with a sentence naming slice 7c's four roots, and correct `1872 cells total (208 root×lakāra blocks × 9)` to the new figures. If the generator's distribution shows a new five- or six-form cell, the "sharpest branch-count witnesses in the repo" sentence must name it too — and `docs/ARCHITECTURE.md` says the same thing, so note it for Task 7.

- [ ] **Step 6: Delete the generator**

```bash
rm crates/panini/tests/print_7c_goldens.rs
```

It is throwaway by construction: it duplicates `VIKALPA_RULES`, and a second copy that can drift is exactly the kind of thing this repo deletes rather than maintains.

- [ ] **Step 7: Run the full workspace suite**

```bash
mise run fmt && mise run test
```

Expected: PASS, and specifically these five, which are the ones that judge the goldens:
- `paradigm_covers_every_enumerable_cell` — now green again; every (root, lakāra, pada) triple is pinned.
- `derivation_set_is_exactly_pinned` — the sharp one: each cell's derivation set must be **exactly** golden + alternates.
- `every_alternate_names_the_vikalpa_rules_that_produced_it` — each row's key must be the rules its branch really applied.
- `every_alternate_names_a_real_paradigm_block`.
- `derivation_set_shape_matches_the_audited_numbers`.

This run takes several minutes (paradigm ~205s and roundtrip ~236s at the previous size, and this suite is larger). Run it in the foreground and wait; do not background it.

- [ ] **Step 8: Commit**

```bash
git add crates/panini/tests/paradigm.rs
git commit -m "test(paradigm): the four roots get their audited paradigms

32 blocks, 288 cells, generated from the engine the vidyut audit certified
in the previous commit rather than hand-authored. PARADIGM 208 -> 240
blocks, 1872 -> 2160 cells."
```

---

### Task 4: The ṇatva trace pins

**Files:**
- Modify: `crates/panini/tests/trace.rs` (imports, a new `cell_trace` helper, two tests)

**Interfaces:**
- Consumes: the four `Dhatu` rows and the pinned goldens.
- Produces: `fn cell_trace(number: &str, lakara: Lakara, pada: Pada, purusha: Purusha, vacana: Vacana) -> (String, Vec<String>)` — returns the declined derivation's surface and its sūtra trace.

Goldens pin surfaces, not the path taken to reach them. `tfRatti` is the right string whether ṇatva arrived by 8.4.1 or 8.4.2 — and drafting the spec, these two roots were first grouped together as 8.4.2 witnesses before the adjacency of √tṛd's `f` was noticed. These tests pin the distinction.

- [ ] **Step 1: Write the failing tests**

Add to `crates/panini/tests/trace.rs`. First extend the import at line 33:

```rust
use panini::Panini;
use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};
use panini_prakriya::derive;
```

Then add the helper next to `trace_for`:

```rust
/// The trace of one paradigm cell, addressed by COORDINATES rather than by
/// surface string. `trace_for` resolves a word, and a word is ambiguous for
/// an ubhayapadī root whose two padas can share a surface — these ṇatva pins
/// care about a specific (root, lakāra, pada, cell), so they address it
/// directly and read the declined branch's own log.
fn cell_trace(
    number: &str,
    lakara: Lakara,
    pada: Pada,
    purusha: Purusha,
    vacana: Vacana,
) -> (String, Vec<String>) {
    let d = dhatus()
        .iter()
        .find(|d| d.dhatupatha == number)
        .unwrap_or_else(|| panic!("{number} is not a curated root"));
    let p = derive(d, lakara, pada, purusha, vacana)
        .into_iter()
        .next()
        .expect("every enumerable cell derives at least one branch");
    (p.text(), p.log.iter().map(|s| s.sutra.clone()).collect())
}
```

And the two tests:

```rust
#[test]
fn kshud_natva_is_the_intervening_arm_under_a_sibilant_trigger() {
    // 07.0006 kzu\di~^r. The strong stem's trigger is the `z` of `kz`, the
    // target is Snam's `n`, and the root's own aw vowel `u` separates them,
    // so this is 8.4.2 awkupvANnumvyavAye'pi and NOT the adjacent 8.4.1.
    // That is √rudh's shape (ruRadDi, r-u-n) reached through a sibilant
    // rather than an r — 8.4.2's other curated witnesses (vrIRAti, muzARa)
    // are kryādi, where 8.3.24 never competes.
    let (strong, t) = cell_trace(
        "07.0006",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert!(t.contains(&"8.4.2".to_string()), "{strong}: got {t:?}");
    assert!(!t.contains(&"8.4.1".to_string()), "{strong}: got {t:?}");

    // The WEAK stem takes no ṇatva at all: 6.4.111 SnasorallopaH elides
    // Snam's `a`, leaving the nasal directly before a jhal, and 8.3.24
    // naScApadAntasya Jali — gaṇa-guarded to rudhādi and ordered above
    // ṇatva in the tripādī — turns it into an anusvāra before either ṇatva
    // rule looks. Same bleed √rudh shows at runDanti.
    let (weak, t) = cell_trace(
        "07.0006",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert!(!t.contains(&"8.4.1".to_string()), "{weak}: got {t:?}");
    assert!(!t.contains(&"8.4.2".to_string()), "{weak}: got {t:?}");

    // An ĀTMANEPADA strong-stem cell, sanctioned by 1.3.72 rather than
    // 1.3.78, still retroflexes. Without this the two pins above could not
    // tell "ṇatva follows stem strength" from "ṇatva follows pada" — the
    // same reason rudh_natva_follows_stem_strength_not_pada includes ruRaDE.
    let (atma, t) = cell_trace(
        "07.0006",
        Lakara::Lot,
        Pada::Atmanepada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert!(t.contains(&"1.3.72".to_string()), "{atma}: got {t:?}");
    assert!(t.contains(&"8.4.2".to_string()), "{atma}: got {t:?}");
}

#[test]
fn trd_natva_is_the_adjacent_arm_through_an_r_vowel_trigger() {
    // 07.0009 u~tfdi~^r. Structurally √kṛt, NOT √kṣud: the trigger `f` sits
    // directly against Snam's `n` with nothing intervening, so this is the
    // adjacent 8.4.1 razAByAM no RaH and not 8.4.2. It leans on
    // is_natva_trigger's `f | F` arm — the r-vowels counting as triggers by
    // 1.1.51 uraR raparaH, which until now existed for kryādi's √vṛ.
    let (strong, t) = cell_trace(
        "07.0009",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert!(t.contains(&"8.4.1".to_string()), "{strong}: got {t:?}");
    assert!(!t.contains(&"8.4.2".to_string()), "{strong}: got {t:?}");

    // Weak stem: 8.3.24 bleeds ṇatva, exactly as for √kṣud and √rudh.
    let (weak, t) = cell_trace(
        "07.0009",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert!(!t.contains(&"8.4.1".to_string()), "{weak}: got {t:?}");
    assert!(!t.contains(&"8.4.2".to_string()), "{weak}: got {t:?}");

    // Ātmanepada strong stem, sanctioned by 1.3.72: still retroflexes.
    let (atma, t) = cell_trace(
        "07.0009",
        Lakara::Lot,
        Pada::Atmanepada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert!(t.contains(&"1.3.72".to_string()), "{atma}: got {t:?}");
    assert!(t.contains(&"8.4.1".to_string()), "{atma}: got {t:?}");
}
```

- [ ] **Step 2: Run them**

```bash
mise exec -- cargo test -p panini --test trace 2>&1 | tail -20
```

Expected: PASS. These pin behaviour Task 3 already certified against vidyut, so a failure here means an assertion in this plan is wrong about *which arm* fires, not that the engine is broken. Read the printed trace in the failure message and check it against the reasoning in the comment before changing anything — and if the engine really does take the other arm, that is a finding worth reporting, since it contradicts the spec.

- [ ] **Step 3: Commit**

```bash
git add crates/panini/tests/trace.rs
git commit -m "test(trace): the two natva arms are pinned per root, not per surface

kzuRatti takes 8.4.2 (intervening aw vowel, sibilant trigger); tfRatti
takes the adjacent 8.4.1 through its f. Goldens cannot tell these apart —
both strings are right either way — and the spec's first draft grouped
both roots under 8.4.2."
```

---

### Task 5: The cross-pada ambiguity test

**Files:**
- Modify: `crates/panini/tests/paradigm.rs` (one new test at the end)

**Interfaces:**
- Consumes: the pinned `PARADIGM` blocks.
- Produces: a pinned set of pada-ambiguous surfaces that `README.md` quotes in Task 7.

`README.md` enumerates the surfaces that are genuinely pada-ambiguous. That enumeration is a derived fact stated by hand, with nothing verifying it: `roundtrip.rs` asserts only that *some* analysis recovers the input, so it is blind to how many there are. Going from three ubhayapadī roots to seven grows the set, and today it would grow silently.

- [ ] **Step 1: Write the test with a deliberately wrong expectation, to read the real set**

Append to `crates/panini/tests/paradigm.rs`:

```rust
/// The surfaces that are genuinely pada-ambiguous — the same string pinned
/// as both a parasmaipada and an ātmanepada cell, so `check` reports two
/// analyses differing in pada. `README.md` quotes this list; before this
/// test it was hand-maintained prose with nothing behind it, and the
/// ubhayapadī root count going from three to seven in slice 7c is exactly
/// the kind of change that would have grown it silently.
///
/// `roundtrip.rs` cannot serve this purpose: it asks only whether SOME
/// analysis recovers the input, never how many there are.
#[test]
fn pada_ambiguous_surfaces_are_exactly_these() {
    let mut para: Vec<&str> = Vec::new();
    let mut atma: Vec<&str> = Vec::new();
    for (_root, _lakara, pada, forms) in PARADIGM {
        let bucket = match pada {
            Pada::Parasmaipada => &mut para,
            Pada::Atmanepada => &mut atma,
        };
        bucket.extend(forms.iter().copied());
    }

    let mut both: Vec<&str> = para
        .iter()
        .copied()
        .filter(|f| atma.contains(f))
        .collect();
    both.sort_unstable();
    both.dedup();

    assert_eq!(both, Vec::<&str>::new());
}
```

- [ ] **Step 2: Run it to read the actual set**

```bash
mise exec -- cargo test -p panini --test paradigm pada_ambiguous -- --nocapture 2>&1 | tail -30
```

Expected: **FAIL**, with the assertion printing the real list on the left. Copy that list.

This deliberate-failure step exists because the set is a measurement. Guessing it and then adjusting until green would pin whatever the engine happens to do; reading it once and pinning it deliberately is a different act.

- [ ] **Step 3: Pin the measured set**

Replace the final assertion with the measured list, and add a comment naming which roots contribute. `runDAm` (`07.0001`, loṭ) and √nī's and √tud's six (`anayata`, `nayatAm`, `nayeta`, `atudata`, `tudatAm`, `tudeta`) were the pre-7c members and must all still appear — if one has vanished, an existing golden was disturbed and that is a defect, not a new baseline.

```rust
    assert_eq!(
        both,
        vec![
            // paste the sorted list from Step 2 here, one per line
        ]
    );
```

- [ ] **Step 4: Run it green**

```bash
mise exec -- cargo test -p panini --test paradigm pada_ambiguous
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/panini/tests/paradigm.rs
git commit -m "test(paradigm): the pada-ambiguous surfaces get a test, not just prose

README.md enumerated them by hand with nothing checking the list, and
going from three ubhayapadī roots to seven would have grown it silently.
roundtrip.rs cannot catch this: it asks only whether SOME analysis
recovers the input."
```

---

### Task 6: The mutation gate

**Files:**
- Modify: `mise.toml` (the `mutants` task's `run` line and comment)

**Interfaces:**
- Consumes: the complete, green suite from Tasks 1–5.
- Produces: the measured 2160-cell floor and campaign outcome distribution that Task 7 records in `AGENTS.md`.

The mutant set is **unchanged** — mutants come from `crates/panini-prakriya`, which this slice never edits, so the campaign enumerates the pada audit's same 522. What is under test is the suite at 15% more cells: whether any mutant now crosses the cap and is reclassified from MISSED to TIMEOUT. That reclassification is the vacuity failure 7a hit at 1620 cells and 7b hit through `-j 16`, and it makes a "zero survivors" report meaningless.

- [ ] **Step 1: Measure the uncontended floor**

```bash
time mise run test 2>&1 | tail -30
```

Run in the foreground and wait — this takes roughly 8 minutes and must not be backgrounded. Record the per-binary times for `paradigm`, `roundtrip` and `trace`; their sum is the uncontended uncaught floor.

Priors from the pada audit at 1872 cells: paradigm ~205s, roundtrip ~236s, trace ~2s, total ~443s. Scaling to 2160 cells suggests ~510s. **Treat that as a prior to check, not a prediction to trust** — the 1872-cell floor came in slightly *below* the 1800-cell figure, so cell count is demonstrably not a reliable multiplier here.

- [ ] **Step 2: Confirm the environment cannot defeat the cap**

```bash
echo "CARGO_MUTANTS_JOBS=${CARGO_MUTANTS_JOBS:-<unset>}"
```

Expected: `<unset>`, or a value ≤ 4. `cargo mutants` reads `-j` from this variable, so an unqualified cap can be defeated by the environment alone.

- [ ] **Step 3: Fix `mise.toml`'s self-contradiction**

The task ships `--timeout 1200` under a comment four lines above reading "pass `--timeout 2400` explicitly rather than trusting the 1200 default below". Three slices have worked around that by hand. Change the `run` line:

```toml
run = "cargo mutants --package panini-prakriya --test-workspace=true --timeout 2400 -j 4"
```

and amend the comment block so it describes the cap the task now actually uses, rather than instructing the reader to override it. Keep every other line of that comment — the reasoning about `--test-workspace`, the baseline/mutant asymmetry, and the `timeout.txt` discipline is all still true.

- [ ] **Step 4: Run the campaign**

```bash
mise run mutants 2>&1 | tail -40
```

Foreground, and expect hours. Do not background this and end the turn — a backgrounded suite gets orphaned.

- [ ] **Step 5: Check BOTH outcome files**

```bash
wc -l mutants.out/missed.txt mutants.out/timeout.txt mutants.out/caught.txt mutants.out/unviable.txt
cat mutants.out/timeout.txt
```

Expected: `missed.txt` empty, and `timeout.txt` holding **exactly one** entry — the known-permanent `tripadi.rs` ṇatva backward scan, whose `j -= 1` mutates to `j /= 1` so the loop never terminates. That one is the correct verdict at any cap: the mutated run never reaches an assertion, so the timeout itself *is* the detection mechanism. Do not chase it with a bigger `--timeout` and do not change the loop; it is correct working code.

**Any other timeout entry must be re-run alone before any conclusion is drawn** — under too short a cap a genuine survivor is recorded as a timeout, which is exactly how a "zero survivors" result becomes vacuous.

- [ ] **Step 6: Record the timing distribution**

```bash
python3 - <<'EOF'
import json, statistics
outcomes = json.load(open('mutants.out/outcomes.json'))['outcomes']
caught = []
for o in outcomes:
    if o.get('summary') != 'CaughtMutant':
        continue
    for phase in o.get('phase_results', []):
        if phase.get('phase') == 'Test':
            caught.append(phase['duration'])
caught.sort()
def pct(p):
    return caught[min(len(caught) - 1, int(len(caught) * p))]
print(f"caught       : {len(caught)}")
print(f"median       : {statistics.median(caught):.1f}s")
print(f"p90          : {pct(0.90):.1f}s")
print(f"p99          : {pct(0.99):.1f}s")
print(f"max          : {caught[-1]:.1f}s")
print(f"over 600s    : {sum(1 for d in caught if d > 600)}")
print(f"over 1200s   : {sum(1 for d in caught if d > 1200)}")
EOF
```

This script was verified while writing this plan against the pada audit's own `mutants.out/outcomes.json`, where it reproduces AGENTS.md's recorded figures exactly: 482 caught, median 30.0s, p90 346.6s, p99 547.2s, max 754.6s, 4 over 600s, none over 1200s. If the JSON shape has changed under a newer `cargo-mutants`, read the file and adjust — the numbers are what matter, not the script. The pada audit's figures at 1872 cells were median 30.1s, p90 346.6s, p99 547.2s, max 754.6s, with 4 over 600s and none over 1200s. These numbers go into `AGENTS.md` in Task 7 and are what the next slice will scale from.

- [ ] **Step 7: Commit**

```bash
git add mise.toml
git commit -m "build: the mutants task stops contradicting its own comment

It shipped --timeout 1200 under a comment telling the reader to pass 2400
instead; three slices worked around that by hand. At 2160 cells 1200 would
leave roughly 1.4x margin over the projected worst case, against 2400's
~2.8x."
```

---

### Task 7: The documentation sweep

**Files:**
- Modify: `README.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`

**Interfaces:**
- Consumes: every measurement from Tasks 2, 3 and 6.
- Produces: the repo's front-door prose, matching the code.

Every claim below is currently false. Each substitution uses a number measured earlier in this plan — none is invented here.

- [ ] **Step 1: `README.md`**

In the Scope paragraph:
- "*rudhādi* (7, śnam) **partial**: seven of its roots (√kṛt, √hiṃs, √khid, √bhañj, √piṣ, √indh, √rudh) of 25" → eleven roots, listing the four new ones.
- Delete the clause deferring "√bhid, √kṣud, √yuj and √tṛd purely for want of curation" from the remaining-roots sentence, and reduce that sentence to the real remainder: √ric and √vic (8.2.30), √chid and √chṛd (6.1.73, 8.4.40), the nine uncurated reachable roots, and √bhuj. State plainly that **14 of 25 remain out**.
- Every count: `49`-root set → 53; `1872` cells → 2160; `2114` forms → the Task 2 figure; `170 of the 1872 cells hold more than one form` and its breakdown (109 two, 56 three, one four, two five, two six) → the Task 3 distribution.
- "√rudh, √nī and √tud each derive a full parasmaipada and a full ātmanepada paradigm" → all seven ubhayapadī roots.
- The pada-ambiguous surface enumeration → the set Task 5 pinned, and add that it is now checked by `pada_ambiguous_surfaces_are_exactly_these` rather than maintained by hand.

- [ ] **Step 2: `AGENTS.md`**

- The golden-suite line (~135): `1872 cells, six complete gaṇas` → 2160 cells.
- The rudhādi paragraph (~209): rewrite the "**√bhid, √kṣud, √yuj and √tṛd are curation-only**" sentence. It no longer describes deferred roots — it describes what slice 7c curated, and the byte-identity claim now cites the audit run that established it (date, vidyut commit, and the fact that the `entry` control was verified failing). The gaṇa's root count goes seven → eleven, and the closing tally of "what remains" drops the four.
- The recorded audit result (~324): the totals and the run description → Task 2's figures.
- The cargo-mutants paragraph: append the 2160-cell measurements from Task 6 — the uncontended floor, the campaign's outcome counts, and the caught-mutant duration distribution — in the same style as the "**The pada audit measured both at 1872 cells**" passage. Also correct the paragraph's instruction to "pass `--timeout 2400` explicitly rather than trusting the 1200 default", since the task now defaults to 2400.

- [ ] **Step 3: `docs/ARCHITECTURE.md`**

The rudhādi paragraph (~83–110): "The gaṇa carries seven roots" → eleven, naming slice 7c's four; the "**√bhid, √kṣud, √yuj and √tṛd** are curation-only" sentence replaced by what 7c did; the closing "what is left" tally reduced to the real remainder and the 14-of-25 figure. If Task 3's distribution produced a new five- or six-form cell, the "sharpest branch-count witnesses" claim lives here too and must be corrected in step with `paradigm.rs`'s comment.

- [ ] **Step 4: Verify no stale number survives**

```bash
grep -rn "1872\|2114\|seven of its roots\|curation-only\|49 curated\|49 roots" README.md AGENTS.md docs/ARCHITECTURE.md crates/ tools/ data/
```

Expected: no hit that still asserts a pre-7c fact. Hits inside historical spec/plan documents under `docs/superpowers/` are fine and must not be edited — those record what was true when written.

- [ ] **Step 5: Full suite, then commit**

```bash
mise run fmt-check && mise run lint && mise run test && mise run audit
```

Expected: all PASS.

```bash
git add README.md AGENTS.md docs/ARCHITECTURE.md
git commit -m "docs: rudhadi carries eleven roots, and the byte-identity claim is sourced

The four curation-only roots stop being a deferral and become an audited
fact with a run behind it. 49 -> 53 roots, 1872 -> 2160 cells, and the
gaṇa's remaining fourteen are stated as a number rather than left to
'partial'."
```

---

### Task 8: Push, PR, and finish the branch

**Files:** none.

- [ ] **Step 1: Confirm the whole gate is green from a clean state**

```bash
mise run fmt-check && mise run lint && mise run test && mise run audit
```

Expected: all PASS. Do not open the PR on a remembered green — run it.

- [ ] **Step 2: Push and open the PR**

```bash
git push -u origin rudhadi-gana-7c
gh pr create --fill
```

- [ ] **Step 3: Finish the branch**

Use the `superpowers:finishing-a-development-branch` skill.

---

## Deferred, and why

Recorded here so the next slice does not have to re-derive it:

- **8.2.30 *coḥ kuḥ*'s generalisation past the hardcoded `j` → `g`**, and with it √ric and √vic. Both the match (which reads `j` alone) and the substitute (a literal `'g'`, where the comment claims a 1.1.50 nearest-velar substitution) must be generalised in the same slice: widening only the match reaches the right surface `riRakti` through a wrong intermediate `riRagti`. **This slice deliberately left it alone** so that √yuj's 72 audited cells exist as an independent anchor before the rule changes — √bhañj had been its only witness since 7b.
- **6.1.73 *che ca* and 8.4.40 *stoḥ ścunā ścuḥ***, and with them √chid and √chṛd, whose laṅ cells otherwise surface `aCinat` for `acCinat`.
- **The nine reachable non-ubhayapadī rudhādi roots** — √śiṣ, √tṛh, √und, √añj, √tañc, √vij, √vṛj, √pṛc, √vid — each bringing machinery of its own (7.1.58 *idito num dhātoḥ* for √und, 6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti* for √añj and √tañc).
- **√bhuj** (`07.0017`), whose 1.3.66 *bhujo'navane* forks its pada on sense rather than on an axis this engine models.
- **It-stripping as real sūtras** (1.3.2, 1.3.3, 1.3.5, 1.3.9). Four more curated rows strengthen the case without pre-empting the design; upadeśa preprocessing still needs its own pipeline concept, since it is not what `TINANTA_RULES` models.
- **The trace pins cover ṇatva only.** √bhid and √yuj got no trace test: neither reaches a rule whose *choice* is in question, and √yuj's 8.2.30 path is pinned by its goldens — which is precisely what the generalisation slice needs from it.
