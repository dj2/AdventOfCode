#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

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

fn process(input: &str) -> usize {
    input.lines().map(|l| process_line(l)).sum()
}

fn process_line(input: &str) -> usize {
    let n: usize = input
        .chars()
        .map(|c| if matches!(c, '\\' | '"') { 2 } else { 1 })
        .sum();
    n + 2 - input.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all() {
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

        assert_eq!(19, process(&input));
    }

    #[test]
    fn ex1() {
        assert_eq!(4, process_line(r#""""#));
    }

    #[test]
    fn ex2() {
        assert_eq!(4, process_line(r#""abc""#));
    }

    #[test]
    fn ex3() {
        assert_eq!(6, process_line(r#""aaa\"aaa""#));
    }

    #[test]
    fn ex4() {
        assert_eq!(5, process_line(r#""\x27""#));
    }
}
