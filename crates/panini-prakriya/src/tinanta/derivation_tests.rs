//! Whole-derivation tests: cases that assert a surface form, a trace, or an
//! invariant over the pipeline as a whole, rather than one rule's guard.
//!
//! Per-rule guard tests live beside their rule in the stage files. If a test
//! you are adding calls `(rule.apply)(&mut p)` on a hand-built prakriya, it
//! belongs there, not here.

use super::*;
// `cartva_of` is otherwise only used by `tripadi.rs`; imported by path since
// `mod.rs` re-exports nothing from `sound`.
use crate::tinanta::sound::cartva_of;
use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};

/// Unwrap a derivation that must not have forked.
///
/// Every single-form helper below goes through this, so a cell that gains
/// an unexpected optional-rule branch fails loudly at the unit-test level
/// rather than silently having its first branch read.
pub(super) fn sole(branches: Vec<Prakriya>) -> Prakriya {
    assert_eq!(
        branches.len(),
        1,
        "expected exactly one derivation, got {}: {:?}",
        branches.len(),
        branches.iter().map(|p| p.text()).collect::<Vec<_>>()
    );
    branches.into_iter().next().unwrap()
}

/// Unwrap a derivation that IS expected to fork, asserting the branch count.
///
/// `sole` stays the default so an unexpected fork fails loudly; this is its
/// counterpart for cells an optional rule legitimately forks. It returns
/// branch 0 — the declined derivation, i.e. what the pipeline produces with
/// no optional rule applied — and still fails if the count is not exactly
/// what the caller expects, so an over-firing optional rule cannot hide here.
pub(super) fn declined(branches: Vec<Prakriya>, expected: usize) -> Prakriya {
    assert_eq!(
        branches.len(),
        expected,
        "expected {expected} derivations, got {}: {:?}",
        branches.len(),
        branches.iter().map(|p| p.text()).collect::<Vec<_>>()
    );
    branches.into_iter().next().unwrap()
}

pub(super) fn form(number: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
    sole(derive(d, Lakara::Lat, Pada::Parasmaipada, pu, va)).text()
}

// `pub(super)` — the narrowest visibility that lets `mod.rs` re-export it
// as `crate::tinanta::form_g`; `anga.rs` and `tripadi.rs` import it by that
// stable path. The other helpers carry the same visibility so a stage test
// module can import any of them the same way.
pub(super) fn form_g(number: &str, la: Lakara, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
    sole(derive(d, la, d.pada.padas()[0], pu, va)).text()
}

/// `form_g` for a cell an optional rule forks: same lookup, `declined`
/// instead of `sole`.
pub(super) fn form_g_forked(
    number: &str,
    la: Lakara,
    pu: Purusha,
    va: Vacana,
    branches: usize,
) -> String {
    let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
    declined(derive(d, la, d.pada.padas()[0], pu, va), branches).text()
}

pub(super) fn lin_form(number: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
    sole(derive(d, Lakara::VidhiLin, Pada::Parasmaipada, pu, va)).text()
}

pub(super) fn lat_a_form(number: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
    sole(derive(d, Lakara::Lat, Pada::Atmanepada, pu, va)).text()
}

pub(super) fn lot_a_form(number: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
    sole(derive(d, Lakara::Lot, Pada::Atmanepada, pu, va)).text()
}

pub(super) fn lin_a_form(number: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
    sole(derive(d, Lakara::VidhiLin, Pada::Atmanepada, pu, va)).text()
}

/// The ordered rule list IS the grammar this crate implements, so its
/// sequence is pinned verbatim, not merely by the per-derivation traces
/// in `crates/panini/tests/trace.rs` (which only pin order along the
/// paths representative forms happen to take).
///
/// If you add a rule, add its id here in position. If this test fails
/// after a refactor that was supposed to move code without changing it,
/// the refactor reordered the grammar — fix the refactor, not this list.
///
/// 1.2.4 appears twice, deliberately: once tagging apit ātmanepada
/// endings, once tagging the apit vikaraṇa after 3.1.68 inserts it.
///
/// 7.3.84 also appears twice, for the analogous reason (1.4.13): the aṅga
/// is affix-relative, so the sūtra has two occasions in one derivation —
/// once with respect to the vikaraṇa (guṇates the root), once with
/// respect to the tiṅ ending (guṇates the vikaraṇa). See its second entry
/// in `tinanta/guna.rs` for the full rationale.
///
/// 7.3.92 sits between the two 7.3.84 applications: in sūtra order, and
/// necessarily above 6.1.87 in `tinanta/adesha.rs`, which coalesces the
/// āgama it inserts.
///
/// 8.3.13 sits BELOW 8.4.41, against sūtra order: the second ḍh it needs
/// is 8.4.41's own output. See its comment in `tinanta/tripadi.rs`.
#[test]
fn tinanta_rule_order_is_pinned() {
    let expected = [
        "1.3.12", "1.3.72", "1.3.78", "3.4.78", "1.3.9", "1.2.4", "3.4.85", "3.4.108", "3.4.105",
        "3.4.106", "3.4.101", "3.4.99", "3.4.87", "3.4.89", "3.4.86", "3.4.100", "3.4.80",
        "3.4.79", "3.4.91", "3.4.93", "3.4.90", "3.4.92", "3.4.103", "3.4.102", "7.1.35", "3.1.69",
        "3.1.73", "3.1.77", "3.1.78", "3.1.81", "3.1.68", "2.4.72", "3.4.111", "3.1.83", "1.2.4",
        "6.4.71", "6.4.72", "7.3.100", "7.1.5", "7.1.6", "7.1.3", "7.2.79", "7.2.80", "7.2.81",
        "6.4.23", "7.4.21", "7.3.84", "7.3.86", "7.3.92", "7.3.84", "6.4.87", "6.4.77", "6.1.78",
        "7.3.101", "6.4.112", "6.4.113", "6.1.101", "6.1.96", "6.1.90", "6.1.97", "6.1.87",
        "6.1.66", "6.4.105", "6.4.106", "6.4.107", "6.4.101", "6.4.111", "8.2.77", "8.2.23",
        "8.2.25", "8.2.30", "8.2.31", "8.2.39", "8.2.40", "8.2.41", "8.2.74", "8.2.75", "8.2.73",
        "8.3.15", "8.3.24", "8.3.59", "8.4.41", "8.3.13", "8.4.53", "8.4.55", "8.4.1", "8.4.2",
        "8.4.58", "8.4.65", "8.4.56",
    ];
    let actual: Vec<&str> = rules().map(|r| r.id).collect();
    assert_eq!(actual, expected);
}

/// Optionality is grammar: a rule is *vikalpa* because its sūtra says
/// *anyatarasyām* / *vā* / *vibhāṣā*, and a mis-set flag silently doubles
/// every branch the rule touches — a failure no surface-form golden
/// necessarily catches, because both branches are plausible Sanskrit.
/// Pin the whole set by id, not just the count.
#[test]
fn exactly_the_pinned_vikalpa_rules_are_optional() {
    let actual: Vec<&str> = rules().filter(|r| r.vikalpa).map(|r| r.id).collect();
    let expected = [
        "7.1.35", "3.4.111", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56",
    ];
    assert_eq!(actual, expected);
}

