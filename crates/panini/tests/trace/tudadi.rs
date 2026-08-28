//! tudadi's ordered-trace witnesses. Helpers live in
//! `crate::helpers`; the module doc governing this suite is in
//! `main.rs`.

use crate::helpers::trace_for;

#[test]
fn tudati_trace_is_exactly_the_sa_block_path() {
    // tud laṭ P 3sg: śa (3.1.77) → ṅit (1.2.4); 7.3.86 blocked (no todati).
    assert_eq!(
        trace_for("tudati"),
        vec!["1.3.78", "3.4.78", "1.3.9", "3.1.77", "1.3.9", "1.2.4"]
    );
}

#[test]
fn jusate_trace_is_exactly_the_sa_atmanepada_block_path() {
    // juṣ laṭ Ā 3sg: śa path; 7.3.86 blocked (juṣate NOT joṣate).
    assert_eq!(
        trace_for("juzate"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.77", "1.3.9", "1.2.4"
        ]
    );
}
