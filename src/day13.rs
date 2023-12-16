use std::{process, fs};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/13
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path> [part1|part2]", args[0], args[1]);

	if args.len() < 3 || args.len() > 4 {
		eprintln!("{syntax}");
		process::exit(1);
	}
	let part2 = args.len() == 3 || args[3] == "part2";

	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);
	let patterns: Vec<Vec<Vec<bool>>> = content
		.split("\n\n")
		.map(|p| p.lines().map(|l| l.chars().map(|c| c == '#').collect()).collect())
		.collect();

	let mut solution = 0;
	'pattern: for pattern in &patterns {
		'axis: for mirror_axis in 1..pattern.first().unwrap_or_exit("Found empty pattern", 1).len() {
			let start = (mirror_axis*2).saturating_sub(pattern[0].len());
			let mut found_smudge = false; 
			for x in start..mirror_axis {
				let other_x = mirror_axis*2-1-x;
				
				for row in pattern {
					if row[x] != row[other_x] {
						if !part2 || found_smudge {
							continue 'axis;
						}
						found_smudge = true;
					}
				}
			}
			if part2 && !found_smudge {
				continue 'axis;
			}
			solution += mirror_axis;
			for _ in 0..mirror_axis-1 {
				print!(" ");
			}
			println!("><");
			for row in pattern {
				for val in row {
					print!("{}", if *val { '#' } else {'.'});
				}
				println!();
			}
			println!();
			continue 'pattern;
		}

		'axis: for mirror_axis in 1..pattern.len() {
			let start = (mirror_axis*2).saturating_sub(pattern.len());
			let mut found_smudge = false; 
			for y in start..mirror_axis {
				let other_y = mirror_axis*2-1-y;
				
				for x in 0..pattern[0].len() {
					if pattern[y][x] != pattern[other_y][x] {
						if !part2 || found_smudge {
							continue 'axis;
						}
						found_smudge = true;
					}
				}
			}
			if part2 && !found_smudge {
				continue 'axis;
			}
			solution += 100 * mirror_axis;
			for (y, row) in pattern.iter().enumerate() {
				if y == mirror_axis-1 {
					print!("v");
				}
				else if y == mirror_axis {
					print!("^");
				}
				else {
					print!(" ");
				}
				for val in row {
					print!("{}", if *val { '#' } else {'.'});
				}
				println!();
			}
			println!();
			continue 'pattern;
		}
		
		eprintln!("Pattern does not have any mirror axis:");
		for line in pattern {
			for val in line {
				eprint!("{}", if *val { '#' } else {'.'});
			}
			eprintln!();
		}
		process::exit(1);
	}

	println!("{solution}")
}

