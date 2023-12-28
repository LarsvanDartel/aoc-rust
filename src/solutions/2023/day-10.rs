use std::collections::{HashSet, VecDeque};
use std::ops::Add;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    Parser,
};

use aoc_rust::*;

struct Day10 {
    grid: Vec<Vec<Pipe>>,
}

impl std::fmt::Debug for Day10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for pipe in row.iter() {
                write!(f, "{:?}", pipe)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Empty,
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Source,
}

impl Pipe {
    fn parse(input: &str) -> ParseResult<Pipe> {
        alt((
            tag(".").map(|_| Pipe::Empty),
            tag("-").map(|_| Pipe::Horizontal),
            tag("|").map(|_| Pipe::Vertical),
            tag("L").map(|_| Pipe::NorthEast),
            tag("J").map(|_| Pipe::NorthWest),
            tag("7").map(|_| Pipe::SouthWest),
            tag("F").map(|_| Pipe::SouthEast),
            tag("S").map(|_| Pipe::Source),
        ))
        .parse(input)
    }

    const fn increase_resolution(&self) -> [[Pipe; 3]; 3] {
        let mut grid = [[Pipe::Empty; 3]; 3];
        grid[1][1] = *self;
        match self {
            Pipe::Empty => {}
            Pipe::Horizontal => {
                grid[1][0] = Pipe::Horizontal;
                grid[1][2] = Pipe::Horizontal;
            }
            Pipe::Vertical => {
                grid[0][1] = Pipe::Vertical;
                grid[2][1] = Pipe::Vertical;
            }
            Pipe::NorthEast => {
                grid[0][1] = Pipe::Vertical;
                grid[1][2] = Pipe::Horizontal;
            }
            Pipe::NorthWest => {
                grid[0][1] = Pipe::Vertical;
                grid[1][0] = Pipe::Horizontal;
            }
            Pipe::SouthEast => {
                grid[2][1] = Pipe::Vertical;
                grid[1][2] = Pipe::Horizontal;
            }
            Pipe::SouthWest => {
                grid[2][1] = Pipe::Vertical;
                grid[1][0] = Pipe::Horizontal;
            }
            _ => unreachable!(),
        }
        grid
    }

