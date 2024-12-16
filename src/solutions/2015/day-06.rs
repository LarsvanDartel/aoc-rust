use std::ops::Range;

use aoc_rust::*;
use common::*;

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
    fn parse(input: &mut &str) -> PResult<Self> {
        alt((
            "turn on".map(|_| InstructionType::TurnOn),
            "turn off".map(|_| InstructionType::TurnOff),
            "toggle".map(|_| InstructionType::Toggle),
        ))
        .parse_next(input)
    }
}

impl Instruction {
    fn parse(input: &mut &str) -> PResult<Self> {
        let instruction = InstructionType::parse(input)?;
        let _ = space1(input)?;
        let x0 = dec_uint(input)?;
        let _ = ','.parse_next(input)?;
        let y0 = dec_uint(input)?;
        let _ = " through ".parse_next(input)?;
        let x1: usize = dec_uint(input)?;
        let _ = ','.parse_next(input)?;
        let y1: usize = dec_uint(input)?;

        let x_range = x0..x1 + 1;
        let y_range = y0..y1 + 1;

        Ok(Self {
            instruction,
            x_range,
            y_range,
        })
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
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(0.., Instruction::parse, line_ending)
            .map(|instructions| Self { instructions })
            .parse_next(input)
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
