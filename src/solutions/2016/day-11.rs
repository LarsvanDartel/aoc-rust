use std::collections::{HashSet, VecDeque};

use aoc_rust::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
    Parser,
};

#[derive(Clone)]
struct Day11 {
    floors: [HashSet<Item>; 4],
    elevator: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Item {
    MicroChip(Element),
    Generator(Element),
}

impl Item {
    fn parse(input: &str) -> ParseResult<Self> {
        preceded(
            tag("a "),
            alt((
                terminated(Element::parse, tag("-compatible microchip")).map(Self::MicroChip),
                terminated(Element::parse, tag(" generator")).map(Self::Generator),
            )),
        )
        .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Element {
    Thulium = 0b0000001,
    Plutonium = 0b00000010,
    Strontium = 0b0000100,
    Promethium = 0b0001000,
    Ruthenium = 0b0010000,
    Elerium = 0b0100000,
    Dilithium = 0b1000000,
}

impl Element {
    fn parse(input: &str) -> ParseResult<Self> {
        alt((
            tag("thulium").map(|_| Self::Thulium),
            tag("plutonium").map(|_| Self::Plutonium),
            tag("strontium").map(|_| Self::Strontium),
            tag("promethium").map(|_| Self::Promethium),
            tag("ruthenium").map(|_| Self::Ruthenium),
        ))
        .parse(input)
    }
}

impl Day11 {
    fn next_states(&self) -> Vec<Self> {
        let mut next_states = Vec::new();

        for item_1 in self.floors[self.elevator].iter() {
            if self.elevator > 0 {
                let mut next = self.clone();
                next.floors[self.elevator].remove(item_1);
                next.floors[self.elevator - 1].insert(*item_1);
                next.elevator -= 1;
                next_states.push(next);
            }
            if self.elevator < 3 {
                let mut next = self.clone();
                next.floors[self.elevator].remove(item_1);
                next.floors[self.elevator + 1].insert(*item_1);
                next.elevator += 1;
                next_states.push(next);
            }
            for item_2 in self.floors[self.elevator].iter() {
                if item_2 == item_1 {
                    continue;
                }
                if self.elevator > 0 {
                    let mut next = self.clone();
                    next.floors[self.elevator].remove(item_1);
                    next.floors[self.elevator].remove(item_2);
                    next.floors[self.elevator - 1].insert(*item_1);
                    next.floors[self.elevator - 1].insert(*item_2);
                    next.elevator -= 1;
                    next_states.push(next);
                }
                if self.elevator < 3 {
                    let mut next = self.clone();
                    next.floors[self.elevator].remove(item_1);
                    next.floors[self.elevator].remove(item_2);
                    next.floors[self.elevator + 1].insert(*item_1);
                    next.floors[self.elevator + 1].insert(*item_2);
                    next.elevator += 1;
                    next_states.push(next);
                }
            }
        }

        next_states
    }

    fn is_valid(&self) -> bool {
        for floor in self.floors.iter() {
            let mut generators = 0;
            let mut microchips = 0;
            for item in floor {
                match item {
                    Item::MicroChip(element) => microchips |= *element as u8,
                    Item::Generator(element) => generators |= *element as u8,
                }
            }
            if generators > 0 && microchips & !generators > 0 {
                return false;
            }
        }
        true
    }

    fn state_hash(&self) -> u32 {
        // 2 bits for elevator position
        // 3 bits per floor for microchip count (we only have 7 elements)
        // 3 bits per floor for generator count
        // total = 2 + (3 + 3) * 4 = 28
        let mut hash = 0;
        for (i, floor) in self.floors.iter().enumerate() {
            let mut g_cnt = 0;
            let mut m_cnt = 0;
            for item in floor {
                match item {
                    Item::MicroChip(_) => m_cnt += 1,
                    Item::Generator(_) => g_cnt += 1,
                }
            }
            hash |= ((g_cnt << 3) | m_cnt) << (i * 6);
        }
        hash | ((self.elevator as u32) << (4 * 6))
    }

    fn solve(&self) -> usize {
        let mut visited = HashSet::new();

        let mut min_steps = usize::MAX;
        let mut queue = VecDeque::new();

        queue.push_back((self.clone(), 0));

        while let Some((state, steps)) = queue.pop_front() {
            if state.floors[0].is_empty()
                && state.floors[1].is_empty()
                && state.floors[2].is_empty()
            {
                min_steps = min_steps.min(steps);
                continue;
            }

            if steps >= min_steps {
                continue;
            }

            if !visited.insert(state.state_hash()) {
                continue;
            }

            for next in state.next_states() {
                if next.is_valid() {
                    queue.push_back((next, steps + 1));
                }
            }
        }

        min_steps
    }
}

impl Problem<usize, usize> for Day11 {
    fn parse(mut input: &str) -> ParseResult<Self> {
        const FLOORS: [&str; 4] = ["first", "second", "third", "fourth"];

        fn parse_floor<'a>(floor: &str, input: &'a str) -> ParseResult<'a, Vec<Item>> {
            delimited(
                tag(format!("The {} floor contains ", floor).as_str()),
                alt((
                    separated_list1(alt((tag(" and "), tag(", and "), tag(", "))), Item::parse),
                    tag("nothing relevant").map(|_| Vec::new()),
                )),
                tag(".").and(line_ending),
            )
            .parse(input)
        }

        let mut floors: [HashSet<Item>; 4] = Default::default();
        for (i, floor) in FLOORS.iter().enumerate() {
            let floor_vec;
            (input, floor_vec) = parse_floor(floor, input)?;
            floors[i] = floor_vec.into_iter().collect();
        }

        Ok((
            input,
            Self {
                floors,
                elevator: 0,
            },
        ))
    }

    fn part1(self) -> Result<usize> {
        Ok(self.solve())
    }

    fn part2(mut self) -> Result<usize> {
        self.floors[0].insert(Item::MicroChip(Element::Elerium));
        self.floors[0].insert(Item::Generator(Element::Elerium));
        self.floors[0].insert(Item::MicroChip(Element::Dilithium));
        self.floors[0].insert(Item::Generator(Element::Dilithium));
        Ok(self.solve())
    }
}

aoc_main!(Day11);
