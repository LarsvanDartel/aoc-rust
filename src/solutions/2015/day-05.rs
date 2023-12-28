use aoc_rust::*;

const VOWELS: &str = "aeiou";
const BAD_STRINGS: [(char, char); 4] = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];

struct Day05 {
    strings: Vec<String>,
}

impl Day05 {
    fn is_nice_1(s: &str) -> bool {
        let mut vowels = 0;
        let mut last_char = ' ';
        let mut has_double = false;
        for c in s.chars() {
            if VOWELS.contains(c) {
                vowels += 1;
            }
            if c == last_char {
                has_double = true;
            }
            if BAD_STRINGS.contains(&(last_char, c)) {
                return false;
            }
            last_char = c;
        }
        vowels >= 3 && has_double
    }

    fn is_nice_2(s: &str) -> bool {
        let mut has_pair = false;
        for i in 0..s.len() - 1 {
            let pair = &s[i..i + 2];
            if s[i + 2..].contains(pair) {
                has_pair = true;
                break;
            }
        }

        let has_repeat = s.chars().zip(s.chars().skip(2)).any(|(a, b)| a == b);

        has_pair && has_repeat
    }
}

impl Problem<usize, usize> for Day05 {
    fn parse(input: &str) -> ParseResult<Self> {
        Ok((
            "",
            Self {
                strings: input.lines().map(|s| s.to_string()).collect(),
            },
        ))
    }

    fn part1(self) -> Result<usize> {
        Ok(self.strings.iter().filter(|s| Self::is_nice_1(s)).count())
    }

    fn part2(self) -> Result<usize> {
        Ok(self.strings.iter().filter(|s| Self::is_nice_2(s)).count())
    }
}

aoc_main!(Day05);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_task!(Day05, 1, "ugknbfddgicrmopn", 1);
        assert_task!(Day05, 1, "aaa", 1);
        assert_task!(Day05, 1, "jchzalrnumimnmhp", 0);
        assert_task!(Day05, 1, "haegwjzuvuyypxyu", 0);
        assert_task!(Day05, 1, "dvszwmarrgswjxmb", 0);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day05, 2, "qjhvhtzxzqqjkmpb", 1);
        assert_task!(Day05, 2, "xxyxx", 1);
        assert_task!(Day05, 2, "uurcxstgmygtbstg", 0);
    }
}
