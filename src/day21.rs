use std::{process, fs, collections::HashSet};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/21
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path>", args[0], args[1]);

	if args.len() < 3 || args.len() > 4 {
		eprintln!("{syntax}");
		process::exit(1);
	}

	let mut start_x = 0;
	let mut start_y=  0;
	let grid: Vec<Vec<_>> = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1)
		.lines()
		.enumerate()
		.map(|(y, row)|
			row.bytes().enumerate()
			.map(|(x, c)| match c {
				b'.' => Tile::Ground,
				b'#' => Tile::Rock,
				b'S' => {
					start_x = x;
					start_y = y;
					Tile::Ground
				},
				b => {
					eprintln!("Unknown tile {}", b as char);
					process::exit(1);
				}
			})
			.collect()
		).collect();

	let mut next_fields = HashSet::from([(start_x, start_y)]);
	for _ in 0..26501365 {
		let current_fields = next_fields;
		next_fields = HashSet::new();

		for (x, y) in current_fields {
			if let Some(row) = grid.get(y + 1) {
				if let Some(Tile::Ground) = row.get(x) {
					next_fields.insert((x, y+1));
				}
			}
			if let Some(row) = grid.get(y + 1) {
				if let Some(Tile::Ground) = row.get(x) {
					next_fields.insert((x, y+1));
				}
			}
			if let Some(row) = grid.get(y) {
				if let Some(Tile::Ground) = row.get(x.wrapping_sub(1)) {
					next_fields.insert((x-1, y));
				}
				if let Some(Tile::Ground) = row.get(x+1) {
					next_fields.insert((x+1, y));
				}
			}
		}

		for (y, row) in grid.iter().enumerate() {
			for (x, t) in row.iter().enumerate() {
				if next_fields.contains(&(x, y)) {
					print!("O");
				} else {
					print!("{}", *t as u8 as char);
				}
			}
			println!();
		}
		
		println!("\n####################\n");
	}

	println!("{}", next_fields.len())
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
	Ground = b'.',
	Rock = b'#'
}