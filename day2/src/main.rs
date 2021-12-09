use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let directions = include_str!("input.txt")
        .split_terminator('\n')
        .map(str::parse::<Movement>)
        .map(Result::unwrap);

    let mut p = Position {
        forward: 0,
        depth: 0,
        aim: 0,
    };
    for d in directions {
        p.travel(&d);
    }

    dbg!(&p);
    dbg!(p.forward * p.depth);
}

#[derive(Debug)]
struct Position {
    forward: i64,
    depth: i64,
    aim: i64,
}

impl Position {
    fn travel(&mut self, m: &Movement) {
        match m.dir {
            Direction::Down => self.aim += m.n,
            Direction::Up => self.aim -= m.n,
            Direction::Forward => {
                self.forward += m.n;
                self.depth += m.n * self.aim;
            }
        }
    }
}

#[derive(Debug)]
struct Movement {
    dir: Direction,
    n: i64,
}

impl FromStr for Movement {
    type Err = MyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        Ok(Movement {
            dir: parts
                .next()
                .ok_or(MyError::new("missing direction"))?
                .parse()?,
            n: parts
                .next()
                .ok_or(MyError::new("missing amount"))?
                .parse()?,
        })
    }
}

#[derive(Debug)]
enum Direction {
    Down,
    Up,
    Forward,
}

impl FromStr for Direction {
    type Err = MyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            "forward" => Ok(Direction::Forward),
            _ => Err(MyError::new(&format!("unrecognised direction:{}", s))),
        }
    }
}

#[derive(Debug)]
struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError {
            details: msg.to_string(),
        }
    }
}
impl std::error::Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        MyError::new(&err.to_string())
    }
}
