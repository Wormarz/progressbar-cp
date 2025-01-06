use super::super::filecopier::FileCopier;
use std::io::{Read, Write};

pub struct Copier {
    buffer: Vec<u8>,
}

impl Copier {
    pub fn new(buf_sz: usize) -> Self {
        Self {
            buffer: vec![0u8; buf_sz],
        }
    }
}

impl FileCopier for Copier {
    fn copy<'a>(
        &'a mut self,
        mut src: std::fs::File,
        mut des: std::fs::File,
        total: Option<u64>,
        progress_callback: Option<&'a dyn Fn(u64, u64)>,
    ) -> std::io::Result<u64> {
        let mut copied = 0;

        loop {
            match src.read(&mut self.buffer) {
                Ok(0) => break,
                Ok(n) => match des.write_all(&self.buffer[0..n]) {
                    Ok(()) => {
                        copied += n as u64;
                        if let Some(f) = progress_callback {
                            f(copied, total.unwrap_or(0));
                        }
                    }
                    Err(e) => return Err(e),
                },
                Err(e) => return Err(e),
            }
        }
        Ok(copied)
    }
}
