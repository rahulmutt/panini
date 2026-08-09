# Retiring the Conventions Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Retire the two standing one-form-per-cell conventions (7.1.35 tātaṅ,
8.4.56 pausal cartva) and the one divergence an audit found beside them
(3.4.110/111 Śākaṭāyana's jus), so the engine's derivation set equals
vidyut-prakriya's in all 1512 cells with no filter.

**Architecture:** Four new sūtras in `TINANTA_RULES` — 7.1.35 (vikalpa),
8.2.39 (obligatory), 8.4.56 (vikalpa), 3.4.111 (vikalpa) — plus a second arm
on the existing 6.1.96. Each new rule is a self-guarding `Rule` in the stage
its pipeline position falls in, never a branch inside `derive`. 8.2.39 is
obligatory and moves index 0 for 48 cells, so 48 of `PARADIGM`'s strings
change; every other new form lands in `ALTERNATES`, which grows from 8 rows to
154 and gains a column naming the optional rules that produced each row.

**Tech Stack:** Rust (pinned to 1.97.1 via `mise`), workspace crates
`panini-prakriya` (the rule engine), `panini-data` (the root table), `panini`
(integration tests). No new dependencies.

**Spec:** `docs/superpowers/specs/2026-08-09-retiring-the-conventions-design.md`

## Global Constraints

- Toolchain comes from `mise`. Build with `mise run build`, test with
  `mise run test`, lint with `mise run lint`, format with `mise run fmt`.
  Never install Rust globally. To scope tests to one crate use
  `mise exec -- cargo test -p <crate>` — `mise run test -- -p <crate>` does
  **not** scope.
- SLP1 is the only internal representation. All forms in this plan are SLP1.
- `#![forbid(unsafe_code)]` holds in every crate touched here.
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, never as a
  branch inside `derive`.
- Every new rule id must appear in `tinanta_rule_order_is_pinned`
  (`crates/panini-prakriya/src/tinanta/derivation_tests.rs`) **in position**.
- Every new optional rule must appear in
  `exactly_the_pinned_vikalpa_rules_are_optional` in the same file.
- Per-rule guard tests go beside the rule, in its stage file. Tests asserting
  a surface form or a trace go in `derivation_tests.rs` (unit) or
  `crates/panini/tests/trace.rs` (integration).
- Sūtra ids and names are checked against vidyut-prakriya's
  `data/sutrapatha.tsv`. The four used here, verbatim:
  - `7.1.35` → `tuhyostAtaNNASizyanyatarasyAm`
  - `8.2.39` → `JalAM jaSo'nte`
  - `8.4.56` → `vA'vasAne`
  - `3.4.111` → `laNaH SAkawAyanasyEva`
- Guards are written to the **reachable** slice only, so that every arm has a
  test witness and mutation testing stays clean. Widen a guard only when a
  root that needs it lands.
- Branch is `retiring-the-conventions`, already checked out with the spec
  committed. Commit after every task.

---

## File Structure

| File | Responsibility | Change |
| --- | --- | --- |
| `crates/panini/tests/paradigm.rs` | the golden table | `ALTERNATES` gains a column and 146 rows; `PARADIGM` takes 48 flips; one new test |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | tripādī rules (8.2.x–8.4.x) | 8.2.39 and 8.4.56 + their guard tests |
| `crates/panini-prakriya/src/tinanta/tin.rs` | tiṅ-ending substitutions | 7.1.35 + its guard tests |
| `crates/panini-prakriya/src/tinanta/vikarana.rs` | vikaraṇa insertion/luk | 3.4.111 + its guard tests |
| `crates/panini-prakriya/src/tinanta/adesha.rs` | ādeśa rules after 3.1.68 | 6.1.96 gains its junction arm |
| `crates/panini-prakriya/src/tinanta/derivation_tests.rs` | pipeline-wide invariants + helpers | `declined()` helper; order pin; vikalpa pin; 10 call-site conversions |
| `crates/panini/tests/trace.rs` | ordered-trace pins | 5 pins updated, 4 added |
| `AGENTS.md`, `README.md`, `docs/ARCHITECTURE.md` | contributor docs | rewritten claims |

---

### Task 1: `ALTERNATES` gains the vikalpa-key column

Pure refactor: no grammar changes, no new forms. It widens the golden row
shape and adds the test that will police the 146 rows the later tasks add.
Doing it first means those tasks add data, not structure.

**Files:**
- Modify: `crates/panini/tests/paradigm.rs:1794-1816` (the `ALTERNATES`
  const), `:1892-1909` (`every_alternate_validates_and_matches`),
  `:1917-1929` (`every_alternate_names_a_real_cell`), `:1963-1968` (the
  filter closure inside `derivation_set_is_exactly_pinned`)
- Test: `crates/panini/tests/paradigm.rs` (same file — this is a test file)

**Interfaces:**
- Consumes: nothing.
- Produces: `const ALTERNATES: &[(&str, &str, usize, &str, &str)]` — the tuple
  is `(root_id, lakara_label, cell, form, vikalpa_key)`. `vikalpa_key` is the
  `+`-joined, pipeline-ordered list of optional-rule ids applied on the branch
  that derives `form`. Also `const VIKALPA_RULES: &[&str]`, the pinned set of
  optional rule ids, in pipeline order.

- [ ] **Step 1: Write the failing test**

Add to `crates/panini/tests/paradigm.rs`, immediately after
`every_alternate_names_a_real_cell`:

```rust
/// The optional rules, in pipeline order. Mirrors
/// `exactly_the_pinned_vikalpa_rules_are_optional` in `panini-prakriya`;
/// duplicated here rather than exported because this is an integration test
/// and the rule table is crate-internal.
const VIKALPA_RULES: &[&str] = &["6.4.107"];

/// `ALTERNATES` is otherwise 154 bare strings, and a string can be right for
/// the wrong reason — `BavatAt` is a real form whether or not 8.4.56 is what
/// produced it. This ties each row to the grammar: find the branch that
/// derives the row's form, intersect its log with the optional-rule set, and
/// require exactly the rules the row claims.
#[test]
fn every_alternate_names_the_vikalpa_rules_that_produced_it() {
    for (root, lakara, cell, form, key) in ALTERNATES {
        let d = dhatus().iter().find(|d| d.id == *root).unwrap();
        let (pu, va) = CELLS[*cell];
        let lak = *LAKARA_BY_NAME
            .iter()
            .find_map(|(n, l)| (n == lakara).then_some(l))
            .unwrap();
        let branch = derive(d, lak, d.pada, pu, va)
            .into_iter()
            .find(|p| !p.blocked && p.text() == *form)
            .unwrap_or_else(|| {
                panic!("no branch of {root} {lakara} cell {cell} derives {form}")
            });
        let applied: Vec<&str> = branch
            .log
            .iter()
            .map(|s| s.sutra.as_str())
            .filter(|s| VIKALPA_RULES.contains(s))
            .collect();
        assert_eq!(
            applied.join("+"),
            *key,
            "{form} ({root} {lakara} cell {cell})"
        );
    }
}
```

- [ ] **Step 2: Run it to make sure it fails**

Run: `mise exec -- cargo test -p panini --test paradigm every_alternate_names_the_vikalpa`

Expected: FAIL to **compile** — `ALTERNATES` rows are 4-tuples, so the
`for (root, lakara, cell, form, key)` destructuring does not match.

- [ ] **Step 3: Widen the row shape**

In `crates/panini/tests/paradigm.rs`, replace the `ALTERNATES` const
(including its doc comment) with:

```rust
/// Second and third valid forms, for cells where an optional (vikalpa) rule
/// forks the derivation. `(root_id, lakara_label, cell index into the
/// [&str; 9], alternate form, vikalpa key)`.
///
/// The vikalpa key names the optional rules applied on the branch that
/// derives this form, `+`-joined in pipeline order. It is not decoration:
/// `every_alternate_names_the_vikalpa_rules_that_produced_it` checks it
/// against the branch's own log, so a right form reached by the wrong rule
/// fails here.
///
/// `PARADIGM` holds index 0 — the derivation with no optional rule applied —
/// so an alternate is by construction never `PARADIGM`'s own string.
/// Cell order is [P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B], so 7 and 8
/// are uttama dvi and uttama bahu.
const ALTERNATES: &[(&str, &str, usize, &str, &str)] = &[
    ("hi", "laT", 7, "hinvaH", "6.4.107"),
    ("hi", "laT", 8, "hinmaH", "6.4.107"),
    ("hi", "laN", 7, "ahinva", "6.4.107"),
    ("hi", "laN", 8, "ahinma", "6.4.107"),
    ("ri", "laT", 7, "riRvaH", "6.4.107"),
    ("ri", "laT", 8, "riRmaH", "6.4.107"),
    ("ri", "laN", 7, "ariRva", "6.4.107"),
    ("ri", "laN", 8, "ariRma", "6.4.107"),
];
```

- [ ] **Step 4: Update the three existing consumers**

In `every_alternate_validates_and_matches`, change the loop header:

```rust
    for (root, lakara, _cell, form, _key) in ALTERNATES {
```

In `every_alternate_names_a_real_cell`, change the loop header:

```rust
    for (root, lakara, cell, form, _key) in ALTERNATES {
```

In `derivation_set_is_exactly_pinned`, change the filter and map:

```rust
            want.extend(
                ALTERNATES
                    .iter()
                    .filter(|(r, l, c, _, _)| r == root && l == lakara && *c == cell)
                    .map(|(_, _, _, f, _)| (*f).to_string()),
            );
```

- [ ] **Step 5: Run the paradigm tests**

Run: `mise exec -- cargo test -p panini --test paradigm`

Expected: PASS, all tests, including the new one (the eight 6.4.107 rows each
have exactly `6.4.107` in their branch's log).

- [ ] **Step 6: Lint and format**

Run: `mise run fmt && mise run lint`

Expected: clean.

- [ ] **Step 7: Commit**

```bash
git add crates/panini/tests/paradigm.rs
git commit -m "test(paradigm): ALTERNATES rows name the vikalpa rules that produced them"
```

---

### Task 2: 8.2.39 *jhalāṁ jaśo'nte* and 8.4.56 *vāvasāne*

The pair lands together. 8.2.39 alone would voice 48 finals with nothing to
restore them; 8.4.56 alone would have no voiced final to devoice. This is also
the task that moves `PARADIGM`.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tripadi.rs` (insert 8.2.39
  after the 8.2.25 rule ending at `:189`, and 8.4.56 after the 8.4.2 rule
  ending at `:391`)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (the
  `declined` helper; `tinanta_rule_order_is_pinned`;
  `exactly_the_pinned_vikalpa_rules_are_optional`; 4 call sites)
- Modify: `crates/panini/tests/paradigm.rs` (48 `PARADIGM` flips, 48
  `ALTERNATES` rows, `VIKALPA_RULES`, non-form pins)
- Modify: `crates/panini/tests/trace.rs` (5 pins)
- Test: guard tests in `tripadi.rs`'s own `mod tests`

**Interfaces:**
- Consumes: `is_jhal(c: char) -> bool` and `cartva_of(c: char) -> Option<char>`
  from `crate::tinanta::sound` (both already imported by `tripadi.rs`);
  `ALTERNATES`'s 5-tuple shape and `VIKALPA_RULES` from Task 1.
- Produces: `pub(super) fn declined(branches: Vec<Prakriya>, expected: usize)
  -> Prakriya` in `derivation_tests.rs` — returns branch 0 after asserting the
  branch count is exactly `expected`. Re-exported from `mod.rs` alongside
  `form_g` so stage files can use it.

- [ ] **Step 1: Write the failing guard tests for both rules**

Append to the `mod tests` block at the end of
`crates/panini-prakriya/src/tinanta/tripadi.rs` (line 395). It already imports
`Prakriya`, `Term` and `rules`, which is all these tests need.

```rust
    /// 8.2.39 voices a pada-final `t` and nothing else. The `s` case belongs
    /// to its apavāda 8.2.66 (implemented inside the rule labelled 8.3.15),
    /// and a `t` that is not pada-final is untouched.
    #[test]
    fn jhalam_jasho_ante_fires_only_on_a_pada_final_t() {
        let rule = rules().find(|r| r.id == "8.2.39").unwrap();

        let mut p = Prakriya {
            terms: vec![Term::new("aBav"), Term::new("a"), Term::new("t")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "aBavad");

        // not pada-final: the `t` is followed by more of the ending
        let mut p = Prakriya {
            terms: vec![Term::new("aBav"), Term::new("a"), Term::new("tAm")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        // `s`-final belongs to 8.2.66/8.3.15, not here
        let mut p = Prakriya {
            terms: vec![Term::new("aBav"), Term::new("a"), Term::new("s")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        // vowel-final
        let mut p = Prakriya {
            terms: vec![Term::new("Bav"), Term::new("a"), Term::new("ti")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
    }

    /// 8.4.56 devoices a pada-final jhal. After 8.2.39 the only reachable
    /// one is `d`; a vowel, a visarga and a nasal all decline.
    #[test]
    fn va_avasane_fires_only_on_a_pada_final_jhal() {
        let rule = rules().find(|r| r.id == "8.4.56").unwrap();

        let mut p = Prakriya {
            terms: vec![Term::new("aBav"), Term::new("a"), Term::new("d")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "aBavat");

        let mut p = Prakriya {
            terms: vec![Term::new("aBav"), Term::new("a"), Term::new("H")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        let mut p = Prakriya {
            terms: vec![Term::new("aBav"), Term::new("a"), Term::new("m")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        let mut p = Prakriya {
            terms: vec![Term::new("Bav"), Term::new("a"), Term::new("ti")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
    }
```

- [ ] **Step 2: Run them to make sure they fail**

Run: `mise exec -- cargo test -p panini-prakriya tripadi`

Expected: FAIL — `rules().find(|r| r.id == "8.2.39").unwrap()` panics on
`None`, because neither rule exists yet.

- [ ] **Step 3: Add 8.2.39, immediately after the 8.2.25 rule**

In `crates/panini-prakriya/src/tinanta/tripadi.rs`, insert between the
8.2.25 `Rule { … }` (which ends at line 189 with `},`) and the
`// 8.2.66 sasajuṣo ruḥ + 8.3.15 …` comment:

```rust
    // 8.2.39 jhalāṁ jaśo'nte: a pada-final jhal becomes its jaś (voiced
    // unaspirated). This is what makes `aBavad` the engine's DECLINED form —
    // it is obligatory, and 8.4.56 below optionally undoes it. Before this
    // rule existed the pipeline simply never voiced a final, which is why
    // the goldens read `aBavat` and the repo carried a "drop the pausal d"
    // convention.
    //
    // NARROW GUARD, by design, as with 8.3.59 and 8.2.25: the only jhal
    // reachable pada-finally in this suite is `t` (every other form ends in
    // a vowel, `H`, `m` or `n`, none of them jhal). The other candidate is
    // `s`, and 8.2.66 sasajuṣo ruḥ — implemented inside the rule labelled
    // 8.3.15 just below — is its apavāda, so `s` must NOT be voiced here.
    // Widen the moment a root lands whose pada-final sound is another jhal.
    //
    // No contention with 8.4.55 cartva: the shape that would collide, an
    // aṅga-final jhal directly before a pada-final `t`, cannot arise because
    // 8.2.23 saṁyogāntasya lopaḥ sits above and drops the second consonant
    // first. √ad, the one root whose aṅga ends in a jhal, presents `Adat` —
    // a vowel before the ending.
    Rule {
        id: "8.2.39",
        name: "JalAM jaSo'nte",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.text().ends_with('t') {
                return false;
            }
            // Read the bearing term positionally rather than as ENDING:
            // 6.4.105 / 6.4.106 luk the ending outright (Bava, hinu), so
            // that index is not reliably the last non-empty one.
            let Some(idx) = p.terms.iter().rposition(|t| !t.text.is_empty()) else {
                return false;
            };
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            s.push('d');
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.2.39", "JalAM jaSo'nte", before);
            true
        },
    },
```

- [ ] **Step 4: Add 8.4.56 as the last rule in the stage**

In the same file, insert after the 8.4.2 `Rule { … }` (ending at line 391 with
`},`) and before the closing `];`:

```rust
    // 8.4.56 vāvasāne: at the end of an utterance a jhal OPTIONALLY becomes
    // its car, continuing khari ca's operation. After 8.2.39 the only
    // reachable jhal-final is `d`, so in practice this restores the `t` that
    // 8.2.39 voiced — which is exactly the relationship the sūtras state,
    // and why `aBavat` is now an alternate rather than the pinned form.
    //
    // LAST rule in the pipeline, deliberately. Avasāna is the end of the
    // utterance, so the rule must see the finished word; and being last, it
    // satisfies the ordering constraint on optional rules trivially, since
    // no consumer sits below it at all.
    Rule {
        id: "8.4.56",
        name: "vA'vasAne",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            let Some(last) = p.text().chars().last() else {
                return false;
            };
            if !is_jhal(last) {
                return false;
            }
            let Some(sub) = cartva_of(last) else {
                return false;
            };
            if sub == last {
                return false;
            }
            let Some(idx) = p.terms.iter().rposition(|t| !t.text.is_empty()) else {
                return false;
            };
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            s.push(sub);
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.4.56", "vA'vasAne", before);
            true
        },
    },
```

- [ ] **Step 5: Run the guard tests**

Run: `mise exec -- cargo test -p panini-prakriya tripadi`

Expected: the two new guard tests PASS. Other tests in the crate will now
fail — that is the next step's work.

- [ ] **Step 6: Pin the rule order and the vikalpa set**

In `crates/panini-prakriya/src/tinanta/derivation_tests.rs`, in
`tinanta_rule_order_is_pinned`, change the tail of `expected` from:

```rust
        "6.4.77", "6.1.78", "7.3.101", "6.4.112", "6.4.113", "6.1.101", "6.1.96", "6.1.90",
        "6.1.97", "6.1.87", "6.1.66", "6.4.105", "6.4.106", "6.4.107", "6.4.101", "8.2.77",
        "8.2.23", "8.2.25", "8.3.15", "8.3.59", "8.4.55", "8.4.1", "8.4.2",
```

to:

```rust
        "6.4.77", "6.1.78", "7.3.101", "6.4.112", "6.4.113", "6.1.101", "6.1.96", "6.1.90",
        "6.1.97", "6.1.87", "6.1.66", "6.4.105", "6.4.106", "6.4.107", "6.4.101", "8.2.77",
        "8.2.23", "8.2.25", "8.2.39", "8.3.15", "8.3.59", "8.4.55", "8.4.1", "8.4.2", "8.4.56",
```

In `exactly_the_pinned_vikalpa_rules_are_optional`, change:

```rust
    let expected = ["6.4.107"];
```

to:

```rust
    let expected = ["6.4.107", "8.4.56"];
```

- [ ] **Step 7: Add the `declined` helper and convert the four forking call sites**

In `crates/panini-prakriya/src/tinanta/derivation_tests.rs`, immediately after
the `sole` function, add:

```rust
/// Unwrap a derivation that IS expected to fork, asserting the branch count.
///
/// `sole` stays the default so an unexpected fork fails loudly; this is its
/// counterpart for cells an optional rule legitimately forks. It returns
/// branch 0 — the declined derivation, i.e. what the pipeline produces with
/// no optional rule applied — and still fails if the count is not exactly
/// what the caller expects, so an over-firing optional rule cannot hide here.
pub(super) fn declined(branches: Vec<Prakriya>, expected: usize) -> Prakriya {
    assert_eq!(
        branches.len(),
        expected,
        "expected {expected} derivations, got {}: {:?}",
        branches.len(),
        branches.iter().map(|p| p.text()).collect::<Vec<_>>()
    );
    branches.into_iter().next().unwrap()
}

/// `form_g` for a cell an optional rule forks: same lookup, `declined`
/// instead of `sole`.
pub(super) fn form_g_forked(
    code: &str,
    la: Lakara,
    pu: Purusha,
    va: Vacana,
    branches: usize,
) -> String {
    let d = dhatus().iter().find(|d| d.id == code).unwrap();
    declined(derive(d, la, d.pada, pu, va), branches).text()
}
```

Then convert the four call sites this task's rules fork. Each becomes a
`form_g_forked` call with `2` branches, and each expected form gains its `d`:

| line | was | becomes |
| --- | --- | --- |
| `:200` | `form_g("kup", Lakara::VidhiLin, Purusha::Prathama, Vacana::Eka)` → `"kupyet"` | `form_g_forked("kup", Lakara::VidhiLin, Purusha::Prathama, Vacana::Eka, 2)` → `"kupyed"` |
| `:218` | `form_g("div", Lakara::Lan, Purusha::Prathama, Vacana::Eka)` → `"adIvyat"` | `form_g_forked("div", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2)` → `"adIvyad"` |
| `:241` | `form_g("yA", Lakara::Lan, Purusha::Prathama, Vacana::Eka)` → `"ayAt"` | `form_g_forked("yA", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2)` → `"ayAd"` |
| `:348` | `form_g("ad", Lakara::Lan, Purusha::Prathama, Vacana::Eka)` → `"Adat"` | `form_g_forked("ad", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2)` → `"Adad"` |

Update each assertion's expected string to the `d`-form shown, and update the
comment above each test if it names the old form.

- [ ] **Step 8: Run the crate's tests**

Run: `mise exec -- cargo test -p panini-prakriya`

Expected: PASS. If a test not in the table above fails with "expected exactly
one derivation, got 2", it is another forking cell — convert it the same way
(`form_g_forked`, 2 branches, `d`-form), and note it in the commit message.

- [ ] **Step 9: Flip the 48 `PARADIGM` cells**

In `crates/panini/tests/paradigm.rs`, in each of the 48 blocks below, change
the **first** string of the nine (cell 0, prathama eka):

| root | lakāra | was | becomes |
| --- | --- | --- | --- |
| BU | laN | aBavat | aBavad |
| nI | laN | anayat | anayad |
| ji | laN | ajayat | ajayad |
| smf | laN | asmarat | asmarad |
| paW | laN | apaWat | apaWad |
| vad | laN | avadat | avadad |
| BU | viDiliN | Bavet | Baved |
| nI | viDiliN | nayet | nayed |
| ji | viDiliN | jayet | jayed |
| smf | viDiliN | smaret | smared |
| paW | viDiliN | paWet | paWed |
| vad | viDiliN | vadet | vaded |
| div | laN | adIvyat | adIvyad |
| naS | laN | anaSyat | anaSyad |
| kup | laN | akupyat | akupyad |
| tud | laN | atudat | atudad |
| liK | laN | aliKat | aliKad |
| viS | laN | aviSat | aviSad |
| div | viDiliN | dIvyet | dIvyed |
| naS | viDiliN | naSyet | naSyed |
| kup | viDiliN | kupyet | kupyed |
| tud | viDiliN | tudet | tuded |
| liK | viDiliN | liKet | liKed |
| viS | viDiliN | viSet | viSed |
| yA | laN | ayAt | ayAd |
| vA | laN | avAt | avAd |
| yA | viDiliN | yAyAt | yAyAd |
| vA | viDiliN | vAyAt | vAyAd |
| ad | laN | Adat | Adad |
| ad | viDiliN | adyAt | adyAd |
| kliS | laN | akliSnAt | akliSnAd |
| kliS | viDiliN | kliSnIyAt | kliSnIyAd |
| guD | laN | aguDnAt | aguDnAd |
| guD | viDiliN | guDnIyAt | guDnIyAd |
| aS | laN | ASnAt | ASnAd |
| aS | viDiliN | aSnIyAt | aSnIyAd |
| muz | laN | amuzRAt | amuzRAd |
| muz | viDiliN | muzRIyAt | muzRIyAd |
| vrI | laN | avrIRAt | avrIRAd |
| vrI | viDiliN | vrIRIyAt | vrIRIyAd |
| Ap | laN | Apnot | Apnod |
| Ap | viDiliN | ApnuyAt | ApnuyAd |
| Sak | laN | aSaknot | aSaknod |
| Sak | viDiliN | SaknuyAt | SaknuyAd |
| hi | laN | ahinot | ahinod |
| hi | viDiliN | hinuyAt | hinuyAd |
| ri | laN | ariRot | ariRod |
| ri | viDiliN | riRuyAt | riRuyAd |

The `aS` rows here are kryādi's `aS` (`Dhatu::id == "aS"`), not svādi's
`aS.5`, which is ātmanepada and has no `t`-final cell. Leave every other
string in the file untouched: exactly 48 of 1512 change.

- [ ] **Step 10: Add the 48 `8.4.56` rows to `ALTERNATES` and widen `VIKALPA_RULES`**

In `crates/panini/tests/paradigm.rs`, change:

```rust
const VIKALPA_RULES: &[&str] = &["6.4.107"];
```

to:

```rust
const VIKALPA_RULES: &[&str] = &["6.4.107", "8.4.56"];
```

and add these rows to `ALTERNATES` (order within the const does not matter to
any test; keeping them grouped by rule reads best):

```rust
    ("BU", "laN", 0, "aBavat", "8.4.56"),
    ("nI", "laN", 0, "anayat", "8.4.56"),
    ("ji", "laN", 0, "ajayat", "8.4.56"),
    ("smf", "laN", 0, "asmarat", "8.4.56"),
    ("paW", "laN", 0, "apaWat", "8.4.56"),
    ("vad", "laN", 0, "avadat", "8.4.56"),
    ("BU", "viDiliN", 0, "Bavet", "8.4.56"),
    ("nI", "viDiliN", 0, "nayet", "8.4.56"),
    ("ji", "viDiliN", 0, "jayet", "8.4.56"),
    ("smf", "viDiliN", 0, "smaret", "8.4.56"),
    ("paW", "viDiliN", 0, "paWet", "8.4.56"),
    ("vad", "viDiliN", 0, "vadet", "8.4.56"),
    ("div", "laN", 0, "adIvyat", "8.4.56"),
    ("naS", "laN", 0, "anaSyat", "8.4.56"),
    ("kup", "laN", 0, "akupyat", "8.4.56"),
    ("tud", "laN", 0, "atudat", "8.4.56"),
    ("liK", "laN", 0, "aliKat", "8.4.56"),
    ("viS", "laN", 0, "aviSat", "8.4.56"),
    ("div", "viDiliN", 0, "dIvyet", "8.4.56"),
    ("naS", "viDiliN", 0, "naSyet", "8.4.56"),
    ("kup", "viDiliN", 0, "kupyet", "8.4.56"),
    ("tud", "viDiliN", 0, "tudet", "8.4.56"),
    ("liK", "viDiliN", 0, "liKet", "8.4.56"),
    ("viS", "viDiliN", 0, "viSet", "8.4.56"),
    ("yA", "laN", 0, "ayAt", "8.4.56"),
    ("vA", "laN", 0, "avAt", "8.4.56"),
    ("yA", "viDiliN", 0, "yAyAt", "8.4.56"),
    ("vA", "viDiliN", 0, "vAyAt", "8.4.56"),
    ("ad", "laN", 0, "Adat", "8.4.56"),
    ("ad", "viDiliN", 0, "adyAt", "8.4.56"),
    ("kliS", "laN", 0, "akliSnAt", "8.4.56"),
    ("kliS", "viDiliN", 0, "kliSnIyAt", "8.4.56"),
    ("guD", "laN", 0, "aguDnAt", "8.4.56"),
    ("guD", "viDiliN", 0, "guDnIyAt", "8.4.56"),
    ("aS", "laN", 0, "ASnAt", "8.4.56"),
    ("aS", "viDiliN", 0, "aSnIyAt", "8.4.56"),
    ("muz", "laN", 0, "amuzRAt", "8.4.56"),
    ("muz", "viDiliN", 0, "muzRIyAt", "8.4.56"),
    ("vrI", "laN", 0, "avrIRAt", "8.4.56"),
    ("vrI", "viDiliN", 0, "vrIRIyAt", "8.4.56"),
    ("Ap", "laN", 0, "Apnot", "8.4.56"),
    ("Ap", "viDiliN", 0, "ApnuyAt", "8.4.56"),
    ("Sak", "laN", 0, "aSaknot", "8.4.56"),
    ("Sak", "viDiliN", 0, "SaknuyAt", "8.4.56"),
    ("hi", "laN", 0, "ahinot", "8.4.56"),
    ("hi", "viDiliN", 0, "hinuyAt", "8.4.56"),
    ("ri", "laN", 0, "ariRot", "8.4.56"),
    ("ri", "viDiliN", 0, "riRuyAt", "8.4.56"),
```

- [ ] **Step 11: Add the non-form pins for these two guards**

Determine each pin by actually breaking the guard and recording the output —
do not guess. For each of the four mutations below, apply it, run
`mise exec -- cargo test -p panini --test paradigm derivation_set`, read the
form the failure reports, revert the mutation, and add that form to
`known_nonforms_are_invalid` with a comment naming the mutation:

1. 8.2.39's `p.text().ends_with('t')` → `p.text().contains('t')` (fires on a
   non-final `t`).
2. 8.2.39's `s.push('d')` → `s.push('D')` (wrong jaś substitute).
3. 8.4.56's `is_jhal(last)` guard removed (fires on a non-jhal final).
4. 8.4.56's `vikalpa: true` → `false` (the devoicing becomes obligatory, so
   the `d`-forms vanish rather than an extra form appearing).

Mutation 4 removes a form rather than adding one, so it is caught by
`derivation_set_is_exactly_pinned`, not by a non-form pin — record that in the
comment rather than inventing a pin for it.

- [ ] **Step 12: Update the five exact-vector trace pins**

In `crates/panini/tests/trace.rs`, append `"8.2.39", "8.4.56"` to the expected
vector of each of these five tests. `ashnat_trace_takes_the_vowel_initial_anga_augment`
uses `contains` assertions and needs no change.

```rust
// abhavat_trace_is_exactly_the_lan_augment_path
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.100", "3.1.68", "1.3.9", "6.4.71", "7.3.84",
            "6.1.78", "8.2.39", "8.4.56"
        ]

// adat_trace_a_augment_precedes_and_blocks_cartva
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.100", "3.1.68", "1.3.9", "2.4.72", "6.4.72",
            "7.3.100", "6.1.90", "8.2.39", "8.4.56"
        ]

// akupyat_trace_shows_7_3_100_declines_for_non_adadi_roots
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.100", "3.1.69", "1.3.9", "1.2.4", "6.4.71",
            "8.2.39", "8.4.56"
        ]

// bhavet_trace_is_exactly_the_vidhilin_vali_lopa_path
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.100", "3.4.103", "3.1.68", "1.3.9", "7.2.79",
            "7.2.80", "7.3.84", "6.1.78", "6.1.87", "6.1.66", "8.2.39", "8.4.56"
        ]

// kupyet_trace_is_exactly_the_syan_vidhilin_path
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.100", "3.4.103", "3.1.69", "1.3.9", "1.2.4",
            "7.2.79", "7.2.80", "6.1.87", "6.1.66", "8.2.39", "8.4.56"
        ]
```

Each of these five now names the **forked** branch — `trace_for` matches on
the surface form, and the `t`-final form is now the branch on which 8.4.56
fired. Add a one-line comment to that effect above each vector.

- [ ] **Step 13: Run the whole suite**

Run: `mise run test`

Expected: PASS.

- [ ] **Step 14: Lint, format, commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/tripadi.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs \
        crates/panini/tests/paradigm.rs crates/panini/tests/trace.rs
git commit -m "feat(prakriya): 8.2.39 jhalAM jaSo'nte and 8.4.56 vA'vasAne"
```

---

### Task 3: 7.1.35 *tuhyos tātaṅ*

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/tin.rs` (insert after the
  3.4.102 rule ending at `:467`, i.e. last in the `TIN` stage)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (order
  pin; vikalpa pin; 6 call sites)
- Modify: `crates/panini/tests/paradigm.rs` (96 `ALTERNATES` rows,
  `VIKALPA_RULES`, non-form pins)
- Modify: `crates/panini/tests/trace.rs` (3 new pins)
- Test: guard tests in `tin.rs`'s own test module

**Interfaces:**
- Consumes: `Tag::Pit` and `Tag::Ngit` from `crate::term` (already imported by
  `tin.rs`); `ENDING_PRE_SHAP` (already imported); `form_g_forked` and
  `declined` from Task 2.
- Produces: no new public items — the rule is a `TIN` entry.

- [ ] **Step 1: Write the failing guard test**

Append to `tin.rs`'s test module (line 471).

```rust
    /// 7.1.35 replaces `tu` and `hi` — and only those — with tātaṅ, whose
    /// ṅ is a real it-marker. The tag work is the point: `tu` arrives pit
    /// from 3.4.78 and must not keep that alongside the new ṅit, or 7.3.84
    /// guṇates where 1.1.5 should block.
    #[test]
    fn tatan_replaces_only_tu_and_hi_and_lands_ngit() {
        let rule = rules().find(|r| r.id == "7.1.35").unwrap();

        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("tu")],
            ..Default::default()
        };
        p.terms[ENDING_PRE_SHAP].add(Tag::Pit);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "tAt");
        assert!(p.terms[ENDING_PRE_SHAP].has(Tag::Ngit));
        assert!(!p.terms[ENDING_PRE_SHAP].has(Tag::Pit));

        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("hi")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "tAt");
        assert!(p.terms[ENDING_PRE_SHAP].has(Tag::Ngit));

        for ending in ["ti", "te", "tAm", "sva", "si", "mi", "Ji"] {
            let mut p = Prakriya {
                terms: vec![Term::new("BU"), Term::new(ending)],
                ..Default::default()
            };
            assert!(!(rule.apply)(&mut p), "7.1.35 fired on {ending}");
        }
    }
```

`tin.rs`'s test module imports `super::*` (which brings `Tag` and
`ENDING_PRE_SHAP`); add `use crate::prakriya::Prakriya; use crate::term::Term;
use crate::tinanta::rules;` if they are not already there.

- [ ] **Step 2: Run it to make sure it fails**

Run: `mise exec -- cargo test -p panini-prakriya tatan`

Expected: FAIL — `rules().find(|r| r.id == "7.1.35").unwrap()` panics.

- [ ] **Step 3: Add the rule, last in the `TIN` stage**

In `crates/panini-prakriya/src/tinanta/tin.rs`, insert after the 3.4.102
`Rule { … }` and before the closing `];`:

```rust
    // 7.1.35 tuhyos tātaṅ āśiṣy anyatarasyām: `tu` and `hi` are OPTIONALLY
    // replaced by tātaṅ. Both endings occur only in loṭ parasmaipada, so
    // reading the ending is necessary and sufficient — no lakāra test.
    //
    // tātaṅ's ṅ is a real it-marker, not 1.2.4's atideśa, and the ṅitva is
    // what earns the forms: Apnotu guṇates śnu through 7.3.84's second
    // application because `tu` is pit, ApnutAt does not because 1.1.5
    // blocks it. Bavatu -> BavatAt KEEPS its guṇa, which is śap-relative and
    // untouched by the ending's ṅitva — the two applications of 7.3.84 that
    // svādi forced apart, seen from the other side.
    //
    // ORDERING, invisible and permanent: this rule MUST sit above every rule
    // that reads the ending `hi` — 3.1.83 halaḥ śnaḥ śānac ca (or kryādi's
    // tātaṅ branch surfaces *kliSAnatAt instead of kliSnItAt), 6.4.105 ato
    // heḥ, and 6.4.106 utaś ca — and above 7.3.84, whose second application
    // reads the ending's ṅitva. It is above all of them here, at the end of
    // the tiṅ stage, and nothing enforces that but the kliSnItAt trace pin.
    //
    // Note this is the opposite of 6.4.107's ordering constraint. 6.4.107
    // destroys the EVIDENCE for a predicate without changing the fact, so
    // its consumers must sit above it; 7.1.35 changes the fact itself — the
    // ending genuinely is no longer `hi` — so its consumers must sit below.
    //
    // The sūtra's āśiṣi (benedictive sense) is a semantic condition the
    // engine cannot evaluate and deliberately ignores: `check` answers "is
    // this derivable within the covered grammar", not "is this the right
    // word here". Every form admitted is a real Sanskrit form.
    Rule {
        id: "7.1.35",
        name: "tuhyostAtaNNASizyanyatarasyAm",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            let e = p.terms[ENDING_PRE_SHAP].text.as_str();
            if e != "tu" && e != "hi" {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "tAtaN".into();
            p.record("7.1.35", "tuhyostAtaNNASizyanyatarasyAm", before);
            // The it-stripping is recorded separately, as 3.4.108 does for
            // jus -> us. `tu` arrived pit from 3.4.78; clear that before
            // adding the ṅit, or the term claims both — the same two-line
            // shape 3.4.87 uses for hi.
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "tAt".into();
            p.terms[ENDING_PRE_SHAP].remove(Tag::Pit);
            p.terms[ENDING_PRE_SHAP].add(Tag::Ngit);
            p.record("1.3.9", "tasya lopaH", before);
            true
        },
    },
```

- [ ] **Step 4: Run the guard test**

Run: `mise exec -- cargo test -p panini-prakriya tatan`

Expected: PASS.

- [ ] **Step 5: Pin the order and the vikalpa set**

In `derivation_tests.rs`, in `tinanta_rule_order_is_pinned`, insert `"7.1.35"`
immediately after `"3.4.102"`:

```rust
        "3.4.93", "3.4.90", "3.4.92", "3.4.103", "3.4.102", "7.1.35", "3.1.69", "3.1.73",
        "3.1.77", "3.1.81", "3.1.68", "2.4.72", "3.1.83", "1.2.4", "6.4.71", "6.4.72",
```

(Reflow the surrounding lines as `cargo fmt` dictates; the requirement is the
id's position, not the line breaks.)

In `exactly_the_pinned_vikalpa_rules_are_optional`:

```rust
    let expected = ["7.1.35", "6.4.107", "8.4.56"];
```

The order here is pipeline order, and 7.1.35 is in the tiṅ stage, above both
others.

- [ ] **Step 6: Convert the six loṭ call sites**

These cells now have **three** branches (base, tātaṅ+8.2.39, tātaṅ+8.4.56).
In `crates/panini-prakriya/src/tinanta/derivation_tests.rs`:

| line | was | becomes |
| --- | --- | --- |
| `:205` | `form_g("naS", Lakara::Lot, Purusha::Madhyama, Vacana::Eka)` | `form_g_forked("naS", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3)` |
| `:246` | `form_g("yA", Lakara::Lot, Purusha::Madhyama, Vacana::Eka)` | `form_g_forked("yA", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3)` |
| `:333` | `form_g("ad", Lakara::Lot, Purusha::Madhyama, Vacana::Eka)` | `form_g_forked("ad", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3)` |
| `:338` | `form_g("BU", Lakara::Lot, Purusha::Madhyama, Vacana::Eka)` | `form_g_forked("BU", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3)` |
| `:892` | `form_g("ad", Lakara::Lot, Purusha::Madhyama, Vacana::Eka)` | `form_g_forked("ad", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3)` |
| `:953` | `form_g("yA", Lakara::Lot, Purusha::Madhyama, Vacana::Eka)` | `form_g_forked("yA", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3)` |

The **expected strings do not change** — branch 0 is still `naSya`, `yAhi`,
`adDi`, `Bava`. Only the helper and the branch count change.

Also check `tripadi.rs:` two `form_g("As", …)` / `form_g("laB", …)` loṭ
madhyama eka call sites: both roots are ātmanepada, whose loṭ 2sg is `sva`,
so 7.1.35 declines and they need no change. Leave them.

- [ ] **Step 7: Run the crate's tests**

Run: `mise exec -- cargo test -p panini-prakriya`

Expected: PASS. Any remaining "expected exactly one derivation, got 3" is
another parasmaipada loṭ prathama/madhyama eka cell — convert it the same way.

- [ ] **Step 8: Add the 96 `ALTERNATES` rows and widen `VIKALPA_RULES`**

In `crates/panini/tests/paradigm.rs`:

```rust
const VIKALPA_RULES: &[&str] = &["7.1.35", "6.4.107", "8.4.56"];
```

The order is pipeline order, so a branch with both fires produces the key
`"7.1.35+8.4.56"`.

Add these rows:

```rust
    ("BU", "loT", 0, "BavatAd", "7.1.35"),
    ("BU", "loT", 0, "BavatAt", "7.1.35+8.4.56"),
    ("BU", "loT", 3, "BavatAd", "7.1.35"),
    ("BU", "loT", 3, "BavatAt", "7.1.35+8.4.56"),
    ("nI", "loT", 0, "nayatAd", "7.1.35"),
    ("nI", "loT", 0, "nayatAt", "7.1.35+8.4.56"),
    ("nI", "loT", 3, "nayatAd", "7.1.35"),
    ("nI", "loT", 3, "nayatAt", "7.1.35+8.4.56"),
    ("ji", "loT", 0, "jayatAd", "7.1.35"),
    ("ji", "loT", 0, "jayatAt", "7.1.35+8.4.56"),
    ("ji", "loT", 3, "jayatAd", "7.1.35"),
    ("ji", "loT", 3, "jayatAt", "7.1.35+8.4.56"),
    ("smf", "loT", 0, "smaratAd", "7.1.35"),
    ("smf", "loT", 0, "smaratAt", "7.1.35+8.4.56"),
    ("smf", "loT", 3, "smaratAd", "7.1.35"),
    ("smf", "loT", 3, "smaratAt", "7.1.35+8.4.56"),
    ("paW", "loT", 0, "paWatAd", "7.1.35"),
    ("paW", "loT", 0, "paWatAt", "7.1.35+8.4.56"),
    ("paW", "loT", 3, "paWatAd", "7.1.35"),
    ("paW", "loT", 3, "paWatAt", "7.1.35+8.4.56"),
    ("vad", "loT", 0, "vadatAd", "7.1.35"),
    ("vad", "loT", 0, "vadatAt", "7.1.35+8.4.56"),
    ("vad", "loT", 3, "vadatAd", "7.1.35"),
    ("vad", "loT", 3, "vadatAt", "7.1.35+8.4.56"),
    ("div", "loT", 0, "dIvyatAd", "7.1.35"),
    ("div", "loT", 0, "dIvyatAt", "7.1.35+8.4.56"),
    ("div", "loT", 3, "dIvyatAd", "7.1.35"),
    ("div", "loT", 3, "dIvyatAt", "7.1.35+8.4.56"),
    ("naS", "loT", 0, "naSyatAd", "7.1.35"),
    ("naS", "loT", 0, "naSyatAt", "7.1.35+8.4.56"),
    ("naS", "loT", 3, "naSyatAd", "7.1.35"),
    ("naS", "loT", 3, "naSyatAt", "7.1.35+8.4.56"),
    ("kup", "loT", 0, "kupyatAd", "7.1.35"),
    ("kup", "loT", 0, "kupyatAt", "7.1.35+8.4.56"),
    ("kup", "loT", 3, "kupyatAd", "7.1.35"),
    ("kup", "loT", 3, "kupyatAt", "7.1.35+8.4.56"),
    ("tud", "loT", 0, "tudatAd", "7.1.35"),
    ("tud", "loT", 0, "tudatAt", "7.1.35+8.4.56"),
    ("tud", "loT", 3, "tudatAd", "7.1.35"),
    ("tud", "loT", 3, "tudatAt", "7.1.35+8.4.56"),
    ("liK", "loT", 0, "liKatAd", "7.1.35"),
    ("liK", "loT", 0, "liKatAt", "7.1.35+8.4.56"),
    ("liK", "loT", 3, "liKatAd", "7.1.35"),
    ("liK", "loT", 3, "liKatAt", "7.1.35+8.4.56"),
    ("viS", "loT", 0, "viSatAd", "7.1.35"),
    ("viS", "loT", 0, "viSatAt", "7.1.35+8.4.56"),
    ("viS", "loT", 3, "viSatAd", "7.1.35"),
    ("viS", "loT", 3, "viSatAt", "7.1.35+8.4.56"),
    ("yA", "loT", 0, "yAtAd", "7.1.35"),
    ("yA", "loT", 0, "yAtAt", "7.1.35+8.4.56"),
    ("yA", "loT", 3, "yAtAd", "7.1.35"),
    ("yA", "loT", 3, "yAtAt", "7.1.35+8.4.56"),
    ("vA", "loT", 0, "vAtAd", "7.1.35"),
    ("vA", "loT", 0, "vAtAt", "7.1.35+8.4.56"),
    ("vA", "loT", 3, "vAtAd", "7.1.35"),
    ("vA", "loT", 3, "vAtAt", "7.1.35+8.4.56"),
    ("ad", "loT", 0, "attAd", "7.1.35"),
    ("ad", "loT", 0, "attAt", "7.1.35+8.4.56"),
    ("ad", "loT", 3, "attAd", "7.1.35"),
    ("ad", "loT", 3, "attAt", "7.1.35+8.4.56"),
    ("kliS", "loT", 0, "kliSnItAd", "7.1.35"),
    ("kliS", "loT", 0, "kliSnItAt", "7.1.35+8.4.56"),
    ("kliS", "loT", 3, "kliSnItAd", "7.1.35"),
    ("kliS", "loT", 3, "kliSnItAt", "7.1.35+8.4.56"),
    ("guD", "loT", 0, "guDnItAd", "7.1.35"),
    ("guD", "loT", 0, "guDnItAt", "7.1.35+8.4.56"),
    ("guD", "loT", 3, "guDnItAd", "7.1.35"),
    ("guD", "loT", 3, "guDnItAt", "7.1.35+8.4.56"),
    ("aS", "loT", 0, "aSnItAd", "7.1.35"),
    ("aS", "loT", 0, "aSnItAt", "7.1.35+8.4.56"),
    ("aS", "loT", 3, "aSnItAd", "7.1.35"),
    ("aS", "loT", 3, "aSnItAt", "7.1.35+8.4.56"),
    ("muz", "loT", 0, "muzRItAd", "7.1.35"),
    ("muz", "loT", 0, "muzRItAt", "7.1.35+8.4.56"),
    ("muz", "loT", 3, "muzRItAd", "7.1.35"),
    ("muz", "loT", 3, "muzRItAt", "7.1.35+8.4.56"),
    ("vrI", "loT", 0, "vrIRItAd", "7.1.35"),
    ("vrI", "loT", 0, "vrIRItAt", "7.1.35+8.4.56"),
    ("vrI", "loT", 3, "vrIRItAd", "7.1.35"),
    ("vrI", "loT", 3, "vrIRItAt", "7.1.35+8.4.56"),
    ("Ap", "loT", 0, "ApnutAd", "7.1.35"),
    ("Ap", "loT", 0, "ApnutAt", "7.1.35+8.4.56"),
    ("Ap", "loT", 3, "ApnutAd", "7.1.35"),
    ("Ap", "loT", 3, "ApnutAt", "7.1.35+8.4.56"),
    ("Sak", "loT", 0, "SaknutAd", "7.1.35"),
    ("Sak", "loT", 0, "SaknutAt", "7.1.35+8.4.56"),
    ("Sak", "loT", 3, "SaknutAd", "7.1.35"),
    ("Sak", "loT", 3, "SaknutAt", "7.1.35+8.4.56"),
    ("hi", "loT", 0, "hinutAd", "7.1.35"),
    ("hi", "loT", 0, "hinutAt", "7.1.35+8.4.56"),
    ("hi", "loT", 3, "hinutAd", "7.1.35"),
    ("hi", "loT", 3, "hinutAt", "7.1.35+8.4.56"),
    ("ri", "loT", 0, "riRutAd", "7.1.35"),
    ("ri", "loT", 0, "riRutAt", "7.1.35+8.4.56"),
    ("ri", "loT", 3, "riRutAd", "7.1.35"),
    ("ri", "loT", 3, "riRutAt", "7.1.35+8.4.56"),
```

Note the shapes worth reading twice: `attAt` (√ad — 8.4.55 cartva applies at
the `d`+`t` junction before 8.2.39 ever sees the final), and `kliSnItAt`
(kryādi — 3.1.83 declines because the ending is no longer `hi`, and 6.4.113
then gives `nI`). Cells 0 and 3 carry the same strings: tātaṅ makes prathama
eka and madhyama eka syncretic.

- [ ] **Step 9: Add the non-form pins**

Add to `known_nonforms_are_invalid`, with these comments:

```rust
        // 7.1.35 tātaṅ. Because the rule is optional, a broken guard ADDS a
        // wrong second form rather than replacing a right one — invisible to
        // any test that only asks whether the right form still derives.
        "ApnotAt", // 7.1.35 failing to clear Pit / set Ngit, so 7.3.84's
        // second (vikaraṇa-relative) application guṇates śnu — real form
        // ApnutAt
        "kliSAnatAt", // 7.1.35 ordered AFTER 3.1.83 instead of above it, so
        // śnā had already become śāna when the ending was still `hi` — real
        // form kliSnItAt
```

Then verify both by construction: make the mutation, run
`mise exec -- cargo test -p panini --test paradigm`, confirm the reported
extra form is exactly this string, and revert. If a mutation emits something
else, pin what it actually emits and correct the comment.

- [ ] **Step 10: Add the three trace pins**

Add to `crates/panini/tests/trace.rs`:

```rust
#[test]
fn klishnitat_trace_pins_tatan_above_3_1_83() {
    // kliś loṭ madhyama eka, tātaṅ branch. 7.1.35 replaces `hi` BEFORE
    // 3.1.83 can see it, so śnā is never reshaped to śāna; 6.4.113 then
    // gives nI. This is the pin that fails if 7.1.35 is ever moved down —
    // the absence of 3.1.83 is the assertion, not the surface form, because
    // the wrong order still produces a plausible-looking word.
    let t = trace_for("kliSnItAt");
    assert!(t.contains(&"7.1.35".to_string()), "got {t:?}");
    assert!(!t.contains(&"3.1.83".to_string()), "got {t:?}");
    assert!(t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(t.contains(&"8.4.56".to_string()), "got {t:?}");
}

#[test]
fn apnutat_trace_shows_tatan_blocking_the_vikarana_guna() {
    // Ap loṭ prathama eka, tātaṅ branch. `tu` is pit and guṇates śnu
    // (Apnotu); tātaṅ is ṅit and 1.1.5 blocks the same application, so the
    // vikaraṇa stays `nu`. 7.3.84 must be absent entirely: the first
    // (root-relative) application never fires in svādi either.
    let t = trace_for("ApnutAt");
    assert!(t.contains(&"7.1.35".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.84".to_string()), "got {t:?}");
}

#[test]
fn bhavatu_forks_twice_into_three_branches() {
    // The first derivation in the engine on which TWO optional rules stack.
    // 7.1.35 forks Bavatu into a tātaṅ branch; 8.2.39 then voices that
    // branch's final t obligatorily; 8.4.56 forks it again. Index 0 is
    // still the declined derivation.
    let engine = Panini::new();
    let mut forms: Vec<String> = engine
        .check("BavatAt")
        .analyses
        .iter()
        .map(|a| a.form_slp1.clone())
        .collect();
    forms.sort();
    forms.dedup();
    assert_eq!(forms, vec!["BavatAt".to_string()]);

    let t = trace_for("BavatAd");
    assert!(t.contains(&"7.1.35".to_string()), "got {t:?}");
    assert!(t.contains(&"8.2.39".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.56".to_string()), "got {t:?}");
}
```

The branch-count assertion itself lives in `paradigm.rs`'s
`derivation_set_is_exactly_pinned`, which already requires the loṭ cells to
hold exactly `Bavatu` + the two alternates; this trace pin covers the part
that test cannot see — which rules produced which branch.

- [ ] **Step 11: Run the whole suite**

Run: `mise run test`

Expected: PASS.

- [ ] **Step 12: Lint, format, commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/tin.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs \
        crates/panini/tests/paradigm.rs crates/panini/tests/trace.rs
git commit -m "feat(prakriya): 7.1.35 tuhyos tAtaN, the loT alternates"
```

---

### Task 4: 3.4.111 *laṅaḥ śākaṭāyanasyaiva* and the 6.1.96 junction arm

These land together: the 6.1.96 arm is unreachable without 3.4.111, and an
arm with no witness is what the narrow-guard discipline forbids and what
mutation testing kills.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta/vikarana.rs` (insert 3.4.111
  between the 2.4.72 rule ending at `:217` and the 3.1.83 comment block)
- Modify: `crates/panini-prakriya/src/tinanta/adesha.rs:138-167` (6.1.96)
- Modify: `crates/panini-prakriya/src/tinanta/derivation_tests.rs` (order
  pin; vikalpa pin)
- Modify: `crates/panini/tests/paradigm.rs` (2 rows, `VIKALPA_RULES`, pins)
- Modify: `crates/panini/tests/trace.rs` (1 new pin)
- Test: guard tests in `vikarana.rs` and `adesha.rs` test modules

**Interfaces:**
- Consumes: `sound_before_ending(p: &Prakriya) -> Option<char>` from
  `crate::tinanta::terms` — **must be added to `vikarana.rs`'s import list**,
  which currently imports only `{ANGA, ENDING, SHAP}`. `adesha.rs` already
  imports it.
- Produces: no new public items.

- [ ] **Step 1: Write the failing guard tests**

Append to `vikarana.rs`'s test module:

```rust
    /// 3.4.111 replaces jhi with jus in laṅ after an ā — optionally, per
    /// Śākaṭāyana. The ending's text at this point is `J`, not `Ji`: 3.4.100
    /// itaś ca has already dropped the final `i` in the tiṅ stage.
    #[test]
    fn shakatayana_jus_needs_lan_a_and_jhi() {
        let rule = rules().find(|r| r.id == "3.4.111").unwrap();

        let mut p = lan_prakriya("yA", "", "J");
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "us");

        // not ā-final
        let mut p = lan_prakriya("Bava", "", "J");
        assert!(!(rule.apply)(&mut p));

        // not jhi
        let mut p = lan_prakriya("yA", "", "t");
        assert!(!(rule.apply)(&mut p));

        // a live vikaraṇa stands between the ā and the ending, so the ā is
        // not what precedes the ending — the affix-relative reading
        let mut p = lan_prakriya("yA", "a", "J");
        assert!(!(rule.apply)(&mut p));
    }
```

with this helper beside it:

```rust
    /// A laṅ prakriyā `[ANGA, SHAP, ENDING]` with the given texts. The ctx
    /// matters here: 3.4.111 is one of the few vikaraṇa-stage rules that
    /// reads the lakāra.
    fn lan_prakriya(anga: &str, shap: &str, ending: &str) -> Prakriya {
        Prakriya {
            terms: vec![Term::new(anga), Term::new(shap), Term::new(ending)],
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Bahu,
            ),
            ..Default::default()
        }
    }
```

`vikarana.rs`'s test module imports `super::*`, `Prakriya`, `Term` and
`rules`; add `use crate::context::Context; use panini_data::{Lakara, Pada,
Purusha, Vacana};` for the helper.

Append to `adesha.rs`'s test module:

```rust
    /// 6.1.96 has two arms. The original elides an a/ā that sits INSIDE the
    /// ending, before its final `us` (the yāsuṭ case, yAus -> yus). The
    /// junction arm elides the aṅga's final a/ā when the ending is a bare
    /// `us` — reachable only via 3.4.111, which is its sole witness today.
    #[test]
    fn usyapadantat_has_an_ending_arm_and_a_junction_arm() {
        let rule = rules().find(|r| r.id == "6.1.96").unwrap();

        // junction arm: ayA + us -> ay + us
        let mut p = Prakriya {
            terms: vec![Term::new("ayA"), Term::new(""), Term::new("us")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "ayus");

        // ending arm, unchanged: the a/ā is inside the ending
        let mut p = Prakriya {
            terms: vec![Term::new("yA"), Term::new(""), Term::new("yAus")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "yus");

        // junction arm declines when the aṅga is not a/ā-final
        let mut p = Prakriya {
            terms: vec![Term::new("yAy"), Term::new(""), Term::new("us")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
    }
```

- [ ] **Step 2: Run them to make sure they fail**

Run: `mise exec -- cargo test -p panini-prakriya shakatayana usyapadantat`

Expected: FAIL — the 3.4.111 lookup panics, and the 6.1.96 junction assertion
fails because that arm returns `false`.

- [ ] **Step 3: Add 3.4.111**

In `crates/panini-prakriya/src/tinanta/vikarana.rs`, first extend the import:

```rust
use crate::tinanta::terms::{ANGA, ENDING, SHAP, sound_before_ending};
```

and add whatever `Lakara` import the guard needs (`use panini_data::Lakara;`
if the file does not already have it).

Then insert after the 2.4.72 `Rule { … }` and before the `// 3.1.83 …`
comment block:

```rust
    // 3.4.110 ātaḥ / 3.4.111 laṅaḥ śākaṭāyanasyaiva: after an ā-final aṅga,
    // jhi is replaced by jus — and in laṅ that replacement is Śākaṭāyana's,
    // i.e. OPTIONAL. One rule implements the pair, cited under 3.4.111,
    // because 3.4.110 supplies only the condition and is never separately
    // observable here; vidyut-prakriya records the single step the same way.
    // Its witnesses are the two ā-final adādi roots: ayAn / ayuH, avAn /
    // avuH.
    //
    // `J`, not `Ji`: 3.4.100 itaś ca has already dropped jhi's final `i` in
    // the tiṅ stage (laṅ is ṅit-like and this is parasmaipada). The term is
    // still jhi — 3.4.110/111 replace the whole of it — but its text is not.
    //
    // The ā is read AFFIX-RELATIVELY via sound_before_ending, not from the
    // dhātu. For adādi the two agree, because śap is luk'd; they diverge the
    // moment a thematic ā-final root lands, whose śap would stand between
    // the ā and the ending and defeat 3.4.110's condition. Reading the dhātu
    // would over-fire there, silently, on a branch nobody inspects. Placing
    // the rule after 2.4.72 is what makes the affix-relative reading
    // available at all — and it is also what forces the `J` guard above.
    //
    // Must sit above 7.1.3 jho'ntaḥ, which turns a surviving `J` into `ant`.
    Rule {
        id: "3.4.111",
        name: "laNaH SAkawAyanasyEva",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lan) || p.terms[ENDING].text != "J" {
                return false;
            }
            if sound_before_ending(p) != Some('A') {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = "jus".into();
            p.record("3.4.111", "laNaH SAkawAyanasyEva", before);
            let before = p.snapshot();
            p.terms[ENDING].text = "us".into();
            p.record("1.3.9", "tasya lopaH", before);
            true
        },
    },
```

- [ ] **Step 4: Add the 6.1.96 junction arm**

In `crates/panini-prakriya/src/tinanta/adesha.rs`, replace the 6.1.96 rule's
doc comment and `apply` body with:

```rust
    // 6.1.96 usyapadāntāt: an a/ā immediately before the ending `us` is
    // elided (a single substitution in the ekaḥ pūrvaparayoḥ section). Two
    // arms, one sūtra:
    // - Ending arm: the a/ā sits INSIDE the ending. Fires for adādi vidhiliṅ
    //   3pl — after 7.2.79 strips yāsuṭ's s the ending is `yAus`, and the ā
    //   before `us` drops -> `yus` -> yA + yuH. Inert for the thematic
    //   gaṇas: 7.2.80 has already rewritten their liṅ 3pl ending to `iyus`,
    //   whose segment before `us` is `y`, not a/ā.
    // - Junction arm: the ending is a bare `us`, so the a/ā to elide is the
    //   aṅga's final sound. ayA + us -> ay + us -> ayuH. Reachable ONLY via
    //   3.4.111, which is its sole witness today: 3.4.108 jher jus is
    //   vidhiliṅ-only, and by the time this rule runs in vidhiliṅ the yāsuṭ
    //   of 3.4.103 has already made the ending `yAus` (or `yus`, if the
    //   ending arm fired). Every other cell reaches here with an ending that
    //   is not `us` at all.
    Rule {
        id: "6.1.96",
        name: "usyapadAntAt",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let e = &p.terms[ENDING].text;
            if !e.ends_with("us") {
                return false;
            }
            let n = e.chars().count();
            // the char immediately before the final `us` (None if the ending
            // is just "us", which wrapping_sub keeps panic-free)
            let pre = e.chars().nth(n.wrapping_sub(3));
            if matches!(pre, Some('a') | Some('A')) {
                let before = p.snapshot();
                let kept: String = e.chars().take(n - 3).collect();
                p.terms[ENDING].text = format!("{kept}us");
                p.record("6.1.96", "usyapadAntAt", before);
                return true;
            }
            if pre.is_some() {
                return false;
            }
            // Junction arm: nothing precedes `us` inside the ending, so look
            // to the nearest non-empty term before it.
            let Some(prev) = p.terms[..ENDING].iter().rposition(|t| !t.text.is_empty()) else {
                return false;
            };
            let Some(last) = p.terms[prev].text.chars().last() else {
                return false;
            };
            if !matches!(last, 'a' | 'A') {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[prev].text.chars().collect();
            s.pop();
            p.terms[prev].text = s.into_iter().collect();
            p.record("6.1.96", "usyapadAntAt", before);
            true
        },
    },
```

- [ ] **Step 5: Run the guard tests**

Run: `mise exec -- cargo test -p panini-prakriya shakatayana usyapadantat`

Expected: PASS.

- [ ] **Step 6: Pin the order and the vikalpa set**

In `derivation_tests.rs`, in `tinanta_rule_order_is_pinned`, insert
`"3.4.111"` immediately after `"2.4.72"`:

```rust
        "3.1.77", "3.1.81", "3.1.68", "2.4.72", "3.4.111", "3.1.83", "1.2.4", "6.4.71",
```

In `exactly_the_pinned_vikalpa_rules_are_optional`:

```rust
    let expected = ["7.1.35", "3.4.111", "6.4.107", "8.4.56"];
```

- [ ] **Step 7: Add the two `ALTERNATES` rows and widen `VIKALPA_RULES`**

In `crates/panini/tests/paradigm.rs`:

```rust
const VIKALPA_RULES: &[&str] = &["7.1.35", "3.4.111", "6.4.107", "8.4.56"];
```

```rust
    ("yA", "laN", 2, "ayuH", "3.4.111"),
    ("vA", "laN", 2, "avuH", "3.4.111"),
```

- [ ] **Step 8: Add the non-form pins**

Add to `known_nonforms_are_invalid`:

```rust
        // 3.4.110/111 Śākaṭāyana's jus. Optional, so a broken guard adds a
        // wrong form rather than removing a right one.
        "aBavuH", // 3.4.111 firing after a non-ā aṅga — real form aBavan
        "yuH",    // 3.4.111 not gated to laṅ, so laṭ's yAnti forks — real
                  // form yAnti
```

Verify both by construction: make the mutation, run
`mise exec -- cargo test -p panini --test paradigm`, confirm the extra form is
exactly this string, revert. If a mutation emits something else, pin what it
emits and correct the comment.

- [ ] **Step 9: Add the trace pin**

Add to `crates/panini/tests/trace.rs`:

```rust
#[test]
fn ayuh_trace_is_the_shakatayana_jus_path() {
    // yā laṅ prathama bahu, Śākaṭāyana branch. 3.4.111 replaces jhi with
    // jus before 7.1.3 can turn it into `ant`; 6.1.96's junction arm then
    // elides the aṅga's ā across the boundary (ayA + us -> ay + us), and
    // 8.3.15 gives the visarga.
    let t = trace_for("ayuH");
    assert!(t.contains(&"3.4.111".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.1.3".to_string()), "got {t:?}");
    assert!(t.contains(&"6.1.96".to_string()), "got {t:?}");
    assert!(t.contains(&"8.3.15".to_string()), "got {t:?}");
}
```

- [ ] **Step 10: Run the whole suite**

Run: `mise run test`

Expected: PASS.

- [ ] **Step 11: Lint, format, commit**

```bash
mise run fmt && mise run lint
git add crates/panini-prakriya/src/tinanta/vikarana.rs \
        crates/panini-prakriya/src/tinanta/adesha.rs \
        crates/panini-prakriya/src/tinanta/derivation_tests.rs \
        crates/panini/tests/paradigm.rs crates/panini/tests/trace.rs
git commit -m "feat(prakriya): 3.4.111 SAkawAyana's jus and 6.1.96's junction arm"
```

---

### Task 5: Close the audit and update the docs

The slice's claim is that the engine's derivation set equals
vidyut-prakriya's in all 1512 cells. This task proves it and records it.

**Files:**
- Modify: `AGENTS.md`, `README.md`, `docs/ARCHITECTURE.md`
- No source changes expected; if the audit finds a divergence, that is a bug
  in Tasks 2–4 and goes back to the task that owns it.

**Interfaces:**
- Consumes: everything from Tasks 1–4.
- Produces: nothing consumed by later tasks.

- [ ] **Step 1: Re-run the audit probe**

The probe lives in a vidyut-prakriya checkout as
`vidyut-prakriya/examples/panini_full_audit.rs` (it was written during design;
its source is in the spec's Verification section). Build and run it, then
compare its output against `PARADIGM ∪ ALTERNATES` as a set per cell.

Run (substituting the vidyut checkout path):

```bash
mise exec -- cargo run --offline \
  --manifest-path <vidyut>/vidyut-prakriya/Cargo.toml \
  --example panini_full_audit > /tmp/vidyut_audit.tsv
```

Expected: 1512 rows plus one banner line on stdout, no `MISSING` lines on
stderr.

- [ ] **Step 2: Diff both directions**

Compare per cell. Expected: **empty in both directions** — no form vidyut
derives that the engine does not, and none the engine derives that vidyut does
not. Any residue is a bug in Tasks 2–4, not something to record as a known
divergence.

Cross-check the shape while you are there: 1406 cells with one form, 58 with
two, 48 with three; `ALTERNATES` has 154 rows keyed 48 `8.4.56`, 48 `7.1.35`,
48 `7.1.35+8.4.56`, 2 `3.4.111`, 8 `6.4.107`.

- [ ] **Step 3: Run mutation testing**

Run: `mise run mutants`

Expected: zero survivors. A survivor means a guard arm has no witness — shrink
the guard rather than adding a test, per the repo's narrow-guard discipline.
Use the pinned generous `--timeout` that `mise run mutants` already passes; a
short cap turns undetectable mutants into timeouts and makes a zero-survivor
report vacuous.

- [ ] **Step 4: Update AGENTS.md**

Three edits:

1. In the optional-rules bullet, replace "**6.4.107 is currently the only
   one.**" with a sentence naming all four — 3.4.111, 6.4.107, 7.1.35, 8.4.56
   — and noting that 7.1.35 and 8.4.56 can stack on one derivation, giving a
   three-branch cell.
2. Replace the ordering-constraint bullet's opening rule with both directions:
   an optional rule must sit **after** consumers whose predicate its mutation
   makes *lie* (6.4.107's case: the evidence is destroyed but the grammatical
   fact is unchanged), and **before** consumers when the mutation changes the
   *fact* itself (7.1.35's case: the ending genuinely is no longer `hi`, so
   3.1.83, 6.4.105 and 6.4.106 must read the new value). Keep the existing
   6.4.107 worked example and add the 7.1.35 one.
3. In the golden-paradigm paragraph, record that prathama eka of laṅ and
   vidhiliṅ is now the jaś form for parasmaipada roots (`aBavad`, `Baved`) —
   8.2.39 is obligatory, so that is index 0 — and that the suite is no longer
   filtered by any one-form-per-cell convention.

- [ ] **Step 5: Update README.md**

In the Scope paragraph, the sentence "A cell may have more than one valid form
where an optional (*vikalpa*) sūtra applies — `hinvaH` and `hinuvaH` are both
correct" now understates the case. Say that 106 of the 1512 cells hold more
than one form and that 48 of them hold three (`Bavatu`, `BavatAt`, `BavatAd`),
keeping the `hinvaH`/`hinuvaH` example.

- [ ] **Step 6: Update docs/ARCHITECTURE.md**

Three edits:

1. The stage table (`:36` and its neighbours): add the four new ids to the
   rule ranges of `tin.rs`, `vikarana.rs`, `adesha.rs` and `tripadi.rs`.
2. `:148`: "**6.4.107 …** is the only optional rule" — replace with the set of
   four.
3. `:144`: "Branch count is 2^k in the number of optional rules that fire; k
   is 1" is **wrong** now, not merely stale. loṭ prathama eka has k = 2 and
   **three** branches, not four, because 8.4.56 declines on the vowel-final
   base branch and forks only the tātaṅ one. Replace with: the branch count is
   the number of distinct subsets of optional rules that actually apply, which
   is bounded by 2^k and reaches it only when every optional rule fires on
   every branch.

- [ ] **Step 7: Full verification**

Run: `mise run test && mise run lint && mise run fmt-check && mise run audit`

Expected: all pass.

- [ ] **Step 8: Commit**

```bash
git add AGENTS.md README.md docs/ARCHITECTURE.md
git commit -m "docs: the suite is no longer filtered by any convention"
```

---

## Self-Review

**Spec coverage.** Every section of the spec maps to a task: 7.1.35 → Task 3;
8.2.39 and 8.4.56 → Task 2; 3.4.110/111 and the 6.1.96 junction arm → Task 4;
the `ALTERNATES` key column and its test → Task 1; the audit, mutation run and
all three doc files → Task 5. The spec's non-form-pin discipline (pins must be
what a mutation actually emits, verified by breaking the guard) is a step in
each of Tasks 2, 3 and 4 rather than a single late task, so each guard is
pinned by the person who wrote it.

**One thing the spec did not anticipate**, discovered while writing this plan
and now handled in Tasks 2 and 3: every single-form test helper in
`derivation_tests.rs` routes through `sole()`, which asserts exactly one
branch. Ten existing call sites cover cells these rules fork and would panic.
Task 2 adds `declined`/`form_g_forked` and converts the four cells it forks;
Task 3 converts the six it forks. Keeping `sole` as the default is deliberate:
an unexpected fork must still fail loudly.

**Type consistency.** `ALTERNATES` is a 5-tuple from Task 1 onward and every
later task adds 5-tuples. `VIKALPA_RULES` grows in pipeline order
(`7.1.35`, `3.4.111`, `6.4.107`, `8.4.56`) so that a two-rule branch keys as
`"7.1.35+8.4.56"`, matching the rows Task 3 adds. `form_g_forked` takes its
branch count as the fifth argument in both tasks that call it.
