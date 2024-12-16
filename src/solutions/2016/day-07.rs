use aoc_rust::*;
use common::*;

struct Day07 {
    ip_addresses: Vec<IpAddress>,
}

struct IpAddress {
    parts: Vec<IpPart>,
}

impl IpAddress {
    fn parse(input: &mut &str) -> PResult<Self> {
        repeat(0.., IpPart::parse)
            .map(|parts| IpAddress { parts })
            .parse_next(input)
    }

    fn supports_tls(&self) -> bool {
        self.parts.iter().any(|p| p.has_abba() == Some(true))
            && self.parts.iter().all(|p| p.has_abba() != Some(false))
    }

    fn supports_ssl(&self) -> bool {
        let abas: Vec<String> = self
            .parts
            .iter()
            .filter(|p| !p.is_hypernet)
            .flat_map(|p| p.abas())
            .collect();
        let babs: Vec<String> = self
            .parts
            .iter()
            .filter(|p| p.is_hypernet)
            .flat_map(|p| p.abas())
            .collect();

        abas.iter().any(|aba| {
            let bab = format!(
                "{}{}{}",
                aba.chars().nth(1).unwrap(),
                aba.chars().nth(0).unwrap(),
                aba.chars().nth(1).unwrap()
            );
            babs.contains(&bab)
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct IpPart {
    is_hypernet: bool,
    value: String,
}

impl IpPart {
    fn parse(input: &mut &str) -> PResult<Self> {
        alt((
            delimited("[", alpha1, "]").map(|s: &str| IpPart {
                is_hypernet: true,
                value: s.to_string(),
            }),
            alpha1.map(|s: &str| IpPart {
                is_hypernet: false,
                value: s.to_string(),
            }),
        ))
        .parse_next(input)
    }

    fn has_abba(&self) -> Option<bool> {
        if self
            .value
            .chars()
            .collect::<Vec<_>>()
            .windows(4)
            .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
        {
            Some(!self.is_hypernet)
        } else {
            None
        }
    }

    fn abas(&self) -> Vec<String> {
        self.value
            .chars()
            .collect::<Vec<_>>()
            .windows(3)
            .filter_map(|w| {
                if w[0] == w[2] && w[0] != w[1] {
                    Some(w.iter().collect())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Problem<usize, usize> for Day07 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(0.., IpAddress::parse, line_ending)
            .map(|ip_addresses| Day07 { ip_addresses })
            .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self
            .ip_addresses
            .iter()
            .filter(|ip| ip.supports_tls())
            .count())
    }

    fn part2(self) -> Result<usize> {
        Ok(self
            .ip_addresses
            .iter()
            .filter(|ip| ip.supports_ssl())
            .count())
    }
}

aoc_main!(Day07);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_task!(Day07, 1, "abba[mnop]qrst", 1);
        assert_task!(Day07, 1, "abcd[bddb]xyyx", 0);
        assert_task!(Day07, 1, "aaaa[qwer]tyui", 0);
        assert_task!(Day07, 1, "ioxxoj[asdfgh]zxcvbn", 1);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day07, 2, "aba[bab]xyz", 1);
        assert_task!(Day07, 2, "xyx[xyx]xyx", 0);
        assert_task!(Day07, 2, "aaa[kek]eke", 1);
        assert_task!(Day07, 2, "zazbz[bzb]cdb", 1);
    }
}
