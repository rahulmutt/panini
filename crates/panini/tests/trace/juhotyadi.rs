//! juhotyadi's ordered-trace witnesses. Helpers live in
//! `crate::helpers`; the module doc governing this suite is in
//! `main.rs`.
//!
//! The order these pins hold is THIS engine's: dvitva (6.1.10) and the
//! abhyāsa rules run before guṇa (7.3.84), the Kaumudī sequence, where
//! vidyut-prakriya guṇates first, copies, and shortens the copy back by
//! 7.4.59. The same choice makes 7.4.66 *ur at* fire on every cell of a
//! slice-3d ṛ-root (vidyut's fires only on the kṅit ones), and puts 7.1.102
//! *ud oṣṭhyapūrvasya* after dvitva, on the aṅga alone (*pipūrtaḥ*'s
//! abhyāsa is copied from `pF`). The same order runs 6.4.78 (slice 3d2's √ṛ)
//! inside the abhyāsa stage, before laṅ's 6.4.72 and before guṇa, where vidyut
//! guṇates first (on pit cells only) and reaches 6.4.78 later. Slice 3e's
//! 7.4.75 runs inside the same stage, after 7.4.60, so 7.4.59 never fires on
//! its rows, and 7.3.87 sits in `guna` after the whole stage, just before
//! 7.3.84, where vidyut credits it between 6.1.10 and 7.4.60. Forms agree; the traces do not, and
//! these pins are what make the engine's own order a checked fact rather than an
//! accident.

use crate::helpers::{at, cell_trace, credited};
use panini_data::{Gana, Lakara, Pada, Purusha, Vacana, dhatus};
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

#[test]
fn jihreti_trace_orders_haladih_shesha_before_hrasvah_before_kuhoshcuh() {
    // hrI laT P.E. The abhyāsa's whole shaping chain in pipeline order:
    // hrI → hI (7.4.60) → hi (7.4.59) → Ji (7.4.62) → ji (8.4.54).
    let (text, t) = cell_trace(
        "03.0003",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "jihreti", "got {t:?}");
    assert!(at(&t, "6.1.10") < at(&t, "7.4.60"), "got {t:?}");
    assert!(at(&t, "7.4.60") < at(&t, "7.4.59"), "got {t:?}");
    assert!(at(&t, "7.4.59") < at(&t, "7.4.62"), "got {t:?}");
    assert!(at(&t, "7.4.62") < at(&t, "8.4.54"), "got {t:?}");
}

#[test]
fn jihriyati_trace_credits_6_4_77_and_not_6_4_82() {
    // hrI laT P.B. The abhyasta span is `Ji` + `hrI`, so the two sounds
    // before the final I are r then h — saṁyogapūrva. 6.4.82 declines and
    // its utsarga 6.4.77 takes the cell.
    let (text, t) = cell_trace(
        "03.0003",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "jihriyati", "got {t:?}");
    assert!(t.contains(&"6.4.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.82".to_string()), "got {t:?}");
    assert!(at(&t, "7.1.4") < at(&t, "6.4.77"), "got {t:?}");
}

