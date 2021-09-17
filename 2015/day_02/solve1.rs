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

    let dim1 = length * width;
    let dim2 = width * height;
    let dim3 = height * length;

    let min = std::cmp::min(dim1, std::cmp::min(dim2, dim3));
    (2 * dim1) + (2 * dim2) + (2 * dim3) + min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(58, calc_size("2x3x4"));
    }

    #[test]
    fn ex2() {
        assert_eq!(43, calc_size("1x1x10"));
    }
}
