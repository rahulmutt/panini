# Adādi √vas + 8.2.25 *dhi ca* Correction (slice 5e) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Land 8.2.25 *dhi ca*, correcting three wrong √ās forms shipped in slice 5d, delete the now-unreachable 8.4.53, and add √vas as the corrected rule's second witness.

**Architecture:** 8.2.25 becomes a self-guarding entry in the ordered `TINANTA_RULES` array in `crates/panini-prakriya/src/tinanta.rs`, placed between 8.2.23 and 8.3.15 — its numeric tripādī slot, which is also what makes it bleed the 8.4 voiced junction. √vas is registered as data only (`panini-data` + `data/dhatupatha.tsv`); it needs no grammar of its own. Everything else is tests and docs.

**Tech Stack:** Rust 1.97.1 pinned via `mise`; `cargo test` / `cargo clippy` / `cargo fmt` / `cargo mutants` driven through `mise run <task>`.

**Spec:** `docs/superpowers/specs/2026-07-24-adadi-vas-dhi-ca-5e-design.md`

## Global Constraints

- Toolchain is pinned via `mise` to rust 1.97.1. Never install Rust globally. Run tasks as `mise run build | test | lint | fmt | fmt-check | mutants | audit`.
- To scope a test run to one crate, `mise run test -- -p X` does **not** work. Use `mise exec -- cargo test -p X`.
- `cargo-mutants` must be run in a **foreground** shell; the mise shim fails in background shells.
- SLP1 is the only internal representation. Transliterate only in `panini-lipi`.
- `#![forbid(unsafe_code)]` in every non-fuzz crate.
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, never as a branch inside `derive`.
- Golden surface forms and trace order in `crates/panini/tests/paradigm.rs` and `crates/panini/tests/trace.rs` are the source of truth. **Write goldens from the independent reference (vidyut-prakriya), never from the engine's own output** — that inversion is what caused the 5d error this plan corrects.
- `mise run mutants` must reach **0 missed and 0 survivors**. Do not weaken the gate; add a distinguishing negative pin instead.
- Every task ends green: `mise run test` passes before you commit.
- Work on branch `adadi-vas-dhi-ca-5e` (already created; the design spec is committed there as `bfc8c1a`).

## Reference: the corrected forms

Cross-checked against vidyut-prakriya. These are the authority for every golden in this plan.

√ās (correcting 5d) — only the 2pl cells change:

| lakāra | 2pl (madhyama bahu) |
| --- | --- |
| laṭ | `ADve` (was `AdDve`) |
| laṅ | `ADvam` (was `AdDvam`) |
| loṭ | `ADvam` (was `AdDvam`) |

√vas (new), rows read prathama / madhyama / uttama:

| | eka | dvi | bahu |
| --- | --- | --- | --- |
| **laṭ** | `vaste` / `vasse` / `vase` | `vasAte` / `vasATe` / `vasvahe` | `vasate` / `vaDve` / `vasmahe` |
| **laṅ** | `avasta` / `avasTAH` / `avasi` | `avasAtAm` / `avasATAm` / `avasvahi` | `avasata` / `avaDvam` / `avasmahi` |
| **loṭ** | `vastAm` / `vassva` / `vasE` | `vasAtAm` / `vasATAm` / `vasAvahE` | `vasatAm` / `vaDvam` / `vasAmahE` |
| **vidhiliṅ** | `vasIta` / `vasITAH` / `vasIya` | `vasIyAtAm` / `vasIyATAm` / `vasIvahi` | `vasIran` / `vasIDvam` / `vasImahi` |

## File Structure

- `crates/panini-prakriya/src/tinanta.rs` — add the 8.2.25 `Rule`; delete the 8.4.53 `Rule` and its `is_jhas` / `jastva_of` helpers; add and revise unit/guard tests. (Tasks 1, 2, 4)
- `crates/panini/tests/paradigm.rs` — correct three √ās cells; add four √vas golden blocks; add negative pins. (Tasks 1, 3, 4)
- `crates/panini/tests/trace.rs` — rewrite the `AdDve` ordered-trace pin as `ADve`; add a `vaDve` pin. (Tasks 1, 4)
- `crates/panini-data/src/lib.rs` — add the √vas `Dhatu` entry; bump the root-count test. (Task 3)
- `data/dhatupatha.tsv` — mirror the √vas row. (Task 3)
- `AGENTS.md`, `README.md`, `docs/ARCHITECTURE.md`, `docs/superpowers/specs/2026-07-22-adadi-gana-design.md` — documentation. (Task 5)

---

