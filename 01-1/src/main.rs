use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
	let file = File::open("input.txt").expect("Could not read input.txt");
	let depths = io::BufReader::new(file)
		.lines()
		// Map each line (string) to an int
		.map(|line| line.unwrap().parse::<i32>().unwrap())
		// tuple_windows() slides a window of two elements over the iterator of ints, grouping them into pairs.
		.tuple_windows()
		// Check for which pairs next (the element later on in the list) is greater
		.filter(|(prev, next)| next > prev)
		// The elements we have left is the number of times the list increases
		.count();

	println!("{}", depths)
}
