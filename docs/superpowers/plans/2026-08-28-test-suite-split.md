# Test-Suite Split Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Split `crates/panini/tests/paradigm.rs` (6,428 lines) and
`crates/panini/tests/trace.rs` (1,838 lines) into per-gaṇa directory-form
modules with zero behavior change, proven by byte-identical golden dumps and
an identical test inventory.

**Architecture:** Cargo directory form — `tests/paradigm/main.rs` and
`tests/trace/main.rs` each stay ONE test binary (two binaries before and
after, so AGENTS.md's mutation-timing lore stays valid). Golden data moves
to `tests/paradigm/data/<gaṇa>.rs`; trace witnesses move to
`tests/trace/<gaṇa>.rs`. Concatenation via `LazyLock<Vec<Row>>` statics
keeps the `PARADIGM`/`ALTERNATES` names and almost every call site. The
partition is scripted, not hand-copied.

**Tech Stack:** Rust 1.98 via mise; Python 3 for the two one-shot partition
scripts (run from the repo root, kept in the scratchpad, never committed).

**Spec:** `docs/superpowers/specs/2026-08-28-test-suite-split-design.md`

## Global Constraints

- Work on branch `test-suite-split` in `/workspace` (already created; the
  spec is its first commit).
- Toolchain via mise only. To scope cargo, use `mise exec -- cargo <...>`;
  `mise run test -- -p X` does NOT scope (known repo gotcha).
- Run every test command in the FOREGROUND with an explicit generous
  timeout (600000 ms for full-suite runs). Never background a suite run.
- No engine crate is touched (nothing under `crates/panini-prakriya/src`,
  `crates/panini-data`, `crates/panini-lipi`, `crates/panini/src`), except
  the enumerated comment-path edits in Task 5. No golden string changes
  anywhere, in any task.
- No mutation campaign in this slice (spec, "Verification"). The merge gate
  is the structural proof in Task 4.
- `git add` only the exact paths each step names — never `git add -A`.
- Proof artifacts live in
  `/tmp/claude-1000/-workspace/bb9efc37-8a37-48e9-91a0-55de82345e6c/scratchpad/split-proof/`
  (referred to as `$PROOF` below). They persist across tasks in this
  session; do not delete them.
- Expected shape figures (pinned by existing tests): `PARADIGM` 316 blocks
  (2844 cells), `ALTERNATES` 494 rows, paradigm.rs 12 `#[test]` fns,
  trace.rs 102 `#[test]` fns, roundtrip.rs 1.

---

### Task 1: Baseline proof captures

**Files:**
- Modify: `crates/panini/tests/paradigm.rs` (append one temporary test at
  end of file)
- Create: `$PROOF/dump-pre.txt`, `$PROOF/inventory-pre.txt` (not committed)

**Interfaces:**
- Consumes: the pre-split monolith.
- Produces: `dump_goldens` test (name used verbatim by Tasks 2 and 4);
  `$PROOF/dump-pre.txt` and `$PROOF/inventory-pre.txt` (diffed in Task 4).

- [ ] **Step 1: Create the proof directory**

```bash
PROOF=/tmp/claude-1000/-workspace/bb9efc37-8a37-48e9-91a0-55de82345e6c/scratchpad/split-proof
mkdir -p "$PROOF"
```

- [ ] **Step 2: Append the dump test to `crates/panini/tests/paradigm.rs`**

Append at the very end of the file (after the last `}`):

```rust

/// TEMPORARY split-proof harness — removed at the end of the test-split
/// slice. Prints every golden row in a canonical sorted line form so the
/// pre-split and post-split tables can be diffed byte-for-byte.
#[test]
fn dump_goldens() {
    let mut rows: Vec<String> = Vec::new();
    for (root, lakara, pada, forms) in PARADIGM.iter() {
        rows.push(format!("P|{root}|{lakara}|{pada:?}|{}", forms.join(",")));
    }
    for (root, lakara, pada, cell, form, key) in ALTERNATES.iter() {
        rows.push(format!("A|{root}|{lakara}|{pada:?}|{cell}|{form}|{key}"));
    }
    rows.sort();
    for row in &rows {
        println!("{row}");
    }
}
```

(`.iter()` deliberately: it compiles against today's `&[Row]` consts AND
against Task 2's `LazyLock<Vec<Row>>` statics unchanged.)

- [ ] **Step 3: Capture the golden dump**

```bash
cd /workspace
mise exec -- cargo test -p panini --test paradigm dump_goldens -- --exact --nocapture 2>/dev/null \
  | grep -E '^[PA]\|' > "$PROOF/dump-pre.txt"
wc -l "$PROOF/dump-pre.txt"
```

Expected: exactly **810** lines (316 `P|` + 494 `A|`). Verify both halves:
`grep -c '^P|' "$PROOF/dump-pre.txt"` → 316, `grep -c '^A|'` → 494. If not,
STOP — do not proceed with wrong baselines.

- [ ] **Step 4: Capture the test inventory**

```bash
mise exec -- cargo test -p panini -- --list 2>/dev/null \
  | grep ': test$' | sed 's/: test$//; s/^.*:://' | sort > "$PROOF/inventory-pre.txt"
wc -l "$PROOF/inventory-pre.txt"
```

Record the count. Expected: 116 integration tests (12 paradigm + 1
`dump_goldens` + 102 trace + 1 roundtrip) plus whatever lib/doc tests the
`panini` crate lists — the exact total matters less than that Task 4
reproduces it identically.

- [ ] **Step 5: Format check and commit**

```bash
mise run fmt && mise run lint
git add crates/panini/tests/paradigm.rs
git commit -m "test(paradigm): add the temporary dump_goldens split-proof harness"
```

---

### Task 2: Split paradigm.rs into directory form

**Files:**
- Create: `crates/panini/tests/paradigm/main.rs`,
  `crates/panini/tests/paradigm/data/mod.rs`,
  `crates/panini/tests/paradigm/data/{bhvadi,adadi,divadi,svadi,tudadi,rudhadi,kryadi}.rs`
- Delete: `crates/panini/tests/paradigm.rs`
- Script (not committed): `$PROOF/split_paradigm.py`

**Interfaces:**
- Consumes: the monolith with Task 1's `dump_goldens` appended (13 `#[test]`
  fns total).
- Produces: `data::PARADIGM` / `data::ALTERNATES` as
  `pub static ...: LazyLock<Vec<ParadigmRow|AlternateRow>>`; per-gaṇa
  `pub const PARADIGM: &[ParadigmRow]` / `pub const ALTERNATES: &[AlternateRow]`
  in each `data/<gaṇa>.rs`; row type aliases `ParadigmRow` =
  `(&'static str, &'static str, Pada, [&'static str; 9])` and `AlternateRow`
  = `(&'static str, &'static str, Pada, usize, &'static str, &'static str)`.
  Task 4 runs `dump_goldens` against these.

- [ ] **Step 1: Write the partition script**

Write exactly this to `$PROOF/split_paradigm.py`:

```python
#!/usr/bin/env python3
"""One-shot partition of crates/panini/tests/paradigm.rs into directory
form. Run once from the repo root. Every assert is a stop-the-line check:
if one fires, report and stop — do not patch around it."""
import os
import re
import sys

SRC = "crates/panini/tests/paradigm.rs"
DST = "crates/panini/tests/paradigm"
GANA = {"01": "bhvadi", "02": "adadi", "04": "divadi", "05": "svadi",
        "06": "tudadi", "07": "rudhadi", "09": "kryadi"}
ORDER = ["bhvadi", "adadi", "divadi", "svadi", "tudadi", "rudhadi", "kryadi"]

lines = open(SRC).read().splitlines(keepends=True)


def find(pred, start=0):
    for i in range(start, len(lines)):
        if pred(lines[i]):
            return i
    sys.exit(f"anchor not found from line {start + 1}")


p_doc = find(lambda l: l.startswith("/// (root_number, lakara_label,"))
p_const = find(lambda l: l.startswith("const PARADIGM:"))
p_end = find(lambda l: l.rstrip("\n") == "];", p_const)
a_doc = find(lambda l: l.startswith("/// Second and third valid forms"), p_end)
a_const = find(lambda l: l.startswith("const ALTERNATES:"), a_doc)
a_end = find(lambda l: l.rstrip("\n") == "];", a_const)
assert p_doc < p_const < p_end < a_doc < a_const < a_end


def blocks(lo, hi):
    """Top-level `(...),` entries between the const header at lo and the
    `];` at hi, keyed by the gana of the dhatupatha number inside."""
    out, i = [], lo + 1
    while i < hi:
        assert lines[i].startswith("    ("), \
            f"line {i + 1} is not a block start: {lines[i]!r}"
        depth, j, blk = 0, i, []
        while True:
            blk.append(lines[j])
            depth += lines[j].count("(") - lines[j].count(")")
            j += 1
            if depth == 0:
                break
        m = re.search(r'"(\d\d)\.\d{4}"', "".join(blk))
        out.append((GANA[m.group(1)], blk))
        i = j
    return out


p_blocks = blocks(p_const, p_end)
a_blocks = blocks(a_const, a_end)
assert len(p_blocks) == 316, len(p_blocks)
assert len(a_blocks) == 494, len(a_blocks)


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w").write(text)


for g in ORDER:
    pg = [b for gg, b in p_blocks if gg == g]
    ag = [b for gg, b in a_blocks if gg == g]
    assert pg and ag, f"{g} has no rows"
    body = (
        f"//! {g}'s golden rows. See `super` (`data/mod.rs`) for the row\n"
        f"//! contracts and the concatenated `PARADIGM` / `ALTERNATES`"
        f" statics.\n\n"
        "use panini_data::Pada;\n\n"
        "use super::{AlternateRow, ParadigmRow};\n\n"
        "pub const PARADIGM: &[ParadigmRow] = &[\n"
        + "".join("".join(b) for b in pg)
        + "];\n\n"
        "pub const ALTERNATES: &[AlternateRow] = &[\n"
        + "".join("".join(b) for b in ag)
        + "];\n"
    )
    write(f"{DST}/data/{g}.rs", body)
    print(g, "paradigm blocks:", len(pg), "alternates rows:", len(ag))

p_doc_text = "".join(lines[p_doc:p_const])
a_doc_text = "".join(lines[a_doc:a_const])
mods = "".join(f"pub mod {g};\n" for g in sorted(ORDER))
concat_p = ", ".join(f"{g}::PARADIGM" for g in ORDER)
concat_a = ", ".join(f"{g}::ALTERNATES" for g in ORDER)
mod_rs = (
    "//! The golden tables, one file per gaṇa, keyed by the\n"
    "//! dhātupāṭha-number prefix of every row: 01 bhvādi, 02 adādi, 04\n"
    "//! divādi, 05 svādi, 06 tudādi, 07 rudhādi, 09 kryādi. Row order\n"
    "//! within a file preserves the pre-split monolith's order; the\n"
    "//! concatenated statics below are what the tests in `main.rs`\n"
    "//! consume, and no test depends on row order. A new gaṇa lands as\n"
    "//! one new file here plus its lines in the `mod` list and both\n"
    "//! `concat` arrays.\n\n"
    "use std::sync::LazyLock;\n\n"
    "use panini_data::Pada;\n\n"
    + mods + "\n"
    "pub type ParadigmRow = (&'static str, &'static str, Pada,"
    " [&'static str; 9]);\n"
    "pub type AlternateRow = (&'static str, &'static str, Pada, usize,"
    " &'static str, &'static str);\n\n"
    + p_doc_text
    + "pub static PARADIGM: LazyLock<Vec<ParadigmRow>> =\n"
    + f"    LazyLock::new(|| [{concat_p}].concat());\n\n"
    + a_doc_text
    + "pub static ALTERNATES: LazyLock<Vec<AlternateRow>> =\n"
    + f"    LazyLock::new(|| [{concat_a}].concat());\n"
)
write(f"{DST}/data/mod.rs", mod_rs)

header = "".join(lines[:p_doc])
assert header.startswith("mod common;\n")
header = header.replace(
    "mod common;\n",
    '#[path = "../common/mod.rs"]\nmod common;\nmod data;\n',
    1,
)
header = header.replace(
    "use common::{CELLS, LAKARA_BY_NAME};\n",
    "use common::{CELLS, LAKARA_BY_NAME};\n"
    "use data::{ALTERNATES, PARADIGM};\n",
    1,
)
tail = "".join(lines[a_end + 1:]).lstrip("\n")
tail = tail.replace("in PARADIGM {", "in PARADIGM.iter() {")
tail = tail.replace("in ALTERNATES {", "in ALTERNATES.iter() {")
n_tests = tail.count("#[test]")
assert n_tests == 13, n_tests  # 12 originals + dump_goldens
write(f"{DST}/main.rs", header + tail)
os.remove(SRC)
print("main.rs tests:", n_tests)
```

