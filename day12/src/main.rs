use std::collections::{HashMap, HashSet};

fn main() {
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();

    for (a, b) in include_str!("input.txt").split_terminator('\n').map(|s| {
        let p = s.find('-').unwrap();
        (&s[..p], &s[p + 1..])
    }) {
        add(&mut edges, a, b);
        add(&mut edges, b, a);
    }
    let paths = dfs(&edges, "start", &mut Tracker::new());
    dbg!(paths);
}

fn add<'a>(edges: &mut HashMap<&'a str, Vec<&'a str>>, a: &'a str, b: &'a str) {
    edges.entry(a).or_insert_with(Vec::new).push(b);
}

struct Tracker<'a> {
    seen: HashSet<&'a str>,
    doubled: Option<&'a str>,
    path: Vec<&'a str>,
}

impl<'a> Tracker<'a> {
    fn new() -> Self {
        Self {
            seen: HashSet::new(),
            doubled: None,
            path: Vec::new(),
        }
    }

    fn push(&mut self, s: &'a str) {
        if !big(s) {
            if self.seen.contains(s) {
                self.doubled = Some(s);
            } else {
                self.seen.insert(s);
            }
        }
        self.path.push(s);
    }

    fn pop(&mut self, s: &str) {
        if !big(s) {
            if self.doubled == Some(s) {
                self.doubled = None;
            } else {
                self.seen.remove(s);
            }
        }
        self.path.pop();
    }

    fn can_visit(&self, s: &str) -> bool {
        s != "start" && (big(s) || !self.seen.contains(s) || self.doubled == None)
    }
}

fn big(s: &str) -> bool {
    s.chars().next().unwrap().is_uppercase()
}

fn dfs<'a>(edges: &HashMap<&'a str, Vec<&'a str>>, src: &str, tracker: &mut Tracker<'a>) -> i32 {
    let mut paths = 0;
    for e in edges.get(src).unwrap() {
        if *e == "end" {
            //println!("start,{},end",tracker.path.join(","));
            paths += 1;
            continue;
        }
        if !tracker.can_visit(e) {
            continue;
        }
        tracker.push(e);
        paths += dfs(edges, e, tracker);
        tracker.pop(e);
    }
    paths
}
