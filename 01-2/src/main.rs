use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
	let file = File::open("../01-1/input.txt").expect("Could not read input.txt");
	let depths = io::BufReader::new(file)
		.lines()
		.map(|line| line.unwrap().parse::<i32>().unwrap())
		// This is the only change from previous - now we are grouping by 3 and summing before comparing
		.tuple_windows()
		// Unfortunately, there doesn't seem to be a great syntax for summing over Tuples,
		// which makes sense (since tuples are for heterogeneous data) and itertools is only using tuples for convenience.
		.map(|(a, b, c)| a + b + c)
		.tuple_windows()
		.filter(|(prev, next)| next > prev)
		.count();

	println!("{}", depths)
}
