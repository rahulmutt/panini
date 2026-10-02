//! curadi's ordered-trace witnesses. Helpers live in `crate::helpers`; the
//! module doc governing this suite is in `main.rs`.
//!
//! Every curādi trace opens with the sanādi stage — 3.1.25 ṇic, its 1.3.9,
//! 3.4.114, then 7.2.116 or 7.3.86 where the root's upadhā takes one, then
//! 3.1.32 — before the pada sūtra, where every other gaṇa's trace opens.
//! An ākusmīya root's trace opens one step earlier, with the gaṇasūtra
//! 10.0496, and has no pada sūtra at all. A jñapādi root's also opens one
//! step earlier, with the gaṇasūtra 10.0493, and 6.4.92 follows its
//! 7.2.116. An adanta root's has 6.4.48 after 3.4.114 and neither 7.2.116
//! nor 7.3.86; an ā-garvīya root's also opens with the gaṇasūtra 10.0497,
//! and has no pada sūtra.

use crate::helpers::{at, cell_trace, credited};
use panini_data::{
    AA_GARVIYA, AKUSMIYA, Gana, JNAPADI, Lakara, Pada, PadaAssignment, Purusha, Vacana, dhatus,
};
use panini_prakriya::derive;

#[test]
fn corayati_trace_is_nic_guna_sanadyanta_then_the_thematic_core() {
    // cur P laT P.E. 7.3.86 guṇates `cur` before ṇic, 3.1.32 makes `cori`
    // the dhātu, and only then does the pada sūtra run. 7.3.84 guṇates ṇic's
    // `i` before śap and 6.1.78 makes it `ay`.
    let (text, t) = cell_trace(
        "10.0001",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "corayati", "got {t:?}");
    assert_eq!(
        t,
        [
            "3.1.25", "1.3.9", "3.4.114", "7.3.86", "3.1.32", "1.3.78", "3.4.78", "1.3.9",
            "3.1.68", "1.3.9", "7.3.84", "6.1.78",
        ],
    );
}

#[test]
fn corayate_trace_credits_nicas_ca() {
    // cur A laT P.E. 1.3.74 sanctions the ātmanepada, where 1.3.78 stands in
    // corayati; neither 1.3.72 nor 1.3.78 is credited.
    let (text, t) = cell_trace(
        "10.0001",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "corayate", "got {t:?}");
    assert!(at(&t, "3.1.32") < at(&t, "1.3.74"), "got {t:?}");
    assert!(at(&t, "1.3.74") < at(&t, "3.4.78"), "got {t:?}");
    for absent in ["1.3.72", "1.3.78", "1.3.12"] {
        assert!(!t.contains(&absent.to_string()), "{absent}: got {t:?}");
    }
}

#[test]
#[allow(non_snake_case)]
fn lAqayati_trace_takes_ata_upadhayah_not_guna() {
    // laq P laT P.E. The `a` upadhā takes vṛddhi before ṇit ṇic; 7.3.86 has
    // nothing to guṇate.
    let (text, t) = cell_trace(
        "10.0010",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "lAqayati", "got {t:?}");
    assert!(at(&t, "3.4.114") < at(&t, "7.2.116"), "got {t:?}");
    assert!(at(&t, "7.2.116") < at(&t, "3.1.32"), "got {t:?}");
    assert!(!t.contains(&"7.3.86".to_string()), "got {t:?}");
}

