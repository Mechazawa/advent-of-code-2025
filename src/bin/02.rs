use fancy_regex::Regex;
use itertools::Itertools;

advent_of_code::solution!(2);

fn solve(input: &str, regex: &Regex) -> u64 {
    input
        .split(',')
        .map(|line| {
            let (min, max) = line
                .split('-')
                .map(|n| n.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();

            (min, max)
        })
        .map(|(min, max)| {
            (min..=max)
                .filter(|&v| regex.is_match(&v.to_string()).is_ok_and(|v| v))
                .sum::<u64>()
        })
        .sum()
}

#[must_use] 
pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, &Regex::new("^(\\d+)\\1$").unwrap()))
}

#[must_use] 
pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, &Regex::new("^(\\d+)\\1+$").unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