## Task 1: Land 8.2.25 *dhi ca* and correct the √ās forms

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` — insert the new `Rule` between the 8.2.23 and 8.3.15 entries (currently lines 1536–1575); rewrite the `voiced_junction_s_becomes_d_before_dhve` test (currently lines ~3401–3421)
- Modify: `crates/panini/tests/paradigm.rs:1092,1099,1106`
- Modify: `crates/panini/tests/trace.rs` — the `addhve_trace_ends_in_voiced_junction` test (currently lines ~457–470)

**Interfaces:**
- Consumes: the existing `ANGA` constant (`crates/panini-prakriya/src/tinanta.rs:122`), `Prakriya::snapshot` / `Prakriya::record`, `RuleKind::Vidhi`, and the test helper `form_g(code, lakara, purusha, vacana) -> String` defined in `tinanta.rs`'s test module.
- Produces: a `TINANTA_RULES` entry with `id: "8.2.25"`, `name: "Di ca"`. After this task, `derive("As", Lat, Atmanepada, Madhyama, Bahu).text() == "ADve"`. Tasks 3 and 4 depend on this rule existing.

- [ ] **Step 1: Rewrite the √ās junction unit test to the corrected expectation**

In `crates/panini-prakriya/src/tinanta.rs`, replace the whole `voiced_junction_s_becomes_d_before_dhve` test with:

```rust
    #[test]
    fn dhi_ca_elides_s_before_dhve() {
        // √ās 2pl: the root-final `s` meets the `Dh` of Dve/Dvam and is
        // ELIDED by 8.2.25 dhi ca — it is not voiced to `d`. 8.2.25 sits at
        // 8.2 in the tripādī and is asiddha to 8.4, so the `s` is gone before
        // any 8.4 junction rule can look at it: As + Dve -> A + Dve -> ADve.
        assert_eq!(
            form_g("As", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
            "ADve"
        );
        assert_eq!(
            form_g("As", Lakara::Lan, Purusha::Madhyama, Vacana::Bahu),
            "ADvam"
        );
        assert_eq!(
            form_g("As", Lakara::Lot, Purusha::Madhyama, Vacana::Bahu),
            "ADvam"
        );
        // Guard boundary: the affix must be Dh-initial. A clean `s`-meets-`s`
        // cell is untouched, so 2sg stays Asse — the rule must not
        // over-apply.
        assert_eq!(
            form_g("As", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
            "Asse"
        );
    }
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `mise exec -- cargo test -p panini-prakriya dhi_ca_elides_s_before_dhve 2>&1 | tail -20`
Expected: FAIL — `assertion \`left == right\` failed: left: "AdDve", right: "ADve"`. The engine still applies 8.4.53.

- [ ] **Step 3: Add the 8.2.25 rule**

In `crates/panini-prakriya/src/tinanta.rs`, insert this `Rule` immediately **after** the `8.2.23 saMyogAntasya lopaH` entry and **before** the `8.3.15 KaravasAnayor visarjanIyaH` entry:

```rust
    // 8.2.25 dhi ca: the final `s` of the term preceding a `Dh`-initial affix
    // is ELIDED — not voiced. As + Dve -> A + Dve -> ADve; vas + Dve -> vaDve
    // (the sūtra's own stock example, and this slice's second witness).
    //
    // Placement is the whole point: 8.2 is asiddha to 8.4, so this fires
    // before any 8.4 junction rule and the `s` never survives to take a jaś
    // substitute. Slice 5d analysed this junction as 8.4.53 jaśtva (s → d)
    // and shipped *AdDve; 8.2.25 bleeds that rule completely, which is why
    // 8.4.53 has no reachable witness and was removed.
    //
    // The guard reads the term PRECEDING the Dh-initial affix, not the aṅga.
    // In laṭ/laṅ/loṭ the śap is luk'd (empty) so the aṅga is what precedes
    // the ending and the rule fires. In the vidhiliṅ the sīyuṭ residue sits
    // between (AsIDvam), so the first non-empty term after the aṅga does not
    // begin with `D` and the rule correctly declines — the `s` is retained.
    Rule {
        id: "8.2.25",
        name: "Di ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // The first non-empty term after the aṅga (śap, if present, is
            // luk'd/empty for adādi) must be the Dh-initial affix.
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
            if !p.terms[next_idx].text.starts_with('D') {
                return false;
            }
            // The nearest non-empty term before it must end in `s`.
            let prev_idx = p.terms[..next_idx]
                .iter()
                .enumerate()
                .rev()
                .find(|(_, t)| !t.text.is_empty())
                .map(|(i, _)| i);
            let Some(prev_idx) = prev_idx else {
                return false;
            };
            if !p.terms[prev_idx].text.ends_with('s') {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[prev_idx].text.chars().collect();
            s.pop();
            p.terms[prev_idx].text = s.into_iter().collect();
            p.record("8.2.25", "Di ca", before);
            true
        },
    },
```

- [ ] **Step 4: Run the unit test to verify it passes**

Run: `mise exec -- cargo test -p panini-prakriya dhi_ca_elides_s_before_dhve 2>&1 | tail -10`
Expected: PASS.

- [ ] **Step 5: Correct the three √ās golden cells**

In `crates/panini/tests/paradigm.rs`, in the three `"As"` blocks, change the sixth (madhyama bahu) entry:

- line 1092 (`"As", "laT"`): `"AdDve"` → `"ADve"`
- line 1099 (`"As", "laN"`): `"AdDvam"` → `"ADvam"`
- line 1106 (`"As", "loT"`): `"AdDvam"` → `"ADvam"`

After the edit those three arrays read:

```rust
            "Aste", "AsAte", "Asate", "Asse", "AsATe", "ADve", "Ase", "Asvahe", "Asmahe",
```
```rust
            "Asta", "AsAtAm", "Asata", "AsTAH", "AsATAm", "ADvam", "Asi", "Asvahi", "Asmahi",
```
```rust
            "AstAm", "AsAtAm", "AsatAm", "Assva", "AsATAm", "ADvam", "AsE", "AsAvahE", "AsAmahE",
```

- [ ] **Step 6: Capture the corrected `ADve` trace**

Run: `mise exec -- cargo run -q -p panini-cli -- check ADve --trace`
Expected output — surface `ādhve`, with `8.2.25 Di ca` as the final step:

```
VALID ✓  ādhve (As, laT)
  1.3.12 anudAttaNita Atmanepadam -> As
  3.4.78 tiptasJisipTasTamibvasmas -> AsDvam
  1.2.4 sArvaDAtukam apit -> AsDvam
  3.4.79 wita AtmanepadAnAM wer e -> AsDve
  3.1.68 kartari Sap -> AsSapDve
  1.3.9 tasya lopaH -> AsaDve
  2.4.72 adipraBftiByaH SapaH -> AsDve
  8.2.25 Di ca -> ADve
```

If the captured sequence differs from the one pinned in Step 7, **pin what you captured** and re-check it against the spec's reasoning before proceeding.

- [ ] **Step 7: Rewrite the ordered-trace pin**

In `crates/panini/tests/trace.rs`, replace the whole `addhve_trace_ends_in_voiced_junction` test with:

```rust
#[test]
fn adhve_trace_ends_in_dhi_ca() {
    // √ās adādi ātmanepada laṭ 2pl: Dvam → Dve (3.4.79), śap luk'd (2.4.72),
    // then 8.2.25 dhi ca ELIDES the aṅga-final `s` before the `Dh` of Dve:
    // As + Dve -> A + Dve -> ADve. Slice 5d pinned *AdDve here via 8.4.53
    // jaśtva; that rule is asiddha to this one and has been removed.
    assert_eq!(
        trace_for("ADve"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.68", "1.3.9", "2.4.72", "8.2.25"
        ]
    );
    // Neither junction rule may fire: 8.4.53 is gone, and cartva (8.4.55) is
    // the voiceless junction, which a `Dh` never triggers.
    assert!(!trace_for("ADve").contains(&"8.4.53".to_string()));
    assert!(!trace_for("ADve").contains(&"8.4.55".to_string()));
}
```

- [ ] **Step 8: Run the whole suite**

Run: `mise run test 2>&1 | tail -30`
Expected: PASS. Every crate green — in particular `paradigm.rs`'s `every_form_validates_and_matches` and `trace.rs`.

- [ ] **Step 9: Commit**

```bash
git add crates/panini-prakriya/src/tinanta.rs crates/panini/tests/paradigm.rs crates/panini/tests/trace.rs
git commit -m "fix(prakriya): 8.2.25 dhi ca — √ās 2pl is ADve, not AdDve (corrects 5d)"
```

---

## Task 2: Delete the unreachable 8.4.53

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` — delete the `is_jhas` helper (lines ~89–96), the `jastva_of` helper (lines ~98–116), the `8.4.53` `Rule` entry (lines ~1579–1626), and the `jastva_of_maps_each_jhal_to_its_jas` and `is_jhas_is_voiced_stops_only` tests (lines ~3451–3490)

**Interfaces:**
- Consumes: nothing new.
- Produces: nothing. This task only removes code. `is_jhal` and `cartva_of` **stay** — 8.4.55 cartva still uses both.

**Why this is safe:** `Dvam` / `Dve` is the only jhaś-initial tiṅ ending, and Task 1's 8.2.25 now consumes every case a registered root can reach. 8.4.53 is correct grammar with no reachable witness, and the mutation gate cannot be satisfied for code no test can execute. It returns with the first voiced-stop-final root (√duh / √lih / √dviṣ), all already deferred by the parent adādi design.

- [ ] **Step 1: Delete the rule entry**

In `crates/panini-prakriya/src/tinanta.rs`, delete the entire `8.4.53` block — its leading comment (which begins `// 8.4.53 jhalāṃ jaś jhaśi (voiced junction / jaśtva):`) through the closing `},` of its `Rule { ... }`. The `8.4.55 khari ca` entry that follows it stays untouched and becomes the last entry in `TINANTA_RULES`.

In the `8.4.55` comment immediately below, the phrase `// Placed last: latest tripādī rule (8.4 > 8.3).` is still accurate — leave it.

- [ ] **Step 2: Delete the two helpers**

Delete the `is_jhas` function and its doc comment, and the `jastva_of` function and its doc comment. Leave `is_jhal`, `is_khar` and `cartva_of` in place.

- [ ] **Step 3: Delete the two helper tests**

Delete the `jastva_of_maps_each_jhal_to_its_jas` test and the `is_jhas_is_voiced_stops_only` test in full.

- [ ] **Step 4: Run the whole suite**

Run: `mise run test 2>&1 | tail -30`
Expected: PASS. Nothing else referenced 8.4.53 — the corrected `ADve` path (Task 1) already asserts its absence.

- [ ] **Step 5: Verify no dangling references and no dead code**

Run: `grep -rn '8\.4\.53\|is_jhas\|jastva_of' crates/ && echo FOUND || echo CLEAN`
Expected: `CLEAN` (grep finds nothing, so the `||` branch runs). Documentation references in `AGENTS.md` are Task 5's job; historical spec/plan files under `docs/superpowers/` are deliberately left alone.

Run: `mise run lint`
Expected: PASS with no `dead_code` warning.

- [ ] **Step 6: Format and commit**

```bash
mise run fmt
git add crates/panini-prakriya/src/tinanta.rs
git commit -m "refactor(prakriya): drop 8.4.53 jaśtva — unreachable once 8.2.25 bleeds it"
```

---

## Task 3: Register √vas and pin its golden paradigm

**Files:**
- Modify: `crates/panini-data/src/lib.rs` — append a `Dhatu` entry after the `"As"` entry (currently ends line 220); update `has_twentyeight_curated_roots_with_padas` (currently lines ~260–285)
- Modify: `data/dhatupatha.tsv` — append one row
- Modify: `crates/panini/tests/paradigm.rs` — append four `PARADIGM` blocks after the last `"As"` block (which currently ends at line 1114)

**Interfaces:**
- Consumes: `panini_data::{Dhatu, Gana, Pada}`; the 8.2.25 rule from Task 1.
- Produces: `dhatus()` returns 29 entries including `Dhatu { code: "vas", gana: Gana::Adadi, pada: Pada::Atmanepada, artha: "AcCAdane" }`. `PARADIGM` gains four `("vas", ...)` blocks, taking it to 1044 forms. Task 4's negative pins rely on the root being registered.

- [ ] **Step 1: Add the `Dhatu` entry**

In `crates/panini-data/src/lib.rs`, immediately after the `"As"` entry and before the closing `];` of `DHATUS`, add:

```rust
    Dhatu {
        code: "vas",
        gana: Gana::Adadi,
        pada: Pada::Atmanepada,
        artha: "AcCAdane",
    },
```

Also update the comment that heads the adādi group (currently `// adādi (gaṇa 2) — śap luk (2.4.72). √ad/√yā/√vā parasmaipada; √ās // ātmanepada — covered across all four lakāras (laṭ/laṅ/loṭ/vidhiliṅ).`) to read:

```rust
    // adādi (gaṇa 2) — śap luk (2.4.72). √ad/√yā/√vā parasmaipada; √ās/√vas
    // ātmanepada — covered across all four lakāras (laṭ/laṅ/loṭ/vidhiliṅ).
    // √vas here is `vas` ācchādane (2Ā, "to wear"), NOT the far commoner
    // `vas` nivāse (1P, "to dwell", vasati); artha is the only disambiguator.
```

- [ ] **Step 2: Mirror the row in the TSV**

Append to `data/dhatupatha.tsv` (tab-separated, after the `As` row):

```
vas	adadi	atmanepada	AcCAdane
```

- [ ] **Step 3: Update the root-count test**

In `crates/panini-data/src/lib.rs`, rename `has_twentyeight_curated_roots_with_padas` to `has_twentynine_curated_roots_with_padas`, change `assert_eq!(dhatus().len(), 28);` to `29`, and replace the trailing `// New this slice` block with:

```rust
        // adādi ātmanepada: √ās (slice 5d) and √vas (this slice).
        let as_ = dhatus().iter().find(|d| d.code == "As").unwrap();
        assert!(matches!(as_.gana, Gana::Adadi) && matches!(as_.pada, Pada::Atmanepada));
        let vas = dhatus().iter().find(|d| d.code == "vas").unwrap();
        assert!(matches!(vas.gana, Gana::Adadi) && matches!(vas.pada, Pada::Atmanepada));
        // √vas ācchādane (2Ā), not √vas nivāse (1P) — artha disambiguates.
        assert_eq!(vas.artha, "AcCAdane");
```

- [ ] **Step 4: Run the tests to verify the coverage gate fails**

Run: `mise run test 2>&1 | tail -30`
Expected: FAIL in `paradigm_covers_every_enumerable_cell` — the four `("vas", ...)` pairs are unpinned, so `unpinned` is non-empty while `gated` is empty. This is the coverage gate doing its job: a root cannot be registered without golden rows.

- [ ] **Step 5: Add the four golden blocks**

In `crates/panini/tests/paradigm.rs`, append after the last `"As"` block and before the closing `];` of `PARADIGM`:

```rust
    (
        "vas",
        "laT",
        [
            "vaste", "vasAte", "vasate", "vasse", "vasATe", "vaDve", "vase", "vasvahe", "vasmahe",
        ],
    ),
    (
        "vas",
        "laN",
        [
            "avasta",
            "avasAtAm",
            "avasata",
            "avasTAH",
            "avasATAm",
            "avaDvam",
            "avasi",
            "avasvahi",
            "avasmahi",
        ],
    ),
    (
        "vas",
        "loT",
        [
            "vastAm", "vasAtAm", "vasatAm", "vassva", "vasATAm", "vaDvam", "vasE", "vasAvahE",
            "vasAmahE",
        ],
    ),
    (
        "vas",
        "viDiliN",
        [
            "vasIta",
            "vasIyAtAm",
            "vasIran",
            "vasITAH",
            "vasIyATAm",
            "vasIDvam",
            "vasIya",
            "vasIvahi",
            "vasImahi",
        ],
    ),
```

These 36 forms are the vidyut-prakriya output reproduced in this plan's reference table. Do **not** adjust any cell to match engine output — if a cell disagrees, that is an engine bug to escalate, which is exactly how slice 5d went wrong.

- [ ] **Step 6: Run the tests to verify they pass**

Run: `mise run test 2>&1 | tail -30`
Expected: PASS, including `every_form_validates_and_matches` and `paradigm_covers_every_enumerable_cell` (whose second assertion now checks `PARADIGM.len() + 0 == 29 * 4 == 116`).

- [ ] **Step 7: Format and commit**

```bash
mise run fmt
git add crates/panini-data/src/lib.rs data/dhatupatha.tsv crates/panini/tests/paradigm.rs
git commit -m "feat(data): register √vas (adādi, ātmanepada) + 36-form golden block (1008→1044)"
```

---

## Task 4: Pin the 8.2.25 guard boundaries

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` — add three tests next to `dhi_ca_elides_s_before_dhve`
- Modify: `crates/panini/tests/paradigm.rs` — add entries to `known_nonforms_are_invalid` (currently ends line ~1281)
- Modify: `crates/panini/tests/trace.rs` — add the `vaDve` pin

**Interfaces:**
- Consumes: `form_g` from `tinanta.rs`'s test module; `trace_for` from `trace.rs`; the √vas registration from Task 3.
- Produces: no production code. These pins are what the mutation gate in Task 5 needs in order to reach zero survivors.

**What each pin kills.** 8.2.25's guard has three arms; each gets a witness on both sides:

| arm | fires (under-application killer) | declines (over-application killer) |
| --- | --- | --- |
| affix is `D`-initial | `vaDve`, `ADve` | `vaste`, `vasse`, `avasTAH`, `vassva` |
| preceding term ends in `s` | `vaDve` | `laBaDve`, `alaBaDvam` (śap's `a` precedes) |
| term selection is the affix's neighbour, not the aṅga | `vaDve` | `AsIDvam`, `vasIDvam` (sīyuṭ `I` between) |

- [ ] **Step 1: Add the √vas witness and the `D`-initial boundary test**

In `crates/panini-prakriya/src/tinanta.rs`, immediately after `dhi_ca_elides_s_before_dhve`, add:

```rust
    #[test]
    fn dhi_ca_fires_for_vas_and_only_before_dh() {
        // √vas is the sūtra's second witness: vas + Dve -> va + Dve -> vaDve.
        assert_eq!(
            form_g("vas", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
            "vaDve"
        );
        assert_eq!(
            form_g("vas", Lakara::Lan, Purusha::Madhyama, Vacana::Bahu),
            "avaDvam"
        );
        assert_eq!(
            form_g("vas", Lakara::Lot, Purusha::Madhyama, Vacana::Bahu),
            "vaDvam"
        );
        // The affix must be Dh-initial. These four cells put the same aṅga-
        // final `s` in front of `t`, `T` and `s` and it must survive intact —
        // and they are also the first pins that cartva (8.4.55) leaves an `s`
        // alone before a khar, an arm √ad and √ās could not reach.
        assert_eq!(
            form_g("vas", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "vaste"
        );
        assert_eq!(
            form_g("vas", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
            "vasse"
        );
        assert_eq!(
            form_g("vas", Lakara::Lan, Purusha::Madhyama, Vacana::Eka),
            "avasTAH"
        );
        assert_eq!(
            form_g("vas", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
            "vassva"
        );
    }
```

- [ ] **Step 2: Add the "elided segment must be `s`" boundary test**

Add immediately after:

```rust
    #[test]
    fn dhi_ca_does_not_elide_a_non_s_before_dh() {
        // Thematic ātmanepada √labh keeps its śap `a` in front of Dve/Dvam:
        // the term preceding the affix ends in `a`, not `s`, so 8.2.25 must
        // decline. These are the slice-3 goldens, unchanged.
        assert_eq!(
            form_g("laB", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
            "laBaDve"
        );
        assert_eq!(
            form_g("laB", Lakara::Lan, Purusha::Madhyama, Vacana::Bahu),
            "alaBaDvam"
        );
    }
```

- [ ] **Step 3: Add the term-selection boundary test**

Add immediately after:

```rust
    #[test]
    fn dhi_ca_reads_the_affixs_neighbour_not_the_anga() {
        // vidhiliṅ 2pl: the sīyuṭ residue `I` sits between the aṅga and the
        // Dh-initial affix, so the aṅga's `s` is NOT adjacent to the `Dh` and
        // must be retained: As + I + Dvam -> AsIDvam (never *AIDvam). This is
        // the arm that fails if the guard reads the aṅga instead of the term
        // actually preceding the affix.
        assert_eq!(
            form_g("As", Lakara::VidhiLin, Purusha::Madhyama, Vacana::Bahu),
            "AsIDvam"
        );
        assert_eq!(
            form_g("vas", Lakara::VidhiLin, Purusha::Madhyama, Vacana::Bahu),
            "vasIDvam"
        );
    }
```

- [ ] **Step 4: Run the three new tests**

Run: `mise exec -- cargo test -p panini-prakriya dhi_ca 2>&1 | tail -15`
Expected: PASS — four tests (`dhi_ca_elides_s_before_dhve` plus the three new ones).

- [ ] **Step 5: Add the negative pins**

In `crates/panini/tests/paradigm.rs`, in `known_nonforms_are_invalid`, replace the trailing `"AsDve",` line and its comment with:

```rust
        "Asati",  // √ās is ātmanepada; a parasmaipada ending must not derive
        "Asante", // 3pl must be Asate (7.1.5), never the `ante` of 7.1.3
        // 8.2.25 dhi ca elides the aṅga-final `s` before Dve/Dvam. Both the
        // un-applied shape and slice 5d's jaśtva'd shape are non-words.
        "AsDve",    // s retained: the rule did not fire
        "AdDve",    // 5d's wrong form: s voiced to `d` instead of elided
        "AdDvam",   // ditto, laṅ/loṭ
        "vasDve",   // √vas, s retained
        "vadDve",   // √vas, 5d's wrong analysis
        "avasDvam", // √vas laṅ, s retained
        "vasati",   // √vas is ātmanepada; a parasmaipada ending must not derive
```

- [ ] **Step 6: Add the `vaDve` ordered-trace pin**

First capture it:

Run: `mise exec -- cargo run -q -p panini-cli -- check vaDve --trace`
Expected: surface `vadhve`, ending in `8.2.25 Di ca -> vaDve`.

Then, in `crates/panini/tests/trace.rs`, add immediately after `adhve_trace_ends_in_dhi_ca`:

```rust
#[test]
fn vadhve_trace_is_the_second_dhi_ca_witness() {
    // √vas adādi ātmanepada laṭ 2pl — the cell the commentaries use to state
    // 8.2.25. Same ordered path as ADve on a consonant-initial root: the
    // aṅga-final `s` is elided before the `Dh` of Dve, giving vaDve.
    assert_eq!(
        trace_for("vaDve"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.68", "1.3.9", "2.4.72", "8.2.25"
        ]
    );
    // Non-Dh-initial cells of the same root reach no junction rule at all:
    // the `s` of vaste survives, so cartva must not appear either.
    assert!(!trace_for("vaste").contains(&"8.4.55".to_string()));
    assert!(!trace_for("vaste").contains(&"8.2.25".to_string()));
}
```

If the captured sequence differs from the pinned one, pin what you captured.

- [ ] **Step 7: Run the whole suite**

Run: `mise run test 2>&1 | tail -30`
Expected: PASS.

- [ ] **Step 8: Format and commit**

```bash
mise run fmt
git add crates/panini-prakriya/src/tinanta.rs crates/panini/tests/paradigm.rs crates/panini/tests/trace.rs
git commit -m "test(prakriya): pin 8.2.25 guard boundaries + √vas trace witness"
```

---

## Task 5: Documentation and the full gate

**Files:**
- Modify: `AGENTS.md:26-36` (the golden-paradigm scope note)
- Modify: `README.md:16-27` (the Scope paragraph)
- Modify: `docs/ARCHITECTURE.md` (the adādi coverage paragraph, near the end)
- Modify: `docs/superpowers/specs/2026-07-22-adadi-gana-design.md` (correction footnote on the root table)

**Interfaces:**
- Consumes: everything from Tasks 1–4.
- Produces: the slice's final green state. Nothing depends on this task.

- [ ] **Step 1: Update `AGENTS.md`**

Replace the parenthetical inside the golden-paradigm bullet — from `(`crates/panini/tests/paradigm.rs`, 1008 forms;` through `remaining ātmanepada roots (√vas, √śī) land in later slices)` — with:

```
  (`crates/panini/tests/paradigm.rs`, 1044 forms; bhvādi/divādi/tudādi are
    complete across laṭ/laṅ/loṭ/vidhiliṅ × parasmaipada/ātmanepada, and adādi
    (gaṇa 2) is being added — √yā/√vā/√ad/√ās/√vas are complete across all
    four lakāras (laṭ/laṅ/loṭ/vidhiliṅ). √ad (parasmaipada) lands the
    internal junction sandhi cartva (8.4.55); √ās (ātmanepada) lands 7.1.5
    ātmanepadeṣv anataḥ and extends 6.1.90 āṭaś ca / 6.1.66 lopo vyor vali to
    the athematic (śap-luk'd) ātmanepada path (loṭ 1sg + optative); √vas
    (ātmanepada) is the second witness for 8.2.25 dhi ca, which elides an
    aṅga-final `s` before a Dh-initial affix (ADve, vaDve) — it replaced the
    8.4.53 jaśtva analysis slice 5d shipped, and 8.4.53 was removed as
    unreachable; √śī lands in a later slice)
```

- [ ] **Step 2: Update `README.md`**

Replace the Scope paragraph's root count and adādi sentence so it reads:

```
Finite verbs (*tiṅanta*), four gaṇas — *bhvādi* (1), *divādi* (4), *tudādi*
(6) fully, and *adādi* (2) partially — *parasmaipada* and *ātmanepada* (pada
taken from each root's tag), over a curated 29-root set, in four lakāras:
*laṭ* (present), *laṅ* (imperfect), *loṭ* (imperative), and *vidhiliṅ*
(optative). The adādi entry now covers √yā, √vā (parasmaipada, ā-final) and
√ad (parasmaipada) across all four lakāras, plus √ās and √vas (*ātmanepada*)
across all four lakāras including the athematic (śap-luk'd) ātmanepada
optative. `INVALID` means "not derivable within this covered grammar,"
not "ungrammatical in Sanskrit." See `docs/ARCHITECTURE.md`.
```

- [ ] **Step 3: Update `docs/ARCHITECTURE.md`**

Replace the sentence beginning `adādi is now covered across all four lakāras:` with:

```
adādi is now covered across all four lakāras: √yā and √vā (parasmaipada),
√ad (parasmaipada), and √ās and √vas (ātmanepada) each derive in laṭ, laṅ,
loṭ and vidhiliṅ, including the athematic (śap-luk'd) optative. The
consonant-final ātmanepada roots meet the ending directly, so they are the
engine's junction witnesses: 8.4.55 cartva (√ad), and 8.2.25 dhi ca, which
elides an aṅga-final `s` before a Dh-initial affix (ADve, vaDve).
```

- [ ] **Step 4: Add the correction footnote to the parent adādi design**

`docs/superpowers/specs/2026-07-22-adadi-gana-design.md` is the governing roadmap for slice 5f, and its root table still claims √vas exercises 8.2.39 jaśtva. Immediately below that table (after the "Notes on the curation" list), add:

```markdown
> **Correction (2026-07-24, slice 5e).** The √vas row above and the √ās
> analysis inherited by slice 5d are wrong: the `s` before the `dh` of
> Dve/Dvam is **elided by 8.2.25 *dhi ca***, not voiced by jaśtva (neither
> 8.2.39 *jhalāṃ jaśo'nte* nor 8.4.53 *jhalāṃ jaś jhaśi*). 8.2.25 is asiddha
> to 8.4 and bleeds both. The correct forms are `ADve` / `ADvam` and
> `vaDve` / `vaDvam`. 8.4.53 was removed as unreachable and returns only with
> a voiced-stop-final root (√duh / √lih / √dviṣ). See
> `2026-07-24-adadi-vas-dhi-ca-5e-design.md`.
```

Leave the 5d spec and plan untouched — they are historical record.

- [ ] **Step 5: Verify no stale 8.4.53 claims remain in live docs**

Run: `grep -rn '8\.4\.53\|jaśtva' AGENTS.md README.md docs/ARCHITECTURE.md`
Expected: no hits in `README.md` or `docs/ARCHITECTURE.md`; the only `AGENTS.md` hits are inside the Step 1 text, which describes 8.4.53 as removed.

- [ ] **Step 6: Static gates**

Run: `mise run fmt && mise run fmt-check && mise run lint && mise run audit`
Expected: all PASS — no diff after `fmt`, no clippy warnings with `-D warnings`, and `cargo audit && cargo deny check` clean.

- [ ] **Step 7: Mutation gate**

Ensure the dev tooling is installed once: `MISE_ENV=dev mise install`.

Run **in a foreground shell** (the mise shim fails in background shells): `mise run mutants`
Expected: **0 missed, 0 survivors.** The regions that matter are 8.2.25's three guard arms — the `starts_with('D')` test, the `ends_with('s')` test, and the two term-index searches. Deleting 8.4.53 in Task 2 removed its mutants; `is_jhal` retains coverage through 8.4.55.

If a mutant survives, add a negative that distinguishes the mutated guard from the real one — the table in Task 4 is the model for which cell witnesses which arm. Do not weaken the gate.

- [ ] **Step 8: Final verification and commit**

Run: `mise run test && mise run fmt-check && mise run lint`
Expected: all PASS.

```bash
git add -A
git commit -m "docs: adādi √vas + 8.2.25 dhi ca correction (slice 5e complete)"
```

---

## Self-review notes

- **Spec coverage.** Rule ① (8.2.25) → Task 1. Rule ② (8.4.53 deletion) → Task 2. Root registration, tsv mirror, 28→29 count, 1008→1044 golden → Task 3. The corrected `ADve` trace pin → Task 1 Step 7; the `vaDve` pin → Task 4 Step 6. Every negative/guard pin listed in the spec's Testing section → Task 4 (under-application → Step 5; adjacency → Step 3; "must be `s`" → Step 2; "must be Dh-initial" → Step 1; wrong pada → Step 5). Mutation and static gates → Task 5 Steps 6–7. All four documentation targets, including the parent-design correction footnote → Task 5 Steps 1–4. The spec's "Verification method" section is honoured by Task 3 Step 5's instruction to write goldens from the reference table, never from engine output.
- **Risk 1 (8.2.25's scope)** is pinned from both sides by Task 4's boundary tests rather than left to review.
- **Risk 2 (deleting correct grammar)** — the restore trigger is written into Task 2's preamble and into the Task 5 Step 4 footnote, so it survives in a doc a future slice will actually read.
- **Naming consistency.** Rule id `8.2.25` and name string `"Di ca"` are identical in the rule, the trace pins, and the docs. Test names `dhi_ca_elides_s_before_dhve`, `dhi_ca_fires_for_vas_and_only_before_dh`, `dhi_ca_does_not_elide_a_non_s_before_dh`, `dhi_ca_reads_the_affixs_neighbour_not_the_anga` all share the `dhi_ca` prefix, so Task 4 Step 4's `cargo test dhi_ca` filter catches all four. Root code `vas` and artha `AcCAdane` are identical in `panini-data`, the tsv, and every test.
- **Ordering.** Task 2 must follow Task 1 — deleting 8.4.53 before 8.2.25 exists would leave `AdDve`-expecting tests failing with no replacement. Task 4 must follow Task 3, since its pins reference the registered √vas.
