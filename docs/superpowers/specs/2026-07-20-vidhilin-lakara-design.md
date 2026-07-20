# Vidhiliṅ lakāra (Phase 2, slice 2)

Status: approved, not yet planned
Supersedes: nothing. Extends `2026-07-19-panini-astadhyayi-design.md` (v1) and
`2026-07-20-lan-lot-lakaras-design.md` (Phase 2, slice 1).

## Goal

Extend the tiṅanta engine from three lakāras to four: add **vidhiliṅ**
(optative — bhavet "may he be / he should be") alongside laṭ, laṅ, and loṭ.

## Scope

Unchanged from slice 1: *bhvādi* (gaṇa 1), *parasmaipada*, the 6 curated roots
(`BU`, `nI`, `ji`, `smf`, `paW`, `vad`), single *pada* input, on-the-fly
derivation, all three *puruṣa* × three *vacana*.

Golden coverage grows 162 → **216 forms** (4 lakāras × 6 roots × 9 cells).

Pāṇini's liṅ covers two semantically distinct uses with different derivations:
**vidhiliṅ** (optative, sārvadhātuka: bhavet) and **āśīrliṅ** (benedictive,
ārdhadhātuka: bhūyāt). This slice implements only vidhiliṅ, and the `Lakara`
enum names it `VidhiLin` — treating the two as separate lakāra values from day
one, so adding āśīrliṅ later is purely additive (no rename, no mode flag).

Out of scope, deferred to later slices: āśīrliṅ, *ātmanepada*, other gaṇas,
other lakāras (lṛṭ, luṅ, liṭ, …), *subanta*, compounds, the precomputed FST
layer.

## Why vidhiliṅ next

It completes the four common sārvadhātuka lakāras while staying inside the
machinery slice 1 built: śap, the ṅit ending substitutions, and the fixed
ANGA/SHAP/ENDING term vector. No iṭ-āgama, no ārdhadhātuka behavior, no new
axes. All six curated roots are fully regular in vidhiliṅ.

What it adds that slice 1 did not have: an ending-side **āgama with its own
downstream rule chain**. Vidhiliṅ's endings are built from the yāsuṭ-āgama
(3.4.103), which is then reshaped by 7.2.79 (s-elision), 7.2.80 (yA → iy after
a), 6.1.87 (a + i → e), and 6.1.66 (y dropped before a consonant):

```
tip → ti → t → yAst → yAt → iyt ;  Bava + iyt → Bave + yt → Bavet
```

## Architecture

Unchanged. Vidhiliṅ is six new self-guarding `Rule`s in `TINANTA_RULES` plus
two guard widenings; `derive`, `run_pipeline`, `Prakriya`, and `Term` are
untouched.

Yāsuṭ is modeled as a **text prefix on the ending term**, exactly as the āṭ
(3.4.92) and aṭ (6.4.71) āgamas already are, so the ANGA/SHAP/ENDING indices
stay stable for every rule. The alternative — a fourth `Term` — was rejected
for the same reason the 6.4.71 comment records: nothing in this slice reads the
āgama as a separate morpheme, and a shifting term vector would destabilize
every later rule.

`Context::new` initialises `is_ngit_like = true` for `VidhiLin`: liṅ, like
laṅ, is a ṅit lakāra by its own name (the anubandha ṅ), so no atideśa rule is
needed or traced — unlike loṭ, which gets ṅit-likeness from 3.4.85.

## The rules

Ordered as they appear in `TINANTA_RULES`. **Bold** = new in this slice.
*(widened)* = existing rule whose guard changes.

