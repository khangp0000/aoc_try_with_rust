pub mod y2021;
pub mod y2023;

use crate::solver::y2021::Y2021_SOLVER;
use crate::solver::y2023::Y2023_SOLVER;
use crate::utils::Result2Parts;
use anyhow::{anyhow, Result};
use derive_new::new;
use phf::{phf_map, Map};

use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::Deref;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

pub static AOC_PROBLEMS_SOLVER: Map<
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

pub trait ProblemSolver<D, P = Self>: FromStr<Err = anyhow::Error> {
    type SolutionType: Display;
    fn solve(&self) -> Result<Self::SolutionType>;
}

pub trait TwoPartsProblemSolver: FromStr<Err = anyhow::Error> {
    type Solution1Type: Display;
    type Solution2Type: Display;

    fn solve_1(&self) -> Result<Self::Solution1Type>;
    fn solve_2(&self) -> Result<Self::Solution2Type>;
}

impl<
        T: TwoPartsProblemSolver<Solution1Type = T1, Solution2Type = T2>,
        T1: Display,
        T2: Display,
    > ProblemSolver<T, T> for T
{
    type SolutionType = Result2Parts<T1, T2>;
    fn solve(&self) -> Result<Result2Parts<T1, T2>> {
        return Ok(Result2Parts::new(self.solve_1()?, self.solve_2()?));
    }
}

#[derive(new)]
pub struct TwoSolversCombined<P1: ProblemSolver<B1>, P2: ProblemSolver<B2>, B1 = P1, B2 = P2> {
    problem_1: P1,
    problem_2: P2,
    phantom_b1: PhantomData<B1>,
    phantom_b2: PhantomData<B2>,
}

impl<T1, T2, P1, P2, B1, B2> TwoPartsProblemSolver for TwoSolversCombined<P1, P2, B1, B2>
where
    T1: Display,
    T2: Display,
    P1: ProblemSolver<B1, SolutionType = T1>,
    P2: ProblemSolver<B2, SolutionType = T2>,
{
    type Solution1Type = T1;
    type Solution2Type = T2;

    fn solve_1(&self) -> Result<T1> {
        self.problem_1.solve()
    }
    fn solve_2(&self) -> Result<T2> {
        self.problem_2.solve()
    }
}

impl<T1, T2, P1, P2, B1, B2> FromStr for TwoSolversCombined<P1, P2, B1, B2>
where
    T1: Display,
    T2: Display,
    P1: ProblemSolver<B1, SolutionType = T1>,
    P2: ProblemSolver<B2, SolutionType = T2>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(TwoSolversCombined::new(
            P1::from_str(s).map_err(|e| anyhow!(e))?,
            P2::from_str(s).map_err(|e| anyhow!(e))?,
        ));
    }
}

impl<D, P1, P2, B1, B2> ProblemSolver<TwoSolversCombined<P1, P2, B1, B2>, D> for D
where
    D: Deref<Target = TwoSolversCombined<P1, P2, B1, B2>> + FromStr<Err = anyhow::Error>,
    P1: ProblemSolver<B1>,
    P2: ProblemSolver<B2>,
{
    type SolutionType = Result2Parts<P1::SolutionType, P2::SolutionType>;
    fn solve(&self) -> Result<Self::SolutionType> {
        return Ok(Result2Parts::new(
            self.deref().solve_1()?,
            self.deref().solve_2()?,
        ));
    }
}
