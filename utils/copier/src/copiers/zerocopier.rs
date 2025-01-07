use super::super::filecopy::FileCopy;
use std::os::fd::{RawFd, AsRawFd};
pub struct Copier {
    buf_sz: usize,
}

impl Copier {
    pub fn new(buf_sz: usize) -> Self {
        Self {
            buf_sz,
        }
    }

    fn zero_copy(sfd: RawFd, dfd: RawFd, count: usize) -> std::io::Result<u64> {
        let ret = unsafe { libc::sendfile(dfd, sfd, std::ptr::null_mut(), count) };
        if ret < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(ret as u64)
        }
    }
}

impl FileCopy for Copier {
    fn simple_copy_once(&mut self, src: &mut std::fs::File, des: &mut std::fs::File) -> std::io::Result<u64> {
        let sfd = src.as_raw_fd();
        let dfd = des.as_raw_fd();

        Self::zero_copy(sfd, dfd, self.buf_sz)
    }
}