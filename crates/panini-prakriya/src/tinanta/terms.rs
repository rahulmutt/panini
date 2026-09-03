//! Term layout for the tiṅanta pipeline: which index holds what, when that
//! changes, and which term counts as "the follower" for rules that ask.
//!
//! Every rule in this pipeline addresses terms by these constants. The two
//! caveats below are load-bearing — a rule that ignores either produces a
//! non-word or panics, with no test able to name the cause.

use crate::prakriya::Prakriya;
use crate::term::Term;
use crate::tinanta::sound::is_vowel;

/// Index of the aṅga (the dhātu) in `terms`. Stable across the pipeline.
pub(crate) const ANGA: usize = 0;

/// Index of the tiṅ ending *before* śap is inserted (3.1.68).
pub(crate) const ENDING_PRE_SHAP: usize = 1;

/// Index of śap once inserted, and of the ending thereafter.
pub(crate) const SHAP: usize = 1;
pub(crate) const ENDING: usize = 2;

// NOTE: `ENDING_PRE_SHAP` and `SHAP` are deliberately the same value (1), not
// a typo. Rule 3.1.68 (kartari śap) inserts śap between the aṅga and the
// ending, which shifts the ending from index 1 to index 2. This bisects the
// flattened `TINANTA_RULES` sequence (across its six stage files) into two
// halves along that sequence's position, not along any lakāra or
// rule-family boundary:
//   - Rules ordered BEFORE 3.1.68 must address the ending via
//     `ENDING_PRE_SHAP` (index 1, where the ending still lives).
//   - Rules ordered AFTER 3.1.68 must address the ending via `ENDING`
//     (index 2, where it lives once śap has been inserted) and may address
//     śap itself via `SHAP` (also index 1).
// A rule placed on the wrong side of 3.1.68 either mutates śap while
// believing it is mutating the ending, or panics indexing `terms[2]` before
// that slot exists. This matters in particular for new `3.4.x` rules, which
// look like they could go "anywhere in the first block" but must in fact be
// placed relative to 3.1.68, not just relative to other 3.4.x rules.
//
// A further caveat since adādi (gaṇa 2) landed: `terms[SHAP].text` may be
// EMPTY. 2.4.72 (adiprabhṛtibhyaḥ śapaḥ) luks śap by emptying its text while
// keeping the term in place, precisely so these indices stay valid. Any rule
// that reads "the segment after the aṅga" must therefore handle an empty
// string — `ends_with` / `is_empty` / `chars().next()` matched as an Option
// are safe, while `chars().next().unwrap()` (or indexing byte 0) panics.
//
// A SECOND caveat since rudhādi (gaṇa 7) landed: `terms[SHAP].text` is not
// always the vikaraṇa's own text. śnam is an infix (3.1.78 with 1.1.47), and
// the only way to seat an infix in a three-slot layout is to split the root
// across ANGA and SHAP: ANGA keeps the head through its last vowel, SHAP
// holds `na` followed by the root's tail. So for rudhādi, SHAP reads `nat`
// (kft), `nans` (hins) or `nad` (Kid) — never a bare `na`. A rule that reads
// SHAP expecting the vikaraṇa alone must guard on the gaṇa. This is a
// stronger form of a hazard the file already carries: 6.4.107 leaves
// `terms[SHAP].text == "n"` for svādi, which is why `vikarana_u_asamyogapurva` and
// `sound_before_ending` both have ordering constraints written around them.
//
// Two structural consequences of that split, recorded here because nothing
// else states them:
//
//   - For rudhādi, `terms[ANGA]` is VOWEL-FINAL BY CONSTRUCTION. 3.1.78
//     keeps the head through the root's last vowel, so any rule asking
//     "does the aṅga end in a consonant" now answers no for the whole
//     gaṇa. This is why 7.3.84's first application (`guna.rs:87-90`) reads
//     `f` for √kṛt and `i` for √hiṃs and would guṇate them to
//     `*karRatti` / `*henasti` — the ONLY thing stopping it is the ṅit
//     test on that same line, which holds because 1.2.4 unconditionally
//     tags śnam ṅit (śnam is apit). That block is structurally guaranteed
//     by the tag, not by luck, but it depends on this vowel-final fact to
//     even be the relevant guard.
//   - `terms[SHAP]` is consonant-final only because gaṇa 7, as curated
//     here, has no vowel-final root. A vowel-final rudhādi root would
//     leave SHAP as exactly `"na"`, and the five rules that use
//     `SHAP.ends_with('a')` as a proxy for "the vikaraṇa is thematic śap"
//     (`adesha.rs`'s 6.1.101, 6.1.97, 6.1.87, 6.1.66, 6.4.105) would then
//     treat śnam as śap. No such root exists in the dhātupāṭha's gaṇa 7,
//     so this is a caveat for a future slice to re-check, not a live
//     defect.

