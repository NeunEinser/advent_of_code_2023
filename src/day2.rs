use std::{process, fs};

use crate::UnwrapOrExit;

pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path> <max_red> <max_green> <max_blue>", args[0], args[1]);

	if args.len() < 6 {
		eprintln!("{syntax}");
		process::exit(1);
	}

	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);
	let max_red = args[3].parse::<u32>().unwrap_or_exit(&format!("{syntax}\nCould not read <max_red>, expected number, got {}", args[3]), 1);
	let max_green = args[4].parse::<u32>().unwrap_or_exit(&format!("{syntax}\nCould not read <max_green>, expected number, got {}", args[4]), 1);
	let max_blue = args[5].parse::<u32>().unwrap_or_exit(&format!("{syntax}\nCould not read <max_blue>, expected number, got {}", args[5]), 1);

	let mut result = 0;

	for line in content.lines() {
		let line = line.trim();
		let err = format!("{line} does not contain a valid game");
		if !line.starts_with("Game ") {
			eprintln!("{err}");
			process::exit(1);
		}

		let line = &line[5..];
		let game_id_length = line.find(":").unwrap_or_exit(&err, 1);
		let game_id = String::from(&line[..game_id_length]).parse::<u32>().unwrap_or_exit(&err, 1);

		if parse_line(line[game_id_length + 1..].trim(), max_red, max_green, max_blue, &err) {
			println!("Game {game_id} is possible!");
			result += game_id;
		} else {
			println!("Game {game_id} is impossible!");
		}
	}

	println!("The sum of possible game IDs is {result}");
}

fn parse_line(line: &str, max_red: u32, max_green: u32, max_blue: u32, err: &str) -> bool {
	let mut line = line;

	while !line.is_empty() {
		let subset_len = line.find(";").unwrap_or(line.len());
		let mut subset = line[..subset_len].trim();

		while !subset.is_empty() {
			let len = subset.find(",").unwrap_or(subset.len());
			let item = subset[..len].trim();

			let num_length = item.find(|c: char| !c.is_ascii_digit()).unwrap_or_exit(&err, 1);
			let num = String::from(&item[..num_length]).parse::<u32>().unwrap_or_exit(&err, 1);
		
			let item = item[num_length + 1..].trim();

			let impossible_amount = match item {
				"red" => num > max_red,
				"green" => num > max_green,
				"blue" => num > max_blue,
				_ => {
					eprintln!("{err}");
					process::exit(1);
				}
			};

			if impossible_amount {
				return false;
			}

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

	true
}