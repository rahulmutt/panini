use crate::context::Context;
use crate::term::Term;

#[derive(Debug, Clone)]
pub struct RuleStep {
    pub sutra: String,
    pub name: String,
    pub before: String,
    pub after: String,
}

#[derive(Debug, Clone, Default)]
pub struct Prakriya {
    pub terms: Vec<Term>,
    pub log: Vec<RuleStep>,
    pub ctx: Context,
    /// Set when a pada-sanction rule (1.3.12 / 1.3.78) determines the
    /// requested derivation is impossible (wrong pada for the root), so
    /// it must not be reported. The pipeline stops with partial,
    /// not-a-real-surface-form text — that text can still collide with
    /// a genuine input string, so callers (e.g. `Panini::check`) must
    /// check `!blocked` before matching it, not rely on the text alone.
    pub blocked: bool,
}

impl Prakriya {
    pub fn text(&self) -> String {
        self.terms.iter().map(|t| t.text.as_str()).collect()
    }
    pub fn snapshot(&self) -> String {
        self.text()
    }
    pub fn record(&mut self, sutra: &str, name: &str, before: String) {
        let after = self.text();
        self.log.push(RuleStep {
            sutra: sutra.into(),
            name: name.into(),
            before,
            after,
        });
    }
}
