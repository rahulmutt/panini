# Laṅ and loṭ lakāras (Phase 2, slice 1)

Status: approved, not yet planned
Supersedes: nothing. Extends `2026-07-19-panini-astadhyayi-design.md` (v1).

## Goal

Extend the tiṅanta engine from one lakāra to three: add **laṅ** (imperfect) and
**loṭ** (imperative) alongside the existing **laṭ** (present).

## Scope

Unchanged from v1: *bhvādi* (gaṇa 1), *parasmaipada*, the 6 curated roots
(`BU`, `nI`, `ji`, `smf`, `paW`, `vad`), single *pada* input, on-the-fly
derivation, all three *puruṣa* × three *vacana*.

Golden coverage grows 54 → **162 forms** (3 lakāras × 6 roots × 9 cells).

Out of scope, deferred to later slices: *ātmanepada*, other gaṇas, other
lakāras (lṛṭ, vidhiliṅ, …), *subanta*, compounds, the precomputed FST layer.

## Motivating problem

`tinanta::derive` ignores its `_lakara` parameter entirely and is ~130 lines of
straight-line mutation in which every step implicitly assumes laṭ. Meanwhile
`Rule` (`rule.rs`) and `controller::run_pipeline` (`controller.rs`) exist but
are dead code — `run_pipeline` is called only from its own unit test. v1 shipped
the pipeline abstraction unexercised.

A second and third lakāra is the point at which that abstraction either becomes
real or should be deleted. Branching the existing procedure three ways would
scatter lakāra conditionals across a function that is already the largest in the
crate, and would make rule order implicit and unauditable.

## Architecture

The Aṣṭādhyāyī is one ordered rule set whose rules self-guard on context. The
engine adopts that shape.

### 1. `Context` on `Prakriya`

Rules cannot self-guard today: `apply` has signature
`fn(&mut Prakriya) -> bool`, and `Prakriya` holds only `terms` and `log`.

Add a `ctx` field:

```rust
pub struct Context {
    pub lakara: Lakara,
    pub pada: Pada,
    pub purusha: Purusha,
    pub vacana: Vacana,
    /// True inherently for laṅ (laṅ is ṅit); set for loṭ by 3.4.85 loṭo laṅvat.
    pub is_ngit_like: bool,
}
```

`is_ngit_like` is initialised `true` for laṅ, which is ṅit by nature. For loṭ it
is *not* a static property — it is set by rule 3.4.85, an *atideśa*. Modelling
loṭ's case as rule output rather than a match arm keeps that piece of grammar in
the rule list where it can be traced.

### 2. `TINANTA_RULES`: one ordered rule list

Every step in today's `derive` becomes a `Rule` with an explicit guard.
`tinanta::derive` shrinks to building the initial `Prakriya` (dhātu term +
context) and calling `run_pipeline(&mut p, TINANTA_RULES)`.

One list, one order, readable top-to-bottom against the Aṣṭādhyāyī. Rules that
apply to a single lakāra guard on `ctx.lakara`; rules shared by laṅ and loṭ
guard on `ctx.is_ngit_like`.

### 3. Trace

Unchanged. `Prakriya::record` already logs a `RuleStep` per application, so the
existing trace works as-is and now covers all three lakāras.

## The rules

Ordered as they appear in `TINANTA_RULES`. **Bold** = new in this slice.

| #  | Sūtra                              | Effect                              | Guard        |
|----|------------------------------------|-------------------------------------|--------------|
| 1  | 3.4.78 tiptasjhi…                  | lakāra → base tiṅ ending            | all          |
| 2  | 1.3.3 halantyam + 1.3.9 tasya lopaḥ | elide the ending's anubandhas, with 1.3.4 na vibhaktau tusmāḥ suppressing it for a vibhakti's final t/T/d/D/n/s/m | all |
| 3  | **3.4.85 loṭo laṅvat**             | set `is_ngit_like`                  | loṭ          |
| 4  | **3.4.99 nityaṃ ṅitaḥ**            | vas→va, mas→ma                      | ṅit-like     |
| 5  | **3.4.101 tas-thas-tha-mipām**     | tas→tAm, Tas→tam, Ta→ta (ṅit-like); mip→am (laṅ only) | see effect |
| 6  | **3.4.87 ser hyapic ca**           | si→hi                               | loṭ          |
| 7  | **3.4.89 mer niḥ**                 | mi→ni                               | loṭ          |
| 8  | **3.4.86 er uḥ**                   | final i→u (ti→tu, Ji→Ju)            | loṭ          |
| 9  | **3.4.100 itaś ca**                | elide final i (ti→t, si→s, Ji→J)    | laṅ          |
| 10 | **3.4.92 āḍ uttamasya pic ca**     | prefix `A` to uttama ending         | loṭ          |
| 11 | 3.1.68 kartari śap                 | insert śap; it-samjña; mark aṅga    | all          |
| 12 | **6.4.71 luṅ-laṅ-lṛṅkṣv aḍ-udāttaḥ** | prefix `a` to the aṅga            | laṅ          |
| 13 | 7.1.3 jho'ntaḥ                     | leading J → ant                     | all          |
| 14 | 7.3.84 sārvadhātukārdhadhātukayoḥ  | aṅga-final ik → guṇa                | all          |
| 15 | 6.1.78 eco'yavāyāvaḥ               | e/o/E/O → ay/av/Ay/Av               | all          |
| 16 | 7.3.101 ato dīrgho yañi            | śap `a` → `A` before m/v            | laṭ, laṅ     |
| 17 | **6.1.101 akaḥ savarṇe dīrghaḥ**   | śap `a` + `A` → `A`                 | loṭ uttama   |
| 18 | 6.1.97 ato guṇe                    | śap `a` + `a` → single `a`          | all          |
| 19 | **6.4.105 ato heḥ**                | elide `hi` after `a`                | loṭ          |
| 20 | **8.2.23 saṃyogāntasya lopaḥ**     | drop final of a word-final conjunct | all          |
| 21 | 8.3.15 kharavasānayoḥ              | word-final s → visarga              | all          |

