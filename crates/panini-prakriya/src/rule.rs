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
    /// The apavāda relation, declared: rule ids this rule BARS on every branch
    /// where it fires. `run_pipeline` skips a barred rule on that branch, so the
    /// barred rules' own guards never have to know about their apavāda. For a
    /// vikalpa rule that is the applied clone only; the declined branch still
    /// runs every barred rule. Eleven rules bar others today (`exactly_the_pinned_bars`
    /// lists them: eight optional-ṇic rules, 7.3.87, 6.4.117 and 8.4.39).
    /// 6.4.117 *ā ca hau* keeps
    /// √hā's `A` before *hi* by changing no text, and bars 6.4.116, 6.4.113 and
    /// 6.4.112, the three rules that would otherwise change that `A`.
    /// 7.3.87 *nābhyastasyāci piti sārvadhātuke* is a mandatory barrer: it changes no text and bars 7.3.86, keeping an abhyasta
    /// aṅga's laghu upadhā before a vowel-initial pit ending (nenijAni).
    /// 8.4.39 *kṣubhnādiṣu ca* is another: it changes no text and bars 8.4.1
    /// and 8.4.2, keeping √tṛp's śnu dental (tfpnoti).
    /// Barring is by id, so barring an id that occurs twice in the pipeline
    /// (7.3.84, 7.3.86, 1.2.4) skips every occurrence on that branch.
    ///
    /// Scope is the BRANCH, not a site. A tinanta prakriyā has one aṅga, so the
    /// two coincide today; a pipeline with several sites must revisit this.
    ///
    /// Ids are strings, checked by `exactly_the_pinned_bars` in
    /// `tinanta/derivation_tests.rs`: every barred id must name a rule that
    /// runs AFTER its barrer, because barring an earlier rule does nothing.
    pub bars: &'static [&'static str],
    /// Returns true if it mutated the prakriya (and recorded a RuleStep).
    pub apply: fn(&mut Prakriya) -> bool,
}