What the two `replace` calls in the tail do: exactly 7 loop headers change
— `in PARADIGM {` at 4 sites (`every_form_validates_and_matches`,
`derivation_set_is_exactly_pinned`,
`derivation_set_shape_matches_the_audited_numbers`,
`pada_ambiguous_surfaces_are_exactly_these`) and `in ALTERNATES {` at 3
sites (`every_alternate_validates_and_matches`,
`every_alternate_names_a_real_cell`,
`every_alternate_names_the_vikalpa_rules_that_produced_it`). All other
consumers (`PARADIGM.len()`, `PARADIGM.iter()`, `ALTERNATES.len()`,
`ALTERNATES.iter()`) work unchanged through `LazyLock`'s deref. Iterating
`&Vec<Row>` yields `&Row` exactly as iterating `&[Row]` did, so the
destructuring patterns and binding modes are untouched.

- [ ] **Step 2: Run the script**

```bash
cd /workspace && python3 "$PROOF/split_paradigm.py"
```

Expected: seven `<gaṇa> paradigm blocks: N alternates rows: M` lines (the
Ns summing to 316, Ms to 494; rudhadi much the largest) and
`main.rs tests: 13`. Any assert failure: STOP, report, do not improvise.

- [ ] **Step 3: Verify the tree shape**

