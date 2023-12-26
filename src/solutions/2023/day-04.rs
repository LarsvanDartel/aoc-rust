use aoc_rust::*;

use nom::{
    bytes::complete::tag,
    character::complete::space1,
    character::complete::{line_ending, u32 as parse_u32},
    multi::separated_list1,
    sequence::tuple,
    Parser,
};

struct Day04 {
    cards: Vec<Card>,
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn parse(input: &str) -> ParseResult<Self> {
        tuple((
            tag("Card"),
            space1,
            parse_u32,
            tag(":"),
            space1,
            separated_list1(space1, parse_u32),
            space1,
            tag("|"),
            space1,
            separated_list1(space1, parse_u32),
        ))
        .map(|(_, _, _, _, _, winning_numbers, _, _, _, numbers)| Card {
            winning_numbers,
            numbers,
        })
        .parse(input)
    }

    fn score(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32
    }
}

impl Problem<u32, u32> for Day04 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, Card::parse)
            .map(|cards| Self { cards })
            .parse(input)
    }

    fn part1(self) -> Result<u32> {
        Ok(self
            .cards
            .iter()
            .map(|card| {
                let score = card.score();
                if score == 0 {
                    0
                } else {
                    1 << (score - 1)
                }
            })
            .sum())
    }

    fn part2(self) -> Result<u32> {
        let mut c = vec![1; self.cards.len()];
        for i in 0..self.cards.len() {
            let mut score = self.cards[i].score();
            let mut j = i + 1;
            while j < self.cards.len() && score > 0 {
                score -= 1;
                c[j] += c[i];
                j += 1;
            }
        }
        Ok(c.iter().sum())
    }
}

aoc_main!(Day04);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_part1() {
        assert_task!(Day04, 1, EXAMPLE, 13)
    }

    #[test]
    fn test_part2() {
        assert_task!(Day04, 2, EXAMPLE, 30)
    }
}
