use std::fmt::Display;
use std::path::Path;

use anyhow::Result;
use phf::{phf_map, Map};

use crate::solver::y2023::day1::Day1;
use crate::solver::y2023::day10::Day10;
use crate::solver::y2023::day11::Day11;
use crate::solver::y2023::day12::Day12;
use crate::solver::y2023::day13::Day13;
use crate::solver::y2023::day14::Day14;
use crate::solver::y2023::day15::Day15;
use crate::solver::y2023::day16::Day16;
use crate::solver::y2023::day17::Day17;
use crate::solver::y2023::day18::Day18;
use crate::solver::y2023::day19::Day19;
use crate::solver::y2023::day2::Day2;
use crate::solver::y2023::day20::Day20;
use crate::solver::y2023::day21::Day21;
use crate::solver::y2023::day22::Day22;
use crate::solver::y2023::day23::Day23;
use crate::solver::y2023::day24::Day24;
use crate::solver::y2023::day25::Day25;
use crate::solver::y2023::day3::Day3;
use crate::solver::y2023::day4::Day4;
use crate::solver::y2023::day5::Day5;
use crate::solver::y2023::day6::Day6;
use crate::solver::y2023::day7::Day7;
use crate::solver::y2023::day8::Day8;
use crate::solver::y2023::day9::Day9;
use crate::utils::boxed_try_get_input_and_solve;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
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
    15_u8 => boxed_try_get_input_and_solve!(Day15),
    16_u8 => boxed_try_get_input_and_solve!(Day16),
    17_u8 => boxed_try_get_input_and_solve!(Day17),
    18_u8 => boxed_try_get_input_and_solve!(Day18),
    19_u8 => boxed_try_get_input_and_solve!(Day19),
    20_u8 => boxed_try_get_input_and_solve!(Day20),
    21_u8 => boxed_try_get_input_and_solve!(Day21),
    22_u8 => boxed_try_get_input_and_solve!(Day22),
    23_u8 => boxed_try_get_input_and_solve!(Day23),
    24_u8 => boxed_try_get_input_and_solve!(Day24),
    25_u8 => boxed_try_get_input_and_solve!(Day25),
};
