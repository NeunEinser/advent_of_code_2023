use std::{process, fs};

use crate::UnwrapOrExit;

/// https://adventofcode.com/2023/day/4
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path>", args[0], args[1]);

	if args.len() != 3 {
		eprintln!("{syntax}");
		process::exit(1);
	}

	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);
	let lines: Vec<&str> = content.lines().collect();

	let mut sum = 0;
	let mut cards = vec![1; lines.len()];
	for  (i, line) in lines.iter().enumerate() {
		let numstrs = line
			.split_once(':')
			.and_then(|(_, s)| s.split_once('|'))
			.unwrap_or_exit(&format!("line {line} does not contain a valid card (unable to split winning numbers from own numbers)"), 1);

		let nums: Vec<u32> = numstrs.0
			.split_whitespace()
			.map(|n| n.parse().unwrap_or_exit(&format!("line {line} does not contain a valid card (unable to parse winning numbers)"), 1))
			.collect();
		
		let count = numstrs.1
			.split_whitespace()
			.filter(|n| nums.contains(&n.parse().unwrap_or_exit(&format!("line {line} does not contain a valid card (unable to parse own numbers)"), 1)))
			.count();

		let points = if count > 0 { 2u32.pow(count as u32 - 1) } else { 0 };

		let instances = cards[i];
		for num in i+1..i+count+1 {
			let val = cards.get_mut(num).unwrap_or_exit(&format!("Won unknown card {num}"), 1);
			*val += instances;
		}

		sum += points
	}

	println!("Sum of points: {sum}");
	println!("Card counts: {:?}", cards);
	println!("Sum of card counts: {}", cards.iter().sum::<usize>());
}