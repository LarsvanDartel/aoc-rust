use aoc_rust::*;
use common::*;

struct Day19 {
    towels: Vec<Vec<char>>,
    designs: Vec<Vec<char>>,
}

impl Day19 {
    fn num_arrangements(&self, design: &[char]) -> usize {
        let mut possible = vec![0; design.len() + 1];
        possible[0] = 1;
        for i in 0..design.len() {
            if possible[i] > 0 {
                for towel in &self.towels {
                    if towel
                        .iter()
                        .enumerate()
                        .all(|(j, ch)| i + j < design.len() && *ch == design[i + j])
                    {
                        possible[i + towel.len()] += possible[i];
                    }
                }
            }
        }

        possible[design.len()]
    }
}

impl Problem<usize, usize> for Day19 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated_pair(
            list(alpha1.map(|s: &str| s.chars().collect()), ", "),
            (line_ending, line_ending),
            list(alpha1.map(|s: &str| s.chars().collect()), line_ending),
        )
        .map(|(towels, designs)| Day19 { towels, designs })
        .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self
            .designs
            .iter()
            .filter(|&design| self.num_arrangements(design) > 0)
            .count())
    }

    fn part2(self) -> Result<usize> {
        Ok(self
            .designs
            .iter()
            .map(|design| self.num_arrangements(design))
            .sum())
    }
}

aoc_main!(Day19);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day19, 1, EXAMPLE, 6);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day19, 2, EXAMPLE, 16);
    }
}
