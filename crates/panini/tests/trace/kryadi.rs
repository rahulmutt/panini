//! kryadi's ordered-trace witnesses. Helpers live in
//! `crate::helpers`; the module doc governing this suite is in
//! `main.rs`.

use crate::helpers::trace_for;

#[test]
fn klishnati_trace_is_the_shna_pit_path() {
    // kliS kryAdi lat 3sg: tip is PIT, so neither 6.4.112 nor 6.4.113 fires
    // and SnA's A survives -- this is the baseline the whole paradigm splits
    // away from. 7.3.86 does NOT appear: SnA is apit, the second 1.2.4 makes
    // it Nit, and 1.1.5 blocks guNa of kliS's laghu upadha `i`.
    let t = trace_for("kliSnAti");
    assert!(t.contains(&"3.1.81".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.112".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.86".to_string()), "got {t:?}");
}

#[test]
fn klishnitah_trace_takes_i_halyaghoh() {
    // tas is apit -> Nit (1.2.4) and consonant-initial -> 6.4.113 gives nI.
    let t = trace_for("kliSnItaH");
    assert!(t.contains(&"1.2.4".to_string()), "got {t:?}");
    assert!(t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.112".to_string()), "got {t:?}");
}

#[test]
fn klishnanti_trace_takes_shnabhyastayor_atah() {
    // Ji -> anti (7.1.3) must precede 6.4.112, or the ending is not yet
    // vowel-initial and the A survives as *kliSnAnti.
    let t = trace_for("kliSnanti");
    let i713 = t.iter().position(|r| r == "7.1.3").expect("7.1.3 present");
    let i6412 = t
        .iter()
        .position(|r| r == "6.4.112")
        .expect("6.4.112 present");
    assert!(i713 < i6412, "7.1.3 must precede 6.4.112: {t:?}");
    assert!(!t.contains(&"6.4.113".to_string()), "got {t:?}");
}

#[test]
fn klishana_trace_is_shanac_then_ato_heh() {
    // lot 2sg after a consonant-final root: 3.1.83 replaces SnA by SAnac,
    // then the existing 6.4.105 ato heH drops the hi. 6.4.113 must NOT
    // appear -- it would have given *kliSnIhi -- and 7.3.86 must not fire.
    let t = trace_for("kliSAna");
    let i3183 = t
        .iter()
        .position(|r| r == "3.1.83")
        .expect("3.1.83 present");
    let i64105 = t
        .iter()
        .position(|r| r == "6.4.105")
        .expect("6.4.105 present");
    assert!(i3183 < i64105, "3.1.83 must precede 6.4.105: {t:?}");
    assert!(!t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.86".to_string()), "got {t:?}");
}

#[test]
fn ashnat_trace_takes_the_vowel_initial_anga_augment() {
    // aS is vowel-initial, so lan takes AT (6.4.72) rather than aT, and
    // 6.1.90 AwaS ca vRddhi-fuses the augment A with the root's a into A:
    // ASnAt. (This is the same mechanism the "ad" precedent test
    // `adat_trace_a_augment_precedes_and_blocks_cartva` documents for a+ad ->
    // Ad -- augment+root-vowel fusion is 6.1.90, not the SHAP/ENDING-junction
    // 6.1.101.)
    let t = trace_for("ASnAt");
    assert!(t.contains(&"6.4.72".to_string()), "got {t:?}");
    assert!(t.contains(&"6.1.90".to_string()), "got {t:?}");
    assert!(t.contains(&"3.1.81".to_string()), "got {t:?}");
}

#[test]
fn mushnati_trace_takes_adjacent_natva() {
    // z directly precedes SnA's n -> 8.4.1, not 8.4.2.
    let t = trace_for("muzRAti");
    assert!(t.contains(&"8.4.1".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.2".to_string()), "got {t:?}");
}

#[test]
fn vrinati_trace_takes_intervening_natva() {
    // r, then the aw vowel I, then n -> 8.4.2, not 8.4.1.
    let t = trace_for("vrIRAti");
    assert!(t.contains(&"8.4.2".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.1".to_string()), "got {t:?}");
}

#[test]
fn mushana_trace_is_shanac_plus_intervening_natva() {
    // lot 2sg: 3.1.83 gives Ana, 6.4.105 drops the hi, and 8.4.2 then
    // retroflexes across the A. Both rules in one derivation.
    let t = trace_for("muzARa");
    assert!(t.contains(&"3.1.83".to_string()), "got {t:?}");
    assert!(t.contains(&"6.4.105".to_string()), "got {t:?}");
    assert!(t.contains(&"8.4.2".to_string()), "got {t:?}");
}

#[test]
fn vrinite_trace_is_the_atmanepada_shna_path() {
    // vf + SnA + te: te is apit -> Nit (1.2.4), consonant-initial -> 6.4.113,
    // and the r-vowel triggers 8.4.1.
    let t = trace_for("vfRIte");
    assert!(t.contains(&"3.1.81".to_string()), "got {t:?}");
    assert!(t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(t.contains(&"8.4.1".to_string()), "got {t:?}");
}

#[test]
fn vrinishva_trace_reaches_the_existing_shatva() {
    // lot 2sg atmanepada: 6.4.113 gives nI, and the existing 8.3.59
    // AdeSapratyayayoH then retroflexes sva's s after that I -> vfRIzva.
    let t = trace_for("vfRIzva");
    assert!(t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(t.contains(&"8.3.59".to_string()), "got {t:?}");
}

#[test]
fn klishnitat_trace_pins_tatan_above_3_1_83() {
    // kliś loṭ madhyama eka, tātaṅ branch. 7.1.35 replaces `hi` BEFORE
    // 3.1.83 can see it, so śnā is never reshaped to śāna; 6.4.113 then
    // gives nI. This is the pin that fails if 7.1.35 is ever moved down —
    // the absence of 3.1.83 is the assertion, not the surface form, because
    // the wrong order still produces a plausible-looking word.
    let t = trace_for("kliSnItAt");
    assert!(t.contains(&"7.1.35".to_string()), "got {t:?}");
    assert!(!t.contains(&"3.1.83".to_string()), "got {t:?}");
    assert!(t.contains(&"6.4.113".to_string()), "got {t:?}");
    assert!(t.contains(&"8.4.56".to_string()), "got {t:?}");
}
