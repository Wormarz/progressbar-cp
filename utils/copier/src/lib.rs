pub mod copiers;

pub trait FileCopy {
    fn simple_copy_once(
        &mut self,
        src: &mut std::fs::File,
        des: &mut std::fs::File,
    ) -> std::io::Result<u64>;

    fn copy<'a>(
        &'a mut self,
        mut src: std::fs::File,
        mut des: std::fs::File,
        progress_callback: &'a dyn InCopyAction,
    ) -> std::io::Result<u64> {
        let mut copied = 0;

        progress_callback.set_length(src.metadata().unwrap().len());

        loop {
            match Self::simple_copy_once(self, &mut src, &mut des) {
                Ok(0) => break,
                Ok(n) => {
                    copied += n;
                    progress_callback.in_copy_run(copied);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
                Err(e) => return Err(e),
            }
        }

        Ok(copied)
    }
}

pub trait InCopyAction {
    fn set_length(&self, length: u64);
    fn in_copy_run(&self, copied: u64);
}
