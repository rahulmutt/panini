# Ātmanepada Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add ātmanepada (labhate) across all four existing lakāras — six new ātmanepadī roots, pada sanction in the derivation, fifteen new self-guarding rules, seven guard widenings — growing golden coverage 216 → 432 forms.

**Architecture:** Everything lands in the existing shape: `panini-data` gains the pada axis (enum variant, `Dhatu.pada`, nine raw ātmanepada tiṅ endings), `panini-prakriya` gains rules in the single ordered `TINANTA_RULES` array (each self-guarding, each recording its sūtra), and the analyzer/facade/CLI thread the pada through. Spec: `docs/superpowers/specs/2026-07-20-atmanepada-design.md` (the rule table there, rows 0–40, is the target order).

**Tech Stack:** Rust 1.97.1 via mise. `mise run build | test | lint | fmt | mutants`.

## Global Constraints

- SLP1 is the only internal representation (AGENTS.md). Aspirates are single capitals: `Bavate` not `bhavate`.
- `#![forbid(unsafe_code)]` stays in every crate touched.
- New grammar = self-guarding `Rule` entries in `TINANTA_RULES`, never branches in `derive`.
- Every recorded sūtra id must have a `TINANTA_RULES` entry whose `name` matches the recorded name exactly (pinned by `recorded_step_names_match_tinanta_rules_for_every_id`).
- Sūtra names must match ashtadhyayi.com (Task 4 verifies them; do not skip it).
- The 216 existing golden surface forms must never break in any task. Their pinned traces change exactly once (Task 2: each gains a leading `"1.3.78"`); any other trace change is a bug.
- Rule order in `TINANTA_RULES` is load-bearing. Insert new rules exactly where each task says, relative to named existing rules.
- Rules before the 3.1.68 boundary address the ending as `terms[ENDING_PRE_SHAP]` (index 1); rules after it use `terms[ENDING]` (index 2). See the NOTE at `crates/panini-prakriya/src/tinanta.rs:44`.
- Run `mise run fmt` before every commit; `mise run lint` must stay clean.

**Working state between tasks:** Tasks are ordered so `mise run test` passes after every task. Between Task 2 and Task 9 some ātmanepada cells derive *placeholder-wrong but self-consistent* surface forms (e.g. loṭ ātmanepada = laṭ text until Task 7); that is expected — golden blocks for each lakāra are added in the same task as its rules.

---

### Task 1: Data layer — the pada axis

**Files:**
- Modify: `crates/panini-data/src/lib.rs`
- Modify: `data/dhatupatha.tsv`, `data/tin.tsv` (reference mirrors of the Rust statics — keep in sync)

**Interfaces:**
- Produces: `Pada::Atmanepada` variant; `Dhatu { code, gana, pada, artha }` (new `pub pada: Pada` field, declared between `gana` and `artha`); `dhatus()` returns 12 roots; `tin_ending(Pada::Atmanepada, …)` returns the nine raw ātmanepada endings.
- Consumed by: every later task.

- [ ] **Step 1: Write the failing tests** — in the `tests` module of `crates/panini-data/src/lib.rs`, replace `has_six_curated_roots` and add an ātmanepada endings test:

```rust
    #[test]
    fn has_twelve_curated_roots_with_padas() {
        assert_eq!(dhatus().len(), 12);
        let bu = dhatus().iter().find(|d| d.code == "BU").unwrap();
        assert!(matches!(bu.pada, Pada::Parasmaipada));
        let labh = dhatus().iter().find(|d| d.code == "laB").unwrap();
        assert!(matches!(labh.pada, Pada::Atmanepada));
        // Both vowel-initial atmanepadi roots must be present (they exercise
        // the AT-augment path 6.4.72/6.1.90).
        assert!(dhatus().iter().any(|d| d.code == "eD"));
        assert!(dhatus().iter().any(|d| d.code == "Ikz"));
    }

    #[test]
    fn atmanepada_tin_endings_are_raw_upadesha_forms() {
        use Purusha::*;
        use Vacana::*;
        let cases = [
            ((Prathama, Eka), "ta"),
            ((Prathama, Dvi), "AtAm"),
            ((Prathama, Bahu), "Ja"),
            ((Madhyama, Eka), "TAs"),
            ((Madhyama, Dvi), "ATAm"),
            ((Madhyama, Bahu), "Dvam"),
            ((Uttama, Eka), "iw"),
            ((Uttama, Dvi), "vahi"),
            ((Uttama, Bahu), "mahiN"),
        ];
        for ((pu, va), expected) in cases {
            assert_eq!(tin_ending(Pada::Atmanepada, pu, va), expected);
        }
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd /workspace && mise run test -- -p panini-data`
Expected: compile FAIL — `Atmanepada` not found in `Pada`, no field `pada` on `Dhatu`.

- [ ] **Step 3: Implement.** In `crates/panini-data/src/lib.rs`:

Extend `Pada`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pada {
    Parasmaipada,
    Atmanepada,
}
```

Add the field to `Dhatu` (between `gana` and `artha`):

```rust
#[derive(Debug, Clone, Copy)]
pub struct Dhatu {
    pub code: &'static str,
    pub gana: Gana,
    /// Which pada this root takes. Ubhayapadi roots are out of scope; each
    /// curated root has exactly one pada.
    pub pada: Pada,
    pub artha: &'static str,
}
```

Add `pada: Pada::Parasmaipada,` to each of the six existing `DHATUS` entries, then append the six ātmanepadī roots after `vad`:

```rust
    Dhatu {
        code: "eD",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "vfdDO",
    },
    Dhatu {
        code: "laB",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "prAptO",
    },
    Dhatu {
        code: "sev",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "sevane",
    },
    Dhatu {
        code: "vft",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "vartane",
    },
    Dhatu {
        code: "BAz",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        code: "Ikz",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "darSane",
    },
```

Extend `tin_ending` with the ātmanepada arm (the raw upadeśa forms; all divergence is rule-driven, matching how the parasmaipada arm stores `tip` not `ti`):

```rust
        Pada::Atmanepada => match (purusha, vacana) {
            (Prathama, Eka) => "ta",
            (Prathama, Dvi) => "AtAm",
            (Prathama, Bahu) => "Ja",
            (Madhyama, Eka) => "TAs",
            (Madhyama, Dvi) => "ATAm",
            (Madhyama, Bahu) => "Dvam",
            (Uttama, Eka) => "iw",
            (Uttama, Dvi) => "vahi",
            (Uttama, Bahu) => "mahiN",
        },
