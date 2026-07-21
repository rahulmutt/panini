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
    /// Set when a samjna/sanction rule determines the requested derivation
    /// is impossible (wrong pada for the root). The pipeline stops; the
    /// prakriya's text can never equal a real surface form.
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
