use crate::solver::{share_struct_solver, ProblemSolver};
use crate::utils::int_range::IntRange;
use anyhow::{anyhow, ensure};
use derive_more::{Deref, FromStr};
use derive_new::new;
use dyn_iter::{DynIter, IntoDynIterator};
use itertools::Itertools;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter;
use std::rc::Rc;

share_struct_solver!(Day22, Day22Part1, Day22Part2);

type BitSet = bit_set::BitSet<usize>;

#[derive(new, Debug)]
pub struct Day22Part1 {
    brick_supported_by: Vec<BitSet>,
    brick_supporting: Vec<BitSet>,
}

type BrickIdx = usize;
type BrickHeight = u16;

#[derive(Debug)]
enum Brick {
    XBar(IntRange<u16>, u16, u16),
    YBar(u16, IntRange<u16>, u16),
    ZBar(u16, u16, IntRange<u16>),
    Cube(u16, u16, u16),
}

impl FromStr for Brick {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once('~').ok_or_else(|| anyhow!("Cannot find '~' in {:?}", s)).and_then(
            |(left, right)| {
                let mut left_iter = left.split(',');
                let (x1, y1, z1) = left_iter
                    .next()
                    .and_then(|x| {
                        left_iter.next().and_then(|y| left_iter.next().map(|z| (x, y, z)))
                    })
                    .ok_or_else(|| anyhow!("Cannot parse (x, y, z) from {:?}", left))
                    .and_then(|(x, y, z)| {
                        Ok((<u16>::from_str(x)?, <u16>::from_str(y)?, <u16>::from_str(z)?))
                    })?;
                let mut right_iter = right.split(',');
                let (x2, y2, z2) = right_iter
                    .next()
                    .and_then(|x| {
                        right_iter.next().and_then(|y| right_iter.next().map(|z| (x, y, z)))
                    })
                    .ok_or_else(|| anyhow!("Cannot parse (x, y, z) from {:?}", right))
                    .and_then(|(x, y, z)| {
                        Ok((<u16>::from_str(x)?, <u16>::from_str(y)?, <u16>::from_str(z)?))
                    })?;
                if x1 != x2 {
                    ensure!(
                        y1 == y2 && z1 == z2,
                        "Cannot have 2 pair of different value axis {:?}",
                        s
                    );
                    Ok(Brick::XBar(IntRange::new_unknown_order(x1, x2), y1, z1))
                } else if y1 != y2 {
                    ensure!(z1 == z2, "Cannot have 2 pair of different value axis {:?}", s);
                    Ok(Brick::YBar(x1, IntRange::new_unknown_order(y1, y2), z1))
                } else if z1 != z2 {
                    Ok(Brick::ZBar(x1, y1, IntRange::new_unknown_order(z1, z2)))
                } else {
                    Ok(Brick::Cube(x1, y1, z1))
                }
            },
        )
    }
}

impl Brick {
    #[allow(dead_code)]
    fn to_str(&self) -> String {
        match self {
            Brick::XBar(x, y, z) => format!("{},{},{}~{},{},{}", x.start, y, z, x.end, y, z),
            Brick::YBar(x, y, z) => format!("{},{},{}~{},{},{}", x, y.start, z, x, y.end, z),
            Brick::ZBar(x, y, z) => format!("{},{},{}~{},{},{}", x, y, z.start, x, y, z.end),
            Brick::Cube(x, y, z) => format!("{},{},{}~{},{},{}", x, y, z, x, y, z),
        }
    }

    fn x_y_iter(&self) -> DynIter<(u16, u16)> {
        match self {
            Brick::XBar(x, y, _) => (x.start..=x.end).map(|x| (x, *y)).into_dyn_iter(),
            Brick::YBar(x, y, _) => (y.start..=y.end).map(|y| (*x, y)).into_dyn_iter(),
            Brick::ZBar(x, y, _) => iter::once((*x, *y)).into_dyn_iter(),
            Brick::Cube(x, y, _) => iter::once((*x, *y)).into_dyn_iter(),
        }
    }

    #[allow(dead_code)]
    fn x_y_z_iter(&self) -> DynIter<(u16, u16, u16)> {
        match self {
            Brick::XBar(x, y, z) => (x.start..=x.end).map(|x| (x, *y, *z)).into_dyn_iter(),
            Brick::YBar(x, y, z) => (y.start..=y.end).map(|y| (*x, y, *z)).into_dyn_iter(),
            Brick::ZBar(x, y, z) => (z.start..=z.end).map(|z| (*x, *y, z)).into_dyn_iter(),
            Brick::Cube(x, y, z) => iter::once((*x, *y, *z)).into_dyn_iter(),
        }
    }

    fn get_bottom(&self) -> u16 {
        match self {
            Brick::XBar(_, _, z) => *z,
            Brick::YBar(_, _, z) => *z,
            Brick::ZBar(_, _, z) => z.start,
            Brick::Cube(_, _, z) => *z,
        }
    }

    fn get_height(&self) -> u16 {
        match self {
            Brick::XBar(_, _, _) => 1,
            Brick::YBar(_, _, _) => 1,
            Brick::ZBar(_, _, z) => 1 + z.end - z.start,
            Brick::Cube(_, _, _) => 1,
        }
    }

