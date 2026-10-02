# Curādi gaṇa (gaṇa 10), slice 10a — ṇic, 3.1.32 and 1.3.74

## Summary

Juhotyādi closed at 26 of 26 in slice 3f3; every gaṇa the engine has opened
is complete. Curādi is the one gaṇa it has never opened — `Gana` has nine
variants, and the vendored dhātupāṭha has 509 `10.x` rows. This slice opens it
with four roots that need nothing beyond the gaṇa's own machinery:

| row | upadeśa | code | laṭ prathama eka | pre-ṇic change |
|---|---|---|---|---|
| `10.0001` | `cura~` | `cur` | *corayati* / *corayate* | 7.3.86 laghūpadha guṇa |
| `10.0010` | `laqa~` | `laq` | *lAqayati* / *lAqayate* | 7.2.116 *ata upadhāyāḥ* |
| `10.0033` | `Bakza~` | `Bakz` | *Bakzayati* / *Bakzayate* | none |
| `10.0255` | `BUza~` | `BUz` | *BUzayati* / *BUzayate* | none |

Every curādi root takes **ṇic** (3.1.25) before the vikaraṇa, and by 1.3.74
*ṇicaś ca* every one of these four is ubhayapadī. The slice's one
architectural decision is where ṇic lives: it is folded into the aṅga by
**3.1.32 *sanādyantā dhātavaḥ***, in a new first pipeline stage, so the rest
of the pipeline sees an i-final dhātu (`cori`) and derives it exactly as it
derives √nī: 7.3.84 → `core`, 6.1.78 → `coray-a-ti`.

## Scope

**In:** `Gana::Curadi`; the four rows above across laṭ / laṅ / loṭ /
vidhiliṅ × parasmaipada / ātmanepada (288 cells); 3.1.25, 3.4.114, 7.2.116,
a pre-ṇic arm of 7.3.86, 3.1.32 and 1.3.74.

**Out — later curādi slices, each its own class:**

- ātmanepadī (ākusmīya) roots such as √cit *cetayate* — how 1.3.12 and 1.3.74
  meet on one row (corrected in slice 10b,
  `2026-10-02-curadi-gana-10b-design.md`: the ākusmīya rows carry no marker;
  their ātmanepada is the dhātupāṭha gaṇasūtra 10.0496's, and no pada sūtra
  is credited);
- optional-ṇic roots (*ā-dhṛṣād vā*: √cint *cintati* / *cintayati*, √arc,
  √pṝ *parati* / *pArayati*) — a whole-paradigm vikalpa, plus 7.1.58 and
  7.2.115;
- mit roots (√jñap *jYapayati*, 6.4.92 *mitāṃ hrasvaḥ*);
- adanta roots (√kath *kaTayati*, √gaṇ — 6.4.48 *ato lopaḥ* and the
  sthānivadbhāva that blocks 7.2.116);
- the causative (ṇijanta of other gaṇas), which reuses this slice's stage;
- ārdhadhātuka lakāras, where 6.4.51 *ṇer aniṭi* will read `Tag::Nijanta`.

## Evidence

