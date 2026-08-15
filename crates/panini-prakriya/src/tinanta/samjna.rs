//! Saṃjñā, pada sanction and ending insertion: 1.3.12, 1.3.72, 1.3.78,
//! 3.4.78, 1.3.9, 1.2.4.
//!
//! Ordered **BEFORE** 3.1.68 — the ending lives at `ENDING_PRE_SHAP`
//! (index 1) and śap does not exist yet. See `super::terms`.
//!
//! 3.4.78 is what inserts the ending; `super::tin` picks up from 3.4.85 and
//! reshapes it. 1.2.4 appears here tagging apit sārvadhātuka endings ṅit
//! (not pada-conditioned), and again in `super::vikarana` tagging the apit
//! vikaraṇa once it exists.

use crate::it_samjna::run_it_samjna;
use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use crate::tinanta::sound::is_vibhakti_protected_final;
use crate::tinanta::terms::{ANGA, ENDING_PRE_SHAP};
use panini_data::{Lakara, Pada, Purusha, tin_ending};

pub(crate) static SAMJNA: &[Rule] = &[
    // 1.3.12 anudāttaṅita ātmanepadam: a root carrying the anudātta/ṅit
    // marker (here: the data-layer Atmanepadin tag) takes ātmanepada.
    // Sanctions the requested pada; the wrong pada BLOCKS the derivation —
    // derivation, not the analyzer, is the source of truth for pada.
    Rule {
        id: "1.3.12",
        name: "anudAttaNita Atmanepadam",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Atmanepadin) {
                return false; // parasmaipada roots are 1.3.78's business
            }
            match p.ctx.pada {
                Pada::Atmanepada => {
                    let before = p.snapshot();
                    p.record("1.3.12", "anudAttaNita Atmanepadam", before);
                    true
                }
                Pada::Parasmaipada => {
                    p.blocked = true;
                    false
                }
            }
        },
    },
    // 1.3.72 svaritaYitaH kartraBiprAye kriyAPale: a root carrying the
    // svarita or ñit marker (here: the data-layer Ubhayapadin tag) takes
    // ātmanepada when the fruit of the action accrues to the agent.
    //
    // THE SEMANTICS ARE NOT MODELLED, and that is deliberate. The engine
    // knows nothing of *kartrabhiprāye kriyāphale*, and 1.3.78's *śeṣāt* is
    // exactly its complementary residue — so both arms derive, each trace
    // crediting the sūtra that sanctioned it, and the reader selects by
    // sense. This is why the rule is NOT vikalpa: pada is a context
    // coordinate, so the two readings are two CELLS, not two branches of one
    // cell, and they must not enter the fork machinery `docs/ARCHITECTURE.md`
    // reserves for anyatarasyām / vā / vibhāṣā.
    //
    // The parasmaipada arm DECLINES rather than blocks. 1.3.78 immediately
    // below is what sanctions it; blocking here would collapse the
    // ubhayapada arm this rule exists to open. See `Tag::Ubhayapadin`'s doc
    // comment for why the tag is named for 1.3.12's residue rather than for
    // this sūtra's own marker (√indh is ñit and must never reach here).
    Rule {
        id: "1.3.72",
        name: "svaritaYitaH kartraBiprAye kriyAPale",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            // parasmaipada-only roots are 1.3.78's business; ātmanepada-only
            // ones are 1.3.12's.
            if !p.terms[ANGA].has(Tag::Ubhayapadin) {
                return false;
            }
            match p.ctx.pada {
                Pada::Atmanepada => {
                    let before = p.snapshot();
                    p.record("1.3.72", "svaritaYitaH kartraBiprAye kriyAPale", before);
                    true
                }
                Pada::Parasmaipada => false,
            }
        },
    },
    // 1.3.78 śeṣāt kartari parasmaipadam: everything else takes parasmaipada.
    Rule {
        id: "1.3.78",
        name: "SezAt kartari parasmEpadam",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if p.terms[ANGA].has(Tag::Atmanepadin) {
                return false;
            }
            match p.ctx.pada {
                Pada::Parasmaipada => {
                    let before = p.snapshot();
                    p.record("1.3.78", "SezAt kartari parasmEpadam", before);
                    true
                }
                // The guard above already admits an ubhayapadī root — it is
                // `!Atmanepadin` — so this arm is where the two sūtras
                // overlap, and where they split on ctx.pada: 1.3.72 has
                // already sanctioned this cell, so decline instead of
                // blocking. Only the genuine śeṣa (no pada tag at all)
                // blocks here.
                Pada::Atmanepada => {
                    if p.terms[ANGA].has(Tag::Ubhayapadin) {
                        return false;
                    }
                    p.blocked = true;
                    false
                }
            }
        },
    },
    // 3.4.78 tiptasjhi...: replace the lakāra by the tiṅ ending.
    // 3.4.113 tiṅ-śit sārvadhātukam makes it sārvadhātuka.
    Rule {
        id: "3.4.78",
        name: "tiptasJisipTasTamibvasmas",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let before = p.snapshot();
            let ending = tin_ending(p.ctx.pada, p.ctx.purusha, p.ctx.vacana);
            let mut e = Term::new(ending);
            e.add(Tag::Tin);
            e.add(Tag::Sarvadhatuka);
            // 1.3.3 halantyam identifies the final `p` of tip/sip/mip as an
            // it; that anubandha is what makes those three endings pit. 1.3.9
            // strips it below, and 1.2.4 runs after that -- so the fact has to
            // be recorded here, while the raw text still carries it.
            if ending.ends_with('p') {
                e.add(Tag::Pit);
            }
            p.terms.push(e);
            p.record("3.4.78", "tiptasJisipTasTamibvasmas", before);
            true
        },
    },
    // it-samjña on the tiṅ ending (1.3.3 halantyam / 1.3.9 tasya lopaḥ),
    // respecting 1.3.4: the final s/t/m of a vibhakti is protected, so only
    // endings whose final is a genuine anubandha (tip/sip/mip → the pit marker
    // `p`) are reduced.
    //
    // This MUST precede the lakāra-specific substitutions in `tin`: 3.4.100
    // itaś ca elides the `i` of `tip`, and that `i` is only exposed once
    // halantyam has stripped the `p`.
    Rule {
        id: "1.3.9",
        name: "tasya lopaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let last = p.terms[ENDING_PRE_SHAP].text.chars().last();
            if last.map(is_vibhakti_protected_final).unwrap_or(false) {
                return false;
            }
            let mut e = p.terms[ENDING_PRE_SHAP].clone();
            let original = e.text.clone();
            run_it_samjna(&mut e, p, ENDING_PRE_SHAP);
            p.terms[ENDING_PRE_SHAP] = e;
            p.terms[ENDING_PRE_SHAP].text != original
        },
    },
    // 1.2.4 sārvadhātukam apit: an apit sārvadhātuka behaves as ṅit. An
    // atideśa (the 3.4.85 precedent): a rule that appears in the trace and
    // sets a term-level tag — distinct from ctx.is_ngit_like, which says the
    // *lakāra* is ṅit and drives 3.4.99/100/101.
    //
    // Guard notes (see the spec's 1.2.4 section):
    // - Not pada-conditioned. Every apit sārvadhātuka is ṅid-vat, parasmaipada
    //   included: 6.4.112 / 6.4.113 read exactly this tag, and the whole
    //   kryādi paradigm is the pit/apit split (kliSnAti from pit tip against
    //   kliSnItaH from apit tas). Pit-ness comes from the `p` anubandha,
    //   recorded by 3.4.78 before 1.3.9 strips it.
    // - Loṭ uttama is a genuine exclusion, not trace-minimalism: 3.4.92's
    //   own "pic ca" makes those endings pit, hence not apit — which is what
    //   keeps 7.2.81 off the āṭ-āgama (AvahE goes to 6.1.101 instead).
    Rule {
        id: "1.2.4",
        name: "sArvaDAtukam apit",
        kind: RuleKind::Atidesha,
        vikalpa: false,
        apply: |p| {
            if p.terms[ENDING_PRE_SHAP].has(Tag::Pit)
                || (matches!(p.ctx.lakara, Lakara::Lot) && matches!(p.ctx.purusha, Purusha::Uttama))
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].add(Tag::Ngit);
            p.record("1.2.4", "sArvaDAtukam apit", before);
            true
        },
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::prakriya::Prakriya;
    use crate::tinanta::rules;
    use panini_data::{Dhatu, PadaAssignment, Vacana, dhatus};

    /// The aṅga the three pada sūtras read, tagged the way `super::derive`
    /// tags it. Only the pada assignment matters to 1.3.12 / 1.3.72 / 1.3.78,
    /// so the gaṇa tags are left off. The match is exhaustive, so a new
    /// `PadaAssignment` variant stops compiling here rather than silently
    /// arriving untagged; that the *pipeline* applies this tagging at all is
    /// pinned end-to-end by `rudh_derives_in_both_padas`.
    fn pada_anga(d: &Dhatu) -> Term {
        let mut t = Term::new(d.code);
        t.add(Tag::Dhatu);
        match d.pada {
            PadaAssignment::Parasmaipada => {}
            PadaAssignment::Atmanepada => t.add(Tag::Atmanepadin),
            PadaAssignment::Ubhayapada => t.add(Tag::Ubhayapadin),
        }
        t
    }

    fn pada_prakriya(id: &str, pada: Pada) -> Prakriya {
        let d = dhatus().iter().find(|d| d.id == id).unwrap();
        let mut p = Prakriya {
            ctx: Context::new(Lakara::Lat, pada, Purusha::Prathama, Vacana::Eka),
            ..Default::default()
        };
        p.terms.push(pada_anga(d));
        p
    }

    #[test]
    fn svaritanit_reports_firing_only_on_atmanepada() {
        // `run_pipeline` discards `apply`'s return value (see
        // `it_samjna_rule_reports_when_ending_is_reduced`), so pin it here.
        // 1.3.72 sanctions the ātmanepada reading and DECLINES the
        // parasmaipada one — declines, not blocks: 1.3.78 immediately below
        // is what sanctions parasmaipada, and blocking here would collapse
        // the ubhayapada arm this rule exists to open.
        let rule = rules().find(|r| r.id == "1.3.72").unwrap();
        for (pada, fires) in [(Pada::Atmanepada, true), (Pada::Parasmaipada, false)] {
            let mut p = pada_prakriya("ruD", pada);
            assert_eq!((rule.apply)(&mut p), fires, "1.3.72 on {pada:?}");
            assert!(!p.blocked, "1.3.72 must never block, {pada:?}");
        }
    }

    #[test]
    fn svaritanit_declines_for_roots_that_are_not_ubhayapadin() {
        // The guard is Tag::Ubhayapadin and nothing else. A parasmaipada-only
        // root (√bhū, untagged) is 1.3.78's business and an ātmanepada-only
        // one (√khid, √indh) is 1.3.12's; 1.3.72 must leave both alone in
        // both padas, without recording and without blocking.
        let rule = rules().find(|r| r.id == "1.3.72").unwrap();
        for id in ["BU", "Kid", "inD"] {
            for pada in [Pada::Parasmaipada, Pada::Atmanepada] {
                let mut p = pada_prakriya(id, pada);
                assert!(!(rule.apply)(&mut p), "1.3.72 fired on {id} {pada:?}");
                assert!(!p.blocked, "1.3.72 blocked {id} {pada:?}");
                assert!(p.log.is_empty(), "1.3.72 recorded on {id} {pada:?}");
            }
        }
    }

    #[test]
    fn pada_sutras_are_order_independent() {
        // Every pair of the three is disjoint. Atmanepadin and Ubhayapadin
        // are mutually exclusive on the root, so 1.3.12 and 1.3.72 can never
        // both fire; and where 1.3.72 and 1.3.78 overlap — an ubhayapadī root
        // is `!Atmanepadin`, so 1.3.78's guard admits it — they split on
        // `ctx.pada`, with 1.3.78's ātmanepada arm declining in exactly the
        // case 1.3.72 handles.
        //
        // Commit ee35a30 had to go back and qualify an order-independence
        // claim that prose had overstated, so this one is a test: the three
        // rules are run in each of the 3! orders and must leave the same
        // (blocked, text, pada-sūtra log) triple every time. Blocking stops
        // the run, mirroring `run_pipeline`, so the claim covers the
        // pipeline's actual semantics rather than an idealised sweep.
        const IDS: [&str; 3] = ["1.3.12", "1.3.72", "1.3.78"];
        const ORDERS: [[usize; 3]; 6] = [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ];
        // `None` = the request is blocked; `Some(id)` = the sūtra that
        // sanctions it. Pinned so a mutant that turned all three rules into
        // no-ops could not pass by being trivially order-independent.
        let cells = [
            ("ruD", Pada::Parasmaipada, Some("1.3.78")),
            ("ruD", Pada::Atmanepada, Some("1.3.72")),
            ("inD", Pada::Parasmaipada, None),
            ("inD", Pada::Atmanepada, Some("1.3.12")),
            ("BU", Pada::Parasmaipada, Some("1.3.78")),
            ("BU", Pada::Atmanepada, None),
            ("Kid", Pada::Parasmaipada, None),
            ("Kid", Pada::Atmanepada, Some("1.3.12")),
        ];
        for (id, pada, expected) in cells {
            let mut results = Vec::new();
            for order in ORDERS {
                let mut p = pada_prakriya(id, pada);
                for i in order {
                    if p.blocked {
                        break;
                    }
                    let rule = rules().find(|r| r.id == IDS[i]).unwrap();
                    (rule.apply)(&mut p);
                }
                let logged: Vec<String> = p
                    .log
                    .iter()
                    .filter(|s| IDS.contains(&s.sutra.as_str()))
                    .map(|s| s.sutra.clone())
                    .collect();
                results.push((p.blocked, p.text(), logged));
            }
            let want: (bool, String, Vec<String>) = match expected {
                Some(sutra) => (false, pada_anga_text(id), vec![sutra.to_string()]),
                None => (true, pada_anga_text(id), vec![]),
            };
            assert_eq!(results[0], want, "{id} {pada:?} in sūtra order");
            for (n, got) in results.iter().enumerate() {
                assert_eq!(
                    got, &results[0],
                    "{id} {pada:?}: order {:?} differs from sūtra order",
                    ORDERS[n]
                );
            }
        }
    }

    fn pada_anga_text(id: &str) -> String {
        dhatus().iter().find(|d| d.id == id).unwrap().code.into()
    }

    #[test]
    fn it_samjna_rule_reports_when_ending_is_reduced() {
        // 1.3.9 tasya lopaH's `apply` returns whether it actually elided the
        // ending's it; pin that return value directly, since `run_pipeline`
        // discards it and no golden form exercises it in isolation.
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("tip")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "1.3.9").unwrap();
        assert!(
            (rule.apply)(&mut p),
            "1.3.9 should report firing when tip loses its final p"
        );
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "ti");
    }

    #[test]
    fn sarvadhatukam_apit_tags_atmanepada_endings_ngit() {
        let mut p = Prakriya {
            terms: vec![Term::new("laB"), Term::new("ta")],
            log: vec![],
            ctx: Context::new(
                Lakara::Lat,
                Pada::Atmanepada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "1.2.4").unwrap();
        assert!((rule.apply)(&mut p));
        assert!(p.terms[ENDING_PRE_SHAP].has(Tag::Ngit));
        assert!(p.log.iter().any(|s| s.sutra == "1.2.4"));
    }

    #[test]
    fn sarvadhatukam_apit_tags_parasmaipada_apit_endings() {
        // 1.2.4 is not pada-conditioned. `tas` is apit (no p-anubandha), so
        // it is Nid-vat in a parasmaipada derivation exactly as `ta` is in an
        // atmanepada one. This is the tag 6.4.112/6.4.113 fire on.
        for (purusha, vacana) in [
            (Purusha::Prathama, Vacana::Dvi),  // tas
            (Purusha::Prathama, Vacana::Bahu), // Ji
            (Purusha::Uttama, Vacana::Dvi),    // vas
            (Purusha::Uttama, Vacana::Bahu),   // mas
        ] {
            let mut p = Prakriya {
                ctx: Context::new(Lakara::Lat, Pada::Parasmaipada, purusha, vacana),
                ..Default::default()
            };
            p.terms.push(Term::new("kliS"));
            for id in ["3.4.78", "1.3.9", "1.2.4"] {
                let rule = rules().find(|r| r.id == id).unwrap();
                (rule.apply)(&mut p);
            }
            assert!(
                p.terms[ENDING_PRE_SHAP].has(Tag::Ngit),
                "{purusha:?} {vacana:?} should be Nit"
            );
        }
    }

    #[test]
    fn sarvadhatukam_apit_declines_for_pit_endings() {
        // tip/sip/mip carry the p-anubandha. They must stay untagged, or
        // 6.4.113 would fire on them and kliSnAti would surface as
        // *kliSnIti.
        for (purusha, vacana) in [
            (Purusha::Prathama, Vacana::Eka), // tip
            (Purusha::Madhyama, Vacana::Eka), // sip
            (Purusha::Uttama, Vacana::Eka),   // mip
        ] {
            let mut p = Prakriya {
                ctx: Context::new(Lakara::Lat, Pada::Parasmaipada, purusha, vacana),
                ..Default::default()
            };
            p.terms.push(Term::new("kliS"));
            for id in ["3.4.78", "1.3.9", "1.2.4"] {
                let rule = rules().find(|r| r.id == id).unwrap();
                (rule.apply)(&mut p);
            }
            assert!(p.terms[ENDING_PRE_SHAP].has(Tag::Pit));
            assert!(
                !p.terms[ENDING_PRE_SHAP].has(Tag::Ngit),
                "{purusha:?} {vacana:?} is pit and must not be Nit"
            );
        }
    }

    #[test]
    fn sarvadhatukam_apit_still_declines_for_lot_uttama_in_both_padas() {
        // 3.4.92 AD uttamasya pic ca makes the lot-uttama endings pit
        // outright. Tagging them Nit would let 7.2.81 rewrite the AT-Agama
        // and turn BavAva into *Baviyva. This exclusion is grammar, not
        // trace-minimalism -- it must survive the widening.
        for pada in [Pada::Parasmaipada, Pada::Atmanepada] {
            let mut p = Prakriya {
                ctx: Context::new(Lakara::Lot, pada, Purusha::Uttama, Vacana::Dvi),
                ..Default::default()
            };
            p.terms.push(Term::new("BU"));
            for id in ["3.4.78", "1.3.9", "1.2.4"] {
                let rule = rules().find(|r| r.id == id).unwrap();
                (rule.apply)(&mut p);
            }
            assert!(
                !p.terms[ENDING_PRE_SHAP].has(Tag::Ngit),
                "{pada:?} lot uttama must not be Nit"
            );
        }
    }
}
