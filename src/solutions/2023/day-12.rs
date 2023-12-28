use aoc_rust::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    Parser,
};

struct Day12 {
    records: Vec<Record>,
}

#[derive(Clone)]
struct Record {
    springs: Vec<Spring>,
    counts: Vec<usize>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn parse(input: &str) -> ParseResult<Self> {
        alt((
            tag("?").map(|_| Self::Unknown),
            tag(".").map(|_| Self::Operational),
            tag("#").map(|_| Self::Damaged),
        ))
        .parse(input)
    }
}

impl std::fmt::Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operational => write!(f, "."),
            Self::Damaged => write!(f, "#"),
            Self::Unknown => write!(f, "?"),
        }
    }
}

impl Record {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_pair(
            many1(Spring::parse),
            space1,
            separated_list1(tag(","), digit1)
                .map(|v| v.into_iter().map(|s: &str| s.parse().unwrap()).collect()),
        )
        .map(|(springs, counts)| Self { springs, counts })
        .parse(input)
    }

    fn num_ways(&self) -> usize {
        let mut s = self.springs.clone();
        s.push(Spring::Operational);
        let n = s.len();
        let k = self.counts.len();

        let mut dp = vec![vec![vec![0; n + 2]; k + 2]; n + 1];
        dp[0][0][0] = 1;

        for i in 0..n {
            for j in 0..=k {
                for l in 0..=n {
                    let cur = dp[i][j][l];
                    if cur == 0 {
                        continue;
                    }
                    if (s[i] == Spring::Operational || s[i] == Spring::Unknown)
                        && (l == 0 || l == self.counts[j - 1])
                    {
                        dp[i + 1][j][0] += cur
                    }
                    if s[i] == Spring::Damaged || s[i] == Spring::Unknown {
                        if l == 0 {
                            dp[i + 1][j + 1][1] += cur
                        } else {
                            dp[i + 1][j][l + 1] += cur
                        }
                    }
                }
            }
        }
        dp[n][k][0]
    }

    fn expand(&self, n: usize) -> Self {
        // copy springs and counts n times
        let mut springs = Vec::with_capacity(self.springs.len() * (n + 1) - 1);
        let mut counts = Vec::with_capacity(self.counts.len() * n);

        for i in 0..n {
            if i > 0 {
                springs.push(Spring::Unknown);
            }
            springs.extend(self.springs.iter().cloned());
            counts.extend(self.counts.iter().cloned());
        }

        Self { springs, counts }
    }

    fn _num_ways(&mut self, s_idx: usize, c_idx: usize, count: usize) -> usize {
        if s_idx == self.springs.len() {
            return if c_idx == self.counts.len() - 1 && count == self.counts[c_idx]
                || count == 0 && c_idx == self.counts.len()
            {
                1
            } else {
                0
            };
        }

        if c_idx == self.counts.len() && count > 0 {
            return 0;
        }

        match self.springs[s_idx] {
            Spring::Operational => {
                if count == 0 {
                    self._num_ways(s_idx + 1, c_idx, count)
                } else if count == self.counts[c_idx] {
                    self._num_ways(s_idx + 1, c_idx + 1, 0)
                } else {
                    0
                }
            }
            Spring::Damaged => self._num_ways(s_idx + 1, c_idx, count + 1),
            Spring::Unknown => {
                let mut sum = 0;
                self.springs[s_idx] = Spring::Operational;
                sum += self._num_ways(s_idx, c_idx, count);
                self.springs[s_idx] = Spring::Damaged;
                sum += self._num_ways(s_idx, c_idx, count);
                self.springs[s_idx] = Spring::Unknown;
                sum
            }
        }
    }
}

impl std::fmt::Debug for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for spring in &self.springs {
            write!(f, "{:?}", spring)?;
        }

        write!(f, " ")?;

        for count in &self.counts {
            write!(f, "{},", count)?;
        }

        Ok(())
    }
}

impl Problem<usize, usize> for Day12 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, Record::parse)
            .map(|records| Self { records })
            .parse(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self.records.iter().map(|r| r.num_ways()).sum())
    }

    fn part2(self) -> Result<usize> {
        Ok(self.records.iter().map(|r| r.expand(5).num_ways()).sum())
    }
}

aoc_main!(Day12);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn test_part1() {
        assert_task!(Day12, 1, EXAMPLE, 21);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day12, 2, EXAMPLE, 525152);
    }
}
