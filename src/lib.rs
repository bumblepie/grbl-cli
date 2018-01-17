extern crate grbl;
extern crate serial;

use std::time::Duration;
use std::io;
use std::io::prelude::*;

use grbl::GrblPort;
use serial::prelude::*;

pub struct GrblConfig {
    port: String,
}

impl GrblConfig {
    pub fn new(args: &[String]) -> GrblConfig {
        let port = args[1].clone();

        GrblConfig {
            port
        }
    }
}

pub fn run(config: GrblConfig) -> Result<(), serial::Error> {
    let mut port = serial::open(&config.port).unwrap();
    port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(serial::Baud115200));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }).expect("Could not reconfigure port");
    port.set_timeout(Duration::from_millis(5000))
        .expect("setting timeout failed");

    println!("Enter GRBL commands to execute them and view GRBL's output");
    println!("Enter \"exit\" to exit the program");

    let mut port = GrblPort::new(port);
    port.wakeup().expect("oh no");

    //read 2 lines of input given by wakeup
    let line = port.read_line().expect("oh no");
    println!("{}", line);
    let line = port.read_line().expect("oh no");
    println!("{}", line);
    
    loop {
        //Add prompt to separate cmds from output
        print!(">>");
        io::stdout().flush().expect("oh no");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("oh no");

        println!("{}", input);
        if input.trim().to_lowercase() == "exit" {
            break;
        }

        port.send_command(&input).expect("oh no");
        let lines = port.read_until_ok(2).expect("oh no");
        for line in lines.iter() {
            println!("{}", line);
        }
    }

    Ok(())
}