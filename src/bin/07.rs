advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Beam,
    Splitter,
}

struct Row (Vec<Tile>);

impl Row {
    fn propagate(&self, next: &Row) -> (Row, u64) {
        let mut result = vec![Tile::Empty; self.0.len()];
        let mut splits = 0;

        for idx in 1..(self.0.len() - 1) {
            match (self.0[idx], next.0[idx]) {
                (Tile::Beam, Tile::Empty) => result[idx] = Tile::Beam,
                (Tile::Beam, Tile::Splitter) => {
                    result[idx - 1] = Tile::Beam;
                    result[idx] = Tile::Splitter;
                    result[idx + 1] = Tile::Beam;

                    splits += 1;
                },
                _ => {}
            }
        }

        (Row(result), splits)
    }
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        Row(s.chars().map(|c| match c {
            '.' => Tile::Empty,
            '|' => Tile::Beam,
            'S' => Tile::Beam,
            '^' => Tile::Splitter,
            _ => unreachable!(),
        }).collect())
    }
}

fn parse_input(input: &str) -> Vec<Row> {
    input.trim().lines().map(Row::from).collect()
}

#[must_use] 
pub fn part_one(input: &str) -> Option<u64> {
    let mut rows = parse_input(input);
    let mut acc = (rows.remove(0), 0);

    for row in &rows {
        let (row, splits) = acc.0.propagate(row);

        acc = (row, acc.1 + splits);
    }

    Some(acc.1)
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
