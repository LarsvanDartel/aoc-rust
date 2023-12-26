use std::collections::{HashMap, HashSet};

use aoc_rust::*;

struct Day03 {
    grid: Vec<Vec<char>>,
}

impl Day03 {
    fn is_symbol(symbol: char) -> bool {
        !symbol.is_digit(10) && symbol != '.'
    }

    fn numbers(&self) -> Vec<u32> {
        let mut numbers = Vec::new();
        let mut current_number = None;
        let mut included = false;
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col].is_digit(10) {
                    if let Some(number) = current_number {
                        current_number =
                            Some(number * 10 + self.grid[row][col].to_digit(10).unwrap());
                    } else {
                        current_number = Some(self.grid[row][col].to_digit(10).unwrap());
                    }

                    for dy in -1..=1 {
                        let y = row as isize + dy;

                        if y < 0 || y >= self.grid.len() as isize {
                            continue;
                        }
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let x = col as isize + dx;

                            if x < 0 || x >= self.grid[y as usize].len() as isize {
                                continue;
                            }
                            if Self::is_symbol(self.grid[y as usize][x as usize]) {
                                included = true;
                                break;
                            }
                        }
                    }
                } else {
                    if let Some(number) = current_number {
                        if included {
                            numbers.push(number);
                        }
                        current_number = None;
                        included = false;
                    }
                }
            }
            if let Some(number) = current_number {
                if included {
                    numbers.push(number);
                }
                current_number = None;
                included = false;
            }
        }

        if let Some(number) = current_number {
            if included {
                numbers.push(number);
            }
        }

        numbers
    }

    fn gear_ratios(&self) -> Vec<u32> {
        let mut gear_options = HashMap::<(usize, usize), u32>::new();
        let mut gear_ratios = Vec::new();

        let mut current_number = None;
        let mut current_gears = HashSet::new();
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col].is_digit(10) {
                    if let Some(number) = current_number {
                        current_number =
                            Some(number * 10 + self.grid[row][col].to_digit(10).unwrap());
                    } else {
                        current_number = Some(self.grid[row][col].to_digit(10).unwrap());
                    }

                    for dy in -1..=1 {
                        let y = row as isize + dy;

                        if y < 0 || y >= self.grid.len() as isize {
                            continue;
                        }
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let x = col as isize + dx;

                            if x < 0 || x >= self.grid[y as usize].len() as isize {
                                continue;
                            }
                            if self.grid[y as usize][x as usize] == '*' {
                                current_gears.insert((x as usize, y as usize));
                            }
                        }
                    }
                } else {
                    if let Some(number) = current_number {
                        for gear in &current_gears {
                            if let Some(gear_number) = gear_options.get(gear) {
                                gear_ratios.push(number * gear_number);
                            } else {
                                gear_options.insert(*gear, number);
                            }
                        }
                        current_number = None;
                        current_gears.clear();
                    }
                }
            }
            if let Some(number) = current_number {
                for gear in &current_gears {
                    if let Some(gear_number) = gear_options.get(gear) {
                        gear_ratios.push(number * gear_number);
                    } else {
                        gear_options.insert(*gear, number);
                    }
                }
                current_number = None;
                current_gears.clear();
            }
        }

        gear_ratios
    }
}

impl Problem<u32, u32> for Day03 {
    fn parse(input: &str) -> ParseResult<Self> {
        let grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>();

        Ok(("", Self { grid }))
    }

    fn part1(&self) -> Result<u32> {
        Ok(self.numbers().iter().sum())
    }

    fn part2(&self) -> Result<u32> {
        Ok(self.gear_ratios().iter().sum())
    }
}

aoc_main!(Day03);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_part1() {
        assert_task!(Day03, 1, EXAMPLE, 4361)
    }

    #[test]
    fn test_part2() {
        assert_task!(Day03, 2, EXAMPLE, 467835)
    }
}
