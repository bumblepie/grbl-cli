
extern crate serial;
extern crate grbl;
extern crate grbl_cli;

use std::time::Duration;
use grbl_cli::GrblConfig;

#[macro_use]
extern crate clap;
use clap::App;


const DEFAULT_TIMEOUT: &'static str = "5000";

fn main() {
	let yaml = load_yaml!("grbl-cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let port = matches.value_of("PORT").unwrap().to_string();
    let timeout = matches.value_of("timeout").unwrap_or(DEFAULT_TIMEOUT);
    let timeout = match timeout.parse::<u64>() {
    	Ok(val) => Duration::from_millis(val),
    	Err(_) => {
    		eprintln!("Could not parse timeout \"{}\"", timeout);
    		std::process::exit(1);
    	}
    };

	let config = GrblConfig {
		port,
		timeout,
	};

    if let Err(err) = grbl_cli::run(config) {
    	match err {
    		grbl::Error::SerialError(err) => {
    			match err.kind() {
    				serial::ErrorKind::NoDevice => eprintln!("Device not found"),
    				_ => {
    					eprintln!("{:?}", err);
    					panic!(err)
    				},
    			}
    		},
    		_ => {
    			eprintln!("{:?}", err);
    			panic!(err);
    		},
    	}
    	std::process::exit(1)
    };
}

