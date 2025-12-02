use aoc_rust::*;
use common::*;

struct Day07 {
    hands: Vec<Hand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Card {
    Number(u32),
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn value(&self, jokers: bool) -> u64 {
        match self {
            Card::Number(n) => *n as u64,
            Card::Ten => 10,
            Card::Jack => {
                if jokers {
                    1
                } else {
                    11
                }
            }
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
        }
    }

    fn parse(input: &mut &str) -> PResult<Card> {
        alt((
            one_of('2'..='9').map(|c: char| Card::Number(c.to_digit(10).unwrap())),
            "T".map(|_| Card::Ten),
            "J".map(|_| Card::Jack),
            "Q".map(|_| Card::Queen),
            "K".map(|_| Card::King),
            "A".map(|_| Card::Ace),
        ))
        .parse_next(input)
    }
}

impl Hand {
    fn parse(input: &mut &str) -> PResult<Hand> {
        separated_pair(many(Card::parse), space1, dec_u64)
            .map(|(cards, bid)| Hand { cards, bid })
            .parse_next(input)
    }

    fn score(&self, jokers: bool) -> u64 {
        let mut counts = [0; 15];
        let mut n = 0;
        for card in &self.cards {
            if card.value(jokers) != 1 {
                counts[card.value(jokers) as usize] += 1;
            } else {
                n += 1;
            }
        }

        let counts = counts.iter().filter(|&&c| c > 0).collect::<Vec<_>>();
        let mut counts = counts.iter().map(|&&c| c).collect::<Vec<_>>();
        counts.sort();
        counts.reverse();
        counts.push(0);

        let score = match n + counts[0] {
            5 => 6,
            4 => 5,
            3 => {
                if counts[1] == 2 {
                    4
                } else {
                    3
                }
            }
            2 => {
                if counts[1] == 2 {
                    2
                } else {
                    1
                }
            }
            1 => 0,
            _ => panic!("invalid hand"),
        };

        self.cards
            .iter()
            .fold(score, |a, card| a * 15 + card.value(jokers))
    }
}

impl Problem<u64, u64> for Day07 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(Hand::parse, line_ending)
            .map(|hands| Self { hands })
            .parse_next(input)
    }

    fn part1(self) -> Result<u64> {
        let mut hands = self.hands;
        hands.sort_by_key(|hand| hand.score(false));

        Ok(hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) as u64 * hand.bid)
            .sum())
    }

    fn part2(self) -> Result<u64> {
        let mut hands = self.hands;
        hands.sort_by_key(|hand| hand.score(true));

        Ok(hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) as u64 * hand.bid)
            .sum())
    }
}

aoc_main!(Day07);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_part1() {
        assert_task!(Day07, 1, EXAMPLE, 6440)
    }

    #[test]
    fn test_part2() {
        assert_task!(Day07, 2, EXAMPLE, 5905)
    }
}
