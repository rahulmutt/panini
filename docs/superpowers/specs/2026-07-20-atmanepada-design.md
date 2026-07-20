# Ātmanepada (Phase 2, slice 3)

Status: approved, not yet planned
Supersedes: nothing. Extends `2026-07-19-panini-astadhyayi-design.md` (v1),
`2026-07-20-lan-lot-lakaras-design.md` (slice 1), and
`2026-07-20-vidhilin-lakara-design.md` (slice 2).

## Goal

Extend the tiṅanta engine from one pada to two: add **ātmanepada** (labhate
"he obtains") alongside parasmaipada, across all four existing lakāras
(laṭ, laṅ, loṭ, vidhiliṅ).

## Scope

Unchanged from slice 2: *bhvādi* (gaṇa 1), the four lakāras, single *pada*
input, on-the-fly derivation, all three *puruṣa* × three *vacana*.

New axis: **pada**. Six new purely-ātmanepadī bhvādi roots join the curated
set — `eD` (eDate), `laB` (laBate), `sev` (sevate), `vft` (vartate), `BAz`
(BAzate), `Ikz` (Ikzate). Two of them (`eD`, `Ikz`) are vowel-initial, which
brings the āṭ variant of the laṅ augment into scope (6.4.72, 6.1.90). The six
existing roots stay parasmaipada-only; ubhayapadī roots are out of scope.

Golden coverage grows 216 → **432 forms** (12 roots × 4 lakāras × 9 cells;
each root has exactly one pada).

Out of scope, deferred to later slices: ubhayapadī roots and the
intent-conditioned pada rules (1.3.72), āśīrliṅ, other gaṇas, other lakāras
(lṛṭ, luṅ, liṭ, …), *subanta*, compounds, the precomputed FST layer.

## Why ātmanepada next

It is the roadmap's stated order (v1 spec: "more lakāras …, ātmanepada, more
gaṇas"), and it multiplies all four existing lakāras at once instead of adding
a fifth column. The heavy machinery is already built: every ātmanepada ending
chain terminates in rules slice 2 introduced (7.2.79 salopa, 6.1.87 ād guṇaḥ,
6.1.66 lopo vyor vali) or slice 1 did (7.1.3, 6.1.97, 6.1.101, 7.3.101).

What it adds that slices 1–2 did not have:

- a **pada axis** that the derivation itself sanctions (1.3.12 / 1.3.78), so
  wrong-pada forms (`laBati`, `Bavate`) are INVALID by derivation, not by
  analyzer fiat;
- **term-level ṅid-vat** via the atideśa 1.2.4 — laṭ and loṭ are ṭit lakāras,
  yet their apit ātmanepada endings behave as ṅit for 7.2.81;
- the **sīyuṭ-āgama** (3.4.102), the ātmanepada sibling of slice 2's yāsuṭ,
  feeding the identical downstream chain:

```
ta → sIyta → Iyta ;  laBa + Iyta → laBe + yta → laBeta
```

## Architecture

Unchanged in shape. Ātmanepada is fifteen new self-guarding `Rule`s in
`TINANTA_RULES` plus seven guard/shape widenings; `derive`, `run_pipeline`,
`Prakriya`, and `Term` are untouched except for one new term tag.

**Data.** `Pada` gains `Atmanepada`. `tin.tsv` gains the nine raw ātmanepada
upadeśa endings: `ta, AtAm, Ja` / `TAs, ATAm, Dvam` / `iw, vahi, mahiN`
(`tin_ending` keys on pada). `dhatupatha.tsv` gains a `pada` column; the six
existing roots are tagged `parasmaipada`, the six new roots `atmanepada`. As
in prior slices, the data layer holds only upadeśa material — every surface
divergence is rule-driven.

**Pada sanction.** The first pipeline step is a rule pair: 1.3.12
anudāttaṅita ātmanepadam fires when root tag and requested pada are both
ātmanepada; 1.3.78 śeṣāt kartari parasmaipadam is the elsewhere case. Exactly
one records into the trace; if neither sanctions the requested pada the
derivation fails, keeping derivation the source of truth. The anudātta-it
marker that 1.3.12 names is represented by the data tag — full upadeśa accent
modeling was considered and rejected (no accent data in the bootstrapped
source; machinery with a single consumer).

