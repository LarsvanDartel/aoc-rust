use aoc_rust::*;

use itertools::Itertools;
use nom::{
    character::complete::{line_ending, u32 as number},
    multi::separated_list0,
    Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Package(u32);

impl Package {
    fn parse(input: &str) -> ParseResult<Self> {
        number.map(Self).parse(input)
    }
}

struct Day24 {
    packages: Vec<Package>,
}

fn quantum_entanglement(packages: &[&Package]) -> u64 {
    packages.iter().map(|p| p.0 as u64).product::<u64>()
}

fn find_quantum_entanglement(packages: &[Package], group_count: u32, group_sum: u32) -> u64 {
    let mut groups = Vec::new();

    for group_size in 1..packages.len() {
        for group in packages.iter().combinations(group_size) {
            if group.iter().map(|p| p.0).sum::<u32>() == group_sum {
                groups.push(group.clone());
            }
        }

        if !groups.is_empty() {
            break;
        }
    }

    // Sort the groups by quantum entanglement.
    groups.sort_by_key(|g| quantum_entanglement(g));

    for group in groups {
        let remaining = packages
            .iter()
            .filter(|p| !group.contains(p))
            .copied()
            .collect_vec();

        if group_count == 2 {
            return quantum_entanglement(&group);
        } else {
            let qe = find_quantum_entanglement(&remaining, group_count - 1, group_sum);
            if qe != 0 {
                return quantum_entanglement(&group);
            }
        }
    }

    0
}

impl Problem<u64, u64> for Day24 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list0(line_ending, Package::parse)
            .map(|packages| Self { packages })
            .parse(input)
    }

    fn part1(self) -> Result<u64> {
        let group_sum = self.packages.iter().map(|p| p.0).sum::<u32>() / 3;
        Ok(find_quantum_entanglement(&self.packages, 3, group_sum))
    }

    fn part2(self) -> Result<u64> {
        let group_sum = self.packages.iter().map(|p| p.0).sum::<u32>() / 4;
        Ok(find_quantum_entanglement(&self.packages, 4, group_sum))
    }
}

aoc_main!(Day24);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"1
2
3
4
5
7
8
9
10
11
    "#;

    #[test]
    fn test_part1() {
        assert_task!(Day24, 1, EXAMPLE, 99);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day24, 2, EXAMPLE, 44);
    }
}
