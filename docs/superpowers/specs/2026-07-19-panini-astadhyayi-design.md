# Pāṇini — Astādhyāyī Derivation Engine & Validator (v1 Design)

**Status:** Approved design, ready for implementation planning
**Date:** 2026-07-19
**Scope:** v1 vertical slice — see "Scope" below

## Summary

A Rust workspace providing a **library** (`panini`) and a **CLI** (`panini`) that,
given a single Sanskrit word (*pada*), reports whether it is valid according to
Pāṇini's Aṣṭādhyāyī and returns the ordered sequence of sūtras (rules) that
derive it.

The Aṣṭādhyāyī is fundamentally a **generative** grammar: its sūtras derive
surface words forward from underlying roots and affixes (*dhātu + pratyaya →
pada*), resolving rule conflicts in a defined order. Validation therefore uses a
**hybrid strategy**: a lightweight analyzer proposes candidate morphological
inputs for the surface word, a faithful forward-derivation engine derives each
candidate, and any candidate whose generated form **exactly matches** the input
is reported as valid — the derivation that produced the match *is* the rule
trace.

## Goals

- Given one word, return `VALID` / `INVALID` **within the covered grammar**, and
  for valid words the ordered list of sūtras that fired (id + name + effect).
- Be faithful to Pāṇini: conflict ordering follows the *paribhāṣā* meta-rules
  (apavāda ≻ utsarga, para, nitya, antaraṅga), owned in one place.
- Be fast: near-instant single-word checks (on-the-fly derivation), with a clear
  path to corpus-scale throughput later.
- Accept and emit multiple transliteration schemes.
- Follow the `/devkit:*` practices for environment, structure, testing, and
  security.

## Non-Goals (v1)

- Sentences / *vākya*, sandhi-splitting across words, *kāraka*/agreement checks.
- Compounds (*samāsa*) and cross-morpheme internal sandhi beyond what the v1
  slice requires.
- Lakāras other than *laṭ*; gaṇas other than *bhvādi*; *ātmanepada*; *subanta*
  (nominal declension). All deferred to later phases.
- The precomputed FST / index layer (Phase 2+).

## Scope (v1 vertical slice)

- **Input unit:** a single inflected word (*pada*).
- **Word class:** *tiṅanta* (finite verbs).
- **Coverage:** *laṭ* (present tense), *bhvādi* (gaṇa 1), *parasmaipada*, all
  three *puruṣa* × all three *vacana* (9 forms per root), over a small curated
  root set (~20–50 roots).
- **Direction:** analyze → forward-derive → match → trace, on-the-fly.

Everything outside this slice is future work (see Roadmap) but the architecture
is designed to extend to it.

## Core Design Decisions

These were settled during brainstorming and are fixed for v1:

1. **Bridging strategy:** Hybrid — analyzer proposes candidates, forward
   derivation confirms and produces the trace. The analyzer only needs to
   *narrow*; the derivation is the source of truth.
2. **Input unit:** single *pada*.
3. **v1 word class:** *tiṅanta*, *laṭ* / *bhvādi* / *parasmaipada*, all
   *puruṣa* × *vacana*.
4. **Encoding:** SLP1 internally (one ASCII char per phoneme); a transliteration
   boundary accepts and emits IAST, SLP1, Harvard-Kyoto, and Devanāgarī, with
   input-scheme auto-detection and selectable output scheme.
5. **Prior art / data:** we implement the engine ourselves; linguistic **data**
   (Dhātupāṭha, gaṇa tables, sūtra metadata) is bootstrapped from an
   openly-licensed source (Vidyut / ashtadhyayi.com), attributed with license
   text preserved.
6. **Performance:** phased — on-the-fly derivation first (fast single-check);
   an optional precomputed FST/index layer later, built on the same derivation
   output.
7. **Rule model:** each sūtra is a coded rule carrying declarative metadata
   (id, name, kind, adhikāra scope, tags); a central controller uses that
   metadata to resolve *paribhāṣā* ordering. Ordering logic lives in the
   controller, not scattered across sūtras.

## Architecture

### Workspace layout

