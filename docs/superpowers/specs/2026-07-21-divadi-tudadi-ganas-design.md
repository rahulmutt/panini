# Divādi + Tudādi gaṇas (Phase 2, slice 4)

**Status:** Design, approved in brainstorming 2026-07-21.

Builds on `2026-07-19-panini-astadhyayi-design.md` (v1 vertical slice),
`2026-07-20-lan-lot-lakaras-design.md` (slice 1),
`2026-07-20-vidhilin-lakara-design.md` (slice 2), and
`2026-07-20-atmanepada-design.md` (slice 3).

## Summary

Extend the engine from one gaṇa to three by adding **divādi** (gaṇa 4) and
**tudādi** (gaṇa 6) alongside the existing **bhvādi** (gaṇa 1). The only thing
that changes between these gaṇas in the present system (laṭ / laṅ / loṭ /
vidhiliṅ) is the **vikaraṇa** — the affix inserted between root and ending:

- bhvādi → **śap** (3.1.68), *pit*
- divādi → **śyan** (3.1.69), *apit*
- tudādi → **śa** (3.1.77), *apit*

śyan and śa are *apit* sārvadhātuka affixes, so by 1.2.4 (*sārvadhātukam apit*)
they are treated as *ṅit*, and by 1.1.5 (*kṅiti ca*) a following ṅit affix
blocks the guṇa that śap (being *pit*) allows. That single pit/apit contrast is
the whole grammatical payload of this slice:

- bhvādi: BU + **śap** → guṇa → **bhavati**
- divādi: div + **śyan** → guṇa **blocked** → **dīvyati** (+ 8.2.77 lengthening)
- tudādi: tud + **śa** → guṇa **blocked** → **tudati**

## Scope

Unchanged from slice 3: single *pada*, *tiṅanta*, the four lakāras (laṭ, laṅ,
loṭ, vidhiliṅ), both padas (parasmaipada, ātmanepada), all nine *puruṣa* ×
*vacana* cells.

New in this slice: two more gaṇas (divādi, tudādi), each with three
parasmaipadī and three ātmanepadī roots, giving the full 4 lakāras × 2 padas ×
9 cells for each new root.

Out of scope, deferred to later slices: other gaṇas (adādi, curādi, …);
ubhayapadī roots and the intent-conditioned pada rules (1.3.72
*svaritañitaḥ...*); āśīrliṅ and other lakāras; the num-infixing tudādi roots
(e.g. √vid *lābhe* → vindati, √muc → muñcati, 7.1.59 *śe mucādīnām*); the
saṁprasāraṇa / cvi roots (√pracch → pṛcchati). Root choice below is curated to
avoid every one of these.

## Why divādi + tudādi next

Of "more gaṇas" (the next roadmap item), these two are the gentlest extension
and the highest coverage-per-rule. Each gaṇa is, in the present system, *just a
different vikaraṇa*. Neither śyan nor śa changes the term-index layout — both
occupy the same slot śap does and are a-final like śap — so every rule on
either side of the 3.1.68 boundary keeps working unmodified. Adding them
multiplies coverage across all four existing lakāras and both padas at once,
while introducing exactly one genuinely new piece of grammar: **guṇa-blocking by
the ṅit vikaraṇa** (1.1.5 *kṅiti ca* + 1.2.4 *sārvadhātukam apit*).

The pit/apit machinery this slice builds is also directly reused later: curādi's
śap returns as *pit*, and the causative ṇic is *ñit*; the 1.1.5 guard on the
guṇa rules is what makes all of that come out right.

## Root set

Twelve new roots (3 P + 3 Ā per gaṇa), verified against the Vidyut Dhātupāṭha
mirror (`vidyut-prakriya/data/dhatupatha.tsv`). Stored, as in prior slices, as
the resolved upadeśa root (anubandhas already stripped at data-entry time) plus
gaṇa, pada, and artha.

### divādi (gaṇa 4) — vikaraṇa śyan

