use crate::solver::{share_struct_solver, ProblemSolver};
use crate::utils::WarningResult;
use anyhow::{anyhow, bail, ensure, Context};
use bitvec::bitvec;
use bitvec::vec::BitVec;
use derive_more::{Deref, FromStr};
use derive_new::new;
use std::cell::OnceCell;

use num::Integer;

use std::fmt::{Debug, Display, Formatter};

use crate::utils::graph::bfs;
use crate::utils::grid::grid_2d_bitvec::Grid2dBitVec;
use crate::utils::grid::{Grid2d, GridDirection};
use itertools::Itertools;
use std::rc::Rc;

share_struct_solver!(Day21, Day21Part1, Day21Part2);

#[derive(new, Debug)]
pub struct Day21Part1 {
    start: (usize, usize),
    grid: Grid2dBitVec,
}

type Blocked = bool;
type IsStart = bool;

fn parse_position(c: char) -> anyhow::Result<(Blocked, IsStart)> {
    match c {
        '#' => Ok((true, false)),
        '.' => Ok((false, false)),
        'S' => Ok((false, true)),
        _ => bail!("Invalid input: {:?}", c),
    }
}

#[derive(Deref)]
pub struct Day21Part2(Rc<Day21Part1>);

impl FromStr for Day21Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let start = OnceCell::new();
        let start_ref = &start;
        let iter = s.lines().enumerate().map(|(y, line)| {
            line.chars().enumerate().map(|(x, c)| (x, parse_position(c))).map(move |(x, res)| {
                res.and_then(|(blocked, is_start)| {
                    if is_start {
                        start_ref
                            .set((x, y))
                            .map(|_| blocked)
                            .map_err(|_| anyhow!("Found 2 starting position"))
                    } else {
                        Ok(blocked)
                    }
                })
            })
        });

        let grid = Grid2dBitVec::try_new(iter)?;
        let start = start.into_inner().context("Cannot find starting pos")?;

        Ok(Day21Part1::new(start, grid))
    }
}

impl Display for Day21Part1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output = Itertools::intersperse(
            self.grid.rows().enumerate().map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, blocked)| {
                        if (x, y) == self.start {
                            'S'
                        } else if *blocked {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            }),
            "\n".to_owned(),
        )
        .collect::<String>();

        f.write_str(output.as_ref())
    }
}

impl ProblemSolver for Day21Part1 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        Ok(self.step(64).0.count_ones())
    }
}
const CARDINAL: &[GridDirection; 4] =
    &[GridDirection::North, GridDirection::South, GridDirection::East, GridDirection::West];

impl Day21Part1 {
    fn step(&self, step_count: usize) -> (BitVec, BitVec) {
        let step_count_inner = step_count + 1;
        let mut occupied_even_step = bitvec!(0; self.grid.size());
        let mut occupied_odd_step = occupied_even_step.clone();
        bfs(
            self.start,
            |(x, y)| self.get_neighbor(*x, *y),
            |depth, (x, y)| {
                if *depth > step_count_inner {
                    true
                } else {
                    if depth.is_odd() {
                        assert!(
                            !occupied_even_step.replace(self.grid.flatten_idx(*x, *y), true),
                            "A position should not be applied twice"
                        );
                    } else {
                        assert!(
                            !occupied_odd_step.replace(self.grid.flatten_idx(*x, *y), true),
                            "A position should not be applied twice"
                        );
                    }
                    false
                }
            },
            0_usize,
            |prev_depth, _| 1 + prev_depth,
        );

        (occupied_even_step, occupied_odd_step)
    }

    fn get_neighbor(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        CARDINAL
            .iter()
            .filter_map(|direction| {
                self.grid.move_from_coordinate_to_direction(x, y, 1, *direction)
            })
            .filter(|(x, y)| !self.grid[(*x, *y)])
            .collect()
    }

