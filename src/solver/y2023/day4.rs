use crate::solver::TwoPartsProblemSolver;
use std::cmp::min;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day4 {
    cards: Vec<(HashSet<u32>, HashSet<u32>)>,
}

impl FromStr for Day4 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Day4 {
            cards: s
                .lines()
                .map(|s| s.split_once(':').unwrap().1)
                .map(|s| s.split_once('|').unwrap())
                .map(|(l, r)| {
                    Ok::<_, anyhow::Error>((
                        parse_vec_u32_white_space_delimiter(l)?,
                        parse_vec_u32_white_space_delimiter(r)?,
                    ))
                })
                .collect::<Result<_, _>>()?,
        });
    }
}

impl TwoPartsProblemSolver<u64, u64> for Day4 {
    fn solve_1(&self) -> anyhow::Result<u64> {
        return Ok(self
            .cards
            .iter()
            .map(|(l, r)| l.intersection(r).count())
            .filter(|&count| count != 0)
            .map(|count| 1u64 << (count - 1))
            .sum());
    }

    fn solve_2(&self) -> anyhow::Result<u64> {
        let num_card = self.cards.len();
        let mut counts = vec![1_u64; num_card];
        for (index, (l, r)) in self.cards.iter().enumerate() {
            let bonus = l.intersection(r).count();
            let upper_bound = min(num_card, index + bonus + 1);
            for bonus_idx in index + 1..upper_bound {
                counts[bonus_idx] += counts[index];
            }
        }
        Ok(counts.iter().sum())
    }
}

fn parse_vec_u32_white_space_delimiter<B: FromIterator<u32>>(
    input: &str,
) -> Result<B, ParseIntError> {
    return input
        .split_whitespace()
        .filter(|&s| !s.is_empty())
        .map(<u32>::from_str)
        .collect::<Result<B, _>>();
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day4::Day4;
    use crate::solver::TwoPartsProblemSolver;
    use indoc::indoc;
    use std::str::FromStr;

    static SAMPLE_INPUT: &str = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day4::from_str(SAMPLE_INPUT)?.solve_1()?, 13);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day4::from_str(SAMPLE_INPUT)?.solve_2()?, 30);
        Ok(())
    }
}
