//! adadi's ordered-trace witnesses. Helpers live in
//! `crate::helpers`; the module doc governing this suite is in
//! `main.rs`.

use crate::helpers::trace_for;

#[test]
fn yati_trace_is_the_bare_luk_path() {
    // yA laṭ P 3sg: 3.1.68 inserts śap (1.3.9 its it-samjña), 2.4.72 luks it,
    // then guṇa (7.3.84) and 6.1.78 both decline (ā-final root, empty śap).
    assert_eq!(
        trace_for("yAti"),
        vec!["1.3.78", "3.4.78", "1.3.9", "3.1.68", "1.3.9", "2.4.72"]
    );
}

#[test]
fn yanti_trace_is_the_luk_plus_savarna_path() {
    // yA laṭ P 3pl: Ji → anti (7.1.3) after the luk, then root ā + a → ā
    // (6.1.101).
    assert_eq!(
        trace_for("yAnti"),
        vec![
            "1.3.78", "3.4.78", "1.2.4", "3.1.68", "1.3.9", "2.4.72", "7.1.3", "6.1.101"
        ]
    );
}

#[test]
fn yayuh_trace_is_the_adadi_us_junction_path() {
    // √yā adādi vidhiliṅ 3pl: Ji -> jus (3.4.108) -> us, śap inserted
    // (3.1.68) then luk'd (2.4.72), yāsuṭ's s elided (7.2.79) -> yAus, the ā
    // before us drops (6.1.96) -> yus, word-final s -> visarga (8.3.15):
    // yA + yuH -> yAyuH.
    assert_eq!(
        trace_for("yAyuH"),
        vec![
            "1.3.78", "3.4.78", "1.2.4", "3.4.108", "1.3.9", "3.4.103", "3.1.68", "1.3.9",
            "2.4.72", "7.2.79", "6.1.96", "8.3.15"
        ]
    );
}

#[test]
fn yayam_trace_is_the_adadi_am_junction_path() {
    // √yā adādi vidhiliṅ 1sg: mip -> am (3.4.101), yāsuṭ prefixed (3.4.103)
    // -> yAsam, śap inserted (3.1.68) then luk'd (2.4.72), yāsuṭ's s elided
    // (7.2.79) -> yAam, then yāsuṭ ā + ending a coalesce (6.1.101 new arm)
    // -> yAm: yA + yAm -> yAyAm. No 8.3.15 (ends in m).
    assert_eq!(
        trace_for("yAyAm"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.101", "3.4.103", "3.1.68", "1.3.9", "2.4.72",
            "7.2.79", "6.1.101"
        ]
    );
}

#[test]
fn atti_trace_ends_in_cartva() {
    // √ad adādi laṭ 3sg: tip's it-lopa (1.3.9) -> ti, śap inserted (3.1.68)
    // then it-lopa'd (1.3.9) -> a, then luk'd entirely (2.4.72) since adādi
    // roots take no visible vikaraṇa, leaving `ti` to attach straight onto
    // `ad`; cartva (8.4.55) then turns the aṅga-final `d` into `t` before the
    // khar `t` of the ending: ad + ti -> atti.
    assert_eq!(
        trace_for("atti"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.1.68", "1.3.9", "2.4.72", "8.4.55"
        ]
    );
}

#[test]
fn addhi_trace_uses_her_dhih_after_ato_heh_declines() {
    // √ad adādi loṭ 2sg: si -> hi (3.4.87). 6.4.105 ato heḥ (hi elides after
    // a short `a`) is structurally inapplicable here — the aṅga ends in the
    // consonant `d`, not a short `a` — so it declines and is not recorded;
    // 6.4.101 hujhalbhyo her dhiḥ then fires on the jhal-final aṅga instead,
    // turning `hi` into `Di`: ad + Di -> adDi.
    let t = trace_for("adDi");
    let i87 = t
        .iter()
        .position(|r| r == "3.4.87")
        .expect("3.4.87 present");
    let i101 = t
        .iter()
        .position(|r| r == "6.4.101")
        .expect("6.4.101 present");
    assert!(i87 < i101, "3.4.87 must precede 6.4.101");
    assert!(
        !t.contains(&"6.4.105".to_string()),
        "6.4.105 declines, not recorded"
    );
}

