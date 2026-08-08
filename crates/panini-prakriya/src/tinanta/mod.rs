//! The tiṅanta pipeline, as seven ordered rule-stage modules plus two support
//! layers.
//!
//! The seven stages — `samjna`, `tin`, `vikarana`, `anga`, `guna`, `adesha`,
//! `tripadi` — are declared below in *pipeline* order in `TINANTA_RULES`,
//! which is the grammar's actual sequencing; the `mod` declarations above
//! them are alphabetical and carry no ordering meaning of their own. `terms`
//! and `sound` are support layers underneath the rules (term-layout constants
//! and sandhi/sound predicates respectively) — neither holds a `Rule`.
//! Which stage a rule belongs to is decided by its position relative to
//! **3.1.68**, not by its sūtra family; see `terms.rs`.

use crate::context::Context;
use crate::controller::run_pipeline;
use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::term::{Tag, Term};
use panini_data::{Dhatu, Gana, Lakara, Pada, Purusha, Vacana};

mod adesha;
mod anga;
mod guna;
mod samjna;
mod sound;
mod terms;
mod tin;
mod tripadi;
mod vikarana;

#[cfg(test)]
pub(crate) mod derivation_tests;

/// The shared derivation helper, re-exported here so the stage files' test
/// modules import it by a stable path (`crate::tinanta::form_g`) rather than
/// reaching into `derivation_tests` by path. `anga.rs` and `tripadi.rs`
/// import it through this line; keep the path stable if the helper moves
/// again. This one function goes through a re-export while `sound`/`terms`
/// items are imported by direct path because `derivation_tests` is
/// `#[cfg(test)]` and expected to move again; routing its consumers through
/// one re-export here means only this line needs updating, rather than N
/// direct paths across the stage files.
#[cfg(test)]
pub(in crate::tinanta) use derivation_tests::form_g;

/// The ordered rule list, as a sequence of pipeline stages. Read the stages
/// in order, and the rules within each stage in order: that flattened
/// sequence IS the grammar this crate implements. Every rule self-guards and
/// returns whether it fired.
pub static TINANTA_RULES: &[&[Rule]] = &[
    samjna::SAMJNA,
    tin::TIN,
    vikarana::VIKARANA,
    anga::ANGA_RULES,
    guna::GUNA,
    adesha::ADESHA,
    tripadi::TRIPADI,
];

/// The rules in pipeline order, flattened across stages.
pub fn rules() -> impl Iterator<Item = &'static Rule> {
    TINANTA_RULES.iter().flat_map(|stage| stage.iter())
}

/// Returns every branch of the derivation, not just one: an optional
/// (vikalpa) rule can fork the prakriyā, so the vec may hold more than one
/// entry, and any entry may be `blocked` — a blocked prakriyā's `text()` is
/// a partial string, not a surface form, and callers must filter those out
/// before using the result. Index 0 is always the declined reading, i.e.
/// what this function would have returned with no optional rule in play.
pub fn derive(
    dhatu: &Dhatu,
    lakara: Lakara,
    pada: Pada,
    purusha: Purusha,
    vacana: Vacana,
) -> Vec<Prakriya> {
    let mut p = Prakriya {
        ctx: Context::new(lakara, pada, purusha, vacana),
        ..Default::default()
    };
    p.terms.push({
        let mut t = Term::new(dhatu.code);
        t.add(Tag::Dhatu);
        if matches!(dhatu.pada, Pada::Atmanepada) {
            t.add(Tag::Atmanepadin);
        }
        match dhatu.gana {
            Gana::Divadi => t.add(Tag::Divadi),
            Gana::Tudadi => t.add(Tag::Tudadi),
            Gana::Adadi => t.add(Tag::Adadi),
            Gana::Kryadi => t.add(Tag::Kryadi),
            Gana::Svadi => t.add(Tag::Svadi),
            Gana::Bhvadi => {}
        }
        t
    });
    run_pipeline(p, TINANTA_RULES)
}
