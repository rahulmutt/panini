use crate::context::Context;
use crate::controller::run_pipeline;
use crate::it_samjna::run_it_samjna;
use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use panini_data::{Dhatu, Gana, Lakara, Pada, Purusha, Vacana, tin_ending};

mod adesha;
mod anga;
mod sound;
mod terms;
mod tripadi;
mod vikarana;

pub(crate) use sound::{is_vibhakti_protected_final, is_vowel};
pub(crate) use terms::{ANGA, ENDING_PRE_SHAP};
/// The shared derivation helper, re-exported here so the stage files' test
/// modules import it by a stable path (`crate::tinanta::form_g`) rather than
/// reaching into this module's private `tests` by path. When the helpers move
/// to `derivation_tests.rs`, only this line changes.
#[cfg(test)]
pub(crate) use tests::form_g;

/// Every rule not yet extracted into its own stage file. Shrinks to nothing
/// as the split proceeds; delete this constant when the last stage moves out.
///
/// Every rule here is ordered BEFORE 3.1.68 — the ending is at
/// `ENDING_PRE_SHAP` (index 1), and `SHAP`/`ENDING` do not yet exist. See
/// `terms`.
static UNSPLIT: &[Rule] = &[
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
];

/// The ordered rule list, as a sequence of pipeline stages. Read the stages
/// in order, and the rules within each stage in order: that flattened
/// sequence IS the grammar this crate implements. Every rule self-guards and
/// returns whether it fired.
pub static TINANTA_RULES: &[&[Rule]] = &[
    UNSPLIT,
    vikarana::VIKARANA,
    anga::ANGA_RULES,
    adesha::ADESHA,
    tripadi::TRIPADI,
];

