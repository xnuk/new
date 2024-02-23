use std::{env, io};

fn main() -> anyhow::Result<()> {
	let args: Vec<_> = env::args_os().skip(1).collect();
	let stdin = io::stdin();

	for line in stdin.lines() {
		let Ok(line) = line else { break };

		println!("[DEBUG] {args:?} {line}");
	}

	println!("Get fucked!");
	Ok(())
}
