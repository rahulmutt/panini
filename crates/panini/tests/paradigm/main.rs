#[path = "../common/mod.rs"]
mod common;
mod data;

use common::{CELLS, LAKARA_BY_NAME};
use data::{ALTERNATES, PARADIGM};
use panini::{Panini, Verdict};
use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};
use panini_prakriya::derive;

fn lan_a_form(number: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
    let branches = derive(d, Lakara::Lan, Pada::Atmanepada, pu, va);
    assert_eq!(
        branches.len(),
        1,
        "{number} laṅ ātmanepada {pu:?} {va:?} forked unexpectedly"
    );
    branches[0].text()
}

#[test]
fn labh_lan_atmanepada_all_nine_cells() {
    let expected = [
        (Purusha::Prathama, Vacana::Eka, "alaBata"),
        (Purusha::Prathama, Vacana::Dvi, "alaBetAm"),
        (Purusha::Prathama, Vacana::Bahu, "alaBanta"),
        (Purusha::Madhyama, Vacana::Eka, "alaBaTAH"),
        (Purusha::Madhyama, Vacana::Dvi, "alaBeTAm"),
        (Purusha::Madhyama, Vacana::Bahu, "alaBaDvam"),
        (Purusha::Uttama, Vacana::Eka, "alaBe"),
        (Purusha::Uttama, Vacana::Dvi, "alaBAvahi"),
        (Purusha::Uttama, Vacana::Bahu, "alaBAmahi"),
    ];
    for (pu, va, form) in expected {
        assert_eq!(lan_a_form("01.1130", pu, va), form, "{pu:?} {va:?}");
    }
}

#[test]
fn vowel_initial_roots_take_at_not_a() {
    // 6.4.72 āḍ ajādīnām (apavāda to 6.4.71) + 6.1.90 vṛddhi:
    // a+eD → ED (aidhata), a+Ikz → Ekz (aikṣata).
    assert_eq!(
        lan_a_form("01.0002", Purusha::Prathama, Vacana::Eka),
        "EData"
    );
    assert_eq!(
        lan_a_form("01.0694", Purusha::Prathama, Vacana::Eka),
        "Ekzata"
    );
}

#[test]
fn every_form_validates_and_matches() {
    let engine = Panini::new();
    for (root, lakara, row_pada, forms) in PARADIGM.iter() {
        // `PARADIGM`'s first column is a `Dhatu::dhatupatha`, but
        // `Analysis::dhatu` reports the surface `code` (deliberately not
        // unique — it's a user-facing spelling, not a key). The two must be
        // resolved against each other rather than compared directly. Because
        // both √aś rows share `code == "aS"`, matching on `code` alone would
        // let a mis-transcribed row silently bind to the WRONG root's forms
        // as long as the two roots' surfaces happen to be disjoint.
        // Comparing against `row_pada` — the row's own declared pada — rather than
        // `d.pada.padas()[0]` pins the row's claim, not the root's: it still
        // closes the √aś hole (kryādi's is parasmaipada, svādi's is
        // ātmanepada), and it is the form that also works once a root's
        // `PadaAssignment` is `Ubhayapada` and `padas()[0]` alone can no
        // longer stand in for "the pada this block is for".
        let d = dhatus().iter().find(|d| d.dhatupatha == *root).unwrap();
        for expected in forms {
            let r = engine.check(expected);
            assert!(
                matches!(r.verdict, Verdict::Valid),
                "expected VALID for {expected} ({root} {lakara})"
            );
            assert!(
                r.analyses.iter().any(|a| a.form_slp1 == *expected
                    && a.dhatu == d.code
                    && a.pada == *row_pada
                    && panini::lakara_name(a.lakara) == *lakara),
                "no {lakara} analysis of {root} produced {expected}"
            );
        }
    }
}

/// Every alternate must itself check out as a real form of the root and
/// lakāra it is filed under — same `Dhatu::dhatupatha` → `code` resolution
/// `every_form_validates_and_matches` uses, since `Analysis::dhatu` reports
/// the non-unique surface `code`. Pinned against the row's own `pada`, for
/// the same reason `every_form_validates_and_matches` is.
#[test]
fn every_alternate_validates_and_matches() {
    let engine = Panini::new();
    for (root, lakara, row_pada, _cell, form, _key) in ALTERNATES.iter() {
        let d = dhatus().iter().find(|d| d.dhatupatha == *root).unwrap();
        let r = engine.check(form);
        assert!(
            matches!(r.verdict, Verdict::Valid),
            "expected VALID for alternate {form} ({root} {lakara})"
        );
        assert!(
            r.analyses.iter().any(|a| a.form_slp1 == *form
                && a.dhatu == d.code
                && a.pada == *row_pada
                && panini::lakara_name(a.lakara) == *lakara),
            "no {lakara} analysis of {root} produced alternate {form}"
        );
    }
}

/// `derivation_set_is_exactly_pinned`'s `(r, l, p, c, _, _)` filter and
/// `every_alternate_validates_and_matches`'s `_cell` both silently ignore a
/// row whose `cell` is out of range or whose `(root, lakara, pada)` is
/// mistyped — neither assertion would ever touch the cell such a row meant
/// to name. This closes that: every `ALTERNATES` row must name a real cell
/// of a real `PARADIGM` block, pada included.
#[test]
fn every_alternate_names_a_real_cell() {
    for (root, lakara, pada, cell, form, _key) in ALTERNATES.iter() {
        assert!(
            *cell < 9,
            "alternate {form} ({root} {lakara}) has out-of-range cell {cell}"
        );
        assert!(
            PARADIGM
                .iter()
                .any(|(r, l, p, _)| r == root && l == lakara && p == pada),
            "alternate {form} names {root} {lakara} {pada:?}, which is not a PARADIGM block"
        );
    }
}

