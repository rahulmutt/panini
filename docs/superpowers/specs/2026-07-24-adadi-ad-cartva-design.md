# Adādi gaṇa — Slice 5c (√ad, cartva)

**Status:** Design, approved in brainstorming 2026-07-24.

Builds on `2026-07-22-adadi-gana-design.md` (the slice-5 design) and the two
realized adādi sub-slices, `2026-07-22-adadi-gana-5a-aluk-core.md` (aluk core)
and `2026-07-23-adadi-vidhilin-5b-design.md` (vidhiliṅ ungate). Also depends on
`2026-07-20-vidhilin-lakara-design.md` (slice 2), whose yāsuṭ / jus chain this
slice reuses unchanged for a consonant-final aṅga.

## Summary

Slices 5a/5b completed adādi's two ā-final parasmaipadī roots (√yā, √vā) across
all four lakāras. The full adādi spec bundles three remaining pieces: the
consonant-final junction sandhi (√ad + cartva, this slice), the voiced junction
(√vas + jaśtva), and the ātmanepada side (√ās, √śī). This sub-slice takes the
**consonant-final junction** — √ad, the gaṇa's namesake — because it lands the
engine's first **internal junction sandhi**, **8.4.55 *khari ca* (cartva)**, a
maximally general rule every later gaṇa/subanta/compound slice reuses
immediately.

√ad is the first root whose tiṅ ending meets the root's final consonant
directly, so it is also the first to exercise phonology the thematic gaṇas kept
hidden behind an a-final vikaraṇa. The full paradigm needs **three** new rules,
not the one the parent spec's √ad row sketched — but the fourth lakāra
(vidhiliṅ) comes free, and the parent's fourth rule (8.2.23) is already in the
engine and never actually fires for √ad (the a-augment preempts it). See
"Rules" below.

The slice ships √ad's full 36 cells in one golden block, so √ad never needs a
second visit — the uniform-paradigm invariant slice 5b deliberately
established.

## Scope

Unchanged from 5a/5b: adādi (gaṇa 2), parasmaipada, all four lakāras (laṭ, laṅ,
loṭ, vidhiliṅ), all nine *puruṣa* × *vacana* cells, single *pada* / *tiṅanta*.

New in this slice: one root, **√ad** (*bhakṣaṇe*), consonant-final — the first
such root in the engine — and with it the first internal junction sandhi.

Out of scope, deferred to the remaining adādi pieces: the voiced junction (√vas
+ 8.2.39 jaśtva), the ātmanepada side (√ās, √śī + 7.4.21 / 7.1.6), and
everything the parent spec already defers (other gaṇas, ubhayapadī roots,
āśīrliṅ, …). This slice adds no new gaṇa, no new pada, and no voiced-junction
rule.

Coverage: golden paradigm **936 → 972** forms (1 root × 36 cells). Root count
26 → 27.

## Paradigm map — where the 36 cells come from

√ad's forms, verified cell-by-cell against ashtadhyayi.com when the golden block
is written (order: 3s 3d 3p / 2s 2d 2p / 1s 1d 1p, SLP1):

| Lakāra    | forms |
|-----------|-------|
| laṭ       | `atti attaH adanti` / `atsi atTaH atTa` / `admi advaH admaH` |
| laṅ       | `Adat AttAm Adan` / `AdaH Attam Atta` / `Adam Adva Adma` |
| loṭ       | `attu attAm adantu` / `adDi attam atta` / `adAni adAva adAma` |
| vidhiliṅ  | `adyAt adyAtAm adyuH` / `adyAH adyAtam adyAta` / `adyAm adyAva adyAma` |

The cells split three ways:

- **vidhiliṅ (9) — free.** √ad is consonant-final, so slice-2's yāsuṭ chain
  lands `y` directly on `d` (`adyAt`, `adyAH`, …) and the 3pl rides the existing
  jus path (`adyuH`). No vowel junction, no cartva (`y` is not a khar). This
  lakāra needs **zero** new rules — it just extends slice 2 to a consonant-final
  aṅga.
- **laṭ / loṭ / laṅ (27) — the new work.** These are where the ending meets `d`
  directly. ~11 cells fire cartva; one fires her dhiḥ (`adDi`); two fire the laṅ
  a-augment (`AdaH`, `Adat`); the remaining vowel-initial-ending cells
  (`adanti`, `adAni`, `Adam`, `Adan`, `advaH`, `admi`, …) are already clean.

## Rules — three new, and *not* 8.2.23

