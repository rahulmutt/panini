use crate::it_samjna::run_it_samjna;
use crate::prakriya::Prakriya;
use crate::term::{Tag, Term};
use panini_data::{tin_ending, Dhatu, Lakara, Pada, Purusha, Vacana};

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

pub fn derive(
    dhatu: &Dhatu,
    _lakara: Lakara,
    pada: Pada,
    purusha: Purusha,
    vacana: Vacana,
) -> Prakriya {
    let mut p = Prakriya::default();
    p.terms.push({
        let mut t = Term::new(dhatu.code);
        t.add(Tag::Dhatu);
        t
    });

    // 3.4.78 tiptasjhi...: replace laṭ by the tiṅ ending.
    // 3.4.113 tiṅ-śit sārvadhātukam makes it sārvadhātuka.
    let ending = tin_ending(pada, purusha, vacana);
    {
        let before = p.snapshot();
        let mut e = Term::new(ending);
        e.add(Tag::Tin);
        e.add(Tag::Sarvadhatuka);
        p.terms.push(e);
        p.record("3.4.78", "tiptasjhisipthasthamipvasmas", before);
    }

    // it-samjna on the tiṅ ending (elide anubandhas), respecting 1.3.4:
    // the final s/t/m of a vibhakti is protected, so only endings whose final
    // is a genuine anubandha (tip/sip/mip → the pit marker `p`) are reduced.
    {
        let last = p.terms[1].text.chars().last();
        let protected = last.map(is_vibhakti_protected_final).unwrap_or(false);
        if !protected {
            let mut e = p.terms[1].clone();
            run_it_samjna(&mut e, &mut p, 1);
            p.terms[1] = e;
        }
    }

    // 3.1.68 kartari śap: insert śap between dhātu and ending.
    {
        let before = p.snapshot();
        let mut s = Term::new("Sap");
        s.add(Tag::Vikarana);
        s.add(Tag::Sarvadhatuka);
        p.terms.insert(1, s);
        p.record("3.1.68", "kartari Sap", before);
    }
    // it-samjna on śap (→ `a`); mark the dhātu an aṅga.
    {
        let mut s = p.terms[1].clone();
        run_it_samjna(&mut s, &mut p, 1);
        p.terms[1] = s;
    }
    p.terms[0].add(Tag::Anga);

    // 7.1.3 jho'ntaḥ: the `J` of `Ji` (prathama-bahu) → `ant`, yielding `anti`.
    if p.terms[2].text == "Ji" {
        let before = p.snapshot();
        p.terms[2].text = "anti".into();
        p.record("7.1.3", "jho'ntaH", before);
    }

    // 7.3.84 sārvadhātukārdhadhātukayoḥ: guṇa of the aṅga's final ik.
    {
        let last = p.terms[0].text.chars().last().unwrap();
        if let Some(g) = guna_of(last) {
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[0].text.chars().collect();
            s.pop();
            p.terms[0].text = s.into_iter().collect::<String>() + g;
            p.record("7.3.84", "sArvadhAtukArdhadhAtukayoH", before);
        }
    }

    // 6.1.78 eco'yavāyāvaḥ: e/o/E/O before a vowel → ay/av/Ay/Av.
    {
        let anga_last = p.terms[0].text.chars().last().unwrap();
        let next_first = p.terms[1].text.chars().next().unwrap();
        let sub = match anga_last {
            'e' => Some("ay"),
            'o' => Some("av"),
            'E' => Some("Ay"),
            'O' => Some("Av"),
            _ => None,
        };
        if let (Some(sub), true) = (sub, is_vowel(next_first)) {
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[0].text.chars().collect();
            s.pop();
            p.terms[0].text = s.into_iter().collect::<String>() + sub;
            p.record("6.1.78", "eco'yavAyAvaH", before);
        }
    }

    // 6.1.97 ato guṇe: a short `a` (the śap) followed by a guṇa vowel yields
    // para-rūpa — a single vowel identical to the following one. For the `anti`
    // ending (Ji → anti), śap `a` + initial `a` of `anti` → a single short `a`
    // (NOT savarṇa-dīrgha `A`), so `Bav`+`a`+`nti` = `Bavanti`. Drop the
    // ending's leading `a`; the surviving śap `a` stands in for the coalesced
    // vowel and the term vector stays consistent for `.text()`.
    if p.terms[1].text == "a" && p.terms[2].text.starts_with('a') {
        let before = p.snapshot();
        p.terms[2].text = p.terms[2].text.chars().skip(1).collect();
        p.record("6.1.97", "ato guRe", before);
    }

    // 7.3.101 ato dīrgho yañi: aṅga-final `a` (śap) → `A` before mi/vas/mas.
    {
        let ending_first = p.terms[2].text.chars().next().unwrap();
        if (ending_first == 'm' || ending_first == 'v') && p.terms[1].text == "a" {
            let before = p.snapshot();
            p.terms[1].text = "A".into();
            p.record("7.3.101", "ato dIrgho yaYi", before);
        }
    }

    // 8.2.66 sasajuṣo ruḥ + 8.3.15 kharavasānayoḥ: word-final `s` → visarga `H`.
    if p.terms.last().unwrap().text.ends_with('s') {
        let before = p.snapshot();
        let idx = p.terms.len() - 1;
        let mut s: Vec<char> = p.terms[idx].text.chars().collect();
        s.pop();
        s.push('H');
        p.terms[idx].text = s.into_iter().collect();
        p.record("8.3.15", "kharavasAnayoH visarjanIyaH", before);
    }

    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use panini_data::{dhatus, Lakara, Pada, Purusha, Vacana};

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
