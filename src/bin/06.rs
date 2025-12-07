use std::str::FromStr;

advent_of_code::solution!(6);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Operation {
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

impl Default for Operation {
    fn default() -> Self {
        Self::Add
    }
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
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
        self.values.iter().copied().reduce(|a, b| self.operation.apply(a, b))
    }
}

fn parse(input: &str) -> Vec<Problem> {
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

    problems
}

pub fn part_one(input: &str) -> Option<u64> {
    parse(input).iter().map(|problem| problem.evaluate()).sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
