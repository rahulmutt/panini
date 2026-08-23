//! Ādeśa and sandhi: 6.1.101 … 6.4.101.
//!
//! Ordered AFTER 3.1.68 — ending at `ENDING`, śap at `SHAP`, and
//! `terms[SHAP].text` may be empty (2.4.72). See `super::terms`.
//!
//! Three rules here (6.1.90 āṭaś ca, 6.1.66 lopo vyor vali, and 6.1.78 over
//! in `super::guna`) carry explicit *athematic arms*, and each now sits at a
//! different point after slice 9b (kryādi) and the svādi slice's Task 9
//! widened two of them:
//!   - 6.1.66 guards its athematic arm on `!SHAP.has(Tag::Thematic)`, which
//!     covers BOTH athematic paths — adādi's śap-luk'd empty SHAP and
//!     kryādi's śnā, reduced by 6.4.112/6.4.113 to a non-empty, non-`a`-final
//!     `n`/`nI`. It was widened from `SHAP.is_empty()` in slice 9b, which
//!     silently declined for kryādi's non-empty SHAP and produced *vfRIyta
//!     instead of vfRIta; slice 7b's Task 9 widened it again, from
//!     `!SHAP.ends_with('a')` to the tag test, once a hypothetical
//!     vowel-final rudhādi root (śnam's infix can itself leave SHAP
//!     `a`-final, e.g. `"na"`) showed the text test conflated SHAP's shape
//!     with the vikaraṇa's actual identity; see its own comment.
//!   - 6.1.90's athematic arm (below) guards on "SHAP ends in neither `a`
//!     nor `A`", widened from `is_empty()` in the svādi slice's Task 9 to
//!     admit svādi's `nav` (that slice's Task 5), which is non-empty but
//!     also neither `a`- nor `A`-final. Kryādi's `A`-final `nA` is still
//!     excluded by the same guard — 6.1.101's
//!     kryādi arm has already elided the ending's redundant leading vowel
//!     against SHAP's pre-existing `A`, routing the result through 6.1.90's
//!     *thematic* arm instead (nA + AE → nA + E → nE → vfRE). Kryādi's
//!     REDUCED `n`/`nI` are never excluded by this guard directly (they are
//!     neither `a`- nor `A`-final either) but are unreachable here for a
//!     pipeline reason, not a guard reason: 6.4.112/6.4.113 both require the
//!     ending to be Ngit, and 1.2.4 excludes the only endings that ever carry
//!     an āṭ (loṭ uttama, 3.4.92) from Ngit — so an ending shaped `A ec`
//!     never coexists with a reduced `n`/`nI` SHAP. See the athematic arm's
//!     own comment below for the mechanics, and `vikarana.rs`'s 3.1.81
//!     comment for the general correction this widening implements.
//!   - 6.1.78's athematic arm (`super::guna`) is UNCHANGED by that Task 9 and
//!     still guards on `SHAP.is_empty()`, staying adādi-only correctly:
//!     kryādi never guṇates its aṅga (the ṅit śnā blocks 7.3.84/7.3.86 via
//!     1.1.5), so an `e`/`o`-final aṅga — that arm's whole precondition — is
//!     unreachable for it regardless of SHAP shape. Svādi reaches 6.1.78 by
//!     an entirely separate, THIRD arm added in the svādi slice (its Task 5): a
//!     vikaraṇa arm that reads an `e`/`o`-final SHAP directly (`no` → `nav`),
//!     never touching the athematic arm's `is_empty()` guard at all.
//!
//! Those arms duplicate a follower lookup on purpose: each is pinned by its
//! own `*_athematic_*` guard tests asserting disjointness from its thematic
//! arm, and funnelling them through one shared helper would collapse three
//! independent mutation pins into one.

