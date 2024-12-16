use aoc_rust::*;
use common::*;

struct Day18 {
    dig_plan: Vec<Dig>,
}

#[derive(Debug, Clone, Copy)]
struct Dig {
    dir: Direction,
    len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn parse(input: &mut &str) -> PResult<Self> {
        alt((
            "L".map(|_| Direction::Left),
            "R".map(|_| Direction::Right),
            "U".map(|_| Direction::Up),
            "D".map(|_| Direction::Down),
        ))
        .parse_next(input)
    }
}

impl std::ops::Add<Direction> for (isize, isize) {
    type Output = (isize, isize);

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Left => (self.0 - 1, self.1),
            Direction::Right => (self.0 + 1, self.1),
            Direction::Up => (self.0, self.1 - 1),
            Direction::Down => (self.0, self.1 + 1),
        }
    }
}

impl std::ops::Mul<usize> for Direction {
    type Output = (isize, isize);

    fn mul(self, rhs: usize) -> Self::Output {
        match self {
            Direction::Left => (-(rhs as isize), 0),
            Direction::Right => (rhs as isize, 0),
            Direction::Up => (0, -(rhs as isize)),
            Direction::Down => (0, rhs as isize),
        }
    }
}

impl Dig {
    fn parse(input: &mut &str, use_color: bool) -> PResult<Self> {
        separated_pair(
            separated_pair(Direction::parse, space1, dec_usize),
            space1,
            delimited("(", preceded("#", hex_digit1.map(String::from)), ")"),
        )
        .map(|((dir, len), color)| {
            if !use_color {
                Dig { dir, len }
            } else {
                // len is the first 5 digits of the hex color
                let len = usize::from_str_radix(&color[..5], 16).unwrap();
                let dir = match &color.chars().nth(5).unwrap() {
                    '0' => Direction::Right,
                    '1' => Direction::Down,
                    '2' => Direction::Left,
                    '3' => Direction::Up,
                    _ => unreachable!(),
                };
                Dig { dir, len }
            }
        })
        .parse_next(input)
    }
}

impl Day18 {
    fn calculate_area(&self) -> usize {
        let mut pos: (isize, isize) = (0, 0);
        let mut boundary = vec![pos];
        let mut perimeter_len = 0;
        for dig in &self.dig_plan {
            pos.0 += (dig.dir * dig.len).0;
            pos.1 += (dig.dir * dig.len).1;
            boundary.push(pos);
            perimeter_len += dig.len;
        }
        // use the shoelace formula to calculate the area
        let area = boundary.windows(2).fold(0, |acc, pair| {
            acc + pair[0].0 * pair[1].1 - pair[1].0 * pair[0].1
        });

        // pick's theorem:
        // A = i + b/2 - 1 => i = A - b/2 + 1
        // we add the perimeter length to the area because the perimeter is not included in the area
        // so we get i = A + b/2 + 1
        (area.unsigned_abs() / 2) + (perimeter_len / 2) + 1
    }
}

impl Problem<usize, usize> for Day18 {
    fn parse_1(input: &mut &str) -> PResult<Self> {
        fn dig_parse(input: &mut &str) -> PResult<Dig> {
            Dig::parse(input, false)
        }
        list(dig_parse, line_ending)
            .map(|dig_plan| Self { dig_plan })
            .parse_next(input)
    }

    fn parse_2(input: &mut &str) -> PResult<Self> {
        fn dig_parse(input: &mut &str) -> PResult<Dig> {
            Dig::parse(input, true)
        }
        list(dig_parse, line_ending)
            .map(|dig_plan| Self { dig_plan })
            .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self.calculate_area())
    }

    fn part2(self) -> Result<usize> {
        Ok(self.calculate_area())
    }
}

aoc_main!(Day18);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn test_part1() {
        assert_task!(Day18, 1, EXAMPLE, 62);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day18, 2, EXAMPLE, 952408144115usize);
    }
}
