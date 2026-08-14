# rudhādi gaṇa, slice 7b — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Land √bhañj, √piṣ and √indh, bringing rudhādi (gaṇa 7) to six roots by adding the tripādī consonant sandhi a consonant-final śnam stem walks through.

**Architecture:** No new structure. 7a built śnam's infix representation (the root split across `ANGA` and `SHAP` by 3.1.78); every rule here is an ordinary tripādī edit on the word that representation produces. Four new sūtras — 8.2.30 *coḥ kuḥ* (√bhañj), 8.4.41 *ṣṭunā ṣṭuḥ* and 8.2.41 *ṣaḍhoḥ kaḥ si* (√piṣ), 8.2.40 *jhaṣas tathor dho'dhaḥ* (√indh) — plus three guard widenings (8.2.39, 8.3.59, 8.4.53) and two `sound.rs` helper changes. No new optional rule; the vikalpa set stays at seven.

**Tech Stack:** Rust (workspace pinned to 1.97.1 via `mise`), no new dependencies.

**Spec:** `docs/superpowers/specs/2026-08-13-rudhadi-gana-7b-design.md` (commit `d8784f3`).

## Global Constraints

- Toolchain is pinned via `mise`. Never install Rust globally. Build/test with `mise exec -- cargo …` or the `mise run` tasks.
- `#![forbid(unsafe_code)]` holds in every crate touched here.
- SLP1 is the only internal representation. No transliteration outside `panini-lipi`.
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, **never** as a branch inside `derive`. The only gaṇa-conditioned logic in `derive` is aṅga tagging.
- Every new rule id must be added to `tinanta_rule_order_is_pinned` in `crates/panini-prakriya/src/tinanta/derivation_tests.rs`, in its pipeline position. **No rule in this slice is optional**, so `exactly_the_pinned_vikalpa_rules_are_optional` must remain unchanged at seven ids — if you find yourself editing it, you have made a rule `vikalpa: true` by mistake.
- **Guards are written narrow, to the reachable slice**, per the discipline that landed 8.3.59 and 8.2.25. A mutation-test survivor means an arm has no witness: shrink the guard, do not grow the test.
- Sūtra ids and names must match `vidyut-prakriya`'s `data/sutrapatha.tsv`. The exact strings this slice needs are given verbatim in each task.
- **Rule bodies are specified, not pre-written.** Plan code blocks in this repo get transcribed verbatim into the implementation, and that has previously shipped scaffolding text and pre-emptive plumbing to `main`. So each rule task below gives a **Rule specification** — id, name, kind, placement, guard conditions, operation, firing witnesses and declining witnesses — and you write the Rust against it, in the idiom of the rules already in that file.
- **What IS given verbatim:** derivation tests, `sound.rs` unit tests, and all golden data (`PARADIGM`, `ALTERNATES`). Their exact content is the requirement, so they are written out in full and should be transcribed as given.
- **What is enumerated rather than written:** per-rule guard tests. Each builds a `Prakriya` by hand, and the exact construction differs per stage file (`terms` layout, which `Tag`s the aṅga needs, whether `ctx` matters). Every such step below names the test and enumerates the exact arms it must assert; write them in the idiom of the guard tests already in the file you are editing — `anga.rs` and `tripadi.rs` each have working examples to copy the construction from.

**Scoped test command:** `mise exec -- cargo test -p panini-prakriya`
(`mise run test -- -p panini-prakriya` does **not** scope — it runs the whole workspace.)

**Full gate:** `mise run test`

### The workspace gate is expected RED from Task 1 to Task 10

`paradigm_covers_every_enumerable_cell` in `crates/panini/tests/paradigm.rs` asserts one `PARADIGM` block per `dhatus()` entry per lakāra, and `derivation_set_shape_matches_the_audited_numbers` pins the cell and alternate counts. **Task 1 adds three roots and therefore breaks both immediately.** They stay broken until Task 10 lands the goldens.

This is expected and is not a signal to stop. During Tasks 2–9:

- The working gate is the **scoped** command, `mise exec -- cargo test -p panini-prakriya`, which must be **green at the end of every task**.
- Each rule task carries its own derivation tests in `panini-prakriya`, which is where the real verification happens task-by-task.
- Do **not** "fix" the red `crates/panini` tests by adding partial `PARADIGM` blocks early. Task 10 lands them complete, from data already verified against vidyut-prakriya.

**Test-helper imports:** the stage files' `mod tests` blocks reach the shared helpers (`sole`, `declined`, `form_g`, `form_g_forked`) through `mod.rs`'s re-export — `anga.rs` and `tripadi.rs` already import them that way. Copy whichever import line the file you are editing already uses; do not invent a new path.

**Helper signatures** you will use (all in `crates/panini-prakriya/src/tinanta/derivation_tests.rs`, `pub(super)`):

```rust
fn form_g(code: &str, la: Lakara, pu: Purusha, va: Vacana) -> String
fn form_g_forked(code: &str, la: Lakara, pu: Purusha, va: Vacana, branches: usize) -> String
```

`form_g` asserts the cell did **not** fork; `form_g_forked` asserts it forked into exactly `branches` and returns branch 0 (the declined derivation). Both look the root up by `Dhatu::id` and use the root's own recorded pada.

---

### Task 1: Data layer — the three 7b roots

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (the `DHATUS` table, and the two rudhādi tests in the in-file `mod tests`)
- Modify: `data/dhatupatha.tsv` (reference mirror, not parsed by any code)

**Interfaces:**
- Consumes: `Gana::Rudhadi` and `Tag::Rudhadi`, both landed in 7a.
- Produces: three `Dhatu` rows with ids `"Banj"`, `"piz"`, `"inD"`. Every later task derives through them via `form_g("Banj", …)` and friends.

- [ ] **Step 1: Write the failing test**

In `crates/panini-data/src/lib.rs`, **rename** the existing `rudhadi_holds_exactly_the_slice_7a_roots` to `rudhadi_holds_exactly_the_slice_7b_roots` and replace its body with:

```rust
    #[test]
    fn rudhadi_holds_exactly_the_slice_7b_roots() {
        // Six roots, in table order. √hiṃs is stored `hins`, NOT `his`:
        // see its row comment. The gaṇa is still PARTIAL — nine of its 25
        // dhātupāṭha roots are ubhayapadī (`~^r`) and 1.3.72 is deferred,
        // so √rudh, the eponym, is absent. More roots would not change
        // that; only 1.3.72 will.
        let rows: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.gana == Gana::Rudhadi)
            .map(|d| (d.id, d.code, d.pada))
            .collect();
        assert_eq!(
            rows,
            vec![
                ("kft", "kft", Pada::Parasmaipada),
                ("his", "hins", Pada::Parasmaipada),
                ("Kid", "Kid", Pada::Atmanepada),
                ("Banj", "Banj", Pada::Parasmaipada),
                ("piz", "piz", Pada::Parasmaipada),
                ("inD", "inD", Pada::Atmanepada),
            ]
        );
    }
```

Then **rename** `slice_7a_ids_do_not_collide` to `slice_7b_ids_do_not_collide` and extend it so it asserts that each of the six rudhādi ids equals its own `code` and appears exactly once across the whole `DHATUS` table:

```rust
    #[test]
    fn slice_7b_ids_do_not_collide() {
        // rudhādi also holds `vi\da~\` and `o~vijI~`, which WOULD collide
        // with divādi's `vid` and tudādi's `vij`. Neither is in 7b — the
        // slice stops at six roots — so every rudhādi id is still its own
        // unqualified SLP1 code and the `aS.5` qualification mechanism
        // stays at exactly one user. 7a's spec predicted this would not
        // survive 7b; it does, because that prediction assumed a root set
        // including √vid.
        for d in dhatus().iter().filter(|d| d.gana == Gana::Rudhadi) {
            let n = dhatus().iter().filter(|o| o.id == d.id).count();
            assert_eq!(n, 1, "rudhādi id {} is not unique in DHATUS", d.id);
        }
        let qualified: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.id.contains('.'))
            .map(|d| d.id)
            .collect();
        assert_eq!(qualified, vec!["aS.5"]);
    }
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `mise exec -- cargo test -p panini-data`
Expected: FAIL — `rudhadi_holds_exactly_the_slice_7b_roots` reports three rows where six are expected.

- [ ] **Step 3: Add the three rows**

Append three `Dhatu` rows to `DHATUS` **after the `Kid` row**, in the order `Banj`, `piz`, `inD`. Follow the shape of the existing rudhādi rows exactly (`id`, `code`, `gana`, `pada`, `artha`), each with a leading comment giving the dhātupāṭha number, its aupadeśika form and gloss, and what the root is here to witness:

| id | code | dhātupāṭha | pada | artha | witnesses |
| --- | --- | --- | --- | --- | --- |
| `Banj` | `Banj` | 07.0016 `Ba\njo~` *āmardane* | `Pada::Parasmaipada` | `"Amardane"` | 8.2.30 *coḥ kuḥ* |
| `piz` | `piz` | 07.0015 `pi\zx~` *sañcūrṇane hiṁsāyāṁ ca* | `Pada::Parasmaipada` | `"saYcUrRane hiMsAyAM ca"` | 8.4.41, 8.2.41 |
| `inD` | `inD` | 07.0011 `YiinDI~\` *dīptau* | `Pada::Atmanepada` | `"dIptO"` | 8.2.40 *jhaṣas tathor dho'dhaḥ* |

The `inD` row's comment must record the pada finding, because it is the one thing here a reader would otherwise get wrong: `YiinDI~\` carries a **ñi** it-marker, and 1.3.72 *svaritañitaḥ* reads ñit as well as svarita, so the root looks ubhayapadī. It is not. Its anudātta `~\` fixes the pada by 1.3.12 *anudāttaṅita ātmanepadam*, and vidyut-prakriya derives it ātmanepada-only — checked against a `~^r` control (√rudh) that does derive both padas.

Unlike `his`/`hins` and `stiG`, none of these three is stored post-any-rule: each `code` is the plain it-stripped SLP1 stem, so no simplification comment is needed on `Banj` or `piz`.

- [ ] **Step 4: Run the tests to verify they pass**

Run: `mise exec -- cargo test -p panini-data`
Expected: PASS.

- [ ] **Step 5: Mirror the rows into the reference TSV**

Append to `data/dhatupatha.tsv`, after the existing `Kid` line, tab-separated in the file's existing column order (`code`, gaṇa, pada, artha):

```
Banj	rudhadi	parasmaipada	Amardane
piz	rudhadi	parasmaipada	saYcUrRane hiMsAyAM ca
inD	rudhadi	atmanepada	dIptO
```

- [ ] **Step 6: Confirm the expected workspace breakage, and that it is the ONLY breakage**

Run: `mise run test`
Expected: FAIL, and **only** in `crates/panini` — `paradigm_covers_every_enumerable_cell` and `derivation_set_shape_matches_the_audited_numbers`. Read the failure output and confirm no other test is red. Task 10 closes these.

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add crates/panini-data/src/lib.rs data/dhatupatha.tsv
git commit -m "feat(data): rudhādi 7b roots — √bhañj, √piṣ and √indh

√indh's pada was verified, not assumed: its ñi is read by 1.3.72, but
vidyut-prakriya derives it ātmanepada-only against a ~^r control that
derives both. All three ids equal their codes, so the aS.5 mechanism
stays at one user."
```

