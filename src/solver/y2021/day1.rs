use std::str::FromStr;

use anyhow::Result;

use crate::solver::TwoPartsProblemSolver;

pub struct Day1 {
    report: Vec<u32>,
}

impl FromStr for Day1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Day1 {
            report: s
                .lines()
                .map(<u32>::from_str)
                .map(|r| r.map_err(anyhow::Error::from))
                .collect::<Result<_>>()?,
        })
    }
}

impl TwoPartsProblemSolver for Day1 {
    type Solution1Type = usize;
    type Solution2Type = usize;
    fn solve_1(&self) -> Result<usize> {
        let report_slice = self.report.as_slice();
        return Ok(report_slice[1..]
            .iter()
            .zip(report_slice[..report_slice.len() - 1].iter())
            .filter(|&(&l, &r)| l > r)
            .count());
    }

    fn solve_2(&self) -> Result<usize> {
        let report_slice = self.report.as_slice();
        return Ok(report_slice[3..]
            .iter()
            .zip(report_slice[..report_slice.len() - 3].iter())
            .filter(|&(&l, &r)| l > r)
            .count());
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Result;
    use indoc::indoc;

    use crate::solver::y2021::day1::Day1;
    use crate::solver::TwoPartsProblemSolver;

    const SAMPLE_INPUT: &str = indoc! {"
            199
            200
            208
            210
            200
            207
            240
            269
            260
            263
    "};

    #[test]
    fn test_sample_1() -> Result<()> {
        assert_eq!(Day1::from_str(SAMPLE_INPUT)?.solve_1()?, 7);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> Result<()> {
        assert_eq!(Day1::from_str(SAMPLE_INPUT)?.solve_2()?, 5);
        Ok(())
    }
}
