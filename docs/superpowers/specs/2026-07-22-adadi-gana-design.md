# Adādi gaṇa (Phase 2, slice 5)

**Status:** Design, approved in brainstorming 2026-07-22.

Builds on `2026-07-19-panini-astadhyayi-design.md` (v1 vertical slice),
`2026-07-20-lan-lot-lakaras-design.md` (slice 1),
`2026-07-20-vidhilin-lakara-design.md` (slice 2),
`2026-07-20-atmanepada-design.md` (slice 3), and
`2026-07-21-divadi-tudadi-ganas-design.md` (slice 4).

## Summary

Add **adādi** (gaṇa 2), the *aluk* gaṇa, alongside the existing bhvādi (1),
divādi (4), and tudādi (6). Where slice 4 turned on swapping one a-final
vikaraṇa for another, adādi has **no vikaraṇa at all in surface**: by
**2.4.72 *adiprabhṛtibhyaḥ śapaḥ*** the śap that 3.1.68 introduces undergoes
*luk*, so the tiṅ ending attaches directly to the root:

- bhvādi: BU + **śap** → guṇa → **bhavati**
- adādi: ad + śap → **luk** (2.4.72) → **atti**

That single *luk* is the gaṇa's identity, and it exposes something no prior
slice has seen: a **direct root+ending junction**. Every earlier gaṇa kept an
a-final vikaraṇa (śap/śyan/śa) between root and ending, so a consonant-final
root never met a consonant-initial ending. adādi removes that buffer, so the
engine needs its first **internal junction sandhi**. This slice takes on that
junction in a deliberately curated, minimal form — one general voiceless rule
(cartva) and one general voiced rule (jaśtva) — rather than the full ṣṭutva /
*dādho dhātoḥ* apparatus the irregular aluk stars would demand.

## Scope

Unchanged from slice 4: single *pada*, *tiṅanta*, the four lakāras (laṭ, laṅ,
loṭ, vidhiliṅ), both padas (parasmaipada, ātmanepada), all nine *puruṣa* ×
*vacana* cells.

New in this slice: one gaṇa (adādi), with three parasmaipadī and three
ātmanepadī roots, giving the full 4 lakāras × 2 padas × 9 cells for each new
root.

Out of scope, deferred to later slices: the irregular aluk stars — √as
(*asti / santi*, 6.4.111 a-lopa), √han (*hanti / ghnanti*), √brū (*bravīti*,
7.3.93 ī-augment), √i (*eti / yanti*, 6.4.81 iyaṅ/yaṇ), √dviṣ / √duh / √lih
(ubhayapadī and/or 8.2.31 *ho ḍhaḥ* / 8.2.32 *dādho dhātoḥ* / ṣṭutva-heavy);
ubhayapadī roots and the intent-conditioned pada rules (1.3.72
*svaritañitaḥ...*); other gaṇas (curādi, juhotyādi, …); āśīrliṅ and other
lakāras. Root choice below is curated to avoid every one of these.

## Why adādi next

"More gaṇas" remains the roadmap item, and of the gaṇas still open adādi is the
one that extends the engine along a *new* axis for the smallest committed cost.
Slices 1–4 stayed inside the comfortable world where the tiṅ ending always sat
behind an a-final vikaraṇa. adādi is the first gaṇa to break that, and the
junction sandhi it forces — **cartva** (8.4.55) and **jaśtva** (8.2.39) — is
maximally general grammar that every later slice (more gaṇas, subanta,
compounds) reuses immediately. Taking it on now, against a curated root set that
needs only those two general rules plus one root's cited specials, buys that
foundational machinery cheaply and correctly.

The slice also reuses slice 4's payload in a satisfying way. The **1.1.5 *kṅiti
ca*** guard that slice 4 put on the guṇa rules — to let a ṅit vikaraṇa block
guṇa — is the very reason √śī needs its special **7.4.21 *śīṅaḥ [guṇaḥ]***: the
ṅit ātmanepada ending would otherwise block guṇa, and 7.4.21 is śī's targeted
override. The machinery built last slice is what makes this slice's one
irregular root come out right.

## Root set

Six new roots (3 P + 3 Ā), verified against ashtadhyayi.com / the Vidyut
Dhātupāṭha mirror when the golden block is written (as in slice 4). Stored, as
before, as the resolved upadeśa root plus gaṇa, pada, and artha. The table pins
only the laṭ 3sg surface; the full paradigms are pinned in the golden block.

