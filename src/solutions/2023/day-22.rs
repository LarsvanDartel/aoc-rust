use aoc_rust::*;
use common::*;

struct Day22 {
    bricks: Vec<Brick>,
}

#[derive(Clone)]
struct Brick {
    x0: u32,
    x1: u32,
    y0: u32,
    y1: u32,
    z0: u32,
    z1: u32,
}

impl Brick {
    fn parse(input: &mut &str) -> PResult<Self> {
        (
            dec_u32, ",", dec_u32, ",", dec_u32, "~", dec_u32, ",", dec_u32, ",", dec_u32,
        )
            .map(|(x0, _, y0, _, z0, _, x1, _, y1, _, z1)| Brick {
                x0,
                x1,
                y0,
                y1,
                z0,
                z1,
            })
            .parse_next(input)
    }
}

impl std::fmt::Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}-{}; {}-{}; {}-{}]",
            self.x0, self.x1, self.y0, self.y1, self.z0, self.z1
        )
    }
}

fn drop_brick(brick: &Brick, tallest: &mut [[u32; 10]; 10]) -> Brick {
    let mut brick = brick.clone();
    let mut highest_brick = 0;
    for x in brick.x0..=brick.x1 {
        for y in brick.y0..=brick.y1 {
            highest_brick = highest_brick.max(tallest[x as usize][y as usize]);
        }
    }
    let dz = brick.z0 as i32 - highest_brick as i32 - 1;
    let dz = dz.max(0) as u32;
    brick.z0 -= dz;
    brick.z1 -= dz;

    for x in brick.x0..=brick.x1 {
        for y in brick.y0..=brick.y1 {
            tallest[x as usize][y as usize] = brick.z1;
        }
    }

    brick
}

impl Problem<usize, usize> for Day22 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(Brick::parse, line_ending)
            .map(|bricks| Self { bricks })
            .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        let mut bricks = self.bricks;
        bricks.sort_by_key(|brick| brick.z0);

        let mut tallest = [[0; 10]; 10];
        for b in &mut bricks {
            *b = drop_brick(b, &mut tallest);
        }

        let mut cnt = 0;
        for i in 0..bricks.len() {
            tallest = [[0; 10]; 10];
            let mut dropped = false;
            for (j, b) in bricks.iter().enumerate() {
                if i == j {
                    continue;
                }
                if b.z0 != drop_brick(b, &mut tallest).z0 {
                    dropped = true;
                    break;
                }
            }
            if !dropped {
                cnt += 1;
            }
        }

        Ok(cnt)
    }

    fn part2(self) -> Result<usize> {
        let mut bricks = self.bricks;
        bricks.sort_by_key(|brick| brick.z0);

        let mut tallest = [[0; 10]; 10];
        for b in &mut bricks {
            *b = drop_brick(b, &mut tallest);
        }

        let mut cnt = 0;
        for i in 0..bricks.len() {
            tallest = [[0; 10]; 10];
            for (j, b) in bricks.iter().enumerate() {
                if i == j {
                    continue;
                }
                if b.z0 != drop_brick(b, &mut tallest).z0 {
                    cnt += 1;
                }
            }
        }

        Ok(cnt)
    }
}

aoc_main!(Day22);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;

    #[test]
    fn test_part1() {
        assert_task!(Day22, 1, EXAMPLE, 5);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day22, 2, EXAMPLE, 7);
    }
}
