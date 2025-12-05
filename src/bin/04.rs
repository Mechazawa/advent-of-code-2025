advent_of_code::solution!(4);

struct Grid(Vec<Vec<bool>>);

impl Grid {
    fn new(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| line.chars().map(|c| c == '@').collect())
                .collect(),
        )
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize, bool)> {
        let mut output = Vec::with_capacity(8);

        for ix in (x.saturating_sub(1))..=(x + 1) {
            for iy in (y.saturating_sub(1))..=(y + 1) {
                if let Some(marked) = self.0.get(iy).and_then(|row| row.get(ix)) {
                    output.push((ix, iy, *marked));
                }
            }
        }

        output
    }

    fn set(&mut self, x: usize, y: usize, marked: bool) {
        if x < self.width() && y < self.height() {
            self.0[y][x] = marked;
        }
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
}

impl Iterator for GridIter<'_> {
    type Item = (usize, usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height() {
            return None;
        }

        let result = (self.x, self.y, self.grid.0[self.y][self.x]);

        self.x += 1;
        if self.x >= self.grid.width() {
            self.x = 0;
            self.y += 1;
        }

        Some(result)
    }
}

// Implement IntoIterator for references
impl<'a> IntoIterator for &'a Grid {
    type Item = (usize, usize, bool);
    type IntoIter = GridIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new(input);

    Some(
        grid.into_iter()
            .filter(|(_, _, marked)| *marked)
            .filter(|(x, y, _)| {
                grid.neighbors(*x, *y)
                    .iter()
                    .filter(|(_, _, marked)| *marked)
                    .count()
                    <= 4
            })
            .count() as u64,
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let mut grid = Grid::new(input);

    loop {
        let accessible = grid.into_iter()
            .filter(|(_, _, marked)| *marked)
            .filter(|(x, y, _)| {
                grid.neighbors(*x, *y)
                    .iter()
                    .filter(|(_, _, marked)| *marked)
                    .count()
                    <= 4
            })
            .map(|(x, y, _)| (x, y))
            .collect::<Vec<_>>();

        if accessible.is_empty() {
            break;
        }

        result += accessible.len() as u64;

        for (x, y) in accessible {
            grid.set(x, y, false);
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
