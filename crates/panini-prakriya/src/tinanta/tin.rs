//! Lakāra → tiṅ substitution and ending reshaping: 3.4.85 … 3.4.102.
//!
//! Ordered **BEFORE** 3.1.68, so every rule here addresses the ending as
//! `ENDING_PRE_SHAP` (index 1) — śap does not exist yet, and `ENDING`
//! (index 2) would panic. See `super::terms`.
//!
//! The split from `super::samjna` falls at 3.4.78, which is what *inserts*
//! the ending; everything from 3.4.85 on substitutes and reshapes it.

use crate::rule::{Rule, RuleKind};
use crate::term::Tag;
use crate::tinanta::sound::is_vowel;
use crate::tinanta::terms::ENDING_PRE_SHAP;
use panini_data::{Lakara, Pada, Purusha};

pub(crate) static TIN: &[Rule] = &[
    // 3.4.85 loṭo laṅvat: loṭ behaves as laṅ, so the ṅit-conditioned rules
    // (3.4.99, 3.4.101) apply to it. An atideśa, so it is a rule and appears
    // in the trace rather than being folded into Context::new.
    Rule {
        id: "3.4.85",
        name: "loTo laNvat",
        kind: RuleKind::Atidesha,
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lot) || p.terms[ENDING_PRE_SHAP].text != "si" {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = "hi".into();
            // "apit ca": hi is apit, so 1.2.4's atideśa reaches it. sip
            // arrived pit from 3.4.78; clear that before adding the ṅit, or
            // the term claims both. 1.2.4 has already run by now (samjna
            // stage), so the tag is set here rather than left to it.
            p.terms[ENDING_PRE_SHAP].remove(Tag::Pit);
            p.terms[ENDING_PRE_SHAP].add(Tag::Ngit);
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
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin) || matches!(p.ctx.pada, Pada::Atmanepada) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING_PRE_SHAP].text = format!("yAs{}", p.terms[ENDING_PRE_SHAP].text);
            // "Nic ca": yāsuṭ is ṅit, and the ending it augments is ṅit with
            // it. This is what 6.4.113 reads to give kliSnIyAt; tip's own pit
            // tag is left in place, since 1.1.5 asks only about ṅit.
            p.terms[ENDING_PRE_SHAP].add(Tag::Ngit);
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
        vikalpa: false,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::prakriya::Prakriya;
    use crate::term::Term;
    use crate::tinanta::rules;
    use panini_data::{Pada, Purusha, Vacana};

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
    fn ser_hyapic_ca_makes_hi_apit_and_ngit() {
        // "ser hi apit ca": the sutra names hi as apit in its own text. sip
        // arrives pit (3.4.78), so 3.4.87 must clear that and tag Nit --
        // otherwise 6.4.113 declines and vrIRIhi surfaces as *vrIRAhi.
        let mut p = Prakriya {
            ctx: Context::new(
                Lakara::Lot,
                Pada::Parasmaipada,
                Purusha::Madhyama,
                Vacana::Eka,
            ),
            ..Default::default()
        };
        p.terms.push(Term::new("vrI"));
        for id in ["3.4.78", "1.3.9", "1.2.4", "3.4.85", "3.4.87"] {
            let rule = rules().find(|r| r.id == id).unwrap();
            (rule.apply)(&mut p);
        }
        assert_eq!(p.terms[ENDING_PRE_SHAP].text, "hi");
        assert!(!p.terms[ENDING_PRE_SHAP].has(Tag::Pit));
        assert!(p.terms[ENDING_PRE_SHAP].has(Tag::Ngit));
    }

    #[test]
    fn yasut_is_ngit() {
        // 3.4.103's own name ends "Nic ca". Without the tag, 6.4.113 declines
        // in vidhilin and kliSnIyAt surfaces as *kliSnAyAt.
        let mut p = Prakriya {
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            ..Default::default()
        };
        p.terms.push(Term::new("kliS"));
        for id in ["3.4.78", "1.3.9", "1.2.4", "3.4.100", "3.4.103"] {
            let rule = rules().find(|r| r.id == id).unwrap();
            (rule.apply)(&mut p);
        }
        assert!(p.terms[ENDING_PRE_SHAP].has(Tag::Ngit));
    }
}
