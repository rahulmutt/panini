# rudhādi (gaṇa 7), slice 7a — śnam and the infix

Every vikaraṇa the engine knows is a suffix. śap, śyan, śa, śnā, śnu all sit
between the aṅga and the ending, in the term slot the pipeline calls `SHAP`.
rudhādi's is not: 3.1.78 *ruḍhādibhyaḥ śnam* introduces a **mit** affix, and
1.1.47 *mid aco'ntyāt paraḥ* places a mit affix after the last vowel of what it
attaches to. `kft` + śnam is `kfnat`, not `kft` + `na`.

That is the slice's one new mechanism, and it is why rudhādi is split. 7a lands
śnam, its placement, the weak-stem elision it feeds, and the tripādī every
rudhādi weak cell walks through — on three roots that need nothing else. 7b
adds the consonant families (√bhañj, √piṣ, √indh) and closes the gaṇa.

## Scope

New: three roots, `Gana::Rudhadi`, ten sūtras, three of them optional. 108 new
cells, 145 new forms. `PARADIGM` goes 1512 → 1620 cells and 42 → 45 roots;
`ALTERNATES` goes 154 → 191 rows.

| id | dhātupāṭha | pada | laṭ prathama eka |
| --- | --- | --- | --- |
| `kft` | 07.0010 `kftI~` *veṣṭane* | parasmaipada | `kfRatti` |
| `his` | 07.0019 `hisi~` *hiṁsāyām* | parasmaipada | `hinasti` |
| `Kid` | 07.0012 `Ki\da~\` *dainye* | ātmanepada | `Kintte` |

None of the three collides with an existing SLP1 code, so all three ids equal
their codes and the `aS.5` qualification mechanism stays at one user. That does
not survive 7b: rudhādi also holds `vi\da~\` and `o~vijI~`, which collide with
divādi's `vid` and tudādi's `vij`.

Out of scope, deferred:

- **Ubhayapadī roots and 1.3.72 *svaritañitaḥ***, still deferred as in every
  gaṇa spec so far — but here the deferral costs the gaṇa its own name-root.
  Nine of rudhādi's 25 roots are marked `~^` in the dhātupāṭha (`ru\Di~^r`,
  `Bi\di~^r`, `Ci\di~^r`, `ri\ci~^r`, `vi\ci~^r`, `kzu\di~^r`, `yu\ji~^r`,
  `u~Cfdi~^r`, `u~tfdi~^r`), and vidyut derives both padas for all nine
  (`ruRadDi|runDe|rundDe`). **√rudh, √bhid, √chid and √yuj are therefore not in
  this slice or in 7b**, and rudhādi is the first gaṇa the engine covers
  without its eponymous root. A later 1.3.72 slice is what unlocks them.
  `Bu\ja~` is excluded for a different reason: 1.3.66 *bhujo'navane* forks it
  into `BuNkte|Bunakti` on sense, which is not a pada axis the engine models.
- **7b's consonant families**: 6.4.23's remaining witnesses, 6.4.24
  *aniditāṁ hala upadhāyāḥ kṅiti*, 8.2.30 *coḥ kuḥ*, 8.4.41 *ṣṭunā ṣṭuḥ*,
  8.2.41 *ṣaḍhoḥ kaḥ si*, 8.2.40 *jhaṣas tathor dho'dhaḥ*.

## The root set, and why these three

The three roots are chosen so that 7a pays for the gaṇa's spine and nothing
else. Each earns its place differently:

- **√kṛt** needs no new segmental rule at all — `kfRatti` is śnam plus 8.4.1
  *ṇatva*, which kryādi already landed and which already works across the
  ANGA/SHAP junction (`vf` + `nA` → `vfRAti`). What √kṛt brings instead is
  **fork depth**: its loṭ eka cells stack three optional rules and hold five
  and six forms respectively. See "The six-form cell" below.
- **√hiṃs** is the root that proves the anusvāra round trip is conditional.
  Its weak stem keeps its anusvāra (`hiMstaH`) where every other 7a weak stem
  resolves it (`kfntaH`), because 8.4.58's *yayi* does not cover `s`. It is
  also the only 7a root that takes 6.4.23 *śnān nalopaḥ*, and its `hinDi`
  is a third witness for 8.2.25 *dhi ca* on a stem shape that rule has not
  seen.
- **√khid** is the ātmanepada arm, and the lightest of the three ātmanepada
  roots rudhādi offers (the others are √indh, which wants 8.2.40, and √vid,
  which is √khid's phonological twin and collides with divādi's id besides).

## The rules

Ten sūtras. Five are universal to the gaṇa, three are optional, two are narrow.

### 3.1.78 `ruDAdiByaH Snam`

The vikaraṇa. Introduced for `Gana::Rudhadi` exactly as 3.1.69 / 3.1.73 /
3.1.77 / 3.1.81 are for their gaṇas, guarded on the aṅga's gaṇa tag, and
ordered among them by sūtra number: 3.1.77, **3.1.78**, 3.1.81 — all before
3.1.68 *kartari śap*, which they block.

Unlike its siblings it does not merely fill `SHAP`. It performs the split
1.1.47 mandates; see "Representation" below.

### 1.1.47 `midaco'ntyAtparaH` — cited, not implemented

