use aoc_rust::*;
use common::*;
use hashbrown::{HashMap, HashSet};

struct Day05 {
    rules: HashMap<u32, HashSet<u32>>,
    sorted_updates: Vec<Vec<u32>>,
    unsorted_updates: Vec<Vec<u32>>,
}

impl Problem<u32, u32> for Day05 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated_pair(
            list(separated_pair(dec_u32, "|", dec_u32), line_ending).map(|orderings| {
                let mut rules = HashMap::new();
                for (a, b) in orderings {
                    rules.entry(b).or_insert_with(HashSet::new).insert(a);
                }
                rules
            }),
            (line_ending, line_ending),
            list(list(dec_u32, ','), line_ending),
        )
        .map(|(mut rules, updates)| {
            for update in &updates {
                for x in update {
                    rules.entry(*x).or_insert_with(HashSet::new);
                }
            }
            let (sorted_updates, unsorted_updates): (Vec<_>, Vec<_>) = updates
                .into_iter()
                .partition(|update| update.is_sorted_by(|a, b| rules[b].contains(a)));

            Day05 {
                rules,
                sorted_updates,
                unsorted_updates,
            }
        })
        .parse_next(input)
    }

    fn part1(self) -> Result<u32> {
        Ok(self
            .sorted_updates
            .iter()
            .map(|update| update[update.len() / 2])
            .sum())
    }

    fn part2(mut self) -> Result<u32> {
        Ok(self
            .unsorted_updates
            .iter_mut()
            .map(|update| {
                update.sort_by(|a, b| self.rules[b].contains(a).cmp(&true));
                update[update.len() / 2]
            })
            .sum())
    }
}

aoc_main!(Day05);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day05, 1, EXAMPLE, 143);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day05, 2, EXAMPLE, 123);
    }
}
