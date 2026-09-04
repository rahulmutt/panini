//! tanadi's ordered-trace witnesses. Helpers live in
//! `crate::helpers`; the module doc governing this suite is in
//! `main.rs`.

use crate::helpers::{at, cell_trace};
use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};
use panini_prakriya::derive;

#[test]
fn tanoti_trace_is_the_ardhadhatuka_u_core() {
    // tan laT P.E. 3.1.79 inserts the bare u; the SECOND 7.3.84 guṇates
    // it before pit ti. Load-bearing absences: no 1.2.4 anywhere (ti is
    // pit; the u is ārdhadhātuka and the second 1.2.4's Sarvadhatuka
    // guard excludes it — tanoti exists BECAUSE that guard does), and no
    // second 1.3.9 (the bare u has no anubandha to strip, unlike śnu).
    let (text, t) = cell_trace(
        "08.0001",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "tanoti", "got {t:?}");
    assert!(at(&t, "3.1.79") < at(&t, "7.3.84"), "got {t:?}");
    assert!(!t.contains(&"1.2.4".to_string()), "got {t:?}");
    assert!(!t.contains(&"3.1.68".to_string()), "got {t:?}");
}

#[test]
fn tanvanti_trace_credits_6_1_77_not_the_shnu_rules() {
    // tan laT P.B. u -> v before the vowel-initial ṅit ending is 6.1.77
    // iko yaR aci — the sūtra vidyut credits — and must NOT be 6.4.87
    // (names hu/śnu) or 6.4.77 (uvaṅ): a widened-guard regression on
    // either would derive the same surface by the wrong rule, which is
    // exactly what a trace pin exists to catch.
    let (text, t) = cell_trace(
        "08.0001",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert_eq!(text, "tanvanti", "got {t:?}");
    assert!(t.contains(&"6.1.77".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.87".to_string()), "got {t:?}");
    assert!(!t.contains(&"6.4.77".to_string()), "got {t:?}");
}

#[test]
fn tanute_trace_opens_with_1_3_72() {
    // tan laT ātmanepada P.E: the svarita-it row reaches 1.3.72 (not
    // 1.3.12, not 1.3.66), and the u then survives unguṇated behind the
    // ṅit te.
    let (text, t) = cell_trace(
        "08.0001",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "tanute", "got {t:?}");
    assert!(at(&t, "1.3.72") < at(&t, "3.1.79"), "got {t:?}");
    assert!(!t.contains(&"1.3.66".to_string()), "got {t:?}");
}

#[test]
fn trnoti_forks_on_7_3_86_and_only_there() {
    // tfR laT P.E: the Kaumudī-2547.1 optionality, keyed by the Pāṇinian
    // id. Branch 0 (declined) is the golden tfRoti with NO 7.3.86 in its
    // log; the other live branch is tarRoti WITH it. Exactly two live
    // branches — the vikalpa arm is the only fork in this cell.
    let d = dhatus().iter().find(|d| d.dhatupatha == "08.0006").unwrap();
    let branches: Vec<_> = derive(
        d,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    )
    .into_iter()
    .filter(|p| !p.blocked)
    .collect();
    let texts: Vec<String> = branches.iter().map(|p| p.text()).collect();
    assert_eq!(texts, vec!["tfRoti", "tarRoti"], "declined first");
    assert!(!branches[0].log.iter().any(|s| s.sutra == "7.3.86"));
    assert!(branches[1].log.iter().any(|s| s.sutra == "7.3.86"));
}

#[test]
fn rnu_and_arnuhi_split_the_asamyogapurva_test() {
    // fR loT M.E: one cell, two stems, opposite 6.4.106 verdicts. The
    // declined stem fRu (R after the vowel f) luks hi; the guṇa stem
    // arRu (rR conjunct) keeps it — and 6.4.101 must not touch that hi
    // (the sound before it is u, not jhal). The widened helper's whole
    // truth table in one cell.
    let d = dhatus().iter().find(|d| d.dhatupatha == "08.0005").unwrap();
    let branches: Vec<_> = derive(
        d,
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    )
    .into_iter()
    .filter(|p| !p.blocked)
    .collect();
    let texts: Vec<String> = branches.iter().map(|p| p.text()).collect();
    assert!(texts.contains(&"fRu".to_string()), "got {texts:?}");
    assert!(texts.contains(&"arRuhi".to_string()), "got {texts:?}");
    let frnu = branches.iter().find(|p| p.text() == "fRu").unwrap();
    assert!(frnu.log.iter().any(|s| s.sutra == "6.4.106"));
    let arnuhi = branches.iter().find(|p| p.text() == "arRuhi").unwrap();
    assert!(!arnuhi.log.iter().any(|s| s.sutra == "6.4.106"));
    assert!(!arnuhi.log.iter().any(|s| s.sutra == "6.4.101"));
}

#[test]
fn arnot_trace_reaches_the_f_arm_of_vrddhi_and_the_fork_converges() {
    // fR laN P.E. 6.4.72 puts the āṭ on, 6.1.90 contracts A+f to Ar —
    // the FIRST golden derivation through vrddhi_of's f arm (the 7d
    // √und/Onad story, repeated) — and the 7.3.86 fork CONVERGES under
    // it: A+fR and A+arR are both ArR, so run_pipeline's collapse must
    // leave exactly two live branches (the 8.2.39/8.4.56 pair), neither
    // carrying 7.3.86.
    let d = dhatus().iter().find(|d| d.dhatupatha == "08.0005").unwrap();
    let branches: Vec<_> = derive(
        d,
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    )
    .into_iter()
    .filter(|p| !p.blocked)
    .collect();
    let texts: Vec<String> = branches.iter().map(|p| p.text()).collect();
    assert_eq!(
        texts,
        vec!["ArRod", "ArRot"],
        "collapsed to the 8.4.56 pair"
    );
    for b in &branches {
        assert!(
            !b.log.iter().any(|s| s.sutra == "7.3.86"),
            "a surviving branch may not carry the converged fork's rule"
        );
        assert!(
            at(
                &b.log.iter().map(|s| s.sutra.clone()).collect::<Vec<_>>(),
                "6.4.72"
            ) < at(
                &b.log.iter().map(|s| s.sutra.clone()).collect::<Vec<_>>(),
                "6.1.90"
            )
        );
    }
}

#[test]
fn tanu_trace_is_the_hi_luk() {
    // tan loT M.E, declined branch: 6.4.106 luks hi behind the widened
    // helper — the tanādi twin of svādi's hinu.
    let (text, t) = cell_trace(
        "08.0001",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "tanu", "got {t:?}");
    assert!(t.contains(&"6.4.106".to_string()), "got {t:?}");
}

#[test]
fn kurutah_trace_orders_guna_before_ata_ut() {
    // kf laT P.D: 7.3.84 makes kar against the ārdhadhātuka u, then
    // 6.4.110 makes kur against the ṅit tas. Reversed, 6.4.110's `kar`
    // guard never matches and kurutaH is underivable.
    let (text, t) = cell_trace(
        "08.0010",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "kurutaH", "got {t:?}");
    assert!(at(&t, "7.3.84") < at(&t, "6.4.110"), "got {t:?}");
}

#[test]
fn kurmah_is_nitya_no_6_4_107_fork() {
    // kf laT U.B: 6.4.108 does what 6.4.107 would only offer, so the
    // cell must not fork at all — one live branch, 6.4.108 in its log,
    // 6.4.107 in no branch's.
    let d = dhatus().iter().find(|d| d.dhatupatha == "08.0010").unwrap();
    let branches: Vec<_> = derive(
        d,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Bahu,
    )
    .into_iter()
    .filter(|p| !p.blocked)
    .collect();
    assert_eq!(branches.len(), 1, "6.4.108 is nitya");
    assert_eq!(branches[0].text(), "kurmaH");
    assert!(branches[0].log.iter().any(|s| s.sutra == "6.4.108"));
    assert!(!branches[0].log.iter().any(|s| s.sutra == "6.4.107"));
}

#[test]
fn kuryat_trace_takes_ye_ca() {
    // kf viDiliN P.E, declined branch: 6.4.110 then 6.4.109.
    let (text, t) = cell_trace(
        "08.0010",
        Lakara::VidhiLin,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "kuryAd", "got {t:?}");
    assert!(at(&t, "6.4.110") < at(&t, "6.4.109"), "got {t:?}");
}

#[test]
fn karoti_runs_7_3_84_twice_and_the_specials_not_at_all() {
    // kf laT P.E: root guṇa against the u, vikaraṇa guṇa against pit ti
    // — the double application the pipeline's two 7.3.84 entries exist
    // for — and every 6.4.10x special declines on the pit ending.
    let (text, t) = cell_trace(
        "08.0010",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "karoti", "got {t:?}");
    assert_eq!(t.iter().filter(|s| *s == "7.3.84").count(), 2, "got {t:?}");
    for absent in ["6.4.110", "6.4.108", "6.4.109"] {
        assert!(!t.contains(&absent.to_string()), "{absent} in {t:?}");
    }
}
