# Dhātupāṭha Number as Identity — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the dhātupāṭha entry number (`07.0001`) the unique identity of every curated root, retiring `Dhatu::id`, and check all 49 assignments against a vendored upstream dhātupāṭha so the cross-implementation audit no longer anchors on this engine's own output.

**Architecture:** Add `Dhatu.dhatupatha` and vendor vidyut-prakriya's `data/dhatupatha.tsv` (2260 rows, commit `8da2f90`) in place of our hand-synced 49-row mirror. A test-private it-stripping normalizer relates each upstream upadeśa to our stored `code`, which is the assertion that breaks the circularity — number and artha alone cannot, because upstream has 8- and 15-way artha collisions. Then re-key the 442 golden-table keys, delete `id`, and rewrite the docs that describe the old mechanism.

**Tech Stack:** Rust (workspace pinned to 1.97.1 via mise), `cargo test`, `include_str!` for the vendored TSV, Python 3 for the one mechanical re-key.

**Spec:** `docs/superpowers/specs/2026-08-15-dhatupatha-number-identity-design.md`

## Global Constraints

- **SLP1 is the only internal representation.** Transliteration lives only in `panini-lipi`. Every string in this plan is SLP1.
- **`#![forbid(unsafe_code)]`** in every non-fuzz crate. Nothing here needs unsafe.
- **No grammar changes.** `PARADIGM` stays at **1800 cells**, `ALTERNATES` at **242 rows**, **2042 forms**, **49 roots**, **seven** optional rules. **No surface form may change.** Any diff to a form string is a defect in this plan's execution, not a result.
- **Three `artha` strings change** and nothing else in shipped data. No rule reads `artha`.
- **The normalizer is test-private.** It must not become a `pub fn`, and it must not become a `Rule`. It never runs in a derivation. Implementing it-stripping as real sūtras (1.3.2, 1.3.3, 1.3.5, 1.3.9) is explicitly deferred by the spec.
- **Toolchain:** `mise run build | test | lint | fmt | fmt-check | audit`. Do not install Rust globally.
- **Every task ends green.** Tasks 1–6 each leave `mise run test` passing, so the branch is bisectable throughout.

---

## File Structure

| file | responsibility after this plan |
| --- | --- |
| `data/dhatupatha.tsv` | **Replaced.** Upstream's 2260-row dhātupāṭha, verbatim, with a provenance header. Read by exactly one test. |
| `crates/panini-data/src/lib.rs` | `Dhatu` gains `dhatupatha` and loses `id`; the test module gains the normalizer and two resolution tests, and loses the three tests that existed to police `id`'s uniqueness. |
| `crates/panini/tests/paradigm.rs` | `PARADIGM` and `ALTERNATES` keyed by number; six lookups resolve on `dhatupatha`. |
| `crates/panini/src/lib.rs` | `Analysis::dhatu`'s doc comment names `dhatupatha` as the unique key. |
| `AGENTS.md` | The audit paragraph describes number-keyed selection; the two `Dhatu::id` references go. |

---

### Task 1: Vendor upstream, add the field, and check it resolves

**Files:**
- Modify: `data/dhatupatha.tsv` (replaced wholesale)
- Modify: `crates/panini-data/src/lib.rs` (struct at `:65-92`, 49 rows at `:94-508`, test module)

**Interfaces:**
- Consumes: nothing.
- Produces: `Dhatu.dhatupatha: &'static str` — the entry number, e.g. `"07.0001"`. Every later task keys on this field. Also the test-private helpers `strip_anubandhas(&str) -> String`, `dhatvadeh_sha_sa(String) -> String`, `stored_form(&str) -> String`, and `upstream_rows() -> Vec<(&'static str, &'static str, &'static str)>` returning `(number, upadeśa, artha)`.

- [ ] **Step 1: Vendor the upstream file**

```bash
cd /tmp && rm -rf vidyut-vendor && \
git clone --depth 1 --filter=blob:none --sparse https://github.com/ambuda-org/vidyut vidyut-vendor && \
cd vidyut-vendor && git sparse-checkout set vidyut-prakriya/data && git rev-parse HEAD
```

Expected: prints `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`. **If it prints anything else, stop** — upstream has moved since the spec was written, and the 49 numbers in Step 3 were resolved against `8da2f90`. Check out that commit explicitly before continuing.

Now replace our mirror, prepending the provenance header:

```bash
cd /workspace && { cat <<'EOF'
# The Dhātupāṭha, vendored verbatim from vidyut-prakriya.
#
#   source:  https://github.com/ambuda-org/vidyut
#   path:    vidyut-prakriya/data/dhatupatha.tsv
#   commit:  8da2f90bee3ce1c07505fa432fc3729e3f7e02ea
#
# Licensed MIT. vidyut-prakriya's data/README.md records that these files were
# sourced from ashtadhyayi.com, whose author shared them under an MIT license.
# This repository is Apache-2.0; the MIT notice above is retained as required.
#
# Columns: entry number, upadeśa (with accent and anubandha notation), artha.
#
# All ten gaṇas are vendored, not just the seven this engine covers: filtering
# would be curation, and the point of this file is to be a source we did not
# curate. Read by `dhatupatha_numbers_resolve_upstream` in
# crates/panini-data/src/lib.rs, which is the only consumer.
#
# The pinned commit makes re-vendoring a deliberate act with a visible diff.
# Do not refresh it casually: the 49 numbers in DHATUS were resolved against
# this exact revision.
EOF
cat /tmp/vidyut-vendor/vidyut-prakriya/data/dhatupatha.tsv; } > data/dhatupatha.tsv
wc -l data/dhatupatha.tsv && head -22 data/dhatupatha.tsv | tail -3
```

