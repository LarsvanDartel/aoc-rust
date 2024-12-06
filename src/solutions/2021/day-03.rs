use aoc_rust::*;
use common::*;

struct Day03 {
    report: Grid<bool>,
}

impl Problem<u32, u32> for Day03 {
    fn parse(input: &str) -> ParseResult<Self> {
        Grid::parse(one_of("01").map(|c| c == '1'))
            .map(|report| Self { report })
            .parse(input)
    }

    fn part1(self) -> Result<u32> {
        let (gamma, eps) = (0..self.report.width)
            .map(|c| {
                let col = self.report.get_column(c).unwrap();
                col.iter().filter(|&&b| *b).count() > col.iter().filter(|&&b| !b).count()
            })
            .fold((0, 0), |acc, b| {
                ((acc.0 << 1) | b as u32, (acc.1 << 1) | !b as u32)
            });

        Ok(gamma * eps)
    }

    fn part2(self) -> Result<u32> {
        let numbers = (0..self.report.height)
            .map(|r| self.report.get_row(r).unwrap())
            .collect::<Vec<_>>();

        let mut n = numbers.clone();
        for i in 0..self.report.width {
            let col = (0..n.len()).map(|r| n[r][i]).collect::<Vec<_>>();
            if col.iter().filter(|&&b| b).count() >= col.iter().filter(|&&b| !b).count() {
                n.retain(|&r| r[i]);
            } else {
                n.retain(|&r| !r[i]);
            }
        }
        let oxy = n[0].iter().fold(0, |acc, &b| (acc << 1) | b as u32);

        let mut n = numbers.clone();
        for i in 0..self.report.width {
            let col = (0..n.len()).map(|r| n[r][i]).collect::<Vec<_>>();
            if col.iter().filter(|&&b| b).count() >= col.iter().filter(|&&b| !b).count() {
                n.retain(|&r| !r[i]);
            } else {
                n.retain(|&r| r[i]);
            }
            if n.len() == 1 {
                break;
            }
        }
        let co2 = n[0].iter().fold(0, |acc, &b| (acc << 1) | b as u32);

        Ok(oxy * co2)
    }
}

aoc_main!(Day03);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day03, 1, EXAMPLE, 198);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day03, 2, EXAMPLE, 230);
    }
}
