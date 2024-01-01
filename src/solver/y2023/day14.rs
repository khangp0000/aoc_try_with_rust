use std::cell::OnceCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::ControlFlow::{Break, Continue};
use std::rc::Rc;

use anyhow::bail;
use anyhow::Result;
use bitvec::bitvec;
use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use derive_more::{Deref, Display, FromStr};
use indexmap::IndexSet;
use itertools::Itertools;

use crate::solver::{share_struct_solver, ProblemSolver};

share_struct_solver!(Day14, Day14Part1, Day14Part2);

#[derive(Display, Deref, Debug)]
pub struct Day14Part1(WeirdGrid);

#[derive(Clone, Debug)]
pub struct WeirdGrid {
    width: u8,
    height: u8,
    cube_y_inc_x_inc: Rc<Vec<(u8, u8)>>,
    cube_y_dec_x_dec: Rc<OnceCell<Vec<(u8, u8)>>>,
    cube_x_inc_y_inc: Rc<OnceCell<Vec<(u8, u8)>>>,
    cube_x_dec_y_dec: Rc<OnceCell<Vec<(u8, u8)>>>,
    rounds: Rc<BitVec>,
}

impl FromStr for WeirdGrid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut cube_y_inc_x_inc = Vec::default();
        let (rounds, width, height) = s.lines().map(|line| line.bytes()).enumerate().try_fold(
            (BitVec::<usize, Lsb0>::with_capacity(s.len()), None, 0_u8),
            |(mut bitvec, mut len, height), (y, line_bytes)| {
                let current_len = line_bytes.len();
                if current_len != *len.get_or_insert(current_len) {
                    bail!("There is a mismatch horizontal length in following input:\n{}", s);
                }

                bitvec.extend(
                    line_bytes
                        .enumerate()
                        .map(|(x, b)| match b {
                            b'.' => Ok(false),
                            b'#' => {
                                cube_y_inc_x_inc.push((x as u8, y as u8));
                                Ok(false)
                            }
                            b'O' => Ok(true),
                            _ => bail!("Cannot parse character {}", b as char),
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                );

                Ok((bitvec, len, height + 1))
            },
        )?;

        let width = width.unwrap_or(0) as u8;
        Ok(WeirdGrid::new(width, height, cube_y_inc_x_inc, rounds))
    }
}

impl Display for WeirdGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string_chars =
            self.rounds.iter().map(|b| if *b { b'O' } else { b'.' }).collect::<Vec<u8>>();
        self.cube_y_inc_x_inc.iter().for_each(|(x, y)| {
            string_chars[self.width as usize * (*y as usize) + (*x as usize)] = b'#'
        });
        write!(
            f,
            "{}",
            Itertools::intersperse(
                string_chars.chunks(self.width as usize).map(std::str::from_utf8),
                Ok("\n"),
            )
            .collect::<Result<String, _>>()
            .map_err(|_| std::fmt::Error)?
        )
    }
}

impl WeirdGrid {
    fn new(width: u8, height: u8, cube_y_inc_x_inc: Vec<(u8, u8)>, rounds: BitVec) -> Self {
        WeirdGrid {
            width,
            height,
            cube_y_inc_x_inc: Rc::new(cube_y_inc_x_inc),
            cube_y_dec_x_dec: Rc::new(OnceCell::default()),
            cube_x_inc_y_inc: Rc::new(OnceCell::default()),
            cube_x_dec_y_dec: Rc::new(OnceCell::default()),
            rounds: Rc::new(rounds),
        }
    }

    fn clone_with_new_rounds(&self, rounds: BitVec) -> Self {
        WeirdGrid {
            width: self.width,
            height: self.height,
            cube_y_inc_x_inc: self.cube_y_inc_x_inc.clone(),
            cube_y_dec_x_dec: self.cube_y_dec_x_dec.clone(),
            cube_x_inc_y_inc: self.cube_x_inc_y_inc.clone(),
            cube_x_dec_y_dec: self.cube_x_dec_y_dec.clone(),
            rounds: Rc::new(rounds),
        }
    }