A paribhāṣā, not an operation: it says where a mit affix lands. 3.1.78 does the
placing and cites 1.1.47 in a comment.

This matches how the engine already treats 1.4.13 *yasmāt pratyayavidhis…* and
1.1.5 *kṅiti ca* — both load-bearing, neither a `Rule` — and it matches vidyut,
whose trace for `kfRatti` emits 3.1.78 and never 1.1.47. A `Rule` here would put
an id in the engine's trace that the cited reference does not have.

### 6.4.111 `SnasorallopaH`

Drops śnam's `a` before a kṅit sārvadhātuka, producing the weak stem:
`kfnat` + `taH` → `kfnt` + `taH`, `hinas` + `taH` → `hins` + `taH`. This is
the rule that makes rudhādi's strong/weak split visible, and with the
representation below it is a term-local edit — delete `SHAP`'s `a`.

Its position is pinned by `hinDi`, which traces 6.4.101 *her dhiḥ* **then**
6.4.111: it goes at the end of the 6.4.10x block in `ADESHA`, after 6.4.101.

### 6.4.23 `SnAnnalopaH`

After śnam, the root's own nasal drops. In 7a this is √hiṃs alone: stored
`hins`, śnam gives `hinans`, 6.4.23 gives `hinas`, and `hinasti` follows.
Ordered in `ANGA`, before 6.4.111 — the trace order is 6.4.23 then 6.4.111 and
the reverse would elide the wrong vowel.

Its guard is written to the reachable slice, per the discipline that landed
8.3.59 and 8.2.25. 7b widens it for √bhañj, √und and √indh.

### 8.3.24 `naScApadAntasya Jali` and 8.4.58 `anusvArasya yayi parasavarRaH`

The round trip every rudhādi weak cell makes. 8.3.24 turns śnam's `n` into an
anusvāra before a jhal (`kfnt` → `kfMt`), and 8.4.58 turns the anusvāra back
into the following sound's homorganic nasal (`kfMt` → `kfnt`).

They look like a no-op pair and are not. **√hiṃs is the witness**: `hiMs` +
`taH` stops after 8.3.24, because 8.4.58 requires a *yay* to follow and the
anusvāra there is followed by the root's own `s`, which is śal, not yay. So
`hiMstaH` keeps its anusvāra while `kfntaH` does not. An implementation that
folded the two rules into one operation would derive `*hintaH`.

Placement in `TRIPADI`: 8.3.24 between 8.3.15 and 8.3.59; 8.4.58 after 8.4.55.

### 8.4.53 `JalAM jaS JaSi` — restored

