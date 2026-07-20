//! Golden test for the ordered sūtra trace itself.
//!
//! The product's headline deliverable is "the ordered sequence of sūtras that
//! derive the word" (see `Analysis.trace` / `Prakriya.log`). The rest of the
//! suite only checks that particular sūtras are *present* in a trace; that
//! would still pass if a `record()` call were dropped or reordered. This test
//! pins the FULL ORDERED sequence for three representative forms so a
//! regression like that fails loudly.
//!
//! The three forms exercise the three distinct rule paths documented in the
//! v1 plan's fixed pipeline:
//! `3.4.78 -> it-samjna[1.3.9] -> 3.1.68 -> it-samjna[1.3.9] -> 7.1.3 (Ji only)
//!  -> 7.3.84 guna -> 6.1.78 ayadesa -> 6.1.97 (anti coalescence only)
//!  -> 7.3.101 (before m/v endings only) -> 8.3.15 (final s -> visarga only)`.
//!
//! Note that an it-samjna step only records 1.3.9 when it actually elides an
//! anubandha: `tip` -> `ti` and `Sap` -> `a` do record it, but `Ji` (nothing to
//! strip) and the 1.3.4-protected `mas` (run_it_samjna skipped entirely) do
//! not, so the three sequences below differ in more than just which optional
//! steps fire.

use panini::Panini;

fn trace_for(word: &str) -> Vec<String> {
    let engine = Panini::new();
    let r = engine.check(word);
    let a = r
        .analyses
        .iter()
        .find(|a| a.form_slp1 == word)
        .expect("expected an analysis deriving exactly this surface form");
    a.trace.iter().map(|s| s.sutra.clone()).collect()
}

#[test]
fn bhavati_trace_is_exactly_the_base_path() {
    // BU prathama eka: base path, ending `tip` -> `ti`.
    assert_eq!(
        trace_for("Bavati"),
        vec!["3.4.78", "1.3.9", "3.1.68", "1.3.9", "7.3.84", "6.1.78"]
    );
}

#[test]
fn bhavanti_trace_is_exactly_the_ji_coalescence_path() {
    // BU prathama bahu: Ji -> anti (7.1.3) + 6.1.97 para-rupa coalescence path.
    assert_eq!(
        trace_for("Bavanti"),
        vec![
            "3.4.78", "3.1.68", "1.3.9", "7.1.3", "7.3.84", "6.1.78", "6.1.97"
        ]
    );
}

#[test]
fn bhavamah_trace_is_exactly_the_dirgha_visarga_path() {
    // BU uttama bahu: 7.3.101 dirgha before `mas` + 8.3.15 visarga path.
    assert_eq!(
        trace_for("BavAmaH"),
        vec![
            "3.4.78", "3.1.68", "1.3.9", "7.3.84", "6.1.78", "7.3.101", "8.3.15"
        ]
    );
}
