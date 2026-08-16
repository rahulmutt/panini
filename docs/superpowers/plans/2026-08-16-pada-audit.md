# Pada Audit Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Re-derive every curated root's pada from its own upadeśa, fix the two rows that disagree (`01.1049 √nī`, `06.0001 √tud`), and leave behind a test that keeps the column honest.

**Architecture:** A test-private `pada_from_upadesha` applies 1.3.12 / 1.3.72 / 1.3.78 to the vendored dhātupāṭha and compares the verdict against `Dhatu::pada`, consulting nothing this repo wrote — the same non-circularity that makes `dhatupatha_numbers_resolve_upstream` worth having. The two disagreeing rows flip to `PadaAssignment::Ubhayapada`, which is all the engine needs: the tag, the two sūtras and the `padas()` fan-out all landed with √rudh. The 72 new golden forms are transcribed from the cross-implementation audit's dump, never hand-authored.

**Tech Stack:** Rust 1.97.1 via mise; `cargo test`; `cargo-mutants`; the committed audit harness at `tools/audit/` run as a `vidyut-prakriya` example.

**Spec:** `docs/superpowers/specs/2026-08-16-pada-audit-design.md`

## Global Constraints

- **No grammar changes.** No `Rule` added, removed or reordered. `TINANTA_RULES` and `tinanta_rule_order_is_pinned` are untouched. The vikalpa set stays at seven: 7.1.35, 3.4.111, 6.4.107, 8.2.74, 8.2.75, 8.4.65, 8.4.56.
- **No existing form may change.** This slice only *adds* cells. If any form outside the 72 new ones moves, stop — that is a grammar change and the slice needs re-scoping.
- **No golden form may be hand-authored.** All 72 are transcribed from the audit dump (Task 2). A hand-derived paradigm that is 70/72 right is worse than none.
- **Root count stays 49.** No root is added or removed.
- Target counts after the slice: `PARADIGM` **208** blocks / **1872** cells; `ALTERNATES` **242** rows (unchanged); **2114** forms.
- Toolchain: run scoped tests as `mise exec -- cargo test -p <crate>`. `mise run test -- -p X` does **not** scope. The full suite (`mise run test`) takes roughly 7 minutes — run it in the **foreground** with a generous timeout; never background it.
- SLP1 is the only internal representation. `#![forbid(unsafe_code)]` holds.

---

## File Structure

| File | Responsibility this slice |
|---|---|
| `crates/panini-data/src/lib.rs` | The two `pada` flips; the `Dhatu::pada` doc comment; the new `pada_from_upadesha` helper and `curated_pada_agrees_with_upadesha_markers` test, in the existing `#[cfg(test)] mod tests` beside `strip_anubandhas` |
| `crates/panini/tests/paradigm.rs` | Eight new `PARADIGM` blocks; `GATED` used then re-emptied; `derivation_set_shape_matches_the_audited_numbers` counts; delete the `tudate` INVALID entry |
| `tools/audit/panini_full_audit.rs` | Corpus-total assertions 1800→1872, 2042→2114 |
| `tools/audit/README.md` | Totals, "Last recorded result", and the "Scope" paragraph that calls this slice still-open |
| `AGENTS.md`, `docs/ARCHITECTURE.md`, `README.md` | Counts and the stale ubhayapada/deferral prose |
| `docs/superpowers/specs/2026-07-21-divadi-tudadi-ganas-design.md` | Footnote the √tud deferral as discharged |

---

### Task 1: The guard test, and the two flips it forces

The test is written first and must fail naming exactly the two rows. That ordering is the point: it means the flips are justified by something written before anyone encoded which rows would flip.

`GATED` absorbs the eight new (root, lakāra, pada) triples so the workspace stays green at the commit boundary — that constant exists for exactly this ("a future partial slice may repopulate it"). Task 2 empties it again.

**Files:**
- Modify: `crates/panini-data/src/lib.rs` — helper + test in `mod tests`; `pada` on two `Dhatu` rows; the `pada` field doc comment
- Modify: `crates/panini/tests/paradigm.rs:3739` — the `GATED` constant

**Interfaces:**
- Consumes: `is_hal(char) -> bool` and `upstream_rows() -> Vec<(&'static str, &'static str, &'static str)>`, both already in `mod tests`.
- Produces: `fn pada_from_upadesha(upadesha: &str) -> PadaAssignment`, test-private. Task 2 does not use it.

- [ ] **Step 1: Write the helper**

In `crates/panini-data/src/lib.rs`, inside `#[cfg(test)] mod tests`, directly after `strip_anubandhas`:

```rust
    /// The pada a root's own upadeśa assigns it, by 1.3.12 / 1.3.72 / 1.3.78.
    ///
    /// **Not grammar the pipeline owes a `Rule`** — the same standing as
    /// `strip_anubandhas`, for the same reason: it never runs in a
    /// derivation. It exists so `curated_pada_agrees_with_upadesha_markers`
    /// can re-derive the `pada` column from upstream without consulting
    /// anything this repo wrote about the root.
    ///
    /// The accent notation is the whole difficulty. Upstream writes an accent
    /// AFTER the `~` that marks an anunāsika it, so `~\` is an anudātta it and
    /// `~^` a svarita it — whereas a `\` sitting directly on a vowel elsewhere
    /// is the ROOT's own accent and says nothing about pada. 35 of the 49
    /// curated roots carry such an accent — `01.0642 ji\`, `01.1082 smf\` and
    /// `02.0001 a\da~` among them — so conflating the two does not fail
    /// loudly; it silently calls most of the table ātmanepada.
    fn pada_from_upadesha(upadesha: &str) -> PadaAssignment {
        // Accents attached to an it vowel, and only those.
        let anudatta_it = upadesha.contains("~\\");
        let svarita_it = upadesha.contains("~^");

        // 1.3.3 halantyam, decided on the accent-stripped upadeśa: a final hal
        // is an it. `SIN` and `vfN` reach 1.3.12 this way, `RI\Y` reaches
        // 1.3.72, and none of the three carries a `~` at all.
        let bare: String = upadesha
            .chars()
            .filter(|c| *c != '\\' && *c != '^')
            .collect();
        let final_it = bare.chars().last().filter(|c| is_hal(*c));
        let ngit = final_it == Some('N');
        // 1.3.5 ādir ñiṭuḍavaḥ supplies a ñ it as an initial `Yi` too.
        let nyit = final_it == Some('Y') || bare.starts_with("Yi");

        // ORDER IS LOAD-BEARING. 1.3.12 is tested first because `YiinDI~\`
        // (√indh) satisfies both it and 1.3.72, and must come out ātmanepada.
        // Pinned by `indh_is_atmanepada_despite_satisfying_1_3_72`.
        if anudatta_it || ngit {
            // 1.3.12 anudāttaṅita ātmanepadam.
            return PadaAssignment::Atmanepada;
        }
        if svarita_it || nyit {
            // 1.3.72 svaritañitaḥ kartrabhiprāye kriyāphale — ubhayapada,
            // since 1.3.78 supplies the parasmaipada arm.
            return PadaAssignment::Ubhayapada;
        }
        // 1.3.78 śeṣāt kartari parasmaipadam.
        PadaAssignment::Parasmaipada
    }
```

- [ ] **Step 2: Write the failing test**

Immediately after `dhatupatha_numbers_resolve_upstream` in the same module:

```rust
    #[test]
    fn curated_pada_agrees_with_upadesha_markers() {
        let rows = upstream_rows();
        let mut wrong: Vec<String> = Vec::new();
        for d in dhatus() {
            let (_, upadesha, _) = rows
                .iter()
                .find(|(n, _, _)| *n == d.dhatupatha)
                .unwrap_or_else(|| panic!("{} names no upstream row", d.dhatupatha));
            let derived = pada_from_upadesha(upadesha);
            if derived != d.pada {
                wrong.push(format!(
                    "{} {} ({upadesha}): curated {:?}, markers say {derived:?}",
                    d.dhatupatha, d.code, d.pada
                ));
            }
        }
        assert!(
            wrong.is_empty(),
            "pada column disagrees with the vendored upadeśa:\n  {}",
            wrong.join("\n  ")
        );
    }

    #[test]
    fn curated_gana_agrees_with_the_dhatupatha_number() {
        // The same class of hole as the pada column: a hand-copied verdict
        // sitting beside the data that determines it. A dhātupāṭha number's
        // first two digits ARE its gaṇa, and nothing asserted the agreement.
        // Not decorative — `05.0020` and `09.0059` share the code `aS` and are
        // distinguished only by gaṇa.
        for d in dhatus() {
            let expected = match &d.dhatupatha[..2] {
                "01" => Gana::Bhvadi,
                "02" => Gana::Adadi,
                "04" => Gana::Divadi,
                "05" => Gana::Svadi,
                "06" => Gana::Tudadi,
                "07" => Gana::Rudhadi,
                "09" => Gana::Kryadi,
                other => panic!("{} names gaṇa {other}, which no root curates", d.dhatupatha),
            };
            assert_eq!(
                d.gana, expected,
                "{} carries the wrong gaṇa for its number",
                d.dhatupatha
            );
        }
    }

    #[test]
    fn indh_is_atmanepada_despite_satisfying_1_3_72() {
        // `YiinDI~\` carries a ñi that 1.3.72 reads AND an anudātta it that
        // 1.3.12 reads. 1.3.12 wins: vidyut-prakriya derives √indh in
        // ātmanepada only, checked in the ubhayapada slice against √rudh as a
        // `~^r` control. Reversing the two clauses in `pada_from_upadesha`
        // grows √indh a parasmaipada column it must not have.
        //
        // This is the second, independent encoding of the precedence that
        // `Tag::Ubhayapadin`'s doc comment in `panini-prakriya` states. It is
        // asserted here so a reversal fails rather than quietly re-deriving
        // that tag's own opinion.
        assert_eq!(pada_from_upadesha("YiinDI~\\"), PadaAssignment::Atmanepada);
    }

    #[test]
    fn a_final_hal_it_assigns_pada_without_any_tilde() {
        // 1.3.3 halantyam is the only marker these three have — no `~`
        // anywhere — so a check that looked only for `~\` / `~^` would call
        // all three parasmaipada and still agree with the column on two.
        assert_eq!(pada_from_upadesha("SIN"), PadaAssignment::Atmanepada); // 02.0026 √śī
        assert_eq!(pada_from_upadesha("vfN"), PadaAssignment::Atmanepada); // 09.0045 √vṛṅ
        assert_eq!(pada_from_upadesha("RI\\Y"), PadaAssignment::Ubhayapada); // 01.1049 √nī
    }

    #[test]
    fn a_root_vowel_accent_does_not_assign_pada() {
        // The failure mode that would make the whole audit vacuous: 35 of the
        // 49 curated roots carry a `\` on a root vowel, and reading it as
        // 1.3.12's anudātta calls 35 of them ātmanepada. Agreement with the
        // column would still hold on every genuinely ātmanepada root, so only
        // a parasmaipada witness catches it.
        assert_eq!(pada_from_upadesha("ji\\"), PadaAssignment::Parasmaipada); // 01.0642
        assert_eq!(pada_from_upadesha("a\\da~"), PadaAssignment::Parasmaipada); // 02.0001
        assert_eq!(pada_from_upadesha("Ba\\njo~"), PadaAssignment::Parasmaipada); // 07.0016
        // And the converse: the accent that DOES assign, on an it vowel.
        assert_eq!(pada_from_upadesha("Ki\\da~\\"), PadaAssignment::Atmanepada); // 07.0012
        assert_eq!(pada_from_upadesha("tu\\da~^"), PadaAssignment::Ubhayapada); // 06.0001
    }
```

