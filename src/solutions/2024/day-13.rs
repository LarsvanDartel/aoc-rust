use aoc_rust::*;
use common::*;

struct Game {
    a: Vec2<isize>,
    b: Vec2<isize>,
    prize: Vec2<isize>,
}

impl Game {
    fn parse(input: &mut &str) -> PResult<Self> {
        let btn_a = preceded("Button A: X+", separated_pair(dec_isize, ", Y+", dec_isize))
            .parse_next(input)?;
        let _ = line_ending(input)?;
        let btn_b = preceded("Button B: X+", separated_pair(dec_isize, ", Y+", dec_isize))
            .parse_next(input)?;
        let _ = line_ending(input)?;
        let prize = preceded("Prize: X=", separated_pair(dec_isize, ", Y=", dec_isize))
            .parse_next(input)?;

        Ok(Self {
            a: btn_a.into(),
            b: btn_b.into(),
            prize: prize.into(),
        })
    }

    fn cost(&self, offset: isize) -> Option<isize> {
        let a = self.a;
        let b = self.b;
        let p = self.prize + Vec2::new(offset, offset);

        let nb = (a.x * p.y - a.y * p.x) / (a.x * b.y - a.y * b.x);
        let na = (p.x - nb * b.x) / a.x;

        if a * na + b * nb == p {
            Some(3 * na + nb)
        } else {
            None
        }
    }
}

struct Day13 {
    games: Vec<Game>,
}

impl Problem<isize, isize> for Day13 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(Game::parse, (line_ending, line_ending))
            .map(|games| Day13 { games })
            .parse_next(input)
    }

    fn part1(self) -> Result<isize> {
        Ok(self.games.iter().flat_map(|game| game.cost(0)).sum())
    }

    fn part2(self) -> Result<isize> {
        Ok(self
            .games
            .iter()
            .flat_map(|game| game.cost(10000000000000))
            .sum())
    }
}

aoc_main!(Day13);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day13, 1, EXAMPLE, 480);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day13, 2, EXAMPLE, 875318608908isize);
    }
}
