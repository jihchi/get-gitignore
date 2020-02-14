extern crate getopts;

use getopts::Options;
use std::env;

fn ls() -> Result<(), Box<dyn std::error::Error>> {
	print!("ls");
	Ok(())
}

fn help(program: &str, opts: Options) -> Result<(), Box<dyn std::error::Error>> {
	let brief = format!("Usage: {} [Options] [Project Type]", program);
	print!("{}", opts.usage(&brief));
	Ok(())
}

fn get(types: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
	print!("{:?}", types);
	Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut opts = Options::new();
	opts.optflag("l", "ls", "list out all of the available types");
	opts.optflag("h", "help", "print this help menu");

	let matches = opts.parse(&args[1..])?;

	if matches.opt_present("h") {
		help(&program, opts)
	} else if matches.opt_present("l") {
		ls()
	} else {
		let types = &matches.free;
		if types.len() > 0 {
			get(types)
		} else {
			help(&program, opts)
		}
	}
}
