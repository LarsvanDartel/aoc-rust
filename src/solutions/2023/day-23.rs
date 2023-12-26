use aoc_rust::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{many1, separated_list1},
    Parser,
};

struct Day23 {
    map: Vec<Vec<Path>>,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Path {
    Empty,
    Wall,
    Slope(Direction),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn all() -> impl Iterator<Item = Self> {
        use Direction::*;
        [North, South, East, West].iter().copied()
    }
}

impl std::ops::Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        use Direction::*;
        match rhs {
            North => (self.0 - 1, self.1),
            South => (self.0 + 1, self.1),
            East => (self.0, self.1 + 1),
            West => (self.0, self.1 - 1),
        }
    }
}

impl Day23 {
    fn find_longest_path(&self) -> usize {
        let mut visited = vec![vec![false; self.map[0].len()]; self.map.len()];
        self.find_longest_path_recursive(self.start.1, self.start.0, &mut visited) - 1
    }

    fn find_longest_path_recursive(
        &self,
        x: usize,
        y: usize,
        visited: &mut Vec<Vec<bool>>,
    ) -> usize {
        if (y, x) == self.end {
            return 1;
        }
        visited[y][x] = true;
        let mut max = 0;
        for dir in Direction::all() {
            if (y, x) == self.start && dir != Direction::South {
                continue;
            }
            if let Path::Slope(d) = self.map[y][x] {
                if d != dir {
                    continue;
                }
            }
            let (y, x) = (y, x) + dir;
            if !visited[y][x] && self.map[y][x] != Path::Wall {
                max = max.max(self.find_longest_path_recursive(x, y, visited));
            }
        }
        visited[y][x] = false;
        if max == 0 {
            return 0;
        }
        max + 1
    }
}

impl std::fmt::Debug for Day23 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.map.iter().enumerate() {
            for (j, path) in row.iter().enumerate() {
                if (i, j) == self.start {
                    write!(f, "S")?;
                } else if (i, j) == self.end {
                    write!(f, "E")?;
                } else {
                    match path {
                        Path::Empty => write!(f, ".")?,
                        Path::Wall => write!(f, "#")?,
                        Path::Slope(Direction::North) => write!(f, "^")?,
                        Path::Slope(Direction::South) => write!(f, "v")?,
                        Path::Slope(Direction::West) => write!(f, "<")?,
                        Path::Slope(Direction::East) => write!(f, ">")?,
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn max_distance(
    start: usize,
    end: usize,
    adj: &Vec<Vec<(usize, usize)>>,
    visited: &mut Vec<bool>,
) -> usize {
    if start == end {
        return 0;
    }
    visited[start] = true;
    let mut max = 0;
    for (i, dist) in adj[start].iter() {
        if visited[*i] {
            continue;
        }
        max = max.max(dist + max_distance(*i, end, adj, visited));
    }
    visited[start] = false;
    max
}

impl Problem<usize, usize> for Day23 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(
            newline,
            many1(alt((
                tag(".").map(|_| Path::Empty),
                tag("#").map(|_| Path::Wall),
                tag("^").map(|_| Path::Slope(Direction::North)),
                tag("v").map(|_| Path::Slope(Direction::South)),
                tag("<").map(|_| Path::Slope(Direction::West)),
                tag(">").map(|_| Path::Slope(Direction::East)),
            ))),
        )
        .map(|map| {
            let start = (0, map[0].iter().position(|p| p == &Path::Empty).unwrap());
            let end = (
                map.len() - 1,
                map[map.len() - 1]
                    .iter()
                    .position(|p| p == &Path::Empty)
                    .unwrap(),
            );
            Self { map, start, end }
        })
        .parse(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self.find_longest_path())
    }

    fn part2(self) -> Result<usize> {
        let grid = self.map.clone();

        let mut nodes = vec![];

        for (i, row) in grid.iter().enumerate() {
            for (j, path) in row.iter().enumerate() {
                if (i, j) == self.start || (i, j) == self.end {
                    nodes.push((i, j));
                    continue;
                }
                if path == &Path::Wall {
                    continue;
                }
                let mut neighbors = vec![];
                for dir in Direction::all() {
                    let (y, x) = (i, j) + dir;
                    if grid[y][x] != Path::Wall {
                        neighbors.push((y, x));
                    }
                }
                if neighbors.len() > 2 {
                    nodes.push((i, j));
                }
            }
        }

        let mut adj = vec![vec![]; nodes.len()];
        for (i, node) in nodes.iter().enumerate() {
            for dir in Direction::all() {
                if node == &self.start && dir != Direction::South {
                    continue;
                }
                if node == &self.end && dir != Direction::North {
                    continue;
                }
                let mut dist = 1;
                let mut pos = *node + dir;
                let mut prev = *node;
                'walk: loop {
                    if grid[pos.0][pos.1] == Path::Wall {
                        break;
                    }
                    if nodes.contains(&pos) {
                        adj[i].push((nodes.iter().position(|n| n == &pos).unwrap(), dist));
                        break;
                    }
                    for dir in Direction::all() {
                        let (y, x) = pos + dir;
                        if grid[y][x] != Path::Wall && (y, x) != prev {
                            prev = pos;
                            pos = (y, x);
                            dist += 1;
                            continue 'walk;
                        }
                    }
                    break;
                }
            }
        }

        let mut visited = vec![false; nodes.len()];

        Ok(max_distance(
            nodes.iter().position(|n| n == &self.start).unwrap(),
            nodes.iter().position(|n| n == &self.end).unwrap(),
            &adj,
            &mut visited,
        ))
    }
}

aoc_main!(Day23);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

    #[test]
    fn test_part1() {
        assert_task!(Day23, 1, EXAMPLE, 94);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day23, 2, EXAMPLE, 154);
    }
}
