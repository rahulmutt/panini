# Juhotyādi gaṇa (gaṇa 3), slice 3f3 — √jan, closing the gaṇa

Slice 3f2's spec (`2026-10-01-juhotyadi-gana-3f2-design.md`) split √jan off
into this slice:

> **3f3** (√jan) closes the gaṇa. From this spec's probe: new 6.4.98
> *gamahanajanakhanaghasāṁ lopaḥ* (*jajYati*, *ajajYuH*), 6.4.42
> *janasanakhanāṁ sañjhaloḥ* (*jajAtaH*, *jajAhi*), and the vikalpa 6.4.43 *ye
> vibhāṣā* (*jajAyAt* ~ *jajanyAt*, the engine's twelfth optional rule), with
> 8.4.40 widened to a palatal on the left (`jn` → `jY`). 6.4.42 must stay off
> tanādi's √san, where `u` intervenes. vidyut runs 6.4.42 before 6.1.10; check
> that this engine's post-dvitva placement reaches the same forms. Re-probe
> before writing its spec.

We re-probed at the audited commit `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`
against this engine at `436a507`, on a hand-built row (parasmaipada, stored
text `jan`). The probe is
`/tmp/vidyut-full/vidyut-prakriya/examples/juhotyadi_3f3_probe.rs`, a copy of
3f2's probe with the √bhas row removed. HEAD matches **13 of 36** cells, as
3f2 recorded. vidyut holds **51 forms**.

| row | root | HEAD | new rules | widenings |
|---|---|---|---|---|
| 03.0025 | √jan | 13/36 | 6.4.98, 6.4.42, 6.4.43 (vikalpa) | 8.4.40 (converse arm) |

The 13 matching cells are the pit cells (laṭ/laṅ eka; loṭ uttama) and the
uttama dvi/bahu cells, where `v`/`m` is neither a vowel nor a jhal. None of
the new rules fires in any of them.

## Scope

**Row (1):** `03.0025 jana~ janane` (√jan), parasmaipadī by 1.3.78. (Not
`janI~`: that is divādi's `04.0044 janI~\` *prādurbhāve*.) **36 cells, 51
forms.** Suite 4608 → 4644 cells (512 → 516 blocks × 9), 5699 → 5750 forms;
roots 102 → 103; juhotyādi 25 → **26 of its 26 rows. The gaṇa is closed.**

**New rules (3), all in `guna.rs`:** 6.4.98 *gamahanajanakhanaghasāṁ lopaḥ
kṅity anaṅi*, 6.4.42 *janasanakhanāṁ sañjhaloḥ*, and 6.4.43 *ye vibhāṣā*. The
last is a vikalpa, the engine's **twelfth** optional rule.

**Widened rule (1):** 8.4.40 *stoḥ ścunā ścuḥ* gains its converse arm (a ścu
on the left, a stu on the right). The 8.4.44 *śāt* exemption is a guard inside
that arm. `tripadi.rs`.

**Unchanged, confirmed on the prototype:** 1.2.4, 1.3.78, 2.4.75, 3.4.103,
6.1.10, 6.4.71, 7.1.4, 7.1.35, 7.2.79, 7.3.87, 7.4.60, 8.2.39, 8.3.15,
8.3.24, 8.4.56 and 8.4.58.

**Out of scope:** √gam, √han, √khan and √ghas (the other roots 6.4.98 and
6.4.42 name; none is curated); 8.4.44 as a rule of its own; curādi (opened in
slice 10a — `2026-10-01-curadi-gana-10a-design.md`).

## Evidence the design is complete and inert

A throwaway worktree at `436a507` carried exactly the four changes below
(sketch-quality code, not shipped). With it:

- the 3f3 probe matched **√jan 36/36**;
- a dump of every prior curated cell's live-branch credited-rule log (5699
  lines, one per form) was **byte-identical** between `main` and the
  prototype. No prior cell's trace moves, not only its golden. In particular,
  the 8.4.40 converse arm meets no other ścu-before-stu site in the corpus
  that its *śāt* guard does not exempt;
- the whole suite passed except six tests, each failing by construction:
  - `tinanta_rule_order_is_pinned` and
    `exactly_the_pinned_vikalpa_rules_are_optional` (rules were added);
  - `juhotyadi_rows_are_the_twenty_five_curated_roots`,
    `curated_roots_have_expected_ganas_and_padas` and
    `paradigm_covers_every_enumerable_cell` (a row was added without goldens);
  - two allow-list trace tests, which √jan legitimately joins and vidyut
    credits too: 7.3.87 (`nabhyastasyaci_is_credited_only_on_…`, on
    *ajajanam* and the other pit vowel-initial cells) and 8.3.24
    (`nas_capadantasya_is_credited_only_on_rudhadi_and_dhan`, on *jajaMsi*,
    *jajanti*, *jajantu*).

## The new rules

All three read the follower through `following_sarvadhatuka`, as 6.4.100 and
6.4.115 do. Under ślu, SHAP is empty, so it resolves to the ending. The
ending's `Tag::Ngit` already encodes 1.2.4, 7.1.35 and 3.4.103. All three edit
ANGA's text with `strip_suffix("an")`. That works whether or not laṅ's aṭ has
been prefixed onto a term, and it declines on an already-edited `jn` or `jA`.

### 6.4.98 *gamahanajanakhanaghasāṁ lopaḥ kṅity anaṅi*

The upadhā `a` is elided before a **vowel-initial** kṅit (*aci*, by anuvṛtti
from 6.4.77). `jan` → `jn`:

| cell | follower | result |
|---|---|---|
| laṭ prathama bahu | `ati` (7.1.4) | *jajYati* |
| laṅ prathama bahu | `us` (3.4.109) | *ajajYuH* |
| loṭ prathama bahu | `atu` | *jajYatu* |

The loṭ uttama endings (`Ani`, `Ava`, `Ama`) are pit by 3.4.92 āṭ, so the rule
declines there: *jajanAni*. *anaṅi* is vacuous in this engine's four lakāras
and gets no test; the comment says so.

- **Stage:** `guna.rs`, **immediately before 6.4.100**, its sibling
  upadhā-lopa rule, in sūtra order. No ā-rule meets `jan` or `jn`, so nothing
  forces the position. The *jajYati* trace pin holds it.
- **KEYED BY ROW NUMBER, `03.0025`.** Of the five roots the sūtra names, only
  √jan is curated, and the comment names the key as the place a √gam, √han,
  √khan or √ghas row would extend.

### 6.4.42 *janasanakhanāṁ sañjhaloḥ*

The final `n` becomes `ā` before a **jhal-initial** kṅit (the san arm is out of
scope: no desideratives). `jan` → `jA`:

| cell | follower | result |
|---|---|---|
| laṭ prathama/madhyama dvi, madhyama bahu | `tas`, `Tas`, `Ta` | *jajAtaH*, *jajATaH*, *jajATa* |
| laṅ dvi/bahu (non-uttama) | `tAm`, `tam`, `ta` | *ajajAtAm*, … |
| loṭ prathama eka | `tAt` (7.1.35 tātaṅ, ṅit) | *jajAtAt* — beside *jajantu*, where `tu` is pit |
| loṭ madhyama eka | `hi`; `tAt` | *jajAhi*, *jajAtAt* |
| loṭ dvi/bahu (non-uttama) | `tAm`, `tam`, `ta` | *jajAtAm*, … |

- **KEYED BY ROW NUMBER, `{03.0025, 08.0002}`**: √jan and tanādi's √san. Both
  curated roots the sūtra names are in the key. **√san's exclusion is
  grammatical, not a missing key.** On √san, `following_sarvadhatuka` returns
  the vikaraṇa `u`, which is neither ṅit (1.2.4's second arm excludes it) nor
  a jhal. 6.4.107's u-lopa runs later, in `adesha.rs`, so this stage always
  sees the `u`. Both tests decline on every √san cell, and √san's existing
  72 goldens (*sanutaH*, *sanvaH*, …) pin the decline. The comment says this, and that √khan would extend the key.
- **Writes `jA` directly** and credits only 6.4.42. vidyut writes `jaA` and
  credits 6.1.101. This engine's 6.1.101 is arm-based with no in-term arm, and
  widening it would change no form. The omission is listed below.

### 6.4.43 *ye vibhāṣā* — the twelfth vikalpa

The same substitution, optionally, before a **y-initial** kṅit. In this corpus
that means yāsuṭ, so all nine vidhiliṅ cells fork: *jajanyAt* (declined,
branch 0) ~ *jajAyAt*. Same key, same `√san` argument: `sanuyAt`'s follower
is `u`.

### Placement of 6.4.42 and 6.4.43 — after the ā-of-abhyasta block

Both sit **after 6.4.115, at the end of `GUNA`**, below 6.4.119, 6.4.118,
6.4.117, 6.4.116, 6.4.113, 6.4.100 (and the new 6.4.98) and 6.4.112.

The new `jA` is an `A`-final abhyasta aṅga, which is exactly what those rules
read:

| if 6.4.42/6.4.43 ran earlier | rule | wrong form |
|---|---|---|
| `jA` + `tas` | 6.4.112 *śnābhyastayor ātaḥ* | *jajtaH |
| `jA` + `tas` (hal-initial) | 6.4.113 *ī halyaghoḥ* | *jajItaH |
| `jA` + `yAt` | 6.4.112, or 6.4.118 if its key widened | *jajyAt |

Textually this is 6.4.22 *asiddhavat atrābhāt*: 6.4.42 and 6.4.112/6.4.113
are both in the ābhīya section, so 6.4.42's `ā` is asiddha to them. Placing
the substitution after them implements that. The comment cites 6.4.22, names
the three wrong forms, and points at the trace pins that hold the order.

**Post-dvitva placement reaches vidyut's forms** (3f2's open question). vidyut
runs 6.4.42 before 6.1.10, so the abhyāsa copies `jaA`, and 7.4.60 / 7.4.59
reduce it to `ja`. Here dvitva copies `jan`, and 7.4.60 reduces it to `ja`.
The abhyāsa is `ja` on both paths. The probe's 36/36 confirms it.

## The widening

### 8.4.40 *stoḥ ścunā ścuḥ* — the converse arm, with *śāt*

*jajYati*: `ja` `jn` `ati` — the stu `n` follows the ścu `j` and becomes `Y`.
HEAD reads only stu-before-ścu.

- **The arm:** in the existing `word_chars` scan, a ścu at `w[i]` followed by a
  stu at `w[i + 1]` substitutes `shcutva_of(w[i + 1])`, **unless `w[i]` is
  `S`**. That guard is 8.4.44 *śāt*, and the comment names it. The
  substitute is still the map, with no literal.
- **Why a guard, not an 8.4.44 rule:** vidyut credits 8.4.44 118 times over
  √kliś (*kliSnAti*) and √aś (*aSnoti*). A crediting rule would move those
  118 prior traces and introduce a niṣedha shape the engine does not have.
  As a guard, no prior trace moves. The kliSnAti/aSnoti goldens kill a mutant
  on the guard. The omitted credit is listed below.
- **The "ONE DIRECTION ONLY" comment is rewritten.** It recorded the converse
  arm as a deliberate non-implementation, to be added "together [with SAt]
  the moment a curated root puts a stu after a ścu". √jan is that root. The
  comment now records both arms, the guard, and √jan's three witnesses. Its
  sections on 8.4.41 and 8.3.24 stay, re-read against the new arm.
- **Order unchanged** (immediately above 8.4.41).

## What vidyut credits that this slice does not write

- **6.1.101** on every 6.4.42/6.4.43 form (`jaA` → `jA`). This engine writes
  `jA` in one step.
- **8.4.44** *śāt* on √kliś/√aś (a guard here, not a rule).
- **6.1.68**, **8.2.66**, **8.4.68**, **6.1.4**, **6.1.5** — the standing
  conventions.

## Corpus

`panini-data` gains one row, in dhātupāṭha order between `03.0024` and
`03.0026`:

| row | entry | root | pada | sanction |
|---|---|---|---|---|
| 03.0025 | `jana~` | jan | parasmai | 1.3.78 |

Its comment names its path: 6.4.98, 6.4.42, 6.4.43, 8.4.40. The juhotyādi
row-list assertion gains it, and the test is renamed from
`juhotyadi_rows_are_the_twenty_five_curated_roots` to say twenty-six. Its
comment records the gaṇa as closed.
`curated_pada_agrees_with_upadesha_markers` and
`dhatupatha_numbers_resolve_upstream` must pass without edits.

## Tests

- **Goldens** (`crates/panini/tests/paradigm/data/juhotyadi.rs`): the 36
  cells, transcribed from vidyut's probe output. **ALTERNATES gains 15 rows**
  over 11 cells:

  | cell | forms |
  |---|---|
  | loṭ prathama eka | *jajantu*, *jajAtAt*, *jajAtAd* (3) |
  | loṭ madhyama eka | *jajAhi*, *jajAtAt*, *jajAtAd* (3) |
  | vidhiliṅ prathama eka | *jajanyAt*, *jajanyAd*, *jajAyAt*, *jajAyAd* (4) |
  | vidhiliṅ, the other 8 cells | *jajany…* ~ *jajAy…* (2 each) |

  The engine's own log ∩ `VIKALPA_RULES` keys them, never an invented key.
  `VIKALPA_RULES` in `paradigm/main.rs` gains `6.4.43`.
- **Trace pins** (`crates/panini/tests/trace/juhotyadi.rs`):
  - *jajYati* (laṭ prathama bahu): 6.4.98, then 8.4.40;
  - *ajajYuH* (laṅ prathama bahu): 6.4.71 → 6.4.98 → 8.4.40;
  - *jajAtaH* (laṭ prathama dvi): 6.4.42, and neither 6.4.112 nor 6.4.113;
  - *jajAhi* (loṭ madhyama eka): 6.4.42, no 6.4.101;
  - *jajAyAt* ~ *jajanyAt* (vidhiliṅ prathama eka): 6.4.43 on the taken branch
    only, and no 6.4.112 or 6.4.118 on either;
  - *jajanti* (laṭ prathama eka): none of 6.4.98, 6.4.42, 6.4.43 (tip is pit).
- **Allow-lists:** the 7.3.87 and 8.3.24 allow-list tests gain `03.0025`, and
  their names and comments are updated to match. The juhotyādi allowed-list
  test gains `03.0025` and records the gaṇa closed.
- **Guard unit tests** in `derivation_tests.rs`, slice-7 style:
  - 6.4.98 fires on `03.0025` before a vowel-initial ṅit follower; declines
    before a consonant-initial one, before a pit one, and on another row;
  - 6.4.42 fires on `03.0025` before a jhal-initial ṅit follower; declines
    before `y`/a vowel, before a pit one, on another row, and on `08.0002`
    with `u` at SHAP;
  - 6.4.43 fires before a y-initial ṅit follower, and declines before a jhal;
  - the order pin: 6.4.112 and 6.4.113 run on `jan` and decline, so 6.4.42's
    `jA` reaches the tripādī intact.
- **8.4.40's unit test** (`shcutva_fires_on_stu_before_shcu_and_declines_after_sha`)
  gains a converse firing case (`ja` `jn` `ati` → `jajYati`). Its kliSnAti case
  stays, and its comment changes from "not implementing the direction" to "the
  *śāt* guard".
- **Rule order:** `tinanta_rule_order_is_pinned` gains 6.4.98, 6.4.42 and
  6.4.43 at their positions. `exactly_the_pinned_vikalpa_rules_are_optional`
  gains 6.4.43.
- **Corpus-wide fires-only-on-rows test**, extending 3f2's: over every curated
  cell, 6.4.98, 6.4.42 and 6.4.43 are credited only on `03.0025`. A trace
  step carries no direction, so 8.4.40 is pinned by its credits off √jan:
  only √chid's and √chṛd's rows (`07.0003`, `07.0008`), 54 branches, the count
  on `main`.
- **Prior traces:** diff every prior cell's trace between `main` and the slice
  HEAD. The 4608 prior cells stay byte-identical, traces included.
- **Spot check** in `paradigm/main.rs` beside 3f2's: *jajYati*, *jajAtaH*,
  *jajAyAt*.

## Audit

Copy the committed `tools/audit/panini_full_audit.rs`, never rewrite it. Bump
its invariants to **103 roots, 4644 cells (516 blocks × 9), 5750 forms (4644 +
1106 ALTERNATES rows)**, with
`derivation_set_shape_matches_the_audited_numbers` raised to match. Repoint
`/tmp/vidyut-full`'s dev-deps at the slice worktree before running, and
restore them after. Zero difference against `8da2f90b` is required, with the
mis-resolved-root negative control shown failing first.

## Mutation gate

Re-measure the uncaught-suite floor at `-j 4` and set the cap from that
measurement (6× the floor, rounded up to the next 10 s, the AGENTS.md rule).
Copy `outcomes.json` durably before any further invocation. Expected new
non-caught mutants: none.

- 6.4.98's row key, ṅit test and vowel test are killed by its decline tests
  and the √jan goldens.
- 6.4.42's jhal test is killed by √jan's goldens (`ati` would become *jajAti*)
  and by the √san unit case. Its key match is killed by both.
- 6.4.43's `y` test is killed by its unit test and the vidhiliṅ goldens.
- The 8.4.40 arm is killed by *jajYati*. Its `S` guard is killed by
  *kliSnAti* / *aSnoti*.

AGENTS.md names the non-caught set verbatim.

## Doc sweep

README, AGENTS.md and `docs/ARCHITECTURE.md`:

- 102 → 103 roots;
- twenty-five → **twenty-six of twenty-six** juhotyādi rows, *gaṇa closed*:
  nine gaṇas, **nine complete**;
- 4608 → 4644 cells, 5699 → 5750 forms;
- ALTERNATES 1091 → 1106;
- multi-form cells 797 → 808 (two-form 596 → 604, three-form 155 → 157, and
  one more four-form cell, √jan's vidhiliṅ prathama eka);
- eleven → twelve optional rules.

The grep covers:

- wrapped counts, spelled-out numbers and rule-scoped counts;
- the root-shape literals (`jan`, `jn`, `jA`, `jY`);
- every "one direction only" or "stu before ścu" phrasing of 8.4.40;
- every "eleven optional" or "eleventh" vikalpa count;
- every "3f3" promise in code and docs, and every "N of its 26" or "closes
  the gaṇa" phrase.

Each 3f3 promise is rewritten as done. Recorded file:line anchors are
re-grepped at final HEAD.

## Success criteria

- 4644 cells valid with the trace pins; the 4608 priors byte-identical,
  traces included.
- Audit zero-difference at 103 / 4644 / 5750 against `8da2f90b`, with the
  negative control verified failing.
- Gate clean at a re-measured cap; the non-caught set named verbatim in
  AGENTS.md.
- Juhotyādi at twenty-six of its twenty-six rows; no "3f3" promise left
  undone.

## Later slices

Juhotyādi closes here. The next gaṇa (curādi, the only one with no curated
row) needs its own scoping probe and spec. This slice does not pre-plan it.
