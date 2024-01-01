use std::fmt::Debug;
use std::rc::Rc;

use anyhow::Result;
use derive_more::{Deref, FromStr};
use derive_new::new;

use crate::solver::{share_struct_solver, ProblemSolver};

share_struct_solver!(Day22, Day22Part1, Day22Part2);

type BitSet = bit_set::BitSet<usize>;

#[derive(new, Debug)]
pub struct Day22Part1 {}

#[derive(Deref)]
pub struct Day22Part2(Rc<Day22Part1>);

impl FromStr for Day22Part1 {
    type Err = anyhow::Error;

    fn from_str(_s: &str) -> Result<Self> {
        Ok(Day22Part1 {})
    }
}

impl ProblemSolver for Day22Part1 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        todo!()
    }
}

impl ProblemSolver for Day22Part2 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Result;
    use indoc::indoc;

    use crate::solver::y2023::day22::Day22;
    use crate::solver::TwoPartsProblemSolver;

    const SAMPLE_INPUT_1: &str = indoc! {r"
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
    "};

    #[test]
    fn test_solve_1() -> Result<()> {
        assert_eq!(Day22::from_str(SAMPLE_INPUT_1)?.solve_1()?, 5);
        Ok(())
    }

    #[test]
    fn test_solve_2() -> Result<()> {
        assert_eq!(Day22::from_str(SAMPLE_INPUT_1)?.solve_2()?, 7);
        Ok(())
    }
}
