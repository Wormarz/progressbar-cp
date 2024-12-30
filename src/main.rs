use std::fs;
use clap::Parser;
use std::process;
use std::io::{Read, Write};
use std::process::exit;
use indicatif::ProgressBar;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = "rs_cp - copy files")]
struct Args {
    /// COPY FROM SRCS... TO DES
    srcs_des: Vec<String>,
}

impl Args {
    fn check(&self) {
        if self.srcs_des.len() < 2 {
            panic!("Need at least a src and a des");
        }
    }
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
    args.check();

    do_copy(&args);
}

fn do_copy(args: &Args) {
    let mut buffer = vec![0u8; 4096];
    let total_pbar = ProgressBar::new(args.srcs_des.len() as u64 - 1);

    for src in &args.srcs_des[0..args.srcs_des.len() - 1] {
        let src_file = fs::File::open(src).unwrap();
        let mut des_file = fs::File::create(args.srcs_des[args.srcs_des.len() - 1].clone() + "/" + src).unwrap();
        println!("Copying {} to {}", src, args.srcs_des[args.srcs_des.len() - 1].clone() + "/" + src);

    }
    // loop {
    //     let nbytes = src_file.read(&mut buffer).unwrap();
    //     des_file.write(&buffer[..nbytes]).unwrap();
    //     pb.inc(nbytes.try_into().unwrap());
    //     if nbytes < buffer.len() { break; }
    // }
}
