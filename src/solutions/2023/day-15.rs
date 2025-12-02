use aoc_rust::*;
use common::*;

struct Day15 {
    initialization_sequence: Vec<String>,
}

impl Day15 {
    fn hash<T: ToString>(s: T) -> u8 {
        s.to_string()
            .chars()
            .fold(0, |acc, c| ((acc + c as u16) * 17) % 256) as u8
    }
}

#[derive(Debug)]
enum Operation {
    Set(String, u32),
    Remove(String),
}

impl Operation {
    fn parse(input: &mut &str) -> PResult<Self> {
        alt((
            terminated(alpha1.map(String::from), "-").map(Operation::Remove),
            separated_pair(alpha1.map(String::from), "=", dec_u32)
                .map(|(s, n)| Operation::Set(s, n)),
        ))
        .parse_next(input)
    }
}

impl Problem<u32, u32> for Day15 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(take_till(0.., |c| c == ',').map(String::from), ',')
            .map(|v| Self {
                initialization_sequence: v,
            })
            .parse_next(input)
    }

    fn part1(self) -> Result<u32> {
        Ok(self
            .initialization_sequence
            .iter()
            .map(|s| Self::hash(s.to_string()) as u32)
            .sum())
    }

    fn part2(self) -> Result<u32> {
        let operations = self
            .initialization_sequence
            .iter()
            .map(|s| Operation::parse(&mut s.as_str()).unwrap())
            .collect::<Vec<_>>();

        Ok(operations
            .iter()
            .fold::<[Vec<(String, u32)>; 256], _>(
                std::array::from_fn(|_| Vec::new()),
                |mut map, operation| {
                    match operation {
                        Operation::Set(s, n) => {
                            let hash = Self::hash(s.to_string());
                            if let Some(i) = map[hash as usize].iter().position(|(k, _)| k == s) {
                                map[hash as usize][i] = (s.to_string(), *n);
                            } else {
                                map[hash as usize].push((s.to_string(), *n));
                            }
                        }
                        Operation::Remove(s) => {
                            let hash = Self::hash(s.to_string());
                            if let Some(i) = map[hash as usize].iter().position(|(k, _)| k == s) {
                                map[hash as usize].remove(i);
                            }
                        }
                    }
                    map
                },
            )
            .iter()
            .enumerate()
            .map(|(i, v)| {
                v.iter()
                    .enumerate()
                    .map(|(j, (_, k))| ((i + 1) * (j + 1)) as u32 * k)
                    .sum::<u32>()
            })
            .sum())
    }
}

aoc_main!(Day15);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn test_part1() {
        assert_task!(Day15, 1, EXAMPLE, 1320);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day15, 2, EXAMPLE, 145);
    }

    #[test]
    fn test_hash() {
        assert_eq!(Day15::hash("".to_string()), 0);
        assert_eq!(Day15::hash("HASH".to_string()), 52);
    }
}
