use std::cmp::{Ordering, Reverse};
use anyhow::Context;
use fancy_regex::Regex;
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Copy, Clone)]
struct Indicator(u16);
#[derive(Debug, PartialEq, Copy, Clone)]
struct WiringSchematic(u16);
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct Joltage(Vec<u8>);
#[derive(Debug, PartialEq, Clone)]
struct WiringSchematicCollection(Vec<WiringSchematic>);

#[derive(Debug, Clone)]
struct Machine {
    indicator: Indicator,
    wiring_schematics: WiringSchematicCollection,
    joltage: Joltage,
}

impl Indicator {
    fn solvable(&self, wiring: &WiringSchematicCollection) -> bool {
        self.0 & wiring.sum() == self.0
    }

    fn wire(&self, wiring: &WiringSchematic) -> Self {
        Self(self.0 ^ wiring.0)
    }
}

impl FromStr for Indicator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Indicator(
            Regex::new(r"\[([.#]+)\]")
                .context("Failed to compile regex")?
                .captures(s)?
                .context("Failed to capture group")?
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .enumerate()
                .fold(
                    0u16,
                    |acc, (i, c)| {
                        if c == '#' { acc | (1 << i) } else { acc }
                    },
                ),
        ))
    }
}

impl FromStr for WiringSchematic {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(WiringSchematic(
            s.split(',')
                .filter_map(|n| n.parse::<u8>().ok())
                .fold(0u16, |acc, v| acc | (1 << v)),
        ))
    }
}

impl FromStr for Joltage {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Joltage(
            Regex::new(r"\{([0-9,]+)\}")
                .context("Failed to compile regex")?
                .captures(s)?
                .context("Failed to capture group")?
                .get(1)
                .unwrap()
                .as_str()
                .split(',')
                .filter_map(|n| n.parse::<u8>().ok())
                .collect(),
        ))
    }
}

impl WiringSchematic {
    fn indexes(&self) -> Vec<usize> {
        (0..16)
            .filter(|i| self.0 & (1u16 << i) != 0)
            .collect()
    }
}

impl WiringSchematicCollection {
    fn sum(&self) -> u16 {
        self.0.iter().fold(0u16, |acc, v| acc | v.0)
    }
}

impl FromStr for WiringSchematicCollection {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(WiringSchematicCollection(
            Regex::new(r"\(([0-9,]+)\)")
                .context("Failed to compile regex")?
                .captures_iter(s)
                .filter_map(Result::ok)
                .filter_map(|c| c.get(1))
                .map(|c| {
                    c.as_str()
                        .parse::<WiringSchematic>()
                        .context("Failed to parse wiring schematic")
                })
                .collect::<Result<Vec<WiringSchematic>, _>>()?,
        ))
    }
}

impl FromStr for Machine {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Machine {
            indicator: input.parse()?,
            wiring_schematics: input.parse()?,
            joltage: input.parse()?,
        })
    }
}

#[derive(Debug)]
struct MachineWiringSolver {
    state: Indicator,
    available: WiringSchematicCollection,
    used: WiringSchematicCollection,
}

impl Joltage {
    fn wire(&self, wire: &WiringSchematic) -> Result<Self, anyhow::Error> {
        let mut output = self.clone();

        for index in wire.indexes() {
            let mut value = output.0.get_mut(index).context("Joltage list overflow")?;

           *value = value.checked_sub(1).context("Joltage underflow")?;
        }

        Ok(output)
    }
}

#[derive(Debug, Eq, Clone)]
struct JoltageSolveStep(usize, Joltage, u8);

impl PartialEq<Self> for JoltageSolveStep {
    fn eq(&self, other: &Self) -> bool {
         self.0 == other.0 && self.1 == other.1
    }
}

impl PartialOrd<Self> for JoltageSolveStep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.2.partial_cmp(&other.2).map(|v|v.reverse())
    }
}

impl Ord for JoltageSolveStep {
    fn cmp(&self, other: &Self) -> Ordering {
        self.2.cmp(&other.2).reverse()
    }
}

impl JoltageSolveStep {
    fn new(steps: usize, joltage: Joltage) -> Self {
        let remaining = joltage.0.iter().sum();

        Self(steps, joltage, remaining)
    }
}

impl Machine {
    fn build_wiring_solver(&self) -> MachineWiringSolver {
        MachineWiringSolver {
            state: self.indicator,
            available: self.wiring_schematics.clone(),
            used: WiringSchematicCollection(vec![]),
        }
    }

    fn joltage_solver(&self) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        let mut cache: HashMap<Joltage, JoltageSolveStep> = HashMap::new();
        let mut min_steps: Option<usize> = None;

        heap.push(JoltageSolveStep::new(0, self.joltage.clone()));

        println!("{:?}", self.joltage);

        while let Some(JoltageSolveStep(steps, joltage, remaining)) = heap.pop() {
            if remaining == 0 {
                min_steps = Some(min_steps.map_or(steps, |min| min.min(steps)));

                println!("{:5} -> {:3} -> {:?}", heap.len(), steps, joltage);
                continue;
            }

            let next_steps = steps + 1;
            // println!("{:5} -> {:3} -> {:?}", heap.len(), steps, joltage);

            if let Some(min) = min_steps {
                if next_steps > min {
                    continue;
                }
            }

            for wiring in self.wiring_schematics.0.iter() {
                let next_joltage = joltage.wire(wiring);

                if next_joltage.is_err() {
                    continue;
                }

                let next_joltage = next_joltage.unwrap();

                if let Some(JoltageSolveStep(depth, _, _)) = cache.get(&next_joltage) {
                    if *depth <= next_steps {
                        continue;
                    }
                }

                let next = JoltageSolveStep::new(next_steps, next_joltage.clone());

                cache.insert(next_joltage, next.clone());
                heap.push(next);
            }
        }

        min_steps
    }
}

impl Display for Indicator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..16 {
            write!(f, "{}", if self.0 & (1 << i) != 0 { "#" } else { "." })?;
        }

        Ok(())
    }
}

impl Display for WiringSchematic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..16 {
            write!(f, "{}", if self.0 & (1 << i) != 0 { "#" } else { "." })?;
        }

        Ok(())
    }
}

impl Display for MachineWiringSolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.state)?;

        for available in &self.available.0 {
            write!(f, " ({available})")?;
        }

        Ok(())
    }
}

impl MachineWiringSolver {
    fn shortest(&self) -> Option<WiringSchematicCollection> {
        if self.state.0 == 0 {
            Some(self.used.clone())
        } else if self.state.solvable(&self.available) {
            (0..self.available.0.len())
                .filter_map(|i| {
                    let mut available = WiringSchematicCollection(Vec::from(&self.available.0[i..]));
                    let wiring = available.0.remove(0);

                    let state = self.state.wire(&wiring);
                    let mut used = self.used.clone();

                    used.0.push(wiring);

                    (Self {state, available, used}).shortest()
                })
                .min_by_key(|s| s.0.len())
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .trim()
        .lines()
        .map(|l| l.parse::<Machine>().unwrap())
        .collect()
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Some(parse_input(input)
        .par_iter()
        .map(Machine::build_wiring_solver)
        .map(|s| s.shortest().map_or(0, |v| v.0.len()) as u64)
        .sum())
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    Some(parse_input(input)
        .iter()
        .filter_map(Machine::joltage_solver)
        .sum::<usize>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
