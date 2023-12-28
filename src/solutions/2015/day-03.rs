use std::collections::HashSet;

use aoc_rust::*;

use nom::{branch::alt, bytes::complete::tag, multi::many1, Parser};

struct Day03 {
    moves: Vec<Move>,
}

enum Move {
    North,
    South,
    East,
    West,
}

impl Move {
    fn parse(input: &str) -> ParseResult<Self> {
        alt((
            tag("^").map(|_| Move::North),
            tag("v").map(|_| Move::South),
            tag(">").map(|_| Move::East),
            tag("<").map(|_| Move::West),
        ))
        .parse(input)
    }
}

impl Problem<usize, usize> for Day03 {
    fn parse(input: &str) -> ParseResult<Self> {
        many1(Move::parse).map(|moves| Self { moves }).parse(input)
    }

    fn part1(self) -> Result<usize> {
        let mut houses = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        houses.insert((x, y));
        for m in &self.moves {
            match m {
                Move::North => y += 1,
                Move::South => y -= 1,
                Move::East => x += 1,
                Move::West => x -= 1,
            }
            houses.insert((x, y));
        }
        Ok(houses.len())
    }

    fn part2(self) -> Result<usize> {
        let mut houses = HashSet::new();
        let mut santa_x = 0;
        let mut santa_y = 0;

        let mut robo_x = 0;
        let mut robo_y = 0;

        houses.insert((santa_x, santa_y));

        for (i, m) in self.moves.iter().enumerate() {
            let (x, y) = match i % 2 {
                0 => (&mut santa_x, &mut santa_y),
                1 => (&mut robo_x, &mut robo_y),
                _ => unreachable!(),
            };
            match m {
                Move::North => *y += 1,
                Move::South => *y -= 1,
                Move::East => *x += 1,
                Move::West => *x -= 1,
            }
            houses.insert((*x, *y));
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
