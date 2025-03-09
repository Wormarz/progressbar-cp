use super::{Ending, PostAction, Preparation};
use anyhow::Context;
use copier::InCopyAction;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub struct ShowBar {
    _m: MultiProgress,
    total_pbar: ProgressBar,
    pb: ProgressBar,
}

impl ShowBar {
    pub fn new() -> anyhow::Result<Self> {
        let m = MultiProgress::new();
        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .with_context(|| "Failed to create progress style")?
        .progress_chars("##-");

        let total_pbar = m.add(ProgressBar::new(0));
        total_pbar.set_style(sty.clone());

        let pb = m.add(ProgressBar::new(0));
        pb.set_style(sty);

        Ok(Self {
            _m: m,
            total_pbar,
            pb,
        })
    }
}

impl Preparation for ShowBar {
    fn get_ready(&self, total: u64) -> anyhow::Result<()> {
        self.total_pbar.set_length(total);
        Ok(())
    }
}

impl InCopyAction for ShowBar {
    fn set_length(&self, length: u64) {
        self.pb.set_length(length);
    }

    fn in_copy_run(&self, copied: u64) {
        self.pb.set_position(copied);
        self.pb.set_message("bytes copied");
    }
}

impl PostAction for ShowBar {
    fn post_run(&self, _: &str, _: &str) -> anyhow::Result<()> {
        self.total_pbar.inc(1);
        self.total_pbar.set_message("files copied");
        Ok(())
    }
}

impl Ending for ShowBar {
    fn done(&self) -> anyhow::Result<()> {
        self.total_pbar.finish_with_message("All files copied");
        Ok(())
    }
}

pub struct NoBar;

impl Preparation for NoBar {
    fn get_ready(&self, _: u64) -> anyhow::Result<()> {
        Ok(())
    }
}

impl InCopyAction for NoBar {
    fn set_length(&self, _: u64) {}
    fn in_copy_run(&self, _: u64) {}
}

impl Ending for NoBar {
    fn done(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
