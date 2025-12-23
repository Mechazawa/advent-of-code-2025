use fancy_regex::Regex;
use log::warn;

advent_of_code::solution!(12);

const PRESENT_SIZE: usize = 3;
const PRESENT_COUNT: usize = 6;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Present([bool; PRESENT_SIZE * PRESENT_SIZE]);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct RegionDefinition {
    required: [usize; PRESENT_COUNT],
    width: usize,
    height: usize,
}

fn parse_presents(input: &str) -> [Present; PRESENT_COUNT] {
    let regex: Regex = Regex::new(r"(?m)^(\d):\n([.#\n]+)$").unwrap();
    let mut output = [Present::default(); PRESENT_COUNT];

    for captures in regex.captures_iter(input) {
        let captures = captures.unwrap();

        let id: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let present = output.get_mut(id).expect("id out of bounds");

        captures
            .get(2)
            .unwrap()
            .as_str()
            .chars()
            .filter_map(|c| match c {
                '#' => Some(true),
                '.' => Some(false),
                _ => None,
            })
            .enumerate()
            .for_each(|(i, v)| *present.0.get_mut(i).expect("index out of bounds") = v);
    }

    output
}

fn parse_regions(input: &str) -> Vec<RegionDefinition> {
    let mut regex_str: String = r"(?m)^(\d+)x(\d+):".into();

    for _ in 0..PRESENT_COUNT {
        regex_str.push_str(r"\s+(\d+)");
    }

    let regex: Regex = Regex::new(&regex_str).unwrap();
    let mut output = Vec::new();

    for captures in regex.captures_iter(input) {
        let captures = captures.unwrap();
        let mut region = RegionDefinition::default();

        region.width = captures
            .get(1)
            .map(|c| c.as_str().parse().unwrap_or(1))
            .unwrap_or(1);
        region.height = captures
            .get(2)
            .map(|c| c.as_str().parse().unwrap_or(1))
            .unwrap_or(1);

        for i in 0..PRESENT_COUNT {
            region.required[i] = captures
                .get(3 + i)
                .map(|c| c.as_str().parse().unwrap_or(0))
                .unwrap_or(0);
        }

        output.push(region);
    }

    output
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    let presents = parse_presents(input);
    let regions = parse_regions(input);

    println!("{:?}", presents);
    println!("{:?}", regions);

    None
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
