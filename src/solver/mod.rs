pub mod y2021;
pub mod y2023;
use crate::solver::y2021::Y2021_SOLVER;
use crate::solver::y2023::Y2023_SOLVER;
use crate::utils::Result2Parts;
use anyhow::Result;
use phf::{phf_map, Map};
use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

pub const AOC_PROBLEMS_SOLVER: Map<
    u16,
    &Map<u8, fn(u16, u8, &Path, &Path) -> Result<Box<dyn Display>>>,
> = phf_map! {
    2023_u16 => &Y2023_SOLVER,
    2021_u16 => &Y2021_SOLVER
};

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    InputParseError(#[from] anyhow::Error),
}

pub trait ProblemSolver: FromStr<Err = anyhow::Error> {
    type SolutionType: Display;
    fn solve(&self) -> Result<Self::SolutionType>;
}

pub trait TwoPartsProblemSolver: FromStr<Err = anyhow::Error> {
    type Solution1Type: Display;
    type Solution2Type: Display;

    fn solve_1(&self) -> Result<Self::Solution1Type>;
    fn solve_2(&self) -> Result<Self::Solution2Type>;
}

impl<T, T1, T2> ProblemSolver for T
where
    T: TwoPartsProblemSolver<Solution1Type = T1, Solution2Type = T2>,
    T1: Display,
    T2: Display,
{
    type SolutionType = Result2Parts<T1, T2>;
    fn solve(&self) -> Result<Result2Parts<T1, T2>> {
        Ok(Result2Parts::new(self.solve_1()?, self.solve_2()?))
    }
}

macro_rules! combine_solver {
    ($wrapper:ident, $solver1:ident, $solver2:ident ) => {
        pub struct $wrapper($solver1, $solver2);

        impl std::str::FromStr for $wrapper {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> anyhow::Result<Self> {
                Ok($wrapper($solver1::from_str(s)?, $solver2::from_str(s)?))
            }
        }

        impl crate::solver::TwoPartsProblemSolver for $wrapper {
            type Solution1Type = <$solver1 as ProblemSolver>::SolutionType;
            type Solution2Type = <$solver2 as ProblemSolver>::SolutionType;

            fn solve_1(&self) -> anyhow::Result<Self::Solution1Type> {
                self.0.solve()
            }

            fn solve_2(&self) -> anyhow::Result<Self::Solution2Type> {
                self.1.solve()
            }
        }
    };
}

macro_rules! share_struct_solver {
    ($wrapper:ident, $solver1:ident, $solver2:ident ) => {
        pub struct $wrapper(std::rc::Rc<$solver1>, $solver2);

        impl std::str::FromStr for $wrapper {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> anyhow::Result<Self> {
                let rc = std::rc::Rc::new($solver1::from_str(s)?);
                Ok($wrapper(rc.clone(), $solver2(rc)))
            }
        }

        impl crate::solver::TwoPartsProblemSolver for $wrapper {
            type Solution1Type = <$solver1 as ProblemSolver>::SolutionType;
            type Solution2Type = <$solver2 as ProblemSolver>::SolutionType;

            fn solve_1(&self) -> anyhow::Result<Self::Solution1Type> {
                self.0.solve()
            }

            fn solve_2(&self) -> anyhow::Result<Self::Solution2Type> {
                self.1.solve()
            }
        }

        impl std::str::FromStr for $solver2 {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
                Ok($solver2(std::rc::Rc::new($solver1::from_str(s)?)))
            }
        }
    };
}

pub(crate) use combine_solver;
pub(crate) use share_struct_solver;
