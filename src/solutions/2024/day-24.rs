use std::time;

use aoc_rust::*;
use common::*;

#[derive(Clone, PartialEq)]
enum Value {
    Value(bool),
    Gate(Gate),
}

impl Value {
    #[allow(dead_code)]
    fn print(&self, value_map: &HashMap<String, Value>, depth: usize) {
        match self {
            Value::Value(_v) => {}, //print!("{}", if *v { 1 } else { 0 }),
            Value::Gate(g) => match g {
                Gate::And(a, b, _) => {
                    print!("({}", a);
                    if depth < 3 {
                        value_map[a].print(value_map, depth + 1);
                    }
                    print!(" AND {}", b);
                    if depth < 3 {
                        value_map[b].print(value_map, depth + 1);
                    }
                    print!(")");
                },
                Gate::Or(a, b, _) => {
                    print!("({}", a);
                    if depth < 3 {
                        value_map[a].print(value_map, depth + 1);
                    }
                    print!(" OR {}", b);
                    if depth < 3 {
                        value_map[b].print(value_map, depth + 1);
                    }
                    print!(")");
                },
                Gate::Xor(a, b, _) => {
                    print!("({}", a);
                    if depth < 3 {
                        value_map[a].print(value_map, depth + 1);
                    }
                    print!(" XOR {}", b);
                    if depth < 3 {
                        value_map[b].print(value_map, depth + 1);
                    }
                    print!(")");
                },
            },
        }
    }

    fn eval(&self, value_map: &HashMap<String, Value>) -> bool {
        match self {
            Value::Value(v) => *v,
            Value::Gate(g) => match g {
                Gate::And(a, b, _) => {
                    let a = value_map[a].eval(value_map);
                    let b = value_map[b].eval(value_map);
                    a & b
                },
                Gate::Or(a, b, _) => {
                    let a = value_map[a].eval(value_map);
                    let b = value_map[b].eval(value_map);
                    a | b
                },
                Gate::Xor(a, b, _) => {
                    let a = value_map[a].eval(value_map);
                    let b = value_map[b].eval(value_map);
                    a ^ b
                },
            },
        }
    }
}

#[derive(Clone)]
enum Gate {
    And(String, String, String),
    Or(String, String, String),
    Xor(String, String, String),
}

impl Gate {
    fn parse(input: &mut &str) -> PResult<Self> {
        let (a, _, b, _, c, _, d) = (
            alphanumeric1.map(String::from),
            space1,
            alpha1,
            space1,
            alphanumeric1.map(String::from),
            " -> ",
            alphanumeric1.map(String::from),
        )
            .parse_next(input)?;

        let gate = match b {
            "AND" => Gate::And(a, c, d),
            "OR" => Gate::Or(a, c, d),
            "XOR" => Gate::Xor(a, c, d),
            _ => panic!("Unknown gate type: {}", b),
        };
        Ok(gate)
    }

    fn result(&self) -> &str {
        match self {
            Gate::And(_, _, res) => res,
            Gate::Or(_, _, res) => res,
            Gate::Xor(_, _, res) => res,
        }
    }
}

impl std::cmp::PartialEq for Gate {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Gate::And(a1, b1, _), Gate::And(a2, b2, _)) => {
                a1 == a2 && b1 == b2 || a1 == b2 && a2 == b1
            },
            (Gate::Or(a1, b1, _), Gate::Or(a2, b2, _)) => {
                a1 == a2 && b1 == b2 || a1 == b2 && a2 == b1
            },
            (Gate::Xor(a1, b1, _), Gate::Xor(a2, b2, _)) => {
                a1 == a2 && b1 == b2 || a1 == b2 && a2 == b1
            },
            _ => false,
        }
    }
}

impl std::fmt::Debug for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gate::And(a, b, c) => write!(f, "{} AND {} -> {}", a, b, c),
            Gate::Or(a, b, c) => write!(f, "{} OR {} -> {}", a, b, c),
            Gate::Xor(a, b, c) => write!(f, "{} XOR {} -> {}", a, b, c),
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Value(v) => write!(f, "{}", if *v { 1 } else { 0 }),
            Value::Gate(g) => write!(f, "{:?}", g),
        }
    }
}

struct Day24 {
    values: HashMap<String, Value>,
}

