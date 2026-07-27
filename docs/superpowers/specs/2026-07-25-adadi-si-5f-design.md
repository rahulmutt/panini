# Adādi √śī, closing the gaṇa (Phase 2, slice 5f)

**Status:** Design, approved in brainstorming 2026-07-25.

Builds on `2026-07-22-adadi-gana-design.md` (the adādi roadmap),
`2026-07-25-adadi-as-atmanepada-5d-design.md` (slice 5d, √ās) and
`2026-07-24-adadi-vas-dhi-ca-5e-design.md` (slice 5e, √vas + the 8.2.25
correction). Corrects two claims in the 5e spec and one row of the roadmap.

## Summary

Add **√śī** (śīṅ, adādi, ātmanepada, *svapne*) — the last root of the curated
adādi set and the only irregular one — closing the gaṇa. Roots 29 → **30**;
golden paradigm 1044 → **1080**.

√śī is the slice where the gaṇa's *guṇa* story finally appears. Every other
adādi root either has no *ik* to guṇate (√yā, √vā, √ās) or is consonant-final
(√ad, √vas), and on the ātmanepada side the ṅit endings block guṇa via 1.1.5.
√śī overrides that block by **7.4.21 *śīṅaḥ sārvadhātuke guṇaḥ***, which is
exactly what makes *śete* appear, and it carries a second root-specific
augment, **7.1.6 *śīṅo ruṭ*** (*śerate*).

It also forces one rule the roadmap did not anticipate: **8.3.59
*ādeśapratyayayoḥ*** (ṣatva), the engine's first retroflexion rule. With the
aṅga guṇated to `Se`, the `se` and `sva` endings meet a non-`a`/`ā` vowel and
retroflex → **`Seze`**, **`Sezva`**. No earlier root could reach this: √ās
ends in `A`, √vas in `s`, and every thematic root puts the vikaraṇa's `a`
before the ending.

Three new rules, one new root, no change to any existing rule's behaviour
(this later proved false for 6.1.78 — see the dated Correction below).

## Scope

Unchanged from slice 5e: single *pada*, *tiṅanta*, four lakāras (laṭ, laṅ,
loṭ, vidhiliṅ), both padas, all nine *puruṣa* × *vacana* cells.

New in this slice: one root (√śī) and three rules (7.4.21, 7.1.6, 8.3.59),
plus one test-only item — retiring the parked 7.3.100 mutant (below).

Out of scope and unchanged: the irregular aluk stars (√as, √han, √brū, √i,
√dviṣ / √duh / √lih), ubhayapadī roots and the intent-conditioned pada rules,
other gaṇas, other lakāras. Adādi is complete after this slice; the next
roadmap item is a different gaṇa or a different axis, not more adādi.

Also deliberately **not** in this slice, by explicit decision:

- **Splitting `tinanta.rs`** (3644 lines, and it grows again here). Rule
  *order* is the grammar, so a split belongs in its own slice where the trace
  tests are the only thing under scrutiny.
- **A committed vidyut-prakriya cross-check harness.** Adding vidyut as a
  dev-dependency is a supply-chain decision with its own gate. This slice gets
  the same benefit from a *scratch* run (see Verification method).

## The paradigm

All 36 forms, verified against vidyut-prakriya before any code was written
(see Verification method). vidyut reports **no optional variants** for any
cell, so each is a single expected string.

| lakāra | | eka | dvi | bahu |
|---|---|---|---|---|
| laṭ | prathama | `Sete` | `SayAte` | `Serate` |
| | madhyama | `Seze` | `SayATe` | `SeDve` |
| | uttama | `Saye` | `Sevahe` | `Semahe` |
| laṅ | prathama | `aSeta` | `aSayAtAm` | `aSerata` |
| | madhyama | `aSeTAH` | `aSayATAm` | `aSeDvam` |
| | uttama | `aSayi` | `aSevahi` | `aSemahi` |
| loṭ | prathama | `SetAm` | `SayAtAm` | `SeratAm` |
| | madhyama | `Sezva` | `SayATAm` | `SeDvam` |
| | uttama | `SayE` | `SayAvahE` | `SayAmahE` |
| vidhiliṅ | prathama | `SayIta` | `SayIyAtAm` | `SayIran` |
| | madhyama | `SayITAH` | `SayIyATAm` | `SayIDvam` |
| | uttama | `SayIya` | `SayIvahi` | `SayImahi` |

