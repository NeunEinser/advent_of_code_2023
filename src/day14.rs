use std::{process, fs, iter};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/14
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path>", args[0], args[1]);

	if args.len() < 3 || args.len() > 4 {
		eprintln!("{syntax}");
		process::exit(1);
	}

	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);

	let mut round_rocks = vec![0; content.chars().position(|c| c=='\n').unwrap_or_exit("Only a single line of input", 1)];

	let mut sum = 0;
	for (row, line) in content.lines().rev().enumerate() {
		sum += calculate_row(line, &mut round_rocks, row);
	}

	sum += calculate_row(&iter::repeat('#').take(round_rocks.len()).collect::<String>(), &mut round_rocks, content.lines().count());

	println!("{sum}")
}

fn calculate_row(line: &str, round_rocks: &mut Vec<usize>, row: usize) -> usize {
	let mut sum = 0;
    for (i, c) in line.chars().enumerate() {
		let count = round_rocks.get_mut(i).unwrap_or_exit("Inconsistent line length", 1);
				
		match c {
			'O' => *count += 1,
			'#' => {
				// modified gauss summation `n * (max+min) / 2`
				// min = max - n + 1
				// <=> max+min = 2*max - n + 1
				sum += (*count * (2*row - *count + 1)) / 2;
				*count = 0;
			},
			_ => ()
		}
	}
	sum
}

