use std::cmp::max;
use std::collections::HashSet;

type Point = (u32, u32);

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let mut points: HashSet<Point> = (&mut lines)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    for line in lines {
        let (axis, amount) = line
            .strip_prefix("fold along ")
            .unwrap()
            .split_once('=')
            .unwrap();
        let amount = amount.parse().unwrap();
        let deltas: Vec<(Point,Point)> = points
            .iter()
            .filter_map(|&p| foldp(p, axis, amount))
            .collect();
        for (insert, remove) in deltas.iter() {
            points.insert(*insert);
            points.remove(remove);
        }
    }
    display(points);
}

fn display(points: HashSet<Point>) {
    let (maxx, maxy) = points
        .iter()
        .fold((0, 0), |acc, p| (max(acc.0, p.0), max(acc.1, p.1)));

    for y in 0..=maxy {
        println!(
            "{}",
            (0..=maxx)
                .map(|x| if points.contains(&(x, y)) { '#' } else { ' ' })
                .collect::<String>()
        );
    }
}

fn foldp(c: Point, axis: &str, mid: u32) -> Option<(Point, Point)> {
    if axis == "x" {
        if c.0 < mid {
            None
        } else {
            Some(((2 * mid - c.0, c.1), c))
        }
    } else {
        if c.1 < mid {
            None
        } else {
            Some(((c.0, 2 * mid - c.1), c))
        }
    }
}
