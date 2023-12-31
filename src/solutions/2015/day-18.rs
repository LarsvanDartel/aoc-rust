use aoc_rust::*;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    Parser,
};

struct Day18 {
    grid: Vec<Vec<bool>>,
}

impl Day18 {
    fn step(&mut self) {
        let mut new_grid = vec![vec![false; self.grid[0].len()]; self.grid.len()];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, is_on) in row.iter().enumerate() {
                let mut neighbors = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx < 0 || ny < 0 {
                            continue;
                        }
                        let nx = nx as usize;
                        let ny = ny as usize;
                        if nx >= self.grid[y].len() || ny >= self.grid.len() {
                            continue;
                        }
                        if self.grid[ny][nx] {
                            neighbors += 1;
                        }
                    }
                }
                if *is_on {
                    new_grid[y][x] = neighbors == 2 || neighbors == 3;
                } else {
                    new_grid[y][x] = neighbors == 3;
                }
            }
        }
        self.grid = new_grid;
    }
}

impl Problem<usize, usize> for Day18 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, many1(one_of(".#").map(|c| c == '#')))
            .map(|grid| Self { grid })
            .parse(input)
    }

    fn part1(mut self) -> Result<usize> {
        for _ in 0..100 {
            self.step();
        }
        Ok(self.grid.iter().flatten().filter(|&&b| b).count())
    }

    fn part2(mut self) -> Result<usize> {
        let n = self.grid.len() - 1;
        let m = self.grid[0].len() - 1;
        for _ in 0..100 {
            self.step();
            self.grid[0][0] = true;
            self.grid[0][m] = true;
            self.grid[n][0] = true;
            self.grid[n][m] = true;
        }
        Ok(self.grid.iter().flatten().filter(|&&b| b).count())
    }
}

aoc_main!(Day18);
