use aoc_rust::*;
use common::*;

struct Day05 {
    boarding_passes: Vec<String>,
}

impl Problem<u32, u32> for Day05 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(0.., alpha1.map(String::from), line_ending)
            .map(|boarding_passes| Day05 { boarding_passes })
            .parse_next(input)
    }

    fn part1(self) -> Result<u32> {
        let mut max_id = 0;
        for pass in self.boarding_passes {
            let pass: u32 = pass.chars().fold(0, |acc, c| {
                acc << 1
                    | match c {
                        'F' | 'L' => 0,
                        'B' | 'R' => 1,
                        _ => unreachable!(),
                    }
            });

            max_id = max_id.max(pass);
        }

        Ok(max_id)
    }

    fn part2(self) -> Result<u32> {
        let mut ids = self
            .boarding_passes
            .iter()
            .map(|pass| {
                pass.chars().fold(0, |acc, c| {
                    acc << 1
                        | match c {
                            'F' | 'L' => 0,
                            'B' | 'R' => 1,
                            _ => unreachable!(),
                        }
                })
            })
            .collect::<Vec<u32>>();

        ids.sort_unstable();

        for i in 0..ids.len() - 1 {
            if ids[i + 1] - ids[i] == 2 {
                return Ok(ids[i] + 1);
            }
        }

        Err(AoCError::Message("Seat not found".to_string()))
    }
}

aoc_main!(Day05);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day05, 1, EXAMPLE, 820);
    }
}