#[test]
fn adat_trace_a_augment_precedes_and_blocks_cartva() {
    // √ad adādi laṅ 3sg: ti -> t (3.4.100 itaś ca), śap inserted (3.1.68)
    // then it-lopa'd (1.3.9) -> a, luk'd (2.4.72), āṭ-augmented (6.4.72,
    // ad -> Aad), 7.3.100 adaḥ sarvezām prefixes `a` onto the consonant
    // ending (t -> at) so the word stays vowel-final (8.2.23 declines and
    // cartva 8.4.55 never sees a khar after the aṅga's `d`), and 6.1.90 āṭaś
    // ca vṛddhi-fuses the augment `A` with the root's `a` (Aad -> Ad):
    // Ad + at -> Adat. `Adat` now names the forked branch: 8.2.39
    // obligatorily voices the pada-final `t` to `d`, and 8.4.56 optionally
    // devoices it back.
    assert_eq!(
        trace_for("Adat"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.100", "3.1.68", "1.3.9", "2.4.72", "6.4.72",
            "7.3.100", "6.1.90", "8.2.39", "8.4.56"
        ]
    );
}

#[test]
fn asate_trace_uses_anatah_not_jhontah() {
    // √ās adādi ātmanepada laṭ 3pl: Ja → Je (3.4.79) → luk of śap (2.4.72) →
    // 7.1.5 ātmanepadeṣv anataḥ replaces the leading J with `at` (Je → ate),
    // and 7.1.3 declines (ending no longer starts with J): As + ate -> Asate.
    assert_eq!(
        trace_for("Asate"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.68", "1.3.9", "2.4.72", "7.1.5"
        ]
    );
    // 7.1.3 must NOT appear — 7.1.5 is its apavāda here.
    assert!(!trace_for("Asate").contains(&"7.1.3".to_string()));
}

#[test]
fn adhve_trace_ends_in_dhi_ca() {
    // √ās adādi ātmanepada laṭ 2pl: Dvam → Dve (3.4.79), śap luk'd (2.4.72),
    // then 8.2.25 dhi ca ELIDES the aṅga-final `s` before the `Dh` of Dve:
    // As + Dve -> A + Dve -> ADve. Slice 5d pinned *AdDve here via 8.4.53
    // jaśtva, before 8.2.25 existed to bleed it on this branch; 8.4.53 itself
    // is back in the grammar (restored in rudhādi 7a's Task 6), it simply
    // never gets a chance to fire once 8.2.25 has already removed the `s`.
    assert_eq!(
        trace_for("ADve"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.68", "1.3.9", "2.4.72", "8.2.25"
        ]
    );
    // Neither junction rule may fire: 8.2.25 already elided the aṅga-final
    // `s` above, so 8.4.53 jaśtva has nothing left to voice, and cartva
    // (8.4.55) is the voiceless junction, which a `Dh` never triggers.
    assert!(!trace_for("ADve").contains(&"8.4.53".to_string()));
    assert!(!trace_for("ADve").contains(&"8.4.55".to_string()));
}

#[test]
fn vadhve_trace_is_the_second_dhi_ca_witness() {
    // √vas adādi ātmanepada laṭ 2pl — `vaDve` is the cell the
    // Siddhāntakaumudī's adādi paradigm gives (vidyut-prakriya pins it at
    // `kaumudi_44::sk_2440`), not the sūtra's own example. Same ordered path
    // as ADve on a consonant-initial root: the aṅga-final `s` is elided
    // before the `Dh` of Dve, giving vaDve.
    assert_eq!(
        trace_for("vaDve"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.68", "1.3.9", "2.4.72", "8.2.25"
        ]
    );
    // Non-Dh-initial cells of the same root reach no junction rule at all:
    // the `s` of vaste survives, so cartva must not appear either.
    assert!(!trace_for("vaste").contains(&"8.4.55".to_string()));
    assert!(!trace_for("vaste").contains(&"8.2.25".to_string()));
}

