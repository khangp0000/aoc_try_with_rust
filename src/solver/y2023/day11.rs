use crate::solver::{share_struct_solver, ProblemSolver};
use derive_more::{Deref, FromStr};
use itertools::Itertools;
use std::rc::Rc;

share_struct_solver!(Day11, Day11Part1, Day11Part2);

pub struct Day11Part1 {
    galaxies: Vec<(usize, usize)>,
    sorted_x: Vec<usize>,
    sorted_y: Vec<usize>,
}

#[derive(Deref)]
pub struct Day11Part2(Rc<Day11Part1>);

impl FromStr for Day11Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let galaxies = s
            .lines()
            .enumerate()
            .flat_map(move |(y, line)| {
                line.bytes()
                    .enumerate()
                    .filter_map(move |(x, b)| if b == b'#' { Some((x, y)) } else { None })
            })
            .collect::<Vec<_>>();
        let (mut non_sorted_x, sorted_y) = galaxies.iter().fold(
            (Vec::new(), Vec::new()),
            |(mut non_sorted_x, mut sorted_y), (x, y)| {
                non_sorted_x.push(*x);
                if Some(y) != sorted_y.last() {
                    sorted_y.push(*y);
                }
                (non_sorted_x, sorted_y)
            },
        );

        non_sorted_x.sort_unstable();
        non_sorted_x.dedup();
        Ok(Day11Part1 { galaxies, sorted_x: non_sorted_x, sorted_y })
    }
}

impl ProblemSolver for Day11Part1 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        Ok(self.find_distance_with_expand_factor(&2))
    }
}

impl Day11Part1 {
    fn find_distance_with_expand_factor(&self, expand_factor: &usize) -> usize {
        return self
            .galaxies
            .iter()
            .tuple_combinations::<(_, _)>()
            .map(|((lx, ly), (rx, ry))| {
                find_galaxy_1d_distance(lx, rx, expand_factor, &self.sorted_x)
                    + find_galaxy_1d_distance(ly, ry, expand_factor, &self.sorted_y)
            })
            .sum::<usize>();
    }
}

pub fn find_galaxy_1d_distance(
    d_1: &usize,
    d_2: &usize,
    expand_factor: &usize,
    sorted_dedup_d_with_galaxies: &[usize],
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
        let index_diff = sorted_dedup_d_with_galaxies.binary_search(hi).unwrap()
            - sorted_dedup_d_with_galaxies.binary_search(lo).unwrap();
        diff = expand_factor * (diff - index_diff) + (index_diff);
    }

    diff
}

impl ProblemSolver for Day11Part2 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        Ok(self.find_distance_with_expand_factor(&1000000))
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day11::{Day11, Day11Part1};
    use crate::solver::TwoPartsProblemSolver;

    use indoc::indoc;

    use std::str::FromStr;

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
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day11::from_str(SAMPLE_INPUT_1)?.solve_1()?, 374);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        Ok(())
    }

    #[test]
    fn test_expand() -> anyhow::Result<()> {
        assert_eq!(
            Day11Part1::from_str(SAMPLE_INPUT_1)?.find_distance_with_expand_factor(&10),
            1030
        );
        assert_eq!(
            Day11Part1::from_str(SAMPLE_INPUT_1)?.find_distance_with_expand_factor(&100),
            8410
        );
        Ok(())
    }
}
