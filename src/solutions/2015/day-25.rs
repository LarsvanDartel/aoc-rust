use aoc_rust::*;
use common::*;

struct Day25 {
    row: u32,
    col: u32,
}

impl Problem<u64, String> for Day25 {
    fn parse(input: &mut &str) -> PResult<Self> {
        let _ = "To continue, please consult the code grid in the manual.  Enter the code at row "
            .parse_next(input)?;
        let row = dec_uint(input)?;
        let _ = ", column ".parse_next(input)?;
        let col = dec_uint(input)?;
        Ok(Self { row, col })
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const EXAMPLE: &str = r#"
//     "#;
//
//     #[test]
//     fn test_part1() {
//         assert_task!(Day25, 1, EXAMPLE, ());
//     }
//
//     #[test]
//     fn test_part2() {
//         assert_task!(Day25, 2, EXAMPLE, ());
//     }
// }
