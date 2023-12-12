use anyhow::{anyhow, bail};
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::ControlFlow;

use crate::solver::{ProblemSolver, TwoSolversCombined};
use derive_more::{Deref, Display, FromStr};

use crate::utils::{FromSScanfError, WarningResult};
use thiserror::Error;

static MAX_MAP_COUNT: usize = 100000_usize;

#[derive(Deref, FromStr)]
pub struct Day8(TwoSolversCombined<Day8Part1, Day8Part2>);

pub struct Day8Part1 {
    directions: Vec<Direction>,
    map: HashMap<String, (String, String)>,
}

#[derive(Deref, FromStr)]
pub struct Day8Part2(Day8Part1);

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Display, Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cannot convert {0:?} to Direction")]
    InvalidInputDirectionChar(char),
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        return match value.to_ascii_lowercase() {
            'l' | 'L' => Ok(Direction::Left),
            'r' | 'R' => Ok(Direction::Right),
            _ => Err(Error::InvalidInputDirectionChar(value))?,
        };
    }
}

fn parse_map_line(s: &str) -> anyhow::Result<(String, (String, String))> {
    let (key, value_left, value_right) =
        sscanf::sscanf!(s, "{str:/.../} = ({str:/.../}, {str:/.../})").map_err(|e| {
            crate::utils::Error::from_sscanf_err(
                &e,
                s.to_owned(),
                "{str:/.../} = ({str:/.../}, {str:/.../})",
            )
        })?;
    return Ok((
        key.to_owned(),
        (value_left.to_owned(), value_right.to_owned()),
    ));
}

impl FromStr for Day8Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let directions = if let Some(direction_line) = lines.next() {
            direction_line
                .chars()
                .map(Direction::try_from)
                .collect::<Result<Vec<_>, _>>()?
        } else {
            bail!("Direction line is missing from input.")
        };

        if let Some(separation_line) = lines.next() {
            if !separation_line.trim().is_empty() {
                bail!("Expected empty separation line. Got {:?}", separation_line)
            }
        } else {
            bail!("Separation empty line is missing from input.")
        }

        let map = lines.map(parse_map_line).collect::<Result<_, _>>()?;
        return Ok(Day8Part1 { directions, map });
    }
}

impl ProblemSolver<Day8Part1> for Day8Part1 {
    type SolutionType = u32;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        return match self.directions.iter().cycle().take(MAX_MAP_COUNT).try_fold(
            ("AAA", 0_u32),
            |(key, count), direction| {
                return if let Some((value_left, value_right)) = self.map.get(key) {
                    let key = match direction {
                        Direction::Left => value_left.as_str(),
                        Direction::Right => value_right.as_str(),
                    };

                    if key == "ZZZ" {
                        return ControlFlow::Break(Ok(count + 1));
                    }

                    ControlFlow::Continue((key, count + 1))
                } else {
                    ControlFlow::Break(Err(anyhow!("Cannot find value for key {:?}", key)))
                };
            },
        ) {
            ControlFlow::Continue(_) => bail!("Cannot find \"ZZZ\" after {} step.", MAX_MAP_COUNT),
            ControlFlow::Break(r) => r,
        };
    }
}

impl ProblemSolver<Day8Part2> for Day8Part2 {
    type SolutionType = WarningResult<usize>;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        return match self.directions.iter().cycle()
            .take(MAX_MAP_COUNT)
            .try_fold(((*self).map.keys().filter(|s| s.ends_with('A')).collect::<Vec<_>>(), 0_u32, 1_usize), |(keys, mut count, mut lcm), direction| {
                let res = keys.iter().map(|&key| self.map.get(key).map(|(value_left, value_right)|
                    match direction {
                        Direction::Left => value_left,
                        Direction::Right => value_right,
                    }
                ).ok_or_else(|| anyhow!("Cannot find value for key {:?}", key)))
                    .collect::<Result<Vec<_>, _>>();

                return match res {
                    Ok(new_keys) => {
                        let len_before_filter = new_keys.len();
                        let new_keys = new_keys.into_iter().filter(|key| !key.ends_with('Z')).collect::<Vec<_>>();
                        let len_after_filter = new_keys.len();
                        count += 1;
                        if len_after_filter !=  len_before_filter {
                            lcm = num::integer::lcm(lcm, count as usize);

                            if len_after_filter == 0 {
                                return ControlFlow::Break(Ok(lcm));
                            }
                        }

                        ControlFlow::Continue((new_keys, count, lcm))
                    }
                    Err(e) => ControlFlow::Break(Err(e))
                }
            }) {
            ControlFlow::Continue(_) => bail!("Cannot find value set ending with 'Z' after {} step.", MAX_MAP_COUNT),
            ControlFlow::Break(r) => r.map(|count| WarningResult::new(count, "--Assuming \"**Z\" repeat and repeat cycle is divisible by directions length--"))
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day8::Day8;
    use crate::solver::TwoPartsProblemSolver;

    use indoc::indoc;

    use std::str::FromStr;

    static SAMPLE_INPUT_1: &str = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
    "};

    static SAMPLE_INPUT_2: &str = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
    "};

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day8::from_str(SAMPLE_INPUT_1)?.solve_1()?, 6);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(*Day8::from_str(SAMPLE_INPUT_2)?.solve_2()?, 6);
        Ok(())
    }
}
