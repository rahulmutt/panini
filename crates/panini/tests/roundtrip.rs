use panini::Panini;
use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};

const CELLS: &[(Purusha, Vacana)] = &[
    (Purusha::Prathama, Vacana::Eka),
    (Purusha::Prathama, Vacana::Dvi),
    (Purusha::Prathama, Vacana::Bahu),
    (Purusha::Madhyama, Vacana::Eka),
    (Purusha::Madhyama, Vacana::Dvi),
    (Purusha::Madhyama, Vacana::Bahu),
    (Purusha::Uttama, Vacana::Eka),
    (Purusha::Uttama, Vacana::Dvi),
    (Purusha::Uttama, Vacana::Bahu),
];

#[test]
fn generate_then_check_recovers_inputs() {
    let engine = Panini::new();
    for d in dhatus() {
        for &(pu, va) in CELLS {
            let form = engine
                .derive(d, Lakara::Lat, Pada::Parasmaipada, pu, va)
                .text();
            let r = engine.check(&form);
            assert!(
                r.analyses
                    .iter()
                    .any(|a| a.dhatu == d.code && a.form_slp1 == form),
                "roundtrip failed: {} -> {}",
                d.code,
                form
            );
        }
    }
}