Expected: 2280 lines (20 header + 2260 upstream), and the last three lines shown are the blank-free tail of the header plus `code	dhatu	artha` and `01.0001	BU	sattAyAm`.

- [ ] **Step 2: Write the failing tests**

Add to the bottom of the `mod tests` block in `crates/panini-data/src/lib.rs` (before its closing `}`):

```rust
    /// Upstream's dhātupāṭha, vendored at the commit named in its header.
    /// `include_str!` sits inside `#[cfg(test)]`, so the 54K reaches the test
    /// binary only and never the library.
    const UPSTREAM: &str = include_str!("../../../data/dhatupatha.tsv");

    /// `(number, upadeśa, artha)` for every upstream row.
    fn upstream_rows() -> Vec<(&'static str, &'static str, &'static str)> {
        UPSTREAM
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .filter_map(|l| {
                let mut f = l.split('\t');
                match (f.next(), f.next(), f.next()) {
                    // Skip upstream's own `code	dhatu	artha` header row.
                    (Some(n), Some(u), Some(a)) if n != "code" => Some((n, u, a)),
                    _ => None,
                }
            })
            .collect()
    }

    /// True for an SLP1 consonant (*hal*). SLP1's vowels are the fourteen
    /// listed here; `~`, being notation rather than a sound, is not a hal.
    fn is_hal(c: char) -> bool {
        c.is_alphabetic() && !"aAiIuUfFxXeEoO".contains(c)
    }

    /// Strips the anubandhas from an upstream upadeśa.
    ///
    /// **Not grammar the pipeline owes a `Rule`** — it never runs in a
    /// derivation. It exists so `dhatupatha_numbers_resolve_upstream` can
    /// relate an upstream row to our stored `code` without consulting
    /// anything this repo wrote, which is the assertion that makes the
    /// cross-implementation audit non-circular.
    fn strip_anubandhas(upadesha: &str) -> String {
        // Accent notation: anudātta `\`, svarita `^`. Marks, not sounds.
        let s: String = upadesha.chars().filter(|c| *c != '\\' && *c != '^').collect();

        // 1.3.3 halantyam is decided on the ORIGINAL upadeśa, before 1.3.2
        // deletes anything. Getting this order wrong corrupts silently rather
        // than failing loudly: `paWa~` ends in the vowel `a` (marked
        // anunāsika by the `~` after it), so its `W` is root-final and must
        // survive — deciding after the deletion would strip it to `pa`, and
        // would strip `tfha~` to `tf`, destroying a real root-final `h` while
        // still producing a plausible string. `ru\Di~^r` genuinely ends in
        // the consonant `r`, so that `r` IS an it.
        let ends_in_hal = s.chars().last().is_some_and(is_hal);

        // 1.3.2 upadeśe'j-anunāsika it, with 1.3.9 tasya lopaḥ. Upstream
        // marks an anunāsika it with a following `~`, so each `X~` pair goes.
        let mut t = String::new();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if chars.peek() == Some(&'~') {
                chars.next();
                continue;
            }
            t.push(c);
        }

        // 1.3.5 ādir ñiṭuḍavaḥ: an initial ñi / ṭu / ḍu is it.
        for prefix in ["Yi", "wu", "qu"] {
            if let Some(rest) = t.strip_prefix(prefix) {
                t = rest.to_string();
                break;
            }
        }

        // 1.3.3 halantyam, on the verdict reached above.
        if ends_in_hal && t.chars().count() > 1 {
            t.pop();
        }
        t
    }

    /// 6.1.64 dhātvādeḥ ṣaḥ saḥ / ṇaḥ naḥ. A root-initial ṣ or ṇ in the
    /// upadeśa is stored as s / n, because no rule in this engine performs
    /// the substitution. For `zwiGa~\` the retroflex immediately after goes
    /// with it (ṣṭ → st), which is exactly what `stiG` records.
    fn dhatvadeh_sha_sa(code: String) -> String {
        if let Some(rest) = code.strip_prefix('z') {
            let rest = rest
                .strip_prefix('w')
                .map_or_else(|| rest.to_string(), |r| format!("t{r}"));
            return format!("s{rest}");
        }
        if let Some(rest) = code.strip_prefix('R') {
            return format!("n{rest}");
        }
        code
    }

    /// The form this repo stores as `Dhatu::code`, derived from an upstream
    /// upadeśa.
    fn stored_form(upadesha: &str) -> String {
        let s = dhatvadeh_sha_sa(strip_anubandhas(upadesha));
        // 7.1.58 idito num dhātoḥ is not derivable here, so √hiṃs is stored
        // with the num already inserted. This is the single deviation between
        // an it-stripped upadeśa and a stored `code`, and it is the same one
        // the retired `Dhatu::id` doc comment recorded.
        if s == "his" { "hins".to_string() } else { s }
    }

    #[test]
    fn dhatupatha_numbers_resolve_upstream() {
        let rows = upstream_rows();
        let count = rows.len();
        assert!(count > 2000, "vendored dhātupāṭha looks truncated: {count} rows");
        let mut numbers: Vec<&str> = rows.iter().map(|(n, _, _)| *n).collect();
        numbers.sort_unstable();
        numbers.dedup();
        assert_eq!(
            numbers.len(),
            count,
            "upstream numbers must be unique for one to serve as our key"
        );

        for d in dhatus() {
            let (_, upadesha, _) = rows
                .iter()
                .find(|(n, _, _)| *n == d.dhatupatha)
                .unwrap_or_else(|| panic!("{} names no upstream row", d.dhatupatha));
            // THIS is the assertion that breaks the circularity. Matching on
            // number and artha alone would still pass if a number pointed at
            // a sibling entry sharing an artha, and upstream has 8- and
            // 15-way artha collisions (`vyaktAyAM vAci`, `vfdDO`). Relating
            // the upadeśa to the code is the only check that cannot be
            // satisfied by copying back the choice we made.
            let stripped = stored_form(upadesha);
            assert_eq!(
                stripped, d.code,
                "{} {upadesha} it-strips to {stripped}, but DHATUS stores {}",
                d.dhatupatha, d.code
            );
        }
    }

    #[test]
    fn gana_matches_dhatupatha_prefix() {
        // The number's prefix encodes the gaṇa, so `Dhatu::gana` is redundant
        // with it. The field stays (the rule pipeline reads the enum
        // pervasively, and deriving it would mean parsing a string on every
        // lookup), and the redundancy becomes this check instead — a number
        // typed into the wrong gaṇa's block still names a real upstream row,
        // so nothing else would catch it.
        //
        // Mapped variant → prefix, not the inverse: this engine covers seven
        // of the ten gaṇas, so 03, 08 and 10 have no `Gana` variant.
        for d in dhatus() {
            let expected = match d.gana {
                Gana::Bhvadi => "01",
                Gana::Adadi => "02",
                Gana::Divadi => "04",
                Gana::Svadi => "05",
                Gana::Tudadi => "06",
                Gana::Rudhadi => "07",
                Gana::Kryadi => "09",
            };
            assert!(
                d.dhatupatha.starts_with(expected),
                "{:?} root {} has number {}, which is not in gaṇa {expected}",
                d.gana,
                d.code,
                d.dhatupatha
            );
        }
    }
