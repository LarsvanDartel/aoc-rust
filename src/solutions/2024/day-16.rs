use std::collections::VecDeque;

use aoc_rust::*;
use common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Empty,
    Start,
    End,
}

impl Cell {
    fn parse(input: &mut &str) -> PResult<Self> {
        one_of(['.', '#', 'S', 'E'])
            .map(|c| match c {
                '.' => Cell::Empty,
                '#' => Cell::Wall,
                'S' => Cell::Start,
                'E' => Cell::End,
                _ => unreachable!(),
            })
            .parse_next(input)
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Wall => '#',
            Cell::Empty => '.',
            Cell::Start => 'S',
            Cell::End => 'E',
        };
        write!(f, "{}", c)
    }
}

struct Day16 {
    maze: Grid<Cell>,
}

impl Problem<usize, usize> for Day16 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Grid::parse(Cell::parse)
            .map(|maze| Day16 { maze })
            .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        let start = self.maze.find(Cell::Start).ok_or("Could not find start")?;
        let end = self.maze.find(Cell::End).ok_or("Could not find end")?;

        let s = move |&(pos, dir): &(Vec2<isize>, Direction)| {
            let mut successors = vec![((pos, dir.right()), 1000), ((pos, dir.left()), 1000)];
            if self.maze.contains(pos + dir) && self.maze[pos + dir] != Cell::Wall {
                successors.push(((pos + dir, dir), 1));
            }
            successors
        };

        let (_, len) = dijkstra(&(start, Direction::East), s, |&(p, _)| p == end)
            .ok_or("Could not find path")?;

        Ok(len)
    }

    fn part2(self) -> Result<usize> {
        let start = self.maze.find(Cell::Start).ok_or("Could not find start")?;
        let end = self.maze.find(Cell::End).ok_or("Could not find end")?;

        let s = move |&(pos, dir): &(Vec2<isize>, Direction)| {
            let mut successors = vec![((pos, dir.right()), 1000), ((pos, dir.left()), 1000)];
            if self.maze.contains(pos + dir) && self.maze[pos + dir] != Cell::Wall {
                successors.push(((pos + dir, dir), 1));
            }
            successors
        };

        let map = dijkstra_all(&(start, Direction::East), s);
        let end_state = Direction::cardinal()
            .map(move |dir| (end, dir))
            .min_by_key(|state| map.get(state).unwrap().1)
            .unwrap();

        let mut visited = HashSet::new();
        let mut q = VecDeque::from([(end_state, map[&end_state].1)]);

        while let Some(((pos, dir), dst)) = q.pop_front() {
            if visited.contains(&(pos, dir)) {
                continue;
            }
            visited.insert((pos, dir));
            if let Some(&(_, prev)) = map.get(&(pos - dir, dir)) {
                if prev + 1 == dst {
                    q.push_back(((pos - dir, dir), prev));
                }
            }
            if let Some(&(_, prev)) = map.get(&(pos, dir.right())) {
                if prev + 1000 == dst {
                    q.push_back(((pos, dir.right()), prev));
                }
            }
            if let Some(&(_, prev)) = map.get(&(pos, dir.left())) {
                if prev + 1000 == dst {
                    q.push_back(((pos, dir.left()), prev));
                }
            }
        }

        Ok(visited
            .into_iter()
            .map(|(pos, _)| pos)
            .collect::<HashSet<_>>()
            .len())
    }
}

aoc_main!(Day16);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#;

    const EXAMPLE2: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day16, 1, EXAMPLE1, 7036);
        assert_task!(Day16, 1, EXAMPLE2, 11048);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day16, 2, EXAMPLE1, 45);
        assert_task!(Day16, 2, EXAMPLE2, 64);
    }
}
