use std::collections::BTreeMap;
use anyhow::Context;

advent_of_code::solution!(11);

type DeviceName = [char; 3];
struct DeviceMap(BTreeMap<DeviceName, Vec<DeviceName>>);
type DevicePath = Vec<DeviceName>;

const UNKNOWN_DEVICE: DeviceName = ['?'; 3];

impl DeviceMap {
    fn paths(&self, start: &DeviceName, end: &DeviceName, filter: fn(DevicePath) -> bool) -> Vec<DevicePath> {
        let mut output = vec![];
        let mut stacks = self.0.get(start)
            .map(|v| v
                .iter()
                .copied()
                .map(|e| vec![e])
                .collect::<Vec<DevicePath>>())
            .unwrap_or_default();

       while let Some(stack) = stacks.pop() {
            let last = stack.last().unwrap_or(&UNKNOWN_DEVICE);

           if last == end {
               if filter(stack.clone()) {
                   output.push(stack);
               }
           } else {
               for target_item in self.0.get(last).unwrap_or(&vec![]).iter() {
                   if !stack.contains(target_item) {
                       let mut new_stack = stack.clone();

                       new_stack.push(*target_item);

                       stacks.push(new_stack);
                   }
               }
           }
       }

        output
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
    DeviceMap(input
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

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    Some(input.paths(&parse_name("you"), &parse_name("out"), |_| true).len() as u64)
}

const NAME_FFT: DeviceName = ['f', 'f' , 't'];
const NAME_DAC: DeviceName = ['d', 'a', 'c'];

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    println!("Searching for paths from svr to out");
    let paths = input.paths(
        &parse_name("svr"),
        &parse_name("out"),
        |path| path.contains(&NAME_FFT) && path.contains(&NAME_DAC)
    );

    Some(paths.len() as u64)
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
