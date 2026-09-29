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
/// rule that actually fires; with no optional rule in play the result is
/// always exactly one branch, byte-identical to what the single-prakriyā
/// pipeline produced.
///
/// A blocked branch is skipped by every later rule but is still returned:
/// callers already test `blocked` and must keep doing so, since a blocked
/// branch's partial text is not a surface form.
///
/// A rule whose id is in a branch's `barred` list is skipped on that branch.
/// A rule that fires adds its own `bars` to that list (for a vikalpa rule, on
/// the applied clone only). This is how an apavāda that changes no text, such
/// as 6.4.117, keeps its branch from the rules it overrides.
pub fn run_pipeline(p: Prakriya, stages: &[&[Rule]]) -> Vec<Prakriya> {
    let mut branches = vec![p];
    for stage in stages {
        for rule in *stage {
            // Forks are collected during the sweep and inserted after it,
            // never mid-iteration: every branch must see the rule against
            // the same branch list.
            let mut forks: Vec<(usize, Prakriya)> = Vec::new();
            for (i, branch) in branches.iter_mut().enumerate() {
                // A barred rule is skipped exactly as a blocked branch is:
                // an apavāda that fired earlier on this branch has already
                // decided what this rule would change (`Rule.bars`).
                if branch.blocked || branch.barred.contains(&rule.id) {
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
                        applied.barred.extend_from_slice(rule.bars);
                        forks.push((i, applied));
                    }
                } else if (rule.apply)(branch) {
                    branch.barred.extend_from_slice(rule.bars);
                }
            }
            // Back to front, so each insertion leaves the earlier recorded
            // indices valid.
            for (i, fork) in forks.into_iter().rev() {
                branches.insert(i + 1, fork);
            }
        }
    }
    // Convergent forks collapse: two LIVE branches that assemble the same
    // text are one form, and every consumer — the golden suite's
    // derivation-set comparison, the audit's n_branches == n_forms
    // invariant ("no cell may yield two live branches with the same
    // text") — is written against that invariant. The first real case is
    // the 7.3.86 vikalpa arm under 6.1.90's āṭ-vṛddhi: A+fR and A+arR
    // both surface ArRot. Keep the FIRST occurrence — the declined
    // branch — so index 0 remains the ruleless derivation. Blocked
    // branches are exempt: their partial text is not a surface form and
    // callers filter them on `blocked`.
    let mut seen: Vec<String> = Vec::new();
    branches.retain(|b| {
        if b.blocked {
            return true;
        }
        let text = b.text();
        if seen.contains(&text) {
            false
        } else {
            seen.push(text);
            true
        }
    });
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
        bars: &[],
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
        bars: &[],
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
        bars: &[],
        apply: |_p| false,
    };

    /// A mandatory rule, for testing that the non-forking path is untouched.
    const PUSH_M: Rule = Rule {
        id: "m",
        name: "test",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let b = p.snapshot();
            p.terms[0].text.push('m');
            p.record("m", "test", b);
            true
        },
    };

    /// A mandatory rule that pushes `b` and bars `x`.
    const BARS_X: Rule = Rule {
        id: "bx",
        name: "test",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &["x"],
        apply: |p| {
            let b = p.snapshot();
            p.terms[0].text.push('b');
            p.record("bx", "test", b);
            true
        },
    };

    /// The same, optional.
    const BARS_X_OPTIONALLY: Rule = Rule {
        id: "bxv",
        name: "test",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["x"],
        apply: |p| {
            let b = p.snapshot();
            p.terms[0].text.push('b');
            p.record("bxv", "test", b);
            true
        },
    };

    /// A barring rule that always declines its own guard.
    const BARS_X_BUT_DECLINES: Rule = Rule {
        id: "bxd",
        name: "test",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &["x"],
        apply: |_p| false,
    };

    fn texts(branches: &[Prakriya]) -> Vec<String> {
        branches.iter().map(|p| p.text()).collect()
    }

    #[test]
    fn a_vikalpa_rule_that_declines_forks_nothing() {
        // The overwhelmingly common case: 6.4.107 sees 1864 of the
        // grammar's 1872 cells and fires on none of them (only 8 cells fire
        // it at all — counted by walking every `PARADIGM` cell and checking
        // each branch's log for "6.4.107"). If declining forked, the whole
        // suite would double.
        let out = run_pipeline(p1("a"), &[&[DECLINES][..]]);
        assert_eq!(texts(&out), vec!["a"]);
    }

    #[test]
    fn a_firing_vikalpa_rule_forks_declined_first() {
        // Declined-first is load-bearing, not cosmetic: index 0 must stay
        // byte-identical to the pre-fork engine's single output, which is
        // what makes "the 1800 goldens are unchanged" checkable.
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
            bars: &[],
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
            bars: &[],
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

    #[test]
    fn convergent_forks_collapse_to_the_declined_branch() {
        // Two branches that assemble the same text are one form. The first
        // real case is fR's laṅ: A+fR and A+arR both surface ArRot once
        // 6.1.90's vṛddhi runs. Keep the FIRST (declined) branch so index
        // 0 stays the ruleless derivation.
        // Build: one vikalpa rule that fires but leaves the text equal.
        let noop_fork = Rule {
            id: "test.noop",
            name: "noop",
            kind: RuleKind::Vidhi,
            vikalpa: true,
            bars: &[],
            apply: |p| {
                let before = p.snapshot();
                p.record("test.noop", "noop", before);
                true
            },
        };
        let out = run_pipeline(p1("x"), &[&[noop_fork]]);
        assert_eq!(out.len(), 1, "the converged fork must be pruned");
        assert!(out[0].log.iter().all(|s| s.sutra != "test.noop"));
    }

    #[test]
    fn a_barred_rule_is_skipped_on_the_barring_branch() {
        // Without the bar, PUSH_X would fork: ["ab", "abx"].
        let out = run_pipeline(p1("a"), &[&[BARS_X, PUSH_X][..]]);
        assert_eq!(texts(&out), vec!["ab"]);
        assert_eq!(out[0].barred, vec!["x"]);
    }

    #[test]
    fn an_optional_barrer_bars_its_applied_branch_only() {
        // Declined branch "a" still forks on PUSH_X; applied branch "ab" is barred.
        let out = run_pipeline(p1("a"), &[&[BARS_X_OPTIONALLY, PUSH_X][..]]);
        assert_eq!(texts(&out), vec!["a", "ax", "ab"]);
        assert!(out[0].barred.is_empty());
        assert_eq!(out[2].barred, vec!["x"]);
    }

    #[test]
    fn a_barring_rule_that_declines_bars_nothing() {
        let out = run_pipeline(p1("a"), &[&[BARS_X_BUT_DECLINES, PUSH_X][..]]);
        assert_eq!(texts(&out), vec!["a", "ax"]);
        assert!(out.iter().all(|b| b.barred.is_empty()));
    }

    #[test]
    fn a_bar_skips_only_the_rule_it_names() {
        let out = run_pipeline(p1("a"), &[&[BARS_X, PUSH_Y, PUSH_M][..]]);
        assert_eq!(texts(&out), vec!["abm", "abym"]);
    }

    #[test]
    fn forks_of_a_barred_branch_inherit_the_bar() {
        // bxv: [a, ab]. PUSH_Y forks both: [a, ay, ab, aby]. PUSH_X forks
        // only the unbarred two.
        let out = run_pipeline(p1("a"), &[&[BARS_X_OPTIONALLY, PUSH_Y, PUSH_X][..]]);
        assert_eq!(texts(&out), vec!["a", "ax", "ay", "ayx", "ab", "aby"]);
    }

    #[test]
    fn a_text_neutral_barring_fork_survives_once_a_barred_rule_diverges_it() {
        // 6.4.117's shape: an optional rule that changes no text and bars the
        // rule that would. Unlike `convergent_forks_collapse_to_the_declined_branch`,
        // the barred rule makes the two branches differ, so both survive,
        // declined first.
        const KEEP: Rule = Rule {
            id: "keep",
            name: "test",
            kind: RuleKind::Vidhi,
            vikalpa: true,
            bars: &["m"],
            apply: |p| {
                let b = p.snapshot();
                p.record("keep", "test", b);
                true
            },
        };
        let out = run_pipeline(p1("a"), &[&[KEEP, PUSH_M][..]]);
        assert_eq!(texts(&out), vec!["am", "a"]);
        let ids: Vec<&str> = out[1].log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, vec!["keep"]);
    }
}
