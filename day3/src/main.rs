type Words = Vec<Vec<char>>;

fn main() {
    let lines: Words = include_str!("input.txt")
        .split_terminator('\n')
        .map(|s| s.chars().collect())
        .collect();

    // dbg!(&lines);
    let (a, b) = partition(lines, 0);
    let (x, y) = if o2(a.len(), b.len()) {
        (isolate(a, 1, o2), isolate(b, 1, co2))
    } else {
        (isolate(b, 1, o2), isolate(a, 1, co2))
    };
    dbg!(x, y, x * y);
}

fn isolate(w: Words, p: usize, f: fn(usize, usize) -> bool) -> i32 {
    if w.len() == 1 {
        let s: String = w[0].iter().collect();
        dbg!(&s);
        i32::from_str_radix(&s, 2).unwrap()
    } else {
        let (a, b) = partition(w, p);
        isolate(if f(a.len(), b.len()) { a } else { b }, p + 1, f)
    }
}

fn o2(ones: usize, zeroes: usize) -> bool {
    return ones >= zeroes;
}

fn co2(ones: usize, zeroes: usize) -> bool {
    return ones < zeroes;
}

fn partition(v: Words, p: usize) -> (Words, Words) {
    let mut ones = Vec::new();
    let mut zeroes = Vec::new();
    for cs in v {
        (if cs[p] == '1' { &mut ones } else { &mut zeroes }).push(cs);
    }
    (ones, zeroes)
}
