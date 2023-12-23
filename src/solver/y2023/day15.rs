use crate::solver::{combine_solver, ProblemSolver};
use anyhow::bail;
use derive_more::{Deref, FromStr};
use linked_hash_map::LinkedHashMap;
use std::fmt::Debug;

combine_solver!(Day15, Day15Part1, Day15Part2);

#[derive(Deref, Debug)]
pub struct Day15Part1(Box<[Box<[u8]>]>);

#[derive(Deref, Debug)]
pub struct Day15Part2 {
    map: LinkedHashMap<Box<[u8]>, u8>,
}

fn my_hash(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0_u8, move |hash, val| ((hash as u16 + *val as u16) * 17) as u8)
}

impl FromStr for Day15Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        Ok(Day15Part1(s.trim().as_bytes().split(|b| *b == b',').map(Box::from).collect()))
    }
}

impl ProblemSolver for Day15Part1 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        Ok(self.iter().map(|b| my_hash(b) as usize).sum::<usize>())
    }
}

impl FromStr for Day15Part2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = LinkedHashMap::new();
        s.trim().as_bytes().split(|b| *b == b',').try_for_each(|b| {
            let len = b.len();
            match b[len - 1] {
                b'1'..=b'9' => {
                    let val = b[len - 1] - b'0';

                    // insert will move existing to front, use entry to avoid that
                    map.entry(Box::<[u8]>::from(&b[0..len - 2]))
                        .and_modify(|v| *v = val)
                        .or_insert(val);
                    Ok(())
                }
                b'-' => {
                    map.remove(&b[0..len - 1]);
                    Ok(())
                }
                c => bail!("Unknown ending character {:?}", c as char),
            }
        })?;
        Ok(Day15Part2 { map })
    }
}

impl ProblemSolver for Day15Part2 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
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
    use crate::solver::y2023::day15::{my_hash, Day15};
    use crate::solver::TwoPartsProblemSolver;

    use indoc::indoc;

    use std::str::FromStr;

    const SAMPLE_INPUT_1: &str = indoc! {"
            rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    "};

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day15::from_str(SAMPLE_INPUT_1)?.solve_1()?, 1320);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day15::from_str(SAMPLE_INPUT_1)?.solve_2()?, 145);
        Ok(())
    }

    #[test]
    fn test_hash() -> anyhow::Result<()> {
        assert_eq!(my_hash("HASH".as_bytes()), 52_u8);
        Ok(())
    }
}
