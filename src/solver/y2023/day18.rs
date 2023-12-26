use crate::solver::{combine_solver, ProblemSolver};
use crate::utils::grid::GridDirection;
use anyhow::{anyhow, bail};
use derive_more::{Deref, FromStr};
use itertools::Itertools;
use std::fmt::Debug;

combine_solver!(Day18, Day18Part1, Day18Part2);

#[derive(Deref, Debug)]
pub struct Day18Part1(Box<[(GridDirection, isize)]>);

#[derive(Deref, Debug)]
pub struct Day18Part2(Box<[(GridDirection, isize)]>);

impl FromStr for Day18Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let inner = s
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                let direction = if let Some(direction) = iter.next() {
                    if direction.len() == 1 {
                        match direction.as_bytes()[0] {
                            b'U' => GridDirection::North,
                            b'D' => GridDirection::South,
                            b'L' => GridDirection::West,
                            b'R' => GridDirection::East,
                            _ => bail!("Invalid direction: {:?}", direction),
                        }
                    } else {
                        bail!("Invalid direction: {:?}", direction)
                    }
                } else {
                    bail!("Invalid input: {:?}", line)
                };

                let step = if let Some(step) = iter.next() {
                    <isize>::from_str(step)?
                } else {
                    bail!("Invalid input: {:?}", line)
                };

                Ok((direction, step))
            })
            .try_collect()?;
        Ok(Day18Part1(inner))
    }
}

impl ProblemSolver for Day18Part1 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        let (area, perimeter, (last_x, last_y)) = self.iter().fold(
            (0_isize, 0_isize, (0_isize, 0_isize)),
            |(mut area, mut perimeter, (prev_x, prev_y)), (direction, step)| {
                let (next_x, next_y) = match direction {
                    GridDirection::North => (prev_x, prev_y - step),
                    GridDirection::South => (prev_x, prev_y + step),
                    GridDirection::East => (prev_x + step, prev_y),
                    GridDirection::West => (prev_x - step, prev_y),
                    _ => unreachable!(),
                };
                area += (prev_y + next_y) * (prev_x - next_x);
                perimeter += step;
                (area, perimeter, (next_x, next_y))
            },
        );
        if last_x != 0 && last_y != 0 {
            bail!("Last vertex is not the beginning vertex")
        }

        Ok((area.unsigned_abs() + perimeter as usize) / 2 + 1)
    }
}

impl FromStr for Day18Part2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                if iter.next().is_none() || iter.next().is_none() {
                    bail!("Invalid input: {:?}", line)
                };

                iter.next().ok_or_else(|| anyhow!("Invalid input: {:?}", line)).and_then(|s| {
                    let direction = s.as_bytes()[7];
                    let direction = match s.as_bytes()[7] {
                        b'0' => GridDirection::East,
                        b'1' => GridDirection::South,
                        b'2' => GridDirection::West,
                        b'3' => GridDirection::North,
                        _ => bail!("Invalid direction: {:?}", direction as char),
                    };

                    let step = <isize>::from_str_radix(&s[2..7], 16)?;
                    Ok((direction, step))
                })
            })
            .try_collect()?;
        Ok(Day18Part2(inner))
    }
}

impl ProblemSolver for Day18Part2 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        let (area, perimeter, (last_x, last_y)) = self.iter().fold(
            (0_isize, 0_isize, (0_isize, 0_isize)),
            |(mut area, mut perimeter, (prev_x, prev_y)), (direction, step)| {
                let (next_x, next_y) = match direction {
                    GridDirection::North => (prev_x, prev_y - step),
                    GridDirection::South => (prev_x, prev_y + step),
                    GridDirection::East => (prev_x + step, prev_y),
                    GridDirection::West => (prev_x - step, prev_y),
                    _ => unreachable!(),
                };
                area += (prev_y + next_y) * (prev_x - next_x);
                perimeter += step;
                (area, perimeter, (next_x, next_y))
            },
        );
        if last_x != 0 && last_y != 0 {
            bail!("Last vertex is not the beginning vertex")
        }

        Ok((area.unsigned_abs() + perimeter as usize) / 2 + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day18::Day18;
    use crate::solver::TwoPartsProblemSolver;

    use indoc::indoc;

    use std::str::FromStr;

    const SAMPLE_INPUT_1: &str = indoc! {"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
    "};

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day18::from_str(SAMPLE_INPUT_1)?.solve_1()?, 62);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day18::from_str(SAMPLE_INPUT_1)?.solve_2()?, 952408144115);
        Ok(())
    }
}
