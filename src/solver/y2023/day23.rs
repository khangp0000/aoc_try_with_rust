use std::cell::RefCell;
use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::ops::ControlFlow;
use std::rc::Rc;

use anyhow::{anyhow, ensure, Context, Result};
use derive_more::{Deref, FromStr};
use derive_new::new;
use indexmap::{IndexMap, IndexSet};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::solver::{share_struct_solver, ProblemSolver};
use crate::utils::graph::try_dfs;
use crate::utils::grid::grid_2d_bitvec::Grid2dBitVec;
use crate::utils::grid::Grid2d;

share_struct_solver!(Day23, Day23Part1, Day23Part2);

type BitSet = bit_set::BitSet<usize>;

#[derive(new, Deref, Debug)]
pub struct Day23Part1(Vec<(EdgeMap, ParentNodes)>);

#[derive(Deref)]
pub struct Day23Part2(Rc<Day23Part1>);

type EdgeMap = HashMap<NodeId, EdgeLen>;
type ParentNodes = BitSet;
type NodeId = usize;
type EdgeLen = usize;

impl FromStr for Day23Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let grid: Grid2dBitVec = Grid2dBitVec::try_new(s.lines().map(|line| {
            line.bytes().map(|b| match b {
                b'^' | b'<' => {
                    Err(anyhow!("Failed to assume you can only go east or south at intersection."))
                }
                b'#' => Ok(true),
                b'.' | b'v' | b'>' => Ok(false),
                _ => Err(anyhow!("Failed to parse char: {:?}", b as char)),
            })
        }))?;
        ensure!(grid[(0, 0..)].all(), "Failed to ensure left wall");
        ensure!(grid[(grid.width() - 1, 0..)].all(), "Failed to ensure left wall");
        ensure!(grid[(2.., 0)].all(), "Failed to ensure top wall");
        ensure!(
            grid[(..grid.height() - 3, grid.height() - 1)].all(),
            "Failed to ensure bottom wall"
        );
        let mut all_nodes = vec![((1, 0), 0)];
        let mut intersection_nodes = IndexMap::new();
        intersection_nodes.insert(0, (1, 0));
        let grid_ref = &grid;
        let intersection_nodes_iter = grid
            .rows()
            .enumerate()
            .skip(1)
            .take(grid.height() - 2)
            .flat_map(|(y, row_slice)| {
                (1..grid.width() - 1).filter(|x| !row_slice[*x]).filter_map(move |x| {
                    let mut count_v = 0_u8;
                    if !grid_ref[(x, y - 1)] {
                        count_v += 1;
                    }
                    if !grid_ref[(x, y + 1)] {
                        count_v += 1;
                    }
                    if count_v > 0 {
                        let mut count_h = 0_u8;
                        if !grid_ref[(x - 1, y)] {
                            count_h += 1;
                        }
                        if !grid_ref[(x + 1, y)] {
                            count_h += 1;
                        }
                        if count_h > 0 { Some((x, y, count_h + count_v)) } else { None }
                    } else {
                        None
                    }
                })
            })
            .filter_map(|(x, y, count_path)| {
                let idx = all_nodes.len();
                all_nodes.push(((x, y), idx));
                if count_path > 2 { Some((idx, (x, y))) } else { None }
            });

        intersection_nodes.extend(intersection_nodes_iter);
        let end_node_idx = all_nodes.len();
        let end_node_coord = (grid.width() - 2, grid.height() - 1);
        assert!(intersection_nodes.insert(end_node_idx, end_node_coord).is_none());
        all_nodes.push((end_node_coord, end_node_idx));

        let mut edges = HashMap::new();
        all_nodes
            .windows(2)
            .filter_map(|node_pair| {
                let ((x1, y1), n_id1) = node_pair[0];
                let ((x2, y2), n_id2) = node_pair[1];
                if y1 == y2 {
                    let slice = &grid[(x1..x2, y1)];
                    if slice.not_any() { Some(((n_id1, n_id2), slice.len())) } else { None }
                } else {
                    None
                }
            })
            .for_each(|((node1, node2), weight)| {
                if !intersection_nodes.contains_key(&node1)
                    && !intersection_nodes.contains_key(&node2)
                {
                    edges.entry(node2).or_insert_with(HashMap::new).insert(node1, weight);
                }
                edges.entry(node1).or_insert_with(HashMap::new).insert(node2, weight);
            });

        all_nodes.sort_unstable();
        all_nodes
            .windows(2)
            .filter_map(|node_pair| {
                let ((x1, y1), n_id1) = node_pair[0];
                let ((x2, y2), n_id2) = node_pair[1];
                if x1 == x2 {
                    let slice = &grid[(x1, y1..y2)];
                    if slice.not_any() { Some(((n_id1, n_id2), slice.len())) } else { None }
                } else {
                    None
                }
            })
            .for_each(|((node1, node2), weight)| {
                if !intersection_nodes.contains_key(&node1)
                    && !intersection_nodes.contains_key(&node2)
                {
                    edges.entry(node2).or_insert_with(HashMap::new).insert(node1, weight);
                }
                edges.entry(node1).or_insert_with(HashMap::new).insert(node2, weight);
            });

        let mut intersection_edges =
            vec![(HashMap::new(), BitSet::default()); intersection_nodes.len() - 1];

        try_dfs(0, (0, 0), |(start_id, len_from_start), s| {
            ControlFlow::<(), _>::Continue(
                edges
                    .get(s)
                    .iter()
                    .flat_map(|neighbor_edge| {
                        neighbor_edge.iter().map(|(neighbor_id, len)| {
                            if intersection_nodes.contains_key(neighbor_id) {
                                (
                                    ((*neighbor_id, 0), *neighbor_id),
                                    Some((*start_id, *neighbor_id, *len_from_start + *len)),
                                )
                            } else {
                                (((*start_id, *len_from_start + *len), *neighbor_id), None)
                            }
                        })
                    })
                    .map(|(neighbor, intersection_edge)| {
                        if let Some((start_node_id, end_node_id, len)) = intersection_edge {
                            let start_id = intersection_nodes.get_index_of(&start_node_id).unwrap();
                            let end_id = intersection_nodes.get_index_of(&end_node_id).unwrap();
                            intersection_edges[start_id]
                                .0
                                .entry(intersection_nodes.get_index_of(&end_node_id).unwrap())
                                .and_modify(|v| *v = max(*v, len))
                                .or_insert(len);
                            intersection_edges
                                .get_mut(end_id)
                                .map(|(_, parent_set)| parent_set.insert(start_id));
                        }
                        neighbor
                    })
                    .collect::<Vec<_>>(),
            )
        });

        // println!("{intersection_edges:?}");
        Ok(Day23Part1::new(intersection_edges))
    }
}

