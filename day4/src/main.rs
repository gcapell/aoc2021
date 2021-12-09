use std::collections::HashMap;

fn main() {
    let mut paras = include_str!("input.txt").split("\n\n");

    let draw: Vec<i32> = paras
        .next()
        .unwrap()
        .split(',')
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect();

    let mut boards: Vec<Board> = paras.map(Board::from_str).collect();

    for n in draw {
        let last = boards.len() == 1;
        for b in &mut boards {
            if b.call(n) {
                b.won = true;
                if last {
                    dbg!(b.sum_unmarked() * n);
                    return;
                }
            }
        }
        boards.retain(|b| !b.won);
    }
}

#[derive(Debug)]
struct Square {
    row: usize,
    col: usize,
    called: bool,
}

#[derive(Debug)]
struct Board {
    size: usize,
    row: Vec<usize>,
    col: Vec<usize>,
    squares: HashMap<i32, Square>,
    won: bool,
}

impl Board {
    fn from_str(s: &str) -> Board {
        let nums: Vec<i32> = s
            .split_whitespace()
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect();
        let size = (nums.len() as f64).sqrt() as usize;
        let row = vec![0; size];
        let col = vec![0; size];
        let mut squares = HashMap::new();
        let mut r = 0;
        let mut c = 0;
        for n in nums {
            squares.insert( n, Square { row: r, col: c,  called: false});
            c += 1;
            if c == size {
                r += 1;
                c = 0;
            }
        }
        Board {
            size,
            row,
            col,
            squares,
            won: false,
        }
    }
    fn call(&mut self, n: i32) -> bool {
        if let Some(b) = self.squares.get_mut(&n) {
            self.row[b.row] += 1;
            self.col[b.col] += 1;
            b.called = true;
            self.row[b.row] == self.size || self.col[b.col] == self.size
        } else {
            false
        }
    }
    fn sum_unmarked(&self) -> i32 {
        let mut sum = 0;
        for (n, b) in &self.squares {
            if !b.called {
                sum += n;
            }
        }
        sum
    }
}
