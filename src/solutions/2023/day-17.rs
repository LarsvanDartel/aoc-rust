use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_rust::*;

use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    Parser,
};

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
        use Direction::*;

        match self {
            North => South,
            East => West,
            South => North,
            West => East,
            None => None,
        }
    }
}

impl std::ops::Add<Direction> for (isize, isize) {
    type Output = (isize, isize);

    fn add(self, rhs: Direction) -> Self::Output {
        use Direction::*;

        match rhs {
            None => panic!("Cannot add None direction"),
            North => (self.0, self.1 - 1),
            East => (self.0 + 1, self.1),
            South => (self.0, self.1 + 1),
            West => (self.0 - 1, self.1),
        }
    }
}

impl std::ops::Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        use Direction::*;

        match rhs {
            None => panic!("Cannot add None direction"),
            North => (self.0, self.1 - 1),
            East => (self.0 + 1, self.1),
            South => (self.0, self.1 + 1),
            West => (self.0 - 1, self.1),
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
        let mut prev = HashMap::new();

        queue.push(std::cmp::Reverse((0, ((0, 0), Direction::None))));

        while let Some(std::cmp::Reverse((dist, node))) = queue.pop() {
            let (pos, dir) = node;
            if pos == (self.grid[0].len() - 1, self.grid.len() - 1) {
                //self.print_path(node, &prev);
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
                    prev.insert(new_node, node);
                    queue.push(std::cmp::Reverse((new_dist, new_node)));
                }
            }
        }

        None
    }

    #[allow(dead_code)]
    fn print_path(
        &self,
        node: ((usize, usize), Direction),
        prev: &HashMap<((usize, usize), Direction), ((usize, usize), Direction)>,
    ) {
        let mut node = node;
        let mut path = vec![];
        while let Some(&prev_node) = prev.get(&node) {
            while node.0 != prev_node.0 {
                path.push(node);
                node.0 += node.1.opposite();
            }
            node = prev_node;
        }
        path.push(node);
        path.reverse();

        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if let Some((_, dir)) = path.iter().find(|(pos, _)| *pos == (x, y)) {
                    print!(
                        "{}",
                        match dir {
                            Direction::North => '↑',
                            Direction::East => '→',
                            Direction::South => '↓',
                            Direction::West => '←',
                            Direction::None => ' ',
                        }
                    );
                } else {
                    print!("{}", self.grid[y][x]);
                }
            }
            println!();
        }

        println!();
        println!();
    }
}

impl Problem<u32, u32> for Day17 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(
            line_ending,
            many1(one_of("0123456789").map(|c| c.to_digit(10).unwrap())),
        )
        .map(|grid| Self { grid })
        .parse(input)
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
