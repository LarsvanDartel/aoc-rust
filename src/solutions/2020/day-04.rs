use aoc_rust::*;
use common::*;
use hashbrown::HashMap;

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(
            alt((space1, line_ending)),
            separated_pair(alpha1, char(':'), take_till(char::is_whitespace)),
        )
        .map(|fields| {
            let fields = fields
                .into_iter()
                .map(|(k, v): (&str, &str)| (k.to_string(), v.to_string()))
                .collect();
            Passport { fields }
        })
        .parse(input)
    }

    fn is_valid(&self) -> bool {
        let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        required_fields.iter().all(|f| self.fields.contains_key(*f))
    }

    fn is_valid2(&self) -> bool {
        self.is_valid()
            && self.fields.iter().all(|(k, v)| match k.as_str() {
                "byr" => {
                    let year = v.parse::<u32>().unwrap();
                    (1920..=2002).contains(&year)
                }
                "iyr" => {
                    let year = v.parse::<u32>().unwrap();
                    (2010..=2020).contains(&year)
                }
                "eyr" => {
                    let year = v.parse::<u32>().unwrap();
                    (2020..=2030).contains(&year)
                }
                "hgt" => {
                    let unit = &v[v.len() - 2..];
                    let value = v[..v.len() - 2].parse::<u32>().unwrap();
                    match unit {
                        "cm" => (150..=193).contains(&value),
                        "in" => (59..=76).contains(&value),
                        _ => false,
                    }
                }
                "hcl" => {
                    v.len() == 7
                        && v.starts_with('#')
                        && v.chars().skip(1).all(|c| c.is_ascii_hexdigit())
                }
                "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v.as_str()),
                "pid" => v.len() == 9 && v.chars().all(char::is_numeric),
                _ => true,
            })
    }
}

struct Day04 {
    passports: Vec<Passport>,
}

impl Problem<usize, usize> for Day04 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending.and(line_ending), Passport::parse)
            .map(|passports| Day04 { passports })
            .parse(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self.passports.iter().filter(|p| p.is_valid()).count())
    }

    fn part2(self) -> Result<usize> {
        Ok(self.passports.iter().filter(|p| p.is_valid2()).count())
    }
}

aoc_main!(Day04);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day04, 1, EXAMPLE, 2);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day04, 2, EXAMPLE, 2);
    }
}
