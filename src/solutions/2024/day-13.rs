use aoc_rust::*;
use common::*;

struct Game {
    a: Vec2<isize>,
    b: Vec2<isize>,
    prize: Vec2<isize>,
}

impl Game {
    fn parse(input: &str) -> ParseResult<Self> {
        let (input, btn_a) = preceded(
            tag("Button A: X+"),
            separated_pair(digit1, tag(", Y+"), digit1),
        )(input)?;
        let (input, _) = line_ending(input)?;
        let (input, btn_b) = preceded(
            tag("Button B: X+"),
            separated_pair(digit1, tag(", Y+"), digit1),
        )(input)?;
        let (input, _) = line_ending(input)?;
        let (input, prize) = preceded(
            tag("Prize: X="),
            separated_pair(digit1, tag(", Y="), digit1),
        )(input)?;

        let a = Vec2::new(btn_a.0.parse().unwrap(), btn_a.1.parse().unwrap());
        let b = Vec2::new(btn_b.0.parse().unwrap(), btn_b.1.parse().unwrap());
        let prize = Vec2::new(prize.0.parse().unwrap(), prize.1.parse().unwrap());

        Ok((input, Game { a, b, prize }))
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
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending.and(line_ending), Game::parse)
            .map(|games| Day13 { games })
            .parse(input)
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
