//! Reduplication: 6.1.10, 7.4.62 — dvitva and the rules that reshape the
//! abhyāsa.
//!
//! Ordered AFTER 3.1.68 (ending at `ENDING`, śap at `SHAP` — empty on
//! exactly the path this stage cares about) and BEFORE `anga`, so 6.4.71
//! always sees a finished abhyāsa in `ABHYASA` and 7.1.4 reads a real
//! `Tag::Abhyasta`. See `super::terms`.
//!
//! DVITVA RUNS BEFORE GUṆA. vidyut-prakriya copies the already-guṇated
//! stem and shortens the copy back by 7.4.59 (*ho ho* → *hu ho*); this
//! engine follows the Kaumudī's order — ślu, dvitva, abhyāsa-kārya, then
//! guṇa — and copies the bare root, so 7.4.59 *hrasvaḥ* (slice 3b) fires
//! only on the long-vowel roots it names. The two orders give the same
//! forms for all 26 juhotyādi roots (the spec's appendix probe); the audit
//! compares form sets, not traces, so the divergence is visible only in
//! this engine's own trace pins, which pin THIS order (juhoti: 6.1.10 <
//! 7.4.62 < 7.3.84).
//!
//! 6.1.4 *pūrvo'bhyāsaḥ* and 6.1.5 *ubhe abhyastam* are saṁjñā verdicts
//! and live as tags set here (`Tag::Abhyasa`, `Tag::Abhyasta`), not as
//! steps — the 3.4.113 precedent.

use crate::rule::{Rule, RuleKind};
use crate::term::Tag;
use crate::tinanta::sound::{cutva_of, hrasva_of, is_vowel};
use crate::tinanta::terms::{ABHYASA, ANGA, SHAP};

