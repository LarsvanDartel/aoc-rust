use aoc_rust::*;
use common::*;
use itertools::Itertools;

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
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(
            0..,
            (
                alpha1.map(|s: &str| s.to_string()),
                alt((" would gain ".map(|_| 1), " would lose ".map(|_| -1))),
                dec_int::<_, i32, _>,
                delimited(
                    " happiness units by sitting next to ",
                    alpha1.map(|s: &str| s.to_string()),
                    '.',
                ),
            )
                .map(|(a, sign, n, b)| ((a, b), sign * n)),
            line_ending,
        )
        .map(|pairs: Vec<((String, String), i32)>| Day13 {
            happiness: pairs.into_iter().collect(),
        })
        .parse_next(input)
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