    #[allow(dead_code)]
    fn to_string_with_occupied(&self, occupied: &BitVec) -> String {
        Itertools::intersperse(
            self.grid.rows().enumerate().map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, blocked)| {
                        if occupied[self.grid.flatten_idx(x, y)] {
                            'O'
                        } else if *blocked {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            }),
            "\n".to_owned(),
        )
        .collect::<String>()
    }
}

impl ProblemSolver for Day21Part2 {
    type SolutionType = WarningResult<usize>;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        ensure!(
            self.grid.width() == self.grid.height(),
            "Failed to assume provided grid is a square"
        );
        let grid_edge = self.grid.width();
        ensure!(grid_edge.is_odd(), "Failed to assume provided grid edge length is odd");
        let radius = grid_edge / 2;
        ensure!(radius.is_odd(), "Failed to assume radius is odd");
        ensure!(
            self.start == (radius, radius),
            "Failed to assume starting position is in center of grid"
        );
        ensure!(
            self.grid.rows().all(|slice| !slice[radius]),
            "Failed to assume middle column of grid is empty"
        );
        ensure!(
            self.grid.get_row(radius).not_any(),
            "Failed to assume middle row of grid is empty"
        );
        ensure!(
            (26501365 - radius) % grid_edge == 0,
            "Failed to assume 26501365 step will end next to a grid edge"
        );
        let grid_count_radius = (26501365 - radius) / grid_edge;
        ensure!(grid_count_radius.is_even(), "Failed to grid count radius is even");

        let corner_mask: BitVec = (0..grid_edge)
            .cartesian_product(0..grid_edge)
            .map(|(y, x)| x.abs_diff(radius) + y.abs_diff(radius) > radius)
            .collect();

        // valid position if fill every thing.
        let (valid_even_grid_mask, valid_odd_grid_mask) = self.step(grid_edge);
        let odd_grid_count = valid_odd_grid_mask.count_ones();
        let even_grid_count = valid_even_grid_mask.count_ones();

        let valid_even_grid_corner_mask = corner_mask.clone() & (&valid_even_grid_mask);
        let even_grid_corner_count = valid_even_grid_corner_mask.count_ones();

        let valid_odd_grid_corner_mask = corner_mask & (&valid_odd_grid_mask);
        let odd_grid_corner_count = valid_odd_grid_corner_mask.count_ones();

        let res = (grid_count_radius + 1).pow(2) * odd_grid_count
            + grid_count_radius.pow(2) * even_grid_count
            + grid_count_radius * even_grid_corner_count
            - (grid_count_radius + 1) * odd_grid_corner_count;