**Term-level ṅid-vat (1.2.4).** The existing `is_ngit_like` context flag means
"the lakāra is ṅit(-like)" and drives 3.4.100/3.4.101. 7.2.81 āto ṅitaḥ needs
a different fact: the *ending* is ṅit — true in laṭ/loṭ ātmanepada only via
1.2.4 sārvadhātukam apit. Following the 3.4.85 precedent, 1.2.4 is a
`RuleKind::Atidesha` rule that appears in the trace and sets a ṅid-vat tag on
the ending term. It is guarded to ātmanepada endings in this slice: parasmaipada
apit endings (tas, Ji, …) are equally ṅid-vat in principle, but no implemented
rule consumes the fact, and firing it would perturb the 216 pinned parasmaipada
traces. Widening later is additive; the rule comment records this. One genuine
exclusion inside ātmanepada: loṭ uttama endings are pit by 3.4.92's own *pic
ca*, hence not apit, hence outside 1.2.4 — which is exactly what stops 7.2.81
from eating the āṭ-initial `AvahE`/`AmahE` (they belong to 6.1.101 instead).

**Sīyuṭ** is a text prefix on the ending term, exactly like yāsuṭ/āṭ/aṭ, so
the ANGA/SHAP/ENDING indices stay stable.

## The rules

Ordered as they appear in `TINANTA_RULES`. **Bold** = new in this slice.
*(widened)* = existing rule whose guard or shape changes.

