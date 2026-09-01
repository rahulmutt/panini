//! Golden test for the ordered sūtra trace itself.
//!
//! The product's headline deliverable is "the ordered sequence of sūtras that
//! derive the word" (see `Analysis.trace` / `Prakriya.log`). The rest of the
//! suite only checks that particular sūtras are *present* in a trace; that
//! would still pass if a `record()` call were dropped or reordered. This
//! file pins the FULL ORDERED sequence for representative forms so a
//! regression like that fails loudly, across all four lakāras this crate
//! covers (laṭ, laṅ, loṭ, vidhiliṅ) — one ordered-trace pin per `#[test]`
//! function in the gaṇa modules. Don't hardcode a count in this comment; it will drift as
//! tests are added. Run
//! `cat crates/panini/tests/trace/*.rs | grep -c '^#\[test\]'`
//! for the current total (anchored to line start so it doesn't match this
//! sentence's own `#[test]` mentions).
//!
//! For the authoritative rule order itself, do not rely on a diagram here:
//! read `TINANTA_RULES` in `crates/panini-prakriya/src/tinanta/mod.rs` and
//! then its six stage files in that order. That flattened static sequence —
//! not this comment — is the source of truth for sequencing; a diagram in
//! this header would drift out of sync with it as rules are added (as
//! happened to the laṭ-only diagram this comment used to carry, before laṅ
//! and loṭ support existed) and a stale diagram is worse than none.
//!
//! Note that an it-samjna step only records 1.3.9 when it actually elides an
//! anubandha: `tip` -> `ti` and `Sap` -> `a` do record it, but `Ji` (nothing to
//! strip) and the 1.3.4-protected `mas` (run_it_samjna skipped entirely) do
//! not, so the pinned sequences differ in more than just which optional steps
//! fire.
//!
//! The pada-sanction step (1.3.78 for these parasmaipada roots; 1.3.12 for
//! atmanepada roots) is the derivation's source of truth for pada and now
//! opens every trace.

mod helpers;

mod adadi;
mod bhvadi;
mod divadi;
mod kryadi;
mod rudhadi;
mod svadi;
mod tanadi;
mod tudadi;
