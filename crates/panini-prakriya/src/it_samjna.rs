use crate::prakriya::Prakriya;
use crate::term::{Tag, Term};

/// Consonants that are always `it` when final in a pratyaya (1.3.3 halantyam).
fn is_hal(c: char) -> bool {
    !matches!(
        c,
        'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O'
    )
}

pub fn run_it_samjna(term: &mut Term, p: &mut Prakriya, idx: usize) {
    let before = p.snapshot();
    let original = term.text.clone();
    let mut s: Vec<char> = original.chars().collect();

    // 1.3.8 laSakvataddhite: initial S, z, or ku-class marker of a pratyaya is it.
    // v1: only leading `S` occurs (Sap).
    if matches!(s.first(), Some('S') | Some('z')) {
        s.remove(0);
    }
    // 1.3.3 halantyam: final consonant is it.
    if let Some(&last) = s.last() {
        if is_hal(last) {
            s.pop();
        }
    }
    let reduced: String = s.into_iter().collect();
    if reduced != original {
        term.text = reduced;
        // Reflect into the live prakriya then log 1.3.9 tasya lopaH.
        p.terms[idx].text = term.text.clone();
        p.record("1.3.9", "tasya lopaH", before);
    }
    let _ = Tag::It; // Tag reserved for future explicit it-marking.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prakriya::Prakriya;
    use crate::term::Term;

    #[test]
    fn shap_loses_sh_and_leaves_a() {
        let mut p = Prakriya {
            terms: vec![Term::new("Sap")],
            log: vec![],
        };
        let mut t = p.terms[0].clone();
        run_it_samjna(&mut t, &mut p, 0);
        p.terms[0] = t;
        assert_eq!(p.terms[0].text, "a");
        assert!(p.log.iter().any(|s| s.sutra == "1.3.9"));
    }

    #[test]
    fn tip_loses_final_p() {
        let mut p = Prakriya {
            terms: vec![Term::new("tip")],
            log: vec![],
        };
        let mut t = p.terms[0].clone();
        run_it_samjna(&mut t, &mut p, 0);
        p.terms[0] = t;
        assert_eq!(p.terms[0].text, "ti");
    }
}
