use anyhow::Context;
use num::{FromPrimitive, PrimInt};

use crate::solver::TwoPartsProblemSolver;
use std::str::FromStr;

pub struct Day6 {
    part_1: Day6Part1,
    part_2: Day6Part2,
}

pub struct Day6Part1 {
    times: Vec<i32>,
    distances: Vec<i32>,
}

pub struct Day6Part2 {
    time: i64,
    distance: i64,
}

impl FromStr for Day6 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_part1 = s.lines();
        let mut lines_part2 = s.lines();

        return Ok(Day6 {
            part_1: Day6Part1 {
                times: lines_part1
                    .next()
                    .with_context(|| format!("Invalid input: {}", s))?
                    .split_whitespace()
                    .skip(1)
                    .map(<i32>::from_str)
                    .collect::<Result<_, _>>()?,
                distances: lines_part1
                    .next()
                    .with_context(|| format!("Invalid input: {}", s))?
                    .split_whitespace()
                    .skip(1)
                    .map(<i32>::from_str)
                    .collect::<Result<_, _>>()?,
            },
            part_2: Day6Part2 {
                time: lines_part2
                    .next()
                    .with_context(|| format!("Invalid input: {}", s))?
                    .split_once(':')
                    .with_context(|| format!("Invalid input: {}", s))?
                    .1
                    .bytes()
                    .filter(|b| *b >= b'0')
                    .map(|c| (c - b'0') as i64)
                    .filter(|d| *d < 10)
                    .reduce(|l, r| l * 10 + r)
                    .with_context(|| format!("Invalid input: {}", s))?,
                distance: lines_part2
                    .next()
                    .with_context(|| format!("Invalid input: {}", s))?
                    .split_once(':')
                    .with_context(|| format!("Invalid input: {}", s))?
                    .1
                    .bytes()
                    .filter(|b| *b >= b'0')
                    .map(|c| (c - b'0') as i64)
                    .filter(|d| *d < 10)
                    .reduce(|l, r| l * 10 + r)
                    .with_context(|| format!("Invalid input: {}", s))?,
            },
        });
    }
}

impl TwoPartsProblemSolver<i32, i64> for Day6 {
    fn solve_1(&self) -> anyhow::Result<i32> {
        let part_1 = &self.part_1;
        return part_1
            .times
            .iter()
            .zip(part_1.distances.iter())
            .map(|(time, distance)| {
                find_time_hold_range(*time, *distance).with_context(|| {
                    format!(
                        "Failed to solve for pair time {} and distance record {}",
                        time, distance
                    )
                })
            })
            .try_fold(1, |acc, res| {
                res.map(|(left, right)| acc * (right - left + 1))
            });
    }

    fn solve_2(&self) -> anyhow::Result<i64> {
        let (left, right) = find_time_hold_range(self.part_2.time, self.part_2.distance).with_context(|| {
            format!(
                "Failed to solve for pair time {} and distance record {}",
                self.part_2.time, self.part_2.distance
            )
        })?;
        return Ok(right - left + 1)
    }
}

fn find_time_hold_range<T: PrimInt + FromPrimitive>(time: T, record: T) -> Option<(T, T)> {
    let delta = time * time - (record << 2);
    if delta < T::zero() {
        return None;
    }

    let delta_sqrt = delta.to_f64()?.sqrt();
    let time = T::to_f64(&time)?;
    let (left, right) = ((time - delta_sqrt) / 2.0, (time + delta_sqrt) / 2.0);
    let (left_ceil, right_floor) = (left.ceil(), right.floor());
    let left_ceil = if left_ceil == left {
        T::from_f64(left_ceil)? + T::one()
    } else {
        T::from_f64(left_ceil)?
    };
    let right_floor = if right_floor == right {
        T::from_f64(right_floor)? - T::one()
    } else {
        T::from_f64(right_floor)?
    };
    return Some((left_ceil, right_floor));
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day6::{find_time_hold_range, Day6};
    use crate::solver::TwoPartsProblemSolver;
    use indoc::indoc;
    use std::str::FromStr;

    static SAMPLE_INPUT: &str = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
    "};

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day6::from_str(SAMPLE_INPUT)?.solve_1()?, 288);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day6::from_str(SAMPLE_INPUT)?.solve_2()?, 71503_i64);
        Ok(())
    }

    #[test]
    fn test_small_1() -> anyhow::Result<()> {
        assert_eq!(find_time_hold_range(7, 9).unwrap(), (2, 5));
        assert_eq!(find_time_hold_range(15, 40).unwrap(), (4, 11));
        assert_eq!(find_time_hold_range(30, 200).unwrap(), (11, 19));
        Ok(())
    }
}
