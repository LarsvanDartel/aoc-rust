use aoc_rust::*;
use common::*;

struct Day04 {
    grid: Grid<bool>,
}

impl Problem<usize, usize> for Day04 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Ok(Day04 {
            grid: Grid::parse(one_of(['.', '@']).map(|c| c == '@'))
                .parse_next(input)?
                .with_display_fn(|_, x| (if *x { "@" } else { " " }).to_string()),
        })
    }

    fn part1(self) -> Result<usize> {
        Ok(self
            .grid
            .coordinates()
            .filter(|c| {
                self.grid[*c]
                    && Direction::all()
                        .filter(|d| self.grid.contains(*c + *d) && self.grid[*c + *d])
                        .count()
                        < 4
            })
            .count())
    }

    fn part2(mut self) -> Result<usize> {
        let mut cnt = 0;
        loop {
            let to_remove = self
                .grid
                .coordinates()
                .filter(|c| {
                    self.grid[*c]
                        && Direction::all()
                            .filter(|d| self.grid.contains(*c + *d) && self.grid[*c + *d])
                            .count()
                            < 4
                })
                .collect::<Vec<_>>();
            if to_remove.is_empty() {
                break;
            }
            cnt += to_remove.len();
            for p in to_remove {
                self.grid[p] = false;
            }
        }
        Ok(cnt)
    }
}

aoc_main!(Day04);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day04, 1, EXAMPLE, 13);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day04, 2, EXAMPLE, 43);
    }
}
