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
(9, śnā) and *svādi* (5, śnu) — plus *rudhādi* (7, śnam) **partial**: six of
its roots (√kṛt, √hiṃs, √khid, √bhañj, √piṣ, √indh) of 25. Nine more rudhādi
roots are ubhayapadī and deferred behind 1.3.72 — which is why the gaṇa lacks
its own eponym, √rudh — nine reachable ones are simply not curated yet, and
the last, √bhuj, is deferred behind 1.3.66 *bhujo'navane*, which forks its
pada on sense rather than on an axis this engine models.
*parasmaipada* and *ātmanepada* (pada taken from each root's tag), over a
curated 48-root set, in four lakāras: *laṭ* (present), *laṅ*
(imperfect), *loṭ* (imperative), and *vidhiliṅ* (optative). A cell may have
more than one valid form where an optional (*vikalpa*) sūtra applies —
`hinvaH` and `hinuvaH` are both correct — and in fact 149 of the 1728 cells
hold more than one form: 91 hold two, 55 hold three (`Bavatu`, `BavatAd`,
`BavatAt`), one holds four (rudhādi's √piṣ loṭ madhyama eka), one holds
five, and one — rudhādi's √kṛt loṭ madhyama eka, the sharpest fork in the
suite — holds six, all valid readings of the same
cell: `kfndDi` / `kfnDi` / `kfnttAd` / `kfntAd` / `kfnttAt` / `kfntAt`;
`check` reports every derivation, each with its own trace. `INVALID` means
"not derivable within this covered grammar," not "ungrammatical in
Sanskrit." See `docs/ARCHITECTURE.md`.

## Layout

See `docs/ARCHITECTURE.md` for the crate map.
