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

Finite verbs (*tiṅanta*), four gaṇas — *bhvādi* (1), *divādi* (4), *tudādi*
(6) fully, and *adādi* (2) partially — *parasmaipada* and *ātmanepada* (pada
taken from each root's tag), over a curated 26-root set, in four lakāras:
*laṭ* (present), *laṅ* (imperfect), *loṭ* (imperative), and *vidhiliṅ*
(optative). The adādi entry is the two ā-final roots √yā and √vā in laṭ, laṅ
and loṭ only: adādi in vidhiliṅ needs the athematic optative, which is not
implemented yet and is therefore declined rather than derived. `INVALID`
means "not derivable within this covered grammar," not "ungrammatical in
Sanskrit." See `docs/ARCHITECTURE.md`.

## Layout

See `docs/ARCHITECTURE.md` for the crate map.
