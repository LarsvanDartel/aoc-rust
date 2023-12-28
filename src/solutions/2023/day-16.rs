use std::collections::HashSet;

use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    Parser,
};

use aoc_rust::*;

#[derive(Clone)]
struct Day16 {
    grid: Vec<Vec<Mirror>>,
    ingoing: Vec<Vec<HashSet<Direction>>>,
}

impl Day16 {
    fn new(grid: Vec<Vec<Mirror>>) -> Self {
        let ingoing = vec![vec![HashSet::new(); grid[0].len()]; grid.len()];
        Self { grid, ingoing }
    }

    fn propagate(&mut self, x: isize, y: isize, dir: Direction) {
        let mut queue = vec![(x, y, dir)];

        while let Some((x, y, dir)) = queue.pop() {
            if x < 0
                || y < 0
                || y >= self.grid.len() as isize
                || x >= self.grid[y as usize].len() as isize
            {
                continue;
            }
            if self.ingoing[y as usize][x as usize].contains(&dir) {
                continue;
            }
            self.ingoing[y as usize][x as usize].insert(dir);

            let mirror = &mut self.grid[y as usize][x as usize];

            let next = mirror.next(dir);

            for dir in next {
                match dir {
                    Direction::North => queue.push((x, y - 1, dir)),
                    Direction::East => queue.push((x + 1, y, dir)),
                    Direction::South => queue.push((x, y + 1, dir)),
                    Direction::West => queue.push((x - 1, y, dir)),
                }
            }
        }
    }

    fn cnt(&self) -> usize {
        self.ingoing
            .iter()
            .map(|row| row.iter().filter(|m| !m.is_empty()).count())
            .sum()
    }
}

impl std::fmt::Debug for Day16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for m in row {
                write!(f, "{:?}", m)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone)]
enum Mirror {
    Pipe,
    Dash,
    Slash,
    Backslash,
    Empty,
}

impl Mirror {
    fn parse(input: &str) -> ParseResult<Self> {
        one_of("|-/\\.")
            .map(|c| match c {
                '|' => Mirror::Pipe,
                '-' => Mirror::Dash,
                '/' => Mirror::Slash,
                '\\' => Mirror::Backslash,
                '.' => Mirror::Empty,
                _ => unreachable!(),
            })
            .parse(input)
    }

    fn next(&self, dir: Direction) -> Vec<Direction> {
        match (self, dir) {
            (Mirror::Pipe, Direction::North) => vec![Direction::North],
            (Mirror::Pipe, Direction::East) => vec![Direction::North, Direction::South],
            (Mirror::Pipe, Direction::South) => vec![Direction::South],
            (Mirror::Pipe, Direction::West) => vec![Direction::North, Direction::South],
            (Mirror::Dash, Direction::North) => vec![Direction::East, Direction::West],
            (Mirror::Dash, Direction::East) => vec![Direction::East],
            (Mirror::Dash, Direction::South) => vec![Direction::East, Direction::West],
            (Mirror::Dash, Direction::West) => vec![Direction::West],
            (Mirror::Slash, Direction::North) => vec![Direction::East],
            (Mirror::Slash, Direction::East) => vec![Direction::North],
            (Mirror::Slash, Direction::South) => vec![Direction::West],
            (Mirror::Slash, Direction::West) => vec![Direction::South],
            (Mirror::Backslash, Direction::North) => vec![Direction::West],
            (Mirror::Backslash, Direction::East) => vec![Direction::South],
            (Mirror::Backslash, Direction::South) => vec![Direction::East],
            (Mirror::Backslash, Direction::West) => vec![Direction::North],
            (Mirror::Empty, _) => vec![dir],
        }
    }
}

impl std::fmt::Debug for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mirror::Pipe => write!(f, "|"),
            Mirror::Dash => write!(f, "-"),
            Mirror::Slash => write!(f, "/"),
            Mirror::Backslash => write!(f, "\\"),
            Mirror::Empty => write!(f, "."),
        }
    }
}

impl Problem<usize, usize> for Day16 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, many1(Mirror::parse))
            .map(Self::new)
            .parse(input)
    }

    fn part1(mut self) -> Result<usize> {
        self.propagate(0, 0, Direction::East);
        Ok(self.cnt())
    }

    fn part2(self) -> Result<usize> {
        let mut best = 0;

        for x in 0..self.grid[0].len() {
            let mut input = self.clone();
            input.propagate(x as isize, 0, Direction::South);
            best = best.max(input.cnt());
        }

        for x in 0..self.grid[0].len() {
            let mut input = self.clone();
            input.propagate(x as isize, input.grid.len() as isize - 1, Direction::North);
            best = best.max(input.cnt());
        }

        for y in 0..self.grid.len() {
            let mut input = self.clone();
            input.propagate(0, y as isize, Direction::East);
            best = best.max(input.cnt());
        }

        for y in 0..self.grid.len() {
            let mut input = self.clone();
            input.propagate(
                input.grid[0].len() as isize - 1,
                y as isize,
                Direction::West,
            );
            best = best.max(input.cnt());
        }

        Ok(best)
    }
}

aoc_main!(Day16);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_part1() {
        assert_task!(Day16, 1, EXAMPLE, 46);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day16, 2, EXAMPLE, 51);
    }
}
