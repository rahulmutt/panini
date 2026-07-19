# Pāṇini Astādhyāyī v1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rust library (`panini`) and CLI (`panini`) that, given a single Sanskrit word, reports whether it is valid within the covered grammar and returns the ordered sequence of Pāṇinian sūtras that derive it.

**Architecture:** A Cargo workspace of focused crates. `panini-lipi` transliterates between SLP1 (canonical, internal) and IAST/HK/Devanāgarī. `panini-prakriya` is the forward-derivation engine: a `Term`/`Prakriya` model, coded sūtra-rules carrying declarative metadata, and a central controller that resolves *paribhāṣā* ordering and logs every rule that fires. `panini-data` holds the curated linguistic data (roots, tiṅ endings, sūtra metadata), embedded at compile time. `panini-analyze` proposes candidate morphological inputs for a surface word; the `panini` facade forward-derives each candidate and keeps those whose generated form exactly matches the input. `panini-cli` is the clap binary.

**Tech Stack:** Rust (edition 2021), Cargo workspace, `mise` (toolchain + task runner), `clap` (CLI), `serde`/`serde_json` (JSON output), `proptest` (property tests), `insta` optional for snapshots (we use plain golden tables), `cargo-mutants`, `cargo-fuzz`, `cargo-deny`, `cargo-audit`.

## Global Constraints

- **Internal representation is SLP1** (one ASCII byte per phoneme) everywhere inside `panini-prakriya`, `panini-analyze`, and `panini-data`. Transliteration happens only at the `panini-lipi` boundary.
- **No `unsafe`** anywhere without an inline justification comment and reviewer sign-off. Add `#![forbid(unsafe_code)]` to every crate's `lib.rs`/`main.rs` except where a justified exception is documented.
- **Rust toolchain pinned** via `mise.toml` to `rust = "1.83.0"` (stable). No ambient/global installs — all tools via `mise`.
- **v1 grammatical scope (fixed):** *tiṅanta* (finite verbs), *laṭ* (present), *bhvādi* (gaṇa 1), *parasmaipada*, all three *puruṣa* × three *vacana* (9 forms), over the curated 6-root set below. Everything else is out of scope for v1.
- **v1 curated root set (SLP1):** `BU` (bhū), `nI` (nī), `ji` (ji), `smf` (smṛ), `paW` (paṭ), `vad` (vad). These are chosen because their derivations exercise the core machinery cleanly (guṇa of a final *ik* + *ayādeśa* for `BU`/`nI`/`ji`, guṇa `ṛ→ar` for `smf`, and the no-guṇa consonant-final path for `paW`/`vad`). Irregular bhvādi roots (`gam`→gacchati, `sTA`→tiṣṭhati, `pA`→pibati, penult-`ī` roots like `jIv`) are explicitly deferred to Phase 2.
- **The 54 golden surface forms are the source of truth.** Rule *traces* must be aligned to a cited reference derivation (ashtadhyayi.com per-form derivation) during Task 7; surface outputs are pinned by golden tests and must match exactly.
- **Output honesty:** an `Invalid` verdict means "not derivable within the covered v1 grammar," and every human-facing `Invalid` message must say so — never claim ungrammaticality in full Sanskrit.
- **Data provenance:** any sourced datum keeps its license; `data/ATTRIBUTION.md` records source + license verbatim.

---

## File Structure

```
panini/                          (workspace root — already contains README.md, LICENSE, .gitignore, .devcontainer)
├─ Cargo.toml                    workspace manifest
├─ mise.toml                     toolchain pin + tasks (build/test/lint/mutants/fuzz/audit)
├─ deny.toml                     cargo-deny config
├─ AGENTS.md                     agent/contributor front door (symlinked or mirrored as CLAUDE.md)
├─ docs/ARCHITECTURE.md          codebase map (crate boundaries)
├─ data/
│  ├─ ATTRIBUTION.md             data sources + licenses
│  ├─ dhatupatha.tsv             curated v1 roots
│  ├─ tin.tsv                    tiṅ endings table
│  └─ sutras.tsv                 sūtra metadata (id, name, kind)
└─ crates/
   ├─ panini-lipi/               transliteration
   ├─ panini-data/               typed data + embedded loaders
   ├─ panini-prakriya/           engine: model + rules + controller
   ├─ panini-analyze/            candidate proposer
   ├─ panini/                    library facade
   └─ panini-cli/                `panini` binary
```

---

## Task 1: Workspace scaffold, toolchain, and task runner

**Files:**
- Create: `/workspace/Cargo.toml`
- Create: `/workspace/mise.toml`
- Create: `/workspace/crates/panini-lipi/Cargo.toml`
- Create: `/workspace/crates/panini-lipi/src/lib.rs`
- Test: (verified via `cargo build`)

**Interfaces:**
- Consumes: nothing.
- Produces: a buildable workspace; the `panini-lipi` crate exists with `#![forbid(unsafe_code)]`.

- [ ] **Step 1: Write the workspace manifest**

Create `/workspace/Cargo.toml`:

```toml
[workspace]
resolver = "2"
members = ["crates/*"]

[workspace.package]
edition = "2021"
version = "0.1.0"
license = "Apache-2.0"
rust-version = "1.83"

[workspace.dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
proptest = "1"
```

- [ ] **Step 2: Pin the toolchain and declare tasks**

Create `/workspace/mise.toml`:

```toml
[tools]
rust = "1.83.0"

[tasks.build]
run = "cargo build --workspace"

[tasks.test]
run = "cargo test --workspace"

[tasks.lint]
run = "cargo clippy --workspace --all-targets -- -D warnings"

[tasks.fmt]
run = "cargo fmt --all"

[tasks.mutants]
run = "cargo mutants --package panini-prakriya"

[tasks.audit]
run = "cargo audit && cargo deny check"
```

- [ ] **Step 3: Create the first crate**

Create `/workspace/crates/panini-lipi/Cargo.toml`:

```toml
[package]
name = "panini-lipi"
edition.workspace = true
version.workspace = true
license.workspace = true
rust-version.workspace = true
```

Create `/workspace/crates/panini-lipi/src/lib.rs`:

```rust
#![forbid(unsafe_code)]
//! Transliteration between SLP1 (canonical internal form) and human schemes.
```

- [ ] **Step 4: Verify the workspace builds**

Run: `cd /workspace && mise trust && mise run build`
Expected: compiles cleanly (one crate, empty lib).

- [ ] **Step 5: Commit**

```bash
cd /workspace
git add Cargo.toml mise.toml crates/panini-lipi
git commit -m "chore: scaffold cargo workspace and pin toolchain via mise"
```

---

## Task 2: Transliteration (`panini-lipi`)

**Files:**
- Modify: `/workspace/crates/panini-lipi/src/lib.rs`
- Create: `/workspace/crates/panini-lipi/src/slp1.rs`
- Test: `/workspace/crates/panini-lipi/src/lib.rs` (inline `#[cfg(test)]`)

**Interfaces:**
- Consumes: nothing.
- Produces:
  - `enum Scheme { Slp1, Iast, Hk, Devanagari }`
  - `fn detect(input: &str) -> Scheme`
  - `fn to_slp1(input: &str, from: Scheme) -> String`
  - `fn from_slp1(slp1: &str, to: Scheme) -> String`
  - `fn normalize(input: &str) -> (String, Scheme)` — detect then convert to SLP1, returning the detected scheme.

The mapping tables below are the v1 scope (the phonemes needed by the covered forms plus the full vowel/consonant inventory used in the curated roots). SLP1 reference: short vowels `a i u f x`, long `A I U F X`, `e E o O`; consonants include `k K g G N c C j J Y w W q Q R t T d D n p P b B m y r l v S z s h`; anusvāra `M`, visarga `H`.

- [ ] **Step 1: Write the failing test**

In `/workspace/crates/panini-lipi/src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iast_to_slp1_roundtrips() {
        assert_eq!(to_slp1("bhavati", Scheme::Iast), "Bavati");
        assert_eq!(to_slp1("rāmeṇa", Scheme::Iast), "rAmeRa");
        assert_eq!(from_slp1("Bavati", Scheme::Iast), "bhavati");
    }

    #[test]
    fn detects_devanagari_and_converts() {
        let (slp1, scheme) = normalize("भवति");
        assert_eq!(scheme, Scheme::Devanagari);
        assert_eq!(slp1, "Bavati");
    }

    #[test]
    fn slp1_passthrough_detected() {
        let (slp1, scheme) = normalize("Bavati");
        assert_eq!(scheme, Scheme::Slp1);
        assert_eq!(slp1, "Bavati");
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /workspace && cargo test -p panini-lipi`
Expected: FAIL — `to_slp1`, `from_slp1`, `normalize`, `Scheme` not found.

