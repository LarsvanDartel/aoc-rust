use std::collections::BinaryHeap;

use aoc_rust::*;
use common::*;

struct Day17 {
    grid: Vec<Vec<u32>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
    None,
}

impl Direction {
    const fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    const fn opposite(&self) -> Direction {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::None => Self::None,
        }
    }
}

impl std::ops::Add<Direction> for (isize, isize) {
    type Output = (isize, isize);

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::None => panic!("Cannot add None direction"),
            Direction::North => (self.0, self.1 - 1),
            Direction::East => (self.0 + 1, self.1),
            Direction::South => (self.0, self.1 + 1),
            Direction::West => (self.0 - 1, self.1),
        }
    }
}

impl std::ops::Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::None => panic!("Cannot add None direction"),
            Direction::North => (self.0, self.1 - 1),
            Direction::East => (self.0 + 1, self.1),
            Direction::South => (self.0, self.1 + 1),
            Direction::West => (self.0 - 1, self.1),
        }
    }
}

impl std::ops::AddAssign<Direction> for (isize, isize) {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

impl std::ops::AddAssign<Direction> for (usize, usize) {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

impl Day17 {
    fn find_path(&self, min_length: usize, max_length: usize) -> Option<u32> {
        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::new();
        let mut distances = HashMap::new();

        queue.push(std::cmp::Reverse((0, ((0, 0), Direction::None))));

        while let Some(std::cmp::Reverse((dist, node))) = queue.pop() {
            let (pos, dir) = node;
            if pos == (self.grid[0].len() - 1, self.grid.len() - 1) {
                return Some(dist);
            }

            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);

            for new_dir in Direction::all() {
                if dir == Direction::None && new_dir != Direction::East {
                    continue;
                }
                if new_dir == dir || new_dir == dir.opposite() {
                    continue;
                }

                let mut weight = 0;
                let mut new_pos = (pos.0 as isize, pos.1 as isize);

                for len in 1..=max_length {
                    new_pos += new_dir;
                    if new_pos.0 < 0
                        || new_pos.1 < 0
                        || new_pos.1 >= self.grid.len() as isize
                        || new_pos.0 >= self.grid[new_pos.1 as usize].len() as isize
                    {
                        break;
                    }
                    let new_pos = (new_pos.0 as usize, new_pos.1 as usize);

                    weight += self.grid[new_pos.1][new_pos.0];

                    if len < min_length {
                        continue;
                    }

                    let new_node = (new_pos, new_dir);
                    let new_dist = dist + weight;

                    if let Some(prev_dist) = distances.get(&new_node) {
                        if *prev_dist <= new_dist {
                            continue;
                        }
                    }
                    distances.insert(new_node, new_dist);
                    queue.push(std::cmp::Reverse((new_dist, new_node)));
                }
            }
        }

        None
    }
}

impl Problem<u32, u32> for Day17 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(
            many(one_of('0'..='9').map(|c: char| c.to_digit(10).unwrap())),
            line_ending,
        )
        .map(|grid| Self { grid })
        .parse_next(input)
    }

    fn part1(self) -> Result<u32> {
        Ok(self.find_path(1, 3).unwrap())
    }

    fn part2(self) -> Result<u32> {
        Ok(self.find_path(4, 10).unwrap())
    }
}

aoc_main!(Day17);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    const EXAMPLE_2: &str = r#"111111111111
999999999991
999999999991
999999999991
999999999991"#;

    #[test]
    fn test_part1() {
        assert_task!(Day17, 1, EXAMPLE_1, 102);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day17, 2, EXAMPLE_1, 94);
        assert_task!(Day17, 2, EXAMPLE_2, 71);
    }
}
