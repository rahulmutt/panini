# Svādi gaṇa (gaṇa 5) — prep, slice 5a, slice 5b

**Status:** Design, approved in brainstorming 2026-07-29.

Builds on `2026-07-21-divadi-tudadi-ganas-design.md` (the vikaraṇa contrast),
`2026-07-22-adadi-gana-design.md` and its sub-slices 5a–5f (adādi — note those
slice letters belong to *adādi* and are unrelated to the 5a/5b below, which name
gaṇa 5), `2026-07-27-kniti-guard-and-tinanta-split-design.md` (the 1.1.5 kṅiti
guard and the `tinanta/` stage split), and
`2026-07-28-kryadi-gana-design.md` (the apit layer and the first ṇatva).

## Summary

Add **svādi** (gaṇa 5) as the sixth gaṇa, taking the engine from 36 roots /
1296 golden forms to 42 roots / 1512. Svādi's vikaraṇa is **śnu** (3.1.73),
which occupies the `SHAP` slot as an ordinary vikaraṇa exactly as śyan, śa and
śnā do, and which the existing second 1.2.4 tags ṅit without modification (śnu
is apit).

What is new is **where the guṇa lands**. In every gaṇa so far the guṇa that
7.3.84 performs falls on the *root*: bhvādi guṇates `BU` with respect to śap;
adādi's śap is luk'd, so the aṅga with respect to the ending simply *is* the
root; divādi, tudādi and kryādi block guṇa outright through 1.1.5. Svādi is the
first gaṇa where the aṅga that 7.3.84 operates on is **root + vikaraṇa**, and
the ik it guṇates belongs to the vikaraṇa:

```
Ap + nu + ti   →  Ap + no + ti   →  Apnoti
Ap + nu + taH  →  (blocked, tas is apit → ṅit)  →  ApnutaH
```

1.4.13 *yasmāt pratyayavidhis tadādi pratyaye'ṅgam* makes the aṅga
**affix-relative**, so 7.3.84 genuinely applies at two junctures — once with
respect to the vikaraṇa, once with respect to the ending. The engine models the
aṅga as a fixed term index (`ANGA`, "the dhātu"), so it currently expresses only
the first. Slice 5a adds the second as a **second application of 7.3.84**, in
the same way the pipeline already carries two applications of 1.2.4.

That second application is the first of **four** places where the engine has to
stop treating "the aṅga" as "the root": 6.1.78, 6.1.90 and 6.4.101 all read the
root's final sound where svādi needs the vikaraṇa's, and all three are modified
rather than added. See "Three existing rules assume the aṅga is the root".

The rest of the gaṇa is the śnu alternation before vowel-initial endings
(6.4.87 / 6.4.77) and the hi-luk (6.4.106), all three turning on one predicate:
whether śnu's `u` is preceded by a conjunct.

Svādi is also, for the same reason, where the pit/apit distinction the previous
slice built finally does visible work in the *parasmaipada* column:
`Apnoti` against `ApnutaH` is that distinction and nothing else.

## Scope

Unchanged: single pada per root, tiṅanta, the four lakāras (laṭ, laṅ, loṭ,
vidhiliṅ), all nine puruṣa × vacana cells, one form per cell.

New: six svādi roots — four parasmaipadī, two ātmanepadī — each across the full
4 lakāras × 9 cells.

Out of scope, deferred:

- **Ubhayapadī roots and 1.3.72 *svaritañitaḥ***, still. This costs more here
  than in any previous gaṇa: **10 of gaṇa 5's 38 roots are ñit**, including
  √su (`zu\Y`, sunoti / sunute) and √ci (`ci\Y`), the two roots every grammar
  uses to teach śnu. See "The ātmanepada column". (√dhṛṣ, `YiDfzA~`, is *not*
  among them — its `Y` is the initial *ñi* it-marker of 1.3.5 *ādir ñiṭuḍavaḥ*,
  not the final ñ that 1.3.72 reads, and it is parasmaipadī.)
- **6.4.107 *lopaś cāsyānyatarasyāṁ mvoḥ***, deferred to slice 5b, because it
  is the engine's first genuinely **optional** rule. See "Slice split".
