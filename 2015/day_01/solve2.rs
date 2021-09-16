fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Missing input filename");
        return;
    }
    let filename = &args[1];

    let data = std::fs::read_to_string(filename)
        .expect(&format!("Unable to read {}", filename).to_string());

    let ret = process(&data);
    match ret {
        Some(v) => println!("{}", v),
        None => panic!("Failed to find value"),
    }
}

fn process(input: &str) -> Option<isize> {
    let mut floor = 0;
    for (idx, c) in input.chars().enumerate() {
        match c {
            ')' => floor -= 1,
            '(' => floor += 1,
            _ => panic!("invalid character"),
        }
        if floor == -1 {
            return Some((idx + 1) as isize);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(process("(())"), None);
    }

    #[test]
    fn ex2() {
        assert_eq!(process("((("), None);
    }

    #[test]
    fn ex3() {
        assert_eq!(process("(()(()("), None);
    }

    #[test]
    fn ex4() {
        assert_eq!(process("))((((("), Some(1));
    }

    #[test]
    fn ex5() {
        assert_eq!(process("())"), Some(3));
    }

    #[test]
    fn ex6() {
        assert_eq!(process("))("), Some(1));
    }

    #[test]
    fn ex7() {
        assert_eq!(process(")))"), Some(1));
    }

    #[test]
    fn ex8() {
        assert_eq!(process(")())())"), Some(1));
    }
}
