#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

extern crate regex;
extern crate itertools;

use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

fn cap_to_str(cap: Option<regex::Match<'_>>) -> String {
    cap.unwrap().as_str().to_string()
}
fn cap_to_val(cap: Option<regex::Match<'_>>) -> usize {
    cap.unwrap().as_str().parse().unwrap()
}

fn process(input: &str) -> usize {
    let mut cities: Vec<String> = Vec::new();
    let mut dists: Vec<HashMap<String, usize>> = Vec::new();

    let line_re = Regex::new(r"^\s*(\w+)\s+to\s+(\w+)\s+=\s+(\d+)\s*$").unwrap();
    input.lines().for_each(|line| {
        let caps = line_re.captures(line).unwrap();

        let city1 = cap_to_str(caps.get(1));
        let city2 = cap_to_str(caps.get(2));
        let dist = cap_to_val(caps.get(3));

        let mut store = |city1: &String, city2: &String, dist: usize| {
            if let Some(idx) = cities.iter().position(|x| x == city1) {
                let map = dists.get_mut(idx).unwrap();
                map.insert(city2.to_string(), dist);
            } else {
                cities.push(city1.to_string());
                let mut hmap = HashMap::new();
                hmap.insert(city2.to_string(), dist);
                dists.push(hmap);
            }
        };
        store(&city1, &city2, dist);
        store(&city2, &city1, dist);
    });

    let permutations = (0..cities.len()).permutations(cities.len());
    let mut longest: usize = 0;
    for permutation in permutations {
        let mut sum: usize = 0;
        let mut iter = permutation.iter();
        let mut last = *iter.next().unwrap();
        for pos in iter {
            sum += dists[last][&cities[*pos]];
            last = *pos;
        }
        if sum > longest {
            longest = sum
        }
    }
    longest
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
    fn ex1() {
        let input = r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#;
        assert_eq!(982, process(input));
    }
}

