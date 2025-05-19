use super::{ActRet, PreAction};
use std::path::Path;
pub struct RecursiveAction;

impl PreAction for RecursiveAction {
    fn pre_run(&self, src: &str, des: &str) -> anyhow::Result<ActRet> {
        let src_path = Path::new(src);

        if !src_path.exists() {
            Err(anyhow::anyhow!("Source path does not exist: {}", src))
        } else if src_path.is_dir() && !src_path.is_symlink() {
            // create directory
            match std::fs::create_dir(des) {
                Ok(_) => Ok(ActRet::SkipCopy),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(ActRet::SkipCopy),
                Err(e) => Err(e.into()),
            }
        } else {
            Ok(ActRet::GoOn)
        }
    }
}
