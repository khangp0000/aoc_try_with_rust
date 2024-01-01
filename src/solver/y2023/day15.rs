use std::fmt::Debug;

use anyhow::bail;
use anyhow::Result;
use derive_more::{Deref, FromStr};
use linked_hash_map::LinkedHashMap;

use crate::solver::{combine_solver, ProblemSolver};

combine_solver!(Day15, Day15Part1, Day15Part2);

#[derive(Deref, Debug)]
pub struct Day15Part1(Vec<String>);

#[derive(Deref, Debug)]
pub struct Day15Part2 {
    map: LinkedHashMap<String, u8>,
}

fn my_hash(s: &str) -> u8 {
    s.bytes().fold(0_u8, move |hash, val| ((hash as u16 + val as u16) * 17) as u8)
}

impl FromStr for Day15Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Day15Part1(s.trim().split(',').map(str::to_owned).collect()))
    }
}

impl ProblemSolver for Day15Part1 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        Ok(self.iter().map(|b| my_hash(b) as usize).sum::<usize>())
    }
}

impl FromStr for Day15Part2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut map = LinkedHashMap::new();
        s.trim().split(',').try_for_each(|b| {
            let len = b.len();
            let mut char_rev_iter = b.chars().rev();
            let last_char = char_rev_iter.next().unwrap();
            match last_char {
                '1'..='9' => {
                    let val = last_char as u8 - b'0';

                    // insert will move existing to front, use entry to avoid that
                    char_rev_iter.next().unwrap();
                    map.entry(char_rev_iter.rev().collect())
                        .and_modify(|v| *v = val)
                        .or_insert(val);
                    Ok(())
                }
                '-' => {
                    map.remove(&b[0..len - 1]);
                    Ok(())
                }
                c => bail!("Unknown ending character {:?}", { c }),
            }
        })?;
        Ok(Day15Part2 { map })
    }
}

impl ProblemSolver for Day15Part2 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        let mut counts = vec![1_usize; 256];
        Ok(self
            .iter()
            .map(|(slice, focal)| {
                let hash = my_hash(slice) as usize;
                let slot = &mut counts[hash];
                let res = (hash + 1) * (*slot) * (*focal as usize);
                *slot += 1;
                res
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Result;
    use indoc::indoc;

    use crate::solver::y2023::day15::{my_hash, Day15};
    use crate::solver::TwoPartsProblemSolver;

    const SAMPLE_INPUT_1: &str = indoc! {"
            rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    "};

    #[test]
    fn test_sample_1() -> Result<()> {
        assert_eq!(Day15::from_str(SAMPLE_INPUT_1)?.solve_1()?, 1320);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> Result<()> {
        assert_eq!(Day15::from_str(SAMPLE_INPUT_1)?.solve_2()?, 145);
        Ok(())
    }

    #[test]
    fn test_hash() -> Result<()> {
        assert_eq!(my_hash("HASH"), 52_u8);
        Ok(())
    }
}