```
panini/                      (workspace root)
├─ crates/
│  ├─ panini-lipi/           transliteration: SLP1 ⇄ IAST/HK/Devanāgarī + scheme auto-detect
│  ├─ panini-data/           sourced datasets (Dhātupāṭha, gaṇas, sūtra metadata) + typed loaders
│  ├─ panini-prakriya/       THE ENGINE: Term model, Prakriyā state, rules, controller, paribhāṣā
│  ├─ panini-analyze/        analyzer: surface pada → candidate (dhātu, lakāra, puruṣa, vacana)
│  ├─ panini/                library facade: ties analyze + derive + match; public API
│  └─ panini-cli/            the `panini` binary (clap-based)
```

Each crate has one clear purpose and a narrow interface:

- `panini-prakriya` never touches I/O or transliteration; it operates purely in
  SLP1 and is testable with zero I/O.
- `panini-lipi` knows nothing about grammar.
- `panini` (facade) is the only crate that composes analysis + derivation +
  matching + rendering.

### Engine data model (`panini-prakriya`)

- **`Term`** — one morpheme unit: its SLP1 text plus a set of **saṃjñā tags**
  (`dhātu`, `pratyaya`, `aṅga`, `sārvadhātuka`, it-markers, etc.). Tags are how
  sūtras locate their targets, mirroring Pāṇini's *saṃjñā*-s.
- **`Prakriyā`** — the derivation state: an ordered `Vec<Term>` **plus a log**
  `Vec<RuleStep>`, where `RuleStep { sutra: "3.1.68", name: "kartari śap",
  before, after }`. This log *is* the rule trace — a first-class output, not an
  afterthought.

### Rules and controller

Each sūtra is a coded rule carrying declarative metadata:

```rust
Rule {
  id: "3.1.68", name: "kartari śap",
  kind: Vidhi, adhikāra: /* scope */, tags: /* utsarga | apavāda, … */,
  matches: |p| /* condition */,
  apply:   |p| /* mutate Prakriyā + auto-log */,
}
```

A central **controller** runs the derivation loop:

1. Find all rules whose `matches` condition holds in the current state.
2. Resolve conflicts using the *paribhāṣā* meta-rules **in one place**
   (apavāda ≻ utsarga; para; nitya; antaraṅga).
3. Apply the winning rule; log the `RuleStep`.
4. Repeat to a fixpoint (no rule applies).

**Determinism:** given fixed inputs the derivation is deterministic. Optional
*vibhāṣā* rules branch into multiple valid outputs — the engine returns a set,
and each branch carries its own trace.

## Data Flow — a single `check`

```
input "भवति" / "bhavati" / "Bavati"
   │
   ▼  panini-lipi: auto-detect scheme → normalize to SLP1 "Bavati"
   │
   ▼  panini-analyze: propose candidate inputs
   │     inverse-index the tiṅ endings + root set →
   │     [ (√bhū, bhvādi, laṭ, prathama, eka), … ]   (small candidate set)
   │
   ▼  panini-prakriya: forward-derive EACH candidate (deterministic; vibhāṣā → set)
   │     √bhū + laṭ + tiṅ → … → "Bavati"  + trace[]
   │
   ▼  panini (facade): keep candidates whose derived SLP1 == input
   │
   ▼  panini-lipi: render output in requested scheme
   ▼
VALID ✓  √bhū (bhvādi) + laṭ, prathama-puruṣa eka-vacana
trace: 3.1.68 kartari śap → 7.3.84 sārvadhātukārdhadhātukayoḥ
     → 6.1.78 eco'yavāyāvaḥ → 3.4.78 tiptasjhi… → …
```

The analyzer only needs to *narrow*, not be correct — derivation is the source
of truth. If zero candidates match, the word is `INVALID` **within v1's covered
grammar**, and the output states this explicitly to avoid over-claiming
(absence of a derivation in the covered slice is not proof of ungrammaticality
in full Sanskrit).

## Interfaces

### Library API (`panini` crate)

