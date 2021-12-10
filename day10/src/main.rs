fn main() {
    let mut scores: Vec<u64> = include_str!("input.txt")
        .split_terminator('\n')
        .filter_map(score)
        .collect();
    scores.sort();
    println!("scores {:?}", scores);
    println!("answer: {}", scores[scores.len() / 2]);
}

enum Bracket {
    Open(u64),
    Close(char),
}

fn bracket(c: char) -> Bracket {
    match c {
        ')' => Bracket::Close('('),
        ']' => Bracket::Close('['),
        '}' => Bracket::Close('{'),
        '>' => Bracket::Close('<'),

        '(' => Bracket::Open(1),
        '[' => Bracket::Open(2),
        '{' => Bracket::Open(3),
        '<' => Bracket::Open(4),
        _ => panic!("unrecognised bracket"),
    }
}

fn score(s: &str) -> Option<u64> {
    let mut stack = Vec::new();
    for c in s.chars() {
        match bracket(c) {
            Bracket::Open(_) => stack.push(c),
            Bracket::Close(opposite) => {
                if stack.pop() != Some(opposite) {
                    return None;
                }
            }
        }
    }
    let mut scores: Vec<u64> = stack
        .iter()
        .map(|&c| match bracket(c) {
            Bracket::Open(n) => n,
            _ => panic!("not open"),
        })
        .collect();
    scores.reverse();
    Some(scores.iter().fold(0, |total, n| total * 5 + n))
}
