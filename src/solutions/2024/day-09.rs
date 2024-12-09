use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

use aoc_rust::*;
use common::*;

struct File {
    desc: Option<usize>,
    start: usize,
    size: usize,
}

struct FileSystem {
    files: Vec<File>,
    space: [BinaryHeap<Reverse<usize>>; 10],
}

impl From<Vec<usize>> for FileSystem {
    fn from(filesystem: Vec<usize>) -> Self {
        let mut start = 0;
        let mut files = Vec::new();
        let mut space: [BinaryHeap<Reverse<usize>>; 10] = Default::default();
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
                space[size].push(Reverse(i));
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

        Self { files, space }
    }
}

impl FileSystem {
    fn compact(&mut self, preserve_contiguous: bool) {
        let mut l_idx = 1;
        let mut r_idx = self.files.len() - 2;
        while r_idx > 0 {
            let File {
                desc,
                size: r_size,
                start: r_start,
            } = self.files[r_idx];

            if preserve_contiguous {
                l_idx = self.space[r_size..10]
                    .iter()
                    .flat_map(|heap| heap.peek().cloned())
                    .map(|Reverse(idx)| idx)
                    .min()
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
                        self.space[l_size - r_size].push(Reverse(l_idx));
                    }
                }
            }
            if preserve_contiguous {
                self.space[l_size].pop();
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
