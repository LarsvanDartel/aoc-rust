use std::collections::HashSet;

use aoc_rust::*;
use aoc_rust::common::{Vec2, Direction};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32 as parse_u32,
    multi::separated_list1, Parser,
};

struct Day01 {
    instructions: Vec<Instruction>,
}

struct Instruction {
    turn_direction: bool,
    distance: u32,
}

impl Instruction {
    fn parse(input: &str) -> ParseResult<Self> {
        let (input, turn_direction) =
            alt((tag("R").map(|_| true), tag("L").map(|_| false))).parse(input)?;
        let (input, distance) = parse_u32(input)?;
        Ok((
            input,
            Instruction {
                turn_direction,
                distance,
            },
        ))
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction = if self.turn_direction { "R" } else { "L" };
        write!(f, "{}{}", direction, self.distance)
    }
}

impl Problem<isize, isize> for Day01 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(tag(", "), Instruction::parse)
            .map(|instructions| Self { instructions })
            .parse(input)
    }

    fn part1(self) -> Result<isize> {
        let mut position = Vec2::new(0, 0);
        let mut direction = Direction::North;

        for instruction in self.instructions {
            if instruction.turn_direction {
                direction = direction.right();
            } else {
                direction = direction.left();
            }

            position += direction * instruction.distance as isize;
        }

        Ok(position.x.abs() + position.y.abs())
    }

    fn part2(self) -> Result<isize> {
        let mut position = Vec2::new(0, 0);
        let mut direction = Direction::North;
        let mut visited = HashSet::new();

        for instruction in self.instructions {
            if instruction.turn_direction {
                direction = direction.right();
            } else {
                direction = direction.left();
            }

            for _ in 0..instruction.distance {
                position += direction;
                if !visited.insert(position) {
                    return Ok(position.x.abs() + position.y.abs());
                }
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
        assert_task!(Day01, 1, "R2, L3", 5);
        assert_task!(Day01, 1, "R2, R2, R2", 2);
        assert_task!(Day01, 1, "R5, L5, R5, R3", 12);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day01, 2, "R8, R4, R4, R8", 4);
    }
}
