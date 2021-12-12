use std::collections::{HashMap,HashSet};

fn main() {
	let mut edges: HashMap<String,Vec<String>> = HashMap::new();
	
    for (a, b) in include_str!("input.txt").split_terminator('\n').map(|s| {
        let p = s.find('-').unwrap();
        (&s[..p], &s[p + 1..])
    }) {
			add(&mut edges, a,b);
			add(&mut edges, b, a);
    }
		let paths = dfs(&edges, "start", &mut Tracker::new());
		dbg!(paths);
}

fn add(edges: &mut HashMap<String, Vec<String>>, a:&str, b:&str){
	edges.entry(a.to_owned()).or_insert(Vec::new()).push(b.to_owned());
}

struct Tracker {
	seen : HashSet<String>,
	doubled: Option<String>,
	path: Vec<String>,
}

impl Tracker {
	fn new()->Self {
		Self{seen:HashSet::new(), doubled:None, path:Vec::new()}
	}
	
	fn push(&mut self, s:&str){
		if !big(s){
		if self.seen.contains(s){
			self.doubled = Some(s.to_string());
		}
		self.seen.insert(s.to_string());
	}
		self.path.push(s.to_string());
	}
	
	fn pop(&mut self, s:&str){
		if !big(s){
			if self.doubled == Some(s.to_string()){
				self.doubled = None;
				} else {
					self.seen.remove(s);
				}
			}
			self.path.pop();
	}
	
	fn can_visit(&self, s:&str) -> bool {
		s != "start" && ( big(s) || !self.seen.contains(s) || self.doubled == None)
	}
}

fn big(s:&str)->bool {
	s.chars().next().unwrap().is_uppercase()
}

fn dfs(edges: &HashMap<String, Vec<String>>, src:&str, tracker: &mut Tracker) -> i32 {
	let mut paths = 0;
	for e in edges.get(src).unwrap() {
		if e == "end" {
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
	return paths;
}