//! Shared helpers for the ordered-trace witnesses in the sibling
//! modules.

use panini::Panini;
use panini_data::{Gana, Lakara, Pada, Purusha, Vacana, dhatus};
use panini_prakriya::derive;

pub fn trace_for(word: &str) -> Vec<String> {
    let engine = Panini::new();
    let r = engine.check(word);
    let a = r
        .analyses
        .iter()
        .find(|a| a.form_slp1 == word)
        .expect("expected an analysis deriving exactly this surface form");
    a.trace.iter().map(|s| s.sutra.clone()).collect()
}

/// The trace of one paradigm cell, addressed by COORDINATES rather than by
/// surface string. `trace_for` resolves a word, and a word is ambiguous for
/// an ubhayapadī root whose two padas can share a surface — these ṇatva pins
/// care about a specific (root, lakāra, pada, cell), so they address it
/// directly and read the declined branch's own log.
pub fn cell_trace(
    number: &str,
    lakara: Lakara,
    pada: Pada,
    purusha: Purusha,
    vacana: Vacana,
) -> (String, Vec<String>) {
    let d = dhatus()
        .iter()
        .find(|d| d.dhatupatha == number)
        .unwrap_or_else(|| panic!("{number} is not a curated root"));
    let p = derive(d, lakara, pada, purusha, vacana)
        .into_iter()
        .next()
        .expect("every enumerable cell derives at least one branch");
    (p.text(), p.log.iter().map(|s| s.sutra.clone()).collect())
}

/// Index of a sūtra in a trace, for the pins that must assert ORDER rather
/// than mere presence.
pub fn at(trace: &[String], sutra: &str) -> usize {
    trace
        .iter()
        .position(|s| s == sutra)
        .unwrap_or_else(|| panic!("{sutra} absent from {trace:?}"))
}

const ALL_CELLS: [(Purusha, Vacana); 9] = [
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

/// Every (root, lakāra, pada, cell, branch) of the curated corpus whose log
/// carries `sutra`, as (dhatupatha, gaṇa).
pub fn credited(sutra: &str) -> Vec<(&'static str, Gana)> {
    let mut out = Vec::new();
    for d in dhatus() {
        for lakara in [Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin] {
            for &pada in d.pada.padas() {
                for (purusha, vacana) in ALL_CELLS {
                    for p in derive(d, lakara, pada, purusha, vacana) {
                        if !p.blocked && p.log.iter().any(|s| s.sutra == sutra) {
                            out.push((d.dhatupatha, d.gana));
                        }
                    }
                }
            }
        }
    }
    out
}
