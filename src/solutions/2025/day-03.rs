use aoc_rust::*;
use common::*;

struct Day03 {
    banks: Vec<Vec<u8>>,
}

impl Day03 {
    fn joltage(bank: Vec<u8>, size: usize) -> u64 {
        (bank.len() - size..bank.len())
            .fold((0, 0), |(j, start), i| {
                let (cell, cell_j) =
                    bank[start..=i]
                        .iter()
                        .enumerate()
                        .fold(
                            (0, 0),
                            |(pi, px), (ci, cx)| if *cx > px { (ci, *cx) } else { (pi, px) },
                        );
                (10 * j + cell_j as u64, start + cell + 1)
            })
            .0
    }
}

impl Problem<u64, u64> for Day03 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Ok(Day03 {
            banks: list(many(digit), line_ending).parse_next(input)?,
        })
    }

    fn part1(self) -> Result<u64> {
        Ok(self
            .banks
            .into_iter()
            .map(|bank| Day03::joltage(bank, 2))
            .sum())
    }

    fn part2(self) -> Result<u64> {
        Ok(self
            .banks
            .into_iter()
            .map(|bank| Day03::joltage(bank, 12))
            .sum())
    }
}

aoc_main!(Day03);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day03, 1, EXAMPLE, 357);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day03, 2, EXAMPLE, 3121910778619u64);
    }
}
