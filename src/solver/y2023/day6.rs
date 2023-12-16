use crate::solver::{combine_solver, ProblemSolver};
use crate::utils::int_trait::Integer;
use anyhow::Context;
use anyhow::Result;
use derive_more::FromStr;

combine_solver! {Day6, Day6Part1, Day6Part2}

pub struct Day6Part1 {
    times: Vec<i32>,
    distances: Vec<i32>,
}

pub struct Day6Part2 {
    time: i64,
    distance: i64,
}

impl FromStr for Day6Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        return Ok(Day6Part1 {
            times: lines
                .next()
                .with_context(|| format!("Invalid input: {}", s))?
                .split_whitespace()
                .skip(1)
                .map(<i32>::from_str)
                .map(|res| res.map_err(anyhow::Error::from))
                .collect::<Result<_>>()?,
            distances: lines
                .next()
                .with_context(|| format!("Invalid input: {}", s))?
                .split_whitespace()
                .skip(1)
                .map(<i32>::from_str)
                .map(|res| res.map_err(anyhow::Error::from))
                .collect::<Result<_>>()?,
        });
    }
}

impl FromStr for Day6Part2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        return Ok(Day6Part2 {
            time: lines
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
            distance: lines
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
        });
    }
}

impl ProblemSolver for Day6Part1 {
    type SolutionType = i32;
    fn solve(&self) -> Result<i32> {
        return self
            .times
            .iter()
            .zip(self.distances.iter())
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
}

impl ProblemSolver for Day6Part2 {
    type SolutionType = i64;
    fn solve(&self) -> Result<i64> {
        let (left, right) = find_time_hold_range(self.time, self.distance).with_context(|| {
            format!(
                "Failed to solve for pair time {} and distance record {}",
                self.time, self.distance
            )
        })?;
        return Ok(right - left + 1);
    }
}

fn find_time_hold_range<T: Integer>(time: T, record: T) -> Option<(T, T)> {
    let delta = time * time - (record << 2_u32);
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
    use crate::solver::{ProblemSolver, TwoPartsProblemSolver};
    use crate::utils::Result2Parts;
    use indoc::indoc;

    use anyhow::Result;
    use std::str::FromStr;

    const SAMPLE_INPUT: &str = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
    "};

    #[test]
    fn test_sample_1() -> Result<()> {
        assert_eq!(Day6::from_str(SAMPLE_INPUT)?.solve_1()?, 288);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day6::from_str(SAMPLE_INPUT)?.solve_2()?, 71503_i64);
        Ok(())
    }

    #[test]
    fn test_sample() -> anyhow::Result<()> {
        assert_eq!(
            Day6::from_str(SAMPLE_INPUT)?.solve()?,
            Result2Parts::new(288, 71503_i64)
        );

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