```

- [ ] **Step 3: Run to verify it fails**

```bash
mise exec -- cargo test -p panini-data 2>&1 | tail -20
```

Expected: **compile error**, `no field 'dhatupatha' on type '&Dhatu'`. That is the correct failure — the field does not exist yet.

- [ ] **Step 4: Add the field to the struct**

In `crates/panini-data/src/lib.rs`, insert as the first field of `pub struct Dhatu` (immediately after the opening brace at `:65`), leaving the existing `id` field and its doc comment untouched for now:

```rust
    /// Dhātupāṭha entry number — the unique key. Names a row of
    /// `data/dhatupatha.tsv`, and `dhatupatha_numbers_resolve_upstream`
    /// checks that the row it names is the right one, by it-stripping that
    /// row's upadeśa and comparing against `code`.
    pub dhatupatha: &'static str,
```

- [ ] **Step 5: Populate all 49 rows**

Add a `dhatupatha:` line as the first field of every `Dhatu { … }` literal in `DHATUS`. The values, **in the table's existing order** (the order below is exactly the order the rows appear in the file, so this can be applied top to bottom):

| # | `id` | `dhatupatha` | # | `id` | `dhatupatha` |
| --- | --- | --- | --- | --- | --- |
| 1 | `BU` | `01.0001` | 26 | `vA` | `02.0045` |
| 2 | `nI` | `01.1049` | 27 | `ad` | `02.0001` |
| 3 | `ji` | `01.0642` | 28 | `As` | `02.0011` |
| 4 | `smf` | `01.1082` | 29 | `vas` | `02.0013` |
| 5 | `paW` | `01.0381` | 30 | `SI` | `02.0026` |
| 6 | `vad` | `01.1164` | 31 | `kliS` | `09.0058` |
| 7 | `eD` | `01.0002` | 32 | `guD` | `09.0053` |
| 8 | `laB` | `01.1130` | 33 | `aS` | `09.0059` |
| 9 | `sev` | `01.0574` | 34 | `muz` | `09.0066` |
| 10 | `vft` | `01.0862` | 35 | `vrI` | `09.0040` |
| 11 | `BAz` | `01.0696` | 36 | `vf` | `09.0045` |
| 12 | `Ikz` | `01.0694` | 37 | `Ap` | `05.0016` |
| 13 | `div` | `04.0001` | 38 | `Sak` | `05.0017` |
| 14 | `naS` | `04.0091` | 39 | `hi` | `05.0012` |
| 15 | `kup` | `04.0146` | 40 | `ri` | `05.0032` |
| 16 | `man` | `04.0073` | 41 | `aS.5` | `05.0020` |
| 17 | `yuD` | `04.0069` | 42 | `stiG` | `05.0021` |
| 18 | `vid` | `04.0067` | 43 | `kft` | `07.0010` |
| 19 | `tud` | `06.0001` | 44 | `his` | `07.0019` |
| 20 | `liK` | `06.0092` | 45 | `Kid` | `07.0012` |
| 21 | `viS` | `06.0160` | 46 | `Banj` | `07.0016` |
| 22 | `juz` | `06.0008` | 47 | `piz` | `07.0015` |
| 23 | `vij` | `06.0009` | 48 | `inD` | `07.0011` |
| 24 | `gur` | `06.0131` | 49 | `ruD` | `07.0001` |
| 25 | `yA` | `02.0044` | | | |

So `BU`'s row becomes:

```rust
    Dhatu {
        dhatupatha: "01.0001",
        id: "BU",
        code: "BU",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "sattAyAm",
    },
