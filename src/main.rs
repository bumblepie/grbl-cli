extern crate grbl_cli;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let config = grbl_cli::GrblConfig::new(&args);
    grbl_cli::run(config).expect("something went wrong");
}

