use std::collections::VecDeque;

use aoc_rust::*;
use common::*;

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
    fn parse(input: &mut &str) -> PResult<Self> {
        preceded(
            "a ",
            alt((
                terminated(Element::parse, "-compatible microchip").map(Self::MicroChip),
                terminated(Element::parse, " generator").map(Self::Generator),
            )),
        )
        .parse_next(input)
    }

    fn custom_hash(&self) -> u16 {
        match self {
            Self::MicroChip(element) => *element as u16,
            Self::Generator(element) => (*element as u16) << (Element::N_ELEMENTS as u16),
        }
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
    const N_ELEMENTS: usize = 7;

    fn parse(input: &mut &str) -> PResult<Self> {
        alt((
            "thulium".map(|_| Self::Thulium),
            "plutonium".map(|_| Self::Plutonium),
            "strontium".map(|_| Self::Strontium),
            "promethium".map(|_| Self::Promethium),
            "ruthenium".map(|_| Self::Ruthenium),
        ))
        .parse_next(input)
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

    fn state_hash(&self) -> u64 {
        let mut hash = 0;
        for (i, floor) in self.floors.iter().enumerate() {
            for item in floor {
                hash |= (item.custom_hash() as u64) << (i as u64 * 2 * Element::N_ELEMENTS as u64);
            }
        }
        hash |= (self.elevator as u64) << (4 * 2 * Element::N_ELEMENTS as u64);
        hash
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
    fn parse(input: &mut &str) -> PResult<Self> {
        const FLOORS: [&str; 4] = ["first", "second", "third", "fourth"];

        fn parse_floor(floor: &str, input: &mut &str) -> PResult<Vec<Item>> {
            delimited(
                format!("The {} floor contains ", floor).as_str(),
                alt((
                    separated(1.., Item::parse, alt((" and ", ", and ", ", "))),
                    "nothing relevant".map(|_| Vec::new()),
                )),
                (".", line_ending),
            )
            .parse_next(input)
        }

        let mut floors: [HashSet<Item>; 4] = Default::default();
        for (i, floor) in FLOORS.iter().enumerate() {
            let floor_vec = parse_floor(floor, input)?;
            floors[i] = floor_vec.into_iter().collect();
        }

        Ok(Self {
            floors,
            elevator: 0,
        })
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