```bash
ls crates/panini/tests/paradigm/data/
test ! -f crates/panini/tests/paradigm.rs && echo monolith-gone
```

Expected: the seven gaṇa files plus `mod.rs`; `monolith-gone`.

- [ ] **Step 4: Format, lint, and run the paradigm binary**

```bash
mise run fmt
mise exec -- cargo test -p panini --test paradigm
mise run lint
```

Expected: all 13 tests pass. If clippy flags an unused import in
`main.rs`, remove exactly that import line and nothing else; any other
lint complaint: STOP and report.

- [ ] **Step 5: Commit**

```bash
git add crates/panini/tests/paradigm.rs crates/panini/tests/paradigm/
git commit -m "test(paradigm): split the golden tables into per-gana data modules"
```

(The first path stages the deletion; the second the new directory.)

---

### Task 3: Split trace.rs into directory form

**Files:**
- Create: `crates/panini/tests/trace/main.rs`,
  `crates/panini/tests/trace/helpers.rs`,
  `crates/panini/tests/trace/{bhvadi,adadi,divadi,svadi,tudadi,rudhadi,kryadi}.rs`
- Delete: `crates/panini/tests/trace.rs`
- Script (not committed): `$PROOF/split_trace.py`

**Interfaces:**
- Consumes: pre-split `trace.rs` (3 helper fns + 102 `#[test]` fns).
- Produces: `helpers.rs` exporting `pub fn trace_for(word: &str) ->
  Vec<String>`, `pub fn cell_trace(number: &str, lakara: Lakara, pada:
  Pada, purusha: Purusha, vacana: Vacana) -> (String, Vec<String>)`,
  `pub fn at(trace: &[String], sutra: &str) -> usize`; seven gaṇa modules
  of unchanged tests.

