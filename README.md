# panini

A Rust library and CLI that validates a single Sanskrit word against Pāṇini's
Aṣṭādhyāyī and returns the sequence of sūtras that derive it.

## Quick start

```
mise install          # pins Rust toolchain
mise run test          # runs the workspace test suite
cargo run -p panini-cli -- check 'bhavati' --trace
```

## v1 scope

Finite verbs (*tiṅanta*), present tense (*laṭ*), gaṇa 1 (*bhvādi*),
*parasmaipada*, over a curated 6-root set. `INVALID` means "not derivable within
this covered grammar," not "ungrammatical in Sanskrit." See `docs/ARCHITECTURE.md`
and `docs/superpowers/plans/2026-07-19-panini-astadhyayi-v1.md`.

## Layout

See `docs/ARCHITECTURE.md` for the crate map.
