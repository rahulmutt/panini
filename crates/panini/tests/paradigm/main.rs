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
            let analyses = &r.analyses;
            assert!(
                !analyses.is_empty(),
                "expected VALID for {expected} ({root} {lakara})"
            );
            assert!(
                analyses.iter().any(|a| a.dhatu == d.code
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
        let analyses = &r.analyses;
        assert!(
            !analyses.is_empty(),
            "expected VALID for alternate {form} ({root} {lakara})"
        );
        assert!(
            analyses.iter().any(|a| a.dhatu == d.code
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

/// The optional rules — the same set `exactly_the_pinned_vikalpa_rules_are_optional`
/// pins in `panini-prakriya`, duplicated here rather than exported because
/// this is an integration test and the rule table is crate-internal. Order
/// here is unconstrained, since the list below is read only via
/// `.contains()`: it is NOT the pipeline order — the pin has 6.4.117,
/// 6.4.116, 6.4.115 and 6.4.43 right after 7.3.86, not last as they sit
/// here.
const VIKALPA_RULES: &[&str] = &[
    "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3", "2573.2", "6.1.54",
    "7.3.37.2", "7.1.35", "3.4.111", "7.3.86", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56",
    "6.4.115", "6.4.117", "6.4.116", "6.4.43",
];

/// `ALTERNATES` is otherwise 10646 bare strings, and a string can be right for
/// the wrong reason — `BavatAt` is a real form whether or not 8.4.56 is what
/// produced it. This ties each row to the grammar: find the branch that
/// derives the row's form, intersect its log with the optional-rule set, and
/// require exactly the rules the row claims.
///
/// `VIKALPA_RULES` holds ids, not arms: 7.3.86 is listed once but runs three times,
/// as the mandatory guṇa-stage laghūpadha guṇa, as the tanādi vikalpa entry and,
/// since slice 10a, as the sanādi entry before ṇic, so a key
/// naming 7.3.86 does not by itself mean the rule was optional. Seventy of the 77 `7.3.86+8.4.56` keys are the mandatory firing (ten juhotyādi: 3e's laṅ eka cells and 3f's √kit and √dhiṣ ones; sixty curādi: slice 10a's √cur laṅ and vidhiliṅ prathama eka, and slice 10g's √div and √śṛdh, slice 10h's sixteen laghu-ik ādhṛṣīya rows and slice 10i's eleven laghu-ik rows on their ṇic branch, where the firing is the sanādi entry before ṇic; the other seven are the tanādi vikalpa), and so is the 7.3.86 of the one `7.3.86+8.2.75` key (√kit's `acikeH`), of every `7.3.86+7.1.35`/`7.3.86+7.1.35+8.4.56` key (the same curādi roots' loṭ tātaṅ cells — the only keys where 7.3.86 precedes 7.1.35, because the sanādi stage runs first), and of every `2570+…7.3.86…`, `2571+…7.3.86…`, `10.0498+…7.3.86…` and `10.0499+…7.3.86…` key (a ṇic-less root's guṇa).
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
            // The pinned form is the first LIVE branch. Before slice 10f that
            // was always index 0. An optional-ṇic root's ṇic branch is index
            // 0 and, in an ākusmīya or ā-garvīya root's parasmaipada, blocked
            // by 10.0496 / 10.0497: a later (applied) branch is the cell.
            let first_live = branches
                .iter()
                .find(|p| !p.blocked)
                .unwrap_or_else(|| panic!("no live branch for {root} {lakara} cell {cell}"));
            assert_eq!(
                first_live.text(),
                *expected,
                "the pinned form must be the first live branch for {root} {lakara} cell {cell}"
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
/// 26424 cells total (2936 root×lakāra blocks × 9), of which 18648 hold exactly one form, 6432 hold two, 601 hold three (√hrī's loṭ prathama and madhyama
/// eka, new in slice 3b, √dā's and √dhā's, new in slice 3c, and √gā's, new in
/// slice 3c2, and the six ṛ-roots', new in slice 3d, and √ṛ's, new in slice 3d2, and √ṇij's, √vij's and √viṣ's, new in slice 3e, and √kit's, √tur's, √dhiṣ's and √dhan's, new in slice 3f, and √bhas's, new in slice 3f2, and √jan's, new in slice 3f3, and the four curādi roots', new in slice 10a, and the six jñapādi roots', new in slice 10d, and the eighty-three ubhayapadī adanta roots', new in slice 10e, each by
/// 7.1.35/8.4.56, plus √bhas's laṅ madhyama eka, by 8.2.74/8.4.56; slice 10f's
/// optional-ṇic rows add 114 two-form and 46 three-form cells, and slice 10g's
/// fifty-nine add 1888 two-form cells, a ṇic and a ṇic-less reading each, and
/// slice 10h's fifty add 1680 two-form cells the same way, in parasmaipada and
/// in four `NicUbhayapada` rows' ātmanepada, and 136 three-form ones, √dhū's
/// and √prī's ṇic branch forking again on 7.3.37.2's nuk, and slice 10i's
/// sixty-one add 1952 two-form cells, a ṇic and a ṇic-less reading each, and
/// slice 10j's √ghṛ, √jñā, √cyu and √bhū eight two-form and eight three-form
/// cells as √cur's, and √ci 68 three-form ones), 359 hold four (piṣ's loṭ madhyama eka, the deepest
/// fork added in 7b, Siz's loṭ parasmaipada madhyama eka (slice 7d), and — new in
/// slice 8a — fifteen more spread across the four ik-upadhā tanādi roots
/// kziR/fR/tfR/GfR; √kṛ, slice 8b, adds none to this bucket; and — new in slice
/// 3b — √bhī's vidhiliṅ prathama eka, forking on 6.4.115 alongside 8.4.56; and — new in
/// slice 3f3 — √jan's vidhiliṅ prathama eka, forking on 6.4.43 alongside 8.4.56; and —
/// new in slice 10f — `mUtra`'s and `katra`'s laṅ and vidhiliṅ parasmaipada
/// prathama eka, their ṇic and ṇic-less readings × 8.4.56; and — new in slice
/// 10g — the fifty-nine optional-ṇic rows' laṅ and vidhiliṅ parasmaipada
/// prathama eka, the same way; and — new in slice 10h — forty-eight of the
/// fifty ādhṛṣīya rows', the same way; and — new in slice 10i — the
/// sixty-one āsvadīya, √pṝ and √ghuṣ rows', the same way), and
/// — the sharpest branch-count witnesses in
/// the repo, per `docs/ARCHITECTURE.md` — ten hold five (√kṛt's loṭ
/// prathama eka, ruD's loṭ parasmaipada prathama eka, Bid's, kzud's and
/// tfd's loṭ parasmaipada prathama eka, und's (slice 7d), and — new in
/// slice 7f — Cid's and Cfd's loṭ parasmaipada prathama eka; tanādi 8a adds
/// none to this bucket, neither does √kṛ, slice 8b, and — new in slice
/// 3b — √bhī's loṭ prathama eka, forking on 7.1.35/6.4.115/8.4.56, and — new in slice 3c2 — √hā's loṭ
/// prathama eka, forking on 7.1.35/6.4.116/8.4.56) and
/// 365
/// hold six (√kṛt's loṭ madhyama eka, `kfndDi`/`kfnDi`'s cell, ruD's loṭ
/// parasmaipada madhyama eka, `rundDi`/`runDi`/`rundDAd`/`runDAd`/
/// `rundDAt`/`runDAt`, Bid's, kzud's and tfd's loṭ
/// parasmaipada madhyama eka, und's (slice 7d), Cid's and Cfd's loṭ
/// parasmaipada madhyama eka (slice 7f — the record stood at eight cells
/// there), and — new in slice 8a — kziR's, fR's, tfR's and GfR's loṭ
/// parasmaipada prathama AND madhyama eka, each stacking 7.1.35/7.3.86/8.4.56
/// (k = 3 against the same 2³ bound of eight, this slice's own guṇa/aguṇa
/// fork standing in for 8.4.65's junction): the six-form record now stands
/// at sixteen cells, not eight — ric and vic do not
/// join this record; per the 8.2.30/8.2.39 slice's own audit their deepest
/// cells are three forms; √kṛ, slice 8b, joins neither the five- nor the
/// six-form record either — its sharpest cells (loṭ tātaṅ, prathama and
/// madhyama eka) hold three forms, 7.1.35 and 7.1.35+8.4.56 stacked against
/// the plain -oti skeleton, with no third rule available to stack a fourth;
/// and — new in slice 3b — √bhī's loṭ parasmaipada madhyama eka reaches six
/// by 7.1.35/6.4.115/8.4.56, a third distinct k = 3 stack against the same
/// 2³ bound of eight, beside rudhādi's 7.1.35/8.4.65/8.4.56 route and
/// tanādi's 7.1.35/7.3.86/8.4.56 route: the six-form record now stands at
/// seventeen cells, not sixteen, and at three mechanisms, not two; and — new in
/// slice 10f — six more: `pata`'s laṅ and vidhiliṅ parasmaipada prathama eka
/// (three readings × 8.4.56) and `mUtra`'s and `katra`'s loṭ parasmaipada
/// prathama and madhyama eka (two readings × the tātaṅ triple); and — new in
/// slice 10g — the fifty-nine optional-ṇic rows' loṭ parasmaipada prathama
/// and madhyama eka, the same way; and — new in slice 10h — those forty-eight
/// ādhṛṣīya rows', the same way, and √dhū's and √prī's laṅ and vidhiliṅ
/// parasmaipada prathama eka, three readings × 8.4.56; and — new in slice
/// 10i — the sixty-one rows' loṭ parasmaipada prathama and madhyama eka,
/// the same way; and — new in slice 10j — √ci's laṅ and vidhiliṅ parasmaipada
/// prathama eka, three readings × 8.4.56), one — new in
/// slice 3c2 — holds SEVEN: √hā's loṭ parasmaipada madhyama eka, where 6.4.117
/// is the first optional rule to bar others (`Rule.bars`), so its three
/// readings before *hi* are not a 2^k product; and eight hold NINE, the engine's
/// record: `pata`'s loṭ parasmaipada prathama and madhyama eka (new in slice
/// 10f), its three readings (2573.1's ṇic-less *pata-*, 2573.2's *pāta-*, and
/// 6.4.48's *pata-* with ṇic) × the tātaṅ triple, and — new in slice 10h —
/// √dhū's and √prī's, theirs (10.0498's ṇic-less one, 7.3.37.2's nuk, and
/// 7.2.115's vṛddhi with ṇic) × the same triple, and — new in slice 10j —
/// √ci's (2570's ṇic-less one, 6.1.54's `cA` with puk, and the declined
/// ṇic branch) × the same triple. No cell holds eight.
/// `ALTERNATES`
/// itself has 10646 rows, keyed 646 `8.4.56`, 638 `7.1.35`, 638 `7.1.35+8.4.56`,
/// 2 `3.4.111`, 72 `6.4.107`, 145 `8.4.65`, 8 `8.2.75`, 2 `8.2.74` (√hiṃs's ahinaH and, new in slice 3f2, √bhas's abaBaH), 16
/// `7.1.35+8.4.65`, 16 `7.1.35+8.4.65+8.4.56`, 270 `7.3.86` (tanādi 8a's
/// ik-upadhā fork), 8 `7.1.35+7.3.86`, 8 `7.1.35+7.3.86+8.4.56`, 8
/// `7.3.86+6.4.107`, 77 `7.3.86+8.4.56` (seventy of them name the MANDATORY
/// 7.3.86, through the id it shares with the tanādi vikalpa arm: ten juhotyādi, slice 3e's
/// laṅ prathama and madhyama eka cells and slice 3f's √kit and √dhiṣ ones, whose root guṇa 7.3.86 credits, and sixty curādi, slice 10a's √cur laṅ and vidhiliṅ prathama eka and slice 10g's √div and √śṛdh, slice 10h's sixteen laghu-ik ādhṛṣīya rows' and slice 10i's eleven laghu-ik rows' ṇic-branch ones, whose guṇa before ṇic the sanādi 7.3.86 credits; the other seven are the tanādi vikalpa), 1 `7.3.86+8.2.75` (√kit's acikeH, the same mandatory 7.3.86), 23 `6.4.115`, 2 `7.1.35+6.4.115`,
/// 2 `7.1.35+6.4.115+8.4.56`, and 1 `6.4.115+8.4.56`, 14 `6.4.116`, 1 `6.4.117`, 2 `7.1.35+6.4.116` and 2
/// `7.1.35+6.4.116+8.4.56`, 9 `6.4.43` and 1 `6.4.43+8.4.56` (slice 3f3's √jan), 60 `7.3.86+7.1.35` and 60
/// `7.3.86+7.1.35+8.4.56` (slice 10a's √cur, its sanādi 7.3.86 ahead of 7.1.35, and slice 10g's √śṛdh and
/// √div, slice 10h's sixteen laghu-ik ādhṛṣīya rows and slice 10i's eleven laghu-ik rows on their ṇic
/// branch), and slice 10f's twenty-one
/// keys on its five Kaumudī vikalpa ids, at their counts as of slice 10g: 36 `2573.1`, 72 `2573.2`, 72 `2573.3`, 114 apiece
/// `2564+8.4.56` / `2564+7.1.35` / `2564+7.1.35+8.4.56` (8 from 10f's four idit rows, 106 from 10g's
/// fifty-three), 6 apiece `2573.3+8.4.56` /
/// `2573.3+7.1.35` / `2573.3+7.1.35+8.4.56`, 10 apiece `2570+8.4.56`,
/// `2570+7.1.35` and `2570+7.1.35+8.4.56` (10f's √vañc, 10g's √śraṇ, both √jas and √añc), 6 apiece
/// `2570+7.3.86+8.4.56`, `2570+7.1.35+7.3.86` and
/// `2570+7.1.35+7.3.86+8.4.56` (10f's √div, 10g's √div and √śṛdh: the ṇic-less branch, whose root guṇa the
/// MANDATORY 7.3.86 credits), and 2 apiece `2573.1+8.4.56`, `2573.1+7.1.35`,
/// `2573.1+7.1.35+8.4.56`, `2573.2+8.4.56`, `2573.2+7.1.35` and
/// `2573.2+7.1.35+8.4.56`; its other eighteen rows fold into `8.4.56`,
/// `7.1.35` and `7.1.35+8.4.56`, six apiece. Slice 10g opens three keys,
/// 1908 `2564`, 144 `2570` and 72 `2570+7.3.86` — a ṇic-less form beside
/// the ṇic one in a cell that otherwise does not fork — and folds 114 rows
/// apiece into `8.4.56`, `7.1.35` and `7.1.35+8.4.56` and four apiece into
/// `7.3.86+8.4.56`, `7.3.86+7.1.35` and `7.3.86+7.1.35+8.4.56`. Slice 10h
/// opens twelve keys — 1404 `10.0498` and 612 `10.0498+7.3.86` (a ṇic-less
/// form beside the ṇic one, in parasmaipada and in a `NicUbhayapada` row's
/// ātmanepada; the second key's 7.3.86 is the MANDATORY guṇa of the sixteen
/// laghu-ik rows), 68 apiece `10.0498+8.4.56`, `10.0498+7.1.35` and
/// `10.0498+7.1.35+8.4.56`, 32 apiece `10.0498+7.3.86+8.4.56`,
/// `10.0498+7.1.35+7.3.86` and `10.0498+7.1.35+7.3.86+8.4.56`, 144
/// `7.3.37.2` and 4 apiece `7.3.37.2+8.4.56`, `7.3.37.2+7.1.35` and
/// `7.3.37.2+7.1.35+8.4.56` (√dhū's and √prī's nuk branch) — and folds 68
/// rows apiece into `8.4.56`, `7.1.35` and `7.1.35+8.4.56` and 32 apiece into
/// `7.3.86+8.4.56`, `7.3.86+7.1.35` and `7.3.86+7.1.35+8.4.56`. Slice 10i
/// opens sixteen keys — 1764 `10.0499` and 360 `10.0499+7.3.86` (the second
/// key's 7.3.86 the MANDATORY guṇa of the ten laghu-ik āsvadīya rows), 98
/// apiece `10.0499+8.4.56`, `10.0499+7.1.35` and `10.0499+7.1.35+8.4.56`,
/// 20 apiece `10.0499+7.3.86+8.4.56`, `10.0499+7.1.35+7.3.86` and
/// `10.0499+7.1.35+7.3.86+8.4.56`, 36 `2565` and 2 apiece `2565+8.4.56`,
/// `2565+7.1.35` and `2565+7.1.35+8.4.56` (√pṝ), 36 `2571+7.3.86` and 2
/// apiece `2571+7.3.86+8.4.56`, `2571+7.1.35+7.3.86` and
/// `2571+7.1.35+7.3.86+8.4.56` (√ghuṣ, whose ṇic-less guṇa is the same
/// mandatory 7.3.86) — and folds 100 rows apiece into `8.4.56`, `7.1.35` and
/// `7.1.35+8.4.56` and 22 apiece into `7.3.86+8.4.56`, `7.3.86+7.1.35` and
/// `7.3.86+7.1.35+8.4.56`. Slice 10j opens four keys — 72 `6.1.54` and 2
/// apiece `6.1.54+8.4.56`, `6.1.54+7.1.35` and `6.1.54+7.1.35+8.4.56` (√ci's
/// 6.1.54 branch) — adds 72 to `2570` and 2 apiece to `2570+8.4.56`,
/// `2570+7.1.35` and `2570+7.1.35+8.4.56` (√ci's ṇic-less branch), and folds
/// 10 rows apiece into `8.4.56`, `7.1.35` and `7.1.35+8.4.56` — √kṛ (slice 8b) adds six more
/// rows, all folded into the pre-existing `8.4.56`/`7.1.35`/`7.1.35+8.4.56`
/// keys above, two apiece: `8.4.56` gains `akarot` (laṅ parasmaipada
/// prathama eka) and `kuryAt` (vidhiliṅ parasmaipada prathama eka);
/// `7.1.35` gains `kurutAd` from both loṭ
/// parasmaipada's tātaṅ cell and its madhyama eka, and `7.1.35+8.4.56`
/// gains `kurutAt` from the same two cells — opening no new key. Slice 3b
/// (√bhī, √hrī) adds forty more: twelve folded into the same three
/// pre-existing keys, four apiece (`8.4.56` 134→138, `7.1.35` 112→116,
/// `7.1.35+8.4.56` 112→116), and twenty-eight on 6.4.115's four keys —
/// 23 `6.4.115` alone, 2 `7.1.35+6.4.115`, 2 `7.1.35+6.4.115+8.4.56` and 1
/// `6.4.115+8.4.56` — the engine's ninth vikalpa rule opening its first new
/// keys since 7.3.86 — slice 3c (√dā, √dhā, √mā, √hā) adds twelve more, all in
/// those same three keys, four apiece (`8.4.56` 138→142, `7.1.35` 116→120,
/// `7.1.35+8.4.56` 116→120) —
/// the assertions below are complete. The audit probe that produced the original numbers ran against
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
/// failing, and tanādi 8a's cross-implementation audit re-ran the same
/// probe against vidyut-prakriya at commit
/// `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` over all 3420 cells / 4321
/// forms / 76 roots with zero differences, its `entry` negative control
/// verified failing (36 √bhū cells), and tanādi 8b's cross-implementation
/// audit re-ran the same probe against vidyut-prakriya at the same commit
/// `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` over all 3492 cells / 4399
/// forms / 77 roots with zero differences, its `entry` negative control
/// verified failing (36 √bhū cells) first, and juhotyādi 3a's
/// cross-implementation audit re-ran the same probe against vidyut-prakriya
/// at the same commit `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` over all
/// 3564 cells / 4483 forms / 79 roots with zero differences, its `entry`
/// negative control verified failing first (exit 1, 36 √bhū cells), and
/// juhotyādi 3b's cross-implementation audit re-ran the same probe against
/// vidyut-prakriya at the same commit
/// `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` over all 3636 cells / 4595
/// forms / 81 roots with zero differences, its `entry` negative control
/// verified failing (36 √bhū cells), and juhotyādi 3c's cross-implementation
/// audit re-ran the same probe against vidyut-prakriya at the same commit
/// `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` over all 3852 cells / 4823
/// forms / 85 roots with zero differences, its `entry` negative control
/// verified failing (36 √bhū cells), and juhotyādi 3c2's re-ran the same probe
/// at the same commit over all 3924 cells / 4926 forms / 87 roots with zero
/// differences, its `entry` negative control verified failing (36 √bhū
/// cells), and juhotyādi 3d's re-ran it at the same commit over all 4176
/// cells / 5208 forms / 93 roots with zero differences, its `entry` negative
/// control verified failing (36 √bhū cells), and juhotyādi 3d2's re-ran it at
/// the same commit over all 4212 cells / 5249 forms / 94 roots with zero
/// differences, its `entry` negative control verified failing (36 √bhū
/// cells), and juhotyādi 3e's re-ran it at the same commit over all 4428
/// cells / 5486 forms / 97 roots with zero differences, its `entry` negative
/// control verified failing (36 √bhū cells), and juhotyādi 3f's re-ran it at
/// the same commit over all 4572 cells / 5655 forms / 101 roots with zero
/// differences, its `entry` negative control verified failing (36 √bhū
/// cells), and juhotyādi 3f2's re-ran it at the same commit over all 4608
/// cells / 5699 forms / 102 roots with zero differences, its `entry` negative
/// control verified failing (36 √bhū cells), and juhotyādi 3f3's re-ran it at
/// the same commit over all 4644 cells / 5750 forms / 103 roots with zero
/// differences, its `entry` negative control verified failing (36 √bhū
/// cells), and curādi 10a's re-ran it at the same commit over all 4932 cells /
/// 6062 forms / 107 roots with zero differences, its `entry` negative
/// control verified failing (36 √bhū cells), and curādi 10b's re-ran it at the
/// same commit over all 5076 cells / 6206 forms / 111 roots with zero
/// differences, its `entry` negative control verified failing (36 √bhū
/// cells), and curādi 10c's re-ran it at the same commit over all 6264 cells
/// / 7394 forms / 144 roots with zero differences, its `entry` negative
/// control verified failing (36 √bhū cells), and curādi 10d's re-ran it at
/// the same commit over all 6696 cells / 7862 forms / 150 roots with zero
/// differences, its `entry` negative control verified failing (36 √bhū
/// cells), and curādi 10e's re-ran it at the same commit over all 12996
/// cells / 14660 forms / 242 roots with zero differences, its `entry`
/// negative control verified failing (36 √bhū cells), and curādi 10f's
/// re-ran it at the same commit over all 13716 cells / 15644 forms / 252
/// roots with zero differences, its `entry` negative control verified
/// failing (36 √bhū cells), and curādi 10g's re-ran it at the same commit
/// over all 17964 cells / 22724 forms / 311 roots with zero differences,
/// its `entry` negative control verified failing (36 √bhū cells), and
/// curādi 10h's re-ran it at the same commit over all 21564 cells / 29096
/// forms / 361 roots with zero differences, its `entry` negative control
/// verified failing (36 √bhū cells), and curādi 10i's re-ran it at the same
/// commit over all 25956 cells / 36416 forms / 422 roots with zero
/// differences, its `entry` negative control verified failing (36 √bhū
/// cells), and curādi 10j's re-ran it at the same commit over all 26424
/// cells / 37070 forms / 430 roots with zero differences, its `entry`
/// negative control verified failing (36 √bhū cells). √tṛh joins none of the fork
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
///
/// Slice 8a leaves rudhādi behind and curates the tanādi gaṇa's first nine
/// roots — √tan (tan), √san (san), √kṣaṇ (kzaR), √kṣiṇ (kziR), √ṛṇ (fR),
/// √tṛ (tfR) and √ghṛ (GfR), all seven ubhayapadī by 1.3.72, plus
/// √van (van) and √man (man), both ātmanepada-only by 1.3.12 — on the
/// gaṇa's own vikaraṇa, 3.1.79 tanādikṛñbhya uḥ, and one optional rule new
/// to this repo, 7.3.86 (a guṇa/aguṇa fork available only to the four
/// ik-upadhā roots kziR/fR/tfR/GfR). tan, san and kzaR, the three regular
/// a-upadhā roots, fork exactly on the shape bhvādi's -oti roots already
/// established — 6.4.107 (laṭ and laṅ uttama dvi/bahu, both padas), 7.1.35
/// alone and stacked with 8.4.56 (loṭ parasmaipada tātaṅ, prathama and
/// madhyama eka), and 8.4.56 alone (laṅ parasmaipada prathama eka pausal
/// cartva, vidhiliṅ parasmaipada cell 0) — no new sūtra, nothing deeper
/// than two branches. The four ik-upadhā roots ride the same skeleton with
/// 7.3.86 layered on nearly every cell, reaching the corpus's first
/// four-deep stack (7.1.35+7.3.86+8.4.56, loṭ parasmaipada tātaṅ) and its
/// sharpest cells: kziR's, fR's, tfR's and GfR's loṭ parasmaipada prathama
/// AND madhyama eka each hold six live forms, stacking 7.1.35/7.3.86/8.4.56
/// where every earlier six-form cell stacked 7.1.35/8.4.65/8.4.56 — 8.4.65
/// has nothing to elide in tanādi's u-final stems, so 7.3.86's guṇa/aguṇa
/// fork stands in its place. fR's laṅ cells are where the convergence is
/// easiest to see, but not the only place it happens: all eighteen of
/// them (both padas, all nine puruṣa/vacana cells) are 7.3.86-eligible,
/// and none carries a 7.3.86-keyed `ALTERNATES` row — the guṇa branch
/// (`ArR-`) and the aguṇa branch converge on the same surface under
/// 6.1.90's āṭ-vṛddhi ekādeśa in every one of them, the corpus's first
/// convergent-fork collapse, not a one-cell anomaly. Prathama eka is
/// merely where the missing branch is most visible: it is the one laṅ
/// cell that also stacks 8.4.56's pausal cartva, so the comparable cell
/// at tfR's, GfR's and kziR's own laṅ parasmaipada prathama eka holds
/// **four** forms (`atfRod`/`atfRot`/`atarRod`/`atarRot` for tfR, and
/// likewise in shape for GfR and kziR), while fR's holds only two
/// (`ArRod` golden, `ArRot` alternate, keyed `8.4.56` alone) — the
/// missing branch is 7.3.86's, not 8.4.56's. √van and √man, ātmanepada-only, fork
/// only in laṭ and laṅ uttama dvi/bahu, on 6.4.107 — unlike every other
/// tanādi root, they take no loṭ fork at all, because 7.1.35 tātaṅ names
/// only the parasmaipada loṭ ending and van/man never derive a
/// parasmaipada branch for it to apply to.
///
/// Slice 8b curates √kṛ (`08.0010`, ubhayapada by 1.3.72), the tenth and
/// last tanādi root, closing the gaṇa at 10/10. It rides the gaṇa's own
/// vikaraṇa (3.1.79) but breaks the fork pattern every other tanādi root
/// shares: three new aṅga-level rules landed for it alone — 6.4.110
/// *ata ut sārvadhātuke* (7.3.84's guṇa has already turned the aṅga to
/// `kar`; 6.4.110 itself then replaces that `a` with `u` before a ṅit
/// sārvadhātuka ending, giving `kur`), 6.4.108 *nityaṃ karoteḥ*, and
/// 6.4.109 *ye ca* — and 6.4.108 is nitya where 6.4.107 is optional for
/// every other tanādi root: uttama dvi/bahu (`kurvaH`/`kurmaH`) derive as
/// single live branches with no 6.4.107-keyed alternate at all, not the
/// `tanuvaH`/`tanvaH`-style two-branch fork tan/san/kzaR/kziR/fR/tfR/GfR
/// all show there. √kṛ's only forks are the shapes every -oti root already
/// has: laṅ parasmaipada prathama eka (`akarod`/`akarot`, 8.4.56 pausal
/// cartva), loṭ parasmaipada tātaṅ — prathama eka (`karotu`/`kurutAd`/
/// `kurutAt`) and madhyama eka (`kuru`/`kurutAd`/`kurutAt`) alike, each a
/// three-form cell, 7.1.35 alone and stacked with 8.4.56 — and vidhiliṅ
/// parasmaipada prathama eka (`kuryAd`/`kuryAt`, 8.4.56 alone, keeping
/// 8.2.39's `d` on the declined
/// branch and devoicing to `t` only on the alternate — the `aBavad`
/// convention). Six `ALTERNATES` rows total, all folded into pre-existing
/// keys; no cell reaches four forms, so √kṛ joins neither the four-, five-
/// nor six-form record.
///
/// Slice 3a opens the ninth gaṇa, juhotyādi (3), the ślu gaṇa, with its
/// eponym √hu (`03.0001`) and √ki (`03.0020`), both parasmaipadī by 1.3.78
/// — the first roots whose derivations put a term in the `ABHYASA` slot.
/// Eight new sūtras (2.4.75 *ślu*, 6.1.10 *ślau* dvitva, 7.4.62, 3.4.109,
/// 7.1.4, 7.3.83, 6.4.82, 8.4.54) and two widened arms (6.4.87's and
/// 6.4.101's *hu*), yet no new fork kind: each root forks exactly where
/// every -oti parasmaipada root already does — laṅ prathama eka and
/// vidhiliṅ prathama eka on 8.4.56 (`ajuhod`/`ajuhot`, `juhuyAd`/`juhuyAt`),
/// and the two loṭ tātaṅ cells three ways (`juhotu`/`juhutAd`/`juhutAt`,
/// `juhuDi`/`juhutAd`/`juhutAt`) — six `ALTERNATES` rows per root, all in
/// pre-existing keys. The gaṇa is PARTIAL at 2 of its 26 rows.
///
/// Slice 3b curates two more juhotyādi roots, √bhī (`03.0002`) and √hrī
/// (`03.0003`), both parasmaipadī by 1.3.78, bringing the gaṇa to four of
/// its twenty-six rows. Two new sūtras shape the abhyāsa now that a
/// consonant-initial, long-vowel root sits in the `ABHYASA` slot — 7.4.60
/// *halādiḥ śeṣaḥ* and 7.4.59 *hrasvaḥ* — 6.4.82 widens from its 3a shape
/// to cover a long `I` (first exercised by √bhī), 6.4.77 gains an iyaṅ arm
/// for the saṁyogapūrva roots 6.4.82 declines on (first exercised by
/// √hrī), and 6.4.115 *bhiyo'nyatarasyām* lands as the engine's ninth
/// vikalpa rule — root-keyed to √bhī and forking across all four lakāras
/// (a kṅit-sārvadhātuka fork; loṭ is where it stacks with 7.1.35). √hrī
/// forks exactly where
/// every -oti parasmaipada root already does — laṅ prathama eka and
/// vidhiliṅ prathama eka on 8.4.56 (`ajihred`/`ajihret`,
/// `jihrIyAd`/`jihrIyAt`), and the two loṭ tātaṅ cells three ways
/// (`jihretu`/`jihrItAd`/`jihrItAt`, `jihrIhi`/`jihrItAd`/`jihrItAt`) — six
/// `ALTERNATES` rows, all in pre-existing keys, the same shape √hu and √ki
/// already established. √bhī reaches further: 6.4.115 stacks with 7.1.35
/// and 8.4.56 on its loṭ parasmaipada prathama eka
/// (`biBetu`/`biBItAd`/`biBitAd`/`biBitAt`/`biBItAt`, five forms) and its
/// loṭ parasmaipada madhyama eka
/// (`biBIhi`/`biBihi`/`biBItAd`/`biBitAd`/`biBItAt`/`biBitAt`, six forms) —
/// a third distinct k = 3 stack against the same 2³ bound of eight, beside
/// rudhādi's 7.1.35/8.4.65/8.4.56 route and tanādi's 7.1.35/7.3.86/8.4.56
/// route. The six-form record now stands at seventeen cells and three
/// mechanisms, not two. √bhī's vidhiliṅ prathama eka
/// (`biBIyAd`/`biBiyAd`/`biBiyAt`/`biBIyAt`) forks on 6.4.115 alongside
/// 8.4.56 alone, joining the four-form bucket. Forty new `ALTERNATES` rows
/// in total: twelve folded into the pre-existing
/// `8.4.56`/`7.1.35`/`7.1.35+8.4.56` keys, four apiece, and twenty-eight on
/// 6.4.115's four new keys. The gaṇa is PARTIAL at 4 of its 26 rows.
///
/// Slice 3c curates four more juhotyādi roots — √dā (`03.0010`) and √dhā
/// (`03.0011`), ubhayapadī by 1.3.72, and √mā (`03.0007`) and √hā
/// (`03.0008`), ātmanepadī by 1.3.12 — bringing the gaṇa to eight of its
/// twenty-six rows. Its machinery (1.1.20 as `Tag::Ghu`, 7.4.76, 6.4.119,
/// 6.1.88, 8.2.38, the 6.4.113/6.4.112 abhyasta arms, 8.2.40's *adhaḥ*)
/// adds no vikalpa and no fork kind: √dā and √dhā fork exactly where every
/// -oti parasmaipada root does — laṅ and vidhiliṅ prathama eka on 8.4.56
/// (`adadAd`/`adadAt`, `dadyAd`/`dadyAt`) and the two loṭ tātaṅ cells three
/// ways (`dadAtu`/`dattAd`/`dattAt`, `dehi`/`dattAd`/`dattAt`) — and their
/// ātmanepada columns, √mā and √hā fork nowhere. Twelve new `ALTERNATES`
/// rows, four apiece on `8.4.56` (138→142), `7.1.35` (116→120) and
/// `7.1.35+8.4.56` (116→120). The gaṇa is PARTIAL at 8 of its 26 rows.
///
/// Slice 3c2 curates √hā parasmaipada (`03.0009 o~hA\k`, jahāti) and √gā
/// (`03.0026`), both parasmaipadī by 1.3.78, bringing the gaṇa to ten of its
/// twenty-six rows. √gā forks exactly where every -oti parasmaipada root does
/// (`ajigAd`/`ajigAt`, `jigIyAd`/`jigIyAt`, and the two loṭ tātaṅ cells three
/// ways): six rows in the three pre-existing keys. √hā brings the engine's
/// tenth and eleventh vikalpas. 6.4.116 *jahāteś ca* forks every
/// consonant-initial kṅit cell (`jahItaH`/`jahitaH`). 6.4.117 *ā ca hau*
/// changes no text but bars 6.4.116/6.4.113/6.4.112 on its branch (`jahAhi`).
/// Together they make its loṭ madhyama eka the first seven-form cell. Its
/// vidhiliṅ takes 6.4.118 *lopo yi* (`jahyAt`) and forks only on 8.4.56.
/// Thirty-one new rows: 25 for √hā, 6 for √gā. The gaṇa is PARTIAL at 10 of
/// its 26 rows.
///
/// Slice 3d curates the six consonant-initial ṛ-roots — √pṝ (`03.0004`),
/// √pṛ (`03.0005`), √bhṛ (`03.0006`, ubhayapadī by 1.3.72), √ghṛ (`03.0015`),
/// √hṛ (`03.0016`) and √sṛ (`03.0018`) — bringing the gaṇa to sixteen of its
/// twenty-six rows. Its machinery (7.4.66, the widened 7.4.60, 7.4.76's √bhṛñ
/// row, 7.4.77, 7.1.102, 6.1.77's aṅga arm, 8.2.77 on the ślu path, 8.3.59's
/// `r` arm) adds no vikalpa and no fork kind. Each parasmaipada column forks
/// exactly where every -oti parasmaipada root does: vidhiliṅ prathama eka on 8.4.56
/// (`biBfyAd`/`biBfyAt`) and the two loṭ tātaṅ cells three ways
/// (`biBartu`/`biBftAd`/`biBftAt`, `biBfhi`/`biBftAd`/`biBftAt`). Laṅ prathama
/// eka does not fork: its `r` goes to visarga (`abiBaH`). √bhṛ's ātmanepada
/// column forks nowhere. Thirty new rows, five per root, all in the three
/// pre-existing keys. The gaṇa is PARTIAL at 16 of its 26 rows.
///
/// Slice 3d2 curates √ṛ (`03.0017`), the gaṇa's one vowel-initial ṛ-root,
/// bringing it to seventeen of its twenty-six rows. Its machinery (6.4.78,
/// 7.4.60 on a vowel-initial abhyāsa, 7.4.77's √ṛ row) adds no vikalpa and no
/// fork kind: vidhiliṅ prathama eka forks on 8.4.56 (`iyfyAd`/`iyfyAt`) and
/// the two loṭ tātaṅ cells three ways (`iyartu`/`iyftAd`/`iyftAt`,
/// `iyfhi`/`iyftAd`/`iyftAt`); laṅ prathama eka ends in visarga (`EyaH`) and
/// does not fork. Five new rows, all in the three pre-existing keys. The gaṇa
/// is PARTIAL at 17 of its 26 rows.
///
/// Slice 3e curates √ṇij (`03.0012`), √vij (`03.0013`) and √viṣ (`03.0014`),
/// ubhayapadī by 1.3.72, bringing the gaṇa to twenty of its twenty-six rows.
/// Its machinery (7.4.75, 7.3.87 barring 7.3.86) adds no vikalpa and no fork
/// kind. Each parasmaipada column forks where every consonant-final
/// parasmaipada root does: laṅ prathama AND madhyama eka on 8.4.56
/// (`aneneg`/`anenek`, 8.2.23 having taken the 2sg `s`), vidhiliṅ prathama eka
/// on 8.4.56 (`nenijyAd`/`nenijyAt`), and the two loṭ tātaṅ cells three ways
/// (`nenektu`/`neniktAd`/`neniktAt`, `nenigDi`/`neniktAd`/`neniktAt`). The
/// ātmanepada columns fork nowhere. Twenty-one new rows, seven per root, all in
/// pre-existing keys: the two laṅ eka rows per root fall in `7.3.86+8.4.56`
/// (7.3.86 credits the pit guṇa of `aneneg`), the vidhiliṅ row in `8.4.56`, and
/// the four loṭ tātaṅ rows per root in `7.1.35` and `7.1.35+8.4.56`. The gaṇa
/// is PARTIAL at 20 of its 26 rows.
///
/// Slice 3f curates √kit (`03.0021`), √tur (`03.0022`), √dhiṣ (`03.0023`) and
/// √dhan (`03.0024`), parasmaipadī by 1.3.78, bringing the gaṇa to twenty-four
/// of its twenty-six rows. It adds no rule and no vikalpa: 8.2.75 *daś ca*
/// loses its rudhādi gaṇa test and 8.3.24 admits juhotyādi. Every root forks
/// its vidhiliṅ prathama eka on 8.4.56 and its two loṭ tātaṅ cells three
/// ways. √kit and √dhiṣ also fork laṅ prathama and madhyama eka on 8.4.56
/// (`aciked`/`aciket`, `adiDeq`/`adiDew`), keyed `7.3.86+8.4.56` like 3e's,
/// and √kit's laṅ madhyama eka forks a third way on 8.2.75 (`acikeH`, keyed
/// `7.3.86+8.2.75`, the only such key). √tur's laṅ eka ends in visarga and
/// √dhan's in `n`, so neither forks there. Twenty-five new rows. The gaṇa is
/// PARTIAL at 24 of its 26 rows.
///
/// Slice 3f2 curates √bhas (`03.0019`), parasmaipadī by 1.3.78, bringing the
/// gaṇa to twenty-five of its twenty-six rows. It adds 6.4.100 *ghasibhasor
/// hali ca* and 8.2.26 *jhalo jhali*, drops the rudhādi gaṇa test from 8.2.73
/// and 8.2.74, and lets 8.4.55 read the whole word; no new vikalpa. √bhas
/// forks its vidhiliṅ and laṅ prathama eka on 8.4.56 (`bapsyAd`/`bapsyAt`,
/// `abaBad`/`abaBat`), its two loṭ tātaṅ cells three ways, and its laṅ
/// madhyama eka three ways on 8.2.74 and 8.4.56 (`abaBad`/`abaBat`/`abaBaH`)
/// — the second `8.2.74` key, after √hiṃs's. Eight new rows. The gaṇa is
/// PARTIAL at 25 of its 26 rows.
///
/// Slice 3f3 curates √jan (`03.0025`), parasmaipadī by 1.3.78, and closes the
/// gaṇa at all twenty-six of its rows. It adds 6.4.98 *gamahanajanakhanaghasāṁ
/// lopaḥ*, 6.4.42 *janasanakhanāṁ sañjhaloḥ* and the engine's twelfth vikalpa,
/// 6.4.43 *ye vibhāṣā*, and gives 8.4.40 its converse arm. 6.4.43 forks all
/// nine vidhiliṅ cells (`jajanyAm`/`jajAyAm`), the prathama eka four ways with
/// 8.4.56 — the first `6.4.43` keys — and √jan's two loṭ tātaṅ cells fork
/// three ways on 7.1.35/8.4.56. Fifteen new rows. The gaṇa is COMPLETE.
///
/// Slice 10a opens curādi (gaṇa 10) with √cur, √laḍ, √bhakṣ and √bhūṣ
/// (`10.0001`, `10.0010`, `10.0033`, `10.0255`), ubhayapadī by 1.3.74. No new
/// vikalpa rule: each root's parasmaipada forks only where every thematic
/// root does — laṅ and vidhiliṅ prathama eka on 8.4.56, the two loṭ tātaṅ
/// cells three ways on 7.1.35/8.4.56. √cur's keys carry its mandatory
/// sanādi 7.3.86 in front, opening `7.3.86+7.1.35` and
/// `7.3.86+7.1.35+8.4.56`. Twenty-four new rows. The gaṇa is OPEN at 4 of
/// its 509 rows.
///
/// Slice 10b curates four ākusmīya roots, √cit, √vṛṣ, √mad and √kusm
/// (`10.0192`, `10.0228`, `10.0229`, `10.0236`), ātmanepadī by the
/// dhātupāṭha gaṇasūtra 10.0496 *ā kusmād ātmanepadinaḥ*. Ātmanepada only,
/// and the thematic ātmanepada paradigm forks nowhere, so all 144 new cells
/// hold one form. No new rows. The gaṇa is OPEN at 8 of its 509 rows.
///
/// Slice 10c curates thirty-three more ākusmīya roots (`10.0195` through
/// `10.0234`; six take 7.3.86 before ṇic, ten 7.2.116, seventeen neither),
/// again ātmanepada only and one form per cell: 1188 new cells, no new
/// rows. `10.0233 mAna~` and `10.0234 mana~` share every form. The gaṇa is
/// OPEN at 41 of its 509 rows.
///
/// Slice 10d curates six of the seven jñapādi roots, √jñap, √yam, √cah,
/// √cap, √rah and √bal (`10.0118` through `10.0123`), mit by the gaṇasūtra
/// 10.0493 and ubhayapadī by 1.3.74. 6.4.92 *mitāṃ hrasvaḥ* shortens back
/// the upadhā 7.2.116 lengthened before ṇic (*jñapayati*). No new vikalpa
/// rule: each root forks only where √bhūṣ does — laṅ and vidhiliṅ
/// parasmaipada prathama eka on 8.4.56, the two loṭ tātaṅ cells three ways
/// on 7.1.35/8.4.56 — since neither 7.2.116 nor 6.4.92 is a vikalpa key.
/// 432 new cells, thirty-six new rows. The gaṇa is OPEN at 47 of its 509
/// rows.
///
/// Slice 10e curates ninety-two adanta roots (`10.0108 mArga` and `10.0389
/// kaTa` … `10.0492 Deka`). 6.4.48 *ato lopaḥ ārdhadhātuke* deletes the final
/// `a` before ṇic, and by 1.1.57 neither 7.2.116 nor 7.3.86 reads the
/// upadhā (*kathayati*, *kuhayate*). Eighty-three are ubhayapadī by 1.3.74
/// and fork exactly where √cur does; the nine ā-garvīya (`10.0440 pada` …
/// `10.0448 satra`) are ātmanepadī by the gaṇasūtra 10.0497, one form per
/// cell. 6300 new cells, 498 new rows. The gaṇa is OPEN at 139 of its 509
/// rows.
///
/// Slice 10f curates the ten optional-ṇic rows (Kaumudī 2564, 2570, 2573.1,
/// 2573.3): six ākusmīya, `garva`, `mUtra`, `katra` and `pata`. Each forks
/// into a ṇic branch, at index 0, and a ṇic-less one, parasmaipada by 1.3.78;
/// `pata`'s ṇic branch forks again on 2573.2. In an ākusmīya or ā-garvīya
/// root's parasmaipada the ṇic branch blocks, so the pinned form is the
/// first live branch, the ṇic-less one. 720 new cells, 264 new rows. The
/// gaṇa is OPEN at 149 of its 509 rows.
///
/// Slice 10g curates fifty-nine more optional-ṇic rows: every idit (Kaumudī
/// 2564) and ñit/udit (2570) curādi row outside the āsvadīya and ādhṛṣīya but
/// `10.0124 ciY`, all `Nic`. The ṇic branch is live in both padas, so it is
/// the pinned form; every parasmaipada cell adds the ṇic-less reading
/// beside it. 4248 new cells, 2832 new rows. The gaṇa is OPEN at 208 of its
/// 509 rows.
///
/// Slice 10h curates fifty of the fifty-one ādhṛṣīya rows (the gaṇasūtra
/// 10.0498 *ā dhṛṣād vā*; `10.0368 za\da~` waits for upasargas), forty-four
/// `Nic` and six `NicUbhayapada`. The ṇic branch is the pinned form; every
/// parasmaipada cell adds the ṇic-less reading, and so does every
/// ātmanepada cell of a `NicUbhayapada` row, 1.3.72's. √dhū's and √prī's ṇic
/// branch forks again on 7.3.37.2's nuk, three readings per cell. 3600 new
/// cells, 2772 new rows. The gaṇa is OPEN at 258 of its 509 rows.
///
/// Slice 10i curates the fifty-nine āsvadīya rows (the gaṇasūtra 10.0499
/// *ā svadaḥ sakarmakāt*), `10.0022 pF` (Kaumudī 2565) and `10.0251 Guzi~r`
/// (2571), all `Nic`. The ṇic branch is the pinned form; every parasmaipada
/// cell adds the ṇic-less reading beside it — for √dhūp and √vich, 3.1.28's
/// āya stem (*dhūpāyati*, *vicchāyati*). 4392 new cells, 2928 new rows. The
/// gaṇa is OPEN at 319 of its 509 rows.
///
/// Slice 10j curates the eight ajanta rows 7.2.115, 6.1.54 and 7.3.36 reach:
/// √ghṛ, √jñā, √cyu and √bhū (`Nic`), which fork exactly where √cur does;
/// √gṛ and √yu, ākusmīya, and √smiṅ, ātmanepadī under ṇic by Kaumudī 2567,
/// one form per cell; and √ci (`NicUbhayapada`), whose every cell holds
/// three readings: the ṇic branch declined (*cayayati*, the pinned form),
/// 6.1.54's (*capayati*) and 2570's ṇic-less one (*cayati*), in both padas.
/// 468 new cells, 186 new rows. The gaṇa is OPEN at 327 of its 509 rows.
/// This test is what keeps the numbers true day to day.
#[test]
fn derivation_set_shape_matches_the_audited_numbers() {
    let total_cells = PARADIGM.len() * 9;
    assert_eq!(total_cells, 26424, "2936 root×lakāra blocks × 9 cells each");

    let mut ones = 0usize;
    let mut twos = 0usize;
    let mut threes = 0usize;
    let mut fours = 0usize;
    let mut fives = 0usize;
    let mut sixes = 0usize;
    let mut sevens = 0usize;
    let mut nines = 0usize;
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
                7 => sevens += 1,
                9 => nines += 1,
                n => panic!("unexpected {n}-form cell in ({root}, {lakara}, {cell})"),
            }
        }
    }
    assert_eq!(ones, 18648, "one-form cells");
    assert_eq!(twos, 6432, "two-form cells");
    assert_eq!(
        threes, 601,
        "three-form cells — new in slice 3b — √hrī's loṭ prathama and madhyama eka, each by \
         7.1.35/8.4.56; and — new in slice 3c — √dā's and √dhā's, the same way; and — new in \
         slice 3c2 — √gā's, the same way; and — new in slice 3d — the six ṛ-roots', the same way; \
         and — new in slice 3d2 — √ṛ's, the same way; and — new in slice 3e — √ṇij's, √vij's and \
         √viṣ's, the same way; and — new in slice 3f — √kit's, √tur's, √dhiṣ's and √dhan's, \
         the same way; and — new in slice 3f2 — √bhas's laṅ madhyama eka (8.2.74 beside 8.4.56) \
         and its two loṭ tātaṅ cells; and — new in slice 3f3 — √jan's two loṭ tātaṅ cells; \
         and — new in slice 10a — the four curādi roots' two loṭ tātaṅ cells each, by \
         7.1.35/8.4.56; and — new in slice 10d — the six jñapādi roots', the same way; and — new \
         in slice 10e — the eighty-three ubhayapadī adanta roots', the same way; and — new in slice \
         10f — the optional-ṇic rows' (2564/2570/2573.x beside 7.1.35/8.4.56); and — new in \
         slice 10h — √dhū's and √prī's every cell that forks on neither 7.1.35 nor 8.4.56 \
         (10.0498 and 7.3.37.2 beside the ṇic form); and — new in slice 10j — √ghṛ's, √jñā's, \
         √cyu's and √bhū's two loṭ tātaṅ cells, by 7.1.35/8.4.56, and √ci's every cell that \
         forks on neither (6.1.54 and 2570 beside the ṇic form)"
    );
    assert_eq!(
        fours, 359,
        "four-form cells — piṣ's loṭ madhyama eka, Siz's loṭ parasmaipada madhyama eka (slice \
         7d), and — new in slice 8a — fifteen more across the four ik-upadhā roots (kziR, fR, \
         tfR, GfR), each forking on 7.3.86 alongside the pre-existing rules; and — new in \
         slice 3b — √bhī's vidhiliṅ prathama eka, forking on 6.4.115 alongside 8.4.56; and — \
         new in slice 3f3 — √jan's vidhiliṅ prathama eka, forking on 6.4.43 alongside 8.4.56; \
         and — new in slice 10f — mUtra's and katra's laṅ and vidhiliṅ parasmaipada prathama \
         eka, 2573.3 alongside 8.4.56; and — new in slice 10g — the fifty-nine optional-ṇic \
         rows' laṅ and vidhiliṅ parasmaipada prathama eka, 2564/2570 alongside 8.4.56; and — \
         new in slice 10h — forty-eight ādhṛṣīya rows' the same cells, 10.0498 alongside 8.4.56; \
         and — new in slice 10i — the sixty-one rows' the same cells, 10.0499/2565/2571 \
         alongside 8.4.56"
    );
    assert_eq!(
        fives, 10,
        "five-form cells — kft loṭ prathama eka, ruD loṭ parasmaipada prathama eka, Bid, kzud \
         and tfd's loṭ parasmaipada prathama eka, und's (slice 7d), and — new in slice 7f — \
         Cid's and Cfd's loṭ parasmaipada prathama eka; and — new in slice 3b — √bhī's loṭ \
         prathama eka, forking on 7.1.35/6.4.115/8.4.56; and — new in slice 3c2 — √hā's loṭ \
         prathama eka, forking on 7.1.35/6.4.116/8.4.56"
    );
    assert_eq!(
        sixes, 365,
        "six-form cells — kft loṭ madhyama eka, ruD loṭ parasmaipada madhyama eka, Bid, kzud \
         and tfd's loṭ parasmaipada madhyama eka, und's (slice 7d), Cid's and Cfd's loṭ \
         parasmaipada madhyama eka (slice 7f), and — new in slice 8a — kziR, fR, tfR and GfR's \
         loṭ parasmaipada prathama AND madhyama eka (the four ik-upadhā roots, each stacking \
         7.3.86 alongside 7.1.35/8.4.56); and — new in slice 3b — √bhī's loṭ parasmaipada \
         madhyama eka, a third distinct k=3 stack (7.1.35/6.4.115/8.4.56) beside rudhādi's \
         7.1.35/8.4.65/8.4.56 and tanādi's 7.1.35/7.3.86/8.4.56; and — new in slice 10f — \
         pata's laṅ and vidhiliṅ parasmaipada prathama eka (2573.1/2573.2 beside 8.4.56) and \
         mUtra's and katra's loṭ parasmaipada prathama and madhyama eka (2573.3 beside \
         7.1.35/8.4.56); and — new in slice 10g — the fifty-nine optional-ṇic rows' loṭ \
         parasmaipada prathama and madhyama eka (2564/2570 beside 7.1.35/8.4.56); and — new in \
         slice 10h — forty-eight ādhṛṣīya rows' the same cells (10.0498 beside 7.1.35/8.4.56), \
         and √dhū's and √prī's laṅ and vidhiliṅ parasmaipada prathama eka (10.0498 and \
         7.3.37.2 beside 8.4.56); and — new in slice 10i — the sixty-one rows' loṭ \
         parasmaipada prathama and madhyama eka (10.0499/2565/2571 beside 7.1.35/8.4.56); and \
         — new in slice 10j — √ci's laṅ and vidhiliṅ parasmaipada prathama eka (6.1.54 and 2570 \
         beside 8.4.56)"
    );
    assert_eq!(
        sevens, 1,
        "seven-form cells — new in slice 3c2, the record until slice 10f: √hā's loṭ parasmaipada \
         madhyama eka, three readings before hi (6.4.113's jahIhi, 6.4.116's jahihi, 6.4.117's \
         jahAhi, the last barring the other two) plus four tātaṅ forms (7.1.35 with 6.4.116 and \
         8.4.56 stacked)"
    );

    assert_eq!(
        nines, 8,
        "nine-form cells — new in slice 10f, the engine's record: pata's loṭ parasmaipada \
         prathama and madhyama eka, three readings (2573.1 ṇic-less, 2573.2 pAta-, 6.4.48 \
         pata-) × the tātaṅ triple (7.1.35/8.4.56); and — new in slice 10h — √dhū's and \
         √prī's, three readings (10.0498 ṇic-less, 7.3.37.2's nuk, 7.2.115's vṛddhi) × the \
         same triple; and — new in slice 10j — √ci's, three readings (2570 ṇic-less, 6.1.54's \
         `cA` with 7.3.36's puk, 7.2.115's vṛddhi) × the same triple"
    );

    assert_eq!(ALTERNATES.len(), 10646, "ALTERNATES row count");
    let key_count = |key: &str| {
        ALTERNATES
            .iter()
            .filter(|(_, _, _, _, _, k)| *k == key)
            .count()
    };
    assert_eq!(key_count("8.4.56"), 646, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 638, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 638, "7.1.35+8.4.56 alternates");
    assert_eq!(key_count("3.4.111"), 2, "3.4.111 alternates");
    assert_eq!(key_count("6.4.107"), 72, "6.4.107 alternates");
    assert_eq!(key_count("8.4.65"), 145, "8.4.65-only alternates");
    assert_eq!(key_count("8.2.75"), 8, "8.2.75-only alternates");
    assert_eq!(key_count("8.2.74"), 2, "8.2.74-only alternates");
    assert_eq!(key_count("7.1.35+8.4.65"), 16, "7.1.35+8.4.65 alternates");
    assert_eq!(
        key_count("7.1.35+8.4.65+8.4.56"),
        16,
        "7.1.35+8.4.65+8.4.56 alternates"
    );
    assert_eq!(key_count("7.3.86"), 270, "7.3.86-only alternates");
    assert_eq!(key_count("7.1.35+7.3.86"), 8, "7.1.35+7.3.86 alternates");
    assert_eq!(
        key_count("7.1.35+7.3.86+8.4.56"),
        8,
        "7.1.35+7.3.86+8.4.56 alternates"
    );
    assert_eq!(key_count("7.3.86+6.4.107"), 8, "7.3.86+6.4.107 alternates");
    assert_eq!(key_count("7.3.86+8.4.56"), 77, "7.3.86+8.4.56 alternates");
    assert_eq!(key_count("7.3.86+8.2.75"), 1, "7.3.86+8.2.75 alternates");
    assert_eq!(key_count("6.4.115"), 23, "6.4.115-only alternates");
    assert_eq!(key_count("7.1.35+6.4.115"), 2, "7.1.35+6.4.115 alternates");
    assert_eq!(
        key_count("7.1.35+6.4.115+8.4.56"),
        2,
        "7.1.35+6.4.115+8.4.56 alternates"
    );
    assert_eq!(key_count("6.4.115+8.4.56"), 1, "6.4.115+8.4.56 alternates");
    assert_eq!(key_count("6.4.116"), 14, "6.4.116-only alternates");
    assert_eq!(key_count("6.4.117"), 1, "6.4.117-only alternates");
    assert_eq!(key_count("7.1.35+6.4.116"), 2, "7.1.35+6.4.116 alternates");
    assert_eq!(
        key_count("7.1.35+6.4.116+8.4.56"),
        2,
        "7.1.35+6.4.116+8.4.56 alternates"
    );
    assert_eq!(key_count("6.4.43"), 9, "6.4.43-only alternates");
    assert_eq!(key_count("6.4.43+8.4.56"), 1, "6.4.43+8.4.56 alternates");
    assert_eq!(key_count("7.3.86+7.1.35"), 60, "7.3.86+7.1.35 alternates");
    assert_eq!(
        key_count("7.3.86+7.1.35+8.4.56"),
        60,
        "7.3.86+7.1.35+8.4.56 alternates"
    );
    // Slice 10f's five Kaumudī vikalpa ids, alone and stacked, with slice
    // 10g's fifty-nine rows: 2564 alone, 2570 alone and 2570+7.3.86 are 10g's.
    // Slice 10j's √ci adds 72 to 2570 alone, its ṇic-less branch in both
    // padas, and two to each of 2570's 8.4.56 and 7.1.35 stacks.
    for (key, n) in [
        ("2564", 1908),
        ("2570", 216),
        ("2570+7.3.86", 72),
        ("2573.1", 36),
        ("2573.2", 72),
        ("2573.3", 72),
        ("2564+8.4.56", 114),
        ("2564+7.1.35", 114),
        ("2564+7.1.35+8.4.56", 114),
        ("2573.3+8.4.56", 6),
        ("2573.3+7.1.35", 6),
        ("2573.3+7.1.35+8.4.56", 6),
        ("2570+8.4.56", 12),
        ("2570+7.1.35", 12),
        ("2570+7.1.35+8.4.56", 12),
        ("2570+7.3.86+8.4.56", 6),
        ("2570+7.1.35+7.3.86", 6),
        ("2570+7.1.35+7.3.86+8.4.56", 6),
        ("2573.1+8.4.56", 2),
        ("2573.1+7.1.35", 2),
        ("2573.1+7.1.35+8.4.56", 2),
        ("2573.2+8.4.56", 2),
        ("2573.2+7.1.35", 2),
        ("2573.2+7.1.35+8.4.56", 2),
    ] {
        assert_eq!(key_count(key), n, "{key} alternates");
    }
    // Slice 10h's two vikalpa ids, alone and stacked: the gaṇasūtra 10.0498's
    // ṇic-less branch (with the mandatory 7.3.86 on the sixteen laghu-ik
    // rows) and 7.3.37.2's nuk on √dhū's and √prī's ṇic branch.
    for (key, n) in [
        ("10.0498", 1404),
        ("10.0498+7.3.86", 612),
        ("10.0498+8.4.56", 68),
        ("10.0498+7.1.35", 68),
        ("10.0498+7.1.35+8.4.56", 68),
        ("10.0498+7.3.86+8.4.56", 32),
        ("10.0498+7.1.35+7.3.86", 32),
        ("10.0498+7.1.35+7.3.86+8.4.56", 32),
        ("7.3.37.2", 144),
        ("7.3.37.2+8.4.56", 4),
        ("7.3.37.2+7.1.35", 4),
        ("7.3.37.2+7.1.35+8.4.56", 4),
    ] {
        assert_eq!(key_count(key), n, "{key} alternates");
    }
    // Slice 10i's three vikalpa ids, alone and stacked: the gaṇasūtra
    // 10.0499's ṇic-less branch (with the mandatory 7.3.86 on the ten
    // laghu-ik rows), Kaumudī 2565's on √pṝ, and 2571's on √ghuṣ, whose
    // ṇic-less guṇa is the same mandatory 7.3.86.
    for (key, n) in [
        ("10.0499", 1764),
        ("10.0499+7.3.86", 360),
        ("10.0499+8.4.56", 98),
        ("10.0499+7.1.35", 98),
        ("10.0499+7.1.35+8.4.56", 98),
        ("10.0499+7.3.86+8.4.56", 20),
        ("10.0499+7.1.35+7.3.86", 20),
        ("10.0499+7.1.35+7.3.86+8.4.56", 20),
        ("2565", 36),
        ("2565+8.4.56", 2),
        ("2565+7.1.35", 2),
        ("2565+7.1.35+8.4.56", 2),
        ("2571+7.3.86", 36),
        ("2571+7.3.86+8.4.56", 2),
        ("2571+7.1.35+7.3.86", 2),
        ("2571+7.1.35+7.3.86+8.4.56", 2),
    ] {
        assert_eq!(key_count(key), n, "{key} alternates");
    }
    // Slice 10j's vikalpa id, alone and stacked: 6.1.54 on √ci's ṇic branch,
    // in both padas.
    for (key, n) in [
        ("6.1.54", 72),
        ("6.1.54+8.4.56", 2),
        ("6.1.54+7.1.35", 2),
        ("6.1.54+7.1.35+8.4.56", 2),
    ] {
        assert_eq!(key_count(key), n, "{key} alternates");
    }
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
    // arriving. √hu's and √ki's eight (root, lakāra, pada) triples -- 2
    // roots × 1 pada × 4 lakāras -- were likewise gated for one commit in
    // slice 3a.
    const GATED: &[(&str, &str, Pada)] = &[];

    let pinned: Vec<(&str, &str, Pada)> =
        PARADIGM.iter().map(|(r, l, p, _)| (*r, *l, *p)).collect();
    let mut unpinned: Vec<(&str, &str, Pada)> = Vec::new();
    for d in dhatus() {
        for &lakara in panini_analyze::LAKARAS {
            for &pada in d.padas() {
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
        .map(|d| d.padas().len() * panini_analyze::LAKARAS.len())
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

/// √pṛ (`03.0005 pf\`) and √pṝ (`03.0004 pF`) spell the same form wherever
/// guṇa has run — both aṅgas become `par` — and diverge wherever it has not
/// (`pf` stays, `pF` becomes `pur` by 7.1.102). `check` must report both roots
/// for the first kind and only the right one for the second.
#[test]
fn both_pr_roots_analyse_their_shared_forms() {
    let engine = Panini::new();
    let roots = |form: &str| {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        let mut v: Vec<String> = r.analyses.iter().map(|a| a.dhatu.clone()).collect();
        v.sort_unstable();
        v.dedup();
        v
    };
    for form in ["piparti", "apipaH", "piparARi"] {
        assert_eq!(roots(form), vec!["pF", "pf"], "{form}");
    }
    assert_eq!(roots("pipftaH"), vec!["pf"]);
    assert_eq!(roots("pipUrtaH"), vec!["pF"]);
}

/// √ṛ (`03.0017 f\`) is the corpus's only single-letter root and its only
/// vowel-initial abhyasta: every surface starts with the abhyāsa (`iy-`) or
/// the āṭ merged into it (`Ey-`), never with the root. `check` must still find
/// √ṛ, and only √ṛ.
#[test]
fn r_root_analyses_its_reduplicated_forms() {
    let engine = Panini::new();
    for form in ["iyarti", "EyaH", "iyrati", "iyfhi"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        let mut v: Vec<String> = r.analyses.iter().map(|a| a.dhatu.clone()).collect();
        v.sort_unstable();
        v.dedup();
        assert_eq!(v, vec!["f"], "{form}");
    }
}

/// √vij (`03.0013`) shares its code with tudādi's `06.0009` and rudhādi's
/// `07.0023`, so `check`'s `dhatu` field cannot tell the three apart. Only the
/// juhotyādi row reduplicates, and only it is credited 7.4.75. Every analysis
/// of a 3e form must carry 7.4.75 in its trace. A pada-ambiguous surface must
/// report both padas.
#[test]
fn nij_roots_analyse_their_reduplicated_forms() {
    let engine = Panini::new();
    for (form, root) in [
        ("nenekti", "nij"),
        ("nenijAni", "nij"),
        ("vevekti", "vij"),
        ("vevezwi", "viz"),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert!(!r.analyses.is_empty(), "{form}");
        for a in &r.analyses {
            assert_eq!(a.dhatu, root, "{form}");
            assert!(
                a.trace.iter().any(|s| s.sutra == "7.4.75"),
                "{form}: {:?}",
                a.trace
            );
        }
    }
    for form in ["neniktAm", "anenikta", "veviktAm", "avevizwa"] {
        let r = engine.check(form);
        let mut padas: Vec<Pada> = r.analyses.iter().map(|a| a.pada).collect();
        padas.dedup();
        assert!(
            padas.contains(&Pada::Parasmaipada) && padas.contains(&Pada::Atmanepada),
            "{form}: {padas:?}"
        );
    }
}

/// Slice 3f's four rows through `check`. Each code is unique among the
/// curated roots, so every analysis must name the 3f row. *daDaMhi* is 8.3.24's
/// first juhotyādi form. *acikeH* is the one surface a 3f row shares with a
/// prior row (√ki's laṅ madhyama eka), so it is checked apart below.
#[test]
fn kit_tur_dhish_dhan_analyse_their_reduplicated_forms() {
    let engine = Panini::new();
    for (form, root, sutra) in [
        ("ciketti", "kit", "7.4.62"),
        ("tutorti", "tur", "7.3.86"),
        ("diDezwi", "Diz", "8.4.41"),
        ("daDaMhi", "Dan", "8.3.24"),
        ("daDaMsi", "Dan", "8.3.24"),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert!(!r.analyses.is_empty(), "{form}");
        for a in &r.analyses {
            assert_eq!(a.dhatu, root, "{form}");
            assert!(
                a.trace.iter().any(|s| s.sutra == sutra),
                "{form}: {:?}",
                a.trace
            );
        }
    }
}

/// *acikeH* is √ki's laṅ madhyama eka (slice 3a) and, by 8.2.75 *daś ca* once
/// it loses its rudhādi gaṇa test, √kit's third reading. `check` must report
/// both roots, and √kit's analyses must carry 8.2.75.
#[test]
fn acikeh_analyses_as_both_ki_and_kit() {
    let r = Panini::new().check("acikeH");
    assert!(matches!(r.verdict, Verdict::Valid));
    let mut roots: Vec<&str> = r.analyses.iter().map(|a| a.dhatu.as_str()).collect();
    roots.sort_unstable();
    roots.dedup();
    assert_eq!(roots, vec!["ki", "kit"]);
    for a in r.analyses.iter().filter(|a| a.dhatu == "kit") {
        assert!(a.trace.iter().any(|s| s.sutra == "8.2.75"), "{:?}", a.trace);
    }
}

/// The surfaces that are genuinely pada-ambiguous — the same string pinned
/// as both a parasmaipada and an ātmanepada cell, so `check` reports two
/// analyses differing in pada. The list is of the PINNED (`PARADIGM`) forms
/// only: from slice 10h, a ṇic-less branch can be live in both padas, and
/// the surfaces that only `ALTERNATES` pins (`avadata`, say) are pada-ambiguous
/// too but uncounted here. `README.md` quotes this list; before this
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
    // surface. Tanādi 8a's seven ubhayapadī roots — tan, san, kzaR, kziR,
    // fR, tfR and GfR — each contribute two more, a new shape: a laṅ
    // `-uta` collision (ātmanepada prathama eka equals parasmaipada
    // madhyama bahu — `atanuta`, `asanuta`, `akzaRuta`, `akziRuta`,
    // `ArRuta`, `atfRuta`, `aGfRuta`) and a loṭ `-utAm` collision
    // (ātmanepada prathama eka equals parasmaipada prathama dvi —
    // `tanutAm`, `sanutAm`, `kzaRutAm`, `kziRutAm`, `fRutAm`, `tfRutAm`,
    // `GfRutAm`) — fourteen more, taking the set from twenty-eight to
    // forty-two with no new collision against any pre-slice surface. √van
    // and √man, ātmanepada-only, never enter this bucket: with no
    // parasmaipada branch of their own, they have nothing to collide
    // against. Tanādi 8b's √kṛ, ubhayapada, contributes exactly the same
    // two-surface shape as the seven 8a roots: a laṅ `-uta` collision
    // (`akuruta`, ātmanepada prathama eka = parasmaipada madhyama bahu)
    // and a loṭ `-utAm` collision (`kurutAm`, ātmanepada prathama eka =
    // parasmaipada prathama dvi), taking the set from forty-two to
    // forty-four with no new collision against any pre-slice surface.
    // Slice 3a's two roots, √hu and √ki, are parasmaipada-only and
    // contribute nothing; the set stands at forty-four. Slice 3c's √dā and
    // √dhā, ubhayapadī, each contribute the same two-surface shape as
    // tanādi's ubhayapadī roots: √dā's `dattAm` (loṭ ātmanepada prathama
    // eka = parasmaipada prathama dvi) and `adatta` (laṅ ātmanepada
    // prathama eka = parasmaipada madhyama bahu), and √dhā's `DattAm` and
    // `aDatta` the same way — taking the set from forty-four to
    // forty-eight with no new collision against any pre-slice surface.
    // √mā and √hā, ātmanepada-only, contribute nothing, like √van and
    // √man. Slice 3d's √bhṛ, ubhayapadī, contributes the same two-surface
    // shape as √dā's: `biBftAm` (loṭ ātmanepada prathama eka = parasmaipada
    // prathama dvi) and `abiBfta` (laṅ ātmanepada prathama eka = parasmaipada
    // madhyama bahu), taking the set from forty-eight to fifty with no new
    // collision against any pre-slice surface. The other five 3d roots are
    // parasmaipada-only and contribute nothing.
    // Slice 3e's √ṇij, √vij and √viṣ, ubhayapadī, each contribute the same
    // two-surface shape as √bhṛ's: `neniktAm`/`anenikta`,
    // `veviktAm`/`avevikta` and `vevizwAm`/`avevizwa`. That takes the set from
    // fifty to fifty-six, with no new collision against any pre-slice surface.
    // Measured: no 3e form equals any pre-slice form in either pada.
    // Slice 10a's four curādi roots, ubhayapadī by 1.3.74, are thematic like
    // √nī and √tud and contribute the same four-surface shape each: laṅ
    // ātmanepada prathama eka = parasmaipada madhyama bahu (`acorayata`), loṭ
    // ātmanepada prathama eka = parasmaipada prathama dvi (`corayatAm`), and
    // the two vidhiliṅ ones (`corayetAm`, `corayeta`) — sixteen more, taking
    // the set from fifty-six to seventy-two, with no new collision against
    // any pre-slice surface.
    // Slice 10d's six jñapādi roots, ubhayapadī by
    // 1.3.74, contribute the same four-surface shape each (`ajYapayata`,
    // `jYapayatAm`, `jYapayetAm`, `jYapayeta`) — twenty-four more, taking the
    // set from seventy-two to ninety-six, again with no collision against
    // any pre-slice surface. The ākusmīya roots, ātmanepada-only, contribute
    // nothing.
    // Slice 10e's eighty-three ubhayapadī adanta roots contribute the same
    // four-surface shape each (`akaTayata`, `kaTayatAm`, `kaTayetAm`,
    // `kaTayeta`), except `10.0396 raha` and `10.0405 caha`, whose surfaces
    // 10d's √rah and √cah already contributed — 324 more, taking the set from
    // ninety-six to 420. The nine ā-garvīya roots, ātmanepada-only, contribute
    // nothing; nor do `kUwa` and `vizka`, whose ātmanepada the ākusmīya `kUwa~`
    // and `vizka~` share, beyond their own four.
    // Slice 10f's `mUtra`, `katra` and `pata` contribute the same four each from
    // their ṇic branch (`amUtrayata`, …), which alone derives in both padas —
    // twelve more, taking the set from 420 to 432. The ākusmīya rows and
    // `garva` derive each pada on a different branch (ṇic-less parasmaipada,
    // ṇic ātmanepada), and those surfaces never meet: they contribute nothing.
    // Slice 10g's fifty-nine `Nic` rows contribute the same four each from
    // their ṇic branch — 236 — less the homographs': `lanj`, `vanw` and `jas`
    // each have two rows sharing all four (−12), and `10.0014 olanq` and
    // `10.0105 ulanq` share their laṅ one, `OlaRqayata` (−1). `10.0249 div`'s
    // four are new: 10f's `10.0230 div` has only their ātmanepada half. 223
    // more, taking the set from 432 to 655.
    // Slice 10h's fifty ādhṛṣīya rows contribute the same four each from
    // their ṇic branch — 200 — less the homographs': `tfp`, `dfB` and `granT`
    // each have two rows sharing all four (−12), and `10.0384 mArga~`'s four
    // are `10.0108 mArga`'s already (−4). `10.0381 mAna~`'s four are new:
    // `10.0233 mAna~`, ākusmīya, has only their ātmanepada half. 184 more,
    // taking the set from 655 to 839.
    // Slice 10i's sixty-one rows contribute the same four each from their
    // ṇic branch — 244 — less those already there: `10.0022 pF`'s four are
    // `10.0453 pAra`'s, and six rows share a code with a curated `Nic` row
    // whose four they already are (`tunj`, `pinj`, `lunj`, `lanj`, `lanq`, and
    // `SIk`, the ādhṛṣīya `10.0363`'s) (−28); and the two in-slice pairs,
    // `laGi~` twice and `svad` / `svAd`, share theirs (−8). `danS` and `dans`
    // add theirs: their earlier rows are ākusmīya, with only the ātmanepada
    // half. 208 more, taking the set from 839 to 1047.
    // Slice 10j's √ghṛ, √jñā, √cyu and √ci (its declined ṇic branch, the
    // pinned form) contribute the same four each — 16 more, taking the set
    // from 1047 to 1063. √bhū's four are the ādhṛṣīya `10.0382 BU`'s already;
    // √gṛ, √yu and √smiṅ are ātmanepadī only.
    assert_eq!(
        both,
        vec![
            "AMhayata",
            "AMsayata",
            "ANgayata",
            "ANkayata",
            "AYcayata",
            "AYjayata",
            "AnDayata",
            "ApayatAm",
            "Apayata",
            "ApayetAm",
            "Apayeta",
            "ArRuta",
            "Arcayata",
            "Ardayata",
            "Arhayata",
            "BAjayatAm",
            "BAjayetAm",
            "BAjayeta",
            "BAmayatAm",
            "BAmayetAm",
            "BAmayeta",
            "BAvayatAm",
            "BAvayetAm",
            "BAvayeta",
            "BUzayatAm",
            "BUzayetAm",
            "BUzayeta",
            "BaRqayatAm",
            "BaRqayetAm",
            "BaRqayeta",
            "BaYjayatAm",
            "BaYjayetAm",
            "BaYjayeta",
            "BakzayatAm",
            "BakzayetAm",
            "Bakzayeta",
            "BfMSayatAm",
            "BfMSayetAm",
            "BfMSayeta",
            "BinttAm",
            "BuNktAm",
            "CAdayatAm",
            "CAdayetAm",
            "CAdayeta",
            "CaYjayatAm",
            "CaYjayetAm",
            "CaYjayeta",
            "CadayatAm",
            "CadayetAm",
            "Cadayeta",
            "CampayatAm",
            "CampayetAm",
            "Campayeta",
            "CandayatAm",
            "CandayetAm",
            "Candayeta",
            "CardayatAm",
            "CardayetAm",
            "Cardayeta",
            "CarpayatAm",
            "CarpayetAm",
            "Carpayeta",
            "CedayatAm",
            "CedayetAm",
            "Cedayeta",
            "CfnttAm",
            "CidrayatAm",
            "CidrayetAm",
            "Cidrayeta",
            "CinttAm",
            "DAvayatAm",
            "DAvayetAm",
            "DAvayeta",
            "DUpayatAm",
            "DUpayetAm",
            "DUpayeta",
            "DarzayatAm",
            "DarzayetAm",
            "Darzayeta",
            "DattAm",
            "DekayatAm",
            "DekayetAm",
            "Dekayeta",
            "DvanayatAm",
            "DvanayetAm",
            "Dvanayeta",
            "Erayata",
            "GArayatAm",
            "GArayetAm",
            "GArayeta",
            "GAwayatAm",
            "GAwayetAm",
            "GAwayeta",
            "GaRwayatAm",
            "GaRwayetAm",
            "GaRwayeta",
            "GfRutAm",
            "GozayatAm",
            "GozayetAm",
            "Gozayeta",
            "IrayatAm",
            "IrayetAm",
            "Irayeta",
            "KaRqayatAm",
            "KaRqayetAm",
            "KaRqayeta",
            "KewayatAm",
            "KewayetAm",
            "Kewayeta",
            "KowayatAm",
            "KowayetAm",
            "Kowayeta",
            "KuRqayatAm",
            "KuRqayetAm",
            "KuRqayeta",
            "OlaRqayata",
            "Onayata",
            "SIkayatAm",
            "SIkayetAm",
            "SIkayeta",
            "SIlayatAm",
            "SIlayetAm",
            "SIlayeta",
            "SaWayatAm",
            "SaWayetAm",
            "SaWayeta",
            "SarDayatAm",
            "SarDayetAm",
            "SarDayeta",
            "SezayatAm",
            "SezayetAm",
            "Sezayeta",
            "SrARayatAm",
            "SrARayetAm",
            "SrARayeta",
            "SrATayatAm",
            "SrATayetAm",
            "SrATayeta",
            "SraTayatAm",
            "SraTayetAm",
            "SraTayeta",
            "SranTayatAm",
            "SranTayetAm",
            "SranTayeta",
            "SuRWayatAm",
            "SuRWayetAm",
            "SuRWayeta",
            "SunDayatAm",
            "SunDayetAm",
            "SunDayeta",
            "SvaRWayatAm",
            "SvaRWayetAm",
            "SvaRWayeta",
            "SvaWayatAm",
            "SvaWayetAm",
            "SvaWayeta",
            "UnayatAm",
            "UnayetAm",
            "Unayeta",
            "aBAjayata",
            "aBAmayata",
            "aBAvayata",
            "aBUzayata",
            "aBaRqayata",
            "aBaYjayata",
            "aBakzayata",
            "aBfMSayata",
            "aBintta",
            "aBuNkta",
            "aDAvayata",
            "aDUpayata",
            "aDarzayata",
            "aDatta",
            "aDekayata",
            "aDvanayata",
            "aGArayata",
            "aGAwayata",
            "aGaRwayata",
            "aGfRuta",
            "aGozayata",
            "aKaRqayata",
            "aKewayata",
            "aKowayata",
            "aKuRqayata",
            "aMhayatAm",
            "aMhayetAm",
            "aMhayeta",
            "aMsayatAm",
            "aMsayetAm",
            "aMsayeta",
            "aNgayatAm",
            "aNgayetAm",
            "aNgayeta",
            "aNkayatAm",
            "aNkayetAm",
            "aNkayeta",
            "aSIkayata",
            "aSIlayata",
            "aSaWayata",
            "aSarDayata",
            "aSezayata",
            "aSrARayata",
            "aSrATayata",
            "aSraTayata",
            "aSranTayata",
            "aSuRWayata",
            "aSunDayata",
            "aSvaRWayata",
            "aSvaWayata",
            "aYcayatAm",
            "aYcayetAm",
            "aYcayeta",
            "aYjayatAm",
            "aYjayetAm",
            "aYjayeta",
            "abalayata",
            "abalhayata",
            "abarhayata",
            "abfMhayata",
            "abiBfta",
            "acAnayata",
            "acAyayata",
            "acCAdayata",
            "acCaYjayata",
            "acCadayata",
            "acCampayata",
            "acCandayata",
            "acCardayata",
            "acCarpayata",
            "acCedayata",
            "acCfntta",
            "acCidrayata",
            "acCintta",
            "acIkayata",
            "acIvayata",
            "acaRqayata",
            "acahayata",
            "acampayata",
            "acapayata",
            "acarpayata",
            "acayayata",
            "acintayata",
            "acitrayata",
            "acorayata",
            "acuRwayata",
            "acumbayata",
            "acyAvayata",
            "adAlayata",
            "adaMSayata",
            "adaMhayata",
            "adaMsayata",
            "adaRqayata",
            "adarBayata",
            "adarpayata",
            "adatta",
            "adevayata",
            "aduHKayata",
            "agaRayata",
            "agadayata",
            "agarhayata",
            "agavezayata",
            "agomayata",
            "agopayata",
            "agrAmayata",
            "agrAsayata",
            "agranTayata",
            "aguRWayata",
            "aguRayata",
            "aguRqayata",
            "ahiMsayata",
            "ajArayata",
            "ajAsayata",
            "ajAyayata",
            "ajYApayata",
            "ajYapayata",
            "ajaMsayata",
            "ajamBayata",
            "ajozayata",
            "ajrAyayata",
            "akAlayata",
            "akUwayata",
            "akaRWayata",
            "akaRqayata",
            "akaTayata",
            "akalayata",
            "akatrayata",
            "aketayata",
            "akfpayata",
            "akopayata",
            "akuMSayata",
            "akuMsayata",
            "akuRWayata",
            "akuRayata",
            "akuRqayata",
            "akumArayata",
            "akumBayata",
            "akumbayata",
            "akundrayata",
            "akuruta",
            "akzaRuta",
            "akzaYjayata",
            "akzampayata",
            "akziRuta",
            "akzipayata",
            "akzowayata",
            "akzuntta",
            "alABayata",
            "alAkayata",
            "alAqayata",
            "alAwayata",
            "alAyayata",
            "alaNGayata",
            "alaRqayata",
            "alaYjayata",
            "alajayata",
            "aliNgayata",
            "alocayata",
            "alokayata",
            "alowayata",
            "aluYjayata",
            "alumbayata",
            "amAnayata",
            "amArgayata",
            "amArjayata",
            "amAyayata",
            "amUtrayata",
            "amaMhayata",
            "amaRqayata",
            "amahayata",
            "amarzayata",
            "amiSrayata",
            "amiYjayata",
            "amindayata",
            "anAdayata",
            "anAlayata",
            "anDayatAm",
            "anDayetAm",
            "anDayeta",
            "anaRwayata",
            "anayata",
            "anenikta",
            "anivAsayata",
            "apArayata",
            "apAwayata",
            "apUrayata",
            "apaMsayata",
            "apaRqayata",
            "apaWayata",
            "apaYcayata",
            "apalpUlayata",
            "apalyUlayata",
            "apanTayata",
            "aparRayata",
            "aparcayata",
            "apatayata",
            "apazayata",
            "apiMsayata",
            "apiRqayata",
            "apiYjayata",
            "apoTayata",
            "apowayata",
            "apozayata",
            "aprAyayata",
            "apuRwayata",
            "apuwayata",
            "arUkzayata",
            "arUpayata",
            "araMhayata",
            "aracayata",
            "arahayata",
            "aranDayata",
            "arasayata",
            "arcayatAm",
            "arcayetAm",
            "arcayeta",
            "ardayatAm",
            "ardayetAm",
            "ardayeta",
            "arecayata",
            "arhayatAm",
            "arhayetAm",
            "arhayeta",
            "ariNkta",
            "arojayata",
            "arowayata",
            "aruMSayata",
            "aruMsayata",
            "arundDa",
            "asAhayata",
            "asAmayata",
            "asArayata",
            "asPuRqayata",
            "asPuRwayata",
            "asUcayata",
            "asUtrayata",
            "asaBAjayata",
            "asaNgrAmayata",
            "asaNketayata",
            "asanuta",
            "aspfhayata",
            "aspuRqayata",
            "astanayata",
            "astenayata",
            "astomayata",
            "asuKayata",
            "asvAdayata",
            "asvarayata",
            "atAnayata",
            "atApayata",
            "atAqayata",
            "atIrayata",
            "ataMsayata",
            "atanuta",
            "atarkayata",
            "atarpayata",
            "atfRuta",
            "atfntta",
            "atraMsayata",
            "atuYjayata",
            "atudata",
            "atumbayata",
            "atutTayata",
            "avAcayata",
            "avAdayata",
            "avArayata",
            "avAsayata",
            "avAtayata",
            "avaRqayata",
            "avaRwayata",
            "avaWayata",
            "avalkayata",
            "avarDayata",
            "avarRayata",
            "avarayata",
            "avarjayata",
            "avartayata",
            "avasayata",
            "avawayata",
            "avelayata",
            "avevikta",
            "avevizwa",
            "aviNkta",
            "avicCayata",
            "avizkayata",
            "avraRayata",
            "avyayayata",
            "awaNkayata",
            "ayamayata",
            "ayantrayata",
            "ayojayata",
            "ayuNkta",
            "balayatAm",
            "balayetAm",
            "balayeta",
            "balhayatAm",
            "balhayetAm",
            "balhayeta",
            "barhayatAm",
            "barhayetAm",
            "barhayeta",
            "bfMhayatAm",
            "bfMhayetAm",
            "bfMhayeta",
            "biBftAm",
            "cAnayatAm",
            "cAnayetAm",
            "cAnayeta",
            "cAyayatAm",
            "cAyayetAm",
            "cAyayeta",
            "cIkayatAm",
            "cIkayetAm",
            "cIkayeta",
            "cIvayatAm",
            "cIvayetAm",
            "cIvayeta",
            "caRqayatAm",
            "caRqayetAm",
            "caRqayeta",
            "cahayatAm",
            "cahayetAm",
            "cahayeta",
            "campayatAm",
            "campayetAm",
            "campayeta",
            "capayatAm",
            "capayetAm",
            "capayeta",
            "carpayatAm",
            "carpayetAm",
            "carpayeta",
            "cayayatAm",
            "cayayetAm",
            "cayayeta",
            "cintayatAm",
            "cintayetAm",
            "cintayeta",
            "citrayatAm",
            "citrayetAm",
            "citrayeta",
            "corayatAm",
            "corayetAm",
            "corayeta",
            "cuRwayatAm",
            "cuRwayetAm",
            "cuRwayeta",
            "cumbayatAm",
            "cumbayetAm",
            "cumbayeta",
            "cyAvayatAm",
            "cyAvayetAm",
            "cyAvayeta",
            "dAlayatAm",
            "dAlayetAm",
            "dAlayeta",
            "daMSayatAm",
            "daMSayetAm",
            "daMSayeta",
            "daMhayatAm",
            "daMhayetAm",
            "daMhayeta",
            "daMsayatAm",
            "daMsayetAm",
            "daMsayeta",
            "daRqayatAm",
            "daRqayetAm",
            "daRqayeta",
            "darBayatAm",
            "darBayetAm",
            "darBayeta",
            "darpayatAm",
            "darpayetAm",
            "darpayeta",
            "dattAm",
            "devayatAm",
            "devayetAm",
            "devayeta",
            "duHKayatAm",
            "duHKayetAm",
            "duHKayeta",
            "fRutAm",
            "gaRayatAm",
            "gaRayetAm",
            "gaRayeta",
            "gadayatAm",
            "gadayetAm",
            "gadayeta",
            "garhayatAm",
            "garhayetAm",
            "garhayeta",
            "gavezayatAm",
            "gavezayetAm",
            "gavezayeta",
            "gomayatAm",
            "gomayetAm",
            "gomayeta",
            "gopayatAm",
            "gopayetAm",
            "gopayeta",
            "grAmayatAm",
            "grAmayetAm",
            "grAmayeta",
            "grAsayatAm",
            "grAsayetAm",
            "grAsayeta",
            "granTayatAm",
            "granTayetAm",
            "granTayeta",
            "guRWayatAm",
            "guRWayetAm",
            "guRWayeta",
            "guRayatAm",
            "guRayetAm",
            "guRayeta",
            "guRqayatAm",
            "guRqayetAm",
            "guRqayeta",
            "hiMsayatAm",
            "hiMsayetAm",
            "hiMsayeta",
            "jArayatAm",
            "jArayetAm",
            "jArayeta",
            "jAsayatAm",
            "jAsayetAm",
            "jAsayeta",
            "jAyayatAm",
            "jAyayetAm",
            "jAyayeta",
            "jYApayatAm",
            "jYApayetAm",
            "jYApayeta",
            "jYapayatAm",
            "jYapayetAm",
            "jYapayeta",
            "jaMsayatAm",
            "jaMsayetAm",
            "jaMsayeta",
            "jamBayatAm",
            "jamBayetAm",
            "jamBayeta",
            "jozayatAm",
            "jozayetAm",
            "jozayeta",
            "jrAyayatAm",
            "jrAyayetAm",
            "jrAyayeta",
            "kAlayatAm",
            "kAlayetAm",
            "kAlayeta",
            "kUwayatAm",
            "kUwayetAm",
            "kUwayeta",
            "kaRWayatAm",
            "kaRWayetAm",
            "kaRWayeta",
            "kaRqayatAm",
            "kaRqayetAm",
            "kaRqayeta",
            "kaTayatAm",
            "kaTayetAm",
            "kaTayeta",
            "kalayatAm",
            "kalayetAm",
            "kalayeta",
            "katrayatAm",
            "katrayetAm",
            "katrayeta",
            "ketayatAm",
            "ketayetAm",
            "ketayeta",
            "kfpayatAm",
            "kfpayetAm",
            "kfpayeta",
            "kopayatAm",
            "kopayetAm",
            "kopayeta",
            "kuMSayatAm",
            "kuMSayetAm",
            "kuMSayeta",
            "kuMsayatAm",
            "kuMsayetAm",
            "kuMsayeta",
            "kuRWayatAm",
            "kuRWayetAm",
            "kuRWayeta",
            "kuRayatAm",
            "kuRayetAm",
            "kuRayeta",
            "kuRqayatAm",
            "kuRqayetAm",
            "kuRqayeta",
            "kumArayatAm",
            "kumArayetAm",
            "kumArayeta",
            "kumBayatAm",
            "kumBayetAm",
            "kumBayeta",
            "kumbayatAm",
            "kumbayetAm",
            "kumbayeta",
            "kundrayatAm",
            "kundrayetAm",
            "kundrayeta",
            "kurutAm",
            "kzaRutAm",
            "kzaYjayatAm",
            "kzaYjayetAm",
            "kzaYjayeta",
            "kzampayatAm",
            "kzampayetAm",
            "kzampayeta",
            "kziRutAm",
            "kzipayatAm",
            "kzipayetAm",
            "kzipayeta",
            "kzowayatAm",
            "kzowayetAm",
            "kzowayeta",
            "kzunttAm",
            "lABayatAm",
            "lABayetAm",
            "lABayeta",
            "lAkayatAm",
            "lAkayetAm",
            "lAkayeta",
            "lAqayatAm",
            "lAqayetAm",
            "lAqayeta",
            "lAwayatAm",
            "lAwayetAm",
            "lAwayeta",
            "lAyayatAm",
            "lAyayetAm",
            "lAyayeta",
            "laNGayatAm",
            "laNGayetAm",
            "laNGayeta",
            "laRqayatAm",
            "laRqayetAm",
            "laRqayeta",
            "laYjayatAm",
            "laYjayetAm",
            "laYjayeta",
            "lajayatAm",
            "lajayetAm",
            "lajayeta",
            "liNgayatAm",
            "liNgayetAm",
            "liNgayeta",
            "locayatAm",
            "locayetAm",
            "locayeta",
            "lokayatAm",
            "lokayetAm",
            "lokayeta",
            "lowayatAm",
            "lowayetAm",
            "lowayeta",
            "luYjayatAm",
            "luYjayetAm",
            "luYjayeta",
            "lumbayatAm",
            "lumbayetAm",
            "lumbayeta",
            "mAnayatAm",
            "mAnayetAm",
            "mAnayeta",
            "mArgayatAm",
            "mArgayetAm",
            "mArgayeta",
            "mArjayatAm",
            "mArjayetAm",
            "mArjayeta",
            "mAyayatAm",
            "mAyayetAm",
            "mAyayeta",
            "mUtrayatAm",
            "mUtrayetAm",
            "mUtrayeta",
            "maMhayatAm",
            "maMhayetAm",
            "maMhayeta",
            "maRqayatAm",
            "maRqayetAm",
            "maRqayeta",
            "mahayatAm",
            "mahayetAm",
            "mahayeta",
            "marzayatAm",
            "marzayetAm",
            "marzayeta",
            "miSrayatAm",
            "miSrayetAm",
            "miSrayeta",
            "miYjayatAm",
            "miYjayetAm",
            "miYjayeta",
            "mindayatAm",
            "mindayetAm",
            "mindayeta",
            "nAdayatAm",
            "nAdayetAm",
            "nAdayeta",
            "nAlayatAm",
            "nAlayetAm",
            "nAlayeta",
            "naRwayatAm",
            "naRwayetAm",
            "naRwayeta",
            "nayatAm",
            "nayetAm",
            "nayeta",
            "neniktAm",
            "nivAsayatAm",
            "nivAsayetAm",
            "nivAsayeta",
            "olaRqayatAm",
            "olaRqayetAm",
            "olaRqayeta",
            "pArayatAm",
            "pArayetAm",
            "pArayeta",
            "pAwayatAm",
            "pAwayetAm",
            "pAwayeta",
            "pUrayatAm",
            "pUrayetAm",
            "pUrayeta",
            "paMsayatAm",
            "paMsayetAm",
            "paMsayeta",
            "paRqayatAm",
            "paRqayetAm",
            "paRqayeta",
            "paWayatAm",
            "paWayetAm",
            "paWayeta",
            "paYcayatAm",
            "paYcayetAm",
            "paYcayeta",
            "palpUlayatAm",
            "palpUlayetAm",
            "palpUlayeta",
            "palyUlayatAm",
            "palyUlayetAm",
            "palyUlayeta",
            "panTayatAm",
            "panTayetAm",
            "panTayeta",
            "parRayatAm",
            "parRayetAm",
            "parRayeta",
            "parcayatAm",
            "parcayetAm",
            "parcayeta",
            "patayatAm",
            "patayetAm",
            "patayeta",
            "pazayatAm",
            "pazayetAm",
            "pazayeta",
            "piMsayatAm",
            "piMsayetAm",
            "piMsayeta",
            "piRqayatAm",
            "piRqayetAm",
            "piRqayeta",
            "piYjayatAm",
            "piYjayetAm",
            "piYjayeta",
            "poTayatAm",
            "poTayetAm",
            "poTayeta",
            "powayatAm",
            "powayetAm",
            "powayeta",
            "pozayatAm",
            "pozayetAm",
            "pozayeta",
            "prAyayatAm",
            "prAyayetAm",
            "prAyayeta",
            "puRwayatAm",
            "puRwayetAm",
            "puRwayeta",
            "puwayatAm",
            "puwayetAm",
            "puwayeta",
            "rUkzayatAm",
            "rUkzayetAm",
            "rUkzayeta",
            "rUpayatAm",
            "rUpayetAm",
            "rUpayeta",
            "raMhayatAm",
            "raMhayetAm",
            "raMhayeta",
            "racayatAm",
            "racayetAm",
            "racayeta",
            "rahayatAm",
            "rahayetAm",
            "rahayeta",
            "ranDayatAm",
            "ranDayetAm",
            "ranDayeta",
            "rasayatAm",
            "rasayetAm",
            "rasayeta",
            "recayatAm",
            "recayetAm",
            "recayeta",
            "riNktAm",
            "rojayatAm",
            "rojayetAm",
            "rojayeta",
            "rowayatAm",
            "rowayetAm",
            "rowayeta",
            "ruMSayatAm",
            "ruMSayetAm",
            "ruMSayeta",
            "ruMsayatAm",
            "ruMsayetAm",
            "ruMsayeta",
            "rundDAm",
            "sAhayatAm",
            "sAhayetAm",
            "sAhayeta",
            "sAmayatAm",
            "sAmayetAm",
            "sAmayeta",
            "sArayatAm",
            "sArayetAm",
            "sArayeta",
            "sPuRqayatAm",
            "sPuRqayetAm",
            "sPuRqayeta",
            "sPuRwayatAm",
            "sPuRwayetAm",
            "sPuRwayeta",
            "sUcayatAm",
            "sUcayetAm",
            "sUcayeta",
            "sUtrayatAm",
            "sUtrayetAm",
            "sUtrayeta",
            "saBAjayatAm",
            "saBAjayetAm",
            "saBAjayeta",
            "saNgrAmayatAm",
            "saNgrAmayetAm",
            "saNgrAmayeta",
            "saNketayatAm",
            "saNketayetAm",
            "saNketayeta",
            "sanutAm",
            "spfhayatAm",
            "spfhayetAm",
            "spfhayeta",
            "spuRqayatAm",
            "spuRqayetAm",
            "spuRqayeta",
            "stanayatAm",
            "stanayetAm",
            "stanayeta",
            "stenayatAm",
            "stenayetAm",
            "stenayeta",
            "stomayatAm",
            "stomayetAm",
            "stomayeta",
            "suKayatAm",
            "suKayetAm",
            "suKayeta",
            "svAdayatAm",
            "svAdayetAm",
            "svAdayeta",
            "svarayatAm",
            "svarayetAm",
            "svarayeta",
            "tAnayatAm",
            "tAnayetAm",
            "tAnayeta",
            "tApayatAm",
            "tApayetAm",
            "tApayeta",
            "tAqayatAm",
            "tAqayetAm",
            "tAqayeta",
            "tIrayatAm",
            "tIrayetAm",
            "tIrayeta",
            "taMsayatAm",
            "taMsayetAm",
            "taMsayeta",
            "tanutAm",
            "tarkayatAm",
            "tarkayetAm",
            "tarkayeta",
            "tarpayatAm",
            "tarpayetAm",
            "tarpayeta",
            "tfRutAm",
            "tfnttAm",
            "traMsayatAm",
            "traMsayetAm",
            "traMsayeta",
            "tuYjayatAm",
            "tuYjayetAm",
            "tuYjayeta",
            "tudatAm",
            "tudetAm",
            "tudeta",
            "tumbayatAm",
            "tumbayetAm",
            "tumbayeta",
            "tutTayatAm",
            "tutTayetAm",
            "tutTayeta",
            "ulaRqayatAm",
            "ulaRqayetAm",
            "ulaRqayeta",
            "vAcayatAm",
            "vAcayetAm",
            "vAcayeta",
            "vAdayatAm",
            "vAdayetAm",
            "vAdayeta",
            "vArayatAm",
            "vArayetAm",
            "vArayeta",
            "vAsayatAm",
            "vAsayetAm",
            "vAsayeta",
            "vAtayatAm",
            "vAtayetAm",
            "vAtayeta",
            "vaRqayatAm",
            "vaRqayetAm",
            "vaRqayeta",
            "vaRwayatAm",
            "vaRwayetAm",
            "vaRwayeta",
            "vaWayatAm",
            "vaWayetAm",
            "vaWayeta",
            "valkayatAm",
            "valkayetAm",
            "valkayeta",
            "varDayatAm",
            "varDayetAm",
            "varDayeta",
            "varRayatAm",
            "varRayetAm",
            "varRayeta",
            "varayatAm",
            "varayetAm",
            "varayeta",
            "varjayatAm",
            "varjayetAm",
            "varjayeta",
            "vartayatAm",
            "vartayetAm",
            "vartayeta",
            "vasayatAm",
            "vasayetAm",
            "vasayeta",
            "vawayatAm",
            "vawayetAm",
            "vawayeta",
            "velayatAm",
            "velayetAm",
            "velayeta",
            "veviktAm",
            "vevizwAm",
            "viNktAm",
            "vicCayatAm",
            "vicCayetAm",
            "vicCayeta",
            "vizkayatAm",
            "vizkayetAm",
            "vizkayeta",
            "vraRayatAm",
            "vraRayetAm",
            "vraRayeta",
            "vyayayatAm",
            "vyayayetAm",
            "vyayayeta",
            "waNkayatAm",
            "waNkayetAm",
            "waNkayeta",
            "yamayatAm",
            "yamayetAm",
            "yamayeta",
            "yantrayatAm",
            "yantrayetAm",
            "yantrayeta",
            "yojayatAm",
            "yojayetAm",
            "yojayeta",
            "yuNktAm",
        ]
    );
}

/// Slice 3f2's row through `check`. `Bas` is unique among the curated roots,
/// and none of these surfaces is a prior row's, so every analysis must name
/// √bhas and carry the rule that shaped the form.
#[test]
fn bhas_analyses_its_reduplicated_forms() {
    let engine = Panini::new();
    for (form, sutra) in [
        ("bapsati", "6.4.100"),
        ("babDaH", "8.2.26"),
        ("abaBat", "8.2.73"),
        ("abaBaH", "8.2.74"),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert!(!r.analyses.is_empty(), "{form}");
        for a in &r.analyses {
            assert_eq!(a.dhatu, "Bas", "{form}");
            assert!(
                a.trace.iter().any(|s| s.sutra == sutra),
                "{form}: {:?}",
                a.trace
            );
        }
    }
}

/// Slice 3f3's row through `check`. `jan` is unique among the curated roots,
/// and none of these surfaces is a prior row's, so every analysis must name
/// √jan and carry the rule that shaped the form. `jajAyAt` is an alternate:
/// `check` reaches it through 6.4.43's branch.
#[test]
fn jan_analyses_its_reduplicated_forms() {
    let engine = Panini::new();
    for (form, sutra) in [
        ("jajYati", "6.4.98"),
        ("ajajYuH", "8.4.40"),
        ("jajAtaH", "6.4.42"),
        ("jajAyAt", "6.4.43"),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert!(!r.analyses.is_empty(), "{form}");
        for a in &r.analyses {
            assert_eq!(a.dhatu, "jan", "{form}");
            assert!(
                a.trace.iter().any(|s| s.sutra == sutra),
                "{form}: {:?}",
                a.trace
            );
        }
    }
}

/// Slice 10a's rows through `check`. None of these surfaces is a prior row's,
/// so every analysis must name the curādi root and carry the sanādi rule that
/// shaped its stem — and `corayate`, ātmanepada only, must carry 1.3.74.
#[test]
fn curadi_analyses_its_nijanta_forms() {
    let engine = Panini::new();
    for (form, dhatu, sutra) in [
        ("corayati", "cur", "7.3.86"),
        ("corayate", "cur", "1.3.74"),
        ("lAqayati", "laq", "7.2.116"),
        ("BakzayARi", "Bakz", "3.1.32"),
        ("aBUzayat", "BUz", "3.1.25"),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert!(!r.analyses.is_empty(), "{form}");
        for a in &r.analyses {
            assert_eq!(a.dhatu, dhatu, "{form}");
            assert!(
                a.trace.iter().any(|s| s.sutra == sutra),
                "{form}: {:?}",
                a.trace
            );
        }
    }
}

/// Slice 10b's ākusmīya rows through `check`. None of these surfaces is a
/// prior row's, so every analysis must name the ākusmīya root, be
/// ātmanepada, and credit the gaṇasūtra 10.0496 with no pada sūtra beside
/// it. The parasmaipada shapes (`cetayati` …) derive nothing: 10.0496 blocks
/// every parasmaipada cell, and `check` must not report a blocked branch.
#[test]
fn curadi_analyses_its_akusmiya_forms() {
    let engine = Panini::new();
    for (form, dhatu) in [
        ("cetayate", "cit"),
        ("avarzayata", "vfz"),
        ("mAdayaDvam", "mad"),
        ("kusmayeta", "kusm"),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert!(!r.analyses.is_empty(), "{form}");
        for a in &r.analyses {
            assert_eq!(a.dhatu, dhatu, "{form}");
            assert_eq!(a.pada, Pada::Atmanepada, "{form}");
            let ids: Vec<&str> = a.trace.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids[0], "10.0496", "{form}: {ids:?}");
            for absent in ["1.3.12", "1.3.66", "1.3.72", "1.3.74", "1.3.78"] {
                assert!(!ids.contains(&absent), "{form} {absent}: {ids:?}");
            }
        }
    }
    for form in ["cetayati", "varzayati", "amAdayat", "kusmayatu"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}

/// Slice 10c's ākusmīya rows through `check`: one witness per pre-ṇic shape
/// and per homograph row. The goldens were grepped first — each surface below
/// is its own row's alone, so each must yield exactly one analysis, naming
/// that root, ātmanepada, opening with 10.0496 and crediting no pada sūtra.
/// `mAnayate` and `amAnayata` are the one pair two ākusmīya rows share
/// (`10.0233 mAna~` unchanged before ṇic, `10.0234 mana~` by 7.2.116): one
/// analysis per root opening with 10.0496, and only √man's credits 7.2.116.
/// Slice 10h's ādhṛṣīya `10.0381 mAna~` shares them too, by 1.3.74 on its ṇic
/// branch — and `amAnayata` twice over, its parasmaipada madhyama bahu
/// as well — so `mAnayate` has three analyses and `amAnayata` four. The
/// parasmaipada shapes derive nothing. The single-analysis and Invalid
/// assertions depend on the homograph partners being uncurated — `10.0189`,
/// `10.0438`, `10.0041`, `10.0006`, `10.0034` — so a slice that curates one
/// must revisit this test (e.g. `kuwwayatu` becomes Valid once `10.0034`
/// is). `mAnayati` became Valid when slice 10h curated `10.0381`.
#[test]
fn curadi_analyses_its_bulk_akusmiya_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    for (form, dhatu) in [
        ("dAsayate", "das"),
        ("aqepayata", "qip"),
        ("spASayatAm", "spaS"),
        ("tarjayeta", "tarj"),
        ("trowayaDve", "truw"),
        ("vedayate", "vid"),
        ("kURayasva", "kUR"),
        ("SAWayate", "SaW"),
        ("SAmayeran", "Sam"),
        ("syAmayate", "syam"),
        ("alakzayanta", "lakz"),
        ("kuwwayate", "kuww"),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), 1, "{form}");
        let a = &r.analyses[0];
        assert_eq!(a.dhatu, dhatu, "{form}");
        assert_eq!(a.pada, Pada::Atmanepada, "{form}");
        let ids = ids_of(a);
        assert_eq!(ids[0], "10.0496", "{form}: {ids:?}");
        for absent in ["1.3.12", "1.3.66", "1.3.72", "1.3.74", "1.3.78"] {
            assert!(!ids.iter().any(|i| i == absent), "{form} {absent}: {ids:?}");
        }
    }
    // (form, `10.0381 mAna~`'s padas for it)
    for (form, adhrsiya_padas) in [
        ("mAnayate", &[Pada::Atmanepada][..]),
        ("amAnayata", &[Pada::Atmanepada, Pada::Parasmaipada][..]),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), 2 + adhrsiya_padas.len(), "{form}");
        let mut akusmiya: Vec<&str> = Vec::new();
        let mut adhrsiya: Vec<Pada> = Vec::new();
        for a in &r.analyses {
            let ids = ids_of(a);
            if ids[0] == "10.0496" {
                assert_eq!(a.pada, Pada::Atmanepada, "{form}");
                let lengthened = ids.iter().any(|i| i == "7.2.116");
                assert_eq!(lengthened, a.dhatu == "man", "{form} {}: {ids:?}", a.dhatu);
                akusmiya.push(a.dhatu.as_str());
            } else {
                // `10.0381 mAna~`'s ṇic branch: 1.3.74 in ātmanepada, 1.3.78
                // in parasmaipada.
                assert_eq!(a.dhatu, "mAn", "{form}");
                assert!(ids.iter().any(|i| i == "3.1.25"), "{form}: {ids:?}");
                let sanction = match a.pada {
                    Pada::Atmanepada => "1.3.74",
                    Pada::Parasmaipada => "1.3.78",
                };
                assert!(ids.iter().any(|i| i == sanction), "{form}: {ids:?}");
                adhrsiya.push(a.pada);
            }
        }
        akusmiya.sort_unstable();
        assert_eq!(akusmiya, ["mAn", "man"], "{form}");
        assert_eq!(adhrsiya.len(), adhrsiya_padas.len(), "{form}");
        for pada in adhrsiya_padas {
            assert!(adhrsiya.contains(pada), "{form} {pada:?}");
        }
    }
    for form in ["dAsayati", "vedayati", "kuwwayatu"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}

/// Slice 10d's `check()` witnesses: all six jñapādi rows, both padas. The
/// goldens were grepped first — each laṭ prathama eka surface below is its
/// own row's alone, so each must yield exactly one analysis, naming that
/// root and pada and crediting 10.0493 first, then 7.2.116, then
/// 6.4.92 — except that since slice 10e √cah's and √rah's surfaces are
/// shared with the adanta homographs `10.0405 caha` and `10.0396 raha`
/// (see `curadi_analyses_its_adanta_forms`), and since slice 10j √cap's with
/// √ci's 6.1.54 branch (`10.0124`, see `curadi_analyses_its_ajanta_forms`):
/// two analyses, of which the √cap one, 7.2.116's, is checked here.
/// `ajYapayata` is pada-ambiguous within √jñap (laṅ parasmaipada
/// madhyama bahu = ātmanepada prathama eka): two analyses, one per pada, both
/// mit. The 7.2.116-only shapes (`yAmayati`, …) — what this engine derived
/// before 6.4.92 — derive nothing; √jñap's, `jYApayati`, is √jñā's
/// (`10.0258`, 7.3.36) since slice 10j.
#[test]
fn curadi_analyses_its_jnapadi_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    let assert_mit = |form: &str, ids: &[String]| {
        assert_eq!(ids[0], "10.0493", "{form}: {ids:?}");
        let pos = |id: &str| ids.iter().position(|i| i == id);
        let (lengthen, shorten) = (pos("7.2.116"), pos("6.4.92"));
        assert!(
            lengthen.is_some() && shorten.is_some() && lengthen < shorten,
            "{form}: {ids:?}"
        );
    };
    for (dhatu, parasmai, atmane) in [
        ("jYap", "jYapayati", "jYapayate"),
        ("yam", "yamayati", "yamayate"),
        ("cah", "cahayati", "cahayate"),
        ("cap", "capayati", "capayate"),
        ("rah", "rahayati", "rahayate"),
        ("bal", "balayati", "balayate"),
    ] {
        for (form, pada) in [(parasmai, Pada::Parasmaipada), (atmane, Pada::Atmanepada)] {
            let r = engine.check(form);
            assert!(matches!(r.verdict, Verdict::Valid), "{form}");
            let homograph = matches!(dhatu, "cah" | "rah" | "cap");
            assert_eq!(r.analyses.len(), if homograph { 2 } else { 1 }, "{form}");
            let a = r.analyses.iter().find(|a| a.dhatu == dhatu).unwrap();
            assert_eq!(a.pada, pada, "{form}");
            assert_mit(form, &ids_of(a));
        }
    }
    let r = engine.check("ajYapayata");
    assert!(matches!(r.verdict, Verdict::Valid));
    let mut padas: Vec<Pada> = r.analyses.iter().map(|a| a.pada).collect();
    padas.sort_by_key(|p| *p == Pada::Atmanepada);
    assert_eq!(padas, [Pada::Parasmaipada, Pada::Atmanepada]);
    for a in &r.analyses {
        assert_eq!(a.dhatu, "jYap");
        assert_mit("ajYapayata", &ids_of(a));
    }
    for form in ["yAmayati", "cAhayati", "cApayati", "rAhayati", "bAlayati"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}

/// Slice 10e's `check()` witnesses, one per shape class of the spec's
/// enumeration plus every homograph pair. Each adanta analysis credits
/// 6.4.48 and neither 7.2.116 nor 7.3.86 (1.1.57's block); each ā-garvīya
/// one opens with 10.0497 and credits no pada sūtra. The goldens were
/// grepped for every witness first: the single-analysis forms are their own
/// row's alone, and each homograph surface is exactly its two rows'.
/// `mArgayARi`, the adanta 8.4.2 witness, became a pair in slice 10h: the
/// ādhṛṣīya `10.0384 mArga~` is `mArg` before ṇic, the adanta `mArga` after
/// 6.4.48.
/// The shapes the block prevents, and the anusvāra rows' pre-8.3.24 shape,
/// derive nothing.
#[test]
fn curadi_analyses_its_adanta_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    let has = |ids: &[String], id: &str| ids.iter().any(|i| i == id);
    let assert_adanta = |form: &str, ids: &[String]| {
        assert!(has(ids, "6.4.48"), "{form}: {ids:?}");
        assert!(!has(ids, "7.2.116"), "{form}: {ids:?}");
        assert!(!has(ids, "7.3.86"), "{form}: {ids:?}");
    };
    // (form, root, pada, an id the class adds beyond 6.4.48)
    for (form, dhatu, pada, extra) in [
        ("kaTayati", "kaTa", Pada::Parasmaipada, None),
        ("gaRayati", "gaRa", Pada::Parasmaipada, None),
        ("guRayati", "guRa", Pada::Parasmaipada, None),
        ("kuRayate", "kuRa", Pada::Atmanepada, None),
        ("Onayan", "Una", Pada::Parasmaipada, Some("6.4.72")),
        ("acCidrayan", "Cidra", Pada::Parasmaipada, Some("6.1.73")),
        ("saNketayati", "sanketa", Pada::Parasmaipada, Some("8.4.58")),
        ("daRqayate", "danqa", Pada::Atmanepada, Some("8.4.58")),
        ("aMsayati", "ansa", Pada::Parasmaipada, Some("8.3.24")),
        ("padayate", "pada", Pada::Atmanepada, Some("10.0497")),
        ("gfhayate", "gfha", Pada::Atmanepada, Some("10.0497")),
        ("kuhayate", "kuha", Pada::Atmanepada, Some("10.0497")),
        ("ArTayata", "arTa", Pada::Atmanepada, Some("10.0497")),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), 1, "{form}");
        let a = &r.analyses[0];
        assert_eq!(a.dhatu, dhatu, "{form}");
        assert_eq!(a.pada, pada, "{form}");
        let ids = ids_of(a);
        assert_adanta(form, &ids);
        if let Some(id) = extra {
            assert!(has(&ids, id), "{form} {id}: {ids:?}");
        }
        if extra == Some("10.0497") {
            assert_eq!(ids[0], "10.0497", "{form}: {ids:?}");
            for absent in ["1.3.12", "1.3.66", "1.3.72", "1.3.74", "1.3.78"] {
                assert!(!has(&ids, absent), "{form} {absent}: {ids:?}");
            }
        }
    }
    // The homograph pairs: the adanta row and its curated twin.
    for (form, adanta, twin, twin_id) in [
        ("rahayati", "raha", "rah", "6.4.92"),
        ("cahayati", "caha", "cah", "6.4.92"),
        ("kUwayate", "kUwa", "kUw", "10.0496"),
        ("vizkayate", "vizka", "vizk", "10.0496"),
        ("mArgayARi", "mArga", "mArg", "8.4.2"),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        let mut roots: Vec<&str> = r.analyses.iter().map(|a| a.dhatu.as_str()).collect();
        roots.sort_unstable();
        let mut want = [adanta, twin];
        want.sort_unstable();
        assert_eq!(roots, want, "{form}");
        for a in &r.analyses {
            let ids = ids_of(a);
            if a.dhatu == adanta {
                assert_adanta(form, &ids);
            } else {
                assert!(has(&ids, twin_id), "{form} {twin}: {ids:?}");
                assert!(!has(&ids, "6.4.48"), "{form} {twin}: {ids:?}");
            }
        }
    }
    for form in [
        "kATayati",
        "gARayati",
        "kohayate",
        "goRayati",
        "sanketayati",
        "padayati",
        "kuhayati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}

/// Slices 10f's and 10g's `check()` witnesses: 10f's one per row class ×
/// pada of its spec's Forms table, and 10g's one ṇic-less and one ṇic
/// witness per new code shape (num before `w W`, `p b B` and `h`, the
/// stripped `o~`, √kṣamp's 8.4.2, and the four udit shapes), plus every
/// homograph pair its spec names. A ṇic-less analysis opens with its
/// optional-ṇic id, credits 1.3.78 and never 3.1.25; a ṇic one credits
/// 3.1.25 and never an optional-ṇic id but 2573.2. Since slice 10i a
/// homograph's analyses may open with different ids, one per row: `daMSati`
/// is `10.0193`'s 2564 and the āsvadīya `10.0295`'s 10.0499. The goldens were grepped for every witness first:
/// a single-root witness is its own row's alone, and a homograph witness is
/// exactly its rows', one analysis each (the `cah` / `rah` precedent). The
/// shapes the slices rule out — ṇic in an ākusmīya or ā-garvīya root's
/// parasmaipada, no ṇic in any ātmanepada, the unmerged aṅga–śap junction,
/// and the dental `n` 8.4.2 retroflexes across num — derive nothing.
#[test]
fn curadi_analyses_its_optional_nic_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    let has = |ids: &[String], id: &str| ids.iter().any(|i| i == id);
    // (form, its roots, pada, the optional-ṇic ids its analyses open with,
    // one per analysis, or none for ṇic)
    for (form, dhatus, pada, triggers) in [
        // Since slice 10i the āsvadīya `10.0295` is `danS` too: its
        // ṇic-less analysis opens with 10.0499.
        (
            "daMSati",
            &["danS", "danS"][..],
            Pada::Parasmaipada,
            &["2564", "10.0499"][..],
        ),
        ("daMSayate", &["danS", "danS"], Pada::Atmanepada, &[]),
        ("vaYcati", &["vanc"], Pada::Parasmaipada, &["2570"]),
        // `10.0230` (10f, ākusmīya) and `10.0249` (10g, `Nic`) share these;
        // only `10.0249` derives the ṇic parasmaipada.
        (
            "devati",
            &["div", "div"],
            Pada::Parasmaipada,
            &["2570", "2570"],
        ),
        ("devayate", &["div", "div"], Pada::Atmanepada, &[]),
        ("devayati", &["div"], Pada::Parasmaipada, &[]),
        ("garvati", &["garva"], Pada::Parasmaipada, &["2573.3"]),
        ("garvayate", &["garva"], Pada::Atmanepada, &[]),
        ("mUtrati", &["mUtra"], Pada::Parasmaipada, &["2573.3"]),
        ("mUtrayati", &["mUtra"], Pada::Parasmaipada, &[]),
        ("mUtrayate", &["mUtra"], Pada::Atmanepada, &[]),
        ("katrAmi", &["katra"], Pada::Parasmaipada, &["2573.3"]),
        ("patati", &["pata"], Pada::Parasmaipada, &["2573.1"]),
        ("patayati", &["pata"], Pada::Parasmaipada, &[]),
        ("pAtayati", &["pata"], Pada::Parasmaipada, &[]),
        ("patayate", &["pata"], Pada::Atmanepada, &[]),
        ("pAtayate", &["pata"], Pada::Atmanepada, &[]),
        // Slice 10g: one witness pair per new code shape.
        ("cintati", &["cint"], Pada::Parasmaipada, &["2564"]),
        ("cintayate", &["cint"], Pada::Atmanepada, &[]),
        ("sPuRwati", &["sPunw"], Pada::Parasmaipada, &["2564"]),
        ("sPuRwayate", &["sPunw"], Pada::Atmanepada, &[]),
        ("campati", &["canp"], Pada::Parasmaipada, &["2564"]),
        ("campayate", &["canp"], Pada::Atmanepada, &[]),
        ("daMhati", &["danh"], Pada::Parasmaipada, &["2564"]),
        ("daMhayate", &["danh"], Pada::Atmanepada, &[]),
        // `lanq`, too, is the āsvadīya `10.0331` since slice 10i.
        (
            "laRqati",
            &["lanq", "lanq"],
            Pada::Parasmaipada,
            &["2564", "10.0499"],
        ),
        ("laRqayate", &["lanq", "lanq"], Pada::Atmanepada, &[]),
        ("kzampARi", &["kzanp"], Pada::Parasmaipada, &["2564"]),
        ("kzampayARi", &["kzanp"], Pada::Parasmaipada, &[]),
        ("SraRati", &["SraR"], Pada::Parasmaipada, &["2570"]),
        ("SrARayate", &["SraR"], Pada::Atmanepada, &[]),
        ("SarDati", &["SfD"], Pada::Parasmaipada, &["2570"]),
        ("SarDayate", &["SfD"], Pada::Atmanepada, &[]),
        ("aYcati", &["anc"], Pada::Parasmaipada, &["2570"]),
        ("aYcayate", &["anc"], Pada::Atmanepada, &[]),
        // Slice 10g's homographs: two rows each, and a third `lanj`, the
        // āsvadīya `10.0315`, since slice 10i.
        (
            "laYjati",
            &["lanj", "lanj", "lanj"],
            Pada::Parasmaipada,
            &["2564", "2564", "10.0499"],
        ),
        (
            "vaRwati",
            &["vanw", "vanw"],
            Pada::Parasmaipada,
            &["2564", "2564"],
        ),
        (
            "jasati",
            &["jas", "jas"],
            Pada::Parasmaipada,
            &["2570", "2570"],
        ),
        // āṭ's vṛddhi gives `O` for both `o` and `u`: laṅ meets, laṭ does not.
        (
            "OlaRqad",
            &["olanq", "ulanq"],
            Pada::Parasmaipada,
            &["2564", "2564"],
        ),
        ("olaRqati", &["olanq"], Pada::Parasmaipada, &["2564"]),
        ("ulaRqati", &["ulanq"], Pada::Parasmaipada, &["2564"]),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), dhatus.len(), "{form}");
        let mut got: Vec<&str> = r.analyses.iter().map(|a| a.dhatu.as_str()).collect();
        got.sort_unstable();
        let mut want = dhatus.to_vec();
        want.sort_unstable();
        assert_eq!(got, want, "{form}");
        for a in &r.analyses {
            assert_eq!(a.pada, pada, "{form}");
            let ids = ids_of(a);
            if triggers.is_empty() {
                assert!(has(&ids, "3.1.25"), "{form}: {ids:?}");
                for id in [
                    "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3",
                ] {
                    assert!(!has(&ids, id), "{form} {id}: {ids:?}");
                }
                assert_eq!(
                    has(&ids, "2573.2"),
                    form.starts_with("pA"),
                    "{form}: {ids:?}"
                );
            } else {
                assert!(has(&ids, "1.3.78"), "{form}: {ids:?}");
                assert!(!has(&ids, "3.1.25"), "{form}: {ids:?}");
            }
        }
        // A ṇic-less witness's analyses open with its ids, one apiece.
        if !triggers.is_empty() {
            let mut opened: Vec<String> = r.analyses.iter().map(|a| ids_of(a)[0].clone()).collect();
            opened.sort_unstable();
            let mut want = triggers.to_vec();
            want.sort_unstable();
            assert_eq!(opened, want, "{form}");
        }
    }
    for form in [
        "tantrayati",
        "garvayati",
        "daMSate",
        "garvate",
        "mUtrate",
        "patate",
        "katraati",
        "pataAmi",
        // Slice 10g: no ṇic in ātmanepada, and 8.4.2 across num.
        "cintate",
        "SarDate",
        "kzampAni",
        "kzampayAni",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}

