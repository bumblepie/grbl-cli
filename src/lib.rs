extern crate grbl;
extern crate serial;

use grbl::GrblPort;
use std::time::Duration;
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

    let mut port = GrblPort::new(port);
    port.wakeup().expect("oh no");
    let line = port.read_line().expect("oh no");
    println!("{}", line);
    port.send_command(&String::from("$$")).expect("oh no");
    let lines = port.read_until_ok(2).expect("oh no");
    for line in lines.iter() {
        println!("{}", line);
    }
    Ok(())
}