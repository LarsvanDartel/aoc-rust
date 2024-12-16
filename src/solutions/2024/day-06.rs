use aoc_rust::*;
use common::*;
use hashbrown::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Empty,
    Wall,
    Start,
    Passed,
}

impl Cell {
    fn parse(input: &mut &str) -> PResult<Self> {
        one_of(['.', '#', '^'])
            .map(|c| match c {
                '.' => Self::Empty,
                '#' => Self::Wall,
                '^' => Self::Start,
                _ => unreachable!(),
            })
            .parse_next(input)
    }
}

struct Day06 {
    start: Vec2<isize>,
    grid: Grid<Cell>,
    jump_table: HashMap<(Vec2<isize>, Direction), Vec2<isize>>,
}

impl Day06 {
    fn loops(&self, mut pos: Vec2<isize>, mut dir: Direction, stone: Vec2<isize>) -> bool {
        let mut visited = HashSet::with_capacity(500);
        while self.grid.get(pos).is_some() {
            if visited.contains(&(pos, dir)) {
                return true;
            }
            visited.insert((pos, dir));
            if pos.x == stone.x || pos.y == stone.y {
                while let Some(&Cell::Wall) = self.grid.get(pos + dir) {
                    dir = dir.right();
                }
                pos += dir;
            } else {
                if let Some(&jump) = self.jump_table.get(&(pos, dir)) {
                    pos = jump;
                } else {
                    return false;
                }
                dir = dir.right();
            };
        }

        false
    }
}

impl Problem<usize, usize> for Day06 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Grid::parse(Cell::parse)
            .map(|grid| {
                let mut jump_table = HashMap::new();
                let mut start = Default::default();
                for pos in grid.coordinates() {
                    if grid.get(pos) == Some(&Cell::Start) {
                        start = pos;
                    }
                    for dir in Direction::cardinal() {
                        let mut jump = pos;
                        while let Some(c) = grid.get(jump + dir) {
                            if c == &Cell::Wall {
                                jump_table.insert((pos, dir), jump);
                                break;
                            }
                            jump += dir;
                        }
                    }
                }

                Self {
                    start,
                    grid,
                    jump_table,
                }
            })
            .parse_next(input)
    }

    fn part1(mut self) -> Result<usize> {
        let mut cnt = 0;
        let mut pos = self.start;
        let mut dir = Direction::North;
        self.grid.set(self.start, Cell::Empty);

        while let Some(&c) = self.grid.get(pos) {
            if let Cell::Empty = c {
                cnt += 1;
                self.grid.set(pos, Cell::Passed);
            }

            while let Some(&Cell::Wall) = self.grid.get(pos + dir) {
                dir = dir.right();
            }

            pos += dir;
        }

        Ok(cnt)
    }

    fn part2(mut self) -> Result<usize> {
        let mut pos = self.start;
        let mut dir = Direction::North;
        let mut cnt = 0;

        while let Some(&c) = self.grid.get(pos) {
            if let Cell::Empty = c {
                self.grid.set(pos, Cell::Wall);
                if self.loops(pos - dir, dir, pos) {
                    cnt += 1;
                }
                self.grid.set(pos, Cell::Passed);
            }

            while let Some(&Cell::Wall) = self.grid.get(pos + dir) {
                dir = dir.right();
            }

            pos += dir;
        }

        Ok(cnt)
    }
}

aoc_main!(Day06);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day06, 1, EXAMPLE, 41);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day06, 2, EXAMPLE, 6);
    }
}