#[test]
fn ase_lot_uttama_eka_trace_ends_in_atas_ca() {
    // √ās adādi ātmanepada loṭ uttama eka: iṭ → i (1.3.9) → e (3.4.79) → E
    // (3.4.93) → AE (3.4.92 āṭ). The śap is inserted (3.1.68) then luk'd
    // (2.4.72), so — unlike thematic laBE — 6.1.101 never widens śap a + āṭ A
    // (there is no śap). The āṭ A leads the ending as `AE`; 6.1.90 āṭaś ca's
    // athematic ending arm vṛddhi-fuses it to a single E: As + E -> AsE.
    assert_eq!(
        trace_for("AsE"),
        vec![
            "1.3.12", "3.4.78", "1.3.9", "3.4.85", "3.4.79", "3.4.93", "3.4.92", "3.1.68", "1.3.9",
            "2.4.72", "6.1.90"
        ]
    );
}

#[test]
fn asita_vidhilin_trace_ends_in_vali_lopa() {
    // √ās adādi ātmanepada vidhiliṅ prathama eka: ta → sIyta (3.4.102), śap
    // luk'd (2.4.72), yāsuṭ's s elided (7.2.79) -> Iyta. Unlike thematic
    // laBeta, there is no śap a for 6.1.87 to coalesce with the I, so the I is
    // retained as the stem's long vowel (āsī-) and 6.1.66 lopo vyor vali
    // elides the y before the val consonant `t`: As + Ita -> AsIta.
    let t = trace_for("AsIta");
    assert_eq!(
        t,
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.102", "3.1.68", "1.3.9", "2.4.72", "7.2.79", "6.1.66"
        ]
    );
    // 7.2.79 (salopa) must precede 6.1.66; 6.1.87 must NOT fire (no śap a).
    let i79 = t
        .iter()
        .position(|r| r == "7.2.79")
        .expect("7.2.79 present");
    let i66 = t
        .iter()
        .position(|r| r == "6.1.66")
        .expect("6.1.66 present");
    assert!(i79 < i66, "7.2.79 must precede 6.1.66");
    assert!(!t.contains(&"6.1.87".to_string()), "6.1.87 must not fire");
}

#[test]
fn shete_trace_is_the_minimal_shing_guna_path() {
    // √śī adādi Ā laṭ 3sg: ta → te (3.4.79), śap inserted (3.1.68) then luk'd
    // (2.4.72), and 7.4.21 guṇates the aṅga despite the ṅit ending: SI + te
    // → Se + te → Sete. 7.3.84 does NOT appear — `Se` is not ik-final.
    let t = trace_for("Sete");
    assert_eq!(
        t,
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.68", "1.3.9", "2.4.72", "7.4.21"
        ]
    );
    assert!(!t.contains(&"7.3.84".to_string()), "7.3.84 must not fire");
}

#[test]
fn sherate_trace_is_the_rut_path() {
    // √śī adādi Ā laṭ 3pl: Ja → Je (3.4.79) → luk (2.4.72) → 7.1.5 replaces
    // the leading J with `at` (Je → ate) → 7.1.6 prefixes ruṭ's `r` (ate →
    // rate) → 7.4.21 guṇates: Se + r + ate → Serate.
    let t = trace_for("Serate");
    assert_eq!(
        t,
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.68", "1.3.9", "2.4.72", "7.1.5", "7.1.6",
            "7.4.21"
        ]
    );
    // 7.1.5 must precede 7.1.6 — the ruṭ attaches to the `at` 7.1.5 makes.
    let i5 = t.iter().position(|r| r == "7.1.5").expect("7.1.5 present");
    let i6 = t.iter().position(|r| r == "7.1.6").expect("7.1.6 present");
    assert!(i5 < i6, "7.1.5 must precede 7.1.6");
    assert!(!t.contains(&"7.1.3".to_string()), "7.1.3 is bled by 7.1.5");
}

