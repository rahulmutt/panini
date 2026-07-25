# Adādi √vas + the 8.2.25 *dhi ca* correction (Phase 2, slice 5e)

**Status:** Design, approved in brainstorming 2026-07-24.

Builds on `2026-07-22-adadi-gana-design.md` (the adādi roadmap) and
`2026-07-25-adadi-as-atmanepada-5d-design.md` (slice 5d, √ās), and **corrects
a grammar error shipped in 5d**.

## Summary

Two things, deliberately in one slice because they are the same fact seen
twice:

1. **Correct slice 5d.** The engine derives √ās 2pl as `AdDve` / `AdDvam`
   (*āddhve* / *āddhvam*) by **8.4.53 *jhalāṃ jaś jhaśi*** (jaśtva). That is
   wrong. The correct forms are **`ADve` / `ADvam`** (*ādhve* / *ādhvam*),
   produced by **8.2.25 *dhi ca***: the final `s` of the term preceding a
   `dh`-initial affix is *elided*, not voiced. 8.2.25 sits earlier in the
   tripādī and bleeds 8.4.53 entirely.
2. **Add √vas** (adādi, ātmanepada, *ācchādane*) — which is the commentaries'
   stock example for 8.2.25 (*vadhve* / *vadhvam*), and therefore the natural
   second witness for the corrected rule.

√vas needs **no new grammar of its own**: 33 of its 36 cells already derive
correctly on today's engine, and the remaining three are exactly the cells
8.2.25 fixes. Its value is a generality proof for the 5c/5d junction
machinery, plus a set of negative witnesses no earlier root could reach.

## Correction to slice 5d

### What was wrong

Slice 5d introduced 8.4.53 as "the first voiced internal junction," analysing
√ās's `s` before the `dh` of `Dve` / `Dvam` as jaśtva (`s → d`). The design
flagged an open citation risk — **8.4.53 *jhalāṃ jaś jhaśi*** vs **8.2.39
*jhalāṃ jaśo'nte*** — and resolved it in favour of 8.4.53. Neither candidate
was the operative rule. The junction is governed by **8.2.25 *dhi ca***
(s-lopa before a `dh`-initial affix), which precedes both.

The error reached the golden block because the golden was written from the
engine's own output rather than from an independent reference. This is the
exact failure mode the repo already knows about: *an implementer editing a
golden to match engine output is a red flag for a masked engine bug.*

### Evidence

- **vidyut-prakriya** derives `As + Dve` → `8.2.25` → `A + Dve` → **`ADve`**;
  likewise `avaDvam`, `vaDvam`. Its implementation
  (`src/tripadi/pada_8_2.rs`, `try_lopa_of_samyoganta_and_s`) fires 8.2.25 on
  any non-pada term-final `s` before a `D`-initial term.
- **Standard paradigms** give √ās 2pl as *ā́dhve* and √vas 2pl as *vadhve*,
  not *āddhve* / *vaddhve*.
- **√vas is believed to be the sūtra's stock example** — the commentaries
  illustrate 8.2.25 with *vadhve* / *vadhvam*, the very root this slice adds.
  Confirm this at write time before repeating it in a code comment; it is
  colour, not load-bearing evidence. The load-bearing evidence is vidyut's
  derivation, whose sūtra id and name string are what the trace pins.

`dh` is indeed a jhaś (jhaŚ = *jha bha gha ḍha dha ja ba ga ḍa da*, all voiced
stops), so 8.4.53's *trigger* was correctly identified in 5d. The mistake was
ordering: 8.2 is asiddha to 8.4, so the `s` is gone before jaśtva can look at
it.

### Blast radius

Exactly three cells, all √ās: laṭ 2pl (`paradigm.rs:1092`), laṅ 2pl
(`:1099`), loṭ 2pl (`:1106`). Verified by cross-checking **every** adādi root
cell-by-cell against vidyut-prakriya — √ad, √yā and √vā match exactly. No
other root can be affected: every non-adādi gaṇa keeps an a-final vikaraṇa
between root and ending, so no other root has a consonant junction at all.

The `*AsDve` negative pin in `known_nonforms_are_invalid` remains correct
(bare `s + Dve` is still not a form). No test currently asserts `ADve` is
invalid, so no negative pin has to be reversed.

## Scope

Unchanged from slice 5d: single *pada*, *tiṅanta*, four lakāras (laṭ, laṅ,
loṭ, vidhiliṅ), both padas, all nine *puruṣa* × *vacana* cells.

