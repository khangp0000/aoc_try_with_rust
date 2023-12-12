use crate::solver::TwoPartsProblemSolver;
use anyhow::{anyhow, Context};
use bitvec::field::BitField;
use bitvec::order::{BitOrder, Msb0};
use bitvec::ptr::{BitRef, Mutability};
use bitvec::store::BitStore;
use bitvec::vec::BitVec;
use dyn_iter::DynIter;
use std::str::FromStr;

pub struct Day3 {
    report: Vec<BitVec<u32, Msb0>>,
}

impl FromStr for Day3 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Day3 {
            report: s
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '0' => Ok(false),
                            '1' => Ok(true),
                            _ => Err(anyhow!("Invalid character {}", c)),
                        })
                        .collect()
                })
                .collect::<Result<_, _>>()?,
        });
    }
}

impl TwoPartsProblemSolver for Day3 {
    type Solution1Type = u32;
    type Solution2Type = u32;

    fn solve_1(&self) -> anyhow::Result<u32> {
        let (gamma_bit, epsilon_bit) = self
            .report
            .iter()
            .map(|b| DynIter::new(b.iter().map(|b| if *b { 1_i32 } else { -1_i32 })))
            .reduce(|l, r| DynIter::new(l.zip(r).map(|(l_val, r_val)| l_val + r_val)))
            .with_context(|| format!("Failed to reduce, is the list empty?"))?
            .map(|val| {
                let more_common = val > 0;
                return (more_common, !more_common);
            })
            .unzip::<_, _, BitVec<u32, Msb0>, BitVec<u32, Msb0>>();

        let gamma = gamma_bit.load_be::<u32>();
        let epsilon = epsilon_bit.load_be::<u32>();

        Ok(gamma * epsilon)
    }

    fn solve_2(&self) -> anyhow::Result<u32> {
        let mut current_set = self
            .report
            .iter()
            .map(|val| (val, val.iter()))
            .collect::<Vec<_>>();
        while current_set.len() > 1 {
            current_set = get_next_set(current_set, false)?;
        }
        let o2: u32 = current_set[0].0.load_be();

        let mut current_set = self
            .report
            .iter()
            .map(|val| (val, val.iter()))
            .collect::<Vec<_>>();
        while current_set.len() > 1 {
            current_set = get_next_set(current_set, true)?;
        }
        let co2: u32 = current_set[0].0.load_be();
        Ok(o2 * co2)
    }
}

fn get_next_set<
    'a,
    M: Mutability,
    S: BitStore,
    O: BitOrder,
    T: Iterator<Item = BitRef<'a, M, S, O>>,
>(
    current_set: Vec<(&BitVec<S, O>, T)>,
    if_more_one_than_zero_keep_zero: bool,
) -> anyhow::Result<Vec<(&BitVec<S, O>, T)>> {
    let (next_set, compare_value) =
        current_set
            .into_iter()
            .map(|(val, mut iter)| {
                let next_bit = iter.next();
                return Ok::<(&BitVec<_, _>, _, _), anyhow::Error>((
                    val,
                    iter,
                    next_bit.context("Failed to get next bit").map(|b| {
                        if *b {
                            1_i32
                        } else {
                            -1_i32
                        }
                    })?,
                ));
            })
            .try_fold((Vec::new(), 0), |(mut vec, mut acc), r| {
                let (val, iter, one_value) = r?;
                vec.push((val, one_value, iter));
                acc += one_value;
                return Ok::<_, anyhow::Error>((vec, acc));
            })?;
    let more_one_than_zero = compare_value >= 0;
    return if more_one_than_zero {
        Ok(next_set
            .into_iter()
            .filter(|(_, one_val, _)| if_more_one_than_zero_keep_zero ^ (one_val > &0))
            .map(|(val, _, iter)| (val, iter))
            .collect())
    } else {
        Ok(next_set
            .into_iter()
            .filter(|(_, one_val, _)| if_more_one_than_zero_keep_zero ^ (one_val < &0))
            .map(|(val, _, iter)| (val, iter))
            .collect())
    };
}

#[cfg(all(test))]
mod tests {
    use crate::solver::y2021::day3::Day3;
    use crate::solver::TwoPartsProblemSolver;
    use anyhow::Result;
    use indoc::indoc;
    use std::str::FromStr;

    static SAMPLE_INPUT: &str = indoc! {"
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010
    "};

    #[test]
    fn test_sample_1() -> Result<()> {
        assert_eq!(Day3::from_str(SAMPLE_INPUT)?.solve_1()?, 198);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> Result<()> {
        assert_eq!(Day3::from_str(SAMPLE_INPUT)?.solve_2()?, 230);
        Ok(())
    }
}
