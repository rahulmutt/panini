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
(6), and *adādi* (2), all fully covered — *parasmaipada* and *ātmanepada*
(pada taken from each root's tag), over a curated 30-root set, in four
lakāras: *laṭ* (present), *laṅ* (imperfect), *loṭ* (imperative), and
*vidhiliṅ* (optative). Adādi is complete: √yā, √vā (parasmaipada, ā-final)
and √ad (parasmaipada), plus √ās, √vas and √śī (*ātmanepada*), each across
all four lakāras, including the athematic (śap-luk'd) ātmanepada path. See
`docs/superpowers/specs/2026-07-25-adadi-si-5f-design.md` for √śī's rule
analysis. `INVALID` means "not derivable within this covered grammar,"
not "ungrammatical in Sanskrit." See `docs/ARCHITECTURE.md`.

## Layout

See `docs/ARCHITECTURE.md` for the crate map.
