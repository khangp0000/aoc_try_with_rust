use crate::solver::{share_struct_solver, ProblemSolver};
use crate::utils::get_double_newline_regex;
use crate::utils::int_trait::Integer;
use anyhow::{bail, Context};
use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use derive_more::{Deref, FromStr};
use std::cmp::min;
use std::fmt::Debug;
use std::ops::ControlFlow::{Break, Continue};
use std::rc::Rc;

share_struct_solver!(Day13, Day13Part1, Day13Part2);

#[derive(Deref, Debug)]
pub struct Day13Part1(Vec<Day13Grid>);

#[derive(Debug)]
pub struct Day13Grid {
    verticals: Vec<u32>,
    horizontals: Vec<u32>,
}

#[derive(Deref)]
pub struct Day13Part2(Rc<Day13Part1>);

impl FromStr for Day13Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let double_newline_regex = get_double_newline_regex().clone();
        let grids = double_newline_regex
            .split(s.trim_end())
            .map(|grid| {
                let (horizontals, vertical_bitvecs, _) = grid
                    .lines()
                    .map(|line| {
                        line.bytes().map(|b| match b {
                            b'.' => Ok(false),
                            b'#' => Ok(true),
                            _ => bail!("Cannot parse character {}", b as char),
                        })
                    })
                    .try_fold(
                        (Vec::new(), None, None),
                        |(mut horizontals, mut verticals, mut len), mut current_line| {
                            let current_len = current_line.len();
                            if current_len != *len.get_or_insert(current_len) {
                                bail!(
                                    "There is a mismatch horizontal length in following input:\n{}",
                                    grid
                                );
                            }
                            let mut verticals_mut_iter = verticals
                                .get_or_insert_with(|| {
                                    vec![BitVec::<u32, Msb0>::new(); current_len]
                                })
                                .iter_mut();
                            let horizontal_val = current_line
                                .try_fold(
                                    BitVec::<u32, Msb0>::with_capacity(current_len),
                                    |mut bit_vec, current_bool| {
                                        let curren_bool = current_bool?;
                                        bit_vec.push(curren_bool);
                                        verticals_mut_iter.next().unwrap().push(curren_bool);
                                        Ok::<_, anyhow::Error>(bit_vec)
                                    },
                                )?
                                .load_be::<u32>();
                            horizontals.push(horizontal_val);
                            Ok((horizontals, verticals, len))
                        },
                    )?;

                Ok(Day13Grid {
                    horizontals,
                    verticals: vertical_bitvecs
                        .unwrap_or_else(|| Vec::with_capacity(0))
                        .into_iter()
                        .map(|b| b.load_be::<u32>())
                        .collect(),
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Day13Part1(grids))
    }
}

fn find_mirror_idx<T: PartialEq>(slice: &[T]) -> Option<usize> {
    if slice.len() <= 1 {
        return None;
    }
    (1..slice.len()).find(|i| is_mirrored_at(*i, slice))
}

fn is_mirrored_at<T: PartialEq>(idx: usize, slice: &[T]) -> bool {
    let min_len = min(idx, slice.len() - idx);
    slice[idx - min_len..idx].iter().rev().eq(slice[idx..idx + min_len].iter())
}

fn find_mirror_with_1_flip_idx<T: Integer>(slice: &[T]) -> Option<usize> {
    if slice.len() <= 1 {
        return None;
    }
    (1..slice.len()).find(|i| is_mirrored_with_1_flip_at(*i, slice))
}

fn is_mirrored_with_1_flip_at<T: Integer>(idx: usize, slice: &[T]) -> bool {
    Continue(true)
        == slice[0..idx]
            .iter()
            .rev()
            .zip(slice[idx..].iter())
            .map(|(l, r)| l.bitxor(*r).count_ones())
            .try_fold(false, |have_1_mismatch, diff_count| match diff_count {
                0 => Continue(have_1_mismatch),
                1 => {
                    if have_1_mismatch {
                        Break(())
                    } else {
                        Continue(true)
                    }
                }
                _ => Break(()),
            })
}

impl ProblemSolver for Day13Part1 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        self.iter().enumerate().map(|(idx, grid)|
            find_mirror_idx(grid.verticals.as_slice()).or_else(|| find_mirror_idx(grid.horizontals.as_slice()).map(|v| v*100_usize))
                .with_context(|| format!("Cannot find mirror line for both side of grid number {} (count from 0)", idx))
        ).sum()
    }
}

impl ProblemSolver for Day13Part2 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        self.iter().enumerate().map(|(idx, grid)|
            find_mirror_with_1_flip_idx(grid.verticals.as_slice()).or_else(|| find_mirror_with_1_flip_idx(grid.horizontals.as_slice()).map(|v| v*100_usize))
                .with_context(|| format!("Cannot find mirror line for both side of grid number {} (count from 0)", idx))
        ).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day13::{Day13, Day13Part1};
    use crate::solver::{ProblemSolver, TwoPartsProblemSolver};

    use indoc::indoc;

    use std::str::FromStr;

    const SAMPLE_INPUT_1: &str = indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
    "};

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        println!("asdas {:?}", Day13Part1::from_str(SAMPLE_INPUT_1)?.solve()?);
        assert_eq!(Day13::from_str(SAMPLE_INPUT_1)?.solve_1()?, 405);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day13::from_str(SAMPLE_INPUT_1)?.solve_2()?, 400);
        Ok(())
    }
}
