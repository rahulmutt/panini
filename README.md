# panini

A Rust library and CLI that validates a single Sanskrit word against Pāṇini's
Aṣṭādhyāyī and returns the sequence of sūtras that derive it.

## Quick start

```
mise install          # pins Rust toolchain
mise run test          # runs the workspace test suite
cargo run -p panini-cli -- check 'bhavati' --trace
```

## Scope

Finite verbs (*tiṅanta*), nine gaṇas covered, eight of them fully —
*bhvādi* (1, vikaraṇa śap), *divādi* (4, śyan), *tudādi* (6, śa), *adādi*
(2, śap luk'd), *kryādi* (9, śnā), *svādi* (5, śnu) and *rudhādi* (7,
śnam) — plus *tanādi* (8, vikaraṇa the bare *u* of 3.1.79), **complete**
at all ten of its dhātupāṭha rows: √tan, √san,
√kṣaṇ, √kṣiṇ, √ṛ, √tṛ and √ghṛ (all seven ubhayapadī by 1.3.72) plus √van
and √man (both ātmanepadī by 1.3.12), curated in slice 8a; and √kṛ
(`08.0010`), the tenth and last row, the one root 3.1.79 itself names
(*tanādikṛñbhya uḥ*), ubhayapadī by 1.3.72 and curated in slice 8b behind
three new root-keyed specials — 6.4.110 *ata ut sārvadhātuke*, 6.4.108
*nityaṁ karoteḥ* (nitya where 6.4.107 is optional for every other tanādi
root, so `kurvaH`/`kurmaH` derive as single branches with no alternate)
and 6.4.109 *ye ca* — and 8.2.77 *hali ca*'s own guard, 8.2.79 *na
bhakurchurām*, which declines 8.2.77's lengthening on √kṛ's `kur` aṅga
(`kurvanti`, not `*kUrvanti`) — and *juhotyādi* (3, the ślu gaṇa: 2.4.75
elides śap by ślu and 6.1.10 reduplicates the root into the `ABHYASA`
slot), **partial** at 8 of its 26 dhātupāṭha rows, √hu (`03.0001`,
*juhoti*) and √ki (`03.0020`, *ciketi*), curated in slice 3a behind
7.4.62 *kuhoś cuḥ*, 7.1.4 *ad abhyastāt*, 3.4.109 with 7.3.83 *jusi ca*,
6.4.82 *er anekāco'saṁyogapūrvasya* and 8.4.54 *abhyāse car ca*, with
6.4.87 and 6.4.101 grown their √hu arms; and √bhī (`03.0002`, *bibheti*)
and √hrī (`03.0003`, *jihreti*), both parasmaipadī, curated in slice 3b
behind two new sūtras shaping the abhyāsa of a consonant-initial,
long-vowel root — 7.4.60 *halādiḥ śeṣaḥ* and 7.4.59 *hrasvaḥ* — with
6.4.82 widened from its 3a shape to cover a long `I` (first exercised by
√bhī) and 6.4.77 gaining an iyaṅ arm for the roots 6.4.82 declines on as
saṁyogapūrva (first exercised by √hrī), and 6.4.115 *bhiyo'nyatarasyām*
landing as the engine's ninth vikalpa rule — root-keyed to √bhī, forking
across all four lakāras, and stacking with 7.1.35 in loṭ; and √dā
(`03.0010`, *dadāti*) and √dhā (`03.0011`, *dadhāti*), both ubhayapadī, and
√mā (`03.0007`, *mimīte*) and √hā (`03.0008`, *jihīte*), both ātmanepadī,
curated in slice 3c behind the dhātupāṭha number reaching the pipeline —
1.1.20 *dādhā ghv adāp* becomes a saṁjñā decided by row, since √dāp is `dA`
too — and 7.4.76 *bhṛñām it*, 6.4.119 *ghvasor eddhāv abhyāsalopaś ca*,
6.1.88 *vṛddhir eci* and 8.2.38 *dadhas tathoś ca* (run after 8.4.54, out of
sūtra order), with 6.4.113 moved above 6.4.112 and both given abhyasta
arms, and 8.2.40 given its *adhaḥ*. rudhādi is
complete at all
twenty-five of its own roots (√kṛt, √hiṃs, √khid, √bhañj, √piṣ, √indh, √rudh,
and — curated in slice 7c — √bhid, √kṣud, √yuj and √tṛd, and — curated in
the 8.2.30/8.2.39 generalization slice — √ric and √vic, and — curated in
slice 7d, on the audited numbers alone with no new sūtra — √śiṣ, √und,
√añj, √tañc, √vij, √vṛj, √pṛc and √vid, and — curated in slice 7e, with
7.3.92 *tṛṇaha im*, 8.2.31 *ho ḍhaḥ* and 8.3.13 *ḍho ḍhe lopaḥ* — √tṛh, and
— curated in slice 7f, with 6.1.73 *che ca* and 8.4.40 *stoḥ ścunā ścuḥ* —
√chid and √chṛd, and — curated in the √bhuj/1.3.66 slice, behind 1.3.66
*bhujo'navane* — √bhuj). √rudh, the
gaṇa's eponym,
arrived with 1.3.72 *svaritañitaḥ* as the engine's first ubhayapadī root, so
the ubhayapada deferral itself is discharged: 1.3.72 is no longer what keeps
any root out. **None of rudhādi's 25 remain out** — √bhuj derives both padas
behind 1.3.66 *bhujo'navane* instead: vidyut derives all 72 of its cells,
and 1.3.66 is a root-keyed pada assignment structurally identical to
1.3.72's, which this engine already implements, implemented as an
unconditional ubhayapada assignment so the trace always credits 1.3.66
rather than falling through to 1.3.72. What 1.3.66 does not model is the
**sense** restriction *anavane* imposes, recorded as unimplemented on
1.3.72's own precedent,
since neither engine models sense. *parasmaipada* and *ātmanepada*
(which padas a root admits is a curated verdict on its table row), over a
curated 85-root set, in four lakāras: *laṭ* (present), *laṅ* (imperfect), *loṭ*
(imperative), and *vidhiliṅ* (optative). A cell may have more than one valid
form where an optional (*vikalpa*) sūtra applies — `hinvaH` and `hinuvaH` are
both correct — and in fact 719 of the 3852 cells hold more than one form: 554
hold two, 121 hold three (`Bavatu`, `BavatAd`, `BavatAt`, and — new in
slice 3b — √hrī's loṭ prathama and madhyama eka, and — new in slice 3c —
√dā's and √dhā's, each by 7.1.35/8.4.56), eighteen hold four
(rudhādi's √piṣ loṭ madhyama eka, and — new in slice 7d — √śiṣ's, and — new
in slice 8a — fifteen more spread across tanādi's four ik-upadhā roots kziR,
fR, tfR and GfR, and — new in slice 3b — √bhī's vidhiliṅ prathama eka,
forking on 6.4.115 alongside 8.4.56), nine hold
five (and, new in slice 3b, √bhī's loṭ prathama eka, forking on
7.1.35/6.4.115/8.4.56), and seventeen hold six — the loṭ
parasmaipada madhyama eka of rudhādi's √kṛt, √rudh, √bhid, √kṣud, √tṛd, √und
and — new in slice 7f — √chid and √chṛd (eight cells), tied for the record
until this slice, each holding
six valid readings of the
one cell: `kfndDi` / `kfnDi` / `kfnttAd` / `kfntAd` / `kfnttAt` / `kfntAt` for
√kṛt, `rundDi` / `runDi` / `rundDAd` / `runDAd` / `rundDAt` / `runDAt` for
√rudh, `undDi` / `unDi` / `unttAd` / `untAd` / `unttAt` / `untAt` for
√und, `CindDi` / `CinDi` / `CinttAd` / `CintAd` / `CinttAt` / `CintAt` for
√chid, and `CfndDi` / `CfnDi` / `CfnttAd` / `CfntAd` / `CfnttAt` / `CfntAt`
for √chṛd — and, new in slice 8a, the loṭ parasmaipada **prathama and
madhyama** eka of tanādi's four ik-upadhā roots kziR, fR, tfR and GfR (eight
more cells, taking the record to sixteen): where the earlier eight fork on
7.1.35/8.4.65/8.4.56, these fork on 7.1.35/7.3.86/8.4.56 instead, since
tanādi's u-final stems give 8.4.65 nothing to elide and 7.3.86's guṇa/aguṇa
alternation stands in its place — fR's own prathama eka holds `fRotu` /
`arRotu` / `fRutAd` / `fRutAt` / `arRutAd` / `arRutAt`; and, new in slice
3b, √bhī's loṭ parasmaipada madhyama eka (one more cell, taking the record
to seventeen): `biBIhi` / `biBihi` / `biBItAd` / `biBitAd` / `biBItAt` /
`biBitAt`, reaching six by 7.1.35/6.4.115/8.4.56 — a third distinct k = 3
stack against the same 2³ bound of eight, beside rudhādi's 8.4.65 route and
tanādi's 7.3.86 route. Nothing in the suite
forks deeper than six. fR's own laṅ cells — all eighteen of them, both
padas — show a different mechanism: each is 7.3.86-eligible, but the guṇa
and aguṇa branches always converge on the same surface once 6.1.90's
āṭ-vṛddhi ekādeśa merges the augment into `f`, so none of the eighteen
carries a live second branch — the corpus's first convergent-fork
collapse, not a one-cell anomaly. It is starkest at prathama eka, the one
laṅ cell that also stacks 8.4.56's pausal cartva: tfR's, GfR's and
kziR's own prathama eka hold **four** forms there (`atfRod` / `atfRot` /
`atarRod` / `atarRot` for tfR), but fR's holds only **two** (`ArRod` /
`ArRot`) — the missing branch is 7.3.86's, not 8.4.56's. √yuj, ubhayapadī like the
other three roots 7c curated, does *not* fork that deep: 8.2.30 *coḥ kuḥ*
replaces its palatal `j` with the velar `g` (which 8.4.55 *khari ca* later
devoices to `k` before a `t`), and a velar is never savarṇa with the dental
`t`/`D` that follows, so it never reaches the 8.4.65 branch the dental-final
roots take. A root may also admit **both**
padas — twenty-two roots that admit both padas in the curated set
(twenty-one ubhayapadī by 1.3.72: √nī, √tud, √rudh, √bhid, √kṣud, √yuj,
√tṛd, √ric, √vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛ, √tṛ, √ghṛ,
√kṛ, √dā and √dhā; and √bhuj by 1.3.66) derive a full parasmaipada and a full
ātmanepada paradigm, so a single surface can be genuinely pada-ambiguous.
√van, by contrast, never enters this bucket: it is ātmanepadī by its own
anudātta marker (1.3.12), and while vidyut-prakriya additionally derives a
parasmaipada `vanoti` via the gaṇasūtra Kaumudī 2547.2, that is recorded
here, not modelled, on 1.3.72's own sense-restriction precedent, so this
engine's √van has no parasmaipada branch to collide against.
Forty-eight surfaces are pada-ambiguous, each of them a pinned cell in both
padas at once: `ArRuta`, `BinttAm`, `BuNktAm`, `CfnttAm`, `CinttAm`, `DattAm`,
`GfRutAm`, `aBintta`, `aBuNkta`, `aDatta`, `aGfRuta`, `acCfntta`, `acCintta`,
`adatta`, `akuruta`, `akzaRuta`, `akziRuta`, `akzuntta`, `anayata`, `ariNkta`,
`arundDa`, `asanuta`, `atanuta`, `atfRuta`, `atfntta`, `atudata`, `aviNkta`,
`ayuNkta`, `dattAm`, `fRutAm`, `kurutAm`, `kzaRutAm`, `kziRutAm`, `kzunttAm`,
`nayatAm`, `nayetAm`, `nayeta`, `riNktAm`, `rundDAm`, `sanutAm`, `tanutAm`,
`tfRutAm`, `tfnttAm`, `tudatAm`, `tudetAm`, `tudeta`, `viNktAm` and `yuNktAm` —
`rundDAm`, for instance, is √rudh's loṭ parasmaipada prathama dvi *and* its loṭ
ātmanepada prathama eka, and tanādi's seven ubhayapadī roots contribute a new
shape: `atanuta` is both √tan's laṅ ātmanepada prathama eka and its laṅ
parasmaipada madhyama bahu, and `tanutAm` is both its loṭ ātmanepada prathama
eka and its loṭ parasmaipada prathama dvi. √kṛ (slice 8b) contributes the same
shape: `akuruta` (laṅ ātmanepada prathama eka / parasmaipada madhyama bahu) and
`kurutAm` (loṭ ātmanepada prathama eka / parasmaipada prathama dvi). That
enumeration is no longer maintained by hand:
`pada_ambiguous_surfaces_are_exactly_these` in
`crates/panini/tests/paradigm/main.rs` walks `PARADIGM` and asserts exactly this
set. It is therefore a list of ambiguous **pinned cells**. An *alternate* form
can be pada-ambiguous in its own right — √rudh's `runDAm` is the 8.4.65
alternate of both those `rundDAm` cells — but alternates live in `ALTERNATES`,
which that test does not walk, so `runDAm` is outside its scope by design and
must not be added to it. `check --json` — and the `Analysis`
API behind it — reports every analysis of the input, each with its own pada and
its own trace; the default `check` output prints only the first, without its
pada. 1.3.72's semantic condition (*kartrabhiprāye kriyāphale*, the fruit of
the action accruing to the agent) is **not**
modelled: both arms derive, and the reader selects by sense. `INVALID` means
"not derivable within this covered grammar," not "ungrammatical in
Sanskrit." See `docs/ARCHITECTURE.md`.

## Layout

See `docs/ARCHITECTURE.md` for the crate map.
