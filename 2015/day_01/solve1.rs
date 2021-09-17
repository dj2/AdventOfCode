#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Missing input filename");
        return;
    }
    let filename = &args[1];

    let data =
        std::fs::read_to_string(filename).unwrap_or_else(|_| panic!("Unable to read {}", filename));

    println!("{}", process(&data));
}

fn process(input: &str) -> isize {
    let mut floor = 0;
    for c in input.chars() {
        match c {
            ')' => floor -= 1,
            '(' => floor += 1,
            _ => panic!("invalid character"),
        }
    }
    floor
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(process("(())"), 0);
    }

    #[test]
    fn ex2() {
        assert_eq!(process("((("), 3);
    }

    #[test]
    fn ex3() {
        assert_eq!(process("(()(()("), 3);
    }

    #[test]
    fn ex4() {
        assert_eq!(process("))((((("), 3);
    }

    #[test]
    fn ex5() {
        assert_eq!(process("())"), -1);
    }

    #[test]
    fn ex6() {
        assert_eq!(process("))("), -1);
    }

    #[test]
    fn ex7() {
        assert_eq!(process(")))"), -3);
    }

    #[test]
    fn ex8() {
        assert_eq!(process(")())())"), -3);
    }
}
