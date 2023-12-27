use std::{process, fs, collections::{HashMap, VecDeque}};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/20
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path>", args[0], args[1]);

	if args.len() < 3 || args.len() > 4 {
		eprintln!("{syntax}");
		process::exit(1);
	}

	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);

	let mut modules = HashMap::new();
	let mut unknown_children = HashMap::new();
	for line in content.lines() {
		let (input, output) = line.split_once(" -> ").unwrap_or_exit(&format!("Invalid module {line}"), 1);

		let (input, module) = if input.starts_with("%") {
			let module = Module::FlipFlop(FlipFlopModule::create(output.split(", ").collect()));
			(&input[1..],  module)
		} else if input.starts_with('&') {
			let module = Module::Conjunction(ConjunctionModule::create(unknown_children.remove(&input[1..]).unwrap_or_default(), output.split(", ").collect()));
			(&input[1..],  module)
		} else if input == "broadcaster" {
			let module = Module::Broadcast(BroadcastModule::create(output.split(", ").collect()));
			("broadcaster",  module)
		} else {
			eprintln!("Unknown module type for module {line}");
			process::exit(1);
		};

		for &output in module.outputs() {
			if let Some(module) = modules.get_mut(output) {
				if let Module::Conjunction(data) = module {
					data.add_input(input);
				}
			} else {
				let unknown = unknown_children.entry(output).or_default();
				unknown.push(input);
			}
		}

		modules.insert(input, module);
	}

	let mut high_count = 0;
	let mut low_count = 0;
	let mut receivers = VecDeque::new();

	for _ in 0..1000 {
		receivers.clear();
		low_count += 1;
		println!("button -low -> broadcaster");
		receivers.push_back(("broadcaster", "button", false));

		while let Some((module_name, source, signal)) = receivers.pop_front() {
			if let Some(module) = modules.get_mut(module_name) {
				if let Some((signal, recs)) = module.process_signal(source, signal) {
					for &receiver in recs {
						receivers.push_back((receiver, module_name, signal));
						println!("{module_name} {} -> {receiver}", if signal {"+high"} else {"-low"});
					}
					if signal {
						high_count += recs.len() as u64;
					} else {
						low_count += recs.len() as u64;
					}
				}
			}
		}

		println!("\n#####################\n")
	}

	println!("{} * {} = {}", low_count, high_count, low_count * high_count);
}

enum Module<'a, 'b> {
	Broadcast(BroadcastModule<'a>),
	FlipFlop(FlipFlopModule<'a>),
	Conjunction(ConjunctionModule<'a, 'b>),
}

#[derive(Debug, Clone)]
struct BroadcastModule<'a> {
	outputs: Vec<&'a str>,
}
impl<'a, 'b> Module<'a, 'b> {
    pub fn process_signal<'c, 'd>(&'c mut self, source: &'d str, signal: bool) -> Option<(bool, &'c [&'a str])> {
		match self {
			Module::Broadcast(data) => Some((signal, &data.outputs)),
			Module::FlipFlop(data) => {
				if !signal {
					data.state = !data.state;
					Some((data.state, &data.outputs))
				} else {
					None
				}
			},
			Module::Conjunction(data) => {
				let value = data.inputs.get_mut(source)?;
				*value = signal;
				if data.inputs.values().all(|v| *v) {
					Some((false, &data.outputs))
				} else {
					Some((true, &data.outputs))
				}
			},
		}
    }

	pub fn outputs<'c>(&'c self) -> &'c [&'a str] {
		match self {
			Module::Broadcast(data) => &data.outputs,
			Module::FlipFlop(data) => &data.outputs,
			Module::Conjunction(data) => &data.outputs,
		}
	}
}
impl<'a> BroadcastModule<'a> {
	pub fn new() -> Self {
		Self {
			outputs: Vec::new()
		}
	}
	pub fn create(outputs: Vec<&'a str>) -> Self {
		Self {
			outputs
		}
	}
}

#[derive(Debug, Clone)]
struct FlipFlopModule<'a> {
	state: bool,
	outputs: Vec<&'a str>,
}
impl<'a> FlipFlopModule<'a> {
	pub fn new() -> Self {
		Self {
			state: false,
			outputs: Vec::new(),
		}
	}
	pub fn create(outputs: Vec<&'a str>) -> Self {
		Self {
			outputs,
			state: false
		}
	}
}

#[derive(Debug, Clone)]
struct ConjunctionModule<'a, 'b> {
	inputs: HashMap<&'b str, bool>,
	outputs: Vec<&'a str>,
}
impl<'a, 'b> ConjunctionModule<'a, 'b> {
	pub fn new() -> Self {
		Self {
			inputs: HashMap::new(),
			outputs: Vec::new(),
		}
	}
	pub fn create(inputs: Vec<&'b str>, outputs: Vec<&'a str>) -> Self {
		Self {
			inputs: inputs.iter().map(|&i| (i, false)).collect(),
			outputs,
		}
	}
	pub fn add_input(&mut self, input: &'b str) {
		self.inputs.insert(input, false);
	}
}