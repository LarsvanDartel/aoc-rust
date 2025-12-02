use aoc_rust::*;
use common::*;

struct Day21 {
    map: Vec<Vec<bool>>,
    start_pos: (usize, usize),
}

impl Problem<usize, usize> for Day21 {
    fn parse(input: &mut &str) -> PResult<Self> {
        let mut start_pos = (0, 0);
        let map = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => true,
                        '.' => false,
                        'S' => {
                            start_pos = (x, y);
                            false
                        }
                        _ => panic!("Invalid character"),
                    })
                    .collect()
            })
            .collect();

        Ok(Self { map, start_pos })
    }

    fn part1(self) -> Result<usize> {
        let mut reachable = HashSet::new();
        reachable.insert(self.start_pos);

        for _ in 0..64 {
            let mut new_reachable = HashSet::new();
            for pos in reachable.iter() {
                for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let new_pos = (pos.0 as isize + dx, pos.1 as isize + dy);
                    if new_pos.0 < 0
                        || new_pos.1 < 0
                        || new_pos.0 >= self.map.len() as isize
                        || new_pos.1 >= self.map[0].len() as isize
                    {
                        continue;
                    }
                    let new_pos = (new_pos.0 as usize, new_pos.1 as usize);

                    if !self.map[new_pos.1][new_pos.0] {
                        new_reachable.insert(new_pos);
                    }
                }
            }
            reachable = new_reachable;
        }

        Ok(reachable.len())
    }

    fn part2(self) -> Result<usize> {
        let mut reachable = HashSet::new();
        reachable.insert(((0, 0), self.start_pos));

        let steps = 26501365;
        let n = self.map.len();
        let mut cnt = [0, 0, 0];
        for i in 0..usize::MAX {
            if i % n == steps % n {
                println!("{}: {} {}", i, reachable.len(), i / n);
                cnt[i / n] = reachable.len();
                if i / n == 2 {
                    break;
                }
            }
            let mut new_reachable = HashSet::new();

            for (grid, pos) in reachable.iter() {
                for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let mut new_pos = (pos.0 as isize + dx, pos.1 as isize + dy);
                    let mut new_grid = *grid;
                    if new_pos.0 < 0 {
                        new_grid.0 -= 1;
                        new_pos.0 = self.map[0].len() as isize - 1;
                    } else if new_pos.1 < 0 {
                        new_grid.1 -= 1;
                        new_pos.1 = self.map.len() as isize - 1;
                    } else if new_pos.0 >= self.map.len() as isize {
                        new_grid.0 += 1;
                        new_pos.0 = 0;
                    } else if new_pos.1 >= self.map[0].len() as isize {
                        new_grid.1 += 1;
                        new_pos.1 = 0;
                    }

                    let new_pos = (new_pos.0 as usize, new_pos.1 as usize);

                    if !self.map[new_pos.1][new_pos.0] {
                        new_reachable.insert((new_grid, new_pos));
                    }
                }
            }
            reachable = new_reachable;
        }

        // use newton's forward difference formula to create a polynomial of degree 2
        // through the points (65, cnt[0]), (131 + 65, cnt[1]), (131 + 131 + 65,
        // cnt[2]) then evaluate the polynomial at the s such that s * 131  + 65
        // = steps

        let [y1, y2, y3] = cnt;

        let p = |s: usize| y1 + s * (y2 - y1) + s * (s - 1) * (y3 - 2 * y2 + y1) / 2;

        Ok(p(steps / n))
    }
}

aoc_main!(Day21);

//#[cfg(test)]
//mod tests {
//    use super::*;

//    const EXAMPLE: &str = r#"
//    "#;

//    #[test]
//    fn test_part1() {
//        assert_task!(Day21, 1, EXAMPLE, ());
//    }

//    #[test]
//    fn test_part2() {
//        assert_task!(Day21, 2, EXAMPLE, ());
//    }
//}
