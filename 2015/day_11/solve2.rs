#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

fn main() {
    println!("{}", process("cqjxxyzz"));
}

fn has_straight(input: &str) -> bool {
    let data: Vec<u32> = input.chars().map(|c| c as u32).collect();
    for i in 2..data.len() {
        if data[i - 2] + 1 == data[i - 1] && data[i - 1] + 1 == data[i] {
            return true;
        }
    }
    false
}

fn valid_characters(input: &str) -> bool {
    None == input.chars().find(|&c| c == 'i' || c == 'o' || c == 'l')
}

fn has_overlapping(input: &str) -> bool {
    let data: Vec<u32> = input.chars().map(|c| c as u32).collect();
    let mut overlap_val: Option<u32> = None;
    for i in 1..data.len() {
        if data[i - 1] == data[i] && Some(data[i]) != overlap_val {
            if overlap_val != None {
                return true;
            }
            overlap_val = Some(data[i]);
        }
    }
    false
}

fn is_valid(input: &str) -> bool {
    has_straight(input) && valid_characters(input) && has_overlapping(input)
}

fn next_password(input: &str) -> String {
    let mut data: Vec<char> = input.chars().collect();

    // Short circuit as soon as we find a bad char, replace it and 'a' out the remainder
    if let Some(idx) = data.iter().position(|c| *c == 'i' || *c == 'o' || *c == 'l') {
        data[idx] = char::from_u32((data[idx] as u32) + 1).unwrap();
        for item in data.iter_mut().skip(idx + 1) {
            *item = 'a';
        }
    }

    let mut idx = data.len() - 1;
    loop {
        let mut cur = data[idx] as u32;
        cur += 1;

        if cur >= 123 {
            data[idx] = 'a';
        } else {
            data[idx] = char::from_u32(cur).unwrap();
            break;
        }
        if idx == 0 {
            break;
        }
        idx -= 1;
    }
    data.iter().collect()
}

fn process(input: &str) -> String {
    let mut res = next_password(input);
    while !is_valid(&res) {
        res = next_password(&res);
        if res.chars().all(|c| c == 'a') {
            panic!("All a's");
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn has_straight_with_straight() {
        assert!(has_straight("abc"));
    }

    #[test]
    fn has_straight_without_straight() {
        assert!(!has_straight("abd"));
    }

    #[test]
    fn valid_characters_with_i() {
        assert!(!valid_characters("i"));
    }

    #[test]
    fn valid_characters_with_o() {
        assert!(!valid_characters("o"));
    }

    #[test]
    fn valid_characters_with_l() {
        assert!(!valid_characters("l"));
    }

    #[test]
    fn valid_characters_all_valid() {
        assert!(valid_characters("abc"));
    }

    #[test]
    fn has_overlapping_with_2_overlap() {
        assert!(has_overlapping("aabb"));
    }

    #[test]
    fn has_overlapping_only_one() {
        assert!(!has_overlapping("aabc"));
    }

    #[test]
    fn has_overlapping_none() {
        assert!(!has_overlapping("abcd"));
    }

    #[test]
    fn ex1() {
        assert!(!is_valid("hijklmmn"));
    }

    #[test]
    fn ex2() {
        assert!(!is_valid("abbceffg"));
    }

    #[test]
    fn ex3() {
        assert!(is_valid("abcdffaa"));
    }

    #[test]
    fn ex4() {
        assert!(is_valid("ghjaabcc"));
    }

    #[test]
    fn next_ex1() {
        assert_eq!("xy", next_password("xx"));
    }

    #[test]
    fn next_ex2() {
        assert_eq!("ya", next_password("xz"));
    }

    #[test]
    fn next_ex3() {
        assert_eq!("abcdffaa", process("abcdefgh"));
    }

    #[test]
    fn next_ex4() {
        assert_eq!("ghjaabcc", process("ghijklmn"));
    }
}

