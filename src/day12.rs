use std::{process, fs, iter, collections::HashMap};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/12
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path> [part1|part2]", args[0], args[1]);

	if args.len() < 3 || args.len() > 4 {
		eprintln!("{syntax}");
		process::exit(1);
	}
	let part2 = args.len() == 3 || args[3] == "part2";
	let repetitions = if part2 { 5 } else { 1 };

	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);
	
	let mut sum = 0;
	for line in content.lines() {
		let (springs, nums) = line.split_once(' ').unwrap_or_exit(&format!("Could not parse line {line}"), 1);
		let springs: Vec<_> = iter::repeat(
			springs.chars().map(|c| match c {
				'.' => SpringState::Operational,
				'#' => SpringState::Damaged,
				_ => SpringState::Unknown
			}))
			.take(repetitions)
			.flat_map(|i| i.chain(iter::once(SpringState::Unknown)))
			.take((springs.len()+1) * repetitions - 1)
			.collect();

		let nums: Result<Vec<usize>, _> = iter::repeat(
			nums.split(',').map(str::parse))
			.take(repetitions)
			.flatten()
			.collect();
		let nums = nums.unwrap_or_exit(&format!("Could not parse num section of line {line}"), 1);

		sum += get_arrangements(&springs, &nums, &mut HashMap::new());
	}

	println!("{sum}");
}

fn get_arrangements(springs: &[SpringState], nums: &[usize], cache: &mut HashMap<(usize, usize), u64>) -> u64 {
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
				if springs.len() <= nums[0]+1 {
					break;
				}
				// This would be nicer as cache.entry(...).or_insert_with(...), but the borrow checker cries there
				// because the recursive call requires the cache, borrowing the cache mutable twice.
				let springs = get_slice_with_first_non_operational_after(springs, nums[0]+1);
				let nums = &nums[1..];
				let key = (springs.len(), nums.len());
				if cache.contains_key(&key) {
					cache[&key]
				} else {
					let result = get_arrangements(springs, nums, cache);
					cache.insert(key, result);
					result
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
		springs = get_slice_with_first_non_operational_after(springs, 1);
	}

	arrangements
}

fn get_slice_with_first_non_operational_after(slice: &[SpringState], after: usize) -> &[SpringState] {
	&slice[slice.iter().skip(after).position(|s| *s != SpringState::Operational).unwrap_or(slice.len()-after)+after..]
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum SpringState {
	Unknown = b'?',
	Operational = b'.',
	Damaged = b'#',
}