use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::iter;
use std::ops::ControlFlow;

#[derive(Debug)]
pub struct StateWithWeight<A, S, W: Ord> {
    accumulator: A,
    state: S,
    weight: W,
}

impl<A, S, W: Ord> From<StateWithWeight<A, S, W>> for (A, S, W) {
    fn from(value: StateWithWeight<A, S, W>) -> Self {
        (value.accumulator, value.state, value.weight)
    }
}

impl<A, S, W: Ord> Eq for StateWithWeight<A, S, W> {}

impl<A, S, W: Ord> PartialEq<Self> for StateWithWeight<A, S, W> {
    fn eq(&self, other: &Self) -> bool {
        self.weight.eq(&other.weight)
    }
}

impl<A, S, W: Ord> PartialOrd<Self> for StateWithWeight<A, S, W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<A, S: Eq + Hash, W: Ord> Ord for StateWithWeight<A, S, W> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

pub fn dfs<S, N, E, I, A, AF>(
    start: S,
    mut neighbor_fn: N,
    mut end_state_fn: E,
    acc_init: A,
    mut acc_fn: AF,
) -> Option<(A, S)>
where
    A: Clone,
    S: Eq + PartialEq + Hash + Debug,
    N: FnMut(&S) -> I,
    E: FnMut(&A, &S) -> bool,
    I: IntoIterator<Item = S>,
    AF: FnMut(&A, &S) -> A,
{
    try_dfs(start, acc_init, |acc, current_state| {
        let acc = acc_fn(acc, current_state);
        if end_state_fn(&acc, current_state) {
            return ControlFlow::Break(acc);
        }
        ControlFlow::Continue(
            neighbor_fn(current_state).into_iter().map(move |next_state| (acc.clone(), next_state)),
        )
    })
}

#[allow(dead_code)]
pub fn dfs_full<S, N, E, I, A, AF>(
    work_stack: Vec<(A, S)>,
    visited: HashSet<S>,
    mut neighbor_fn: N,
    mut end_state_fn: E,
    mut acc_fn: AF,
) -> ControlFlow<(A, S, Vec<(A, S)>, HashSet<S>), HashSet<S>>
where
    A: Clone,
    S: Eq + PartialEq + Hash + Debug,
    N: FnMut(&S) -> I,
    E: FnMut(&A, &S) -> bool,
    I: IntoIterator<Item = S>,
    AF: FnMut(&A, &S) -> A,
{
    try_dfs_full(work_stack, visited, |acc, current_state| {
        let acc = acc_fn(acc, current_state);
        if end_state_fn(&acc, current_state) {
            return ControlFlow::Break(acc);
        }
        ControlFlow::Continue(
            neighbor_fn(current_state).into_iter().map(move |next_state| (acc.clone(), next_state)),
        )
    })
}

pub fn try_dfs<S, A, C, R, I>(start: S, acc_init: A, compute_neighbor_fn: C) -> Option<(R, S)>
where
    A: Clone,
    S: Eq + PartialEq + Hash + Debug,
    C: FnMut(&A, &S) -> ControlFlow<R, I>,
    I: IntoIterator<Item = (A, S)>,
{
    match try_dfs_full(vec![(acc_init, start)], HashSet::default(), compute_neighbor_fn) {
        ControlFlow::Continue(_) => None,
        ControlFlow::Break((a, s, _, _)) => Some((a, s)),
    }
}

pub fn try_dfs_full<S, C, I, A, R>(
    mut work_stack: Vec<(A, S)>,
    mut visited: HashSet<S>,
    mut compute_neighbor_fn: C,
) -> ControlFlow<(R, S, Vec<(A, S)>, HashSet<S>), HashSet<S>>
where
    A: Clone,
    S: Eq + PartialEq + Hash + Debug,
    C: FnMut(&A, &S) -> ControlFlow<R, I>,
    I: IntoIterator<Item = (A, S)>,
{
    while let Some((acc, current_state)) = work_stack.pop() {
        if !visited.contains(&current_state) {
            match compute_neighbor_fn(&acc, &current_state) {
                ControlFlow::Continue(iter) => {
                    iter.into_iter().for_each(|item| work_stack.push(item))
                }
                ControlFlow::Break(b) => {
                    return ControlFlow::Break((b, current_state, work_stack, visited));
                }
            }

            visited.insert(current_state);
        }
    }

    ControlFlow::Continue(visited)
}

pub fn bfs<S, N, E, I, A, AF>(
    start: S,
    mut neighbor_fn: N,
    mut end_state_fn: E,
    acc_init: A,
    mut acc_fn: AF,
) -> Option<(A, S)>
where
    A: Clone,
    S: Eq + PartialEq + Hash + Debug,
    N: FnMut(&S) -> I,
    E: FnMut(&A, &S) -> bool,
    I: IntoIterator<Item = S>,
    AF: FnMut(&A, &S) -> A,
{
    try_bfs(start, acc_init, |acc, current_state| {
        let acc = acc_fn(acc, current_state);
        if end_state_fn(&acc, current_state) {
            return ControlFlow::Break(acc);
        }
        ControlFlow::Continue(
            neighbor_fn(current_state).into_iter().map(move |next_state| (acc.clone(), next_state)),
        )
    })
}