    fn create_pipe(d1: Dir, d2: Dir) -> Pipe {
        match (d1, d2) {
            (Dir::North, Dir::East) => Pipe::NorthEast,
            (Dir::North, Dir::West) => Pipe::NorthWest,
            (Dir::East, Dir::North) => Pipe::NorthEast,
            (Dir::East, Dir::South) => Pipe::SouthEast,
            (Dir::South, Dir::East) => Pipe::SouthEast,
            (Dir::South, Dir::West) => Pipe::SouthWest,
            (Dir::West, Dir::North) => Pipe::NorthWest,
            (Dir::West, Dir::South) => Pipe::SouthWest,
            (Dir::North, Dir::North) => Pipe::Vertical,
            (Dir::East, Dir::East) => Pipe::Horizontal,
            (Dir::South, Dir::South) => Pipe::Vertical,
            (Dir::West, Dir::West) => Pipe::Horizontal,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pipe::Empty => ' ',
                Pipe::Horizontal => '─',
                Pipe::Vertical => '│',
                Pipe::NorthEast => '└',
                Pipe::NorthWest => '┘',
                Pipe::SouthEast => '┌',
                Pipe::SouthWest => '┐',
                Pipe::Source => '.',
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl From<(isize, isize)> for Dir {
    fn from((x, y): (isize, isize)) -> Self {
        match (x, y) {
            (0, -1) => Dir::North,
            (1, 0) => Dir::East,
            (0, 1) => Dir::South,
            (-1, 0) => Dir::West,
            _ => unreachable!(),
        }
    }
}

impl From<Dir> for (isize, isize) {
    fn from(dir: Dir) -> Self {
        match dir {
            Dir::North => (0, -1),
            Dir::East => (1, 0),
            Dir::South => (0, 1),
            Dir::West => (-1, 0),
        }
    }
}

impl Add<Dir> for (isize, isize) {
    type Output = (isize, isize);

    fn add(self, rhs: Dir) -> Self::Output {
        let rhs: (isize, isize) = rhs.into();
        (self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Dir {
    fn rotate(&self, pipe: Pipe) -> Option<Self> {
        match (self, pipe) {
            (Dir::North, Pipe::SouthEast) => Some(Dir::East),
            (Dir::North, Pipe::SouthWest) => Some(Dir::West),
            (Dir::East, Pipe::NorthWest) => Some(Dir::North),
            (Dir::East, Pipe::SouthWest) => Some(Dir::South),
            (Dir::South, Pipe::NorthEast) => Some(Dir::East),
            (Dir::South, Pipe::NorthWest) => Some(Dir::West),
            (Dir::West, Pipe::NorthEast) => Some(Dir::North),
            (Dir::West, Pipe::SouthEast) => Some(Dir::South),
            (Dir::North, Pipe::Vertical) => Some(Dir::North),
            (Dir::East, Pipe::Horizontal) => Some(Dir::East),
            (Dir::South, Pipe::Vertical) => Some(Dir::South),
            (Dir::West, Pipe::Horizontal) => Some(Dir::West),
            _ => None,
        }
    }

    const fn all() -> [Self; 4] {
        [Dir::North, Dir::East, Dir::South, Dir::West]
    }
}

impl Day10 {
    fn find_cycle(&self) -> Option<Vec<(usize, usize)>> {
        let start = self
            .grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, pipe)| {
                    if *pipe == Pipe::Source {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        'dir_loop: for start_dir in Dir::all().iter() {
            let mut dir = *start_dir;
            let mut pos = (start.0 as isize, start.1 as isize);
            let mut visited = vec![start];

            loop {
                pos = pos + dir;

                if pos.0 < 0
                    || pos.1 < 0
                    || pos.0 >= self.grid[0].len() as isize
                    || pos.1 >= self.grid.len() as isize
                {
                    continue 'dir_loop;
                }

                let pipe = self.grid[pos.1 as usize][pos.0 as usize];
                if pipe == Pipe::Source {
                    break;
                }

                if let Some(new_dir) = dir.rotate(pipe) {
                    dir = new_dir;
                } else {
                    continue 'dir_loop;
                }

                visited.push((pos.0 as usize, pos.1 as usize));
            }

            return Some(visited);
        }

        None
    }
}

impl Problem<usize, usize> for Day10 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, many1(Pipe::parse))
            .map(|grid| Self { grid })
            .parse(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self.find_cycle().unwrap().len() / 2)
    }

    fn part2(self) -> Result<usize> {
        let cycle = self.find_cycle().unwrap();

        // replace source in the cycle with correct pipe
        let mut grid = self.grid.clone();
        let start = cycle[0];
        let p1 = cycle[1];
        let p2 = cycle[cycle.len() - 1];

        let d1 = (
            p1.0 as isize - start.0 as isize,
            p1.1 as isize - start.1 as isize,
        );
        let d2 = (
            p2.0 as isize - start.0 as isize,
            p2.1 as isize - start.1 as isize,
        );

        let pipe = Pipe::create_pipe(d1.into(), d2.into());
        grid[start.1][start.0] = pipe;

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if !cycle.contains(&(x, y)) {
                    grid[y][x] = Pipe::Empty;
                }
            }
        }

        grid = grid
            .iter()
            .flat_map(|row| {
                [
                    row.iter()
                        .flat_map(|pipe| pipe.increase_resolution()[0])
                        .collect(),
                    row.iter()
                        .flat_map(|pipe| pipe.increase_resolution()[1])
                        .collect(),
                    row.iter()
                        .flat_map(|pipe| pipe.increase_resolution()[2])
                        .collect(),
                ]
            })
            .map(|row: Vec<Pipe>| {
                let mut row = row;
                row.insert(0, Pipe::Source);
                row.push(Pipe::Source);
                row
            })
            .collect();

        grid.insert(0, vec![Pipe::Source; grid[0].len()]);
        grid.push(vec![Pipe::Source; grid[0].len()]);

        let rows = grid.len();
        let cols = grid[0].len();

        let mut queue = VecDeque::new();
        let mut outside = HashSet::new();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == Pipe::Source {
                    queue.push_back((j as isize, i as isize));
                }
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            if x > 0 && y > 0 && x < cols as isize - 1 && y < rows as isize - 1 {
                outside.insert((((x - 1) / 3) as usize, ((y - 1) / 3) as usize));
            }

            for dir in Dir::all().iter() {
                let pos = (x, y) + *dir;

                if pos.0 < 0 || pos.1 < 0 || pos.0 >= cols as isize || pos.1 >= rows as isize {
                    continue;
                }

                if grid[pos.1 as usize][pos.0 as usize] == Pipe::Empty {
                    grid[pos.1 as usize][pos.0 as usize] = Pipe::Source;
                    queue.push_back(pos);
                }
            }
        }

        Ok(self.grid[0].len() * self.grid.len() - outside.len())
    }
}

aoc_main!(Day10);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

    const EXAMPLE_2: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;

    const EXAMPLE_3: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;

    const EXAMPLE_4: &str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

    #[test]
    fn test_part1() {
        assert_task!(Day10, 1, EXAMPLE_1, 4);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day10, 2, EXAMPLE_2, 4);
        assert_task!(Day10, 2, EXAMPLE_3, 8);
        assert_task!(Day10, 2, EXAMPLE_4, 10);
    }
}
