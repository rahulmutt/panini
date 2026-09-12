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
