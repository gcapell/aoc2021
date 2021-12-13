use std::collections::HashSet;

type Point = (u32, u32);

fn main() {
    let mut lines = include_str!("input.txt").split_terminator('\n');
    let mut points = HashSet::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let coords = line
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect::<Vec<u32>>();
        points.insert((coords[0], coords[1]));
    }
    for line in lines {
        let mut line = line.strip_prefix("fold along ").unwrap().split('=');
        let fold_fn = if line.next() == Some("x") {
            foldx
        } else {
            foldy
        };
        let mid = line.next().unwrap().parse::<u32>().unwrap();

        points = fold(&points, fold_fn, mid);
    }
		display(points);
}

fn display(points:HashSet<Point>){
	let maxx = points.iter().map(|p|p.0).max().unwrap();
	let maxy = points.iter().map(|p|p.1).max().unwrap();

	for y in 0..=maxy {
		let mut chars = Vec::new();
		for x in 0..=maxx {
			chars.push(if points.contains( &(x,y)) {'#'} else {' '});
		}
		println!("{}",chars.iter().collect::<String>());
	}
}

fn fold(ps: &HashSet<Point>, f: fn(Point, u32) -> Point, mid: u32) -> HashSet<Point> {
    let mut points = HashSet::new();
    for p in ps {
        points.insert(f(*p, mid));
    }
    points
}

fn foldy(c: Point, mid: u32) -> Point {
    if c.1 < mid {
        c
    } else {
        (c.0, 2 * mid - c.1)
    }
}

fn foldx(c: Point, mid: u32) -> Point {
    if c.0 < mid {
        c
    } else {
        (2 * mid - c.0, c.1)
    }
}