- [ ] **Step 3: Run the test and confirm it fails on exactly two rows**

```bash
mise exec -- cargo test -p panini-data curated_pada_agrees 2>&1 | tail -20
```

Expected: FAIL. The panic message must name **exactly** these two lines and no others:

```
01.1049 nI (RI\Y): curated Parasmaipada, markers say Ubhayapada
06.0001 tud (tu\da~^): curated Parasmaipada, markers say Ubhayapada
```

The other three tests must PASS. If the failure list contains anything else, the helper is wrong — do not proceed by flipping whatever it names.

- [ ] **Step 4: Flip the two rows**

In `crates/panini-data/src/lib.rs`, in `static DHATUS`, on the `01.1049` entry (`code: "nI"`) and the `06.0001` entry (`code: "tud"`), change:

```rust
        pada: PadaAssignment::Parasmaipada,
```

to:

```rust
        pada: PadaAssignment::Ubhayapada,
```

Both entries also carry an explanatory comment. Add one to each, above the row:

```rust
    // 01.1049 `RI\Y`: the final `Y` is an it by 1.3.3 halantyam, so 1.3.72
    // svaritañitaḥ sanctions both padas (nayati / nayate). Curated
    // parasmaipada from the v1 slice until the pada audit; no deferral list
    // ever named it.
```

```rust
    // 06.0001 `tu\da~^`: the `~^` is a svarita it, so 1.3.72 sanctions both
    // padas (tudati / tudate). Deferred behind 1.3.72 by the divādi/tudādi
    // slice, then behind curation once 1.3.72 landed; discharged by the pada
    // audit.
```

- [ ] **Step 5: Rewrite the `Dhatu::pada` doc comment**

Replace the whole existing comment on the `pada` field (the paragraph beginning "Which pada(s) this engine derives for this root — a curated verdict") with:

```rust
    /// Which pada(s) this engine derives for this root. Curated rather than
    /// read from the upadeśa's it-markers — but no longer a *deferral*:
    /// `curated_pada_agrees_with_upadesha_markers` re-derives every one of
    /// these 49 verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 /
    /// 1.3.78 and requires it to match, the same way
    /// `dhatupatha_numbers_resolve_upstream` holds `code` to upstream.
    ///
    /// The column stayed hand-written because deriving it in production means
    /// running it-stripping in production, and upadeśa preprocessing is not
    /// the tiṅanta pipeline `TINANTA_RULES` models — it needs its own pipeline
    /// concept. Until it has one, a curated column plus a non-circular test is
    /// the honest arrangement; see the deferral in
    /// `docs/superpowers/specs/2026-08-16-pada-audit-design.md`.
    ///
    /// The test covers the 49 roots curated here, not the dhātupāṭha's 2259.
    /// It catches a mis-assigned pada on a root a future slice adds; it does
    /// not make the table self-maintaining.
    pub pada: PadaAssignment,
```

