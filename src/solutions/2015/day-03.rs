use aoc_rust::*;
use common::*;

struct Day03 {
    moves: Vec<Direction>,
}

impl Problem<usize, usize> for Day03 {
    fn parse(input: &mut &str) -> PResult<Self> {
        repeat(0.., Direction::parse_arrows)
            .map(|moves| Self { moves })
            .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        let mut houses = HashSet::new();
        let mut pos = Vec2::new(0, 0);
        houses.insert(pos);
        for &m in &self.moves {
            pos += m;
            houses.insert(pos);
        }
        Ok(houses.len())
    }

    fn part2(self) -> Result<usize> {
        let mut houses = HashSet::new();
        let mut santa = Vec2::new(0, 0);
        let mut robot = Vec2::new(0, 0);

        houses.insert(santa);

        for (i, &m) in self.moves.iter().enumerate() {
            if i % 2 == 0 {
                santa += m;
                houses.insert(santa);
            } else {
                robot += m;
                houses.insert(robot);
            }
        }

        Ok(houses.len())
    }
}

aoc_main!(Day03);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_task!(Day03, 1, ">", 2);
        assert_task!(Day03, 1, "^>v<", 4);
        assert_task!(Day03, 1, "^v^v^v^v^v", 2);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day03, 2, "^v", 3);
        assert_task!(Day03, 2, "^>v<", 3);
        assert_task!(Day03, 2, "^v^v^v^v^v", 11);
    }
}
