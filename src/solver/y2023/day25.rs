use std::cmp::Ordering;
use std::f64::consts::FRAC_1_SQRT_2;
use std::fmt::Debug;
use std::rc::Rc;

use anyhow::{anyhow, bail, Context, Result};
use bit_set::BitSet;
use derive_more::{Deref, FromStr};
use derive_new::new;
use indexmap::{IndexMap, IndexSet};
use rand::Rng;

use crate::solver::{share_struct_solver, ProblemSolver};
use crate::utils::WarningResult;

share_struct_solver!(Day25, Day25Part1, Day25Part2);

#[derive(new, Deref, Debug)]
pub struct Day25Part1(IndexMap<String, BitSet<usize>>);

#[derive(Deref, Debug)]
pub struct Day25Part2(Rc<Day25Part1>);

impl FromStr for Day25Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        s.lines()
            .try_fold(IndexMap::<_, BitSet<usize>>::default(), |mut neighbors, line| {
                let (key, vals) =
                    line.split_once(':').with_context(|| format!("Missing \':\' for {line:?}"))?;
                let key_entry = neighbors.entry(key.trim().to_owned());
                let key_idx = key_entry.index();
                key_entry.or_default();

                vals.split_whitespace().map(str::to_owned).try_for_each(|val| {
                    let entry = neighbors.entry(val);
                    let idx = entry.index();

                    match idx.cmp(&key_idx) {
                        Ordering::Less => {
                            entry.or_default().insert(key_idx);
                        }
                        Ordering::Greater => {
                            entry.or_default();
                            neighbors[key_idx].insert(idx);
                        }
                        Ordering::Equal => bail!("Detected self cycle at node {}", entry.key()),
                    }

                    // if idx < key_idx {
                    //     e.or_default().insert(key_idx);
                    // } else if key_idx < idx {
                    //     e.or_default();
                    //     neighbors[key_idx].insert(idx);
                    // } else {
                    //     bail!("Detected self cycle at node {}", e.key())
                    // }
                    Ok(())
                })?;
                Ok(neighbors)
            })
            .map(Day25Part1)
    }
}

impl ProblemSolver for Day25Part1 {
    type SolutionType = WarningResult<usize>;

    fn solve(&self) -> Result<Self::SolutionType> {
        let edges = self
            .values()
            .enumerate()
            .flat_map(|(start, ends)| ends.iter().map(move |end| ((start, end), 1)))
            .collect::<IndexMap<_, _>>();

        let rand = &mut rand::thread_rng();
        let contracted_node_count = vec![1; self.len()];
        for _ in 0..100 {
            if let Some((_, contracted_node_count)) =
                Self::fast_cut_3(rand, edges.clone(), contracted_node_count.clone())?
            {
                return Ok(WarningResult::new(contracted_node_count.into_iter().product(), "Since random is involve, runtime may varied"));
            }
        }

        bail!("Failed to find 3-cut after retry several times.");
    }
}

impl Day25Part1 {
    fn fast_cut_3<R: Rng>(
        rand: &mut R,
        edges: IndexMap<(usize, usize), usize>,
        contracted_node_count: Vec<usize>,
    ) -> Result<Option<(IndexMap<(usize, usize), usize>, Vec<usize>)>> {
        if contracted_node_count.len() <= 6 {
            return Self::contract_until(rand, edges, contracted_node_count, 2)
                .map_err(|_e| anyhow!("Contraction failed"))
                .map(|(edges, contracted_node_count)| {
                    if *edges.first().unwrap().1 == 3 {
                        Some((edges, contracted_node_count))
                    } else {
                        None
                    }
                });
        }
        let t = (contracted_node_count.len() as f64 * FRAC_1_SQRT_2 + 1.0).ceil() as usize;
        let first_try = Self::contract_until(rand, edges.clone(), contracted_node_count.clone(), t)
            .map_err(|_| anyhow!("Contraction failed"))
            .map(|(edges, contracted_node_count)| {
                Self::fast_cut_3(rand, edges, contracted_node_count)
            })??;

        if first_try.is_some() {
            Ok(first_try)
        } else {
            Self::contract_until(rand, edges.clone(), contracted_node_count.clone(), t)
                .map_err(|_| anyhow!("Contraction failed"))
                .map(|(edges, contracted_node_count)| {
                    Self::fast_cut_3(rand, edges, contracted_node_count)
                })?
        }
    }

