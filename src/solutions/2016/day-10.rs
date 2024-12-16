use aoc_rust::*;
use common::*;

struct Day10 {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
enum Instruction {
    Value {
        value: usize,
        to: Destination,
    },
    Transfer {
        from: Destination,
        low: Destination,
        high: Destination,
    },
}

impl Instruction {
    fn parse(input: &mut &str) -> PResult<Self> {
        alt((
            preceded(
                "value ",
                separated_pair(dec_uint, " goes to ", Destination::parse),
            )
            .map(|(value, to)| Self::Value { value, to }),
            (
                Destination::parse,
                " gives low to ",
                Destination::parse,
                " and high to ",
                Destination::parse,
            )
                .map(|(from, _, low, _, high)| Self::Transfer { from, low, high }),
        ))
        .parse_next(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Destination {
    Bot(usize),
    Output(usize),
}

impl Destination {
    fn parse(input: &mut &str) -> PResult<Self> {
        alt((
            preceded("bot ", dec_uint).map(Self::Bot),
            preceded("output ", dec_uint).map(Self::Output),
        ))
        .parse_next(input)
    }
}

#[derive(Debug)]
struct Robot {
    id: Destination,
    low: Option<Destination>,
    high: Option<Destination>,
    inventory: Vec<usize>,
}

impl Robot {
    fn new(id: Destination) -> Self {
        Self {
            id,
            low: None,
            high: None,
            inventory: vec![],
        }
    }

    fn obtain(&mut self, value: usize) -> Result<()> {
        if self.is_full() {
            Err("Inventory full")?;
        }

        self.inventory.push(value);
        Ok(())
    }

    fn is_full(&self) -> bool {
        matches!(self.id, Destination::Bot(_)) && self.inventory.len() == 2
    }

    fn get_id(&self) -> usize {
        match self.id {
            Destination::Bot(id) => id,
            Destination::Output(id) => id,
        }
    }
}

fn build_robots(instructions: Vec<Instruction>) -> Result<HashMap<Destination, Robot>> {
    let mut robots = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::Transfer { from, low, high } => {
                if let Destination::Bot(_) = from {
                    let r = robots.entry(from).or_insert(Robot::new(from));
                    r.low = Some(low);
                    r.high = Some(high);
                } else {
                    Err("Cannot transfer from output")?;
                }
            },
            Instruction::Value { value, to } => {
                if let Destination::Bot(_) = to {
                    let r = robots.entry(to).or_insert(Robot::new(to));
                    r.obtain(value)?;
                } else {
                    Err("Cannont transfer to output in initialization")?;
                }
            },
        }
    }

    Ok(robots)
}

impl Problem<usize, usize> for Day10 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(0.., Instruction::parse, line_ending)
            .map(|instructions| Self { instructions })
            .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        let mut robots = build_robots(self.instructions)?;
        loop {
            let robot = robots
                .values_mut()
                .find(|r| r.is_full())
                .ok_or("Could not find robot with full inventory")?;

            let low_dest = robot.low.ok_or("Destination undefined")?;
            let high_dest = robot.high.ok_or("Destination undefined")?;

            let low = *robot.inventory.iter().min().unwrap();
            let high = *robot.inventory.iter().max().unwrap();

            if low == 17 && high == 61 {
                return Ok(robot.get_id());
            }

            robot.inventory.clear();

            robots
                .entry(low_dest)
                .or_insert(Robot::new(low_dest))
                .obtain(low)?;
            robots
                .entry(high_dest)
                .or_insert(Robot::new(high_dest))
                .obtain(high)?;
        }
    }

    fn part2(self) -> Result<usize> {
        let mut robots = build_robots(self.instructions)?;
        loop {
            let robot = robots.values_mut().find(|r| r.is_full());

            if robot.is_none() {
                let out0 = robots
                    .get(&Destination::Output(0))
                    .ok_or("Could not find output 0")?
                    .inventory
                    .first()
                    .ok_or("Output 0 empty")?;
                let out1 = robots
                    .get(&Destination::Output(1))
                    .ok_or("Could not find output 1")?
                    .inventory
                    .first()
                    .ok_or("Output 1 empty")?;
                let out2 = robots
                    .get(&Destination::Output(2))
                    .ok_or("Could not find output 2")?
                    .inventory
                    .first()
                    .ok_or("Output 2 empty")?;

                return Ok(out0 * out1 * out2);
            }

            let robot = robot.unwrap();

            let low_dest = robot.low.ok_or("Destination undefined")?;
            let high_dest = robot.high.ok_or("Destination undefined")?;

            let low = *robot.inventory.iter().min().unwrap();
            let high = *robot.inventory.iter().max().unwrap();

            robot.inventory.clear();

            robots
                .entry(low_dest)
                .or_insert(Robot::new(low_dest))
                .obtain(low)?;
            robots
                .entry(high_dest)
                .or_insert(Robot::new(high_dest))
                .obtain(high)?;
        }
    }
}

aoc_main!(Day10);
