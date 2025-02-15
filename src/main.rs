mod actions;
mod arg;

use actions::{ActRet, Action};
use anyhow::Context;
use arg::Args;
use clap::Parser;
use copier::FileCopy;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::{debug, trace};
use std::fs::File;

#[cfg(feature = "basecopier")]
use copier::copiers::basecopier::Copier;
#[cfg(feature = "zerocopier")]
use copier::copiers::zerocopier::Copier;

fn do_pbcopy(
    src_paths: &[String],
    des_paths: &[String],
    precopy_acts: Vec<Box<dyn Action>>,
    postcopy_acts: Vec<Box<dyn Action>>,
) -> anyhow::Result<()> {
    let mut copier = Copier::new(4096 * 1024);

    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .with_context(|| "Failed to create progress style")?
    .progress_chars("##-");

    let total_pbar = m.add(ProgressBar::new(src_paths.len() as u64));
    total_pbar.set_style(sty.clone());

    let pb = m.add(ProgressBar::new(0));
    pb.set_style(sty);

    let progress_callback = |copied: u64, _: u64| {
        pb.set_position(copied);
        pb.set_message(format!("bytes copied"));
    };

    for (src, des) in src_paths.iter().zip(des_paths.iter()) {
        trace!("Copy from {} to {}", src, des);

        let mut act_ret = ActRet::GoOn;
        for act in &precopy_acts {
            act_ret = match act.run(src, des)? {
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

        pb.set_length(
            src_file
                .metadata()
                .with_context(|| format!("Failed to get metadata of {}", src))?
                .len(),
        );

        copier.copy(src_file, des_file, None, Some(&progress_callback))?;

        total_pbar.inc(1);
        total_pbar.set_message(format!("files copied"));

        for act in &postcopy_acts {
            act.run(src, des)?;
        }
    }

    total_pbar.finish_with_message("All files copied");
    Ok(m.clear()?)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    env_logger::init();

    debug!("{:?}", args);

    let (src_paths, des_paths) = args.zip_src2des_pairs()?;

    let (precopy_acts, poscopy_acts) = args.build_in_progress_actions()?;

    debug!("src_paths: {:?}\ndes_paths: {:?}", src_paths, des_paths);

    Ok(do_pbcopy(
        &src_paths,
        &des_paths,
        precopy_acts,
        poscopy_acts,
    )?)
}
