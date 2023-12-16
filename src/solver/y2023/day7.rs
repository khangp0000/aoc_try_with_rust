use crate::solver::{combine_solver, ProblemSolver};
use anyhow::{bail, Context};
use derive_more::{Deref, Display, FromStr};
use std::collections::BinaryHeap;

combine_solver! {Day7, Day7Part1, Day7Part2}

#[derive(Deref)]
pub struct Day7Part1(Vec<(CardHand, u32)>);

#[derive(Deref)]
pub struct Day7Part2(Vec<(CardHandWithJoker, u32)>);

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Display, Debug)]
pub enum CardHand {
    HighCard(u32),
    OnePair(u32),
    TwoPair(u32),
    ThreeOfAKind(u32),
    FullHouse(u32),
    FourOfAKind(u32),
    FiveOfAKind(u32),
}

impl FromStr for CardHand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            bail!("Invalid input for CardHand: {:?}", s);
        }
        let (value, count) = s
            .bytes()
            .map(|b| match b {
                b'2'..=b'9' => Ok((b - b'2') as u32),
                b'T' => Ok(8_u32),
                b'J' => Ok(9_u32),
                b'Q' => Ok(10_u32),
                b'K' => Ok(11_u32),
                b'A' => Ok(12_u32),
                _ => bail!("Invalid input for CardHand: {:?}", s),
            })
            .try_fold((0_u32, vec![0_u8; 13]), |(value, mut counts), digit| {
                let digit = digit?;
                counts[digit as usize] += 1_u8;
                return Ok::<_, anyhow::Error>((value * 13_u32 + digit, counts));
            })?;
        let mut count_max_heap: BinaryHeap<_> = count
            .into_iter()
            .enumerate()
            .filter(|(_, count)| count != &0_u8)
            .map(|(index, count)| (count, index))
            .collect();

        return Ok(match count_max_heap.pop().unwrap() {
            (5, _) => CardHand::FiveOfAKind(value),
            (4, _) => CardHand::FourOfAKind(value),
            (3, _) => match count_max_heap.pop().unwrap() {
                (2, _) => CardHand::FullHouse(value),
                (1, _) => CardHand::ThreeOfAKind(value),
                _ => unreachable!(),
            },
            (2, _) => match count_max_heap.pop().unwrap() {
                (2, _) => CardHand::TwoPair(value),
                (1, _) => CardHand::OnePair(value),
                _ => unreachable!(),
            },
            (1, _) => CardHand::HighCard(value),
            _ => unreachable!(),
        });
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Display, Debug)]
pub struct CardHandWithJoker(CardHand);

impl FromStr for CardHandWithJoker {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            bail!("Invalid input for CardHand: {:?}", s);
        }
        let (value, count) = s
            .bytes()
            .map(|b| match b {
                b'2'..=b'9' => Ok((b - b'1') as u32),
                b'T' => Ok(9_u32),
                b'J' => Ok(0_u32),
                b'Q' => Ok(10_u32),
                b'K' => Ok(11_u32),
                b'A' => Ok(12_u32),
                _ => bail!("Invalid input for CardHand: {:?}", s),
            })
            .try_fold((0_u32, vec![0_u8; 13]), |(value, mut counts), digit| {
                let digit = digit?;
                counts[digit as usize] += 1_u8;
                return Ok::<_, anyhow::Error>((value * 13_u32 + digit, counts));
            })?;
        let joker_count = count[0];
        if joker_count == 5_u8 {
            return Ok(CardHandWithJoker(CardHand::FiveOfAKind(value)));
        }
        let mut count_max_heap: BinaryHeap<_> = count
            .into_iter()
            .enumerate()
            .skip(1)
            .filter(|(_, count)| count != &0_u8)
            .map(|(index, count)| (count, index))
            .collect();

        return Ok(CardHandWithJoker(match count_max_heap.pop().unwrap() {
            (5, _) => CardHand::FiveOfAKind(value),
            (4, _) => match joker_count {
                0 => CardHand::FourOfAKind(value),
                1 => CardHand::FiveOfAKind(value),
                _ => unreachable!(),
            },
            (3, _) => match count_max_heap.pop() {
                Some((2, _)) => CardHand::FullHouse(value),
                Some((1, _)) => match joker_count {
                    0 => CardHand::ThreeOfAKind(value),
                    1 => CardHand::FourOfAKind(value),
                    _ => unreachable!(),
                },
                Some(_) => unreachable!(),
                None => CardHand::FiveOfAKind(value),
            },
            (2, _) => match count_max_heap.pop() {
                Some((2, _)) => match joker_count {
                    0 => CardHand::TwoPair(value),
                    1 => CardHand::FullHouse(value),
                    _ => unreachable!(),
                },
                Some((1, _)) => match joker_count {
                    0 => CardHand::OnePair(value),
                    1 => CardHand::ThreeOfAKind(value),
                    2 => CardHand::FourOfAKind(value),
                    _ => unreachable!(),
                },
                Some(_) => unreachable!(),
                None => CardHand::FiveOfAKind(value),
            },
            (1, _) => match joker_count {
                0 => CardHand::HighCard(value),
                1 => CardHand::OnePair(value),
                2 => CardHand::ThreeOfAKind(value),
                3 => CardHand::FourOfAKind(value),
                4 => CardHand::FiveOfAKind(value),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }));
    }
}