- **6.1.64 *dhātvādeḥ ṣaḥ saḥ***. √ṣṭigh ships as `stiG` in
  `data/dhatupatha.tsv` rather than being derived from `zwiGa~\`. See "Data".
- **7.1.35 *tuhyos tātaṅ āśiṣy anyatarasyām*** (loṭ `ApnutAt`) and **8.4.56
  *vāvasāne*** (pausal `Apnod`). Both are pre-existing repo-wide conventions
  for picking one form per cell, not svādi decisions.
- The svādi roots needing machinery of their own: √śru (3.1.74 *śruvaḥ śṛ ca*),
  and the ñit roots √stṛ / √kṛ, which additionally want 7.1.100 and the 6.4.10x
  kṛ-specials. The root set below avoids every one.

## Slice split

| slice | content | forms | new rules |
|---|---|---|---|
| **prep 1** | split `anga.rs` into `anga.rs` + `guna.rs` | 1296 → 1296 | none |
| **prep 2** | `Dhatu::id`, gaṇa-qualified root identity | 1296 → 1296 | none |
| **5a** | six roots, the śnu core | 1296 → 1512 | new: 3.1.73, 6.4.87, 6.4.77, 6.4.106; 7.3.84 second application; modified: 6.1.78, 6.1.90, 6.4.101 |
| **5b** | optional-rule support | 1512 → 1512 (+8 alternates) | 6.4.107 |

Both prep commits are behaviour-preserving and are verified the same way: the
pinned rule order and all 1296 forms and traces unchanged. Neither introduces
grammar, and neither should be reviewed as though it did.

Prep 1 is forced by size. `anga.rs` is **1110 lines**, already the
largest stage file, and slice 5a adds roughly 250 more. The kryādi spec named
this exact follow-up in its Risk 4: *"extracting the 7.x guṇa rules into their
own stage file is the obvious mechanical follow-up."* Doing it **before** any
new grammar means the new rules land in the file they belong in, and the move
itself is verifiable to the byte.

The 5a/5b split is about attribution, on the same logic that split kryādi into
9a and 9b. Optional-rule support is not a svādi feature: it changes what a
derivation *is* (one prakriyā becomes a set), which touches `derive`'s return
type, the `panini` facade, the CLI's `--json` shape and the golden table's
schema. Landing it alone, against a suite that 5a has already frozen at 1512
byte-identical forms, means any fallout is attributable to the fork machinery
and nothing else.

5b's witness is exactly **8 cells**, all in the uttama dvi/bahu of laṭ and laṅ
for the two non-conjunct roots:

| | laṭ dvi | laṭ bahu | laṅ dvi | laṅ bahu |
|---|---|---|---|---|
| √hi | hinvaH ~ hinuvaH | hinmaH ~ hinumaH | ahinva ~ ahinuva | ahinma ~ ahinuma |
| √ri | riRvaH ~ riRuvaH | riRmaH ~ riRumaH | ariRva ~ ariRuva | ariRma ~ ariRuma |

Slice 5a derives the right-hand (un-elided) member of each pair, which is what
the golden table pins; 5b adds the left-hand member as a second valid form.
Until 5b lands, `hinmaH` is reported INVALID — correct under the project's
standing reading of INVALID as "not derivable within this covered grammar," but
recorded here as a real gap, not an accident. (Slice 5b has since landed —
see `docs/superpowers/specs/2026-08-08-optional-rules-6-4-107-design.md` —
so `hinmaH` is now reported VALID, as an alternate alongside `hinumaH`.)

## Root selection

| root | pada | artha | code | contributes |
|---|---|---|---|---|
| √āp | parasmaipada | vyāptau | 05.0016 | conjunct `pn`: 6.4.77 uvaṅ, 6.4.106 blocked; 6.1.101 for `Apnot` |
| √śak | parasmaipada | śaktau | 05.0017 | second conjunct witness, with an ordinary `a`-initial augment (`aSaknot`) |
| √hi | parasmaipada | gatau vṛddhau ca | 05.0012 | **non-conjunct**: 6.4.87 yaṇ (`hinvanti`), 6.4.106 luk (`hinu`) |
| √ri | parasmaipada | hiṁsāyām | 05.0032 | second non-conjunct root, and ṇatva under vowel intervention (`riRoti`, `riRvanti`, `riRu`) |
| √aś | ātmanepada | vyāptau saṅghāte ca | 05.0020 | the ātmanepada column; minimal pair with kryādi's √aś (`aSnute` vs `aSnAti`) |
| √ṣṭigh | ātmanepada | āskandane | 05.0021 | the only other ātmanepadī root in the gaṇa |

Two roots earn their place by what they make *contrastable*. √hi and √ri are
the only members of the set whose śnu is **asaṁyogapūrva** — the `n` follows a
vowel, so the `u` is preceded by a single consonant. Every rule in this slice
that has a condition at all has *that* condition, and a set containing only
conjunct roots would leave all three untested on the affirmative side.

√ri additionally combines ṇatva with the u → v alternation in one form
(`riRvanti`), reusing the vowel-intervention path that `vrIRAti` already pins,
and gives ṇatva a witness where the trigger `r` is the root's own initial.

√rādh was considered as a fourth parasmaipadī root and rejected: it is a third
conjunct root, and its one distinctive contribution — a *negative* ṇatva
witness, `rADnoti` keeping a dental `n` because `D` is outside 8.4.2's
intervention set — duplicates what `avartanta` already guards.

### The ātmanepada column

Gaṇa 5 has exactly **two** roots that are ātmanepadī outright: `aSU~\` and
`zwiGa~\`. Every other ātmanepada form in svādi belongs to a ñit (ubhayapadī)
root, and 1.3.72 remains deferred. So the 3P + 3Ā shape every previous gaṇa
used is unavailable, and the slice ships 4P + 2Ā instead.

Both ātmanepadī roots are **conjunct-preceded** (`aSnu` — the `u` follows the
cluster `S` + `n`; `stiGnu` — `G` + `n`). This is worth stating because it is
counter-intuitive: √aś looks like √su, but the root's final consonant joins
śnu's `n` to form the conjunct, and vidyut-prakriya confirms the consequence —
`aSnuvate`, `aSnuvIta`, `aSnuvahe`, and **no** lopa alternate at `aSnumahe`,
against √su's `sunvate`, `sunvIta`, `sunvahe ~ sunuvahe`, `sunmahe ~ sunumahe`.

The ātmanepada column therefore adds no new *rule*; it is a pure reuse witness
(7.1.5, 7.2.79/7.2.80, 8.3.59, 6.1.101, 6.4.77). That is why both roots sit in
5a rather than in a slice of their own: they carry no novelty to isolate.

## Grammar

### 3.1.73 *svādibhyaḥ śnuḥ* — `vikarana.rs`

Insert śnu between dhātu and ending, run it-saṁjñā (1.3.8 strips `S`, leaving
`nu`), and tag the dhātu an aṅga. Structurally identical to 3.1.69, 3.1.77 and
3.1.81; guarded on `Tag::Svadi`. Ordered among the other apavādas in sūtra
order (3.1.69, 3.1.73, 3.1.77, 3.1.81) and before the utsarga 3.1.68, which
declines because a vikaraṇa is already present.

No new tagging work: śnu carries no `p`-anubandha, so the **existing second
1.2.4** tags it ṅit with no change — its guard is `has(Vikarana) && !has(Pit)`.
1.1.5 then blocks 7.3.84's *first* application, which is what keeps `hi` and
`ri` from guṇating to `he` / `re`.

### 7.3.84 *sārvadhātukārdhadhātukayoḥ*, second application — `guna.rs`

The slice's central addition. A second `Rule` entry carrying the **same id** as
the existing 7.3.84, ordered after it, which guṇates the final ik of
`terms[SHAP]` with `terms[ENDING]` as the 1.1.5 follower:

```
Ap + nu + ti    →  Ap + no + ti      Apnoti
Ap + nu + Ani   →  Ap + no + Ani     ApnavAni  (via 6.1.78)
Ap + nu + taH   →  blocked, tas is ṅit    ApnutaH
```

Two applications, not one generalised rule, for three reasons. It is what the
sūtra says: 1.4.13 defines the aṅga relative to the affix, and śnu and the tiṅ
ending are two affixes, so 7.3.84 has two occasions. It matches the pipeline's
existing idiom — **1.2.4 already appears twice** in the pinned order for exactly
this reason, once on the ending and once on the vikaraṇa. And its no-delta
claim rests on a guard rather than on an argument (see below), which the kryādi
spec's Risk 5 asks for explicitly.

**Placement carries two constraints, both failing visibly:**

- **BEFORE 6.1.78 *eco'yavāyāvaḥ***. The loṭ uttama endings are vowel-initial
  and pit (3.4.92 *āḍ uttamasya pic ca*), so guṇa fires and leaves `Apno`,
  which 6.1.78 must then turn into `Apnav`. Ordered after 6.1.78, the `o`
  never converts and the form surfaces as \*`ApnoAni`.
- **BEFORE 6.4.87 / 6.4.77**. Those rules also fire before a vowel-initial
  ending. Ordered after them, the āṭ-augmented loṭ ending takes śnu's `u` to
  `uv` first and the form surfaces as \*`ApnuvAni` instead of `ApnavAni`.

Both constraints are satisfied by inserting the rule immediately after 7.3.86,
which is also its natural position in juncture order (root-aṅga guṇa, then
vikaraṇa-aṅga guṇa).

**The no-delta guard.** The second application requires a non-empty `SHAP`
whose text ends in an ik, plus a non-ṅit `ENDING`. The complete inventory of
`SHAP` texts reaching this point in the existing suite is:

| SHAP | source | ik-final? |
|---|---|---|
| `a` | śap (3.1.68), śa (3.1.77) | no |
| `ya` | śyan (3.1.69) | no |
| *(empty)* | adādi luk (2.4.72) | n/a |
| `Ana` | śānac (3.1.83) | no |
| `nA` | śnā (3.1.81) | no |
| `n` | śnā after 6.4.112 | no |
| `nI` | śnā after 6.4.113 | **yes** |

Only `nI` is ik-final, and 6.4.113 produces `nI` **only before a ṅit ending** —
so on that one path the rule declines on its 1.1.5 guard. Every existing
derivation is untouched by construction, not by coincidence, and two dedicated
tests pin it.

### 6.4.87 *huśnuvoḥ sārvadhātuke* — `guna.rs`

Before a vowel-initial sārvadhātuka, śnu's `u` becomes `v` (yaṇ) rather than
`uv`, when the `u` is **asaṁyogapūrva**:

```
hi + nu + anti  →  hi + nv + anti   hinvanti
ri + nu + antu  →  ri + nv + antu   riRvantu  (ṇatva in tripadi)
```

The rule names *hu* and *śnu*; only the śnu arm is implemented, since √hu is
juhotyādi and out of scope. The *asaṁyogapūrva* restriction is anuvṛtti from
6.4.82 *er anekāco'saṁyogapūrvasya* — recorded in the rule comment, since the
guard would otherwise look invented.

Ordered **before** 6.4.77, of which it is the apavāda. This is the pipeline's
established shape (the 3.1.69 comment cites 6.4.72 preceding 6.4.71 for it), and
it self-guards: once 6.4.87 has rewritten `nu` to `nv`, 6.4.77's `nu` test no
longer matches, so no explicit "did the apavāda fire?" check is needed.

### 6.4.77 *aci śnudhātubhruvāṁ yvor iyaṅuvaṅau* — `guna.rs`

Before a vowel, śnu's `u` becomes `uv` (uvaṅ):

```
Ap + nu + anti   →  Ap + nuv + anti   Apnuvanti
aS + nu + ate    →  aS + nuv + ate    aSnuvate
aS + nu + Iyta   →  aS + nuv + Iyta   aSnuvIta  (y dropped later by 6.1.66)
```

Only the śnu arm of the sūtra is implemented. The *dhātu* arm (ī/ū-final roots)
and the *bhrū* arm have no root in scope and are recorded as unimplemented, in
the same style as 6.4.112's *abhyasta* half and 6.4.113's *aghoḥ*.

Both 6.4.87 and 6.4.77 read `terms[ENDING]` directly rather than
`following_sarvadhatuka`: as with 6.4.112/113, the helper answers "what follows
the aṅga", which here is śnu itself — these rules need what follows *śnu*.

### 6.4.106 *utaś ca pratyayād asaṁyogapūrvāt* — `adesha.rs`

Luk of `hi` after an affix-final `u` that is not conjunct-preceded:

```
hi + nu + hi   →  hinu        ri + nu + hi   →  riRu
Ap + nu + hi   →  Apnuhi      (conjunct: declines)
```

Placed immediately after the existing **6.4.105 *ato heḥ***, which it continues
and which supplies the luk. 6.4.105 already declines for svādi on its own guard
(the stem ends in `u`, not `a`), so the two coexist without interaction.

### Three existing rules assume the aṅga is the root

The second application of 7.3.84 is not an isolated curiosity. It is the first
of **four** instances of one problem, and the other three are modifications to
existing rules rather than additions.

Every rule that operates on "the aṅga's final sound" was written when that
sound was always the *root's* final, because in bhvādi it is (`Bo` + śap), in
adādi it is (śap luk'd), and in divādi / tudādi / kryādi the aṅga-final vowel
this family cares about never arises — guṇa is blocked, so no `e`/`o` is ever
created. Svādi is the first gaṇa where the operative aṅga-final sound sits on
the **vikaraṇa**, and every such rule has to be told.

| rule | reads | svādi wants | left alone |
|---|---|---|---|
| **7.3.84** | `ANGA` final ik | `SHAP` final ik (`nu` → `no`) | \*`Apnuti` |
| **6.1.78** *eco'yavāyāvaḥ* | `ANGA` final `e`/`o` | `SHAP` final `o` (`no` → `nav`) | \*`ApnoAni` |
| **6.1.90** *āṭaś ca* | athematic arm gated on `SHAP.is_empty()` | fire for any non-`a`/`A`-final SHAP | \*`aSnavAE` |
| **6.4.101** *hujhalbhyo her dhiḥ* | `ANGA` final jhal | the sound before the ending | \*`ApnuDi` |

Each is treated below. None of the four is optional, and each is revealed by a
*different* handful of cells, so no one golden catches them all.

### 6.4.101 must stop assuming the aṅga abuts the ending — `adesha.rs`

**A modification to an existing rule, not a new one**, and the second instance
in this slice of the affix-relative aṅga problem 7.3.84 has.

6.4.101 *hujhalbhyo her dhiḥ* turns loṭ 2sg `hi` into `Di` after a jhal-final
aṅga — √ad's `adDi`. It tests the last character of `terms[ANGA]`, i.e. the
*root's* final. For the two conjunct parasmaipadī roots that is a jhal (`p` in
`Ap`, `k` in `Sak`), while the sound actually preceding `hi` is śnu's `u`. Left
alone, the rule fires and produces \*`ApnuDi` and \*`SaknuDi`.

Nothing upstream saves it. 6.4.105 declines (the stem ends in `u`, not `a`), and
the new 6.4.106 declines on exactly these two roots, by design — so `hi`
survives to 6.4.101 with a jhal-final root behind it. The two ātmanepadī roots
never take `hi` and √hi / √ri end in vowels, so **√āp and √śak are the only
witnesses, and their loṭ 2sg cell is the only cell that reveals it.**

The fix is the idiom 8.3.59 already uses: read the sound immediately preceding
the ending — the last character of the nearest non-empty term before `ENDING` —
rather than the root's final. This is extracted as a shared helper rather than
written inline a third time.

It is **behaviour-preserving on all 1296 existing forms**, and provably so by
enumeration of what precedes `hi` when 6.4.101 runs:

| path | term before ENDING | old test | new test | outcome |
|---|---|---|---|---|
| adādi (√ad) | śap, empty → falls back to `ad` | `d` jhal | `d` jhal | `adDi`, unchanged |
| bhvādi / divādi / tudādi | śap `a` | — | — | `hi` already luk'd by 6.4.105; rule declines either way |
| kryādi (√vrī) | śnā `nI` | `I` on `vrI`, not jhal | `I`, not jhal | `vrIRIhi`, unchanged |
| kryādi (√kliś) | śānac `Ana` | — | — | `hi` already luk'd by 6.4.105 |
| svādi (√āp, √śak) | śnu `nu` | `p`/`k` jhal — **wrong** | `u`, not jhal | `Apnuhi`, fixed |

Every existing path either reaches the same answer by the same character or has
no `hi` left to rewrite. The adādi row is the one that matters: the fallback to
`ANGA` when śap is empty is what keeps `adDi` working, and it is the reason the
helper returns the nearest *non-empty* term rather than simply `terms[SHAP]`.

### 6.1.78 needs a vikaraṇa arm — `guna.rs`

6.1.78 *eco'yavāyāvaḥ* opens with

```rust
let anga_last = p.terms[ANGA].text.chars().last().unwrap();
let sub = match anga_last { 'e' => "ay", 'o' => "av", _ => return false };
```

so for √āp (`Ap`, final `p`) it returns before either of its two existing arms
is consulted. The `o` that needs converting is the one the second 7.3.84 just
created on the vikaraṇa. Without a third arm, `ApnavAni` surfaces as
\*`ApnoAni` and `aSnavE` as \*`aSnoAE` — every loṭ uttama cell and the laṅ
uttama-eka cells, 18 of the 216.

**Third arm:** when `SHAP` ends in `e`/`o` and `ENDING` is vowel-initial,
convert `SHAP`'s final rather than `ANGA`'s. It is mutually exclusive with both
existing arms by construction: those two both require `ANGA` to end in `e`/`o`,
which no svādi root does, and this one requires `SHAP` to end in `e`/`o`, which
neither śap (`a`), śyan (`ya`), śa (`a`), śnā (`nA`/`n`/`nI`), śānac (`Ana`) nor
adādi's empty śap ever does. The existing early return has to be restructured so
it no longer short-circuits before the new arm is reached — that restructuring,
not the arm itself, is the risky edit, and the 1296-form regression is what
pins it.

### 6.1.90's athematic arm must widen — `adesha.rs`

The arm that handles loṭ uttama-eka for a stem the coalescence rules did not
touch is gated on `p.terms[SHAP].text.is_empty()`, with a comment reasoning that
kryādi is safely excluded because 6.1.101's kryādi arm has already consumed the
āṭ `A` into SHAP. That reasoning is correct and stays correct. But svādi's SHAP
at that point is `nav` — not empty, not `a`-final, not `A`-final — so **no arm
of 6.1.101 fires and no arm of 6.1.90 fires**, and the āṭ `A` is left stranded
at the head of the ending: `aS` + `nav` + `AE` = \*`aSnavAE`.

**Widen the guard** from `is_empty()` to "SHAP ends in neither `a` nor `A`".
This is the correction the 3.1.81 comment in `vikarana.rs` predicted in general
terms — *"a rule that guards on `SHAP.is_empty()` to detect 'the thematic
coalescence rules didn't apply' still silently declines"* — now with a second
witness. The no-delta check is by enumeration of SHAP at that point: adādi
`""` (fires, as before), bhvādi/tudādi `a` or `A`, divādi `ya`/`yA`, kryādi
`nA`, śānac `Ana` — all excluded by the new test exactly as they were by the
old one. Svādi's `nav` is the only shape whose answer changes.

Only **two cells** in the whole slice reveal this: `aSnavE` and `stiGnavE`. The
parasmaipada loṭ uttama endings are `Ani`/`Ava`/`Ama`, whose second character is
a consonant, so they never enter this arm at all.

### The *asaṁyogapūrva* predicate

6.4.87, 6.4.106 and (in 5b) 6.4.107 share one condition, which becomes a single
helper rather than three transcriptions of the same test:

> Is śnu's `u` preceded by a single consonant? Equivalently: **is the last
> character of `terms[ANGA]` a vowel?**

| root | aṅga-final | asaṁyogapūrva |
|---|---|---|
| √hi | `i` | yes |
| √ri | `i` | yes |
| √āp | `p` | no |
| √śak | `k` | no |
| √aś | `S` | no |
| √ṣṭigh | `G` | no |

This predicate fails **silently** if wrong — an inverted or mis-scoped test
turns `hinu` into \*`hinuhi` and `Apnuhi` into \*`Apnu` with no other symptom,
and both are plausible-looking Sanskrit. It therefore gets an enumerated
membership test on the helper itself, over all six roots plus a kryādi and a
bhvādi control, rather than being covered only end-to-end.

The helper lives beside the other shared shape tests and is called from two
different stage files (`guna.rs` and `adesha.rs`), which is why it is a helper
and not a private function in either.

### What svādi gets for free

The slice adds four sūtras because almost everything else is already present.
Recorded here so that the small rule count is not mistaken for thin coverage:

- **1.2.4's apit layer** (widened in kryādi 9a) does the gaṇa's signature work.
  It blocks the first 7.3.84 on the ik-final roots (`hinoti`, not \*`henoti`)
  and blocks the second before apit endings (`ApnutaH`, not \*`ApnotaH`).
- **6.1.101 *akaḥ savarṇe dīrghaḥ*** — `Apnot` (aṭ + `A`) and `ASnuta`
  (aṭ + `a`).
- **6.1.78 *eco'yavāyāvaḥ*** — `ApnavAni`, `aSnavE`.
- **7.1.5 *ātmanepadeṣv anataḥ*** — `aSnuvate` (jha → `ate`, the aṅga not being
  `a`-final).
- **7.2.79 / 7.2.80** and **6.1.66** — `aSnuvIta`.
- **8.3.59 *ādeśapratyayayoḥ*** — seven new ṣatva witnesses (`Apnozi`,
  `Saknozi`, `hinozi`, `riRozi`, `aSnuze`, `aSnuzva`, `stiGnuzva`) with **no
  code change**. The rule already searches for the s-initial term and reads the
  nearest non-empty preceding term's final vowel, so it sees śnu's `u` and the
  guṇated `o` without knowing what they are.
- **8.4.1 / 8.4.2 ṇatva** — `riRoti`, `riRvanti`, `riRu`, reusing the
  vowel-intervention path `vrIRAti` pins.
- **3.4.87 *ser hyapic ca*** and **3.4.103** supply the ṅit tags that 6.4.106
  and the optative depend on.

### Prep 1: splitting `anga.rs`

`anga.rs` holds 16 rules over 1110 lines. The cut falls **after 7.2.81, before
7.4.21**:

| file | rules |
|---|---|
| `anga.rs` — augments and ending reshaping | 6.4.71, 6.4.72, 7.3.100, 7.1.5, 7.1.6, 7.1.3, 7.2.79, 7.2.80, 7.2.81 |
| `guna.rs` — vowel gradation and vikaraṇa reshaping | 7.4.21, 7.3.84, 7.3.86, 6.1.78, 7.3.101, 6.4.112, 6.4.113 |

`TINANTA_RULES` goes from six stage arrays to seven. The flattened sequence is
unchanged, so `tinanta_rule_order_is_pinned` is untouched by prep 1 —
which is the point: the move is verified by the order test and the goldens
staying byte-identical, with no new assertion needed to prove it was safe.

Every svādi rule outside `vikarana.rs` and `adesha.rs` then lands in `guna.rs`,
which after 5a holds ten rules and is the file the next gaṇa will also grow.

## Data

Six rows appended to `data/dhatupatha.tsv`:

```
Ap	svadi	parasmaipada	vyAptO
Sak	svadi	parasmaipada	SaktO
hi	svadi	parasmaipada	gatO vfdDO ca
ri	svadi	parasmaipada	hiMsAyAm
aS	svadi	atmanepada	vyAptO saNGAte ca
stiG	svadi	atmanepada	Askandane
```

`Gana::Svadi` and `Tag::Svadi` follow the existing pattern; `derive` gains the
aṅga tagging only, no grammar branch.

**√ṣṭigh is stored as `stiG`.** Its upadeśa is `zwiGa~\`, and 6.1.64 *dhātvādeḥ
ṣaḥ saḥ* is what turns the initial `z` into `s`; the retroflex `w` surfaces as
`t` along with it, having had no retroflexing cause once the `z` is gone. The
file already stores post-upadeśa, it-stripped SLP1 — `vf` for `vfN`, `BU` for
`BU`-with-accent — so this is consistent with its convention rather than a new
liberty. It is recorded as a stated simplification because unlike it-stripping,
which 1.3.x rules perform in the pipeline, no rule in the engine performs this
substitution: the trace for `stiGnute` will not mention 6.1.64. Implement it
when a slice needs a second ṣ-initial root.

**√aś collides with kryādi's √aś, and the collision is silent.** They are
distinct roots — 05.0020 *vyāptau* and 09.0059 *bhojane* — that happen to share
an SLP1 form. `Dhatu` currently has no identity field separate from the root
text: `code` is the text *and* the lookup key, and every caller resolves a root
with `dhatus().iter().find(|d| d.code == …)`. A second `aS` row makes that
lookup return the kryādi entry and leaves the svādi root **unreachable**, in
`paradigm.rs`'s helpers and in `panini-data`'s own tests alike, with no error.

The validator itself is unaffected — `check` enumerates all roots, so `aSnute`
and `aSnAti` both validate regardless. The breakage is confined to by-code
lookup, which is exactly where it is hardest to notice.

Fix, in slice 5a: give `Dhatu` a distinct `pub id: &'static str`, gaṇa-qualified
(`"aS.5"`, `"aS.9"`), used as the lookup key and as the `PARADIGM` row key;
`code` keeps its meaning as the root's SLP1 text. Every existing root's `id` is
its `code`, so the 36 existing rows and 1296 existing golden rows change only in
which field they are keyed on. This is a data-model change, small but real, and
it is a prerequisite for the √aś row rather than a consequence of it.

