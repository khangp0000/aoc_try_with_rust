use crate::solver::TwoPartsProblemSolver;
use anyhow::Context;
use anyhow::Result;
use regex::Regex;
use std::str::FromStr;

pub struct Day1 {
    input: String,
}

impl FromStr for Day1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Day1 {
            input: s.to_owned(),
        });
    }
}

impl TwoPartsProblemSolver<u32, u32> for Day1 {
    fn solve_1(&self) -> anyhow::Result<u32> {
        let mut sum = 0_u32;
        for line in self.input.lines() {
            sum += line
                .matches(|c: char| c.is_ascii_digit())
                .next()
                .map(<u32>::from_str)
                .transpose()?
                .map(|v| v * 10)
                .with_context(|| format!("There is no digit in string {}.", line))?;

            sum += line
                .rmatches(|c: char| c.is_ascii_digit())
                .next()
                .map(<u32>::from_str)
                .transpose()?
                .with_context(|| format!("There is no digit in string {}.", line))?;
        }
        return Ok(sum);
    }

    fn solve_2(&self) -> anyhow::Result<u32> {
        let forward_search =
            Regex::new(r"(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|\d")?;
        let backward_search =
            Regex::new(r"(eno)|(owt)|(eerht)|(ruof)|(evif)|(xis)|(neves)|(thgie)|(enin)|\d")?;
        let mut sum = 0_u32;
        for line in self.input.lines() {
            sum += str_or_rev_digit_to_u32(
                forward_search
                    .find(line)
                    .with_context(|| format!("There is no digit in string {}.", line))?
                    .as_str(),
            )? * 10_u32;
            let rev_line: String = line.chars().rev().collect();
            sum += str_or_rev_digit_to_u32(
                backward_search
                    .find(rev_line.as_str())
                    .with_context(|| format!("There is no reverse digit in string {}.", line))?
                    .as_str(),
            )?;
        }
        return Ok(sum);
    }
}

fn str_or_rev_digit_to_u32(s: &str) -> anyhow::Result<u32> {
    return Ok(match s {
        "one" | "eno" => 1u32,
        "two" | "owt" => 2u32,
        "three" | "eerht" => 3u32,
        "four" | "ruof" => 4u32,
        "five" | "evif" => 5u32,
        "six" | "xis" => 6u32,
        "seven" | "neves" => 7u32,
        "eight" | "thgie" => 8u32,
        "nine" | "enin" => 9u32,
        val => <u32>::from_str(val)?,
    });
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day1::Day1;
    use crate::solver::TwoPartsProblemSolver;
    use indoc::indoc;
    use std::str::FromStr;

    static SAMPLE_INPUT_1: &str = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
    "};

    static SAMPLE_INPUT_2: &str = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
    "};

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day1::from_str(SAMPLE_INPUT_1)?.solve_1()?, 142);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day1::from_str(SAMPLE_INPUT_2)?.solve_2()?, 281);
        Ok(())
    }
}
