use std::collections::VecDeque;

use aoc_rust::*;
use common::*;

struct Day12 {
    garden: Grid<u8>,
}

impl Day12 {
    fn plots(&self) -> Vec<HashSet<Vec2<isize>>> {
        let mut plots = Vec::new();
        let mut visited = Grid::<bool>::new(self.garden.width, self.garden.height);

        for pos in self.garden.coordinates() {
            if visited[pos] {
                continue;
            }
            let val = self.garden[pos];
            let mut queue = VecDeque::new();
            let mut plot = HashSet::new();
            queue.push_back(pos);
            while let Some(pos) = queue.pop_front() {
                if !self.garden.contains(pos) || plot.contains(&pos) || self.garden[pos] != val {
                    continue;
                }
                plot.insert(pos);
                visited[pos] = true;
                for dir in Direction::cardinal() {
                    queue.push_back(pos + dir);
                }
            }

            plots.push(plot);
        }
        plots
    }
}

impl Problem<usize, usize> for Day12 {
    fn parse(input: &str) -> ParseResult<Self> {
        Grid::parse(verify(anychar, char::is_ascii_alphabetic).map(|c| c as u8 - b'A'))
            .map(|garden| Self { garden })
            .parse(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self
            .plots()
            .into_iter()
            .map(|plot| {
                let area = plot.len();
                let perimeter = plot
                    .iter()
                    .flat_map(|&pos| Direction::cardinal().map(move |dir| pos + dir))
                    .filter(|&pos| !plot.contains(&pos))
                    .count();

                area * perimeter
            })
            .sum())
    }

    fn part2(self) -> Result<usize> {
        Ok(self
            .plots()
            .into_iter()
            .map(|plot| {
                let area = plot.len();
                let sides: usize = Direction::cardinal()
                    .map(|dir| {
                        let side = plot
                            .iter()
                            .map(|&pos| pos + dir)
                            .filter(|&pos| !plot.contains(&pos))
                            .collect::<HashSet<_>>();
                        let r = dir.right();
                        let offset_side = side.iter().map(|&pos| pos + r).collect::<HashSet<_>>();
                        (&side - &offset_side).len()
                    })
                    .sum();
                area * sides
            })
            .sum())
    }
}

aoc_main!(Day12);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day12, 1, EXAMPLE, 1930);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day12, 2, EXAMPLE, 1206);
    }
}
