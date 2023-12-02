use std::{fs, env, process};

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		eprintln!("Syntax: {} <file path>", args[0]);
		process::exit(1);
	}

	let content = fs::read_to_string(&args[1]).unwrap_or_else(|err| {
		eprintln!("Could not read file content as Utf-8 string: {}", err);
		process::exit(1);
	});

	let mut sum = 0;
	for line in content.lines() {
		let first = line.find(|c| char::is_ascii_digit(&c)).unwrap_or_exit(&format!( "Line {} does not contain a digit", line), 1);
		let last = line.rfind(|c| char::is_digit(c, 10)).expect("If rfind fails, the previous find should have failed already");

		let line = line.as_bytes();
		let first = line[first] - b'0';
		let last= line[last] - b'0';

		sum += (first * 10 + last) as u32;
	}

	println!("The sum of all calibration values is {sum}")
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
