extern crate serial;

use std::fmt;
use std::str;
use std::collections::VecDeque;

use std::str::Utf8Error as Utf8Error;
use std::io::Error as IOError;
use std::error::Error as StdError;

use serial::prelude::*;

pub struct GrblPort<P: SerialPort> {
	port: P,
	output_buffer: VecDeque<u8>,
}

impl<P: SerialPort> GrblPort<P> {
    pub fn new(port: P) -> Self {
        GrblPort {
        	port: port,
        	output_buffer: VecDeque::new(),
        }
    }

    pub fn wakeup(&mut self) -> Result<(), Error> {
    	self.write_command(&String::from("\r\n\r\n"))
    }

    fn write_command(&mut self, command: &String) -> Result<(), Error> {
    	let command = command.as_bytes();
    	self.port.write(&command[..])?;
    	self.port.flush()?;
    	Ok(())
    }

    pub fn send_command(&mut self, command: &str) -> Result<(), Error> {
        //split commands by newlines
        let lines = command.lines()
            .filter(|cmd| !cmd.is_empty())
            .map(|cmd| format!("{}\r\n", cmd));
        for cmd in lines {
            self.write_command(&cmd)?;
        }
        Ok(())
    }

    pub fn read_line(&mut self) -> Result<String, Error> {

    	let mut result: Vec<u8> = Vec::new();
    	let mut byte_buffer: [u8; 8] = [0; 8];
    	let mut line_complete = false;
    	while !line_complete {
    		while self.output_buffer.is_empty() {
    			//read input
    			let bytes_read = self.port.read(&mut byte_buffer)?;
    			let buffer_slice = &byte_buffer[..bytes_read];
    			for &byte in buffer_slice {
    				self.output_buffer.push_back(byte);
    			}
    		}
    		while !self.output_buffer.is_empty() && !line_complete {
    			let next_byte = self.output_buffer.pop_front().unwrap();
				if next_byte == b'\r' {
					line_complete = true;
				} else {
					result.push(next_byte);
				}
    		}
    	}

    	let result = str::from_utf8(&result)?;
    	Ok(result.trim().to_string())
    }

    pub fn read_until_ok(&mut self, max_oks: u8) -> Result<GrblResult, Error> {
    	let mut output = Vec::new();
    	let mut num_oks = 0;
        let mut succeeded = true;

    	while num_oks < max_oks {
    		let line = self.read_line()?; 
    		if line == "ok" || line.starts_with("error") {
    			num_oks += 1;
    		}
            if line.starts_with("error") {
                succeeded = false;
            }
            output.push(line);
    	}
    	Ok(GrblResult {
            output,
            succeeded
        })
    }
}

pub struct GrblResult {
    pub output: Vec<String>,
    pub succeeded: bool,
}

#[derive(Debug)]
pub enum Error {
    SerialError(serial::Error),
    IOError(IOError),
    Utf8Error(Utf8Error),
}

impl From<serial::Error> for Error {
    fn from(serial_error: serial::Error) -> Error {
        Error::SerialError(serial_error)
    }
}

impl From<IOError> for Error {
    fn from(io_error: IOError) -> Error {
        Error::IOError(io_error)
    }
}

impl From<Utf8Error> for Error {
    fn from(utf8_error: Utf8Error) -> Error {
        Error::Utf8Error(utf8_error)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}