    fn tilt_north(&self) -> Self {
        let cube_x_dec_y_dec = self
            .cube_x_inc_y_inc
            .get_or_init(|| {
                let mut vec = Vec::from_iter(self.cube_y_inc_x_inc.iter().copied());
                vec.sort_unstable_by(|(xl, yl), (xr, yr)| xl.cmp(xr).then_with(|| yl.cmp(yr)));
                vec
            })
            .iter()
            .copied()
            .rev()
            .collect::<Vec<_>>();

        let mut moved_bit_vec = bitvec!(0; self.rounds.len());

        let count_map = self
            .rounds
            .iter_ones()
            .map(|idx| (idx as u16 % self.width as u16, idx as u16 / self.width as u16))
            .map(|(x, y)| {
                let x = x as u8;
                let y = y as u8;

                cube_x_dec_y_dec
                    .get(cube_x_dec_y_dec.partition_point(|&cube_pos| cube_pos > (x, y)))
                    .map_or(
                        (x, 0),
                        |(cube_x, cube_y)| if *cube_x == x { (x, *cube_y + 1) } else { (x, 0) },
                    )
            })
            .fold(HashMap::new(), |mut count_map, pos| {
                count_map.entry(pos).and_modify(|v| *v += 1_u8).or_insert(1_u8);
                count_map
            });

        count_map
            .into_iter()
            .flat_map(|((x, first_round_y), count)| {
                (first_round_y..first_round_y + count)
                    .map(move |y| y as usize * self.width as usize + x as usize)
            })
            .for_each(|idx| moved_bit_vec.set(idx, true));

        self.clone_with_new_rounds(moved_bit_vec)
    }

    fn tilt_south(&self) -> Self {
        let cube_x_inc_y_inc = self.cube_x_inc_y_inc.get_or_init(|| {
            let mut vec = Vec::from_iter(self.cube_y_inc_x_inc.iter().copied());
            vec.sort_unstable_by(|(xl, yl), (xr, yr)| xl.cmp(xr).then_with(|| yl.cmp(yr)));
            vec
        });

        let mut moved_bit_vec = bitvec!(0; self.rounds.len());

        self.rounds
            .iter_ones()
            .map(|idx| (idx as u16 % self.width as u16, idx as u16 / self.width as u16))
            .map(|(x, y)| {
                let x = x as u8;
                let y = y as u8;

                cube_x_inc_y_inc
                    .get(cube_x_inc_y_inc.partition_point(|&cube_pos| cube_pos < (x, y)))
                    .map_or((x, self.height), |(cube_x, cube_y)| {
                        if *cube_x == x { (x, *cube_y) } else { (x, self.height) }
                    })
            })
            .fold(HashMap::new(), |mut count_map, pos| {
                count_map.entry(pos).and_modify(|v| *v += 1_u8).or_insert(1_u8);
                count_map
            })
            .into_iter()
            .flat_map(|((x, first_round_y), count)| {
                (first_round_y - count..first_round_y)
                    .map(move |y| y as usize * self.width as usize + x as usize)
            })
            .for_each(|idx| moved_bit_vec.set(idx, true));

        self.clone_with_new_rounds(moved_bit_vec)
    }

    fn tilt_west(&self) -> Self {
        let cube_y_dec_x_dec = self.cube_y_inc_x_inc.iter().copied().rev().collect::<Vec<_>>();

        let mut moved_bit_vec = bitvec!(0; self.rounds.len());

        self.rounds
            .iter_ones()
            .map(|idx| (idx as u16 % self.width as u16, idx as u16 / self.width as u16))
            .map(|(x, y)| {
                let x = x as u8;
                let y = y as u8;

                cube_y_dec_x_dec
                    .get(
                        cube_y_dec_x_dec
                            .partition_point(|(cube_x, cube_y)| (*cube_y, *cube_x) > (y, x)),
                    )
                    .map_or(
                        (0, y),
                        |(cube_x, cube_y)| if *cube_y == y { (*cube_x + 1, y) } else { (0, y) },
                    )
            })
            .fold(HashMap::new(), |mut count_map, pos| {
                count_map.entry(pos).and_modify(|v| *v += 1_u8).or_insert(1_u8);
                count_map
            })
            .into_iter()
            .flat_map(|((first_round_x, y), count)| {
                (first_round_x..first_round_x + count)
                    .map(move |x| y as usize * self.width as usize + x as usize)
            })
            .for_each(|idx| moved_bit_vec.set(idx, true));

        self.clone_with_new_rounds(moved_bit_vec)
    }

