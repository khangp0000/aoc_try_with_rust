use crate::solver::{share_struct_solver, ProblemSolver};
use anyhow::bail;
use derive_more::{Deref, FromStr};
use std::borrow::Cow;
use std::rc::Rc;

share_struct_solver!(Day9, Day9Part1, Day9Part2);

#[derive(Deref)]
pub struct Day9Part1(Vec<Vec<i32>>);

#[derive(Deref)]
pub struct Day9Part2(Rc<Day9Part1>);

impl FromStr for Day9Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Day9Part1(
            s.lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(<i32>::from_str)
                        .map(|r| r.map_err(anyhow::Error::from))
                        .collect::<anyhow::Result<Vec<_>>>()
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
        ));
    }
}

impl ProblemSolver for Day9Part1 {
    type SolutionType = i32;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        return self.iter().map(predict_next_val).sum::<anyhow::Result<_>>();
    }
}

fn predict_next_val(input: &Vec<i32>) -> anyhow::Result<i32> {
    let mut current = Cow::Borrowed(input);
    let mut sum = 0_i32;
    while current.len() > 1 {
        sum += current.last().unwrap();
        current = Cow::Owned(
            current.iter().zip(current[1..].iter()).map(|(l, r)| *r - *l).collect::<Vec<_>>(),
        );
    }

    if !current.is_empty() && current[0] != 0 {
        bail!("Cannot reduce following sequence to 0s: {:?}", input);
    }

    return Ok(sum);
}

impl ProblemSolver for Day9Part2 {
    type SolutionType = i32;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        return self.iter().map(predict_prev_val).sum::<anyhow::Result<_>>();
    }
}

fn predict_prev_val(input: &Vec<i32>) -> anyhow::Result<i32> {
    let mut current = Cow::Borrowed(input);
    let mut acc = 0_i32;
    let mut adding = true;
    let diff = 1_usize;
    while current.len() > 1 {
        if diff == 1 {
            if adding {
                acc += current.first().unwrap();
            } else {
                acc -= current.first().unwrap();
            }
            adding = !adding;
        }
        current = Cow::Owned(
            current.iter().zip(current[1..].iter()).map(|(l, r)| *r - *l).collect::<Vec<_>>(),
        );
    }

    if !current.is_empty() && current[0] != 0 {
        bail!("Cannot reduce following sequence to 0s: {:?}", input);
    }

    return Ok(acc);
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day9::Day9;
    use crate::solver::TwoPartsProblemSolver;
    use indoc::indoc;
    use std::str::FromStr;

    const SAMPLE_INPUT: &str = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
    "};

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day9::from_str(SAMPLE_INPUT)?.solve_1()?, 114);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day9::from_str(SAMPLE_INPUT)?.solve_2()?, 2);
        Ok(())
    }
}
