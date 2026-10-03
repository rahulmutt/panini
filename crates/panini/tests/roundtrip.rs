//! Every form the engine derives must be recoverable by `check()`, with
//! exactly the analyses a brute-force pass over the whole corpus gives it.
//!
//! This runs over the full cross-product in the blocking tier.
//! `panini_analyze::candidates()` answers from a corpus index, so each
//! `check()` derives only the candidates that match.

use std::collections::HashMap;

use panini::Panini;
use panini_analyze::all_candidates;
use panini_data::{Lakara, Pada, Purusha, Vacana, optional_nic};
use panini_prakriya::derive;

fn fingerprint(
    dhatu: &str,
    lakara: Lakara,
    pada: Pada,
    purusha: Purusha,
    vacana: Vacana,
) -> String {
    format!("{:?}", (dhatu, lakara, pada, purusha, vacana))
}

/// What `check()` computed on every call before the analyzer had an index:
/// the whole corpus, derived by brute force and grouped by surface form. It
/// is the specification `panini_analyze::candidates()` must meet, not a
/// shortcut for other tests, so it lives here and nowhere else.
///
/// Per surface form it holds two things. The fingerprints are not
/// deduplicated: a candidate whose vikalpa branches produce one form twice
/// contributes two fingerprints here, and two analyses to `check()`. The
/// count is of distinct candidates (one per candidate with at least one
/// unblocked branch of that form, however many branches), which is what
/// `panini_analyze::candidates()` must return. It is counted per candidate
/// rather than by distinct fingerprint because fingerprints use the root
/// `code`, which two dhātupāṭha rows can share (both √aś rows are "aS").
struct Oracle {
    fingerprints: Vec<String>,
    candidates: usize,
}

fn oracle() -> HashMap<String, Oracle> {
    let mut by_form: HashMap<String, Oracle> = HashMap::new();
    for c in all_candidates() {
        let mut forms: Vec<String> = Vec::new();
        for p in derive(c.dhatu, c.lakara, c.pada, c.purusha, c.vacana) {
            if p.blocked {
                continue;
            }
            let form = p.text();
            by_form
                .entry(form.clone())
                .or_insert_with(|| Oracle {
                    fingerprints: Vec::new(),
                    candidates: 0,
                })
                .fingerprints
                .push(fingerprint(
                    c.dhatu.code,
                    c.lakara,
                    c.pada,
                    c.purusha,
                    c.vacana,
                ));
            if !forms.contains(&form) {
                forms.push(form);
            }
        }
        for form in forms {
            by_form.get_mut(&form).expect("inserted above").candidates += 1;
        }
    }
    for v in by_form.values_mut() {
        v.fingerprints.sort();
    }
    by_form
}

#[test]
fn roundtrip() {
    let engine = Panini::new();
    let oracle = oracle();
    for c in all_candidates() {
        let (d, lakara, pada, purusha, vacana) = (c.dhatu, c.lakara, c.pada, c.purusha, c.vacana);
        let branches = engine.derive(d, lakara, pada, purusha, vacana);
        // The cross-product only ever asks for padas the root admits, so
        // every cell has a live branch, and only a root whose ṇic is
        // optional (`OPTIONAL_NIC`) blocks one: its ṇic and ṇic-less
        // branches can differ in pada (10.0496 / 10.0497 / 1.3.78). Any
        // other blocked branch would mean `padas()` and the pada-sanction
        // rules had come apart.
        let cell = format!(
            "{} {} {:?} {:?} {:?}",
            d.code,
            panini::lakara_name(lakara),
            pada,
            purusha,
            vacana
        );
        assert!(
            branches.iter().any(|p| !p.blocked),
            "{cell} derived no live branch"
        );
        for p in &branches {
            assert!(
                !p.blocked || optional_nic(d.dhatupatha).is_some(),
                "{cell} derived a blocked branch"
            );
        }
        for p in branches.iter().filter(|p| !p.blocked) {
            let form = p.text();
            let r = engine.check(&form);
            assert!(
                r.analyses
                    .iter()
                    .any(|a| a.dhatu == d.code && a.form_slp1 == form && a.lakara == lakara),
                "roundtrip failed: {} {} -> {}",
                d.code,
                panini::lakara_name(lakara),
                form
            );
            let mut from_check: Vec<String> = r
                .analyses
                .iter()
                .map(|a| fingerprint(&a.dhatu, a.lakara, a.pada, a.purusha, a.vacana))
                .collect();
            from_check.sort();
            assert_eq!(
                from_check, oracle[&form].fingerprints,
                "check() and the brute-force oracle disagree on {form}"
            );
            // check() filters by re-deriving, so an over-proposing index
            // passes everything above and only costs time. Pin precision.
            assert_eq!(
                panini_analyze::candidates(&form).len(),
                oracle[&form].candidates,
                "candidates() is not precise for {form}: it must return exactly the candidates that derive it"
            );
        }
    }
}
