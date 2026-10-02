# Curādi gaṇa (gaṇa 10), slice 10b — the ākusmīya roots and 10.0496

## Summary

Slice 10a opened curādi at 4 of its 509 rows, all ubhayapadī by 1.3.74
*ṇicaś ca*. Its "Later slices" list put the ātmanepadī roots next, framed as
"how 1.3.12 and 1.3.74 meet on one row". The vidyut probe shows that framing
is wrong for the class it names. The ātmanepadī curādi roots are the
**ākusmīya** antargaṇa, rows `10.0192` (`cita~`) through `10.0236`
(`kusma~`). Their upadeśas carry no anudātta or ṅit marker. Their ātmanepada
comes from the dhātupāṭha's own gaṇasūtra **10.0496 *ā kusmād
ātmanepadinaḥ***, which vidyut fires right after the dhātu's it-saṁjñā,
before 3.1.25. After it, no pada sūtra fires at all: no 1.3.12, no 1.3.74,
no 1.3.78. Parasmaipada derives nothing.

This slice curates four of those rows, one per pre-ṇic shape, plus the two
range ends:

| row | upadeśa | code | laṭ prathama eka | pre-ṇic change |
|---|---|---|---|---|
| `10.0192` | `cita~` | `cit` | *cetayate* | 7.3.86 (i → e) |
| `10.0228` | `vfza~` | `vfz` | *varzayate* | 7.3.86 (ṛ → ar) |
| `10.0229` | `mada~` | `mad` | *mAdayate* | 7.2.116 |
| `10.0236` | `kusma~` | `kusm` | *kusmayate* | none |

`10.0192` and `10.0236` are the first and last rows of the range. √vṛṣ is
the first golden reach of the pre-ṇic 7.3.86's ṛ arm (`guna_of('f')` →
`ar`); 10a's goldens only reached its i arm.

The slice's one architectural decision is how to credit a rule that is not
an Aṣṭādhyāyī sūtra. 10.0496 is the engine's first gaṇasūtra id.

## Scope

**In:** the four rows above, ātmanepada only, across laṭ / laṅ / loṭ /
vidhiliṅ (144 cells); `PadaAssignment::Akusmiya`; the `AKUSMIYA` range;
`Tag::Akusmiya`; the 10.0496 rule; 1.3.78's ātmanepada decline for an
ākusmīya aṅga.

**Out:**

- the other ākusmīya rows. Most need nothing new and are left for a later
  slice to curate in bulk. The exceptions belong to other classes:
  optional-ṇic rows (`10.0193 daSi~`, `10.0194 dasi~`, `10.0198 tatri~`,
  `10.0199 matri~`, `10.0227 vancu~`, `10.0230 divu~`), and 7.2.115 rows
  (`10.0231 gf`, `10.0235 yu`). Slice 10c curated thirty-three of them in bulk — see
  `2026-10-02-curadi-gana-10c-design.md`;
- `10.0219 lakza~`, deliberately: `10.0006 lakza~` is an ubhayapadī
  homograph that a later slice curates. (Reversed in slice 10c: rows are keyed by number, so the homograph needs
  no structural support, and 10c curated it.);
- the ā-garvīya roots (10.0497 *ā garvād ātmanepadinaḥ*: pada, gṛha,
  mṛga …). Every one of them is adanta, so they wait for the adanta slice;
- `10.0058 zmiN`, the only ṅit curādi row. It is ātmanepadī by 1.3.12 but
  needs 7.2.115 (`smAyayate`). This is where 1.3.12 and ṇic actually meet;
- everything 10a listed: optional ṇic, mit, adanta, the causative,
  ārdhadhātuka lakāras.

## Evidence

