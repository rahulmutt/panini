//! Every form the engine derives must be recoverable by `check()`.
//!
//! Two entry points share one body, so the sampled and exhaustive versions
//! cannot drift apart — only the iterator differs. The sampled one runs in the
//! blocking tier; the exhaustive one runs via `mise run test-full`.
//!
//! The body also reconciles `common::index` against the real `check()`. That
//! is what lets the paradigm binary trust the index: it is never taken on its
//! own authority, but checked against the thing it stands in for, on every
//! root.

mod common;

use common::CELLS;
use common::index::corpus_index;
use panini::Panini;
use panini_analyze::LAKARAS;
use panini_data::dhatus;
use panini_data::{Dhatu, Lakara, Pada, Purusha, Vacana};

type Cell = (&'static Dhatu, Lakara, Pada, Purusha, Vacana);

/// One cell per root, rotating the lakāra and the (puruṣa, vacana) pair
/// independently. `lcm(4, 9) == 36` and there are more than 72 roots, so every
/// lakāra × cell pair appears at least twice while every root appears exactly
/// once. No RNG, so no seed to record — and a new root adds itself, which is
/// what keeps the number of `check()` calls here growing linearly with the
/// corpus. Each of those calls still re-derives the whole corpus, so this
/// test's cost stays quadratic, at roughly 1/45 of the exhaustive constant.
fn sample() -> impl Iterator<Item = Cell> {
    dhatus().iter().enumerate().map(|(i, d)| {
        let padas = d.pada.padas();
        let (purusha, vacana) = CELLS[i % CELLS.len()];
        (
            d,
            LAKARAS[i % LAKARAS.len()],
            padas[i % padas.len()],
            purusha,
            vacana,
        )
    })
}

/// The same cross-product `panini_analyze::candidates()` builds.
fn full_cross_product() -> impl Iterator<Item = Cell> {
    dhatus().iter().flat_map(|d| {
        // `CELLS` is a `const` array, so `CELLS.iter()` would borrow a
        // temporary that is dropped at the end of the statement while the
        // returned iterator still holds it. `into_iter()` takes an owned copy.
        // `LAKARAS` is already a `&'static [Lakara]`, so `.iter()` is fine.
        LAKARAS.iter().flat_map(move |&lakara| {
            CELLS.into_iter().flat_map(move |(purusha, vacana)| {
                d.pada
                    .padas()
                    .iter()
                    .map(move |&pada| (d, lakara, pada, purusha, vacana))
            })
        })
    })
}

fn fingerprints(
    analyses: impl Iterator<Item = (String, Lakara, Pada, Purusha, Vacana)>,
) -> Vec<String> {
    let mut v: Vec<String> = analyses.map(|a| format!("{a:?}")).collect();
    v.sort();
    v
}

fn roundtrip_over(cells: impl Iterator<Item = Cell>) {
    let engine = Panini::new();
    let index = corpus_index();
    for (d, lakara, pada, purusha, vacana) in cells {
        for p in engine.derive(d, lakara, pada, purusha, vacana) {
            // The loop only ever asks for padas the root admits, so nothing
            // here should be blocked. Assert it rather than filtering: a
            // blocked branch appearing would mean `padas()` and the pada-
            // sanction rules (1.3.12 / 1.3.78) had come apart.
            assert!(
                !p.blocked,
                "{} {} {:?} {:?} {:?} derived a blocked branch",
                d.code,
                panini::lakara_name(lakara),
                pada,
                purusha,
                vacana
            );
            let form = p.text();
            let r = engine.check(&form);
            assert!(
                r.analyses
                    .iter()
                    .any(|a| a.dhatu == d.code && a.form_slp1 == form && a.lakara == lakara),
                "roundtrip failed: {} {} -> {}",
                d.code,
                panini::lakara_name(lakara),
                form
            );
            let from_check = fingerprints(
                r.analyses
                    .iter()
                    .map(|a| (a.dhatu.clone(), a.lakara, a.pada, a.purusha, a.vacana)),
            );
            let from_index = fingerprints(
                index
                    .analyses(&form)
                    .iter()
                    .map(|a| (a.dhatu.to_string(), a.lakara, a.pada, a.purusha, a.vacana)),
            );
            assert_eq!(
                from_index, from_check,
                "common::index disagrees with check() for {form}"
            );
        }
    }
}

#[test]
fn roundtrip_sampled() {
    roundtrip_over(sample());
}

#[test]
#[ignore = "exhaustive; run via `mise run test-full`"]
fn roundtrip_exhaustive() {
    roundtrip_over(full_cross_product());
}
