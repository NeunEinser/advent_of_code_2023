use std::{process, fs};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/12
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path>", args[0], args[1]);

	if args.len() != 3 {
		eprintln!("{syntax}");
		process::exit(1);
	}
	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);
	
	let mut sum = 0;
	for line in content.lines() {
		let (springs, nums) = line.split_once(' ').unwrap_or_exit(&format!("Could not parse line {line}"), 1);
		let springs: Vec<_> = springs.chars().map(|c| match c {
			'.' => SpringState::Operational,
			'#' => SpringState::Damaged,
			_ => SpringState::Unknown
		}).collect();
		let nums: Result<Vec<usize>, _> = nums.split(',').map(str::parse).collect();
		let nums = nums.unwrap_or_exit(&format!("Could not parse num section of line {line}"), 1);

		sum += get_arrangements(&springs, &nums);
	}

	println!("{sum}");
}

fn get_arrangements(springs: &[SpringState], nums: &[usize]) -> u32 {
	if springs.is_empty() || nums.is_empty() {
		return 0;
	}
	let mut springs = springs;
	let mut arrangements = 0;

	while !springs.is_empty() {
		if springs.len() >= nums[0]
		&& springs[..nums[0]].iter().all(|s| *s == SpringState::Damaged || *s == SpringState::Unknown)
		&& (springs.len() == nums[0] || springs[nums[0]] == SpringState::Operational || springs[nums[0]] == SpringState::Unknown) {
			arrangements += if nums.len() > 1 {
				if springs.len() > nums[0]+1 {
					get_arrangements(&springs[nums[0]+1..], &nums[1..])
				} else {
					0
				}
			} else if springs.len() <= nums[0] || springs[nums[0]..].iter().all(|s| *s == SpringState::Unknown || *s == SpringState::Operational) {
				1
			} else {
				0
			};
		}

		if springs[0] == SpringState::Damaged {
			break;
		}
		springs = &springs[1..];
	}

	return arrangements;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum SpringState {
	Unknown,
	Operational,
	Damaged,
}