        Ok(WarningResult::new(
            res,
            "Check code for assumption. Also assume every fillable position within 26501365 euclidean distance is filled.",
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day21::{Day21, Day21Part1};
    use crate::solver::TwoPartsProblemSolver;
    use std::ops::Deref;

    use indoc::indoc;

    use std::str::FromStr;

    const SAMPLE_INPUT_1: &str = indoc! {r"
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........
    "};

    const SAMPLE_INPUT_2: &str = indoc! {r"
            ...................................................................................................................................
            ..##....#...#.............#......................#......#..............#....#....#.#...............#......#.....#.....#............
            ..##.......#......#...###..#...#............................................................#............#..#.....#................
            .....##...#.......................#..#.......#...#.#..#..................................#.........#.............#.#.##............
            ............##....................#.....#........#..##....................#..........#...........#...............................#.
            ..#.......#...........................#..#.##..#.....##.........#......................#...#...##.........#.............#.#....#...
            .....##.................#.#.###..#.........#................................#.......#....#..#.........#...#.......##..#..........#.
            .#....#.........#.......#....#.##..#.....#........................#.............#......#.......#...............#......#.........#..
            ......#.............#......#.#................#...#..........................#...........#..#.....#.......#...........#....#...#...
            ..........#....#...............#....................#.............#.............##..#.#.#.#.......#...........##..............#....
            ..###.....#......#.......#...#....#.....#.........#...................................#.#....#..#..#...#...#......#................
            ....#.........#.#..........##.#............#....................................#....#..............###............#....#.......#..
            ....#....#............#...............#.....................#...#.#...#..................#.............................#...........
            ..............#...#.........#......#.....#...............#...........................#.............#...#...........................
            .................#............#............#.#.....................#.....#..........#..#..#..##..........#.............#...#.......
            ................##......#.............#..............................#....#............#..#.........##........##......#..........#.
            ......#..........#...#..#............#..#.....................##...#....#.#.................................#.....#...........##...
            ..........#.....#......#...#......#......##...........#.....#...........#.#..#................#..#..#.....#...#......#...#...#...#.
            ..........#........#...#...........##....#.........#...#.....#...............#........................#...#.................#......
            .........#.....#...#....#.....#.......................##...#......#........................#.............#.......#....#..#....#....
            ..#............#.#.....#............................###...#.#.#....#..###...................###..#.#........#....#......#........#.
            ...#........#....#.#........##....#..#.............##.....#....#.......#......#.......................#...#.............#........#.
            .......#...###..#..................#.............#......#.............................................#....#.......#......#..#.#...
            ....................................##.........#..............#...........#.................##....#..............#.#........#......
            .#....##......#.............#.....................#....#..........##..#..##....#.........................#......................#..
            ...........#.............#........#...........#............................##....#...#........#..........##.##....#.....#.#........
            .#..................##..#.......#.............#........#..#.#........#............#...................#.#...##.#.#.................
            .....#.....#.....#...........#.#....................#.....#..#..#............#.........#.........#......#......#...............#...
            .....#........#..#....#..............................#..#...........#............#....#.#...................#.....#.#........#..#..
            .....#.......#......#...#...#...........#.......#.......#.................................#..........#...#.#......#.............#..
            ......#..........#........#..................#...#.........##.....#.............##...#..............#................#...........#.
            ................#........#...#..............#.......##......#.#....##...#.....#..#......#...#.......#.................#....#.#...#.
            .....##.......#.....###.........................................#..........###.....#...##......................#............#......
            ...##..#....#........................#........#..##.#.##.#.........#...#........#.#.........#...................#.#...........#..#.
            ....#..............#...............#...................#............#.......................#...........#..#.#.....................
            ...##............#..#...............#.#.#.....#............................##..#.#.........#..#....................................
            ........####..#.#.................#..#.......#.#...................................#.........................#..#...........#.#....
            ....................................##.....#.............##..#....#.........##.....#..............................#......#.#.......
            ..#.........##..##.#...................#....#...............#.......#.........#..............................#.#.#.....#....#..#...
            ..........#...#..#....#.........#..#........#.........##......#......#.............###.....#.......#...................##.#.#......
            .........#....................#...##.......#............##..........#..#..#...#....#..........#..............#........#...#.##.....
            .#........#..........................#.......##....#.#...#......#........#.....#...........#.....................#......#..........
            ...................................................................#..##.............###..........##............#........#.......#.
            .#..............................................#.#.#.......#.........#........#................................#...#....#.........
            ..............#.#.............####.....#.#.....#......#.........#...............#......#...#.....##..............##.#......##......
            ...##...#...#..............#.......#.#....#....#.....#..#......#...#.#.#...#..#...............#.......................#........#...
            ........#..#................#.....#..#.#.............#.#....#......#.................................................#.............
            ...#...#......#..............#..#.......#...#.....#.#..##..............##...............#..#.#.#........#...............#..........
            ......#.....................#..##..#..........#........#....##............#......#.........#.....#....#.......................#.#..
            .........#.#.........#...............#.#...........#....#......#...........#..#..#.........................................#.......
            ..##..............................##...#......#...........#...........#...........................#.....................#..#.......
            .....#...................#.#.#...........###....#.....................##...........................#...........#........#.#....#...
            .#.....#................#..##........##....#............#..........#............#....#...#..#.....#...........#...............#....
            ........#.............##.#.......................#.......................#...#......#.####.............#....................#......
            ....#...............#....#......#.#.............##........#....#.......#...#..#............#.......................#..........##...
            .#.#..................#..#..............#......#..#............#...#.......#.#....#.#..#......#.#.#.##.......#......#...........##.
            .................#.......#.........#...........#.......#.......#...#....................###......#.....#..#.....#..............#...
            ..#...............#.....#.#.....#....#.#.......#..#..........#.............................#......#...#............#............#..
            ...........#....##...##...........#...#...........#.................#...........................#...##....#........................
            .#.........#.....##.........###......#................##...#..#.#......#...................#.#...................#......#..........
            ..................#.#........##...............#.......#..#...#..#.........##....#.#....#.............#................##...........
            ........#..................#..........#..#....#.............#.............#.........#...............#........#.....................
            ...................#...............#....#.....#................#...#.............#.......................#.#.....#..#......#.......
            .......#..###.............#..#........#..................#........................#...................#...#..........##....#.......
            .......#................##...................#.................................#.#.....#..#............#.......#........#..........
            .................................................................S.................................................................
            ........###..#.........#.........##..............#..............#.#.....#.......##.......#.....#...#.............#.................
            ............#.................#....#.....................................#...#..#...#.............#............##.....#............
            .............#..............................................................#.##..........................#.........#..............
            ........##...................#.....#..#...#.....#.....#.............#............#...#..#........#.......#.#............#..........
            .#.........#.......#......#....#...##.###.....#..#..........##........#....#....#.##.......#......#..##.............#.#............
            ..............#.....#.....#.......#...................#......#....##....#.#............#.................##........................
            ..#........#.....#.#.#.#.............#.#............#....#......#.......#...........#.......##..#.....#.#.##...####....#.......#...
            .#..............#....#.......#........................#........#........#.#.#...#..........#........#..##..#...#..#...........#....
            ....#..........#.##.##.##......##....#...............#.............#..#...#...........#.###...........#.....##.#...#............#..
            .#.................#..............#.................#..........#.....#..................#..................#.......................
            ...#.##............#..#.....#.......#......##.#....#...........#...................#.#............#.##.#.....#.................#.#.
            ......#.............#.....#...#.............#.......................##....##............#.##..#..#.......#......................#..
            ...#...............#.........#.#...........#......#.....##..#...#..#.........#.#....#......#...#....#......#.#...................#.
            .......#...............#..##.......#..#.....#.....#.....#..#.#....##..#....#..........##........#.........................#...#....
            .......#..#.................##..............#.......#.#....................#.#....#....#..................#............##..........
            .......#.............#.............#....#.#.###...........#....#...#......#........#......#.....#........#...#.....................
            ........#.....................#.#...#....................................#..#..#....#..#.......#.......................#.#....#....
            ..#.....##............#................#........##...#.....##..............#.....#............##..#..#...............#....#....#...
            ....#.#..................###.##..#.#...#...........................#.#.............#...##.#.............#.....................##...
            .............#..............#......#..................................#...................#.........#.....#.......#..#..#..........
            .....#..#......#.............#.............#...#.........#....##....##..................#.........#................#.....#...#.#...
            .................#............................#...........#..............#.....#....#.#.........#....#.#........#........#.........
            ..#.......#............................#.#....#.##.......................#..................#.#....####..........#.#...#.....##....
            ......#...#.....#...............#.#..........##....#........#..............#.......#...#...#...................#...........#.#.....
            .....................#.......#..#.............#...................#..............................#...............#.....#....#......
            ....#..#.......#.........................#.#..........#....#...............#............#.......#...........................#......
            .........#......#...............##.#......#................#....#.....#................#.....#.#....................#..#...........
            ..#....#..#...#....#.#................#..#............#.#.#.............#.#....#......#.#.................#....#..#.........#....#.
            .....#.....#..#..................................#......#.......#............................................#.............#...#...
            .##..#.................#...............#...........................#...........#..#.......#............................#.....#.....
            .................##......##...........#.#.....#......#.........#......#.......#.........#....#.............##.....#............#...
            ....#....#...##...#...#....##.........#......##.................#.........#.......#....#...#...................#....#...#.....#....
            ........#.#......................................#.............#......#.#.#.....#.......#.............##...#.....#.................
            .#.....#.#..............................................#.#...##......................#.....#........##..#............#....#.......
            .........#........#........................##..#........##....#.#...#......#.#...#...#.............#.......#..#.#..................
            ........#..#.#...............#...........##.....#....#...#..#..#..........#......#......#..........#.....#.....##.........#.....##.
            ...#....#...#.........#..#.#...#.#........##...#............#..##.#.##.#..##............#.........#.#..........#...................
            .#...........##..#........##....#.............#...#...#.......#.............#...#.#...#............#.........#.............##......
            .#.....................#......................#...........#...#.....#...#..............................#.........#.....#.#....#.#..
            ............#......#...............................#......#....#...............#...................#....................#..........
            ...##...............................#...............#..##........................#....................#.#..#...##................#.
            ...#..#......#..#....#....#...#.#...#.....................#.#......#.#..........................#..#............#....#....#........
            .....##.#.....#..........#..#......##............##.........#.........#.....##.....#............#.............#...........##...#...
            ...........#....#............#......#....................#...#..........##......#................##.........##....#...#............
            .....##...........#.....#.#.............#............#........................#.............#......##..#......#..#.....#........#..
            ...#.#......#.......#..#.................#................#..#........#...................###............##....#..#..........##....
            .#..#.................###....##......................#.#...................#...............#...#.#....#.#.##...........#.......#...
            ....#..#................#........#..#................................#.......#...........#.......#.#..............##....#......#...
            ......#................#....#................#............#.#...#......#...............#............#..##....#..................#..
            .........#..................................#.#...........#..................................#............#.#..........##..##.#....
            .#..............#......#...........####.....#..............#.............................#...#......#....#........#.............#..
            ....####...........#..#....#.............#.....#...........#...#..........................#..#....#.....#..#...........#.........#.
            .....#.#.....#...........................#.....#...........#...............................##...............##..#.#.#.........#....
            .##............#.#.#.#....##.#...#.#.............................................#.........#......#.#.............#...##...........
            .#....#.................#.#....#............#............................................#........#...........#..#.................
            ..................#....#.##.#.#.#..........#........#.............#....................#...........#.....#.........................
            .........#...#.........#..................#........#...............#.................#......##........#........#.#.#.......##....#.
            .......#.......#......#.......##........#....#....#.#.......................................#..............#.....#.....#...#..#....
            ......#.................#.........##....#.......................#...............#.#.....................#..#....###.#........##....
            .......#....#.....#.............................................................#..........#..#.............#..#.#..#..............
            .#.....#................#...#....#..#....##..#..##........................................#...#..............#.....#...........##..
            ........#....#...##..#.....#.#.........#..............#.........................................................#......#..#.....##.
            ....#..#.#....#..............................#.#.....##..##...........................#...............#......#.....................
            .##.....#.#...........##.......#........#....#.........................##.#......#.........................#.......................
            ...................................................................................................................................
    "};

    #[test]
    fn test_solve_1() -> anyhow::Result<()> {
        assert_eq!(Day21Part1::from_str(SAMPLE_INPUT_1)?.step(6).0.count_ones(), 16);
        assert_eq!(Day21::from_str(SAMPLE_INPUT_2)?.solve_1()?, 3758);
        Ok(())
    }

    #[test]
    fn test_solve_2() -> anyhow::Result<()> {
        assert_eq!(*Day21::from_str(SAMPLE_INPUT_2)?.solve_2()?.deref(), 621494544278648);
        Ok(())
    }
}