vidyut-prakriya at `8da2f90b` (the vendored dhātupāṭha's commit) derives all
four rows as above, via a throwaway probe example
(`/tmp/vidyut-full/vidyut-prakriya/examples/curadi_probe_10a.rs`, not
shipped). Its laṭ prathama eka traces, it-saṁjñā steps omitted:

- *corayati*: 3.1.25 `cur Ric` → 3.4.114 → 7.3.86 `cor i` → 3.1.32 →
  3.2.123 → 1.3.78 → 3.4.78 → 3.1.68 → 7.3.84 `cor e a ti` → 6.1.78
  `cor ay a ti`;
- *corayate*: the same, with **1.3.74** where 1.3.78 stood, then 3.4.79;
- *lAqayati*: 3.1.25 → 3.4.114 → **7.2.116** `lAq i` → 3.1.32 → … ;
- *Bakzayati*, *BUzayati*: neither 7.2.116 nor 7.3.86.

The other three lakāras are ordinary thematic paradigms (*acorayat*,
*corayatu*, *corayet* …). Loṭ uttama eka shows ṇatva across the stem for the
roots with an r or ṣ trigger (*corayARi*, *BakzayARi*, *BUzayARi*) and not
for √laḍ (*lAqayAni*).

## Architecture

Three ways to seat ṇic were weighed:

1. **A 3.1.32 merge in a new pre-samjña stage (chosen).**
2. A permanent `NIC` slot between `ANGA` and `SHAP`, empty outside curādi
   (the AGAMA/ABHYASA idiom). Rejected: every index after ANGA shifts, and
   for curādi the aṅga's operative final vowel would sit in `NIC`, not
   `ANGA`, so the 358 non-test `ANGA` reads would each need a "which term is
   the aṅga" audit — the hazard class recorded twice before (reading SHAP on
   the śap-luk'd path; reading ANGA past a live vikaraṇa).
3. Replacing the slot constants with lookup functions. Rejected: the largest
   refactor, and nothing in curādi needs it.

### The `sanadi` stage — `tinanta/sanadi.rs`

A new stage, first in `TINANTA_RULES`, before `samjna`. It works on the
layout `[AGAMA, ABHYASA, ANGA, ṇic]`: before 3.1.68 index 3 is
`ENDING_PRE_SHAP`, but no tiṅ exists yet when `sanadi` runs, so the slot
holds ṇic. `terms.rs` gains a NOTE recording this, and the module-level
pipeline comment in `tinanta/mod.rs` goes from eight stages to nine.

Every rule in the stage self-guards on `Tag::Curadi` (3.1.25) or on ṇic's
*identity* at `NIC`: text `Ric` (1.3.9), `Tag::Rit` (3.4.114, 7.2.116, 3.1.32)
or `Tag::Ardhadhatuka` (7.3.86). It is never enough that some term exists at
index 3, because once `samjna` runs that index holds the tiṅ ending. So for
gaṇas 1–9 the stage is inert, and it stays inert even in a hand-built chain
that puts a tiṅ there. Rules,
in order:

1. **3.1.25** *satyāpapāśarūpavīṇātūlaślokasenālomatvacavarmavarṇacūrṇa-
   curādibhyo ṇic*: if ANGA has `Tag::Curadi`, push a term `Ric`, tagged as
   a pratyaya.
2. **It-saṁjñā on ṇic.** 1.3.7 *cuṭū* marks the initial ṇ, 1.3.3 *hal
   antyam* the final c, and 1.3.9 removes both, leaving `i` tagged with a
   new `Tag::Rit` (ṇit, named in SLP1 as `Ngit` is). The shared
   `it_samjna::run_it_samjna` is **not** reused or widened: it has no 1.3.7
   arm (it would leave `Ri`), and a general cuṭū arm would wrongly strip the
   jh of the tiṅ ending `Ji`, which tradition exempts. The stripping is local
   to the ṇic term in `sanadi`, and follows the engine's trace convention:
   one `1.3.9` record, no separate 1.3.7 / 1.3.3 credits.
3. **3.4.114** *ārdhadhātukaṃ śeṣaḥ*: tag the `i` `Ardhadhatuka`.
4. **7.2.116** *ata upadhāyāḥ*: if the follower is ṇit (`Tag::Rit`; ñit has no carrier yet) and the
   aṅga's upadhā is `a`, lengthen it. `laq` → `lAq`; declines on `Bakz`
   (upadhā `k`).
5. **7.3.86** *pugantalaghūpadhasya ca*, pre-ṇic arm: a second `Rule`
   entry under id `7.3.86`, as the tanādi vikalpa arm already is. Guarded on
   the ṇic term being present; same laghu-ik upadhā test as the guṇa-stage
   entry. `cur` → `cor`; declines on `BUz` (long upadhā) and `Bakz`. The
   existing guṇa-stage entry is untouched and stays inert on curādi, because
   after the merge the aṅga (`cori`) is vowel-final and its
   "final-vowel aṅgas are 7.3.84's business" early return fires.
6. **3.1.32** *sanādyantā dhātavaḥ*: append the ṇic text to `ANGA`
   (`cori`, `lAqi`, `Bakzi`, `BUzi`), remove the ṇic term so the layout is
   `[AGAMA, ABHYASA, ANGA]` again — exactly what `samjna` expects — and tag
   ANGA `Tag::Nijanta`. ANGA keeps `Tag::Dhatu`.

Nothing downstream changes. The aṅga is an i-final dhātu; 7.3.84 guṇates it
before pit śap and 6.1.78 makes `ay`.

### What the merge costs

The root/ṇic boundary is gone after 3.1.32. Any later rule that needs "this
final i is ṇic" reads `Tag::Nijanta` instead. None exists in 10a; 6.4.51
*ṇer aniṭi* is the first, in an ārdhadhātuka-lakāra slice.

### Hazards, and the check for each

- **Text guards on ANGA now meet ṇijanta stems.** A rule matching a specific
  root by `text == …` could match `cori`, `lAqi`, `Bakzi`, `BUzi`. The
  corpus-wide fires-only-on-rows test and the prior-trace diff (Tests) catch
  a misfire in either direction.
- **Upadhā reads see the ṇijanta's upadhā** (`r` in `cori`), not the root's.
  After 3.1.32 that is what the śāstra wants; the trace diff and goldens are
  the check.
- **Exhaustive `match` on `Gana`** in other crates (analyzer, CLI) must gain
  the arm; the compiler finds them. The audit harness's `gana_name` is outside
  the workspace and needs the arm by hand.
- **Duplicate ids shadow unit-test lookups.** 1.3.9 now occurs twice and
  7.3.86 three times, and `sanadi` runs first, so a stage test that finds its
  rule with `rules().find(…)` gets the sanādi entry. Found by the plan's
  prototype: seven guard tests failed and others passed by testing the wrong
  entry. Stage tests look rules up in their own stage's static (`GUNA`,
  `SAMJNA`, `TIN`), and AGENTS.md records the rule.