Commit `9b7adee` deleted 8.4.53 as unreachable when 8.2.25 *dhi ca* replaced
the analysis slice 5d had shipped. √kṛt's loṭ madhyama eka reaches it:
`kfnd` + `Di` needs the `d` voiced-aspirated against the following `Dh`, giving
`kfndDi`. Restored in `TRIPADI` before 8.4.55.

The comment on its restoration should say plainly that it was removed for cause
and is back with a witness, so it does not read as a revert of a considered
decision.

### 8.4.65 `Jaro Jari savarRe` (vikalpa)

Optionally elides a jhar before a savarṇa jhar: `kfnttaH` → `kfntaH`,
`kfndDi` → `kfnDi`, `Kintte` → `Kinte`. It fires wherever the weak stem's
final consonant meets a homorganic ending — which in 7a is most of √kṛt's and
√khid's weak cells, and none of √hiṃs's (`hiMstaH`'s `s` and `t` are not
savarṇa).

**It must be ordered before 8.4.56 *vāvasāne*.** Both are optional and both sit
at the end of `TRIPADI`. If 8.4.56 ran first, only one of 8.4.65's two branches
would receive the pausal fork and `kfnttAt` would never be derived.

### 8.2.73 `tipyanasteH` (obligatory), 8.2.74 `sipi DAto rurvA` and 8.2.75 `daSca` (vikalpa)

These three are one alternation, and getting them apart is the fiddliest part
of the slice. Before tip and sip, a consonant-final stem surfaces either with a
stop or with a visarga: `ahinad` ~ `ahinaH`, `akfRad` ~ `akfRaH`.

**√kṛt is already served by an existing rule.** `akfRat` + tip `t` gives the
conjunct `akfRatt`; 8.2.23 *saṁyogāntasya lopaḥ* — already in the pipeline, and
guarded generally on the word's last two characters — drops the second `t`, and
8.2.39 *jhalāṁ jaśo'nte* voices the survivor to `akfRad`. So √kṛt needs only
the optional half.

**√hiṃs is not**, and this is the gap. `ahinas` + tip `t` reduces the same way
to `ahinas`, but **8.2.39 declines**: its guard is narrow to a final `t` by
design, because a final `s` must become a visarga through 8.2.66 / 8.3.15,
which is 8.2.39's apavāda. Left alone the engine would derive `*ahinaH` for
laṅ prathama eka. **8.2.73 *tipy anasteḥ* is what supplies the `d`** — before
tip, a dhātu other than √as takes `d` for its final — and nothing currently in
the engine does that job.

So the three divide as:

| rule | kind | supplies | witness |
| --- | --- | --- | --- |
| 8.2.73 | obligatory | `s` → `d` before **tip or sip** | `ahinad` |
| 8.2.74 | vikalpa | `s` → *ru* before sip | `ahinaH` |
| 8.2.75 | vikalpa | `d` → *ru* before sip | `akfRaH` |

**8.2.74 is ordered before 8.2.73**, which is non-numeric and deliberate. 8.2.74
replaces the *dhātu's own final* — the `s` — so it must see `ahinas`, not the
`ahinad` that 8.2.73 would already have produced. The declined branch then falls
through to 8.2.73 and gets its `d`. Ordering them numerically instead would
leave 8.2.74 with no `s` to act on, and `ahinaH` would never be derived.

8.2.73's guard must cover **sip as well as tip**, even though the sūtra says
*tipi*. That is a deliberate widening, and the reason is structural: 8.2.74's
*vā* presupposes the `d` as the alternant it is optional against, so the
declined branch has to reach it somehow. Record it in the rule comment as an
over-application of the stated condition, in the shape the previous slice used
for 7.1.35's *āśiṣi*.

The whole group is ordered in `TRIPADI` after 8.2.39 and before 8.3.15 — before
8.3.15 because otherwise `ahinas` becomes a visarga before 8.2.73 can reach it.

