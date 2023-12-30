use crate::solver::{share_struct_solver, ProblemSolver};
use crate::utils::graph::dijkstra_starts_iter;
use crate::utils::grid::grid_2d_vec::Grid2dVec;
use crate::utils::grid::{Grid2d, GridDirection};
use derive_more::{Deref, FromStr};
use std::fmt::Debug;
use std::rc::Rc;
use thiserror::Error;

share_struct_solver!(Day17, Day17Part1, Day17Part2);

pub struct Day17Part1 {
    grid: Grid2dVec<u8>,
}

#[derive(Deref)]
pub struct Day17Part2(Rc<Day17Part1>);

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cannot convert {:?} to digit", <char>::from(*.0))]
    InvalidPositionChar(u8),
}

impl FromStr for Day17Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
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
        let mut res = Vec::with_capacity(3);
        let cw_90 = face.clock_wise_90();
        let ccw_90 = cw_90.reverse();
        if let Some((moved_x, moved_y)) = self.grid.move_from_coordinate_to_direction(
            *x,
            *y,
            minimum_block_move_after_turn,
            cw_90,
        ) {
            let mut curr_x = *x;
            let mut curr_y = *y;
            let weight =
                (0_usize..minimum_block_move_after_turn).fold(weight, |mut weight, _step| {
                    (curr_x, curr_y) = self
                        .grid
                        .move_from_coordinate_to_direction(curr_x, curr_y, 1, cw_90)
                        .unwrap();
                    weight += *self.grid.get(curr_x, curr_y).unwrap() as usize;
                    weight
                });
            res.push((
                (
                    moved_x,
                    moved_y,
                    cw_90,
                    max_block_straight_after_turn - minimum_block_move_after_turn,
                ),
                weight,
            ));
        }

        if let Some((moved_x, moved_y)) = self.grid.move_from_coordinate_to_direction(
            *x,
            *y,
            minimum_block_move_after_turn,
            ccw_90,
        ) {
            let mut curr_x = *x;
            let mut curr_y = *y;
            let weight =
                (0_usize..minimum_block_move_after_turn).fold(weight, |mut weight, _step| {
                    (curr_x, curr_y) = self
                        .grid
                        .move_from_coordinate_to_direction(curr_x, curr_y, 1, ccw_90)
                        .unwrap();
                    weight += *self.grid.get(curr_x, curr_y).unwrap() as usize;
                    weight
                });
            res.push((
                (
                    moved_x,
                    moved_y,
                    ccw_90,
                    max_block_straight_after_turn - minimum_block_move_after_turn,
                ),
                weight,
            ));
        }

        if *can_go_straight != 0 {
            if let Some((x, y)) = self.grid.move_from_coordinate_to_direction(*x, *y, 1, *face) {
                res.push((
                    (x, y, *face, can_go_straight - 1),
                    *self.grid.get(x, y).unwrap() as usize + weight,
                ))
            }
        }

        res
    }
}

impl ProblemSolver for Day17Part1 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
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

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
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
    use crate::solver::y2023::day17::Day17;
    use crate::solver::TwoPartsProblemSolver;

    use indoc::indoc;

    use std::str::FromStr;

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
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day17::from_str(SAMPLE_INPUT_1)?.solve_1()?, 102);
        assert_eq!(Day17::from_str(SAMPLE_INPUT_1)?.solve_2()?, 94);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day17::from_str(SAMPLE_INPUT_2)?.solve_2()?, 71);
        Ok(())
    }
}
