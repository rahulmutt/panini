#![forbid(unsafe_code)]
pub mod controller;
pub mod it_samjna;
pub mod prakriya;
pub mod rule;
pub mod term;

pub use controller::run_pipeline;
pub use prakriya::{Prakriya, RuleStep};
pub use rule::{Rule, RuleKind};
pub use term::{Tag, Term};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prakriya_concatenates_and_logs() {
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("a"), Term::new("ti")],
            log: vec![],
        };
        assert_eq!(p.text(), "BUati");
        let before = p.snapshot();
        p.terms[0].text = "Bo".into();
        p.record("7.3.84", "sArvadhAtukArdhadhAtukayoH", before);
        assert_eq!(p.log.len(), 1);
        assert_eq!(p.log[0].sutra, "7.3.84");
        assert_eq!(p.log[0].after, "Boati");
    }

    #[test]
    fn term_tags() {
        let mut t = Term::new("ti");
        t.add(Tag::Tin);
        assert!(t.has(Tag::Tin));
        assert!(!t.has(Tag::Dhatu));
    }
}