    fn tilt_east(&self) -> Self {
        let cube_y_inc_x_inc = self.cube_y_inc_x_inc.as_ref();

        let mut moved_bit_vec = bitvec!(0; self.rounds.len());
        self.rounds
            .iter_ones()
            .map(|idx| (idx as u16 % self.width as u16, idx as u16 / self.width as u16))
            .map(|(x, y)| {
                let x = x as u8;
                let y = y as u8;

                cube_y_inc_x_inc
                    .get(
                        cube_y_inc_x_inc
                            .partition_point(|(cube_x, cube_y)| (*cube_y, *cube_x) < (y, x)),
                    )
                    .map_or((self.width, y), |(cube_x, cube_y)| {
                        if *cube_y == y { (*cube_x, y) } else { (self.width, y) }
                    })
            })
            .fold(HashMap::new(), |mut count_map, pos| {
                count_map.entry(pos).and_modify(|v| *v += 1_u8).or_insert(1_u8);
                count_map
            })
            .into_iter()
            .flat_map(|((first_round_x, y), count)| {
                (first_round_x - count..first_round_x)
                    .map(move |x| y as usize * self.width as usize + x as usize)
            })
            .for_each(|idx| moved_bit_vec.set(idx, true));

        self.clone_with_new_rounds(moved_bit_vec)
    }

    fn tilt_cycle(&self) -> Self {
        self.tilt_north().tilt_west().tilt_south().tilt_east()
    }
}

#[derive(Deref)]
pub struct Day14Part2(Rc<Day14Part1>);

impl FromStr for Day14Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Day14Part1(WeirdGrid::from_str(s)?))
    }
}

impl ProblemSolver for Day14Part1 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        Ok(self
            .deref()
            .clone()
            .tilt_north()
            .rounds
            .chunks(self.width as usize)
            .map(|line| line.count_ones())
            .enumerate()
            .map(|(idx, round_num_on_line)| (self.height as usize - idx) * round_num_on_line)
            .sum())
    }
}

impl ProblemSolver for Day14Part2 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        let mut processed_state = IndexSet::new();
        let current = self.tilt_cycle();
        processed_state.insert(current.rounds.clone());
        let run_status = (1..1000000000).try_fold(current, |mut current, _| {
            current = current.tilt_cycle();
            if let (idx, false) = processed_state.insert_full(current.rounds.clone()) {
                let cycle_len = processed_state.len() - idx;
                let value_idx = idx + ((999999999_usize - idx) % cycle_len);
                let value = processed_state
                    .get_index(value_idx)
                    .unwrap()
                    .chunks(self.width as usize)
                    .map(|line| line.count_ones())
                    .enumerate()
                    .map(|(idx, round_num_on_line)| {
                        (self.height as usize - idx) * round_num_on_line
                    })
                    .sum();
                return Break(value);
            }
            Continue(current)
        });
        if let Break(value) = run_status {
            return Ok(value);
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Result;
    use indoc::indoc;

    use crate::solver::y2023::day14::Day14;
    use crate::solver::TwoPartsProblemSolver;

    const SAMPLE_INPUT_1: &str = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
    "};

    #[test]
    fn test_sample_1() -> Result<()> {
        assert_eq!(Day14::from_str(SAMPLE_INPUT_1)?.solve_1()?, 136);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> Result<()> {
        assert_eq!(Day14::from_str(SAMPLE_INPUT_1)?.solve_2()?, 64);
        Ok(())
    }
}