**This also settles which branch is index 0.** vidyut is internally
inconsistent here — for √kṛt it attributes `akfRaH` to 8.2.75 and leaves
`akfRad` ruleless, but for √hiṃs it attributes `ahinad` to 8.2.74 and leaves
`ahinaH` ruleless, the same alternation with opposite bases. Under the model
above the question does not arise: the `d` comes from an **obligatory** rule in
both roots (8.2.39 for √kṛt, 8.2.73 for √hiṃs), so `akfRad` and `ahinad` are
index 0 by construction, and the visarga forms are the vikalpa branches. The
tables below reflect that.

### Reuses that gain witnesses

8.4.1 / 8.4.2 *ṇatva* across the ANGA/SHAP junction (`kfRatti`); 8.2.25
*dhi ca*, third witness (`hinDi`); 6.4.101 *her dhiḥ*; 7.1.35 *tātaṅ*; 8.4.56
*vāvasāne*; 8.2.39 *jhalāṁ jaśo'nte*.

### √hiṃs is stored as `hins`

The dhātupāṭha form is `hisi~`, idit, and 7.1.58 *idito num dhātoḥ* inserts the
num that gives `hins`. The engine has no it-marker machinery — roots are stored
post-it-elision throughout — so 7.1.58 is **not derivable in the current model**
and the root is stored in its post-num form, with 7.1.58 cited in a comment.

This is the `stiG` precedent exactly: svādi's √ṣṭigh is "stored post-6.1.64
*dhātvādeḥ ṣaḥ saḥ*: no rule in the engine performs that substitution, so it is
a stated simplification, not a derivation step." The same sentence, with the
same force, belongs on `his`. It is a simplification and must not later be
mistaken for an implemented rule.

## Representation: where śnam lives

The term layout is three fixed slots — `ANGA`, `SHAP`, `ENDING` — and every
rule addresses them by those constants. An infix fits none of them cleanly.

**The chosen model: `ANGA` holds the root's head, `SHAP` holds śnam followed by
whatever the root had after its last vowel.** `kft` becomes `[kf, nat, ti]`,
surface `kfnatti` → `kfRatti`. `hins` becomes `[hi, nans, ti]`, and 6.4.23
reduces `SHAP` to `nas`.

What this buys:

- **The three-slot layout is untouched.** No existing rule changes, and
  `following_sarvadhatuka` / `sound_before_ending` / `shnu_asamyogapurva` all
  keep working unmodified.
- **6.4.111 is a term-local edit** — delete `SHAP`'s `a` — rather than a
  positional search inside a merged string. So is 6.4.23.
- **Cross-term ṇatva is already proven.** kryādi derives `vfRAti` from
  `vf` + `nA`; 8.4.1 seeing `kf` + `nat` is the same shape.

What it costs, stated plainly: **`terms[SHAP].text` is no longer purely the
vikaraṇa.** It is śnam plus a fragment of the root. Two consequences need
documenting in `terms.rs`, beside the existing 2.4.72 caveat:

- Any rule that reads `SHAP` expecting the vikaraṇa's own text must guard on
  the gaṇa. The engine already lives with a weaker version of this — 6.4.107
  leaves `terms[SHAP].text == "n"`, which is why `shnu_asamyogapurva` and
  `sound_before_ending` both have ordering constraints written around it.
- 6.4.23 *śnān nalopaḥ* deletes a nasal that came from the **root** but now
  lives in `SHAP`. Without a comment saying so, that rule reads as a bug.

Two alternatives were considered and rejected. **`ANGA` holding the whole
infixed stem `kfnat` with `SHAP` empty** (the adādi śap-luk shape) is truer to
1.4.13's aṅga, but forces 6.4.111 and 6.4.23 to locate a character by position
inside a merged string — the failure mode this codebase has been bitten by, and
the one the `terms.rs` header exists to warn about. **An explicit infix-offset
field on `Term`** is honest but adds state one gaṇa uses.

## Placement and ordering

`TINANTA_RULES` gains entries in three stages. `tinanta_rule_order_is_pinned`
gains the ids in these positions:

