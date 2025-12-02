use aoc_rust::*;
use common::*;

struct Day08 {
    strings: Vec<String>,
}

impl Problem<usize, usize> for Day08 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Ok(Self {
            strings: input.lines().map(|line| line.trim().to_string()).collect(),
        })
    }

    fn part1(self) -> Result<usize> {
        Ok(self
            .strings
            .iter()
            .map(|s| {
                let mut chars = s.chars();
                let mut count = 0;
                while let Some(c) = chars.next() {
                    if c == '\\' {
                        match chars.next() {
                            Some('x') => {
                                chars.next();
                                chars.next();
                            }
                            Some('\\') => {}
                            Some('"') => {}
                            None => break,
                            _ => unreachable!(),
                        }
                    }
                    count += 1;
                }
                s.len() - count + 2
            })
            .sum())
    }

    fn part2(self) -> Result<usize> {
        Ok(self
            .strings
            .iter()
            .map(|s| {
                let mut count = 0;
                for c in s.chars() {
                    if c == '\\' || c == '"' {
                        count += 1;
                    }
                }
                count + 2
            })
            .sum())
    }
}

aoc_main!(Day08);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

    #[test]
    fn test_part1() {
        assert_task!(Day08, 1, EXAMPLE, 12);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day08, 2, EXAMPLE, 19);
    }
}
