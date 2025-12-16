advent_of_code::solution!(1);

fn parse_input(input: &str) -> Vec<(i64, i64, i64)> {
    let mut dail = 50i64;

    input
        .lines()
        .map(|line| {
            line.replace('L', "-")
                .replace('R', "")
                .parse::<i64>()
                .unwrap()
        })
        .map(|steps| {
            let from = dail;
            dail += steps;

            (steps, from, dail)
        })
        .collect()
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .iter()
            .filter(|(_, _, to)| to % 100 == 0)
            .count() as u64,
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .iter()
            .map(|(steps, from, to)| {
                if *steps >= 0 {
                    to.div_euclid(100) - from.div_euclid(100)
                } else {
                    (from - 1).div_euclid(100) - (to - 1).div_euclid(100)
                }
            })
            .sum::<i64>().cast_unsigned(),
    )
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
        assert_eq!(result, Some(6));
    }
}