| root (SLP1) | pada | artha | surface (laṭ 3sg) | pins |
| --- | --- | --- | --- | --- |
| `div` | parasmaipada | krīḍāyām | **dīvyati** | guṇa-block + 8.2.77 |
| `naS` | parasmaipada | adarśane | **naśyati** | — (a-upadhā, no guṇa) |
| `kup` | parasmaipada | krodhe | **kupyati** | guṇa-block (u-upadhā) |
| `man` | ātmanepada | jñāne | **manyate** | — (a-upadhā) |
| `yuD` | ātmanepada | samprahāre | **yudhyate** | guṇa-block (u-upadhā) |
| `vid` | ātmanepada | sattāyām | **vidyate** | guṇa-block (i-upadhā) |

*(Vidyut ids: divu~ 04.0001; Ra\Sa~ 04.0091; kupa~ 04.0146; ma\na~\ 04.0073;
yu\Da~\ 04.0069; vi\da~\ 04.0067.)*

`div` (the gaṇa namesake) is the one root needing grammar beyond the vikaraṇa
swap: after guṇa is blocked it stays `div`, then **8.2.77 hali ca** lengthens
the *i* before the *y* of śyan → `dīv` → **dīvyati**.

### tudādi (gaṇa 6) — vikaraṇa śa

| root (SLP1) | pada | artha | surface (laṭ 3sg) | pins |
| --- | --- | --- | --- | --- |
| `tud` | parasmaipada | vyathane | **tudati** | guṇa-block (u-upadhā) |
| `liK` | parasmaipada | akṣaravinyāse | **likhati** | — (i-upadhā, blocked) |
| `viS` | parasmaipada | praveśane | **viśati** | — (i-upadhā, blocked) |
| `juz` | ātmanepada | prītisevanayoḥ | **juṣate** | guṇa-block (u-upadhā) |
| `vij` | ātmanepada | bhayacalanayoḥ | **vijate** | guṇa-block (i-upadhā) |
| `gur` | ātmanepada | udyamane | **gurate** | guṇa-block (u-upadhā) |

*(Vidyut ids: tu\da~^ 06.0001; liKa~ 06.0092; vi\Sa~ 06.0160; juzI~\ 06.0008;
o~vijI~\ 06.0009; gurI~\ 06.0131.)*

**Deviation from the brainstorm's illustrative list** (`tud, likh, kṣip` /
`juṣ, vij, +1`): the third tudādi parasmaipada slot is **√viś, not √kṣip**.
`kṣip` in the Dhātupāṭha is svarita-marked (`kzi\pa~^` → ubhayapada, 1.3.72,
deferred) *and* appears in more than one gaṇa, which would create a genuinely
ambiguous candidate; `viś` (viśati) is cleanly parasmaipada tudādi. The third
tudādi ātmanepada slot resolves to **√gur** (gurate) — a clean pure-ātmanepada
tudādi root, whereas most tudādi ātmanepada roots (√dṛ, √dhṛ, √lasj, √mṛ) drag
in riṅ/ścutva/irregular machinery this slice defers.

One fidelity note: **√tud is ubhayapadī** in the Dhātupāṭha (svarita it). This
slice takes only its **parasmaipada** pada (tudati), consistent with slice 3's
explicit deferral of ubhayapadī roots and 1.3.72; the ātmanepada tudati-form is
future work. The gaṇa is still represented by its namesake.

Coverage: 12 → 24 roots; golden paradigm 432 → **864** forms (12 new roots × 4
lakāras × 9 cells = 432 new).

## Data layer (`panini-data`)

Small and mechanical, following the slice-3 pattern:

- `Gana` enum gains `Divadi` and `Tudadi` variants (currently `Bhvadi` only).
- Twelve new `Dhatu` entries in the curated set, each carrying its gaṇa, pada,
  and artha; mirrored as rows in `data/dhatupatha.tsv` with a gaṇa column value
  (`divadi` / `tudadi`) and the pada column already present from slice 3.
- No structural change to `Dhatu` (it already holds `gana` and `pada`). The
  data layer continues to hold only upadeśa material; every surface change
  happens in the engine.
- `data/ATTRIBUTION.md` unchanged in structure; the new rows are cross-checked
  against the same ashtadhyayi.com / Vidyut sources already cited.

## Engine (`panini-prakriya`)

All changes are self-guarding `Rule`s in `TINANTA_RULES` (per AGENTS.md), or
guard additions to existing rules. No change to the `ANGA / SHAP /
ENDING_PRE_SHAP / ENDING` index layout: śyan and śa are inserted at the same
`SHAP` slot śap uses, and are a-final like śap.

