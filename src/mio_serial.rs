use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;
use std::time::Duration;
use mio::{Evented, Poll, Token, Ready, PollOpt};
use mio::unix::EventedFd;
use serial::{self, SerialPort, SystemPort, BaudRate};

pub struct UnixSerial(SystemPort);

impl UnixSerial {
    pub fn new(dev: &str, speed: u32) -> io::Result<Self> {
        Ok(UnixSerial(get_port(dev, speed)?))
    }
}

fn get_port(dev: &str, speed: u32) -> io::Result<SystemPort> {
    let mut port = serial::open(dev)?;
    port.reconfigure(&|settings| {
            let baud = BaudRate::from_speed(speed as usize);
            settings.set_baud_rate(baud)?;
            Ok(())
        })?;
    port.set_timeout(Duration::from_secs(1))?;
    Ok(port)
}

impl Evented for UnixSerial {
    fn register(&self,
                poll: &Poll,
                token: Token,
                interest: Ready,
                opts: PollOpt)
                -> io::Result<()> {
        EventedFd(&self.0.as_raw_fd()).register(poll, token, interest, opts)
    }

    fn reregister(&self,
                  poll: &Poll,
                  token: Token,
                  interest: Ready,
                  opts: PollOpt)
                  -> io::Result<()> {
        EventedFd(&self.0.as_raw_fd()).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        EventedFd(&self.0.as_raw_fd()).deregister(poll)
    }
}

impl Read for UnixSerial {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl Write for UnixSerial {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}
