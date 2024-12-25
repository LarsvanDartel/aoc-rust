use aoc_rust::*;
use common::*;

struct Day25 {
    schematics: Vec<Grid<bool>>,
}

impl Problem<usize, String> for Day25 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(
            Grid::parse(one_of(('.', '#')).map(|c| c == '#')),
            (line_ending, line_ending),
        )
        .map(|schematics| Self { schematics })
        .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        let mut locks: Vec<Vec<usize>> = Vec::new();
        let mut keys: Vec<Vec<usize>> = Vec::new();
        for schematic in self.schematics.iter() {
            let lock = schematic[Vec2::new(0, 0)];
            let cols = (0..schematic.width)
                .map(|i| {
                    schematic
                        .get_column(i)
                        .unwrap()
                        .iter()
                        .filter(|b| **b ^ lock)
                        .count()
                })
                .collect();
            if lock {
                locks.push(cols);
            } else {
                keys.push(cols);
            }
        }

        Ok(locks
            .iter()
            .cartesian_product(keys.iter())
            .filter(|(l, k)| l.iter().zip(k.iter()).all(|(l, k)| k <= l))
            .count())
    }

    fn part2(self) -> Result<String> {
        Ok("Merry Christmas!".to_string())
    }
}

aoc_main!(Day25);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day25, 1, EXAMPLE, 3);
    }
}
