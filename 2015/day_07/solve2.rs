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

    fn process(&mut self, cmds: &[Command]) {
        for c in cmds {
            self.wires.insert(c.dest.clone(), c.input.clone());
        }
    }

    fn overwrite(&mut self, wire: &str, val: u16) {
        self.wires
            .insert(wire.to_string(), Input::Value(Signal::Value(val)));
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
                res.push(Command { dest, input: v });
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
    let value_match = || -> Option<Signal> {
        value_re
            .captures(input)
            .map(|cap| Signal::Value(cap_to_val(cap.get(1))))
    };

    let wire_re = Regex::new(r"^\s*(\w+)\s*$").unwrap();
    let wire_match = || -> Option<Signal> {
        wire_re
            .captures(input)
            .map(|cap| Signal::Wire(cap_to_str(cap.get(1))))
    };

    let sig = value_match().or_else(wire_match);
    match sig {
        Some(s) => s,
        None => panic!("Invalid input {}", input),
    }
}

fn parse_input(input: &str) -> Option<Input> {
    let signal_match = || -> Option<Input> { Some(Input::Value(parse_signal(input))) };

    let and_re = Regex::new(r"^\s*(\w+)\s+AND\s+(\w+)\s*$").unwrap();
    let and_match = || -> Option<Input> {
        and_re.captures(input).map(|cap| {
            Input::And(
                parse_signal(&cap_to_str(cap.get(1))),
                parse_signal(&cap_to_str(cap.get(2))),
            )
        })
    };

    let or_re = Regex::new(r"^\s*(\w+)\s+OR\s+(\w+)\s*$").unwrap();
    let or_match = || -> Option<Input> {
        or_re.captures(input).map(|cap| {
            Input::Or(
                parse_signal(&cap_to_str(cap.get(1))),
                parse_signal(&cap_to_str(cap.get(2))),
            )
        })
    };

    let not_re = Regex::new(r"^\s*NOT\s+(\w+)\s*$").unwrap();
    let not_match = || -> Option<Input> {
        not_re
            .captures(input)
            .map(|cap| Input::Not(parse_signal(&cap_to_str(cap.get(1)))))
    };

    let lshift_re = Regex::new(r"^\s*(\w+)\s+LSHIFT\s+(\d+)\s*$").unwrap();
    let lshift_match = || -> Option<Input> {
        lshift_re.captures(input).map(|cap| {
            Input::LShift(
                parse_signal(&cap_to_str(cap.get(1))),
                cap_to_val(cap.get(2)),
            )
        })
    };

    let rshift_re = Regex::new(r"^\s*(\w+)\s+RSHIFT\s+(\d+)\s*$").unwrap();
    let rshift_match = || -> Option<Input> {
        rshift_re.captures(input).map(|cap| {
            Input::RShift(
                parse_signal(&cap_to_str(cap.get(1))),
                cap_to_val(cap.get(2)),
            )
        })
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

    let data =
        std::fs::read_to_string(filename).unwrap_or_else(|_| panic!("Unable to read {}", filename));
    let cmds = parse(&data);

    let mut c = Circuit::new();
    c.process(&cmds);
    c.overwrite("b", 956);
    println!("{}", c.calc_val("a"));
}