## Golden forms

216 new forms, generated by a `vidyut-prakriya` probe and reduced to one form
per cell by the repo's standing conventions: drop tātaṅ (7.1.35), drop the
pausal `d` variant where a `t` sibling exists (8.4.56), and keep the un-elided
member of a 6.4.107 pair until 5b lands. Applying those three filters to
vidyut's output leaves **exactly one form in all 216 cells** — no cell required
a judgement call.

Order within a row is prathama/madhyama/uttama × eka/dvi/bahu.

### Parasmaipada

```
Ap   laT   Apnoti ApnutaH Apnuvanti Apnozi ApnuTaH ApnuTa Apnomi ApnuvaH ApnumaH
Ap   laN   Apnot ApnutAm Apnuvan ApnoH Apnutam Apnuta Apnavam Apnuva Apnuma
Ap   loT   Apnotu ApnutAm Apnuvantu Apnuhi Apnutam Apnuta ApnavAni ApnavAva ApnavAma
Ap   viDi  ApnuyAt ApnuyAtAm ApnuyuH ApnuyAH ApnuyAtam ApnuyAta ApnuyAm ApnuyAva ApnuyAma

Sak  laT   Saknoti SaknutaH Saknuvanti Saknozi SaknuTaH SaknuTa Saknomi SaknuvaH SaknumaH
Sak  laN   aSaknot aSaknutAm aSaknuvan aSaknoH aSaknutam aSaknuta aSaknavam aSaknuva aSaknuma
Sak  loT   Saknotu SaknutAm Saknuvantu Saknuhi Saknutam Saknuta SaknavAni SaknavAva SaknavAma
Sak  viDi  SaknuyAt SaknuyAtAm SaknuyuH SaknuyAH SaknuyAtam SaknuyAta SaknuyAm SaknuyAva SaknuyAma

hi   laT   hinoti hinutaH hinvanti hinozi hinuTaH hinuTa hinomi hinuvaH hinumaH
hi   laN   ahinot ahinutAm ahinvan ahinoH ahinutam ahinuta ahinavam ahinuva ahinuma
hi   loT   hinotu hinutAm hinvantu hinu hinutam hinuta hinavAni hinavAva hinavAma
hi   viDi  hinuyAt hinuyAtAm hinuyuH hinuyAH hinuyAtam hinuyAta hinuyAm hinuyAva hinuyAma

ri   laT   riRoti riRutaH riRvanti riRozi riRuTaH riRuTa riRomi riRuvaH riRumaH
ri   laN   ariRot ariRutAm ariRvan ariRoH ariRutam ariRuta ariRavam ariRuva ariRuma
ri   loT   riRotu riRutAm riRvantu riRu riRutam riRuta riRavAni riRavAva riRavAma
ri   viDi  riRuyAt riRuyAtAm riRuyuH riRuyAH riRuyAtam riRuyAta riRuyAm riRuyAva riRuyAma
```

