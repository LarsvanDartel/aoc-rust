use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1, u64 as parse_u64},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    Parser,
};

use aoc_rust::*;

struct Day05 {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<(u64, u64, u64)>,
}

impl Map {
    fn parse(input: &str) -> ParseResult<Self> {
        let range = tuple((parse_u64, space1, parse_u64, space1, parse_u64))
            .map(|(dst_start, _, src_start, _, length)| (src_start, dst_start, length));

        separated_pair(
            terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:")),
            line_ending,
            separated_list1(line_ending, range),
        )
        .map(|(_, ranges)| Self { ranges })
        .parse(input)
    }

    fn map(&self, val: u64) -> u64 {
        for range in &self.ranges {
            if val >= range.0 && val < range.0 + range.2 {
                return range.1 + val - range.0;
            }
        }
        val
    }

    fn map_range(&self, range: (u64, u64)) -> Vec<(u64, u64)> {
        let mut unmapped = vec![range];
        let mut mapped = vec![];

        for map in &self.ranges {
            let mut m = Vec::new();
            for (start, end) in unmapped {
                let a = (start, end.min(map.0));
                let b = (start.max(map.0), (map.0 + map.2).min(end));
                let c = ((map.0 + map.2).max(start), end);
                if a.0 < a.1 {
                    m.push(a);
                }
                if b.0 < b.1 {
                    mapped.push((b.0 - map.0 + map.1, b.1 - map.0 + map.1));
                }
                if c.0 < c.1 {
                    m.push(c);
                }
            }
            unmapped = m;
        }
        mapped.extend(unmapped);
        mapped
    }
}

impl Problem<u64, u64> for Day05 {
    fn parse(input: &str) -> ParseResult<Self> {
        let seeds = preceded(tag("seeds: "), separated_list1(tag(" "), parse_u64));

        separated_pair(
            seeds,
            line_ending.and(line_ending),
            separated_list1(line_ending.and(line_ending), Map::parse),
        )
        .map(|(seeds, maps)| Self { seeds, maps })
        .parse(input)
    }

    fn part1(self) -> Result<u64> {
        Ok(self
            .seeds
            .iter()
            .map(|seed| self.maps.iter().fold(*seed, |val, map| map.map(val)))
            .min()
            .unwrap())
    }

    fn part2(self) -> Result<u64> {
        let mut seeds = vec![];
        let mut s = None;
        for s2 in &self.seeds {
            if let Some(&s1) = s {
                seeds.push((s1, s1 + s2));
                s = None;
            } else {
                s = Some(s2);
            }
        }

        for layer in &self.maps {
            let mut next = Vec::new();
            for r in seeds {
                next.extend(layer.map_range(r));
            }
            seeds = next;
        }

        Ok(seeds.iter().map(|&(s, _)| s).min().unwrap())
    }
}

aoc_main!(Day05);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_part1() {
        assert_task!(Day05, 1, EXAMPLE, 35)
    }

    #[test]
    fn test_part2() {
        assert_task!(Day05, 2, EXAMPLE, 46)
    }
}
