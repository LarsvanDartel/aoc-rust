use aoc_rust::*;
use common::*;

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn parse(c: char) -> Option<Self> {
        match c {
            '*' => Some(Self::Mul),
            '+' => Some(Self::Add),
            _ => None,
        }
    }

    fn apply(&self, a: &u64, b: &u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
        }
    }

    fn base(&self) -> u64 {
        match self {
            Self::Add => 0,
            Self::Mul => 1,
        }
    }
}

struct CephalopodProblem {
    numbers: Vec<u64>,
    op: Op,
}

impl CephalopodProblem {
    fn solve(self) -> u64 {
        self.numbers
            .iter()
            .fold(self.op.base(), |acc, b| self.op.apply(&acc, b))
    }
}

struct Day06 {
    problems: Vec<CephalopodProblem>,
}

impl Problem<u64, u64> for Day06 {
    fn parse_1(input: &mut &str) -> PResult<Self> {
        let mut lines = input.lines().rev();
        let mut problems: Vec<CephalopodProblem> = lines
            .next()
            .unwrap()
            .chars()
            .filter_map(Op::parse)
            .map(|op| CephalopodProblem {
                op,
                numbers: Vec::new(),
            })
            .collect();

        for l in lines {
            for (n, p) in l.split_whitespace().zip(problems.iter_mut()) {
                p.numbers.push(n.parse().unwrap());
            }
        }

        Ok(Self { problems })
    }

    fn parse_2(input: &mut &str) -> PResult<Self> {
        let mut lines = input.lines().map(|l| l.chars().rev()).collect_vec();
        let (op_line, number_lines) = lines.split_last_mut().unwrap();
        let mut ops = op_line.map(Op::parse);
        let mut problems = Vec::new();
        let mut numbers = Vec::new();
        while let Some(op) = ops.next() {
            numbers.push(
                number_lines
                    .iter_mut()
                    .filter_map(|l| {
                        let c = l.next().unwrap();
                        match c {
                            '0'..='9' => Some(c as u8 - b'0'),
                            _ => None,
                        }
                    })
                    .fold(0, |acc, x| 10 * acc + (x as u64)),
            );

            if let Some(op) = op {
                problems.push(CephalopodProblem { numbers, op });
                numbers = Vec::new();
                ops.next();
                for l in &mut *number_lines {
                    l.next();
                }
            }
        }

        Ok(Self { problems })
    }

    fn part1(self) -> Result<u64> {
        Ok(self
            .problems
            .into_iter()
            .map(CephalopodProblem::solve)
            .sum())
    }

    fn part2(self) -> Result<u64> {
        self.part1()
    }
}

aoc_main!(Day06);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_task!(Day06, 1, EXAMPLE, 4277556u64);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day06, 2, EXAMPLE, 3263827u64);
    }
}
