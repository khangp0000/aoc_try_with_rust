use crate::solver::TwoPartsProblemSolver;
use anyhow::Result;
use anyhow::{bail, Context};
use std::cell::OnceCell;
use std::cmp::min;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Day3 {
    board: Vec<Vec<u8>>,
}

impl FromStr for Day3 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Day3 { board: s.lines().map(str::as_bytes).map(<[u8]>::to_vec).collect() });
    }
}

impl TwoPartsProblemSolver for Day3 {
    type Solution1Type = usize;
    type Solution2Type = usize;

    fn solve_1(&self) -> Result<usize> {
        (0..self.board.len()).map(|i| self.process_line_1(i)).sum()
    }

    fn solve_2(&self) -> Result<usize> {
        let mut container = HashMap::new();

        (0..self.board.len()).try_for_each(|i| self.process_line_2(&mut container, i))?;

        let sum_prod: usize = container
            .iter()
            .filter(|(_, vals)| vals.len() == 2)
            .map(|(_, vals)| vals.iter().product::<usize>())
            .sum();
        Ok(sum_prod)
    }
}

impl Day3 {
    fn process_line_1(&self, idx: usize) -> Result<usize> {
        let mut sum = 0_usize;
        let line = self.board.get(idx).with_context(|| format!("Invalid line number {}", idx))?;
        let mut curr_idx = 0;
        while curr_idx < line.len() {
            if let Some(first_digit_idx_from_curr_idx) =
                line[curr_idx..].iter().position(<u8>::is_ascii_digit)
            {
                let left = curr_idx + first_digit_idx_from_curr_idx;
                let right;
                if let Some(int_len_minus_1) =
                    line[(left + 1)..].iter().position(|c| !c.is_ascii_digit())
                {
                    right = left + int_len_minus_1 + 1;
                } else {
                    right = line.len();
                }

                if (left > 0 && line[left - 1..left].iter().any(is_symbol))
                    || (right < line.len() && line[right..right + 1].iter().any(is_symbol))
                    || (idx > 0
                        && (self.board[idx - 1][(match left {
                            0 => 0,
                            _ => left - 1,
                        })
                            ..min(self.board[idx - 1].len(), right + 1)])
                            .iter()
                            .any(is_symbol))
                    || (idx < (self.board.len() - 1)
                        && (self.board[idx + 1][(match left {
                            0 => 0,
                            _ => left - 1,
                        })
                            ..min(self.board[idx + 1].len(), right + 1)])
                            .iter()
                            .any(is_symbol))
                {
                    sum += parse_usize_str_from_bytes(&line[left..right])?;
                }

                curr_idx = right;
            } else {
                curr_idx = line.len();
            }
        }
        Ok(sum)
    }

    fn process_line_2(
        &self,
        container: &mut HashMap<(usize, usize), Vec<usize>>,
        idx: usize,
    ) -> Result<()> {
        let line = &self.board[idx];
        let mut curr_idx = 0;
        while curr_idx < line.len() {
            if let Some(first_digit_idx_from_curr_idx) =
                line[curr_idx..].iter().position(<u8>::is_ascii_digit)
            {
                let left = curr_idx + first_digit_idx_from_curr_idx;
                let right;
                if let Some(int_len_minus_1) =
                    line[(left + 1)..].iter().position(|c| !c.is_ascii_digit())
                {
                    right = left + int_len_minus_1 + 1;
                } else {
                    right = line.len();
                }

                let value = OnceCell::new();
                let value_init_f = || parse_usize_str_from_bytes(&line[left..right]).unwrap();

                let c_left = if left == 0_usize { 0_usize } else { left - 1 };

                if idx > 0 {
                    let prev_line = &self.board[idx - 1];
                    let line_len = prev_line.len();
                    let c_right = if right == line_len { line_len } else { right + 1 };
                    prev_line[c_left..c_right]
                        .iter()
                        .enumerate()
                        .filter(|(_, &value)| value == b'*')
                        .map(|(index, _)| index + c_left)
                        .for_each(|index| {
                            container
                                .entry((index, idx - 1))
                                .or_default()
                                .push(*value.get_or_init(value_init_f))
                        });
                }

                if idx < self.board.len() - 1 {
                    let next_line = &self.board[idx + 1];
                    let line_len = next_line.len();
                    let c_right = if right == line_len { line_len } else { right + 1 };
                    next_line[c_left..c_right]
                        .iter()
                        .enumerate()
                        .filter(|(_, &value)| value == b'*')
                        .map(|(index, _)| index + c_left)
                        .for_each(|index| {
                            container
                                .entry((index, idx + 1))
                                .or_default()
                                .push(*value.get_or_init(value_init_f))
                        });
                }

                if left > 0 && line[left - 1] == b'*' {
                    container
                        .entry((left - 1, idx))
                        .or_default()
                        .push(*value.get_or_init(value_init_f));
                }

                if right < line.len() && line[right] == b'*' {
                    container
                        .entry((right, idx))
                        .or_default()
                        .push(*value.get_or_init(value_init_f));
                }

                curr_idx = right;
            } else {
                curr_idx = line.len();
            }
        }
        Ok(())
    }
}

fn is_symbol(c: &u8) -> bool {
    !c.is_ascii_digit() && *c != b'.'
}

fn parse_usize_str_from_bytes(input: &[u8]) -> anyhow::Result<usize> {
    let mut res = 0_usize;
    for value in input {
        let curr_val = (*value - b'0') as usize;
        if curr_val > 9_usize {
            bail!("Invalid digit byte");
        }
        res *= 10;
        res += curr_val;
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day3::Day3;
    use crate::solver::TwoPartsProblemSolver;
    use indoc::indoc;
    use std::str::FromStr;

    const SAMPLE_INPUT: &str = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
    "};

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day3::from_str(SAMPLE_INPUT)?.solve_1()?, 4361);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day3::from_str(SAMPLE_INPUT)?.solve_2()?, 467835);
        Ok(())
    }
}
