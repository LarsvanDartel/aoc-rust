use aoc_rust::*;
use common::*;

struct Day01 {
    rotations: Vec<i32>,
}

impl Day01 {
    fn apply_rotations(&self, mut pos: i32, part_one: bool) -> usize {
        let mut cnt = 0;
        for r in &self.rotations {
            if !part_one {
                if *r > 0 {
                    cnt += (pos + r) as usize / 100;
                } else {
                    cnt += ((100 - pos) % 100 - r) as usize / 100;
                }
            }
            pos = (pos + (r % 100) + 100) % 100;

            if part_one && pos == 0 {
                cnt += 1;
            }
        }

        cnt
    }
}

impl Problem<usize, usize> for Day01 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Ok(Day01 {
            rotations: list(
                (one_of(['L', 'R']), dec_u32).map(|(dir, len)| match dir {
                    'L' => -(len as i32),
                    'R' => len as i32,
                    _ => unreachable!(),
                }),
                line_ending,
            )
            .parse_next(input)?,
        })
    }

    fn part1(self) -> Result<usize> {
        Ok(self.apply_rotations(50, true))
    }

    fn part2(self) -> Result<usize> {
        Ok(self.apply_rotations(50, false))
    }
}

aoc_main!(Day01);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day01, 1, EXAMPLE, 3);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day01, 2, EXAMPLE, 6);
    }
}
