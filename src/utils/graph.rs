use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use std::rc::Rc;

pub fn dfs<'a, S, N, E, I, A, AF>(
    start: S,
    neighbor_fn: N,
    end_state_fn: E,
    acc_init: A,
    acc_fn: AF,
) -> Option<(A, S)>
where
    A: Clone,
    S: Eq + PartialEq + Hash + Debug,
    N: Fn(&S) -> I,
    E: Fn(&A, &S) -> bool,
    I: IntoIterator<Item = S>,
    AF: FnMut(&A, &S) -> A,
{
    dfs_full(&mut vec![(acc_init, start)], &mut HashSet::new(), neighbor_fn, end_state_fn, acc_fn)
}

pub fn dfs_full<S, N, E, I, A, AF>(
    work_stack: &mut Vec<(A, S)>,
    visited: &mut HashSet<S>,
    neighbor_fn: N,
    end_state_fn: E,
    mut acc_fn: AF,
) -> Option<(A, S)>
where
    A: Clone,
    S: Eq + PartialEq + Hash + Debug,
    N: Fn(&S) -> I,
    E: Fn(&A, &S) -> bool,
    I: IntoIterator<Item = S>,
    AF: FnMut(&A, &S) -> A,
{
    while let Some((acc, current_state)) = work_stack.pop() {
        if !visited.contains(&current_state) {
            let acc = acc_fn(&acc, &current_state);

            if end_state_fn(&acc, &current_state) {
                return Some((acc, current_state));
            }
            neighbor_fn(&current_state)
                .into_iter()
                .map(|next_state| (acc.clone(), next_state))
                .for_each(|item| work_stack.push(item));

            visited.insert(current_state);
        }
    }

    None
}
