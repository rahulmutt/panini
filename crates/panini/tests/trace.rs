//! Golden test for the ordered sūtra trace itself.
//!
//! The product's headline deliverable is "the ordered sequence of sūtras that
//! derive the word" (see `Analysis.trace` / `Prakriya.log`). The rest of the
//! suite only checks that particular sūtras are *present* in a trace; that
//! would still pass if a `record()` call were dropped or reordered. This
//! file pins the FULL ORDERED sequence for representative forms so a
//! regression like that fails loudly, across all four lakāras this crate
//! covers (laṭ, laṅ, loṭ, vidhiliṅ) — **twelve** tests in total below.
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
//!
//! The pada-sanction step (1.3.78 for these parasmaipada roots; 1.3.12 for
//! atmanepada roots) is the derivation's source of truth for pada and now
//! opens every trace.

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
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.1.68", "1.3.9", "7.3.84", "6.1.78"
        ]
    );
}

#[test]
fn bhavanti_trace_is_exactly_the_ji_coalescence_path() {
    // BU prathama bahu: Ji -> anti (7.1.3) + 6.1.97 para-rupa coalescence path.
    assert_eq!(
        trace_for("Bavanti"),
        vec![
            "1.3.78", "3.4.78", "3.1.68", "1.3.9", "7.1.3", "7.3.84", "6.1.78", "6.1.97"
        ]
    );
}

#[test]
fn bhavamah_trace_is_exactly_the_dirgha_visarga_path() {
    // BU uttama bahu: 7.3.101 dirgha before `mas` + 8.3.15 visarga path.
    assert_eq!(
        trace_for("BavAmaH"),
        vec![
            "1.3.78", "3.4.78", "3.1.68", "1.3.9", "7.3.84", "6.1.78", "7.3.101", "8.3.15"
        ]
    );
}

#[test]
fn abhavat_trace_is_exactly_the_lan_augment_path() {
    // BU laṅ prathama eka: tip -> ti (1.3.9) -> t (3.4.100), aṭ-āgama (6.4.71).
    assert_eq!(
        trace_for("aBavat"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.100", "3.1.68", "1.3.9", "6.4.71", "7.3.84", "6.1.78"
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
            "1.3.78", "3.4.78", "3.4.100", "3.1.68", "1.3.9", "6.4.71", "7.1.3", "7.3.84",
            "6.1.78", "6.1.97", "8.2.23"
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
            "1.3.78", "3.4.78", "1.3.9", "3.4.85", "3.4.86", "3.1.68", "1.3.9", "7.3.84", "6.1.78"
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

#[test]
fn bhavet_trace_is_exactly_the_vidhilin_vali_lopa_path() {
    // BU vidhiliṅ prathama eka: tip -> ti (1.3.9) -> t (3.4.100, now
    // ṅit-wide), yāsuṭ (3.4.103), salopa (7.2.79), yA -> iy (7.2.80),
    // a+i -> e (6.1.87), y dropped before t (6.1.66).
    assert_eq!(
        trace_for("Bavet"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.100", "3.4.103", "3.1.68", "1.3.9", "7.2.79",
            "7.2.80", "7.3.84", "6.1.78", "6.1.87", "6.1.66"
        ]
    );
}

#[test]
fn bhaveyuh_trace_is_exactly_the_jus_path() {
    // BU vidhiliṅ prathama bahu: Ji -> jus (3.4.108) -> us (1.3.9), then the
    // yāsuṭ chain; the y of `yus` SURVIVES 6.1.66 (u is not a val consonant),
    // and word-final s becomes visarga (8.3.15).
    assert_eq!(
        trace_for("BaveyuH"),
        vec![
            "1.3.78", "3.4.78", "3.4.108", "1.3.9", "3.4.103", "3.1.68", "1.3.9", "7.2.79",
            "7.2.80", "7.3.84", "6.1.78", "6.1.87", "8.3.15"
        ]
    );
}

#[test]
fn bhaveyam_trace_is_exactly_the_widened_mip_path() {
    // BU vidhiliṅ uttama eka: mip -> mi (1.3.9) -> am (3.4.101, mip arm now
    // fires outside laṅ), then the yāsuṭ chain; no 6.1.66 (a is a vowel).
    assert_eq!(
        trace_for("Baveyam"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.101", "3.4.103", "3.1.68", "1.3.9", "7.2.79",
            "7.2.80", "7.3.84", "6.1.78", "6.1.87"
        ]
    );
}

#[test]
fn labhate_trace_is_exactly_the_minimal_atmanepada_path() {
    // laB laṭ prathama eka: pada sanction (1.3.12), ṅid-vat atideśa (1.2.4),
    // ta → te (3.4.79). No it-saṃjñā step for `ta` (nothing to strip).
    assert_eq!(
        trace_for("laBate"),
        vec!["1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.68", "1.3.9"]
    );
}

#[test]
fn labhete_trace_is_exactly_the_ato_nitah_path() {
    // laB laṭ prathama dvi: AtAm → Ate (3.4.79) → iyte (7.2.81) →
    // laBe+yte (6.1.87) → laBete (6.1.66).
    assert_eq!(
        trace_for("laBete"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.68", "1.3.9", "7.2.81", "6.1.87", "6.1.66"
        ]
    );
}

#[test]
fn vartate_trace_shows_laghupadha_guna() {
    // vft: 7.3.86 (upadhā guṇa), NOT 7.3.84 (final-ik guṇa).
    let trace = trace_for("vartate");
    assert!(trace.contains(&"7.3.86".to_string()), "got {trace:?}");
    assert!(!trace.contains(&"7.3.84".to_string()), "got {trace:?}");
}

#[test]
fn labhasva_trace_is_exactly_the_savabhyam_path() {
    // laB loṭ madhyama eka: TAs → se (3.4.80) → sva (3.4.91); 3.4.79
    // reports false on `se` (its ṭi is already e) and must not appear.
    assert_eq!(
        trace_for("laBasva"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.85", "3.4.80", "3.4.91", "3.1.68", "1.3.9"
        ]
    );
}

#[test]
fn labhai_trace_is_exactly_the_at_vrddhi_path() {
    // laB loṭ uttama eka: iw → i (1.3.9) → e (3.4.79) → E (3.4.93) →
    // AE (3.4.92); post-śap 6.1.101 (a+A → A) then 6.1.90 (A+E → E).
    // No 1.2.4: loṭ uttama endings are pit (pic ca), not apit.
    assert_eq!(
        trace_for("laBE"),
        vec![
            "1.3.12", "3.4.78", "1.3.9", "3.4.85", "3.4.79", "3.4.93", "3.4.92", "3.1.68", "1.3.9",
            "6.1.101", "6.1.90"
        ]
    );
}

#[test]
fn aidhata_trace_is_exactly_the_at_agama_path() {
    // eD laṅ prathama eka: no pre-śap ending change (ta survives; 3.4.100 is
    // parasmaipada-only), then 6.4.72 āṭ + 6.1.90 vṛddhi on the aṅga.
    assert_eq!(
        trace_for("EData"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.1.68", "1.3.9", "6.4.72", "6.1.90"
        ]
    );
}
