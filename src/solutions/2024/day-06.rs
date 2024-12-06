use aoc_rust::*;
use common::*;
use hashbrown::HashSet;

struct Day06 {
    grid: Grid<char>,
}

impl Problem<usize, usize> for Day06 {
    fn parse(input: &str) -> ParseResult<Self> {
        Grid::parse(one_of(".#^"))
            .map(|grid| Self { grid })
            .parse(input)
    }

    fn part1(self) -> Result<usize> {
        let start = self
            .grid
            .coordinates()
            .find(|&c| self.grid.get(c) == Some(&'^'))
            .ok_or("No starting position")?;

        let mut visited = HashSet::new();
        let mut pos = start;
        let mut dir = Direction::North;

        while self.grid.contains(pos) {
            visited.insert(pos);
            while self.grid.get(pos + dir) == Some(&'#') {
                dir = dir.right();
            }
            pos += dir;
        }

        Ok(visited.len())
    }

    fn part2(self) -> Result<usize> {
        let start = self
            .grid
            .coordinates()
            .find(|&c| self.grid.get(c) == Some(&'^'))
            .ok_or("No starting position")?;

        let mut visited = HashSet::new();
        let mut pos = start;
        let mut dir = Direction::North;

        while self.grid.contains(pos) {
            visited.insert(pos);
            while self.grid.get(pos + dir) == Some(&'#') {
                dir = dir.right();
            }
            pos += dir;
        }

        Ok(visited
            .into_iter()
            .filter(|&box_pos| {
                let mut pos = start;
                let mut dir = Direction::North;
                let mut visited = HashSet::new();

                while self.grid.contains(pos) {
                    if visited.contains(&(pos, dir)) {
                        return true;
                    }
                    visited.insert((pos, dir));
                    while self.grid.get(pos + dir) == Some(&'#') || pos + dir == box_pos {
                        dir = dir.right();
                    }
                    pos += dir;
                }

                false
            })
            .count())
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
