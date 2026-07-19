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
