use std::{process, fs, cmp};

use crate::UnwrapOrExit;

/// https://adventofcode.com/2023/day/3
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path>", args[0], args[1]);

	if args.len() != 3 {
		eprintln!("{syntax}");
		process::exit(1);
	}

	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);
	let lines: Vec<&str> =  content.lines().collect();

	let mut sum = 0;
	for (i, line) in lines.iter().enumerate() {
		let mut j = line.find(|c: char| c.is_ascii_digit()).unwrap_or(line.len());
		while !line[j..].is_empty() {
			let num_end = line[j..].find(|c: char| !c.is_ascii_digit()).unwrap_or(line[j..].len()) + j;
			let num = line[j..num_end].parse::<u32>().unwrap();

			let mut counts = false;
			if i > 0 {
				if lines[i-1][cmp::max(0, j as isize - 1) as usize..cmp::min(lines[i-1].len(), num_end + 1)].contains(|c: char| !c.is_ascii_digit() && c != '.') {
					counts = true;
				}
			}
				
			if j > 0 && !line.as_bytes()[j-1].is_ascii_digit() && line.as_bytes()[j-1] != b'.' {
				counts = true
			}
			
			if num_end + 1 < line.len() && line.as_bytes()[num_end] != b'.' {
				println!("{}", line.as_bytes()[num_end] as char);
				counts = true
			}

			if i + 2 < lines.len() {
				if lines[i+1][cmp::max(0, j as isize - 1) as usize..cmp::min(lines[i+1].len(), num_end + 1)].contains(|c: char| !c.is_ascii_digit() && c != '.') {
					counts = true;
				}
			}

			if counts {
				sum+=num;
				//println!("{}", &line[j..num_end]);
			}

			j = num_end;
			j += line[j..].find(|c: char| c.is_ascii_digit()).unwrap_or(line[j..].len());
		}
	}

	println!("{sum}")
}