```

- [ ] **Step 4: Update the TSV mirrors.** `data/dhatupatha.tsv` becomes four columns (`code<TAB>gana<TAB>pada<TAB>artha`), full new content:

```
BU	bhvadi	parasmaipada	sattAyAm
nI	bhvadi	parasmaipada	prApaRe
ji	bhvadi	parasmaipada	jaye
smf	bhvadi	parasmaipada	cintAyAm
paW	bhvadi	parasmaipada	vyaktAyAM vAci
vad	bhvadi	parasmaipada	vyaktAyAM vAci
eD	bhvadi	atmanepada	vfdDO
laB	bhvadi	atmanepada	prAptO
sev	bhvadi	atmanepada	sevane
vft	bhvadi	atmanepada	vartane
BAz	bhvadi	atmanepada	vyaktAyAM vAci
Ikz	bhvadi	atmanepada	darSane
```

Append to `data/tin.tsv`:

```
atmanepada	prathama	eka	ta
atmanepada	prathama	dvi	AtAm
atmanepada	prathama	bahu	Ja
atmanepada	madhyama	eka	TAs
atmanepada	madhyama	dvi	ATAm
atmanepada	madhyama	bahu	Dvam
atmanepada	uttama	eka	iw
atmanepada	uttama	dvi	vahi
atmanepada	uttama	bahu	mahiN
```

- [ ] **Step 5: Run the full suite** (not just panini-data — the new roots flow into analyzer candidates and the roundtrip test, which still hardcode `Pada::Parasmaipada` and stay green at this point).

Run: `cd /workspace && mise run test`
Expected: PASS (all crates).

- [ ] **Step 6: Format and commit**

```bash
mise run fmt
git add crates/panini-data data/dhatupatha.tsv data/tin.tsv
git commit -m "feat(data): Pada::Atmanepada, Dhatu.pada, six atmanepadi roots, atmanepada tin endings"
```

---

### Task 2: Pada sanction (1.3.12 / 1.3.78) + pada-domain guards

The derivation itself must sanction the requested pada, so `derive(laB, …, Parasmaipada, …)` cannot silently produce `laBati`. Also widen the three rules whose sūtras are parasmaipada-only, and point the analyzer/roundtrip at the root's tagged pada.

**Files:**
- Modify: `crates/panini-prakriya/src/term.rs` (two new tags)
- Modify: `crates/panini-prakriya/src/prakriya.rs` (`blocked` flag)
- Modify: `crates/panini-prakriya/src/controller.rs` (`run_pipeline` respects `blocked`)
- Modify: `crates/panini-prakriya/src/tinanta.rs` (two new rules first in `TINANTA_RULES`; widen 3.4.100, 3.4.101, 3.4.103; tag the dhātu term in `derive`)
- Modify: `crates/panini-analyze/src/lib.rs` (propose the root's pada)
- Modify: `crates/panini/tests/roundtrip.rs`, `crates/panini/tests/trace.rs`, `crates/panini/tests/paradigm.rs`

**Interfaces:**
- Consumes: `Dhatu.pada` (Task 1).
- Produces: `Tag::Atmanepadin` (on the dhātu term), `Tag::Ngit` (declared now, first consumed in Task 5), `Prakriya.blocked: bool`, rules `1.3.12`/`1.3.78` as the first two entries of `TINANTA_RULES`. Every parasmaipada trace now starts with `"1.3.78"`; every ātmanepada trace with `"1.3.12"`.

- [ ] **Step 1: Write the failing engine tests** — in the `tests` module of `tinanta.rs`:

```rust
    #[test]
    fn pada_sanction_blocks_wrong_pada_derivations() {
        // 1.3.12/1.3.78: derivation is the source of truth for pada. A
        // wrong-pada derive must not silently produce a surface form.
        let labh = dhatus().iter().find(|d| d.code == "laB").unwrap();
        let p = derive(
            labh,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert!(p.blocked, "atmanepadin root + parasmaipada must block");
        assert_eq!(p.text(), "laB", "no rule may run after the block");
        assert!(p.log.is_empty(), "a blocked derivation records nothing");

        let bhu = dhatus().iter().find(|d| d.code == "BU").unwrap();
        let p = derive(
            bhu,
            Lakara::Lat,
            Pada::Atmanepada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert!(p.blocked, "parasmaipada root + atmanepada must block");
    }

    #[test]
    fn pada_sanction_records_the_sanctioning_sutra() {
        let bhu = dhatus().iter().find(|d| d.code == "BU").unwrap();
        let p = derive(
            bhu,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert_eq!(p.log.first().unwrap().sutra, "1.3.78");

        let labh = dhatus().iter().find(|d| d.code == "laB").unwrap();
        let p = derive(
            labh,
            Lakara::Lat,
            Pada::Atmanepada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert_eq!(p.log.first().unwrap().sutra, "1.3.12");
    }

    #[test]
    fn itash_ca_and_yasut_are_parasmaipada_only() {
        // 3.4.100 must not eat the final i of atmanepada vahi/mahi/i in Nit
        // lakaras (aBavAvahi, not aBavAvah), and 3.4.103's own text says
        // parasmEpadezu — atmanepada lin takes siyut (3.4.102) instead.
        for (id, ending, lakara) in [
            ("3.4.100", "vahi", Lakara::Lan),
            ("3.4.100", "i", Lakara::Lan),
            ("3.4.103", "ta", Lakara::VidhiLin),
        ] {
            let mut p = Prakriya {
                terms: vec![Term::new("laB"), Term::new(ending)],
                log: vec![],
                ctx: Context::new(lakara, Pada::Atmanepada, Purusha::Uttama, Vacana::Dvi),
            };
            let rule = TINANTA_RULES.iter().find(|r| r.id == id).unwrap();
            assert!(!(rule.apply)(&mut p), "{id} must not fire for atmanepada");
            assert_eq!(p.terms[ENDING_PRE_SHAP].text, ending);
        }
    }
```

- [ ] **Step 2: Run to verify failure**

Run: `cd /workspace && mise run test -- -p panini-prakriya`
Expected: compile FAIL (`blocked` field does not exist), then assertion failures once it compiles.

- [ ] **Step 3: Implement.**

`term.rs` — extend the `Tag` enum:

```rust
    Abhyasa,
    /// The dhatu takes atmanepada (the data-layer stand-in for the anudatta
    /// it-marker that 1.3.12 reads; see the spec's pada-sanction section).
    Atmanepadin,
    /// The term behaves as Nit (set by the atidesha 1.2.4 sarvadhatukam apit;
    /// consumed by 7.2.81 Ato NitaH).
    Ngit,
```

`prakriya.rs` — add the field to `Prakriya` (keeps `#[derive(Default)]` working):

```rust
#[derive(Debug, Clone, Default)]
pub struct Prakriya {
    pub terms: Vec<Term>,
    pub log: Vec<RuleStep>,
    pub ctx: Context,
    /// Set when a samjna/sanction rule determines the requested derivation
    /// is impossible (wrong pada for the root). The pipeline stops; the
    /// prakriya's text can never equal a real surface form.
    pub blocked: bool,
}
```

`controller.rs` — `run_pipeline` stops on block:

```rust
pub fn run_pipeline(p: &mut Prakriya, rules: &[Rule]) {
    for rule in rules {
        if p.blocked {
            return;
        }
        (rule.apply)(p);
    }
}
```

`tinanta.rs` — in `derive`, tag the dhātu term:

```rust
    p.terms.push({
        let mut t = Term::new(dhatu.code);
        t.add(Tag::Dhatu);
        if matches!(dhatu.pada, Pada::Atmanepada) {
            t.add(Tag::Atmanepadin);
        }
        t
    });
```

`tinanta.rs` — insert as the FIRST two entries of `TINANTA_RULES`, before 3.4.78 (names are drafts until Task 4 confirms them):

```rust
    // 1.3.12 anudāttaṅita ātmanepadam: a root carrying the anudātta/ṅit
    // marker (here: the data-layer Atmanepadin tag) takes ātmanepada.
    // Sanctions the requested pada; the wrong pada BLOCKS the derivation —
    // derivation, not the analyzer, is the source of truth for pada.
    Rule {
        id: "1.3.12",
        name: "anudAttaNita Atmanepadam",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Atmanepadin) {
                return false; // parasmaipada roots are 1.3.78's business
            }
            match p.ctx.pada {
                Pada::Atmanepada => {
                    let before = p.snapshot();
                    p.record("1.3.12", "anudAttaNita Atmanepadam", before);
                    true
                }
                Pada::Parasmaipada => {
                    p.blocked = true;
                    false
                }
            }
        },
    },
    // 1.3.78 śeṣāt kartari parasmaipadam: everything else takes parasmaipada.
    Rule {
        id: "1.3.78",
        name: "SezAt kartari parasmEpadam",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms[ANGA].has(Tag::Atmanepadin) {
                return false;
            }
            match p.ctx.pada {
                Pada::Parasmaipada => {
                    let before = p.snapshot();
                    p.record("1.3.78", "SezAt kartari parasmEpadam", before);
                    true
                }
                Pada::Atmanepada => {
                    p.blocked = true;
                    false
                }
            }
        },
    },
```

Widen the three parasmaipada-only guards:

- **3.4.100**: add `|| matches!(p.ctx.pada, Pada::Atmanepada)` to the early-return guard (comment: the sūtra elides the i of *parasmaipada* ṅit endings; ātmanepada `vahi`/`mahi`/`i` keep theirs — aBavAvahi).
- **3.4.101**: same addition to its guard (the sūtra's tas/thas/tha/mip are parasmaipada endings; today this is also safe by text, the guard states the domain).
- **3.4.103**: change guard to

```rust
            if !matches!(p.ctx.lakara, Lakara::VidhiLin)
                || matches!(p.ctx.pada, Pada::Atmanepada)
            {
                return false;
            }
```

and update its comment: the *parasmaipadeṣu* of the sūtra is now enforced; ātmanepada liṅ takes sīyuṭ (3.4.102, Task 9).

`panini-analyze/src/lib.rs` — propose the root's own pada (delete the hardcoded literal):

```rust
                out.push(Candidate {
                    dhatu: d,
                    lakara,
                    pada: d.pada,
                    purusha,
                    vacana,
                });
```

`crates/panini/tests/roundtrip.rs` — derive with the root's pada:

```rust
                let form = engine.derive(d, lakara, d.pada, pu, va).text();
```

(and drop the now-unused `Pada` import). In `tinanta.rs`, update `recorded_step_names_match_tinanta_rules_for_every_id` the same way: `derive(d, lakara, d.pada, purusha, vacana)`.

- [ ] **Step 4: Update the twelve pinned traces.** Every parasmaipada derivation now records `1.3.78` first. In `crates/panini/tests/trace.rs`, prepend `"1.3.78"` to each `assert_eq!` vector (the three `contains`-style tests — `aBavam`, `Bava`, `BavAni` — need no change). The nine updated vectors:

```rust
// Bavati:
vec!["1.3.78", "3.4.78", "1.3.9", "3.1.68", "1.3.9", "7.3.84", "6.1.78"]
// Bavanti:
vec!["1.3.78", "3.4.78", "3.1.68", "1.3.9", "7.1.3", "7.3.84", "6.1.78", "6.1.97"]
// BavAmaH:
vec!["1.3.78", "3.4.78", "3.1.68", "1.3.9", "7.3.84", "6.1.78", "7.3.101", "8.3.15"]
// aBavat:
vec!["1.3.78", "3.4.78", "1.3.9", "3.4.100", "3.1.68", "1.3.9", "6.4.71", "7.3.84", "6.1.78"]
// aBavan:
vec!["1.3.78", "3.4.78", "3.4.100", "3.1.68", "1.3.9", "6.4.71", "7.1.3", "7.3.84", "6.1.78", "6.1.97", "8.2.23"]
// Bavatu:
vec!["1.3.78", "3.4.78", "1.3.9", "3.4.85", "3.4.86", "3.1.68", "1.3.9", "7.3.84", "6.1.78"]
// Bavet:
vec!["1.3.78", "3.4.78", "1.3.9", "3.4.100", "3.4.103", "3.1.68", "1.3.9", "7.2.79", "7.2.80", "7.3.84", "6.1.78", "6.1.87", "6.1.66"]
// BaveyuH:
vec!["1.3.78", "3.4.78", "3.4.108", "1.3.9", "3.4.103", "3.1.68", "1.3.9", "7.2.79", "7.2.80", "7.3.84", "6.1.78", "6.1.87", "8.3.15"]
// Baveyam:
vec!["1.3.78", "3.4.78", "1.3.9", "3.4.101", "3.4.103", "3.1.68", "1.3.9", "7.2.79", "7.2.80", "7.3.84", "6.1.78", "6.1.87"]
```

Also update the file's header comment: the pada-sanction step (1.3.78/1.3.12) now opens every trace.

- [ ] **Step 5: Add the wrong-pada negatives.** In `crates/panini/tests/paradigm.rs`, extend `known_nonforms_are_invalid`'s list:

```rust
        // Wrong pada: the root's pada tag gates the whole derivation
        // (1.3.12 / 1.3.78) and the analyzer proposes only the tagged pada.
        "laBati",   // atmanepadin root with a parasmaipada ending
        "Bavate",   // parasmaipada root with an atmanepada ending
        "eDati",    // vowel-initial atmanepadin root, parasmaipada ending
        "alaBat",   // laN parasmaipada shape on an atmanepadin root
        "laB",      // a bare root code is not a surface form
```

- [ ] **Step 6: Run everything**

Run: `cd /workspace && mise run test`
Expected: PASS. (Roundtrip now derives ātmanepada roots with raw endings — self-consistent placeholder forms like `laBata`; fine until Tasks 6–9.)

- [ ] **Step 7: Format and commit**

```bash
mise run fmt
git add -A
git commit -m "feat(prakriya): 1.3.12/1.3.78 pada sanction; parasmaipada-only guards on 3.4.100/101/103; analyzer proposes the root's pada"
```

---

### Task 3: Facade and CLI carry the pada

**Files:**
- Modify: `crates/panini/src/lib.rs`
- Modify: `crates/panini-cli/src/main.rs`

**Interfaces:**
- Produces: `Analysis.pada: Pada`; `pub fn pada_name(pada: Pada) -> &'static str` returning `"parasmEpadam"` / `"Atmanepadam"` (SLP1; Task 4 re-checks the spelling); CLI JSON objects gain a `"pada"` key.

- [ ] **Step 1: Write the failing tests** — in `crates/panini/src/lib.rs` tests:

```rust
    #[test]
    fn analysis_reports_its_pada() {
        let engine = Panini::new();
        let r = engine.check("Bavati");
        let a = r.analyses.iter().find(|a| a.form_slp1 == "Bavati").unwrap();
        assert!(matches!(a.pada, Pada::Parasmaipada));
        assert_eq!(pada_name(a.pada), "parasmEpadam");
        assert_eq!(pada_name(Pada::Atmanepada), "Atmanepadam");
    }
```

- [ ] **Step 2: Run to verify failure**

Run: `cd /workspace && mise run test -- -p panini`
Expected: compile FAIL — no field `pada`, no `pada_name`.

- [ ] **Step 3: Implement.** In `crates/panini/src/lib.rs`: add `pub pada: Pada,` to `Analysis` (after `lakara`); fill it in `check` with `pada: c.pada,`; add next to `lakara_name`:

```rust
/// SLP1 name of a pada, for display in traces and CLI output.
pub fn pada_name(pada: Pada) -> &'static str {
    match pada {
        Pada::Parasmaipada => "parasmEpadam",
        Pada::Atmanepada => "Atmanepadam",
    }
}
```

In `crates/panini-cli/src/main.rs`: import `pada_name`, and add to the per-analysis JSON object (after `"lakara"`):

```rust
                        "pada": pada_name(a.pada),
```

Human-readable output is unchanged (spec: JSON only).

- [ ] **Step 4: Run the suite**

Run: `cd /workspace && mise run test && cargo run -p panini-cli -- check bhavati --json | grep pada`
Expected: tests PASS; the grep prints `"pada": "parasmEpadam"`.

- [ ] **Step 5: Format and commit**

```bash
mise run fmt
git add crates/panini crates/panini-cli
git commit -m "feat(facade,cli): Analysis.pada and pada field in check --json"
```

---

### Task 4: Verify the fifteen sūtra names against the reference

Sūtra ids/names in traces must match ashtadhyayi.com (AGENTS.md hard gate). Slice 2's precedent: ashtadhyayi.com itself is a JS SPA that plain fetch cannot render, so use the underlying open data (the `ashtadhyayi-com/data` GitHub repository, or the ashtadhyayi.github.io mirror) — see the "Reference verification" section of `docs/superpowers/specs/2026-07-20-vidhilin-lakara-design.md` for how this was done last time.

**Files:**
- Modify: `docs/superpowers/specs/2026-07-20-atmanepada-design.md` (only if a draft name is wrong)

**Interfaces:**
- Produces: the confirmed SLP1 name literal for each of: 1.3.12, 1.3.78, 1.2.4, 3.4.79, 3.4.80, 3.4.90, 3.4.91, 3.4.93, 3.4.102, 3.4.105, 3.4.106, 6.4.72, 6.1.90, 7.2.81, 7.3.86. Tasks 5–9 use these literals verbatim in `Rule.name` and `p.record(...)`.

- [ ] **Step 1: Fetch the reference text for each sūtra** (Devanāgarī) from the data source and transliterate to SLP1. Draft names being checked (from the spec):
  `anudAttaNita Atmanepadam` (1.3.12), `SezAt kartari parasmEpadam` (1.3.78), `sArvaDAtukam apit` (1.2.4), `TAsaH se` (3.4.80), `wita AtmanepadAnAM wer e` (3.4.79), `savAByAM vAmO` (3.4.91), `eta E` (3.4.93), `Am etaH` (3.4.90), `Jasya ran` (3.4.105), `iwo 't` (3.4.106), `liNaH sIyuw` (3.4.102), `Aq ajAdInAm` (6.4.72), `AwaS ca` (6.1.90), `Ato NitaH` (7.2.81), `pugantalaGUpaDasya ca` (7.3.86).
- [ ] **Step 2: Confirm word-spacing** the way slice 2 caught `yAsuw parasmEpadezUdAtto Nic ca` — compare against the Devanāgarī, not another transliteration. Also verify 3.4.102's full name (the sūtra is *liṅaḥ sīyuṭ* — confirm whether the reference text includes more words, as 3.4.103's did).
- [ ] **Step 3: Confirm the two attribution questions** flagged in the spec: (a) the loṭ-uttama `A+E` merge — if the reference derivation for *bhavai*-type forms cites 6.1.88 *vṛddhir eci* rather than 6.1.90 *āṭaś ca*, implement the ending arm of Task 7's rule under id 6.1.88 with name `vfdDir eci` and keep 6.1.90 for the aṅga arm only; (b) 3.4.106's name (`iwo 't`).
- [ ] **Step 4: Record findings.** Update the spec's draft-name list with any corrections (as slice 2 did) and note per-sūtra sourcing in the commit message. If nothing changed, state that explicitly in the commit message.

```bash
git add docs/superpowers/specs/2026-07-20-atmanepada-design.md
git commit -m "docs: confirm atmanepada sutra names against the reference"
```

---

### Task 5: 1.2.4 sārvadhātukam apit — term-level ṅid-vat

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs`

**Interfaces:**
- Consumes: `Tag::Ngit` (declared Task 2), confirmed name (Task 4).
- Produces: rule `1.2.4` ordered directly after the shared it-saṃjñā rule (id `1.3.9`) and before `3.4.85`. Ātmanepada endings (except loṭ uttama) carry `Tag::Ngit` from here on; 7.2.81 (Task 6) is the consumer.

- [ ] **Step 1: Write the failing tests** (tinanta.rs tests module):

```rust
    #[test]
    fn sarvadhatukam_apit_tags_atmanepada_endings_ngit() {
        let mut p = Prakriya {
            terms: vec![Term::new("laB"), Term::new("ta")],
            log: vec![],
            ctx: Context::new(Lakara::Lat, Pada::Atmanepada, Purusha::Prathama, Vacana::Eka),
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "1.2.4").unwrap();
        assert!((rule.apply)(&mut p));
        assert!(p.terms[ENDING_PRE_SHAP].has(Tag::Ngit));
        assert!(p.log.iter().any(|s| s.sutra == "1.2.4"));
    }

    #[test]
    fn sarvadhatukam_apit_skips_parasmaipada_and_lot_uttama() {
        // Parasmaipada apit endings are Nid-vat in principle too, but no
        // implemented rule consumes the fact and firing here would perturb
        // the 216 pinned parasmaipada traces (see the spec). Lot uttama is a
        // GENUINE exclusion: 3.4.92's own "pic ca" makes those endings pit,
        // hence not apit — which is what keeps 7.2.81 off the AT-agama.
        let cases = [
            ("ti", Lakara::Lat, Pada::Parasmaipada, Purusha::Prathama),
            ("iw", Lakara::Lot, Pada::Atmanepada, Purusha::Uttama),
        ];
        for (ending, lakara, pada, purusha) in cases {
            let mut p = Prakriya {
                terms: vec![Term::new("laB"), Term::new(ending)],
                log: vec![],
                ctx: Context::new(lakara, pada, purusha, Vacana::Eka),
            };
            let rule = TINANTA_RULES.iter().find(|r| r.id == "1.2.4").unwrap();
            assert!(!(rule.apply)(&mut p), "{ending} {lakara:?} {pada:?}");
            assert!(!p.terms[ENDING_PRE_SHAP].has(Tag::Ngit));
        }
    }
```

- [ ] **Step 2: Run to verify failure**

Run: `cd /workspace && mise run test -- -p panini-prakriya`
Expected: FAIL — no rule with id "1.2.4".

- [ ] **Step 3: Implement** — insert after the `1.3.9` rule entry and before `3.4.85` in `TINANTA_RULES`:

```rust
    // 1.2.4 sārvadhātukam apit: an apit sārvadhātuka behaves as ṅit. An
    // atideśa (the 3.4.85 precedent): a rule that appears in the trace and
    // sets a term-level tag — distinct from ctx.is_ngit_like, which says the
    // *lakāra* is ṅit and drives 3.4.99/100/101.
    //
    // Guard notes (see the spec's 1.2.4 section):
    // - Ātmanepada only in this slice: parasmaipada apit endings (tas, Ji…)
    //   are equally ṅid-vat in principle, but no implemented rule consumes
    //   that fact, and firing here would add a step to the pinned
    //   parasmaipada traces. Widening later is additive, not a fix.
    // - Loṭ uttama is a genuine exclusion, not trace-minimalism: 3.4.92's
    //   own "pic ca" makes those endings pit, hence not apit — which is what
    //   keeps 7.2.81 off the āṭ-āgama (AvahE goes to 6.1.101 instead).
    Rule {
        id: "1.2.4",
        name: "sArvaDAtukam apit",
        kind: RuleKind::Atidesha,
        apply: |p| {
            if !matches!(p.ctx.pada, Pada::Atmanepada)
                || (matches!(p.ctx.lakara, Lakara::Lot)
                    && matches!(p.ctx.purusha, Purusha::Uttama))
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].add(Tag::Ngit);
            p.record("1.2.4", "sArvaDAtukam apit", before);
            true
        },
    },
```

(Use the Task 4-confirmed name if it differs.)

- [ ] **Step 4: Run the suite** — `mise run test`. Expected: PASS (no parasmaipada trace changes; ātmanepada traces are not pinned yet).

- [ ] **Step 5: Format and commit**

```bash
mise run fmt
git add crates/panini-prakriya
git commit -m "feat(prakriya): 1.2.4 sarvadhatukam apit — term-level Nit on atmanepada endings"
```

---

### Task 6: laṭ ātmanepada — the ṭit chain core

Rules: 3.4.80 *thāsaḥ se*, 3.4.79 *ṭita ātmanepadānām ter e*, 7.2.81 *āto ṅitaḥ*, 7.3.86 *pugantalaghūpadhasya ca*, and the 6.1.97 pararūpa widening. Deliverable: all 54 laṭ ātmanepada golden forms.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs`
- Modify: `crates/panini/tests/paradigm.rs`, `crates/panini/tests/trace.rs`

**Interfaces:**
- Consumes: `Tag::Ngit` (Task 5).
- Produces: rules `3.4.80` then `3.4.79` (in that order, both directly after `3.4.100` and before `3.4.92`); rule `7.2.81` directly after `7.2.80`; rule `7.3.86` directly after `7.3.84`; widened `6.1.97`. Tasks 7–9 rely on 3.4.79/3.4.80/7.2.81 exactly as specified here.

- [ ] **Step 1: Write the failing unit tests**:

```rust
    fn lat_a_form(code: &str, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, Lakara::Lat, Pada::Atmanepada, pu, va).text()
    }

    #[test]
    fn labh_lat_atmanepada_all_nine_cells() {
        let expected = [
            (Purusha::Prathama, Vacana::Eka, "laBate"),
            (Purusha::Prathama, Vacana::Dvi, "laBete"),
            (Purusha::Prathama, Vacana::Bahu, "laBante"),
            (Purusha::Madhyama, Vacana::Eka, "laBase"),
            (Purusha::Madhyama, Vacana::Dvi, "laBeTe"),
            (Purusha::Madhyama, Vacana::Bahu, "laBaDve"),
            (Purusha::Uttama, Vacana::Eka, "laBe"),
            (Purusha::Uttama, Vacana::Dvi, "laBAvahe"),
            (Purusha::Uttama, Vacana::Bahu, "laBAmahe"),
        ];
        for (pu, va, form) in expected {
            assert_eq!(lat_a_form("laB", pu, va), form, "{pu:?} {va:?}");
        }
    }

    #[test]
    fn vrt_lat_uses_laghupadha_guna() {
        // vft's f is PENULTIMATE (upadha), not final like smf's: guna comes
        // from 7.3.86 pugantalaghUpaDasya ca, not 7.3.84.
        assert_eq!(lat_a_form("vft", Purusha::Prathama, Vacana::Eka), "vartate");
    }

    #[test]
    fn thasah_se_precedes_and_preempts_ter_e() {
        // 3.4.80 is the apavada: TAs -> se. Reversed order would give 3.4.79
        // TAs -> Te (wrong). And 3.4.79 must report false on "se" (ti of
        // "se" is already e) rather than record a no-op step.
        let mut p = Prakriya {
            terms: vec![Term::new("laB"), Term::new("se")],
            log: vec![],
            ctx: Context::new(Lakara::Lat, Pada::Atmanepada, Purusha::Madhyama, Vacana::Eka),
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.79").unwrap();
        assert!(!(rule.apply)(&mut p), "3.4.79 must not record a no-op on se");
    }

    #[test]
    fn ato_nitah_requires_the_ngit_tag() {
        // Parasmaipada lot uttama Ani starts with A but is NOT Nid-vat
        // (1.2.4 pic-ca exclusion) — it belongs to 6.1.101, not 7.2.81.
        let mut anga = Term::new("Bav");
        anga.add(Tag::Anga);
        let mut p = Prakriya {
            terms: vec![anga, Term::new("a"), Term::new("Ani")],
            log: vec![],
            ctx: Context::new(Lakara::Lot, Pada::Parasmaipada, Purusha::Uttama, Vacana::Eka),
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.2.81").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "Ani");
    }
```

- [ ] **Step 2: Run to verify failure** — `mise run test -- -p panini-prakriya`. Expected: FAIL (no 3.4.79/3.4.80/7.2.81/7.3.86 rules; laṭ Ā forms derive as raw `laBata` etc.).

- [ ] **Step 3: Implement the four rules and one widening.**

Insert directly after the `3.4.100` entry (before `3.4.92`), in this order:

```rust
    // 3.4.80 thāsaḥ se: ātmanepada madhyama-eka TAs → se. Apavāda to 3.4.79
    // ṭita… ter e, hence ordered before it: reversed, 3.4.79 would rewrite
    // TAs's ṭi (As → e) to Te and this rule would never see TAs.
    Rule {
        id: "3.4.80",
        name: "TAsaH se",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // Guarded to the ṭit lakāras (laṭ, loṭ): in the ṅit lakāras the
            // 3.4.79 context that 3.4.80 carves out does not apply and TAs
            // survives unchanged (laṅ aBavaTAH).
            if !matches!(p.ctx.lakara, Lakara::Lat | Lakara::Lot)
                || p.terms[ENDING_PRE_SHAP].text != "TAs"
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "se".into();
            p.record("3.4.80", "TAsaH se", before);
            true
        },
    },
    // 3.4.79 ṭita ātmanepadānām ter e: in a ṭit lakāra (laṭ, loṭ — the ṭ
    // anubandha in their names), the ṭi of an ātmanepada ending (its last
    // vowel plus anything after, 1.1.64 aco'ntyādi ṭi) → e.
    // ta→te, AtAm→Ate, Ja→Je, ATAm→ATe, Dvam→Dve, i→e, vahi→vahe, mahi→mahe.
    Rule {
        id: "3.4.79",
        name: "wita AtmanepadAnAM wer e",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lat | Lakara::Lot)
                || !matches!(p.ctx.pada, Pada::Atmanepada)
            {
                return false;
            }
            let text = &p.terms[ENDING_PRE_SHAP].text;
            let Some(ti_start) = text.char_indices().rev().find(|&(_, c)| is_vowel(c)).map(|(i, _)| i)
            else {
                return false;
            };
            let replaced = format!("{}e", &text[..ti_start]);
            if replaced == *text {
                return false; // ṭi is already e (post-3.4.80 "se"): no-op
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = replaced;
            p.record("3.4.79", "wita AtmanepadAnAM wer e", before);
            true
        },
    },
```

Insert directly after the `7.2.80` entry:

```rust
    // 7.2.81 āto ṅitaḥ: after an a-final aṅga (the śap), the initial ā of a
    // ṅit ending → iy. Ate→iyte (laṭ), AtAm→iytAm (laṅ/loṭ), ATe→iyTe.
    // The ṅit condition is the TERM tag from 1.2.4 (laṭ/loṭ are ṭit lakāras,
    // yet their apit ātmanepada endings behave as ṅit) — NOT ctx.is_ngit_like.
    // The tag also keeps this rule off parasmaipada loṭ uttama's āṭ (Ani),
    // which 1.2.4 never tags (pic ca) and which belongs to 6.1.101.
    // MUST precede 6.1.101, which would otherwise dīrgha-merge the tagged
    // A-initial endings (laṭ 3du would surface as laBAte, not laBete).
    Rule {
        id: "7.2.81",
        name: "Ato NitaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms[SHAP].text != "a"
                || !p.terms[ENDING].has(Tag::Ngit)
                || !p.terms[ENDING].text.starts_with('A')
            {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("iy{rest}");
            p.record("7.2.81", "Ato NitaH", before);
            true
        },
    },
```

Insert directly after the `7.3.84` entry:

```rust
    // 7.3.86 pugantalaghūpadhasya ca: guṇa of a light (short, pre-single-
    // consonant) penultimate ik before the sārvadhātuka. vft → vart. The
    // only curated root with an ik upadhā; final-ik roots (BU, smf…) are
    // 7.3.84's business and never reach this shape guard.
    Rule {
        id: "7.3.86",
        name: "pugantalaGUpaDasya ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let n = chars.len();
            if n < 2 || is_vowel(chars[n - 1]) {
                return false; // final-vowel aṅgas are 7.3.84's business
            }
            let Some(g) = guna_of(chars[n - 2]) else {
                return false;
            };
            // laghu: the short ik vowels are exactly the lowercase ones our
            // guna_of accepts; long variants are guru and out of scope here.
            if !matches!(chars[n - 2], 'i' | 'u' | 'f' | 'x') {
                return false;
            }
            let before = p.snapshot();
            let mut s: String = chars[..n - 2].iter().collect();
            s.push_str(g);
            s.push(chars[n - 1]);
            p.terms[ANGA].text = s;
            p.record("7.3.86", "pugantalaGUpaDasya ca", before);
            true
        },
    },
```

Widen `6.1.97` (pararūpa for any guṇa vowel after śap `a`, not just `a`); replace its guard and body:

```rust
        apply: |p| {
            let Some(first) = p.terms[ENDING].text.chars().next() else {
                return false;
            };
            if p.terms[SHAP].text != "a" || !matches!(first, 'a' | 'e' | 'o') {
                return false;
            }
            let before = p.snapshot();
            // Para-rūpa: the single substitute is the FOLLOWING vowel. For
            // a+a the śap already spells it; for a+e (laṭ Ā uttama-eka
            // laB+a+e → laBe) the śap must become that vowel.
            p.terms[SHAP].text = first.to_string();
            p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
            p.record("6.1.97", "ato guRe", before);
            true
        },
```

and extend its doc comment: pararūpa now covers `a+a` (anti) and `a+e` (laṭ ātmanepada uttama-eka); no `a+o` case arises but the guard states the sūtra's set.

- [ ] **Step 4: Run the unit tests** — `mise run test -- -p panini-prakriya`. Expected: PASS.

- [ ] **Step 5: Add the laṭ golden block.** Append to `PARADIGM` in `crates/panini/tests/paradigm.rs` (order within each array: P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B):

```rust
    (
        "eD",
        "laT",
        [
            "eDate", "eDete", "eDante", "eDase", "eDeTe", "eDaDve", "eDe", "eDAvahe", "eDAmahe",
        ],
    ),
    (
        "laB",
        "laT",
        [
            "laBate", "laBete", "laBante", "laBase", "laBeTe", "laBaDve", "laBe", "laBAvahe",
            "laBAmahe",
        ],
    ),
    (
        "sev",
        "laT",
        [
            "sevate", "sevete", "sevante", "sevase", "seveTe", "sevaDve", "seve", "sevAvahe",
            "sevAmahe",
        ],
    ),
    (
        "vft",
        "laT",
        [
            "vartate", "vartete", "vartante", "vartase", "varteTe", "vartaDve", "varte",
            "vartAvahe", "vartAmahe",
        ],
    ),
    (
        "BAz",
        "laT",
        [
            "BAzate", "BAzete", "BAzante", "BAzase", "BAzeTe", "BAzaDve", "BAze", "BAzAvahe",
            "BAzAmahe",
        ],
    ),
    (
        "Ikz",
        "laT",
        [
            "Ikzate", "Ikzete", "Ikzante", "Ikzase", "IkzeTe", "IkzaDve", "Ikze", "IkzAvahe",
            "IkzAmahe",
        ],
    ),
```

- [ ] **Step 6: Pin the new traces.** Add to `crates/panini/tests/trace.rs`:

```rust
#[test]
fn labhate_trace_is_exactly_the_minimal_atmanepada_path() {
    // laB laṭ prathama eka: pada sanction (1.3.12), ṅid-vat atideśa (1.2.4),
    // ta → te (3.4.79). No it-saṃjñā step for `ta` (nothing to strip).
    assert_eq!(
        trace_for("laBate"),
        vec!["1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.68", "1.3.9"]
    );
}

#[test]
fn labhete_trace_is_exactly_the_ato_nitah_path() {
    // laB laṭ prathama dvi: AtAm → Ate (3.4.79) → iyte (7.2.81) →
    // laBe+yte (6.1.87) → laBete (6.1.66).
    assert_eq!(
        trace_for("laBete"),
        vec!["1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.68", "1.3.9", "7.2.81", "6.1.87", "6.1.66"]
    );
}

#[test]
fn vartate_trace_shows_laghupadha_guna() {
    // vft: 7.3.86 (upadhā guṇa), NOT 7.3.84 (final-ik guṇa).
    let trace = trace_for("vartate");
    assert!(trace.contains(&"7.3.86".to_string()), "got {trace:?}");
    assert!(!trace.contains(&"7.3.84".to_string()), "got {trace:?}");
}
```

- [ ] **Step 7: Run everything** — `mise run test`. Expected: PASS, including all 216 pre-existing golden forms.

- [ ] **Step 8: Format and commit**

```bash
mise run fmt
git add -A
git commit -m "feat(prakriya): lat atmanepada — 3.4.80, 3.4.79, 7.2.81, 7.3.86; 6.1.97 pararupa widening; golden lat block"
```

---

### Task 7: loṭ ātmanepada — ām etaḥ and the āṭ merge

Rules: 3.4.91 *savābhyāṃ vāmau*, 3.4.93 *eta ai*, 3.4.90 *ām etaḥ*, 6.1.90 *āṭaś ca* (both arms), and the 3.4.92 arm-set widening. Deliverable: all 54 loṭ ātmanepada golden forms.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs`
- Modify: `crates/panini/tests/paradigm.rs`, `crates/panini/tests/trace.rs`

**Interfaces:**
- Consumes: 3.4.80/3.4.79 (Task 6) — the shapes `se`, `Dve`, `e`, `vahe`, `mahe` exist only after them.
- Produces: rules `3.4.91`, `3.4.93`, `3.4.90` (in that order, directly after `3.4.79` and before `3.4.92`); widened `3.4.92`; rule `6.1.90` directly after `6.1.101`. Task 8 uses the aṅga arm of 6.1.90.

- [ ] **Step 1: Write the failing unit tests**:

```rust
    fn lot_a_form(code: &str, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, Lakara::Lot, Pada::Atmanepada, pu, va).text()
    }

    #[test]
    fn labh_lot_atmanepada_all_nine_cells() {
        let expected = [
            (Purusha::Prathama, Vacana::Eka, "laBatAm"),
            (Purusha::Prathama, Vacana::Dvi, "laBetAm"),
            (Purusha::Prathama, Vacana::Bahu, "laBantAm"),
            (Purusha::Madhyama, Vacana::Eka, "laBasva"),
            (Purusha::Madhyama, Vacana::Dvi, "laBeTAm"),
            (Purusha::Madhyama, Vacana::Bahu, "laBaDvam"),
            (Purusha::Uttama, Vacana::Eka, "laBE"),
            (Purusha::Uttama, Vacana::Dvi, "laBAvahE"),
            (Purusha::Uttama, Vacana::Bahu, "laBAmahE"),
        ];
        for (pu, va, form) in expected {
            assert_eq!(lot_a_form("laB", pu, va), form, "{pu:?} {va:?}");
        }
    }

    #[test]
    fn savabhyam_vamau_preempts_am_etah() {
        // 3.4.91 (se→sva, Dve→Dvam) is the apavāda ordered before 3.4.90:
        // reversed, se would become sAm and Dve DvAm.
        assert_eq!(lot_a_form("laB", Purusha::Madhyama, Vacana::Eka), "laBasva");
        assert_eq!(lot_a_form("laB", Purusha::Madhyama, Vacana::Bahu), "laBaDvam");
    }

    #[test]
    fn am_etah_is_lot_only() {
        // laṭ's te/Ate must NOT become tAm/AtAm.
        assert_eq!(lat_a_form("laB", Purusha::Prathama, Vacana::Eka), "laBate");
    }
```

- [ ] **Step 2: Run to verify failure** — `mise run test -- -p panini-prakriya`. Expected: FAIL (loṭ Ā currently derives laṭ-shaped text).

- [ ] **Step 3: Implement.** Insert directly after the new `3.4.79` entry (still before `3.4.92`), in this order:

```rust
    // 3.4.91 savābhyāṃ vāmau: loṭ's e → va after s, → am after v.
    // se → sva, Dve → Dvam. Apavāda to 3.4.90 ām etaḥ, hence ordered
    // before it (reversed: se → sAm, Dve → DvAm).
    Rule {
        id: "3.4.91",
        name: "savAByAM vAmO",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) {
                return false;
            }
            let sub = match p.terms[ENDING_PRE_SHAP].text.as_str() {
                "se" => "sva",
                "Dve" => "Dvam",
                _ => return false,
            };
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = sub.into();
            p.record("3.4.91", "savAByAM vAmO", before);
            true
        },
    },
    // 3.4.93 eta ai: loṭ's uttama e → E. Apavāda to 3.4.90 (ordered before
    // it); afterwards the uttama endings are E-final, which 3.4.90's short-e
    // guard ignores — no explicit uttama exclusion needed there.
    Rule {
        id: "3.4.93",
        name: "eta E",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot)
                || !matches!(p.ctx.purusha, Purusha::Uttama)
                || !matches!(p.ctx.pada, Pada::Atmanepada)
                || !p.terms[ENDING_PRE_SHAP].text.ends_with('e')
            {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ENDING_PRE_SHAP].text.chars().collect();
            s.pop();
            s.push('E');
            p.terms[ENDING_PRE_SHAP].text = s.into_iter().collect();
            p.record("3.4.93", "eta E", before);
            true
        },
    },
    // 3.4.90 ām etaḥ: loṭ's ending-final e → Am. te→tAm, Ate→AtAm, Je→JAm,
    // ATe→ATAm. The A-initial results are then reshaped post-śap by 7.2.81
    // (ṅid-vat) exactly like their laṭ counterparts — the net laṭ/loṭ
    // difference in those cells is this rule alone.
    Rule {
        id: "3.4.90",
        name: "Am etaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot)
                || !matches!(p.ctx.pada, Pada::Atmanepada)
                || !p.terms[ENDING_PRE_SHAP].text.ends_with('e')
            {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ENDING_PRE_SHAP].text.chars().collect();
            s.pop();
            p.terms[ENDING_PRE_SHAP].text = s.into_iter().collect::<String>() + "Am";
            p.record("3.4.90", "Am etaH", before);
            true
        },
    },
