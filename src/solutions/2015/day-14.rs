use aoc_rust::*;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, u32 as number},
    multi::separated_list1,
    Parser,
};

struct Day14 {
    reindeer: Vec<Reindeer>,
}

#[derive(Debug)]
struct Reindeer {
    #[allow(unused)]
    name: String,
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Reindeer {
    fn parse(input: &str) -> ParseResult<Self> {
        let (input, name) = alpha1.parse(input)?;
        let (input, _) = tag(" can fly ").parse(input)?;
        let (input, speed) = number.parse(input)?;
        let (input, _) = tag(" km/s for ").parse(input)?;
        let (input, fly_time) = number.parse(input)?;
        let (input, _) = tag(" seconds, but then must rest for ").parse(input)?;
        let (input, rest_time) = number.parse(input)?;
        let (input, _) = tag(" seconds.").parse(input)?;
        Ok((
            input,
            Reindeer {
                name: name.to_string(),
                speed,
                fly_time,
                rest_time,
            },
        ))
    }

    fn distance_at(&self, time: u32) -> u32 {
        let cycle_time = self.fly_time + self.rest_time;
        let cycles = time / cycle_time;
        let remaining_time = time % cycle_time;
        let remaining_distance = remaining_time.min(self.fly_time) * self.speed;
        (cycles * self.fly_time * self.speed) + remaining_distance
    }
}

impl Problem<u32, u32> for Day14 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, Reindeer::parse)
            .map(|reindeer| Day14 { reindeer })
            .parse(input)
    }

    fn part1(self) -> Result<u32> {
        Ok(self
            .reindeer
            .iter()
            .map(|r| r.distance_at(2503))
            .max()
            .unwrap())
    }

    fn part2(self) -> Result<u32> {
        let mut scores = vec![0; self.reindeer.len()];
        for time in 1..=2503 {
            let distances: Vec<_> = self.reindeer.iter().map(|r| r.distance_at(time)).collect();
            let max_distance = distances.iter().max().unwrap();
            for (i, distance) in distances.iter().enumerate() {
                if distance == max_distance {
                    scores[i] += 1;
                }
            }
        }
        Ok(scores.into_iter().max().unwrap())
    }
}

aoc_main!(Day14);
