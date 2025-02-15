use super::ActRet;
use anyhow::Context;

pub fn update_action(src: &str, des: &str) -> anyhow::Result<ActRet> {
    let des_metadata = std::fs::metadata(des)
        .with_context(|| format!("Failed to get metadata of destination: {}", des))?;
    let des_modified = des_metadata
        .modified()
        .with_context(|| format!("Failed to get modified time of destination: {}", des))?;
    let src_modified = std::fs::metadata(src)
        .with_context(|| format!("Failed to get metadata of source: {}", src))?
        .modified()
        .with_context(|| format!("Failed to get modified time of source: {}", src))?;
    if src_modified <= des_modified {
        return Ok(ActRet::SkipCopy);
    }
    Ok(ActRet::GoOn)
}
