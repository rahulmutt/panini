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

Finite verbs (*tiṅanta*), eight gaṇas covered, seven of them fully —
*bhvādi* (1, vikaraṇa śap), *divādi* (4, śyan), *tudādi* (6, śa), *adādi*
(2, śap luk'd), *kryādi* (9, śnā), *svādi* (5, śnu) and *rudhādi* (7,
śnam) — plus *tanādi* (8, vikaraṇa the bare *u* of 3.1.79, curated in
slice 8a) **partial** at nine of its ten dhātupāṭha rows: √tan, √san,
√kṣaṇ, √kṣiṇ, √ṛ, √tṛ and √ghṛ (all seven ubhayapadī by 1.3.72) plus √van
and √man (both ātmanepadī by 1.3.12) derive; √kṛ (`08.0010`), the one root
3.1.79 itself names (*tanādikṛñbhya uḥ*), is deferred to slice 8b behind
the 6.4.108–110 kṛ-specials. rudhādi is
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
curated 76-root set, in four lakāras: *laṭ* (present), *laṅ* (imperfect), *loṭ*
(imperative), and *vidhiliṅ* (optative). A cell may have more than one valid
form where an optional (*vikalpa*) sūtra applies — `hinvaH` and `hinuvaH` are
both correct — and in fact 670 of the 3420 cells hold more than one form: 520
hold two, 109 hold three (`Bavatu`, `BavatAd`, `BavatAt`), seventeen hold four
(rudhādi's √piṣ loṭ madhyama eka, and — new in slice 7d — √śiṣ's, and — new
in slice 8a — fifteen more spread across tanādi's four ik-upadhā roots kziR,
fR, tfR and GfR), eight hold
five, and sixteen hold six — the loṭ
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
`arRotu` / `fRutAd` / `fRutAt` / `arRutAd` / `arRutAt`. Nothing in the suite
forks deeper than six — fR's laṅ parasmaipada prathama eka is the one
7.3.86-eligible cell that does *not* reach that depth: its guṇa branch
(`ArR-`) and aguṇa branch converge on the same surface once 8.4.56's cartva
applies, so the cell holds only two forms, the corpus's first
convergent-fork collapse. √yuj, ubhayapadī like the
other three roots 7c curated, does *not* fork that deep: 8.2.30 *coḥ kuḥ*
replaces its palatal `j` with the velar `g` (which 8.4.55 *khari ca* later
devoices to `k` before a `t`), and a velar is never savarṇa with the dental
`t`/`D` that follows, so it never reaches the 8.4.65 branch the dental-final
roots take. A root may also admit **both**
padas — nineteen roots that admit both padas in the curated set (eighteen
ubhayapadī by 1.3.72: √nī, √tud, √rudh, √bhid, √kṣud, √yuj, √tṛd, √ric,
√vic, √chid, √chṛd, √tan, √san, √kṣaṇ, √kṣiṇ, √ṛ, √tṛ and √ghṛ; and √bhuj by
1.3.66) derive a full
parasmaipada and a full
ātmanepada paradigm, so a single surface can be genuinely pada-ambiguous.
√van, by contrast, never enters this bucket: it is ātmanepadī by its own
anudātta marker (1.3.12), and while vidyut-prakriya additionally derives a
parasmaipada `vanoti` via the gaṇasūtra Kaumudī 2547.2, that is recorded
here, not modelled, on 1.3.72's own sense-restriction precedent, so this
engine's √van has no parasmaipada branch to collide against.
Forty-two surfaces are pada-ambiguous, each of them a pinned cell in both
padas at once:
`ArRuta`, `BinttAm`, `BuNktAm`, `CfnttAm`, `CinttAm`, `GfRutAm`, `aBintta`,
`aBuNkta`, `aGfRuta`, `acCfntta`, `acCintta`, `akzaRuta`, `akziRuta`,
`akzuntta`, `anayata`, `ariNkta`, `arundDa`, `asanuta`, `atanuta`,
`atfRuta`, `atfntta`, `atudata`, `aviNkta`, `ayuNkta`, `fRutAm`,
`kzaRutAm`, `kziRutAm`, `kzunttAm`, `nayatAm`, `nayetAm`, `nayeta`,
`riNktAm`, `rundDAm`, `sanutAm`, `tanutAm`, `tfRutAm`, `tfnttAm`,
`tudatAm`, `tudetAm`, `tudeta`, `viNktAm` and
`yuNktAm` — `rundDAm`, for
instance, is √rudh's loṭ parasmaipada prathama dvi *and* its loṭ ātmanepada
prathama eka, and tanādi's seven ubhayapadī roots contribute a new shape:
`atanuta` is both √tan's laṅ ātmanepada prathama eka and its laṅ
parasmaipada madhyama bahu, and `tanutAm` is both its loṭ ātmanepada
prathama eka and its loṭ parasmaipada prathama dvi. That enumeration is no longer maintained by hand:
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
