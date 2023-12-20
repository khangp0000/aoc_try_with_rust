use crate::solver::TwoPartsProblemSolver;
use anyhow::{Context, Result};
use std::cmp::max;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse color, expected \"red\", \"green\" or \"blue\"; but got {0:?}")]
    FailedToParseColor(String),
}

pub struct Day2 {
    games: Vec<Game>,
}

pub struct Game {
    index: u32,
    bag: Vec<CubeSet>,
}

pub struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for CubeSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0_u32;
        let mut green = 0_u32;
        let mut blue = 0_u32;
        for step in s.split(',') {
            let step = step.trim();
            let (count, color) = step
                .split_once(' ')
                .with_context(|| format!("Expected \"[count] [color]\" but got {:?}", step))?;
            let count = u32::from_str(count).with_context(|| {
                format!("Expected \"[count] [color]\" but cannot parse number [count]: {:?}", step)
            })?;
            match color {
                "red" => {
                    red = count;
                }
                "green" => {
                    green = count;
                }
                "blue" => {
                    blue = count;
                }
                _ => return Err(Error::FailedToParseColor(color.to_owned()))?,
            }
        }
        Ok(CubeSet { red, green, blue })
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_number, sets) = s
            .split_once(':')
            .ok_or_else(|| crate::utils::Error::FailedToSplit(s.to_owned(), ':'))?;
        let game_number = game_number.trim();
        let sets = sets.trim();
        let (_, game_number) = game_number.split_once(' ').with_context(|| {
            format!("Expected \"Game [game_number]\" but got {:?}", game_number)
        })?;
        let game_number = <u32>::from_str(game_number).with_context(|| {
            format!(
                "Expected \"Game [game_number]\" but cannot parse [game_number]: {:?}",
                game_number
            )
        })?;
        let bag = sets.split(';').map(str::trim).map(CubeSet::from_str).collect::<Result<_>>()?;
        Ok(Game { index: game_number, bag })
    }
}

impl FromStr for Day2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Day2 { games: s.lines().map(Game::from_str).collect::<anyhow::Result<_>>()? })
    }
}

impl TwoPartsProblemSolver for Day2 {
    type Solution1Type = u32;
    type Solution2Type = u32;

    fn solve_1(&self) -> anyhow::Result<u32> {
        return Ok(self
            .games
            .iter()
            .filter(|game| {
                game.bag
                    .iter()
                    .all(|bag| bag.red <= 12_u32 && bag.green <= 13_u32 && bag.blue <= 14_u32)
            })
            .map(|game| game.index)
            .sum::<u32>());
    }

    fn solve_2(&self) -> anyhow::Result<u32> {
        return Ok(self
            .games
            .iter()
            .map(|game| {
                game.bag.iter().fold(
                    CubeSet { red: 0_u32, green: 0_u32, blue: 0_u32 },
                    |mut left, right| {
                        left.red = max(left.red, right.red);
                        left.green = max(left.green, right.green);
                        left.blue = max(left.blue, right.blue);
                        left
                    },
                )
            })
            .map(|cube_set| cube_set.red * cube_set.green * cube_set.blue)
            .sum::<u32>());
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day2::Day2;
    use crate::solver::TwoPartsProblemSolver;
    use indoc::indoc;
    use std::str::FromStr;

    const SAMPLE_INPUT: &str = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day2::from_str(SAMPLE_INPUT)?.solve_1()?, 8);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day2::from_str(SAMPLE_INPUT)?.solve_2()?, 2286);
        Ok(())
    }
}
