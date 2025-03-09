use super::{ActRet, PreAction};
use anyhow::Context;

pub struct UpdateAction;

impl PreAction for UpdateAction {
    fn pre_run(&self, src: &str, des: &str) -> anyhow::Result<ActRet> {
        let des_metadata = match std::fs::metadata(des) {
            Ok(metadata) => metadata,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(ActRet::GoOn),
            Err(e) => return Err(e.into()),
        };
        let des_modified = des_metadata
            .modified()
            .with_context(|| format!("Failed to get modified time of destination: {}", des))?;
        let src_modified = std::fs::metadata(src)
            .with_context(|| format!("Failed to get metadata of source: {}", src))?
            .modified()
            .with_context(|| format!("Failed to get modified time of source: {}", src))?;
        if src_modified <= des_modified {
            return Ok(ActRet::SkipRest);
        }
        Ok(ActRet::GoOn)
    }
}
