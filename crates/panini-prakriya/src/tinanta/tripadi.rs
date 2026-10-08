//! Tripādī: 8.2.18 … 8.4.56.
//!
//! Ordered AFTER 3.1.68, so the ending is at `ENDING` (index 4) and śap at
//! `SHAP` (index 3); `terms[SHAP].text` may be empty (2.4.72). See
//! `super::terms`.

use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::term::Tag;
use crate::tinanta::samjna::{KRP, KSUBHNADI};
use crate::tinanta::sound::{
    cartva_of, deaspirate_of, is_jhal, is_jhash, is_khar, is_natva_intervener, is_natva_trigger,
    is_savarna, is_shcu, is_shtu, is_vowel, jashtva_of, kutva_of, parasavarna_of, shcutva_of,
    shtutva_of,
};
use crate::tinanta::terms::{ABHYASA, ANGA, ENDING, SHAP, remove_char, set_char, word_chars};

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
    // Defensive rather than a bare `p.terms[ENDING..]`: none of 8.2.73,
    // 8.2.74 and 8.2.75 has a gaṇa test (8.2.75 since slice 3f, the other two
    // since 3f2), so a hand-built prakriyā with any layout can reach here.
    // `p.terms.get(ENDING)` at
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
///     *BAzaRte). The rule 8.3.24 itself is modelled, but only for rudhādi,
///     juhotyādi and a curādi root's own `n`; the condition below still
///     covers every other gaṇa, and is exactly equivalent within tripādī
///     order.
///
/// Retire both in favour of the real rules once 8.3.24 reaches every gaṇa.
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
    // 8.2.18 kṛpo ro laḥ: √kṛp's `r` becomes `l`, and an `f` an `x`.
    // 7.3.86's `karp` before ṇic → `kalp` (*kalpayati*). Keyed by row
    // (`super::samjna::KRP`), as vidyut-prakriya keys it by upadeśa:
    // `10.0408 kfpa` is another root. It rewrites the whole aṅga, ṇic's `ay`
    // included, which has no `r`; vidyut rewrites the dhātu term alone.
    // First in the tripādī, as vidyut runs it ahead of the rest of 8.2.
    Rule {
        id: "8.2.18",
        name: "kfpo ro laH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !KRP.contains(&p.ctx.dhatupatha) {
                return false;
            }
            let text = p.terms[ANGA].text.replace('f', "x").replace('r', "l");
            if text == p.terms[ANGA].text {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = text;
            p.record("8.2.18", "kfpo ro laH", before);
            true
        },
    },
    // 8.2.77 hali ca: a root ending in `r`/`v` with a short ik upadhā
    // lengthens that upadhā before a hal (8.2.76 rvorupadhāyā dīrghaḥ is the
    // anuvṛtti source). div, after guṇa is blocked, reaches this shape:
    // div + śyan (y-initial) → dīv → dīvyati. Self-guards on shape; one
    // other curated root fires it (sev has an e-upadhā, vart ends in t):
    // √pṝ's `pur` (7.1.102, juhotyādi 3d), which meets a hal-initial ending
    // across ślu's empty SHAP (pipUrtaH). √kṛ's own `kur` (6.4.110,
    // `tinanta/guna.rs`) matches the shape guard just as readily (short `u`
    // upadhā, `r` final), and 8.2.79 na BakurCurAm below carves it back out.
    Rule {
        id: "8.2.77",
        name: "hali ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
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
            // 8.2.79 na BakurCurAm: kur is exempted from this lengthening
            // — kurvanti, not kUrvanti. `ends_with` rather than `==` is a
            // tolerance for a prefixed aṅga: `akur` is that prefixed shape,
            // not laṅ's own — laṅ's aṅga stays `kur`. A prefixed aṅga is
            // caught too: akurutAm, not akUrutAm.
            if p.terms[ANGA].text.ends_with("kur") {
                return false;
            }
            // The segment the aṅga meets: śap when it has text, else the
            // ending. Juhotyādi's ślu (2.4.75) empties SHAP, so √pṝ's `pur`
            // meets the ending directly (pipUrtaH) — generalized in slice 3d
            // on 6.1.78's athematic arm, which falls back to
            // `p.terms[ENDING]` the same way. Adādi's luk (2.4.72) takes the
            // same path, but no curated adādi aṅga ends in r/v after i/u.
            // Open-coded rather than calling `following_sarvadhatuka`, as the
            // follower lookups in this crate are, so each keeps its own
            // mutation pin.
            let follower = match p.terms.get(SHAP) {
                Some(t) if !t.text.is_empty() => Some(t),
                Some(_) => p.terms.get(ENDING),
                None => None,
            };
            let Some(next) = follower.and_then(|t| t.text.chars().next()) else {
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
    // 8.2.78 upadhāyāṃ ca: a dhātu whose upadhā is `r` or `v` before a final
    // hal lengthens the short ik before that upadhā (8.2.76's dīrghaḥ and
    // 8.2.77's `r`/`v` by anuvṛtti). `urj` → `Urj` (*ūrjayati*), `curR` →
    // `cUrR`, `gurd` → `gUrd`, and √kṝt's `kirt` (7.1.101) → `kIrt`.
    // vidyut-prakriya reads the dhātu term; here ṇic is folded into `ANGA`,
    // and a ṇijanta aṅga reaches the tripādī as root + `ay` (7.3.84 then
    // 6.1.78 on ṇic's `i`, in all four lakāras), so the root is the text
    // before that `ay`. Any other aṅga is the root's own text. In laṅ √ūrj
    // declines: 6.1.90 has made its `u` part of an `O` (*aurjayat*). 8.2.79
    // na bhakurchurām needs no carve-out here: `kur` and `Cur` end in their
    // `r`, which is 8.2.77's shape, not this one's.
    Rule {
        id: "8.2.78",
        name: "upaDAyAM ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let anga = &p.terms[ANGA];
            let root = if anga.has(Tag::Nijanta) {
                anga.text
                    .strip_suffix("ay")
                    .expect("a ṇijanta aṅga reaches the tripādī as root + `ay`")
            } else {
                anga.text.as_str()
            };
            let c: Vec<char> = root.chars().collect();
            let n = c.len();
            if n < 3 || !matches!(c[n - 2], 'r' | 'v') || is_vowel(c[n - 1]) {
                return false;
            }
            let long = match c[n - 3] {
                'i' => 'I',
                'u' => 'U',
                'f' => 'F',
                'x' => 'X',
                _ => return false,
            };
            let text: String = c[..n - 3]
                .iter()
                .chain(std::iter::once(&long))
                .chain(&c[n - 2..])
                .collect::<String>()
                + &anga.text[root.len()..];
            let before = p.snapshot();
            p.terms[ANGA].text = text;
            p.record("8.2.78", "upaDAyAM ca", before);
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
        bars: &[],
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
        bars: &[],
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
    // 8.2.26 jhalo jhali: an `s` between two jhals is elided. Ba + Bs + tas
    // → Ba + B + tas, which 8.2.40 then takes to babDaH (√bhas, slice 3f2,
    // once 6.4.100 has elided the root's upadhā `a`).
    //
    // ORDERED AFTER 8.2.25 dhi ca, in sūtra order. On babDi the `s` stands
    // before `D`, and 8.2.25 — which needs no jhal on its left — takes it
    // first; this rule then finds no `s`. vidyut credits the same split.
    //
    // Reads the WHOLE WORD, as 8.2.40 and 8.4.53 do: the sūtra has no
    // positional condition. No curated cell outside √bhas presents jhal +
    // `s` + jhal anywhere (the 3f2 spec's corpus-wide trace diff), and
    // `ghasibhasor_and_jhalo_jhali_are_credited_only_on_bhas` in `panini`'s
    // trace suite holds that as a fact. An s-aorist would be the first
    // witness elsewhere, and this engine derives no luṅ.
    Rule {
        id: "8.2.26",
        name: "Jalo Jali",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let w = word_chars(p);
            for i in 1..w.len().saturating_sub(1) {
                if w[i].2 != 's' || !is_jhal(w[i - 1].2) || !is_jhal(w[i + 1].2) {
                    continue;
                }
                let (term, idx, _) = w[i];
                let before = p.snapshot();
                remove_char(p, term, idx);
                p.record("8.2.26", "Jalo Jali", before);
                return true;
            }
            false
        },
    },
    // 8.2.30 coH kuH: a cu stop (c C j J) is replaced by its ku counterpart
    // (the nearest velar by 1.1.50 sthāne'ntaratamaḥ, so voicing and
    // aspiration are preserved) when it ends a term and either the next
    // non-empty term is an affix or āgama beginning with a jhal, or nothing
    // follows it in the pada. Banaj + ti -> Banag + ti (before the jhal `t`,
    // then 8.4.55 khari ca devoices to Banakti); aBanaj -> aBanag
    // word-finally.
    //
    // The substitute is `kutva_of`, and so is the MATCH -- one lookup governs
    // both halves, so they cannot drift apart. That matters more than it
    // looks: the rule previously tested a literal `j` and wrote a literal
    // 'g', and widening only the match would have substituted `g` for a `c`
    // and still reached the right surface, 8.4.55 khari ca devoicing it to
    // `k` afterwards. Every paradigm golden would have passed. Do not
    // "simplify" `kutva_of` back into a hardcoded char: only
    // `rinakti_trace_reaches_k_in_one_step` in `crates/panini/tests/trace/rudhadi.rs`
    // can tell the two implementations apart.
    //
    // The 1.1.50 sthAne'ntaratamaH account above is therefore a description
    // of this code, not only of the sūtra.
    //
    // The rule reads TERMS, as vidyut-prakriya's does: the cu must be a
    // term's LAST sound. Slice 10m narrowed it from a whole-word scan, which
    // velarised any cu before a jhal and so turned curādi `picc`'s first `c`
    // (before the second) into `k` (*pikcayati* for *piccayati*). A root-
    // internal cu is not at a morpheme's end, and the sūtra's jhal is the
    // initial of what follows the term. The test
    // `coh_kuh_reads_only_a_terms_final_cu_before_an_affix` pins the case.
    //
    // The jhal is the first sound of the next NON-EMPTY term: śnam's infix
    // leaves the root's own tail — the `j` — in `SHAP` (`Ba | naj | ti`), so
    // the conditioning `t` is `ENDING`'s first sound, and adādi's luk'd śap
    // leaves an empty `SHAP` between a root and its ending. Pada-final is the
    // case where no non-empty term follows at all: 8.2.23 has eaten tip/sip's
    // own letter, leaving `ENDING` empty and the dhātu's `j` last.
    //
    // That next term must be an affix or āgama, as vidyut requires. This
    // engine does not tag terms by kind, but its slots fix it: every term
    // after `ANGA` is one. The non-affix terms that can follow a term are
    // `ABHYASA` (after `AGAMA`) and `ANGA`, the dhātu (after `AGAMA` or
    // `ABHYASA`). The guard `j != ANGA` excludes `ANGA` only: an āgama
    // ending in a cu before a jhal-initial abhyāsa is unreachable, because
    // aṭ and āṭ are vowel-final, so the narrower guard matches vidyut on
    // every reachable word. No abhyāsa or aṭ in the corpus ends in a
    // consonant (7.4.60 halādiḥ śeṣaḥ), so the guard has one witness, the
    // hand-built one in `coh_kuh_reads_only_a_terms_final_cu_before_an_affix`
    // (in the tests module below), and no golden.
    //
    // vidyut fires 8.2.30 on EVERY qualifying term; `apply` runs once per
    // derivation and velarises only the first. The two are equivalent on
    // the corpus (no word has two qualifying cu-final terms; the audit and
    // the main-vs-branch trace dump confirm). It is a deliberate
    // simplification the old scan shared; a future slice that reaches two
    // such terms must loop.
    //
    // The test lives INSIDE the search, not after it, the way 8.3.24's and
    // 8.4.58's own searches further down this array do: the rule finds the
    // first term whose final cu qualifies, so a term-final cu earlier in the
    // word that does not (a vowel follows) never hides a later one that does.
    Rule {
        id: "8.2.30",
        name: "coH kuH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let hit = (0..p.terms.len()).find_map(|i| {
                let to = kutva_of(p.terms[i].text.chars().last()?)?;
                let next = (i + 1..p.terms.len()).find(|&j| !p.terms[j].text.is_empty());
                let qualifies = match next {
                    Some(j) => j != ANGA && p.terms[j].text.chars().next().is_some_and(is_jhal),
                    None => true,
                };
                qualifies.then(|| (i, p.terms[i].text.chars().count() - 1, to))
            });
            let Some((term, idx, to)) = hit else {
                return false;
            };
            let before = p.snapshot();
            set_char(p, term, idx, to);
            p.record("8.2.30", "coH kuH", before);
            true
        },
    },
    // 8.2.31 ho ḍhaḥ: `h` becomes `Q` (ḍh). The *jhali* and *padasya*
    // conditions come by anuvṛtti from the same place 8.2.30 coH kuH reads
    // them. The guard is the whole-word scan 8.2.30 used until slice 10m
    // narrowed that rule alone to a term-final cu — find the first `h` that
    // is genuinely word-final or jhal-followed, rather than the first `h`
    // in the word, so a non-applicable `h` earlier can never hide a later
    // applicable one.
    //
    // tfneh + ti → tfneQ + ti; tfnh + tas → tfnQ + tas; atfneh → atfneQ
    // (pada-final, the laṅ arm, after 8.2.23 above has eaten tip's `t`).
    //
    // It must DECLINE before `m` and `v` — neither is a jhal — which is
    // exactly what leaves tfRehmi and tfMhvaH their `h`, and before a
    // vowel, which leaves tfMhanti its own. `is_jhal` already carries `h`
    // itself, so an `h h` junction would qualify; none arises here (6.4.101
    // has already taken loṭ's `hi` to `Di` by this point), and the general
    // form is kept rather than special-cased.
    Rule {
        id: "8.2.31",
        name: "ho QaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let w = word_chars(p);
            let Some(pos) = w.iter().enumerate().position(|(i, (_, _, c))| {
                *c == 'h' && w.get(i + 1).is_none_or(|(_, _, next)| is_jhal(*next))
            }) else {
                return false;
            };
            let (term, idx, _) = w[pos];
            let before = p.snapshot();
            set_char(p, term, idx, 'Q');
            p.record("8.2.31", "ho QaH", before);
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
    // The guard is one `jashtva_of` lookup, read as both match and
    // substitute — the same discipline 8.2.30 coH kuH now applies: there is
    // no separate literal-character test to drift out of sync with the
    // substitution table. `jashtva_of` already covers every varga (`t`, `z`,
    // and `D`, but also the velar/palatal/labial arms), so the rule fires on
    // any pada-final jhal it has a jaś for, not just the three sounds a
    // curated root used to reach.
    //
    // The velar arm is reachable only now that 8.2.30 stopped writing a
    // literal `g` and started substituting the nearest velar (1.1.50): a
    // `c`-final root like √ric presents a word-final `k` pada-finally
    // (`arinak`), and this rule voices it to `g` (`arinag`) the same way it
    // already voices `t`, `z`, and `D`. `D` itself became reachable earlier,
    // with the ubhayapada 1.3.72 slice's √rudh (`ruD`): √rudh's laṅ
    // prathama/madhyama eka expose the dhātu's OWN final — `D`, not an
    // ending's `t` — once 8.2.23 saṁyogāntasya lopaḥ elides tip/sip's own
    // consonant. The golden `aruRad` (`crates/panini/tests/paradigm/data/rudhadi.rs`,
    // `ruD laN Parasmaipada` cell 0) is the witness that made that arm
    // reachable, and 8.2.75 daś ca's own `ends_with('d')` guard depends on
    // it too: without the `D` arm here, 8.2.75 never sees a `d` to act on
    // and the `aruRaH` branch (cell 3) never derives at all.
    //
    // `s` still declines: `jashtva_of('s')` is `None`, because 8.2.66
    // sasajuṣo ruḥ — implemented inside the rule labelled 8.3.15 just below
    // — is its apavāda, so `s` must NOT be voiced here. A word-final `s`
    // (e.g. √hiṃs's `ahinas`) is 8.2.66's business, not jaśtva's, and this
    // guard leaves it alone exactly as the narrower guard did.
    //
    // The `Some(jash) == last` no-op check exists because `jashtva_of`'s
    // domain contains fixed points — `g`, `j`, `q`, `d`, `b` all map to
    // themselves — that the old three-literal guard never reached (none of
    // `t`, `z`, `D` is a jaś already). Without the check the rule would
    // "fire" vacuously on every already-jaś pada-final, recording a no-op
    // step in the trace log without changing any surface. This mirrors the
    // no-op guard 8.4.55 already carries.
    //
    // No contention with 8.4.55 cartva (which since 3f2 scans the whole
    // word, not only the aṅga/ending boundary): the shape that would
    // collide, an aṅga-final jhal directly before a pada-final `t`, cannot
    // arise because
    // 8.2.23 saṁyogāntasya lopaḥ sits above and drops the second consonant
    // first. √ad, the one root whose aṅga ends in a jhal, presents `Adat` —
    // a vowel before the ending.
    Rule {
        id: "8.2.39",
        name: "JalAM jaSo'nte",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let last = p.text().chars().last();
            let Some(jash) = last.and_then(jashtva_of) else {
                return false;
            };
            if Some(jash) == last {
                return false;
            }
            // Read the bearing term positionally rather than as ENDING:
            // 6.4.105 / 6.4.106 luk the ending outright (Bava, hinu), so
            // that index is not reliably the last non-empty one.
            let Some(idx) = p.terms.iter().rposition(|t| !t.text.is_empty()) else {
                return false;
            };
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[idx].text.chars().collect();
            s.pop();
            s.push(jash);
            p.terms[idx].text = s.into_iter().collect();
            p.record("8.2.39", "JalAM jaSo'nte", before);
            true
        },
    },
    // 8.2.40 jhaṣas tathor dho'dhaḥ: after a jhaṣ (voiced aspirated stop),
    // the ending's `t` or `T` (th) becomes `D`. inD + te -> inD + De, and
    // the widened 8.4.53 (7b Task 6, below) then voices the stem's own `D` to
    // `d` before it: indDe. 8.4.65 optionally elides that `d` before the
    // savarṇa `D`, which is where inDe comes from.
    //
    // The ONLY NEW source of a D-initial ending in this suite besides the
    // pre-existing 6.4.101 her dhiḥ (which already supplies one: √hiṃs's
    // hinDi) — see 8.4.53's comment below for why that bounds its
    // widening. Two roots present a jhaṣ immediately before their ending:
    // √indh, and — once 6.4.112 has elided its ā, slice 3c — √dhā
    // (da + D + tas). Every other root represented here either inserts a
    // real vikaraṇa syllable between the two — bhvādi's laBate, divādi's
    // yuDyate, svādi's stiG, kryādi's guDnAti — or ends in no jhaṣ. The
    // sūtra's own *adhaḥ* excludes √dhā (DattaH, not *DadDaH), so this
    // rule, and hence a fresh D-initial ending, is still reachable only
    // through √indh.
    //
    // *adhaḥ* is KEYED BY ROW NUMBER, `03.0011 quDA\Y`. √dhā's only jhaṣ
    // before a `t`/`T` in any of its cells is its own final `D`, so a
    // narrower "the jhaṣ belongs to the aṅga" clause could never be
    // falsified and is not written.
    //
    // DECLINES wherever the ending does not begin with a `t`/`T`: intse
    // (`s`), inDvahe (`v`) and inDmahe (`m`) all fail that match. Only
    // `intse` then feeds 8.4.55 khari ca afterward — `s` is khar
    // (sound.rs's is_khar), so the stem's `D` devoices to `t`. `v` and `m`
    // are NOT khar, so 8.4.55 declines too and inDvahe/inDmahe keep their
    // stem `D` untouched.
    //
    // The jhaṣ test lives INSIDE the scan (the loop `continue`s rather than
    // bailing), so a `t`/`T` earlier in the word that is not jhaṣ-preceded
    // can never hide a later one that is; √indh's only candidate is its
    // ending's own `t` either way, so the scan is a deliberate
    // non-narrowing, hardening against a shape no witness here exercises.
    Rule {
        id: "8.2.40",
        name: "JazastaTorDo'DaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if p.ctx.dhatupatha == "03.0011" {
                return false;
            }
            let w = word_chars(p);
            for i in 1..w.len() {
                if !matches!(w[i].2, 't' | 'T') {
                    continue;
                }
                if !is_jhash(w[i - 1].2) {
                    continue;
                }
                let (term, idx, _) = w[i];
                let before = p.snapshot();
                set_char(p, term, idx, 'D');
                p.record("8.2.40", "JazastaTorDo'DaH", before);
                return true;
            }
            false
        },
    },
    // 8.2.41 zaQoH kaH si: `ṣ` (z) or `ḍh` (Q) becomes `k` when the
    // immediately following sound is `s`. pinaz + si → pinak + si, and
    // 8.3.59 (widened below) then retroflexes that `s` back to `z` after
    // the new `k`: pinakzi.
    //
    // NO LONGER NARROW. This guard used to be narrow "by design," matching
    // the discipline 8.2.30 followed before `kutva_of` widened it (see
    // above) and the discipline 8.4.41's CORRESPONDENCE half (below) still
    // follows: implement only the reachable arm, and widen the match the
    // moment a new one lands. rudhādi 7e is that widening for this guard —
    // see BOTH SOUNDS THE SŪTRA NAMES below (past the two guard-mechanics
    // paragraphs) for why. ṣaḍhoḥ names exactly two sounds and both are
    // now covered, so nothing is left narrow here; a future widening on
    // this guard would mean the sūtra itself was misread, not that a new
    // root arrived.
    //
    // Read via `word_chars`, not a term-boundary check, for the same reason
    // 8.4.41 does: śnam's infix leaves √piṣ's own tail — the `z` — at
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
    //
    // BOTH SOUNDS THE SŪTRA NAMES. *ṣaḍhoḥ* is a dvandva — ṣ **and** ḍh —
    // and until rudhādi 7e the guard read `z` alone, because √piṣ was the
    // only root that reached the rule and it presents a `z`. √tṛh presents
    // the other: 8.2.31 ho ḍhaḥ turns its `h` into a `Q`, and tfneQ + si
    // must become tfnek + si (→ tfRekzi by 8.3.59 and 8.4.1). Same shape as
    // the 8.2.30 episode — a rule whose own name promised two cases and
    // whose code implemented one — caught here before an audit had to.
    Rule {
        id: "8.2.41",
        name: "zaQoH kaH si",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let w = word_chars(p);
            for i in 1..w.len() {
                if w[i].2 != 's' || !matches!(w[i - 1].2, 'z' | 'Q') {
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
    // ahinas + s → ahinaH; abaBas + s → abaBaH (juhotyādi √bhas, slice 3f2).
    //
    // NO GAṆA TEST since slice 3f2, as 8.2.75 since 3f: the guard is sip,
    // the dhātu pada-final (8.2.23 has eaten the ending), and a final `s`.
    // `tipy_anasteh_and_sipi_dhato_are_credited_only_on_rudhadi_and_bhas` in
    // `panini`'s trace suite holds that no curated root outside rudhādi and
    // √bhas reaches it.
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
        bars: &[],
        apply: |p| {
            if !p.ctx.is_sip() {
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
    // 8.2.75 daś ca (vikalpa): and a final `d` likewise becomes ru before
    // sip. akfRad + s → akfRaH; aciked + s → acikeH (juhotyādi √kit, slice
    // 3f). The counterpart of 8.2.74 for a stem whose final is already a
    // stop, voiced by 8.2.39 just above.
    //
    // NO GAṆA TEST. Its guard is the sūtra's own: sip, the dhātu pada-final
    // (8.2.23 has eaten the ending), and a final `d`. Through slice 3e it
    // also required Tag::Rudhadi; slice 3f dropped that, and the whole suite
    // passed unchanged without it, so no curated root outside rudhādi and
    // √kit reaches this rule. `das_ca_is_credited_only_on_rudhadi_and_kit`
    // in `panini`'s trace suite holds that as a fact.
    //
    // ORDERED ABOVE 8.2.73 (7b Task 8), against sūtra order, for the same
    // structural reason as 8.2.74 just above: this rule needs to see the
    // dhātu's OWN `d`, not one 8.2.73 manufactured from an `s`. At this
    // position 8.2.73 has not run yet, so the guard rests on phonology:
    // √hiṃs presents `ahinas`, which fails `ends_with('d')` and falls
    // through to 8.2.73 unchanged; √kṛt presents `akfRad` and √kit
    // `aciked`, 8.2.39 having voiced their `t`, and this rule fires on them
    // directly. `shnams_ru_fires_on_the_dhatus_own_final` and the 7a laṅ
    // cell tests in `super::derivation_tests` are the witnesses.
    Rule {
        id: "8.2.75",
        name: "daSca",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &[],
        apply: |p| {
            if !p.ctx.is_sip() {
                return false;
            }
            if !dhatu_is_pada_final(p) {
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
    // 8.2.73 tipy anasteḥ: before tip, a dhātu other than √as takes `d` for
    // its final. ahinas + t → ahinad; abaBas + t → abaBad (juhotyādi √bhas,
    // slice 3f2). No gaṇa test since 3f2, and no √as clause: √as is not
    // curated.
    //
    // This is what fills the hole 8.2.39 leaves. 8.2.39 jhalāṁ jaśo'nte
    // declines on a final `s` because `jashtva_of('s')` is `None` — a final
    // `s` is 8.2.66 / 8.3.15's business, not jaśtva's — so without this rule
    // √hiṃs would surface as *ahinaH in laṅ prathama eka. √kṛt needs
    // nothing here: its final really is a `t` and 8.2.39 handles it.
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
    // the restriction only as long as that fact holds.
    //
    // RE-VERIFIED (7b Task 8), against the deferred hazard above. √bhañj and
    // √piṣ are the first roots ADDED SINCE THE WARNING WAS WRITTEN to
    // empty `ENDING` under 8.2.23 — NOT the first roots after √hiṃs to do
    // so at all: √kṛt has emptied it at these same cells since 7a too
    // (8.2.75 fires for it there — `crates/panini/tests/paradigm/` pins
    // `("kft", "laN", 3, "akfRaH", "8.2.75")` — and firing requires
    // `dhatu_is_pada_final`). The warning was about widening the root set
    // beyond 7a's two, not about a single prior witness. √bhañj and √piṣ
    // do so at exactly the same slot family — laṅ prathama/madhyama eka —
    // so the invariant holds. This rule still declines on both anyway, on
    // its own `s`-final check: by the time this rule runs, 8.2.30 has
    // already velarised √bhañj's stem to `aBanag` and 8.2.39 has already
    // voiced √piṣ's to `apinaq`, and neither ends in `s`.
    // `the_ru_alternation_stays_off_the_new_roots` and
    // `no_8_2_73_step_appears_for_bhanj_or_pish` in
    // `super::derivation_tests` are the witnesses.
    //
    // RE-VERIFIED AGAIN (slice 3f2), when this rule and 8.2.74 dropped their
    // Tag::Rudhadi test. √bhas is the first non-rudhādi root to reach them,
    // and it empties `ENDING` at exactly laṅ prathama/madhyama eka — the same
    // slot family. With the gaṇa test gone, the whole suite and every prior
    // cell's trace were unchanged (the 3f2 spec's corpus-wide trace diff).
    //
    // This rule is OBLIGATORY (`vikalpa: false`), so the hazard is only
    // narrowed, not closed: if a future slice's root set ever makes
    // `ENDING` empty at some other slot (a different saṁyoga shape, or
    // another rule that luks the ending), this guard would over-fire there.
    // Since slice 3f2 a non-rudhādi root other than √bhas reaching it in
    // the four derived lakāras fails
    // `tipy_anasteh_and_sipi_dhato_are_credited_only_on_rudhadi_and_bhas`.
    // That test exempts every rudhādi row, and `credited()` in `panini`'s
    // trace suite covers only those four lakāras, so rudhādi rows and any
    // new lakāra are not covered: re-verify this invariant before adding a
    // lakāra, widening `credited()`, or widening the root set.
    Rule {
        id: "8.2.73",
        name: "tipyanasteH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
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
        bars: &[],
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
    // anusvāra before a jhal. In this suite that `n` is śnam's (rudhādi) or,
    // in juhotyādi, the root's own (√dhan, slice 3f: daDaMsi, daDaMhi), and
    // the jhal is whatever the stem's tail or the ending supplies.
    //
    // Paired with 8.4.58 below, which usually turns the anusvāra straight
    // back into the same `n`. The pair is not a no-op, and √hiṃs is why:
    // hiMs + taH stops here, because 8.4.58 needs a YAY to follow and what
    // follows is the root's own `s`, which is śal. hiMstaH keeps its
    // anusvāra where kfntaH does not.
    //
    // NARROW GUARD: rudhādi and juhotyādi only. The `n` of 7.1.3 jho'ntaH
    // (aBavan, kfntan) is pada-final and out of scope by the sūtra's own
    // `apadāntasya`; guarding on the gaṇa keeps this rule away from it
    // without needing a pada-boundary notion the engine does not have.
    //
    // CURĀDI, ROOT-INTERNAL ONLY (slice 10e). Seven adanta curādi roots carry
    // their own `n` before a jhal (`sanketa`, `ansa`, `sangrAma`, `anDa`,
    // `danqa`, `anka`, `anga`: *saṅketayati*, *aṃsayati*; 8.4.58 restores
    // √andha's `n` before `D`, as it does √gandh's); 10c's √gandh (`ganD`)
    // does too, and vidyut credits the pair on it as on them. For a curādi
    // root the search is confined to `ANGA`'s own characters: that `n` is
    // inside the dhātu, so `apadāntasya` holds by construction, and the
    // 7.1.3 `n` of *corayanti* — in the tiṅ term, never `ANGA` — stays out
    // of reach exactly as for every other gaṇa.
    // Dropping the gaṇa test entirely credits an 8.3.24 → 8.4.58 pair on
    // every 7.1.3 `n` (bhavanti, yanti, Apnuvanti, hinvanti: four trace pins
    // fail). Juhotyādi is safe under it: its aṅga is abhyasta, so 7.1.4 ad
    // abhyastāt takes the `J` (daDati) and laṅ takes jus (3.4.109), and no
    // juhotyādi `n` is 7.1.3's. Where √dhan's `n` meets `t`/`T`, 8.4.58
    // turns the anusvāra straight back (daDantaH), as in vidyut's trace.
    //
    // The `apadāntasya` / jhal test lives INSIDE the search, not after it:
    // the rule finds the first `n` that is genuinely followed by a jhal
    // rather than the first `n` in the word full stop, so a non-applicable
    // `n` earlier in the word can never hide a later, applicable one. Some
    // cells DO have more than one `n` candidate — Banjanti reaches this
    // rule as `B a n j a n t i`, and both `n`s (before `j` and before `t`)
    // are jhal-adjacent — but every cell traced for this suite has its
    // first candidate already be the applicable one (SHAP's own `n`, not a
    // later root- or ending-supplied one), so widening the search from
    // "first `n`" to "first APPLICABLE `n`" changes no trace here. This is
    // hardening against an ordering no witness here exercises, not a fix
    // for one observed.
    Rule {
        id: "8.3.24",
        name: "naScApadAntasya Jali",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let anga = &p.terms[ANGA];
            let root_only = anga.has(Tag::Curadi);
            if !anga.has(Tag::Rudhadi) && !anga.has(Tag::Juhotyadi) && !root_only {
                return false;
            }
            let w = word_chars(p);
            let Some(pos) = w.iter().enumerate().position(|(i, (term, _, c))| {
                *c == 'n'
                    && (!root_only || *term == ANGA)
                    && w.get(i + 1).is_some_and(|(_, _, next)| is_jhal(*next))
            }) else {
                return false;
            };
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
    // than a/ā, plus `g`, `k` and `r` — so every arm is
    // executed by a test and the mutation gate stays clean. Same discipline
    // that removed 6.1.78's E/O arms in slice 5e (and 8.4.53 itself, in
    // `9fa8e5f` — since restored below, rudhādi having supplied it a witness
    // the discipline still required), and the same shape as 8.2.25's narrow
    // guard. Widen further the moment a root lands whose aṅga ends in
    // h/y/v/l or another ku sound (K/G/N) before an s-initial affix.
    //
    // Three consonant triggers have now landed, each widening this rule
    // once, and all are inside 8.3.57 iṇ-koḥ's own scope; the first two are
    // ku sounds, the third an iṇ sound:
    //
    // The `g` arm is √bhañj's: coH kuH (8.2.30, above in this file's
    // pipeline order) has already turned the dhātu's final `j` into `g`
    // before this rule runs (Banaj + si → Banag + si), so what precedes
    // `si` here is the ku sound `g`, not yet devoiced to `k` — khari ca
    // (8.4.55) sits below this rule and does that afterwards. Banakzi
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
    // The `r` arm is juhotyādi's ṛ-roots' (3d's six, and 3d2's √ṛ): guṇa
    // gives `-ar` before the pit `si` with SHAP empty (ślu), so the sound
    // before `si` is the aṅga's own `r`, which is inside iṇ. biBar + si →
    // biBarzi, and likewise piparzi, jaGarzi, jaharzi, sasarzi, iyarzi.
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
        bars: &[],
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
            if !is_in_trigger && !matches!(prev, 'g' | 'k' | 'r') {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[next_idx].text.chars().skip(1).collect();
            p.terms[next_idx].text = format!("z{rest}");
            p.record("8.3.59", "AdeSapratyayayoH", before);
            true
        },
    },
    // 8.4.40 stoH ScunA ScuH: a stu (`s` and the t-varga) in contact with a
    // ścu (`S` and the c-varga) takes its own ścu counterpart, on either side
    // of it. Stu before ścu: atCinad → acCinad; atCfRad → acCfRad. Ścu before
    // stu: ja + jn + ati → jajYati, ajajYuH, jajYatu (√jan, once 6.4.98 has
    // elided its upadhā; slice 3f3).
    //
    // SŪTRA ORDER, immediately above 8.4.41. The two rules' TRIGGER classes
    // are disjoint — 8.4.41's is the ṣṭu class and `C` is not in it; this
    // rule's is the ścu class and no ṣṭu sound is in that — so neither rule
    // ever reads a sound the other one writes. Their TARGET classes are NOT
    // disjoint: this rule targets `s t T d D n`; 8.4.41's ṣṭu-first arm
    // targets `t T D`, its stu-before-ṭu arm `s t T d D n` (`shtutva_of`). A
    // `t` with `z` left and `C` right is applicable to both, as is a stu
    // between a ścu and a ṭu; array order would decide it. No curated root
    // has either shape, so the two rules do not contend on any reachable
    // input; that is placement, not the trigger-class argument: do not conflate.
    //
    // 8.4.41 next door scans for the same "a stu takes its neighbour's
    // class" pattern against the ṭu-varga instead of the c-varga, in both
    // directions since slice 10l, as this rule does since 3f3.
    //
    // BOTH DIRECTIONS since slice 3f3, and the converse arm carries 8.4.44
    // SAt as a guard: a stu that FOLLOWS a `S` is exempt. Until 3f3 the arm
    // was deliberately left out, because SAt was the only thing it would
    // ever meet — vidyut-prakriya credits 8.4.44 one hundred and eighteen
    // times over this corpus, every one an `S` before an `n`: aSnoti
    // (`05.0020`, 36; `09.0059`, 41) and kliSnAti (`09.0058`, 41). Shipped
    // without SAt it turns kliSnAti into *kliSYAti; shipped with SAt and no
    // witness it was code no cell could make fire. √jan's `jn` is the first
    // ścu-before-stu site SAt does not cover, so the two shipped together.
    // SAt is a guard here, not a rule of its own: a crediting 8.4.44 would
    // move those 118 prior traces for no change of form. Over the whole
    // corpus the converse arm fires on √jan and √khac alone
    // (`shcutva_off_jan_is_credited_exactly_as_before_3f3` in `panini`'s
    // trace suite).
    //
    // The forward arm is tried first at each position. The two cannot both
    // match one pair: the forward arm needs a ścu on the right, the converse
    // a stu there, and no sound is both.
    //
    // THIS CORPUS DOES present stu-immediately-before-ścu sites — three of
    // them, in √bhañj's, √añj's and √tañc's own root text (`Banj`, `anj`,
    // `tanc`: an `n` immediately before `j`/`c`) — but none of them are
    // still stu-before-ścu by the time this rule's turn comes. Both rules
    // that consume them sit ABOVE this one in the array, but only ONE of
    // the two is unconditionally sufficient on its own. 8.3.24 naS
    // cApadAntasya jhali fires unconditionally here: its trigger is `n`
    // before ANY jhal, and `j`/`c` are themselves jhal, so it needs no
    // jhal-initial ending to act — it turns the root's `n` into `M` before
    // this rule ever sees it, and `shcutva_of('M')` is `None`. 8.2.30 coH
    // kuH is NOT independently sufficient: it turns a term-final `j`/`c`
    // that is pada-final or before a jhal-initial affix or āgama into its
    // velar, but its own comment above (and
    // `coh_kuh_fires_only_word_finally_or_before_a_jhal`'s `Banjanti` case)
    // records that it DECLINES on exactly this shape when the ending is
    // vowel-initial — `Ba`/`nj`/`anti` leaves the `j` untouched, since a
    // following `a` is neither jhal nor word-final. Were 8.3.24 hypothetically
    // absent, that cell's `n`-`j` pair would reach this rule's turn intact
    // and this rule WOULD fire on it. It is 8.3.24 alone that is guaranteed
    // to have already run, in every cell, because its trigger needs nothing
    // from what follows `j`/`c`. 8.3.24's guard is `Tag::Rudhadi` or
    // `Tag::Juhotyadi` (or `Tag::Curadi`, for a root-internal `n` only),
    // gaṇa tags rather than a grammatical predicate, so
    // this coverage is contingent on those tags rather than derived from the
    // sūtra itself — a
    // documentation gap, not a latent wrongness: were 8.3.24 ever to decline
    // on one of these roots, `shcutva_of('n')` is `Some('Y')`, the same `Y`
    // that 8.3.24's `M` reaches anyway once 8.4.58 anusvArasya yayi
    // parasavarRaH parasavarnas it against the following `j`/`c`
    // (`parasavarna_of` maps the whole c-varga to `Y` too) — so even in that
    // counterfactual, THIS rule firing directly reaches the same surface
    // form 8.3.24's path would have. That is a claim about the output being
    // the same either way, not a claim that this rule fails to fire in that
    // counterfactual; keep the two distinct.
    //
    // The rules below are inert on the site this one writes. 8.4.55 Kari ca
    // reads the whole word since slice 3f2, so it does reach the tuk's `c`
    // before `C`, but `c` is already its own car and its no-op guard
    // declines (`sub == w[i - 1].2`). 8.4.53 wants a jhaś after the jhal, and
    // `C` is voiceless. 8.4.1 works on Cfnad's
    // adjacent `f` and `n`, which the tuk sits in front of rather than
    // between — so it is not an 8.4.2 intervener question either. The
    // converse arm's only outputs (√jan's and √khac's `Y`) are nasals: no jhal for 8.4.53,
    // 8.4.55 or 8.4.65, and not the dental `n` 8.4.1 retroflexes.
    //
    // 8.4.65 Jaro Jari savarRe does NOT fork the cell this rule creates,
    // and the reason is worth stating because the surface looks like it
    // should: `c` and `C` are savarṇa jhars. 8.4.65 carries 8.4.64's
    // *halaḥ* by anuvṛtti, implemented there as `!is_vowel(w[i - 1])`, and
    // the character before this rule's `c` is the aṭ's own `a`.
    // `acchinat_has_exactly_two_forms` in `panini`'s trace suite is the pin.
    //
    // The substitute IS the map: `shcutva_of` carries every stu arm and a
    // `None` from it is this rule's match test as well. That is the shape
    // 8.2.30 coH kuH had to be rewritten into once a hardcoded pair proved
    // wrong for √ric and √vic; do not reintroduce a literal here.
    Rule {
        id: "8.4.40",
        name: "stoH ScunA ScuH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let w = word_chars(p);
            for i in 0..w.len().saturating_sub(1) {
                let (left, right) = (w[i].2, w[i + 1].2);
                let target = if is_shcu(right) {
                    i
                } else if is_shcu(left) && left != 'S' {
                    i + 1
                } else {
                    continue;
                };
                let Some(sub) = shcutva_of(w[target].2) else {
                    continue;
                };
                let (term, idx, _) = w[target];
                let before = p.snapshot();
                set_char(p, term, idx, sub);
                p.record("8.4.40", "stoH ScunA ScuH", before);
                return true;
            }
            false
        },
    },
    // 8.4.41 ṣṭunā ṣṭuḥ: a dental (`s`, or a t-varga stop) retroflexes when
    // it immediately neighbours `ṣ` (z) or a ṭ-varga stop. pinaz + ti →
    // pinaz + wi → pinazwi; piMz + tas → piMzwaH; piMz + Di — in the loṭ
    // madhyama eka cell — takes the same D → Q step, and 8.4.53 below
    // carries it the rest of the way to the paradigm's finished piRqQi
    // (7b Task 6). Since slice 10l it also fires the other way round, on a
    // stu BEFORE a ṭu: √aṭṭ's `adw` → `aqw`, which 8.4.55 then makes
    // *aṭṭayati*.
    //
    // SŪTRA ORDER; LOAD-BEARING AS IMPLEMENTED. It sits above 8.4.53 because
    // that is where vidyut-prakriya's data/sutrapatha.tsv places it — but
    // since 7b Task 5 gave `jashtva_of` a `z → q` arm, the two rules no longer
    // touch disjoint sounds on this junction: piMz + Di's `z` is now exactly
    // what jaśtva would take if it saw it first. With 8.4.41 above, it fires
    // on the `z`/`D` pair before 8.4.53 runs, retroflexing D → Q; 8.4.53
    // then voices that same `z` to `q` before the new `Q`, giving piMqQi.
    // Run 8.4.53 first instead and it would read piMz + Di's `z` as the
    // jaśtva target — jashtva_of('z') is no longer a no-op — and rewrite it
    // to `q` before 8.4.41 ever ran; with 8.4.41's trigger `z`-only, as it
    // was through 7b Task 6 (before rudhādi 7e's widening below), that `z`
    // is gone by the time 8.4.41 runs and it has nothing left to fire on,
    // giving piMqDi instead.
    //
    // The two orders failed to converge for an implementation reason, not
    // a sūtra one. BEFORE 7b Task 6, two separate narrowings held them
    // apart: THIS rule's trigger set was `z` only (see the TRIGGER
    // paragraph below — titled NARROW GUARD until rudhādi 7e renamed it),
    // not the full ṭ-varga ṣṭunā ṣṭuḥ names (`w W q Q R`); and 8.4.53's
    // guard used to check for a literal penult `D`, not "any jhaś." 7b
    // Task 6 generalised 8.4.53 from literal-`D` to jhal-before-jhaś (Q
    // qualifies), which removed 8.4.53's narrowing — the AS-IMPLEMENTED
    // order (8.4.41 above 8.4.53) now converges correctly to piMqQi, as
    // traced above and pinned by `pish_lot_madhyama_eka_is_pinddhi`'s
    // piRqQi. THIS rule's `z`-only trigger then became the ONE narrowing
    // left standing: reorder the two rules (8.4.53 above 8.4.41) and it
    // still stalled at piMqDi, restorable only by also widening this
    // trigger to include `q` — the full `w W q Q R` — so a q-triggered
    // 8.4.41 could still retroflex D → Q afterward. rudhādi 7e made
    // exactly that widening (see TRIGGER below). Do not reorder these two
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
    // TRIGGER: the full ṣṭu class (`is_shtu`), widened from a bare `z`
    // literal in rudhādi 7e. That literal was, as the paragraph above says,
    // the ONE narrowing left holding this rule and 8.4.53 apart under a
    // reordering — so widening it does not weaken the pair, it removes the
    // pair's last order-dependence. 8.2.31 ho ḍhaḥ is what made the wider
    // class reachable: it produces a `Q` that must retroflex 8.2.40's `D`
    // (tfneQ + Di → tfneQ + Qi → 8.3.13 → tfneQi). Verified inert against
    // the pre-7e 2592-cell corpus by a byte-for-byte dump diff before any
    // new root was curated.
    //
    // The CORRESPONDENCE side stays narrow, and deliberately: only t/T/D
    // have a witness. √tṛh reaches `D` → `Q` and nothing wider, so d/n/s
    // are still absent. Widen that half the moment a junction reaches it —
    // it is a separate claim from the trigger's, with separate evidence.
    //
    // The neighbour just above, 8.4.40, took the opposite route: a full
    // substitution table (`shcutva_of`), with `shcutva_of_stu_all_arms`
    // pinning all six stu arms, five of them unwitnessed by the corpus.
    // Both choices are defensible here, not a repeat of 8.2.30's old
    // defect — that bug was two independent literals (a match testing `j`,
    // a substitute writing `'g'`) that could drift apart, and did. This
    // rule fuses match and substitute into one `match` expression, so
    // CORRESPONDENCE staying narrow can go stale but cannot go
    // inconsistent the way 8.2.30 did. The stu-before-ṭu arm (slice 10l)
    // took 8.4.40's route instead: its target is the sound BEFORE the ṭu,
    // a separate claim, and it reads the full table (`shtutva_of`), with
    // `shtutva_of_all_arms` pinning the five arms √aṭṭ's `d` does not reach.
    Rule {
        id: "8.4.41",
        name: "zwunA zwuH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let w = word_chars(p);
            for i in 1..w.len() {
                // The sūtra's other order: a stu before a ṭu takes its ṣṭu
                // (√aṭṭ: `dw` → `qw`). The trigger is the ṭ-varga only, not
                // `z`: 8.4.43 toḥ ṣi keeps a tu before `z`.
                if matches!(w[i].2, 'w' | 'W' | 'q' | 'Q' | 'R')
                    && let Some(sub) = shtutva_of(w[i - 1].2)
                {
                    let (term, idx, _) = w[i - 1];
                    let before = p.snapshot();
                    set_char(p, term, idx, sub);
                    p.record("8.4.41", "zwunA zwuH", before);
                    return true;
                }
                if !is_shtu(w[i - 1].2) {
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
    // 8.3.13 ḍho ḍhe lopaḥ: a `Q` is elided before a `Q`.
    // tfReQ + Qi → tfRe + Qi; tfMQ + QaH → tfM + QaH.
    //
    // OUT OF SŪTRA ORDER, immediately below 8.4.41, and this is
    // load-bearing twice over.
    //
    // First, the condition. The SECOND ḍh does not exist until ṣṭutva has
    // run: 8.2.31 makes the stem-final `Q`, 8.2.40 makes the ending's `t`
    // into `D`, and only 8.4.41 above turns that `D` into the `Q` this rule
    // needs. Placed in numeric order it would see tfneQ + Di, decline, and
    // the cell would surface *tfReQQi. The file already orders by operation
    // where the derivation demands it — 8.2.73 sits below 8.2.75, and
    // 8.4.56 sits last, below 8.4.65.
    //
    // Second, the fork count. √tṛh reaches loṭ madhyama eka in the same
    // kfnt + Di shape that makes every other stop-final rudhādi root a
    // SIX-form cell (8.4.53 voices, 8.4.65 optionally elides, 7.1.35 and
    // 8.4.56 multiply). √tṛh's is a three-former, because this rule
    // obligatorily eats the very ḍh 8.4.65 would have forked on. Move this
    // rule below 8.4.65 and the cell silently grows to six forms —
    // `trnaddhi_trace_has_8_3_13_and_no_8_4_65` in `panini`'s trace suite
    // is the pin, and the ALTERNATES count is the second alarm.
    //
    // 6.3.111 ḍhralope pūrvasya dīrgho'ṇaḥ does NOT follow this elision
    // here, and its absence is deliberate rather than an omission: it
    // lengthens a preceding **aṇ**, and in every √tṛh cell the sound before
    // the elided ḍh is `e` (tfRe + Qi) or `M` (tfM + QaH), neither of which
    // is one. vidyut-prakriya's traces do not emit it either. Implement it
    // when a root presents a short a/i/u there — this comment is the note
    // that says why there is nothing to implement yet.
    Rule {
        id: "8.3.13",
        name: "Qo Qe lopaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let w = word_chars(p);
            let Some(i) = (1..w.len()).find(|&i| w[i - 1].2 == 'Q' && w[i].2 == 'Q') else {
                return false;
            };
            // EQUIVALENT MUTANT, documented on purpose (rudhādi 7e
            // mutation campaign, 8.3.13's own guard, `replace - with /`
            // i.e. `w[i - 1]` -> `w[i]`, eliding the second ḍh instead of
            // the first): both `Q`s here are the identical character, so
            // removing either one concatenates to the same surface string
            // — the golden suite's full corpus, its ALTERNATES, and
            // its traces cannot and will not distinguish which of the two
            // was elided.
            //
            // This engine has a dual representation — per-term text plus
            // the flattened `word_chars`/`p.text()` view — and one later
            // tripādī rule DOES read term structure directly rather than
            // going through the flattened view: 8.4.56 (`vA'vasAne`, the
            // pipeline's last rule) does `p.terms.rposition(|t|
            // !t.text.is_empty())` and pops/pushes on that specific term.
            // (8.4.55 `Kari ca` did too, reading `ENDING`'s first char and
            // the last non-empty term before it, until slice 3f2 moved it to
            // the flattened view. It was unaffected either way: `is_khar('Q')`
            // is always false, since ḍh is a voiced aspirate and khar is
            // voiceless.) 8.4.56 was checked rather than assumed safe: it is
            // unaffected because its `rposition` search for the last
            // non-empty term is self-correcting: it always lands on
            // whichever term physically holds the word's trailing
            // character, the same character `p.text()` already read one
            // line above, so mutant and non-mutant land on the same `sub`
            // regardless of which term is credited with holding it. That
            // second case is a currently LATENT non-divergence, not a
            // structural one: today's golden suite's actual `ENDING`
            // values (`"Qi"`, `"QaH"`) are both two-plus characters, so
            // `ENDING` never empties out in practice — a hypothetical
            // single-character `ENDING` fully elided by the mutant would
            // still need re-checking against this argument, not assumed
            // to inherit it.
            //
            // `w[i - 1]` is still the only correct choice: the sūtra is
            // ḍho ḍhe lopaḥ, "of ḍh, before ḍh, elision" — the FIRST ḍh is
            // the one the grammar elides, and `w[i]` would elide the
            // second, a different (wrong) analysis that happens to be
            // unobservable at the surface. Do not add a test to try to
            // kill this mutant; do not treat a surviving mutant here as a
            // missing test without first checking whether it is this
            // exact index-choice mutation, and without re-running the
            // 8.4.56 argument above rather than assuming it still
            // applies. (An earlier task-9 planning note in this slice
            // predicted this survivor but reasoned "eliding the second
            // gives tfReQ" — that arithmetic was wrong: eliding the second
            // Q gives the same tfReQi as eliding the first, which is
            // exactly why the mutant survives.)
            let (term, idx, _) = w[i - 1];
            let before = p.snapshot();
            remove_char(p, term, idx);
            p.record("8.3.13", "Qo Qe lopaH", before);
            true
        },
    },
    // 8.4.53 jhalāṁ jaś jhaśi: a jhal becomes its jaś before a jhaś (a
    // voiced aspirate), anywhere the two sit adjacent in the word — not
    // only at the word's own end. kfnt + Di → kfnd + Di → kfndDi.
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
    //
    // GENERALISED (7b Task 6). The guard used to read "the word ends in `i`
    // and the penult is `D`" — the only shape 7a's two witnesses (kfndDi,
    // hinDi) ever reached it through, so it was never written wider than
    // that. It stops being true here: √piṣ's piRqQi conditions this rule
    // on a `Q` — 8.4.41 just above has already retroflexed 6.4.101's `Di`
    // to `Qi` before this rule ever runs — and √indh (7b Task 7) conditions
    // it on a `D` that sits mid-word, in `De`, `Da`, `DAm`, `Dve`, `DAH`
    // and `Dvam`, never at the word's own end. Four of those six (`De`,
    // `Da`, `DAm`, `DAH`) are newly created by the upcoming 8.2.40 jhaṣas
    // tathor dho'dhaḥ (t/th → D after a stem's jhaṣ); `Dve`/`Dvam` are not
    // — see NOT NARROWED below for why they still need this rule. The
    // guard now reads the sūtra's actual condition instead: `is_jhash`
    // locates the jhaś, `jashtva_of` on the sound immediately before it
    // supplies the substitute, checked at every adjacent pair
    // `word_chars` reports.
    //
    // NOT NARROWED THE WAY 8.4.41's CORRESPONDENCE side above still is.
    // 8.4.41's TRIGGER side was deliberately `z`-only too, pending a later
    // root, until rudhādi 7e widened it to the full ṣṭu class (see
    // TRIGGER above); its correspondence side (t/T/D) remains narrow the
    // same way. This rule instead implements the full jhalāṁ jaś jhaśi
    // condition, with no positional restriction and no restriction on
    // which jhal or which jhaś. What keeps it from over-firing is
    // upstream, not in this guard, in two separate ways for the two kinds
    // of jhaś-initial affix in this grammar:
    //
    // 8.2.40 (7b Task 7) is the only NEW source of a D-initial ending —
    // besides the pre-existing 6.4.101 her dhiḥ — and it requires a jhaṣ
    // already abutting the ending. Only √indh and √dhā present one (see
    // 8.2.40's comment), and 8.2.40's *adhaḥ* excludes √dhā, so a fresh
    // D-initial ending is still reachable only through √indh.
    //
    // `Dve`/`Dvam` (and the iṭ-augmented `IDvam`) are a SEPARATE case: no
    // rule creates their `D`. `Dvam` is the raw ātmanepada
    // madhyama-bahu pratyaya straight out of `panini-data`'s `tin_ending`
    // table, and 3.4.79 wita AtmanepadAnAM wer e — a general ṭi
    // substitution with no jhaṣ condition of its own — turns it to `Dve`
    // for laṭ/loṭ. What keeps THIS rule from over-firing there is a fact
    // about the STEMS that reach them, not the endings: every Dve/Dvam
    // cell pinned in this suite puts either a vowel (laBaDve, ADve,
    // vaDve, AsIDvam, laBaDvam) or an already-jaś `d` (KindDve, √dā's
    // dadDve) immediately before the `D` — never an untreated jhal — so
    // this rule either has nothing to see or the no-op guard declines it.
    // √indh's own indDve/indDvam (7b Task 7) and √dhā's DadDve, aDadDvam
    // and DadDvam (slice 3c, the root's `D` bared by 6.4.112) are the
    // cells where a stem-final jhaṣ genuinely meets this native `D`, and
    // that is exactly where this rule is supposed to fire.
    //
    // One real scope limit remains, inherited from the engine rather than
    // written into this guard: `apply` runs at most once per branch per
    // pipeline pass (see `controller::run_pipeline`), so this loop returns
    // on the FIRST qualifying pair it finds and never revisits the word
    // for a second one. No form in this suite needs two — verify before
    // relying on it for a future root with more than one jhaś-initial
    // affix boundary.
    Rule {
        id: "8.4.53",
        name: "JalAM jaS JaSi",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let w = word_chars(p);
            for i in 1..w.len() {
                if !is_jhash(w[i].2) {
                    continue;
                }
                let Some(jash) = jashtva_of(w[i - 1].2) else {
                    continue;
                };
                // No-op guard, as 8.4.55 below takes for the identical
                // reason: a target that is already its own jaś (√ad's
                // `d`, adDi) must not record a vacuous step.
                if jash == w[i - 1].2 {
                    continue;
                }
                let (term, idx, _) = w[i - 1];
                let before = p.snapshot();
                set_char(p, term, idx, jash);
                p.record("8.4.53", "JalAM jaS JaSi", before);
                return true;
            }
            false
        },
    },
    // 8.4.54 abhyāse car ca: a jhal in the abhyāsa becomes its car — and its
    // jaś, by 8.4.53's anuvṛtti (jhalāṁ jaś) — i.e. the abhyāsa loses its
    // aspiration: Ju → ju (juhoti; the `J` 7.4.62 wrote for h), and in
    // slice 3b Bi → bi (bibheti) and Ji → ji (jihreti; the `J` 7.4.62 wrote
    // for h here too). Both start from a SHORT vowel by the time this rule
    // sees them — 7.4.59 shortened the abhyāsa long before the tripādī —
    // so this rule never sees `BI` or `hI`; see the 7.4.60/7.4.59/7.4.62/
    // 8.4.54 chain on hrI pinned in tests/trace/juhotyadi.rs. Reads the
    // ABHYASA slot directly and whole:
    // *abhyāse* is the slot (non-empty exactly when 6.1.10 filled it — see
    // 7.4.62 in abhyasa.rs), and every aspirate in it is deaspirated in one
    // step. The no-op guard is 8.4.53's: √ki's abhyāsa `ci` is already car
    // and the rule must record nothing there — vidyut-prakriya credits
    // 8.4.54 on √hu's 42 forms, on √bhī's and √hrī's own forms too, on
    // √hā's and √dhā's (slice 3c: Ja → ja, Da → da — a `da` that 8.2.38,
    // just below, re-aspirates before t/th/s/dhv), and on none of √ki's,
    // √dā's or √mā's.
    Rule {
        id: "8.4.54",
        name: "aByAse car ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let s: Vec<char> = p.terms[ABHYASA].text.chars().collect();
            let t: Vec<char> = s.iter().map(|&c| deaspirate_of(c).unwrap_or(c)).collect();
            if t == s {
                return false;
            }
            let before = p.snapshot();
            p.terms[ABHYASA].text = t.into_iter().collect();
            p.record("8.4.54", "aByAse car ca", before);
            true
        },
    },
    // 8.2.38 dadhas tathoś ca: the reduplicated √dhā — *dadh*, its ā gone —
    // takes bhaṣ for the abhyāsa's baś before `t`, `th`, `s` or `dhv`: the
    // abhyāsa's `d` becomes `D`. da + D + tas → Da + D + tas (DattaH, once
    // 8.4.55 devoices the root's `D`); Datse; DadDve.
    //
    // OUT OF SŪTRA ORDER, after 8.4.54 and before 8.4.55. The sūtra names
    // *dadh* — the stem after 8.4.54 has already deaspirated the abhyāsa to
    // `da` — so it must see that output. In sūtra position it would find
    // `Da`, decline, and 8.4.54 would then produce *dattaH / *dadDve.
    // vidyut-prakriya orders it the same way (8.4.54 < 8.2.38 < 8.4.55), and
    // the DattaH and DadDve trace pins hold it here.
    //
    // Only the abhyāsa changes. The root's own `D` is left to 8.4.53 (before
    // `Dv`: DadDve) and 8.4.55 (before `t`/`s`: DattaH, Datse), so the
    // credited rules match vidyut's step for step.
    //
    // KEYED BY ROW NUMBER, `03.0011 quDA\Y`: √dā (`03.0010`) has the
    // identical shape — da + d + tas — and takes no 8.2.38 (dattaH). The
    // single-consonant aṅga test is *dadh*'s lost ā: daDAti keeps its ā
    // before the pit `ti` and must not become *DaDAti.
    Rule {
        id: "8.2.38",
        name: "daDastaToSca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if p.ctx.dhatupatha != "03.0011" {
                return false;
            }
            let Some(e) = p.terms.get(ENDING).map(|t| t.text.as_str()) else {
                return false;
            };
            if !(e.starts_with(['t', 'T', 's']) || e.starts_with("Dv")) {
                return false;
            }
            if p.terms[ANGA].text.chars().count() != 1 {
                return false;
            }
            let Some(rest) = p.terms[ABHYASA].text.strip_prefix('d') else {
                return false;
            };
            let t = format!("D{rest}");
            let before = p.snapshot();
            p.terms[ABHYASA].text = t;
            p.record("8.2.38", "daDastaToSca", before);
            true
        },
    },
    // 8.4.55 khari ca (cartva): a jhal immediately before a khar becomes its
    // car (voiceless unaspirated), anywhere the two sit adjacent in the word.
    // √ad's d before ti/tas/si/tha → t: atti, attaH, atsi, atTa; √bhas's
    // aṅga-internal `Bs` → `ps` before a vowel or `y`: bapsati, bapsyAt
    // (slice 3f2). General, reused by every later gaṇa/subanta slice. No
    // longer the pipeline's last rule — 8.4.65 and 8.4.56 both follow it now
    // — but still ordered after every other 8.3/8.4 rule that precedes it.
    //
    // READS THE WHOLE WORD since slice 3f2, through `word_chars`, as 8.2.40
    // and 8.4.53 do; the sūtra has no positional condition. Until then it
    // read only the aṅga/ending junction: the last non-empty term's final
    // char before `ENDING` against `ENDING`'s own first sound. That reading
    // was itself a fix (7a Task 7: rudhādi's śnam-split puts the root's own
    // tail in SHAP, and an ANGA-only read gave Kindte for Kintte), and the
    // whole-word scan subsumes it — every junction pair is a pair of
    // adjacent chars in the flattened word. What the junction reading could
    // not see was a pair inside one term: √bhas's `Bs`, once 6.4.100 has
    // elided the `a` between them. Widening it changed no prior cell's trace
    // (the 3f2 spec's corpus-wide diff), and
    // `khari_ca_off_bhas_is_credited_exactly_as_before_3f2` in `panini`'s
    // trace suite holds the count.
    //
    // The scan takes the FIRST pair in the word. Each rule fires once per
    // branch, so a word with two such pairs would devoice only the first;
    // no curated cell has two.
    Rule {
        id: "8.4.55",
        name: "Kari ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let w = word_chars(p);
            for i in 1..w.len() {
                if !is_khar(w[i].2) {
                    continue;
                }
                // `cartva_of` is defined only on the stops, so it is the jhal
                // test as well: a sibilant is already its own car, and `h`
                // has none.
                let Some(sub) = cartva_of(w[i - 1].2) else {
                    continue;
                };
                // No-op guard: a stop that is already its own car records
                // nothing.
                if sub == w[i - 1].2 {
                    continue;
                }
                let (term, idx, _) = w[i - 1];
                let before = p.snapshot();
                set_char(p, term, idx, sub);
                p.record("8.4.55", "Kari ca", before);
                return true;
            }
            false
        },
    },
    // 8.4.39 kṣubhnādiṣu ca: no ṇatva in the kṣubhnādi. √tṛp's `f` would
    // otherwise reach śnu's `n` across the pu-varga `p` by 8.4.2 (*tfpRoti*
    // for tfpnoti). Keyed by row (`super::samjna::KSUBHNADI`), as vidyut-
    // prakriya keys it by upadeśa: curādi's `10.0351` and `10.0355` are
    // `tfpa~`, stored `tfp`, too. vidyut also requires śnu after the dhātu:
    // the next non-empty term after ANGA is śnu, read by identity
    // (`Tag::Snu`, added by 3.1.73), not by text: by the tripādī śnu's text
    // is `nu`, `no`, `nuv` (6.4.77) or `nav` (guṇa and 6.1.78 before a
    // vowel-initial pit ending). Kryādi's śnā will get an analogous tag when
    // 9d curates `09.0055 kzuBa~`.
    //
    // Changes no text: it records and bars 8.4.1 and 8.4.2, as 6.4.117 ā ca
    // hau bars the rules that would change its `A`. Placed just above 8.4.1,
    // the first rule it bars.
    Rule {
        id: "8.4.39",
        name: "kzuBnAdizu ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &["8.4.1", "8.4.2"],
        apply: |p| {
            if !KSUBHNADI.contains(&p.ctx.dhatupatha) {
                return false;
            }
            let Some(next) = p.terms[ANGA + 1..].iter().find(|t| !t.text.is_empty()) else {
                return false;
            };
            if !next.has(Tag::Snu) {
                return false;
            }
            let before = p.snapshot();
            p.record("8.4.39", "kzuBnAdizu ca", before);
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
        bars: &[],
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
        bars: &[],
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
    // rudhādi and juhotyādi: 8.3.24 above is gaṇa-guarded, so BAzante's `n` is still an
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
    // 8.3.24 past rudhādi and juhotyādi. Slice 10e's curādi widening does
    // not: it reaches only a curādi root's own `n`, so the 7.1.3 `n` of
    // every non-rudhādi, non-juhotyādi root still needs the fold.
    //
    // The `yayi` / parasavarṇa test lives INSIDE the search, not after it:
    // the rule finds the first anusvāra that actually HAS a parasavarṇa
    // (i.e. is genuinely followed by a yay), rather than the first anusvāra
    // in the word full stop, so a non-applicable anusvāra earlier in the
    // word can never hide a later, applicable one. No cell in this suite
    // has more than one anusvāra candidate to distinguish, so this is
    // hardening against a shape no witness here exercises, not a fix for
    // one observed.
    Rule {
        id: "8.4.58",
        name: "anusvArasya yayi parasavarRaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let w = word_chars(p);
            let found = w.iter().enumerate().find_map(|(i, (_, _, c))| {
                if *c != 'M' {
                    return None;
                }
                let next = w.get(i + 1)?.2;
                parasavarna_of(next).map(|nasal| (i, nasal))
            });
            let Some((pos, nasal)) = found else {
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
    // 8.4.56 — which 7a Task 9's `kfntAt` trace pin in
    // `crates/panini/tests/trace/` asserts directly
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
        bars: &[],
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
    // its car, continuing khari ca's operation. After 8.2.39 the reachable
    // jhal-finals are `d` and, since 8.2.39's widening, `g` too (√ric's and
    // √vic's laṅ prathama/madhyama eka, whose `c`-final stems reach 8.2.30
    // then 8.2.39): in practice this restores the `t` or `k` that 8.2.39
    // voiced — `aBavat` from `aBavad`, `ariRak` from `ariRag` — which is
    // exactly the relationship the sūtras state, and why those are now
    // alternates rather than the pinned forms.
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
    // 8.2.39 now obligatorily turns every pada-final jhal it reaches into
    // its jaś, and every jaś's car is a different sound (`cartva_of` has no
    // fixed points among `g`, `j`, `q`, `d`, `b`), so `cartva_of(last)`
    // never yields its argument back here.
    //
    // That was NOT true before this task's widening, and the inversion is
    // worth recording: with the old three-literal 8.2.39 guard, a
    // word-final `k` (√ric's, √vic's) passed straight through un-voiced,
    // and `cartva_of('k')` is `Some('k')` — a genuine fixed point — so this
    // rule fired vacuously on it (`ariRak` -> `ariRak`), which is exactly
    // the "more branches than distinct forms" symptom the diagnosis found.
    // Widening 8.2.39 to read `jashtva_of` on both sides is what finally
    // makes this paragraph's claim true, by construction: everything
    // 8.2.39 now writes is a jaś, and no jaś is its own car. Widen this
    // guard with a real check, not a speculative one, the moment some
    // future rule produces a jaś-final `s` or similar exception 8.2.39
    // doesn't cover.
    Rule {
        id: "8.4.56",
        name: "vA'vasAne",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &[],
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
    use crate::context::Context;
    use crate::prakriya::Prakriya;
    use crate::term::Term;
    use crate::tinanta::rules;
    use crate::tinanta::terms::with_slots;
    // `form_g` lives in `derivation_tests.rs`; `mod.rs` re-exports it, so
    // this import stays on the stable `crate::tinanta::form_g` path.
    use crate::tinanta::derivation_tests::sole;
    use crate::tinanta::derive;
    use crate::tinanta::form_g;
    use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};

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
            terms: with_slots(vec![Term::new("iv"), Term::new("ta")]),
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
            terms: with_slots(vec![Term::new("aBiur"), Term::new("ta")]),
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "8.2.77").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "aBiUr");
    }

    #[test]
    fn hali_ca_declines_for_kur_per_8_2_79() {
        // 8.2.79 na BakurCurAm carves kur back out of 8.2.77's shape --
        // kurvanti, not kUrvanti. `akur` (a prefixed-aṅga tolerance, not
        // laṅ's own shape) must decline too, since the guard reads the
        // tail of the text.
        for anga in ["kur", "akur"] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new(anga), Term::new("v"), Term::new("anti")]),
                log: vec![],
                ..Default::default()
            };
            let rule = rules().find(|r| r.id == "8.2.77").unwrap();
            assert!(!(rule.apply)(&mut p), "{anga}");
            assert_eq!(p.terms[ANGA].text, anga, "{anga}");
        }
    }

    #[test]
    fn hali_ca_still_lengthens_the_divyati_shaped_root() {
        // Positive control alongside the 8.2.79 carve-out above: div
        // itself -- the rule's original target -- is unaffected by the
        // new kur-specific guard.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("div"), Term::new("ya"), Term::new("ti")]),
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "8.2.77").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "dIv");
    }

    #[test]
    fn hali_ca_reads_the_ending_when_slu_leaves_shap_empty() {
        // √pṝ (03.0004) after 7.1.102: pur + "" + tas. With śap ślu'd the
        // ending is what meets the aṅga, so its `t` is the hal: pUr
        // (pipUrtaH). Before a vowel-initial ending the rule declines
        // (pipurati).
        let rule = rules().find(|r| r.id == "8.2.77").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("pur"), Term::new(""), Term::new("tas")]),
            log: vec![],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "pUr");
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("pur"), Term::new(""), Term::new("ati")]),
            log: vec![],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "pur");
    }

    #[test]
    fn hali_ca_reads_a_live_vikarana_not_the_ending() {
        // A non-empty, vowel-initial SHAP in front of a consonant-initial
        // ending: the aṅga meets the vikaraṇa's vowel and must not lengthen.
        // Hand-built — no curated r/v-final aṅga carries śap — and it is what
        // kills a mutant that reads the ending whenever SHAP is present.
        let rule = rules().find(|r| r.id == "8.2.77").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("div"), Term::new("a"), Term::new("ti")]),
            log: vec![],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "div");
    }

    #[test]
    fn krpo_ro_lah_turns_krps_r_to_l_on_its_row_only() {
        let rule = rules().find(|r| r.id == "8.2.18").unwrap();
        let krp = |row: &'static str, anga: &str| {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new(anga), Term::new("a"), Term::new("ti")]),
                ..Default::default()
            };
            p.ctx.dhatupatha = row;
            p
        };
        // `karp` (7.3.86's guṇa before ṇic) → `kalp`: *kalpayati*.
        let mut p = krp("10.0278", "karpay");
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kalpay");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["8.2.18"]);
        // The `f` arm: an unguṇated `kfp` → `kxp`.
        let mut p = krp("10.0278", "kfp");
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kxp");
        // `10.0408 kfpa` is another root, and a row with no number is none.
        for row in ["10.0408", ""] {
            let mut p = krp(row, "karpay");
            assert!(!(rule.apply)(&mut p), "{row}");
            assert_eq!(p.terms[ANGA].text, "karpay");
            assert!(p.log.is_empty(), "{row}");
        }
        // On its row with no `r` or `f` left: nothing to record.
        let mut p = krp("10.0278", "kalpay");
        assert!(!(rule.apply)(&mut p));
        assert!(p.log.is_empty());
        assert!(!rule.vikalpa);
    }

    #[test]
    fn upadhayam_ca_lengthens_the_ik_before_a_roots_r_or_v_upadha() {
        let rule = rules().find(|r| r.id == "8.2.78").unwrap();
        let with_anga = |anga: &str, nijanta: bool| {
            let mut t = Term::new(anga);
            if nijanta {
                t.add(Tag::Nijanta);
            }
            Prakriya {
                terms: with_slots(vec![t, Term::new("a"), Term::new("ti")]),
                ..Default::default()
            }
        };
        // A ṇijanta aṅga is read through ṇic's `ay`: each short ik, before
        // an `r` or a `v` upadhā.
        for (anga, want) in [
            ("urjay", "Urjay"),
            ("curRay", "cUrRay"),
            ("gurday", "gUrday"),
            ("kirtay", "kIrtay"),
            ("pfrkay", "pFrkay"),
            ("kxvpay", "kXvpay"),
            ("divkay", "dIvkay"),
            // A five-letter root, where the upadhā's index `n - 2` is not
            // `n / 2`.
            ("stirpay", "stIrpay"),
        ] {
            let mut p = with_anga(anga, true);
            assert!((rule.apply)(&mut p), "{anga}");
            assert_eq!(p.terms[ANGA].text, want);
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["8.2.78"]);
        }
        // Any other aṅga is the root's own text.
        let mut p = with_anga("urj", false);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Urj");
        // Decline: laṅ's `Orjay` (6.1.90 left no ik), an `a` before the `r`
        // (arjay), a vowel-final root (uri), a two-letter root (rj), and an
        // `r` that is the root's last sound (kur, 8.2.77's shape). A
        // non-ṇijanta `urjay` is read whole and ends in `ay`.
        for (anga, nijanta) in [
            ("Orjay", true),
            ("arjay", true),
            ("uri", false),
            ("rjay", true),
            ("kur", false),
            ("urjay", false),
        ] {
            let mut p = with_anga(anga, nijanta);
            assert!(!(rule.apply)(&mut p), "{anga}");
            assert_eq!(p.terms[ANGA].text, anga);
            assert!(p.log.is_empty(), "{anga}");
        }
        assert!(!rule.vikalpa);
    }

    #[test]
    fn shtutva_retroflexes_a_stu_before_a_tu_too() {
        let rule = rules().find(|r| r.id == "8.4.41").unwrap();
        let word = |anga: &str| Prakriya {
            terms: with_slots(vec![Term::new(anga), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        // √aṭṭ: `adway` → `aqway` (8.4.55 then makes the `q` a `w`).
        let mut p = word("adway");
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "aqwayati");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["8.4.41"]);
        // Every ṭu after it triggers, and every stu before one takes its ṣṭu.
        for (anga, want) in [
            ("adW", "aqW"),
            ("adq", "aqq"),
            ("adQ", "aqQ"),
            ("adR", "aqR"),
            ("atw", "aww"),
            ("aTw", "aWw"),
            ("aDw", "aQw"),
            ("anw", "aRw"),
            ("asw", "azw"),
        ] {
            let mut p = word(anga);
            assert!((rule.apply)(&mut p), "{anga}");
            assert_eq!(p.terms[ANGA].text, want);
        }
        // 8.4.43 toḥ ṣi: a tu before `z` stays. A sound between them, or a
        // non-stu before the ṭu, and nothing happens.
        for anga in ["adz", "adaw", "akw"] {
            let mut p = word(anga);
            assert!(!(rule.apply)(&mut p), "{anga}");
            assert_eq!(p.terms[ANGA].text, anga);
        }
    }

    #[test]
    fn shatva_declines_for_every_pre_existing_junction() {
        // Each of these pins one boundary of 8.3.59's guard, and each is a
        // form the suite already ships — so a mutant that widens the guard
        // breaks a golden, not just this test.
        //
        // aṅga-final `A` is excluded (a/ā are not iṇ):
        assert_eq!(
            form_g("02.0011", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
            "Assva"
        );
        // aṅga-final `s` is not a vowel at all:
        assert_eq!(
            form_g("02.0013", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
            "vasse"
        );
        // Thematic path: the ending is preceded by the śap's `a`, excluded.
        assert_eq!(
            form_g("01.1130", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
            "laBasva"
        );
        // And a non-s-initial ending after √śī's `e` is left alone — the
        // clause an `||` → `&&` mutant would drop.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Se"), Term::new(""), Term::new("te")]),
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
            terms: with_slots(vec![Term::new("a"), Term::new(""), Term::new("se")]),
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
            terms: with_slots(vec![Term::new("pi"), Term::new("naK"), Term::new("si")]),
            log: vec![],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "si");
    }

    #[test]
    fn shatva_fires_after_the_r_of_a_guned_ri_root() {
        // The `r` arm is the 3d ṛ-roots': guṇa gives Bar, and the pit `si`
        // follows an empty SHAP (ślu), so the sound before it is the aṅga's
        // own `r`, which is in iṇ. biBar + si -> biBarzi.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Bar"), Term::new(""), Term::new("si")]),
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "8.3.59").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "zi");
        assert_eq!(p.log.last().unwrap().sutra, "8.3.59");
    }

    #[test]
    fn shatva_reads_the_sound_before_the_affix_not_the_anga() {
        // vf + nI + sva: the iN trigger is SnA's I, not the anga's f. The
        // pre-kryadi guard read ANGA and would have declined here.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("vf"), Term::new("nI"), Term::new("sva")]),
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
            terms: with_slots(vec![Term::new("pi"), Term::new("nak"), Term::new("si")]),
            log: vec![],
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "pinakzi");
        // And the thematic case still declines on the vikaraNa's `a`, which
        // is what keeps laBasva intact.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("laB"), Term::new("a"), Term::new("sva")]),
            log: vec![],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "laBasva");
    }

    #[test]
    fn shatva_affix_search_skips_the_anga_itself() {
        // Pins that the s-initial affix search starts AFTER the aGga
        // (`.skip(ANGA + 1)`), not AT it (`.skip(ANGA * 1)` == `.skip(2)`,
        // since ANGA == 2). With the mutant the search would start at the
        // aṅga itself instead of skipping the two leading `AGAMA`/`ABHYASA`
        // slots plus the aṅga. The corpus alone can't catch a `+` -> `*`
        // mutant here: its only s-initial roots (smf, sev) both decline
        // 8.3.59 on other grounds, so both versions of the search agree on
        // every golden and every known-nonform. An s-initial aGga is
        // needed to force the two versions apart.
        //
        // sI + nI + sva: with skip(ANGA + 1) == skip(3), the search starts
        // past the aGga and finds `sva` at index 4; the preceding non-empty
        // term's last char is SnA's `I` (a non-a/A vowel), so 8.3.59 fires:
        // sInIzva.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("sI"), Term::new("nI"), Term::new("sva")]),
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "8.3.59").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "sInIzva");
        // With the `ANGA * 1` mutant (skip(2)), the search would instead
        // match the aGga `sI` itself at index 2 (it too starts with `s`).
        // The two leading `AGAMA`/`ABHYASA` slots before it are both empty,
        // so no preceding non-empty term is found to read a trigger sound
        // from, and the rule would wrongly decline and `sva` would surface
        // unchanged.
    }

    fn natva_prakriya(anga: &str, vikarana: &str, ending: &str) -> Prakriya {
        Prakriya {
            terms: with_slots(vec![
                Term::new(anga),
                Term::new(vikarana),
                Term::new(ending),
            ]),
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
    fn natva_fires_across_nums_anusvara_under_8_4_2() {
        // kzaMp + Ani (√kṣamp's loṭ uttama eka, 10.0112): z, the aw vowel a,
        // num's anusvāra M (8.3.24 has run), the pu p, the aw vowel A, then n.
        // 8.4.2 names num; read as M, it carries the scan to the z.
        let mut p = natva_prakriya("kzaMp", "", "Ani");
        let r841 = rules().find(|r| r.id == "8.4.1").unwrap();
        assert!(!(r841.apply)(&mut p), "8.4.1 must not fire non-adjacently");
        let r842 = rules().find(|r| r.id == "8.4.2").unwrap();
        assert!((r842.apply)(&mut p));
        assert_eq!(p.text(), "kzaMpARi");
        // Contrast, the converse of the above: the same root with num still
        // written as `n` (not yet 8.3.24's M). `n` is no intervener, so the
        // scan from the ending's `n` stops at it and 8.4.2 declines; the
        // root's own `n` is spared by the jhal guard (`p` follows it). Only
        // M carries the scan. (Removing `'M'` from the intervener set fails
        // the assertion above, at `(r842.apply)(&mut p)`; this one pins that
        // `n` itself never became an intervener.)
        let mut p = natva_prakriya("kzanp", "", "Ani");
        assert!(!(r842.apply)(&mut p), "n is no intervener: the scan stops");
        assert_eq!(p.text(), "kzanpAni");
    }

    #[test]
    fn natva_declines_word_finally_per_8_4_37() {
        // asmaran: r, the aw vowel a, then a WORD-FINAL n. 8.4.37 padAntasya
        // forbids Natva there. This is an existing golden -- a mutant that
        // drops this guard breaks the 1080, not just this test.
        assert_eq!(
            form_g("01.1082", Lakara::Lan, Purusha::Prathama, Vacana::Bahu),
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
        // restores it afterwards. The engine's 8.3.24 reaches only rudhAdi,
        // juhotyAdi and a curAdi root's own n, and this is a bhvAdi root, so
        // the bleeding is encoded as this guard. Another existing golden.
        assert_eq!(
            form_g("01.0696", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
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
        // tests/paradigm/), but t is a jhal immediately after n, so this case is
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

    /// 8.2.30 velarises a term-final cu sound that is pada-final or followed
    /// by a jhal-initial affix or āgama, and declines otherwise. Both
    /// reachable arms are pinned here: `j -> g` (√bhañj, √yuj) and `c -> k` (√ric, √vic). The `c` case
    /// is the one that distinguishes a real 1.1.50 substitution from the
    /// literal 'g' this rule used to write -- see `kutva_of`.
    #[test]
    fn coh_kuh_fires_only_word_finally_or_before_a_jhal() {
        let rule = rules().find(|r| r.id == "8.2.30").unwrap();

        // before a jhal: the `j` sits at the end of a non-final term (śnam's
        // infix leaves it in SHAP), and the jhal that conditions it is the
        // first character of the term after.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Ba"), Term::new("naj"), Term::new("ti")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "Banagti");

        // word-final: nothing follows the `j` at all.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Ba"), Term::new("naj")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "Banag");

        // a `c` takes the VOICELESS velar `k`, not `g`. This is the case the
        // old hardcoded substitute got wrong while still reaching the right
        // surface: 8.4.55 khari ca would have devoiced a spurious `g` to `k`
        // downstream, hiding the error from every paradigm golden.
        //
        // The `n` is still DENTAL here: √ric's ṇatva is 8.4.2's, which runs
        // later in the tripādī than 8.2.30, so `riRakti`'s retroflex has not
        // happened yet at this rule's turn. These fixtures are the real
        // intermediates, not the finished surfaces.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ri"), Term::new("nac"), Term::new("ti")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "rinakti");

        // word-final `c`, same arm.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ari"), Term::new("nac")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "arinak");

        // before a vowel: neither jhal nor word-final, so the rule declines.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Ba"), Term::new("nj"), Term::new("anti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "Banjanti");
    }

    /// 8.2.30 reads a term's FINAL sound only, and the term after it must be
    /// an affix or āgama: the cu has to stand at a morpheme's end. vidyut
    /// reads terms too, though it fires on every qualifying term and this
    /// engine on the first only (equivalent on the corpus). Main's
    /// whole-word scan velarised
    /// `picc`'s first `c` before the second (*pikcayati*); these pin the
    /// narrowing, each on a hand-built prakriyā.
    #[test]
    fn coh_kuh_reads_only_a_terms_final_cu_before_an_affix() {
        let rule = rules().find(|r| r.id == "8.2.30").unwrap();

        // a cu inside a term declines even before a jhal: √picc's real
        // tripādī intermediate, ṇic folded into the aṅga by 3.1.32 and made
        // `ay` by 6.1.78, then śap.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("piccay"), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "piccayati");

        // an earlier term-final cu that does not qualify (a vowel follows)
        // does not hide a later one that does, and only the later one moves.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("pac"), Term::new("anaj"), Term::new("ti")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "pacanagti");

        // an empty term is skipped: the jhal that conditions the cu is the
        // first sound of the next NON-EMPTY term (adādi's luk'd śap).
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("vac"), Term::new(""), Term::new("ti")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "vakti");

        // the dhātu is not an affix: an abhyāsa ending in a cu before an aṅga
        // beginning with a jhal declines. No abhyāsa in the corpus ends in a
        // consonant (7.4.60 halādiḥ śeṣaḥ), so this is the guard's only
        // witness.
        let mut p = Prakriya {
            terms: vec![
                Term::new(""),
                Term::new("jaj"),
                Term::new("da"),
                Term::new("a"),
                Term::new("ti"),
            ],
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "jajdaati");
    }

    /// 8.2.39 voices any pada-final jhal that `jashtva_of` can resolve — not
    /// just `t`, `z`, and `D`. The velar arm is newly reachable: 8.2.30 used
    /// to write a literal `g` and never produced a word-final `k` for this
    /// rule to see, but now that it substitutes the nearest velar, a
    /// `c`-final root like √ric presents a word-final `k` here too, and this
    /// rule must voice it exactly as it already voices `t`/`z`/`D`. The `s`
    /// case still belongs to its apavāda 8.2.66 (implemented inside the rule
    /// labelled 8.3.15) — `jashtva_of('s')` is `None`, so the widened guard
    /// still declines for it — and a `t` that is not pada-final is untouched.
    #[test]
    fn jhalam_jasho_ante_fires_on_any_pada_final_jhal_jashtva_of_resolves() {
        let rule = rules().find(|r| r.id == "8.2.39").unwrap();

        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("aBav"), Term::new("a"), Term::new("t")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "aBavad");

        // not pada-final: the `t` is followed by more of the ending
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("aBav"), Term::new("a"), Term::new("tAm")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        // word-final `z`: jashtva_of('z') is `q` (1.1.50 nearest-substitute,
        // not place-and-manner correspondence).
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("apina"), Term::new("z")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "apinaq");

        // word-final `D`: √rudh's laṅ prathama eka, once 8.2.23 has eaten
        // tip's own `t` and left the dhātu's own final exposed pada-finally.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("aru"), Term::new("Ra"), Term::new("D")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "aruRad");

        // `s`-final belongs to 8.2.66/8.3.15, not here
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("aBav"), Term::new("a"), Term::new("s")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        // word-final `k`: newly reachable now that 8.2.30 substitutes the
        // nearest velar instead of writing a literal `g`. `arinak` is
        // √ric's laṅ prathama eka intermediate right after 8.2.30 has fired
        // (see `coh_kuh_fires_only_word_finally_or_before_a_jhal` above);
        // this rule must voice its `k` to `g` the same way it voices
        // `t`/`z`/`D`.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ari"), Term::new("nak")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "arinag");

        // already-jaś pada-final: this pins the `Some(jash) == last` no-op
        // guard directly. `aBanag` is √bhañj's laṅ prathama eka intermediate
        // (the golden `aBanag` is pinned in `tests/paradigm/`) once 8.2.30 has
        // already velarised its stem to a `g` — `jashtva_of('g')` is `g`
        // itself, a fixed point, so this rule must decline rather than
        // "voice" it again. Without the no-op guard this would fire
        // vacuously and stamp a spurious 8.2.39 step into every already-jaś
        // trace in the corpus.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("aBana"), Term::new("g")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        // the other four fixed points in `jashtva_of`'s domain (`j`, `q`,
        // `d`, `b`) decline the same way, so the no-op guard is pinned
        // across its whole domain, not just the `g` witness above.
        for already_jash in ["j", "q", "d", "b"] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new("a"), Term::new(already_jash)]),
                ..Default::default()
            };
            assert!(!(rule.apply)(&mut p), "fired on already-{already_jash}");
        }

        // vowel-final
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Bav"), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
    }

    /// 8.2.40 takes a `t` or `T` to `D` immediately after a jhaṣ, and
    /// declines both when the preceding sound is not a jhaṣ and when the
    /// following sound is not a dental stop at all.
    #[test]
    fn jhashas_tathor_dhodhah_fires_only_on_a_dental_after_a_jhash() {
        let rule = rules().find(|r| r.id == "8.2.40").unwrap();

        // a `t` immediately after a jhaṣ (`D`) becomes `D`.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("inD"), Term::new(""), Term::new("te")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "inDDe");

        // a `t` after a non-jhaṣ: the rule declines.
        for stem in ["ind", "inn", "ins"] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new(stem), Term::new(""), Term::new("te")]),
                ..Default::default()
            };
            assert!(!(rule.apply)(&mut p), "fired after {stem}");
            assert_eq!(p.text(), format!("{stem}te"));
        }

        // an `s` after a jhaṣ: not a dental stop, so the rule declines.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("inD"), Term::new(""), Term::new("se")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "inDse");
    }

    #[test]
    fn jhashas_tathor_dhodhah_declines_after_dha_by_adhah() {
        // *adhaḥ*. √dhā's D meets the t of tas once 6.4.112 has elided its ā:
        // DattaH, not *DadDaH. The identical shape on any other row fires.
        let rule = rules().find(|r| r.id == "8.2.40").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("D"), Term::new(""), Term::new("taH")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "da".into();
        p.ctx.dhatupatha = "03.0011";
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "daDtaH");
        assert!(p.log.is_empty());
        p.ctx.dhatupatha = "";
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "daDDaH");
    }

    /// √dhā after 6.4.112 and 8.4.54: abhyāsa `da`, the aṅga reduced to one
    /// consonant, an empty śap, `ending`, and row 03.0011.
    fn dadh_prakriya(anga: &str, ending: &str) -> Prakriya {
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new(anga), Term::new(""), Term::new(ending)]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "da".into();
        p.ctx.dhatupatha = "03.0011";
        p
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
            terms: with_slots(vec![Term::new("pi"), Term::new("naz"), Term::new("si")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "pinaksi");

        // before any other sound: the rule declines.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("pi"), Term::new("naz"), Term::new("ti")]),
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
            terms: with_slots(vec![Term::new("piz"), Term::new(""), Term::new("ti")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "pizwi");

        // the D arm, whose only derivation-level cell (loṭ madhyama eka) a
        // later task finishes; pinned here so the arm is not witness-free.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("piz"), Term::new(""), Term::new("Di")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "pizQi");

        // one character between the `z` and the dental: no contact, decline.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("piz"), Term::new("a"), Term::new("nti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "pizanti");

        // a non-dental neighbour: y, v and m are none of them shtutva's
        // target, so the rule declines on each.
        for ending in ["ya", "va", "ma"] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new("piz"), Term::new(""), Term::new(ending)]),
                ..Default::default()
            };
            assert!(!(rule.apply)(&mut p), "fired before {ending}");
            assert_eq!(p.text(), format!("piz{ending}"));
        }
    }

    /// 8.4.53 turns a jhal into its jaś before a jhaś, anywhere the two are
    /// adjacent — not only at the word's own end, the shape both of 7a's
    /// witnesses (kfndDi, hinDi) happened to share. Pinned directly against
    /// the rule since neither witness exercises the non-final case.
    #[test]
    fn jhalam_jash_jhashi_fires_anywhere_not_just_word_finally() {
        let rule = rules().find(|r| r.id == "8.4.53").unwrap();

        // non-final: the jhaś sits mid-word, with more of the affix after it.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("kfnt"), Term::new(""), Term::new("Deva")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "kfndDeva");

        // a jhal before a non-jhaś: nothing for the rule to see.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("kfnt"), Term::new(""), Term::new("ati")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "kfntati");

        // a target already its own jaś: the no-op guard declines.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("kfnd"), Term::new(""), Term::new("Di")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "kfndDi");
    }

    #[test]
    fn abhyase_car_ca_deaspirates_the_abhyasa_and_nothing_else() {
        // Ju + ho + "" + ti → ju + ho + ti (juhoti): the abhyāsa's J goes to
        // j; the root's own h is not an abhyāsa sound and stays.
        let rule = rules().find(|r| r.id == "8.4.54").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ho"), Term::new(""), Term::new("ti")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "Ju".into();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "ju");
        assert_eq!(p.text(), "juhoti");
        assert_eq!(p.log.last().unwrap().sutra, "8.4.54");
        // Already car (√ki's ci) and no abhyāsa at all: no step recorded.
        for abhyasa in ["ci", ""] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new("ke"), Term::new(""), Term::new("ti")]),
                ..Default::default()
            };
            p.terms[ABHYASA].text = abhyasa.into();
            assert!(!(rule.apply)(&mut p), "{abhyasa:?}");
            assert_eq!(p.terms[ABHYASA].text, abhyasa);
            assert!(p.log.is_empty(), "{abhyasa:?}");
        }
    }

    #[test]
    fn dadhas_tathos_ca_aspirates_the_abhyasa_before_t_th_s_and_dhv() {
        // DattaH, DatTaH, Datse, and DadDve (whose root D 8.4.53 has already
        // made d). Only the abhyāsa changes; the root's sound is 8.4.53's or
        // 8.4.55's business.
        let rule = rules().find(|r| r.id == "8.2.38").unwrap();
        for (anga, ending) in [("D", "taH"), ("D", "TaH"), ("D", "se"), ("d", "Dve")] {
            let mut p = dadh_prakriya(anga, ending);
            assert!((rule.apply)(&mut p), "{anga}+{ending}");
            assert_eq!(p.terms[ABHYASA].text, "Da", "{anga}+{ending}");
            assert_eq!(p.terms[ANGA].text, anga, "{anga}+{ending}");
            assert_eq!(p.log.last().unwrap().sutra, "8.2.38");
        }
    }

    #[test]
    fn dadhas_tathos_ca_declines_with_the_a_present_before_other_sounds_and_off_dha() {
        let rule = rules().find(|r| r.id == "8.2.38").unwrap();
        // The ā survives before a pit ending: daDAti, not *DaDAti.
        let mut p = dadh_prakriya("DA", "ti");
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "da");
        // A vowel or another consonant follows: daDati, daDIDvam, daDvaH.
        for ending in ["ati", "IDvam", "vaH"] {
            let mut p = dadh_prakriya("D", ending);
            assert!(!(rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ABHYASA].text, "da", "{ending}");
        }
        // √dā has the identical shape and takes no 8.2.38: dattaH, not *DattaH.
        for number in ["03.0010", ""] {
            let mut p = dadh_prakriya("d", "taH");
            p.ctx.dhatupatha = number;
            assert!(!(rule.apply)(&mut p), "{number:?}");
            assert_eq!(p.terms[ABHYASA].text, "da", "{number:?}");
            assert!(p.log.is_empty(), "{number:?}");
        }
        // No ending term: must not panic.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("D"), Term::new("")]),
            ..Default::default()
        };
        p.ctx.dhatupatha = "03.0011";
        assert!(!(rule.apply)(&mut p));

        // Zero-length aṅga: hand-built only, since no real derivation ever
        // reaches row 03.0011 with an empty ANGA. Pinned anyway, because
        // `!= 1` and `> 1` behave identically at every aṅga length the
        // cases above exercise (1 and 2) and diverge only at 0.
        let mut p = dadh_prakriya("", "taH");
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "da");

        // An abhyāsa not starting with `d`: hand-built only, since every
        // real derivation reaching this rule has already run 8.4.54's
        // Da -> da. The only witness for `strip_prefix('d')`'s decline arm.
        let mut p = dadh_prakriya("D", "taH");
        p.terms[ABHYASA].text = "ja".into();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ABHYASA].text, "ja");
        assert!(p.log.is_empty());
    }

    /// 8.4.56 devoices a pada-final jhal. After 8.2.39 the reachable jhal
    /// finals are `d` and, since 8.2.39's widening, `g` (√ric's and √vic's
    /// laṅ eka cells); a vowel, a visarga and a nasal all decline.
    #[test]
    fn va_avasane_fires_only_on_a_pada_final_jhal() {
        let rule = rules().find(|r| r.id == "8.4.56").unwrap();

        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("aBav"), Term::new("a"), Term::new("d")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "aBavat");

        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("aBav"), Term::new("a"), Term::new("H")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("aBav"), Term::new("a"), Term::new("m")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Bav"), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
    }

    #[test]
    fn parasavarna_requires_a_yay() {
        // Enumerated rather than golden-driven: a predicate that fired
        // unconditionally still produces plausible Sanskrit for two of the
        // three 7a roots, and only √hiṃs catches it.
        for (number, la, pu, va, has_anusvara) in [
            ("07.0019", Lakara::Lat, Purusha::Prathama, Vacana::Dvi, true),
            (
                "07.0010",
                Lakara::Lat,
                Purusha::Prathama,
                Vacana::Bahu,
                false,
            ),
        ] {
            let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
            let p = sole(derive(d, la, d.pada.padas()[0], pu, va));
            assert!(
                p.log.iter().any(|s| s.sutra == "8.3.24"),
                "{}: 8.3.24 should always fire on a weak rudhādi cell",
                d.code
            );
            assert_eq!(
                p.text().contains('M'),
                has_anusvara,
                "{}: anusvāra retention",
                d.code
            );
        }
    }

    #[test]
    fn shcutva_fires_on_either_side_of_a_shcu_and_declines_after_sha() {
        let rule = rules().find(|r| r.id == "8.4.40").unwrap();

        // √chid laṅ prathama eka, after 6.1.73 has inserted the tuk: the
        // `t` is a stu, the `C` a ścu, so the `t` takes its palatal.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("atCi"), Term::new("nad")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "acCinad");

        // √jan laṭ prathama bahu after 6.4.98: the stu `n` FOLLOWS the ścu
        // `j` -- the converse arm (slice 3f3).
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("jn"), Term::new(""), Term::new("ati")]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "ja".into();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "jajYati");

        // 8.4.44 SAt: a stu FOLLOWING a `S` is exempt, and the converse arm
        // carries that exemption as its guard. Without it √kliś surfaces
        // *kliSYAti -- 41 invocations of 8.4.44 on that one root in
        // vidyut-prakriya over this corpus.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("kliS"), Term::new("nA"), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.text(), "kliSnAti");

        // Not a ścu after it: `z` is 8.4.41's trigger, not this rule's, and
        // this is √piṣ's laṭ prathama eka mid-derivation.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("pina"), Term::new("zwi")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        // Not a stu before it: a velar is neither `s` nor t-varga, so
        // `shcutva_of` returns None and the scan moves on.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ak"), Term::new("Ci")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
    }

    // --- 8.2.75 daś ca: gaṇa-free since slice 3f -------------------------

    /// A laṅ prakriyā at the tripādī with the whole stem in ANGA and no gaṇa
    /// tag: 8.2.75 reads none. `ending` "" is the 8.2.23-emptied sip.
    fn sip_prakriya(stem: &str, ending: &str, purusha: Purusha) -> Prakriya {
        Prakriya {
            terms: with_slots(vec![Term::new(stem), Term::new(""), Term::new(ending)]),
            ctx: Context::new(Lakara::Lan, Pada::Parasmaipada, purusha, Vacana::Eka),
            ..Default::default()
        }
    }

    #[test]
    fn das_ca_fires_on_any_ganas_pada_final_d_before_sip() {
        // √kit laṅ madhyama eka after 8.2.39: aciked → aciker, which 8.3.15
        // finishes to acikeH. No Rudhadi tag on the aṅga.
        let rule = rules().find(|r| r.id == "8.2.75").unwrap();
        let mut p = sip_prakriya("aciked", "", Purusha::Madhyama);
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "aciker");
        assert_eq!(p.log.last().unwrap().sutra, "8.2.75");
    }

    #[test]
    fn das_ca_declines_off_sip_off_d_and_before_a_live_ending() {
        let rule = rules().find(|r| r.id == "8.2.75").unwrap();
        for (stem, ending, purusha, why) in [
            // tip: laṅ prathama eka keeps aciked.
            ("aciked", "", Purusha::Prathama, "tip"),
            // a pada-final `t`, not `d`.
            ("aciket", "", Purusha::Madhyama, "t-final"),
            // is_sip() is lakāra-blind, so a vidhiliṅ madhyama eka shape
            // passes it; only dhatu_is_pada_final keeps the rule off.
            ("cikit", "yAd", Purusha::Madhyama, "live ending"),
        ] {
            let mut p = sip_prakriya(stem, ending, purusha);
            assert!(!(rule.apply)(&mut p), "{why}");
            assert!(p.log.is_empty(), "{why}");
        }
    }

    // --- 8.3.24 naś cāpadāntasya jhali: rudhādi and juhotyādi ------------

    /// √dhan at the tripādī: abhyāsa `da`, aṅga `Dan`, an empty śap (ślu),
    /// and `ending`. The aṅga carries Tag::Juhotyadi.
    fn dhan_prakriya(ending: &str) -> Prakriya {
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Dan"), Term::new(""), Term::new(ending)]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "da".into();
        p.terms[ANGA].add(Tag::Juhotyadi);
        p
    }

    #[test]
    fn nas_capadantasya_fires_on_a_juhotyadi_n_before_a_jhal() {
        // daDaMsi, daDaMhi (`h` is a jhal), and daDaMtaH, which 8.4.58
        // takes back to daDantaH.
        let rule = rules().find(|r| r.id == "8.3.24").unwrap();
        for (ending, want) in [("si", "daDaMsi"), ("hi", "daDaMhi"), ("taH", "daDaMtaH")] {
            let mut p = dhan_prakriya(ending);
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.text(), want);
            assert_eq!(p.log.last().unwrap().sutra, "8.3.24");
        }
    }

    #[test]
    fn nas_capadantasya_declines_off_its_two_ganas_and_before_a_non_jhal() {
        let rule = rules().find(|r| r.id == "8.3.24").unwrap();
        // bhvādi bhavanti: 7.1.3's `n` before `t`. The gaṇa test is what
        // keeps the rule off it; *apadāntasya* is not modelled.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Bav"), Term::new("a"), Term::new("nti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p), "bhavanti");
        assert!(p.log.is_empty());
        // √dhan before m, v, y: none is a jhal (daDanmi, daDanvaH, daDanyAt).
        for ending in ["mi", "vaH", "yAt"] {
            let mut p = dhan_prakriya(ending);
            assert!(!(rule.apply)(&mut p), "{ending}");
            assert!(p.log.is_empty(), "{ending}");
        }
    }

    // --- 8.2.73 tipy anasteḥ / 8.2.74 sipi dhāto rur vā: gaṇa-free since 3f2

    #[test]
    fn tipy_anasteh_and_sipi_dhato_fire_on_any_ganas_pada_final_s() {
        // √bhas laṅ after 8.2.23 has eaten the ending: abaBas, with no
        // Rudhadi tag on the aṅga. 8.2.73 writes the `d` at tip; 8.2.74 the
        // ru at sip, which 8.3.15 finishes to abaBaH.
        let r73 = rules().find(|r| r.id == "8.2.73").unwrap();
        let mut p = sip_prakriya("abaBas", "", Purusha::Prathama);
        assert!((r73.apply)(&mut p));
        assert_eq!(p.text(), "abaBad");
        assert_eq!(p.log.last().unwrap().sutra, "8.2.73");
        let r74 = rules().find(|r| r.id == "8.2.74").unwrap();
        let mut p = sip_prakriya("abaBas", "", Purusha::Madhyama);
        assert!((r74.apply)(&mut p));
        assert_eq!(p.text(), "abaBar");
        assert_eq!(p.log.last().unwrap().sutra, "8.2.74");
    }

    #[test]
    fn tipy_anasteh_and_sipi_dhato_decline_before_a_live_ending_and_off_s() {
        let r73 = rules().find(|r| r.id == "8.2.73").unwrap();
        let r74 = rules().find(|r| r.id == "8.2.74").unwrap();
        for (stem, ending, why) in [
            // is_sip() is lakāra-blind, so a vidhiliṅ madhyama eka shape
            // passes it; only dhatu_is_pada_final keeps both rules off.
            ("bapsyA", "s", "live ending"),
            // a pada-final `d` is 8.2.75's, not theirs.
            ("abaBad", "", "d-final"),
        ] {
            for rule in [r73, r74] {
                let mut p = sip_prakriya(stem, ending, Purusha::Madhyama);
                assert!(!(rule.apply)(&mut p), "{} {why}", rule.id);
                assert!(p.log.is_empty(), "{} {why}", rule.id);
            }
        }
        // 8.2.74 is sip-only: at tip the `s` is left for 8.2.73.
        let mut p = sip_prakriya("abaBas", "", Purusha::Prathama);
        assert!(!(r74.apply)(&mut p));
        assert!(p.log.is_empty());
    }

    // --- 8.4.55 khari ca: the whole word since slice 3f2 -----------------

    /// √bhas at the tripādī: abhyāsa `Ba`, aṅga `anga` (`Bs` once 6.4.100
    /// has run), an empty śap (ślu), and `ending`. No gaṇa tag: neither rule
    /// tested with it reads one.
    fn bhas_prakriya(anga: &str, ending: &str) -> Prakriya {
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new(anga), Term::new(""), Term::new(ending)]),
            ..Default::default()
        };
        p.terms[ABHYASA].text = "Ba".into();
        p
    }

    #[test]
    fn khari_ca_fires_inside_the_anga_and_at_the_junction() {
        let rule = rules().find(|r| r.id == "8.4.55").unwrap();
        // Word-internal: `Bs` + ati → `ps` (bapsati, before 8.4.54 takes the
        // abhyāsa's `B`). The junction holds `s` + `a`, which is not khar.
        let mut p = bhas_prakriya("Bs", "ati");
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "Bapsati");
        assert_eq!(p.log.last().unwrap().sutra, "8.4.55");
        // At the junction, as the rule always read: √indh's inD + se → intse.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("inD"), Term::new(""), Term::new("se")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "intse");
    }

    #[test]
    fn khari_ca_records_nothing_on_an_already_voiceless_jhal() {
        // A `t` before `s` (atsi) is already its own car, and nothing else in
        // the word is a jhal before a khar. The khar differs from the jhal on
        // purpose: the no-op guard must compare the substitute with the jhal,
        // not with the khar after it.
        let rule = rules().find(|r| r.id == "8.4.55").unwrap();
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("at"), Term::new(""), Term::new("si")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert!(p.log.is_empty());
    }

    // --- 8.2.26 jhalo jhali (slice 3f2) -----------------------------------

    #[test]
    fn jhalo_jhali_elides_an_s_between_two_jhals() {
        // √bhas after 6.4.100: the `s` of `Bs` before a `t`/`T`-initial
        // ending (babDaH, babDa, babDAt once 8.2.40 has run).
        let rule = rules().find(|r| r.id == "8.2.26").unwrap();
        for (ending, want) in [("tas", "BaBtas"), ("Ta", "BaBTa"), ("tAt", "BaBtAt")] {
            let mut p = bhas_prakriya("Bs", ending);
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.text(), want);
            assert_eq!(p.log.last().unwrap().sutra, "8.2.26");
        }
    }

    #[test]
    fn jhalo_jhali_declines_unless_both_neighbours_are_jhals() {
        let rule = rules().find(|r| r.id == "8.2.26").unwrap();
        for (anga, ending, why) in [
            // A vowel on the left: baBasti keeps its `s`.
            ("Bas", "ti", "a + s + t"),
            // A vowel on the right: bapsati keeps it, for 8.4.55.
            ("Bs", "ati", "B + s + a"),
            // A non-jhal on the right: bapsyAt keeps it too.
            ("Bs", "yAt", "B + s + y"),
            // A word-final `s` after a jhal has no right neighbour: the scan
            // stops short of it (a `1..w.len()` bound would index past the
            // word).
            ("Bs", "", "B + s + end of word"),
        ] {
            let mut p = bhas_prakriya(anga, ending);
            assert!(!(rule.apply)(&mut p), "{why}");
            assert!(p.log.is_empty(), "{why}");
        }
    }

    #[test]
    fn ksubhnadi_records_on_its_row_before_snu_only() {
        // This stage's entry, looked up in its own static.
        let rule = TRIPADI.iter().find(|r| r.id == "8.4.39").unwrap();
        assert!(!rule.vikalpa);
        assert_eq!(rule.bars, ["8.4.1", "8.4.2"]);
        let trp = |row: &'static str, vikarana: &str, tagged: bool| {
            let mut v = Term::new(vikarana);
            v.add(Tag::Vikarana);
            if tagged {
                v.add(Tag::Snu);
            }
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new("tfp"), v, Term::new("ti")]),
                ..Default::default()
            };
            p.ctx.dhatupatha = row;
            p
        };

        // √tṛp before śnu, whatever shape the tripādī has given it: `nu`,
        // `no` (7.3.84), `nuv` (6.4.77) and `nav` (guṇa and 6.1.78). Recorded,
        // no text changed; `run_pipeline` then skips the barred 8.4.1 and
        // 8.4.2 on the branch.
        for vikarana in ["nu", "no", "nuv", "nav"] {
            let mut p = trp("05.0028", vikarana, true);
            assert!((rule.apply)(&mut p), "{vikarana}");
            assert_eq!(p.text(), format!("tfp{vikarana}ti"));
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["8.4.39"], "{vikarana}");
        }

        // Another row declines even before śnu: curādi's `tfpa~` rows store
        // the same `tfp`, and a hand-built row has no number.
        for row in ["10.0351", "10.0355", ""] {
            let mut p = trp(row, "nu", true);
            assert!(!(rule.apply)(&mut p), "{row}");
            assert!(p.log.is_empty(), "{row}");
        }

        // Its row, but the next term is not śnu: an untagged vikaraṇa whose
        // text is `nu`, śap's `a`, an untagged `nA`, and an empty term.
        for vikarana in ["nu", "a", "nA", ""] {
            let mut p = trp("05.0028", vikarana, false);
            assert!(!(rule.apply)(&mut p), "{vikarana}");
            assert!(p.log.is_empty(), "{vikarana}");
        }
    }
}
