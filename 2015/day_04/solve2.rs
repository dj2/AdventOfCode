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

        match res[0..6].parse::<i32>() {
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
