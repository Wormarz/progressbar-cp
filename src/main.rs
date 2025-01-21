use std::str;

use anyhow::Context;
use clap::Parser;
use log::debug;

/// rs_cp - copy files
#[derive(Parser, Debug)]
#[command(
    version,
    about = "rs_cp - copy files",
    long_about = "rs_cp - copy files"
)]
struct Args {
    /// Copy from SRCS... to DES
    srcs_des: Vec<String>,
    /// recursive copy
    #[arg(short, long)]
    recursive: bool,
}

impl Args {
    fn check(&self) -> anyhow::Result<()> {
        if self.srcs_des.len() < 2 {
            Err(anyhow::anyhow!("Need at least a src and a des"))
        } else {
            Ok(())
        }
    }

    fn apart_srcs_des(&self) -> anyhow::Result<(&[String], &String)> {
        let srcs = &self.srcs_des[0..self.srcs_des.len() - 1];
        let des = &self.srcs_des[self.srcs_des.len() - 1];
        Ok((srcs, des))
    }
}

struct BaseAction;

impl copier::InCopyAction for BaseAction {
    fn has_before(&self) -> bool {
        true
    }

    fn has_after(&self) -> bool {
        false
    }

    fn before(&self, spath: &str, _: &str) -> anyhow::Result<()> {
        if std::fs::metadata(spath).with_context(|| format!("failed to read metadata of {}", spath))?.is_dir() {
            anyhow::bail!("{} is a directory, should specify -r.", spath);
        }

        Ok(())
    }

    fn after(&self, _: &str, _: &str) -> anyhow::Result<()> {
        todo!();
    }
}

struct RecursiveAction;

impl copier::InCopyAction for RecursiveAction {
    fn has_before(&self) -> bool {
        true
    }

    fn has_after(&self) -> bool {
        false
    }

    fn before(&self, spath: &str, dpath: &str) -> anyhow::Result<()> {
        debug!("Before copy {} to {}", spath, dpath);
        Ok(())
    }

    fn after(&self, _: &str, _: &str) -> anyhow::Result<()> {
        todo!();
    }
    
}

struct ActionBuilder;

impl ActionBuilder {
    fn build(args: &Args) -> Box<dyn copier::InCopyAction> {
        if args.recursive {
            Box::new(RecursiveAction)
        } else {
            Box::new(BaseAction)
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    env_logger::init();

    let cus_act = ActionBuilder::build(&args);

    debug!("{:?}", args);
    args.check()?;

    let (srcs, des) = args.apart_srcs_des()?;

    Ok(copier::do_copy(srcs, des, &*cus_act)?)
}
