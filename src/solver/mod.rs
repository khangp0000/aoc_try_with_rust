pub mod y2021;
pub mod y2023;

use crate::solver::y2021::Y2021_SOLVER;
use crate::solver::y2023::Y2023_SOLVER;
use crate::utils::Result2Parts;
use anyhow::Result;
use phf::{phf_map, Map};
use std::fmt::Display;
use std::path::Path;

pub static AOC_PROBLEMS_SOLVER: Map<
    u16,
    &Map<u8, fn(u16, u8, &Path, &Path) -> Result<Box<dyn Display>>>,
> = phf_map! {
    2023_u16 => &Y2023_SOLVER,
    2021_u16 => &Y2021_SOLVER
};

pub trait ProblemSolver<T: Display> {
    fn solve(&self) -> Result<T>;
}

pub trait TwoPartsProblemSolver<T1: Display, T2: Display> {
    fn solve_1(&self) -> Result<T1>;
    fn solve_2(&self) -> Result<T2>;
}

impl<T: TwoPartsProblemSolver<T1, T2>, T1: Display, T2: Display> ProblemSolver<Result2Parts<T1, T2>>
    for T
{
    fn solve(&self) -> Result<Result2Parts<T1, T2>> {
        return Ok(Result2Parts::new(self.solve_1()?, self.solve_2()?));
    }
}
