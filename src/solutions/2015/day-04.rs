use aoc_rust::*;
use common::*;

struct Day04 {
    key: String,
}

impl Problem<usize, usize> for Day04 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Ok(Self {
            key: input.trim().to_string(),
        })
    }

    fn part1(self) -> Result<usize> {
        let mut md5 = MD5::init(self.key);
        Ok(md5.next_key_zeroes(5))
    }

    fn part2(self) -> Result<usize> {
        let mut md5 = MD5::init(self.key);
        Ok(md5.next_key_zeroes(6))
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
