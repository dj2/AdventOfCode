#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing input filename");
    }

    let filename = &args[1];
    let data =
        std::fs::read_to_string(filename).unwrap_or_else(|_| panic!("Unable to read {}", filename));

    let nice = data.lines().filter(|l| check(l)).count();
    println!("{}", nice);
}

fn check(input: &str) -> bool {
    let mut duplicate_pair = false;
    let mut pairs: HashMap<String, usize> = HashMap::new();

    let chars = input.as_bytes();
    for idx in 0..(chars.len() - 1) {
        let s = format!("{}{}", chars[idx], chars[idx + 1]);

        match pairs.get(&s) {
            Some(pos) => {
                if idx > pos + 1 {
                    duplicate_pair = true;
                    break;
                }
            }
            None => {
                pairs.insert(s, idx);
            }
        }
    }

    let mut has_double_letter = false;
    for (idx, c) in input.chars().enumerate() {
        if idx == 0 || idx == 1 {
            continue;
        }

        if c == input.chars().nth(idx - 2).unwrap() {
            has_double_letter = true;
            break;
        }
    }

    duplicate_pair && has_double_letter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(check("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn ex2() {
        assert!(check("xxyxx"));
    }

    #[test]
    fn ex3() {
        assert!(!check("uurcxstgmygtbstg"));
    }

    #[test]
    fn ex4() {
        assert!(!check("ieodomkazucvgmuy"));
    }
}
