use aoc_rust::*;
use common::*;

#[derive(Debug)]
struct Display {
    input: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

impl Display {
    fn parse(input: &mut &str) -> PResult<Self> {
        fn charset(input: &mut &str) -> PResult<HashSet<char>> {
            alpha1.map(|s: &str| s.chars().collect()).parse_next(input)
        }
        separated_pair(list(charset, space1), " | ", list(charset, space1))
            .map(|(input, output)| Self { input, output })
            .parse_next(input)
    }

    fn decode(&self) -> usize {
        let one = self.input.iter().find(|i| i.len() == 2).unwrap();
        let seven = self.input.iter().find(|i| i.len() == 3).unwrap();
        let four = self.input.iter().find(|i| i.len() == 4).unwrap();
        let eight = self.input.iter().find(|i| i.len() == 7).unwrap();
        let three = self.input.iter().find(|i| (*i - one).len() == 3).unwrap();
        let nine = self
            .input
            .iter()
            .find(|i| i.len() == 6 && (*i - three).len() == 1)
            .unwrap();
        let six = self
            .input
            .iter()
            .find(|i| i.len() == 6 && (*i - one).len() == 5)
            .unwrap();
        let zero = self
            .input
            .iter()
            .find(|i| i.len() == 6 && (&(three - *i) - one).len() == 1)
            .unwrap();
        let five = self
            .input
            .iter()
            .find(|i| i.len() == 5 && (*i - six).is_empty())
            .unwrap();
        let two = self
            .input
            .iter()
            .find(|i| i.len() == 5 && (*i - five).len() == 2)
            .unwrap();

        let digits = [zero, one, two, three, four, five, six, seven, eight, nine];

        let output = self
            .output
            .iter()
            .map(|o| digits.iter().position(|&d| d == o).unwrap());
        output.fold(0, |acc, x| acc * 10 + x)
    }
}

struct Day08 {
    displays: Vec<Display>,
}

impl Problem<usize, usize> for Day08 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(Display::parse, line_ending)
            .map(|displays| Self { displays })
            .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self
            .displays
            .into_iter()
            .map(|d| {
                d.output
                    .into_iter()
                    .filter(|o| o.len() == 2 || o.len() == 3 || o.len() == 7 || o.len() == 4)
                    .count()
            })
            .sum())
    }

    fn part2(self) -> Result<usize> {
        Ok(self.displays.into_iter().map(|d| d.decode()).sum())
    }
}

aoc_main!(Day08);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day08, 1, EXAMPLE, 26);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day08, 2, EXAMPLE, 61229);
    }
}
