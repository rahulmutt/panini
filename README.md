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

Finite verbs (*tiṅanta*), ten gaṇas covered, nine of them fully —
*bhvādi* (1, vikaraṇa śap), *divādi* (4, śyan), *tudādi* (6, śa), *adādi*
(2, śap luk'd), *kryādi* (9, śnā), *svādi* (5, śnu) and *rudhādi* (7,
śnam) — plus *tanādi* (8, vikaraṇa the bare *u* of 3.1.79), **complete**
at all ten of its dhātupāṭha rows: √tan, √san,
√kṣaṇ, √kṣiṇ, √ṛṇ, √tṛ and √ghṛ (all seven ubhayapadī by 1.3.72) plus √van
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
slot), **complete** at all 26 of its dhātupāṭha rows, √hu (`03.0001`,
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
arms, and 8.2.40 given its *adhaḥ*. Slice 3c2 added √hā parasmaipada
(`03.0009`, *jahāti*) and √gā (`03.0026`, *jigāti*, by the chāndasa 7.4.78 on
the Kaumudī's authority), with 6.4.118 *lopo yi* and two more vikalpas, 6.4.116
*jahāteś ca* and 6.4.117 *ā ca hau*. The latter changes no text and instead bars
the rules that would, the first rule to declare an apavāda relation
(`Rule.bars`). Slice 3d added the six consonant-initial ṛ-roots — √pṝ
(`03.0004`) and √pṛ (`03.0005`), both *piparti*, √bhṛ (`03.0006`, *bibharti*,
ubhayapadī), √ghṛ (`03.0015`, *jagharti*), √hṛ (`03.0016`, *jaharti*) and √sṛ
(`03.0018`, *sasarti*) — behind 7.4.66 *ur at*, 7.4.77 *arti-pipartyoś ca*
and 7.1.102 *ud oṣṭhyapūrvasya*, with 7.4.60 widened to every non-initial
consonant, 7.4.76 given its √bhṛñ row, 6.1.77 given an aṅga arm, 8.2.77
reading the ending when ślu empties the śap, and 8.3.59 *ādeśapratyayayoḥ*
given an `r` arm (*bibharṣi*). Slice 3d2 added √ṛ (`03.0017`, *iyarti*), the
gaṇa's one vowel-initial ṛ-root, behind 6.4.78 *abhyāsasyāsavarṇe*, with
7.4.60 trimming a vowel-initial abhyāsa (`ar` → `a`) and 7.4.77 given its √ṛ
row; in laṅ the āṭ merges into the abhyāsa (*aiyaḥ*, `EyaH`).
Slice 3e added √ṇij (`03.0012`, *nenekti*), √vij (`03.0013`, *vevekti*) and
√viṣ (`03.0014`, *veveṣṭi*), all ubhayapadī, behind 7.4.75 *nijāṁ trayāṇāṁ
guṇaḥ ślau* (the abhyāsa's guṇa) and 7.3.87 *nābhyastasyāci piti
sārvadhātuke*, which bars 7.3.86's guṇa before a vowel-initial pit ending
(*nenijāni*); √ṇij's `ṇ` is stored as `n`, the stored-form convention covering
6.1.65.
Slice 3f added √kit (`03.0021`, *ciketti*), √tur (`03.0022`, *tutorti*), √dhiṣ
(`03.0023`, *didheṣṭi*) and √dhan (`03.0024`, *dadhanti*), all parasmaipadī, with
no new rule: 8.2.75 *daś ca* dropped its rudhādi gaṇa test (*acikeḥ*), and 8.3.24
*naś cāpadāntasya jhali* now admits juhotyādi beside rudhādi (*dadhaṁsi*,
*dadhaṁhi*).
Slice 3f2 added √bhas (`03.0019`, *babhasti*), parasmaipadī, behind two new
sūtras — 6.4.100 *ghasibhasor hali ca*, which elides its upadhā `a` before
every kṅit (*bapsati*), and 8.2.26 *jhalo jhali* (*babdhaḥ*) — with 8.2.73
*tipy anasteḥ* and 8.2.74 *sipi dhāto rur vā* dropping their rudhādi gaṇa test
(*ababhat*, *ababhaḥ*) and 8.4.55 *khari ca* reading the whole word.
Slice 3f3 added √jan (`03.0025`, *jajanti*), parasmaipadī, the gaṇa's last
row, behind three new sūtras — 6.4.98 *gamahanajanakhanaghasāṁ lopaḥ*
(*jajñati*), 6.4.42 *janasanakhanāṁ sañjhaloḥ* (*jajātaḥ*) and the engine's
twelfth vikalpa, 6.4.43 *ye vibhāṣā* (*jajanyāt* ~ *jajāyāt*) — with 8.4.40
*stoḥ ścunā ścuḥ* gaining its converse arm, a stu after a ścu, guarded by the
8.4.44 *śāt* exemption. *curādi* (10) is **open** at 4 of its 509
dhātupāṭha rows: √cur (`10.0001`, *corayati*), √laḍ (`10.0010`,
*lāḍayati*), √bhakṣ (`10.0033`) and √bhūṣ (`10.0255`), curated in slice 10a,
all ubhayapadī by 1.3.74 *ṇicaś ca*. Every curādi root takes ṇic (3.1.25)
before the vikaraṇa; a new first pipeline stage adds it, guṇates or
lengthens the root before it (7.3.86, 7.2.116 *ata upadhāyāḥ*), and folds it
into the dhātu by 3.1.32 *sanādyantā dhātavaḥ*, so √cur's stem is `cori` and
derives on through 7.3.84 and 6.1.78 like √nī's. rudhādi is
complete at all twenty-five of its
own roots (√kṛt, √hiṃs, √khid, √bhañj, √piṣ, √indh, √rudh,
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
curated 107-root set, in four lakāras: *laṭ* (present), *laṅ* (imperfect), *loṭ*
(imperative), and *vidhiliṅ* (optative). A cell may have more than one valid
form where an optional (*vikalpa*) sūtra applies — `hinvaH` and `hinuvaH` are
both correct — and in fact 824 of the 4932 cells hold more than one form: 612
hold two, 165 hold three (`Bavatu`, `BavatAd`, `BavatAt`, and — new in
slice 3b — √hrī's loṭ prathama and madhyama eka, and — new in slice 3c —
√dā's and √dhā's, and — new in slice 3c2 — √gā's, and — new in slice 3d —
the six ṛ-roots', and — new in slice 3d2 — √ṛ's, and — new in slice 3e —
√ṇij's, √vij's and √viṣ's, and — new in slice 3f — √kit's, √tur's, √dhiṣ's
and √dhan's, and — new in slice 3f2 — √bhas's, and — new in slice 3f3 —
√jan's, and — new in slice 10a — the four curādi roots', each by
7.1.35/8.4.56, √bhas's laṅ madhyama eka by 8.2.74/8.4.56),
nineteen hold four
(rudhādi's √piṣ loṭ madhyama eka, and — new in slice 7d — √śiṣ's, and — new
in slice 8a — fifteen more spread across tanādi's four ik-upadhā roots kziR,
fR, tfR and GfR, and — new in slice 3b — √bhī's vidhiliṅ prathama eka,
forking on 6.4.115 alongside 8.4.56, and — new in slice 3f3 — √jan's
vidhiliṅ prathama eka, forking on 6.4.43 alongside 8.4.56), ten hold
five (and, new in slice 3b, √bhī's loṭ prathama eka, forking on
7.1.35/6.4.115/8.4.56, and — new in slice 3c2 — √hā's, forking on
7.1.35/6.4.116/8.4.56), and seventeen hold six — the loṭ
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
tanādi's 7.3.86 route. One cell — new in slice 3c2 — holds seven: √hā's (`03.0009`) loṭ
parasmaipada madhyama eka, `jahIhi` / `jahihi` / `jahAhi` / `jahItAd` /
`jahItAt` / `jahitAd` / `jahitAt`, where 6.4.117 *ā ca hau* keeps the `A` by
barring the rules that would change it. Nothing forks deeper than seven. fR's own laṅ cells — all eighteen of them, both
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
padas — thirty roots that admit both padas in the curated set
(twenty-five ubhayapadī by 1.3.72: √nī, √tud, √rudh, √bhid, √kṣud, √yuj,
√tṛd, √ric, √vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛṇ, √tṛ, √ghṛ,
√kṛ, √dā, √dhā, √bhṛ, √ṇij, √vij and √viṣ; √bhuj by 1.3.66; and curādi's √cur,
√laḍ, √bhakṣ and √bhūṣ by 1.3.74) derive a full
parasmaipada and a full ātmanepada paradigm, so a single surface can be
genuinely pada-ambiguous.
√van, by contrast, never enters this bucket: it is ātmanepadī by its own
anudātta marker (1.3.12), and while vidyut-prakriya additionally derives a
parasmaipada `vanoti` via the gaṇasūtra Kaumudī 2547.2, that is recorded
here, not modelled, on 1.3.72's own sense-restriction precedent, so this
engine's √van has no parasmaipada branch to collide against.
Seventy-two surfaces are pada-ambiguous, each of them a pinned cell in both
padas at once: `ArRuta`, `BUzayatAm`, `BUzayetAm`, `BUzayeta`, `BakzayatAm`,
`BakzayetAm`, `Bakzayeta`, `BinttAm`, `BuNktAm`, `CfnttAm`, `CinttAm`, `DattAm`,
`GfRutAm`, `aBUzayata`, `aBakzayata`, `aBintta`, `aBuNkta`, `aDatta`, `aGfRuta`,
`abiBfta`, `acCfntta`, `acCintta`, `acorayata`, `adatta`, `akuruta`, `akzaRuta`,
`akziRuta`, `akzuntta`, `alAqayata`, `anayata`, `anenikta`, `ariNkta`,
`arundDa`, `asanuta`, `atanuta`, `atfRuta`, `atfntta`, `atudata`, `avevikta`,
`avevizwa`, `aviNkta`, `ayuNkta`, `biBftAm`, `corayatAm`, `corayetAm`,
`corayeta`, `dattAm`, `fRutAm`, `kurutAm`, `kzaRutAm`, `kziRutAm`, `kzunttAm`,
`lAqayatAm`, `lAqayetAm`, `lAqayeta`, `nayatAm`, `nayetAm`, `nayeta`,
`neniktAm`, `riNktAm`, `rundDAm`, `sanutAm`, `tanutAm`, `tfRutAm`, `tfnttAm`,
`tudatAm`, `tudetAm`, `tudeta`, `veviktAm`, `vevizwAm`, `viNktAm` and `yuNktAm` — `rundDAm`, for instance, is √rudh's loṭ
parasmaipada prathama dvi *and* its loṭ ātmanepada prathama eka, and tanādi's seven ubhayapadī roots
contribute a new shape: `atanuta` is both √tan's laṅ ātmanepada prathama eka and its laṅ
parasmaipada madhyama bahu, and `tanutAm` is both its loṭ ātmanepada prathama
eka and its loṭ parasmaipada prathama dvi. √kṛ (slice 8b) contributes the same
shape: `akuruta` (laṅ ātmanepada prathama eka / parasmaipada madhyama bahu) and
`kurutAm` (loṭ ātmanepada prathama eka / parasmaipada prathama dvi), and √bhṛ
(slice 3d) the same again: `abiBfta` and `biBftAm`. Slice 3e's three roots do the same again:
`anenikta`/`neniktAm`, `avevikta`/`veviktAm` and `avevizwa`/`vevizwAm`. Slice 10a's four curādi roots, thematic like √nī, contribute √nī's
four-surface shape each: `acorayata`/`corayatAm`/`corayetAm`/`corayeta` and
the same for `lAqaya-`, `Bakzaya-` and `BUzaya-`. That enumeration is no
longer maintained by hand:
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
