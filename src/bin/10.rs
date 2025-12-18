use rayon::iter::ParallelIterator;
use std::fmt::{Display, Formatter};
use anyhow::Context;
use fancy_regex::Regex;
use itertools::Itertools;
use std::str::FromStr;
use rayon::prelude::IntoParallelRefIterator;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Copy, Clone)]
struct Indicator(u16);
#[derive(Debug, PartialEq, Copy, Clone)]
struct WiringSchematic(u16);
#[derive(Debug, PartialEq, Clone)]
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
struct MachineButtonSolver {
    state: Indicator,
    available: WiringSchematicCollection,
    used: WiringSchematicCollection,
}

impl Machine {
    fn build_solver(&self) -> MachineButtonSolver {
        MachineButtonSolver {
            state: self.indicator,
            available: self.wiring_schematics.clone(),
            used: WiringSchematicCollection(vec![]),
        }
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

impl Display for MachineButtonSolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.state);

        for available in &self.available.0 {
            write!(f, " ({available})")?;
        }

        Ok(())
    }
}

impl MachineButtonSolver {
    fn shortest(&self) -> Option<WiringSchematicCollection> {
        if self.state.0 == 0 {
            Some(self.used.clone())
        } else if self.solvable() {
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

    fn solvable(&self) -> bool {
        self.state.solvable(&self.available)
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
        .map(Machine::build_solver)
        .map(|s| s.shortest().map_or(0, |v| v.0.len()) as u64)
        .sum())
}

#[must_use]
pub fn part_two(_input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