Only three rules fire on √ad's goldens.

### ① 6.4.101 *hujhalbhyo her dhiḥ* — loṭ 2sg `adDi` (addhí)

3.4.87 *ser hyapic ca* already turns the loṭ 2sg `si` → `hi`. For thematic
roots, 6.4.105 *ato heḥ* then luks that `hi` behind śap's short `a`
(`tinanta.rs`, guarded to a short `a`; the √yā note there records that `hi`
survives after `ā`). √ad is the first root where `hi` survives to meet a
*consonant*, so 6.4.101 converts hi → dhi: ad + hi → ad + dhi → `adDi`.

- **Guard:** aṅga-final ∈ jhal (√ad's `d` qualifies).
- **Placement:** aṅga layer, as 6.4.105's sibling (both are heḥ-operations on
  the loṭ 2sg). Ordered so it acts only when 6.4.105 has declined.

### ② laṅ singular a-augment — laṅ 2sg / 3sg `AdaH` / `Adat`

The laṅ singular endings reduce to a single consonant (2sg `s`, 3sg `t`). After
the aṭ-augmented stem `Ad`, a bare `Ad + s` / `Ad + t` leaves a word-final
consonant cluster that the **pre-existing** 8.2.23 *saṃyogāntasya lopaḥ*
(`tinanta.rs`, already in the array — it strips `aBavant` → `aBavan`) would
reduce to bare `Ad`, collapsing 2sg = 3sg = 1sg-stem. To prevent that, √ad
inserts an `a` before the ending: `Ad + a + s` → `Adas`, `Ad + a + t` → `Adat`.
The inserted `a` makes the word vowel-final, so 8.2.23 — which checks the last
two characters of `p.text()` and declines when either is a vowel — correctly
does **not** fire. This is Whitney's explicitly-special insertion; it does
**not** generalize (√vas, √dviṣ do not insert).

