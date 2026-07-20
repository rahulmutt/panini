use crate::context::Context;
use crate::controller::run_pipeline;
use crate::it_samjna::run_it_samjna;
use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use panini_data::{Dhatu, Lakara, Pada, Purusha, Vacana, tin_ending};

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

fn is_vowel(c: char) -> bool {
    matches!(
        c,
        'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O'
    )
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

/// The ordered rule list. Read it top to bottom against the Aṣṭādhyāyī: this
/// sequence IS the grammar this crate implements. Every rule self-guards and
/// returns whether it fired.
pub static TINANTA_RULES: &[Rule] = &[
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
    Rule {
        id: "3.4.101",
        name: "tasTasTamipAM tAMtaMtAmaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.ctx.is_ngit_like {
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
    // 3.4.86 er uḥ.
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
    // 3.4.92 āḍ uttamasya pic ca: the āṭ-āgama is prefixed to loṭ's uttama
    // endings. ni → Ani, va → Ava, ma → Ama.
    //
    // Guarded to exactly `ni`/`va`/`ma` rather than "any uttama ending in
    // loṭ": those forms only exist because 3.4.89 mer niḥ and 3.4.99 nityaṃ
    // ṅitaḥ have already normalized mi→ni and vas/mas→va/ma. The explicit set
    // makes the preemption independent of ordering accidents — MUST follow
    // 3.4.89 and 3.4.99, but the guard no longer silently depends on it.
    Rule {
        id: "3.4.92",
        name: "Aq uttamasya pic ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot)
                || !matches!(p.ctx.purusha, Purusha::Uttama)
                || !matches!(p.terms[ENDING_PRE_SHAP].text.as_str(), "ni" | "va" | "ma")
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
    // stable. "parasmaipadeṣu" is trivially satisfied — Pada has one variant;
    // revisit the guard when ātmanepada arrives (its liṅ takes sīyuṭ, 3.4.102).
    //
    // MUST follow the 3.4.9x/10x ending substitutions above: their guards
    // match the ending text exactly ("mi", "vas", …), so prefixing yAs first
    // would make every one of them miss.
    Rule {
        id: "3.4.103",
        name: "yAsuw parasmEpadezUdAtto Nic ca",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = format!("yAs{}", p.terms[ENDING_PRE_SHAP].text);
            p.record("3.4.103", "yAsuw parasmEpadezUdAtto Nic ca", before);
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
    // 3.1.68 kartari śap: insert śap between dhātu and ending, run it-samjña
    // on it (Sap → a), and mark the dhātu an aṅga.
    Rule {
        id: "3.1.68",
        name: "kartari Sap",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let before = p.snapshot();
            let mut s = Term::new("Sap");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.68", "kartari Sap", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP);
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
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
            if !matches!(p.ctx.lakara, Lakara::Lan) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = format!("a{}", p.terms[ANGA].text);
            p.record("6.4.71", "luNlaNlfNkzvaqudAttaH", before);
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
                || p.terms[SHAP].text != "a"
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
    // 7.3.84 sārvadhātukārdhadhātukayoḥ: guṇa of the aṅga's final ik.
    Rule {
        id: "7.3.84",
        name: "sArvaDAtukArDaDAtukayoH",
        kind: RuleKind::Vidhi,
        apply: |p| {
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
    // 6.1.78 eco'yavāyāvaḥ: e/o/E/O before a vowel → ay/av/Ay/Av.
    Rule {
        id: "6.1.78",
        name: "eco'yavAyAvaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let anga_last = p.terms[ANGA].text.chars().last().unwrap();
            let next_first = p.terms[SHAP].text.chars().next().unwrap();
            let sub = match anga_last {
                'e' => "ay",
                'o' => "av",
                'E' => "Ay",
                'O' => "Av",
                _ => return false,
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
            if !matches!(ending_first, 'm' | 'v') || p.terms[SHAP].text != "a" {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "A".into();
            p.record("7.3.101", "ato dIrGo yaYi", before);
            true
        },
    },
    // 6.1.101 akaḥ savarṇe dīrghaḥ: śap `a` + the ending's initial `A`
    // (from 3.4.92 āḍ) coalesce to a single `A`. Bav + a + Ani → BavAni.
    Rule {
        id: "6.1.101",
        name: "akaH savarRe dIrGaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms[SHAP].text != "a" || !p.terms[ENDING].text.starts_with('A') {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "A".into();
            p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
            p.record("6.1.101", "akaH savarRe dIrGaH", before);
            true
        },
    },
    // 6.1.97 ato guṇe: a short `a` (the śap) followed by a guṇa vowel yields
    // para-rūpa — a single vowel identical to the following one. For the `anti`
    // ending (Ji → anti), śap `a` + initial `a` of `anti` → a single short `a`
    // (NOT savarṇa-dīrgha `A`), so `Bav`+`a`+`nti` = `Bavanti`. Drop the
    // ending's leading `a`; the surviving śap `a` stands in for the coalesced
    // vowel and the term vector stays consistent for `.text()`.
    Rule {
        id: "6.1.97",
        name: "ato guRe",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if p.terms[SHAP].text != "a" || !p.terms[ENDING].text.starts_with('a') {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = p.terms[ENDING].text.chars().skip(1).collect();
            p.record("6.1.97", "ato guRe", before);
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
            if p.terms[SHAP].text != "a" || p.terms[ENDING].text != "hi" {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = String::new();
            p.record("6.4.105", "ato heH", before);
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
    /// roots and all three lakaras/nine cells, and assert every recorded
    /// `RuleStep.name` matches the `TINANTA_RULES` entry for its `sutra` id.
    #[test]
    fn recorded_step_names_match_tinanta_rules_for_every_id() {
        let lakaras = [Lakara::Lat, Lakara::Lan, Lakara::Lot];
        let purushas = [Purusha::Prathama, Purusha::Madhyama, Purusha::Uttama];
        let vacanas = [Vacana::Eka, Vacana::Dvi, Vacana::Bahu];

        let mut steps_checked = 0usize;
        for d in dhatus() {
            for &lakara in &lakaras {
                for &purusha in &purushas {
                    for &vacana in &vacanas {
                        let p = derive(d, lakara, Pada::Parasmaipada, purusha, vacana);
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
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "3.4.101").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "am");
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
        };
        let rule = TINANTA_RULES.iter().find(|r| r.id == "7.2.80").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "iyt");
    }
}
