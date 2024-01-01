use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{ControlFlow, Not};
use std::rc::Rc;

use anyhow::{anyhow, bail, ensure, Result};
use bitvec::bitvec;
use bitvec::vec::BitVec;
use derive_more::{Deref, DerefMut, From, FromStr, Into};
use derive_new::new;
use indexmap::IndexMap;
use num::Integer;

use crate::solver::{share_struct_solver, ProblemSolver};
use crate::utils::WarningResult;

share_struct_solver!(Day20, Day20Part1, Day20Part2);

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Signal {
    Low,
    Hi,
}

impl From<Signal> for bool {
    fn from(val: Signal) -> Self {
        match val {
            Signal::Low => false,
            Signal::Hi => true,
        }
    }
}

impl From<bool> for Signal {
    fn from(value: bool) -> Self {
        match value {
            true => Signal::Hi,
            false => Signal::Low,
        }
    }
}

impl Not for Signal {
    type Output = Signal;

    fn not(self) -> Self::Output {
        match self {
            Signal::Low => Signal::Hi,
            Signal::Hi => Signal::Low,
        }
    }
}

impl Not for &Signal {
    type Output = Signal;

    fn not(self) -> Self::Output {
        match self {
            Signal::Low => Signal::Hi,
            Signal::Hi => Signal::Low,
        }
    }
}

type ModuleId = usize;
type ConnectionId = usize;

#[derive(new, From, Into, Eq, PartialEq, Copy, Clone, Hash, Debug)]
struct OutputConnection {
    module_id: ModuleId,
    connection_idx: ConnectionId,
}

#[derive(Default, Deref, DerefMut, Debug)]
struct InputConnectionList(Vec<ModuleId>);

impl InputConnectionList {
    fn add_input(&mut self, source_module_id: ModuleId) -> ConnectionId {
        let connection_id = self.len();
        self.push(source_module_id);
        connection_id
    }
}

#[derive(Default, Deref, DerefMut, Debug)]
struct OutputConnectionList(Vec<OutputConnection>);

impl OutputConnectionList {
    fn connect_to(&mut self, target_module_id: ModuleId, connection_id: ConnectionId) {
        self.push(OutputConnection::new(target_module_id, connection_id));
    }
}

#[derive(new, Debug)]
pub struct ModuleInfo {
    #[new(default)]
    parents: InputConnectionList,
    #[new(default)]
    children: OutputConnectionList,
    module_type: ModuleType,
}

impl ModuleInfo {
    fn create_state(&self) -> Result<ModuleState> {
        Ok(ModuleState::new(ModuleReg::create(&self.module_type, self.parents.len())?, Signal::Low))
    }

    fn add_input(&mut self, source_module_id: ModuleId) -> ConnectionId {
        self.parents.add_input(source_module_id)
    }

    fn connect_to(&mut self, target_module_id: ModuleId, connection_id: ConnectionId) {
        self.children.connect_to(target_module_id, connection_id);
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
    Output,
}

#[derive(Debug)]
enum ModuleReg {
    Broadcaster(Signal),
    FlipFlop,
    Conjunction(BitVec),
    Output(Signal),
}

#[derive(new, Debug)]
struct ModuleState {
    reg: ModuleReg,
    current_output: Signal,
}

impl ModuleState {
    fn receive(&mut self, signal: Signal, connection_id: ConnectionId) -> bool {
        self.reg.receive(signal, connection_id)
    }

    fn save_output(&mut self) -> Signal {
        self.current_output = self.reg.get_output(self.current_output);
        self.current_output
    }
}

impl ModuleReg {
    fn create(module_type: &ModuleType, in_connection_count: usize) -> Result<ModuleReg> {
        match module_type {
            ModuleType::Broadcaster => {
                ensure!(in_connection_count == 0, "Broadcaster cannot have input");
                Ok(ModuleReg::Broadcaster(Signal::Low))
            }
            ModuleType::FlipFlop => {
                ensure!(in_connection_count > 0, "Cannot create flipflop without input");
                Ok(ModuleReg::FlipFlop)
            }
            ModuleType::Conjunction => {
                ensure!(in_connection_count > 0, "Cannot create conjunction without input");
                Ok(ModuleReg::Conjunction(bitvec!(0; in_connection_count)))
            }
            ModuleType::Output => {
                ensure!(in_connection_count == 1, "Output can only have 1 input");
                Ok(ModuleReg::Output(Signal::Low))
            }
        }
    }

