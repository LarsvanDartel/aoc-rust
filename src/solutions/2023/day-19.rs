use aoc_rust::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending, one_of},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    Parser,
};
use std::collections::HashMap;
struct Day19 {
    parts: Vec<Part>,
    workflows: HashMap<String, Vec<Rule>>,
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn parse(input: &str) -> ParseResult<Self> {
        delimited(
            tag("{"),
            tuple((
                preceded(tag("x="), digit1.map(|s: &str| s.parse::<usize>().unwrap())),
                preceded(
                    tag(",m="),
                    digit1.map(|s: &str| s.parse::<usize>().unwrap()),
                ),
                preceded(
                    tag(",a="),
                    digit1.map(|s: &str| s.parse::<usize>().unwrap()),
                ),
                preceded(
                    tag(",s="),
                    digit1.map(|s: &str| s.parse::<usize>().unwrap()),
                ),
            )),
            tag("}"),
        )
        .map(|(x, m, a, s)| Self { x, m, a, s })
        .parse(input)
    }

    fn apply_rules(&self, workflow: Vec<Rule>) -> Rule {
        for rule in workflow {
            if let Some(rule) = rule.apply(self) {
                return rule;
            }
        }
        unreachable!()
    }

    fn apply_workflows(&self, workflows: &HashMap<String, Vec<Rule>>) -> bool {
        let mut rules = workflows.get("in").unwrap().clone();
        let mut rule = self.apply_rules(rules);

        while let Rule::Workflow(workflow) = rule {
            rules = workflows.get(&workflow).unwrap().clone();
            rule = self.apply_rules(rules);
        }

        if let Rule::Result(result) = rule {
            result
        } else {
            unreachable!()
        }
    }
}

#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn size(&self) -> usize {
        self.end - self.start + 1
    }

    fn split(&self, value: usize, upper: bool) -> (Range, Range) {
        assert!(self.start < value && value < self.end);
        (
            Range {
                start: self.start,
                end: if upper { value - 1 } else { value },
            },
            Range {
                start: if upper { value } else { value + 1 },
                end: self.end,
            },
        )
    }
}

struct RangePart {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl RangePart {
    fn size(&self) -> usize {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }

    fn apply_workflow(&self, workflows: &HashMap<String, Vec<Rule>>, workflow: &String) -> usize {
        let rules = workflows.get(workflow).unwrap();

        for rule in rules {
            match rule.apply_range(self) {
                Ok(None) => continue,
                Ok(Some(Rule::Result(true))) => return self.size(),
                Ok(Some(Rule::Result(false))) => return 0,
                Ok(Some(Rule::Workflow(w))) => return self.apply_workflow(workflows, &w),
                Ok(_) => unreachable!(),
                Err((attr, (r1, r2))) => {
                    let (r1, r2) = match attr {
                        'x' => (RangePart { x: r1, ..*self }, RangePart { x: r2, ..*self }),
                        'm' => (RangePart { m: r1, ..*self }, RangePart { m: r2, ..*self }),
                        'a' => (RangePart { a: r1, ..*self }, RangePart { a: r2, ..*self }),
                        's' => (RangePart { s: r1, ..*self }, RangePart { s: r2, ..*self }),
                        _ => unreachable!(),
                    };
                    return r1.apply_workflow(workflows, &workflow)
                        + r2.apply_workflow(workflows, &workflow);
                }
            }
        }
        unreachable!()
    }
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}..{}]", self.start, self.end)
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Condition {
        attribute: char,
        less_than: bool,
        value: usize,
        result: Box<Rule>,
    },
    Workflow(String),
    Result(bool),
}

impl Rule {
    fn parse_result(input: &str) -> ParseResult<Self> {
        alt((
            tag("R").map(|_| Rule::Result(false)),
            tag("A").map(|_| Rule::Result(true)),
        ))
        .parse(input)
    }

