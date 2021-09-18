#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

use std::iter::FromIterator;

fn process(input: &str, times: usize) -> usize {
    let mut res = input.to_string();
    for _ in 0..times {
        res = process_line(&res);
    }
    res.len()
}

fn process_line(input: &str) -> String {
    let mut chars = input.chars();

    let mut result: Vec<char> = Vec::new();
    let mut last = chars.next().unwrap();
    let mut count = 1;
    for next in chars {
        if next == last {
            count += 1;
            continue;
        }
        result.push(char::from_digit(count, 10).unwrap());
        result.push(last);

        last = next;
        count = 1;
    }

    result.push(char::from_digit(count, 10).unwrap());
    result.push(last);
    String::from_iter(result)
}

fn main() {
    println!("{}", process("1113222113", 40));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn process_str() {
        assert_eq!(6, process("1", 5));
    }

    #[test]
    fn ex1() {
        assert_eq!("11", process_line("1"));
    }

    #[test]
    fn ex2() {
        assert_eq!("21", process_line("11"));
    }

    #[test]
    fn ex3() {
        assert_eq!("1211", process_line("21"));
    }

    #[test]
    fn ex4() {
        assert_eq!("111221", process_line("1211"));
    }

    #[test]
    fn ex5() {
        assert_eq!("312211", process_line("111221"));
    }
}