pub(crate) static ABHYASA_RULES: &[Rule] = &[
    // 6.1.10 ślau: after ślu the aṅga is doubled (6.1.1 ekāco dve
    // prathamasya, the first ekāc). The copy goes into the permanent
    // `ABHYASA` slot, empty for every other gaṇa; it is tagged Abhyasa
    // (6.1.4 — the earlier of the two) and both it and the root are tagged
    // Abhyasta (6.1.5).
    //
    // NARROW: copies the whole aṅga text. Every juhotyādi root is a single
    // ekāc when this rule fires (hu, ki, BI, dA, … — 6.1.2 ajāder
    // dvitīyasya has no customer here either), so "the first ekāc" and
    // "the aṅga" coincide. liṭ is the first slice that reduplicates a
    // polysyllabic aṅga and must implement the ekāc cut then.
    //
    // Reads Tag::Slu, not "SHAP is empty": adādi's śap is empty too (2.4.72,
    // luk) and does not reduplicate — atti, not *atatti. SHAP is indexed
    // directly, as every post-3.1.68 rule does: the slot always exists here.
    Rule {
        id: "6.1.10",
        name: "SlO",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[SHAP].has(Tag::Slu) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ABHYASA].text = p.terms[ANGA].text.clone();
            p.terms[ABHYASA].add(Tag::Abhyasa);
            p.terms[ABHYASA].add(Tag::Abhyasta);
            p.terms[ANGA].add(Tag::Abhyasta);
            p.record("6.1.10", "SlO", before);
            true
        },
    },
    // 7.4.60 halādiḥ śeṣaḥ: of the abhyāsa's initial consonant cluster only
    // the first consonant remains. hrI → hI, which 7.4.59 then shortens to
    // hi and 7.4.62 palatalizes to Ji (jihreti).
    //
    // √hrī is the ONLY cluster-initial row in the whole gaṇa — the other
    // twenty-five all begin with a single consonant or a vowel — so this
    // rule will never gain a second witness, and
    // `haladih_shesha_keeps_only_the_first_consonant_of_the_abhyasa` carries
    // the load a second root would otherwise share.
    //
    // The no-op guard is 8.4.53's: for a single-consonant abhyāsa the result
    // equals the input, and the rule must record nothing there or every √hu,
    // √ki and √bhī trace grows a step and the 3636 priors break.
    Rule {
        id: "7.4.60",
        name: "halAdiH SezaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let s: Vec<char> = p.terms[ABHYASA].text.chars().collect();
            let Some(&first) = s.first() else {
                return false;
            };
            // *halādiḥ* names a consonant-initial abhyāsa. 3d's √ṛ is the
            // vowel-initial row and must fall through untouched.
            if is_vowel(first) {
                return false;
            }
            let tail: String = s.iter().skip(1).skip_while(|c| !is_vowel(**c)).collect();
            let t = format!("{first}{tail}");
            if t == p.terms[ABHYASA].text {
                return false;
            }
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("7.4.60", "halAdiH SezaH", before);
            true
        },
    },
    // 7.4.59 hrasvaḥ: the abhyāsa's vowel becomes hrasva. BI → Bi
    // (bibheti), and hI → hi on 7.4.60's output (jihreti). The AṄGA's own
    // vowel is untouched — that is what leaves √bhī a long I for 6.4.82 to
    // find (bibhyati) and what 6.4.115 optionally shortens later.
    //
    // Ordered AFTER 7.4.60 (vidyut's order; the forms agree either way, so
    // the trace pins are what hold it) and BEFORE 7.4.62.
    //
    // Same no-op guard: √hu and √ki are already hrasva, and `hrasva_of`
    // returning None for a short vowel is what makes the guard a one-lookup
    // test. See that function for why the ec arm of 1.1.48 is absent.
    Rule {
        id: "7.4.59",
        name: "hrasvaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let t: String = p.terms[ABHYASA]
                .text
                .chars()
                .map(|c| hrasva_of(c).unwrap_or(c))
                .collect();
            if t == p.terms[ABHYASA].text {
                return false;
            }
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("7.4.59", "hrasvaH", before);
            true
        },
    },
    // 7.4.62 kuhoś cuḥ: the abhyāsa's initial velar or `h` becomes the
    // palatal — ku → cu by place, and h → J (jh), which 8.4.54 abhyāse car
    // ca deaspirates to `j` in the tripādī: hu → Ju → ju (juhoti), ki → ci
    // (ciketi).
    //
    // *abhyāse* (anuvṛtti from 7.4.59) is structurally satisfied: `ABHYASA`
    // is non-empty exactly when 6.1.10 has filled it — the slot is the
    // saṁjñā's whole extension — so a Tag::Abhyasa clause here could never
    // be falsified and is deliberately not written, the same reason 7.4.21
    // omits its *sārvadhātuke* test.
    Rule {
        id: "7.4.62",
        name: "kuhoScuH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let Some(first) = p.terms[ABHYASA].text.chars().next() else {
                return false;
            };
            let Some(cu) = cutva_of(first) else {
                return false;
            };
            let before = p.snapshot();
            let rest: String = p.terms[ABHYASA].text.chars().skip(1).collect();
            p.terms[ABHYASA].text = format!("{cu}{rest}");
            p.record("7.4.62", "kuhoScuH", before);
            true
        },
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prakriya::Prakriya;
    use crate::term::Term;
    use crate::tinanta::rules;
    use crate::tinanta::terms::{ENDING, with_slots};

    /// The shape 2.4.75 leaves: root, an empty śap tagged Slu, the ending.
    fn slu_prakriya(root: &str, ending: &str) -> Prakriya {
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new(root), Term::new(""), Term::new(ending)]),
            ..Default::default()
        };
        p.terms[ANGA].add(Tag::Dhatu);
        p.terms[ANGA].add(Tag::Anga);
        p.terms[SHAP].add(Tag::Vikarana);
        p.terms[SHAP].add(Tag::Slu);
        p
    }

    #[test]
    fn slau_copies_the_anga_into_abhyasa_and_tags_both_terms() {
        // 6.1.10 SlO, with 6.1.4 and 6.1.5 as tags. All three tag writes are
        // asserted here: only ANGA's Abhyasta is read by a rule in this
        // slice (7.1.4), so the other two would otherwise be writes nothing
        // checks — and therefore unkillable under mutation.
        let mut p = slu_prakriya("hu", "ti");
        let rule = rules().find(|r| r.id == "6.1.10").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "hu");
        assert_eq!(p.terms[ANGA].text, "hu");
        assert!(p.terms[ABHYASA].has(Tag::Abhyasa));
        assert!(p.terms[ABHYASA].has(Tag::Abhyasta));
        assert!(p.terms[ANGA].has(Tag::Abhyasta));
        assert!(!p.terms[ANGA].has(Tag::Abhyasa));
        assert_eq!(p.terms[ENDING].text, "ti");
        assert_eq!(p.text(), "huhuti");
        assert_eq!(p.log.last().unwrap().sutra, "6.1.10");
    }

    #[test]
    fn slau_declines_for_luk_and_for_a_live_vikarana() {
        // adādi's empty śap is luk (2.4.72), not ślu: atti, not *atatti.
        let rule = rules().find(|r| r.id == "6.1.10").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ad"), Term::new(""), Term::new("ti")]),
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "");
        // A thematic vikaraṇa: nothing to do either.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("BU"), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "");
    }

    #[test]
    fn kuhos_cuh_palatalises_the_abhyasa_initial_only() {
        // hu → Ju (jh; 8.4.54 makes it j in the tripādī), ki → ci. The
        // root's own initial is untouched: the sūtra names the abhyāsa.
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        let r_62 = rules().find(|r| r.id == "7.4.62").unwrap();
        for (root, expected) in [("hu", "Ju"), ("ki", "ci")] {
            let mut p = slu_prakriya(root, "ti");
            assert!((r_10.apply)(&mut p));
            assert!((r_62.apply)(&mut p), "{root}");
            assert_eq!(p.terms[ABHYASA].text, expected, "{root}");
            assert_eq!(p.terms[ANGA].text, root, "{root}");
            assert_eq!(p.log.last().unwrap().sutra, "7.4.62");
        }
    }

    #[test]
    fn kuhos_cuh_declines_without_an_abhyasa_and_on_a_non_velar() {
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        let r_62 = rules().find(|r| r.id == "7.4.62").unwrap();
        // No abhyāsa (the slot is empty): every other gaṇa.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ki"), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(r_62.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "");
        // An abhyāsa whose initial is neither ku nor h (slice 3c's dA):
        // 6.1.10 fires, 7.4.62 has nothing to see.
        let mut p = slu_prakriya("dA", "ti");
        assert!((r_10.apply)(&mut p));
        assert!(!(r_62.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "dA");
    }

    #[test]
    fn haladih_shesha_keeps_only_the_first_consonant_of_the_abhyasa() {
        // 7.4.60. √hrī's abhyāsa `hrI` loses its r: jihreti. This is the
        // ONLY cluster-initial row in the whole gaṇa, so this test is the
        // rule's only witness and no later slice adds a second.
        let mut p = slu_prakriya("hrI", "ti");
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        assert!((r_10.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "hrI");
        let r_60 = rules().find(|r| r.id == "7.4.60").unwrap();
        assert!((r_60.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "hI");
        assert_eq!(p.terms[ANGA].text, "hrI", "the aṅga is untouched");
        assert_eq!(p.log.last().unwrap().sutra, "7.4.60");
    }

    #[test]
    fn haladih_shesha_records_nothing_for_a_single_initial_consonant() {
        // The no-op guard. √hu, √ki and √bhī all have one initial consonant,
        // so 7.4.60 must return false and leave the log empty — otherwise
        // every one of their traces grows a step and the 3636 priors break.
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        let r_60 = rules().find(|r| r.id == "7.4.60").unwrap();
        for root in ["hu", "ki", "BI"] {
            let mut p = slu_prakriya(root, "ti");
            assert!((r_10.apply)(&mut p));
            p.log.clear();
            assert!(!(r_60.apply)(&mut p), "{root}");
            assert_eq!(p.terms[ABHYASA].text, root, "{root}");
            assert!(p.log.is_empty(), "{root}");
        }
    }

    #[test]
    fn haladih_shesha_declines_for_a_vowel_initial_abhyasa() {
        // 3d's √ṛ is the vowel-initial row. *halādiḥ* names a consonant, so
        // the rule has nothing to keep and must not touch the term.
        let mut p = slu_prakriya("f", "ti");
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        assert!((r_10.apply)(&mut p));
        p.log.clear();
        let r_60 = rules().find(|r| r.id == "7.4.60").unwrap();
        assert!(!(r_60.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "f");
        assert!(p.log.is_empty());
    }

    #[test]
    fn hrasvah_shortens_the_abhyasa_vowel_and_leaves_the_anga_long() {
        // 7.4.59. √bhī: BI → Bi, with the aṅga's own I untouched — that is
        // what lets 6.4.82 fire on a long I later (bibhyati).
        let mut p = slu_prakriya("BI", "ti");
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        assert!((r_10.apply)(&mut p));
        let r_59 = rules().find(|r| r.id == "7.4.59").unwrap();
        assert!((r_59.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "Bi");
        assert_eq!(p.terms[ANGA].text, "BI", "the aṅga keeps its long vowel");
        assert_eq!(p.log.last().unwrap().sutra, "7.4.59");
    }

    #[test]
    fn hrasvah_records_nothing_for_an_already_short_abhyasa() {
        // The no-op guard, on 3a's two roots.
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        let r_59 = rules().find(|r| r.id == "7.4.59").unwrap();
        for root in ["hu", "ki"] {
            let mut p = slu_prakriya(root, "ti");
            assert!((r_10.apply)(&mut p));
            p.log.clear();
            assert!(!(r_59.apply)(&mut p), "{root}");
            assert_eq!(p.terms[ABHYASA].text, root, "{root}");
            assert!(p.log.is_empty(), "{root}");
        }
    }

    #[test]
    fn haladih_shesha_runs_before_hrasvah_and_both_before_kuhoshcuh() {
        // √hrī end to end through this stage: hrI → hI → hi → Ji, which
        // 8.4.54 finishes as `ji` in the tripādī. The order is vidyut's; the
        // forms agree under 7.4.60/7.4.59 either way, so this is the pin
        // that makes the order a checked fact rather than an accident.
        let mut p = slu_prakriya("hrI", "ti");
        for id in ["6.1.10", "7.4.60", "7.4.59", "7.4.62"] {
            let r = rules().find(|r| r.id == id).unwrap();
            assert!((r.apply)(&mut p), "{id}");
        }
        assert_eq!(p.terms[ABHYASA].text, "Ji");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, vec!["6.1.10", "7.4.60", "7.4.59", "7.4.62"]);
    }
}
