use crate::context::Context;
use crate::controller::run_pipeline;
use crate::it_samjna::run_it_samjna;
use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use panini_data::{Dhatu, Lakara, Pada, Purusha, Vacana, tin_ending};

/// Guṇa substitute of an ik vowel (1.1.2 aden guṇaḥ, applied by 7.3.84).
fn guna_of(v: char) -> Option<&'static str> {
    match v {
        'i' | 'I' => Some("e"),
        'u' | 'U' => Some("o"),
        'f' | 'F' => Some("ar"),
        'x' | 'X' => Some("al"),
        _ => None,
    }
}

fn is_vowel(c: char) -> bool {
    matches!(
        c,
        'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O'
    )
}

/// 1.3.4 na vibhaktau tusmāḥ: a final tu-varga (t/T/d/D/n), `s`, or `m` of a
/// vibhakti is NOT an it, so the shared halantyam elision must be suppressed
/// for such tiṅ endings (e.g. tas, Tas, vas, mas keep their final `s`).
fn is_vibhakti_protected_final(c: char) -> bool {
    matches!(c, 't' | 'T' | 'd' | 'D' | 'n' | 's' | 'm')
}

/// Index of the aṅga (the dhātu) in `terms`. Stable across the pipeline.
const ANGA: usize = 0;

/// Index of the tiṅ ending *before* śap is inserted (3.1.68).
const ENDING_PRE_SHAP: usize = 1;

/// Index of śap once inserted, and of the ending thereafter.
const SHAP: usize = 1;
const ENDING: usize = 2;

/// The ordered rule list. Read it top to bottom against the Aṣṭādhyāyī: this
/// sequence IS the grammar this crate implements. Every rule self-guards and
/// returns whether it fired.
pub static TINANTA_RULES: &[Rule] = &[
    // 3.4.78 tiptasjhi...: replace the lakāra by the tiṅ ending.
    // 3.4.113 tiṅ-śit sārvadhātukam makes it sārvadhātuka.
    Rule {
        id: "3.4.78",
        name: "tiptasjhisipthasthamipvasmas",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let before = p.snapshot();
            let ending = tin_ending(p.ctx.pada, p.ctx.purusha, p.ctx.vacana);
            let mut e = Term::new(ending);
            e.add(Tag::Tin);
            e.add(Tag::Sarvadhatuka);
            p.terms.push(e);
            p.record("3.4.78", "tiptasjhisipthasthamipvasmas", before);
            true
        },
    },
    // it-samjña on the tiṅ ending (1.3.3 halantyam / 1.3.9 tasya lopaḥ),
    // respecting 1.3.4: the final s/t/m of a vibhakti is protected, so only
    // endings whose final is a genuine anubandha (tip/sip/mip → the pit marker
    // `p`) are reduced.
    //
    // This MUST precede the lakāra-specific substitutions below: 3.4.100
    // itaś ca elides the `i` of `tip`, and that `i` is only exposed once
    // halantyam has stripped the `p`.
    Rule {
        id: "1.3.9",
        name: "tasya lopaH",
        kind: RuleKind::Vidhi,
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
    // 3.1.68 kartari śap: insert śap between dhātu and ending, run it-samjña
    // on it (Sap → a), and mark the dhātu an aṅga.
    Rule {
        id: "3.1.68",
        name: "kartari Sap",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let before = p.snapshot();
            let mut s = Term::new("Sap");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.68", "kartari Sap", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP);
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 7.1.3 jho'ntaḥ: a leading `J` of the ending → `ant`.
    Rule {
        id: "7.1.3",
        name: "jho'ntaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ENDING].text.starts_with('J') {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("ant{rest}");
            p.record("7.1.3", "jho'ntaH", before);
            true
        },
    },
    // 7.3.84 sārvadhātukārdhadhātukayoḥ: guṇa of the aṅga's final ik.
    Rule {
        id: "7.3.84",
        name: "sArvadhAtukArdhadhAtukayoH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let last = p.terms[ANGA].text.chars().last().unwrap();
            let Some(g) = guna_of(last) else {
                return false;
            };
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ANGA].text.chars().collect();
            s.pop();
            p.terms[ANGA].text = s.into_iter().collect::<String>() + g;
            p.record("7.3.84", "sArvadhAtukArdhadhAtukayoH", before);
            true
        },
    },
    // 6.1.78 eco'yavāyāvaḥ: e/o/E/O before a vowel → ay/av/Ay/Av.
    Rule {
        id: "6.1.78",
        name: "eco'yavAyAvaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let anga_last = p.terms[ANGA].text.chars().last().unwrap();
            let next_first = p.terms[SHAP].text.chars().next().unwrap();
            let sub = match anga_last {
                'e' => "ay",
                'o' => "av",
                'E' => "Ay",
                'O' => "Av",
                _ => return false,
            };
            if !is_vowel(next_first) {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ANGA].text.chars().collect();
            s.pop();
            p.terms[ANGA].text = s.into_iter().collect::<String>() + sub;
            p.record("6.1.78", "eco'yavAyAvaH", before);
            true
        },
    },
    // 7.3.101 ato dīrgho yañi: aṅga-final `a` (śap) → `A` before a yañ-initial
    // sārvadhātuka ending (here: mi/vas/mas).
    Rule {
        id: "7.3.101",
        name: "ato dIrgho yaYi",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let ending_first = p.terms[ENDING].text.chars().next().unwrap();
            if !matches!(ending_first, 'm' | 'v') || p.terms[SHAP].text != "a" {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "A".into();
            p.record("7.3.101", "ato dIrgho yaYi", before);
            true
        },
    },
    // 6.1.97 ato guṇe: a short `a` (the śap) followed by a guṇa vowel yields
    // para-rūpa — a single vowel identical to the following one. For the `anti`
    // ending (Ji → anti), śap `a` + initial `a` of `anti` → a single short `a`
    // (NOT savarṇa-dīrgha `A`), so `Bav`+`a`+`nti` = `Bavanti`. Drop the
    // ending's leading `a`; the surviving śap `a` stands in for the coalesced
    // vowel and the term vector stays consistent for `.text()`.
    Rule {
        id: "6.1.97",
        name: "ato guRe",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms[SHAP].text != "a" || !p.terms[ENDING].text.starts_with('a') {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
            p.record("6.1.97", "ato guRe", before);
            true
        },
    },
    // 8.2.66 sasajuṣo ruḥ + 8.3.15 kharavasānayoḥ: word-final `s` → visarga.
    Rule {
        id: "8.3.15",
        name: "kharavasAnayoH visarjanIyaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.text().ends_with('s') {
                return false;
            }
            let before = p.snapshot();
            let idx = p.terms.len() - 1;
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            s.push('H');
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.3.15", "kharavasAnayoH visarjanIyaH", before);
            true
        },
    },
];

