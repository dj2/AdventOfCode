#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct Deer {
    speed: usize,
    move_time: usize,
    rest_time: usize,

    dist: usize,
    time: usize,
    score: usize,
    is_resting: bool,
}
impl Deer {
    fn new(speed: usize, move_time: usize, rest_time: usize) -> Self {
        Deer {
            speed,
            move_time,
            rest_time,

            time: 0,
            dist: 0,
            score: 0,
            is_resting: false,
        }
    }

    fn tick(&mut self) {
        self.time += 1;
        if self.is_resting {
            if self.time >= self.rest_time {
                self.is_resting = false;
                self.time = 0;
            }
        } else {
            self.dist += self.speed;

            if self.time >= self.move_time {
                self.is_resting = true;
                self.time = 0;
            }
        }
    }

    fn inc_score(&mut self) {
        self.score += 1;
    }
}

fn cap_to_val(cap: Option<regex::Match<'_>>) -> usize {
    cap.unwrap().as_str().parse().unwrap()
}

fn process_line(input: &str) -> Deer {
    let line_re = Regex::new(r"fly (\d+) km/s for (\d+) seconds.* (\d+) seconds.").unwrap();
    let caps = line_re.captures(input).unwrap();

    let dist = cap_to_val(caps.get(1));
    let time = cap_to_val(caps.get(2));
    let rest = cap_to_val(caps.get(3));

    Deer::new(dist, time, rest)
}

fn process(input: &str, seconds: usize) -> usize {
    let mut data: Vec<Deer> = input.lines().map(|l| process_line(l)).collect();

    let mut tick = 0;
    loop {
        tick += 1;
        for deer in data.iter_mut() {
            deer.tick();
        }

        data.sort_by(|a, b| b.dist.cmp(&a.dist));
        let dist = data[0].dist;

        data.iter_mut()
            .filter(|c| c.dist == dist)
            .for_each(|x| x.inc_score());
        if tick > seconds {
            break;
        }
    }
    data.iter().map(|x| x.score).max().unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing filename argument");
    }
    let filename = &args[1];

    let data =
        std::fs::read_to_string(filename).unwrap_or_else(|_| panic!("Unable to read {}", filename));
    println!("{}", process(&data, 2503));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let input = r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        assert_eq!(689, process(input, 1000));
    }
}
