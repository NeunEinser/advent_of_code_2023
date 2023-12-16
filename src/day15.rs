use std::{process, fs, mem::MaybeUninit};

use crate::UnwrapOrExit;
/// https://adventofcode.com/2023/day/15
pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path>", args[0], args[1]);

	if args.len() < 3 || args.len() > 4 {
		eprintln!("{syntax}");
		process::exit(1);
	}

	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);
	let sum = content.split(',')
		.map(|i| i
			.bytes()
			.filter(|c| *c != b'\r' && *c != b'\n')
			.fold(0u8, |hash, c| hash.wrapping_add(c).wrapping_mul(17)))
		.fold(0, |sum, h| sum + h as u32);

	println!("Part 1: {sum}");

	let mut hash = HashMap::new();
	for instr in content.split(',') {
		let instr = instr.trim();
		if let Some(key) = instr.strip_suffix('-') {
			hash.remove(key);
		} else {
			let (key, val) = instr.split_once('=').unwrap_or_exit(&format!("Invalid instruction {instr}"), 1);
			let val = val.parse().unwrap_or_exit(&format!("Invalid value in instruction {instr}"), 1);
			hash.insert(key, val);
		}
	}

	let mut sum = 0;
	for (i, bucket) in hash.get_raw().iter().enumerate() {
		for (j, (_, value)) in bucket.iter().copied().enumerate() {
			sum += (i + 1) * (j + 1) * value;
		}
	}

	println!("Part 2: {sum}");
}

struct HashMap<'a> {
	buckets: [Vec<(&'a str, usize)>; 256]
}

impl<'a> HashMap<'a> {
	pub fn new() -> Self {
		let mut buckets: [MaybeUninit<Vec<(&'a str, usize)>>; 256] = unsafe { MaybeUninit::uninit().assume_init() };
		for b in buckets.iter_mut() {
			b.write(Vec::new());
		}

		Self {
			buckets: unsafe { buckets.map(|b| b.assume_init()) }
		}
	}

	pub fn insert(&mut self, key: &'a str, val: usize) {
		let bucket = &mut self.buckets[Self::get_hash(key) as usize];

		if let Some(i) = bucket.iter().position(|(k, _)| *k == key) {
			bucket[i] = (key, val);
		} else {
			bucket.push((key, val));
		}
	}

	pub fn remove(&mut self, key: &'a str) -> Option<usize> {
		let bucket = &mut self.buckets[Self::get_hash(key) as usize];

		let i = bucket.iter().position(|(k, _)| *k == key)?;
		Some(bucket.remove(i).1)
	}

	pub fn get_raw(&self) -> &[Vec<(&'a str, usize)>; 256] {
		&self.buckets
	}

	fn get_hash(val: &'a str) -> u8 {
		val.bytes()
		.filter(|c| *c != b'\r' && *c != b'\n')
		.fold(0u8, |hash, c| hash.wrapping_add(c).wrapping_mul(17))
	}
}