### Ātmanepada

```
aS    laT   aSnute aSnuvAte aSnuvate aSnuze aSnuvATe aSnuDve aSnuve aSnuvahe aSnumahe
aS    laN   ASnuta ASnuvAtAm ASnuvata ASnuTAH ASnuvATAm ASnuDvam ASnuvi ASnuvahi ASnumahi
aS    loT   aSnutAm aSnuvAtAm aSnuvatAm aSnuzva aSnuvATAm aSnuDvam aSnavE aSnavAvahE aSnavAmahE
aS    viDi  aSnuvIta aSnuvIyAtAm aSnuvIran aSnuvITAH aSnuvIyATAm aSnuvIDvam aSnuvIya aSnuvIvahi aSnuvImahi

stiG  laT   stiGnute stiGnuvAte stiGnuvate stiGnuze stiGnuvATe stiGnuDve stiGnuve stiGnuvahe stiGnumahe
stiG  laN   astiGnuta astiGnuvAtAm astiGnuvata astiGnuTAH astiGnuvATAm astiGnuDvam astiGnuvi astiGnuvahi astiGnumahi
stiG  loT   stiGnutAm stiGnuvAtAm stiGnuvatAm stiGnuzva stiGnuvATAm stiGnuDvam stiGnavE stiGnavAvahE stiGnavAmahE
stiG  viDi  stiGnuvIta stiGnuvIyAtAm stiGnuvIran stiGnuvITAH stiGnuvIyATAm stiGnuvIDvam stiGnuvIya stiGnuvIvahi stiGnuvImahi
```

