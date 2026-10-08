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

Finite verbs (*tiṅanta*), ten gaṇas covered, eight of them fully —
*bhvādi* (1, vikaraṇa śap), *divādi* (4, śyan), *tudādi* (6, śa), *adādi*
(2, śap luk'd), *svādi* (5, śnu), **complete** at all
38 of its dhātupāṭha rows since svādi 5b curated the thirty-two beyond the six
curated in the first svādi slice (√hi, √ri, √āp, √śak, √aś, √ṣṭigh), behind 6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti* (√dambh's
*dabhnoti*, its *aniditām* read from the curated `IDIT` rows) and 8.4.39
*kṣubhnādiṣu ca* (√tṛp's *tṛpnoti*, kept free of ṇatva), and *rudhādi* (7,
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
8.4.44 *śāt* exemption. *kryādi* (9, śnā) is **open** at 28 of its 71
dhātus: √kliś, √gudh and √aś (slice 9a); √muṣ, √vrī and √vṛṅ (slice 9b, behind
8.4.1 / 8.4.2, the engine's first ṇatva); and twenty-two more that slice 9c
curated on the rules already in the pipeline — eight of its ñit rows √krī
(*krīṇāti*, *krīṇīte*), √prī, √śrī, √mī, √ṣi (stored `si` by 6.1.64), √yu,
√knū and √drū, the gaṇa's first ubhayapadī roots, by 1.3.72, and fourteen
parasmaipadī by 1.3.78: √bhrī, √kṣīṣ, √mṛd, √kuṣ, √ṇabh (stored `naB`),
√tubh, √dhras, √iṣ, √viṣ, √pruṣ, √pluṣ, √puṣ, √khac (*khacñāti*, the second
root of 8.4.40's converse arm) and √svṝ (*svūrṇāti*, by 7.1.102 and 8.2.77).
*curādi* (10) is **open** at 491 of its 492
dhātus (upstream numbers 509 rows, but the last seventeen, `10.0493`–`10.0509`,
are gaṇasūtras, not dhātus; the one dhātu out is `10.0368 za\da~`, which
waits for upasargas): √cur (`10.0001`, *corayati*), √laḍ (`10.0010`,
*lāḍayati*), √bhakṣ (`10.0033`) and √bhūṣ (`10.0255`), curated in slice 10a,
all ubhayapadī by 1.3.74 *ṇicaś ca*; and four roots of the ākusmīya
antargaṇa, √cit (`10.0192`, *cetayate*), √vṛṣ (`10.0228`, *varṣayate*), √mad
(`10.0229`, *mādayate*) and √kusm (`10.0236`), curated in slice 10b,
ātmanepadī by the dhātupāṭha's own gaṇasūtra 10.0496 *ā kusmād
ātmanepadinaḥ* — the engine's first rule that is not an Aṣṭādhyāyī sūtra.
Slice 10c curated thirty-three more ākusmīya rows in bulk (`10.0195` through
`10.0234`), among them √vid (*vedayate*), √śam (*śāmayate*) and the pair √mān
/ √man, which share every form (*mānayate*). Slice 10d curated six of the
seven jñapādi (`10.0118` through `10.0123`: √jñap, √yam, √cah, √cap, √rah,
√bal), ubhayapadī by 1.3.74 and mit by the gaṇasūtra 10.0493 *jñapādayo
mitaḥ*, the engine's second non-Aṣṭādhyāyī rule: 6.4.92 *mitāṃ hrasvaḥ*
shortens back the upadhā 7.2.116 lengthened before ṇic, so *jñapayati*, not
*jñāpayati*. Slice 10e curated ninety-two adanta roots (`10.0108 mArga` and
`10.0389 kaTa` … `10.0492 Deka`), whose upadeśa ends in a bare `a`: 6.4.48
*ato lopaḥ ārdhadhātuke* deletes it before ṇic, and by 1.1.57 *acaḥ parasmin
pūrvavidhau* the deleted `a` still stands for 7.2.116 and 7.3.86, so
*kathayati*, not *kāthayati*, and *kuhayate*, not *kohayate*. Eighty-three are
ubhayapadī by 1.3.74; nine are ātmanepadī by the gaṇasūtra 10.0497 *ā garvād
ātmanepadinaḥ*, the engine's third non-Aṣṭādhyāyī rule. Seven of them
(√saṅketa, *saṅketayati*) carry their own `n` before a jhal (six change form,
the anusvāra rows; 8.4.58 restores √andha's `n`), and 8.3.24
*naś cāpadāntasya jhali*, until then rudhādi's and juhotyādi's, now reaches a
curādi root's own `n` too. Slice 10f curated the ten rows whose ṇic is
optional (Kaumudī 2564, 2570, 2573.1 and 2573.3, the engine's first
Kaumudī vikalpas): six ākusmīya roots (√daṃś, *daṃśati* beside
*daṃśayate*), `garva`, `mUtra`, `katra` and `pata`. Each derives a ṇic
branch and a ṇic-less one. The ṇic-less branch is parasmaipada by 1.3.78
and runs the bhvādi path, where an adanta root's own `a` merges with śap's
by 6.1.97 or, after 7.3.101, 6.1.101 (*mūtrati*, *patāmi*); `pata` has a
third reading, 2573.2's *pātayati*. Slice 10g curated fifty-nine more:
every idit (2564) and ñit/udit (2570) curādi row outside the āsvadīya and
ādhṛṣīya but √ci, all ubhayapadī by 1.3.74 with ṇic (√cint, *cintayati*
beside *cintati*). One of them, √kṣamp, made 8.4.2 *aṭkupvāṅnumvyavāye 'pi*
read its *num*: the root's stored `n` reaches ṇatva as the anusvāra 8.3.24
made of it, and the anusvāra now intervenes (*kṣampāṇi*). Slice 10h curated
fifty of the fifty-one ādhṛṣīya rows, whose ṇic the dhātupāṭha gaṇasūtra
10.0498 *ā dhṛṣād vā* makes optional (√yuj, *yojayati* beside *yojati*);
`10.0368 za\da~` waits for upasargas. The six svarita or ñit among them are
ubhayapadī without ṇic too, by 1.3.72 (√vṛ, *varate*). Before ṇic, 7.2.115
*aco ñṇiti* takes vṛddhi of a final vowel (*lāyayati*, *bhāvayati*), the vārttika
7.3.37.2 gives √dhū and √prī an optional nuk instead (*dhūnayati*,
*prīṇayati*), and 7.2.114 *mṛjer vṛddhiḥ* gives √mṛj vṛddhi on both branches
(*mārjati*, *mārjayati*). Slice 10i curated the fifty-nine āsvadīya rows,
whose ṇic the gaṇasūtra 10.0499 *ā svadaḥ sakarmakāt* makes optional (√gras,
*grāsayati* beside *grasati*), and the last two one-row optional-ṇic
triggers, Kaumudī 2565's √pṝ (*parati*) and 2571's √ghuṣ (*ghoṣati*). Where
√dhūp and √vich take no ṇic, 3.1.28 *gupūdhūpavicchipaṇipanibhya āyaḥ*
gives them āya (*dhūpāyati*, *vicchāyati*), and √vich's tuk (6.1.73 *che
ca*) comes before guṇa on every branch, so *vicchayati*, not *vechayati*.
Slice 10j curated the eight ajanta rows: √ghṛ, √jñā, √cyu and √bhū
(*bhāvayati*), ubhayapadī by 1.3.74; √gṛ and √yu, the last two ākusmīya
rows; √smiṅ, ṅit, which Kaumudī 2567 keeps ātmanepadī under ṇic
(*smāyayate*); and √ci, the last jñapādi root, whose ṇic is optional (2570)
and which forks three ways: 6.1.54 *cisphuror ṇau* optionally gives `cA`,
7.3.36 *artihrīvlīrīknūyīkṣmāyyātāṃ puk ṇau* adds puk to that and to √jñā
(*jñāpayati*), and 6.4.92 shortens both mit branches (*capayati*,
*cayayati*, beside *cayati*). Slice 10k curated the 155 plain
obligatory-ṇic rows that need nothing new, all ubhayapadī by 1.3.74: 7.3.86
or 7.2.116 before ṇic or neither (*codayati*, *jālayati*, *pīḍayati*), and
existing rules on new rows: 8.3.24 on a root's own `n` (*puṃsayati*),
8.3.24 with 8.4.58 (*sambayati*), and the sanādi 6.1.73's tuk on √pich
(*picchayati*). Two of its rows, `10.0051 zAntva~` and `10.0052 sAntva~`,
are the same root once 6.1.64 makes the `z` an `s`; both are curated.
Slice 10l curated eight rows that bring rules of their own, all ubhayapadī
by 1.3.74: 8.2.78 *upadhāyāṃ ca* lengthens an ik before an `r` upadhā and a
final hal (*ūrjayati*, *cūrṇayati*, *gūrdayati*); 7.1.101 *upadhāyāś ca*
makes √kṝt's `F` upadhā `ir` before ṇic, ahead of guṇa (*kīrtayati*);
8.2.18 *kṛpo ro laḥ* gives √kṛp its `l` (*kalpayati*); 6.1.75 *dīrghāt*
adds tuk after √mlecch's long `e` (*mlecchayati*); and 8.4.41 *ṣṭunā ṣṭuḥ*
now also retroflexes a stu before a ṭu (*aṭṭayati*). √dhras (`10.0270
u~Drasa~`) is udit by its initial `u~`, so its ṇic is optional
(*dhrāsayati* beside *dhrasati*). Slice 10m curated √picc (`10.0175
picca~`, *piccayati*), ubhayapadī by 1.3.74, once 8.2.30 *coḥ kuḥ* was
narrowed, as vidyut-prakriya reads it, to a term-final cu before a
jhal-initial affix or āgama, or pada-final: the whole-word scan before it
velarised the root-internal `cc` (*pikcayati*).
Every curādi root takes ṇic (3.1.25) before the vikaraṇa; a new first
pipeline stage adds it, guṇates or
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
curated 648-root set, in four lakāras: *laṭ* (present), *laṅ* (imperfect), *loṭ*
(imperative), and *vidhiliṅ* (optative). A cell may have more than one valid
form where an optional (*vikalpa*) sūtra applies — `hinvaH` and `hinuvaH` are
both correct — and in fact 8796 of the 40824 cells hold more than one form: 7014
hold two, 1035 hold three (`Bavatu`, `BavatAd`, `BavatAt`, and — new in
slice 3b — √hrī's loṭ prathama and madhyama eka, and — new in slice 3c —
√dā's and √dhā's, and — new in slice 3c2 — √gā's, and — new in slice 3d —
the six ṛ-roots', and — new in slice 3d2 — √ṛ's, and — new in slice 3e —
√ṇij's, √vij's and √viṣ's, and — new in slice 3f — √kit's, √tur's, √dhiṣ's
and √dhan's, and — new in slice 3f2 — √bhas's, and — new in slice 3f3 —
√jan's, and — new in slice 10a — the four curādi roots', and — new in slice
10d — the six jñapādi roots', and — new in slice 10e — the eighty-three
ubhayapadī adanta roots', and — new in slice 10k — the 155 plain
obligatory-ṇic rows', and — new in slice 10l — seven of its eight rows', and —
new in slice 10m — √picc's, and — new in svādi 5b — its thirty-two rows', and — new in slice 9c
— the twenty-two kryādi rows', each by
7.1.35/8.4.56, √bhas's laṅ madhyama eka by 8.2.74/8.4.56; slice 10f's
optional-ṇic rows add 114 two-form and 46 three-form cells, and slice 10g's
fifty-nine add 1888 two-form cells, a ṇic and a ṇic-less reading each, and
slice 10h's fifty add 1680 two-form and 136 three-form cells, √dhū's and
√prī's ṇic branch forking again on 7.3.37.2's nuk, and slice 10i's
sixty-one add 1952 two-form cells the same way, and slice 10j's √ghṛ, √jñā,
√cyu and √bhū add eight two-form and eight three-form cells as √cur's do,
and √ci 68 three-form cells, its three readings where neither 7.1.35 nor
8.4.56 forks, and slice 10l's √dhras 32 two-form cells, a ṇic and a
ṇic-less reading each, and svādi 5b's thirty-two rows 180 two-form cells,
64 on 8.4.56 and 116 on 6.4.107),
361 hold four
(rudhādi's √piṣ loṭ madhyama eka, and — new in slice 7d — √śiṣ's, and — new
in slice 8a — fifteen more spread across tanādi's four ik-upadhā roots kziR,
fR, tfR and GfR, and — new in slice 3b — √bhī's vidhiliṅ prathama eka,
forking on 6.4.115 alongside 8.4.56, and — new in slice 3f3 — √jan's
vidhiliṅ prathama eka, forking on 6.4.43 alongside 8.4.56, and — new in
slice 10f — `mUtra`'s and `katra`'s laṅ and vidhiliṅ parasmaipada prathama
eka, two readings × 8.4.56, and — new in slice 10g — the fifty-nine
optional-ṇic rows' laṅ and vidhiliṅ parasmaipada prathama eka, the same way,
and — new in slice 10h — forty-eight of the fifty ādhṛṣīya rows', the same
way, and — new in slice 10i — the sixty-one rows', the same way, and — new
in slice 10l — √dhras's, the same way),
ten hold
five (and, new in slice 3b, √bhī's loṭ prathama eka, forking on
7.1.35/6.4.115/8.4.56, and — new in slice 3c2 — √hā's, forking on
7.1.35/6.4.116/8.4.56), and 367 hold six — the loṭ
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
tanādi's 7.3.86 route; and, new in slice 10f, six more: `pata`'s laṅ and
vidhiliṅ parasmaipada prathama eka (three readings × 8.4.56) and `mUtra`'s and
`katra`'s loṭ parasmaipada prathama and madhyama eka (two readings × the
tātaṅ triple); and, new in slice 10g, 118 more: the fifty-nine optional-ṇic
rows' loṭ parasmaipada prathama and madhyama eka, the same way; and, new in
slice 10h, 100 more: forty-eight ādhṛṣīya rows' loṭ parasmaipada prathama
and madhyama eka, the same way, and √dhū's and √prī's laṅ and vidhiliṅ
parasmaipada prathama eka, three readings × 8.4.56; and, new in slice 10i,
122 more: the sixty-one rows' loṭ parasmaipada prathama and madhyama eka,
the same way; and, new in slice 10j, √ci's laṅ and vidhiliṅ parasmaipada
prathama eka, three readings × 8.4.56; and, new in slice 10l, √dhras's loṭ
parasmaipada prathama and madhyama eka, two readings × the tātaṅ triple. One cell — new in slice 3c2 — holds seven: √hā's (`03.0009`) loṭ
parasmaipada madhyama eka, `jahIhi` / `jahihi` / `jahAhi` / `jahItAd` /
`jahItAt` / `jahitAd` / `jahitAt`, where 6.4.117 *ā ca hau* keeps the `A` by
barring the rules that would change it. Eight cells hold **nine**, the record:
`pata`'s loṭ parasmaipada prathama and madhyama eka (new in slice 10f),
three readings × the tātaṅ triple (`patayatu` / `patayatAd` / `patayatAt` /
`pAtayatu` / `pAtayatAd` / `pAtayatAt` / `patatu` / `patatAd` / `patatAt`),
and — new in slice 10h — √dhū's and √prī's, the same way (`DAvayatu` /
`DAvayatAd` / `DAvayatAt` / `DUnayatu` / `DUnayatAd` / `DUnayatAt` /
`Davatu` / `DavatAd` / `DavatAt` for √dhū), and — new in slice 10j — √ci's
(`cayayatu` / `capayatu` / `cayatu` and their tātaṅ pairs).
No cell holds eight. fR's own laṅ cells — all eighteen of them, both
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
padas — 486 roots that admit both padas in the curated set
(forty-three ubhayapadī by 1.3.72: √nī, √tud, √rudh, √bhid, √kṣud, √yuj,
√tṛd, √ric, √vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛṇ, √tṛ, √ghṛ,
√kṛ, √dā, √dhā, √bhṛ, √ṇij, √vij and √viṣ, and svādi 5b's √su, √ṣi, √śi, √mi, √ci, √stṛ, √kṛ, √vṛ, √dhu and √dhū, and kryādi's √krī, √prī, √śrī,
√mī, √ṣi, √yu, √knū and √drū; √bhuj by 1.3.66; and curādi's √cur,
√laḍ, √bhakṣ, √bhūṣ, √jñap, √yam, √cah, √cap, √rah, √bal, the
eighty-three ubhayapadī adanta roots, slice 10f's `mUtra`, `katra` and
`pata`, slice 10g's fifty-nine optional-ṇic rows, slice 10h's fifty
ādhṛṣīya rows (the six svarita or ñit among them by 1.3.72 too, without
ṇic), slice 10i's sixty-one rows, slice 10j's √ghṛ, √jñā, √cyu, √bhū and
√ci (by 1.3.72 too, without ṇic), slice 10k's 155 rows, slice 10l's eight and slice 10m's √picc by 1.3.74; and slice 10f's six optional-ṇic ākusmīya roots and
`garva`, ātmanepadī by 10.0496 / 10.0497 with ṇic and parasmaipadī by
1.3.78 without) derive a full
parasmaipada and a full ātmanepada paradigm, so a single surface can be
genuinely pada-ambiguous.
√van, by contrast, never enters this bucket: it is ātmanepadī by its own
anudātta marker (1.3.12), and while vidyut-prakriya additionally derives a
parasmaipada `vanoti` via the gaṇasūtra Kaumudī 2547.2, that is recorded
here, not modelled, on 1.3.72's own sense-restriction precedent, so this
engine's √van has no parasmaipada branch to collide against.
1687 of the pinned (`PARADIGM`) surfaces are pada-ambiguous, each of them a
pinned cell in both padas at once — `rundDAm`, for instance, is √rudh's loṭ
parasmaipada prathama dvi *and* its loṭ ātmanepada prathama eka, and tanādi's seven ubhayapadī roots
contribute a new shape: `atanuta` is both √tan's laṅ ātmanepada prathama eka and its laṅ
parasmaipada madhyama bahu, and `tanutAm` is both its loṭ ātmanepada prathama
eka and its loṭ parasmaipada prathama dvi. √kṛ (slice 8b) contributes the same
shape: `akuruta` (laṅ ātmanepada prathama eka / parasmaipada madhyama bahu) and
`kurutAm` (loṭ ātmanepada prathama eka / parasmaipada prathama dvi), and √bhṛ
(slice 3d) the same again: `abiBfta` and `biBftAm`. Slice 3e's three roots do the same again:
`anenikta`/`neniktAm`, `avevikta`/`veviktAm` and `avevizwa`/`vevizwAm`. Slice 10a's four curādi roots, thematic like √nī, contribute √nī's
four-surface shape each: `acorayata`/`corayatAm`/`corayetAm`/`corayeta` and
the same for `lAqaya-`, `Bakzaya-` and `BUzaya-`, and slice 10d's six
jñapādi roots the same for `jYapaya-`, `yamaya-`, `cahaya-`, `capaya-`,
`rahaya-` and `balaya-`, and slice 10e's eighty-three ubhayapadī adanta roots
the same for `kaTaya-` and the rest, except `10.0396 raha` and `10.0405 caha`,
whose surfaces 10d's √rah and √cah already supply, and slice 10f's `mUtra`,
`katra` and `pata` the same again on their ṇic branch (`mUtraya-`,
`katraya-`, `pataya-`); the six optional-ṇic ākusmīya roots and `garva`
derive their two padas on different branches and add none; slice 10g's
fifty-nine rows add 223 more on their ṇic branch, four each but where a
homograph pair shares them (`lanj`, `vanw` and `jas`, and the laṅ
`OlaRqayata` of `olanq` and `ulanq`); slice 10h's fifty add 184 more, four
each but where a homograph shares them (`tfp`, `dfB` and `granT`, and
`10.0384 mArga~`'s, which `10.0108 mArga` already supplies); slice 10i's
sixty-one add 208 more, four each but where another row already supplies
them (`pF`'s are `pAra`'s; `tunj`, `pinj`, `lunj`, `lanj`, `lanq` and `SIk`
repeat earlier curādi codes) or an in-slice pair shares them (`laGi~`
twice, `svad` and `svAd`); slice 10j's √ghṛ, √jñā, √cyu and √ci add 16 more,
four each, √bhū's being the ādhṛṣīya `10.0382 BU`'s already; slice 10k's 155
rows add 540 more, four each but where an ubhayapadī row curated earlier
already supplies them (sixteen homographs) or an in-slice pair shares them
(`pfT` and `parT`, `pul` twice, `pAl` and `pal`, `sAntv` twice); slice
10l's eight add 28 more, four each but `10.0026 curRa~`'s, which its
homograph `10.0143 cUrRa~` already supplies; slice 10m's √picc adds four
more; and svādi 5b's ten ñit rows add twenty, two each in the tanādi shape
(`asunuta`, `sunutAm`); slice 9c's eight ñit kryādi roots add 32 more in a
shape of their own, four each: `akrIRIta` (laṅ ātmanepada prathama eka and
parasmaipada madhyama bahu), `krIRItAm` (loṭ ātmanepada prathama eka and
parasmaipada prathama dvi), `krIRIta` (vidhiliṅ ātmanepada prathama eka and
loṭ parasmaipada madhyama bahu) and `krIRIyAtAm` (vidhiliṅ ātmanepada and
parasmaipada prathama dvi). The
enumeration is not
maintained by hand: `pada_ambiguous_surfaces_are_exactly_these` in
`crates/panini/tests/paradigm/main.rs` walks `PARADIGM` and asserts the whole
set, all 1687. It is therefore a list of ambiguous **pinned cells**, not of every
pada-ambiguous surface: since slice 10h's ṇic-less branches are live in both
padas, some surfaces that only `ALTERNATES` pins (`avadata`) are ambiguous too. An *alternate* form
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
