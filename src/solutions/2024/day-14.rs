use aoc_rust::*;
use common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
    pos: Vec2<i32>,
    vel: Vec2<i32>,
}

impl Robot {
    fn parse(input: &mut &str) -> PResult<Self> {
        let _ = "p=".parse_next(input)?;
        let pos = separated_pair(dec_i32, ",", dec_i32).parse_next(input)?;
        let _ = " v=".parse_next(input)?;
        let vel = separated_pair(dec_i32, ",", dec_i32).parse_next(input)?;
        Ok(Robot {
            pos: pos.into(),
            vel: vel.into(),
        })
    }
}

struct Day14 {
    robots: Vec<Robot>,
}

impl Problem<usize, usize> for Day14 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(Robot::parse, line_ending)
            .map(|robots| Day14 { robots })
            .parse_next(input)
    }

    fn part1(mut self) -> Result<usize> {
        #[cfg(not(test))]
        let (width, height) = (101, 103);
        #[cfg(test)]
        let (width, height) = (11, 7);
        let size = Vec2::new(width, height);
        for _ in 0..100 {
            for r in self.robots.iter_mut() {
                r.pos = (r.pos + r.vel + size) % size;
            }
        }

        let mut quadrants = [0; 4];
        for r in self.robots {
            if r.pos.x < width / 2 && r.pos.y < height / 2 {
                quadrants[0] += 1;
            } else if r.pos.x > width / 2 && r.pos.y < height / 2 {
                quadrants[1] += 1;
            } else if r.pos.x < width / 2 && r.pos.y > height / 2 {
                quadrants[2] += 1;
            } else if r.pos.x > width / 2 && r.pos.y > height / 2 {
                quadrants[3] += 1;
            }
        }

        println!("{:?}", quadrants);

        Ok(quadrants.iter().product())
    }

    fn part2(mut self) -> Result<usize> {
        #[cfg(not(test))]
        let (width, height) = (101, 103);
        #[cfg(test)]
        let (width, height) = (11, 7);
        let size = Vec2::new(width, height);
        let mut i = 0;
        loop {
            i += 1;
            for r in self.robots.iter_mut() {
                r.pos = (r.pos + r.vel + size) % size;
            }
            if self
                .robots
                .iter()
                .map(|r| r.pos)
                .collect::<HashSet<_>>()
                .len()
                == self.robots.len()
            {
                break;
            }
        }

        for y in 0..height {
            for x in 0..width {
                if self.robots.iter().any(|r| r.pos == Vec2::new(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        Ok(i)
    }
}

aoc_main!(Day14);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day14, 1, EXAMPLE, 12);
    }
}
