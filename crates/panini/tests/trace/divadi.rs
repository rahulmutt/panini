//! divadi's ordered-trace witnesses. Helpers live in
//! `crate::helpers`; the module doc governing this suite is in
//! `main.rs`.

use crate::helpers::trace_for;

#[test]
fn divyati_trace_is_exactly_the_syan_block_lengthen_path() {
    // div laṭ P 3sg: śyan (3.1.69) → apit → ṅit (1.2.4, 2nd application);
    // 7.3.84/7.3.86 blocked (no record); 8.2.77 lengthens div → dīv.
    assert_eq!(
        trace_for("dIvyati"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.1.69", "1.3.9", "1.2.4", "8.2.77"
        ]
    );
}

#[test]
fn manyate_trace_is_exactly_the_syan_atmanepada_path() {
    // man laṭ Ā 3sg: laBate's path with 3.1.68→3.1.69 and the second 1.2.4
    // (śyan ṅit) appended.
    assert_eq!(
        trace_for("manyate"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.69", "1.3.9", "1.2.4"
        ]
    );
}

#[test]
fn kupyet_trace_is_exactly_the_syan_vidhilin_path() {
    // kup vidhiliṅ P 3sg: bhavet's yāsuṭ chain with śyan instead of śap, the
    // second 1.2.4 (śyan ṅit), and NO 7.3.84/6.1.78 (guṇa blocked; kup has no
    // guṇable final and its upadhā guṇa is blocked). `kupyet` now names the
    // forked branch: 8.2.39 obligatorily voices the pada-final `t` to `d`,
    // and 8.4.56 optionally devoices it back.
    assert_eq!(
        trace_for("kupyet"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.100", "3.4.103", "3.1.69", "1.3.9", "1.2.4",
            "7.2.79", "7.2.80", "6.1.87", "6.1.66", "8.2.39", "8.4.56"
        ]
    );
}

#[test]
fn akupyat_trace_shows_7_3_100_declines_for_non_adadi_roots() {
    // Slice 5f: this pin retires the 7.3.100 mutant slice 5e parked.
    //
    // 7.3.100 adaḥ sarveṣām's guard is `!laṅ || !adādi`. The `||` → `&&`
    // mutant makes the rule proceed for a laṅ NON-adādi derivation, and its
    // inner checks do not exclude that case: when 7.3.100 runs the aṅga is
    // still the bare root (`kup`, consonant-final) and 3.4.100 itaś ca has
    // already reduced the ending to a single `t`. The mutant therefore fires,
    // producing akupya + at, and 6.1.97 ato guṇe then merges a + a back to a
    // — so the SURFACE FORM is repaired and no golden notices. Only the
    // ordered trace does: under the mutant it carries two extra steps,
    // "7.3.100" and "6.1.97", at the end.
    // `akupyat` now names the forked branch: 8.2.39 obligatorily voices the
    // pada-final `t` to `d`, and 8.4.56 optionally devoices it back.
    let t = trace_for("akupyat");
    assert_eq!(
        t,
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.100", "3.1.69", "1.3.9", "1.2.4", "6.4.71", "8.2.39",
            "8.4.56"
        ]
    );
    assert!(
        !t.contains(&"7.3.100".to_string()),
        "7.3.100 is √ad's rule and must decline for a divādi root"
    );
}
