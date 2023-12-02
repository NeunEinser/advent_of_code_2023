use std::{process, fs};

pub fn main(args: Vec<String>) {
	if args.len() < 3 {
		eprintln!("Syntax: {} {} <file path>", args[0], args[1]);
		process::exit(1);
	}

	let content = fs::read_to_string(&args[2]).unwrap_or_else(|err| {
		eprintln!("Could not read file content as Utf-8 string: {}", err);
		process::exit(1);
	});

	let mut sum = 0;
	for line in content.lines() {
		let first = get_first_digit(line,  line.char_indices().map(|t| t.0)).unwrap_or_else(|| {
			eprintln!("Line {line} does not contain a digit!");
			process::exit(1);
		});
		let last = get_first_digit(line,  line.char_indices().map(|t| t.0).rev())
			.expect("If this fails, the previous find should have failed already");

		sum += (first * 10 + last) as u32;
	}

	println!("The sum of all calibration values is {sum}")
}

fn get_first_digit<T>(input: &str, index_iter: T) -> Option<u8>
	where T: Iterator<Item = usize> {
	for offset in index_iter {
		if let Some(digit) = get_digit(&input[offset..]) {
			return Some(digit)
		}
	}
	None
}

fn get_digit(input: &str) -> Option<u8> {
	if input.is_empty() {
		return None;
	}

	let first_byte = input.as_bytes()[0];
	match first_byte {
		b'0'..=b'9' => Some(first_byte - b'0'),
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

fn get_digit_from_word(input: &str, map: &[(&str, u8)]) -> Option<u8> {
	for (word, digit) in map {
		if input.starts_with(word) {
			return Some(*digit);
		}
	}

	None
}

trait UnwrapOrExit<T> {
	fn unwrap_or_exit(self, msg: &str, code: i32) -> T;
}

impl<T> UnwrapOrExit<T> for Option<T> {
	fn unwrap_or_exit(self, msg: &str, code: i32) -> T {
		self.unwrap_or_else(|| {
			eprintln!("{msg}");
			process::exit(code);
		})
	}
}
