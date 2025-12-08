use aoc_rust::*;
use common::*;

struct Day08 {
    junction_boxes: Vec<Vec3<i64>>,
}

impl Problem<usize, i64> for Day08 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Ok(Day08 {
            junction_boxes: list(
                list(dec_int, ',').map(|p| Vec3::new(p[0], p[1], p[2])),
                line_ending,
            )
            .parse_next(input)?,
        })
    }

    fn part1(self) -> Result<usize> {
        let n = self.junction_boxes.len();
        let edges = self
            .junction_boxes
            .iter()
            .enumerate()
            .tuple_combinations()
            .map(|((ai, a), (bi, b))| ((*a - *b).len2(), ai, bi))
            .sorted_by_key(|(x, _, _)| *x)
            .map(|(_, a, b)| (a, b));

        let mut uf = UnionFind::new(n);
        #[cfg(test)]
        let n_edges = 10;
        #[cfg(not(test))]
        let n_edges = 1000;

        for (a, b) in edges.take(n_edges) {
            uf.union(a, b);
        }

        Ok((0..n)
            .filter_map(|x| {
                if uf.find(x) == x {
                    Some(uf.size(x))
                } else {
                    None
                }
            })
            .sorted_unstable()
            .rev()
            .take(3)
            .product())
    }

    fn part2(self) -> Result<i64> {
        let n = self.junction_boxes.len();
        let edges = self
            .junction_boxes
            .iter()
            .enumerate()
            .tuple_combinations()
            .map(|((ai, a), (bi, b))| ((*a - *b).len2(), ai, bi))
            .sorted_unstable_by_key(|(x, _, _)| *x)
            .map(|(_, a, b)| (a, b));

        let mut uf = UnionFind::new(n);
        let mut cnt = n;

        for (a, b) in edges {
            if uf.union(a, b) {
                cnt -= 1;
                if cnt == 1 {
                    return Ok(self.junction_boxes[a].x * self.junction_boxes[b].x);
                }
            }
        }

        Err(AoCError::NoSolution)
    }
}

aoc_main!(Day08);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day08, 1, EXAMPLE, 40);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day08, 2, EXAMPLE, 25272);
    }
}
