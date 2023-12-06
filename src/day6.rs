use std::{process, fs};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/6
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path>", args[0], args[1]);

	if args.len() != 3 {
		eprintln!("{syntax}");
		process::exit(1);
	}	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);

	let content = content.split_once('\n').unwrap_or_exit("Invalid number of lines", 1);
	let content = [ content.0, content.1 ]
		.map(|l| l.split_once(':').unwrap_or_exit(&format!("Invalid line {l}"), 1))
		.map(|l|
			l.1.split_whitespace()
			.map(str::parse)
			.collect::<Result<Vec<u32>, _>>()
		)
		.map(|l| l.unwrap_or_exit("Could not read line values", 1));

	let times = &content[0];
	let records = &content[1];

	if times.len() != records.len() {
		eprintln!("Received inconsistent amount of times and records");
		process::exit(1);
	}

	let mut product = 1;
	for i in 0..times.len() {
		let time = times[i];
		let record = records[i];
		let ftime_halves = Into::<f64>::into(time) / 2.0;
		let frecord: f64 = record.into();

		let res = ftime_halves * ftime_halves - frecord;
		let res = if res > 0.0 { res.sqrt() } else { 0.0 };
		let res12 = [
			(ftime_halves - res).ceil() as u32,
			(ftime_halves + res).floor() as u32,
		];

		let res = res12[1] - res12[0] + 1 - res12.iter().filter(|r| **r * (time - **r) == record).count() as u32;

		product *= res;
		println!("Press for time {} and record {} must be between {} and {} miliseconds, resulting in {} possible winning millis", time, record, res12[0], res12[1], res);
	}

	println!("Product of possible millis: {product}");

}