Three shapes of aṅga appear across the table and account for every cell:

- **`Se`** before a consonant-initial ending (7.4.21 guṇa only) — `Sete`,
  `SeDve`, `Serate`.
- **`Say`** before a vowel-initial ending (7.4.21, then 6.1.78 *eco'yavāyāvaḥ*
  turns `e` → `ay`) — `SayAte`, `Saye`, `SayIta`, `SayE`.
- **`Se` + retroflexed ending** where the ending begins `s` — `Seze`, `Sezva`.

## Grammar

### ① New: 7.4.21 *śīṅaḥ sārvadhātuke guṇaḥ* — guṇa despite the ṅit ending

Root-specific. √śī takes guṇa (`SI` → `Se`) before a sārvadhātuka ending even
though the ātmanepada endings are ṅit by 1.2.4 and 1.1.5 would otherwise block
it.

Implemented as its own self-guarding `Rule` in the guṇa band, ordered
**immediately before 7.3.84**. 7.3.84 then declines by itself — its target
must be *ik*-final and `Se` is not — so 7.3.84's existing 1.1.5 guard is not
touched. This keeps one `Rule` per sūtra, as AGENTS.md requires, and keeps the
trace honest: the guṇa is credited to 7.4.21, which is the sūtra that
licenses it.

**Guard:** aṅga text **`ends_with("SI")`** — that single clause, and no gaṇa
tag. A `Tag::Adadi` clause would be redundant (√śī is the only `SI`-final root)
*and* unkillable: under the `||`→`&&` mutant the rule would proceed for the
other adādi roots, but their aṅga-finals (`d`, `A`, `s`) have no guṇa
substitute, so `guna_of` returns `None` and no form changes. It also matches
the sūtra, which names śīṅ rather than the gaṇa. The substitution itself reuses
the existing `guna_of` helper, whose arms are already table-tested.

The sūtra's *sārvadhātuke* condition is structurally satisfied and is
deliberately **not** written as a guard clause: every tiṅ ending in this
engine's scope is tagged `Tag::Sarvadhatuka` when it is introduced (3.4.78 /
3.4.113), across all four lakāras, so the clause would be always-true — and an
always-true clause is precisely what leaves a `&&`→`||` mutant unkillable.
7.3.84 omits the same check for the same reason. Record the condition in the
doc comment instead, and note that it must become a real guard the moment an
ārdhadhātuka affix enters scope.

