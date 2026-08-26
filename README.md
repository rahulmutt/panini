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

Finite verbs (*tiṅanta*), six gaṇas fully covered — *bhvādi* (1, vikaraṇa
śap), *divādi* (4, śyan), *tudādi* (6, śa), *adādi* (2, śap luk'd), *kryādi*
(9, śnā) and *svādi* (5, śnu) — plus *rudhādi* (7, śnam) **partial**: twenty-four
of its roots (√kṛt, √hiṃs, √khid, √bhañj, √piṣ, √indh, √rudh, and — curated
in slice 7c — √bhid, √kṣud, √yuj and √tṛd, and — curated in the 8.2.30/8.2.39
generalization slice — √ric and √vic, and — curated in slice 7d, on the
audited numbers alone with no new sūtra — √śiṣ, √und, √añj, √tañc, √vij,
√vṛj, √pṛc and √vid, and — curated in slice 7e, with 7.3.92 *tṛṇaha im*,
8.2.31 *ho ḍhaḥ* and 8.3.13 *ḍho ḍhe lopaḥ* — √tṛh, and — curated in slice
7f, with 6.1.73 *che ca* and 8.4.40 *stoḥ ścunā ścuḥ* — √chid and √chṛd) of
25. √rudh, the
gaṇa's eponym,
arrived with 1.3.72 *svaritañitaḥ* as the engine's first ubhayapadī root, so
the ubhayapada deferral itself is discharged: 1.3.72 is no longer what keeps
any root out. **1 of the 25 remains out** — √bhuj, deferred
behind 1.3.66 *bhujo'navane*: vidyut derives all 72 of its cells, and 1.3.66
is the only rule this engine lacks for it, a root-keyed pada assignment
structurally identical to 1.3.72's, which this engine already implements —
what keeps √bhuj out is the **sense** restriction *anavane* imposes, which
neither engine models,
not the cost of implementing 1.3.66. *parasmaipada* and *ātmanepada*
(which padas a root admits is a curated verdict on its table row), over a
curated 66-root set, in four lakāras: *laṭ* (present), *laṅ* (imperfect), *loṭ*
(imperative), and *vidhiliṅ* (optative). A cell may have more than one valid
form where an optional (*vikalpa*) sūtra applies — `hinvaH` and `hinuvaH` are
both correct — and in fact 346 of the 2772 cells hold more than one form: 247
hold two, 81 hold three (`Bavatu`, `BavatAd`, `BavatAt`), two hold four
(rudhādi's √piṣ loṭ madhyama eka, and — new in slice 7d — √śiṣ's), eight hold
five, and eight hold six — the loṭ
parasmaipada madhyama eka of rudhādi's √kṛt, √rudh, √bhid, √kṣud, √tṛd, √und
and — new in slice 7f — √chid and √chṛd, tied as the sharpest forks in the
suite, each holding
six valid readings of the
one cell: `kfndDi` / `kfnDi` / `kfnttAd` / `kfntAd` / `kfnttAt` / `kfntAt` for
√kṛt, `rundDi` / `runDi` / `rundDAd` / `runDAd` / `rundDAt` / `runDAt` for
√rudh, `undDi` / `unDi` / `unttAd` / `untAd` / `unttAt` / `untAt` for
√und, `CindDi` / `CinDi` / `CinttAd` / `CintAd` / `CinttAt` / `CintAt` for
√chid, and `CfndDi` / `CfnDi` / `CfnttAd` / `CfntAd` / `CfnttAt` / `CfntAt`
for √chṛd. Nothing in the suite forks deeper than six. √yuj, ubhayapadī like the
other three roots 7c curated, does *not* fork that deep: 8.2.30 *coḥ kuḥ*
replaces its palatal `j` with the velar `g` (which 8.4.55 *khari ca* later
devoices to `k` before a `t`), and a velar is never savarṇa with the dental
`t`/`D` that follows, so it never reaches the 8.4.65 branch the dental-final
roots take. A root may also admit **both**
padas — all eleven ubhayapadī roots in the curated set (√nī, √tud, √rudh,
√bhid, √kṣud, √yuj, √tṛd, √ric, √vic, √chid and √chṛd) derive a full
parasmaipada and a full
ātmanepada paradigm, so a single surface can be genuinely pada-ambiguous.
Twenty-six surfaces are, each of them a pinned cell in both padas at once:
`BinttAm`, `CfnttAm`, `CinttAm`, `aBintta`, `acCfntta`, `acCintta`,
`akzuntta`, `anayata`, `ariNkta`, `arundDa`, `atfntta`,
`atudata`, `aviNkta`, `ayuNkta`, `kzunttAm`, `nayatAm`, `nayetAm`, `nayeta`,
`riNktAm`, `rundDAm`, `tfnttAm`, `tudatAm`, `tudetAm`, `tudeta`, `viNktAm` and
`yuNktAm` — `rundDAm`, for
instance, is √rudh's loṭ parasmaipada prathama dvi *and* its loṭ ātmanepada
prathama eka. That enumeration is no longer maintained by hand:
`pada_ambiguous_surfaces_are_exactly_these` in
`crates/panini/tests/paradigm.rs` walks `PARADIGM` and asserts exactly this
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
