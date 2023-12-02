use std::{process, fs};

use crate::UnwrapOrExit;

pub fn main(args: Vec<String>) {
	let syntax = format!("Syntax: {} {} <file path> [<ignore-words (0 or 1; default 0)>]", args[0], args[1]);

	if args.len() < 3 || args.len() > 5 {
		eprintln!("{syntax}");
		process::exit(1);
	}

	let ignore_words = if args.len() == 3 {
		false
	} else {
		let val = args[3].parse::<u8>().unwrap_or_exit(&format!("{syntax}\nCould not read ignore-words parameter, expected 0 or 1, got {}", args[3]), 1);

		if val >= 2 {
			eprintln!("{syntax}\nCould not read ignore-words parameter, expected 0 or 1, got {}", args[3]);
			process::exit(1);
		}
		val == 1
	};

	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);

	let mut sum = 0;
	for line in content.lines() {
		let first = get_first_digit(line,  line.char_indices().map(|t| t.0), ignore_words)
			.unwrap_or_exit( &format!("Line {line} does not contain a digit!"), 1);
		let last = get_first_digit(line,  line.char_indices().map(|t| t.0).rev(), ignore_words)
			.expect("If this fails, the previous find should have failed already");

		sum += (first * 10 + last) as u32;
	}

	println!("The sum of all calibration values is {sum}")
}

fn get_first_digit<T>(input: &str, index_iter: T, ignore_words: bool) -> Option<u8>
	where T: Iterator<Item = usize> {
	for offset in index_iter {
		if let Some(digit) = get_digit(&input[offset..], ignore_words) {
			return Some(digit)
		}
	}
	None
}

fn get_digit(input: &str, ignore_words: bool) -> Option<u8> {
	if input.is_empty() {
		return None;
	}

	let first_byte = input.as_bytes()[0];
	match first_byte {
		b'0'..=b'9' => Some(first_byte - b'0'),
		first_byte => {
			if ignore_words {
				None
			} else {
				match first_byte {
					b'z' | b'Z' => get_digit_from_word(input, &[("zero", 0)]),
					b'o' | b'O' => get_digit_from_word(input, &[("one", 1)]),
					b't' | b'T' => get_digit_from_word(input, &[("two", 2), ("three", 3)]),
					b'f' | b'F' => get_digit_from_word(input, &[("four", 4), ("five", 5)]),
					b's' | b'S' => get_digit_from_word(input, &[("six", 6), ("seven", 7)]),
					b'e' | b'E' => get_digit_from_word(input, &[("eight", 8)]),
					b'n' | b'N' => get_digit_from_word(input, &[("nine", 9)]),
					_ => None
				}
			}
		}
	}
}

fn get_digit_from_word(input: &str, map: &[(&str, u8)]) -> Option<u8> {
	for (word, digit) in map {
		if input.starts_with(word) {
			return Some(*digit);
		}
	}

	None
}
