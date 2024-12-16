use aoc_rust::*;
use common::*;

struct Day09 {
    readings: Vec<Reading>,
}

struct Reading {
    numbers: Vec<i32>,
}

impl Reading {
    fn parse(input: &mut &str) -> PResult<Reading> {
        list(dec_i32, space1)
            .map(|numbers| Reading { numbers })
            .parse_next(input)
    }

    fn diff(&self) -> Reading {
        Reading {
            numbers: self.numbers.windows(2).map(|w| w[1] - w[0]).collect(),
        }
    }

    fn get_next_value(&self) -> i32 {
        if self.numbers.iter().all(|n| *n == 0) {
            return 0;
        }
        let diff = self.diff();

        self.numbers[self.numbers.len() - 1] + diff.get_next_value()
    }

    fn get_previous_value(&self) -> i32 {
        if self.numbers.iter().all(|n| *n == 0) {
            return 0;
        }
        let diff = self.diff();

        self.numbers[0] - diff.get_previous_value()
    }
}

impl Problem<i32, i32> for Day09 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(Reading::parse, line_ending)
            .map(|readings| Self { readings })
            .parse_next(input)
    }

    fn part1(self) -> Result<i32> {
        Ok(self.readings.iter().map(|r| r.get_next_value()).sum())
    }

    fn part2(self) -> Result<i32> {
        Ok(self.readings.iter().map(|r| r.get_previous_value()).sum())
    }
}

aoc_main!(Day09);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn test_part1() {
        assert_task!(Day09, 1, EXAMPLE, 114)
    }

    #[test]
    fn test_part2() {
        assert_task!(Day09, 2, EXAMPLE, 2)
    }
}
