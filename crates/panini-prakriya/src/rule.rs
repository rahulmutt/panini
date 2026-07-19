use crate::prakriya::Prakriya;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleKind {
    Vidhi,
    Samjna,
    Adhikara,
    Paribhasha,
}

#[derive(Clone, Copy)]
pub struct Rule {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: RuleKind,
    /// Returns true if it mutated the prakriya (and recorded a RuleStep).
    pub apply: fn(&mut Prakriya) -> bool,
}
