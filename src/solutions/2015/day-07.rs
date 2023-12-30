use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, u16 as number},
    multi::separated_list1,
    sequence::tuple,
    Parser,
};

use aoc_rust::*;

struct Day07 {
    wires: HashMap<String, Wire>,
}

#[derive(Clone, PartialEq, Eq)]
enum Wire {
    Source(Value),
    Not(Value),
    And(Value, Value),
    Or(Value, Value),
    LShift(Value, Value),
    RShift(Value, Value),
}

impl Wire {
    fn parse(input: &str) -> ParseResult<Self> {
        alt((
            tuple((tag("NOT "), Value::parse)).map(|(_, v)| Wire::Not(v)),
            tuple((Value::parse, tag(" AND "), Value::parse)).map(|(v0, _, v1)| Wire::And(v0, v1)),
            tuple((Value::parse, tag(" OR "), Value::parse)).map(|(v0, _, v1)| Wire::Or(v0, v1)),
            tuple((Value::parse, tag(" LSHIFT "), Value::parse))
                .map(|(v0, _, v1)| Wire::LShift(v0, v1)),
            tuple((Value::parse, tag(" RSHIFT "), Value::parse))
                .map(|(v0, _, v1)| Wire::RShift(v0, v1)),
            Value::parse.map(Wire::Source),
        ))
        .parse(input)
    }
}

impl std::fmt::Debug for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Wire::Source(v) => write!(f, "{:?}", v),
            Wire::Not(v) => write!(f, "NOT {:?}", v),
            Wire::And(v0, v1) => write!(f, "{:?} AND {:?}", v0, v1),
            Wire::Or(v0, v1) => write!(f, "{:?} OR {:?}", v0, v1),
            Wire::LShift(v0, v1) => write!(f, "{:?} LSHIFT {:?}", v0, v1),
            Wire::RShift(v0, v1) => write!(f, "{:?} RSHIFT {:?}", v0, v1),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Value {
    Value(u16),
    Wire(String),
}

impl Value {
    fn parse(input: &str) -> ParseResult<Self> {
        alt((
            number.map(Value::Value),
            alpha1.map(|s: &str| Value::Wire(s.to_string())),
        ))
        .parse(input)
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Value(n) => write!(f, "{}", n),
            Value::Wire(s) => write!(f, "{}", s),
        }
    }
}

impl Day07 {
    fn eval(&self, wire_name: &str, values: &mut HashMap<String, u16>) -> Result<u16> {
        if let Some(value) = values.get(wire_name) {
            return Ok(*value);
        }

        let wire = self
            .wires
            .get(wire_name)
            .ok_or(format!("wire {} not found", wire_name))?;
        let value = match wire {
            Wire::Source(v) => self.eval_value(v, values)?,
            Wire::Not(v) => !self.eval_value(v, values)?,
            Wire::And(v0, v1) => self.eval_value(v0, values)? & self.eval_value(v1, values)?,
            Wire::Or(v0, v1) => self.eval_value(v0, values)? | self.eval_value(v1, values)?,
            Wire::LShift(v0, v1) => self.eval_value(v0, values)? << self.eval_value(v1, values)?,
            Wire::RShift(v0, v1) => self.eval_value(v0, values)? >> self.eval_value(v1, values)?,
        };
        values.insert(wire_name.to_string(), value);
        Ok(value)
    }

    fn eval_value(&self, value: &Value, values: &mut HashMap<String, u16>) -> Result<u16> {
        match value {
            Value::Value(n) => Ok(*n),
            Value::Wire(w) => self.eval(w, values),
        }
    }
}

impl Problem<u16, u16> for Day07 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, tuple((Wire::parse, tag(" -> "), alpha1)))
            .map(|wires| {
                let wires = wires
                    .into_iter()
                    .map(|(op, _, name)| (name.to_string(), op))
                    .collect();
                Self { wires }
            })
            .parse(input)
    }

    fn part1(self) -> Result<u16> {
        let mut values = HashMap::new();
        self.eval("a", &mut values)
    }

    fn part2(mut self) -> Result<u16> {
        let mut values = HashMap::new();
        let a = self.eval("a", &mut values)?;
        self.wires
            .insert("b".to_string(), Wire::Source(Value::Value(a)));
        values.clear();
        self.eval("a", &mut values)
    }
}

aoc_main!(Day07);