## Pada — 1.3.74 *ṇicaś ca*

**Data.** A new `PadaAssignment::Nic`: both padas derive, the ātmanepada arm
sanctioned by 1.3.74. Its doc comment follows `UbhayapadaAnavane`'s: this
variant must never reach 1.3.72. `derive` maps it to `Tag::Nic` on ANGA — the
pada licence — kept distinct from `Tag::Nijanta`, the stem's shape.

The pada is row-driven, not computed from ṇic's presence, so the data layer
stays the single source of pada truth across all five variants. When the
causative slice lands, 1.3.74 should key on `Tag::Nijanta` instead; the
variant's doc comment names that slice as the trigger.

**Rule.** 1.3.74 in `samjna.rs`, between 1.3.72 and 1.3.78 (sūtra order), as
1.3.66's structural twin:

- `ctx.pada == Atmanepada` and ANGA has `Tag::Nic`: fires, records
  `"Ricaś ca"`;
- parasmaipada: declines, and 1.3.78 *śeṣāt* sanctions it unchanged, as
  vidyut credits it;
- 1.3.78's ātmanepada-blocking branch learns `Tag::Nic` alongside
  `Ubhayapadin` and `Anavane`.

1.3.12 and 1.3.72 stay silent on these rows; neither tag is present.

## Data

`crates/panini-data/src/lib.rs`: `Gana::Curadi`; `PadaAssignment::Nic`; four
`DHATUS` rows (`dhatupatha`, `code`, `gana: Gana::Curadi`, `pada:
PadaAssignment::Nic`, `artha` from the TSV). `dhatupatha_numbers_resolve_upstream`
checks the four against the vendored TSV as for every row. `derive` gains
`Gana::Curadi => t.add(Tag::Curadi)` and the `Nic` pada arm.

## Tests

Counts are the probe's expectation; the plan re-derives them from vidyut's
output before writing any golden.

- **Goldens** — new `crates/panini/tests/paradigm/data/curadi.rs`, 288 cells
  (4 roots × 2 padas × 4 lakāras × 9), transcribed from vidyut's probe output.
  **ALTERNATES gains 24 rows over 16 cells**, all parasmaipada, per root: laṅ
  prathama eka (*-at* / *-ad*), loṭ prathama eka (*-atAt* / *-atAd* /
  *-atu*), loṭ madhyama eka (*-atAt* / *-atAd* / bare *-a*), vidhiliṅ
  prathama eka (*-et* / *-ed*). Keyed by the engine's own log ∩
  `VIKALPA_RULES`; no new vikalpa rule. √cur's keys carry its mandatory
  sanādi 7.3.86 in front (`7.3.86+8.4.56`, `7.3.86+7.1.35`,
  `7.3.86+7.1.35+8.4.56`). This is the id it shares with tanādi's optional
  entry, the same artifact the 3e/3f keys already document.
- **Pada-ambiguous surfaces:** the four roots are thematic and ubhayapadī, so
  each adds √nī's four collisions (`acorayata`, `corayatAm`, `corayetAm`,
  `corayeta`): 56 → 72.
- **Trace pins** — new `crates/panini/tests/trace/curadi.rs`:
  - *corayati*: 3.1.25 → 3.4.114 → 7.3.86 → 3.1.32 → 1.3.78 → 3.1.68 →
    7.3.84 → 6.1.78;
  - *corayate*: 1.3.74, no 1.3.78, no 1.3.72;
  - *lAqayati*: 7.2.116, no 7.3.86;
  - *Bakzayati*, *BUzayati*: neither 7.2.116 nor 7.3.86;
  - *BakzayARi*: 8.4.2;
  - *acorayat*: 6.4.71 on the merged aṅga.
