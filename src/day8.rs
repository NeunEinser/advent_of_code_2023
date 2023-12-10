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

	let mut count = 1u64;
	let mut path = first_line.chars().cycle();
	let mut unknown_cycle: Vec<(&str, Vec<(&str, u64, u64)>)> = graph.keys().filter(|n| n.ends_with('A')).map(|n| (*n, Vec::new())).collect();
	let mut cycles = Vec::new();

	while !unknown_cycle.is_empty() {
		let instr = path.next().expect("Path should cycle and not be empty here.");
		let mut i = 0;
		while i < unknown_cycle.len() {
			let cycle = unknown_cycle.get_mut(i).expect("index out of bounds even though index is smaller than length");
			let continuations = graph.get(cycle.0).unwrap_or_exit(&format!("Could not find node {}", cycle.0), 1);
			cycle.0 = match instr {
				'L' => continuations.0,
				'R' => continuations.1,
				i => {
					eprintln!("Found invalid instruction {i}, expected R or L");
					process::exit(1);
				}
			};
			if cycle.0.ends_with('Z') {
				let path_i = count % first_line.len() as u64;
				let prev_count: u64 = cycle.1.iter().map(|c| c.2).sum();
				if let Some(cycle_ind) = cycle.1.iter().position(|c| *c == (cycle.0, path_i, count - prev_count)) {
					let (_, cycle) = unknown_cycle.remove(i);
					cycles.push((cycle.iter().map(|c| c.2).collect::<Vec<u64>>(), cycle_ind));
					continue;
				} else {
					cycle.1.push((cycle.0, path_i, count - prev_count))
				}
			}
			i += 1;
		}
		count += 1;
	}

	if cycles.iter().any(|c| c.0.len() != 1) {
		eprintln!("In the puzzle input, all cycles have a consistent length, but found cycle with inconsistent length");
		process::exit(1);
	}

	let mut res = 1u64; 
	for (v, _) in cycles {
		res = lcm(res, v[0]);
	}
	println!("Part2: {res} steps required");

}

fn gcd(a: u64, b: u64) -> u64 {
	let mut a = a;
	let mut b = b;

	while a % b > 0 {
		let r = a % b;
		a = b;
		b = r;
	}
	b
}

fn lcm(a: u64, b: u64) -> u64 {
	a / gcd(a, b) * b
}