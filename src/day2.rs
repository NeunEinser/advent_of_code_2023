use std::{process, fs, cmp};

use crate::UnwrapOrExit;

struct CubeCounts {
	red: u32,
	green: u32,
	blue: u32
}

/// https://adventofcode.com/2023/day/2
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path> [<max_red> <max_green> <max_blue>]", args[0], args[1]);

	if args.len() != 3 && args.len() != 6 {
		eprintln!("{syntax}");
		process::exit(1);
	}

	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);
	let max = if args.len() == 3 {
		[0u32; 3]
	} else {
		[ 
			args[3].parse().unwrap_or_exit(&format!("{syntax}\nCould not read <max_red>, expected number, got {}", args[3]), 1),
			args[4].parse().unwrap_or_exit(&format!("{syntax}\nCould not read <max_green>, expected number, got {}", args[4]), 1),
			args[5].parse().unwrap_or_exit(&format!("{syntax}\nCould not read <max_blue>, expected number, got {}", args[5]), 1)
		]
	};

	let mut possible_sum = 0;
	let mut power_sum = 0;

	for line in content.lines() {
		let line = line.trim();
		let err = format!("{line} does not contain a valid game");
		if !line.starts_with("Game ") {
			eprintln!("{err}");
			process::exit(1);
		}

		let line = &line[5..];
		let game_id_length = line.find(':').unwrap_or_exit(&err, 1);
		let game_id = String::from(&line[..game_id_length]).parse::<u32>().unwrap_or_exit(&err, 1);

		let counts = get_min_cubes(line[game_id_length + 1..].trim()).unwrap_or_exit(&err, 1);

		power_sum += counts.red * counts.green * counts.blue;
		if counts.red <= max[0] && counts.green <= max[1] && counts.blue <= max[2] {
			possible_sum += game_id;
		}
	}

	if args.len() == 6 {
		println!("The sum of possible game IDs is {possible_sum}");
	}
	println!("The sum of powers is {power_sum}");
}

fn get_min_cubes(line: &str) -> Result<CubeCounts, String> {
	let mut line = line;
	let mut counts = CubeCounts {
		red: 0,
		green: 0,
		blue: 0
	};

	while !line.is_empty() {
		let subset_len = line.find(';').unwrap_or(line.len());
		let mut subset = line[..subset_len].trim();

		while !subset.is_empty() {
			let len = subset.find(',').unwrap_or(subset.len());
			let item = subset[..len].trim();

			let num_length = item.find(|c: char| !c.is_ascii_digit()).ok_or(String::from("invalid cube count"))?;
			let num = String::from(&item[..num_length]).parse::<u32>().map_err(|err| format!("Invalid cube count: {err}"))?;
		
			let item = item[num_length + 1..].trim();

			match item {
				"red" => counts.red = cmp::max(counts.red, num),
				"green" => counts.green = cmp::max(counts.green, num),
				"blue" => counts.blue = cmp::max(counts.blue, num),
				_ => {
					return Err(format!("invalid color {item}"));
				}
			};

			if len >= subset.len() {
				break;
			}
			subset = subset[len+1..].trim();
		}

		if subset_len >= line.len() {
			break;
		}
		line = line[subset_len+1..].trim();
	}

	Ok(counts)
}