use clap::Parser;
use log::debug;

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

    fn apart_srcs_des(&self) -> Result<(&[String], &String), std::io::Error> {
        let srcs = &self.srcs_des[0..self.srcs_des.len() - 1];
        let des = &self.srcs_des[self.srcs_des.len() - 1];
        Ok((srcs, des))
    }
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    env_logger::init();

    debug!("{:?}", args);
    args.check()?;

    let (srcs, des) = args.apart_srcs_des()?;

    copier::do_copy(srcs, des)
}
