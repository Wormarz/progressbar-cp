mod actions;
mod arg;

use actions::ActRet;
use arg::Args;
use clap::Parser;
use copier::FileCopy;
use log::{debug, trace};
use std::fs::File;

#[cfg(feature = "basecopier")]
use copier::copiers::basecopier::Copier;
#[cfg(feature = "zerocopier")]
use copier::copiers::zerocopier::Copier;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    env_logger::init();

    debug!("{:?}", args);

    let (src_paths, des_paths) = args.zip_src2des_pairs()?;

    let (preparation, precopy_acts, in_copy_action, postcopy_acts, ending) =
        args.build_in_progress_actions()?;

    debug!("src_paths: {:?}\ndes_paths: {:?}", src_paths, des_paths);

    let mut copier = Copier::new(4096 * 1024);
    preparation.get_ready(src_paths.len() as u64)?;

    for (src, des) in src_paths.iter().zip(des_paths.iter()) {
        trace!("Copy from {} to {}", src, des);

        let mut act_ret = ActRet::GoOn;
        for act in precopy_acts.iter() {
            act_ret = match act.pre_run(src, des)? {
                ActRet::SkipCopy => ActRet::SkipCopy,
                _ => act_ret,
            }
        }

        match act_ret {
            ActRet::SkipCopy => continue,
            _ => {}
        }

        //copy file
        let src_file = File::open(src)?;
        let des_file = File::create(des)?;

        copier.copy(src_file, des_file, &*in_copy_action)?;

        for act in postcopy_acts.iter() {
            act.post_run(src, des)?;
        }
    }

    ending.done()?;

    Ok(())
}
