use std::io::{Read, Write};

pub struct Copier {
    buffer: Vec<u8>,
}

impl Copier {
    pub fn new(buf_sz: usize) -> Self {
        Self { buffer: vec![0u8; buf_sz] }
    }

    pub fn copy<R: Read, W: Write>(
        &mut self,
        mut src: R,
        mut des: W,
        total: Option<u64>,
        progress_callback: impl FnOnce(u64, u64) + Copy,
    ) -> std::io::Result<u64> {
        let mut copied = 0;

        loop {
            match src.read(&mut self.buffer) {
                Ok(0) => break,
                Ok(n) => {
                    match des.write_all(&self.buffer[0..n]) {
                        Ok(()) => {
                            copied += n as u64;
                            progress_callback(copied, total.unwrap_or(0));
                        }
                        Err(e) => return Err(e),
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(copied)
    }
}
