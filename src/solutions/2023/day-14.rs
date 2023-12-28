use std::collections::HashMap;
use std::ops::Add;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    Parser,
};

use aoc_rust::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ]
    }
}

impl Add<Direction> for (isize, isize) {
    type Output = (isize, isize);

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => (self.0, self.1 - 1),
            Direction::East => (self.0 + 1, self.1),
            Direction::South => (self.0, self.1 + 1),
            Direction::West => (self.0 - 1, self.1),
        }
    }
}

struct Day14 {
    grid: Vec<Vec<Rock>>,
}

impl Day14 {
    fn move_rocks(&mut self, dir: Direction) -> bool {
        let mut moved = false;
        for a in 0..self.grid.len() {
            for b in 0..self.grid[a].len() {
                let (mut x, mut y) = match dir {
                    Direction::North => (b as isize, a as isize),
                    Direction::East => ((self.grid[a].len() - b - 1) as isize, a as isize),
                    Direction::South => (b as isize, (self.grid.len() - a - 1) as isize),
                    Direction::West => (b as isize, a as isize),
                };

                loop {
                    let (nx, ny) = (x, y) + dir;
                    if nx < 0
                        || ny < 0
                        || nx >= self.grid[0].len() as isize
                        || ny >= self.grid.len() as isize
                    {
                        break;
                    }
                    if self.grid[y as usize][x as usize] == Rock::Rounded
                        && self.grid[ny as usize][nx as usize] == Rock::Empty
                    {
                        self.grid[y as usize][x as usize] = Rock::Empty;
                        self.grid[ny as usize][nx as usize] = Rock::Rounded;
                        moved = true;
                    }
                    x = nx;
                    y = ny;
                }
            }
        }

        moved
    }

    fn hash(&self) -> Vec<u128> {
        let mut hash = Vec::new();
        for row in &self.grid {
            let mut h = 0;
            for rock in row {
                h <<= 1;
                if *rock == Rock::Rounded {
                    h |= 1;
                }
            }
            hash.push(h);
        }
        hash
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for (i, row) in self.grid.iter().enumerate() {
            for rock in row {
                if *rock == Rock::Rounded {
                    score += self.grid.len() - i;
                }
            }
        }
        score
    }
}

impl std::fmt::Debug for Day14 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for rock in row {
                write!(f, "{:?}", rock)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Rock {
    Rounded,
    Cube,
    Empty,
}

impl Rock {
    fn parse(input: &str) -> ParseResult<Rock> {
        alt((
            tag("O").map(|_| Rock::Rounded),
            tag("#").map(|_| Rock::Cube),
            tag(".").map(|_| Rock::Empty),
        ))
        .parse(input)
    }
}

impl std::fmt::Debug for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rock::Rounded => write!(f, "O"),
            Rock::Cube => write!(f, "#"),
            Rock::Empty => write!(f, "."),
        }
    }
}

impl Problem<usize, usize> for Day14 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, many1(Rock::parse))
            .map(|grid| Self { grid })
            .parse(input)
    }

    fn part1(mut self) -> Result<usize> {
        self.move_rocks(Direction::North);
        Ok(self.score())
    }

    fn part2(mut self) -> Result<usize> {
        let mut i: i32 = 0;
        let mut hashes = HashMap::new();
        while i < 1000000000 {
            let mut moved = false;
            for dir in Direction::all().iter() {
                moved |= self.move_rocks(*dir);
            }
            i += 1;

            let hash = self.hash();
            if let Some(j) = hashes.insert(hash, i) {
                let cycle = i - j;
                let remaining = 1000000000 - i;
                let remaining = remaining % cycle;

                for _ in 0..remaining {
                    for dir in Direction::all().iter() {
                        self.move_rocks(*dir);
                        i += 1;
                    }
                }
                break;
            }

            if !moved {
                break;
            }
        }
        Ok(self.score())
    }
}

aoc_main!(Day14);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn test_part1() {
        assert_task!(Day14, 1, EXAMPLE, 136);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day14, 2, EXAMPLE, 64);
    }
}
