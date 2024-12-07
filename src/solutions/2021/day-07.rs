use aoc_rust::*;
use common::*;

struct Day07 {
    crabs: Vec<i32>,
}

impl Problem<usize, usize> for Day07 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(tag(","), parse_i32)
            .map(|crabs| Day07 { crabs })
            .parse(input)
    }

    fn part1(mut self) -> Result<usize> {
        self.crabs.sort_unstable();
        let median = if self.crabs.len() % 2 == 0 {
            let mid = self.crabs.len() / 2;
            (self.crabs[mid - 1] + self.crabs[mid]) / 2
        } else {
            self.crabs[self.crabs.len() / 2]
        };
        Ok(self
            .crabs
            .iter()
            .map(|&c| (c - median).unsigned_abs() as usize)
            .sum())
    }
    fn part2(self) -> Result<usize> {
        let mean = self.crabs.iter().sum::<i32>() / self.crabs.len() as i32;
        let a = self
            .crabs
            .iter()
            .map(|&c| (c - mean).unsigned_abs())
            .map(|n| n * (n + 1) / 2)
            .sum::<u32>() as usize;
        let b = self
            .crabs
            .iter()
            .map(|&c| (c - mean - 1).unsigned_abs())
            .map(|n| n * (n + 1) / 2)
            .sum::<u32>() as usize;
        Ok(a.min(b))
    }
}

aoc_main!(Day07);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"16,1,2,0,4,2,7,1,2,14
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day07, 1, EXAMPLE, 37);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day07, 2, EXAMPLE, 168);
    }
}
