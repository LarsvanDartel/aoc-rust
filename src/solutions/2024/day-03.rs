use aoc_rust::*;
use common::*;
use nom::{character::complete::anychar, multi::many_till};

enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

impl Instruction {
    fn parse(input: &str) -> ParseResult<Self> {
        alt((
            delimited(
                tag("mul("),
                separated_pair(parse_i32, tag(","), parse_i32),
                tag(")"),
            )
            .map(|(a, b)| Self::Mul(a, b)),
            tag("do()").map(|_| Self::Do),
            tag("don't()").map(|_| Self::Dont),
        ))
        .parse(input)
    }
}

struct Day03 {
    instructions: Vec<Instruction>,
}

impl Problem<i32, i32> for Day03 {
    fn parse(input: &str) -> ParseResult<Self> {
        // instructions separated by any sequence of characters
        many1(
            many_till(anychar, Instruction::parse)
                .map(|(_, i)| i)
                .map(|i| i),
        )
        .map(|instructions| Self { instructions })
        .parse(input)
    }

    fn part1(self) -> Result<i32> {
        Ok(self
            .instructions
            .iter()
            .map(|i| match i {
                Instruction::Mul(a, b) => a * b,
                Instruction::Do => 0,
                Instruction::Dont => 0,
            })
            .sum())
    }

    fn part2(self) -> Result<i32> {
        let mut sum = 0;
        let mut d = true;
        for i in self.instructions {
            match i {
                Instruction::Mul(a, b) => {
                    if d {
                        sum += a * b
                    }
                }
                Instruction::Do => d = true,
                Instruction::Dont => d = false,
            }
        }
        Ok(sum)
    }
}

aoc_main!(Day03);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_task!(
            Day03,
            1,
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            161
        );
    }

    #[test]
    fn test_part2() {
        assert_task!(
            Day03,
            2,
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            48
        );
    }
}
