use itertools::Itertools;
advent_of_code::solution!(8);

type PVec = glam::Vec3;

fn parse_input(input: &str) -> Vec<PVec> {
    input
        .lines()
        .filter_map(|line| line
            .split_terminator(',')
            .map(|value| value.parse::<f32>().expect("Invalid value"))
            .collect_tuple::<(f32, f32, f32)>()
        )
        .map(PVec::from)
        .collect::<Vec<_>>()
}

#[must_use] 
pub fn part_one(input: &str) -> Option<u64> {
    let coordinates = parse_input(input);
    let mut parents = (0..coordinates.len()).collect::<Vec<_>>();
    let sorted = coordinates
        .iter()
        .enumerate()
        .combinations(2)
        .map(|v| (v[0], v[1]))
        .map(|((ai, av),(bi ,bv))| (av.distance_squared(*bv), ai, bi))
        .sorted_unstable_by(|(d1,_,_), (d2,_,_)| d1.partial_cmp(d2).unwrap())
        .map(|(_,a,b)| (a,b));

    fn find_parent(parents: &mut Vec<usize>, i: usize) -> Option<usize> {
        if parents[i] != i {
            parents[i] = find_parent(parents, parents[i])?;
        }

        Some(parents[i])
    }

    let mut grouped = 0;

    for (a, b) in sorted {
        if grouped >= 1000 {
            break;
        }

        let parent_a = find_parent(&mut parents, a);
        let parent_b = find_parent(&mut parents, b);

        if parent_a == parent_b || parent_a.is_none() || parent_b.is_none() {
            continue;
        }

        grouped += 1;
        parents[parent_a.unwrap()] = parent_b.unwrap();
    }

    Some(parents
        .iter().copied()
        .counts()
        .iter()
        .sorted_by_key(|(_, count)| usize::MAX - *count)
        .take(3)
        .map(|(_, count)| count)
        .product::<usize>() as u64)
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
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