`ends_with`, not `==`: 6.4.71 *luṅlaṅlṛṅkṣvaḍudāttaḥ* runs earlier in the
array and prepends the aṭ augment directly to the aṅga's text, so in laṅ the
aṅga is `aSI` when this rule executes. (vidyut keeps the augment as a separate
term — `a + SI` — but our engine does not, and the guard must match our term
layout, not vidyut's.)

Identifying the root by text shape is the codebase's established idiom for a
root-specific rule; 8.2.77 *hali ca* self-guards the same way. A purely
phonological guard ("adādi ∧ `I`-final") would coincide with √śī in today's
root set but would misstate the sūtra, which names śīṅ.

**Rejected alternative:** a √śī exception inside 7.3.84's 1.1.5 guard. Fewer
moving parts, but the trace would then credit 7.3.84 with guṇa that 7.4.21
licenses, and the trace tests are the source of truth for rule attribution.

### ② New: 7.1.6 *śīṅo ruṭ* — the ruṭ augment of the 3pl *jha*

Root-specific. The *jha* (3pl ātmanepada) of √śī takes the *ruṭ* augment:
`Se + ate` → `Se + r + ate` → **`Serate`**.

Ordered **immediately after 7.1.5 *ātmanepadeṣv anataḥ***, which is the rule
that turns the leading `J` into `at` (7.1.5 fires for √śī because the segment
before the ending is `I`, not short `a`). 7.1.6 prepends `r` to that `at…`:
`ate` → `rate` (laṭ), `ata` → `rata` (laṅ), `atAm` → `ratAm` (loṭ).

**Guard:** aṅga is √śī (the same single `ends_with("SI")` test, no gaṇa tag) ∧
**7.1.5 fired in this derivation**, read from `p.log`. Both clauses
discriminate: under the `||`→`&&` mutant the rule would prepend `r` to √ās's
3pl ending and yield \*`Asrate`, which the shipped golden rejects. Reading the trace for a prior rule is an
established idiom here — 6.4.72 *āḍ ajādīnām* uses exactly this to test
whether 6.4.71 already augmented the aṅga. It is also grammatically exact: the
*ruṭ* attaches to the `at` that 7.1.5 produced, so predicating on 7.1.5 is the
condition, not a proxy for it.

This is why **vidhiliṅ needs no special case**: 3.4.105 *jhasya ran* replaces
the *jha* with `ran` far earlier in the array, so 7.1.5 never fires there,
so 7.1.6 cannot → **`SayIran`**, not \*`SayIraran`. Confirmed in vidyut's
trace for that cell.

### ③ New: 8.3.59 *ādeśapratyayayoḥ* — ṣatva, the engine's first retroflexion

General grammar, not a √śī special. The `s` of an ādeśa or pratyaya, when not
word-final, retroflexes to `ṣ` after *iṇ-koḥ*. Fires for exactly two cells:

- laṭ 2sg: `Se` + `se` → **`Seze`**
- loṭ 2sg madhyama: `Se` + `sva` → **`Sezva`**

Placed in the tripādī block **between 8.3.15 and 8.4.55**, following the
block's numeric ordering. No conflict with 8.3.15
*kharavasānayor visarjanīyaḥ*, which is word-final while 8.3.59 is
*apadāntasya*.

**Guard (narrow, by decision):** the ending's first character is `s` ∧ the
aṅga's final character is a vowel other than `a`/`A`. That is precisely the
reachable condition in the current grammar, so every arm of the rule is
executed by a test and the mutation gate stays clean — the same discipline
under which 5e deleted 8.4.53 and dropped 6.1.78's E/O arms as unreachable.

The doc comment must state the full scope the guard does **not** implement —
the rest of the *iṇ* pratyāhāra (`h y v r l` and the remaining vowels) and
`k` — and name the restore trigger: the first root that puts one of those
before an `s`-initial affix. This mirrors how 8.2.25's deliberately narrow
guard is documented.

The rule declines correctly for every existing root without needing to know
about them: √ās's aṅga ends in `A` (excluded), √vas's in `s` (not a vowel),
and every thematic root presents the vikaraṇa's `a` (excluded). Verified
against the shipped golden — `Asse`, `Assva`, `vasse`, `vassva` all reach the
tripādī with the `s` intact and must stay unchanged.

### Unchanged rules that carry √śī

- **6.1.78 *eco'yavāyāvaḥ*** — the `e` → `ay` arm is live and already
  exercised by √ji (*jayati*); it needs no edit to produce `SayAte`,
  `SayIta`, `SayE`. Only the E/O (vṛddhi) arms were dropped in 5e, and √śī
  does not revive them (see Corrections).
- **3.4.105 *jhasya ran*** — already ordered ahead of the 7.x band; it is what
  keeps ruṭ out of vidhiliṅ.
- **2.4.72, 3.1.68, 7.1.5, 7.2.79, 6.1.66, 6.1.90** — all unchanged, but
  6.1.66 and 6.1.90 are the two flagged as at-risk; see Risks.

> **Correction (2026-07-25, slice 5f).** The claim above that 6.1.78 "needs
> no edit" is false and was falsified during implementation. 6.1.78's live
> arm reads `p.terms[SHAP]` for the vowel that follows the aṅga, but adādi's
> **2.4.72** *empties* that term (the śap is luk'd), so for √śī the arm
> always declined — producing `SeIran` / `SeAte`, not `SayIran` / `SayAte`.
> √ji's thematic path never exercises the śap-luk'd case, so this gap was
> invisible until √śī. The fix (this slice) is an **athematic arm**: when
> `p.terms[SHAP].text.is_empty()`, the rule falls back to
> `p.terms[ENDING]`'s first character instead. The two arms' guards
> (SHAP vowel-initial vs. SHAP empty) are mutually exclusive by
> construction, so at most one ever fires — provably disjoint from the
> thematic arm above it.

### Rule order (delta only)

```
      7.1.5  AtmanepadezvanataH
[new] 7.1.6  SINo ruw                     (√śī ∧ 7.1.5 fired → Se + r + ate)
      7.1.3  Jo'ntaH
      …
[new] 7.4.21 SINaH sArvaDAtuke guRaH      (√śī: guṇa despite the ṅit ending)
      7.3.84 sArvaDAtukArDaDAtukayoH      (unchanged; declines on Se)
      7.3.86 pugantalaGUpaDasya ca
      6.1.78 eco'yavAyAvaH                (Se + A → Say; gained an athematic
                                            arm this slice — see the dated
                                            Correction above)
      …
      8.3.15 KaravasAnayor visarjanIyaH
[new] 8.3.59 AdeSapratyayayoH             (tripādī ṣatva: Se + se → Seze)
      8.4.55 Kari ca
```

`TINANTA_RULES` gains three entries. All three are guarded on √śī or on a
junction no existing root reaches, so no pre-existing golden or trace output
can change.

## Data layer

Mechanical, following the 5d/5e pattern:

- One `Dhatu` entry in `panini-data`'s curated array: code `SI`,
  `Gana::Adadi`, `Pada::Atmanepada`, artha `svapne`.
- The mirrored row in `data/dhatupatha.tsv`: `SI	adadi	atmanepada	svapne`.
- The registry test extended to assert √śī is adādi ∧ ātmanepada, alongside
  the existing √ās / √vas assertions.

`Gana::Adadi` and the pada column already exist. No structural change to
`Dhatu`.

## Analyzer, facade, CLI

No code change, by construction. `panini-analyze` brute-forces every
(root × lakāra × cell) and lets the engine confirm by exact match; the
candidate set grows 1044 → 1080. The `panini` facade and `panini-cli check`
output shapes are unchanged. `panini-lipi` is untouched: `z` and every other
character in the new forms is already supported, so the Devanāgarī / IAST / HK
round-trips come free.

## Verification method

The golden block is written from **vidyut-prakriya**, not from our engine.
This is the mechanical cross-check that slice 5e concluded would have caught
the 5d error, taken without adding a dependency to this repo:

1. Clone `ambuda-org/vidyut` into the session scratchpad (not the repo).
2. Add a throwaway `examples/si_dump.rs` that iterates √śī (`SIN`, `Adadi`)
   over 4 lakāras × 3 puruṣa × 3 vacana, printing each cell's derived form(s)
   and full rule history.
3. Run it with the repo's pinned toolchain:
   `mise exec rust@1.97.1 -- cargo run --release --example si_dump`.
4. Transcribe the forms into the golden block and the histories into the trace
   expectations.

The run is reproduced in the appendix. Independently corroborating: vidyut's
own `tests/integration/kashika_7_1.rs::sutra_7_1_6` pins `Serate` / `SeratAm`
/ `aSerata`, and `kashika_7_4.rs::sutra_7_4_21` pins `Sete` / `SayAte` /
`Serate`; the sūtra id/name strings for 7.1.6, 7.4.21, 6.1.78, 2.4.72 and
3.4.105 were read from `data/sutrapatha.tsv`.

**If the engine disagrees with a golden, the engine is wrong.** Editing a
golden to match engine output is the exact failure mode that shipped the 5d
error, and it is forbidden here.

## Retiring the parked 7.3.100 mutant

Slice 5e parked a surviving `||`→`&&` mutant on 7.3.100 *adaḥ sarveṣām*'s
guard line and recorded a two-case analysis concluding it was unkillable. That
analysis is wrong, and the mutant is killable. Both are fixed here.

**What actually happens.** Under the mutant the guard becomes
`!Lan && !Adadi`, so for a **laṅ, non-adādi** derivation the rule no longer
declines. Its inner checks do *not* exclude that case: at the point 7.3.100
runs, `ANGA` still holds the bare root (guṇa is later), so a consonant-final
root like √kup passes the `is_vowel(anga_last)` test, and the laṅ 3sg ending
has already been reduced to a single `t` by 3.4.100 *itaś ca*. The mutated
rule therefore fires and produces `akupya` + `at`. The form survives only
because **6.1.97 *ato guṇe*** then merges `a + a` back to `a`, repairing
`akupyaat` → `akupyat`. It is masked by a downstream repair, not excluded by
the inner checks.

Verified by applying the mutation and tracing:

```
6.4.71 luNlaNlfNkzvaqudAttaH -> akupyat
7.3.100 adaH sarvezAm        -> akupyaat     <-- only under the mutant
6.1.97 ato guRe              -> akupyat
```

**The kill.** An ordered-trace pin on **`akupyat`** fails under the mutant,
because the mutated derivation records two extra steps. No grammar change is
needed — the correct engine already declines 7.3.100 there. Slice 5f adds that
trace pin, deletes the parked-mutant comment at `tinanta.rs:867`, and returns
the mutation gate to **zero** survivors.

√śī itself supplies no distinguishing case: its aṅga is `SI`, vowel-final, at
the point 7.3.100 runs, so the `is_vowel` check excludes it. The 5e
prediction that √śī might resolve this is superseded by the trace pin, which
resolves it independently of any root.

## Corrections to the 5e spec and the roadmap

Dated corrections to record, in the same style 5e used for 5d:

1. **5e's 7.3.100 case analysis** (`2026-07-24-adadi-vas-dhi-ca-5e-design.md`,
   "Parked mutant" section) — Case A's claim that the aṅga is always
   vowel-final for non-adādi roots is false: `ANGA` holds the bare root and
   the vikaraṇa is a separate term, so √kup reaches the guard consonant-final
   and the mutated rule does fire. Case B's stated reason is also wrong —
   3.4.103 is *earlier* in the array than 7.3.100, not later — though its
   conclusion survives, since the yāsuṭ-bearing ending is multi-character by
   the time 7.3.100 runs and the length check excludes it either way. The
   survivor's real cause is 6.1.97's repair, and it is killable. Superseded
   here.
2. **5e's E/O restore trigger.** The comment above 6.1.78 and the 5e spec name
   √śī as the leading candidate for restoring the dropped E/O (`Ay`/`Av`)
   arms. It is not: 7.4.21 gives √śī *guṇa* (`Se`), never vṛddhi, so its aṅga
   is never `E`/`O`-final. The restore trigger is a root needing vṛddhi at the
   aṅga-final position before a vowel-initial ending; the recorded restore
   *shape* (an extracted `ayadi_of` helper with an exhaustive table test)
   stands.
3. **The roadmap's √śī row**
   (`2026-07-22-adadi-gana-design.md`) — it omits 8.3.59 from √śī's rule list,
   and gives 7.4.21's name as `SINaH [guNaH]`; the reference string is
   `SINaH sArvaDAtuke guRaH`. Corrected by a dated footnote, as 5e did for the
   √vas row.

## Testing

- **Golden paradigm** (`crates/panini/tests/paradigm.rs`): a 36-form √śī block
  (4 lakāras × 9 cells), total 1044 → **1080**. Forms as in The paradigm
  above.
- **Ordered trace** (`crates/panini/tests/trace.rs`), five √śī pins chosen so
  every new rule and every at-risk shared rule is covered by at least one:
  - `Sete` — 2.4.72 luk → 7.4.21 guṇa, the minimal √śī trace;
  - `Serate` — 7.1.5 → 7.1.6 ruṭ → 7.4.21;
  - `Seze` — 8.3.59 ṣatva;
  - `SayIta` — 7.2.79 → 6.1.66 → 7.4.21 → 6.1.78, the ruṭ-free vidhiliṅ path;
  - `SayE` — 6.1.90's āṭ arm together with 6.1.78.

  Note: the orderings above are vidyut's own rule sequence (see the Appendix),
  not this engine's. Our shipped `TINANTA_RULES` array order differs at both
  junctions — `SayIta` runs 7.2.79 → 7.4.21 → 6.1.78 → 6.1.66, and `SayE`
  runs 7.4.21 → 6.1.78 → 6.1.90 where vidyut runs 7.3.84 → 7.4.21 → 6.1.90 →
  6.1.78 — though the surface outputs agree. In this repo rule order IS the
  grammar, so the pins in `crates/panini/tests/trace.rs`
  (`shayita_trace_is_the_shing_vali_lopa_path`, `shayai_trace_is_the_shing_atas_ca_path`)
  are authoritative, not the orderings written here or in the Appendix.
- **One non-√śī trace pin**: `akupyat`, which kills the 7.3.100 mutant (above).
- **Negatives** (`known_nonforms_are_invalid`), each chosen to kill a specific
  mutated guard:
  - `SIte` — guṇa not applied (kills a mutated 7.4.21 guard);
  - `Sese` — ṣatva not applied (kills a mutated 8.3.59 guard);
  - `Seate` — ruṭ not applied (kills a mutated 7.1.6 guard);
  - `SayIraran` — ruṭ wrongly applied in vidhiliṅ (kills a 7.1.6 guard that
    ignores 7.1.5);
  - `Sayati` — wrong pada (parasmaipada cross);
  - `Sayate` — the śap surviving 2.4.72 (`SI` + `Sap` + `te`, guṇated) — kills
    a mutated 2.4.72 guard for this root;
  - `SIyate` — a divādi/tudādi-style vikaraṇa leaking into adādi.
- **Rule-level guard-boundary tests** in `tinanta.rs` for all three new
  guards, following the `awas_ca_athematic_*` / `lopo_vyor_vali_athematic_*`
  pattern: for each guard, one case just inside and one just outside.
- **Mutation testing**: `mise run mutants` at slice end, expected at **zero**
  survivors. Run the binary directly rather than through the shim if the task
  fails in a background shell.

`panini-lipi` roundtrip/property tests are untouched.

## Risks

1. **Athematic arms on shared rules — the main risk.** √śī is the first root
   whose aṅga is `e`-final at the 6.1.90 *āṭaś ca* and 6.1.66
   *lopo vyor vali* junctions. Slice 5d proved that rules written for the
   thematic path silently fail on the śap-luk'd path, producing non-words
   (`āsāai`, `āsīyta`) until given an explicit athematic arm. **Mitigation:**
   the plan's first task after landing the three new rules is to derive all
   36 cells against the engine and diff against the table above. Any
   mismatch means a shared rule needs an athematic arm — provably disjoint
   from its thematic arm, and pinned with guard tests. It never means the
   golden is wrong.
2. **8.3.59's narrow guard.** It implements the reachable slice of *iṇ-koḥ*,
   not the whole condition. Named in the doc comment with its restore
   trigger, exactly as 8.2.25's narrow guard is. A later slice with an
   `h`/`y`/`v`/`r`/`l`- or `k`-final aṅga before an `s`-initial affix must
   widen it.
3. **Root identity by text shape.** `ends_with("SI")` is correct for the
   current term layout and root set. Upasarga-prefixed roots (*praśī*) are
   out of scope; if they land, root identity needs to become a `Term` field
   rather than a text test, and 8.2.77 has the same exposure.
4. **`tinanta.rs` grows again** (3644 lines before this slice). Flagged, not
   fixed — see Scope.

## Success criteria

- All **1080** golden forms validate `VALID` with correct ordered traces; the
  1044 pre-existing forms are unchanged.
- The seven √śī non-forms return `INVALID`.
- No existing golden or trace output changes — every new rule is guarded on
  √śī or on a junction no existing root reaches.
- Every sūtra id and name in a trace matches the reference (vidyut's
  `sutrapatha.tsv`).
- `mise run mutants` reports **zero** survivors, including the previously
  parked 7.3.100 mutant, and the park comment is gone.
- Adādi is complete: 6 roots, 216 forms, gaṇa closed.

## Appendix: vidyut-prakriya run

Generator: scratchpad clone of `ambuda-org/vidyut`, `examples/si_dump.rs`,
`mise exec rust@1.97.1 -- cargo run --release --example si_dump`.

Forms (lakāra, puruṣa, vacana → form):

```
law     prathama eka  => Sete        law     prathama dvi  => SayAte
law     prathama bahu => Serate      law     madhyama eka  => Seze
law     madhyama dvi  => SayATe      law     madhyama bahu => SeDve
law     uttama   eka  => Saye        law     uttama   dvi  => Sevahe
law     uttama   bahu => Semahe
laN     prathama eka  => aSeta       laN     prathama dvi  => aSayAtAm
laN     prathama bahu => aSerata     laN     madhyama eka  => aSeTAH
laN     madhyama dvi  => aSayATAm    laN     madhyama bahu => aSeDvam
laN     uttama   eka  => aSayi       laN     uttama   dvi  => aSevahi
laN     uttama   bahu => aSemahi
low     prathama eka  => SetAm       low     prathama dvi  => SayAtAm
low     prathama bahu => SeratAm     low     madhyama eka  => Sezva
low     madhyama dvi  => SayATAm     low     madhyama bahu => SeDvam
low     uttama   eka  => SayE        low     uttama   dvi  => SayAvahE
low     uttama   bahu => SayAmahE
viDiliN prathama eka  => SayIta      viDiliN prathama dvi  => SayIyAtAm
viDiliN prathama bahu => SayIran     viDiliN madhyama eka  => SayITAH
viDiliN madhyama dvi  => SayIyATAm   viDiliN madhyama bahu => SayIDvam
viDiliN uttama   eka  => SayIya      viDiliN uttama   dvi  => SayIvahi
viDiliN uttama   bahu => SayImahi
```

Key histories (vidyut's own rule sequence). This is NOT the same sequence our
trace pins use, restricted or otherwise: for `SayIta` and `SayE` our shipped
`TINANTA_RULES` array orders 7.4.21 relative to 6.1.66/6.1.78/6.1.90/7.3.84
differently than vidyut does below (see the note in Testing above for the
specifics), though the surface outputs agree. The pins in
`crates/panini/tests/trace.rs` are authoritative for this engine's order;
treat the sequences below as vidyut's derivation history only:

```
Sete       3.4.78 SI+ta | 3.1.68 SI+Sap+ta | 2.4.72 SI+ta | 3.4.79 SI+te
           | 1.2.4 | 1.1.5 | 7.4.21 Se+te

Serate     3.4.78 SI+Ja | 3.1.68 SI+Sap+Ja | 2.4.72 SI+Ja | 3.4.79 SI+Je
           | 1.2.4 | 7.1.5 SI+ate | 7.1.6 SI+ru~w+ate | 1.3.9 SI+r+ate
           | 1.1.5 | 7.4.21 Se+r+ate

Seze       3.4.78 SI+TAs | 3.1.68 SI+Sap+TAs | 2.4.72 SI+TAs | 3.4.80 SI+se
           | 1.2.4 | 1.1.5 | 7.4.21 Se+se | 8.3.59 Se+ze

SayIta     3.4.78 SI+ta | 3.1.68 | 2.4.72 SI+ta | 3.4.102 SI+sIyu~w+ta
           | 1.3.9 SI+sIy+ta | 3.4.107 SI+sIy+sta | 1.2.4
           | 7.2.79 SI+Iy+ta | 6.1.66 SI+I+ta | 1.1.5 | 7.4.21 Se+I+ta
           | 6.1.78 Say+I+ta

SayIran    … 3.4.105 SI+sIy+ran | 7.2.79 SI+Iy+ran | 6.1.66 SI+I+ran
           | 7.4.21 Se+I+ran | 6.1.78 Say+I+ran      (no 7.1.5, so no ruṭ)

SayE       3.4.78 SI+iw | 1.3.9 SI+i | 2.4.72 SI+i | 3.4.79 SI+e
           | 3.4.93 SI+E | 3.4.92 SI+Aw+E | 1.3.9 SI+A+E | 7.3.84 Se+A+E
           | 7.4.21 | 6.1.90 Se+E | 6.1.78 Say+E
```
