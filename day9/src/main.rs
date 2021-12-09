fn main() {
    let mut heights: Vec<Vec<bool>> = include_str!("input.txt")
        .split_terminator('\n')
        .map(|s| s.chars().map(|x| x != '9').collect())
        .collect();

    let mut sizes = Vec::new();

    for r in 0..heights.len() {
        for c in 0..heights[0].len() {
            if heights[r][c] {
                sizes.push(fill(&mut heights, r, c))
            }
        }
    }
    sizes.sort_unstable();
    sizes.reverse();
    dbg!(&sizes[..3], sizes[0] * sizes[1] * sizes[2]);
}

fn fill(h: &mut Vec<Vec<bool>>, r: usize, c: usize) -> i32 {
    let mut border = vec![(r, c)];
    let mut size = 0;
    while let Some((r, c)) = border.pop() {
        if !h[r][c] {
            continue;
        }
        h[r][c] = false;
        size += 1;
        for p in neighbors((r, c), h.len(), h[0].len()) {
            if h[p.0][p.1] {
                border.push(p)
            }
        }
    }
    size
}

fn neighbors(p: (usize, usize), rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut reply = Vec::new();
    if p.0 > 0 {
        reply.push((p.0 - 1, p.1));
    }
    if p.0 + 1 < rows {
        reply.push((p.0 + 1, p.1));
    }
    if p.1 > 0 {
        reply.push((p.0, p.1 - 1));
    }
    if p.1 + 1 < cols {
        reply.push((p.0, p.1 + 1));
    }
    reply
}