- **Guard:** `Tag::Adadi` ∧ lakāra = laṅ ∧ consonant-final aṅga ∧
  single-consonant ending ∈ {`s` (2sg), `t` (3sg)}. Neither `Context` nor `Term`
  carries a root code, so — following the codebase's structural-guard precedent
  (only √div reaches 8.2.77) — the guard is written structurally; in the current
  root set (adādi's only consonant-final root is √ad; √yā/√vā are ā-final) it
  selects exactly √ad. When √vas lands it is retightened, pinned by a negative
  mutation guard. The length guard separates 2sg/3sg from the multi-char endings
  `tam` / `tAm` / `ta` (2du/3du/2pl), which keep their cluster-free junction and
  go through cartva (`Attam`, `AttAm`, `Atta`).
- **Placement:** aṅga layer — necessarily **before** cartva (③), so the inserted
  `a` shields the `d`.
- **Sūtra id:** a √ad-specific aṭ-vārttika; the exact id/name is pinned against
  ashtadhyayi.com's अदत् / अदः prakriya at golden-block time (see Risks). The
  rule ships as a self-guarding `Rule` in `TINANTA_RULES` regardless of which id
  the reference assigns.

### ③ 8.4.55 *khari ca* (cartva) — the headline junction, ~11 cells

A root-final jhal that meets a *khar* (here `t` / `th` / `s`) assimilates to the
corresponding first-varga voiceless stop: `d` → `t`. Fires across laṭ (`atti`,
`atsi`, `attaH`, `atTaH`, `atTa`), loṭ (`attu`, `attAm`, `attam`, `atta`), and
laṅ dual/plural (`Attam`, `AttAm`, `Atta`). General, reusable tripādī rule —
every later gaṇa/subanta/compound slice inherits it.

- **Guard:** aṅga-final ∈ jhal ∧ following segment ∈ khar. Does **not** fire on
  vowel-initial endings (`adanti`) or on `m` / `v` (`admi`, `advaH` — not khar),
  nor on the a-augmented laṅ singular (`Adat`: `d` is now before `a`).
- **Placement:** tripādī, the engine's first 8.x rule. Ordered after ② by
  construction (aṅga before tripādī); its interaction with the existing
  final-`s` → visarga machinery is independent (both act on `attas`) but the
  ordering is asserted (see Risks).

### On 8.2.23 *saṃyogāntasya lopaḥ* (already present)

The parent spec's √ad row listed 8.2.23 alongside cartva. It is already in the
array (it produces `aBavan` etc.), so this slice adds no 8.2.23 code. Its only
role here is as the rule the a-augment (②) must **preempt**: without the
inserted `a`, `Adt` / `Ads` would hit 8.2.23 and collapse to `Ad`; with it, the
word is vowel-final and 8.2.23 declines. So across all 36 √ad forms, 8.2.23
never actually fires — the a-augment shields every one. The three *new* rules
this slice adds remain ①/②/③; 8.2.23 is unchanged.

### Ordering summary

```
[aṅga layer]
  ... existing sārvadhātuka / aṅga rules ...
  6.4.105 ato heH            (unchanged; short-a guard declines for √ad)
[new] 6.4.101 her dhiH       (loṭ 2sg: hi → dhi → adDi)
[new] laṅ a-augment (adādi)  (laṅ 2sg/3sg: Ad+s/Ad+t → Adas/Adat)
[tripādī]
  8.2.23 saMyogAntasya lopaH (unchanged; a-augment makes it decline for √ad)
  8.3.15 KaravasAnayoH       (unchanged; final s → visarga: Adas → AdaH)
[new] 8.4.55 khari ca        (cartva: d → t before khar → atti, Attam, ...)
```

## Data

√ad is stored as the resolved upadeśa root plus gaṇa (adādi), pada
(parasmaipada), and artha (*bhakṣaṇe*), exactly as the earlier adādi roots — no
new storage shape. `derive` carries no new grammar branch: the three new rules
are self-guarding entries in `TINANTA_RULES`, fed by the existing gaṇa tagging
(`Tag::Adadi`).

## Testing

- **Golden paradigm** (`crates/panini/tests/paradigm.rs`): one √ad row, 36
  forms, 936 → 972. Each cell verified against ashtadhyayi.com at write time
  (`Adat` / `AdaH` / `adDi` especially).
- **Ordered trace** (`crates/panini/tests/trace.rs`): three new pins, one per
  new mechanism —
  - `atti` — the cartva showcase: 3.1.68 → 2.4.72 luk → … → **8.4.55 cartva**.
  - `adDi` — the her-dhiḥ path: 3.4.87 *ser hi* → 6.4.105 declines → **6.4.101
    *her dhiḥ***.
  - `Adat` — the a-augment ordering proof: laṅ augment → **a-augment** →
    **8.4.55 cartva declines** (`d` now before `a`). Pins ② before ③.
  - vidhiliṅ (`adyAt`, `adyuH`) needs no new trace — it reuses slice-2's yāsuṭ /
    jus pins; golden coverage suffices. Sūtra ids/names in every new trace match
    ashtadhyayi.com.
- **Unit tests** (guard boundaries, the 5b negative-pin discipline):
  - cartva: `*adti` (not applied) and `*atmi` (over-applied to non-khar `m`)
    stay INVALID.
  - her dhiḥ: `*adhi` (hi not converted) INVALID; guard is jhal-final only.
  - a-augment: `*ADt` / `*Ad` (saṃyogānta collapse) INVALID; `*Adatam`
    (a-augment leaking into 2du) INVALID — pins the single-consonant length
    guard.
- **Mutation testing** (`mise run mutants`) at slice end: the new guard arms
  (cartva's khar test, her-dhiḥ's jhal test, the a-augment's root+lakāra+length
  guard) must reach **0 survivors** on the target regions. This slice also adds
  `--timeout 60` to `[tasks.mutants]` in `mise.toml`: the auto-set 20 s
  per-mutant timeout is under the ~30 s workspace suite, producing false
  `Timeout`s and a nonzero exit despite 0 missed. The bump makes the gate report
  clean.
- **Static gates unchanged:** `fmt-check`, `lint`, `audit`,
  `#![forbid(unsafe_code)]`, SLP1-only internal representation. No existing
  bhvādi/divādi/tudādi/adādi(√yā,√vā) form or trace changes — the three new
  rules are guarded to fire only on √ad's consonant junction.

## Risks

1. **a-augment sūtra id (②).** The one genuinely open citation. It is a
   √ad-specific aṭ-vārttika; its id/name is pinned against ashtadhyayi.com's
   अदत् / अदः prakriya before the `Adat` trace is finalized. The trace pin is not
   committed until it matches the reference.
2. **√ad form verification.** Confirm all 36 forms against the reference at
   write time, `Adat` / `AdaH` / `adDi` most of all — these are the cells the
   new rules produce.
3. **cartva tripādī placement.** 8.4.55 is the engine's first 8.x rule; assert
   its order relative to the existing final-`s` → visarga machinery (independent
   operations on `attas`, but the ordering is pinned by the `atti` / `attaH`
   traces).
