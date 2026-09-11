mod common;

use common::CELLS;
use panini::Panini;
use panini_data::dhatus;

#[test]
fn generate_then_check_recovers_inputs() {
    let engine = Panini::new();
    for d in dhatus() {
        for &lakara in panini_analyze::LAKARAS {
            for (pu, va) in CELLS {
                for &pada in d.pada.padas() {
                    for p in engine.derive(d, lakara, pada, pu, va) {
                        let form = p.text();
                        let r = engine.check(&form);
                        assert!(
                            r.analyses.iter().any(|a| a.dhatu == d.code
                                && a.form_slp1 == form
                                && a.lakara == lakara),
                            "roundtrip failed: {} {} -> {}",
                            d.code,
                            panini::lakara_name(lakara),
                            form
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn index_agrees_with_check_on_known_forms() {
    let engine = Panini::new();
    let index = common::index::corpus_index();
    for form in ["Bavati", "paWati", "aBavat", "juhoti", "alaBata"] {
        let mut from_check: Vec<String> = engine
            .check(form)
            .analyses
            .iter()
            .map(|a| {
                format!(
                    "{:?}",
                    (a.dhatu.as_str(), a.lakara, a.pada, a.purusha, a.vacana)
                )
            })
            .collect();
        let mut from_index: Vec<String> = index
            .analyses(form)
            .iter()
            .map(|a| format!("{:?}", (a.dhatu, a.lakara, a.pada, a.purusha, a.vacana)))
            .collect();
        from_check.sort();
        from_index.sort();
        assert_eq!(
            from_index, from_check,
            "index disagrees with check() for {form}"
        );
    }
}
