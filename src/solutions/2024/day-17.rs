use aoc_rust::*;
use common::*;

struct Day17 {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
    pc: usize,
}

impl Day17 {
    fn get_combo(&self, value: u8) -> u64 {
        match value {
            0..=3 => value as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
    fn step(&mut self) -> Option<u8> {
        let op = self.program.get(self.pc)?;
        let value = *self.program.get(self.pc + 1)?;
        let combo = self.get_combo(value);
        self.pc += 2;
        match op {
            0 => self.a >>= combo,
            1 => self.b ^= value as u64,
            2 => self.b = combo & 0b111,
            3 => {
                if self.a != 0 {
                    self.pc = value as usize
                }
            }
            4 => self.b ^= self.c,
            5 => return Some((combo & 0b111) as u8),
            6 => self.b = self.a >> combo,
            7 => self.c = self.a >> combo,
            _ => unreachable!(),
        }
        None
    }

    fn run_until_output(&mut self) -> Option<u8> {
        self.pc = 0;
        while self.pc < self.program.len() {
            if let Some(value) = self.step() {
                return Some(value);
            }
        }
        None
    }

    fn run(&mut self) -> Vec<u8> {
        self.pc = 0;
        let mut output = Vec::new();
        while self.pc < self.program.len() {
            if let Some(value) = self.step() {
                output.push(value);
            }
        }
        output
    }
}

impl Problem<String, u64> for Day17 {
    fn parse(input: &mut &str) -> PResult<Self> {
        let a = preceded("Register A: ", dec_uint).parse_next(input)?;
        let _ = line_ending.parse_next(input)?;
        let b = preceded("Register B: ", dec_uint).parse_next(input)?;
        let _ = line_ending.parse_next(input)?;
        let c = preceded("Register C: ", dec_uint).parse_next(input)?;
        let _ = (line_ending, line_ending).parse_next(input)?;
        let program = preceded("Program: ", list(dec_uint, ',')).parse_next(input)?;
        Ok(Self {
            a,
            b,
            c,
            program,
            pc: 0,
        })
    }

    fn part1(mut self) -> Result<String> {
        Ok(self.run().iter().join(","))
    }

    fn part2(mut self) -> Result<u64> {
        let nums = self.program.clone();

        let mut possible = HashSet::new();
        possible.insert(0);

        for out in nums.into_iter().rev() {
            let mut next = HashSet::new();
            for val in possible {
                for i in 0..8 {
                    let a = val << 3 | i;
                    self.a = a;
                    if self.run_until_output() == Some(out) {
                        next.insert(a);
                    }
                }
            }
            possible = next;
        }

        possible.into_iter().min().ok_or(AoCError::NoSolution)
    }
}

aoc_main!(Day17);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#;

    const EXAMPLE2: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day17, 1, EXAMPLE1, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        assert_task!(Day17, 2, EXAMPLE2, 117440);
    }
}
