use panini::Panini;
use panini_data::{Purusha, Vacana, dhatus};

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
        for &lakara in panini_analyze::LAKARAS {
            for &(pu, va) in CELLS {
                let p = engine.derive(d, lakara, d.pada, pu, va);
                let form = p.text();
                let r = engine.check(&form);
                if p.blocked {
                    // The derivation was declined (adādi × vidhiliṅ is gated
                    // until slice 5b; see `panini_prakriya::derive`), so there
                    // is no surface form to recover. What must hold instead is
                    // that its partial text is never accepted as a word at
                    // all — not merely never attributed back to this root,
                    // which would still allow it to be misread as some
                    // other root's form.
                    assert!(
                        r.analyses.is_empty(),
                        "blocked derivation leaked a form: {} {} -> {} (analyses: {:?})",
                        d.code,
                        panini::lakara_name(lakara),
                        form,
                        r.analyses
                            .iter()
                            .map(|a| (
                                a.dhatu.as_str(),
                                panini::lakara_name(a.lakara),
                                a.form_slp1.as_str()
                            ))
                            .collect::<Vec<_>>()
                    );
                    continue;
                }
                assert!(
                    r.analyses
                        .iter()
                        .any(|a| a.dhatu == d.code && a.form_slp1 == form && a.lakara == lakara),
                    "roundtrip failed: {} {} -> {}",
                    d.code,
                    panini::lakara_name(lakara),
                    form
                );
            }
        }
    }
}
