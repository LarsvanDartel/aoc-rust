use aoc_rust::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, one_of},
    multi::separated_list0,
    sequence::{terminated, tuple},
    Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    A,
    B,
}

impl Register {
    fn parse(input: &str) -> ParseResult<Self> {
        one_of("ab")
            .map(|c| match c {
                'a' => Self::A,
                'b' => Self::B,
                _ => unreachable!(),
            })
            .parse(input)
    }
}

impl From<Register> for usize {
    fn from(r: Register) -> Self {
        match r {
            Register::A => 0,
            Register::B => 1,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

impl Instruction {
    fn parse(input: &str) -> ParseResult<Self> {
        fn number(input: &str) -> ParseResult<isize> {
            let (input, sign) = one_of("+-")(input)?;
            let (input, number) = digit1(input)?;
            let number = number.parse::<isize>().unwrap();
            Ok((input, if sign == '+' { number } else { -number } - 1))
        }
        alt((
            tuple((tag("hlf "), Register::parse)).map(|(_, r)| Self::Half(r)),
            tuple((tag("tpl "), Register::parse)).map(|(_, r)| Self::Triple(r)),
            tuple((tag("inc "), Register::parse)).map(|(_, r)| Self::Increment(r)),
            tuple((tag("jmp "), number)).map(|(_, n)| Self::Jump(n)),
            tuple((tag("jie "), terminated(Register::parse, tag(", ")), number))
                .map(|(_, r, n)| Self::JumpIfEven(r, n)),
            tuple((tag("jio "), terminated(Register::parse, tag(", ")), number))
                .map(|(_, r, n)| Self::JumpIfOne(r, n)),
        ))
        .parse(input)
    }

    fn apply(&self, registers: &mut [u32; 2], instruction_pointer: &mut usize) {
        match self {
            Self::Half(r) => registers[usize::from(*r)] /= 2,
            Self::Triple(r) => registers[usize::from(*r)] *= 3,
            Self::Increment(r) => registers[usize::from(*r)] += 1,
            Self::Jump(n) => *instruction_pointer = (*instruction_pointer as isize + n) as usize,
            Self::JumpIfEven(r, n) => {
                if registers[usize::from(*r)] % 2 == 0 {
                    *instruction_pointer = (*instruction_pointer as isize + n) as usize;
                }
            }
            Self::JumpIfOne(r, n) => {
                if registers[usize::from(*r)] == 1 {
                    *instruction_pointer = (*instruction_pointer as isize + n) as usize;
                }
            }
        }
    }
}

struct Day23 {
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
    registers: [u32; 2],
}

impl Day23 {
    fn run(&mut self) {
        while self.instruction_pointer < self.instructions.len() {
            self.instructions[self.instruction_pointer]
                .apply(&mut self.registers, &mut self.instruction_pointer);
            self.instruction_pointer += 1;
        }
    }
}

impl Problem<u32, u32> for Day23 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list0(line_ending, Instruction::parse)
            .map(|instructions| Self {
                instructions,
                instruction_pointer: 0,
                registers: [0, 0],
            })
            .parse(input)
    }

    fn part1(mut self) -> Result<u32> {
        self.run();
        Ok(self.registers[1])
    }

    fn part2(mut self) -> Result<u32> {
        self.registers[0] = 1;
        self.run();
        Ok(self.registers[1])
    }
}

aoc_main!(Day23);
