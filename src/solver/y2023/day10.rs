use crate::solver::{share_struct_solver, ProblemSolver};
use crate::utils::graph::dfs;
use crate::utils::grid::grid_2d_vec::Grid2dVec;
use crate::utils::grid::{Grid2d, GridDirection};
use anyhow::{anyhow, bail, Context};
use derive_more::{Deref, DerefMut, Display, FromStr};
use enumset::{enum_set, EnumSet};
use std::cell::OnceCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::rc::{Rc, Weak};
use std::sync::Arc;
use thiserror::Error;

share_struct_solver!(Day10, Day10Part1, Day10Part2);

pub struct Day10Part1 {
    grid: Grid2dVec<PositionKind>,
    start: (usize, usize),
    pipe_path: OnceCell<Result<ChainPathRc, Arc<anyhow::Error>>>,
}

#[derive(Deref)]
pub struct Day10Part2(Rc<Day10Part1>);

const CARDINAL: &[GridDirection; 4] =
    &[GridDirection::North, GridDirection::South, GridDirection::East, GridDirection::West];

const HORIZONTAL_PIPE: &Pipe = &Pipe::new(enum_set!(GridDirection::West | GridDirection::East));

const VERTICAL_PIPE: &Pipe = &Pipe::new(enum_set!(GridDirection::South | GridDirection::North));

const L_PIPE_NORTH_EAST: &Pipe = &Pipe::new(enum_set!(GridDirection::North | GridDirection::East));

const L_PIPE_NORTH_WEST: &Pipe = &Pipe::new(enum_set!(GridDirection::North | GridDirection::West));

const L_PIPE_SOUTH_WEST: &Pipe = &Pipe::new(enum_set!(GridDirection::South | GridDirection::West));

const L_PIPE_SOUTH_EAST: &Pipe = &Pipe::new(enum_set!(GridDirection::South | GridDirection::East));

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
    const fn new(direction_set: EnumSet<GridDirection>) -> Self {
        Self { entrances: direction_set }
    }
}

impl Pipe {
    pub fn can_enter_from(&self, direction: GridDirection) -> Option<EnumSet<GridDirection>> {
        if self.entrances.contains(direction) { Some(self.entrances & !direction) } else { None }
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
        let starting_position = OnceCell::default();
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
            pipe_path: OnceCell::default(),
        })
    }
}

impl ProblemSolver for Day10Part1 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        let res = self.get_pipe_path();
        res.clone().map(|path| path.len() / 2).map_err(|e| anyhow!(e))
    }
}

#[derive(Clone, Debug)]
struct ChainPath {
    prev: Option<ChainPathRc>,
    position_and_facing: ((usize, usize), GridDirection),
    enter_direction: Option<GridDirection>,
    start: Weak<ChainPath>,
    len: usize,
}

#[derive(Deref, DerefMut, Clone, Debug, Display)]
struct ChainPathRc(Rc<ChainPath>);

impl ChainPathRc {
    fn start(position_and_facing: ((usize, usize), GridDirection)) -> ChainPathRc {
        ChainPathRc(Rc::new_cyclic(|me| ChainPath {
            prev: None,
            position_and_facing,
            enter_direction: None,
            start: me.clone(),
            len: 1,
        }))
    }
}

trait ChainPathTrait {
    fn push(&self, item: ((usize, usize), GridDirection)) -> ChainPathRc;
    fn len(&self) -> usize;
}

impl Display for ChainPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.prev {
            None => write!(f, "{:?}", self.position_and_facing),
            Some(prev_chain) => {
                write!(f, "{}->{:?}", prev_chain.as_ref(), self.position_and_facing)
            }
        }
    }
}

impl ChainPathTrait for ChainPathRc {
    fn push(&self, position_and_facing: ((usize, usize), GridDirection)) -> ChainPathRc {
        ChainPathRc(Rc::new(ChainPath {
            prev: Some(self.clone()),
            position_and_facing,
            enter_direction: Some(self.position_and_facing.1.reverse()),
            start: self.start.clone(),
            len: self.len + 1,
        }))
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl IntoIterator for ChainPathRc {
    type Item = ((usize, usize), (Option<GridDirection>, GridDirection));
    type IntoIter = ChainPathIter;

    fn into_iter(self) -> Self::IntoIter {
        ChainPathIter { current: Some(self) }
    }
}

struct ChainPathIter {
    current: Option<ChainPathRc>,
}

impl Iterator for ChainPathIter {
    type Item = ((usize, usize), (Option<GridDirection>, GridDirection));

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            None => None,
            Some(chain) => {
                let (pos, exit) = chain.position_and_facing;
                let enter = chain.enter_direction;
                self.current = chain.prev.clone();
                Some((pos, (enter, exit)))
            }
        }
    }
}

