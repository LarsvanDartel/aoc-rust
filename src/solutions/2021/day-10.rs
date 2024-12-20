use aoc_rust::*;
use common::*;

struct Day10 {
    chunks: Vec<String>,
}

impl Day10 {
    fn find_error(chunk: &str) -> std::result::Result<Vec<char>, char> {
        let mut stack = Vec::new();
        let open = ['(', '[', '{', '<'];
        let close = [')', ']', '}', '>'];
        for ch in chunk.chars() {
            if let Some(i) = open.iter().position(|&c| c == ch) {
                stack.push(close[i]);
            } else if stack.pop() != Some(ch) {
                return Err(ch);
            }
        }

        Ok(stack)
    }
}

impl Problem<usize, usize> for Day10 {
    fn parse(input: &mut &str) -> PResult<Self> {
        list(till_line_ending.map(String::from), line_ending)
            .map(|chunks| Day10 { chunks })
            .parse_next(input)
    }

    fn part1(self) -> Result<usize> {
        let mut scores = HashMap::new();
        scores.insert(')', 3);
        scores.insert(']', 57);
        scores.insert('}', 1197);
        scores.insert('>', 25137);

        Ok(self
            .chunks
            .iter()
            .flat_map(|ch| Day10::find_error(ch).err())
            .map(|ch| scores[&ch])
            .sum())
    }

    fn part2(self) -> Result<usize> {
        let mut scores = HashMap::new();
        scores.insert(')', 1);
        scores.insert(']', 2);
        scores.insert('}', 3);
        scores.insert('>', 4);

        let mut completion_scores = self
            .chunks
            .iter()
            .flat_map(|ch| Day10::find_error(ch).ok())
            .map(|ch| ch.iter().rev().fold(0, |acc, &c| acc * 5 + scores[&c]))
            .collect::<Vec<_>>();

        completion_scores.sort_unstable();
        Ok(completion_scores[completion_scores.len() / 2])
    }
}

aoc_main!(Day10);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day10, 1, EXAMPLE, 26397);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day10, 2, EXAMPLE, 288957);
    }
}