```

Widen `3.4.92`'s arm set (guard line only):

```rust
                || !matches!(
                    p.terms[ENDING_PRE_SHAP].text.as_str(),
                    "ni" | "va" | "ma" | "E" | "vahE" | "mahE"
                )
```

with the comment extended: `E`/`vahE`/`mahE` are the ātmanepada shapes left by 3.4.93, the same explicit-set style as the parasmaipada arms (which 3.4.89/3.4.99 leave); MUST stay ordered after 3.4.93.

Add a vṛddhi helper next to `guna_of`:

```rust
/// Vṛddhi substitute of a vowel (1.1.1 vṛddhir ādaic; only the arms the
/// curated roots exercise via 6.1.90 — e/I from eD/Ikz, E from loṭ's 3.4.93).
fn vrddhi_of(v: char) -> Option<char> {
    match v {
        'a' | 'A' => Some('A'),
        'i' | 'I' | 'e' | 'E' => Some('E'),
        'u' | 'U' | 'o' | 'O' => Some('O'),
        _ => None,
    }
}
```

Insert directly after the `6.1.101` entry:

```rust
    // 6.1.90 āṭaś ca: āṭ + a following vowel yield a single vṛddhi. Two
    // shapes, one sūtra:
    // - Aṅga arm (laṅ, Task 8): 6.4.72's āṭ + the root's initial vowel.
    //   AeD → ED, AIkz → Ekz.
    // - Ending arm (loṭ uttama eka, ātmanepada): after 6.1.101 has coalesced
    //   śap a + āṭ A into śap A, that A + the ending's E merge to E
    //   (laB+A+E → laBE). MUST follow 6.1.101 — before it the shape is
    //   a + AE and this arm cannot see it.
    Rule {
        id: "6.1.90",
        name: "AwaS ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // Aṅga arm: āṭ prefix on a vowel-initial aṅga.
            let anga: Vec<char> = p.terms[ANGA].text.chars().collect();
            if anga.len() >= 2 && anga[0] == 'A' && is_vowel(anga[1]) {
                if let Some(v) = vrddhi_of(anga[1]) {
                    let before = p.snapshot();
                    let mut s = String::new();
                    s.push(v);
                    s.extend(&anga[2..]);
                    p.terms[ANGA].text = s;
                    p.record("6.1.90", "AwaS ca", before);
                    return true;
                }
            }
            // Ending arm: śap A (āṭ via 6.1.101) + ending-initial ec.
            if p.terms.len() > ENDING && p.terms[SHAP].text == "A" {
                if let Some(first) = p.terms[ENDING].text.chars().next()
                    && matches!(first, 'e' | 'E' | 'o' | 'O')
                {
                    let before = p.snapshot();
                    p.terms[SHAP].text = vrddhi_of(first).unwrap().to_string();
                    p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
                    p.record("6.1.90", "AwaS ca", before);
                    return true;
                }
            }
            false
        },
    },
