use aoc_rust::*;
use common::*;

struct Day12 {
    nodes: HashMap<String, u8>,
    adj: HashMap<u8, Vec<u8>>,
}

impl Day12 {
    fn num_paths(
        &self,
        start: u8,
        end: u8,
        visited: &mut HashSet<u8>,
        path: &mut Vec<u8>,
        visit_multiple: bool,
    ) -> usize {
        if start == end {
            println!("{:?}", path);
            return 1;
        }
        if start & 0x80 == 0 {
            visited.insert(start);
        }
        let mut paths = 0;
        path.push(start);
        for &next in self.adj[&start].iter() {
            if visited.contains(&next) && visit_multiple {
                continue;
            }
            paths += self.num_paths(
                next,
                end,
                visited,
                path,
                visit_multiple || visited.contains(&next),
            );
        }
        path.pop();
        visited.remove(&start);
        paths
    }
}

impl Problem<usize, usize> for Day12 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(
            separated_pair(alpha1.map(String::from), '-', alpha1.map(String::from)),
            line_ending,
        )
        .map(|edges| {
            let mut nodes = HashMap::new();
            let mut node_idx = 0;
            let mut adj = HashMap::new();
            for (from, to) in edges {
                let from_big = from.chars().all(|c| c.is_ascii_uppercase());
                let to_big = to.chars().all(|c| c.is_ascii_uppercase());
                let from = *nodes.entry(from).or_insert_with(|| {
                    let idx = node_idx;
                    node_idx += 1;
                    idx | if from_big { 0x80 } else { 0 }
                });
                let to = *nodes.entry(to).or_insert_with(|| {
                    let idx = node_idx;
                    node_idx += 1;
                    idx | if to_big { 0x80 } else { 0 }
                });

                adj.entry(from).or_insert_with(Vec::new).push(to);
                adj.entry(to).or_insert_with(Vec::new).push(from);
            }
            Self { nodes, adj }
        })
        .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        let start = self.nodes["start"];
        let end = self.nodes["end"];
        for (k, v) in &self.nodes {
            println!("{}: {}", k, v);
        }
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        Ok(self.num_paths(start, end, &mut visited, &mut path, true))
    }

    fn part2(self) -> Result<usize> {
        let start = self.nodes["start"];
        let end = self.nodes["end"];
        for (k, v) in &self.nodes {
            println!("{}: {}", k, v);
        }
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        Ok(self.num_paths(start, end, &mut visited, &mut path, false))
    }
}

aoc_main!(Day12);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;

    const EXAMPLE2: &str = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"#;

    const EXAMPLE3: &str = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day12, 1, EXAMPLE1, 10);
        assert_task!(Day12, 1, EXAMPLE2, 19);
        assert_task!(Day12, 1, EXAMPLE3, 226);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day12, 2, EXAMPLE1, 36);
        assert_task!(Day12, 2, EXAMPLE2, 103);
        assert_task!(Day12, 2, EXAMPLE3, 3509);
    }
}