- [ ] **Step 6: Confirm `panini-data` is green**

```bash
mise exec -- cargo test -p panini-data 2>&1 | tail -20
```

Expected: PASS, 18 tests (13 before, 5 added).

- [ ] **Step 7: Gate the eight new triples so the workspace stays green**

Flipping the column makes `paradigm_covers_every_enumerable_cell` fail — the two roots now enumerate ātmanepada cells with no `PARADIGM` block. That is the guard working. Populate `GATED` at `crates/panini/tests/paradigm.rs:3739`:

```rust
    // Repopulated by the pada audit, and emptied again in the same slice once
    // the audited forms land. √nī and √tud became ubhayapadī when the audit
    // corrected their column; their ātmanepada blocks are transcribed from the
    // cross-implementation audit's dump, which runs against the corrected data
    // — so the data flip necessarily lands one commit ahead of its goldens.
    const GATED: &[(&str, &str, Pada)] = &[
        ("01.1049", "laT", Pada::Atmanepada),
        ("01.1049", "laN", Pada::Atmanepada),
        ("01.1049", "loT", Pada::Atmanepada),
        ("01.1049", "viDiliN", Pada::Atmanepada),
        ("06.0001", "laT", Pada::Atmanepada),
        ("06.0001", "laN", Pada::Atmanepada),
        ("06.0001", "loT", Pada::Atmanepada),
        ("06.0001", "viDiliN", Pada::Atmanepada),
    ];
```

The lakāra labels must match `panini::lakara_name`'s output exactly; `laT` / `laN` / `loT` / `viDiliN` are the strings `PARADIGM` already uses.

- [ ] **Step 8: Delete the `tudate` INVALID entry**

`GATED` only silences `paradigm_covers_every_enumerable_cell`; it does not stop the analyzer enumerating the cell. So the moment √tud's column flips, the engine derives `tudate` and `known_nonforms_are_invalid` fails — the deletion belongs in *this* commit, not with the goldens.

In `known_nonforms_are_invalid`, remove the `"tudate",` entry together with the nine-line comment above it beginning "`tudate` is a REAL Sanskrit form". That comment already anticipates this slice ("Auditing the whole table for mis-assigned pada is its own slice; until then this entry pins the documented meaning of INVALID … not a claim about Sanskrit").

Leave the neighbouring `manyati` and `vidyati` entries alone — those are wrong-pada crosses on genuinely ātmanepada-only roots and stay wrong. There is no equivalent entry for √nī to remove; `nayate` was never pinned INVALID.

- [ ] **Step 9: Run the full suite**

```bash
mise run test 2>&1 | tail -30
```

Expected: PASS. Run in the foreground; it takes roughly 7 minutes.

- [ ] **Step 10: Commit**

```bash
git add crates/panini-data/src/lib.rs crates/panini/tests/paradigm.rs
git commit -m "fix(data): the pada column disagreed with the upadesa on two roots

curated_pada_agrees_with_upadesha_markers re-derives all 49 verdicts from the
vendored upadesa via 1.3.12 / 1.3.72 / 1.3.78 and consults nothing this repo
wrote. Two rows disagree: 06.0001 tu\\da~^ (Vtud), which four deferrals across
two slices named, and 01.1049 RI\\Y (Vni), which none did -- ubhayapadi by a
n-it that 1.3.3 halantyam identifies, curated parasmaipada since the v1 slice.

The accent notation is what makes this worth a test rather than an act: an
accent after the ~ marks an it vowel, one on a root vowel does not, and 35 of
the 49 roots carry the latter. Reading them alike calls most of the table
atmanepada while still agreeing with the column on every atmanepadin root, so
a_root_vowel_accent_does_not_assign_pada carries parasmaipada witnesses.

Clause order is load-bearing and separately pinned: YiinDI~\\ (Vindh) satisfies
both 1.3.12 and 1.3.72 and must come out atmanepada.

The eight new (root, lakara, pada) triples are GATED for one commit; their
goldens are transcribed from the audit, which has to run against the corrected
data to produce them. tudate stops being INVALID here rather than with the
goldens: GATED silences the coverage guard, not the analyzer, so the form goes
live the moment the column flips.

curated_gana_agrees_with_the_dhatupatha_number closes the same hole one column
over -- a number's first two digits are its gana, and nothing checked it."
```

---

### Task 2: Audit the two roots, transcribe their goldens, ungate

**Files:**
- Modify: `tools/audit/panini_full_audit.rs:577-579` — corpus totals
- Modify: `crates/panini/tests/paradigm.rs` — eight `PARADIGM` blocks; empty `GATED`; `derivation_set_shape_matches_the_audited_numbers`; delete the `tudate` INVALID entry