| rule | stage | ordered by |
| --- | --- | --- |
| 3.1.78 | `VIKARANA` | between 3.1.77 and 3.1.81, before 3.1.68 |
| 6.4.23 | `ANGA` | before 6.4.111 |
| 6.4.111 | `ADESHA` | after 6.4.101 — pinned by `hinDi` |
| 8.2.74, 8.2.73, 8.2.75 | `TRIPADI` | after 8.2.39, before 8.3.15; **8.2.74 before 8.2.73** — pinned by `ahinaH` / `ahinad` |
| 8.3.24 | `TRIPADI` | between 8.3.15 and 8.3.59 |
| 8.4.53 | `TRIPADI` | before 8.4.55 |
| 8.4.58 | `TRIPADI` | immediately after 8.4.55 — see the caveat below |
| 8.4.65 | `TRIPADI` | **before 8.4.56** — pinned by `kfnttAt` |

`TRIPADI` therefore reads, in full:

```
8.2.77, 8.2.23, 8.2.25, 8.2.39, 8.2.74, 8.2.73, 8.2.75, 8.3.15, 8.3.24,
8.3.59, 8.4.53, 8.4.55, 8.4.1, 8.4.2, 8.4.58, 8.4.65, 8.4.56
```

`exactly_the_pinned_vikalpa_rules_are_optional` goes from four ids to seven:
`7.1.35, 3.4.111, 6.4.107, 8.4.65, 8.2.74, 8.2.75, 8.4.56`.

### 8.4.58 must sit after ṇatva, because of a fold already in the code

`tripadi.rs`'s `is_natva_target` — the shared precondition for 8.4.1 and 8.4.2
— **already folds 8.3.24 in as a guard**, with the comment that "a non-padānta
n before a jhal has ALREADY become an anusvāra by the time the 8.4 rules run,
and 8.4.58 restores it afterwards", and the note "retire both in favour of the
real rules when liṭ/luṅ bring 8.3.24 in". It was a simplification taken when
the engine had no anusvāra machinery.

This slice brings the real 8.3.24, but **guarded to rudhādi**, so the fold does
not become retirable: `BAzante`'s `n` is still an `n` when ṇatva runs, and
without the fold 8.4.1 would derive `*BAzaRte`. The fold stays.

Given that it stays, 8.4.58's position is *constrained*, not free. Placed
before ṇatva, `kfMt` would already be `kfnt` when 8.4.1 looks, and the weak
stem would decline only by falling through the stale fold. Placed after,
`kfntaH` declines for the right reason — its nasal is genuinely an anusvāra —
while `kfRatti`, whose `n` precedes a vowel so 8.3.24 never fired, still takes
ṇatva. **8.4.58 is ordered after 8.4.2.**

The constraint is retired together with the fold, whenever a slice widens
8.3.24 past rudhādi.

## What PARADIGM and ALTERNATES become

### PARADIGM — three new 4 × 9 blocks

Index 0 is the derivation with no optional rule applied, so where 8.4.65 forks
a cell, `PARADIGM` holds the **un-elided** member (`kfnttaH`, not `kfntaH`).

