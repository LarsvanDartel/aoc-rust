use std::cmp::Reverse;

use aoc_rust::*;
use common::*;

type Point = Vec2<i64>;
type Rect = (Point, Point);

struct Day09 {
    points: Vec<Point>,
}

impl Day09 {
    fn area((a, b): Rect) -> u64 {
        (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
    }

    fn is_valid(&self, rect: Rect) -> bool {
        let (min_x, max_x) = (rect.0.x.min(rect.1.x), rect.0.x.max(rect.1.x));
        let (min_y, max_y) = (rect.0.y.min(rect.1.y), rect.0.y.max(rect.1.y));

        !self.points.iter().circular_tuple_windows().any(|(a, b)| {
            // check if both x and y ranges intersect, i.e. the edge intersects the
            // interior of the rectangle
            (a.x > min_x || b.x > min_x)
                && (a.x < max_x || b.x < max_x)
                && (a.y > min_y || b.y > min_y)
                && (a.y < max_y || b.y < max_y)
        })
    }
}

impl Problem<u64, u64> for Day09 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Ok(Day09 {
            points: list(seq!(dec_int, _: ',', dec_int).map(Vec2::from), line_ending)
                .parse_next(input)?,
        })
    }

    fn part1(self) -> Result<u64> {
        Ok(self
            .points
            .into_iter()
            .tuple_combinations()
            .map(Day09::area)
            .max()
            .unwrap())
    }

    fn part2(self) -> Result<u64> {
        self.points
            .iter()
            .cloned()
            .tuple_combinations()
            .sorted_by_key(|x| Reverse(Day09::area(*x)))
            .find(|r| self.is_valid(*r))
            .map(Day09::area)
            .ok_or(AoCError::NoSolution)
    }
}

aoc_main!(Day09);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day09, 1, EXAMPLE, 50);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day09, 2, EXAMPLE, 24);
    }
}
