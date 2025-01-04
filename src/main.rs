use clap::Parser;
use log::trace;

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

    copier::do_copy(&args.srcs_des)
}
