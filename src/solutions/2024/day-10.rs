use aoc_rust::*;
use common::*;
use hashbrown::HashSet;

struct Day10 {
    grid: Grid<u8>,
}

impl Day10 {
    fn solve(&self, pos: Vec2<isize>) -> Vec<Vec2<isize>> {
        if self.grid[pos] == 9 {
            return vec![pos];
        }

        let mut v = Vec::new();
        for d in Direction::cardinal() {
            let new_pos = pos + d;
            if self.grid.contains(new_pos) && self.grid[new_pos] == self.grid[pos] + 1 {
                v.extend(self.solve(new_pos));
            }
        }

        v
    }
}

impl Problem<usize, usize> for Day10 {
    fn parse(input: &str) -> ParseResult<Self> {
        Grid::parse(verify(anychar, char::is_ascii_digit).map(|c| c as u8 - b'0'))
            .map(|grid| Day10 { grid })
            .parse(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self
            .grid
            .coordinates()
            .filter(|&c| self.grid[c] == 0)
            .map(|pos| self.solve(pos).iter().collect::<HashSet<_>>().len())
            .sum())
    }

    fn part2(self) -> Result<usize> {
        Ok(self
            .grid
            .coordinates()
            .filter(|&c| self.grid[c] == 0)
            .map(|pos| self.solve(pos).len())
            .sum())
    }
}

aoc_main!(Day10);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day10, 1, EXAMPLE, 36);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day10, 2, EXAMPLE, 81);
    }
}
