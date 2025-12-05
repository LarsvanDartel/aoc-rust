use std::ops::RangeInclusive;

use aoc_rust::*;
use common::*;

struct Day05 {
    ranges: Vec<RangeInclusive<u64>>,
    ids: Vec<u64>,
}

impl Problem<usize, usize> for Day05 {
    fn parse(input: &mut &str) -> PResult<Self> {
        seq!(Day05 {
            ranges: list(
                separated_pair(dec_u64, '-', dec_u64).map(|(a, b)| a..=b),
                line_ending
            ),
            _: (line_ending, line_ending),
            ids: list(dec_u64, line_ending)
        })
        .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self
            .ids
            .into_iter()
            .filter(|x| self.ranges.iter().any(|r| r.contains(x)))
            .count())
    }

    fn part2(self) -> Result<usize> {
        let mut ranges = self.ranges.into_iter().sorted_by_key(|r| *r.start());
        let r = ranges.next().unwrap();
        let (sum, range) = ranges.fold(
            (0, r),
            |(acc, curr), next| {
                if *next.start() <= *curr.end() {
                    (acc, *curr.start()..=*next.end().max(curr.end()))
                } else {
                    (acc + curr.count(), next)
                }
            },
        );
        Ok(sum + range.count())
    }
}

aoc_main!(Day05);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day05, 1, EXAMPLE, 3);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day05, 2, EXAMPLE, 14);
    }
}
