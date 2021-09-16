extern crate regex;

use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum Signal {
    Value(u16),
    Wire(String),
}

#[derive(Clone, Debug, PartialEq)]
enum Input {
    Value(Signal),
    And(Signal, Signal),
    Not(Signal),
    Or(Signal, Signal),
    LShift(Signal, u16),
    RShift(Signal, u16),
}

struct Command {
    input: Input,
    dest: String,
}

struct Circuit {
    wires: HashMap<String, Input>,
}
impl Circuit {
    fn new() -> Self {
        Circuit {
            wires: HashMap::new(),
        }
    }

    fn process(&mut self, cmds: &Vec<Command>) {
        for c in cmds {
            self.wires.insert(c.dest.clone(), c.input.clone());
        }
    }

    fn calc_val(&mut self, wire: &str) -> u16 {
        let input = self.wires.get(wire).unwrap().clone();
        self.resolve_input(&input)
    }

    fn resolve_input(&mut self, input: &Input) -> u16 {
        let mut resolve_signal = |s: &Signal| match s {
            Signal::Wire(w) => {
                let val = self.resolve_input(&self.wires.get(w).unwrap().clone());
                self.wires
                    .insert(w.to_string(), Input::Value(Signal::Value(val)));
                val
            }
            Signal::Value(v) => *v,
        };

        match input {
            Input::Value(v) => resolve_signal(v),
            Input::And(a, b) => {
                let a = resolve_signal(a);
                let b = resolve_signal(b);
                a & b
            }
            Input::Or(a, b) => {
                let a = resolve_signal(a);
                let b = resolve_signal(b);
                a | b
            }
            Input::Not(a) => {
                let a = resolve_signal(a);
                !a
            }
            Input::LShift(a, b) => {
                let a = resolve_signal(a);
                a << b
            }
            Input::RShift(a, b) => {
                let a = resolve_signal(a);
                a >> b
            }
        }
    }
}

fn parse(input: &str) -> Vec<Command> {
    let mut res = Vec::new();
    for l in input.lines() {
        let parts: Vec<&str> = l.split(" -> ").collect();
        if parts.len() < 2 {
            panic!("Invalid line {}", l);
        }
        let dest = parts[1].to_string();

        let input = parse_input(&parts[0].to_string());
        match input {
            Some(v) => {
                res.push(Command {
                    dest: dest,
                    input: v,
                });
            }
            None => panic!("Unable to handle {}", l),
        }
    }
    res
}

fn cap_to_str(cap: Option<regex::Match>) -> String {
    cap.unwrap().as_str().to_string()
}
fn cap_to_val(cap: Option<regex::Match>) -> u16 {
    cap.unwrap().as_str().parse().unwrap()
}

fn parse_signal(input: &str) -> Signal {
    let value_re = Regex::new(r"^\s*(\d+)\s*$").unwrap();
    let value_match = |line: &str| -> Option<Signal> {
        match value_re.captures(line) {
            Some(cap) => Some(Signal::Value(cap_to_val(cap.get(1)))),
            None => None,
        }
    };

    let wire_re = Regex::new(r"^\s*(\w+)\s*$").unwrap();
    let wire_match = |line: &str| -> Option<Signal> {
        match wire_re.captures(line) {
            Some(cap) => Some(Signal::Wire(cap_to_str(cap.get(1)))),
            None => None,
        }
    };

    let sig = value_match(input).or(wire_match(input));
    match sig {
        Some(s) => s,
        None => panic!("Invalid input {}", input),
    }
}

