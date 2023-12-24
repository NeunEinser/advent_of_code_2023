use std::{process, fs, collections::HashMap, ops::RangeInclusive};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/16
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path>", args[0], args[1]);

	if args.len() < 3 || args.len() > 4 {
		eprintln!("{syntax}");
		process::exit(1);
	}

	let mut workflows = HashMap::new();

	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);
	let content = content.split_once("\n\n").unwrap_or_exit("Could not split instructions and parts", 1);

	for line in content.0.lines() {
		let (name, raw_rules) = line.split_once('{').unwrap_or_exit(&format!("Could not get workflow name of workflow {line}"), 1);
		let rules: Vec<_> = raw_rules[..raw_rules.len() - 1].split(',')
			.map(|rule| {
				if let Some((predicate, target)) = rule.split_once(':') {
					let variable = predicate.chars().nth(0).unwrap_or_exit(&format!("Could not read condition {predicate} of workflow {line}"), 1);
					let predicate = &predicate[predicate.char_indices().nth(1).unwrap().0..];
	
					let condition = match predicate.as_bytes()[0] {
						b'>' => Condition::GT,
						b'<' => Condition::LT,
						b => {
							eprintln!("Invalid condition operation {} in workflow {}", b as char, line);
							process::exit(1);
						}
					};
	
					let compare_val = predicate[1..].parse().unwrap_or_exit(&format!("Could not read compare number of workflow {line}"), 1);

					let target = match target {
						"A" => Target::Accepted,
						"R" => Target::Rejected,
						t => Target::Workflow(t)
					};
	
					Rule::Conditional{ variable, condition, compare_val, target }
				} else {
					let target = match rule {
						"A" => Target::Accepted,
						"R" => Target::Rejected,
						t => Target::Workflow(t)
					};
					Rule::Always(target)
				}
			})
			.collect();

		workflows.insert(name, rules);
	}

	let mut sum = 0; 
	for part in content.1.lines() {
		let variables: Vec<_> = part.get(1..part.len() - 1).unwrap_or_exit(&format!("Could not read part {part}"), 1)
			.split(',')
			.map(|kvp| {
				(
					kvp.chars().nth(0).unwrap_or_exit(&format!("Could not read kvp {kvp} of part {part}"), 1),
					kvp.get(kvp.char_indices().nth(2).unwrap_or_exit(&format!("Could not read kvp {kvp} of part {part}"), 1).0..)
						.unwrap_or_exit(&format!("Could not read value of kvp {kvp} from part {part}"), 1)
						.parse()
						.unwrap_or_exit(&format!("Could not read value of kvp {kvp} from part {part}"), 1)
				)
			})
			.collect();

		let mut current_workflow = "in";
		let accepted = loop {
			let workflow = workflows.get(current_workflow).unwrap_or_exit(&format!("Could not find workflow {current_workflow}"), 1);
			let target = workflow.iter().filter_map(|r| r.get_matching(&variables)).nth(0).unwrap_or_exit(&format!("Workflow {current_workflow} does not have a matching target for part {part}"), 1);

			match target {
				Target::Accepted => break true,
				Target::Rejected => break false,
				Target::Workflow(workflow) => current_workflow = workflow
			}
		};

		if accepted {
			sum += variables.iter().map(|&(_, v)| v).sum::<u64>();
		}
	}

	const MAX_RANGE: RangeInclusive<u64> = 1..=4000;
	let ranges = [('x', MAX_RANGE), ('m', MAX_RANGE), ('a', MAX_RANGE), ('s', MAX_RANGE)];
	println!("Possibilities:");
	let possibilities = get_possibilities(&ranges[..], &workflows, "in").unwrap_or_exit("Failed to traverse all possibilities", 1);


	println!("Part1: {sum}");
	println!("Part2: {possibilities}");
}

fn get_possibilities(ranges: &[(char, RangeInclusive<u64>)], workflows: &HashMap<&str, Vec<Rule<'_>>>, current_workflow: &str) -> Result<u64, String> {
	let workflow = workflows.get(current_workflow).ok_or(format!("Workflow {current_workflow} does not exist"))?;
	let mut possibilities = 0;
	let mut ranges = ranges.to_vec();
	for rule in workflow {
		let (successful, unsuccessful, target) = rule.get_with_reduced_range(&ranges);
		possibilities += match target {
			Target::Workflow(wf) => get_possibilities(&successful, workflows, wf)?,
			Target::Accepted => {
				let product = successful.iter().map(|(_, r)| r.clone().count() as u64).product();
				println!("{current_workflow}: {product}; {successful:?}");
				product
			},
			Target::Rejected => 0
		};
		ranges = unsuccessful;
	}

	Ok(possibilities)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Rule<'a> {
	Always(Target<'a>),
	Conditional{ variable: char, condition: Condition, compare_val: u64, target: Target<'a> }
}
impl<'a> Rule<'a> {
	pub fn get_matching(&self, variables: &[(char, u64)]) -> Option<Target> {
		match self {
			Rule::Always(t) => Some(*t),
			Rule::Conditional { variable, condition, compare_val, target } => {
				let &(_, v) = variables.iter().find(|&&(v, _)| v == *variable)?;
				match condition {
					Condition::GT => if v > *compare_val { Some(*target) } else { None },
					Condition::LT => if v < *compare_val { Some(*target) } else { None }
				}
			}
		}
	}

	pub fn get_with_reduced_range(&self, ranges: &[(char, RangeInclusive<u64>)]) -> (Vec<(char, RangeInclusive<u64>)>, Vec<(char, RangeInclusive<u64>)>, Target) {
		match self {
			Rule::Always(target) => (ranges.to_vec(), Vec::new(), *target),
			Rule::Conditional { variable, condition, compare_val, target } => {
				let successful: Vec<_> = ranges.iter()
					.map(|(v, r)| {
						if v != variable {
							(*v, r.clone())
						} else {
							let ranges = match condition {
								Condition::GT => *compare_val+1..=*r.end(),
								Condition::LT => *r.start()..=*compare_val-1,
							};
							(*v, ranges)
						}
					})
					.collect();
				let unsuccessful: Vec<_> = ranges.iter()
					.map(|(v, r)| {
						if v != variable {
							(*v, r.clone())
						} else {
							let ranges = match condition {
								Condition::GT => *r.start()..=*compare_val,
								Condition::LT => *compare_val..=*r.end(),
							};
							(*v, ranges)
						}
					})
					.collect();

				(successful, unsuccessful, *target)
			}
		}
	}
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Condition {
	GT = b'>',
	LT = b'<'
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Target<'a> {
	Accepted,
	Rejected,
	Workflow(&'a str)
}