/// The optional rules, in pipeline order. Mirrors
/// `exactly_the_pinned_vikalpa_rules_are_optional` in `panini-prakriya`;
/// duplicated here rather than exported because this is an integration test
/// and the rule table is crate-internal.
const VIKALPA_RULES: &[&str] = &[
    "7.1.35", "3.4.111", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56",
];

/// `ALTERNATES` is otherwise 494 bare strings, and a string can be right for
/// the wrong reason — `BavatAt` is a real form whether or not 8.4.56 is what
/// produced it. This ties each row to the grammar: find the branch that
/// derives the row's form, intersect its log with the optional-rule set, and
/// require exactly the rules the row claims.
#[test]
fn every_alternate_names_the_vikalpa_rules_that_produced_it() {
    for (root, lakara, pada, cell, form, key) in ALTERNATES.iter() {
        let d = dhatus().iter().find(|d| d.dhatupatha == *root).unwrap();
        let (pu, va) = CELLS[*cell];
        let lak = *LAKARA_BY_NAME
            .iter()
            .find_map(|(n, l)| (n == lakara).then_some(l))
            .unwrap();
        let branch = derive(d, lak, *pada, pu, va)
            .into_iter()
            .find(|p| !p.blocked && p.text() == *form)
            .unwrap_or_else(|| panic!("no branch of {root} {lakara} cell {cell} derives {form}"));
        let applied: Vec<&str> = branch
            .log
            .iter()
            .map(|s| s.sutra.as_str())
            .filter(|s| VIKALPA_RULES.contains(s))
            .collect();
        assert_eq!(
            applied.join("+"),
            *key,
            "{form} ({root} {lakara} cell {cell})"
        );
    }
}

/// The other half of `every_form_validates_and_matches`, which only ever
/// asks "is this form derivable?" and never "what else is?". That asymmetry
/// is what lets alternates land without touching PARADIGM's strings, and it
/// is also a hole: an over-firing optional rule would fork cells nobody
/// checks. This closes it — for every cell, the set of forms the engine
/// derives must be EXACTLY its pinned form plus its pinned alternates.
#[test]
fn derivation_set_is_exactly_pinned() {
    for (root, lakara, row_pada, forms) in PARADIGM.iter() {
        let d = dhatus().iter().find(|d| d.dhatupatha == *root).unwrap();
        for (cell, expected) in forms.iter().enumerate() {
            let (pu, va) = CELLS[cell];
            let lak = *LAKARA_BY_NAME
                .iter()
                .find_map(|(n, l)| (n == lakara).then_some(l))
                .unwrap();

            let branches = derive(d, lak, *row_pada, pu, va);
            assert_eq!(
                branches[0].text(),
                *expected,
                "index 0 must be the declined derivation for {root} {lakara} cell {cell}"
            );

            let mut actual: Vec<String> = branches
                .iter()
                .filter(|p| !p.blocked)
                .map(|p| p.text())
                .collect();
            actual.sort();

            let mut want: Vec<String> = vec![(*expected).to_string()];
            want.extend(
                ALTERNATES
                    .iter()
                    .filter(|(r, l, p, c, _, _)| {
                        r == root && l == lakara && p == row_pada && *c == cell
                    })
                    .map(|(_, _, _, _, f, _)| (*f).to_string()),
            );
            want.sort();

            assert_eq!(
                actual, want,
                "derivation set for {root} {lakara} cell {cell} \
                 (pinned {expected}) is not exactly what PARADIGM + ALTERNATES say"
            );
        }
    }
}

