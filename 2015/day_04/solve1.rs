extern crate md5;

fn main() {
    println!("{}", calc("iwrupvqb"));
}

fn calc(input: &str) -> i32 {
    let mut cur = 0;
    loop {
        let val = format!("{}{}", input, cur);
        let digest = md5::compute(val.as_bytes());
        let res = format!("{:x}", digest);

        match res[0..5].parse::<i32>() {
            Ok(v) => {
                if v == 0 {
                    break;
                }
            }
            _ => {}
        }
        cur += 1;
    }

    cur
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(calc("abcdef"), 609043);
    }

    #[test]
    fn ex2() {
        assert_eq!(calc("pqrstuv"), 1048970);
    }
}