#[test]
#[allow(non_snake_case)]
fn Bakzayati_and_BUzayati_traces_touch_no_upadha() {
    // Bakz and BUz have guru upadhās: ṇic goes on unchanged roots.
    for (number, form) in [("10.0033", "Bakzayati"), ("10.0255", "BUzayati")] {
        let (text, t) = cell_trace(
            number,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert_eq!(text, form, "got {t:?}");
        assert!(at(&t, "3.4.114") < at(&t, "3.1.32"), "got {t:?}");
        for absent in ["7.2.116", "7.3.86"] {
            assert!(
                !t.contains(&absent.to_string()),
                "{form} {absent}: got {t:?}"
            );
        }
    }
}

#[test]
#[allow(non_snake_case)]
fn BakzayARi_trace_takes_natva_across_the_stem() {
    // Bakz P loT U.E. The ṣ of the root reaches the ending's `n` across
    // `ayA` (8.4.2); √laḍ, with no trigger, keeps lAqayAni.
    let (text, t) = cell_trace(
        "10.0033",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "BakzayARi", "got {t:?}");
    assert!(at(&t, "6.1.101") < at(&t, "8.4.2"), "got {t:?}");
    let (text, t) = cell_trace(
        "10.0010",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert_eq!(text, "lAqayAni", "got {t:?}");
    assert!(!t.contains(&"8.4.2".to_string()), "got {t:?}");
}

#[test]
fn acorayad_trace_puts_the_augment_on_the_merged_anga() {
    // cur P laN P.E. The aṭ comes after 3.1.32, in front of the ṇijanta.
    let (text, t) = cell_trace(
        "10.0001",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "acorayad", "got {t:?}");
    assert!(at(&t, "3.1.32") < at(&t, "6.4.71"), "got {t:?}");
    assert!(at(&t, "6.4.71") < at(&t, "7.3.84"), "got {t:?}");
}

#[test]
fn the_sanadi_rules_and_nicas_ca_are_credited_only_on_curadi() {
    // Every curated branch whose log carries one of these belongs to a
    // `10.x` row. Goldens ignore traces, so this is what holds the new
    // stage inert on the 103 prior roots.
    for sutra in ["3.1.25", "3.4.114", "7.2.116", "3.1.32", "1.3.74"] {
        let hits = credited(sutra);
        assert!(!hits.is_empty(), "curādi no longer witnesses {sutra}");
        for (number, _) in &hits {
            assert!(number.starts_with("10."), "{sutra} credited on {number}");
        }
    }
}

#[test]
fn pugantalaghupadhasya_off_curadi_is_credited_exactly_as_before_10a() {
    // 7.3.86 gains a third entry in slice 10a, the sanādi one. A trace step
    // carries no entry, so "the guṇa-stage entries are untouched" is held as
    // the credit count off curādi: 384 branches, measured on `main` before
    // the slice. A new firing on a prior row, from any entry, changes it.
    let hits = credited("7.3.86");
    assert!(
        hits.iter().any(|(n, _)| *n == "10.0001"),
        "√cur no longer witnesses 7.3.86"
    );
    let off = hits.iter().filter(|(n, _)| !n.starts_with("10.")).count();
    assert_eq!(off, 384);
}

#[test]
fn cetayate_trace_opens_with_a_kusmad_and_credits_no_pada_sutra() {
    // cit A laT P.E. 10.0496 settles the pada before ṇic exists; 7.3.86
    // guṇates `cit` before ṇic, 3.1.32 makes `ceti` the dhātu, and 3.4.78
    // follows 3.1.32 directly — no 1.3.12, 1.3.74 or 1.3.78 between them.
    let (text, t) = cell_trace(
        "10.0192",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "cetayate", "got {t:?}");
    assert_eq!(
        t,
        [
            "10.0496", "3.1.25", "1.3.9", "3.4.114", "7.3.86", "3.1.32", "3.4.78", "1.2.4",
            "3.4.79", "3.1.68", "1.3.9", "7.3.84", "6.1.78",
        ],
    );
}

#[test]
#[allow(non_snake_case)]
fn varzayate_mAdayate_and_kusmayate_take_their_pre_nic_change_or_none() {
    // √vṛṣ: 7.3.86's ṛ arm (`vfz` → `varz`). √mad: 7.2.116, and no 7.3.86
    // (`a` is not ik). √kusm: guru upadhā, neither. Each opens with 10.0496.
    for (number, form, present, absent) in [
        ("10.0228", "varzayate", &["7.3.86"][..], &["7.2.116"][..]),
        ("10.0229", "mAdayate", &["7.2.116"][..], &["7.3.86"][..]),
        ("10.0236", "kusmayate", &[][..], &["7.2.116", "7.3.86"][..]),
    ] {
        let (text, t) = cell_trace(
            number,
            Lakara::Lat,
            Pada::Atmanepada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert_eq!(text, form, "got {t:?}");
        assert_eq!(t[0], "10.0496", "{form}: got {t:?}");
        for sutra in present {
            assert!(at(&t, "3.4.114") < at(&t, sutra), "{form}: got {t:?}");
            assert!(at(&t, sutra) < at(&t, "3.1.32"), "{form}: got {t:?}");
        }
        for sutra in absent {
            assert!(!t.contains(&sutra.to_string()), "{form} {sutra}: got {t:?}");
        }
    }
}

#[test]
fn an_akusmiya_roots_parasmaipada_is_blocked_by_a_kusmad_alone() {
    // Every parasmaipada cell of every curated ākusmīya row derives only
    // blocked branches, and the block is 10.0496's: nothing is recorded, so
    // no later rule ran on the branch. The rows are found by the positional
    // `AKUSMIYA` range, not by the curated `pada` column.
    let rows: Vec<_> = dhatus()
        .iter()
        .filter(|d| d.gana == Gana::Curadi && AKUSMIYA.contains(&d.dhatupatha))
        .collect();
    assert_eq!(rows.len(), 37, "curated ākusmīya rows");
    for d in rows {
        let number = d.dhatupatha;
        for lakara in [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin] {
            for purusha in [Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama] {
                for vacana in [Vacana::Eka, Vacana::Dvi, Vacana::Bahu] {
                    let ps = derive(d, lakara, Pada::Parasmaipada, purusha, vacana);
                    let cell = format!("{number} {lakara:?} {purusha:?} {vacana:?}");
                    assert!(!ps.is_empty(), "{cell}");
                    for p in &ps {
                        assert!(p.blocked, "{cell}: {}", p.text());
                        assert!(p.log.is_empty(), "{cell}: {:?}", p.log);
                    }
                }
            }
        }
    }
}

#[test]
fn a_kusmad_is_credited_on_exactly_the_akusmiya_cells() {
    // 10.0496 fires on every ātmanepada cell of the 37 curated ākusmīya
    // rows — 37 roots × 4 lakāras × 9 cells, one branch each — and nowhere
    // else: every credit's number lies in the positional `AKUSMIYA` range.
    // And 1.3.74 never reaches them: its credits stay on the 93 `Nic` rows,
    // read from the curated `pada` column (ten before slice 10e, listed
    // literally until then).
    let hits = credited("10.0496");
    assert_eq!(hits.len(), 1332);
    for (number, _) in &hits {
        assert!(AKUSMIYA.contains(number), "10.0496 credited on {number}");
    }
    let nic: Vec<&str> = dhatus()
        .iter()
        .filter(|d| d.gana == Gana::Curadi && d.pada == PadaAssignment::Nic)
        .map(|d| d.dhatupatha)
        .collect();
    assert_eq!(nic.len(), 93, "curated 1.3.74 rows");
    for (number, _) in credited("1.3.74") {
        assert!(nic.contains(&number), "1.3.74 credited on {number}");
    }
}

#[test]
fn jnapayati_trace_lengthens_then_shortens_the_upadha() {
    // jYap P laT P.E. 10.0493 credits the mit-tva before ṇic exists; 7.2.116
    // lengthens the `a` upadhā before ṇit ṇic (`jYAp`), and 6.4.92 mitāṃ
    // hrasvaḥ shortens it back (`jYap`) before 3.1.32 folds ṇic in.
    let (text, t) = cell_trace(
        "10.0118",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "jYapayati", "got {t:?}");
    assert_eq!(
        t,
        [
            "10.0493", "3.1.25", "1.3.9", "3.4.114", "7.2.116", "6.4.92", "3.1.32", "1.3.78",
            "3.4.78", "1.3.9", "3.1.68", "1.3.9", "7.3.84", "6.1.78",
        ],
    );
}

#[test]
#[allow(non_snake_case)]
fn syAmayate_and_SAmayate_keep_their_vrddhi() {
    // √syam and √śam are am-final curādi roots, but not jñapādi: neither
    // 10.0493 nor 6.4.92 reaches them, and 7.2.116's vṛddhi stands.
    for (number, form) in [("10.0216", "syAmayate"), ("10.0218", "SAmayate")] {
        let (text, t) = cell_trace(
            number,
            Lakara::Lat,
            Pada::Atmanepada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert_eq!(text, form, "got {t:?}");
        assert!(t.contains(&"7.2.116".to_string()), "{form}: got {t:?}");
        for absent in ["10.0493", "6.4.92"] {
            assert!(
                !t.contains(&absent.to_string()),
                "{form} {absent}: got {t:?}"
            );
        }
    }
}

#[test]
fn the_mit_rules_are_credited_on_exactly_the_jnapadi_cells() {
    // 10.0493 and 6.4.92 fire on every branch of the six curated jñapādi
    // rows — 42 parasmaipada and 36 ātmanepada branches each, 468 in all —
    // and nowhere else: every credit's number lies in the positional
    // `JNAPADI` range. Goldens ignore traces, so this is what holds both
    // rules inert on the 144 prior roots.
    for sutra in ["10.0493", "6.4.92"] {
        let hits = credited(sutra);
        assert_eq!(hits.len(), 468, "{sutra}");
        for (number, _) in &hits {
            assert!(JNAPADI.contains(number), "{sutra} credited on {number}");
        }
    }
}

/// The curated adanta rows: curādi, with an `a`-final code. No row curated
/// before slice 10e has one, so this is exactly 10e's ninety-two.
fn adanta_rows() -> Vec<&'static str> {
    dhatus()
        .iter()
        .filter(|d| d.gana == Gana::Curadi && d.code.ends_with('a'))
        .map(|d| d.dhatupatha)
        .collect()
}

#[test]
#[allow(non_snake_case)]
fn kaTayati_trace_deletes_the_a_and_skips_the_vrddhi() {
    // kaTa P laT P.E. 6.4.48 deletes the final `a` once 3.4.114 has made ṇic
    // ārdhadhātuka (`kaT`); by 1.1.57 the deleted `a` still stands, so
    // 7.2.116 does not lengthen `kaT`'s `a` (not *kATayati*).
    let (text, t) = cell_trace(
        "10.0389",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "kaTayati", "got {t:?}");
    assert_eq!(
        t,
        [
            "3.1.25", "1.3.9", "3.4.114", "6.4.48", "3.1.32", "1.3.78", "3.4.78", "1.3.9",
            "3.1.68", "1.3.9", "7.3.84", "6.1.78",
        ],
    );
    assert!(at(&t, "3.4.114") < at(&t, "6.4.48"), "got {t:?}");
    assert!(at(&t, "6.4.48") < at(&t, "3.1.32"), "got {t:?}");
}

#[test]
fn kuhayate_trace_opens_with_a_garvad() {
    // kuha A laT P.E. The gaṇasūtra 10.0497 settles the pada before ṇic
    // exists; 6.4.48 deletes the `a`, and 7.3.86 does not guṇate `kuh`'s
    // laghu `u` (not *kohayate*). No pada sūtra is credited.
    let (text, t) = cell_trace(
        "10.0443",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "kuhayate", "got {t:?}");
    assert_eq!(
        t,
        [
            "10.0497", "3.1.25", "1.3.9", "3.4.114", "6.4.48", "3.1.32", "3.4.78", "1.2.4",
            "3.4.79", "3.1.68", "1.3.9", "7.3.84", "6.1.78",
        ],
    );
}

#[test]
fn ato_lopa_is_credited_on_exactly_the_adanta_cells() {
    // 6.4.48 fires on every live branch of the 92 adanta rows — 83
    // ubhayapadī roots × 78 branches and 9 ā-garvīya roots × 36 — and
    // nowhere else. Neither 7.2.116 nor 7.3.86 is credited on any of them:
    // that is 1.1.57's block, held corpus-wide. Goldens ignore traces, so
    // this is also what holds 6.4.48 inert on the 150 prior roots.
    let adanta = adanta_rows();
    assert_eq!(adanta.len(), 92);
    let hits = credited("6.4.48");
    assert_eq!(hits.len(), 83 * 78 + 9 * 36);
    for (number, _) in &hits {
        assert!(adanta.contains(number), "6.4.48 credited on {number}");
    }
    for sutra in ["7.2.116", "7.3.86"] {
        for (number, _) in credited(sutra) {
            assert!(!adanta.contains(&number), "{sutra} credited on {number}");
        }
    }
}

#[test]
fn a_garvad_is_credited_on_exactly_the_a_garviya_cells() {
    // 10.0497 fires on every ātmanepada cell of the nine curated ā-garvīya
    // rows, one branch each, and nowhere else: every credit's number lies in
    // the positional `AA_GARVIYA` range. Their parasmaipada derives only
    // blocked branches, with nothing recorded: the block is 10.0497's.
    let hits = credited("10.0497");
    assert_eq!(hits.len(), 9 * 36);
    for (number, _) in &hits {
        assert!(AA_GARVIYA.contains(number), "10.0497 credited on {number}");
    }
    let rows: Vec<_> = dhatus()
        .iter()
        .filter(|d| d.gana == Gana::Curadi && AA_GARVIYA.contains(&d.dhatupatha))
        .collect();
    assert_eq!(rows.len(), 9, "curated ā-garvīya rows");
    for d in rows {
        for lakara in [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin] {
            for purusha in [Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama] {
                for vacana in [Vacana::Eka, Vacana::Dvi, Vacana::Bahu] {
                    let ps = derive(d, lakara, Pada::Parasmaipada, purusha, vacana);
                    let cell = format!("{} {lakara:?} {purusha:?} {vacana:?}", d.dhatupatha);
                    assert!(!ps.is_empty(), "{cell}");
                    for p in &ps {
                        assert!(p.blocked, "{cell}: {}", p.text());
                        assert!(p.log.is_empty(), "{cell}: {:?}", p.log);
                    }
                }
            }
        }
    }
}