/// The sārvadhātuka that immediately follows the aṅga — the term **1.1.5
/// *kṅiti ca*** interrogates when it asks whether guṇa is blocked.
///
/// Normally that is the vikaraṇa at `SHAP`. But 2.4.72
/// *adiprabhṛtibhyaḥ śapaḥ* luks śap for adādi by emptying its text while
/// leaving the term in place (so these indices stay valid), and an empty
/// term interposes nothing: the ending is then what immediately follows the
/// aṅga, and the ending is what carries the ṅit tag 1.2.4's first
/// application assigns. Reading a fixed `SHAP` index therefore renders
/// 1.1.5 inoperative for the whole śap-luk'd path.
///
/// Returning the *immediate* follower — rather than testing every later
/// term — is what keeps the thematic path correct: for vRt (vartate) śap is
/// pit (not ṅit), so guṇa rightly proceeds despite the ending being ṅit;
/// for div (dIvyati) śyan IS ṅit and blocks guṇa despite the ending being
/// non-ṅit. Testing the ending instead of the immediate follower would get
/// dIvyati wrong.
///
/// Only meaningful after 3.1.68 has inserted śap. Every caller is ordered
/// after it. Returns `None` when there is no follower at all (a hand-built
/// one-term prakriya in a unit test), in which case nothing can block.
///
/// The name is now one affix too narrow: since 3.1.79, the follower can be
/// the tanādi `u`, which is ārdhadhātuka, not sārvadhātuka. The callers'
/// question — 1.1.5's "is the follower kṅit?" — is saṁjñā-independent, and
/// the u is never ṅit (the second 1.2.4 excludes it by tag), so returning
/// it is correct. The guard this comment used to demand exists as
/// Tag::Ardhadhatuka on the u itself; a future ārdhadhātuka affix that CAN
/// be ṅit is the new restore trigger.
pub(crate) fn following_sarvadhatuka(p: &Prakriya) -> Option<&Term> {
    match p.terms.get(SHAP) {
        Some(shap) if !shap.text.is_empty() => Some(shap),
        Some(_) => p.terms.get(ENDING),
        None => None,
    }
}

/// The sound immediately preceding the ending — the last character of the
/// nearest **non-empty** term before `ENDING`.
///
/// Rules that ask "what does the ending attach to?" cannot read `ANGA`: a
/// non-empty vikaraṇa sits between the two, and it is the vikaraṇa's final
/// sound the ending actually meets. They cannot read `SHAP` either, because
/// 2.4.72 luks śap to an empty string for adādi, where the ending really
/// does attach to the root.
///
/// The fallback is what keeps `adDi` working: with śap empty the search
/// walks past it to the root's `d`. Two other rules open-code the identical
/// walk for the same reason, rather than calling this helper, because each
/// keeps its own independent mutation pin (see `adesha.rs`'s note on why
/// follower lookups are duplicated rather than shared): 8.3.59 in
/// `tripadi.rs` (parameterized on the s-initial affix's own index rather
/// than `ENDING`) and 7.1.5 in `anga.rs` (expression-for-expression
/// identical). Keep this comment's enumeration current if a third copy is
/// ever added — the point is to always know how many there are.
///
/// Returns `None` for a prakriyā with no term before the ending.
pub(crate) fn sound_before_ending(p: &Prakriya) -> Option<char> {
    p.terms
        .get(..ENDING)?
        .iter()
        .rev()
        .find_map(|t| t.text.chars().last())
}