/// The rules in pipeline order, flattened across stages.
pub fn rules() -> impl Iterator<Item = &'static Rule> {
    TINANTA_RULES.iter().flat_map(|stage| stage.iter())
}

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
    // Only `tripadi.rs`'s rules use these now, so they are no longer
    // re-exported from this module; the unit tests below still cover them.
    use crate::tinanta::sound::cartva_of;
    use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};

    fn form(code: &str, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, Lakara::Lat, Pada::Parasmaipada, pu, va).text()
    }

    // `pub(crate)` so the parent module can re-export it as
    // `crate::tinanta::form_g`; `anga.rs` and `tripadi.rs` import it by that
    // stable path.
    pub(crate) fn form_g(code: &str, la: Lakara, pu: Purusha, va: Vacana) -> String {
        let d = dhatus().iter().find(|d| d.code == code).unwrap();
        derive(d, la, d.pada, pu, va).text()
    }

    /// The ordered rule list IS the grammar this crate implements, so its
    /// sequence is pinned verbatim, not merely by the per-derivation traces
    /// in `crates/panini/tests/trace.rs` (which only pin order along the
    /// paths representative forms happen to take).
    ///
    /// If you add a rule, add its id here in position. If this test fails
    /// after a refactor that was supposed to move code without changing it,
    /// the refactor reordered the grammar — fix the refactor, not this list.
    ///
    /// 1.2.4 appears twice, deliberately: once tagging apit ātmanepada
    /// endings, once tagging the apit vikaraṇa after 3.1.68 inserts it.
    #[test]
    fn tinanta_rule_order_is_pinned() {
        let expected = [
            "1.3.12", "1.3.78", "3.4.78", "1.3.9", "1.2.4", "3.4.85", "3.4.108", "3.4.105",
            "3.4.106", "3.4.101", "3.4.99", "3.4.87", "3.4.89", "3.4.86", "3.4.100", "3.4.80",
            "3.4.79", "3.4.91", "3.4.93", "3.4.90", "3.4.92", "3.4.103", "3.4.102", "3.1.69",
            "3.1.77", "3.1.68", "2.4.72", "1.2.4", "6.4.71", "6.4.72", "7.3.100", "7.1.5", "7.1.6",
            "7.1.3", "7.2.79", "7.2.80", "7.2.81", "7.4.21", "7.3.84", "7.3.86", "6.1.78",
            "7.3.101", "6.1.101", "6.1.96", "6.1.90", "6.1.97", "6.1.87", "6.1.66", "6.4.105",
            "6.4.101", "8.2.77", "8.2.23", "8.2.25", "8.3.15", "8.3.59", "8.4.55",
        ];
        let actual: Vec<&str> = rules().map(|r| r.id).collect();
        assert_eq!(actual, expected);
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
    fn her_dhih_gives_addhi_for_consonant_root() {
        // √ad loṭ 2sg: 3.4.87 si→hi, 6.4.105 declines (d, not short a),
        // 6.4.101 hi→Di → adDi.
        assert_eq!(
            form_g("ad", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
            "adDi"
        );
        // Thematic root unaffected: √bhū loṭ 2sg is Bava (hi luk'd by 6.4.105).
        assert_eq!(
            form_g("BU", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
            "Bava"
        );
    }

    #[test]
    fn adadi_lan_singular_a_augment() {
        // √ad laṅ 3sg Adat, 2sg AdaH — the inserted `a` blocks the saṃyogānta
        // collapse (Adt/Ads → Ad) and cartva (d now before `a`, not a khar).
        assert_eq!(
            form_g("ad", Lakara::Lan, Purusha::Prathama, Vacana::Eka),
            "Adat"
        );
        assert_eq!(
            form_g("ad", Lakara::Lan, Purusha::Madhyama, Vacana::Eka),
            "AdaH"
        );
        // Dual/plural keep the direct junction (multi-char endings, no a-augment):
        // cartva gives Attam/AttAm; 1sg Adam untouched.
        assert_eq!(
            form_g("ad", Lakara::Lan, Purusha::Madhyama, Vacana::Dvi),
            "Attam"
        );
        assert_eq!(
            form_g("ad", Lakara::Lan, Purusha::Prathama, Vacana::Dvi),
            "AttAm"
        );
        assert_eq!(
            form_g("ad", Lakara::Lan, Purusha::Uttama, Vacana::Eka),
            "Adam"
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
        let rule = rules().find(|r| r.id == "1.3.9").unwrap();
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
        let rule = rules().find(|r| r.id == "3.4.108").unwrap();
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
            let rule = rules().find(|r| r.id == "3.4.108").unwrap();
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
                            let rule = rules().find(|r| r.id == step.sutra).unwrap_or_else(|| {
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
        for rule in rules() {
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
        let rule = rules().find(|r| r.id == "3.4.100").unwrap();
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
        let rule = rules().find(|r| r.id == "3.4.100").unwrap();
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
        let rule = rules().find(|r| r.id == "3.4.101").unwrap();
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
        let rule = rules().find(|r| r.id == "3.4.92").unwrap();
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
        let rule = rules().find(|r| r.id == "3.4.103").unwrap();
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
        let rule = rules().find(|r| r.id == "3.4.103").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "t");
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
            let rule = rules().find(|r| r.id == id).unwrap();
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
        let rule = rules().find(|r| r.id == "1.2.4").unwrap();
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
            let rule = rules().find(|r| r.id == "1.2.4").unwrap();
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
    fn as_lot_atmanepada_uttama_eka_is_ase() {
        // adādi √ās (śap luk'd) loṭ uttama-eka: the āṭ A never merged into a
        // śap (there is none), so it leads the ending as `A E`. 6.1.90's
        // athematic ending arm must vṛddhi it to a single E (Asai → AsE).
        // Bug: AsAE (āsāai). Cf. the thematic laBE.
        assert_eq!(
            form_g("As", Lakara::Lot, Purusha::Uttama, Vacana::Eka),
            "AsE"
        );
    }

    #[test]
    fn as_vidhilin_atmanepada_elides_optative_y_before_val() {
        // adādi √ās vidhiliṅ: on the śap-luk'd path 6.1.87 never consumes the
        // optative I, so the ending stays `I y val`; 6.1.66 must elide the y
        // (Iyta → Ita) before a val consonant — but KEEP it before a vowel.
        for (pu, va, want) in [
            (Purusha::Prathama, Vacana::Eka, "AsIta"),
            (Purusha::Prathama, Vacana::Bahu, "AsIran"),
            (Purusha::Madhyama, Vacana::Eka, "AsITAH"),
            (Purusha::Madhyama, Vacana::Bahu, "AsIDvam"),
            (Purusha::Uttama, Vacana::Dvi, "AsIvahi"),
            (Purusha::Uttama, Vacana::Bahu, "AsImahi"),
        ] {
            assert_eq!(
                form_g("As", Lakara::VidhiLin, pu, va),
                want,
                "{pu:?} {va:?}"
            );
        }
        // Guard boundary: the y survives before a vowel (must NOT over-elide).
        for (pu, va, want) in [
            (Purusha::Uttama, Vacana::Eka, "AsIya"),
            (Purusha::Prathama, Vacana::Dvi, "AsIyAtAm"),
            (Purusha::Madhyama, Vacana::Dvi, "AsIyATAm"),
        ] {
            assert_eq!(
                form_g("As", Lakara::VidhiLin, pu, va),
                want,
                "{pu:?} {va:?}"
            );
        }
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
        let rule = rules().find(|r| r.id == "3.4.79").unwrap();
        assert!(
            !(rule.apply)(&mut p),
            "3.4.79 must not record a no-op on se"
        );
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
        let rule = rules().find(|r| r.id == "3.4.93").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "sva");
    }

    #[test]
    fn eco_yavayavah_athematic_arm_produces_the_ay_adesha() {
        // 6.1.78's athematic arm (śap luk'd, adādi): with no vikaraṇa buffer,
        // the ending attaches directly to the aṅga, so the ending's own
        // first character is the "next" vowel this sūtra tests. √śī laṭ
        // prathama-dvi: guṇa (7.4.21) has already made the aṅga `Se`, and the
        // ending is the vowel-initial `Ate`; this arm must turn `Se` into
        // `Say` (Se + Ate → Say + Ate → SayAte), the same mechanism that
        // gives vidhiliṅ 3pl its `SayIran` (Se + Iyran → Say + Iyran →
        // 6.1.66 → SayIran).
        assert_eq!(
            form_g("SI", Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
            "SayAte"
        );
        assert_eq!(
            form_g("SI", Lakara::VidhiLin, Purusha::Prathama, Vacana::Bahu),
            "SayIran"
        );
    }

    #[test]
    fn shi_takes_guna_despite_the_ngit_ending() {
        // 7.4.21 śīṅaḥ sārvadhātuke guṇaḥ: √śī guṇates (SI → Se) even though
        // the ātmanepada endings are ṅit (1.2.4) and 1.1.5 would otherwise
        // block guṇa. This is the only visible guṇa in the whole adādi gaṇa.
        //
        // Note: the surface forms below (`Sete`, `aSeta`) now pin
        // attribution as well as shape. Before 7.3.84's 1.1.5 guard was
        // rewired to `following_sarvadhatuka`, it read only the (always-pit)
        // SHAP term and never actually blocked on this śap-luk'd path, so
        // removing 7.4.21 would have left 7.3.84 to fire unaided and produce
        // these same two forms. Now the ṅit ending IS the immediate
        // follower, so 1.1.5 blocks 7.3.84 here too: removing 7.4.21 would
        // leave the aṅga un-guṇated (`SIte`/`aSIta`), not `Sete`/`aSeta`.
        // The attribution is still pinned independently by the
        // ordered-trace test `shete_trace_is_the_minimal_shing_guna_path` in
        // `crates/panini/tests/trace.rs`.
        assert_eq!(
            form_g("SI", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "Sete"
        );
        // laṅ: 6.4.71 has already prefixed the aṭ-augment, so the aṅga is
        // `aSI` when 7.4.21 runs — the guard must match on the tail, not the
        // whole string.
        assert_eq!(
            form_g("SI", Lakara::Lan, Purusha::Prathama, Vacana::Eka),
            "aSeta"
        );
    }

    // --- cartva, her-dhih, a-augment guard pins for 5c adadi rules -------
    #[test]
    fn cartva_guard_is_khar_only_not_m_or_vowel() {
        // Over-application killer: d before `m` (admi) or vowel (adanti) must NOT
        // cartva-ize. Under-application killer: d before `t` MUST (atti, not adti).
        assert_eq!(
            form_g("ad", Lakara::Lat, Purusha::Uttama, Vacana::Eka),
            "admi"
        );
        assert_eq!(
            form_g("ad", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "adanti"
        );
        assert_eq!(
            form_g("ad", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "atti"
        );
    }

    #[test]
    fn dhi_ca_elides_s_before_dhve() {
        // √ās 2pl: the root-final `s` meets the `Dh` of Dve/Dvam and is
        // ELIDED by 8.2.25 dhi ca — it is not voiced to `d`. 8.2.25 sits at
        // 8.2 in the tripādī and is asiddha to 8.4, so the `s` is gone before
        // any 8.4 junction rule can look at it: As + Dve -> A + Dve -> ADve.
        assert_eq!(
            form_g("As", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
            "ADve"
        );
        assert_eq!(
            form_g("As", Lakara::Lan, Purusha::Madhyama, Vacana::Bahu),
            "ADvam"
        );
        assert_eq!(
            form_g("As", Lakara::Lot, Purusha::Madhyama, Vacana::Bahu),
            "ADvam"
        );
        // Guard boundary: the affix must be Dh-initial. A clean `s`-meets-`s`
        // cell is untouched, so 2sg stays Asse — the rule must not
        // over-apply.
        assert_eq!(
            form_g("As", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
            "Asse"
        );
    }

    #[test]
    fn dhi_ca_fires_for_vas_and_only_before_dh() {
        // √vas is the sūtra's second witness: vas + Dve -> va + Dve -> vaDve.
        assert_eq!(
            form_g("vas", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
            "vaDve"
        );
        assert_eq!(
            form_g("vas", Lakara::Lan, Purusha::Madhyama, Vacana::Bahu),
            "avaDvam"
        );
        assert_eq!(
            form_g("vas", Lakara::Lot, Purusha::Madhyama, Vacana::Bahu),
            "vaDvam"
        );
        // The affix must be Dh-initial. These four cells put the same aṅga-
        // final `s` in front of `t`, `T` and `s` and it must survive intact —
        // and they are also the first pins that cartva (8.4.55) leaves an `s`
        // alone before a khar, an arm √ad and √ās could not reach.
        assert_eq!(
            form_g("vas", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "vaste"
        );
        assert_eq!(
            form_g("vas", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
            "vasse"
        );
        assert_eq!(
            form_g("vas", Lakara::Lan, Purusha::Madhyama, Vacana::Eka),
            "avasTAH"
        );
        assert_eq!(
            form_g("vas", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
            "vassva"
        );
    }

    #[test]
    fn dhi_ca_does_not_elide_a_non_s_before_dh() {
        // Thematic ātmanepada √labh keeps its śap `a` in front of Dve/Dvam.
        // √labh is bhvādi, so śap is never luk'd: the first non-empty term
        // after the aṅga is the śap `a` itself, and `"a".starts_with('D')` is
        // false — the guard declines at its FIRST arm (the `D`-initial
        // affix search), never reaching the "preceding term ends in `s`"
        // arm at all. These are the slice-3 goldens, unchanged.
        assert_eq!(
            form_g("laB", Lakara::Lat, Purusha::Madhyama, Vacana::Bahu),
            "laBaDve"
        );
        assert_eq!(
            form_g("laB", Lakara::Lan, Purusha::Madhyama, Vacana::Bahu),
            "alaBaDvam"
        );
        // adDi (√ad loṭ 2sg, pinned at paradigm level in
        // crates/panini/tests/paradigm.rs) is the one cell in the current
        // root set that actually reaches the "ends in `s`" arm: śap is
        // luk'd (adādi) so ENDING (`Di`, from 6.4.101 her dhiḥ) is the first
        // non-empty term after the aṅga and the `D`-initial arm PASSES; the
        // preceding non-empty term is the aṅga `ad`, which does not end in
        // `s`, so the second arm declines and the `d` survives. Dropping
        // this arm would wrongly yield *aDi.
        assert_eq!(
            form_g("ad", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
            "adDi"
        );
    }

    #[test]
    fn dhi_ca_reads_the_affixs_neighbour_not_the_anga() {
        // vidhiliṅ 2pl: the sīyuṭ augment is prefixed INTO the ending's text
        // (`IDvam`), not a separate term, so `terms` is still the plain
        // three-element `[ANGA, SHAP, ENDING]` layout here. The first
        // non-empty term after the aṅga is the ending `IDvam` itself, and
        // `"IDvam".starts_with('D')` is false — these two pins decline at the
        // guard's FIRST arm (the `D`-initial affix search), the same arm as
        // laB above, not because the guard is reading some other neighbour
        // instead of the aṅga. In today's layout the backward search always
        // resolves to the aṅga whenever the first arm passes; it is written
        // to walk to the nearest non-empty term (rather than index ANGA
        // directly) for the multi-term layouts a later slice will bring —
        // mirroring vidyut-prakriya's own `prev_not_empty`. As + I + Dvam ->
        // AsIDvam (never *AIDvam): the `s` is retained because the affix
        // search declines, not because of anything more subtle.
        assert_eq!(
            form_g("As", Lakara::VidhiLin, Purusha::Madhyama, Vacana::Bahu),
            "AsIDvam"
        );
        assert_eq!(
            form_g("vas", Lakara::VidhiLin, Purusha::Madhyama, Vacana::Bahu),
            "vasIDvam"
        );
    }

    #[test]
    fn cartva_of_maps_each_jhal_to_its_first_varga_car() {
        // 8.4.55 car table. Only d→t is exercised by √ad this slice; the other
        // vargas are written generally for later jhal-final roots. Pin the whole
        // mapping so a dropped/altered arm is caught by mutation testing.
        for c in ['d', 'D', 't', 'T'] {
            assert_eq!(cartva_of(c), Some('t'), "{c}");
        }
        for c in ['g', 'G', 'k', 'K'] {
            assert_eq!(cartva_of(c), Some('k'), "{c}");
        }
        for c in ['b', 'B', 'p', 'P'] {
            assert_eq!(cartva_of(c), Some('p'), "{c}");
        }
        for c in ['j', 'J', 'c', 'C'] {
            assert_eq!(cartva_of(c), Some('c'), "{c}");
        }
        for c in ['q', 'Q', 'w', 'W'] {
            assert_eq!(cartva_of(c), Some('w'), "{c}");
        }
        // Non-car targets return None (e.g. h, sibilants, vowels).
        for c in ['h', 'S', 'z', 's', 'a'] {
            assert_eq!(cartva_of(c), None, "{c}");
        }
    }

    #[test]
    fn her_dhih_guard_is_jhal_final_only() {
        // ā-final √yā loṭ 2sg keeps hi (yAhi), NOT *yADi: 6.4.101 needs a jhal.
        assert_eq!(
            form_g("yA", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
            "yAhi"
        );
    }

    #[test]
    fn a_augment_does_not_leak_into_dual_or_plural() {
        // The single-char length guard: 2du ending `tam` must NOT get an `a`
        // (no *Adatam); it stays Attam via cartva.
        assert_eq!(
            form_g("ad", Lakara::Lan, Purusha::Madhyama, Vacana::Dvi),
            "Attam"
        );
    }

    #[test]
    fn seventwone_five_atmanepada_3pl_uses_at_not_ant() {
        // 7.1.5 ātmanepadeṣv anataḥ: √ās (adādi, s-final) 3pl → Asate/Asata/
        // AsatAm (Ja → at, not the `ant` of 7.1.3). A-final thematic roots keep
        // `ante` (7.1.5 declines), so laB is unchanged.
        assert_eq!(
            form_g("As", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "Asate"
        );
        assert_eq!(
            form_g("As", Lakara::Lan, Purusha::Prathama, Vacana::Bahu),
            "Asata"
        );
        assert_eq!(
            form_g("As", Lakara::Lot, Purusha::Prathama, Vacana::Bahu),
            "AsatAm"
        );
        // Guard boundary: a-final ātmanepada aṅga still takes 7.1.3's `ante`.
        assert_eq!(
            form_g("laB", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "laBante"
        );
    }

    #[test]
    fn anatah_declines_for_a_final_atmanepada_angas() {
        // 7.1.5's "anataḥ" arm: every a-final (thematic / vikaraṇa-buffered)
        // ātmanepada 3pl keeps 7.1.3's `ante`. Pins that the guard reads the
        // preceding segment's `a`, not the consonant-final root.
        assert_eq!(
            form_g("laB", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "laBante"
        );
        assert_eq!(
            form_g("man", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "manyante"
        );
        assert_eq!(
            form_g("juz", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "juzante"
        );
    }

    #[test]
    fn voiced_junction_does_not_touch_non_jhas_or_non_jhal_junctions() {
        // Under-application guard: `s` before the non-jhaś `s`/`th`/`v` of
        // se/sva/thās stays `s` (Asse, Assva, AsTAH) — only a jhaś triggers it.
        assert_eq!(
            form_g("As", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
            "Assva"
        );
        assert_eq!(
            form_g("As", Lakara::Lan, Purusha::Madhyama, Vacana::Eka),
            "AsTAH"
        );
        // Over-application guard: √ad is parasmaipada; its voiceless junctions
        // stay cartva's business:
        assert_eq!(
            form_g("ad", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "atti"
        );
    }

    #[test]
    fn shings_jha_takes_the_rut_augment() {
        // 7.1.6 śīṅo ruṭ: the *jha* (3pl ātmanepada) of √śī takes the ruṭ
        // augment. 7.1.5 has just turned the leading J into `at` (Je → ate);
        // ruṭ's `r` prefixes that: Se + r + ate → Serate.
        assert_eq!(
            form_g("SI", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "Serate"
        );
        assert_eq!(
            form_g("SI", Lakara::Lot, Purusha::Prathama, Vacana::Bahu),
            "SeratAm"
        );
        assert_eq!(
            form_g("SI", Lakara::Lan, Purusha::Prathama, Vacana::Bahu),
            "aSerata"
        );
    }

    #[test]
    fn shings_vidhilin_3pl_takes_no_rut() {
        // 3.4.105 jhasya ran replaces the jha with `ran` long before the 7.x
        // band, so 7.1.5 never fires in vidhiliṅ and ruṭ cannot attach:
        // SayIran, NOT *SayIraran.
        assert_eq!(
            form_g("SI", Lakara::VidhiLin, Purusha::Prathama, Vacana::Bahu),
            "SayIran"
        );
    }

    #[test]
    fn shatva_retroflexes_the_endings_s_after_shings_e() {
        // 8.3.59 ādeśapratyayayoḥ: the `s` of a pratyaya retroflexes after a
        // non-a/ā vowel. With the aṅga guṇated to `Se`, the `se` and `sva`
        // endings meet an `e` → Seze, Sezva.
        assert_eq!(
            form_g("SI", Lakara::Lat, Purusha::Madhyama, Vacana::Eka),
            "Seze"
        );
        assert_eq!(
            form_g("SI", Lakara::Lot, Purusha::Madhyama, Vacana::Eka),
            "Sezva"
        );
    }
}
