use crate::solver::{share_struct_solver, ProblemSolver};
use crate::utils::graph::{dfs, dfs_full};
use crate::utils::grid::grid_2d_vec::Grid2dVec;
use crate::utils::grid::{Grid2d, GridDirection};
use anyhow::{bail, Context};
use derive_more::{Deref, Display, FromStr};
use enumset::{enum_set, EnumSet};
use std::cell::{OnceCell, RefCell};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::rc::Rc;
use thiserror::Error;

share_struct_solver!(Day10, Day10Part1, Day10Part2);

pub struct Day10Part1 {
    grid: Grid2dVec<PositionKind>,
    start: (usize, usize),
}

#[derive(Deref)]
pub struct Day10Part2(Rc<Day10Part1>);

const CARDINAL: &[GridDirection; 4] =
    &[GridDirection::North, GridDirection::South, GridDirection::East, GridDirection::West];

const HORIZONTAL_PIPE: &Pipe =
    &Pipe { entrances: enum_set!(GridDirection::West | GridDirection::East) };

const VERTICAL_PIPE: &Pipe =
    &Pipe { entrances: enum_set!(GridDirection::South | GridDirection::North) };

const L_PIPE_NORTH_EAST: &Pipe =
    &Pipe { entrances: enum_set!(GridDirection::North | GridDirection::East) };

const L_PIPE_NORTH_WEST: &Pipe =
    &Pipe { entrances: enum_set!(GridDirection::North | GridDirection::West) };

const L_PIPE_SOUTH_WEST: &Pipe =
    &Pipe { entrances: enum_set!(GridDirection::South | GridDirection::West) };

const L_PIPE_SOUTH_EAST: &Pipe =
    &Pipe { entrances: enum_set!(GridDirection::South | GridDirection::East) };

#[derive(Eq, PartialEq, Copy, Clone, Debug, Display, Hash)]
enum PipeKind {
    Horizontal,
    Vertical,
    LNorthEast,
    LNorthWest,
    LSouthWest,
    LSouthEast,
}
impl Deref for PipeKind {
    type Target = Pipe;

    fn deref(&self) -> &Self::Target {
        match self {
            PipeKind::Horizontal => HORIZONTAL_PIPE,
            PipeKind::Vertical => VERTICAL_PIPE,
            PipeKind::LNorthEast => L_PIPE_NORTH_EAST,
            PipeKind::LNorthWest => L_PIPE_NORTH_WEST,
            PipeKind::LSouthWest => L_PIPE_SOUTH_WEST,
            PipeKind::LSouthEast => L_PIPE_SOUTH_EAST,
        }
    }
}

struct Pipe {
    entrances: EnumSet<GridDirection>,
}

impl Pipe {
    pub fn can_enter_from(&self, direction: GridDirection) -> Option<EnumSet<GridDirection>> {
        if self.entrances.contains(direction) {
            Some(self.entrances & !direction)
        } else {
            None
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Display, Hash)]
enum PositionKind {
    Start,
    Ground,
    Pipe(PipeKind),
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
            b'S' => Ok(PositionKind::Start),
            b'|' => Ok(PositionKind::Pipe(PipeKind::Vertical)),
            b'-' => Ok(PositionKind::Pipe(PipeKind::Horizontal)),
            b'L' => Ok(PositionKind::Pipe(PipeKind::LNorthEast)),
            b'J' => Ok(PositionKind::Pipe(PipeKind::LNorthWest)),
            b'7' => Ok(PositionKind::Pipe(PipeKind::LSouthWest)),
            b'F' => Ok(PositionKind::Pipe(PipeKind::LSouthEast)),
            _ => Err(Error::InvalidPositionChar(value))?,
        }
    }
}

impl FromStr for Day10Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let starting_position = OnceCell::new();
        let grid = Grid2dVec::<PositionKind>::try_new(s.lines().map(str::bytes).enumerate().map(
            |(y, iter)| {
                let starting_position = &starting_position;
                iter.enumerate().map(move |(x, b)| {
                    let position_kind_res = PositionKind::try_from(b);
                    match position_kind_res {
                        Ok(position_kind) => match position_kind {
                            PositionKind::Start => {
                                if let Err(existing_start) = starting_position.set((x, y)) {
                                    bail!(
                                        "Found 2 starting pos {:?} and {:?}",
                                        existing_start,
                                        (x, y)
                                    )
                                }
                                Ok(PositionKind::Start)
                            }
                            PositionKind::Ground => Ok(PositionKind::Ground),
                            PositionKind::Pipe(p) => Ok(PositionKind::Pipe(p)),
                        },
                        Err(e) => Err(e),
                    }
                })
            },
        ))?;

        Ok(Day10Part1 {
            grid,
            start: starting_position.into_inner().context("Cannot find starting position")?,
        })
    }
}

impl ProblemSolver for Day10Part1 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        Ok(self.find_pipe_loop()?.0.len() / 2)
    }
}

