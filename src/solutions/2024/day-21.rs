use std::iter;

use aoc_rust::*;
use common::*;

struct Day21 {
    keypad_num: KeyPad,
    keypad_dir: KeyPad,
    codes: Vec<String>,
}

struct KeyPad {
    paths: HashMap<(char, char), String>,
}

impl KeyPad {
    const KEYPAD_NUM: [(Vec2<usize>, char); 11] = [
        (Vec2::new(0, 0), '7'),
        (Vec2::new(1, 0), '8'),
        (Vec2::new(2, 0), '9'),
        (Vec2::new(0, 1), '4'),
        (Vec2::new(1, 1), '5'),
        (Vec2::new(2, 1), '6'),
        (Vec2::new(0, 2), '1'),
        (Vec2::new(1, 2), '2'),
        (Vec2::new(2, 2), '3'),
        (Vec2::new(1, 3), '0'),
        (Vec2::new(2, 3), 'A'),
    ];

    const KEYPAD_DIR: [(Vec2<usize>, char); 5] = [
        (Vec2::new(1, 0), '^'),
        (Vec2::new(2, 0), 'A'),
        (Vec2::new(0, 1), '<'),
        (Vec2::new(1, 1), 'v'),
        (Vec2::new(2, 1), '>'),
    ];

    fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = (Vec2<usize>, char)> + Clone,
    {
        let map: HashMap<_, _> = iter.collect();
        let mut paths = HashMap::new();
        for &pos in map.keys() {
            for &next in map.keys() {
                let mut path = String::new();
                let mut p = pos;
                while p != next {
                    if next.x < p.x && map.contains_key(&Vec2::new(next.x, p.y)) {
                        path.extend(iter::repeat_n('<', p.x - next.x));
                        p.x = next.x;
                    }
                    if next.y < p.y && map.contains_key(&Vec2::new(p.x, next.y)) {
                        path.extend(iter::repeat_n('^', p.y - next.y));
                        p.y = next.y;
                    }
                    if next.y > p.y && map.contains_key(&Vec2::new(p.x, next.y)) {
                        path.extend(iter::repeat_n('v', next.y - p.y));
                        p.y = next.y;
                    }
                    if next.x > p.x && map.contains_key(&Vec2::new(next.x, p.y)) {
                        path.extend(iter::repeat_n('>', next.x - p.x));
                        p.x = next.x;
                    }
                }
                path.push('A');
                paths.insert((map[&pos], map[&next]), path);
            }
        }
        Self { paths }
    }
}

fn hash(a: char, b: char, n_keypads: usize) -> u64 {
    let mut hash = 0;
    hash |= a as u64;
    hash |= (b as u64) << 8;
    hash |= (n_keypads as u64) << 16;
    hash
}

impl Day21 {
    fn new(codes: Vec<String>) -> Self {
        Self {
            keypad_num: KeyPad::new(KeyPad::KEYPAD_NUM.into_iter()),
            keypad_dir: KeyPad::new(KeyPad::KEYPAD_DIR.into_iter()),
            codes,
        }
    }

    fn cost(
        &self,
        presses: &str,
        n_keypads: usize,
        keypad: &KeyPad,
        cache: &mut HashMap<u64, usize>,
    ) -> usize {
        if n_keypads == 0 {
            return presses.len();
        }

        let mut pos = 'A';
        let mut total_cost = 0;

        for key in presses.chars() {
            let hash = hash(pos, key, n_keypads);
            let cost = if let Some(&cost) = cache.get(&hash) {
                cost
            } else {
                let path = &keypad.paths[&(pos, key)];
                let cost = self.cost(path, n_keypads - 1, &self.keypad_dir, cache);
                cache.insert(hash, cost);
                cost
            };
            total_cost += cost;
            pos = key;
        }

        total_cost
    }

    fn complexity(&mut self, n_keypads: usize) -> usize {
        let codes = self.codes.clone();
        let mut cache = HashMap::new();
        codes
            .iter()
            .map(|code| {
                let cost = self.cost(code, n_keypads, &self.keypad_num, &mut cache);
                let num = code.chars().fold(0, |acc, key| {
                    if key.is_ascii_digit() {
                        acc * 10 + key.to_digit(10).unwrap() as usize
                    } else {
                        acc
                    }
                });
                cost * num
            })
            .sum()
    }
}

impl Problem<usize, usize> for Day21 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(alphanumeric1.map(String::from), line_ending)
            .map(Self::new)
            .parse_next(input)
    }

    fn part1(mut self) -> Result<usize> {
        Ok(self.complexity(3))
    }

    fn part2(mut self) -> Result<usize> {
        Ok(self.complexity(26))
    }
}

aoc_main!(Day21);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"029A
980A
179A
456A
379A
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day21, 1, EXAMPLE, 126384);
    }
}
