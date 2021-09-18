#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

extern crate regex;
use regex::Regex;

fn cap_to_val(cap: Option<regex::Match<'_>>) -> usize {
    cap.unwrap().as_str().parse().unwrap()
}

fn process_line(input: &str, seconds: usize) -> usize {
    let line_re = Regex::new(r"fly (\d+) km/s for (\d+) seconds.* (\d+) seconds.").unwrap();
    let caps = line_re.captures(input).unwrap();

    let dist = cap_to_val(caps.get(1));
    let time = cap_to_val(caps.get(2));
    let rest = cap_to_val(caps.get(3));

    let mut tot = 0;
    let mut tick = 0;
    let mut travel_time = 0;
    loop {
        tot += dist;
        tick += 1;
        travel_time += 1;
        if travel_time >= time {
            travel_time = 0;
            tick += rest;
        }

        if tick >= seconds {
            break;
        }
    }
    tot
}

fn process(input: &str, seconds: usize) -> usize {
    input.lines().map(|l| process_line(l, seconds)).max().unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing filename argument");
    }
    let filename = &args[1];

    let data =
        std::fs::read_to_string(filename).unwrap_or_else(|_| panic!("Unable to read {}", filename));
    println!("{}", process(&data, 2503));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let input = r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        assert_eq!(1120, process(input, 1000));
    }

    #[test]
    fn test_process_line() {
        assert_eq!(1056, process("Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.", 1000));
    }
}
