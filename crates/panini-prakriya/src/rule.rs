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
    /// mutation invalidates — unless that consumer's guard is provably
    /// disjoint from the optional rule's own guard. 6.4.107 leaves
    /// `SHAP.text == "n"` for svādi and `SHAP.text == ""` for tanādi, which
    /// invalidates two predicates: `vikarana_u_asamyogapurva` (whose
    /// consumers all precede 6.4.107) and `sound_before_ending` (`u` before
    /// the mutation, `n`/`` after). Either returning the wrong answer
    /// downstream, on the forked branch only, surfaces as half a paradigm
    /// being wrong with both halves individually plausible.
    /// `sound_before_ending` does have one consumer below 6.4.107 — 6.4.101
    /// `her DiH` — but it is safe: its guard requires `ENDING.text == "hi"`,
    /// which 6.4.107 already excludes by requiring an m- or v-initial
    /// ending, so the two never contend. See 6.4.107's own comment in
    /// `tinanta/adesha.rs` for the worked argument.
    pub vikalpa: bool,
    /// Returns true if it mutated the prakriya (and recorded a RuleStep).
    pub apply: fn(&mut Prakriya) -> bool,
}
