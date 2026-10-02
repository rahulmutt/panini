#![forbid(unsafe_code)]
use std::collections::HashMap;
use std::sync::LazyLock;

use panini_data::{Dhatu, Lakara, Pada, Purusha, Vacana, dhatus};
use panini_prakriya::derive;

#[derive(Clone, Copy)]
pub struct Candidate {
    pub dhatu: &'static Dhatu,
    pub lakara: Lakara,
    pub pada: Pada,
    pub purusha: Purusha,
    pub vacana: Vacana,
}

/// The lakāras this build can derive. The analyzer proposes the
/// (root × lakāra × cell) inputs that derive a surface form; the engine
/// confirms by exact surface match.
pub const LAKARAS: &[Lakara] = &[Lakara::Lat, Lakara::Lan, Lakara::Lot, Lakara::VidhiLin];

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

/// Every (root × lakāra × cell × pada) this build can derive, in a fixed
/// order: the single source of what the analyzer can propose.
pub fn all_candidates() -> Vec<Candidate> {
    let mut out = Vec::new();
    for d in dhatus() {
        for &lakara in LAKARAS {
            for &(purusha, vacana) in CELLS {
                for &pada in d.pada.padas() {
                    out.push(Candidate {
                        dhatu: d,
                        lakara,
                        pada,
                        purusha,
                        vacana,
                    });
                }
            }
        }
    }
    out
}

/// Exactly the candidates whose derivation produces `surface_slp1` as an
/// unblocked branch, in `all_candidates()` order. The engine re-derives
/// them to confirm and to attach traces.
pub fn candidates(surface_slp1: &str) -> Vec<Candidate> {
    INDEX.get(surface_slp1).cloned().unwrap_or_default()
}

/// The whole corpus, derived once per process on the first `candidates()`
/// call.
static INDEX: LazyLock<HashMap<String, Vec<Candidate>>> = LazyLock::new(|| {
    index_from(all_candidates().into_iter().map(|c| {
        let branches = derive(c.dhatu, c.lakara, c.pada, c.purusha, c.vacana)
            .into_iter()
            .map(|p| (p.blocked, p.text()))
            .collect();
        (c, branches)
    }))
});

/// Groups candidates by the surface forms their `(blocked, text)` branches
/// produce, keeping input order under every form.
///
/// A blocked branch's text is a partial string (often the bare root code),
/// never a surface form, so it is not indexed (cf. the pada blocks in
/// 1.3.12 / 1.3.78 / 10.0496). A candidate whose vikalpa branches produce one form
/// twice is listed under it once: `check()` re-derives the candidate and
/// reports every matching branch itself, so a second listing would report
/// each branch twice.
fn index_from(
    derived: impl IntoIterator<Item = (Candidate, Vec<(bool, String)>)>,
) -> HashMap<String, Vec<Candidate>> {
    let mut index: HashMap<String, Vec<Candidate>> = HashMap::new();
    for (c, branches) in derived {
        let mut forms: Vec<String> = Vec::new();
        for (blocked, text) in branches {
            if !blocked && !forms.contains(&text) {
                forms.push(text);
            }
        }
        for form in forms {
            index.entry(form).or_default().push(c);
        }
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `Dhatu::dhatupatha` is the unique key; `code` is not (both √aś rows
    /// spell `aS`).
    fn key(c: &Candidate) -> (&'static str, Lakara, Pada, Purusha, Vacana) {
        (c.dhatu.dhatupatha, c.lakara, c.pada, c.purusha, c.vacana)
    }

    fn keys(cs: &[Candidate]) -> Vec<(&'static str, Lakara, Pada, Purusha, Vacana)> {
        cs.iter().map(key).collect()
    }

    /// Two distinct candidates to feed `index_from` synthetic branches.
    fn two() -> (Candidate, Candidate) {
        let all = all_candidates();
        (all[0], all[1])
    }

    fn b(blocked: bool, text: &str) -> (bool, String) {
        (blocked, text.to_string())
    }

    #[test]
    fn proposes_bhu_prathama_eka_for_bhavati() {
        let cands = candidates("Bavati");
        assert!(cands.iter().any(|c| c.dhatu.code == "BU"
            && matches!(c.purusha, panini_data::Purusha::Prathama)
            && matches!(c.vacana, panini_data::Vacana::Eka)));
    }

    #[test]
    fn proposes_candidates_for_a_derived_form() {
        assert!(!candidates("BavAmaH").is_empty());
    }

    #[test]
    fn proposes_nothing_for_a_nonform() {
        assert!(candidates("gacCati").is_empty());
        assert!(candidates("").is_empty());
    }

    #[test]
    fn all_candidates_is_the_full_cross_product() {
        let expected: usize = dhatus()
            .iter()
            .map(|d| LAKARAS.len() * CELLS.len() * d.pada.padas().len())
            .sum();
        assert_eq!(all_candidates().len(), expected);
    }

    #[test]
    fn blocked_branch_is_not_indexed() {
        let (a, _) = two();
        let index = index_from([(a, vec![b(true, "x"), b(false, "y")])]);
        assert!(!index.contains_key("x"));
        assert_eq!(keys(&index["y"]), keys(&[a]));
    }

    #[test]
    fn same_form_branches_list_candidate_once() {
        let (a, _) = two();
        let index = index_from([(a, vec![b(false, "x"), b(false, "x")])]);
        assert_eq!(keys(&index["x"]), keys(&[a]));
    }

    #[test]
    fn candidate_with_two_forms_is_indexed_under_both() {
        let (a, _) = two();
        let index = index_from([(a, vec![b(false, "x"), b(false, "y")])]);
        assert_eq!(keys(&index["x"]), keys(&[a]));
        assert_eq!(keys(&index["y"]), keys(&[a]));
        assert_eq!(index.len(), 2);
    }

    #[test]
    fn candidates_keep_input_order_under_a_shared_form() {
        let (a, c) = two();
        let ab = index_from([(a, vec![b(false, "x")]), (c, vec![b(false, "x")])]);
        assert_eq!(keys(&ab["x"]), keys(&[a, c]));
        let ba = index_from([(c, vec![b(false, "x")]), (a, vec![b(false, "x")])]);
        assert_eq!(keys(&ba["x"]), keys(&[c, a]));
    }
}
