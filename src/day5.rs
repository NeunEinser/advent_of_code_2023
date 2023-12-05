use std::{process, fs, collections::HashMap, ops::Range, cmp};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/5
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path> [part1|part2]", args[0], args[1]);

	if args.len() < 3 && args.len() > 4 {
		eprintln!("{syntax}");
		process::exit(1);
	}

	let part = if args.len() == 4 {
		&args[3]
	} else {
		"part2"
	};

	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);
	
	match part {
		"part1" => part1(content.lines()),
		"part2" => part2(content.lines()),
		p => {
			eprintln!("Invalid argument, expected part1 or part2, found {p}");
			process::exit(1);
		}
	}
}

fn part1<'a, T>(lines: T)
	where T: Iterator<Item = &'a str> {
	let mut lines = lines;

	let mut seeds: HashMap<u64, HashMap<&str, u64>> = lines.next()
		.unwrap_or_exit("Received empty input file", 1)
		.split_once(':')
		.unwrap_or_exit("Could not parse seeds", 1)
		.1.split_whitespace()
		.map(|n| {
			let value = n.parse().unwrap_or_exit(&format!("Could not parse seeds (found invalid number {n})"), 1);
			
			(value, HashMap::from([("seed", value)]))
		})
		.collect();

	let mut maps = Vec::new();

	for line in lines {
		let line = line.trim();
		if line.is_empty() {
			continue;
		}

		if line.ends_with(" map:") {
			let parts = line
				.split_once(' ').unwrap_or_exit(&format!("Found invalid map head {line}"), 1)
				.0.split_once("-to-")
				.unwrap_or_exit(&format!("Found invalid map head {line}"), 1);

			maps.push(Map {
				source_name: parts.0,
				dest_name: parts.1,
				mappings: HashMap::new()
			})
		} else {
			let current_map = maps.last_mut().unwrap_or_exit(&format!("Found invalid map head {line}"), 1);
			let mut parts = line
				.splitn(3, ' ')
				.map(|n| n.parse::<u64>().unwrap_or_exit(&format!("Found invalid map content {line}"), 1));

			let dest = parts.next().unwrap_or_exit(&format!("Found invalid map content {line}"), 1);
			let src = parts.next().unwrap_or_exit(&format!("Found invalid map content {line}"), 1);
			let len = parts.next().unwrap_or_exit(&format!("Found invalid map content {line}"), 1);

			current_map.mappings.insert(src..src+len, dest);
		}
	}

	for (_, properties) in seeds.iter_mut() {
		while let Some(source) = maps
			.iter()
			.find(|m| properties.contains_key(&m.source_name) && !properties.contains_key(&m.dest_name)) {
			let val = properties[source.source_name];
			let mapped_val = source.mappings.iter()
				.find(|(key, _)| key.contains(&val))
				.map(|(range, v)| val - range.start + v)
				.unwrap_or(val);

			properties.insert(source.dest_name, mapped_val);
		}
	}

	let min_location = seeds
		.iter()
		.flat_map(|(_, m)| m.iter())
		.filter(|(key, _)| **key == "location" )
		.map(|(_, val)| *val)
		.min()
		.unwrap_or_default();

	println!("Seed data: {seeds:#?}");
	println!("Lowest location (Part 1): {min_location}");
}

fn part2<'a, T>(lines: T)
	where T: Iterator<Item = &'a str> {
	let mut lines = lines;

	let mut seeds_iter = lines.next()
		.unwrap_or_exit("Received empty input file", 1)
		.split_once(':')
		.unwrap_or_exit("Could not parse seeds", 1)
		.1.split_whitespace()
		.map(|n| n.parse::<u64>().unwrap_or_exit(&format!("Could not parse seeds (found invalid number {n})"), 1));

	let mut seeds = Vec::new();
	while let Some(start) = seeds_iter.next() {
		let len = seeds_iter.next().unwrap_or_exit("Could not parse seeds (uneven number of values; could not generate ranges)", 1);
		seeds.push(start..start+len)
	}

	let mut maps = Vec::new();

	for line in lines {
		let line = line.trim();
		if line.is_empty() {
			continue;
		}

		if line.ends_with(" map:") {
			let parts = line
				.split_once(' ').unwrap_or_exit(&format!("Found invalid map head {line}"), 1)
				.0.split_once("-to-")
				.unwrap_or_exit(&format!("Found invalid map head {line}"), 1);

			maps.push(Map {
				source_name: parts.0,
				dest_name: parts.1,
				mappings: HashMap::new()
			})
		} else {
			let current_map = maps.last_mut().unwrap_or_exit(&format!("Found invalid map head {line}"), 1);
			let mut parts = line
				.splitn(3, ' ')
				.map(|n| n.parse::<u64>().unwrap_or_exit(&format!("Found invalid map content {line}"), 1));

			let dest = parts.next().unwrap_or_exit(&format!("Found invalid map content {line}"), 1);
			let src = parts.next().unwrap_or_exit(&format!("Found invalid map content {line}"), 1);
			let len = parts.next().unwrap_or_exit(&format!("Found invalid map content {line}"), 1);

			current_map.mappings.insert(src..src+len, dest);
		}
	}

	let mut seed_props = HashMap::new();
	let mut min_loc_part2 = u64::MAX;
	for mut seed_range in seeds {
		while let Some(seed) = seed_range.next() {
			seed_props.clear();
			seed_props.insert("seed", seed);
			let mut can_skip = u64::MAX;
			while let Some(source) = maps.iter().find(|m| seed_props.contains_key(&m.source_name) && !seed_props.contains_key(&m.dest_name)) {
				let val = seed_props[source.source_name];
				let mapped_val = source.mappings.iter()
					.find(|(key, _)| key.contains(&val))
					.map(|(range, v)| (range.end - val, val - range.start + v))
					.unwrap_or_else(|| (
						source.mappings
							.keys()
							.map(|r| r.start)
							.filter(|s| *s > val)
							.min()
							.unwrap_or(u64::MAX),
						val));

				can_skip = cmp::min(mapped_val.0, can_skip);
				seed_props.insert(source.dest_name, mapped_val.1);
			}

			min_loc_part2 = cmp::min(min_loc_part2, seed_props.get("location").copied().unwrap_or(u64::MAX));

			seed_range = seed+can_skip..seed_range.end;
		}
	}

	println!("Lowest location (Part 2): {min_loc_part2}");
}

#[derive(Debug, Clone)]
struct Map<'a> {
	source_name: &'a str,
	dest_name: &'a str,
	mappings: HashMap<Range<u64>, u64>,
}