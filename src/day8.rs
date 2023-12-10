use std::{process, fs, collections::HashMap};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/8
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path>", args[0], args[1]);

	if args.len() != 3 {
		eprintln!("{syntax}");
		process::exit(1);
	}
	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);
 	let mut content = content.lines();
	let first_line = content.next().unwrap_or_exit("Received empty input", 1);
	if first_line.is_empty() {
		eprintln!("Right/Left instructions were empty!");
		process::exit(1);
	}
	
	
	let graph: HashMap<&str, (&str, &str)> = content
		.skip_while(|l| l.chars().all(|c| c.is_whitespace()))
		.map(|l| {
			let parts = l.split_once(" = ").unwrap_or_exit(&format!("Unable to parse node {l}"), 1);
			(
				parts.0,
				parts.1
					.get(1..parts.1.len() - 1)
					.unwrap_or_exit(&format!("Unable to parse node {l}"), 1)
					.split_once(", ")
					.unwrap_or_exit(&format!("Unable to parse node {l}"), 1)
			)
		})
		.collect();

	let mut count = 0;
	let mut node = "AAA";
	let mut path = first_line.chars().cycle();

	while node != "ZZZ" {
		let continuations = graph.get(node).unwrap_or_exit(&format!("Could not find node {node}"), 1);
		match path.next().expect("Path should cycle and not be empty here.") {
			'L' => node = continuations.0,
			'R' => node = continuations.1,
			i => {
				eprintln!("Found invalid instruction {i}, expected R or L");
				process::exit(1);
			}
		}
		count += 1;
	}
	println!("Part1: {count} steps required");

}