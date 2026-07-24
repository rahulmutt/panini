# Adādi gaṇa — Slice 5d (√ās, ātmanepada, first voiced junction)

**Status:** Design, approved in brainstorming 2026-07-25.

Builds on `2026-07-22-adadi-gana-design.md` (the slice-5 design) and the three
realized adādi sub-slices — `2026-07-22-adadi-gana-5a-aluk-core.md` (aluk core),
`2026-07-23-adadi-vidhilin-5b-design.md` (vidhiliṅ ungate), and
`2026-07-24-adadi-ad-cartva-design.md` (√ad, the first voiceless junction). Also
depends on `2026-07-20-atmanepada-design.md` (slice 3), whose ātmanepada endings
and tiṅ machinery this slice reuses unchanged for a consonant-final aṅga, and on
`2026-07-20-vidhilin-lakara-design.md` (slice 2), whose `ran` (3.4.105) path the
vidhiliṅ 3pl reuses.

## Summary

Slices 5a–5c completed adādi's three **parasmaipadī** roots (√yā, √vā ā-final;
√ad consonant-final, landing cartva). This slice opens the **ātmanepada** side
with the gaṇa's cleanest ātmanepadī root, **√ās** (*upaveśane*), across all four
lakāras × 9 cells = **36 forms in one golden block** (the 5b uniform-paradigm
invariant — √ās never needs a second visit).

The parent spec's one-line table entry called √ās "ā-final, no junction." That
is too optimistic: √ās is **s-final** (`ās` = `ā` + `s`), and once its real
ātmanepada paradigm is traced, two cells force genuinely new machinery — exactly
as slice 5c found √ad needed three rules, not the one its parent row sketched.
The two new mechanisms are:

1. **7.1.5 *ātmanepadeṣv anataḥ*** — the 3pl `jha` becomes `at` (not `ant`)
   after a non-a-final aṅga → `āsate`, not `*āsante`.
2. **The engine's first *voiced* internal junction** — `s → d` before the `dh`
   of `dhve`/`dhvam` → `āddhve` / `āddhvam`, the voiced mirror of 5c's cartva.

Everything else reuses slice-3 ātmanepada endings + the adādi luk (2.4.72)
unchanged. No new gaṇa, no new pada.

The slice ships √ās's full 36 cells in one golden block, so √ās never needs a
second visit — the uniform-paradigm invariant slice 5b established.

## Scope

Unchanged from 5a–5c: adādi (gaṇa 2), all four lakāras (laṭ, laṅ, loṭ,
vidhiliṅ), all nine *puruṣa* × *vacana* cells, single *pada* / *tiṅanta*. New in
this slice: **pada = ātmanepada** for an adādi root (the parasmaipada adādi
roots landed in 5a–5c), one root, **√ās** (*upaveśane*), and with it the first
voiced internal junction.

Out of scope, deferred to the remaining adādi pieces:

- **√vas** (*ācchādane*), slice 5e — it shares √ās's identical `s → d`-before-
  `dhv` junction and is otherwise clean, so once this slice lands the voiced
  junction, √vas is **near-free**: data + goldens, **no new rule** (`vaste` /
  `vasse` / `vaddhve`, …).
- **√śī** (*svapne*), slice 5f — the one irregular adādi root: 7.4.21 *śīṅaḥ
  [guṇaḥ]*, 7.1.6 *śīṅo ruṭ*, 6.1.78 *ayādeśa*.

Also deferred, as in every prior adādi slice: the irregular aluk stars (√as,
√han, √brū, √i, √dviṣ/√duh/√lih), ubhayapadī roots and intent-conditioned pada,
other gaṇas, āśīrliṅ. This slice adds no new gaṇa, no new pada beyond the
ātmanepada machinery already present, and no voiceless-junction rule.

Coverage: golden paradigm **972 → 1008** forms (1 root × 36 cells). Root count
**27 → 28**.

## Paradigm map — where the 36 cells come from

