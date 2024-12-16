use aoc_rust::*;
use common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    A,
    B,
}

impl Register {
    fn parse(input: &mut &str) -> PResult<Self> {
        one_of(['a', 'b'])
            .map(|c| match c {
                'a' => Self::A,
                'b' => Self::B,
                _ => unreachable!(),
            })
            .parse_next(input)
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
    fn parse(input: &mut &str) -> PResult<Self> {
        fn number(input: &mut &str) -> PResult<isize> {
            let sign = one_of(['+', '-']).parse_next(input)?;
            let number: isize = dec_int(input)?;
            Ok(if sign == '+' { number } else { -number } - 1)
        }

        alt((
            preceded("hlf ", Register::parse).map(Self::Half),
            preceded("tpl ", Register::parse).map(Self::Triple),
            preceded("inc ", Register::parse).map(Self::Increment),
            preceded("jmp ", number).map(Self::Jump),
            preceded("jie ", separated_pair(Register::parse, ", ", number))
                .map(|(r, n)| Self::JumpIfEven(r, n)),
            preceded("jio ", separated_pair(Register::parse, ", ", number))
                .map(|(r, n)| Self::JumpIfOne(r, n)),
        ))
        .parse_next(input)
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
            },
            Self::JumpIfOne(r, n) => {
                if registers[usize::from(*r)] == 1 {
                    *instruction_pointer = (*instruction_pointer as isize + n) as usize;
                }
            },
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
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(0.., Instruction::parse, line_ending)
            .map(|instructions| Self {
                instructions,
                instruction_pointer: 0,
                registers: [0, 0],
            })
            .parse_next(input)
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
