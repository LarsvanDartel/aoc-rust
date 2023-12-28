use aoc_rust::*;

extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

struct Day04 {
    key: String,
}

impl Day04 {
    fn starts_with_zeroes(&self, zeroes: usize) -> u32 {
        let mut hasher = Md5::new();
        let mut i = 0;
        loop {
            hasher.input_str(&format!("{}{}", self.key, i));
            let hash = hasher.result_str();
            hasher.reset();
            if hash.starts_with(&"0".repeat(zeroes)) {
                return i;
            }
            i += 1;
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