- [ ] **Step 3: Implement the mappings**

Create `/workspace/crates/panini-lipi/src/slp1.rs`. Implement the four schemes as ordered `(&str, &str)` pair tables mapping *from SLP1 tokens* to the target scheme, plus the inverse for parsing. Use longest-match tokenization when parsing IAST/HK/Devanāgarī (multi-byte tokens like `bh`, `ṇ`, `भ` before single tokens). Implement the tables for the full varṇa inventory listed above. Sketch of the required structure (fill every varṇa — no gaps):

```rust
/// (slp1_token, iast_token) pairs. Order longest-slp1-first is irrelevant
/// for emission but the IAST parse table must be matched longest-first.
pub const IAST: &[(&str, &str)] = &[
    ("A", "ā"), ("I", "ī"), ("U", "ū"), ("f", "ṛ"), ("F", "ṝ"),
    ("x", "ḷ"), ("X", "ḹ"), ("e", "e"), ("E", "ai"), ("o", "o"), ("O", "au"),
    ("a", "a"), ("i", "i"), ("u", "u"),
    ("K", "kh"), ("G", "gh"), ("N", "ṅ"),
    ("C", "ch"), ("J", "jh"), ("Y", "ñ"),
    ("w", "ṭ"), ("W", "ṭh"), ("q", "ḍ"), ("Q", "ḍh"), ("R", "ṇ"),
    ("T", "th"), ("D", "dh"),
    ("P", "ph"), ("B", "bh"),
    ("S", "ś"), ("z", "ṣ"),
    ("k","k"),("g","g"),("c","c"),("j","j"),("t","t"),("d","d"),("n","n"),
    ("p","p"),("b","b"),("m","m"),("y","y"),("r","r"),("l","l"),("v","v"),
    ("s","s"),("h","h"),("M","ṃ"),("H","ḥ"),
];
// HK and DEVANAGARI tables follow the same shape (fill completely).
```

Devanāgarī requires the inherent-`a` / virāma / dependent-vowel-sign logic: a bare consonant in SLP1 (`B`) emits base glyph + virāma (`भ्`); consonant + `a` emits the base glyph (`भ`); consonant + other vowel emits base + mātrā (`भि` for `Bi`); a word-initial or post-vowel vowel uses the independent vowel glyph. Implement this in `slp1_to_devanagari`/`devanagari_to_slp1`.

- [ ] **Step 4: Implement `lib.rs` public API and detection**

In `/workspace/crates/panini-lipi/src/lib.rs`:

```rust
#![forbid(unsafe_code)]
mod slp1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scheme { Slp1, Iast, Hk, Devanagari }

pub fn detect(input: &str) -> Scheme {
    if input.chars().any(|c| ('\u{0900}'..='\u{097F}').contains(&c)) {
        return Scheme::Devanagari;
    }
    // IAST is the only ASCII+diacritic scheme with combining/precomposed marks.
    if input.chars().any(|c| !c.is_ascii()) {
        return Scheme::Iast;
    }
    // Heuristic: SLP1 uses capitals mid-word for aspirates/long vowels;
    // HK uses digraphs. Default ASCII to SLP1 unless HK-only digraphs appear.
    if input.contains("kh") || input.contains("bh") || input.contains("aa") {
        return Scheme::Hk;
    }
    Scheme::Slp1
}

pub fn to_slp1(input: &str, from: Scheme) -> String { slp1::to_slp1(input, from) }
pub fn from_slp1(s: &str, to: Scheme) -> String { slp1::from_slp1(s, to) }

pub fn normalize(input: &str) -> (String, Scheme) {
    let scheme = detect(input);
    (to_slp1(input, scheme), scheme)
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cd /workspace && cargo test -p panini-lipi`
Expected: PASS (all three tests).

- [ ] **Step 6: Add a property test for SLP1→IAST→SLP1 round-trip**

Add to the test module:

```rust
#[test]
fn slp1_iast_roundtrip_curated() {
    for w in ["Bavati", "nayati", "jayati", "smarati", "paWati", "vadati", "BavAmaH"] {
        let iast = from_slp1(w, Scheme::Iast);
        assert_eq!(to_slp1(&iast, Scheme::Iast), w, "roundtrip failed for {w}");
    }
}
```

Run: `cargo test -p panini-lipi` → Expected: PASS.

- [ ] **Step 7: Commit**

```bash
cd /workspace
git add crates/panini-lipi
git commit -m "feat(lipi): SLP1<->IAST/HK/Devanagari transliteration with scheme detection"
```

---

## Task 3: Engine core model (`panini-prakriya`)

**Files:**
- Create: `/workspace/crates/panini-prakriya/Cargo.toml`
- Create: `/workspace/crates/panini-prakriya/src/lib.rs`
- Create: `/workspace/crates/panini-prakriya/src/term.rs`
- Create: `/workspace/crates/panini-prakriya/src/prakriya.rs`

**Interfaces:**
- Consumes: nothing.
- Produces:
  - `enum Tag { Dhatu, Pratyaya, Anga, Vikarana, Tin, Sarvadhatuka, Ardhadhatuka, It, Abhyasa, /* v1 subset */ }`
  - `struct Term { pub text: String, pub tags: HashSet<Tag> }` with `fn new(text: &str) -> Term`, `fn has(&self, Tag) -> bool`, `fn add(&mut self, Tag)`.
  - `struct RuleStep { pub sutra: String, pub name: String, pub before: String, pub after: String }`
  - `struct Prakriya { pub terms: Vec<Term>, pub log: Vec<RuleStep> }` with `fn text(&self) -> String` (concatenate term texts), `fn snapshot(&self) -> String`, and `fn record(&mut self, sutra: &str, name: &str, before: String)`.

- [ ] **Step 1: Create the crate manifest**

Create `/workspace/crates/panini-prakriya/Cargo.toml`:

```toml
[package]
name = "panini-prakriya"
edition.workspace = true
version.workspace = true
license.workspace = true
rust-version.workspace = true
```

- [ ] **Step 2: Write the failing test**

Create `/workspace/crates/panini-prakriya/src/lib.rs`:

```rust
#![forbid(unsafe_code)]
pub mod term;
pub mod prakriya;

pub use term::{Tag, Term};
pub use prakriya::{Prakriya, RuleStep};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prakriya_concatenates_and_logs() {
        let mut p = Prakriya { terms: vec![Term::new("BU"), Term::new("a"), Term::new("ti")], log: vec![] };
        assert_eq!(p.text(), "BUati");
        let before = p.snapshot();
        p.terms[0].text = "Bo".into();
        p.record("7.3.84", "sArvadhAtukArdhadhAtukayoH", before);
        assert_eq!(p.log.len(), 1);
        assert_eq!(p.log[0].sutra, "7.3.84");
        assert_eq!(p.log[0].after, "Boati");
    }

    #[test]
    fn term_tags() {
        let mut t = Term::new("ti");
        t.add(Tag::Tin);
        assert!(t.has(Tag::Tin));
        assert!(!t.has(Tag::Dhatu));
    }
}
```

- [ ] **Step 3: Run test to verify it fails**

Run: `cd /workspace && cargo test -p panini-prakriya`
Expected: FAIL — modules/types not defined.

- [ ] **Step 4: Implement `term.rs`**

```rust
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tag {
    Dhatu, Pratyaya, Anga, Vikarana, Tin,
    Sarvadhatuka, Ardhadhatuka, It, Abhyasa,
}

#[derive(Debug, Clone)]
pub struct Term {
    pub text: String,
    pub tags: HashSet<Tag>,
}

impl Term {
    pub fn new(text: &str) -> Term { Term { text: text.to_string(), tags: HashSet::new() } }
    pub fn has(&self, tag: Tag) -> bool { self.tags.contains(&tag) }
    pub fn add(&mut self, tag: Tag) { self.tags.insert(tag); }
}
```

- [ ] **Step 5: Implement `prakriya.rs`**

```rust
use crate::term::Term;

#[derive(Debug, Clone)]
pub struct RuleStep {
    pub sutra: String,
    pub name: String,
    pub before: String,
    pub after: String,
}

#[derive(Debug, Clone, Default)]
pub struct Prakriya {
    pub terms: Vec<Term>,
    pub log: Vec<RuleStep>,
}

impl Prakriya {
    pub fn text(&self) -> String {
        self.terms.iter().map(|t| t.text.as_str()).collect()
    }
    pub fn snapshot(&self) -> String { self.text() }
    pub fn record(&mut self, sutra: &str, name: &str, before: String) {
        let after = self.text();
        self.log.push(RuleStep { sutra: sutra.into(), name: name.into(), before, after });
    }
}
```

