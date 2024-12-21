use aoc_rust::*;
use common::*;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Wall,
    Start,
    End,
}

impl Cell {
    fn parse(input: &mut &str) -> PResult<Self> {
        one_of(('.', '#', 'S', 'E'))
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
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Start => 'S',
            Cell::End => 'E',
        };
        write!(f, "{}", c)
    }
}

struct Day20 {
    racetrack: Grid<Cell>,
}

impl Day20 {
    fn time_saves(&self, max_cheat_time: usize) -> Result<usize> {
        let mut start_dst =
            Grid::new_default(self.racetrack.width, self.racetrack.height, usize::MAX);
        let mut end_dst =
            Grid::new_default(self.racetrack.width, self.racetrack.height, usize::MAX);
        let start = self
            .racetrack
            .find(&Cell::Start)
            .ok_or("Could not find start")?;
        let end = self
            .racetrack
            .find(&Cell::End)
            .ok_or("Could not find end")?;

        start_dst[start] = 0;
        end_dst[end] = 0;

        let mut q = VecDeque::new();
        q.push_back(start);
        q.push_back(end);

        let mut path = Vec::new();

        while let Some(pos) = q.pop_front() {
            for dir in Direction::cardinal() {
                let new_pos = pos + dir;
                if self.racetrack.contains(new_pos) && self.racetrack[new_pos] != Cell::Wall {
                    if start_dst[pos] < usize::MAX && start_dst[new_pos] > start_dst[pos] + 1 {
                        start_dst[new_pos] = start_dst[pos] + 1;
                        q.push_back(new_pos);
                        path.push(new_pos);
                    }

                    if end_dst[pos] < usize::MAX && end_dst[new_pos] > end_dst[pos] + 1 {
                        end_dst[new_pos] = end_dst[pos] + 1;
                        q.push_back(new_pos);
                    }
                }
            }
        }

        let dist = start_dst[end];
        let mut ans = 0;
        for (i, &p1) in path.iter().enumerate() {
            for &p2 in path.iter().skip(i + 1) {
                let d = p2 - p1;
                let d = (d.x.abs() + d.y.abs()) as usize;
                if d <= max_cheat_time {
                    let total_dist = start_dst[p1] + end_dst[p2] + d;
                    if (dist as isize - total_dist as isize) >= 100 {
                        ans += 1;
                    }
                }
            }
        }

        Ok(ans)
    }
}

impl Problem<usize, usize> for Day20 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Grid::parse(Cell::parse)
            .map(|racetrack| Day20 { racetrack })
            .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        self.time_saves(2)
    }

    fn part2(self) -> Result<usize> {
        self.time_saves(20)
    }
}

aoc_main!(Day20);
