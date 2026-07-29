//! Varṇa and pratyāhāra classification: the sound layer the rules stand on.
//!
//! Pure functions over SLP1 characters, with no knowledge of terms, tags or
//! derivation state. Several are deliberately narrower than the pratyāhāra
//! they name — each says so, with the trigger for widening it.

use crate::prakriya::Prakriya;
use crate::tinanta::terms::{ANGA, SHAP};

/// Guṇa substitute of an ik vowel (1.1.2 aden guṇaḥ, applied by 7.3.84).
pub(crate) fn guna_of(v: char) -> Option<&'static str> {
    match v {
        'i' | 'I' => Some("e"),
        'u' | 'U' => Some("o"),
        'f' | 'F' => Some("ar"),
        'x' | 'X' => Some("al"),
        _ => None,
    }
}

/// Vṛddhi substitute of a vowel (1.1.1 vṛddhir ādaic; only the arms the
/// curated roots exercise via 6.1.90 — e/I from eD/Ikz, E from loṭ's 3.4.93).
pub(crate) fn vrddhi_of(v: char) -> Option<char> {
    match v {
        'a' | 'A' => Some('A'),
        'i' | 'I' | 'e' | 'E' => Some('E'),
        'u' | 'U' | 'o' | 'O' => Some('O'),
        _ => None,
    }
}

pub(crate) fn is_vowel(c: char) -> bool {
    matches!(
        c,
        'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O'
    )
}

/// A jhal (obstruent) — the set 8.4.55's target ranges over. For this slice
/// only `d` is exercised, but the classifier is written generally.
pub(crate) fn is_jhal(c: char) -> bool {
    matches!(
        c,
        'k' | 'K'
            | 'g'
            | 'G'
            | 'c'
            | 'C'
            | 'j'
            | 'J'
            | 'w'
            | 'W'
            | 'q'
            | 'Q'
            | 't'
            | 'T'
            | 'd'
            | 'D'
            | 'p'
            | 'P'
            | 'b'
            | 'B'
            | 'S'
            | 'z'
            | 's'
            | 'h'
    )
}

/// A khar (voiceless obstruent) — the trigger of 8.4.55 (khari ca).
pub(crate) fn is_khar(c: char) -> bool {
    matches!(
        c,
        'k' | 'K' | 'c' | 'C' | 'w' | 'W' | 't' | 'T' | 'p' | 'P' | 'S' | 'z' | 's'
    )
}

/// 8.4.1's trigger set: `r`, `z`, and the r-vowels `f`/`F`, which contain the
/// r-sound by 1.1.51 *uraṇ raparaḥ*. `S` (the palatal śa) is deliberately
/// absent — it is not `z` (the retroflex ṣa) despite the visual similarity,
/// so a following `n` stays dental across it.
pub(crate) fn is_natva_trigger(c: char) -> bool {
    matches!(c, 'r' | 'z' | 'f' | 'F')
}

/// 8.4.2's intervention set: aṭ (the vowels plus `h y v r`), ku (`k K g G N`)
/// and pu (`p P b B m`).
///
/// The sūtra also names **āṅ** and **num**, which are morphemes rather than
/// varṇa classes. Ṇatva runs in the tripādī over assembled text, where
/// morpheme identity is gone — and neither is a loss: āṅ is the upasarga `ā`,
/// already an aṭ vowel, and num's nasal cannot occur in the intervening
/// position for any form in the covered grammar (no num-infixing root is in
/// scope, and upasargas are out of scope entirely). Revisit when either
/// enters scope.
///
/// Note `r` and the r-vowels are BOTH triggers and interveners. Callers must
/// test for a trigger first; see 8.4.2's backward scan.
pub(crate) fn is_natva_intervener(c: char) -> bool {
    is_vowel(c)
        || matches!(
            c,
            'h' | 'y' | 'v' | 'r' | 'k' | 'K' | 'g' | 'G' | 'N' | 'p' | 'P' | 'b' | 'B' | 'm'
        )
}

