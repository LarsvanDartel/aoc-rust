use aoc_rust::common::MD5;
use aoc_rust::*;

struct Day04 {
    key: String,
}

impl Day04 {
    fn starts_with_zeroes(&self, zeroes: usize) -> u32 {
        let mut index = 0;
        loop {
            let hash = MD5::hash(&format!("{}{}", self.key, index));
            if hash < (1 << (128 - zeroes * 4)) {
                return index;
            }
            index += 1;
        }
    }
}

impl Problem<u32, u32> for Day04 {
    fn parse(input: &str) -> ParseResult<Self> {
        Ok((
            "",
            Self {
                key: input.trim().to_string(),
            },
        ))
    }

    fn part1(self) -> Result<u32> {
        Ok(self.starts_with_zeroes(5))
    }

    fn part2(self) -> Result<u32> {
        Ok(self.starts_with_zeroes(6))
    }
}

aoc_main!(Day04);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_task!(Day04, 1, "abcdef", 609043);
        assert_task!(Day04, 1, "pqrstuv", 1048970);
    }
}
