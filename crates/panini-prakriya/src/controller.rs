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
            // Forks are collected during the sweep and inserted after it,
            // never mid-iteration: every branch must see the rule against
            // the same branch list.
            let mut forks: Vec<(usize, Prakriya)> = Vec::new();
            for (i, branch) in branches.iter_mut().enumerate() {
                if branch.blocked {
                    continue;
                }
                if rule.vikalpa {
                    // Clone first, apply to the clone: the branch in place
                    // is the DECLINED reading and must stay untouched, so
                    // that index 0 remains byte-identical to what the
                    // pre-fork engine produced for this cell.
                    let mut applied = branch.clone();
                    // Keep the clone only if the rule actually fired. A
                    // vikalpa rule that declines its own guard offers no
                    // choice at all, so there is nothing to fork.
                    if (rule.apply)(&mut applied) {
                        forks.push((i, applied));
                    }
                } else {
                    (rule.apply)(branch);
                }
            }
            // Back to front, so each insertion leaves the earlier recorded
            // indices valid.
            for (i, fork) in forks.into_iter().rev() {
                branches.insert(i + 1, fork);
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

    fn p1(text: &str) -> Prakriya {
        Prakriya {
            terms: vec![Term::new(text)],
            ..Default::default()
        }
    }

    const PUSH_X: Rule = Rule {
        id: "x",
        name: "test",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            let b = p.snapshot();
            p.terms[0].text.push('x');
            p.record("x", "test", b);
            true
        },
    };

    const PUSH_Y: Rule = Rule {
        id: "y",
        name: "test",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            let b = p.snapshot();
            p.terms[0].text.push('y');
            p.record("y", "test", b);
            true
        },
    };

    /// A vikalpa rule that always declines its own guard.
    const DECLINES: Rule = Rule {
        id: "d",
        name: "test",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |_p| false,
    };

    /// A mandatory rule, for testing that the non-forking path is untouched.
    const PUSH_M: Rule = Rule {
        id: "m",
        name: "test",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let b = p.snapshot();
            p.terms[0].text.push('m');
            p.record("m", "test", b);
            true
        },
    };

    fn texts(branches: &[Prakriya]) -> Vec<String> {
        branches.iter().map(|p| p.text()).collect()
    }

    #[test]
    fn a_vikalpa_rule_that_declines_forks_nothing() {
        // The overwhelmingly common case: 6.4.107 sees ~1504 of 1512 cells
        // and fires on none of them. If declining forked, the whole suite
        // would double.
        let out = run_pipeline(p1("a"), &[&[DECLINES][..]]);
        assert_eq!(texts(&out), vec!["a"]);
    }

    #[test]
    fn a_firing_vikalpa_rule_forks_declined_first() {
        // Declined-first is load-bearing, not cosmetic: index 0 must stay
        // byte-identical to the pre-fork engine's single output, which is
        // what makes "the 1512 goldens are unchanged" checkable.
        let out = run_pipeline(p1("a"), &[&[PUSH_X][..]]);
        assert_eq!(texts(&out), vec!["a", "ax"]);
    }

    #[test]
    fn the_declined_branch_records_no_step() {
        let out = run_pipeline(p1("a"), &[&[PUSH_X][..]]);
        assert!(out[0].log.is_empty(), "declined branch ran nothing");
        assert_eq!(out[1].log.len(), 1);
        assert_eq!(out[1].log[0].sutra, "x");
    }

    #[test]
    fn later_rules_apply_to_every_branch() {
        let out = run_pipeline(p1("a"), &[&[PUSH_X, PUSH_M][..]]);
        assert_eq!(texts(&out), vec!["am", "axm"]);
    }

    #[test]
    fn two_firing_vikalpa_rules_yield_four_ordered_branches() {
        // Branch count is 2^k. Order is fully determined: the second rule
        // forks each of the first rule's branches in place.
        let out = run_pipeline(p1("a"), &[&[PUSH_X, PUSH_Y][..]]);
        assert_eq!(texts(&out), vec!["a", "ay", "ax", "axy"]);
    }

    #[test]
    fn a_blocked_branch_is_skipped_but_still_returned() {
        const BLOCKS: Rule = Rule {
            id: "b",
            name: "test",
            kind: RuleKind::Vidhi,
            vikalpa: false,
            apply: |p| {
                p.blocked = true;
                true
            },
        };
        let out = run_pipeline(p1("a"), &[&[BLOCKS, PUSH_M, PUSH_X][..]]);
        assert_eq!(texts(&out), vec!["a"], "no rule may run after a block");
        assert!(out[0].blocked);
    }

    #[test]
    fn a_mandatory_rule_takes_one_branch_to_one_branch() {
        let out = run_pipeline(p1("a"), &[&[PUSH_M, PUSH_M][..]]);
        assert_eq!(texts(&out), vec!["amm"]);
    }

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