```

**Nine rows already carry their number in a comment above them** (`07.0001`, `07.0010`, `07.0011`, `07.0012`, `07.0015`, `07.0016`, `07.0019`, `05.0020`, `05.0021`). Confirm each matches the table above as you go — they were recorded independently by earlier slices, so a disagreement means the table is wrong, not the comment. **Leave those comments in place**; they carry the upadeśa and per-root notes, not just the number.

- [ ] **Step 6: Run tests to verify they pass**

```bash
mise exec -- cargo test -p panini-data 2>&1 | tail -20
```

Expected: PASS, including `dhatupatha_numbers_resolve_upstream` and `gana_matches_dhatupatha_prefix`. If `dhatupatha_numbers_resolve_upstream` fails on a specific root, the assertion message names the number, the upstream upadeśa, what it stripped to, and what DHATUS stores — fix the number, not the normalizer, unless the same root also appears in the nine hand-recorded comments.

- [ ] **Step 7: Run the full suite**

```bash
mise run test 2>&1 | tail -15
```

Expected: all green. Nothing outside `panini-data` has changed yet.

- [ ] **Step 8: Commit**

```bash
git add data/dhatupatha.tsv crates/panini-data/src/lib.rs
git commit -m "feat(data): the dhAtupATha number becomes a real field

Vendors vidyut-prakriya's dhatupatha.tsv at 8da2f90 (all ten ganas, MIT,
commit pinned in the header) in place of our hand-synced 49-row mirror,
and adds Dhatu.dhatupatha for all 49 curated roots.

Two tests hold the assignments honest. The load-bearing one it-strips each
upstream upadesa (1.3.2/1.3.5/1.3.3, then 6.1.64) and compares against our
stored code: number and artha alone cannot do this, because upstream has
8- and 15-way artha collisions. The other pins gana against the number's
prefix, converting a redundancy into a check.

All nine numbers earlier slices recorded by hand in comments agree with
what the normalizer resolves independently."
```

---

### Task 2: Correct the three divergent arthas

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (three `artha` values, and the artha assertion in the test module)

**Interfaces:**
- Consumes: `Dhatu.dhatupatha` and `upstream_rows()` from Task 1.
- Produces: nothing new. `artha` for `04.0001`, `02.0045`, `07.0015` now matches upstream byte-for-byte.

- [ ] **Step 1: Write the failing assertion**

In `dhatupatha_numbers_resolve_upstream`, change the destructuring to bind the artha and add the comparison. Replace:

```rust
            let (_, upadesha, _) = rows
                .iter()
                .find(|(n, _, _)| *n == d.dhatupatha)
                .unwrap_or_else(|| panic!("{} names no upstream row", d.dhatupatha));
```

with:

```rust
            let (_, upadesha, artha) = rows
                .iter()
                .find(|(n, _, _)| *n == d.dhatupatha)
                .unwrap_or_else(|| panic!("{} names no upstream row", d.dhatupatha));
            assert_eq!(
                *artha, d.artha,
                "{} artha diverges from upstream",
                d.dhatupatha
            );
```

- [ ] **Step 2: Run to verify it fails**

```bash
mise exec -- cargo test -p panini-data dhatupatha_numbers_resolve_upstream 2>&1 | tail -15
```

Expected: FAIL on `04.0001` (the first divergent row in table order), reporting `krIqAyAm` against upstream's long compound.

- [ ] **Step 3: Correct the three arthas**

Exactly three rows change. Each takes upstream's value verbatim:

| number | root | was | becomes |
| --- | --- | --- | --- |
| `04.0001` | √div | `krIqAyAm` | `krIqAvijigIzAvyavahAradyutistutimodamadasvapnakAntigatizu` |
| `02.0045` | √vā | `gatigandhanayoH` | `gatiganDanayoH` |
| `07.0015` | √piṣ | `saYcUrRane hiMsAyAM ca` | `saYcUrRane hiMsAyAm ca` |

√vā's old value is malformed SLP1: `ndh` reads as *d* + *h*, where the aspirate `D` is meant. √piṣ's is an anusvāra slip. √div's looks like a deliberate abbreviation of a ten-sense compound, and upstream's full string is adopted because after this slice the field's job is provenance rather than gloss.

**Check for stale assertions on these values.** `curated_roots_have_expected_ganas_and_padas` pins `artha` for several roots by hand (e.g. `assert_eq!(vas.artha, "AcCAdane")`). Grep before running:

```bash
grep -n 'krIqAyAm\|gatigandhanayoH\|hiMsAyAM ca' crates/ data/ README.md AGENTS.md docs/ARCHITECTURE.md -r
```

Update every hit outside `docs/superpowers/specs/` and `docs/superpowers/plans/` (those are historical and stay as written).

- [ ] **Step 4: Run tests to verify they pass**

```bash
mise run test 2>&1 | tail -15
```

Expected: all green. **No form string may have changed** — no rule reads `artha`.

- [ ] **Step 5: Commit**

```bash
git add crates/panini-data/src/lib.rs
git commit -m "fix(data): three arthas diverged from the dhAtupATha

Adding the artha assertion to the upstream resolution test turns up three
rows our hand-synced mirror had drifted on. One is a real bug: 02.0045
vA's gatigandhanayoH is malformed SLP1 -- ndh reads as d + h where the
aspirate D is meant. 07.0015 piz was an anusvara slip. 04.0001 div was an
abbreviation of a ten-sense compound; upstream's full string is adopted,
since the field's job is now provenance rather than gloss.

No rule reads artha, so no form changes."
```

---

### Task 3: Re-key the golden tables

**Files:**
- Modify: `crates/panini/tests/paradigm.rs` (`PARADIGM` 200 keys, `ALTERNATES` 242 keys, six lookups, two doc comments)
- Create: `/tmp/rekey.py` (throwaway; not committed)

**Interfaces:**
- Consumes: `Dhatu.dhatupatha` from Task 1.
- Produces: `PARADIGM` and `ALTERNATES` whose first column is a number. Task 5 depends on no `.id` remaining in this file.

**This task carries the plan's only real risk.** A block whose key changed *and* whose forms changed would still be self-consistent, and the golden suite could not see it — the suite checks that the engine reproduces the table, not that the table still says what it said yesterday. So the re-key is mechanical and the diff is verified structurally.

- [ ] **Step 1: Write the re-key script**

Create `/tmp/rekey.py`. It rewrites only the **first string literal of each tuple** inside the two const bodies, so a form string or lakāra name that happens to match an id can never be hit:

```python
import re, sys

