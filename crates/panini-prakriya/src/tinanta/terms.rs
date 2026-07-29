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
/// This helper never checks `Tag::Sarvadhatuka` by name — it is safe only
/// because every tiṅ ending in scope is tagged Sarvadhatuka when introduced
/// (see `anga.rs`'s 7.4.21 comment). It must become a real guard the moment
/// an ārdhadhātuka affix enters scope; 7.3.84's own ṅit-only narrowness
/// carries the same restore trigger locally.
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
/// walks past it to the root's `d`. 8.3.59 in `tripadi.rs` open-codes the
/// same walk for the same reason.
///
/// Returns `None` for a prakriyā with no term before the ending.
pub(crate) fn sound_before_ending(p: &Prakriya) -> Option<char> {
    p.terms
        .get(..ENDING)?
        .iter()
        .rev()
        .find_map(|t| t.text.chars().last())
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
