use clap::Parser;
use copier::Copier;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::{debug, trace};
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(
    version,
    about = "rs_cp - copy files",
    long_about = "rs_cp - copy files"
)]
struct Args {
    /// Copy from SRCS... to DES
    srcs_des: Vec<String>,
}

impl Args {
    fn check(&self) -> Result<(), std::io::Error> {
        if self.srcs_des.len() < 2 {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Need at least a src and a des",
            ))
        } else {
            Ok(())
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    trace!("{:?}", args);
    args.check()?;

    do_copy(&args)
}

fn do_copy(args: &Args) -> Result<(), std::io::Error> {
    let mut copier = Copier::new(4096);

    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let total_pbar = m.add(ProgressBar::new(args.srcs_des.len() as u64 - 1));
    total_pbar.set_style(sty.clone());

    let pb = m.add(ProgressBar::new(0));
    pb.set_style(sty);

    for src in &args.srcs_des[0..args.srcs_des.len() - 1] {
        let src_file = fs::File::open(src)?;
        let des_file =
            fs::File::create(args.srcs_des[args.srcs_des.len() - 1].clone() + "/" + src)?;
        let len = src_file.metadata().unwrap().len();
        pb.set_length(len);

        debug!(
            "Copying {} to {}",
            src,
            args.srcs_des[args.srcs_des.len() - 1].clone() + "/" + src
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
