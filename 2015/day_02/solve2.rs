fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument");
    }

    let filename = &args[1];
    let data = std::fs::read_to_string(filename)
            .expect(format!("Unable to read: {}", filename).as_str());

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

    let mut vec = vec![l, w, h];
    vec.sort();

    let a = vec[0];
    let b = vec[1];

    a + a + b + b + (l * w * h)
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