/// Pins the shape of the derivation set the slice produces, derived from
/// `PARADIGM ∪ ALTERNATES` — the same union `derivation_set_is_exactly_pinned`
/// builds — rather than from a hand-written list. These are the numbers the
/// design-time vidyut-prakriya audit predicted for the two conventions the
/// svādi slice retired (7.1.35 tātaṅ, 8.4.56 pausal cartva), the one audited
/// divergence it resolved (3.4.111 Śākaṭāyana's jus), the three roots added
/// in rudhādi 7a (kft, his, Kid), three more added in rudhādi 7b (Banj, piz,
/// inD), and — new in the ubhayapada 1.3.72 slice — √rudh (ruD), pinned in
/// both padas, joined by the pada audit's √nī and √tud, also pinned in both
/// padas, and — new in the 8.2.30/8.2.39 generalization slice — √ric and
/// √vic, also pinned in both padas, joined by rudhādi 7d's eight roots —
/// √śiṣ (Siz), √und (und), √añj (anj), √tañc (tanc), √vij (vij), √vṛj (vfj)
/// and √pṛc (pfc), all parasmaipadī, plus √vid (vid), ātmanepadī, curated
/// with no new sūtra and cleared by their own cross-implementation audit:
/// every one of the twenty-five rudhādi roots forks in both loṭ and laṅ,
/// and two of them — kft and ruD — fork
/// in all four lakāras: laṭ (kft
/// cells 1/4/5, Kid cells 0/5, inD cells 0/5, and — new in the ubhayapada
/// 1.3.72 slice — ruD
/// parasmaipada cells 1/4/5 and ātmanepada cells 0/5, all on 8.4.65), laṅ (on
/// 8.4.65, the 8.2.74/8.2.75 ru alternation, and the 8.2.23-above-8.2.41
/// śa-luk jaśtva 8.4.56 branch), loṭ (on 7.1.35/8.4.65/8.4.56, stacking up to
/// three deep, and piṣ's loṭ madhyama eka, which stacks 8.4.65 alongside
/// 7.1.35/8.4.56 four deep), and vidhiliṅ
/// (kft/his/Banj/piz/ruD/Bid/kzud/yuj/tfd/ric/vic/Siz/und/anj/tanc/vij/vfj/pfc/tfh/Cid/Cfd/Buj
/// cell 0, on 8.4.56 — Kid, inD and vid do not fork here). Slice 7c curated four more roots —
/// √bhid (Bid), √kṣud (kzud), √yuj (yuj) and √tṛd (tfd), all four ubhayapadī
/// by 1.3.72 and pinned in both padas — and three of them join kft and ruD as
/// four-lakāra forkers: Bid, kzud and tfd each stack 7.1.35/8.4.65/8.4.56 in
/// loṭ parasmaipada exactly as kft and ruD do, while yuj forks only two deep
/// there (7.1.35/8.4.56, no 8.4.65 branch — 8.2.30 coH kuH replaces its
/// stem-final palatal `j` with the VELAR `g`, which 8.4.55 khari ca later
/// devoices to `k` before the `t` of tātaṅ, so the junction 8.4.65 would need
/// is velar-against-dental at both sites — `g`+`D` in yuNgDi, `k`+`t` in
/// yuNktAd — and never savarṇa the way the dental-final roots' `d`+`D` and
/// geminate `t`+`t` are, so 8.4.65's site never arises). The 8.2.30/8.2.39
/// generalization slice curated two more roots on exactly this shape for
/// exactly this reason — √ric (ric) and √vic (vic), each ending in a
/// palatal (`c`) rather than `j`, which 8.2.30 coH kuH — now one
/// substitution-table lookup instead of a literal `g` — substitutes with
/// the VELAR `k` rather than `g`: the same velar-against-dental mismatch
/// (`g`+`D` in riNgDi/viNgDi — 8.4.53 jaśtva has already voiced 8.2.30's
/// `k` to `g` before the jhaś `D`, so this junction is velar-against-dental
/// too, never savarṇa — and `k`+`t` in riNktAd/viNktAd) keeps 8.4.65 out
/// of their loṭ parasmaipada prathama/madhyama eka too, so they join yuj
/// forking only two deep there, on 7.1.35/8.4.56. The other rule this slice
/// widened, 8.2.39 jhalāṁ jaśo'nte, now reads its own substitution table on
/// both sides instead of a `t`/`z`/`D`-only literal guard, which reaches a
/// pada-final velar for the first time: ric's and vic's laṅ prathama and
/// madhyama eka decline to `ariRag`/`avinag` (jaśtva-voiced) with 8.4.56
/// vā'vasāne supplying the optional `ariRak`/`avinak` — the same
/// √bhañj-pattern fork yuj's `ayunag`/`ayunak` already witnesses, now with
/// a second pair of roots on it. Rudhādi 7d curated eight more roots on the
/// audited numbers alone, with no new sūtra: Siz's loṭ parasmaipada
/// madhyama eka joins piṣ's as a second four-form cell of the same shape
/// (8.4.65 alone, 7.1.35 alone, and 7.1.35+8.4.56 stacked), and und is the
/// sharpest of the eight — its loṭ parasmaipada prathama eka stacks
/// 7.1.35/8.4.65/8.4.56 exactly as kft/ruD/Bid/kzud/tfd's do (a five-form
/// cell), and its loṭ parasmaipada madhyama eka ties the six-form record
/// with the same k = 3 against the 2³ bound of eight:
/// 2844 cells total (316 root×lakāra blocks × 9), of which 2493 hold exactly one form,
/// 250 hold two, 83 hold three, two hold four (piṣ's loṭ madhyama eka, the
/// deepest fork added in 7b, and — new in slice 7d — Siz's loṭ parasmaipada
/// madhyama eka), and — the sharpest branch-count witnesses in
/// the repo, per `docs/ARCHITECTURE.md` — exactly eight hold five (√kṛt's loṭ
/// prathama eka, ruD's loṭ parasmaipada prathama eka, Bid's, kzud's and
/// tfd's loṭ parasmaipada prathama eka, und's (slice 7d), and — new in
/// slice 7f — Cid's and Cfd's loṭ parasmaipada prathama eka) and eight
/// hold six (√kṛt's loṭ madhyama eka, `kfndDi`/`kfnDi`'s cell, ruD's loṭ
/// parasmaipada madhyama eka, `rundDi`/`runDi`/`rundDAd`/`runDAd`/
/// `rundDAt`/`runDAt`, Bid's, kzud's and tfd's loṭ
/// parasmaipada madhyama eka, und's (slice 7d), and — new in slice 7f —
/// Cid's and Cfd's loṭ parasmaipada madhyama eka, each tying √kṛt's record
/// with the same k = 3
/// (7.1.35, 8.4.65, 8.4.56) against a 2³ bound of eight — ric and vic do not
/// join this record; per the 8.2.30/8.2.39 slice's own audit their deepest
/// cells are three forms). `ALTERNATES`
/// itself has 494 rows, keyed 114 `8.4.56`, 92 `7.1.35`, 92 `7.1.35+8.4.56`,
/// 2 `3.4.111`, 8 `6.4.107`, 145 `8.4.65`, 8 `8.2.75`, 1 `8.2.74`, 16
/// `7.1.35+8.4.65`, and 16 `7.1.35+8.4.65+8.4.56` — the assertions below are
/// complete. The audit probe that produced the original numbers ran against
/// a vidyut-prakriya checkout during design; slice 9's cross-implementation
/// audit re-ran the full check against a scratchpad vidyut-prakriya checkout
/// across all 1620 pre-7b cells with zero differences, every 7b form was
/// cross-checked the same way during that slice's design, the ubhayapada
/// slice's √rudh forms were audited against a vidyut-prakriya checkout at commit
/// 8da2f90 the same way, and slice 7c's four roots were audited the same way
/// against vidyut `8da2f90`, zero differences across all 2160 cells / 2496
/// forms / 53 roots, with the `entry` negative control verified failing —
/// the probe's source is committed at `tools/audit/panini_full_audit.rs`,
/// and the pada audit re-ran it over all 1872 pre-7c cells, and the
/// 8.2.30/8.2.39 generalization slice's own cross-implementation audit
/// re-ran the same probe against vidyut-prakriya at commit `8da2f90` over
/// all 2304 cells / 2654 forms / 55 roots with zero differences, its
/// `entry` negative control verified failing (36 √bhū cells) both times
/// the audit was run, and rudhādi 7d's cross-implementation audit re-ran
/// the same probe against vidyut-prakriya at commit `8da2f90` over all
/// 2592 cells / 3014 forms / 63 roots with zero differences, its `entry`
/// negative control verified failing — so the numbers are re-verified as well as pinned,
/// and rudhādi 7e's cross-implementation audit re-ran the same probe
/// against vidyut-prakriya at commit `8da2f90` over all 2628 cells /
/// 3057 forms / 64 roots with zero differences, its `entry` negative
/// control verified failing, and rudhādi 7f's cross-implementation audit
/// re-ran the same probe against vidyut-prakriya at commit `8da2f90` over
/// all 2772 cells / 3259 forms / 66 roots with zero differences, its
/// `entry` negative control verified failing, and the √bhuj/1.3.66 slice's
/// cross-implementation audit re-ran the same probe against
/// vidyut-prakriya at commit `8da2f90` over all 2844 cells / 3338 forms /
/// 67 roots with zero differences, its `entry` negative control verified
/// failing. √tṛh joins none of the fork
/// records: its deepest cells hold three forms, because 8.3.13 Qo Qe lopaH
/// obligatorily elides the ḍh that 8.4.65 forks on for every other
/// stop-final rudhādi root.
///
/// √chid and √chṛd, by contrast, join both fork records: they are
/// dental-final like √bhid and √tṛd, nothing elides the junction 8.4.65
/// wants, and their loṭ parasmaipada eka cells stack 7.1.35, 8.4.65 and
/// 8.4.56 into five branches at prathama eka and six at madhyama eka. The
/// six-form record now stands at eight cells, not six. Their laṅ cells
/// that 8.4.65 might have forked hold two forms, not three, despite
/// acCinad's `c` and `C` being savarṇa jhars: 8.4.65 carries 8.4.64's
/// *halaḥ* by anuvṛtti and the sound before that `c` is the aṭ's own
/// vowel.
///
/// √bhuj joins neither fork record, exactly as √yuj does not: its `j`
/// junctions are velar after 8.2.30's kutva, never savarṇa with a dental,
/// so 8.4.65 has nothing to elide anywhere in its paradigm — it forks
/// only on 8.4.56 (laṅ and vidhiliṅ finals) and 7.1.35 (loṭ tātaṅ),
/// seven ALTERNATES rows with √yuj's exact key profile.
/// This test is what keeps the numbers true day to day.
#[test]
fn derivation_set_shape_matches_the_audited_numbers() {
    let total_cells = PARADIGM.len() * 9;
    assert_eq!(total_cells, 2844, "316 root×lakāra blocks × 9 cells each");

    let mut ones = 0usize;
    let mut twos = 0usize;
    let mut threes = 0usize;
    let mut fours = 0usize;
    let mut fives = 0usize;
    let mut sixes = 0usize;
    for (root, lakara, row_pada, _forms) in PARADIGM.iter() {
        for cell in 0..9usize {
            let alt_count = ALTERNATES
                .iter()
                .filter(|(r, l, p, c, _, _)| {
                    r == root && l == lakara && p == row_pada && *c == cell
                })
                .count();
            match 1 + alt_count {
                1 => ones += 1,
                2 => twos += 1,
                3 => threes += 1,
                4 => fours += 1,
                5 => fives += 1,
                6 => sixes += 1,
                n => panic!("unexpected {n}-form cell in ({root}, {lakara}, {cell})"),
            }
        }
    }
    assert_eq!(ones, 2493, "one-form cells");
    assert_eq!(twos, 250, "two-form cells");
    assert_eq!(threes, 83, "three-form cells");
    assert_eq!(
        fours, 2,
        "four-form cells — piṣ's loṭ madhyama eka, and — new in slice 7d — Siz's loṭ \
         parasmaipada madhyama eka"
    );
    assert_eq!(
        fives, 8,
        "five-form cells — kft loṭ prathama eka, ruD loṭ parasmaipada prathama eka, Bid, kzud \
         and tfd's loṭ parasmaipada prathama eka, und's (slice 7d), and — new in slice 7f — \
         Cid's and Cfd's loṭ parasmaipada prathama eka"
    );
    assert_eq!(
        sixes, 8,
        "six-form cells — kft loṭ madhyama eka, ruD loṭ parasmaipada madhyama eka, Bid, kzud \
         and tfd's loṭ parasmaipada madhyama eka, und's (slice 7d), and — new in slice 7f — \
         Cid's and Cfd's loṭ parasmaipada madhyama eka"
    );

    assert_eq!(ALTERNATES.len(), 494, "ALTERNATES row count");
    let key_count = |key: &str| {
        ALTERNATES
            .iter()
            .filter(|(_, _, _, _, _, k)| *k == key)
            .count()
    };
    assert_eq!(key_count("8.4.56"), 114, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 92, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 92, "7.1.35+8.4.56 alternates");
    assert_eq!(key_count("3.4.111"), 2, "3.4.111 alternates");
    assert_eq!(key_count("6.4.107"), 8, "6.4.107 alternates");
    assert_eq!(key_count("8.4.65"), 145, "8.4.65-only alternates");
    assert_eq!(key_count("8.2.75"), 8, "8.2.75-only alternates");
    assert_eq!(key_count("8.2.74"), 1, "8.2.74-only alternates");
    assert_eq!(key_count("7.1.35+8.4.65"), 16, "7.1.35+8.4.65 alternates");
    assert_eq!(
        key_count("7.1.35+8.4.65+8.4.56"),
        16,
        "7.1.35+8.4.65+8.4.56 alternates"
    );
}

