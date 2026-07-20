use panini_data::{Lakara, Pada, Purusha, Vacana};

/// The grammatical coordinates of a derivation.
///
/// Rules in `TINANTA_RULES` self-guard on this rather than being selected into
/// per-lakāra lists, mirroring the Aṣṭādhyāyī: one ordered rule set whose rules
/// state their own conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Context {
    pub lakara: Lakara,
    pub pada: Pada,
    pub purusha: Purusha,
    pub vacana: Vacana,
    /// Whether ṅit-conditioned rules (3.4.99, 3.4.100, 3.4.101) apply.
    ///
    /// True inherently for laṅ and vidhiliṅ, which are ṅit by nature (the ṅ
    /// anubandha in their names). For loṭ it is set at derivation time by
    /// 3.4.85 loṭo laṅvat, an *atideśa* — keeping that piece of grammar in
    /// the rule list where it appears in the trace, rather than hiding it in
    /// a match arm here.
    pub is_ngit_like: bool,
}

impl Context {
    pub fn new(lakara: Lakara, pada: Pada, purusha: Purusha, vacana: Vacana) -> Context {
        Context {
            lakara,
            pada,
            purusha,
            vacana,
            // laṅ and liṅ are ṅit inherently (the ṅ anubandha in their own
            // names); loṭ acquires it via rule 3.4.85.
            is_ngit_like: matches!(lakara, Lakara::Lan | Lakara::VidhiLin),
        }
    }
}

impl Default for Context {
    /// A placeholder context, so `Prakriya::default()` keeps working for unit
    /// tests that build a `Prakriya` by hand and exercise no context-guarded
    /// rule. It is deliberately NOT `#[derive(Default)]` on the `Lakara` /
    /// `Pada` enums: there is no such thing as a "default lakāra" in the
    /// grammar, and claiming one in the public data API would be a lie.
    fn default() -> Context {
        Context::new(
            Lakara::Lat,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vidhilin_is_ngit_like_inherently() {
        // liṅ, like laṅ, is a ṅit lakāra by its own name (the anubandha ṅ),
        // so no atideśa rule is involved — unlike loṭ (3.4.85).
        let c = Context::new(
            Lakara::VidhiLin,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert!(c.is_ngit_like);
    }

    #[test]
    fn lot_is_not_ngit_like_at_construction() {
        // loṭ acquires ṅit-likeness only via rule 3.4.85 at derivation time.
        let c = Context::new(
            Lakara::Lot,
            Pada::Parasmaipada,
            Purusha::Prathama,
            Vacana::Eka,
        );
        assert!(!c.is_ngit_like);
    }
}
