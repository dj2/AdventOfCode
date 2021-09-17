#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument");
    }

    let filename = &args[1];
    let data =
        std::fs::read_to_string(filename).unwrap_or_else(|_| panic!("Unable to read {}", filename));

    let mut sum = 0;
    for p in data.lines() {
        sum += calc_size(p);
    }

    println!("{}", sum);
}

fn calc_size(input: &str) -> i32 {
    let parts: Vec<&str> = input.split('x').collect();

    if parts.len() < 3 {
        panic!("Invalid input {}", input);
    }

    let length: i32 = parts[0].parse().unwrap();
    let width: i32 = parts[1].parse().unwrap();
    let height: i32 = parts[2].parse().unwrap();

    let mut vec = vec![length, width, height];
    vec.sort_unstable();

    let dim1 = vec[0];
    let dim2 = vec[1];

    dim1 + dim1 + dim2 + dim2 + (length * width * height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(34, calc_size("2x3x4"));
    }

    #[test]
    fn ex2() {
        assert_eq!(14, calc_size("1x1x10"));
    }
}
