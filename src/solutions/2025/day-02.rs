use std::ops::RangeInclusive;

use aoc_rust::*;
use common::*;
use num_integer::Integer;
use num_traits::PrimInt;

struct Day02 {
    ranges: Vec<RangeInclusive<usize>>,
}

impl Day02 {
    fn has_repetition(mut i: usize, n: usize) -> bool {
        let pow = 10.pow(n as u32);
        let seq = i % pow;
        i /= pow;
        if i == 0 {
            return false;
        }
        while i > 0 {
            if (i % pow) != seq {
                return false;
            }
            i /= pow;
        }
        true
    }
}

impl Problem<usize, usize> for Day02 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Ok(Day02 {
            ranges: list(
                separated_pair(dec_usize, '-', dec_usize).map(|(a, b)| a..=b),
                ',',
            )
            .parse_next(input)?,
        })
    }

    fn part1(self) -> Result<usize> {
        Ok(self
            .ranges
            .into_iter()
            .flat_map(|r| {
                r.filter(|&x| {
                    let l: usize = log_floor(x, 10);
                    l.is_even() && Day02::has_repetition(x, l / 2)
                })
            })
            .sum())
    }

    fn part2(self) -> Result<usize> {
        Ok(self
            .ranges
            .into_iter()
            .flat_map(|r| {
                r.filter(|&x| {
                    let l: usize = log_floor(x, 10);
                    (1..=l / 2).any(|n| Day02::has_repetition(x, n))
                })
            })
            .sum())
    }
}

aoc_main!(Day02);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn test_part1() {
        assert_task!(Day02, 1, EXAMPLE, 1227775554usize);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day02, 2, EXAMPLE, 4174379265usize);
    }
}