- [ ] **Step 1: Write the partition script**

Write exactly this to `$PROOF/split_trace.py`. The `ASSIGN` table maps
every test to the gaṇa of the root it derives (verified against each
test's `trace_for` word or `cell_trace` dhātupāṭha number during
planning); the script cross-checks it is total and exact.

```python
#!/usr/bin/env python3
"""One-shot partition of crates/panini/tests/trace.rs into directory form.
Run once from the repo root. Every assert is a stop-the-line check."""
import os
import re

SRC = "crates/panini/tests/trace.rs"
DST = "crates/panini/tests/trace"
ORDER = ["bhvadi", "adadi", "divadi", "svadi", "tudadi", "rudhadi", "kryadi"]
HELPERS = {"trace_for", "cell_trace", "at"}

ASSIGN = {}
for name in [
    # √bhū, √labh, √vṛt, √edh — all bhvādi (01)
    "bhavati_trace_is_exactly_the_base_path",
    "bhavanti_trace_is_exactly_the_ji_coalescence_path",
    "bhavamah_trace_is_exactly_the_dirgha_visarga_path",
    "abhavat_trace_is_exactly_the_lan_augment_path",
    "abhavan_trace_is_exactly_the_samyoganta_path",
    "abhavam_trace_shows_dirgha_does_not_fire",
    "bhavatu_trace_is_exactly_the_lot_er_uh_path",
    "bhava_trace_shows_hi_elision",
    "bhavani_trace_shows_aat_not_dirgha",
    "bhavet_trace_is_exactly_the_vidhilin_vali_lopa_path",
    "bhaveyuh_trace_is_exactly_the_jus_path",
    "bhaveyam_trace_is_exactly_the_widened_mip_path",
    "labhate_trace_is_exactly_the_minimal_atmanepada_path",
    "labhete_trace_is_exactly_the_ato_nitah_path",
    "vartate_trace_shows_laghupadha_guna",
    "labhasva_trace_is_exactly_the_savabhyam_path",
    "labhai_trace_is_exactly_the_at_vrddhi_path",
    "aidhata_trace_is_exactly_the_at_agama_path",
    "labheran_trace_is_exactly_the_siyut_ran_path",
    "labheya_trace_is_exactly_the_ito_t_path",
    "bhavatu_forks_twice_into_three_branches",
]:
    ASSIGN[name] = "bhvadi"
for name in [
    # √yā, √ad, √ās, √vas, √śī — adādi (02)
    "yati_trace_is_the_bare_luk_path",
    "yanti_trace_is_the_luk_plus_savarna_path",
    "yayuh_trace_is_the_adadi_us_junction_path",
    "yayam_trace_is_the_adadi_am_junction_path",
    "ayuh_trace_is_the_shakatayana_jus_path",
    "atti_trace_ends_in_cartva",
    "addhi_trace_uses_her_dhih_after_ato_heh_declines",
    "adat_trace_a_augment_precedes_and_blocks_cartva",
    "asate_trace_uses_anatah_not_jhontah",
    "adhve_trace_ends_in_dhi_ca",
    "vadhve_trace_is_the_second_dhi_ca_witness",
    "ase_lot_uttama_eka_trace_ends_in_atas_ca",
    "asita_vidhilin_trace_ends_in_vali_lopa",
    "shete_trace_is_the_minimal_shing_guna_path",
    "sherate_trace_is_the_rut_path",
    "sheshe_trace_ends_in_shatva",
    "shayita_trace_is_the_shing_vali_lopa_path",
    "shayai_trace_is_the_shing_atas_ca_path",
]:
    ASSIGN[name] = "adadi"
for name in [
    # √div, √man, √kup — divādi (04)
    "divyati_trace_is_exactly_the_syan_block_lengthen_path",
    "manyate_trace_is_exactly_the_syan_atmanepada_path",
    "kupyet_trace_is_exactly_the_syan_vidhilin_path",
    "akupyat_trace_shows_7_3_100_declines_for_non_adadi_roots",
]:
    ASSIGN[name] = "divadi"
for name in [
    # √āp, √hi, √ri, √aś (05.0020), √stigh — svādi (05)
    "apnoti_trace_pins_the_vikarana_guna",
    "apnavani_trace_pins_the_guna_before_6_1_78_order",
    "apnuvanti_trace_is_the_conjunct_uvang_path",
    "apnuhi_trace_is_the_conjunct_hi_luk_block",
    "apnutat_trace_shows_tatan_blocking_the_vikarana_guna",
    "hinvanti_trace_is_the_asamyogapurva_yan_path",
    "hinu_trace_pins_the_hi_luk",
    "hinvah_trace_is_hinuvah_plus_exactly_the_optional_step",
    "ahinma_trace_shows_the_optional_lopa_after_the_augment",
    "rinoti_trace_ends_in_natva_over_the_new_stem",
    "ashnuvate_trace_is_7_1_5_then_6_4_77",
    "ashnushva_trace_reaches_8_3_59_through_the_vikarana",
    "ashnuvita_trace_is_7_2_79_then_6_4_77_then_6_1_66",
    "ashnave_trace_pins_the_widened_6_1_90_athematic_arm",
    "stighnute_trace_has_no_6_1_64_substitution",
]:
    ASSIGN[name] = "svadi"
for name in [
    # √tud, √juṣ — tudādi (06)
    "tudati_trace_is_the_sa_block_path",
    "jusate_trace_is_the_sa_atmanepada_block_path",
]:
    ASSIGN[name] = "tudadi"
for name in [
    # rudhādi (07): √kṛt, √hiṃs, √piṣ, √bhañj, √indh, √rudh, √kṣud,
    # √tṛd, √ric, √vic, √und, √añj, √tṛh, √chid, √chṛd, √bhuj
    "krnatti_trace_shows_the_infix_then_natva",
    "krntat_trace_shows_savarna_elision_above_pausal",
    "kndhi_trace_shows_jashtva_where_dhi_ca_declines",
    "hindi_trace_shows_dhi_ca_bleeding_jashtva",
    "ahinah_trace_shows_ru_fires_on_the_dhatus_own_final",
    "apinaq_trace_pins_8_2_23_above_8_2_41",
    "pinakshi_trace_shows_8_2_41_then_8_3_59",
    "pimzwah_trace_is_the_round_trips_second_witness",
    "pindhi_trace_is_the_full_8_4_41_chain",
    "bhanakti_trace_shows_8_2_30_then_8_4_55",
    "indhe_trace_shows_8_2_40_then_8_4_53",
    "rudh_lat_prathama_eka_credits_both_pada_sutras",
    "rudh_natva_follows_stem_strength_not_pada",
    "runde_is_ambiguous_within_atmanepada",
    "kshud_natva_is_the_intervening_arm_under_a_sibilant_trigger",
    "trd_natva_is_the_adjacent_arm_through_an_r_vowel_trigger",
    "rinakti_trace_reaches_k_in_one_step",
    "rinakti_trace_takes_intervening_natva",
    "vinakti_trace_takes_no_natva_at_all",
    "unantas_trace_orders_6_4_23_before_6_4_111",
    "aunat_trace_takes_the_u_vrddhi_arm",
    "anaktas_trace_is_the_kutva_path_on_a_vowel_initial_root",
    "trneddhi_trace_puts_8_3_13_below_8_4_41",
    "trnaddhi_trace_has_8_3_13_and_no_8_4_65",
    "atrned_trace_takes_the_im_before_8_2_23_eats_tips_t",
    "acchinat_trace_orders_the_tuk_between_the_augment_and_shcutva",
    "acchrnat_trace_runs_natva_and_shcutva_on_disjoint_sites",
    "chinatti_trace_cites_neither_new_sutra",
    "acchinat_has_exactly_two_forms",
    "bhunkte_trace_credits_1_3_66_not_1_3_72",
    "bhunakti_trace_credits_the_shesa_1_3_78",
]:
    ASSIGN[name] = "rudhadi"
for name in [
    # √kliś, √aś (09.0059), √muṣ, √vṝ — kryādi (09)
    "klishnati_trace_is_the_shna_pit_path",
    "klishnitah_trace_takes_i_halyaghoh",
    "klishnanti_trace_takes_shnabhyastayor_atah",
    "klishana_trace_is_shanac_then_ato_heh",
    "klishnitat_trace_pins_tatan_above_3_1_83",
    "ashnat_trace_takes_the_vowel_initial_anga_augment",
    "mushnati_trace_takes_adjacent_natva",
    "mushana_trace_is_shanac_plus_intervening_natva",
    "vrinati_trace_takes_intervening_natva",
    "vrinite_trace_is_the_atmanepada_shna_path",
    "vrinishva_trace_reaches_the_existing_shatva",
]:
    ASSIGN[name] = "kryadi"

assert len(ASSIGN) == 102, len(ASSIGN)

lines = open(SRC).read().splitlines(keepends=True)
first_item = next(
    i for i, l in enumerate(lines) if l.startswith("fn trace_for")
)
doc = [l for l in lines[:first_item] if l.startswith("//!")]
assert doc, "module doc not found"

items, i, n = [], first_item, len(lines)
while i < n:
    if lines[i].strip() == "":
        i += 1
        continue
    j = i
    while lines[j].rstrip("\n") != "}":
        j += 1
    chunk = "".join(lines[i:j + 1])
    m = re.search(r"^fn (\w+)\(", chunk, re.M)
    items.append((m.group(1), chunk))
    i = j + 1

names = [nm for nm, _ in items]
assert len(items) == 105, len(items)
assert set(names) == set(ASSIGN) | HELPERS, \
    set(names) ^ (set(ASSIGN) | HELPERS)

mods, helpers_src = {}, []
for nm, chunk in items:
    if nm in HELPERS:
        helpers_src.append(chunk)
    else:
        mods.setdefault(ASSIGN[nm], []).append(chunk)


def imports_for(body):
    out = []
    h = sorted(x for x in ("at", "cell_trace", "trace_for")
               if re.search(rf"\b{x}\b", body))
    if h:
        out.append("use crate::helpers::{" + ", ".join(h) + "};\n")
    if re.search(r"\bPanini\b", body):
        out.append("use panini::Panini;\n")
    pd = sorted(x for x in ("Lakara", "Pada", "Purusha", "Vacana", "dhatus")
                if re.search(rf"\b{x}\b", body))
    if pd:
        out.append("use panini_data::{" + ", ".join(pd) + "};\n")
    if re.search(r"\bderive\b", body):
        out.append("use panini_prakriya::derive;\n")
    return "".join(out)


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w").write(text)


for g, bodies in mods.items():
    write(
        f"{DST}/{g}.rs",
        f"//! {g}'s ordered-trace witnesses. Helpers live in\n"
        f"//! `crate::helpers`; the module doc governing this suite is in\n"
        f"//! `main.rs`.\n\n"
        + imports_for("".join(bodies))
        + "\n"
        + "\n".join(bodies),
    )
    print(g, "tests:", len(bodies))

helpers_rs = (
    "//! Shared helpers for the ordered-trace witnesses in the sibling\n"
    "//! modules.\n\n"
    "use panini::Panini;\n"
    "use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};\n"
    "use panini_prakriya::derive;\n\n"
    + "\n".join(helpers_src)
)
for h in sorted(HELPERS):
    assert f"fn {h}(" in helpers_rs
    helpers_rs = helpers_rs.replace(f"fn {h}(", f"pub fn {h}(", 1)
write(f"{DST}/helpers.rs", helpers_rs)

doctext = "".join(doc)
old_grep = ("//! tests are added. Run `grep -c '^#\\[test\\]'"
            " crates/panini/tests/trace.rs`\n")
