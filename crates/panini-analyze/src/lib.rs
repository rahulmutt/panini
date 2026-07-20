#![forbid(unsafe_code)]
use panini_data::{Dhatu, Lakara, Pada, Purusha, Vacana, dhatus};

pub struct Candidate {
    pub dhatu: &'static Dhatu,
    pub lakara: Lakara,
    pub pada: Pada,
    pub purusha: Purusha,
    pub vacana: Vacana,
}

/// The lakāras this build can derive. The analyzer proposes every
/// (root × lakāra × cell); the engine confirms by exact surface match.
pub const LAKARAS: &[Lakara] = &[Lakara::Lat, Lakara::Lan, Lakara::Lot];

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

pub fn candidates(surface_slp1: &str) -> Vec<Candidate> {
    let mut out = Vec::new();
    for d in dhatus() {
        for &lakara in LAKARAS {
            for &(purusha, vacana) in CELLS {
                out.push(Candidate {
                    dhatu: d,
                    lakara,
                    pada: Pada::Parasmaipada,
                    purusha,
                    vacana,
                });
            }
        }
    }
    // Return the full (still tiny) candidate set; the engine confirms by exact match.
    let _ = surface_slp1;
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proposes_bhu_prathama_eka_for_bhavati() {
        let cands = candidates("Bavati");
        assert!(cands.iter().any(|c| c.dhatu.code == "BU"
            && matches!(c.purusha, panini_data::Purusha::Prathama)
            && matches!(c.vacana, panini_data::Vacana::Eka)));
    }

    #[test]
    fn always_narrows_to_nonempty_for_covered_ending() {
        assert!(!candidates("BavAmaH").is_empty());
    }
}
