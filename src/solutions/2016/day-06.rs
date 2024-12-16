use aoc_rust::*;
use common::*;

struct Day06 {
    messages: Vec<String>,
}

impl Problem<String, String> for Day06 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Ok(Day06 {
            messages: input.lines().map(|l| l.trim().to_string()).collect(),
        })
    }

    fn part1(self) -> Result<String> {
        let mut counts = vec![vec![0; 26]; self.messages[0].len()];
        for message in self.messages {
            for (i, c) in message.chars().enumerate() {
                counts[i][c as usize - 'a' as usize] += 1;
            }
        }

        let mut result = String::new();
        for count in counts {
            let max = count.iter().max().unwrap();
            let idx = count.iter().position(|&x| x == *max).unwrap();
            result.push((b'a' + idx as u8) as char);
        }

        Ok(result)
    }

    fn part2(self) -> Result<String> {
        let mut counts = vec![vec![0; 26]; self.messages[0].len()];
        for message in self.messages {
            for (i, c) in message.chars().enumerate() {
                counts[i][c as usize - 'a' as usize] += 1;
            }
        }

        let mut result = String::new();
        for count in counts {
            let min = count.iter().filter(|&&x| x > 0).min().unwrap();
            let idx = count.iter().position(|&x| x == *min).unwrap();
            result.push((b'a' + idx as u8) as char);
        }

        Ok(result)
    }
}

aoc_main!(Day06);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"#;

    #[test]
    fn test_part1() {
        assert_task!(Day06, 1, EXAMPLE, "easter");
    }

    #[test]
    fn test_part2() {
        assert_task!(Day06, 2, EXAMPLE, "advent");
    }
}
