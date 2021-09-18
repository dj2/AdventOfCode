#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

extern crate regex;

use regex::Regex;

fn cap_to_val(cap: Option<regex::Match<'_>>) -> i32 {
    cap.unwrap().as_str().parse().unwrap()
}

fn process(input: &str) -> i32 {
    sum(&strip(input))
}

fn strip(input: &str) -> String {
    let mut json: serde_json::Value = serde_json::from_str(input).expect("malformed JSON");
    if let Some(v) = strip_json(&mut json) {
        v.to_string()
    } else {
        "".to_string()
    }
}

fn strip_json(json: &mut serde_json::Value) -> Option<&serde_json::Value> {
    if let serde_json::Value::Array(data) = json {
        for mut item in data.iter_mut() {
            if None == strip_json(&mut item) {
                *item = serde_json::Value::Null;
            }
        }
    } else if let serde_json::Value::Object(data) = json {
        for mut value in data.values_mut() {
            if let serde_json::Value::String(content) = value {
                if content == "red" {
                    return None;
                }
            } else if None == strip_json(&mut value) {
                *value = serde_json::Value::Null;
            }
        }
    }
    Some(json)
}

fn sum(input: &str) -> i32 {
    let digit_re = Regex::new(r"(-?\d+)").unwrap();
    let mut sum = 0;
    for caps in digit_re.captures_iter(input) {
        sum += cap_to_val(caps.get(1));
    }
    sum
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
        assert_eq!(6, process("[1,2,3]"));
    }

    #[test]
    fn ex2() {
        assert_eq!(4, process(r#"[1,{"c":"red","b":2},3]"#));
    }

    #[test]
    fn ex3() {
        assert_eq!(0, process(r#"{"d":"red","e":[1,2,3,4],"f":5}"#));
    }

    #[test]
    fn ex4() {
        assert_eq!(6, process(r#"[1,"red",5]"#));
    }
}
