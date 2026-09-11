//! The corpus derived once, for tests that would otherwise call
//! `Panini::check()` in a loop.
//!
//! `panini_analyze::candidates()` discards its argument and returns the full
//! cross-product, so every `check()` re-derives the entire corpus and a loop
//! over N forms costs N². This module does that derivation once per test
//! binary and answers lookups from the result.
//!
//! The build below mirrors `Panini::check()`'s predicate exactly — same
//! candidate set, same `!blocked` filter, same exact-match on `text()`. That
//! duplication is deliberate but load-bearing, so `roundtrip.rs` reconciles
//! this index against the real `check()`. In the blocking tier,
//! `roundtrip_sampled` catches drift on one cell per root only: a change that
//! affects just the unsampled forms passes it while the paradigm loops pass
//! against a stale index. `roundtrip_exhaustive` (`mise run test-full`)
//! reconciles every derived form. Run it after changing `Panini::check()`,
//! `panini_analyze::candidates()`, or this file.

use std::collections::HashMap;
use std::sync::LazyLock;

use panini_analyze::candidates;
use panini_data::{Lakara, Pada, Purusha, Vacana};
use panini_lipi::normalize;
use panini_prakriya::derive as derive_prakriya;

/// One analysis of one form. Deliberately carries no `trace`: holding every
/// branch's rule log would balloon the index for a comparison the
/// `tests/trace/` binary already owns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexedAnalysis {
    /// `Dhatu::code`, matching what `Analysis::dhatu` reports — the
    /// user-facing spelling, deliberately not unique.
    pub dhatu: &'static str,
    pub lakara: Lakara,
    pub pada: Pada,
    pub purusha: Purusha,
    pub vacana: Vacana,
}

pub struct FormIndex {
    map: HashMap<String, Vec<IndexedAnalysis>>,
}

impl FormIndex {
    /// Every analysis of `form`, empty if the corpus derives no such form.
    /// Normalizes the way `check()` does, so a caller cannot accidentally
    /// bypass scheme detection by passing IAST or Devanagari.
    pub fn analyses(&self, form: &str) -> &[IndexedAnalysis] {
        let (slp1, _) = normalize(form);
        self.map.get(&slp1).map_or(&[][..], |v| v.as_slice())
    }
}

static INDEX: LazyLock<FormIndex> = LazyLock::new(|| {
    let mut map: HashMap<String, Vec<IndexedAnalysis>> = HashMap::new();
    for c in candidates("") {
        for p in derive_prakriya(c.dhatu, c.lakara, c.pada, c.purusha, c.vacana) {
            // Same guard as `check()`: a blocked prakriyā's text is a partial
            // string that can still collide with a genuine input.
            if p.blocked {
                continue;
            }
            map.entry(p.text()).or_default().push(IndexedAnalysis {
                dhatu: c.dhatu.code,
                lakara: c.lakara,
                pada: c.pada,
                purusha: c.purusha,
                vacana: c.vacana,
            });
        }
    }
    FormIndex { map }
});

/// The corpus index, derived on first use and shared for the rest of the
/// binary's life.
pub fn corpus_index() -> &'static FormIndex {
    &INDEX
}
