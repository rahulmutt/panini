//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, ṇic's
//! it-lopa (1.3.9), 3.4.114, 7.2.116, 7.3.86, 3.1.32.
//!
//! First in the pipeline, before any lakāra or tiṅ exists. The layout here
//! is `[AGAMA, ABHYASA, ANGA, ṇic]`, ṇic at `NIC`; 3.1.32 folds ṇic into
//! `ANGA` and removes it, so `super::samjna` starts on the same
//! `[AGAMA, ABHYASA, ANGA]` every other gaṇa does. From there on the aṅga is
//! an ordinary i-final dhātu (`cori`), and 7.3.84 then 6.1.78 make `coray-`
//! exactly as they make √nī's `nay-`. See `super::terms`.
//!
//! Every rule self-guards: 3.1.25 on `Tag::Curadi`, the rest on ṇic being
//! present. For gaṇas 1–9 the stage adds nothing and records nothing.

use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use crate::tinanta::sound::guna_of;
use crate::tinanta::terms::{ANGA, NIC};

pub(crate) static SANADI: &[Rule] = &[
    // 3.1.25 satyāpapāśarūpavīṇātūlaślokasenālomatvacavarmavarṇacūrṇa-
    // curādibhyo ṇic: ṇic after a curādi root. The sūtra's nominal bases
    // (satya, pāśa, …) are not in scope; the gaṇa is.
    Rule {
        id: "3.1.25",
        name: "satyApapASarUpavIRAtUlaSlokasenAlomatvacavarmavarRacUrRacurAdiByo Ric",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Curadi) {
                return false;
            }
            let before = p.snapshot();
            let mut nic = Term::new("Ric");
            nic.add(Tag::Pratyaya);
            p.terms.push(nic);
            p.record(
                "3.1.25",
                "satyApapASarUpavIRAtUlaSlokasenAlomatvacavarmavarRacUrRacurAdiByo Ric",
                before,
            );
            true
        },
    },
    // 1.3.9 tasya lopaḥ, on ṇic: 1.3.7 cuṭū makes the initial ṇ an it, 1.3.3
    // halantyam the final c, and both go, leaving `i`. Local to ṇic, not
    // `it_samjna::run_it_samjna`: that helper has no 1.3.7 arm, and a
    // general one would strip the jh of the tiṅ ending `Ji`, which the
    // tradition exempts. The ṇ is recorded as `Tag::Rit` for 7.2.116. One
    // `1.3.9` step, the engine's it-lopa trace convention.
    Rule {
        id: "1.3.9",
        name: "tasya lopaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let Some(nic) = p.terms.get(NIC).filter(|t| t.text == "Ric") else {
                return false;
            };
            let before = p.snapshot();
            let mut nic = nic.clone();
            nic.text = "i".to_string();
            nic.add(Tag::Rit);
            p.terms[NIC] = nic;
            p.record("1.3.9", "tasya lopaH", before);
            true
        },
    },
    // 3.4.114 ārdhadhātukaṃ śeṣaḥ: ṇic is neither tiṅ nor śit, so it is
    // ārdhadhātuka. Read by 7.3.86 below, whose *sārvadhātukārdhadhātukayoḥ*
    // is inherited from 7.3.84.
    Rule {
        id: "3.4.114",
        name: "ArDaDAtukaM SezaH",
        kind: RuleKind::Samjna,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) {
                return false;
            }
            let before = p.snapshot();
            p.terms[NIC].add(Tag::Ardhadhatuka);
            p.record("3.4.114", "ArDaDAtukaM SezaH", before);
            true
        },
    },
    // 7.2.116 ata upadhāyāḥ: vṛddhi of an `a` upadhā before a ñit or ṇit
    // affix. `laq` → `lAq` before ṇic. Only ṇit has a carrier in this engine
    // (`Tag::Rit`); no ñit affix is in scope.
    Rule {
        id: "7.2.116",
        name: "ata upaDAyAH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) {
                return false;
            }
            let mut chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let n = chars.len();
            if n < 2 || chars[n - 2] != 'a' {
                return false;
            }
            let before = p.snapshot();
            chars[n - 2] = 'A';
            p.terms[ANGA].text = chars.into_iter().collect();
            p.record("7.2.116", "ata upaDAyAH", before);
            true
        },
    },
    // 7.3.86 pugantalaghūpadhasya ca, before ṇic: guṇa of a laghu ik upadhā
    // before an ārdhadhātuka. `cur` → `cor`. A third entry under this id —
    // the guṇa-stage pair (nitya, and tanādi's vikalpa) reads the aṅga
    // before the vikaraṇa, by which point a curādi aṅga is the vowel-final
    // `cori` and their "final-vowel aṅgas are 7.3.84's business" early
    // return declines. This entry is the one occasion that sees the root
    // with ṇic beside it.
    Rule {
        id: "7.3.86",
        name: "pugantalaGUpaDasya ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Ardhadhatuka)) {
                return false;
            }
            let chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let n = chars.len();
            if n < 2 || !matches!(chars[n - 2], 'i' | 'u' | 'f' | 'x') {
                return false;
            }
            let g = guna_of(chars[n - 2]).expect("an ik vowel has a guṇa");
            let before = p.snapshot();
            let mut s: String = chars[..n - 2].iter().collect();
            s.push_str(g);
            s.push(chars[n - 1]);
            p.terms[ANGA].text = s;
            p.record("7.3.86", "pugantalaGUpaDasya ca", before);
            true
        },
    },
    // 3.1.32 sanādyantā dhātavaḥ: root + ṇic is a dhātu. Fold ṇic's text into
    // `ANGA` and remove its term, so every later stage sees the three-slot
    // layout and an i-final dhātu. `Tag::Dhatu` stays; `Tag::Nijanta`
    // records that the final `i` is ṇic's.
    Rule {
        id: "3.1.32",
        name: "sanAdyantA DAtavaH",
        kind: RuleKind::Samjna,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) {
                return false;
            }
            let before = p.snapshot();
            let nic = p.terms.remove(NIC);
            p.terms[ANGA].text.push_str(&nic.text);
            p.terms[ANGA].add(Tag::Nijanta);
            p.record("3.1.32", "sanAdyantA DAtavaH", before);
            true
        },
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prakriya::Prakriya;
    use crate::tinanta::terms::with_slots;

    /// The sanādi stage's own entry for `id` — looked up in `SANADI`, not
    /// `rules()`, so it can never be another stage's 1.3.9 or 7.3.86.
    fn rule(id: &str) -> &'static Rule {
        SANADI.iter().find(|r| r.id == id).unwrap()
    }

    /// `root`, as a curādi dhātu, alone or followed by ṇic as it stands
    /// after its it-lopa (`i`, ṇit), optionally already ārdhadhātuka.
    fn with_nic(root: &str, nic: Option<&[Tag]>) -> Prakriya {
        let mut anga = Term::new(root);
        anga.add(Tag::Dhatu);
        anga.add(Tag::Curadi);
        let mut terms = vec![anga];
        if let Some(tags) = nic {
            let mut t = Term::new("i");
            t.add(Tag::Pratyaya);
            for tag in tags {
                t.add(*tag);
            }
            terms.push(t);
        }
        Prakriya {
            terms: with_slots(terms),
            ..Default::default()
        }
    }

    #[test]
    fn nic_is_added_after_a_curadi_root_only() {
        let mut p = with_nic("cur", None);
        assert!((rule("3.1.25").apply)(&mut p));
        assert_eq!(p.terms[NIC].text, "Ric");
        assert!(p.terms[NIC].has(Tag::Pratyaya));
        // √bhū: no Curadi tag, no ṇic.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("BU")]),
            ..Default::default()
        };
        assert!(!(rule("3.1.25").apply)(&mut p));
        assert_eq!(p.terms.len(), NIC);
        assert!(p.log.is_empty());
    }

    #[test]
    fn nic_it_lopa_leaves_a_nit_i() {
        let mut p = with_nic("cur", None);
        (rule("3.1.25").apply)(&mut p);
        assert!((rule("1.3.9").apply)(&mut p));
        assert_eq!(p.terms[NIC].text, "i");
        assert!(p.terms[NIC].has(Tag::Rit));
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["3.1.25", "1.3.9"]);
        // Nothing at NIC, or something that is not ṇic: decline. The second
        // case is a tiṅ ending where ṇic would sit — the layout `samjna`
        // builds — which this rule must never touch.
        let mut p = with_nic("BU", None);
        assert!(!(rule("1.3.9").apply)(&mut p));
        let mut p = with_nic("BU", None);
        p.terms.push(Term::new("tip"));
        assert!(!(rule("1.3.9").apply)(&mut p));
        assert_eq!(p.terms[NIC].text, "tip");
    }

    #[test]
    fn ardhadhatuka_sesah_tags_nic_only() {
        let mut p = with_nic("cur", Some(&[Tag::Rit]));
        assert!((rule("3.4.114").apply)(&mut p));
        assert!(p.terms[NIC].has(Tag::Ardhadhatuka));
        let mut p = with_nic("cur", None);
        assert!(!(rule("3.4.114").apply)(&mut p));
        let mut p = with_nic("cur", None);
        p.terms.push(Term::new("ti"));
        assert!(!(rule("3.4.114").apply)(&mut p));
        assert!(!p.terms[NIC].has(Tag::Ardhadhatuka));
    }

    #[test]
    fn ata_upadhayah_lengthens_an_a_upadha_before_nit() {
        for (root, want) in [("laq", "lAq"), ("aw", "Aw")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("7.2.116").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
        }
        // Not an `a` upadhā (Bakz: k; cur: u), too short to have one (a), or
        // no ṇit follower.
        for root in ["Bakz", "cur", "a"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("7.2.116").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        let mut p = with_nic("laq", Some(&[Tag::Ardhadhatuka]));
        assert!(!(rule("7.2.116").apply)(&mut p));
    }

    #[test]
    fn pugantalaghupadhasya_gunates_a_laghu_ik_upadha_before_nic() {
        for (root, want) in [("cur", "cor"), ("uz", "oz"), ("kft", "kart")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("7.3.86").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
        }
        // Guru upadhā (BUz: long U; Bakz: k), too short (u), or a ṇic not
        // (yet) ārdhadhātuka.
        for root in ["BUz", "Bakz", "u"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("7.3.86").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        let mut p = with_nic("cur", Some(&[Tag::Rit]));
        assert!(!(rule("7.3.86").apply)(&mut p));
    }

    #[test]
    fn sanadyanta_folds_nic_into_the_dhatu() {
        let mut p = with_nic("cor", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!((rule("3.1.32").apply)(&mut p));
        assert_eq!(p.terms.len(), ANGA + 1, "the ṇic term is gone");
        assert_eq!(p.terms[ANGA].text, "cori");
        assert!(p.terms[ANGA].has(Tag::Dhatu));
        assert!(p.terms[ANGA].has(Tag::Nijanta));
        let mut p = with_nic("BU", None);
        assert!(!(rule("3.1.32").apply)(&mut p));
        let mut p = with_nic("BU", None);
        p.terms.push(Term::new("ti"));
        assert!(!(rule("3.1.32").apply)(&mut p));
        assert_eq!(p.terms.len(), NIC + 1);
    }
}
