use aoc_rust::*;
use common::*;

struct Test {
    value: u64,
    equation: Vec<u64>,
}

enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Operation {
    fn apply_backwards(&self, result: u64, operand: u64) -> Option<u64> {
        match self {
            Self::Add => result.checked_sub(operand),
            Self::Multiply => {
                if result % operand == 0 {
                    Some(result / operand)
                } else {
                    None
                }
            }
            Self::Concatenate => {
                let mut p10 = 1;
                while p10 <= operand {
                    p10 *= 10;
                }
                if result % p10 == operand {
                    Some(result / p10)
                } else {
                    None
                }
            }
        }
    }
}

impl Test {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_pair(parse_u64, tag(": "), separated_list1(space1, parse_u64))
            .map(|(value, equation)| Self { value, equation })
            .parse(input)
    }

    fn is_valid(&self, operations: &[Operation]) -> bool {
        let mut possible = vec![self.value];
        for val in self.equation.iter().skip(1).rev() {
            let mut next = Vec::new();
            for res in possible {
                for op in operations {
                    if let Some(res) = op.apply_backwards(res, *val) {
                        next.push(res);
                    }
                }
            }
            possible = next;
        }
        possible.contains(&self.equation[0])
    }
}

struct Day07 {
    tests: Vec<Test>,
}

impl Problem<u64, u64> for Day07 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, Test::parse)
            .map(|tests| Self { tests })
            .parse(input)
    }

    fn part1(self) -> Result<u64> {
        Ok(self
            .tests
            .iter()
            .filter(|t| t.is_valid(&[Operation::Add, Operation::Multiply]))
            .map(|t| t.value)
            .sum())
    }

    fn part2(self) -> Result<u64> {
        Ok(self
            .tests
            .iter()
            .filter(|t| t.is_valid(&[Operation::Add, Operation::Multiply, Operation::Concatenate]))
            .map(|t| t.value)
            .sum())
    }
}

aoc_main!(Day07);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day07, 1, EXAMPLE, 3749);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day07, 2, EXAMPLE, 11387);
    }
}
