# Kryādi gaṇa (gaṇa 9) — slices 9a and 9b

**Status:** Design, approved in brainstorming 2026-07-28.

Builds on `2026-07-21-divadi-tudadi-ganas-design.md` (slice 4, the vikaraṇa
contrast), `2026-07-22-adadi-gana-design.md` and its sub-slices 5a–5f (adādi),
and `2026-07-27-kniti-guard-and-tinanta-split-design.md` (the 1.1.5 kṅiti guard
fix and the `tinanta/` stage split).

## Summary

Add **kryādi** (gaṇa 9) as the fifth gaṇa, taking the engine from 30 roots /
1080 golden forms to 36 roots / 1296. kryādi's vikaraṇa is **śnā** (3.1.81),
and unlike adādi's luk'd śap it is a perfectly ordinary vikaraṇa: it occupies
the `SHAP` slot exactly as śyan (3.1.69) and śa (3.1.77) do. **kryādi is a
fully thematic gaṇa, so none of adādi's athematic-arm hazard applies.**

What is new is that **śnā itself alternates**. It is apit, so the existing
second 1.2.4 makes it ṅit — that much is slice 4's machinery. But its final `ā`
is then reshaped by the *following* ending, in three ways:

- before a kṅit sārvadhātuka beginning with a **vowel**, the `ā` is elided
  (6.4.112) → `kliSnanti`, `vfRate`
- before a kṅit sārvadhātuka beginning with a **consonant**, the `ā` becomes
  `ī` (6.4.113) → `kliSnItaH`, `vfRIze`
- before `hi`, after a **consonant-final root**, śnā is replaced wholesale by
  **śānac** (3.1.83) → `kliSAna`

So kryādi is a second, independent witness for the 1.1.5 / 1.2.4 guard repaired
in the previous slice, and the first gaṇa where the vikaraṇa's own shape is
conditioned on the ending. It also lands the engine's **first ṇatva** (8.4.1 /
8.4.2), which is a *tripadi* rule and therefore runs over every root already in
the repo — the reason the work is split in two.

## Scope

Unchanged: single pada per root, tiṅanta, the four lakāras (laṭ, laṅ, loṭ,
vidhiliṅ), both padas, all nine puruṣa × vacana cells, one form per cell.

New: six kryādi roots — five parasmaipadī, one ātmanepadī — each across the
full 4 lakāras × 9 cells.

Out of scope, deferred:

- **Ubhayapadī roots and 1.3.72 *svaritañitaḥ*.** See "The ātmanepada column"
  below; this is the main deliberate omission and it is forced.
- **8.4.39 *kṣubhnādiṣu ca***, the ṇatva niṣedha for the kṣubhnādi list. Its
  absence is why √kṣubh is not in the root set: the engine would over-derive
  \*`kzuBRAti`.
- The kryādi roots needing machinery of their own: √bandh / √manth (6.4.24
  *aniditāṁ hala upadhāyāḥ kṅiti*), √jñā (7.3.79 *jñājanor jā*), √grah (6.1.16
  saṁprasāraṇa), √pū / √lū / √rī (7.3.80 *pvādīnāṁ hrasvaḥ*), √śṝ / √stṝ
  (7.1.100). Root choice below avoids every one.
- The anusvāra round-trip (8.3.24 / 8.4.58) and 8.4.37 *padāntasya* as recorded
  sūtras. See "The ṇatva simplification".

## Slice split

| slice | roots | forms | new rules |
|---|---|---|---|
| **9a** vikaraṇa core | √kliś, √gudh, √aś (P) | 108 → 1188 | 3.1.81, 3.1.83, 6.4.112, 6.4.113 |
| **9b** ṇatva + ātmanepada | √muṣ, √vrī (P), √vṛṅ (A) | 108 → 1296 | 8.4.1, 8.4.2 |

The split is not about diff size — it is about attribution. 8.4.1 / 8.4.2 are
tripadi rules that run over the assembled text of *every* derivation in the
repo, so a mistake there is a global regression, not a kryādi-local one. All
three 9a roots have **no ṇatva anywhere in their paradigms**, so 9a's golden
diff is provably about the vikaraṇa alone; when 9b lands, any fallout in the
1188 forms already passing is unambiguously the tripadi rule's.

## Root selection

Every root is chosen for what it witnesses, not for frequency.

