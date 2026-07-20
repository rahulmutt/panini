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
    /// Whether ṅit-conditioned rules (3.4.99, 3.4.101) apply.
    ///
    /// True inherently for laṅ, which is ṅit by nature. For loṭ it is set at
    /// derivation time by 3.4.85 loṭo laṅvat, an *atideśa* — keeping that piece
    /// of grammar in the rule list where it appears in the trace, rather than
    /// hiding it in a match arm here.
    pub is_ngit_like: bool,
}

impl Context {
    pub fn new(lakara: Lakara, pada: Pada, purusha: Purusha, vacana: Vacana) -> Context {
        Context {
            lakara,
            pada,
            purusha,
            vacana,
            // laṅ is ṅit inherently; loṭ acquires it via rule 3.4.85.
            is_ngit_like: matches!(lakara, Lakara::Lan),
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
