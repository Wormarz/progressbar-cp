pub mod scanners;

pub trait DirScan {
    fn in_scan_action(&self, cur_entry: &str) -> anyhow::Result<(Vec<String>, Vec<String>)>;

    fn scan(&self, paths: &[String]) -> anyhow::Result<(Vec<String>, Vec<String>)> {
        let mut src_paths: Vec<String> = Vec::with_capacity(paths.len() * 2);
        let mut des_paths: Vec<String> = Vec::with_capacity(paths.len() * 2);

        for path in paths {
            let (mut src_path, mut des_path) = self.in_scan_action(path)?;
            src_paths.append(&mut src_path);
            des_paths.append(&mut des_path);
        }

        Ok((src_paths, des_paths))
    }
}
