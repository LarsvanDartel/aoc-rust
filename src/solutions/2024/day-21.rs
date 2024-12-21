use std::cmp::Ordering;

use aoc_rust::*;
use common::*;

struct Day21 {
    codes: Vec<String>,
    memo: HashMap<u64, usize>,
}

impl Day21 {
    const KEYPAD_NUMERIC: [[char; 3]; 4] = [
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        [' ', '0', 'A'],
    ];
    const KEYPAD_DIRECTION: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];

    fn hash(pos: Vec2<usize>, next: Vec2<usize>, n_keypads: usize) -> u64 {
        let mut hash = 0;
        hash |= pos.x as u64;
        hash |= (pos.y as u64) << 8;
        hash |= (next.x as u64) << 16;
        hash |= (next.y as u64) << 24;
        hash |= (n_keypads as u64) << 32;
        hash
    }

    fn cost_step(
        &mut self,
        pos: Vec2<usize>,
        next: Vec2<usize>,
        keypads: &[&[[char; 3]]],
    ) -> usize {
        let hash = Self::hash(pos, next, keypads.len());
        if let Some(&cost) = self.memo.get(&hash) {
            return cost;
        }
        let mut cost = usize::MAX;
        let mut q = VecDeque::new();
        q.push_back((pos, String::new()));
        let keypad = keypads[0];
        while let Some((mut p, presses)) = q.pop_front() {
            if p == next {
                cost = cost.min(self.cost(&(presses + "A"), &keypads[1..]));
                continue;
            }
            if keypad[p.y][p.x] == ' ' {
                continue;
            }

            match p.x.cmp(&next.x) {
                Ordering::Less => {
                    p.x += 1;
                    q.push_back((p, presses.clone() + ">"));
                    p.x -= 1;
                },
                Ordering::Greater => {
                    p.x -= 1;
                    q.push_back((p, presses.clone() + "<"));
                    p.x += 1;
                },
                Ordering::Equal => {},
            }
            match p.y.cmp(&next.y) {
                Ordering::Less => {
                    p.y += 1;
                    q.push_back((p, presses.clone() + "v"));
                    p.y -= 1;
                },
                Ordering::Greater => {
                    p.y -= 1;
                    q.push_back((p, presses.clone() + "^"));
                    p.y += 1;
                },
                Ordering::Equal => {},
            }
        }

        self.memo.insert(hash, cost);
        cost
    }

    fn cost(&mut self, presses: &str, keypads: &[&[[char; 3]]]) -> usize {
        if keypads.is_empty() {
            return presses.len();
        }
        let keypad = keypads[0];
        let mut pos: Vec2<usize> = keypad
            .iter()
            .enumerate()
            .find_map(|(i, row)| row.iter().position(|&c| c == 'A').map(|j| (j, i)))
            .unwrap()
            .into();

        let mut cost = 0;

        for key in presses.chars() {
            let next_pos: Vec2<usize> = keypad
                .iter()
                .enumerate()
                .find_map(|(i, row)| row.iter().position(|&ch| ch == key).map(|j| (j, i)))
                .unwrap()
                .into();
            cost += self.cost_step(pos, next_pos, keypads);
            pos = next_pos;
        }

        cost
    }

    fn complexity(&mut self, keypads: &[&[[char; 3]]]) -> usize {
        let codes = self.codes.clone();
        codes
            .iter()
            .map(|code| {
                let cost = self.cost(code, keypads);
                cost * code.strip_suffix('A').unwrap().parse::<usize>().unwrap()
            })
            .sum()
    }
}

impl Problem<usize, usize> for Day21 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(alphanumeric1.map(String::from), line_ending)
            .map(|codes| Self {
                codes,
                memo: HashMap::new(),
            })
            .parse_next(input)
    }

    fn part1(mut self) -> Result<usize> {
        let mut keypads: Vec<&[[char; 3]]> = Vec::new();
        keypads.push(&Self::KEYPAD_NUMERIC);
        for _ in 0..2 {
            keypads.push(&Self::KEYPAD_DIRECTION);
        }
        Ok(self.complexity(&keypads))
    }

    fn part2(mut self) -> Result<usize> {
        let mut keypads: Vec<&[[char; 3]]> = Vec::new();
        keypads.push(&Self::KEYPAD_NUMERIC);
        for _ in 0..25 {
            keypads.push(&Self::KEYPAD_DIRECTION);
        }
        Ok(self.complexity(&keypads))
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
