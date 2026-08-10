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

pub(super) fn form(code: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.id == code).unwrap();
    sole(derive(d, Lakara::Lat, Pada::Parasmaipada, pu, va)).text()
}

// `pub(super)` — the narrowest visibility that lets `mod.rs` re-export it
// as `crate::tinanta::form_g`; `anga.rs` and `tripadi.rs` import it by that
// stable path. The other helpers carry the same visibility so a stage test
// module can import any of them the same way.
pub(super) fn form_g(code: &str, la: Lakara, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.id == code).unwrap();
    sole(derive(d, la, d.pada, pu, va)).text()
}

/// `form_g` for a cell an optional rule forks: same lookup, `declined`
/// instead of `sole`.
pub(super) fn form_g_forked(
    code: &str,
    la: Lakara,
    pu: Purusha,
    va: Vacana,
    branches: usize,
) -> String {
    let d = dhatus().iter().find(|d| d.id == code).unwrap();
    declined(derive(d, la, d.pada, pu, va), branches).text()
}

pub(super) fn lin_form(code: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.id == code).unwrap();
    sole(derive(d, Lakara::VidhiLin, Pada::Parasmaipada, pu, va)).text()
}

pub(super) fn lat_a_form(code: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.id == code).unwrap();
    sole(derive(d, Lakara::Lat, Pada::Atmanepada, pu, va)).text()
}

pub(super) fn lot_a_form(code: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.id == code).unwrap();
    sole(derive(d, Lakara::Lot, Pada::Atmanepada, pu, va)).text()
}

