use std::{process, fs, collections::HashMap};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/16
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path>", args[0], args[1]);

	if args.len() < 3 || args.len() > 4 {
		eprintln!("{syntax}");
		process::exit(1);
	}

	let grid: Vec<Vec<Tile>> = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1)
		.lines()
		.map(|l| l.bytes().map(|t| t.into()).collect())
		.collect();

	let mut beam = HashMap::new();
	let mut last_splits = Vec::new();
	let mut max_beam = 0;

	for (start_x, start_y, start_dir) in (0..=3).flat_map(|i| {
		// I tried returning (0..len).map(|j| (...)) with the map doing the same thing
		// as below, but Rust complains because it can't figure out the exact type,
		// as different closures are incompatible.
		// It feels like there should be some way to use a trait object here, but 
		// maybe not. I don't really understand Rust's closure type system. 
		let inner = match i {
			RIGHT | LEFT  => 0..grid.len(),
			UP | DOWN  => 0..grid[0].len(),
			_ => unreachable!(),
		};

		// The rust compiler told me I need to use move, and then it told me I
		// cannot move grid out of scope, so I make a copy of the required
		// lengths instead.
		let max_x = grid[0].len() - 1;
		let max_y = grid.len() - 1;
		inner.map(move |c| match i {
			RIGHT => (0, c, RIGHT),
			LEFT => (max_x, c, LEFT),
			DOWN => (c, 0, DOWN),
			UP => (c, max_y, UP),
			_ => unreachable!(),
		})
	}) {
		beam.clear();
		last_splits.clear();
		last_splits.push((start_x, start_y, start_dir));

		while !last_splits.is_empty() {
			while let Some((x, y, i)) = last_splits.pop() {
				let mut x = x;
				let mut y = y;
				let mut dir = i;
				while let Some(t) = grid.get(y).and_then(|r| r.get(x)) {
					dir = match t {
						Tile::MirrorTLBR => {
							match dir {
								RIGHT => DOWN,
								LEFT => UP,
								DOWN => RIGHT,
								UP => LEFT,
								_ => unreachable!()
							}
						},
						Tile::MirrorBLTR => {
							match dir {
								RIGHT => UP,
								LEFT => DOWN,
								DOWN => LEFT,
								UP => RIGHT,
								_ => unreachable!()
							}
						},
						Tile::SplitterH => {
							if dir == UP || dir == DOWN {
								last_splits.push((x, y, RIGHT));
								LEFT
							} else {
								dir
							}
						},
						Tile::SplitterV => {
							if dir == LEFT || dir == RIGHT {
								last_splits.push((x, y, UP));
								DOWN
							} else {
								dir
							}
						},
						Tile::Empty => dir,
					};
					
					
					let entry = beam.entry((x, y)).or_insert(0);
					let dir_flag = !*entry & (1 << dir);
					if dir_flag == 0 {
						break;
					}
					*entry |= dir_flag;
					match dir {
						RIGHT => x += 1,
						LEFT => x = x.wrapping_sub(1),
						DOWN => y += 1,
						UP =>  y = y.wrapping_sub(1),
						_ => unreachable!()
					}
				}
			}
		}

		if max_beam < beam.len() {
			if max_beam == 0 {
				println!("Part 1: {}", beam.len());
			}
			max_beam = beam.len();
		}
	}
	
	println!("Part 2: {}", max_beam);
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Tile {
	Empty = b'.',
	MirrorTLBR = b'\\',
	MirrorBLTR = b'/',
	SplitterH = b'-',
	SplitterV = b'|',
}

const RIGHT: u8 = 0;
const LEFT: u8 = 1;
const UP: u8 = 2;
const DOWN: u8 = 3;

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
			b'\\' => Tile::MirrorTLBR,
			b'/' => Tile::MirrorBLTR,
			b'-'=> Tile::SplitterH,
			b'|' => Tile::SplitterV,
			_ => Tile::Empty
		}
    }
}