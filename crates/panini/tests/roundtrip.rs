//! Every form the engine derives must be recoverable by `check()`, with
//! exactly the analyses a brute-force pass over the whole corpus gives it.
//!
//! This runs over the full cross-product in the blocking tier.
//! `panini_analyze::candidates()` answers from a corpus index, so each
//! `check()` derives only the candidates that match.

use std::collections::HashMap;

use panini::Panini;
use panini_analyze::all_candidates;
use panini_data::{Lakara, Pada, Purusha, Vacana};
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
/// Neither side is deduplicated. A candidate whose vikalpa branches produce
/// one form twice contributes two fingerprints here, and two analyses to
/// `check()`.
fn oracle() -> HashMap<String, Vec<String>> {
    let mut by_form: HashMap<String, Vec<String>> = HashMap::new();
    for c in all_candidates() {
        for p in derive(c.dhatu, c.lakara, c.pada, c.purusha, c.vacana) {
            if p.blocked {
                continue;
            }
            by_form.entry(p.text()).or_default().push(fingerprint(
                c.dhatu.code,
                c.lakara,
                c.pada,
                c.purusha,
                c.vacana,
            ));
        }
    }
    for v in by_form.values_mut() {
        v.sort();
    }
    by_form
}

#[test]
fn roundtrip() {
    let engine = Panini::new();
    let oracle = oracle();
    for c in all_candidates() {
        let (d, lakara, pada, purusha, vacana) = (c.dhatu, c.lakara, c.pada, c.purusha, c.vacana);
        for p in engine.derive(d, lakara, pada, purusha, vacana) {
            // The cross-product only ever asks for padas the root admits, so
            // nothing here should be blocked. Assert it rather than
            // filtering: a blocked branch appearing would mean `padas()` and
            // the pada-sanction rules (1.3.12 / 1.3.78) had come apart.
            assert!(
                !p.blocked,
                "{} {} {:?} {:?} {:?} derived a blocked branch",
                d.code,
                panini::lakara_name(lakara),
                pada,
                purusha,
                vacana
            );
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
                from_check, oracle[&form],
                "check() and the brute-force oracle disagree on {form}"
            );
        }
    }
}