```rust
    (
        "kft",
        "laT",
        [
            "kfRatti", "kfnttaH", "kfntanti", "kfRatsi", "kfntTaH", "kfntTa", "kfRatmi",
            "kfntvaH", "kfntmaH",
        ],
    ),
    (
        "kft",
        "laN",
        [
            "akfRad", "akfnttAm", "akfntan", "akfRad", "akfnttam", "akfntta", "akfRatam",
            "akfntva", "akfntma",
        ],
    ),
    (
        "kft",
        "loT",
        [
            "kfRattu", "kfnttAm", "kfntantu", "kfndDi", "kfnttam", "kfntta", "kfRatAni",
            "kfRatAva", "kfRatAma",
        ],
    ),
    (
        "kft",
        "viDiliN",
        [
            "kfntyAd", "kfntyAtAm", "kfntyuH", "kfntyAH", "kfntyAtam", "kfntyAta", "kfntyAm",
            "kfntyAva", "kfntyAma",
        ],
    ),
    (
        "his",
        "laT",
        [
            "hinasti", "hiMstaH", "hiMsanti", "hinassi", "hiMsTaH", "hiMsTa", "hinasmi",
            "hiMsvaH", "hiMsmaH",
        ],
    ),
    (
        "his",
        "laN",
        [
            "ahinad", "ahiMstAm", "ahiMsan", "ahinad", "ahiMstam", "ahiMsta", "ahinasam",
            "ahiMsva", "ahiMsma",
        ],
    ),
    (
        "his",
        "loT",
        [
            "hinastu", "hiMstAm", "hiMsantu", "hinDi", "hiMstam", "hiMsta", "hinasAni",
            "hinasAva", "hinasAma",
        ],
    ),
    (
        "his",
        "viDiliN",
        [
            "hiMsyAd", "hiMsyAtAm", "hiMsyuH", "hiMsyAH", "hiMsyAtam", "hiMsyAta", "hiMsyAm",
            "hiMsyAva", "hiMsyAma",
        ],
    ),
    (
        "Kid",
        "laT",
        [
            "Kintte", "KindAte", "Kindate", "Kintse", "KindATe", "KindDve", "Kinde",
            "Kindvahe", "Kindmahe",
        ],
    ),
    (
        "Kid",
        "laN",
        [
            "aKintta", "aKindAtAm", "aKindata", "aKintTAH", "aKindATAm", "aKindDvam", "aKindi",
            "aKindvahi", "aKindmahi",
        ],
    ),
    (
        "Kid",
        "loT",
        [
            "KinttAm", "KindAtAm", "KindatAm", "Kintsva", "KindATAm", "KindDvam", "KinadE",
            "KinadAvahE", "KinadAmahE",
        ],
    ),
    (
        "Kid",
        "viDiliN",
        [
            "KindIta", "KindIyAtAm", "KindIran", "KindITAH", "KindIyATAm", "KindIDvam",
            "KindIya", "KindIvahi", "KindImahi",
        ],
    ),
```

### ALTERNATES — 37 new rows

The vikalpa key names the optional rules applied on the branch that derives the
form, `+`-joined in pipeline order, and
`every_alternate_names_the_vikalpa_rules_that_produced_it` checks it against the
branch's own log.

```rust
    ("kft", "laT", 1, "kfntaH", "8.4.65"),
    ("kft", "laT", 4, "kfnTaH", "8.4.65"),
    ("kft", "laT", 5, "kfnTa", "8.4.65"),
    ("kft", "laN", 0, "akfRat", "8.4.56"),
    ("kft", "laN", 1, "akfntAm", "8.4.65"),
    ("kft", "laN", 3, "akfRat", "8.4.56"),
    ("kft", "laN", 3, "akfRaH", "8.2.75"),
    ("kft", "laN", 4, "akfntam", "8.4.65"),
    ("kft", "laN", 5, "akfnta", "8.4.65"),
    ("kft", "loT", 0, "kfnttAd", "7.1.35"),
    ("kft", "loT", 0, "kfntAd", "7.1.35+8.4.65"),
    ("kft", "loT", 0, "kfnttAt", "7.1.35+8.4.56"),
    ("kft", "loT", 0, "kfntAt", "7.1.35+8.4.65+8.4.56"),
    ("kft", "loT", 1, "kfntAm", "8.4.65"),
    ("kft", "loT", 3, "kfnDi", "8.4.65"),
    ("kft", "loT", 3, "kfnttAd", "7.1.35"),
    ("kft", "loT", 3, "kfntAd", "7.1.35+8.4.65"),
    ("kft", "loT", 3, "kfnttAt", "7.1.35+8.4.56"),
    ("kft", "loT", 3, "kfntAt", "7.1.35+8.4.65+8.4.56"),
    ("kft", "loT", 4, "kfntam", "8.4.65"),
    ("kft", "loT", 5, "kfnta", "8.4.65"),
    ("kft", "viDiliN", 0, "kfntyAt", "8.4.56"),
    ("his", "laN", 0, "ahinat", "8.4.56"),
    ("his", "laN", 3, "ahinat", "8.4.56"),
    ("his", "laN", 3, "ahinaH", "8.2.74"),
    ("his", "loT", 0, "hiMstAd", "7.1.35"),
    ("his", "loT", 0, "hiMstAt", "7.1.35+8.4.56"),
    ("his", "loT", 3, "hiMstAd", "7.1.35"),
    ("his", "loT", 3, "hiMstAt", "7.1.35+8.4.56"),
    ("his", "viDiliN", 0, "hiMsyAt", "8.4.56"),
    ("Kid", "laT", 0, "Kinte", "8.4.65"),
    ("Kid", "laT", 5, "KinDve", "8.4.65"),
    ("Kid", "laN", 0, "aKinta", "8.4.65"),
    ("Kid", "laN", 3, "aKinTAH", "8.4.65"),
    ("Kid", "laN", 5, "aKinDvam", "8.4.65"),
    ("Kid", "loT", 0, "KintAm", "8.4.65"),
    ("Kid", "loT", 5, "KinDvam", "8.4.65"),
```

