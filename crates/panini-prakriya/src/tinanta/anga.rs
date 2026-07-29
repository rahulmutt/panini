//! Aṅga operations: 6.4.71 … 7.2.81.
//!
//! Ordered AFTER 3.1.68 — ending at `ENDING`, śap at `SHAP`, and
//! `terms[SHAP].text` may be empty (2.4.72). See `super::terms`.

use crate::rule::{Rule, RuleKind};
use crate::term::Tag;
use crate::tinanta::sound::is_vowel;
use crate::tinanta::terms::{ANGA, ENDING, SHAP};
use panini_data::{Lakara, Pada};

pub(crate) static ANGA_RULES: &[Rule] = &[
    // 6.4.71 luṅlaṅlṛṅkṣvaḍudāttaḥ: the aṭ-āgama is prefixed to the aṅga in laṅ.
    //
    // Modelled as a prefix on the aṅga's text rather than as a separate term,
    // so the ANGA/SHAP/ENDING indices stay stable for every later rule. The
    // trace still cites 6.4.71, which is what the reader checks.
    Rule {
        id: "6.4.71",
        name: "luNlaNlfNkzvaqudAttaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let first = p.terms[ANGA].text.chars().next().unwrap();
            if !matches!(p.ctx.lakara, Lakara::Lan) || is_vowel(first) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = format!("a{}", p.terms[ANGA].text);
            p.record("6.4.71", "luNlaNlfNkzvaqudAttaH", before);
            true
        },
    },
    // 6.4.72 āḍ ajādīnām: vowel-initial aṅgas take the āṭ-āgama in laṅ
    // (apavāda to 6.4.71's aṭ). The A then merges with the root's initial
    // vowel by 6.1.90 āṭaś ca into vṛddhi: a+eD → ED, a+Ikz → Ekz, a+ad → Ad.
    Rule {
        id: "6.4.72",
        name: "Aq ajAdInAm",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let first = p.terms[ANGA].text.chars().next().unwrap();
            // Only apply to true vowel-initial roots, not to an aṅga that
            // already carries 6.4.71's aṭ augment. 6.4.71's augment is
            // itself the character `a`, which is indistinguishable from a
            // genuinely a-initial root (√ad) by first-char alone — so check
            // whether 6.4.71 actually fired in this derivation (the trace)
            // rather than sniffing the character. Roots 6.4.71 augmented are
            // consonant-initial by its own guard, so this never double-fires;
            // genuinely a-initial roots (√ad) never trigger 6.4.71 (their
            // first char is already a vowel), so they reach here untouched
            // and correctly take āṭ.
            let already_augmented = p.log.iter().any(|s| s.sutra == "6.4.71");
            if !matches!(p.ctx.lakara, Lakara::Lan) || !is_vowel(first) || already_augmented {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = format!("A{}", p.terms[ANGA].text);
            p.record("6.4.72", "Aq ajAdInAm", before);
            true
        },
    },
    // 7.3.100 adaH sarvezAm: √ad prefixes aṭ (`a`) to a laṅ singular
    // consonant ending (2sg s, 3sg t). Without it, Ad+s / Ad+t are word-final
    // conjuncts that 8.2.23 saṃyogāntasya lopaḥ would strip to bare Ad,
    // collapsing 2sg=3sg=1sg-stem. The inserted `a` makes the word
    // vowel-final: 8.2.23 declines, and cartva (8.4.55) skips the `d` (now
    // before `a`, not a khar) → Adat, Adas→AdaH. Guarded structurally
    // (Tag::Adadi ∧ laṅ ∧ consonant-final aṅga ∧ single-char s/t ending); in
    // the current root set that is exactly √ad, and √vas landing (5e) adds
    // no new case here (its ātmanepada endings never collapse to a bare
    // single-char ending at the point this rule runs).
    //
    // The `||`→`&&` mutant on the guard line below is killed by the
    // `akupyat_trace_shows_7_3_100_declines_for_non_adadi_roots` pin in
    // `crates/panini/tests/trace.rs`: the mutant fires for laṅ non-adādi
    // derivations and 6.1.97 repairs the surface form, so only the ordered
    // trace exposes it. (Slice 5e parked this mutant as unkillable on a case
    // analysis that slice 5f corrected.)
    Rule {
        id: "7.3.100",
        name: "adaH sarvezAm",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lan) || !p.terms[ANGA].has(Tag::Adadi) {
                return false;
            }
            // Consonant-final aṅga only (ā-final √yā/√vā never insert).
            let Some(anga_last) = p.terms[ANGA].text.chars().last() else {
                return false;
            };
            if is_vowel(anga_last) {
                return false;
            }
            // Single-consonant ending: 2sg `s` / 3sg `t` (not the multi-char
            // tam/tAm/ta of dual/plural).
            let e = &p.terms[ENDING].text;
            if e.chars().count() != 1 || !matches!(e.as_str(), "s" | "t") {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = format!("a{e}");
            p.record("7.3.100", "adaH sarvezAm", before);
            true
        },
    },
    // 7.1.5 ātmanepadeṣv anataḥ: in ātmanepada, the leading `J` (jh) of the
    // ending becomes `at` — not the `ant` of 7.1.3 — when the segment the
    // ending attaches to does not end in short `a`. Apavāda to 7.1.3, ordered
    // before it; 7.1.3 then declines on its own (ending no longer starts `J`).
    // The "anataḥ" test reads the last non-empty char BEFORE the ending: for a
    // thematic root that is the śap vikaraṇa `a` (rule declines → laBante); for
    // adādi √ās the śap is luk'd/empty, so it is the root-final `s` (rule fires
    // → Asate). By this point 3.4.79 has already turned `Ja` → `Je` (laṭ/loṭ),
    // so 7.1.5 strips the leading `J` and prepends `at`: Je → ate, Ja → ata,
    // JAm → atAm. First non-a-final ātmanepadī aṅga in the engine.
    Rule {
        id: "7.1.5",
        name: "AtmanepadezvanataH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.pada, Pada::Atmanepada) {
                return false;
            }
            if !p.terms[ENDING].text.starts_with('J') {
                return false;
            }
            // "anataḥ": the segment before the ending must NOT end in short `a`.
            // Scan the terms before ENDING (skipping the luk'd/empty śap) for
            // the last non-empty char.
            let prev = p.terms[..ENDING]
                .iter()
                .rev()
                .find_map(|t| t.text.chars().last());
            let Some(prev) = prev else {
                return false;
            };
            if prev == 'a' {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("at{rest}");
            p.record("7.1.5", "AtmanepadezvanataH", before);
            true
        },
    },
    // 7.1.6 śīṅo ruṭ: the *jha* of √śī takes the ruṭ augment. 7.1.5 has just
    // replaced the ending's leading `J` with `at` (Je → ate, Ja → ata,
    // JAm → atAm); ruṭ's `r` prefixes that, giving Se + r + ate → Serate.
    //
    // Guarded on 7.1.5 having FIRED IN THIS DERIVATION rather than on the
    // ending's surface shape: the ruṭ attaches to the `at` that 7.1.5
    // produced, so that is the condition itself and not a proxy for it.
    // Reading the log for a prior rule is the idiom 6.4.72 already uses to
    // test whether 6.4.71 augmented the aṅga.
    //
    // This is why vidhiliṅ needs no special case: 3.4.105 jhasya ran (in
    // `super::tin`) has already replaced the jha with `ran` earlier in the
    // pipeline, so 7.1.5 never fires there and ruṭ cannot attach → SayIran,
    // not *SayIraran.
    Rule {
        id: "7.1.6",
        name: "SINo ruw",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].text.ends_with("SI") || !p.log.iter().any(|s| s.sutra == "7.1.5") {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = format!("r{}", p.terms[ENDING].text);
            p.record("7.1.6", "SINo ruw", before);
            true
        },
    },
    // 7.1.3 jho'ntaḥ: a leading `J` of the ending → `ant`.
    Rule {
        id: "7.1.3",
        name: "Jo'ntaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ENDING].text.starts_with('J') {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("ant{rest}");
            p.record("7.1.3", "Jo'ntaH", before);
            true
        },
    },
    // 7.2.79 liṅaḥ salopo 'nantyasya: the non-final s of sārvadhātuka liṅ's
    // ending is elided. yAst → yAt, yAss → yAs (madhyama-eka: only the first
    // s is non-final!), yAsus → yAus. MUST precede 7.2.80: only after the s
    // goes does the ending start with the `yA` shape 7.2.80 rewrites.
    // Every non-final s reaching this rule is yāsuṭ- or sīyuṭ-derived; the
    // invariant is that the only non-final s is āgama-initial.
    Rule {
        id: "7.2.79",
        name: "liNaH salopo'nantyasya",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin) {
                return false;
            }
            let text = &p.terms[ENDING].text;
            let n = text.chars().count();
            let reduced: String = text
                .chars()
                .enumerate()
                .filter(|&(i, c)| c != 's' || i + 1 == n)
                .map(|(_, c)| c)
                .collect();
            if reduced == *text {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = reduced;
            p.record("7.2.79", "liNaH salopo'nantyasya", before);
            true
        },
    },
    // 7.2.80 ato yeyaḥ: after an a-final aṅga (here: the śap), the yA of the
    // yāsuṭ is replaced by iy. yAt → iyt, yAus → iyus.
    Rule {
        id: "7.2.80",
        name: "ato yeyaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin)
                || !p.terms[SHAP].text.ends_with('a')
                || !p.terms[ENDING].text.starts_with("yA")
            {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(2).collect();
            p.terms[ENDING].text = format!("iy{rest}");
            p.record("7.2.80", "ato yeyaH", before);
            true
        },
    },
    // 7.2.81 āto ṅitaḥ: after an a-final aṅga (the śap), the initial ā of a
    // ṅit ending → iy. Ate→iyte (laṭ), AtAm→iytAm (laṅ/loṭ), ATe→iyTe.
    // The ṅit condition is the TERM tag from 1.2.4 (laṭ/loṭ are ṭit lakāras,
    // yet their apit ātmanepada endings behave as ṅit) — NOT ctx.is_ngit_like.
    // The tag also keeps this rule off parasmaipada loṭ uttama's āṭ (Ani),
    // which 1.2.4 never tags (pic ca) and which belongs to 6.1.101.
    // MUST precede 6.1.101, which would otherwise dīrgha-merge the tagged
    // A-initial endings (laṭ 3du would surface as laBAte, not laBete).
    Rule {
        id: "7.2.81",
        name: "Ato NitaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[SHAP].text.ends_with('a')
                || !p.terms[ENDING].has(Tag::Ngit)
                || !p.terms[ENDING].text.starts_with('A')
            {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("iy{rest}");
            p.record("7.2.81", "Ato NitaH", before);
            true
        },
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::prakriya::Prakriya;
    use crate::term::Term;
    use crate::tinanta::form_g;
    use crate::tinanta::rules;
    use panini_data::{Purusha, Vacana};

    #[test]
    fn salopa_elides_only_the_non_final_s() {
        // Madhyama-eka is the trap: yAs + s = yAss, and only the FIRST s is
        // non-final. Eliding both would derive *Bave for BaveH.
        for (ending, want) in [("yAst", "yAt"), ("yAss", "yAs"), ("yAsus", "yAus")] {
            let mut p = Prakriya {
                terms: vec![Term::new("Bav"), Term::new("a"), Term::new(ending)],
                log: vec![],
                ctx: Context::new(
                    Lakara::VidhiLin,
                    Pada::Parasmaipada,
                    Purusha::Prathama,
                    Vacana::Eka,
                ),
                blocked: false,
            };
            let rule = rules().find(|r| r.id == "7.2.79").unwrap();
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ENDING].text, want, "{ending}");
        }
    }

    #[test]
    fn ato_yeyah_rewrites_the_ya_prefix_after_shap_a() {
        let mut p = Prakriya {
            terms: vec![Term::new("Bav"), Term::new("a"), Term::new("yAt")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "7.2.80").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "iyt");
    }

    #[test]
    fn ato_yeyah_requires_vidhilin_lakara() {
        // shap == "a" and ending starts_with "yA" are both satisfied, but the
        // lakara isn't vidhilin: the guard's first `||` must still short-
        // circuit to false. Kills the `||` -> `&&` mutant at the first
        // operator, which would otherwise let this fire whenever the other
        // two conditions hold regardless of lakara.
        let mut p = Prakriya {
            terms: vec![Term::new("Bav"), Term::new("a"), Term::new("yAt")],
            log: vec![],
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "7.2.80").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "yAt");
    }

    #[test]
    fn ato_yeyah_requires_shap_a() {
        // lakara is vidhilin and ending starts_with "yA", but shap isn't
        // "a": the guard's second `||` must still short-circuit to false.
        // Kills the `||` -> `&&` mutant at the second operator, which would
        // otherwise let this fire whenever lakara is vidhilin regardless of
        // shap.
        let mut p = Prakriya {
            terms: vec![Term::new("i"), Term::new("i"), Term::new("yAt")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "7.2.80").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "yAt");
    }

    #[test]
    fn ato_nitah_requires_the_ngit_tag() {
        // Parasmaipada lot uttama Ani starts with A but is NOT Nid-vat
        // (1.2.4 pic-ca exclusion) — it belongs to 6.1.101, not 7.2.81.
        let mut anga = Term::new("Bav");
        anga.add(Tag::Anga);
        let mut p = Prakriya {
            terms: vec![anga, Term::new("a"), Term::new("Ani")],
            log: vec![],
            ctx: Context::new(
                Lakara::Lot,
                Pada::Parasmaipada,
                Purusha::Uttama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "7.2.81").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "Ani");
    }

    #[test]
    fn rut_requires_both_shing_and_a_fired_seven_one_five() {
        // Both clauses of 7.1.6's guard must hold. Dropping either one is a
        // live mutant, and each half is pinned here.
        //
        // (a) 7.1.5 fired, but the aṅga is √ās, not √śī: no ruṭ (Asate, not
        //     *Asrate). This is the clause an `||` → `&&` mutant drops.
        assert_eq!(
            form_g("As", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "Asate"
        );
        // (b) The aṅga IS √śī, but 7.1.5 never fired (empty log): the rule
        //     must decline and leave the ending untouched.
        let mut p = Prakriya {
            terms: vec![Term::new("SI"), Term::new(""), Term::new("ate")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "7.1.6").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "ate");
    }
}
