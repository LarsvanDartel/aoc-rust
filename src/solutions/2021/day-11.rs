use aoc_rust::*;
use common::*;

struct Day11 {
    octopi: Grid<u8>,
}

impl Day11 {
    fn step(&mut self) -> usize {
        let mut new = self.octopi.map(|_, &v| v + 1);
        let mut flashes = 0;
        fn flash(new: &mut Grid<u8>, pos: Vec2<isize>, flashes: &mut usize) {
            new[pos] = 0;
            *flashes += 1;
            for d in Direction::all() {
                let npos = pos + d;
                if new.contains(npos) {
                    if new[npos] == 0 {
                        continue;
                    }
                    new[npos] += 1;
                    if new[npos] > 9 {
                        flash(new, npos, flashes);
                    }
                }
            }
        }

        for pos in self.octopi.coordinates() {
            if new[pos] > 9 {
                flash(&mut new, pos, &mut flashes);
            }
        }
        self.octopi = new;

        flashes
    }
}

impl Problem<usize, usize> for Day11 {
    fn parse(input: &mut &str) -> PResult<Self> {
        Grid::parse(anychar.verify(char::is_ascii_digit).map(|c| c as u8 - b'0'))
            .map(|octopi| Day11 { octopi })
            .parse_next(input)
    }

    fn part1(mut self) -> Result<usize> {
        Ok((0..100).map(|_| self.step()).sum())
    }

    fn part2(mut self) -> Result<usize> {
        Ok((1..)
            .find(|_| self.step() == self.octopi.width * self.octopi.height)
            .unwrap())
    }
}

aoc_main!(Day11);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day11, 1, EXAMPLE, 1656);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day11, 2, EXAMPLE, 195);
    }
}
