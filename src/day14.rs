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

	// part 1 (kept separetley because this solution is cool but completely
	// incompatible with part2)
	let mut round_rocks = vec![0; content.chars().position(|c| c=='\n').unwrap_or_exit("Only a single line of input", 1)];

	let mut sum = 0;
	for (row, line) in content.lines().rev().enumerate() {
		sum += calculate_row_part1(line, &mut round_rocks, row);
	}

	sum += calculate_row_part1(&iter::repeat('#').take(round_rocks.len()).collect::<String>(), &mut round_rocks, content.lines().count());

	println!("Part 1: {sum}");

	let mut arrangement: Vec<Vec<Tile>> = content.lines().map(|l| l.bytes().map(|c| {
		match c {
			b'#' => Tile::Stopper,
			b'O' => Tile::RoundRock,
			_ => Tile::Empty
		}
	}).collect()).collect();

	let mut previous = Vec::new();
	let loop_index = 'outer: loop {
		tilt_north(&mut arrangement);
		tilt_west(&mut arrangement);
		tilt_south(&mut arrangement);
		tilt_east(&mut arrangement);

		let mut cur_slice = &previous[..];
		while let Some(distance) = cur_slice.iter().rev().position(|v| v == &arrangement) {
			if 2*distance >= previous.len() {
				break;
			}
			let other = previous.len() - distance - 1;
			if arrangement != previous[other] {
				cur_slice = &cur_slice[..distance];
				continue;
			}

			break 'outer other;
		}
		// println!("{}", caclulate_load(&arrangement));
		// print(&arrangement);
		previous.push(arrangement.clone());
	};
	// println!("{}", caclulate_load(&arrangement));
	// print(&arrangement);

	println!("loop: {}-{}", loop_index, previous.len());

	let missing_iters = 1_000_000_000 - previous.len() - 1;
	let offset = missing_iters % (previous.len() - loop_index);

	println!("Part 2: {}", caclulate_load(&previous[loop_index + offset]));
}

fn calculate_row_part1(line: &str, round_rocks: &mut [usize], row: usize) -> usize {
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

fn tilt_north(arrangement: &mut [Vec<Tile>]) {
	for x in 0..arrangement[0].len() {
		let mut dest_y = 0;

		for y in 0..arrangement.len() {
			match arrangement[y][x] {
				Tile::Stopper => dest_y = y + 1,
				Tile::RoundRock => {
					arrangement[y][x] = Tile::Empty;
					arrangement[dest_y][x] = Tile::RoundRock;
					dest_y += 1;
				},
				_ => ()
			}
		}
	}
}

fn tilt_west(arrangement: &mut [Vec<Tile>]) {
	for y in 0..arrangement.len() {
		let mut dest_x = 0;

		for x in 0..arrangement[y].len() {
			match arrangement[y][x] {
				Tile::Stopper => dest_x = x + 1,
				Tile::RoundRock => {
					arrangement[y][x] = Tile::Empty;
					arrangement[y][dest_x] = Tile::RoundRock;
					dest_x += 1;
				},
				_ => ()
			}
		}
	}
}

fn tilt_south(arrangement: &mut [Vec<Tile>]) {
	for x in 0..arrangement[0].len() {
		let mut dest_y = arrangement.len() - 1;

		for y in (0..arrangement.len()).rev() {
			match arrangement[y][x] {
				Tile::Stopper => dest_y = y.overflowing_sub(1).0,
				Tile::RoundRock => {
					arrangement[y][x] = Tile::Empty;
					arrangement[dest_y][x] = Tile::RoundRock;
					dest_y = dest_y.overflowing_sub(1).0;
				},
				_ => ()
			}
		}
	}
}

fn tilt_east(arrangement: &mut [Vec<Tile>]) {
	for y in 0..arrangement.len() {
		let mut dest_x = arrangement[y].len() - 1;

		for x in (0..arrangement[y].len()).rev() {
			match arrangement[y][x] {
				Tile::Stopper => dest_x = x.overflowing_sub(1).0,
				Tile::RoundRock => {
					arrangement[y][x] = Tile::Empty;
					arrangement[y][dest_x] = Tile::RoundRock;
					dest_x = dest_x.overflowing_sub(1).0;
				},
				_ => ()
			}
		}
	}
}

fn caclulate_load(arrangement: &[Vec<Tile>]) -> usize{
	let mut sum = 0;
	for (y, row) in arrangement.iter().rev().enumerate() {
		sum += (y+1) * row.iter().filter(|t| **t == Tile::RoundRock).count();
	}
	sum
}

#[allow(dead_code)]
fn print(arrangement: &[Vec<Tile>]) {
	for row in arrangement.iter() {
		for tile in row.iter() {
			print!("{}", *tile as u8 as char);
		}
		println!();
	}
	println!();
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Tile {
	Stopper = b'#',
	RoundRock = b'O',
	Empty = b'.',
}

// 0  3
// 1  2
// 2  1
// 3  0
// 4  -1
// 5  3
// 6  2
// 7  1
// 8  0
// 9 -1


// 0 0
// 1 -1
// 2 0
// 3 -1