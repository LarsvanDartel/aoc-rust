use aoc_rust::*;

struct Day11 {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Day11 {
    fn new(grid: Vec<Vec<bool>>) -> Self {
        let empty_rows = (0..grid.len())
            .filter(|&i| (0..grid[0].len()).all(|j| !grid[i][j]))
            .collect::<Vec<_>>();
        let empty_rows = (0..grid.len())
            .map(|i| empty_rows.iter().filter(|&&j| j <= i).count())
            .collect::<Vec<_>>();

        let empty_cols = (0..grid[0].len())
            .filter(|&i| (0..grid.len()).all(|j| !grid[j][i]))
            .collect::<Vec<_>>();
        let empty_cols = (0..grid[0].len())
            .map(|i| empty_cols.iter().filter(|&&j| j <= i).count())
            .collect::<Vec<_>>();

        let galaxies = grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &c)| c)
                    .map(move |(j, _)| (i, j))
            })
            .collect();

        Self {
            galaxies,
            empty_rows,
            empty_cols,
        }
    }

    fn distance(&self, i: usize, j: usize, expansion: usize) -> usize {
        let (y1, x1) = self.galaxies[i];
        let (y2, x2) = self.galaxies[j];

        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

        let empty_rows_between = self.empty_rows[y2] - self.empty_rows[y1];
        let empty_cols_between = self.empty_cols[x2] - self.empty_cols[x1];

        (x1 as i32 - x2 as i32).abs() as usize
            + (y1 as i32 - y2 as i32).abs() as usize
            + (expansion - 1) * (empty_rows_between + empty_cols_between)
    }

    fn distance_sum(&self, expansion: usize) -> usize {
        let mut sum = 0;
        for i in 0..self.galaxies.len() {
            for j in i + 1..self.galaxies.len() {
                sum += self.distance(i, j, expansion);
            }
        }
        sum
    }
}

impl Problem<usize, usize> for Day11 {
    fn parse(input: &str) -> ParseResult<Self> {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => panic!("Invalid input"),
                    })
                    .collect()
            })
            .collect();

        Ok(("", Self::new(grid)))
    }

    fn part1(self) -> Result<usize> {
        Ok(self.distance_sum(2))
    }

    fn part2(self) -> Result<usize> {
        Ok(self.distance_sum(1_000_000))
    }
}

aoc_main!(Day11);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn test_part1() {
        assert_task!(Day11, 1, EXAMPLE, 374);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day11, 2, EXAMPLE, 82000210);
    }
}
