use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;

use anyhow::Result;
use derive_more::{Deref, FromStr};
use itertools::Itertools;

use crate::solver::{share_struct_solver, ProblemSolver};

share_struct_solver!(Day11, Day11Part1, Day11Part2);

pub struct Day11Part1 {
    galaxies: Vec<(usize, usize)>,
    x_to_index: HashMap<usize, usize>,
    y_to_index: HashMap<usize, usize>,
}

#[derive(Deref)]
pub struct Day11Part2(Rc<Day11Part1>);

impl FromStr for Day11Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let galaxies = s
            .lines()
            .enumerate()
            .flat_map(move |(y, line)| {
                line.bytes()
                    .enumerate()
                    .filter_map(move |(x, b)| if b == b'#' { Some((x, y)) } else { None })
            })
            .collect::<Vec<_>>();
        let (sorted_x, y_to_index) = galaxies.iter().fold(
            (BTreeSet::default(), HashMap::new()),
            |(mut sorting_x, mut y_to_index), (x, y)| {
                sorting_x.insert(*x);
                let current_y_to_index_len = y_to_index.len();
                y_to_index.entry(*y).or_insert(current_y_to_index_len);
                (sorting_x, y_to_index)
            },
        );

        Ok(Day11Part1 {
            galaxies,
            x_to_index: sorted_x.into_iter().enumerate().map(|(idx, val)| (val, idx)).collect(),
            y_to_index,
        })
    }
}

impl ProblemSolver for Day11Part1 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        Ok(self.find_distance_with_expand_factor(2))
    }
}

impl Day11Part1 {
    fn find_distance_with_expand_factor(&self, expand_factor: usize) -> usize {
        return self
            .galaxies
            .iter()
            .tuple_combinations::<(_, _)>()
            .map(|((lx, ly), (rx, ry))| {
                find_galaxy_1d_distance(*lx, *rx, expand_factor, &self.x_to_index)
                    + find_galaxy_1d_distance(*ly, *ry, expand_factor, &self.y_to_index)
            })
            .sum::<usize>();
    }
}

pub fn find_galaxy_1d_distance(
    d_1: usize,
    d_2: usize,
    expand_factor: usize,
    d_to_index: &HashMap<usize, usize>,
) -> usize {
    let lo;
    let hi;
    if d_1 < d_2 {
        lo = d_1;
        hi = d_2;
    } else {
        lo = d_2;
        hi = d_1;
    }

    let mut diff = hi - lo;
    if diff > 1 {
        let index_diff = d_to_index[&hi] - d_to_index[&lo];
        diff = expand_factor * (diff - index_diff) + (index_diff);
    }

    diff
}

impl ProblemSolver for Day11Part2 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        Ok(self.find_distance_with_expand_factor(1000000))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Result;
    use indoc::indoc;

    use crate::solver::y2023::day11::{Day11, Day11Part1};
    use crate::solver::TwoPartsProblemSolver;

    const SAMPLE_INPUT_1: &str = indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
    "};

    #[test]
    fn test_sample_1() -> Result<()> {
        assert_eq!(Day11::from_str(SAMPLE_INPUT_1)?.solve_1()?, 374);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> Result<()> {
        Ok(())
    }

    #[test]
    fn test_expand() -> Result<()> {
        assert_eq!(
            Day11Part1::from_str(SAMPLE_INPUT_1)?.find_distance_with_expand_factor(10),
            1030
        );
        assert_eq!(
            Day11Part1::from_str(SAMPLE_INPUT_1)?.find_distance_with_expand_factor(100),
            8410
        );
        Ok(())
    }
}