impl ProblemSolver for Day23Part1 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        let mut work = VecDeque::from([0_usize]);
        let last_node_id = self.len();
        let mut topological_nodes = IndexSet::with_capacity(last_node_id);
        let mut visited = BitSet::default();
        visited.insert(0);
        while let Some(node_id) = work.pop_front() {
            if !topological_nodes.contains(&node_id) {
                let (edge, _) = &self[node_id];
                edge.iter().for_each(|(k, _)| {
                    if self.get(*k).filter(|(_, p)| visited.is_superset(p)).is_some() {
                        work.push_back(*k);
                        visited.insert(*k);
                    };
                });
            }

            topological_nodes.insert(node_id);
        }

        ensure!(
            self.iter().all(|(_, p)| visited.is_superset(p)),
            "Failed to assume graph is directed acyclic"
        );
        let mut dist = vec![0; last_node_id + 1];
        topological_nodes.into_iter().for_each(|start| {
            let (edges, _) = &self[start];
            edges.iter().for_each(|(end, len)| {
                dist[*end] = max(dist[*end], dist[start] + len);
            })
        });

        Ok(*dist.last().unwrap())
    }
}

impl ProblemSolver for Day23Part2 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        let mut fence_node = BitSet::default();
        fence_node.insert(0);
        let mut work = Vec::from_iter(self[0].0.keys().copied());

        while let Some(node) = work.pop() {
            if let Some((edges, parent)) = self.get(node) {
                if !parent.is_disjoint(&fence_node) && edges.len() + parent.len() < 4 {
                    fence_node.insert(node);
                    work.extend(edges.iter().map(|(neighbor, _)| *neighbor))
                }
            }
        }

        let all_edges = self
            .iter()
            .enumerate()
            .map(|(current_node_id, (edges, parent))| {
                if let Some(len) = edges.get(&self.len()) {
                    assert_eq!(edges.len(), 1);
                    vec![(self.len(), *len)]
                } else {
                    let reverse_edges = parent
                        .iter()
                        .map(|neighbor| (neighbor, &self[neighbor]))
                        .filter_map(|(neighbor, (neighbor_edges, _))| {
                            if fence_node.contains(current_node_id) && fence_node.contains(neighbor)
                            {
                                None
                            } else {
                                Some((neighbor, neighbor_edges[&current_node_id]))
                            }
                        });
                    let edges = edges
                        .iter()
                        .map(|(dest, edge_len)| (*dest, *edge_len))
                        .chain(reverse_edges)
                        .collect::<Vec<_>>();
                    edges
                }
            })
            .collect::<Vec<_>>();

        if rayon::current_num_threads() > 1 {
            Day23Part2::find_longest_path_par(0, 0, &all_edges, BitSet::default())
        } else {
            Day23Part2::find_longest_path(0, 0, &all_edges, &RefCell::new(BitSet::default()))
        }
        .context("Cannot find path to end")
    }
}

