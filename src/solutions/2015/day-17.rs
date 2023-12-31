use aoc_rust::*;

use nom::{
    character::complete::{line_ending, u32 as parse_u32},
    multi::separated_list1,
    Parser,
};

struct Day17 {
    containers: Vec<u32>,
}

impl Problem<u32, u32> for Day17 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, parse_u32)
            .map(|containers| Day17 { containers })
            .parse(input)
    }

    fn part1(self) -> Result<u32> {
        let mut combinations = 0;
        for i in 0..(1 << self.containers.len()) {
            let mut sum = 0;
            for j in 0..self.containers.len() {
                if i & (1 << j) != 0 {
                    sum += self.containers[j];
                }
            }
            if sum == 150 {
                combinations += 1;
            }
        }
        Ok(combinations)
    }

    fn part2(self) -> Result<u32> {
        let mut combinations = 0;
        let mut min_containers = usize::MAX;
        for i in 0..(1 << self.containers.len()) {
            let mut sum = 0;
            let mut containers = 0;
            for j in 0..self.containers.len() {
                if i & (1 << j) != 0 {
                    sum += self.containers[j];
                    containers += 1;
                }
            }
            if sum == 150 {
                match containers.cmp(&min_containers) {
                    std::cmp::Ordering::Less => {
                        min_containers = containers;
                        combinations = 1;
                    }
                    std::cmp::Ordering::Equal => {
                        combinations += 1;
                    }
                    _ => {}
                }
            }
        }
        Ok(combinations)
    }
}

aoc_main!(Day17);
