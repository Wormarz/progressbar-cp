use super::super::filecopier::FileCopier;
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

    fn zero_copy(sfd: RawFd, dfd: RawFd, count: usize) -> std::io::Result<usize> {
        let ret = unsafe { libc::sendfile(dfd, sfd, std::ptr::null_mut(), count) };
        if ret < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(ret as usize)
        }
    }
}

impl FileCopier for Copier {
    fn copy<'a>(
        &'a mut self,
        src: std::fs::File,
        des: std::fs::File,
        total: Option<u64>,
        progress_callback: Option<&'a dyn Fn(u64, u64)>,
    ) -> std::io::Result<u64> {
        let sfd = src.as_raw_fd();
        let dfd = des.as_raw_fd();
        let mut copied = 0;

        loop {
            match Copier::zero_copy(sfd, dfd, self.buf_sz) {
                Ok(0) => break,
                Ok(n) => {
                    copied += n as u64;
                    if let Some(f) = progress_callback {
                        f(copied, total.unwrap_or(0));
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(copied)
    }
}