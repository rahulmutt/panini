# rudhādi gaṇa slice 7f — √chid, √chṛd, and the two sūtras behind them — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Curate √chid (`07.0003`) and √chṛd (`07.0008`), the last two ubhayapadī rudhādi roots, taking the gaṇa from 22 to 24 of its 25 dhātupāṭha roots and leaving only √bhuj out.

**Architecture:** Two new sūtras, in two groups with a hard gate between them. First the **support layer** — three sound-table entries and a whole-word addressing helper promoted out of `tripadi.rs` — then the **two rules**, 6.1.73 *che ca* in `anga.rs` and 8.4.40 *stoḥ ścunā ścuḥ* in `tripadi.rs`, verified inert against the *existing* 2628-cell corpus by a byte-for-byte dump diff before a single new row lands. Then the two data rows and goldens generated from the engine the cross-implementation audit certified. Because the rules are proven inert first, a difference the audit finds after the roots arrive has exactly one candidate cause.

**Tech Stack:** Rust 1.98.0 pinned via `mise`; `cargo test` for the golden suite; `cargo-mutants` for the mutation gate; `vidyut-prakriya` (external, via the committed harness at `tools/audit/`) for the cross-implementation audit.

**Spec:** `docs/superpowers/specs/2026-08-25-rudhadi-gana-7f-design.md`

**Branch:** `rudhadi-gana-7f` already exists, with the spec committed as `b617738`. Work continues on it; do not branch again. `main` is at `ef97c25`, unchanged.

## Global Constraints

- **Toolchain is pinned to rust 1.98.0** via `mise.toml`. Use `mise run <task>` or `mise exec -- cargo …`; never install Rust globally. `mise run test -- -p X` does **not** scope to a package — use `mise exec -- cargo test -p X`.
- **`mise run lint` is `cargo clippy --workspace --all-targets -- -D warnings`, and `dead_code` is a warning.** A `pub(crate)` item whose only consumers are `#[cfg(test)]` still fails the plain lib build. This is why Task 2 lands the three sound-table entries, `insert_char` and the two rules that read them **in one task**: run `lint` only at the end of a task, never between adding a predicate and adding its rule.
- **Run the golden suite in the FOREGROUND.** It takes ~16 minutes at the current corpus size and grows in Task 6. Do not background it and do not end a turn while it runs; a backgrounded suite gets orphaned and its result is lost.
- **`mise run mutants` is `-j 4 --timeout 4800`.** Run the task; do not reconstruct the flags. If the mise shim errors with `no version set for shim: cargo-mutants`, invoke the `cargo-mutants` binary directly with those same flags. `cargo-mutants` also reads `-j` from `CARGO_MUTANTS_JOBS`, so the environment can defeat the cap.
- **Goldens are generated, never hand-authored.** Every `PARADIGM` block and `ALTERNATES` row comes out of the throwaway generator in Task 6, run against the engine the audit certified. Surfaces quoted in this document exist to make a wrong result recognisable; they are **not** to be typed into a test.
- **The audit's negative controls run first.** A zero-difference result recorded without a verified-failing control proves nothing.
- **`tools/audit/panini_full_audit.rs` is copied, never rewritten.** It is committed precisely so no slice reconstructs it.
- **SLP1 throughout.** `C` is the aspirate palatal ch, `c` its unaspirate, `S` palatal ś, `z` retroflex ṣ, `Y` palatal ñ, `R` retroflex ṇ, `J` aspirate palatal jh, `f` vocalic ṛ, `x` vocalic ḷ.
- **`C` and `S` are different sounds and this slice turns on the difference.** `C` is ch (an aspirate stop, and a *ścu* by varga); `S` is ś (a sibilant, also *ścu*); `z` is ṣ (a *ṣṭu*, 8.4.41's business, not this slice's). A guard that conflates any two of them will pass its own unit test and break a different gaṇa.

## Numbers this slice changes

Old values, for the arithmetic in Tasks 4, 5 and 6. Every one is asserted somewhere and will fail loudly if missed.

| quantity | old | new |
|---|---|---|
| `dhatus().len()` | 64 | **66** |
| `PARADIGM.len()` (blocks) | 292 | **308** |
| cells (`PARADIGM.len() * 9`) | 2628 | **2772** |
| `ALTERNATES.len()` | 429 | **487** |
| forms (cells + alternates) | 3057 | **3259** |
| rudhādi curated roots | 22 | **24** |
| rudhādi entries still out | 3 | **1** |
| ubhayapadī curated roots | 9 | **11** |
| `TINANTA_RULES` (flattened) | 90 | **92** |
| pada-collision surfaces | 22 | **26** |

Cell-multiplicity buckets in `derivation_set_shape_matches_the_audited_numbers`, old → new: ones **2324 → 2426**, twos **211 → 247**, threes **79 → 81**, fours **2 → 2 (unchanged)**, fives **6 → 8**, sixes **6 → 8**.

`ALTERNATES` key counts, old → new: `8.4.65` **111 → 145**, `8.4.56` **105 → 111**, `7.1.35` **86 → 90**, `7.1.35+8.4.56` **86 → 90**, `7.1.35+8.4.65` **12 → 16**, `7.1.35+8.4.65+8.4.56` **12 → 16**, `8.2.75` **6 → 8**, `6.4.107` **8 (unchanged)**, `3.4.111` **2 (unchanged)**, `8.2.74` **1 (unchanged)**.

Those "new" columns are **expectations to recognise a wrong result by**, not values to type in ahead of measurement. They are unusually well-founded for once: √chid and √chṛd are shape-identical to √bhid (`07.0002`) and √tṛd (`07.0009`), and both of those contribute exactly 29 `ALTERNATES` rows with an identical key profile — `8.4.65` ×17, `8.4.56` ×3, `7.1.35` ×2, `7.1.35+8.4.56` ×2, `7.1.35+8.4.65` ×2, `7.1.35+8.4.65+8.4.56` ×2, `8.2.75` ×1. Tasks 5 and 6 still measure rather than assume.

## What √chid's and √chṛd's paradigms look like

Probed against vidyut-prakriya at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` during design. Both ubhayapadī, so 72 cells each, not 36. **Do not type these into a test**; they exist so a wrong generator output is recognisable at a glance.

√chid (`07.0003`), parasmaipada:

| lakāra | the nine cells |
|---|---|
| laṭ | `Cinatti` `CintaH`/`CinttaH` `Cindanti` `Cinatsi` `CinTaH`/`CintTaH` `CinTa`/`CintTa` `Cinadmi` `CindvaH` `CindmaH` |
| laṅ | `acCinat`/`acCinad` `acCintAm`/`acCinttAm` `acCindan` `acCinaH`/`acCinat`/`acCinad` `acCintam`/`acCinttam` `acCinta`/`acCintta` `acCinadam` `acCindva` `acCindma` |
| loṭ | `Cinattu`/`CintAt`/`CinttAt`/`CintAd`/`CinttAd` `CintAm`/`CinttAm` `Cindantu` `CinDi`/`CindDi`/`CintAt`/`CinttAt`/`CintAd`/`CinttAd` `Cintam`/`Cinttam` `Cinta`/`Cintta` `CinadAni` `CinadAva` `CinadAma` |
| vidhiliṅ | `CindyAt`/`CindyAd` `CindyAtAm` `CindyuH` `CindyAH` `CindyAtam` `CindyAta` `CindyAm` `CindyAva` `CindyAma` |

√chid, ātmanepada:

| lakāra | the nine cells |
|---|---|
| laṭ | `Cinte`/`Cintte` `CindAte` `Cindate` `Cintse` `CindATe` `CinDve`/`CindDve` `Cinde` `Cindvahe` `Cindmahe` |
| laṅ | `acCinta`/`acCintta` `acCindAtAm` `acCindata` `acCinTAH`/`acCintTAH` `acCindATAm` `acCinDvam`/`acCindDvam` `acCindi` `acCindvahi` `acCindmahi` |
| loṭ | `CintAm`/`CinttAm` `CindAtAm` `CindatAm` `Cintsva` `CindATAm` `CinDvam`/`CindDvam` `CinadE` `CinadAvahE` `CinadAmahE` |
| vidhiliṅ | `CindIta` `CindIyAtAm` `CindIran` `CindITAH` `CindIyATAm` `CindIDvam` `CindIya` `CindIvahi` `CindImahi` |

√chṛd (`07.0008`) is the same table with `Cf` for `Ci` and 8.4.1's ṇatva applied — `CfRatti`, `acCfRat`, `CfRattu`, `CfndyAt`, `Cfnte`, `acCfnta`, and so on. Both roots: 72 cells, 101 forms, **29 new `ALTERNATES` rows apiece**, distributed 51 one-form cells, 18 two-form, 1 three-form, 1 five-form, 1 six-form.

## File Structure

| file | responsibility | task |
|---|---|---|
| `crates/panini-prakriya/src/tinanta/terms.rs` | `word_chars`, `set_char`, `remove_char` moved in | 1 |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | the three moved helpers deleted and re-imported | 1 |
| `crates/panini-prakriya/src/tinanta/sound.rs` | `is_hrasva`, `is_shcu`, `shcutva_of`, and their all-arms unit tests | 2 |
| `crates/panini-prakriya/src/tinanta/terms.rs` | the new `insert_char` | 2 |
| `crates/panini-prakriya/src/tinanta/anga.rs` | the new 6.1.73 `Rule` and its guard test | 2 |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | the new 8.4.40 `Rule` and its guard test | 2 |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | `tinanta_rule_order_is_pinned` | 2 |
| `crates/panini-data/src/lib.rs` | two `Dhatu` rows; `rudhadi_rows_…` renamed and extended; `dhatus().len()`; the prose counts | 4 |
| `crates/panini/tests/paradigm.rs` | `GATED`, then `PARADIGM`, `ALTERNATES`, the audited-numbers test, its doc comment, and the pada-collision vec | 4, 6 |
| `tools/audit/panini_full_audit.rs`, `tools/audit/README.md` | corpus totals; "Last recorded result" | 5 |
| `crates/panini/tests/trace.rs` | the four new pins | 7 |
| `AGENTS.md` | the mutation paragraph | 8 |
| `README.md`, `docs/ARCHITECTURE.md`, `AGENTS.md` | prose, counts, recorded results | 9 |

---

### Task 1: Whole-word addressing leaves the tripādī

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/terms.rs` (add the three helpers at the end)
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs:17-40` (delete the three helpers) and its `use` block

**Interfaces:**
- Consumes: nothing.
- Produces: `word_chars(&Prakriya) -> Vec<(usize, usize, char)>`, `set_char(&mut Prakriya, usize, usize, char)` and `remove_char(&mut Prakriya, usize, usize)` in `crate::tinanta::terms`. Task 2's two rules consume `word_chars` and `set_char`.

**A pure move, and nothing else.** No new item, no `Rule`, no behaviour change. It is its own task precisely so the green suite at the end of it certifies that the move was mechanical — twenty-odd call sites in `tripadi.rs` change how they resolve, and a mistake there is far easier to see against an otherwise-empty diff than alongside two new sūtras.

`insert_char` and the three sound-table entries deliberately do **not** land here: they would have no non-test consumer until Task 2's rules exist, and `mise run lint` fails a `pub(crate)` item that only `#[cfg(test)]` code uses.

