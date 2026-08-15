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
