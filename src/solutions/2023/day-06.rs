use aoc_rust::*;
use common::*;

struct Day06 {
    races: Vec<Race>,
}

struct Race {
    time: i128,
    distance: i128,
}

impl Race {
    fn num_ways(&self) -> i128 {
        let d = (self.time * self.time) - 4 * self.distance;

        if d <= 0 {
            return 0;
        }

        let d = (d as f64).sqrt();
        let x1 = ((self.time as f64 - d) / 2.0).floor() as i128 + 1;
        let x2 = ((self.time as f64 + d) / 2.0).ceil() as i128 - 1;

        x2 - x1 + 1
    }
}

impl Problem<i128, i128> for Day06 {
    fn parse(input: &mut &str) -> PResult<Self> {
        let times = preceded(("Time:", space1), list(dec_i128, space1));
        let distances = preceded(("Distance:", space1), list(dec_i128, space1));

        separated_pair(times, line_ending, distances)
            .map(|(times, distances)| {
                let races = times
                    .iter()
                    .zip(distances.iter())
                    .map(|(&time, &distance)| Race { time, distance })
                    .collect();
                Self { races }
            })
            .parse_next(input)
    }

    fn part1(self) -> Result<i128> {
        Ok(self.races.iter().fold(1, |a, race| a * race.num_ways()))
    }

    fn part2(self) -> Result<i128> {
        let (time, distance) =
            self.races
                .iter()
                .map(|r| (r.time, r.distance))
                .fold((0, 0), |(ta, da), (tb, db)| {
                    (
                        ta * 10i128.pow(tb.to_string().len() as u32) + tb,
                        da * 10i128.pow(db.to_string().len() as u32) + db,
                    )
                });
        Ok(Race { time, distance }.num_ways())
    }
}

aoc_main!(Day06);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_part1() {
        assert_task!(Day06, 1, EXAMPLE, 288)
    }

    #[test]
    fn test_part2() {
        assert_task!(Day06, 2, EXAMPLE, 71503)
    }
}
