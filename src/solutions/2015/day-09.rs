use std::{cmp::Ordering, collections::HashMap};

use aoc_rust::*;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    Parser,
};

struct Day09 {
    dst: HashMap<String, HashMap<String, usize>>,
}

impl Day09 {
    fn find_distance(&self, ord: Ordering) -> usize {
        if ord == Ordering::Equal {
            return 0;
        }
        let nodes = self.dst.keys().collect::<Vec<_>>();
        let mut dist = match ord {
            Ordering::Less => usize::MAX,
            Ordering::Greater => usize::MIN,
            _ => unreachable!(),
        };
        for p in nodes.iter().permutations(nodes.len()) {
            let d = p
                .iter()
                .tuple_windows()
                .fold(0, |acc, (&&a, &&b)| acc + self.dst[a][b]);
            if d.cmp(&dist) == ord {
                dist = d;
            }
        }
        dist
    }
}

impl Problem<usize, usize> for Day09 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(
            line_ending,
            separated_pair(
                separated_pair(alpha1, tag(" to "), alpha1),
                tag(" = "),
                digit1.map(|s: &str| s.parse::<usize>().unwrap()),
            ),
        )
        .map(|distances| {
            let mut dst: HashMap<String, HashMap<String, usize>> = HashMap::new();
            for ((start, end), dist) in distances {
                dst.entry(start.to_string())
                    .or_default()
                    .insert(end.to_string(), dist);
                dst.entry(end.to_string())
                    .or_default()
                    .insert(start.to_string(), dist);
            }
            Self { dst }
        })
        .parse(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self.find_distance(Ordering::Less))
    }

    fn part2(self) -> Result<usize> {
        Ok(self.find_distance(Ordering::Greater))
    }
}

aoc_main!(Day09);