```rust
let engine = Panini::new();                     // loads embedded data once
let result = engine.check("bhavati");           // auto-detect scheme
// result.verdict: Valid | Invalid
// result.analyses: Vec<Analysis>               // each: inputs + Vec<RuleStep>
result.analyses[0].trace();                     // ordered sūtra steps

engine.derive(Dhatu::bhu(), Lakara::Lat, Purusha::Prathama, Vacana::Eka); // direct generation
```

Traces are structured data (sūtra id + name + before/after), so consumers render
them however they like.

### CLI (`panini-cli`)

```
panini check <word> [--in auto|slp1|iast|hk|deva] [--out iast|slp1|deva] [--trace] [--json]
panini derive --dhatu bhU --lakara lat --purusha prathama --vacana eka
panini --version
```

- `--trace` prints the full rule sequence.
- `--json` emits the structured result for programmatic use.
- Exit code reflects validity (`0` valid / `1` invalid) for scripting.

## Data Sourcing (`panini-data`)

Dhātupāṭha (roots + gaṇa + markers/meanings), gaṇa tables, and sūtra metadata
are bootstrapped from an openly-licensed source (Vidyut's data /
ashtadhyayi.com), **attributed in `data/ATTRIBUTION.md` with license text
preserved**. For v1 (laṭ / bhvādi) this is a small curated subset checked into
the repo and **embedded at compile time** (no runtime file I/O; fast cold
start). An `xtask` regenerates/updates the data reproducibly.

**Open sub-decision (to resolve during implementation):** `panini-lipi`
transliteration is infrastructure, not the Aṣṭādhyāyī itself. Preference is to
write a small SLP1/IAST/HK/Devanāgarī converter ourselves to keep the dependency
tree lean and fully ours; depending on an existing crate remains an option. To
be confirmed before that crate is built.

## Testing & Tooling (respecting `/devkit:*`)

- **developer-environment:** `mise` pins the Rust toolchain and dev tools (no
  ambient installs); reproducible across machines.
- **navigable-codebases:** `README` + `AGENTS.md` / `CLAUDE.md` front door;
  common workflows exposed as named tasks (build / test / lint / mutants); a
  short codebase map documenting crate boundaries; onboarding verified by
  running it.
- **testing-practices (speed-tiered):**
  - *Golden / snapshot tests* — the workhorse: tables of
    `input → expected(verdict, form, trace)`. Every covered form and its exact
    sūtra sequence is pinned, so any drift in rule ordering is caught.
  - *Unit tests* per rule and per *paribhāṣā* resolution.
  - *Property tests* (proptest) — e.g. round-trip: `derive(inputs)` then
    `check(output)` must recover those inputs; transliteration round-trips.
  - *Mutation testing* (`cargo mutants`) on the engine/controller to ensure
    tests actually pin behavior.
- **writing-clean-code:** domain vocabulary throughout (*dhātu*, *pratyaya*,
  *aṅga*, *saṃjñā*) — the code reads in the grammar's own language.
- **security-practices:** input is untrusted text — fuzz the transliteration /
  analyzer boundary (`cargo fuzz`) so malformed bytes cannot panic;
  `cargo audit` / `cargo deny` for supply chain; no `unsafe` unless justified
  and reviewed.

## Roadmap

- **Phase 1 (v1, this spec):** the vertical slice — laṭ / bhvādi / parasmaipada,
  all *puruṣa* × *vacana*; full analyze → derive → match → trace; multi-scheme
  I/O; CLI + library; golden / property / mutation tests; devkit tooling.
  On-the-fly derivation only.
- **Phase 2+ (future specs, out of scope now):** more lakāras (laṅ, loṭ, …),
  *ātmanepada*, more gaṇas, then *subanta*, then compounds / sandhi / sentences;
  and the optional precomputed FST / index layer for corpus-scale throughput,
  built on the same derivation output.

## Success Criteria (v1)

- Every form in the v1 coverage set (curated roots × 9 tiṅ forms) validates as
  `VALID` and returns a correct, ordered sūtra trace, pinned by golden tests.
- Known non-forms in the covered slice return `INVALID`.
- All four I/O schemes round-trip; Devanāgarī input and output work.
- Single-word `check` is near-instant (interactive-fast) with near-zero cold
  start.
- Mutation testing shows the engine/controller behavior is genuinely pinned by
  tests.