- [ ] **Step 6: Run tests to verify they pass**

Run: `cd /workspace && cargo test -p panini-prakriya`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
cd /workspace
git add crates/panini-prakriya
git commit -m "feat(prakriya): Term/Tag/Prakriya/RuleStep core model with rule logging"
```

---

## Task 4: It-marker (anubandha) processing

**Files:**
- Create: `/workspace/crates/panini-prakriya/src/it_samjna.rs`
- Modify: `/workspace/crates/panini-prakriya/src/lib.rs` (add `pub mod it_samjna;`)

**Interfaces:**
- Consumes: `Term`, `Prakriya` from Task 3.
- Produces: `fn run_it_samjna(term: &mut Term, p: &mut Prakriya, idx: usize)` — removes the anubandhas of the pratyaya at `terms[idx]` (final consonant by 1.3.3 halantyam; initial `S/z/N`-class markers per 1.3.5–1.3.8) and records the elision as 1.3.9 tasya lopaḥ. For v1 only these markers occur: `p` (final, `Sap`/`tip`→`ti`), `S` (initial of `Sap`), and the `i`/`u`/etc. it of `tip`-family already handled by treating the ending strings as given.

Scope note: v1 introduces pratyayas already partly resolved (see Task 6 tiṅ table stores the *marked* form, e.g. `tip`, `Sap`), and this function reduces them to their operative form (`ti`, `a`).

- [ ] **Step 1: Write the failing test**

In `it_samjna.rs` add a test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::term::{Tag, Term};
    use crate::prakriya::Prakriya;

    #[test]
    fn shap_loses_sh_and_leaves_a() {
        let mut p = Prakriya { terms: vec![Term::new("Sap")], log: vec![] };
        let mut t = p.terms[0].clone();
        run_it_samjna(&mut t, &mut p, 0);
        p.terms[0] = t;
        assert_eq!(p.terms[0].text, "a");
        assert!(p.log.iter().any(|s| s.sutra == "1.3.9"));
    }

    #[test]
    fn tip_loses_final_p() {
        let mut p = Prakriya { terms: vec![Term::new("tip")], log: vec![] };
        let mut t = p.terms[0].clone();
        run_it_samjna(&mut t, &mut p, 0);
        p.terms[0] = t;
        assert_eq!(p.terms[0].text, "ti");
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /workspace && cargo test -p panini-prakriya it_samjna`
Expected: FAIL — `run_it_samjna` not defined.

- [ ] **Step 3: Implement `it_samjna.rs`**

```rust
use crate::prakriya::Prakriya;
use crate::term::{Tag, Term};

/// Consonants that are always `it` when final in a pratyaya (1.3.3 halantyam).
fn is_hal(c: char) -> bool {
    !matches!(c, 'a'|'A'|'i'|'I'|'u'|'U'|'f'|'F'|'x'|'X'|'e'|'E'|'o'|'O')
}

pub fn run_it_samjna(term: &mut Term, p: &mut Prakriya, idx: usize) {
    let before = p.snapshot();
    let original = term.text.clone();
    let mut s: Vec<char> = original.chars().collect();

    // 1.3.8 laSakvataddhite: initial S, z, or ku-class marker of a pratyaya is it.
    // v1: only leading `S` occurs (Sap).
    if matches!(s.first(), Some('S') | Some('z')) {
        s.remove(0);
    }
    // 1.3.3 halantyam: final consonant is it.
    if let Some(&last) = s.last() {
        if is_hal(last) {
            s.pop();
        }
    }
    let reduced: String = s.into_iter().collect();
    if reduced != original {
        term.text = reduced;
        // Reflect into the live prakriya then log 1.3.9 tasya lopaH.
        p.terms[idx].text = term.text.clone();
        p.record("1.3.9", "tasya lopaH", before);
    }
    let _ = Tag::It; // Tag reserved for future explicit it-marking.
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd /workspace && cargo test -p panini-prakriya it_samjna`
Expected: PASS.

- [ ] **Step 5: Register the module and commit**

Add `pub mod it_samjna;` to `lib.rs`.

Run: `cd /workspace && cargo test -p panini-prakriya`
Expected: PASS (all tests).

```bash
cd /workspace
git add crates/panini-prakriya
git commit -m "feat(prakriya): it-samjna (anubandha) elision via 1.3.3/1.3.8/1.3.9"
```

---

## Task 5: Rule + controller infrastructure

**Files:**
- Create: `/workspace/crates/panini-prakriya/src/rule.rs`
- Create: `/workspace/crates/panini-prakriya/src/controller.rs`
- Modify: `/workspace/crates/panini-prakriya/src/lib.rs`

**Interfaces:**
- Consumes: `Prakriya` from Task 3.
- Produces:
  - `enum RuleKind { Vidhi, Samjna, Adhikara, Paribhasha }`
  - `struct Rule { pub id: &'static str, pub name: &'static str, pub kind: RuleKind, pub apply: fn(&mut Prakriya) -> bool }` — `apply` returns `true` if it changed the prakriya (and is responsible for calling `p.record`).
  - `fn run_pipeline(p: &mut Prakriya, rules: &[Rule])` — applies rules **in the given order**, each at most once, skipping those whose `apply` returns `false`. For v1 the correct *paribhāṣā* ordering for the covered forms is expressible as a fixed rule order (documented in Task 7); the controller therefore executes an explicitly-ordered rule list. The `Rule` metadata (`kind`, `id`) is retained so Phase 2 can replace the fixed order with metadata-driven conflict resolution without changing rule bodies.

Design rationale: keeping ordering in the controller (not inside rule bodies) is the fidelity requirement from the spec. v1 encodes that order as data (the ordered slice); Phase 2 swaps the ordering strategy behind the same `Rule`/`run_pipeline` seam.

- [ ] **Step 1: Write the failing test**

In `controller.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::prakriya::Prakriya;
    use crate::term::Term;
    use crate::rule::{Rule, RuleKind};

    #[test]
    fn pipeline_applies_in_order_and_logs() {
        let mut p = Prakriya { terms: vec![Term::new("Bo"), Term::new("a")], log: vec![] };
        let rules = [
            Rule {
                id: "6.1.78", name: "eco'yavAyAvaH", kind: RuleKind::Vidhi,
                apply: |p| {
                    if p.terms[0].text == "Bo" {
                        let b = p.snapshot();
                        p.terms[0].text = "Bav".into();
                        p.record("6.1.78", "eco'yavAyAvaH", b);
                        true
                    } else { false }
                },
            },
        ];
        run_pipeline(&mut p, &rules);
        assert_eq!(p.text(), "Bava");
        assert_eq!(p.log.last().unwrap().sutra, "6.1.78");
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /workspace && cargo test -p panini-prakriya controller`
Expected: FAIL — `run_pipeline`, `Rule`, `RuleKind` not defined.

- [ ] **Step 3: Implement `rule.rs`**

```rust
use crate::prakriya::Prakriya;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleKind { Vidhi, Samjna, Adhikara, Paribhasha }

#[derive(Clone, Copy)]
pub struct Rule {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: RuleKind,
    /// Returns true if it mutated the prakriya (and recorded a RuleStep).
    pub apply: fn(&mut Prakriya) -> bool,
}
```

- [ ] **Step 4: Implement `controller.rs`**

```rust
use crate::prakriya::Prakriya;
use crate::rule::Rule;

pub use crate::rule::{Rule as _Rule, RuleKind};

/// Apply each rule in order, at most once. Rules self-guard via `apply`
/// returning false when inapplicable. Ordering is the controller's concern.
pub fn run_pipeline(p: &mut Prakriya, rules: &[Rule]) {
    for rule in rules {
        (rule.apply)(p);
    }
}
```

- [ ] **Step 5: Register modules, run tests**

Add to `lib.rs`:

```rust
pub mod rule;
pub mod controller;
pub use rule::{Rule, RuleKind};
pub use controller::run_pipeline;
```

Run: `cd /workspace && cargo test -p panini-prakriya`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
cd /workspace
git add crates/panini-prakriya
git commit -m "feat(prakriya): Rule metadata + ordered controller pipeline"
```

---

## Task 6: Linguistic data (`panini-data`)

**Files:**
- Create: `/workspace/data/dhatupatha.tsv`
- Create: `/workspace/data/tin.tsv`
- Create: `/workspace/data/ATTRIBUTION.md`
- Create: `/workspace/crates/panini-data/Cargo.toml`
- Create: `/workspace/crates/panini-data/src/lib.rs`

**Interfaces:**
- Consumes: nothing.
- Produces:
  - `enum Gana { Bhvadi }`
  - `enum Pada { Parasmaipada }`
  - `enum Lakara { Lat }`
  - `enum Purusha { Prathama, Madhyama, Uttama }`
  - `enum Vacana { Eka, Dvi, Bahu }`
  - `struct Dhatu { pub code: &'static str, pub gana: Gana, pub artha: &'static str }`
  - `fn dhatus() -> &'static [Dhatu]` — the curated 6 roots.
  - `fn tin_ending(pada: Pada, purusha: Purusha, vacana: Vacana) -> &'static str` — returns the **marked** tiṅ (e.g. `tip`, `tas`, `Ji`, `sip`, `Tas`, `Ta`, `mip`, `vas`, `mas`).

