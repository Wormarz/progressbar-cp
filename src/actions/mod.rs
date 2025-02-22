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

pub trait Preparation {
    fn get_ready(&self, total: u64) -> anyhow::Result<()>;
}

pub trait Ending {
    fn done(&self) -> anyhow::Result<()>;
}

pub mod preserve;
pub mod recursive;
pub mod showbar;
pub mod update;