assert old_grep in doctext
doctext = doctext.replace(
    old_grep,
    "//! tests are added. Run\n"
    "//! `cat crates/panini/tests/trace/*.rs | grep -c '^#\\[test\\]'`\n",
    1,
)
assert "//! function below. Don't hardcode" in doctext
doctext = doctext.replace(
    "//! function below. Don't hardcode",
    "//! function in the gaṇa modules. Don't hardcode",
    1,
)
assert "so the sequences below differ" in doctext
doctext = doctext.replace(
    "so the sequences below differ",
    "so the pinned sequences differ",
    1,
)
present = sorted(g for g in ORDER if g in mods)
write(
    f"{DST}/main.rs",
    doctext + "\nmod helpers;\n\n"
    + "".join(f"mod {g};\n" for g in present),
)
os.remove(SRC)
```

- [ ] **Step 2: Run the script**

```bash
cd /workspace && python3 "$PROOF/split_trace.py"
```

Expected: seven `<gaṇa> tests: N` lines summing to 102 (bhvadi 21, adadi
18, divadi 4, svadi 15, tudadi 2, rudhadi 31, kryadi 11). Any assert
failure: STOP and report — in particular, a set-difference printout from
the `ASSIGN` check means the table and the file disagree, and the fix is a
diagnosis, not a guess.

- [ ] **Step 3: Format, lint, and run the trace binary**

```bash
mise run fmt
mise exec -- cargo test -p panini --test trace
mise run lint
```

Expected: 102 tests pass. The import scanner errs toward inclusion (an
identifier in a comment can pull in an import the code doesn't use); if
clippy flags an unused import in a gaṇa file, delete exactly that import
and nothing else. A MISSING identifier error means the scanner logic was
edited — STOP and report rather than hand-patch.

- [ ] **Step 4: Commit**

```bash
git add crates/panini/tests/trace.rs crates/panini/tests/trace/
git commit -m "test(trace): split the ordered-trace witnesses by gana"
```

---

### Task 4: The structural proof, and dump removal

**Files:**
- Modify: `crates/panini/tests/paradigm/main.rs` (remove `dump_goldens`)
- Create: `$PROOF/dump-post.txt`, `$PROOF/inventory-post.txt` (not
  committed)

**Interfaces:**
- Consumes: Task 1's `$PROOF/dump-pre.txt` and `inventory-pre.txt`; Tasks
  2–3's directory-form suite.
- Produces: the merge gate's evidence — record both diff results (empty)
  and the full-suite pass in the task report; Task 6 quotes them in the PR
  body.

- [ ] **Step 1: Capture the post-split dump and diff**

```bash
PROOF=/tmp/claude-1000/-workspace/bb9efc37-8a37-48e9-91a0-55de82345e6c/scratchpad/split-proof
cd /workspace
mise exec -- cargo test -p panini --test paradigm dump_goldens -- --exact --nocapture 2>/dev/null \
  | grep -E '^[PA]\|' > "$PROOF/dump-post.txt"
