fn main() -> anyhow::Result<()> {
    let n = include_str!("input.txt")
        .split_terminator('\n')
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect::<Vec<i64>>().windows(3)
        .map(|s| s.iter().sum())
        .collect::<Vec<i64>>().windows(2)
        .filter(|t| t[1] > t[0])
        .count();

    dbg!(n);

    Ok(())
}