impl Day24 {
    fn get_value(&self, name: &str) -> u64 {
        let names: Vec<&String> = self
            .values
            .keys()
            .filter(|&s| s.starts_with(name))
            .sorted()
            .rev()
            .collect();

        let mut result = 0;
        for name in names {
            let value = &self.values[name];
            result <<= 1;
            result |= value.eval(&self.values) as u64;
        }

        result
    }

    fn swap(&mut self, a: &str, b: &str) {
        let val_a = self.values.remove(a).unwrap();
        let val_b = self.values.remove(b).unwrap();
        self.values.insert(a.to_string(), val_b);
        self.values.insert(b.to_string(), val_a);
    }
}

impl Problem<u64, String> for Day24 {
    fn parse(input: &mut &str) -> PResult<Self> {
        let mut values: HashMap<String, Value> = list(
            separated_pair(
                alphanumeric1.map(String::from),
                ": ",
                one_of(('0', '1')).map(|c| c == '1'),
            )
            .map(|(name, value)| (name, Value::Value(value))),
            line_ending,
        )
        .map(|values| values.into_iter().collect())
        .parse_next(input)?;

        let _ = line_ending.parse_next(input)?;
        let _ = line_ending.parse_next(input)?;

        let gates = list(Gate::parse, line_ending).parse_next(input)?;
        for gate in gates {
            let res = gate.result();
            values.insert(res.to_string(), Value::Gate(gate));
        }

        Ok(Day24 { values })
    }

    fn part1(self) -> Result<u64> {
        Ok(self.get_value("z"))
    }

    fn part2(mut self) -> Result<String> {
        let mut swaps: Vec<(String, String)> = Vec::new();

        while swaps.len() < 4 {
            for v in self.values.values_mut() {
                if let Value::Value(v) = v {
                    let nanos = time::SystemTime::now()
                        .duration_since(time::UNIX_EPOCH)
                        .unwrap()
                        .subsec_nanos()
                        .count_ones();
                    *v = nanos % 2 == 1;
                }
            }

            let x = self.get_value("x");
            let y = self.get_value("y");
            let z = self.get_value("z");

            // println!("x     : {:64b}", x);
            // println!("y     : {:64b}", y);
            // println!("x + y : {:64b}", x + y);
            // println!("z     : {:64b}", z);
            // println!("diff  : {:64b}", (x + y) ^ z);

            for i in 0..64 {
                if ((x + y) ^ z) & (1 << i) == 0 {
                    continue;
                }

                let xi = format!("x{:02}", i);
                let yi = format!("y{:02}", i);
                let zi = format!("z{:02}", i);

                let (xi_xor_yi, _) = self
                    .values
                    .iter()
                    .find(|&(_, v)| {
                        v == &Value::Gate(Gate::Xor(xi.clone(), yi.clone(), zi.clone()))
                    })
                    .unwrap();

                let gate_zi = if let Value::Gate(g) = &self.values[&zi] {
                    g
                } else {
                    return Err(format!("{} is not a gate", zi))?;
                };

                if let Gate::Xor(a, b, _) = gate_zi {
                    if a == xi_xor_yi || b == xi_xor_yi {
                        return Err("not implemented")?;
                    } else {
                        let incorrect = if let Value::Gate(Gate::Or(..)) = &self.values[a] {
                            b
                        } else if let Value::Gate(Gate::Or(..)) = &self.values[b] {
                            a
                        } else {
                            return Err("Could not solve")?;
                        };
                        swaps.push((xi_xor_yi.clone(), incorrect.clone()));
                    };
                } else {
                    let true_zi = self
                        .values
                        .iter()
                        .find(|&(_, v)| {
                            if let Value::Gate(Gate::Xor(a, b, _)) = v {
                                if a == xi_xor_yi || b == xi_xor_yi {
                                    return true;
                                }
                            }
                            false
                        })
                        .map(|(k, _)| k.clone())
                        .unwrap();
                    self.swap(&zi, &true_zi);
                    swaps.push((zi, true_zi));
                }

                break;
            }
        }

        let swapped = swaps
            .into_iter()
            .flat_map(|(a, b)| vec![a, b])
            .sorted()
            .collect::<Vec<_>>();
        Ok(swapped.join(","))
    }
}

aoc_main!(Day24);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
"#;

    const EXAMPLE2: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"#;

    #[test]
    fn test_part1() {
        assert_task!(Day24, 1, EXAMPLE1, 4);
        assert_task!(Day24, 1, EXAMPLE2, 2024);
    }
}
