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

Finite verbs (*tiṅanta*), three gaṇas — *bhvādi* (1), *divādi* (4), *tudādi*
(6) — *parasmaipada* and *ātmanepada* (pada taken from each root's tag), over
a curated 24-root set, in four lakāras: *laṭ* (present), *laṅ* (imperfect),
*loṭ* (imperative), and *vidhiliṅ* (optative). `INVALID` means "not derivable
within this covered grammar," not "ungrammatical in Sanskrit." See
`docs/ARCHITECTURE.md`.

## Layout

See `docs/ARCHITECTURE.md` for the crate map.