```

**Note:** if Task 4 found the reference attributes the ending merge to 6.1.88 *vṛddhir eci*, split the ending arm into its own `Rule { id: "6.1.88", name: "vfdDir eci", … }` at the same position and keep 6.1.90 aṅga-only.

- [ ] **Step 4: Run the unit tests** — `mise run test -- -p panini-prakriya`. Expected: PASS.

- [ ] **Step 5: Add the loṭ golden block** to `PARADIGM`:

```rust
    (
        "eD",
        "loT",
        [
            "eDatAm", "eDetAm", "eDantAm", "eDasva", "eDeTAm", "eDaDvam", "eDE", "eDAvahE",
            "eDAmahE",
        ],
    ),
    (
        "laB",
        "loT",
        [
            "laBatAm", "laBetAm", "laBantAm", "laBasva", "laBeTAm", "laBaDvam", "laBE",
            "laBAvahE", "laBAmahE",
        ],
    ),
    (
        "sev",
        "loT",
        [
            "sevatAm", "sevetAm", "sevantAm", "sevasva", "seveTAm", "sevaDvam", "sevE",
            "sevAvahE", "sevAmahE",
        ],
    ),
    (
        "vft",
        "loT",
        [
            "vartatAm", "vartetAm", "vartantAm", "vartasva", "varteTAm", "vartaDvam", "vartE",
            "vartAvahE", "vartAmahE",
        ],
    ),
    (
        "BAz",
        "loT",
        [
            "BAzatAm", "BAzetAm", "BAzantAm", "BAzasva", "BAzeTAm", "BAzaDvam", "BAzE",
            "BAzAvahE", "BAzAmahE",
        ],
    ),
    (
        "Ikz",
        "loT",
        [
            "IkzatAm", "IkzetAm", "IkzantAm", "Ikzasva", "IkzeTAm", "IkzaDvam", "IkzE",
            "IkzAvahE", "IkzAmahE",
        ],
    ),
