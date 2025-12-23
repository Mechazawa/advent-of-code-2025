use std::collections::{BTreeMap, HashMap};
use anyhow::Context;

advent_of_code::solution!(11);

type DeviceName = [char; 3];
struct DeviceMap {
    graph: BTreeMap<DeviceName, Vec<DeviceName>>,
    cache: HashMap<(DeviceName, DeviceName), usize>,
}

type DevicePath = Vec<DeviceName>;

const UNKNOWN_DEVICE: DeviceName = ['?'; 3];

impl DeviceMap {
    fn new(graph: BTreeMap<DeviceName, Vec<DeviceName>>) -> Self {
        Self { graph, cache: HashMap::new() }
    }

    fn count_paths(
        &mut self,
        start: DeviceName,
        end: DeviceName,
    ) -> usize {
        if start == end {
            return 1;
        }
        if let Some(&count) = self.cache.get(&(start, end)) {
            return count;
        }

        let mut total = 0;
        if let Some(neighbors) = self.graph.get(&start).cloned() {
            for neighbor in neighbors {
                total += self.count_paths(neighbor, end);
            }
        }

        self.cache.insert((start, end), total);
        total
    }
}

fn parse_name(input: &str) -> DeviceName {
    let mut output = UNKNOWN_DEVICE.clone();

    for (i, c) in input.chars().take(3).enumerate() {
        output[i] = c;
    }

    output
}

fn parse_input(input: &str) -> DeviceMap {
    DeviceMap::new(input
        .trim()
        .lines()
        .filter_map(|line| line.split_once(":"))
        .map(|(device, connections)| {
            let connections = connections
                .trim()
                .split_whitespace()
                .map(parse_name)
                .collect();

            (parse_name(device), connections)
        })
        .collect())
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = parse_input(input);

    let you = parse_name("you");
    let out = parse_name("out");

    Some(input.count_paths(you, out))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input = parse_input(input);

    let svr = parse_name("svr");
    let fft = parse_name("fft");
    let dac = parse_name("dac");
    let out = parse_name("out");

    let srv_fft = input.count_paths(svr, fft);
    let fft_dac = input.count_paths(fft, dac);
    let dac_out = input.count_paths(dac, out);

    let srv_dac = input.count_paths(svr, dac);
    let dac_fft = input.count_paths(dac, fft);
    let fft_out = input.count_paths(fft, out);

    let direction1 = srv_fft * fft_dac * dac_out;
    let direction2 = srv_dac * dac_fft * fft_out;

    Some(direction1 + direction2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}