## Testing

**Ordered traces** (`crates/panini/tests/trace.rs`), one per behaviour the
slice introduces:

| form | pins |
|---|---|
| `Apnoti` | the second 7.3.84 fires; the first declines on shape |
| `ApnutaH` | the second 7.3.84 blocked by 1.1.5 (apit ending) |
| `Apnuvanti` | 6.4.77 |
| `ApnavAni` | the 7.3.84 → 6.1.78 order, 6.1.78's vikaraṇa arm, and that 6.4.77 does not preempt it |
| `aSnavE` | 6.1.90's widened athematic arm |
| `Apnuhi` | 6.4.106 declines on the conjunct, and 6.4.101 no longer fires |
| `hinoti` | the *first* 7.3.84 blocked by 1.1.5 (śnu is ṅit) — no \*`henoti` |
| `hinvanti` | 6.4.87, and that it precedes 6.4.77 |
| `hinu` | 6.4.106 fires |
| `riRoti`, `riRvanti` | ṇatva over the new stems |
| `aSnuvate` | 7.1.5 then 6.4.77 |
| `aSnuzva` | 8.3.59 reached through the vikaraṇa |
| `aSnuvIta` | 7.2.79/7.2.80 then 6.4.77 then 6.1.66 |

**Guard tests** beside each new rule in its stage file, inside and outside, as
the codebase requires.