```

- [ ] **Step 6: Pin the new traces** in `trace.rs`:

```rust
#[test]
fn labhasva_trace_is_exactly_the_savabhyam_path() {
    // laB loṭ madhyama eka: TAs → se (3.4.80) → sva (3.4.91); 3.4.79
    // reports false on `se` (its ṭi is already e) and must not appear.
    assert_eq!(
        trace_for("laBasva"),
        vec!["1.3.12", "3.4.78", "1.2.4", "3.4.85", "3.4.80", "3.4.91", "3.1.68", "1.3.9"]
    );
}

#[test]
fn labhai_trace_is_exactly_the_at_vrddhi_path() {
    // laB loṭ uttama eka: iw → i (1.3.9) → e (3.4.79) → E (3.4.93) →
    // AE (3.4.92); post-śap 6.1.101 (a+A → A) then 6.1.90 (A+E → E).
    // No 1.2.4: loṭ uttama endings are pit (pic ca), not apit.
    assert_eq!(
        trace_for("laBE"),
        vec![
            "1.3.12", "3.4.78", "1.3.9", "3.4.85", "3.4.79", "3.4.93", "3.4.92", "3.1.68",
            "1.3.9", "6.1.101", "6.1.90"
        ]
    );
}
```

- [ ] **Step 7: Run everything** — `mise run test`. Expected: PASS.

- [ ] **Step 8: Format and commit**

```bash
mise run fmt
git add -A
git commit -m "feat(prakriya): lot atmanepada — 3.4.91, 3.4.93, 3.4.90, 6.1.90; 3.4.92 arm-set widening; golden lot block"
```

---

### Task 8: laṅ ātmanepada — the āṭ augment for vowel-initial roots

No new ending rules (laṅ ātmanepada endings pass through untouched pre-śap; 7.2.81/6.1.87 from Task 6 do the post-śap work). New: 6.4.72 *āḍ ajādīnām* and the 6.4.71 consonant-initial widening. Deliverable: all 54 laṅ ātmanepada golden forms.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs`
- Modify: `crates/panini/tests/paradigm.rs`, `crates/panini/tests/trace.rs`

