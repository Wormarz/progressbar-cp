use super::actions;
use anyhow::Context;
use clap::Parser;
use scanner::DirScan;
use std::rc::Rc;

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
    /// make the progress bar invisible
    #[arg(short, long)]
    mute: bool,
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
    ) -> anyhow::Result<(
        Rc<dyn actions::Preparation>,
        Vec<Rc<dyn actions::PreAction>>,
        Rc<dyn copier::InCopyAction>,
        Vec<Rc<dyn actions::PostAction>>,
        Rc<dyn actions::Ending>,
    )> {
        let mut precopy_actions = Vec::<Rc<dyn actions::PreAction>>::new();
        let mut postcopy_actions = Vec::<Rc<dyn actions::PostAction>>::new();
        let preparation: Rc<dyn actions::Preparation>;
        let in_copy_action: Rc<dyn copier::InCopyAction>;
        let ending: Rc<dyn actions::Ending>;

        if self.recursive {
            precopy_actions.push(Rc::new(actions::recursive::RecursiveAction));
        }

        if self.update {
            precopy_actions.push(Rc::new(actions::update::UpdateAction));
        }

        if let Some(preserve) = self.preserve.clone() {
            let pact_rc = Rc::new(actions::preserve::PreserveAction::new(preserve));
            precopy_actions.push(pact_rc.clone());
            postcopy_actions.push(pact_rc);
        }

        if self.mute {
            let no_bar = Rc::new(actions::showbar::NoBar);
            preparation = no_bar.clone();
            in_copy_action = no_bar.clone();
            ending = no_bar.clone();
        } else {
            let show_bar = Rc::new(actions::showbar::ShowBar::new()?);
            preparation = show_bar.clone();
            in_copy_action = show_bar.clone();
            ending = show_bar.clone();
            postcopy_actions.push(show_bar);
        };

        Ok((
            preparation,
            precopy_actions,
            in_copy_action,
            postcopy_actions,
            ending,
        ))
    }
}
