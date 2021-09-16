fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing input filename");
    }

    let filename = &args[1];
    let data = std::fs::read_to_string(filename).expect(&format!("Unable to read {}", filename));

    let nice: Vec<&str> = data.lines().filter(|l| check(l)).collect();
    println!("{}", nice.len());
}

fn check(input: &str) -> bool {
    let vowel_count = input
        .chars()
        .filter(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        })
        .collect::<Vec<char>>()
        .len();

    let bad_combo = input.contains("ab")
        || input.contains("cd")
        || input.contains("pq")
        || input.contains("xy");

    let mut has_double_letter = false;
    for (idx, c) in input.chars().enumerate() {
        if idx == 0 {
            continue;
        }

        if c == input.chars().nth(idx - 1).unwrap() {
            has_double_letter = true;
            break;
        }
    }

    vowel_count >= 3 && !bad_combo && has_double_letter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(check("ugknbfddgicrmopn"));
    }

    #[test]
    fn ex2() {
        assert!(check("aaa"));
    }

    #[test]
    fn ex3() {
        assert!(!check("jchzalrnumimnmhp"));
    }

    #[test]
    fn ex4() {
        assert!(!check("haegwjzuvuyypxyu"));
    }

    #[test]
    fn ex5() {
        assert!(!check("dvszwmarrgswjxmb"));
    }
}
