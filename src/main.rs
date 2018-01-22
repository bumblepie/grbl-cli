extern crate serial;
extern crate grbl;
extern crate grbl_cli;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let config = grbl_cli::GrblConfig::new(&args);
    if let Err(err) = grbl_cli::run(config) {
    	match err {
    		grbl::Error::SerialError(err) => {
    			match err.kind() {
    				serial::ErrorKind::NoDevice => eprintln!("Device not found"),
    				_ => panic!(err),
    			}
    		},
    		_ => panic!(err),
    	}
    	std::process::exit(1)
    };
}