### The six-form cell

The 108 new cells distribute as 82 with one form, 20 with two, 4 with three,
one with five, and one with six. The last two are √kṛt's loṭ eka cells, and
they are the deepest forks the engine has produced — today's maximum is three.

loṭ madhyama eka stacks 7.1.35, 8.4.65 and 8.4.56:

| branch | form |
| --- | --- |
| — | `kfndDi` |
| 8.4.65 | `kfnDi` |
| 7.1.35 | `kfnttAd` |
| 7.1.35+8.4.65 | `kfntAd` |
| 7.1.35+8.4.56 | `kfnttAt` |
| 7.1.35+8.4.65+8.4.56 | `kfntAt` |

**k = 3 with six branches, against a 2³ bound of eight.** The two missing
subsets are the ones where 8.4.56 fires on the non-tātaṅ branch, which it
declines because `kfndDi` and `kfnDi` are vowel-final. This is the sharpest
available witness for the branch-count claim `docs/ARCHITECTURE.md` had to
correct in the previous slice — that the count is the number of distinct
subsets of optional rules that actually apply, bounded by 2^k and reaching it
only when every optional rule fires on every branch.

It is also the reason √kṛt belongs in 7a rather than 7b: it stresses the fork
machinery without needing a single new segmental rule to do it.

## Verification

### The audit

The `examples/panini_full_audit.rs` probe from the previous slice, re-run over
45 roots × 4 lakāras × 9 cells, printing the complete derivation set per cell
rather than one form. The claim it has to sustain is the one the previous slice
established: the engine's derivation set equals vidyut's, in every cell,
exactly — no over-generation and no under-generation.

√hiṃs laṅ madhyama eka is the one cell where the *sets* match but the
attribution does not: vidyut treats `ahinaH` as ruleless there and `ahinad` as
8.2.74's, where the engine has it the other way round. The audit compares sets,
so it passes; the difference is confined to `ALTERNATES`' key column, which is
checked against the engine's own log rather than vidyut's.

### Per-rule guard tests

Beside each rule in its stage file, per the repo's discipline. The ones that
need enumeration rather than a single case:

- **8.4.58's *yayi* condition**, enumerated over `kfnt` (fires) and `hiMs`
  (declines). A predicate that fires unconditionally still produces plausible
  Sanskrit for two of the three roots.
- **8.4.65's *savarṇa* condition**, enumerated over `kfntt` / `KindD` (fires)
  and `hiMst` (declines).
- **6.4.111's kṅit condition**, over a strong cell and a weak one.
- **8.2.73's ending condition**, enumerated over tip and sip (both fire) and a
  third ending such as `tas` (declines) — the widened guard is a stated
  divergence and needs its boundary pinned, not just its positive case.