**Interfaces:**
- Consumes: 6.1.90 aṅga arm (Task 7), 7.2.81 (Task 6).
- Produces: rule `6.4.72` directly after `6.4.71`; widened `6.4.71`.

- [ ] **Step 1: Write the failing unit tests**:

```rust
    fn lan_a_form(code: &str, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, Lakara::Lan, Pada::Atmanepada, pu, va).text()
    }

    #[test]
    fn labh_lan_atmanepada_all_nine_cells() {
        let expected = [
            (Purusha::Prathama, Vacana::Eka, "alaBata"),
            (Purusha::Prathama, Vacana::Dvi, "alaBetAm"),
            (Purusha::Prathama, Vacana::Bahu, "alaBanta"),
            (Purusha::Madhyama, Vacana::Eka, "alaBaTAH"),
            (Purusha::Madhyama, Vacana::Dvi, "alaBeTAm"),
            (Purusha::Madhyama, Vacana::Bahu, "alaBaDvam"),
            (Purusha::Uttama, Vacana::Eka, "alaBe"),
            (Purusha::Uttama, Vacana::Dvi, "alaBAvahi"),
            (Purusha::Uttama, Vacana::Bahu, "alaBAmahi"),
        ];
        for (pu, va, form) in expected {
            assert_eq!(lan_a_form("laB", pu, va), form, "{pu:?} {va:?}");
        }
    }

    #[test]
    fn vowel_initial_roots_take_at_not_a() {
        // 6.4.72 āḍ ajādīnām (apavāda to 6.4.71) + 6.1.90 vṛddhi:
        // a+eD → ED (aidhata), a+Ikz → Ekz (aikṣata).
        assert_eq!(lan_a_form("eD", Purusha::Prathama, Vacana::Eka), "EData");
        assert_eq!(lan_a_form("Ikz", Purusha::Prathama, Vacana::Eka), "Ekzata");
    }
```