**Two dedicated no-delta tests** for the second 7.3.84, asserting it declines
on a kryādi `nI` stem and on a bhvādi `a` stem. These exist because the
no-delta claim in "The no-delta guard" is the slice's load-bearing safety
argument, and an argument in a comment is not a test.

**Enumerated membership test** on the *asaṁyogapūrva* helper over all six svādi
roots plus a kryādi and a bhvādi control.

**Regression, stated as an exact claim:** all **1296** pre-existing surface
forms **and all 1296 pre-existing traces** are byte-identical to `main` after
prep and after 5a. Unlike kryādi, which had to accept six trace
deltas from the apit widening, svādi's new rules all decline on existing paths,
so the expected delta is exactly zero. Any delta is a bug, not a re-pin.

**Gates:** `mise run mutants` reports zero survivors (invoking the
`cargo-mutants` binary directly rather than through the `mise` shim, which fails
in background shells; scope iteration with `mise exec -- cargo test -p
panini-prakriya`). `mise run fmt-check`, `lint` and `audit` clean.
`panini-lipi`'s roundtrip, property and fuzz targets are untouched.

## Risks

1. **The two ordering constraints on the second 7.3.84.** Both fail visibly —
   after 6.1.78 gives \*`ApnoAni`, after 6.4.87/6.4.77 gives \*`ApnuvAni` — so
   the goldens catch either and the traces pin them. Low risk, and the failure
   is loud.
