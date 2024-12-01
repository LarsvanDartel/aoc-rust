use aoc_rust::*;
use common::*;

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Command {
    fn parse(input: &str) -> ParseResult<Self> {
        alt((
            preceded(tag("forward "), parse_i32).map(Command::Forward),
            preceded(tag("up "), parse_i32).map(Command::Up),
            preceded(tag("down "), parse_i32).map(Command::Down),
        ))
        .parse(input)
    }
}

struct Day02 {
    commands: Vec<Command>,
}

impl Problem<i32, i32> for Day02 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, Command::parse)
            .map(|commands| Day02 { commands })
            .parse(input)
    }

    fn part1(self) -> Result<i32> {
        let mut depth = 0;
        let mut horizontal = 0;
        for command in self.commands {
            match command {
                Command::Forward(distance) => horizontal += distance,
                Command::Up(distance) => depth -= distance,
                Command::Down(distance) => depth += distance,
            }
        }

        Ok(depth * horizontal)
    }

    fn part2(self) -> Result<i32> {
        let mut depth = 0;
        let mut horizontal = 0;
        let mut aim = 0;
        for command in self.commands {
            match command {
                Command::Forward(distance) => {
                    horizontal += distance;
                    depth += aim * distance;
                }
                Command::Up(distance) => aim -= distance,
                Command::Down(distance) => aim += distance,
            }
        }

        Ok(depth * horizontal)
    }
}

aoc_main!(Day02);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day02, 1, EXAMPLE, 150);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day02, 2, EXAMPLE, 900);
    }
}
