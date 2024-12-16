use aoc_rust::*;
use common::*;

struct Day01 {
    numbers: Vec<u32>,
}

impl Problem<u32, u32> for Day01 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(0.., dec_uint::<_, u32, _>, line_ending)
            .map(|numbers| Day01 { numbers })
            .parse_next(input)
    }

    fn part1(self) -> Result<u32> {
        let res = self
            .numbers
            .iter()
            .tuple_combinations()
            .find_map(|(a, b)| if a + b == 2020 { Some(a * b) } else { None })
            .ok_or("No soltution found")?;

        Ok(res)
    }

    fn part2(self) -> Result<u32> {
        let res = self
            .numbers
            .iter()
            .tuple_combinations()
            .find_map(|(a, b, c)| {
                if a + b + c == 2020 {
                    Some(a * b * c)
                } else {
                    None
                }
            })
            .ok_or("No soltution found")?;

        Ok(res)
    }
}

aoc_main!(Day01);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"1721
979
366
299
675
1456
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day01, 1, EXAMPLE, 514579);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day01, 2, EXAMPLE, 241861950);
    }
}
