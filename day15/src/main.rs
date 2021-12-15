use priority_queue::PriorityQueue;
use std::cmp::{min, Reverse};
use std::collections::HashSet;

type Point = (usize, usize);

fn main() {
    let lines: Vec<Vec<u32>> = include_str!("input.txt").lines().map(risks).collect();
    let mut dp: Vec<Vec<Option<u32>>> = Vec::new();
    let h = lines.len();
    let w = lines[0].len();
    for _ in 0..h * 5 {
        dp.push(vec![None; w * 5]);
    }
    let mut pq = PriorityQueue::new();
    pq.push((0, 0), Reverse(0));
    let mut done: HashSet<Point> = HashSet::new();
    loop {
        let (n, Reverse(risk)) = pq.pop().unwrap();
        if n == (h * 5 - 1, w * 5 - 1) {
            println!("risk @ {:?} is {}", n, risk);
            break;
        }
        for ne in neighbours(n, h * 5, w * 5) {
            if done.contains(&ne) {
                continue;
            }
            let new_cost = risk + cost(&lines, ne, h, w);
            let updated_cost = match dp[ne.0][ne.1] {
                Some(r) => min(r, new_cost),
                None => new_cost,
            };
            dp[ne.0][ne.1] = Some(updated_cost);
            pq.push(ne, Reverse(updated_cost));
        }
        done.insert(n);
    }
}

fn neighbours(p: Point, h: usize, w: usize) -> Vec<Point> {
    let mut v = Vec::new();
    if p.0 > 0 {
        v.push((p.0 - 1, p.1));
    }
    if p.0 + 1 < h {
        v.push((p.0 + 1, p.1));
    }
    if p.1 > 0 {
        v.push((p.0, p.1 - 1));
    }
    if p.1 + 1 < w {
        v.push((p.0, p.1 + 1));
    }
    v
}

fn cost(lines: &[Vec<u32>], p: Point, h: usize, w: usize) -> u32 {
    let orig = lines[p.0 % h][p.1 % w];
    let delta = p.0 / h + p.1 / w;
    ((orig - 1 + delta as u32) % 9) + 1
}

fn risks(s: &str) -> Vec<u32> {
    s.bytes().map(|n| (n - b'0') as u32).collect()
}