√ās's forms, verified cell-by-cell against ashtadhyayi.com when the golden block
is written (order: 3s 3d 3p / 2s 2d 2p / 1s 1d 1p, SLP1). The **bold** cells are
the only ones needing new rules; the other 30 are clean concatenation over
existing machinery.

| Lakāra    | forms |
|-----------|-------|
| laṭ       | `Aste AsAte` **`Asate`** / `Asse AsATe` **`AdDve`** / `Ase Asvahe Asmahe` |
| laṅ       | `Asta AsAtAm` **`Asata`** / `AsTAH AsATAm` **`AdDvam`** / `Asi Asvahi Asmahi` |
| loṭ       | `AstAm AsAtAm` **`AsatAm`** / `Assva AsATAm` **`AdDvam`** / `Asai AsAvahai AsAmahai` |
| vidhiliṅ  | `AsIta AsIyAtAm AsIran` / `AsITAH AsIyATAm AsIDvam` / `AsIya AsIvahi AsImahi` |

(SLP1: `D` = dh, `T` = th, `A` = ā, `I` = ī. IAST of the bold cells: `āsate`,
`āddhve`, `āsata`, `āddhvam`, `āsatām`.)

The cells split three ways:

- **vidhiliṅ (9) — free.** The `ī` of the athematic optative sits between the
  root-final `s` and every ending, so no junction ever forms (`AsIDvam` is
  `s` + `ī` + `Dvam`, clean), and the 3pl rides slice-2's existing `ran` path
  (3.4.105) → `AsIran`. Zero new rules — this lakāra just extends slices 2/5b to
  a consonant-final ātmanepada aṅga.
- **3pl of laṭ / laṅ / loṭ (3) — rule ①.** `Asate` / `Asata` / `AsatAm`. The
  ātmanepada `jha` meets a non-a-final aṅga.
- **2pl of laṭ / laṅ / loṭ (3) — rule ②.** `AdDve` / `AdDvam` / `AdDvam`. The
  root-final `s` meets the voiced `dh` of `Dve`/`Dvam`.

The remaining 2sg / 2du / 1st-person / dual cells (`Aste`, `Asse`, `AsAte`,
`AsATe`, `Ase`, `Asvahe`, `Asmahe`, `AsTAH`, `Assva`, `Asai`, …) are clean:
either vowel-initial endings, or `s` meeting a non-jhaś (`Asse` `s`+`s`, `Assva`
`s`+`sv`, `AsTAH` `s`+`th`), needing no new phonology. `AsTAH` / `Asai` /
`AdDvam` etc. reuse the existing final-`s` → visarga (8.3.15) and ātmanepada
ending machinery unchanged.

## Rules — two new

Only two rules fire on √ās's goldens.

### ① 7.1.5 *ātmanepadeṣv anataḥ* — 3pl `Asate` (not `*Asante`)

The base rule **7.1.3 *jho'ntaḥ*** replaces the leading `J` (jh) of the
ātmanepada 3pl `Ja` (and the parasmaipada `Ji`) with `ant` → `anta` → `ante`.
For every ātmanepada root so far — all a-final (thematic, e.g. `laBante`) — that
is correct. **7.1.5 *ātmanepadeṣv anataḥ*** is its apavāda: in ātmanepada, when
the aṅga does **not** end in short `a`, the `jh` becomes `at` (not `ant`). √ās is
the **first** non-a-final ātmanepada aṅga in the engine, so today its 3pl would
misfire to `*Asante`; 7.1.5 gives `Ja → at` → `Asate` (and `Asata`, `AsatAm`).

- **Guard:** `Pada::Atmanepada` ∧ ending starts with `J` ∧ aṅga-final ∉ short
  `a`. General — fires for any future non-a-final ātmanepadī root, not just √ās.
