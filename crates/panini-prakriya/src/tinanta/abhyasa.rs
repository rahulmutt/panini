//! Reduplication: 6.1.10, 7.4.66, 7.4.60, 7.4.59, 7.4.62, 7.4.76, 7.4.77, 7.4.78 — dvitva and the
//! rules that reshape the abhyāsa.
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
//! only on the long-vowel roots it names. The same choice makes 7.4.66 *ur
//! at* (slice 3d) fire on every cell of a ṛ-root, where vidyut's fires only on
//! the kṅit ones. The two orders give the same
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
        bars: &[],
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
    // 7.4.66 ur at: the abhyāsa's ṛ-vowel becomes `a` — `ar` by 1.1.51
    // uraṇ raparaḥ, which (like guṇa's `ar`) is not credited. ṛ names ṝ too
    // (1.1.9's savarṇa): Bf → Bar, pF → par. 7.4.60 then elides the `r`.
    //
    // FIRST after 6.1.10, as vidyut orders it (7.4.66 < 7.4.60). It fires on
    // EVERY cell of a ṛ-root here, where vidyut's fires only on the kṅit
    // cells: vidyut copies the guṇated stem (`Bar` on bibharti) and finds no
    // ṛ to change, while this stage copies the bare root (the header's
    // dvitva-before-guṇa note). The forms agree; the trace pins hold this
    // order.
    //
    // Guarded on the sound, not the row: the sūtra names a vowel, and no
    // abhyāsa before slice 3d holds one. The no-op test is the whole guard —
    // an abhyāsa without `f`/`F`, and the empty slot of every other gaṇa,
    // comes back unchanged.
    Rule {
        id: "7.4.66",
        name: "urat",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let t = p.terms[ABHYASA].text.replace(['f', 'F'], "ar");
            if t == p.terms[ABHYASA].text {
                return false;
            }
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("7.4.66", "urat", before);
            true
        },
    },
    // 7.4.60 halādiḥ śeṣaḥ: of the abhyāsa's consonants only the first
    // remains. Two witnesses: √hrī's initial cluster (hrI → hI, which 7.4.59
    // then shortens to hi and 7.4.62 palatalizes to Ji: jihreti), and — new
    // in slice 3d — the `r` 7.4.66 leaves on every ṛ-root's abhyāsa (Bar →
    // Ba: bibharti, pa: piparti, Ga: jagharti).
    //
    // WIDENED in slice 3d from an initial-cluster trim to the sūtra: keep the
    // first consonant and every vowel, drop every other consonant. For a
    // single-ekāc abhyāsa (6.1.10's NARROW note) that is the whole rule.
    //
    // The no-op guard is 8.4.53's: for a single-consonant open abhyāsa the
    // result equals the input, and the rule must record nothing there or
    // every √hu, √ki and √bhī trace grows a step and the priors break.
    Rule {
        id: "7.4.60",
        name: "halAdiH SezaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let s: Vec<char> = p.terms[ABHYASA].text.chars().collect();
            let Some(&first) = s.first() else {
                return false;
            };
            // *halādiḥ* names a consonant-initial abhyāsa. vidyut elides √ṛ's
            // `r` too (ar → a on 03.0017, its 7.4.60), so whether this
            // fall-through survives is slice 3d2's to decide with that
            // witness; no curated row reaches it today.
            if is_vowel(first) {
                return false;
            }
            let t: String = std::iter::once(first)
                .chain(s[1..].iter().copied().filter(|c| is_vowel(*c)))
                .collect();
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
    //
    // NARROW, same deferral as 6.1.10's own note above: the map runs over
    // EVERY character of the abhyāsa, not just its vowel, which is harmless
    // only because every abhyāsa is still a single ekāc.
    Rule {
        id: "7.4.59",
        name: "hrasvaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
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
        bars: &[],
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
    // 7.4.76 bhṛñām it: the abhyāsa of √bhṛñ, √māṅ and √ohāṅ takes `i`.
    // Ba → Bi (bibharti, after 7.4.66 and 7.4.60), ma → mi (mimIte), and —
    // after 7.4.59 and 7.4.62 — Ja → Ji (jihIte).
    // vidyut's order is 7.4.59 → 7.4.62 → 7.4.76, which this stage's
    // sequence already is.
    //
    // KEYED BY ROW NUMBER (`ctx.dhatupatha`), not by `ANGA.text`: the sūtra
    // names three roots, and `03.0009 o~hA\k` enters the derivation as `hA`
    // exactly like `03.0008 o~hA\N` while taking no 7.4.76 (jahAti).
    //   03.0006 quBf\Y √bhṛñ (slice 3d: Ba → Bi, bibharti)
    //   03.0007 mA\N   √māṅ
    //   03.0008 o~hA\N √ohāṅ
    //
    // Every abhyāsa this reaches is a single ekāc (6.1.10's NARROW note), so
    // mapping each vowel to `i` is mapping THE vowel to `i`.
    Rule {
        id: "7.4.76",
        name: "BfYAm it",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !matches!(p.ctx.dhatupatha, "03.0006" | "03.0007" | "03.0008") {
                return false;
            }
            let t: String = p.terms[ABHYASA]
                .text
                .chars()
                .map(|c| if is_vowel(c) { 'i' } else { c })
                .collect();
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("7.4.76", "BfYAm it", before);
            true
        },
    },
    // 7.4.77 arti-pipartyoś ca: the abhyāsa of √ṛ (*arti*) and √pṛ
    // (*piparti*) takes `i` too. vidyut applies *piparti* to both pf\ and pF,
    // the two rows that spell it: after 7.4.66 and 7.4.60, pa → pi.
    //
    // KEYED BY ROW NUMBER, like 7.4.76 above it: by this stage `ANGA.text`
    // still reads `pf`/`pF`, but the sūtra names roots, and the precedent is
    // the number.
    //   03.0004 pF  √pṝ
    //   03.0005 pf\ √pṛ
    // `03.0017 f\` (√ṛ, *arti*) is the sūtra's first root and joins with its
    // witness in slice 3d2; an unwitnessed number here would be a mutation
    // survivor.
    //
    // After 7.4.76: the two name disjoint roots, so only the trace pins decide
    // the order, and they follow vidyut's.
    Rule {
        id: "7.4.77",
        name: "artipipartyoSca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !matches!(p.ctx.dhatupatha, "03.0004" | "03.0005") {
                return false;
            }
            let t: String = p.terms[ABHYASA]
                .text
                .chars()
                .map(|c| if is_vowel(c) { 'i' } else { c })
                .collect();
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("7.4.77", "artipipartyoSca", before);
            true
        },
    },
    // 7.4.78 bahulaṁ chandasi: in the Veda, the abhyāsa takes `i` variously.
    // A CHĀNDASA sūtra, applied here to √gā (`03.0026 gA\`) on the authority
    // of the Siddhānta-kaumudī, which derives jigAti by it. vidyut-prakriya
    // does the same and says so ("This is a chAndasa rule, but the SK applies
    // it to derive jigAti from gA, which is a Vedic root."). The Pāṇinian id
    // is kept and the source is recorded here: the 7.3.86 vikalpa-arm
    // precedent for a non-Aṣṭādhyāyī authority.
    //
    // *bahulam* is not modelled as a vikalpa: vidyut gives jig- only, and so
    // does the Kaumudī's form.
    //
    // KEYED BY ROW NUMBER, like 7.4.76 above it: the sūtra itself names no
    // root, and the application is to this one row. After 7.4.62: ja → ji.
    Rule {
        id: "7.4.78",
        name: "bahulaM Candasi",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            // 03.0026 gA\ (√gā)
            if p.ctx.dhatupatha != "03.0026" {
                return false;
            }
            let t: String = p.terms[ABHYASA]
                .text
                .chars()
                .map(|c| if is_vowel(c) { 'i' } else { c })
                .collect();
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("7.4.78", "bahulaM Candasi", before);
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
        // 7.4.60. √hrī's abhyāsa `hrI` loses its r: jihreti. This is the only cluster-initial row in the whole gaṇa;
        // slice 3d's ṛ-roots witness the rule's other arm, a non-initial
        // consonant after the vowel.
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
        // every one of their traces grows a step and the 3924 priors break.
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
        // √ṛ (slice 3d2) is the vowel-initial row, and vidyut does trim
        // its abhyāsa (ar → a); 3d2 revisits this guard. *halādiḥ* names a consonant, so
        // the rule has nothing to keep and must not touch the term. `f`
        // alone cannot kill the guard, though: a single-character abhyāsa
        // already declines via the no-op check below, guard or no guard.
        // `ap` is the guard's real witness — no curated root has this
        // shape, but without the guard a vowel-initial abhyāsa followed by
        // a consonant would fall through to the tail computation and be
        // wrongly truncated to `a`.
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        let r_60 = rules().find(|r| r.id == "7.4.60").unwrap();
        for root in ["f", "ap"] {
            let mut p = slu_prakriya(root, "ti");
            assert!((r_10.apply)(&mut p));
            p.log.clear();
            assert!(!(r_60.apply)(&mut p), "{root}");
            assert_eq!(p.terms[ABHYASA].text, root, "{root}");
            assert!(p.log.is_empty(), "{root}");
        }
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

    #[test]
    fn bhrnam_it_makes_the_abhyasa_vowel_i_for_the_three_rows_it_names() {
        // 7.4.76. √bhṛñ (03.0006), √māṅ (03.0007) and √ohāṅ (03.0008), after
        // 7.4.60, 7.4.59 and 7.4.62: Ba → Bi (bibharti), ma → mi (mimIte), Ja →
        // Ji (jihIte).
        let rule = rules().find(|r| r.id == "7.4.76").unwrap();
        for (root, number, abhyasa, want) in [
            ("Bf", "03.0006", "Ba", "Bi"),
            ("mA", "03.0007", "ma", "mi"),
            ("hA", "03.0008", "Ja", "Ji"),
        ] {
            let mut p = slu_prakriya(root, "te");
            p.ctx.dhatupatha = number;
            p.terms[ABHYASA].text = abhyasa.into();
            assert!((rule.apply)(&mut p), "{number}");
            assert_eq!(p.terms[ABHYASA].text, want, "{number}");
            assert_eq!(p.log.last().unwrap().sutra, "7.4.76");
        }
    }

    #[test]
    fn bhrnam_it_declines_for_the_other_ha_row_despite_identical_text() {
        // 03.0009 o~hA\k enters the derivation as `hA`, exactly like
        // 03.0008, and takes no 7.4.76: jahAti, not *jihAti. This is the test
        // that the number, not the text, decides.
        let rule = rules().find(|r| r.id == "7.4.76").unwrap();
        for number in ["03.0009", ""] {
            let mut p = slu_prakriya("hA", "ti");
            p.ctx.dhatupatha = number;
            p.terms[ABHYASA].text = "Ja".into();
            assert!(!(rule.apply)(&mut p), "{number:?}");
            assert_eq!(p.terms[ABHYASA].text, "Ja", "{number:?}");
            assert!(p.log.is_empty(), "{number:?}");
        }
    }

    #[test]
    fn bahulam_chandasi_makes_the_abhyasa_vowel_i_for_ga() {
        // 7.4.78, applied to √gā (03.0026) on the Kaumudī's authority. After
        // 7.4.62: ja → ji (jigAti).
        let rule = rules().find(|r| r.id == "7.4.78").unwrap();
        let mut p = slu_prakriya("gA", "ti");
        p.ctx.dhatupatha = "03.0026";
        p.terms[ABHYASA].text = "ja".into();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "ji");
        assert_eq!(p.log.last().unwrap().sutra, "7.4.78");
    }

    #[test]
    fn bahulam_chandasi_declines_off_its_row() {
        // Keyed by number: `03.0009` (jahAti) and the hand-built default must
        // keep their abhyāsa `a`.
        let rule = rules().find(|r| r.id == "7.4.78").unwrap();
        for (root, number, abhyasa) in [("gA", "", "ja"), ("hA", "03.0009", "Ja")] {
            let mut p = slu_prakriya(root, "ti");
            p.ctx.dhatupatha = number;
            p.terms[ABHYASA].text = abhyasa.into();
            assert!(!(rule.apply)(&mut p), "{number:?}");
            assert_eq!(p.terms[ABHYASA].text, abhyasa, "{number:?}");
            assert!(p.log.is_empty(), "{number:?}");
        }
    }

    #[test]
    fn ur_at_makes_the_abhyasa_r_vowel_ar() {
        // 7.4.66, with 1.1.51's r-rapara uncredited (as guṇa's `ar` is): the
        // copied root's ṛ-vowel, short or long, becomes `ar`. The aṅga is
        // untouched: the sūtra names the abhyāsa.
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        let rule = rules().find(|r| r.id == "7.4.66").unwrap();
        for (root, want) in [("Bf", "Bar"), ("pF", "par"), ("Gf", "Gar"), ("sf", "sar")] {
            let mut p = slu_prakriya(root, "ti");
            assert!((r_10.apply)(&mut p));
            assert!((rule.apply)(&mut p), "{root}");
            assert_eq!(p.terms[ABHYASA].text, want, "{root}");
            assert_eq!(p.terms[ANGA].text, root, "{root}: the aṅga is untouched");
            assert_eq!(p.log.last().unwrap().sutra, "7.4.66");
        }
    }

    #[test]
    fn ur_at_records_nothing_without_an_r_vowel() {
        // The no-op guard is the whole guard: every pre-3d abhyāsa, and the
        // empty slot of every other gaṇa, must come back unchanged and
        // unlogged, or every prior trace grows a step.
        let r_10 = rules().find(|r| r.id == "6.1.10").unwrap();
        let rule = rules().find(|r| r.id == "7.4.66").unwrap();
        for root in ["hu", "ki", "BI", "hrI", "dA", "gA"] {
            let mut p = slu_prakriya(root, "ti");
            assert!((r_10.apply)(&mut p));
            p.log.clear();
            assert!(!(rule.apply)(&mut p), "{root}");
            assert_eq!(p.terms[ABHYASA].text, root, "{root}");
            assert!(p.log.is_empty(), "{root}");
        }
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Bf"), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "");
        assert!(p.log.is_empty());
    }

    #[test]
    fn haladih_shesha_elides_every_consonant_but_the_first() {
        // 7.4.60, widened in slice 3d: the abhyāsa keeps its first hal and
        // loses every other one, final ones included. 7.4.66's `ar` is the
        // corpus witness (Bar → Ba, bibharti). `Bas` is √bhas's (slice 3f)
        // shape, a final consonant 7.4.66 did not put there; the rule must
        // not depend on where the consonant came from.
        let r_60 = rules().find(|r| r.id == "7.4.60").unwrap();
        for (abhyasa, want) in [
            ("Bar", "Ba"),
            ("par", "pa"),
            ("Gar", "Ga"),
            ("har", "ha"),
            ("sar", "sa"),
            ("Bas", "Ba"),
        ] {
            let mut p = slu_prakriya("Bf", "ti");
            p.terms[ABHYASA].text = abhyasa.into();
            assert!((r_60.apply)(&mut p), "{abhyasa}");
            assert_eq!(p.terms[ABHYASA].text, want, "{abhyasa}");
            assert_eq!(p.log.last().unwrap().sutra, "7.4.60");
        }
    }

    #[test]
    fn ur_at_runs_before_haladih_shesha() {
        // √bhṛ through the first three rules of this stage: Bf → Bf Bf →
        // Bar Bf → Ba Bf. The order is vidyut's (7.4.66 < 7.4.60), and it is
        // load-bearing: 7.4.60 first would find `Bf`, trim nothing, and leave
        // 7.4.66's `r` for good.
        let mut p = slu_prakriya("Bf", "ti");
        for id in ["6.1.10", "7.4.66", "7.4.60"] {
            let r = rules().find(|r| r.id == id).unwrap();
            assert!((r.apply)(&mut p), "{id}");
        }
        assert_eq!(p.terms[ABHYASA].text, "Ba");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, vec!["6.1.10", "7.4.66", "7.4.60"]);
    }

    #[test]
    fn arti_pipartyos_ca_makes_the_abhyasa_vowel_i_for_pr_and_prr() {
        // 7.4.77, *piparti*: vidyut applies it to both pf\ (03.0005) and pF
        // (03.0004). After 7.4.66 and 7.4.60: pa → pi.
        let rule = rules().find(|r| r.id == "7.4.77").unwrap();
        for (root, number) in [("pf", "03.0005"), ("pF", "03.0004")] {
            let mut p = slu_prakriya(root, "ti");
            p.ctx.dhatupatha = number;
            p.terms[ABHYASA].text = "pa".into();
            assert!((rule.apply)(&mut p), "{number}");
            assert_eq!(p.terms[ABHYASA].text, "pi", "{number}");
            assert_eq!(p.log.last().unwrap().sutra, "7.4.77");
        }
    }

    #[test]
    fn arti_pipartyos_ca_declines_off_its_rows() {
        // Keyed by number. √bhṛ's `i` is 7.4.76's, √sṛ keeps its `a`
        // (sasarti), and the hand-built default names no row. √ṛ (03.0017)
        // is the sūtra's other root and arrives with its witness in 3d2.
        let rule = rules().find(|r| r.id == "7.4.77").unwrap();
        for (root, number, abhyasa) in [
            ("Bf", "03.0006", "Ba"),
            ("sf", "03.0018", "sa"),
            ("pf", "", "pa"),
        ] {
            let mut p = slu_prakriya(root, "ti");
            p.ctx.dhatupatha = number;
            p.terms[ABHYASA].text = abhyasa.into();
            assert!(!(rule.apply)(&mut p), "{number:?}");
            assert_eq!(p.terms[ABHYASA].text, abhyasa, "{number:?}");
            assert!(p.log.is_empty(), "{number:?}");
        }
    }

    /// The whole abhyāsa stage, in pipeline order, on one ślu'd root.
    fn run_abhyasa_stage(root: &str, number: &'static str) -> Prakriya {
        let mut p = slu_prakriya(root, "ti");
        p.ctx.dhatupatha = number;
        for r in ABHYASA_RULES {
            (r.apply)(&mut p);
        }
        p
    }

    #[test]
    fn the_r_roots_reach_their_abhyasa_through_ur_at_then_haladih_shesha() {
        // Each of 3d's six rows through the whole stage: exactly the rules
        // named, in this order, and the abhyāsa 8.4.54 later finishes
        // (Bi → bi, Ja → ja).
        for (root, number, want, ids) in [
            (
                "Bf",
                "03.0006",
                "Bi",
                vec!["6.1.10", "7.4.66", "7.4.60", "7.4.76"],
            ),
            (
                "pf",
                "03.0005",
                "pi",
                vec!["6.1.10", "7.4.66", "7.4.60", "7.4.77"],
            ),
            (
                "pF",
                "03.0004",
                "pi",
                vec!["6.1.10", "7.4.66", "7.4.60", "7.4.77"],
            ),
            (
                "Gf",
                "03.0015",
                "Ja",
                vec!["6.1.10", "7.4.66", "7.4.60", "7.4.62"],
            ),
            (
                "hf",
                "03.0016",
                "Ja",
                vec!["6.1.10", "7.4.66", "7.4.60", "7.4.62"],
            ),
            ("sf", "03.0018", "sa", vec!["6.1.10", "7.4.66", "7.4.60"]),
        ] {
            let p = run_abhyasa_stage(root, number);
            assert_eq!(p.terms[ABHYASA].text, want, "{number}");
            assert_eq!(p.terms[ANGA].text, root, "{number}");
            let got: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(got, ids, "{number}");
        }
    }
}
