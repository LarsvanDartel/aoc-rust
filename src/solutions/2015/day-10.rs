use aoc_rust::*;

struct Day10 {
    seq: String,
}

impl Day10 {
    fn look_and_say(&self, n: usize) -> String {
        let mut seq = self.seq.clone();
        for _ in 0..n {
            let mut new_seq = String::new();
            let mut chars = seq.chars();
            let mut prev = chars.next().unwrap();
            let mut count = 1;
            for c in chars {
                if c == prev {
                    count += 1;
                } else {
                    new_seq.push_str(&format!("{}{}", count, prev));
                    prev = c;
                    count = 1;
                }
            }
            new_seq.push_str(&format!("{}{}", count, prev));
            seq = new_seq;
        }
        seq
    }
}

impl Problem<usize, usize> for Day10 {
    fn parse(input: &str) -> ParseResult<Self> {
        Ok((
            "",
            Self {
                seq: input.trim().to_string(),
            },
        ))
    }

    fn part1(self) -> Result<usize> {
        Ok(self.look_and_say(40).len())
    }

    fn part2(self) -> Result<usize> {
        Ok(self.look_and_say(50).len())
    }
}

aoc_main!(Day10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day10 {
                seq: "1".to_string()
            }
            .look_and_say(1),
            "11"
        );
        assert_eq!(
            Day10 {
                seq: "1".to_string()
            }
            .look_and_say(2),
            "21"
        );
        assert_eq!(
            Day10 {
                seq: "1".to_string()
            }
            .look_and_say(3),
            "1211"
        );
        assert_eq!(
            Day10 {
                seq: "1".to_string()
            }
            .look_and_say(4),
            "111221"
        );
        assert_eq!(
            Day10 {
                seq: "1".to_string()
            }
            .look_and_say(5),
            "312211"
        );
    }
}