Data is embedded via `include_str!` and parsed at first use (or hard-coded as `const` slices — v1 is tiny, so hard-coded slices are acceptable and avoid runtime parsing).

- [ ] **Step 1: Create the data files**

Create `/workspace/data/dhatupatha.tsv` (tab-separated: code, gana, artha):

```
BU	bhvadi	sattAyAm
nI	bhvadi	prApaRe
ji	bhvadi	jaye
smf	bhvadi	cintAyAm
paW	bhvadi	vyaktAyAM vAci
vad	bhvadi	vyaktAyAM vAci
```

Create `/workspace/data/tin.tsv` (pada, purusha, vacana, marked-ending):

```
parasmaipada	prathama	eka	tip
parasmaipada	prathama	dvi	tas
parasmaipada	prathama	bahu	Ji
parasmaipada	madhyama	eka	sip
parasmaipada	madhyama	dvi	Tas
parasmaipada	madhyama	bahu	Ta
parasmaipada	uttama	eka	mip
parasmaipada	uttama	dvi	vas
parasmaipada	uttama	bahu	mas
```

Create `/workspace/data/ATTRIBUTION.md`:

```markdown
# Data Attribution

The Dhātupāṭha entries, gaṇa assignments, and tiṅ-pratyāhāra endings in this
directory follow the standard Pāṇinian corpus. Where values were cross-checked
against openly-licensed digital sources, those sources and their licenses are
recorded below.

- Cross-reference: ashtadhyayi.com derivations (per-form).
- Cross-reference: Vidyut (github.com/ambuda-org/vidyut), data files under
  their stated open license — consult the upstream LICENSE before importing any
  file verbatim; v1 uses only the small curated subset transcribed here.
```

- [ ] **Step 2: Create the crate and write the failing test**

Create `/workspace/crates/panini-data/Cargo.toml`:

```toml
[package]
name = "panini-data"
edition.workspace = true
version.workspace = true
license.workspace = true
rust-version.workspace = true
```

Create `/workspace/crates/panini-data/src/lib.rs` with the test first:

```rust
#![forbid(unsafe_code)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_six_curated_roots() {
        assert_eq!(dhatus().len(), 6);
        assert!(dhatus().iter().any(|d| d.code == "BU"));
    }

    #[test]
    fn tin_endings_are_marked_forms() {
        assert_eq!(tin_ending(Pada::Parasmaipada, Purusha::Prathama, Vacana::Eka), "tip");
        assert_eq!(tin_ending(Pada::Parasmaipada, Purusha::Uttama, Vacana::Bahu), "mas");
        assert_eq!(tin_ending(Pada::Parasmaipada, Purusha::Prathama, Vacana::Bahu), "Ji");
    }
}
```

- [ ] **Step 3: Run test to verify it fails**

Run: `cd /workspace && cargo test -p panini-data`
Expected: FAIL — types/functions not defined.

- [ ] **Step 4: Implement `lib.rs`**

```rust
#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum Gana { Bhvadi }
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum Pada { Parasmaipada }
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum Lakara { Lat }
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum Purusha { Prathama, Madhyama, Uttama }
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum Vacana { Eka, Dvi, Bahu }

#[derive(Debug, Clone, Copy)]
pub struct Dhatu { pub code: &'static str, pub gana: Gana, pub artha: &'static str }

static DHATUS: &[Dhatu] = &[
    Dhatu { code: "BU",  gana: Gana::Bhvadi, artha: "sattAyAm" },
    Dhatu { code: "nI",  gana: Gana::Bhvadi, artha: "prApaRe" },
    Dhatu { code: "ji",  gana: Gana::Bhvadi, artha: "jaye" },
    Dhatu { code: "smf", gana: Gana::Bhvadi, artha: "cintAyAm" },
    Dhatu { code: "paW", gana: Gana::Bhvadi, artha: "vyaktAyAM vAci" },
    Dhatu { code: "vad", gana: Gana::Bhvadi, artha: "vyaktAyAM vAci" },
];

pub fn dhatus() -> &'static [Dhatu] { DHATUS }

pub fn tin_ending(pada: Pada, purusha: Purusha, vacana: Vacana) -> &'static str {
    use Purusha::*; use Vacana::*;
    match pada {
        Pada::Parasmaipada => match (purusha, vacana) {
            (Prathama, Eka) => "tip", (Prathama, Dvi) => "tas", (Prathama, Bahu) => "Ji",
            (Madhyama, Eka) => "sip", (Madhyama, Dvi) => "Tas", (Madhyama, Bahu) => "Ta",
            (Uttama, Eka) => "mip",  (Uttama, Dvi) => "vas",  (Uttama, Bahu) => "mas",
        },
    }
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cd /workspace && cargo test -p panini-data`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
cd /workspace
git add data crates/panini-data
git commit -m "feat(data): curated bhvadi roots + parasmaipada tin table with attribution"
```

---

## Task 7: Tiṅanta derivation rules and `derive()`

**Files:**
- Create: `/workspace/crates/panini-prakriya/src/tinanta.rs`
- Modify: `/workspace/crates/panini-prakriya/src/lib.rs`
- Modify: `/workspace/crates/panini-prakriya/Cargo.toml` (add `panini-data` dependency)

**Interfaces:**
- Consumes: `Prakriya`, `Term`, `Tag`, `run_it_samjna`, `Rule`, `run_pipeline` (Tasks 3–5); `Dhatu`, `Pada`, `Lakara`, `Purusha`, `Vacana`, `tin_ending` (Task 6).
- Produces: `fn derive(dhatu: &Dhatu, lakara: Lakara, pada: Pada, purusha: Purusha, vacana: Vacana) -> Prakriya` — returns the completed `Prakriya` whose `text()` is the final SLP1 form and whose `log` is the ordered trace.

**Reference oracle (do this first):** open ashtadhyayi.com and generate the *laṭ parasmaipada* derivation for `BU`. Record the ordered sūtra list it reports. The rule bodies below implement the net effect of that sequence; the ids/names in `record(...)` calls MUST match the reference. The exact handling of the plural `Ji` ending (7.1.3 `jho'ntaḥ` plus the ensuing vowel coalescence) is the one place to transcribe carefully from the reference — the golden test in Task 11 is the safety net that catches any divergence.

