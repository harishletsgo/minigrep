use std::env;
use std::fs;

struct Config{
	query: String,
	filename: String,
}

fn parse_config(args: &[String]) -> Config {
	let query = args[1].clone();
	let filename = args[2].clone();

	Config{query, filename}
}

fn main() {
	// Store all arguments from the env::args() iterator by converting it to a vector using collect()
	// It is required to supply type for collect(). In this case it is a String vector
	let args: Vec<String> = env::args().collect();

	// Store the search string and input file name
	let config = parse_config(&args);

	println!("Searching for {}", config.query);
	println!("In file {}", config.filename);

	let contents = fs::read_to_string(config.filename).expect("Something wrong with file");

	println!("With text:\n{}", contents);
}