| root (SLP1) | pada | artha | surface (laṭ 3sg) | new grammar exercised |
| --- | --- | --- | --- | --- |
| `yA` | parasmaipada | prāpaṇe | **yāti** | luk only (ā-final; no guṇa, no junction) |
| `vA` | parasmaipada | gati-gandhanayoḥ | **vāti** | luk only (ā-final) |
| `ad` | parasmaipada | bhakṣaṇe | **atti** | luk + **8.4.55** cartva (d+t → tt) |
| `As` | ātmanepada | upaveśane | **āste** | luk only (ā-final ātmanepada) |
| `vas` | ātmanepada | ācchādane | **vaste** | luk + **8.2.39** jaśtva (voiced junction) |
| `SI` | ātmanepada | svapne | **śete** | luk + **7.4.21** guṇa + **7.1.6** ruṭ + **6.1.78** ayādeśa |

Notes on the curation:

- **√ad** is the gaṇa's namesake and the clean consonant-final showcase: its only
  junction is d before the t of *ti*, resolved by the single most general
  voiceless-assimilation rule, **8.4.55 *khari ca*** (cartva) → **atti**.
- **√yā / √vā** are ā-final, so *luk* is the whole story — guṇa does not apply
  to ā and there is no consonant junction. They anchor the "gentle" end and
  demonstrate the bare 2.4.72 trace.
- **√ās** is the only genuinely clean ā-final ātmanepada root in gaṇa 2 and
  anchors the ātmanepada side.
- **√vas** carries the general voiced junction: its consonant-initial /
  voiced-initial ātmanepada cells (e.g. *dhve* / *dhvam*) meet the root-final
  *s*, resolved by **8.2.39 *jhalāṁ jaśo'nte*** (jaśtva). Like cartva, this is
  general grammar, not root-specific.
- **√śī** (śīṅ) is the one irregular root, admitted to keep the tidy 3+3 /
  +216-golden invariant, on the slice-4 precedent of accepting a targeted
  special (8.2.77 *hali ca* for div). It needs two cited root-specifics —
  **7.4.21 *śīṅaḥ [sārvadhātuke guṇaḥ]*** (forces guṇa śī → śe despite the ṅit
  ending, → *śete*) and **7.1.6 *śīṅo ruṭ*** (the *ruṭ*-augment of the 3pl
  *jha*, → *śerate*) — plus the already-present **6.1.78 *eco'yavāyāvaḥ***
  (*ayādeśa*, śe + āte → *śayāte*).

**On guṇa in this slice.** With śap gone, root-final guṇa is conditioned by the
following tiṅ ending directly (7.3.84). None of the three parasmaipadī roots has
an *ik* final (yā/vā are ā-final, ad is d-final), so parasmaipada guṇa never
visibly fires. On the ātmanepada side the endings are ṅit (1.2.4), so 1.1.5
blocks guṇa — except for √śī, whose 7.4.21 override is exactly what makes *śete*
appear. So the slice's guṇa story is carried entirely by √śī, by design.

Coverage: 24 → **30** roots; golden paradigm 864 → **1080** forms (6 new roots ×
4 lakāras × 9 cells = 216 new).

## Data layer (`panini-data`)

Small and mechanical, following the slice-4 pattern:

- `Gana` enum gains an **`Adadi`** variant (currently `Bhvadi` / `Divadi` /
  `Tudadi`).
- Six new `Dhatu` entries in the curated set, each carrying its gaṇa, pada, and
  artha; mirrored as rows in `data/dhatupatha.tsv` with gaṇa column value
  `adadi` and the existing pada column.
- No structural change to `Dhatu`. The data layer continues to hold only
  upadeśa material; every surface change happens in the engine.
- `data/ATTRIBUTION.md` unchanged in structure; the new rows are cross-checked
  against the same ashtadhyayi.com / Vidyut sources already cited.

## Engine (`panini-prakriya`)

All changes are self-guarding `Rule`s in `TINANTA_RULES` (per AGENTS.md), or
guard additions to existing rules. No change to the term-index layout: for
adādi 3.1.68 still inserts śap at the `SHAP` slot, and 2.4.72 then empties that
slot by *luk*.

### Threading the gaṇa

