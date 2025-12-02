use aoc_rust::*;
use common::*;

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Command {
    fn parse(input: &mut &str) -> PResult<Self> {
        alt((
            preceded("forward ", dec_int).map(Command::Forward),
            preceded("up ", dec_int).map(Command::Up),
            preceded("down ", dec_int).map(Command::Down),
        ))
        .parse_next(input)
    }
}

struct Day02 {
    commands: Vec<Command>,
}

impl Problem<i32, i32> for Day02 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(0.., Command::parse, line_ending)
            .map(|commands| Day02 { commands })
            .parse_next(input)
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
