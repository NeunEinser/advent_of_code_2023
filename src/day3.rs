use std::{process, fs, cmp, collections::HashMap};

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
	let mut nodes: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

	let mut sum = 0;
	for (i, line) in lines.iter().enumerate() {
		let mut j = line.find(|c: char| c.is_ascii_digit()).unwrap_or(line.len());
		while !line[j..].is_empty() {
			let num_end = line[j..].find(|c: char| !c.is_ascii_digit()).unwrap_or(line[j..].len()) + j;
			let num = line[j..num_end].parse::<u32>().expect("j should be a digit and num end should be the first non-digit, so this should always produce a valid u32");

			let mut counts = false;
			if i > 0 {
				let start = cmp::max(0, j as isize - 1) as usize;
				let end = cmp::min(lines[i-1].len(), num_end + 1);
				if let Some(j) = lines[i-1][start..end].find(|c: char| !c.is_ascii_digit() && c != '.') {
					counts = true;
					add_node(&mut nodes, &lines, i - 1, j + start, num);
				}
			}
				
			if j > 0 && !line.as_bytes()[j-1].is_ascii_digit() && line.as_bytes()[j-1] != b'.' {
				counts = true;
				add_node(&mut nodes, &lines, i, j - 1, num);
			}
			
			if num_end + 1 < line.len() && line.as_bytes()[num_end] != b'.' {
				counts = true;
				add_node(&mut nodes, &lines, i, num_end, num);
			}

			if i + 2 < lines.len() {
				let start = cmp::max(0, j as isize - 1) as usize;
				let end = cmp::min(lines[i+1].len(), num_end + 1);
				if let Some(j) = lines[i+1][start..end].find(|c: char| !c.is_ascii_digit() && c != '.') {
					counts = true;
					add_node(&mut nodes, &lines, i + 1, j + start, num);
				}
			}

			if counts {
				sum+=num;
			}

			j = num_end;
			j += line[j..].find(|c: char| c.is_ascii_digit()).unwrap_or(line[j..].len());
		}
	}

	let mut gear_sum = 0;
	for (_, v) in nodes {
		if v.len() == 2 {
			gear_sum += v[0] * v[1];
		}
	}

	println!("Sum of part numbers: {sum}");
	println!("Sum of gear ratios: {gear_sum}");
}

fn add_node(nodes: &mut HashMap<(usize, usize), Vec<u32>>, lines: &[&str], i: usize, j : usize, num: u32) {
	if lines[i].as_bytes()[j] != b'*' {
		return;
	}

	if !nodes.contains_key(&(i, j)) {
		nodes.insert((i, j), Vec::new());
	}
	let v = nodes.get_mut(&(i, j)).expect("Should be inserted the line before");
	v.push(num);
}