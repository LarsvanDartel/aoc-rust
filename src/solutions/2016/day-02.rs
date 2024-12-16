use aoc_rust::*;
use common::*;

struct Day02 {
    instructions: Vec<Instruction>,
}

struct Instruction {
    code: Vec<Direction>,
}

impl Instruction {
    fn parse(input: &mut &str) -> PResult<Self> {
        repeat(0.., Direction::parse_udlr)
            .map(|code| Self { code })
            .parse_next(input)
    }
}

impl Problem<String, String> for Day02 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(0.., Instruction::parse, line_ending)
            .map(|instructions| Self { instructions })
            .parse_next(input)
    }

    fn part1(self) -> Result<String> {
        const KEYPAD: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

        let mut pos = Vec2::new(0, 0);
        let mut code = String::new();
        for instruction in self.instructions {
            for dir in instruction.code {
                let new_pos = pos + dir;
                if new_pos.map(|x| x.abs() <= 1).all() {
                    pos = new_pos;
                }
            }

            pos += Vec2::new(1, 1);
            code.push(KEYPAD[pos.y as usize][pos.x as usize]);
            pos -= Vec2::new(1, 1);
        }

        Ok(code)
    }

    fn part2(self) -> Result<String> {
        const KEYPAD: [[char; 5]; 5] = [
            [' ', ' ', '1', ' ', ' '],
            [' ', '2', '3', '4', ' '],
            ['5', '6', '7', '8', '9'],
            [' ', 'A', 'B', 'C', ' '],
            [' ', ' ', 'D', ' ', ' '],
        ];

        let mut pos = Vec2::new(-2, 0);
        let mut code = String::new();
        for instruction in self.instructions {
            for dir in instruction.code {
                let new_pos = pos + dir;
                if new_pos.map(isize::abs).sum() <= 2 {
                    pos = new_pos;
                }
            }

            pos += Vec2::new(2, 2);
            code.push(KEYPAD[pos.y as usize][pos.x as usize]);
            pos -= Vec2::new(2, 2);
        }

        Ok(code)
    }
}

aoc_main!(Day02);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"ULL
RRDDD
LURDL
UUUUD"#;

    #[test]
    fn test_part1() {
        assert_task!(Day02, 1, EXAMPLE, "1985");
    }

    #[test]
    fn test_part2() {
        assert_task!(Day02, 2, EXAMPLE, "5DB3");
    }
}
