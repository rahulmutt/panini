use crate::prakriya::Prakriya;
use crate::rule::Rule;

pub use crate::rule::{Rule as _Rule, RuleKind};

/// Apply each rule in order, at most once. Rules self-guard via `apply`
/// returning false when inapplicable. Ordering is the controller's concern.
pub fn run_pipeline(p: &mut Prakriya, rules: &[Rule]) {
    for rule in rules {
        (rule.apply)(p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prakriya::Prakriya;
    use crate::rule::{Rule, RuleKind};
    use crate::term::Term;

    #[test]
    fn pipeline_applies_in_order_and_logs() {
        let mut p = Prakriya {
            terms: vec![Term::new("Bo"), Term::new("a")],
            log: vec![],
        };
        let rules = [Rule {
            id: "6.1.78",
            name: "eco'yavAyAvaH",
            kind: RuleKind::Vidhi,
            apply: |p| {
                if p.terms[0].text == "Bo" {
                    let b = p.snapshot();
                    p.terms[0].text = "Bav".into();
                    p.record("6.1.78", "eco'yavAyAvaH", b);
                    true
                } else {
                    false
                }
            },
        }];
        run_pipeline(&mut p, &rules);
        assert_eq!(p.text(), "Bava");
        assert_eq!(p.log.last().unwrap().sutra, "6.1.78");
    }
}
