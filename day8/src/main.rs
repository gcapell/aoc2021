use std::collections::HashMap;

fn main() {
    let answer: u32 = include_str!("input.txt")
        .split_terminator('\n')
        .map(Problem::from_str)
        .map(|x| x.solve())
        .sum();

    dbg!(answer);
}

type Signals = u32;

#[derive(Debug)]
struct Problem {
    patterns: [Signals; 10],
    out: [Signals; 4],
}

fn signals_from_str(s: &str, o: &mut [Signals]) {
    for (i, w) in s.split_whitespace().enumerate() {
        o[i] = signal_from_str(w);
    }
}

fn signal_from_str(w: &str) -> Signals {
    let mut reply = 0;
    for c in w.chars() {
        reply |= 1 << (c as i32 - 'a' as i32)
    }
    reply
}

impl Problem {
    fn from_str(s: &str) -> Problem {
        let p = s.find('|').unwrap() as usize;
        let mut prob = Problem {
            patterns: [0; 10],
            out: [0; 4],
        };
        signals_from_str(&s[..p], &mut prob.patterns[..]);
        signals_from_str(&s[(p + 1)..], &mut prob.out[..]);
        prob
    }
    fn solve(&self) -> u32 {
        let mut bitcounts: HashMap<u32, Vec<Signals>> = HashMap::new();
        for p in self.patterns {
            let c = p.count_ones();
            bitcounts.entry(c).or_insert(Vec::new()).push(p);
        }
        let cf = bitcounts.get(&2).unwrap()[0];
        let acf = bitcounts.get(&3).unwrap()[0];
        let a = acf & !cf;
        let bcdf = bitcounts.get(&4).unwrap()[0];
        let bd = bcdf & !cf;

        let adg = bitcounts
            .get(&5)
            .unwrap()
            .iter()
            .fold(0xff, |acc, x| acc & x);
        let dg = adg & !a;
        let d = dg & bd;
        let g = dg & !d;
        let b = bd & !d;

        let mut f = 0;
        for digit in bitcounts.get(&5).unwrap() {
            if digit & b != 0 {
                // digit 5
                f = digit & !a & !b & !d & !g;
                break;
            }
        }
        let c = cf & !f;
        let mut e = 0;
        for digit in bitcounts.get(&5).unwrap() {
            if digit & f == 0 {
                // digit 2
                e = digit & !a & !c & !d & !g;
                break;
            }
        }

        let digits = vec![
            a | b | c | e | f | g,
            c | f,
            a | c | d | e | g,
            a | c | d | f | g,
            b | c | d | f,
            a | b | d | f | g,
            a | b | d | e | f | g,
            a | c | f,
            127,
            a | b | c | d | f | g,
        ];
        let mut outputs: HashMap<u32, u32> = HashMap::new();
        for (i, v) in digits.iter().enumerate() {
            outputs.insert(*v, i as u32);
        }

        let mut reply = 0;
        for v in self.out {
            let digit = outputs.get(&v).unwrap();
            reply = reply * 10 + digit;
        }
        reply
    }
}
