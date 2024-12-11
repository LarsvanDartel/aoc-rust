use aoc_rust::*;
use common::*;

struct Day11 {
    stones: HashMap<String, usize>,
}

impl Day11 {
    fn update(&mut self) {
        let mut new_map = HashMap::new();
        for (stone, count) in self.stones.clone() {
            if stone == "0" || stone.is_empty() {
                *new_map.entry("1".to_string()).or_default() += count;
                continue;
            } else if stone.len() % 2 == 0 {
                let mid = stone.len() / 2;
                let (left, mut right) = stone.split_at(mid);
                *new_map.entry(left.to_string()).or_default() += count;
                right = right.trim_start_matches('0');
                *new_map.entry(right.to_string()).or_default() += count;
            } else {
                // Multiply by 2024
                *new_map
                    .entry((stone.parse::<usize>().unwrap() * 2024).to_string())
                    .or_default() += count;
            }
        }
        self.stones = new_map;
    }
}

impl Problem<usize, usize> for Day11 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(space1, digit1)
            .map(|stones: Vec<&str>| {
                let mut map = HashMap::new();
                for s in stones {
                    *map.entry(s.to_string()).or_default() += 1;
                }
                Day11 { stones: map }
            })
            .parse(input)
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

    #[test]
    fn test_part2() {
        assert_task!(Day11, 2, EXAMPLE, ());
    }
}