fn parse_input(input: &str) -> Option<Input> {
    let signal_match = || -> Option<Input> { Some(Input::Value(parse_signal(input))) };

    let and_re = Regex::new(r"^\s*(\w+)\s+AND\s+(\w+)\s*$").unwrap();
    let and_match = || -> Option<Input> {
        match and_re.captures(input) {
            Some(cap) => Some(Input::And(
                parse_signal(&cap_to_str(cap.get(1))),
                parse_signal(&cap_to_str(cap.get(2))),
            )),
            None => None,
        }
    };

    let or_re = Regex::new(r"^\s*(\w+)\s+OR\s+(\w+)\s*$").unwrap();
    let or_match = || -> Option<Input> {
        match or_re.captures(input) {
            Some(cap) => Some(Input::Or(
                parse_signal(&cap_to_str(cap.get(1))),
                parse_signal(&cap_to_str(cap.get(2))),
            )),
            None => None,
        }
    };

    let not_re = Regex::new(r"^\s*NOT\s+(\w+)\s*$").unwrap();
    let not_match = || -> Option<Input> {
        match not_re.captures(input) {
            Some(cap) => Some(Input::Not(parse_signal(&cap_to_str(cap.get(1))))),
            None => None,
        }
    };

    let lshift_re = Regex::new(r"^\s*(\w+)\s+LSHIFT\s+(\d+)\s*$").unwrap();
    let lshift_match = || -> Option<Input> {
        match lshift_re.captures(input) {
            Some(cap) => Some(Input::LShift(
                parse_signal(&cap_to_str(cap.get(1))),
                cap_to_val(cap.get(2)),
            )),
            None => None,
        }
    };

    let rshift_re = Regex::new(r"^\s*(\w+)\s+RSHIFT\s+(\d+)\s*$").unwrap();
    let rshift_match = || -> Option<Input> {
        match rshift_re.captures(input) {
            Some(cap) => Some(Input::RShift(
                parse_signal(&cap_to_str(cap.get(1))),
                cap_to_val(cap.get(2)),
            )),
            None => None,
        }
    };

    and_match()
        .or_else(or_match)
        .or_else(not_match)
        .or_else(lshift_match)
        .or_else(rshift_match)
        .or_else(signal_match)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing input file");
    }
    let filename = &args[1];

    let data = std::fs::read_to_string(filename).expect(&format!("Unable to parse {}", filename));
    let cmds = parse(&data);

    let mut c = Circuit::new();
    c.process(&cmds);
    println!("{}", c.calc_val("a"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"#;

        let cmds = parse(input);

        let mut c = Circuit::new();
        c.process(&cmds);
        assert_eq!(c.calc_val("d"), 72);
        assert_eq!(c.calc_val("e"), 507);
        assert_eq!(c.calc_val("f"), 492);
        assert_eq!(c.calc_val("g"), 114);
        assert_eq!(c.calc_val("h"), 65412);
        assert_eq!(c.calc_val("i"), 65079);
        assert_eq!(c.calc_val("x"), 123);
        assert_eq!(c.calc_val("y"), 456);
    }

    #[test]
    fn ex1_unordered() {
        let input = r#"x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
123 -> x
456 -> y
x -> z"#;

        let cmds = parse(input);

        let mut c = Circuit::new();
        c.process(&cmds);
        assert_eq!(c.calc_val("d"), 72);
        assert_eq!(c.calc_val("e"), 507);
        assert_eq!(c.calc_val("f"), 492);
        assert_eq!(c.calc_val("g"), 114);
        assert_eq!(c.calc_val("h"), 65412);
        assert_eq!(c.calc_val("i"), 65079);
        assert_eq!(c.calc_val("x"), 123);
        assert_eq!(c.calc_val("y"), 456);
        assert_eq!(c.calc_val("z"), 123);
    }

    #[test]
    fn parse_empty() {
        let cmds = parse("");
        assert_eq!(cmds.len(), 0);
    }

    #[test]
    fn parse_value() {
        let cmds = parse("456 -> y");
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].dest, "y");
        assert_eq!(cmds[0].input, Input::Value(Signal::Value(456)));
    }

    #[test]
    fn parse_value_with_wire() {
        let cmds = parse("x -> y");
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].dest, "y");
        assert_eq!(cmds[0].input, Input::Value(Signal::Wire("x".to_string())));
    }

    #[test]
    fn parse_and() {
        let cmds = parse("x AND y -> z");
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].dest, "z");
        assert_eq!(
            cmds[0].input,
            Input::And(Signal::Wire("x".to_string()), Signal::Wire("y".to_string()))
        );
    }

    #[test]
    fn parse_and_val() {
        let cmds = parse("1 AND y -> z");
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].dest, "z");
        assert_eq!(
            cmds[0].input,
            Input::And(Signal::Value(1), Signal::Wire("y".to_string()))
        );
    }

    #[test]
    fn parse_or() {
        let cmds = parse("x OR y -> z");
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].dest, "z");
        assert_eq!(
            cmds[0].input,
            Input::Or(Signal::Wire("x".to_string()), Signal::Wire("y".to_string()))
        );
    }

    #[test]
    fn parse_not() {
        let cmds = parse("NOT x -> z");
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].dest, "z");
        assert_eq!(cmds[0].input, Input::Not(Signal::Wire("x".to_string())));
    }

    #[test]
    fn parse_lshift() {
        let cmds = parse("x LSHIFT 2 -> z");
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].dest, "z");
        assert_eq!(
            cmds[0].input,
            Input::LShift(Signal::Wire("x".to_string()), 2)
        );
    }

    #[test]
    fn parse_rshift() {
        let cmds = parse("x RSHIFT 4 -> z");
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].dest, "z");
        assert_eq!(
            cmds[0].input,
            Input::RShift(Signal::Wire("x".to_string()), 4)
        );
    }
}
