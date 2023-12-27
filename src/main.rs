use std::{env, process, fmt::Display};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		eprintln!("Syntax: {} day<1-25>", args[0]);
		process::exit(1);
	}

	match &args[1][..] {
		"day1" => day1::main(args),
		"day2" => day2::main(args),
		"day3" => day3::main(args),
		"day4" => day4::main(args),
		"day5" => day5::main(args),
		"day6" => day6::main(args),
		"day7" => day7::main(args),
		"day8" => day8::main(args),
		"day9" => day9::main(args),
		"day10" => day10::main(args),
		"day11" => day11::main(args),
		"day12" => day12::main(args),
		"day13" => day13::main(args),
		"day14" => day14::main(args),
		"day15" => day15::main(args),
		"day16" => day16::main(args),
		"day17" => day17::main(args),
		"day18" => day18::main(args),
		"day19" => day19::main(args),
		"day20" => day20::main(args),
		"day21" => day21::main(args),
		cmd => {
			eprintln!("Syntax: {} day<1-25>", args[0]);
			eprintln!("Unknown command: {}", cmd);
			process::exit(1);
		}
	}
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

impl<T, E: Display> UnwrapOrExit<T> for Result<T, E> {
	fn unwrap_or_exit(self, msg: &str, code: i32) -> T {
		self.unwrap_or_else(|err| {
			eprintln!("{msg}: {err}");
			process::exit(code);
		})
	}
}