use aoc_rust::*;
use common::*;
use rustworkx_core::petgraph::prelude::UnGraphMap;

struct Day23 {
    edges: Vec<(String, String)>,
}

impl Problem<usize, String> for Day23 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(
            separated_pair(alpha1.map(String::from), '-', alpha1.map(String::from)),
            line_ending,
        )
        .map(|edges| Day23 { edges })
        .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        let graph: UnGraphMap<&str, ()> =
            UnGraphMap::from_edges(self.edges.iter().map(|(a, b)| (a.as_str(), b.as_str())));

        let cliques = iter_cliques(&graph);
        Ok(cliques
            .filter(|c| c.len() == 3 && c.iter().any(|n| n.starts_with("t")))
            .count())
    }

    fn part2(self) -> Result<String> {
        let graph: UnGraphMap<&str, ()> =
            UnGraphMap::from_edges(self.edges.iter().map(|(a, b)| (a.as_str(), b.as_str())));

        let mut lan = iter_cliques(&graph).last().unwrap();
        lan.sort();
        Ok(lan.join(","))
    }
}

aoc_main!(Day23);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day23, 1, EXAMPLE, 7);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day23, 2, EXAMPLE, "co,de,ka,ta");
    }
}
