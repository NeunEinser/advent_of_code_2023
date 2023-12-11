use std::{process, fs, collections::HashSet};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/10
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path>", args[0], args[1]);

	if args.len() != 3 {
		eprintln!("{syntax}");
		process::exit(1);
	}
	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);
	let content: Vec<&str> = content.lines().collect();

	let start_y = content.iter().position(|l| l.contains('S')).unwrap_or_exit("Could not find Start", 1);
	let start_x = content[start_y].bytes().position(|c| c == b'S').expect("Line should contain start");

	let maze: Vec<Vec<Tile>> = content.iter().map(|l| {
		l.bytes().map(|b| {
			match b {
				b'|' => Tile::NS,
				b'-' => Tile::EW,
				b'L' => Tile::NE,
				b'J' => Tile::NW,
				b'7' => Tile::SW,
				b'F' => Tile::SE,
				b'.' => Tile::None,
				b'S' => {
					let mut ns = 0;
					let mut ew = 0;

					if start_y > 0 {
						let north = *content[start_y-1].as_bytes().get(start_x).unwrap_or_exit("Inconsistent line length", 1);
						if north == b'|' || north == b'7' || north == b'F' {
							ns = 1;
						}
					}
					if start_y + 1 < content.len() {
						let south = *content[start_y+1].as_bytes().get(start_x).unwrap_or_exit("Inconsistent line length", 1);
						if south == b'|' || south == b'L' || south == b'J' {
							ns = 2;
						}
					}
					if start_x + 1 < content[start_y].len() {
						let east = content[start_y].as_bytes()[start_x + 1];
						if east == b'-' || east == b'J' || east == b'7' {
							ew = 1;
						}
					}
					if start_x > 0 {
						let west = content[start_y].as_bytes()[start_x - 1];
						if west == b'-' || west == b'L' || west == b'F' {
							ew = 2;
						}
					}

					match ns {
						0 => Tile::EW,
						1 => if ew == 1 { Tile::NE } else { Tile::NW },
						2 => if ew == 1 { Tile::SE } else { Tile::SW },
						_ => unreachable!()
					}
				},
				b => {
					eprintln!("Invalid pipe symbol {}", b as char);
					process::exit(1);
				}
			}
		}).collect::<Vec<Tile>>()
	}).collect();

	// println!("Maze: {maze:#?}");

	let mut x = start_x;
	let mut y = start_y;
	let mut come_from = Direction::None;
	let mut pipe_loop = HashSet::from([(x, y)]);

	while pipe_loop.len() == 1 || x != start_x || y != start_y {
		let dirs = maze[y][x].directions();
		let dir = *dirs.iter().filter(|d| **d != come_from).next().unwrap();
		(x, y) = dir.apply(x, y);
		come_from = dir.opposite();
		pipe_loop.insert((x, y));
	}

	println!("Furthest point: {}", pipe_loop.len() / 2);

	let mut outside_tile_borders = HashSet::from([(0, 0)]);
	let mut new_borders = vec![(0, 0)];
	while !new_borders.is_empty() {
		let prev = new_borders;
		new_borders = Vec::new();
		for (x, y) in prev.iter().copied() {
			if y > 0 {
				if x == 0 || !pipe_loop.contains(&(x-1, y-1)) || !maze[y-1][x-1].directions().contains(&Direction::East) {
					if outside_tile_borders.insert((x, y-1)) {
						new_borders.push((x, y-1));
					}
				}
			}
			if y < maze.len() {
				if x == 0 || !pipe_loop.contains(&(x-1, y)) || !maze[y][x-1].directions().contains(&Direction::East) {
					if outside_tile_borders.insert((x, y+1)) {
						new_borders.push((x, y+1));
					}
				}
			}
			if x > 0 {
				if y == 0  || !pipe_loop.contains(&(x-1, y-1)) || !maze[y-1][x-1].directions().contains(&Direction::South) {
					if outside_tile_borders.insert((x-1, y)) {
						new_borders.push((x-1, y));
					}
				}
			}
			if x < maze[0].len() {
				if y == 0 || !pipe_loop.contains(&(x, y-1)) || !maze[y-1][x].directions().contains(&Direction::South) {
					if outside_tile_borders.insert((x+1, y)) {
						new_borders.push((x+1, y));
					}
				}
			}
		}
	}

	let mut outside_count = 0;
	for y in 0..maze.len() {
		for x in 0..maze[y].len() {
			if outside_tile_borders.contains(&(x, y)) && outside_tile_borders.contains(&(x+1, y)) && outside_tile_borders.contains(&(x, y+1)) && outside_tile_borders.contains(&(x+1, y+1)) {
				outside_count += 1;
			}
		}
	}
	println!("Tiles inside loop: {}", maze.len() * maze[0].len() - pipe_loop.len() - outside_count);
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, Eq, PartialEq, PartialOrd, Hash)]
enum Tile {
	NS = b'|',
	EW = b'-',
	NE = b'L',
	NW = b'J',
	SW = b'7',
	SE = b'F',
	None = b'.',
}

impl Tile {
	pub fn directions(&self) -> [Direction; 2] {
		match self {
			Tile::NS => [ Direction::North, Direction::South ],
			Tile::EW => [ Direction::East, Direction::West ],
			Tile::NE => [ Direction::North, Direction::East ],
			Tile::NW => [ Direction::North, Direction::West ],
			Tile::SW => [ Direction::South, Direction::West ],
			Tile::SE =>  [ Direction::South, Direction::East ],
			Tile::None => [ Direction::None; 2 ],
		}
	}
}

#[derive(Debug, Copy, Clone, Ord, Eq, PartialEq, PartialOrd, Hash)]
enum Direction {
	North,
	East,
	South,
	West,
	None
}

impl Direction {
	pub fn apply(&self, x: usize, y: usize) -> (usize, usize) {
		match self {
			Direction::North => (x, y-1),
			Direction::East => (x+1, y),
			Direction::South => (x, y+1),
			Direction::West => (x-1, y),
			Direction::None => (x, y), 
		}
	}

	pub fn opposite(&self) -> Self {
		match self {
			Direction::North => Direction::South,
			Direction::East => Direction::West,
			Direction::South => Direction::North,
			Direction::West => Direction::East,
			Direction::None => Direction::None,
}
	}
}