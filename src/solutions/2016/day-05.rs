use aoc_rust::*;
use common::*;

struct Day05 {
    door_id: String,
}

impl Problem<String, String> for Day05 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Ok(Day05 {
            door_id: input.trim().to_string(),
        })
    }

    fn part1(self) -> Result<String> {
        let mut md5 = MD5::init(self.door_id);
        let mut password = String::new();
        while password.len() < 8 {
            md5.next_key_zeroes(5);
            password.push(
                char::from_digit(MD5::get_hex_digit(md5.hash(), 6), 16).ok_or("Invalid digit")?,
            );
        }
        Ok(password)
    }

    fn part2(self) -> Result<String> {
        let mut md5 = MD5::init(self.door_id);
        let mut password = vec!['_'; 8];
        while password.contains(&'_') {
            md5.next_key_zeroes(5);
            let position = MD5::get_hex_digit(md5.hash(), 6) as usize;
            if position < 8 && password[position] == '_' {
                password[position] = char::from_digit(MD5::get_hex_digit(md5.hash(), 7), 16)
                    .ok_or("Invalid digit")?;
            }
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
