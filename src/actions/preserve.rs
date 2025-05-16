use super::{ActRet, PostAction, PreAction};
use anyhow::Context;
use filetime;
use log::debug;
use std::fs;
use std::os::unix::fs::MetadataExt;

pub struct PreserveAction {
    attrs: Vec<String>,
}

impl PreserveAction {
    pub fn new(attrs: String) -> Self {
        let attrs = if attrs == "all" {
            vec!["links", "mode", "ownership", "timestamps"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        } else {
            attrs.split(',').map(|s| s.to_string()).collect()
        };
        PreserveAction { attrs }
    }
}

impl PreAction for PreserveAction {
    fn pre_run(&self, src: &str, des: &str) -> anyhow::Result<ActRet> {
        for attr in self.attrs.iter() {
            if attr == "links" {
                if let Ok(target) = fs::read_link(src) {
                    std::os::unix::fs::symlink(&target, des)
                        .with_context(|| format!("Failed to create symlink for: {}", des))?;
                    return Ok(ActRet::SkipCopy);
                }
            }
        }

        Ok(ActRet::GoOn)
    }
}

impl PostAction for PreserveAction {
    fn post_run(&self, src: &str, des: &str) -> anyhow::Result<()> {
        let src_metadata = fs::metadata(src)
            .with_context(|| format!("Failed to get metadata of source: {}", src))?;

        for attr in self.attrs.iter() {
            match attr.as_str() {
                "mode" => {
                    match fs::set_permissions(des, src_metadata.permissions()) {
                        Ok(_) => {}
                        Err(e) => {
                            debug!("Failed to set permissions for: {}", des);
                            debug!("Error: {}", e);
                        }
                    };
                }
                "ownership" => {
                    let uid = nix::unistd::Uid::from_raw(src_metadata.uid());
                    let gid = nix::unistd::Gid::from_raw(src_metadata.gid());
                    match nix::unistd::chown(des, Some(uid), Some(gid)) {
                        Ok(_) => {}
                        Err(e) => {
                            debug!("Failed to set ownership for: {}", des);
                            debug!("Error: {}", e);
                        }
                    };
                }
                "timestamps" => {
                    let atime = filetime::FileTime::from_last_access_time(&src_metadata);
                    let mtime = filetime::FileTime::from_last_modification_time(&src_metadata);
                    match filetime::set_file_times(des, atime, mtime) {
                        Ok(_) => {}
                        Err(e) => {
                            debug!("Failed to set timestamps for: {}", des);
                            debug!("Error: {}", e);
                        }
                    };
                }
                _ => {}
            }
        }

        Ok(())
    }
}
