use aoc_rust::*;
use common::*;

struct Day02 {
    presents: Vec<Present>,
}

struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn parse(input: &mut &str) -> PResult<Self> {
        let l = dec_uint.parse_next(input)?;
        let _ = 'x'.parse_next(input)?;
        let w = dec_uint.parse_next(input)?;
        let _ = 'x'.parse_next(input)?;
        let h = dec_uint.parse_next(input)?;
        Ok(Self { l, w, h })
    }
}

impl Problem<u32, u32> for Day02 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(0.., Present::parse, line_ending)
            .map(|presents| Self { presents })
            .parse_next(input)
    }

    fn part1(self) -> Result<u32> {
        Ok(self
            .presents
            .iter()
            .map(|p| {
                let mut sides = [p.l * p.w, p.w * p.h, p.h * p.l];
                sides.sort();
                3 * sides[0] + 2 * sides[1] + 2 * sides[2]
            })
            .sum())
    }

    fn part2(self) -> Result<u32> {
        Ok(self
            .presents
            .iter()
            .map(|p| {
                let mut sides = [p.l, p.w, p.h];
                sides.sort();
                2 * sides[0] + 2 * sides[1] + sides[0] * sides[1] * sides[2]
            })
            .sum())
    }
}

aoc_main!(Day02);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_task!(Day02, 1, "2x3x4", 58);
        assert_task!(Day02, 1, "1x1x10", 43);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day02, 2, "2x3x4", 34);
        assert_task!(Day02, 2, "1x1x10", 14);
    }
}
