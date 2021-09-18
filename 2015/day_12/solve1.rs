#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

extern crate regex;

use regex::Regex;

fn cap_to_val(cap: Option<regex::Match<'_>>) -> i32 {
    cap.unwrap().as_str().parse().unwrap()
}

fn process(input: &str) -> i32 {
    let digit_re = Regex::new(r"(-?\d+)").unwrap();
    let mut sum = 0;
    for caps in digit_re.captures_iter(input) {
        sum += cap_to_val(caps.get(1));
    }
    sum
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
        assert_eq!(6, process("[1,2,3]"));
    }

    #[test]
    fn ex2() {
        assert_eq!(6, process(r#"{"a":2,"b":4}"#));
    }

    #[test]
    fn ex3() {
        assert_eq!(3, process("[[[3]]]"));
    }

    #[test]
    fn ex4() {
        assert_eq!(3, process(r#"{"a":{"b":4},"c":-1}"#));
    }

    #[test]
    fn ex5() {
        assert_eq!(0, process(r#"{"a":[-1,1]}"#));
    }

    #[test]
    fn ex6() {
        assert_eq!(0, process(r#"[-1,{"a":1}]"#));
    }

    #[test]
    fn ex7() {
        assert_eq!(0, process("[]"));
    }

    #[test]
    fn ex8() {
        assert_eq!(0, process("{}"));
    }
}