| root | slice | pada | laṭ 3sg | witnesses |
|---|---|---|---|---|
| √kliś `kliS` | 9a | P | `kliSnAti` | laghu upadhā `i` — 1.1.5 must block 7.3.86, else \*`kleSnAti`. Consonant-final → 3.1.83 śānac (`kliSAna`). |
| √gudh `guD` | 9a | P | `guDnAti` | second consonant-final / laghu-upadhā witness, no ṇatva |
| √aś `aS` | 9a | P | `aSnAti` | vowel-initial aṅga → 6.4.72 āṭ + 6.1.101 in laṅ (`ASnAt`) |
| √muṣ `muz` | 9b | P | `muzRAti` | ṇatva with r/ṣ **adjacent** (8.4.1); and *intervening* (8.4.2) inside a śānac form, `muzARa` |
| √vrī `vrI` | 9b | P | `vrIRAti` | ṇatva with aṭ **intervening** throughout (8.4.2); vowel-final → loṭ 2sg `vrIRIhi`, the contrast with `kliSAna` |
| √vṛṅ `vf` | 9b | A | `vfRIte` | the gaṇa's only ātmanepadī root; ṇatva after `ṛ`; re-lands the existing 8.3.59 ṣatva (`vfRIze`, `vfRIzva`) |

### The ātmanepada column

**√vṛṅ is the only pure-ātmanepadī root in all 71 kryādi entries.** Every other
ātmanepada form in the gaṇa — `krIRIte`, `jAnIte`, `gfhRIte`, `punIte` — comes
from an **ubhayapadī** root (ñit by 1.3.72, or svarita). The curated data model
carries one pada per root, so a symmetric 3P + 3A column is not available
without either adding ubhayapada support or recording something false about
√krī and √jñā.

The decision is to **accept the asymmetry**: five parasmaipadī roots and √vṛṅ.
√vṛṅ alone still exercises the entire ātmanepada ṅit path (`vfRIte` / `vfRAte`
/ `vfRate` covers 6.4.113, 6.4.112 on a long ending, and 6.4.112 on a short
one), so nothing about the grammar goes untested — only the root count is
lopsided. Ubhayapada support (1.3.72, pada as a set on the root, the
consequent changes to `panini-analyze` and to the shape of the golden grid) is
a slice of its own, and it would also retro-fix √nī in bhvādi, which is ṇīñ —
ubhayapadī — but tagged parasmaipada today.

## Grammar

### 3.1.81 *kryādibhyaḥ śnā* — `vikarana.rs`

A near-copy of 3.1.77, ordered before 3.1.68 as an apavāda alongside 3.1.69 and
3.1.77 (3.1.68's guard already declines when a vikaraṇa is present, so no
change there). Insert `SnA` at `SHAP`, run it-saṁjñā — 1.3.8 *laśakvataddhite*
strips the initial `S`, leaving `nA`; the existing `run_it_samjna` handles this
unchanged — and tag `Vikarana` + `Sarvadhatuka`,
then mark the dhātu an aṅga. No `Tag::Pit`: śnā is apit, so the second 1.2.4
makes it ṅit, and 1.1.5 then blocks guṇa of the root. That is what gives
`krIRAti`-shaped stems rather than \*`kreRAti`, and for √kliś and √gudh it is
what blocks 7.3.86 on the laghu upadhā.

### 3.1.83 *halaḥ śnaḥ śānac* — `vikarana.rs`

When the root ends in a consonant and the ending is `hi`, śnā is replaced
wholesale by **śānac**; it-saṁjñā strips the initial `S` (1.3.8) and the final `c`
(1.3.3 *halantyam*), leaving `Ana` — again with no change to
`run_it_samjna`, whose two existing branches already cover both strips. The existing **6.4.105 *ato heḥ*** then
elides the `hi` after śāna's short `a`, giving `kliSAna`. No new rule is needed
for the `hi`-lopa.

Two ordering constraints, both load-bearing, both failing visibly:

1. **Before 6.4.113.** 6.4.113 would otherwise turn śnā's `ā` into `ī` before
   the consonant-initial ṅit `hi` and yield \*`kliSnIhi`. 3.1.83 is an apavāda
   that removes śnā before 6.4.113 can see it. Placing 3.1.83 in `vikarana.rs`
   (a stage that runs entirely before `anga.rs`) satisfies this structurally.
2. **Before the second 1.2.4.** śānac is apit; it must be tagged ṅit or 7.3.86
   guṇates √kliś's upadhā and the engine emits \*`kleSAna`. So 3.1.83 sits
   between 2.4.72 and 1.2.4 in `vikarana.rs`.

