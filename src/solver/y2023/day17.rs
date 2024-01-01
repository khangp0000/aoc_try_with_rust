use std::fmt::Debug;
use std::rc::Rc;

use anyhow::Result;
use derive_more::{Deref, FromStr};
use itertools::Itertools;
use thiserror::Error;

use crate::solver::{share_struct_solver, ProblemSolver};
use crate::utils::graph::dijkstra_starts_iter;
use crate::utils::grid::grid_2d_vec::Grid2dVec;
use crate::utils::grid::{Grid2d, GridDirection};

share_struct_solver!(Day17, Day17Part1, Day17Part2);

pub struct Day17Part1 {
    grid: Grid2dVec<u8>,
}

#[derive(Deref)]
pub struct Day17Part2(Rc<Day17Part1>);

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cannot convert {:?} to digit", < char >::from(*.0))]
    InvalidPositionChar(u8),
}

impl FromStr for Day17Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let grid = Grid2dVec::<u8>::try_new(s.lines().map(str::bytes).map(|iter| {
            iter.map(|b| match b {
                b'1'..=b'9' => Ok(b - b'0'),
                _ => Err(Error::InvalidPositionChar(b))?,
            })
        }))?;

        Ok(Day17Part1 { grid })
    }
}

impl Day17Part1 {
    fn get_neighbor(
        &self,
        state: &(usize, usize, GridDirection, usize),
        weight: usize,
        minimum_block_move_after_turn: usize,
        max_block_straight_after_turn: usize,
    ) -> Vec<((usize, usize, GridDirection, usize), usize)> {
        let (x, y, face, can_go_straight) = state;
        let cw_90 = face.clock_wise_90();
        let ccw_90 = cw_90.reverse();

        let neighbor_iter = [cw_90, ccw_90]
            .into_iter()
            .filter_map(|dir| {
                self.grid
                    .move_from_coordinate_to_direction(*x, *y, minimum_block_move_after_turn, dir)
                    .map(|(x, y)| (x, y, dir))
            })
            .map(|(moved_x, moved_y, dir)| {
                let (weight, _, _) = (0_usize..minimum_block_move_after_turn).fold(
                    (weight, *x, *y),
                    |(mut weight, x, y), _step| {
                        let (x, y) =
                            self.grid.move_from_coordinate_to_direction(x, y, 1, dir).unwrap();
                        weight += self.grid[(x, y)] as usize;
                        (weight, x, y)
                    },
                );

                (
                    (
                        moved_x,
                        moved_y,
                        dir,
                        max_block_straight_after_turn - minimum_block_move_after_turn,
                    ),
                    weight,
                )
            });

        if *can_go_straight != 0 {
            self.grid
                .move_from_coordinate_to_direction(*x, *y, 1, *face)
                .map(|(x, y)| {
                    ((x, y, *face, can_go_straight - 1), self.grid[(x, y)] as usize + weight)
                })
                .into_iter()
                .chain(neighbor_iter)
                .collect_vec()
        } else {
            neighbor_iter.collect_vec()
        }
    }
}

impl ProblemSolver for Day17Part1 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        let starts = [
            ((0_usize, 0_usize, GridDirection::West, 0_usize), 0),
            ((0_usize, 0_usize, GridDirection::North, 0_usize), 0),
        ];
        if let Some((_, _, weight)) = dijkstra_starts_iter(
            starts,
            |state, weight| self.get_neighbor(state, *weight, 1, 3),
            |_, (x, y, _, _), _| *x == self.grid.width() - 1 && *y == self.grid.height() - 1,
            (),
            |_, _, _| (),
        ) {
            return Ok(weight);
        }

        unreachable!()
    }
}

impl ProblemSolver for Day17Part2 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        let starts = [
            ((0_usize, 0_usize, GridDirection::West, 0_usize), 0),
            ((0_usize, 0_usize, GridDirection::North, 0_usize), 0),
        ];
        if let Some((_, _, weight)) = dijkstra_starts_iter(
            starts,
            |state, weight| self.get_neighbor(state, *weight, 4, 10),
            |_, (x, y, _, _), _| *x == self.grid.width() - 1 && *y == self.grid.height() - 1,
            (),
            |_, _, _| (),
        ) {
            return Ok(weight);
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Result;
    use indoc::indoc;

    use crate::solver::y2023::day17::Day17;
    use crate::solver::TwoPartsProblemSolver;

    const SAMPLE_INPUT_1: &str = indoc! {r"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
    "};

    const SAMPLE_INPUT_2: &str = indoc! {r"
            111111111111
            999999999991
            999999999991
            999999999991
            999999999991
    "};

    #[test]
    fn test_sample_1() -> Result<()> {
        assert_eq!(Day17::from_str(SAMPLE_INPUT_1)?.solve_1()?, 102);
        assert_eq!(Day17::from_str(SAMPLE_INPUT_1)?.solve_2()?, 94);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> Result<()> {
        assert_eq!(Day17::from_str(SAMPLE_INPUT_2)?.solve_2()?, 71);
        Ok(())
    }
}