- **Guard unit tests** in `derivation_tests.rs`, slice-7 style:
  - 3.1.25 fires only under `Tag::Curadi`;
  - ṇic's it-stripping leaves `i` tagged `Rit`, and records 1.3.9 once;
  - 7.2.116 fires on `laq` before ṇic; declines on `Bakz`, and with no ṇit
    follower;
  - the pre-ṇic 7.3.86 fires on `cur` before ṇic; declines on `BUz`, and
    without ṇic;
  - 3.1.32 leaves the layout `[AGAMA, ABHYASA, ANGA]` and tags `Nijanta`;
  - 1.3.74 opens ātmanepada only for `Nic` rows; 1.3.78 still blocks
    ātmanepada for a plain parasmaipada row; 1.3.72 never fires on a `Nic`
    row.
- **Rule order:** `tinanta_rule_order_is_pinned` gains the `sanadi` stage
  and 1.3.74 at their positions.
- **Corpus-wide fires-only-on-rows test**, using `credited`, which moves from
  `trace/juhotyadi.rs` to `trace/helpers.rs`: 3.1.25, 3.4.114,
  7.2.116, 3.1.32 and 1.3.74 are credited only on the four `10.x` rows; the
  guṇa-stage 7.3.86's credit count off curādi equals its count on `main`.
- **Prior traces:** diff every prior cell's trace between `main` and the
  slice HEAD. The 4644 prior cells stay byte-identical, traces included —
  the guard on the new stage's inertness.
- **Allow-list:** records curādi open at **4 of 509**.
- **Spot check** in `paradigm/main.rs`: *corayati*, *lAqayate*,
  *BakzayARi*.

## Audit

Copy the committed `tools/audit/panini_full_audit.rs`, never rewrite it.
Bump its invariants to **107 roots, 4932 cells (548 blocks × 9), 6062 forms
(4932 + 1130 ALTERNATES rows)**, with
`derivation_set_shape_matches_the_audited_numbers` raised to match. Repoint
`/tmp/vidyut-full`'s dev-deps at the slice worktree before running and
restore them after. Zero difference against `8da2f90b` is required, with the
mis-resolved-root negative control shown failing first.

## Mutation gate

Re-measure the uncaught-suite floor at `-j 4` and set the cap from it (6× the
floor, rounded up to the next 10 s). Copy `outcomes.json` durably before any
further invocation. Expected new non-caught mutants: none —

- 3.1.25's tag guard is killed by every curādi golden and by the inertness
  diff;
- 7.2.116's upadhā and ṇit tests by *lAqayati* and the `Bakz` decline;
- the pre-ṇic 7.3.86's laghu test by *BUzayati*, its ṇic guard by the
  without-ṇic unit case;
- 3.1.32's term removal by every curādi golden (a surviving ṇic slot shifts
  śap off index 3);
- 1.3.74's pada test by the ātmanepada goldens and the 1.3.78 block test.

AGENTS.md names the non-caught set verbatim.

## Doc sweep

README, AGENTS.md and `docs/ARCHITECTURE.md`:

- the both-pada root count 26 → 30 (README, ARCHITECTURE, the audit harness);
  pada-ambiguous surfaces 56 → 72;
- 103 → 107 roots; 4644 → 4932 cells; 5750 → 6062 forms; ALTERNATES
  1106 → 1130; the multi-form census (16 more multi-form cells: eight
  two-form — laṅ and vidhiliṅ prathama eka — and eight three-form — loṭ
  prathama and madhyama eka);
- nine gaṇas, nine complete → **ten gaṇas, nine complete, curādi open at 4
  of 509**;
- eight → nine pipeline stages;
- the 3f3 spec's "out of scope: … curādi" gains a pointer to this spec.

The grep covers wrapped counts, spelled-out numbers and rule-scoped counts;
the root-shape literals (`cur`, `cor`, `cori`, `laq`, `lAqi`, `Bakzi`,
`BUzi`); every "eight stages" / "eight ordered" phrasing; every "nine gaṇas"
and "every gaṇa" claim. Recorded file:line anchors are re-grepped at final
HEAD.

## Success criteria

- 288 new cells match vidyut, all 4644 prior cells and their traces
  byte-identical.
- The audit reports zero differences over 107 roots / 4932 cells / 6062
  forms, after its negative control fails.
- The mutation campaign's non-caught set is unchanged from 3f3's, named in
  AGENTS.md.
- `mise run test` and clippy are clean.

## Later slices

10b onward take the out-of-scope classes in roughly ascending machinery:
ātmanepadī roots, mit roots, adanta roots, then optional ṇic. (10b took the
ātmanepadī class as the ākusmīya roots and gaṇasūtra 10.0496 — see
`2026-10-02-curadi-gana-10b-design.md`.) The causative
and the ārdhadhātuka lakāras build on `sanadi` and `Tag::Nijanta`.
