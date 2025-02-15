pub enum ActRet {
    GoOn,
    SkipCopy,
}

pub trait Action {
    fn run(&self, src: &str, dst: &str) -> anyhow::Result<ActRet>;
}

pub mod recursive;
pub mod update;
