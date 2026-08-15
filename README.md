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
(9, śnā) and *svādi* (5, śnu) — plus *rudhādi* (7, śnam) **partial**: seven of
its roots (√kṛt, √hiṃs, √khid, √bhañj, √piṣ, √indh, √rudh) of 25. √rudh, the
gaṇa's eponym, arrived with 1.3.72 *svaritañitaḥ* as the engine's first
ubhayapadī root; the eight other ubhayapadī roots of the gaṇa are now deferred
by **curation** rather than by missing machinery — the engine derives them;
they are simply not curated — nine reachable non-ubhayapadī ones are
likewise not curated yet, and the last, √bhuj, is deferred behind 1.3.66
*bhujo'navane*, which forks its pada on sense rather than on an axis this
engine models.
*parasmaipada* and *ātmanepada* (which padas a root admits is a curated verdict
on its table row), over a curated 49-root set, in four lakāras: *laṭ* (present),
*laṅ* (imperfect), *loṭ* (imperative), and *vidhiliṅ* (optative). A cell may
have more than one valid form where an optional (*vikalpa*) sūtra applies —
`hinvaH` and `hinuvaH` are both correct — and in fact 170 of the 1800 cells
hold more than one form: 109 hold two, 56 hold three (`Bavatu`, `BavatAd`,
`BavatAt`), one holds four (rudhādi's √piṣ loṭ madhyama eka), two hold
five, and two hold six — rudhādi's √kṛt and √rudh loṭ parasmaipada madhyama
eka, tied as the sharpest forks in the suite, each holding six valid readings
of the one cell: `kfndDi` / `kfnDi` / `kfnttAd` / `kfntAd` / `kfnttAt` /
`kfntAt`, and `rundDi` / `runDi` / `rundDAd` / `runDAd` / `rundDAt` / `runDAt`;
`check` reports every derivation, each with its own trace. A root may also
admit **both** padas — √rudh derives a full parasmaipada and a full ātmanepada
paradigm, and `check` reports each analysis with its own pada. 1.3.72's
semantic condition (*kartrabhiprāye kriyāphale*, the fruit of the action
accruing to the agent) is **not** modelled: both arms derive, and the reader
selects by sense. `INVALID` means
"not derivable within this covered grammar," not "ungrammatical in
Sanskrit." See `docs/ARCHITECTURE.md`.

## Layout

See `docs/ARCHITECTURE.md` for the crate map.
