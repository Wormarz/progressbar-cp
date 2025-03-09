use super::{ActRet, PreAction};
use anyhow::Context;

pub struct RecursiveAction;

impl PreAction for RecursiveAction {
    fn pre_run(&self, src: &str, des: &str) -> anyhow::Result<ActRet> {
        if std::fs::metadata(src)
            .with_context(|| format!("Failed to get metadata of {}", src))?
            .is_dir()
        {
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
