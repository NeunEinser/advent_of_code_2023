use std::{process, fs, cmp};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/11
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path> [part1|part2]", args[0], args[1]);

	if args.len() < 3 || args.len() > 4 {
		eprintln!("{syntax}");
		process::exit(1);
	}
	let part2 = args.len() == 3 || args[3] == "part2";
	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);

	let mut galaxies = Vec::new();
	let mut y: u64 = 0;
	let mut width = 0;
	for line in content.lines() {
		if line.chars().all(|c| c == '.') {
			y += if part2 { 1_000_000 } else { 2 };
			continue;
		}
		for x in line.chars().enumerate().filter(|(_, c)| *c == '#').map(|(x, _)| x as u64) {
			galaxies.push((x, y));
			width = cmp::max(width, x);
		}
		y += 1;
	}
	
	for x in (0..=width).rev() {
		if !galaxies.iter().any(|g| g.0 == x) {
			for galaxy in galaxies.iter_mut().filter(|g| g.0 > x) {
				galaxy.0 += if part2 { 999_999 } else { 1 };
			}
		}
	}

	let mut sum = 0;
	for (i, (x, y)) in galaxies.iter().copied().enumerate() {
		for (other_x, other_y) in galaxies.iter().skip(i+1).copied() {
			let dist = x.abs_diff(other_x) + y.abs_diff(other_y);
			// println!("distance between {i} ({x}, {y}) and {j} ({other_x}, {other_y}): {dist}");
			sum += dist
		}
	}

	println!("{sum}");
}