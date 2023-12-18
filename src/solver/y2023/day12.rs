use crate::solver::{share_struct_solver, ProblemSolver};
use anyhow::Context;
use derive_more::{Deref, Display, FromStr};

use std::rc::Rc;
use std::sync::Mutex;
use thiserror::Error;

share_struct_solver!(Day12, Day12Part1, Day12Part2);

pub struct Day12Part1 {
    springs: Mutex<Vec<Spring>>,
}

struct Spring {
    spring_statuses: Vec<SpringSectionStatus>,
    damaged_count: Vec<u8>,
    min_len_required: Vec<u8>,
    dp: Vec<Vec<Option<usize>>>,
}

impl Spring {
    fn new(spring_statuses: Vec<SpringSectionStatus>, damaged_count: Vec<u8>) -> Self {
        let last_damaged_pos_option =
            spring_statuses.iter().rposition(|&v| v == SpringSectionStatus::Damaged);
        let mut dp: Vec<Vec<Option<usize>>> =
            vec![vec![None; damaged_count.len()]; spring_statuses.len()];
        dp.push(vec![Some(0); damaged_count.len()]);
        dp.iter_mut().enumerate().for_each(|(idx, vec)| {
            vec.push(
                last_damaged_pos_option
                    .map(|last_damaged_pos| if idx <= last_damaged_pos { 0 } else { 1 })
                    .or(Some(1)),
            )
        });

        let min_len_required = damaged_count
            .iter()
            .rev()
            .scan(0_u8, |prefix_sum, x| {
                if *prefix_sum != 0 {
                    *prefix_sum += 1;
                }
                *prefix_sum += x;
                Some(*prefix_sum)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        return Self { spring_statuses, damaged_count, min_len_required, dp };
    }

    fn expand(&self, n: usize) -> Self {
        let mut spring_statuses = self.spring_statuses.clone();
        spring_statuses.push(SpringSectionStatus::Unknown);
        let new_spring_statuses_len = spring_statuses.len() * n - 1;
        spring_statuses =
            spring_statuses.into_iter().cycle().take(new_spring_statuses_len).collect();
        let new_damaged_count_len = self.damaged_count.len() * n;
        let damaged_count = self
            .damaged_count
            .iter()
            .map(Clone::clone)
            .cycle()
            .take(new_damaged_count_len)
            .collect();
        return Spring::new(spring_statuses, damaged_count);
    }
}

impl Spring {
    fn combination_count(
        &mut self,
        spring_section_idx: usize,
        damaged_count_idx: usize,
    ) -> anyhow::Result<usize> {
        if let Some(res) = self.dp[spring_section_idx][damaged_count_idx] {
            return Ok(res);
        }

        let computed_val = self.spring_statuses[spring_section_idx..]
            .iter()
            .position(|&v| v == SpringSectionStatus::Damaged || v == SpringSectionStatus::Unknown)
            .map(|damaged_start_idx_offset| -> anyhow::Result<_> {
                let damaged_start_idx = spring_section_idx + damaged_start_idx_offset;
                if self.spring_statuses.len() - damaged_start_idx
                    < self.min_len_required[damaged_count_idx] as usize
                {
                    return Ok(0_usize);
                }
                let operational_should_start_idx =
                    damaged_start_idx + self.damaged_count[damaged_count_idx] as usize;

                let mut sum = 0;

                if self.spring_statuses[damaged_start_idx + 1..operational_should_start_idx]
                    .iter()
                    .all(|&v| {
                        v == SpringSectionStatus::Damaged || v == SpringSectionStatus::Unknown
                    })
                {
                    let advanced_damaged_count_idx = damaged_count_idx + 1;
                    if operational_should_start_idx == self.spring_statuses.len() {
                        sum += 1;
                    } else if self.spring_statuses[operational_should_start_idx]
                        == SpringSectionStatus::Unknown
                        || self.spring_statuses[operational_should_start_idx]
                            == SpringSectionStatus::Operational
                    {
                        sum += self.combination_count(
                            operational_should_start_idx + 1,
                            advanced_damaged_count_idx,
                        )?
                    }
                }

                sum += if self.spring_statuses[damaged_start_idx] == SpringSectionStatus::Unknown {
                    self.combination_count(damaged_start_idx + 1, damaged_count_idx)?
                } else {
                    0_usize
                };

                Ok(sum)
            })
            .transpose()?
            .or(Some(0));

        let res = computed_val.as_ref().unwrap().clone();
        self.dp[spring_section_idx][damaged_count_idx] = computed_val;
        return Ok(res);
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Display, Hash)]
enum SpringSectionStatus {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cannot convert {:?} to ngStatus", <char>::from(*.0))]
    InvalidSpringStatusChar(u8),
}

impl TryFrom<u8> for SpringSectionStatus {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> anyhow::Result<Self> {
        Ok(match value {
            b'.' => SpringSectionStatus::Operational,
            b'#' => SpringSectionStatus::Damaged,
            b'?' => SpringSectionStatus::Unknown,
            _ => Err(Error::InvalidSpringStatusChar(value))?,
        })
    }
}

#[derive(Deref)]
pub struct Day12Part2(Rc<Day12Part1>);

impl FromStr for Day12Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let springs = s
            .lines()
            .map(|s_part| {
                let (left, right) =
                    s_part.split_once(' ').with_context(|| format!("Invalid line {:?}", s_part))?;
                let spring_statuses = left
                    .bytes()
                    .map(SpringSectionStatus::try_from)
                    .collect::<anyhow::Result<_>>()?;
                let damaged_count = right
                    .split(',')
                    .map(<u8>::from_str)
                    .map(|r| r.map_err(anyhow::Error::from))
                    .collect::<anyhow::Result<_>>()?;
                return Ok::<_, anyhow::Error>(Spring::new(spring_statuses, damaged_count));
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        return Ok(Day12Part1 { springs: Mutex::new(springs) });
    }
}

impl ProblemSolver for Day12Part1 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        return self.springs.lock().unwrap().iter_mut().map(|s| s.combination_count(0, 0)).sum();
    }
}

impl ProblemSolver for Day12Part2 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        return self
            .springs
            .lock()
            .unwrap()
            .iter()
            .map(|s| s.expand(5))
            .map(|mut s| (&mut s).combination_count(0, 0))
            .sum();
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day12::{Day12, Spring, SpringSectionStatus};
    use crate::solver::TwoPartsProblemSolver;

    use indoc::indoc;

    use std::str::FromStr;

    const SAMPLE_INPUT_1: &str = indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
    "};

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day12::from_str(SAMPLE_INPUT_1)?.solve_1()?, 21);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        Ok(())
    }

    #[test]
    fn test_expand() -> anyhow::Result<()> {
        assert_eq!(
            Spring::new(
                ".??..??...?##."
                    .bytes()
                    .map(SpringSectionStatus::try_from)
                    .collect::<anyhow::Result<_>>()?,
                vec![1, 1, 3],
            )
            .expand(5)
            .combination_count(0, 0)?,
            16384
        );
        Ok(())
    }
}