- [ ] **Step 1: Move the three whole-word helpers into `terms.rs`**

Delete `word_chars`, `set_char` and `remove_char` from `crates/panini-prakriya/src/tinanta/tripadi.rs` (currently `:17-40`, the three functions and their doc comments, immediately below the `use` block).

Add them to `crates/panini-prakriya/src/tinanta/terms.rs`, at the end of the file, together with the new `insert_char`:

```rust
/// The assembled word as `(term index, char index, char)`, so a rule can
/// reason over the whole pada and still write back into the right term.
///
/// Lived in `tripadi.rs` until slice 7f, when 6.1.73 Ce ca — an aṅga-stage
/// rule with a saṁhitā condition — became the first rule outside the
/// tripādī to need whole-word addressing.
pub(crate) fn word_chars(p: &Prakriya) -> Vec<(usize, usize, char)> {
    let mut out = Vec::new();
    for (ti, t) in p.terms.iter().enumerate() {
        for (ci, c) in t.text.chars().enumerate() {
            out.push((ti, ci, c));
        }
    }
    out
}

/// Replace one character of one term, addressed as `word_chars` reports it.
pub(crate) fn set_char(p: &mut Prakriya, term: usize, idx: usize, to: char) {
    let mut s: Vec<char> = p.terms[term].text.chars().collect();
    s[idx] = to;
    p.terms[term].text = s.into_iter().collect();
}

/// Delete one character of one term, addressed as `word_chars` reports it.
/// Companion to `set_char`, for the rules that elide rather than substitute.
pub(crate) fn remove_char(p: &mut Prakriya, term: usize, idx: usize) {
    let mut s: Vec<char> = p.terms[term].text.chars().collect();
    s.remove(idx);
    p.terms[term].text = s.into_iter().collect();
}
```

- [ ] **Step 2: Re-import them in `tripadi.rs`**

Change `crates/panini-prakriya/src/tinanta/tripadi.rs`'s `terms` import from:

```rust
use crate::tinanta::terms::{ANGA, ENDING, SHAP};
```

to:

```rust
use crate::tinanta::terms::{ANGA, ENDING, SHAP, remove_char, set_char, word_chars};
```

Every call site in `tripadi.rs` stays exactly as it is — this is an import change, not a rewrite. Do not touch any rule's body in this step.

- [ ] **Step 3: Run the prakriyā crate's own tests**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: all pass. Then `mise run fmt-check && mise run lint`.

If `fmt-check` reorders the import list, take its ordering — do not fight rustfmt over it.

- [ ] **Step 4: Run the full golden suite in the foreground**

Run: `mise run test`
Expected: all pass, **unchanged**. This task adds no rule, so a golden that moves here means the move was not mechanical — revert and redo it as a pure move.

- [ ] **Step 5: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/terms.rs crates/panini-prakriya/src/tinanta/tripadi.rs
git commit -m "refactor(terms): whole-word addressing leaves tripadi

word_chars, set_char and remove_char move to terms.rs, the term-layout
support layer. 6.1.73 Ce ca is an anga-stage rule with a samhita
condition -- the first rule outside the tripadi to need whole-word
addressing -- and it lands in the next commit.

A pure move: no new item, no behaviour change, goldens unmoved."
```

---

### Task 2: The sound table, and 6.1.73 with 8.4.40

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/sound.rs` (`is_hrasva`, `is_shcu`, `shcutva_of` and their unit tests)
- Modify: `crates/panini-prakriya/src/tinanta/terms.rs` (`insert_char`)
- Modify: `crates/panini-prakriya/src/tinanta/anga.rs` (the new `Rule` after 6.4.72; its guard test; the `use` block)
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (the new `Rule` above 8.4.41; its guard test; the `sound` import)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs:121-133` (`tinanta_rule_order_is_pinned`)

**Interfaces:**
- Consumes: Task 1's `word_chars` and `set_char`.
- Produces: `is_hrasva(char) -> bool`, `is_shcu(char) -> bool`, `shcutva_of(char) -> Option<char>` in `crate::tinanta::sound`; `insert_char(&mut Prakriya, usize, usize, char)` in `crate::tinanta::terms`; and the two rules whose inertness Task 3 gates and whose correctness Task 5 certifies.

**Everything in this task lands together, on purpose.** `mise run lint` is `-D warnings` and `dead_code` is one, so a predicate whose only consumer is a `#[cfg(test)]` block fails the plain lib build. Run `mise run lint` at Step 11 and not before — the intermediate states between Steps 3 and 9 are expected to warn.

- [ ] **Step 1: Write the failing sound-table tests**

In `crates/panini-prakriya/src/tinanta/sound.rs`, inside `mod tests`, after `parasavarna_of_stops_all_arms`:

```rust
    #[test]
    fn shcutva_of_stu_all_arms() {
        // 8.4.40 stoH ScunA ScuH: pin every arm of the stu -> Scu
        // substitution table directly. Only `t -> c` is reachable from any
        // golden -- the tuk 6.1.73 inserts before √chid's and √chṛd's
        // initial `C` -- so without this test a mutant rewriting any of the
        // other five arms would be invisible to the whole suite.
        assert_eq!(shcutva_of('s'), Some('S'));
        assert_eq!(shcutva_of('t'), Some('c'));
        assert_eq!(shcutva_of('T'), Some('C'));
        assert_eq!(shcutva_of('d'), Some('j'));
        assert_eq!(shcutva_of('D'), Some('J'));
        assert_eq!(shcutva_of('n'), Some('Y'));
        // Already Scu, so not stu. `None` here is what lets 8.4.40 use this
        // one lookup as its match test as well as its substitute.
        for c in ['S', 'c', 'C', 'j', 'J', 'Y'] {
            assert_eq!(shcutva_of(c), None, "{c} is Scu, not stu");
        }
        // Not stu at all: a velar, and the retroflex sibilant that belongs
        // to 8.4.41 rather than to this rule.
        assert_eq!(shcutva_of('k'), None);
        assert_eq!(shcutva_of('z'), None);
    }

    #[test]
    fn is_shcu_and_is_hrasva_membership() {
        // 8.4.40's trigger class: `S` plus the whole c-varga, and nothing
        // else. `z` is the one that must NOT be in it -- that is 8.4.41's
        // trigger, and conflating the two would put stutva and Scutva in
        // contention on every cell either reaches.
        for c in ['S', 'c', 'C', 'j', 'J', 'Y'] {
            assert!(is_shcu(c), "{c} is Scu");
        }
        for c in ['z', 'w', 'W', 'q', 'Q', 'R', 's', 't', 'k'] {
            assert!(!is_shcu(c), "{c} is not Scu");
        }
        // 6.1.73's conditioning class: the short vowels only. `F` and `X`
        // are the long vocalic r and l, and e/o/E/O are long by 1.2.27
        // having no short counterpart at all.
        for c in ['a', 'i', 'u', 'f', 'x'] {
            assert!(is_hrasva(c), "{c} is hrasva");
        }
        for c in ['A', 'I', 'U', 'F', 'X', 'e', 'o', 'E', 'O', 't'] {
            assert!(!is_hrasva(c), "{c} is not hrasva");
        }
    }
```