ID_TO_NUMBER = {
    "BU": "01.0001", "nI": "01.1049", "ji": "01.0642", "smf": "01.1082",
    "paW": "01.0381", "vad": "01.1164", "eD": "01.0002", "laB": "01.1130",
    "sev": "01.0574", "vft": "01.0862", "BAz": "01.0696", "Ikz": "01.0694",
    "div": "04.0001", "naS": "04.0091", "kup": "04.0146", "man": "04.0073",
    "yuD": "04.0069", "vid": "04.0067", "tud": "06.0001", "liK": "06.0092",
    "viS": "06.0160", "juz": "06.0008", "vij": "06.0009", "gur": "06.0131",
    "yA": "02.0044", "vA": "02.0045", "ad": "02.0001", "As": "02.0011",
    "vas": "02.0013", "SI": "02.0026", "kliS": "09.0058", "guD": "09.0053",
    "aS": "09.0059", "muz": "09.0066", "vrI": "09.0040", "vf": "09.0045",
    "Ap": "05.0016", "Sak": "05.0017", "hi": "05.0012", "ri": "05.0032",
    "aS.5": "05.0020", "stiG": "05.0021", "kft": "07.0010", "his": "07.0019",
    "Kid": "07.0012", "Banj": "07.0016", "piz": "07.0015", "inD": "07.0011",
    "ruD": "07.0001",
}

path = "crates/panini/tests/paradigm.rs"
lines = open(path).read().split("\n")
out, depth, in_const, expect_key, changed = [], 0, False, False, 0

for line in lines:
    stripped = line.strip()
    if stripped.startswith("const PARADIGM") or stripped.startswith("const ALTERNATES"):
        in_const, depth = True, 0
    if in_const:
        # A tuple opens either as a bare `(` on its own line, or inline as `("x", ...`.
        if stripped == "(" or stripped.startswith('("'):
            depth += 1
            if depth == 1:
                expect_key = True
        if expect_key:
            m = re.search(r'"([^"]*)"', line)
            if m and m.group(1) in ID_TO_NUMBER:
                line = line[:m.start(1)] + ID_TO_NUMBER[m.group(1)] + line[m.end(1):]
                changed += 1
                expect_key = False
            elif m:
                sys.exit(f"unrecognised key {m.group(1)!r} on: {line}")
        if stripped.startswith(")") or stripped.endswith("),"):
            depth = max(0, depth - 1)
        if stripped == "];":
            in_const, depth = False, 0
    out.append(line)

open(path, "w").write("\n".join(out))
print(f"rewrote {changed} keys")
```

- [ ] **Step 2: Run it**

```bash
cd /workspace && python3 /tmp/rekey.py
```

Expected: `rewrote 442 keys` — exactly 200 `PARADIGM` blocks plus 242 `ALTERNATES` rows. **Any other number means stop and diagnose**; do not proceed on a partial rewrite.

- [ ] **Step 3: Verify the diff structurally**

Every changed line must differ *only* by an id becoming a number. This check reads the diff and fails if any changed pair is anything else:

```bash
git diff -U0 crates/panini/tests/paradigm.rs | python3 -c '
import sys, re
old, new = [], []
for l in sys.stdin:
    if l.startswith("-") and not l.startswith("---"): old.append(l[1:])
    elif l.startswith("+") and not l.startswith("+++"): new.append(l[1:])
assert len(old) == len(new) == 442, f"expected 442 changed lines, got {len(old)}/{len(new)}"
num = re.compile(r"^\d\d\.\d{4}$")
for o, n in zip(old, new):
    mo, mn = re.search(r"\"([^\"]*)\"", o), re.search(r"\"([^\"]*)\"", n)
    assert num.match(mn.group(1)), f"new key not a number: {n.strip()}"
    assert o[:mo.start(1)] == n[:mn.start(1)], f"prefix changed: {o.strip()}"
    assert o[mo.end(1):] == n[mn.end(1):], f"suffix changed: {o.strip()}"