diff "$PROOF/dump-pre.txt" "$PROOF/dump-post.txt" && echo GOLDENS-IDENTICAL
```

Expected: `GOLDENS-IDENTICAL` (diff exit 0, no output). Any difference is
a lost/duplicated/edited row: STOP, report the differing lines, do not
touch golden data to make the diff pass.

- [ ] **Step 2: Capture the post-split inventory and diff**

```bash
mise exec -- cargo test -p panini -- --list 2>/dev/null \
  | grep ': test$' | sed 's/: test$//; s/^.*:://' | sort > "$PROOF/inventory-post.txt"
diff "$PROOF/inventory-pre.txt" "$PROOF/inventory-post.txt" && echo INVENTORY-IDENTICAL
```

Expected: `INVENTORY-IDENTICAL`. (Module prefixes are stripped by the
`sed`, so the trace tests' new `bhvadi::` etc. prefixes don't appear;
`dump_goldens` is present on both sides.)

- [ ] **Step 3: Remove the dump harness**

Delete the entire `dump_goldens` block from
`crates/panini/tests/paradigm/main.rs` — the `/// TEMPORARY split-proof
harness` doc comment through the test's closing `}` (the exact block Task
1 appended).

- [ ] **Step 4: Full-suite green on the final state**

```bash
mise run fmt-check && mise run lint
mise run test
```

Run `mise run test` in the foreground with timeout 600000 ms; the suite
alone is ~450s uncontended. If the harness cap kills it anyway, run the
pieces separately and report all four:
`mise exec -- cargo test -p panini --test paradigm` (~207s),
`--test roundtrip` (~240s), `--test trace`, and
`mise exec -- cargo test -p panini --lib`, plus
`mise exec -- cargo test -p panini-prakriya -p panini-data -p panini-lipi`.
Expected: everything passes (now 115 integration tests — the 116 minus
`dump_goldens`).

- [ ] **Step 5: Commit**

```bash
git add crates/panini/tests/paradigm/main.rs
git commit -m "test: certify the split -- byte-identical goldens, identical inventory"
```

The commit message body should record: 810 dump rows byte-identical
(316 + 494), inventory diff empty, full suite green, no mutation campaign
per the spec's structural-proof gate.

---

### Task 5: Doc sweep

**Files:**
- Modify: `README.md`, `docs/ARCHITECTURE.md`, `tools/audit/README.md`,
  `AGENTS.md`, `crates/panini/tests/common/mod.rs`,
  `crates/panini/tests/paradigm/main.rs`,
  `crates/panini-prakriya/src/tinanta/{anga.rs,tripadi.rs,derivation_tests.rs}`

**Interfaces:**
- Consumes: the directory-form tree from Tasks 2–4.
- Produces: no current-state reference to `tests/paradigm.rs` or
  `tests/trace.rs` outside `docs/superpowers/` (which is historical and
  must NOT be rewritten).

- [ ] **Step 1: Apply the path edits**

Each row is one Edit: find the old fragment (unique in its file), replace
with the new. Line numbers are the pre-sweep locations, for orientation
only — match on text.

| File:line | Old fragment | New fragment |
|---|---|---|
| `README.md:77` | `` `crates/panini/tests/paradigm.rs` walks `PARADIGM` `` | `` `crates/panini/tests/paradigm/main.rs` walks `PARADIGM` `` |
| `docs/ARCHITECTURE.md:56` | `` `crates/panini/tests/trace.rs` are what pin them. `` | `` `crates/panini/tests/trace/` are what pin them. `` |
| `docs/ARCHITECTURE.md:252` | `` `crates/panini/tests/paradigm.rs::paradigm_covers_every_enumerable_cell` `` | `` `crates/panini/tests/paradigm/main.rs::paradigm_covers_every_enumerable_cell` `` |
| `tools/audit/README.md:33` | `` `crates/panini/tests/paradigm.rs`, which each slice raises `` | `` `crates/panini/tests/paradigm/main.rs`, which each slice raises `` |
| `tools/audit/README.md:126` | `` `crates/panini/tests/paradigm.rs`'s golden table, generated `` | `` `crates/panini/tests/paradigm/data/`'s golden tables, generated `` |
| `AGENTS.md:551` | `` (`crates/panini/tests/paradigm.rs`, 2844 cells `` | `` (`crates/panini/tests/paradigm/`, 2844 cells `` |
| `AGENTS.md:763` | `` and pins the rejection in `trace.rs`.) `` | `` and pins the rejection in `tests/trace/`.) `` |
| `AGENTS.md:852` | `` `crates/panini/tests/trace.rs` is the sole pin `` | `` `crates/panini/tests/trace/rudhadi.rs` is the sole pin `` |
| `AGENTS.md:854` | `` ordered-trace test (`crates/panini/tests/trace.rs`), which pins `` | `` ordered-trace test (`crates/panini/tests/trace/`), which pins `` |
| `AGENTS.md:937-938` | `` pinned at\n  `paradigm.rs:5934`) `` | `` pinned in\n  `derivation_set_shape_matches_the_audited_numbers`,\n  `crates/panini/tests/paradigm/main.rs`) `` |
| `crates/panini/tests/common/mod.rs:2` | `` test file (`paradigm.rs`, `roundtrip.rs`, ...) compiles `` | `` test binary (`paradigm/`, `trace/`, `roundtrip.rs`, ...) compiles `` |
| `crates/panini/tests/paradigm/main.rs` (was `paradigm.rs:6232`) | `` `crates/panini/tests/trace.rs`, which asserts `7.4.21` `` | `` `crates/panini/tests/trace/adadi.rs`, which asserts `7.4.21` `` |
| `crates/panini-prakriya/src/tinanta/anga.rs:126` | `` `crates/panini/tests/trace.rs`: the mutant fires `` | `` `crates/panini/tests/trace/`: the mutant fires `` |
| `crates/panini-prakriya/src/tinanta/tripadi.rs:248` | `` in `crates/panini/tests/trace.rs` `` | `` in `crates/panini/tests/trace/rudhadi.rs` `` |
| `crates/panini-prakriya/src/tinanta/tripadi.rs:360` | `` (`crates/panini/tests/paradigm.rs`, `` | `` (`crates/panini/tests/paradigm/data/rudhadi.rs`, `` |
| `crates/panini-prakriya/src/tinanta/tripadi.rs:651` | `` `crates/panini/tests/paradigm.rs` pins `` | `` `crates/panini/tests/paradigm/` pins `` |
| `crates/panini-prakriya/src/tinanta/tripadi.rs:1510` | `` `crates/panini/tests/trace.rs` asserts directly `` | `` `crates/panini/tests/trace/` asserts directly `` |
| `crates/panini-prakriya/src/tinanta/tripadi.rs:1879` | `` paradigm.rs), but `` | `` tests/paradigm/), but `` |
| `crates/panini-prakriya/src/tinanta/tripadi.rs:2021` | `` pinned in `paradigm.rs`) `` | `` pinned in `tests/paradigm/`) `` |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs:97` | `` in `crates/panini/tests/trace.rs` (which only pin `` | `` in `crates/panini/tests/trace/` (which only pin `` |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs:312` | `` pinned in `paradigm.rs`'s ALTERNATES) `` | `` pinned in `tests/paradigm/`'s ALTERNATES) `` |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs:1012` | `` `crates/panini/tests/trace.rs`. `` | `` `crates/panini/tests/trace/`. `` |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs:1126` | `` crates/panini/tests/paradigm.rs) `` | `` crates/panini/tests/paradigm/) `` |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs:1997` | `` pinned again in trace.rs. `` | `` pinned again in tests/trace/. `` |

These are this slice's **permitted engine-comment edits**: path-only swaps
in comments, nothing else. The two deliberately stale corpus-figure
comments AGENTS.md tracks (`controller.rs:130`, `tinanta/guna.rs:943`)
stay untouched. The AGENTS.md:852 and tripadi.rs:248 rows may name
`trace/rudhadi.rs` because `krntat_trace_shows_savarna_elision_above_pausal`
and `rinakti_trace_reaches_k_in_one_step` both land there (√kṛt and √ric
are rudhādi); if Task 3 moved them elsewhere, follow the actual location.

- [ ] **Step 2: Add the AGENTS.md mutation-section note**

In AGENTS.md's mutation bullet, immediately after the sentence ending
`and re-measure both the floor and the margin the next time the golden
suite grows.` insert:

```
    The 2026-08-28 test-split slice restructured both golden files into
    directory form (`tests/paradigm/`, `tests/trace/`) with the
    timing-relevant shape — the same two integration binaries — unchanged,
    so no timing figure above was invalidated by it.
```

(Match the surrounding indentation of that bullet's continuation lines.)

- [ ] **Step 3: Verify the sweep is complete**

```bash
grep -rn "tests/paradigm\.rs\|tests/trace\.rs" \
  README.md AGENTS.md docs/ARCHITECTURE.md tools/ crates/ \
  --include='*.rs' --include='*.md'
grep -rn '`paradigm\.rs`\|`trace\.rs`\|paradigm\.rs:' \
  README.md AGENTS.md docs/ARCHITECTURE.md tools/ crates/ \
  --include='*.rs' --include='*.md'
```

Expected: both empty. (`docs/superpowers/` is deliberately excluded —
specs and plans are historical.) If a hit appears that this table missed,
apply the same path-only treatment and note it in the commit message.

- [ ] **Step 4: Confirm the comment edits broke nothing**

```bash
mise exec -- cargo test -p panini-prakriya
mise run lint
```

Expected: pass (~2s suite; comments only, but prove it).

- [ ] **Step 5: Commit**

```bash
git add README.md docs/ARCHITECTURE.md tools/audit/README.md AGENTS.md \
  crates/panini/tests/common/mod.rs crates/panini/tests/paradigm/main.rs \
  crates/panini-prakriya/src/tinanta/anga.rs \
  crates/panini-prakriya/src/tinanta/tripadi.rs \
  crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "docs: re-point every current-state reference at the directory-form suite"
```

---

### Task 6: Finish the branch

**Files:** none.

**Interfaces:**
- Consumes: every prior task; Task 4's recorded proof results.
- Produces: the merge.

- [ ] **Step 1: Open the PR**

```bash
git push -u origin test-suite-split
gh pr create --fill
```

The PR body must state: the structural proof (810 golden rows — 316
`PARADIGM` blocks + 494 `ALTERNATES` rows — byte-identical pre/post via
the temporary `dump_goldens` harness; test inventory identical; full
suite green), that no mutation campaign ran and why (no engine change;
test content proven identical; the spec's structural-proof gate), and the
one-sentence version of the slice: the two golden test files split into
per-gaṇa directory-form modules, two test binaries before and after, so
the mutation-timing calibration survives.

Append `https://claude.ai/code/session_01FwouqTt88YaSPxLs1LoXRC` to the
body.

- [ ] **Step 2: Merge and clean up**

Use the `superpowers:finishing-a-development-branch` skill with the
standing instruction: wait for CI, merge the green PR, verify the commits
are on `main`, then delete the branch and any worktree.