impl FromStr for Day7Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards: Vec<_> = s
            .lines()
            .map(|line| line.split_whitespace())
            .map(|mut iter| {
                Ok::<_, anyhow::Error>((
                    CardHand::from_str(
                        iter.next()
                            .with_context(|| format!("Invalid input: {:?}", s))?,
                    )?,
                    <u32>::from_str(
                        iter.next()
                            .with_context(|| format!("Invalid input: {:?}", s))?,
                    )?,
                ))
            })
            .collect::<anyhow::Result<_>>()?;
        cards.sort_unstable();
        return Ok(Day7Part1(cards));
    }
}

impl ProblemSolver for Day7Part1 {
    type SolutionType = u32;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        return Ok(get_hands_rank(self.deref()));
    }
}

impl FromStr for Day7Part2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let mut cards: Vec<_> = s
            .lines()
            .map(|line| line.split_whitespace())
            .map(|mut iter| {
                Ok::<_, anyhow::Error>((
                    CardHandWithJoker::from_str(
                        iter.next()
                            .with_context(|| format!("Invalid input: {:?}", s))?,
                    )?,
                    <u32>::from_str(
                        iter.next()
                            .with_context(|| format!("Invalid input: {:?}", s))?,
                    )?,
                ))
            })
            .collect::<anyhow::Result<_>>()?;
        cards.sort_unstable();
        return Ok(Day7Part2(cards));
    }
}

impl ProblemSolver for Day7Part2 {
    type SolutionType = u32;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        return Ok(get_hands_rank(self.deref()));
    }
}

fn get_hands_rank<'a, H: 'a, I: IntoIterator<Item = &'a (H, u32)>>(hands: I) -> u32 {
    return hands
        .into_iter()
        .enumerate()
        .map(|(index, (_, bid))| (index as u32 + 1) * bid)
        .sum::<u32>();
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day7::{CardHand, CardHandWithJoker, Day7};
    use crate::solver::TwoPartsProblemSolver;
    use indoc::indoc;
    use std::str::FromStr;

    const SAMPLE_INPUT: &str = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
    "};

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day7::from_str(SAMPLE_INPUT)?.solve_1()?, 6440);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day7::from_str(SAMPLE_INPUT)?.solve_2()?, 5905);
        Ok(())
    }

    #[test]
    fn test_card_hand() -> anyhow::Result<()> {
        assert_eq!(CardHand::from_str("T55J5")?, CardHand::ThreeOfAKind(235706));
        Ok(())
    }

    #[test]
    fn test_card_hand_with_joker() -> anyhow::Result<()> {
        assert_eq!(
            CardHandWithJoker::from_str("T55J5")?,
            CardHandWithJoker(CardHand::FourOfAKind(266517))
        );
        Ok(())
    }
}