/// Slice 10h's `check()` witnesses, from its spec's tables: one ṇic-less and
/// one ṇic witness per pada assignment and per new rule, plus every homograph
/// the spec names. A ṇic-less analysis opens with 10.0498 and credits no
/// 3.1.25, and its pada sūtra is 1.3.78 — or 1.3.72, in a `NicUbhayapada`
/// row's ātmanepada; a ṇic one credits 3.1.25 and 1.3.74 or 1.3.78, and no
/// 10.0498. The goldens were grepped for every witness first: a single-root
/// witness is its own row's alone, and a homograph witness is exactly its
/// rows', one analysis each (the `cah` / `rah` precedent). `Bavati` and
/// `vadati` are bhvādi's too, and the bhvādi analysis comes first, so
/// `trace_for` keeps answering with it; that order is pinned here. The
/// shapes the slice rules out — no ṇic in a `Nic` row's ātmanepada, a ṇic
/// branch without its vṛddhi, √mṛj's guṇa, and nuk without ṇic — derive
/// nothing.
#[test]
fn curadi_analyses_its_adhrsiya_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    let has = |ids: &[String], id: &str| ids.iter().any(|i| i == id);
    // (form, its ādhṛṣīya roots, pada, ṇic-less?, the ids it must credit)
    for (form, dhatus, pada, nicless, credits) in [
        (
            "yojati",
            &["yuj"][..],
            Pada::Parasmaipada,
            true,
            &["1.3.78"][..],
        ),
        ("yojayate", &["yuj"], Pada::Atmanepada, false, &["1.3.74"]),
        ("layati", &["lI"], Pada::Parasmaipada, true, &["1.3.78"]),
        (
            "lAyayati",
            &["lI"],
            Pada::Parasmaipada,
            false,
            &["7.2.115", "6.1.78"],
        ),
        ("varate", &["vf"], Pada::Atmanepada, true, &["1.3.72"]),
        (
            "vArayate",
            &["vf"],
            Pada::Atmanepada,
            false,
            &["7.2.115", "1.3.74"],
        ),
        (
            "DUnayati",
            &["DU"],
            Pada::Parasmaipada,
            false,
            &["7.3.37.2"],
        ),
        (
            "DAvayati",
            &["DU"],
            Pada::Parasmaipada,
            false,
            &["7.2.115", "6.1.78"],
        ),
        ("Davate", &["DU"], Pada::Atmanepada, true, &["1.3.72"]),
        (
            "prIRayate",
            &["prI"],
            Pada::Atmanepada,
            false,
            &["7.3.37.2", "1.3.74"],
        ),
        ("mArjati", &["mfj"], Pada::Parasmaipada, true, &["7.2.114"]),
        (
            "mArjayati",
            &["mfj"],
            Pada::Parasmaipada,
            false,
            &["7.2.114"],
        ),
        ("hiMsati", &["hins"], Pada::Parasmaipada, true, &["8.3.24"]),
        ("kaRWati", &["kanW"], Pada::Parasmaipada, true, &["8.4.58"]),
        ("mAnayati", &["mAn"], Pada::Parasmaipada, false, &["1.3.78"]),
        // Homographs inside the slice: two rows each.
        (
            "tarpati",
            &["tfp", "tfp"],
            Pada::Parasmaipada,
            true,
            &["7.3.86"],
        ),
        (
            "darBati",
            &["dfB", "dfB"],
            Pada::Parasmaipada,
            true,
            &["7.3.86"],
        ),
        (
            "granTati",
            &["granT", "granT"],
            Pada::Parasmaipada,
            true,
            &["8.3.24"],
        ),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), dhatus.len(), "{form}");
        for a in &r.analyses {
            assert!(dhatus.contains(&a.dhatu.as_str()), "{form}: {}", a.dhatu);
            assert_eq!(a.pada, pada, "{form}");
            let ids = ids_of(a);
            if nicless {
                assert_eq!(ids[0], "10.0498", "{form}: {ids:?}");
                assert!(!has(&ids, "3.1.25"), "{form}: {ids:?}");
            } else {
                assert!(has(&ids, "3.1.25"), "{form}: {ids:?}");
                assert!(!has(&ids, "10.0498"), "{form}: {ids:?}");
            }
            for id in credits {
                assert!(has(&ids, id), "{form} {id}: {ids:?}");
            }
        }
    }
    // √mṛj's vṛddhi replaces guṇa on both branches; √dhū's nuk replaces vṛddhi.
    for form in ["mArjati", "mArjayati"] {
        let ids = ids_of(&engine.check(form).analyses[0]);
        assert!(!has(&ids, "7.3.86"), "{form}: {ids:?}");
    }
    let ids = ids_of(&engine.check("DUnayati").analyses[0]);
    assert!(!has(&ids, "7.2.115"), "DUnayati: {ids:?}");
    // Homographs with bhvādi: the bhvādi row's analysis first, then the
    // ādhṛṣīya row's ṇic-less one.
    for form in ["Bavati", "vadati"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), 2, "{form}");
        let first = ids_of(&r.analyses[0]);
        let second = ids_of(&r.analyses[1]);
        assert!(!has(&first, "10.0498"), "{form}: {first:?}");
        assert_eq!(second[0], "10.0498", "{form}: {second:?}");
        assert_eq!(r.analyses[0].dhatu, r.analyses[1].dhatu, "{form}");
    }
    for form in ["yojate", "layayati", "marjati", "marjayati", "DUnati"] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}