#[test]
fn bibhyati_trace_credits_6_4_82_on_a_long_i_and_no_6_4_115() {
    // BI laT P.B. 7.4.59 shortened the ABHYĀSA only, so 6.4.82 fires on a
    // long I. 6.4.115 is absent because `ati` is ajādi — the *hali*
    // clause's witness, and why this cell has exactly one form.
    let (text, t) = cell_trace(
        "03.0002",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "biByati", "got {t:?}");
    assert!(at(&t, "7.4.59") < at(&t, "6.4.82"), "got {t:?}");
    assert!(!t.contains(&"6.4.115".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.4.60".to_string()), "got {t:?}");
}

#[test]
fn bibhitah_trace_shows_the_unforked_arm_with_the_abhyasa_shortened() {
    // BI laT P.D, the unforked arm. 7.4.59 has shortened the abhyāsa to Bi,
    // but 6.4.115 does not fire here, so the aṅga stays long: biBItaH. The
    // forked arm, where 6.4.115 also shortens the aṅga, is pinned below.
    let (text, t) = cell_trace(
        "03.0002",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "biBItaH", "got {t:?}");
    assert!(!t.contains(&"6.4.115".to_string()), "got {t:?}");
    // The forked arm is the one 6.4.115 produces; `cell_trace` returns the
    // unforked derivation, so the fork itself is pinned by the ALTERNATES
    // row `("03.0002", "laT", Pada::Parasmaipada, 1, "biBitaH", "6.4.115")`
    // that Task 5 added.
    assert!(at(&t, "6.1.10") < at(&t, "7.4.59"), "got {t:?}");
}

#[test]
fn jihrayani_trace_credits_8_4_2_across_the_intervening_sounds() {
    // hrI loT U.E. The r of `hray`, then a, y, A, then ni → Ri. All three
    // interveners are aṭ, which `is_natva_intervener` already carries; this
    // is ṇatva's first abhyāsa-bearing word and it needed no edit.
    let (text, t) = cell_trace(
        "03.0003",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "jihrayARi", "got {t:?}");
    assert!(t.contains(&"8.4.2".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.1".to_string()), "got {t:?}");
    assert!(at(&t, "6.1.78") < at(&t, "8.4.2"), "got {t:?}");
}

#[test]
fn mimite_trace_orders_hrasvah_bhrnam_it_then_i_halyaghoh() {
    // mA Ā laT P.E. 7.4.59 shortens the abhyāsa (ma), 7.4.76 makes it mi,
    // and 6.4.113's abhyasta arm turns the aṅga's ā to ī before the kṅit
    // hal-initial te. No 6.4.112 (a consonant follows and √mā is not ghu)
    // and no 7.4.62 (m is no velar).
    let (text, t) = cell_trace(
        "03.0007",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "mimIte", "got {t:?}");
    assert!(at(&t, "7.4.59") < at(&t, "7.4.76"), "got {t:?}");
    assert!(at(&t, "7.4.76") < at(&t, "6.4.113"), "got {t:?}");
    assert!(!t.contains(&"6.4.112".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.4.62".to_string()), "got {t:?}");
}

#[test]
fn dehi_trace_credits_6_4_119_and_neither_6_4_112_nor_6_4_101() {
    // dA P loT M.E, branch 0 (no tātaṅ). 6.4.119 gives de and elides the
    // abhyāsa; 6.4.112 never sees an ā, and 6.4.101 sees `e`, not a jhal,
    // before hi.
    let (text, t) = cell_trace(
        "03.0010",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "dehi", "got {t:?}");
    assert!(at(&t, "6.1.10") < at(&t, "6.4.119"), "got {t:?}");
    for absent in ["6.4.112", "6.4.113", "6.4.101", "7.1.35"] {
        assert!(!t.contains(&absent.to_string()), "{absent} in {t:?}");
    }
}

#[test]
fn dattah_trace_credits_6_4_112_and_not_6_4_113_by_aghoh() {
    // dA P laT P.D. √dā is ghu, so 6.4.113 declines before the hal-initial
    // tas and 6.4.112 elides the ā; 8.4.55 then devoices d before t. No
    // 8.2.38: the identical shape is √dhā's alone.
    let (text, t) = cell_trace(
        "03.0010",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "dattaH", "got {t:?}");
    assert!(at(&t, "6.4.112") < at(&t, "8.4.55"), "got {t:?}");
    assert!(!t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.2.38".to_string()), "got {t:?}");
}

#[test]
fn dhattah_trace_orders_car_ca_then_dadhas_tathos_ca_then_khari_ca() {
    // DA P laT P.D. 8.4.54 deaspirates the abhyāsa, 8.2.38 — out of sūtra
    // order — re-aspirates it, 8.4.55 devoices the root's D. 8.2.40's adhaḥ
    // keeps the t a t.
    let (text, t) = cell_trace(
        "03.0011",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "DattaH", "got {t:?}");
    assert!(at(&t, "6.4.112") < at(&t, "8.4.54"), "got {t:?}");
    assert!(at(&t, "8.4.54") < at(&t, "8.2.38"), "got {t:?}");
    assert!(at(&t, "8.2.38") < at(&t, "8.4.55"), "got {t:?}");
    assert!(!t.contains(&"8.2.40".to_string()), "got {t:?}");
}

#[test]
fn dhaddhve_trace_orders_jas_jhasi_then_car_ca_then_dadhas_tathos_ca() {
    // DA Ā laT M.B. 8.4.53 voices the root's D before Dve, 8.4.54
    // deaspirates the abhyāsa, 8.2.38 re-aspirates it before dhv.
    let (text, t) = cell_trace(
        "03.0011",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Madhyama,
        Vacana::Bahu,
    );
    assert_eq!(text, "DadDve", "got {t:?}");
    assert!(at(&t, "8.4.53") < at(&t, "8.4.54"), "got {t:?}");
    assert!(at(&t, "8.4.54") < at(&t, "8.2.38"), "got {t:?}");
}

#[test]
fn dadhati_trace_carries_no_dadhas_tathos_ca() {
    // DA P laT P.E. The ti is pit, so the ā survives and there is no dadh:
    // 8.2.38's single-consonant test declines. 8.4.54 still deaspirates.
    let (text, t) = cell_trace(
        "03.0011",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "daDAti", "got {t:?}");
    assert!(t.contains(&"8.4.54".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.2.38".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.112".to_string()), "got {t:?}");
}

#[test]
fn dadai_trace_is_atas_ca_then_vrddhir_eci_not_akah_savarne() {
    // dA Ā loT U.E. The Kaumudī's path: the āṭ merges with its own ending
    // (6.1.90), then the root's ā with that (6.1.88). 6.1.101 must NOT
    // appear: its adādi arm declines on āṭ + ec. The form is the same either
    // way, so this pin is what holds the decline.
    let (text, t) = cell_trace(
        "03.0010",
        Lakara::Lot,
        Pada::Atmanepada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "dadE", "got {t:?}");
    assert!(at(&t, "6.1.90") < at(&t, "6.1.88"), "got {t:?}");
    assert!(!t.contains(&"6.1.101".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.112".to_string()), "got {t:?}");
}

#[test]
fn adaduh_trace_credits_6_4_112_not_usy_apadantat() {
    // dA P laN P.B. 3.4.109 makes jhi into us, which keeps 1.2.4's ṅit, so
    // 6.4.112 elides the ā before it. 6.1.96's junction arm spells the same
    // form and must not fire. 7.3.83 jusi ca is genuinely LIVE on this
    // ending too — 3.4.109's us is exactly what it conditions on — but it
    // declines on the aṅga: √dā's `A`/`d` offers no ik for guṇa.
    let (text, t) = cell_trace(
        "03.0010",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "adaduH", "got {t:?}");
    assert!(at(&t, "3.4.109") < at(&t, "6.4.112"), "got {t:?}");
    assert!(!t.contains(&"6.1.96".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.83".to_string()), "got {t:?}");
}

#[test]
fn dadaate_trace_credits_6_4_112_not_akah_savarne() {
    // dA Ā laT P.D. The ā goes by 6.4.112 before the kṅit Ate; 6.1.101
    // would spell the same dadAte, which is why this is pinned. (Named
    // dadaate, not dadate, to distinguish this dadAte/prathama-dvi pin
    // from the distinct dadate/prathama-bahu golden in the same block.)
    let (text, t) = cell_trace(
        "03.0010",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "dadAte", "got {t:?}");
    assert!(t.contains(&"6.4.112".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.1.101".to_string()), "got {t:?}");
}

/// Every live branch of one √hā cell, as (text, trace).
fn ha_branches(lakara: Lakara, purusha: Purusha, vacana: Vacana) -> Vec<(String, Vec<String>)> {
    let d = dhatus().iter().find(|d| d.dhatupatha == "03.0009").unwrap();
    derive(d, lakara, Pada::Parasmaipada, purusha, vacana)
        .into_iter()
        .filter(|p| !p.blocked)
        .map(|p| (p.text(), p.log.iter().map(|s| s.sutra.clone()).collect()))
        .collect()
}

fn branch<'a>(bs: &'a [(String, Vec<String>)], form: &str) -> &'a Vec<String> {
    &bs.iter()
        .find(|(t, _)| t == form)
        .unwrap_or_else(|| panic!("no branch {form}"))
        .1
}

#[test]
fn jahaati_trace_takes_no_bhrnam_it() {
    // hA P laT P.E. 03.0009 is `hA` exactly like 03.0008 (jihIte), and
    // 7.4.76 names only the latter: the derivation witness 3c's guard test
    // promised.
    let (text, t) = cell_trace(
        "03.0009",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "jahAti", "got {t:?}");
    assert!(!t.contains(&"7.4.76".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.116".to_string()), "got {t:?}");
}

#[test]
fn jahahi_trace_is_a_ca_hau_and_nothing_it_bars() {
    // hA P loT M.E, the seven-form cell. 6.4.117 changes no text; its branch
    // must carry none of the three rules it bars — 6.4.112 included, whose
    // absence is what keeps *jahhi out.
    let bs = ha_branches(Lakara::Lot, Purusha::Madhyama, Vacana::Eka);
    assert_eq!(bs.len(), 7, "got {bs:?}");
    assert_eq!(bs[0].0, "jahIhi", "declined first");
    let t = branch(&bs, "jahAhi");
    assert!(t.contains(&"6.4.117".to_string()), "got {t:?}");
    for barred in ["6.4.116", "6.4.113", "6.4.112"] {
        assert!(!t.contains(&barred.to_string()), "{barred} in {t:?}");
    }
    let t = branch(&bs, "jahihi");
    assert!(t.contains(&"6.4.116".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.113".to_string()), "got {t:?}");
    let t = &bs[0].1;
    assert!(t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.117".to_string()), "got {t:?}");
}

#[test]
fn jahitah_trace_credits_jahates_ca_not_i_halyaghoh() {
    // hA P laT P.D. The alternate reading: 6.4.116's i bleeds 6.4.113.
    let bs = ha_branches(Lakara::Lat, Purusha::Prathama, Vacana::Dvi);
    let t = branch(&bs, "jahitaH");
    assert!(at(t, "6.1.10") < at(t, "6.4.116"), "got {t:?}");
    assert!(!t.contains(&"6.4.113".to_string()), "got {t:?}");
    let t = branch(&bs, "jahItaH");
    assert!(t.contains(&"6.4.113".to_string()), "got {t:?}");
}

#[test]
fn jahyat_trace_is_lopo_yi_before_any_i_substitute() {
    // hA P viDiliN P.E. 6.4.118 elides the ā before yāsuṭ's y, so neither
    // 6.4.116 nor 6.4.113 can fire (not *jahIyAt, the pre-slice form).
    let (text, t) = cell_trace(
        "03.0009",
        Lakara::VidhiLin,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "jahyAd", "got {t:?}");
    assert!(at(&t, "7.2.79") < at(&t, "6.4.118"), "got {t:?}");
    for absent in ["6.4.116", "6.4.113", "6.4.112"] {
        assert!(!t.contains(&absent.to_string()), "{absent} in {t:?}");
    }
}

#[test]
fn jigati_trace_is_kuhos_cuh_then_bahulam_chandasi() {
    // gA P laT P.E. 7.4.62 palatalises ga → ja, then 7.4.78 gives ji.
    let (text, t) = cell_trace(
        "03.0026",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "jigAti", "got {t:?}");
    assert!(at(&t, "7.4.62") < at(&t, "7.4.78"), "got {t:?}");
    assert!(!t.contains(&"7.4.76".to_string()), "got {t:?}");
}

#[test]
fn bibharti_trace_is_ur_at_then_haladih_shesha_then_bhrnam_it() {
    // Bf P laT P.E. 7.4.66 fires even here, on a pit cell, because this
    // engine copies the bare root before guṇa. vidyut copies `Bar` and skips
    // it. This is the √bhṛñ witness 7.4.76's comment promised.
    let (text, t) = cell_trace(
        "03.0006",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "biBarti", "got {t:?}");
    assert!(at(&t, "6.1.10") < at(&t, "7.4.66"), "got {t:?}");
    assert!(at(&t, "7.4.66") < at(&t, "7.4.60"), "got {t:?}");
    assert!(at(&t, "7.4.60") < at(&t, "7.4.76"), "got {t:?}");
    assert!(at(&t, "7.4.76") < at(&t, "7.3.84"), "got {t:?}");
    assert!(at(&t, "7.3.84") < at(&t, "8.4.54"), "got {t:?}");
    assert!(!t.contains(&"6.1.77".to_string()), "got {t:?}");
}

#[test]
fn bibhrati_trace_takes_the_anga_arm_of_iko_yan_aci() {
    // Bf P laT P.B. 7.1.4 gives `ati`; the ṛ meets it directly under ślu.
    let (text, t) = cell_trace(
        "03.0006",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "biBrati", "got {t:?}");
    assert!(at(&t, "7.4.76") < at(&t, "7.1.4"), "got {t:?}");
    assert!(at(&t, "7.1.4") < at(&t, "6.1.77"), "got {t:?}");
    assert!(!t.contains(&"7.3.84".to_string()), "got {t:?}");
}

#[test]
fn piparti_trace_is_arti_pipartyos_ca_not_bhrnam_it() {
    // pf P laT P.E.
    let (text, t) = cell_trace(
        "03.0005",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "piparti", "got {t:?}");
    assert!(at(&t, "7.4.66") < at(&t, "7.4.60"), "got {t:?}");
    assert!(at(&t, "7.4.60") < at(&t, "7.4.77"), "got {t:?}");
    assert!(!t.contains(&"7.4.76".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.1.102".to_string()), "got {t:?}");
}

#[test]
fn pipurtah_trace_is_ud_oshthyapurvasya_then_hali_ca() {
    // pF P laT P.D: kṅit `tas`, so guṇa declines, 7.1.102 makes `pur`, and
    // 8.2.77 lengthens it before the ending's `t` with SHAP empty.
    let (text, t) = cell_trace(
        "03.0004",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "pipUrtaH", "got {t:?}");
    assert!(at(&t, "7.4.77") < at(&t, "7.1.102"), "got {t:?}");
    assert!(at(&t, "7.1.102") < at(&t, "8.2.77"), "got {t:?}");
    assert!(!t.contains(&"7.3.84".to_string()), "got {t:?}");
}

#[test]
fn pipurati_trace_has_ud_oshthyapurvasya_but_no_lengthening() {
    // pF P laT P.B: `ati` is vowel-initial, so 8.2.77 declines, and 6.1.77
    // never sees a ṛ-final aṅga.
    let (text, t) = cell_trace(
        "03.0004",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "pipurati", "got {t:?}");
    assert!(t.contains(&"7.1.102".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.2.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.1.77".to_string()), "got {t:?}");
}

#[test]
fn jagharti_trace_is_ur_at_haladih_shesha_kuhos_cuh_then_car_ca() {
    // Gf P laT P.E. gh → jh (7.4.62) → j (8.4.54).
    let (text, t) = cell_trace(
        "03.0015",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "jaGarti", "got {t:?}");
    assert!(at(&t, "7.4.66") < at(&t, "7.4.60"), "got {t:?}");
    assert!(at(&t, "7.4.60") < at(&t, "7.4.62"), "got {t:?}");
    assert!(at(&t, "7.4.62") < at(&t, "8.4.54"), "got {t:?}");
}

#[test]
fn sasrati_trace_has_no_shatva_and_no_8_3_110() {
    // sf P laT P.B. vidyut credits 8.3.110 here, a bar on a ṣatva that this
    // engine's 8.3.59 cannot reach: it retroflexes only an affix or ādeśa `s`
    // after the aṅga, so the root-initial `s` is out of its reach. Both
    // absences are the pin that 8.3.110 was deliberately not transcribed.
    let (text, t) = cell_trace(
        "03.0018",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "sasrati", "got {t:?}");
    assert!(t.contains(&"6.1.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.3.59".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.3.110".to_string()), "got {t:?}");
}

#[test]
fn iyarti_trace_is_ur_at_haladih_shesha_arti_then_abhyasasyasavarne() {
    // f P laT P.E. A pit cell: 7.4.66 fires on the bare copy here (vidyut
    // copies the guṇated `ar` and skips it), and 6.4.78 precedes guṇa.
    let (text, t) = cell_trace(
        "03.0017",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "iyarti", "got {t:?}");
    assert!(at(&t, "6.1.10") < at(&t, "7.4.66"), "got {t:?}");
    assert!(at(&t, "7.4.66") < at(&t, "7.4.60"), "got {t:?}");
    assert!(at(&t, "7.4.60") < at(&t, "7.4.77"), "got {t:?}");
    assert!(at(&t, "7.4.77") < at(&t, "6.4.78"), "got {t:?}");
    assert!(at(&t, "6.4.78") < at(&t, "7.3.84"), "got {t:?}");
}

#[test]
fn iyftah_trace_has_abhyasasyasavarne_and_no_guna() {
    // f P laT P.D. kṅit: the aṅga keeps its `f`, which is what 6.4.78 reads.
    let (text, t) = cell_trace(
        "03.0017",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "iyftaH", "got {t:?}");
    assert!(at(&t, "7.4.77") < at(&t, "6.4.78"), "got {t:?}");
    assert!(!t.contains(&"7.3.84".to_string()), "got {t:?}");
}

#[test]
fn iyrati_trace_is_abhyasasyasavarne_before_iko_yan_aci() {
    // f P laT P.B. 6.4.78 must see the root's vowel before 6.1.77 turns it
    // into `r`, or the abhyāsa would stay `i` (*irati).
    let (text, t) = cell_trace(
        "03.0017",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "iyrati", "got {t:?}");
    assert!(at(&t, "6.4.78") < at(&t, "7.1.4"), "got {t:?}");
    assert!(at(&t, "7.1.4") < at(&t, "6.1.77"), "got {t:?}");
}

#[test]
fn eyaruh_trace_is_abhyasasyasavarne_then_at_then_awas_ca_on_the_abhyasa() {
    // f P laN P.B. 6.4.72 (āṭ, not 6.4.71's aṭ) reads ANGA's vowel initial;
    // 7.3.83 guṇates before jus; 6.1.90 merges A+iy → Ey into the abhyāsa.
    let (text, t) = cell_trace(
        "03.0017",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "EyaruH", "got {t:?}");
    assert!(at(&t, "6.4.78") < at(&t, "6.4.72"), "got {t:?}");
    assert!(at(&t, "6.4.72") < at(&t, "7.3.83"), "got {t:?}");
    assert!(at(&t, "7.3.83") < at(&t, "6.1.90"), "got {t:?}");
    assert!(!t.contains(&"6.4.71".to_string()), "got {t:?}");
}

#[test]
fn nenekti_trace_is_dvitva_haladih_shesha_nijam_then_upadha_guna() {
    // nij P laT P.E. A pit, consonant-initial ending: 7.3.87 declines (*aci*),
    // and 7.3.86 guṇates the root after the abhyāsa stage has run.
    let (text, t) = cell_trace(
        "03.0012",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "nenekti", "got {t:?}");
    assert!(at(&t, "6.1.10") < at(&t, "7.4.60"), "got {t:?}");
    assert!(at(&t, "7.4.60") < at(&t, "7.4.75"), "got {t:?}");
    assert!(at(&t, "7.4.75") < at(&t, "7.3.86"), "got {t:?}");
    assert!(at(&t, "7.3.86") < at(&t, "8.2.30"), "got {t:?}");
    assert!(at(&t, "8.2.30") < at(&t, "8.4.55"), "got {t:?}");
    assert!(!t.contains(&"7.4.59".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.87".to_string()), "got {t:?}");
}

#[test]
fn nenijati_trace_has_nijam_and_no_upadha_guna_or_its_block() {
    // nij P laT P.B. `ati` is vowel-initial but ṅit (1.2.4). 7.3.86 declines
    // on its own, and 7.3.87 must not fire (*piti*).
    let (text, t) = cell_trace(
        "03.0012",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "nenijati", "got {t:?}");
    assert!(at(&t, "7.4.75") < at(&t, "7.1.4"), "got {t:?}");
    assert!(!t.contains(&"7.3.86".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.87".to_string()), "got {t:?}");
}

#[test]
fn nenijani_trace_is_nabhyastasyaci_barring_upadha_guna() {
    // nij P loT U.E. 3.4.92's `Ani` is pit but untagged. 7.3.87 fires and
    // bars 7.3.86, so the root keeps its `i`.
    let (text, t) = cell_trace(
        "03.0012",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "nenijAni", "got {t:?}");
    assert!(at(&t, "3.4.92") < at(&t, "7.4.75"), "got {t:?}");
    assert!(at(&t, "7.4.75") < at(&t, "7.3.87"), "got {t:?}");
    assert!(!t.contains(&"7.3.86".to_string()), "got {t:?}");
}

#[test]
fn anenijam_trace_is_at_then_nabhyastasyaci_on_the_tagged_am() {
    // nij P laN U.E. Laṅ's `am` keeps mip's Pit tag. 6.4.71's aṭ goes on, and
    // 7.3.87 blocks the guṇa.
    let (text, t) = cell_trace(
        "03.0012",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "anenijam", "got {t:?}");
    assert!(at(&t, "7.4.75") < at(&t, "6.4.71"), "got {t:?}");
    assert!(at(&t, "6.4.71") < at(&t, "7.3.87"), "got {t:?}");
    assert!(!t.contains(&"7.3.86".to_string()), "got {t:?}");
}

#[test]
fn nenikte_trace_has_nijam_and_no_guna() {
    // nij A laT P.E. Ātmanepada `te` is ṅit: no guṇa, no block. 8.2.30 then
    // 8.4.55 take j → g → k.
    let (text, t) = cell_trace(
        "03.0012",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "nenikte", "got {t:?}");
    assert!(at(&t, "7.4.75") < at(&t, "8.2.30"), "got {t:?}");
    assert!(at(&t, "8.2.30") < at(&t, "8.4.55"), "got {t:?}");
    assert!(!t.contains(&"7.3.86".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.87".to_string()), "got {t:?}");
}

#[test]
fn vevezwi_trace_is_upadha_guna_then_shtutva() {
    // viz P laT P.E. z + t: 8.4.41 retroflexes the `t`; 8.2.41 (before s only)
    // stays out.
    let (text, t) = cell_trace(
        "03.0014",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "vevezwi", "got {t:?}");
    assert!(at(&t, "7.4.75") < at(&t, "7.3.86"), "got {t:?}");
    assert!(at(&t, "7.3.86") < at(&t, "8.4.41"), "got {t:?}");
    assert!(!t.contains(&"8.2.41".to_string()), "got {t:?}");
}

#[test]
fn vevekzi_trace_is_shadhoh_kah_si_then_adesha_pratyayayoh() {
    // viz P laT M.E. z + s: 8.2.41 makes the root's `z` a `k`, then 8.3.59
    // makes the ending's `s` a `z`.
    let (text, t) = cell_trace(
        "03.0014",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "vevekzi", "got {t:?}");
    assert!(at(&t, "7.3.86") < at(&t, "8.2.41"), "got {t:?}");
    assert!(at(&t, "8.2.41") < at(&t, "8.3.59"), "got {t:?}");
}

#[test]
fn juhavani_trace_has_no_nabhyastasyaci() {
    // hu P loT U.E. A prior cell, but NOT the witness of 7.3.87's
    // *laghūpadhasya* clause: √hu's aṅga is vowel-final (hu → ho → hav), so
    // the clause declines it trivially. The ṛ-roots are the prior cells that
    // test that clause; see `biBarARi_trace_has_no_nabhyastasyaci` and
    // `nabhyastasyaci_is_credited_only_on_the_3e_3f_3f2_and_3f3_rows`.
    let (text, t) = cell_trace(
        "03.0001",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "juhavAni", "got {t:?}");
    assert!(at(&t, "3.4.92") < at(&t, "7.3.84"), "got {t:?}");
    assert!(!t.contains(&"7.3.87".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn biBarARi_trace_has_no_nabhyastasyaci() {
    // bhṛ P loT U.E. A prior ṛ-root cell. 7.3.87 reads the aṅga BEFORE
    // 7.3.84's guṇa, where `Bf` is vowel-final; read after it, `Bar` passes
    // the laghūpadha clause and the rule credits a no-op step (slice 3e's
    // final-review C1).
    let (text, t) = cell_trace(
        "03.0006",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "biBarARi", "got {t:?}");
    assert!(!t.contains(&"7.3.87".to_string()), "got {t:?}");
}

#[test]
fn nabhyastasyaci_is_credited_only_on_the_3e_3f_3f2_and_3f3_rows() {
    // Corpus-wide: every branch of every curated root x lakāra x pada x cell
    // whose log carries 7.3.87 belongs to √ṇij, √vij, √viṣ (3e), √kit, √tur,
    // √dhiṣ, √dhan (3f), √bhas (3f2) or √jan (3f3) — a credited no-op on
    // √dhan's, √bhas's and √jan's a-upadhā. A new rule that credits itself on
    // prior rows' traces (forms unchanged) fails here.
    const ALLOWED: [&str; 9] = [
        "03.0012", "03.0013", "03.0014", "03.0019", "03.0021", "03.0022", "03.0023", "03.0024",
        "03.0025",
    ];
    const CELLS: [(Purusha, Vacana); 9] = [
        (Purusha::Prathama, Vacana::Eka),
        (Purusha::Prathama, Vacana::Dvi),
        (Purusha::Prathama, Vacana::Bahu),
        (Purusha::Madhyama, Vacana::Eka),
        (Purusha::Madhyama, Vacana::Dvi),
        (Purusha::Madhyama, Vacana::Bahu),
        (Purusha::Uttama, Vacana::Eka),
        (Purusha::Uttama, Vacana::Dvi),
        (Purusha::Uttama, Vacana::Bahu),
    ];
    let mut credited = 0;
    for d in dhatus() {
        for lakara in [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin] {
            for &pada in d.padas() {
                for (purusha, vacana) in CELLS {
                    for p in derive(d, lakara, pada, purusha, vacana) {
                        if p.log.iter().any(|s| s.sutra == "7.3.87") {
                            credited += 1;
                            assert!(
                                ALLOWED.contains(&d.dhatupatha),
                                "7.3.87 credited on {} {lakara:?} {pada:?} {purusha:?} {vacana:?}: {}",
                                d.dhatupatha,
                                p.text()
                            );
                        }
                    }
                }
            }
        }
    }
    assert!(credited > 0, "the allowed rows no longer witness 7.3.87");
}

/// The derivation of `form` in one cell, for the pins on a non-declined
/// branch (cell_trace reads branch 0 only).
fn branch_trace(
    number: &str,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
    form: &str,
) -> Vec<String> {
    let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
    let p = derive(d, lakara, Pada::Parasmaipada, purusha, vacana)
        .into_iter()
        .find(|p| !p.blocked && p.text() == form)
        .unwrap_or_else(|| panic!("no branch derives {form}"));
    p.log.iter().map(|s| s.sutra.clone()).collect()
}

#[test]
#[allow(non_snake_case)]
fn acikeH_trace_is_jashtva_then_das_ca() {
    // kit P laN M.E. 7.3.86's guṇa, 8.2.23 eats the sip, 8.2.39 voices the
    // `t`, then 8.2.75 takes the `d` to ru. 8.2.73 never runs on it: that
    // rule wants an `s`.
    let t = branch_trace(
        "03.0021",
        Lakara::Lan,
        Purusha::Madhyama,
        Vacana::Eka,
        "acikeH",
    );
    assert!(at(&t, "7.3.86") < at(&t, "8.2.39"), "got {t:?}");
    assert!(at(&t, "8.2.39") < at(&t, "8.2.75"), "got {t:?}");
    assert!(!t.contains(&"8.2.73".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.56".to_string()), "got {t:?}");
}

#[test]
fn aciked_trace_has_no_ru() {
    // kit P laN P.E. Tip, not sip: 8.2.75 declines, and the cell forks only
    // on 8.4.56.
    let (text, t) = cell_trace(
        "03.0021",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "aciked", "got {t:?}");
    assert!(!t.contains(&"8.2.75".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.2.73".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn daDaMsi_trace_keeps_its_anusvara() {
    // Dan P laT M.E. 8.3.24 before `s`; 8.4.58 needs a following yay and
    // `s` is not one, so the anusvāra stays.
    let (text, t) = cell_trace(
        "03.0024",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "daDaMsi", "got {t:?}");
    assert!(t.contains(&"8.3.24".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.58".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn daDaMhi_trace_reaches_nas_capadantasya_before_h() {
    // Dan P loT M.E. The prep spec's open question: `h` is a jhal, and
    // 8.3.24 reaches it. 6.4.101 does not fire: `n` is not a jhal.
    let (text, t) = cell_trace(
        "03.0024",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "daDaMhi", "got {t:?}");
    assert!(t.contains(&"8.3.24".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.101".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn daDantaH_trace_is_the_anusvara_round_trip() {
    // Dan P laT P.D. 8.3.24 makes the `n` an anusvāra before `t`, and 8.4.58
    // turns it straight back: vidyut's trace too.
    let (text, t) = cell_trace(
        "03.0024",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "daDantaH", "got {t:?}");
    assert!(at(&t, "8.3.24") < at(&t, "8.4.58"), "got {t:?}");
}

#[test]
fn das_ca_is_credited_only_on_rudhadi_and_kit() {
    // 8.2.75 has no gaṇa test since slice 3f. Goldens ignore traces, so this
    // is what holds "no other root reaches it".
    let hits = credited("8.2.75");
    for (number, gana) in &hits {
        assert!(
            *gana == Gana::Rudhadi || *number == "03.0021",
            "8.2.75 credited on {number}"
        );
    }
    assert!(
        hits.iter().any(|(n, _)| *n == "03.0021"),
        "√kit no longer witnesses 8.2.75"
    );
}

#[test]
fn nas_capadantasya_is_credited_only_on_rudhadi_dhan_jan_and_curadi_roots() {
    // 8.3.24 admits juhotyādi since slice 3f; only √dhan and √jan (3f3) have
    // an `n` before a jhal there — √jan only before a pit ending (jajanti,
    // jajaMsi, jajantu), since 6.4.42 takes the `n` before a kṅit one.
    // Since slice 10e it also admits a curādi root's own `n` before a jhal:
    // 10c's √gandh and seven adanta roots, on every live branch.
    const CURADI: [&str; 8] = [
        "10.0204", "10.0433", "10.0460", "10.0467", "10.0471", "10.0472", "10.0473", "10.0474",
    ];
    let hits = credited("8.3.24");
    for (number, gana) in &hits {
        assert!(
            *gana == Gana::Rudhadi
                || *number == "03.0024"
                || *number == "03.0025"
                || CURADI.contains(number),
            "8.3.24 credited on {number}"
        );
    }
    let curadi: Vec<_> = hits.iter().filter(|(_, g)| *g == Gana::Curadi).collect();
    // √gandh's 36 ātmanepada branches and the seven ubhayapadī roots' 78 each.
    assert_eq!(curadi.len(), 36 + 7 * 78);
    for number in CURADI {
        assert!(
            curadi.iter().any(|(n, _)| *n == number),
            "{number} no longer witnesses 8.3.24"
        );
    }
    assert!(
        hits.iter().any(|(n, _)| *n == "03.0024"),
        "√dhan no longer witnesses 8.3.24"
    );
}

#[test]
fn bapsati_trace_elides_the_upadha_then_devoices_inside_the_anga() {
    // Bas P laT P.B. 6.4.100 before the ṅit `ati` (7.1.4's), then 8.4.55
    // turns the aṅga's own `B` to `p` before its `s`: a word-internal pair the
    // junction-only reading never saw.
    let (text, t) = cell_trace(
        "03.0019",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "bapsati", "got {t:?}");
    assert!(at(&t, "7.1.4") < at(&t, "6.4.100"), "got {t:?}");
    assert!(at(&t, "6.4.100") < at(&t, "8.4.55"), "got {t:?}");
    assert!(!t.contains(&"8.2.26".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn babDaH_trace_is_upadha_lopa_then_jhalo_jhali_then_jhashas_tathoh() {
    // Bas P laT P.D. Bs + tas: 8.2.26 elides the `s` between `B` and `t`,
    // 8.2.40 voices the `t` after the jhaṣ, and 8.4.53 takes `B` to `b`.
    let (text, t) = cell_trace(
        "03.0019",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "babDaH", "got {t:?}");
    assert!(at(&t, "6.4.100") < at(&t, "8.2.26"), "got {t:?}");
    assert!(at(&t, "8.2.26") < at(&t, "8.2.40"), "got {t:?}");
    assert!(at(&t, "8.2.40") < at(&t, "8.4.53"), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn babDi_trace_elides_the_upadha_before_her_dhih() {
    // Bas P loT M.E. 6.4.100 runs before 6.4.101 (vidyut's order), and the
    // `s` before `D` is 8.2.25's, not 8.2.26's.
    let (text, t) = cell_trace(
        "03.0019",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "babDi", "got {t:?}");
    assert!(at(&t, "6.4.100") < at(&t, "6.4.101"), "got {t:?}");
    assert!(at(&t, "6.4.101") < at(&t, "8.2.25"), "got {t:?}");
    assert!(!t.contains(&"8.2.26".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn baBastu_trace_keeps_the_upadha_before_a_pit() {
    // Bas P loT P.E., declined branch: `tu` is pit, so 6.4.100 declines.
    let (text, t) = cell_trace(
        "03.0019",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "baBastu", "got {t:?}");
    assert!(!t.contains(&"6.4.100".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn abaBat_trace_is_tipy_anasteh_then_car() {
    // Bas P laN P.E. 8.2.23 eats the tip, 8.2.73 writes the `d`, 8.4.56
    // devoices it. The first 8.2.73 outside rudhādi.
    let t = branch_trace(
        "03.0019",
        Lakara::Lan,
        Purusha::Prathama,
        Vacana::Eka,
        "abaBat",
    );
    assert!(at(&t, "8.2.23") < at(&t, "8.2.73"), "got {t:?}");
    assert!(at(&t, "8.2.73") < at(&t, "8.4.56"), "got {t:?}");
    assert!(!t.contains(&"6.4.100".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn abaBaH_trace_is_sipi_dhatoh_without_tipy_anasteh() {
    // Bas P laN M.E., 8.2.74's branch: the `s` goes to ru and 8.3.15 to a
    // visarga, and 8.2.73 finds no `s` left.
    let t = branch_trace(
        "03.0019",
        Lakara::Lan,
        Purusha::Madhyama,
        Vacana::Eka,
        "abaBaH",
    );
    assert!(at(&t, "8.2.74") < at(&t, "8.3.15"), "got {t:?}");
    assert!(!t.contains(&"8.2.73".to_string()), "got {t:?}");
}

#[test]
fn ghasibhasor_and_jhalo_jhali_are_credited_only_on_bhas() {
    // 6.4.100 is keyed to √bhas's row; 8.2.26 reads the whole word with no
    // key at all. Goldens ignore traces, so this is what holds "no other
    // root reaches 8.2.26".
    for sutra in ["6.4.100", "8.2.26"] {
        let hits = credited(sutra);
        assert!(!hits.is_empty(), "√bhas no longer witnesses {sutra}");
        for (number, _) in &hits {
            assert_eq!(*number, "03.0019", "{sutra} credited on {number}");
        }
    }
}

#[test]
fn tipy_anasteh_and_sipi_dhato_are_credited_only_on_rudhadi_and_bhas() {
    // 8.2.73 and 8.2.74 have no gaṇa test since slice 3f2.
    for sutra in ["8.2.73", "8.2.74"] {
        let hits = credited(sutra);
        for (number, gana) in &hits {
            assert!(
                *gana == Gana::Rudhadi || *number == "03.0019",
                "{sutra} credited on {number}"
            );
        }
        assert!(
            hits.iter().any(|(n, _)| *n == "03.0019"),
            "√bhas no longer witnesses {sutra}"
        );
    }
}

#[test]
fn khari_ca_off_bhas_is_credited_exactly_as_before_3f2() {
    // 8.4.55 reads the whole word since slice 3f2. A trace step carries no
    // term boundaries, so "it fires only at the junction off √bhas" is held
    // as the count of crediting branches outside √bhas, measured on `main`
    // before the widening: a new word-internal firing on a prior row would
    // add one. Update it only when a slice adds rows that credit 8.4.55.
    let off_bhas = credited("8.4.55")
        .into_iter()
        .filter(|(number, _)| *number != "03.0019")
        .count();
    assert_eq!(off_bhas, 455);
}

#[test]
#[allow(non_snake_case)]
fn jajYati_trace_is_upadha_lopa_then_shcutva() {
    // jan P laT P.B. 6.4.98 elides the upadhā before the ṅit `ati` (7.1.4's),
    // and 8.4.40's converse arm takes the `n` after the `j` to `Y`.
    let (text, t) = cell_trace(
        "03.0025",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "jajYati", "got {t:?}");
    assert!(at(&t, "7.1.4") < at(&t, "6.4.98"), "got {t:?}");
    assert!(at(&t, "6.4.98") < at(&t, "8.4.40"), "got {t:?}");
    assert!(!t.contains(&"6.4.42".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn ajajYuH_trace_is_at_then_upadha_lopa_then_shcutva() {
    // jan P laN P.B. The aṭ, then 6.4.98 before `us`, then 8.4.40.
    let (text, t) = cell_trace(
        "03.0025",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "ajajYuH", "got {t:?}");
    assert!(at(&t, "6.4.71") < at(&t, "6.4.98"), "got {t:?}");
    assert!(at(&t, "6.4.98") < at(&t, "8.4.40"), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn jajAtaH_trace_takes_the_a_without_the_abhyasta_rules() {
    // jan P laT P.D. 6.4.42 writes `jA` after 6.4.112 and 6.4.113 have run
    // on `jan` and declined (6.4.22), so neither elides nor raises the ā.
    let (text, t) = cell_trace(
        "03.0025",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "jajAtaH", "got {t:?}");
    assert!(t.contains(&"6.4.42".to_string()), "got {t:?}");
    for absent in ["6.4.112", "6.4.113", "6.4.98", "8.3.24"] {
        assert!(!t.contains(&absent.to_string()), "{absent}: got {t:?}");
    }
}

#[test]
#[allow(non_snake_case)]
fn jajAhi_trace_takes_the_a_and_keeps_hi() {
    // jan P loT M.E. `hi` is jhal-initial and ṅit: 6.4.42. The stem `jA` is
    // not jhal-final, so 6.4.101 hu-jhalbhyo her dhiḥ declines.
    let (text, t) = cell_trace(
        "03.0025",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "jajAhi", "got {t:?}");
    assert!(t.contains(&"6.4.42".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.101".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn jajAyAt_trace_forks_on_ye_vibhasa() {
    // jan P viDiliN P.E. 6.4.43's branch takes the ā before yāsuṭ, and no
    // ā-of-abhyasta rule touches it; the declined branch keeps the `n`.
    let t = branch_trace(
        "03.0025",
        Lakara::VidhiLin,
        Purusha::Prathama,
        Vacana::Eka,
        "jajAyAt",
    );
    assert!(at(&t, "7.2.79") < at(&t, "6.4.43"), "got {t:?}");
    for absent in ["6.4.112", "6.4.118", "6.4.42"] {
        assert!(!t.contains(&absent.to_string()), "{absent}: got {t:?}");
    }
    let t = branch_trace(
        "03.0025",
        Lakara::VidhiLin,
        Purusha::Prathama,
        Vacana::Eka,
        "jajanyAt",
    );
    assert!(!t.contains(&"6.4.43".to_string()), "got {t:?}");
}

#[test]
fn jajanti_trace_takes_none_of_the_jan_rules_before_a_pit() {
    // jan P laT P.E. tip is pit: no 6.4.98, 6.4.42 or 6.4.43, and the `n`
    // before `t` goes through 8.3.24 and back by 8.4.58.
    let (text, t) = cell_trace(
        "03.0025",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "jajanti", "got {t:?}");
    for absent in ["6.4.98", "6.4.42", "6.4.43"] {
        assert!(!t.contains(&absent.to_string()), "{absent}: got {t:?}");
    }
    assert!(at(&t, "8.3.24") < at(&t, "8.4.58"), "got {t:?}");
}

#[test]
fn gamahana_janasana_and_ye_vibhasa_are_credited_only_on_jan() {
    // All three are keyed by row number; 6.4.42 and 6.4.43's key holds √san
    // too, which must never take them (its follower is the vikaraṇa `u`).
    for sutra in ["6.4.98", "6.4.42", "6.4.43"] {
        let hits = credited(sutra);
        assert!(!hits.is_empty(), "√jan no longer witnesses {sutra}");
        for (number, _) in &hits {
            assert_eq!(*number, "03.0025", "{sutra} credited on {number}");
        }
    }
}

#[test]
fn shcutva_off_jan_is_credited_exactly_as_before_3f3() {
    // 8.4.40 has a converse arm (a ścu, then a stu) since slice 3f3. A trace
    // step carries no direction, so "the converse arm fires only on √jan" is
    // held as the credits off √jan: exactly √chid's and √chṛd's tuk (the
    // forward arm), 54 branches, measured on `main` before the widening. A
    // A new firing on a prior row's branch that does not already credit 8.4.40,
    // in either direction, changes the count; one on a √chid/√chṛd branch that
    // does would change its form, which the goldens hold. Slice 10e adds the
    // three ch-initial adanta curādi roots (`Cidra`, `Ceda`, `Cada`), whose laṅ
    // aṭ takes the same forward-arm tuk (acCidrayat).
    let hits = credited("8.4.40");
    assert!(
        hits.iter().any(|(n, _)| *n == "03.0025"),
        "√jan no longer witnesses 8.4.40"
    );
    let off_jan: Vec<_> = hits.iter().filter(|(n, _)| *n != "03.0025").collect();
    for (number, _) in &off_jan {
        assert!(
            ["07.0003", "07.0008", "10.0469", "10.0480", "10.0481"].contains(number),
            "8.4.40 credited on {number}"
        );
    }
    let adanta = off_jan.iter().filter(|(n, _)| n.starts_with("10.")).count();
    assert_eq!(off_jan.len() - adanta, 54);
    // Each ch-initial adanta root: laṅ's 18 cells plus its one 8.4.56 fork.
    assert_eq!(adanta, 3 * 19);
}