/// Is the vikaraṇa's final `u` *asaṁyogapūrva* — preceded by a single
/// consonant (or directly by a vowel) rather than by a conjunct?
///
/// The condition 6.4.87 inherits by anuvṛtti from 6.4.82 *er anekāco'saṁ-
/// yogapūrvasya*, and the one 6.4.106 states in its own text. Two vikaraṇa
/// shapes carry such a `u`: śnu's `nu` (the `u` preceded by śnu's own `n`,
/// so the question is whether the AṄGA ends in a vowel — hinu yes, Apnu
/// no) and tanādi's bare `u` (3.1.79 — the question is whether the aṅga's
/// final consonant follows a vowel: tanu and fRu yes, arRu no, since the
/// guṇa branch's `rR` is a conjunct; that split is exactly vidyut's
/// arRuhi-beside-fRu).
///
/// Returns false for every other SHAP text (śap/śa `a`, śyan `ya`, śnā's
/// shapes, śnam-plus-tail, adādi's empty string, and the post-6.4.107
/// remnants `n`/``), so callers still need no gaṇa test of their own.
pub(crate) fn vikarana_u_asamyogapurva(p: &Prakriya) -> bool {
    let Some(shap) = p.terms.get(SHAP) else {
        return false;
    };
    let anga: Vec<char> = p.terms[ANGA].text.chars().collect();
    // The two sounds before the vikaraṇa's `u`, nearest first.
    let (c1, c2) = match shap.text.as_str() {
        "nu" => (Some('n'), anga.last().copied()),
        "u" => (
            anga.last().copied(),
            anga.len().checked_sub(2).and_then(|i| anga.get(i).copied()),
        ),
        _ => return false,
    };
    match (c1, c2) {
        // `u` directly after a vowel: trivially not conjunct-preceded. No
        // curated root reaches this arm; it is written because it is what
        // the sūtra says, not because a form needs it.
        (Some(v), _) if is_vowel(v) => true,
        // One consonant after a vowel: hinu, tanu, fRu, kuru.
        (Some(c), Some(v)) if !is_vowel(c) && is_vowel(v) => true,
        // A conjunct (Apnu, aSnu, arRu) or nothing readable.
        _ => false,
    }
}

/// The assembled word as `(term index, char index, char)`, so a rule can
/// reason over the whole pada and still write back into the right term.
///
/// Lived in `tripadi.rs` until slice 7f, when 6.1.73 Ce ca — an aṅga-stage
/// rule with a saṁhitā condition — became the first rule outside the
/// tripādī to need whole-word addressing.
pub(crate) fn word_chars(p: &Prakriya) -> Vec<(usize, usize, char)> {
    let mut out = Vec::new();
    for (ti, t) in p.terms.iter().enumerate() {
        for (ci, c) in t.text.chars().enumerate() {
            out.push((ti, ci, c));
        }
    }
    out
}

/// Replace one character of one term, addressed as `word_chars` reports it.
pub(crate) fn set_char(p: &mut Prakriya, term: usize, idx: usize, to: char) {
    let mut s: Vec<char> = p.terms[term].text.chars().collect();
    s[idx] = to;
    p.terms[term].text = s.into_iter().collect();
}

/// Delete one character of one term, addressed as `word_chars` reports it.
/// Companion to `set_char`, for the rules that elide rather than substitute.
pub(crate) fn remove_char(p: &mut Prakriya, term: usize, idx: usize) {
    let mut s: Vec<char> = p.terms[term].text.chars().collect();
    s.remove(idx);
    p.terms[term].text = s.into_iter().collect();
}