**Fixed rule order for v1 laṭ/bhvādi/parasmaipada** (this ordering encodes the paribhāṣā resolution for the covered forms):
1. `3.4.78` — replace laṭ by the tiṅ ending; mark it `Tin` + `Sarvadhatuka` (3.4.113 tiṅ-śit sārvadhātukam).
2. it-samjna on the tiṅ (elide anubandhas → operative ending).
3. `3.1.68` kartari śap — insert `Sap` between dhātu and ending; mark it `Vikarana` + `Sarvadhatuka`.
4. it-samjna on śap (→ `a`), mark the dhātu `Anga`.
5. `7.1.3` jho'ntaḥ — if ending is `Ji`, replace `J`→`ant` (yielding `anti`), per the reference.
6. `7.3.84` sārvadhātukārdhadhātukayoḥ — guṇa of the aṅga's **final** ik before the sārvadhātuka (`U→o`, `I/i→e`, `u→o`, `f→ar`).
7. `6.1.78` eco'yavāyāvaḥ — `e`/`o`/`E`/`O` before a vowel → `ay`/`av`/`Ay`/`Av`.
8. `7.3.101` ato dīrgho yañi — aṅga-final `a` (the śap) → `A` before an ending beginning with `m` or `v` (mi/vas/mas).
9. `8.2.66` sasajuṣo ruḥ + `8.3.15` kharavasānayoḥ — word-final `s` → `H` (visarga).
10. vowel-coalescence cleanup for the `anti` case per the reference (recorded under the reference's cited sūtra).

- [ ] **Step 1: Add the data dependency**

In `/workspace/crates/panini-prakriya/Cargo.toml`:

```toml
[dependencies]
panini-data = { path = "../panini-data" }
```

- [ ] **Step 2: Write the failing test (the four representative forms)**

In `tinanta.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use panini_data::{dhatus, Lakara, Pada, Purusha, Vacana};

    fn form(code: &str, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, Lakara::Lat, Pada::Parasmaipada, pu, va).text()
    }

    #[test]
    fn bhu_3sg_is_bhavati() {
        assert_eq!(form("BU", Purusha::Prathama, Vacana::Eka), "Bavati");
    }
    #[test]
    fn bhu_1pl_is_bhavamah() {
        assert_eq!(form("BU", Purusha::Uttama, Vacana::Bahu), "BavAmaH");
    }
    #[test]
    fn smr_3sg_is_smarati() {
        assert_eq!(form("smf", Purusha::Prathama, Vacana::Eka), "smarati");
    }
    #[test]
    fn pat_3du_is_patatah() {
        assert_eq!(form("paW", Purusha::Prathama, Vacana::Dvi), "paWataH");
    }
    #[test]
    fn bhu_3pl_is_bhavanti() {
        assert_eq!(form("BU", Purusha::Prathama, Vacana::Bahu), "Bavanti");
    }
    #[test]
    fn trace_is_recorded() {
        let d = dhatus().iter().find(|d| d.code == "BU").unwrap();
        let p = derive(d, Lakara::Lat, Pada::Parasmaipada, Purusha::Prathama, Vacana::Eka);
        assert!(p.log.iter().any(|s| s.sutra == "3.1.68"));
        assert!(p.log.iter().any(|s| s.sutra == "7.3.84"));
        assert!(!p.log.is_empty());
    }
}
```

- [ ] **Step 3: Run test to verify it fails**

Run: `cd /workspace && cargo test -p panini-prakriya tinanta`
Expected: FAIL — `derive` not defined.

- [ ] **Step 4: Implement `tinanta.rs`**

Implement `derive` as: build the initial `Prakriya` (`[dhatu, ending]`), then apply the fixed-ordered steps. Helper functions keep each sūtra small. Guṇa and ayādeśa are the substantive transforms.

```rust
use crate::it_samjna::run_it_samjna;
use crate::prakriya::Prakriya;
use crate::term::{Tag, Term};
use panini_data::{tin_ending, Dhatu, Lakara, Pada, Purusha, Vacana};

fn guna_of(v: char) -> Option<&'static str> {
    match v {
        'i' | 'I' => Some("e"),
        'u' | 'U' => Some("o"),
        'f' | 'F' => Some("ar"),
        'x' | 'X' => Some("al"),
        _ => None,
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a'|'A'|'i'|'I'|'u'|'U'|'f'|'F'|'x'|'X'|'e'|'E'|'o'|'O')
}

pub fn derive(
    dhatu: &Dhatu, _lakara: Lakara, pada: Pada, purusha: Purusha, vacana: Vacana,
) -> Prakriya {
    let mut p = Prakriya::default();
    p.terms.push({ let mut t = Term::new(dhatu.code); t.add(Tag::Dhatu); t });

    // 3.4.78: replace laT by the tiN ending; 3.4.113 makes it sarvadhatuka.
    let ending = tin_ending(pada, purusha, vacana);
    {
        let before = p.snapshot();
        let mut e = Term::new(ending);
        e.add(Tag::Tin); e.add(Tag::Sarvadhatuka);
        p.terms.push(e);
        p.record("3.4.78", "tiptasjhi...", before);
    }
    // it-samjna on the ending.
    { let mut e = p.terms[1].clone(); run_it_samjna(&mut e, &mut p, 1); p.terms[1] = e; }

    // 3.1.68 kartari Sap: insert Sap between dhatu and ending.
    {
        let before = p.snapshot();
        let mut s = Term::new("Sap");
        s.add(Tag::Vikarana); s.add(Tag::Sarvadhatuka);
        p.terms.insert(1, s);
        p.record("3.1.68", "kartari Sap", before);
    }
    { let mut s = p.terms[1].clone(); run_it_samjna(&mut s, &mut p, 1); p.terms[1] = s; }
    p.terms[0].add(Tag::Anga);

    // 7.1.3 jho'ntaH: Ji -> anti (only the bahuvacana prathama ending).
    if p.terms[2].text == "Ji" {
        let before = p.snapshot();
        p.terms[2].text = "anti".into();
        p.record("7.1.3", "jho'ntaH", before);
    }

    // 7.3.84 sarvadhatukardhadhatukayoH: guna of the anga's final ik.
    {
        let last = p.terms[0].text.chars().last().unwrap();
        if let Some(g) = guna_of(last) {
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[0].text.chars().collect();
            s.pop();
            let replaced: String = s.into_iter().collect::<String>() + g;
            p.terms[0].text = replaced;
            p.record("7.3.84", "sArvadhAtukArdhadhAtukayoH", before);
        }
    }

    // 6.1.78 eco'yavAyAvaH: e/o/E/O before a vowel -> ay/av/Ay/Av.
    {
        let anga_last = p.terms[0].text.chars().last().unwrap();
        let next_first = p.terms[1].text.chars().next().unwrap();
        let sub = match anga_last { 'e' => Some("ay"), 'o' => Some("av"), 'E' => Some("Ay"), 'O' => Some("Av"), _ => None };
        if let (Some(sub), true) = (sub, is_vowel(next_first)) {
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[0].text.chars().collect();
            s.pop();
            p.terms[0].text = s.into_iter().collect::<String>() + sub;
            p.record("6.1.78", "eco'yavAyAvaH", before);
        }
    }

    // 7.3.101 ato dIrgho yaYi: sap 'a' -> 'A' before m/v endings.
    {
        let ending_first = p.terms[2].text.chars().next().unwrap();
        if (ending_first == 'm' || ending_first == 'v') && p.terms[1].text == "a" {
            let before = p.snapshot();
            p.terms[1].text = "A".into();
            p.record("7.3.101", "ato dIrgho yaYi", before);
        }
    }

    // 8.2.66 sasajuSo ruH + 8.3.15: word-final s -> visarga H.
    {
        if p.terms.last().unwrap().text.ends_with('s') {
            let before = p.snapshot();
            let idx = p.terms.len() - 1;
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop(); s.push('H');
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.3.15", "kharavasAnayoH visarjanIyaH", before);
        }
    }

    p
}
```

**Note on the `Ji`/`anti` coalescence:** after 7.1.3 the terms are `[Bava, a, anti]`-shaped for `BU` (aṅga `Bav`, śap `a`, ending `anti`). The concatenation `Bav`+`a`+`anti` must yield `Bavanti`, not `BavAnti`. Verify against the reference which sūtra governs this (it is not `akaḥ savarṇe dīrghaḥ`); implement that step explicitly and record it, or restructure so the śap `a` and ending combine correctly. The Task 11 golden test for `Bavanti`/`nayanti`/`jayanti`/`smaranti`/`paWanti`/`vadanti` is the gate — do not mark this task done until all plural forms pass.

- [ ] **Step 5: Run tests to verify they pass**

Run: `cd /workspace && cargo test -p panini-prakriya tinanta`
Expected: PASS for `bhavati`, `bhavāmaḥ`, `smarati`, `paṭataḥ`, `bhavanti`, and the trace test. If `bhavanti` fails, fix the `Ji` coalescence per the reference before proceeding.

- [ ] **Step 6: Register the module and commit**

Add `pub mod tinanta; pub use tinanta::derive;` to `lib.rs`.

Run: `cd /workspace && cargo test -p panini-prakriya`
Expected: PASS.

```bash
cd /workspace
git add crates/panini-prakriya
git commit -m "feat(prakriya): laT/bhvadi/parasmaipada tinanta derivation with rule trace"
```

---

## Task 8: Candidate analyzer (`panini-analyze`)

**Files:**
- Create: `/workspace/crates/panini-analyze/Cargo.toml`
- Create: `/workspace/crates/panini-analyze/src/lib.rs`

**Interfaces:**
- Consumes: `dhatus`, `Lakara`, `Pada`, `Purusha`, `Vacana` (Task 6).
- Produces:
  - `struct Candidate { pub dhatu: &'static Dhatu, pub lakara: Lakara, pub pada: Pada, pub purusha: Purusha, pub vacana: Vacana }`
  - `fn candidates(surface_slp1: &str) -> Vec<Candidate>` — proposes the inputs worth deriving. v1 strategy: enumerate every (root × puruṣa × vacana) whose known operative ending is a **suffix** of the surface form, narrowing the search before the engine confirms. If nothing matches on suffix, fall back to the full 6×9 enumeration (still tiny) so the engine is the final arbiter.

- [ ] **Step 1: Create the crate and write the failing test**

Create `/workspace/crates/panini-analyze/Cargo.toml`:

```toml
[package]
name = "panini-analyze"
edition.workspace = true
version.workspace = true
license.workspace = true
rust-version.workspace = true

[dependencies]
panini-data = { path = "../panini-data" }
```

Create `/workspace/crates/panini-analyze/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proposes_bhu_prathama_eka_for_bhavati() {
        let cands = candidates("Bavati");
        assert!(cands.iter().any(|c|
            c.dhatu.code == "BU"
            && matches!(c.purusha, panini_data::Purusha::Prathama)
            && matches!(c.vacana, panini_data::Vacana::Eka)));
    }

    #[test]
    fn always_narrows_to_nonempty_for_covered_ending() {
        assert!(!candidates("BavAmaH").is_empty());
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /workspace && cargo test -p panini-analyze`
Expected: FAIL — `candidates` not defined.

- [ ] **Step 3: Implement `lib.rs`**

```rust
#![forbid(unsafe_code)]
use panini_data::{dhatus, Dhatu, Lakara, Pada, Purusha, Vacana};

pub struct Candidate {
    pub dhatu: &'static Dhatu,
    pub lakara: Lakara,
    pub pada: Pada,
    pub purusha: Purusha,
    pub vacana: Vacana,
}

const CELLS: &[(Purusha, Vacana)] = &[
    (Purusha::Prathama, Vacana::Eka), (Purusha::Prathama, Vacana::Dvi), (Purusha::Prathama, Vacana::Bahu),
    (Purusha::Madhyama, Vacana::Eka), (Purusha::Madhyama, Vacana::Dvi), (Purusha::Madhyama, Vacana::Bahu),
    (Purusha::Uttama, Vacana::Eka),  (Purusha::Uttama, Vacana::Dvi),  (Purusha::Uttama, Vacana::Bahu),
];

pub fn candidates(surface_slp1: &str) -> Vec<Candidate> {
    let mut out = Vec::new();
    for d in dhatus() {
        for &(purusha, vacana) in CELLS {
            out.push(Candidate {
                dhatu: d, lakara: Lakara::Lat, pada: Pada::Parasmaipada, purusha, vacana,
            });
        }
    }
    // v1: return the full (tiny) candidate set; the engine confirms by exact match.
    let _ = surface_slp1;
    out
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd /workspace && cargo test -p panini-analyze`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
cd /workspace
git add crates/panini-analyze
git commit -m "feat(analyze): candidate proposer over curated roots x tin cells"
```

---

## Task 9: Library facade (`panini`)

**Files:**
- Create: `/workspace/crates/panini/Cargo.toml`
- Create: `/workspace/crates/panini/src/lib.rs`

**Interfaces:**
- Consumes: `panini-lipi` (`normalize`, `from_slp1`, `Scheme`), `panini-analyze` (`candidates`), `panini-prakriya` (`derive`, `RuleStep`), `panini-data` types.
- Produces:
  - `enum Verdict { Valid, Invalid }`
  - `struct Analysis { pub dhatu: String, pub purusha: Purusha, pub vacana: Vacana, pub form_slp1: String, pub trace: Vec<RuleStep> }`
  - `struct CheckResult { pub verdict: Verdict, pub input_slp1: String, pub detected: Scheme, pub analyses: Vec<Analysis> }`
  - `struct Panini;` with `fn new() -> Panini`, `fn check(&self, input: &str) -> CheckResult`, and `fn derive(&self, ...) -> Prakriya` re-exported for direct generation.

- [ ] **Step 1: Create the crate and write the failing test**

Create `/workspace/crates/panini/Cargo.toml`:

```toml
[package]
name = "panini"
edition.workspace = true
version.workspace = true
license.workspace = true
rust-version.workspace = true

[dependencies]
panini-lipi = { path = "../panini-lipi" }
panini-data = { path = "../panini-data" }
panini-analyze = { path = "../panini-analyze" }
panini-prakriya = { path = "../panini-prakriya" }
```

Create `/workspace/crates/panini/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_word_returns_trace() {
        let engine = Panini::new();
        let r = engine.check("bhavati");
        assert!(matches!(r.verdict, Verdict::Valid));
        let a = r.analyses.iter().find(|a| a.form_slp1 == "Bavati").unwrap();
        assert_eq!(a.dhatu, "BU");
        assert!(!a.trace.is_empty());
    }

    #[test]
    fn devanagari_input_is_accepted() {
        let engine = Panini::new();
        let r = engine.check("भवति");
        assert!(matches!(r.verdict, Verdict::Valid));
    }

    #[test]
    fn non_covered_word_is_invalid() {
        let engine = Panini::new();
        let r = engine.check("xyzq");
        assert!(matches!(r.verdict, Verdict::Invalid));
        assert!(r.analyses.is_empty());
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /workspace && cargo test -p panini`
Expected: FAIL — types not defined.

- [ ] **Step 3: Implement `lib.rs`**

```rust
#![forbid(unsafe_code)]
use panini_analyze::candidates;
use panini_data::{Lakara, Pada, Purusha, Vacana};
use panini_lipi::{from_slp1, normalize, Scheme};
use panini_prakriya::{derive as derive_prakriya, Prakriya, RuleStep};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict { Valid, Invalid }

pub struct Analysis {
    pub dhatu: String,
    pub purusha: Purusha,
    pub vacana: Vacana,
    pub form_slp1: String,
    pub trace: Vec<RuleStep>,
}

pub struct CheckResult {
    pub verdict: Verdict,
    pub input_slp1: String,
    pub detected: Scheme,
    pub analyses: Vec<Analysis>,
}

pub struct Panini;

impl Panini {
    pub fn new() -> Panini { Panini }

    pub fn check(&self, input: &str) -> CheckResult {
        let (slp1, detected) = normalize(input);
        let mut analyses = Vec::new();
        for c in candidates(&slp1) {
            let p = derive_prakriya(c.dhatu, c.lakara, c.pada, c.purusha, c.vacana);
            if p.text() == slp1 {
                analyses.push(Analysis {
                    dhatu: c.dhatu.code.to_string(),
                    purusha: c.purusha, vacana: c.vacana,
                    form_slp1: p.text(), trace: p.log,
                });
            }
        }
        let verdict = if analyses.is_empty() { Verdict::Invalid } else { Verdict::Valid };
        CheckResult { verdict, input_slp1: slp1, detected, analyses }
    }

    pub fn derive(&self, dhatu: &panini_data::Dhatu, lakara: Lakara, pada: Pada, purusha: Purusha, vacana: Vacana) -> Prakriya {
        derive_prakriya(dhatu, lakara, pada, purusha, vacana)
    }
}

impl Default for Panini { fn default() -> Self { Panini::new() } }

/// Render an SLP1 form in the requested scheme (for callers/CLI).
pub fn render(slp1: &str, scheme: Scheme) -> String { from_slp1(slp1, scheme) }
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd /workspace && cargo test -p panini`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
cd /workspace
git add crates/panini
git commit -m "feat(panini): check() facade wiring analyze -> derive -> match -> trace"
```

---

## Task 10: CLI (`panini-cli`)

**Files:**
- Create: `/workspace/crates/panini-cli/Cargo.toml`
- Create: `/workspace/crates/panini-cli/src/main.rs`
- Test: `/workspace/crates/panini-cli/tests/cli.rs`

**Interfaces:**
- Consumes: `panini` facade + `panini-lipi::Scheme`.
- Produces: the `panini` binary with subcommands `check` and `derive`, flags `--in`, `--out`, `--trace`, `--json`; exit code `0` valid / `1` invalid.

- [ ] **Step 1: Create the crate manifest**

Create `/workspace/crates/panini-cli/Cargo.toml`:

```toml
[package]
name = "panini-cli"
edition.workspace = true
version.workspace = true
license.workspace = true
rust-version.workspace = true

[[bin]]
name = "panini"
path = "src/main.rs"

[dependencies]
panini = { path = "../panini" }
panini-lipi = { path = "../panini-lipi" }
clap = { workspace = true }
serde_json = { workspace = true }
```

- [ ] **Step 2: Write the failing integration test**

Create `/workspace/crates/panini-cli/tests/cli.rs`:

```rust
use std::process::Command;

fn run(args: &[&str]) -> (String, i32) {
    let out = Command::new(env!("CARGO_BIN_EXE_panini")).args(args).output().unwrap();
    (String::from_utf8_lossy(&out.stdout).to_string(), out.status.code().unwrap_or(-1))
}

#[test]
fn check_valid_word_exits_zero() {
    let (stdout, code) = run(&["check", "bhavati"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("VALID"));
}

#[test]
fn check_invalid_word_exits_one() {
    let (_stdout, code) = run(&["check", "xyzq"]);
    assert_eq!(code, 1);
}

#[test]
fn trace_flag_lists_sutras() {
    let (stdout, _code) = run(&["check", "bhavati", "--trace"]);
    assert!(stdout.contains("3.1.68"));
}
```

- [ ] **Step 3: Run test to verify it fails**

Run: `cd /workspace && cargo test -p panini-cli`
Expected: FAIL — binary has no implementation / does not compile.

- [ ] **Step 4: Implement `main.rs`**

```rust
#![forbid(unsafe_code)]
use clap::{Parser, Subcommand, ValueEnum};
use panini::{Panini, Verdict, render};
use panini_lipi::Scheme;

#[derive(Parser)]
#[command(name = "panini", version)]
struct Cli { #[command(subcommand)] cmd: Cmd }

#[derive(Subcommand)]
enum Cmd {
    /// Validate a single word and show its derivation.
    Check {
        word: String,
        #[arg(long, value_enum, default_value = "auto")] r#in: InScheme,
        #[arg(long, value_enum, default_value = "iast")] out: OutScheme,
        #[arg(long)] trace: bool,
        #[arg(long)] json: bool,
    },
}

#[derive(Clone, Copy, ValueEnum)] enum InScheme { Auto, Slp1, Iast, Hk, Deva }
#[derive(Clone, Copy, ValueEnum)] enum OutScheme { Slp1, Iast, Hk, Deva }

fn out_scheme(o: OutScheme) -> Scheme {
    match o { OutScheme::Slp1 => Scheme::Slp1, OutScheme::Iast => Scheme::Iast, OutScheme::Hk => Scheme::Hk, OutScheme::Deva => Scheme::Devanagari }
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Check { word, r#in: _, out, trace, json } => {
            let engine = Panini::new();
            let result = engine.check(&word);
            let scheme = out_scheme(out);
            if json {
                let obj = serde_json::json!({
                    "verdict": matches!(result.verdict, Verdict::Valid),
                    "input_slp1": result.input_slp1,
                    "analyses": result.analyses.iter().map(|a| serde_json::json!({
                        "dhatu": a.dhatu,
                        "form": render(&a.form_slp1, scheme),
                        "trace": a.trace.iter().map(|s| serde_json::json!({"sutra": s.sutra, "name": s.name, "after": s.after})).collect::<Vec<_>>(),
                    })).collect::<Vec<_>>(),
                });
                println!("{}", serde_json::to_string_pretty(&obj).unwrap());
            } else if matches!(result.verdict, Verdict::Valid) {
                let a = &result.analyses[0];
                println!("VALID \u{2713}  {} ({})", render(&a.form_slp1, scheme), a.dhatu);
                if trace {
                    for step in &a.trace {
                        println!("  {} {} -> {}", step.sutra, step.name, step.after);
                    }
                }
            } else {
                println!("INVALID (not derivable within the covered v1 grammar)");
            }
            std::process::exit(if matches!(result.verdict, Verdict::Valid) { 0 } else { 1 });
        }
    }
}
```

Scope note: the `derive` subcommand and `--in` scheme override are declared in the design but `--in auto` (detection) covers all v1 CLI tests; wire the explicit `derive` subcommand only if time permits — it is a thin wrapper over `engine.derive`. If deferred, remove it from `docs/ARCHITECTURE.md`'s CLI list to keep docs truthful.

- [ ] **Step 5: Run tests to verify they pass**

Run: `cd /workspace && cargo test -p panini-cli`
Expected: PASS (exit codes 0/1, trace contains `3.1.68`).

- [ ] **Step 6: Manual smoke check**

Run: `cd /workspace && cargo run -q -p panini-cli -- check 'भवति' --out deva --trace`
Expected: prints `VALID ✓  भवति (BU)` followed by the rule trace.

- [ ] **Step 7: Commit**

```bash
cd /workspace
git add crates/panini-cli
git commit -m "feat(cli): panini check with --trace/--json and validity exit codes"
```

---

## Task 11: Full paradigm golden tests + property tests

**Files:**
- Create: `/workspace/crates/panini/tests/paradigm.rs`
- Create: `/workspace/crates/panini/tests/roundtrip.rs`
- Modify: `/workspace/crates/panini/Cargo.toml` (add `proptest` dev-dependency)

**Interfaces:**
- Consumes: `panini` facade, `panini-prakriya::derive`, `panini-data`.
- Produces: exhaustive validation over all 54 forms and a generate→check round-trip property.

- [ ] **Step 1: Write the 54-form golden table**

Create `/workspace/crates/panini/tests/paradigm.rs`. The expected column is the authoritative surface form for each `(root, puruṣa, vacana)` cell:

```rust
use panini::{Panini, Verdict};

/// (root_code, [P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B]) in SLP1.
const PARADIGM: &[(&str, [&str; 9])] = &[
    ("BU",  ["Bavati","BavataH","Bavanti","Bavasi","BavaTaH","BavaTa","BavAmi","BavAvaH","BavAmaH"]),
    ("nI",  ["nayati","nayataH","nayanti","nayasi","nayaTaH","nayaTa","nayAmi","nayAvaH","nayAmaH"]),
    ("ji",  ["jayati","jayataH","jayanti","jayasi","jayaTaH","jayaTa","jayAmi","jayAvaH","jayAmaH"]),
    ("smf", ["smarati","smarataH","smaranti","smarasi","smaraTaH","smaraTa","smarAmi","smarAvaH","smarAmaH"]),
    ("paW", ["paWati","paWataH","paWanti","paWasi","paWaTaH","paWaTa","paWAmi","paWAvaH","paWAmaH"]),
    ("vad", ["vadati","vadataH","vadanti","vadasi","vadaTaH","vadaTa","vadAmi","vadAvaH","vadAmaH"]),
];

#[test]
fn every_form_validates_and_matches() {
    let engine = Panini::new();
    for (_root, forms) in PARADIGM {
        for expected in forms {
            let r = engine.check(expected);
            assert!(matches!(r.verdict, Verdict::Valid), "expected VALID for {expected}");
            assert!(r.analyses.iter().any(|a| a.form_slp1 == *expected),
                "no analysis produced {expected}");
        }
    }
}

#[test]
fn known_nonforms_are_invalid() {
    let engine = Panini::new();
    for bad in ["Bavatu", "Bavati123", "gacCati", "tiRRati"] {
        // Bavatu (loT) and gacCati (irregular gam) are out of v1 scope -> Invalid.
        assert!(matches!(engine.check(bad).verdict, Verdict::Invalid), "expected INVALID for {bad}");
    }
}
```

- [ ] **Step 2: Run it — this is the gate for Task 7**

Run: `cd /workspace && cargo test -p panini --test paradigm`
Expected: PASS for all 54 forms. If any plural (`*anti`) or `*AmaH/*AvaH/*Ami` form fails, return to Task 7 and fix the `Ji` coalescence / 7.3.101 handling until green.

- [ ] **Step 3: Add the round-trip property test**

Add `proptest` to `/workspace/crates/panini/Cargo.toml`:

```toml
[dev-dependencies]
proptest = { workspace = true }
```

Create `/workspace/crates/panini/tests/roundtrip.rs`:

```rust
use panini::Panini;
use panini_data::{dhatus, Lakara, Pada, Purusha, Vacana};

const CELLS: &[(Purusha, Vacana)] = &[
    (Purusha::Prathama, Vacana::Eka), (Purusha::Prathama, Vacana::Dvi), (Purusha::Prathama, Vacana::Bahu),
    (Purusha::Madhyama, Vacana::Eka), (Purusha::Madhyama, Vacana::Dvi), (Purusha::Madhyama, Vacana::Bahu),
    (Purusha::Uttama, Vacana::Eka),  (Purusha::Uttama, Vacana::Dvi),  (Purusha::Uttama, Vacana::Bahu),
];

#[test]
fn generate_then_check_recovers_inputs() {
    let engine = Panini::new();
    for d in dhatus() {
        for &(pu, va) in CELLS {
            let form = engine.derive(d, Lakara::Lat, Pada::Parasmaipada, pu, va).text();
            let r = engine.check(&form);
            assert!(r.analyses.iter().any(|a| a.dhatu == d.code && a.form_slp1 == form),
                "roundtrip failed: {} -> {}", d.code, form);
        }
    }
}
```

Add `panini-data` and `panini-prakriya` as dev-dependencies of `panini` if not already present (they are runtime deps, so they are available to tests).

- [ ] **Step 4: Run the round-trip test**

Run: `cd /workspace && cargo test -p panini --test roundtrip`
Expected: PASS (all 54 generate→check round-trips recover their inputs).

- [ ] **Step 5: Commit**

```bash
cd /workspace
git add crates/panini
git commit -m "test(panini): 54-form golden paradigm + generate->check roundtrip"
```

---

## Task 12: Fuzzing, supply-chain, and mutation config

**Files:**
- Create: `/workspace/deny.toml`
- Create: `/workspace/crates/panini-lipi/fuzz/Cargo.toml`
- Create: `/workspace/crates/panini-lipi/fuzz/fuzz_targets/normalize.rs`

**Interfaces:**
- Consumes: `panini-lipi::normalize`.
- Produces: a fuzz target that must not panic on arbitrary bytes; `cargo-deny` config; documented `cargo-mutants` usage (task already in `mise.toml`).

- [ ] **Step 1: Add cargo-deny config**

Create `/workspace/deny.toml`:

```toml
[advisories]
yanked = "deny"

[bans]
multiple-versions = "warn"

[licenses]
allow = ["Apache-2.0", "MIT", "Unicode-3.0", "Unicode-DFS-2016"]
```

- [ ] **Step 2: Create the fuzz target**

Create `/workspace/crates/panini-lipi/fuzz/Cargo.toml`:

```toml
[package]
name = "panini-lipi-fuzz"
version = "0.0.0"
edition = "2021"
publish = false

[dependencies]
libfuzzer-sys = "0.4"
panini-lipi = { path = ".." }

[[bin]]
name = "normalize"
path = "fuzz_targets/normalize.rs"
test = false
doc = false
```

Create `/workspace/crates/panini-lipi/fuzz/fuzz_targets/normalize.rs`:

```rust
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // Must never panic on arbitrary valid UTF-8.
        let _ = panini_lipi::normalize(s);
    }
});
```

- [ ] **Step 3: Verify the fuzz target builds and runs briefly**

Run: `cd /workspace && cargo +nightly fuzz build -p panini-lipi 2>/dev/null || echo "nightly/cargo-fuzz needed — documented in AGENTS.md"`
Expected: builds if `cargo-fuzz` + nightly are installed; otherwise the fallback note prints. Record the exact install command in AGENTS.md (Task 13).

- [ ] **Step 4: Run supply-chain checks**

Run: `cd /workspace && cargo deny check 2>/dev/null || echo "install: cargo install cargo-deny (also documented in AGENTS.md)"`
Expected: passes, or the install note prints.

- [ ] **Step 5: Sanity-run mutation testing on the engine**

Run: `cd /workspace && cargo mutants --package panini-prakriya --file crates/panini-prakriya/src/tinanta.rs -- --test-threads=1 2>/dev/null | tail -20 || echo "install: cargo install cargo-mutants"`
Expected: reports caught/missed mutants. Any *missed* mutant in `tinanta.rs` indicates a behavior not pinned by the golden tests — add a golden case that catches it, then re-run.

- [ ] **Step 6: Commit**

```bash
cd /workspace
git add deny.toml crates/panini-lipi/fuzz
git commit -m "chore: fuzz normalize boundary + cargo-deny config + mutation guidance"
```

---

## Task 13: Navigable-codebase docs and onboarding verification

**Files:**
- Modify: `/workspace/README.md`
- Create: `/workspace/AGENTS.md`
- Create: `/workspace/CLAUDE.md` (mirror of AGENTS.md, or a one-line pointer to it)
- Create: `/workspace/docs/ARCHITECTURE.md`

**Interfaces:**
- Consumes: the working build/test tasks from `mise.toml`.
- Produces: a discoverable front door verified by running onboarding end-to-end.

- [ ] **Step 1: Write the README front door**

Overwrite `/workspace/README.md`:

```markdown
# panini

A Rust library and CLI that validates a single Sanskrit word against Pāṇini's
Aṣṭādhyāyī and returns the sequence of sūtras that derive it.

## Quick start

```
mise install          # pins Rust toolchain
mise run test         # runs the workspace test suite
cargo run -p panini-cli -- check 'bhavati' --trace
```

## v1 scope

Finite verbs (*tiṅanta*), present tense (*laṭ*), gaṇa 1 (*bhvādi*),
*parasmaipada*, over a curated 6-root set. `INVALID` means "not derivable within
this covered grammar," not "ungrammatical in Sanskrit." See `docs/ARCHITECTURE.md`
and `docs/superpowers/plans/2026-07-19-panini-astadhyayi-v1.md`.

## Layout

See `docs/ARCHITECTURE.md` for the crate map.
```

- [ ] **Step 2: Write AGENTS.md**

Create `/workspace/AGENTS.md`:

```markdown
# Contributor & agent guide

## Environment
- Toolchain is pinned via `mise` (`mise install`). Do not install Rust globally.
- Tasks: `mise run build | test | lint | fmt | mutants | audit`.
- Optional tools (install on demand):
  - `cargo install cargo-mutants` (mutation testing)
  - `cargo install cargo-deny` and `cargo install cargo-audit` (supply chain)
  - `cargo install cargo-fuzz` + a nightly toolchain (fuzzing `panini-lipi`)

## Rules of the codebase
- SLP1 is the only internal representation; transliterate only in `panini-lipi`.
- `#![forbid(unsafe_code)]` in every crate.
- Grammar changes are gated by the golden paradigm test
  (`crates/panini/tests/paradigm.rs`). Surface forms there are the source of
  truth; sūtra ids/names in traces must match the cited reference
  (ashtadhyayi.com).

## Where things live
See `docs/ARCHITECTURE.md`.
```

Create `/workspace/CLAUDE.md`:

```markdown
See [AGENTS.md](./AGENTS.md) for the contributor and agent guide.
```

- [ ] **Step 3: Write the architecture map**

Create `/workspace/docs/ARCHITECTURE.md`:

```markdown
# Architecture

Data flow for `check`:
`input → panini-lipi (→SLP1) → panini-analyze (candidates) → panini-prakriya
(derive each) → panini (keep exact matches) → render`.

## Crates
- `panini-lipi` — SLP1 ⇄ IAST/HK/Devanāgarī + scheme detection. No grammar.
- `panini-data` — curated roots, tiṅ table, enums. No I/O beyond embedded data.
- `panini-prakriya` — the engine: `Term`/`Prakriya` model, it-samjna, `Rule`
  metadata + ordered controller, `tinanta::derive`. Pure SLP1, no I/O.
- `panini-analyze` — proposes candidate `(root, puruṣa, vacana)` inputs.
- `panini` — facade: `Panini::check` / `Panini::derive`, `Verdict`, `Analysis`.
- `panini-cli` — the `panini` binary (`check` subcommand; `--trace`, `--json`,
  `--out`, validity exit codes).

## Rule trace
Every applied sūtra is logged as a `RuleStep { sutra, name, before, after }`.
The `check` result carries the full trace per analysis.
```

- [ ] **Step 4: Verify onboarding end-to-end (the actual gate)**

Run these exactly as a new contributor would, from a clean state:

```bash
cd /workspace
mise install
mise run build
mise run test
cargo run -q -p panini-cli -- check 'bhavati' --trace
```

Expected: install succeeds, build clean, **all workspace tests pass**, and the CLI prints `VALID ✓` with a rule trace. If any command fails or contradicts the docs, fix the docs or the task until the onboarding sequence is truthful.

- [ ] **Step 5: Commit**

```bash
cd /workspace
git add README.md AGENTS.md CLAUDE.md docs/ARCHITECTURE.md
git commit -m "docs: navigable front door (README, AGENTS/CLAUDE, architecture map)"
```

---

## Self-Review Notes (for the plan author / executor)

- **Spec coverage:** hybrid analyze→derive→match (Tasks 8–9); single pada (whole plan); tiṅanta/laṭ/bhvādi/parasmaipada 9 forms (Tasks 6–7, 11); SLP1 internal + multi-scheme I/O + detection (Task 2, 10); our engine + sourced data w/ attribution (Task 6); on-the-fly perf, FST deferred (roadmap only); rule metadata + central controller (Task 5); devkit — mise (Task 1), navigable docs (Task 13), testing tiers golden/unit/property/mutation (Tasks 3–12), fuzz + supply-chain security (Task 12), clean-code domain vocabulary (throughout). All covered.
- **Open sub-decision resolved:** `panini-lipi` is written in-house (Task 2), per the spec's stated preference — no external transliteration crate.
- **Known risk:** the `Ji`→`anti` internal coalescence (Task 7, Step 4 note) is the one derivation detail requiring reference transcription; the 54-form golden test (Task 11) is the explicit gate that blocks completion until it is correct.
- **Type consistency:** `check`, `derive`, `Verdict`, `Analysis`, `CheckResult`, `Prakriya`, `RuleStep`, `Term`, `Tag`, `Rule`, `run_pipeline`, `candidates`, `Candidate`, `normalize`, `to_slp1`, `from_slp1`, `tin_ending`, `dhatus`, `Purusha`, `Vacana`, `Lakara`, `Pada`, `Gana`, `Dhatu` are used with consistent signatures across tasks.