### New: term-level *pit* on the vikaraṇa

The vikaraṇa's pit/apit status is the hinge of this slice. It is determined from
the **p-anubandha** of the upadeśa: `Sap` carries final `p` (pit); `Syan` and
`Sa` do not (apit). When a vikaraṇa is inserted, the rule tags it `Tag::Pit`
iff its raw upadeśa bears the p-anubandha, *before* it-saṁjñā strips the `p`.
This adds a new `Tag::Pit` to `term.rs` (the existing `Tag::Ngit` is the *ṅit*
marker 1.2.4 already sets on apit endings; there is no pit marker today). śap is
the only curated pit affix; śyan/śa carry no `Pit` tag.

### New rules

**3.1.69 divādibhyaḥ śyan** — apavāda to 3.1.68 for divādi. Guard:
`ctx.gana == Divadi`. Inserts `Syan` at `SHAP`, runs it-saṁjñā (1.3.8 strips
leading `S`; 1.3.3 *halantyam* strips final `n` → `ya`), tags it
`Vikarana + Sarvadhatuka` (not `Pit`), marks the aṅga. Ordered immediately
before 3.1.68, exactly as 6.4.72 precedes its utsarga 6.4.71.

**3.1.77 tudādibhyaḥ śaḥ** — apavāda to 3.1.68 for tudādi. Guard:
`ctx.gana == Tudadi`. Inserts `Sa` → `a` (1.3.8 strips `S`), same tagging shape,
not `Pit`.

**8.2.77 hali ca** — root-specific, self-guarding on shape: a root ending in `v`
(or `r`) with a short ik upadhā lengthens that upadhā when a hal follows. Fires
only for `div` before the `y` of śyan → `dīv`. Placed in the tripādī block
(before 8.2.23). Never fires for any other curated root. (The exact citation for
dīvyati — 8.2.77 *hali ca*, with 8.2.76 *rvorupadhāyā dīrghaḥ* as anuvṛtti — is
pinned against ashtadhyayi.com when writing the golden block; default citation
is 8.2.77.)

### Changed rules

**3.1.68 kartari śap** — gains a guard declining when a vikaraṇa term is already
present (i.e. when 3.1.69 or 3.1.77 has fired). This keeps it the *utsarga*
rather than hard-coding `gana == Bhvadi`, so curādi (which re-uses śap) works
later. Because 3.1.69/3.1.77 are ordered before it, the apavāda wins by simple
precedence; the guard makes the win explicit and order-independent.

**1.2.4 sārvadhātukam apit** — currently tags apit ātmanepada *endings* ṅit,
early (before the 3.1.68 boundary), which slice-2/3 rules such as 3.4.99 depend
on. That early application is unchanged. A **second ordered application** of
1.2.4 is added immediately after the vikaraṇa boundary (after 3.1.68/69/77,
before 6.4.71): it tags the inserted vikaraṇa ṅit iff the vikaraṇa is
sārvadhātuka and *not* `Pit`. So śyan/śa (apit) get tagged; śap (pit) does not.
Applying 1.2.4 twice is faithful — it genuinely applies to each apit
sārvadhātuka pratyaya — and the second application is guarded so it **does not
fire in bhvādi** (śap is pit), leaving all existing bhvādi traces unchanged.

**7.3.84 sārvadhātukārdhadhātukayoḥ** and **7.3.86 pugantalaghūpadhasya ca** —
each gains a **1.1.5 kṅiti ca** guard: decline when the immediately-following
term (the vikaraṇa at `SHAP`) is ṅit (the `Tag::Ngit` set by the second 1.2.4
application) or kit. In bhvādi the following vikaraṇa is śap (pit, un-tagged) →
guṇa fires unchanged (bhavati; vartate via 7.3.86). In divādi/tudādi the
vikaraṇa is ṅit → guṇa blocked (kupyati not *kopyati*; dīvyati not *devyati*;
tudati not *todati*; juṣate not *joṣate*). The 1.1.5 citation lives in the rule
comments (guards are cited in comments in this codebase, not the trace).

### Rule order (delta only)

