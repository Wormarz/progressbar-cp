use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;

fn show_usage() {
    println!("rs_cp - copy files\nUsage: cp <src> <des>\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        show_usage();
        return;
    }

    let src_path = &args[1];
    let des_path = &args[2];

    let mut src_file = fs::File::open(src_path).unwrap();
    let mut des_file = fs::File::create(des_path).unwrap();
    let mut buffer = [0u8; 4096];

    loop {
        let nbytes = src_file.read(&mut buffer).unwrap();
        des_file.write(&buffer[..nbytes]).unwrap();
        if nbytes < buffer.len() { break; }
    }
}
