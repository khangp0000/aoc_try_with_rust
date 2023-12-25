use crate::solver::{share_struct_solver, ProblemSolver};
use crate::utils::grid::grid_2d_vec::Grid2dVec;
use crate::utils::grid::{Grid2d, GridDirection};
use anyhow::Context;
use bitvec::bitvec;
use derive_more::{Deref, Display, FromStr};
use std::cell::RefCell;
use std::cmp::max;
use std::fmt::Debug;
use std::rc::Rc;

use crate::utils::graph::dfs;
use thiserror::Error;

share_struct_solver!(Day16, Day16Part1, Day16Part2);

pub struct Day16Part1 {
    grid: Grid2dVec<PositionKind>,
}

#[derive(Deref)]
pub struct Day16Part2(Rc<Day16Part1>);

#[derive(Eq, PartialEq, Copy, Clone, Debug, Display, Hash)]
enum PositionKind {
    Ground,
    VerticalSplitter,
    HorizontalSplitter,
    MirrorNWToSE,
    MirrorSWToNE,
}

impl PositionKind {
    const fn get_next_directions(
        &self,
        current_direction: GridDirection,
    ) -> &'static [GridDirection] {
        const NORTH: &[GridDirection] = &[GridDirection::North];
        const SOUTH: &[GridDirection] = &[GridDirection::South];
        const EAST: &[GridDirection] = &[GridDirection::East];
        const WEST: &[GridDirection] = &[GridDirection::West];
        const NORTH_SOUTH: &[GridDirection] = &[GridDirection::North, GridDirection::South];
        const EAST_WEST: &[GridDirection] = &[GridDirection::East, GridDirection::West];

        match self {
            PositionKind::Ground => match current_direction {
                GridDirection::North => NORTH,
                GridDirection::South => SOUTH,
                GridDirection::East => EAST,
                GridDirection::West => WEST,
                _ => unreachable!(),
            },
            PositionKind::VerticalSplitter => match current_direction {
                GridDirection::North => NORTH,
                GridDirection::South => SOUTH,
                GridDirection::East | GridDirection::West => NORTH_SOUTH,
                _ => unreachable!(),
            },
            PositionKind::HorizontalSplitter => match current_direction {
                GridDirection::North | GridDirection::South => EAST_WEST,
                GridDirection::East => EAST,
                GridDirection::West => WEST,
                _ => unreachable!(),
            },
            PositionKind::MirrorNWToSE => match current_direction {
                GridDirection::North => WEST,
                GridDirection::South => EAST,
                GridDirection::East => SOUTH,
                GridDirection::West => NORTH,
                _ => unreachable!(),
            },
            PositionKind::MirrorSWToNE => match current_direction {
                GridDirection::North => EAST,
                GridDirection::South => WEST,
                GridDirection::East => NORTH,
                GridDirection::West => SOUTH,
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cannot convert {:?} to PositionKind", <char>::from(*.0))]
    InvalidPositionChar(u8),
}

impl TryFrom<u8> for PositionKind {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(PositionKind::Ground),
            b'|' => Ok(PositionKind::VerticalSplitter),
            b'-' => Ok(PositionKind::HorizontalSplitter),
            b'\\' => Ok(PositionKind::MirrorNWToSE),
            b'/' => Ok(PositionKind::MirrorSWToNE),
            _ => Err(Error::InvalidPositionChar(value))?,
        }
    }
}

impl FromStr for Day16Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let grid = Grid2dVec::<PositionKind>::try_new(
            s.lines().map(str::bytes).map(|iter| iter.map(PositionKind::try_from)),
        )?;

        Ok(Day16Part1 { grid })
    }
}

impl Day16Part1 {
    fn find_num_energized(
        &self,
        x: usize,
        y: usize,
        starting_face: GridDirection,
    ) -> anyhow::Result<usize> {
        let visited_pos = Rc::new(RefCell::new(bitvec!(0; self.grid.height() * self.grid.width())));
        dfs(
            (x, y, starting_face),
            |current_state| {
                let (x, y, current_face) = *current_state;
                self.grid
                    .get(&x, &y)
                    .unwrap()
                    .clone()
                    .get_next_directions(current_face)
                    .iter()
                    .filter_map(move |next_face| {
                        self.grid
                            .move_from_coordinate_to_direction(&x, &y, next_face)
                            .map(|(x, y)| (x, y, *next_face))
                    })
            },
            |_, _| false,
            visited_pos.clone(),
            |visited_pos, (x, y, _)| {
                let visited_pos = visited_pos.clone();
                visited_pos.borrow_mut().set(y * self.grid.width() + x, true);
                visited_pos
            },
        );
        let res = visited_pos.borrow().count_ones();
        Ok(res)
    }
}

impl ProblemSolver for Day16Part1 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        self.find_num_energized(0, 0, GridDirection::East)
    }
}

impl ProblemSolver for Day16Part2 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        (0..self.grid.width())
            .flat_map(|x| {
                [(x, 0, GridDirection::South), (x, self.grid.height() - 1, GridDirection::North)]
            })
            .chain((0..self.grid.height()).flat_map(|y| {
                [(0, y, GridDirection::East), (self.grid.width() - 1, y, GridDirection::West)]
            }))
            .map(|(x, y, facing)| self.find_num_energized(x, y, facing))
            .try_fold(None, |max_res, val| {
                let val = val?;
                Ok::<_, anyhow::Error>(max_res.map(|curr_max| max(curr_max, val)).or(Some(val)))
            })?
            .context("Cannot find max, is the grid empty?")
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day16::Day16;
    use crate::solver::TwoPartsProblemSolver;

    use indoc::indoc;

    use std::str::FromStr;

    const SAMPLE_INPUT_1: &str = indoc! {r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
    "};

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day16::from_str(SAMPLE_INPUT_1)?.solve_1()?, 46);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day16::from_str(SAMPLE_INPUT_1)?.solve_2()?, 51);
        Ok(())
    }
}