    fn receive(&mut self, signal: Signal, connection_id: ConnectionId) -> bool {
        match self {
            ModuleReg::Broadcaster(output) => {
                *output = signal;
                true
            }
            ModuleReg::FlipFlop => signal == Signal::Low,
            ModuleReg::Conjunction(vec) => {
                vec.set(connection_id, signal.into());
                true
            }
            ModuleReg::Output(output) => {
                *output = signal;
                true
            }
        }
    }

    fn get_output(&self, current_output: Signal) -> Signal {
        match self {
            ModuleReg::Broadcaster(output) => *output,
            ModuleReg::FlipFlop => !current_output,
            ModuleReg::Conjunction(vec) => vec.not_all().into(),
            ModuleReg::Output(output) => *output,
        }
    }
}

#[derive(new, Deref, Debug)]
pub struct Day20Part1 {
    module_infos: IndexMap<String, ModuleInfo>,
}

#[derive(Deref)]
pub struct Day20Part2(Rc<Day20Part1>);

impl FromStr for Day20Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut module_map = IndexMap::default();
        s.lines().try_for_each(|line| {
            let (module_name_full, target_modules) =
                line.split_once(" -> ").ok_or_else(|| anyhow!("Invalid input string: {line:?}"))?;

            let first_char = module_name_full.chars().next().unwrap();
            let module_type = match first_char {
                'a'..='z' => ModuleType::Broadcaster,
                '&' => ModuleType::Conjunction,
                '%' => ModuleType::FlipFlop,
                _ => bail!("Invalid module: {module_name_full:?}"),
            };

            let module_name = if module_type == ModuleType::Broadcaster {
                module_name_full.into()
            } else {
                module_name_full[1..].into()
            };

            let entry = module_map
                .entry(module_name)
                .and_modify(|v: &mut ModuleInfo| v.module_type = module_type);
            let source_module_id = entry.index();
            entry.or_insert_with(|| ModuleInfo::new(module_type));

            target_modules.split(',').map(str::trim).map(|b| b.into()).for_each(|name| {
                let entry = module_map.entry(name);
                let target_module_id = entry.index();
                let connection_id = entry
                    .or_insert_with(|| ModuleInfo::new(ModuleType::Output))
                    .add_input(source_module_id);
                module_map[source_module_id].connect_to(target_module_id, connection_id);
            });

            Ok(())
        })?;

        Ok(Day20Part1::new(module_map))
    }
}

impl ProblemSolver for Day20Part1 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        let broadcaster_id = self.get_index_of("broadcaster").unwrap();
        let mut states =
            self.values().map(|v| v.create_state()).collect::<Result<Vec<ModuleState>>>()?;
        let mut lo = 0_usize;
        let mut hi = 0_usize;
        for _ in 0..1000 {
            let mut input = vec![(broadcaster_id, Signal::Low, 0_usize)];

            while !input.is_empty() {
                (lo, hi) = input.iter().fold((lo, hi), |(lo, hi), (_, signal, _)| match signal {
                    Signal::Low => (lo + 1, hi),
                    Signal::Hi => (lo, hi + 1),
                });
                input = self.step(&mut states, input);
            }
        }
        Ok(lo * hi)
    }
}