### `terms.rs`

A unit test for the new `SHAP`-holds-śnam-plus-tail invariant, in the shape of
the existing `shnu_asamyogapurva` enumeration: for each of the three roots, the
term split 3.1.78 produces, including √hiṃs's pre-6.4.23 `nans`.

### `derivation_tests.rs` and `trace.rs`

Three trace pins carry the slice's invisible decisions — each produces
plausible Sanskrit when wrong, so each must assert rule *order*, not just the
surface form:

- **`kfRatti`** — 3.1.78 splitting the root, then 8.4.1 across the junction.
- **`hinDi`** — 6.4.101 before 6.4.111, and 8.2.25 on a new stem shape.
- **`kfntAt`** — 8.4.65 before 8.4.56, asserting the presence of both. The
  wrong order yields a real word, just not this one.
- **`ahinaH`** — 8.2.74 firing above 8.2.73, on `ahinas` rather than `ahinad`.
  The numeric order still derives `ahinad`, so only an order assertion catches
  it.

### `roundtrip.rs`

Unchanged in shape; it walks whatever `PARADIGM` holds.

### Mutation testing

`mise run mutants` with the explicit generous timeout. The new guards are
narrow and each arm has a witness, so the survivor count should stay at zero;
a survivor means an arm has no witness and the guard should shrink rather than
the test grow.

## Documentation

- **AGENTS.md**: rudhādi added to the gaṇa list as **partial**, not complete —
  the first gaṇa in the repo to be described that way, and the note should say
  why (ubhayapadī, and 7b pending). The vikalpa set becomes seven rules. The
  `terms.rs` caveat about `SHAP` gains its second entry.
- **README.md**: "six gaṇas … all fully covered" becomes six complete plus one
  partial; the multi-form-cell sentence gains the six-form case.
- **docs/ARCHITECTURE.md**: the stage table's rule ranges gain the nine new
  ids; the branch-count paragraph gains `kfnDi`'s cell as its witness.

## Risks

- **The `SHAP` representation is the slice's irreversible decision.** Every
  rudhādi rule in 7a and 7b is written against it, and changing it later means
  rewriting all of them. It is recorded above with its alternatives and the
  reason each was rejected, so a later reader can tell it was chosen rather
  than fallen into.
- **8.4.58's order against ṇatva turned out to be constrained**, by a fold in
  `is_natva_target` rather than by any sūtra. The fold and the constraint are
  retired together, and the rule comment says so.
- **8.2.73's guard is widened past its sūtra's stated *tipi*** to cover sip as
  well, because 8.2.74's *vā* presupposes the `d` as its alternant. A
  deliberate divergence, recorded so it is not later mistaken for a bug — the
  same treatment the previous slice gave 7.1.35's *āśiṣi* over-application.
- **8.2.74 sits above 8.2.73, against sūtra order.** Nothing in the code
  enforces it; the `ahinaH` trace pin is the guard, and it must assert that
  8.2.74 fired on `ahinas` rather than merely that the surface is right — the
  numeric order still produces a real word (`ahinad` on both branches), it just
  never produces `ahinaH`.
- **6.1.68 `halNyAbByo dIrGAt sutisyapfktaM hal` was investigated and is not
  needed.** It appears in vidyut's `akfRat` trace, but the engine reaches the
  same result through 8.2.23 *saṁyogāntasya lopaḥ*, whose guard is written
  generally on the word's last two characters and already covers `akfRatt` and
  `ahinast`. The reason it has never fired before is that √ad — the only
  consonant-final aṅga in the current set — takes 7.3.100 *adaḥ sarveṣām*,
  which inserts the `a` of `Adat` and leaves the ending with a vowel before it.
- **√hiṃs stored as `hins`** is a stated simplification, not a derivation
  step. If it is later read as one, 7.1.58 looks implemented when it is not.
