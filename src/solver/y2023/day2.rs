use crate::solver::TwoPartsProblemSolver;
use crate::utils::FromSScanfError;
use anyhow::{anyhow};
use sscanf::sscanf;
use std::cmp::max;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to split with delimiter {1:?}: {0:?}")]
    FailedToSplit(String, char),
    #[error("Failed to parse color, expected \"red\", \"green\" or \"blue\"; but got {0:?} ")]
    FailedToParseColor(String),
    #[error(
        "Failed to sscanf {:?} with pattern {:?} caused by {:?}",
        string_to_scan,
        pattern,
        source
    )]
    FailedToSScanf {
        string_to_scan: String,
        pattern: &'static str,
        source: Option<anyhow::Error>,
    },
}

impl FromSScanfError for Error {
    fn from_sscanf_err(
        err: &sscanf::Error,
        string_to_scan: String,
        pattern: &'static str,
    ) -> Error {
        return match err {
            sscanf::Error::MatchFailed => Error::FailedToSScanf {
                string_to_scan,
                pattern,
                source: None,
            },
            sscanf::Error::ParsingFailed(inner_error) => Error::FailedToSScanf {
                string_to_scan,
                pattern,
                source: Some(anyhow!(inner_error.to_string())),
            },
        };
    }
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
            let (count, color) = sscanf!(step, "{u32} {str}")
                .map_err(|e| Error::from_sscanf_err(&e, step.to_owned(), "{u32} {str}"))?;
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
        return Ok(CubeSet { red, green, blue });
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_number, sets) = s
            .split_once(':')
            .ok_or_else(|| Error::FailedToSplit(s.to_owned(), ':'))?;
        let game_number = game_number.trim();
        let sets = sets.trim();
        let game_number = sscanf!(game_number, "Game {u32}")
            .map_err(|e| Error::from_sscanf_err(&e, game_number.to_owned(), "Game {u32}"))?;
        let bag = sets
            .split(';')
            .map(str::trim)
            .map(CubeSet::from_str)
            .collect::<Result<_, _>>()?;
        return Ok(Game {
            index: game_number,
            bag,
        });
    }
}

impl FromStr for Day2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Day2 {
            games: s.lines().map(Game::from_str).collect::<Result<_, _>>()?,
        });
    }
}

impl TwoPartsProblemSolver for Day2 {
    type Target1 = u32;
    type Target2 = u32;

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
                    CubeSet {
                        red: 0_u32,
                        green: 0_u32,
                        blue: 0_u32,
                    },
                    |mut left, right| {
                        left.red = max(left.red, right.red);
                        left.green = max(left.green, right.green);
                        left.blue = max(left.blue, right.blue);
                        return left;
                    },
                )
            })
            .map(|cube_set| cube_set.red * cube_set.green * cube_set.blue)
            .sum::<u32>());
    }
}

#[cfg(all(test))]
mod tests {
    use crate::solver::y2023::day2::{CubeSet, Day2, Game};
    use crate::solver::TwoPartsProblemSolver;
    use indoc::indoc;
    use std::str::FromStr;

    static SAMPLE_INPUT: &str = indoc! {"
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
