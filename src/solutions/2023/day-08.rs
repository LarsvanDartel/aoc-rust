use std::collections::HashMap;

use aoc_rust::*;
use num_integer::Integer;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    Parser,
};

struct Day08 {
    moves: Vec<Move>,
    network: HashMap<String, (String, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Left,
    Right,
}

impl Move {
    fn parse(input: &str) -> ParseResult<Move> {
        alt((tag("L").map(|_| Move::Left), tag("R").map(|_| Move::Right))).parse(input)
    }
}

impl Problem<usize, usize> for Day08 {
    fn parse(input: &str) -> ParseResult<Self> {
        let node = separated_pair(
            alpha1,
            tag(" = "),
            preceded(
                tag("("),
                terminated(separated_pair(alpha1, tag(", "), alpha1), tag(")")),
            ),
        );

        let nodes =
            separated_list1(line_ending, node).map(|nodes_repr: Vec<(&str, (&str, &str))>| {
                let mut network: HashMap<String, (String, String)> = HashMap::new();
                for (name, (left, right)) in nodes_repr {
                    network.insert(name.to_string(), (left.to_string(), right.to_string()));
                }
                network
            });

        separated_pair(many1(Move::parse), line_ending.and(line_ending), nodes)
            .map(|(moves, network)| Self { moves, network })
            .parse(input)
    }

    fn part1(self) -> Result<usize> {
        let mut root = "AAA".to_string();
        let mut i = 0;
        while root != "ZZZ" {
            let (left, right) = self.network.get(&root).unwrap();
            if self.moves[i % self.moves.len()] == Move::Left {
                root = left.to_string();
            } else {
                root = right.to_string();
            }
            i += 1;
        }
        Ok(i)
    }

    fn part2(self) -> Result<usize> {
        Ok(self
            .network
            .iter()
            .filter(|(name, _)| name.ends_with("A"))
            .map(|(name, _)| {
                let mut root = name.clone();
                let mut i = 0;
                for m in self.moves.iter().cycle() {
                    let (left, right) = self.network.get(&root).unwrap();
                    if *m == Move::Left {
                        root = left.to_string();
                    } else {
                        root = right.to_string();
                    }
                    i += 1;
                    if root.ends_with("Z") {
                        break;
                    }
                }
                i
            })
            .fold(1, |a, b| a.lcm(&b)))
    }
}

aoc_main!(Day08);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    const EXAMPLE_2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    const EXAMPLE_3: &str = r#"LR

AAA = (AAB, XXX)
AAB = (XXX, AAZ)
AAZ = (AAB, XXX)
BBA = (BBB, XXX)
BBB = (BBC, BBC)
BBC = (BBZ, BBZ)
BBZ = (BBB, BBB)
XXX = (XXX, XXX)"#;

    #[test]
    fn test_part1() {
        assert_task!(Day08, 1, EXAMPLE_1, 2);
        assert_task!(Day08, 1, EXAMPLE_2, 6);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day08, 2, EXAMPLE_3, 6)
    }
}