- **Placement:** aṅga / 7.1 band, immediately **before** 7.1.3. After 7.1.5
  rewrites the leading `J`, 7.1.3 declines on its own (the ending no longer
  starts with `J`). Same "apavāda ordered before the utsarga, which then
  declines" shape as 5c's 6.4.101 vs 6.4.105.
- **Does not fire** in vidhiliṅ (the 3pl there is `ran` by 3.4.105, not `Ja`),
  nor for the existing a-final ātmanepada roots (the *anataḥ* arm declines) — so
  `laBante` and every slice-3 golden are unchanged.

### ② The first voiced junction — jaśtva, `s → d` before `dh` → `AdDve`

A root-final jhal that meets a voiced stop (`jhaś`) takes its voiced (`jaś`)
counterpart: √ās's `s` → `d` before the `dh` of `Dve` / `Dvam` → `AdDve` /
`AdDvam`. This is the **voiced mirror of 5c's cartva** (8.4.55, which assimilates
a jhal to its *voiceless* `car` before a *voiceless* `khar`). Fires across laṭ
2pl (`AdDve`), laṅ 2pl and loṭ 2pl (`AdDvam`). General, reusable junction — √vas
(5e) inherits it unchanged, and every later gaṇa/subanta/compound slice reuses
it.

- **Guard:** aṅga-final ∈ jhal ∧ following segment ∈ jhaś (voiced stop). Written
  generally as a `jaśtva_of` map (jhal → its jaś counterpart), paralleling the
  existing `cartva_of` + `is_jhal` helpers. Does **not** fire on the voiceless
  junctions (those are cartva's `khar` arm), on vowel-initial endings, or on the
  `s`-meets-`s`/`th` clean cells (`Asse`, `Assva`, `AsTAH` — `s`/`th` are not
  jhaś).
- **Placement:** tripādī. Ordered relative to cartva (8.4.55) per the resolved
  sūtra id (see Risks / Ordering) — the two are independent (voiced vs voiceless
  junction, disjoint triggers) but the order is pinned by trace.
- **Citation — the slice's one open risk:** `8.4.53 *jhalāṃ jaś jhaśi*`
  (word-internal jaśtva before a voiced stop) is the grammatically precise id,
  since √ās's `s` is not pada-final. The parent spec wrote `8.2.39 *jhalāṃ
  jaśo'nte*` (pada-final jaśtva) for this junction. Which id the trace pins is
  **verified against ashtadhyayi.com's `AdDve` prakriya at golden-block time** —
  the same discipline slice 5c used for its a-augment id. The rule ships as a
  self-guarding `Rule` in `TINANTA_RULES` regardless of which id the reference
  assigns.

### Ordering summary (delta only)

```
[aṅga / 7.1 band]
[new] 7.1.5 AtmanepadezvanataH   (Ja → at for non-a-final Atmanepada aṅga → Asate)
      7.1.3 Jo'ntaH              (unchanged; now declines for √ās — ending no longer starts J)
      ...
[tripādī]
[new] 8.4.53 JalAM jaS JaSi      (jaśtva: s → d before Dh → AdDve; id/placement pinned at write-time)
      8.4.55 Kari ca             (unchanged; cartva — voiceless junction, does not fire for √ās)
      8.3.15 KaravasAnayoH       (unchanged; final s → visarga: e.g. AsTAs → AsTAH)
```

If the reference pins the voiced junction as 8.2.39 instead, it moves earlier in
the tripādī (8.2 precedes 8.4); the `AdDve` / `Aste` traces assert whichever
order resolves. The two junction rules never both fire on one cell (voiced vs
voiceless triggers are disjoint), so their relative order is a citation/trace
concern, not a derivation-correctness one.

## Data

√ās is stored as the resolved upadeśa root plus gaṇa (adādi), pada
(ātmanepada), and artha (*upaveśane*), exactly as the earlier adādi roots — no
new storage shape — and mirrored as a row in `data/dhatupatha.tsv` with gaṇa
column `adadi` and pada column `atmanepada`. `derive` carries no new grammar
branch: the two new rules are self-guarding entries in `TINANTA_RULES`, fed by
the existing gaṇa tagging (`Tag::Adadi`) and the existing pada threading. The
upadeśa-form path in `panini-data/src/lib.rs` (`(Uttama, Dvi) => "vas"` etc.) is
unchanged. Root count 27 → 28.

## Testing

- **Golden paradigm** (`crates/panini/tests/paradigm.rs`): one √ās block, 36
  forms, 972 → 1008. Each cell verified against ashtadhyayi.com at write time —
  `Asate`, `AdDve`, `AdDvam` above all (the cells the new rules produce).
- **Ordered trace** (`crates/panini/tests/trace.rs`): two new pins, one per new
  mechanism —
  - `Asate` — the 7.1.5 showcase: … → **7.1.5 anataḥ** (`Ja → at`) → 7.1.3
    declines → `Asate`.
  - `AdDve` — the voiced-junction showcase: … → **jaśtva** (`s → d` before
    `Dh`). Pins the id the reference assigns (①'s Risk).
  - `Aste` needs no new trace (bare adādi-ātmanepada luk, covered by golden);
    vidhiliṅ (`AsIta`, `AsIran`) reuses slice-2/5b `yāsuṭ` / `ran` pins. Sūtra
    ids/names in every new trace match ashtadhyayi.com.
- **Unit / negative pins** (the 5b negative-pin discipline; chosen to kill
  exactly the new guard mutants):
  - 7.1.5: `*Asante` (rule didn't fire) INVALID; an existing thematic
    ātmanepada root's 3pl stays `-ante` (the *anataḥ* arm — non-a-final only —
    does not leak to a-final aṅgas), asserted `assert_eq` against the slice-3
    golden.
  - jaśtva: `*AsDve` (junction not applied) and `*ADve` (over-applied as lopa)
    INVALID; a voiceless-junction / clean cell (`Asse`) is untouched by jaśtva
    (that arm is cartva's / needs a jhaś).
  - wrong-pada cross: `*Asati` (√ās forced parasmaipada) INVALID.
- **Mutation testing** (`mise run mutants`) at slice end: the new guard arms
  (7.1.5's *anataḥ* + ātmanepada + leading-`J` test; jaśtva's jhal-final +
  jhaś-following test, and the `jaśtva_of` map) must reach **0 survivors** on the
  target regions. `--timeout 60` is already set in `[tasks.mutants]` (5c).
- **Static gates unchanged:** `fmt-check`, `lint`, `audit`,
  `#![forbid(unsafe_code)]`, SLP1-only internal representation. No existing
  bhvādi / divādi / tudādi / adādi(√yā,√vā,√ad) form or trace changes — both new
  rules are guarded to fire only on √ās's non-a-final ātmanepada junction.
  `panini-lipi` roundtrip/property tests untouched (no new phonemes; `d` / `dh`
  already supported).

## Risks

1. **Voiced-junction sūtra id (②).** The one genuinely open citation: `8.4.53
   *jhalāṃ jaś jhaśi*` (this design's analysis — the `s` is word-internal, not
   pada-final) vs `8.2.39 *jhalāṃ jaśo'nte*` (the parent spec's label). Pinned
   against ashtadhyayi.com's `AdDve` prakriya before the trace is committed. It
   also fixes the rule's tripādī placement (8.2 precedes 8.4.55; 8.4.53 sits just
   before it) — the `AdDve` / `Aste` traces assert whichever order resolves.
2. **√ās form verification.** Confirm all 36 forms against the reference at write
   time, `AdDve` / `AdDvam` / `Asate` most of all — these are the cells the new
   rules produce.
3. **7.1.5 vs 7.1.3 interaction for a-final ātmanepada.** 7.1.5 must decline for
   the existing thematic ātmanepada roots (so `laBante` etc. stay unchanged) —
   pinned by the `*Asante`/`laBante` negative and by the untouched slice-3
   goldens.
