use aoc_rust::*;
use common::*;

use rustworkx_core::connectivity::stoer_wagner_min_cut;

struct Day25 {
    adjacency: Vec<(String, Vec<String>)>,
}

impl Problem<usize, String> for Day25 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(
            separated_pair(
                take_until(0.., ":").map(String::from),
                (":", space1),
                list(alpha1.map(String::from), space1),
            ),
            line_ending,
        )
        .map(|adjacency| Self { adjacency })
        .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        let mut graph = UnGraph::<String, ()>::new_undirected();

        let nodes = self
            .adjacency
            .iter()
            .flat_map(|(from, to)| std::iter::once(from).chain(to.iter()))
            .unique()
            .map(|s| (s.clone(), graph.add_node(s.clone())))
            .collect::<HashMap<_, _>>();

        for (from, to) in self.adjacency {
            for to in to {
                graph.add_edge(nodes[&from], nodes[&to], ());
            }
        }

        let cut: rustworkx_core::Result<Option<(i32, Vec<NodeIndex>)>> =
            stoer_wagner_min_cut(&graph, |_| Ok(1));

        let cut = cut.unwrap().unwrap();

        Ok(cut.1.len() * (nodes.len() - cut.1.len()))
    }

    fn part2(self) -> Result<String> {
        Ok(String::from("🎄 Merry Christmas! 🎄"))
    }
}

aoc_main!(Day25);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;

    #[test]
    fn test_part1() {
        assert_task!(Day25, 1, EXAMPLE, 54);
    }
}
