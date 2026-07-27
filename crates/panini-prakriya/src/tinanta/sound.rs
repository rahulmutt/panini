//! Varṇa and pratyāhāra classification: the sound layer the rules stand on.
//!
//! Pure functions over SLP1 characters, with no knowledge of terms, tags or
//! derivation state. Several are deliberately narrower than the pratyāhāra
//! they name — each says so, with the trigger for widening it.

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
