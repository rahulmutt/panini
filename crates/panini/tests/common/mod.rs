//! Shared fixtures for the integration test binaries under `tests/`. Each
//! test file (`paradigm.rs`, `roundtrip.rs`, ...) compiles this module
//! separately as its own crate, so an item only one of them uses still
//! triggers `dead_code` in the others; allow it here rather than contorting
//! the tests to use everything.
#![allow(dead_code)]

use panini_data::{Lakara, Purusha, Vacana};

pub const CELLS: [(Purusha, Vacana); 9] = [
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

pub const LAKARA_BY_NAME: [(&str, Lakara); 4] = [
    ("laT", Lakara::Lat),
    ("laN", Lakara::Lan),
    ("loT", Lakara::Lot),
    ("viDiliN", Lakara::VidhiLin),
];