Stage placement is by pipeline position, not sūtra family — the repo's stated
rule. 3.1.83 is a 3.1.x id living after the 3.1.68 boundary, and it addresses
the ending as `ENDING` (index 2). `hi` already exists at that point: 3.4.87
*ser hyapic ca* runs in `tin.rs`, an earlier stage.

Vowel-final roots are outside 3.1.83's *halaḥ*, so √vrī takes the 6.4.113 path
instead and gives `vrIRIhi`. That pair — `kliSAna` vs `vrIRIhi` — is the
cleanest available pin on the rule's shape guard.

### 6.4.112 *śnābhyastayor ātaḥ* — `anga.rs`

Elide śnā's `ā` before a kṅit sārvadhātuka beginning with a vowel:
`kliS + nA + anti` → `kliSnanti`; `vf + nA + ate` → `vfRate`; `vf + nA + e` →
`vfRe`. The *abhyasta* half of the sūtra is not in scope (no reduplication in
the engine yet) and the rule is guarded to śnā accordingly.

### 6.4.113 *ī hal-yaghoḥ* — `anga.rs`

`ā` → `ī` before a kṅit sārvadhātuka beginning with a consonant:
`kliSnItaH`, `kliSnIyAt`, `vfRIze`, `vrIRIhi`. *aghoḥ* excludes the ghu roots
(√dā, √dhā), which are gaṇa 3 and out of scope; the exclusion is noted in the
rule comment rather than implemented, since no ghu root can reach this rule.

Both rules read the following ending through the existing
`following_sarvadhatuka` helper and sit in `anga.rs` in sūtra order, after
6.4.105. Order between them is immaterial — their conditions are disjoint
(ajādi vs halādi) — but sūtra order is kept. Order against 6.4.105 is also
safe in either direction: 6.4.105 requires a short `a` before `hi`, and śnā's
`ā` is long.

Because śnā's text goes `nA` → `n` or `nI` and never to empty, no rule reading
`p.terms[SHAP]` can silently decline the way the athematic adādi path made them
decline. This gaṇa introduces no new athematic arms.

### 8.4.1 *raṣābhyāṁ no ṇaḥ samānapade* + 8.4.2 *aṭkupvāṅnumvyavāye'pi* — `tripadi.rs`

`n` → `ṇ` after `r` or `ṣ` within the same pada. Two disjoint rule entries, so
traces stay honest about which one fired:

- **8.4.1** when `r`/`ṣ` **directly** precedes the `n` — `muz + nAti` →
  `muzRAti`, `vf + nIte` → `vfRIte`.
- **8.4.2** when only **aṭ / ku / pu / āṅ / num** intervene — `vrI + nAti` →
  `vrIRAti` (aṭ `ī`), `muz + Ana` → `muzARa` (aṭ `ā`).

The intervention predicate is the rule. It goes in `sound.rs` beside the
existing varṇa classifiers, with enumerated membership tests: **aṭ** = the
vowels plus `h y v r`; **ku** = `k K g G N`; **pu** = `p P b B m`. Encode it
too widely and existing goldens break; too narrowly and `vrIRAti` silently
loses its ṇ.

The sūtra's remaining two members, **āṅ** and **num**, are morphemes rather
than varṇa classes, and ṇatva here runs over assembled text in the tripadi
stage, where morpheme identity is no longer available. Neither is a loss: āṅ
is the upasarga `ā`, already admitted as an aṭ vowel, and num's nasal cannot
occur in the intervening position for any form in the covered grammar (no
kryādi root in the set is a num-infixing one, and upasargas are out of scope
entirely). The predicate is therefore aṭ ∪ ku ∪ pu, with this reasoning stated
in the classifier's doc comment so the two omitted members read as a deliberate
scope boundary rather than an oversight.

### The ṇatva simplification

Adding 8.4.1 / 8.4.2 naively breaks two forms that are already goldens:

- `asmaran` (√smṛ laṅ 3pl) → \*`asmaraR` — `r`, `a`, then `n`.
- `BAzante` (√bhāṣ laṭ 3pl) → \*`BAzaRte` — `ṣ`, `a`, then `n`.

Pāṇini blocks each differently, and the reference traces confirm it:

- `asmaran`'s `n` is **padānta**, and **8.4.37 *padāntasya*** forbids ṇatva
  there.