**Interfaces:**
- Consumes: the corrected `Dhatu::pada` column from Task 1.
- Produces: eight `PARADIGM` blocks of shape `(&str, &str, Pada, [&str; 9])`, cell order `[P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B]`.

- [ ] **Step 1: Update the harness's corpus totals**

The harness asserts the old totals and would panic before reporting. In `tools/audit/panini_full_audit.rs`:

```rust
    assert_eq!(roots_seen.len(), 49, "curated roots");
    assert_eq!(n_cells, 1872, "cells: 208 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 2114, "forms: 1872 cells + 242 ALTERNATES rows");
```

Update the module doc comment at lines 12, 24 and 54 to match (`49 curated roots` stays; `1800`→`1872`, `2042`→`2114`).

`n_forms` is the tripwire for this slice's one prediction — that none of the 72 new cells forks. If it comes out above 2114, a new cell has an alternate; stop and report which, rather than adjusting the constant to fit.

- [ ] **Step 2: Set up the vidyut checkout**

```bash
head -20 /workspace/data/dhatupatha.tsv | grep -i commit
```

Clone and check out that commit, then follow `tools/audit/README.md`:

```bash
cd /tmp && git clone --filter=blob:none https://github.com/ambuda-org/vidyut vidyut-full
cd /tmp/vidyut-full && git checkout <the commit from that header>
```

Add to `/tmp/vidyut-full/vidyut-prakriya/Cargo.toml` under `[dev-dependencies]`:

```toml
panini = { path = "/workspace/crates/panini" }
panini-data = { path = "/workspace/crates/panini-data" }
```

```bash
cp /workspace/tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
```

- [ ] **Step 3: Run both negative controls first**

A zero-difference result means nothing until the harness is shown able to detect a difference.

```bash
cd /tmp/vidyut-full/vidyut-prakriya
PANINI_AUDIT_PERTURB=form mise exec rust@1.97.1 -- cargo run --release --example panini_full_audit 2>&1 | tail -20
PANINI_AUDIT_PERTURB=entry mise exec rust@1.97.1 -- cargo run --release --example panini_full_audit 2>&1 | tail -20
```

Expected: both exit 1 with real form-vs-form differences. `entry` must flag all 36 of √bhū's cells with `Bavati` vs `paWati`. If either passes, stop — the harness is not measuring anything.

- [ ] **Step 4: Run the honest audit and dump the table**

```bash
cd /tmp/vidyut-full/vidyut-prakriya
PANINI_AUDIT_DUMP=/tmp/audit-table.tsv \
  mise exec rust@1.97.1 -- cargo run --release --example panini_full_audit 2>&1 | tail -25
```

Expected: `AUDIT PASSED: 1872 cells, 2114 forms, zero differences.`

If there are differences, they will be on `01.1049` or `06.0001` ātmanepada cells, and they mean the slice has grown a grammar change. Stop and report the differing cells rather than pinning this engine's output.

- [ ] **Step 5: Extract the 72 forms**

```bash
awk -F'\t' '$1=="01.1049" || $1=="06.0001"' /tmp/audit-table.tsv \
  | awk -F'\t' '$3=="atmanepada"' | cut -f1,4,5,6
```

The dump's columns are `number code pada lakara cell ours theirs same`. Confirm before transcribing:

```bash
awk -F'\t' '($1=="01.1049"||$1=="06.0001") && $3=="atmanepada" && $6 ~ / /' /tmp/audit-table.tsv | wc -l
```

Expected: `0` — no cell yields more than one form. A non-zero count is the fork prediction failing; stop and report.

- [ ] **Step 6: Add the eight `PARADIGM` blocks**

Transcribe the `ours` column. Place each block next to the root's existing parasmaipada block for that lakāra, matching how `07.0001`'s two padas sit. Shape:

```rust
    (
        "01.1049",
        "laT",
        Pada::Atmanepada,
        [
            /* nine forms, transcribed from the dump, cell order
               P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B */
        ],
    ),
```

Do not type any form from memory or derive one by hand — including ones that look obvious. Every one of the 72 comes from `/tmp/audit-table.tsv`.

- [ ] **Step 7: Empty `GATED` again**

Restore it to the empty constant, with its original comment plus this slice's line:

```rust
    // adādi × vidhiliṅ was gated in slice 5a and ungated in slice 5b; √śī was
    // gated in slice 5f task 1 and ungated there; √nī and √tud's ātmanepada
    // blocks were gated for one commit by the pada audit, between the column
    // being corrected and the audited goldens landing. There are no gated
    // cells any more. This constant stays (empty) so the two assertions below
    // keep documenting that EVERY enumerable (root, lakara, pada) triple must
    // be pinned in PARADIGM — a future partial slice may repopulate it, but it
    // must never silently hide a missing golden block.
    const GATED: &[(&str, &str, Pada)] = &[];
```

