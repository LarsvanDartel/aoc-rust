use aoc_rust::*;
use common::*;

struct Day02 {
    numbers: Vec<Vec<i32>>,
}

impl Day02 {
    fn is_safe(n: &[i32], part1: bool) -> bool {
        if part1 {
            return n.windows(2).all(|w| (1..=3).contains(&(w[0] - w[1])))
                || n.windows(2).all(|w| (-3..=-1).contains(&(w[0] - w[1])));
        }

        for i in 0..n.len() {
            // Check if the nubmers are safe without n[i]
            let without_i = n
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, &x)| x)
                .collect::<Vec<_>>();

            if Day02::is_safe(&without_i, true) {
                return true;
            }
        }

        false
    }
}

impl Problem<usize, usize> for Day02 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(list(dec_i32, space1), line_ending)
            .map(|numbers| Day02 { numbers })
            .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self
            .numbers
            .iter()
            .filter(|n| Day02::is_safe(n, true))
            .count())
    }

    fn part2(self) -> Result<usize> {
        Ok(self
            .numbers
            .iter()
            .filter(|n| Day02::is_safe(n, false))
            .count())
    }
}

aoc_main!(Day02);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day02, 1, EXAMPLE, 2);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day02, 2, EXAMPLE, 4);
    }
}