- `BAzante`'s `n` is not padānta, but by the time 8.4.1 runs it is **no longer
  an `n`**: **8.3.24 *naś cāpadāntasya jhali*** has already turned it into an
  anusvāra before the jhal `t`, and **8.4.58 *anusvārasya yayi parasavarṇaḥ***
  restores it as a dental `n` afterwards. That bleeding is what protects every
  `-nti` / `-nte` form in the repo.

This engine has no anusvāra machinery. Rather than introduce it — which would
add two tripadi steps to roughly 69 existing goldens across all four covered
gaṇas and force a re-pin of every trace test touching an `nt` form, all in code
this slice otherwise does not go near — **8.4.1 and 8.4.2 are guarded to skip
an `n` that is word-final or immediately followed by a jhal.**

That guard is exactly equivalent within the tripadi order: 8.3.24 removes
*every* non-padānta `n` before a jhal before ṇatva can run, so the two
conditions together characterise precisely the set 8.4.37 and 8.3.24 exclude.
The cost is trace fidelity, and it is a real cost: a derivation whose correct
history *records* 8.4.37 will show no such step. This is stated in both rule
comments, and it is the first thing liṭ and luṅ will want retired — they need
the anusvāra machinery for their own reasons.

## Data

Six rows appended to `data/dhatupatha.tsv`, same four-column shape as the
existing 30:

```
kliS	kryadi	parasmaipada	vibADane
guD	kryadi	parasmaipada	roze
aS	kryadi	parasmaipada	Bojane
muz	kryadi	parasmaipada	steye
vrI	kryadi	parasmaipada	varaRe
vf	kryadi	atmanepada	samBaktO
```

`Tag::Kryadi` joins `Tag::Divadi` / `Tag::Tudadi` / `Tag::Adadi`, set by
`derive`'s aṅga tagging and read by 3.1.81 alone. `derive` gains no grammar
branch. `panini-analyze` needs no change: pada still comes from the root's tag,
and a wrong-pada request is still blocked by 1.3.12 / 1.3.78 setting
`Prakriya.blocked`. `data/ATTRIBUTION.md` gains the six entries.

## Golden forms

