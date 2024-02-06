use aoc_rust::common::MD5;
use aoc_rust::*;
struct Day05 {
    door_id: String,
}

fn starts_with_zeroes(hash: u128, zeroes: usize) -> bool {
    hash < (1 << (128 - zeroes * 4))
}

impl Problem<String, String> for Day05 {
    fn parse(input: &str) -> ParseResult<Self> {
        Ok((
            "",
            Day05 {
                door_id: input.trim().to_string(),
            },
        ))
    }

    fn part1(self) -> Result<String> {
        let mut password = String::new();
        let mut index = 0;
        while password.len() < 8 {
            let hash = MD5::hash(&format!("{}{}", self.door_id, index));
            if starts_with_zeroes(hash, 5) {
                password.push(std::char::from_digit((hash >> 104) as u32, 16).unwrap());
            }
            index += 1;
        }
        Ok(password)
    }

    fn part2(self) -> Result<String> {
        let mut password = vec!['_'; 8];
        let mut index = 0;
        while password.iter().any(|&c| c == '_') {
            let hash = MD5::hash(&format!("{}{}", self.door_id, index));
            if starts_with_zeroes(hash, 5) {
                let position = (hash >> 104) as usize;
                if position < 8 && password[position] == '_' {
                    password[position] =
                        std::char::from_digit(((hash >> 100) & 0xf) as u32, 16).unwrap();
                }
            }
            index += 1;
        }
        Ok(password.into_iter().collect())
    }
}

aoc_main!(Day05);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"abc"#;

    #[test]
    fn test_part1() {
        assert_task!(Day05, 1, EXAMPLE, "18f47a30");
    }

    #[test]
    fn test_part2() {
        assert_task!(Day05, 2, EXAMPLE, "05ace8e3");
    }
}
