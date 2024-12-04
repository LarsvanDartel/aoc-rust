use aoc_rust::*;
use common::*;

struct Day04 {
    grid: Grid<u8>,
}

impl Problem<usize, usize> for Day04 {
    fn parse(input: &str) -> ParseResult<Self> {
        Grid::parse(one_of("XMAS").map(|c| c as u8))
            .map(|grid| Self { grid })
            .parse(input)
    }

    fn part1(self) -> Result<usize> {
        let target = "XMAS".as_bytes();
        Ok((self.grid.coordinates().map(|c| {
            Direction::all()
                .filter(|&d| {
                    target
                        .iter()
                        .enumerate()
                        .all(|(i, ch)| self.grid.get(c + (d * i as isize)) == Some(ch))
                })
                .count()
        }))
        .sum())
    }

    fn part2(self) -> Result<usize> {
        Ok(self
            .grid
            .coordinates()
            .filter(|c| self.grid.get(*c) == Some(&b'A'))
            .filter(|c| {
                let corners = Direction::ordinal()
                    .map(|d| self.grid.get(*c + d).unwrap_or(&b' '))
                    .collect::<Vec<_>>();

                let d1 = [*corners[0], *corners[2]];
                let d2 = [*corners[1], *corners[3]];

                (&d1 == b"MS" || &d1 == b"SM") && (&d2 == b"MS" || &d2 == b"SM")
            })
            .count())
    }
}

aoc_main!(Day04);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test_part1() {
        assert_task!(Day04, 1, EXAMPLE, 18);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day04, 2, EXAMPLE, 9);
    }
}
