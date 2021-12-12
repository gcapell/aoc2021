use std::collections::HashSet;

fn main() {
    let mut board: Vec<Vec<u8>> = include_str!("input.txt")
        .split_terminator('\n')
        .map(|s| s.bytes().map(|c| c - b'0').collect::<Vec<u8>>())
        .collect();

    let mut s = 0;
    loop {
        let n = step(&mut board);
        s += 1;
        if n == 100 {
            break;
        }
    }
    println!("all flash after step {}", s);
}

fn step(b: &mut Vec<Vec<u8>>) -> usize {
    let mut flashes = HashSet::new();
    let mut flashed = HashSet::new();

    for r in 0..b.len() {
        for c in 0..b[0].len() {
            b[r][c] += 1;
            if b[r][c] > 9 {
                flashes.insert((r, c));
                flashed.insert((r, c));
            }
        }
    }

    while !flashes.is_empty() {
        let mut new_flashes = HashSet::new();
        for &(r, c) in &flashes {
            b[r][c] = 0;
            for (r1, c1) in neighbours((r, c), b.len(), b[0].len()) {
                if flashed.contains(&(r1, c1)) {
                    continue;
                }
                b[r1][c1] += 1;
                if b[r1][c1] > 9 {
                    new_flashes.insert((r1, c1));
                    flashed.insert((r1, c1));
                }
            }
        }
        flashes = new_flashes;
    }
    flashed.len()
}

#[derive(Debug)]
struct Neighbours {
    p: (usize, usize),
    n: i32,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

fn neighbours(p: (usize, usize), rows: usize, cols: usize) -> Neighbours {
    Neighbours {
        p,
        n: 0,
        up: p.0 > 0,
        down: p.0 + 1 < rows,
        left: p.1 > 0,
        right: p.1 + 1 < cols,
    }
}

impl Iterator for Neighbours {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (r, c) = self.p;
        loop {
            let m = match (self.n, self.up, self.down, self.left, self.right) {
                (0, true, _, true, _) => Some((r - 1, c - 1)),
                (1, true, _, _, _) => Some((r - 1, c)),
                (2, true, _, _, true) => Some((r - 1, c + 1)),
                (3, _, _, true, _) => Some((r, c - 1)),
                (4, _, _, _, true) => Some((r, c + 1)),
                (5, _, true, true, _) => Some((r + 1, c - 1)),
                (6, _, true, _, _) => Some((r + 1, c)),
                (7, _, true, _, true) => Some((r + 1, c + 1)),
                (8, _, _, _, _) => return None,
                _ => None,
            };
            self.n += 1;
            if m.is_some() {
                return m;
            }
        }
    }
}
