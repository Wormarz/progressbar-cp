use std::env;
use std::fs;
use std::process;
use std::io::{Read, Write};
use indicatif::ProgressBar;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cp_configs = Config::new(&args);

    do_copy(cp_configs);
}

fn show_usage() {
    println!("rs_cp - copy files\nUsage: cp <src> <des>");
}

struct Config {
    src: String,
    des: String,
}

impl Config {
    /*constructor */
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            show_usage();
            process::exit(1);
        }
        let src = args[1].clone();
        let des = args[2].clone();

        Config { src, des }
    }
}

fn do_copy(config: Config) {
    let mut src_file = fs::File::open(config.src).unwrap();
    let mut des_file = fs::File::create(config.des).unwrap();
    let mut buffer = [0u8; 4096];
    let count = src_file.metadata().unwrap().len() as u64;
    let pb = ProgressBar::new(count);

    loop {
        let nbytes = src_file.read(&mut buffer).unwrap();
        des_file.write(&buffer[..nbytes]).unwrap();
        pb.inc(nbytes.try_into().unwrap());
        if nbytes < buffer.len() { break; }
    }
}