impl Day10Part1 {
    fn find_pipe_loop(
        &self,
    ) -> anyhow::Result<Rc<(HashMap<(usize, usize), GridDirection>, GridDirection)>> {
        let result =
            CARDINAL.iter().map(|direction| (self.start, *direction)).find_map(|start_state| {
                dfs(
                    start_state,
                    move |((prev_x, prev_y), prev_state_face)| {
                        let prev_x = *prev_x;
                        let prev_y = *prev_y;
                        let prev_state_face = *prev_state_face;
                        self.grid
                            .move_from_coordinate_to_direction(&prev_x, &prev_y, &prev_state_face)
                            .into_iter()
                            .flat_map(move |(x, y)| {
                                self.grid.get(&x, &y).into_iter().flat_map(move |p| {
                                    let iter: Box<
                                        dyn Iterator<Item = ((usize, usize), GridDirection)>,
                                    > = match p {
                                        PositionKind::Start => {
                                            Box::new(std::iter::once(start_state))
                                        }
                                        PositionKind::Ground => Box::new(std::iter::empty()),
                                        PositionKind::Pipe(pipe_kind) => Box::new(
                                            pipe_kind
                                                .can_enter_from(prev_state_face.reverse())
                                                .into_iter()
                                                .flat_map(move |out_directions| {
                                                    out_directions.iter().map(
                                                        move |out_direction| {
                                                            ((x, y), out_direction)
                                                        },
                                                    )
                                                }),
                                        ),
                                    };

                                    iter
                                })
                            })
                    },
                    |_, ((x, y), facing)| {
                        Some(self.start)
                            == self.grid.move_from_coordinate_to_direction(x, y, facing)
                    },
                    Rc::new((HashMap::new(), start_state.1)),
                    |path, (coordinate, facing)| {
                        let (mut path, _) = path.as_ref().clone();
                        path.insert(*coordinate, *facing);
                        Rc::new((path, *facing))
                    },
                )
            });
        match result {
            None => bail!("Cannot find a path loop back to start"),
            Some((path, _)) => Ok(path),
        }
    }
}

impl ProblemSolver for Day10Part2 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        let (loop_pipe, last_face) = Rc::try_unwrap(self.find_pipe_loop()?).unwrap();
        let grid = &self.grid.map_out_place(|x, y, t| {
            if loop_pipe.contains_key(&(x, y)) { *t } else { PositionKind::Ground }
        });

        let start_entrance = last_face.reverse();
        let (clock_wise, counter_clock_wise) = loop_pipe.iter().try_fold(
            (RefCell::new(Some(Vec::new())), RefCell::new(Some(Vec::new()))),
            |(mut clock_wise, counter_clock_wise), ((x, y), facing)| {
                let mut non_facing = *facing;
                let mut swapped = false;
                let acc = Rc::new(());
                let entrance = match grid[(*x, *y)] {
                    PositionKind::Start => start_entrance,
                    PositionKind::Ground => unreachable!(),
                    PositionKind::Pipe(pipe_kind) => pipe_kind
                        .can_enter_from(*facing)
                        .unwrap()
                        .into_iter()
                        .try_fold(None, |opt, direction| {
                            if opt.is_none() {
                                Ok(Some(direction))
                            } else {
                                bail!("There can't be two entrance")
                            }
                        })?
                        .context("There can't be zero entrance")?,
                };

                (0..3).try_for_each(|_| {
                    non_facing = non_facing.clock_wise_90();
                    if non_facing == entrance {
                        if swapped {
                            bail!("Swapped twice, should never happens in valid input");
                        }
                        swapped = true;
                        clock_wise.swap(&counter_clock_wise);
                    }
                    match grid.move_from_coordinate_to_direction(x, y, &non_facing) {
                        None => {
                            clock_wise.replace(None);
                        }
                        Some(pos) => {
                            match grid[pos] {
                                PositionKind::Ground => match clock_wise.get_mut() {
                                    Some(vec) => {
                                        vec.push((acc.clone(), pos));
                                    }
                                    _ => (),
                                },
                                _ => (),
                            };
                        }
                    }
                    Ok(())
                })?;

                Ok::<_, anyhow::Error>((counter_clock_wise, clock_wise))
            },
        )?;

        let clock_wise = clock_wise.into_inner();
        if let Some(value) = clock_wise.and_then(|mut work_stack| {
            let mut visited = HashSet::new();
            if dfs_full(
                &mut work_stack,
                &mut visited,
                |(x, y)| {
                    let x = *x;
                    let y = *y;
                    CARDINAL
                        .iter()
                        .filter_map(move |direction| {
                            grid.move_from_coordinate_to_direction(&x, &y, direction)
                        })
                        .filter(|(x, y)| grid[(*x, *y)] == PositionKind::Ground)
                },
                |_, (x, y)| *x == 0 || *y == 0 || *x == grid.width() - 1 || *y == grid.width() - 1,
                |acc, _| acc.clone(),
            )
            .is_none()
            {
                Some(visited.len())
            } else {
                None
            }
        }) {
            return Ok(value);
        }

        let counter_clock_wise = counter_clock_wise.into_inner();

        if let Some(value) = counter_clock_wise.and_then(|mut work_stack| {
            let mut visited = HashSet::new();
            if dfs_full(
                &mut work_stack,
                &mut visited,
                |(x, y)| {
                    let x = *x;
                    let y = *y;
                    CARDINAL
                        .iter()
                        .filter_map(move |direction| {
                            grid.move_from_coordinate_to_direction(&x, &y, direction)
                        })
                        .filter(|(x, y)| grid[(*x, *y)] == PositionKind::Ground)
                },
                |_, (x, y)| *x == 0 || *y == 0 || *x == grid.width() - 1 || *y == grid.width() - 1,
                |acc, _| acc.clone(),
            )
            .is_none()
            {
                Some(visited.len())
            } else {
                None
            }
        }) {
            return Ok(value);
        }
        bail!("Somehow both side can reach the edge ¯\\_(ツ)_/¯")
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day10::Day10;
    use crate::solver::TwoPartsProblemSolver;

    use indoc::indoc;

    use std::str::FromStr;

    const SAMPLE_INPUT_1: &str = indoc! {"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
    "};

    const SAMPLE_INPUT_2: &str = indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
    "};

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day10::from_str(SAMPLE_INPUT_1)?.solve_1()?, 8);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day10::from_str(SAMPLE_INPUT_2)?.solve_2()?, 10);
        Ok(())
    }
}
