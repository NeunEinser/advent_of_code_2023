use std::{process, fs, collections::HashSet, cmp};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/18
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path>", args[0], args[1]);

	if args.len() < 3 || args.len() > 4 {
		eprintln!("{syntax}");
		process::exit(1);
	}

	let mut trench_coords = HashSet::from([(0, 0)]);
	let mut x = 0;
	let mut y = 0;
	let mut min_x = 0;
	let mut min_y = 0;
	let mut max_x = 0;
	let mut max_y = 0;

	for line in fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1).lines() {
		let mut parts = line.splitn(3, ' ');
		let direction: Direction = parts.next()
			.unwrap_or_exit(&format!("Line {line} is invalid"), 1)
			.bytes().next()
			.unwrap_or_exit(&format!("Line {line} is invalid"), 1)
			.try_into()
			.unwrap_or_exit(&format!("Line {line} is invalid"), 1);

		let amount = parts.next()
			.unwrap_or_exit(&format!("Line {line} is invalid"), 1)
			.parse()
			.unwrap_or_exit(&format!("Line {line} is invalid"), 1);

		for _ in 0..amount {
			(x, y) = direction.move_coords(x, y);
			trench_coords.insert((x, y));
			min_x = cmp::min(min_x, x);
			min_y = cmp::min(min_y, y);
			max_x = cmp::max(max_x, x);
			max_y = cmp::max(max_y, y);
		}
	}

	let mut start_x = min_x;
	let mut start_y = min_y;
	'outer: for x in min_x + 1..max_x {
		for y in min_y..max_y {
			if trench_coords.contains(&(x, y)) {
				if trench_coords.contains(&(x, y + 1)) {
					break;
				}

				start_x = x;
				start_y = y+1;
				break 'outer;
			}
		}
	}

	println!("{start_x}, {start_y}");
	let mut unprocessed = vec![(start_x, start_y)];

	while let Some((x, y)) = unprocessed.pop() {
		if trench_coords.contains(&(x, y)) {
			continue;
		}
		trench_coords.insert((x, y));

		unprocessed.extend([(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]);
	}

	println!("{}", trench_coords.len());
}

#[repr(u8)]
enum Direction {
	Up = b'U',
	Down = b'D',
	Left = b'L',
	Right = b'R',
}

impl Direction {
	pub fn move_coords(&self, x: i32, y: i32) -> (i32, i32) {
		match self {
			Self::Up => (x, y - 1),
			Self::Down => (x, y + 1),
			Self::Left => (x - 1, y),
			Self::Right => (x + 1, y),
		}
	}
}

impl TryFrom<u8> for Direction {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			b'U' => Ok(Self::Up),
			b'D' => Ok(Self::Down),
			b'L' => Ok(Self::Left),
			b'R' => Ok(Self::Right),
			_ => Err("Invalid direction")
		}
    }
}