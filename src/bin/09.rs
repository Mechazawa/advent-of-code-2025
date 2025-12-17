use itertools::Itertools;
use geo::{Rect, Polygon, LineString, Contains, coord};
use rayon::prelude::{ParallelBridge, ParallelIterator};

advent_of_code::solution!(9);

type PVec = geo::Coord<u64>;

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

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .iter()
            .tuple_combinations()
            .map(|(a, b)| Rect::new(*a, *b))
            .map(|r| (r.width() + 1) * (r.height() + 1))
            .max()
            .unwrap_or_default(),
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let positions = parse_input(input);
    let polygon: Polygon<f64> = Polygon::new(
        LineString::from(positions.iter().copied().map(|c| coord!{x: c.x as f64, y: c.y as f64}).collect::<Vec<_>>()),
        vec![]
    );

    positions
        .iter()
        .enumerate()
        .tuple_combinations()
        .par_bridge()
        .map(|((ai, av), (bi, bv))| (Rect::new(*av, *bv), (ai, bi)))
        .map(|(r, i)| ((r.width() + 1) * (r.height() + 1), r, i))
        .filter(|(_, r, _)| {
            polygon.contains(&Rect::new(
                coord! {x: r.min().x as f64, y: r.min().y as f64},
                coord! {x: r.max().x as f64, y: r.max().y as f64},
            ))
        })
        .map(|(v, _, _)| v)
        .max()
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
