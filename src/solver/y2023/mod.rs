use crate::solver::y2023::day1::Day1;
use crate::solver::y2023::day10::Day10;
use crate::solver::y2023::day11::Day11;
use crate::solver::y2023::day12::Day12;
use crate::solver::y2023::day13::Day13;
use crate::solver::y2023::day14::Day14;
use crate::solver::y2023::day2::Day2;
use crate::solver::y2023::day3::Day3;
use crate::solver::y2023::day4::Day4;
use crate::solver::y2023::day5::Day5;
use crate::solver::y2023::day6::Day6;
use crate::solver::y2023::day7::Day7;
use crate::solver::y2023::day8::Day8;
use crate::solver::y2023::day9::Day9;
use crate::utils::boxed_try_get_input_and_solve;
use anyhow::Result;
use phf::{phf_map, Map};
use std::fmt::Display;
use std::path::Path;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub const Y2023_SOLVER: Map<u8, fn(u16, u8, &Path, &Path) -> Result<Box<dyn Display>>> = phf_map! {
    1_u8 => boxed_try_get_input_and_solve!(Day1),
    2_u8 => boxed_try_get_input_and_solve!(Day2),
    3_u8 => boxed_try_get_input_and_solve!(Day3),
    4_u8 => boxed_try_get_input_and_solve!(Day4),
    5_u8 => boxed_try_get_input_and_solve!(Day5<u32>),
    6_u8 => boxed_try_get_input_and_solve!(Day6),
    7_u8 => boxed_try_get_input_and_solve!(Day7),
    8_u8 => boxed_try_get_input_and_solve!(Day8),
    9_u8 => boxed_try_get_input_and_solve!(Day9),
    10_u8 => boxed_try_get_input_and_solve!(Day10),
    11_u8 => boxed_try_get_input_and_solve!(Day11),
    12_u8 => boxed_try_get_input_and_solve!(Day12),
    13_u8 => boxed_try_get_input_and_solve!(Day13),
    14_u8 => boxed_try_get_input_and_solve!(Day14),
};
