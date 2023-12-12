use phf::{phf_map, Map};
use std::fmt::Display;
use std::path::Path;

use crate::solver::y2023::day1::Day1;
use crate::solver::y2023::day2::Day2;
use crate::solver::y2023::day3::Day3;
use crate::solver::y2023::day4::Day4;
use crate::solver::y2023::day5::Day5;
use crate::solver::y2023::day6::Day6;
use crate::solver::y2023::day7::Day7;
use crate::solver::y2023::day8::Day8;
use crate::utils::boxed_try_get_input_and_solve;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

pub static Y2023_SOLVER: Map<u8, fn(u16, u8, &Path, &Path) -> anyhow::Result<Box<dyn Display>>> = phf_map! {
    1_u8 => boxed_try_get_input_and_solve!(Day1),
    2_u8 => boxed_try_get_input_and_solve!(Day2),
    3_u8 => boxed_try_get_input_and_solve!(Day3),
    4_u8 => boxed_try_get_input_and_solve!(Day4),
    5_u8 => boxed_try_get_input_and_solve!(Day5<u32>),
    6_u8 => boxed_try_get_input_and_solve!(Day6),
    7_u8 => boxed_try_get_input_and_solve!(Day7),
    8_u8 => boxed_try_get_input_and_solve!(Day8),
};
