#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

use std::collections::HashSet;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument");
    }
    let filename = &args[1];

    let data =
        std::fs::read_to_string(filename).unwrap_or_else(|_| panic!("Unable to read {}", filename));
    println!("{}", calc(&data))
}

fn calc(input: &str) -> i32 {
    struct Data {
        x: i32,
        y: i32,
    }
    let mut pos = Data { x: 0, y: 0 };

    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    seen.insert((pos.x, pos.y));

    for c in input.chars() {
        match c {
            '>' => pos.x += 1,
            '<' => pos.x -= 1,
            '^' => pos.y += 1,
            'v' => pos.y -= 1,
            _ => panic!("Invalid input {}", c),
        }

        seen.insert((pos.x, pos.y));
    }

    seen.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(calc(">"), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(calc("^>v<"), 4);
    }

    #[test]
    fn ex3() {
        assert_eq!(calc("^v^v^v^v^v"), 2);
    }
}
