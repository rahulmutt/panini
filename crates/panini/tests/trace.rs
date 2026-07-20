//! Golden test for the ordered sūtra trace itself.
//!
//! The product's headline deliverable is "the ordered sequence of sūtras that
//! derive the word" (see `Analysis.trace` / `Prakriya.log`). The rest of the
//! suite only checks that particular sūtras are *present* in a trace; that
//! would still pass if a `record()` call were dropped or reordered. This
//! file pins the FULL ORDERED sequence for representative forms so a
//! regression like that fails loudly, across all three lakāras this crate
//! covers (laṭ, laṅ, loṭ) — **nine** tests in total below.
//!
//! For the authoritative rule order itself, do not rely on a diagram here:
//! read `TINANTA_RULES` in `crates/panini-prakriya/src/tinanta.rs` top to
//! bottom. That static array — not this comment — is the source of truth for
//! sequencing; a diagram in this header would drift out of sync with it as
//! rules are added (as happened to the laṭ-only diagram this comment used to
//! carry, before laṅ and loṭ support existed) and a stale diagram is worse
//! than none.
//!
//! Note that an it-samjna step only records 1.3.9 when it actually elides an
//! anubandha: `tip` -> `ti` and `Sap` -> `a` do record it, but `Ji` (nothing to
//! strip) and the 1.3.4-protected `mas` (run_it_samjna skipped entirely) do
//! not, so the sequences below differ in more than just which optional steps
//! fire.

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

#[test]
fn abhavat_trace_is_exactly_the_lan_augment_path() {
    // BU laṅ prathama eka: tip -> ti (1.3.9) -> t (3.4.100), aṭ-āgama (6.4.71).
    assert_eq!(
        trace_for("aBavat"),
        vec![
            "3.4.78", "1.3.9", "3.4.100", "3.1.68", "1.3.9", "6.4.71", "7.3.84", "6.1.78"
        ]
    );
}

#[test]
fn abhavan_trace_is_exactly_the_samyoganta_path() {
    // BU laṅ prathama bahu: Ji -> J (3.4.100) -> ant (7.1.3), then 6.1.97
    // coalescence and 8.2.23 conjunct-final elision: aBavant -> aBavan.
    assert_eq!(
        trace_for("aBavan"),
        vec![
            "3.4.78", "3.4.100", "3.1.68", "1.3.9", "6.4.71", "7.1.3", "7.3.84", "6.1.78",
            "6.1.97", "8.2.23"
        ]
    );
}

#[test]
fn abhavam_trace_shows_dirgha_does_not_fire() {
    // BU laṅ uttama eka: mip -> mi (1.3.9) -> am (3.4.101). The ending begins
    // with a vowel, so 7.3.101 ato dIrGo yaYi must NOT fire — this is why
    // 7.3.101 is ordered before 6.1.97, which strips that leading `a`.
    let trace = trace_for("aBavam");
    assert!(!trace.contains(&"7.3.101".to_string()), "got {trace:?}");
    assert!(trace.contains(&"6.1.97".to_string()), "got {trace:?}");
}

#[test]
fn bhavatu_trace_is_exactly_the_lot_er_uh_path() {
    // BU loṭ prathama eka: tip -> ti (1.3.9) -> tu (3.4.86), via 3.4.85.
    assert_eq!(
        trace_for("Bavatu"),
        vec![
            "3.4.78", "1.3.9", "3.4.85", "3.4.86", "3.1.68", "1.3.9", "7.3.84", "6.1.78"
        ]
    );
}

#[test]
fn bhava_trace_shows_hi_elision() {
    // BU loṭ madhyama eka: sip -> si -> hi (3.4.87), elided by 6.4.105.
    let trace = trace_for("Bava");
    assert!(trace.contains(&"3.4.87".to_string()), "got {trace:?}");
    assert!(trace.contains(&"6.4.105".to_string()), "got {trace:?}");
}

#[test]
fn bhavani_trace_shows_aat_not_dirgha() {
    // BU loṭ uttama eka: mip -> mi -> ni (3.4.89) -> Ani (3.4.92), then
    // 6.1.101 savarṇa-dīrgha. 7.3.101 must NOT fire, or the vowel would be
    // lengthened twice.
    let trace = trace_for("BavAni");
    assert!(trace.contains(&"3.4.89".to_string()), "got {trace:?}");
    assert!(trace.contains(&"3.4.92".to_string()), "got {trace:?}");
    assert!(trace.contains(&"6.1.101".to_string()), "got {trace:?}");
    assert!(!trace.contains(&"7.3.101".to_string()), "got {trace:?}");
}
