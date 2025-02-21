pub enum ActRet {
    GoOn,
    SkipCopy,
}

pub trait PreAction {
    fn pre_run(&self, src: &str, dst: &str) -> anyhow::Result<ActRet>;
}

pub trait PostAction {
    fn post_run(&self, src: &str, dst: &str) -> anyhow::Result<()>;
}

pub mod preserve;
pub mod recursive;
pub mod update;
