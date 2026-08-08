use crate::prakriya::Prakriya;
use crate::rule::Rule;

pub use crate::rule::{Rule as _Rule, RuleKind};

/// Apply each stage in order, and each rule within a stage in order, at
/// most once, to every live branch. Rules self-guard via `apply` returning
/// false when inapplicable. Ordering is the controller's concern.
///
/// Stages are a file-organisation boundary, not a grammatical one: the
/// flattened sequence is what the grammar is, and it must read the same as
/// it did when the rules lived in a single array.
///
/// Returns a *set* of derivations. A prakriyā forks only at a `vikalpa`
/// rule that actually fires (see task 3); with no optional rule in play the
/// result is always exactly one branch, byte-identical to what the
/// single-prakriyā pipeline produced.
///
/// A blocked branch is skipped by every later rule but is still returned:
/// callers already test `blocked` and must keep doing so, since a blocked
/// branch's partial text is not a surface form.
pub fn run_pipeline(p: Prakriya, stages: &[&[Rule]]) -> Vec<Prakriya> {
    let mut branches = vec![p];
    for stage in stages {
        for rule in *stage {
            for branch in branches.iter_mut() {
                if branch.blocked {
                    continue;
                }
                (rule.apply)(branch);
            }
        }
    }
    branches
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prakriya::Prakriya;
    use crate::rule::{Rule, RuleKind};
    use crate::term::Term;

    #[test]
    fn pipeline_applies_in_order_and_logs() {
        let p = Prakriya {
            terms: vec![Term::new("Bo"), Term::new("a")],
            log: vec![],
            ..Default::default()
        };
        let rules = [Rule {
            id: "6.1.78",
            name: "eco'yavAyAvaH",
            kind: RuleKind::Vidhi,
            vikalpa: false,
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
        let out = run_pipeline(p, &[&rules[..]]);
        assert_eq!(out.len(), 1, "no vikalpa rule: exactly one branch");
        let p = &out[0];
        assert_eq!(p.text(), "Bava");
        assert_eq!(p.log.last().unwrap().sutra, "6.1.78");
        // The logged `before` snapshot must be the pre-mutation text, not a
        // placeholder (pins `Prakriya::snapshot`).
        assert_eq!(p.log.last().unwrap().before, "Boa");
    }
}