The gaṇa is carried as a tag on the aṅga term, exactly as slice 4 threads
`Tag::Divadi` / `Tag::Tudadi`. `derive` tags the aṅga **`Tag::Adadi`** from
`dhatu.gana`. The new rules guard on `p.terms[ANGA].has(Tag::Adadi)`. No new
coordinate through `Context::new`.

### New rules

**2.4.72 adiprabhṛtibhyaḥ śapaḥ** — *luk* of śap for adādi. Guard:
`p.terms[ANGA].has(Tag::Adadi)` and a śap (pit vikaraṇa) present at `SHAP`. It
empties the vikaraṇa slot (luk leaves no residue), leaving the tiṅ ending
adjacent to the root. This is the apavāda-consuming counterpart to 3.1.68:
3.1.68 introduces śap as utsarga (unchanged), and 2.4.72 removes it for adādi.
Ordered immediately **after 3.1.68** (so the luk sees the śap that 3.1.68 just
inserted) and **before the guṇa band** (7.3.84 etc.), so all downstream
operations see the ending directly on the root. By 1.1.62
*pratyayalope pratyayalakṣaṇam* the luk'd śap would still be available to
condition affix-referring operations, but nothing in this curated slice relies
on it — guṇa is conditioned by the following tiṅ, not by śap.

**8.4.55 khari ca** (cartva) — a root-final jhal that meets a *khar*
(voiceless) ending-initial assimilates to the corresponding *car*. Fires for
√ad: d before the *t* of *ti/tas/...* → *t* → **atti**. General; placed in the
tripādī block. First internal-junction rule in the engine.

**8.2.39 jhalāṁ jaśo'nte** (jaśtva) — a pada-/junction-final jhal takes its
*jaś* (voiced) counterpart before a voiced segment. Fires for √vas at the
voiced-initial ātmanepada cells (the *dhve/dhvam* junction and word-final
positions). General; tripādī block, ordered before cartva per tripādī
sequencing (verified against ashtadhyayi.com when the golden block is written).

**7.4.21 śīṅaḥ [sārvadhātuke guṇaḥ]** — root-specific: √śī takes guṇa (śī → śe)
before a sārvadhātuka ending even though the ātmanepada ending is ṅit and 1.1.5
would otherwise block it. Guard: `Tag::Adadi` aṅga whose root text is `SI`.
Ordered in the guṇa band, as an override reached only for √śī.

**7.1.6 śīṅo ruṭ** — root-specific: the *jha* (3pl ātmanepada) of √śī takes the
*ruṭ*-augment → *śe-r-ate* → **śerate**. Guard: √śī aṅga with a *jha* ending.
Ordered in the 7.x augment band, before the ending's vowel resolves.

### Changed rules

**3.1.68 kartari śap** — no new guard needed for adādi: it fires as the utsarga
and inserts śap exactly as for bhvādi; 2.4.72 then luks it. (The slice-4
"no vikaraṇa already present" guard stays as is; adādi has no apavāda vikaraṇa
rule, so 3.1.68 fires.)

**7.3.84 sārvadhātukārdhadhātukayoḥ** / **7.3.86 pugantalaghūpadhasya ca** — no
change beyond slice 4. With the vikaraṇa luk'd, guṇa is now conditioned by the
tiṅ ending. For parasmaipadī adādi roots there is no *ik* to guṇate; for
ātmanepadī roots the ṅit ending blocks guṇa via the slice-4 1.1.5 guard —
except √śī, handled by 7.4.21 above. No edit to these rules is anticipated; the
golden block confirms.

### Rule order (delta only)

```
      3.1.68 kartari Sap               (utsarga: inserts śap for adādi too)
[new] 2.4.72 adipraBftiByaH SapaH      (guard: ANGA has Tag::Adadi; luk of śap)
      6.4.71 luNlaNlfNkzvaqudAttaH
      …
[new] 7.1.6  SINo ruw                  (guard: √śī, jha ending → śerate)
      …
[new] 7.4.21 SINaH [guNaH]             (guard: √śī; guṇa despite ṅit ending)
      7.3.84 sArvaDAtukArDaDAtukayoH    (unchanged; 1.1.5 guard from slice 4)
      7.3.86 pugantalaGUpaDasya ca      (unchanged)
      6.1.78 eco'yavAyAvaH              (unchanged; śe + āte → śayāte)
      …
[new] 8.2.39 JalAM jaSo'nte            (tripādī; jaśtva, √vas voiced junction)
[new] 8.4.55 Kari ca                   (tripādī; cartva, ad + ti → atti)
      8.2.23 saMyogAntasya lopaH
      8.3.15 KaravasAnayor visarjanIyaH
```