2. **The *asaṁyogapūrva* predicate fails silently.** It is the one condition in
   the slice whose error produces well-formed-looking Sanskrit (\*`hinuhi`,
   \*`Apnu`). The enumerated helper test is the real guard here; the goldens
   are a backstop, not the primary defence.
3. **The duplicate `7.3.84` id reads as a copy-paste bug.** `tinanta_rule_order_is_pinned`
   will list `7.3.84` twice, and a fresh reader's first instinct is to delete
   one. Both entries carry comments naming 1.4.13 and citing the double 1.2.4
   as precedent, and the order test's own comment records why the repetition is
   correct.
4. **The `anga.rs` split is churn.** A large diff with no behaviour change is
   where a rule silently changes position. Mitigated by landing it alone and by
   the unchanged pinned order plus byte-identical goldens; nothing else in the
   prep 1 commit should be reviewed for grammar.
5. **`guna.rs` inherits `anga.rs`'s growth problem.** It starts at roughly 640
   lines and 5a takes it past 900. It is the right home for these rules now,
   but the next gaṇa that adds vowel-gradation work should expect to split
   again — juhotyādi's reduplication rules in particular do **not** belong
   there.
6. **The `Dhatu::id` change touches every root row and every golden row.** It
   is mechanical, but it is the one change in 5a that is not self-guarding: a
   missed call site keeps compiling if it happens to read `code`, and simply
   resolves the wrong √aś. The mitigation is to land the `id` field and the
   call-site migration as its own commit *before* the √aś row exists, so the
   suite is still provably at 1296 forms when it goes in — the same isolation
   logic as prep 1.