/// Slice 10i's `check()` witnesses, from its spec's tables: a ṇic-less and a
/// ṇic witness per optional-ṇic id (10.0499, 2565, 2571), witnesses for each
/// new rule (3.1.28's āya, the sanādi 6.1.73) and each vowel-final row, and
/// every homograph the spec names. A ṇic-less analysis opens with its id,
/// credits 1.3.78 and no 3.1.25; a ṇic one credits 3.1.25 and none of the
/// three ids, and never 3.1.28. The goldens were grepped for every witness
/// first: a single-root witness is its own row's alone, and a homograph
/// witness is exactly its rows', one analysis each. Where the other row
/// came first in the table (`pArayati`'s adanta `10.0453 pAra`, and the
/// other gaṇas' `jayati`, `BaYjanti`, `aYjanti`, `vartatAm`), its analysis
/// comes first, so `trace_for` keeps answering with it; that order is pinned
/// here. The shapes the slice rules out — no ṇic in a `Nic` row's
/// ātmanepada, a ṇic-less √dhūp or √vich without āya, āya where ātmanepada
/// blocks it, guṇa before √vich's tuk, and guṇa where 7.2.115 gives vṛddhi
/// — derive nothing.
#[test]
fn curadi_analyses_its_asvadiya_forms() {
    let engine = Panini::new();
    let ids_of =
        |a: &panini::Analysis| -> Vec<String> { a.trace.iter().map(|s| s.sutra.clone()).collect() };
    let has = |ids: &[String], id: &str| ids.iter().any(|i| i == id);
    // (form, its curādi roots, pada, the optional-ṇic ids its analyses open
    // with, one per analysis, or none for ṇic, the ids every analysis credits)
    for (form, dhatus, pada, opens, credits) in [
        (
            "grasati",
            &["gras"][..],
            Pada::Parasmaipada,
            &["10.0499"][..],
            &["1.3.78"][..],
        ),
        (
            "grAsayati",
            &["gras"],
            Pada::Parasmaipada,
            &[],
            &["7.2.116"],
        ),
        ("grAsayate", &["gras"], Pada::Atmanepada, &[], &["1.3.74"]),
        ("parati", &["pF"], Pada::Parasmaipada, &["2565"], &[]),
        (
            "Gozati",
            &["Guz"],
            Pada::Parasmaipada,
            &["2571"],
            &["7.3.86"],
        ),
        ("Gozayati", &["Guz"], Pada::Parasmaipada, &[], &["7.3.86"]),
        (
            "DUpAyati",
            &["DUp"],
            Pada::Parasmaipada,
            &["10.0499"],
            &["3.1.28", "3.4.114", "3.1.32"],
        ),
        ("DUpayati", &["DUp"], Pada::Parasmaipada, &[], &[]),
        (
            "vicCAyati",
            &["viC"],
            Pada::Parasmaipada,
            &["10.0499"],
            &["3.1.28", "6.1.73", "8.4.40"],
        ),
        (
            "vicCayati",
            &["viC"],
            Pada::Parasmaipada,
            &[],
            &["6.1.73", "8.4.40"],
        ),
        (
            "jAyayati",
            &["ji"],
            Pada::Parasmaipada,
            &[],
            &["7.2.115", "6.1.78"],
        ),
        (
            "cAyayati",
            &["ci"],
            Pada::Parasmaipada,
            &[],
            &["7.2.115", "6.1.78"],
        ),
        // √ci (`10.0124`, slice 10j) shares the ṇic-less `cayati`, 2570's.
        (
            "cayati",
            &["ci", "ci"],
            Pada::Parasmaipada,
            &["10.0499", "2570"],
            &[],
        ),
        // The īdit `pUrI~` and the udit `vftu~` are 10.0499's, not 2572's
        // or 2570's.
        ("pUrati", &["pUr"], Pada::Parasmaipada, &["10.0499"], &[]),
        (
            "vartati",
            &["vft"],
            Pada::Parasmaipada,
            &["10.0499"],
            &["7.3.86"],
        ),
        // Homographs: a curādi row of the same code before this slice, or
        // two in it.
        (
            "daMsati",
            &["dans", "dans"],
            Pada::Parasmaipada,
            &["2564", "10.0499"],
            &["8.3.24"],
        ),
        (
            "tuYjati",
            &["tunj", "tunj"],
            Pada::Parasmaipada,
            &["2564", "10.0499"],
            &["8.3.24"],
        ),
        (
            "piYjati",
            &["pinj", "pinj"],
            Pada::Parasmaipada,
            &["2564", "10.0499"],
            &["8.3.24"],
        ),
        (
            "luYjati",
            &["lunj", "lunj"],
            Pada::Parasmaipada,
            &["2564", "10.0499"],
            &["8.3.24"],
        ),
        (
            "SIkati",
            &["SIk", "SIk"],
            Pada::Parasmaipada,
            &["10.0498", "10.0499"],
            &[],
        ),
        ("SIkayati", &["SIk", "SIk"], Pada::Parasmaipada, &[], &[]),
        (
            "laNGati",
            &["lanG", "lanG"],
            Pada::Parasmaipada,
            &["10.0499", "10.0499"],
            &["8.3.24"],
        ),
        (
            "laNGayate",
            &["lanG", "lanG"],
            Pada::Atmanepada,
            &[],
            &["1.3.74"],
        ),
        ("svAdayati", &["svad", "svAd"], Pada::Parasmaipada, &[], &[]),
        ("pArayati", &["pAra", "pF"], Pada::Parasmaipada, &[], &[]),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        let mut got: Vec<&str> = r.analyses.iter().map(|a| a.dhatu.as_str()).collect();
        got.sort_unstable();
        let mut want = dhatus.to_vec();
        want.sort_unstable();
        assert_eq!(got, want, "{form}");
        for a in &r.analyses {
            assert_eq!(a.pada, pada, "{form}");
            let ids = ids_of(a);
            if opens.is_empty() {
                assert!(has(&ids, "3.1.25"), "{form}: {ids:?}");
                for id in ["10.0499", "2565", "2571", "3.1.28"] {
                    assert!(!has(&ids, id), "{form} {id}: {ids:?}");
                }
            } else {
                assert!(has(&ids, "1.3.78"), "{form}: {ids:?}");
                assert!(!has(&ids, "3.1.25"), "{form}: {ids:?}");
            }
            for id in credits {
                assert!(has(&ids, id), "{form} {id}: {ids:?}");
            }
        }
        if !opens.is_empty() {
            // The id each analysis opens with, after 10.0493 on a mit row.
            let mut opened: Vec<String> = r
                .analyses
                .iter()
                .map(|a| ids_of(a).into_iter().find(|i| i != "10.0493").unwrap())
                .collect();
            opened.sort_unstable();
            let mut want = opens.to_vec();
            want.sort_unstable();
            assert_eq!(opened, want, "{form}");
        }
    }
    // √vich's tuk leaves guṇa nothing to read on either branch.
    for form in ["vicCAyati", "vicCayati"] {
        let ids = ids_of(&engine.check(form).analyses[0]);
        assert!(!has(&ids, "7.3.86"), "{form}: {ids:?}");
    }
    // `pArayati`: the adanta `pAra`'s analysis first (6.4.48), then √pṝ's
    // (7.2.115).
    let r = engine.check("pArayati");
    assert_eq!(r.analyses[0].dhatu, "pAra");
    assert!(has(&ids_of(&r.analyses[0]), "6.4.48"));
    assert_eq!(r.analyses[1].dhatu, "pF");
    assert!(has(&ids_of(&r.analyses[1]), "7.2.115"));
    // Homographs with other gaṇas: that gaṇa's row's analysis first, then
    // the āsvadīya row's ṇic-less one.
    for (form, dhatu, first_pada) in [
        ("jayati", "ji", Pada::Parasmaipada),
        ("BaYjanti", "Banj", Pada::Parasmaipada),
        ("aYjanti", "anj", Pada::Parasmaipada),
        ("vartatAm", "vft", Pada::Atmanepada),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        assert_eq!(r.analyses.len(), 2, "{form}");
        for a in &r.analyses {
            assert_eq!(a.dhatu, dhatu, "{form}");
        }
        let first = ids_of(&r.analyses[0]);
        let second = ids_of(&r.analyses[1]);
        assert_eq!(r.analyses[0].pada, first_pada, "{form}");
        assert!(!has(&first, "10.0499"), "{form}: {first:?}");
        assert_eq!(r.analyses[1].pada, Pada::Parasmaipada, "{form}");
        assert_eq!(second[0], "10.0499", "{form}: {second:?}");
    }
    for form in [
        "grasate", "parate", "Gozate", "DUpati", "vicCati", "DUpAyate", "veCayati", "jayayati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}

/// Slice 10j's `check()` witnesses, from its spec's tables: each row's laṭ
/// prathama eka in every pada it admits, √ci's three readings in each, the
/// parasmaipada `smi`, `gf` and `yu` block, and every homograph the spec
/// names. The goldens were grepped for every witness first: a single-root
/// witness is its own row's alone, and a homograph witness is exactly its
/// rows', one analysis each. The other row came first in the table
/// (`capayati`'s √cap, `cayati`'s āsvadīya `10.0325 ci`, `BAvayati`'s
/// ādhṛṣīya `10.0382 BU`), so its analysis comes first and `trace_for` keeps
/// answering with it; that order is pinned here, and the slice's row's
/// analysis, the last, must credit the listed ids in order. The shapes the
/// slice rules out derive nothing.
#[test]
fn curadi_analyses_its_ajanta_forms() {
    let engine = Panini::new();
    // (form, its roots in analysis order, pada, the ids the slice's row's
    // analysis credits, in order)
    for (form, dhatus, pada, credits) in [
        (
            "smAyayate",
            &["smi"][..],
            Pada::Atmanepada,
            &["3.1.25", "7.2.115", "6.1.78", "3.1.32", "2567", "1.3.12"][..],
        ),
        (
            "cayayati",
            &["ci"],
            Pada::Parasmaipada,
            &["10.0493", "3.1.25", "7.2.115", "6.1.78", "6.4.92", "1.3.78"],
        ),
        (
            "cayayate",
            &["ci"],
            Pada::Atmanepada,
            &["10.0493", "3.1.25", "7.2.115", "6.1.78", "6.4.92", "1.3.74"],
        ),
        (
            "capayati",
            &["cap", "ci"],
            Pada::Parasmaipada,
            &["10.0493", "3.1.25", "6.1.54", "7.3.36", "6.4.92", "1.3.78"],
        ),
        (
            "capayate",
            &["cap", "ci"],
            Pada::Atmanepada,
            &["10.0493", "3.1.25", "6.1.54", "7.3.36", "6.4.92", "1.3.74"],
        ),
        (
            "cayati",
            &["ci", "ci"],
            Pada::Parasmaipada,
            &["10.0493", "2570", "1.3.78"],
        ),
        (
            "cayate",
            &["ci"],
            Pada::Atmanepada,
            &["10.0493", "2570", "1.3.72"],
        ),
        (
            "GArayati",
            &["Gf"],
            Pada::Parasmaipada,
            &["7.2.115", "1.3.78"],
        ),
        (
            "GArayate",
            &["Gf"],
            Pada::Atmanepada,
            &["7.2.115", "1.3.74"],
        ),
        (
            "gArayate",
            &["gf"],
            Pada::Atmanepada,
            &["10.0496", "3.1.25", "7.2.115"],
        ),
        (
            "yAvayate",
            &["yu"],
            Pada::Atmanepada,
            &["10.0496", "3.1.25", "7.2.115", "6.1.78"],
        ),
        (
            "jYApayati",
            &["jYA"],
            Pada::Parasmaipada,
            &["3.1.25", "7.3.36", "1.3.78"],
        ),
        (
            "jYApayate",
            &["jYA"],
            Pada::Atmanepada,
            &["3.1.25", "7.3.36", "1.3.74"],
        ),
        (
            "cyAvayati",
            &["cyu"],
            Pada::Parasmaipada,
            &["7.2.115", "6.1.78", "1.3.78"],
        ),
        (
            "cyAvayate",
            &["cyu"],
            Pada::Atmanepada,
            &["7.2.115", "6.1.78", "1.3.74"],
        ),
        (
            "BAvayati",
            &["BU", "BU"],
            Pada::Parasmaipada,
            &["7.2.115", "6.1.78", "1.3.78"],
        ),
        (
            "BAvayate",
            &["BU", "BU"],
            Pada::Atmanepada,
            &["7.2.115", "6.1.78", "1.3.74"],
        ),
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Valid), "{form}");
        let got: Vec<&str> = r.analyses.iter().map(|a| a.dhatu.as_str()).collect();
        assert_eq!(got, dhatus, "{form}");
        for a in &r.analyses {
            assert_eq!(a.pada, pada, "{form}");
        }
        let ids: Vec<&str> = r
            .analyses
            .last()
            .unwrap()
            .trace
            .iter()
            .map(|s| s.sutra.as_str())
            .collect();
        let mut at = 0;
        for id in credits {
            let next = ids[at..].iter().position(|i| i == id);
            assert!(next.is_some(), "{form} {id} out of order: {ids:?}");
            at += next.unwrap() + 1;
        }
        // Only √ci's 6.1.54 branch and √jñā take puk, and neither takes
        // 7.2.115 then.
        if credits.contains(&"7.3.36") {
            assert!(!ids.contains(&"7.2.115"), "{form}: {ids:?}");
        }
    }
    // The parasmaipada of an ātmanepadī root (√smiṅ by 2567 and 1.3.12, √gṛ
    // and √yu by 10.0496), guṇa where 7.2.115 gives vṛddhi, √ci's 6.1.54
    // branch without 6.4.92, and √jñā without its puk.
    for form in [
        "smAyayati",
        "gArayati",
        "yAvayati",
        "smayayate",
        "Garayati",
        "cyavayati",
        "cApayati",
        "jYAyayati",
    ] {
        let r = engine.check(form);
        assert!(matches!(r.verdict, Verdict::Invalid), "{form}");
        assert!(r.analyses.is_empty(), "{form}");
    }
}
