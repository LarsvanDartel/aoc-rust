use std::iter;

use aoc_rust::*;
use common::*;

struct Day04 {
    numbers: Vec<u32>,
    cards: Vec<Grid<u32>>,
}

impl Problem<u32, u32> for Day04 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated_pair(
            separated(0.., dec_uint::<_, u32, _>, ','),
            (line_ending, line_ending),
            separated(0.., Grid::parse(preceded(space0, dec_uint)), line_ending),
        )
        .map(|(numbers, cards)| Day04 { numbers, cards })
        .parse_next(input)
    }

    fn part1(self) -> Result<u32> {
        let mut cards = iter::repeat(Grid::<bool>::new(5, 5))
            .take(self.cards.len())
            .collect::<Vec<_>>();
        for number in &self.numbers {
            for (i, card) in self.cards.iter().enumerate() {
                if let Some(pos) = card
                    .coordinates()
                    .find(|&pos| card.get(pos) == Some(number))
                {
                    cards[i].set(pos, true);

                    if (0..5).any(|k| {
                        let row = (0..5).all(|j| cards[i].get(Vec2::new(k, j)) == Some(&true));
                        let col = (0..5).all(|j| cards[i].get(Vec2::new(j, k)) == Some(&true));
                        row || col
                    }) {
                        return Ok(card
                            .coordinates()
                            .filter(|&pos| cards[i].get(pos) == Some(&false))
                            .map(|pos| card.get(pos).unwrap())
                            .sum::<u32>()
                            * number);
                    }
                }
            }
        }

        Err(AoCError::Message("No solution found".to_string()))
    }

    fn part2(self) -> Result<u32> {
        let mut cards = iter::repeat(Grid::<bool>::new(5, 5))
            .take(self.cards.len())
            .collect::<Vec<_>>();
        let mut unsolved = (0..self.cards.len()).collect::<HashSet<_>>();
        for number in &self.numbers {
            for i in unsolved.clone() {
                let card = &self.cards[i];
                if let Some(pos) = card
                    .coordinates()
                    .find(|&pos| card.get(pos) == Some(number))
                {
                    cards[i].set(pos, true);

                    if (0..5).any(|k| {
                        let row = (0..5).all(|j| cards[i].get(Vec2::new(k, j)) == Some(&true));
                        let col = (0..5).all(|j| cards[i].get(Vec2::new(j, k)) == Some(&true));
                        row || col
                    }) {
                        unsolved.remove(&i);
                        if unsolved.is_empty() {
                            return Ok(card
                                .coordinates()
                                .filter(|&pos| cards[i].get(pos) == Some(&false))
                                .map(|pos| card.get(pos).unwrap())
                                .sum::<u32>()
                                * number);
                        }
                    }
                }
            }
        }
        Err(AoCError::Message("No solution found".to_string()))
    }
}

aoc_main!(Day04);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day04, 1, EXAMPLE, 4512);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day04, 2, EXAMPLE, 1924);
    }
}
