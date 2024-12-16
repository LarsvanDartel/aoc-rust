use aoc_rust::*;
use common::*;

struct Day13 {
    grids: Vec<Grid>,
}

struct Grid {
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl Grid {
    fn new(grid: Vec<Vec<bool>>) -> Self {
        let rows = grid
            .iter()
            .map(|row| row.iter().fold(0, |acc, &c| (acc << 1) | (c as usize)))
            .collect::<Vec<_>>();

        let cols = (0..grid[0].len())
            .map(|i| {
                grid.iter()
                    .fold(0, |acc, row| (acc << 1) | (row[i] as usize))
            })
            .collect::<Vec<_>>();

        Self { rows, cols }
    }

    fn mirror_line(&self, smudge: bool) -> Option<usize> {
        if let Some(i) = Self::find_mirror_line(&self.rows, smudge) {
            Some(100 * i)
        } else {
            Self::find_mirror_line(&self.cols, smudge)
        }
    }

    fn find_mirror_line(vec: &[usize], smudge: bool) -> Option<usize> {
        for i in 1..vec.len() {
            let mut smudge = smudge;
            let mut a = i as isize - 1;
            let mut b = i as isize;

            while a >= 0 && b < vec.len() as isize {
                if vec[a as usize] != vec[b as usize] {
                    if smudge && (vec[a as usize] ^ vec[b as usize]).count_ones() == 1 {
                        smudge = false;
                    } else {
                        break;
                    }
                }
                a -= 1;
                b += 1;
            }

            if (a < 0 || b >= vec.len() as isize) && !smudge {
                return Some(i);
            }
        }

        None
    }
}

impl Problem<usize, usize> for Day13 {
    fn parse(input: &mut &str) -> PResult<Self> {
        let grid = list(many(one_of(['.', '#']).map(|c| c == '#')), line_ending).map(Grid::new);

        list(grid, (line_ending, line_ending))
            .map(|grids| Self { grids })
            .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self
            .grids
            .iter()
            .map(|grid| grid.mirror_line(false).unwrap())
            .sum())
    }

    fn part2(self) -> Result<usize> {
        Ok(self
            .grids
            .iter()
            .map(|grid| grid.mirror_line(true).unwrap())
            .sum())
    }
}

aoc_main!(Day13);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn test_part1() {
        assert_task!(Day13, 1, EXAMPLE, 405);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day13, 2, EXAMPLE, 400);
    }
}
