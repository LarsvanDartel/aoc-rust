use aoc_rust::*;
use common::*;

#[derive(PartialEq, Eq)]
enum Taychon {
    Splitter,
    Empty,
    Start,
}

impl Taychon {
    fn parse(input: &mut &str) -> PResult<Self> {
        one_of(['.', '^', 'S'])
            .map(|c| match c {
                '.' => Self::Empty,
                '^' => Self::Splitter,
                'S' => Self::Start,
                _ => unreachable!(),
            })
            .parse_next(input)
    }
}

impl std::fmt::Display for Taychon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => ' ',
                Self::Splitter => '^',
                Self::Start => 'S',
            }
        )
    }
}

struct Day07 {
    manifold: Grid<Taychon>,
}

impl Problem<usize, usize> for Day07 {
    fn parse(input: &mut &str) -> PResult<Self> {
        seq!(Day07 {
            manifold: Grid::parse(Taychon::parse)
        })
        .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        let mut rows = (0..self.manifold.height).map(|i| self.manifold.get_row(i).unwrap());
        let head = rows
            .next()
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(i, t)| match t {
                Taychon::Start => Some(i),
                _ => None,
            })
            .collect::<HashSet<_>>();

        Ok(rows
            .fold((0, head), |(mut splits, head), row| {
                let mut new_head = HashSet::new();
                for c in head {
                    match row[c] {
                        Taychon::Splitter => {
                            splits += 1;
                            new_head.insert(c + 1);
                            new_head.insert(c - 1);
                        }
                        _ => {
                            new_head.insert(c);
                        }
                    }
                }
                (splits, new_head)
            })
            .0)
    }

    fn part2(self) -> Result<usize> {
        let mut rows = (0..self.manifold.height).map(|i| self.manifold.get_row(i).unwrap());
        let head = rows
            .next()
            .unwrap()
            .iter()
            .map(|t| match t {
                Taychon::Start => 1,
                _ => 0,
            })
            .collect_vec();

        Ok(rows
            .fold(head, |head, row| {
                let mut new_head = vec![0; head.len()];
                for (i, c) in head.into_iter().enumerate() {
                    match row[i] {
                        Taychon::Splitter => {
                            new_head[i + 1] += c;
                            new_head[i - 1] += c;
                        }
                        _ => {
                            new_head[i] += c;
                        }
                    }
                }
                new_head
            })
            .into_iter()
            .sum())
    }
}

aoc_main!(Day07);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day07, 1, EXAMPLE, 21);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day07, 2, EXAMPLE, 40);
    }
}
