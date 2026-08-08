//! Tripādī: 8.2.77 … 8.4.55.
//!
//! Ordered AFTER 3.1.68, so the ending is at `ENDING` (index 2) and śap at
//! `SHAP` (index 1); `terms[SHAP].text` may be empty (2.4.72). See
//! `super::terms`.

use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::tinanta::sound::{
    cartva_of, is_jhal, is_khar, is_natva_intervener, is_natva_trigger, is_vowel,
};
use crate::tinanta::terms::{ANGA, SHAP};

/// The assembled word as `(term index, char index, char)`, so a tripādī rule
/// can reason over the whole pada and still write back into the right term.
fn word_chars(p: &Prakriya) -> Vec<(usize, usize, char)> {
    let mut out = Vec::new();
    for (ti, t) in p.terms.iter().enumerate() {
        for (ci, c) in t.text.chars().enumerate() {
            out.push((ti, ci, c));
        }
    }
    out
}

/// Replace one character of one term, addressed as `word_chars` reports it.
fn set_char(p: &mut Prakriya, term: usize, idx: usize, to: char) {
    let mut s: Vec<char> = p.terms[term].text.chars().collect();
    s[idx] = to;
    p.terms[term].text = s.into_iter().collect();
}

/// Shared precondition for 8.4.1 and 8.4.2: the `n` at `i` is a legal target.
///
/// Two sūtras are folded in here as guards rather than modelled as rules,
/// which is this slice's one stated simplification:
///   - **8.4.37 padāntasya**: ṇatva never applies to a word-final n
///     (asmaran, not *asmaraR).
///   - **8.3.24 naś cāpadāntasya jhali**: a non-padānta n before a jhal has
///     ALREADY become an anusvāra by the time the 8.4 rules run, and 8.4.58
///     restores it afterwards — so no such n can be a target (BAzante, not
///     *BAzaRte). This engine has no anusvāra machinery; the condition below
///     is exactly equivalent within tripādī order.
///
/// Retire both in favour of the real rules when liṭ/luṅ bring 8.3.24 in.
fn is_natva_target(w: &[(usize, usize, char)], i: usize) -> bool {
    if w[i].2 != 'n' {
        return false;
    }
    if i + 1 == w.len() {
        return false; // 8.4.37 padAntasya
    }
    !is_jhal(w[i + 1].2) // 8.3.24 has already bled this case
}

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
        vikalpa: false,
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
        vikalpa: false,
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
    // vasIDvam (asserted in `super::derivation_tests`) decline at the forward
    // (`D`-initial) arm, not because the guard distinguishes the aṅga from
    // some other neighbour. The search is written generally on purpose, for
    // the multi-term layouts a later slice will bring — mirroring
    // vidyut-prakriya's own `prev_not_empty`.
    Rule {
        id: "8.2.25",
        name: "Di ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
        apply: |p| {
            // The affix whose s retroflexes: the first s-initial term after
            // the aṅga. Searching for the s-initial term — rather than taking
            // the first non-empty one and testing it — is what lets a
            // non-empty vikaraṇa sit between the aṅga and the affix.
            let next_idx = p
                .terms
                .iter()
                .enumerate()
                .skip(ANGA + 1)
                .find(|(_, t)| t.text.starts_with('s'))
                .map(|(i, _)| i);
            let Some(next_idx) = next_idx else {
                return false;
            };
            // The iṇ-koḥ trigger is the sound IMMEDIATELY before that affix —
            // the last char of the nearest non-empty preceding term, which is
            // the aṅga only when nothing intervenes. For kryādi it is śnā's
            // `ī` (vf + nI + sva → vfRIzva); reading ANGA here would ask
            // about `f` and miss the rule entirely.
            let Some(prev) = p.terms[..next_idx]
                .iter()
                .rev()
                .find_map(|t| t.text.chars().last())
            else {
                return false;
            };
            if !is_vowel(prev) || matches!(prev, 'a' | 'A') {
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
        vikalpa: false,
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
    // 8.4.1 raṣābhyāṁ no ṇaḥ samānapade: `n` → `ṇ` when `r`/`ṣ` DIRECTLY
    // precedes it within the same pada. muz + nAti → muzRAti; vf + nIte →
    // vfRIte (the r-vowel triggers it by 1.1.51 uraṇ raparaḥ).
    //
    // The engine's first ṇatva. Kept disjoint from 8.4.2 — adjacency here,
    // intervention there — so a trace names the sūtra that actually applied.
    Rule {
        id: "8.4.1",
        name: "razAByAM no RaH samAnapade",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let w = word_chars(p);
            for i in 0..w.len() {
                if !is_natva_target(&w, i) || i == 0 {
                    continue;
                }
                if !is_natva_trigger(w[i - 1].2) {
                    continue;
                }
                let before = p.snapshot();
                set_char(p, w[i].0, w[i].1, 'R');
                p.record("8.4.1", "razAByAM no RaH samAnapade", before);
                return true;
            }
            false
        },
    },
    // 8.4.2 aṭkupvāṅnumvyavāye'pi: 8.4.1 applies even when aṭ, ku or pu
    // intervene. vrI + nAti → vrIRAti (the aṭ vowel `I`); muz + Ana → muzARa
    // (the aṭ vowel `A`).
    //
    // The backward scan takes the NEAREST trigger, and must test for a
    // trigger BEFORE testing for an intervener: `r` and the r-vowels are in
    // both sets, so a greedy intervener scan would walk straight past the `r`
    // of `vrI` and find nothing.
    //
    // `j == i` means nothing intervened — that is 8.4.1's case, and this rule
    // declines so the trace credits the right sūtra.
    Rule {
        id: "8.4.2",
        name: "awkupvANnumvyavAye'pi",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let w = word_chars(p);
            for i in 0..w.len() {
                if !is_natva_target(&w, i) {
                    continue;
                }
                let mut j = i;
                let fired = loop {
                    if j == 0 {
                        break false;
                    }
                    let c = w[j - 1].2;
                    if is_natva_trigger(c) {
                        break j < i;
                    }
                    if !is_natva_intervener(c) {
                        break false;
                    }
                    j -= 1;
                };
                if !fired {
                    continue;
                }
                let before = p.snapshot();
                set_char(p, w[i].0, w[i].1, 'R');
                p.record("8.4.2", "awkupvANnumvyavAye'pi", before);
                return true;
            }
            false
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
    // `form_g` lives in `derivation_tests.rs`; `mod.rs` re-exports it, so
    // this import stays on the stable `crate::tinanta::form_g` path.
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

    #[test]
    fn shatva_reads_the_sound_before_the_affix_not_the_anga() {
        // vf + nI + sva: the iN trigger is SnA's I, not the anga's f. The
        // pre-kryadi guard read ANGA and would have declined here.
        let mut p = Prakriya {
            terms: vec![Term::new("vf"), Term::new("nI"), Term::new("sva")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "8.3.59").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "vfnIzva");
        // And the thematic case still declines on the vikaraNa's `a`, which
        // is what keeps laBasva intact.
        let mut p = Prakriya {
            terms: vec![Term::new("laB"), Term::new("a"), Term::new("sva")],
            log: vec![],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "laBasva");
    }

    #[test]
    fn shatva_affix_search_skips_the_anga_itself() {
        // Pins that the s-initial affix search starts AFTER the aGga
        // (`.skip(ANGA + 1)`), not AT it (`.skip(ANGA * 1)` == `.skip(0)`,
        // since ANGA == 0). The corpus alone can't catch a `+` -> `*`
        // mutant here: its only s-initial roots (smf, sev) both decline
        // 8.3.59 on other grounds, so both versions of the search agree on
        // every golden and every known-nonform. An s-initial aGga is
        // needed to force the two versions apart.
        //
        // sI + nI + sva: with skip(1), the search starts past the aGga and
        // finds `sva` at index 2; the preceding non-empty term's last char
        // is SnA's `I` (a non-a/A vowel), so 8.3.59 fires: sInIzva.
        let mut p = Prakriya {
            terms: vec![Term::new("sI"), Term::new("nI"), Term::new("sva")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "8.3.59").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "sInIzva");
        // With the `ANGA * 1` mutant, the search would instead match the
        // aGga `sI` itself at index 0 (it too starts with `s`), leaving no
        // preceding term to read a trigger sound from, so the rule would
        // wrongly decline and `sva` would surface unchanged.
    }

    fn natva_prakriya(anga: &str, vikarana: &str, ending: &str) -> Prakriya {
        Prakriya {
            terms: vec![Term::new(anga), Term::new(vikarana), Term::new(ending)],
            log: vec![],
            ..Default::default()
        }
    }

    #[test]
    fn natva_fires_adjacent_under_8_4_1() {
        // muz + nA + ti: z directly precedes the n.
        let mut p = natva_prakriya("muz", "nA", "ti");
        let rule = rules().find(|r| r.id == "8.4.1").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "muzRAti");
        // vf + nI + te: the r-vowel triggers it (1.1.51).
        let mut p = natva_prakriya("vf", "nI", "te");
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "vfRIte");
        // 8.4.2 must decline on the same adjacent input: `j == i` (nothing
        // intervened) is `break j < i` = `break false`, and the two rules
        // must stay disjoint so a trace credits 8.4.1, not 8.4.2, here. A
        // mutant turning that `break j < i` into `break true` would make
        // 8.4.2 fire wherever 8.4.1 does, and nothing else in this file
        // would catch it.
        let mut p = natva_prakriya("muz", "nA", "ti");
        let r842 = rules().find(|r| r.id == "8.4.2").unwrap();
        assert!(!(r842.apply)(&mut p), "8.4.2 must not fire on adjacency");
    }

    #[test]
    fn natva_fires_across_intervention_under_8_4_2() {
        // vrI + nA + ti: r, then the aw vowel I, then n. 8.4.1 must DECLINE
        // here (not adjacent) and 8.4.2 must fire.
        let mut p = natva_prakriya("vrI", "nA", "ti");
        let r841 = rules().find(|r| r.id == "8.4.1").unwrap();
        assert!(!(r841.apply)(&mut p), "8.4.1 must not fire non-adjacently");
        let r842 = rules().find(|r| r.id == "8.4.2").unwrap();
        assert!((r842.apply)(&mut p));
        assert_eq!(p.text(), "vrIRAti");
        // muz + Ana (the SAnac form): z, the aw vowel A, then n.
        let mut p = natva_prakriya("muz", "Ana", "");
        assert!((r842.apply)(&mut p));
        assert_eq!(p.text(), "muzARa");
    }

    #[test]
    fn natva_declines_word_finally_per_8_4_37() {
        // asmaran: r, the aw vowel a, then a WORD-FINAL n. 8.4.37 padAntasya
        // forbids Natva there. This is an existing golden -- a mutant that
        // drops this guard breaks the 1080, not just this test.
        assert_eq!(
            form_g("smf", Lakara::Lan, Purusha::Prathama, Vacana::Bahu),
            "asmaran"
        );
        let mut p = natva_prakriya("a", "smar", "an");
        for id in ["8.4.1", "8.4.2"] {
            let rule = rules().find(|r| r.id == id).unwrap();
            assert!(!(rule.apply)(&mut p), "{id} fired word-finally");
        }
        assert_eq!(p.text(), "asmaran");
    }

    #[test]
    fn natva_declines_before_a_jhal_because_8_3_24_bleeds_it() {
        // BAzante: z, the aw vowel a, then n -- but the n is followed by the
        // jhal `t`. In the full grammar 8.3.24 naS cApadAntasya jhali has
        // already made that n an anusvAra by the time 8.4.1 runs, and 8.4.58
        // restores it afterwards. This engine has no anusvAra machinery, so
        // the bleeding is encoded as this guard. Another existing golden.
        assert_eq!(
            form_g("BAz", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "BAzante"
        );
        let mut p = natva_prakriya("BAz", "a", "nte");
        for id in ["8.4.1", "8.4.2"] {
            let rule = rules().find(|r| r.id == id).unwrap();
            assert!(!(rule.apply)(&mut p), "{id} fired before a jhal");
        }
        assert_eq!(p.text(), "BAzante");
    }

    #[test]
    fn natva_declines_when_a_non_intervener_breaks_the_run() {
        // varS + A + ni: v a r S A n i. The n is followed by i (not jhal), so
        // it IS a target and the backward scan actually runs -- unlike a
        // pre-jhal case, where is_natva_target declines before the scan ever
        // starts. The scan walks the aw vowel A, then hits S: not a trigger
        // (z, not S) and not an intervener, so it breaks. varS is not a
        // curated root; this case is constructed to exercise that break.
        //
        // a + varta + nta: avartanta IS an existing golden (see
        // paradigm.rs), but t is a jhal immediately after n, so this case is
        // decided by is_natva_target's jhal guard (8.3.24) before the scan
        // ever runs -- it does not exercise the intervener break above.
        for (anga, vikarana, ending) in [("varS", "A", "ni"), ("a", "varta", "nta")] {
            let mut p = natva_prakriya(anga, vikarana, ending);
            let before = p.text();
            for id in ["8.4.1", "8.4.2"] {
                let rule = rules().find(|r| r.id == id).unwrap();
                assert!(!(rule.apply)(&mut p), "{id} fired on {before}");
            }
            assert_eq!(p.text(), before);
        }
    }
}
