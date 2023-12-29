use std::ops::Range;

use aoc_rust::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, u32 as number},
    multi::separated_list1,
    Parser,
};

struct Day06 {
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    instruction: InstructionType,
    x_range: Range<usize>,
    y_range: Range<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InstructionType {
    TurnOn,
    TurnOff,
    Toggle,
}

impl InstructionType {
    fn parse(input: &str) -> ParseResult<Self> {
        alt((
            tag("turn on").map(|_| InstructionType::TurnOn),
            tag("turn off").map(|_| InstructionType::TurnOff),
            tag("toggle").map(|_| InstructionType::Toggle),
        ))(input)
    }
}

impl Instruction {
    fn parse(input: &str) -> ParseResult<Self> {
        let (input, instruction) = InstructionType::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, x0) = number(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y0) = number(input)?;
        let (input, _) = tag(" through ")(input)?;
        let (input, x1) = number(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y1) = number(input)?;

        let x_range = (x0 as usize)..(x1 as usize + 1);
        let y_range = (y0 as usize)..(y1 as usize + 1);

        Ok((
            input,
            Self {
                instruction,
                x_range,
                y_range,
            },
        ))
    }

    fn apply_1(&self, grid: &mut [Vec<bool>]) {
        for x in self.x_range.clone() {
            for y in self.y_range.clone() {
                match self.instruction {
                    InstructionType::TurnOn => grid[x][y] = true,
                    InstructionType::TurnOff => grid[x][y] = false,
                    InstructionType::Toggle => grid[x][y] = !grid[x][y],
                }
            }
        }
    }

    fn apply_2(&self, grid: &mut [Vec<u32>]) {
        for x in self.x_range.clone() {
            for y in self.y_range.clone() {
                match self.instruction {
                    InstructionType::TurnOn => grid[x][y] += 1,
                    InstructionType::TurnOff => grid[x][y] = grid[x][y].saturating_sub(1),
                    InstructionType::Toggle => grid[x][y] += 2,
                }
            }
        }
    }
}

impl Problem<usize, u32> for Day06 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, Instruction::parse)
            .map(|instructions| Self { instructions })
            .parse(input)
    }

    fn part1(self) -> Result<usize> {
        let mut grid = vec![vec![false; 1000]; 1000];
        for instruction in self.instructions {
            instruction.apply_1(&mut grid);
        }
        Ok(grid.iter().flatten().filter(|&&x| x).count())
    }

    fn part2(self) -> Result<u32> {
        let mut grid = vec![vec![0; 1000]; 1000];
        for instruction in self.instructions {
            instruction.apply_2(&mut grid);
        }
        Ok(grid.iter().flatten().sum())
    }
}

aoc_main!(Day06);