print("all 442 changed lines differ only in the key column")
'
```

Expected: `all 442 changed lines differ only in the key column`. This is what rules out a form string having moved.

- [ ] **Step 4: Point the six lookups at the new key**

The tables now hold numbers, so `find(|d| d.id == *root)` no longer matches. In `crates/panini/tests/paradigm.rs`, change all six sites (at approximately `:3065`, `:3119`, `:3146`, `:3200`, `:3233`, `:3401`) from `d.id` to `d.dhatupatha`:

```bash
sed -i 's/d\.id == \*root/d.dhatupatha == *root/g; s/d\.id == id/d.dhatupatha == id/g; s/(d\.id, panini::lakara_name/(d.dhatupatha, panini::lakara_name/g' crates/panini/tests/paradigm.rs
grep -n '\.id\b' crates/panini/tests/paradigm.rs
```

Expected: the grep prints nothing.

The helper at `:3064` is named for its parameter, not the field, so rename its parameter for honesty:

```rust
fn lan_a_form(number: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
```

and update its `assert_eq!` message and its call sites (`lan_a_form("Ikz", …)` → `lan_a_form("01.0694", …)`) accordingly. Find them with:

```bash
grep -n 'lan_a_form(' crates/panini/tests/paradigm.rs
```

- [ ] **Step 5: Update the two doc comments that describe the old key**

At `:10-14`, `PARADIGM`'s doc comment says the first column is a `Dhatu::id` that is "gaṇa-qualified, so the two √aś rows stay distinct: `aS.5` vs `aS`". Replace that clause with:

```rust
/// `PARADIGM`'s first column is a `Dhatu::dhatupatha` — the entry number,
/// unique by construction, so the two √aś rows are distinct without anyone
/// deciding which gaṇa's was the incumbent (`09.0059` kryādi, `05.0020`
/// svādi). Resolve a number against the `DHATUS` table in `panini-data` to
/// see which root a block is for; the tables carry no per-row comment,
/// deliberately, since 442 uncheckable comments is a staleness liability no
/// test could pin.
```

At `:3105-3117`, `every_form_validates_and_matches`'s comment makes the same claim. Rewrite its first sentences to:

```rust
        // `PARADIGM`'s first column is a `Dhatu::dhatupatha`, but
        // `Analysis::dhatu` reports the surface `code` (deliberately not
        // unique — it's a user-facing spelling, not a key). The two must be
        // resolved against each other rather than compared directly. Because
        // both √aś rows share `code == "aS"`, matching on `code` alone would
        // let a mis-transcribed row silently bind to the WRONG root's forms
        // as long as the two roots' surfaces happen to be disjoint.
```

Leave the rest of that comment (the `row_pada` reasoning) exactly as it stands — it is about pada, not about keys. Do the same for `every_alternate_validates_and_matches`'s doc comment at `:3139-3142`, which says "same `Dhatu::id` → `code` resolution".

- [ ] **Step 6: Run the full suite**

```bash
mise run test 2>&1 | tail -15
```

Expected: all green, 1800 cells, no form change.

- [ ] **Step 7: Commit**

```bash
git add crates/panini/tests/paradigm.rs
git commit -m "refactor(test): the goldens key on the dhAtupATha number

442 keys -- 200 PARADIGM blocks and 242 ALTERNATES rows -- re-keyed
mechanically, with the diff checked line by line to confirm every change is
an id becoming a number and nothing else moved. A block whose key AND forms
changed would be self-consistent and invisible to the suite, which checks
that the engine reproduces the table, not that the table still says what it
said yesterday.

The tables carry no per-root comment to replace the lost greppability: 442
uncheckable comments is a staleness liability nothing could pin, and a
number resolves against DHATUS."
```

---

### Task 4: Re-key the data crate's own tests

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (test module, ~25 lookups and three tests that exist only to police `id`)

**Interfaces:**
- Consumes: `Dhatu.dhatupatha`.
- Produces: no `.id` reference anywhere in the crate, so Task 5 can delete the field.

- [ ] **Step 1: Re-key the lookups in `curated_roots_have_expected_ganas_and_padas`**

Every `find(|d| d.id == "X")` becomes `find(|d| d.dhatupatha == "N")`, using the Task 1 table. The nineteen sites and their new keys:

`BU`→`01.0001`, `laB`→`01.1130`, `eD`→`01.0002`, `Ikz`→`01.0694`, `div`→`04.0001`, `tud`→`06.0001`, `yA`→`02.0044`, `vA`→`02.0045`, `As`→`02.0011`, `vas`→`02.0013`, `SI`→`02.0026`, `kliS`→`09.0058`, `guD`→`09.0053`, `aS`→`09.0059`, `muz`→`09.0066`, `vrI`→`09.0040`, `vf`→`09.0045`.

The svādi loop at `:607-612` becomes:

```rust
        // New: svādi (gaṇa 5), all four parasmaipadī.
        for number in ["05.0016", "05.0017", "05.0012", "05.0032"] {
            let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
            assert!(matches!(d.gana, Gana::Svadi));
            assert!(matches!(d.pada, PadaAssignment::Parasmaipada));
        }
```

Also `:652` (`ad`→`02.0001`) and `:660` (`As`→`02.0011`) in the two tests below it.

- [ ] **Step 2: Replace `id_is_the_lookup_key_and_is_unique`**

Delete it entirely (`:666-689`) and put this in its place. The old test's whole second half checked a contract — "id equals code except for two exceptions" — that no longer exists:

```rust
    #[test]
    fn dhatupatha_is_the_key_and_is_unique() {
        let keys: Vec<&str> = dhatus().iter().map(|d| d.dhatupatha).collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), keys.len(), "dhātupāṭha numbers must be unique");
        // Uniqueness here is a property of the source, not of a convention
        // this repo maintains: upstream numbers are unique across all 2260
        // entries, which `dhatupatha_numbers_resolve_upstream` also asserts.
        // That is the whole reason the number can serve as the key where the
        // SLP1 `code` could not — `code` is NOT unique (both √aś rows share
        // it), and the retired `Dhatu::id` existed only to paper over that.
        for d in dhatus() {
            assert_eq!(
                d.dhatupatha.len(),
                7,
                "{} is not a well-formed dhātupāṭha number",
                d.dhatupatha
            );
        }
    }
```

- [ ] **Step 3: Rewrite `the_two_ash_roots_are_distinct_rows`**

At `:692-694`, the two lookups change and the closing comment's claim about ids is now false:

```rust
        let svadi = dhatus().iter().find(|d| d.dhatupatha == "05.0020").unwrap();
        let kryadi = dhatus().iter().find(|d| d.dhatupatha == "09.0059").unwrap();
```

Replace the comment "Same surface text, different rows. If ids ever collapse, one of these roots silently stops being derivable." with:

```rust
        // Same surface text, different rows — and now distinct by
        // construction rather than by a hand-applied qualifier, since their
        // numbers come from different gaṇas of the source.
