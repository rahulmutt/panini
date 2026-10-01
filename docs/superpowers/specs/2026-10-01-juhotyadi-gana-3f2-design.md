# Juhotyādi gaṇa (gaṇa 3), slice 3f2 — √bhas

Slice 3f's spec (`2026-10-01-juhotyadi-gana-3f-design.md`) deferred two roots
to a 3f2 that would close the gaṇa:

> **3f2** (√bhas, √jan) closes the gaṇa. From this spec's probe: new 6.4.100
> and 8.2.26 for √bhas (*bapsati*, *babDaH*, *babDi*), with 8.2.73 / 8.2.74
> widened (*abaBat*, *abaBaH*) and 8.4.55 reading word-internally (`Bs` →
> `ps`); new 6.4.98, 6.4.42 and the vikalpa 6.4.43 for √jan (*jajYati*,
> *jajAtaH*, *jajAyAt* ~ *jajanyAt*), with 8.4.40 widened to a palatal on the
> left (`jn` → `jY`). … Re-probe before writing its spec.

We re-probed both rows at the audited commit
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` and ran this engine at `e660c48`
over the same cells on hand-built rows (parasmaipada, stored text `Bas` /
`jan`), diffing derivation sets cell by cell
(`/tmp/vidyut-full/vidyut-prakriya/examples/juhotyadi_3f2_probe.rs`, a copy of
3f's probe with the other four rows removed). HEAD matches **20 of 72** cells;
vidyut holds 95 forms.

| row | root | HEAD | new rules | widenings |
|---|---|---|---|---|
| 03.0019 | √bhas | 7/36 | 6.4.100, 8.2.26 | 8.2.73, 8.2.74, 8.4.55 |
| 03.0025 | √jan | 13/36 | 6.4.98, 6.4.42, 6.4.43 (vikalpa) | 8.4.40 |

(√jan is at 13, not 3f's 12: laṭ madhyama eka *jajaMsi* matches now, through
the 8.3.24 widening 3f landed. Its remaining 23 cells need the three new rules
and 8.4.40.)

**This spec splits 3f2.** The two roots share no rule. √bhas is two new rules
and three tripādī widenings. √jan is three aṅga rules, a new vikalpa, and a
guard that must keep 6.4.42 *jana-sana-khanāṁ* off tanādi's already-curated
√san (*sanutaH*, where `u` intervenes). **3f2 takes √bhas; √jan becomes slice
3f3, which closes the gaṇa.**

The slice also carries the **Rust toolchain bump, 1.98.1 → 1.99.0**
(`mise.toml`), as its first commit. fmt-check, clippy `-D warnings` and the
full suite all pass on 1.99.0 at `e660c48`, checked while writing this spec.
The mutation gate's floor is re-measured on 1.99.0 anyway (below), so a
toolchain timing shift is caught there.

## Scope

**Row (1):** `03.0019 Basa~` (√bhasa), parasmaipadī by 1.3.78 (no ṅit or
svarita marker). **36 cells, 44 forms.** Suite 4572 → 4608 cells (508 → 512
blocks × 9), 5655 → 5699 forms; roots 101 → 102; juhotyādi 24 → 25 of its 26
rows.

**New rules (2):** 6.4.100 *ghasibhasor hali ca* (`guna.rs`) and 8.2.26 *jhalo
jhali* (`tripadi.rs`).

**Widened rules (3):** 8.2.73 *tipy anasteḥ* and 8.2.74 *sipi dhāto rur vā*
(both drop their rudhādi test), and 8.4.55 *khari ca* (reads the whole word,
not only the aṅga/ending junction). All in `tripadi.rs`.

**No new vikalpa** — the engine stays at eleven optional rules. The 8 extra
forms are 8.2.74's existing fork (newly reachable) plus the standing 7.1.35 /
8.4.56 forks.

**Unchanged, confirmed on the prototype:** 1.2.4, 1.3.78, 2.4.75, 3.4.103,
6.1.10, 6.4.71, 6.4.101, 7.1.4, 7.1.35, 7.2.79, 7.4.60, and in `tripadi.rs`
8.2.23, 8.2.25, 8.2.39, 8.2.40, 8.3.15, 8.4.53, 8.4.54 and 8.4.56.

**Out of scope:** √jan (3f3); 6.4.98, 6.4.42, 6.4.43; widening 8.4.40; √ghas
(6.4.100's other root, not curated); curādi.

## Evidence the design is complete and inert

A throwaway worktree at `e660c48` carried exactly the five changes below
(sketch-quality code, not shipped). With it:

- the 3f2 probe matched **√bhas 36/36**;
- the whole suite passed except `tinanta_rule_order_is_pinned`, which fails by
  construction when rules are added;
- a dump of every curated cell's credited-rule log (5805 prakriyā lines) was
  **byte-identical** between `main` and the prototype. No prior cell's trace
  moves, not only its golden.

The widening of 8.2.73 / 8.2.74 alone was also run first: the suite passed
unchanged, and it took √bhas to 9/36 (the two laṅ eka cells).

## The new rules

### 6.4.100 *ghasibhasor hali ca*

The upadhā `a` of √ghas and √bhas is elided before a kṅit — a hal-initial one
by *hali*, a vowel-initial one by *ca*, which carries 6.4.98's *aci*. So the
condition is simply "before a kṅit". `Bas` → `Bs`:

| cell | follower | result |
|---|---|---|
| laṭ prathama dvi | `tas` (ṅit by 1.2.4) | *babDaH* |
| laṭ prathama bahu | `ati` (7.1.4) | *bapsati* |
| loṭ prathama eka | `tAt` (7.1.35 tātaṅ, ṅit) | *babDAt* — beside *baBastu*, where `tu` is pit and the rule declines |
| loṭ madhyama eka | `hi` | *babDi* |
| vidhiliṅ, all | `yA…` (3.4.103 yāsuṭ, ṅit) | *bapsyAt* |

Laṭ/laṅ eka (tip, sip, mip) are pit and decline: *baBasti*, *abaBat*.

- **Stage: `guna.rs`, beside 6.4.112 and the other kṅit-on-abhyasta rules.**
  That is after dvitva (6.1.10 has filled ABHYASA, and ANGA holds `Bas`) and
  before `adesha.rs`'s 6.4.101 *hu-jhalbhyo her dhiḥ*. vidyut runs 6.4.100
  then 6.4.101 on *babDi*; the order is immaterial to 6.4.101's own guard
  (`Bas` and `Bs` both end in the jhal `s`) but is pinned by the trace test
  below.
- **The follower is read through `following_sarvadhatuka`**, as 6.4.112's
  abhyasta arm reads it. For juhotyādi SHAP is luk'd empty, so it resolves to
  the ending, whose `Tag::Ngit` already encodes 1.2.4, 7.1.35 and 3.4.103.
- **The root is KEYED BY ROW NUMBER, `03.0019`**, following 8.2.40's *adhaḥ*
  (`03.0011`). A text test on `Bas` is the shape memory warns against: laṅ's
  aṭ has been prefixed onto aṅga text before. The rule then edits ANGA's
  text, which must be `Bas`. √ghas is not curated; the comment says so and
  names the row-number key as the place a √ghas row would extend.
- **Comment** records that *ca* brings *aci* (so no hal/vowel test is
  written), names √bhas's witnesses, and states the stage placement and why.

### 8.2.26 *jhalo jhali*

An `s` between two jhals is elided. `Ba` `Bs` `tas` → `Ba` `B` `tas`, then
8.2.40 (`t` → `D` after the jhaṣ `B`), 8.4.53 (`B` → `b` before the jhaś
`D`) and 8.4.54 give *babDaH*. Likewise *babDa*, *ababDAm*, *babDAt*.

- **Placement: `tripadi.rs`, immediately after 8.2.25 *dhi ca*,** in sūtra
  order. On *babDi* 8.2.25 is the one that fires (the `s` is before `D`, and
  8.2.25 does not need a jhal on its left); 8.2.26 then finds no `s`. vidyut's
  trace is the same.
- **Scope: the whole word**, via `word_chars`, as 8.2.40 and 8.4.53 scan. The
  sūtra has no positional restriction, and the trace diff shows no prior cell
  presents jhal + `s` + jhal anywhere.
- **Comment** names *babDaH* as the witness, records the corpus-wide
  inertness, and says what would be the first non-√bhas witness (an
  s-aorist, out of this engine's four lakāras).

## The widenings

### 8.2.73 *tipy anasteḥ* and 8.2.74 *sipi dhāto rur vā* — the gaṇa test is dropped

√bhas laṅ prathama eka: a + Ba + Bas + t → 6.1.68 / 8.2.23 → *abaBas* →
8.2.73 → *abaBad* → *abaBat* ~ *abaBad* (8.4.56). Laṅ madhyama eka: 8.2.74's
ru branch gives *abaBaH*; its declined branch reaches 8.2.73 (the recorded
deliberate over-application to sip) and gives *abaBad* ~ *abaBat*. HEAD gives
*abaBaH* alone in both cells.

- **Both guards lose `Tag::Rudhadi`** and keep the rest: 8.2.74
  `ctx.is_sip()`, `dhatu_is_pada_final`, `s`-final; 8.2.73
  `dhatu_is_pada_final`, `s`-final. This mirrors 3f's 8.2.75 change.
- **Evidence the gaṇa test was an engine artefact:** removing both left the
  suite and every prior trace unchanged. No curated non-rudhādi cell presents
  an `s`-final dhātu with an empty ending. *anasteḥ* (√as) needs no clause:
  √as is not curated.
- **Order unchanged** (8.2.74 → 8.2.75 → 8.2.73, against sūtra order, for the
  reasons the existing comments give).
- **8.2.73's recorded hazard is re-verified, as its comment demands.** It
  asks for re-verification "before widening the root set further". The new
  record: the rule is now gaṇa-free; √bhas empties `ENDING` at exactly laṅ
  prathama/madhyama eka, the same slot family; the corpus-wide test below
  makes "fires only on rudhādi and √bhas" a standing fact. The obligatory
  rule's silent over-fire hazard is restated, not closed.
- **Comment sweep in this region:** `dhatu_is_pada_final`'s comment says
  "8.2.73 and 8.2.74 guard on `Tag::Rudhadi` first"; 8.2.75's says it stays
  safe "because 8.2.73 still runs later and is still rudhādi-only". Both are
  rewritten.

### 8.4.55 *khari ca* — reads the whole word

A jhal before a khar becomes its car. *bapsati*: `Bs` → `ps`, inside the aṅga
term, where HEAD's rule never looks — it reads only the last sound before
`ENDING` against the ending's first.

- **The rule becomes a whole-word scan** via `word_chars`: the first jhal
  immediately followed by a khar whose `cartva_of` differs from itself. The
  no-op guard stays (an already-car jhal records nothing).
- **Why whole-word rather than an aṅga-internal arm:** it is the sūtra's own
  scope, it matches how 8.2.40 and 8.4.53 already read, and the trace diff
  shows it changes no prior cell. Every prior 8.4.55 firing is at the
  junction, and the scan meets the junction pair at the same position.
- **The existing long comment** (the √ad / rudhādi `Kindte` history of
  reading the junction) is rewritten to say the rule now scans the word, and
  why the junction reasoning it records is subsumed rather than lost.

## What vidyut credits that this slice does not write

- **6.1.68** on laṅ eka — this engine credits 8.2.23.
- **8.2.66**, **8.4.68**, **6.1.4**, **6.1.5** — the standing conventions.

## Corpus

`panini-data` gains one row, in dhātupāṭha order before `03.0020`:

| row | entry | root | pada | sanction |
|---|---|---|---|---|
| 03.0019 | `Basa~` | Bas | parasmai | 1.3.78 |

Its comment names its path: 6.4.100, 8.2.26, 8.4.55, 8.2.73 / 8.2.74. The
juhotyādi row-list assertion gains it, and its comment moves to twenty-five of
twenty-six with "slice 3f3 closes it".
`curated_pada_agrees_with_upadesha_markers` and
`dhatupatha_numbers_resolve_upstream` must pass without edits.

## Tests

- **Goldens** (`crates/panini/tests/paradigm/data/juhotyadi.rs`): the 36
  cells, transcribed from vidyut's probe output. **ALTERNATES gains 8 rows**
  over 5 cells:

  | cell | forms |
  |---|---|
  | laṅ prathama eka | *abaBat*, *abaBad* (2) |
  | laṅ madhyama eka | *abaBaH*, *abaBat*, *abaBad* (3, with 8.2.74) |
  | loṭ prathama eka | *baBastu*, *babDAt*, *babDAd* (3) |
  | loṭ madhyama eka | *babDi*, *babDAt*, *babDAd* (3) |
  | vidhiliṅ prathama eka | *bapsyAt*, *bapsyAd* (2) |

  The engine's own log ∩ `VIKALPA_RULES` keys them, never an invented key.
- **Trace pins** (`crates/panini/tests/trace/juhotyadi.rs`):
  - *bapsati* (laṭ prathama bahu): 6.4.100, then 8.4.55;
  - *babDaH* (laṭ prathama dvi): 6.4.100 → 8.2.26 → 8.2.40 → 8.4.53;
  - *babDi* (loṭ madhyama eka): 6.4.100 before 6.4.101, then 8.2.25, no 8.2.26;
  - *baBastu* (loṭ prathama eka): no 6.4.100 (tip is pit);
  - *abaBat* (laṅ prathama eka): 8.2.73;
  - *abaBaH* (laṅ madhyama eka): 8.2.74, no 8.2.73.
  The juhotyādi allowed-list test gains `03.0019` and its comment retargets to
  3f3.
- **Guard unit tests** in `derivation_tests.rs`, slice-7 style:
  - 6.4.100 fires on `03.0019` before a ṅit follower, and declines before a
    pit one, and on another juhotyādi row before a ṅit one;
  - 8.2.26 elides an `s` between two jhals, and declines with a vowel on
    either side;
  - 8.4.55 fires word-internally (`Bs` + vowel-initial ending) and still at
    the junction, and records nothing on an already-car jhal.
- **Rule order:** `tinanta_rule_order_is_pinned` gains 6.4.100 and 8.2.26 at
  their positions.
- **Corpus-wide fires-only-on-rows test**, extending 3f's: over every curated
  cell, 6.4.100 and 8.2.26 are credited only on `03.0019`; 8.2.73 and 8.2.74
  only on rudhādi rows and `03.0019`; 8.4.55 credited at a non-junction
  position only on `03.0019`. Goldens ignore traces, so this is what pins
  "no prior cell changed".
- **Prior traces:** diff every prior cell's trace between `main` and the
  slice HEAD; the 4572 prior cells stay byte-identical, traces included.
- **Spot check** in `paradigm/main.rs` beside 3f's: *bapsati*, *babDaH*,
  *abaBat*.

## Audit

Copy the committed `tools/audit/panini_full_audit.rs`, never rewrite it. Bump
its invariants to **102 roots, 4608 cells (512 blocks × 9), 5699 forms (4608 +
1091 ALTERNATES rows)**, with
`derivation_set_shape_matches_the_audited_numbers` raised to match. Repoint
`/tmp/vidyut-full`'s dev-deps at the slice worktree before running, and
restore them after. Zero difference against `8da2f90b` is required, with the
mis-resolved-root negative control shown failing first.

## Mutation gate

Re-measure the uncaught-suite floor at `-j 4` on Rust 1.99.0 and set the cap
from that measurement (6× the floor, rounded up to the next 10 s, the
AGENTS.md rule). Copy `outcomes.json` durably before any further invocation.
Expected new non-caught mutants: none. 6.4.100's row key and ṅit test are
killed by its decline tests and the √bhas goldens; 8.2.26's two jhal tests by
its unit tests; 8.4.55's scan by *bapsati* on the internal side and the
existing junction goldens on the other. AGENTS.md names the non-caught set
verbatim.

## Doc sweep

README, AGENTS.md and `docs/ARCHITECTURE.md`: 101 → 102 roots, twenty-four →
twenty-five of twenty-six juhotyādi rows, 4572 → 4608 cells, 5655 → 5699
forms, ALTERNATES 1083 → 1091, multi-form cells 792 → 797 (two-form 594 →
596, three-form 152 → 155, four-form unchanged).

The grep covers wrapped counts, spelled-out numbers, rule-scoped counts, the
root-shape literals (`Bas`, `Bs`, `Ba`), every "rudhādi only" phrasing of
8.2.73 / 8.2.74 anywhere in comments, every comment that describes 8.4.55 as
reading the junction, and every "3f2" promise in code and docs. Each 3f2
promise is rewritten as done where it is √bhas's, or retargeted to 3f3 where
it is √jan's; the 3f spec's "Later slices" line gains a note recording this
split. Recorded file:line anchors are re-grepped at final HEAD.

## Success criteria

- `mise.toml` at Rust 1.99.0; fmt-check, lint and test pass on it.
- 4608 cells valid with the trace pins; the 4572 priors byte-identical,
  traces included.
- Audit zero-difference at 102 / 4608 / 5699 against `8da2f90b`, with the
  negative control verified failing.
- Gate clean at a re-measured cap; the non-caught set named verbatim in
  AGENTS.md.
- Juhotyādi at twenty-five of its twenty-six rows; no "3f2" promise left that
  is not done or 3f3's.

## Later slices

- **3f3** (√jan) closes the gaṇa. From this spec's probe: new 6.4.98
  *gamahanajanakhanaghasāṁ lopaḥ* (*jajYati*, *ajajYuH*), 6.4.42
  *janasanakhanāṁ sañjhaloḥ* (*jajAtaH*, *jajAhi*), and the vikalpa 6.4.43 *ye
  vibhāṣā* (*jajAyAt* ~ *jajanyAt*, the engine's twelfth optional rule), with
  8.4.40 widened to a palatal on the left (`jn` → `jY`). 6.4.42 must stay off
  tanādi's √san, where `u` intervenes. vidyut runs 6.4.42 before 6.1.10; check
  that this engine's post-dvitva placement reaches the same forms. Re-probe
  before writing its spec.
