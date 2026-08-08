use crate::prakriya::Prakriya;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleKind {
    Vidhi,
    Samjna,
    Adhikara,
    Paribhasha,
    Atidesha,
}

#[derive(Clone, Copy)]
pub struct Rule {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: RuleKind,
    /// vikalpa — the sūtra applies optionally (anyatarasyām / vā /
    /// vibhāṣā). `run_pipeline` forks the prakriyā here: both the applied
    /// and the declined reading continue as independent derivations.
    ///
    /// A separate axis from `kind`, not a `RuleKind` variant: 6.4.107 is a
    /// vidhi *and* optional, and collapsing the two would lose that.
    ///
    /// ORDERING CAVEAT, unenforceable by the compiler: an optional rule
    /// must be ordered after every consumer of a predicate its own
    /// mutation invalidates. 6.4.107 leaves `SHAP.text == "n"`, so
    /// `shnu_asamyogapurva` returns false downstream — on the forked
    /// branch only, which surfaces as half a paradigm being wrong with
    /// both halves individually plausible.
    pub vikalpa: bool,
    /// Returns true if it mutated the prakriya (and recorded a RuleStep).
    pub apply: fn(&mut Prakriya) -> bool,
}