#[test]
fn divadi_tudadi_present_third_singular() {
    // Guṇa blocked by 1.1.5 (śyan/śa are ṅit): kup→kupyati NOT kopyati,
    // tud→tudati NOT todati, juṣ→juṣate NOT joṣate.
    assert_eq!(
        form_g("04.0091", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "naSyati"
    );
    assert_eq!(
        form_g("04.0146", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "kupyati"
    );
    assert_eq!(
        form_g("04.0073", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "manyate"
    );
    assert_eq!(
        form_g("04.0069", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "yuDyate"
    );
    assert_eq!(
        form_g("04.0067", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "vidyate"
    );
    assert_eq!(
        form_g("06.0001", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "tudati"
    );
    assert_eq!(
        form_g("06.0092", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "liKati"
    );
    assert_eq!(
        form_g("06.0160", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "viSati"
    );
    assert_eq!(
        form_g("06.0008", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "juzate"
    );
    assert_eq!(
        form_g("06.0009", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "vijate"
    );
    assert_eq!(
        form_g("06.0131", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "gurate"
    );
}

#[test]
fn divadi_tudadi_vowel_sandhi_cells() {
    // 3rd-singular alone never exercises the SHAP-final-vowel sandhi
    // rules (6.1.97/6.1.101/6.1.90/6.1.87/7.3.101): its `ti` ending is
    // consonant-initial. These cells pin the fix that generalized those
    // rules from "SHAP.text == a single a" to "SHAP.text ends in a" so
    // śyan's two-character `ya` residue (not just śa's/śap's `a`)
    // coalesces correctly with a following vowel.
    //
    // 3rd plural (6.1.97 para-rūpa: śyan/śa `a` + Ji→`anti`'s `a` → `a`,
    // not `aa`). naS/tud/juz avoid √div, whose 8.2.77 lengthening is
    // Task 5's job — its short-vowel `divyanti` is already correct here.
    assert_eq!(
        form_g("04.0091", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "naSyanti"
    );
    assert_eq!(
        form_g("06.0001", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "tudanti"
    );
    assert_eq!(
        form_g("06.0008", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "juzante"
    );
    // 1st singular (7.3.101 ato dIrgho yaYi: śyan/śa `a` + `mi` → `Ami`).
    assert_eq!(
        form_g("06.0001", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "tudAmi"
    );
    // Ātmanepada uttama-eka (6.1.97 a+e para-rūpa: śyan `ya` + `e` → `ye`).
    assert_eq!(
        form_g("04.0073", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "manye"
    );
    // 7.2.81 Ato NitaH: ātmanepada dual Ate→iyte, then coalesced.
    assert_eq!(
        form_g("04.0073", Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
        "manyete"
    );
    // 7.2.80 ato yeyaH: vidhiliṅ yA→iy after śyan's `ya`.
    assert_eq!(
        form_g_forked(
            "04.0146",
            Lakara::VidhiLin,
            Purusha::Prathama,
            Vacana::Eka,
            2
        ),
        "kupyed"
    );
    // 6.4.105 ato heH: imperative hi-elision after śyan's `ya`.
    assert_eq!(
        form_g_forked("04.0091", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "naSya"
    );
}

#[test]
fn div_lengthens_before_syan() {
    assert_eq!(
        form_g("04.0001", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "dIvyati"
    );
    // laṅ: augment does not disturb the upadhā i.
    assert_eq!(
        form_g_forked("04.0001", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2),
        "adIvyad"
    );
}

#[test]
fn adadi_luk_present_no_junction_cells() {
    // ā-final adādi roots: śap is luk'd (2.4.72), the ending attaches to
    // the root directly. These cells need only the luk (no ā+a junction).
    assert_eq!(
        form_g("02.0044", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "yAti"
    );
    assert_eq!(
        form_g("02.0044", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
        "yAsi"
    );
    assert_eq!(
        form_g("02.0044", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "yAmi"
    );
    // laṅ: aṭ-augment (yā is consonant-initial) → ayā; ending attaches.
    assert_eq!(
        form_g_forked("02.0044", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2),
        "ayAd"
    );
    // loṭ 2sg: hi does NOT elide after ā (6.4.105 needs short a) → yāhi.
    assert_eq!(
        form_g_forked("02.0044", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "yAhi"
    );
}

#[test]
fn adadi_root_final_a_coalesces_with_vowel_endings() {
    // ā + a(nti) → ā : yānti (laṭ 3pl), yAntu (loṭ 3pl), ayAn (laṅ 3pl).
    assert_eq!(
        form_g("02.0044", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "yAnti"
    );
    assert_eq!(
        form_g("02.0044", Lakara::Lot, Purusha::Prathama, Vacana::Bahu),
        "yAntu"
    );
    // Now forks: 3.4.111 (Task 4) adds the Śākaṭāyana jus branch (ayuH,
    // pinned in `paradigm.rs`'s ALTERNATES). Branch 0 is still the declined
    // derivation.
    assert_eq!(
        form_g_forked("02.0044", Lakara::Lan, Purusha::Prathama, Vacana::Bahu, 2),
        "ayAn"
    );
    // ā + A(ṭ) → ā : loṭ uttama-eka takes āṭ (yA + Ani → yAni).
    assert_eq!(
        form_g("02.0044", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
        "yAni"
    );
}

#[test]
fn adadi_vidhilin_derives_the_yas_yuh_reduction() {
    // adādi × vidhiliṅ now
    // derives through the full pipeline, running the yāsuṭ chain plus
    // the 6.1.96 / 6.1.101 junction reductions, for every cell and pada.
    //
    // prathama eka is the one cell in this loop whose ending is a bare
    // pada-final `t` (yAyAt/vAyAt): 8.2.39/8.4.56 fork it into two branches,
    // same as every other jhal-final cell this task touches. `declined`
    // asserts the branch count directly here (rather than routing through
    // `form_g_forked`) because this loop checks blocked/log/text-non-empty
    // invariants across every cell, not one pinned surface form.
    for number in ["02.0044", "02.0045"] {
        let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
        for pu in [Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama] {
            for va in [Vacana::Eka, Vacana::Dvi, Vacana::Bahu] {
                let expected = if pu == Purusha::Prathama && va == Vacana::Eka {
                    2
                } else {
                    1
                };
                let p = declined(
                    derive(d, Lakara::VidhiLin, d.pada.padas()[0], pu, va),
                    expected,
                );
                assert!(!p.blocked, "{} vidhiliṅ {pu:?} {va:?} was blocked", d.code);
                assert!(!p.log.is_empty(), "{} vidhiliṅ ran no rules", d.code);
                assert!(
                    !p.text().is_empty(),
                    "{} vidhiliṅ {pu:?} {va:?} is empty",
                    d.code
                );
            }
        }
    }
    assert_eq!(
        form_g("02.0044", Lakara::VidhiLin, Purusha::Prathama, Vacana::Bahu),
        "yAyuH"
    );
    assert_eq!(
        form_g("02.0044", Lakara::VidhiLin, Purusha::Uttama, Vacana::Eka),
        "yAyAm"
    );
}

#[test]
fn cartva_turns_d_to_t_before_khar() {
    // √ad laṭ: 3sg atti (d+t), 2sg atsi (d+s), 2pl atTa (d+T).
    assert_eq!(
        form_g("02.0001", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "atti"
    );
    assert_eq!(
        form_g("02.0001", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
        "atsi"
    );
    assert_eq!(
        form_g("02.0001", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
        "atTa"
    );
    // Not before a non-khar (m/v) or a vowel: admi, adanti stay.
    assert_eq!(
        form_g("02.0001", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "admi"
    );
    assert_eq!(
        form_g("02.0001", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "adanti"
    );
}

#[test]
fn her_dhih_gives_addhi_for_consonant_root() {
    // √ad loṭ 2sg: 3.4.87 si→hi, 6.4.105 declines (d, not short a),
    // 6.4.101 hi→Di → adDi.
    assert_eq!(
        form_g_forked("02.0001", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "adDi"
    );
    // Thematic root unaffected: √bhū loṭ 2sg is Bava (hi luk'd by 6.4.105).
    assert_eq!(
        form_g_forked("01.0001", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "Bava"
    );
}

#[test]
fn adadi_lan_singular_a_augment() {
    // √ad laṅ 3sg Adad, 2sg AdaH — the inserted `a` blocks the saṃyogānta
    // collapse (Adt/Ads → Ad) and cartva (d now before `a`, not a khar).
    assert_eq!(
        form_g_forked("02.0001", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2),
        "Adad"
    );
    assert_eq!(
        form_g("02.0001", Lakara::Lan, Purusha::Madhyama, Vacana::Eka),
        "AdaH"
    );
    // Dual/plural keep the direct junction (multi-char endings, no a-augment):
    // cartva gives Attam/AttAm; 1sg Adam untouched.
    assert_eq!(
        form_g("02.0001", Lakara::Lan, Purusha::Madhyama, Vacana::Dvi),
        "Attam"
    );
    assert_eq!(
        form_g("02.0001", Lakara::Lan, Purusha::Prathama, Vacana::Dvi),
        "AttAm"
    );
    assert_eq!(
        form_g("02.0001", Lakara::Lan, Purusha::Uttama, Vacana::Eka),
        "Adam"
    );
}

#[test]
fn bhu_3sg_is_bhavati() {
    assert_eq!(form("01.0001", Purusha::Prathama, Vacana::Eka), "Bavati");
}
#[test]
fn bhu_1pl_is_bhavamah() {
    assert_eq!(form("01.0001", Purusha::Uttama, Vacana::Bahu), "BavAmaH");
}
#[test]
fn smr_3sg_is_smarati() {
    assert_eq!(form("01.1082", Purusha::Prathama, Vacana::Eka), "smarati");
}
#[test]
fn pat_3du_is_patatah() {
    assert_eq!(form("01.0381", Purusha::Prathama, Vacana::Dvi), "paWataH");
}
#[test]
fn bhu_3pl_is_bhavanti() {
    assert_eq!(form("01.0001", Purusha::Prathama, Vacana::Bahu), "Bavanti");
}
#[test]
fn shap_is_pit_and_bhvadi_guna_survives() {
    // Regression guard for Task 3: adding the guṇa-block mechanism must
    // not disturb bhvādi. śap is pit, so 7.3.84 still fires for BU.
    assert_eq!(form("01.0001", Purusha::Prathama, Vacana::Eka), "Bavati");
    let d = dhatus().iter().find(|d| d.dhatupatha == "01.0862").unwrap();
    // vṛt uses 7.3.86 (laghūpadhā guṇa) before śap (pit) → vartate.
    assert_eq!(
        sole(derive(
            d,
            Lakara::Lat,
            Pada::Atmanepada,
            Purusha::Prathama,
            Vacana::Eka
        ))
        .text(),
        "vartate"
    );
}

#[test]
fn ji_3sg_is_jayati() {
    // "ji" ends in `i`; 7.3.84 guNa gives "je", then 6.1.78 eco'yavAyAvaH
    // (the `e` arm, distinct from the `o` arm already exercised by BU)
    // turns je+a into jaya, yielding "jayati".
    assert_eq!(form("01.0642", Purusha::Prathama, Vacana::Eka), "jayati");
}

#[test]
fn trace_is_recorded() {
    let d = dhatus().iter().find(|d| d.dhatupatha == "01.0001").unwrap();
    let p = sole(derive(
        d,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    ));
    assert!(p.log.iter().any(|s| s.sutra == "3.1.68"));
    assert!(p.log.iter().any(|s| s.sutra == "7.3.84"));
    assert!(!p.log.is_empty());
}

// --- Fix 2: the sutra-name hard gate --------------------------------
//
// AGENTS.md declares that sutra ids/names in traces must match the cited
// reference. Nothing previously asserted that a `RuleStep.name` emitted
// into a user-facing trace actually equals the `name` field of the
// `TINANTA_RULES` entry with the matching `id`. `Rule.name` itself is
// write-only dead data: what a user sees is `RuleStep.name`, populated
// solely from the string literal passed to `p.record(...)` at each call
// site (there are two for id "1.3.9": this rule's own body, and
// `run_it_samjna` in `it_samjna.rs`, called from 3.1.68's body — both
// currently pass the literal "tasya lopaH"). Comparing every recorded
// step's name against `TINANTA_RULES` by id, over real derivations,
// catches either call site drifting from `Rule.name` without having to
// special-case which call site fired.

/// Drive every (root, lakara, purusha, vacana) cell over the curated
/// roots and all four lakaras/nine cells, and assert every recorded
/// `RuleStep.name` matches the `TINANTA_RULES` entry for its `sutra` id.
#[test]
fn recorded_step_names_match_tinanta_rules_for_every_id() {
    let lakaras = [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin];
    let purushas = [Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama];
    let vacanas = [Vacana::Eka, Vacana::Dvi, Vacana::Bahu];

    let mut steps_checked = 0usize;
    for d in dhatus() {
        for &lakara in &lakaras {
            for &purusha in &purushas {
                for &vacana in &vacanas {
                    for p in derive(d, lakara, d.pada.padas()[0], purusha, vacana) {
                        for step in &p.log {
                            let rule = rules().find(|r| r.id == step.sutra).unwrap_or_else(|| {
                                panic!(
                                    "recorded step cites sutra id {:?} which is not in TINANTA_RULES \
                                     (dhatu {}, {lakara:?} {purusha:?} {vacana:?})",
                                    step.sutra, d.code
                                )
                            });
                            assert_eq!(
                                step.name, rule.name,
                                "RuleStep.name for sutra {} (dhatu {}, {lakara:?} {purusha:?} {vacana:?}) \
                                 is {:?} but TINANTA_RULES[id={:?}].name is {:?} -- a record() call site \
                                 has drifted from the Rule.name field",
                                step.sutra, d.code, step.name, rule.id, rule.name
                            );
                            steps_checked += 1;
                        }
                    }
                }
            }
        }
    }
    assert!(
        steps_checked > 0,
        "sanity: the derivations above should have recorded at least one RuleStep"
    );
}

/// SLP1 validity of every sutra name in `TINANTA_RULES`: none may contain
/// one of the digraphs (`gh`, `jh`, `dh`, `kh`, `th`, `bh`, `ph`, `ch`)
/// that are always wrong inside SLP1 (SLP1 is one-char-per-phoneme; those
/// aspirates are `G`, `J`, `D`, `K`, `T`, `B`, `P`, `C`). This is the
/// error class that produced the non-SLP1 names swept by hand earlier on
/// this branch (see commit 892cfa4).
///
/// This check is intentionally narrow: it flags exactly these eight
/// lowercase digraphs and nothing else. A legitimate SLP1 name may
/// contain a genuine consonant-then-`h`-vowel sequence (e.g. `hy`, as in
/// "ser hyapic ca") or the avagraha apostrophe (as in "Jo'ntaH"); this
/// check does not touch either. It also cannot detect a name whose
/// *content* is wrong (a mistranscribed sutra) as long as it avoids these
/// eight substrings -- it only catches the specific historical error
/// class of "wrote an aspirate as two ASCII letters instead of SLP1's one
/// capital letter."
#[test]
fn sutra_names_contain_no_forbidden_slp1_digraphs() {
    const FORBIDDEN: [&str; 8] = ["gh", "jh", "dh", "kh", "th", "bh", "ph", "ch"];
    for rule in rules() {
        for bad in FORBIDDEN {
            assert!(
                !rule.name.contains(bad),
                "rule {} name {:?} contains forbidden non-SLP1 digraph {:?}",
                rule.id,
                rule.name,
                bad
            );
        }
    }
}

#[test]
fn bhu_vidhilin_all_nine_cells() {
    // Prathama eka forks (8.2.39/8.4.56, same as `kupyet`/`Bavet` elsewhere in
    // this file), so it is pinned separately with `form_g_forked` rather than
    // through the single-branch loop below.
    assert_eq!(
        form_g_forked(
            "01.0001",
            Lakara::VidhiLin,
            Purusha::Prathama,
            Vacana::Eka,
            2
        ),
        "Baved"
    );
    for (pu, va, want) in [
        (Purusha::Prathama, Vacana::Dvi, "BavetAm"),
        (Purusha::Prathama, Vacana::Bahu, "BaveyuH"),
        (Purusha::Madhyama, Vacana::Eka, "BaveH"),
        (Purusha::Madhyama, Vacana::Dvi, "Bavetam"),
        (Purusha::Madhyama, Vacana::Bahu, "Baveta"),
        (Purusha::Uttama, Vacana::Eka, "Baveyam"),
        (Purusha::Uttama, Vacana::Dvi, "Baveva"),
        (Purusha::Uttama, Vacana::Bahu, "Bavema"),
    ] {
        assert_eq!(lin_form("01.0001", pu, va), want, "{pu:?} {va:?}");
    }
}

#[test]
fn pada_sanction_blocks_wrong_pada_derivations() {
    // 1.3.12/1.3.78: derivation is the source of truth for pada. A
    // wrong-pada derive must not silently produce a surface form.
    let labh = dhatus().iter().find(|d| d.dhatupatha == "01.1130").unwrap();
    let p = sole(derive(
        labh,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    ));
    assert!(p.blocked, "atmanepadin root + parasmaipada must block");
    assert_eq!(p.text(), "laB", "no rule may run after the block");
    assert!(p.log.is_empty(), "a blocked derivation records nothing");

    let bhu = dhatus().iter().find(|d| d.dhatupatha == "01.0001").unwrap();
    let p = sole(derive(
        bhu,
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    ));
    assert!(p.blocked, "parasmaipada root + atmanepada must block");
}

#[test]
fn pada_sanction_records_the_sanctioning_sutra() {
    let bhu = dhatus().iter().find(|d| d.dhatupatha == "01.0001").unwrap();
    let p = sole(derive(
        bhu,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    ));
    assert_eq!(p.log.first().unwrap().sutra, "1.3.78");

    let labh = dhatus().iter().find(|d| d.dhatupatha == "01.1130").unwrap();
    let p = sole(derive(
        labh,
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    ));
    assert_eq!(p.log.first().unwrap().sutra, "1.3.12");
}

#[test]
fn rudh_derives_in_both_padas() {
    // The slice's witness. √rudh is the first root DHATUS marks
    // PadaAssignment::Ubhayapada, so BOTH pada cells must derive: neither
    // is a wrong-pada request, and neither may block. 1.3.78 sanctions the
    // parasmaipada cell, 1.3.72 the ātmanepada one, and the trace says which.
    //
    // This is also the end-to-end pin on `derive`'s Ubhayapada → Ubhayapadin
    // tagging arm: with that arm missing, the ātmanepada cell would block.
    let rudh = dhatus().iter().find(|d| d.dhatupatha == "07.0001").unwrap();

    let p = sole(derive(
        rudh,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    ));
    assert!(!p.blocked, "an ubhayapadī root must derive in parasmaipada");
    assert_eq!(p.text(), "ruRadDi");
    assert_eq!(p.log.first().unwrap().sutra, "1.3.78");

    // 8.4.65 jharo jhari savarṇe optionally elides the `d` of rundDe, so
    // the ātmanepada cell forks in two; branch 0 is the declined reading.
    let branches = derive(
        rudh,
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(
        branches.iter().map(|p| p.text()).collect::<Vec<_>>(),
        vec!["rundDe", "runDe"]
    );
    for b in &branches {
        assert!(!b.blocked, "an ubhayapadī root must derive in ātmanepada");
        assert_eq!(b.log.first().unwrap().sutra, "1.3.72");
    }
}

#[test]
fn indh_is_atmanepada_only_despite_its_nit() {
    // This test is what protects the whole data-model choice, so it is worth
    // stating why. √indh's upadeśa is `YiinDI~\`: it carries a ñi, and 1.3.72
    // svaritaYitaH reads ñit — so a tag named for 1.3.72's *marker* would
    // have to be true on √indh, and √indh would silently grow a parasmaipada
    // column. It must not: the anudātta `~\` on top of the ñi settles pada by
    // 1.3.12, and vidyut-prakriya derives √indh ātmanepada-only.
    //
    // Tag::Ubhayapadin is therefore named for the RESIDUE — 1.3.72's
    // condition holds *and* 1.3.12's does not — which is exactly why √indh
    // does not carry it. If the tag ever drifts back toward "has a ñi
    // marker", this test is the one that fails.
    let indh = dhatus().iter().find(|d| d.dhatupatha == "07.0011").unwrap();
    let p = sole(derive(
        indh,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    ));
    assert!(
        p.blocked,
        "√indh is ñit but not ubhayapadī: 1.3.12 settles it"
    );
    assert!(p.log.is_empty(), "a blocked derivation records nothing");

    for b in derive(
        indh,
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    ) {
        assert!(!b.blocked);
        assert_eq!(b.log.first().unwrap().sutra, "1.3.12");
    }
}

#[test]
fn parasmaipada_only_root_still_blocks_atmanepada() {
    // 1.3.72's guard declines on an aṅga with no Ubhayapadin tag, leaving
    // 1.3.78's ātmanepada arm to block as it always did. Widening that arm
    // to decline unconditionally would surface *BavatE here.
    let bhu = dhatus().iter().find(|d| d.dhatupatha == "01.0001").unwrap();
    let p = sole(derive(
        bhu,
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    ));
    assert!(p.blocked, "√bhū carries no Ubhayapadin tag");
    assert!(p.log.is_empty());
}

#[test]
fn atmanepada_only_root_still_blocks_parasmaipada() {
    // The mirror image, inside the gaṇa 1.3.72 just opened: √khid is
    // rudhādi and ātmanepadin, so 1.3.72 declines on its guard and 1.3.12
    // blocks the parasmaipada request. Being a rudhādi root is not what
    // makes √rudh ubhayapadī.
    let khid = dhatus().iter().find(|d| d.dhatupatha == "07.0012").unwrap();
    let p = sole(derive(
        khid,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    ));
    assert!(p.blocked, "√khid carries Atmanepadin, not Ubhayapadin");
    assert!(p.log.is_empty());
}

#[test]
fn labh_lat_atmanepada_all_nine_cells() {
    let expected = [
        (Purusha::Prathama, Vacana::Eka, "laBate"),
        (Purusha::Prathama, Vacana::Dvi, "laBete"),
        (Purusha::Prathama, Vacana::Bahu, "laBante"),
        (Purusha::Madhyama, Vacana::Eka, "laBase"),
        (Purusha::Madhyama, Vacana::Dvi, "laBeTe"),
        (Purusha::Madhyama, Vacana::Bahu, "laBaDve"),
        (Purusha::Uttama, Vacana::Eka, "laBe"),
        (Purusha::Uttama, Vacana::Dvi, "laBAvahe"),
        (Purusha::Uttama, Vacana::Bahu, "laBAmahe"),
    ];
    for (pu, va, form) in expected {
        assert_eq!(lat_a_form("01.1130", pu, va), form, "{pu:?} {va:?}");
    }
}

#[test]
fn labh_lot_atmanepada_all_nine_cells() {
    let expected = [
        (Purusha::Prathama, Vacana::Eka, "laBatAm"),
        (Purusha::Prathama, Vacana::Dvi, "laBetAm"),
        (Purusha::Prathama, Vacana::Bahu, "laBantAm"),
        (Purusha::Madhyama, Vacana::Eka, "laBasva"),
        (Purusha::Madhyama, Vacana::Dvi, "laBeTAm"),
        (Purusha::Madhyama, Vacana::Bahu, "laBaDvam"),
        (Purusha::Uttama, Vacana::Eka, "laBE"),
        (Purusha::Uttama, Vacana::Dvi, "laBAvahE"),
        (Purusha::Uttama, Vacana::Bahu, "laBAmahE"),
    ];
    for (pu, va, form) in expected {
        assert_eq!(lot_a_form("01.1130", pu, va), form, "{pu:?} {va:?}");
    }
}

#[test]
fn savabhyam_vamau_preempts_am_etah() {
    // 3.4.91 (se→sva, Dve→Dvam) is the apavāda ordered before 3.4.90:
    // reversed, se would become sAm and Dve DvAm.
    assert_eq!(
        lot_a_form("01.1130", Purusha::Madhyama, Vacana::Eka),
        "laBasva"
    );
    assert_eq!(
        lot_a_form("01.1130", Purusha::Madhyama, Vacana::Bahu),
        "laBaDvam"
    );
}

#[test]
fn am_etah_is_lot_only() {
    // laṭ's te/Ate must NOT become tAm/AtAm.
    assert_eq!(
        lat_a_form("01.1130", Purusha::Prathama, Vacana::Eka),
        "laBate"
    );
}

#[test]
fn labh_vidhilin_atmanepada_all_nine_cells() {
    let expected = [
        (Purusha::Prathama, Vacana::Eka, "laBeta"),
        (Purusha::Prathama, Vacana::Dvi, "laBeyAtAm"),
        (Purusha::Prathama, Vacana::Bahu, "laBeran"),
        (Purusha::Madhyama, Vacana::Eka, "laBeTAH"),
        (Purusha::Madhyama, Vacana::Dvi, "laBeyATAm"),
        (Purusha::Madhyama, Vacana::Bahu, "laBeDvam"),
        (Purusha::Uttama, Vacana::Eka, "laBeya"),
        (Purusha::Uttama, Vacana::Dvi, "laBevahi"),
        (Purusha::Uttama, Vacana::Bahu, "laBemahi"),
    ];
    for (pu, va, form) in expected {
        assert_eq!(lin_a_form("01.1130", pu, va), form, "{pu:?} {va:?}");
    }
}

#[test]
fn siyut_survives_salopa_as_long_i() {
    // sIyta → (7.2.79) Iyta: 6.1.87's widened guard must accept the
    // long I (yāsuṭ's chain produced short iy via 7.2.80).
    let p = {
        let d = dhatus().iter().find(|d| d.dhatupatha == "01.1130").unwrap();
        sole(derive(
            d,
            Lakara::VidhiLin,
            Pada::Atmanepada,
            Purusha::Prathama,
            Vacana::Eka,
        ))
    };
    assert!(p.log.iter().any(|s| s.sutra == "3.4.102"));
    assert!(p.log.iter().any(|s| s.sutra == "7.2.79"));
    assert!(p.log.iter().any(|s| s.sutra == "6.1.87"));
    assert_eq!(p.text(), "laBeta");
}

#[test]
fn as_lot_atmanepada_uttama_eka_is_ase() {
    // adādi √ās (śap luk'd) loṭ uttama-eka: the āṭ A never merged into a
    // śap (there is none), so it leads the ending as `A E`. 6.1.90's
    // athematic ending arm must vṛddhi it to a single E (Asai → AsE).
    // Bug: AsAE (āsāai). Cf. the thematic laBE.
    assert_eq!(
        form_g("02.0011", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
        "AsE"
    );
}

#[test]
fn as_vidhilin_atmanepada_elides_optative_y_before_val() {
    // adādi √ās vidhiliṅ: on the śap-luk'd path 6.1.87 never consumes the
    // optative I, so the ending stays `I y val`; 6.1.66 must elide the y
    // (Iyta → Ita) before a val consonant — but KEEP it before a vowel.
    for (pu, va, want) in [
        (Purusha::Prathama, Vacana::Eka, "AsIta"),
        (Purusha::Prathama, Vacana::Bahu, "AsIran"),
        (Purusha::Madhyama, Vacana::Eka, "AsITAH"),
        (Purusha::Madhyama, Vacana::Bahu, "AsIDvam"),
        (Purusha::Uttama, Vacana::Dvi, "AsIvahi"),
        (Purusha::Uttama, Vacana::Bahu, "AsImahi"),
    ] {
        assert_eq!(
            form_g("02.0011", Lakara::VidhiLin, pu, va),
            want,
            "{pu:?} {va:?}"
        );
    }
    // Guard boundary: the y survives before a vowel (must NOT over-elide).
    for (pu, va, want) in [
        (Purusha::Uttama, Vacana::Eka, "AsIya"),
        (Purusha::Prathama, Vacana::Dvi, "AsIyAtAm"),
        (Purusha::Madhyama, Vacana::Dvi, "AsIyATAm"),
    ] {
        assert_eq!(
            form_g("02.0011", Lakara::VidhiLin, pu, va),
            want,
            "{pu:?} {va:?}"
        );
    }
}

#[test]
fn vrt_lat_uses_laghupadha_guna() {
    // vft's f is PENULTIMATE (upadha), not final like smf's: guna comes
    // from 7.3.86 pugantalaghUpaDasya ca, not 7.3.84.
    assert_eq!(
        lat_a_form("01.0862", Purusha::Prathama, Vacana::Eka),
        "vartate"
    );
}

#[test]
fn eco_yavayavah_athematic_arm_produces_the_ay_adesha() {
    // 6.1.78's athematic arm (śap luk'd, adādi): with no vikaraṇa buffer,
    // the ending attaches directly to the aṅga, so the ending's own
    // first character is the "next" vowel this sūtra tests. √śī laṭ
    // prathama-dvi: guṇa (7.4.21) has already made the aṅga `Se`, and the
    // ending is the vowel-initial `Ate`; this arm must turn `Se` into
    // `Say` (Se + Ate → Say + Ate → SayAte), the same mechanism that
    // gives vidhiliṅ 3pl its `SayIran` (Se + Iyran → Say + Iyran →
    // 6.1.66 → SayIran).
    assert_eq!(
        form_g("02.0026", Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
        "SayAte"
    );
    assert_eq!(
        form_g("02.0026", Lakara::VidhiLin, Purusha::Prathama, Vacana::Bahu),
        "SayIran"
    );
}

#[test]
fn shi_takes_guna_despite_the_ngit_ending() {
    // 7.4.21 śīṅaḥ sārvadhātuke guṇaḥ: √śī guṇates (SI → Se) even though
    // the ātmanepada endings are ṅit (1.2.4) and 1.1.5 would otherwise
    // block guṇa. This is the only visible guṇa in the whole adādi gaṇa.
    //
    // Note: the surface forms below (`Sete`, `aSeta`) now pin
    // attribution as well as shape. Before 7.3.84's 1.1.5 guard was
    // rewired to `following_sarvadhatuka`, it read only the (always-pit)
    // SHAP term and never actually blocked on this śap-luk'd path, so
    // removing 7.4.21 would have left 7.3.84 to fire unaided and produce
    // these same two forms. Now the ṅit ending IS the immediate
    // follower, so 1.1.5 blocks 7.3.84 here too: removing 7.4.21 would
    // leave the aṅga un-guṇated (`SIte`/`aSIta`), not `Sete`/`aSeta`.
    // The attribution is still pinned independently by the
    // ordered-trace test `shete_trace_is_the_minimal_shing_guna_path` in
    // `crates/panini/tests/trace.rs`.
    assert_eq!(
        form_g("02.0026", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "Sete"
    );
    // laṅ: 6.4.71 has already prefixed the aṭ-augment, so the aṅga is
    // `aSI` when 7.4.21 runs — the guard must match on the tail, not the
    // whole string.
    assert_eq!(
        form_g("02.0026", Lakara::Lan, Purusha::Prathama, Vacana::Eka),
        "aSeta"
    );
}

// --- cartva, her-dhih, a-augment guard pins for 5c adadi rules -------
#[test]
fn cartva_guard_is_khar_only_not_m_or_vowel() {
    // Over-application killer: d before `m` (admi) or vowel (adanti) must NOT
    // cartva-ize. Under-application killer: d before `t` MUST (atti, not adti).
    assert_eq!(
        form_g("02.0001", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "admi"
    );
    assert_eq!(
        form_g("02.0001", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "adanti"
    );
    assert_eq!(
        form_g("02.0001", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "atti"
    );
}

#[test]
fn dhi_ca_elides_s_before_dhve() {
    // √ās 2pl: the root-final `s` meets the `Dh` of Dve/Dvam and is
    // ELIDED by 8.2.25 dhi ca — it is not voiced to `d`. 8.2.25 sits at
    // 8.2 in the tripādī and is asiddha to 8.4, so the `s` is gone before
    // any 8.4 junction rule can look at it: As + Dve -> A + Dve -> ADve.
    assert_eq!(
        form_g("02.0011", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
        "ADve"
    );
    assert_eq!(
        form_g("02.0011", Lakara::Lan, Purusha::Madhyama, Vacana::Bahu),
        "ADvam"
    );
    assert_eq!(
        form_g("02.0011", Lakara::Lot, Purusha::Madhyama, Vacana::Bahu),
        "ADvam"
    );
    // Guard boundary: the affix must be Dh-initial. A clean `s`-meets-`s`
    // cell is untouched, so 2sg stays Asse — the rule must not
    // over-apply.
    assert_eq!(
        form_g("02.0011", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
        "Asse"
    );
}

#[test]
fn dhi_ca_fires_for_vas_and_only_before_dh() {
    // √vas is the sūtra's second witness: vas + Dve -> va + Dve -> vaDve.
    assert_eq!(
        form_g("02.0013", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
        "vaDve"
    );
    assert_eq!(
        form_g("02.0013", Lakara::Lan, Purusha::Madhyama, Vacana::Bahu),
        "avaDvam"
    );
    assert_eq!(
        form_g("02.0013", Lakara::Lot, Purusha::Madhyama, Vacana::Bahu),
        "vaDvam"
    );
    // The affix must be Dh-initial. These four cells put the same aṅga-
    // final `s` in front of `t`, `T` and `s` and it must survive intact —
    // and they are also the first pins that cartva (8.4.55) leaves an `s`
    // alone before a khar, an arm √ad and √ās could not reach.
    assert_eq!(
        form_g("02.0013", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "vaste"
    );
    assert_eq!(
        form_g("02.0013", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
        "vasse"
    );
    assert_eq!(
        form_g("02.0013", Lakara::Lan, Purusha::Madhyama, Vacana::Eka),
        "avasTAH"
    );
    assert_eq!(
        form_g("02.0013", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
        "vassva"
    );
}

#[test]
fn dhi_ca_does_not_elide_a_non_s_before_dh() {
    // Thematic ātmanepada √labh keeps its śap `a` in front of Dve/Dvam.
    // √labh is bhvādi, so śap is never luk'd: the first non-empty term
    // after the aṅga is the śap `a` itself, and `"a".starts_with('D')` is
    // false — the guard declines at its FIRST arm (the `D`-initial
    // affix search), never reaching the "preceding term ends in `s`"
    // arm at all. These are the slice-3 goldens, unchanged.
    assert_eq!(
        form_g("01.1130", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
        "laBaDve"
    );
    assert_eq!(
        form_g("01.1130", Lakara::Lan, Purusha::Madhyama, Vacana::Bahu),
        "alaBaDvam"
    );
    // adDi (√ad loṭ 2sg, pinned at paradigm level in
    // crates/panini/tests/paradigm.rs) is the one cell in the current
    // root set that actually reaches the "ends in `s`" arm: śap is
    // luk'd (adādi) so ENDING (`Di`, from 6.4.101 her dhiḥ) is the first
    // non-empty term after the aṅga and the `D`-initial arm PASSES; the
    // preceding non-empty term is the aṅga `ad`, which does not end in
    // `s`, so the second arm declines and the `d` survives. Dropping
    // this arm would wrongly yield *aDi.
    assert_eq!(
        form_g_forked("02.0001", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "adDi"
    );
}

#[test]
fn dhi_ca_reads_the_affixs_neighbour_not_the_anga() {
    // vidhiliṅ 2pl: the sīyuṭ augment is prefixed INTO the ending's text
    // (`IDvam`), not a separate term, so `terms` is still the plain
    // three-element `[ANGA, SHAP, ENDING]` layout here. The first
    // non-empty term after the aṅga is the ending `IDvam` itself, and
    // `"IDvam".starts_with('D')` is false — these two pins decline at the
    // guard's FIRST arm (the `D`-initial affix search), the same arm as
    // laB above, not because the guard is reading some other neighbour
    // instead of the aṅga. In today's layout the backward search always
    // resolves to the aṅga whenever the first arm passes; it is written
    // to walk to the nearest non-empty term (rather than index ANGA
    // directly) for the multi-term layouts a later slice will bring —
    // mirroring vidyut-prakriya's own `prev_not_empty`. As + I + Dvam ->
    // AsIDvam (never *AIDvam): the `s` is retained because the affix
    // search declines, not because of anything more subtle.
    assert_eq!(
        form_g("02.0011", Lakara::VidhiLin, Purusha::Madhyama, Vacana::Bahu),
        "AsIDvam"
    );
    assert_eq!(
        form_g("02.0013", Lakara::VidhiLin, Purusha::Madhyama, Vacana::Bahu),
        "vasIDvam"
    );
}

#[test]
fn cartva_of_maps_each_jhal_to_its_first_varga_car() {
    // 8.4.55 car table. Only d→t is exercised by √ad this slice; the other
    // vargas are written generally for later jhal-final roots. Pin the whole
    // mapping so a dropped/altered arm is caught by mutation testing.
    for c in ['d', 'D', 't', 'T'] {
        assert_eq!(cartva_of(c), Some('t'), "{c}");
    }
    for c in ['g', 'G', 'k', 'K'] {
        assert_eq!(cartva_of(c), Some('k'), "{c}");
    }
    for c in ['b', 'B', 'p', 'P'] {
        assert_eq!(cartva_of(c), Some('p'), "{c}");
    }
    for c in ['j', 'J', 'c', 'C'] {
        assert_eq!(cartva_of(c), Some('c'), "{c}");
    }
    for c in ['q', 'Q', 'w', 'W'] {
        assert_eq!(cartva_of(c), Some('w'), "{c}");
    }
    // Non-car targets return None (e.g. h, sibilants, vowels).
    for c in ['h', 'S', 'z', 's', 'a'] {
        assert_eq!(cartva_of(c), None, "{c}");
    }
}

#[test]
fn her_dhih_guard_is_jhal_final_only() {
    // ā-final √yā loṭ 2sg keeps hi (yAhi), NOT *yADi: 6.4.101 needs a jhal.
    assert_eq!(
        form_g_forked("02.0044", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "yAhi"
    );
}

#[test]
fn a_augment_does_not_leak_into_dual_or_plural() {
    // The single-char length guard: 2du ending `tam` must NOT get an `a`
    // (no *Adatam); it stays Attam via cartva.
    assert_eq!(
        form_g("02.0001", Lakara::Lan, Purusha::Madhyama, Vacana::Dvi),
        "Attam"
    );
}

#[test]
fn seventwone_five_atmanepada_3pl_uses_at_not_ant() {
    // 7.1.5 ātmanepadeṣv anataḥ: √ās (adādi, s-final) 3pl → Asate/Asata/
    // AsatAm (Ja → at, not the `ant` of 7.1.3). A-final thematic roots keep
    // `ante` (7.1.5 declines), so laB is unchanged.
    assert_eq!(
        form_g("02.0011", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "Asate"
    );
    assert_eq!(
        form_g("02.0011", Lakara::Lan, Purusha::Prathama, Vacana::Bahu),
        "Asata"
    );
    assert_eq!(
        form_g("02.0011", Lakara::Lot, Purusha::Prathama, Vacana::Bahu),
        "AsatAm"
    );
    // Guard boundary: a-final ātmanepada aṅga still takes 7.1.3's `ante`.
    assert_eq!(
        form_g("01.1130", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "laBante"
    );
}

#[test]
fn anatah_declines_for_a_final_atmanepada_angas() {
    // 7.1.5's "anataḥ" arm: every a-final (thematic / vikaraṇa-buffered)
    // ātmanepada 3pl keeps 7.1.3's `ante`. Pins that the guard reads the
    // preceding segment's `a`, not the consonant-final root.
    assert_eq!(
        form_g("01.1130", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "laBante"
    );
    assert_eq!(
        form_g("04.0073", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "manyante"
    );
    assert_eq!(
        form_g("06.0008", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "juzante"
    );
}

#[test]
fn voiced_junction_does_not_touch_non_jhas_or_non_jhal_junctions() {
    // Under-application guard: `s` before the non-jhaś `s`/`th`/`v` of
    // se/sva/thās stays `s` (Asse, Assva, AsTAH) — only a jhaś triggers it.
    assert_eq!(
        form_g("02.0011", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
        "Assva"
    );
    assert_eq!(
        form_g("02.0011", Lakara::Lan, Purusha::Madhyama, Vacana::Eka),
        "AsTAH"
    );
    // Over-application guard: √ad is parasmaipada; its voiceless junctions
    // stay cartva's business:
    assert_eq!(
        form_g("02.0001", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "atti"
    );
}

#[test]
fn shings_jha_takes_the_rut_augment() {
    // 7.1.6 śīṅo ruṭ: the *jha* (3pl ātmanepada) of √śī takes the ruṭ
    // augment. 7.1.5 has just turned the leading J into `at` (Je → ate);
    // ruṭ's `r` prefixes that: Se + r + ate → Serate.
    assert_eq!(
        form_g("02.0026", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "Serate"
    );
    assert_eq!(
        form_g("02.0026", Lakara::Lot, Purusha::Prathama, Vacana::Bahu),
        "SeratAm"
    );
    assert_eq!(
        form_g("02.0026", Lakara::Lan, Purusha::Prathama, Vacana::Bahu),
        "aSerata"
    );
}

#[test]
fn shings_vidhilin_3pl_takes_no_rut() {
    // 3.4.105 jhasya ran replaces the jha with `ran` long before the 7.x
    // band, so 7.1.5 never fires in vidhiliṅ and ruṭ cannot attach:
    // SayIran, NOT *SayIraran.
    assert_eq!(
        form_g("02.0026", Lakara::VidhiLin, Purusha::Prathama, Vacana::Bahu),
        "SayIran"
    );
}

#[test]
fn shatva_retroflexes_the_endings_s_after_shings_e() {
    // 8.3.59 ādeśapratyayayoḥ: the `s` of a pratyaya retroflexes after a
    // non-a/ā vowel. With the aṅga guṇated to `Se`, the `se` and `sva`
    // endings meet an `e` → Seze, Sezva.
    assert_eq!(
        form_g("02.0026", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
        "Seze"
    );
    assert_eq!(
        form_g("02.0026", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
        "Sezva"
    );
}

#[test]
fn rudhadi_strong_cells() {
    // The strong stem is śnam with its `a` intact. kft needs no new rule at
    // all beyond 3.1.78 — 8.4.1 ṇatva already fires across the ANGA/SHAP
    // junction, exactly as it does for kryādi's vf + nA → vfRAti.
    assert_eq!(
        form_g("07.0010", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "kfRatti"
    );
    assert_eq!(
        form_g("07.0010", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "kfRatmi"
    );
    // √hiṃs needs 6.4.23: hins + śnam is hinans, and the root's own n comes
    // back out.
    assert_eq!(
        form_g("07.0019", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "hinasti"
    );
    assert_eq!(
        form_g("07.0019", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
        "hinasAni"
    );
    // The ātmanepada arm's strong cells keep śnam's `a` too.
    assert_eq!(
        form_g("07.0012", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
        "KinadE"
    );
}

#[test]
fn rudhadi_weak_cells_lose_shnams_a() {
    // 6.4.111 fires before a kṅit sārvadhātuka and makes the strong/weak
    // split visible. These are the cells 8.4.65 does NOT fork, so they are
    // safe to assert with `form_g` at this stage.
    assert_eq!(
        form_g("07.0010", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "kfntanti"
    );
    assert_eq!(
        form_g("07.0010", Lakara::Lat, Purusha::Uttama, Vacana::Dvi),
        "kfntvaH"
    );
    assert_eq!(
        form_g("07.0012", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "Kinde"
    );
    assert_eq!(
        form_g("07.0012", Lakara::VidhiLin, Purusha::Prathama, Vacana::Eka),
        "KindIta"
    );
}

#[test]
fn rudhadi_anusvara_round_trip_is_conditional() {
    // 8.3.24 turns śnam's n into an anusvāra before a jhal; 8.4.58 turns it
    // back into the following sound's homorganic nasal — but only before a
    // YAY. √hiṃs is the witness that the return leg is conditional: the
    // anusvāra there is followed by the ROOT's own `s`, which is śal, not
    // yay, so it survives. Folding the two rules into one operation would
    // derive *hintaH.
    assert_eq!(
        form_g("07.0019", Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
        "hiMstaH"
    );
    assert_eq!(
        form_g("07.0019", Lakara::VidhiLin, Purusha::Prathama, Vacana::Bahu),
        "hiMsyuH"
    );
    assert_eq!(
        form_g("07.0010", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "kfntanti"
    );
}

#[test]
fn rudhadi_lot_madhyama_eka_takes_jashtva() {
    // 6.4.101 her DiH makes the ending `Di`; the weak stem's final `t`
    // meets it and is voiced-aspirated to `d` by 8.4.53. This is the rule
    // commit 9fa8e5f deleted as unreachable once 8.2.25 dhi ca replaced
    // slice 5d's analysis — √kṛt is its first genuine witness.
    //
    // 7.1.35 tātaṅ optionally forks this cell (parasmaipada loṭ madhyama
    // eka), independently of anything in this slice; branch 0 is the
    // derivation with no optional rule applied.
    //
    // 8.4.65 jharo jhari savarṇe (Task 7) stacks on top of that fork: it
    // optionally elides the weak stem's final `t`/`d` before a savarṇa
    // `D`/`t` (the `n` before it survives in every branch), on both the
    // tātaṅ and non-tātaṅ branches. 8.4.56
    // vāvasāne then optionally forks the tātaṅ branches' pada-final vowel-
    // adjacent `t`/`d` at pause but declines on the two vowel-final
    // non-tātaṅ branches (kfndDi, kfnDi) — so k = 3 optional forks give six
    // derivations, not eight: kfndDi, kfnDi (8.4.65), kfnttAd (7.1.35),
    // kfntAd (7.1.35+8.4.65), kfnttAt (7.1.35+8.4.56), kfntAt
    // (7.1.35+8.4.65+8.4.56).
    assert_eq!(
        form_g_forked("07.0010", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 6),
        "kfndDi"
    );
    // √hiṃs reaches the same cell through 8.2.25 instead: its stem-final
    // `s` is ELIDED before the Dh-initial ending, not voiced. Third witness
    // for that rule, on a stem shape it has not seen.
    assert_eq!(
        form_g_forked("07.0019", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "hinDi"
    );
}

#[test]
fn rudhadi_savarna_elision_forks() {
    // The declined branch keeps both consonants and is index 0.
    assert_eq!(
        form_g_forked("07.0010", Lakara::Lat, Purusha::Prathama, Vacana::Dvi, 2),
        "kfnttaH"
    );
    assert_eq!(
        form_g_forked("07.0012", Lakara::Lat, Purusha::Prathama, Vacana::Eka, 2),
        "Kintte"
    );
    // √hiṃs never forks here: `s` and `t` are not savarṇa.
    assert_eq!(
        form_g("07.0019", Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
        "hiMstaH"
    );
}

#[test]
fn rudhadi_savarna_elision_derives_both_members() {
    let d = dhatus().iter().find(|d| d.dhatupatha == "07.0010").unwrap();
    let forms: Vec<String> = derive(
        d,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    )
    .iter()
    .map(|p| p.text())
    .collect();
    assert_eq!(forms, vec!["kfnttaH".to_string(), "kfntaH".to_string()]);
}

#[test]
fn rudhadi_lan_eka_cells() {
    // prathama eka: √kṛt's `d` comes from the existing 8.2.39, √hiṃs's from
    // the new 8.2.73 — 8.2.39 declines on a final `s` by design.
    assert_eq!(
        form_g_forked("07.0010", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2),
        "akfRad"
    );
    assert_eq!(
        form_g_forked("07.0019", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2),
        "ahinad"
    );
    // madhyama eka forks three ways: the stop, its pausal variant, and ru.
    assert_eq!(
        form_g_forked("07.0010", Lakara::Lan, Purusha::Madhyama, Vacana::Eka, 3),
        "akfRad"
    );
    assert_eq!(
        form_g_forked("07.0019", Lakara::Lan, Purusha::Madhyama, Vacana::Eka, 3),
        "ahinad"
    );
}

#[test]
fn ru_branch_derives_the_visarga_forms() {
    for (number, expected) in [("07.0010", "akfRaH"), ("07.0019", "ahinaH")] {
        let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
        let forms: Vec<String> = derive(
            d,
            Lakara::Lan,
            Pada::Parasmaipada,
            Purusha::Madhyama,
            Vacana::Eka,
        )
        .iter()
        .map(|p| p.text())
        .collect();
        assert!(
            forms.contains(&expected.to_string()),
            "{}: {forms:?}",
            d.code
        );
    }
}

#[test]
fn shnams_ru_fires_on_the_dhatus_own_final() {
    // 8.2.74 must see `ahinas`, not the `ahinad` 8.2.73 would already have
    // produced — which is why it is ordered ABOVE 8.2.73, against sūtra
    // order. Assert the order, not just the surface: numeric order still
    // derives ahinad on both branches, it simply never derives ahinaH.
    let d = dhatus().iter().find(|d| d.dhatupatha == "07.0019").unwrap();
    let p = derive(
        d,
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    )
    .into_iter()
    .find(|p| p.text() == "ahinaH")
    .expect("ahinaH branch");
    let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
    let ru = ids.iter().position(|s| *s == "8.2.74").expect("8.2.74");
    assert!(
        !ids[..ru].contains(&"8.2.73"),
        "8.2.73 must not precede 8.2.74: {ids:?}"
    );
}

#[test]
fn the_ru_alternation_stays_off_the_new_roots() {
    // 8.2.73's deferred re-verification, discharged. √bhañj and √piṣ are
    // the first roots ADDED SINCE THE WARNING WAS WRITTEN to empty ENDING
    // under 8.2.23 — NOT the first roots after √hiṃs to do so at all:
    // √kṛt has emptied it at these same cells since 7a too (8.2.75 fires
    // for it there, which requires `dhatu_is_pada_final`). √bhañj and
    // √piṣ are the first live test of the invariant since the warning
    // was written, i.e. since the root set was last widened.
    //
    // The invariant HOLDS: both empty it at laṅ prathama/madhyama eka,
    // i.e. still tip and sip. And 8.2.73 declines on them regardless: by
    // the time it runs, 8.2.30 has already velarised √bhañj's stem to
    // `aBanag` and 8.2.39 has already voiced √piṣ's to `apinaq`, and its
    // `s`-final check does not match either. If it over-fired, these
    // cells would surface a `d` and then a visarga via 8.2.75 and 8.3.15.
    for (number, want) in [("07.0016", "aBanag"), ("07.0015", "apinaq")] {
        let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
        for pu in [Purusha::Prathama, Purusha::Madhyama] {
            assert_eq!(
                form_g_forked(number, Lakara::Lan, pu, Vacana::Eka, 2),
                want,
                "{} laṅ eka took the ru alternation",
                d.code
            );
        }
    }
}

#[test]
fn no_8_2_73_step_appears_for_bhanj_or_pish() {
    for number in ["07.0016", "07.0015"] {
        for pu in [Purusha::Prathama, Purusha::Madhyama] {
            let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
            for p in derive(d, Lakara::Lan, d.pada.padas()[0], pu, Vacana::Eka) {
                assert!(
                    !p.log.iter().any(|s| s.sutra == "8.2.73"),
                    "{}: 8.2.73 fired outside √hiṃs",
                    d.code
                );
            }
        }
    }
}

#[test]
fn rudhadi_vidhilin_madhyama_eka_is_untouched_by_the_ru_alternation() {
    // `Context::is_sip` (8.2.74's guard) is a lakāra-blind slot predicate
    // (parasmaipada madhyama eka, regardless of lakāra), so this cell —
    // ending `yAs`, which 8.2.23 saṁyogāntasya lopaḥ leaves fully intact
    // because a VOWEL (`A`) precedes the `s`, not a consonant conjunct —
    // ALSO satisfies `is_sip()`, with `ENDING` genuinely holding `yAs`/`yAd`.
    // `dhatu_is_pada_final` is what keeps 8.2.73/8.2.74 off it: without that
    // guard, 8.2.73 (OBLIGATORY, not optional, and no longer guarded by any
    // slot predicate at all — mutation testing showed `is_tip`/`is_sip`
    // wasn't load-bearing on it, so it was dropped) rewrites `ENDING`'s own
    // `s` to `d`, corrupting the cell's single, primary surface form to
    // `kfntyAd`/`hiMsyAd` instead of the correct `kfntyAH`/`hiMsyAH`
    // (6.1.68 reduces `yAs` to `yAH`). `form_g` goes through `sole`, so this
    // also pins the branch count at exactly one — witnessing that 8.2.74's
    // copy of the guard declines here too, not just 8.2.73's.
    assert_eq!(
        form_g("07.0010", Lakara::VidhiLin, Purusha::Madhyama, Vacana::Eka),
        "kfntyAH"
    );
    assert_eq!(
        form_g("07.0019", Lakara::VidhiLin, Purusha::Madhyama, Vacana::Eka),
        "hiMsyAH"
    );
}

#[test]
fn bhanj_lat_all_nine_cells() {
    // The strong stem velarises (Banaj + ti -> Banag + ti -> Banakti, via
    // 8.2.30 then 8.4.55); the weak stem does the same across the anusvāra
    // round trip (Banj + taH -> Bang + taH -> BaMgtaH -> BaMktaH ->
    // BaNktaH). The `n` that survives in BaNktaH is śnam's: 6.4.23 already
    // took the root's own `n` out.
    let cells = [
        (Purusha::Prathama, Vacana::Eka, "Banakti"),
        (Purusha::Prathama, Vacana::Dvi, "BaNktaH"),
        (Purusha::Prathama, Vacana::Bahu, "BaYjanti"),
        (Purusha::Madhyama, Vacana::Eka, "Banakzi"),
        (Purusha::Madhyama, Vacana::Dvi, "BaNkTaH"),
        (Purusha::Madhyama, Vacana::Bahu, "BaNkTa"),
        (Purusha::Uttama, Vacana::Eka, "Banajmi"),
        (Purusha::Uttama, Vacana::Dvi, "BaYjvaH"),
        (Purusha::Uttama, Vacana::Bahu, "BaYjmaH"),
    ];
    for (pu, va, want) in cells {
        assert_eq!(form_g("07.0016", Lakara::Lat, pu, va), want);
    }
}

#[test]
fn bhanj_lan_eka_velarises_word_finally() {
    // 8.2.23 eats tip's own `t` (and sip's own `s`), leaving the dhātu's
    // `j` as the true word end; 8.2.30 then applies word-finally rather
    // than before a jhal. Both eka cells fork on 8.4.56 alone.
    assert_eq!(
        form_g_forked("07.0016", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2),
        "aBanag"
    );
    assert_eq!(
        form_g_forked("07.0016", Lakara::Lan, Purusha::Madhyama, Vacana::Eka, 2),
        "aBanag"
    );
}

#[test]
fn bhanj_lot_madhyama_eka_is_bhangdhi() {
    // 6.4.101 her dhiH gives the `Di`; 8.2.30 velarises the `j` before it
    // (a jhal), and 8.4.53 declines because `g` is already its own jaś.
    // Three branches: the declined one plus 7.1.35's tātaṅ and its 8.4.56
    // pausal fork.
    assert_eq!(
        form_g_forked("07.0016", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "BaNgDi"
    );
}

#[test]
fn coh_kuh_declines_before_a_non_jhal_non_final() {
    // The witnesses that keep 8.2.30's guard from being written too wide.
    // In BaYjanti what follows the `j` is `a`, and in BaYjvaH it is `v` —
    // neither a jhal nor a word end — so the `j` survives to take 8.3.24's
    // anusvāra and 8.4.58's palatal parasavarṇa instead.
    assert_eq!(
        form_g("07.0016", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "BaYjanti"
    );
    assert_eq!(
        form_g("07.0016", Lakara::Lat, Purusha::Uttama, Vacana::Dvi),
        "BaYjvaH"
    );
}

#[test]
fn pish_lat_retroflexes_around_the_shnam_stem() {
    // 8.4.41 ṣṭunā ṣṭuḥ: the ending's dental retroflexes in contact with
    // the root's ṣ. Madhyama eka (pinakzi) is deliberately absent — it
    // needs 8.2.41, which lands in the next task.
    let cells = [
        (Purusha::Prathama, Vacana::Eka, "pinazwi"),
        (Purusha::Prathama, Vacana::Dvi, "piMzwaH"),
        (Purusha::Prathama, Vacana::Bahu, "piMzanti"),
        (Purusha::Madhyama, Vacana::Dvi, "piMzWaH"),
        (Purusha::Madhyama, Vacana::Bahu, "piMzWa"),
        (Purusha::Uttama, Vacana::Eka, "pinazmi"),
        (Purusha::Uttama, Vacana::Dvi, "piMzvaH"),
        (Purusha::Uttama, Vacana::Bahu, "piMzmaH"),
    ];
    for (pu, va, want) in cells {
        assert_eq!(form_g("07.0015", Lakara::Lat, pu, va), want);
    }
}

#[test]
fn pish_weak_stem_keeps_its_anusvara() {
    // The SECOND witness that 8.3.24 and 8.4.58 are not a no-op pair.
    // 8.4.58 needs a yay to follow; what follows here is the root's own
    // `z`, which is śal — so piMzwaH keeps the anusvāra that kfntaH
    // resolves. √hiṃs's hiMstaH was the first witness, in 7a.
    assert_eq!(
        form_g("07.0015", Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
        "piMzwaH"
    );
}

#[test]
fn shtutva_requires_strict_adjacency() {
    // piMzanti keeps a DENTAL n: the `a` between the ṣ and the n breaks
    // the contact 8.4.41 requires. pinazARi's retroflex ṇ is a different
    // rule's — ṇatva (8.4.1 / 8.4.2), which 8.4.2 explicitly lets an aṭ
    // intervene in. Conflating the two would retroflex piMzanti as well.
    assert_eq!(
        form_g("07.0015", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "piMzanti"
    );
    assert_eq!(
        form_g("07.0015", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
        "pinazARi"
    );
}

#[test]
fn pish_vidhilin_all_nine_cells() {
    // The optative's `y` is neither dental stop nor `s`, so 8.4.41 has
    // nothing to do here; the cells are pure weak stem plus 8.4.56 on
    // prathama eka.
    assert_eq!(
        form_g_forked(
            "07.0015",
            Lakara::VidhiLin,
            Purusha::Prathama,
            Vacana::Eka,
            2
        ),
        "piMzyAd"
    );
    let cells = [
        (Purusha::Prathama, Vacana::Dvi, "piMzyAtAm"),
        (Purusha::Prathama, Vacana::Bahu, "piMzyuH"),
        (Purusha::Madhyama, Vacana::Eka, "piMzyAH"),
        (Purusha::Madhyama, Vacana::Dvi, "piMzyAtam"),
        (Purusha::Madhyama, Vacana::Bahu, "piMzyAta"),
        (Purusha::Uttama, Vacana::Eka, "piMzyAm"),
        (Purusha::Uttama, Vacana::Dvi, "piMzyAva"),
        (Purusha::Uttama, Vacana::Bahu, "piMzyAma"),
    ];
    for (pu, va, want) in cells {
        assert_eq!(form_g("07.0015", Lakara::VidhiLin, pu, va), want);
    }
}

#[test]
fn pish_lat_madhyama_eka_is_pinakshi() {
    // 8.2.41 ṣaḍhoḥ kaḥ si takes the ṣ to `k` before the ending's `s`,
    // and 8.3.59 then retroflexes that `s` after the new `k` — the
    // widening this cell forces, and the one 8.3.59's own comment
    // predicted ("h/y/v/r/l or k").
    assert_eq!(
        form_g("07.0015", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
        "pinakzi"
    );
}

#[test]
fn shadhoh_kah_si_declines_when_8_2_23_ate_the_s_first() {
    // THE LOAD-BEARING ORDER of this slice. At laṅ madhyama eka the
    // ending is a bare `s`, so 8.2.23 saṁyogāntasya lopaḥ elides it
    // before 8.2.41 can see it, and the cell reduces through 8.2.39 and
    // 8.4.56 to exactly what laṅ prathama eka gives. Run 8.2.41 above
    // 8.2.23 and you get `apinak` instead — a plausible-looking form that
    // splits madhyama eka from prathama eka.
    assert_eq!(
        form_g_forked("07.0015", Lakara::Lan, Purusha::Madhyama, Vacana::Eka, 2),
        "apinaq"
    );
    assert_eq!(
        form_g_forked("07.0015", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2),
        "apinaq"
    );
}

#[test]
fn pish_lan_all_nine_cells() {
    let cells = [
        (Purusha::Prathama, Vacana::Dvi, "apiMzwAm"),
        (Purusha::Prathama, Vacana::Bahu, "apiMzan"),
        (Purusha::Madhyama, Vacana::Dvi, "apiMzwam"),
        (Purusha::Madhyama, Vacana::Bahu, "apiMzwa"),
        (Purusha::Uttama, Vacana::Eka, "apinazam"),
        (Purusha::Uttama, Vacana::Dvi, "apiMzva"),
        (Purusha::Uttama, Vacana::Bahu, "apiMzma"),
    ];
    for (pu, va, want) in cells {
        assert_eq!(form_g("07.0015", Lakara::Lan, pu, va), want);
    }
}

#[test]
fn pish_lot_madhyama_eka_is_pinddhi() {
    // The deepest cell in this slice: four branches. 6.4.101 her dhiH
    // gives the `Di`; 8.4.41 retroflexes it to `Qi`; 8.4.53 (widened to
    // any jhaś, not just `D`) voices the ṣ to `q` before it; 8.4.58 takes
    // the anusvāra to `R` as that `q`'s parasavarṇa; and 8.4.65 optionally
    // elides the `q` before the savarṇa `Q`. 7.1.35's tātaṅ and its 8.4.56
    // fork supply the other two branches.
    assert_eq!(
        form_g_forked("07.0015", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 4),
        "piRqQi"
    );
}

#[test]
fn pish_lot_all_nine_cells() {
    assert_eq!(
        form_g_forked("07.0015", Lakara::Lot, Purusha::Prathama, Vacana::Eka, 3),
        "pinazwu"
    );
    let cells = [
        (Purusha::Prathama, Vacana::Dvi, "piMzwAm"),
        (Purusha::Prathama, Vacana::Bahu, "piMzantu"),
        (Purusha::Madhyama, Vacana::Dvi, "piMzwam"),
        (Purusha::Madhyama, Vacana::Bahu, "piMzwa"),
        (Purusha::Uttama, Vacana::Eka, "pinazARi"),
        (Purusha::Uttama, Vacana::Dvi, "pinazAva"),
        (Purusha::Uttama, Vacana::Bahu, "pinazAma"),
    ];
    for (pu, va, want) in cells {
        assert_eq!(form_g("07.0015", Lakara::Lot, pu, va), want);
    }
}

#[test]
fn jhalam_jash_jhashi_still_declines_on_its_two_pre_existing_shapes() {
    // The widening must not disturb either 7a cell. √khid's KindDve
    // presents a `d` that is already its own jaś — the NO-OP GUARD
    // declines it. √hiṃs's hinDi presents an `n`, for which jashtva_of
    // returns None — a DIFFERENT clause. Both remain branch 0.
    assert_eq!(
        form_g_forked("07.0012", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu, 2),
        "KindDve"
    );
    assert_eq!(
        form_g_forked("07.0019", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "hinDi"
    );
}

#[test]
fn indh_lat_all_nine_cells() {
    // 8.2.40 turns the ending's `t` into `D` after the stem's jhaṣ, and
    // the widened 8.4.53 then voices the stem's own `D` to `d` before it:
    // inD + te -> inD + De -> indDe. 8.4.65 optionally elides that `d`
    // before the savarṇa `D`, which is where inDe comes from.
    assert_eq!(
        form_g_forked("07.0011", Lakara::Lat, Purusha::Prathama, Vacana::Eka, 2),
        "indDe"
    );
    assert_eq!(
        form_g_forked("07.0011", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu, 2),
        "indDve"
    );
    let cells = [
        (Purusha::Prathama, Vacana::Dvi, "inDAte"),
        (Purusha::Prathama, Vacana::Bahu, "inDate"),
        (Purusha::Madhyama, Vacana::Eka, "intse"),
        (Purusha::Madhyama, Vacana::Dvi, "inDATe"),
        (Purusha::Uttama, Vacana::Eka, "inDe"),
        (Purusha::Uttama, Vacana::Dvi, "inDvahe"),
        (Purusha::Uttama, Vacana::Bahu, "inDmahe"),
    ];
    for (pu, va, want) in cells {
        assert_eq!(form_g("07.0011", Lakara::Lat, pu, va), want);
    }
}

#[test]
fn jhashas_tathor_dhodhah_declines_before_a_non_dental() {
    // intse is the witness that 8.2.40 is not simply "voice everything
    // after the stem". sip's `se` begins with `s`, not `t`/`th`, so the
    // rule declines and 8.4.55 khari ca devoices the stem's `D` to `t`
    // instead. inDvahe and inDmahe make the same point for `v` and `m`.
    assert_eq!(
        form_g("07.0011", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
        "intse"
    );
    assert_eq!(
        form_g("07.0011", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
        "intsva"
    );
}

#[test]
fn indh_strong_stem_appears_only_in_lot_uttama() {
    // The ātmanepada endings are ṅit throughout except loṭ uttama, where
    // the strong stem inaD survives 6.4.111 and shows śnam's `a`.
    assert_eq!(
        form_g("07.0011", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
        "inaDE"
    );
    assert_eq!(
        form_g("07.0011", Lakara::Lot, Purusha::Uttama, Vacana::Dvi),
        "inaDAvahE"
    );
    assert_eq!(
        form_g("07.0011", Lakara::Lot, Purusha::Uttama, Vacana::Bahu),
        "inaDAmahE"
    );
}

#[test]
fn indh_lan_and_lot_and_vidhilin_cells() {
    // laṅ takes the āṭ augment, which 6.1.90 āṭaś ca raises to `E`.
    for (pu, va, want) in [
        (Purusha::Prathama, Vacana::Eka, "EndDa"),
        (Purusha::Madhyama, Vacana::Eka, "EndDAH"),
        (Purusha::Madhyama, Vacana::Bahu, "EndDvam"),
    ] {
        assert_eq!(form_g_forked("07.0011", Lakara::Lan, pu, va, 2), want);
    }
    for (pu, va, want) in [
        (Purusha::Prathama, Vacana::Dvi, "EnDAtAm"),
        (Purusha::Prathama, Vacana::Bahu, "EnData"),
        (Purusha::Madhyama, Vacana::Dvi, "EnDATAm"),
        (Purusha::Uttama, Vacana::Eka, "EnDi"),
        (Purusha::Uttama, Vacana::Dvi, "EnDvahi"),
        (Purusha::Uttama, Vacana::Bahu, "EnDmahi"),
    ] {
        assert_eq!(form_g("07.0011", Lakara::Lan, pu, va), want);
    }

    assert_eq!(
        form_g_forked("07.0011", Lakara::Lot, Purusha::Prathama, Vacana::Eka, 2),
        "indDAm"
    );
    assert_eq!(
        form_g_forked("07.0011", Lakara::Lot, Purusha::Madhyama, Vacana::Bahu, 2),
        "indDvam"
    );
    for (pu, va, want) in [
        (Purusha::Prathama, Vacana::Dvi, "inDAtAm"),
        (Purusha::Prathama, Vacana::Bahu, "inDatAm"),
        (Purusha::Madhyama, Vacana::Dvi, "inDATAm"),
    ] {
        assert_eq!(form_g("07.0011", Lakara::Lot, pu, va), want);
    }

    // vidhiliṅ takes no fork at all: the optative `I` is neither a jhal
    // nor pada-final, so neither 8.4.65 nor 8.4.56 reaches these cells.
    for (pu, va, want) in [
        (Purusha::Prathama, Vacana::Eka, "inDIta"),
        (Purusha::Prathama, Vacana::Dvi, "inDIyAtAm"),
        (Purusha::Prathama, Vacana::Bahu, "inDIran"),
        (Purusha::Madhyama, Vacana::Eka, "inDITAH"),
        (Purusha::Madhyama, Vacana::Dvi, "inDIyATAm"),
        (Purusha::Madhyama, Vacana::Bahu, "inDIDvam"),
        (Purusha::Uttama, Vacana::Eka, "inDIya"),
        (Purusha::Uttama, Vacana::Dvi, "inDIvahi"),
        (Purusha::Uttama, Vacana::Bahu, "inDImahi"),
    ] {
        assert_eq!(form_g("07.0011", Lakara::VidhiLin, pu, va), want);
    }
}

#[test]
fn trh_takes_the_im_agama_only_before_a_hal_initial_pit_sarvadhatuka() {
    // 7.3.92 tfRaha im, all four conjuncts of its guard. Three have a cell
    // that breaks if the conjunct is dropped; the pit conjunct does not --
    // see the "tas -> tfRQaH" cell below and the comment on the Rule
    // itself in `guna.rs` for why it is a documented equivalent mutant,
    // kept for faithfulness to the sUtra rather than for engine coverage.
    //
    // Asserted on the LOG rather than on a surface or a stem, deliberately.
    // Both are already rewritten by the time `derive` returns -- 8.4.1 has
    // taken Snam's `n` to `R` -- and neither settles until 8.2.31 and
    // 8.3.13 land in the next task. Whether the Agama fired is the claim,
    // and the log states it directly. The surfaces get asserted next task,
    // in `trh_lat_reaches_its_three_shapes`.
    //
    // 6.1.87 is asserted alongside because for this root only its im arm
    // can fire: SHAP holds Snam, which is not Thematic, so the junction arm
    // declines. The two rules stand or fall together.
    fn fired(la: Lakara, pu: Purusha, va: Vacana) -> (bool, bool) {
        let d = dhatus()
            .iter()
            .find(|d| d.dhatupatha == "07.0018")
            .expect("07.0018 is curated");
        let p = derive(d, la, Pada::Parasmaipada, pu, va)
            .into_iter()
            .next()
            .expect("every enumerable cell derives at least one branch");
        let has = |id: &str| p.log.iter().any(|step| step.sutra == id);
        (has("7.3.92"), has("6.1.87"))
    }

    // FIRES: hal-initial, pit, sArvadhAtuka, not Ngit.
    for (la, pu, va, why) in [
        (Lakara::Lat, Purusha::Prathama, Vacana::Eka, "ti"),
        (Lakara::Lat, Purusha::Madhyama, Vacana::Eka, "si"),
        (Lakara::Lat, Purusha::Uttama, Vacana::Eka, "mi"),
        // laN tip's apRkta `t`. 8.2.23 saMyogAntasya lopaH eats it, but not
        // until the tripAdI -- one stage BELOW this rule -- so the hal test
        // still sees it here. That ordering is pinned again in trace.rs.
        (Lakara::Lan, Purusha::Prathama, Vacana::Eka, "t"),
    ] {
        assert_eq!(fired(la, pu, va), (true, true), "{why}");
    }

    // DECLINES on the hal conjunct: the ending is vowel-initial.
    for (la, pu, va, why) in [
        (Lakara::Lan, Purusha::Uttama, Vacana::Eka, "am -> atfRaham"),
        (Lakara::Lot, Purusha::Uttama, Vacana::Eka, "Ani -> tfRahAni"),
    ] {
        assert_eq!(fired(la, pu, va), (false, false), "{why}");
    }

    // DECLINES here, but NOT an isolated control for the pit conjunct:
    // 1.2.4 makes tas/Ta/vas Ngit as well as apit, so the Ngit conjunct
    // below already rejects this cell on its own. Disabling the pit check
    // alone would not change this outcome -- see guna.rs's comment on the
    // Rule for why that conjunct is a documented equivalent mutant. Kept
    // as a cell anyway because it is still the textbook example of what
    // the pit clause names (6.4.111 takes Snam's `a` instead of the Agama
    // going in), even though it does not isolate that clause.
    assert_eq!(
        fired(Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
        (false, false),
        "tas -> tfRQaH"
    );

    // DECLINES on the Ngit conjunct, which the pit conjunct does NOT cover:
    // under yAsuT the ending's own `t` is still pit, and it is the Agama
    // that carries the N. The ending is hal-initial too, so this cell
    // isolates the fourth conjunct exactly.
    assert_eq!(
        fired(Lakara::VidhiLin, Purusha::Prathama, Vacana::Eka),
        (false, false),
        "yAsuT -> tfMhyAt"
    );
}

#[test]
fn trh_lat_reaches_its_three_shapes() {
    // The three tails √tṛh's laṭ splits into, one assertion each, and the
    // reason 8.2.31's *jhali* condition has to be a real guard:
    //
    //   tfReQi   `h` before the jhal `t`     -> 8.2.31, then 8.3.13
    //   tfRekzi  `h` before `s`              -> 8.2.31, then 8.2.41's Q arm
    //   tfRehmi  `h` before `m`, NOT a jhal  -> 8.2.31 declines, `h` stays
    //
    // tfRehmi is the load-bearing one: an 8.2.31 that fired on every `h`
    // would give *tfReQmi, a form that looks no less Sanskrit than the
    // right one.
    // Every laṭ cell of this root is single-form, so `form_g` (which goes
    // through `sole`) is the right helper: a cell that unexpectedly gains
    // an optional branch fails loudly here rather than having its first
    // branch read silently.
    let lat = |pu, va| form_g("07.0018", Lakara::Lat, pu, va);

    assert_eq!(lat(Purusha::Prathama, Vacana::Eka), "tfReQi");
    assert_eq!(lat(Purusha::Madhyama, Vacana::Eka), "tfRekzi");
    assert_eq!(lat(Purusha::Uttama, Vacana::Eka), "tfRehmi");
    // The apit cells, where 6.4.111 runs instead of the āgama and 8.3.13
    // still fires -- on the `Q` that 8.4.41 makes out of 8.2.40's `D`.
    assert_eq!(lat(Purusha::Prathama, Vacana::Dvi), "tfRQaH");
    // And the one where nothing retroflexes at all: `h` before a vowel.
    assert_eq!(lat(Purusha::Prathama, Vacana::Bahu), "tfMhanti");
}