```

- [ ] **Step 4: Rewrite `rudhadi_ids_do_not_collide`**

Its premise dissolves with `id`. Rename it and drop the `aS.5` pin (`:769-786`), keeping the rudhādi row inventory, which is still worth pinning:

```rust
    #[test]
    fn rudhadi_rows_are_the_seven_curated_roots() {
        // rudhādi also holds `vi\da~\` and `o~vijI~`, whose SLP1 surfaces
        // WOULD have collided with divādi's `vid` and tudādi's `vij` under
        // the retired `id` scheme. Neither is curated — the gaṇa stops at
        // seven roots — and under number keying the question no longer
        // arises: `07.0013` and `07.0023` would be distinct from `04.0067`
        // and `06.0009` whether or not their surfaces agree. The whole
        // `aS.5` qualification mechanism retired with the field.
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
            ]
        );
    }
```

Note the second column is now `code`, so `07.0019` reads `hins` where the old tuple's first column read `his` — the lookup-key-vs-stored-form split is exactly what retires here.

- [ ] **Step 5: Fix the remaining `.id` in an error message**

At `:763`, `every_curated_root_admits_at_least_one_pada` interpolates `d.id`:

```rust
            assert!(
                !d.pada.padas().is_empty(),
                "{} admits no pada at all",
                d.dhatupatha
            );
```

- [ ] **Step 6: Verify no `.id` remains**

```bash
grep -n '\.id\b\|d\.id\|"aS\.5"' crates/panini-data/src/lib.rs | grep -v '^\s*[0-9]*:\s*///'
```

Expected: only the 49 `id: "…"` field initialisers and the struct's own `pub id` declaration and doc comment — all of which Task 5 deletes. No *reads* of `.id`.

- [ ] **Step 7: Run the full suite**

```bash
mise run test 2>&1 | tail -15
```

Expected: all green.

- [ ] **Step 8: Commit**

```bash
git add crates/panini-data/src/lib.rs
git commit -m "refactor(data): the crate's own tests key on the number

Re-keys ~25 lookups, and rewrites the three tests that existed only to
police id's uniqueness. id_is_the_lookup_key_and_is_unique checked a
contract -- 'id equals code except for two exceptions' -- that no longer
exists; rudhadi_ids_do_not_collide pinned the aS.5 mechanism, whose
question number keying does not raise.

The rudhadi inventory survives, now reading 07.0019 hins where it read
his: the lookup-key-vs-stored-form split is exactly what retires."
```

---

### Task 5: Delete `id`

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (struct field + doc comment at `:66-76`, and 49 `id:` lines)

**Interfaces:**
- Consumes: nothing — every reader was re-keyed in Tasks 3 and 4.
- Produces: `Dhatu` at its final five fields.

- [ ] **Step 1: Delete the field and its doc comment**

Remove `:66-76` entirely — the eleven-line doc comment describing the two exceptions, and `pub id: &'static str,`. `Dhatu` becomes:

```rust
#[derive(Debug, Clone, Copy)]
pub struct Dhatu {
    /// Dhātupāṭha entry number — the unique key. Names a row of
    /// `data/dhatupatha.tsv`, and `dhatupatha_numbers_resolve_upstream`
    /// checks that the row it names is the right one, by it-stripping that
    /// row's upadeśa and comparing against `code`.
    pub dhatupatha: &'static str,
    /// The root's SLP1 text, as it enters the derivation. Deliberately not
    /// unique — both √aś rows spell `aS` — and never a lookup key. Where it
    /// differs from the it-stripped upadeśa the reason is a rule this engine
    /// does not derive: `07.0019` stores `hins` for `hisi~` because 7.1.58
    /// idito num dhātoḥ is kept as a stated simplification, and `05.0021`
    /// stores `stiG` for `zwiGa~\` per 6.1.64 dhātvādeḥ ṣaḥ saḥ.
    pub code: &'static str,
    pub gana: Gana,
    /// Which pada(s) this engine derives for this root — a curated verdict,
    …
}
```

Keep the existing `pada` doc comment verbatim; only `id` goes and `code`'s comment absorbs the two storage notes the `id` comment used to carry.

- [ ] **Step 2: Delete the 49 `id:` lines**

```bash
sed -i '/^        id: "/d' crates/panini-data/src/lib.rs
grep -c '^        id: "' crates/panini-data/src/lib.rs
```

Expected: `0`.

- [ ] **Step 3: Build to verify nothing else read it**

```bash
mise run build 2>&1 | tail -20
```

Expected: clean. A compile error here means a reader was missed in Task 3 or 4 — fix it there in spirit (re-key it), do not reintroduce the field.

- [ ] **Step 4: Run the full suite and the linter**

```bash
mise run test 2>&1 | tail -15 && mise run lint 2>&1 | tail -10
```

Expected: all green, no clippy warnings.

- [ ] **Step 5: Commit**

```bash
git add crates/panini-data/src/lib.rs
git commit -m "refactor(data): retire Dhatu::id

The field existed to be a unique key where the SLP1 code is not, and both
of its documented exceptions were artifacts of that: the aS.5 collision
qualifier, and the his/hins lookup-key-vs-stored-form split. A globally
unique number dissolves both. The two storage notes move onto `code`,
where they describe what they actually are -- rules this engine does not
derive -- rather than exceptions to a key's contract."
```

---

### Task 6: The docs the slice falsifies

**Files:**
- Modify: `AGENTS.md` (`:143`, `:186`, and the audit paragraph around `:269-284`)
- Modify: `crates/panini/src/lib.rs` (`:13-19`)

**Interfaces:** none.

- [ ] **Step 1: Fix `Analysis::dhatu`'s doc comment**

At `crates/panini/src/lib.rs:14-18`, it points readers at a field that no longer exists. Replace the comment with:

