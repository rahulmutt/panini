use crate::context::Context;
use crate::controller::run_pipeline;
use crate::it_samjna::run_it_samjna;
use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use panini_data::{Dhatu, Gana, Lakara, Pada, Purusha, Vacana, tin_ending};

/// Guṇa substitute of an ik vowel (1.1.2 aden guṇaḥ, applied by 7.3.84).
fn guna_of(v: char) -> Option<&'static str> {
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
fn vrddhi_of(v: char) -> Option<char> {
    match v {
        'a' | 'A' => Some('A'),
        'i' | 'I' | 'e' | 'E' => Some('E'),
        'u' | 'U' | 'o' | 'O' => Some('O'),
        _ => None,
    }
}

fn is_vowel(c: char) -> bool {
    matches!(
        c,
        'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O'
    )
}

/// A jhal (obstruent) — the set 8.4.55's target ranges over. For this slice
/// only `d` is exercised, but the classifier is written generally.
fn is_jhal(c: char) -> bool {
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
fn is_khar(c: char) -> bool {
    matches!(
        c,
        'k' | 'K' | 'c' | 'C' | 'w' | 'W' | 't' | 'T' | 'p' | 'P' | 'S' | 'z' | 's'
    )
}

/// The car (voiceless unaspirated) substitute of a jhal, per 8.4.55.
/// Only `d → t` is exercised this slice; extend as later roots demand.
fn cartva_of(c: char) -> Option<char> {
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
fn is_vibhakti_protected_final(c: char) -> bool {
    matches!(c, 't' | 'T' | 'd' | 'D' | 'n' | 's' | 'm')
}

/// Index of the aṅga (the dhātu) in `terms`. Stable across the pipeline.
const ANGA: usize = 0;

/// Index of the tiṅ ending *before* śap is inserted (3.1.68).
const ENDING_PRE_SHAP: usize = 1;

/// Index of śap once inserted, and of the ending thereafter.
const SHAP: usize = 1;
const ENDING: usize = 2;

// NOTE: `ENDING_PRE_SHAP` and `SHAP` are deliberately the same value (1), not
// a typo. Rule 3.1.68 (kartari śap) inserts śap between the aṅga and the
// ending, which shifts the ending from index 1 to index 2. This bisects
// `TINANTA_RULES` into two halves along the array's shape, not along any
// lakāra or rule-family boundary:
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

/// The ordered rule list. Read it top to bottom against the Aṣṭādhyāyī: this
/// sequence IS the grammar this crate implements. Every rule self-guards and
/// returns whether it fired.
pub static TINANTA_RULES: &[Rule] = &[
    // 1.3.12 anudāttaṅita ātmanepadam: a root carrying the anudātta/ṅit
    // marker (here: the data-layer Atmanepadin tag) takes ātmanepada.
    // Sanctions the requested pada; the wrong pada BLOCKS the derivation —
    // derivation, not the analyzer, is the source of truth for pada.
    Rule {
        id: "1.3.12",
        name: "anudAttaNita Atmanepadam",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Atmanepadin) {
                return false; // parasmaipada roots are 1.3.78's business
            }
            match p.ctx.pada {
                Pada::Atmanepada => {
                    let before = p.snapshot();
                    p.record("1.3.12", "anudAttaNita Atmanepadam", before);
                    true
                }
                Pada::Parasmaipada => {
                    p.blocked = true;
                    false
                }
            }
        },
    },
    // 1.3.78 śeṣāt kartari parasmaipadam: everything else takes parasmaipada.
    Rule {
        id: "1.3.78",
        name: "SezAt kartari parasmEpadam",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms[ANGA].has(Tag::Atmanepadin) {
                return false;
            }
            match p.ctx.pada {
                Pada::Parasmaipada => {
                    let before = p.snapshot();
                    p.record("1.3.78", "SezAt kartari parasmEpadam", before);
                    true
                }
                Pada::Atmanepada => {
                    p.blocked = true;
                    false
                }
            }
        },
    },
    // 3.4.78 tiptasjhi...: replace the lakāra by the tiṅ ending.
    // 3.4.113 tiṅ-śit sārvadhātukam makes it sārvadhātuka.
    Rule {
        id: "3.4.78",
        name: "tiptasJisipTasTamibvasmas",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let before = p.snapshot();
            let ending = tin_ending(p.ctx.pada, p.ctx.purusha, p.ctx.vacana);
            let mut e = Term::new(ending);
            e.add(Tag::Tin);
            e.add(Tag::Sarvadhatuka);
            p.terms.push(e);
            p.record("3.4.78", "tiptasJisipTasTamibvasmas", before);
            true
        },
    },
    // it-samjña on the tiṅ ending (1.3.3 halantyam / 1.3.9 tasya lopaḥ),
    // respecting 1.3.4: the final s/t/m of a vibhakti is protected, so only
    // endings whose final is a genuine anubandha (tip/sip/mip → the pit marker
    // `p`) are reduced.
    //
    // This MUST precede the lakāra-specific substitutions below: 3.4.100
    // itaś ca elides the `i` of `tip`, and that `i` is only exposed once
    // halantyam has stripped the `p`.
    Rule {
        id: "1.3.9",
        name: "tasya lopaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let last = p.terms[ENDING_PRE_SHAP].text.chars().last();
            if last.map(is_vibhakti_protected_final).unwrap_or(false) {
                return false;
            }
            let mut e = p.terms[ENDING_PRE_SHAP].clone();
            let original = e.text.clone();
            run_it_samjna(&mut e, p, ENDING_PRE_SHAP);
            p.terms[ENDING_PRE_SHAP] = e;
            p.terms[ENDING_PRE_SHAP].text != original
        },
    },
    // 1.2.4 sārvadhātukam apit: an apit sārvadhātuka behaves as ṅit. An
    // atideśa (the 3.4.85 precedent): a rule that appears in the trace and
    // sets a term-level tag — distinct from ctx.is_ngit_like, which says the
    // *lakāra* is ṅit and drives 3.4.99/100/101.
    //
    // Guard notes (see the spec's 1.2.4 section):
    // - Ātmanepada only in this slice: parasmaipada apit endings (tas, Ji…)
    //   are equally ṅid-vat in principle, but no implemented rule consumes
    //   that fact, and firing here would add a step to the pinned
    //   parasmaipada traces. Widening later is additive, not a fix.
    // - Loṭ uttama is a genuine exclusion, not trace-minimalism: 3.4.92's
    //   own "pic ca" makes those endings pit, hence not apit — which is what
    //   keeps 7.2.81 off the āṭ-āgama (AvahE goes to 6.1.101 instead).
    Rule {
        id: "1.2.4",
        name: "sArvaDAtukam apit",
        kind: RuleKind::Atidesha,
        apply: |p| {
            if !matches!(p.ctx.pada, Pada::Atmanepada)
                || (matches!(p.ctx.lakara, Lakara::Lot) && matches!(p.ctx.purusha, Purusha::Uttama))
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].add(Tag::Ngit);
            p.record("1.2.4", "sArvaDAtukam apit", before);
            true
        },
    },
    // 3.4.85 loṭo laṅvat: loṭ behaves as laṅ, so the ṅit-conditioned rules
    // (3.4.99, 3.4.101) apply to it. An atideśa, so it is a rule and appears
    // in the trace rather than being folded into Context::new.
    Rule {
        id: "3.4.85",
        name: "loTo laNvat",
        kind: RuleKind::Atidesha,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) || p.ctx.is_ngit_like {
                return false;
            }
            let before = p.snapshot();
            p.ctx.is_ngit_like = true;
            p.record("3.4.85", "loTo laNvat", before);
            true
        },
    },
    // 3.4.108 jher jus: in liṅ, the ending Ji is replaced by jus. Apavāda to
    // 3.4.100 itaś ca (Ji is i-final), hence ordered before it — the same
    // preemption pattern as 3.4.87/3.4.89 before 3.4.86.
    //
    // The initial j of jus is an anubandha (1.3.7 cuṭū), elided here and
    // recorded as 1.3.9 per the existing convention that saṃjñā rules
    // (1.3.3/1.3.7/1.3.8) are silent and only the elision is traced. It is
    // NOT folded into run_it_samjna: a general cuṭū arm there would also eat
    // the J of laṭ/loṭ's Ji, which is not an anubandha but a coded segment
    // that must survive for 7.1.3 jho'ntaḥ.
    Rule {
        id: "3.4.108",
        name: "Jer jus",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin) || p.terms[ENDING_PRE_SHAP].text != "Ji" {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "jus".into();
            p.record("3.4.108", "Jer jus", before);
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "us".into();
            p.record("1.3.9", "tasya lopaH", before);
            true
        },
    },
    // 3.4.105 jhasya ran: in liṅ, ātmanepada Ja → ran. Apavāda to 7.1.3
    // jho'ntaḥ by position: 7.1.3 runs post-śap, by which time Ja is gone.
    // The liṅ ātmanepada sibling of 3.4.108 jher jus.
    Rule {
        id: "3.4.105",
        name: "Jasya ran",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin) || p.terms[ENDING_PRE_SHAP].text != "Ja" {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "ran".into();
            p.record("3.4.105", "Jasya ran", before);
            true
        },
    },
    // 3.4.106 iṭo 't: in liṅ, the ātmanepada uttama-eka i (from iw) → a.
    // laBeya, not laBeyi.
    Rule {
        id: "3.4.106",
        name: "iwo't",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin)
                || !matches!(p.ctx.pada, Pada::Atmanepada)
                || p.terms[ENDING_PRE_SHAP].text != "i"
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "a".into();
            p.record("3.4.106", "iwo't", before);
            true
        },
    },
    // 3.4.101 tasthasthamipāṃ tāṃtaṃtāmaḥ: tas→tAm, Tas→tam, Ta→ta, mip→am.
    //
    // The mip→am arm excludes loṭ: loṭ's uttama-eka is `ni` by the more specific
    // 3.4.89 mer niḥ, so it must not be captured here.
    //
    // MUST precede 3.4.99: 3.4.101 is the apavAda (the specific rule) for
    // tas/Tas/Ta/mip, while 3.4.99 is the utsarga (the general Ngit rule).
    // By 1.4.2 vipratizeDhe paraM kAryam ("in conflict, the [more specific/
    // later-scoped] rule prevails"), the apavAda wins over the general rule
    // whenever both would otherwise apply. Ordering 3.4.101 first realizes
    // that outcome directly (verified by hand-tracing `aBavatAm`, which the
    // reversed order corrupts into a spurious `aBavata`).
    //
    // The sutra's tas/thas/tha/mip are parasmaipada endings; today this is
    // also safe by text, the guard states the domain.
    Rule {
        id: "3.4.101",
        name: "tasTasTamipAM tAMtaMtAmaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.ctx.is_ngit_like || matches!(p.ctx.pada, Pada::Atmanepada) {
                return false;
            }
            let sub = match p.terms[ENDING_PRE_SHAP].text.as_str() {
                "tas" => "tAm",
                "Tas" => "tam",
                "Ta" => "ta",
                // loṭ keeps its apavāda 3.4.89 mer niḥ (mi → ni); every
                // other ṅit-like lakāra takes am.
                "mi" if !matches!(p.ctx.lakara, Lakara::Lot) => "am",
                _ => return false,
            };
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = sub.into();
            p.record("3.4.101", "tasTasTamipAM tAMtaMtAmaH", before);
            true
        },
    },
    // 3.4.99 nityaṃ ṅitaḥ: the final `s` of a ṅit-lakāra's tiṅ is elided.
    // vas → va, mas → ma.
    Rule {
        id: "3.4.99",
        name: "nityaM NitaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.ctx.is_ngit_like
                || !matches!(p.terms[ENDING_PRE_SHAP].text.as_str(), "vas" | "mas")
            {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ENDING_PRE_SHAP].text.chars().collect();
            s.pop();
            p.terms[ENDING_PRE_SHAP].text = s.into_iter().collect();
            p.record("3.4.99", "nityaM NitaH", before);
            true
        },
    },
    // 3.4.87 ser hyapic ca: loṭ madhyama-eka `si` → `hi`.
    // Apavāda to 3.4.86 er uḥ, hence ordered before it.
    Rule {
        id: "3.4.87",
        name: "ser hyapic ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) || p.terms[ENDING_PRE_SHAP].text != "si" {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "hi".into();
            p.record("3.4.87", "ser hyapic ca", before);
            true
        },
    },
    // 3.4.89 mer niḥ: loṭ uttama-eka `mi` → `ni`.
    // Apavāda to 3.4.86 er uḥ, hence ordered before it.
    Rule {
        id: "3.4.89",
        name: "mer niH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) || p.terms[ENDING_PRE_SHAP].text != "mi" {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "ni".into();
            p.record("3.4.89", "mer niH", before);
            true
        },
    },
    // 3.4.86 er uḥ: the final `i` of the tiṅ → `u`. ti → tu, Ji → Ju.
    //
    // Guarded to exactly `ti`/`Ji` rather than "any i-final ending": `si` and
    // `mi` are preempted by the apavādas above, and by this point they have
    // already become `hi`/`ni`, which are also i-final. The explicit set makes
    // the preemption independent of ordering accidents.
    Rule {
        id: "3.4.86",
        name: "er uH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) {
                return false;
            }
            if !matches!(p.terms[ENDING_PRE_SHAP].text.as_str(), "ti" | "Ji") {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ENDING_PRE_SHAP].text.chars().collect();
            s.pop();
            s.push('u');
            p.terms[ENDING_PRE_SHAP].text = s.into_iter().collect();
            p.record("3.4.86", "er uH", before);
            true
        },
    },
    // 3.4.100 itaś ca: the final `i` of a ṅit-lakāra's tiṅ is elided.
    // laṅ/vidhiliṅ: ti → t, si → s, Ji → J (laṅ; liṅ's Ji is gone by
    // 3.4.108). loṭ is excluded: its final `i` is handled by the apavāda
    // 3.4.86 er uḥ. The sutra elides the i of *parasmaipada* ṅit endings;
    // ātmanepada vahi/mahi/i keep theirs — aBavAvahi.
    Rule {
        id: "3.4.100",
        name: "itaS ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // ṅit lakāras generally — but NOT loṭ, whose i-finals belong to
            // the apavāda set 3.4.86/87/89 (and 3.4.87's output `hi` is
            // itself i-final, so a bare ṅit guard would corrupt it to `h`).
            if !p.ctx.is_ngit_like
                || matches!(p.ctx.lakara, Lakara::Lot)
                || matches!(p.ctx.pada, Pada::Atmanepada)
                || !p.terms[ENDING_PRE_SHAP].text.ends_with('i')
            {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ENDING_PRE_SHAP].text.chars().collect();
            s.pop();
            p.terms[ENDING_PRE_SHAP].text = s.into_iter().collect();
            p.record("3.4.100", "itaS ca", before);
            true
        },
    },
    // 3.4.80 thāsaḥ se: ātmanepada madhyama-eka TAs → se. Apavāda to 3.4.79
    // ṭita… ter e, hence ordered before it: reversed, 3.4.79 would rewrite
    // TAs's ṭi (As → e) to Te and this rule would never see TAs.
    Rule {
        id: "3.4.80",
        name: "TAsas se",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // Guarded to the ṭit lakāras (laṭ, loṭ): in the ṅit lakāras the
            // 3.4.79 context that 3.4.80 carves out does not apply and TAs
            // survives unchanged (laṅ aBavaTAH).
            if !matches!(p.ctx.lakara, Lakara::Lat | Lakara::Lot)
                || p.terms[ENDING_PRE_SHAP].text != "TAs"
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "se".into();
            p.record("3.4.80", "TAsas se", before);
            true
        },
    },
    // 3.4.79 ṭita ātmanepadānām ter e: in a ṭit lakāra (laṭ, loṭ — the ṭ
    // anubandha in their names), the ṭi of an ātmanepada ending (its last
    // vowel plus anything after, 1.1.64 aco'ntyādi ṭi) → e.
    // ta→te, AtAm→Ate, Ja→Je, ATAm→ATe, Dvam→Dve, i→e, vahi→vahe, mahi→mahe.
    Rule {
        id: "3.4.79",
        name: "wita AtmanepadAnAM wer e",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lat | Lakara::Lot)
                || !matches!(p.ctx.pada, Pada::Atmanepada)
            {
                return false;
            }
            let text = &p.terms[ENDING_PRE_SHAP].text;
            let Some(ti_start) = text
                .char_indices()
                .rev()
                .find(|&(_, c)| is_vowel(c))
                .map(|(i, _)| i)
            else {
                return false;
            };
            let replaced = format!("{}e", &text[..ti_start]);
            if replaced == *text {
                return false; // ṭi is already e (post-3.4.80 "se"): no-op
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = replaced;
            p.record("3.4.79", "wita AtmanepadAnAM wer e", before);
            true
        },
    },
    // 3.4.91 savābhyāṃ vāmau: loṭ's e → va after s, → am after v.
    // se → sva, Dve → Dvam. Apavāda to 3.4.90 ām etaḥ, hence ordered
    // before it (reversed: se → sAm, Dve → DvAm).
    Rule {
        id: "3.4.91",
        name: "savAByAM vAmO",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) {
                return false;
            }
            let sub = match p.terms[ENDING_PRE_SHAP].text.as_str() {
                "se" => "sva",
                "Dve" => "Dvam",
                _ => return false,
            };
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = sub.into();
            p.record("3.4.91", "savAByAM vAmO", before);
            true
        },
    },
    // 3.4.93 eta ai: loṭ's uttama e → E. Apavāda to 3.4.90 (ordered before
    // it); afterwards the uttama endings are E-final, which 3.4.90's short-e
    // guard ignores — no explicit uttama exclusion needed there.
    Rule {
        id: "3.4.93",
        name: "eta E",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot)
                || !matches!(p.ctx.purusha, Purusha::Uttama)
                || !matches!(p.ctx.pada, Pada::Atmanepada)
                || !p.terms[ENDING_PRE_SHAP].text.ends_with('e')
            {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ENDING_PRE_SHAP].text.chars().collect();
            s.pop();
            s.push('E');
            p.terms[ENDING_PRE_SHAP].text = s.into_iter().collect();
            p.record("3.4.93", "eta E", before);
            true
        },
    },
    // 3.4.90 ām etaḥ: loṭ's ending-final e → Am. te→tAm, Ate→AtAm, Je→JAm,
    // ATe→ATAm. The A-initial results are then reshaped post-śap by 7.2.81
    // (ṅid-vat) exactly like their laṭ counterparts — the net laṭ/loṭ
    // difference in those cells is this rule alone.
    Rule {
        id: "3.4.90",
        name: "Am etaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot)
                || !matches!(p.ctx.pada, Pada::Atmanepada)
                || !p.terms[ENDING_PRE_SHAP].text.ends_with('e')
            {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ENDING_PRE_SHAP].text.chars().collect();
            s.pop();
            p.terms[ENDING_PRE_SHAP].text = s.into_iter().collect::<String>() + "Am";
            p.record("3.4.90", "Am etaH", before);
            true
        },
    },
    // 3.4.92 āḍ uttamasya pic ca: the āṭ-āgama is prefixed to loṭ's uttama
    // endings. ni → Ani, va → Ava, ma → Ama. E/vahE/mahE (the ātmanepada
    // shapes 3.4.93 leaves) also take it: E → AE, vahE → AvahE, mahE → AmahE.
    //
    // Guarded to exactly `ni`/`va`/`ma`/`E`/`vahE`/`mahE` rather than "any
    // uttama ending in loṭ": those forms only exist because 3.4.89 mer niḥ,
    // 3.4.99 nityaṃ ṅitaḥ, and 3.4.93 eta ai have already normalized
    // mi→ni, vas/mas→va/ma, and the ātmanepada e→E. The explicit set makes
    // the preemption independent of ordering accidents — MUST follow
    // 3.4.89, 3.4.99, and 3.4.93, but the guard no longer silently depends
    // on it.
    Rule {
        id: "3.4.92",
        name: "Aq uttamasya pic ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot)
                || !matches!(p.ctx.purusha, Purusha::Uttama)
                || !matches!(
                    p.terms[ENDING_PRE_SHAP].text.as_str(),
                    "ni" | "va" | "ma" | "E" | "vahE" | "mahE"
                )
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = format!("A{}", p.terms[ENDING_PRE_SHAP].text);
            p.record("3.4.92", "Aq uttamasya pic ca", before);
            true
        },
    },
    // 3.4.103 yāsuṭ parasmaipadeṣūdātto ṅic ca: the yāsuṭ-āgama is prefixed
    // to liṅ's parasmaipada endings. Modelled as a text prefix on the ending
    // term (the āṭ 3.4.92 / aṭ 6.4.71 precedent) so the term indices stay
    // stable. The sutra's own text says parasmaipadeṣu, now enforced;
    // ātmanepada liṅ takes sīyuṭ instead (3.4.102, Task 9).
    //
    // MUST follow the 3.4.9x/10x ending substitutions above: their guards
    // match the ending text exactly ("mi", "vas", …), so prefixing yAs first
    // would make every one of them miss.
    Rule {
        id: "3.4.103",
        name: "yAsuw parasmEpadezUdAtto Nic ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin) || matches!(p.ctx.pada, Pada::Atmanepada) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = format!("yAs{}", p.terms[ENDING_PRE_SHAP].text);
            p.record("3.4.103", "yAsuw parasmEpadezUdAtto Nic ca", before);
            true
        },
    },
    // 3.4.102 liṅaḥ sīyuṭ: liṅ's ātmanepada endings take the sīyuṭ-āgama,
    // prefixed as text like yāsuṭ (3.4.103). Its s is non-final, so the
    // existing 7.2.79 salopa elides it: sIyta → Iyta — then 6.1.87 (a+I→e)
    // and 6.1.66 finish exactly as in the yāsuṭ chain.
    // Same ordering constraint as 3.4.103: MUST follow the ending
    // substitutions (3.4.105/3.4.106 match exact text).
    Rule {
        id: "3.4.102",
        name: "liNas sIyuw",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin) || !matches!(p.ctx.pada, Pada::Atmanepada)
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = format!("sIy{}", p.terms[ENDING_PRE_SHAP].text);
            p.record("3.4.102", "liNas sIyuw", before);
            true
        },
    },
    // ============================================================
    // BOUNDARY: 3.1.68 kartari śap inserts śap and shifts the ending from
    // index 1 to index 2 (see the ANGA/ENDING_PRE_SHAP/SHAP/ENDING doc
    // comment above). Every rule ABOVE this point addresses the ending as
    // `ENDING_PRE_SHAP` (index 1). Every rule BELOW this point addresses the
    // ending as `ENDING` (index 2), and may address śap as `SHAP` (index 1).
    // ============================================================
    // 3.1.69 divādibhyaḥ śyan: divādi (gaṇa 4) takes śyan, not śap. Apavāda
    // to the utsarga 3.1.68, ordered before it (as 6.4.72 precedes 6.4.71).
    // śyan is apit; the second 1.2.4 makes it ṅit and 1.1.5 then blocks guṇa.
    Rule {
        id: "3.1.69",
        name: "divAdiByaH Syan",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Divadi) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("Syan");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.69", "divAdiByaH Syan", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S, 1.3.3 strips n → ya
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 3.1.77 tudādibhyaḥ śaḥ: tudādi (gaṇa 6) takes śa, not śap. Apavāda to
    // 3.1.68, same shape as 3.1.69. śa is apit → ṅit (1.2.4) → guṇa blocked.
    Rule {
        id: "3.1.77",
        name: "tudAdiByaH SaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Tudadi) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("Sa");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.77", "tudAdiByaH SaH", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S → a
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 3.1.68 kartari śap: insert śap between dhātu and ending, run it-samjña
    // on it (Sap → a), and mark the dhātu an aṅga.
    Rule {
        id: "3.1.68",
        name: "kartari Sap",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // Utsarga: fires only when no apavāda vikaraṇa (śyan 3.1.69 / śa
            // 3.1.77) is already present. Guarding on the vikaraṇa's presence
            // keeps śap the default without hard-coding a gaṇa, so curādi can
            // reuse śap later.
            if p.terms.len() > SHAP && p.terms[SHAP].has(Tag::Vikarana) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("Sap");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            s.add(Tag::Pit); // p-anubandha: śap is pit, so 1.2.4 leaves it alone
            p.terms.insert(SHAP, s);
            p.record("3.1.68", "kartari Sap", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP);
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 2.4.72 adiprabhṛtibhyaḥ śapaḥ: adādi (gaṇa 2) luks the śap that 3.1.68
    // inserts, so the tiṅ ending attaches directly to the root. Modelled by
    // emptying the śap term's text (the term stays, keeping ENDING at index 2
    // and text() = root + "" + ending). Guarded on Tag::Adadi and on a real
    // śap being present, so it never touches divādi/tudādi (śyan/śa) or bhvādi
    // that has already been processed differently.
    Rule {
        id: "2.4.72",
        name: "adipraBftiByaH SapaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Adadi) {
                return false;
            }
            if !(p.terms.len() > SHAP
                && p.terms[SHAP].has(Tag::Vikarana)
                && !p.terms[SHAP].text.is_empty())
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = String::new();
            p.record("2.4.72", "adipraBftiByaH SapaH", before);
            true
        },
    },
    // 1.2.4 sārvadhātukam apit — second application, on the vikaraṇa. The
    // first application (above the boundary) tags apit ātmanepada endings;
    // this one tags the apit sārvadhātuka VIKARAṆA ṅit once it exists. śyan
    // and śa are apit (no p-anubandha); śap carries Tag::Pit (3.1.68) and is
    // skipped — so bhvādi is untouched. NOT pada-gated: śyan/śa are apit in
    // parasmaipada derivations too, which is what blocks guṇa in dīvyati /
    // kupyati / tudati.
    Rule {
        id: "1.2.4",
        name: "sArvaDAtukam apit",
        kind: RuleKind::Atidesha,
        apply: |p| {
            if !(p.terms.len() > SHAP
                && p.terms[SHAP].has(Tag::Vikarana)
                && !p.terms[SHAP].has(Tag::Pit)
                && !p.terms[SHAP].has(Tag::Ngit))
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].add(Tag::Ngit);
            p.record("1.2.4", "sArvaDAtukam apit", before);
            true
        },
    },
    // 6.4.71 luṅlaṅlṛṅkṣvaḍudāttaḥ: the aṭ-āgama is prefixed to the aṅga in laṅ.
    //
    // Modelled as a prefix on the aṅga's text rather than as a separate term,
    // so the ANGA/SHAP/ENDING indices stay stable for every later rule. The
    // trace still cites 6.4.71, which is what the reader checks.
    Rule {
        id: "6.4.71",
        name: "luNlaNlfNkzvaqudAttaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let first = p.terms[ANGA].text.chars().next().unwrap();
            if !matches!(p.ctx.lakara, Lakara::Lan) || is_vowel(first) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = format!("a{}", p.terms[ANGA].text);
            p.record("6.4.71", "luNlaNlfNkzvaqudAttaH", before);
            true
        },
    },
    // 6.4.72 āḍ ajādīnām: vowel-initial aṅgas take the āṭ-āgama in laṅ
    // (apavāda to 6.4.71's aṭ). The A then merges with the root's initial
    // vowel by 6.1.90 āṭaś ca into vṛddhi: a+eD → ED, a+Ikz → Ekz.
    Rule {
        id: "6.4.72",
        name: "Aq ajAdInAm",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let first = p.terms[ANGA].text.chars().next().unwrap();
            // Only apply to true vowel-initial roots, not to 'a' prefixed by 6.4.71
            if !matches!(p.ctx.lakara, Lakara::Lan) || !is_vowel(first) || first == 'a' {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = format!("A{}", p.terms[ANGA].text);
            p.record("6.4.72", "Aq ajAdInAm", before);
            true
        },
    },
    // 7.1.3 jho'ntaḥ: a leading `J` of the ending → `ant`.
    Rule {
        id: "7.1.3",
        name: "Jo'ntaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ENDING].text.starts_with('J') {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("ant{rest}");
            p.record("7.1.3", "Jo'ntaH", before);
            true
        },
    },
    // 7.2.79 liṅaḥ salopo 'nantyasya: the non-final s of sārvadhātuka liṅ's
    // ending is elided. yAst → yAt, yAss → yAs (madhyama-eka: only the first
    // s is non-final!), yAsus → yAus. MUST precede 7.2.80: only after the s
    // goes does the ending start with the `yA` shape 7.2.80 rewrites.
    // Every non-final s reaching this rule is yāsuṭ- or sīyuṭ-derived; the
    // invariant is that the only non-final s is āgama-initial.
    Rule {
        id: "7.2.79",
        name: "liNaH salopo'nantyasya",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin) {
                return false;
            }
            let text = &p.terms[ENDING].text;
            let n = text.chars().count();
            let reduced: String = text
                .chars()
                .enumerate()
                .filter(|&(i, c)| c != 's' || i + 1 == n)
                .map(|(_, c)| c)
                .collect();
            if reduced == *text {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = reduced;
            p.record("7.2.79", "liNaH salopo'nantyasya", before);
            true
        },
    },
    // 7.2.80 ato yeyaḥ: after an a-final aṅga (here: the śap), the yA of the
    // yāsuṭ is replaced by iy. yAt → iyt, yAus → iyus.
    Rule {
        id: "7.2.80",
        name: "ato yeyaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin)
                || !p.terms[SHAP].text.ends_with('a')
                || !p.terms[ENDING].text.starts_with("yA")
            {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(2).collect();
            p.terms[ENDING].text = format!("iy{rest}");
            p.record("7.2.80", "ato yeyaH", before);
            true
        },
    },
    // 7.2.81 āto ṅitaḥ: after an a-final aṅga (the śap), the initial ā of a
    // ṅit ending → iy. Ate→iyte (laṭ), AtAm→iytAm (laṅ/loṭ), ATe→iyTe.
    // The ṅit condition is the TERM tag from 1.2.4 (laṭ/loṭ are ṭit lakāras,
    // yet their apit ātmanepada endings behave as ṅit) — NOT ctx.is_ngit_like.
    // The tag also keeps this rule off parasmaipada loṭ uttama's āṭ (Ani),
    // which 1.2.4 never tags (pic ca) and which belongs to 6.1.101.
    // MUST precede 6.1.101, which would otherwise dīrgha-merge the tagged
    // A-initial endings (laṭ 3du would surface as laBAte, not laBete).
    Rule {
        id: "7.2.81",
        name: "Ato NitaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[SHAP].text.ends_with('a')
                || !p.terms[ENDING].has(Tag::Ngit)
                || !p.terms[ENDING].text.starts_with('A')
            {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("iy{rest}");
            p.record("7.2.81", "Ato NitaH", before);
            true
        },
    },
    // 7.3.84 sārvadhātukārdhadhātukayoḥ: guṇa of the aṅga's final ik.
    Rule {
        id: "7.3.84",
        name: "sArvaDAtukArDaDAtukayoH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // 1.1.5 kṅiti ca: a following ṅit sārvadhātuka blocks guṇa. The
            // vikaraṇa at SHAP is ṅit (1.2.4) exactly when apit (śyan, śa);
            // śap is pit and is not, so bhvādi guṇa is unaffected.
            if p.terms.len() > SHAP && p.terms[SHAP].has(Tag::Ngit) {
                return false;
            }
            let last = p.terms[ANGA].text.chars().last().unwrap();
            let Some(g) = guna_of(last) else {
                return false;
            };
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ANGA].text.chars().collect();
            s.pop();
            p.terms[ANGA].text = s.into_iter().collect::<String>() + g;
            p.record("7.3.84", "sArvaDAtukArDaDAtukayoH", before);
            true
        },
    },
    // 7.3.86 pugantalaghūpadhasya ca: guṇa of a light (short, pre-single-
    // consonant) penultimate ik before the sārvadhātuka. vft → vart. The
    // only curated root with an ik upadhā; final-ik roots (BU, smf…) are
    // 7.3.84's business and never reach this shape guard.
    Rule {
        id: "7.3.86",
        name: "pugantalaGUpaDasya ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // 1.1.5 kṅiti ca: a following ṅit sārvadhātuka blocks guṇa. The
            // vikaraṇa at SHAP is ṅit (1.2.4) exactly when apit (śyan, śa);
            // śap is pit and is not, so bhvādi guṇa is unaffected.
            if p.terms.len() > SHAP && p.terms[SHAP].has(Tag::Ngit) {
                return false;
            }
            let chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let n = chars.len();
            if n < 2 || is_vowel(chars[n - 1]) {
                return false; // final-vowel aṅgas are 7.3.84's business
            }
            let Some(g) = guna_of(chars[n - 2]) else {
                return false;
            };
            // laghu: the short ik vowels are exactly the lowercase ones our
            // guna_of accepts; long variants are guru and out of scope here.
            if !matches!(chars[n - 2], 'i' | 'u' | 'f' | 'x') {
                return false;
            }
            let before = p.snapshot();
            let mut s: String = chars[..n - 2].iter().collect();
            s.push_str(g);
            s.push(chars[n - 1]);
            p.terms[ANGA].text = s;
            p.record("7.3.86", "pugantalaGUpaDasya ca", before);
            true
        },
    },
    // 6.1.78 eco'yavāyāvaḥ: e/o/E/O before a vowel → ay/av/Ay/Av.
    Rule {
        id: "6.1.78",
        name: "eco'yavAyAvaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let anga_last = p.terms[ANGA].text.chars().last().unwrap();
            let sub = match anga_last {
                'e' => "ay",
                'o' => "av",
                'E' => "Ay",
                'O' => "Av",
                _ => return false,
            };
            // śap may be luk'd (adādi, 2.4.72): then it is empty and this rule
            // has no a-final vikaraṇa to work against. Decline rather than
            // panic. (when the consonant-final and ātmanepada adādi roots land,
            // this will generalize to the root+ending junction for √śī.)
            let Some(next_first) = p.terms[SHAP].text.chars().next() else {
                return false;
            };
            if !is_vowel(next_first) {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ANGA].text.chars().collect();
            s.pop();
            p.terms[ANGA].text = s.into_iter().collect::<String>() + sub;
            p.record("6.1.78", "eco'yavAyAvaH", before);
            true
        },
    },
    // 7.3.101 ato dīrgho yañi: aṅga-final `a` (śap) → `A` before a yañ-initial
    // sārvadhātuka ending (here: mi/vas/mas).
    Rule {
        id: "7.3.101",
        name: "ato dIrGo yaYi",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // loṭ uttama gets its dīrgha from 3.4.92 āḍ + 6.1.101 instead.
            if matches!(p.ctx.lakara, Lakara::Lot) {
                return false;
            }
            let ending_first = p.terms[ENDING].text.chars().next().unwrap();
            // Ends in `a`, not equal to `a`: śyan's residue is `ya`, not `a`
            // (see 6.1.97's comment for why only the final vowel matters).
            if !matches!(ending_first, 'm' | 'v') || !p.terms[SHAP].text.ends_with('a') {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[SHAP].text.chars().collect();
            s.pop();
            s.push('A');
            p.terms[SHAP].text = s.into_iter().collect();
            p.record("7.3.101", "ato dIrGo yaYi", before);
            true
        },
    },
    // 6.1.101 akaḥ savarṇe dīrghaḥ: an ak vowel followed by a savarṇa vowel
    // coalesces into the corresponding long vowel. Three arms:
    //   - adādi vidhiliṅ 1sg (śap luk'd, 7.2.80 declined): the yāsuṭ ā + the
    //     ending a coalesce inside the ending, yAam → yAm (→ yAyAm);
    //   - adādi (śap luk'd by 2.4.72): the aṅga's own final `A` meets an
    //     a/ā-initial ending, yA + anti → yAnti, yA + Ani → yAni;
    //   - bhvādi &c.: śap `a` + the ending's initial `A` (from 3.4.92 āḍ),
    //     Bav + a + Ani → BavAni.
    Rule {
        id: "6.1.101",
        name: "akaH savarRe dIrGaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            // adādi vidhiliṅ 1sg: after 7.2.79 the ending is `yAam` (yāsuṭ ā +
            // the uttama-eka `am`). 7.2.80 would have rewritten `yA`->`iy` for
            // a thematic gaṇa, but śap is luk'd so it declined; the yāsuṭ ā and
            // the ending a are savarṇa -> a single ā: yAam -> yAm. Guard is
            // tight: VidhiLin + empty śap + a `yA`+vowel ending (never `yAt`/
            // `yAs`/... whose yA is followed by a consonant).
            if p.terms.len() > ENDING
                && matches!(p.ctx.lakara, Lakara::VidhiLin)
                && p.terms[SHAP].text.is_empty()
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
    // 6.1.66 lopo vyor vali: v or y is elided before a val consonant. Here
    // only the ending-initial y from the yāsuṭ chain ever matches: yt → t,
    // yva → va; yus survives (u is a vowel, not in val). The val pratyāhāra
    // is every consonant except y, and no `yy` sequence arises in this
    // engine, so "any consonant" is an exact guard here.
    Rule {
        id: "6.1.66",
        name: "lopo vyor vali",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let mut chars = p.terms[ENDING].text.chars();
            if chars.next() != Some('y') {
                return false;
            }
            let Some(second) = chars.next() else {
                return false;
            };
            if is_vowel(second) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
            p.record("6.1.66", "lopo vyor vali", before);
            true
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
            // Currently unreachable (no r/v-final adādi root in scope); when the
            // consonant-final and ātmanepada adādi roots land, this must generalize
            // this to the root+ending junction, as 6.1.78 already flags.
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

pub fn derive(
    dhatu: &Dhatu,
    lakara: Lakara,
    pada: Pada,
    purusha: Purusha,
    vacana: Vacana,
) -> Prakriya {
    let mut p = Prakriya {
        ctx: Context::new(lakara, pada, purusha, vacana),
        ..Default::default()
    };
    p.terms.push({
        let mut t = Term::new(dhatu.code);
        t.add(Tag::Dhatu);
        if matches!(dhatu.pada, Pada::Atmanepada) {
            t.add(Tag::Atmanepadin);
        }
        match dhatu.gana {
            Gana::Divadi => t.add(Tag::Divadi),
            Gana::Tudadi => t.add(Tag::Tudadi),
            Gana::Adadi => t.add(Tag::Adadi),
            Gana::Bhvadi => {}
        }
        t
    });
    run_pipeline(&mut p, TINANTA_RULES);
    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};

    fn form(code: &str, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, Lakara::Lat, Pada::Parasmaipada, pu, va).text()
    }

    fn form_g(code: &str, la: Lakara, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, la, d.pada, pu, va).text()
    }

    #[test]
    fn divadi_tudadi_present_third_singular() {
        // Guṇa blocked by 1.1.5 (śyan/śa are ṅit): kup→kupyati NOT kopyati,
        // tud→tudati NOT todati, juṣ→juṣate NOT joṣate.
        assert_eq!(
            form_g("naS", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "naSyati"
        );
        assert_eq!(
            form_g("kup", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "kupyati"
        );
        assert_eq!(
            form_g("man", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "manyate"
        );
        assert_eq!(
            form_g("yuD", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "yuDyate"
        );
        assert_eq!(
            form_g("vid", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "vidyate"
        );
        assert_eq!(
            form_g("tud", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "tudati"
        );
        assert_eq!(
            form_g("liK", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "liKati"
        );
        assert_eq!(
            form_g("viS", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "viSati"
        );
        assert_eq!(
            form_g("juz", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "juzate"
        );
        assert_eq!(
            form_g("vij", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "vijate"
        );
        assert_eq!(
            form_g("gur", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "gurate"
        );
    }

    #[test]
    fn divadi_tudadi_vowel_sandhi_cells() {
        // 3rd-singular alone never exercises the SHAP-final-vowel sandhi
        // rules (6.1.97/6.1.101/6.1.90/6.1.87/7.3.101): its `ti` ending is
        // consonant-initial. These cells pin the fix that generalized those
        // rules from "SHAP.text == a single a" to "SHAP.text ends in a" so
        // śyan's two-character `ya` residue (not just śa's/śap's `a`)
        // coalesces correctly with a following vowel.
        //
        // 3rd plural (6.1.97 para-rūpa: śyan/śa `a` + Ji→`anti`'s `a` → `a`,
        // not `aa`). naS/tud/juz avoid √div, whose 8.2.77 lengthening is
        // Task 5's job — its short-vowel `divyanti` is already correct here.
        assert_eq!(
            form_g("naS", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "naSyanti"
        );
        assert_eq!(
            form_g("tud", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "tudanti"
        );
        assert_eq!(
            form_g("juz", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "juzante"
        );
        // 1st singular (7.3.101 ato dIrgho yaYi: śyan/śa `a` + `mi` → `Ami`).
        assert_eq!(
            form_g("tud", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
            "tudAmi"
        );
        // Ātmanepada uttama-eka (6.1.97 a+e para-rūpa: śyan `ya` + `e` → `ye`).
        assert_eq!(
            form_g("man", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
            "manye"
        );
        // 7.2.81 Ato NitaH: ātmanepada dual Ate→iyte, then coalesced.
        assert_eq!(
            form_g("man", Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
            "manyete"
        );
        // 7.2.80 ato yeyaH: vidhiliṅ yA→iy after śyan's `ya`.
        assert_eq!(
            form_g("kup", Lakara::VidhiLin, Purusha::Prathama, Vacana::Eka),
            "kupyet"
        );
        // 6.4.105 ato heH: imperative hi-elision after śyan's `ya`.
        assert_eq!(
            form_g("naS", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
            "naSya"
        );
    }

    #[test]
    fn div_lengthens_before_syan() {
        assert_eq!(
            form_g("div", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "dIvyati"
        );
        // laṅ: augment does not disturb the upadhā i.
        assert_eq!(
            form_g("div", Lakara::Lan, Purusha::Prathama, Vacana::Eka),
            "adIvyat"
        );
    }

    #[test]
    fn adadi_luk_present_no_junction_cells() {
        // ā-final adādi roots: śap is luk'd (2.4.72), the ending attaches to
        // the root directly. These cells need only the luk (no ā+a junction).
        assert_eq!(
            form_g("yA", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "yAti"
        );
        assert_eq!(
            form_g("yA", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
            "yAsi"
        );
        assert_eq!(
            form_g("yA", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
            "yAmi"
        );
        // laṅ: aṭ-augment (yā is consonant-initial) → ayā; ending attaches.
        assert_eq!(
            form_g("yA", Lakara::Lan, Purusha::Prathama, Vacana::Eka),
            "ayAt"
        );
        // loṭ 2sg: hi does NOT elide after ā (6.4.105 needs short a) → yāhi.
        assert_eq!(
            form_g("yA", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
            "yAhi"
        );
    }

    #[test]
    fn adadi_root_final_a_coalesces_with_vowel_endings() {
        // ā + a(nti) → ā : yānti (laṭ 3pl), yAntu (loṭ 3pl), ayAn (laṅ 3pl).
        assert_eq!(
            form_g("yA", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "yAnti"
        );
        assert_eq!(
            form_g("yA", Lakara::Lot, Purusha::Prathama, Vacana::Bahu),
            "yAntu"
        );
        assert_eq!(
            form_g("yA", Lakara::Lan, Purusha::Prathama, Vacana::Bahu),
            "ayAn"
        );
        // ā + A(ṭ) → ā : loṭ uttama-eka takes āṭ (yA + Ani → yAni).
        assert_eq!(
            form_g("yA", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
            "yAni"
        );
    }

    #[test]
    fn adadi_vidhilin_derives_the_yas_yuh_reduction() {
        // adādi × vidhiliṅ now
        // derives through the full pipeline, running the yāsuṭ chain plus
        // the 6.1.96 / 6.1.101 junction reductions, for every cell and pada.
        for code in ["yA", "vA"] {
            let d = dhatus().iter().find(|d| d.code == code).unwrap();
            for pu in [Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama] {
                for va in [Vacana::Eka, Vacana::Dvi, Vacana::Bahu] {
                    let p = derive(d, Lakara::VidhiLin, d.pada, pu, va);
                    assert!(!p.blocked, "{code} vidhiliṅ {pu:?} {va:?} was blocked");
                    assert!(!p.log.is_empty(), "{code} vidhiliṅ ran no rules");
                    assert!(
                        !p.text().is_empty(),
                        "{code} vidhiliṅ {pu:?} {va:?} is empty"
                    );
                }
            }
        }
        assert_eq!(
            form_g("yA", Lakara::VidhiLin, Purusha::Prathama, Vacana::Bahu),
            "yAyuH"
        );
        assert_eq!(
            form_g("yA", Lakara::VidhiLin, Purusha::Uttama, Vacana::Eka),
            "yAyAm"
        );
    }

    #[test]
    fn cartva_turns_d_to_t_before_khar() {
        // √ad laṭ: 3sg atti (d+t), 2sg atsi (d+s), 2pl atTa (d+T).
        assert_eq!(
            form_g("ad", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "atti"
        );
        assert_eq!(
            form_g("ad", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
            "atsi"
        );
        assert_eq!(
            form_g("ad", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
            "atTa"
        );
        // Not before a non-khar (m/v) or a vowel: admi, adanti stay.
        assert_eq!(
            form_g("ad", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
            "admi"
        );
        assert_eq!(
            form_g("ad", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "adanti"
        );
    }

    #[test]
    fn bhu_3sg_is_bhavati() {
        assert_eq!(form("BU", Purusha::Prathama, Vacana::Eka), "Bavati");
    }
    #[test]
    fn bhu_1pl_is_bhavamah() {
        assert_eq!(form("BU", Purusha::Uttama, Vacana::Bahu), "BavAmaH");
    }
    #[test]
    fn smr_3sg_is_smarati() {
        assert_eq!(form("smf", Purusha::Prathama, Vacana::Eka), "smarati");
    }
    #[test]
    fn pat_3du_is_patatah() {
        assert_eq!(form("paW", Purusha::Prathama, Vacana::Dvi), "paWataH");
    }
    #[test]
    fn bhu_3pl_is_bhavanti() {
        assert_eq!(form("BU", Purusha::Prathama, Vacana::Bahu), "Bavanti");
    }
    #[test]
    fn shap_is_pit_and_bhvadi_guna_survives() {
        // Regression guard for Task 3: adding the guṇa-block mechanism must
        // not disturb bhvādi. śap is pit, so 7.3.84 still fires for BU.
        assert_eq!(form("BU", Purusha::Prathama, Vacana::Eka), "Bavati");
        let d = dhatus().iter().find(|d| d.code == "vft").unwrap();
        // vṛt uses 7.3.86 (laghūpadhā guṇa) before śap (pit) → vartate.
        assert_eq!(
            derive(
                d,
                Lakara::Lat,
                Pada::Atmanepada,
                Purusha::Prathama,
                Vacana::Eka
            )
            .text(),
            "vartate"
        );
    }
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

    #[test]
    fn ji_3sg_is_jayati() {
        // "ji" ends in `i`; 7.3.84 guNa gives "je", then 6.1.78 eco'yavAyAvaH
        // (the `e` arm, distinct from the `o` arm already exercised by BU)
        // turns je+a into jaya, yielding "jayati".
        assert_eq!(form("ji", Purusha::Prathama, Vacana::Eka), "jayati");
    }

    #[test]
    fn trace_is_recorded() {
        let d = dhatus().iter().find(|d| d.code == "BU").unwrap();
        let p = derive(
            d,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert!(p.log.iter().any(|s| s.sutra == "3.1.68"));
        assert!(p.log.iter().any(|s| s.sutra == "7.3.84"));
        assert!(!p.log.is_empty());
    }

    #[test]
    fn it_samjna_rule_reports_when_ending_is_reduced() {
        // 1.3.9 tasya lopaH's `apply` returns whether it actually elided the
        // ending's it; pin that return value directly, since `run_pipeline`
        // discards it and no golden form exercises it in isolation.
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("tip")],
            log: vec![],
            ..Default::default()
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "1.3.9").unwrap();
        assert!(
            (rule.apply)(&mut p),
            "1.3.9 should report firing when tip loses its final p"
        );
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "ti");
    }

    #[test]
    fn jher_jus_replaces_ji_and_elides_the_j_marker() {
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("Ji")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Bahu,
            ),
            blocked: false,
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.108").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "us");
        // Both the substitution and the marker elision must be traced.
        assert!(p.log.iter().any(|s| s.sutra == "3.4.108"));
        assert!(p.log.iter().any(|s| s.sutra == "1.3.9"));
    }

    #[test]
    fn jher_jus_leaves_lat_and_lot_ji_alone() {
        // laṭ's Ji must survive to 7.1.3 jho'ntaḥ (Bavanti), loṭ's to
        // 3.4.86 er uḥ (Bavantu).
        for lakara in [Lakara::Lat, Lakara::Lot] {
            let mut p = Prakriya {
                terms: vec![Term::new("BU"), Term::new("Ji")],
                log: vec![],
                ctx: Context::new(lakara, Pada::Parasmaipada, Purusha::Prathama, Vacana::Bahu),
                blocked: false,
            };
            let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.108").unwrap();
            assert!(!(rule.apply)(&mut p), "{lakara:?}");
            assert_eq!(p.terms[ENDING_PRE_SHAP].text, "Ji");
        }
    }

    // --- Fix 2: the sutra-name hard gate --------------------------------
    //
    // AGENTS.md declares that sutra ids/names in traces must match the cited
    // reference. Nothing previously asserted that a `RuleStep.name` emitted
    // into a user-facing trace actually equals the `name` field of the
    // `TINANTA_RULES` entry with the matching `id`. `Rule.name` itself is
    // write-only dead data: what a user sees is `RuleStep.name`, populated
    // solely from the string literal passed to `p.record(...)` at each call
    // site (there are two for id "1.3.9": this rule's own body, and
    // `run_it_samjna` in `it_samjna.rs`, called from 3.1.68's body — both
    // currently pass the literal "tasya lopaH"). Comparing every recorded
    // step's name against `TINANTA_RULES` by id, over real derivations,
    // catches either call site drifting from `Rule.name` without having to
    // special-case which call site fired.

    /// Drive every (root, lakara, purusha, vacana) cell over the curated
    /// roots and all four lakaras/nine cells, and assert every recorded
    /// `RuleStep.name` matches the `TINANTA_RULES` entry for its `sutra` id.
    #[test]
    fn recorded_step_names_match_tinanta_rules_for_every_id() {
        let lakaras = [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin];
        let purushas = [Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama];
        let vacanas = [Vacana::Eka, Vacana::Dvi, Vacana::Bahu];

        let mut steps_checked = 0usize;
        for d in dhatus() {
            for &lakara in &lakaras {
                for &purusha in &purushas {
                    for &vacana in &vacanas {
                        let p = derive(d, lakara, d.pada, purusha, vacana);
                        for step in &p.log {
                            let rule = TINANTA_RULES.iter().find(|r| r.id == step.sutra).unwrap_or_else(|| {
                                panic!(
                                    "recorded step cites sutra id {:?} which is not in TINANTA_RULES \
                                     (dhatu {}, {lakara:?} {purusha:?} {vacana:?})",
                                    step.sutra, d.code
                                )
                            });
                            assert_eq!(
                                step.name, rule.name,
                                "RuleStep.name for sutra {} (dhatu {}, {lakara:?} {purusha:?} {vacana:?}) \
                                 is {:?} but TINANTA_RULES[id={:?}].name is {:?} -- a record() call site \
                                 has drifted from the Rule.name field",
                                step.sutra, d.code, step.name, rule.id, rule.name
                            );
                            steps_checked += 1;
                        }
                    }
                }
            }
        }
        assert!(
            steps_checked > 0,
            "sanity: the derivations above should have recorded at least one RuleStep"
        );
    }

    /// SLP1 validity of every sutra name in `TINANTA_RULES`: none may contain
    /// one of the digraphs (`gh`, `jh`, `dh`, `kh`, `th`, `bh`, `ph`, `ch`)
    /// that are always wrong inside SLP1 (SLP1 is one-char-per-phoneme; those
    /// aspirates are `G`, `J`, `D`, `K`, `T`, `B`, `P`, `C`). This is the
    /// error class that produced the non-SLP1 names swept by hand earlier on
    /// this branch (see commit 892cfa4).
    ///
    /// This check is intentionally narrow: it flags exactly these eight
    /// lowercase digraphs and nothing else. A legitimate SLP1 name may
    /// contain a genuine consonant-then-`h`-vowel sequence (e.g. `hy`, as in
    /// "ser hyapic ca") or the avagraha apostrophe (as in "Jo'ntaH"); this
    /// check does not touch either. It also cannot detect a name whose
    /// *content* is wrong (a mistranscribed sutra) as long as it avoids these
    /// eight substrings -- it only catches the specific historical error
    /// class of "wrote an aspirate as two ASCII letters instead of SLP1's one
    /// capital letter."
    #[test]
    fn sutra_names_contain_no_forbidden_slp1_digraphs() {
        const FORBIDDEN: [&str; 8] = ["gh", "jh", "dh", "kh", "th", "bh", "ph", "ch"];
        for rule in TINANTA_RULES {
            for bad in FORBIDDEN {
                assert!(
                    !rule.name.contains(bad),
                    "rule {} name {:?} contains forbidden non-SLP1 digraph {:?}",
                    rule.id,
                    rule.name,
                    bad
                );
            }
        }
    }

    #[test]
    fn itash_ca_fires_for_vidhilin() {
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("ti")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.100").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "t");
    }

    #[test]
    fn itash_ca_never_touches_lot_even_when_ngit_like() {
        // After 3.4.85 loṭ is ṅit-like, and after 3.4.87 its madhyama-eka
        // ending is `hi` — which is i-final. A bare ṅit guard would corrupt
        // it to `h`; the guard must exclude loṭ explicitly.
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("hi")],
            log: vec![],
            ctx: Context::new(
                Lakara::Lot,
                Pada::Parasmaipada,
                Purusha::Madhyama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        p.ctx.is_ngit_like = true; // as 3.4.85 would have set it
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.100").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "hi");
    }

    #[test]
    fn mip_becomes_am_in_vidhilin() {
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("mi")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Uttama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.101").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "am");
    }

    #[test]
    fn aduttamasya_requires_uttama_purusha() {
        // loT, madhyama, ending "va": the ending is in the {ni, va, ma} set
        // and lakara is loT, but puruSa is madhyama, not uttama. The guard's
        // second `||` must still short-circuit to false. Kills the `||` ->
        // `&&` mutant at the second operator, which would otherwise let
        // this fire whenever loT holds and the ending matches, regardless
        // of puruSa.
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("va")],
            log: vec![],
            ctx: Context::new(
                Lakara::Lot,
                Pada::Parasmaipada,
                Purusha::Madhyama,
                Vacana::Bahu,
            ),
            blocked: false,
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.92").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "va");
    }

    #[test]
    fn yasut_prefixes_the_substituted_ending() {
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("t")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.103").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "yAst");
    }

    #[test]
    fn yasut_is_vidhilin_only() {
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("t")],
            log: vec![],
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.103").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "t");
    }

    #[test]
    fn salopa_elides_only_the_non_final_s() {
        // Madhyama-eka is the trap: yAs + s = yAss, and only the FIRST s is
        // non-final. Eliding both would derive *Bave for BaveH.
        for (ending, want) in [("yAst", "yAt"), ("yAss", "yAs"), ("yAsus", "yAus")] {
            let mut p = Prakriya {
                terms: vec![Term::new("Bav"), Term::new("a"), Term::new(ending)],
                log: vec![],
                ctx: Context::new(
                    Lakara::VidhiLin,
                    Pada::Parasmaipada,
                    Purusha::Prathama,
                    Vacana::Eka,
                ),
                blocked: false,
            };
            let rule = TINANTA_RULES.iter().find(|r| r.id == "7.2.79").unwrap();
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ENDING].text, want, "{ending}");
        }
    }

    #[test]
    fn ato_yeyah_rewrites_the_ya_prefix_after_shap_a() {
        let mut p = Prakriya {
            terms: vec![Term::new("Bav"), Term::new("a"), Term::new("yAt")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.2.80").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "iyt");
    }

    #[test]
    fn ato_yeyah_requires_vidhilin_lakara() {
        // shap == "a" and ending starts_with "yA" are both satisfied, but the
        // lakara isn't vidhilin: the guard's first `||` must still short-
        // circuit to false. Kills the `||` -> `&&` mutant at the first
        // operator, which would otherwise let this fire whenever the other
        // two conditions hold regardless of lakara.
        let mut p = Prakriya {
            terms: vec![Term::new("Bav"), Term::new("a"), Term::new("yAt")],
            log: vec![],
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.2.80").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "yAt");
    }

    #[test]
    fn ato_yeyah_requires_shap_a() {
        // lakara is vidhilin and ending starts_with "yA", but shap isn't
        // "a": the guard's second `||` must still short-circuit to false.
        // Kills the `||` -> `&&` mutant at the second operator, which would
        // otherwise let this fire whenever lakara is vidhilin regardless of
        // shap.
        let mut p = Prakriya {
            terms: vec![Term::new("i"), Term::new("i"), Term::new("yAt")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.2.80").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "yAt");
    }

    fn lin_form(code: &str, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, Lakara::VidhiLin, Pada::Parasmaipada, pu, va).text()
    }

    #[test]
    fn bhu_vidhilin_all_nine_cells() {
        for (pu, va, want) in [
            (Purusha::Prathama, Vacana::Eka, "Bavet"),
            (Purusha::Prathama, Vacana::Dvi, "BavetAm"),
            (Purusha::Prathama, Vacana::Bahu, "BaveyuH"),
            (Purusha::Madhyama, Vacana::Eka, "BaveH"),
            (Purusha::Madhyama, Vacana::Dvi, "Bavetam"),
            (Purusha::Madhyama, Vacana::Bahu, "Baveta"),
            (Purusha::Uttama, Vacana::Eka, "Baveyam"),
            (Purusha::Uttama, Vacana::Dvi, "Baveva"),
            (Purusha::Uttama, Vacana::Bahu, "Bavema"),
        ] {
            assert_eq!(lin_form("BU", pu, va), want, "{pu:?} {va:?}");
        }
    }

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
            let rule = TINANTA_RULES.iter().find(|r| r.id == "6.1.66").unwrap();
            assert_eq!((rule.apply)(&mut p), fires, "{ending}");
            assert_eq!(p.terms[ENDING].text, want, "{ending}");
        }
    }

    #[test]
    fn pada_sanction_blocks_wrong_pada_derivations() {
        // 1.3.12/1.3.78: derivation is the source of truth for pada. A
        // wrong-pada derive must not silently produce a surface form.
        let labh = dhatus().iter().find(|d| d.code == "laB").unwrap();
        let p = derive(
            labh,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert!(p.blocked, "atmanepadin root + parasmaipada must block");
        assert_eq!(p.text(), "laB", "no rule may run after the block");
        assert!(p.log.is_empty(), "a blocked derivation records nothing");

        let bhu = dhatus().iter().find(|d| d.code == "BU").unwrap();
        let p = derive(
            bhu,
            Lakara::Lat,
            Pada::Atmanepada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert!(p.blocked, "parasmaipada root + atmanepada must block");
    }

    #[test]
    fn pada_sanction_records_the_sanctioning_sutra() {
        let bhu = dhatus().iter().find(|d| d.code == "BU").unwrap();
        let p = derive(
            bhu,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert_eq!(p.log.first().unwrap().sutra, "1.3.78");

        let labh = dhatus().iter().find(|d| d.code == "laB").unwrap();
        let p = derive(
            labh,
            Lakara::Lat,
            Pada::Atmanepada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert_eq!(p.log.first().unwrap().sutra, "1.3.12");
    }

    #[test]
    fn itash_ca_and_yasut_are_parasmaipada_only() {
        // 3.4.100 must not eat the final i of atmanepada vahi/mahi/i in Nit
        // lakaras (aBavAvahi, not aBavAvah), and 3.4.103's own text says
        // parasmEpadezu — atmanepada lin takes siyut (3.4.102) instead.
        for (id, ending, lakara) in [
            ("3.4.100", "vahi", Lakara::Lan),
            ("3.4.100", "i", Lakara::Lan),
            ("3.4.103", "ta", Lakara::VidhiLin),
        ] {
            let mut p = Prakriya {
                terms: vec![Term::new("laB"), Term::new(ending)],
                log: vec![],
                ctx: Context::new(lakara, Pada::Atmanepada, Purusha::Uttama, Vacana::Dvi),
                blocked: false,
            };
            let rule = TINANTA_RULES.iter().find(|r| r.id == id).unwrap();
            assert!(!(rule.apply)(&mut p), "{id} must not fire for atmanepada");
            assert_eq!(p.terms[ENDING_PRE_SHAP].text, ending);
        }
    }

    #[test]
    fn sarvadhatukam_apit_tags_atmanepada_endings_ngit() {
        let mut p = Prakriya {
            terms: vec![Term::new("laB"), Term::new("ta")],
            log: vec![],
            ctx: Context::new(
                Lakara::Lat,
                Pada::Atmanepada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "1.2.4").unwrap();
        assert!((rule.apply)(&mut p));
        assert!(p.terms[ENDING_PRE_SHAP].has(Tag::Ngit));
        assert!(p.log.iter().any(|s| s.sutra == "1.2.4"));
    }

    #[test]
    fn sarvadhatukam_apit_skips_parasmaipada_and_lot_uttama() {
        // Parasmaipada apit endings are Nid-vat in principle too, but no
        // implemented rule consumes the fact and firing here would perturb
        // the 216 pinned parasmaipada traces (see the spec). Lot uttama is a
        // GENUINE exclusion: 3.4.92's own "pic ca" makes those endings pit,
        // hence not apit — which is what keeps 7.2.81 off the AT-agama.
        let cases = [
            ("ti", Lakara::Lat, Pada::Parasmaipada, Purusha::Prathama),
            ("iw", Lakara::Lot, Pada::Atmanepada, Purusha::Uttama),
        ];
        for (ending, lakara, pada, purusha) in cases {
            let mut p = Prakriya {
                terms: vec![Term::new("laB"), Term::new(ending)],
                log: vec![],
                ctx: Context::new(lakara, pada, purusha, Vacana::Eka),
                blocked: false,
            };
            let rule = TINANTA_RULES.iter().find(|r| r.id == "1.2.4").unwrap();
            assert!(!(rule.apply)(&mut p), "{ending} {lakara:?} {pada:?}");
            assert!(!p.terms[ENDING_PRE_SHAP].has(Tag::Ngit));
        }
    }

    fn lat_a_form(code: &str, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, Lakara::Lat, Pada::Atmanepada, pu, va).text()
    }

    #[test]
    fn labh_lat_atmanepada_all_nine_cells() {
        let expected = [
            (Purusha::Prathama, Vacana::Eka, "laBate"),
            (Purusha::Prathama, Vacana::Dvi, "laBete"),
            (Purusha::Prathama, Vacana::Bahu, "laBante"),
            (Purusha::Madhyama, Vacana::Eka, "laBase"),
            (Purusha::Madhyama, Vacana::Dvi, "laBeTe"),
            (Purusha::Madhyama, Vacana::Bahu, "laBaDve"),
            (Purusha::Uttama, Vacana::Eka, "laBe"),
            (Purusha::Uttama, Vacana::Dvi, "laBAvahe"),
            (Purusha::Uttama, Vacana::Bahu, "laBAmahe"),
        ];
        for (pu, va, form) in expected {
            assert_eq!(lat_a_form("laB", pu, va), form, "{pu:?} {va:?}");
        }
    }

    fn lot_a_form(code: &str, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, Lakara::Lot, Pada::Atmanepada, pu, va).text()
    }

    #[test]
    fn labh_lot_atmanepada_all_nine_cells() {
        let expected = [
            (Purusha::Prathama, Vacana::Eka, "laBatAm"),
            (Purusha::Prathama, Vacana::Dvi, "laBetAm"),
            (Purusha::Prathama, Vacana::Bahu, "laBantAm"),
            (Purusha::Madhyama, Vacana::Eka, "laBasva"),
            (Purusha::Madhyama, Vacana::Dvi, "laBeTAm"),
            (Purusha::Madhyama, Vacana::Bahu, "laBaDvam"),
            (Purusha::Uttama, Vacana::Eka, "laBE"),
            (Purusha::Uttama, Vacana::Dvi, "laBAvahE"),
            (Purusha::Uttama, Vacana::Bahu, "laBAmahE"),
        ];
        for (pu, va, form) in expected {
            assert_eq!(lot_a_form("laB", pu, va), form, "{pu:?} {va:?}");
        }
    }

    #[test]
    fn savabhyam_vamau_preempts_am_etah() {
        // 3.4.91 (se→sva, Dve→Dvam) is the apavāda ordered before 3.4.90:
        // reversed, se would become sAm and Dve DvAm.
        assert_eq!(lot_a_form("laB", Purusha::Madhyama, Vacana::Eka), "laBasva");
        assert_eq!(
            lot_a_form("laB", Purusha::Madhyama, Vacana::Bahu),
            "laBaDvam"
        );
    }

    #[test]
    fn am_etah_is_lot_only() {
        // laṭ's te/Ate must NOT become tAm/AtAm.
        assert_eq!(lat_a_form("laB", Purusha::Prathama, Vacana::Eka), "laBate");
    }

    fn lin_a_form(code: &str, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, Lakara::VidhiLin, Pada::Atmanepada, pu, va).text()
    }

    #[test]
    fn labh_vidhilin_atmanepada_all_nine_cells() {
        let expected = [
            (Purusha::Prathama, Vacana::Eka, "laBeta"),
            (Purusha::Prathama, Vacana::Dvi, "laBeyAtAm"),
            (Purusha::Prathama, Vacana::Bahu, "laBeran"),
            (Purusha::Madhyama, Vacana::Eka, "laBeTAH"),
            (Purusha::Madhyama, Vacana::Dvi, "laBeyATAm"),
            (Purusha::Madhyama, Vacana::Bahu, "laBeDvam"),
            (Purusha::Uttama, Vacana::Eka, "laBeya"),
            (Purusha::Uttama, Vacana::Dvi, "laBevahi"),
            (Purusha::Uttama, Vacana::Bahu, "laBemahi"),
        ];
        for (pu, va, form) in expected {
            assert_eq!(lin_a_form("laB", pu, va), form, "{pu:?} {va:?}");
        }
    }

    #[test]
    fn siyut_survives_salopa_as_long_i() {
        // sIyta → (7.2.79) Iyta: 6.1.87's widened guard must accept the
        // long I (yāsuṭ's chain produced short iy via 7.2.80).
        let p = {
            let d = dhatus().iter().find(|d| d.code == "laB").unwrap();
            derive(
                d,
                Lakara::VidhiLin,
                Pada::Atmanepada,
                Purusha::Prathama,
                Vacana::Eka,
            )
        };
        assert!(p.log.iter().any(|s| s.sutra == "3.4.102"));
        assert!(p.log.iter().any(|s| s.sutra == "7.2.79"));
        assert!(p.log.iter().any(|s| s.sutra == "6.1.87"));
        assert_eq!(p.text(), "laBeta");
    }

    #[test]
    fn vrt_lat_uses_laghupadha_guna() {
        // vft's f is PENULTIMATE (upadha), not final like smf's: guna comes
        // from 7.3.86 pugantalaghUpaDasya ca, not 7.3.84.
        assert_eq!(lat_a_form("vft", Purusha::Prathama, Vacana::Eka), "vartate");
    }

    #[test]
    fn thasah_se_precedes_and_preempts_ter_e() {
        // 3.4.80 is the apavada: TAs -> se. Reversed order would give 3.4.79
        // TAs -> Te (wrong). And 3.4.79 must report false on "se" (ti of
        // "se" is already e) rather than record a no-op step.
        let mut p = Prakriya {
            terms: vec![Term::new("laB"), Term::new("se")],
            log: vec![],
            ctx: Context::new(
                Lakara::Lat,
                Pada::Atmanepada,
                Purusha::Madhyama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.79").unwrap();
        assert!(
            !(rule.apply)(&mut p),
            "3.4.79 must not record a no-op on se"
        );
    }

    #[test]
    fn ato_nitah_requires_the_ngit_tag() {
        // Parasmaipada lot uttama Ani starts with A but is NOT Nid-vat
        // (1.2.4 pic-ca exclusion) — it belongs to 6.1.101, not 7.2.81.
        let mut anga = Term::new("Bav");
        anga.add(Tag::Anga);
        let mut p = Prakriya {
            terms: vec![anga, Term::new("a"), Term::new("Ani")],
            log: vec![],
            ctx: Context::new(
                Lakara::Lot,
                Pada::Parasmaipada,
                Purusha::Uttama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.2.81").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "Ani");
    }

    #[test]
    fn eta_ai_requires_ending_to_actually_end_in_e() {
        // 3.4.93's guard is a 4-clause `||` chain: lakara != loT, purusha !=
        // uttama, pada != Atmanepada, or the ending isn't e-final -> return
        // false. Here the first three clauses are all false (loT, uttama,
        // Atmanepada all hold), so only the last clause -- ending
        // "sva" doesn't end in 'e' -- makes the guard true and the rule
        // report false, leaving "sva" untouched.
        //
        // The targeted mutant flips the LAST `||` to `&&`, turning the
        // guard into `c1 || c2 || (c3 && c4)`. With c1=c2=c3=false and
        // c4=true, that mutant guard evaluates to false -- the early
        // return is skipped, and the rule wrongly pops "sva"'s final char
        // and appends 'E', corrupting it to "svE". Asserting both the
        // false return AND the unchanged text kills the mutant.
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("sva")],
            log: vec![],
            ctx: Context::new(Lakara::Lot, Pada::Atmanepada, Purusha::Uttama, Vacana::Eka),
            blocked: false,
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.93").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "sva");
    }

    // --- 7.3.86 pugantalaGUpaDasya ca: guard-edge pins -------------------
    //
    // The guard `n < 2 || is_vowel(chars[n - 1])` followed by index
    // arithmetic on `chars[n - 2]` / `chars[..n - 2]` is reachable-
    // equivalent to its mutants for every curated aGga except the 3-char
    // "vft" (where n-2 == n/2), so each case below is a constructed
    // Prakriya chosen to separate the mutant from the original at a
    // different edge.

    #[test]
    fn pugantalaghupadhasya_one_char_anga_returns_false_without_panic() {
        // n=1: `n < 2` alone is true, so `||` short-circuits and the body
        // never touches `chars[n - 2]`. The `<` -> `==` mutant makes
        // `n == 2` false for n=1; evaluating the right disjunct then needs
        // `chars[n - 1]` (fine, n-1=0) but the guard as a whole is now
        // false, so the mutant falls through to `chars[n - 2]` with n=1,
        // a usize underflow that panics. The original must return false
        // cleanly.
        let mut p = Prakriya {
            terms: vec![Term::new("d"), Term::new("a")],
            log: vec![],
            ..Default::default()
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.3.86").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "d");
    }

    #[test]
    fn pugantalaghupadhasya_two_char_ik_penult_fires() {
        // n=2, final char 'd' is a consonant so the guard is false and the
        // rule fires: guNa of penult 'i' is "e", giving "ed". The
        // `<` -> `<=` mutant makes `n <= 2` true for n=2, so the mutant
        // guard short-circuits to true and wrongly returns false instead
        // of firing.
        let mut p = Prakriya {
            terms: vec![Term::new("id"), Term::new("a")],
            log: vec![],
            ..Default::default()
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.3.86").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "ed");
    }

    #[test]
    fn pugantalaghupadhasya_skips_vowel_final_anga() {
        // n=3, final char 'u' is a vowel, so the guard's `is_vowel` disjunct
        // is true and the rule must not fire (this shape is 7.3.84's
        // business). The `||` -> `&&` mutant makes the guard
        // `n < 2 && is_vowel(...)` = false && true = false, so the mutant
        // falls through and wrongly fires on the ik penult 'f'.
        let mut p = Prakriya {
            terms: vec![Term::new("Bfu"), Term::new("a")],
            log: vec![],
            ..Default::default()
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.3.86").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Bfu");
    }

    #[test]
    fn pugantalaghupadhasya_uses_n_minus_2_not_n_over_2() {
        // n=5 ("aBiur"): n-2=3 (penult 'u') but n/2=2 (chars[2]='i') --
        // these differ, so this case separates both `-` -> `/` mutants
        // from the original at once. By hand: guNa of chars[3]='u' is
        // "o"; laghu-check on chars[3]='u' passes; prefix is chars[..3]
        // = "aBi"; result = "aBi" + "o" + chars[4]='r' = "aBior".
        // Mutating `chars[n - 2]` (line 806) to `chars[n / 2]` would guNa
        // 'i' instead ("e"), yielding "aBier". Mutating `chars[..n - 2]`
        // (line 815) to `chars[..n / 2]` would prefix with "aB" instead
        // of "aBi", yielding "aBor". Both diverge from "aBior".
        let mut p = Prakriya {
            terms: vec![Term::new("aBiur"), Term::new("a")],
            log: vec![],
            ..Default::default()
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.3.86").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "aBior");
    }

    #[test]
    fn pugantalaghupadhasya_single_term_still_applies_guna() {
        // 7.3.86 has its own 1.1.5 (Girit) guard, `p.terms.len() > SHAP &&
        // p.terms[SHAP].has(Tag::Girit)`, mirroring 7.3.84's. Unlike
        // 7.3.84 (unreachable for divAdi/tudAdi, whose aGgas are all
        // consonant-final), 7.3.86's Girit-true branch IS reached by the
        // curated corpus (div, tud, juz, ...), so the `==`/`<` boundary
        // mutants are already caught there. Only the `>` -> `>=` mutant on
        // the `len() > SHAP` half survives: with len == 1 (no vikaraNa
        // term), the original short-circuits (`1 > 1` false) without
        // indexing terms[SHAP], so guNa proceeds normally: vft -> vart.
        // The mutant makes `1 >= 1` true, forcing an out-of-bounds index
        // into terms[SHAP] on a 1-element Vec, which panics.
        let mut p = Prakriya {
            terms: vec![Term::new("vft")],
            log: vec![],
            ..Default::default()
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.3.86").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "vart");
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
        let rule = TINANTA_RULES.iter().find(|r| r.id == "6.1.90").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kf");
        assert_eq!(p.terms[SHAP].text, "A");
    }

    // --- 3.1.68 / second 1.2.4: `len() > SHAP` boundary pins --------------
    //
    // Both guards read `p.terms.len() > SHAP && p.terms[SHAP]. ...` to
    // avoid indexing the not-yet-inserted vikaraNa slot. Every real
    // derivation always has an ending term present (terms.len() >= 2)
    // before either rule runs, so `> SHAP` (i.e. `> 1`) and `>= SHAP`
    // never diverge on any golden or negative derivation: len() is never
    // exactly 1 there. Pin the boundary directly with a single-term
    // Prakriya (aGga only, no ending) so the two outcomes diverge: the
    // original short-circuits before indexing terms[SHAP]; the `>` -> `>=`
    // mutant does not, and panics indexing out of bounds on a 1-element
    // Vec (an unexpected panic still fails the test).
    #[test]
    fn kartari_sap_single_term_anga_does_not_panic() {
        let mut p = Prakriya {
            terms: vec![Term::new("kf")],
            log: vec![],
            ..Default::default()
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.1.68").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kf");
        assert_eq!(p.terms[SHAP].text, "a");
    }

    #[test]
    fn sarvadhatukam_apit_second_application_single_term_does_not_panic() {
        // The SECOND "1.2.4" rule in TINANTA_RULES (the vikaraNa-Girit
        // application, ordered after 3.1.68) is targeted here, not the
        // first (ENDING_PRE_SHAP) application above the 3.1.68 boundary.
        let mut p = Prakriya {
            terms: vec![Term::new("kf")],
            log: vec![],
            ..Default::default()
        };
        assert_eq!(
            TINANTA_RULES.iter().filter(|r| r.id == "1.2.4").count(),
            2,
            "expected exactly two 1.2.4 rule entries; nth(1) locator assumes this"
        );
        let rule = TINANTA_RULES
            .iter()
            .filter(|r| r.id == "1.2.4")
            .nth(1)
            .unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kf");
    }

    // --- 2.4.72: `len() > SHAP` boundary + guard-order pins ----------------
    //
    // 2.4.72's guard is `len() > SHAP && has(Vikarana) && !text.is_empty()`,
    // each conjunct short-circuiting before the next would index the
    // not-yet-inserted vikaraNa slot. Every real derivation reaches this
    // rule only after 3.1.68 has already inserted Sap (terms.len() >= 2),
    // so `> SHAP` vs `>= SHAP`, and `&&` vs `||` at either join, never
    // diverge on any golden or negative derivation. Pin the boundary
    // directly: a single-term Prakriya (aGga only, tagged Adadi so the
    // outer gana guard passes) makes `len() > SHAP` (1 > 1) false, so the
    // original short-circuits before ever touching terms[SHAP]. Each of
    // the three mutants below removes a different short-circuit and
    // indexes terms[SHAP] out of bounds on this 1-element Vec, panicking:
    //   - `>` -> `>=`: `1 >= 1` is true, so `has(Vikarana)` is evaluated.
    //   - first `&&` -> `||`: `len() > SHAP` (false) forces evaluation of
    //     `has(Vikarana)` to resolve the OR.
    //   - second `&&` -> `||`: `(len() > SHAP && has(Vikarana))` (false)
    //     forces evaluation of `!text.is_empty()` to resolve the OR.
    // One construction catches all three.
    #[test]
    fn adiprabhrtibhyah_sapah_single_term_anga_does_not_panic() {
        let mut anga = Term::new("kf");
        anga.add(Tag::Adadi);
        let mut p = Prakriya {
            terms: vec![anga],
            log: vec![],
            ..Default::default()
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "2.4.72").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kf");
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
        let rule = TINANTA_RULES.iter().find(|r| r.id == "6.1.101").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "yA");
    }

    // --- 7.3.84 sArvaDAtukArDaDAtukayoH: 1.1.5 (Girit) guard pins ---------
    //
    // No curated divAdi/tudAdi root has a vowel-final aGga (they are all
    // consonant-final: div, naS, kup, man, yuD, vid, tud, liK, viS, juz,
    // vij, gur all end in a consonant), so 7.3.84's guNa-blocking business
    // — final-ik aGgas — is only ever reached by bhvAdi roots (BU, nI, ji,
    // smf), whose vikaraNa (Sap) is never Girit. The `has(Tag::Girit)`
    // guard's `true` branch is therefore never exercised by any golden or
    // negative derivation, and boundary mutants on `p.terms.len() > SHAP`
    // are invisible to the suite. Pin both edges directly.
    #[test]
    fn sarvadhatukardhadhatukayoh_blocks_guna_when_vikarana_is_ngit() {
        // Constructed vowel-final aGga ("nI") + a Girit vikaraNa (as Syan/
        // Sa would be via the second 1.2.4): guNa must be blocked. The
        // `>` -> `==` and `>` -> `<` mutants both make `len() > SHAP`
        // false at len=2, so the guard's early return is skipped and the
        // mutant wrongly applies guNa ("nI" -> "ne").
        let mut p = Prakriya {
            terms: vec![Term::new("nI"), Term::new("ya")],
            log: vec![],
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Ngit);
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.3.84").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "nI");
    }

    #[test]
    fn sarvadhatukardhadhatukayoh_single_term_anga_still_applies_guna() {
        // len == 1 (no vikaraNa term at all): the original guard
        // short-circuits (`1 > 1` is false) without indexing terms[SHAP],
        // so guNa proceeds normally: "nI" -> "ne". The `>` -> `>=` mutant
        // makes `1 >= 1` true, forcing an out-of-bounds index into
        // terms[SHAP] on a 1-element Vec, which panics (an unexpected
        // panic still fails the test).
        let mut p = Prakriya {
            terms: vec![Term::new("nI")],
            log: vec![],
            ..Default::default()
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.3.84").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "ne");
    }

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
        let rule = TINANTA_RULES.iter().find(|r| r.id == "8.2.77").unwrap();
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
        let rule = TINANTA_RULES.iter().find(|r| r.id == "8.2.77").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "aBiUr");
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
        let rule = TINANTA_RULES.iter().find(|r| r.id == "6.1.96").unwrap();
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

    #[test]
    fn savarna_dirgha_adadi_lin_1sg_arm() {
        let rule = TINANTA_RULES.iter().find(|r| r.id == "6.1.101").unwrap();

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

        // Declines: thematic liṅ (śap = `a`, non-empty) is never touched by
        // this arm — the SHAP-empty guard is what scopes it to adādi.
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
    // The 1sg arm's guard (line ~1025) is `len() > ENDING && lakara ==
    // VidhiLin && SHAP.is_empty() && ENDING.starts_with("yA") && ...`. A
    // 2-term Prakriya (aGga "yA" + an empty Sap slot, no ENDING term at
    // all) makes `len() > ENDING` (2 > 2) false in the original, so the
    // if-block short-circuits before ever indexing terms[ENDING]; control
    // falls through the second (pre-adAdi) arm (also guarded by the same
    // `len() > ENDING`, equally false) to the third (thematic) branch,
    // whose `!SHAP.text.ends_with('a')` is true for an empty SHAP and
    // short-circuits the `||` there too -- so the original returns false
    // with no panic. Unlike the existing two-term regression test for the
    // adAdi arm above, this one pins the lakara to VidhiLin: the `>` ->
    // `>=` mutant needs `lakara == VidhiLin` to be true to reach its
    // fourth conjunct, which indexes the nonexistent terms[ENDING] (index
    // 2 on a 2-element Vec) and panics. A default-lakara (Lat) Prakriya
    // would let the mutant's second conjunct short-circuit first and
    // never distinguish it -- this is why the earlier two-term test alone
    // didn't kill this mutant.
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
        let rule = TINANTA_RULES.iter().find(|r| r.id == "6.1.101").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "yA");
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
        let rule = TINANTA_RULES.iter().find(|r| r.id == "6.1.96").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "yAus");
    }
}
