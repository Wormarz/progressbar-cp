use super::{ActRet, Action};
use anyhow::Context;
use filetime;
use std::fs;
use std::os::unix::fs::MetadataExt;

pub struct PreserveAction {
    attrs: String,
}

impl PreserveAction {
    pub fn new(attrs: String) -> Self {
        PreserveAction { attrs }
    }
}

impl Action for PreserveAction {
    fn run(&self, src: &str, des: &str) -> anyhow::Result<ActRet> {
        let src_metadata = fs::metadata(src)
            .with_context(|| format!("Failed to get metadata of source: {}", src))?;

        let attributes: Vec<&str> = self.attrs.split(',').collect();

        for attr in attributes {
            match attr {
                "mode" => {
                    fs::set_permissions(des, src_metadata.permissions())
                        .with_context(|| format!("Failed to set permissions for: {}", des))?;
                }
                "ownership" => {
                    let uid = nix::unistd::Uid::from_raw(src_metadata.uid());
                    let gid = nix::unistd::Gid::from_raw(src_metadata.gid());
                    nix::unistd::chown(des, Some(uid), Some(gid))
                        .with_context(|| format!("Failed to set ownership for: {}", des))?;
                }
                "timestamps" => {
                    let atime = filetime::FileTime::from_last_access_time(&src_metadata);
                    let mtime = filetime::FileTime::from_last_modification_time(&src_metadata);
                    filetime::set_file_times(des, atime, mtime)
                        .with_context(|| format!("Failed to set timestamps for: {}", des))?;
                }
                _ => {}
            }
        }

        Ok(ActRet::GoOn)
    }
}
