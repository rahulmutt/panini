//! Aṅga operations: 6.4.71 … 7.2.81.
//!
//! Ordered AFTER 3.1.68 — ending at `ENDING`, śap at `SHAP`, and
//! `terms[SHAP].text` may be empty (2.4.72). See `super::terms`.

use crate::rule::{Rule, RuleKind};
use crate::term::Tag;
use crate::tinanta::sound::{is_hrasva, is_vowel};
use crate::tinanta::terms::{AGAMA, ANGA, ENDING, SHAP, insert_char, word_chars};
use panini_data::{Lakara, Pada};

pub(crate) static ANGA_RULES: &[Rule] = &[
    // 6.4.71 luṅlaṅlṛṅkṣvaḍudāttaḥ: the aṭ-āgama precedes the aṅga in laṅ.
    //
    // Written into the permanent `AGAMA` slot, not prefixed onto the aṅga's
    // text: the aṅga's own first character and text stay the root's, so a
    // rule reading either needs no allowance for the augment. (Until the
    // juhotyādi prep this was a text prefix, which is why several guards
    // downstream match with `ends_with` — see their comments.)
    //
    // READS `ANGA`, NOT THE FIRST NON-EMPTY TERM AFTER `AGAMA`. Grammatically
    // the augment precedes the whole aṅga, abhyāsa included, and 6.1.90
    // already merges the āṭ into the first non-empty term after the slot.
    // The consonant/vowel verdict is the same either way for every
    // juhotyādi row: the abhyāsa is a copy of the root's first ekāc, and no
    // rule in 7.4.59–7.4.78 changes its initial's class — 7.4.60 keeps the
    // first consonant, 7.4.62 substitutes consonant for consonant, 7.4.66
    // and 7.4.77 vowel for vowel. Reading the abhyāsa instead would add a
    // clause no 3a root can falsify (both are consonant-initial), so the
    // ANGA read stays; slice 3d's √ṛ (iyarti, aiyaḥ) is the vowel-initial
    // row that re-checks this argument against a live witness.
    Rule {
        id: "6.4.71",
        name: "luNlaNlfNkzvaqudAttaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let first = p.terms[ANGA].text.chars().next().unwrap();
            if !matches!(p.ctx.lakara, Lakara::Lan) || is_vowel(first) {
                return false;
            }
            let before = p.snapshot();
            p.terms[AGAMA].text = "a".into();
            p.record("6.4.71", "luNlaNlfNkzvaqudAttaH", before);
            true
        },
    },
    // 6.4.72 āḍ ajādīnām: vowel-initial aṅgas take the āṭ-āgama in laṅ
    // (apavāda to 6.4.71's aṭ). The A then merges by 6.1.90 āṭaś ca into
    // vṛddhi with the initial vowel of the term that follows it: A+eD → ED,
    // A+Ikz → Ekz, A+ad → Ad.
    //
    // Written into `AGAMA`, like aṭ. The guard is the aṅga's own shape and
    // nothing else: 6.4.71 leaves ANGA's text untouched, so `is_vowel(first)`
    // tells a genuinely vowel-initial root from one 6.4.71 just augmented.
    // This rule used to scan the log for a prior 6.4.71 because the aṭ was
    // once a text prefix (`aBU` reads as vowel-initial); an "is AGAMA empty"
    // clause in its place would be unkillable — 6.4.71 declines for every
    // vowel-initial aṅga, so the slot is always empty when this rule looks
    // — and is deliberately not written.
    // Reads ANGA's own initial for the same reason 6.4.71 does — see its comment.
    Rule {
        id: "6.4.72",
        name: "Aq ajAdInAm",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let first = p.terms[ANGA].text.chars().next().unwrap();
            if !matches!(p.ctx.lakara, Lakara::Lan) || !is_vowel(first) {
                return false;
            }
            let before = p.snapshot();
            p.terms[AGAMA].text = "A".into();
            p.record("6.4.72", "Aq ajAdInAm", before);
            true
        },
    },
    // 6.1.73 Ce ca: a short vowel before `C` takes the tuk-āgama — a `t`,
    // placed AFTER the vowel by 1.1.46 ādyantau ṭakitau, since tuk is kit.
    // aCid → atCid, which 8.4.40 stoH ScunA ScuH then carries to acCid.
    //
    // Below the 6.4.71/6.4.72 āgama pair (6.4.72 sits between this rule
    // and 6.4.71 in the array), and 6.4.71 is what manufactures the whole
    // of this rule's precondition: the only short vowel any curated root
    // presents before a `C` is the aṭ-āgama laṅ prefixes onto a C-initial
    // aṅga. Outside laṅ the `C` is word-initial and this rule has nothing
    // to sit after, which is why √chid's and √chṛd's laṭ, loṭ and
    // vidhiliṅ cells never take it.
    //
    // WHOLE-WORD, not ANGA-local, and deliberately. 6.1.73's condition is a
    // saṁhitā condition; the aṭ-plus-root site is where this corpus happens
    // to present one, not what the sūtra says. An ANGA-local scan would need
    // a NARROW GUARD comment arguing that a `C` can only ever be
    // root-initial — true today, and the shape of argument that has twice
    // cost this repo a real defect (8.2.39's three-literal guard, 8.4.41's
    // `z`-only trigger).
    //
    // The tuk lands in whichever term holds the short vowel — `AGAMA`, for
    // the laṅ aṭ that is this corpus's only site — because `word_chars`
    // addresses the whole word. ANGA's first character stays `C` and its
    // penult is untouched, so 6.4.72's `is_vowel(first)` guard and every
    // upadhā read below this point are unmoved.
    //
    // 6.1.76 padāntād vā, which makes the tuk OPTIONAL after a PADA-final
    // short vowel, is deliberately absent rather than overlooked: the aṭ is
    // word-internal here, so no site in this corpus is pada-final and the
    // augment is obligatory. Implement it when an upasarga or a preceding
    // pada enters scope — and note it would be this engine's eighth vikalpa
    // rule, so `exactly_the_pinned_vikalpa_rules_are_optional` must change
    // with it.
    Rule {
        id: "6.1.73",
        name: "Ce ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let w = word_chars(p);
            let Some(pos) = (1..w.len()).find(|i| w[*i].2 == 'C' && is_hrasva(w[i - 1].2)) else {
                return false;
            };
            let (term, idx, _) = w[pos - 1];
            let before = p.snapshot();
            insert_char(p, term, idx + 1, 't');
            p.record("6.1.73", "Ce ca", before);
            true
        },
    },
    // 7.3.100 adaH sarvezAm: √ad prefixes aṭ (`a`) to a laṅ singular
    // consonant ending (2sg s, 3sg t). Without it, Ad+s / Ad+t are word-final
    // conjuncts that 8.2.23 saṃyogāntasya lopaḥ would strip to bare Ad,
    // collapsing 2sg=3sg=1sg-stem. The inserted `a` makes the word
    // vowel-final: 8.2.23 declines, and cartva (8.4.55) skips the `d` (now
    // before `a`, not a khar) → Adat, Adas→AdaH. Guarded structurally
    // (Tag::Adadi ∧ laṅ ∧ consonant-final aṅga ∧ single-char s/t ending); in
    // the current root set that is exactly √ad, and √vas landing (5e) adds
    // no new case here (its ātmanepada endings never collapse to a bare
    // single-char ending at the point this rule runs).
    //
    // The `||`→`&&` mutant on the guard line below is killed by the
    // `akupyat_trace_shows_7_3_100_declines_for_non_adadi_roots` pin in
    // `crates/panini/tests/trace/`: the mutant fires for laṅ non-adādi
    // derivations and 6.1.97 repairs the surface form, so only the ordered
    // trace exposes it. (Slice 5e parked this mutant as unkillable on a case
    // analysis that slice 5f corrected.)
    Rule {
        id: "7.3.100",
        name: "adaH sarvezAm",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lan) || !p.terms[ANGA].has(Tag::Adadi) {
                return false;
            }
            // Consonant-final aṅga only (ā-final √yā/√vā never insert).
            let Some(anga_last) = p.terms[ANGA].text.chars().last() else {
                return false;
            };
            if is_vowel(anga_last) {
                return false;
            }
            // Single-consonant ending: 2sg `s` / 3sg `t` (not the multi-char
            // tam/tAm/ta of dual/plural).
            let e = &p.terms[ENDING].text;
            if e.chars().count() != 1 || !matches!(e.as_str(), "s" | "t") {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = format!("a{e}");
            p.record("7.3.100", "adaH sarvezAm", before);
            true
        },
    },
    // 7.1.5 ātmanepadeṣv anataḥ: in ātmanepada, the leading `J` (jh) of the
    // ending becomes `at` — not the `ant` of 7.1.3 — when the segment the
    // ending attaches to does not end in short `a`. Apavāda to 7.1.3, ordered
    // before it; 7.1.3 then declines on its own (ending no longer starts `J`).
    // The "anataḥ" test reads the last non-empty char BEFORE the ending: for a
    // thematic root that is the śap vikaraṇa `a` (rule declines → laBante); for
    // adādi √ās the śap is luk'd/empty, so it is the root-final `s` (rule fires
    // → Asate). By this point 3.4.79 has already turned `Ja` → `Je` (laṭ/loṭ),
    // so 7.1.5 strips the leading `J` and prepends `at`: Je → ate, Ja → ata,
    // JAm → atAm. First non-a-final ātmanepadī aṅga in the engine.
    Rule {
        id: "7.1.5",
        name: "AtmanepadezvanataH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !matches!(p.ctx.pada, Pada::Atmanepada) {
                return false;
            }
            if !p.terms[ENDING].text.starts_with('J') {
                return false;
            }
            // "anataḥ": the segment before the ending must NOT end in short `a`.
            // Scan the terms before ENDING (skipping the luk'd/empty śap) for
            // the last non-empty char.
            let prev = p.terms[..ENDING]
                .iter()
                .rev()
                .find_map(|t| t.text.chars().last());
            let Some(prev) = prev else {
                return false;
            };
            if prev == 'a' {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("at{rest}");
            p.record("7.1.5", "AtmanepadezvanataH", before);
            true
        },
    },
    // 7.1.6 śīṅo ruṭ: the *jha* of √śī takes the ruṭ augment. 7.1.5 has just
    // replaced the ending's leading `J` with `at` (Je → ate, Ja → ata,
    // JAm → atAm); ruṭ's `r` prefixes that, giving Se + r + ate → Serate.
    //
    // Guarded on 7.1.5 having FIRED IN THIS DERIVATION rather than on the
    // ending's surface shape: the ruṭ attaches to the `at` that 7.1.5
    // produced, so that is the condition itself and not a proxy for it.
    // Reading the log for a prior rule is the idiom 6.4.72 used, until the
    // augment moved into its own slot, to test whether 6.4.71 had fired;
    // here it remains the condition itself.
    //
    // This is why vidhiliṅ needs no special case: 3.4.105 jhasya ran (in
    // `super::tin`) has already replaced the jha with `ran` earlier in the
    // pipeline, so 7.1.5 never fires there and ruṭ cannot attach → SayIran,
    // not *SayIraran.
    Rule {
        id: "7.1.6",
        name: "SINo ruw",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].text.ends_with("SI") || !p.log.iter().any(|s| s.sutra == "7.1.5") {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = format!("r{}", p.terms[ENDING].text);
            p.record("7.1.6", "SINo ruw", before);
            true
        },
    },
    // 7.1.4 ad abhyastāt: after an abhyasta aṅga the jh of the ending is
    // replaced by `at`, not by 7.1.3's `ant` — juhvati, cikyati; juhvatu,
    // cikyatu. Apavāda to 7.1.3 and ordered before it; self-guarding in
    // the usual way, since once the J is gone 7.1.3 has nothing to match.
    // Reads Tag::Abhyasta on ANGA: the aṅga before the tiṅ affix (1.4.13)
    // is the abhyasta pair, and 6.1.10 tagged its root half.
    Rule {
        id: "7.1.4",
        name: "ad aByastAt",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Abhyasta) || !p.terms[ENDING].text.starts_with('J') {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("at{rest}");
            p.record("7.1.4", "ad aByastAt", before);
            true
        },
    },
    // 7.1.3 jho'ntaḥ: a leading `J` of the ending → `ant`.
    Rule {
        id: "7.1.3",
        name: "Jo'ntaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
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
        vikalpa: false,
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
    // 6.4.23 śnān nalopaḥ: after śnam, the root's own nasal is elided.
    // hins + śnam gives hinans (3.1.78 seats `na` after the last vowel and
    // pushes the root's `ns` behind it); 6.4.23 takes the root's `n` out,
    // leaving hinas, whence hinasti.
    //
    // NARROW GUARD, by design. The nasal this rule deletes lives in SHAP,
    // not ANGA — an artefact of the infix representation (see
    // `super::vikarana`'s 3.1.78 and `super::terms`), and the reason the
    // rule reads SHAP at all. The guard looks for a nasal immediately after
    // śnam's own `na` (`rest.starts_with('n')`) and does nothing otherwise.
    // In 7a the only reachable witness was √hiṃs, whose tail is `ns`. 7b
    // brings √bhañj and √indh: the guard is UNCHANGED, since both roots'
    // tails also begin with `n` (`nj`, `nD`) — they are new witnesses for
    // the existing guard, not a widening of it.
    //
    // √bhañj concretely: `Banj` splits (3.1.78) as `Ba | na | nj` — head
    // through the last vowel in ANGA, śnam's `na` plus the root's own tail
    // `nj` in SHAP. This rule removes the root's own `n`, leaving
    // `Ba | na | j`; 6.4.111 below then removes śnam's own `a` in the weak
    // cells, leaving `Ba | n | j` — the `n` that survives there is śnam's,
    // not the root's.
    //
    // This is also why 6.4.24 aniditāṁ hala upadhāyāḥ kṅiti is not needed in
    // this slice: it governs the PENULTIMATE nasal of roots like √añj and
    // √tañc (out of scope here), whereas the nasal 6.4.23 removes sits
    // immediately behind śnam's `na` and is already this rule's by its own
    // terms.
    //
    // Ordered before 6.4.111: the trace order is 6.4.23 then 6.4.111, and
    // reversing them elides śnam's `a` first, after which this rule can no
    // longer tell śnam's `n` from the root's.
    Rule {
        id: "6.4.23",
        name: "SnAnnalopaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Rudhadi) {
                return false;
            }
            let Some(shap) = p.terms.get(SHAP) else {
                return false;
            };
            // śnam's own `na`, then the root's tail. Only a tail whose first
            // sound is `n` is in scope — √hiṃs's `ns`, √bhañj's `nj` and
            // √indh's `nD`; see the NARROW GUARD note above.
            let rest: String = shap.text.chars().skip(2).collect();
            if !rest.starts_with('n') {
                return false;
            }
            let before = p.snapshot();
            let head: String = p.terms[SHAP].text.chars().take(2).collect();
            p.terms[SHAP].text = format!("{head}{}", &rest[1..]);
            p.record("6.4.23", "SnAnnalopaH", before);
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
    use crate::tinanta::derivation_tests::sole;
    use crate::tinanta::derive;
    use crate::tinanta::form_g;
    use crate::tinanta::rules;
    use crate::tinanta::terms::{AGAMA, with_slots};
    use panini_data::{Purusha, Vacana, dhatus};

    #[test]
    fn at_augment_lands_in_the_agama_slot_not_the_anga_text() {
        // 6.4.71 luNlaNlfNkzvaqudAttaH. The augment is its own term now:
        // ANGA keeps the bare root, so no later guard has to tolerate a
        // leading `a` that is not the root's.
        let rule = rules().find(|r| r.id == "6.4.71").unwrap();
        let mut p = Prakriya {
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            terms: with_slots(vec![Term::new("BU"), Term::new("a"), Term::new("t")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[AGAMA].text, "a");
        assert_eq!(p.terms[ANGA].text, "BU");
        assert_eq!(p.text(), "aBUat");
        assert_eq!(p.log.last().unwrap().sutra, "6.4.71");
    }

    #[test]
    fn at_augment_declines_outside_lan_and_for_vowel_initial_angas() {
        let rule = rules().find(|r| r.id == "6.4.71").unwrap();
        // laṭ: no augment at all.
        let mut p = Prakriya {
            ctx: Context::new(
                Lakara::Lat,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            terms: with_slots(vec![Term::new("BU"), Term::new("a"), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[AGAMA].text, "");
        // laṅ, vowel-initial aṅga: 6.4.72's business, not this rule's.
        let mut p = Prakriya {
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            terms: with_slots(vec![Term::new("ad"), Term::new(""), Term::new("t")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[AGAMA].text, "");
    }

    #[test]
    fn aat_augment_lands_in_the_agama_slot_for_vowel_initial_angas() {
        // 6.4.72 Aq ajAdInAm: āṭ, like aṭ, is its own term.
        let rule = rules().find(|r| r.id == "6.4.72").unwrap();
        let mut p = Prakriya {
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            terms: with_slots(vec![Term::new("ad"), Term::new(""), Term::new("t")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[AGAMA].text, "A");
        assert_eq!(p.terms[ANGA].text, "ad");
        assert_eq!(p.text(), "Aadt");
        assert_eq!(p.log.last().unwrap().sutra, "6.4.72");
    }

    #[test]
    fn aat_declines_by_the_angas_own_shape_with_no_log_lookup() {
        // A consonant-initial aṅga that 6.4.71 has just augmented must not
        // take āṭ too. With the augment in its own slot, ANGA's first
        // character is still the consonant, so the shape guard alone
        // decides — there is no log scan left to mutate.
        let r71 = rules().find(|r| r.id == "6.4.71").unwrap();
        let r72 = rules().find(|r| r.id == "6.4.72").unwrap();
        let mut p = Prakriya {
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            terms: with_slots(vec![Term::new("BU"), Term::new("a"), Term::new("t")]),
            ..Default::default()
        };
        assert!((r71.apply)(&mut p));
        assert!(!(r72.apply)(&mut p));
        assert_eq!(p.terms[AGAMA].text, "a");
        // laṭ, vowel-initial: no augment in any slot.
        let mut p = Prakriya {
            ctx: Context::new(
                Lakara::Lat,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            terms: with_slots(vec![Term::new("ad"), Term::new(""), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(r72.apply)(&mut p));
        assert_eq!(p.terms[AGAMA].text, "");
    }

    #[test]
    fn ad_abhyastat_preempts_jho_ntah_after_an_abhyasta_anga() {
        // Ji → ati (laṭ), Ju → atu (loṭ) when ANGA is abhyasta; 7.1.3 then
        // finds no J. Without the tag, 7.1.3's ant is the answer (Bavanti).
        let r_4 = rules().find(|r| r.id == "7.1.4").unwrap();
        let r_3 = rules().find(|r| r.id == "7.1.3").unwrap();
        for (ending, expected) in [("Ji", "ati"), ("Ju", "atu")] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new("hu"), Term::new(""), Term::new(ending)]),
                ..Default::default()
            };
            p.terms[ANGA].add(Tag::Abhyasta);
            assert!((r_4.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ENDING].text, expected);
            assert_eq!(p.log.last().unwrap().sutra, "7.1.4");
            assert!(!(r_3.apply)(&mut p), "7.1.3 must find no J after 7.1.4");
            assert_eq!(p.terms[ENDING].text, expected);
        }
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("BU"), Term::new("a"), Term::new("Ji")]),
            ..Default::default()
        };
        assert!(!(r_4.apply)(&mut p));
        assert!((r_3.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "anti");
        // An abhyasta aṅga before a J-less ending: nothing to do (juhoti).
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("hu"), Term::new(""), Term::new("ti")]),
            ..Default::default()
        };
        p.terms[ANGA].add(Tag::Abhyasta);
        assert!(!(r_4.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "ti");
    }

    #[test]
    fn salopa_elides_only_the_non_final_s() {
        // Madhyama-eka is the trap: yAs + s = yAss, and only the FIRST s is
        // non-final. Eliding both would derive *Bave for BaveH.
        for (ending, want) in [("yAst", "yAt"), ("yAss", "yAs"), ("yAsus", "yAus")] {
            let mut p = Prakriya {
                terms: with_slots(vec![Term::new("Bav"), Term::new("a"), Term::new(ending)]),
                log: vec![],
                ctx: Context::new(
                    Lakara::VidhiLin,
                    Pada::Parasmaipada,
                    Purusha::Prathama,
                    Vacana::Eka,
                ),
                blocked: false,
            };
            let rule = rules().find(|r| r.id == "7.2.79").unwrap();
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ENDING].text, want, "{ending}");
        }
    }

    #[test]
    fn ato_yeyah_rewrites_the_ya_prefix_after_shap_a() {
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Bav"), Term::new("a"), Term::new("yAt")]),
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "7.2.80").unwrap();
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
            terms: with_slots(vec![Term::new("Bav"), Term::new("a"), Term::new("yAt")]),
            log: vec![],
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "7.2.80").unwrap();
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
            terms: with_slots(vec![Term::new("i"), Term::new("i"), Term::new("yAt")]),
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "7.2.80").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "yAt");
    }

    #[test]
    fn ato_nitah_requires_the_ngit_tag() {
        // Parasmaipada lot uttama Ani starts with A but is NOT Nid-vat
        // (1.2.4 pic-ca exclusion) — it belongs to 6.1.101, not 7.2.81.
        let mut anga = Term::new("Bav");
        anga.add(Tag::Anga);
        let mut p = Prakriya {
            terms: with_slots(vec![anga, Term::new("a"), Term::new("Ani")]),
            log: vec![],
            ctx: Context::new(
                Lakara::Lot,
                Pada::Parasmaipada,
                Purusha::Uttama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "7.2.81").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "Ani");
    }

    #[test]
    fn rut_requires_both_shing_and_a_fired_seven_one_five() {
        // Both clauses of 7.1.6's guard must hold. Dropping either one is a
        // live mutant, and each half is pinned here.
        //
        // (a) 7.1.5 fired, but the aṅga is √ās, not √śī: no ruṭ (Asate, not
        //     *Asrate). This is the clause an `||` → `&&` mutant drops.
        assert_eq!(
            form_g("02.0011", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "Asate"
        );
        // (b) The aṅga IS √śī, but 7.1.5 never fired (empty log): the rule
        //     must decline and leave the ending untouched.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("SI"), Term::new(""), Term::new("ate")]),
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "7.1.6").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "ate");
    }

    #[test]
    fn nalopa_removes_only_the_roots_own_nasal() {
        // 6.4.23 deletes the nasal the ROOT contributed, which after 3.1.78
        // sits in SHAP behind śnam's `na`. It must not touch śnam's own `n`:
        // hinans → hinas, never *hias or *hins.
        let d = dhatus().iter().find(|d| d.dhatupatha == "07.0019").unwrap();
        let p = sole(derive(
            d,
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        ));
        let step = p.log.iter().find(|s| s.sutra == "6.4.23").unwrap();
        assert_eq!(step.before, "hinansti");
        assert_eq!(step.after, "hinasti");
    }

    #[test]
    fn nalopa_declines_where_the_tail_has_no_nasal() {
        // kft's tail is `t` and Kid's is `d`. A guard that fired on any
        // rudhādi root would produce *kfRatti from a mangled stem.
        for number in ["07.0010", "07.0012"] {
            let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
            let branches = derive(
                d,
                Lakara::Lat,
                d.pada.padas()[0],
                Purusha::Prathama,
                Vacana::Eka,
            );
            for p in &branches {
                assert!(
                    !p.log.iter().any(|s| s.sutra == "6.4.23"),
                    "{}: 6.4.23 fired with no root nasal",
                    d.code
                );
            }
        }
    }

    #[test]
    fn che_ca_inserts_tuk_only_after_a_short_vowel() {
        let rule = rules().find(|r| r.id == "6.1.73").unwrap();

        // The one site this corpus reaches: 6.4.71's aṭ before a C-initial
        // aṅga. The augment is its own term, so `word_chars` finds the short
        // vowel at (AGAMA, 0) and the tuk lands after it — in AGAMA, which
        // then reads `at`. The word is atCinadt exactly as before.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Ci"), Term::new("nad"), Term::new("t")]),
            ..Default::default()
        };
        p.terms[AGAMA].text = "a".into();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[AGAMA].text, "at");
        assert_eq!(p.terms[ANGA].text, "Ci");
        assert_eq!(p.text(), "atCinadt");

        // Word-initial `C`: nothing precedes it, so there is no short vowel
        // to attach to. This is every laṭ, loṭ and vidhiliṅ cell of √chid
        // and √chṛd, and it is why the two new sūtras are laṅ-only.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("Ci"), Term::new("nad"), Term::new("ti")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        // A long vowel before the `C`: *hrasva* is the sūtra's own
        // condition and a dīrgha does not satisfy it.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("ACi"), Term::new("nad"), Term::new("t")]),
            ..Default::default()
        };
        assert!(!(rule.apply)(&mut p));

        // Across a term boundary — the whole-word scan's reason for being.
        // No curated root presents this shape today; the scan states
        // 6.1.73's saṁhitā condition rather than the one site that happens
        // to reach it.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("a"), Term::new("Cid")]),
            ..Default::default()
        };
        assert!((rule.apply)(&mut p));
        assert_eq!(p.text(), "atCid");
    }
}
