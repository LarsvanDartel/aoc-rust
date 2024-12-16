use aoc_rust::*;
use common::*;

struct Day11 {
    stones: HashMap<u64, usize>,
}

impl Day11 {
    fn update(&mut self) {
        let mut new_map = HashMap::new();
        for (stone, count) in self.stones.clone() {
            if stone == 0 {
                *new_map.entry(1).or_default() += count;
                continue;
            }
            let mut s = stone;
            let mut l = 0;
            let mut p = 1;
            while s > 0 {
                if l % 2 == 0 {
                    p *= 10;
                }
                s /= 10;
                l += 1;
            }

            if l % 2 == 0 {
                *new_map.entry(stone / p).or_default() += count;
                *new_map.entry(stone % p).or_default() += count;
            } else {
                *new_map.entry(stone * 2024).or_default() += count;
            }
        }
        self.stones = new_map;
    }
}

impl Problem<usize, usize> for Day11 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(dec_u64, space1)
            .map(|stones| {
                let mut map = HashMap::new();
                for s in stones {
                    *map.entry(s).or_default() += 1;
                }
                Day11 { stones: map }
            })
            .parse_next(input)
    }

    fn part1(mut self) -> Result<usize> {
        for _ in 0..25 {
            self.update();
        }
        Ok(self.stones.values().sum())
    }

    fn part2(mut self) -> Result<usize> {
        for _ in 0..75 {
            self.update();
        }
        Ok(self.stones.values().sum())
    }
}

aoc_main!(Day11);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"125 17
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day11, 1, EXAMPLE, 55312);
    }
}
