use super::super::filecopy::FileCopy;
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

impl FileCopy for Copier {
    fn simple_copy_once(&mut self, src: &mut std::fs::File, des: &mut std::fs::File) -> std::io::Result<u64> {
        match src.read(&mut self.buffer) {
            Ok(0) => Ok(0),
            Ok(n) => match des.write_all(&self.buffer[..n]) {
                Ok(()) => Ok(n as u64),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}
