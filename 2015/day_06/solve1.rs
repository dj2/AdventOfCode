#[derive(Debug, Copy, Clone, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}
#[derive(Debug, Copy, Clone, PartialEq)]
struct Block {
    start: Pos,
    end: Pos,
}
#[derive(Debug, PartialEq)]
enum Command {
    Toggle(Block),
    Off(Block),
    On(Block),
}
#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    points: Vec<bool>,
}
impl Grid {
    fn new(w: usize, h: usize) -> Self {
        Grid {
            width: w,
            height: h,
            points: vec![false; (w * h) as usize],
        }
    }

    fn process_cmd(&mut self, cmd: &Command) {
        let mut exec = |b: &Block, cb: &dyn Fn(bool) -> bool| {
            for y in b.start.y..(b.end.y + 1) {
                for x in b.start.x..(b.end.x + 1) {
                    let idx = (y * self.height) + x;
                    self.points[idx] = cb(self.points[idx]);
                }
            }
        };

        match cmd {
            Command::Toggle(b) => exec(b, &|val: bool| !val),
            Command::On(b) => exec(b, &|_| true),
            Command::Off(b) => exec(b, &|_| false),
        }
    }

    fn process(&mut self, cmds: Vec<Command>) {
        for c in cmds {
            self.process_cmd(&c)
        }
    }

    fn count(&self) -> usize {
        self.points.iter().filter(|&b| *b).count()
    }
}

fn parse(input: &str) -> Vec<Command> {
    let mut ret: Vec<Command> = Vec::new();

    for l in input.lines() {
        match parse_line(l) {
            Some(cmd) => ret.push(cmd),
            None => {}
        }
    }
    ret
}

fn make_pos(input: &str) -> Pos {
    let parts: Vec<&str> = input.split(",").collect();
    Pos {
        x: parts[0].parse::<usize>().unwrap(),
        y: parts[1].parse::<usize>().unwrap(),
    }
}

fn parse_line(input: &str) -> Option<Command> {
    if input.is_empty() {
        return None;
    }

    let parts: Vec<&str> = input.split_ascii_whitespace().collect();

    if input.starts_with("toggle ") {
        return Some(Command::Toggle(Block {
            start: make_pos(parts[1]),
            end: make_pos(parts[3]),
        }));
    }
    if input.starts_with("turn on ") {
        return Some(Command::On(Block {
            start: make_pos(parts[2]),
            end: make_pos(parts[4]),
        }));
    }
    if input.starts_with("turn off ") {
        return Some(Command::Off(Block {
            start: make_pos(parts[2]),
            end: make_pos(parts[4]),
        }));
    }

    panic!("Invalid input {}", input);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing input filename");
    }
    let filename = &args[1];

    let input = std::fs::read_to_string(filename).expect(&format!("Unable to read {}", filename));
    let data = parse(&input);

    let mut g = Grid::new(1000, 1000);
    g.process(data);
    println!("{:?}", g.count());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_input() {
        let input = r#"
toggle 753,664 through 970,926
turn off 150,300 through 213,740
turn on 141,242 through 932,871
"#;

        let data = parse(&input);
        assert_eq!(data.len(), 3);
        assert_eq!(
            data[0],
            Command::Toggle(Block {
                start: Pos { x: 753, y: 664 },
                end: Pos { x: 970, y: 926 },
            })
        );
        assert_eq!(
            data[1],
            Command::Off(Block {
                start: Pos { x: 150, y: 300 },
                end: Pos { x: 213, y: 740 },
            })
        );
        assert_eq!(
            data[2],
            Command::On(Block {
                start: Pos { x: 141, y: 242 },
                end: Pos { x: 932, y: 871 },
            })
        );
    }

    #[test]
    fn parse_toggle() {
        let data = parse_line("toggle 753,664 through 970,926");
        assert_eq!(
            data,
            Some(Command::Toggle(Block {
                start: Pos { x: 753, y: 664 },
                end: Pos { x: 970, y: 926 },
            }))
        );
    }

    #[test]
    fn parse_off() {
        let data = parse_line("turn off 150,300 through 213,740");
        assert_eq!(
            data,
            Some(Command::Off(Block {
                start: Pos { x: 150, y: 300 },
                end: Pos { x: 213, y: 740 },
            }))
        );
    }

    #[test]
    fn parse_on() {
        let data = parse_line("turn on 141,242 through 932,871");
        assert_eq!(
            data,
            Some(Command::On(Block {
                start: Pos { x: 141, y: 242 },
                end: Pos { x: 932, y: 871 },
            }))
        );
    }

    #[test]
    fn parse_blank() {
        let data = parse_line("");
        assert_eq!(data, None);
    }

    #[test]
    fn empty_grid_count() {
        let g = Grid::new(20, 20);
        assert_eq!(g.count(), 0);
    }

    #[test]
    fn full_grid_count() {
        let mut g = Grid::new(20, 20);
        for i in 0..(20 * 20) {
            g.points[i] = true
        }
        assert_eq!(g.count(), 20 * 20);
    }

    #[test]
    // ..........
    // ..........
    // ..|||||...
    // ..|xxx|...
    // ..|xxx|...
    // ..|||||...
    // ..|||||...
    // ..........
    // ..........
    // ..........
    //
    fn grid_process_toggle() {
        let mut g = Grid::new(10, 10);
        for i in 34..37 {
            g.points[i] = true;
        }
        for i in 44..47 {
            g.points[i] = true;
        }
        g.process_cmd(&Command::Toggle(Block {
            start: Pos { x: 2, y: 2 },
            end: Pos { x: 6, y: 6 },
        }));
        assert_eq!(g.count(), 19);
    }

    #[test]
    fn grid_process_on() {
        let mut g = Grid::new(10, 10);
        g.points[0] = true;
        for i in 34..37 {
            g.points[i] = true;
        }
        for i in 44..47 {
            g.points[i] = true;
        }
        g.process_cmd(&Command::On(Block {
            start: Pos { x: 2, y: 2 },
            end: Pos { x: 6, y: 6 },
        }));
        assert_eq!(g.count(), 26);
    }

    #[test]
    fn grid_process_off() {
        let mut g = Grid::new(10, 10);
        g.points[0] = true;
        for i in 34..37 {
            g.points[i] = true;
        }
        for i in 44..47 {
            g.points[i] = true;
        }
        g.process_cmd(&Command::Off(Block {
            start: Pos { x: 2, y: 2 },
            end: Pos { x: 6, y: 6 },
        }));
        assert_eq!(g.count(), 1);
    }
}
