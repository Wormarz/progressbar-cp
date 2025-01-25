use super::super::DirScan;
use anyhow::{Context, Result};
use log::trace;
use std::fs;
use walkdir::WalkDir;

pub struct BaseScanner<'a> {
    des_path: &'a str,
}

impl BaseScanner<'_> {
    pub fn new<'a>(des_path: &'a str) -> BaseScanner<'a> {
        BaseScanner { des_path }
    }
}

impl DirScan for BaseScanner<'_> {
    fn in_scan_action(&self, cur_entry: &str) -> Result<(Vec<String>, Vec<String>)> {
        let mut src_paths: Vec<String> = Vec::new();
        let mut des_paths: Vec<String> = Vec::new();

        let parent_entry = cur_entry
            .trim_end_matches('/')
            .trim_end_matches(cur_entry.trim_end_matches('/').rsplit('/').next().unwrap());
        trace!("parent_entry: {}", parent_entry);

        let metadata = fs::metadata(cur_entry)
            .with_context(|| format!("failed to read metadata of {}", cur_entry))?;
        if metadata.is_dir() {
            for entry in WalkDir::new(cur_entry).into_iter().filter_map(Result::ok) {
                let src_entry = entry.path().to_str().unwrap();
                trace!("{} found!", src_entry);

                let mut des_entry = if parent_entry == "" {
                    String::from("/") + src_entry
                } else {
                    src_entry.replace(parent_entry, "/").to_string()
                };

                des_entry.insert_str(0, self.des_path);

                src_paths.push(src_entry.to_string());
                des_paths.push(des_entry);
            }
        } else {
            src_paths.push(cur_entry.to_string());
            des_paths.push(self.des_path.to_string() + &cur_entry.replace(parent_entry, "/"));
        }

        Ok((src_paths, des_paths))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, vec};
    use tempfile;

    #[test]
    fn scan_dir_works() {
        let src_dir = tempfile::tempdir_in(".").unwrap();
        let src_dir_path = src_dir.path();
        let src_file_path = src_dir_path.join("my-temporary-note.txt");
        let _file = File::create(&src_file_path).unwrap();

        let des_dir = tempfile::tempdir_in(".").unwrap();
        let des_dir_path = des_dir.path();

        let scanner = BaseScanner::new(des_dir_path.to_str().unwrap());

        let relative_src = src_dir_path.file_name().unwrap().to_string_lossy().to_string();

        let (src_paths, des_paths) = scanner
            .scan(&vec![src_dir_path.to_str().unwrap().to_string(), relative_src.clone()])
            .unwrap();

        println!("src_paths: {:?}\ndes_paths: {:?}", src_paths, des_paths);

        let sfolder = src_dir_path.to_str().unwrap().to_string();
        let sfile = src_file_path.to_str().unwrap().to_string();
        let sfolder_relative = relative_src.clone();
        let sfile_relative = relative_src + "/my-temporary-note.txt";
        assert_eq!(src_paths, vec![sfolder, sfile, sfolder_relative, sfile_relative]);

        let dfolder = des_dir_path.to_str().unwrap().to_string()
            + "/"
            + src_dir_path.to_str().unwrap().rsplit('/').next().unwrap();
        let dfile =
            dfolder.clone() + "/" + src_file_path.to_str().unwrap().rsplit('/').next().unwrap();

        assert_eq!(des_paths, vec![dfolder.clone(), dfile.clone(), dfolder, dfile]);
    }

    #[test]
    fn scan_file_works() {
        let src_dir = tempfile::tempdir_in(".").unwrap();
        let src_dir_path = src_dir.path();
        let src_file_path = src_dir_path.join("my-temporary-note.txt");
        let _file = File::create(&src_file_path).unwrap();

        let des_dir = tempfile::tempdir_in(".").unwrap();
        let des_dir_path = des_dir.path();

        let scanner = BaseScanner::new(des_dir_path.to_str().unwrap());

        let (src_paths, des_paths) = scanner
            .scan(&vec![src_file_path.to_str().unwrap().to_string()])
            .unwrap();

        println!("src_paths: {:?}\ndes_paths: {:?}", src_paths, des_paths);

        let sfile = src_file_path.to_str().unwrap().to_string();
        assert_eq!(src_paths, vec![sfile]);

        let dfile = des_dir_path.to_str().unwrap().to_string()
            + "/"
            + src_file_path.to_str().unwrap().rsplit('/').next().unwrap();
        assert_eq!(des_paths, vec![dfile]);
    }

    #[test]
    fn scan_stop_at_link() {
        let temp_dir = tempfile::tempdir_in(".").unwrap();
        let temp_dir_path = temp_dir.path();
        let file_path = temp_dir_path.join("my-temporary-note.txt");
        let _file = File::create(&file_path).unwrap();

        let src_dir = tempfile::tempdir_in(".").unwrap();
        let src_dir_path = src_dir.path();
        let src_link_path = src_dir_path.join("my-temporary-note.link");
        let _link = std::os::unix::fs::symlink(temp_dir_path, &src_link_path).unwrap();

        let des_dir = tempfile::tempdir_in(".").unwrap();
        let des_dir_path = des_dir.path();

        let scanner = BaseScanner::new(des_dir_path.to_str().unwrap());
        let (src_paths, des_paths) = scanner
            .scan(&vec![src_dir_path.to_str().unwrap().to_string()])
            .unwrap();
        println!("src_paths: {:?}\ndes_paths: {:?}", src_paths, des_paths);

        let sfolder = src_dir_path.to_str().unwrap().to_string();
        let slink = src_link_path.to_str().unwrap().to_string();
        assert_eq!(src_paths, vec![sfolder, slink]);

        let dfolder = des_dir_path.to_str().unwrap().to_string()
            + "/"
            + src_dir_path.to_str().unwrap().rsplit('/').next().unwrap();
        let dlink =
            dfolder.clone() + "/" + src_link_path.to_str().unwrap().rsplit('/').next().unwrap();
        assert_eq!(des_paths, vec![dfolder, dlink]);
    }

    #[test]
    fn scan_nonexistent_dir() {
        let des_dr = tempfile::tempdir_in(".").unwrap();
        let des_dir_path = des_dr.path();

        let scanner = BaseScanner::new(des_dir_path.to_str().unwrap());
        let ret = scanner.scan(&vec!["nonexistent".to_string()]);
        assert!(ret.is_err());
    }
}