### Ordering decisions that carry weight

Each of these was derived by hand-working the 18 new forms; getting any of them
backwards produces a wrong surface form.

- **#2 before #3–#9.** The lakāra substitutions operate on the *it*-stripped
  ending. 3.4.100 *itaś ca* elides the `i` of `tip`, but that `i` is only
  exposed after 1.3.3 *halantyam* removes the anubandha `p`. Run the
  substitutions first and laṅ 3sg derives `tp` instead of `t`.
- **#6 and #7 before #8.** *ser hyapic ca* and *mer niḥ* are *apavāda* to
  *er uḥ*: `hi` and `ni` both end in `i`, so er uḥ would corrupt them to
  `hu`/`nu` if it ran first. #8 additionally guards on the ending being exactly
  `ti` or `Ji`, making the preemption explicit rather than order-dependent.
- **#8 vs #9.** loṭ replaces the ending's final `i` with `u`; laṅ elides it.
  Only the lakāra guards keep them apart.
- **#7 vs #5's mip arm.** loṭ *uttama-eka* is `ni` (3.4.89 mer niḥ), not `am`.
  3.4.101's mip→am arm therefore guards on laṅ specifically, not on
  `is_ngit_like`.
- **#16 before #18.** Laṅ *uttama-eka*'s ending is `am`, which begins with a
  vowel, so *ato dīrgho yañi* must not fire on it. But once #18 strips the
  leading `a`, the ending looks like `m` and #16 would wrongly lengthen,
  giving `aBavAm` for `aBavam`. No v1 form fires both rules, so this ordering
  is behavior-preserving for laṭ.
- **#10 + #17 replace #16 for loṭ uttama.** `BavAni` / `BavAva` / `BavAma` come
  from the āḍ augment plus savarṇa-dīrgha, not from *ato dīrgho yañi*. #16 must
  exclude loṭ or it will double-lengthen.

### Reference verification

Per AGENTS.md, sūtra ids and names in traces must match ashtadhyayi.com. The
table above is a reconstruction and **must be verified against the reference
during implementation**, before the golden test is written. The division of
labour between #6 and #8 is the most likely error.

## Data and facade changes

**`panini-data`:** `Lakara` gains `Lan` and `Lot`.

`tin_ending` is **unchanged**, and `data/tin.tsv` needs no new rows: 3.4.78
supplies the same base endings for every lakāra, and all divergence is
rule-driven. That the data layer does not grow is a check on the rule
decomposition being right.

**`panini-analyze`:** `candidates` iterates the 3 lakāras × 9 cells × 6 roots =
162 candidates per check, up from 54. Still trivially fast; no narrowing logic
needed yet.

**`panini` facade:** `Analysis` gains `lakara: Lakara`. It currently reports
puruṣa and vacana but not which lakāra matched, which becomes ambiguous output
as soon as more than one exists. `CheckResult` is otherwise unchanged; multiple
analyses are already supported.

**`panini-cli`:** `--trace` and `--json` render the lakāra. No new subcommands
or flags.

## Testing

- **Golden paradigm** (`crates/panini/tests/paradigm.rs`) extends to 162 forms,
  keyed by `(root, lakara)`. Remains the source of truth and the gate on grammar
  changes.
- **A currently-passing assertion must change.** `known_nonforms_are_invalid`
  asserts `Bavatu` is INVALID; it becomes VALID under loṭ. Replace the negatives
  with real cross-lakāra confusions: `aBavatu`, `Bavat`, `aBavanti`. Keep
  `gacCati` (irregular gam, still out of scope) and a junk string.
- **Roundtrip** (`crates/panini/tests/roundtrip.rs`) extends over the lakāra axis.
- **Mutation testing** (`mise run mutants` over `panini-prakriya`) is the real
  gate for this slice. With 21 guarded rules the likely defect class is a
  dropped or inverted guard, which is exactly what a surviving mutant exposes.
- **Trace test** (`crates/panini/tests/trace.rs`) gains laṅ and loṭ cases
  asserting the lakāra-specific sūtras appear (6.4.71 for laṅ, 3.4.86 for loṭ).

### Expected forms (BU), for reference

- **laṅ:** aBavat, aBavatAm, aBavan, aBavaH, aBavatam, aBavata, aBavam,
  aBavAva, aBavAma
- **loṭ:** Bavatu, BavatAm, Bavantu, Bava, Bavatam, Bavata, BavAni, BavAva,
  BavAma

## Success criteria

- All 162 golden forms validate as VALID with a correct, ordered sūtra trace.
- Every sūtra id and name in a trace matches ashtadhyayi.com.
- `tinanta::derive` no longer ignores its `lakara` argument, and
  `run_pipeline` is on the real derivation path (not only in tests).
- Cross-lakāra non-forms (`aBavatu`, `Bavat`, `aBavanti`) return INVALID.
- Mutation testing shows the new guards are pinned by tests.
- Single-word `check` remains interactive-fast.

## Risks

- **The refactor touches working code.** Converting v1's straight-line
  derivation into 21 guarded rules is a substantial diff over code that
  currently passes. The 54 existing golden forms must keep passing throughout;
  they are the safety net, and should be run before the new lakāras are added,
  not only after.
- **Sūtra attribution.** Getting a form right by the wrong rule is a silent
  failure the golden test cannot catch, since it checks surface forms. The
  trace test and reference verification are the only guards against it.