    fn contract_until<R: Rng>(
        rand: &mut R,
        mut edges: IndexMap<(usize, usize), usize>,
        mut contracted_node_count: Vec<usize>,
        target_node_count: usize,
    ) -> Result<
        (IndexMap<(usize, usize), usize>, Vec<usize>),
        (IndexMap<(usize, usize), usize>, Vec<usize>),
    > {
        while contracted_node_count.len() > target_node_count {
            let res = Self::contract_random(rand, edges, contracted_node_count);
            if res.is_err() {
                return res;
            } else {
                (edges, contracted_node_count) = res.unwrap();
            }
        }

        Ok((edges, contracted_node_count))
    }

    fn contract_random<R: Rng>(
        rand: &mut R,
        edges: IndexMap<(usize, usize), usize>,
        contracted_node_count: Vec<usize>,
    ) -> Result<
        (IndexMap<(usize, usize), usize>, Vec<usize>),
        (IndexMap<(usize, usize), usize>, Vec<usize>),
    > {
        let sample_range = 0..edges.len();
        if sample_range.is_empty() {
            Err((edges, contracted_node_count))
        } else {
            Self::contract(rand.gen_range(sample_range), edges, contracted_node_count)
        }
    }

    fn contract(
        edge_idx: usize,
        mut edges: IndexMap<(usize, usize), usize>,
        mut contracted_node_count: Vec<usize>,
    ) -> Result<
        (IndexMap<(usize, usize), usize>, Vec<usize>),
        (IndexMap<(usize, usize), usize>, Vec<usize>),
    > {
        if let Some(((left, right), _)) = edges.swap_remove_index(edge_idx) {
            let len = edges.len();
            let edges = edges
                .into_iter()
                .map(|((mut l, mut r), c)| {
                    if r == right {
                        if l < left { ((l, left), c) } else { ((left, l), c) }
                    } else if l == right {
                        ((left, r - 1), c)
                    } else {
                        if l > right {
                            l -= 1;
                        }
                        if r > right {
                            r -= 1;
                        }
                        ((l, r), c)
                    }
                })
                .fold(IndexMap::with_capacity(len), |mut map, (edge, c)| {
                    map.entry(edge).and_modify(|v| *v += c).or_insert(c);
                    map
                });

            contracted_node_count[left] += contracted_node_count[right];
            contracted_node_count.remove(right);
            Ok((edges, contracted_node_count))
        } else {
            Err((edges, contracted_node_count))
        }
    }

    #[allow(dead_code)]
    fn hamilton_cycle(&self) -> IndexSet<usize> {
        let mut res = IndexSet::with_capacity(self.len());
        self.hamilton_cycle_step(0, &mut res);
        res
    }

    #[allow(dead_code)]
    fn hamilton_cycle_step(&self, current: usize, path: &mut IndexSet<usize>) -> bool {
        if self.len() == path.len() && current == 0 {
            return true;
        }

        if path.contains(&current) {
            return false;
        }

        path.insert(current);
        let res = self[current].iter().any(|next| self.hamilton_cycle_step(next, path));
        if !res {
            path.pop();
        }

        res
    }
}

impl ProblemSolver for Day25Part2 {
    type SolutionType = &'static str;

    fn solve(&self) -> Result<Self::SolutionType> {
        Ok("Ho ho ho, there is no part 2!!!! Merry christmas!!!")
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::str::FromStr;

    use anyhow::Result;
    use indoc::indoc;

    use crate::solver::y2023::day25::{Day25Part1, Day25Part2};
    use crate::solver::ProblemSolver;

    const SAMPLE_INPUT_1: &str = indoc! {"\
            jqt: rhn xhk nvd
            rsh: frs pzl lsr
            xhk: hfx
            cmg: qnr nvd lhk bvb
            rhn: xhk bvb hfx
            bvb: xhk hfx
            pzl: lsr hfx nvd
            qnr: nvd
            ntq: jqt hfx bvb xhk
            nvd: lhk
            lsr: lhk
            rzs: qnr cmg lsr rsh
            frs: qnr lhk lsr
    "};

    #[test]
    fn test_solve_1() -> Result<()> {
        assert_eq!(*Day25Part1::from_str(SAMPLE_INPUT_1)?.solve()?.deref(), 54);
        Ok(())
    }

    #[test]
    fn test_solve_2() -> Result<()> {
        assert_eq!(
            Day25Part2::from_str(SAMPLE_INPUT_1)?.solve()?,
            "Ho ho ho, there is no part 2!!!! Merry christmas!!!"
        );
        Ok(())
    }
}
