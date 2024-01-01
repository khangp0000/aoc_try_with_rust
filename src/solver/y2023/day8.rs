use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::ControlFlow;
use std::rc::Rc;

use anyhow::{anyhow, bail, Result};
use derive_more::{Deref, Display, FromStr};
use thiserror::Error;

use crate::solver::{share_struct_solver, ProblemSolver};
use crate::utils::WarningResult;

const MAX_MAP_COUNT: usize = 100000_usize;

share_struct_solver! {Day8, Day8Part1, Day8Part2}

pub struct Day8Part1 {
    directions: Vec<Direction>,
    map: HashMap<String, (String, String)>,
}

#[derive(Deref)]
pub struct Day8Part2(Rc<Day8Part1>);

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Display, Debug)]
enum Direction {
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

    fn try_from(value: char) -> Result<Self> {
        match value.to_ascii_lowercase() {
            'l' | 'L' => Ok(Direction::Left),
            'r' | 'R' => Ok(Direction::Right),
            _ => Err(Error::InvalidInputDirectionChar(value))?,
        }
    }
}

fn parse_map_line(s: &str) -> Result<(String, (String, String))> {
    if &s[3..7] != " = (" || &s[10..12] != ", " || &s[15..16] != ")" {
        bail!(format!("Expected \"... = (..., ...)\" but got {:?}", s));
    }
    let key = &s[0..3];
    let value_left = &s[7..10];
    let value_right = &s[12..15];

    Ok((key.to_owned(), (value_left.to_owned(), value_right.to_owned())))
}

impl FromStr for Day8Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let directions = if let Some(direction_line) = lines.next() {
            direction_line.chars().map(Direction::try_from).collect::<Result<Vec<_>>>()?
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

        let map = lines.map(parse_map_line).collect::<Result<_>>()?;
        Ok(Day8Part1 { directions, map })
    }
}

impl ProblemSolver for Day8Part1 {
    type SolutionType = u32;

    fn solve(&self) -> Result<Self::SolutionType> {
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

impl ProblemSolver for Day8Part2 {
    type SolutionType = WarningResult<usize>;

    fn solve(&self) -> Result<Self::SolutionType> {
        return match self.directions.iter().cycle()
            .take(MAX_MAP_COUNT)
            .try_fold((self.map.keys().filter(|s| s.ends_with('A')).collect::<Vec<_>>(), 0_u32, 1_usize), |(keys, mut count, mut lcm), direction| {
                let res = keys.iter().map(|&key| self.map.get(key).map(|(value_left, value_right)|
                    match direction {
                        Direction::Left => value_left,
                        Direction::Right => value_right,
                    }
                ).ok_or_else(|| anyhow!("Cannot find value for key {:?}", key)))
                    .collect::<Result<Vec<_>>>();

                match res {
                    Ok(new_keys) => {
                        let len_before_filter = new_keys.len();
                        let new_keys = new_keys.into_iter().filter(|key| !key.ends_with('Z')).collect::<Vec<_>>();
                        let len_after_filter = new_keys.len();
                        count += 1;
                        if len_after_filter != len_before_filter {
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
    use std::str::FromStr;

    use anyhow::Result;
    use indoc::indoc;

    use crate::solver::y2023::day8::Day8;
    use crate::solver::TwoPartsProblemSolver;

    const SAMPLE_INPUT_1: &str = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
    "};

    const SAMPLE_INPUT_2: &str = indoc! {"
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
    fn test_sample_1() -> Result<()> {
        assert_eq!(Day8::from_str(SAMPLE_INPUT_1)?.solve_1()?, 6);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> Result<()> {
        assert_eq!(*Day8::from_str(SAMPLE_INPUT_2)?.solve_2()?, 6);
        Ok(())
    }
}
