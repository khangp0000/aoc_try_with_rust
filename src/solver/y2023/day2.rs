use crate::solver::TwoPartsProblemSolver;
use anyhow::{anyhow, bail, Context, Result};
use sscanf::sscanf;
use std::cmp::max;
use std::str::FromStr;

pub struct Day2 {
    input: String,
}

impl FromStr for Day2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Day2 {
            input: s.to_owned(),
        });
    }
}

impl TwoPartsProblemSolver for Day2 {
    type Target1 = u64;
    type Target2 = u64;

    fn solve_1(&self) -> anyhow::Result<u64> {
        return self
            .input
            .lines()
            .map(is_valid_game)
            .map(Result::transpose)
            .filter(Option::is_some)
            .map(Option::unwrap)
            .sum();
    }

    fn solve_2(&self) -> anyhow::Result<u64> {
        return self.input.lines().map(power_factor).sum();
    }
}

fn is_valid_game(line: &str) -> anyhow::Result<Option<u64>> {
    let (game_number, sets) = line
        .split_once(':')
        .with_context(|| format!("Failed to split with delimiter ':' for string: {}", line))?;
    let game_number = game_number.trim();
    let sets = sets.trim();

    for set in sets.split(';') {
        let set = set.trim();
        for step in set.split(',') {
            let step = step.trim();
            let (count, color) = sscanf!(step, "{u64} {str}").map_err(|_| {
                anyhow!(format!(
                    "Failed to get count and color from string: {}",
                    step
                ))
            })?;
            match color {
                "red" => {
                    if count > 12 {
                        return Ok(None);
                    }
                }
                "green" => {
                    if count > 13 {
                        return Ok(None);
                    }
                }
                "blue" => {
                    if count > 14 {
                        return Ok(None);
                    }
                }
                _ => {
                    bail!(format!("Invalid color: {}", color));
                }
            }
        }
    }

    return Ok(Some(sscanf!(game_number, "Game {u64}").map_err(|_| {
        anyhow!(format!(
            "Failed to get game number from string: {}",
            game_number
        ))
    })?));
}

fn power_factor(line: &str) -> anyhow::Result<u64> {
    let (_, sets) = line
        .split_once(':')
        .with_context(|| format!("Failed to split with delimiter ':' for string: {}", line))?;
    let sets = sets.trim();
    let mut red = 0_u64;
    let mut green = 0_u64;
    let mut blue = 0_u64;
    for set in sets.split(';') {
        let set = set.trim();
        for step in set.split(',') {
            let step = step.trim();
            let (count, color) = sscanf!(step, "{u64} {str}").map_err(|_| {
                anyhow!(format!(
                    "Failed to get count and color from string: {}",
                    step
                ))
            })?;
            match color {
                "red" => {
                    red = max(count, red);
                }
                "green" => {
                    green = max(count, green);
                }
                "blue" => {
                    blue = max(count, blue);
                }
                _ => {
                    bail!(format!("Invalid color: {}", color));
                }
            }
        }
    }

    return Ok(red * green * blue);
}

#[cfg(all(test))]
mod tests {
    use crate::solver::y2023::day2::{is_valid_game, power_factor, Day2};
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
    fn test_line_1() -> anyhow::Result<()> {
        assert_eq!(
            is_valid_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")?,
            Some(1)
        );
        assert_eq!(
            is_valid_game(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            )?,
            None
        );
        Ok(())
    }

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day2::from_str(SAMPLE_INPUT)?.solve_1()?, 8);
        Ok(())
    }

    #[test]
    fn test_line_2() -> anyhow::Result<()> {
        assert_eq!(
            power_factor("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")?,
            48_u64
        );
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day2::from_str(SAMPLE_INPUT)?.solve_2()?, 2286_u64);
        Ok(())
    }
}
