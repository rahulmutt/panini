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

/// A jhal (obstruent) — the set 8.4.55's target ranges over. `d` was the
/// only member this suite exercised when that comment was first written;
/// rudhādi has since brought `t`, `s` and `D` into play too (via 8.3.24,
/// 8.4.53 and 6.4.101 respectively), but the classifier was written
/// generally from the start and needed no change for any of them.
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

/// A jhaś (voiced aspirated stop) — the five vargas' fourth member, `G J Q D
/// B`. 8.4.53's conditioning class: a jhal immediately before a jhaś becomes
/// its own jaś (`jashtva_of`) — the conditioning jhaś's place plays no part
/// in the substitute.
pub(crate) fn is_jhash(c: char) -> bool {
    matches!(c, 'G' | 'J' | 'Q' | 'D' | 'B')
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

/// The *jaś* (voiced unaspirated) counterpart of a jhal, by place of
/// articulation. `ṣ` (z) is the one sibilant with a jaś here: it has none by
/// place-and-manner correspondence (the sibilants are not stops), but 1.1.50
/// sthāne'ntaratamaḥ selects the nearest substitute, which for retroflex ṣ is
/// retroflex ḍ (q). `S` and `s` deliberately stay `None` — `S` is unreached by
/// any curated root, and a word-final `s` is 8.2.66 / 8.3.15's business, not
/// jaśtva's. `h` also has no jaś.
pub(crate) fn jashtva_of(c: char) -> Option<char> {
    Some(match c {
        'k' | 'K' | 'g' | 'G' => 'g',
        'c' | 'C' | 'j' | 'J' => 'j',
        'w' | 'W' | 'q' | 'Q' | 'z' => 'q',
        't' | 'T' | 'd' | 'D' => 'd',
        'p' | 'P' | 'b' | 'B' => 'b',
        _ => return None,
    })
}

/// The *ku* (velar) counterpart of a cu sound — 8.2.30 coH kuH's substitute.
/// By 1.1.50 sthAne'ntaratamaH the nearest substitute preserves voicing and
/// aspiration, so `c` goes to `k` and `j` to `g`, never both to one letter.
///
/// `C` and `J` have no curated witness — no aspirate-cu-final root is in
/// scope — and are present anyway because the table covers every cu STOP
/// arm, the same reason `jashtva_of` carries its 1.1.50-derived `z -> q`
/// arm. `kutva_of_cu_all_arms` is what keeps them from rotting.
///
/// `Y` (ñ) is cu too, and this table deliberately omits its arm. `cu~` is
/// udit, so by 1.1.69 it denotes the whole varga including the nasal, and
/// ñ -> ṅ (prāñc -> prāṅ) is 8.2.30's classic witness — vidyut-prakriya's
/// `map("cu~", "ku~")` includes it. But every `Y` this engine produces
/// comes from 8.4.58, which runs BELOW 8.2.30 in the tripadi ordering, and
/// no curated root's stem carries one going in, so the arm is unreachable
/// from a tiṅanta corpus. A subanta slice reaching prāñc -> prāṅ must add
/// `Y -> N` here.
///
/// The velars are deliberately absent rather than mapped to themselves: they
/// are already ku, not cu, and `None` is what lets 8.2.30 use this single
/// lookup as its match test as well as its substitute.
pub(crate) fn kutva_of(c: char) -> Option<char> {
    Some(match c {
        'c' => 'k',
        'C' => 'K',
        'j' => 'g',
        'J' => 'G',
        _ => return None,
    })
}

/// The homorganic nasal of a *yay*. Covers only the stops — yay's
/// semivowel arm (`y v r l`) is unreached while 8.3.24 fires solely before
/// a jhal, and jhal excludes semivowels, so no anusvāra this engine
/// produces is ever followed by one. `None` covers both śal (the
/// sibilants and `h`, 8.4.58's real declining case) and that unreached
/// semivowel gap — widen the match, not this comment, if a future slice
/// lets 8.3.24 leave an anusvāra before `y v r l`.
pub(crate) fn parasavarna_of(c: char) -> Option<char> {
    Some(match c {
        'k' | 'K' | 'g' | 'G' | 'N' => 'N',
        'c' | 'C' | 'j' | 'J' | 'Y' => 'Y',
        'w' | 'W' | 'q' | 'Q' | 'R' => 'R',
        't' | 'T' | 'd' | 'D' | 'n' => 'n',
        'p' | 'P' | 'b' | 'B' | 'm' => 'm',
        // Semivowels are yay too, but unreached (see doc comment above);
        // no arm returns a value for them, so they fall to `None` below
        // along with śal.
        _ => return None,
    })
}

/// Are the two sounds *savarṇa* — same place and same manner of closure?
/// For 8.4.65's purposes that reduces to sharing a stop series: `t` and
/// `T` are savarṇa, `s` and `t` are not.
pub(crate) fn is_savarna(a: char, b: char) -> bool {
    fn series(c: char) -> Option<u8> {
        Some(match c {
            'k' | 'K' | 'g' | 'G' => 0,
            'c' | 'C' | 'j' | 'J' => 1,
            'w' | 'W' | 'q' | 'Q' => 2,
            't' | 'T' | 'd' | 'D' => 3,
            'p' | 'P' | 'b' | 'B' => 4,
            _ => return None,
        })
    }
    match (series(a), series(b)) {
        (Some(x), Some(y)) => x == y,
        _ => false,
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
    fn parasavarna_of_stops_all_arms() {
        // 8.4.58 anusvArasya yayi parasavarRaH: pin every varga's stop-arm
        // directly, since only the dental arm (t/d -> n) is reachable from
        // 7a's golden forms, and both reachable inputs (hiMs + taH's `t`,
        // kfMt + a's `t`) map to that same `n` -- a mutant rewriting the
        // velar, palatal, retroflex or labial arm to any other nasal would
        // be invisible to the whole suite without this.
        for c in ['k', 'K', 'g', 'G', 'N'] {
            assert_eq!(parasavarna_of(c), Some('N'), "{c} should parasavarna to N");
        }
        for c in ['c', 'C', 'j', 'J', 'Y'] {
            assert_eq!(parasavarna_of(c), Some('Y'), "{c} should parasavarna to Y");
        }
        for c in ['w', 'W', 'q', 'Q', 'R'] {
            assert_eq!(parasavarna_of(c), Some('R'), "{c} should parasavarna to R");
        }
        for c in ['t', 'T', 'd', 'D', 'n'] {
            assert_eq!(parasavarna_of(c), Some('n'), "{c} should parasavarna to n");
        }
        for c in ['p', 'P', 'b', 'B', 'm'] {
            assert_eq!(parasavarna_of(c), Some('m'), "{c} should parasavarna to m");
        }
        // Sal (the sibilants and h) declines -- this is 8.4.58's real
        // guard case (hiMs + taH keeps its anusvAra before `s`).
        for c in ['s', 'S', 'z', 'h'] {
            assert_eq!(parasavarna_of(c), None, "{c} should not parasavarna");
        }
    }

    #[test]
    fn jashtva_of_stops_all_arms() {
        // 8.4.53 JalAM jaS JaSi: pin every varga's stop-arm directly, since
        // only the dental arm (t/T/d/D -> d) is reachable from 7a's golden
        // forms (kfnt + Di -> kfndDi) -- a mutant rewriting the velar,
        // palatal, retroflex or labial arm to any other jaś would be
        // invisible to the whole suite without this. Mirrors
        // parasavarna_of_stops_all_arms above.
        for c in ['k', 'K', 'g', 'G'] {
            assert_eq!(jashtva_of(c), Some('g'), "{c} should jashtva to g");
        }
        for c in ['c', 'C', 'j', 'J'] {
            assert_eq!(jashtva_of(c), Some('j'), "{c} should jashtva to j");
        }
        for c in ['w', 'W', 'q', 'Q'] {
            assert_eq!(jashtva_of(c), Some('q'), "{c} should jashtva to q");
        }
        for c in ['t', 'T', 'd', 'D'] {
            assert_eq!(jashtva_of(c), Some('d'), "{c} should jashtva to d");
        }
        for c in ['p', 'P', 'b', 'B'] {
            assert_eq!(jashtva_of(c), Some('b'), "{c} should jashtva to b");
        }
        // h has no jaś counterpart.
        for c in ['s', 'h'] {
            assert_eq!(jashtva_of(c), None, "{c} should not jashtva");
        }
        // ṣ has no jaś by place alone — the sibilants are not stops. 1.1.50
        // sthāne'ntaratamaḥ selects the nearest, which for retroflex ṣ is
        // retroflex ḍ. `S` and `s` stay absent: `S` is unreachable here,
        // and a word-final `s` is 8.2.66 / 8.3.15's, not jaśtva's.
        assert_eq!(jashtva_of('z'), Some('q'));
        assert_eq!(jashtva_of('S'), None);
        assert_eq!(jashtva_of('s'), None);
    }

    #[test]
    fn kutva_of_cu_all_arms() {
        // 8.2.30 coH kuH: pin every arm of the cu -> ku substitution table
        // directly. Only `j -> g` (√bhañj, √yuj) and `c -> k` (√ric, √vic)
        // are reachable from the golden forms, so a mutant rewriting the
        // aspirate arms would be invisible to the whole suite without this.
        // Mirrors jashtva_of_stops_all_arms above.
        //
        // 1.1.50 sthAne'ntaratamaH picks the NEAREST velar, so voicing and
        // aspiration carry across: voiceless unaspirated `c` goes to the
        // voiceless unaspirated `k`, never to `g`.
        assert_eq!(kutva_of('c'), Some('k'));
        assert_eq!(kutva_of('C'), Some('K'));
        assert_eq!(kutva_of('j'), Some('g'));
        assert_eq!(kutva_of('J'), Some('G'));

        // The velars are already ku and are not cu; the rule must not
        // re-fire on its own output.
        for c in ['k', 'K', 'g', 'G', 'N'] {
            assert_eq!(kutva_of(c), None, "{c} is ku already, not cu");
        }
        // `S` (ś) is genuinely not cu -- 8.2.30's `coH` names the stops,
        // not the sibilant. `Y` (ñ) IS cu (`cu~` is udit, so 1.1.69 pulls
        // the nasal in too) but the table omits that arm as unreachable
        // from this tiṅanta corpus; see kutva_of's doc comment.
        for c in ['Y', 'S'] {
            assert_eq!(kutva_of(c), None, "{c} is not a cu stop");
        }
        // Off-domain sanity: a vowel and a dental.
        for c in ['a', 't'] {
            assert_eq!(kutva_of(c), None, "{c} should not kutva");
        }
    }

    #[test]
    fn is_savarna_stop_series_all_arms() {
        // 8.4.65 jharo jhari savarṇe: pin every varga's stop-series directly,
        // since only the dental arm (t/T/d/D) is reachable from 7a's golden
        // forms (kfnttaH, kfndDi, Kintte) -- a mutant collapsing the velar,
        // palatal, retroflex or labial series into the dental one would be
        // invisible to the whole suite without this. Mirrors
        // parasavarna_of_stops_all_arms above.
        for (a, b) in [('k', 'K'), ('g', 'G'), ('k', 'g'), ('K', 'G')] {
            assert!(is_savarna(a, b), "{a}/{b} should be savarRa (ka-varga)");
        }
        for (a, b) in [('c', 'C'), ('j', 'J'), ('c', 'j'), ('C', 'J')] {
            assert!(is_savarna(a, b), "{a}/{b} should be savarRa (ca-varga)");
        }
        for (a, b) in [('w', 'W'), ('q', 'Q'), ('w', 'q'), ('W', 'Q')] {
            assert!(is_savarna(a, b), "{a}/{b} should be savarRa (wa-varga)");
        }
        for (a, b) in [('t', 'T'), ('d', 'D'), ('t', 'd'), ('T', 'D')] {
            assert!(is_savarna(a, b), "{a}/{b} should be savarRa (ta-varga)");
        }
        for (a, b) in [('p', 'P'), ('b', 'B'), ('p', 'b'), ('P', 'B')] {
            assert!(is_savarna(a, b), "{a}/{b} should be savarRa (pa-varga)");
        }
        // s/t are NOT savarRa -- 8.4.65's real guard case (hiMstaH never
        // forks).
        assert!(!is_savarna('s', 't'), "s/t should not be savarRa");
        // A vowel or a sibilant against a stop is never savarRa either.
        assert!(!is_savarna('a', 't'), "a/t should not be savarRa");
        assert!(!is_savarna('S', 'c'), "S/c should not be savarRa");
        // Different stop series are not savarRa with each other.
        assert!(!is_savarna('t', 'k'), "t/k should not be savarRa");
    }

    #[test]
    fn is_jhash_covers_exactly_the_voiced_aspirates() {
        for c in ['G', 'J', 'Q', 'D', 'B'] {
            assert!(is_jhash(c), "{c} is a jhaś");
        }
        for c in [
            'g', 'j', 'q', 'd', 'b', 'k', 'c', 'w', 't', 'p', 's', 'z', 'S', 'h', 'a',
        ] {
            assert!(!is_jhash(c), "{c} is not a jhaś");
        }
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
}
