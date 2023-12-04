use std::{process, fs, cmp};

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
	let mut cards: Vec<usize> = (0..lines.len()).map(|_| 1).collect();
	for  (i, line) in lines.iter().enumerate() {
		let start = line.find(':').unwrap_or_exit(&format!("line {line} does not contain a valid card (missing colon ':')"), 1) + 1;
		let end = line.find('|').unwrap_or_exit(&format!("line {line} does not contain a valid card (missing pipe '|')"), 1);

		let mut numstr = line[start..end].trim();
		let mut nums = Vec::new();
		while !numstr.is_empty() {
			let num_end = numstr.find(|c: char| !c.is_ascii_digit()).unwrap_or(numstr.len());
			let num = &numstr[..num_end];
			nums.push(num.parse::<u32>().unwrap_or_exit(&format!("line {line} does not contain a valid card (unable to parse numbers)"), 1));
			numstr = numstr[num_end..].trim();
		}
		
		let mut numstr = line[end+2..].trim();
		let mut points = 0;
		let mut count = 0;
		while !numstr.is_empty() {
			let num_end = numstr.find(|c: char| !c.is_ascii_digit()).unwrap_or(numstr.len());
			let num = &numstr[..num_end];
			let num = num.parse::<u32>().unwrap_or_exit(&format!("line {line} does not contain a valid card (unable to parse numbers)"), 1);

			if nums.contains(&num) {
				points *= 2;
				points = cmp::max(points, 1);
				count += 1
			}
			numstr = numstr[num_end..].trim();
		}

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