/// `every_form_validates_and_matches` only walks `PARADIGM`, so a root or
/// lakāra added to the enumerable space without golden rows would be checked
/// by nothing at all. This test closes that hole from the other side: every
/// (root × lakāra) pair the analyzer enumerates must either be pinned by a
/// `PARADIGM` block or appear in the explicit gated list below.
#[test]
fn paradigm_covers_every_enumerable_cell() {
    // adādi × vidhiliṅ was gated in slice 5a and ungated in slice 5b; √śī was
    // gated in slice 5f task 1 and ungated there; √nī and √tud's ātmanepada
    // blocks were gated for one commit by the pada audit, between the column
    // being corrected and the audited goldens landing; √chid's and √chṛd's
    // sixteen (root, lakāra, pada) triples -- 2 roots × 2 padas × 4 lakāras
    // -- were likewise gated for one commit in slice 7f, between their Dhatu
    // rows landing and their cross-implementation-audited goldens arriving.
    // √bhuj's eight (root, lakāra, pada) triples -- 1 root × 2 padas × 4
    // lakāras -- were likewise gated for one commit in the Buj/1.3.66
    // slice, between its Dhatu row landing and its audited goldens
    // arriving.
    const GATED: &[(&str, &str, Pada)] = &[];

    let pinned: Vec<(&str, &str, Pada)> =
        PARADIGM.iter().map(|(r, l, p, _)| (*r, *l, *p)).collect();
    let mut unpinned: Vec<(&str, &str, Pada)> = Vec::new();
    for d in dhatus() {
        for &lakara in panini_analyze::LAKARAS {
            for &pada in d.pada.padas() {
                let triple = (d.dhatupatha, panini::lakara_name(lakara), pada);
                if !pinned.contains(&triple) {
                    unpinned.push(triple);
                }
            }
        }
    }
    // `Pada` has no `Ord` of its own (`Context.pada` never needs to be
    // sorted); `pada_name` gives a stable, already-public key to sort by.
    fn sort_key<'a>(t: &(&'a str, &'a str, Pada)) -> (&'a str, &'a str, &'static str) {
        (t.0, t.1, panini::pada_name(t.2))
    }
    unpinned.sort_unstable_by_key(sort_key);
    let mut gated = GATED.to_vec();
    gated.sort_unstable_by_key(sort_key);
    assert_eq!(
        unpinned, gated,
        "every enumerable (root, lakara, pada) triple needs golden rows in PARADIGM \
         (or an explicit entry in GATED, for a cell deliberately withheld from golden coverage)"
    );
    // Catches a duplicated PARADIGM block masking a missing one above.
    let enumerable: usize = dhatus()
        .iter()
        .map(|d| d.pada.padas().len() * panini_analyze::LAKARAS.len())
        .sum();
    assert_eq!(
        PARADIGM.len() + GATED.len(),
        enumerable,
        "PARADIGM has a duplicate or stale (root, lakara, pada) block"
    );
}

