use crate::actions;
use anyhow::Context;
use clap::Parser;
use scanner::DirScan;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args {
    /// the copy sources
    #[arg(required(true))]
    srcs: Vec<String>,
    /// the copy destination
    #[arg(last(true), required(true))]
    des: String,
    /// recursive copy
    #[arg(short, long)]
    recursive: bool,
    /// copy only when the source file is newer than the destination file or when the destination file is missing
    #[arg(short, long)]
    update: bool,
    /// preserve the specified attributes (default: mode,ownership,timestamps), if possible additional attributes: context, links, xattr, all
    #[arg(short, long, value_name = "ATTR_LIST")]
    preserve: Option<String>,
}

impl Args {
    pub fn zip_src2des_pairs(&self) -> anyhow::Result<(Vec<String>, Vec<String>)> {
        let len = self.srcs.len() + 1;
        let src_paths = &self.srcs[..];
        let des = &self.des;

        let is_des_dir = match std::fs::metadata(des) {
            Ok(metadata) => metadata.is_dir(),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    false
                } else {
                    return Err(e.into());
                }
            }
        };

        match len {
            2 => {
                let is_src_dir = std::fs::metadata(&src_paths[0])
                    .with_context(|| format!("Failed to get metadata of {}", &src_paths[0]))?
                    .is_dir();
                let is_src_link = std::fs::symlink_metadata(&src_paths[0])
                    .with_context(|| {
                        format!("Failed to get symlink metadata of {}", &src_paths[0])
                    })?
                    .file_type()
                    .is_symlink();

                match (is_src_dir, is_des_dir, is_src_link) {
                    (false, true, _) => Ok((
                        vec![src_paths[0].clone()],
                        vec![des.clone() + "/" + src_paths[0].rsplit('/').next().unwrap()],
                    )),
                    (false, false, _) => Ok((vec![src_paths[0].clone()], vec![des.clone()])),
                    (true, true, _) => {
                        if self.recursive {
                            let scanner = scanner::scanners::basescanner::BaseScanner::new(des);
                            let (src_paths, des_paths) = scanner.scan(src_paths)?;

                            Ok((src_paths, des_paths))
                        } else {
                            Err(anyhow::anyhow!(
                                "{} is a directory, should specify -r",
                                src_paths[0]
                            ))
                        }
                    }
                    (true, false, false) => Err(anyhow::anyhow!(
                        "\'{}\' is a directory, should specify a directory as the last argument",
                        src_paths[0]
                    )),
                    (true, false, true) => Ok((vec![src_paths[0].clone()], vec![des.clone()])),
                }
            }
            _ => {
                if is_des_dir {
                    if self.recursive {
                        let scanner = scanner::scanners::basescanner::BaseScanner::new(des);
                        let (src_paths, des_paths) = scanner.scan(src_paths)?;

                        Ok((src_paths, des_paths))
                    } else {
                        let mut des_paths = Vec::new();
                        for src in src_paths {
                            let is_src_dir = std::fs::metadata(src)
                                .with_context(|| format!("Failed to get metadata of {}", src))?
                                .is_dir();
                            if is_src_dir {
                                return Err(anyhow::anyhow!(
                                    "\'{}\' is a directory, should specify -r",
                                    src
                                ));
                            } else {
                                des_paths.push(des.clone() + "/" + src.rsplit('/').next().unwrap());
                            }
                        }
                        Ok((src_paths.to_vec(), des_paths))
                    }
                } else {
                    Err(anyhow::anyhow!(
                        "\'{}\' is not a directory, should specify a directory as the last argument when having multiple srcs", des
                    ))
                }
            }
        }
    }

    pub fn build_in_progress_actions(
        &self,
    ) -> anyhow::Result<(Vec<Box<dyn actions::Action>>, Vec<Box<dyn actions::Action>>)> {
        let mut precopy_actions = Vec::<Box<dyn actions::Action>>::new();
        let mut postcopy_actions = Vec::<Box<dyn actions::Action>>::new();

        if self.recursive {
            precopy_actions.push(Box::new(crate::actions::recursive::RecursiveAction));
        }

        if self.update {
            precopy_actions.push(Box::new(crate::actions::update::UpdateAction));
        }

        if let Some(preserve) = self.preserve.clone() {
            precopy_actions.push(Box::new(crate::actions::preserve::PreserveAction::new(
                preserve.clone(),
            )));
            postcopy_actions.push(Box::new(crate::actions::preserve::PreserveAction::new(
                preserve,
            )));
        }

        Ok((precopy_actions, postcopy_actions))
    }
}