---

### Task 2: √bhañj and 8.2.30 `coH kuH`

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (new rule, plus its guard tests in the in-file `mod tests`)
- Modify: `crates/panini-prakriya/src/tinanta/anga.rs` (6.4.23's comment only — no code change)
- Test: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: `Dhatu` id `"Banj"` from Task 1.
- Produces: rule id `"8.2.30"` in `TRIPADI`, between 8.2.25 and 8.2.39. All 36 √bhañj cells become final in this task.

- [ ] **Step 1: Write the failing derivation tests**

Add to `crates/panini-prakriya/src/tinanta/derivation_tests.rs`:

```rust
#[test]
fn bhanj_lat_all_nine_cells() {
    // The strong stem velarises (Banaj + ti -> Banag + ti -> Banakti, via
    // 8.2.30 then 8.4.55); the weak stem does the same across the anusvāra
    // round trip (Banj + taH -> Bang + taH -> BaMgtaH -> BaMktaH ->
    // BaNktaH). The `n` that survives in BaNktaH is śnam's: 6.4.23 already
    // took the root's own `n` out.
    let cells = [
        (Purusha::Prathama, Vacana::Eka, "Banakti"),
        (Purusha::Prathama, Vacana::Dvi, "BaNktaH"),
        (Purusha::Prathama, Vacana::Bahu, "BaYjanti"),
        (Purusha::Madhyama, Vacana::Eka, "Banakzi"),
        (Purusha::Madhyama, Vacana::Dvi, "BaNkTaH"),
        (Purusha::Madhyama, Vacana::Bahu, "BaNkTa"),
        (Purusha::Uttama, Vacana::Eka, "Banajmi"),
        (Purusha::Uttama, Vacana::Dvi, "BaYjvaH"),
        (Purusha::Uttama, Vacana::Bahu, "BaYjmaH"),
    ];
    for (pu, va, want) in cells {
        assert_eq!(form_g("Banj", Lakara::Lat, pu, va), want);
    }
}

#[test]
fn bhanj_lan_eka_velarises_word_finally() {
    // 8.2.23 eats tip's own `t` (and sip's own `s`), leaving the dhātu's
    // `j` as the true word end; 8.2.30 then applies word-finally rather
    // than before a jhal. Both eka cells fork on 8.4.56 alone.
    assert_eq!(
        form_g_forked("Banj", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2),
        "aBanag"
    );
    assert_eq!(
        form_g_forked("Banj", Lakara::Lan, Purusha::Madhyama, Vacana::Eka, 2),
        "aBanag"
    );
}

#[test]
fn bhanj_lot_madhyama_eka_is_bhangdhi() {
    // 6.4.101 her dhiH gives the `Di`; 8.2.30 velarises the `j` before it
    // (a jhal), and 8.4.53 declines because `g` is already its own jaś.
    // Three branches: the declined one plus 7.1.35's tātaṅ and its 8.4.56
    // pausal fork.
    assert_eq!(
        form_g_forked("Banj", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "BaNgDi"
    );
}

#[test]
fn coh_kuh_declines_before_a_non_jhal_non_final() {
    // The witnesses that keep 8.2.30's guard from being written too wide.
    // In BaYjanti what follows the `j` is `a`, and in BaYjvaH it is `v` —
    // neither a jhal nor a word end — so the `j` survives to take 8.3.24's
    // anusvāra and 8.4.58's palatal parasavarṇa instead.
    assert_eq!(
        form_g("Banj", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "BaYjanti"
    );
    assert_eq!(
        form_g("Banj", Lakara::Lat, Purusha::Uttama, Vacana::Dvi),
        "BaYjvaH"
    );
}
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya bhanj`
Expected: FAIL — `bhanj_lat_all_nine_cells` reports `Banajti` (or similar unvelarised text) where `Banakti` is expected.

- [ ] **Step 3: Add 8.2.30 to `TRIPADI`**

**Rule specification.** Insert in `crates/panini-prakriya/src/tinanta/tripadi.rs` **between the 8.2.25 rule and the 8.2.39 rule** — sūtra order among the 8.2 block.

| field | value |
| --- | --- |
| `id` | `"8.2.30"` |
| `name` | `"coH kuH"` |
| `kind` | `RuleKind::Vidhi` |
| `vikalpa` | `false` |

**Condition.** A *cu* sound (`c`, `C`, `j`, `J`) is replaced by its *ku* counterpart when it is either word-final or immediately followed by a jhal. The substitute is the nearest velar by 1.1.50 *sthāne'ntaratamaḥ* — voicing and aspiration are preserved, so `j` → `g` and `c` → `k`. In this slice only `j` → `g` is reachable; write only the reachable arms, per the narrow-guard discipline, and let the mutation gate confirm each has a witness.

**Operation.** Rewrite that one character in place, then `p.record("8.2.30", "coH kuH", before)`.

**Reading the bearing term.** Locate the target the way 8.2.39 and 8.2.23 already do in this file — `p.terms.iter().rposition(|t| !t.text.is_empty())` — not a fixed `terms.len() - 1`. `ENDING` is empty in exactly the laṅ eka cells this rule must fire on, and a fixed index would write onto the empty term and leave the real `j` untouched.

**Witnesses.**

- Fires before a jhal: `Banakti` (`Banaj` + `ti`), `BaNktaH` (`Banj` + `taH`), `BaNgDi`.
- Fires word-finally: `aBanag` at laṅ prathama and madhyama eka.
- Declines: `BaYjanti` (`a` follows), `BaYjvaH` (`v` follows). Both are asserted in `coh_kuh_declines_before_a_non_jhal_non_final`.

**No interaction with 8.2.39.** `j` → `g` is already voiced and unaspirated, so `aBanag` needs no jaśtva; 8.2.39's `ends_with('t')` guard declines on it untouched. Do not widen 8.2.39 here — that widening belongs to Task 5 and is for √piṣ's `z`.

- [ ] **Step 4: Add the rule id to the pinned order**

In `crates/panini-prakriya/src/tinanta/derivation_tests.rs`, add `"8.2.30"` to `tinanta_rule_order_is_pinned` between `"8.2.25"` and `"8.2.39"`.

Leave `exactly_the_pinned_vikalpa_rules_are_optional` alone — 8.2.30 is obligatory.

- [ ] **Step 5: Add the per-rule guard test**

Add to `tripadi.rs`'s `mod tests` a test named `coh_kuh_fires_only_word_finally_or_before_a_jhal`, built on a hand-constructed `Prakriya` in the idiom of the neighbouring `jhalam_jasho_ante_fires_only_on_a_pada_final_t`. Assert all three arms: a `j` before a jhal is velarised, a word-final `j` is velarised, and a `j` before a vowel is left alone with the rule returning `false`.

- [ ] **Step 6: Correct 6.4.23's comment**

In `crates/panini-prakriya/src/tinanta/anga.rs`, 6.4.23's comment currently claims "7b widens it for √bhañj, √und and √indh, whose tails are `fj`, `nd` and `nD`". **No code changes.** Both halves of that sentence are wrong and must be replaced:

- The guard is already `rest.starts_with('n')`, which covers √bhañj's `nj` and √indh's `nD` unchanged. The rule gains **witnesses**, not a widening.
- √bhañj's tail is `nj`, not `fj` — a typo.

Rewrite the paragraph to say so, and record the split it performs on √bhañj concretely: `Banj` splits as `Ba | na | nj`; 6.4.23 removes the root's own `n` leaving `Ba | na | j`; 6.4.111 removes śnam's `a` in the weak cells leaving `Ba | n | j`. Add that this is also why **6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti* is not needed** in this slice: the nasal that drops sits immediately behind śnam's `na` and is 6.4.23's by its own terms, while 6.4.24 governs the penultimate nasal of roots like √añj and √tañc, both out of scope.

- [ ] **Step 7: Run the scoped suite**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS, including all four new derivation tests and the new guard test.

- [ ] **Step 8: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/tripadi.rs \
        crates/panini-prakriya/src/tinanta/anga.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 8.2.30 coH kuH, and √bhañj

Velarises the dhātu's palatal before a jhal (Banakti, BaNktaH) and
word-finally (aBanag). BaYjanti and BaYjvaH are the declining witnesses.

Also corrects 6.4.23's comment: its guard already covers √bhañj's tail
unchanged, so the rule gains a witness rather than a widening, and the
tail is nj, not fj. 6.4.24 is not needed in this slice."
```

---

### Task 3: √piṣ's retroflexion — 8.4.41 `zwunA zwuH`

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs`
- Test: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: `Dhatu` id `"piz"` from Task 1.
- Produces: rule id `"8.4.41"` in `TRIPADI`, between 8.3.59 and 8.4.53.

**Cells this task finalises:** all of vidhiliṅ; laṭ except madhyama eka (`pinakzi`, Task 4); laṅ except the two eka cells (`apinaq`, Task 5); loṭ except madhyama eka (`piRqQi`, Task 6). Do **not** write tests for those three cells yet — they are still intermediate and will read `pinazsi`-ish, `apinaz` and `piMzQi` respectively.

- [ ] **Step 1: Write the failing derivation tests**

Add to `derivation_tests.rs`:

```rust
#[test]
fn pish_lat_retroflexes_around_the_shnam_stem() {
    // 8.4.41 ṣṭunā ṣṭuḥ: the ending's dental retroflexes in contact with
    // the root's ṣ. Madhyama eka (pinakzi) is deliberately absent — it
    // needs 8.2.41, which lands in the next task.
    let cells = [
        (Purusha::Prathama, Vacana::Eka, "pinazwi"),
        (Purusha::Prathama, Vacana::Dvi, "piMzwaH"),
        (Purusha::Prathama, Vacana::Bahu, "piMzanti"),
        (Purusha::Madhyama, Vacana::Dvi, "piMzWaH"),
        (Purusha::Madhyama, Vacana::Bahu, "piMzWa"),
        (Purusha::Uttama, Vacana::Eka, "pinazmi"),
        (Purusha::Uttama, Vacana::Dvi, "piMzvaH"),
        (Purusha::Uttama, Vacana::Bahu, "piMzmaH"),
    ];
    for (pu, va, want) in cells {
        assert_eq!(form_g("piz", Lakara::Lat, pu, va), want);
    }
}

#[test]
fn pish_weak_stem_keeps_its_anusvara() {
    // The SECOND witness that 8.3.24 and 8.4.58 are not a no-op pair.
    // 8.4.58 needs a yay to follow; what follows here is the root's own
    // `z`, which is śal — so piMzwaH keeps the anusvāra that kfntaH
    // resolves. √hiṃs's hiMstaH was the first witness, in 7a.
    assert_eq!(
        form_g("piz", Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
        "piMzwaH"
    );
}

#[test]
fn shtutva_requires_strict_adjacency() {
    // piMzanti keeps a DENTAL n: the `a` between the ṣ and the n breaks
    // the contact 8.4.41 requires. pinazARi's retroflex ṇ is a different
    // rule's — ṇatva (8.4.1 / 8.4.2), which 8.4.2 explicitly lets an aṭ
    // intervene in. Conflating the two would retroflex piMzanti as well.
    assert_eq!(
        form_g("piz", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "piMzanti"
    );
    assert_eq!(
        form_g("piz", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
        "pinazARi"
    );
}

#[test]
fn pish_vidhilin_all_nine_cells() {
    // The optative's `y` is neither dental stop nor `s`, so 8.4.41 has
    // nothing to do here; the cells are pure weak stem plus 8.4.56 on
    // prathama eka.
    assert_eq!(
        form_g_forked("piz", Lakara::VidhiLin, Purusha::Prathama, Vacana::Eka, 2),
        "piMzyAd"
    );
    let cells = [
        (Purusha::Prathama, Vacana::Dvi, "piMzyAtAm"),
        (Purusha::Prathama, Vacana::Bahu, "piMzyuH"),
        (Purusha::Madhyama, Vacana::Eka, "piMzyAH"),
        (Purusha::Madhyama, Vacana::Dvi, "piMzyAtam"),
        (Purusha::Madhyama, Vacana::Bahu, "piMzyAta"),
        (Purusha::Uttama, Vacana::Eka, "piMzyAm"),
        (Purusha::Uttama, Vacana::Dvi, "piMzyAva"),
        (Purusha::Uttama, Vacana::Bahu, "piMzyAma"),
    ];
    for (pu, va, want) in cells {
        assert_eq!(form_g("piz", Lakara::VidhiLin, pu, va), want);
    }
}
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya pish`
Expected: FAIL — `pinazti` where `pinazwi` is expected.

- [ ] **Step 3: Add 8.4.41 to `TRIPADI`**

**Rule specification.** Insert **between the 8.3.59 rule and the 8.4.53 rule**.

| field | value |
| --- | --- |
| `id` | `"8.4.41"` |
| `name` | `"zwunA zwuH"` |
| `kind` | `RuleKind::Vidhi` |
| `vikalpa` | `false` |

**Condition.** A dental — `s`, or a *t*-varga stop (`t`, `T`, `d`, `D`, `n`) — becomes its retroflex counterpart when **immediately** adjacent to `ṣ` (`z`) or a *ṭ*-varga stop (`w`, `W`, `q`, `Q`, `R`). Correspondence is by manner: `t`→`w`, `T`→`W`, `d`→`q`, `D`→`Q`, `n`→`R`, `s`→`z`. Only the arms reachable from this root set need writing; `piMzwaH`, `piMzWaH` and `piMz`+`Di` cover `t`, `T` and `D`.

**Strict adjacency is the load-bearing part of the guard.** The trigger and the target must be neighbouring characters in the word. An implementation that scanned forward for a dental after a `ṣ` would retroflex `piMzanti`'s `n` and derive `*piMzaRti`. The `pinazARi` assertion in `shtutva_requires_strict_adjacency` is what proves the two rules stay separate: that retroflexion is ṇatva's, across an intervening `A`.

**Operation.** Rewrite the one character, then `p.record("8.4.41", "zwunA zwuH", before)`.

**Placement rationale, to record in the comment.** Sūtra order puts it above 8.4.53, and that is the only reason it sits there — it is **not** a load-bearing order. `piMz` + `Di` reaches `piRqQi` identically either way, because ṣṭutva and jaśtva touch different sounds in either sequence. Record this explicitly so a later reader does not rediscover it as a constraint and build on it.

- [ ] **Step 4: Add the rule id to the pinned order**

Add `"8.4.41"` to `tinanta_rule_order_is_pinned` between `"8.3.59"` and `"8.4.53"`.

- [ ] **Step 5: Add the per-rule guard test**

Add to `tripadi.rs`'s `mod tests` a test named `shtutva_fires_only_on_an_adjacent_dental`. Assert: a dental immediately after `z` retroflexes; the same dental with one character between it and the `z` is left alone and the rule returns `false`; and a non-dental neighbour (`y`, `v`, `m`) is left alone.

- [ ] **Step 6: Run the scoped suite**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/tripadi.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 8.4.41 zwunA zwuH, and √piṣ's retroflexion

Retroflexes the ending's dental against the root's ṣ (pinazwi, piMzwaH,
piMzWaH). The guard requires strict adjacency: piMzanti keeps its dental
n, and pinazARi's ṇ is ṇatva's, which 8.4.2 lets an aṭ intervene in.

piMzwaH is the second witness that 8.3.24 and 8.4.58 are not a no-op
pair — 8.4.58 needs a yay and `z` is śal."
```

---

### Task 4: `pinakzi` — 8.2.41 `zaQoH kaH si` and the 8.3.59 widening

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (new rule, plus 8.3.59's guard)
- Test: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: `"piz"`, and 8.3.59 as it stands after 7a.
- Produces: rule id `"8.2.41"` in `TRIPADI` between 8.2.39 and 8.2.74; a widened 8.3.59.

- [ ] **Step 1: Write the failing derivation tests**

```rust
#[test]
fn pish_lat_madhyama_eka_is_pinakshi() {
    // 8.2.41 ṣaḍhoḥ kaḥ si takes the ṣ to `k` before the ending's `s`,
    // and 8.3.59 then retroflexes that `s` after the new `k` — the
    // widening this cell forces, and the one 8.3.59's own comment
    // predicted ("h/y/v/r/l or k").
    assert_eq!(
        form_g("piz", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
        "pinakzi"
    );
}

#[test]
fn shadhoh_kah_si_declines_when_8_2_23_ate_the_s_first() {
    // THE LOAD-BEARING ORDER of this slice. At laṅ madhyama eka the
    // ending is a bare `s`, so 8.2.23 saṁyogāntasya lopaḥ elides it
    // before 8.2.41 can see it, and the cell reduces exactly as laṅ
    // prathama eka does. Run 8.2.41 above 8.2.23 and you get `apinak`
    // instead — a plausible-looking form that splits madhyama eka from
    // prathama eka and that no guard test would flag.
    //
    // Asserted here on the intermediate `apinaz`: 8.2.39's widening (the
    // next task) is what carries it on to apinaq/apinaw.
    assert_eq!(
        form_g("piz", Lakara::Lan, Purusha::Madhyama, Vacana::Eka),
        "apinaz"
    );
}
```

Note the second test asserts an **intermediate** value that Task 5 changes. That is deliberate: it pins the ordering fact at the point it becomes true, before 8.2.39's widening can mask it. Task 5 updates the expectation and keeps the test.

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya pish_lat_madhyama_eka shadhoh`
Expected: FAIL — `pinazsi` or `pinazzi` where `pinakzi` is expected.

- [ ] **Step 3: Add 8.2.41 to `TRIPADI`**

**Rule specification.** Insert **between the 8.2.39 rule and the 8.2.74 rule**, so that it sits **below 8.2.23**.

| field | value |
| --- | --- |
| `id` | `"8.2.41"` |
| `name` | `"zaQoH kaH si"` |
| `kind` | `RuleKind::Vidhi` |
| `vikalpa` | `false` |

**Condition.** `ṣ` (`z`) or `ḍh` (`Q`) is replaced by `k` when the immediately following sound is `s`. Only the `z` arm is reachable in this slice.

**Operation.** Rewrite the one character, then `p.record("8.2.41", "zaQoH kaH si", before)`.

**The placement comment must record why it sits below 8.2.23**, in the terms of `shadhoh_kah_si_declines_when_8_2_23_ate_the_s_first`: at laṅ madhyama eka the ending is a bare `s` that 8.2.23 elides as a word-final conjunct's second member, so 8.2.41 finds no trigger and the cell reduces exactly as laṅ prathama eka does. Reversed, the `ṣ` becomes `k` before the `s` is elided and the cell surfaces `apinak`. This is a real word-looking form; the golden and the trace pin are the only things that catch it.

- [ ] **Step 4: Widen 8.3.59**

8.3.59's trigger guard currently requires the sound before the s-initial affix to be a vowel other than `a`/`A`. `pinak` + `si` presents a `k`.

Add `k` to the accepted triggers — **only** `k`. The rest of the iṇ pratyāhāra (`h y v r l`, and the other vowels) stays unreachable and unwritten, per the narrow-guard discipline and per what the rule's own comment already asks for. Update that comment: the prediction it makes ("widen it the moment a root lands whose aṅga ends in h/y/v/r/l or `k` before an s-initial affix") has now come true for `k`, and should be restated as a record of what happened plus a standing note for the remaining sounds.

Note the trigger here is **not** an aṅga-final sound: it is `SHAP`'s final `k`, because rudhādi's śnam split puts the root's tail in `SHAP`. 8.3.59 already reads "the last char of the nearest non-empty preceding term" rather than `ANGA` directly, so it needs no structural change — only the character class.

- [ ] **Step 5: Add the rule id to the pinned order**

Add `"8.2.41"` to `tinanta_rule_order_is_pinned` between `"8.2.39"` and `"8.2.74"`.

- [ ] **Step 6: Add the per-rule guard tests**

In `tripadi.rs`'s `mod tests`:

- `shadhoh_kah_si_fires_only_before_an_s` — a `z` before `s` becomes `k`; a `z` before any other sound is untouched and the rule returns `false`.
- Extend the existing 8.3.59 guard tests (`shatva_declines_for_every_pre_existing_junction`, `shatva_reads_the_sound_before_the_affix_not_the_anga`) with a `k`-trigger case. Do **not** write a new parallel test where an existing one covers the same ground.

- [ ] **Step 7: Run the scoped suite**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS.

- [ ] **Step 8: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/tripadi.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 8.2.41 zaQoH kaH si, and 8.3.59's k trigger

pinakzi: the ṣ becomes k before sip's s, and 8.3.59 then retroflexes
that s after the new k — the widening 8.3.59's own comment predicted.

8.2.41 sits BELOW 8.2.23 and that is load-bearing: at laṅ madhyama eka
8.2.23 elides the bare `s` first, so this rule correctly finds no
trigger. Reversed, the cell would surface apinak."
```

---

### Task 5: `apinaq` / `apinaw` — the 8.2.39 widening and `jashtva_of('z')`

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/sound.rs` (`jashtva_of`)
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (8.2.39's guard and substitute)
- Test: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`, and `sound.rs`'s `mod tests`

**Interfaces:**
- Consumes: `"piz"`.
- Produces: `jashtva_of('z') == Some('q')`, consumed again by Task 6's 8.4.53 widening.

- [ ] **Step 1: Write the failing tests**

In `sound.rs`'s `mod tests`, extend `parasavarna_of_stops_all_arms`'s sibling `jashtva_of` test (or add one if none exists) with:

```rust
        // ṣ has no jaś by place alone — the sibilants are not stops. 1.1.50
        // sthāne'ntaratamaḥ selects the nearest, which for retroflex ṣ is
        // retroflex ḍ. `S` and `s` stay absent: `S` is unreachable here,
        // and a word-final `s` is 8.2.66 / 8.3.15's, not jaśtva's.
        assert_eq!(jashtva_of('z'), Some('q'));
        assert_eq!(jashtva_of('S'), None);
        assert_eq!(jashtva_of('s'), None);
```

In `derivation_tests.rs`, **update** `shadhoh_kah_si_declines_when_8_2_23_ate_the_s_first` from Task 4 to its final expectation, and add the prathama eka cell:

```rust
#[test]
fn shadhoh_kah_si_declines_when_8_2_23_ate_the_s_first() {
    // THE LOAD-BEARING ORDER of this slice. At laṅ madhyama eka the
    // ending is a bare `s`, so 8.2.23 saṁyogāntasya lopaḥ elides it
    // before 8.2.41 can see it, and the cell reduces through 8.2.39 and
    // 8.4.56 to exactly what laṅ prathama eka gives. Run 8.2.41 above
    // 8.2.23 and you get `apinak` instead — a plausible-looking form that
    // splits madhyama eka from prathama eka.
    assert_eq!(
        form_g_forked("piz", Lakara::Lan, Purusha::Madhyama, Vacana::Eka, 2),
        "apinaq"
    );
    assert_eq!(
        form_g_forked("piz", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2),
        "apinaq"
    );
}

#[test]
fn pish_lan_all_nine_cells() {
    let cells = [
        (Purusha::Prathama, Vacana::Dvi, "apiMzwAm"),
        (Purusha::Prathama, Vacana::Bahu, "apiMzan"),
        (Purusha::Madhyama, Vacana::Dvi, "apiMzwam"),
        (Purusha::Madhyama, Vacana::Bahu, "apiMzwa"),
        (Purusha::Uttama, Vacana::Eka, "apinazam"),
        (Purusha::Uttama, Vacana::Dvi, "apiMzva"),
        (Purusha::Uttama, Vacana::Bahu, "apiMzma"),
    ];
    for (pu, va, want) in cells {
        assert_eq!(form_g("piz", Lakara::Lan, pu, va), want);
    }
}
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: FAIL — `jashtva_of('z')` returns `None`, and the laṅ eka cells return `apinaz` un-forked (so `form_g_forked` fails on the branch count, expecting 2 and getting 1).

- [ ] **Step 3: Add the `z` arm to `jashtva_of`**

Add `'z' => 'q'` to the match in `crates/panini-prakriya/src/tinanta/sound.rs`. Update the doc comment: it currently says `None` covers "the sibilants and `h`", which stops being true. State that `ṣ` is the one sibilant with a jaś here, selected by 1.1.50 as the nearest rather than by place-and-manner correspondence, and that `S` and `s` deliberately remain `None`.

- [ ] **Step 4: Widen 8.2.39**

8.2.39's guard is `p.text().ends_with('t')` and its operation hardcodes `s.push('d')`.

Widen the guard to accept a word-final `t` **or** `z`, and take the substitute from `jashtva_of` instead of the literal `'d'`.

**Do not widen it to all jhal.** A word-final `s` is 8.2.66 / 8.3.15's business, not jaśtva's, and a blanket widening would rewrite √hiṃs's `ahinas` to `ahinad` before 8.2.74 and 8.2.73 could act on it, destroying the ru alternation and the `ahinaH` branch. Record that in the comment as the reason the guard names its two characters rather than calling `is_jhal`.

Rename the existing guard test `jhalam_jasho_ante_fires_only_on_a_pada_final_t` to `jhalam_jasho_ante_fires_only_on_a_pada_final_t_or_sh`, and add arms for `z` → `q` and for a word-final `s` being left alone.

- [ ] **Step 5: Run the scoped suite**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS. In particular `hins_lan_*` and `shnams_ru_fires_on_the_dhatus_own_final` from 7a must still pass — they are what prove the `s` exclusion held.

- [ ] **Step 6: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/sound.rs \
        crates/panini-prakriya/src/tinanta/tripadi.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 8.2.39 takes a final ṣ, via jashtva_of('z')

apinaq: ṣ has no jaś by place, so 1.1.50 sthāne'ntaratamaḥ selects the
nearest — retroflex ḍ. 8.4.56 then gives apinaw.

The guard names `t` and `z` rather than calling is_jhal: a word-final s
is 8.2.66/8.3.15's, and widening to all jhal would rewrite √hiṃs's
ahinas before 8.2.74 and 8.2.73 could act."
```

---

### Task 6: `piRqQi` / `piRQi` — the 8.4.53 widening and `is_jhash`

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/sound.rs` (new `is_jhash`)
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (8.4.53's guard)
- Test: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`, and `sound.rs`'s `mod tests`

**Interfaces:**
- Consumes: `jashtva_of('z')` from Task 5.
- Produces: `is_jhash(c: char) -> bool`; a generalised 8.4.53, consumed by Task 7's √indh.

- [ ] **Step 1: Write the failing tests**

In `sound.rs`'s `mod tests`, a test pinning every arm of the new classifier:

```rust
    #[test]
    fn is_jhash_covers_exactly_the_voiced_aspirates() {
        for c in ['G', 'J', 'Q', 'D', 'B'] {
            assert!(is_jhash(c), "{c} is a jhaś");
        }
        for c in ['g', 'j', 'q', 'd', 'b', 'k', 'c', 'w', 't', 'p', 's', 'z', 'S', 'h', 'a'] {
            assert!(!is_jhash(c), "{c} is not a jhaś");
        }
    }
```

In `derivation_tests.rs`:

```rust
#[test]
fn pish_lot_madhyama_eka_is_pinddhi() {
    // The deepest cell in this slice: four branches. 6.4.101 her dhiH
    // gives the `Di`; 8.4.41 retroflexes it to `Qi`; 8.4.53 (widened to
    // any jhaś, not just `D`) voices the ṣ to `q` before it; 8.4.58 takes
    // the anusvāra to `R` as that `q`'s parasavarṇa; and 8.4.65 optionally
    // elides the `q` before the savarṇa `Q`. 7.1.35's tātaṅ and its 8.4.56
    // fork supply the other two branches.
    assert_eq!(
        form_g_forked("piz", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 4),
        "piRqQi"
    );
}

#[test]
fn pish_lot_all_nine_cells() {
    assert_eq!(
        form_g_forked("piz", Lakara::Lot, Purusha::Prathama, Vacana::Eka, 3),
        "pinazwu"
    );
    let cells = [
        (Purusha::Prathama, Vacana::Dvi, "piMzwAm"),
        (Purusha::Prathama, Vacana::Bahu, "piMzantu"),
        (Purusha::Madhyama, Vacana::Dvi, "piMzwam"),
        (Purusha::Madhyama, Vacana::Bahu, "piMzwa"),
        (Purusha::Uttama, Vacana::Eka, "pinazARi"),
        (Purusha::Uttama, Vacana::Dvi, "pinazAva"),
        (Purusha::Uttama, Vacana::Bahu, "pinazAma"),
    ];
    for (pu, va, want) in cells {
        assert_eq!(form_g("piz", Lakara::Lot, pu, va), want);
    }
}

#[test]
fn jhalam_jash_jhashi_still_declines_on_its_two_pre_existing_shapes() {
    // The widening must not disturb either 7a cell. √khid's KindDve
    // presents a `d` that is already its own jaś — the NO-OP GUARD
    // declines it. √hiṃs's hinDi presents an `n`, for which jashtva_of
    // returns None — a DIFFERENT clause. Both remain branch 0.
    assert_eq!(
        form_g_forked("Kid", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu, 2),
        "KindDve"
    );
    assert_eq!(
        form_g_forked("his", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "hinDi"
    );
}
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: FAIL — `is_jhash` does not exist (compile error), so fix that first, then `pish_lot_madhyama_eka_is_pinddhi` fails on branch count (2, not 4) and text (`piMzQi`).

- [ ] **Step 3: Add `is_jhash` to `sound.rs`**

A predicate for the *jhaś* pratyāhāra — the five voiced aspirated stops `G`, `J`, `Q`, `D`, `B`. Place it beside `is_jhal` and `is_khar`, with a doc comment naming the pratyāhāra and noting that it is 8.4.53's conditioning class.

- [ ] **Step 4: Generalise 8.4.53's guard**

8.4.53 currently requires the word to end in `i` with `D` as its penult — that is, only the `Di` ending 7a reached it through. Replace that with the rule's actual condition: **a jhal immediately followed by a jhaś**, located anywhere in the word, with the jhal replaced by its `jashtva_of`.

Keep the existing **no-op guard** (a target already equal to its own jaś must not record a vacuous step) — it is load-bearing, and `jhalam_jash_jhashi_still_declines_on_its_two_pre_existing_shapes` is the witness for it and for the `jashtva_of` → `None` path.

Update the comment. It currently describes the conditioning jhaś as "in this suite always the `D` of 6.4.101's Di, at the last position", which stops being true: √piṣ conditions it on a `Q` and √indh (next task) on a `D` in `De`, `Da`, `DAm`, `Dve`, `DAH` and `Dvam`. Record also that the widening is self-limiting upstream — 8.2.40 is the only new source of a `D`-initial ending, no thematic root reaches it because the vikaraṇa intervenes (`laBate`, `yuDyate`, `guDnAti`), and no other athematic stem in the suite ends in a jhaṣ (`vaste`, `Aste`, `Sete`).

- [ ] **Step 5: Extend the per-rule guard test**

Extend 8.4.53's existing guard test in `tripadi.rs`'s `mod tests` (do not add a parallel one) with: a jhal before a jhaś in a **non-final** position fires; a jhal before a non-jhaś does not; and a target already its own jaś returns `false`.

- [ ] **Step 6: Run the scoped suite**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS. All 36 √piṣ cells are now final.

- [ ] **Step 7: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/sound.rs \
        crates/panini-prakriya/src/tinanta/tripadi.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 8.4.53 conditions on any jhaś, not just Di

piRqQi needs the ṣ voiced before a retroflex Qi, which the old guard
(word ends in `i`, penult `D`) could not see. Generalised to the rule's
real condition — a jhal before a jhaś — via a new is_jhash.

Both 7a cells are untouched, by two DIFFERENT clauses: KindDve by the
no-op guard, hinDi by jashtva_of returning None."
```

---

### Task 7: √indh and 8.2.40 `JazastaTorDo'DaH`

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs`
- Test: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: `"inD"` from Task 1; the widened 8.4.53 from Task 6.
- Produces: rule id `"8.2.40"` in `TRIPADI`, between 8.2.39 and 8.2.41. All 36 √indh cells become final; the grammar for this slice is then complete.

- [ ] **Step 1: Write the failing derivation tests**

```rust
#[test]
fn indh_lat_all_nine_cells() {
    // 8.2.40 turns the ending's `t` into `D` after the stem's jhaṣ, and
    // the widened 8.4.53 then voices the stem's own `D` to `d` before it:
    // inD + te -> inD + De -> indDe. 8.4.65 optionally elides that `d`
    // before the savarṇa `D`, which is where inDe comes from.
    assert_eq!(
        form_g_forked("inD", Lakara::Lat, Purusha::Prathama, Vacana::Eka, 2),
        "indDe"
    );
    assert_eq!(
        form_g_forked("inD", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu, 2),
        "indDve"
    );
    let cells = [
        (Purusha::Prathama, Vacana::Dvi, "inDAte"),
        (Purusha::Prathama, Vacana::Bahu, "inDate"),
        (Purusha::Madhyama, Vacana::Eka, "intse"),
        (Purusha::Madhyama, Vacana::Dvi, "inDATe"),
        (Purusha::Uttama, Vacana::Eka, "inDe"),
        (Purusha::Uttama, Vacana::Dvi, "inDvahe"),
        (Purusha::Uttama, Vacana::Bahu, "inDmahe"),
    ];
    for (pu, va, want) in cells {
        assert_eq!(form_g("inD", Lakara::Lat, pu, va), want);
    }
}

#[test]
fn jhashas_tathor_dhodhah_declines_before_a_non_dental() {
    // intse is the witness that 8.2.40 is not simply "voice everything
    // after the stem". sip's `se` begins with `s`, not `t`/`th`, so the
    // rule declines and 8.4.55 khari ca devoices the stem's `D` to `t`
    // instead. inDvahe and inDmahe make the same point for `v` and `m`.
    assert_eq!(
        form_g("inD", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
        "intse"
    );
    assert_eq!(
        form_g("inD", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
        "intsva"
    );
}

#[test]
fn indh_strong_stem_appears_only_in_lot_uttama() {
    // The ātmanepada endings are ṅit throughout except loṭ uttama, where
    // the strong stem inaD survives 6.4.111 and shows śnam's `a`.
    assert_eq!(
        form_g("inD", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
        "inaDE"
    );
    assert_eq!(
        form_g("inD", Lakara::Lot, Purusha::Uttama, Vacana::Dvi),
        "inaDAvahE"
    );
    assert_eq!(
        form_g("inD", Lakara::Lot, Purusha::Uttama, Vacana::Bahu),
        "inaDAmahE"
    );
}

#[test]
fn indh_lan_and_lot_and_vidhilin_cells() {
    // laṅ takes the āṭ augment, which 6.1.90 āṭaś ca raises to `E`.
    for (pu, va, want) in [
        (Purusha::Prathama, Vacana::Eka, "EndDa"),
        (Purusha::Madhyama, Vacana::Eka, "EndDAH"),
        (Purusha::Madhyama, Vacana::Bahu, "EndDvam"),
    ] {
        assert_eq!(form_g_forked("inD", Lakara::Lan, pu, va, 2), want);
    }
    for (pu, va, want) in [
        (Purusha::Prathama, Vacana::Dvi, "EnDAtAm"),
        (Purusha::Prathama, Vacana::Bahu, "EnData"),
        (Purusha::Madhyama, Vacana::Dvi, "EnDATAm"),
        (Purusha::Uttama, Vacana::Eka, "EnDi"),
        (Purusha::Uttama, Vacana::Dvi, "EnDvahi"),
        (Purusha::Uttama, Vacana::Bahu, "EnDmahi"),
    ] {
        assert_eq!(form_g("inD", Lakara::Lan, pu, va), want);
    }

    assert_eq!(
        form_g_forked("inD", Lakara::Lot, Purusha::Prathama, Vacana::Eka, 2),
        "indDAm"
    );
    assert_eq!(
        form_g_forked("inD", Lakara::Lot, Purusha::Madhyama, Vacana::Bahu, 2),
        "indDvam"
    );
    for (pu, va, want) in [
        (Purusha::Prathama, Vacana::Dvi, "inDAtAm"),
        (Purusha::Prathama, Vacana::Bahu, "inDatAm"),
        (Purusha::Madhyama, Vacana::Dvi, "inDATAm"),
    ] {
        assert_eq!(form_g("inD", Lakara::Lot, pu, va), want);
    }

    // vidhiliṅ takes no fork at all: the optative `I` is neither a jhal
    // nor pada-final, so neither 8.4.65 nor 8.4.56 reaches these cells.
    for (pu, va, want) in [
        (Purusha::Prathama, Vacana::Eka, "inDIta"),
        (Purusha::Prathama, Vacana::Dvi, "inDIyAtAm"),
        (Purusha::Prathama, Vacana::Bahu, "inDIran"),
        (Purusha::Madhyama, Vacana::Eka, "inDITAH"),
        (Purusha::Madhyama, Vacana::Dvi, "inDIyATAm"),
        (Purusha::Madhyama, Vacana::Bahu, "inDIDvam"),
        (Purusha::Uttama, Vacana::Eka, "inDIya"),
        (Purusha::Uttama, Vacana::Dvi, "inDIvahi"),
        (Purusha::Uttama, Vacana::Bahu, "inDImahi"),
    ] {
        assert_eq!(form_g("inD", Lakara::VidhiLin, pu, va), want);
    }
}
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya indh`
Expected: FAIL — `inte` or `intte` where `indDe` is expected, and branch counts of 1 where 2 are expected.

- [ ] **Step 3: Add 8.2.40 to `TRIPADI`**

**Rule specification.** Insert **between the 8.2.39 rule and the 8.2.41 rule**.

| field | value |
| --- | --- |
| `id` | `"8.2.40"` |
| `name` | `"JazastaTorDo'DaH"` |
| `kind` | `RuleKind::Vidhi` |
| `vikalpa` | `false` |

**Condition.** `t` or `T` is replaced by `D` when the immediately preceding sound is a *jhaṣ* — a voiced aspirated stop, i.e. `is_jhash` from Task 6. In this slice the preceding jhaṣ is always the stem's own `D`.

**Operation.** Rewrite the one character, then `p.record("8.2.40", "JazastaTorDo'DaH", before)`.

**Note in the comment** that this rule is the only new source of a `D`-initial ending in the suite, which is what bounds Task 6's 8.4.53 widening; and that it declines wherever the ending does not begin with a dental stop — `intse` (`s`), `inDvahe` (`v`), `inDmahe` (`m`) — leaving 8.4.55 *khari ca* to devoice the stem instead.

- [ ] **Step 4: Add the rule id to the pinned order**

Add `"8.2.40"` to `tinanta_rule_order_is_pinned` between `"8.2.39"` and `"8.2.41"`.

- [ ] **Step 5: Add the per-rule guard test**

Add `jhashas_tathor_dhodhah_fires_only_on_a_dental_after_a_jhash` to `tripadi.rs`'s `mod tests`: a `t` after `D` becomes `D`; a `t` after a non-jhaṣ (`d`, `n`, `s`) is untouched; and an `s` after `D` is untouched.

- [ ] **Step 6: Run the scoped suite**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS. **The grammar for this slice is now complete** — all 108 new cells derive correctly, though `crates/panini` is still red pending Task 10.

- [ ] **Step 7: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/tripadi.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(prakriya): 8.2.40 JazastaTorDo'DaH, and √indh

inD + te -> inD + De -> indDe, whence inDe by 8.4.65. Reaches seven of
√indh's cells. intse, inDvahe and inDmahe are the declining witnesses,
where 8.4.55 khari ca devoices the stem instead.

Completes the grammar for slice 7b: all 108 cells now derive."
```

---

### Task 8: 7a deferred #1 and #3 — the 8.2.7x block

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (move 8.2.75; rewrite 8.2.73's comment)
- Test: `crates/panini-prakriya/src/tinanta/derivation_tests.rs`

**Interfaces:**
- Consumes: nothing new.
- Produces: `TRIPADI` order `8.2.74, 8.2.75, 8.2.73`; `tinanta_rule_order_is_pinned` updated to match.

- [ ] **Step 1: Write the failing test for #3's verdict**

```rust
#[test]
fn the_ru_alternation_stays_off_the_new_roots() {
    // 8.2.73's deferred re-verification, discharged. √bhañj and √piṣ are
    // the first roots other than √hiṃs to empty ENDING under 8.2.23, so
    // they are the first live test of the invariant 8.2.73 leans on in
    // place of a slot predicate.
    //
    // The invariant HOLDS: both empty it at laṅ prathama/madhyama eka,
    // i.e. still tip and sip. And 8.2.73 declines on them regardless,
    // because its `s`-final check does not match `aBanaj` or `apinaz`.
    // If it over-fired, these cells would surface a `d` and then a
    // visarga via 8.2.75 and 8.3.15.
    for (root, want) in [("Banj", "aBanag"), ("piz", "apinaq")] {
        for pu in [Purusha::Prathama, Purusha::Madhyama] {
            assert_eq!(
                form_g_forked(root, Lakara::Lan, pu, Vacana::Eka, 2),
                want,
                "{root} laṅ eka took the ru alternation"
            );
        }
    }
}
```

Also assert the log directly, since a surface match alone would not prove *which* rule stayed away:

```rust
#[test]
fn no_8_2_73_step_appears_for_bhanj_or_pish() {
    for root in ["Banj", "piz"] {
        for pu in [Purusha::Prathama, Purusha::Madhyama] {
            let d = dhatus().iter().find(|d| d.id == root).unwrap();
            for p in derive(d, Lakara::Lan, d.pada, pu, Vacana::Eka) {
                assert!(
                    !p.log.iter().any(|s| s.sutra == "8.2.73"),
                    "{root}: 8.2.73 fired outside √hiṃs"
                );
            }
        }
    }
}
```

- [ ] **Step 2: Run them to verify they pass already**

Run: `mise exec -- cargo test -p panini-prakriya ru_alternation no_8_2_73`
Expected: **PASS immediately.** This is a characterisation test, not a red-green cycle — the point is to pin a verdict that was previously only a comment. If either fails, stop: 8.2.73 is over-firing and the invariant did not hold, which is a spec-level finding, not an implementation bug.

- [ ] **Step 3: Replace 8.2.73's standing warning with the verdict**

8.2.73's comment ends with "Re-verify this invariant before widening the root set." That has now been done. Replace it with the finding: √bhañj and √piṣ do empty `ENDING` under 8.2.23, and both do so at laṅ prathama/madhyama eka, so the invariant holds; 8.2.73 declines on them anyway via its `s`-final check; `the_ru_alternation_stays_off_the_new_roots` and `no_8_2_73_step_appears_for_bhanj_or_pish` are the witnesses. Keep the warning's forward-looking half — a future root set that empties `ENDING` at some other slot would still be a hazard.

- [ ] **Step 4: Move 8.2.75 above 8.2.73 and delete its `p.log` read**

Move the whole 8.2.75 `Rule` so it sits **between 8.2.74 and 8.2.73**, and delete this clause from its body:

```rust
            if p.log.iter().any(|s| s.sutra == "8.2.73") {
                return false;
            }
```

At the new position that clause is unreachable by construction — 8.2.73 has not run yet.

The forms do not change, and now rest on phonology rather than on rule history: at the new position √hiṃs presents `ahinas`, which fails 8.2.75's own `ends_with('d')` check, and √kṛt presents `akfRad` (8.2.39 having voiced it) and fires. Rewrite the comment accordingly — the long "MUST DECLINE ON A `d` THAT CAME FROM 8.2.73" paragraph is now history, and should be recorded as *why the rule moved*, not as a live constraint.

- [ ] **Step 5: Update the pinned order**

In `tinanta_rule_order_is_pinned`, reorder to `"8.2.74"`, `"8.2.75"`, `"8.2.73"`.

`VIKALPA_RULES` and `exactly_the_pinned_vikalpa_rules_are_optional` are **unchanged** — 8.2.74 still precedes 8.2.75 either way.

- [ ] **Step 6: Run the scoped suite**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS, with `shnams_ru_fires_on_the_dhatus_own_final` and the 7a laṅ cell tests unchanged. Any change to a √hiṃs or √kṛt form means the move was not neutral — revert and investigate rather than adjusting the expectation.

- [ ] **Step 7: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/tripadi.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "refactor(prakriya): 8.2.75 above 8.2.73, and 8.2.73's verdict

Moving 8.2.75 above 8.2.73 makes its p.log read unreachable, so the rule
now declines on its own phonology (√hiṃs presents ahinas, not a `d`)
rather than on rule history. Forms unchanged.

Also discharges 8.2.73's deferred re-verification: √bhañj and √piṣ are
the first roots after √hiṃs to empty ENDING under 8.2.23, they do so at
tip/sip as the invariant requires, and the s-final check keeps 8.2.73
off them. Verdict replaces the standing warning."
```

---

### Task 9: 7a deferred #2 and #4 — byte-identical hardening

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (8.3.24, 8.4.58)
- Modify: `crates/panini-prakriya/src/tinanta/adesha.rs` (6.1.101, 6.1.97, 6.1.87, 6.1.66, 6.4.105)
- Modify: `crates/panini-prakriya/src/tinanta/vikarana.rs` (tag the thematic vikaraṇas)
- Modify: `crates/panini-prakriya/src/term.rs` (the `Tag` enum)

**Interfaces:**
- Consumes: nothing new.
- Produces: a `Tag` marking a thematic (a-final) vikaraṇa, set by 3.1.68, 3.1.69 and 3.1.77 and read by the five `adesha.rs` rules.

**Acceptance for this whole task: the scoped suite is byte-identical.** Neither change may move a single form or trace. A moved cell is a defect, not a discovery.

- [ ] **Step 1: Capture the baseline**

```bash
mise exec -- cargo test -p panini-prakriya 2>&1 | tail -30 > /tmp/7b-task9-before.txt
```

Record the passing test count. You will compare against it in Step 6.

- [ ] **Step 2: #2 — make 8.3.24 and 8.4.58 find the first *applicable* candidate**

Both rules currently locate the first `n` (8.3.24) or `M` (8.4.58) in the word with `position(...)` and then bail if that character fails the rule's own following-sound test. The failure mode is silent under-application: a later, genuinely applicable candidate is never considered.

Move the following-sound test **into** the search, so each rule finds the first candidate that satisfies its full condition rather than the first candidate of the right character.

**This is hardening, not a fix.** All 108 new cells were traced and none distinguishes the two forms — in every one the first candidate is either the correct target or correctly declines (`Banaj` + `ti` bails on the following `a`, as it should). Record that in both comments so the generality is not later mistaken for a live requirement.

Because the condition **moves** rather than multiplies, it keeps its existing witnesses and the mutation gate is unaffected. If the gate in Task 11 nevertheless reports a survivor here, the verdict is to revert to the simpler `position` form rather than to add a test.

- [ ] **Step 3: #4 — replace the five `SHAP.ends_with('a')` proxies**

`adesha.rs` tests `p.terms[SHAP].text.ends_with('a')` in five rules — 6.1.101, 6.1.97, 6.1.87, 6.1.66 and 6.4.105 — as a stand-in for "the vikaraṇa is thematic śap". A vowel-final rudhādi root would leave `SHAP` as exactly `"na"` and all five would treat śnam as śap.

Add a `Tag` marking a thematic vikaraṇa, set where the a-final vikaraṇas are introduced — 3.1.68 śap, 3.1.69 śyan, 3.1.77 śa, in `vikarana.rs` — and replace the text test in all five rules with a test of that tag.

**Replace; do not add a clause.** An added `&& !p.terms[ANGA].has(Tag::Rudhadi)` would have no witness — gaṇa 7 holds no vowel-final root — and the mutation gate would flag all five as dead weight, exactly as it flagged `Context::is_tip` in 7a. A replacement leaves a single guard clause that adādi (empty `SHAP`), kryādi (`nA`/`nI`) and svādi (`nu`, and `n` after 6.4.107) all still witness.

**The risk this bounds**, and what Step 6 is checking for: a rule that mutates `SHAP` mid-derivation, where a persistent tag and a re-read of the text could diverge. 6.4.107 is the known such rule — it leaves `SHAP` as `"n"`. Under both the old proxy and the new tag śnu is non-thematic, so it should agree; the byte-identical run is what proves it.

- [ ] **Step 4: Run the scoped suite**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: PASS.

- [ ] **Step 5: Confirm byte-identity**

```bash
mise exec -- cargo test -p panini-prakriya 2>&1 | tail -30 > /tmp/7b-task9-after.txt
diff /tmp/7b-task9-before.txt /tmp/7b-task9-after.txt
```

Expected: no differences beyond timings. Same tests, same count, all passing. If any assertion changed, **stop and investigate** — do not update an expectation to match.

- [ ] **Step 6: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/tripadi.rs \
        crates/panini-prakriya/src/tinanta/adesha.rs \
        crates/panini-prakriya/src/tinanta/vikarana.rs \
        crates/panini-prakriya/src/term.rs
git commit -m "refactor(prakriya): retire two latent proxies, byte-identically

8.3.24 and 8.4.58 now find the first APPLICABLE candidate rather than
bailing on the first candidate of the right character. No cell in the
suite distinguishes the two; this closes a silent under-application
path before a later root set reaches it.

The five SHAP.ends_with('a') proxies in adesha.rs become a positive tag
set by the a-final vikaraṇa rules. Replacing rather than adding keeps
one witnessed clause — an added Rudhadi exclusion would have no witness
and the mutation gate would flag all five as dead weight."
```

---

### Task 10: The goldens — `PARADIGM`, `ALTERNATES` and the trace pins

**Files:**
- Modify: `crates/panini/tests/paradigm.rs` (`PARADIGM`, `ALTERNATES`, and the audited-numbers test)
- Modify: `crates/panini/tests/trace.rs`

**Interfaces:**
- Consumes: every rule from Tasks 2–7.
- Produces: a green workspace gate for the first time since Task 1.

- [ ] **Step 1: Add the twelve `PARADIGM` blocks**

Append to `PARADIGM`, after the existing `Kid` blocks, in root order `Banj`, `piz`, `inD` and lakāra order `laT`, `laN`, `loT`, `viDiliN`. Each block is `(root_id, lakara_name, [9 forms])` in cell order prathama eka/dvi/bahu, madhyama eka/dvi/bahu, uttama eka/dvi/bahu.

**Index 0 of a forked cell is the declined derivation — what the pipeline produces with no optional rule applied.** These are already so ordered; do not re-sort them.

```rust
    (
        "Banj",
        "laT",
        [
            "Banakti", "BaNktaH", "BaYjanti", "Banakzi", "BaNkTaH", "BaNkTa", "Banajmi", "BaYjvaH",
            "BaYjmaH",
        ],
    ),
    (
        "Banj",
        "laN",
        [
            "aBanag", "aBaNktAm", "aBaYjan", "aBanag", "aBaNktam", "aBaNkta", "aBanajam",
            "aBaYjva", "aBaYjma",
        ],
    ),
    (
        "Banj",
        "loT",
        [
            "Banaktu", "BaNktAm", "BaYjantu", "BaNgDi", "BaNktam", "BaNkta", "BanajAni",
            "BanajAva", "BanajAma",
        ],
    ),
    (
        "Banj",
        "viDiliN",
        [
            "BaYjyAd",
            "BaYjyAtAm",
            "BaYjyuH",
            "BaYjyAH",
            "BaYjyAtam",
            "BaYjyAta",
            "BaYjyAm",
            "BaYjyAva",
            "BaYjyAma",
        ],
    ),
    (
        "piz",
        "laT",
        [
            "pinazwi", "piMzwaH", "piMzanti", "pinakzi", "piMzWaH", "piMzWa", "pinazmi", "piMzvaH",
            "piMzmaH",
        ],
    ),
    (
        "piz",
        "laN",
        [
            "apinaq", "apiMzwAm", "apiMzan", "apinaq", "apiMzwam", "apiMzwa", "apinazam",
            "apiMzva", "apiMzma",
        ],
    ),
    (
        "piz",
        "loT",
        [
            "pinazwu", "piMzwAm", "piMzantu", "piRqQi", "piMzwam", "piMzwa", "pinazARi",
            "pinazAva", "pinazAma",
        ],
    ),
    (
        "piz",
        "viDiliN",
        [
            "piMzyAd",
            "piMzyAtAm",
            "piMzyuH",
            "piMzyAH",
            "piMzyAtam",
            "piMzyAta",
            "piMzyAm",
            "piMzyAva",
            "piMzyAma",
        ],
    ),
    (
        "inD",
        "laT",
        [
            "indDe", "inDAte", "inDate", "intse", "inDATe", "indDve", "inDe", "inDvahe", "inDmahe",
        ],
    ),
    (
        "inD",
        "laN",
        [
            "EndDa", "EnDAtAm", "EnData", "EndDAH", "EnDATAm", "EndDvam", "EnDi", "EnDvahi",
            "EnDmahi",
        ],
    ),
    (
        "inD",
        "loT",
        [
            "indDAm", "inDAtAm", "inDatAm", "intsva", "inDATAm", "indDvam", "inaDE", "inaDAvahE",
            "inaDAmahE",
        ],
    ),
    (
        "inD",
        "viDiliN",
        [
            "inDIta",
            "inDIyAtAm",
            "inDIran",
            "inDITAH",
            "inDIyATAm",
            "inDIDvam",
            "inDIya",
            "inDIvahi",
            "inDImahi",
        ],
    ),
```

- [ ] **Step 2: Add the twenty-two `ALTERNATES` rows**

Append to `ALTERNATES`. The fifth field names the optional rule(s) that produced the alternate, `+`-joined in pipeline order — `every_alternate_names_the_vikalpa_rules_that_produced_it` checks it.

```rust
    ("Banj", "laN", 0, "aBanak", "8.4.56"),
    ("Banj", "laN", 3, "aBanak", "8.4.56"),
    ("Banj", "loT", 0, "BaNktAd", "7.1.35"),
    ("Banj", "loT", 0, "BaNktAt", "7.1.35+8.4.56"),
    ("Banj", "loT", 3, "BaNktAd", "7.1.35"),
    ("Banj", "loT", 3, "BaNktAt", "7.1.35+8.4.56"),
    ("Banj", "viDiliN", 0, "BaYjyAt", "8.4.56"),
    ("piz", "laN", 0, "apinaw", "8.4.56"),
    ("piz", "laN", 3, "apinaw", "8.4.56"),
    ("piz", "loT", 0, "piMzwAd", "7.1.35"),
    ("piz", "loT", 0, "piMzwAt", "7.1.35+8.4.56"),
    ("piz", "loT", 3, "piRQi", "8.4.65"),
    ("piz", "loT", 3, "piMzwAd", "7.1.35"),
    ("piz", "loT", 3, "piMzwAt", "7.1.35+8.4.56"),
    ("piz", "viDiliN", 0, "piMzyAt", "8.4.56"),
    ("inD", "laT", 0, "inDe", "8.4.65"),
    ("inD", "laT", 5, "inDve", "8.4.65"),
    ("inD", "laN", 0, "EnDa", "8.4.65"),
    ("inD", "laN", 3, "EnDAH", "8.4.65"),
    ("inD", "laN", 5, "EnDvam", "8.4.65"),
    ("inD", "loT", 0, "inDAm", "8.4.65"),
    ("inD", "loT", 5, "inDvam", "8.4.65"),
```

√indh takes no tātaṅ and no pausal branch — 7.1.35 is parasmaipada loṭ's, and 8.4.56 needs a pada-final jhal — so all seven of its alternates are 8.4.65's.

- [ ] **Step 3: Update the audited numbers**

In `derivation_set_shape_matches_the_audited_numbers`, move the counts to **1728** cells and **213** alternate rows, i.e. **1941** forms. Update any prose in that test's comment naming the old figures.

The 22 new rows cross-check independently: 7 for √bhañj, 8 for √piṣ, 7 for √indh.

- [ ] **Step 4: Run the full gate**

Run: `mise run test`
Expected: **PASS — green for the first time since Task 1.** In particular `derivation_set_is_exactly_pinned` must pass, which is what proves there is no over- or under-generation in any of the 108 new cells.

- [ ] **Step 5: Add the trace pins**

Add to `crates/panini/tests/trace.rs`, one per new mechanism. Follow the file's existing pin idiom.

| pin | asserts |
| --- | --- |
| `apinaq` | 8.2.23 fires **above** 8.2.41 — the load-bearing order |
| `Banakti` | 8.2.30 then 8.4.55 |
| `indDe` | 8.2.40 then 8.4.53 |
| `pinakzi` | 8.2.41 then 8.3.59 |
| `piMzwaH` | 8.3.24 fires and 8.4.58 **declines** — the round trip's second witness |
| `piRQi` | the full 8.4.41 → 8.4.53 → 8.4.58 → 8.4.65 chain |

- [ ] **Step 6: Run the full gate again**

Run: `mise run test`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add crates/panini/tests/paradigm.rs crates/panini/tests/trace.rs
git commit -m "test: pin the rudhādi 7b paradigm, alternates and rule order

108 cells and 22 alternates: PARADIGM 1620 -> 1728, ALTERNATES 191 ->
213, 1811 -> 1941 forms. Index 0 of every forked cell is the declined
derivation.

Six trace pins, one per new mechanism. apinaq is the one that matters
most: it pins 8.2.23 above 8.2.41, the only load-bearing new order in
the slice, and the reversal produces a real-looking apinak that no
guard test would catch."
```

---

### Task 11: Documentation, the guard-test convention, and the gates

**Files:**
- Modify: `AGENTS.md`
- Modify: `README.md`
- Modify: `docs/ARCHITECTURE.md`
- Modify: `data/ATTRIBUTION.md`

**Interfaces:**
- Consumes: everything.
- Produces: a merge-ready branch.

- [ ] **Step 1: Update every count**

7a needed a post-merge fix wave for exactly this, so treat it as a checklist, not a skim. Grep for each old figure before editing and again after:

```bash
grep -rn "1620\|1811\|154 rows\|191 rows\|45 roots\|42 roots" \
  AGENTS.md README.md docs/ARCHITECTURE.md data/ATTRIBUTION.md
```

New values: **1728** cells, **213** `ALTERNATES` rows, **1941** forms, **48** roots.

- [ ] **Step 2: Update the rudhādi prose in `AGENTS.md`**

- The gaṇa now holds **six** roots. It is still **partial**, and the reason is unchanged: nine of its 25 roots are ubhayapadī and 1.3.72 is deferred, so √rudh, √bhid, √chid and √yuj are absent and the gaṇa lacks its eponym. Replace the sentence claiming "7b (√bhañj, √piṣ, √indh) closes what is reachable" — it does not. Nine reachable roots remain (√śiṣ, √tṛh, √und, √añj, √tañc, √vij, √vṛj, √pṛc, √vid); six is simply the size every completed gaṇa here has, and those nine bring machinery of their own (7.1.58 for √und, 6.4.24 for √añj and √tañc, two id collisions).
- Record that √indh's pada was verified against vidyut-prakriya rather than inferred from its ñi.
- The vikalpa set is still **seven** rules. Say so explicitly — this is the first gaṇa slice that added none.
- Note the new load-bearing order (8.2.41 below 8.2.23) alongside the two already documented, and note that 8.4.41 above 8.4.53 is **not** load-bearing, so it is not mistaken for one.
- Update the 8.2.7x ordering description for Task 8's move: the order is now 8.2.74, 8.2.75, 8.2.73.

- [ ] **Step 3: Amend the guard-test convention (deferred #5)**

`AGENTS.md` says "Per-rule guard tests go beside the rule in its stage file", and the plan convention has been read as requiring one for every rule. 7a recorded that this is unachievable for tripādī rules and deferred the question.

That framing was too strong: `tripadi.rs` already carries thirteen guard tests, including `jhalam_jasho_ante_fires_only_on_a_pada_final_t` and `va_avasane_fires_only_on_a_pada_final_jhal`. What is genuinely unachievable is a guard test for a rule whose precondition only an upstream rule chain can produce.

Amend the convention to say exactly that:

> Write a per-rule guard test where the rule's precondition can be built
> directly on a hand-built `Prakriya`. Where it cannot — because only an
> upstream rule chain produces that state — cite the covering derivation
> or trace test in the rule's own comment instead.

- [ ] **Step 4: Update `docs/ARCHITECTURE.md`'s branch-count paragraph**

√piṣ's loṭ madhyama eka holds **four** forms (`piRqQi`, `piRQi`, `piMzwAd`, `piMzwAt`), stacking 8.4.65, 7.1.35 and 8.4.56. That is the deepest cell this slice adds and it is shallower than 7a's six-form √kṛt cell, which remains the suite's deepest. Say so, and note that √indh's cells fork on 8.4.65 alone.

- [ ] **Step 5: Run the cross-implementation audit**

Extend the vidyut-prakriya harness to all 48 roots and compare derivation **sets**, not just index 0 — that comparison is what caught 8.2.74's ordering in 7a. The 7b probe written during design (`rudhadi_7b_probe.rs`, including the `~^r` control that settled √indh's pada) is its seed.

Expected: exact agreement on all 1941 forms. Any divergence is a finding to adjudicate against the sūtras before it is a bug to fix — 7a found one case where this engine was right and the naive reading was not.

- [ ] **Step 6: Run the mutation gate**

The suite grew 6.7%, so the floor a **full uncaught** mutant run has to clear rose with it. Re-derive the cap against that floor, not against a caught-and-aborted run: under too small a cap a real survivor is recorded as a timeout and a "0 missed" report is vacuous. This bit slice 7a exactly.

```bash
MISE_ENV=dev mise install
# run the cargo-mutants binary directly — the mise shim fails in background shells
```

Check **both** `missed.txt` and `timeout.txt`.

Expected: zero survivors, and exactly **one** timeout — `tripadi.rs`'s ṇatva backward scan, where `j -= 1` mutates to `j /= 1` and the loop never terminates. That is a permanent, correct verdict, not a symptom of too short a cap; do not chase it with a bigger timeout or a code change.

Any survivor in a guard added by this slice means that arm has no witness: **shrink the guard**, do not grow the test. That is how 7a's `Context::is_tip` was found and deleted.

- [ ] **Step 7: Run the full gate and the lint/format/audit tasks**

```bash
mise run fmt
mise run lint
mise run test
mise run audit
```

Expected: all pass.

- [ ] **Step 8: Commit**

```bash
git add AGENTS.md README.md docs/ARCHITECTURE.md data/ATTRIBUTION.md
git commit -m "docs: rudhādi 7b — six roots, and the counts that move with them

1728 cells, 213 alternates, 1941 forms, 48 roots.

Corrects the claim that 7b 'closes what is reachable': nine reachable
rudhādi roots remain, and six is simply the size every completed gaṇa
here has. The gaṇa is still partial for the same reason as before —
1.3.72, not the root count.

Also amends the per-rule guard-test convention (7a deferred #5): write
one where the precondition can be built directly, otherwise cite the
covering derivation or trace test."
```

---

## Notes for the reviewer

**What this slice does not do.** It does not close rudhādi. Nine reachable roots remain, and the gaṇa still lacks its eponym behind 1.3.72. 7a's spec said 7b "closes the gaṇa"; that was optimistic and Task 11 corrects it in `AGENTS.md` rather than leaving the claim standing.

**Three predictions from 7a that did not survive contact**, each corrected in place rather than silently:

1. **6.4.24 is not needed.** The nasal that drops in all three roots sits immediately behind śnam's `na` and is 6.4.23's. 6.4.24 governs √añj and √tañc, both out of scope. (Task 2, Step 6.)
2. **6.4.23 needs no widening** — its guard is already `rest.starts_with('n')`. Its comment also miscopies √bhañj's tail as `fj` for `nj`. (Task 2, Step 6.)
3. **The `aS.5` id-qualification mechanism stays at one user**, because √vid is out of scope. (Task 1, Step 1.)

**The one order that matters.** 8.2.41 below 8.2.23. Reversed, laṅ madhyama eka surfaces `apinak` instead of `apinaq`/`apinaw` — a real-looking form that splits madhyama eka from prathama eka, and that no guard test would flag. It is pinned twice, by a derivation test (Task 4) and a trace pin (Task 10). By contrast 8.4.41 above 8.4.53 is **not** load-bearing; both tasks say so, to stop it being built on.

**Two tasks must not change behaviour.** Task 9's acceptance is a byte-identical suite. If a form or trace moves there, the change was not the hardening it was meant to be — stop and investigate rather than updating the expectation.

**The red window.** The workspace gate is red from Task 1 to Task 10 by design, because `paradigm_covers_every_enumerable_cell` demands a block per root. The scoped `-p panini-prakriya` command is the working gate throughout, and it must be green at the end of every task. Do not paper over the red with partial `PARADIGM` blocks.

**Ground truth.** Every form in Task 10 was derived from vidyut-prakriya during design and cross-checked three ways: 1620 + 108 = 1728, 191 + 22 = 213, and the per-root alternate counts (7 + 8 + 7 = 22) sum independently of the form tables.
