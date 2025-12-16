use std::collections::HashMap;
use itertools::Itertools;
advent_of_code::solution!(8);

type PVec = glam::Vec3;

fn parse_input(input: &str) -> Vec<PVec> {
    input
        .lines()
        .filter_map(|line| {
            line.split_terminator(',')
                .map(|value| value.parse::<f32>().expect("Invalid value"))
                .collect_tuple::<(f32, f32, f32)>()
        })
        .map(PVec::from)
        .collect::<Vec<_>>()
}

fn find_parent(parents: &mut Vec<usize>, i: usize) -> Option<usize> {
    if parents[i] != i {
        parents[i] = find_parent(parents, parents[i])?;
    }

    Some(parents[i])
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    let coordinates = parse_input(input);
    let mut parents = (0..coordinates.len()).collect::<Vec<_>>();
    let sorted = coordinates
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((ai, av), (bi, bv))| (av.distance_squared(*bv).to_bits(), ai, bi))
        .sorted_unstable_by_key(|(d, _, _)| *d)
        .map(|(_, a, b)| (a, b));

    let max_attempts = if input.lines().count() > 100 { 1000 } else { 10 };
    let mut attempts = 0;

    for (a, b) in sorted {
        attempts += 1;

        if attempts > max_attempts {
            break;
        }

        let parent_a = find_parent(&mut parents, a)?;
        let parent_b = find_parent(&mut parents, b)?;

        if parent_a != parent_b {
            parents[parent_a] = parent_b;
        }
    }

    Some((0..coordinates.len())
        .map(|i| find_parent(&mut parents, i))
        .counts()
        .iter()
        .sorted_by_key(|(_, count)| usize::MAX - *count)
        .take(3)
        .map(|(_, count)| count)
        .product::<usize>() as u64)
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let coordinates = parse_input(input);
    let mut parents = (0..coordinates.len()).collect::<Vec<_>>();
    let sorted = coordinates
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((ai, av), (bi, bv))| (av.distance_squared(*bv).to_bits(), ai, bi))
        .sorted_unstable_by_key(|(d, _, _)| *d)
        .map(|(_, a, b)| (a, b));

    let mut last = (0,0);

    for (a, b) in sorted {
        let parent_a = find_parent(&mut parents, a)?;
        let parent_b = find_parent(&mut parents, b)?;

        if parent_a != parent_b {
            last = (a, b);
            parents[parent_a] = parent_b;
        }
    }

    Some((coordinates[last.0].x as u64) * (coordinates[last.1].x as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
