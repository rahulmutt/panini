//! Tripādī: 8.2.77 … 8.4.56.
//!
//! Ordered AFTER 3.1.68, so the ending is at `ENDING` (index 2) and śap at
//! `SHAP` (index 1); `terms[SHAP].text` may be empty (2.4.72). See
//! `super::terms`.

use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::term::Tag;
use crate::tinanta::sound::{
    cartva_of, is_jhal, is_khar, is_natva_intervener, is_natva_trigger, is_savarna, is_vowel,
    jashtva_of, parasavarna_of,
};
use crate::tinanta::terms::{ANGA, ENDING, SHAP};

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

/// Delete one character of one term, addressed as `word_chars` reports it.
/// Companion to `set_char`, for the rules that elide rather than substitute.
fn remove_char(p: &mut Prakriya, term: usize, idx: usize) {
    let mut s: Vec<char> = p.terms[term].text.chars().collect();
    s.remove(idx);
    p.terms[term].text = s.into_iter().collect();
}

/// Whether the dhātu — held across `ANGA`/`SHAP` — still sits at the pada
/// boundary, i.e. nothing with real text occupies `ENDING` or beyond.
///
/// Shared guard for 8.2.73–8.2.75: those rules touch the dhātu's OWN final
/// letter, and may only do so when that letter is actually pada-final. In
/// every laṅ prathama/madhyama eka cell they target, it is — 8.2.23
/// saṁyogāntasya lopaḥ has already eaten tip/sip's own letter, leaving
/// `ENDING` empty and the dhātu's letter as the true word end. Two DIFFERENT
/// shapes of counterexample show why the guard has to be positive (checking
/// `ENDING` is empty) rather than checked in some narrower, single-cell way:
///
/// - 7.1.35's tātaṅ (loṭ madhyama eka) substitutes real material — `tAt` —
///   into `ENDING`, leaving the dhātu word-medial; without this guard the
///   `rposition` search below still finds *some* non-empty final (now
///   `ENDING`'s own, already jaśtva-voiced `d`) and mutates it, spuriously
///   deriving `kfnttAr`/`kfntAr`. Checked against vidyut-prakriya: its loṭ
///   madhyama eka set for kft is exactly six forms — `kfntAt`, `kfnttAt`,
///   `kfntAd`, `kfnttAd`, `kfnDi`, `kfndDi` — never eight.
/// - `Context::is_sip` (8.2.74's guard) is a lakāra-blind slot predicate
///   (parasmaipada madhyama eka, regardless of lakāra), so vidhiliṅ madhyama
///   eka — ending `yAs`, which 8.2.23 leaves untouched because a vowel (`A`)
///   precedes the `s`, not a conjunct — ALSO satisfies `is_sip()`, with
///   `ENDING` genuinely holding `yAs`/`yAd`. `dhatu_is_pada_final` is what
///   keeps 8.2.74 off it there (`ENDING` is non-empty). 8.2.73 has no slot
///   predicate at all — the mutation gate showed one wasn't load-bearing and
///   it was removed — so `dhatu_is_pada_final` is doing the ENTIRE job of
///   keeping it off this cell too: without it, 8.2.73 (obligatory) would
///   rewrite `ENDING`'s own `s` regardless of what ending it belonged to,
///   corrupting the cell's PRIMARY output to `kfntyAd`/`hiMsyAd` instead of
///   leaving it to reduce via 6.1.68 to `kfntyAH`/`hiMsyAH`.
///   `rudhadi_vidhilin_madhyama_eka_is_untouched_by_the_ru_alternation` in
///   `super::derivation_tests` is the witness.
fn dhatu_is_pada_final(p: &Prakriya) -> bool {
    // Defensive rather than a bare `p.terms[ENDING..]`: every call site
    // guards on `Tag::Rudhadi` first, so a hand-built two-term `Prakriya`
    // never actually reaches here today, but this helper is file-scoped and
    // a future caller might not carry that guard. `p.terms.get(ENDING)` at
    // 8.2.25 above is the same defensive idiom for a single index; `None`
    // here (fewer than `ENDING` terms at all) means there is nothing past
    // the dhātu to hold it back, so the dhātu counts as pada-final.
    p.terms
        .get(ENDING..)
        .is_none_or(|rest| rest.iter().all(|t| t.text.is_empty()))
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
            // Read the bearing term as the last NON-EMPTY one, not a fixed
            // `terms.len() - 1`: the same fix 8.3.15 needed below, for the
            // same reason. A fixed last index is only safe while `ENDING`
            // is always the true word end; the moment some rule can leave
            // `ENDING` empty (6.4.105 / 6.4.106 luk it outright) while an
            // earlier term still holds the word-final letters, the fixed
            // index writes onto the empty term instead and leaves the real
            // target untouched. This is exactly the shape that bit 8.3.15's
            // twin during this slice and produced `ahinasH`; nothing here
            // currently exercises it (this guard needs two word-final
            // consonants, and every path that empties `ENDING` before the
            // tripādī also leaves a vowel-final word), so this is a
            // preventive match to 8.2.39/8.3.15's shape, not a live fix.
            let Some(idx) = p.terms.iter().rposition(|t| !t.text.is_empty()) else {
                return false;
            };
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
    // substitute. Slice 5d analysed the ās/vas junction as 8.4.53 jaśtva
    // (s → d) and shipped *AdDve; 8.2.25 bleeds that rule completely for
    // every s-final stem it reaches. rudhādi's kft is the first stem this
    // junction sees whose final consonant is NOT an `s` — 8.2.25 declines
    // there and 8.4.53 (restored below) is what fires instead. See 8.4.53's
    // own comment for that history.
    //
    // The guard reads the Dh-initial affix as `ENDING` directly, the same
    // way 6.4.101 above reads it — no vikaraṇa in this grammar ever begins
    // with `D`, so `ENDING` is the only place one can be. It then walks
    // BACKWARD from `ENDING` to the nearest non-empty term, which must end
    // in `s`; that backward search is written generally (rather than
    // reading ANGA by index) for the multi-term layouts a later slice will
    // bring, mirroring vidyut-prakriya's own `prev_not_empty`. For adādi it
    // resolves to ANGA (SHAP is luk'd empty there); for rudhādi's √hiṃs it
    // resolves to SHAP instead (`ns`, śnam's infix residue) — the case a
    // forward search from the aṅga cannot see, since a forward "first
    // non-empty term after ANGA" search only ever lands on the ending when
    // SHAP itself is empty, and rudhādi's SHAP never is (hins + Di →
    // hinDi, only reachable once the affix is read as `ENDING` directly).
    // AsIDvam / vasIDvam (asserted in `super::derivation_tests`) still
    // decline correctly: their ending is `IDvam`, which does not start
    // with `D`.
    Rule {
        id: "8.2.25",
        name: "Di ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let Some(ending) = p.terms.get(ENDING) else {
                return false;
            };
            if !ending.text.starts_with('D') {
                return false;
            }
            // The nearest non-empty term before the ending must end in `s`.
            let prev_idx = p.terms[..ENDING]
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
    // 8.2.30 coH kuH: a cu sound (c C j J) is replaced by its ku counterpart
    // (the nearest velar by 1.1.50 sthāne'ntaratamaḥ, so voicing and
    // aspiration are preserved) when it is either word-final or immediately
    // followed by a jhal. Banaj + ti -> Banag + ti (before the jhal `t`,
    // then 8.4.55 khari ca devoices to Banakti); aBanaj -> aBanag
    // word-finally.
    //
    // NARROW GUARD, by design, as with 8.2.39 just below: only `j` -> `g`
    // is reachable this slice (rudhādi's one cu-final curated root is
    // √bhañj). Widen the match the moment a `c`-tailed root lands.
    //
    // Read via `word_chars`, not a term-boundary check: the target `j` sits
    // at the END of a non-final term (śnam's infix leaves the root's own
    // tail — the `j` — in `SHAP`, one term short of the actual word end,
    // e.g. `Ba | naj | ti`), so the jhal that conditions it can be the
    // FIRST character of the NEXT term rather than anything in the bearing
    // term itself. `word_chars` already flattens exactly this cross-term
    // adjacency for the same reason 8.3.24 above reads it. Word-final falls
    // out of the same scan for free: `i + 1 == w.len()` after 8.2.23 has
    // eaten tip/sip's own letter, leaving `ENDING` empty and the dhātu's
    // `j` as the last entry `word_chars` reports.
    Rule {
        id: "8.2.30",
        name: "coH kuH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let w = word_chars(p);
            let Some(i) = w.iter().position(|(_, _, c)| *c == 'j') else {
                return false;
            };
            let word_final = i + 1 == w.len();
            if !word_final && !is_jhal(w[i + 1].2) {
                return false;
            }
            let (term, idx, _) = w[i];
            let before = p.snapshot();
            set_char(p, term, idx, 'g');
            p.record("8.2.30", "coH kuH", before);
            true
        },
    },
    // 8.2.39 jhalāṁ jaśo'nte: a pada-final jhal becomes its jaś (voiced
    // unaspirated). This is what makes `aBavad` the engine's DECLINED form —
    // it is obligatory, and 8.4.56 below optionally undoes it. Before this
    // rule existed the pipeline simply never voiced a final, which is why
    // the goldens read `aBavat` and the repo carried a "drop the pausal d"
    // convention.
    //
    // NARROW GUARD, by design, as with 8.3.59 and 8.2.25: the only jhal
    // sounds reachable pada-finally in this suite are `t` and `z` (every
    // other form ends in a vowel, `H`, `m` or `n`, none of them jhal). The
    // guard names those two characters rather than calling `is_jhal`: the
    // other candidate is `s`, and 8.2.66 sasajuṣo ruḥ — implemented inside
    // the rule labelled 8.3.15 just below — is its apavāda, so `s` must NOT
    // be voiced here. A blanket widening to every jhal would rewrite √hiṃs's
    // `ahinas` to `ahinad` before 8.2.74 and 8.2.73 could act on it,
    // destroying the ru alternation and the `ahinaH` branch. Widen the
    // guard's character set the moment a root lands whose pada-final sound
    // is some other jhal.
    //
    // No contention with 8.4.55 cartva: the shape that would collide, an
    // aṅga-final jhal directly before a pada-final `t`, cannot arise because
    // 8.2.23 saṁyogāntasya lopaḥ sits above and drops the second consonant
    // first. √ad, the one root whose aṅga ends in a jhal, presents `Adat` —
    // a vowel before the ending.
    Rule {
        id: "8.2.39",
        name: "JalAM jaSo'nte",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let last = p.text().chars().last();
            if !matches!(last, Some('t') | Some('z')) {
                return false;
            }
            // Read the bearing term positionally rather than as ENDING:
            // 6.4.105 / 6.4.106 luk the ending outright (Bava, hinu), so
            // that index is not reliably the last non-empty one.
            let Some(idx) = p.terms.iter().rposition(|t| !t.text.is_empty()) else {
                return false;
            };
            let jash = jashtva_of(last.unwrap()).expect("t and z both have a jaS");
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            s.push(jash);
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.2.39", "JalAM jaSo'nte", before);
            true
        },
    },
    // 8.2.41 zaQoH kaH si: `ṣ` (z) or `ḍh` (Q) becomes `k` when the
    // immediately following sound is `s`. pinaz + si → pinak + si, and
    // 8.3.59 (widened below) then retroflexes that `s` back to `z` after
    // the new `k`: pinakzi.
    //
    // NARROW GUARD, by design, matching 8.2.30/8.4.41's discipline: only the
    // `z` arm is reachable this slice (rudhādi's one z-final curated root is
    // √piṣ; no curated root's aṅga ends in `Q`). Widen the match the moment
    // a Q-tailed root lands.
    //
    // Read via `word_chars`, not a term-boundary check, for the same reason
    // 8.2.30/8.4.41 do: śnam's infix leaves √piṣ's own tail — the `z` — at
    // the end of a non-final term (SHAP), one term short of the actual word
    // end (pi | naz | si), so the `s` that conditions it is the FIRST
    // character of the NEXT term.
    //
    // BELOW 8.2.23, and that is load-bearing. At laṅ madhyama eka the ending
    // is a bare `s`; 8.2.23 saṁyogāntasya lopaḥ, above in this file, elides
    // that `s` as the second member of a word-final conjunct before this
    // rule ever runs, so 8.2.41 finds no trigger here and the cell reduces
    // exactly as laṅ prathama eka does
    // (`shadhoh_kah_si_declines_when_8_2_23_ate_the_s_first` in
    // `super::derivation_tests`). Reversed — this rule above 8.2.23 — the
    // `z` becomes `k` before the `s` is elided, and the cell surfaces
    // `apinak`: a real-word-looking form that splits madhyama eka from
    // prathama eka and that no guard test would flag; only the golden and
    // the trace pin catch it.
    Rule {
        id: "8.2.41",
        name: "zaQoH kaH si",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let w = word_chars(p);
            for i in 1..w.len() {
                if w[i].2 != 's' || w[i - 1].2 != 'z' {
                    continue;
                }
                let (term, idx, _) = w[i - 1];
                let before = p.snapshot();
                set_char(p, term, idx, 'k');
                p.record("8.2.41", "zaQoH kaH si", before);
                return true;
            }
            false
        },
    },
    // 8.2.74 sipi dhāto rur vā (vikalpa): before sip, the dhātu's final
    // optionally becomes ru, which 8.3.15 then takes to a visarga.
    // ahinas + s → ahinaH.
    //
    // ORDERED ABOVE 8.2.73, against sūtra order, and this is load-bearing.
    // This rule replaces the DHĀTU'S OWN FINAL — the `s` — so it must see
    // `ahinas`. Below 8.2.73 it would find `ahinad` and have no `s` to act
    // on, and ahinaH would never be derived. Nothing in the code enforces
    // the order; `shnams_ru_fires_on_the_dhatus_own_final` in
    // `super::derivation_tests` is the guard, and it asserts the ORDER,
    // because the wrong one still produces a real word.
    Rule {
        id: "8.2.74",
        name: "sipi DAto rurvA",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Rudhadi) || !p.ctx.is_sip() {
                return false;
            }
            if !dhatu_is_pada_final(p) {
                return false;
            }
            if !p.text().ends_with('s') {
                return false;
            }
            let before = p.snapshot();
            let Some(idx) = p.terms.iter().rposition(|t| !t.text.is_empty()) else {
                return false;
            };
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            s.push('r');
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.2.74", "sipi DAto rurvA", before);
            true
        },
    },
    // 8.2.73 tipy anasteḥ: before tip, a dhātu other than √as takes `d` for
    // its final. ahinas + t → ahinad.
    //
    // This is what fills the hole 8.2.39 leaves. 8.2.39 jhalāṁ jaśo'nte is
    // guarded narrowly to a final `t`, and correctly so — a final `s` is
    // 8.2.66 / 8.3.15's, not jaśtva's — so without this rule √hiṃs would
    // surface as *ahinaH in laṅ prathama eka. √kṛt needs nothing here: its
    // final really is a `t` and 8.2.39 handles it.
    //
    // DELIBERATE OVER-APPLICATION, recorded so it is not later read as a
    // bug: the sūtra says *tipi*, and this guard covers sip as well. The
    // reason is structural — 8.2.74 above is optional *against* the `d`,
    // so its declined branch has to be able to reach one. Same treatment
    // the previous slice gave 7.1.35's āśiṣi condition.
    //
    // NOT a slot predicate: `dhatu_is_pada_final` plus the `s`-final check
    // below are what actually select the cells this rule fires on, and in
    // this grammar those happen to be exactly tip and sip — no separate
    // `is_tip() || is_sip()` clause is needed to say so. There WAS one; the
    // mutation gate proved it had no witness (mutating `is_tip` to always
    // return `true` survived, because the clause was already true wherever
    // this rule could fire), so it was removed along with `Context::is_tip`
    // itself, which had no other caller. `Context::is_sip` stays: 8.2.74 and
    // 8.2.75 both still guard on it directly, and those two really are
    // sip-only (8.2.74) or downstream of a sip-only branch (8.2.75).
    //
    // WHY tip/sip is what falls out: `ENDING` is only ever empty because
    // 8.2.23 saṁyogāntasya lopaḥ collapsed a word-final consonant conjunct,
    // and in this grammar that happens at exactly one slot family — laṅ
    // prathama/madhyama eka, i.e. tip and sip. `dhatu_is_pada_final` is
    // testing for that emptiness, not for tip/sip directly, so it inherits
    // the restriction only as long as that fact holds. This rule is
    // OBLIGATORY (`vikalpa: false`), so if a future slice's root set ever
    // makes `ENDING` empty at some other slot (a different saṁyoga shape,
    // or another rule that luks the ending), this guard would over-fire
    // there silently — no test failure until a golden happens to catch it.
    // Re-verify this invariant before widening the root set.
    Rule {
        id: "8.2.73",
        name: "tipyanasteH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Rudhadi) {
                return false;
            }
            if !dhatu_is_pada_final(p) {
                return false;
            }
            if !p.text().ends_with('s') {
                return false;
            }
            let before = p.snapshot();
            let Some(idx) = p.terms.iter().rposition(|t| !t.text.is_empty()) else {
                return false;
            };
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            s.push('d');
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.2.73", "tipyanasteH", before);
            true
        },
    },
    // 8.2.75 daś ca (vikalpa): and a final `d` likewise becomes ru before
    // sip. akfRad + s → akfRaH. The counterpart of 8.2.74 for a stem whose
    // final is already a stop — √kṛt's, voiced by 8.2.39 just above.
    //
    // MUST DECLINE ON A `d` THAT CAME FROM 8.2.73, not just any pada-final
    // `d`: for √hiṃs, 8.2.74 declining leaves `ahinas`, which 8.2.73 (its
    // sip over-application) then voices to `ahinad`. If this rule read that
    // `d` too it would offer a second, redundant route to the SAME surface
    // `ahinaH` — verified empirically, by removing this clause and
    // rerunning `rudhadi_lan_eka_cells`/`shnams_ru_fires_on_the_dhatus_own_final`:
    // laṅ madhyama eka for √hiṃs forked into four branches, not three, two
    // of them the identical text `ahinar` (one via 8.2.74 directly, one via
    // 8.2.73 then this rule) — a real duplicate, not merely two equally
    // valid derivations, and `shnams_ru_fires_on_the_dhatus_own_final`
    // failed because `.find` could land on the 8.2.73-then-8.2.75 branch,
    // whose trace never mentions 8.2.74 at all. √kṛt is untouched: its `d`
    // is always 8.2.39 jaśtva's, and 8.2.73 never fires on it (kft's own
    // final is never `s`), so the log never has "8.2.73" to check for.
    Rule {
        id: "8.2.75",
        name: "daSca",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Rudhadi) || !p.ctx.is_sip() {
                return false;
            }
            if !dhatu_is_pada_final(p) {
                return false;
            }
            if p.log.iter().any(|s| s.sutra == "8.2.73") {
                return false;
            }
            if !p.text().ends_with('d') {
                return false;
            }
            let before = p.snapshot();
            let Some(idx) = p.terms.iter().rposition(|t| !t.text.is_empty()) else {
                return false;
            };
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            s.push('r');
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.2.75", "daSca", before);
            true
        },
    },
    // 8.2.66 sasajuṣo ruḥ + 8.3.15 kharavasānayoḥ: word-final `s` → visarga.
    // Widened to a final `r` as of this slice: 8.2.74/8.2.75 above now
    // produce a genuine intermediate ru (`r`) for √hiṃs and √kṛt, and this
    // is what finishes it to `H` — no other rule in this suite ever leaves
    // a word-final `r` for it to misfire on (grep confirms `push('r')` has
    // exactly those two call sites).
    //
    // The bearing term is now found the same way 8.2.39 finds it —
    // `rposition`, not a fixed `terms.len() - 1` — for the same reason:
    // in `hiMstaH` (an existing, pre-slice golden) `ENDING` genuinely holds
    // the final `s` and the fixed index already worked, but in this
    // slice's own laṅ prathama/madhyama cells `ENDING` is empty (8.2.23
    // consumed it) and the fixed index would silently write `H` onto an
    // empty term while leaving the real `s`/`r` untouched, producing
    // `ahinasH` instead of `ahinaH`. This is a pure widening, not a
    // behaviour change, for every case this rule was previously reachable
    // in: this pipeline's tinanta terms are always exactly
    // `[ANGA, SHAP, ENDING]`, so `rposition` degrades to the old
    // `terms.len() - 1` whenever `ENDING` is non-empty, e.g. `hiMstaH`.
    Rule {
        id: "8.3.15",
        name: "KaravasAnayor visarjanIyaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !matches!(p.text().chars().last(), Some('s') | Some('r')) {
                return false;
            }
            let Some(idx) = p.terms.iter().rposition(|t| !t.text.is_empty()) else {
                return false;
            };
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            s.push('H');
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.3.15", "KaravasAnayor visarjanIyaH", before);
            true
        },
    },
    // 8.3.24 naścāpadāntasya jhali: a non-pada-final `n` becomes an
    // anusvāra before a jhal. In this suite that `n` is always śnam's, and
    // the jhal is whatever the weak stem's tail or the ending supplies.
    //
    // Paired with 8.4.58 below, which usually turns the anusvāra straight
    // back into the same `n`. The pair is not a no-op, and √hiṃs is why:
    // hiMs + taH stops here, because 8.4.58 needs a YAY to follow and what
    // follows is the root's own `s`, which is śal. hiMstaH keeps its
    // anusvāra where kfntaH does not.
    //
    // NARROW GUARD: rudhādi only. The `n` of 7.1.3 jho'ntaH (aBavan,
    // kfntan) is pada-final and out of scope by the sūtra's own
    // `apadāntasya`; guarding on the gaṇa keeps this rule away from it
    // without needing a pada-boundary notion the engine does not have.
    Rule {
        id: "8.3.24",
        name: "naScApadAntasya Jali",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Rudhadi) {
                return false;
            }
            let w = word_chars(p);
            let Some(pos) = w.iter().position(|(_, _, c)| *c == 'n') else {
                return false;
            };
            // `apadāntasya`: something must follow, and it must be a jhal.
            let Some((_, _, next)) = w.get(pos + 1) else {
                return false;
            };
            if !is_jhal(*next) {
                return false;
            }
            let (term, idx, _) = w[pos];
            let before = p.snapshot();
            set_char(p, term, idx, 'M');
            p.record("8.3.24", "naScApadAntasya Jali", before);
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
    // than a/ā, plus (as of this slice) `g` and `k` — so every arm is
    // executed by a test and the mutation gate stays clean. Same discipline
    // that removed 6.1.78's E/O arms in slice 5e (and 8.4.53 itself, in
    // `9fa8e5f` — since restored below, rudhādi having supplied it a witness
    // the discipline still required), and the same shape as 8.2.25's narrow
    // guard. Widen further the moment a root lands whose aṅga ends in
    // h/y/v/r/l or another ku sound (K/G/N) before an s-initial affix.
    //
    // Two ku triggers have now landed, each widening this rule once, and
    // both are inside 8.3.57 iṇ-koḥ's own scope:
    //
    // The `g` arm is √bhañj's: coH kuH (8.2.30, above in this file's
    // pipeline order) has already turned the dhātu's final `j` into `g`
    // before this rule runs (bhanaj + si → bhanag + si), so what precedes
    // `si` here is the ku sound `g`, not yet devoiced to `k` — khari ca
    // (8.4.55) sits below this rule and does that afterwards. bhanakzi
    // (`super::derivation_tests::bhanj_lat_all_nine_cells`) is the witness.
    //
    // The `k` arm is √piṣ's: zaQoH kaH si (8.2.41, above in this file's
    // pipeline order) has already turned the dhātu's own `z` into `k` before
    // this rule runs (pinak + si), so what precedes `si` here is the ku
    // sound `k` directly — not an aṅga-final sound at all, since rudhādi's
    // śnam split (3.1.78) puts √piṣ's tail in SHAP, one term short of ANGA.
    // pinakzi (`super::derivation_tests::pish_lat_madhyama_eka_is_pinakshi`)
    // is the witness.
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
            let is_in_trigger = is_vowel(prev) && !matches!(prev, 'a' | 'A');
            if !is_in_trigger && !matches!(prev, 'g' | 'k') {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[next_idx].text.chars().skip(1).collect();
            p.terms[next_idx].text = format!("z{rest}");
            p.record("8.3.59", "AdeSapratyayayoH", before);
            true
        },
    },
    // 8.4.41 ṣṭunā ṣṭuḥ: a dental (`s`, or a t-varga stop) retroflexes when
    // it immediately neighbours `ṣ` (z) or a ṭ-varga stop. pinaz + ti →
    // pinaz + wi → pinazwi; piMz + tas → piMzwaH; piMz + Di — in the loṭ
    // madhyama eka cell, which is DELIBERATELY LEFT INTERMEDIATE this task
    // (it currently reaches piMzQi, not the finished piRqQi) — takes the
    // same D → Q step; Task 6 is what finishes the cell around it.
    //
    // SŪTRA ORDER, AND NOW ALSO LOAD-BEARING ORDER. It sits above 8.4.53
    // because that is where vidyut-prakriya's data/sutrapatha.tsv places it
    // — but since Task 5 gave `jashtva_of` a `z → q` arm, the two rules no
    // longer touch disjoint sounds on this junction: piMz + Di's `z` is now
    // exactly what jaśtva would take if it saw it first. With 8.4.41 above,
    // it fires on the `z`/`D` pair before 8.4.53 runs, retroflexing D → Q
    // and leaving 8.4.53 nothing to see (its guard requires a literal `D` at
    // that position, which is gone). Run 8.4.53 first instead and it would
    // read piMz + Di's `z` as the jaśtva target — jashtva_of('z') is no
    // longer a no-op — and rewrite it to `q` before 8.4.41 ever saw its `z`
    // trigger, giving piMqDi instead of piMzQi. Do not reorder these two
    // rules without re-deriving this cell.
    //
    // STRICT ADJACENCY is the load-bearing part of the guard: only the
    // IMMEDIATELY preceding character is read, never scanned past. A
    // forward scan for "some dental after a z" would wrongly retroflex
    // piMzanti's `n` (across the intervening `a`) into *piMzaRti; that
    // retroflexion is ṇatva's (8.4.1 / 8.4.2), which 8.4.2 explicitly lets
    // an aṭ intervene in — `shtutva_requires_strict_adjacency` in
    // `super::derivation_tests` is the witness that the two rules stay
    // disjoint.
    //
    // NARROW GUARD, by design, matching 8.3.59's discipline just above: the
    // only trigger √piṣ ever presents is its own `z`, so only `z` is checked
    // here — no curated root reaches a ṭ-varga-stop trigger yet. The
    // correspondence match below is narrowed the same way: only t/T/D have
    // a witness (d/n/s do not). Widen both the moment a root or a junction
    // reaches the wider cases.
    Rule {
        id: "8.4.41",
        name: "zwunA zwuH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let w = word_chars(p);
            for i in 1..w.len() {
                if w[i - 1].2 != 'z' {
                    continue;
                }
                let sub = match w[i].2 {
                    't' => 'w',
                    'T' => 'W',
                    'D' => 'Q',
                    _ => continue,
                };
                let (term, idx, _) = w[i];
                let before = p.snapshot();
                set_char(p, term, idx, sub);
                p.record("8.4.41", "zwunA zwuH", before);
                return true;
            }
            false
        },
    },
    // 8.4.53 jhalāṁ jaś jhaśi: a jhal becomes its jaś before a jhaś (a
    // voiced aspirate). kfnt + Di → kfnd + Di → kfndDi.
    //
    // RESTORED, not reverted. This rule was removed in 9fa8e5f as
    // unreachable: slice 5d had analysed the ās/vas junction as jaśtva and
    // shipped *AdDve, and 8.2.25 dhi ca — which ELIDES the `s` rather than
    // voicing it, and sits in 8.2, asiddha to all of 8.4 — bled it
    // completely. Nothing else in the suite reached it. rudhādi does: √kṛt's
    // stem-final `t` is not an `s`, so 8.2.25 declines and this junction is
    // genuinely jaśtva's.
    //
    // 8.2.25 still bleeds it for √hiṃs, which is why hinDi and kfndDi differ
    // in shape — the same cell of the same gaṇa, reached by two different
    // rules. Both are asserted in `super::derivation_tests`.
    Rule {
        id: "8.4.53",
        name: "JalAM jaS JaSi",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let w = word_chars(p);
            let Some(pos) = w.len().checked_sub(2) else {
                return false;
            };
            // The jhaś that conditions it: in this suite always the `D` of
            // 6.4.101's Di, at the last position.
            if w.last().map(|(_, _, c)| *c) != Some('i') || w[pos].2 != 'D' {
                return false;
            }
            let Some(target) = pos.checked_sub(1) else {
                return false;
            };
            let Some(jash) = jashtva_of(w[target].2) else {
                return false;
            };
            // No-op guard, as 8.4.55 below takes for the identical reason:
            // a target that is already its own jaś (√ad's `d`, adDi) must
            // not record a vacuous step.
            if jash == w[target].2 {
                return false;
            }
            let (term, idx, _) = w[target];
            let before = p.snapshot();
            set_char(p, term, idx, jash);
            p.record("8.4.53", "JalAM jaS JaSi", before);
            true
        },
    },
    // 8.4.55 khari ca (cartva): a jhal immediately before the ending, meeting
    // a khar across that junction, becomes its car (voiceless unaspirated).
    // √ad's d before ti/tas/si/tha → t: atti, attaH, atsi, atTa. The engine's
    // first internal junction sandhi; general, reused by every later
    // gaṇa/subanta slice. No longer the pipeline's last rule — 8.4.65 and
    // 8.4.56 both follow it now — but still ordered after every other 8.3/8.4
    // rule that precedes it.
    //
    // FIXED for rudhādi's ANGA/SHAP split (Task 7, √khid's Kintte the
    // witness). This rule predates gaṇa 7 and originally read
    // `p.terms[ANGA]` directly for both "the aṅga's final sound" and, via
    // "the first non-empty term after ANGA", for "the ending's first
    // sound" — sound reasoning only while ANGA held the whole root and
    // SHAP was either empty (adādi's luk) or a genuine vikaraṇa. rudhādi's
    // śnam-split root (3.1.78) puts the root's OWN tail in SHAP (`Ki` /
    // `nd` for Kid's weak stem), so the old code asked about `i` (ANGA's
    // vowel) meeting `n` (SHAP's own first char) — never the real
    // boundary, SHAP's `d` meeting the ending's `t`. Kindte, not Kintte,
    // was the result. Now the target is the last non-empty term's final
    // char before `ENDING` (matching `sound_before_ending`'s reasoning in
    // `terms.rs`; open-coded rather than calling it, joining 8.3.59 and
    // 7.1.5 as the enumerated duplicates that helper's doc comment tracks,
    // because this rule also needs the term index to write back into), and
    // the trigger is `ENDING`'s own first sound directly, since tripādī
    // rules always run after 3.1.68 and `ENDING` is always the pada's last
    // term. √ad is unaffected: SHAP is empty there, so both reads reduce
    // to exactly what they were.
    Rule {
        id: "8.4.55",
        name: "Kari ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let next = p.terms.get(ENDING).and_then(|t| t.text.chars().next());
            let Some(next) = next else { return false };
            if !is_khar(next) {
                return false;
            }
            let Some((term, idx)) =
                p.terms[..ENDING]
                    .iter()
                    .enumerate()
                    .rev()
                    .find_map(|(ti, t)| {
                        if t.text.is_empty() {
                            None
                        } else {
                            Some((ti, t.text.chars().count() - 1))
                        }
                    })
            else {
                return false;
            };
            let last = p.terms[term].text.chars().nth(idx).unwrap();
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
            set_char(p, term, idx, sub);
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
    // 8.4.58 anusvārasya yayi parasavarṇaḥ: an anusvāra becomes the
    // following sound's homorganic nasal, before a YAY only. This is the
    // return leg of the 8.3.24 pair — kfMt → kfnt — and it declines for
    // hiMs + taH, whose anusvāra is followed by śal `s`.
    //
    // ORDERED AFTER 8.4.1 / 8.4.2, and this is constrained — contrary to
    // what the spec assumed. `is_natva_target` in this file FOLDS 8.3.24 in
    // as a guard ("a non-padānta n before a jhal has ALREADY become an
    // anusvāra by the time the 8.4 rules run"), a simplification taken when
    // the engine had no anusvāra machinery. It does now, but only for
    // rudhādi: 8.3.24 above is gaṇa-guarded, so BAzante's `n` is still an
    // `n` when ṇatva runs and the fold is still load-bearing for every
    // other root. The fold therefore stays.
    //
    // Given that, this rule must run AFTER ṇatva. Placed before it, kfMt
    // would already be kfnt when 8.4.1 looks, and the weak stem would
    // decline only by falling through the stale fold rather than because
    // its nasal is genuinely an anusvāra. Placed here, kfntaH declines for
    // the right reason (`M` is not `n`) while kfRatti — whose `n` precedes
    // a vowel, so 8.3.24 never fired — still takes ṇatva.
    //
    // Retire the fold, and this constraint with it, when a slice widens
    // 8.3.24 past rudhādi.
    Rule {
        id: "8.4.58",
        name: "anusvArasya yayi parasavarRaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let w = word_chars(p);
            let Some(pos) = w.iter().position(|(_, _, c)| *c == 'M') else {
                return false;
            };
            let Some((_, _, next)) = w.get(pos + 1) else {
                return false;
            };
            let Some(nasal) = parasavarna_of(*next) else {
                return false;
            };
            let (term, idx, _) = w[pos];
            let before = p.snapshot();
            set_char(p, term, idx, nasal);
            p.record("8.4.58", "anusvArasya yayi parasavarRaH", before);
            true
        },
    },
    // 8.4.65 jharo jhari savarṇe (vikalpa): a jhar is optionally elided
    // before a savarṇa jhar. kfnttaH ~ kfntaH, kfndDi ~ kfnDi,
    // Kintte ~ Kinte.
    //
    // GUARD CARRIES 8.4.64's `halaḥ` BY ANUVṚTTI. 8.4.64 halo yamāṁ yami
    // lopaḥ sits immediately above this sūtra in the tripādī (verified
    // against vidyut-prakriya's data/sutrapatha.tsv) and its `halaḥ` —
    // "when preceded by a consonant" — carries down. Without it this rule
    // over-applies to kfRatti's `tt`, whose first `t` follows the vowel
    // `a`, and forks it to *kfRati — contradicting the pinned
    // `kfRatti` golden. With the guard, kfnttaH's `t` (after `n`) and
    // kfndDi's `d` (after `n`) fire, while kfRatti's `t` (after `a`)
    // declines, exactly matching the ALTERNATES table.
    //
    // The scan starts at index 1, not 0: index 0 has no preceding sound, so
    // `halaḥ` cannot be satisfied there and `w[i - 1]` would underflow.
    //
    // NO SEPARATE JHAL ARM, by design, as 8.4.56 below states for the
    // analogous case: `is_savarna`'s `series()` recognises exactly the 20
    // varga stops, and every one of them is already in `is_jhal`'s set — so
    // `is_savarna`'s series test IS the jhal test, and a standalone
    // `is_jhal` conjunct on either side would be dead code. This also lines
    // up with the sūtra's own *jharaḥ*, which excludes `h` from *jhal* —
    // `series()` already rejects `h` (it has no varga), so the narrower
    // `is_savarna` alone tracks *jhar*, not merely *jhal*.
    //
    // PLACEMENT AGAINST 8.4.56 IS LOAD-BEARING and unenforceable by the
    // compiler, but it governs TRACE ORDER within a branch, not WHICH forms
    // exist: both rules are optional and both sit at the end of the tripādī,
    // and running 8.4.56 first would still reach every member this rule
    // does — e.g. 8.4.56 could fork kfnttAd to kfnttAt directly, and this
    // rule would then fire on that pada-final `tt` just as readily, since
    // both `t`s are savarṇa either way. What the stated order fixes is the
    // sequence each branch's trace records the two rules in — 8.4.65 before
    // 8.4.56 — which Task 9's `kfntAt` trace pin in
    // `crates/panini/tests/trace.rs` asserts directly
    // (`at(&t, "8.4.65") < at(&t, "8.4.56")`). `tinanta_rule_order_is_pinned`
    // in `super::derivation_tests` is what holds this file's order today.
    //
    // It is also the rule that takes √kṛt's loṭ eka cells to five and six
    // forms, stacking with 7.1.35 and 8.4.56. That is the deepest fork the
    // engine produces, and the witness for ARCHITECTURE.md's branch-count
    // claim: k = 3 gives six branches, not eight, because 8.4.56 declines on
    // the vowel-final non-tātaṅ branch.
    Rule {
        id: "8.4.65",
        name: "Jaro Jari savarRe",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            let w = word_chars(p);
            let Some(pos) = (1..w.len().saturating_sub(1))
                .find(|i| !is_vowel(w[i - 1].2) && is_savarna(w[*i].2, w[i + 1].2))
            else {
                return false;
            };
            let (term, idx, _) = w[pos];
            let before = p.snapshot();
            remove_char(p, term, idx);
            p.record("8.4.65", "Jaro Jari savarRe", before);
            true
        },
    },
    // 8.4.56 vāvasāne: at the end of an utterance a jhal OPTIONALLY becomes
    // its car, continuing khari ca's operation. After 8.2.39 the only
    // reachable jhal-final is `d`, so in practice this restores the `t` that
    // 8.2.39 voiced — which is exactly the relationship the sūtras state,
    // and why `aBavat` is now an alternate rather than the pinned form.
    //
    // LAST rule in the pipeline, deliberately. Avasāna is the end of the
    // utterance, so the rule must see the finished word; and being last, it
    // satisfies the ordering constraint on optional rules trivially, since
    // no consumer sits below it at all.
    //
    // NARROW GUARD, by design: `cartva_of` alone carries the jhal test here.
    // There is no separate `is_jhal(last)` arm — `cartva_of`'s `Some` domain
    // (the five vargas' stops) is already a strict subset of `is_jhal`'s (it
    // omits the sibilants and `h`), so a standalone jhal check would be dead
    // code, unreachable by any input that doesn't already fail the
    // `cartva_of` let-else below. Nor is there a `sub == last` no-op check:
    // 8.2.39 obligatorily turns every pada-final `t` into `d` upstream, and
    // no cell in this suite ends in any other jhal, so `cartva_of(last)`
    // never yields its argument back. Widen with a real guard, not a
    // speculative one, the moment either assumption stops holding.
    Rule {
        id: "8.4.56",
        name: "vA'vasAne",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            let Some(last) = p.text().chars().last() else {
                return false;
            };
            let Some(sub) = cartva_of(last) else {
                return false;
            };
            let Some(idx) = p.terms.iter().rposition(|t| !t.text.is_empty()) else {
                return false;
            };
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            s.push(sub);
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.4.56", "vA'vasAne", before);
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
    // `form_g` lives in `derivation_tests.rs`; `mod.rs` re-exports it, so
    // this import stays on the stable `crate::tinanta::form_g` path.
    use crate::tinanta::derivation_tests::sole;
    use crate::tinanta::derive;
    use crate::tinanta::form_g;
    use panini_data::{Lakara, Purusha, Vacana, dhatus};

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

        // No current root reaches a `K` trigger — only `g` and `k` are
        // widened in. This case pins that the guard checks the exact chars
        // `g`/`k`, not the whole ku set (K/G/N), per the comment's own
        // "widen further" note.
        let mut p = Prakriya {
            terms: vec![Term::new("pi"), Term::new("naK"), Term::new("si")],
            log: vec![],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "si");
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
        // pi + nak + si: the iN trigger is SHAP's `k` (8.2.41's zaQoH kaH si
        // having already turned piz's own `z` into `k`), not the anga's `i`
        // -- and not even an anga-final sound at all, since rudhAdi's Snam
        // split (3.1.78) puts piz's tail in SHAP, one term short of ANGA.
        let mut p = Prakriya {
            terms: vec![Term::new("pi"), Term::new("nak"), Term::new("si")],
            log: vec![],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "pinakzi");
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

    /// 8.2.30 velarises a `j` that is word-final or immediately followed by
    /// a jhal, and declines otherwise. Only the `j` -> `g` arm is reachable
    /// this slice, so this pins that guard rather than the wider cu/ku set.
    #[test]
    fn coh_kuh_fires_only_word_finally_or_before_a_jhal() {
        let rule = rules().find(|r| r.id == "8.2.30").unwrap();

        // before a jhal: the `j` sits at the end of a non-final term (śnam's
        // infix leaves it in SHAP), and the jhal that conditions it is the
        // first character of the term after — the cross-term adjacency
        // `word_chars` exists for.
        let mut p = Prakriya {
            terms: vec![Term::new("Ba"), Term::new("naj"), Term::new("ti")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "Banagti");

        // word-final: nothing follows the `j` at all.
        let mut p = Prakriya {
            terms: vec![Term::new("Ba"), Term::new("naj")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "Banag");

        // before a vowel: neither jhal nor word-final, so the rule declines.
        let mut p = Prakriya {
            terms: vec![Term::new("Ba"), Term::new("nj"), Term::new("anti")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "Banjanti");
    }

    /// 8.2.39 voices a pada-final `t` or `z` and nothing else. The `s` case
    /// belongs to its apavāda 8.2.66 (implemented inside the rule labelled
    /// 8.3.15), and a `t` that is not pada-final is untouched.
    #[test]
    fn jhalam_jasho_ante_fires_only_on_a_pada_final_t_or_sh() {
        let rule = rules().find(|r| r.id == "8.2.39").unwrap();

        let mut p = Prakriya {
            terms: vec![Term::new("aBav"), Term::new("a"), Term::new("t")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "aBavad");

        // not pada-final: the `t` is followed by more of the ending
        let mut p = Prakriya {
            terms: vec![Term::new("aBav"), Term::new("a"), Term::new("tAm")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        // word-final `z`: jashtva_of('z') is `q` (1.1.50 nearest-substitute,
        // not place-and-manner correspondence).
        let mut p = Prakriya {
            terms: vec![Term::new("apina"), Term::new("z")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "apinaq");

        // `s`-final belongs to 8.2.66/8.3.15, not here
        let mut p = Prakriya {
            terms: vec![Term::new("aBav"), Term::new("a"), Term::new("s")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        // vowel-final
        let mut p = Prakriya {
            terms: vec![Term::new("Bav"), Term::new("a"), Term::new("ti")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
    }

    /// 8.2.41 takes `z` to `k` immediately before an `s`, and declines
    /// otherwise. Only the `z` arm is reachable this slice (no curated root
    /// ends in `Q`), so this pins that guard rather than the wider zaQoH set.
    #[test]
    fn shadhoh_kah_si_fires_only_before_an_s() {
        let rule = rules().find(|r| r.id == "8.2.41").unwrap();

        // before an s: the `z` sits at the end of a non-final term (śnam's
        // infix leaves it in SHAP), and the `s` that conditions it is the
        // first character of the term after — the cross-term adjacency
        // `word_chars` exists for.
        let mut p = Prakriya {
            terms: vec![Term::new("pi"), Term::new("naz"), Term::new("si")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "pinaksi");

        // before any other sound: the rule declines.
        let mut p = Prakriya {
            terms: vec![Term::new("pi"), Term::new("naz"), Term::new("ti")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "pinazti");
    }

    /// 8.4.41 retroflexes a dental immediately after `z`, and declines both
    /// when a character intervenes and when the neighbour is not a dental
    /// at all — the shape `shtutva_requires_strict_adjacency` pins at the
    /// derivation level, here pinned directly against the rule.
    #[test]
    fn shtutva_fires_only_on_an_adjacent_dental() {
        let rule = rules().find(|r| r.id == "8.4.41").unwrap();

        // immediately adjacent: the `t` retroflexes to `w`.
        let mut p = Prakriya {
            terms: vec![Term::new("piz"), Term::new(""), Term::new("ti")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "pizwi");

        // the D arm, whose only derivation-level cell (loṭ madhyama eka) a
        // later task finishes; pinned here so the arm is not witness-free.
        let mut p = Prakriya {
            terms: vec![Term::new("piz"), Term::new(""), Term::new("Di")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "pizQi");

        // one character between the `z` and the dental: no contact, decline.
        let mut p = Prakriya {
            terms: vec![Term::new("piz"), Term::new("a"), Term::new("nti")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "pizanti");

        // a non-dental neighbour: y, v and m are none of them shtutva's
        // target, so the rule declines on each.
        for ending in ["ya", "va", "ma"] {
            let mut p = Prakriya {
                terms: vec![Term::new("piz"), Term::new(""), Term::new(ending)],
                ..Default::default()
            };
            assert!(!(rule.apply)(&mut p), "fired before {ending}");
            assert_eq!(p.text(), format!("piz{ending}"));
        }
    }

    /// 8.4.56 devoices a pada-final jhal. After 8.2.39 the only reachable
    /// one is `d`; a vowel, a visarga and a nasal all decline.
    #[test]
    fn va_avasane_fires_only_on_a_pada_final_jhal() {
        let rule = rules().find(|r| r.id == "8.4.56").unwrap();

        let mut p = Prakriya {
            terms: vec![Term::new("aBav"), Term::new("a"), Term::new("d")],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "aBavat");

        let mut p = Prakriya {
            terms: vec![Term::new("aBav"), Term::new("a"), Term::new("H")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        let mut p = Prakriya {
            terms: vec![Term::new("aBav"), Term::new("a"), Term::new("m")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        let mut p = Prakriya {
            terms: vec![Term::new("Bav"), Term::new("a"), Term::new("ti")],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
    }

    #[test]
    fn parasavarna_requires_a_yay() {
        // Enumerated rather than golden-driven: a predicate that fired
        // unconditionally still produces plausible Sanskrit for two of the
        // three 7a roots, and only √hiṃs catches it.
        for (id, la, pu, va, has_anusvara) in [
            ("his", Lakara::Lat, Purusha::Prathama, Vacana::Dvi, true),
            ("kft", Lakara::Lat, Purusha::Prathama, Vacana::Bahu, false),
        ] {
            let d = dhatus().iter().find(|d| d.id == id).unwrap();
            let p = sole(derive(d, la, d.pada, pu, va));
            assert!(
                p.log.iter().any(|s| s.sutra == "8.3.24"),
                "{id}: 8.3.24 should always fire on a weak rudhādi cell"
            );
            assert_eq!(
                p.text().contains('M'),
                has_anusvara,
                "{id}: anusvāra retention"
            );
        }
    }
}
