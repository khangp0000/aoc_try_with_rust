use crate::solver::y2021::day1::Day1;
use crate::solver::y2021::day2::Day2;
use crate::solver::y2021::day3::Day3;
use crate::utils::boxed_try_get_input_and_solve;
use anyhow::Result;
use phf::{phf_map, Map};
use std::fmt::Display;
use std::path::Path;

pub mod day1;
pub mod day2;
pub mod day3;
// pub mod day4;

pub const Y2021_SOLVER: Map<u8, fn(u16, u8, &Path, &Path) -> Result<Box<dyn Display>>> = phf_map! {
    1_u8 => boxed_try_get_input_and_solve!(Day1),
    2_u8 => boxed_try_get_input_and_solve!(Day2),
    3_u8 => boxed_try_get_input_and_solve!(Day3),
};
