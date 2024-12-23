use std::iter;

use aoc_rust::*;
use common::*;

struct Day22 {
    numbers: Vec<u64>,
}

impl Day22 {
    const M: u64 = 16777216;
    fn step(&mut self) {
        for i in 0..self.numbers.len() {
            let mut x = self.numbers[i];
            x = (x ^ (x << 6)) % Self::M;
            x = (x ^ (x >> 5)) % Self::M;
            x = (x ^ (x << 11)) % Self::M;
            self.numbers[i] = x;
        }
    }

    fn prices_map(&self, i: usize, n: usize) -> HashMap<(i8, i8, i8, i8), u64> {
        let mut x = self.numbers[i];
        let prices = iter::from_fn(move || {
            let a = (x % 10) as i8;
            x = (x ^ (x << 6)) % Self::M;
            x = (x ^ (x >> 5)) % Self::M;
            x = (x ^ (x << 11)) % Self::M;
            Some((x % 10, (x % 10) as i8 - a))
        })
        .take(n)
        .tuple_windows()
        .map(|(pa, pb, pc, pd)| ((pa.1, pb.1, pc.1, pd.1), pd.0));
        let mut prices_map = HashMap::new();
        for (k, v) in prices {
            if !prices_map.contains_key(&k) {
                prices_map.insert(k, v);
            }
        }
        prices_map
    }
}

impl Problem<u64, u64> for Day22 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(dec_u64, line_ending)
            .map(|numbers| Self { numbers })
            .parse_next(input)
    }

    fn part1(mut self) -> Result<u64> {
        for _ in 0..2000 {
            self.step();
        }
        Ok(self.numbers.iter().sum())
    }

    fn part2(self) -> Result<u64> {
        let mut price_map = HashMap::new();
        for i in 0..self.numbers.len() {
            for (k, v) in self.prices_map(i, 2000) {
                *price_map.entry(k).or_insert(0) += v;
            }
        }

        Ok(price_map.into_values().max().unwrap())
    }
}

aoc_main!(Day22);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"1
10
100
2024
"#;
    const EXAMPLE2: &str = r#"1
2
3
2024
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day22, 1, EXAMPLE1, 37327623);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day22, 2, EXAMPLE2, 23);
    }
}
