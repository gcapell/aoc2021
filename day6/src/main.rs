fn main() {
    let ages: Vec<usize> = include_str!("input.txt")
        .split(',')
        .map(str::trim)
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();
    dbg!(&ages);
    let mut counts: Vec<i64> = vec![0; 9];
    for age in ages {
        counts[age] += 1;
    }
    for j in 0..=256 {
        let sum = (&counts).iter().sum::<i64>();
        dbg!(j, sum);
        counts = gen(counts);
    }
}

fn gen(src: Vec<i64>) -> Vec<i64> {
    let mut dst = vec![0; 9];
    for j in 0..=8 {
        dst[j] = src[(j + 1) % 9];
    }
    dst[6] += src[0];
    dst
}
