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
    // 3.4.85 loṭo laṅvat: loṭ behaves as laṅ, so the ṅit-conditioned rules
    // (3.4.99, 3.4.101) apply to it. An atideśa, so it is a rule and appears
    // in the trace rather than being folded into Context::new.
    Rule {
        id: "3.4.85",
        name: "loTo laNvat",
        kind: RuleKind::Atidesha,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) || p.ctx.is_ngit_like {
                return false;
            }
            let before = p.snapshot();
            p.ctx.is_ngit_like = true;
            p.record("3.4.85", "loTo laNvat", before);
            true
        },
    },
    // 3.4.101 tasthasthamipāṃ tāṃtaṃtāmaḥ: tas→tAm, Tas→tam, Ta→ta, mip→am.
    //
    // The mip→am arm is laṅ-only: loṭ's uttama-eka is `ni` by the more specific
    // 3.4.89 mer niḥ, so it must not be captured here.
    //
    // MUST precede 3.4.99: `tas`/`Tas` also end in `s`, and 3.4.99's guard
    // does not distinguish them from `vas`/`mas`. Ordering 3.4.101 first
    // substitutes tas/Tas/Ta/mi away before 3.4.99 can wrongly strip their
    // final `s` (verified by hand-tracing `aBavatAm`, which the reversed
    // order corrupts into a spurious `aBavata`).
    Rule {
        id: "3.4.101",
        name: "tasTasTamipAM tAMtaMtAmaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.ctx.is_ngit_like {
                return false;
            }
            let sub = match p.terms[ENDING_PRE_SHAP].text.as_str() {
                "tas" => "tAm",
                "Tas" => "tam",
                "Ta" => "ta",
                "mi" if matches!(p.ctx.lakara, Lakara::Lan) => "am",
                _ => return false,
            };
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = sub.into();
            p.record("3.4.101", "tasTasTamipAM tAMtaMtAmaH", before);
            true
        },
    },
    // 3.4.99 nityaṃ ṅitaḥ: the final `s` of a ṅit-lakāra's tiṅ is elided.
    // vas → va, mas → ma.
    Rule {
        id: "3.4.99",
        name: "nityaM NitaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.ctx.is_ngit_like
                || !matches!(p.terms[ENDING_PRE_SHAP].text.as_str(), "vas" | "mas")
            {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ENDING_PRE_SHAP].text.chars().collect();
            s.pop();
            p.terms[ENDING_PRE_SHAP].text = s.into_iter().collect();
            p.record("3.4.99", "nityaM NitaH", before);
            true
        },
    },
    // 3.4.87 ser hyapic ca: loṭ madhyama-eka `si` → `hi`.
    // Apavāda to 3.4.86 er uḥ, hence ordered before it.
    Rule {
        id: "3.4.87",
        name: "ser hyapic ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) || p.terms[ENDING_PRE_SHAP].text != "si" {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "hi".into();
            p.record("3.4.87", "ser hyapic ca", before);
            true
        },
    },
    // 3.4.89 mer niḥ: loṭ uttama-eka `mi` → `ni`.
    // Apavāda to 3.4.86 er uḥ, hence ordered before it.
    Rule {
        id: "3.4.89",
        name: "mer niH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) || p.terms[ENDING_PRE_SHAP].text != "mi" {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "ni".into();
            p.record("3.4.89", "mer niH", before);
            true
        },
    },
    // 3.4.86 er uḥ: the final `i` of the tiṅ → `u`. ti → tu, Ji → Ju.
    //
    // Guarded to exactly `ti`/`Ji` rather than "any i-final ending": `si` and
    // `mi` are preempted by the apavādas above, and by this point they have
    // already become `hi`/`ni`, which are also i-final. The explicit set makes
    // the preemption independent of ordering accidents.
    Rule {
        id: "3.4.86",
        name: "er uH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) {
                return false;
            }
            if !matches!(p.terms[ENDING_PRE_SHAP].text.as_str(), "ti" | "Ji") {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ENDING_PRE_SHAP].text.chars().collect();
            s.pop();
            s.push('u');
            p.terms[ENDING_PRE_SHAP].text = s.into_iter().collect();
            p.record("3.4.86", "er uH", before);
            true
        },
    },
    // 3.4.100 itaś ca: laṅ-only. The final `i` of laṅ's tiṅ is elided.
    // loṭ's final `i` is handled by 3.4.86 er uḥ (apavāda).
    // ti → t, si → s, Ji → J.
    Rule {
        id: "3.4.100",
        name: "itaS ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lan) || !p.terms[ENDING_PRE_SHAP].text.ends_with('i')
            {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ENDING_PRE_SHAP].text.chars().collect();
            s.pop();
            p.terms[ENDING_PRE_SHAP].text = s.into_iter().collect();
            p.record("3.4.100", "itaS ca", before);
            true
        },
    },
    // 3.4.92 āḍ uttamasya pic ca: the āṭ-āgama is prefixed to loṭ's uttama
    // endings. ni → Ani, va → Ava, ma → Ama.
    Rule {
        id: "3.4.92",
        name: "Aq uttamasya pic ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) || !matches!(p.ctx.purusha, Purusha::Uttama) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = format!("A{}", p.terms[ENDING_PRE_SHAP].text);
            p.record("3.4.92", "Aq uttamasya pic ca", before);
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
            if !matches!(p.ctx.lakara, Lakara::Lan) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = format!("a{}", p.terms[ANGA].text);
            p.record("6.4.71", "luNlaNlfNkzvaqudAttaH", before);
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
        name: "ato dIrGo yaYi",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // loṭ uttama gets its dīrgha from 3.4.92 āḍ + 6.1.101 instead.
            if matches!(p.ctx.lakara, Lakara::Lot) {
                return false;
            }
            let ending_first = p.terms[ENDING].text.chars().next().unwrap();
            if !matches!(ending_first, 'm' | 'v') || p.terms[SHAP].text != "a" {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "A".into();
            p.record("7.3.101", "ato dIrGo yaYi", before);
            true
        },
    },
    // 6.1.101 akaḥ savarṇe dīrghaḥ: śap `a` + the ending's initial `A`
    // (from 3.4.92 āḍ) coalesce to a single `A`. Bav + a + Ani → BavAni.
    Rule {
        id: "6.1.101",
        name: "akaH savarRe dIrGaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms[SHAP].text != "a" || !p.terms[ENDING].text.starts_with('A') {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "A".into();
            p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
            p.record("6.1.101", "akaH savarRe dIrGaH", before);
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
    // 6.4.105 ato heḥ: `hi` is elided after a short `a` (the śap).
    // Bav + a + hi → Bava.
    Rule {
        id: "6.4.105",
        name: "ato heH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms[SHAP].text != "a" || p.terms[ENDING].text != "hi" {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = String::new();
            p.record("6.4.105", "ato heH", before);
            true
        },
    },
    // 8.2.23 saṃyogāntasya lopaḥ: the final consonant of a word-final conjunct
    // is elided. aBavant → aBavan.
    Rule {
        id: "8.2.23",
        name: "saMyogAntasya lopaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let word = p.text();
            let mut tail = word.chars().rev();
            let (Some(last), Some(prev)) = (tail.next(), tail.next()) else {
                return false;
            };
            if is_vowel(last) || is_vowel(prev) {
                return false;
            }
            let before = p.snapshot();
            let idx = p.terms.len() - 1;
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.2.23", "saMyogAntasya lopaH", before);
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
