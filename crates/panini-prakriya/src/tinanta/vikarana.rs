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
use crate::tinanta::terms::{ANGA, SHAP};

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
}
