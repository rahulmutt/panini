//! Tripādī: 8.2.77 … 8.4.55.
//!
//! Ordered AFTER 3.1.68, so the ending is at `ENDING` (index 2) and śap at
//! `SHAP` (index 1); `terms[SHAP].text` may be empty (2.4.72). See
//! `super::terms`.

use crate::rule::{Rule, RuleKind};
use crate::tinanta::sound::{cartva_of, is_jhal, is_khar, is_vowel};
use crate::tinanta::terms::{ANGA, SHAP};

pub(crate) static TRIPADI: &[Rule] = &[
    // 8.2.77 hali ca: a root ending in `r`/`v` with a short ik upadhā
    // lengthens that upadhā before a hal (8.2.76 rvorupadhāyā dīrghaḥ is the
    // anuvṛtti source). The only curated root reaching this is div, after
    // guṇa is blocked: div + śyan (y-initial) → dīv → dīvyati. Self-guards on
    // shape; no other curated root fires it (sev has an e-upadhā, vart ends
    // in t).
    Rule {
        id: "8.2.77",
        name: "hali ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let n = chars.len();
            if n < 2 {
                return false;
            }
            let final_c = chars[n - 1];
            let upadha = chars[n - 2];
            if !matches!(final_c, 'r' | 'v') || !matches!(upadha, 'i' | 'u') {
                return false;
            }
            // Reads śap as "the segment following the aṅga"; when śap is luk'd
            // (adādi, 2.4.72) that is empty and the rule silently declines.
            // Currently unreachable (no r/v-final adādi root in scope); when a
            // consonant-final r/v-upadhā adādi root lands, this must generalize
            // to the root+ending junction — 6.1.78's athematic arm (added in
            // slice 5f for √śī, which falls back to `p.terms[ENDING]` when
            // SHAP is empty) is the worked example to follow.
            let Some(next) = p.terms.get(SHAP).and_then(|t| t.text.chars().next()) else {
                return false;
            };
            if is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            let long = if upadha == 'i' { 'I' } else { 'U' };
            let mut s: String = chars[..n - 2].iter().collect();
            s.push(long);
            s.push(final_c);
            p.terms[ANGA].text = s;
            p.record("8.2.77", "hali ca", before);
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
    // 8.2.25 dhi ca: the final `s` of the term preceding a `Dh`-initial affix
    // is ELIDED — not voiced. As + Dve -> A + Dve -> ADve; vas + Dve -> vaDve
    // (this slice's second witness; `vaDve` is the cell the Siddhāntakaumudī's
    // adādi paradigm gives, per vidyut-prakriya's `kaumudi_44::sk_2440`, not
    // the sūtra's own example).
    //
    // Placement is the whole point: 8.2 is asiddha to 8.4, so this fires
    // before any 8.4 junction rule and the `s` never survives to take a jaś
    // substitute. Slice 5d analysed this junction as 8.4.53 jaśtva (s → d)
    // and shipped *AdDve; 8.2.25 bleeds that rule completely, which is why
    // 8.4.53 has no reachable witness and was removed.
    //
    // The guard walks backward from the Dh-initial affix to the nearest
    // non-empty term rather than reading ANGA by index. In today's three-term
    // `[ANGA, SHAP, ENDING]` layout that search always resolves to ANGA
    // whenever the forward arm passes (SHAP is luk'd for adādi), so AsIDvam /
    // vasIDvam below decline at the forward (`D`-initial) arm, not because
    // the guard distinguishes the aṅga from some other neighbour. The search
    // is written generally on purpose, for the multi-term layouts a later
    // slice will bring — mirroring vidyut-prakriya's own `prev_not_empty`.
    Rule {
        id: "8.2.25",
        name: "Di ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // The first non-empty term after the aṅga (śap, if present, is
            // luk'd/empty for adādi) must be the Dh-initial affix.
            let next_idx = p
                .terms
                .iter()
                .enumerate()
                .skip(ANGA + 1)
                .find(|(_, t)| !t.text.is_empty())
                .map(|(i, _)| i);
            let Some(next_idx) = next_idx else {
                return false;
            };
            if !p.terms[next_idx].text.starts_with('D') {
                return false;
            }
            // The nearest non-empty term before it must end in `s`.
            let prev_idx = p.terms[..next_idx]
                .iter()
                .enumerate()
                .rev()
                .find(|(_, t)| !t.text.is_empty())
                .map(|(i, _)| i);
            let Some(prev_idx) = prev_idx else {
                return false;
            };
            if !p.terms[prev_idx].text.ends_with('s') {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[prev_idx].text.chars().collect();
            s.pop();
            p.terms[prev_idx].text = s.into_iter().collect();
            p.record("8.2.25", "Di ca", before);
            true
        },
    },
    // 8.2.66 sasajuṣo ruḥ + 8.3.15 kharavasānayoḥ: word-final `s` → visarga.
    Rule {
        id: "8.3.15",
        name: "KaravasAnayor visarjanIyaH",
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
            p.record("8.3.15", "KaravasAnayor visarjanIyaH", before);
            true
        },
    },
    // 8.3.59 ādeśapratyayayoḥ: the `s` of an ādeśa or a pratyaya, when not
    // word-final, retroflexes to `z` after iṇ-koḥ. The engine's first
    // retroflexion rule, and general grammar rather than a √śī special — √śī
    // is merely the first root to reach it, being the first whose aṅga ends
    // in a vowel other than a/ā right before an s-initial ending:
    // Se + se → Seze (laṭ 2sg), Se + sva → Sezva (loṭ 2sg).
    //
    // NARROW GUARD, by design. The sūtra's trigger is the whole iṇ
    // pratyāhāra (every vowel but a/ā, plus h y v r l) and `k`; this
    // implements only the reachable slice of it — an aṅga-final vowel other
    // than a/ā — so every arm is executed by a test and the mutation gate
    // stays clean. Same discipline that removed 8.4.53 and 6.1.78's E/O arms
    // in slice 5e, and the same shape as 8.2.25's narrow guard. Widen it the
    // moment a root lands whose aṅga ends in h/y/v/r/l or k before an
    // s-initial affix.
    //
    // No conflict with 8.3.15 above: that rule is word-final
    // (kharavasānayoḥ), this one is apadāntasya. It also declines for every
    // existing root without knowing about them — √ās's aṅga ends in `A`
    // (excluded), √vas's in `s` (not a vowel), and every thematic root
    // presents the vikaraṇa's `a` (excluded): Asse, Assva, vasse, vassva and
    // laBase are all unchanged.
    Rule {
        id: "8.3.59",
        name: "AdeSapratyayayoH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // The affix is the first non-empty term after the aṅga (śap, if
            // present, is luk'd/empty for adādi) — the same idiom 8.2.25 and
            // 8.4.55 use, which avoids indexing a term that may not exist.
            let next_idx = p
                .terms
                .iter()
                .enumerate()
                .skip(ANGA + 1)
                .find(|(_, t)| !t.text.is_empty())
                .map(|(i, _)| i);
            let Some(next_idx) = next_idx else {
                return false;
            };
            if !p.terms[next_idx].text.starts_with('s') {
                return false;
            }
            let Some(anga_last) = p.terms[ANGA].text.chars().last() else {
                return false;
            };
            if !is_vowel(anga_last) || matches!(anga_last, 'a' | 'A') {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[next_idx].text.chars().skip(1).collect();
            p.terms[next_idx].text = format!("z{rest}");
            p.record("8.3.59", "AdeSapratyayayoH", before);
            true
        },
    },
    // 8.4.55 khari ca (cartva): a jhal at the aṅga's final position, meeting a
    // khar across the root+ending junction, becomes its car (voiceless
    // unaspirated). √ad's d before ti/tas/si/tha → t: atti, attaH, atsi, atTa.
    // The engine's first internal junction sandhi; general, reused by every
    // later gaṇa/subanta slice. Placed last: latest tripādī rule (8.4 > 8.3).
    Rule {
        id: "8.4.55",
        name: "Kari ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // The following segment is the first char of the first non-empty
            // term after the aṅga (the ending; śap, if present, is luk'd/empty).
            let next = p
                .terms
                .iter()
                .skip(ANGA + 1)
                .find_map(|t| t.text.chars().next());
            let Some(next) = next else { return false };
            if !is_khar(next) {
                return false;
            }
            let Some(last) = p.terms[ANGA].text.chars().last() else {
                return false;
            };
            if !is_jhal(last) {
                return false;
            }
            let Some(sub) = cartva_of(last) else {
                return false;
            };
            if sub == last {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ANGA].text.chars().collect();
            s.pop();
            s.push(sub);
            p.terms[ANGA].text = s.into_iter().collect();
            p.record("8.4.55", "Kari ca", before);
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
    use crate::tinanta::terms::ENDING;
    // `form_g` lives in `mod.rs`'s test module; Task 12 moves it into
    // `derivation_tests.rs`, at which point this import follows it.
    use crate::tinanta::form_g;
    use panini_data::{Lakara, Purusha, Vacana};

    // --- 8.2.77 hali ca: guard-edge pin -----------------------------------
    //
    // Every curated root reaching 8.2.77 (only div) has an aGga of length
    // 3+, so `n < 2` is never observed at the boundary n == 2 by any golden
    // or negative form: the only 2-char roots in the corpus (nI, ji) fail
    // the immediately following `r`/`v` shape check regardless of this
    // guard's outcome, making mutants at this boundary (`<` -> `==`,
    // `<` -> `<=`) behaviorally invisible to the golden 864 and to
    // known_nonforms_are_invalid. Pin the boundary directly with a
    // constructed 2-char aGga that DOES match the rest of the rule's shape
    // (upadhA `i`/`u`, final `r`/`v`, hal-initial vikaraNa) so the two
    // outcomes diverge.
    #[test]
    fn hali_ca_two_char_anga_still_fires() {
        // n=2, "iv": upadhA 'i', final 'v' - matches 8.2.77's shape. The
        // original `n < 2` guard is false (2 < 2 is false), so the rule
        // proceeds and lengthens: "iv" -> "Iv". The `<` -> `==` mutant
        // (n == 2 is true here) and the `<` -> `<=` mutant (2 <= 2 is
        // true) both wrongly take the early-return branch and leave the
        // aGga untouched.
        let mut p = Prakriya {
            terms: vec![Term::new("iv"), Term::new("ta")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "8.2.77").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Iv");
    }

    #[test]
    fn hali_ca_uses_n_minus_2_not_n_over_2() {
        // n=5 ("aBiur"): n-2=3 (upadhA 'u') but n/2=2 (chars[2]='i') --
        // these differ, separating both `-` -> `/` mutants (on the upadhA
        // index and the prefix slice) from the original at once. By hand:
        // final_c=chars[4]='r', upadhA=chars[3]='u' (both match the
        // shape); lengthened upadhA is 'U'; prefix is chars[..3]="aBi";
        // result = "aBi" + "U" + "r" = "aBiUr". Mutating `chars[n - 2]`
        // (upadhA) to `chars[n / 2]` would read upadhA as 'i' instead,
        // giving long 'I' and result "aBiIr". Mutating `chars[..n - 2]`
        // (the prefix) to `chars[..n / 2]` would prefix with "aB"
        // instead of "aBi", giving "aBUr". Both diverge from "aBiUr".
        let mut p = Prakriya {
            terms: vec![Term::new("aBiur"), Term::new("ta")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "8.2.77").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "aBiUr");
    }

    #[test]
    fn shatva_declines_for_every_pre_existing_junction() {
        // Each of these pins one boundary of 8.3.59's guard, and each is a
        // form the suite already ships — so a mutant that widens the guard
        // breaks a golden, not just this test.
        //
        // aṅga-final `A` is excluded (a/ā are not iṇ):
        assert_eq!(
            form_g("As", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
            "Assva"
        );
        // aṅga-final `s` is not a vowel at all:
        assert_eq!(
            form_g("vas", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
            "vasse"
        );
        // Thematic path: the ending is preceded by the śap's `a`, excluded.
        assert_eq!(
            form_g("laB", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
            "laBasva"
        );
        // And a non-s-initial ending after √śī's `e` is left alone — the
        // clause an `||` → `&&` mutant would drop.
        let mut p = Prakriya {
            terms: vec![Term::new("Se"), Term::new(""), Term::new("te")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "8.3.59").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "te");

        // No current root's aṅga ends in a bare short `a` at this point —
        // thematic aṅgas keep the śap's `a` as a separate term, and neither
        // guṇa nor vṛddhi ever yields a bare aṅga-final `a`. This case exists
        // purely to pin the `a` half of the a/ā exclusion: the sūtra's iṇ-koḥ
        // condition excludes both `a` and `ā` (neither is in the iṇ
        // pratyāhāra), so a future `a`-final aṅga must decline here too, not
        // silently retroflex.
        let mut p = Prakriya {
            terms: vec![Term::new("a"), Term::new(""), Term::new("se")],
            log: vec![],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "se");
    }
}
