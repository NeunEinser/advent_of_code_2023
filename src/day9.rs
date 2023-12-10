use std::{process, fs};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/9
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path>", args[0], args[1]);

	if args.len() != 3 {
		eprintln!("{syntax}");
		process::exit(1);
	}
	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);
 	
	let mut future = 0;
	let mut past = 0;
	for line in content.lines() {
		let mut predictions = vec![line
			.split_whitespace()
			.map(str::parse)
			.collect::<Result<Vec<i32>, _>>()
			.unwrap_or_exit(&format!("Could not read line {line}"), 1)];

		// ideally we would just save current prediction here, but borrow checker doesn't let us push in the loop then.
		let mut prediction_ind = 0; 
		while predictions[prediction_ind].iter().any(|r| *r != 0) {
			let mut v = Vec::with_capacity(predictions[prediction_ind].len() - 1);
			for i in 1..predictions[prediction_ind].len() {
				v.push(predictions[prediction_ind][i] - predictions[prediction_ind][i-1]);
			}
			predictions.push(v);
			prediction_ind += 1;
		}

		let mut i = predictions.len() - 2;
		let mut future_addition = 0;
		let mut past_subtraction = 0;
		loop {
			future_addition += *predictions[i].last().expect("Each prediction should have at least one entry");
			past_subtraction = predictions[i][0] - past_subtraction;

			if i == 0 {
				break;
			}
			i -= 1;
		}
		future += future_addition;
		past += past_subtraction;
	}

	println!("Part1: {future}");
	println!("Part2: {past}");
}