/// The car (voiceless unaspirated) substitute of a jhal, per 8.4.55.
/// Only `d → t` is exercised this slice; extend as later roots demand.
pub(crate) fn cartva_of(c: char) -> Option<char> {
    match c {
        'd' | 'D' | 't' | 'T' => Some('t'),
        'g' | 'G' | 'k' | 'K' => Some('k'),
        'b' | 'B' | 'p' | 'P' => Some('p'),
        'j' | 'J' | 'c' | 'C' => Some('c'),
        'q' | 'Q' | 'w' | 'W' => Some('w'),
        _ => None,
    }
}

/// 1.3.4 na vibhaktau tusmāḥ: a final tu-varga (t/T/d/D/n), `s`, or `m` of a
/// vibhakti is NOT an it, so the shared halantyam elision must be suppressed
/// for such tiṅ endings (e.g. tas, Tas, vas, mas keep their final `s`).
pub(crate) fn is_vibhakti_protected_final(c: char) -> bool {
    matches!(c, 't' | 'T' | 'd' | 'D' | 'n' | 's' | 'm')
}

/// Is śnu's `u` *asaṁyogapūrva* — preceded by a single consonant rather
/// than a conjunct?
///
/// The condition 6.4.87 inherits by anuvṛtti from 6.4.82 *er anekāco'saṁ-
/// yogapūrvasya*, and the same condition 6.4.106 states in its own text.
/// The `u` is always preceded by śnu's own `n`, so the question reduces to
/// whether that `n` follows a vowel — i.e. whether the aṅga's final
/// character is a vowel.
///
/// √hi and √ri qualify (`hinu`, `riRu`); √āp, √śak, √aś and √ṣṭigh do not
/// (`Apnu`, `Saknu`, `aSnu`, `stiGnu`). √aś is the counter-intuitive one:
/// it looks like √su, but the root's own final `S` joins śnu's `n` into a
/// conjunct — which is why `aSnumahe` has no lopa alternate while
/// `sunmahe` does.
///
/// Returns false whenever the vikaraṇa is not śnu, so callers do not need
/// their own gaṇa test.
pub(crate) fn shnu_asamyogapurva(p: &Prakriya) -> bool {
    if p.terms.get(SHAP).map(|t| t.text.as_str()) != Some("nu") {
        return false;
    }
    p.terms[ANGA].text.chars().last().is_some_and(is_vowel)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::term::{Tag, Term};

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
    fn vrddhi_of_ac_vowels_all_arms() {
        // 1.1.1 vRddhir Adaic: pin every arm of the vRddhi substitution
        // table directly, since the curated roots only ever drive
        // vrddhi_of through 6.1.90 with e/I/E inputs (never a/A/u/U/o/O),
        // leaving those arms unreachable via golden derivations. Mirrors
        // guna_of_ik_vowels_all_arms above.
        assert_eq!(vrddhi_of('a'), Some('A'));
        assert_eq!(vrddhi_of('A'), Some('A'));
        assert_eq!(vrddhi_of('i'), Some('E'));
        assert_eq!(vrddhi_of('I'), Some('E'));
        assert_eq!(vrddhi_of('e'), Some('E'));
        assert_eq!(vrddhi_of('E'), Some('E'));
        assert_eq!(vrddhi_of('u'), Some('O'));
        assert_eq!(vrddhi_of('U'), Some('O'));
        assert_eq!(vrddhi_of('o'), Some('O'));
        assert_eq!(vrddhi_of('O'), Some('O'));
        // Non-ac letters (consonants) have no vRddhi substitute.
        assert_eq!(vrddhi_of('t'), None);
        assert_eq!(vrddhi_of('f'), None);
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
    fn natva_trigger_is_ra_sha_and_the_r_vowels() {
        // 8.4.1 "razAByAm": r and z. f/F (R/RR) count too -- they contain the
        // r-sound by 1.1.51 uraN raparaH, and that is the ONLY reason vfN
        // retroflexes (vf + nIte -> vfRIte).
        for c in ['r', 'z', 'f', 'F'] {
            assert!(is_natva_trigger(c), "{c} should trigger Natva");
        }
        // S is NOT z: a following n stays dental across it (avartanta's t
        // is the existing golden that pins the analogous non-trigger case).
        for c in ['S', 's', 'n', 'a', 'l', 'v'] {
            assert!(!is_natva_trigger(c), "{c} should not trigger Natva");
        }
    }

    #[test]
    fn natva_intervener_is_at_ku_pu_and_nothing_else() {
        // 8.4.2 aw-ku-pu-AN-num-vyavAye'pi. aw = the vowels plus h y v r.
        for c in [
            'a', 'A', 'i', 'I', 'u', 'U', 'f', 'F', 'x', 'X', 'e', 'E', 'o', 'O', 'h', 'y', 'v',
            'r',
        ] {
            assert!(is_natva_intervener(c), "aw member {c} should intervene");
        }
        // ku = k K g G N
        for c in ['k', 'K', 'g', 'G', 'N'] {
            assert!(is_natva_intervener(c), "ku member {c} should intervene");
        }
        // pu = p P b B m
        for c in ['p', 'P', 'b', 'B', 'm'] {
            assert!(is_natva_intervener(c), "pu member {c} should intervene");
        }
        // Everything else BREAKS the intervention. `t` is the one that
        // protects an existing golden (avartanta); `S` and the retroflex `R`
        // itself are the same non-trigger, non-intervener shape but have no
        // curated root exercising them yet.
        for c in [
            'S', 's', 'z', 't', 'T', 'd', 'D', 'n', 'R', 'c', 'j', 'w', 'q', 'l',
        ] {
            assert!(!is_natva_intervener(c), "{c} must break intervention");
        }
    }

    #[test]
    fn shnu_asamyogapurva_is_true_exactly_for_the_vowel_final_roots() {
        // The `u` of śnu is asaṁyogapūrva iff the `n` follows a vowel — i.e.
        // iff the aṅga's final character is a vowel. A wrong predicate here
        // turns hinu into *hinuhi and Apnuhi into *Apnu, both of which look
        // like plausible Sanskrit, so enumerate rather than rely on goldens.
        //
        // BU carries its real bhvādi vikaraṇa text ("a", not "nu"): unlike
        // the other controls, its point is to pin the helper's OTHER guard —
        // "vikaraṇa is not śnu at all" — which only engages when SHAP's text
        // truly isn't "nu". BU ends in the vowel U, so pairing it with a
        // literal "nu" SHAP (as every other row does) would make the
        // vowel-final check itself return true, giving a false positive that
        // masks exactly the silent-failure risk this test exists to catch.
        for (root, vikarana, expected) in [
            ("hi", "nu", true),    // svādi, vowel-final
            ("ri", "nu", true),    // svādi, vowel-final
            ("Ap", "nu", false),   // svādi, `pn` conjunct
            ("Sak", "nu", false),  // svādi, `kn` conjunct
            ("aS", "nu", false),   // svādi, `Sn` conjunct — the counter-intuitive one
            ("stiG", "nu", false), // svādi, `Gn` conjunct
            ("kliS", "nu", false), // kryādi control (consonant-final guard)
            ("BU", "a", false),    // bhvādi control (vikaraṇa-is-not-śnu guard)
        ] {
            let mut p = Prakriya {
                terms: vec![Term::new(root), Term::new(vikarana), Term::new("anti")],
                ..Default::default()
            };
            p.terms[SHAP].add(Tag::Vikarana);
            assert_eq!(
                shnu_asamyogapurva(&p),
                expected,
                "{root}: asaṁyogapūrva should be {expected}"
            );
        }
    }
}