#[test]
fn known_nonforms_are_invalid() {
    let engine = Panini::new();
    for bad in [
        // Real cross-lakāra confusions, not junk: laṅ endings require the
        // aṭ-āgama (6.4.71), and laṭ endings forbid it.
        "Bavat",    // laṅ 3sg ending without the augment
        "aBavanti", // augment on a laṭ form
        "aBavatu",  // augment on a loṭ form
        "aBavet",   // laṅ's aṭ-āgama on a vidhiliṅ form
        "Bavetu",   // loṭ's er uḥ ending on a vidhiliṅ stem
        // Still out of scope entirely.
        "gacCati",
        "Bavati123",
        "tiRRati",
        // Wrong pada: the root's pada assignment gates the whole derivation
        // (1.3.12 / 1.3.72 / 1.3.78) and the analyzer proposes exactly the
        // padas that assignment admits — one each for the single-pada roots
        // below, both for an ubhayapadī root like √rudh.
        "laBati", // atmanepadin root with a parasmaipada ending
        "Bavate", // parasmaipada root with an atmanepada ending
        "eDati",  // vowel-initial atmanepadin root, parasmaipada ending
        "alaBat", // laN parasmaipada shape on an atmanepadin root
        "laB",    // a bare root code is not a surface form
        // Cross-lakāra atmanepada confusions.
        "alaBeta", // laN's augment on a vidhilin form
        "laBatam", // parasmaipada dual ending on an atmanepadin root
        "laBAte",  // 7.2.81 skipped: A must become iy after the shap
        "laBesva", // lot's sva on a lat stem (3.4.91 without 3.4.90's lakara)
        "IkzAmi",  // parasmaipada uttama ending on the vowel-initial A-root
        // Wrong vikaraṇa: divādi/tudādi roots take śyan/śa, not śap, and
        // bhvādi does not take śyan.
        "divati",  // div with śap instead of śyan
        "tudyati", // tud with śyan instead of śa
        "Bavyati", // BU (bhvādi) with a śyan it has no claim to
        "naSati",  // naś with śap
        "kupati",  // kup with śap
        // Guṇa should have been blocked (1.1.5): these are the guṇa'd forms.
        "kopyati", // kup guṇa'd — 7.3.86 must be blocked by śyan's ṅit
        "todati",  // tud guṇa'd — 7.3.86 must be blocked by śa's ṅit
        "jozate",  // juṣ guṇa'd — block under ātmanepada too
        "devyati", // div guṇa'd (before 8.2.77): guṇa must be blocked
        // Wrong pada: the root's curated pada verdict gates the whole
        // derivation.
        "manyati", // atmanepadin divādi root with a parasmaipada ending
        "vidyati", // atmanepadin divādi root, parasmaipada ending
        // adādi (gaṇa 2): śap is luk'd (2.4.72). A retained-śap surface must
        // not derive, and the parasmaipada roots reject ātmanepada endings.
        "yAyati", // yā with a spurious y-śap — no derivation yields it
        "yAte",   // parasmaipada yā with an ātmanepada ending (wrong pada)
        "vAte",   // parasmaipada vā with an ātmanepada ending (wrong pada)
        "yAati",  // luk skipped: śap's `a` left standing after ā (uncoalesced)
        "yA",     // a bare root code is not a surface form
        "vA",
        // These four are the non-words the pre-5b pipeline emitted for adādi
        // vidhiliṅ before 6.1.96 / the 6.1.101 arm reduced the yāsuṭ-ā + vowel
        // junction. They stay pinned INVALID as the regression that the
        // reduction actually RAN: the real forms are yAyuH / yAyAm (and the vā
        // pair), now pinned as goldens in PARADIGM. If any of these four ever
        // validates, the junction reduction regressed.
        "yAyAuH", // 3pl: real form yāyuḥ
        "yAyAam", // 1sg: real form yāyām
        "vAyAuH",
        "vAyAam",
        "Asati",  // √ās is ātmanepada; a parasmaipada ending must not derive
        "Asante", // 3pl must be Asate (7.1.5), never the `ante` of 7.1.3
        // 8.2.25 dhi ca elides the aṅga-final `s` before Dve/Dvam. Both the
        // un-applied shape and slice 5d's jaśtva'd shape are non-words.
        "AsDve",    // s retained: the rule did not fire
        "AdDve",    // 5d's wrong form: s voiced to `d` instead of elided
        "AdDvam",   // ditto, laṅ/loṭ
        "vasDve",   // √vas, s retained
        "vadDve",   // √vas, 5d's wrong analysis
        "avasDvam", // √vas laṅ, s retained
        "vasati",   // √vas is ātmanepada; a parasmaipada ending must not derive
        // √śī (slice 5f). Each of these is a non-form the engine must never
        // produce, chosen around the slice's three new guards — but not all
        // seven are what a mutation of that guard would actually emit; see
        // the per-entry notes below where the naive reading is wrong.
        "SIte", // A genuine witness for 7.4.21's removal, not an unreachable
        // shape: 7.3.84's 1.1.5 guard now calls `following_sarvadhatuka`,
        // which on this śap-luk'd path returns the ṅit `te` ending itself
        // (there is no non-empty śap to interpose), so 1.1.5 really does
        // block 7.3.84 here. Without 7.4.21, nothing else guṇates `SI`, and
        // the surface form would be exactly `SIte`. It stays pinned INVALID
        // because 7.4.21 has not been removed; if 7.4.21 is ever dropped or
        // its own guard broken, this is the entry that would flip to VALID
        // and catch it. The rule actually responsible for the guṇa is
        // pinned independently by the ordered-trace test
        // `shete_trace_is_the_minimal_shing_guna_path` in
        // `crates/panini/tests/trace/adadi.rs`, which asserts `7.4.21` present and
        // `7.3.84` absent.
        "Sese",  // 8.3.59 not applied: ṣatva missing (real form Seze)
        "Seate", // NOT what removing 7.1.6 emits: without the ruṭ the ending
        // stays `ate`, and 6.1.78's athematic arm then fires (śap empty, `a`
        // is a vowel), emitting `Sayate` — already pinned below, which is
        // the actual witness for 7.1.6's removal.
        "SayIraran", // NOT a real derivation: dropping 7.1.6's guard against
        // firing in vidhiliṅ makes it prepend `r` to the sīyuṭ-bearing
        // ending `sIyran` (→ `rsIyran`); 7.2.79 still elides the non-final
        // `s` regardless (→ `rIyran`), but 6.1.78's athematic arm then
        // requires the ending's first character to be a vowel, and `r`
        // isn't one, so the ay-ādeśa never fires and the output diverges
        // from this string entirely. Kept pinned as a plain non-form; the
        // real form is `SayIran`.
        "Sayati", // wrong pada: an ātmanepadin root with a parasmaipada ending
        "Sayate", // the śap surviving 2.4.72 (SI + Sap + te, guṇa'd)
        "SIyate", // a divādi/tudādi-style vikaraṇa leaking into adādi
        // kryādi (gaṇa 9, slices 9a/9b). Each of these is what the slice's
        // own rule comments say would surface if the named rule misfired;
        // pinning them keeps those rules' guards honest the same way the
        // adādi and √śī groups above pin theirs.
        "kliSnIti",  // 1.2.4 misfiring on the pit ending tip (śnā stays anit)
        "kleSAna",   // 7.3.86 not blocked by 1.1.5 for śānac (guṇa'd upadhā)
        "kliSnIhi",  // 3.1.83 (śnā-lopa before hi) ordered after 6.4.113
        "vfReta",    // 6.4.112 (nA -> n) running after 6.1.87, not before
        "vfRIyta",   // 6.1.66's old is_empty() guard, silently declining for kryādi
        "vfRIsva",   // 8.3.59 before it read the preceding term instead of ANGA
        "vrIRAhi",   // 3.4.87 not tagging hi as pit
        "kliSnAyAt", // 3.4.103 not tagging yāsuṭ's ending ṅit
        // svādi (gaṇa 5). Four sūtras, three widened guards and six roots
        // landed with nothing pinned here until now; pinning them keeps
        // those rules' guards honest the same way the adādi, √śī and kryādi
        // groups above pin theirs.
        "aSnoti", // wrong pada: svādi's √aś is ātmanepada (real form aSnute);
        // also catches an id/code collapse from the other side — kryādi's
        // √aś (id "aS") is parasmaipada and DOES take this ending, so this
        // string would wrongly validate if the two "aS" rows' padas were
        // ever merged or mismatched.
        "Apnute", // wrong pada: √āp is parasmaipada (real form Apnoti)
        "ApnuDi", // 6.4.101 reading ANGA ("p", a jhal) instead of
        // sound_before_ending (śnu's "u", not a jhal) — real form Apnuhi
        "SaknuDi", // same guard, second conjunct root — real form Saknuhi
        "ApnoAni", // 6.1.78's vikaraṇa arm (svādi's third arm) removed —
        // real form ApnavAni
        "ApnuvAni", // 7.3.84's second application ordered AFTER 6.4.77/
        // 6.4.87 instead of before them — real form ApnavAni
        "hinuhi", // 6.4.106 under-firing (declining to luk hi after a
        // non-conjunct u) — real form hinu
        "Apnu", // 6.4.106 over-firing (luking hi after a conjunct u) —
        // real form Apnuhi
        "hinuvanti", // 6.4.87/6.4.77 swapped: the non-conjunct root taking
        // 6.4.77's uvaṅ instead of 6.4.87's yaṇ — real form hinvanti
        "Apnvanti", // the conjunct root taking 6.4.87's yaṇ instead of
        // 6.4.77's uvaṅ — real form Apnuvanti
        "aSnavAE", // 6.1.90's athematic arm not widened past is_empty() to
        // admit svādi's non-empty, non-a/A-final `nav` — real form aSnavE
        "henoti", // the FIRST 7.3.84 (root-relative) not blocked by śnu's
        // ṅit vikaraṇa — svādi never guṇates the root itself; real form
        // hinoti
        "reRoti", // same guard, second non-conjunct root — real form riRoti
        "kliSne", // 7.3.84's SECOND application (vikaraṇa-relative, svādi's
        // own addition) firing on kryādi's `nI` instead of declining by
        // 1.1.5 — real form kliSnAti
        // 6.4.107 over-firing. It is optional, so an over-firing guard
        // ADDS a wrong second form rather than replacing a right one —
        // invisible to any test that only asks whether the right form
        // still derives. Each pin names the guard it would breach.
        "ApnvaH",  // fired on a conjunct root — real form ApnuvaH
        "ApnmaH",  // same, bahu — real form ApnumaH
        "aSnvahe", // fired in the ātmanepada conjunct column, where no
        // svādi root is asaṁyogapūrva — real form aSnuvahe
        "hinTaH", // fired on an ending that is not m/v-initial — real
        // form hinuTaH
        "hinyAma", // `starts_with` mistaken for `contains`: vidhiliṅ's
        // yAma has an `m` but does not begin with one — real form hinuyAma
        "BavmaH", // fired where the vikaraṇa is not śnu at all, i.e. the
        // vikarana_u_asamyogapurva guard dropped — real form BavAmaH
        // 8.2.39 jhalāṁ jaśo'nte guard pins.
        "Bavatd", // `ends_with('t')` mistaken for `contains('t')`: fires on
        // BU laṭ 3sg (which merely contains a medial `t`, not a pada-final
        // one) and blindly voices whatever the actual last character is —
        // real form Bavati
        "aBavaD", // `s.push('d')` mistaken for `s.push('D')`: the wrong jaś
        // substitute (aspirated, not the plain voiced stop the sūtra names)
        // — real form aBavad
        // 8.4.56's `is_jhal(last)` guard (Step 11 mutation 3) has since been
        // deleted outright — it was dead code, subsumed by the `cartva_of`
        // let-else right below it — so there is no longer a mutation for it
        // to pin here. 8.4.56's `vikalpa: true` -> `false` (mutation 4)
        // removes the `d`-form rather than adding a non-form, so it is
        // caught by `derivation_set_is_exactly_pinned`'s index-0 assertion,
        // not by a pin in this list.
        // 7.1.35 tātaṅ. Because the rule is optional, a broken guard ADDS a
        // wrong second form rather than replacing a right one — invisible to
        // any test that only asks whether the right form still derives.
        "ApnotAt", // 7.1.35 failing to set Ngit, so 7.3.84's second
        // (vikaraṇa-relative) application guṇates śnu — real form ApnutAt
        "kliSAnatAt", // 7.1.35 ordered AFTER 3.1.83 instead of above it, so
        // śnā had already become śāna when the ending was still `hi` — real
        // form kliSnItAt
        // 3.4.110/111 Śākaṭāyana's jus. Optional, so a broken guard adds a
        // wrong form rather than removing a right one.
        "aBavuH", // 3.4.111 losing BOTH of its second `if`'s conjuncts (the
        // ā-check and the SHAP-empty check together, not either alone —
        // dropping only the ā-check still declines on SHAP being `Bava`'s
        // live śap `a`) — real form aBavan
        "yuH", // 3.4.111 not gated to laṅ, so laṭ's yAnti forks — real
               // form yAnti
    ] {
        assert!(
            matches!(engine.check(bad).verdict, Verdict::Invalid),
            "expected INVALID for {bad}"
        );
    }
}

