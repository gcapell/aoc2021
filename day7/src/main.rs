fn main() {
  let mut pos: Vec<i32> = include_str!("input.txt")
      .split(',')
      .map(str::trim)
      .map(str::parse::<i32>)
      .map(Result::unwrap)
      .collect();
			pos.sort();
			dbg!(&pos);
	
			let sum = pos.iter().sum::<i32>();
			let avg = (sum as f64 / pos.len() as f64).round() as i32;
			dbg!(sum,pos.len(), avg);
	
	
			dbg!(avg, fuel(&pos, avg));
			dbg!(avg+1, fuel(&pos, avg+1));
			dbg!(avg-1, fuel(&pos, avg-1));
			dbg!(avg-2, fuel(&pos, avg-2));
}

fn fuel(ps: &Vec<i32>, avg:i32) -> i32 {
	let mut fuel = 0;
	for p in ps {
		let d = (p-avg).abs();
		let df = d * (d+1)/ 2;
		fuel += df;
	}
	fuel
}