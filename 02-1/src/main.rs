use std::fs::File;
use std::io::{self, BufRead};

fn main() {
	let file = File::open("input.txt").expect("Could not read input.txt");
	let mut horizontal = 0;
	let mut depth = 0;
	io::BufReader::new(file)
		.lines()
		.map(|line| {
			line.unwrap()
				.splitn(2, " ")
				.map(|s| String::from(s))
				.collect()
		})
		.for_each(|x: Vec<_>| {
			let amount: i32 = x[1].parse().unwrap();
			match x[0].as_str() {
				"forward" => horizontal += amount,
				"down" => depth += amount,
				"up" => depth -= amount,
				_ => {}
			}
		});

	println!("{}", horizontal * depth)
}
