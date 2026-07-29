//! Vikaraṇa selection and luk: 3.1.69, 3.1.77, 3.1.68, 2.4.72, 1.2.4.
//!
//! **This stage contains the 3.1.68 boundary.** Rules before 3.1.68 in this
//! file address the ending as `ENDING_PRE_SHAP` (index 1); rules after it use
//! `ENDING` (index 2) and may use `SHAP`. Get this wrong and a rule mutates
//! śap while believing it is mutating the ending, or panics indexing a slot
//! that does not exist yet. See `super::terms`.
//!
//! 2.4.72 luks śap by emptying its text in place rather than removing the
//! term, which is what keeps every later index valid — and what makes
//! `terms[SHAP].text` possibly empty for the rest of the pipeline.

use crate::it_samjna::run_it_samjna;
use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use crate::tinanta::sound::is_vowel;
use crate::tinanta::terms::{ANGA, ENDING, SHAP};

pub(crate) static VIKARANA: &[Rule] = &[
    // 3.1.69 divādibhyaḥ śyan: divādi (gaṇa 4) takes śyan, not śap. Apavāda
    // to the utsarga 3.1.68, ordered before it (as 6.4.72 precedes 6.4.71).
    // śyan is apit; the second 1.2.4 makes it ṅit and 1.1.5 then blocks guṇa.
    Rule {
        id: "3.1.69",
        name: "divAdiByaH Syan",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Divadi) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("Syan");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.69", "divAdiByaH Syan", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S, 1.3.3 strips n → ya
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 3.1.73 svādibhyaḥ śnuḥ: svādi (gaṇa 5) takes śnu, not śap. Apavāda to
    // 3.1.68, ordered before it, exactly as 3.1.69, 3.1.77 and 3.1.81 are.
    //
    // śnu is apit, so the second 1.2.4 below tags it ṅit with no change of
    // its own — which is what blocks the FIRST 7.3.84 on the ik-final roots
    // (hi, ri): hinoti, not *henoti. The guṇa svādi IS famous for lands on
    // śnu's own `u` and belongs to 7.3.84's SECOND application (`guna.rs`),
    // because by 1.4.13 the aṅga for the tiṅ ending is root + vikaraṇa.
    //
    // Unlike śnā, śnu's text never changes shape here — 6.4.87 and 6.4.77
    // rewrite its `u` later, in `guna.rs`, and only before a vowel.
    Rule {
        id: "3.1.73",
        name: "svAdiByaH SnuH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Svadi) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("Snu");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.73", "svAdiByaH SnuH", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S → nu
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 3.1.77 tudādibhyaḥ śaḥ: tudādi (gaṇa 6) takes śa, not śap. Apavāda to
    // 3.1.68, same shape as 3.1.69. śa is apit → ṅit (1.2.4) → guṇa blocked.
    Rule {
        id: "3.1.77",
        name: "tudAdiByaH SaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Tudadi) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("Sa");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.77", "tudAdiByaH SaH", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S → a
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 3.1.81 kryādibhyaḥ śnā: kryādi (gaṇa 9) takes śnā, not śap. Apavāda to
    // 3.1.68, ordered before it, exactly as 3.1.69 and 3.1.77 are. śnā is
    // apit; the second 1.2.4 makes it ṅit and 1.1.5 then blocks guṇa — which
    // is what keeps kliS from guṇating to kleS under 7.3.86.
    //
    // Unlike adādi's śap, śnā is never luk'd: its text goes nA → nI (6.4.113)
    // or nA → n (6.4.112), and never to empty. But a rule that guards on
    // `SHAP.is_empty()` to detect "the thematic coalescence rules didn't
    // apply" still silently declines for kryādi: its SHAP is non-empty but
    // also non-`a`-final, so `is_empty()` misses it exactly where an
    // athematic arm is needed. 6.1.66 (`adesha.rs`) learned this the hard
    // way — its old emptiness guard produced *vfRIyta instead of vfRIta
    // until it was widened to `!SHAP.ends_with('a')`, which is the correct
    // test. Any new rule reading terms[SHAP] to distinguish the athematic
    // path from the thematic one should use that test, not emptiness.
    Rule {
        id: "3.1.81",
        name: "kryAdiByaH SnA",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Kryadi) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("SnA");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.81", "kryAdiByaH SnA", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S → nA
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 3.1.68 kartari śap: insert śap between dhātu and ending, run it-samjña
    // on it (Sap → a), and mark the dhātu an aṅga.
    Rule {
        id: "3.1.68",
        name: "kartari Sap",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // Utsarga: fires only when no apavāda vikaraṇa (śyan 3.1.69 / śa
            // 3.1.77) is already present. Guarding on the vikaraṇa's presence
            // keeps śap the default without hard-coding a gaṇa, so curādi can
            // reuse śap later.
            if p.terms.len() > SHAP && p.terms[SHAP].has(Tag::Vikarana) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("Sap");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            s.add(Tag::Pit); // p-anubandha: śap is pit, so 1.2.4 leaves it alone
            p.terms.insert(SHAP, s);
            p.record("3.1.68", "kartari Sap", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP);
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 2.4.72 adiprabhṛtibhyaḥ śapaḥ: adādi (gaṇa 2) luks the śap that 3.1.68
    // inserts, so the tiṅ ending attaches directly to the root. Modelled by
    // emptying the śap term's text (the term stays, keeping ENDING at index 2
    // and text() = root + "" + ending). Guarded on Tag::Adadi and on a real
    // śap being present, so it never touches divādi/tudādi (śyan/śa) or bhvādi
    // that has already been processed differently.
    Rule {
        id: "2.4.72",
        name: "adipraBftiByaH SapaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Adadi) {
                return false;
            }
            if !(p.terms.len() > SHAP
                && p.terms[SHAP].has(Tag::Vikarana)
                && !p.terms[SHAP].text.is_empty())
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = String::new();
            p.record("2.4.72", "adipraBftiByaH SapaH", before);
            true
        },
    },
    // 3.1.83 halaḥ śnaḥ śānac: after a CONSONANT-final root, with `hi`
    // following, śnā is replaced wholesale by śānac. it-samjña strips the
    // leading S (1.3.8) and the final c (1.3.3), leaving `Ana`; the existing
    // 6.4.105 ato heḥ then elides the hi after śāna's short `a`, giving
    // kliSAna. No new rule is needed for the hi-lopa.
    //
    // Placement carries two constraints, both failing visibly:
    //   - BEFORE 6.4.113 (anga stage, later): that rule would otherwise turn
    //     śnā's ā into ī before the consonant-initial ṅit `hi` and give
    //     *kliSnIhi. As an apavāda, 3.1.83 must remove śnā first.
    //   - BEFORE the second 1.2.4, immediately below: śānac is apit and must
    //     be tagged ṅit, or 7.3.86 guṇates kliS's laghu upadhā and the form
    //     surfaces as *kleSAna.
    //
    // Vowel-final roots fall outside "halaḥ" and keep śnā, taking 6.4.113 to
    // vrIRIhi. That pair — kliSAna against vrIRIhi — is the rule's pin.
    //
    // Its id is 3.1.x but it lives after the 3.1.68 boundary, so it addresses
    // the ending as ENDING (index 2). Stage placement is by pipeline position,
    // not sūtra family; see `super::terms`. The `hi` it reads already exists:
    // 3.4.87 ser hyapic ca runs in the earlier `tin` stage.
    Rule {
        id: "3.1.83",
        name: "halaH SnaH SAnajJO",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms.len() <= ENDING || p.terms[SHAP].text != "nA" {
                return false;
            }
            if p.terms[ENDING].text != "hi" {
                return false;
            }
            let Some(last) = p.terms[ANGA].text.chars().last() else {
                return false;
            };
            if is_vowel(last) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("SAnac");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms[SHAP] = s;
            p.record("3.1.83", "halaH SnaH SAnajJO", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S, 1.3.3 strips c → Ana
            p.terms[SHAP] = s;
            true
        },
    },
    // 1.2.4 sārvadhātukam apit — second application, on the vikaraṇa. The
    // first application (above the boundary) tags apit ātmanepada endings;
    // this one tags the apit sārvadhātuka VIKARAṆA ṅit once it exists. śyan
    // and śa are apit (no p-anubandha); śap carries Tag::Pit (3.1.68) and is
    // skipped — so bhvādi is untouched. NOT pada-gated: śyan/śa are apit in
    // parasmaipada derivations too, which is what blocks guṇa in dīvyati /
    // kupyati / tudati.
    Rule {
        id: "1.2.4",
        name: "sArvaDAtukam apit",
        kind: RuleKind::Atidesha,
        apply: |p| {
            if !(p.terms.len() > SHAP
                && p.terms[SHAP].has(Tag::Vikarana)
                && !p.terms[SHAP].has(Tag::Pit)
                && !p.terms[SHAP].has(Tag::Ngit))
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].add(Tag::Ngit);
            p.record("1.2.4", "sArvaDAtukam apit", before);
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

    #[test]
    fn svadibhyah_shnu_inserts_nu_for_svadi_only() {
        let mut p = Prakriya {
            terms: vec![Term::new("Ap"), Term::new("ti")],
            ..Default::default()
        };
        p.terms[ANGA].add(Tag::Svadi);
        let rule = rules().find(|r| r.id == "3.1.73").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nu");
        assert!(p.terms[SHAP].has(Tag::Vikarana));
        assert!(p.terms[SHAP].has(Tag::Sarvadhatuka));
        assert_eq!(p.terms[ENDING].text, "ti");
    }

    #[test]
    fn svadibhyah_shnu_declines_without_the_gana_tag() {
        // bhvādi: no Tag::Svadi, so the apavāda must not fire and 3.1.68 keeps
        // its utsarga job.
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("ti")],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "3.1.73").unwrap();
        assert!(!(rule.apply)(&mut p));
    }

    #[test]
    fn shnu_is_tagged_ngit_by_the_second_1_2_4_without_change() {
        // śnu carries no p-anubandha, so the existing second 1.2.4 must tag it
        // ṅit with no edit. This is what blocks the FIRST 7.3.84 on ik-final
        // roots (hinoti, not *henoti).
        let mut p = Prakriya {
            terms: vec![Term::new("hi"), Term::new("ti")],
            ..Default::default()
        };
        p.terms[ANGA].add(Tag::Svadi);
        let shnu = rules().find(|r| r.id == "3.1.73").unwrap();
        assert!((shnu.apply)(&mut p));
        assert_eq!(rules().filter(|r| r.id == "1.2.4").count(), 2);
        let second = rules().filter(|r| r.id == "1.2.4").nth(1).unwrap();
        assert!((second.apply)(&mut p));
        assert!(p.terms[SHAP].has(Tag::Ngit));
    }

    // --- 3.1.68 / second 1.2.4: `len() > SHAP` boundary pins --------------
    //
    // Both guards read `p.terms.len() > SHAP && p.terms[SHAP]. ...` to
    // avoid indexing the not-yet-inserted vikaraNa slot. Every real
    // derivation always has an ending term present (terms.len() >= 2)
    // before either rule runs, so `> SHAP` (i.e. `> 1`) and `>= SHAP`
    // never diverge on any golden or negative derivation: len() is never
    // exactly 1 there. Pin the boundary directly with a single-term
    // Prakriya (aGga only, no ending) so the two outcomes diverge: the
    // original short-circuits before indexing terms[SHAP]; the `>` -> `>=`
    // mutant does not, and panics indexing out of bounds on a 1-element
    // Vec (an unexpected panic still fails the test).
    #[test]
    fn kartari_sap_single_term_anga_does_not_panic() {
        let mut p = Prakriya {
            terms: vec![Term::new("kf")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "3.1.68").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kf");
        assert_eq!(p.terms[SHAP].text, "a");
    }

    #[test]
    fn sarvadhatukam_apit_second_application_single_term_does_not_panic() {
        // The SECOND "1.2.4" rule in TINANTA_RULES (the vikaraNa-Girit
        // application, ordered after 3.1.68) is targeted here, not the
        // first (ENDING_PRE_SHAP) application above the 3.1.68 boundary.
        let mut p = Prakriya {
            terms: vec![Term::new("kf")],
            log: vec![],
            ..Default::default()
        };
        assert_eq!(
            rules().filter(|r| r.id == "1.2.4").count(),
            2,
            "expected exactly two 1.2.4 rule entries; nth(1) locator assumes this"
        );
        let rule = rules().filter(|r| r.id == "1.2.4").nth(1).unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kf");
    }

    // --- 2.4.72: `len() > SHAP` boundary + guard-order pins ----------------
    //
    // 2.4.72's guard is `len() > SHAP && has(Vikarana) && !text.is_empty()`,
    // each conjunct short-circuiting before the next would index the
    // not-yet-inserted vikaraNa slot. Every real derivation reaches this
    // rule only after 3.1.68 has already inserted Sap (terms.len() >= 2),
    // so `> SHAP` vs `>= SHAP`, and `&&` vs `||` at either join, never
    // diverge on any golden or negative derivation. Pin the boundary
    // directly: a single-term Prakriya (aGga only, tagged Adadi so the
    // outer gana guard passes) makes `len() > SHAP` (1 > 1) false, so the
    // original short-circuits before ever touching terms[SHAP]. Each of
    // the three mutants below removes a different short-circuit and
    // indexes terms[SHAP] out of bounds on this 1-element Vec, panicking:
    //   - `>` -> `>=`: `1 >= 1` is true, so `has(Vikarana)` is evaluated.
    //   - first `&&` -> `||`: `len() > SHAP` (false) forces evaluation of
    //     `has(Vikarana)` to resolve the OR.
    //   - second `&&` -> `||`: `(len() > SHAP && has(Vikarana))` (false)
    //     forces evaluation of `!text.is_empty()` to resolve the OR.
    // One construction catches all three.
    #[test]
    fn adiprabhrtibhyah_sapah_single_term_anga_does_not_panic() {
        let mut anga = Term::new("kf");
        anga.add(Tag::Adadi);
        let mut p = Prakriya {
            terms: vec![anga],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "2.4.72").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kf");
    }

    #[test]
    fn kryadibhyah_shna_inserts_shna_for_kryadi_only() {
        // 3.1.81 is an apavAda to 3.1.68, same shape as 3.1.69/3.1.77.
        // it-samjNa strips the S (1.3.8), leaving nA. No Tag::Pit: SnA is
        // apit, so the second 1.2.4 makes it Nit and 1.1.5 then blocks guNa
        // -- which is why kliS gives kliSnAti and not *kleSnAti.
        let mut anga = Term::new("kliS");
        anga.add(Tag::Kryadi);
        let mut p = Prakriya {
            terms: vec![anga, Term::new("ti")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "3.1.81").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nA");
        assert!(p.terms[SHAP].has(Tag::Vikarana));
        assert!(p.terms[SHAP].has(Tag::Sarvadhatuka));
        assert!(!p.terms[SHAP].has(Tag::Pit));
        assert!(p.terms[ANGA].has(Tag::Anga));
    }

    #[test]
    fn kryadibhyah_shna_declines_for_every_other_gana() {
        // bhvAdi carries no gana tag at all; the other three carry their own.
        // A mutant that drops the tag guard would give every root SnA.
        for tag in [None, Some(Tag::Divadi), Some(Tag::Tudadi), Some(Tag::Adadi)] {
            let mut anga = Term::new("BU");
            if let Some(t) = tag {
                anga.add(t);
            }
            let mut p = Prakriya {
                terms: vec![anga, Term::new("ti")],
                log: vec![],
                ..Default::default()
            };
            let rule = rules().find(|r| r.id == "3.1.81").unwrap();
            assert!(!(rule.apply)(&mut p), "fired for {tag:?}");
        }
    }

    /// `[anga, SnA, ending]`, the shape 3.1.83 inspects.
    fn shna_before(anga: &str, ending: &str) -> Prakriya {
        let mut vik = Term::new("nA");
        vik.add(Tag::Vikarana);
        vik.add(Tag::Sarvadhatuka);
        Prakriya {
            terms: vec![Term::new(anga), vik, Term::new(ending)],
            log: vec![],
            ..Default::default()
        }
    }

    #[test]
    fn halah_shnah_shanac_replaces_shna_after_a_consonant_final_root() {
        // kliS + nA + hi -> kliS + Ana + hi; 6.4.105 ato heH (adesha stage)
        // then drops the hi, giving kliSAna.
        let mut p = shna_before("kliS", "hi");
        let rule = rules().find(|r| r.id == "3.1.83").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "Ana");
        assert!(p.terms[SHAP].has(Tag::Vikarana));
        assert!(!p.terms[SHAP].has(Tag::Pit)); // apit: the next 1.2.4 tags it
    }

    #[test]
    fn halah_shnah_shanac_declines_after_a_vowel_final_root() {
        // "halaH" is the whole condition. vrI is vowel-final, so it keeps SnA
        // and takes 6.4.113 instead: vrIRIhi, not *vrIRAna. This pair is the
        // rule's shape guard.
        let mut p = shna_before("vrI", "hi");
        let rule = rules().find(|r| r.id == "3.1.83").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nA");
    }

    #[test]
    fn halah_shnah_shanac_declines_for_endings_other_than_hi() {
        // The sutra is conditioned on hi alone. A mutant dropping this would
        // rewrite the entire consonant-final paradigm as *kliSAnati.
        for ending in ["ti", "taH", "anti", "tAt"] {
            let mut p = shna_before("kliS", ending);
            let rule = rules().find(|r| r.id == "3.1.83").unwrap();
            assert!(!(rule.apply)(&mut p), "fired on {ending}");
            assert_eq!(p.terms[SHAP].text, "nA");
        }
    }

    #[test]
    fn halah_shnah_shanac_ignores_other_vikaranas_and_short_prakriyas() {
        for vikarana in ["a", "ya", ""] {
            let mut p = shna_before("kliS", "hi");
            p.terms[SHAP].text = vikarana.to_string();
            let rule = rules().find(|r| r.id == "3.1.83").unwrap();
            assert!(!(rule.apply)(&mut p), "fired on {vikarana:?}");
        }
        // A one-term prakriya must not panic indexing SHAP or ENDING.
        let mut p = Prakriya {
            terms: vec![Term::new("kliS")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "3.1.83").unwrap();
        assert!(!(rule.apply)(&mut p));
    }
}
