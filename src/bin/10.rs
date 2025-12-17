use fancy_regex::Regex;

advent_of_code::solution!(10);

struct Indicator(Vec<bool>);
struct WiringSchematic(Vec<u16>);
struct Joltage(Vec<u16>);

struct Machine {
    indicator: Indicator,
    wiring_schematic: WiringSchematic,
    joltage: Joltage,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .trim()
        .lines()
        .map(|l| Machine {
            // @todo this sucks
            indicator: Indicator(Regex::new(r"(?m)\[([.#]+)\]").unwrap().captures(l).unwrap().unwrap().get(1).unwrap().as_str().chars().map(|c| c == '#').collect()),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