#[test]
fn sheshe_trace_ends_in_shatva() {
    // √śī adādi Ā laṭ 2sg: TAs → se (3.4.80), śap luk'd (2.4.72), 7.4.21
    // guṇates, and 8.3.59 retroflexes the ending's `s` after the aṅga's `e`:
    // Se + se → Se + ze → Seze. The engine's first ṣatva.
    assert_eq!(
        trace_for("Seze"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.80", "3.1.68", "1.3.9", "2.4.72", "7.4.21", "8.3.59"
        ]
    );
}

#[test]
fn shayita_trace_is_the_shing_vali_lopa_path() {
    // √śī adādi Ā vidhiliṅ 3sg: AsIta's path with two rules added. yāsuṭ's s
    // elided (7.2.79) → SI + Iy + ta, 7.4.21 guṇates → Se + Iy + ta, 6.1.78
    // turns the aṅga's `e` into `ay` before the ending's vowel → Say + Iy +
    // ta, and 6.1.66 elides the y before the val `t` → SayIta.
    let t = trace_for("SayIta");
    assert_eq!(
        t,
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.102", "3.1.68", "1.3.9", "2.4.72", "7.2.79",
            "7.4.21", "6.1.78", "6.1.66"
        ]
    );
    // 6.1.78 must precede 6.1.66: the ay-ādeśa happens while the y is still
    // there to be elided afterwards.
    let i78 = t
        .iter()
        .position(|r| r == "6.1.78")
        .expect("6.1.78 present");
    let i66 = t
        .iter()
        .position(|r| r == "6.1.66")
        .expect("6.1.66 present");
    assert!(i78 < i66, "6.1.78 must precede 6.1.66");
    assert!(!t.contains(&"6.1.87".to_string()), "6.1.87 must not fire");
}

#[test]
fn shayai_trace_is_the_shing_atas_ca_path() {
    // √śī adādi Ā loṭ uttama eka: AsE's path plus guṇa and the ay-ādeśa. iṭ →
    // i (1.3.9) → e (3.4.79) → E (3.4.93) → AE (3.4.92 āṭ); śap luk'd
    // (2.4.72); 7.4.21 guṇates → Se; 6.1.78 turns `e` → `ay` before the
    // ending's A → Say; 6.1.90's athematic arm fuses the āṭ A with E → SayE.
    let t = trace_for("SayE");
    assert_eq!(
        t,
        vec![
            "1.3.12", "3.4.78", "1.3.9", "3.4.85", "3.4.79", "3.4.93", "3.4.92", "3.1.68", "1.3.9",
            "2.4.72", "7.4.21", "6.1.78", "6.1.90"
        ]
    );
    assert!(!t.contains(&"6.1.101".to_string()), "no śap to widen");
}

#[test]
fn ayuh_trace_is_the_shakatayana_jus_path() {
    // yā laṅ prathama bahu, Śākaṭāyana branch. 3.4.111 replaces jhi with
    // jus before 7.1.3 can turn it into `ant`; 6.1.96's junction arm then
    // elides the aṅga's ā across the boundary (ayA + us -> ay + us), and
    // 8.3.15 gives the visarga.
    let t = trace_for("ayuH");
    assert!(t.contains(&"3.4.111".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.1.3".to_string()), "got {t:?}");
    assert!(t.contains(&"6.1.96".to_string()), "got {t:?}");
    assert!(t.contains(&"8.3.15".to_string()), "got {t:?}");
}