pub fn derive(
    dhatu: &Dhatu,
    lakara: Lakara,
    pada: Pada,
    purusha: Purusha,
    vacana: Vacana,
) -> Prakriya {
    let mut p = Prakriya {
        ctx: Context::new(lakara, pada, purusha, vacana),
        ..Default::default()
    };
    p.terms.push({
        let mut t = Term::new(dhatu.code);
        t.add(Tag::Dhatu);
        t
    });
    run_pipeline(&mut p, TINANTA_RULES);
    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};

    fn form(code: &str, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, Lakara::Lat, Pada::Parasmaipada, pu, va).text()
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
    fn guna_of_ik_vowels_all_arms() {
        // 1.1.2 aden guNaH: pin every arm of the ik -> guNa substitution table,
        // not just the ones a v1 golden root happens to touch.
        assert_eq!(guna_of('i'), Some("e"));
        assert_eq!(guna_of('I'), Some("e"));
        assert_eq!(guna_of('u'), Some("o"));
        assert_eq!(guna_of('U'), Some("o"));
        assert_eq!(guna_of('f'), Some("ar"));
        assert_eq!(guna_of('F'), Some("ar"));
        assert_eq!(guna_of('x'), Some("al"));
        assert_eq!(guna_of('X'), Some("al"));
        // Non-ik letters (consonants, and non-ik vowels like `a`) have no guNa
        // substitute.
        assert_eq!(guna_of('a'), None);
        assert_eq!(guna_of('t'), None);
    }

    #[test]
    fn is_vowel_distinguishes_vowels_from_consonants() {
        for c in [
            'a', 'A', 'i', 'I', 'u', 'U', 'f', 'F', 'x', 'X', 'e', 'E', 'o', 'O',
        ] {
            assert!(is_vowel(c), "{c} should be a vowel");
        }
        for c in ['t', 'k', 'p', 's', 'm'] {
            assert!(!is_vowel(c), "{c} should not be a vowel");
        }
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
        let d = dhatus().iter().find(|d| d.code == "BU").unwrap();
        let p = derive(
            d,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert!(p.log.iter().any(|s| s.sutra == "3.1.68"));
        assert!(p.log.iter().any(|s| s.sutra == "7.3.84"));
        assert!(!p.log.is_empty());
    }
}