    fn parse_workflow(input: &str) -> ParseResult<Self> {
        alpha1
            .map(|s: &str| Rule::Workflow(s.to_string()))
            .parse(input)
    }

    fn parse_condition(input: &str) -> ParseResult<Self> {
        tuple((
            one_of("xmas"),
            one_of("<>").map(|c| c == '<'),
            digit1.map(|s: &str| s.parse::<usize>().unwrap()),
            tag(":"),
            alt((Self::parse_result, Self::parse_workflow)),
        ))
        .map(|(c, l, v, _, r)| Rule::Condition {
            attribute: c,
            less_than: l,
            value: v,
            result: Box::new(r),
        })
        .parse(input)
    }

    fn parse(input: &str) -> ParseResult<Self> {
        alt((
            Self::parse_condition,
            Self::parse_result,
            Self::parse_workflow,
        ))
        .parse(input)
    }

    fn apply(&self, part: &Part) -> Option<Self> {
        match self {
            Rule::Condition {
                attribute,
                less_than,
                value,
                result,
            } => {
                let attribute = match attribute {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => unreachable!(),
                };
                if (*less_than && attribute < *value) || (!*less_than && attribute > *value) {
                    Some((**result).clone())
                } else {
                    None
                }
            }
            Rule::Workflow(_) => Some(self.clone()),
            Rule::Result(_) => Some(self.clone()),
        }
    }

    fn apply_range(
        &self,
        r: &RangePart,
    ) -> std::result::Result<Option<Self>, (char, (Range, Range))> {
        match self {
            Rule::Condition {
                attribute,
                less_than,
                value,
                result,
            } => {
                let a = match attribute {
                    'x' => &r.x,
                    'm' => &r.m,
                    'a' => &r.a,
                    's' => &r.s,
                    _ => unreachable!(),
                };
                if *less_than {
                    if *value <= a.start {
                        return Ok(None);
                    }
                    if *value > a.end {
                        return Ok(Some((**result).clone()));
                    }
                    return Err((*attribute, a.split(*value, true)));
                } else {
                    if *value >= a.end {
                        return Ok(None);
                    }
                    if *value < a.start {
                        return Ok(Some((**result).clone()));
                    }
                    return Err((*attribute, a.split(*value, false)));
                }
            }
            Rule::Workflow(_) => Ok(Some(self.clone())),
            Rule::Result(_) => Ok(Some(self.clone())),
        }
    }
}

impl Problem<usize, usize> for Day19 {
    fn parse(input: &str) -> ParseResult<Self> {
        let workflow = pair(
            alpha1.map(|s: &str| s.to_string()),
            delimited(tag("{"), separated_list1(tag(","), Rule::parse), tag("}")),
        );

        separated_pair(
            separated_list1(line_ending, workflow)
                .map(|v| v.into_iter().collect::<HashMap<_, _>>()),
            line_ending.and(line_ending),
            separated_list1(line_ending, Part::parse),
        )
        .map(|(workflows, parts)| Self { workflows, parts })
        .parse(input)
    }

    fn part1(self) -> Result<usize> {
        Ok(self
            .parts
            .iter()
            .filter(|part| part.apply_workflows(&self.workflows))
            .map(|part| part.x + part.m + part.a + part.s)
            .sum())
    }

    fn part2(self) -> Result<usize> {
        Ok(RangePart {
            x: Range {
                start: 1,
                end: 4000,
            },
            m: Range {
                start: 1,
                end: 4000,
            },
            a: Range {
                start: 1,
                end: 4000,
            },
            s: Range {
                start: 1,
                end: 4000,
            },
        }
        .apply_workflow(&self.workflows, &String::from("in")))
    }
}

aoc_main!(Day19);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    #[test]
    fn test_part1() {
        assert_task!(Day19, 1, EXAMPLE, 19114);
    }

    #[test]
    fn test_part2() {
        assert_task!(Day19, 2, EXAMPLE, 167409079868000usize);
    }
}
