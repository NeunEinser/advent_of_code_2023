use std::{process, fs, collections::{BinaryHeap, HashMap}, cmp::Ordering};
use self::Direction::*;

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/17
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path> [part1|part2]", args[0], args[1]);

	if args.len() < 3 || args.len() > 4 {
		eprintln!("{syntax}");
		process::exit(1);
	}
	let part2 = args.len() == 3 || args[3] == "part2";

	let nodes: Vec<Vec<_>> = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1)
		.lines().enumerate()
		.map(|(y, line)| line.bytes().enumerate().map(|(x, b)| Node { heat_loss: b - b'0', x, y }).collect())
		.collect();

	let start_node_east = PathNode {
		node: nodes[0][0],
		move_direction: East,
		move_direction_amount: 0,
	};
	let start_node_south = PathNode {
		move_direction: South,
		..start_node_east
	};
	let start_east = Step {
		path_node: start_node_east,
		total_heat_loss: 0,
	};

	let mut path_nodes = HashMap::from([(start_node_east, (0, None)), (start_node_south, (0, None))]);
	let mut heap = BinaryHeap::from([start_east, Step { path_node: start_node_south, ..start_east }]);
	// let mut staight_steps = 0;
	// let mut last_dir = Direction::None;

	let goal_y = nodes.len() - 1;
	let goal_x = nodes[goal_y].len() - 1;
	let mut goal_step = None;

	while let Some(step) = heap.pop() {
		if path_nodes.get(&step.path_node).map_or(false, |(heat_loss, _)| *heat_loss < step.total_heat_loss) {
			continue;
		}

		if step.path_node.node.x == goal_x && step.path_node.node.y == goal_y && (!part2 || step.path_node.move_direction_amount >= 4) {
			goal_step = Some(step);
			break;
		}

		for dir in Direction::iter() {
			if let Some((x, y)) = dir.move_coords(step.path_node.node.x, step.path_node.node.y) {
				if let Some(node) = nodes.get(y).and_then(|n| n.get(x)) {
					if dir == step.path_node.move_direction.get_opposite() {
						continue;
					}

					let dir_count = if step.path_node.move_direction == dir {
						step.path_node.move_direction_amount + 1
					} else {
						1
					};

					if (part2 && (dir_count > 10 || (step.path_node.move_direction_amount < 4 && dir != step.path_node.move_direction)))
					|| (!part2 && dir_count > 3) {
						continue;
					}

					let path_node = PathNode {
						move_direction: dir,
						move_direction_amount: dir_count,
						node: *node
					};
					let total_heat_loss = step.total_heat_loss + node.heat_loss as u64;
					let (previous_heat_loss, _) = path_nodes.entry(path_node).or_insert((u64::MAX, None));

					if total_heat_loss < *previous_heat_loss {
						*previous_heat_loss = total_heat_loss;
						path_nodes.insert(path_node, (total_heat_loss, Some(step)));
						heap.push(Step { path_node, total_heat_loss });
					}
				}
			}
		}
	}
	let mut traversed_path = HashMap::new();
	let mut step = goal_step;
	while let Some(cur_step) = step {
		let path = traversed_path.entry((cur_step.path_node.node.x, cur_step.path_node.node.y)).or_insert(0);
		*path |= 1 << cur_step.path_node.move_direction as u8;
		step = path_nodes.get(&cur_step.path_node).and_then(|s| s.1);
	}

	for (y, row) in nodes.iter().enumerate() {
		for (x, node) in row.iter().enumerate() {
			let char = if let Some(dir) = traversed_path.get(&(x, y)).copied() {
				let dir = Direction::iter().find(|d| 1 << *d as u8 == dir);
				match dir {
					Some(North) => '^',
					Some(East) => '>',
					Some(South) => 'v',
					Some(West) => '<',
					_ => 'M'
				}
			} else {
				(node.heat_loss + b'0') as char
			};
			print!("{char}");
		}
		println!();
	}
	println!("{:?}", goal_step.map(|s| s.total_heat_loss));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
	heat_loss: u8,
	x: usize,
	y: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PathNode {
	node: Node,
	move_direction: Direction,
	move_direction_amount: u8,
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Step {
	path_node: PathNode,
	total_heat_loss: u64,
}


impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_heat_loss.cmp(&self.total_heat_loss) // swapped, smaller value should be ranked higher
			.then_with(|| self.path_node.node.y.cmp(&other.path_node.node.y))
			.then_with(|| self.path_node.node.x.cmp(&other.path_node.node.x))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
	North,
	East,
	South,
	West
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        [North, South, East, West].iter().copied()
    }

	pub fn move_coords(&self, x: usize, y: usize) -> Option<(usize, usize)> {
		match self {
			North => Some((x, y.checked_sub(1)?)),
			East => Some((x + 1, y)),
			South => Some((x, y + 1)),
			West => Some((x.checked_sub(1)?, y)),
		}
	}

	pub fn get_opposite(&self) -> Self {
		match self {
			North => South,
			East => West,
			South => North,
			West => East
		}
	}
}