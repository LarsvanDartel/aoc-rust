use std::iter::zip;

use aoc_rust::*;
use common::*;

struct Day01 {
    n1: Vec<i32>,
    n2: Vec<i32>,
}

impl Problem<i32, i32> for Day01 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, separated_pair(parse_i32, space1, parse_i32))
            .map(|n| {
                let (n1, n2) = n.iter().cloned().unzip();
                Day01 { n1, n2 }
            })
            .parse(input)
    }

    fn part1(mut self) -> Result<i32> {
        self.n1.sort();
        self.n2.sort();
        Ok(zip(self.n1, self.n2).map(|(a, b)| (a - b).abs()).sum())
    }

    fn part2(self) -> Result<i32> {
        Ok(self
            .n1
            .iter()
            .map(|a| a * self.n2.iter().filter(|&b| *a == *b).count() as i32)
            .sum())
    }
}

aoc_main!(Day01);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3
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
