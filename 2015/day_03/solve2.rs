use std::collections::HashSet;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument");
    }
    let filename = &args[1];

    let data = std::fs::read_to_string(filename)
            .expect(format!("Unable to read {}", filename).as_str());
    println!("{}", calc(&data))
}

fn calc(input: &str) -> i32 {
    struct Data {
        x: i32,
        y: i32,
    }
    let mut pos = vec![Data{
        x: 0,
        y: 0,
    },
    Data{
        x: 0,
        y: 0,
    }];
    let mut idx = 0;

    let mut seen : HashSet<(i32, i32)> = HashSet::new();
    seen.insert((0, 0));

    for c in input.chars() {
        match c {
            '>' => pos[idx].x += 1,
            '<' => pos[idx].x -= 1,
            '^' => pos[idx].y += 1,
            'v' => pos[idx].y -= 1,
            _ => panic!("Invalid input {}", c),
        }

        seen.insert((pos[idx].x, pos[idx].y));
        idx = (idx + 1) % 2;
    }

    seen.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(calc(">v"), 3);
    }

    #[test]
    fn ex2() {
        assert_eq!(calc("^>v<"), 3);
    }

    #[test]
    fn ex3() {
        assert_eq!(calc("^v^v^v^v^v"), 11);
    }
}
