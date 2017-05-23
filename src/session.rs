use error::CustomError;
use mio_serial::UnixSerial;
use mio_stdio::UnixStdio;
use mio::{Events, Poll, PollOpt, Ready, Token};
use std::io::{Read, Write};
use std::time::Duration;
use time;

pub struct SessionConfig<'a> {
    pub device: &'a str,
    pub speed: u32,
    pub timestamps: bool,
}

pub struct Session<'a> {
    config: SessionConfig<'a>,
    stdio: UnixStdio,
    serial: UnixSerial,
    poll: Poll,
    buffer: [u8; 1024],
    //history: Vec<u8>,
    line_flag: bool,
}

const STDIO_TOKEN: Token = Token(0);
const SERIAL_TOKEN: Token = Token(1);

enum Action {
    Nothing,
    Exit,
}

impl<'a> Session<'a> {
    pub fn new(config: SessionConfig<'a>) -> Result<Self, CustomError> {
        let session = Session {
            stdio: UnixStdio::new()?,
            serial: UnixSerial::new(&config.device[..], config.speed)?,
            poll: Poll::new()?,
            buffer: [0; 1024],
            //history: Vec::new(),
            line_flag: true,
            config: config,
        };
        session.poll
            .register(&session.stdio,
                      STDIO_TOKEN,
                      Ready::readable(),
                      PollOpt::level())?;
        session.poll
            .register(&session.serial,
                      SERIAL_TOKEN,
                      Ready::readable(),
                      PollOpt::level())?;
        Ok(session)
    }

    pub fn run(&mut self) -> Result<(), CustomError> {
        let mut events = Events::with_capacity(1024);
        loop {
            self.poll.poll(&mut events, Some(Duration::from_secs(1)))?;
            for event in events.iter() {
                match event.token() {
                    STDIO_TOKEN => {
                        let action = self.process_control()?;
                        match action {
                            Action::Nothing => {}
                            Action::Exit => return Ok(()),
                        }
                    }
                    SERIAL_TOKEN => {
                        let timestamp = time::now();
                        self.process_data(timestamp)?;
                    }
                    _ => unreachable!(),
                };
            }
        }
    }

    fn process_control(&mut self) -> Result<Action, CustomError> {
        let bytes = self.stdio.read(&mut self.buffer)?;
        for &x in &self.buffer[..bytes] {
            if x == 0x01 {
                return Ok(Action::Exit);
            }
            self.serial.write(&[x])?;
        }
        self.serial.flush()?;
        Ok(Action::Nothing)
    }

    fn process_data(&mut self, ts: time::Tm) -> Result<(), CustomError> {
        let bytes = self.serial.read(&mut self.buffer)?;
        for &x in &self.buffer[..bytes] {
            if self.config.timestamps && self.line_flag {
                self.line_flag = false;
                write!(self.stdio,
                       "\x1b[90m[{:02}:{:02}:{:02}.{:04}]\x1b[0m ",
                       ts.tm_hour,
                       ts.tm_min,
                       ts.tm_sec,
                       ts.tm_nsec / 100000)?;
            }
            if x == 0x0A {
                self.line_flag = true;
            }
            self.stdio.write(&[x])?;
        }
        self.stdio.flush()?;
        Ok(())
    }
}