| #  | Sūtra                                  | Effect                                                      | Guard |
|----|----------------------------------------|-------------------------------------------------------------|-------|
| 0  | **1.3.12 anudāttaṅita ātmanepadam / 1.3.78 śeṣāt kartari parasmaipadam** | sanction the requested pada against the root's pada tag; mismatch fails the derivation | all |
| 1  | 3.4.78 tiptasjhi…                      | lakāra → base tiṅ ending (now keyed by pada)                | all |
| 2  | 1.3.3 + 1.3.9 it-saṃjñā                | iw→i, mahiN→mahi; 1.3.4 protects TAs/AtAm/Dvam finals       | all |
| 3  | **1.2.4 sārvadhātukam apit**           | mark the ending term ṅid-vat (atideśa)                      | ātmanepada, except loṭ uttama (pit via 3.4.92 pic ca) |
| 4  | 3.4.85 loṭo laṅvat                     | set `is_ngit_like`                                          | loṭ |
| 5  | 3.4.108 jher jus                       | Ji → jus → us                                               | vidhiliṅ (P by text) |
| 6  | **3.4.105 jhasya ran**                 | Ja → ran (apavāda to 7.1.3 for liṅ)                         | vidhiliṅ Ā |
| 7  | **3.4.106 iṭo 't**                     | i (from iw) → a                                             | vidhiliṅ Ā |
| 8  | 3.4.101 tas-thas-tha-mipām *(widened)* | tas→tAm, Tas→tam, Ta→ta, mip→am                             | + parasmaipada |
| 9  | 3.4.99 nityaṃ ṅitaḥ                    | vas→va, mas→ma                                              | ṅit-like (P by text) |
| 10 | 3.4.87 ser hyapic ca                   | si→hi                                                       | loṭ |
| 11 | 3.4.89 mer niḥ                         | mi→ni                                                       | loṭ |
| 12 | 3.4.86 er uḥ                           | ti→tu, Ji→Ju                                                | loṭ |
| 13 | 3.4.100 itaś ca *(widened)*            | elide final i of the tiṅ                                    | + parasmaipada |
| 14 | **3.4.80 thāsaḥ se**                   | TAs → se (apavāda to 3.4.79, hence ordered before it)       | ṭit lakāras, Ā |
| 15 | **3.4.79 ṭita ātmanepadānām ter e**    | ṭi (last vowel + rest) → e: ta→te, AtAm→Ate, Ja→Je, ATAm→ATe, Dvam→Dve, i→e, vahi→vahe, mahi→mahe | ṭit lakāras, Ā |
| 16 | **3.4.91 savābhyāṃ vāmau**             | se→sva, Dve→Dvam (apavāda to 3.4.90)                        | loṭ |
| 17 | **3.4.93 eta ai**                      | uttama e → E (apavāda to 3.4.90)                            | loṭ, Ā |
| 18 | **3.4.90 ām etaḥ**                     | final e → Am: te→tAm, Ate→AtAm, Je→JAm, ATe→ATAm            | loṭ, Ā |
| 19 | 3.4.92 āḍ uttamasya pic ca *(widened)* | prefix `A` to uttama ending                                 | loṭ, both padas; arm set gains E/vahE/mahE (the post-3.4.93 Ā shapes) |
| 20 | 3.4.103 yāsuṭ … *(widened)*            | prefix `yAs`                                                | + parasmaipada (the sūtra's own *parasmaipadeṣu*; the `tinanta.rs:317` revisit) |
| 21 | **3.4.102 liṅaḥ sīyuṭ**                | prefix `sIy` to the (already substituted) ending            | vidhiliṅ, Ā |
| 22 | 3.1.68 kartari śap                     | insert śap; mark aṅga — the index boundary                  | all |
| 23 | 6.4.71 luṅ-laṅ-lṛṅkṣv aḍ-udāttaḥ *(widened)* | prefix `a` to the aṅga                                | laṅ, consonant-initial |
| 24 | **6.4.72 āḍ ajādīnām**                 | prefix `A` to the aṅga (apavāda to 6.4.71)                  | laṅ, vowel-initial |
| 25 | 7.1.3 jho'ntaḥ                         | leading J → ant (Je→ante, JAm→antAm)                        | all |
| 26 | 7.2.79 liṅaḥ salopo 'nantyasya         | elide the non-final s: sIyta→Iyta (comment already says non-final) | vidhiliṅ |
| 27 | 7.2.80 ato yeyaḥ                       | yA → iy (never fires for Ā: sīyuṭ yields Iy, not yA)        | vidhiliṅ |
| 28 | **7.2.81 āto ṅitaḥ**                   | ending-initial `A` → `iy` after a-final aṅga, ending ṅid-vat: Ate→iyte, AtAm→iytAm, ATe→iyTe | shape + ṅid-vat |
| 29 | 7.3.84 sārvadhātukārdhadhātukayoḥ      | aṅga-final ik → guṇa                                        | all |
| 30 | **7.3.86 pugantalaghūpadhasya ca**     | guṇa of the aṅga's light penultimate ik before the sārvadhātuka: vft → vart | shape (vft is the only curated root with an ik upadhā) |
| 31 | 6.1.78 eco'yavāyāvaḥ                   | e/o/E/O → ay/av/Ay/Av                                       | all |
| 32 | 7.3.101 ato dīrgho yañi                | śap a → A before m/v (now also Ā vahi/mahi)                 | laṭ, laṅ |
| 33 | 6.1.101 akaḥ savarṇe dīrghaḥ           | a + A → A (āṭ of 3.4.92, both padas)                        | loṭ uttama |
| 34 | **6.1.90 āṭaś ca**                     | āṭ + ac → vṛddhi ekādeśa. Aṅga arm (laṅ): A+eD→ED, A+Ikz→Ekz. Ending arm (loṭ uttama eka, after 6.1.101 has made śap `A`): A+E→E | āṭ shape |
| 35 | 6.1.97 ato guṇe *(widened)*            | a + guṇa vowel → pararūpa: a+a (anta) **and a+e (laṭ Ā 1sg: Bava+e→Bave)** | shape |
| 36 | 6.1.87 ād guṇaḥ *(widened)*            | a + i/**I** → e: aBava+i→aBave; laBa+Iyta→laBe+yta; laBa+iyte→laBe+yte | shape |
| 37 | 6.1.66 lopo vyor vali                  | ending-initial y before val is dropped: yta, yTAs, yTe, yvahi, yran drop it; yAtAm, yATAm, ya keep it | shape |
| 38 | 6.4.105 ato heḥ                        | elide hi after a                                            | loṭ (P by text) |
| 39 | 8.2.23 saṃyogāntasya lopaḥ             | drop final of word-final conjunct                           | all |
| 40 | 8.3.15 kharavasānayoḥ                  | word-final s → visarga (TAs→TAH)                            | all |

### Guard widenings on existing rules

- **3.4.100 itaś ca** and **3.4.101** gain `pada == Parasmaipada`. Both sūtras
  are parasmaipada substitutions; without the guard, 3.4.100 eats the final `i`
  of ātmanepada `vahi` / `mahi` / `i` in laṅ and vidhiliṅ (aBavAvahi would
  derive as aBavAvah). 3.4.101 is additionally safe by text today (`TAs` ≠
  `Tas`), but the pada guard states the sūtra's actual domain.
- **3.4.103 yāsuṭ** gains `pada == Parasmaipada` — the sūtra itself says
  *parasmaipadeṣu*, and the slice-2 comment at `tinanta.rs:317` explicitly
  deferred this until ātmanepada arrived.
- **6.4.71 aṭ** cedes vowel-initial aṅgas to its apavāda **6.4.72 āṭ**
  (ordered adjacent; guard by aṅga-initial phoneme class).
- **3.4.92 āḍ uttamasya pic ca** widens its exact-text arm set from
  `ni|va|ma` (the parasmaipada shapes left by 3.4.89/3.4.99) to also accept
  `E|vahE|mahE` (the ātmanepada shapes left by 3.4.93). Same explicit-set
  style as today; MUST stay ordered after 3.4.93.
- **6.1.97 ato guṇe** widens from `a + a` to `a + guṇa vowel` (adds the `a + e`
  pararūpa for laṭ ātmanepada 1sg `Bave`); **6.1.87 ād guṇaḥ** widens its
  ending-initial match from `i` to `i/I` (sīyuṭ survives salopa as long-I `Iy`,
  unlike yāsuṭ's short `iy`). Both are pure shape generalizations; the 216
  existing forms pin them.

### Ordering decisions that carry weight

Each was derived by hand-working all 72 ātmanepada cells.

- **#14 before #15.** 3.4.80 thāsaḥ se is the apavāda to ter e for `TAs`;
  reversed, 3.4.79 rewrites TAs's ṭi (`As` → e) to `Te` and 3.4.80 never sees
  `TAs` — 2sg derives wrong in both ṭit lakāras.
- **#16, #17 before #18.** Both are apavādas to ām etaḥ: reversed, `se` → `sAm`
  (not sva) and loṭ uttama `e` → `Am` (not E). After #17 the uttama endings are
  `E`, which 3.4.90's short-e match ignores — no explicit uttama exclusion
  needed on #18.
- **#6, #7 before #21** (and #5–#18 before #20/#21 generally): substitutions
  fire on exact ending text, so the āgama prefix must come last — the same
  reasoning that ordered yāsuṭ after the parasmaipada substitutions in slice 2.
- **#6 before #25.** 3.4.105 jhasya ran preempts jho'ntaḥ for liṅ's Ja; #25
  runs post-śap, by which time Ja is `ran` — the preemption is by position,
  like 3.4.108 before 3.4.100.
- **#28 after #18.** loṭ's 3du/2du loop textually (AtAm →#15 Ate →#18 AtAm)
  before 7.2.81 rewrites the initial A — the net effect (BavetAm ≠ laṭ's
  Bavete) comes from the ṭit/loṭ rule pair, not from skipping either.
- **#28 before #33.** Both 7.2.81 and 6.1.101 match "śap `a` + ending-initial
  `A`". The ṅid-vat tag disambiguates (7.2.81 requires it, and loṭ uttama's
  āṭ-carrying endings never get it, per the 1.2.4 pic-ca exclusion), but the
  order still matters for the tagged cells: 6.1.101 first would merge laṭ 3du
  into `laBAte` (wrong) before 7.2.81 could fire. Pinned by `laBete`.
- **#34 after #33.** 6.1.90's ending arm consumes "śap `A` + ending `E`",
  a shape that exists only after 6.1.101 has coalesced the āṭ into the śap.
  At the spec's earlier draft position (before 7.3.101) the arm could never
  fire and loṭ uttama eka would surface as `laBAE`.
- **#36 before #37** (unchanged from slice 2): 6.1.66 needs the ending to
  start with `y`, which only happens after ād guṇaḥ absorbs the `i`/`I`.

### Reference verification

Per AGENTS.md, sūtra ids and names in traces must match ashtadhyayi.com
(via the `sanskrit/ashtadhyayi` data mirror, as in slice 2). The fifteen new
sūtras must each be verified for id, SLP1 name, and effect before their trace
literals are pinned. Draft SLP1 names, to be confirmed verbatim:
`anudAttaNita Atmanepadam` (1.3.12), `SezAt kartari parasmEpadam` (1.3.78),
`sArvaDAtukam apit` (1.2.4), `TAsaH se` (3.4.80), `wita AtmanepadAnAM wer e`
(3.4.79), `savAByAM vAmO` (3.4.91), `eta E` (3.4.93), `Am etaH` (3.4.90),
`Jasya ran` (3.4.105), `iwo 't` (3.4.106), `liNaH sIyuw` (3.4.102),
`Aq ajAdInAm` (6.4.72), `AwaS ca` (6.1.90), `Ato NitaH` (7.2.81),
`pugantalaGUpaDasya ca` (7.3.86).

One attribution needs explicit confirmation: loṭ uttama eka (`laBE`) merges
`A + E` — this spec assigns it to **6.1.90 āṭaś ca** (the merging A *is* the
3.4.92 āṭ), not the generic 6.1.88 vṛddhir eci. If the reference derivation
cites 6.1.88 instead, implement 6.1.88 and drop 6.1.90's loṭ arm (the laṅ arm
for EData/Ekzata is unambiguously 6.1.90).

## Data and facade changes

**`panini-data`:** `Pada` gains `Atmanepada`; `tin_ending` keys on pada with
the nine new upadeśa endings; root records gain the pada tag; six new roots.

**`panini-analyze`:** candidates iterate each root's tagged pada instead of
hardcoding parasmaipada; candidates per check 216 → 432. The ending heuristics
learn the ātmanepada surface shapes.

**`panini` facade / CLI:** `Analysis` gains the pada (SLP1 names
`parasmEpadam` / `Atmanepadam` — confirm spelling against the reference);
the JSON output adds a `pada` field. No new flags or subcommands.

## Testing

- **Golden paradigm** extends to 432 forms. The 216 new forms are verified
  against ashtadhyayi.com before being committed. The 216 existing
  parasmaipada surface forms must remain byte-identical; run them before
  adding the new forms, not only after (the seven widenings and the 1.2.4
  guard are designed for exactly this). Their pinned traces change in exactly
  one way: every parasmaipada derivation now begins with the 1.3.78 pada
  sanction step (matching the reference prakriyā, which records it), so the
  twelve sequences in `trace.rs` each gain a leading `"1.3.78"` — a one-time
  mechanical update, and any other trace difference is a bug.
- **Negative forms.** Wrong-pada: `laBati`, `Bavate`, `eDati`, `alaBat` must
  be INVALID (pada sanction failure). Cross-lakāra: `laBatAm` as laṭ, `laBeta`
  as laṭ are INVALID cells (they are valid only as loṭ/vidhiliṅ). No
  currently-asserted negative becomes valid.
- **Trace test** pins the full ordered sūtra sequence for one exemplar per new
  chain: `laBete` (1.2.4 + 7.2.81 path), `laBasva` (3.4.80 + 3.4.91), `laBE`
  (3.4.93 + 3.4.92 + the A+E merge), `laBeran` (3.4.105 + 3.4.102 + 7.2.79),
  `laBeya` (3.4.106), `EData` (6.4.72 + 6.1.90), `vartate` (7.3.86), and
  `laBate` (the minimal 1.3.12 + 3.4.79 path).
- **Roundtrip** extends over the pada axis automatically.
- **Unit tests** in `tinanta.rs`: name-gate and digraph tests cover the new
  rules; targeted tests for 3.4.100's new pada guard (vahi keeps its i), 7.2.81
  (fires on ṅid-vat `Ate`, not on parasmaipada shapes), and 6.1.87's long-I arm.
- **Mutation testing** (`mise run mutants`) remains the real gate: with 42
  guarded rules the likely defect class is a dropped or inverted guard,
  especially the new pada guards.

### Expected forms, for reference

- **laB laṭ:** laBate, laBete, laBante; laBase, laBeTe, laBaDve; laBe,
  laBAvahe, laBAmahe
- **laB laṅ:** alaBata, alaBetAm, alaBanta; alaBaTAH, alaBeTAm, alaBaDvam;
  alaBe, alaBAvahi, alaBAmahi
- **laB loṭ:** laBatAm, laBetAm, laBantAm; laBasva, laBeTAm, laBaDvam; laBE,
  laBAvahE, laBAmahE
- **laB vidhiliṅ:** laBeta, laBeyAtAm, laBeran; laBeTAH, laBeyATAm, laBeDvam;
  laBeya, laBevahi, laBemahi
- **eD laṅ** (vowel-initial): EData, EDetAm, EDanta; EDaTAH, EDeTAm, EDaDvam;
  EDe, EDAvahi, EDAmahi

## Success criteria

- All 432 golden forms validate as VALID with a correct, ordered sūtra trace.
- Every sūtra id and name in a trace matches ashtadhyayi.com.
- The 216 pre-existing surface forms are byte-identical; their traces differ
  only by the new leading 1.3.78 pada-sanction step.
- Wrong-pada forms return INVALID via pada-sanction failure in the derivation.
- Mutation testing shows the new guards (especially the pada guards) pinned.
- Single-word `check` remains interactive-fast at 432 candidates.

## Risks

- **The pada guards touch all four parasmaipada lakāras.** 3.4.100/3.4.101/
  3.4.103 are load-bearing everywhere; the 216 existing golden forms and
  pinned traces are the safety net.
- **Attribution of the A+E merge** (6.1.90 vs 6.1.88) and of loṭ's e-chain
  ordering: a right form by the wrong rule is invisible to the golden test —
  reference verification and pinned traces are the only guards.
- **3.4.100 under-guarding** silently corrupts ātmanepada uttama cells
  (vahi→vah) in ṅit lakāras; covered by golden cells and a unit test.
- **1.2.4 scope choice** (ātmanepada-only) is deliberate trace-minimalism, not
  grammar: the comment must say so, or a later slice may mistake it for a
  claim that parasmaipada endings are not ṅid-vat.
