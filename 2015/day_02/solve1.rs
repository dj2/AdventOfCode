fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument");
    }

    let filename = &args[1];
    let data =
        std::fs::read_to_string(filename).expect(format!("Unable to read: {}", filename).as_str());

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

    let l: i32 = parts[0].parse().unwrap();
    let w: i32 = parts[1].parse().unwrap();
    let h: i32 = parts[2].parse().unwrap();

    let a = l * w;
    let b = w * h;
    let c = h * l;

    let m = std::cmp::min(a, std::cmp::min(b, c));
    (2 * a) + (2 * b) + (2 * c) + m
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
