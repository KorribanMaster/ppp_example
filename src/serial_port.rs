use async_io::IoSafe;
use nix::errno::Errno;
use nix::fcntl::OFlag;
use nix::sys::termios;
use std::io;
use std::os::fd::AsFd;
use std::os::fd::AsRawFd;
use std::os::fd::BorrowedFd;
use std::os::fd::RawFd;
use std::sync::{Mutex, MutexGuard};

pub struct SerialPort {
    fd: Mutex<RawFd>,
}

impl SerialPort {
    pub fn new<P: ?Sized + nix::NixPath>(
        path: &P,
        baudrate: termios::BaudRate,
    ) -> io::Result<Self> {
        let fd: RawFd = nix::fcntl::open(
            path,
            OFlag::O_RDWR | OFlag::O_NOCTTY | OFlag::O_NONBLOCK,
            nix::sys::stat::Mode::empty(),
        )
        .map_err(to_io_error)?;

        let mut cfg = termios::tcgetattr(fd).map_err(to_io_error)?;
        cfg.input_flags = termios::InputFlags::empty();
        cfg.output_flags = termios::OutputFlags::empty();
        cfg.control_flags = termios::ControlFlags::empty();
        cfg.local_flags = termios::LocalFlags::empty();
        termios::cfmakeraw(&mut cfg);
        cfg.input_flags |= termios::InputFlags::IGNBRK;
        cfg.control_flags |= termios::ControlFlags::CREAD;
        //cfg.control_flags |= termios::ControlFlags::CRTSCTS;
        termios::cfsetospeed(&mut cfg, baudrate).map_err(to_io_error)?;
        termios::cfsetispeed(&mut cfg, baudrate).map_err(to_io_error)?;
        termios::cfsetspeed(&mut cfg, baudrate).map_err(to_io_error)?;
        // Set VMIN = 1 to block until at least one character is received.
        cfg.control_chars[termios::SpecialCharacterIndices::VMIN as usize] = 1;
        termios::tcsetattr(fd, termios::SetArg::TCSANOW, &cfg).map_err(to_io_error)?;
        termios::tcflush(fd, termios::FlushArg::TCIOFLUSH).map_err(to_io_error)?;
        let mutex = Mutex::new(fd);
        Ok(Self { fd: mutex })
    }
}

impl AsRawFd for SerialPort {
    fn as_raw_fd(&self) -> RawFd {
        *self.fd.lock().unwrap()
    }
}

impl AsFd for SerialPort {
    fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(*self.fd.lock().unwrap()) }
    }
}

impl SerialPort {
    pub fn lock(&self) -> MutexGuard<RawFd> {
        self.fd.lock().unwrap() // You should handle the error properly in your code
    }
}
impl io::Read for SerialPort {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        nix::unistd::read(*self.fd.lock().unwrap(), buf).map_err(to_io_error)
    }
}

unsafe impl IoSafe for SerialPort {}

impl io::Write for SerialPort {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        nix::unistd::write(*self.fd.lock().unwrap(), buf).map_err(to_io_error)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn to_io_error(e: Errno) -> io::Error {
    e.into()
}
