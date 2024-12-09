use std::cmp::Ordering;

use aoc_rust::*;
use common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum File {
    File(usize, usize),
    Empty(usize),
}

#[derive(Debug)]
struct FileSystem(Vec<File>);

impl From<Vec<usize>> for FileSystem {
    fn from(filesystem: Vec<usize>) -> Self {
        let filesystem = filesystem
            .into_iter()
            .enumerate()
            .map(|(i, f)| {
                if i % 2 == 1 {
                    File::Empty(f)
                } else {
                    File::File(i / 2, f)
                }
            })
            .collect::<Vec<_>>();
        Self(filesystem)
    }
}

impl FileSystem {
    fn compact(&mut self, preserve_contiguous: bool) {
        let mut i = self.0.len() - 1;
        let mut j = 0;
        while i > j {
            match (self.0[i], self.0[j]) {
                (File::Empty(_), _) => i -= 1,
                (_, File::File(_, _)) => {
                    j += 1;
                    if j >= i {
                        j = 0;
                        i -= 1;
                    }
                }
                (File::File(desc, s1), File::Empty(s2)) => {
                    if preserve_contiguous {
                        if s1 > s2 {
                            j += 1;
                            if j >= i {
                                j = 0;
                                i -= 1;
                            }
                            continue;
                        }
                        self.0[j] = File::File(desc, s1);
                        self.0[i] = File::Empty(s1);
                        let rem = s2 - s1;
                        if rem > 0 {
                            self.0.insert(j + 1, File::Empty(rem));
                        } else {
                            i -= 1;
                        }
                        j = 0;
                    } else {
                        self.0[j] = File::File(desc, s1.min(s2));
                        match s1.cmp(&s2) {
                            Ordering::Greater => {
                                self.0[i] = File::File(desc, s1 - s2);
                                self.0.insert(i + 1, File::Empty(s2));
                            }
                            Ordering::Less => {
                                self.0[i] = File::Empty(s1);
                                self.0.insert(j + 1, File::Empty(s2 - s1));
                            }
                            Ordering::Equal => {
                                self.0[i] = File::Empty(s1);
                                i += 1;
                            }
                        }
                        j -= 1;
                    }
                }
            }
        }

        let mut i = self.0.len() - 1;
        while i > 0 {
            match (self.0[i], self.0[i - 1]) {
                (File::Empty(s1), File::Empty(s2)) => {
                    self.0[i - 1] = File::Empty(s1 + s2);
                    self.0.remove(i);
                }
                (File::File(d1, s1), File::File(d2, s2)) if d1 == d2 => {
                    self.0[i - 1] = File::File(d1, s1 + s2);
                    self.0.remove(i);
                }
                _ => (),
            }
            i -= 1;
        }
    }

    fn checksum(&self) -> usize {
        let mut checksum = 0;
        let mut i = 0;
        for file in &self.0 {
            match file {
                File::File(desc, size) => {
                    checksum += desc * (i * size + size * (size - 1) / 2);
                    i += size;
                }
                File::Empty(size) => i += size,
            }
        }
        checksum
    }
}

struct Day09 {
    filesystem: FileSystem,
}

impl Problem<usize, usize> for Day09 {
    fn parse(input: &str) -> ParseResult<Self> {
        many1(verify(anychar, char::is_ascii_digit).map(|c| (c as u8 - b'0') as usize))
            .map(|filesystem| Day09 {
                filesystem: filesystem.into(),
            })
            .parse(input)
    }

    fn part1(mut self) -> Result<usize> {
        self.filesystem.compact(false);
        Ok(self.filesystem.checksum())
    }

    fn part2(mut self) -> Result<usize> {
        self.filesystem.compact(true);
        Ok(self.filesystem.checksum())
    }
}

aoc_main!(Day09);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"2333133121414131402
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day09, 1, EXAMPLE, 1928);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day09, 2, EXAMPLE, 2858);
    }
}
