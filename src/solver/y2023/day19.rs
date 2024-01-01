use std::borrow::Cow;
use std::fmt::Debug;
use std::ops::ControlFlow::{Break, Continue};
use std::rc::Rc;

use anyhow::{anyhow, bail, Result};
use derive_more::{Deref, FromStr};
use indexmap::IndexMap;
use thiserror::Error;

use crate::solver::y2023::day19::Error::InvalidCategory;
use crate::solver::{share_struct_solver, ProblemSolver};
use crate::utils::get_double_newline_regex;
use crate::utils::graph::dfs;
use crate::utils::int_range::IntRange;

share_struct_solver!(Day19, Day19Part1, Day19Part2);

#[derive(Debug)]
pub struct Day19Part1 {
    accepted: Vec<[IntRange<usize>; 4]>,
    input: Vec<[usize; 4]>,
}

#[derive(Deref, Debug)]
struct RuleMap(IndexMap<String, Option<Vec<MappingRule>>>);

#[derive(Debug)]
struct MappingRule {
    constraint: Option<MappingRuleConstraint>,
    target_rule_idx: usize,
}

#[derive(Debug)]
struct MappingRuleConstraint {
    category: usize,
    range_constraint: RangeConstraint,
}

#[derive(Debug)]
enum RangeConstraint {
    LessThan(usize),
    MoreThan(usize),
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct State {
    xmas: [IntRange<usize>; 4],
    evaluate_rule_idx: usize,
}

impl FromStr for RuleMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut map = IndexMap::default();
        s.lines().try_for_each(|line| parse_one_map_from_str_and_name_idx_set(line, &mut map))?;
        Ok(RuleMap(map))
    }
}

fn parse_one_map_from_str_and_name_idx_set(
    s: &str,
    map: &mut IndexMap<String, Option<Vec<MappingRule>>>,
) -> Result<()> {
    let (rule_name, rule_val) =
        s.split_once('{').ok_or_else(|| anyhow!("Invalid input line: {:?}", s))?;
    if rule_val.ends_with('}') {
        let rule_val = &rule_val[0..rule_val.len() - 1];
        let rule_set = rule_val
            .split(',')
            .map(|rule| MappingRule::from_str_and_name_idx_set(rule, map))
            .collect::<Result<Vec<MappingRule>>>()?;
        let old_val = map.insert(rule_name.to_owned(), Some(rule_set));
        if matches!(old_val, Some(Some(_))) {
            bail!("There are more than 1 rule for key: {:?}", rule_name)
        }
    } else {
        bail!("Invalid input line: {:?}", s)
    };

    Ok(())
}

impl State {
    fn apply_rule(&self, rules: &[MappingRule]) -> Result<Vec<State>> {
        let mut next_states = Vec::default();
        let execute_result = rules.iter().try_fold(Cow::Borrowed(&self.xmas), |mut state, rule| {
            if let Some(constraint) = &rule.constraint {
                let category = constraint.category;
                let range_constraint = &constraint.range_constraint;
                let (valid, remainder) = range_constraint.split(&state[category]);
                if let Some(new_range) = valid {
                    let mut new_xmas = state.clone().into_owned();
                    new_xmas[category] = new_range;
                    next_states
                        .push(State { xmas: new_xmas, evaluate_rule_idx: rule.target_rule_idx });
                }

                if let Some(remainder_range) = remainder {
                    state.to_mut()[category] = remainder_range;
                    Continue(state)
                } else {
                    Break(())
                }
            } else {
                next_states.push(State {
                    xmas: state.into_owned(),
                    evaluate_rule_idx: rule.target_rule_idx,
                });

                Break(())
            }
        });

        if Break(()) != execute_result {
            bail!("Rule flow not terminate correctly. Missing default target rule? {:?}", rules);
        }

        Ok(next_states)
    }
}

impl RangeConstraint {
    fn split(
        &self,
        int_range: &IntRange<usize>,
    ) -> (Option<IntRange<usize>>, Option<IntRange<usize>>) {
        match self {
            RangeConstraint::LessThan(upper_limit) => {
                if *upper_limit > int_range.end {
                    (Some(*int_range), None)
                } else if *upper_limit <= int_range.start {
                    (None, Some(*int_range))
                } else {
                    (
                        Some(IntRange::new(int_range.start, *upper_limit - 1).unwrap()),
                        Some(IntRange::new(*upper_limit, int_range.end).unwrap()),
                    )
                }
            }
            RangeConstraint::MoreThan(lower_limit) => {
                if *lower_limit < int_range.start {
                    (Some(*int_range), None)
                } else if *lower_limit >= int_range.end {
                    (None, Some(*int_range))
                } else {
                    (
                        Some(IntRange::new(*lower_limit + 1, int_range.end).unwrap()),
                        Some(IntRange::new(int_range.start, *lower_limit).unwrap()),
                    )
                }
            }
        }
    }
}

