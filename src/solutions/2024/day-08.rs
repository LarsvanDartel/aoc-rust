use aoc_rust::*;
use common::*;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

enum Cell {
    Empty,
    Antenna(char),
}

impl Cell {
    fn parse(input: &str) -> ParseResult<Self> {
        alt((
            char('.').map(|_| Cell::Empty),
            verify(anychar, char::is_ascii_alphanumeric).map(Cell::Antenna),
        ))
        .parse(input)
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Antenna(c) => write!(f, "{}", c),
        }
    }
}

struct Day08 {
    grid: Grid<Cell>,
}

impl Problem<usize, usize> for Day08 {
    fn parse(input: &str) -> ParseResult<Self> {
        Grid::parse(Cell::parse)
            .map(|grid| Self { grid })
            .parse(input)
    }

    fn part1(self) -> Result<usize> {
        let antennas = self
            .grid
            .coordinates()
            .filter_map(|pos| {
                self.grid.get(pos).and_then(|cell| match cell {
                    Cell::Antenna(c) => Some((*c, pos)),
                    _ => None,
                })
            })
            .fold(HashMap::new(), |mut antennas, (c, pos)| {
                antennas.entry(c).or_insert_with(Vec::new).push(pos);
                antennas
            });

        let mut antinodes = HashSet::new();

        for (_, positions) in antennas {
            for (a, b) in positions.into_iter().tuple_combinations() {
                let d = b - a;
                antinodes.insert(a - d);
                antinodes.insert(b + d);
            }
        }

        Ok(antinodes
            .into_iter()
            .filter(|&pos| self.grid.contains(pos))
            .count())
    }

    fn part2(self) -> Result<usize> {
        let antennas = self
            .grid
            .coordinates()
            .filter_map(|pos| {
                self.grid.get(pos).and_then(|cell| match cell {
                    Cell::Antenna(c) => Some((*c, pos)),
                    _ => None,
                })
            })
            .fold(HashMap::new(), |mut antennas, (c, pos)| {
                antennas.entry(c).or_insert_with(Vec::new).push(pos);
                antennas
            });

        let mut antinodes = HashSet::new();

        for (_, positions) in antennas {
            for (mut a, mut b) in positions.into_iter().tuple_combinations() {
                let d = b - a;
                while self.grid.contains(a) {
                    antinodes.insert(a);
                    a -= d;
                }
                while self.grid.contains(b) {
                    antinodes.insert(b);
                    b += d;
                }
            }
        }

        Ok(antinodes.len())
    }
}

aoc_main!(Day08);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day08, 1, EXAMPLE, 14);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day08, 2, EXAMPLE, 34);
    }
}
