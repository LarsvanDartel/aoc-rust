use aoc_rust::*;
use common::*;

struct Day01 {
    values: Vec<CalibrationValue>,
}

struct CalibrationValue {
    line: String,
}

impl CalibrationValue {
    fn parse(input: &mut &str) -> PResult<Self> {
        alphanumeric1
            .map(|value: &str| Self {
                line: value.to_string(),
            })
            .parse_next(input)
    }

    fn value(&self) -> u32 {
        let mut first = None;
        let mut last = None;

        for c in self.line.chars() {
            if c.is_ascii_digit() {
                if first.is_none() {
                    first = Some(c);
                }
                last = Some(c);
            }
        }

        let first = first.unwrap().to_digit(10).unwrap();
        let last = last.unwrap().to_digit(10).unwrap();
        first * 10 + last
    }

    fn value_with_letters(&self) -> u32 {
        let mut line = self.line.clone();
        for (i, s) in [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .enumerate()
        {
            line = line.replace(s, &format!("{}{}{}", s, i + 1, s));
        }

        Self { line }.value()
    }
}

impl Problem<u32, u32> for Day01 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(CalibrationValue::parse, line_ending)
            .map(|values| Self { values })
            .parse_next(input)
    }

    fn part1(self) -> Result<u32> {
        Ok(self.values.iter().map(|v| v.value()).sum())
    }

    fn part2(self) -> Result<u32> {
        Ok(self.values.iter().map(|v| v.value_with_letters()).sum())
    }
}

aoc_main!(Day01);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part1() {
        assert_task!(Day01, 1, EXAMPLE_1, 142)
    }

    #[test]
    fn test_part2() {
        assert_task!(Day01, 2, EXAMPLE_2, 281)
    }
}
