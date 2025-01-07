pub trait FileCopy {
    fn simple_copy_once(&mut self, src: &mut std::fs::File, des: &mut std::fs::File) -> std::io::Result<u64>;

    fn copy<'a>(
        &'a mut self,
        mut src: std::fs::File,
        mut des: std::fs::File,
        total: Option<u64>,
        progress_callback: Option<&'a dyn Fn(u64, u64)>,
    ) -> std::io::Result<u64> {
        let mut copied = 0;

        loop {
            match Self::simple_copy_once(self, &mut src, &mut des) {
                Ok(0) => break,
                Ok(n) => {
                    copied += n;
                    if let Some(progress_callback) = progress_callback {
                        progress_callback(copied, total.unwrap_or(0));
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(copied)
    }
}