`TINANTA_RULES` gains five array entries (2.4.72, 7.1.6, 7.4.21, 8.2.39,
8.4.55). No existing rule's behavior changes for bhvādi/divādi/tudādi — all five
new rules are guarded on `Tag::Adadi` or on adādi-only junctions, so every
pre-existing golden and trace output is unchanged. (Exact tripādī ordering of
8.2.39 vs 8.4.55, and the precise 7.x placement of 7.1.6 / 7.4.21, are pinned
against ashtadhyayi.com when writing the golden block; the order above is the
default.)

## Analyzer, facade, CLI

No code change, by construction (as in slice 4):

- **`panini-analyze`** brute-forces every (root × lakāra × cell) and lets the
  engine confirm by exact match. New roots flow in through `dhatus()`
  automatically; the candidate set grows 864 → 1080, still trivially small.
- **`panini`** (facade): `Analysis` already carries pada and everything needed.
- **`panini-cli`**: `check` output shape unchanged. Devanāgarī / IAST / HK
  round-trips of the new forms come free from `panini-lipi` (no new phonemes;
  cartva/jaśtva outputs are all already-supported consonants).

## Testing

Follows the pattern of slices 1–4.

- **Golden paradigm** (`crates/panini/tests/paradigm.rs`): a new block per pada
  × lakāra for the six adādi roots — all 216 new forms pinned against the
  ashtadhyayi.com reference, total 1080. This is where every non-laṭ paradigm
  cell (especially the intricate √śī forms — *śayāte*, *śerate*, *aśeta*,
  *śayīta* — and the √ad / √vas augmented/junction cells) is verified.
- **Ordered trace** (`crates/panini/tests/trace.rs`): pin the full sūtra
  sequence for at least four representative derivations —
  - **atti** (3.1.68 → 2.4.72 luk → 8.4.55 cartva),
  - **yāti** (3.1.68 → 2.4.72 luk; no guṇa, no junction — the bare aluk trace),
  - **āste** (adādi ātmanepada; luk + ātmanepada ending machinery),
  - a **√śī** form — **śete** (7.4.21 guṇa) and/or **śerate** (7.1.6 ruṭ) and/or
    **śayāte** (6.1.78 ayādeśa) — exercising the specials.

  Sūtra ids and names checked against ashtadhyayi.com.
- **Junction / aluk negatives**: forms that must return INVALID —
  \**adati* / \**adyati* (śap must be luk'd, no surviving vikaraṇa; and no
  śa/śyan leak into adādi), \**yāyati*, cartva-not-applied \**adti*, and
  wrong-pada crosses for the new roots (\**āsati* / \**vasati* / \**śayati*
  parasmaipada; \**yāte* / \**atte* ātmanepada). Same style as slice-4
  cross-gaṇa / wrong-pada negatives.
- **Mutation testing**: `mise run mutants` at slice end. The new guard arms (the
  `Tag::Adadi` guard on 2.4.72, the √śī guards on 7.4.21 / 7.1.6, and the
  final-consonant / voicing guards in 8.4.55 cartva and 8.2.39 jaśtva) must not
  survive — the negatives above are chosen to kill exactly those mutants (e.g.
  \**adati* kills a mutated 2.4.72 guard; \**adti* kills a mutated cartva guard;
  \**śite* kills a mutated 7.4.21 guard).

`panini-lipi` roundtrip/property tests are untouched (no new phonemes).

## Success criteria

- All 1080 golden forms (30 roots × 9 tiṅ cells × 4 lakāras) validate `VALID`
  with correct ordered traces; the 864 pre-existing forms are unchanged.
- The new aluk / junction / wrong-pada non-forms return `INVALID`.
- No existing bhvādi / divādi / tudādi golden or trace output changes (verified
  by all five new rules being guarded on `Tag::Adadi` or adādi-only junctions).
- Every sūtra id and name in a trace matches ashtadhyayi.com.
- Mutation testing shows the new guards are genuinely pinned by tests.
