pub mod scanners;

pub trait DirScan {
    fn in_scan_action(
        &self,
        cur_entry: &str,
        strip_depth: u32,
    ) -> anyhow::Result<(Vec<String>, Vec<String>)>;

    fn scan(&self, paths: &[String], strip: bool) -> anyhow::Result<(Vec<String>, Vec<String>)> {
        let mut src_paths: Vec<String> = Vec::with_capacity(paths.len() * 2);
        let mut des_paths: Vec<String> = Vec::with_capacity(paths.len() * 2);

        for path in paths {
            let components: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
            let strip_depth = components.len() as u32 - if strip { 0 } else { 1 };
            let (mut src_path, mut des_path) = self.in_scan_action(path, strip_depth)?;
            src_paths.append(&mut src_path);
            des_paths.append(&mut des_path);
        }

        Ok((src_paths, des_paths))
    }
}