impl MappingRule {
    fn from_str_and_name_idx_set(
        s: &str,
        name_idx_map: &mut IndexMap<String, Option<Vec<MappingRule>>>,
    ) -> Result<MappingRule> {
        if let Some((left, right)) = s.split_once(':') {
            let entry = name_idx_map.entry(right.to_owned());
            let target_rule_idx = entry.index();
            entry.or_insert(None);
            Ok(MappingRule {
                constraint: Some(MappingRuleConstraint::from_str(left)?),
                target_rule_idx,
            })
        } else {
            let entry = name_idx_map.entry(s.to_owned());
            let target_rule_idx = entry.index();
            entry.or_insert(None);
            Ok(MappingRule { constraint: None, target_rule_idx })
        }
    }
}

impl MappingRuleConstraint {
    fn from_str(s: &str) -> Result<Self> {
        if let Some((left, right)) = s.split_once('<') {
            let category = from_category_to_index(left)?;
            let upper_limit = <usize>::from_str(right)?;
            return Ok(MappingRuleConstraint {
                category,
                range_constraint: RangeConstraint::LessThan(upper_limit),
            });
        } else if let Some((left, right)) = s.split_once('>') {
            let category = from_category_to_index(left)?;
            let lower_limit = <usize>::from_str(right)?;
            return Ok(MappingRuleConstraint {
                category,
                range_constraint: RangeConstraint::MoreThan(lower_limit),
            });
        }

        bail!("Cannot parse mapping rule with constraint {:?}", s)
    }
}

fn from_category_to_index(category: &str) -> Result<usize> {
    match category {
        "x" => Ok(0),
        "m" => Ok(1),
        "a" => Ok(2),
        "s" => Ok(3),
        _ => bail!(InvalidCategory(category.to_owned())),
    }
}

#[derive(Deref)]
pub struct Day19Part2(Rc<Day19Part1>);

#[derive(Error, Debug)]
pub enum Error {
    #[error("Category {0:?} is not valid")]
    InvalidCategory(String),
}

impl FromStr for Day19Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let double_newline_regex = get_double_newline_regex().clone();
        let mut part_iter = double_newline_regex.split(s);
        let rule_part =
            part_iter.next().ok_or_else(|| anyhow!("Cannot get rule part in input: \n{}", s))?;
        let rule_map = RuleMap::from_str(rule_part)?;
        let start = State {
            xmas: [IntRange::new(1, 4000)?; 4],
            evaluate_rule_idx: rule_map.get_index_of("in").unwrap(),
        };
        let accept_rule_idx = rule_map.get_index_of("A").unwrap();
        let mut accepted = Vec::default();
        dfs(
            start,
            |state| {
                if state.evaluate_rule_idx == accept_rule_idx {
                    accepted.push(state.xmas);
                    Vec::default()
                } else if let (_, Some(rule)) = rule_map.get_index(state.evaluate_rule_idx).unwrap()
                {
                    state.apply_rule(rule).unwrap()
                } else {
                    Vec::default()
                }
            },
            |_, _| false,
            (),
            |_, _| (),
        );

        let rating_part =
            part_iter.next().ok_or_else(|| anyhow!("Cannot get rating part in input: \n{}", s))?;
        let ratings = rating_part.lines().map(parse_rating_line).collect::<Result<_>>()?;
        Ok(Day19Part1 { accepted, input: ratings })
    }
}

impl Day19Part1 {
    fn is_valid(&self, input: &[usize; 4]) -> bool {
        self.accepted.iter().any(|ranges| (0..4).all(|i| ranges[i].contains(&input[i])))
    }
}

fn parse_rating_line(s: &str) -> Result<[usize; 4]> {
    if s.starts_with('{') && s.ends_with('}') {
        let s = &s[1..s.len() - 1];
        let mut res = [0_usize; 4];
        s.split(',').try_for_each(|s| {
            let (category, value) =
                s.split_once('=').ok_or_else(|| anyhow!("Invalid input line: {:?}", s))?;
            let category = from_category_to_index(category)?;
            let value = <usize>::from_str(value)?;
            res[category] = value;
            Ok::<_, anyhow::Error>(())
        })?;
        return Ok(res);
    }
    bail!("Invalid input line: {:?}", s)
}

impl ProblemSolver for Day19Part1 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        Ok(self.input.iter().filter(|i| self.is_valid(i)).map(|i| i.iter().sum::<usize>()).sum())
    }
}

impl ProblemSolver for Day19Part2 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        Ok(self.accepted.iter().map(|i| i.iter().map(IntRange::len).product::<usize>()).sum())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Result;
    use indoc::indoc;

    use crate::solver::y2023::day19::Day19;
    use crate::solver::TwoPartsProblemSolver;

    const SAMPLE_INPUT_1: &str = indoc! {r"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
    "};

    #[test]
    fn test_sample_1() -> Result<()> {
        assert_eq!(Day19::from_str(SAMPLE_INPUT_1)?.solve_1()?, 19114);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> Result<()> {
        assert_eq!(Day19::from_str(SAMPLE_INPUT_1)?.solve_2()?, 167409079868000);
        Ok(())
    }
}
