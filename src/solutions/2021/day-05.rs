use aoc_rust::*;
use common::*;
use hashbrown::HashSet;
use num_integer::gcd;

struct Line {
    start: Vec2<i32>,
    end: Vec2<i32>,
}

impl Line {
    fn parse(s: &mut &str) -> PResult<Self> {
        fn parse_vec2(s: &mut &str) -> PResult<Vec2<i32>> {
            separated_pair(dec_int, ',', dec_int)
                .map(Into::into)
                .parse_next(s)
        }

        separated_pair(parse_vec2, " -> ", parse_vec2)
            .map(|(start, end)| Line { start, end })
            .parse_next(s)
    }

    fn is_orthogonal(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn points(&self) -> impl Iterator<Item = Vec2<i32>> + '_ {
        let d = self.end - self.start;
        let n = gcd(d.x.abs(), d.y.abs());
        let d = d / n;
        (0..=n).map(move |i| self.start + d * i)
    }
}

struct Day05 {
    lines: Vec<Line>,
}

impl Problem<usize, usize> for Day05 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(0.., Line::parse, line_ending)
            .map(|lines| Day05 { lines })
            .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        let mut intersections = HashSet::<Vec2<i32>>::new();
        let n = self.lines.len();
        for i in 0..n {
            if !self.lines[i].is_orthogonal() {
                continue;
            }
            for j in (i + 1)..n {
                if !self.lines[j].is_orthogonal() {
                    continue;
                }
                let a = &self.lines[i];
                let b = &self.lines[j];

                let p = a.points().collect::<HashSet<_>>();
                let q = b.points().collect::<HashSet<_>>();

                intersections.extend(p.intersection(&q));
            }
        }

        Ok(intersections.len())
    }

    fn part2(self) -> Result<usize> {
        let mut intersections = HashSet::<Vec2<i32>>::new();
        let n = self.lines.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let a = &self.lines[i];
                let b = &self.lines[j];

                let p = a.points().collect::<HashSet<_>>();
                let q = b.points().collect::<HashSet<_>>();

                intersections.extend(p.intersection(&q));
            }
        }

        Ok(intersections.len())
    }
}

aoc_main!(Day05);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day05, 1, EXAMPLE, 5);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day05, 2, EXAMPLE, 12);
    }
}
