use std::{process, fs, collections::BTreeMap, cmp, ops::RangeInclusive};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/18
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path> [part1|part2]", args[0], args[1]);

	if args.len() < 3 || args.len() > 4 {
		eprintln!("{syntax}");
		process::exit(1);
	}
	let part2 = args.len() == 3 || args[3] == "part2";

	let mut trenches = Vec::new();
	let mut x = 0;
	let mut y = 0;
	let mut min_x = 0;
	let mut min_y = 0;
	let mut max_x = 0;
	let mut max_y = 0;

	for line in fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1).lines() {
		// Uncomment for part 1
		let (direction, amount) = if part2 {
			let parts = &line.rsplit_once(' ').unwrap().1[2..8];
			(
				match &parts[5..6] {
					"0" => Direction::Right,
					"1" => Direction::Down,
					"2" => Direction::Left,
					"3" => Direction::Up,
					_ => {
						eprintln!("Invalid direction in color code");
						process::exit(1);
					}
				},
				i64::from_str_radix(&parts[0..5], 16).unwrap()
			)

		} else {
			let mut parts = line.splitn(3, ' ');
			(
				parts.next()
					.unwrap_or_exit(&format!("Line {line} is invalid"), 1)
					.bytes().next()
					.unwrap_or_exit(&format!("Line {line} is invalid"), 1)
					.try_into()
					.unwrap_or_exit(&format!("Line {line} is invalid"), 1),
				parts.next()
					.unwrap_or_exit(&format!("Line {line} is invalid"), 1)
					.parse::<i64>()
					.unwrap_or_exit(&format!("Line {line} is invalid"), 1)
			)
		};


		let prev_x = x;
		let prev_y = y;
		(x, y) = direction.move_coords_by(x, y, amount as i64);
		min_x = cmp::min(min_x, x);
		min_y = cmp::min(min_y, y);
		max_x = cmp::max(max_x, x);
		max_y = cmp::max(max_y, y);

		if prev_x != x {
			let start = cmp::min(prev_x, x);
			let end = cmp::max(prev_x, x);
			trenches.push((y, start..=end, direction))
		} else {
			let start = cmp::min(prev_y, y);
			let end = cmp::max(prev_y, y);
			trenches.push((x, start..=end, direction))
		}
	}

	let x_trenches: Vec<_> = trenches.iter().enumerate()
		.filter(|(_, (_, _, dir))| *dir == Direction::Left || *dir == Direction::Right)
		.map(|(i, (y, x_range, dir))| {
			let (_, _, prev_dir) = &trenches[if i > 0 {i-1} else {trenches.len()-1}];
			let (_, _, next_dir) = &trenches[if i + 1 < trenches.len() {i+1} else {0}];

			if *dir == Direction::Right {
				HorizontalEdge {
					y: *y,
					x: x_range.clone(),
					left_dir: prev_dir.get_opposite(),
					right_dir: *next_dir
				}
			} else {
				HorizontalEdge {
					y: *y,
					x: x_range.clone(),
					left_dir: *next_dir,
					right_dir: prev_dir.get_opposite()
				}
			}
		})
		.collect();

	let mut x = min_x;
	let mut sum = 0;
	let mut rectangles = Vec::new();
	loop {
		let mut ranges: BTreeMap<_, _> = x_trenches.iter()
			.filter(|e| e.x.contains(&x))
			.map(|e| (e.y, e))
			.collect();

		let mut max = None;
		let mut inside = false;
		let mut i = 0;
		let mut prev_unskipped: Option<&HorizontalEdge> = None;
		let mut prev_skipped: Option<&HorizontalEdge> = None;

		while let Some(y) = ranges.keys().copied().nth(i) {
			let value: &HorizontalEdge = ranges[&y];

			if let Some(prev) = prev_skipped {
				if inside && ((*value.x.start() == x && *prev.x.start() == x) || (*value.x.end() == x && *prev.x.end() == x)) {
					ranges.remove(&y);
					prev_skipped = None;
					continue;
				}
			} else if inside && ((*value.x.start() == x && value.left_dir == Direction::Down) || (*value.x.end() == x && value.right_dir == Direction::Down)) {
				if *value.x.start() == x && value.left_dir == Direction::Down {
					max = Some(*value.x.start());
				}
				ranges.remove(&y);
				prev_skipped = Some(value);
				continue;
			} else if let Some(prev) = prev_unskipped {
				if inside && prev_skipped.is_none() && ((*value.x.start() == x && *prev.x.end() == x) || (*value.x.end() == x && *prev.x.start() == x)) {
					ranges.remove(&y);
					continue;
				}
			}

			let end = if inside == (value.right_dir == Direction::Down) {
				*value.x.end() - 1
			} else {
				*value.x.end()
			};
			max = Some(cmp::min(max.unwrap_or(i64::MAX), end));
			i += 1;
			inside = !inside;
			prev_skipped = None;
			prev_unskipped = Some(value);
		}

		let max = match max {
			Some(v) => v,
			None => break,
		};

		let others: Vec<_> = x_trenches.iter()
			.filter(|e| *e.x.start() > x &&  *e.x.start() <= max)
			.collect();

		let max = others.iter()
			.map(|e| *e.x.start() - 1)
			.min()
			.unwrap_or(max)
			.min(max);

		let amount = max - x + 1;

		let mut ranges = ranges.iter();
		while let Some((&y, _)) = ranges.next() {
			let (&other_y, _) = ranges.next().unwrap_or_exit("Detected open perimeter", 1);
			rectangles.push(((x, y), (max, other_y)));

			sum += (other_y - y + 1) * amount;
		}

		x = max + 1;
	}

	if !part2 {
		for y in min_y..=max_y {
			for x in min_x..=max_x {
				if rectangles.iter().any(|&(p1, p2)| x >= p1.0 && x <= p2.0 && y >= p1.1 && y <= p2.1) {
					print!("#");
				} else {
					print!(".");
				}
			}
			println!();
		}
	}

	println!("{sum}");
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HorizontalEdge {
	y: i64,
	x: RangeInclusive<i64>,
	left_dir: Direction,
	right_dir: Direction,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
	Up = b'U',
	Down = b'D',
	Left = b'L',
	Right = b'R',
}

impl Direction {
	pub fn move_coords_by(&self, x: i64, y: i64, amount: i64) -> (i64, i64) {
		match self {
			Self::Up => (x, y - amount),
			Self::Down => (x, y + amount),
			Self::Left => (x - amount, y),
			Self::Right => (x + amount, y),
		}
	}
	pub fn get_opposite(&self) -> Self {
		match self {
			Self::Up => Self::Down,
			Self::Down => Self::Up,
			Self::Left => Self::Right,
			Self::Right => Self::Left,
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