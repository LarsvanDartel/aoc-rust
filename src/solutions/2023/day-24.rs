use aoc_rust::common::{Vec2, Vec3};
use aoc_rust::*;

use ndarray::prelude::*;
use ndarray_linalg::Solve;
use nom::{
    bytes::complete::tag,
    character::complete::{i64 as parse_i64, newline, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    Parser,
};

struct Day24 {
    hailstones: Vec<HailStone>,
}

#[derive(Clone, Copy)]
struct HailStone {
    pos: Vec3<i64>,
    vel: Vec3<i64>,
}

fn parse_vec3(input: &str) -> ParseResult<Vec3<i64>> {
    tuple((
        parse_i64,
        tag(",").and(space1),
        parse_i64,
        tag(",").and(space1),
        parse_i64,
    ))
    .map(|(x, _, y, _, z)| Vec3::new(x, y, z))
    .parse(input)
}

impl HailStone {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_pair(parse_vec3, delimited(space1, tag("@"), space1), parse_vec3)
            .map(|(pos, vel)| Self { pos, vel })
            .parse(input)
    }

    fn intersects(&self, other: &Self) -> Option<(f64, f64)> {
        let Vec2 { x: x1, y: y1 } = self.pos.xy();
        let Vec2 { x: x2, y: y2 } = other.pos.xy();
        let Vec2 { x: u1, y: v1 } = self.vel.xy();
        let Vec2 { x: u2, y: v2 } = other.vel.xy();

        // [ u1, -u2 ] [ a ] = [ x2 - x1 ]
        // [ v1, -v2 ] [ b ] = [ y2 - y1 ]
        // Ax = b
        let a: Array2<f64> = array![[u1 as f64, -u2 as f64], [v1 as f64, -v2 as f64]];

        let b: Array1<f64> = array![x2 as f64 - x1 as f64, y2 as f64 - y1 as f64];

        let x = a.solve_into(b).ok()?;
        let t = x[0];
        let s = x[1];

        if t < 0. || s < 0. {
            return None;
        }

        let x = x1 as f64 + u1 as f64 * t;
        let y = y1 as f64 + v1 as f64 * t;

        Some((x, y))
    }
}

impl std::fmt::Debug for HailStone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} @ {:?}", self.pos, self.vel)
    }
}

impl Problem<usize, i64> for Day24 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(newline, HailStone::parse)
            .map(|hailstones| Self { hailstones })
            .parse(input)
    }

    fn part1(self) -> Result<usize> {
        let mut cnt = 0;
        let r = 200000000000000.0..=400000000000000.0;
        for (i, a) in self.hailstones.iter().enumerate() {
            for b in self.hailstones.iter().skip(i + 1) {
                if let Some((x, y)) = a.intersects(b) {
                    if r.contains(&x) && r.contains(&y) {
                        cnt += 1;
                    }
                }
            }
        }
        Ok(cnt)
    }

    fn part2(self) -> Result<i64> {
        let HailStone { pos: p1, vel: v1 } = self.hailstones[0];
        let HailStone { pos: p2, vel: v2 } = self.hailstones[1];
        let HailStone { pos: p3, vel: v3 } = self.hailstones[2];

        let Vec3 {
            x: x1,
            y: y1,
            z: z1,
        } = p1;
        let Vec3 {
            x: x2,
            y: y2,
            z: z2,
        } = p2;
        let Vec3 {
            x: x3,
            y: y3,
            z: z3,
        } = p3;

        let Vec3 {
            x: u1,
            y: v1,
            z: w1,
        } = v1;
        let Vec3 {
            x: u2,
            y: v2,
            z: w2,
        } = v2;
        let Vec3 {
            x: u3,
            y: v3,
            z: w3,
        } = v3;

        let a: Array2<i64> = array![
            [0, -w1 + w3, v1 - v3, 0, z1 - z3, -y1 + y3],
            [w1 - w3, 0, -u1 + u3, -z1 + z3, 0, x1 - x3],
            [-v1 + v3, u1 - u3, 0, y1 - y3, -x1 + x3, 0],
            [0, -w2 + w3, v2 - v3, 0, z2 - z3, -y2 + y3],
            [w2 - w3, 0, -u2 + u3, -z2 + z3, 0, x2 - x3],
            [-v2 + v3, u2 - u3, 0, y2 - y3, -x2 + x3, 0],
        ];

        let b: Array1<i64> = -array![
            w1 * y1 - w3 * y3 - v1 * z1 + v3 * z3,
            -w1 * x1 + w3 * x3 + u1 * z1 - u3 * z3,
            v1 * x1 - v3 * x3 - u1 * y1 + u3 * y3,
            w2 * y2 - w3 * y3 - v2 * z2 + v3 * z3,
            -w2 * x2 + w3 * x3 + u2 * z2 - u3 * z3,
            v2 * x2 - v3 * x3 - u2 * y2 + u3 * y3
        ];

        let a: Array2<f64> = a.map(|&x| x as f64);
        let b: Array1<f64> = b.map(|&x| x as f64);

        let x = a.solve(&b).unwrap();

        Ok((x[0] + x[1] + x[2]).ceil() as i64)
    }
}

aoc_main!(Day24);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;

    #[test]
    fn test_part1() {
        assert_task!(Day24, 1, EXAMPLE, 0);
    }
}