| #  | Sūtra                                | Effect                                                        | Guard                |
|----|--------------------------------------|---------------------------------------------------------------|----------------------|
| 1  | 3.4.78 tiptasjhi…                    | lakāra → base tiṅ ending                                      | all                  |
| 2  | 1.3.3 halantyam + 1.3.9 tasya lopaḥ  | elide the ending's anubandhas (1.3.4 protects vibhakti finals) | all                 |
| 3  | 3.4.85 loṭo laṅvat                   | set `is_ngit_like`                                            | loṭ                  |
| 4  | **3.4.108 jher jus**                 | Ji → jus; it-saṃjñā elides the initial j (cuṭū) → us          | vidhiliṅ             |
| 5  | 3.4.101 tas-thas-tha-mipām *(widened)* | tas→tAm, Tas→tam, Ta→ta (ṅit-like); mip→am (ṅit-like **except loṭ**) | see effect |
| 6  | 3.4.99 nityaṃ ṅitaḥ                  | vas→va, mas→ma                                                | ṅit-like             |
| 7  | 3.4.87 ser hyapic ca                 | si→hi                                                         | loṭ                  |
| 8  | 3.4.89 mer niḥ                       | mi→ni                                                         | loṭ                  |
| 9  | 3.4.86 er uḥ                         | ti→tu, Ji→Ju                                                  | loṭ                  |
| 10 | 3.4.100 itaś ca *(widened)*          | elide final i (ti→t, si→s; laṅ Ji→J — liṅ's Ji is gone by #4) | ṅit-like except loṭ  |
| 11 | 3.4.92 āḍ uttamasya pic ca           | prefix `A` to uttama ending                                   | loṭ                  |
| 12 | **3.4.103 yāsuṭ parasmaipadeṣu…**    | prefix `yAs` to the (already substituted) ending              | vidhiliṅ             |
| 13 | 3.1.68 kartari śap                   | insert śap; it-saṃjñā; mark aṅga — **the index boundary**     | all                  |
| 14 | 6.4.71 luṅ-laṅ-lṛṅkṣv aḍ-udāttaḥ     | prefix `a` to the aṅga                                        | laṅ                  |
| 15 | 7.1.3 jho'ntaḥ                       | leading J → ant                                               | all (inert for liṅ)  |
| 16 | **7.2.79 liṅaḥ salopo 'nantyasya**   | elide the ending's non-final s: yAst→yAt, yAss→yAs, yAsus→yAus | vidhiliṅ            |
| 17 | **7.2.80 ato yeyaḥ**                 | śap `a`: leading `yA` → `iy` (yAt→iyt, yAus→iyus)             | vidhiliṅ             |
| 18 | 7.3.84 sārvadhātukārdhadhātukayoḥ    | aṅga-final ik → guṇa                                          | all                  |
| 19 | 6.1.78 eco'yavāyāvaḥ                 | e/o/E/O → ay/av/Ay/Av                                         | all                  |
| 20 | 7.3.101 ato dīrgho yañi              | śap `a` → `A` before m/v                                      | laṭ, laṅ             |
| 21 | 6.1.101 akaḥ savarṇe dīrghaḥ         | śap `a` + `A` → `A`                                           | loṭ uttama           |
| 22 | 6.1.97 ato guṇe                      | śap `a` + `a` → single `a`                                    | all (inert for liṅ)  |
| 23 | **6.1.87 ād guṇaḥ**                  | śap `a` + ending-initial `i` → śap `e`, strip the `i`         | shape (liṅ only in practice) |
| 24 | **6.1.66 lopo vyor vali**            | ending-initial `y` before a consonant is dropped; `yus` survives (vowel follows) | shape |
| 25 | 6.4.105 ato heḥ                      | elide `hi` after `a`                                          | loṭ                  |
| 26 | 8.2.23 saṃyogāntasya lopaḥ           | drop final of a word-final conjunct                           | all                  |
| 27 | 8.3.15 kharavasānayoḥ                | word-final s → visarga                                        | all                  |

### Guard widenings on existing rules

The laṅ-only guards on #5 and #10 were hiding a ṅit generalization; vidhiliṅ
is the second ṅit lakāra and forces it into the open.

- **3.4.100 itaś ca:** `Lakara::Lan` → `is_ngit_like && !Lot`. Loṭ must stay
  excluded *explicitly*: its i-final endings belong to the apavāda set
  3.4.86/87/89, and 3.4.87's output `hi` is itself i-final — a bare ṅit guard
  would corrupt it to `h`.
- **3.4.101 mip arm:** `"mi" if Lan` → `"mi" if !Lot`. Vidhiliṅ needs mi→am
  (Baveyam); loṭ keeps its 3.4.89 apavāda (mi→ni).

### Ordering decisions that carry weight

Each was derived by hand-working all 9 vidhiliṅ cells; getting any of them
backwards produces a wrong surface form.

- **#4 before #10.** *jher jus* is the apavāda to *itaś ca* for liṅ's `Ji`
  (which is i-final). Run #10 first and liṅ 3pl derives from `J` instead of
  `us`. Same pattern as the existing #7/#8-before-#9 preemption.
- **#12 after #4–#10.** The pipeline applies the ending substitutions *first*
  and then prefixes yāsuṭ onto the result. The substitution guards match the
  ending text exactly (`"mi"`, `"vas"`, …); prefixing yāsuṭ first would make
  every one of them miss (`yAsmi` ≠ `mi`). Same reasoning that scoped 3.4.92's
  guard in slice 1.
- **#16 before #17.** 7.2.80 rewrites the leading `yA`, which only exists as a
  prefix-shape once 7.2.79 has removed the s of `yAs` (yAst→yAt). Reversed,
  `yAst` matches nothing and every form keeps a spurious s.
- **#23 before #24.** 6.1.66 needs the ending to *start* with `y` (`yt`, `yva`),
  which only happens after 6.1.87 absorbs the leading `i` into the śap. Before
  that the ending is `iyt` and #24 can never fire.
- **#20 needs no liṅ exclusion.** At #20 the liṅ ending is still `yA…`-initial
  (or `iy…`-initial), never m/v-initial, so the existing shape guard is
  sufficient — unlike loṭ, which needed an explicit exclusion in slice 1.
- **#22 and #25 are inert for liṅ by state, not by guard:** when they run on a
  liṅ derivation the śap is already `e` (from #23), so their `SHAP == "a"`
  guards fail. No change needed.

### Reference verification

Per AGENTS.md, sūtra ids and names in traces must match ashtadhyayi.com. The
six new/bolded sūtras (3.4.108, 3.4.103, 7.2.79, 7.2.80, 6.1.87, 6.1.66) were
verified against the reference (via the `ashtadhyayi.github.io` mirror and its
underlying `sanskrit/ashtadhyayi` data source, since ashtadhyayi.com itself is
a JS SPA that plain fetch cannot render — see task-1-report.md for the
per-sūtra sourcing). All six ids do what this table's Effect column claims.
Five of the six draft SLP1 names were exact; one needed a correction: 3.4.103's
name is a **four-word** name with a space after the yāsuṭ āgama word, not
fused to the following word —

```
yAsuw parasmEpadezUdAtto Nic ca
```

(not `yAsuwparasmEpadezUdAtto Nic ca` — the reference text
यासुट् परस्मैपदेषूदात्तो ङिच्च shows a space between यासुट् and परस्मैपदेषूदात्तो).
This is the literal to use verbatim for rule #12 in Tasks 3–7. The other five
confirmed literals: `Jer jus` (3.4.108), `liNaH salopo'nantyasya` (7.2.79),
`ato yeyaH` (7.2.80), `Ad guRaH` (6.1.87), `lopo vyor vali` (6.1.66).

New names must satisfy the existing SLP1 digraph gate and the
trace-name-matches-`TINANTA_RULES` gate; every recorded id needs a rule entry.
The j-elision inside #4 is recorded as 1.3.9 (the elision rule), matching the
existing convention that 1.3.3/1.3.7 assign saṃjñā silently. `run_it_samjna`
may need a cuṭū (1.3.7) arm for the initial `j` of `jus`; extend it if absent.

## Data and facade changes

**`panini-data`:** `Lakara` gains `VidhiLin`. `tin_ending` is **unchanged** —
vidhiliṅ starts from the same nine base tiṅ endings, and all divergence is
rule-driven. As in slice 1, the data layer not growing is a check on the rule
decomposition being right.

**`panini-analyze`:** `LAKARAS` grows to 4; candidates per check 162 → 216.
Still trivially fast; no narrowing logic needed.

**`panini` facade:** `lakara_name` gains `VidhiLin => "viDiliN"` (SLP1).
`Analysis`, `CheckResult`, and the CLI are otherwise unchanged — the lakāra
flows through existing plumbing.

## Testing

- **Golden paradigm** (`crates/panini/tests/paradigm.rs`) extends to 216 forms
  keyed by `(root, lakara)`. The 54 new forms are verified against
  ashtadhyayi.com before being committed. The 162 existing forms are the safety
  net for the guard widenings and must keep passing throughout.
- **Negative forms.** Add cross-lakāra confusions around liṅ: `aBavet` (laṅ
  augment on a liṅ form) and `Bavetu` (loṭ ending on a liṅ stem) must be
  INVALID. No currently-asserted negative becomes valid (checked: `aBavatu`,
  `Bavat`, `aBavanti`, `gacCati` are not vidhiliṅ forms).
- **Trace test** (`crates/panini/tests/trace.rs`) pins the full ordered sūtra
  sequence for the three distinct liṅ paths: `Bavet` (vali-lopa path),
  `BaveyuH` (jus path — the y survives), `Baveyam` (widened mip arm).
- **Roundtrip** (`crates/panini/tests/roundtrip.rs`) extends over the 4-lakāra
  axis.
- **Unit tests** in `tinanta.rs`: the existing name-gate and digraph tests
  cover the new rules once their lakāra arrays include `VidhiLin`; add targeted
  tests for 7.2.79 (elides only the *non-final* s of `yAss`) and 6.1.66
  (fires on `yva`, not on `yus`).
- **Mutation testing** (`mise run mutants`) remains the real gate: with 27
  guarded rules the likely defect class is a dropped or inverted guard.

### Expected forms (BU), for reference

- **vidhiliṅ:** Bavet, BavetAm, BaveyuH, BaveH, Bavetam, Baveta, Baveyam,
  Baveva, Bavema

## Success criteria

- All 216 golden forms validate as VALID with a correct, ordered sūtra trace.
- Every sūtra id and name in a trace matches ashtadhyayi.com.
- The 162 pre-existing forms are byte-identical (the guard widenings are pure
  generalizations).
- Cross-lakāra non-forms (`aBavet`, `Bavetu`) return INVALID.
- Mutation testing shows the new guards are pinned by tests.
- Single-word `check` remains interactive-fast.

## Risks

- **The guard widenings touch laṅ/loṭ behavior.** 3.4.100 and 3.4.101 are load-
  bearing for three lakāras after this change. The 162 existing golden forms
  pin them; run them before adding the new forms, not only after.
- **7.2.79 over-elision.** "Elide the non-final s" must not eat the final s of
  the sip cell (`yAss` → `yAs`, not `yA`). Pinned by the `BaveH` golden cell
  and a unit test.
- **Sūtra attribution.** A right form by the wrong rule is invisible to the
  golden test; the pinned traces and reference verification are the only
  guards, exactly as in slice 1.
