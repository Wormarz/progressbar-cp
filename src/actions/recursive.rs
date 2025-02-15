use super::{ActRet, Action};
use anyhow::Context;

pub struct RecursiveAction;

impl Action for RecursiveAction {
    fn run(&self, src: &str, des: &str) -> anyhow::Result<ActRet> {
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

// pub fn recursive_action(src: &str, des: &str) -> anyhow::Result<ActRet> {
//     if std::fs::metadata(src)
//         .with_context(|| format!("Failed to get metadata of {}", src))?
//         .is_dir()
//     {
//         // create directory
//         match std::fs::create_dir(des) {
//             Ok(_) => Ok(ActRet::SkipCopy),
//             Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(ActRet::SkipCopy),
//             Err(e) => Err(e.into()),
//         }
//     } else {
//         Ok(ActRet::GoOn)
//     }
// }
