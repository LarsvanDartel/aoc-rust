use aoc_rust::*;
use nom::{bytes::complete::tag, character::complete::u32 as number};

struct Day25 {
    row: u32,
    col: u32,
}

impl Problem<u64, String> for Day25 {
    fn parse(input: &str) -> ParseResult<Self> {
        let (input, _) = tag(
            "To continue, please consult the code grid in the manual.  Enter the code at row ",
        )(input)?;
        let (input, row) = number(input)?;
        let (input, _) = tag(", column ")(input)?;
        let (input, col) = number(input)?;
        let (input, _) = tag(".")(input)?;
        Ok((input, Self { row, col }))
    }

    fn part1(self) -> Result<u64> {
        let mut code = 20151125u64;
        let mut row = 1;
        let mut col = 1;
        while row != self.row || col != self.col {
            code = (code * 252533) % 33554393;
            if row == 1 {
                row = col + 1;
                col = 1;
            } else {
                row -= 1;
                col += 1;
            }
        }
        Ok(code)
    }

    fn part2(self) -> Result<String> {
        Ok("Merry Christmas!".to_string())
    }
}

aoc_main!(Day25);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
    "#;

    #[test]
    fn test_part1() {
        assert_task!(Day25, 1, EXAMPLE, ());
    }

    #[test]
    fn test_part2() {
        assert_task!(Day25, 2, EXAMPLE, ());
    }
}