#[test]
fn both_ash_roots_derive() {
    let engine = Panini::new();
    for form in ["aSnute", "aSnAti"] {
        assert!(
            matches!(engine.check(form).verdict, Verdict::Valid),
            "{form}"
        );
    }
}

/// The surfaces that are genuinely pada-ambiguous — the same string pinned
/// as both a parasmaipada and an ātmanepada cell, so `check` reports two
/// analyses differing in pada. `README.md` quotes this list; before this
/// test it was hand-maintained prose with nothing behind it, and the
/// ubhayapadī root count going from three to seven in slice 7c is exactly
/// the kind of change that would have grown it silently.
///
/// `roundtrip.rs` cannot serve this purpose: it asks only whether SOME
/// analysis recovers the input, never how many there are.
#[test]
fn pada_ambiguous_surfaces_are_exactly_these() {
    let mut para: Vec<&str> = Vec::new();
    let mut atma: Vec<&str> = Vec::new();
    for (_root, _lakara, pada, forms) in PARADIGM.iter() {
        let bucket = match pada {
            Pada::Parasmaipada => &mut para,
            Pada::Atmanepada => &mut atma,
        };
        bucket.extend(forms.iter().copied());
    }

    let mut both: Vec<&str> = para.iter().copied().filter(|f| atma.contains(f)).collect();
    both.sort_unstable();
    both.dedup();

    // Measured (never hand-picked) by running this assertion against
    // `Vec::<&str>::new()` and reading the real set off the failure. The
    // pre-slice baseline (checked separately against `main`, before any
    // 7c commit) was actually ten surfaces, not the seven README.md names:
    // `rundDAm` and `arundDa` (√rudh `07.0001`, loT and laN, each ambiguous
    // against its own two padas), `anayata`/`nayatAm`/`nayetAm`/`nayeta`
    // (√nī) and `atudata`/`tudatAm`/`tudetAm`/`tudeta` (√tud) — README's
    // hand list already missed `arundDa`, `nayetAm` and `tudetAm`, and
    // spells the rudh one without its second `d`. All ten pre-slice
    // surfaces are present below, so nothing was disturbed by slice 7c.
    // Slice 7c's four new ubhayapadī roots contribute the other eight:
    // `BinttAm`/`aBintta` (√Bid `07.0002`), `akzuntta`/`kzunttAm`
    // (√kzud `07.0006`), `ayuNkta`/`yuNktAm` (√yuj `07.0007`), and
    // `atfntta`/`tfnttAm` (√tfd `07.0009`). The 8.2.30/8.2.39 generalization
    // slice's two new ubhayapadī roots contribute four more, the same
    // shape as yuj's pair: `ariNkta`/`riNktAm` (√ric `07.0004`) and
    // `aviNkta`/`viNktAm` (√vic `07.0005`). This slice's own acceptance
    // check was that all eighteen pre-slice surfaces (the pre-7c ten plus
    // 7c's own eight) survive undisturbed, with only the four ric/vic
    // surfaces added — and they do. Rudhādi 7d re-ran this same check: its
    // one new pada-bearing addition to the bucket, √vid (`07.0013`,
    // ātmanepada-only), gets compared against every parasmaipada surface
    // in the corpus, and the result is no new collision — the set is
    // unchanged at twenty-two surfaces. Slice 7e contributed nothing to
    // this bucket: its one root, √tṛh (`07.0018`), is parasmaipada-only, so
    // it has no second pada to collide against and never enters this
    // check. Slice 7f's two new ubhayapadī roots contribute four more, the
    // same shape as √bhid's and √tṛd's pairs: `CfnttAm`/`acCfntta`
    // (√chṛd `07.0008`) and `CinttAm`/`acCintta` (√chid `07.0003`), taking
    // the set to twenty-six with no new collision against any pre-slice
    // surface. The Buj/1.3.66 slice's one new both-pada root contributes two
    // more, the same shape as √yuj's pair: `BuNktAm`/`aBuNkta`, taking the
    // set to twenty-eight with no new collision against any pre-slice
    // surface.
    assert_eq!(
        both,
        vec![
            "BinttAm", "BuNktAm", "CfnttAm", "CinttAm", "aBintta", "aBuNkta", "acCfntta",
            "acCintta", "akzuntta", "anayata", "ariNkta", "arundDa", "atfntta", "atudata",
            "aviNkta", "ayuNkta", "kzunttAm", "nayatAm", "nayetAm", "nayeta", "riNktAm", "rundDAm",
            "tfnttAm", "tudatAm", "tudetAm", "tudeta", "viNktAm", "yuNktAm",
        ]
    );
}
