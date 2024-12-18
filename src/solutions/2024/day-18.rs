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
        let mut g = Grid::new(size, size);

        for pos in self.bytes {
            g.set(pos, true);
            let grid = g.clone();
            let s = move |&pos: &Vec2<isize>| {
                Direction::cardinal()
                    .map(|dir| pos + dir)
                    .filter(|&p| grid.contains(p) && !grid[p])
                    .collect::<Vec<_>>()
            };

            let start = Vec2::new(0, 0);
            let end = Vec2::new(size as isize - 1, size as isize - 1);

            if bfs(&start, s, |&p| p == end).is_none() {
                return Ok(format!("{},{}", pos.x, pos.y));
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
