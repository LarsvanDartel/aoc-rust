use aoc_rust::*;
use itertools::Itertools;

use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{i32 as number, line_ending},
    multi::separated_list1,
    sequence::{delimited, tuple},
    Parser,
};

struct Day13 {
    happiness: HashMap<(String, String), i32>,
}

impl Day13 {
    fn happiness(&self, a: &str, b: &str) -> i32 {
        self.happiness
            .get(&(a.to_string(), b.to_string()))
            .unwrap_or(&0)
            + self
                .happiness
                .get(&(b.to_string(), a.to_string()))
                .unwrap_or(&0)
    }
}

impl Problem<i32, i32> for Day13 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(
            line_ending,
            tuple((
                take_until(" would").map(|s: &str| s.to_string()),
                alt((
                    tag(" would gain ").map(|_| 1),
                    tag(" would lose ").map(|_| -1),
                )),
                number,
                delimited(
                    tag(" happiness units by sitting next to "),
                    take_until("."),
                    tag("."),
                )
                .map(|s: &str| s.to_string()),
            ))
            .map(|(a, sign, n, b)| ((a, b), sign * n)),
        )
        .map(|pairs| Day13 {
            happiness: pairs.into_iter().collect(),
        })
        .parse(input)
    }

    fn part1(self) -> Result<i32> {
        let people = self
            .happiness
            .keys()
            .flat_map(|(a, b)| vec![a, b])
            .cloned()
            .collect::<HashSet<_>>();

        let mut max_happiness = 0;
        for seating in people.iter().permutations(people.len()) {
            let happiness = seating
                .iter()
                .chain(std::iter::once(seating.first().unwrap()))
                .tuple_windows()
                .map(|(a, b)| self.happiness(a, b))
                .sum();

            max_happiness = max_happiness.max(happiness);
        }
        Ok(max_happiness)
    }

    fn part2(self) -> Result<i32> {
        let people = self
            .happiness
            .keys()
            .flat_map(|(a, b)| vec![a, b])
            .cloned()
            .chain(std::iter::once(String::from("Me")))
            .collect::<HashSet<_>>();

        let mut max_happiness = 0;
        for seating in people.iter().permutations(people.len()) {
            let happiness = seating
                .iter()
                .chain(std::iter::once(seating.first().unwrap()))
                .tuple_windows()
                .map(|(a, b)| self.happiness(a, b))
                .sum();

            max_happiness = max_happiness.max(happiness);
        }
        Ok(max_happiness)
    }
}

aoc_main!(Day13);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#;

    #[test]
    fn test_part1() {
        assert_task!(Day13, 1, EXAMPLE, 330);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day13, 2, EXAMPLE, 286);
    }
}
