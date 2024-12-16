use aoc_rust::*;
use common::*;

struct Day01 {
    moves: Vec<Move>,
}

enum Move {
    Up,
    Down,
}

impl Move {
    fn parse(input: &mut &str) -> PResult<Move> {
        alt(('('.map(|_| Move::Up), ')'.map(|_| Move::Down))).parse_next(input)
    }
    fn apply(&self, floor: i32) -> i32 {
        match self {
            Move::Up => floor + 1,
            Move::Down => floor - 1,
        }
    }
}

impl Problem<i32, usize> for Day01 {
    fn parse(input: &mut &str) -> PResult<Self> {
        repeat(0.., Move::parse)
            .map(|moves| Day01 { moves })
            .parse_next(input)
    }

    fn part1(self) -> Result<i32> {
        Ok(self.moves.iter().fold(0, |floor, m| m.apply(floor)))
    }

    fn part2(self) -> Result<usize> {
        let mut floor = 0;
        for (i, m) in self.moves.iter().enumerate() {
            floor = m.apply(floor);
            if floor == -1 {
                return Ok(i + 1);
            }
        }
        Err(AoCError::NoSolution)
    }
}

aoc_main!(Day01);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_task!(Day01, 1, "(())", 0);
        assert_task!(Day01, 1, "()()", 0);
        assert_task!(Day01, 1, "(((", 3);
        assert_task!(Day01, 1, "(()(()(", 3);
        assert_task!(Day01, 1, "))(((((", 3);
        assert_task!(Day01, 1, "())", -1);
        assert_task!(Day01, 1, "))(", -1);
        assert_task!(Day01, 1, ")))", -3);
        assert_task!(Day01, 1, ")())())", -3);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day01, 2, ")", 1);
        assert_task!(Day01, 2, "()())", 5);
    }
}
