use aoc_rust::*;
use common::*;

struct Day04 {
    rooms: Vec<Room>,
}

struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn parse(input: &mut &str) -> PResult<Self> {
        let name = take_till(0.., |c: char| c.is_ascii_digit()).parse_next(input)?;
        let sector_id = dec_uint(input)?;
        let checksum = delimited("[", take(5usize), "]").parse_next(input)?;

        Ok(Self {
            name: name.to_string(),
            sector_id,
            checksum: checksum.to_string(),
        })
    }

    fn is_real(&self) -> bool {
        let counts =
            self.name
                .chars()
                .filter(|c| c.is_ascii_lowercase())
                .fold([0; 26], |mut counts, c| {
                    counts[(c as u8 - b'a') as usize] += 1;
                    counts
                });

        let mut counts: Vec<_> = counts.iter().enumerate().collect();
        counts.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(&b.0)));
        let checksum: String = counts
            .iter()
            .take(5)
            .map(|(i, _)| (b'a' + *i as u8) as char)
            .collect();
        checksum == self.checksum
    }

    fn decrypt_name(&self) -> String {
        self.name
            .chars()
            .map(|c| {
                if c == '-' {
                    ' '
                } else {
                    let c = c as u8 - b'a';
                    let c = (c + (self.sector_id % 26) as u8) % 26;
                    (c + b'a') as char
                }
            })
            .collect()
    }
}

impl std::fmt::Debug for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}[{}]", self.name, self.sector_id, self.checksum)
    }
}

impl Problem<u32, u32> for Day04 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(0.., Room::parse, line_ending)
            .map(|rooms| Self { rooms })
            .parse_next(input)
    }

    fn part1(self) -> Result<u32> {
        Ok(self
            .rooms
            .iter()
            .filter(|r| r.is_real())
            .map(|r| r.sector_id)
            .sum())
    }

    fn part2(self) -> Result<u32> {
        for room in self.rooms {
            if room.is_real() {
                let name = room.decrypt_name();
                if name.contains("north") {
                    return Ok(room.sector_id);
                }
            }
        }
        Err(AoCError::NoSolution)
    }
}

aoc_main!(Day04);