vidyut-prakriya at `8da2f90` (the vendored dhātupāṭha's commit) derives all
four rows as above, via a throwaway probe example
(`/tmp/vidyut-full/vidyut-prakriya/examples/curadi_probe_10b.rs`, a copy of
10a's, not shipped).

- `maybe_find_antargana` (`src/dhatupatha.rs`) assigns
  `Antargana::Akusmiya` to curādi numbers `192..=236`.
- `dhatu_karya.rs` then runs `DP("10.0496")`, adding `PT::Atmanepada` to the
  prakriyā, unless the row took the optional no-ṇic branch.
- `atmanepada.rs` returns early when the prakriyā already carries a pada
  tag, so no pada sūtra is credited.

*cetayate*, it-saṁjñā and 1.4.13 steps omitted: **10.0496** `cit` →
3.1.25 `cit Ric` → 1.3.9 `cit i` → 3.4.114 → 7.3.86 `cet i` → 3.1.32 →
3.2.123 → 3.4.78 `… ta` → 3.1.68 → 3.4.79 `… te` → 1.2.4 → 7.3.84 `cet e a
te` → 6.1.78 `cet ay a te`.

The probe gives every parasmaipada cell of all four rows zero forms. Every
ātmanepada cell has exactly one form. Laṭ for all four:

- `10.0192`: cetayate cetayete cetayante cetayase cetayeTe cetayaDve cetaye cetayAvahe cetayAmahe
- `10.0228`: varzayate varzayete varzayante varzayase varzayeTe varzayaDve varzaye varzayAvahe varzayAmahe
- `10.0229`: mAdayate mAdayete mAdayante mAdayase mAdayeTe mAdayaDve mAdaye mAdayAvahe mAdayAmahe
- `10.0236`: kusmayate kusmayete kusmayante kusmayase kusmayeTe kusmayaDve kusmaye kusmayAvahe kusmayAmahe

The other three lakāras are ordinary thematic ātmanepada paradigms
(*acetayata*, *cetayatAm*, *cetayeta* …). The goldens are transcribed from
the probe, not from this list.

## Architecture

Three ways to credit 10.0496 were weighed:

1. **A rule with id `10.0496`, first in `sanadi`, before 3.1.25 (chosen).**
   The step sits where vidyut's does.
2. Credit it in `samjna`, next to the pada sūtras. Rejected: smaller, but the
   step lands after 3.1.32, out of vidyut's order. That would be the first
   pada sanction whose position disagrees with vidyut's.
3. Curate the rows as plain `Atmanepada` and let 1.3.12 credit them.
   Rejected: the trace would name a sūtra whose condition (an anudātta/ṅit
   marker) the root does not meet. This is exactly what `UbhayapadaAnavane`
   and `Nic` exist to prevent.

### Data layer — `panini-data`

- **`PadaAssignment::Akusmiya`.** `padas()` returns `&[Pada::Atmanepada]`.
  The doc comment follows `Nic`'s: the sanction is the dhātupāṭha
  gaṇasūtra 10.0496, read off the row's position in the dhātupāṭha, not any
  marker of the root or any affix. A root carrying it must never be credited
  1.3.12, 1.3.74 or 1.3.78.
- **`AKUSMIYA`**, the inclusive dhātupāṭha range `10.0192..=10.0236`, with a
  doc comment citing 10.0496 and vidyut's `maybe_find_antargana`. It is
  keyed by number, as `GHU` is, because the gaṇasūtra is positional: it
  reads "up to kusm", not a property of the root.
- **The four rows**, `gana: Gana::Curadi`, `pada: PadaAssignment::Akusmiya`,
  each with a comment naming its pre-ṇic shape and "Ātmanepadī by 10.0496.
  Slice 10b.", in the style of 10a's rows.

Membership stays curated, but it is pinned both ways by a new test: a
curated row is `Akusmiya` **iff** it is curādi and its number lies in
`AKUSMIYA`. Every other curated curādi row is `Nic`.

`curated_pada_agrees_with_upadesha_markers`'s curādi arm is split the same
way. Both kinds of row still assert that the markers derive
`Parasmaipada`; the curated column is then `Akusmiya` in range and `Nic`
out of it. The arm's comment is corrected. It currently says the ākusmīya
roots *carry a marker* and will fail here; they carry none. The test's doc
census ("re-derives 102 of these 107 … the four curādi rows' are 1.3.74's")
becomes 102 of 111, with the four 1.3.74 rows and four 10.0496 rows named.

### Prakriyā — `panini-prakriya`

- **`Tag::Akusmiya`** in `term.rs`, documented like `Tag::Nic`: read only by
  10.0496 and by 1.3.78's ātmanepada decline.
- **`tinanta/mod.rs`** adds it from `PadaAssignment::Akusmiya`, next to the
  `Nic` arm. samjna's test helper `pada_anga` gains the same arm. Both
  matches are exhaustive, so neither compiles until it is updated.
- **10.0496 in `SANADI`**, first, before 3.1.25:

  ```rust
  Rule {
      id: "10.0496",
      name: "A kusmAd AtmanepadinaH",
      kind: RuleKind::Vidhi,
      vikalpa: false,
      bars: &[],
      apply: …,
  }
  ```

  Guard: `Tag::Akusmiya` on `ANGA`; otherwise decline and record nothing.
  On `Pada::Atmanepada` it records the step and returns true. On
  `Pada::Parasmaipada` it sets `p.blocked` and records nothing. This is
  1.3.12's wrong-pada block, moved to where this sanction is decided. The
  controller skips a blocked branch for every later rule, so nothing
  downstream records on it. `padas()` never requests that cell; the block
  exists for a direct `derive` / `check` with the wrong pada.

  The comment above the rule says it is a dhātupāṭha gaṇasūtra, not a
  sūtra of the Aṣṭādhyāyī; that it is numbered as vidyut numbers it
  (`DP("10.0496")`, the gaṇasūtra's position in the dhātupāṭha); and that
  it is the first such id in the engine. No code parses rule ids. The CLI,
  analyzer and audit harness were checked: the id is an opaque string
  everywhere.

  The module doc's rule list (`3.1.25, ṇic's it-lopa (1.3.9), 3.4.114,
  7.2.116, 7.3.86, 3.1.32`) gains 10.0496. "Every rule self-guards" gains
  10.0496's guard.

- **`samjna`.** 1.3.12 guards on `Atmanepadin` and 1.3.74 on `Nic`; an
  ākusmīya aṅga carries neither, so both already decline. 1.3.78's guard is
  `!Atmanepadin`, which admits it, so its ātmanepada arm adds
  `Tag::Akusmiya` to the decline list next to `Ubhayapadin`, `Anavane` and
  `Nic`. Without that, every ākusmīya golden would block. Its parasmaipada
  arm is unreachable for an ākusmīya root, because the branch is already
  blocked. The 1.3.78 comment that lists which sūtra "has already
  sanctioned this cell" names 10.0496 too.

From 3.1.25 on, an ākusmīya row runs through exactly 10a's pipeline;
nothing after `sanadi` learns about it except 1.3.78.

## Testing

TDD; every new test is seen failing before the code that passes it.

- **Goldens:** 144 cells, 4 roots × 4 lakāras × 9 ātmanepada cells,
  transcribed from the probe. All single-form, so ALTERNATES and the
  multi-form census are unchanged.
- **10.0496 unit tests**, in `sanadi.rs`, shaped like 1.3.74's:
  - it fires and records on an `Akusmiya` ātmanepada prakriyā;
  - on parasmaipada it blocks, returns false and records nothing;
  - it declines silently, unblocked, with an empty log, on √cur (`Nic`),
    √rudh (`Ubhayapadin`), √bhū (none) and an `Atmanepadin` root.
- **The other pada sūtras leave an ākusmīya root alone:** 1.3.12, 1.3.66,
  1.3.72 and 1.3.74 decline on it in both padas. 1.3.78 declines on its
  ātmanepada without blocking.
- **Trace:** *cetayate*'s log has `10.0496` immediately before `3.1.25`, and
  no `1.3.12`, `1.3.66`, `1.3.72`, `1.3.74` or `1.3.78` step. Requesting any
  parasmaipada cell of the four roots yields only blocked prakriyās.
- **Corpus-wide:** 10.0496 appears in the trace of exactly the 144 new
  cells. Every one of the 4932 prior cells' traces is byte-identical
  between main and HEAD (the diff, run before the final review).
- **Data:** the `AKUSMIYA` iff test; the split pada-agreement arm;
  `dhatupatha_numbers_resolve_upstream` covers the four numbers;
  `curated_roots_have_expected_ganas_and_padas` gains the four rows.
- **Homographs:** before any `check()` assertion names the root a form
  belongs to, the goldens are grepped for that form.

## Audit

Copy the committed harness from `tools/audit/`; do not rewrite it. Repoint
`/tmp/vidyut-full`'s dev-deps at the worktree before running, or the audit
measures the pre-slice engine. The negative control must fail first. Then
the full run: **111 roots / 5076 cells / 6206 forms, zero differences.** The
harness's own root-count comments move with it. Its both-pada count (30)
does not change.

## Mutation gate

Re-measure the uncaught-suite floor at `-j 4` and set the cap from it (6×
the floor, rounded up to the next 10 s). Pass `-o`. Copy `outcomes.json`
durably before any further invocation. Expected new non-caught mutants:
none.

- 10.0496's tag guard: killed by the decline unit tests and the
  corpus-wide fires-only test.
- Its pada match and the block: killed by the goldens and the parasmaipada
  block test.
- 1.3.78's `Akusmiya` decline: removing it blocks all 144 goldens.

AGENTS.md names the non-caught set verbatim.

## Doc sweep

README, AGENTS.md, `docs/ARCHITECTURE.md`, the audit harness:

- 107 → 111 roots; 4932 → 5076 cells; 6062 → 6206 forms;
- curādi "open at 4 of 509" → **8 of 509**; the gaṇa tallies stay at ten
  gaṇas, nine complete;
- ARCHITECTURE's tātaṅ paragraph: "the curated set's 25 ātmanepada-only
  roots" → 29; its 82-root / 164-cell parasmaipada census and the thirty
  both-pada roots are unchanged;
- ALTERNATES (1130), the multi-form census, pada-ambiguous surfaces (72),
  the stage count (nine): unchanged. Each is re-derived from the test
  census, not assumed;
- every place that says curādi's pada comes from 1.3.74 alone, or that
  enumerates the pada sūtras / sanctions (the `samjna` module doc, the
  `PadaAssignment` docs, AGENTS.md's pada section), gains 10.0496;
- the 10a spec: its out-of-scope bullet "ātmanepadī (ākusmīya) roots … how
  1.3.12 and 1.3.74 meet on one row" and its "Later slices" line gain a
  pointer to this spec and the correction that the class is 10.0496's.

The grep covers wrapped counts, spelled-out numbers and rule-scoped counts;
the root-shape literals (`cit`, `ceti`, `vfz`, `varzi`, `mad`, `mAdi`,
`kusm`, `kusmi`); "1.3.74" wherever it is presented as curādi's only
sanction; every "ātmanepada-only" census. Recorded file:line anchors are
re-grepped at final HEAD.

## Success criteria

- 144 new cells match vidyut; all 4932 prior cells and their traces are
  byte-identical.
- The audit reports zero differences over 111 roots / 5076 cells / 6206
  forms, after its negative control fails.
- The mutation campaign's non-caught set is unchanged from 10a's, named in
  AGENTS.md.
- `mise run test` and clippy are clean.

## Later slices

The remaining ākusmīya rows in bulk (taken by slice 10c, all but the optional-ṇic and 7.2.115 rows); then 10a's order: mit roots, adanta
roots (with the ā-garvīya list, 10.0497), optional ṇic (which takes the
optional-ṇic ākusmīya rows, since 10.0496 applies only on the ṇic branch).
√smiṅ, and with it 7.2.115 before ṇic, rides whichever slice first needs
7.2.115.