```rust
    /// The dhatu's `code` (its SLP1 spelling): this is a user-facing root
    /// spelling, not a lookup key, and is deliberately not guaranteed unique
    /// — two roots in different gaṇas can share an SLP1 form (svādi's
    /// `05.0020` and kryādi's `09.0059` both report `"aS"` here).
    /// `Dhatu::dhatupatha` is the unique key; resolve against it if you need
    /// one.
    pub dhatu: String,
```

- [ ] **Step 2: Rewrite AGENTS.md's audit paragraph**

Find the sentence beginning "One residual circularity is known and unfixed:" and replace it, through the end of "…promoting it to a real field on `Dhatu` first.", with:

```markdown
  The harness resolves each root to a `data/dhatupatha.tsv` entry by its
  **dhātupāṭha number** (`07.0016` for √bhañj), which is `Dhatu::dhatupatha`
  and the root's identity in this repo. That closed the one circularity this
  audit used to carry: selection previously required vidyut to reproduce
  **this engine's own pinned laṭ prathama eka form**, so for a root whose new
  sūtra shaped exactly that cell — √bhañj's `Banakti`, √piṣ's `pinazwi`,
  √indh's `indDe` — the anchoring cell was the one cell the audit could not
  independently validate. The numbers themselves are held honest in-repo by
  `dhatupatha_numbers_resolve_upstream`, which it-strips each vendored
  upadeśa (1.3.2, 1.3.5, 1.3.3, then 6.1.64) and compares it against the
  stored `code` — an assertion that cannot be satisfied by copying back our
  own choice, unlike matching on number or artha alone (upstream has 8- and
  15-way artha collisions).
```

- [ ] **Step 3: Fix the two `Dhatu::id` references**

At `:143`: "√aś (`Dhatu::id` `aS.5`, distinct from …)" becomes "√aś (`05.0020`, distinct from kryādi's `09.0059`)". At `:186`: "two `Dhatu::id` collisions" becomes "two SLP1 surface collisions, which number keying makes moot".

- [ ] **Step 4: Verify no stale claim survives**

```bash
grep -rn 'Dhatu::id\|aS\.5\|residual circularity\|known and unfixed\|next slice should key' README.md AGENTS.md docs/ARCHITECTURE.md crates/ data/
```

Expected: no hit outside `docs/superpowers/specs/` and `docs/superpowers/plans/`, which are historical and stay as written.

- [ ] **Step 5: Commit**

```bash
git add AGENTS.md crates/panini/src/lib.rs
git commit -m "docs: the audit keys on the number, and id is gone

AGENTS.md's 'one residual circularity is known and unfixed' paragraph and
its 'the next slice should...' instruction are both discharged. Two
Dhatu::id references and Analysis::dhatu's pointer at that field are
corrected to name dhatupatha."
```

---

### Task 7: The full gate and the audit re-run

**Files:** none (verification only, plus any fix the gate demands).

- [ ] **Step 1: Full gate**

```bash
mise run fmt-check && mise run lint && mise run test && mise run audit
```

Expected: all clean. If `fmt` has drift, run `mise run fmt` and commit it separately — slice 7b left a `cargo fmt` commit behind (`9899669`) for exactly this reason.

- [ ] **Step 2: Confirm the invariants the spec fixed**

```bash
mise exec -- cargo test -p panini --test paradigm 2>&1 | tail -5
grep -c '^        dhatupatha: "' crates/panini-data/src/lib.rs
```

Expected: paradigm tests pass; the grep prints `49`.

- [ ] **Step 3: Skip the mutation run, and know why**

`mise run mutants` targets `panini-prakriya`. Everything this plan added is `#[cfg(test)]` code in `panini-data`, which cargo-mutants does not mutate, so there are no new mutants, no survivors, and no reason to re-measure the timeout floor or re-read `timeout.txt`.

**Do not run it to "be safe."** A full campaign is hours of wall-clock and, per AGENTS.md, 1200s at `-j 4` against the 1800-cell suite is adequate-but-unverified — running it here would spend that budget on a diff that provably cannot produce a mutant.

- [ ] **Step 4: Re-run the cross-implementation audit**

The harness is out-of-repo, under the vendored vidyut checkout's `examples/`. Re-key its entry selection to look up `data/dhatupatha.tsv` rows **by number** rather than by requiring vidyut to reproduce this engine's pinned laṭ prathama eka form, then re-run the whole corpus: 1800 cells, 2042 forms, 49 roots.

Expected: **zero differences.** Record the vidyut commit with the result.

**A non-zero result is the more interesting outcome and must not be papered over.** If keying on the number changes any cell, the old anchoring was resolving some root to the wrong upstream entry, and the slice has found a real bug. Report it rather than adjusting the number to restore agreement.

- [ ] **Step 5: Open the PR**

```bash
git push -u origin dhatupatha-number-identity
gh pr create --fill
```

---

## Deferred, and why

Recorded here so the next slice does not have to re-derive it:

- **It-stripping as real sūtras** (1.3.2, 1.3.3, 1.3.5, 1.3.9), which would let `code` be *derived* from a stored upadeśa rather than curated — retiring the hand-written `code` column the way this slice retires `id`. The normalizer here is deliberately test-private so it does not pre-empt that design. Upadeśa preprocessing is not the tiṅanta pipeline `TINANTA_RULES` models, so it needs its own pipeline concept.
- **Auditing the 49 roots for mis-assigned pada.** √tud (`06.0001`) remains the known case. This slice makes it visible for the first time in the data: the vendored upadeśa `tu\da~^` carries the svarita `^` that 1.3.72 reads, sitting one column from a `PadaAssignment::Parasmaipada` that contradicts it.
- **The eight remaining `~^r` rudhādi roots**, and 8.2.30 *coḥ kuḥ*'s generalisation past `j`. Unchanged by this slice.
