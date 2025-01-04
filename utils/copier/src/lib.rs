use std::io::{Read, Write};
use log::debug;
use std::fs;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

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


pub fn do_copy(args: &Vec<String>) -> Result<(), std::io::Error> {
    let mut copier = Copier::new(4096);

    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let total_pbar = m.add(ProgressBar::new(args.len() as u64 - 1));
    total_pbar.set_style(sty.clone());

    let pb = m.add(ProgressBar::new(0));
    pb.set_style(sty);

    for src in &args[0..args.len() - 1] {
        let src_file = fs::File::open(src)?;
        let des_file =
            fs::File::create(args[args.len() - 1].clone() + "/" + src)?;
        let len = src_file.metadata().unwrap().len();
        pb.set_length(len);

        debug!(
            "Copying {} to {}",
            src,
            args[args.len() - 1].clone() + "/" + src
        );

        copier.copy(src_file, des_file, None, |copied: u64, _: u64| {
            pb.set_position(copied);
            pb.set_message(format!("bytes copied"));
        })?;

        total_pbar.inc(1);
        total_pbar.set_message(format!("files copied"));
    }
    total_pbar.finish_with_message("All files copied");
    m.clear()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