#[allow(dead_code)]
pub fn bfs_full<S, N, E, I, A, AF>(
    work_queue: VecDeque<(A, S)>,
    visited: HashSet<S>,
    mut neighbor_fn: N,
    mut end_state_fn: E,
    mut acc_fn: AF,
) -> ControlFlow<(A, S, VecDeque<(A, S)>, HashSet<S>), HashSet<S>>
where
    A: Clone,
    S: Eq + PartialEq + Hash + Debug,
    N: FnMut(&S) -> I,
    E: FnMut(&A, &S) -> bool,
    I: IntoIterator<Item = S>,
    AF: FnMut(&A, &S) -> A,
{
    try_bfs_full(work_queue, visited, |acc, current_state| {
        let acc = acc_fn(acc, current_state);
        if end_state_fn(&acc, current_state) {
            return ControlFlow::Break(acc);
        }
        ControlFlow::Continue(
            neighbor_fn(current_state).into_iter().map(move |next_state| (acc.clone(), next_state)),
        )
    })
}

pub fn try_bfs_full<S, C, I, A, R>(
    mut work_queue: VecDeque<(A, S)>,
    mut visited: HashSet<S>,
    mut compute_neighbor_fn: C,
) -> ControlFlow<(R, S, VecDeque<(A, S)>, HashSet<S>), HashSet<S>>
where
    A: Clone,
    S: Eq + PartialEq + Hash + Debug,
    C: FnMut(&A, &S) -> ControlFlow<R, I>,
    I: IntoIterator<Item = (A, S)>,
{
    while let Some((acc, current_state)) = work_queue.pop_front() {
        if !visited.contains(&current_state) {
            match compute_neighbor_fn(&acc, &current_state) {
                ControlFlow::Continue(iter) => {
                    iter.into_iter().for_each(|item| work_queue.push_back(item))
                }
                ControlFlow::Break(b) => {
                    return ControlFlow::Break((b, current_state, work_queue, visited));
                }
            }

            visited.insert(current_state);
        }
    }

    ControlFlow::Continue(visited)
}

pub fn try_bfs<S, A, C, R, I>(start: S, acc_init: A, compute_neighbor_fn: C) -> Option<(R, S)>
where
    A: Clone,
    S: Eq + PartialEq + Hash + Debug,
    C: FnMut(&A, &S) -> ControlFlow<R, I>,
    I: IntoIterator<Item = (A, S)>,
{
    match try_bfs_full(VecDeque::from([(acc_init, start)]), HashSet::default(), compute_neighbor_fn)
    {
        ControlFlow::Continue(_) => None,
        ControlFlow::Break((a, s, _, _)) => Some((a, s)),
    }
}

#[allow(dead_code)]
pub fn dijkstra<S, W, N, E, I, A, AF>(
    start: S,
    start_weight: W,
    neighbor_fn: N,
    end_state_fn: E,
    acc_init: A,
    acc_fn: AF,
) -> Option<(A, S, W)>
where
    A: Clone,
    S: Eq + PartialEq + Hash + Debug,
    W: Ord + Debug,
    N: FnMut(&S, &W) -> I,
    E: FnMut(&A, &S, &W) -> bool,
    I: IntoIterator<Item = (S, W)>,
    AF: FnMut(&A, &S, &W) -> A,
{
    dijkstra_starts_iter(
        iter::once((start, start_weight)),
        neighbor_fn,
        end_state_fn,
        acc_init,
        acc_fn,
    )
}

pub fn dijkstra_starts_iter<S, W, N, E, I, A, AF, SWI>(
    starts: SWI,
    neighbor_fn: N,
    end_state_fn: E,
    acc_init: A,
    acc_fn: AF,
) -> Option<(A, S, W)>
where
    A: Clone,
    S: Eq + PartialEq + Hash + Debug,
    W: Ord + Debug,
    N: FnMut(&S, &W) -> I,
    E: FnMut(&A, &S, &W) -> bool,
    I: IntoIterator<Item = (S, W)>,
    AF: FnMut(&A, &S, &W) -> A,
    SWI: IntoIterator<Item = (S, W)>,
{
    let mut work_heap = starts
        .into_iter()
        .map(|(start, start_weight)| {
            Reverse(StateWithWeight {
                accumulator: acc_init.clone(),
                state: start,
                weight: start_weight,
            })
        })
        .collect::<BinaryHeap<_>>();
    dijkstra_full(&mut work_heap, &mut HashSet::default(), neighbor_fn, end_state_fn, acc_fn)
}

pub fn dijkstra_full<S, W, N, E, I, A, AF>(
    work_heap: &mut BinaryHeap<Reverse<StateWithWeight<A, S, W>>>,
    visited: &mut HashSet<S>,
    mut neighbor_fn: N,
    mut end_state_fn: E,
    mut acc_fn: AF,
) -> Option<(A, S, W)>
where
    A: Clone,
    S: Eq + PartialEq + Hash + Debug,
    W: Ord + Debug,
    N: FnMut(&S, &W) -> I,
    E: FnMut(&A, &S, &W) -> bool,
    I: IntoIterator<Item = (S, W)>,
    AF: FnMut(&A, &S, &W) -> A,
{
    while let Some(Reverse(state_with_weight)) = work_heap.pop() {
        let (acc, current_state, current_weight) = state_with_weight.into();
        if !visited.contains(&current_state) {
            let acc = acc_fn(&acc, &current_state, &current_weight);

            if end_state_fn(&acc, &current_state, &current_weight) {
                return Some((acc, current_state, current_weight));
            }
            neighbor_fn(&current_state, &current_weight)
                .into_iter()
                .map(|(next_state, next_weight)| StateWithWeight {
                    accumulator: acc.clone(),
                    state: next_state,
                    weight: next_weight,
                })
                .for_each(|item| work_heap.push(Reverse(item)));

            visited.insert(current_state);
        }
    }

    None
}
