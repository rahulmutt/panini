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
(9, śnā) and *svādi* (5, śnu) — plus *rudhādi* (7, śnam) **partial**: eleven
of its roots (√kṛt, √hiṃs, √khid, √bhañj, √piṣ, √indh, √rudh, and — curated
in slice 7c — √bhid, √kṣud, √yuj and √tṛd) of 25. √rudh, the gaṇa's eponym,
arrived with 1.3.72 *svaritañitaḥ* as the engine's first ubhayapadī root, so
the ubhayapada deferral itself is discharged: 1.3.72 is no longer what keeps
any root out. **14 of the 25 remain out**, each for a narrow reason of its
own — √ric and √vic for an 8.2.30 *coḥ kuḥ* hardcoded to a single `j` → `g`
pair where the sūtra names the whole cu-varga — its **substitute** needs
generalising as much as its match does, so this is more than widening a guard
— and √chid and √chṛd for two sūtras the engine does not implement (6.1.73
*che ca*, 8.4.40 *stoḥ ścunā ścuḥ*).
Nine reachable non-ubhayapadī roots are likewise not curated yet, and the last,
√bhuj, is deferred behind 1.3.66 *bhujo'navane*, which forks its pada on sense
rather than on an axis this engine models. *parasmaipada* and *ātmanepada*
(which padas a root admits is a curated verdict on its table row), over a
curated 53-root set, in four lakāras: *laṭ* (present), *laṅ* (imperfect), *loṭ*
(imperative), and *vidhiliṅ* (optative). A cell may have more than one valid
form where an optional (*vikalpa*) sūtra applies — `hinvaH` and `hinuvaH` are
both correct — and in fact 238 of the 2160 cells hold more than one form: 166
hold two, 61 hold three (`Bavatu`, `BavatAd`, `BavatAt`), one holds four
(rudhādi's √piṣ loṭ madhyama eka), five hold five, and five hold six — the loṭ
parasmaipada madhyama eka of rudhādi's √kṛt, √rudh, √bhid, √kṣud and √tṛd,
tied as the sharpest forks in the suite, each holding six valid readings of the
one cell: `kfndDi` / `kfnDi` / `kfnttAd` / `kfntAd` / `kfnttAt` / `kfntAt` for
√kṛt, and `rundDi` / `runDi` / `rundDAd` / `runDAd` / `rundDAt` / `runDAt` for
√rudh. Nothing in the suite forks deeper than six. √yuj, ubhayapadī like the
other three roots 7c curated, does *not* fork that deep: 8.2.30 *coḥ kuḥ*
turns its final `j` to `k`, so it never reaches the 8.4.65 branch the
dental-final roots take. A root may also admit **both**
padas — all seven ubhayapadī roots in the curated set (√nī, √tud, √rudh,
√bhid, √kṣud, √yuj and √tṛd) derive a full parasmaipada and a full ātmanepada
paradigm, so a single surface can be genuinely pada-ambiguous. Eighteen
surfaces are, each of them a pinned cell in both padas at once: `BinttAm`,
`aBintta`, `akzuntta`, `anayata`, `arundDa`, `atfntta`,
`atudata`, `ayuNkta`, `kzunttAm`, `nayatAm`, `nayetAm`, `nayeta`, `rundDAm`,
`tfnttAm`, `tudatAm`, `tudetAm`, `tudeta` and `yuNktAm` — `rundDAm`, for
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
