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

Finite verbs (*tiṅanta*), six gaṇas — *bhvādi* (1, vikaraṇa śap), *divādi*
(4, śyan), *tudādi* (6, śa), *adādi* (2, śap luk'd), *kryādi* (9, śnā) and
*svādi* (5, śnu) — all fully covered, *parasmaipada* and *ātmanepada* (pada
taken from each root's tag), over a curated 42-root set, in four lakāras:
*laṭ* (present), *laṅ* (imperfect), *loṭ* (imperative), and *vidhiliṅ*
(optative). A cell may have more than one valid form where an optional
(*vikalpa*) sūtra applies — `hinvaH` and `hinuvaH` are both correct — and in
fact 106 of the 1512 cells hold more than one form, 48 of them three
(`Bavatu`, `BavatAd`, `BavatAt`); `check` reports every derivation, each with
its own trace. `INVALID` means
"not derivable within this covered grammar," not "ungrammatical in
Sanskrit." See `docs/ARCHITECTURE.md`.

## Layout

See `docs/ARCHITECTURE.md` for the crate map.
