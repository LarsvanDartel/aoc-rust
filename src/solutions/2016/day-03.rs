use aoc_rust::*;
use nom::{
    character::complete::{line_ending, space1, u32 as parse_u32},
    multi::separated_list1,
    sequence::tuple,
    Parser,
};

struct Day03 {
    triangles: Vec<Triangle>,
}

struct Triangle((u32, u32, u32));

impl Triangle {
    fn parse(input: &str) -> ParseResult<Self> {
        tuple((space1, parse_u32, space1, parse_u32, space1, parse_u32))
            .map(|(_, a, _, b, _, c)| Self((a, b, c)))
            .parse(input)
    }

    fn is_valid(&self) -> bool {
        let mut sides = [self.0 .0, self.0 .1, self.0 .2];
        sides.sort_unstable();
        sides[0] + sides[1] > sides[2]
    }
}

impl Problem<usize, usize> for Day03 {
    fn parse_1(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, Triangle::parse)
            .map(|triangles| Self { triangles })
            .parse(input)
    }

    fn parse_2(input: &str) -> ParseResult<Self> {
        let mut triangles = Vec::new();
        let mut lines = input.lines().map(str::trim);
        while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
            let a = a
                .split_whitespace()
                .map(str::parse::<u32>)
                .collect::<std::result::Result<Vec<_>, _>>()
                .unwrap();
            let b = b
                .split_whitespace()
                .map(str::parse::<u32>)
                .collect::<std::result::Result<Vec<_>, _>>()
                .unwrap();
            let c = c
                .split_whitespace()
                .map(str::parse::<u32>)
                .collect::<std::result::Result<Vec<_>, _>>()
                .unwrap();

            for i in 0..3 {
                triangles.push(Triangle((a[i], b[i], c[i])));
            }
        }

        Ok(("", Self { triangles }))
    }

    fn part1(self) -> Result<usize> {
        Ok(self.triangles.iter().filter(|t| t.is_valid()).count())
    }

    fn part2(self) -> Result<usize> {
        Self::part1(self)
    }
}

aoc_main!(Day03);