- [ ] **Step 2: Run to verify failure** — `mise run test -- -p panini-prakriya`. Expected: the laB cells PASS already (Tasks 2+6 built them); the vowel-initial test FAILS (`aeData`).

- [ ] **Step 3: Implement.** Widen `6.4.71`'s guard:

```rust
            let first = p.terms[ANGA].text.chars().next().unwrap();
            if !matches!(p.ctx.lakara, Lakara::Lan) || is_vowel(first) {
                return false;
            }
```

(comment: vowel-initial aṅgas take the apavāda 6.4.72 āṭ instead). Insert directly after it:

```rust
    // 6.4.72 āḍ ajādīnām: vowel-initial aṅgas take the āṭ-āgama in laṅ
    // (apavāda to 6.4.71's aṭ). The A then merges with the root's initial
    // vowel by 6.1.90 āṭaś ca into vṛddhi: a+eD → ED, a+Ikz → Ekz.
    Rule {
        id: "6.4.72",
        name: "Aq ajAdInAm",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let first = p.terms[ANGA].text.chars().next().unwrap();
            if !matches!(p.ctx.lakara, Lakara::Lan) || !is_vowel(first) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = format!("A{}", p.terms[ANGA].text);
            p.record("6.4.72", "Aq ajAdInAm", before);
            true
        },
    },
```

- [ ] **Step 4: Run the unit tests** — `mise run test -- -p panini-prakriya`. Expected: PASS.

- [ ] **Step 5: Add the laṅ golden block** to `PARADIGM`:

```rust
    (
        "eD",
        "laN",
        [
            "EData", "EDetAm", "EDanta", "EDaTAH", "EDeTAm", "EDaDvam", "EDe", "EDAvahi",
            "EDAmahi",
        ],
    ),
    (
        "laB",
        "laN",
        [
            "alaBata", "alaBetAm", "alaBanta", "alaBaTAH", "alaBeTAm", "alaBaDvam", "alaBe",
            "alaBAvahi", "alaBAmahi",
        ],
    ),
    (
        "sev",
        "laN",
        [
            "asevata", "asevetAm", "asevanta", "asevaTAH", "aseveTAm", "asevaDvam", "aseve",
            "asevAvahi", "asevAmahi",
        ],
    ),
    (
        "vft",
        "laN",
        [
            "avartata", "avartetAm", "avartanta", "avartaTAH", "avarteTAm", "avartaDvam",
            "avarte", "avartAvahi", "avartAmahi",
        ],
    ),
    (
        "BAz",
        "laN",
        [
            "aBAzata", "aBAzetAm", "aBAzanta", "aBAzaTAH", "aBAzeTAm", "aBAzaDvam", "aBAze",
            "aBAzAvahi", "aBAzAmahi",
        ],
    ),
    (
        "Ikz",
        "laN",
        [
            "Ekzata", "EkzetAm", "Ekzanta", "EkzaTAH", "EkzeTAm", "EkzaDvam", "Ekze",
            "EkzAvahi", "EkzAmahi",
        ],
    ),
```

- [ ] **Step 6: Pin the āṭ trace** in `trace.rs`:

```rust
#[test]
fn aidhata_trace_is_exactly_the_at_agama_path() {
    // eD laṅ prathama eka: no pre-śap ending change (ta survives; 3.4.100 is
    // parasmaipada-only), then 6.4.72 āṭ + 6.1.90 vṛddhi on the aṅga.
    assert_eq!(
        trace_for("EData"),
        vec!["1.3.12", "3.4.78", "1.2.4", "3.1.68", "1.3.9", "6.4.72", "6.1.90"]
    );
}
```

- [ ] **Step 7: Run everything** — `mise run test`. Expected: PASS.

- [ ] **Step 8: Format and commit**

```bash
mise run fmt
git add -A
git commit -m "feat(prakriya): lan atmanepada — 6.4.72 AT-agama, 6.4.71 consonant-initial guard; golden lan block"
```

---

### Task 9: vidhiliṅ ātmanepada — the sīyuṭ chain

Rules: 3.4.105 *jhasya ran*, 3.4.106 *iṭo 't*, 3.4.102 *liṅaḥ sīyuṭ*, and the 6.1.87 long-I widening. Deliverable: all 54 vidhiliṅ ātmanepada golden forms — completing the 432-form grid.

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs`
- Modify: `crates/panini/tests/paradigm.rs`, `crates/panini/tests/trace.rs`

**Interfaces:**
- Consumes: 7.2.79 (existing — sīyuṭ's non-final `s` is exactly the "non-final s" its comment already scopes), 6.1.87, 6.1.66 (existing).
- Produces: rules `3.4.105` and `3.4.106` directly after `3.4.108`; rule `3.4.102` directly after `3.4.103`; widened `6.1.87`.

- [ ] **Step 1: Write the failing unit tests**:

```rust
    fn lin_a_form(code: &str, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, Lakara::VidhiLin, Pada::Atmanepada, pu, va).text()
    }

    #[test]
    fn labh_vidhilin_atmanepada_all_nine_cells() {
        let expected = [
            (Purusha::Prathama, Vacana::Eka, "laBeta"),
            (Purusha::Prathama, Vacana::Dvi, "laBeyAtAm"),
            (Purusha::Prathama, Vacana::Bahu, "laBeran"),
            (Purusha::Madhyama, Vacana::Eka, "laBeTAH"),
            (Purusha::Madhyama, Vacana::Dvi, "laBeyATAm"),
            (Purusha::Madhyama, Vacana::Bahu, "laBeDvam"),
            (Purusha::Uttama, Vacana::Eka, "laBeya"),
            (Purusha::Uttama, Vacana::Dvi, "laBevahi"),
            (Purusha::Uttama, Vacana::Bahu, "laBemahi"),
        ];
        for (pu, va, form) in expected {
            assert_eq!(lin_a_form("laB", pu, va), form, "{pu:?} {va:?}");
        }
    }

    #[test]
    fn siyut_survives_salopa_as_long_i() {
        // sIyta → (7.2.79) Iyta: 6.1.87's widened guard must accept the
        // long I (yāsuṭ's chain produced short iy via 7.2.80).
        let p = {
            let d = dhatus().iter().find(|d| d.code == "laB").unwrap();
            derive(d, Lakara::VidhiLin, Pada::Atmanepada, Purusha::Prathama, Vacana::Eka)
        };
        assert!(p.log.iter().any(|s| s.sutra == "3.4.102"));
        assert!(p.log.iter().any(|s| s.sutra == "7.2.79"));
        assert!(p.log.iter().any(|s| s.sutra == "6.1.87"));
        assert_eq!(p.text(), "laBeta");
    }
```

- [ ] **Step 2: Run to verify failure** — `mise run test -- -p panini-prakriya`. Expected: FAIL (no 3.4.102/105/106).

- [ ] **Step 3: Implement.** Insert directly after the `3.4.108` entry:

```rust
    // 3.4.105 jhasya ran: in liṅ, ātmanepada Ja → ran. Apavāda to 7.1.3
    // jho'ntaḥ by position: 7.1.3 runs post-śap, by which time Ja is gone.
    // The liṅ ātmanepada sibling of 3.4.108 jher jus.
    Rule {
        id: "3.4.105",
        name: "Jasya ran",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin)
                || p.terms[ENDING_PRE_SHAP].text != "Ja"
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "ran".into();
            p.record("3.4.105", "Jasya ran", before);
            true
        },
    },
    // 3.4.106 iṭo 't: in liṅ, the ātmanepada uttama-eka i (from iw) → a.
    // laBeya, not laBeyi.
    Rule {
        id: "3.4.106",
        name: "iwo 't",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin)
                || !matches!(p.ctx.pada, Pada::Atmanepada)
                || p.terms[ENDING_PRE_SHAP].text != "i"
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "a".into();
            p.record("3.4.106", "iwo 't", before);
            true
        },
    },