7. **Root-set discipline.** Four of the six roots were chosen partly for what
   they avoid (3.1.74, 7.1.100, the kṛ-specials, and above all 1.3.72). Adding
   a seventh svādi root casually will reintroduce one of those; the deferral
   list in "Scope" is the checklist.

## Success criteria

- All **1512** goldens validate `VALID`; the 1296 pre-existing surface forms
  **and traces** are byte-identical to `main`, with a delta of exactly zero.
- Neither prep commit changes a rule id, the order, or a form.
- Every new rule is pinned by an ordered trace *and* by inside/outside guard
  tests in its stage file.
- The second 7.3.84's inertness on existing paths is pinned by its own tests,
  not inferred from the goldens.
- The *asaṁyogapūrva* helper has an enumerated membership test naming all six
  roots.
- `hinoti` derives without guṇa on the root and `Apnoti` with guṇa on the
  vikaraṇa, and a test names that pair as the gaṇa's signature contrast.
- `Dhatu::id` resolves `aS.5` and `aS.9` to different rows, and a test asserts
  that both `aSnute` and `aSnAti` validate and that the two ids have different
  gaṇas.
- `adDi` and `vrIRIhi` are byte-identical after the 6.4.101 change, `AsE` after
  the 6.1.90 change, and `Bavati` / `SayIran` after the 6.1.78 change; a test
  names each pair as that change's tripwire.
- The flattened rule order is pinned verbatim at 62 ids (prep) and 67 ids (5a)
  — 6.4.101 is modified in place, so it adds no id.
- `derive` still carries no grammar branch — only the new `Tag::Svadi` aṅga tag.
- `docs/ARCHITECTURE.md` and `AGENTS.md` name svādi, the two-juncture guṇa, and
  the `anga.rs` / `guna.rs` split. **`README.md`'s Scope section is refreshed**
  — it currently still says "four gaṇas" and predates kryādi.
- `mise run mutants` reports **zero** survivors; `lint`, `fmt-check` and
  `audit` are clean.