use crate::rule::{Rule, RuleKind};
use crate::term::Tag;
use crate::tinanta::sound::{is_jhal, is_vowel, vrddhi_of};
use crate::tinanta::terms::{ANGA, ENDING, SHAP, shnu_asamyogapurva, sound_before_ending};
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
        vikalpa: false,
        apply: |p| {
            // vidhiliṅ 1sg: after 7.2.79 the ending is `yAam` (yāsuṭ ā + the
            // uttama-eka `am`). 7.2.80 (`super::anga`) would have rewritten
            // `yA`->`iy` after a thematic śap, but it requires SHAP to end
            // in short `a` — a TEXT test, unchanged by 7b Task 9 — and here it
            // declined (adādi's SHAP is empty, kryādi's is the śnā
            // vikaraṇa's `A`-final nA/nI), so the yāsuṭ ā and the ending a
            // are savarṇa -> a single ā: yAam -> yAm. This guard's own
            // negation, `!SHAP.has(Tag::Thematic)`, asks the IDENTITY
            // question instead (see `Tag::Thematic`'s comment) — the two
            // agree for every root this arm actually reaches (śānac never
            // coexists with vidhiliṅ: it requires the `hi` ending, which is
            // loṭ-only), but they are not the same test and must not be
            // assumed interchangeable elsewhere. Plus a tight ending shape
            // (never `yAt`/`yAs`/... whose yA is followed by a consonant).
            if p.terms.len() > ENDING
                && matches!(p.ctx.lakara, Lakara::VidhiLin)
                && !p.terms[SHAP].has(Tag::Thematic)
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
            if !p.terms[SHAP].has(Tag::Thematic) || !p.terms[ENDING].text.starts_with('A') {
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
    // elided (a single substitution in the ekaḥ pūrvaparayoḥ section). Two
    // arms, one sūtra:
    // - Ending arm: the a/ā sits INSIDE the ending. Fires for adādi vidhiliṅ
    //   3pl — after 7.2.79 strips yāsuṭ's s the ending is `yAus`, and the ā
    //   before `us` drops -> `yus` -> yA + yuH. Inert for the thematic
    //   gaṇas: 7.2.80 has already rewritten their liṅ 3pl ending to `iyus`,
    //   whose segment before `us` is `y`, not a/ā.
    // - Junction arm: the ending is a bare `us`, so the a/ā to elide is the
    //   aṅga's final sound. ayA + us -> ay + us -> ayuH. Reachable ONLY via
    //   3.4.111, which is its sole witness today: 3.4.108 jher jus is
    //   vidhiliṅ-only, and by the time this rule runs in vidhiliṅ the yāsuṭ
    //   of 3.4.103 has already made the ending `yAus` (or `yus`, if the
    //   ending arm fired). Every other cell reaches here with an ending that
    //   is not `us` at all.
    Rule {
        id: "6.1.96",
        name: "usyapadAntAt",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let e = &p.terms[ENDING].text;
            if !e.ends_with("us") {
                return false;
            }
            let n = e.chars().count();
            // the char immediately before the final `us` (None if the ending
            // is just "us", which wrapping_sub keeps panic-free)
            let pre = e.chars().nth(n.wrapping_sub(3));
            if matches!(pre, Some('a') | Some('A')) {
                let before = p.snapshot();
                let kept: String = e.chars().take(n - 3).collect();
                p.terms[ENDING].text = format!("{kept}us");
                p.record("6.1.96", "usyapadAntAt", before);
                return true;
            }
            if pre.is_some() {
                return false;
            }
            // Junction arm: nothing precedes `us` inside the ending, so look
            // to the nearest non-empty term before it.
            let Some(prev) = p.terms[..ENDING].iter().rposition(|t| !t.text.is_empty()) else {
                return false;
            };
            // `prev`'s text is non-empty by the `rposition` predicate above,
            // so `.last()` is always `Some`; a second `let-else` here would
            // be dead code no input could ever reach.
            let last = p.terms[prev].text.chars().last().unwrap();
            if !matches!(last, 'a' | 'A') {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[prev].text.chars().collect();
            s.pop();
            p.terms[prev].text = s.into_iter().collect();
            p.record("6.1.96", "usyapadAntAt", before);
            true
        },
    },
    // 6.1.90 āṭaś ca: āṭ + a following vowel yield a single vṛddhi. Two
    // shapes, one sūtra:
    // - Aṅga arm (laṅ, the ātmanepada slice's Task 8): 6.4.72's āṭ + the
    //   root's initial vowel. AeD → ED, AIkz → Ekz.
    // - Ending arm (loṭ uttama eka, ātmanepada): after 6.1.101 has coalesced
    //   śap a + āṭ A into śap A, that A + the ending's E merge to E
    //   (laB+A+E → laBE). MUST follow 6.1.101 — before it the shape is
    //   a + AE and this arm cannot see it.
    Rule {
        id: "6.1.90",
        name: "AwaS ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
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
            // Athematic ending arm (śap luk'd, e.g. adādi √ās loṭ uttama-eka,
            // or svādi's `nav`, that slice's Task 5): with no thematic
            // coalescence having consumed the āṭ A into SHAP, the āṭ A still
            // leads the ending as `A ec` (ENDING == "AE"). Coalesce the two
            // into the single vṛddhi — A + E → E — dropping the A and
            // vṛddhi-ing the ec: As + AE → AsE. Mirrors the thematic arm's
            // mechanics with the vowel sitting at the front of ENDING instead
            // of in SHAP.
            //
            // Widened from `is_empty()` for svādi. The arm's job is "the
            // coalescence rules never consumed the āṭ A into SHAP, so it
            // still leads the ending" — and emptiness was only ever a proxy
            // for that. adādi's empty śap qualifies, as before; so does
            // svādi's `nav`, which fails every arm of 6.1.101 (its `v` is
            // neither savarṇa with A nor an `a`/`A` for the bhvādi and
            // kryādi arms) and so really does leave the A stranded.
            //
            // `a`- and `A`-final SHAPs are excluded because for them 6.1.101
            // HAS already acted: bhvādi's śap became `A` and kryādi's śnā
            // already swallowed the ending's leading A, so both are the
            // thematic arm's business and reaching here would double-count.
            // This is the correction vikarana.rs's 3.1.81 comment predicts
            // in general terms — is_empty() as a stand-in for "the thematic
            // path didn't apply" silently declines for a non-empty,
            // non-`a`-final vikaraṇa.
            if p.terms.len() > ENDING
                && !p.terms[SHAP].text.ends_with('a')
                && !p.terms[SHAP].text.ends_with('A')
            {
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
        vikalpa: false,
        apply: |p| {
            let Some(first) = p.terms[ENDING].text.chars().next() else {
                return false;
            };
            if !p.terms[SHAP].has(Tag::Thematic) || !matches!(first, 'a' | 'e' | 'o') {
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
    //
    // TWO ARMS since rudhādi 7e. The junction arm (below, and the original)
    // coalesces śap's `a` with the ending's initial `i`/`I` and eats that
    // initial. The im arm coalesces an `a i` that sits wholly inside SHAP,
    // put there by 7.3.92, and eats nothing. Both are ād guṇaḥ; they differ
    // in what the `i` belongs to.
    Rule {
        id: "6.1.87",
        name: "Ad guRaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            // ARM 2, the 7.3.92 im (rudhādi 7e). The āgama put an `i`
            // inside SHAP immediately after śnam's `a` — tfnah → tfnaih —
            // and guṇa coalesces that `a i` into `e`, still inside SHAP,
            // consuming nothing from the ending. That is what makes it a
            // separate arm rather than a widening of the junction arm
            // below: the two operations differ in what they consume, not
            // just in where they look.
            //
            // Gated on 7.3.92 having FIRED IN THIS DERIVATION rather than
            // on sniffing SHAP for an `ai`: the āgama IS the condition, not
            // a proxy for it, and the gate makes the arm structurally
            // unable to fire for a root that does not take it. Same idiom
            // 6.4.72 and 7.1.6 use to read the log for a prior rule.
            if p.log.iter().any(|s| s.sutra == "7.3.92") {
                let chars: Vec<char> = p.terms[SHAP].text.chars().collect();
                let Some(pos) = chars.windows(2).position(|w| w == ['a', 'i']) else {
                    return false;
                };
                let before = p.snapshot();
                let mut s = chars;
                // EQUIVALENT MUTANT, documented on purpose (rudhādi 7e
                // mutation campaign, `adesha.rs:380:30: replace + with *`,
                // i.e. `s.remove(pos + 1)` -> `s.remove(pos)`): removing
                // either half of the adjacent `a i` pair and then
                // overwriting index `pos` with `e` produces the same `s`
                // either way, because whichever character survives the
                // removal shifts into (or already sits at) `pos` and is
                // immediately clobbered by the `'e'` assignment below. This
                // holds for any input reaching this arm, not just the
                // cells this repo's suite happens to cover — do not add a
                // test to try to kill it; add one only if the surviving
                // mutant is `remove` targeting a DIFFERENT index than `pos`
                // or `pos + 1`.
                s.remove(pos + 1);
                s[pos] = 'e';
                p.terms[SHAP].text = s.into_iter().collect();
                p.record("6.1.87", "Ad guRaH", before);
                return true;
            }
            let first = p.terms[ENDING].text.chars().next();
            if !p.terms[SHAP].has(Tag::Thematic) || !matches!(first, Some('i') | Some('I')) {
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
        vikalpa: false,
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
            // Athematic arm (SHAP not thematic: empty for adādi's śap-luk'd
            // path, or kryādi's śnā-vikaraṇa reduced to `n`/`nI` by
            // 6.4.112/6.4.113): 6.1.87 only fires on a thematic SHAP, so
            // whenever it isn't, the retained optative I still leads the
            // ending as `I y val` (Iyta). The y is still elided before the
            // val — the long I survives as the stem vowel (āsī-, vfRI-):
            // Iyta → Ita. Only the y is dropped, and (as in the thematic
            // arm) never before a vowel, so IyAtAm / IyATAm / Iya keep their
            // y. The guard reads `Tag::Thematic` (rather than testing
            // emptiness, which only covered adādi and silently declined for
            // kryādi's non-empty, non-`a`-final SHAP — vfRIta surfaced as
            // *vfRIyta until this was widened to a text test, and later a
            // tag test: rudhādi's śnam-infixed SHAP can itself be `a`-final
            // — `"na"` for a vowel-final root — without being thematic, a
            // shape no curated root in this suite happens to produce, since
            // every one leaves a consonant tail after śnam's `na`).
            if !p.terms[SHAP].has(Tag::Thematic)
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
    // 6.4.105 ato heḥ: `hi` is elided after a short `a`. Bav + a + hi →
    // Bava. The sūtra's condition is a short `a` at SHAP, full stop — it is
    // NOT specific to śap. With the guard reading `Tag::Thematic`, all four
    // thematic vikaraṇas reach it, and this suite witnesses all four: śap
    // (Bav + a + hi → Bava), śyan (dIvya), śa (tuda) and śānac
    // (kliS + Ana + hi → kliSAna, per 3.1.83's own comment).
    //
    // Deliberately reads `Tag::Thematic` rather than `sound_before_ending`:
    // the sūtra's own condition is about the VIKARAṆA's `a` specifically
    // (SHAP), not "whatever sound precedes the ending" in general — that
    // distinction is what keeps 6.4.101 her dhiḥ (below) as the separate
    // rule for a jhal-final ANGA. Within that, `Tag::Thematic` reads "is
    // SHAP one of the four a-final vikaraṇas" rather than re-testing SHAP's
    // current text, for the same reason the other four rules in this file
    // do (see `Tag::Thematic`'s own comment). This is what makes 6.4.105
    // decline outright for svādi — the stem there ends in śnu's `u`, never
    // a short `a`, and śnu is not one of the four — leaving 6.4.106 below
    // as the rule that must handle (or deliberately not handle) the `hi`
    // ending for that gaṇa.
    Rule {
        id: "6.4.105",
        name: "ato heH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[SHAP].has(Tag::Thematic) || p.terms[ENDING].text != "hi" {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = String::new();
            p.record("6.4.105", "ato heH", before);
            true
        },
    },
    // 6.4.106 utaś ca pratyayād asaṁyogapūrvāt: `hi` is luk'd after an
    // affix-final `u` that is not conjunct-preceded. hi + nu + hi → hinu;
    // ri + nu + hi → riRu (ṇatva lands later). Ap + nu + hi keeps its `hi`
    // → Apnuhi, and that pair is the rule's pin.
    //
    // Continues the luk of 6.4.105 ato heḥ immediately above, which is why
    // it sits here rather than in sūtra-number order elsewhere. 6.4.105
    // declines for svādi on its own guard (the stem ends in `u`, not a
    // short `a`), so the two never contend.
    //
    // Must precede 6.4.101 her DhiH below: for the conjunct roots this rule
    // deliberately leaves `hi` standing, and 6.4.101 is what must then also
    // decline — see its own comment on reading the sound before the ending.
    Rule {
        id: "6.4.106",
        name: "utaSca pratyayAdasaMyogapUrvAt",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if p.terms[ENDING].text != "hi" {
                return false;
            }
            if !shnu_asamyogapurva(p) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = String::new();
            p.record("6.4.106", "utaSca pratyayAdasaMyogapUrvAt", before);
            true
        },
    },
    // 6.4.107 lopaś cāsyānyatarasyāṁ mvoḥ: the same `u` 6.4.106 spoke of —
    // affix-final, asaṁyogapūrva — is OPTIONALLY elided before `m` and `v`.
    // hi + nu + mas → hinmaH ~ hinumaH, both valid. This is the engine's
    // first vikalpa rule: `run_pipeline` forks here, and both readings are
    // reported by `Panini::check`.
    //
    // 6.4.108 nityaṁ karoteḥ is what makes this one optional — it states
    // the same lopa as *nitya* for √kṛ, against this rule's anyatarasyām.
    // √kṛ is out of scope (it wants 7.1.100 and the 6.4.10x kṛ-specials),
    // so 6.4.108 is not implemented.
    //
    // Continues the 6.4.105 / 6.4.106 luk-and-lopa run above, which is also
    // where sūtra order puts it. It cannot contend with 6.4.101 below,
    // whose guard requires the ending to be `hi` — neither m- nor
    // v-initial.
    //
    // ORDERING, load-bearing and invisible: this rule must stay after
    // EVERY consumer of `shnu_asamyogapurva`. Its mutation leaves
    // `SHAP.text == "n"`, so the helper's first guard (`== "nu"`) makes it
    // return false for the rest of the pipeline — on the forked branch
    // only. A consumer placed below this rule would read the wrong answer
    // for half a paradigm, with both halves individually plausible. Every
    // rule that reads śnu's `nu` text must precede it — 6.4.87 and 6.4.106
    // (just above) via `shnu_asamyogapurva`, and 6.4.77 in guna.rs, which
    // open-codes the same `text == "nu"` test — and all three do.
    Rule {
        id: "6.4.107",
        name: "lopaScAsyAnyatarasyAM mvoH",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            if !p.terms[ENDING].text.starts_with(['m', 'v']) {
                return false;
            }
            if !shnu_asamyogapurva(p) {
                return false;
            }
            let before = p.snapshot();
            // lopa OF THE `u` — not a rewrite of śnu's text to "n".
            p.terms[SHAP].text.pop();
            p.record("6.4.107", "lopaScAsyAnyatarasyAM mvoH", before);
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
        vikalpa: false,
        apply: |p| {
            if p.terms[ENDING].text != "hi" {
                return false;
            }
            // NOT terms[ANGA]. The jhal this sūtra tests is the sound the
            // ending attaches to, which for a gaṇa with a live vikaraṇa is
            // the vikaraṇa's final, not the root's. Reading ANGA fired on
            // √āp's `p` and √śak's `k` and gave *ApnuDi / *SaknuDi, even
            // though śnu's `u` sits between. adādi still reaches the root
            // because its śap is empty and the helper walks past it.
            let Some(last) = sound_before_ending(p) else {
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
    // 6.4.111 śnasor allopaḥ: śnam's `a` is elided before a kṅit
    // sārvadhātuka. This is what produces rudhādi's weak stem, and the
    // strong/weak split the gaṇa is built around: kfnat + ti (strong,
    // tip is pit) against kfnt + anti (weak, Ji is ṅit by 1.2.4).
    //
    // The `a` deleted is always SHAP's second character — śnam's own — never
    // a vowel of the root, because 3.1.78 put everything of the root that
    // follows its last vowel behind śnam. That is the whole payoff of the
    // representation: this is a term-local edit rather than a positional
    // search inside a merged string.
    //
    // PLACEMENT, pinned by `hinDi`: 6.4.101 her dhiH runs FIRST and rewrites
    // the ending hi → Di, and only then does this rule strip the `a`
    // (hinas + Di → hins + Di → 8.2.25 → hinDi). Ordered last in this stage
    // for that reason; the sūtra number is not what decides it.
    //
    // The sūtra's `sa` — the `a` of √as — is out of scope: √as is not in the
    // root set. Guarded to the rudhādi arm accordingly, per the narrow-guard
    // discipline that landed 8.3.59 and 8.2.25.
    Rule {
        id: "6.4.111",
        name: "SnasorallopaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Rudhadi) {
                return false;
            }
            // 1.1.5 kṅiti ca: the sārvadhātuka that immediately follows must
            // be ṅit. For rudhādi SHAP is never empty, so the follower is
            // always the ending.
            let Some(ending) = p.terms.get(ENDING) else {
                return false;
            };
            if !ending.has(Tag::Ngit) {
                return false;
            }
            let shap: Vec<char> = p.terms[SHAP].text.chars().collect();
            if shap.get(1) != Some(&'a') {
                return false;
            }
            let before = p.snapshot();
            let mut s = shap;
            s.remove(1);
            p.terms[SHAP].text = s.into_iter().collect();
            p.record("6.4.111", "SnasorallopaH", before);
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
    use crate::tinanta::derivation_tests::sole;
    use crate::tinanta::derive;
    use crate::tinanta::rules;
    use panini_data::{Pada, Purusha, Vacana, dhatus};

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
        // must fire ONLY when SHAP is not thematic — that is exactly the
        // condition under which 6.1.87 (which requires a thematic SHAP)
        // could NOT already have consumed the I. Here the śap is the
        // thematic "a" (tagged, as 3.1.68 would tag it) and the ending is
        // "Iyta": the athematic arm must decline (leaving "Iyta"
        // untouched), and the thematic arm also declines (the ending's
        // first char is 'I', not 'y'). The mutant that drops the
        // `!has(Tag::Thematic)` guard would elide the y regardless of śap
        // and wrongly yield "Ita".
        let mut shap = Term::new("a");
        shap.add(Tag::Thematic);
        let mut p = Prakriya {
            terms: vec![Term::new("laB"), shap, Term::new("Iyta")],
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
    fn awas_ca_athematic_arm_declines_for_an_a_final_shap() {
        // The svādi slice's Task 9 widened the athematic arm's guard from
        // `SHAP.is_empty()` to "SHAP ends in neither `a` nor `A`", so it is
        // no longer the empty śap that gates this arm — an `a`-final śap
        // (the ordinary thematic śap `a`, not yet widened to `A` by 6.1.101)
        // must ALSO decline, and for the same reason as an `A`-final one:
        // 6.1.97 (ato guṇe) is that shape's business, not this arm's. Here
        // the śap is the non-empty "a" and the ending is "AE" (A + ec): the
        // thematic arm declines (its own guard is SHAP.ends_with('A'), and
        // "a" does not), and the athematic arm must ALSO decline because "a"
        // ends in `a`, leaving "AE" untouched. The `&&` -> `||` mutant on the
        // length check (`len() > ENDING`) makes the guard always true
        // regardless of the two `ends_with` conjuncts short-circuiting it, so
        // the mutant fires and wrongly coalesces "AE" -> "E".
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
    fn atash_ca_athematic_arm_fires_for_a_svadi_stem() {
        // aS + nav + AE → aS + nav + E → aSnavE (loṭ ātmanepada uttama eka).
        let mut p = Prakriya {
            terms: vec![Term::new("aS"), Term::new("nav"), Term::new("AE")],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.90").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "E");
        assert_eq!(p.terms[SHAP].text, "nav");
    }

    #[test]
    fn atash_ca_athematic_arm_still_fires_for_adadi() {
        // As + "" + AE → AsE. The arm's original job; must not regress.
        let mut p = Prakriya {
            terms: vec![Term::new("As"), Term::new(""), Term::new("AE")],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.90").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "E");
    }

    #[test]
    fn atash_ca_athematic_arm_stays_off_a_and_capital_a_final_shap() {
        // The ending is fixed at "AE", whose first char is 'A' — never one
        // of the thematic arm's `e|E|o|O`, so the thematic arm can never
        // fire here regardless of SHAP. That means for every SHAP in this
        // list, NO arm may fire: bhvādi (`A`) and kryādi (`nA`) are the
        // thematic arm's business in a different ending shape, not this
        // one, and 6.1.101 has already acted for both before this arm would
        // ever see them. The assertion is unconditional, not gated on
        // `fired`: gating on it would let a mutant that drops
        // `!ends_with('a')` or `!ends_with('A')` fire the athematic arm,
        // coalesce "AE" -> "E", and skip the very assertion meant to catch
        // it.
        //
        // Deliberately NOT extended to kryādi's reduced `n`/`nI`
        // (6.4.112/6.4.113): those are excluded from ever meeting an "AE"
        // ending by a PIPELINE guarantee (3.4.92 adds āṭ only to loṭ
        // uttama; 1.2.4 excludes loṭ uttama from Ngit; 6.4.112/6.4.113 both
        // require Ngit) documented in the module doc above, not by this
        // rule's own guard. Verified directly: `"n".ends_with('a')` and
        // `"n".ends_with('A')` are both false (same for `"nI"`), so a
        // hand-built Prakriya pairing SHAP "n"/"nI" with ENDING "AE" WOULD
        // make this arm fire — that combination just never arises through
        // the real pipeline. Adding those two rows here would pin an
        // artificial state this rule was never asked to reject, and would
        // fail on unmodified code (not just under a guard-deletion mutant).
        for shap in ["a", "A", "ya", "yA", "nA", "Ana"] {
            let mut p = Prakriya {
                terms: vec![Term::new("laB"), Term::new(shap), Term::new("AE")],
                ..Default::default()
            };
            let rule = rules().find(|r| r.id == "6.1.90").unwrap();
            let fired = (rule.apply)(&mut p);
            assert!(!fired, "SHAP {shap:?}: no arm may fire on an AE ending");
            assert_eq!(p.terms[ENDING].text, "AE", "SHAP {shap:?}");
        }
    }

    #[test]
    fn atash_ca_declines_when_the_ending_is_not_a_plus_ec() {
        // ApnavAni: `Ani` is A + n, not A + ec, so nothing coalesces.
        let mut p = Prakriya {
            terms: vec![Term::new("Ap"), Term::new("nav"), Term::new("Ani")],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.90").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "Ani");
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

    /// 6.1.96 has two arms. The original elides an a/ā that sits INSIDE the
    /// ending, before its final `us` (the yāsuṭ case, yAus -> yus). The
    /// junction arm elides the aṅga's final a/ā when the ending is a bare
    /// `us`, reachable only via 3.4.111, which is its sole witness today.
    #[test]
    fn usyapadantat_has_an_ending_arm_and_a_junction_arm() {
        let rule = rules().find(|r| r.id == "6.1.96").unwrap();

        // junction arm: ayA + us -> ay + us
        let mut p = Prakriya {
            terms: vec![Term::new("ayA"), Term::new(""), Term::new("us")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "ayus");

        // ending arm, unchanged: the a/ā is inside the ending
        let mut p = Prakriya {
            terms: vec![Term::new("yA"), Term::new(""), Term::new("yAus")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "yus");

        // junction arm declines when the aṅga is not a/ā-final
        let mut p = Prakriya {
            terms: vec![Term::new("yAy"), Term::new(""), Term::new("us")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
    }

    #[test]
    fn usyapadantat_junction_arm_declines_when_nothing_precedes_us() {
        // No non-empty term stands before a bare `us` ending. This state
        // never arises from any real derivation (the aṅga always carries
        // text), but the junction arm's own `rposition` guard must still
        // decline rather than panic on it — pinned directly so that guard
        // has a witness under mutation testing.
        let mut p = Prakriya {
            terms: vec![Term::new(""), Term::new(""), Term::new("us")],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.96").unwrap();
        assert!(!(rule.apply)(&mut p));
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

    #[test]
    fn her_dhih_reads_the_sound_before_the_ending_not_the_root() {
        // Ap + nu + hi must stay Apnuhi. `p` is a jhal, but it is not what
        // precedes `hi` — śnu's `u` is, and `u` is not a jhal.
        let mut p = Prakriya {
            terms: vec![Term::new("Ap"), Term::new("nu"), Term::new("hi")],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.4.101").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "hi");
    }

    #[test]
    fn her_dhih_still_fires_for_adadi_across_an_empty_shap() {
        // √ad: śap is luk'd, so the nearest non-empty term before the ending is
        // the root itself and `d` is still the right character. adDi.
        let mut p = Prakriya {
            terms: vec![Term::new("ad"), Term::new(""), Term::new("hi")],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.4.101").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "Di");
    }

    #[test]
    fn her_dhih_declines_for_kryadi_shni() {
        // vrI + nI + hi → vrIRIhi. `I` is not a jhal. Unchanged by this task,
        // pinned so the change is provably a no-op here too.
        let mut p = Prakriya {
            terms: vec![Term::new("vrI"), Term::new("nI"), Term::new("hi")],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.4.101").unwrap();
        assert!(!(rule.apply)(&mut p));
    }

    #[test]
    fn utash_ca_luks_hi_after_a_non_conjunct_u() {
        // hi + nu + hi → hinu.
        let mut p = Prakriya {
            terms: vec![Term::new("hi"), Term::new("nu"), Term::new("hi")],
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        let rule = rules().find(|r| r.id == "6.4.106").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "");
    }

    #[test]
    fn utash_ca_declines_after_a_conjunct_u() {
        // Ap + nu + hi → Apnuhi. The asaṁyogapūrva clause is the whole rule.
        let mut p = Prakriya {
            terms: vec![Term::new("Ap"), Term::new("nu"), Term::new("hi")],
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        let rule = rules().find(|r| r.id == "6.4.106").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "hi");
    }

    #[test]
    fn utash_ca_declines_when_the_ending_is_not_hi() {
        let mut p = Prakriya {
            terms: vec![Term::new("hi"), Term::new("nu"), Term::new("ti")],
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        let rule = rules().find(|r| r.id == "6.4.106").unwrap();
        assert!(!(rule.apply)(&mut p));
    }

    /// Build a post-3.1.68 svādi prakriyā: root, śnu, ending.
    fn shnu_p(root: &str, ending: &str) -> Prakriya {
        let mut p = Prakriya {
            terms: vec![Term::new(root), Term::new("nu"), Term::new(ending)],
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        p
    }

    fn rule_6_4_107() -> &'static Rule {
        rules()
            .find(|r| r.id == "6.4.107")
            .expect("6.4.107 present")
    }

    #[test]
    fn lopa_of_shnu_u_fires_only_before_m_and_v() {
        // The sūtra's *mvoḥ*. laṭ uttama dvi/bahu (`vas`/`mas`) and laṅ
        // uttama dvi/bahu (`va`/`ma`) are the only endings in scope that
        // qualify — vidhiliṅ's surface as `yAva`/`yAma` (y-initial) and
        // loṭ's āṭ-augmented as `Ava`/`Ama` (A-initial).
        for (ending, expected) in [
            ("vas", true),
            ("mas", true),
            ("va", true),
            ("ma", true),
            ("Tas", false),  // hinuTaH
            ("yAma", false), // hinuyAma
            ("Ama", false),  // loṭ's āṭ-augmented uttama bahu
            ("anti", false), // hinvanti, which is 6.4.87's yaṇ, not this
        ] {
            let mut p = shnu_p("hi", ending);
            assert_eq!(
                (rule_6_4_107().apply)(&mut p),
                expected,
                "ending {ending}: 6.4.107 should fire = {expected}"
            );
        }
    }

    #[test]
    fn lopa_of_shnu_u_needs_asamyogapurva() {
        // *Asya* is 6.4.106's `u`, asaṁyogapūrva by anuvṛtti. Only √hi and
        // √ri qualify; the other four svādi roots put a conjunct before
        // śnu's `u`. Both ātmanepadī svādi roots are among those four,
        // which is why the ātmanepada column never forks.
        for (root, expected) in [
            ("hi", true),
            ("ri", true),
            ("Ap", false),
            ("Sak", false),
            ("aS", false),
            ("stiG", false),
        ] {
            let mut p = shnu_p(root, "mas");
            assert_eq!(
                (rule_6_4_107().apply)(&mut p),
                expected,
                "root {root}: 6.4.107 should fire = {expected}"
            );
        }
    }

    #[test]
    fn lopa_leaves_shnu_as_n_and_records_the_step() {
        // Pins the observable effects of 6.4.107's lopa: the resulting
        // `SHAP.text`, the full surface, the sūtra id, and that `before` is
        // the pre-mutation snapshot. It does NOT (and cannot) distinguish
        // *lopa* of the `u` from a substitution of `n` for `nu` — the two
        // are indistinguishable through `SHAP.text`, `p.text()`, and the
        // log alike; that distinction is intent, not something this test
        // asserts.
        let mut p = shnu_p("hi", "mas");
        assert!((rule_6_4_107().apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "n");
        assert_eq!(p.text(), "hinmas");
        assert_eq!(p.log.last().unwrap().sutra, "6.4.107");
        assert_eq!(p.log.last().unwrap().before, "hinumas");
    }

    #[test]
    fn lopa_of_shnu_u_is_optional() {
        assert!(rule_6_4_107().vikalpa, "6.4.107 is anyatarasyām");
    }

    #[test]
    fn shnasor_allopah_fires_only_before_a_knit_sarvadhatuka() {
        // Strong cell (tip is pit, not ṅit) keeps the `a`; weak cell (Ji is
        // apit → ṅit by 1.2.4) loses it. A guard that ignored ṅitva would
        // derive *kfnttanti and *kfRatvaH, both plausible-looking.
        let d = dhatus().iter().find(|d| d.dhatupatha == "07.0010").unwrap();
        let strong = sole(derive(
            d,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        ));
        assert!(!strong.log.iter().any(|s| s.sutra == "6.4.111"));
        let weak = sole(derive(
            d,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Bahu,
        ));
        let step = weak.log.iter().find(|s| s.sutra == "6.4.111").unwrap();
        assert_eq!(step.before, "kfnatanti");
        assert_eq!(step.after, "kfntanti");
    }
}