impl Day23Part2 {
    fn find_longest_path(
        current_node_id: NodeId,
        len: EdgeLen,
        all_edges: &Vec<Vec<(NodeId, EdgeLen)>>,
        visited: &RefCell<BitSet>,
    ) -> Option<EdgeLen> {
        if current_node_id == all_edges.len() {
            return Some(len);
        }

        visited.borrow_mut().insert(current_node_id);

        let res = all_edges[current_node_id]
            .iter()
            .filter(|(dest, _)| !visited.borrow().contains(*dest))
            .map(|(neighbor, edge_len)| {
                Day23Part2::find_longest_path(*neighbor, *edge_len, all_edges, visited)
            })
            .max()
            .flatten()
            .map(|rest_len| len + rest_len);

        visited.borrow_mut().remove(current_node_id);
        res
    }

    fn find_longest_path_par(
        current_node_id: NodeId,
        len: EdgeLen,
        all_edges: &Vec<Vec<(NodeId, EdgeLen)>>,
        mut visited: BitSet,
    ) -> Option<EdgeLen> {
        if current_node_id == all_edges.len() {
            return Some(len);
        }

        visited.insert(current_node_id);

        let edges = all_edges[current_node_id]
            .iter()
            .filter(|(dest, _)| !visited.contains(*dest))
            .map(|(dest, edge_len)| (*dest, *edge_len))
            .collect::<Vec<_>>();

        edges
            .into_par_iter()
            .map(|(neighbor, edge_len)| {
                if all_edges.len() - visited.len() > 20 {
                    Day23Part2::find_longest_path_par(
                        neighbor,
                        edge_len,
                        all_edges,
                        visited.clone(),
                    )
                } else {
                    Day23Part2::find_longest_path(
                        neighbor,
                        edge_len,
                        all_edges,
                        &RefCell::new(visited.clone()),
                    )
                }
            })
            .max()
            .flatten()
            .map(|rest_len| len + rest_len)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Result;
    use indoc::indoc;

    use crate::solver::y2023::day23::Day23;
    use crate::solver::TwoPartsProblemSolver;

    const SAMPLE_INPUT_1: &str = indoc! {r"
            #.#####################
            #.......#########...###
            #######.#########.#.###
            ###.....#.>.>.###.#.###
            ###v#####.#v#.###.#.###
            ###.>...#.#.#.....#...#
            ###v###.#.#.#########.#
            ###...#.#.#.......#...#
            #####.#.#.#######.#.###
            #.....#.#.#.......#...#
            #.#####.#.#.#########v#
            #.#...#...#...###...>.#
            #.#.#v#######v###.###v#
            #...#.>.#...>.>.#.###.#
            #####v#.#.###v#.#.###.#
            #.....#...#...#.#.#...#
            #.#########.###.#.#.###
            #...###...#...#...#.###
            ###.###.#.###v#####v###
            #...#...#.#.>.>.#.>.###
            #.###.###.#.###.#.#v###
            #.....###...###...#...#
            #####################.#
    "};

    #[test]
    fn test_solve_1() -> Result<()> {
        assert_eq!(Day23::from_str(SAMPLE_INPUT_1)?.solve_1()?, 94);
        Ok(())
    }

    #[test]
    fn test_solve_2() -> Result<()> {
        assert_eq!(Day23::from_str(SAMPLE_INPUT_1)?.solve_2()?, 154);
        Ok(())
    }
}
