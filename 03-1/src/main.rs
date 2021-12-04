use std::fs::File;
use std::io::{self, BufRead};

fn main() {
	let file = File::open("input.txt").expect("Could not read input.txt");
	let counts: Vec<_> = io::BufReader::new(file)
		.lines()
		.map(|line| {
			line.unwrap()
				.chars()
				.map(|s| s.to_digit(2).unwrap())
				.collect::<Vec<_>>()
		})
		.collect();

	let threshold = (counts.len() / 2) as u32;
	let (c1, c2) = counts
		.into_iter()
		.reduce(|acc, x| acc.into_iter().zip(x).map(|(a, b)| a + b).collect())
		.unwrap()
		.into_iter()
		.map(|x| if x > threshold { 1 } else { 0 })
		.fold((0, 0), |(acc1, acc2), x| (2 * acc1 + x, 2 * acc2 + (1 - x)));

	println!("{}", c1 * c2)
}
