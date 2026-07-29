//! Ādeśa and sandhi: 6.1.101 … 6.4.101.
//!
//! Ordered AFTER 3.1.68 — ending at `ENDING`, śap at `SHAP`, and
//! `terms[SHAP].text` may be empty (2.4.72). See `super::terms`.
//!
//! Three rules here (6.1.90 āṭaś ca, 6.1.66 lopo vyor vali, and 6.1.78 over
//! in `super::anga`) carry explicit *athematic arms*, but they split on how
//! kryādi's śnā vikaraṇa is covered:
//!   - 6.1.66 guards its athematic arm on `!SHAP.ends_with('a')`, which
//!     covers BOTH athematic paths — adādi's śap-luk'd empty SHAP and
//!     kryādi's śnā, reduced by 6.4.112/6.4.113 to a non-empty, non-`a`-final
//!     `n`/`nI`. It was widened from `SHAP.is_empty()` in slice 9b, which
//!     silently declined for kryādi's non-empty SHAP and produced *vfRIyta
//!     instead of vfRIta; see its own comment.
//!   - 6.1.90 and 6.1.78 still guard their athematic arms on
//!     `SHAP.is_empty()`, and stay adādi-only — correctly, not by oversight.
//!     Kryādi's `A`-final SHAP never reaches either arm: 6.1.101's kryādi
//!     arm elides the ending's redundant leading vowel against SHAP's
//!     pre-existing `A`, routing the result through 6.1.90's *thematic* arm
//!     (nA + AE → nA + E → nE → vfRE), and kryādi never guṇates
//!     (`following_sarvadhatuka` returns the non-empty ṅit śnā, so 1.1.5
//!     blocks 7.3.84/7.3.86), so 6.1.78's `e`/`o`-final-aṅga precondition
//!     is unreachable for it.
//!
//! Those arms duplicate a follower lookup on purpose: each is pinned by its
//! own `*_athematic_*` guard tests asserting disjointness from its thematic
//! arm, and funnelling them through one shared helper would collapse three
//! independent mutation pins into one.

use crate::rule::{Rule, RuleKind};
use crate::tinanta::sound::{is_jhal, is_vowel, vrddhi_of};
use crate::tinanta::terms::{ANGA, ENDING, SHAP};
use panini_data::Lakara;

