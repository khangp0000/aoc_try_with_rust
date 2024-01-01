use std::str::FromStr;

use anyhow::{bail, Context, Result};

use crate::solver::y2021::day2::Movement::{Down, Forward, Up};
use crate::solver::TwoPartsProblemSolver;

pub struct Day2 {
    movements: Vec<Movement>,
}

pub enum Movement {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Movement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (movement, value) = s
            .split_once(' ')
            .with_context(|| format!("Failed to split whitespace for string: {}", s))?;
        let value = <i32>::from_str(value)?;
        Ok(match movement {
            "forward" => Forward(value),
            "down" => Down(value),
            "up" => Up(value),
            _ => bail!(format!("Unknown movement: {}", movement)),
        })
    }
}

impl FromStr for Day2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        return Ok(Day2 {
            movements: s.lines().map(Movement::from_str).map(Result::unwrap).collect(),
        });
    }
}

impl TwoPartsProblemSolver for Day2 {
    type Solution1Type = i32;
    type Solution2Type = i32;

    fn solve_1(&self) -> Result<i32> {
        let (x, y) = self.movements.iter().fold((0, 0), |(x, y), step| match step {
            Forward(val) => (x + val, y),
            Down(val) => (x, y + val),
            Up(val) => (x, y - val),
        });

        Ok(x * y)
    }

    fn solve_2(&self) -> Result<i32> {
        let (x, y, _) = self.movements.iter().fold((0, 0, 0), |(x, y, aim), step| match step {
            Forward(val) => (x + val, y + aim * val, aim),
            Down(val) => (x, y, aim + val),
            Up(val) => (x, y, aim - val),
        });
        Ok(x * y)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Result;
    use indoc::indoc;

    use crate::solver::y2021::day2::Day2;
    use crate::solver::TwoPartsProblemSolver;

    const SAMPLE_INPUT: &str = indoc! {"
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
    "};

    #[test]
    fn test_sample_1() -> Result<()> {
        assert_eq!(Day2::from_str(SAMPLE_INPUT)?.solve_1()?, 150);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> Result<()> {
        assert_eq!(Day2::from_str(SAMPLE_INPUT)?.solve_2()?, 900);
        Ok(())
    }
}