New in this slice: one rule (8.2.25), one deletion (8.4.53), one root (√vas).

Out of scope and unchanged: the irregular aluk stars (√as, √han, √brū, √i,
√dviṣ / √duh / √lih), ubhayapadī roots, other gaṇas, other lakāras. √śī
remains deferred to slice 5f, which closes adādi.

## Grammar

### ① New: 8.2.25 *dhi ca* — s-lopa before a `dh`-initial affix

The final `s` of the term immediately preceding a `dh`-initial affix is
elided.

```
As + Dve   ->  A + Dve   ->  ADve      (√ās, laṭ 2pl)
vas + Dve  ->  va + Dve  ->  vaDve     (√vas, laṭ 2pl)
```

Guard shape, mirroring the existing junction rules (8.4.55):

- locate the first non-empty term after `ANGA` (skipping the luk'd śap);
  require its text to begin with `D`;
- require the last non-empty term *before* it to end in `s`;
- elide that `s`.

The second clause is load-bearing. In the vidhiliṅ the term preceding `Dvam`
is the sīyuṭ residue `I`, not the aṅga — so `AsIDvam` / `vasIDvam` keep their
`s` and the rule correctly declines. (That holds whether the sīyuṭ residue is
a separate term or has merged into the ending: either way the first non-empty
term after the aṅga does not begin with `D`.) In laṭ / laṅ / loṭ the śap is
empty, so
the term preceding the ending *is* the aṅga and the rule fires. Writing the
guard as "the term preceding the affix" rather than "the aṅga" is what makes
both cases come out right from one statement.

**Placement:** the tripādī, after 8.2.23 *saṃyogāntasya lopaḥ* and before
8.3.15. That is its natural numeric slot, and it is also what makes it bleed
the voiced junction.

### ② Deleted: 8.4.53 *jhalāṃ jaś jhaśi*

Removed, along with its `is_jhas` and `jastva_of` helpers, its ordered-trace
pin and its unit tests.

`dhvam` / `dhve` is the only jhaś-initial tiṅ ending, and 8.2.25 now consumes
every case a registered root can reach, so 8.4.53 is unreachable. It is
correct grammar, but no test can exercise it and the mutation gate — which
the repo has kept at zero survivors — cannot be satisfied for it.

**Restore trigger:** 8.4.53 returns with the first voiced-stop-final root —
√duh, √lih or √dviṣ — where the aṅga-final segment is not an `s` for 8.2.25
to eat, so jaśtva genuinely applies. Those roots are already deferred by the
parent adādi design (they carry 8.2.31 *ho ḍhaḥ* / 8.2.32 *dādho dhātoḥ* /
ṣṭutva).

`is_jhal` stays — 8.4.55 cartva still uses it.

### Rule-table delta

```
[new]     8.2.25 Di ca                  (s-lopa before Dh: As + Dve -> ADve)
[deleted] 8.4.53 JalAM jaS JaSi         (unreachable once 8.2.25 lands)
[kept]    8.4.55 Kari ca                (cartva, unchanged)
```

## Root: √vas

| root (SLP1) | gaṇa | pada | artha | vidyut entry | laṭ 3sg |
| --- | --- | --- | --- | --- | --- |
| `vas` | adādi | ātmanepada | `AcCAdane` | `02.0013 vasa~\` | **vaste** |

This is √vas *ācchādane* ("to wear", gaṇa 2, ātmanepada) — **not** the far
commoner √vas *nivāse* ("to dwell", gaṇa 1, parasmaipada, *vasati*). The
`artha` column is the only disambiguator in `dhatupatha.tsv`.

Full paradigm, cross-checked against vidyut-prakriya (rows read prathama /
madhyama / uttama):

| | eka | dvi | bahu |
| --- | --- | --- | --- |
| **laṭ** | `vaste` / `vasse` / `vase` | `vasAte` / `vasATe` / `vasvahe` | `vasate` / **`vaDve`** / `vasmahe` |
| **laṅ** | `avasta` / `avasTAH` / `avasi` | `avasAtAm` / `avasATAm` / `avasvahi` | `avasata` / **`avaDvam`** / `avasmahi` |
| **loṭ** | `vastAm` / `vassva` / `vasE` | `vasAtAm` / `vasATAm` / `vasAvahE` | `vasatAm` / **`vaDvam`** / `vasAmahE` |
| **vidhiliṅ** | `vasIta` / `vasITAH` / `vasIya` | `vasIyAtAm` / `vasIyATAm` / `vasIvahi` | `vasIran` / `vasIDvam` / `vasImahi` |

Thirty-three of the thirty-six cells already derive correctly on today's
engine. The three bolded cells currently come out as `vadDve` / `avadDvam` /
`vadDvam` and become correct when 8.2.25 lands.

What √vas contributes beyond a root count:

- **Second witness for 8.2.25**, on the root the commentaries use to state it.
- **First witnesses that cartva must *not* fire**: `vaste`, `avasTAH`,
  `vasse`, `vassva` all have an aṅga-final `s` meeting a khar (`t`, `th`,
  `s`) and must come through untouched. √ad and √ās could not reach that arm.

## Data layer (`panini-data`)

Mechanical, following the 5d pattern: one `Dhatu` entry (`code: "vas"`,
`gana: Adadi`, `pada: Atmanepada`, `artha: "AcCAdane"`) and the mirrored row
in `data/dhatupatha.tsv` (`vas  adadi  atmanepada  AcCAdane`). Root count
28 → **29**. No new storage shape. `derive` gains no branch — 8.2.25 is a
self-guarding entry in `TINANTA_RULES`, fed by the existing `Tag::Adadi`
tagging and pada threading.

Golden paradigm 1008 → **1044**.

## Testing

**Golden paradigm** (`crates/panini/tests/paradigm.rs`)

- Correct the three √ās cells in place: `AdDve` → `ADve` (:1092),
  `AdDvam` → `ADvam` (:1099, :1106).
- Add four √vas blocks, 36 forms, 1008 → 1044.
- Every cell comes from the vidyut-prakriya run, **not** from the engine's
  output. This is the discipline whose absence caused the 5d error.
- `paradigm_covers_every_enumerable_cell` picks up the new root
  automatically.

**Ordered trace** (`crates/panini/tests/trace.rs`)

- Rewrite the existing `AdDve` pin as **`ADve`**: … → `3.4.79` →
  **`8.2.25 Di ca`** (`As + Dve` → `A + Dve`). Assert the trace contains
  neither `8.4.53` (deleted) nor `8.4.55` (cartva must not touch a voiced
  junction).
- Add a **`vaDve`** pin — the second witness, and the cell the commentaries
  use to illustrate the sūtra.

**Unit and negative pins** (`crates/panini-prakriya/src/tinanta.rs`), chosen
to bracket 8.2.25's guard on both sides:

- *Under-application:* `*AdDve`, `*vadDve`, `*vasDve`, `*avasDvam` INVALID.
  (`AsDve` is already pinned and stays correct.)
- *Over-application — the `s` must be adjacent to the `dh`:* `AsIDvam` and
  `vasIDvam` asserted with `assert_eq`. The sīyuṭ residue `I` sits between.
  This arm catches a guard that reads the aṅga instead of the term actually
  preceding the affix.
- *Over-application — the elided segment must be `s`:* `alaBaDvam` asserted
  `assert_eq` against the slice-3 golden. The preceding term is śap's `a`, so
  nothing may be elided. Kills a mutant dropping the "ends in `s`" test.
- *Over-application — the affix must be `dh`-initial:* `vaste`, `vasse`,
  `avasTAH`, `vassva` asserted `assert_eq`. These double as the first pins
  that cartva leaves `s` alone before `t` / `th` / `s`.
- *Wrong pada:* `*vasati` INVALID (√vas forced parasmaipada).
- *8.4.53 removal:* no new test. Its trace pin and unit tests are deleted
  with it; the rest of the suite passing unchanged is the evidence that
  nothing else depended on it.

**Mutation testing.** `mise run mutants` to **0 survivors**, targeting
8.2.25's three guard arms (term selection, `D`-initial test, "ends in `s`"
test). Deleting 8.4.53 removes its mutants along with it; `is_jhal` keeps its
coverage through 8.4.55.

**Static gates unchanged:** `fmt-check`, `lint`, `audit`,
`#![forbid(unsafe_code)]`, SLP1-only internal representation. `panini-lipi`
roundtrip and property tests untouched — no new phonemes.

## Verification method

Forms were cross-checked with **vidyut-prakriya**, which is what the repo's
citation practice calls for (ashtadhyayi.com is a JS SPA that cannot be
fetched). The probe builds a `Dhatu::mula(Slp1String::from("vasa~\\"),
Gana::Adadi)` — and `"Asa~\\"`, `"a\\da~"`, `"yA\\"`, `"vA\\"` for the audit —
and enumerates all four lakāras × nine cells via
`Vyakarana::derive_tinantas`, printing `p.text()` and `p.history()`.

Two notes for whoever repeats it: vidyut's `Pada` enum has no
`Parasmaipada` / `Atmanepada` variants (pada is inferred from the dhātu), and
the `mise` cargo shim fails in background shells — invoke the toolchain
binary directly.

vidyut emits optional variants our engine does not (`attAt`/`attu`,
`ayuH`/`ayAn`). The golden pins a single form by existing convention, so
"matches vidyut" always means "is among vidyut's forms."

## Documentation

- `AGENTS.md:32` — rewrite the adādi note: 1044 forms, 29 roots, √vas added,
  and the "first voiced internal junction (8.4.53 *jhalāṃ jaś jhaśi*)" claim
  replaced by 8.2.25 *dhi ca*.
- `README.md` — root count and the adādi scope sentence.
- `docs/ARCHITECTURE.md` — add √vas to the adādi coverage paragraph.
- `docs/superpowers/specs/2026-07-22-adadi-gana-design.md` — dated correction
  footnote on the root table, whose √vas row still cites 8.2.39 jaśtva. This
  doc is the governing roadmap for slice 5f, so a known-wrong claim in it
  must not survive.
- The 5d spec and plan stay untouched as historical record; this document's
  "Correction to slice 5d" section is where the fix is recorded.

## Risks

1. **8.2.25's scope.** vidyut applies it to any non-pada term-final `s`
   before a `dh`-initial affix; our guard is narrower — the junction just
   after the aṅga. The adjacency formulation plus the `AsIDvam` and
   `alaBaDvam` pins bracket it, but a later slice with a different term
   layout may need to widen it. Named here so it is not rediscovered.
2. **Deleting correct grammar.** 8.4.53 is real and will be needed.
   Mitigation: git history, plus the restore trigger recorded above
   (√duh / √lih / √dviṣ).
3. **Optional variants.** Pre-existing, not introduced here — see
   Verification method.
4. **Homonym.** √vas *ācchādane* (2Ā) vs √vas *nivāse* (1P); only `artha`
   disambiguates in the tsv.

## Out of scope, flagged not fixed

- **`tinanta.rs` is 3568 lines** (~1500 of ordered rule table, ~1870 of unit
  tests) and grows again in 5f. A candidate for its own slice; splitting the
  rule table is risky because rule *order* is the grammar, so the trace tests
  are the only guard against a reordering slip.
- **A committed cross-check harness** against vidyut-prakriya would have
  caught the 5d error mechanically. Also a follow-up: adding vidyut as a
  dev-dependency is its own decision, with its own supply-chain gate.

## Parked mutant: 7.3.100's `||`/`&&` guard

7.3.100 *adaḥ sarvezām* guards on `!matches!(p.ctx.lakara, Lakara::Lan) ||
!p.terms[ANGA].has(Tag::Adadi)`. The `||`→`&&` mutant on that line survives
`cargo-mutants` — human-parked 2026-07-25 (slice 5e), so the mutation gate
stands at 1 missed survivor by explicit decision. A filtered run in a scratch
worktree at the branch's merge-base (`79bbd99`, tip of slice 5d) reproduced
the same survivor, so it predates 5e; it is inherited, not introduced here.

It is unkillable in the current grammar — a two-case analysis:

- **Case A (laṅ, non-adādi root).** Every non-adādi gaṇa keeps its a-final
  vikaraṇa (śap/śyan/śa) in all four covered lakāras, so the aṅga is always
  vowel-final; the rule's inner `is_vowel(anga_last)` check excludes the case
  regardless of the outer guard.
- **Case B (adādi root, non-laṅ lakāra).** 7.3.100 runs before 3.4.103,
  7.2.79, 6.1.87 and 6.1.66, so the ending is still in its unreduced
  multi-character shape when the rule executes; the inner
  `e.chars().count() != 1` check excludes the case. (3.4.100 *itaś ca*
  excludes loṭ and ātmanepada and only strips the trailing vowel of an
  already-2-char ending, so it can't produce a counterexample either.)

Net: wherever `||` and `&&` would diverge, an inner check has already
excluded the case. Retightening the guard is grammar design work needing its
own spec; new root coverage (√śī, slice 5f) may supply a real distinguishing
case, but √vas landing in 5e did not — hence the correction to this rule's
older "retighten when √vas lands" comment.