pub(crate) static ADESHA: &[Rule] = &[
    // 6.1.101 akaḥ savarṇe dīrghaḥ: an ak vowel followed by a savarṇa vowel
    // coalesces into the corresponding long vowel. Four arms:
    //   - vidhiliṅ 1sg, śap-final-`a` declined (7.2.80 skipped): the yāsuṭ ā
    //     + the ending a coalesce inside the ending, yAam → yAm (→ yAyAm for
    //     adādi's śap-luk'd path, → nIyAm for kryādi's śnā-vikaraṇa path);
    //   - adādi (śap luk'd by 2.4.72): the aṅga's own final `A` meets an
    //     a/ā-initial ending, yA + anti → yAnti, yA + Ani → yAni;
    //   - kryādi (śnā vikaraṇa, 3.1.81): the vikaraṇa's own final `A` meets
    //     an a/ā-initial ending directly — mip's 3.4.101 `am` (Pit, so
    //     6.4.112/6.4.113 never touch it) or the loṭ uttama āḍ-augmented
    //     Ani/Ava/Ama (excluded from 1.2.4's ṅit tagging, so same untouched
    //     path): nA + am → nAm, nA + Ani → nAni;
    //   - bhvādi &c.: śap `a` + the ending's initial `A` (from 3.4.92 āḍ),
    //     Bav + a + Ani → BavAni.
    Rule {
        id: "6.1.101",
        name: "akaH savarRe dIrGaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // vidhiliṅ 1sg: after 7.2.79 the ending is `yAam` (yāsuṭ ā + the
            // uttama-eka `am`). 7.2.80 would have rewritten `yA`->`iy` after
            // an a-final śap, but it requires SHAP to end in short `a`; here
            // it declined (adādi's SHAP is empty, kryādi's is the śnā
            // vikaraṇa's `A`-final nA/nI), so the yāsuṭ ā and the ending a
            // are savarṇa -> a single ā: yAam -> yAm. Guard mirrors 7.2.80's
            // own negated condition, plus a tight ending shape (never
            // `yAt`/`yAs`/... whose yA is followed by a consonant).
            if p.terms.len() > ENDING
                && matches!(p.ctx.lakara, Lakara::VidhiLin)
                && !p.terms[SHAP].text.ends_with('a')
                && p.terms[ENDING].text.starts_with("yA")
                && matches!(p.terms[ENDING].text.chars().nth(2), Some('a') | Some('A'))
            {
                let before = p.snapshot();
                // drop the ending's third char (the a/A after `yA`)
                let kept: String = p.terms[ENDING]
                    .text
                    .chars()
                    .enumerate()
                    .filter(|&(i, _)| i != 2)
                    .map(|(_, c)| c)
                    .collect();
                p.terms[ENDING].text = kept;
                p.record("6.1.101", "akaH savarRe dIrGaH", before);
                return true;
            }
            // adādi (śap luk'd by 2.4.72): the aṅga's own final ā meets an
            // a/ā-initial ending directly (no vikaraṇa buffer). ā + a/ā are
            // savarṇa → a single long ā. Keep the aṅga's ā, drop the ending's
            // initial vowel: yA + anti → yAnti, yA + Ani (āṭ) → yAni.
            if p.terms.len() > ENDING
                && p.terms[SHAP].text.is_empty()
                && p.terms[ANGA].text.ends_with('A')
                && matches!(p.terms[ENDING].text.chars().next(), Some('a') | Some('A'))
            {
                let before = p.snapshot();
                p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
                p.record("6.1.101", "akaH savarRe dIrGaH", before);
                return true;
            }
            // kryādi: the vikaraṇa (at SHAP) is śnā's own `A`-final `nA`,
            // meeting an a/ā-initial ending directly — unlike the bhvādi arm
            // below, there is no `a` at SHAP to widen into `A`; SHAP already
            // carries the long vowel, so only the ending's leading vowel is
            // dropped.
            if p.terms.len() > ENDING
                && p.terms[SHAP].text.ends_with('A')
                && matches!(p.terms[ENDING].text.chars().next(), Some('a') | Some('A'))
            {
                let before = p.snapshot();
                p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
                p.record("6.1.101", "akaH savarRe dIrGaH", before);
                return true;
            }
            if !p.terms[SHAP].text.ends_with('a') || !p.terms[ENDING].text.starts_with('A') {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[SHAP].text.chars().collect();
            s.pop();
            s.push('A');
            p.terms[SHAP].text = s.into_iter().collect();
            p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
            p.record("6.1.101", "akaH savarRe dIrGaH", before);
            true
        },
    },
    // 6.1.96 usyapadāntāt: an a/ā immediately before the ending `us` is
    // elided (a single substitution in the ekaḥ pūrvaparayoḥ section). Fires
    // only for adādi vidhiliṅ 3pl: after 7.2.79 strips yāsuṭ's s, the ending
    // is `yAus`, and here the ā before `us` drops -> `yus` -> yA + yuH.
    // Inert for the thematic gaṇas: 7.2.80 has already rewritten their liṅ
    // 3pl ending to `iyus`, whose segment before `us` is `y`, not a/ā.
    Rule {
        id: "6.1.96",
        name: "usyapadAntAt",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let e = &p.terms[ENDING].text;
            if !e.ends_with("us") {
                return false;
            }
            let n = e.chars().count();
            // the char immediately before the final `us` (None if the ending
            // is just "us", which wrapping_sub keeps panic-free)
            let pre = e.chars().nth(n.wrapping_sub(3));
            if !matches!(pre, Some('a') | Some('A')) {
                return false;
            }
            let before = p.snapshot();
            let kept: String = e.chars().take(n - 3).collect();
            p.terms[ENDING].text = format!("{kept}us");
            p.record("6.1.96", "usyapadAntAt", before);
            true
        },
    },
    // 6.1.90 āṭaś ca: āṭ + a following vowel yield a single vṛddhi. Two
    // shapes, one sūtra:
    // - Aṅga arm (laṅ, Task 8): 6.4.72's āṭ + the root's initial vowel.
    //   AeD → ED, AIkz → Ekz.
    // - Ending arm (loṭ uttama eka, ātmanepada): after 6.1.101 has coalesced
    //   śap a + āṭ A into śap A, that A + the ending's E merge to E
    //   (laB+A+E → laBE). MUST follow 6.1.101 — before it the shape is
    //   a + AE and this arm cannot see it.
    Rule {
        id: "6.1.90",
        name: "AwaS ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // Aṅga arm: āṭ prefix on a vowel-initial aṅga.
            let anga: Vec<char> = p.terms[ANGA].text.chars().collect();
            if anga.len() >= 2
                && anga[0] == 'A'
                && is_vowel(anga[1])
                && let Some(v) = vrddhi_of(anga[1])
            {
                let before = p.snapshot();
                let mut s = String::new();
                s.push(v);
                s.extend(&anga[2..]);
                p.terms[ANGA].text = s;
                p.record("6.1.90", "AwaS ca", before);
                return true;
            }
            // Ending arm: śap/śyan A-final (āṭ via 6.1.101) + ending-initial
            // ec. Ends in `A`, not equal to `A`, so śyan's `yA` (after
            // 6.1.101 widened the same way) keeps its `y`.
            if p.terms.len() > ENDING
                && p.terms[SHAP].text.ends_with('A')
                && let Some(first) = p.terms[ENDING].text.chars().next()
                && matches!(first, 'e' | 'E' | 'o' | 'O')
            {
                let before = p.snapshot();
                let mut s: Vec<char> = p.terms[SHAP].text.chars().collect();
                s.pop();
                s.push(vrddhi_of(first).unwrap());
                p.terms[SHAP].text = s.into_iter().collect();
                p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
                p.record("6.1.90", "AwaS ca", before);
                return true;
            }
            // Athematic ending arm (śap luk'd, e.g. adādi √ās loṭ uttama-eka):
            // with no śap, 6.1.101 never widened śap a + āṭ A, so the āṭ A
            // still leads the ending as `A ec` (ENDING == "AE"). Coalesce the
            // two into the single vṛddhi — A + E → E — dropping the A and
            // vṛddhi-ing the ec: As + AE → AsE. Mirrors the thematic arm's
            // mechanics with the vowel sitting at the front of ENDING instead
            // of in SHAP. `is_empty()` (not `!ends_with('a')`) is still the
            // right test here: kryādi's `A`-final SHAP never reaches this
            // arm — 6.1.101's kryādi arm already consumed the āṭ A into SHAP,
            // so kryādi is handled by the thematic arm above instead.
            if p.terms.len() > ENDING && p.terms[SHAP].text.is_empty() {
                let mut it = p.terms[ENDING].text.chars();
                if it.next() == Some('A')
                    && let Some(ec) = it.next()
                    && matches!(ec, 'e' | 'E' | 'o' | 'O')
                {
                    let before = p.snapshot();
                    let mut s = String::new();
                    s.push(vrddhi_of(ec).unwrap());
                    s.extend(p.terms[ENDING].text.chars().skip(2));
                    p.terms[ENDING].text = s;
                    p.record("6.1.90", "AwaS ca", before);
                    return true;
                }
            }
            false
        },
    },
    // 6.1.97 ato guṇe: a short `a` (the śap) followed by a guṇa vowel yields
    // para-rūpa — a single vowel identical to the following one. For the `anti`
    // ending (Ji → anti), śap `a` + initial `a` of `anti` → a single short `a`
    // (NOT savarṇa-dīrgha `A`), so `Bav`+`a`+`nti` = `Bavanti`. Drop the
    // ending's leading `a`; the surviving śap `a` stands in for the coalesced
    // vowel and the term vector stays consistent for `.text()`.
    //
    // Widened beyond the `a+a` case to cover any guṇa vowel (a/e/o) following
    // śap `a`, per the sūtra's own text: `a+a` (anti) and `a+e` (laṭ
    // ātmanepada uttama-eka, laB+a+e → laBe) both arise from the curated
    // roots; no `a+o` case arises, but the guard states the sūtra's full set.
    Rule {
        id: "6.1.97",
        name: "ato guRe",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let Some(first) = p.terms[ENDING].text.chars().next() else {
                return false;
            };
            if !p.terms[SHAP].text.ends_with('a') || !matches!(first, 'a' | 'e' | 'o') {
                return false;
            }
            let before = p.snapshot();
            // Para-rūpa: the single substitute is the FOLLOWING vowel. For
            // a+a the śap already spells it; for a+e (laṭ Ā uttama-eka
            // laB+a+e → laBe) the śap must become that vowel. Only the
            // final vowel is replaced — śyan's `ya` keeps its `y` (so
            // divya+anti → divyanti, not divy+anti).
            let mut s: Vec<char> = p.terms[SHAP].text.chars().collect();
            s.pop();
            s.push(first);
            p.terms[SHAP].text = s.into_iter().collect();
            p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
            p.record("6.1.97", "ato guRe", before);
            true
        },
    },
    // 6.1.87 ād guṇaḥ: śap `a` + ending-initial `i` coalesce to guṇa `e`.
    // Bava + iyt → Bave + yt. Same mechanical shape as 6.1.101 above: the
    // śap stands in for the coalesced vowel, the ending loses its initial.
    // MUST precede 6.1.66: only after the `i` is absorbed does the ending
    // start with the `y` that 6.1.66 tests.
    //
    // Short `iy` comes from 7.2.80/7.2.81; long `Iy` is sīyuṭ after salopa
    // (7.2.79). Both coalesce with śap `a` to guṇa `e`.
    Rule {
        id: "6.1.87",
        name: "Ad guRaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let first = p.terms[ENDING].text.chars().next();
            if !p.terms[SHAP].text.ends_with('a') || !matches!(first, Some('i') | Some('I')) {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[SHAP].text.chars().collect();
            s.pop();
            s.push('e');
            p.terms[SHAP].text = s.into_iter().collect();
            p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
            p.record("6.1.87", "Ad guRaH", before);
            true
        },
    },
    // 6.1.66 lopo vyor vali: v or y is elided before a val consonant. Two
    // arms, both eliding the yāsuṭ/optative y before a val (yt → t, yva → va;
    // never before a vowel, so yus / IyAtAm keep their y):
    //  - thematic arm: 6.1.87 has already absorbed the optative i/I into śap's
    //    guṇa e, so the ending leads with the y directly (Bave + yt → Baveta).
    //  - athematic arm (śap luk'd, adādi √ās): 6.1.87 never fired, so the
    //    retained long I still leads the ending as `I y val`; the y is elided
    //    and the I survives as the stem vowel (Iyta → Ita, āsī-).
    // The val pratyāhāra is every consonant except y, and no `yy` sequence
    // arises in this engine, so "not a vowel" is an exact guard here.
    Rule {
        id: "6.1.66",
        name: "lopo vyor vali",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let mut chars = p.terms[ENDING].text.chars();
            let first = chars.next();
            // Thematic arm: 6.1.87 already consumed the optative I into śap's
            // guṇa e, so the ending leads with the yāsuṭ y directly before a
            // val consonant (yt → t, yva → va; yus survives — u is a vowel).
            if first == Some('y') {
                let Some(second) = chars.next() else {
                    return false;
                };
                if is_vowel(second) {
                    return false;
                }
                let before = p.snapshot();
                p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
                p.record("6.1.66", "lopo vyor vali", before);
                return true;
            }
            // Athematic arm (SHAP not `a`-final: empty for adādi's śap-luk'd
            // path, or kryādi's śnā-vikaraṇa reduced to `n`/`nI` by
            // 6.4.112/6.4.113): 6.1.87 only fires when SHAP ends in short
            // `a`, so whenever it doesn't, the retained optative I still
            // leads the ending as `I y val` (Iyta). The y is still elided
            // before the val — the long I survives as the stem vowel
            // (āsī-, vfRI-): Iyta → Ita. Only the y is dropped, and (as in
            // the thematic arm) never before a vowel, so IyAtAm / IyATAm /
            // Iya keep their y. The guard mirrors 6.1.101's own
            // `!ends_with('a')` idiom (rather than testing emptiness, which
            // only covered adādi and silently declined for kryādi's
            // non-empty, non-`a`-final SHAP — vfRIta surfaced as *vfRIyta
            // until this was widened).
            if !p.terms[SHAP].text.ends_with('a')
                && first == Some('I')
                && chars.next() == Some('y')
                && let Some(third) = chars.next()
                && !is_vowel(third)
            {
                let before = p.snapshot();
                let mut s = String::new();
                s.push('I');
                s.extend(p.terms[ENDING].text.chars().skip(2));
                p.terms[ENDING].text = s;
                p.record("6.1.66", "lopo vyor vali", before);
                return true;
            }
            false
        },
    },
    // 6.4.105 ato heḥ: `hi` is elided after a short `a` (the śap).
    // Bav + a + hi → Bava.
    Rule {
        id: "6.4.105",
        name: "ato heH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[SHAP].text.ends_with('a') || p.terms[ENDING].text != "hi" {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = String::new();
            p.record("6.4.105", "ato heH", before);
            true
        },
    },
    // 6.4.101 hujhalbhyo her dhiḥ: the loṭ 2sg `hi` becomes `Di` after a
    // jhal-final aṅga (and after √hu, out of scope). √ad: 6.4.105 ato heḥ
    // declined (its aṅga ends in `d`, not a short `a`), so `hi` survives to
    // here → adDi. Thematic roots never reach this: their `hi` is luk'd by
    // 6.4.105 behind śap's `a`.
    Rule {
        id: "6.4.101",
        name: "her DiH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms[ENDING].text != "hi" {
                return false;
            }
            let Some(last) = p.terms[ANGA].text.chars().last() else {
                return false;
            };
            if !is_jhal(last) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = "Di".into();
            p.record("6.4.101", "her DiH", before);
            true
        },
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::prakriya::Prakriya;
    use crate::term::{Tag, Term};
    use crate::tinanta::rules;
    use panini_data::{Pada, Purusha, Vacana};

    #[test]
    fn vali_lopa_spares_a_following_vowel() {
        // BaveyuH keeps its y because `u` is not a val consonant; Baveva
        // loses it because `v` is. Pin the guard at the rule level.
        for (ending, fires, want) in [("yva", true, "va"), ("yus", false, "yus")] {
            let mut p = Prakriya {
                terms: vec![Term::new("Bav"), Term::new("e"), Term::new(ending)],
                log: vec![],
                ctx: Context::new(
                    Lakara::VidhiLin,
                    Pada::Parasmaipada,
                    Purusha::Uttama,
                    Vacana::Dvi,
                ),
                blocked: false,
            };
            let rule = rules().find(|r| r.id == "6.1.66").unwrap();
            assert_eq!((rule.apply)(&mut p), fires, "{ending}");
            assert_eq!(p.terms[ENDING].text, want, "{ending}");
        }
    }

    #[test]
    fn lopo_vyor_vali_athematic_arm_requires_a_non_a_final_shap() {
        // 6.1.66's athematic arm elides the optative y in an `I y val`
        // ending (Iyta -> Ita), keeping the long I as the stem vowel. It
        // must fire ONLY when SHAP does NOT end in short `a` — that is
        // exactly the condition under which 6.1.87 (which requires an
        // `a`-final SHAP) could NOT already have consumed the I. Here the
        // śap is the thematic "a" and the ending is "Iyta": the athematic
        // arm must decline (leaving "Iyta" untouched), and the thematic arm
        // also declines (the ending's first char is 'I', not 'y'). The
        // mutant that drops the `!ends_with('a')` guard would elide the y
        // regardless of śap and wrongly yield "Ita".
        let mut p = Prakriya {
            terms: vec![Term::new("laB"), Term::new("a"), Term::new("Iyta")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.66").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "Iyta");
    }

    #[test]
    fn lopo_vyor_vali_athematic_arm_fires_for_kryadis_non_empty_shap() {
        // Unlike adādi, kryādi's SHAP is never empty (it holds the śnā
        // vikaraṇa, reduced to `n`/`nI` by 6.4.112/6.4.113) but also never
        // `a`-final, so the athematic arm must still fire: vf + n + Iyta ->
        // vf + n + Ita (the "vfRIta" golden's mechanism, before 8.4.1
        // natva). A guard that tested emptiness instead of `!ends_with('a')`
        // would wrongly decline here and leave the y in place.
        let mut p = Prakriya {
            terms: vec![Term::new("vf"), Term::new("n"), Term::new("Iyta")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.66").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "Ita");
    }

    #[test]
    fn awas_ca_ending_arm_requires_a_third_term() {
        // 6.1.90's ending arm reads p.terms[SHAP] and p.terms[ENDING]
        // (index 2) once its guard passes. With only two terms (aGga +
        // SHAP, no ending inserted yet), `p.terms.len() > ENDING` (2 > 2)
        // is false, so the guard short-circuits before ever indexing
        // terms[2]. The `>` -> `>=` mutant makes `2 >= 2` true, so the
        // mutant guard proceeds to check terms[SHAP].text == "A" (true
        // here) and then indexes terms[ENDING], which is out of bounds
        // for a 2-term vector and panics. The aGga itself ("kf") also
        // must not satisfy the aGga arm (it doesn't start with 'A'), so
        // this isolates the ending-arm guard alone.
        let mut p = Prakriya {
            terms: vec![Term::new("kf"), Term::new("A")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.90").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kf");
        assert_eq!(p.terms[SHAP].text, "A");
    }

    #[test]
    fn awas_ca_athematic_arm_requires_a_third_term() {
        // 6.1.90's ATHEMATIC ending arm (śap luk'd) reads p.terms[ENDING]
        // (index 2) once its guard passes. With only two terms (aGga + an
        // empty śap, no ending inserted yet), `p.terms.len() > ENDING`
        // (2 > 2) is false, so the guard short-circuits before indexing
        // terms[2]. The `>` -> `>=` mutant makes `2 >= 2` true; since the
        // śap here is empty, the mutant guard proceeds and indexes
        // terms[ENDING], out of bounds for a 2-term vector -> panics. The
        // aGga ("As") does not satisfy the aGga arm (its 2nd char 's' is
        // not a vowel), isolating the athematic ending-arm guard.
        let mut p = Prakriya {
            terms: vec![Term::new("As"), Term::new("")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.90").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "As");
    }

    #[test]
    fn awas_ca_athematic_arm_requires_an_empty_shap() {
        // The athematic arm must fire ONLY when the śap is luk'd (empty) —
        // that is what distinguishes the adADi (athematic) path from the
        // thematic one, whose A-widened śap is handled by the thematic arm
        // above. Here the śap is the non-empty "a" and the ending is "AE"
        // (A + ec): the thematic arm declines (its guard is
        // SHAP.ends_with('A'), and "a" does not), and the athematic arm
        // must ALSO decline because the śap is not empty, leaving "AE"
        // untouched. The `&&` -> `||` mutant drops the empty-śap
        // requirement (len() > ENDING is always true), so the mutant fires
        // and wrongly coalesces "AE" -> "E".
        let mut p = Prakriya {
            terms: vec![Term::new("laB"), Term::new("a"), Term::new("AE")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.90").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "AE");
    }

    #[test]
    fn usyapadantat_drops_a_before_us_and_spares_iyus() {
        // Fires: after 7.2.79 the adādi liṅ 3pl ending is `yAus`; the ā
        // before `us` drops -> `yus`.
        let mut p = Prakriya {
            terms: vec![Term::new("yA"), Term::new(""), Term::new("yAus")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Bahu,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "6.1.96").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "yus");

        // Declines: the thematic liṅ 3pl ending is `iyus` (7.2.80 rewrote yA
        // -> iy); the char before `us` is `y`, not a/ā, so nothing changes.
        let mut q = Prakriya {
            terms: vec![Term::new("Bav"), Term::new("a"), Term::new("iyus")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Bahu,
            ),
            blocked: false,
        };
        assert!(!(rule.apply)(&mut q));
        assert_eq!(q.terms[ENDING].text, "iyus");
    }

    // --- 6.1.96 usyapadAntAt: `n - 3` not `n / 3` boundary pin -------------
    //
    // The only real firing ending is `yAus`/`vAus` (n=4), where n-3=1
    // equals n/3=1 (integer division) -- the two expressions are
    // indistinguishable at that length, which is why the existing
    // `usyapadantat_drops_a_before_us_and_spares_iyus` test alone doesn't
    // kill the `-` -> `/` mutant on `e.chars().take(n - 3)`. A synthetic
    // 5-char ending "yAaus" (y,A,a,u,s) still satisfies the guard
    // (ends_with "us"; char at n-3=index 2 is 'a') but separates the two
    // arithmetic expressions: original take(n-3)=take(2)="yA" -> "yAus";
    // mutant take(n/3)=take(1)="y" -> "yus".
    #[test]
    fn usyapadantat_uses_n_minus_3_not_n_over_3() {
        let mut p = Prakriya {
            terms: vec![Term::new("yA"), Term::new(""), Term::new("yAaus")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Bahu,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "6.1.96").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "yAus");
    }

    // --- 6.1.101 adAdi arm: `len() > ENDING` boundary pin ------------------
    //
    // The adAdi arm's own guard is `len() > ENDING && SHAP.is_empty() &&
    // ANGA.ends_with('A') && matches!(ENDING.chars().next(), ...)`. Build a
    // 2-term Prakriya (aGga "yA" + an empty, luk'd Sap slot, no ending term
    // at all) so `len() > ENDING` (2 > 2) is false in the original: the
    // if-block short-circuits before ever indexing terms[ENDING], and
    // control falls to the rule's second (pre-adAdi) branch, whose own
    // `!SHAP.text.ends_with('a')` is true for an empty SHAP (`""` does not
    // end with `'a'`) and short-circuits the `||` there too — so the
    // original returns false with no panic, on only 2 terms. The `>` ->
    // `>=` mutant lets the first if-block through at `len() == ENDING`,
    // and its fourth conjunct indexes the nonexistent terms[ENDING],
    // panicking.
    #[test]
    fn akah_savarne_dirghah_adadi_arm_two_term_anga_does_not_panic() {
        let mut anga = Term::new("yA");
        anga.add(Tag::Adadi);
        let mut p = Prakriya {
            terms: vec![anga, Term::new("")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.101").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "yA");
    }

    #[test]
    fn savarna_dirgha_adadi_lin_1sg_arm() {
        let rule = rules().find(|r| r.id == "6.1.101").unwrap();

        // Fires: adādi liṅ 1sg ending `yAam` (śap empty) -> `yAm`.
        let mut p = Prakriya {
            terms: vec![Term::new("yA"), Term::new(""), Term::new("yAam")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Uttama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "yAm");

        // Declines: the `yA` of `yAt` (2sg-shape) is followed by a consonant,
        // not a vowel, so no savarṇa coalescence.
        let mut q = Prakriya {
            terms: vec![Term::new("yA"), Term::new(""), Term::new("yAt")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        assert!(!(rule.apply)(&mut q));
        assert_eq!(q.terms[ENDING].text, "yAt");

        // Declines: thematic liṅ (śap = `a`) is never touched by this arm —
        // the `!ends_with('a')` guard (true for adādi's empty śap and for
        // kryādi's `A`-final vikaraṇa, false for any `a`-final śap) is what
        // scopes it away from every thematic gaṇa, where 7.2.80 has already
        // consumed the `yA` shape anyway.
        let mut r = Prakriya {
            terms: vec![Term::new("Bav"), Term::new("a"), Term::new("iyam")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Uttama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        assert!(!(rule.apply)(&mut r));
        assert_eq!(r.terms[ENDING].text, "iyam");
    }

    // --- 6.1.101 adAdi vidhiliG 1sg arm: `len() > ENDING` boundary pin ----
    //
    // 6.1.101's first arm's guard, above, is `len() > ENDING && lakara ==
    // VidhiLin && !SHAP.ends_with('a') && ENDING.starts_with("yA") && ...`. A
    // 2-term Prakriya (aGga "yA" + an empty Sap slot, no ENDING term at
    // all) makes `len() > ENDING` (2 > 2) false in the original, so the
    // if-block short-circuits before ever indexing terms[ENDING]; control
    // falls through the second (pre-adAdi) and third (kryAdi) arms (both
    // guarded by the same `len() > ENDING`, equally false) to the fourth
    // (thematic) branch, whose `!SHAP.text.ends_with('a')` is true for an
    // empty SHAP and short-circuits the `||` there too -- so the original
    // returns false with no panic. Unlike the existing two-term regression
    // test for the adAdi arm above, this one pins the lakara to VidhiLin:
    // the `>` -> `>=` mutant needs `lakara == VidhiLin` to be true to reach
    // its fourth conjunct, which indexes the nonexistent terms[ENDING]
    // (index 2 on a 2-element Vec) and panics. A default-lakara (Lat)
    // Prakriya would let the mutant's second conjunct short-circuit first
    // and never distinguish it -- this is why the earlier two-term test
    // alone didn't kill this mutant.
    #[test]
    fn savarna_dirgha_adadi_lin_1sg_arm_two_term_prakriya_does_not_panic() {
        let mut p = Prakriya {
            terms: vec![Term::new("yA"), Term::new("")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Uttama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "6.1.101").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "yA");
    }

    #[test]
    fn savarna_dirgha_kryadi_shna_arm() {
        // kryādi's śnā vikaraṇa (SHAP = "nA") ends in `A`, unlike every other
        // gaṇa's vikaraṇa (empty, or `a`-final): it needs its own arm because
        // neither the adādi arm (SHAP empty) nor the bhvādi arm (SHAP ends
        // `a`) covers it.
        let rule = rules().find(|r| r.id == "6.1.101").unwrap();

        // Fires: mip's 3.4.101 `am` ending meets the vikaraṇa's `A` -> `nAm`
        // (kliS laṅ uttama eka, akliSnAm).
        let mut p = Prakriya {
            terms: vec![Term::new("kliS"), Term::new("nA"), Term::new("am")],
            log: vec![],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "m");

        // Fires: the loṭ uttama āḍ-augmented ending `Ani` meets the same
        // vikaraṇa -> `nAni` (kliS loṭ uttama eka, kliSnAni).
        let mut q = Prakriya {
            terms: vec![Term::new("kliS"), Term::new("nA"), Term::new("Ani")],
            log: vec![],
            ..Default::default()
        };
        assert!((rule.apply)(&mut q));
        assert_eq!(q.terms[ENDING].text, "ni");

        // Declines: a consonant-initial ending (e.g. `ti`) is untouched --
        // this arm is only for a/ā-initial endings meeting the vikaraṇa's ā.
        let mut r = Prakriya {
            terms: vec![Term::new("kliS"), Term::new("nA"), Term::new("ti")],
            log: vec![],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut r));
        assert_eq!(r.terms[ENDING].text, "ti");
    }

    // --- 6.1.101 kryAdi arm: `len() > ENDING` boundary pin -----------------
    //
    // The kryādi arm's own guard, above, is `len() > ENDING &&
    // SHAP.ends_with('A') && ...`. A 2-term Prakriya (aṅga + the śnā
    // vikaraṇa at SHAP, no ENDING term at all) makes `len() > ENDING`
    // (2 > 2) false in the original, so the if-block short-circuits before
    // ever indexing terms[ENDING]. Lat (the default context) keeps the
    // vidhiliṅ 1sg arm above out of the way regardless of the length
    // operator (its own guard requires `lakara == VidhiLin`), and the
    // adādi arm above is skipped because SHAP ("nA") is not empty — so
    // control reaches the kryādi arm on its own terms. It then falls
    // through, safely, to the thematic (bhvādi) arm, whose decline check
    // is `!SHAP.ends_with('a') || !ENDING.starts_with('A')`: SHAP = "nA"
    // does not end in lowercase `a`, so the first disjunct is true and
    // short-circuits the `||` before it can index the nonexistent
    // terms[ENDING] either. The whole call is therefore panic-free and
    // returns false in the original. The `>` -> `>=` mutant lets the
    // kryādi arm's if-block through at `len() == ENDING`, and its third
    // conjunct indexes the nonexistent terms[ENDING], panicking.
    #[test]
    fn akah_savarne_dirghah_kryadi_arm_two_term_prakriya_does_not_panic() {
        let mut p = Prakriya {
            terms: vec![Term::new("kliS"), Term::new("nA")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.101").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kliS");
    }
}
