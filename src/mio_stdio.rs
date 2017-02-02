use std::io::{self, Read, Write, Stdin, Stdout};
use std::os::unix::io::RawFd;
use mio::{Evented, Poll, Token, Ready, PollOpt};
use mio::unix::EventedFd;
use termios::*;

const STDIN_FILENO: RawFd = 0;

pub struct UnixStdio {
    stdin: Stdin,
    stdout: Stdout,
    old_flags: Termios,
}

impl UnixStdio {
    pub fn new() -> io::Result<Self> {
        let flags = setup_fd(STDIN_FILENO)?;
        Ok(UnixStdio {
            stdin: io::stdin(),
            stdout: io::stdout(),
            old_flags: flags,
        })
    }
}

fn setup_fd(fd: RawFd) -> io::Result<Termios> {
    let old_flags = Termios::from_fd(fd)?;

    let mut new_flags = old_flags.clone();
    new_flags.c_cflag = B9600 | CS8 | CLOCAL | CREAD;
    new_flags.c_iflag = IGNPAR;
    new_flags.c_oflag = 0;
    new_flags.c_lflag = 0;
    new_flags.c_cc[VMIN] = 1;
    new_flags.c_cc[VTIME] = 0;
    tcflush(fd, TCIFLUSH)?;
    tcsetattr(fd, TCSANOW, &new_flags)?;

    Ok(old_flags)
}

impl Drop for UnixStdio {
    fn drop(&mut self) {
        tcsetattr(STDIN_FILENO, TCSANOW, &self.old_flags).ok();
    }
}

impl Evented for UnixStdio {
    fn register(&self,
                poll: &Poll,
                token: Token,
                interest: Ready,
                opts: PollOpt)
                -> io::Result<()> {
        EventedFd(&STDIN_FILENO).register(poll, token, interest, opts)
    }

    fn reregister(&self,
                  poll: &Poll,
                  token: Token,
                  interest: Ready,
                  opts: PollOpt)
                  -> io::Result<()> {
        EventedFd(&STDIN_FILENO).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        EventedFd(&STDIN_FILENO).deregister(poll)
    }
}

impl Read for UnixStdio {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stdin.read(buf)
    }
}

impl Write for UnixStdio {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}
