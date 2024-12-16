use aoc_rust::*;
use common::*;

struct Day18 {
    grid: Grid<bool>,
}

impl Day18 {
    fn step(&mut self) {
        let mut new_grid = Grid::new(self.grid.height, self.grid.width);
        for p in self.grid.coordinates() {
            let neigbors = Direction::all()
                .filter(|&d| self.grid.get(p + d) == Some(&true))
                .count();
            if self.grid[p] {
                new_grid[p] = neigbors == 2 || neigbors == 3;
            } else {
                new_grid[p] = neigbors == 3;
            }
        }

        self.grid = new_grid;
    }
}

impl Problem<usize, usize> for Day18 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Grid::parse(one_of(['.', '#']).map(|c| c == '#'))
            .map(|grid| Self { grid })
            .parse_next(input)
    }

    fn part1(mut self) -> Result<usize> {
        for _ in 0..100 {
            self.step();
        }
        Ok(self.grid.coordinates().filter(|&c| self.grid[c]).count())
    }

    fn part2(mut self) -> Result<usize> {
        let n = self.grid.height as isize - 1;
        let m = self.grid.width as isize - 1;
        let corners = [
            Vec2::new(0, 0),
            Vec2::new(0, m),
            Vec2::new(n, 0),
            Vec2::new(n, m),
        ];
        for _ in 0..100 {
            self.step();
            for &corner in &corners {
                self.grid[corner] = true;
            }
        }
        Ok(self.grid.coordinates().filter(|&c| self.grid[c]).count())
    }
}

aoc_main!(Day18);