/// Insert one character into a term, before the character `word_chars`
/// reports at `idx`. Companion to `set_char` and `remove_char`, for the
/// rules that augment rather than substitute or elide.
///
/// `idx == term.text.chars().count()` appends, which is exactly what a kit
/// āgama attaching after a term's last character needs (1.1.46 ādyantau
/// ṭakitau).
pub(crate) fn insert_char(p: &mut Prakriya, term: usize, idx: usize, c: char) {
    let mut s: Vec<char> = p.terms[term].text.chars().collect();
    s.insert(idx, c);
    p.terms[term].text = s.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::term::{Tag, Term};

    #[test]
    fn vikarana_u_asamyogapurva_is_true_exactly_for_the_non_conjunct_stems() {
        // The `u` of śnu (or tanādi's bare `u`) is asaṁyogapūrva iff it is
        // not conjunct-preceded. A wrong predicate here turns hinu into
        // *hinuhi and Apnuhi into *Apnu (the false-negative risks are now
        // *tanuhi — a missed luk — and *arRu — a wrong one — beside that
        // svādi pair), so enumerate rather than rely on goldens.
        //
        // BU carries its real bhvādi vikaraṇa text ("a", not "nu"/"u"):
        // unlike the other controls, its point is to pin the helper's OTHER
        // guard — "vikaraṇa is neither śnu nor tanādi's bare u" — which only
        // engages when SHAP's text truly isn't "nu" or "u". BU ends in the
        // vowel U, so pairing it with a literal "nu" SHAP (as every other
        // row does) would make the vowel-final check itself return true,
        // giving a false positive that masks exactly the silent-failure
        // risk this test exists to catch.
        for (root, vikarana, expected) in [
            ("hi", "nu", true),    // svādi, vowel-final
            ("ri", "nu", true),    // svādi, vowel-final
            ("Ap", "nu", false),   // svādi, `pn` conjunct
            ("Sak", "nu", false),  // svādi, `kn` conjunct
            ("aS", "nu", false),   // svādi, `Sn` conjunct — the counter-intuitive one
            ("stiG", "nu", false), // svādi, `Gn` conjunct
            ("kliS", "nu", false), // kryādi control (consonant-final guard)
            ("BU", "a", false),    // bhvādi control (vikaraṇa-is-not-śnu guard)
            ("tan", "u", true),    // tanādi: single n after a vowel
            ("fR", "u", true),     // tanādi: R after the vowel f
            ("kur", "u", true),    // 8b's √kṛ after 6.4.110: r after u
            ("arR", "u", false),   // guṇa'd fR: rR conjunct — arRuhi keeps hi
            ("tan", "nu", false),  // control: an n-final stem under śnu is a conjunct
            ("ti", "u", true),     // synthetic: no curated root is vowel-final
                                   // under tanādi's bare `u`, so this pins the `(Some(v), _) if
                                   // is_vowel(v) => true` arm on its own — every other "u" row
                                   // ends in a consonant and only ever reaches the second arm.
        ] {
            let mut p = Prakriya {
                terms: vec![Term::new(root), Term::new(vikarana), Term::new("anti")],
                ..Default::default()
            };
            p.terms[SHAP].add(Tag::Vikarana);
            assert_eq!(
                vikarana_u_asamyogapurva(&p),
                expected,
                "{root}: asaṁyogapūrva should be {expected}"
            );
        }
    }

    #[test]
    fn shap_holds_shnam_plus_the_roots_tail_for_rudhadi() {
        // The load-bearing consequence of the infix representation: for
        // rudhādi, terms[SHAP].text is NOT the vikaraṇa's own text. Any
        // rule that reads SHAP expecting `na` must guard on the gaṇa.
        let mut p = Prakriya {
            terms: vec![Term::new("hi"), Term::new("nans"), Term::new("ti")],
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        assert_ne!(p.terms[SHAP].text, "na");
        assert!(p.terms[SHAP].text.starts_with("na"));
        assert_eq!(sound_before_ending(&p), Some('s'));
    }
}
