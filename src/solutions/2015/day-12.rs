use aoc_rust::*;

use nom::{
    branch::alt,
    bytes::complete::take_until,
    character::complete::{char as ch, i32 as number},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    Parser,
};
use std::collections::HashMap;

struct Day12 {
    root: Json,
}

enum Json {
    Number(i32),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

impl Json {
    fn parse(input: &str) -> ParseResult<Self> {
        fn parse_str(input: &str) -> ParseResult<String> {
            delimited(ch('"'), take_until("\""), ch('"'))
                .map(|s: &str| s.to_string())
                .parse(input)
        }
        alt((
            parse_str.map(Json::String),
            number.map(Json::Number),
            delimited(
                ch('['),
                separated_list1(ch(','), Json::parse).map(Json::Array),
                ch(']'),
            ),
            delimited(
                ch('{'),
                separated_list1(ch(','), separated_pair(parse_str, ch(':'), Json::parse))
                    .map(|pairs| Json::Object(pairs.into_iter().collect())),
                ch('}'),
            ),
        ))
        .parse(input)
    }
}

impl std::fmt::Debug for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Json::Number(n) => write!(f, "{}", n),
            Json::String(s) => write!(f, "\"{}\"", s),
            Json::Array(a) => {
                write!(f, "[")?;
                for (i, v) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", v)?;
                }
                write!(f, "]")
            }
            Json::Object(o) => {
                write!(f, "{{")?;
                for (i, (k, v)) in o.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {:?}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl Json {
    fn sum(&self, ignore: Option<&String>) -> i32 {
        match self {
            Json::Number(n) => *n,
            Json::String(_) => 0,
            Json::Array(a) => a.iter().map(|v| v.sum(ignore)).sum(),
            Json::Object(o) => {
                if let Some(ignore) = ignore {
                    if o.values().any(|v| match v {
                        Json::String(s) => s == ignore,
                        _ => false,
                    }) {
                        return 0;
                    }
                }
                o.values().map(|v| v.sum(ignore)).sum()
            }
        }
    }
}

impl Problem<i32, i32> for Day12 {
    fn parse(input: &str) -> ParseResult<Self> {
        Json::parse.map(|root| Self { root }).parse(input)
    }

    fn part1(self) -> Result<i32> {
        Ok(self.root.sum(None))
    }

    fn part2(self) -> Result<i32> {
        Ok(self.root.sum(Some(&"red".to_string())))
    }
}

aoc_main!(Day12);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"{"a":1,"b":[1,2,3],"c":"red","d":{"a":1,"b":2},"e":[1,"red",5]}"#;

    #[test]
    fn test_part1() {
        assert_task!(Day12, 1, EXAMPLE, 16);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day12, 2, EXAMPLE, 0);
    }
}
