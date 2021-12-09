use std::collections::HashMap;

fn main() {
    let lines: Vec<Vector> = include_str!("input.txt")
        .split_terminator('\n')
        .map(Vector::from_str)
        .collect();
    let mut points: HashMap<Point, i32> = HashMap::new();
    for line in lines {
        for p in line {
            points.entry(p).and_modify(|e| *e += 1).or_insert(1);
        }
    }
   let count = points.values().filter(|v|**v>1).count() ;
    dbg!(count);
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_str(s: &str) -> Point {
        let mut parts = s.split(',');
        Point {
            x: parts.next().unwrap().trim().parse().unwrap(),
            y: parts.next().unwrap().trim().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    DownRight,
    UpRight,
}

#[derive(Debug)]
struct Vector {
    a: Point,
    b: Point,
    curr: Point,
    direction: Direction,
    finished: bool,
}

impl Vector {
    fn from_str(s: &str) -> Vector {
        let chunks: Vec<&str> = s.split("->").collect();
        let a = Point::from_str(chunks[0]);
        let b = Point::from_str(chunks[1]);
        let (a, b) = if a.x < b.x || a.x == b.x && a.y < b.y {
            (a, b)
        } else {
            (b, a)
        };

        let direction = if a.x == b.x {
            Direction::Down
        } else if a.y == b.y {
            Direction::Right
        } else if a.y < b.y {
            Direction::DownRight
        } else {
            Direction::UpRight
        };
        Vector {
            a,
            b,
            curr: a,
            direction,
            finished: false,
        }
    }
}

impl Iterator for Vector {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let reply = self.curr;
        if self.curr == self.b {
            self.finished = true
        }

        match self.direction {
            Direction::DownRight => {
                self.curr.x += 1;
                self.curr.y += 1;
            }
            Direction::UpRight => {
                self.curr.x += 1;
                self.curr.y -= 1;
            }
            Direction::Right => self.curr.x += 1,
            Direction::Down => self.curr.y += 1,
        }

        Some(reply)
    }
}