Verified against `vidyut-prakriya` (Dhātupāṭha codes `kliSU~` 09.0058,
`guDa~` 09.0053, `aSa~` 09.0059, `muza~` 09.0066, `vrI\` 09.0040, `vfN`
09.0045). Cell order is 3sg 3du 3pl / 2sg 2du 2pl / 1sg 1du 1pl. loṭ follows
the established non-tāt convention (7.1.35 *tuhyos tātaṅ …anyatarasyām* is
optional; the repo's existing goldens are `Bavatu`, `adDi`, `Assva`).

### Slice 9a

**√kliś (parasmaipada)**

| lakāra | forms |
|---|---|
| laṭ | `kliSnAti` `kliSnItaH` `kliSnanti` / `kliSnAsi` `kliSnITaH` `kliSnITa` / `kliSnAmi` `kliSnIvaH` `kliSnImaH` |
| laṅ | `akliSnAt` `akliSnItAm` `akliSnan` / `akliSnAH` `akliSnItam` `akliSnIta` / `akliSnAm` `akliSnIva` `akliSnIma` |
| loṭ | `kliSnAtu` `kliSnItAm` `kliSnantu` / `kliSAna` `kliSnItam` `kliSnIta` / `kliSnAni` `kliSnAva` `kliSnAma` |
| vidhiliṅ | `kliSnIyAt` `kliSnIyAtAm` `kliSnIyuH` / `kliSnIyAH` `kliSnIyAtam` `kliSnIyAta` / `kliSnIyAm` `kliSnIyAva` `kliSnIyAma` |

**√gudh (parasmaipada)**

| lakāra | forms |
|---|---|
| laṭ | `guDnAti` `guDnItaH` `guDnanti` / `guDnAsi` `guDnITaH` `guDnITa` / `guDnAmi` `guDnIvaH` `guDnImaH` |
| laṅ | `aguDnAt` `aguDnItAm` `aguDnan` / `aguDnAH` `aguDnItam` `aguDnIta` / `aguDnAm` `aguDnIva` `aguDnIma` |
| loṭ | `guDnAtu` `guDnItAm` `guDnantu` / `guDAna` `guDnItam` `guDnIta` / `guDnAni` `guDnAva` `guDnAma` |
| vidhiliṅ | `guDnIyAt` `guDnIyAtAm` `guDnIyuH` / `guDnIyAH` `guDnIyAtam` `guDnIyAta` / `guDnIyAm` `guDnIyAva` `guDnIyAma` |

**√aś (parasmaipada)**

| lakāra | forms |
|---|---|
| laṭ | `aSnAti` `aSnItaH` `aSnanti` / `aSnAsi` `aSnITaH` `aSnITa` / `aSnAmi` `aSnIvaH` `aSnImaH` |
| laṅ | `ASnAt` `ASnItAm` `ASnan` / `ASnAH` `ASnItam` `ASnIta` / `ASnAm` `ASnIva` `ASnIma` |
| loṭ | `aSnAtu` `aSnItAm` `aSnantu` / `aSAna` `aSnItam` `aSnIta` / `aSnAni` `aSnAva` `aSnAma` |
| vidhiliṅ | `aSnIyAt` `aSnIyAtAm` `aSnIyuH` / `aSnIyAH` `aSnIyAtam` `aSnIyAta` / `aSnIyAm` `aSnIyAva` `aSnIyAma` |

### Slice 9b

**√muṣ (parasmaipada)**

| lakāra | forms |
|---|---|
| laṭ | `muzRAti` `muzRItaH` `muzRanti` / `muzRAsi` `muzRITaH` `muzRITa` / `muzRAmi` `muzRIvaH` `muzRImaH` |
| laṅ | `amuzRAt` `amuzRItAm` `amuzRan` / `amuzRAH` `amuzRItam` `amuzRIta` / `amuzRAm` `amuzRIva` `amuzRIma` |
| loṭ | `muzRAtu` `muzRItAm` `muzRantu` / `muzARa` `muzRItam` `muzRIta` / `muzRAni` `muzRAva` `muzRAma` |
| vidhiliṅ | `muzRIyAt` `muzRIyAtAm` `muzRIyuH` / `muzRIyAH` `muzRIyAtam` `muzRIyAta` / `muzRIyAm` `muzRIyAva` `muzRIyAma` |

**√vrī (parasmaipada)**

| lakāra | forms |
|---|---|
| laṭ | `vrIRAti` `vrIRItaH` `vrIRanti` / `vrIRAsi` `vrIRITaH` `vrIRITa` / `vrIRAmi` `vrIRIvaH` `vrIRImaH` |
| laṅ | `avrIRAt` `avrIRItAm` `avrIRan` / `avrIRAH` `avrIRItam` `avrIRIta` / `avrIRAm` `avrIRIva` `avrIRIma` |
| loṭ | `vrIRAtu` `vrIRItAm` `vrIRantu` / `vrIRIhi` `vrIRItam` `vrIRIta` / `vrIRAni` `vrIRAva` `vrIRAma` |
| vidhiliṅ | `vrIRIyAt` `vrIRIyAtAm` `vrIRIyuH` / `vrIRIyAH` `vrIRIyAtam` `vrIRIyAta` / `vrIRIyAm` `vrIRIyAva` `vrIRIyAma` |

**√vṛṅ (ātmanepada)**

| lakāra | forms |
|---|---|
| laṭ | `vfRIte` `vfRAte` `vfRate` / `vfRIze` `vfRATe` `vfRIDve` / `vfRe` `vfRIvahe` `vfRImahe` |
| laṅ | `avfRIta` `avfRAtAm` `avfRata` / `avfRITAH` `avfRATAm` `avfRIDvam` / `avfRi` `avfRIvahi` `avfRImahi` |
| loṭ | `vfRItAm` `vfRAtAm` `vfRatAm` / `vfRIzva` `vfRATAm` `vfRIDvam` / `vfRE` `vfRAvahE` `vfRAmahE` |
| vidhiliṅ | `vfRIta` `vfRIyAtAm` `vfRIran` / `vfRITAH` `vfRIyATAm` `vfRIDvam` / `vfRIya` `vfRIvahi` `vfRImahi` |

## Testing

- **Golden paradigm** (`crates/panini/tests/paradigm.rs`): +108 forms per
  slice, 1080 → 1188 → 1296. `paradigm_covers_every_enumerable_cell` keeps the
  root × lakāra grid honest.
- **Ordered traces** (`crates/panini/tests/trace.rs`), one per new rule and per
  ordering constraint:
  - 9a — `kliSnAti` (3.1.81; `ā` retained under a pit ending), `kliSnItaH`
    (6.4.113), `kliSnanti` (6.4.112), `kliSAna` (3.1.83 then 6.4.105),
    `ASnAt` (6.4.72 + 6.1.101 on a vowel-initial aṅga).
  - 9b — `muzRAti` (8.4.1), `vrIRAti` (8.4.2), `muzARa` (3.1.83 and 8.4.2
    together), `vfRIte` (ātmanepada 6.4.113), `vfRIze` (8.3.59 ṣatva).
- **Rule-level guard tests** beside each rule in its stage file, inside/outside
  pattern, per `AGENTS.md`. Specifically:
  - 3.1.81 fires on `Tag::Kryadi` and declines on every other gaṇa tag.
  - 3.1.83 fires on a consonant-final root before `hi` and declines on a
    vowel-final one (the `vrIRIhi` case), and declines when the ending is not
    `hi`.
  - 6.4.112 / 6.4.113 fire on their own side of the ajādi/halādi split and
    decline on the other; each declines when the following sārvadhātuka is not
    kṅit (the `kliSnAti` / `kliSnAmi` case); each declines on a term whose text
    is not śnā; neither panics on a short `terms` vector.
  - 8.4.1 / 8.4.2 fire on their own adjacency case and decline on the other's;
    both decline on a word-final `n` and on an `n` before a jhal, with
    `asmaran` and `BAzante` named in the test as the goldens those guards
    exist for.
- **`tinanta_rule_order_is_pinned`**: 56 → 60 ids after 9a, → 62 after 9b, each
  at its exact position. Ordering is grammar; this is the test that catches a
  silent reorder even when surfaces are unchanged.
- **Mutation testing**: `mise run mutants` at zero survivors at the end of each
  slice, invoking the `cargo-mutants` binary directly rather than through the
  `mise` shim, which fails in background shells. Scope a single crate during
  iteration with `mise exec -- cargo test -p panini-prakriya`.
- **`mise run fmt-check`, `lint`, `audit`** clean.

`panini-lipi` roundtrip / property / fuzz targets are untouched.

## Risks

1. **8.4.2's intervention set is the whole rule.** Too wide and existing
   goldens break — `varSanti` (`ś` intervenes, must block) and `avartanta`
   (`t` intervenes, must block) are the near misses. Too narrow and `vrIRAti`
   quietly loses its ṇ, which no other form would reveal. Mitigated by
   enumerated membership tests on the classifier itself, not only by
   end-to-end goldens.
2. **The ṇatva guard is a stated simplification, not a hidden one.** Recorded
   above, in both rule comments, and in `ARCHITECTURE.md`. The failure mode to
   watch is someone later reading the guard as a *grammatical* claim about
   8.4.1 rather than as a stand-in for 8.3.24 and 8.4.37.
3. **3.1.83's two ordering constraints.** Both fail visibly — wrong against
   6.4.113 gives `kliSnIhi`, wrong against 1.2.4 gives `kleSAna` — so the
   goldens catch either, and the traces pin them. Low risk, high blast radius
   if missed.
4. **`anga.rs` growth.** At 939 lines it is already the largest stage file;
   6.4.112 / 6.4.113 plus guard tests push it toward ~1100. They belong there
   in sūtra order and this slice keeps them there. If it crosses that mark,
   extracting the 7.x guṇa rules into their own stage file is the obvious
   mechanical follow-up — explicitly **not** part of this slice.
5. **Root-set discipline.** Five of the six roots were chosen partly for what
   they *avoid* (6.4.24, 7.3.79, 7.3.80, 8.4.39, saṁprasāraṇa). Adding a
   seventh kryādi root casually will reintroduce one of those; the deferral
   list above is the checklist.

## Success criteria

- All **1296** goldens validate `VALID`; the 1080 pre-existing forms and their
  traces are byte-identical to `main` after both slices.
- Every new rule is pinned by an ordered trace *and* by inside/outside guard
  tests in its stage file.
- `asmaran` and `BAzante` still derive with a dental `n`, and a test names them
  as the ṇatva tripwires.
- The flattened rule order is pinned verbatim at 60 ids (9a) and 62 ids (9b).
- `derive` still carries no grammar branch — only the new `Tag::Kryadi`
  aṅga tag.
- `docs/ARCHITECTURE.md` and `AGENTS.md` name kryādi, the śnā alternation, and
  the ṇatva simplification.
- `mise run mutants` reports **zero** survivors; `lint`, `fmt-check` and
  `audit` are clean.
