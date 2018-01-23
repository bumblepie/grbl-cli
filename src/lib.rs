extern crate grbl;
extern crate serial;

use std::time::Duration;
use std::io;
use std::io::prelude::*;

use grbl::GrblPort;
use grbl::Error as GrblError;
use serial::prelude::*;

pub struct GrblConfig {
    pub port: String,
    pub timeout: Duration,
}

const CMD_SUCCEEDED_PROMPT: &'static str = ">>";
const CMD_FAILED_PROMPT: &'static str = "!>>";

pub fn run(config: GrblConfig) -> Result<(), GrblError> {
    let mut port = serial::open(&config.port)?;
    port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(serial::Baud115200));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;
    port.set_timeout(config.timeout)?;

    println!("Enter GRBL commands to execute them and view GRBL's output");
    println!("Enter \"exit\" to exit the program");

    let mut port = GrblPort::new(port);
    port.wakeup()?;

    //read 2 lines of input given by wakeup
    let line = port.read_line()?;
    println!("{}", line);
    let line = port.read_line()?;
    println!("{}", line);
    
    let mut previous_command_succeeded = true;

    loop {
        //Add prompt to separate cmds from output
        if previous_command_succeeded {
            print!("{}", CMD_SUCCEEDED_PROMPT);
        } else {
            print!("{}", CMD_FAILED_PROMPT);
        }
        
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "exit" {
            break;
        }

        port.send_command(&input)?;
        let result = port.read_until_ok(2)?;
        previous_command_succeeded = result.succeeded;
        for line in result.output.iter() {
            println!("{}", line);
        }
    }

    Ok(())
}