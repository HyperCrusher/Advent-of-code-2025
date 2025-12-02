use std::fs;
struct Dial {
    value: i32,
    password: u32,
}

enum Direction {
    Left,
    Right,
}

enum Method {
    One,
    Two,
}

impl Dial {
    fn new(value: i32) -> Self {
        Self { value, password: 0 }
    }

    fn turn(&mut self, data: &str, method: Method) {
        let mut data = data.chars();
        let dir = match data.next() {
            None => return,
            Some(t) => match t {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => return,
            },
        };
        let data: String = data.collect();
        let turns: u32 = match data.parse() {
            Ok(num) => num,
            Err(_) => return,
        };
        self.password += match method {
            Method::One => Dial::method_one(self, turns, dir),
            Method::Two => Dial::method_two(self, turns, dir),
        }
    }

    fn method_one(&mut self, mut turns: u32, dir: Direction) -> u32 {
        while turns > 0 {
            match dir {
                Direction::Left => {
                    self.value -= 1;
                    if self.value < 0 {
                        self.value = 99
                    }
                }
                Direction::Right => {
                    self.value += 1;
                    if self.value > 99 {
                        self.value = 0
                    }
                }
            }
            turns -= 1
        }
        if self.value != 0 {
            return 0;
        }
        1
    }
    fn method_two(&mut self, mut turns: u32, dir: Direction) -> u32 {
        let mut passed_go = 0;
        while turns > 0 {
            match dir {
                Direction::Left => {
                    self.value -= 1;
                    if self.value < 0 {
                        self.value = 99
                    }
                }
                Direction::Right => {
                    self.value += 1;
                    if self.value > 99 {
                        self.value = 0
                    }
                }
            }
            if self.value == 0 {
                passed_go += 1
            }
            turns -= 1
        }
        passed_go
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut safe = Dial::new(50);
    for line in input.lines() {
        safe.turn(line.trim(), Method::Two);
    }

    println!("Password is: {}", safe.password)
}
