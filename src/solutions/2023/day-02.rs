use aoc_rust::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, u32 as parse_u32},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    Parser,
};

struct Day02 {
    games: Vec<Game>,
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn parse(input: &str) -> ParseResult<Self> {
        let (input, id) = delimited(tag("Game "), parse_u32, tag(": "))(input)?;

        let (input, draws) = separated_list1(tag("; "), Draw::parse)(input)?;

        Ok((input, Self { id, draws }))
    }

    fn is_possible(&self, draw: &Draw) -> bool {
        for d in &self.draws {
            if d.red > draw.red || d.green > draw.green || d.blue > draw.blue {
                return false;
            }
        }
        true
    }

    fn max(&self) -> Draw {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for d in &self.draws {
            if d.red > red {
                red = d.red;
            }
            if d.green > green {
                green = d.green;
            }
            if d.blue > blue {
                blue = d.blue;
            }
        }

        Draw { red, green, blue }
    }
}

#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(
            tag(", "),
            separated_pair(
                parse_u32,
                tag(" "),
                alt((tag("red"), tag("green"), tag("blue"))),
            ),
        )
        .map(|balls| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for (ball, color) in balls {
                match color {
                    "red" => red = ball,
                    "green" => green = ball,
                    "blue" => blue = ball,
                    _ => unreachable!(),
                }
            }

            Self { red, green, blue }
        })
        .parse(input)
    }
}

impl Problem<u32, u32> for Day02 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, Game::parse)
            .map(|games| Self { games })
            .parse(input)
    }

    fn part1(&self) -> Result<u32> {
        Ok(self
            .games
            .iter()
            .filter(|g| {
                g.is_possible(&Draw {
                    red: 12,
                    green: 13,
                    blue: 14,
                })
            })
            .map(|g| g.id)
            .sum())
    }

    fn part2(&self) -> Result<u32> {
        Ok(self
            .games
            .iter()
            .map(|g| g.max())
            .map(|d| d.red * d.green * d.blue)
            .sum())
    }
}

aoc_main!(Day02);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_part1() {
        assert_task!(Day02, 1, EXAMPLE, 8)
    }

    #[test]
    fn test_part2() {
        assert_task!(Day02, 2, EXAMPLE, 2286)
    }
}
