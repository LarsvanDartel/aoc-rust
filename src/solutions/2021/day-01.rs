use aoc_rust::*;
use common::*;
use itertools::Itertools;

struct Day01 {
    measurements: Vec<i32>,
}

impl Problem<usize, usize> for Day01 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, parse_i32)
            .map(|measurements| Day01 { measurements })
            .parse(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self.measurements.windows(2).filter(|w| w[0] < w[1]).count())
    }

    fn part2(self) -> Result<usize> {
        Ok(self
            .measurements
            .windows(3)
            .map(|w| w[0] + w[1] + w[2])
            .tuple_windows::<(_, _)>()
            .filter(|w| w.0 < w.1)
            .count())
    }
}

aoc_main!(Day01);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
    "#;

    #[test]
    fn test_part1() {
        assert_task!(Day01, 1, EXAMPLE, ());
    }

    #[test]
    fn test_part2() {
        assert_task!(Day01, 2, EXAMPLE, ());
    }
}
