#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

fn process(_input: &str) -> usize {
    0
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
        assert_eq!(true, false);
    }
}