```
… 3.4.102 liNas sIyuw
[new] 3.1.69 divAdibhyaH Syan      (guard: gana == Divadi)   ┐ apavādas,
[new] 3.1.77 tudAdibhyaH SaH       (guard: gana == Tudadi)   ┘ before utsarga
      3.1.68 kartari Sap           (guard: no vikaraṇa yet; tags śap Pit)
[new] 1.2.4  sArvaDAtukam apit     (2nd application: tags apit vikaraṇa ṅit)
      6.4.71 luNlaNlfNkzvaqudAttaH
      6.4.72 Aq ajAdInAm
      7.1.3  Jo'ntaH
      …
      7.3.84 sArvaDAtukArDaDAtukayoH   (+ 1.1.5 guard: skip if vikaraṇa ṅit)
      7.3.86 pugantalaGUpaDasya ca     (+ 1.1.5 guard: skip if vikaraṇa ṅit)
      …
[new] 8.2.77 hali ca               (tripādī; div → dīv only)
      8.2.23 …
```

`TINANTA_RULES` entries 42 → **46**: four new array entries (3.1.69, 3.1.77,
8.2.77, and the second 1.2.4 application), spanning **45 distinct sūtra ids**
(1.2.4 now appears twice). Three existing rules gain guards: 3.1.68, 7.3.84,
7.3.86.

## Analyzer, facade, CLI

No code change, by construction:

- **`panini-analyze`** already brute-forces every (root × lakāra × cell) and
  lets the engine confirm by exact match. New roots flow in through `dhatus()`
  automatically; the candidate set grows 432 → 864, still trivially small.
- **`panini`** (facade): `Analysis` already carries pada and everything needed.
- **`panini-cli`**: `check` output shape unchanged. Devanāgarī / IAST / HK
  round-trips of the new forms come free from `panini-lipi` (no new phonemes).

## Testing

Follows the pattern of slices 1–3.

- **Golden paradigm** (`crates/panini/tests/paradigm.rs`): a new block per gaṇa
  × pada × lakāra — all 432 new forms pinned against the ashtadhyayi.com
  reference, total 864.
- **Ordered trace** (`crates/panini/tests/trace.rs`): pin the full sūtra
  sequence for at least four representative derivations —
  - **dīvyati** (śyan; 1.2.4 vikaraṇa-ṅit; 7.3.84/86 blocked; 8.2.77 lengthening),
  - **tudati** (śa; guṇa blocked),
  - **manyate** (divādi ātmanepada; śyan + ātmanepada ending machinery),
  - one vidhiliṅ new-gaṇa form (e.g. **juṣīṣṭa** or **tudyāt**) exercising
    śyan/śa feeding the slice-2 yāsuṭ/sīyuṭ endings.

  Sūtra ids and names checked against ashtadhyayi.com.
- **Cross-gaṇa negatives**: wrong-vikaraṇa forms must return INVALID —
  \**divati*, \**tudyati*, \**bhavyati*, \**naśati*, \**kupati* — plus
  guṇa-should-have-been-blocked non-forms \**kopyati*, \**todati*, \**joṣate*,
  and wrong-pada crosses for the new roots (\**manyati*, \**vidyati*
  parasmaipada; \**tudate* ātmanepada). Same style as slice-3 cross-lakāra
  negatives.
- **Mutation testing**: `mise run mutants` at slice end. The new guard arms
  (gaṇa guards on 3.1.69/3.1.77, the no-vikaraṇa guard on 3.1.68, the second
  1.2.4's apit/Pit check, the 1.1.5 kṅiti guards on 7.3.84/86, and 8.2.77's
  shape guard) must not survive — the negatives above are chosen to kill exactly
  those mutants (e.g. \**kopyati* kills a mutated 1.1.5 guard; \**divati* kills
  a mutated 3.1.69 guard).

`panini-lipi` roundtrip/property tests are untouched (no new phonemes).

## Success criteria

- All 864 golden forms (24 roots × 9 tiṅ cells × 4 lakāras) validate `VALID`
  with correct ordered traces; the 432 pre-existing forms are unchanged.
- The new cross-gaṇa / guṇa-block / wrong-pada non-forms return `INVALID`.
- No existing bhvādi golden or trace output changes (verified by the second
  1.2.4 application and the 1.1.5 guards being no-ops when the vikaraṇa is śap).
- Every sūtra id and name in a trace matches ashtadhyayi.com.
- Mutation testing shows the new guards are genuinely pinned by tests.
