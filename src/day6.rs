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
		.map(|l| (
			l.1.split_whitespace()
				.map(str::parse)
				.collect::<Result<Vec<u32>, _>>()
				.unwrap_or_exit(&format!("Could not parse {} as number list", l.1), 1),
		l.1.trim().replace(' ', "").parse::<u64>().unwrap_or_exit(&format!("Could not parse {} as number", l.1.replace(' ', "")), 1))
		)
		.map(|l| l);

	let times = &content[0];
	let records = &content[1];

	if times.0.len() != records.0.len() {
		eprintln!("Received inconsistent amount of times and records");
		process::exit(1);
	}

	let mut product = 1;
	for i in 0..times.0.len() {
		let time = times.0[i];
		let record = records.0[i];

		let res = get_res(time.into(), record.into());
		product *= res.range;
		println!("Press for time {} and record {} must be between {} and {} miliseconds, resulting in {} possible winning millis", time, record, res.lower_bound, res.higher_bound, res.range);
	}

	println!("Product of possible millis: {product}");

	let res = get_res(times.1, records.1);
	println!("(Part 2): Press for time {} and record {} must be between {} and {} miliseconds, resulting in {} possible winning millis", times.1, records.1, res.lower_bound, res.higher_bound, res.range);
}

/// This is basically just a problem to find solutions for 0 = -x² + tx - d
/// where t is time and d is the distance record
fn get_res(time: u64, record: u64) -> Res {

	let ftime_halves = time as f64 / 2.0;
	let frecord: f64 = record as f64;

	let res = ftime_halves * ftime_halves - frecord;
	let res = if res > 0.0 { res.sqrt() } else { 0.0 };
	let res12 = [
		(ftime_halves - res).ceil() as u64,
		(ftime_halves + res).floor() as u64,
	];

	let res = res12[1] - res12[0] + 1 - res12.iter().filter(|r| **r * (time - **r) == record).count() as u64;

	Res {
		lower_bound: res12[0],
		higher_bound: res12[1],
		range: res,
	}
}

struct Res {
	lower_bound: u64,
	higher_bound: u64,
	range: u64,
}