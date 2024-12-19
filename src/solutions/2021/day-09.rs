use std::cmp::Reverse;

use aoc_rust::*;
use common::*;

struct Day09 {
    cave: Grid<u8>,
}

impl Problem<usize, usize> for Day09 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Grid::parse(anychar.verify(char::is_ascii_digit).map(|c| c as u8 - b'0'))
            .map(|cave| Day09 { cave })
            .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self
            .cave
            .coordinates()
            .filter(|&pos| {
                Direction::cardinal()
                    .flat_map(|dir| self.cave.get(pos + dir))
                    .all(|&l| l > self.cave[pos])
            })
            .map(|pos| self.cave[pos] as usize + 1)
            .sum())
    }

    fn part2(self) -> Result<usize> {
        Ok(self
            .cave
            .coordinates()
            .filter(|&pos| {
                Direction::cardinal()
                    .flat_map(|dir| self.cave.get(pos + dir))
                    .all(|&l| l > self.cave[pos])
            })
            .map(|basin_drain| {
                let mut visited = HashSet::new();
                let mut q = VecDeque::new();
                q.push_back(basin_drain);

                while let Some(pos) = q.pop_front() {
                    if visited.contains(&pos) {
                        continue;
                    }
                    visited.insert(pos);
                    for next in Direction::cardinal().map(|dir| pos + dir) {
                        if let Some(&l) = self.cave.get(next) {
                            if l < 9 {
                                q.push_back(next);
                            }
                        }
                    }
                }

                visited.len()
            })
            .sorted_by_key(|&size| Reverse(size))
            .take(3)
            .product())
    }
}

aoc_main!(Day09);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day09, 1, EXAMPLE, 15);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day09, 2, EXAMPLE, 1134);
    }
}
