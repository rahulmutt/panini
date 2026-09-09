//! juhotyadi's ordered-trace witnesses. Helpers live in
//! `crate::helpers`; the module doc governing this suite is in
//! `main.rs`.
//!
//! The order these pins hold is THIS engine's: dvitva (6.1.10) and the
//! abhyāsa rules run before guṇa (7.3.84), the Kaumudī sequence, where
//! vidyut-prakriya guṇates first, copies, and shortens the copy back by
//! 7.4.59. Forms agree; the traces do not, and these pins are what make
//! the engine's own order a checked fact rather than an accident.

use crate::helpers::{at, cell_trace};
use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};
use panini_prakriya::derive;

#[test]
fn juhoti_trace_is_slu_dvitva_cutva_guna_then_car_ca() {
    // hu laT P.E. The reduplication core in pipeline order, and two
    // load-bearing absences: no 1.2.4 (ti is pit; the ślu'd śap is pit
    // too, so the second 1.2.4 declines), and no 2.4.72 (ślu, not luk).
    let (text, t) = cell_trace(
        "03.0001",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "juhoti", "got {t:?}");
    assert!(at(&t, "3.1.68") < at(&t, "2.4.75"), "got {t:?}");
    assert!(at(&t, "2.4.75") < at(&t, "6.1.10"), "got {t:?}");
    assert!(at(&t, "6.1.10") < at(&t, "7.4.62"), "got {t:?}");
    assert!(at(&t, "7.4.62") < at(&t, "7.3.84"), "got {t:?}");
    assert!(at(&t, "7.3.84") < at(&t, "8.4.54"), "got {t:?}");
    assert!(!t.contains(&"1.2.4".to_string()), "got {t:?}");
    assert!(!t.contains(&"2.4.72".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.4.59".to_string()), "got {t:?}");
}

#[test]
fn juhvati_trace_credits_7_1_4_and_the_hu_arm_of_6_4_87() {
    // hu laT P.B. The jh goes by 7.1.4 (at), not 7.1.3 (ant); the yaṇ is
    // 6.4.87's hu arm on the ROOT, not 6.1.77 (tanādi's u) nor 6.4.77.
    let (text, t) = cell_trace(
        "03.0001",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "juhvati", "got {t:?}");
    assert!(at(&t, "7.1.4") < at(&t, "6.4.87"), "got {t:?}");
    assert!(t.contains(&"1.2.4".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.1.3".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.1.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.84".to_string()), "got {t:?}");
}

#[test]
fn ajuhavuh_trace_credits_3_4_109_and_7_3_83_with_the_augment_in_agama() {
    // hu laN P.B. jus by 3.4.109 (not 3.4.108, not 7.1.3), guṇa by 7.3.83
    // (not 7.3.84, which the ṅit us blocks), av by 6.1.78; and the augment
    // is its own term, the abhyāsa another.
    let d = dhatus().iter().find(|d| d.dhatupatha == "03.0001").unwrap();
    let branches: Vec<_> = derive(
        d,
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    )
    .into_iter()
    .filter(|p| !p.blocked)
    .collect();
    assert_eq!(branches.len(), 1, "ajuhavuH does not fork");
    let p = &branches[0];
    let t: Vec<String> = p.log.iter().map(|s| s.sutra.clone()).collect();
    assert_eq!(p.text(), "ajuhavuH", "got {t:?}");
    assert!(at(&t, "3.4.109") < at(&t, "3.1.68"), "got {t:?}");
    assert!(at(&t, "6.4.71") < at(&t, "7.3.83"), "got {t:?}");
    assert!(at(&t, "7.3.83") < at(&t, "6.1.78"), "got {t:?}");
    assert!(!t.contains(&"3.4.108".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.84".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.1.3".to_string()), "got {t:?}");
    assert_eq!(p.terms[0].text, "a", "AGAMA");
    assert_eq!(p.terms[1].text, "ju", "ABHYASA");
    assert_eq!(p.terms[2].text, "hav", "ANGA");
}

#[test]
fn juhudhi_trace_credits_6_4_101_and_cikihi_does_not() {
    // hu loT M.E. — a three-form cell (tātaṅ pair); branch 0 is the
    // declined juhuDi, whose Di is 6.4.101's hu arm. ki's cikihi keeps hi:
    // i is no jhal and the root is not √hu.
    let (text, t) = cell_trace(
        "03.0001",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "juhuDi", "got {t:?}");
    assert!(at(&t, "3.4.87") < at(&t, "6.4.101"), "got {t:?}");
    assert!(!t.contains(&"6.4.105".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.106".to_string()), "got {t:?}");
    let (text, t) = cell_trace(
        "03.0020",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "cikihi", "got {t:?}");
    assert!(!t.contains(&"6.4.101".to_string()), "got {t:?}");
}

#[test]
fn cikyati_trace_credits_6_4_82_not_6_4_77() {
    // ki laT P.B. The i → y is 6.4.82 (anekāc over ci-ki, asaṁyogapūrva),
    // the apavāda, not 6.4.77's iyaṅ; and ci records no 8.4.54 (already car).
    let (text, t) = cell_trace(
        "03.0020",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "cikyati", "got {t:?}");
    assert!(at(&t, "7.1.4") < at(&t, "6.4.82"), "got {t:?}");
    assert!(!t.contains(&"6.4.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.54".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.84".to_string()), "got {t:?}");
}

#[test]
fn cikayani_trace_gunates_before_6_4_82_could_see_the_i() {
    // ki loT U.E. Ani is pit: 7.3.84 (ke) then 6.1.78 (kay), and 6.4.82
    // must NOT have fired — the ordering pin the spec names.
    let (text, t) = cell_trace(
        "03.0020",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "cikayAni", "got {t:?}");
    assert!(at(&t, "3.4.92") < at(&t, "7.3.84"), "got {t:?}");
    assert!(at(&t, "7.3.84") < at(&t, "6.1.78"), "got {t:?}");
    assert!(!t.contains(&"6.4.82".to_string()), "got {t:?}");
}

#[test]
fn ciketi_trace_carries_no_8_4_54_and_juhuyuh_no_7_3_83() {
    // Two absences vidyut's credits confirm: ki's abhyāsa ci is already
    // car, and vidhiliṅ's jus sits behind yāsuṭ, out of 7.3.83's reach.
    let (text, t) = cell_trace(
        "03.0020",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "ciketi", "got {t:?}");
    assert!(t.contains(&"7.4.62".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.54".to_string()), "got {t:?}");
    let (text, t) = cell_trace(
        "03.0001",
        Lakara::VidhiLin,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "juhuyuH", "got {t:?}");
    assert!(t.contains(&"3.4.108".to_string()), "got {t:?}");
    assert!(t.contains(&"6.1.96".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.83".to_string()), "got {t:?}");
    assert!(!t.contains(&"3.4.109".to_string()), "got {t:?}");
}
