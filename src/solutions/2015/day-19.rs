use std::collections::HashSet;

use aoc_rust::*;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    Parser,
};

struct Day19 {
    replacements: Vec<(String, String)>,
    molecule: String,
}

impl Problem<usize, usize> for Day19 {
    fn parse(input: &str) -> ParseResult<Self> {
        let (input, replacements) = separated_list1(
            line_ending,
            separated_pair(
                alpha1.map(String::from),
                tag(" => "),
                alpha1.map(String::from),
            ),
        )
        .parse(input)?;

        let (input, _) = line_ending(input)?;
        let (input, _) = line_ending(input)?;

        let (input, molecule) = alpha1.map(String::from).parse(input)?;

        Ok((
            input,
            Self {
                replacements,
                molecule,
            },
        ))
    }

    fn part1(self) -> Result<usize> {
        let mut seen = HashSet::new();
        for (from, to) in self.replacements {
            for i in 0..self.molecule.len() {
                if self.molecule[i..].starts_with(&from) {
                    let mut new = self.molecule.clone();
                    new.replace_range(i..i + from.len(), &to);
                    seen.insert(new);
                }
            }
        }

        Ok(seen.len())
    }

    fn part2(self) -> Result<usize> {
        let elements = self.molecule.chars().filter(|c| c.is_uppercase()).count();
        let rnas = self.molecule.matches("Rn").count() + self.molecule.matches("Ar").count();
        let y = self.molecule.matches('Y').count();
        Ok(elements - rnas - 2 * y - 1)
    }
}

aoc_main!(Day19);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"H => HO
H => OH
O => HH

HOH"#;

    #[test]
    fn test_part1() {
        assert_task!(Day19, 1, EXAMPLE, 4);
    }
}
