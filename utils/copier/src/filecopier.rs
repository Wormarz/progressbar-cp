
pub trait FileCopier {
    fn copy<'a>(
        &'a mut self,
        src: std::fs::File,
        des: std::fs::File,
        total: Option<u64>,
        progress_callback: Option<&'a dyn Fn(u64, u64)>,
    ) -> std::io::Result<u64>;
}
