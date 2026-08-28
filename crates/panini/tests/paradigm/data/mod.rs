//! The golden tables, one file per gaṇa, keyed by the
//! dhātupāṭha-number prefix of every row: 01 bhvādi, 02 adādi, 04
//! divādi, 05 svādi, 06 tudādi, 07 rudhādi, 09 kryādi. Row order
//! within a file preserves the pre-split monolith's order; the
//! concatenated statics below are what the tests in `main.rs`
//! consume, and no test depends on row order. A new gaṇa lands as
//! one new file here plus its lines in the `mod` list and both
//! `concat` arrays.

use std::sync::LazyLock;

use panini_data::Pada;

pub mod adadi;
pub mod bhvadi;
pub mod divadi;
pub mod kryadi;
pub mod rudhadi;
pub mod svadi;
pub mod tudadi;

pub type ParadigmRow = (&'static str, &'static str, Pada, [&'static str; 9]);
pub type AlternateRow = (
    &'static str,
    &'static str,
    Pada,
    usize,
    &'static str,
    &'static str,
);

/// (root_number, lakara_label, pada, [P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B])
/// in SLP1. `PARADIGM`'s first column is a `Dhatu::dhatupatha` — the entry
/// number, unique by construction, so the two √aś rows are distinct without
/// anyone deciding which gaṇa's was the incumbent (`09.0059` kryādi,
/// `05.0020` svādi). Resolve a number against the `DHATUS` table in
/// `panini-data` to see which root a block is for; the tables carry no
/// per-row comment, deliberately, since 450 uncheckable comments is a
/// staleness liability no test could pin. The pada column is no longer
/// inferable from the root alone: 1.3.72 gives some roots a
/// `PadaAssignment` that admits both, so a block has to declare which pada
/// it is a block OF.
pub static PARADIGM: LazyLock<Vec<ParadigmRow>> = LazyLock::new(|| {
    [
        bhvadi::PARADIGM,
        adadi::PARADIGM,
        divadi::PARADIGM,
        svadi::PARADIGM,
        tudadi::PARADIGM,
        rudhadi::PARADIGM,
        kryadi::PARADIGM,
    ]
    .concat()
});

/// Second and third valid forms, for cells where an optional (vikalpa) rule
/// forks the derivation. `(root_number, lakara_label, pada, cell index into the
/// [&str; 9], alternate form, vikalpa key)`.
///
/// The vikalpa key names the optional rules applied on the branch that
/// derives this form, `+`-joined in pipeline order. It is not decoration:
/// `every_alternate_names_the_vikalpa_rules_that_produced_it` checks it
/// against the branch's own log, so a right form reached by the wrong rule
/// fails here.
///
/// `PARADIGM` holds index 0 — the derivation with no optional rule applied —
/// so an alternate is by construction never `PARADIGM`'s own string.
/// Cell order is [P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B], so 7 and 8
/// are uttama dvi and uttama bahu. `pada` names the block the row belongs
/// to, same as `PARADIGM`'s column.
pub static ALTERNATES: LazyLock<Vec<AlternateRow>> = LazyLock::new(|| {
    [
        bhvadi::ALTERNATES,
        adadi::ALTERNATES,
        divadi::ALTERNATES,
        svadi::ALTERNATES,
        tudadi::ALTERNATES,
        rudhadi::ALTERNATES,
        kryadi::ALTERNATES,
    ]
    .concat()
});
