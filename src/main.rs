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

        match precopy_acts.iter().fold(ActRet::GoOn, |pre, act| {
            match (
                act.pre_run(src, des)
                    .expect(&format!("pre actions failed({}, {})", src, des)),
                pre,
            ) {
                (ActRet::GoOn, pre) => pre,
                (ActRet::SkipRest, ActRet::SkipCopy) => ActRet::SkipCopy,
                (ActRet::SkipRest, _) => ActRet::SkipRest,
                (ActRet::SkipCopy, _) => ActRet::SkipCopy,
            }
        }) {
            ActRet::GoOn => {
                let src_file = File::open(src)?;
                let des_file = File::create(des)?;

                copier
                    .copy(src_file, des_file, &*in_copy_action)
                    .expect(&format!("copy failed({}, {})", src, des));
            }
            ActRet::SkipRest => continue,
            ActRet::SkipCopy => {}
        };

        postcopy_acts.iter().for_each(|act| {
            act.post_run(src, des)
                .expect(&format!("post actions failed({}, {})", src, des))
        });
    }

    ending.done()?;

    Ok(())
}
