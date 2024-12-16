use aoc_rust::*;
use common::*;

struct Day01 {
    measurements: Vec<i32>,
}

impl Problem<usize, usize> for Day01 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(0.., dec_int::<_, i32, _>, line_ending)
            .map(|measurements| Day01 { measurements })
            .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self.measurements.windows(2).filter(|w| w[0] < w[1]).count())
    }

    fn part2(self) -> Result<usize> {
        Ok(self
            .measurements
            .windows(3)
            .map(|w| w[0] + w[1] + w[2])
            .tuple_windows()
            .filter(|(a, b)| a < b)
            .count())
    }
}

aoc_main!(Day01);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"199
200
208
210
200
207
240
269
260
263
    "#;

    #[test]
    fn test_part1() {
        assert_task!(Day01, 1, EXAMPLE, 7);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day01, 2, EXAMPLE, 5);
    }
}