    #[allow(dead_code)]
    fn set_height(&self, target_height: u16) -> Self {
        assert!(target_height <= self.get_bottom());
        match self {
            Brick::XBar(x, y, _) => Brick::XBar(*x, *y, target_height),
            Brick::YBar(x, y, _) => Brick::YBar(*x, *y, target_height),
            Brick::ZBar(x, y, z) => Brick::ZBar(
                *x,
                *y,
                IntRange::new_unknown_order(target_height, target_height + (z.end - z.start)),
            ),
            Brick::Cube(x, y, _) => Brick::Cube(*x, *y, target_height),
        }
    }
}

#[derive(Deref)]
pub struct Day22Part2(Rc<Day22Part1>);

impl FromStr for Day22Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let mut bricks = s.lines().map(Brick::from_str).collect::<anyhow::Result<Vec<_>>>()?;
        bricks.sort_unstable_by_key(|brick| brick.get_bottom());

        let mut height_map = HashMap::<(u16, u16), (BrickHeight, BrickIdx)>::default();
        let brick_supported_by = bricks
            .into_iter()
            .enumerate()
            .map(|(brick_idx, brick)| {
                let (supports, height) =
                    brick.x_y_iter().filter_map(|(x, y)| height_map.get(&(x, y))).fold(
                        (BitSet::default(), 0_u16),
                        |(mut supports, curr_max_height), (height, brick_id)| match height
                            .cmp(&curr_max_height)
                        {
                            Ordering::Less => (supports, curr_max_height),
                            Ordering::Equal => {
                                supports.insert(*brick_id);
                                (supports, curr_max_height)
                            }
                            Ordering::Greater => {
                                supports.clear();
                                supports.insert(*brick_id);
                                (supports, *height)
                            }
                        },
                    );
                let height = height + brick.get_height();
                brick.x_y_iter().for_each(|xy| {
                    height_map.insert(xy, (height, brick_idx));
                });
                supports
            })
            .collect_vec();
        let mut brick_supporting = vec![BitSet::default(); brick_supported_by.len()];
        brick_supported_by.iter().enumerate().for_each(|(brick_id, supported_by_ids)| {
            supported_by_ids.iter().for_each(|supporter_id| {
                brick_supporting[supporter_id].insert(brick_id);
            })
        });

        Ok(Day22Part1 { brick_supported_by, brick_supporting })
    }
}

impl ProblemSolver for Day22Part1 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        Ok(self
            .brick_supporting
            .iter()
            .filter(|supporting| {
                supporting.iter().all(|bid| self.brick_supported_by[bid].len() > 1)
            })
            .count())
    }
}

impl ProblemSolver for Day22Part2 {
    type SolutionType = usize;

    fn solve(&self) -> anyhow::Result<Self::SolutionType> {
        let len = self.brick_supported_by.len();
        // destroyed_list[i] is set of brick it will also destroy if brick[i] is destroyed
        let destroyed_list = (0..len)
            .map(|i| {
                let mut map = BitSet::default();
                map.insert(i);
                RefCell::new(map)
            })
            .collect_vec();

        // affected_list[i] is set of all dependent brick of brick[i], meaning they MAY be
        // destroyed if brick[i] is destroyed.
        let mut affected_list =
            self.brick_supporting.iter().cloned().map(RefCell::new).collect_vec();
        self.brick_supporting.iter().enumerate().rev().for_each(|(brick_id, supporting)| {
            let affected = &mut affected_list[brick_id].borrow_mut();
            supporting.iter().for_each(|i| affected.union_with(&affected_list[i].borrow()))
        });

        destroyed_list.iter().rev().for_each(|destroyed| {
            // Since the supporter of a brick id always have smaller id, we can iterate
            // through the affected list once in increasing order of brick id, adding destroy[id]
            // if all the brick supporting that brick is in destroyed. And since affected_list[i]
            // always >= i, we create destroy in reverse order so we can get the previously
            // computed destroy[id].
            affected_list.pop().unwrap().into_inner()
                .iter()
                .filter(|affected_id| {
                    self.brick_supported_by[*affected_id].is_subset(&*destroyed.borrow())
                })
                .for_each(|destroy_id| {
                    let mut destroy_mut = destroyed.borrow_mut();
                    destroy_mut.union_with(&*destroyed_list[destroy_id].borrow())
                });
        });

        Ok(destroyed_list.iter().map(|destroyed| destroyed.borrow().len() - 1).sum())
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day22::Day22;

    use indoc::indoc;

    use crate::solver::TwoPartsProblemSolver;
    use std::str::FromStr;

    const SAMPLE_INPUT_1: &str = indoc! {r"
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
    "};

    #[test]
    fn test_solve_1() -> anyhow::Result<()> {
        assert_eq!(Day22::from_str(SAMPLE_INPUT_1)?.solve_1()?, 5);
        Ok(())
    }

    #[test]
    fn test_solve_2() -> anyhow::Result<()> {
        assert_eq!(Day22::from_str(SAMPLE_INPUT_1)?.solve_2()?, 7);
        Ok(())
    }
}
