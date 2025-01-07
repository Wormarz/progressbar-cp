use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::{trace, warn};
use std::fs;

mod copiers;
mod filecopy;

use crate::filecopy::FileCopy;

#[cfg(feature = "basecopier")]
use copiers::basecopier::Copier;
#[cfg(feature = "zerocopier")]
use copiers::zerocopier::Copier;

pub fn do_copy(args: &Vec<String>) -> Result<(), std::io::Error> {
    let mut copier = Copier::new(4096 * 1024);

    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let total_pbar = m.add(ProgressBar::new(args.len() as u64 - 1));
    total_pbar.set_style(sty.clone());

    let pb = m.add(ProgressBar::new(0));
    pb.set_style(sty);

    let progress_callback = |copied: u64, _: u64| {
        pb.set_position(copied);
        pb.set_message(format!("bytes copied"));
    };

    for src in &args[0..args.len() - 1] {
        trace!(
            "Copying {} to {}",
            src,
            args[args.len() - 1].clone() + "/" + src.rsplit('/').next().unwrap()
        );

        let src_file = fs::File::open(src)?;
        if src_file.metadata().unwrap().is_dir() {
            warn!("-r not specified, {} is a directory, skipping", src);
            total_pbar.set_length(total_pbar.length().unwrap_or(1) - 1);
            continue;
        }

        let des_file =
            fs::File::create(args[args.len() - 1].clone() + "/" + src.rsplit('/').next().unwrap())?;
        let len = src_file.metadata().unwrap().len();
        pb.set_length(len);

        copier.copy(src_file, des_file, None, Some(&progress_callback))?;

        total_pbar.inc(1);
        total_pbar.set_message(format!("files copied"));
    }
    total_pbar.finish_with_message("All files copied");
    m.clear()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
