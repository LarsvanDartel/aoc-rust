use aoc_rust::*;

struct Day20 {
    n: usize,
}

impl Problem<usize, usize> for Day20 {
    fn parse(input: &str) -> ParseResult<Self> {
        let n = input.trim().parse().unwrap();
        Ok(("", Day20 { n }))
    }

    fn part1(self) -> Result<usize> {
        let mut houses = vec![0; self.n / 10];
        for i in 1..self.n / 10 {
            for j in (i..self.n / 10).step_by(i) {
                houses[j] += i * 10;
            }
        }
        for (i, h) in houses.iter().enumerate() {
            if *h >= self.n {
                return Ok(i);
            }
        }
        unreachable!()
    }

    fn part2(self) -> Result<usize> {
        let mut houses = vec![0; self.n / 10];
        for i in 1..self.n / 10 {
            for j in (i..self.n / 10).step_by(i).take(50) {
                houses[j] += i * 11;
            }
        }
        for (i, h) in houses.iter().enumerate() {
            if *h >= self.n {
                return Ok(i);
            }
        }
        unreachable!()
    }
}

aoc_main!(Day20);