- [ ] **Step 2: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya shcutva_of_stu_all_arms is_shcu_and_is_hrasva_membership`
Expected: FAIL to compile — `cannot find function shcutva_of in this scope`, and the same for `is_shcu` and `is_hrasva`.

- [ ] **Step 3: Add the three sound-table entries**

In `crates/panini-prakriya/src/tinanta/sound.rs`, add `is_hrasva` immediately after `is_vowel`:

```rust
/// The short (*hrasva*) vowels — 6.1.73 Ce ca's conditioning class.
///
/// `f` and `x` are the vocalic ṛ and ḷ; their long counterparts `F` and `X`
/// are deliberately absent, as are `e o E O`, which 1.2.27 ūkālo'j
/// hrasvadīrghaplutaḥ makes long by having no short counterpart at all.
pub(crate) fn is_hrasva(c: char) -> bool {
    matches!(c, 'a' | 'i' | 'u' | 'f' | 'x')
}
```

Add `is_shcu` immediately after `is_shtu`, so the two conditioning classes sit together:

```rust
/// 8.4.40's conditioning class — *ścunā*, the ś-and-cu the sūtra names on
/// its trigger side: `S` (ś) plus the whole c-varga.
///
/// Deliberately disjoint from `is_shtu`, which is 8.4.41's `z`-and-ṭu. The
/// two sūtras sit adjacent in the tripādī and both operate on stu; keeping
/// their trigger classes disjoint is what stops them contending.
pub(crate) fn is_shcu(c: char) -> bool {
    matches!(c, 'S' | 'c' | 'C' | 'j' | 'J' | 'Y')
}
```

Add `shcutva_of` immediately after `kutva_of`:

```rust
/// The *ścu* (palatal) counterpart of a *stu* sound — 8.4.40 stoH ScunA
/// ScuH's substitute. *stu* is `s` plus the whole t-varga, and by 1.1.50
/// sthAne'ntaratamaH the nearest substitute preserves voicing, aspiration
/// and nasality, so `t` goes to `c` and `n` to `Y`, never both to one
/// letter.
///
/// Only `t -> c` has a witness: the tuk 6.1.73 Ce ca inserts before √chid's
/// and √chṛd's initial `C`. The other five arms are present because the
/// table covers every stu arm — the same reason `parasavarna_of` carries
/// all five vargas — and `shcutva_of_stu_all_arms` is what keeps them from
/// rotting.
///
/// The palatals are deliberately absent rather than mapped to themselves:
/// they are already ścu, not stu, and `None` is what lets 8.4.40 use this
/// single lookup as its match test as well as its substitute.
pub(crate) fn shcutva_of(c: char) -> Option<char> {
    Some(match c {
        's' => 'S',
        't' => 'c',
        'T' => 'C',
        'd' => 'j',
        'D' => 'J',
        'n' => 'Y',
        _ => return None,
    })
}
```

- [ ] **Step 4: Run them to verify they pass**

Run: `mise exec -- cargo test -p panini-prakriya shcutva_of_stu_all_arms is_shcu_and_is_hrasva_membership`
Expected: 2 passed. `cargo test` warns about the three unused functions; that is expected and Step 9 resolves it. Do not run `mise run lint` yet.

- [ ] **Step 5: Write the failing rule guard tests**

In `crates/panini-prakriya/src/tinanta/anga.rs`, inside `mod tests`:

```rust
    #[test]
    fn che_ca_inserts_tuk_only_after_a_short_vowel() {
        let rule = rules().find(|r| r.id == "6.1.73").unwrap();

        // The one site this corpus reaches: 6.4.71's aṭ before a C-initial
        // aṅga. The `t` lands inside ANGA, after the augment's own `a`,
        // because 6.4.71 models the augment as a text prefix rather than as
        // its own term.
        let mut p = Prakriya {
            terms: vec![Term::new("aCi"), Term::new("nad"), Term::new("t")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "atCi");
        assert_eq!(p.text(), "atCinadt");

        // Word-initial `C`: nothing precedes it, so there is no short vowel
        // to attach to. This is every laṭ, loṭ and vidhiliṅ cell of √chid
        // and √chṛd, and it is why the two new sūtras are laṅ-only.
        let mut p = Prakriya {
            terms: vec![Term::new("Ci"), Term::new("nad"), Term::new("ti")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        // A long vowel before the `C`: *hrasva* is the sūtra's own
        // condition and a dīrgha does not satisfy it.
        let mut p = Prakriya {
            terms: vec![Term::new("ACi"), Term::new("nad"), Term::new("t")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        // Across a term boundary — the whole-word scan's reason for being.
        // No curated root presents this shape today; the scan states
        // 6.1.73's saṁhitā condition rather than the one site that happens
        // to reach it.
        let mut p = Prakriya {
            terms: vec![Term::new("a"), Term::new("Cid")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "atCid");
    }
```

In `crates/panini-prakriya/src/tinanta/tripadi.rs`, inside `mod tests`:

```rust
    #[test]
    fn shcutva_fires_on_stu_before_shcu_and_declines_after_sha() {
        let rule = rules().find(|r| r.id == "8.4.40").unwrap();

        // √chid laṅ prathama eka, after 6.1.73 has inserted the tuk: the
        // `t` is a stu, the `C` a ścu, so the `t` takes its palatal.
        let mut p = Prakriya {
            terms: vec![Term::new("atCi"), Term::new("nad")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "acCinad");

        // 8.4.44 SAt: a stu FOLLOWING a `S` is exempt, and this engine
        // implements that exemption by not implementing the direction at
        // all. Fire here and √kliś surfaces *kliSYAti -- 41 invocations of
        // 8.4.44 on that one root in vidyut-prakriya over this corpus.
        let mut p = Prakriya {
            terms: vec![Term::new("kliS"), Term::new("nA"), Term::new("ti")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "kliSnAti");

        // Not a ścu after it: `z` is 8.4.41's trigger, not this rule's, and
        // this is √piṣ's laṭ prathama eka mid-derivation.
        let mut p = Prakriya {
            terms: vec![Term::new("pina"), Term::new("zwi")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        // Not a stu before it: a velar is neither `s` nor t-varga, so
        // `shcutva_of` returns None and the scan moves on.
        let mut p = Prakriya {
            terms: vec![Term::new("ak"), Term::new("Ci")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
    }
```

- [ ] **Step 6: Run them to verify they fail**

Run: `mise exec -- cargo test -p panini-prakriya che_ca_inserts_tuk shcutva_fires_on_stu`
Expected: FAIL — both panic at `rules().find(...).unwrap()`, since neither `"6.1.73"` nor `"8.4.40"` is in `TINANTA_RULES` yet.

- [ ] **Step 7: Add `insert_char` to `terms.rs`**

At the end of `crates/panini-prakriya/src/tinanta/terms.rs`, beside the three helpers Task 1 moved in:

```rust
/// Insert one character into a term, before the character `word_chars`
/// reports at `idx`. Companion to `set_char` and `remove_char`, for the
/// rules that augment rather than substitute or elide.
///
/// `idx == term.text.chars().count()` appends, which is exactly what a kit
/// āgama attaching after a term's last character needs (1.1.46 ādyantau
/// ṭakitau).
pub(crate) fn insert_char(p: &mut Prakriya, term: usize, idx: usize, c: char) {
    let mut s: Vec<char> = p.terms[term].text.chars().collect();
    s.insert(idx, c);
    p.terms[term].text = s.into_iter().collect();
}
```

- [ ] **Step 8: Add 6.1.73 to `anga.rs`**

First extend the `use` block:

```rust
use crate::tinanta::sound::{is_hrasva, is_vowel};
use crate::tinanta::terms::{ANGA, ENDING, SHAP, insert_char, word_chars};
```

Then insert the rule into `ANGA_RULES` immediately after 6.4.72's closing `},` and before the `// 7.3.100 adaH sarvezAm:` comment:

```rust
    // 6.1.73 Ce ca: a short vowel before `C` takes the tuk-āgama — a `t`,
    // placed AFTER the vowel by 1.1.46 ādyantau ṭakitau, since tuk is kit.
    // aCid → atCid, which 8.4.40 stoH ScunA ScuH then carries to acCid.
    //
    // Immediately below 6.4.71, which manufactures the whole of its
    // precondition: the only short vowel any curated root presents before a
    // `C` is the aṭ-āgama laṅ prefixes onto a C-initial aṅga. Outside laṅ
    // the `C` is word-initial and this rule has nothing to sit after, which
    // is why √chid's and √chṛd's laṭ, loṭ and vidhiliṅ cells never take it.
    //
    // WHOLE-WORD, not ANGA-local, and deliberately. 6.1.73's condition is a
    // saṁhitā condition; the aṭ-plus-root site is where this corpus happens
    // to present one, not what the sūtra says. An ANGA-local scan would need
    // a NARROW GUARD comment arguing that a `C` can only ever be
    // root-initial — true today, and the shape of argument that has twice
    // cost this repo a real defect (8.2.39's three-literal guard, 8.4.41's
    // `z`-only trigger).
    //
    // The tuk lands INSIDE `ANGA`, because 6.4.71 models the aṭ as a text
    // prefix on the aṅga rather than as its own term. ANGA's first character
    // stays `a` and its penult stays `C`, so 6.4.72's `is_vowel(first)`
    // guard and every upadhā read below this point are unmoved.
    //
    // 6.1.76 padāntād vā, which makes the tuk OPTIONAL after a PADA-final
    // short vowel, is deliberately absent rather than overlooked: the aṭ is
    // word-internal here, so no site in this corpus is pada-final and the
    // augment is obligatory. Implement it when an upasarga or a preceding
    // pada enters scope — and note it would be this engine's eighth vikalpa
    // rule, so `exactly_the_pinned_vikalpa_rules_are_optional` must change
    // with it.
    Rule {
        id: "6.1.73",
        name: "Ce ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let w = word_chars(p);
            let Some(pos) = (1..w.len()).find(|i| w[*i].2 == 'C' && is_hrasva(w[i - 1].2)) else {
                return false;
            };
            let (term, idx, _) = w[pos - 1];
            let before = p.snapshot();
            insert_char(p, term, idx + 1, 't');
            p.record("6.1.73", "Ce ca", before);
            true
        },
    },
```

- [ ] **Step 9: Add 8.4.40 to `tripadi.rs`**

First extend the `sound` import to carry the two new items:

```rust
use crate::tinanta::sound::{
    cartva_of, is_jhal, is_jhash, is_khar, is_natva_intervener, is_natva_trigger, is_savarna,
    is_shcu, is_shtu, is_vowel, jashtva_of, kutva_of, parasavarna_of, shcutva_of,
};
```

Then insert the rule into `TRIPADI` immediately above the `// 8.4.41 ṣṭunā ṣṭuḥ` comment block:

```rust
    // 8.4.40 stoH ScunA ScuH: a stu (`s` and the t-varga) in contact with a
    // ścu (`S` and the c-varga) takes its own ścu counterpart.
    // atCinad → acCinad; atCfRad → acCfRad.
    //
    // SŪTRA ORDER, immediately above 8.4.41 — and for once in this file
    // that is ALL it is, because the two rules cannot contend. 8.4.41's
    // trigger is the ṣṭu class and `C` is not in it; this rule's trigger is
    // the ścu class and no ṣṭu sound is in that. Neither reads what the
    // other writes on any reachable input.
    //
    // ONE DIRECTION ONLY — stu before ścu, never ścu before stu — and that
    // is a deliberate non-implementation of the converse arm rather than an
    // oversight. 8.4.44 SAt exempts a stu that FOLLOWS a `S`, and across the
    // whole curated corpus that exemption is the only thing the converse arm
    // would ever meet: vidyut-prakriya invokes 8.4.40 ZERO times over these
    // cells and 8.4.44 one hundred and eighteen, every one of them an `S`
    // before an `n` — aSnoti (`05.0020`, 36; `09.0059`, 41) and kliSnAti
    // (`09.0058`, 41). So the converse arm has exactly two fates and no
    // third: shipped without SAt it turns kliSnAti into *kliSYAti, and
    // shipped with SAt it is code that cannot fire, which the mutation gate
    // reports as a survivor because deleting it changes nothing. Add the two
    // together the moment a curated root puts a stu after a ścu that SAt
    // does not cover.
    //
    // The rules below are inert on the site this one writes. 8.4.55 Kari ca
    // reads the SHAP/ENDING junction rather than the tuk's position inside
    // ANGA, and refuses vacuous fires anyway (`sub == last`). 8.4.53 wants a
    // jhaś after the jhal, and `C` is voiceless. 8.4.1 works on Cfnad's
    // adjacent `f` and `n`, which the tuk sits in front of rather than
    // between — so it is not an 8.4.2 intervener question either.
    //
    // 8.4.65 Jaro Jari savarRe does NOT fork the cell this rule creates,
    // and the reason is worth stating because the surface looks like it
    // should: `c` and `C` are savarṇa jhars. 8.4.65 carries 8.4.64's
    // *halaḥ* by anuvṛtti, implemented there as `!is_vowel(w[i - 1])`, and
    // the character before this rule's `c` is the aṭ's own `a`.
    // `acchinat_has_exactly_two_forms` in `panini`'s trace suite is the pin.
    //
    // The substitute IS the map: `shcutva_of` carries every stu arm and a
    // `None` from it is this rule's match test as well. That is the shape
    // 8.2.30 coH kuH had to be rewritten into once a hardcoded pair proved
    // wrong for √ric and √vic; do not reintroduce a literal here.
    Rule {
        id: "8.4.40",
        name: "stoH ScunA ScuH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let w = word_chars(p);
            for i in 0..w.len().saturating_sub(1) {
                if !is_shcu(w[i + 1].2) {
                    continue;
                }
                let Some(sub) = shcutva_of(w[i].2) else {
                    continue;
                };
                let (term, idx, _) = w[i];
                let before = p.snapshot();
                set_char(p, term, idx, sub);
                p.record("8.4.40", "stoH ScunA ScuH", before);
                return true;
            }
            false
        },
    },
```

- [ ] **Step 10: Update the pinned rule order**

In `crates/panini-prakriya/src/tinanta/derivation_tests.rs`, `tinanta_rule_order_is_pinned`, insert `"6.1.73"` immediately after `"6.4.72"` and `"8.4.40"` immediately before `"8.4.41"`:

```rust
    let expected = [
        "1.3.12", "1.3.72", "1.3.78", "3.4.78", "1.3.9", "1.2.4", "3.4.85", "3.4.108", "3.4.105",
        "3.4.106", "3.4.101", "3.4.99", "3.4.87", "3.4.89", "3.4.86", "3.4.100", "3.4.80",
        "3.4.79", "3.4.91", "3.4.93", "3.4.90", "3.4.92", "3.4.103", "3.4.102", "7.1.35", "3.1.69",
        "3.1.73", "3.1.77", "3.1.78", "3.1.81", "3.1.68", "2.4.72", "3.4.111", "3.1.83", "1.2.4",
        "6.4.71", "6.4.72", "6.1.73", "7.3.100", "7.1.5", "7.1.6", "7.1.3", "7.2.79", "7.2.80",
        "7.2.81", "6.4.23", "7.4.21", "7.3.84", "7.3.86", "7.3.92", "7.3.84", "6.4.87", "6.4.77",
        "6.1.78", "7.3.101", "6.4.112", "6.4.113", "6.1.101", "6.1.96", "6.1.90", "6.1.97",
        "6.1.87", "6.1.66", "6.4.105", "6.4.106", "6.4.107", "6.4.101", "6.4.111", "8.2.77",
        "8.2.23", "8.2.25", "8.2.30", "8.2.31", "8.2.39", "8.2.40", "8.2.41", "8.2.74", "8.2.75",
        "8.2.73", "8.3.15", "8.3.24", "8.3.59", "8.4.40", "8.4.41", "8.3.13", "8.4.53", "8.4.55",
        "8.4.1", "8.4.2", "8.4.58", "8.4.65", "8.4.56",
    ];
```

`exactly_the_pinned_vikalpa_rules_are_optional` is **unchanged** — neither new rule is optional. If that test fails, a `vikalpa: true` was typed by mistake.

- [ ] **Step 11: Run the tests, then lint**

Run: `mise exec -- cargo test -p panini-prakriya`
Expected: all pass, including the four new tests and the reordered pin.

Then `mise run fmt-check && mise run lint`. **This is the first `lint` since Step 3**, and it is where a predicate that never found its rule shows up as `dead_code`. If `is_hrasva`, `is_shcu`, `shcutva_of` or `insert_char` is reported unused, a rule body is reading something else than the plan specifies — fix the rule, do not silence the lint.

- [ ] **Step 12: Run the full golden suite in the foreground**

Run: `mise run test`
Expected: all pass, **unchanged**. No curated root's `code` contains a `C`, and the corpus has no stu-before-ścu site, so neither rule should fire anywhere. A golden that moves here is the same failure Task 3 is built to catch, arriving early — stop and diagnose rather than adjusting a golden.

- [ ] **Step 13: Commit**

```bash
git add crates/panini-prakriya/src/tinanta/sound.rs crates/panini-prakriya/src/tinanta/terms.rs crates/panini-prakriya/src/tinanta/anga.rs crates/panini-prakriya/src/tinanta/tripadi.rs crates/panini-prakriya/src/tinanta/derivation_tests.rs
git commit -m "feat(tinanta): 6.1.73 Ce ca and 8.4.40 stoH ScunA ScuH

The tuk augment before a C after a short vowel, and the Scutva that
follows it, with the is_hrasva/is_shcu/shcutva_of table they read and the
insert_char helper 6.1.73 augments through. All in one commit because
clippy -D warnings fails a predicate whose only consumer is a test. 6.1.73 goes in anga.rs directly below 6.4.71, which
manufactures its whole precondition; 8.4.40 goes above 8.4.41 in sutra
order, the two being unable to contend.

8.4.40 ships stu->Scu only. 8.4.44 SAt exempts every converse-direction
site this corpus has -- 118 of them, all S before n -- so the converse arm
could only break kliSnAti or be unreachable code."
```

---

### Task 3: The dump diff — the first blocking gate

**Files:**
- Create then delete: two dump files under the scratchpad directory (not the repo)

**Interfaces:**
- Consumes: Task 2's two rules.
- Produces: the evidence that the rules perturb no existing cell — the fact every later task's attribution argument rests on.

The spec's central claim is that two new sūtras can land without disturbing one of the 2628 existing cells. Task 2's green suite is **not** that evidence: the golden suite pins the *declined* branch of every cell plus its alternates, and the dump is the full derivation set, cell by cell — a strictly wider object.

- [ ] **Step 1: Set up the vidyut checkout, if it is not already present**

```bash
head -20 data/dhatupatha.tsv | grep commit
```
Expected: the commit the corpus was vendored from — `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`.

```bash
ls /tmp/vidyut-full 2>/dev/null || (cd /tmp && git clone --filter=blob:none https://github.com/ambuda-org/vidyut vidyut-full && cd vidyut-full && git checkout 8da2f90bee3ce1c07505fa432fc3729e3f7e02ea)
```

Copy the committed harness in, per `tools/audit/README.md`. **Copy it; never rewrite it.**

```bash
cp tools/audit/panini_full_audit.rs /tmp/vidyut-full/vidyut-prakriya/examples/
```

If `vidyut-prakriya/Cargo.toml` does not already carry the two `[dev-dependencies]` path entries for `panini` and `panini-data`, add them per `tools/audit/README.md`.

- [ ] **Step 2: Dump the corpus as `main` derives it**

```bash
git stash
cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_DUMP=/tmp/claude-1000/-workspace/*/scratchpad/before.tsv cargo run --release --example panini_full_audit
cd /workspace && git stash pop
```

Expected: the harness asserts 64 roots / 2628 cells / 3057 forms and writes `before.tsv`. Use the literal scratchpad path this session was given rather than a glob if the shell does not expand it.

- [ ] **Step 3: Dump the corpus as the new engine derives it**

```bash
cd /tmp/vidyut-full/vidyut-prakriya && PANINI_AUDIT_DUMP=/tmp/claude-1000/-workspace/*/scratchpad/after.tsv cargo run --release --example panini_full_audit
```

- [ ] **Step 4: Diff them**

```bash
diff /tmp/claude-1000/-workspace/*/scratchpad/before.tsv /tmp/claude-1000/-workspace/*/scratchpad/after.tsv && echo "INERT"
```

Expected: no output, then `INERT`.

**If the diff is non-empty, stop and do not continue to Task 4.** A *correct-looking* changed form is still a failure here: the pre-7f corpus was audited byte-for-byte against vidyut, so any change is a regression by construction. The two likely causes, in order:

- **A changed `kliSnAti`, `aSnoti` or any `09.0058` / `09.0059` / `05.0020` cell** means 8.4.40 was implemented in the ścu-before-stu direction after all. That is the 8.4.44 *śāt* failure the rule's own comment predicts; fix the direction, do not add śāt.
- **A changed cell elsewhere** means `is_shcu` or `shcutva_of` admits a sound it should not — check `z` in particular, which belongs to 8.4.41.

- [ ] **Step 5: Record the result in the commit message and clean up**

```bash
rm -f /tmp/claude-1000/-workspace/*/scratchpad/before.tsv /tmp/claude-1000/-workspace/*/scratchpad/after.tsv
git commit --allow-empty -m "test(audit): 6.1.73 and 8.4.40 are inert on 2628 cells

Full derivation-set dump before and after Task 2's two new rules, over the
whole pre-7f corpus (64 roots / 2628 cells / 3057 forms): byte-identical.
8.4.40's one-direction implementation is what makes this hold -- the
converse direction's only reachable sites are the 118 that 8.4.44 SAt
forbids, in aSnoti and kliSnAti.

Every attribution argument in the rest of this slice rests on this."
```

---

### Task 4: The two data rows

**Files:**
- Modify: `crates/panini-data/src/lib.rs` (two `Dhatu` rows after `07.0018`; `rudhadi_rows_…`; `dhatus().len()`; the prose counts at `:83`, `:94`, `:1136`)
- Modify: `crates/panini/tests/paradigm.rs:5549` (`GATED`)

**Interfaces:**
- Consumes: nothing from Task 3 but its verdict.
- Produces: `07.0003` and `07.0008` as curated `Dhatu` rows with `PadaAssignment::Ubhayapada`, which Tasks 5, 6 and 7 all enumerate over.

- [ ] **Step 1: Write the failing table test**

In `crates/panini-data/src/lib.rs`, rename `rudhadi_rows_are_the_twenty_two_curated_roots` to `rudhadi_rows_are_the_twenty_four_curated_roots` and append two entries to its expected vec, after `("07.0018", "tfh", PadaAssignment::Parasmaipada),`:

```rust
                ("07.0003", "Cid", PadaAssignment::Ubhayapada),
                ("07.0008", "Cfd", PadaAssignment::Ubhayapada),
```

Replace the test's final comment paragraph — the one beginning "Three of rudhādi's 25 are still out after this" — with:

```rust
        // Slice 7f adds √chid and √chṛd, the last two ubhayapadī roots and
        // the last two that needed a sūtra: 6.1.73 Ce ca puts the tuk after
        // laṅ's aṭ-augment before their initial `C`, and 8.4.40 stoH ScunA
        // ScuH makes that `t` a `c` -- acCinat, acCfRat. Neither root needed
        // anything else: √chid is √bhid with a `C` for its `B`, and √chṛd is
        // √tṛd with a `C` for its `t`, ṇatva included, so every cell outside
        // laṅ derives on rules that were already in the pipeline.
        //
        // ONE of rudhādi's 25 is still out after this: √bhuj (`07.0017`),
        // and not for want of phonology -- vidyut derives all 72 of its
        // cells and 1.3.66 Bujo'navane is the only rule this engine lacks,
        // a root-keyed pada assignment structurally identical to 1.3.72's.
        // What keeps it out is that 1.3.66 restricts ātmanepada to senses
        // other than protecting, and neither engine models sense.
```

- [ ] **Step 2: Run it to verify it fails**

Run: `mise exec -- cargo test -p panini-data rudhadi_rows_are_the_twenty_four_curated_roots`
Expected: FAIL — `assertion failed: (left == right)`, the left vec missing the two new triples.

- [ ] **Step 3: Add the two `Dhatu` rows**

In `crates/panini-data/src/lib.rs`, after the `07.0018` row's closing `},` and before the array's closing `];`:

```rust
    Dhatu {
        // 07.0003 Ci\di~^r dvEDIkaraRe. Ubhayapadī by 1.3.72 svaritaYitaH:
        // the `~^` is a svarita it, while the `\` is the root vowel's own
        // accent and says nothing about pada. Shape-identical to √bhid
        // (`07.0002`) -- `Ci` + `nad` where √bhid has `Bi` + `nad` -- so
        // every cell outside laṅ derives on rules already in the pipeline.
        //
        // The laṅ cells are the whole of what this root cost: 6.4.71's aṭ
        // puts a short `a` before the root's initial `C`, 6.1.73 Ce ca
        // inserts the tuk after it, and 8.4.40 stoH ScunA ScuH makes that
        // `t` a `c` -- acCinat, where the engine would otherwise reach
        // *aCinat.
        dhatupatha: "07.0003",
        code: "Cid",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "dvEDIkaraRe",
    },
    Dhatu {
        // 07.0008 u~Cfdi~^r dIptidevanayoH. Ubhayapadī by 1.3.72, on the
        // same svarita it as √chid. Udit, like √tṛd (`07.0009`) -- the
        // initial `u~` matters for 7.2.56 and 1.2.26 in ārdhadhātuka
        // contexts this engine does not cover, and is inert across all four
        // sārvadhātuka lakāras here.
        //
        // Shape-identical to √tṛd: `Cf` + `Rad` where √tṛd has `tf` + `Rad`,
        // 8.4.1's ṇatva included, since the trigger is the root's own `f`.
        // The tuk 6.1.73 inserts sits in FRONT of that `f` rather than
        // between it and the `n`, so it raises no 8.4.2 intervener question.
        dhatupatha: "07.0008",
        code: "Cfd",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "dIptidevanayoH",
    },
```

- [ ] **Step 4: Bump `dhatus().len()`**

In `curated_roots_have_expected_ganas_and_padas` (`:769`):

```rust
        assert_eq!(dhatus().len(), 66);
```

- [ ] **Step 5: Fix the three count-bearing comments in the same file**

- `:94` — "The test covers the 64 roots curated here, not the dhātupāṭha's 2259." → **66**.
- `:83` — "re-derives every one of these 55 verdicts". This figure is **already stale** and contradicts `:94` of the same doc comment, which says 64, as does AGENTS.md. **Read `curated_pada_agrees_with_upadesha_markers` and establish which count it actually enforces before editing either line** — do not resolve the contradiction by trusting AGENTS.md. Then take it to **66**.
- `:1136` — "42 of the 64 curated roots carry a `\` at all, and 29 of those carry one on a root vowel" → **43 of the 66 … and 30 of those**. `Ci\di~^r` adds one to both counts (its `\` is on the root vowel); `u~Cfdi~^r` carries no `\` at all.

- [ ] **Step 6: Run the data crate's tests**

Run: `mise exec -- cargo test -p panini-data`
Expected: all pass, including `curated_pada_agrees_with_upadesha_markers` — which re-derives both new rows' `Ubhayapada` verdict from the vendored upadeśa via 1.3.72. If it fails on either row, the `pada` column is wrong, not the test.

- [ ] **Step 7: Gate the sixteen golden triples**

The two roots are curated but have no `PARADIGM` blocks until Task 6. In `crates/panini/tests/paradigm.rs`, replace `const GATED: &[(&str, &str, Pada)] = &[];` with:

```rust
    const GATED: &[(&str, &str, Pada)] = &[
        ("07.0003", "laT", Pada::Parasmaipada),
        ("07.0003", "laT", Pada::Atmanepada),
        ("07.0003", "laN", Pada::Parasmaipada),
        ("07.0003", "laN", Pada::Atmanepada),
        ("07.0003", "loT", Pada::Parasmaipada),
        ("07.0003", "loT", Pada::Atmanepada),
        ("07.0003", "viDiliN", Pada::Parasmaipada),
        ("07.0003", "viDiliN", Pada::Atmanepada),
        ("07.0008", "laT", Pada::Parasmaipada),
        ("07.0008", "laT", Pada::Atmanepada),
        ("07.0008", "laN", Pada::Parasmaipada),
        ("07.0008", "laN", Pada::Atmanepada),
        ("07.0008", "loT", Pada::Parasmaipada),
        ("07.0008", "loT", Pada::Atmanepada),
        ("07.0008", "viDiliN", Pada::Parasmaipada),
        ("07.0008", "viDiliN", Pada::Atmanepada),
    ];
```

Confirm the lakāra name strings against `panini::lakara_name` before relying on them — `paradigm_covers_every_enumerable_cell` builds its triples from that function, and a mismatched string fails the test with an unhelpful diff. Existing `PARADIGM` blocks are the reference: grep one root's four blocks and copy the exact spellings.

- [ ] **Step 8: Run the full golden suite in the foreground**

Run: `mise run test`
Expected: all pass. `paradigm_covers_every_enumerable_cell` is satisfied by `GATED`; `derivation_set_shape_matches_the_audited_numbers` still asserts the **old** 2628/429 numbers and still passes, because `PARADIGM` has not grown yet.

- [ ] **Step 9: Commit**

```bash
git add crates/panini-data/src/lib.rs crates/panini/tests/paradigm.rs
git commit -m "feat(data): Cid and Cfd, rudhadi's last two ubhayapadi roots

Both by 1.3.72 on their svarita it. Sixteen golden triples gated until the
audit certifies the derivations in the next two tasks.

Also corrects panini-data's own stale verdict count: :83 said 55 while :94
of the same doc comment said 64. Both now 66."
```

---

### Task 5: The cross-implementation audit — the second blocking gate

**Files:**
- Modify: `tools/audit/panini_full_audit.rs` (the corpus-total assertions at `:577–579`, and the header's totals at `:12`, `:24`, `:27`, `:54`)
- Modify: `tools/audit/README.md` (the asserted-totals line at `:30`, "Last recorded result", and the growth arithmetic)

**Interfaces:**
- Consumes: the complete engine (Tasks 1, 2) and the two rows (Task 4).
- Produces: the verdict Task 6's goldens are generated under, and the measured form total `<N>` that Tasks 6 and 9 both quote.

- [ ] **Step 1: Update the harness's corpus totals**

In `tools/audit/panini_full_audit.rs`, at `:577–579`:

```rust
    assert_eq!(roots_seen.len(), 66, "curated roots");
    assert_eq!(n_cells, 2772, "cells: 308 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 3259, "forms: 2772 cells + 487 ALTERNATES rows");
```

and update the four header mentions — `:12` (`for each of the 64 curated roots` → 66), `:24` (`64 roots, 2628 cells, 3057 forms` → `66 roots, 2772 cells, 3259 forms`), `:27` (`292 root×pada×lakāra blocks × 9 cells, plus 429 ALTERNATES rows` → `308 … plus 487 …`), `:54` (`the full 2628-cell table` → 2772).

If the run reports a different **form** total than 3259, **the harness's number is the measurement and 3259 was the projection** — change the assertion to what it measured and carry that value forward into Tasks 6 and 9. A differing **cell** count, by contrast, means a pada or lakāra is miscounted: 308 is `292 + 16`, and 16 is two roots × two padas × four lakāras. Investigate rather than adjust.

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

Expected: **exit 0**, 66 roots / 2772 cells / `<N>` forms, zero differences.

**If any cell differs, stop.** Task 3 already proved both rules inert on the pre-7f corpus, so a difference belongs to √chid or √chṛd, and the failing cell names the cause:

- **A laṅ cell surfacing `aCinat` / `aCfRat`** — 6.1.73 declined. Its guard reads a short vowel before `C`; check that 6.4.71 ran first and that the aṭ really is inside `ANGA`.
- **A laṅ cell surfacing `atCinat` / `atCfRat`** — 6.1.73 fired and 8.4.40 declined. `is_shcu('C')` or `shcutva_of('t')` is wrong.
- **A laṅ cell with three forms where two are expected** (`aCinat` alongside `acCinat`) — 8.4.65 fired, meaning its `halaḥ` guard was weakened. That guard is not this slice's to change.
- **A non-laṅ cell differing at all** — 6.1.73 is firing outside laṅ, which means its whole-word scan found a `C` that is not root-initial.

**What is NOT a difference, and must not be chased:** vidyut's own trace for these cells credits 6.1.68 *hal ṅyāb bhyo dīrghāt su-ti-sy-apṛktaṁ hal* for deleting laṅ's apṛkta `t`, and this engine has no 6.1.68 — it reaches the same surface through 8.2.23 *saṁyogāntasya lopaḥ*. That divergence predates √tṛh, holds across every curated rudhādi root, and is invisible to this harness, which compares derivation **sets** rather than traces. √chid and √chṛd inherit it unchanged. Do not re-litigate it here.

The posture is fixed: fix the rule, do not widen the slice, and do not adjust a golden.

- [ ] **Step 6: Record the result**

In `tools/audit/README.md`, update the asserted-totals line at `:30` to `(66 roots, 2772 cells, 3259 forms)` and "Last recorded result" to name: vidyut commit `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, 66 roots / 2772 cells / `<N>` forms, zero differences, both `entry` and `form` negative controls verified failing first, and the growth arithmetic — 64 → 66 roots (two curated roots, √chid and √chṛd); 2628 → 2772 cells (144 = 2 roots × 2 padas × 4 lakāras × 9); 3057 → 3259 forms, `ALTERNATES` 429 → 487.

- [ ] **Step 7: Commit**

```bash
git add tools/audit/panini_full_audit.rs tools/audit/README.md
git commit -m "test(audit): Cid and Cfd are byte-identical to vidyut across 2772 cells"
```

---

### Task 6: The goldens, generated

**Files:**
- Create then delete: `crates/panini/tests/print_7f_goldens.rs`
- Modify: `crates/panini/tests/paradigm.rs` (`PARADIGM`, `ALTERNATES`, `GATED` back to `&[]`, `derivation_set_shape_matches_the_audited_numbers` and its doc comment, the pada-collision vec and its comment)

**Interfaces:**
- Consumes: Task 5's certified engine.
- Produces: the 16 `PARADIGM` blocks and 58 `ALTERNATES` rows every later task's counts refer to.

- [ ] **Step 1: Write the throwaway generator**

Create `crates/panini/tests/print_7f_goldens.rs`:

```rust
//! THROWAWAY -- slice 7f only. Prints √chid's and √chṛd's `PARADIGM` blocks
//! and `ALTERNATES` rows as Rust source, plus the distribution counts
//! `derivation_set_shape_matches_the_audited_numbers` asserts. Deleted in
//! the same task that pastes its output: goldens are generated from the
//! engine the audit certified, never hand-authored.
//!
//! Run with:
//!   mise exec -- cargo test -p panini --test print_7f_goldens -- --nocapture

mod common;

use common::{CELLS, LAKARA_BY_NAME};
use panini_data::{Pada, dhatus};
use panini_prakriya::derive;

const NEW_ROOTS: [&str; 2] = ["07.0003", "07.0008"];

/// Mirrors `VIKALPA_RULES` in `paradigm.rs`. An alternate's key is the
/// `+`-joined list of optional rules its branch actually applied, which is
/// what `every_alternate_names_the_vikalpa_rules_that_produced_it` checks.
/// 7f adds no optional rule, so this list is unchanged from 7e.
const VIKALPA_RULES: &[&str] = &[
    "7.1.35", "3.4.111", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56",
];

#[test]
fn print_7f_goldens() {
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

Run: `mise exec -- cargo test -p panini --test print_7f_goldens -- --nocapture`

Expected: sixteen `PARADIGM` blocks, **58** `ALTERNATES` rows, and counts reading `key "8.4.65": +34`, `key "8.4.56": +6`, `key "7.1.35": +4`, `key "7.1.35+8.4.56": +4`, `key "7.1.35+8.4.65": +4`, `key "7.1.35+8.4.65+8.4.56": +4`, `key "8.2.75": +2`, `cells with 1 form(s): +102`, `cells with 2 form(s): +36`, `cells with 3 form(s): +2`, `cells with 5 form(s): +2`, `cells with 6 form(s): +2`.

That profile is exactly twice √bhid's and √tṛd's, which are identical to each other. If the counts differ, the derivation differs from the audited one — which cannot happen, since Task 5 certified it. Re-run Task 5 rather than adjusting the numbers.

- [ ] **Step 3: Paste the blocks into `PARADIGM`**

Paste the sixteen printed blocks verbatim into `crates/panini/tests/paradigm.rs`'s `PARADIGM`, after `07.0018`'s blocks. Do not retype any surface.

- [ ] **Step 4: Paste the rows into `ALTERNATES`**

Paste the 58 printed rows verbatim into `ALTERNATES`, at the end.

- [ ] **Step 5: Empty `GATED`**

Restore `crates/panini/tests/paradigm.rs`'s `GATED` to:

```rust
    const GATED: &[(&str, &str, Pada)] = &[];
```

and add one sentence to the comment above it recording that √chid's and √chṛd's sixteen triples were gated for one commit in slice 7f, between the rows landing and the audited goldens arriving.

- [ ] **Step 6: Update the audited-numbers assertions**

In `derivation_set_shape_matches_the_audited_numbers`:

```rust
    let total_cells = PARADIGM.len() * 9;
    assert_eq!(total_cells, 2772, "308 root×lakāra blocks × 9 cells each");
```

```rust
    assert_eq!(ones, 2426, "one-form cells");
    assert_eq!(twos, 247, "two-form cells");
    assert_eq!(threes, 81, "three-form cells");
    assert_eq!(
        fours, 2,
        "four-form cells — piṣ's loṭ madhyama eka, and — new in slice 7d — Siz's loṭ \
         parasmaipada madhyama eka"
    );
    assert_eq!(
        fives, 8,
        "five-form cells — kft loṭ prathama eka, ruD loṭ parasmaipada prathama eka, Bid, kzud \
         and tfd's loṭ parasmaipada prathama eka, und's (slice 7d), and — new in slice 7f — \
         Cid's and Cfd's loṭ parasmaipada prathama eka"
    );
    assert_eq!(
        sixes, 8,
        "six-form cells — kft loṭ madhyama eka, ruD loṭ parasmaipada madhyama eka, Bid, kzud \
         and tfd's loṭ parasmaipada madhyama eka, und's (slice 7d), and — new in slice 7f — \
         Cid's and Cfd's loṭ parasmaipada madhyama eka"
    );

    assert_eq!(ALTERNATES.len(), 487, "ALTERNATES row count");
```

and the key counts: `8.4.56` **111**, `7.1.35` **90**, `7.1.35+8.4.56` **90**, `8.4.65` **145**, `7.1.35+8.4.65` **16**, `7.1.35+8.4.65+8.4.56` **16**, `8.2.75` **8**. `6.4.107` (8), `3.4.111` (2) and `8.2.74` (1) are unchanged — 6.4.107 concerns only svādi's √hi and √ri, which 7f does not touch.

- [ ] **Step 7: Update that test's doc comment**

Replace its closing √tṛh sentence — "√tṛh joins none of the fork records: its deepest cells hold three forms, because 8.3.13 Qo Qe lopaH obligatorily elides the ḍh that 8.4.65 forks on for every other stop-final rudhādi root." — with that sentence plus:

```rust
/// √chid and √chṛd, by contrast, join both fork records: they are
/// dental-final like √bhid and √tṛd, nothing elides the junction 8.4.65
/// wants, and their loṭ parasmaipada eka cells stack 7.1.35, 8.4.65 and
/// 8.4.56 into five branches at prathama eka and six at madhyama eka. The
/// six-form record now stands at eight cells, not six. Their laṅ cells
/// hold two forms rather than three despite acCinad's `c` and `C` being
/// savarṇa jhars: 8.4.65 carries 8.4.64's *halaḥ* by anuvṛtti and the
/// sound before that `c` is the aṭ's own vowel.
```

Also update the audit sentence in the same comment to name 66 roots / 2772 cells / `<N>` forms at vidyut `8da2f90`.

- [ ] **Step 8: Extend the pada-collision vec**

At the end of `crates/panini/tests/paradigm.rs`, the `both` assertion currently pins twenty-two surfaces. √chid and √chṛd each contribute the same pair every dental-final ubhayapadī rudhādi root does. Add to the vec, keeping its existing sort order:

```rust
            "CfnttAm", "CinttAm", "acCfntta", "acCintta",
```

and add a sentence to the narrating comment above it: slice 7f's two new ubhayapadī roots contribute four more, the same shape as √bhid's and √tṛd's pairs, taking the set to twenty-six with no new collision against any pre-slice surface.

**Do not guess the sort position.** Run the test once and let its diff place them — the vec is sorted by the harness, and the assertion prints the expected ordering on failure.

- [ ] **Step 9: Delete the generator**

```bash
rm crates/panini/tests/print_7f_goldens.rs
```

- [ ] **Step 10: Run the full golden suite in the foreground**

Run: `mise run test`
Expected: all pass. This is the first run at 2772 cells and will take noticeably longer than Task 4's.

- [ ] **Step 11: Commit**

```bash
git add crates/panini/tests/paradigm.rs
git commit -m "test(paradigm): Cid and Cfd's audited paradigms

Sixteen PARADIGM blocks and 58 ALTERNATES rows, generated from the engine
the audit certified. Both roots join the six-form record -- dental-final
like Bid and tfd, with nothing eliding the junction 8.4.65 wants -- taking
that record from six cells to eight.

Their laN cells hold two forms, not three: acCinad's `c` and `C` are
savarna jhars, but 8.4.65 carries 8.4.64's halaH and the sound before that
`c` is the aT's own vowel."
```

---

### Task 7: The trace pins

**Files:**
- Modify: `crates/panini/tests/trace.rs`

**Interfaces:**
- Consumes: the curated roots and their goldens.
- Produces: the four ordering and fork-count pins the spec names.

- [ ] **Step 1: Write the four tests**

Append to `crates/panini/tests/trace.rs`:

```rust
#[test]
fn acchinat_trace_orders_the_tuk_between_the_augment_and_shcutva() {
    // Cid laN prathama eka. The slice's central ordering fact, and it
    // spans three stages: 6.4.71 is in `anga`, 6.1.73 immediately below it
    // in the same stage, and 8.4.40 in `tripadi`. Each link is load-bearing
    // in a different way.
    //
    // 6.4.71 < 6.1.73: the aT-augment IS the short vowel 6.1.73 attaches
    // to. Run 6.1.73 first and the `C` is word-initial, the guard declines,
    // and the cell surfaces *aCinat.
    //
    // 6.1.73 < 8.4.40: the tuk IS the stu that Scutva palatalizes. Without
    // it there is nothing before the `C` for 8.4.40 to read, and the cell
    // surfaces *aCinat again -- by a different route, which is why both
    // links are pinned rather than just the surface.
    //
    // 8.4.40 < 8.4.56: vA'vasAne is last in the pipeline by construction.
    let (text, t) = cell_trace(
        "07.0003",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "acCinad", "got {t:?}");
    assert!(at(&t, "6.4.71") < at(&t, "6.1.73"), "got {t:?}");
    assert!(at(&t, "6.1.73") < at(&t, "8.4.40"), "got {t:?}");
    assert!(at(&t, "8.4.40") < at(&t, "8.4.56"), "got {t:?}");
}

#[test]
fn acchrnat_trace_runs_natva_and_shcutva_on_disjoint_sites() {
    // Cfd laN prathama eka -- the one cell in the corpus that reaches both
    // Natva and Scutva. They touch different characters of the same word:
    // 8.4.1 rewrites the `n` of Cfnad, whose trigger is the root's own `f`
    // directly before it, while 8.4.40 rewrites the tuk sitting IN FRONT of
    // that `f`.
    //
    // The negative half is the pin. If the tuk were ever placed between the
    // `f` and the `n`, 8.4.2's intervener test would decide the cell
    // instead -- `t` is not an aT member, so Natva would be blocked and the
    // cell would surface *acCfnad. That this test asserts both rules fired
    // is what says the tuk did not land there.
    let (text, t) = cell_trace(
        "07.0008",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "acCfRad", "got {t:?}");
    assert!(t.contains(&"8.4.1".to_string()), "got {t:?}");
    assert!(t.contains(&"8.4.40".to_string()), "got {t:?}");
    assert!(at(&t, "6.1.73") < at(&t, "8.4.40"), "got {t:?}");
}

#[test]
fn chinatti_trace_cites_neither_new_sutra() {
    // Cid laT prathama eka. Both new sutras are laN-only, and for one
    // reason: outside laN there is no aT-augment, so the root's `C` is
    // word-initial and 6.1.73 has no short vowel to attach the tuk to.
    // 8.4.40 then has no stu to read.
    //
    // This is the cheapest guard against 6.1.73's `is_hrasva(w[i - 1].2)`
    // conjunct being dropped: without it the scan would fire at index 0 on
    // any C-initial word and every one of these 54 non-laN cells would grow
    // a spurious `t`.
    let (text, t) = cell_trace(
        "07.0003",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "Cinatti", "got {t:?}");
    assert!(!t.contains(&"6.1.73".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.40".to_string()), "got {t:?}");
}

#[test]
fn acchinat_has_exactly_two_forms() {
    // Cid laN prathama eka holds acCinad and acCinat, and nothing else --
    // the 8.4.56 vA'vasAne fork alone.
    //
    // The pin is that 8.4.65 Jaro Jari savarRe does NOT also fire. After
    // 8.4.40 the word carries `c` followed by `C`: same sthana, same
    // abhyantara prayatna, so savarna jhars, and the sutra read bare would
    // optionally elide the `c` and give a third form *aCinat. It declines
    // because 8.4.65 carries 8.4.64 halo yamAM yami lopaH's *halaH* by
    // anuvrtti -- implemented as `!is_vowel(w[i - 1])` -- and the sound
    // before that `c` is the aT-augment's own `a`.
    //
    // No previously curated root could put a savarna jhar pair directly
    // after a vowel, so this is the first cell to exercise that guard in
    // the direction that proves it necessary. Weaken it and the ALTERNATES
    // count is the second alarm; this is the one that says why.
    let d = dhatus()
        .iter()
        .find(|d| d.dhatupatha == "07.0003")
        .expect("07.0003 is curated");
    let forms: Vec<String> = derive(
        d,
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    )
    .iter()
    .filter(|p| !p.blocked)
    .map(|p| p.text())
    .collect();
    assert_eq!(forms, vec!["acCinad", "acCinat"], "got {forms:?}");
}
```

- [ ] **Step 2: Run them**

Run: `mise exec -- cargo test -p panini --test trace`
Expected: all pass, including the four new ones.

If `acchinat_has_exactly_two_forms` reports three forms with `aCinat` among them, 8.4.65's `halaḥ` guard has been weakened — that guard is not this slice's to change, so revert whatever touched it.

- [ ] **Step 3: Commit**

```bash
git add crates/panini/tests/trace.rs
git commit -m "test(trace): the tuk's place between the augment and Scutva

Four pins: acCinad orders 6.4.71 < 6.1.73 < 8.4.40 < 8.4.56 across three
stages; acCfRad shows Natva and Scutva on disjoint sites, which is what
says the tuk did not land between the `f` and the `n`; Cinatti cites
neither new sutra; and acCinad holds exactly two forms, pinning that
8.4.65 declines on a savarna jhar pair standing after a vowel."
```

---

### Task 8: The mutation gate

**Files:**
- Modify: `AGENTS.md` (the `cargo-mutants` paragraph)

**Interfaces:**
- Consumes: the finished engine and suite.
- Produces: the recorded floor, contention and campaign figures the next slice reasons from.

- [ ] **Step 1: Measure the uncontended floor**

Run: `mise run test` and record `paradigm`, `roundtrip` and `trace`'s individual times plus the wall clock.

The floor stood at **943.70s** at 2628 cells (paradigm 432.94s, roundtrip 508.54s, trace 2.22s). Cell growth here is +5.5%. **Do not scale — measure.** Cell count has failed as a multiplier for five consecutive slices, most recently under-predicting by a factor of six (+8.2% floor for +1.4% cells).

- [ ] **Step 2: Check the floor against the cap**

Multiply the measured floor by the **1.02×–1.43×** `-j 4` contention range measured across 7e's 508 test phases. Against the 4800s cap that should leave a margin around 3×.

Do **not** re-derive 7e's retired 2.1–2.5× figure as settled, and do not quote the 1.02× end alone — 7e's own entry records that a *caught* mutant ran longer (1345s) than either *uncaught* one (980s, 967s), because scheduling overlap dominates wall-clock duration rather than whether a mutant is caught. If the projected worst case exceeds ~2400s, raise the cap in `mise.toml` and say so in the AGENTS.md entry rather than running under a thin margin.

- [ ] **Step 3: Run the campaign**

Run: `mise run mutants`

If the shim errors with `no version set for shim: cargo-mutants`, invoke the binary directly with the same flags:

```bash
mise exec -- cargo-mutants --package panini-prakriya --test-workspace=true --timeout 4800 -j 4
```

Run it in the **foreground**. Expect several hours.

- [ ] **Step 4: Check both files, not just `missed.txt`**

```bash
cat mutants.out/missed.txt; echo "--- timeouts ---"; cat mutants.out/timeout.txt
```

Expected, and **verify rather than assume**:

- **One timeout** — the known-permanent non-terminating-loop mutant on `tripadi.rs`'s 8.4.2 backward ṇatva scan (`j -= 1` → `j /= 1`, which makes `j` constant so the run never reaches an assertion). Identify it **by that shape**, not by line number: the number has drifted between slices without either slice touching the file, and this slice adds a rule above it. It is the correct verdict at any cap; do not chase it with a bigger `--timeout` or a code change.
- **Two missed**, and they must be 7e's two verified equivalent mutants: `adesha.rs`'s 6.1.87 im-arm `+`→`*` (`s.remove(pos + 1)` → `s.remove(pos)`), and `tripadi.rs`'s 8.3.13 guard `-`→`/` (`w[i - 1]` → `w[i]`). Both are documented in place at their guards.

**A third missed mutant, or a different two, is a real finding.** Likely candidates given what this slice adds: `shcutva_of`'s five unwitnessed arms (covered by `shcutva_of_stu_all_arms` — if one survives, that test is wrong); 8.4.40's `saturating_sub(1)`; and 6.1.73's `idx + 1`, whose `+`→`*` mutant is **not** equivalent here — `idx` is 0 for the aṭ's `a` in the `aCi` case, so `idx * 1 == idx` would insert before the vowel and give `taCi`, which the goldens catch.

- [ ] **Step 5: Fix any genuine survivor**

Add the missing test, or delete the dead code the survivor exposes. 7e's own campaign found `Context::is_tip` was redundant plumbing and deleted it and its one caller — that is the discipline working, not a setback. Re-run only the affected mutants rather than the whole campaign.

- [ ] **Step 6: Record the numbers in `AGENTS.md`**

Append a paragraph to the `cargo-mutants` entry in the same voice as the existing ones, giving: the cell count (2772), the measured floor with its three component times, the growth against 7e's 943.70s at 2628 cells and whether cell count predicted it, the cap sanity check and its basis, the campaign totals (mutants / caught / missed / unviable / timeout), the per-mutant test-phase median, p90, p99 and max with the over-600s count, the two margins (against the worst **caught** mutant, measured; against the worst **uncaught** run, projected — and label which is which), and the ruling on the cap.

- [ ] **Step 7: Commit**

```bash
git add AGENTS.md mutants.out
git commit -m "test(mutants): record the 7f campaign result"
```

---

### Task 9: The documentation sweep

**Files:**
- Modify: `AGENTS.md`, `docs/ARCHITECTURE.md`, `README.md`, `crates/panini-prakriya/src/tinanta/tripadi.rs:1031`

**Interfaces:**
- Consumes: every measured number from Tasks 5, 6 and 8.
- Produces: a repo whose prose matches its assertions.

**A grep for the totals will not find them all.** Several are line-wrapped across two lines (`2628` and `cells` on different lines in `AGENTS.md`) and several are rule-scoped rather than corpus-scoped. Work the list below file by file; do not substitute a grep for it.

- [ ] **Step 1: `AGENTS.md`**

- The golden-suite paragraph (`:361`, `:371`): `2628 cells` → 2772; `429 rows in all, so 2628 + 429 = 3057 forms total` → `487 rows in all, so 2772 + 487 = 3259`; the fork census — the fourth-form cells stay at 2, but the six-way-tied loṭ parasmaipada roots go from six (√kṛt, √rudh, √bhid, √kṣud, √tṛd, √und) to **eight**, adding √chid and √chṛd.
- The rudhādi paragraph (`:442`, `:566`): "twenty-two" → twenty-four; move √chid and √chṛd out of the deferral list into the curated enumeration, naming 6.1.73 and 8.4.40; "22 curated + 2 uncurated ubhayapadī + √bhuj = 25, so **3 of the 25 remain out**" → "24 curated + √bhuj = 25, so **1 of the 25 remains out**". **The sentence beginning "**√chid and √chṛd** need two sūtras the engine does not have" must go** — it is the claim this slice falsifies.
- The √bhuj sentence: it currently says his pada forks "on sense, not on an axis this engine models", which overstates the obstacle. Replace with what the spec measured: vidyut derives all 72 cells, 1.3.66 is the only rule this engine lacks, and it is a root-keyed pada assignment structurally identical to 1.3.72's — what keeps √bhuj out is the *anavane* sense restriction neither engine models, not the cost.
- The audit paragraph (`:510`, `:683`): add 7f's run — 66 roots / 2772 cells / `<N>` forms at vidyut `8da2f90`, both negative controls verified failing first.
- `:283` and `:1031`-adjacent prose citing "the golden suite's full 2628 cells" inside the 8.3.13 equivalent-mutant discussion → 2772.

- [ ] **Step 2: `crates/panini-prakriya/src/tinanta/tripadi.rs:1031`**

Inside 8.3.13's comment, "the golden suite's full 2628 cells" → 2772.

`controller.rs:130` and `tinanta/guna.rs:943` stay as they are. Both cite 1872/1864-of-1872 and have been deliberately stale since 7c; this slice does not adopt them. **Do not "fix" them.**

- [ ] **Step 3: `docs/ARCHITECTURE.md`**

- `:83`: "The gaṇa carries twenty-two roots" → twenty-four, appending √chid (`07.0003`) and √chṛd (`07.0008`), curated in slice 7f with 6.1.73 *che ca* and 8.4.40 *stoḥ ścunā ścuḥ*, both ubhayapadī by 1.3.72.
- `:140-151`: the paragraph naming √chid and √chṛd as still out must be rewritten — they are in. "rudhādi is already past it at twenty-two" → twenty-four; "what is left is the two-sūtra gap that keeps √chid and √chṛd out, and √bhuj's sense axis — **3 of the 25 in all**" → √bhuj's sense axis alone, **1 of the 25**.
- The branch-count paragraph: six roots stacking three optional rules → **eight**.

- [ ] **Step 4: `README.md`**

- `:18-27`: "twenty-two of its roots" → twenty-four, appending √chid and √chṛd to the enumeration with their slice and sūtras; "**3 of the 25 remain out**" → **1**; delete the "√chid and √chṛd for two sūtras the engine does not implement (6.1.73 *che ca*, 8.4.40 *stoḥ ścunā ścuḥ*)" clause; keep √bhuj's, corrected as in Step 1.
- `:27`: "over a curated 64-root set" → 66.
- `:37-40`: "304 of the 2628 cells hold more than one form: 211 hold two, 79 hold three … two hold four … six hold five, and six hold six" → "346 of the 2772 … 247 hold two, 81 hold three … two hold four … eight hold five, and eight hold six".

- [ ] **Step 5: Verify the sweep**

```bash
grep -rn "2628\|3057\|twenty-two\|3 of the 25\|\b429\b\|\b64 roots\b" --include=*.md --include=*.rs . | grep -v "^./target\|^./mutants.out\|docs/superpowers/"
```

Expected: only intentional historical references remain — AGENTS.md's mutation paragraph legitimately cites 2628 and 943.70s as *7e's* measurement, and `tools/audit/README.md`'s growth arithmetic legitimately cites the old totals as the "from" side. Anything else is a miss. `docs/superpowers/` is excluded because past specs and plans are historical records and are never retro-edited.

- [ ] **Step 6: Run the full suite once more and commit**

Run: `mise run test && mise run lint && mise run fmt-check`

```bash
git add AGENTS.md README.md docs/ARCHITECTURE.md crates/panini-prakriya/src/tinanta/tripadi.rs
git commit -m "docs: rudhadi carries twenty-four roots, and only Buj is out

Cid and Cfd move from the deferral list to the curated enumeration. Also
corrects AGENTS.md's framing of Buj: 1.3.66 is the only rule the engine
lacks and it is a root-keyed pada assignment like 1.3.72's, so what keeps
Buj out is the anavane sense restriction, not the cost."
```

---

### Task 10: Finish the branch

**Files:** none.

**Interfaces:**
- Consumes: every prior task.
- Produces: the merge.

- [ ] **Step 1: Open the PR**

```bash
git push -u origin rudhadi-gana-7f
gh pr create --fill
```

The PR body should state: the audit's verdict (vidyut commit, 66 roots / 2772 cells / `<N>` forms, zero differences, both negative controls verified failing first); the dump-diff result that gates the two rules; the mutation result (mutants, caught, the two known equivalent missed, the one known-permanent timeout); and the one-sentence version of what the slice found — that 7e's deferral for √chid and √chṛd was complete, naming exactly the two sūtras they needed and no widenings, because each root is shape-identical to one already curated.

Append `https://claude.ai/code/session_013or8tNwSFxYcZ3gtxfFVV1` to the body.

- [ ] **Step 2: Merge and clean up**

Use the `superpowers:finishing-a-development-branch` skill: wait for CI, merge the green PR, verify the commits are on `main`, then delete the branch.

---

## Deferred, and why

- **√bhuj (`07.0017`)**, and with it **1.3.66 *bhujo'navane***. Costed during 7f's design rather than merely deferred: vidyut derives all 72 cells (79 forms) and 1.3.66 is the only rule this engine lacks — a root-keyed pada assignment structurally identical to the 1.3.72 rule in `samjna.rs`, with no new phonology (√bhuj is √yuj with a `B`). What keeps it out is a ruling, not a cost: 1.3.66 restricts ātmanepada to senses other than protecting, and neither engine models sense, so curating it means shipping an unconditional ubhayapada assignment with *anavane* recorded as unimplemented. Its own slice.
- **8.4.44 *śāt* and the converse direction of 8.4.40.** Argued at 8.4.40's own comment: the converse arm's only reachable sites in this corpus are the 118 that śāt forbids, so it could only break √kliś or be unreachable code.
- **8.4.42 *na padāntāṭ ṭoranaḥ* and 8.4.43 *toḥ ṣi***, the corresponding exceptions on 8.4.41's side. Zero invocations across the corpus, no witness.
- **6.1.76 *padāntād vā***, which makes the tuk optional after a pada-final short vowel. No site in this corpus is pada-final. It would be the engine's eighth vikalpa rule.
- **8.4.41's correspondence side**, still `t`/`T`/`D` only. Unchanged by this slice, still unwitnessed for `d`/`n`/`s`.
- **6.3.111** and **6.1.68**, both deliberately absent, both already documented in place.
- **Splitting `crates/panini/tests/paradigm.rs`**, now past 6,000 lines. Worth doing and its own slice: a large mechanical diff sitting directly next to the data the audit exists to validate is the worst possible neighbour for it.
- **It-stripping as real sūtras** (1.3.2, 1.3.3, 1.3.5, 1.3.9).