- [ ] **Step 8: Update the shape test's counts**

In `derivation_set_shape_matches_the_audited_numbers`:

```rust
    assert_eq!(total_cells, 1872, "208 root×lakāra blocks × 9 cells each");
```

and

```rust
    assert_eq!(ones, 1702, "one-form cells");
```

`twos` (109), `threes` (56), `fours` (1), `fives` (2) and `sixes` (2) are unchanged — 1702 + 109 + 56 + 1 + 2 + 2 = 1872.

- [ ] **Step 9: Correct the shape test's doc comment**

Two claims in it are now wrong. Change `1800 cells total (200 root×lakāra blocks × 9)` to `1872 cells total (208 root×lakāra blocks × 9)` and `of which 1630 hold exactly one form` to `of which 1702 hold exactly one form`.

The last sentence is separately false and predates this slice — it says the audit probe's "source is deliberately not committed to this repo (it is throwaway verification tooling, not shipped code)". The harness was committed to `tools/audit/` in `0ff84a7`. Replace that clause with: "the probe's source is committed at `tools/audit/panini_full_audit.rs` as of `0ff84a7`, and this slice re-ran it over all 1872 cells — so the numbers are re-verified as well as pinned".

- [ ] **Step 10: Run the full suite**

```bash
mise run test 2>&1 | tail -30
```

Expected: PASS. Foreground, ~7 minutes.

- [ ] **Step 11: Commit**

```bash
git add crates/panini/tests/paradigm.rs tools/audit/panini_full_audit.rs
git commit -m "test(paradigm): Vni and Vtud get their atmanepada paradigms

72 cells transcribed from the cross-implementation audit's dump at vidyut
<commit>, both negative controls verified failing first. Nothing was derived by
hand: a hand-written paradigm that is 70/72 correct is worse than none, because
the two wrong cells arrive wearing the same authority as the rest.

Zero differences across 1872 cells / 2114 forms / 49 roots. No existing form
moved, and no cell forked -- the 21 atmanepada alternates in the suite are
still all rudhadi on 8.4.65, so ALTERNATES stays at 242 rows and the one-form
count absorbs all 72. tudate, un-pinned as INVALID in the previous commit, is
now 06.0001 laT atmanepada prathama eka.

The shape test's doc comment also loses a claim that predates this slice: the
audit probe's source is no longer uncommitted throwaway tooling, it is
tools/audit/panini_full_audit.rs as of 0ff84a7."
```

---

### Task 3: Documentation

