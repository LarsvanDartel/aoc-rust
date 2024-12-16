use aoc_rust::*;
use common::*;

struct Day19 {
    replacements: Vec<(String, String)>,
    molecule: String,
}

impl Problem<usize, usize> for Day19 {
    fn parse(input: &mut &str) -> PResult<Self> {
        let replacements = separated(
            0..,
            separated_pair(alpha1.map(String::from), " => ", alpha1.map(String::from)),
            line_ending,
        )
        .parse_next(input)?;

        let _ = line_ending(input)?;
        let _ = line_ending(input)?;

        let molecule = alpha1.map(String::from).parse_next(input)?;

        Ok(Self {
            replacements,
            molecule,
        })
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
