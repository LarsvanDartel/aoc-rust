use aoc_rust::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    Parser,
};

use std::collections::{HashMap, VecDeque};

struct Day20 {
    modules: HashMap<String, Module>,
    inverse: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    r#type: ModuleType,
    connections: Vec<String>,
}

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, (bool, usize)>),
    None,
}

impl Module {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_pair(
            alt((
                tag("broadcaster").map(|name: &str| (name.to_string(), ModuleType::Broadcaster)),
                preceded(tag("%"), alpha1)
                    .map(|name: &str| (name.to_string(), ModuleType::FlipFlop(false))),
                preceded(tag("&"), alpha1)
                    .map(|name: &str| (name.to_string(), ModuleType::Conjunction(HashMap::new()))),
            )),
            tag(" -> "),
            separated_list1(tag(", "), alpha1.map(|name: &str| name.to_string())),
        )
        .map(|((name, r#type), connections)| Module {
            name,
            r#type,
            connections,
        })
        .parse(input)
    }

    fn process(&mut self, from: &String, value: bool) -> Vec<(String, bool)> {
        match &mut self.r#type {
            ModuleType::Broadcaster => self
                .connections
                .iter()
                .map(|connection| (connection.clone(), value))
                .collect(),
            ModuleType::FlipFlop(cur_state) => {
                if value {
                    vec![]
                } else {
                    *cur_state = !*cur_state;
                    self.connections
                        .iter()
                        .map(|connection| (connection.clone(), *cur_state))
                        .collect()
                }
            }
            ModuleType::Conjunction(counts) => {
                let (cur_state, count) = counts.get_mut(from).unwrap();
                if *cur_state != value {
                    *cur_state = value;
                    *count = 0;
                }
                *count += 1;
                let signal = counts.values().any(|(state, _)| !*state);
                self.connections
                    .iter()
                    .map(|connection| (connection.clone(), signal))
                    .collect()
            }
            ModuleType::None => vec![],
        }
    }
}

impl Problem<usize, usize> for Day20 {
    fn parse(input: &str) -> ParseResult<Self> {
        let (input, modules) = separated_list1(newline, Module::parse).parse(input)?;

        let mut modules = modules
            .into_iter()
            .map(|module| (module.name.clone(), module))
            .collect::<HashMap<_, _>>();

        // add all missing connections
        let mut missing = vec![];
        for module in modules.values() {
            for connection in module.connections.iter() {
                if !modules.contains_key(connection) {
                    missing.push(connection.clone());
                }
            }
        }

        for name in missing {
            modules.insert(
                name.clone(),
                Module {
                    name,
                    r#type: ModuleType::None,
                    connections: vec![],
                },
            );
        }

        // Add all connection to conjunctions (for all modules that have a connection to a conjunction, add their name to the conjunction)
        let mut inverse = HashMap::new();
        let mut missing = vec![];
        for module in modules.values() {
            for connection in module.connections.iter() {
                inverse
                    .entry(connection.clone())
                    .or_insert_with(Vec::new)
                    .push(module.name.clone());
                if let ModuleType::Conjunction(_) = &modules.get(connection).unwrap().r#type {
                    missing.push((connection.clone(), module.name.clone()));
                }
            }
        }

        for (connection, name) in missing {
            let module = modules.get_mut(&connection).unwrap();
            if let ModuleType::Conjunction(counts) = &mut module.r#type {
                counts.insert(name, (false, 0));
            }
        }

        Ok((input, Self { modules, inverse }))
    }

    fn part1(self) -> Result<usize> {
        let mut modules = self.modules.clone();

        let mut cnt = [0, 0];
        let mut queue = VecDeque::new();

        for _ in 0..1000 {
            queue.push_back((String::from("broadcaster"), String::from("button"), false));

            while let Some((to, from, value)) = queue.pop_front() {
                cnt[value as usize] += 1;
                let module = modules.get_mut(&to).unwrap();
                let new_signals = module.process(&from, value);
                for (to, value) in new_signals {
                    queue.push_back((to, module.name.clone(), value));
                }
            }
        }

        Ok(cnt[0] * cnt[1])
    }

    fn part2(self) -> Result<usize> {
        let rx_in = self.inverse.get("rx").unwrap();
        assert!(rx_in.len() == 1);
        let rx_in = rx_in[0].clone();
        let mut watch = self
            .inverse
            .get(&rx_in)
            .unwrap()
            .iter()
            .map(|name| (name.clone(), (None, None)))
            .collect::<HashMap<_, _>>();

        let mut modules = self.modules;
        let mut queue = VecDeque::new();
        'main: for t in 0..u32::MAX {
            queue.push_back((String::from("broadcaster"), String::from("button"), false));

            while let Some((to, from, value)) = queue.pop_front() {
                if let Some((first, second)) = watch.get_mut(&to) {
                    if !value {
                        if *first == None {
                            *first = Some(t);
                        } else if *second == None {
                            *second = Some(t);
                            if watch
                                .values()
                                .all(|(first, second)| first.is_some() && second.is_some())
                            {
                                break 'main;
                            }
                        }
                    }
                }

                let module = modules.get_mut(&to).unwrap();
                let new_signals = module.process(&from, value);
                for (to, value) in new_signals {
                    queue.push_back((to, module.name.clone(), value));
                }
            }
        }

        use num_integer::lcm;
        Ok(watch
            .values()
            .map(|(first, second)| (second.unwrap() - first.unwrap()) as usize)
            .fold(1, lcm))
    }
}

aoc_main!(Day20);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

    const EXAMPLE_2: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

    #[test]
    fn test_part1() {
        assert_task!(Day20, 1, EXAMPLE_1, 32000000);
        assert_task!(Day20, 1, EXAMPLE_2, 11687500);
    }

    //#[test]
    //fn test_part2() {
    //}
}