Every claim here is one this slice falsifies. The failure mode the repo has hit twice (`c4b3907`, and the ubhayapada slice's stated top risk) is a sentence that stays *nearly* true and so survives review.

**Files:**
- Modify: `AGENTS.md`, `docs/ARCHITECTURE.md`, `README.md`, `tools/audit/README.md`, `docs/superpowers/specs/2026-07-21-divadi-tudadi-ganas-design.md`

**Interfaces:** none — documentation only.

- [ ] **Step 1: `AGENTS.md` counts**

Line ~99: `1800 cells` → `1872 cells`. Line ~106: `242 rows in all, so 1800 + 242 = 2042 forms total` → `242 rows in all, so 1872 + 242 = 2114 forms total`. Line ~280: `now stands at 1800 cells and 2042 forms` → `1872 cells and 2114 forms`.

Leave the mutation-timing paragraphs (lines ~22, ~46, ~85) alone — those cite measurements taken *at* 1800 and 1620 cells and are historical facts that stay true. Task 4 appends to them rather than editing them.

- [ ] **Step 2: `AGENTS.md` — the ubhayapada prose**

The paragraph describing √rudh as "the engine's first ubhayapadī root, deriving a full paradigm in each pada" stays true as *first*. Add after it:

```
    The pada audit added two more: `01.1049 RI\Y` (√nī, bhvādi) and
    `06.0001 tu\da~^` (√tud, tudādi), both ubhayapadī by 1.3.72 and both
    curated parasmaipada until then. √tud was a known deferral; √nī was
    named by no deferral list and was read past by every slice from v1 on.
    `curated_pada_agrees_with_upadesha_markers` in `panini-data` now
    re-derives all 49 verdicts from the vendored upadeśa, so the column
    cannot drift from the data that determines it.
```

- [ ] **Step 3: `docs/ARCHITECTURE.md`**

Less than it first appears. The pada-coordinate paragraph ("an ubhayapadī root contributes *two* `PARADIGM` blocks per lakāra … 1.3.72 is deliberately absent from the vikalpa set") is exactly right and describes what these two roots now do — **do not touch it**. The file carries no cell or form totals.

One sentence needs re-reading: the rudhādi paragraph's "√rudh (`07.0001`), the gaṇa's own eponym, which arrived with 1.3.72 *svaritañitaḥ* in the ubhayapada slice as the engine's first ubhayapadī root". True as written, but it is the file's only ubhayapada sentence, so it now carries the implication of *only*. Change `as the engine's first ubhayapadī root` to `as the engine's first ubhayapadī root — the pada audit later added √nī and √tud, outside this gaṇa`.

- [ ] **Step 4: `README.md`**

Line ~36: `170 of the 1800 cells hold more than one form: 109` → `170 of the 1872 cells hold more than one form: 109`. The 170 is unchanged — no new cell forks.

- [ ] **Step 5: `tools/audit/README.md`**

Three edits:

- Line ~30: `(49 roots, 1800 cells, 2042 forms)` → `(49 roots, 1872 cells, 2114 forms)`.
- The "Last recorded result" section: replace with this slice's run — date, "pada audit" slice, the vidyut commit, `zero differences across 1872 cells / 2114 forms / 49 roots`, both negative controls verified failing.
- The "Scope" section is now wrong in both sentences. It reads: "The harness tells both engines which pada to derive; it does not audit whether a root's `PadaAssignment` is itself correct. Auditing the 49 roots for mis-assigned pada is a separate, still-open slice — √tud (`06.0001 tu\da~^`) is the known case." Replace the second half with: "Auditing the column itself is `curated_pada_agrees_with_upadesha_markers` in `panini-data`, which re-derives every verdict from the vendored upadeśa and runs in `cargo test` — the two audits are complementary, and the pada audit slice ran both. This harness stays the authority on derived **forms**."

- [ ] **Step 6: Footnote the divādi/tudādi spec**

`docs/superpowers/specs/2026-07-21-divadi-tudadi-ganas-design.md` line ~109 reads: "One fidelity note: **√tud is ubhayapadī** in the Dhātupāṭha (svarita it). This slice takes only its **parasmaipada** pada (tudati) … the ātmanepada tudati-form is future work." Append a footnote directly below it, in the style slice 5f used for the roadmap's √śī row:

```markdown
> **Discharged 2026-08-16** by the pada audit
> (`docs/superpowers/specs/2026-08-16-pada-audit-design.md`). √tud is
> `PadaAssignment::Ubhayapada` and both padas are pinned in `PARADIGM`. The
> same audit found that `01.1049 √nī`, curated in the v1 slice, is ubhayapadī
> on the same sūtra and had been missed here and everywhere else.
```

Do not edit `docs/superpowers/plans/*` — plans are historical records.

- [ ] **Step 7: Verify no stale count survives**

```bash
grep -rn "1800\|2042\|1630" AGENTS.md README.md docs/ARCHITECTURE.md tools/ crates/panini/tests/paradigm.rs
```

Every remaining hit must be a deliberate historical reference to a measurement taken at 1800 cells (the mutation-timing paragraphs in `AGENTS.md`, and the `1800`-era comments in `controller.rs` and `guna.rs`, which describe past reasoning). Anything else is a miss.

- [ ] **Step 8: Lint, format, and commit**

```bash
mise run fmt && mise run lint 2>&1 | tail -20
```

```bash
git add -A
git commit -m "docs: the pada column is checked, and Vrudh is no longer the only one

Counts move to 1872 cells / 2114 forms across AGENTS.md, README.md and the
audit harness's README. ARCHITECTURE.md needed almost nothing -- its
pada-coordinate paragraph already described what an ubhayapadi root does -- but
its single ubhayapada sentence named Vrudh as the first and so read as the only.

tools/audit/README.md's Scope section claimed the pada audit was still open and
named Vtud as the known case. Both halves are now false, and the two audits are
complementary rather than one substituting for the other: the harness is the
authority on derived forms, the panini-data test on the column.

The divadi/tudadi spec's fidelity note is footnoted as discharged."
```

---

### Task 4: Mutation gate

The golden suite grew, so the uncaught-run floor moved. AGENTS.md's rule is to **measure** the floor, not scale it by cell count — the ubhayapada slice recorded a measured ~450s uncontended floor at 1800 cells where a 4%-growth scaling predicted ~395s.

**Files:**
- Modify: `AGENTS.md` — append this slice's measurements to the mutation-timing paragraph

**Interfaces:** none.

- [ ] **Step 1: Measure the uncontended floor at 1872 cells**

```bash
time mise run test 2>&1 | tail -5
```

One suite, no mutation campaign alongside it. Record paradigm, roundtrip and trace timings separately. This is the number the cap must clear, not the caught-and-aborted figure.

- [ ] **Step 2: Run the campaign**

```bash
cd /workspace && mise exec -- cargo mutants --package panini-prakriya \
  --test-workspace=true -j 4 --timeout 2400 2>&1 | tail -30
```

`--timeout 2400` explicitly, following the ubhayapada slice's own campaign rather than `mise.toml`'s 1200 default, which is documented as adequate-but-unverified at `-j 4`. Run in the **foreground** with a long timeout; a backgrounded campaign gets orphaned. Note that `cargo mutants` also reads `-j` from `CARGO_MUTANTS_JOBS`, so confirm the environment is not overriding it.

- [ ] **Step 3: Read both output files, not just one**

```bash
cat mutants.out/missed.txt; echo "--- timeouts ---"; cat mutants.out/timeout.txt
```

`missed.txt` must be empty. `timeout.txt` is expected to contain **exactly one** entry: the `tripadi.rs` ṇatva backward scan, where mutating `j -= 1` to `j /= 1` makes the loop never terminate. That one is a genuine, permanent timeout and the cap *is* its detection mechanism — do not chase it with a bigger `--timeout` or a code change.

Any other timeout entry must be re-run alone before concluding anything; a real survivor reclassified as a timeout is exactly how a "0 missed" run becomes vacuous.

- [ ] **Step 4: Handle survivors**

If `missed.txt` is non-empty, each survivor is either a missing test or dead code. The ubhayapada slice's precedent is worth following: three `Context::is_tip` survivors turned out to be redundant plumbing and were deleted rather than tested. Do not add a test that merely pins a mutant without asking whether the code it covers earns its place.

- [ ] **Step 5: Record the measurements in `AGENTS.md`**

Append to the mutation-timing paragraph, after the sentence ending "pass `--timeout 2400` explicitly until someone actually measures a `-j 4` run against the 1800-cell suite":

```
    **The pada audit measured both at 1872 cells.** Uncontended floor:
    paradigm ~Ns, roundtrip ~Ns, trace ~Ns (uncaught total ~Ns). Campaign at
    `-j 4 --timeout 2400`: N mutants, N caught, 0 missed, N unviable, and the
    one known-permanent `tripadi.rs` timeout. [State whether 2400 kept real
    margin over the measured floor, and whether 1200 would still have.]
```

Fill in the real numbers; the bracketed sentence is a judgement to write, not a template to leave.

- [ ] **Step 6: Commit**

```bash
git add AGENTS.md
git commit -m "test: mutation gate at 1872 cells, with the floor re-measured

Measured rather than scaled, per AGENTS.md's own rule: the last slice found a
~450s floor where a 4%-growth scaling predicted ~395s, so cell-count arithmetic
is not a substitute. Ran at -j 4 --timeout 2400.

0 missed. timeout.txt holds only the known-permanent tripadi.rs natva-scan
mutant, where j /= 1 makes the loop never terminate and the cap is the
detection mechanism rather than a symptom of it being too short."
```

---

### Task 5: Branch finish

- [ ] **Step 1: Confirm the whole suite and the lints are green**

```bash
mise run test 2>&1 | tail -10 && mise run lint 2>&1 | tail -10 && mise run fmt-check && mise run audit 2>&1 | tail -10
```

All four must pass. `mise run audit` (cargo-audit + cargo-deny) is expected to pass including advisories.

- [ ] **Step 2: Re-read the spec against the branch**

```bash
git diff main --stat
```

Check each spec section has a corresponding change, and that nothing outside the spec's scope was touched — in particular that `TINANTA_RULES`, `tinanta_rule_order_is_pinned` and `exactly_the_pinned_vikalpa_rules_are_optional` are untouched.

- [ ] **Step 3: Push and open the PR**

```bash
git push -u origin pada-audit-49-roots
gh pr create --fill
```

- [ ] **Step 4: Finish the branch**

Use the `superpowers:finishing-a-development-branch` skill.

---

## Deferred, and why

Recorded here so the next slice does not have to re-derive it:

- **It-stripping as real sūtras** (1.3.2, 1.3.3, 1.3.5, 1.3.9), so `code` and now `pada` are *derived* from a stored upadeśa rather than curated. This slice adds a second test-private consumer of the same marker logic — `pada_from_upadesha` beside `strip_anubandhas` — which strengthens the case without pre-empting the design. Upadeśa preprocessing is not the tiṅanta pipeline `TINANTA_RULES` models, so it still needs its own pipeline concept.
- **The eight remaining `~^r` rudhādi roots** — √bhid, √chid, √ric, √vic, √kṣud, √yuj, √chṛd, √tṛd — and 8.2.30 *coḥ kuḥ*'s generalisation past the hardcoded `j` → `g`. Unchanged by this slice.
- **√bhuj** (`07.0017`), whose 1.3.66 *bhujo'navane* forks its pada on sense rather than on an axis this engine models. Unchanged.
- **The 2210 uncurated roots.** The new test covers the 49 curated ones. A root curated by a future slice can still arrive with a mis-assigned pada — the test catches it *at that point*, which is its value, but nothing here makes the table self-maintaining.
