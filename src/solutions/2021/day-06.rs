use aoc_rust::*;
use common::*;

struct Day06 {
    fish: [usize; 9],
}

impl Problem<usize, usize> for Day06 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(0.., dec_uint::<_, usize, _>, ',')
            .map(|v: Vec<usize>| {
                let mut fish = [0; 9];
                for i in v {
                    fish[i] += 1;
                }
                Self { fish }
            })
            .parse_next(input)
    }

    fn part1(mut self) -> Result<usize> {
        for _ in 0..80 {
            let newborn = self.fish[0];
            for i in 0..8 {
                self.fish[i] = self.fish[i + 1];
            }
            self.fish[6] += newborn;
            self.fish[8] = newborn;
        }

        Ok(self.fish.iter().sum())
    }

    fn part2(mut self) -> Result<usize> {
        for _ in 0..256 {
            let newborn = self.fish[0];
            for i in 0..8 {
                self.fish[i] = self.fish[i + 1];
            }
            self.fish[6] += newborn;
            self.fish[8] = newborn;
        }

        Ok(self.fish.iter().sum())
    }
}

aoc_main!(Day06);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"3,4,3,1,2"#;

    #[test]
    fn test_part1() {
        assert_task!(Day06, 1, EXAMPLE, 5934);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day06, 2, EXAMPLE, 26984457539usize);
    }
}
