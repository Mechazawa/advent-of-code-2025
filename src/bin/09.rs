use itertools::Itertools;

advent_of_code::solution!(9);

type PVec = glam::U64Vec2;

fn parse_input(input: &str) -> Vec<PVec> {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            line.split_terminator(',')
                .map(|value| value.parse::<u64>().expect("Invalid value"))
                .collect_tuple::<(u64, u64)>()
        })
        .map(PVec::from)
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .iter()
            .tuple_combinations()
            .map(|(a, b)| a.max(*b) - a.min(*b))
            .map(|v| (1 + v.x) * (1 + v.y))
            .max()
            .unwrap_or_default(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let positions = parse_input(input);

    println!("{positions:?}");

    positions
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((ai, av), (bi, bv))| (av.min(*bv), av.max(*bv), (ai, bi)))
        .map(|(sv, lv, idx)| (lv - sv, sv, lv, idx))
        .map(|(v, sv, lv, idx)| ((1 + v.x) * (1 + v.y), sv, lv, idx))
        .sorted_by_key(|(v, _, _, _)| u64::MAX - *v)
        .find(|(_, sv, lv, (ai, bi))| {
            !positions
                .iter()
                .enumerate()
                .filter(|(i, _)| ai != i && bi != i)
                .any(|(_, p)| {
                    ((p.x > sv.x && p.y >= sv.y) || (p.x >= sv.x && p.y > sv.y))
                        && ((p.x < lv.x && p.y <= lv.y) || (p.x <= lv.x && p.y < lv.y))
                })
        })
        // .map(|(v, _, _, _)| v)
        .map(|(v, a, b, (ai, bi))| {println!("{a:?} {b:?}"); println!("{:?} {:?}", positions[ai], positions[bi]); v})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
