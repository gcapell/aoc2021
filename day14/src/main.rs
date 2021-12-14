use std::collections::HashMap;
use std::time::Instant;

type Count = u64;

fn main() {
    let now = Instant::now();
    let (polymer, pairs) = include_str!("input.txt").split_once("\n\n").unwrap();

    let pairs: HashMap<&str, char> = pairs
        .split_terminator('\n')
        .map(|s| {
            let (p, c) = s.split_once(" -> ").unwrap();
            (p, c.chars().next().unwrap())
        })
        .collect();

    let mut interned_pairs: HashMap<&str, usize> = HashMap::new();
    let mut pair_names: Vec<&str> = Vec::new();

    for (i, p) in pairs.keys().enumerate() {
        interned_pairs.insert(*p, i);
        pair_names.push(*p);
    }

    let mut insertions: Vec<(usize, usize)> = Vec::new();
    for pair in &pair_names {
        let insertion = pairs.get(pair).unwrap();
        let chars: Vec<char> = pair.chars().collect();
        let a = chars[0];
        let b = chars[1];
        let a_string: String = vec![a, *insertion].into_iter().collect();
        let a_i = interned_pairs.get(&a_string as &str).unwrap();
        let b_string: String = vec![*insertion, b].into_iter().collect();
        let b_i = interned_pairs.get(&b_string as &str).unwrap();
        insertions.push((*a_i, *b_i));
    }

    let mut counts: Vec<Count> = vec![0; pairs.len()];

    for x in polymer.as_bytes().windows(2) {
        let p: String = vec![x[0] as char, x[1] as char].into_iter().collect();
        let pos: usize = *interned_pairs.get(&p as &str).unwrap();
        counts[pos] += 1;
    }

    let mut counts2: Vec<Count> = vec![0; pairs.len()];
    let setup_t = now.elapsed();
    for _ in 0..20 {
        polymerise(&counts, &mut counts2, &insertions);
        polymerise(&counts2, &mut counts, &insertions);
    }
    show_counts(&counts, &pair_names);
    println!("setup: {:?}, calculate: {:?}", setup_t, now.elapsed());
}

fn polymerise(src: &[Count], dst: &mut [Count], insertions: &[(usize, usize)]) {
    dst.fill(0);
    for (i, &n) in src.iter().enumerate() {
        let (a, b) = insertions[i];
        dst[a] += n;
        dst[b] += n;
    }
}

fn show_counts(counts: &[Count], names: &[&str]) {
    let mut char_counts: HashMap<char, Count> = HashMap::new();
    for (i, &n) in counts.iter().enumerate() {
        if n == 0 {
            continue;
        }
        let name = names[i];
        for c in name.chars() {
            *char_counts.entry(c).or_insert(0) += n;
        }
    }
    for n in char_counts.values_mut() {
        *n = (*n + 1) / 2;
    }
    println!(
        "{}",
        char_counts.values().max().unwrap() - char_counts.values().min().unwrap()
    );
}
