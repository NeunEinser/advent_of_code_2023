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
	let mut last_splits = vec![(0, 0, 0)];

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

	for (y, row) in grid.iter().enumerate() {
		for (x, tile) in row.iter().copied().enumerate() {
			if tile == Tile::Empty && beam.get(&(x, y)).copied().unwrap_or(0) != 0 {
				match beam[&(x, y)] {
					RIGHT_FLAG => print!(">"),
					LEFT_FLAG => print!("<"),
					UP_FLAG => print!("^"),
					DOWN_FLAG => print!("v"),
					b => print!("{b:x}")
				}
			} else {
				print!("{}", tile as u8 as char);
			}
		}
		println!()
	}
	println!("{}", beam.len());
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

const RIGHT_FLAG: u8 = 1 << RIGHT;
const LEFT_FLAG: u8 = 1 << LEFT;
const UP_FLAG: u8 = 1 << UP;
const DOWN_FLAG: u8 = 1 << DOWN;

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