```

(`Ja` only carries a liṅ reading in ātmanepada — parasmaipada liṅ replaced `Ji` via 3.4.108 — so 3.4.105's text guard alone is exact, matching the 3.4.87/3.4.89 style.)

Insert directly after the `3.4.103` entry:

```rust
    // 3.4.102 liṅaḥ sīyuṭ: liṅ's ātmanepada endings take the sīyuṭ-āgama,
    // prefixed as text like yāsuṭ (3.4.103). Its s is non-final, so the
    // existing 7.2.79 salopa elides it: sIyta → Iyta — then 6.1.87 (a+I→e)
    // and 6.1.66 finish exactly as in the yāsuṭ chain.
    // Same ordering constraint as 3.4.103: MUST follow the ending
    // substitutions (3.4.105/3.4.106 match exact text).
    Rule {
        id: "3.4.102",
        name: "liNaH sIyuw",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin)
                || !matches!(p.ctx.pada, Pada::Atmanepada)
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = format!("sIy{}", p.terms[ENDING_PRE_SHAP].text);
            p.record("3.4.102", "liNaH sIyuw", before);
            true
        },
    },
```

Widen `6.1.87`'s guard from `starts_with('i')` to:

```rust
            let first = p.terms[ENDING].text.chars().next();
            if p.terms[SHAP].text != "a" || !matches!(first, Some('i') | Some('I')) {
                return false;
            }
```

(comment: short `iy` comes from 7.2.80/7.2.81; long `Iy` is sīyuṭ after salopa. Both coalesce with śap `a` to guṇa `e`.)

Also update 7.2.79's comment ("Every non-final s reaching this rule is yāsuṭ-derived") — it is now yāsuṭ- or sīyuṭ-derived; the invariant "the only non-final s is āgama-initial" still holds.

- [ ] **Step 4: Run the unit tests** — `mise run test -- -p panini-prakriya`. Expected: PASS.

- [ ] **Step 5: Add the vidhiliṅ golden block** to `PARADIGM`:

```rust
    (
        "eD",
        "viDiliN",
        [
            "eDeta", "eDeyAtAm", "eDeran", "eDeTAH", "eDeyATAm", "eDeDvam", "eDeya", "eDevahi",
            "eDemahi",
        ],
    ),
    (
        "laB",
        "viDiliN",
        [
            "laBeta", "laBeyAtAm", "laBeran", "laBeTAH", "laBeyATAm", "laBeDvam", "laBeya",
            "laBevahi", "laBemahi",
        ],
    ),
    (
        "sev",
        "viDiliN",
        [
            "seveta", "seveyAtAm", "severan", "seveTAH", "seveyATAm", "seveDvam", "seveya",
            "sevevahi", "sevemahi",
        ],
    ),
    (
        "vft",
        "viDiliN",
        [
            "varteta", "varteyAtAm", "varteran", "varteTAH", "varteyATAm", "varteDvam",
            "varteya", "vartevahi", "vartemahi",
        ],
    ),
    (
        "BAz",
        "viDiliN",
        [
            "BAzeta", "BAzeyAtAm", "BAzeran", "BAzeTAH", "BAzeyATAm", "BAzeDvam", "BAzeya",
            "BAzevahi", "BAzemahi",
        ],
    ),
    (
        "Ikz",
        "viDiliN",
        [
            "Ikzeta", "IkzeyAtAm", "Ikzeran", "IkzeTAH", "IkzeyATAm", "IkzeDvam", "Ikzeya",
            "Ikzevahi", "Ikzemahi",
        ],
    ),
```

- [ ] **Step 6: Pin the new traces** in `trace.rs`:

```rust
#[test]
fn labheran_trace_is_exactly_the_siyut_ran_path() {
    // laB vidhiliṅ prathama bahu: Ja → ran (3.4.105) → sIyran (3.4.102) →
    // Iyran (7.2.79) → laBe+yran (6.1.87) → laBeran (6.1.66; r is val).
    assert_eq!(
        trace_for("laBeran"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.105", "3.4.102", "3.1.68", "1.3.9", "7.2.79",
            "6.1.87", "6.1.66"
        ]
    );
}

#[test]
fn labheya_trace_is_exactly_the_ito_t_path() {
    // laB vidhiliṅ uttama eka: iw → i (1.3.9) → a (3.4.106) → sIya
    // (3.4.102) → Iya (7.2.79) → laBe+ya (6.1.87); the y SURVIVES 6.1.66
    // (a is a vowel, not val).
    assert_eq!(
        trace_for("laBeya"),
        vec![
            "1.3.12", "3.4.78", "1.3.9", "1.2.4", "3.4.106", "3.4.102", "3.1.68", "1.3.9",
            "7.2.79", "6.1.87"
        ]
    );
}
```

Note the order in `laBeya`: the shared it-saṃjñā rule (`1.3.9`) precedes `1.2.4` in the pipeline, so for `iw` both record, 1.3.9 first. In `laBeran` the ending `Ja` has no anubandha, so no pre-śap 1.3.9 step. Update the trace.rs header comment's test count (twelve → nineteen full/partial pins after Tasks 6–9).

- [ ] **Step 7: Run everything** — `mise run test`. Expected: PASS — the full 432-form grid is live.

- [ ] **Step 8: Format and commit**

```bash
mise run fmt
git add -A
git commit -m "feat(prakriya): vidhilin atmanepada — 3.4.105, 3.4.106, 3.4.102 siyut; 6.1.87 long-I; golden vidhilin block"
```

---

### Task 10: Cross-lakāra negatives, docs, and full verification

**Files:**
- Modify: `crates/panini/tests/paradigm.rs`
- Modify: `AGENTS.md`, `docs/ARCHITECTURE.md`, `README.md` (scope statements)

- [ ] **Step 1: Add cross-lakāra ātmanepada negatives** to `known_nonforms_are_invalid` (each is a plausible confusion, not junk):

```rust
        // Cross-lakāra atmanepada confusions.
        "alaBeta",   // laN's augment on a vidhilin form
        "laBatam",   // parasmaipada dual ending on an atmanepadin root
        "laBAte",    // 7.2.81 skipped: A must become iy after the shap
        "laBesva",   // lot's sva on a lat stem (3.4.91 without 3.4.90's lakara)
        "IkzAmi",    // parasmaipada uttama ending on the vowel-initial A-root
```

Verify first (run `cargo run -p panini-cli -- check <form>` for each) that none is a valid cell in the 432-form grid — e.g. `laBatam` must not collide with any ātmanepada cell (the valid dual forms are `laBetAm`/`laBeTAm`).

- [ ] **Step 2: Run** — `mise run test`. Expected: PASS. If any new negative comes back VALID, that is a real engine bug found by the test — stop and fix before proceeding (most likely a guard from Tasks 6–9 firing too widely).

- [ ] **Step 3: Update docs.**
- `AGENTS.md` "Rules of the codebase": golden test line becomes "432 forms across laṭ/laṅ/loṭ/vidhiliṅ × parasmaipada/ātmanepada".
- `docs/ARCHITECTURE.md`: `panini-analyze` line — candidates now `(root, lakāra, pada, puruṣa, vacana)` with the pada taken from the root's tag; crates list note for `panini-data` mentions the pada axis.
- `README.md`: if it states the covered slice, extend it the same way (check with `grep -n "parasmaipada\|216\|lakāra" README.md`).

- [ ] **Step 4: Full gates**

Run: `cd /workspace && mise run fmt-check && mise run lint && mise run test`
Expected: all PASS.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "test: atmanepada cross-lakara negatives; docs: two-pada scope in AGENTS, ARCHITECTURE, README"
```

---

### Task 11: Mutation testing — pin the new guards

**Files:**
- Modify: `crates/panini-prakriya/src/tinanta.rs` and/or `crates/panini/tests/*` (survivor-killing tests only)

- [ ] **Step 1: Install the dev toolchain if absent**: `MISE_ENV=dev mise install`
- [ ] **Step 2: Run mutants** (long-running — expect tens of minutes):

Run: `cd /workspace && mise run mutants`
Expected: mutants report in `mutants.out/`; the run must complete.

- [ ] **Step 3: Triage survivors.** For each surviving mutant in a rule added or widened by this plan (`grep -l "3.4.79\|3.4.80\|3.4.90\|3.4.91\|3.4.93\|3.4.102\|3.4.105\|3.4.106\|1.3.12\|1.3.78\|1.2.4\|6.4.72\|6.1.90\|7.2.81\|7.3.86" mutants.out/caught.txt mutants.out/missed.txt`), add a targeted unit test that kills it — the repo's convention is a test named for the guard arm (see `itash_ca_never_touches_lot_even_when_ngit_like` and commit 0899c69's pattern). Typical expected survivors and their killers:
  - `&&`→`||` in a compound guard → a unit test constructing the exact Prakriya where only one conjunct holds, asserting the rule reports `false`.
  - Deleted `p.blocked = true` in 1.3.12/1.3.78 → already killed by `pada_sanction_blocks_wrong_pada_derivations` (verify it appears in `caught.txt`).
  - Swapped match arms in `vrddhi_of` → already killed by `EData`/`Ekzata` goldens if both run; otherwise add direct table assertions like `guna_of_ik_vowels_all_arms` does for `guna_of`.
- [ ] **Step 4: Re-run mutants** on the touched files until no new-rule survivors remain (pre-existing survivors outside this slice's rules are out of scope).
- [ ] **Step 5: Commit**

```bash
mise run fmt
git add -A
git commit -m "test(prakriya): pin atmanepada guard arms against surviving mutants"
```

---

## Completion

After Task 11: run the full gates one last time (`mise run fmt-check && mise run lint && mise run test`), then use superpowers:finishing-a-development-branch — the work is on the `atmanepada` branch; the repo's convention is a PR to `main` (see PR #4 for the vidhiliṅ precedent).
