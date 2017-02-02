extern crate chrono;
extern crate clap;
extern crate mio;
extern crate serial;
extern crate termios;

mod error;
mod mio_serial;
mod mio_stdio;
mod session;

use clap::{App, Arg, ArgMatches};
use error::CustomError;
use session::{Session, SessionConfig};

fn parse_config<'a>(matches: &'a ArgMatches) -> Result<SessionConfig<'a>, CustomError> {
    let device = matches.value_of("device")
        .ok_or("Device name is empty")?;
    let speed: u32 = matches.value_of("speed")
        .ok_or("Speed is empty")?
        .parse()?;

    Ok(SessionConfig {
        device: device,
        speed: speed,
        timestamps: !matches.is_present("no_timestamps"),
    })
}

fn main() {
    let matches = App::new("rcom")
        .version("1.0")
        .author("Vadzim Dambrouski <pftbest@gmail.com>")
        .about("A communication program for accessing serial ports")
        .arg(Arg::with_name("device_name")
            .help("Serial port name")
            .short("d")
            .long("device")
            .default_value("/dev/ttyUSB1"))
        .arg(Arg::with_name("speed")
            .help("Communication speed")
            .short("s")
            .long("speed")
            .default_value("115200"))
        .arg(Arg::with_name("no_timestamps")
            .help("Don't show timestamps")
            .short("n")
            .long("no-timestamps"))
        .get_matches();

    let result = parse_config(&matches)
        .and_then(|config| Session::new(config))
        .and_then(|mut s| s.run());

    match result {
        Err(e) => {
            println!("{}", e);
            return;
        }
        _ => {}
    }
}
