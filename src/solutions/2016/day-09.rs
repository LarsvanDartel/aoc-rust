use aoc_rust::*;

struct Day09 {
    file: String,
}

impl Day09 {
    fn decompress(&self) -> String {
        let mut result = String::new();
        let mut chars = self.file.chars();
        while let Some(c) = chars.next() {
            if c == '(' {
                let mut marker = String::new();
                for c in chars.by_ref() {
                    if c == ')' {
                        break;
                    }
                    marker.push(c);
                }
                let (length, times) = {
                    let mut parts = marker.split('x');
                    (
                        parts.next().unwrap().parse::<usize>().unwrap(),
                        parts.next().unwrap().parse::<usize>().unwrap(),
                    )
                };
                let mut to_repeat = String::new();
                for _ in 0..length {
                    to_repeat.push(chars.next().unwrap());
                }
                for _ in 0..times {
                    result.push_str(&to_repeat);
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    fn decompressed_size(file: &str) -> usize {
        let mut size = 0;
        let mut chars = file.chars();
        while let Some(c) = chars.next() {
            if c == '(' {
                let mut marker = String::new();
                for c in chars.by_ref() {
                    if c == ')' {
                        break;
                    }
                    marker.push(c);
                }
                let (length, times) = {
                    let mut parts = marker.split('x');
                    (
                        parts.next().unwrap().parse::<usize>().unwrap(),
                        parts.next().unwrap().parse::<usize>().unwrap(),
                    )
                };
                let mut to_repeat = String::new();
                for _ in 0..length {
                    to_repeat.push(chars.next().unwrap());
                }
                size += times * Self::decompressed_size(&to_repeat);
            } else {
                size += 1;
            }
        }
        size
    }
}

impl Problem<usize, usize> for Day09 {
    fn parse(input: &str) -> ParseResult<Self> {
        Ok((
            "",
            Day09 {
                file: input.trim().to_string(),
            },
        ))
    }

    fn part1(self) -> Result<usize> {
        Ok(self.decompress().len())
    }

    fn part2(self) -> Result<usize> {
        Ok(Self::decompressed_size(&self.file))
    }
}

aoc_main!(Day09);

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const EXAMPLE: &str = r#"
//     "#;
//
//     #[test]
//     fn test_part1() {
//         assert_task!(Day09, 1, EXAMPLE, ());
//     }
//
//     #[test]
//     fn test_part2() {
//         assert_task!(Day09, 2, EXAMPLE, ());
//     }
// }
