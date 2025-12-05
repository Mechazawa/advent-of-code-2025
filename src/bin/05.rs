use itertools::Itertools;

advent_of_code::solution!(5);

#[must_use] 
pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, indexes) = input.split_once("\n\n")?;

    let ranges = ranges
        .lines()
        .filter_map(|line| line.split_once('-'))
        .map(|(a, b)| (
            a.parse::<u64>().unwrap(),
            b.parse::<u64>().unwrap()
        ))
        .collect::<Vec<_>>();

    Some(indexes
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|index| ranges.iter().any(|(min, max)| *index >= *min && *index <= *max))
        .count() as u64
    )

}

#[must_use] 
pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = input.split_once("\n\n")?;

    let ranges = ranges
        .lines()
        .filter_map(|line| line.split_once('-'))
        .map(|(a, b)| (
            a.parse::<u64>().unwrap(),
            b.parse::<u64>().unwrap()
        ))
        .sorted_by_key(|(a, _)| *a)
        .collect::<Vec<_>>();

    let mut merged_ranges = vec![];

    for (min, max) in ranges {
        if let Some((_, last_max)) = merged_ranges.last_mut() {
            if *last_max + 1 >= min {
                *last_max = max.max(*last_max);
            } else {
                merged_ranges.push((min, max));
            }
        } else {
            merged_ranges.push((min, max));
        }
    }

    Some(merged_ranges.iter().map(|(min, max)| max - min + 1).sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
