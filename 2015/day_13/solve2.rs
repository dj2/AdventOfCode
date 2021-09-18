#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

extern crate itertools;
extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn process(input: &str) -> i32 {
    let mut people: Vec<String> = Vec::new();
    let mut data: Vec<HashMap<String, i32>> = Vec::new();

    people.push("me".to_string());
    data.push(HashMap::new());

    for line in input.lines() {
        let (from, to, val) = process_line(line);
        if people.last() != Some(&from) {
            people.push(from.clone());
            data.push(HashMap::new());

            data[0].insert(from, 0);
            data.last_mut().unwrap().insert("me".to_string(), 0);
        }

        data.last_mut().unwrap().insert(to, val);
    }

    let permutations = (0..people.len()).permutations(people.len());
    let mut highest: i32 = -999999;
    for permutation in permutations {
        let mut sum: i32 = 0;

        for idx in 0..permutation.len() {
            let left = if idx == 0 {
                permutation.len() - 1
            } else {
                idx - 1
            };
            let right = if idx == permutation.len() - 1 {
                0
            } else {
                idx + 1
            };

            let left = &people[permutation[left]];
            let right = &people[permutation[right]];

            sum += data[permutation[idx]][left];
            sum += data[permutation[idx]][right];
        }
        if sum > highest {
            highest = sum
        }
    }
    highest
}

fn cap_to_str(cap: Option<regex::Match<'_>>) -> String {
    cap.unwrap().as_str().to_string()
}
fn cap_to_val(cap: Option<regex::Match<'_>>) -> i32 {
    cap.unwrap().as_str().parse().unwrap()
}

fn process_line(input: &str) -> (String, String, i32) {
    let line_re =
        Regex::new(r"^(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).$").unwrap();

    let caps = line_re.captures(input).unwrap();
    let from = cap_to_str(caps.get(1));
    let dir = cap_to_str(caps.get(2));
    let mut val = cap_to_val(caps.get(3));
    let to = cap_to_str(caps.get(4));

    if dir == "lose" {
        val *= -1;
    }

    (from, to, val)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing filename argument");
    }
    let filename = &args[1];

    let data =
        std::fs::read_to_string(filename).unwrap_or_else(|_| panic!("Unable to read {}", filename));
    println!("{}", process(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_gain() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.";
        assert_eq!(
            ("Alice".to_string(), "Bob".to_string(), 54),
            process_line(input)
        );
    }

    #[test]
    fn parse_loss() {
        let input = "Alice would lose 2 happiness units by sitting next to David.";
        assert_eq!(
            ("Alice".to_string(), "David".to_string(), -2),
            process_line(input)
        );
    }
}
