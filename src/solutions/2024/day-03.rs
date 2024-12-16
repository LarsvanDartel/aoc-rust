use aoc_rust::*;
use common::*;

enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

impl Instruction {
    fn parse(input: &mut &str) -> PResult<Self> {
        alt((
            delimited("mul(", separated_pair(dec_i32, ",", dec_i32), ")")
                .map(|(a, b)| Self::Mul(a, b)),
            "do()".map(|_| Self::Do),
            "don't()".map(|_| Self::Dont),
        ))
        .parse_next(input)
    }
}

struct Day03 {
    instructions: Vec<Instruction>,
}

impl Problem<i32, i32> for Day03 {
    fn parse(input: &mut &str) -> PResult<Self> {
        repeat(
            0..,
            repeat_till(0.., anychar, Instruction::parse).map(|(_, i): ((), Instruction)| i),
        )
        .map(|instructions| Self { instructions })
        .parse_next(input)
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
                },
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
