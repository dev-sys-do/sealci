
use std::io;
use std::os::unix::io::{AsRawFd, RawFd};
use std::result;

pub(crate) const EPOLL_EVENTS_LEN: usize = 10;

pub struct EpollContext {
    raw_fd: RawFd,
}

impl EpollContext {
    pub fn new() -> result::Result<EpollContext, io::Error> {
        let raw_fd = epoll::create(true)?;
        Ok(EpollContext { raw_fd })
    }

    pub fn add_stdin(&self) -> result::Result<(), io::Error> {
        epoll::ctl(
            self.raw_fd,
            epoll::ControlOptions::EPOLL_CTL_ADD,
            libc::STDIN_FILENO,
            epoll::Event::new(epoll::Events::EPOLLIN, libc::STDIN_FILENO as u64),
        )?;

        Ok(())
    }
}

impl AsRawFd for EpollContext {
    fn as_raw_fd(&self) -> RawFd {
        self.raw_fd
    }
}
