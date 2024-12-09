use std::cmp::Ordering;

use aoc_rust::*;
use common::*;

#[derive(Clone)]
struct File {
    desc: Option<usize>,
    start: usize,
    size: usize,
}

struct FileSystem {
    files: Vec<File>,
}

impl From<Vec<usize>> for FileSystem {
    fn from(filesystem: Vec<usize>) -> Self {
        let mut start = 0;
        let mut files = Vec::new();
        for (i, size) in filesystem.into_iter().enumerate() {
            if i % 2 == 0 {
                files.push(File {
                    desc: Some(i / 2),
                    start,
                    size,
                });
            } else {
                files.push(File {
                    desc: None,
                    start,
                    size,
                });
            }
            start += size;
        }
        if files.len() % 2 == 1 {
            files.push(File {
                desc: None,
                start,
                size: 0,
            });
        }

        Self { files }
    }
}

impl FileSystem {
    fn compact(&mut self, preserve_contiguous: bool) {
        let mut l_idx = 1;
        let mut r_idx = self.files.len() - 2;
        let mut min_idx: [Option<usize>; 10] = (0..10)
            .map(|i| {
                self.files
                    .iter()
                    .skip(1)
                    .step_by(2)
                    .position(|f| f.size == i)
                    .map(|i| 2 * i + 1)
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        while r_idx > 0 {
            let File {
                desc,
                size: r_size,
                start: r_start,
            } = self.files[r_idx];

            if preserve_contiguous {
                l_idx = min_idx[r_size..10]
                    .iter()
                    .flatten()
                    .min()
                    .cloned()
                    .unwrap_or(r_idx);
            } else {
                while l_idx <= r_idx && self.files[l_idx].size == 0 {
                    l_idx += 2;
                }
            }

            if l_idx >= r_idx {
                r_idx -= 2;
                continue;
            }

            let File {
                start: l_start,
                size: l_size,
                ..
            } = self.files[l_idx];

            match l_size.cmp(&r_size) {
                Ordering::Less => {
                    self.files[l_idx].desc = desc;
                    self.files[r_idx].size -= l_size;
                    self.files[r_idx + 1].size += l_size;
                    self.files[r_idx + 1].start -= l_size;
                    l_idx += 2;
                }
                Ordering::Equal => {
                    self.files[l_idx].start = r_start;
                    self.files[r_idx].start = l_start;
                    r_idx -= 2;
                    l_idx += 2;
                }
                Ordering::Greater => {
                    self.files[l_idx].size -= r_size;
                    self.files[l_idx].start += r_size;
                    self.files[r_idx].start = l_start;
                    self.files[r_idx + 1].size += r_size;
                    self.files[r_idx + 1].start -= r_size;
                    r_idx -= 2;

                    if preserve_contiguous {
                        if let Some(m) = min_idx[l_size - r_size] {
                            min_idx[l_size - r_size] = Some(m.min(l_idx));
                        } else {
                            min_idx[l_size - r_size] = Some(l_idx);
                        }
                    }
                }
            }
            if preserve_contiguous {
                min_idx[l_size] = (l_idx..=r_idx)
                    .step_by(2)
                    .position(|i| self.files[i].size == l_size)
                    .map(|i| 2 * i + l_idx);
            }
        }
    }

    fn checksum(&self) -> usize {
        self.files.iter().fold(0, |checksum, f| {
            let File { start, size, desc } = f;
            if let Some(desc) = desc {
                checksum + desc * (start * size + size * (size - 1) / 2)
            } else {
                checksum
            }
        })
    }
}

impl std::fmt::Debug for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut files = self.files.clone();
        files.sort_by_key(|f| f.start);

        for file in files.iter() {
            let c = if let Some(desc) = file.desc {
                desc.to_string()
            } else {
                ".".to_string()
            };
            for _ in 0..file.size {
                write!(f, "{}", c)?;
            }
        }
        Ok(())
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
