use aoc_rust::*;
use common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Empty,
    Box,
    BoxLeft,
    BoxRight,
    Robot,
}

impl Cell {
    fn parse(input: &mut &str) -> PResult<Self> {
        one_of([' ', '#', '.', 'O', '@'])
            .map(|c| match c {
                '#' => Cell::Wall,
                '.' => Cell::Empty,
                'O' => Cell::Box,
                '@' => Cell::Robot,
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
            Cell::Box => 'O',
            Cell::BoxLeft => '[',
            Cell::BoxRight => ']',
            Cell::Robot => '@',
        };
        write!(f, "{}", c)
    }
}

struct Day15 {
    grid: Grid<Cell>,
    moves: Vec<Direction>,
}

impl Day15 {
    fn apply_moves(&mut self) {
        let mut robot = self.grid.find(Cell::Robot).unwrap();
        for &m in &self.moves {
            let mut to_move = vec![HashSet::<Vec2<isize>>::from([robot])];
            let mut found_wall = false;

            while !found_wall {
                let mut next = HashSet::new();
                for &pos in &to_move[to_move.len() - 1] {
                    let next_pos = pos + m;
                    match self.grid[next_pos] {
                        Cell::Empty => continue,
                        Cell::Wall => {
                            found_wall = true;
                            break;
                        },
                        _ => next.insert(next_pos),
                    };
                    if self.grid[next_pos] == Cell::BoxLeft && m.is_vertical() {
                        next.insert(next_pos + Direction::East);
                    }
                    if self.grid[next_pos] == Cell::BoxRight && m.is_vertical() {
                        next.insert(next_pos + Direction::West);
                    }
                }

                if next.is_empty() {
                    break;
                }
                to_move.push(next);
            }

            if found_wall {
                continue;
            }

            for step in to_move.into_iter().rev() {
                for pos in step {
                    self.grid[pos + m] = self.grid[pos];
                    self.grid[pos] = Cell::Empty;
                }
            }
            robot += m;

            // println!("{:?}", m);
            // println!("{}", self.grid);
            // std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    fn sum_coordinates(&self) -> usize {
        self.grid
            .coordinates()
            .filter_map(|pos| match self.grid[pos] {
                Cell::Box | Cell::BoxLeft => Some(100 * pos.y as usize + pos.x as usize),
                _ => None,
            })
            .sum()
    }
}

impl Problem<usize, usize> for Day15 {
    fn parse_1(input: &mut &str) -> PResult<Self> {
        separated_pair(
            Grid::parse(Cell::parse),
            line_ending,
            many(alt((
                Direction::parse_arrows.map(Some),
                line_ending.map(|_| None),
            ))),
        )
        .map(|(grid, moves)| Self {
            grid,
            moves: moves.into_iter().flatten().collect(),
        })
        .parse_next(input)
    }

    fn parse_2(input: &mut &str) -> PResult<Self> {
        separated_pair(
            Grid::parse(Cell::parse),
            line_ending,
            many(alt((
                Direction::parse_arrows.map(Some),
                line_ending.map(|_| None),
            ))),
        )
        .map(|(grid, moves)| Self {
            grid: grid.flat_map(|cell| match cell {
                Cell::Box => [Cell::BoxLeft, Cell::BoxRight],
                Cell::Empty => [Cell::Empty, Cell::Empty],
                Cell::Wall => [Cell::Wall, Cell::Wall],
                Cell::Robot => [Cell::Robot, Cell::Empty],
                _ => unreachable!(),
            }),
            moves: moves.into_iter().flatten().collect(),
        })
        .parse_next(input)
    }

    fn part1(mut self) -> Result<usize> {
        self.apply_moves();
        Ok(self.sum_coordinates())
    }

    fn part2(mut self) -> Result<usize> {
        self.apply_moves();
        Ok(self.sum_coordinates())
    }
}

aoc_main!(Day15);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day15, 1, EXAMPLE, 10092);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day15, 2, EXAMPLE, 9021);
    }
}