pub(super) fn lin_a_form(code: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.id == code).unwrap();
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
#[test]
fn tinanta_rule_order_is_pinned() {
    let expected = [
        "1.3.12", "1.3.78", "3.4.78", "1.3.9", "1.2.4", "3.4.85", "3.4.108", "3.4.105", "3.4.106",
        "3.4.101", "3.4.99", "3.4.87", "3.4.89", "3.4.86", "3.4.100", "3.4.80", "3.4.79", "3.4.91",
        "3.4.93", "3.4.90", "3.4.92", "3.4.103", "3.4.102", "7.1.35", "3.1.69", "3.1.73", "3.1.77",
        "3.1.78", "3.1.81", "3.1.68", "2.4.72", "3.4.111", "3.1.83", "1.2.4", "6.4.71", "6.4.72",
        "7.3.100", "7.1.5", "7.1.6", "7.1.3", "7.2.79", "7.2.80", "7.2.81", "6.4.23", "7.4.21",
        "7.3.84", "7.3.86", "7.3.84", "6.4.87", "6.4.77", "6.1.78", "7.3.101", "6.4.112",
        "6.4.113", "6.1.101", "6.1.96", "6.1.90", "6.1.97", "6.1.87", "6.1.66", "6.4.105",
        "6.4.106", "6.4.107", "6.4.101", "6.4.111", "8.2.77", "8.2.23", "8.2.25", "8.2.39",
        "8.3.15", "8.3.24", "8.3.59", "8.4.53", "8.4.55", "8.4.1", "8.4.2", "8.4.58", "8.4.56",
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
    let expected = ["7.1.35", "3.4.111", "6.4.107", "8.4.56"];
    assert_eq!(actual, expected);
}

#[test]
fn divadi_tudadi_present_third_singular() {
    // Guṇa blocked by 1.1.5 (śyan/śa are ṅit): kup→kupyati NOT kopyati,
    // tud→tudati NOT todati, juṣ→juṣate NOT joṣate.
    assert_eq!(
        form_g("naS", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "naSyati"
    );
    assert_eq!(
        form_g("kup", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "kupyati"
    );
    assert_eq!(
        form_g("man", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "manyate"
    );
    assert_eq!(
        form_g("yuD", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "yuDyate"
    );
    assert_eq!(
        form_g("vid", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "vidyate"
    );
    assert_eq!(
        form_g("tud", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "tudati"
    );
    assert_eq!(
        form_g("liK", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "liKati"
    );
    assert_eq!(
        form_g("viS", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "viSati"
    );
    assert_eq!(
        form_g("juz", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "juzate"
    );
    assert_eq!(
        form_g("vij", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "vijate"
    );
    assert_eq!(
        form_g("gur", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
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
        form_g("naS", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "naSyanti"
    );
    assert_eq!(
        form_g("tud", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "tudanti"
    );
    assert_eq!(
        form_g("juz", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "juzante"
    );
    // 1st singular (7.3.101 ato dIrgho yaYi: śyan/śa `a` + `mi` → `Ami`).
    assert_eq!(
        form_g("tud", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "tudAmi"
    );
    // Ātmanepada uttama-eka (6.1.97 a+e para-rūpa: śyan `ya` + `e` → `ye`).
    assert_eq!(
        form_g("man", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "manye"
    );
    // 7.2.81 Ato NitaH: ātmanepada dual Ate→iyte, then coalesced.
    assert_eq!(
        form_g("man", Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
        "manyete"
    );
    // 7.2.80 ato yeyaH: vidhiliṅ yA→iy after śyan's `ya`.
    assert_eq!(
        form_g_forked("kup", Lakara::VidhiLin, Purusha::Prathama, Vacana::Eka, 2),
        "kupyed"
    );
    // 6.4.105 ato heH: imperative hi-elision after śyan's `ya`.
    assert_eq!(
        form_g_forked("naS", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "naSya"
    );
}

#[test]
fn div_lengthens_before_syan() {
    assert_eq!(
        form_g("div", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "dIvyati"
    );
    // laṅ: augment does not disturb the upadhā i.
    assert_eq!(
        form_g_forked("div", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2),
        "adIvyad"
    );
}

#[test]
fn adadi_luk_present_no_junction_cells() {
    // ā-final adādi roots: śap is luk'd (2.4.72), the ending attaches to
    // the root directly. These cells need only the luk (no ā+a junction).
    assert_eq!(
        form_g("yA", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "yAti"
    );
    assert_eq!(
        form_g("yA", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
        "yAsi"
    );
    assert_eq!(
        form_g("yA", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "yAmi"
    );
    // laṅ: aṭ-augment (yā is consonant-initial) → ayā; ending attaches.
    assert_eq!(
        form_g_forked("yA", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2),
        "ayAd"
    );
    // loṭ 2sg: hi does NOT elide after ā (6.4.105 needs short a) → yāhi.
    assert_eq!(
        form_g_forked("yA", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "yAhi"
    );
}

#[test]
fn adadi_root_final_a_coalesces_with_vowel_endings() {
    // ā + a(nti) → ā : yānti (laṭ 3pl), yAntu (loṭ 3pl), ayAn (laṅ 3pl).
    assert_eq!(
        form_g("yA", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "yAnti"
    );
    assert_eq!(
        form_g("yA", Lakara::Lot, Purusha::Prathama, Vacana::Bahu),
        "yAntu"
    );
    // Now forks: 3.4.111 (Task 4) adds the Śākaṭāyana jus branch (ayuH,
    // pinned in `paradigm.rs`'s ALTERNATES). Branch 0 is still the declined
    // derivation.
    assert_eq!(
        form_g_forked("yA", Lakara::Lan, Purusha::Prathama, Vacana::Bahu, 2),
        "ayAn"
    );
    // ā + A(ṭ) → ā : loṭ uttama-eka takes āṭ (yA + Ani → yAni).
    assert_eq!(
        form_g("yA", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
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
    for code in ["yA", "vA"] {
        let d = dhatus().iter().find(|d| d.id == code).unwrap();
        for pu in [Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama] {
            for va in [Vacana::Eka, Vacana::Dvi, Vacana::Bahu] {
                let expected = if pu == Purusha::Prathama && va == Vacana::Eka {
                    2
                } else {
                    1
                };
                let p = declined(derive(d, Lakara::VidhiLin, d.pada, pu, va), expected);
                assert!(!p.blocked, "{code} vidhiliṅ {pu:?} {va:?} was blocked");
                assert!(!p.log.is_empty(), "{code} vidhiliṅ ran no rules");
                assert!(
                    !p.text().is_empty(),
                    "{code} vidhiliṅ {pu:?} {va:?} is empty"
                );
            }
        }
    }
    assert_eq!(
        form_g("yA", Lakara::VidhiLin, Purusha::Prathama, Vacana::Bahu),
        "yAyuH"
    );
    assert_eq!(
        form_g("yA", Lakara::VidhiLin, Purusha::Uttama, Vacana::Eka),
        "yAyAm"
    );
}

#[test]
fn cartva_turns_d_to_t_before_khar() {
    // √ad laṭ: 3sg atti (d+t), 2sg atsi (d+s), 2pl atTa (d+T).
    assert_eq!(
        form_g("ad", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "atti"
    );
    assert_eq!(
        form_g("ad", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
        "atsi"
    );
    assert_eq!(
        form_g("ad", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
        "atTa"
    );
    // Not before a non-khar (m/v) or a vowel: admi, adanti stay.
    assert_eq!(
        form_g("ad", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "admi"
    );
    assert_eq!(
        form_g("ad", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "adanti"
    );
}

#[test]
fn her_dhih_gives_addhi_for_consonant_root() {
    // √ad loṭ 2sg: 3.4.87 si→hi, 6.4.105 declines (d, not short a),
    // 6.4.101 hi→Di → adDi.
    assert_eq!(
        form_g_forked("ad", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "adDi"
    );
    // Thematic root unaffected: √bhū loṭ 2sg is Bava (hi luk'd by 6.4.105).
    assert_eq!(
        form_g_forked("BU", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "Bava"
    );
}

#[test]
fn adadi_lan_singular_a_augment() {
    // √ad laṅ 3sg Adad, 2sg AdaH — the inserted `a` blocks the saṃyogānta
    // collapse (Adt/Ads → Ad) and cartva (d now before `a`, not a khar).
    assert_eq!(
        form_g_forked("ad", Lakara::Lan, Purusha::Prathama, Vacana::Eka, 2),
        "Adad"
    );
    assert_eq!(
        form_g("ad", Lakara::Lan, Purusha::Madhyama, Vacana::Eka),
        "AdaH"
    );
    // Dual/plural keep the direct junction (multi-char endings, no a-augment):
    // cartva gives Attam/AttAm; 1sg Adam untouched.
    assert_eq!(
        form_g("ad", Lakara::Lan, Purusha::Madhyama, Vacana::Dvi),
        "Attam"
    );
    assert_eq!(
        form_g("ad", Lakara::Lan, Purusha::Prathama, Vacana::Dvi),
        "AttAm"
    );
    assert_eq!(
        form_g("ad", Lakara::Lan, Purusha::Uttama, Vacana::Eka),
        "Adam"
    );
}

#[test]
fn bhu_3sg_is_bhavati() {
    assert_eq!(form("BU", Purusha::Prathama, Vacana::Eka), "Bavati");
}
#[test]
fn bhu_1pl_is_bhavamah() {
    assert_eq!(form("BU", Purusha::Uttama, Vacana::Bahu), "BavAmaH");
}
#[test]
fn smr_3sg_is_smarati() {
    assert_eq!(form("smf", Purusha::Prathama, Vacana::Eka), "smarati");
}
#[test]
fn pat_3du_is_patatah() {
    assert_eq!(form("paW", Purusha::Prathama, Vacana::Dvi), "paWataH");
}
#[test]
fn bhu_3pl_is_bhavanti() {
    assert_eq!(form("BU", Purusha::Prathama, Vacana::Bahu), "Bavanti");
}
#[test]
fn shap_is_pit_and_bhvadi_guna_survives() {
    // Regression guard for Task 3: adding the guṇa-block mechanism must
    // not disturb bhvādi. śap is pit, so 7.3.84 still fires for BU.
    assert_eq!(form("BU", Purusha::Prathama, Vacana::Eka), "Bavati");
    let d = dhatus().iter().find(|d| d.id == "vft").unwrap();
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
    assert_eq!(form("ji", Purusha::Prathama, Vacana::Eka), "jayati");
}

#[test]
fn trace_is_recorded() {
    let d = dhatus().iter().find(|d| d.id == "BU").unwrap();
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
                    for p in derive(d, lakara, d.pada, purusha, vacana) {
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
        form_g_forked("BU", Lakara::VidhiLin, Purusha::Prathama, Vacana::Eka, 2),
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
        assert_eq!(lin_form("BU", pu, va), want, "{pu:?} {va:?}");
    }
}

#[test]
fn pada_sanction_blocks_wrong_pada_derivations() {
    // 1.3.12/1.3.78: derivation is the source of truth for pada. A
    // wrong-pada derive must not silently produce a surface form.
    let labh = dhatus().iter().find(|d| d.id == "laB").unwrap();
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

    let bhu = dhatus().iter().find(|d| d.id == "BU").unwrap();
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
    let bhu = dhatus().iter().find(|d| d.id == "BU").unwrap();
    let p = sole(derive(
        bhu,
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    ));
    assert_eq!(p.log.first().unwrap().sutra, "1.3.78");

    let labh = dhatus().iter().find(|d| d.id == "laB").unwrap();
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
        assert_eq!(lat_a_form("laB", pu, va), form, "{pu:?} {va:?}");
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
        assert_eq!(lot_a_form("laB", pu, va), form, "{pu:?} {va:?}");
    }
}

#[test]
fn savabhyam_vamau_preempts_am_etah() {
    // 3.4.91 (se→sva, Dve→Dvam) is the apavāda ordered before 3.4.90:
    // reversed, se would become sAm and Dve DvAm.
    assert_eq!(lot_a_form("laB", Purusha::Madhyama, Vacana::Eka), "laBasva");
    assert_eq!(
        lot_a_form("laB", Purusha::Madhyama, Vacana::Bahu),
        "laBaDvam"
    );
}

#[test]
fn am_etah_is_lot_only() {
    // laṭ's te/Ate must NOT become tAm/AtAm.
    assert_eq!(lat_a_form("laB", Purusha::Prathama, Vacana::Eka), "laBate");
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
        assert_eq!(lin_a_form("laB", pu, va), form, "{pu:?} {va:?}");
    }
}

#[test]
fn siyut_survives_salopa_as_long_i() {
    // sIyta → (7.2.79) Iyta: 6.1.87's widened guard must accept the
    // long I (yāsuṭ's chain produced short iy via 7.2.80).
    let p = {
        let d = dhatus().iter().find(|d| d.id == "laB").unwrap();
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
        form_g("As", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
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
            form_g("As", Lakara::VidhiLin, pu, va),
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
            form_g("As", Lakara::VidhiLin, pu, va),
            want,
            "{pu:?} {va:?}"
        );
    }
}

#[test]
fn vrt_lat_uses_laghupadha_guna() {
    // vft's f is PENULTIMATE (upadha), not final like smf's: guna comes
    // from 7.3.86 pugantalaghUpaDasya ca, not 7.3.84.
    assert_eq!(lat_a_form("vft", Purusha::Prathama, Vacana::Eka), "vartate");
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
        form_g("SI", Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
        "SayAte"
    );
    assert_eq!(
        form_g("SI", Lakara::VidhiLin, Purusha::Prathama, Vacana::Bahu),
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
        form_g("SI", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "Sete"
    );
    // laṅ: 6.4.71 has already prefixed the aṭ-augment, so the aṅga is
    // `aSI` when 7.4.21 runs — the guard must match on the tail, not the
    // whole string.
    assert_eq!(
        form_g("SI", Lakara::Lan, Purusha::Prathama, Vacana::Eka),
        "aSeta"
    );
}

// --- cartva, her-dhih, a-augment guard pins for 5c adadi rules -------
#[test]
fn cartva_guard_is_khar_only_not_m_or_vowel() {
    // Over-application killer: d before `m` (admi) or vowel (adanti) must NOT
    // cartva-ize. Under-application killer: d before `t` MUST (atti, not adti).
    assert_eq!(
        form_g("ad", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "admi"
    );
    assert_eq!(
        form_g("ad", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "adanti"
    );
    assert_eq!(
        form_g("ad", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
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
        form_g("As", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
        "ADve"
    );
    assert_eq!(
        form_g("As", Lakara::Lan, Purusha::Madhyama, Vacana::Bahu),
        "ADvam"
    );
    assert_eq!(
        form_g("As", Lakara::Lot, Purusha::Madhyama, Vacana::Bahu),
        "ADvam"
    );
    // Guard boundary: the affix must be Dh-initial. A clean `s`-meets-`s`
    // cell is untouched, so 2sg stays Asse — the rule must not
    // over-apply.
    assert_eq!(
        form_g("As", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
        "Asse"
    );
}

#[test]
fn dhi_ca_fires_for_vas_and_only_before_dh() {
    // √vas is the sūtra's second witness: vas + Dve -> va + Dve -> vaDve.
    assert_eq!(
        form_g("vas", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
        "vaDve"
    );
    assert_eq!(
        form_g("vas", Lakara::Lan, Purusha::Madhyama, Vacana::Bahu),
        "avaDvam"
    );
    assert_eq!(
        form_g("vas", Lakara::Lot, Purusha::Madhyama, Vacana::Bahu),
        "vaDvam"
    );
    // The affix must be Dh-initial. These four cells put the same aṅga-
    // final `s` in front of `t`, `T` and `s` and it must survive intact —
    // and they are also the first pins that cartva (8.4.55) leaves an `s`
    // alone before a khar, an arm √ad and √ās could not reach.
    assert_eq!(
        form_g("vas", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "vaste"
    );
    assert_eq!(
        form_g("vas", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
        "vasse"
    );
    assert_eq!(
        form_g("vas", Lakara::Lan, Purusha::Madhyama, Vacana::Eka),
        "avasTAH"
    );
    assert_eq!(
        form_g("vas", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
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
        form_g("laB", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
        "laBaDve"
    );
    assert_eq!(
        form_g("laB", Lakara::Lan, Purusha::Madhyama, Vacana::Bahu),
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
        form_g_forked("ad", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
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
        form_g("As", Lakara::VidhiLin, Purusha::Madhyama, Vacana::Bahu),
        "AsIDvam"
    );
    assert_eq!(
        form_g("vas", Lakara::VidhiLin, Purusha::Madhyama, Vacana::Bahu),
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
        form_g_forked("yA", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "yAhi"
    );
}

#[test]
fn a_augment_does_not_leak_into_dual_or_plural() {
    // The single-char length guard: 2du ending `tam` must NOT get an `a`
    // (no *Adatam); it stays Attam via cartva.
    assert_eq!(
        form_g("ad", Lakara::Lan, Purusha::Madhyama, Vacana::Dvi),
        "Attam"
    );
}

#[test]
fn seventwone_five_atmanepada_3pl_uses_at_not_ant() {
    // 7.1.5 ātmanepadeṣv anataḥ: √ās (adādi, s-final) 3pl → Asate/Asata/
    // AsatAm (Ja → at, not the `ant` of 7.1.3). A-final thematic roots keep
    // `ante` (7.1.5 declines), so laB is unchanged.
    assert_eq!(
        form_g("As", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "Asate"
    );
    assert_eq!(
        form_g("As", Lakara::Lan, Purusha::Prathama, Vacana::Bahu),
        "Asata"
    );
    assert_eq!(
        form_g("As", Lakara::Lot, Purusha::Prathama, Vacana::Bahu),
        "AsatAm"
    );
    // Guard boundary: a-final ātmanepada aṅga still takes 7.1.3's `ante`.
    assert_eq!(
        form_g("laB", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "laBante"
    );
}

#[test]
fn anatah_declines_for_a_final_atmanepada_angas() {
    // 7.1.5's "anataḥ" arm: every a-final (thematic / vikaraṇa-buffered)
    // ātmanepada 3pl keeps 7.1.3's `ante`. Pins that the guard reads the
    // preceding segment's `a`, not the consonant-final root.
    assert_eq!(
        form_g("laB", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "laBante"
    );
    assert_eq!(
        form_g("man", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "manyante"
    );
    assert_eq!(
        form_g("juz", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "juzante"
    );
}

#[test]
fn voiced_junction_does_not_touch_non_jhas_or_non_jhal_junctions() {
    // Under-application guard: `s` before the non-jhaś `s`/`th`/`v` of
    // se/sva/thās stays `s` (Asse, Assva, AsTAH) — only a jhaś triggers it.
    assert_eq!(
        form_g("As", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
        "Assva"
    );
    assert_eq!(
        form_g("As", Lakara::Lan, Purusha::Madhyama, Vacana::Eka),
        "AsTAH"
    );
    // Over-application guard: √ad is parasmaipada; its voiceless junctions
    // stay cartva's business:
    assert_eq!(
        form_g("ad", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "atti"
    );
}

#[test]
fn shings_jha_takes_the_rut_augment() {
    // 7.1.6 śīṅo ruṭ: the *jha* (3pl ātmanepada) of √śī takes the ruṭ
    // augment. 7.1.5 has just turned the leading J into `at` (Je → ate);
    // ruṭ's `r` prefixes that: Se + r + ate → Serate.
    assert_eq!(
        form_g("SI", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "Serate"
    );
    assert_eq!(
        form_g("SI", Lakara::Lot, Purusha::Prathama, Vacana::Bahu),
        "SeratAm"
    );
    assert_eq!(
        form_g("SI", Lakara::Lan, Purusha::Prathama, Vacana::Bahu),
        "aSerata"
    );
}

#[test]
fn shings_vidhilin_3pl_takes_no_rut() {
    // 3.4.105 jhasya ran replaces the jha with `ran` long before the 7.x
    // band, so 7.1.5 never fires in vidhiliṅ and ruṭ cannot attach:
    // SayIran, NOT *SayIraran.
    assert_eq!(
        form_g("SI", Lakara::VidhiLin, Purusha::Prathama, Vacana::Bahu),
        "SayIran"
    );
}

#[test]
fn shatva_retroflexes_the_endings_s_after_shings_e() {
    // 8.3.59 ādeśapratyayayoḥ: the `s` of a pratyaya retroflexes after a
    // non-a/ā vowel. With the aṅga guṇated to `Se`, the `se` and `sva`
    // endings meet an `e` → Seze, Sezva.
    assert_eq!(
        form_g("SI", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
        "Seze"
    );
    assert_eq!(
        form_g("SI", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
        "Sezva"
    );
}

#[test]
fn rudhadi_strong_cells() {
    // The strong stem is śnam with its `a` intact. kft needs no new rule at
    // all beyond 3.1.78 — 8.4.1 ṇatva already fires across the ANGA/SHAP
    // junction, exactly as it does for kryādi's vf + nA → vfRAti.
    assert_eq!(
        form_g("kft", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "kfRatti"
    );
    assert_eq!(
        form_g("kft", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "kfRatmi"
    );
    // √hiṃs needs 6.4.23: hins + śnam is hinans, and the root's own n comes
    // back out.
    assert_eq!(
        form_g("his", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
        "hinasti"
    );
    assert_eq!(
        form_g("his", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
        "hinasAni"
    );
    // The ātmanepada arm's strong cells keep śnam's `a` too.
    assert_eq!(
        form_g("Kid", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
        "KinadE"
    );
}

#[test]
fn rudhadi_weak_cells_lose_shnams_a() {
    // 6.4.111 fires before a kṅit sārvadhātuka and makes the strong/weak
    // split visible. These are the cells 8.4.65 does NOT fork, so they are
    // safe to assert with `form_g` at this stage.
    assert_eq!(
        form_g("kft", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "kfntanti"
    );
    assert_eq!(
        form_g("kft", Lakara::Lat, Purusha::Uttama, Vacana::Dvi),
        "kfntvaH"
    );
    assert_eq!(
        form_g("Kid", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
        "Kinde"
    );
    assert_eq!(
        form_g("Kid", Lakara::VidhiLin, Purusha::Prathama, Vacana::Eka),
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
        form_g("his", Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
        "hiMstaH"
    );
    assert_eq!(
        form_g("his", Lakara::VidhiLin, Purusha::Prathama, Vacana::Bahu),
        "hiMsyuH"
    );
    assert_eq!(
        form_g("kft", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
        "kfntanti"
    );
}

#[test]
fn rudhadi_lot_madhyama_eka_takes_jashtva() {
    // 6.4.101 her DiH makes the ending `Di`; the weak stem's final `t`
    // meets it and is voiced-aspirated to `d` by 8.4.53. This is the rule
    // commit 9b7adee deleted as unreachable once 8.2.25 dhi ca replaced
    // slice 5d's analysis — √kṛt is its first genuine witness.
    //
    // 7.1.35 tātaṅ optionally forks this cell (parasmaipada loṭ madhyama
    // eka), independently of anything in this slice; branch 0 is the
    // derivation with no optional rule applied.
    assert_eq!(
        form_g_forked("kft", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "kfndDi"
    );
    // √hiṃs reaches the same cell through 8.2.25 instead: its stem-final
    // `s` is ELIDED before the Dh-initial ending, not voiced. Third witness
    // for that rule, on a stem shape it has not seen.
    assert_eq!(
        form_g_forked("his", Lakara::Lot, Purusha::Madhyama, Vacana::Eka, 3),
        "hinDi"
    );
}
