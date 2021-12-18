use std::ops::RangeInclusive;
type Speed=i32;

fn main() {
	let x_range =  185..=221; //20..=30;
	let y_range =  -122..=-74;// -10..=-5; 
	
	let mut count = 0;
	let y_start = *y_range.start();
	for dx in 1..=*x_range.end(){
		for dy in y_start..=-y_start {
			if hit(dx,dy,&x_range, &y_range){
				count += 1;
			}
		}
	}
	println!("{} combos", count);
}

fn hit(mut dx:Speed, mut dy:Speed, x_range: &std::ops::RangeInclusive<i32>, y_range:&RangeInclusive<i32>) -> bool {
	let (mut x, mut y) = (0,0);
	while y >= *y_range.start() && x <= *x_range.end() {
		x += dx;
		y += dy;
		dy -= 1;
		dx = if dx>0 {dx-1} else {dx};
		if x_range.contains(&x) && y_range.contains(&y) {
			return true;
		}
	}
	false
}
