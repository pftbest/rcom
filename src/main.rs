extern crate chrono;
extern crate clap;
extern crate mio;
extern crate serial;
extern crate termios;

mod error;
mod mio_serial;
mod mio_stdio;
mod session;

use session::{Session, SessionConfig};

fn main() {
    let matches = clap::App::new("rcom").get_matches();

    let conf = SessionConfig {
        device: "/dev/ttyUSB1",
        speed: 115200,
    };

    Session::new(&conf)
        .and_then(|mut s| {
            println!("Hello!");
            s.run()
        })
        .or_else(|e| {
            println!("{}", e);
            Err(e)
        })
        .ok();
}
