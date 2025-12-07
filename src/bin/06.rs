use fancy_regex::Regex;
use std::str::FromStr;

advent_of_code::solution!(6);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[derive(Default)]
enum Operation {
    #[default]
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Operation, ()> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err(()),
        }
    }
}


impl Operation {
    fn apply(self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
struct Problem {
    values: Vec<u64>,
    operation: Operation,
}

impl Problem {
    fn evaluate(&self) -> Option<u64> {
        self.values
            .iter()
            .copied()
            .reduce(|a, b| self.operation.apply(a, b))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
struct ProblemBuffer {
    values: Vec<String>,
    operation: Operation,
    width: usize,
}

impl From<&ProblemBuffer> for Problem {
    fn from(value: &ProblemBuffer) -> Self {
        Self {
            values: value.values.iter().map(|s| s.trim().parse().unwrap()).collect(),
            operation: value.operation,
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    let mut problems = Vec::new();

    for line in input.lines() {
        for (idx, group) in line.split_whitespace().enumerate() {
            if problems.len() <= idx {
                problems.push(Problem::default());
            }

            let problem = problems.get_mut(idx).unwrap();

            if let Ok(value) = u64::from_str(group) {
                problem.values.push(value);
            } else if let Ok(operation) = Operation::from_str(group) {
                problem.operation = operation;
            }
        }
    }

    problems.iter().map(Problem::evaluate).sum()
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let mut last_line = input.lines().last().map(ToString::to_string).unwrap_or_default();

    last_line.push(' ');

    let mut buffers = Regex::new(r"(\S)(\s+)")
        .unwrap()
        .captures_iter(&last_line)
        .map(|mat| {
            let mat = mat.unwrap();
            let width = mat.get(2).unwrap().as_str().len();

            ProblemBuffer {
                values: vec![String::default(); width],
                operation: Operation::from_str(mat.get(1).unwrap().as_str()).unwrap_or_default(),
                width,
            }
        }).collect::<Vec<_>>();

    for line in input.lines().take(input.lines().count() - 1) {
        let mut chars = line.chars();

        for buffer in &mut buffers {
            for idx in 0..buffer.width{
                buffer.values[idx].push(chars.next().unwrap());
            }

            chars.next();
        }
    }

    buffers.iter().map(|buffer| Problem::from(buffer).evaluate()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
