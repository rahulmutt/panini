use crate::context::Context;
use crate::controller::run_pipeline;
use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::term::{Tag, Term};
use panini_data::{Dhatu, Gana, Lakara, Pada, Purusha, Vacana};

mod adesha;
mod anga;
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
/// again.
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
    adesha::ADESHA,
    tripadi::TRIPADI,
];

/// The rules in pipeline order, flattened across stages.
pub fn rules() -> impl Iterator<Item = &'static Rule> {
    TINANTA_RULES.iter().flat_map(|stage| stage.iter())
}

pub fn derive(
    dhatu: &Dhatu,
    lakara: Lakara,
    pada: Pada,
    purusha: Purusha,
    vacana: Vacana,
) -> Prakriya {
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
            Gana::Bhvadi => {}
        }
        t
    });
    run_pipeline(&mut p, TINANTA_RULES);
    p
}
