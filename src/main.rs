use minigrep::*;
use std::env;
use std::process;

fn main() {
	// Store all arguments from the env::args() iterator by converting it to a vector using collect()
	// It is required to supply type for collect(). In this case it is a String vector
	let args: Vec<String> = env::args().collect();

	// Store the search string and input file name
	let config = Config::new(&args).unwrap_or_else(|err|{
		eprintln!("Problem parsing args: {}", err);
		process::exit(1);
	});

	if let Err(e) = run(config){
		eprintln!("App Error: {}", e);
		process::exit(1);
	}
}