impl Day10Part1 {
    fn get_pipe_path(&self) -> Result<ChainPathRc, Arc<anyhow::Error>> {
        self.pipe_path.get_or_init(|| self.find_pipe_loop().map_err(Arc::new)).clone()
    }

    fn find_pipe_loop(&self) -> anyhow::Result<ChainPathRc> {
        let result =
            CARDINAL.iter().map(|direction| (self.start, *direction)).find_map(|start_state| {
                dfs(
                    start_state,
                    move |((prev_x, prev_y), prev_state_face)| {
                        let prev_x = *prev_x;
                        let prev_y = *prev_y;
                        let prev_state_face = *prev_state_face;
                        self.grid
                            .move_from_coordinate_to_direction(prev_x, prev_y, 1, prev_state_face)
                            .into_iter()
                            .flat_map(move |(x, y)| {
                                self.grid.get(x, y).into_iter().flat_map(move |p| {
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
                            == self.grid.move_from_coordinate_to_direction(*x, *y, 1, *facing)
                    },
                    None,
                    |path, next_coordinate_and_facing| {
                        Some(path.as_ref().map_or_else(
                            || ChainPathRc::start(*next_coordinate_and_facing),
                            |path: &ChainPathRc| path.push(*next_coordinate_and_facing),
                        ))
                    },
                )
            });
        match result {
            None => bail!("Cannot find a path loop back to start"),
            Some((path, _)) => Ok(path.unwrap()),
        }
    }
}

impl ProblemSolver for Day10Part2 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        let chain_path = self.get_pipe_path().map_err(|e| anyhow!(e))?;

        let start_enter = chain_path.position_and_facing.1.reverse();
        let start_exit = chain_path.start.clone().upgrade().unwrap().position_and_facing.1;
        let start_pipe = match (start_enter, start_exit) {
            (GridDirection::North, GridDirection::South) => PipeKind::Vertical,
            (GridDirection::North, GridDirection::East) => PipeKind::LNorthEast,
            (GridDirection::North, GridDirection::West) => PipeKind::LNorthWest,
            (GridDirection::South, GridDirection::North) => PipeKind::Vertical,
            (GridDirection::South, GridDirection::West) => PipeKind::LSouthWest,
            (GridDirection::South, GridDirection::East) => PipeKind::LSouthEast,
            (GridDirection::East, GridDirection::North) => PipeKind::LNorthEast,
            (GridDirection::East, GridDirection::West) => PipeKind::Horizontal,
            (GridDirection::East, GridDirection::South) => PipeKind::LSouthEast,
            (GridDirection::West, GridDirection::North) => PipeKind::LNorthWest,
            (GridDirection::West, GridDirection::East) => PipeKind::Horizontal,
            (GridDirection::West, GridDirection::South) => PipeKind::LSouthWest,
            (_, _) => unreachable!(),
        };
        let path_hash_map: HashMap<_, _> = chain_path
            .into_iter()
            .map(|(pos, (enter, exit))| (pos, (enter.unwrap_or(start_enter), exit)))
            .collect();

        let grid = &self.grid.map_out_place(|x, y, t| {
            if path_hash_map.contains_key(&(x, y)) {
                if PositionKind::Start == *t { PositionKind::Pipe(start_pipe) } else { *t }
            } else {
                PositionKind::Ground
            }
        });

        Ok(grid
            .rows()
            .map(|row| {
                row.iter().fold(
                    (false, false, 0_usize),
                    |(mut is_inside, mut is_from_south, mut count_inside), position_kind| {
                        match position_kind {
                            PositionKind::Start => unreachable!(),
                            PositionKind::Ground => {
                                if is_inside {
                                    count_inside += 1;
                                }
                            }
                            PositionKind::Pipe(pipe_kind) => match pipe_kind {
                                PipeKind::Horizontal => {}
                                PipeKind::LNorthEast => is_from_south = false,
                                PipeKind::LSouthEast => is_from_south = true,
                                PipeKind::LNorthWest => {
                                    if is_from_south {
                                        is_inside = !is_inside
                                    }
                                }
                                PipeKind::LSouthWest => {
                                    if !is_from_south {
                                        is_inside = !is_inside
                                    }
                                }
                                PipeKind::Vertical => is_inside = !is_inside,
                            },
                        };
                        (is_inside, is_from_south, count_inside)
                    },
                )
            })
            .map(|(_, _, count_inside)| count_inside)
            .sum())
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