impl Day20Part1 {
    fn step<T: IntoIterator<Item = (ModuleId, Signal, ConnectionId)>>(
        &self,
        states: &mut [ModuleState],
        input: T,
    ) -> Vec<(ModuleId, Signal, ConnectionId)> {
        input
            .into_iter()
            .filter_map(|(target_id, signal, connection_id)| {
                let have_output = states[target_id].receive(signal, connection_id);
                if have_output { Some(target_id) } else { None }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .flat_map(|source_id| {
                let signal = states[source_id].save_output();
                self.module_infos[source_id].children.iter().map(move |output_connection| {
                    (output_connection.module_id, signal, output_connection.connection_idx)
                })
            })
            .collect()
    }
}

impl ProblemSolver for Day20Part2 {
    type SolutionType = WarningResult<usize>;

    fn solve(&self) -> Result<Self::SolutionType> {
        let broadcaster_id = self.get_index_of("broadcaster").unwrap();
        let mut states =
            self.values().map(|v| v.create_state()).collect::<Result<Vec<ModuleState>>>()?;
        let rx_parent_id = self.get("rx").unwrap().parents[0];
        let (_, rx_parent_module) = self.get_index(rx_parent_id).unwrap();
        ensure!(
            rx_parent_module.module_type == ModuleType::Conjunction,
            "Unable to solve, expect parent of rx is a conjunction module."
        );
        let rx_grandparent_ids = &rx_parent_module.parents;
        ensure!(
            rx_grandparent_ids.iter().all(|module_id| self
                .get_index(*module_id)
                .unwrap()
                .1
                .module_type
                == ModuleType::Conjunction),
            "Unable to solve, expect grandparents of rx are all conjunction modules."
        );
        let mut rx_grandparent_id_and_cycle_len =
            rx_grandparent_ids.iter().map(|i| (*i, None)).collect::<HashMap<_, _>>();
        let mut num_grandparents = rx_grandparent_ids.len();

        let run_result = (1_usize..10000_usize).try_for_each(|cycle_len| {
            self.cycle_and_apply_function_to_output(
                &mut states,
                broadcaster_id,
                &mut |id, signal| {
                    if let Some(cycle_len_option) = rx_grandparent_id_and_cycle_len.get_mut(&id) {
                        if signal == Signal::Hi && cycle_len_option.is_none() {
                            cycle_len_option.replace(cycle_len);
                            num_grandparents -= 1;
                        }
                    }
                },
            );

            if num_grandparents == 0 { ControlFlow::Break(()) } else { ControlFlow::Continue(()) }
        });

        if run_result.is_continue() {
            bail!("Cannot find all cycles within 10000 button press.");
        }

        Ok(WarningResult::new(
            rx_grandparent_id_and_cycle_len
                .into_values()
                .map(Option::unwrap)
                .reduce(|l, r| l.lcm(&r))
                .unwrap(),
            "Assuming parent and grandparents of rx are conjunction, grandparents output high in a cycle and result is lcm of all grandparents cycle",
        ))
    }
}

impl Day20Part2 {
    fn cycle_and_apply_function_to_output<F>(
        &self,
        states: &mut [ModuleState],
        broadcaster_id: ModuleId,
        module_output_fn: &mut F,
    ) where
        F: FnMut(ModuleId, Signal),
    {
        let mut input = vec![(broadcaster_id, Signal::Low, 0_usize)];

        while !input.is_empty() {
            input = self.step_and_apply_function_to_output(states, input, module_output_fn);
        }
    }

    fn step_and_apply_function_to_output<T, F>(
        &self,
        states: &mut [ModuleState],
        input: T,
        module_output_fn: &mut F,
    ) -> Vec<(ModuleId, Signal, ConnectionId)>
    where
        T: IntoIterator<Item = (ModuleId, Signal, ConnectionId)>,
        F: FnMut(ModuleId, Signal),
    {
        let next_input = input
            .into_iter()
            .filter_map(|(target_id, signal, connection_id)| {
                let have_output = states[target_id].reg.receive(signal, connection_id);
                if have_output { Some(target_id) } else { None }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .flat_map(|source_id| {
                let signal = states[source_id].save_output();
                module_output_fn(source_id, signal);
                self.module_infos[source_id].children.iter().map(move |output_connection| {
                    (output_connection.module_id, signal, output_connection.connection_idx)
                })
            })
            .collect();

        next_input
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::str::FromStr;

    use anyhow::Result;
    use indoc::indoc;

    use crate::solver::y2023::day20::Day20;
    use crate::solver::TwoPartsProblemSolver;

    const SAMPLE_INPUT_1: &str = indoc! {r"
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
    "};

    const SAMPLE_INPUT_2: &str = indoc! {r"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
    "};

    const SAMPLE_INPUT_3: &str = indoc! {r"
            broadcaster -> a
            %a -> b
            %b -> inv
            &inv -> con
            &con -> rx
    "};

    #[test]
    fn test_solve_1() -> Result<()> {
        assert_eq!(Day20::from_str(SAMPLE_INPUT_1)?.solve_1()?, 32000000);
        assert_eq!(Day20::from_str(SAMPLE_INPUT_2)?.solve_1()?, 11687500);
        Ok(())
    }

    #[test]
    fn test_solve_2() -> Result<()> {
        assert_eq!(*Day20::from_str(SAMPLE_INPUT_3)?.solve_2()?.deref(), 4);
        Ok(())
    }
}
