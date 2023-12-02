use std::{env, process};

mod day1;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		eprintln!("Syntax: {} day<1-25>", args[0]);
		process::exit(1);
	}

	match &args[1][..] {
		"day1" => day1::main(args),
		cmd => {
			eprintln!("Syntax: {} day<1-25>", args[0]);
			eprintln!("Unknown command: {}", cmd);
			process::exit(1);
		}
	}
}