use aoc_rust::*;
use common::*;

struct Day11 {
    password: String,
}

impl Day11 {
    fn is_valid(&self) -> bool {
        let has_straight = self
            .password
            .as_bytes()
            .windows(3)
            .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]);
        let has_iol = self
            .password
            .chars()
            .any(|c| c == 'i' || c == 'o' || c == 'l');
        let has_two_pairs = self
            .password
            .as_bytes()
            .windows(2)
            .filter(|w| w[0] == w[1])
            .map(|w| w[0])
            .collect::<HashSet<_>>()
            .len()
            >= 2;

        has_straight && !has_iol && has_two_pairs
    }

    fn next_password(&mut self) {
        let mut chars = self.password.chars().rev();
        let mut next = String::new();
        let mut carry = true;
        while carry {
            match chars.next() {
                Some('z') => next.push('a'),
                Some(c) => {
                    next.push((c as u8 + 1) as char);
                    carry = false;
                },
                None => next.push('a'),
            }
        }
        for c in chars {
            next.push(c);
        }
        self.password = next.chars().rev().collect();
    }

    fn next_valid_password(&mut self) {
        self.next_password();
        while !self.is_valid() {
            self.next_password();
        }
    }
}

impl Problem<String, String> for Day11 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Ok(Self {
            password: input.trim().to_string(),
        })
    }

    fn part1(mut self) -> Result<String> {
        self.next_valid_password();
        Ok(self.password)
    }

    fn part2(mut self) -> Result<String> {
        self.next_valid_password();
        self.next_valid_password();
        Ok(self.password)
    }
}

aoc_main!(Day11);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut day11 = Day11 {
            password: "xx".to_string(),
        };
        day11.next_password();
        assert_eq!(day11.password, "xy".to_string());
        day11.next_password();
        assert_eq!(day11.password, "xz".to_string());
        day11.next_password();
        assert_eq!(day11.password, "ya".to_string());
        day11.next_password();
        assert_eq!(day11.password, "yb".to_string());
    }

    #[test]
    fn test_valid() {
        let day11 = Day11 {
            password: "hijklmmn".to_string(),
        };
        assert!(!day11.is_valid());
        let day11 = Day11 {
            password: "abbceffg".to_string(),
        };
        assert!(!day11.is_valid());
        let day11 = Day11 {
            password: "abbcegjk".to_string(),
        };
        assert!(!day11.is_valid());
        let day11 = Day11 {
            password: "abcdffaa".to_string(),
        };
        assert!(day11.is_valid());
    }
}
