use aoc_rust::*;
use common::*;

struct Day18 {
    bytes: Vec<Vec2<isize>>,
}

impl Problem<usize, String> for Day18 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(
            separated_pair(dec_isize, ',', dec_isize).map(Into::into),
            line_ending,
        )
        .map(|bytes| Day18 { bytes })
        .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        #[cfg(not(test))]
        let size = 71;
        #[cfg(test)]
        let size = 7;
        #[cfg(not(test))]
        let to_drop = 1024;
        #[cfg(test)]
        let to_drop = 12;
        let mut g = Grid::new(size, size);

        for &pos in &self.bytes[..to_drop] {
            g.set(pos, true);
        }

        let grid = g.clone();
        let s = move |&pos: &Vec2<isize>| {
            Direction::cardinal()
                .map(|dir| pos + dir)
                .filter(|&p| grid.contains(p) && !grid[p])
                .collect::<Vec<_>>()
        };

        let start = Vec2::new(0, 0);
        let end = Vec2::new(size as isize - 1, size as isize - 1);

        let path = bfs(&start, s, |&p| p == end).ok_or("Could not find path")?;

        Ok(path.len() - 1)
    }

    fn part2(self) -> Result<String> {
        #[cfg(not(test))]
        let size = 71;
        #[cfg(test)]
        let size = 7;
        let mut parent = Grid::<()>::new(size + 2, size + 2).map(|pos, _| {
            if pos.x == 0 || pos.y == size as isize + 1 && pos.x != size as isize + 1 {
                Vec2::new(0, 0)
            } else if pos.x == size as isize + 1 || pos.y == 0 {
                Vec2::new(size as isize + 1, size as isize + 1)
            } else {
                Vec2::new(-1, -1)
            }
        });

        fn find_parent(parent: &mut Grid<Vec2<isize>>, pos: Vec2<isize>) -> Vec2<isize> {
            if pos == Vec2::new(-1, -1) {
                return pos;
            }
            if parent[pos] == pos {
                pos
            } else {
                let p = find_parent(parent, parent[pos]);
                parent[pos] = p;
                p
            }
        }

        fn connect(parent: &mut Grid<Vec2<isize>>, a: Vec2<isize>, b: Vec2<isize>) -> bool {
            let pa = find_parent(parent, a);
            let pb = find_parent(parent, b);

            if pa == pb {
                return true;
            }

            if pb == Vec2::new(-1, -1) {
                return true;
            }

            // If at least one of the sets is not the border, we can connect them
            if pb.x != 0
                && pb.x != parent.width as isize - 1
                && pb.y != 0
                && pb.y != parent.height as isize - 1
            {
                parent[pb] = pa;
                return true;
            } else if pa.x != 0
                && pa.x != parent.width as isize - 1
                && pa.y != 0
                && pa.y != parent.height as isize - 1
            {
                parent[pa] = pb;
                return true;
            }

            false
        }

        for &pos in self.bytes.iter() {
            let pos = pos + Vec2::new(1, 1);
            parent[pos] = pos;
            for dir in Direction::all() {
                if !connect(&mut parent, pos, pos + dir) {
                    return Ok(format!("{},{}", pos.x - 1, pos.y - 1));
                }
            }
        }

        Err(AoCError::NoSolution)
    }
}

aoc_main!(Day18);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day18, 1, EXAMPLE, 22);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day18, 2, EXAMPLE, "6,1");
    }
}
